use crate::{
    access_modifier::AccessModifer,
    cil::{CILOp, CallSite},
    codegen_error::CodegenError,
    codegen_error::MethodCodegenError,
    function_sig::FnSig,
    method::Method,
    r#type::TyCache,
    r#type::Type,
    r#type::TypeDef,
    IString,
};
use rustc_middle::mir::{
    interpret::{AllocId, GlobalAlloc},
    mono::MonoItem,
    Local, LocalDecl, Statement, Terminator,
};
use rustc_middle::ty::{Instance, ParamEnv, TyCtxt, TyKind};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
/// Data representing a reference to an external assembly.
pub struct AssemblyExternRef {
    /// A tuple describing the referenced assebmly.
    /// Tuple contains:
    /// (Major Version, Minor Version, Revision number, Build number)
    /// In that order.
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
    /// List of types desined within the assembly.
    types: HashSet<TypeDef>,
    /// List of functions defined within this assembly.
    functions: HashMap<CallSite, Method>,
    /// Callsite representing the entrypoint of this assebmly if any present.
    entrypoint: Option<CallSite>,
    /// List of references to external assemblies
    extern_refs: HashMap<IString, AssemblyExternRef>,
    /// List of all static fields within the assembly
    static_fields: HashMap<IString, Type>,
}
impl Assembly {
    /// Returns iterator over all global fields
    pub fn globals(&self) -> impl Iterator<Item = (&IString, &Type)> {
        self.static_fields.iter()
    }
    /// Returns the `.cctor` function used to initialize static data
    pub fn cctor(&self) -> Option<&Method> {
        self.functions.get(&CallSite::new(
            None,
            ".cctor".into(),
            FnSig::new(&[], &Type::Void),
            true,
        ))
    }
    /// Returns the external assembly reference
    pub fn extern_refs(&self) -> &HashMap<IString, AssemblyExternRef> {
        &self.extern_refs
    }
    /// Creates a new, empty assembly.
    pub fn empty() -> Self {
        let mut res = Self {
            types: HashSet::new(),
            functions: HashMap::new(),
            entrypoint: None,
            extern_refs: HashMap::new(),
            static_fields: HashMap::new(),
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
        let static_initializer = link_static_initializers(self.cctor(), other.cctor());
        let types = self.types.union(&other.types).cloned().collect();
        let mut functions = self.functions;
        functions.extend(other.functions);
        if let Some(static_initializer) = static_initializer {
            functions.insert(static_initializer.call_site(), static_initializer);
        }
        let entrypoint = self.entrypoint.or(other.entrypoint);
        let mut extern_refs = self.extern_refs;
        let mut static_fields = self.static_fields;
        static_fields.extend(other.static_fields);
        extern_refs.extend(other.extern_refs);
        Self {
            types,
            functions,
            entrypoint,
            extern_refs,
            static_fields,
        }
    }
    /// Gets the typdefef at path `path`.
    pub fn get_typedef_by_path(&self, path: &str) -> Option<&TypeDef> {
        if path.contains('/') {
            let mut path_iter = path.split('/');
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
        }
        self.types().find(|&tpe| tpe.name() == path)
    }
    /// Turns a terminator into ops, if ABORT_ON_ERROR set to false, will handle and recover from errors.
    pub fn terminator_to_ops<'tcx>(
        term: &Terminator<'tcx>,
        mir: &'tcx rustc_middle::mir::Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        instance: Instance<'tcx>,
        type_cache: &mut TyCache,
    ) -> Vec<CILOp> {
        let mut terminator = if crate::ABORT_ON_ERROR {
            crate::terminator::handle_terminator(term, mir, tcx, mir, instance, type_cache)
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::terminator::handle_terminator(term, mir, tcx, mir, instance, type_cache)
            })) {
                Ok(ok) => ok,
                Err(payload) => {
                    type_cache.recover_from_panic();
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
        };
        let argc = mir.arg_count as u32;
        let locc = mir.local_decls.len() as u32 - mir.arg_count as u32;
        if !crate::utilis::verify_locals_within_range(&terminator, argc, locc) {
            let msg = rustc_middle::ty::print::with_no_trimmed_paths! {format!("{term:?} failed verification, because it refered to local varibles/arguments that do not exist. ops:{terminator:?} argc:{argc} locc:{locc}")};
            eprintln!("WARING: teminator {msg}");
            terminator.clear();
            rustc_middle::ty::print::with_no_trimmed_paths! {terminator.extend(CILOp::throw_msg(&format!(
                "Tried to execute miscompiled terminator {term:?}, which {msg}"
            )))};
        }
        terminator
    }
    /// Turns a statement into ops, if ABORT_ON_ERROR set to false, will handle and recover from errors.
    pub fn statement_to_ops<'tcx>(
        statement: &Statement<'tcx>,
        tcx: TyCtxt<'tcx>,
        mir: &rustc_middle::mir::Body<'tcx>,
        instance: Instance<'tcx>,
        type_cache: &mut TyCache,
    ) -> Result<Vec<CILOp>, CodegenError> {
        if crate::ABORT_ON_ERROR {
            Ok(crate::statement::handle_statement(
                statement, tcx, mir, instance, type_cache,
            ))
        } else {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::statement::handle_statement(statement, tcx, mir, instance, type_cache)
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
        cache: &mut TyCache,
    ) -> Result<(), MethodCodegenError> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.add_fn(instance, tcx, name, cache)
        })) {
            Ok(success) => success,
            Err(payload) => {
                cache.recover_from_panic();
                if let Some(msg) = payload.downcast_ref::<&str>() {
                    eprintln!("could not compile method {name}. fn_add panicked with unhandled message: {msg:?}");
                    //self.add_method(Method::missing_because(format!("could not compile method {name}. fn_add panicked with unhandled message: {msg:?}")));
                    Ok(())
                } else {
                    eprintln!("could not compile method {name}. fn_add panicked with no message.");
                    Ok(())
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
        cache: &mut TyCache,
    ) -> Result<(), MethodCodegenError> {
        if crate::utilis::is_function_magic(name) {
            return Ok(());
        }
        if let TyKind::FnDef(_, _) = instance.ty(tcx, ParamEnv::reveal_all()).kind() {
            //ALL OK.
        } else if let TyKind::Closure(_, _) = instance.ty(tcx, ParamEnv::reveal_all()).kind() {
            println!("CLOSURE")
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
        let sig = match FnSig::sig_from_instance_(instance, tcx, cache) {
            Ok(sig) => sig,
            Err(err) => {
                eprintln!("Could not get the signature of function {name} because {err:?}");
                return Ok(());
            }
        };

        // Get locals
        //eprintln!("method")
        let locals = locals_from_mir(&mir.local_decls, tcx, mir.arg_count, &instance, cache);
        // Create method prototype
        let mut method = Method::new(access_modifier, true, sig, name, locals);
        let mut ops = Vec::new();
        if crate::TRACE_CALLS {
            ops.extend(CILOp::debug_msg(&format!("Called {name}.")));
        }

        let blocks = &mir.basic_blocks;
        let does_return_void: bool = *method.sig().output() == Type::Void;
        for (last_bb_id, block_data) in blocks.into_iter().enumerate() {
            ops.push(CILOp::Label(last_bb_id as u32));
            for statement in &block_data.statements {
                if crate::INSERT_MIR_DEBUG_COMMENTS {
                    rustc_middle::ty::print::with_no_trimmed_paths! {ops.push(CILOp::Comment(format!("{statement:?}").into()))};
                }
                let statement_ops = match Self::statement_to_ops(
                    statement, tcx, mir, instance, cache,
                ) {
                    Ok(ops) => ops,
                    Err(err) => {
                        cache.recover_from_panic();
                        rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!(
                            "Method \"{name}\" failed to compile statement {statement:?} with message {err:?}"
                        )};
                        rustc_middle::ty::print::with_no_trimmed_paths! {CILOp::throw_msg(&format!("Tired to run a statement {statement:?} which failed to compile with error message {err:?}.")).into()}
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
                    if crate::INSERT_MIR_DEBUG_COMMENTS {
                        //rustc_middle::ty::print::with_no_trimmed_paths! {ops.push(CILOp::Comment(format!("{term:?}").into()))};
                    }
                    let term_ops = Self::terminator_to_ops(term, mir, tcx, instance, cache);
                    if term_ops != [CILOp::Ret] {
                        crate::utilis::check_debugable(&term_ops, term, does_return_void);
                    }
                    ops.extend(term_ops)
                }
                None => (),
            }
        }
        #[allow(clippy::single_match)]
        // This will be slowly expanded with support for new types of allocations.
        ops.iter_mut().for_each(|op| match op {
            CILOp::LoadGlobalAllocPtr { alloc_id } => {
                *op = CILOp::LDStaticField(self.add_allocation(*alloc_id, tcx).into());
            }
            _ => (),
        });

        method.set_ops(ops);
        // Do some basic checks on the method as a whole.
        crate::utilis::check_debugable(method.get_ops(), &method, does_return_void);
        self.types.extend(cache.defs().cloned());
        println!("Compiled method {name}");
        self.add_method(method);
        Ok(())
        //todo!("Can't add function")
    }
    /// Adds a global static field named *name* of type *tpe*
    pub fn add_static(&mut self, tpe: Type, name: &str) {
        self.static_fields.insert(name.into(), tpe);
    }

    /// Adds a static field and initialized for allocation represented by `alloc_id`.
    fn add_allocation(
        &mut self,
        alloc_id: u64,
        tcx: TyCtxt<'_>,
    ) -> crate::cil::StaticFieldDescriptor {
        let const_allocation =
            match tcx.global_alloc(AllocId(alloc_id.try_into().expect("0 alloc id?"))) {
                GlobalAlloc::Memory(alloc) => alloc,
                GlobalAlloc::Static(def_id) => {
                    let alloc = tcx.eval_static_initializer(def_id).unwrap();
                    //tcx.reserve_and_set_memory_alloc(alloc)
                    alloc
                }
                GlobalAlloc::Function(_) | GlobalAlloc::VTable(..) => {
                    unreachable!()
                }
            };
        let const_allocation = const_allocation.inner();
        let bytes: &[u8] = const_allocation
            .inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
        //let byte_hash = calculate_hash(&bytes);

        let alloc_fld: IString = format!("alloc_{alloc_id:x}").into();
        let field_desc = crate::cil::StaticFieldDescriptor::new(
            None,
            Type::Ptr(Type::U8.into()),
            alloc_fld.clone(),
        );
        if self.static_fields.get(&alloc_fld).is_none() {
            let cctor = self
                .functions
                .entry(CallSite::new(
                    None,
                    ".cctor".into(),
                    FnSig::new(&[], &Type::Void),
                    true,
                ))
                .or_insert_with(|| {
                    Method::new(
                        AccessModifer::Public,
                        true,
                        FnSig::new(&[], &Type::Void),
                        ".cctor",
                        vec![
                            (None, Type::Ptr(Type::U8.into())),
                            (None, Type::Ptr(Type::U8.into())),
                        ],
                    )
                });

            let ops: &mut Vec<CILOp> = cctor.ops_mut();
            if !ops.is_empty() && ops[ops.len() - 1] == CILOp::Ret {
                ops.pop();
            }
            let init_method = allocation_initializer_method(bytes, &alloc_fld, tcx);
            ops.extend([
                CILOp::Call(CallSite::boxed(
                    None,
                    init_method.name().into(),
                    init_method.sig().clone(),
                    true,
                )),
                CILOp::STStaticField(field_desc.clone().into()),
                CILOp::Ret,
            ]);
            self.add_method(init_method);
            self.add_static(Type::Ptr(Type::U8.into()), &alloc_fld);
        }
        field_desc
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
        self.functions.insert(method.call_site(), method);
    }
    /// Returns the list of all calls within the method. Calls may repeat.
    pub fn call_sites(&self) -> impl Iterator<Item = &CallSite> {
        self.methods().flat_map(|method| method.calls())
    }
    /// Returns an interator over all methods within the assembly.
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.values()
    }
    /// Returns an iterator over all types witin the assembly.
    pub fn types(&self) -> impl Iterator<Item = &TypeDef> {
        self.types.iter()
    }
    /// Optimizes all the methods witin the assembly.
    pub fn opt(&mut self) {
        let functions: HashMap<_, _> = self
            .functions
            .iter()
            .map(|method| {
                let (site, method) = method;
                let mut method = method.clone();
                crate::opt::opt_method(&mut method, self);
                (site.clone(), method)
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
        cache: &mut TyCache,
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

                self.checked_add_fn(instance, tcx, &symbol_name, cache)
                    .expect("Could not add function!");

                Ok(())
            }
            MonoItem::GlobalAsm(asm) => {
                eprintln!("Unsuported item - Global ASM:{asm:?}");
                Ok(())
            }
            MonoItem::Static(stotic) => {
                let alloc = tcx.eval_static_initializer(stotic).unwrap();
                let alloc_id = tcx.reserve_and_set_memory_alloc(alloc);
                self.add_allocation(crate::utilis::alloc_id_to_u64(alloc_id), tcx);
                //eprintln!("Unsuported item - Static:{stotic:?}");
                Ok(())
            }
        }
    }
    /// Sets the entrypoint of the assembly to the method behind `CallSite`.
    pub fn set_entrypoint(&mut self, entrypoint: CallSite) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        let wrapper = crate::entrypoint::wrapper(&entrypoint);
        self.functions.insert(wrapper.call_site(), wrapper);
        self.entrypoint = Some(entrypoint);
    }
}
fn link_static_initializers(a: Option<&Method>, b: Option<&Method>) -> Option<Method> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (Some(a), Some(b)) => {
            let mut merged: Method = a.clone();
            let ops = merged.ops_mut();
            if !ops.is_empty() && ops[ops.len() - 1] == CILOp::Ret {
                ops.pop();
            }
            ops.extend(b.get_ops().iter().cloned());
            Some(merged)
        }
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
            if crate::PRINT_LOCAL_TYPES {
                println!(
                    "Local type {ty:?},non-morphic: {non_morph}",
                    non_morph = local.ty
                );
            }
            let name = None;
            let tpe = tycache.type_from_cache(ty, tyctx, Some(*method_instance));
            local_types.push((name, tpe));
        }
    }
    local_types
}
fn allocation_initializer_method(bytes: &[u8], name: &str, tyctx: TyCtxt) -> Method {
    let mut ops = Vec::new();
    ops.extend([
        CILOp::LdcI64(bytes.len() as u64 as i64),
        CILOp::ConvISize(false),
        CILOp::Call(CallSite::malloc(tyctx).into()),
        CILOp::Dup,
        CILOp::STLoc(0),
        CILOp::STLoc(1),
    ]);
    for byte in bytes {
        ops.extend([
            CILOp::LDLoc(0),
            CILOp::LdcI32(*byte as i32),
            CILOp::STIndI8,
            CILOp::LDLoc(0),
            CILOp::LdcI32(1),
            CILOp::Add,
            CILOp::STLoc(0),
            CILOp::Comment(name.clone().into()),
        ]);
    }
    ops.extend([CILOp::LDLoc(1), CILOp::Ret]);
    let mut method = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[], &Type::Ptr(Type::U8.into())),
        &format!("init_{name}"),
        vec![
            (Some("curr".into()), Type::Ptr(Type::U8.into())),
            (Some("alloc_ptr".into()), Type::Ptr(Type::U8.into())),
        ],
    );
    method.set_ops(ops);
    method
}
