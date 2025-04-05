use crate::{
    basic_block::handler_for_block,
    codegen_error::{CodegenError, MethodCodegenError},
    rustc_middle::dep_graph::DepContext,
    utilis::{alloc_id_to_u64, is_function_magic},
    IString,
};
use cilly::{
    basic_block::BasicBlock,
    call,
    cil_node::CILNode,
    cil_root::CILRoot,
    cil_tree::CILTree,
    cilnode::PtrCastRes,
    method::{Method, MethodType},
    utilis::{self, encode},
    Access, Assembly, Const, IntoAsmIndex, StringIdx, Type,
    {
        cilnode::MethodKind, v2::method::LocalDef, FnSig, Int, MethodDef, MethodRef, MethodRefIdx,
        StaticFieldDesc,
    },
};
use rustc_codegen_clr_call::CallInfo;
use rustc_codegen_clr_ctx::function_name;
pub use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::{adt::field_descrptor, r#type::get_type, utilis::is_zst, GetTypeExt};
use rustc_middle::mir::mono::Linkage;
use rustc_middle::{
    mir::{
        interpret::{AllocId, Allocation, GlobalAlloc},
        mono::MonoItem,
        Local, LocalDecl, Statement, Terminator,
    },
    ty::{TyCtxt, TyKind},
};
fn linkage_to_access(link: Option<Linkage>) -> Access {
    match link {
        Some(Linkage::External) => Access::Extern,
        _ => Access::Public,
    }
}
type LocalDefList = Vec<LocalDef>;
type ArgsDebugInfo = Vec<Option<StringIdx>>;

/// Returns the list of all local variables within MIR of a function, and converts them to the internal type represenation `Type`
fn locals_from_mir<'tcx>(
    locals: &rustc_index::IndexVec<Local, LocalDecl<'tcx>>,
    argc: usize,
    var_debuginfo: &[rustc_middle::mir::VarDebugInfo<'tcx>],
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> (ArgsDebugInfo, LocalDefList) {
    use rustc_middle::mir::VarDebugInfoContents;
    let mut local_types: Vec<LocalDef> = Vec::with_capacity(locals.len());
    for (local_id, local) in locals.iter().enumerate() {
        if local_id == 0 || local_id > argc {
            let ty = ctx.monomorphize(local.ty);
            if *crate::config::PRINT_LOCAL_TYPES {
                println!(
                    "Local type {ty:?},non-morphic: {non_morph}",
                    non_morph = local.ty
                );
            }
            let name = None;
            let tpe = ctx.type_from_cache(ty);
            let tpe = ctx.alloc_type(tpe);
            local_types.push((name, tpe));
        }
    }
    let mut arg_names: Vec<Option<StringIdx>> = (0..argc).map(|_| None).collect();
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
            local_types[0].0 = Some(var.name.to_string().into_idx(ctx));
        } else if mir_local > argc {
            local_types[mir_local - argc].0 = Some(var.name.to_string().into_idx(ctx));
        } else {
            arg_names[mir_local - 1] = Some(var.name.to_string().into_idx(ctx));
        }
    }
    (arg_names, local_types)
}

