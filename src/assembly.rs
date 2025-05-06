use crate::{
    basic_block::handler_for_block,
    codegen_error::{CodegenError, MethodCodegenError},
    rustc_middle::dep_graph::DepContext,
    utilis::{alloc_id_to_u64, is_function_magic},
    IString,
};
use cilly::{
    basic_block::BasicBlock,
    cil_node::V1Node,
    cil_root::V1Root,
    cil_tree::CILTree,
    cilnode::{MethodKind, PtrCastRes},
    method::{Method, MethodType},
    utilis::{self},
    v2::method::LocalDef,
    Access, Assembly, Int, Interned, IntoAsmIndex, MethodDef, MethodRef, StaticFieldDesc, Type,
};
use rustc_codegen_clr_call::CallInfo;
use rustc_codegen_clr_ctx::function_name;
pub use rustc_codegen_clr_ctx::MethodCompileCtx;
use rustc_codegen_clr_type::{adt::field_descrptor, r#type::get_type, utilis::is_zst, GetTypeExt};
use rustc_codgen_clr_operand::static_data::add_static;
use rustc_middle::mir::mono::Linkage;
use rustc_middle::{
    mir::{interpret::GlobalAlloc, mono::MonoItem, Local, LocalDecl, Statement, Terminator},
    ty::{TyCtxt, TyKind},
};
fn linkage_to_access(link: Option<Linkage>) -> Access {
    match link {
        Some(Linkage::External) => Access::Extern,
        _ => Access::Public,
    }
}
type LocalDefList = Vec<LocalDef>;
type ArgsDebugInfo = Vec<Option<Interned<IString>>>;

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
    let mut arg_names: Vec<Option<Interned<IString>>> = (0..argc).map(|_| None).collect();
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
                vec![V1Root::throw(&msg, ctx).into()]
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
    ctx.set_span(statement.source_info.span);
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
            let arg = spread_arg.as_u32() - 1 + u32::try_from(arg_id).unwrap();
            let arg = V1Node::V2(ctx.alloc_node(cilly::v2::CILNode::LdArg(arg)));
            let repacked = V1Node::V2(ctx.alloc_node(cilly::v2::CILNode::LdLocA(repacked)));
            repack_cil.push(
                V1Root::SetField {
                    addr: Box::new(repacked),
                    value: Box::new(arg),
                    desc: (arg_field),
                }
                .into(),
            );
        }
        repack_cil
    } else {
        vec![]
    };
    let sig_idx = ctx.alloc_sig(sig.clone());
    // Used for type-checking the CIL to ensure its validity.
    for (last_bb_id, block_data) in blocks.into_iter().enumerate() {
        let mut trees = Vec::new();
        for statement in &block_data.statements {
            if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(V1Root::debug(&format!("{statement:?}"),ctx).into())};
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(V1Root::debug(&format!("{:?}",statement.source_info.span),ctx).into())};
            }

            let statement_tree = match statement_to_ops(statement, ctx) {
                Ok(ops) => ops,
                Err(err) => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!(
                        "Method \"{name}\" failed to compile statement {statement:?} with message {err:?}\n"
                    )};
                    rustc_middle::ty::print::with_no_trimmed_paths! {vec![(V1Root::throw(&format!("Tired to run a statement {statement:?} which failed to compile with error message {err:?}."),ctx).into())]}
                }
            };
            for tree in &statement_tree {
                let Err(err) = tree.root().try_typecheck(ctx, sig_idx, &locals) else {
                    continue;
                };
                ctx.tcx().dcx().span_warn(
                    statement.source_info.span,
                    format!("Typecheck failed:{err:?}"),
                );
            }
            // Only save debuginfo for statements which result in ops.
            if !statement_tree.is_empty() {
                trees.push(span_source_info(ctx.tcx(), statement.source_info.span).into());
            }
            trees.extend(statement_tree);
        }
        if let Some(term) = &block_data.terminator {
            if *crate::config::INSERT_MIR_DEBUG_COMMENTS {
                rustc_middle::ty::print::with_no_trimmed_paths! {trees.push(V1Root::debug(&format!("{term:?}"),ctx).into())};
            }
            let term_trees = terminator_to_ops(term, ctx).unwrap_or_else(|err| {
                panic!("Could not compile terminator {term:?} because {err:?}")
            });
            for tree in &term_trees {
                let Err(err) = tree.root().try_typecheck(ctx, sig_idx, &locals) else {
                    continue;
                };
                ctx.tcx()
                    .dcx()
                    .span_warn(term.source_info.span, format!("Typecheck failed:{err:?}"));
            }
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

    let main_module = ctx.main_module();
    let mut method = MethodDef::from_v1(&method, ctx, main_module);
    if let Err(err) = method.typecheck(ctx) {
        ctx.tcx()
            .dcx()
            .span_warn(ctx.body().span, format!("Typecheck failed {err:?}"));
    };
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
            let instance = rustc_middle::ty::Instance::new(stotic, rustc_middle::ty::List::empty());
            let mut ctx = MethodCompileCtx::new(tcx, None, instance, asm);
            let int8_ptr = ctx.nptr(Type::Int(Int::I8));
            let int8_ptr_ptr = ctx.nptr(int8_ptr);
            if let Some(section) = attrs.link_section {
                if section.to_string().contains(".init_array") {
                    let argc = utilis::argc_argv_init_method(&mut ctx);
                    let init_argc = ctx.alloc_root(cilly::CILRoot::call(argc, []));

                    ctx.add_user_init(&[init_argc]);
                    let get_environ: Interned<MethodRef> = utilis::get_environ(&mut ctx);
                    let fn_ptr = alloc.0.provenance().ptrs().iter().next().unwrap();
                    let fn_ptr = tcx.global_alloc(fn_ptr.1.alloc_id());
                    let init_call_site = if let GlobalAlloc::Function {
                        instance: finstance,
                    } = fn_ptr
                    {
                        let mut ctx = MethodCompileCtx::new(tcx, None, finstance, &mut ctx);
                        // If it is a function, patch its pointer up.
                        let call_info = CallInfo::sig_from_instance_(finstance, &mut ctx);
                        let function_name = function_name(tcx.symbol_name(finstance));
                        MethodRef::new(
                            *ctx.main_module(),
                            ctx.alloc_string(function_name),
                            ctx.alloc_sig(call_info.sig().clone()),
                            MethodKind::Static,
                            vec![].into(),
                        )
                    } else {
                        panic!()
                    };

                    let argv = ctx.alloc_string("argv");
                    let argc = ctx.alloc_string("argc");
                    let main_module = ctx.main_module();
                    let mref = ctx.alloc_methodref(init_call_site);
                    let argv =
                        ctx.alloc_sfld(StaticFieldDesc::new(*main_module, argv, int8_ptr_ptr));
                    let argc = ctx.alloc_sfld(StaticFieldDesc::new(
                        *main_module,
                        argc,
                        Type::Int(Int::I32),
                    ));
                    let argv = ctx.alloc_node(cilly::CILNode::LdStaticField(argv));
                    let uint8_ptr = ctx.nptr(Int::U8);
                    let uint8_ptr_idx = ctx.alloc_type(uint8_ptr);
                    let args = [
                        ctx.alloc_node(cilly::CILNode::LdStaticField(argc)),
                        ctx.alloc_node(cilly::CILNode::PtrCast(
                            argv,
                            Box::new(PtrCastRes::Ptr(uint8_ptr_idx)),
                        )),
                        ctx.alloc_node(cilly::CILNode::call(get_environ, [])),
                    ];
                    let root = ctx.alloc_root(cilly::CILRoot::call(mref, args));
                    ctx.add_user_init(&[root]);
                } else {
                    panic!("Unsuported link section {section}.")
                }
            }

            add_static(stotic, &mut ctx);

            drop(static_compile_timer);

            Ok(())
        }
    }
}

pub(crate) fn span_source_info(tcx: TyCtxt, span: rustc_span::Span) -> V1Root {
    let (file, lstart, cstart, lend, mut cend) = tcx.sess.source_map().span_to_location_info(span);
    let file = file.map_or(String::new(), |file| {
        file.name
            .display(rustc_span::FileNameDisplayPreference::Local)
            .to_string()
    });
    if cstart >= cend {
        cend = cstart + 1;
    }
    V1Root::source_info(
        &file,
        (lstart as u64)..(lend as u64),
        (cstart as u64)..(cend as u64),
    )
}
