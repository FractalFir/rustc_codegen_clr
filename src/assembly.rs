use crate::{
    basic_block::handler_for_block,
    cil::span_source_info,
    codegen_error::{CodegenError, MethodCodegenError},
    r#type::{TyCache, Type},
    rustc_middle::dep_graph::DepContext,
    utilis::field_descrptor,
    IString,
};
use cilly::{
    access_modifier::AccessModifer,
    asm::Assembly,
    basic_block::BasicBlock,
    call,
    call_site::CallSite,
    cil_node::{CILNode, ValidationContext},
    cil_root::CILRoot,
    cil_tree::CILTree,
    conv_isize, conv_usize, ldc_u32, ldc_u64,
    method::{Method, MethodType},
    static_field_desc::StaticFieldDescriptor,
    FnSig,
};
use rustc_middle::{
    mir::{
        interpret::{AllocId, Allocation, GlobalAlloc},
        mono::MonoItem,
        Local, LocalDecl, Statement, Terminator,
    },
    ty::{Instance, ParamEnv, TyCtxt, TyKind},
};

type LocalDefList = Vec<(Option<IString>, Type)>;
type ArgsDebugInfo = Vec<Option<IString>>;
fn check_align_adjust<'tyctx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'tyctx>>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: &Instance<'tyctx>,
) -> Vec<Option<u64>> {
    let mut adjusts: Vec<Option<u64>> = Vec::with_capacity(locals.len());
    for local in locals {
        let ty = crate::utilis::monomorphize(method_instance, local.ty, tyctx);
        let adjust = crate::utilis::requries_align_adjustement(ty, tyctx);
        adjusts.push(adjust);
    }
    adjusts
}
/// Returns the list of all local variables within MIR of a function, and converts them to the internal type represenation `Type`
fn locals_from_mir<'tyctx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'tyctx>>,
    tyctx: TyCtxt<'tyctx>,
    argc: usize,
    method_instance: &Instance<'tyctx>,
    tycache: &mut TyCache,
    var_debuginfo: &[rustc_middle::mir::VarDebugInfo<'tyctx>],
) -> (ArgsDebugInfo, LocalDefList) {
    use rustc_middle::mir::VarDebugInfoContents;
    let mut local_types: Vec<(Option<IString>, _)> = Vec::with_capacity(locals.len());
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
            let tpe = tycache.type_from_cache(ty, tyctx, *method_instance);
            local_types.push((name, tpe));
        }
    }
    let mut arg_names: Vec<Option<IString>> = (0..argc).map(|_| None).collect();
    for var in var_debuginfo {
        let mir_local = match var.value {
            VarDebugInfoContents::Place(place) => {
                // Check if this is just a "naked" local(eg. just a local varaible, with no indirction)
                if !place.projection.is_empty() {
                    continue;
                }
                place.local.as_usize()
            }
            VarDebugInfoContents::Const(_) => continue,
        };
        if mir_local == 0 {
            local_types[0].0 = Some(var.name.to_string().into());
        } else if mir_local > argc {
            local_types[mir_local - argc].0 = Some(var.name.to_string().into());
        } else {
            arg_names[mir_local - 1] = Some(var.name.to_string().into());
        }
    }
    (arg_names, local_types)
}

