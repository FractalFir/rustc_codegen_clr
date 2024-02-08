use crate::{
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    operand::handle_operand,
    place::place_set,
    r#type::{tycache, DotnetTypeRef, Type},
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy},
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
    let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    place_set(
        destination,
        tyctx,
        match ty.kind() {
            TyKind::Uint(UintTy::U8) => operand,
            /*
            TyKind::Uint(UintTy::U16) => [
                    operand,
                    vec![
                        CILOp::NewTMPLocal(Type::U16.into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shr,
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shl,
                        CILOp::Or,
                        CILOp::FreeTMPLocal,
                    ],
                ]
                .iter()
                .flatten()
                .cloned()
                .collect(), */
                /*
                TyKind::Uint(UintTy::U32) => [
                    operand,
                    vec![
                        //CILOp::ConvU32(false),
                        CILOp::NewTMPLocal(Type::U32.into()),
                        CILOp::SetTMPLocal,
                        // 1 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(24),
                        CILOp::Shl,
                        // 4 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(24),
                        CILOp::ShrUn,
                        CILOp::ConvU32(false),
                        CILOp::Or,
                        // 2 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shl,
                        CILOp::LdcI32(0xFF << 16),
                        CILOp::And,
                        // 3 byte
                        CILOp::LoadTMPLocal,
                        CILOp::LdcI32(8),
                        CILOp::Shr,
                        CILOp::LdcI32(0xFF << 8),
                        CILOp::And,
                        CILOp::Or,
                        CILOp::Or,
                        //CILOp::ConvU16(false),
                        CILOp::FreeTMPLocal,
                    ],
                ]
                .iter()
                .flatten()
                .cloned()
                .collect(),*/
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
            CILOp::Add,
            CILOp::LoadAdressUnderTMPLocal(1),
            CILOp::ConvUSize(false),
            CILOp::LdcI32((size - offset - 1) as u32 as i32),
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
