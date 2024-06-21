use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{cil_node::CILNode, cil_root::CILRoot, lt_un, Type};

use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn saturating_add<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    let a = handle_operand(&args[0].node, ctx);
    let b = handle_operand(&args[1].node, ctx);
    let a_type = ctx.type_from_cache(
        ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        ),
    );
    let calc = match a_type {
        Type::USize | Type::U64 | Type::U32 | Type::U16 | Type::U8 => {
            let sum = a.clone() + b.clone();
            let or = a | b;
            let flag = lt_un!(sum.clone(), or);
            let max = crate::r#type::max_value(&a_type);
            CILNode::select(a_type, max, sum, flag)
        }
        _ => todo!("Can't use the intrinsic `saturating_add` on {a_type:?}"),
    };
    place_set(destination, calc, ctx)
}
pub fn saturating_sub<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    let a = handle_operand(&args[0].node, ctx);
    let b = handle_operand(&args[1].node, ctx);
    let a_ty = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("saturating_sub works only on types!"),
    );
    let a_type = ctx.type_from_cache(a_ty);
    let calc = match a_type {
        Type::U128 | Type::U64 | Type::U32 | Type::U16 | Type::U8 | Type::USize => {
            let undeflow = crate::binop::cmp::lt_unchecked(a_ty, a.clone(), b.clone());
            let diff = crate::binop::sub_unchecked(a_ty, a_ty, ctx, a, b);
            let max = crate::binop::checked::zero(a_ty);
            CILNode::select(a_type, max, diff, undeflow)
        }
        _ => todo!("Can't use the intrinsic `saturating_sub` on {a_type:?}"),
    };
    place_set(destination, calc, ctx)
}
