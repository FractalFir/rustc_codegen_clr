use crate::cil_op::{CILOp, CallSite};
use crate::codegen_error::MethodCodegenError;
use crate::utilis::monomorphize;
use crate::{
    access_modifier::AccessModifer, codegen_error::CodegenError, function_sig::FnSig,
    method::Method, r#type::Type, type_def::TypeDef,
};
use rustc_middle::mir::{mono::MonoItem, Local, LocalDecl, Statement, Terminator};
use rustc_middle::ty::{Instance, ParamEnv, TyCtxt, TyKind};
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Assembly {
    types: HashSet<TypeDef>,
    functions: HashSet<Method>,
    entrypoint: Option<CallSite>,
}
impl Assembly {
    /// Creates a new, empty assembly.
    pub fn empty() -> Self {
        Self {
            types: HashSet::new(),
            functions: HashSet::new(),
            entrypoint: None,
        }
    }
    /// Joins 2 assemblies together.
    pub fn join(self, other: Self) -> Self {
        let types = self.types.union(&other.types).cloned().collect();
        let functions = self.functions.union(&other.functions).cloned().collect();
        let entrypoint = self.entrypoint.or(other.entrypoint);
        Self {
            types,
            functions,
            entrypoint,
        }
    }
    pub fn terminator_to_ops<'tcx>(
        term: &Terminator<'tcx>,
        mir: &'tcx rustc_middle::mir::Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        instance: Instance<'tcx>,
    ) -> Vec<CILOp> {
        if crate::ABORT_ON_ERROR {
            crate::terminator::handle_terminator(term, mir, tcx, mir, instance)
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::terminator::handle_terminator(term, mir, tcx, mir, instance)
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
    pub fn statement_to_ops<'tcx>(
        statement: &Statement<'tcx>,
        tcx: TyCtxt<'tcx>,
        mir: &rustc_middle::mir::Body<'tcx>,
        instance: Instance<'tcx>,
    ) -> Result<Vec<CILOp>, CodegenError> {
        if crate::ABORT_ON_ERROR {
            Ok(crate::statement::handle_statement(
                statement, tcx, mir, instance,
            ))
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::statement::handle_statement(statement, tcx, mir, instance)
            })) {
                Ok(success) => Ok(success),
                Err(payload) => {
                    if let Some(msg) = payload.downcast_ref::<&str>() {
                        Err(crate::codegen_error::CodegenError::from_panic_message(msg))
                    } else {
                        Err(crate::codegen_error::CodegenError::from_panic_message(
                            "try_from_poly_sig panicked with a non-string message!",
                        ))
                    }
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
        // TODO: check if this is OK. It seems to work for now, but there may be some edge cases.
        let param_env = ParamEnv::empty();
        // Check if function is public or not.
        // FIXME: figure out the source of the bug causing visibility to not be read propely.
        // let access_modifier = AccessModifer::from_visibility(tcx.visibility(instance.def_id()));
        let access_modifier = AccessModifer::Public;
        // Handle the function signature
        let sig = match FnSig::try_from_poly_sig(
            &instance.ty(tcx, param_env).fn_sig(tcx),
            tcx,
            &instance,
        ) {
            Ok(sig) => sig,
            Err(err) => {
                eprintln!("Could not get the signature of function {name} because {err:?}");
                return Ok(());
            }
        };
        // Get locals
        let locals = locals_from_mir(&mir.local_decls, tcx, sig.inputs().len(), &instance);
        // Create method prototype
        let mut method = Method::new(access_modifier, true, sig, name, locals);
        let mut ops = Vec::new();
        let mut last_bb_id = 0;

        let blocks = &(*mir.basic_blocks);

        for block_data in blocks {
            ops.push(CILOp::Label(last_bb_id));
            last_bb_id += 1;
            for statement in &block_data.statements {
                if crate::INSERT_MIR_DEBUG_COMMENTS {
                    rustc_middle::ty::print::with_no_trimmed_paths! {ops.push(CILOp::Comment(format!("{statement:?}").into()))};
                }
                let statement_ops = match Self::statement_to_ops(statement, tcx, mir, instance) {
                    Ok(ops) => ops,
                    Err(err) => {
                        eprintln!(
                            "Method \"{name}\" failed to compile statement with message {err:?}"
                        );
                        CILOp::throw_msg(&format!("Tired to run a statement which failed to compile with error message {err:?}.")).into()
                    }
                };
                crate::utilis::check_debugable(&statement_ops, statement);
                ops.extend(statement_ops);
                if crate::INSERT_MIR_DEBUG_COMMENTS {
                    ops.push(CILOp::Comment("STATEMENT END.".into()));
                }
                //
                //println!("ops:{ops:?}\n\n");
            }
            match &block_data.terminator {
                Some(term) => {
                    let term_ops = Self::terminator_to_ops(term, mir, tcx, instance);
                    if term_ops != &[CILOp::Ret] {
                        crate::utilis::check_debugable(&term_ops, term);
                    }

                    ops.extend(term_ops)
                }
                None => (),
            }
        }
        method.set_ops(ops);
        for local in &mir.local_decls {
            let local_ty = monomorphize(&instance, local.ty, tcx);
            self.add_type(local_ty, tcx, &instance);
        }
        self.add_method(method);
        Ok(())
        //todo!("Can't add function")
    }
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
    /// Adds rust type `ty` and all types contained within it, if such type is not already present.
    pub fn add_type<'tyctx>(
        &mut self,
        ty: rustc_middle::ty::Ty<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
        method: &Instance<'tyctx>,
    ) {
        for type_def in TypeDef::from_ty(ty, tyctx, method) {
            self.types.insert(type_def);
        }
    }
    /// Optimizes all the methods witin the assembly.
    pub fn opt(&mut self) {
        let functions: HashSet<_> = self
            .functions
            .iter()
            .map(|method| {
                let mut method = method.clone();
                crate::opt::opt_method(&mut method,self);
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
    ) -> Result<(), CodegenError> {
        match item {
            MonoItem::Fn(instance) => {
                //let instance = crate::utilis::monomorphize(&instance,tcx);
                let symbol_name = crate::utilis::function_name(item.symbol_name(tcx));

                self.add_fn(instance, tcx, &symbol_name)
                    .expect("Could not add function!");

                Ok(())
            }
            _ => todo!("Unsupported item:\"{item:?}\"!"),
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
) -> Vec<Type> {
    let mut local_types: Vec<Type> = Vec::with_capacity(locals.len());
    for (local_id, local) in locals.iter().enumerate() {
        if local_id == 0 || local_id > argc {
            let ty = crate::utilis::monomorphize(method_instance, local.ty, tyctx);
            if crate::PRINT_LOCAL_TYPES {
                println!(
                    "Setting local to type {ty:?},non-morphic: {non_morph}",
                    non_morph = local.ty
                );
            }
            local_types.push(Type::from_ty(ty, tyctx, method_instance));
        }
    }
    local_types
}
