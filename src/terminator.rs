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
};
fn call<'ctx>(
    fn_type: &Ty<'ctx>,
    body: &'ctx Body<'ctx>,
    tyctx: TyCtxt<'ctx>,
    args: &[Operand<'ctx>],
    destination: &Place<'ctx>,
) -> Vec<CILOp> {
    let instance = if let TyKind::FnDef(def_id, subst_ref) = fn_type.kind() {
        let env = ParamEnv::empty();
        let instance = Instance::resolve(tyctx, env, *def_id, subst_ref)
            .expect("Error: could not resolve a call target due to an external error!")
            .expect("Error: could not resolve a call target!");
        instance
    } else {
        panic!("Trying to call a type which is not a function!");
    };
    let symbol = tyctx.symbol_name(instance);
    let signature = FnSig::from_poly_sig(&fn_type.fn_sig(tyctx), tyctx)
        .expect("Can't get the function signature");
    let function_name = format!("{symbol}").into();
    let mut call = Vec::new();
    for arg in args {
        call.extend(crate::operand::handle_operand(arg, tyctx, body));
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
        crate::place::place_set(destination, tyctx, call, body)
    }
}
pub fn handle_terminator<'ctx>(
    terminator: &Terminator<'ctx>,
    body: &'ctx Body<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
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
                        let call_ops = call(&fn_ty, body, tyctx, args, destination);
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
        TerminatorKind::Return => vec![CILOp::LDLoc(0), CILOp::Ret],
        TerminatorKind::SwitchInt { discr, targets } => {
            let discr = crate::operand::handle_operand(discr, tyctx, method);
            handle_switch(discr, targets)
        }
        _ => todo!("Unhandled terminator kind {kind:?}", kind = terminator.kind),
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
