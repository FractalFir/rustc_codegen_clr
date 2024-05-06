use crate::cil_tree::cil_node::CILNode;
use crate::cil_tree::cil_root::CILRoot;
use crate::cil_tree::CILTree;
use crate::place::place_set;
use rustc_span::source_map::Spanned;

use crate::{cil::CallSite, function_sig::FnSig, utilis::monomorphize};

use rustc_middle::{
    mir::{BasicBlock, Body, Operand, Place, SwitchTargets, Terminator, TerminatorKind},
    ty::{Instance, InstanceDef, Ty, TyCtxt, TyKind},
};

mod call;
mod intrinsics;
pub fn handle_call_terminator<'tycxt>(
    terminator: &Terminator<'tycxt>,
    body: &'tycxt Body<'tycxt>,
    tyctx: TyCtxt<'tycxt>,
    method: &rustc_middle::mir::Body<'tycxt>,
    method_instance: Instance<'tycxt>,
    type_cache: &mut crate::r#type::TyCache,
    args: &[Spanned<Operand<'tycxt>>],
    destination: &Place<'tycxt>,
    func: &Operand<'tycxt>,
    target: &Option<BasicBlock>,
) -> Vec<CILTree> {
    let mut trees = Vec::new();

    let func_ty = match func {
        Operand::Constant(fn_const) => fn_const.ty(),
        Operand::Copy(called) | Operand::Move(called) => {
            called.ty(method, tyctx).ty
            //rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!("Calling func:{func:?} {:?}", operand_ty)};
        }
    };
    // Get the pointed type, if byref;
    let func_ty = match func_ty.builtin_deref(true) {
        None => func_ty,
        Some(inner) => inner.ty,
    };
    match func_ty.kind() {
        TyKind::FnDef(_, _) => {
            //rustc_middle::ty::print::with_no_trimmed_paths! {eprintln!("call terminator {terminator:?}")};
            //eprintln!("calling {operand_ty:?} indirectly");
            let fn_ty = monomorphize(&method_instance, func_ty, tyctx);
            //let fn_instance = Instance::resolve(tyctx,ParamEnv::reveal_all,fn_ty.did,List::empty());
            assert!(
                fn_ty.is_fn(),
                "fn_ty{fn_ty:?} in call is not a function type!"
            );
            let fn_ty = monomorphize(&method_instance, fn_ty, tyctx);
            //let fn_instance = Instance::resolve(tyctx,ParamEnv::reveal_all,fn_ty.did,List::empty());

            let call_ops = call::call(
                fn_ty,
                body,
                tyctx,
                args,
                destination,
                method_instance,
                type_cache,
                terminator.source_info.span,
            );
            //eprintln!("\nCalling FnDef:{fn_ty:?}. call_ops:{call_ops:?}");
            trees.push(call_ops.into());
        }
        TyKind::FnPtr(sig) => {
            //eprintln!("Calling FnPtr:{func_ty:?}");
            let sig = crate::utilis::monomorphize(&method_instance, *sig, tyctx);
            let sig = FnSig::from_poly_sig(Some(method_instance), tyctx, type_cache, sig);
            let mut arg_operands = Vec::new();
            for arg in args {
                arg_operands.push(crate::operand::handle_operand(
                    &arg.node,
                    tyctx,
                    body,
                    method_instance,
                    type_cache,
                ));
            }
            let called_operand =
                crate::operand::handle_operand(func, tyctx, method, method_instance, type_cache);
            if *sig.output() == crate::r#type::Type::Void {
                trees.push(
                    CILRoot::CallI {
                        sig: sig.clone(),
                        fn_ptr: called_operand,
                        args: arg_operands.into(),
                    }
                    .into(),
                );
            } else {
                trees.push(
                    place_set(
                        destination,
                        tyctx,
                        CILNode::CallI(Box::new((
                            sig.clone(),
                            called_operand,
                            arg_operands.into(),
                        ))),
                        method,
                        method_instance,
                        type_cache,
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
        trees.push(CILRoot::throw("Function returning `Never` returned!").into());
    }
    trees
}
pub fn handle_terminator<'ctx>(
    terminator: &Terminator<'ctx>,
    body: &'ctx Body<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    type_cache: &mut crate::r#type::TyCache,
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
        } => handle_call_terminator(
            terminator,
            body,
            tyctx,
            method,
            method_instance,
            type_cache,
            args,
            destination,
            func,
            target,
        ),
        TerminatorKind::Return => {
            let ret = crate::utilis::monomorphize(&method_instance, method.return_ty(), tyctx);
            if type_cache.type_from_cache(ret, tyctx, Some(method_instance))
                == crate::r#type::Type::Void
            {
                CILRoot::VoidRet.into()
            } else {
                CILRoot::Ret {
                    tree: CILNode::LDLoc(0),
                }
                .into()
            }
        }
        TerminatorKind::SwitchInt { discr, targets } => {
            let ty = crate::utilis::monomorphize(&method_instance, discr.ty(method, tyctx), tyctx);
            let discr =
                crate::operand::handle_operand(discr, tyctx, method, method_instance, type_cache);
            handle_switch(ty, discr, targets)
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
            let ty = monomorphize(&method_instance, place.ty(method, tyctx).ty, tyctx);

            let drop_instance = Instance::resolve_drop_in_place(tyctx, ty).polymorphize(tyctx);
            if let InstanceDef::DropGlue(_, None) = drop_instance.def {
                //Empty drop, nothing needs to happen.
                vec![CILRoot::GoTo {
                    target: target.as_u32(),
                    sub_target: 0,
                }
                .into()]
            } else {
                let sig = FnSig::sig_from_instance_(drop_instance, tyctx, type_cache).unwrap();

                let function_name = crate::utilis::function_name(tyctx.symbol_name(drop_instance));

                vec![
                    CILRoot::Call {
                        site: CallSite::new(None, function_name, sig, true),
                        args: [crate::place::place_adress(
                            place,
                            tyctx,
                            method,
                            method_instance,
                            type_cache,
                        )]
                        .into(),
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
        TerminatorKind::Unreachable => {
            let loc = terminator.source_info.span;
            rustc_middle::ty::print::with_no_trimmed_paths! {CILRoot::throw(&format!("Unreachable reached at {loc:?}!")).into()}
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
            CILRoot::throw("Inline assembly is not yet supported!").into()
        }
        TerminatorKind::UnwindTerminate(_) => {
            let loc = terminator.source_info.span;
            rustc_middle::ty::print::with_no_trimmed_paths! {CILRoot::throw(&format!("UnwindTerminate reached at {loc:?}!")).into()}
        }
        TerminatorKind::FalseEdge {
            real_target,
            imaginary_target: _,
        } => {
            // imaginary_target is ignored becase you can't jump to it.
            CILRoot::GoTo {
                target: real_target.as_u32(),
                sub_target: 0,
            }
            .into()
        }
        // Really just a goto, since it can never unwind.
        TerminatorKind::FalseUnwind {
            real_target,
            unwind: _,
        } => {
            // unwind is ignored becase it can't happen.
            CILRoot::GoTo {
                target: real_target.as_u32(),
                sub_target: 0,
            }
            .into()
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

fn handle_switch(ty: Ty, discr: CILNode, switch: &SwitchTargets) -> Vec<CILTree> {
    let mut trees = Vec::new();
    for (value, target) in switch.iter() {
        //ops.extend(CILOp::debug_msg("Switchin"));

        let const_val = match ty.kind() {
            TyKind::Int(int) => crate::constant::load_const_int(value, int),
            TyKind::Uint(uint) => crate::constant::load_const_uint(value, uint),
            TyKind::Bool => CILNode::LdcI32(
                u8::try_from(value)
                    .expect("Bool value outside of range 0-255. Should be either 0 OR 1.")
                    as i32,
            ),
            TyKind::Char => crate::constant::load_const_uint(value, &rustc_middle::ty::UintTy::U64),
            _ => todo!("Unsuported switch discriminant type {ty:?}"),
        };
        //ops.push(CILOp::LdcI64(value as i64));
        trees.push(
            CILRoot::BTrue {
                target: target.into(),
                ops: crate::binop::cmp::eq_unchecked(ty, discr.clone(), const_val),
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
