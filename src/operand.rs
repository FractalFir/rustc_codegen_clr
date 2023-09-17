use crate::cil_op::CILOp;
use rustc_middle::mir::Body;
use rustc_middle::mir::Operand;
use rustc_middle::ty::TyCtxt;
pub(crate) fn handle_operand<'ctx>(
    operand: &Operand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
) -> Vec<CILOp> {
    match operand {
        Operand::Copy(place) => crate::place::place_get(place, tyctx, method),
        Operand::Move(place) => crate::place::place_get(place, tyctx, method),
        Operand::Constant(const_val) => {
            crate::constant::handle_constant(const_val.as_ref(), tyctx, method)
        }
    }
}