fn allocation_initializer_method(
    const_allocation: &Allocation,
    name: &str,
    tyctx: TyCtxt,
    asm: &mut Assembly,
    tycache: &mut TyCache,
) -> Method {
    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let ptrs = const_allocation.provenance().ptrs();
    let mut trees: Vec<CILTree> = Vec::new();
    let align = const_allocation.align.bytes().max(1);
    //trees.push(CILRoot::debug(&format!("Preparing to initialize allocation with size {}",bytes.len())).into());
    trees.push(
        CILRoot::STLoc {
            local: 0,
            tree: CILNode::TransmutePtr {
                val: Box::new(call!(
                    CallSite::alloc(),
                    [
                        conv_isize!(ldc_u64!(bytes.len() as u64)),
                        conv_isize!(ldc_u64!(align))
                    ]
                )),
                new_ptr: Box::new(Type::Ptr(Box::new(Type::U8))),
            },
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
        trees.push(CILRoot::STIndI8(CILNode::LDLoc(0), ldc_u32!(u32::from(*byte))).into());
        //trees.push(CILRoot::debug(&format!("Writing the byte {}",byte)).into());
        trees.push(
            CILRoot::STLoc {
                local: 0,
                tree: CILNode::LDLoc(0) + conv_usize!(ldc_u32!(1)),
            }
            .into(),
        );
    }
    if !ptrs.is_empty() {
        for (offset, prov) in ptrs.iter() {
            let offset = u32::try_from(offset.bytes_usize()).unwrap();
            // Check if this allocation is a function
            let reloc_target_alloc = tyctx.global_alloc(prov.alloc_id());
            if let GlobalAlloc::Function(finstance) = reloc_target_alloc {
                // If it is a function, patch its pointer up.
                let call_info =
                    crate::call_info::CallInfo::sig_from_instance_(finstance, tyctx, tycache);
                let function_name = crate::utilis::function_name(tyctx.symbol_name(finstance));

                trees.push(
                    CILRoot::STIndISize(
                        CILNode::LDLoc(1) + conv_usize!(ldc_u32!(offset)),
                        CILNode::LDFtn(
                            CallSite::new(None, function_name, call_info.sig().clone(), true)
                                .into(),
                        ),
                    )
                    .into(),
                );
            } else {
                let ptr_alloc = add_allocation(asm, prov.alloc_id().0.into(), tyctx, tycache);

                trees.push(
                    CILRoot::STIndISize(
                        CILNode::LDLoc(1) + conv_usize!(ldc_u32!(offset)),
                        CILNode::LDStaticField(ptr_alloc.into()),
                    )
                    .into(),
                );
            }
        }
        //eprintln!("Constant requires rellocation support!");
    }
    //trees.push(CILRoot::debug(&format!("Finished initializing an allocation with size {}",bytes.len())).into());
    trees.push(
        CILRoot::Ret {
            tree: CILNode::LDLoc(1),
        }
        .into(),
    );

    Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[], Type::Ptr(Type::U8.into())),
        &format!("init_{name}"),
        vec![
            (Some("curr".into()), Type::Ptr(Type::U8.into())),
            (Some("alloc_ptr".into()), Type::Ptr(Type::U8.into())),
        ],
        vec![BasicBlock::new(trees, 0, None)],
        vec![],
    )
}
fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::DefaultHasher;
    use std::hash::Hasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
