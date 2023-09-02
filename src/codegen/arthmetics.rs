use crate::{base_ir::BaseIR, types::Type};
use rustc_middle::mir::{BinOp, UnOp};
pub(crate) fn add_unchecked<'ctx>(a: Type, b: Type) -> BaseIR {
    match (a, b) {
        (Type::I128 | Type::U128, _) => todo!("Can't add 128 bit numbers yet!"),
        (_, Type::I128 | Type::U128) => todo!("Can't add 128 bit numbers yet!"),
        _ => BaseIR::Add,
    }
}
pub(crate) fn binop_unchecked<'ctx>(
    binop: BinOp,
    a: (Vec<BaseIR>, Type),
    b: (Vec<BaseIR>, Type),
) -> Vec<BaseIR> {
    let mut ops = Vec::new();
    ops.extend(a.0);
    ops.extend(b.0);
    match binop {
        BinOp::Add | BinOp::AddUnchecked => ops.push(add_unchecked(a.1, b.1)),
        BinOp::Sub | BinOp::SubUnchecked => ops.push(BaseIR::Sub),
        BinOp::Mul | BinOp::MulUnchecked => ops.push(BaseIR::Mul),
        BinOp::Shl | BinOp::ShlUnchecked => ops.push(BaseIR::Shl),
        BinOp::Shr | BinOp::ShrUnchecked => ops.push(BaseIR::Shr),
        BinOp::Eq => ops.push(BaseIR::Eq),
        BinOp::Ne => ops.extend([BaseIR::Eq, BaseIR::LDConstI32(0), BaseIR::Eq]),
        BinOp::Gt => ops.push(BaseIR::Gt),
        BinOp::Lt => ops.push(BaseIR::Lt),
        BinOp::Ge => ops.push(BaseIR::Ge),
        BinOp::Le => ops.push(BaseIR::Le),
        BinOp::Rem => ops.push(BaseIR::Rem),
        BinOp::BitXor => ops.push(BaseIR::Xor),
        BinOp::BitOr => ops.push(BaseIR::Or),
        BinOp::BitAnd => ops.push(BaseIR::And),
        BinOp::Div => ops.push(BaseIR::Div),
        BinOp::Offset => todo!("Can't yet handle the pointer offset operator!"),
    };
    ops
}
pub(crate) fn unop_unchecked<'ctx>(unop: UnOp, operand: (Vec<BaseIR>, Type)) -> Vec<BaseIR> {
    match unop {
        UnOp::Not => {
            let mut ops = operand.0;
            ops.push(BaseIR::Not);
            ops
        }
        UnOp::Neg => match operand.1 {
            Type::I8
            | Type::U8
            | Type::I16
            | Type::U16
            | Type::I32
            | Type::U32
            | Type::I64
            | Type::U64
            | Type::ISize
            | Type::USize => {
                let mut ops = vec![BaseIR::LDConstI32(0), BaseIR::Sub];
                ops.extend(operand.0);
                ops
            }
            Type::F32 | Type::F64 => {
                let mut ops = vec![BaseIR::LDConstF32(0.0), BaseIR::Sub];
                ops.extend(operand.0);
                ops
            }
            Type::I128 | Type::U128 => todo!("Can't negate 128 bit intigers"),
            _ => panic!("Negating \"{:?}\" is invalid", operand.1),
        },
    }
}
