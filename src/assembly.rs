pub use crate::fn_ctx::MethodCompileCtx;
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
    conv_usize, ldc_i32, ldc_u32, ldc_u64,
    method::{Method, MethodType},
    ptr,
    static_field_desc::StaticFieldDescriptor,
    utilis::{self, encode},
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
fn check_align_adjust<'tcx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'tcx>>,
    tcx: TyCtxt<'tcx>,
    method_instance: &Instance<'tcx>,
    argc: usize,
) -> Vec<Option<u64>> {
    let mut adjusts: Vec<Option<u64>> = Vec::with_capacity(locals.len());
    for (local_id, local) in locals.iter().enumerate() {
        if local_id == 0 || local_id > argc {
            let ty = crate::utilis::monomorphize(method_instance, local.ty, tcx);
            let adjust = crate::utilis::requries_align_adjustement(ty, tcx);
            adjusts.push(adjust);
        }
    }
    adjusts
}
/// Returns the list of all local variables within MIR of a function, and converts them to the internal type represenation `Type`
fn locals_from_mir<'tcx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'tcx>>,
    tcx: TyCtxt<'tcx>,
    argc: usize,
    method_instance: &Instance<'tcx>,
    tycache: &mut TyCache,
    var_debuginfo: &[rustc_middle::mir::VarDebugInfo<'tcx>],
) -> (ArgsDebugInfo, LocalDefList) {
    use rustc_middle::mir::VarDebugInfoContents;
    let mut local_types: Vec<(Option<IString>, _)> = Vec::with_capacity(locals.len());
    for (local_id, local) in locals.iter().enumerate() {
        if local_id == 0 || local_id > argc {
            let ty = crate::utilis::monomorphize(method_instance, local.ty, tcx);
            if *crate::config::PRINT_LOCAL_TYPES {
                println!(
                    "Local type {ty:?},non-morphic: {non_morph}",
                    non_morph = local.ty
                );
            }
            let name = None;
            let tpe = tycache.type_from_cache(ty, tcx, *method_instance);
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
    tcx: TyCtxt,
    asm: &mut Assembly,
    tycache: &mut TyCache,
) -> Method {
    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let ptrs = const_allocation.provenance().ptrs();
    let mut trees: Vec<CILTree> = Vec::new();
    let align = const_allocation.align.bytes().max(1);
    //trees.push(CILRoot::debug(&format!("Preparing to initialize allocation with size {}",bytes.len())).into());
    if align > 8 {
        trees.push(
            CILRoot::STLoc {
                local: 0,
                tree: Box::new(call!(
                    CallSite::aligned_alloc(),
                    [
                        conv_usize!(ldc_u64!(bytes.len() as u64)),
                        conv_usize!(ldc_u64!(align))
                    ]
                ))
                .cast_ptr(ptr!(Type::U8)),
            }
            .into(),
        );
    } else {
        trees.push(
            CILRoot::STLoc {
                local: 0,
                tree: Box::new(call!(
                    CallSite::alloc(),
                    [ldc_i32!(
                        i32::try_from(bytes.len()).expect("Static alloc too big")
                    )]
                ))
                .cast_ptr(ptr!(Type::U8)),
            }
            .into(),
        );
    }

    if !bytes.iter().all(|byte| *byte != 0) {
        trees.push(
            CILRoot::InitBlk {
                dst: Box::new(CILNode::LDLoc(0)),
                val: Box::new(CILNode::LdcU8(0)),
                count: Box::new(conv_usize!(ldc_u64!(bytes.len() as u64))),
            }
            .into(),
        );
    }
    let mut offset = 0;
    while offset < bytes.len() {
        let reminder = bytes.len() - offset;
        match reminder {
            8.. => {
                let long = u64::from_ne_bytes(bytes[offset..(offset + 8)].try_into().unwrap());
                if long != 0 {
                    trees.push(
                        CILRoot::STIndI64(
                            (CILNode::LDLoc(0) + conv_usize!(ldc_u32!(offset.try_into().unwrap())))
                                .cast_ptr(ptr!(Type::U64)),
                            CILNode::LdcU64(long),
                        )
                        .into(),
                    );
                }
                offset += 8;
            }
            4.. => {
                let long = u32::from_ne_bytes(bytes[offset..(offset + 4)].try_into().unwrap());
                if long != 0 {
                    trees.push(
                        CILRoot::STIndI32(
                            (CILNode::LDLoc(0) + conv_usize!(ldc_u32!(offset.try_into().unwrap())))
                                .cast_ptr(ptr!(Type::U32)),
                            CILNode::LdcU32(long),
                        )
                        .into(),
                    );
                }
                offset += 4;
            }
            _ => {
                let byte = bytes[offset];
                if byte != 0 {
                    trees.push(
                        CILRoot::STIndI8(
                            CILNode::LDLoc(0) + conv_usize!(ldc_u32!(offset.try_into().unwrap())),
                            CILNode::LdcU8(byte),
                        )
                        .into(),
                    );
                }
                offset += 1;
            }
        }
    }
    if !ptrs.is_empty() {
        for (offset, prov) in ptrs.iter() {
            let offset = u32::try_from(offset.bytes_usize()).unwrap();
            // Check if this allocation is a function
            let reloc_target_alloc = tcx.global_alloc(prov.alloc_id());
            if let GlobalAlloc::Function {
                instance: finstance,
            } = reloc_target_alloc
            {
                // If it is a function, patch its pointer up.
                let call_info =
                    crate::call_info::CallInfo::sig_from_instance_(finstance, tcx, tycache);
                let function_name = crate::utilis::function_name(tcx.symbol_name(finstance));

                trees.push(
                    CILRoot::STIndISize(
                        (CILNode::LDLoc(0) + conv_usize!(ldc_u32!(offset)))
                            .cast_ptr(ptr!(Type::USize)),
                        CILNode::LDFtn(
                            CallSite::new(None, function_name, call_info.sig().clone(), true)
                                .into(),
                        )
                        .cast_ptr(Type::USize),
                    )
                    .into(),
                );
            } else {
                let ptr_alloc = add_allocation(asm, prov.alloc_id().0.into(), tcx, tycache);

                trees.push(
                    CILRoot::STIndISize(
                        (CILNode::LDLoc(0) + conv_usize!(ldc_u32!(offset)))
                            .cast_ptr(ptr!(Type::USize)),
                        ptr_alloc.cast_ptr(Type::USize),
                    )
                    .into(),
                );
            }
        }
    }
    //trees.push(CILRoot::debug(&format!("Finished initializing an allocation with size {}",bytes.len())).into());
    trees.push(
        CILRoot::Ret {
            tree: CILNode::LDLoc(0),
        }
        .into(),
    );

    Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[], ptr!(Type::U8)),
        &format!("init_{name}"),
        vec![(Some("alloc_ptr".into()), ptr!(Type::U8))],
        vec![BasicBlock::new(trees, 0, None)],
        vec![],
    )
}
fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::{DefaultHasher, Hasher};
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
/// Turns a terminator into ops, if `ABORT_ON_ERROR` set to false, will handle and recover from errors.
pub fn terminator_to_ops<'tcx>(
    term: &Terminator<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> Result<Vec<CILTree>, CodegenError> {
    let terminator = if *crate::config::ABORT_ON_ERROR {
        crate::terminator::handle_terminator(term, ctx)
    } else {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::terminator::handle_terminator(term, ctx)
        })) {
            Ok(ok) => {
                if *crate::config::TYPECHECK_CIL {
                    for op in &ok {
                        if let Err(msg) = op.validate(ctx.validator()) {
                            Err(crate::codegen_error::CodegenError::from_panic_message(
                                &format!("VERIFICATION FALIURE:\"{msg}\" op:{op:?}"),
                            ))?;
                        }
                    }
                }
                ok
            }
            Err(payload) => {
                ctx.type_cache().recover_from_panic();
                let msg = if let Some(msg) = payload.downcast_ref::<&str>() {
                    rustc_middle::ty::print::with_no_trimmed_paths! {
                    format!("Tried to execute terminator {term:?} whose compialtion message {msg:?}!")}
                } else {
                    eprintln!("handle_terminator panicked with a non-string message when trying to compile {term:?} !");
                    rustc_middle::ty::print::with_no_trimmed_paths! {
                    format!("Tried to execute terminator {term:?} whose compialtion failed with a no-string message!")
                    }
                };
                vec![CILRoot::throw(&msg).into()]
            }
        }
    };

    Ok(terminator)
}
/// Turns a statement into ops, if `ABORT_ON_ERROR` set to false, will handle and recover from errors.
pub fn statement_to_ops<'tcx>(
    statement: &Statement<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> Result<Option<CILTree>, CodegenError> {
    if *crate::config::ABORT_ON_ERROR {
        Ok(crate::statement::handle_statement(statement, ctx))
    } else {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::statement::handle_statement(statement, ctx)
        })) {
            Ok(success) => {
                match &success {
                    Some(ops) => {
                        if *crate::config::TYPECHECK_CIL {
                            if let Err(msg) = ops.validate(ctx.validator()) {
                                Err(crate::codegen_error::CodegenError::from_panic_message(
                                    &format!("VERIFICATION FALIURE:\"{msg}\" ops:{ops:?}"),
                                ))?;
                            }
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
pub fn add_fn<'tcx>(
    asm: &mut Assembly,
    instance: Instance<'tcx>,
    tcx: TyCtxt<'tcx>,
    name: &str,
    cache: &mut TyCache,
) -> Result<(), MethodCodegenError> {
    if let TyKind::FnDef(_, _) = instance.ty(tcx, ParamEnv::reveal_all()).kind() {
        //ALL OK.
    } else if let TyKind::Closure(_, _) = instance.ty(tcx, ParamEnv::reveal_all()).kind() {
        //println!("CLOSURE")
    } else {
        println!("fn item {instance:?} is not a function definition type or a closure. Skippping.");
        return Ok(());
    }
    let mir = tcx.instance_mir(instance.def);
    if name.contains("rustc_codegen_clr_comptime_entrypoint") {
        if name.contains("rustc_codegen_clr_not_magic") {
            return Ok(());
        }
        crate::comptime::interpret(asm, instance, tcx, mir, cache);
        return Ok(());
    }
    if crate::utilis::is_function_magic(name) {
        println!("fn item {instance:?} is magic and is being skiped.");
        return Ok(());
    }

    let timer = tcx.prof.generic_activity_with_arg("codegen fn", name);
    // Check if function is public or not.
    // FIXME: figure out the source of the bug causing visibility to not be read propely.
    // let access_modifier = AccessModifer::from_visibility(tcx.visibility(instance.def_id()));
    let access_modifier = AccessModifer::Public;
    // Handle the function signature
    let call_site = crate::call_info::CallInfo::sig_from_instance_(instance, tcx, cache);
    let sig = call_site.sig().clone();

    // Get locals
    let (mut arg_names, mut locals) = locals_from_mir(
        &mir.local_decls,
        tcx,
        mir.arg_count,
        &instance,
        cache,
        &mir.var_debug_info,
    );
    if sig.inputs().len() > arg_names.len() {
        arg_names.push(Some("panic_location".into()));
    }

    let blocks = &mir.basic_blocks;
    let mut normal_bbs = Vec::new();
    let mut cleanup_bbs = Vec::new();
    // Used for funcrions with the rust_call ABI
    let mut repack_cil = if let Some(spread_arg) = mir.spread_arg {
        // Prepare for repacking the argument tuple, by allocating a local
        let repacked = u32::try_from(locals.len()).expect("More than 2^32 arguments of a function");
        let repacked_ty: rustc_middle::ty::Ty =
            crate::utilis::monomorphize(&instance, mir.local_decls[spread_arg].ty, tcx);
        let repacked_type = cache.type_from_cache(repacked_ty, tcx, instance);
        locals.push((Some("repacked_arg".into()), repacked_type));
        let mut repack_cil: Vec<CILTree> = Vec::new();
        // For each element of the tuple, get the argument spread_arg + n
        let TyKind::Tuple(packed) = repacked_ty.kind() else {
            panic!("Arg to spread not a tuple???")
        };
        for (arg_id, ty) in packed.iter().enumerate() {
            let validation_context = ValidationContext::new(&sig, &locals);
            let mut method_context =
                MethodCompileCtx::new(tcx, mir, instance, validation_context, cache);
            if crate::utilis::is_zst(ty, tcx) {
                continue;
            }
            let arg_field = field_descrptor(repacked_ty, arg_id as u32, &mut method_context);

            repack_cil.push(
                CILRoot::SetField {
                    addr: Box::new(CILNode::LDLocA(repacked)),
                    value: Box::new(CILNode::LDArg((spread_arg.as_u32() - 1) + (arg_id as u32))),
                    desc: Box::new(arg_field),
                }
                .into(),
            );
        }
        repack_cil
    } else {
        vec![]
    };
    // Used for type-checking the CIL to ensure its validity.
    let validation_context = ValidationContext::new(&sig, &locals);
    let mut method_context = MethodCompileCtx::new(tcx, mir, instance, validation_context, cache);
    for (last_bb_id, block_data) in blocks.into_iter().enumerate() {
        let mut trees = Vec::new();
        for statement in &block_data.statements {
            if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{statement:?}")).into())};
            }

            let statement_tree = match statement_to_ops(statement, &mut method_context) {
                Ok(ops) => ops,
                Err(err) => {
                    method_context.type_cache().recover_from_panic();
                    rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!(
                        "Method \"{name}\" failed to compile statement {statement:?} with message {err:?}\n"
                    )};
                    rustc_middle::ty::print::with_no_trimmed_paths! {Some(CILRoot::throw(&format!("Tired to run a statement {statement:?} which failed to compile with error message {err:?}.")).into())}
                }
            };
            // Only save debuginfo for statements which result in ops.
            if statement_tree.is_some() {
                trees.push(span_source_info(tcx, statement.source_info.span).into());
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
                let term_trees =
                    terminator_to_ops(term, &mut method_context).unwrap_or_else(|err| {
                        panic!("Could not compile terminator {term:?} because {err:?}")
                    });
                if !term_trees.is_empty() {
                    trees.push(span_source_info(tcx, term.source_info.span).into());
                }
                trees.extend(term_trees);
            }
            None => (),
        }
        if block_data.is_cleanup {
            cleanup_bbs.push(BasicBlock::new(
                trees,
                u32::try_from(last_bb_id).unwrap(),
                handler_for_block(block_data, &mir.basic_blocks, tcx, &instance, mir),
            ));
        } else {
            normal_bbs.push(BasicBlock::new(
                trees,
                u32::try_from(last_bb_id).unwrap(),
                handler_for_block(block_data, &mir.basic_blocks, tcx, &instance, mir),
            ));
        }
        //ops.extend(trees.iter().flat_map(|tree| tree.flatten()))
    }

    normal_bbs
        .iter_mut()
        .for_each(|bb| bb.resolve_exception_handlers(&cleanup_bbs));
    normal_bbs
        .iter_mut()
        .for_each(cilly::basic_block::BasicBlock::sheed_trees);
    // Get the first bb, and append repack_cil at its start
    let first_bb: &mut BasicBlock = &mut normal_bbs[0];
    repack_cil.append(first_bb.trees_mut());
    *first_bb.trees_mut() = repack_cil;

    let mut method = Method::new(
        access_modifier,
        MethodType::Static,
        sig.clone(),
        name,
        locals,
        normal_bbs,
        arg_names,
    );

    crate::method::resolve_global_allocations(&mut method, asm, tcx, cache);

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

    let adjust = check_align_adjust(&mir.local_decls, tcx, &instance, mir.arg_count);
    // TODO: find a better way of checking if we are in release
    method.adjust_aligement(adjust);
    if tcx.sess.opts.optimize != rustc_session::config::OptLevel::No {
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
            let symbol_name: IString = crate::utilis::function_name(item.symbol_name(tcx));

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
            let attrs = tcx.codegen_fn_attrs(stotic);
            if let Some(section) = attrs.link_section {
                if section.to_string().contains(".init_array") {
                    let argc = utilis::argc_argv_init_method(asm);
                    asm.add_initialzer(CILRoot::Call {
                        site: Box::new(argc),
                        args: [].into(),
                    });
                    let get_environ: CallSite = utilis::get_environ(asm);
                    let fn_ptr = alloc.0.provenance().ptrs().iter().next().unwrap();
                    let fn_ptr = tcx.global_alloc(fn_ptr.1.alloc_id());
                    let init_call_site = if let GlobalAlloc::Function {
                        instance: finstance,
                    } = fn_ptr
                    {
                        // If it is a function, patch its pointer up.
                        let call_info =
                            crate::call_info::CallInfo::sig_from_instance_(finstance, tcx, cache);
                        let function_name =
                            crate::utilis::function_name(tcx.symbol_name(finstance));
                        CallSite::new(None, function_name, call_info.sig().clone(), true)
                    } else {
                        panic!()
                    };

                    asm.add_initialzer(CILRoot::Call {
                        site: Box::new(init_call_site),
                        args: [
                            CILNode::LDStaticField(Box::new(StaticFieldDescriptor::new(
                                None,
                                Type::I32,
                                "argc".into(),
                            ))),
                            CILNode::LDStaticField(Box::new(StaticFieldDescriptor::new(
                                None,
                                ptr!(ptr!(Type::U8)),
                                "argv".into(),
                            ))),
                            call!(get_environ, []),
                        ]
                        .into(),
                    });
                } else {
                    panic!("Unsuported link section {section}.")
                }
            }
            add_allocation(asm, crate::utilis::alloc_id_to_u64(alloc_id), tcx, cache);

            drop(static_compile_timer);

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
) -> CILNode {
    let (thread_local, const_allocation, krate) = match tcx
        .global_alloc(AllocId(alloc_id.try_into().expect("0 alloc id?")))
    {
        GlobalAlloc::Memory(alloc) => (false, alloc, None),
        GlobalAlloc::Static(def_id) => {
            let alloc = tcx.eval_static_initializer(def_id).unwrap();
            let attrs = tcx.codegen_fn_attrs(def_id);

            (
                attrs.flags.contains(
                    rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags::THREAD_LOCAL,
                ),
                alloc,
                Some(def_id.krate),
            )
        }
        GlobalAlloc::VTable(..) => {
            //TODO: handle VTables
            let alloc_fld: IString = format!("al_{alloc_id:x}").into();
            let field_desc = StaticFieldDescriptor::new(None, ptr!(Type::U8), alloc_fld.clone());
            asm.add_static(ptr!(Type::U8), &alloc_fld, false);
            return CILNode::LDStaticField(Box::new(field_desc));
        }
        GlobalAlloc::Function { .. } => {
            //TODO: handle constant functions
            let alloc_fld: IString = format!("al_{alloc_id:x}").into();
            let field_desc = StaticFieldDescriptor::new(None, ptr!(Type::U8), alloc_fld.clone());
            asm.add_static(ptr!(Type::U8), &alloc_fld, false);

            return CILNode::LDStaticField(Box::new(field_desc));
            //todo!("Function/Vtable allocation.");
        }
    };

    let const_allocation = const_allocation.inner();

    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    // Alloc ids are *not* unique across all crates. Adding the hash here ensures we don't overwrite allocations during linking
    // TODO:consider using something better here / making the hashes stable.
    let byte_hash = calculate_hash(&bytes);
    let alloc_fld: IString = if let Some(krate) = krate {
        format!(
            "al_{}_{}_{}",
            encode(alloc_id),
            encode(byte_hash),
            encode(krate.as_u32() as u64),
        )
        .into()
    } else {
        format!("al_{}_{}", encode(alloc_id), encode(byte_hash)).into()
    };
    let field_desc = StaticFieldDescriptor::new(None, ptr!(Type::U8), alloc_fld.clone());
    if !asm.static_fields().contains_key(&alloc_fld) {
        let init_method =
            allocation_initializer_method(const_allocation, &alloc_fld, tcx, asm, tycache);
        let mut blocks = if thread_local {
            let tcctor = asm.add_tcctor();
            let mut blocks = tcctor.blocks_mut();
            if blocks.is_empty() {
                blocks.push(BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None));
            }
            assert_eq!(
                blocks.len(),
                1,
                "Unexpected number of basic blocks in a static data initializer."
            );
            blocks
        } else {
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
            blocks
        };
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
        asm.add_static(ptr!(Type::U8), &alloc_fld, thread_local);
    }
    CILNode::LDStaticField(Box::new(field_desc))
}
pub fn add_const_value(asm: &mut Assembly, bytes: u128, tcx: TyCtxt) -> StaticFieldDescriptor {
    let alloc_fld: IString = format!("a_{bytes:x}").into();
    let raw_bytes = bytes.to_le_bytes();
    let field_desc = StaticFieldDescriptor::new(None, ptr!(Type::U8), alloc_fld.clone());
    if !asm.static_fields().contains_key(&alloc_fld) {
        let mut trees = vec![CILRoot::STLoc {
            local: 0,
            tree: call!(crate::cil::malloc(tcx), [conv_usize!(ldc_u32!(16))])
                .cast_ptr(ptr!(Type::U8)),
        }
        .into()];
        // This is an optimization if and only if there are enough zero-bytes to justify this.
        if !raw_bytes.iter().all(|byte| *byte != 0) {
            trees.push(
                CILRoot::InitBlk {
                    dst: Box::new(CILNode::LDLoc(0)),
                    val: Box::new(CILNode::LdcU8(0)),
                    count: Box::new(conv_usize!(ldc_u32!(16))),
                }
                .into(),
            );
        }
        for index in 0..16 {
            if raw_bytes[index as usize] != 0 {
                trees.push(
                    CILRoot::STIndI8(
                        CILNode::LDLoc(0) + conv_usize!(ldc_u32!(index)),
                        CILNode::LdcU8(raw_bytes[index as usize]),
                    )
                    .into(),
                );
            }
        }
        trees.push(
            CILRoot::Ret {
                tree: CILNode::LDLoc(0),
            }
            .into(),
        );
        let block = BasicBlock::new(trees, 0, None);
        let init_method = Method::new(
            AccessModifer::Public,
            MethodType::Static,
            FnSig::new(&[], ptr!(Type::U8)),
            &format!("init_a{bytes:x}"),
            vec![(Some("alloc_ptr".into()), ptr!(Type::U8))],
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
                        StaticFieldDescriptor::new(None, ptr!(Type::U8), alloc_fld.clone()).clone(),
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
        asm.add_static(ptr!(Type::U8), &alloc_fld, false);
    }
    field_desc
}
