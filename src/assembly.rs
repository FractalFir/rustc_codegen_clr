use crate::cil_op::{CILOp, CallSite};
use crate::codegen_error::MethodCodegenError;
use crate::r#type::TyCache;
use crate::utilis::monomorphize;
use crate::IString;
use crate::{
    access_modifier::AccessModifer, codegen_error::CodegenError, function_sig::FnSig,
    method::Method, r#type::Type, r#type::TypeDef,
};
use rustc_middle::mir::{mono::MonoItem, Local, LocalDecl, Statement, Terminator};
use rustc_middle::ty::{Instance, ParamEnv, TyCtxt, TyKind};
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
/// Data representing a reference to an external assembly.
pub struct AssemblyExternRef {
    version: (u16, u16, u16, u16),
}
impl AssemblyExternRef {
    /// Returns the version information of this assembly.
    pub fn version(&self) -> (u16, u16, u16, u16) {
        self.version
    }
}
#[derive(Serialize, Deserialize, Debug)]
/// Representation of a .NET assembly.
pub struct Assembly {
    types: HashSet<TypeDef>,
    functions: HashSet<Method>,
    entrypoint: Option<CallSite>,
    extern_refs: HashMap<IString, AssemblyExternRef>,
}
impl Assembly {
    /// Returns the external assembly reference
    pub fn extern_refs(&self) -> &HashMap<IString, AssemblyExternRef> {
        &self.extern_refs
    }
    /// Creates a new, empty assembly.
    pub fn empty() -> Self {
        let mut res = Self {
            types: HashSet::new(),
            functions: HashSet::new(),
            entrypoint: None,
            extern_refs: HashMap::new(),
        };
        let dotnet_ver = AssemblyExternRef {
            version: (6, 12, 0, 0),
        };
        res.extern_refs.insert("System.Runtime".into(), dotnet_ver);
        //res.extern_refs.insert("mscorlib".into(),dotnet_ver);
        res.extern_refs
            .insert("System.Runtime.InteropServices".into(), dotnet_ver);
        res
    }
    /// Joins 2 assemblies together.
    pub fn join(self, other: Self) -> Self {
        let types = self.types.union(&other.types).cloned().collect();
        let functions = self.functions.union(&other.functions).cloned().collect();
        let entrypoint = self.entrypoint.or(other.entrypoint);
        let mut extern_refs = self.extern_refs;
        extern_refs.extend(other.extern_refs);
        Self {
            types,
            functions,
            entrypoint,
            extern_refs,
        }
    }
    /// Gets the typdefef at path `path`.
    pub fn get_typedef_by_path(&self, path: &str) -> Option<&TypeDef> {
        if path.contains("/") {
            let mut path_iter = path.split("/");
            let path_first = path_iter.next().unwrap();
            let mut td = self.get_typedef_by_path(path_first)?;
            // FIXME: this loop is messy.
            for part in path_iter {
                let old = td;
                for inner in td.inner_types() {
                    if inner.name() == part {
                        td = inner;
                        break;
                    }
                }
                if td == old {
                    return None;
                }
            }
            return Some(td);
        } else {
            for tpe in self.types() {
                if tpe.name() == path {
                    return Some(tpe);
                }
            }
        }
        None
    }
    /// Turns a terminator into ops, if ABORT_ON_ERROR set to false, will handle and recover from errors.
    pub fn terminator_to_ops<'tcx>(
        term: &Terminator<'tcx>,
        mir: &'tcx rustc_middle::mir::Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        instance: Instance<'tcx>,
        type_cache:&mut TyCache,
    ) -> Vec<CILOp> {
        if crate::ABORT_ON_ERROR {
            crate::terminator::handle_terminator(term, mir, tcx, mir, instance,type_cache)
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::terminator::handle_terminator(term, mir, tcx, mir, instance,type_cache)
            })) {
                Ok(ok) => ok,
                Err(payload) => {
                    let msg = if let Some(msg) = payload.downcast_ref::<&str>() {
                        rustc_middle::ty::print::with_no_trimmed_paths! {
                        format!("Tried to execute terminator {term:?} whose compialtion message {msg:?}!")}
                    } else {
                        eprintln!("handle_terminator panicked with a non-string message!");
                        rustc_middle::ty::print::with_no_trimmed_paths! {
                        format!("Tried to execute terminator {term:?} whose compialtion failed with a no-string message!")
                        }
                    };
                    CILOp::throw_msg(&msg).into()
                }
            }
        }
    }
    /// Turns a statement into ops, if ABORT_ON_ERROR set to false, will handle and recover from errors.
    pub fn statement_to_ops<'tcx>(
        statement: &Statement<'tcx>,
        tcx: TyCtxt<'tcx>,
        mir: &rustc_middle::mir::Body<'tcx>,
        instance: Instance<'tcx>,
        type_cache:&mut TyCache,
    ) -> Result<Vec<CILOp>, CodegenError> {
        if crate::ABORT_ON_ERROR {
            Ok(crate::statement::handle_statement(
                statement, tcx, mir, instance,type_cache
            ))
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::statement::handle_statement(statement, tcx, mir, instance,type_cache)
            })) {
                Ok(success) => Ok(success),
                Err(payload) => {
                    if let Some(msg) = payload.downcast_ref::<&str>() {
                        Err(crate::codegen_error::CodegenError::from_panic_message(msg))
                    } else {
                        Err(crate::codegen_error::CodegenError::from_panic_message(
                            "statement_to_ops panicked with a non-string message!",
                        ))
                    }
                }
            }
        }
    }
    /// This is used *ONLY* to catch uncaught errors.
    fn checked_add_fn<'tcx>(
        &mut self,
        instance: Instance<'tcx>,
        tcx: TyCtxt<'tcx>,
        name: &str,
        cache:&mut TyCache,
    ) -> Result<(), MethodCodegenError> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.add_fn(instance, tcx, name,cache)
        })) {
            Ok(success) => success,
            Err(payload) => {
                if let Some(msg) = payload.downcast_ref::<&str>() {
                    eprintln!("fn_add panicked with unhandled message: {msg:?}");
                    return Ok(());
                } else {
                    eprintln!("fn_add panicked with no message.");
                    return Ok(());
                }
            }
        }
    }
    //fn terminator_to_ops()
    /// Adds a rust MIR function to the assembly.
    pub fn add_fn<'tcx>(
        &mut self,
        instance: Instance<'tcx>,
        tcx: TyCtxt<'tcx>,
        name: &str,
        cache:&mut TyCache,
    ) -> Result<(), MethodCodegenError> {
        if crate::utilis::is_function_magic(name) {
            return Ok(());
        }
        if let TyKind::FnDef(_, _) = instance.ty(tcx, ParamEnv::reveal_all()).kind() {
            //ALL OK.
        } else {
            eprintln!("fn item {instance:?} is not a function definition type. Skippping.");
            return Ok(());
        }

        // Get the MIR if it exisits. Othervise, return early.
        if !tcx.is_mir_available(instance.def_id()) {
            println!("function {instance:?} has no MIR. Skippping.");
            return Ok(());
        }
        
        let mir = tcx.optimized_mir(instance.def_id());
        // Check if function is public or not.
        // FIXME: figure out the source of the bug causing visibility to not be read propely.
        // let access_modifier = AccessModifer::from_visibility(tcx.visibility(instance.def_id()));
        let access_modifier = AccessModifer::Public;
        // Handle the function signature
        let sig = match FnSig::sig_from_instance_(instance, tcx,  cache) {
            Ok(sig) => sig,
            Err(err) => {
                eprintln!("Could not get the signature of function {name} because {err:?}");
                return Ok(());
            }
        };

        // Get locals
        //eprintln!("method")
        let locals = locals_from_mir(
            &mir.local_decls,
            tcx,
            sig.inputs().len(),
            &instance,
            cache,
        );
        // Create method prototype
        let mut method = Method::new(access_modifier, true, sig, name, locals);
        let mut ops = Vec::new();
        let mut last_bb_id = 0;

        let blocks = &(*mir.basic_blocks);
        let does_return_void: bool = *method.sig().output() == Type::Void;
        for block_data in blocks {
            ops.push(CILOp::Label(last_bb_id));
            last_bb_id += 1;
            for statement in &block_data.statements {
                if crate::INSERT_MIR_DEBUG_COMMENTS {
                    rustc_middle::ty::print::with_no_trimmed_paths! {ops.push(CILOp::Comment(format!("{statement:?}").into()))};
                }
                let statement_ops = match Self::statement_to_ops(statement, tcx, mir, instance,cache) {
                    Ok(ops) => ops,
                    Err(err) => {
                        eprintln!(
                            "Method \"{name}\" failed to compile statement with message {err:?}"
                        );
                        CILOp::throw_msg(&format!("Tired to run a statement which failed to compile with error message {err:?}.")).into()
                    }
                };
                crate::utilis::check_debugable(&statement_ops, statement, does_return_void);
                ops.extend(statement_ops);
                if crate::INSERT_MIR_DEBUG_COMMENTS {
                    ops.push(CILOp::Comment("STATEMENT END.".into()));
                }
            }
            match &block_data.terminator {
                Some(term) => {
                    let term_ops = Self::terminator_to_ops(term, mir, tcx, instance,cache);
                    if term_ops != &[CILOp::Ret] {
                        crate::utilis::check_debugable(&term_ops, term, does_return_void);
                    }

                    ops.extend(term_ops)
                }
                None => (),
            }
        }
        method.set_ops(ops);
        // Do some basic checks on the method as a whole.
        crate::utilis::check_debugable(method.get_ops(), &method, does_return_void);
        self.types.extend(cache.defs().cloned());
        println!("Compiled method {name}");
        self.add_method(method);
        Ok(())
        //todo!("Can't add function")
    }
    /// Adds 100 first array types
    pub fn add_array_types(&mut self){
        for i in 0..25{
            self.types.insert(crate::r#type::type_def::get_array_type(i));
        }
    }
    /// Returns true if assembly contains function named `name`
    pub fn contains_fn_named(&self, name: &str) -> bool {
        //FIXME:This is inefficient.
        self.methods().any(|m| m.name() == name)
    }
    /// Adds a method to the assebmly.
    pub fn add_method(&mut self, mut method: Method) {
        method.allocate_temporaries();
        method.ensure_valid();
        self.functions.insert(method);
    }
    /// Returns the list of all calls within the method. Calls may repeat.
    pub fn call_sites(&self) -> impl Iterator<Item = &CallSite> {
        self.methods().map(|method| method.calls()).flatten()
    }
    /// Returns an interator over all methods within the assembly.
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.iter()
    }
    /// Returns an iterator over all types witin the assembly.
    pub fn types(&self) -> impl Iterator<Item = &TypeDef> {
        self.types.iter()
    }
    /// Optimizes all the methods witin the assembly.
    pub fn opt(&mut self) {
        let functions: HashSet<_> = self
            .functions
            .iter()
            .map(|method| {
                let mut method = method.clone();
                crate::opt::opt_method(&mut method, self);
                method
            })
            .collect();
        self.functions = functions;
    }
    /// Adds a definition of a type to the assembly.
    pub fn add_typedef(&mut self, type_def: TypeDef) {
        self.types.insert(type_def);
    }
    /// Adds a MIR item (method,inline assembly code, etc.) to the assembly.
    pub fn add_item<'tcx>(
        &mut self,
        item: MonoItem<'tcx>,
        tcx: TyCtxt<'tcx>,
        cache:&mut TyCache
    ) -> Result<(), CodegenError> {
        
        if !item.is_instantiable(tcx) {
            let name = item.symbol_name(tcx);
            // TODO: check if this whole if statement is even needed.
            eprintln!(
                "WARNING: {name} is not instantiable. Skipping it, since it should not be needed."
            );
            return Ok(());
        }
        match item {
            MonoItem::Fn(instance) => {
                //let instance = crate::utilis::monomorphize(&instance,tcx);
                let symbol_name = crate::utilis::function_name(item.symbol_name(tcx));

                self.checked_add_fn(instance, tcx, &symbol_name,cache)
                    .expect("Could not add function!");

                Ok(())
            }
            MonoItem::GlobalAsm(asm) => {
                eprintln!("Unsuported item - Global ASM:{asm:?}");
                Ok(())
            }
            MonoItem::Static(stotic) => {
                eprintln!("Unsuported item - Static:{stotic:?}");
                Ok(())
            }
        }
    }
    /// Sets the entrypoint of the assembly to the method behind `CallSite`.
    pub fn set_entrypoint(&mut self, entrypoint: CallSite) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        self.functions
            .insert(crate::entrypoint::wrapper(&entrypoint));
        self.entrypoint = Some(entrypoint);
    }
}
/// Returns the list of all local variables within MIR of a function, and converts them to the internal type represenation `Type`
fn locals_from_mir<'tyctx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'tyctx>>,
    tyctx: TyCtxt<'tyctx>,
    argc: usize,
    method_instance: &Instance<'tyctx>,
    tycache: &mut TyCache,
) -> Vec<(Option<IString>, Type)> {
    let mut local_types: Vec<_> = Vec::with_capacity(locals.len());
    for (local_id, local) in locals.iter().enumerate() {
        if local_id == 0 || local_id > argc {
            let ty = crate::utilis::monomorphize(method_instance, local.ty, tyctx);
            let name: Option<IString> = None;
            if crate::PRINT_LOCAL_TYPES {
                println!(
                    "Setting local to type {ty:?},non-morphic: {non_morph}",
                    non_morph = local.ty
                );
            }
            let name = None;
            let tpe = tycache.type_from_cache(ty, tyctx);
            local_types.push((name, tpe));
        }
    }
    local_types
}
