use crate::{operand::handle_operand, place::place_set, r#type::tycache::TyCache};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, fn_sig::FnSig, DotnetTypeRef,
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, TyCtxt, TyKind, UintTy},
};
use rustc_span::source_map::Spanned;

pub fn bswap<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `bswap` MUST take in exactly 1 argument!"
    );
    let ty = args[0].node.ty(body, tyctx);
    let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
    let tpe = type_cache.type_from_cache(ty, tyctx, Some(method_instance));
    let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    place_set(
        destination,
        tyctx,
        match ty.kind() {
            TyKind::Uint(UintTy::U8) => operand,
            TyKind::Uint(_) | TyKind::Int(_) => {
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::binary_primitives()),
                        "ReverseEndianness".into(),
                        FnSig::new(&[tpe.clone()], tpe),
                        true,
                    ),
                    [operand]
                )
            }

            _ => todo!("Can't bswap {tpe:?}"),
            /*_ => [
                operand,
                stupid_bswap(ty, tyctx, type_cache, method_instance),
            ]
            .iter()
            .flatten()
            .cloned()
            .collect(),*/
        },
        body,
        method_instance,
        type_cache,
    )
}
