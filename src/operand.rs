use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
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
        Operand::Copy(place) | Operand::Move(place) => {
            crate::place::place_get(place, tyctx, method, method_instance, tycache)
        }
        Operand::Constant(const_val) => {
            crate::constant::handle_constant(const_val, tyctx, method, method_instance, tycache)
        }
    }
}
pub(crate) fn operand_address<'ctx>(
    operand: &Operand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    body: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut crate::r#type::TyCache,
) -> CILNode {
    match operand {
        Operand::Copy(place) | Operand::Move(place) => {
            crate::place::place_adress(place, tyctx, body, method_instance, tycache)
        }
        Operand::Constant(const_val) => {
            let local_type =
                tycache.type_from_cache(operand.ty(body, tyctx), tyctx, method_instance);
            let constant =
                crate::constant::handle_constant(const_val, tyctx, body, method_instance, tycache);
            crate::place::deref_op(
                crate::place::PlaceTy::Ty(operand.ty(body, tyctx)),
                tyctx,
                &method_instance,
                tycache,
                CILNode::TemporaryLocal(Box::new((
                    local_type,
                    [CILRoot::SetTMPLocal { value: constant }].into(),
                    CILNode::LoadAddresOfTMPLocal,
                ))),
            )
        }
    }
}
