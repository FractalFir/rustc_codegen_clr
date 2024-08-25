use crate::{assembly::MethodCompileCtx, place::place_set};
use cilly::{
    call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, cil_tree::CILTree, conv_usize,
    field_desc::FieldDescriptor, ld_field, ldc_u32, ptr, FnSig, Type,
};
use rustc_middle::{
    mir::{BasicBlock, Operand, Place, SwitchTargets, Terminator, TerminatorKind},
    ty::{Instance, InstanceKind, ParamEnv, Ty, TyKind},
};
use rustc_span::source_map::Spanned;

mod call;
mod intrinsics;
pub fn handle_call_terminator<'tycxt>(
    terminator: &Terminator<'tycxt>,
    ctx: &mut MethodCompileCtx<'tycxt, '_, '_, '_>,
    args: &[Spanned<Operand<'tycxt>>],
    destination: &Place<'tycxt>,
    func: &Operand<'tycxt>,
    target: Option<BasicBlock>,
) -> Vec<CILTree> {
    let mut trees = Vec::new();

    let func_ty = func.ty(ctx.body(), ctx.tcx());
    // Get the pointed type, if byref;
    let func_ty = match func_ty.builtin_deref(true) {
        None => func_ty,
        Some(inner) => inner,
    };
    match func_ty.kind() {
        TyKind::FnDef(_, _) => {
            let fn_ty = ctx.monomorphize(func_ty);

            assert!(
                fn_ty.is_fn(),
                "fn_ty{fn_ty:?} in call is not a function type!"
            );
            let fn_ty = ctx.monomorphize(fn_ty);
            let call_ops = call::call(fn_ty, ctx, args, destination, terminator.source_info.span);
            //eprintln!("\nCalling FnDef:{fn_ty:?}. call_ops:{call_ops:?}");
            trees.push(call_ops.into());
        }
        TyKind::FnPtr(sig, _) => {
            //eprintln!("Calling FnPtr:{func_ty:?}");

            let sig = ctx
                .tcx()
                .normalize_erasing_late_bound_regions(ParamEnv::reveal_all(), *sig);
            let sig = crate::function_sig::from_poly_sig(
                ctx.instance(),
                ctx.tcx(),
                ctx.type_cache(),
                sig,
            );
            let mut arg_operands = Vec::new();
            for arg in args {
                arg_operands.push(crate::operand::handle_operand(&arg.node, ctx));
            }
            let called_operand = crate::operand::handle_operand(func, ctx);
            if *sig.output() == cilly::Type::Void {
                trees.push(
                    CILRoot::CallI {
                        sig: Box::new(sig.clone()),
                        fn_ptr: Box::new(called_operand),
                        args: arg_operands.into(),
                    }
                    .into(),
                );
            } else {
                trees.push(
                    place_set(
                        destination,
                        CILNode::CallI(Box::new((
                            sig.clone(),
                            called_operand,
                            arg_operands.into(),
                        ))),
                        ctx,
                    )
                    .into(),
                );
            }
        }
        _ => todo!("Can't call type {func_ty:?}"),
    }
    if cilly::mem_checks() {
        trees.push(
            CILRoot::Call {
                site: Box::new(CallSite::mcheck_check_all()),
                args: [].into(),
            }
            .into(),
        );
    }
    // Final Jump
    if let Some(target) = target {
        trees.push(
            CILRoot::GoTo {
                target: target.as_u32(),
                sub_target: 0,
            }
            .into(),
        );
    } else {
        trees.push(CILRoot::throw("Function returning `Never` returned!").into());
    }
    trees
}
pub fn handle_terminator<'tcx>(
    terminator: &Terminator<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> Vec<CILTree> {
    let res = match &terminator.kind {
        TerminatorKind::Call {
            func,
            args,
            destination,
            target,
            unwind: _,
            call_source: _,
            fn_span: _,
        } => handle_call_terminator(terminator, ctx, args, destination, func, *target),
        TerminatorKind::TailCall { .. } => todo!(),
        TerminatorKind::Return => {
            let ret = ctx.monomorphize(ctx.body().return_ty());
            if ctx.type_from_cache(ret) == cilly::Type::Void {
                vec![CILRoot::VoidRet.into()]
            } else {
                vec![CILRoot::Ret {
                    tree: CILNode::LDLoc(0),
                }
                .into()]
            }
        }
        TerminatorKind::SwitchInt { discr, targets } => {
            let ty = ctx.monomorphize(discr.ty(ctx.body(), ctx.tcx()));
            let discr = crate::operand::handle_operand(discr, ctx);
            handle_switch(ty, &discr, targets)
        }
        TerminatorKind::Assert {
            cond: _,
            expected: _,
            msg: _,
            target,
            unwind: _,
        } => {
            vec![CILRoot::GoTo {
                target: target.as_u32(),
                sub_target: 0,
            }
            .into()]
        }
        TerminatorKind::Goto { target } => vec![CILRoot::GoTo {
            target: target.as_u32(),
            sub_target: 0,
        }
        .into()],
        TerminatorKind::UnwindResume => {
            vec![CILRoot::ReThrow.into()]
        }
        TerminatorKind::Drop {
            place,
            target,
            unwind: _,
            replace: _,
        } => {
            let ty = ctx.monomorphize(place.ty(ctx.body(), ctx.tcx()).ty);

            let drop_instance =
                Instance::resolve_drop_in_place(ctx.tcx(), ty).polymorphize(ctx.tcx());
            if let InstanceKind::DropGlue(_, None) = drop_instance.def {
                //Empty drop, nothing needs to happen.
                vec![CILRoot::GoTo {
                    target: target.as_u32(),
                    sub_target: 0,
                }
                .into()]
            } else {
                match ty.kind() {
                    TyKind::Dynamic(_, _, rustc_middle::ty::DynKind::Dyn) => {
                        let fat_ptr_address = crate::place::place_adress(place, ctx);
                        let fat_ptr_type = ctx.type_from_cache(Ty::new_ptr(
                            ctx.tcx(),
                            ty,
                            rustc_middle::ty::Mutability::Mut,
                        ));
                        // Get the vtable
                        let vtable_ptr = ld_field!(
                            fat_ptr_address.clone(),
                            FieldDescriptor::new(
                                fat_ptr_type.as_dotnet().unwrap(),
                                Type::USize,
                                crate::METADATA.into()
                            )
                        );
                        // Get the addres of the object
                        let obj_ptr = ld_field!(
                            fat_ptr_address,
                            FieldDescriptor::new(
                                fat_ptr_type.as_dotnet().unwrap(),
                                Type::Ptr(Type::Void.into()),
                                crate::DATA_PTR.into()
                            )
                        );
                        // We asusme the drop is the first method in the vtable
                        assert_eq!(
                            rustc_middle::ty::vtable::COMMON_VTABLE_ENTRIES_DROPINPLACE,
                            0
                        );
                        let drop_fn_ptr = CILNode::LDIndPtr {
                            ptr: Box::new(vtable_ptr.cast_ptr(ptr!(Type::DelegatePtr(Box::new(
                                FnSig::new([ptr!(Type::Void)], Type::Void,)
                            ),)))),
                            loaded_ptr: Box::new(Type::DelegatePtr(Box::new(FnSig::new(
                                [ptr!(Type::Void)],
                                Type::Void,
                            )))),
                        };
                        vec![
                            CILRoot::BEq {
                                target: target.as_u32(),
                                sub_target: 0,
                                a: Box::new(drop_fn_ptr.clone().cast_ptr(Type::USize)),
                                b: Box::new(conv_usize!(ldc_u32!(0))),
                            }
                            .into(),
                            CILRoot::CallI {
                                sig: Box::new(FnSig::new([ptr!(Type::Void)], Type::Void)),
                                fn_ptr: Box::new(drop_fn_ptr),
                                args: [obj_ptr].into(),
                            }
                            .into(),
                            CILRoot::GoTo {
                                target: target.as_u32(),
                                sub_target: 0,
                            }
                            .into(),
                        ]
                    }
                    TyKind::Dynamic(_, _, rustc_middle::ty::DynKind::DynStar) => {
                        todo!("Can't drop dyn star yet!")
                    }
                    _ => {
                        let sig = crate::function_sig::sig_from_instance_(
                            drop_instance,
                            ctx.tcx(),
                            ctx.type_cache(),
                        )
                        .unwrap();
                        let function_name =
                            crate::utilis::function_name(ctx.tcx().symbol_name(drop_instance));
                        vec![
                            CILRoot::Call {
                                site: Box::new(CallSite::new(None, function_name, sig, true)),
                                args: [crate::place::place_adress(place, ctx)].into(),
                            }
                            .into(),
                            CILRoot::GoTo {
                                target: target.as_u32(),
                                sub_target: 0,
                            }
                            .into(),
                        ]
                    }
                }
            }
        }
        TerminatorKind::Unreachable => {
            let loc = terminator.source_info.span;
            vec![
                rustc_middle::ty::print::with_no_trimmed_paths! {CILRoot::throw(&format!("Unreachable reached at {loc:?}!")).into()},
            ]
        }
        TerminatorKind::InlineAsm {
            template: _,
            operands: _,
            options: _,
            line_spans: _,
            unwind: _,
            targets: _,
        } => {
            eprintln!("Inline assembly is not yet supported!");
            vec![CILRoot::throw("Inline assembly is not yet supported!").into()]
        }
        TerminatorKind::UnwindTerminate(_) => {
            let loc = terminator.source_info.span;
            vec![
                rustc_middle::ty::print::with_no_trimmed_paths! {CILRoot::throw(&format!("UnwindTerminate reached at {loc:?}!")).into()},
            ]
        }
        TerminatorKind::FalseEdge {
            real_target,
            imaginary_target: _,
        } => {
            // imaginary_target is ignored becase you can't jump to it.
            vec![CILRoot::GoTo {
                target: real_target.as_u32(),
                sub_target: 0,
            }
            .into()]
        }
        // Really just a goto, since it can never unwind.
        TerminatorKind::FalseUnwind {
            real_target,
            unwind: _,
        } => {
            // unwind is ignored becase it can't happen.
            vec![CILRoot::GoTo {
                target: real_target.as_u32(),
                sub_target: 0,
            }
            .into()]
        }
        TerminatorKind::CoroutineDrop {} => todo!("Can't drop corutines yet!"),
        TerminatorKind::Yield {
            value: _,
            resume: _,
            resume_arg: _,
            drop: _,
        } => todo!("Can't yeld yet!"), //_ => todo!("Unhandled terminator kind {kind:?}", kind = terminator.kind),
    };
    let last = res.last().unwrap().root();
    assert!(
        matches!(
            last,
            CILRoot::GoTo { .. }
                | CILRoot::Ret { .. }
                | CILRoot::VoidRet
                | CILRoot::ReThrow
                | CILRoot::Throw(_)
        ),
        "Tree {last:?} did not terminate with an uncoditional jump!."
    );
    res
}

fn handle_switch(ty: Ty, discr: &CILNode, switch: &SwitchTargets) -> Vec<CILTree> {
    let mut trees = Vec::new();
    for (value, target) in switch.iter() {
        //ops.extend(CILOp::debug_msg("Switchin"));

        let const_val = match ty.kind() {
            TyKind::Int(int) => crate::constant::load_const_int(value, *int),
            TyKind::Uint(uint) => crate::constant::load_const_uint(value, *uint),
            TyKind::Bool => {
                if value == 0 {
                    CILNode::LdFalse
                } else {
                    CILNode::LdTrue
                }
            }
            TyKind::Char => crate::constant::load_const_uint(value, rustc_middle::ty::UintTy::U32),
            _ => todo!("Unsuported switch discriminant type {ty:?}"),
        };
        //ops.push(CILOp::LdcI64(value as i64));
        trees.push(
            CILRoot::BTrue {
                target: target.into(),
                cond: crate::binop::cmp::eq_unchecked(ty, discr.clone(), const_val),
                sub_target: 0,
            }
            .into(),
        );
    }
    trees.push(
        CILRoot::GoTo {
            target: switch.otherwise().into(),
            sub_target: 0,
        }
        .into(),
    );
    trees
}
