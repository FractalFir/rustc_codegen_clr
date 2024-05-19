use crate::{
    call,
    cil::CallSite,
    cil_tree::cil_node::CILNode,
    conv_f32, conv_f64, conv_f_un, conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u16,
    conv_u32, conv_u64, conv_u8, conv_usize,
    function_sig::FnSig,
    r#type::{DotnetTypeRef, Type},
};
/// Casts from intiger type `src` to target `target`
pub fn int_to_int(src: Type, target: Type, operand: CILNode) -> CILNode {
    if src == target {
        return operand;
    }
    match (&src, &target) {
        // Unsinged casts are special
        (Type::U32 | Type::U16 | Type::U8 | Type::U64 | Type::USize, Type::ISize) => {
            conv_isize!(conv_usize!(operand))
        }
        (Type::U32 | Type::U16 | Type::U8 | Type::U64 | Type::USize, Type::I64) => {
            conv_i64!(conv_u64!(operand))
        }
        (Type::U32 | Type::U16 | Type::U8 | Type::U64 | Type::USize, Type::I32) => {
            conv_i32!(conv_u32!(operand))
        }
        (Type::U32 | Type::U16 | Type::U8 | Type::U64 | Type::USize, Type::I16) => {
            conv_i16!(conv_u16!(operand))
        }
        (Type::U32 | Type::U16 | Type::U8 | Type::U64 | Type::USize, Type::I8) => {
            conv_i8!(conv_u8!(operand))
        }
        //
        (Type::ISize, Type::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Implicit".into(),
                FnSig::new(&[Type::I64], &Type::I128),
                true,
            ),
            [conv_i64!(operand)]
        ),
        (Type::ISize, Type::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Explicit".into(),
                FnSig::new(&[Type::I64], &Type::U128),
                true,
            ),
            [conv_i64!(operand)]
        ),
        (Type::Bool, Type::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Explicit".into(),
                FnSig::new(&[Type::I64], &Type::U128),
                true,
            ),
            [conv_i8!(operand)]
        ),
        (Type::Bool, Type::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Implicit".into(),
                FnSig::new(&[Type::I8], &Type::I128),
                true,
            ),
            [conv_i8!(operand)]
        ),
        // Fixes sign casts
        (Type::I64 | Type::I32 | Type::I16 | Type::I8, Type::USize) => conv_isize!(operand),
        (Type::I64 |Type::I32 | Type::I16 | Type::I8, Type::U64) => conv_i64!(operand),
        // i128 bit casts
        (Type::U128, Type::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (_, Type::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Implicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (Type::I128, Type::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),

        (Type::I8 | Type::I16 | Type::I32 | Type::I64, Type::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (_, Type::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Implicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (Type::I128, _) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        (Type::U128, _) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
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
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        Type::U128 => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [operand]
        ),
        Type::U8 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_u8".into(),
                    FnSig::new(&[Type::F32], &Type::U8),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_u8".into(),
                    FnSig::new(&[Type::F64], &Type::U8),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::U16 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_u16".into(),
                    FnSig::new(&[Type::F32], &Type::U16),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_u16".into(),
                    FnSig::new(&[Type::F64], &Type::U16),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::U32 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_u32".into(),
                    FnSig::new(&[Type::F32], &Type::U32),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_u32".into(),
                    FnSig::new(&[Type::F64], &Type::U32),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::U64 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_u64".into(),
                    FnSig::new(&[Type::F32], &Type::U64),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_u64".into(),
                    FnSig::new(&[Type::F64], &Type::U64),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::USize => match src {
            //TODO: Check why this caused
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_usize".into(),
                    FnSig::new(&[Type::F32], &Type::USize),
                    true
                ),
                [operand]
            ),

            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_usize".into(),
                    FnSig::new(&[Type::F64], &Type::USize),
                    true
                ),
                [operand]
            ),
            //_=> to_int(target, operand),
            _ => panic!("Non-float type!"),
        },
        Type::ISize => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_isize".into(),
                    FnSig::new(&[Type::F32], &Type::ISize),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_isize".into(),
                    FnSig::new(&[Type::F64], &Type::ISize),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::I8 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_i8".into(),
                    FnSig::new(&[Type::F32], &Type::I8),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_i8".into(),
                    FnSig::new(&[Type::F64], &Type::I8),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::I16 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_i16".into(),
                    FnSig::new(&[Type::F32], &Type::I16),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_i16".into(),
                    FnSig::new(&[Type::F64], &Type::I16),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::I32 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_i32".into(),
                    FnSig::new(&[Type::F32], &Type::I32),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_i32".into(),
                    FnSig::new(&[Type::F64], &Type::I32),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::I64 => match src {
            Type::F32 => call!(
                CallSite::builtin(
                    "cast_f32_i64".into(),
                    FnSig::new(&[Type::F32], &Type::I64),
                    true
                ),
                [operand]
            ),
            Type::F64 => call!(
                CallSite::builtin(
                    "cast_f64_i64".into(),
                    FnSig::new(&[Type::F64], &Type::I64),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
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
pub fn int_to_float(src: Type, target: Type, parrent: CILNode) -> CILNode {
    if matches!(src, Type::I128) {
        call!(
            CallSite::boxed(
                DotnetTypeRef::int_128().into(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [parrent]
        )
        //todo!("Casting from 128 bit intiegers is not supported!")
    } else if matches!(src, Type::U128) {
        call!(
            CallSite::boxed(
                DotnetTypeRef::uint_128().into(),
                "op_Explicit".into(),
                FnSig::new(&[src], &target),
                true,
            ),
            [parrent]
        )
    } else if matches!(target, Type::I128 | Type::U128) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        match (&src, &target) {
            (Type::U32 | Type::U64, Type::F32) => conv_f32!(conv_f_un!(parrent)),
            (Type::USize, Type::F32) => conv_f32!(conv_f_un!(conv_u64!(parrent))),
            (_, Type::F32) => conv_f32!(parrent),
            (Type::U32 | Type::U64, Type::F64) => conv_f64!(conv_f_un!(parrent)),
            (Type::USize, Type::F64) => conv_f64!(conv_f_un!(conv_u64!(parrent))),
            (_, Type::F64) => conv_f64!(parrent),
            _ => todo!("Can't  cast {src:?} to {target:?} yet!"),
        }
    }
}
