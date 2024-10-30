use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{cil_node::CILNode, cil_root::CILRoot, conv_usize, eq, Int, IntoAsmIndex, Type};
use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;

use super::utilis::compare_bytes;
/// Takes in 3 args. dst, val, and count. writes count * sizeof(T) bytes of value `val` to dst.
pub fn write_bytes<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        3,
        "The intrinsic `write_bytes` MUST take in exactly 3 argument!"
    );
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);
    let dst = handle_operand(&args[0].node, ctx);
    let val = handle_operand(&args[1].node, ctx);
    let count = handle_operand(&args[2].node, ctx)
        * conv_usize!(CILNode::V2(ctx.size_of(tpe).into_idx(ctx)));
    CILRoot::InitBlk {
        dst: Box::new(dst),
        val: Box::new(val),
        count: Box::new(count),
    }
}
/// Takes in 3 args. dst, src, and count. copies count * sizeof(T) bytes from src to dst .
pub fn copy<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        3,
        "The intrinsic `copy` MUST take in exactly 3 argument!"
    );
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    if ctx.layout_of(tpe).is_zst() {
        return CILRoot::Nop;
    }
    let tpe = ctx.type_from_cache(tpe);
    let src = handle_operand(&args[0].node, ctx);
    let dst = handle_operand(&args[1].node, ctx);
    let count = handle_operand(&args[2].node, ctx)
        * conv_usize!(CILNode::V2(ctx.size_of(tpe).into_idx(ctx)));

    CILRoot::CpBlk {
        src: Box::new(src),
        dst: Box::new(dst),
        len: Box::new(count),
    }
}
pub fn raw_eq<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    // Raw eq returns 0 if values are not equal, and 1 if they are, unlike memcmp, which does the oposite.
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("raw_eq works only on types!"),
    );
    // Raw eq always true for zsts.
    if ctx.layout_of(tpe).is_zst() {
        return place_set(destination, CILNode::V2(ctx.alloc_node(true)), ctx);
    }
    let tpe = ctx.type_from_cache(tpe);
    let size = match tpe {
        Type::Bool
        | Type::Int(
            Int::U8
            | Int::I8
            | Int::U16
            | Int::I16
            | Int::U32
            | Int::I32
            | Int::U64
            | Int::I64
            | Int::USize
            | Int::ISize,
        )
        | Type::Ptr(_) => {
            return place_set(
                destination,
                eq!(
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx)
                ),
                ctx,
            );
        }
        _ => CILNode::V2(ctx.size_of(tpe).into_idx(ctx)),
    };
    place_set(
        destination,
        eq!(
            compare_bytes(
                handle_operand(&args[0].node, ctx).cast_ptr(ctx.nptr(Type::Int(Int::U8))),
                handle_operand(&args[1].node, ctx).cast_ptr(ctx.nptr(Type::Int(Int::U8))),
                conv_usize!(size),
                ctx
            ),
            CILNode::V2(ctx.alloc_node(0_i32))
        ),
        ctx,
    )
}
