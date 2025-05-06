use crate::assembly::MethodCompileCtx;
use cilly::{cil_node::V1Node, cil_root::V1Root, conv_usize, Int, IntoAsmIndex, Type};
use rustc_codegen_clr_place::place_set;
use rustc_codgen_clr_operand::handle_operand;
use rustc_middle::mir::{Operand, Place};
use rustc_span::source_map::Spanned;
/// Gets the aligement of a dynamic object from a fat pointer, by looking it up from the vtable.
pub fn vtable_align<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Root {
    let vtableptr = handle_operand(&args[0].node, ctx);
    let align_ptr = (vtableptr
        + conv_usize!(
            (V1Node::V2(ctx.size_of(Int::ISize).into_idx(ctx)))
                * V1Node::V2(ctx.alloc_node(2_i32))
        ))
    .cast_ptr(ctx.nptr(Type::Int(Int::USize)));
    place_set(
        destination,
        V1Node::LDIndUSize {
            ptr: Box::new(align_ptr),
        },
        ctx,
    )
}
/// Gets the size of a dynamic object from a fat pointer, by looking it up from the vtable.
pub fn vtable_size<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Root {
    let vtableptr = handle_operand(&args[0].node, ctx);
    let size_ptr = (vtableptr + conv_usize!((V1Node::V2(ctx.size_of(Int::ISize).into_idx(ctx)))))
        .cast_ptr(ctx.nptr(Type::Int(Int::USize)));
    place_set(
        destination,
        V1Node::LDIndUSize {
            ptr: Box::new(size_ptr),
        },
        ctx,
    )
}
