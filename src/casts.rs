use cilly::cil_node::CILNode;
use cilly::v2::{Assembly, ClassRef, Float, FnSig, Int};
use cilly::{call_site::CallSite, Type};

use cilly::{
    call, conv_f32, conv_f64, conv_f_un, conv_i16, conv_i32, conv_i64, conv_i8, conv_isize,
    conv_u16, conv_u32, conv_u64, conv_u8, conv_usize,
};
/// Casts from intiger type `src` to target `target`
pub fn int_to_int(src: Type, target: Type, operand: CILNode, asm: &mut Assembly) -> CILNode {
    if src == target {
        return operand;
    }
    match (&src, &target) {
        // Unsinged casts are special
        (
            Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize),
            Type::Int(Int::ISize),
        ) => {
            conv_isize!(conv_usize!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I64)) => {
            conv_i64!(conv_u64!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I32)) => {
            conv_i32!(conv_u32!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I16)) => {
            conv_i16!(conv_u16!(operand))
        }
        (Type::Int(Int::U32 | Int::U16 | Int::U8 | Int::U64 | Int::USize), Type::Int(Int::I8)) => {
            conv_i8!(conv_u8!(operand))
        }
        //
        (Type::Int(Int::ISize), Type::Int(Int::I128)) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Implicit".into(),
                FnSig::new(Box::new([Type::Int(Int::ISize)]), Type::Int(Int::I128)),
                true,
            ),
            [operand]
        ),
        (Type::Int(Int::U32), Type::Int(Int::I128)) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Implicit".into(),
                FnSig::new(Box::new([Type::Int(Int::U32)]), Type::Int(Int::I128)),
                true,
            ),
            [operand]
        ),
        (Type::Int(Int::ISize), Type::Int(Int::U128)) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([Type::Int(Int::I64)]), Type::Int(Int::U128)),
                true,
            ),
            [conv_i64!(operand)]
        ),
        (Type::Bool, Type::Int(Int::U128)) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([Type::Int(Int::I32)]), Type::Int(Int::U128)),
                true,
            ),
            [conv_i32!(operand)]
        ),
        (Type::Bool, Type::Int(Int::I128)) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Implicit".into(),
                FnSig::new(Box::new([Type::Int(Int::I32)]), Type::Int(Int::I128)),
                true,
            ),
            [conv_i32!(operand)]
        ),
        // Fixes sign casts
        (Type::Int(Int::I64 | Int::I32 | Int::I16 | Int::I8), Type::Int(Int::USize)) => {
            CILNode::SignExtendToUSize(operand.into())
        }
        (Type::Int(Int::I64 | Int::I32 | Int::I16 | Int::I8), Type::Int(Int::U64)) => {
            CILNode::SignExtendToU64(operand.into())
        }
        // i128 bit casts
        (Type::Int(Int::U128), Type::Int(Int::I128))
        | (Type::Int(Int::I8 | Int::I16 | Int::I32 | Int::I64), Type::Int(Int::U128)) => {
            call!(
                CallSite::new_extern(
                    ClassRef::uint_128(asm),
                    "op_Explicit".into(),
                    FnSig::new(Box::new([src]), target),
                    true,
                ),
                [operand]
            )
        }
        (_, Type::Int(Int::I128)) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Implicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        (Type::Int(Int::I128), Type::Int(Int::U128)) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        (_, Type::Int(Int::U128)) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "op_Implicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        (Type::Int(Int::I128), _) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        (Type::Int(Int::U128), _) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        //todo!("Casting to 128 bit intiegers is not supported!"),
        _ => to_int(target, operand),
    }
}
/// Returns CIL ops required to convert type src to target
pub fn float_to_int(src: Type, target: Type, operand: CILNode, asm: &mut Assembly) -> CILNode {
    match target {
        Type::Int(Int::I128) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        Type::Int(Int::U128) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [operand]
        ),
        Type::Int(Int::U8) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_u8".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::U8)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_u8".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::U8)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::U16) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_u16".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::U16)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_u16".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::U16)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::U32) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_u32".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::U32)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_u32".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::U32)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::U64) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_u64".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::U64)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_u64".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::U64)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::USize) => match src {
            //TODO: Check why this caused
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_usize".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::USize)),
                    true
                ),
                [operand]
            ),

            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_usize".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::USize)),
                    true
                ),
                [operand]
            ),
            //_=> to_int(target, operand),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::ISize) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_isize".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::ISize)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_isize".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::ISize)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I8) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_i8".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::I8)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_i8".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::I8)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I16) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_i16".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::I16)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_i16".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::I16)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I32) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_i32".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::I32)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_i32".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::I32)),
                    true
                ),
                [operand]
            ),
            _ => panic!("Non-float type!"),
        },
        Type::Int(Int::I64) => match src {
            Type::Float(Float::F32) => call!(
                CallSite::builtin(
                    "cast_f32_i64".into(),
                    FnSig::new(Box::new([Type::Float(Float::F32)]), Type::Int(Int::I64)),
                    true
                ),
                [operand]
            ),
            Type::Float(Float::F64) => call!(
                CallSite::builtin(
                    "cast_f64_i64".into(),
                    FnSig::new(Box::new([Type::Float(Float::F64)]), Type::Int(Int::I64)),
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
        Type::Int(Int::I8) => conv_i8!(operand),
        Type::Int(Int::U8) => conv_u8!(operand),
        Type::Int(Int::I16) => conv_i16!(operand),
        Type::Int(Int::U16) => conv_u16!(operand),
        Type::Int(Int::U32) => conv_u32!(operand),
        Type::Int(Int::I32) => conv_i32!(operand),
        Type::Int(Int::I64) => conv_i64!(operand),
        Type::Int(Int::U64) => conv_u64!(operand),
        Type::Int(Int::ISize) => conv_isize!(operand),
        Type::Int(Int::USize) => conv_usize!(operand),
        Type::Ptr(tpe) => conv_usize!(operand).cast_ptr(Type::Ptr(tpe)),
        _ => todo!("Can't cast to {target:?} yet!"),
    }
}
/// Returns CIL ops required to casts from intiger type `src` to `target` MOVE TO CILLY
pub fn int_to_float(src: Type, target: Type, parrent: CILNode, asm: &mut Assembly) -> CILNode {
    if matches!(src, Type::Int(Int::I128)) {
        call!(
            CallSite::boxed(
                ClassRef::int_128(asm).into(),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [parrent]
        )
        //todo!("Casting from 128 bit intiegers is not supported!")
    } else if matches!(src, Type::Int(Int::U128)) {
        call!(
            CallSite::boxed(
                ClassRef::uint_128(asm).into(),
                "op_Explicit".into(),
                FnSig::new(Box::new([src]), target),
                true,
            ),
            [parrent]
        )
    } else if matches!(target, Type::Int(Int::I128 | Int::U128)) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        match (&src, &target) {
            (Type::Int(Int::U32 | Int::U64), Type::Float(Float::F32)) => {
                conv_f32!(conv_f_un!(parrent))
            }
            (Type::Int(Int::USize), Type::Float(Float::F32)) => {
                conv_f32!(conv_f_un!(conv_u64!(parrent)))
            }
            (_, Type::Float(Float::F32)) => conv_f32!(parrent),
            (Type::Int(Int::U32 | Int::U64), Type::Float(Float::F64)) => {
                conv_f64!(conv_f_un!(parrent))
            }
            (Type::Int(Int::USize), Type::Float(Float::F64)) => {
                conv_f64!(conv_f_un!(conv_u64!(parrent)))
            }
            (_, Type::Float(Float::F64)) => conv_f64!(parrent),
            _ => todo!("Can't  cast {src:?} to {target:?} yet!"),
        }
    }
}
