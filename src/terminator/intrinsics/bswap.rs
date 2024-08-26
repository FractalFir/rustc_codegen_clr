use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, fn_sig::FnSig, v2::ClassRef,
};
use rustc_middle::{
    mir::{Operand, Place},
    ty::{TyKind, UintTy},
};
use rustc_span::source_map::Spanned;

pub fn bswap<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> CILRoot {
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
                call!(
                    CallSite::boxed(
                        Some(ClassRef::binary_primitives(ctx.asm_mut())),
                        "ReverseEndianness".into(),
                        FnSig::new(&[tpe.clone()], tpe),
                        true,
                    ),
                    [operand]
                )
            }

            _ => todo!("Can't bswap {tpe:?}"),
        },
        ctx,
    )
}
