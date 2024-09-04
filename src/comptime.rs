use crate::fn_ctx::MethodCompileCtx;
use cilly::v2::ClassDef;
#[derive(Clone)]
enum ComptimeLocalVar {
    NotSet,
    Void,
    ClassDef(ClassDef),
}

impl ComptimeLocalVar {
    fn as_type_def(&self) -> Option<&ClassDef> {
        if let Self::ClassDef(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
pub fn interpret<'tcx>(
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    body: &'tcx rustc_middle::mir::Body<'tcx>,
) {
    let block_id = rustc_middle::mir::BasicBlock::from_usize(0);
    let locals = vec![ComptimeLocalVar::NotSet; body.local_decls.len()];
    todo!();
    /*
    loop {
        let block_data = &body.basic_blocks[block_id];
        // Skip cleanup in the interpreter
        assert!(
            !block_data.is_cleanup,
            "Can't interpret a cleanup block in rustc_codegen_clr comptime"
        );
        for statement in &block_data.statements {
            match &statement.kind {
                StatementKind::Assign(bx) => {
                    let (target, rvalue) = bx.as_ref();
                    let src = match rvalue {
                        Rvalue::Use(src) => src,
                        Rvalue::Cast(
                            rustc_middle::mir::CastKind::PointerCoercion(
                                rustc_middle::ty::adjustment::PointerCoercion::ReifyFnPointer,
                            ),
                            _,
                            _,
                        ) => continue,
                        _ => panic!(),
                    };

                    let src = src.place().unwrap().as_local().unwrap();
                    let target = target.as_local().unwrap();
                    locals[usize::from(target)] = locals[usize::from(src)].clone();
                }
                StatementKind::StorageLive(_) | StatementKind::StorageDead(_) => (),
                _ => todo!(
                    "can't interpret the statement {statement:?} yet in rustc_codegen_clr comptime"
                ),
            }
        }
        match &block_data.terminator {
            Some(term) => match &term.kind {
                TerminatorKind::Call {
                    func,
                    args,
                    destination,
                    target,
                    unwind: _,
                    call_source: _,
                    fn_span: _,
                } => {
                    let func_ty = func.ty(body, ctx.tcx());
                    let func_ty = ctx.monomorphize(func_ty);
                    let (call_instance, subst_ref) = if let TyKind::FnDef(def_id, subst_ref) =
                        func_ty.kind()
                    {
                        let subst_ref = ctx.monomorphize(*subst_ref);
                        let env = ParamEnv::reveal_all();
                        let Some(call_instance) =
                            Instance::try_resolve(ctx.tcx(), env, *def_id, subst_ref)
                                .expect("Invalid function def")
                        else {
                            panic!("ERROR: Could not get function instance. fn type:{func_ty:?}")
                        };

                        (call_instance, subst_ref)
                    } else {
                        todo!("Trying to call a type which is not a function definition!");
                    };
                    let function_name =
                        crate::utilis::function_name(ctx.tcx().symbol_name(call_instance));
                    let local = destination
                        .as_local()
                        .expect("ERROR: unuported operation in interop type definiton.");
                    locals[usize::from(local)] = if function_name
                        .contains("rustc_codegen_clr_new_typedef")
                    {
                        let name = crate::utilis::garg_to_string(subst_ref[0], ctx.tcx())
                            .replace("::", ".");
                        let is_value_type = crate::utilis::garag_to_bool(subst_ref[1], ctx.tcx());
                        let superclass_asm = crate::utilis::garg_to_string(subst_ref[2], ctx.tcx())
                            .replace("::", ".");
                        let superclass_name =
                            crate::utilis::garg_to_string(subst_ref[3], ctx.tcx())
                                .replace("::", ".");
                        let inherits = if superclass_name.is_empty() {
                            None
                        } else {
                            Some(ClassRef::new(
                                Some(superclass_asm).and_then(|superclass_asm| {
                                    if superclass_asm.is_empty() {
                                        None
                                    } else {
                                        Some(superclass_asm)
                                    }
                                }),
                                superclass_name,
                                is_value_type,
                                [].into(),
                            ))
                        };
                        let tdef = ClassDef::new(
                            cilly::access_modifier::AccessModifer::Public,
                            name.into(),
                            vec![],
                            vec![],
                            vec![],
                            None,
                            0,
                            inherits,
                            None,
                        );

                        ComptimeLocalVar::ClassDef(tdef)
                    } else if function_name.contains("rustc_codegen_clr_finish_type") {
                        let local = args[0]
                            .node
                            .place()
                            .expect("ERROR: unuported operation in interop type definiton.")
                            .as_local()
                            .expect("ERROR: unuported operation in interop type definiton.");
                        ctx.asm_mut()
                            .class_def(locals[usize::from(local)].as_type_def().unwrap().clone());
                        ComptimeLocalVar::Void
                    } else if function_name.as_ref() == "black_box" {
                        ComptimeLocalVar::NotSet
                    } else if function_name.contains("rustc_codegen_clr_add_field_def") {
                        let src = args[0]
                            .node
                            .place()
                            .expect("ERROR: unuported operation in interop type definiton.")
                            .as_local()
                            .expect("ERROR: unuported operation in interop type definiton.");
                        let mut type_def = locals[usize::from(src)].as_type_def().unwrap().clone();
                        let tpe = crate::utilis::monomorphize(
                            &instance,
                            subst_ref[0].as_type().unwrap(),
                            tcx,
                        );
                        let tpe = get_type(tpe, ctx);
                        let name = crate::utilis::garg_to_string(subst_ref[1], ctx.tcx());
                        type_def.add_field(name.into(), tpe);
                        ComptimeLocalVar::ClassDef(type_def)
                    } else if function_name.contains("rustc_codegen_clr_add_method_def") {
                        let src = args[0]
                            .node
                            .place()
                            .expect("ERROR: unuported operation in interop type definiton.")
                            .as_local()
                            .expect("ERROR: unuported operation in interop type definiton.");

                        let mut type_def = locals[usize::from(src)].as_type_def().unwrap().clone();
                        let fn_type = crate::utilis::monomorphize(
                            &instance,
                            subst_ref[3].as_type().unwrap(),
                            ctx.tcx(),
                        );
                        let this_fname = crate::utilis::garg_to_string(subst_ref[2], ctx.tcx())
                            .replace("::", ".");
                        let def_instance = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind()
                        {
                            let subst_ref =
                                crate::utilis::monomorphize(&instance, *subst_ref, ctx.tcx());
                            let env = ParamEnv::reveal_all();
                            let Some(instance) =
                                Instance::try_resolve(ctx.tcx(), env, *def_id, subst_ref)
                                    .expect("Invalid function def")
                            else {
                                panic!(
                                    "ERROR: Could not get function instance. fn type:{fn_type:?}"
                                )
                            };

                            instance
                        } else {
                            todo!("Trying to call a type which is not a function definition!");
                        };
                        let call_info = CallInfo::sig_from_instance_(def_instance, ctx);
                        let target_function_name =
                            crate::utilis::function_name(ctx.tcx().symbol_name(def_instance));
                        let call_site = CallSite::new(
                            None,
                            target_function_name,
                            call_info.sig().clone(),
                            true,
                        );

                        let method = Method::alias_for(
                            cilly::access_modifier::AccessModifer::Public,
                            // Only virtuals for now
                            cilly::method::MethodType::Virtual,
                            this_fname.into(),
                            call_site,
                        );
                        type_def.add_method(method);

                        //type_def.add_method(name.into(), tpe);
                        ComptimeLocalVar::ClassDef(type_def)
                    } else {
                        todo!("Can't yet call the rustc_codegen_clr comptime interop fn named {function_name:?}")
                    };
                    block_id = target.unwrap();
                }
                TerminatorKind::Return => return,
                _ => todo!("can't interpret the term {term:?} yet in rustc_codegen_clr comptime"),
            },
            None => panic!("Terminatorless block"),
        }
    } */
}
