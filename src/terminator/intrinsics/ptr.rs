use crate::assembly::MethodCompileCtx;
use cilly::{
    cil_node::CILNode, cil_root::CILRoot, conv_isize, conv_usize, Int, IntoAsmIndex, Type,
};
use rustc_codegen_clr_place::place_set;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_codgen_clr_operand::handle_operand;
use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn arith_offset<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("arith_offset works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);

    place_set(
        destination,
        handle_operand(&args[0].node, ctx)
            + handle_operand(&args[1].node, ctx)
                * conv_isize!(CILNode::V2(ctx.size_of(tpe).into_idx(ctx))),
        ctx,
    )
}
pub fn ptr_offset_from_unsigned<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        2,
        "The intrinsic `ptr_offset_from_unsigned` MUST take in exactly 2 arguments!"
    );
    let ty = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("ptr_offset_from_unsigned works only on types!"),
    );
    let tpe = ctx.type_from_cache(ty);
    // This is UB, so we can do whatever.
    if ctx.layout_of(ty).is_zst() {
        return CILRoot::throw(
            &format!("ptr_offset_from_unsigned called with zst type:{ty}"),
            ctx,
        );
    }
    place_set(
        destination,
        CILNode::DivUn(
            (handle_operand(&args[0].node, ctx) - handle_operand(&args[1].node, ctx))
                .cast_ptr(Type::Int(Int::USize))
                .into(),
            conv_usize!(CILNode::V2(ctx.size_of(tpe).into_idx(ctx))).into(),
        ),
        ctx,
    )
}
pub fn ptr_offset_from<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        2,
        "The intrinsic `ptr_offset_from` MUST take in exactly 1 argument!"
    );
    let ty = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    // This is UB, so we can do whatever.
    if ctx.layout_of(ty).is_zst() {
        return CILRoot::throw(&format!("ptr_offset_from called with zst type:{ty}"), ctx);
    }
    let tpe = ctx.type_from_cache(ty);

    place_set(
        destination,
        CILNode::Div(
            (handle_operand(&args[0].node, ctx) - handle_operand(&args[1].node, ctx))
                .cast_ptr(Type::Int(Int::ISize))
                .into(),
            conv_isize!(CILNode::V2(ctx.size_of(tpe).into_idx(ctx))).into(),
        ),
        ctx,
    )
}
