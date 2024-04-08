use crate::basic_block::{handler_for_block, BasicBlock};
use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;
use crate::cil_tree::CILTree;
use crate::method::MethodType;
use crate::rustc_middle::dep_graph::DepContext;
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
use crate::{add, call, conv_isize, conv_usize, ldc_u32, ldc_u64};
use rustc_middle::mir::interpret::Allocation;
use rustc_middle::mir::{
    interpret::{AllocId, GlobalAlloc},
    mono::MonoItem,
    Local, LocalDecl, Statement, Terminator,
};
use rustc_middle::ty::{Instance, ParamEnv, TyCtxt, TyKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    types: HashMap<IString, TypeDef>,
    /// List of functions defined within this assembly.
    functions: HashMap<CallSite, Method>,
    /// Callsite representing the entrypoint of this assebmly if any present.
    entrypoint: Option<CallSite>,
    /// List of references to external assemblies
    extern_refs: HashMap<IString, AssemblyExternRef>,
    extern_fns: HashMap<(IString, FnSig), IString>,
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
            types: HashMap::new(),
            functions: HashMap::new(),
            entrypoint: None,
            extern_refs: HashMap::new(),
            static_fields: HashMap::new(),
            extern_fns: HashMap::new(),
        };
        let dotnet_ver = AssemblyExternRef {
            version: (6, 12, 0, 0),
        };
        res.extern_refs.insert("System.Runtime".into(), dotnet_ver);
        //res.extern_refs.insert("mscorlib".into(),dotnet_ver);
        res.extern_refs
            .insert("System.Runtime.InteropServices".into(), dotnet_ver);
        // Needed to get C-Mode to work
        res.add_cctor();
        res
    }
    /// Joins 2 assemblies together.
    pub fn join(self, other: Self) -> Self {
        let static_initializer = link_static_initializers(self.cctor(), other.cctor());
        let mut types = self.types;
        types.extend(other.types);
        let mut functions = self.functions;
        functions.extend(other.functions);
        if let Some(static_initializer) = static_initializer {
            functions.insert(static_initializer.call_site(), static_initializer);
        }
        let entrypoint = self.entrypoint.or(other.entrypoint);
        let mut extern_refs = self.extern_refs;
        let mut static_fields = self.static_fields;
        let mut extern_fns = self.extern_fns;
        static_fields.extend(other.static_fields);
        extern_refs.extend(other.extern_refs);
        extern_fns.extend(other.extern_fns);
        Self {
            types,
            functions,
            entrypoint,
            extern_refs,
            static_fields,
            extern_fns,
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
        self.types()
            .find(|&tpe| tpe.0.as_ref() == path)
            .map(|t| t.1)
    }
    /// Turns a terminator into ops, if ABORT_ON_ERROR set to false, will handle and recover from errors.
    pub fn terminator_to_ops<'tcx>(
        term: &Terminator<'tcx>,
        mir: &'tcx rustc_middle::mir::Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        instance: Instance<'tcx>,
        type_cache: &mut TyCache,
    ) -> Vec<CILTree> {
        let terminator = if *crate::config::ABORT_ON_ERROR {
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
                    CILRoot::throw(&msg).into()
                }
            }
        };
        /*
        if !crate::utilis::verify_locals_within_range(&terminator, argc, locc) {
            let msg = rustc_middle::ty::print::with_no_trimmed_paths! {format!("{term:?} failed verification, because it refered to local varibles/arguments that do not exist. ops:{terminator:?} argc:{argc} locc:{locc}")};
            eprintln!("WARING: teminator {msg}");
            terminator.clear();
            rustc_middle::ty::print::with_no_trimmed_paths! {terminator.extend(CILOp::throw_msg(&format!(
                "Tried to execute miscompiled terminator {term:?}, which {msg}"
            )))};
        }*/
        terminator
    }
    /// Turns a statement into ops, if ABORT_ON_ERROR set to false, will handle and recover from errors.
    pub fn statement_to_ops<'tcx>(
        statement: &Statement<'tcx>,
        tcx: TyCtxt<'tcx>,
        mir: &rustc_middle::mir::Body<'tcx>,
        instance: Instance<'tcx>,
        type_cache: &mut TyCache,
    ) -> Result<Option<CILTree>, CodegenError> {
        if *crate::config::ABORT_ON_ERROR {
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
    pub fn add_fn<'tyctx>(
        &mut self,
        instance: Instance<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
        name: &str,
        cache: &mut TyCache,
    ) -> Result<(), MethodCodegenError> {
        if crate::utilis::is_function_magic(name) {
            return Ok(());
        }
        if let TyKind::FnDef(_, _) = instance.ty(tyctx, ParamEnv::reveal_all()).kind() {
            //ALL OK.
        } else if let TyKind::Closure(_, _) = instance.ty(tyctx, ParamEnv::reveal_all()).kind() {
            //println!("CLOSURE")
        } else {
            eprintln!("fn item {instance:?} is not a function definition type. Skippping.");
            return Ok(());
        }
        let mir = tyctx.instance_mir(instance.def);
        // Check if function is public or not.
        // FIXME: figure out the source of the bug causing visibility to not be read propely.
        // let access_modifier = AccessModifer::from_visibility(tcx.visibility(instance.def_id()));
        let access_modifier = AccessModifer::Public;
        // Handle the function signature
        let call_site = crate::call_info::CallInfo::sig_from_instance_(instance, tyctx, cache);
        let sig = match call_site {
            Ok(call_site) => call_site.sig().clone(),
            Err(err) => {
                eprintln!("Could not get the signature of function {name} because {err:?}");
                return Ok(());
            }
        };

        // Get locals
        //eprintln!("method")
        let locals = locals_from_mir(&mir.local_decls, tyctx, mir.arg_count, &instance, cache);
        // Create method prototype

        let mut ops = Vec::new();
        if *crate::config::TRACE_CALLS {
            ops.extend(CILOp::debug_msg(&format!("Called {name}.")));
        }

        let blocks = &mir.basic_blocks;
        //let mut trees = Vec::new();
        let mut normal_bbs = Vec::new();
        let mut cleanup_bbs = Vec::new();
        for (last_bb_id, block_data) in blocks.into_iter().enumerate() {
            //ops.push(CILOp::Label(last_bb_id as u32));
            let mut trees = Vec::new();
            for statement in &block_data.statements {
                if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                    rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{statement:?}")).into())};
                }
                trees.extend(match Self::statement_to_ops(
                    statement, tyctx, mir, instance, cache,
                ) {
                    Ok(ops) => ops,
                    Err(err) => {
                        cache.recover_from_panic();
                        rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!(
                            "Method \"{name}\" failed to compile statement {statement:?} with message {err:?}"
                        )};
                        rustc_middle::ty::print::with_no_trimmed_paths! {Some(CILRoot::throw(&format!("Tired to run a statement {statement:?} which failed to compile with error message {err:?}.")).into())}
                    }
                });

                //crate::utilis::check_debugable(&statement_ops, statement, does_return_void);
                //ops.extend(statement_ops);
                //
            }
            match &block_data.terminator {
                Some(term) => {
                    if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                        rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{term:?}")).into())};
                    }
                    let term_trees = Self::terminator_to_ops(term, mir, tyctx, instance, cache);
                    trees.extend(term_trees);
                }
                None => (),
            }
            if block_data.is_cleanup {
                cleanup_bbs.push(BasicBlock::new(
                    trees,
                    last_bb_id as u32,
                    handler_for_block(block_data, &mir.basic_blocks, tyctx, &instance, mir),
                ))
            } else {
                normal_bbs.push(BasicBlock::new(
                    trees,
                    last_bb_id as u32,
                    handler_for_block(block_data, &mir.basic_blocks, tyctx, &instance, mir),
                ));
            }
            //ops.extend(trees.iter().flat_map(|tree| tree.flatten()))
        }
        normal_bbs
            .iter_mut()
            .for_each(|bb| bb.resolve_exception_handlers(&cleanup_bbs));
        #[allow(clippy::single_match)]
        // This will be slowly expanded with support for new types of allocations.
        let mut method = Method::new(
            access_modifier,
            MethodType::Static,
            sig.clone(),
            name,
            locals,
            normal_bbs,
        );
        method.resolve_global_allocations(self, tyctx);

        //println!("Compiled method {name}");

        self.add_method(method);
        Ok(())
        //todo!("Can't add function")
    }
    /// Adds a global static field named *name* of type *tpe*
    pub fn add_static(&mut self, tpe: Type, name: &str) {
        self.static_fields.insert(name.into(), tpe);
    }
    fn add_cctor(&mut self) -> &mut Method {
        self.functions
            .entry(CallSite::new(
                None,
                ".cctor".into(),
                FnSig::new(&[], &Type::Void),
                true,
            ))
            .or_insert_with(|| {
                Method::new(
                    AccessModifer::Public,
                    MethodType::Static,
                    FnSig::new(&[], &Type::Void),
                    ".cctor",
                    vec![
                        (None, Type::Ptr(Type::U8.into())),
                        (None, Type::Ptr(Type::U8.into())),
                    ],
                    vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
                )
            })
    }
    /// Adds a static field and initialized for allocation represented by `alloc_id`.
    pub fn add_allocation(
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
                GlobalAlloc::VTable(..) => {
                    //TODO: handle VTables
                    let alloc_fld: IString = format!("alloc_{alloc_id:x}").into();
                    let field_desc = crate::cil::StaticFieldDescriptor::new(
                        None,
                        Type::Ptr(Type::U8.into()),
                        alloc_fld.clone(),
                    );
                    self.static_fields
                        .insert(alloc_fld, Type::Ptr(Type::U8.into()));
                    return field_desc;
                }
                GlobalAlloc::Function(_) => {
                    //TODO: handle constant functions
                    let alloc_fld: IString = format!("alloc_{alloc_id:x}").into();
                    let field_desc = crate::cil::StaticFieldDescriptor::new(
                        None,
                        Type::Ptr(Type::U8.into()),
                        alloc_fld.clone(),
                    );
                    self.static_fields
                        .insert(alloc_fld, Type::Ptr(Type::U8.into()));
                    return field_desc;
                    //todo!("Function/Vtable allocation.");
                }
            };

        let const_allocation = const_allocation.inner();

        let bytes: &[u8] = const_allocation
            .inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
        /// Alloc ids are *not* unique across all crates. Adding the hash here ensures we don't overwrite allocations during linking
        /// TODO:consider using something better here / making the hashes stable.
        let byte_hash = calculate_hash(&bytes);
        let alloc_fld: IString = format!("alloc_{alloc_id:x}_{byte_hash:x}").into();

        let field_desc = crate::cil::StaticFieldDescriptor::new(
            None,
            Type::Ptr(Type::U8.into()),
            alloc_fld.clone(),
        );
        if self.static_fields.get(&alloc_fld).is_none() {
            let init_method =
                allocation_initializer_method(const_allocation, &alloc_fld, tcx, self);
            let cctor = self.add_cctor();
            let mut blocks = cctor.blocks_mut();
            if blocks.is_empty() {
                blocks.push(BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None));
            }
            assert_eq!(
                blocks.len(),
                1,
                "Unexpected number of basic blocks in a static data initializer."
            );
            let trees = blocks[0].trees_mut();
            {
                // Remove return
                let ret = trees.pop().unwrap();
                // Append initailzer
                trees.push(
                    CILRoot::SetStaticField {
                        descr: field_desc.clone(),
                        value: call!(
                            CallSite::new(
                                None,
                                init_method.name().into(),
                                init_method.sig().clone(),
                                true,
                            ),
                            []
                        ),
                    }
                    .into(),
                );
                // Add return again
                trees.push(ret);
            }
            drop(blocks);
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
    /// Returns true if assembly contains function named `name`
    pub fn contains_fn(&self, site: &CallSite) -> bool {
        self.functions.contains_key(site)
    }
    /// Adds a method to the assebmly.
    pub fn add_method(&mut self, mut method: Method) {
        method.allocate_temporaries();
        //method.ensure_valid();
        if *crate::config::VERIFY_METHODS {
            //crate::verify::verify(&method);
        }

        self.functions.insert(method.call_site(), method);
    }
    /// Returns the list of all calls within the method. Calls may repeat.
    pub fn call_sites(&self) -> Vec<CallSite> {
        self.methods().flat_map(|method| method.calls()).collect()
    }
    /// Returns an interator over all methods within the assembly.
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.values()
    }
    /// Returns an iterator over all types witin the assembly.
    pub fn types(&self) -> impl Iterator<Item = (&IString, &TypeDef)> {
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
        self.types.insert(type_def.name().into(), type_def);
    }
    /// Adds a MIR item (method,inline assembly code, etc.) to the assembly.
    pub fn add_item<'tcx>(
        &mut self,
        item: MonoItem<'tcx>,
        tcx: TyCtxt<'tcx>,
        cache: &mut TyCache,
    ) -> Result<(), CodegenError> {
        /*if !item.is_instantiable(tcx) {
            let name = item.symbol_name(tcx);
            // TODO: check if this whole if statement is even needed.
            eprintln!(
                "WARNING: {name} is not instantiable. Skipping it, since it should not be needed."
            );
            return Ok(());
        }*/
        match item {
            MonoItem::Fn(instance) => {
                //let instance = crate::utilis::monomorphize(&instance,tcx);
                let symbol_name = crate::utilis::function_name(item.symbol_name(tcx));
                if symbol_name.contains("map_or_else") {
                    println!(
                        "Defining function with name {symbol_name}. DefId:{:?}, Subst:{:?}",
                        instance.def, instance.args
                    );
                    let sig = crate::utilis::monomorphize(
                        &instance,
                        instance.ty(tcx, ParamEnv::reveal_all()).fn_sig(tcx),
                        tcx,
                    );
                    rustc_middle::ty::print::with_no_trimmed_paths! {println!("map_or_else_sig:{:?}",sig)};
                    let call_info =
                        crate::call_info::CallInfo::sig_from_instance_(instance, tcx, cache)
                            .expect("Could not resolve function sig");
                    for input in call_info.sig().inputs() {
                        println!("\t{input:?}");
                    }
                }

                let function_compile_timer = tcx.profiler().generic_activity_with_arg(
                    "compile function",
                    item.symbol_name(tcx).to_string(),
                );
                self.checked_add_fn(instance, tcx, &symbol_name, cache)
                    .expect("Could not add function!");
                drop(function_compile_timer);
                Ok(())
            }
            MonoItem::GlobalAsm(asm) => {
                eprintln!("Unsuported item - Global ASM:{asm:?}");
                Ok(())
            }
            MonoItem::Static(stotic) => {
                let static_compile_timer = tcx.profiler().generic_activity_with_arg(
                    "compile static initializer",
                    item.symbol_name(tcx).to_string(),
                );
                let alloc = tcx.eval_static_initializer(stotic).unwrap();
                let alloc_id = tcx.reserve_and_set_memory_alloc(alloc);
                self.add_allocation(crate::utilis::alloc_id_to_u64(alloc_id), tcx);
                //let ty = alloc.0;
                drop(static_compile_timer);
                //eprintln!("Unsuported item - Static:{stotic:?}");
                Ok(())
            }
        }
    }
    /// Sets the entrypoint of the assembly to the method behind `CallSite`.
    pub fn set_entrypoint(&mut self, entrypoint: CallSite) {
        assert!(self.entrypoint.is_none(), "ERROR: Multiple entrypoints");
        let wrapper = crate::entrypoint::wrapper(&entrypoint);
        self.entrypoint = Some(wrapper.call_site());
        self.add_method(wrapper);
    }

    pub fn extern_fns(&self) -> &HashMap<(IString, FnSig), IString> {
        &self.extern_fns
    }

    pub fn add_extern_fn(&mut self, name: IString, sig: FnSig, lib: IString) {
        self.extern_fns.insert((name, sig), lib);
    }
    fn get_exported_fn(&self) -> HashMap<CallSite, Method> {
        let mut externs = HashMap::new();
        if let Some(entrypoint) = &self.entrypoint {
            let method = self.functions.get(entrypoint).cloned().unwrap();
            externs.insert(entrypoint.clone(), method);
        }
        if let Some(cctor) = self.cctor() {
            externs.insert(
                CallSite::new(None, ".cctor".into(), FnSig::new(&[], &Type::Void), true),
                cctor.clone(),
            );
        }
        externs
    }
    pub fn eliminate_dead_fn(&mut self) {
        let mut alive: HashMap<CallSite, Method> = HashMap::new();
        let mut resurecting: HashMap<CallSite, Method> = HashMap::new();
        let mut to_resurect: HashMap<CallSite, Method> = self.get_exported_fn();
        while !to_resurect.is_empty() {
            alive.extend(resurecting.clone());
            resurecting.clear();
            resurecting.extend(to_resurect.clone());
            to_resurect.clear();
            for call in resurecting.iter().flat_map(|fnc| fnc.1.calls()) {
                if let Some(_class) = call.class() {
                    // TODO: if dead code elimination too agressive check this
                    // Methods reference by methods inside types are NOT tracked.
                    continue;
                }
                if alive.contains_key(&call) || resurecting.contains_key(&call) {
                    // Already alive, ignore!
                    continue;
                }
                if let Some(method) = self.functions.get(&call).cloned() {
                    to_resurect.insert(call.clone(), method);
                };
            }
        }
        alive.extend(resurecting);
        self.functions = alive;
    }
    pub fn eliminate_dead_code(&mut self) {
        self.eliminate_dead_fn();
        //self.eliminate_dead_types();
    }
    pub fn eliminate_dead_types(&mut self) {
        let mut alive = HashMap::new();
        let mut resurected: HashMap<IString, _> = self
            .functions
            .values()
            .flat_map(|fnc| fnc.dotnet_types())
            .filter_map(|tpe| match tpe.asm() {
                Some(_) => None,
                None => Some(IString::from(tpe.name_path())),
            })
            .map(|name| (name.clone(), self.types.get(&name).unwrap().clone()))
            .collect();
        resurected.insert(
            "RustVoid".into(),
            self.types.get("RustVoid").cloned().unwrap(),
        );
        let mut to_resurect: HashMap<IString, _> = HashMap::new();
        while !resurected.is_empty() {
            for tpe in &resurected {
                alive.insert(tpe.0.clone(), tpe.1.clone());
                for (name, type_def) in tpe
                    .1
                    .all_types()
                    .filter_map(|tpe| tpe.dotnet_refs())
                    .filter_map(|tpe| match tpe.asm() {
                        Some(_) => None,
                        None => Some(IString::from(tpe.name_path())),
                    })
                    .filter_map(|name| name.split_once('\\').map(|(a, _)| a.into()))
                    //.map(|(a,b)|a.into())
                    .map(|name: IString| {
                        (
                            name.to_owned(),
                            self.types
                                .get(&name)
                                .unwrap_or_else(|| panic!("Can't find type {name:?}"))
                                .clone(),
                        )
                    })
                {
                    let name: IString = IString::from(name);
                    to_resurect.insert(name, type_def);
                }
            }
            resurected = to_resurect;
            to_resurect = HashMap::new();
        }
        self.types = alive;
    }
}
fn link_static_initializers(a: Option<&Method>, b: Option<&Method>) -> Option<Method> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (Some(a), Some(b)) => {
            let mut merged: Method = a.clone();
            let mut blocks = merged.blocks_mut();
            let trees = blocks[0].trees_mut();
            trees.pop();
            trees.extend(b.blocks()[0].trees().iter().cloned());
            drop(blocks);
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
            if *crate::config::PRINT_LOCAL_TYPES {
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

fn allocation_initializer_method(
    const_allocation: &Allocation,
    name: &str,
    tyctx: TyCtxt,
    asm: &mut Assembly,
) -> Method {
    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let ptrs = const_allocation.provenance().ptrs();
    let mut trees: Vec<CILTree> = Vec::new();
    trees.push(
        CILRoot::STLoc {
            local: 0,
            tree: call!(
                CallSite::malloc(tyctx),
                [conv_isize!(ldc_u64!(bytes.len() as u64))]
            ),
        }
        .into(),
    );
    trees.push(
        CILRoot::STLoc {
            local: 1,
            tree: CILNode::LDLoc(0),
        }
        .into(),
    );
    for byte in bytes {
        trees.push(CILRoot::STIndI8(CILNode::LDLoc(0), ldc_u32!(*byte as u32)).into());
        trees.push(
            CILRoot::STLoc {
                local: 0,
                tree: add!(CILNode::LDLoc(0), conv_usize!(ldc_u32!(1))),
            }
            .into(),
        )
    }
    if !ptrs.is_empty() {
        for ptr in ptrs.iter() {
            let ptr_alloc = asm.add_allocation(ptr.1.alloc_id().0.into(), tyctx);
            let offset = ptr.0.bytes_usize() as u32;
            trees.push(
                CILRoot::STIndISize(
                    add!(CILNode::LDLoc(1), conv_usize!(ldc_u32!(offset))),
                    CILNode::LDStaticField(ptr_alloc.into()),
                )
                .into(),
            )
        }
        //eprintln!("Constant requires rellocation support!");
    }
    trees.push(
        CILRoot::Ret {
            tree: CILNode::LDLoc(1),
        }
        .into(),
    );

    Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[], &Type::Ptr(Type::U8.into())),
        &format!("init_{name}"),
        vec![
            (Some("curr".into()), Type::Ptr(Type::U8.into())),
            (Some("alloc_ptr".into()), Type::Ptr(Type::U8.into())),
        ],
        vec![BasicBlock::new(trees, 0, None)],
    )
}
fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::DefaultHasher;
    use std::hash::Hasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
