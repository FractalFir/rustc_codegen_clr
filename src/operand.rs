use crate::cil_op::CILOp;

use rustc_middle::mir::Operand;
use rustc_middle::ty::{Instance, TyCtxt};
pub(crate) fn handle_operand<'ctx>(
    operand: &Operand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    match operand {
        Operand::Copy(place) => crate::place::place_get(place, tyctx, method, method_instance),
        Operand::Move(place) => crate::place::place_get(place, tyctx, method, method_instance),
        Operand::Constant(const_val) => {
            crate::constant::handle_constant(&const_val.const_, tyctx, method, method_instance)
        }
    }
}
