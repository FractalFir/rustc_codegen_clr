use crate::{
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    operand::handle_operand,
    place::place_set,
    r#type::{tycache, DotnetTypeRef, Type},
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, ParamEnv, TyCtxt, TyKind, UintTy},
};
use rustc_span::source_map::Spanned;
use tycache::TyCache;
mod bswap;
pub fn handle_intrinsic<'tyctx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
    signature: FnSig,
) -> Vec<CILOp> {
    let mut call = Vec::new();
    for arg in args {
        call.extend(crate::operand::handle_operand(
            &arg.node,
            tyctx,
            body,
            method_instance,
            type_cache,
        ));
    }
    match fn_name {
        "breakpoint" => {
            debug_assert_eq!(
                args.len(),
                0,
                "The intrinsic `breakpoint` MUST take in exactly 1 argument!"
            );
            vec![CILOp::Break]
        }
        "unlikely" | "likely" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `unlikely` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                body,
                method_instance,
                type_cache,
            )
        }
        "is_val_statically_known" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `is_val_statically_known` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                vec![CILOp::LdcI32(0)],
                body,
                method_instance,
                type_cache,
            )
        }
        "needs_drop" => {
            debug_assert_eq!(
                args.len(),
                0,
                "The intrinsic `needs_drop` MUST take in exactly 0 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let needs_drop = tpe.needs_drop(tyctx, ParamEnv::reveal_all());
            let needs_drop = if needs_drop { 1 } else { 0 };
            place_set(
                destination,
                tyctx,
                vec![CILOp::LdcI32(needs_drop)],
                body,
                method_instance,
                type_cache,
            )
        }
        "black_box" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `black_box` MUST take in exactly 1 argument!"
            );
            if signature.output() == &Type::Void {
                return vec![];
            }
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                body,
                method_instance,
                type_cache,
            )
        }
        "fmaf64" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `fmaf64` MUST take in exactly 1 argument!"
            );
            let mut res = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            res.extend(handle_operand(
                &args[1].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            res.extend(handle_operand(
                &args[2].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            res.extend([CILOp::Mul, CILOp::Add]);
            place_set(destination, tyctx, res, body, method_instance, type_cache)
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
            res.extend(handle_operand(
                &args[0].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
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
            place_set(destination, tyctx, res, body, method_instance, type_cache)
        }
        "ctlz" | "ctlz_nonzero" => {
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
            res.extend(handle_operand(
                &args[0].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            let sub = match tpe {
                Type::ISize | Type::USize | Type::Ptr(_) => 0,
                Type::I64 | Type::U64 => 0,
                Type::I32 | Type::U32 => 32,
                Type::I16 | Type::U16 => 48,
                Type::I8 | Type::U8 => 56,
                _ => todo!("Can't `ctlz`  type {tpe:?} yet!"),
            };
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "LeadingZeroCount".into(),
                    FnSig::new(&[Type::U64], &Type::I32),
                    true,
                )),
                CILOp::LdcI32(sub),
                CILOp::Sub,
                CILOp::ConvU64(false),
            ]);
            place_set(destination, tyctx, res, body, method_instance, type_cache)
        }
        "bswap" => bswap::bswap(args, destination, tyctx, body, method_instance, type_cache),
        "cttz" | "cttz_nonzero" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctlz` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            let bit_operations = Some(bit_operations);
            let mut res = Vec::new();
            res.extend(handle_operand(
                &args[0].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            res.extend([CILOp::Call(CallSite::boxed(
                bit_operations.clone(),
                "TrailingZeroCount".into(),
                FnSig::new(&[tpe], &Type::I32),
                true,
            ))]);
            place_set(destination, tyctx, res, body, method_instance, type_cache)
        }
        "rotate_left" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `rotate_left` MUST take in exactly 2 arguments!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let mut res = Vec::new();
            res.extend(handle_operand(
                &args[0].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            res.extend(handle_operand(
                &args[1].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "RotateLeft".into(),
                    FnSig::new(&[Type::U64, Type::U64], &Type::I32),
                    true,
                )),
                CILOp::ConvU64(false),
            ]);
            place_set(destination, tyctx, res, body, method_instance, type_cache)
        }
        "write_bytes" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `write_bytes` MUST take in exactly 3 argument!"
            );
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let val = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);
            let mut ops = dst;
            ops.extend(val);
            ops.extend(count);
            ops.push(CILOp::InitBlk);
            ops
        }
        "copy" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `copy` MUST take in exactly 3 argument!"
            );
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let val = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);
            let mut ops = dst;
            ops.extend(val);
            ops.extend(count);
            ops.push(CILOp::CpBlk);
            ops
        }
        "exact_div" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `exact_div` MUST take in exactly 2 argument!"
            );
            let lhs = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let rhs = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let mut ops = lhs;
            ops.extend(rhs);
            ops.push(CILOp::Div);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "type_id" => {
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            let sig = FnSig::new(
                &[DotnetTypeRef::type_handle_type().into()],
                &DotnetTypeRef::type_type().into(),
            );
            let gethash_sig = FnSig::new(&[DotnetTypeRef::type_type().into()], &Type::I32);
            place_set(
                destination,
                tyctx,
                vec![
                    CILOp::LDTypeToken(tpe.into()),
                    CILOp::Call(CallSite::boxed(
                        DotnetTypeRef::type_type().into(),
                        "GetTypeFromHandle".into(),
                        sig,
                        true,
                    )),
                    CILOp::CallVirt(CallSite::boxed(
                        DotnetTypeRef::object_type().into(),
                        "GetHashCode".into(),
                        gethash_sig,
                        false,
                    )),
                    CILOp::Call(CallSite::boxed(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Implicit".into(),
                        crate::function_sig::FnSig::new(&[Type::U32], &Type::U128),
                        true,
                    )),
                ],
                body,
                method_instance,
                type_cache,
            )
        }
        "volatile_load" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `volatile_load` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Volatile);
            ops.extend(crate::place::deref_op(
                args[0].node.ty(body, tyctx).into(),
                tyctx,
                &method_instance,
                type_cache,
            ));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "atomic_load_unordered" => {
            //NOT ATOMIC!
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_unordered` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Volatile);
            ops.extend(crate::place::deref_op(
                args[0].node.ty(body, tyctx).into(),
                tyctx,
                &method_instance,
                type_cache,
            ));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "atomic_load_acquire" => {
            //NOT ATOMIC!
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_acquire` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Volatile);
            ops.extend(crate::place::deref_op(
                args[0].node.ty(body, tyctx).into(),
                tyctx,
                &method_instance,
                type_cache,
            ));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        //"bswap"
        "assert_inhabited" => vec![],
        "ptr_offset_from_unsigned" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `min_align_of_val` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            ops.extend(handle_operand(
                &args[1].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            ops.extend([CILOp::Sub, CILOp::SizeOf(tpe.into()), CILOp::Div]);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "min_align_of_val" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `min_align_of_val` MUST take in exactly 1 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            place_set(
                destination,
                tyctx,
                vec![
                    CILOp::LdcI64(crate::utilis::align_of(tpe, tyctx) as i64),
                    CILOp::ConvUSize(false),
                ],
                body,
                method_instance,
                type_cache,
            )
        }
        "sqrtf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf32` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                "Sqrt".into(),
                FnSig::new(&[Type::F32], &Type::F32),
                true,
            )));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "floorf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `floorf32` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                "Floor".into(),
                FnSig::new(&[Type::F32], &Type::F32),
                true,
            )));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "ceilf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ceilf32` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                "Ceiling".into(),
                FnSig::new(&[Type::F32], &Type::F32),
                true,
            )));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "maxnumf32" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `maxnumf32` MUST take in exactly 2 arguments!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.extend(handle_operand(
                &args[1].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                "Max".into(),
                FnSig::new(&[Type::F32, Type::F32], &Type::F32),
                true,
            )));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "minnumf32" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `minnumf32` MUST take in exactly 2 arguments!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.extend(handle_operand(
                &args[1].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                "Min".into(),
                FnSig::new(&[Type::F32, Type::F32], &Type::F32),
                true,
            )));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "powif32" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `powif32` MUST take in exactly 2 arguments!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::ConvF64(false));
            ops.extend(handle_operand(
                &args[1].node,
                tyctx,
                body,
                method_instance,
                type_cache,
            ));
            ops.push(CILOp::ConvF64(false));
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.Math")),
                "Pow".into(),
                FnSig::new(&[Type::F64, Type::F64], &Type::F64),
                true,
            )));
            ops.push(CILOp::ConvF32(false));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "size_of_val" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `size_of_val` MUST take in exactly 1 argument!"
            );
            let tpe = args[0].node.ty(body, tyctx);
            let tpe = crate::utilis::monomorphize(&method_instance, tpe, tyctx);
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            place_set(
                destination,
                tyctx,
                vec![CILOp::SizeOf(tpe.into())],
                body,
                method_instance,
                type_cache,
            )
        }
        "sqrtf64" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
            );
            let mut ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            ops.push(CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::new("System.Runtime".into(), "System.Math")),
                "Sqrt".into(),
                FnSig::new(&[Type::F64], &Type::F64),
                true,
            )));
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "abort" => CILOp::throw_msg("Called abort!").into(),
        _ => todo!("Can't handle intrinsic {fn_name}."),
    }
}

