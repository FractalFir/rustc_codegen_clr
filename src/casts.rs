use crate::{
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    r#type::{DotnetTypeRef, Type},
};
/// Casts from intiger type `src` to target `target`
pub fn int_to_int(src: Type, target: Type) -> Vec<CILOp> {
    if src == target {
        return vec![];
    }
    match (&src, &target) {
        (Type::ISize,Type::I128)=>
            vec![CILOp::ConvI64(false),CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Implicit".into(),
                    FnSig::new(&[Type::I64], &Type::I128),
                    true,
                )
                .into(),
            )]
        ,
        (Type::ISize,Type::U128)=>
            vec![CILOp::ConvI64(false),CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Explicit".into(),
                    FnSig::new(&[Type::I64], &Type::U128),
                    true,
                )
                .into(),
            )]
        ,
        (Type::Bool,Type::U128)=>
            vec![CILOp::ConvI8(false),CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Explicit".into(),
                    FnSig::new(&[Type::I8], &Type::U128),
                    true,
                )
                .into(),
            )]
        ,
        (_, Type::I128) => {
            vec![CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Implicit".into(),
                    FnSig::new(&[src], &target),
                    true,
                )
                .into(),
            )]
        }
        (Type::I128, Type::U128) => vec![CILOp::Call(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            )
            .into(),
        )],
        (Type::I8 | Type::I16 | Type::I32 | Type::I64, Type::U128) => vec![CILOp::Call(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            )
            .into(),
        )],
        (_, Type::U128) => vec![CILOp::Call(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                FnSig::new(&[src], &target),
                true,
            )
            .into(),
        )],
        (Type::I128, _) => {
            vec![CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Explicit".into(),
                    FnSig::new(&[src], &target),
                    true,
                )
                .into(),
            )]
        }
        (Type::U128, _) => {
            vec![CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Explicit".into(),
                    FnSig::new(&[src], &target),
                    true,
                )
                .into(),
            )]
        }
        //todo!("Casting to 128 bit intiegers is not supported!"),
        _ => to_int(target),
    }
}
/// Returns CIL ops required to convert type src to target
pub fn float_to_int(src: Type, target: Type) -> Vec<CILOp> {
    match target {
        Type::I128 => {
            vec![CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Explicit".into(),
                    FnSig::new(&[src], &target),
                    true,
                )
                .into(),
            )]
        }
        Type::U128 => {
            vec![CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Explicit".into(),
                    FnSig::new(&[src], &target),
                    true,
                )
                .into(),
            )]
        } //todo!("Casting to 128 bit intiegers is not supported!"),
        _ => to_int(target),
    }

    //call uint64 [System.Runtime]System.Int128::op_Explicit(valuetype [System.Runtime]System.Int128)
    //
}
/// Returns CIL ops required to convert to intiger of type `target`
fn to_int(target: Type) -> Vec<CILOp> {
    match target {
        Type::I8 => vec![CILOp::ConvI8(false)],
        Type::U8 => vec![CILOp::ConvU8(false)],
        Type::I16 => vec![CILOp::ConvI16(false)],
        Type::U16 => vec![CILOp::ConvU16(false)],
        Type::U32 => vec![CILOp::ConvU32(false)],
        Type::I32 => vec![CILOp::ConvI32(false)],
        Type::I64 => vec![CILOp::ConvI64(false)],
        Type::U64 => vec![CILOp::ConvU64(false)],
        Type::ISize => vec![CILOp::ConvISize(false)],
        Type::USize => vec![CILOp::ConvUSize(false)],
        Type::Ptr(_) => vec![CILOp::ConvUSize(false)],
        _ => todo!("Can't cast to {target:?} yet!"),
    }
}
/// Returns CIL ops required to casts from intiger type `src` to `target`
pub fn int_to_float(src: Type, target: Type) -> Vec<CILOp> {
    if matches!(src, Type::I128) {
        vec![CILOp::Call(CallSite::boxed(
            DotnetTypeRef::int_128().into(),
            "op_Explicit".into(),
            FnSig::new(&[src], &target),
            true,
        ))]
        //todo!("Casting from 128 bit intiegers is not supported!")
    } else if matches!(src, Type::U128) {
        vec![CILOp::Call(CallSite::boxed(
            DotnetTypeRef::uint_128().into(),
            "op_Explicit".into(),
            FnSig::new(&[src], &target),
            true,
        ))]
    } else if matches!(target, Type::I128 | Type::U128) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        match target {
            Type::F32 => vec![CILOp::ConvF32(false)],
            Type::F64 => vec![CILOp::ConvF64(false)],
            _ => todo!("Can't cast to {target:?} yet!"),
        }
    }
}
