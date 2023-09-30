use crate::{cil_op::CILOp, r#type::Type};

pub fn int_to_int(src: Type, target: Type) -> Vec<CILOp> {
    if matches!(src, Type::I128 | Type::U128) {
        todo!("Casting from 128 bit intiegers is not supported!")
    }
    if matches!(target, Type::I128 | Type::U128) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        to_int(target)
    }
}
pub fn float_to_int(_src: Type, target: Type) -> Vec<CILOp> {
    if matches!(target, Type::I128 | Type::U128) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        to_int(target)
    }
}
pub fn to_int(target: Type) -> Vec<CILOp> {
    match target {
        Type::I8 => vec![CILOp::ConvI8(false)],
        Type::U8 => vec![CILOp::ConvU8(false)],
        Type::I16 => vec![CILOp::ConvI16(false)],
        Type::I32 => vec![CILOp::ConvI32(false)],
        Type::I64 => vec![CILOp::ConvI64(false)],
        Type::ISize => vec![CILOp::ConvISize(false)],
        Type::USize => vec![CILOp::ConvUSize(false)],
        _ => todo!("Can't cast to {target:?} yet!"),
    }
}
pub fn int_to_float(src: Type, target: Type) -> Vec<CILOp> {
    if matches!(src, Type::I128 | Type::U128) {
        todo!("Casting from 128 bit intiegers is not supported!")
    }
    if matches!(target, Type::I128 | Type::U128) {
        todo!("Casting to 128 bit intiegers is not supported!")
    } else {
        match target {
            Type::F32 => vec![CILOp::ConvF32(false)],
            _ => todo!("Can't cast to {target:?} yet!"),
        }
    }
}