/*
fn saturating_sub<'tyctx>(
    args: &[Operand<'tyctx>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,) ->Vec<CILOp>{
    debug_assert_eq!(
        args.len(),
        2,
        "The intrinsic `saturating_sub` MUST take in exactly 2 arguments!"
    );
    let ty = args[0].ty(body, tyctx);
    let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
    let a = handle_operand(&args[0], tyctx, body, method_instance, type_cache);
    let b = handle_operand(&args[0], tyctx, body, method_instance, type_cache);
    place_set(
        destination,
        tyctx,
        match ty.kind() {
            TyKind::Uint(uint) => match uint {
                UintTy::U8 => operand,
                UintTy::U16 => [
                    operand,
                    vec![
                        CILOp::NewTMPLocal(Type::U16.into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shr,
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shl,
                        CILOp::Or,
                        CILOp::FreeTMPLocal,
                    ],
                ]
                .iter()
                .flatten()
                .cloned()
                .collect(),
                UintTy::U32 => [
                    operand,
                    vec![
                        //CILOp::ConvU32(false),
                        CILOp::NewTMPLocal(Type::U32.into()),
                        CILOp::SetTMPLocal,
                        // 1 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(24),
                        CILOp::Shl,
                        // 4 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(24),
                        CILOp::ShrUn,
                        CILOp::ConvU32(false),
                        CILOp::Or,
                        // 2 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shl,
                        CILOp::LdcI32(0xFF<<16),
                        CILOp::And,
                        // 3 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shr,
                        CILOp::LdcI32(0xFF<<8),
                        CILOp::And,

                        CILOp::Or,
                        CILOp::Or,
                        //CILOp::ConvU16(false),
                        CILOp::FreeTMPLocal,
                    ],
                ]
                .iter()
                .flatten()
                .cloned()
                .collect(),
                _ => todo!("Can't bswap unsigned int {ty:?}"),
            },
            _ => todo!("Can't bswap {ty:?}"),
        },
        body,
        method_instance,
        type_cache,
    )
}*/
