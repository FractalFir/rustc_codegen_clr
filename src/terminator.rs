use rustc_middle::{
    mir::{
        interpret::ConstValue, BasicBlock, Body, Constant, ConstantKind, Operand, Place,
        SwitchTargets, Terminator, TerminatorKind,
    },
    ty::{Instance, ParamEnv, Ty, TyCtxt, TyKind},
};

use crate::{
    cil_op::{CILOp, CallSite},
    function_sig::FnSig,
    operand::handle_operand,
    r#type::DotnetTypeRef,
    utilis::monomorphize,
};
fn call<'ctx>(
    fn_type: &Ty<'ctx>,
    body: &'ctx Body<'ctx>,
    tyctx: TyCtxt<'ctx>,
    args: &[Operand<'ctx>],
    destination: &Place<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    let instance = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind() {
        let env = ParamEnv::reveal_all();
        let instance = Instance::expect_resolve(tyctx, env, *def_id, subst_ref);
        instance
    } else {
        todo!("Trying to call a type which is not a function definition!");
    };
    let signature = FnSig::from_poly_sig(&fn_type.fn_sig(tyctx), tyctx)
        .expect("Can't get the function signature");
    let function_name = crate::utilis::function_name(tyctx.symbol_name(instance));
    let mut call = Vec::new();
    for arg in args {
        call.extend(crate::operand::handle_operand(
            arg,
            tyctx,
            body,
            method_instance,
        ));
    }
    let is_void = matches!(signature.output(), crate::r#type::Type::Void);
    call.push(CILOp::Call(CallSite::boxed(
        None,
        function_name,
        signature,
        true,
    )));
    // Hande
    if is_void {
        call
    } else {
        crate::place::place_set(destination, tyctx, call, body, method_instance)
    }
}
pub fn handle_terminator<'ctx>(
    terminator: &Terminator<'ctx>,
    body: &'ctx Body<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    match &terminator.kind {
        TerminatorKind::Call {
            func,
            args,
            destination,
            target,
            unwind,
            call_source,
            fn_span,
        } => {
            let mut ops = Vec::new();
            match func {
                Operand::Constant(fn_const) => {
                    let Constant {
                        span: _,
                        user_ty: _,
                        literal,
                    } = **fn_const;
                    if let ConstantKind::Val(ConstValue::ZeroSized, fn_ty) = literal {
                        assert!(
                            fn_ty.is_fn(),
                            "literal{literal:?} in call is not a function type!"
                        );
                        let fn_ty = monomorphize(&method_instance, fn_ty, tyctx);
                        let call_ops =
                            call(&fn_ty, body, tyctx, args, destination, method_instance);
                        ops.extend(call_ops);
                    } else {
                        panic!("Invalid function literal!");
                    }
                }
                _ => panic!("called func must be const!"),
            }
            if let Some(target) = target {
                ops.push(CILOp::GoTo(target.as_u32()));
            }
            ops
        }
        TerminatorKind::Return => {
            if crate::r#type::Type::from_ty(method.return_ty(), tyctx) != crate::r#type::Type::Void
            {
                vec![CILOp::LDLoc(0), CILOp::Ret]
            } else {
                vec![CILOp::Ret]
            }
        }
        TerminatorKind::SwitchInt { discr, targets } => {
            let discr = crate::operand::handle_operand(discr, tyctx, method, method_instance);
            handle_switch(discr, targets)
        }
        TerminatorKind::Assert {
            cond,
            expected,
            msg,
            target,
            unwind,
        } => {
            let mut ops = handle_operand(cond, tyctx, method, method_instance);
            ops.push(CILOp::LdcI32(*expected as i32));
            ops.push(CILOp::BEq(target.as_u32()));
            ops.extend(throw_assert_msg(msg, tyctx, method, method_instance));
            ops
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
            unwind,
            replace,
        } => {
            eprintln!("WARNING: drop is not supported yet in rustc_codegen_clr!");
            vec![
                CILOp::Comment("WARNING: drop is not supported yet in rustc_codegen_clr!".into()),
                CILOp::GoTo(target.as_u32()),
            ]
        }
        TerminatorKind::Unreachable => {
            let string_type = crate::r#type::Type::DotnetType(Box::new(DotnetTypeRef::new(
                Some("System.Runtime"),
                "System.String",
            )));
            let exception = DotnetTypeRef::new(Some("System.Runtime"), "System.Exception");
            let sig = FnSig::new(&[string_type], &crate::r#type::Type::Void);
            vec![
                CILOp::LdStr("Undefined behaviour! Unreachable terminator reached!".into()),
                CILOp::NewObj(CallSite::boxed(Some(exception), ".ctor".into(), sig, false)),
                CILOp::Throw,
            ]
        }
        _ => todo!("Unhandled terminator kind {kind:?}", kind = terminator.kind),
    }
}
fn throw_assert_msg<'ctx>(
    msg: &rustc_middle::mir::AssertMessage<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    use rustc_middle::mir::AssertKind;
    match msg {
        AssertKind::BoundsCheck { len, index } => {
            let mut ops = Vec::with_capacity(8);
            ops.push(CILOp::LdStr("index out of bounds: the len is ".into()));
            ops.extend(handle_operand(len, tyctx, method, method_instance));
            let usize_class = DotnetTypeRef::new(Some("System.Runtime"), "System.UIntPtr");
            let string_class = DotnetTypeRef::new(Some("System.Runtime"), "System.String");
            let string_type = crate::r#type::Type::DotnetType(Box::new(string_class.clone()));
            let sig = FnSig::new(&[], &string_type);
            let usize_to_string = CallSite::boxed(Some(usize_class), "ToString".into(), sig, false);
            ops.push(CILOp::Call(usize_to_string.clone()));
            ops.push(CILOp::LdStr(" but the index is".into()));
            ops.extend(handle_operand(index, tyctx, method, method_instance));
            ops.push(CILOp::Call(usize_to_string.clone()));

            let sig = FnSig::new(
                &[
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                ],
                &crate::r#type::Type::Void,
            );
            let out_of_range_exception =
                DotnetTypeRef::new(Some("System.Runtime"), "System.IndexOutOfRangeException");
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(string_class),
                ".ctor".into(),
                sig,
                false,
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
        AssertKind::DivisionByZero(operand) => {
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
        AssertKind::RemainderByZero(operand) => {
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
            let string_class = DotnetTypeRef::new(Some("System.Runtime"), "System.String");
            ops.push(CILOp::LdStr(
                format!("attempt to {binop:?} with overflow lhs:").into(),
            ));
            ops.extend(handle_operand(a, tyctx, method, method_instance));
            let usize_class = DotnetTypeRef::new(Some("System.Runtime"), "System.UIntPtr");
            let string_type = crate::r#type::Type::DotnetType(Box::new(DotnetTypeRef::new(
                Some("System.Runtime"),
                "System.String",
            )));
            let sig = FnSig::new(&[], &string_type);
            let usize_to_string = CallSite::boxed(Some(usize_class), "ToString".into(), sig, false);
            ops.push(CILOp::Call(usize_to_string.clone()));
            ops.push(CILOp::LdStr("rhs:".into()));
            ops.extend(handle_operand(b, tyctx, method, method_instance));
            ops.push(CILOp::Call(usize_to_string.clone()));

            let sig = FnSig::new(
                &[
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                    string_type.clone(),
                ],
                &crate::r#type::Type::Void,
            );
            ops.push(CILOp::NewObj(CallSite::boxed(
                Some(string_class),
                ".ctor".into(),
                sig,
                false,
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
fn handle_switch(discr: Vec<CILOp>, switch: &SwitchTargets) -> Vec<CILOp> {
    let mut ops = Vec::new();
    for (value, target) in switch.iter() {
        ops.extend(discr.iter().cloned());
        ops.push(CILOp::LdcI64(value as i64));
        ops.push(CILOp::BEq(target.into()));
    }
    ops.push(CILOp::GoTo(switch.otherwise().into()));
    ops
}
