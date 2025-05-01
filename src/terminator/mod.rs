use crate::assembly::MethodCompileCtx;
use cilly::{
    cil_node::CILNode, cil_root::CILRoot, cil_tree::CILTree, cilnode::MethodKind, ld_field, BinOp,
    Const, FieldDesc, FnSig, Int, MethodRef, Type,
};
use rustc_codegen_clr_ctx::function_name;
use rustc_codegen_clr_place::{place_adress, place_set};
use rustc_codegen_clr_type::GetTypeExt;
use rustc_middle::mir::AssertKind;

use rustc_codgen_clr_operand::{
    constant::{load_const_int, load_const_uint},
    handle_operand,
};
use rustc_middle::{
    mir::{BasicBlock, Operand, Place, SwitchTargets, Terminator, TerminatorKind},
    ty::{Instance, InstanceKind, Ty, TyKind},
};
use rustc_span::source_map::Spanned;

mod call;
mod intrinsics;
pub fn handle_call_terminator<'tycxt>(
    terminator: &Terminator<'tycxt>,
    ctx: &mut MethodCompileCtx<'tycxt, '_>,
    args: &[Spanned<Operand<'tycxt>>],
    destination: &Place<'tycxt>,
    func: &Operand<'tycxt>,
    target: Option<BasicBlock>,
) -> Vec<CILTree> {
    let mut trees = Vec::new();

    let func_ty = func.ty(ctx.body(), ctx.tcx());
    let fn_ty = ctx.monomorphize(func_ty);
    // Get the pointed type, if byref;
    let func_ty = match func_ty.builtin_deref(true) {
        None => func_ty,
        Some(inner) => inner,
    };
    match func_ty.kind() {
        TyKind::FnDef(_, _) => {
            assert!(
                fn_ty.is_fn(),
                "fn_ty{fn_ty:?} in call is not a function type!"
            );
            let fn_ty = ctx.monomorphize(fn_ty);
            let call_ops = call::call(fn_ty, ctx, args, destination, terminator.source_info.span);
            //eprintln!("\nCalling FnDef:{fn_ty:?}. call_ops:{call_ops:?}");
            trees.extend(call_ops.into_iter().map(std::convert::Into::into));
        }
        TyKind::FnPtr(sig, _) => {
            //eprintln!("Calling FnPtr:{func_ty:?}");

            let sig = ctx.tcx().instantiate_bound_regions_with_erased(*sig);
            let sig = crate::function_sig::from_poly_sig(ctx, sig);
            let mut arg_operands = Vec::new();
            for arg in args {
                arg_operands.push(handle_operand(&arg.node, ctx));
            }
            let called_operand = handle_operand(func, ctx);
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
        trees.push(CILRoot::throw("Function returning `Never` returned!", ctx).into());
    }
    trees
}
pub fn handle_terminator<'tcx>(
    terminator: &Terminator<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
            let discr = handle_operand(discr, ctx);
            handle_switch(ty, &discr, targets, ctx)
        }
        TerminatorKind::Assert {
            cond,
            expected,
            msg,
            target,
            unwind: _,
        } => {
            let cond = if *expected {
                handle_operand(cond, ctx)
            } else {
                CILNode::Eq(
                    Box::new(handle_operand(cond, ctx)),
                    Box::new(CILNode::V2(ctx.alloc_node(*expected))),
                )
            };
            // FIXME: propelrly handle *all* assertion messages.
            let main = ctx.main_module();

            let name = match msg.as_ref() {
                AssertKind::Overflow(op, _, _) => {
                    let op: BinOp = crate::map_binop(op);
                    format!("assert_{}", op.name())
                }
                AssertKind::OverflowNeg(_) => "assert_neg_overflow".into(),
                AssertKind::BoundsCheck { len, index } => {
                    let len = handle_operand(len, ctx);
                    let index = handle_operand(index, ctx);
                    let sig = ctx.sig([Type::Bool], Type::Void);
                    let site = ctx.new_methodref(
                        *main,
                        "assert_bounds_check",
                        sig,
                        MethodKind::Static,
                        vec![],
                    );
                    return vec![
                        CILRoot::Call {
                            site,
                            args: vec![cond].into(),
                        }
                        .into(),
                        CILRoot::GoTo {
                            target: target.as_u32(),
                            sub_target: 0,
                        }
                        .into(),
                    ];
                }
                AssertKind::NullPointerDereference => "assert_notnull".into(),
                AssertKind::MisalignedPointerDereference {
                    required: _,
                    found: _,
                } => "assert_ptr_align".into(),
                AssertKind::DivisionByZero(_) => "assert_zero_div".into(),
                AssertKind::RemainderByZero(_) => "assert_zero_rem".into(),
                AssertKind::ResumedAfterReturn(_) => "assert_coroutine_resume_after_return".into(),
                AssertKind::ResumedAfterPanic(_) => "assert_coroutine_resume_after_panic".into(),
                AssertKind::ResumedAfterDrop(_) => "assert_coroutine_resume_after_drop".into(),
            };
            let sig = ctx.sig([Type::Bool], Type::Void);
            let site = ctx.new_methodref(*main, name, sig, MethodKind::Static, vec![]);
            vec![
                CILRoot::Call {
                    site,
                    args: vec![cond].into(),
                }
                .into(),
                CILRoot::GoTo {
                    target: target.as_u32(),
                    sub_target: 0,
                }
                .into(),
            ]
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
            //TODO: figure out what the hell those 2 fields are doing.
            drop: _,
            async_fut: _,
        } => {
            let ty = ctx.monomorphize(place.ty(ctx.body(), ctx.tcx()).ty);

            let drop_instance = Instance::resolve_drop_in_place(ctx.tcx(), ty);
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
                        let fat_ptr_address = place_adress(place, ctx);
                        let fat_ptr_type = ctx.type_from_cache(Ty::new_ptr(
                            ctx.tcx(),
                            ty,
                            rustc_middle::ty::Mutability::Mut,
                        ));
                        let desc = FieldDesc::new(
                            fat_ptr_type.as_class_ref().unwrap(),
                            ctx.alloc_string(crate::METADATA),
                            Type::Int(Int::USize),
                        );
                        // Get the vtable
                        let vtable_ptr = ld_field!(fat_ptr_address.clone(), ctx.alloc_field(desc));
                        let void_ptr = ctx.nptr(Type::Void);
                        // Get the addres of the object
                        let desc = FieldDesc::new(
                            fat_ptr_type.as_class_ref().unwrap(),
                            ctx.alloc_string(crate::DATA_PTR),
                            void_ptr,
                        );
                        let obj_ptr = ld_field!(fat_ptr_address, ctx.alloc_field(desc));
                        // We asusme the drop is the first method in the vtable
                        assert_eq!(
                            rustc_middle::ty::vtable::COMMON_VTABLE_ENTRIES_DROPINPLACE,
                            0
                        );
                        let sig = ctx.sig([void_ptr], Type::Void);
                        let drop_fn_ptr = CILNode::LDIndPtr {
                            ptr: Box::new(vtable_ptr.cast_ptr(ctx.nptr(Type::FnPtr(sig)))),
                            loaded_ptr: Box::new(Type::FnPtr(sig)),
                        };
                        vec![
                            CILRoot::BEq {
                                target: target.as_u32(),
                                sub_target: 0,
                                a: Box::new(drop_fn_ptr.clone().cast_ptr(Type::Int(Int::USize))),
                                b: Box::new(CILNode::V2(ctx.alloc_node(Const::USize(0)))),
                            }
                            .into(),
                            CILRoot::CallI {
                                sig: Box::new(FnSig::new([void_ptr], Type::Void)),
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
                        let sig =
                            crate::function_sig::sig_from_instance_(drop_instance, ctx).unwrap();
                        let function_name = function_name(ctx.tcx().symbol_name(drop_instance));
                        let mref = MethodRef::new(
                            *ctx.main_module(),
                            ctx.alloc_string(function_name),
                            ctx.alloc_sig(sig),
                            MethodKind::Static,
                            vec![].into(),
                        );
                        vec![
                            CILRoot::Call {
                                site: ctx.alloc_methodref(mref),
                                args: [place_adress(place, ctx)].into(),
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
            let msg = ctx.alloc_string(format!("Unreachable reached at {loc:?}!"));

            vec![
                rustc_middle::ty::print::with_no_trimmed_paths! {CILRoot::V2(ctx.alloc_root(cilly::CILRoot::Unreachable(msg))).into()},
            ]
        }
        TerminatorKind::InlineAsm {
            template: _,
            operands: _,
            options: _,
            line_spans: _,
            unwind: _,
            targets: _,
            asm_macro: _,
        } => {
            eprintln!("Inline assembly is not yet supported!");
            vec![CILRoot::throw("Inline assembly is not yet supported!", ctx).into()]
        }
        TerminatorKind::UnwindTerminate(_) => {
            let loc = terminator.source_info.span;
            vec![
                rustc_middle::ty::print::with_no_trimmed_paths! {CILRoot::debug(&format!("UnwindTerminate reached at {loc:?}!"),ctx).into()},
                CILRoot::ReThrow.into(),
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
                | CILRoot::V2(_)
        ),
        "Tree {last:?} did not terminate with an uncoditional jump!."
    );
    res
}

fn handle_switch<'tcx>(
    ty: Ty<'tcx>,
    discr: &CILNode,
    switch: &SwitchTargets,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Vec<CILTree> {
    let mut trees = Vec::new();
    for (value, target) in switch.iter() {
        //ops.extend(CILOp::debug_msg("Switchin"));

        let const_val = CILNode::V2(match ty.kind() {
            TyKind::Int(int) => load_const_int(value, *int, ctx),
            TyKind::Uint(uint) => load_const_uint(value, *uint, ctx),
            TyKind::Bool => ctx.alloc_node(value != 0),
            TyKind::Char => load_const_uint(value, rustc_middle::ty::UintTy::U32, ctx),
            _ => todo!("Unsuported switch discriminant type {ty:?}"),
        });
        //ops.push(CILOp::LdcI64(value as i64));
        trees.push(
            CILRoot::BTrue {
                target: target.into(),
                cond: crate::binop::cmp::eq_unchecked(ty, discr.clone(), const_val, ctx),
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
