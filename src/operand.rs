use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use cilly::Type;
use rustc_middle::{
    mir::{ConstValue, Operand},
    ty::ParamEnv,
};

use crate::assembly::MethodCompileCtx;
pub(crate) fn handle_operand<'tcx>(
    operand: &Operand<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let res = ctx.type_from_cache(ctx.monomorphize(operand.ty(ctx.body(), ctx.tcx())));
    if res == Type::Void {
        return CILNode::TemporaryLocal(Box::new((Type::Void, [].into(), CILNode::LoadTMPLocal)));
    }
    match operand {
        Operand::Copy(place) | Operand::Move(place) => crate::place::place_get(place, ctx),
        Operand::Constant(const_val) => crate::constant::handle_constant(const_val, ctx),
    }
}
pub(crate) fn operand_address<'tcx>(
    operand: &Operand<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    match operand {
        Operand::Copy(place) | Operand::Move(place) => crate::place::place_adress(place, ctx),
        Operand::Constant(const_val) => {
            let local_type = ctx.type_from_cache(operand.ty(ctx.body(), ctx.tcx()));
            let constant = crate::constant::handle_constant(const_val, ctx);
            crate::place::deref_op(
                crate::place::PlaceTy::Ty(operand.ty(ctx.body(), ctx.tcx())),
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
/// Checks if this operand is uninitialzed, and assigements using it can safely be skipped.
pub(crate) fn is_uninit<'tcx>(
    operand: &Operand<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> bool {
    match operand {
        Operand::Copy(_) | Operand::Move(_) => false,
        Operand::Constant(const_val) => {
            let constant = const_val.const_;
            let constant = ctx.monomorphize(constant);
            let evaluated = constant
                .eval(ctx.tcx(), ParamEnv::reveal_all(), const_val.span)
                .expect("Could not evaluate constant!");
            match evaluated {
                ConstValue::Scalar(_) => false, // Scalars are never uninitialized.
                ConstValue::ZeroSized => {
                    // ZeroSized has no data, so I guess it has no initialized data, so assiments using it could propably be safely skipped.
                    true
                }
                ConstValue::Slice { data, .. } => {
                    let mask = data.inner().init_mask();
                    let mut chunks =
                        mask.range_as_init_chunks(rustc_const_eval::interpret::AllocRange {
                            start: rustc_abi::Size::ZERO,
                            size: data.0.size(),
                        });
                    let Some(only) = chunks.next() else {
                        return false;
                    };
                    // If this is not the only chunk, then the init mask must not be fully uninitialized
                    if chunks.next().is_some() {
                        return false;
                    }
                    !only.is_init()
                }
                ConstValue::Indirect { alloc_id, .. } => {
                    let data = ctx.tcx().global_alloc(alloc_id);
                    let rustc_middle::mir::interpret::GlobalAlloc::Memory(data) = data else {
                        return false;
                    };
                    let mask = data.0.init_mask();
                    let mut chunks =
                        mask.range_as_init_chunks(rustc_const_eval::interpret::AllocRange {
                            start: rustc_abi::Size::ZERO,
                            size: data.0.size(),
                        });
                    let Some(only) = chunks.next() else {
                        return false;
                    };
                    // If this is not the only chunk, then the init mask must not be fully uninitialized
                    if chunks.next().is_some() {
                        return false;
                    }
                    !only.is_init()
                }
            }
        }
    }
}
