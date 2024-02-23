use crate::{
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    operand::handle_operand,
    place::place_set,
    r#type::{tycache, DotnetTypeRef, Type},
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy},
};
use rustc_span::source_map::Spanned;
use tycache::TyCache;
pub fn bswap<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> Vec<CILOp> {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `bswap` MUST take in exactly 1 argument!"
    );
    let ty = args[0].node.ty(body, tyctx);
    let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
    let tpe = type_cache.type_from_cache(ty, tyctx, Some(method_instance));
    let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache).flatten();
    place_set(
        destination,
        tyctx,
        match ty.kind() {
            TyKind::Uint(UintTy::U8) => operand,
            TyKind::Uint(_) | TyKind::Int(_) => {
                let mut ops = operand;
                ops.push(CILOp::Call(CallSite::boxed(
                    Some(DotnetTypeRef::binary_primitives()),
                    "ReverseEndianness".into(),
                    FnSig::new(&[tpe.clone()], &tpe),
                    true,
                )));
                ops
            }
            TyKind::Uint(UintTy::U128) | TyKind::Int(IntTy::I128) => {
                todo!("Can't bswap 128 bit ints")
            }
            _ => [
                operand,
                stupid_bswap(ty, tyctx, type_cache, method_instance),
            ]
            .iter()
            .flatten()
            .cloned()
            .collect(),
        },
        body,
        method_instance,
        type_cache,
    )
}
fn stupid_bswap<'tyctx>(
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    type_cache: &mut TyCache,
    method_instance: Instance<'tyctx>,
) -> Vec<CILOp> {
    let size = crate::utilis::compiletime_sizeof(ty, tyctx, method_instance);
    let tpe = type_cache.type_from_cache(ty, tyctx, method_instance.into());
    let mut res = vec![
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(tpe.clone().into()),
    ];
    if tpe == Type::USize || tpe == Type::USize {
        println!("WARNING:bswap assumes sizeof::<usize>() == 8");
    }
    for offset in 0..size {
        res.extend([
            CILOp::LoadAddresOfTMPLocal,
            CILOp::ConvUSize(false),
            CILOp::LdcI32(offset as u32 as i32),
            //CILOp::ConvU32(false),
            //CILOp::ConvISize(false),
            CILOp::Add,
            CILOp::LoadAdressUnderTMPLocal(1),
            CILOp::ConvUSize(false),
            CILOp::LdcI32((size - offset - 1) as u32 as i32),
            CILOp::ConvUSize(false),
            CILOp::Add,
            CILOp::LDIndI8,
            CILOp::STIndI8,
        ]);
    }
    res.extend([
        CILOp::LoadTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]);
    crate::utilis::check_debugable(&res, "bswap fucked up", false);
    res
}
