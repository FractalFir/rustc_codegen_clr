use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil_op::CILOp;
pub(crate) fn binop_unchecked<'ctx>(
    binop: BinOp,
    operand_a: &Operand<'ctx>,
    operand_b: &Operand<'ctx>,
    tcx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
) -> Vec<CILOp> {
    let ops_a = crate::operand::handle_operand(operand_a, tcx, method);
    let ops_b = crate::operand::handle_operand(operand_b, tcx, method);
    let ty_a = operand_a.ty(&method.local_decls, tcx);
    let ty_b = operand_b.ty(&method.local_decls, tcx);
    match binop {
        BinOp::Add | BinOp::AddUnchecked => [ops_a, ops_b, add_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Ne => [ops_a, ops_b, ne_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        _ => todo!("Unsupported bionp {binop:?}"),
    }
}
fn add_unchecked<'ctx>(ty_a: Ty<'ctx>, ty_b: Ty<'ctx>) -> Vec<CILOp> {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                todo!("Can't add 128 bit intigers yet!");
            } else {
                vec![CILOp::Add]
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                todo!("Can't add 128 bit intigers yet!");
            } else {
                vec![CILOp::Add]
            }
        }
        TyKind::Float(_) => vec![CILOp::Add],
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
fn ne_unchecked<'ctx>(ty_a: Ty<'ctx>, ty_b: Ty<'ctx>) -> Vec<CILOp> {
    vec![CILOp::Eq, CILOp::LdcI32(0), CILOp::Eq]
}
