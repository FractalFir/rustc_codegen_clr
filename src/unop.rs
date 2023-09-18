use rustc_middle::mir::{UnOp, Operand};
use rustc_middle::ty::{IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil_op::CILOp;
pub fn unop<'ctx>(
    unnop: UnOp,
    operand: &Operand<'ctx>,
    tcx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
) -> Vec<CILOp> {
    let ops = crate::operand::handle_operand(operand, tcx, method);
    let ty = operand.ty(&method.local_decls, tcx);
    match unnop {
        UnOp::Neg=>vec![CILOp::Neg],
        UnOp::Not=>vec![CILOp::Not],
    }
}
