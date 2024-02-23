use crate::{
    call, cil::{CILOp, CallSite}, cil_tree::cil_node::CILNode, conv_f32, conv_f64, conv_f64_un, conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8, conv_usize, function_sig::FnSig, ldc_i32, ldc_i64, ldc_u32, ldc_u64, r#type::{DotnetTypeRef, Type}
};
/// Casts from intiger type `src` to target `target`
pub fn int_to_int(src: Type, target: Type, operand: CILNode) -> CILNode {
    if src == target {
        return operand;
    }
    match (&src, &target) {
        /*
        (Type::DotnetType(tpe), Type::I128) | (Type::I128,Type::DotnetType(tpe))=>{
            if Some("System.Runtime".into()) == tpe.asm() && tpe.name_path().contains("System.Int128"){
                return vec![];
            }
            panic!();
        }*/
        (Type::ISize, Type::I128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                FnSig::new(&[Type::I64], &Type::I128),
                true,
            ),
            [conv_i64!(operand)]
        ),
        (Type::ISize, Type::U128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[Type::I64], &Type::U128),
                true,
            ),
            [conv_i64!(operand)]
        ),
        (Type::Bool, Type::U128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[Type::I64], &Type::U128),
                true,
            ),
            [conv_i8!(operand)]
        ),
        (Type::Bool, Type::I128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                FnSig::new(&[Type::I8], &Type::I128),
                true,
            ),
            [conv_i8!(operand)]
        ),

        (Type::U128, Type::I128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (_, Type::I128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (Type::I128, Type::U128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),

        (Type::I8 | Type::I16 | Type::I32 | Type::I64, Type::U128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (_, Type::U128) => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (Type::I128, _) => call!(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (Type::U128, _) => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        //todo!("Casting to 128 bit intiegers is not supported!"),
        _ => to_int(target, operand),
    }
}
/// Returns CIL ops required to convert type src to target
pub fn float_to_int(src: Type, target: Type, operand: CILNode) -> CILNode {
    match target {
        Type::I128 => call!(
            CallSite::new(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        Type::U128 => call!(
            CallSite::new(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        //todo!("Casting to 128 bit intiegers is not supported!"),
        Type::U16 => conv_u16!(call!(
            CallSite::boxed(
                Some(DotnetTypeRef::math()),
                "Min".into(),
                FnSig::new(&[Type::U64, Type::U64], &Type::U64),
                true,
            ),
            [conv_u64!(operand), ldc_u64!(u16::MAX as u64)]
        )),

        Type::U8 => conv_u8!(call!(
            CallSite::boxed(
                Some(DotnetTypeRef::math()),
                "Min".into(),
                FnSig::new(&[Type::U64, Type::U64], &Type::U64),
                true,
            ),
            [ldc_u64!(u8::MAX as u64), conv_u64!(operand)]
        )),
        Type::I16 => conv_i16!(call!(
            CallSite::boxed(
                Some(DotnetTypeRef::math()),
                "Max".into(),
                FnSig::new(&[Type::I64, Type::I64], &Type::I64),
                true,
            ),
            [
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::math()),
                        "Min".into(),
                        FnSig::new(&[Type::I64, Type::I64], &Type::I64),
                        true,
                    ),
                    [conv_i64!(operand), ldc_i64!(i16::MAX as i64)]
                ),
                ldc_i64!(i16::MIN as i64)
            ]
        )),

        Type::I8 => conv_i8!(call!(
            CallSite::boxed(
                Some(DotnetTypeRef::math()),
                "Max".into(),
                FnSig::new(&[Type::I64, Type::I64], &Type::I64),
                true,
            ),
            [
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::math()),
                        "Min".into(),
                        FnSig::new(&[Type::I64, Type::I64], &Type::I64),
                        true,
                    ),
                    [conv_i64!(operand), ldc_i64!(i8::MAX as i64)]
                ),
                ldc_i64!(i8::MIN as i64)
            ]
        )),
        _ => to_int(target, operand),
    }

    //call uint64 [System.Runtime]System.Int128::op_Explicit(valuetype [System.Runtime]System.Int128)
    //
}
/// Returns CIL ops required to convert to intiger of type `target`
fn to_int(target: Type, operand: CILNode) -> CILNode {
    match target {
        Type::I8 => conv_i8!(operand),
        Type::U8 => conv_u8!(operand),
        Type::I16 => conv_i16!(operand),
        Type::U16 => conv_u16!(operand),
        Type::U32 => conv_u32!(operand),
        Type::I32 => conv_i32!(operand),
        Type::I64 => conv_i64!(operand),
        Type::U64 => conv_u64!(operand),
        Type::ISize => conv_isize!(operand),
        Type::USize => conv_usize!(operand),
        Type::Ptr(_) => conv_usize!(operand),
        _ => todo!("Can't cast to {target:?} yet!"),
    }
}
/// Returns CIL ops required to casts from intiger type `src` to `target`
pub fn int_to_float(src: Type, target: Type,parrent:CILNode) -> CILNode {
    if matches!(src, Type::I128) {
        call!(CallSite::boxed(
            DotnetTypeRef::int_128().into(),
            "op_Explicit".into(),
            FnSig::new(&[src], &target),
            true,
        ),[parrent])
        //todo!("Casting from 128 bit intiegers is not supported!")
    } else if matches!(src, Type::U128) {
        call!(CallSite::boxed(
            DotnetTypeRef::uint_128().into(),
            "op_Explicit".into(),
            FnSig::new(&[src], &target),
            true,
        ),[parrent])
       
    } else if matches!(target, Type::I128 | Type::U128) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        match (&src, &target) {
            (Type::U32 | Type::U64, Type::F32) => conv_f32!(conv_f64_un!(parrent)),
            (_, Type::F32) => conv_f32!(parrent),
            (Type::U32 | Type::U64, Type::F64) => conv_f64_un!(parrent),
            (_, Type::F64) => conv_f64!(parrent),
            _ => todo!("Can't  cast {src:?} to {target:?} yet!"),
        }
    }
}
