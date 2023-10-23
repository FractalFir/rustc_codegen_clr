use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil_op::CILOp;
use crate::r#type::Type;
/// Preforms an checked binary operation.
pub(crate) fn binop_checked<'tcx>(
    binop: BinOp,
    operand_a: &Operand<'tcx>,
    operand_b: &Operand<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let ops_a = crate::operand::handle_operand(operand_a, tcx, method, method_instance);
    let ops_b = crate::operand::handle_operand(operand_b, tcx, method, method_instance);
    let ty_a = operand_a.ty(&method.local_decls, tcx);
    let ty_b = operand_b.ty(&method.local_decls, tcx);
    assert_eq!(ty_a, ty_b);
    let ty = Type::from_ty(ty_a, tcx,&method_instance);
    todo!("Checked ops should return a tuple!");
    match binop {
        BinOp::Mul | BinOp::MulUnchecked => [ops_a, ops_b, mul(ty).into()]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Add => [ops_a, ops_b, add(ty).into()]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Sub => [ops_a, ops_b, sub(ty).into()]
            .into_iter()
            .flatten()
            .collect(),
        _ => todo!("Can't preform checked op {binop:?}"),
    }
}
fn mul(tpe: Type) -> &'static [CILOp] {
    match tpe {
        Type::I16 => &[CILOp::MulOvf, CILOp::ConvI16(true)],
        Type::I32 => &[CILOp::MulOvf],
        Type::I64 => &[CILOp::MulOvf],
        _ => todo!("Can't preform checked mul on type {tpe:?} yet!"),
    }
}
fn add(tpe: Type) -> &'static [CILOp] {
    match tpe {
        Type::I8 => &[CILOp::AddOvf, CILOp::ConvI8(true)],
        Type::U8 => &[CILOp::AddOvfUn, CILOp::ConvU8(true)],
        Type::I16 => &[CILOp::AddOvf, CILOp::ConvI16(true)],
        Type::U16 => &[CILOp::AddOvfUn, CILOp::ConvU16(true)],
        Type::I32 => &[CILOp::AddOvf],
        Type::U32 => &[CILOp::AddOvfUn],
        Type::I64 => &[CILOp::AddOvf],
        Type::U64 => &[CILOp::AddOvfUn],
        Type::USize => &[CILOp::AddOvfUn],
        Type::ISize => &[CILOp::AddOvf],
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
fn sub(tpe: Type) -> &'static [CILOp] {
    match tpe {
        Type::I8 => &[CILOp::SubOvf, CILOp::ConvI8(true)],
        Type::U8 => &[CILOp::SubOvfUn, CILOp::ConvU8(true)],
        Type::I16 => &[CILOp::SubOvf, CILOp::ConvI16(true)],
        Type::U16 => &[CILOp::SubOvfUn, CILOp::ConvU16(true)],
        Type::I32 => &[CILOp::SubOvf],
        Type::U32 => &[CILOp::SubOvfUn],
        Type::I64 => &[CILOp::SubOvf],
        Type::U64 => &[CILOp::SubOvfUn],
        Type::USize => &[CILOp::SubOvfUn],
        Type::ISize => &[CILOp::SubOvf],
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
