use rustc_middle::mir::{Operand, UnOp};
use rustc_middle::ty::{Instance, TyCtxt};
use crate::r#type::tycache::TyCache;

use crate::cil_op::CILOp;
pub fn unop<'ctx>(
    unnop: UnOp,
    operand: &Operand<'ctx>,
    tcx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache:&mut TyCache,
) -> Vec<CILOp> {
    let mut ops = crate::operand::handle_operand(operand, tcx, method, method_instance,tycache);
    let _ty = operand.ty(&method.local_decls, tcx);
    match unnop {
        UnOp::Neg => ops.push(CILOp::Neg),
        UnOp::Not => ops.push(CILOp::Not),
    };
    ops
}
