use rustc_middle::mir::{Operand, UnOp};
use rustc_middle::ty::{IntTy, Ty, TyCtxt, TyKind, UintTy,Instance};

use crate::cil_op::CILOp;
pub fn unop<'ctx>(
    unnop: UnOp,
    operand: &Operand<'ctx>,
    tcx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    let ops = crate::operand::handle_operand(operand, tcx, method,method_instance);
    let ty = operand.ty(&method.local_decls, tcx);
    match unnop {
        UnOp::Neg => vec![CILOp::Neg],
        UnOp::Not => vec![CILOp::Not],
    }
}
