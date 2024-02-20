use crate::cil::CILOp;
use crate::cil_tree::cil_node::CILNode;

use rustc_middle::mir::Operand;
use rustc_middle::ty::{Instance, TyCtxt};
pub(crate) fn handle_operand<'ctx>(
    operand: &Operand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut crate::r#type::TyCache,
) -> CILNode {
    match operand {
        Operand::Copy(place) | Operand::Move(place)=> {
            crate::place::place_get(place, tyctx, method, method_instance, tycache)
        }
        Operand::Constant(const_val) => {
            crate::constant::handle_constant(const_val, tyctx, method, method_instance, tycache)
        }
    }
}
