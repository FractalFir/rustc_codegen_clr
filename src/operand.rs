use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use rustc_middle::mir::Operand;
use rustc_middle::ty::{Instance, TyCtxt};

use crate::assembly::MethodCompileCtx;
pub(crate) fn handle_operand<'ctx>(
    operand: &Operand<'ctx>,
    ctx: &mut MethodCompileCtx<'ctx, '_, '_>,
) -> CILNode {
    match operand {
        Operand::Copy(place) | Operand::Move(place) => crate::place::place_get(place, ctx),
        Operand::Constant(const_val) => crate::constant::handle_constant(const_val, ctx),
    }
}
pub(crate) fn operand_address<'ctx>(
    operand: &Operand<'ctx>,
    ctx: &mut MethodCompileCtx<'ctx, '_, '_>,
) -> CILNode {
    match operand {
        Operand::Copy(place) | Operand::Move(place) => crate::place::place_adress(place, ctx),
        Operand::Constant(const_val) => {
            let local_type = ctx.type_from_cache(operand.ty(ctx.method(), ctx.tyctx()));
            let constant = crate::constant::handle_constant(const_val, ctx);
            crate::place::deref_op(
                crate::place::PlaceTy::Ty(operand.ty(ctx.method(), ctx.tyctx())),
                ctx,
                CILNode::TemporaryLocal(Box::new((
                    local_type,
                    [CILRoot::SetTMPLocal { value: constant }].into(),
                    CILNode::LoadAddresOfTMPLocal,
                ))),
            )
        }
    }
}
