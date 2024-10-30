use crate::{
    assembly::MethodCompileCtx,
    operand::handle_operand,
    place::{place_adress, place_set},
    utilis::field_descrptor,
};
use cilly::{
    call, cil_node::CILNode, cil_root::CILRoot, cilnode::MethodKind, conv_usize, v2::ClassRef, Int,
    MethodRef, Type,
};
use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn xchg<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    let interlocked = ClassRef::interlocked(ctx);
    // *T
    let dst = handle_operand(&args[0].node, ctx);
    // T
    let new = handle_operand(&args[1].node, ctx);

    debug_assert_eq!(
        args.len(),
        2,
        "The intrinsic `atomic_xchg` MUST take in exactly 3 argument!"
    );
    let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
    let src_type = ctx.type_from_cache(src_type);
    let uint8_ref = ctx.nref(Type::Int(Int::U8));
    let xchng = MethodRef::new(
        *ctx.main_module(),
        ctx.alloc_string("atomic_xchng_u8"),
        ctx.sig([uint8_ref, Type::Int(Int::U8)], Type::Int(Int::U8)),
        MethodKind::Static,
        vec![].into(),
    );
    match src_type {
        Type::Int(Int::U8) => {
            return place_set(
                destination,
                call!(ctx.alloc_methodref(xchng), [dst, new]),
                ctx,
            )
        }
        Type::Ptr(_) => {
            let usize_ref = ctx.nref(Type::Int(Int::USize));
            let call_site = MethodRef::new(
                interlocked,
                ctx.alloc_string("Exchange"),
                ctx.sig([usize_ref, Type::Int(Int::USize)], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            return place_set(
                destination,
                call!(
                    ctx.alloc_methodref(call_site),
                    [
                        Box::new(dst).cast_ptr(ctx.nref(Type::Int(Int::USize))),
                        conv_usize!(new),
                    ]
                )
                .cast_ptr(src_type),
                ctx,
            );
        }
        Type::Int(Int::I8 | Int::U16 | Int::I16) | Type::Bool | Type::PlatformChar => {
            todo!("can't atomic_xchg {src_type:?}")
        }
        _ => (),
    }
    let src_ref = ctx.nref(src_type);
    let call_site = MethodRef::new(
        interlocked,
        ctx.alloc_string("Exchange"),
        ctx.sig([src_ref, src_type], src_type),
        MethodKind::Static,
        vec![].into(),
    );
    // T
    place_set(
        destination,
        call!(ctx.alloc_methodref(call_site), [dst, new]),
        ctx,
    )
}
pub fn cxchg<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    let interlocked = ClassRef::interlocked(ctx);
    // *T
    let dst = handle_operand(&args[0].node, ctx);
    // T
    let comparand = handle_operand(&args[1].node, ctx);
    // T
    let src = handle_operand(&args[2].node, ctx);
    debug_assert_eq!(
        args.len(),
        3,
        "The intrinsic `atomic_cxchgweak_acquire_acquire` MUST take in exactly 3 argument!"
    );
    let src_type = ctx.monomorphize(args[2].node.ty(ctx.body(), ctx.tcx()));
    let src_type = ctx.type_from_cache(src_type);

    let value = src;

    #[allow(clippy::single_match_else)]
    let exchange_res = match &src_type {
        Type::Ptr(_) => {
            let usize_ref = ctx.nref(Type::Int(Int::USize));
            let call_site = MethodRef::new(
                interlocked,
                ctx.alloc_string("CompareExchange"),
                ctx.sig(
                    [usize_ref, Type::Int(Int::USize), Type::Int(Int::USize)],
                    Type::Int(Int::USize),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(call_site),
                [
                    Box::new(dst).cast_ptr(usize_ref),
                    conv_usize!(value),
                    conv_usize!(comparand.clone())
                ]
            )
            .cast_ptr(src_type)
        }
        // TODO: this is a bug, on purpose. The 1 byte compare exchange is not supported untill .NET 9. Remove after November, when .NET 9 Releases.
        Type::Int(Int::U8) => comparand.clone(),
        _ => {
            let src_ref = ctx.nref(src_type);
            let call_site = MethodRef::new(
                interlocked,
                ctx.alloc_string("CompareExchange"),
                ctx.sig([src_ref, src_type, src_type], src_type),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(call_site),
                [dst, value, comparand.clone()]
            )
        }
    };
    let dst_ty = destination.ty(ctx.body(), ctx.tcx());
    let val_desc = field_descrptor(dst_ty.ty, 0, ctx);
    let flag_desc = field_descrptor(dst_ty.ty, 1, ctx);
    CILNode::cxchng_res_val(
        exchange_res,
        comparand,
        place_adress(destination, ctx),
        val_desc,
        flag_desc,
    )
}