/// Turns a terminator into ops, if `ABORT_ON_ERROR` set to false, will handle and recover from errors.
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
                vec![CILRoot::throw(&msg).into()]
            }
        }
    };

    terminator
}
/// Turns a statement into ops, if `ABORT_ON_ERROR` set to false, will handle and recover from errors.
pub fn statement_to_ops<'tcx>(
    statement: &Statement<'tcx>,
    tcx: TyCtxt<'tcx>,
    mir: &rustc_middle::mir::Body<'tcx>,
    instance: Instance<'tcx>,
    type_cache: &mut TyCache,
    validation_context: ValidationContext,
) -> Result<Option<CILTree>, CodegenError> {
    if *crate::config::ABORT_ON_ERROR {
        Ok(crate::statement::handle_statement(
            statement, tcx, mir, instance, type_cache,
        ))
    } else {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::statement::handle_statement(statement, tcx, mir, instance, type_cache)
        })) {
            Ok(success) => {
                match &success {
                    Some(ops) => {
                        if let Err(msg) = ops.validate(validation_context) {
                            Err(crate::codegen_error::CodegenError::from_panic_message(
                                &format!("VERIFICATION FALIURE:\"{msg}\" ops:{ops:?}"),
                            ))?;
                        }
                    }
                    None => (),
                }
                Ok(success)
            }
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
/// Adds a rust MIR function to the assembly.
pub fn add_fn<'tyctx>(
    asm: &mut Assembly,
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
    let timer = tyctx.prof.generic_activity_with_arg("codegen fn", name);
    // Check if function is public or not.
    // FIXME: figure out the source of the bug causing visibility to not be read propely.
    // let access_modifier = AccessModifer::from_visibility(tcx.visibility(instance.def_id()));
    let access_modifier = AccessModifer::Public;
    // Handle the function signature
    let call_site = crate::call_info::CallInfo::sig_from_instance_(instance, tyctx, cache);
    let sig = call_site.sig().clone();

    // Get locals
    let (arg_names, mut locals) = locals_from_mir(
        &mir.local_decls,
        tyctx,
        mir.arg_count,
        &instance,
        cache,
        &mir.var_debug_info,
    );
    // Used for type-checking the CIL to ensure its validity.
    let validation_context = ValidationContext::new(&sig, &locals);

    let blocks = &mir.basic_blocks;
    let mut normal_bbs = Vec::new();
    let mut cleanup_bbs = Vec::new();
    for (last_bb_id, block_data) in blocks.into_iter().enumerate() {
        let mut trees = Vec::new();
        for statement in &block_data.statements {
            if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{statement:?}")).into())};
            }

            let statement_tree = match statement_to_ops(
                statement,
                tyctx,
                mir,
                instance,
                cache,
                validation_context,
            ) {
                Ok(ops) => ops,
                Err(err) => {
                    cache.recover_from_panic();
                    rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!(
                        "Method \"{name}\" failed to compile statement {statement:?} with message {err:?}\n"
                    )};
                    rustc_middle::ty::print::with_no_trimmed_paths! {Some(CILRoot::throw(&format!("Tired to run a statement {statement:?} which failed to compile with error message {err:?}.")).into())}
                }
            };
            // Only save debuginfo for statements which result in ops.
            if statement_tree.is_some() {
                trees.push(span_source_info(tyctx, statement.source_info.span).into());
            }
            trees.extend(statement_tree);

            //crate::utilis::check_debugable(&statement_ops, statement, does_return_void);
            //ops.extend(statement_ops);
            //
        }
        match &block_data.terminator {
            Some(term) => {
                if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                    rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{term:?}")).into())};
                }
                let term_trees = terminator_to_ops(term, mir, tyctx, instance, cache);
                if !term_trees.is_empty() {
                    trees.push(span_source_info(tyctx, term.source_info.span).into());
                }
                trees.extend(term_trees);
            }
            None => (),
        }
        if block_data.is_cleanup {
            cleanup_bbs.push(BasicBlock::new(
                trees,
                u32::try_from(last_bb_id).unwrap(),
                handler_for_block(block_data, &mir.basic_blocks, tyctx, &instance, mir),
            ));
        } else {
            normal_bbs.push(BasicBlock::new(
                trees,
                u32::try_from(last_bb_id).unwrap(),
                handler_for_block(block_data, &mir.basic_blocks, tyctx, &instance, mir),
            ));
        }
        //ops.extend(trees.iter().flat_map(|tree| tree.flatten()))
    }
    if let Some(spread_arg) = mir.spread_arg {
        // Prepare for repacking the argument tuple, by allocating a local
        let repacked = u32::try_from(locals.len()).expect("More than 2^32 arguments of a function");
        let repacked_ty: rustc_middle::ty::Ty =
            crate::utilis::monomorphize(&instance, mir.local_decls[spread_arg].ty, tyctx);
        let repacked_type = cache.type_from_cache(repacked_ty, tyctx, instance);
        locals.push((Some("repacked_arg".into()), repacked_type));
        let mut repack_cil = Vec::new();
        // For each element of the tuple, get the argument spread_arg + n
        let packed_count = if let TyKind::Tuple(tup) = repacked_ty.kind() {
            u32::try_from(tup.len()).expect("More than 2^32 arguments of a function")
        } else {
            panic!("Arg to spread not a tuple???")
        };
        for arg_id in 0..packed_count {
            let arg_field = field_descrptor(repacked_ty, arg_id, tyctx, instance, cache);
            if *arg_field.tpe() == Type::Void {
                continue;
            }
            repack_cil.push(
                CILRoot::SetField {
                    addr: Box::new(CILNode::LDLocA(repacked)),
                    value: Box::new(CILNode::LDArg((spread_arg.as_u32() - 1) + arg_id)),
                    desc: Box::new(arg_field),
                }
                .into(),
            );
        }
        // Get the first bb, and append repack_cil at its start
        let first_bb = &mut normal_bbs[0];
        repack_cil.append(first_bb.trees_mut());
        *first_bb.trees_mut() = repack_cil;
    }
    normal_bbs
        .iter_mut()
        .for_each(|bb| bb.resolve_exception_handlers(&cleanup_bbs));

    let mut method = Method::new(
        access_modifier,
        MethodType::Static,
        sig.clone(),
        name,
        locals,
        normal_bbs,
        arg_names,
    );
    crate::method::resolve_global_allocations(&mut method, asm, tyctx, cache);

    method.allocate_temporaries();

    if *crate::config::TYPECHECK_CIL {
        match method.validate() {
            Ok(()) => (),
            Err(msg) => eprintln!(
                "\n\nMethod {} failed compilation with message:\ns {msg}",
                method.name()
            ),
        }
    }

    let adjust = check_align_adjust(&mir.local_decls, tyctx, &instance);
    // TODO: find a better way of checking if we are in release
    method.adjust_aligement(adjust);
    if tyctx.sess.opts.optimize != rustc_session::config::OptLevel::No {
        method.opt();
        method.realloc_locals();
    }

    asm.add_method(method);
    drop(timer);
    Ok(())
    //todo!("Can't add function")
}
/// This is used *ONLY* to catch uncaught errors.
pub fn checked_add_fn<'tcx>(
    asm: &mut Assembly,
    instance: Instance<'tcx>,
    tcx: TyCtxt<'tcx>,
    name: &str,
    cache: &mut TyCache,
) -> Result<(), MethodCodegenError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        add_fn(asm, instance, tcx, name, cache)
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
/// Adds a MIR item (method,inline assembly code, etc.) to the assembly.
pub fn add_item<'tcx>(
    asm: &mut Assembly,
    item: MonoItem<'tcx>,
    tcx: TyCtxt<'tcx>,
    cache: &mut TyCache,
) -> Result<(), CodegenError> {
    match item {
        MonoItem::Fn(instance) => {
            //let instance = crate::utilis::monomorphize(&instance,tcx);
            let symbol_name: Box<str> = crate::utilis::function_name(item.symbol_name(tcx));

            let function_compile_timer = tcx
                .profiler()
                .generic_activity_with_arg("compile function", item.symbol_name(tcx).to_string());
            rustc_middle::ty::print::with_no_trimmed_paths! {checked_add_fn(asm,instance, tcx, &symbol_name, cache)
            .expect("Could not add function!")};
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

            add_allocation(asm, crate::utilis::alloc_id_to_u64(alloc_id), tcx, cache);
            //let ty = alloc.0;
            drop(static_compile_timer);
            //eprintln!("Unsuported item - Static:{stotic:?}");
            Ok(())
        }
    }
}
/// Adds a static field and initialized for allocation represented by `alloc_id`.
pub fn add_allocation(
    asm: &mut Assembly,
    alloc_id: u64,
    tcx: TyCtxt<'_>,
    tycache: &mut TyCache,
) -> StaticFieldDescriptor {
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
                let field_desc =
                    StaticFieldDescriptor::new(None, Type::Ptr(Type::U8.into()), alloc_fld.clone());
                asm.static_fields_mut()
                    .insert(alloc_fld, Type::Ptr(Type::U8.into()));
                return field_desc;
            }
            GlobalAlloc::Function(_) => {
                //TODO: handle constant functions
                let alloc_fld: IString = format!("alloc_{alloc_id:x}").into();
                let field_desc =
                    StaticFieldDescriptor::new(None, Type::Ptr(Type::U8.into()), alloc_fld.clone());
                asm.static_fields_mut()
                    .insert(alloc_fld, Type::Ptr(Type::U8.into()));
                return field_desc;
                //todo!("Function/Vtable allocation.");
            }
        };

    let const_allocation = const_allocation.inner();

    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    // Alloc ids are *not* unique across all crates. Adding the hash here ensures we don't overwrite allocations during linking
    // TODO:consider using something better here / making the hashes stable.
    let byte_hash = calculate_hash(&bytes);
    let alloc_fld: IString = format!("alloc_{alloc_id:x}_{byte_hash:x}").into();

    let field_desc =
        StaticFieldDescriptor::new(None, Type::Ptr(Type::U8.into()), alloc_fld.clone());
    if !asm.static_fields_mut().contains_key(&alloc_fld) {
        let init_method =
            allocation_initializer_method(const_allocation, &alloc_fld, tcx, asm, tycache);
        let cctor = asm.add_cctor();
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
                    descr: Box::new(field_desc.clone()),
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
            //trees.push(CILRoot::debug(&format!("Finished initializing allocation {alloc_fld:?}")).into());
            // Add return again
            trees.push(ret);
        }
        drop(blocks);
        asm.add_method(init_method);
        asm.add_static(Type::Ptr(Type::U8.into()), &alloc_fld);
    }
    field_desc
}
pub fn add_const_value(asm: &mut Assembly, bytes: u128, tyctx: TyCtxt) -> StaticFieldDescriptor {
    let alloc_fld: IString = format!("a_{bytes:x}").into();
    let raw_bytes = bytes.to_le_bytes();
    let field_desc =
        StaticFieldDescriptor::new(None, Type::Ptr(Type::U8.into()), alloc_fld.clone());
    if !asm.static_fields_mut().contains_key(&alloc_fld) {
        let block = BasicBlock::new(
            vec![
                CILRoot::STLoc {
                    local: 0,
                    tree: call!(crate::cil::malloc(tyctx), [ldc_u32!(16)]),
                }
                .into(),
                CILRoot::STIndI8(CILNode::LDLoc(0), ldc_u32!(u32::from(raw_bytes[0]))).into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(1),
                    ldc_u32!(u32::from(raw_bytes[1])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(2),
                    ldc_u32!(u32::from(raw_bytes[2])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(3),
                    ldc_u32!(u32::from(raw_bytes[3])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(4),
                    ldc_u32!(u32::from(raw_bytes[4])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(5),
                    ldc_u32!(u32::from(raw_bytes[5])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(6),
                    ldc_u32!(u32::from(raw_bytes[6])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(7),
                    ldc_u32!(u32::from(raw_bytes[7])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(8),
                    ldc_u32!(u32::from(raw_bytes[8])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(9),
                    ldc_u32!(u32::from(raw_bytes[9])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(10),
                    ldc_u32!(u32::from(raw_bytes[10])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(11),
                    ldc_u32!(u32::from(raw_bytes[11])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(12),
                    ldc_u32!(u32::from(raw_bytes[12])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(13),
                    ldc_u32!(u32::from(raw_bytes[13])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(14),
                    ldc_u32!(u32::from(raw_bytes[14])),
                )
                .into(),
                CILRoot::STIndI8(
                    CILNode::LDLoc(0) + ldc_u32!(15),
                    ldc_u32!(u32::from(raw_bytes[15])),
                )
                .into(),
                CILRoot::Ret {
                    tree: CILNode::LDLoc(0),
                }
                .into(),
            ],
            0,
            None,
        );
        let init_method = Method::new(
            AccessModifer::Public,
            MethodType::Static,
            FnSig::new(&[], Type::Ptr(Type::U8.into())),
            &format!("init_a{bytes:x}"),
            vec![(Some("alloc_ptr".into()), Type::Ptr(Type::U8.into()))],
            vec![block],
            vec![],
        );

        let cctor = asm.add_cctor();
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
                    descr: Box::new(
                        StaticFieldDescriptor::new(
                            None,
                            Type::Ptr(Type::U8.into()),
                            alloc_fld.clone(),
                        )
                        .clone(),
                    ),
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
        asm.add_method(init_method);
        asm.add_static(Type::Ptr(Type::U8.into()), &alloc_fld);
    }
    field_desc
}
