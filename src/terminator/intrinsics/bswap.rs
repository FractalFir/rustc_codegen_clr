use crate::assembly::MethodCompileCtx;
use cilly::{
    call,
    cil_node::V1Node,
    cil_root::V1Root,
    {cilnode::MethodKind, ClassRef, MethodRef},
};
use rustc_codegen_clr_place::place_set;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_codgen_clr_operand::handle_operand;
use rustc_middle::{
    mir::{Operand, Place},
    ty::{TyKind, UintTy},
};
use rustc_span::source_map::Spanned;

pub fn bswap<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Root {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `bswap` MUST take in exactly 1 argument!"
    );
    let ty = args[0].node.ty(ctx.body(), ctx.tcx());
    let ty = ctx.monomorphize(ty);
    let tpe = ctx.type_from_cache(ty);
    let operand = handle_operand(&args[0].node, ctx);
    place_set(
        destination,
        match ty.kind() {
            TyKind::Uint(UintTy::U8) => operand,
            TyKind::Uint(_) | TyKind::Int(_) => {
                let mref = MethodRef::new(
                    ClassRef::binary_primitives(ctx),
                    ctx.alloc_string("ReverseEndianness"),
                    ctx.sig([tpe], tpe),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [operand])
            }

            _ => todo!("Can't bswap {tpe:?}"),
        },
        ctx,
    )
}
