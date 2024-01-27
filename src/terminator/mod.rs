use std::process::Termination;

use crate::place::place_set;
use crate::utilis::garg_to_string;
use crate::{
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    operand::handle_operand,
    r#type::DotnetTypeRef,
    utilis::monomorphize,
    utilis::CTOR_FN_NAME,
    utilis::MANAGED_CALL_FN_NAME,
    utilis::MANAGED_CALL_VIRT_FN_NAME,
};
use rustc_middle::ty::InstanceDef;
use rustc_middle::{
    mir::{Body, Operand, Place, SwitchTargets, Terminator, TerminatorKind},
    ty::{GenericArg, Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
mod call;
mod intrinsics;
pub fn handle_terminator<'ctx>(
    terminator: &Terminator<'ctx>,
    body: &'ctx Body<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    match &terminator.kind {
        TerminatorKind::Call {
            func,
            args,
            destination,
            target,
            unwind: _,
            call_source: _,
            fn_span: _,
        } => {
            let mut ops = Vec::new();
            match func {
                Operand::Constant(fn_const) => {
                    let fn_ty = fn_const.ty();
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
                    );
                    ops.extend(call_ops);
                }
                Operand::Copy(operand) | Operand::Move(operand) => {
                    let operand_ty = operand.ty(method, tyctx);
                    if let TyKind::FnPtr(sig) = operand_ty.ty.kind() {
                        let sig = crate::utilis::monomorphize(&method_instance, *sig, tyctx);
                        let sig =
                            FnSig::from_poly_sig(Some(method_instance), tyctx, type_cache, sig);
                        let mut call_ops = Vec::new();
                        for arg in args {
                            call_ops.extend(crate::operand::handle_operand(
                                &arg.node,
                                tyctx,
                                body,
                                method_instance,
                                type_cache,
                            ));
                        }
                        call_ops.extend(crate::place::place_get(
                            operand,
                            tyctx,
                            method,
                            method_instance,
                            type_cache,
                        ));
                        call_ops.push(CILOp::CallI(sig.clone().into()));
                        if *sig.output() == crate::r#type::Type::Void {
                            ops.extend(call_ops);
                        } else {
                            ops.extend(place_set(
                                destination,
                                tyctx,
                                call_ops,
                                method,
                                method_instance,
                                type_cache,
                            ));
                        }
                    } else {
                        let fn_ty = monomorphize(&method_instance, operand_ty, tyctx).ty;
                        //let fn_instance = Instance::resolve(tyctx,ParamEnv::reveal_all,fn_ty.did,List::empty());
                        assert!(
                            fn_ty.is_fn(),
                            "fn_ty{fn_ty:?} in call is not a function type!"
                        );
                        let call_ops = call::call(
                            fn_ty,
                            body,
                            tyctx,
                            args,
                            destination,
                            method_instance,
                            type_cache,
                        );
                        ops.extend(call_ops);
                    };
                }
            }
            if let Some(target) = target {
                ops.push(CILOp::GoTo(target.as_u32()));
            }
            ops
        }
        TerminatorKind::Return => {
            let ret = crate::utilis::monomorphize(&method_instance, method.return_ty(), tyctx);
            if type_cache.type_from_cache(ret, tyctx, Some(method_instance))
                == crate::r#type::Type::Void
            {
                vec![CILOp::Ret]
            } else {
                vec![CILOp::LDLoc(0), CILOp::Ret]
            }
        }
        TerminatorKind::SwitchInt { discr, targets } => {
            let ty = crate::utilis::monomorphize(&method_instance, discr.ty(method, tyctx), tyctx);
            let discr =
                crate::operand::handle_operand(discr, tyctx, method, method_instance, type_cache);
            handle_switch(ty, &discr, targets)
        }
        TerminatorKind::Assert {
            cond: _,
            expected: _,
            msg: _,
            target,
            unwind: _,
        } => {
            //let mut ops = handle_operand(cond, tyctx, method, method_instance, type_cache);
            //ops.push(CILOp::LdcI32(i32::from(*expected)));
            //ops.push(CILOp::BEq(target.as_u32()));
            //ops.extend(throw_assert_msg(
            //msg,
            //tyctx,
            //method,
            //method_instance,
            //type_cache,
            //));
            //ops
            let _ = throw_assert_msg;
            vec![CILOp::GoTo(target.as_u32())]
        }
        TerminatorKind::Goto { target } => vec![CILOp::GoTo((*target).into())],
        TerminatorKind::UnwindResume => {
            eprintln!("WARNING: stack unwiniding is not supported yet in rustc_codegen_clr!");
            vec![CILOp::Comment(
                "WARNING: stack unwiniding is not supported yet in rustc_codegen_clr!".into(),
            )]
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
                vec![CILOp::GoTo(target.as_u32())]
            } else {
                let sig = FnSig::sig_from_instance_(drop_instance, tyctx, type_cache).unwrap();

                let function_name = crate::utilis::function_name(tyctx.symbol_name(drop_instance));
                let mut call =
                    crate::place::place_adress(place, tyctx, method, method_instance, type_cache);

                call.push(CILOp::Call(CallSite::boxed(None, function_name, sig, true)));
                //dprintln!("drop call:{call:?}");
                call.push(CILOp::GoTo(target.as_u32()));
                call
            }
        }
        TerminatorKind::Unreachable => CILOp::throw_msg("Unreachable reached!").into(),
        TerminatorKind::InlineAsm {
            template,
            operands,
            options,
            line_spans,
            destination,
            unwind,
        } => {
            eprintln!("Inline assembly is not yet supported!");
            CILOp::throw_msg("Inline assembly is not yet supported!").to_vec()
        }

        _ => todo!("Unhandled terminator kind {kind:?}", kind = terminator.kind),
    }
}
fn throw_assert_msg<'ctx>(
    msg: &rustc_middle::mir::AssertMessage<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    use rustc_middle::mir::AssertKind;
    // Assertion messages cause miscomplations.
    if true {
        return vec![CILOp::LdNull, CILOp::Throw];
    };
    match msg {
        AssertKind::BoundsCheck { len, index } => {
            let mut ops = Vec::with_capacity(8);
            ops.push(CILOp::LdStr("index out of bounds: the len is ".into()));
            ops.extend(handle_operand(
                len,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            let usize_class = crate::utilis::usize_class();
            let string_class = crate::utilis::string_class();
            let string_type = crate::r#type::Type::DotnetType(Box::new(string_class.clone()));
            let sig = FnSig::new(&[], &string_type);
            let usize_to_string = CallSite::boxed(Some(usize_class), "ToString".into(), sig, false);
            ops.push(CILOp::Call(usize_to_string.clone()));
            ops.push(CILOp::LdStr(" but the index is".into()));
            ops.extend(handle_operand(
                index,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            ops.push(CILOp::Call(usize_to_string.clone()));

            let sig = FnSig::new(
                &[
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                ],
                &string_type.clone(),
            );
            let out_of_range_exception =
                DotnetTypeRef::new(Some("System.Runtime"), "System.IndexOutOfRangeException");
            ops.push(CILOp::Call(CallSite::boxed(
                Some(string_class),
                "Concat".into(),
                sig,
                true,
            )));
            let sig = FnSig::new(&[string_type], &crate::r#type::Type::Void);
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(out_of_range_exception),
                ".ctor".into(),
                sig,
                false,
            )));
            ops.push(CILOp::Throw);
            ops
        }
        AssertKind::DivisionByZero(_operand) => {
            let mut ops = Vec::with_capacity(8);

            let sig = FnSig::new(&[], &crate::r#type::Type::Void);
            let div_by_zero_exception =
                DotnetTypeRef::new(Some("System.Runtime"), "System.DivideByZeroException");
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(div_by_zero_exception),
                ".ctor".into(),
                sig,
                false,
            )));
            ops.push(CILOp::Throw);
            ops
        }
        AssertKind::RemainderByZero(_operand) => {
            let mut ops = Vec::with_capacity(8);

            let sig = FnSig::new(&[], &crate::r#type::Type::Void);
            let div_by_zero_exception =
                DotnetTypeRef::new(Some("System.Runtime"), "System.DivideByZeroException");
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(div_by_zero_exception),
                ".ctor".into(),
                sig,
                false,
            )));
            ops.push(CILOp::Throw);
            ops
        }
        AssertKind::Overflow(binop, a, b) => {
            let mut ops = Vec::with_capacity(8);
            let string_class = crate::utilis::string_class();
            ops.push(CILOp::LdStr(
                format!("attempt to {binop:?} with overflow lhs:").into(),
            ));
            ops.extend(handle_operand(
                a,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            let usize_class = crate::utilis::usize_class();
            let string_type = crate::r#type::Type::DotnetType(Box::new(string_class.clone()));
            let sig = FnSig::new(&[], &string_type);
            let usize_to_string = CallSite::boxed(Some(usize_class), "ToString".into(), sig, false);
            ops.push(CILOp::Call(usize_to_string.clone()));
            ops.push(CILOp::LdStr("rhs:".into()));
            ops.extend(handle_operand(
                b,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            ops.push(CILOp::Call(usize_to_string.clone()));

            let sig = FnSig::new(
                &[
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                ],
                &string_type.clone(),
            );
            ops.push(CILOp::Call(CallSite::boxed(
                Some(string_class),
                "Concat".into(),
                sig,
                true,
            )));
            let sig = FnSig::new(&[string_type], &crate::r#type::Type::Void);
            let ovefow_exception =
                DotnetTypeRef::new(Some("System.Runtime"), "System.ArithmeticException");
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(ovefow_exception),
                ".ctor".into(),
                sig,
                false,
            )));
            ops.push(CILOp::Throw);
            ops
        }
        AssertKind::MisalignedPointerDereference {
            required: _,
            found: _,
        } => {
            /*
            let mut ops = Vec::with_capacity(8);
            let string_class = crate::utilis::string_class();
            ops.push(CILOp::LdStr(
                "Missaligned pointer dereference. required: "
                    .to_string()
                    .into(),
            ));
            ops.extend(handle_operand(
                required,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            let usize_class = crate::utilis::usize_class();
            let string_type = crate::r#type::Type::DotnetType(string_class.clone().into());
            let sig = FnSig::new(&[], &string_type);
            let usize_to_string = CallSite::boxed(Some(usize_class), "ToString".into(), sig, false);
            ops.push(CILOp::Call(usize_to_string.clone()));
            ops.push(CILOp::LdStr(" found: ".into()));
            ops.extend(handle_operand(
                found,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            ops.push(CILOp::Call(usize_to_string.clone()));

            let sig = FnSig::new(
                &[
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                ],
                &string_type.clone(),
            );
            ops.push(CILOp::Call(CallSite::boxed(
                Some(string_class),
                "Concat".into(),
                sig,
                true,
            )));
            let sig = FnSig::new(&[string_type], &crate::r#type::Type::Void);
            let ovefow_exception = DotnetTypeRef::new(Some("System.Runtime"), "System.Exception");
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(ovefow_exception),
                ".ctor".into(),
                sig,
                false,
            )));
            ops.push(CILOp::Throw);
            ops*/
            vec![]
        }
        AssertKind::OverflowNeg(value) => {
            let mut ops = Vec::with_capacity(8);
            let string_class = crate::utilis::string_class();
            ops.push(CILOp::LdStr(
                "attempt to neg with overflow value:".to_string().into(),
            ));
            ops.extend(handle_operand(
                value,
                tyctx,
                method,
                method_instance,
                type_cache,
            ));
            let usize_class = crate::utilis::usize_class();
            let string_type = crate::r#type::Type::DotnetType(Box::new(string_class.clone()));
            let sig = FnSig::new(&[], &string_type);
            let usize_to_string = CallSite::boxed(Some(usize_class), "ToString".into(), sig, false);
            ops.push(CILOp::Call(usize_to_string.clone()));
            ops.push(CILOp::LdStr("rhs:".into()));

            let sig = FnSig::new(
                &[
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                ],
                &string_type.clone(),
            );
            ops.push(CILOp::Call(CallSite::boxed(
                Some(string_class),
                "Concat".into(),
                sig,
                true,
            )));
            let sig = FnSig::new(&[string_type], &crate::r#type::Type::Void);
            let ovefow_exception =
                DotnetTypeRef::new(Some("System.Runtime"), "System.ArithmeticException");
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(ovefow_exception),
                ".ctor".into(),
                sig,
                false,
            )));
            ops.push(CILOp::Throw);
            ops
        }
        _ => todo!("unsuported assertion message:{msg:?}"),
    }
}
fn handle_switch(ty: Ty, discr: &[CILOp], switch: &SwitchTargets) -> Vec<CILOp> {
    let mut ops = Vec::new();
    for (value, target) in switch.iter() {
        ops.extend(discr.iter().cloned());
        ops.extend(match ty.kind() {
            TyKind::Int(int) => crate::constant::load_const_int(value, int),
            TyKind::Uint(uint) => crate::constant::load_const_uint(value, uint),
            TyKind::Bool => vec![CILOp::LdcI32(
                u8::try_from(value)
                    .expect("Bool value outside of range 0-255. Should be either 0 OR 1.")
                    as i32,
            )],
            TyKind::Char => crate::constant::load_const_uint(value, &rustc_middle::ty::UintTy::U64),
            _ => todo!("Unsuported switch discriminant type {ty:?}"),
        });
        //ops.push(CILOp::LdcI64(value as i64));
        ops.push(CILOp::BEq(target.into()));
    }
    ops.push(CILOp::GoTo(switch.otherwise().into()));
    ops
}
