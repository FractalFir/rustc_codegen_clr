use crate::{
    cil::{CILOp, CallSite}, function_sig::FnSig, operand::handle_operand, place::place_set, r#type::{tycache, DotnetTypeRef, Type},
};
use rustc_middle::{
    mir::{Body, Operand, Place, SwitchTargets, Terminator, TerminatorKind},
    ty::{GenericArg, Instance, ParamEnv, Ty, TyCtxt, TyKind},
};
use tycache::TyCache;
pub fn handle_intrinsic<'tyctx>(
    fn_name: &str,
    args: &[Operand<'tyctx>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
    signature: FnSig,
) -> Vec<CILOp> {
    let mut call = Vec::new();
    for arg in args {
        call.extend(crate::operand::handle_operand(
            arg,
            tyctx,
            body,
            method_instance,
            type_cache,
        ));
    }
    match fn_name {
        "unlikely" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `unlikely` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                handle_operand(&args[0], tyctx, body, method_instance, type_cache),
                body,
                method_instance,
                type_cache,
            )
        }
        "ctpop" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctpop` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let mut res = Vec::new();
            res.extend(handle_operand(&args[0], tyctx, body, method_instance, type_cache));
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::U64], &Type::I32),
                    true,
                )),
                CILOp::ConvU64(false),
            ]);
            place_set(
                destination,
                tyctx,
               res,
                body,
                method_instance,
                type_cache,
            )
        }
        "ctlz" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctlz` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let mut res = Vec::new();
            res.extend(handle_operand(&args[0], tyctx, body, method_instance, type_cache));
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "LeadingZeroCount".into(),
                    FnSig::new(&[Type::U64], &Type::I32),
                    true,
                )),
                CILOp::ConvU64(false),
            ]);
            place_set(
                destination,
                tyctx,
               res,
                body,
                method_instance,
                type_cache,
            )
        }
        "cttz"|"cttz_nonzero" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctlz` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let mut res = Vec::new();
            res.extend(handle_operand(&args[0], tyctx, body, method_instance, type_cache));
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::U64], &Type::I32),
                    true,
                )),
                CILOp::ConvU64(false),
            ]);
            place_set(
                destination,
                tyctx,
               res,
                body,
                method_instance,
                type_cache,
            )
        }
        "rotate_left" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `rotate_left` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let mut res = Vec::new();
            res.extend(handle_operand(&args[0], tyctx, body, method_instance, type_cache));
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "RotateLeft".into(),
                    FnSig::new(&[Type::U64], &Type::I32),
                    true,
                )),
                CILOp::ConvU64(false),
            ]);
            place_set(
                destination,
                tyctx,
               res,
                body,
                method_instance,
                type_cache,
            )
        }
        "write_bytes" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `write_bytes` MUST take in exactly 3 argument!"
            );
            let dst = handle_operand(&args[0], tyctx, body, method_instance, type_cache);
            let val = handle_operand(&args[1], tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2], tyctx, body, method_instance, type_cache);
            let mut ops = dst;
            ops.extend(val);
            ops.extend(count);
            ops.push(CILOp::InitBlk);
            ops
        }
        "exact_div" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `exact_div` MUST take in exactly 2 argument!"
            );
            let lhs = handle_operand(&args[0], tyctx, body, method_instance, type_cache);
            let rhs = handle_operand(&args[1], tyctx, body, method_instance, type_cache);
            let mut ops = lhs;
            ops.extend(rhs);
            ops.push(CILOp::Div);
            place_set(
                destination,
                tyctx,
                ops,
                body,
                method_instance,
                type_cache,
            )
        }
        "type_id"=>{
            /* 
            IL_0000: ldtoken C
        IL_0005: call class [System.Runtime]System.Type [System.Runtime]System.Type::GetTypeFromHandle(valuetype [System.Runtime]System.RuntimeTypeHandle)
        IL_000a: callvirt instance valuetype [System.Runtime]System.Guid [System.Runtime]System.Type::get_GUID()
        IL_000f: pop*/
        todo!("Can't handle type_id yet!");
        }
        "volatile_load"=>{
            [CILOp::Volatile].into_iter().chain(crate::place::deref_op(args[0].ty(body,tyctx).into(), tyctx, &method_instance, type_cache).into_iter()).collect()
        }
        "atomic_load_unordered"=>{
            //NOT ATOMIC!
            [CILOp::Volatile].into_iter().chain(crate::place::deref_op(args[0].ty(body,tyctx).into(), tyctx, &method_instance, type_cache).into_iter()).collect()
        }
        "atomic_load_acquire"=>{
            //NOT ATOMIC!
            [CILOp::Volatile].into_iter().chain(crate::place::deref_op(args[0].ty(body,tyctx).into(), tyctx, &method_instance, type_cache).into_iter()).collect()
        }
        //"bswap"
        "assert_inhabited"=>vec![],
        _ => todo!("Can't handle intrinsic {fn_name}."),
    }
}