fn allocation_initializer_method(
    const_allocation: &Allocation,
    name: &str,
    asm: &mut cilly::Assembly,
    tcx: TyCtxt<'_>,
) -> Method {
    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let ptrs = const_allocation.provenance().ptrs();
    let mut trees: Vec<CILTree> = Vec::new();
    let align: u64 = const_allocation.align.bytes().max(1);
    //trees.push(CILRoot::debug(&format!("Preparing to initialize allocation with size {}",bytes.len())).into());
    if align > 8 {
        let aligned_alloc = MethodRef::aligned_alloc(asm);
        trees.push(
            CILRoot::STLoc {
                local: 0,
                tree: Box::new(call!(
                    asm.alloc_methodref(aligned_alloc),
                    [
                        CILNode::V2(asm.alloc_node(Const::USize(bytes.len() as u64))),
                        CILNode::V2(asm.alloc_node(Const::USize(align)))
                    ]
                ))
                .cast_ptr(asm.nptr(Type::Int(Int::U8))),
            }
            .into(),
        );
    } else {
        let alloc = MethodRef::alloc(asm);
        trees.push(
            CILRoot::STLoc {
                local: 0,
                tree: Box::new(call!(
                    asm.alloc_methodref(alloc),
                    [CILNode::V2(asm.alloc_node(Const::ISize(
                        i64::try_from(bytes.len()).expect("Static alloc too big")
                    )))]
                ))
                .cast_ptr(asm.nptr(Type::Int(Int::U8))),
            }
            .into(),
        );
    }

    if !bytes.iter().all(|byte| *byte != 0) {
        trees.push(
            CILRoot::InitBlk {
                dst: Box::new(CILNode::LDLoc(0)),
                val: Box::new(CILNode::V2(asm.alloc_node(0_u8))),
                count: Box::new(CILNode::V2(
                    asm.alloc_node(Const::USize(bytes.len() as u64)),
                )),
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
                            (CILNode::LDLoc(0)
                                + CILNode::V2(
                                    asm.alloc_node(Const::USize(offset.try_into().unwrap())),
                                ))
                            .cast_ptr(asm.nptr(Type::Int(Int::U64))),
                            CILNode::V2(asm.alloc_node(long)),
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
                            (CILNode::LDLoc(0)
                                + CILNode::V2(
                                    asm.alloc_node(Const::USize(offset.try_into().unwrap())),
                                ))
                            .cast_ptr(asm.nptr(Type::Int(Int::U32))),
                            CILNode::V2(asm.alloc_node(long)),
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
                            CILNode::LDLoc(0)
                                + CILNode::V2(
                                    asm.alloc_node(Const::USize(offset.try_into().unwrap())),
                                ),
                            CILNode::V2(asm.alloc_node(byte)),
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
                let mut ctx = MethodCompileCtx::new(tcx, None, finstance, asm);
                let call_info = CallInfo::sig_from_instance_(finstance, &mut ctx);
                let function_name = function_name(tcx.symbol_name(finstance));
                let mref = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string(function_name),
                    asm.alloc_sig(call_info.sig().clone()),
                    MethodKind::Static,
                    vec![].into(),
                );
                trees.push(
                    CILRoot::STIndISize(
                        (CILNode::LDLoc(0)
                            + CILNode::V2(asm.alloc_node(Const::USize(offset.into()))))
                        .cast_ptr(asm.nptr(Type::Int(Int::USize))),
                        CILNode::LDFtn(asm.alloc_methodref(mref)).cast_ptr(Type::Int(Int::USize)),
                    )
                    .into(),
                );
            } else {
                let ptr_alloc = add_allocation(prov.alloc_id().0.into(), asm, tcx);

                trees.push(
                    CILRoot::STIndISize(
                        (CILNode::LDLoc(0)
                            + CILNode::V2(asm.alloc_node(Const::USize(offset.into()))))
                        .cast_ptr(asm.nptr(Type::Int(Int::USize))),
                        ptr_alloc.cast_ptr(Type::Int(Int::USize)),
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
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let uint8_ptr_idx = asm.alloc_type(uint8_ptr);
    Method::new(
        Access::Private,
        MethodType::Static,
        FnSig::new([], uint8_ptr),
        &format!("init_{name}"),
        vec![(Some("alloc_ptr".into_idx(asm)), uint8_ptr_idx)],
        vec![BasicBlock::new(trees, 0, None)],
        vec![],
        asm,
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
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Result<Vec<CILTree>, CodegenError> {
    let terminator = if *crate::config::ABORT_ON_ERROR {
        crate::terminator::handle_terminator(term, ctx)
    } else {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::terminator::handle_terminator(term, ctx)
        })) {
            Ok(ok) => ok,
            Err(payload) => {
                let msg = if let Some(msg) = payload.downcast_ref::<&str>() {
                    rustc_middle::ty::print::with_no_trimmed_paths! {
                    format!("Tried to execute terminator {term:?} whose compialtion message {msg:?}!")}
                } else {
                    eprintln!("handle_terminator panicked with a non-string message when trying to compile {term:?} !");
                    rustc_middle::ty::print::with_no_trimmed_paths! {
                    format!("Tried to execute terminator {term:?} whose compialtion failed with a no-string message!")
                    }
                };
                vec![CILRoot::throw(&msg, ctx).into()]
            }
        }
    };

    Ok(terminator)
}
/// Turns a statement into ops, if `ABORT_ON_ERROR` set to false, will handle and recover from errors.
pub fn statement_to_ops<'tcx>(
    statement: &Statement<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Result<Vec<CILTree>, CodegenError> {
    if *crate::config::ABORT_ON_ERROR {
        Ok(crate::statement::handle_statement(statement, ctx))
    } else {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::statement::handle_statement(statement, ctx)
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
/// Adds a rust MIR function to the assembly.
pub fn add_fn<'tcx, 'asm, 'a: 'asm>(
    name: &str,
    ctx: &'a mut MethodCompileCtx<'tcx, 'asm>,
) -> Result<(), MethodCodegenError> {
    let kind = ctx
        .instance()
        .ty(
            ctx.tcx(),
            rustc_middle::ty::TypingEnv::fully_monomorphized(),
        )
        .kind();
    if let TyKind::FnDef(_, _) = kind {
        //ALL OK.
    } else if let TyKind::Closure(_, _) = kind {
    } else if let TyKind::Coroutine(_, _) = kind {
    } else {
        println!(
            "fn item {instance:?} is not a function definition type or a closure. Skippping.",
            instance = ctx.instance()
        );
        return Ok(());
    }
    let mir = ctx.tcx().instance_mir(ctx.instance().def);
    let mut ctx = ctx.with_body(mir);
    let ctx = &mut ctx;
    if name.contains("rustc_codegen_clr_comptime_entrypoint") {
        if name.contains("rustc_codegen_clr_not_magic") {
            return Ok(());
        }
        crate::comptime::interpret(ctx, mir);
        return Ok(());
    }
    if is_function_magic(name) {
        println!(
            "fn item {instance:?} is magic and is being skiped.",
            instance = ctx.instance()
        );
        return Ok(());
    }

    let timer = ctx.tcx().prof.generic_activity_with_arg("codegen fn", name);
    // Check if function is public or not.
    // FIXME: figure out the source of the bug causing visibility to not be read propely.
    // let access_modifier = Access::from_visibility(tcx.visibility(instance.def_id()));
    let attrs = ctx.tcx().codegen_fn_attrs(ctx.instance().def_id());
    let access_modifier = linkage_to_access(attrs.linkage);
    // Handle the function signature
    let call_site = CallInfo::sig_from_instance_(ctx.instance(), ctx);
    let sig = call_site.sig().clone();

    // Get locals
    let (mut arg_names, mut locals) =
        locals_from_mir(&mir.local_decls, mir.arg_count, &mir.var_debug_info, ctx);
    if sig.inputs().len() > arg_names.len() {
        arg_names.push(Some("panic_location".into_idx(ctx)));
    }

    let blocks = &mir.basic_blocks;
    let mut normal_bbs = Vec::new();
    let mut cleanup_bbs = Vec::new();
    // Used for funcrions with the rust_call ABI
    let mut repack_cil = if let Some(spread_arg) = mir.spread_arg {
        // Prepare for repacking the argument tuple, by allocating a local
        let repacked = u32::try_from(locals.len()).expect("More than 2^32 arguments of a function");
        let repacked_ty: rustc_middle::ty::Ty = ctx.monomorphize(mir.local_decls[spread_arg].ty);
        let repacked_type = get_type(repacked_ty, ctx);
        locals.push((
            Some("repacked_arg".into_idx(ctx)),
            ctx.alloc_type(repacked_type),
        ));
        let mut repack_cil: Vec<CILTree> = Vec::new();
        // For each element of the tuple, get the argument spread_arg + n
        let TyKind::Tuple(packed) = repacked_ty.kind() else {
            panic!("Arg to spread not a tuple???")
        };
        for (arg_id, ty) in packed.iter().enumerate() {
            if is_zst(ty, ctx.tcx()) {
                continue;
            }
            let arg_field = field_descrptor(repacked_ty, arg_id.try_into().unwrap(), ctx);

            repack_cil.push(
                CILRoot::SetField {
                    addr: Box::new(CILNode::LDLocA(repacked)),
                    value: Box::new(CILNode::LDArg(
                        spread_arg.as_u32() - 1 + u32::try_from(arg_id).unwrap(),
                    )),
                    desc: (arg_field),
                }
                .into(),
            );
        }
        repack_cil
    } else {
        vec![]
    };
    // Used for type-checking the CIL to ensure its validity.
    for (last_bb_id, block_data) in blocks.into_iter().enumerate() {
        let mut trees = Vec::new();
        for statement in &block_data.statements {
            if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{statement:?}"),ctx).into())};
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{:?}",statement.source_info.span),ctx).into())};
            }

            let statement_tree = match statement_to_ops(statement, ctx) {
                Ok(ops) => ops,
                Err(err) => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!(
                        "Method \"{name}\" failed to compile statement {statement:?} with message {err:?}\n"
                    )};
                    rustc_middle::ty::print::with_no_trimmed_paths! {vec![(CILRoot::throw(&format!("Tired to run a statement {statement:?} which failed to compile with error message {err:?}."),ctx).into())]}
                }
            };
            // Only save debuginfo for statements which result in ops.
            if !statement_tree.is_empty() {
                trees.push(span_source_info(ctx.tcx(), statement.source_info.span).into());
            }
            trees.extend(statement_tree);
        }
        if let Some(term) = &block_data.terminator {
            if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(CILRoot::debug(&format!("{term:?}"),ctx).into())};
            }
            let term_trees = terminator_to_ops(term, ctx).unwrap_or_else(|err| {
                panic!("Could not compile terminator {term:?} because {err:?}")
            });
            if !term_trees.is_empty() {
                trees.push(span_source_info(ctx.tcx(), term.source_info.span).into());
            }
            trees.extend(term_trees);
        }
        if block_data.is_cleanup {
            cleanup_bbs.push(BasicBlock::new(
                trees,
                u32::try_from(last_bb_id).unwrap(),
                handler_for_block(
                    block_data,
                    &mir.basic_blocks,
                    ctx.tcx(),
                    &ctx.instance(),
                    mir,
                ),
            ));
        } else {
            normal_bbs.push(BasicBlock::new(
                trees,
                u32::try_from(last_bb_id).unwrap(),
                handler_for_block(
                    block_data,
                    &mir.basic_blocks,
                    ctx.tcx(),
                    &ctx.instance(),
                    mir,
                ),
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
        ctx,
    );
    crate::method::resolve_global_allocations(&mut method, ctx);

    let main_module = ctx.main_module();
    let method = MethodDef::from_v1(&method, ctx, main_module);
    ctx.new_method(method);
    drop(timer);
    Ok(())
    //todo!("Can't add function")
}
/// This is used *ONLY* to catch uncaught errors.
pub fn checked_add_fn<'a: 'c, 'b: 'c, 'c>(
    ctx: &'a mut MethodCompileCtx<'b, 'c>,
    name: &str,
) -> Result<(), MethodCodegenError> {
    add_fn(name, ctx)
    /*match std::panic::catch_unwind(add_fn) {
        Ok(success) => success,
        Err(payload) => {
            if let Some(msg) = payload.downcast_ref::<&str>() {
                eprintln!("could not compile method {name}. fn_add panicked with unhandled message: {msg:?}");
                //self.add_method(Method::missing_because(format!("could not compile method {name}. fn_add panicked with unhandled message: {msg:?}")));
                Ok(())
            } else {
                eprintln!("could not compile method {name}. fn_add panicked with no message.");
                Ok(())
            }
        }
    }*/
}
/// Adds a MIR item (method,inline assembly code, etc.) to the assembly.
#[allow(clippy::similar_names)]
pub fn add_item<'tcx>(
    asm: &mut Assembly,
    item: MonoItem<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> Result<(), CodegenError> {
    match item {
        MonoItem::Fn(instance) => {
            let symbol_name: IString = function_name(item.symbol_name(tcx)).into();
            let mut ctx = MethodCompileCtx::new(tcx, None, instance, asm);
            let function_compile_timer = tcx
                .profiler()
                .generic_activity_with_arg("compile function", item.symbol_name(tcx).to_string());
            rustc_middle::ty::print::with_no_trimmed_paths! {checked_add_fn(  &mut ctx,&symbol_name,)
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

            let int8_ptr = asm.nptr(Type::Int(Int::I8));
            let int8_ptr_ptr = asm.nptr(int8_ptr);
            if let Some(section) = attrs.link_section {
                if section.to_string().contains(".init_array") {
                    let argc = utilis::argc_argv_init_method(asm);
                    let init_argc = asm.alloc_root(cilly::CILRoot::call(argc, []));

                    asm.add_user_init(&[init_argc]);
                    let get_environ: MethodRefIdx = utilis::get_environ(asm);
                    let fn_ptr = alloc.0.provenance().ptrs().iter().next().unwrap();
                    let fn_ptr = tcx.global_alloc(fn_ptr.1.alloc_id());
                    let init_call_site = if let GlobalAlloc::Function {
                        instance: finstance,
                    } = fn_ptr
                    {
                        let mut ctx = MethodCompileCtx::new(tcx, None, finstance, asm);
                        // If it is a function, patch its pointer up.
                        let call_info = CallInfo::sig_from_instance_(finstance, &mut ctx);
                        let function_name = function_name(tcx.symbol_name(finstance));
                        MethodRef::new(
                            *asm.main_module(),
                            asm.alloc_string(function_name),
                            asm.alloc_sig(call_info.sig().clone()),
                            MethodKind::Static,
                            vec![].into(),
                        )
                    } else {
                        panic!()
                    };

                    let argv = asm.alloc_string("argv");
                    let argc = asm.alloc_string("argc");
                    let main_module = asm.main_module();
                    let mref = asm.alloc_methodref(init_call_site);
                    let argv =
                        asm.alloc_sfld(StaticFieldDesc::new(*main_module, argv, int8_ptr_ptr));
                    let argc = asm.alloc_sfld(StaticFieldDesc::new(
                        *main_module,
                        argc,
                        Type::Int(Int::I32),
                    ));
                    let argv = asm.alloc_node(cilly::CILNode::LdStaticField(argv));
                    let uint8_ptr = asm.nptr(Int::U8);
                    let uint8_ptr_idx = asm.alloc_type(uint8_ptr);
                    let args = [
                        asm.alloc_node(cilly::CILNode::LdStaticField(argc)),
                        asm.alloc_node(cilly::CILNode::PtrCast(
                            argv,
                            Box::new(PtrCastRes::Ptr(uint8_ptr_idx)),
                        )),
                        asm.alloc_node(cilly::CILNode::call(get_environ, [])),
                    ];
                    let root = asm.alloc_root(cilly::CILRoot::call(mref, args));
                    asm.add_user_init(&[root]);
                } else {
                    panic!("Unsuported link section {section}.")
                }
            }
            add_allocation(alloc_id_to_u64(alloc_id), asm, tcx);

            drop(static_compile_timer);

            Ok(())
        }
    }
}
/// Adds a static field and initialized for allocation represented by `alloc_id`.
pub fn add_allocation(alloc_id: u64, asm: &mut cilly::Assembly, tcx: TyCtxt<'_>) -> CILNode {
    let uint8_ptr = asm.nptr(Type::Int(Int::U8));
    let main_module_id = asm.main_module();
    let (thread_local, const_allocation, krate) =
        match tcx.global_alloc(AllocId(alloc_id.try_into().expect("0 alloc id?"))) {
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

                let field_desc = StaticFieldDesc::new(
                    *asm.main_module(),
                    asm.alloc_string(alloc_fld.clone()),
                    uint8_ptr,
                );
                asm.add_static(uint8_ptr, alloc_fld, false, main_module_id);
                return CILNode::LDStaticField(Box::new(field_desc));
            }
            GlobalAlloc::Function { .. } => {
                //TODO: handle constant functions
                let alloc_fld: IString = format!("al_{alloc_id:x}").into();
                let field_desc = StaticFieldDesc::new(
                    *asm.main_module(),
                    asm.alloc_string(alloc_fld.clone()),
                    uint8_ptr,
                );
                asm.add_static(uint8_ptr, alloc_fld, false, main_module_id);

                return CILNode::LDStaticField(Box::new(field_desc));
                //todo!("Function/Vtable allocation.");
            }
        };

    let const_allocation = const_allocation.inner();

    let bytes: &[u8] =
        const_allocation.inspect_with_uninit_and_ptr_outside_interpreter(0..const_allocation.len());
    let align = const_allocation.align.bytes().max(1);
    if const_allocation.len() == 0 {
        return CILNode::V2(asm.alloc_node(Const::USize(align)));
    }
    // Alloc ids are *not* unique across all crates. Adding the hash here ensures we don't overwrite allocations during linking
    // TODO:consider using something better here / making the hashes stable.
    let byte_hash = calculate_hash(&bytes);
    match (align, bytes.len()) {
        // Assumes this constant is not a pointer.
        (0..=1, len @ 1) | (0..=2, len @ 1..=2) | (0..=4, len @ 1..=4) | (0..=8, len @ 1..=16)
            if len <= asm.max_static_size() =>
        {
            let alloc_name: IString = if let Some(krate) = krate {
                format!(
                    "s_{}_{}_{}_{thread_local}_{}",
                    encode(alloc_id),
                    encode(byte_hash),
                    const_allocation.len(),
                    encode(u64::from(krate.as_u32())),
                )
                .into()
            } else {
                format!(
                    "s_{}_{}_{}_{thread_local}",
                    encode(alloc_id),
                    encode(byte_hash),
                    const_allocation.len()
                )
                .into()
            };
            let name = asm.alloc_string(alloc_name.clone());
            let tpe: Int = Int::from_size_sign(
                u8::try_from(bytes.len()).unwrap().next_power_of_two(),
                false,
            );
            let field_desc = StaticFieldDesc::new(*asm.main_module(), name, cilly::Type::Int(tpe));
            // Currently, all static fields are in one module. Consider spliting them up.
            let main_module = asm.class_mut(main_module_id);
            if main_module.has_static_field(name, field_desc.tpe()) {
                return CILNode::AddressOfStaticField(Box::new(field_desc));
            }

            asm.add_static(
                cilly::Type::Int(tpe),
                &*alloc_name,
                thread_local,
                main_module_id,
            );
            let field = asm.alloc_sfld(field_desc);
            let cst: Const = tpe.from_bytes(bytes);
            let val = asm.alloc_node(cst);
            let mut roots = vec![asm.alloc_root(cilly::CILRoot::SetStaticField { field, val })];
            let addr = CILNode::AddressOfStaticField(Box::new(field_desc));
            for (offset, prov) in const_allocation.provenance().ptrs().iter() {
                let offset = u32::try_from(offset.bytes_usize()).unwrap();
                // Check if this allocation is a function
                let reloc_target_alloc = tcx.global_alloc(prov.alloc_id());
                if let GlobalAlloc::Function {
                    instance: finstance,
                } = reloc_target_alloc
                {
                    // If it is a function, patch its pointer up.
                    let mut ctx = MethodCompileCtx::new(tcx, None, finstance, asm);
                    let call_info = CallInfo::sig_from_instance_(finstance, &mut ctx);
                    let function_name = function_name(tcx.symbol_name(finstance));
                    let mref = MethodRef::new(
                        *asm.main_module(),
                        asm.alloc_string(function_name),
                        asm.alloc_sig(call_info.sig().clone()),
                        MethodKind::Static,
                        vec![].into(),
                    );
                    let root = cilly::CILRoot::from_v1(
                        &CILRoot::STIndISize(
                            (addr.clone()
                                + CILNode::V2(asm.alloc_node(Const::USize(offset.into()))))
                            .cast_ptr(asm.nptr(Type::Int(Int::USize))),
                            CILNode::LDFtn(asm.alloc_methodref(mref))
                                .cast_ptr(Type::Int(Int::USize)),
                        ),
                        asm,
                    );
                    roots.push(asm.alloc_root(root));
                } else {
                    let ptr_alloc = add_allocation(prov.alloc_id().0.into(), asm, tcx);
                    let root = cilly::CILRoot::from_v1(
                        &CILRoot::STIndISize(
                            (addr.clone()
                                + CILNode::V2(asm.alloc_node(Const::USize(offset.into()))))
                            .cast_ptr(asm.nptr(Type::Int(Int::USize))),
                            ptr_alloc.cast_ptr(Type::Int(Int::USize)),
                        ),
                        asm,
                    );
                    roots.push(asm.alloc_root(root));
                }
            }
            if thread_local {
                asm.add_tcctor(&roots);
            } else {
                asm.add_cctor(&roots);
            }
            CILNode::AddressOfStaticField(Box::new(field_desc))
        }
        _ => {
            let alloc_name: IString = if let Some(krate) = krate {
                format!(
                    "al_{}_{}_{}_{thread_local}_{}",
                    encode(alloc_id),
                    encode(byte_hash),
                    const_allocation.len(),
                    encode(u64::from(krate.as_u32())),
                )
                .into()
            } else {
                format!(
                    "al_{}_{}_{}_{thread_local}",
                    encode(alloc_id),
                    encode(byte_hash),
                    const_allocation.len()
                )
                .into()
            };
            let name = asm.alloc_string(alloc_name.clone());
            let field_desc =
                StaticFieldDesc::new(*asm.main_module(), name, asm.nptr(Type::Int(Int::U8)));
            // Currently, all static fields are in one module. Consider spliting them up.
            let main_module = asm.class_mut(main_module_id);
            if main_module.has_static_field(name, field_desc.tpe()) {
                return CILNode::LDStaticField(Box::new(field_desc));
            }
            asm.add_static(uint8_ptr, &*alloc_name, thread_local, main_module_id);

            let init_method =
                allocation_initializer_method(const_allocation, &alloc_name, asm, tcx);
            let init_method = MethodDef::from_v1(&init_method, asm, main_module_id);
            let initialzer = asm.new_method(init_method);
            // Calls the static initialzer, and sets the static field to the returned pointer.
            let val = asm.alloc_node(cilly::CILNode::call(*initialzer, []));

            let field = asm.alloc_sfld(field_desc);
            let root = asm.alloc_root(cilly::CILRoot::SetStaticField { field, val });
            if thread_local {
                asm.add_tcctor(&[root]);
            } else {
                asm.add_cctor(&[root]);
            }

            CILNode::LDStaticField(Box::new(field_desc))
        }
    }
}
pub fn add_const_value(asm: &mut cilly::Assembly, bytes: u128) -> StaticFieldDesc {
    let uint8_ptr = Type::Int(Int::U128);
    let main_module_id = asm.main_module();
    let alloc_fld: IString = format!("a_{bytes:x}").into();
    let alloc_fld_name = asm.alloc_string(alloc_fld.clone());

    let field_desc = StaticFieldDesc::new(*asm.main_module(), alloc_fld_name, Type::Int(Int::U128));

    let main_module = asm.class_mut(main_module_id);
    if main_module.has_static_field(alloc_fld_name, field_desc.tpe()) {
        return field_desc;
    }
    asm.add_static(uint8_ptr, alloc_fld, false, main_module_id);
    let cst = CILNode::const_u128(bytes, asm);

    let field = asm.alloc_sfld(field_desc);
    let val = cilly::CILNode::from_v1(&cst, asm);
    let val = asm.alloc_node(val);
    let set = asm.alloc_root(cilly::CILRoot::SetStaticField { field, val });

    asm.add_cctor(&[set]);

    field_desc
}

pub(crate) fn span_source_info(tcx: TyCtxt, span: rustc_span::Span) -> CILRoot {
    let (file, lstart, cstart, lend, mut cend) = tcx.sess.source_map().span_to_location_info(span);
    let file = file.map_or(String::new(), |file| {
        file.name
            .display(rustc_span::FileNameDisplayPreference::Local)
            .to_string()
    });
    if cstart >= cend {
        cend = cstart + 1;
    }
    CILRoot::source_info(
        &file,
        (lstart as u64)..(lend as u64),
        (cstart as u64)..(cend as u64),
    )
}
