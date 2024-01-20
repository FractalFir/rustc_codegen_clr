use crate::r#type::tycache::TyCache;
use rustc_middle::mir::{Operand, UnOp};
use rustc_middle::ty::{Instance, TyCtxt, TyKind};

use crate::cil::CILOp;
pub fn unop<'ctx>(
    unnop: UnOp,
    operand: &Operand<'ctx>,
    tcx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    let mut ops = crate::operand::handle_operand(operand, tcx, method, method_instance, tycache);
    let ty = operand.ty(&method.local_decls, tcx);
    match unnop {
        UnOp::Neg => ops.push(CILOp::Neg),
        UnOp::Not => match ty.kind() {
            TyKind::Bool => ops.extend([CILOp::LdcI32(0), CILOp::Eq]),
            _ => ops.push(CILOp::Not),
        },
    };
    ops
}
