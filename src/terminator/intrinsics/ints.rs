use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_i32, conv_u32, conv_u64,
    fn_sig::FnSig, ldc_i32, ldc_i64, ldc_u32, ldc_u64, or, size_of, sub, DotnetTypeRef, Type,
};
use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn ctpop<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctpop` MUST take in exactly 1 argument!"
    );
    let tpe = ctx.type_from_cache(
        ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        ),
    );
    let bit_operations =
        DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
            .with_valuetype(false);
    let bit_operations = Some(bit_operations);
    let operand = handle_operand(&args[0].node, ctx);
    place_set(
        destination,
        match tpe {
            Type::U64 => conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::U64], Type::I32),
                    true,
                ),
                [operand]
            )),
            Type::U32 => conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::U32], Type::I32),
                    true,
                ),
                [operand]
            )),
            Type::USize => conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::USize], Type::I32),
                    true,
                ),
                [operand]
            )),
            _ => todo!("Unsported pop count type {tpe:?}"),
        },
        ctx,
    )
}
pub fn ctlz<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctlz` MUST take in exactly 1 argument!"
    );
    let bit_operations =
        DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
            .with_valuetype(false);
    let bit_operations = Some(bit_operations);

    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);
    // TODO: this assumes a 64 bit system!
    let sub = match tpe {
        Type::ISize | Type::USize | Type::Ptr(_) => {
            ldc_i32!(64) - (size_of!(tpe.clone()) * ldc_i32!(8))
        }
        Type::I64 | Type::U64 => ldc_i32!(0),
        Type::I32 | Type::U32 => ldc_i32!(32),
        Type::I16 | Type::U16 => ldc_i32!(48),
        Type::I8 | Type::U8 => ldc_i32!(56),
        Type::I128 => {
            return place_set(
                destination,
                conv_u32!(call!(
                    CallSite::new_extern(
                        DotnetTypeRef::int_128(),
                        "LeadingZeroCount".into(),
                        FnSig::new([Type::I128], Type::I128),
                        true
                    ),
                    [handle_operand(&args[0].node, ctx)]
                )),
                ctx,
            )
        }
        Type::U128 => {
            return place_set(
                destination,
                conv_u32!(call!(
                    CallSite::new_extern(
                        DotnetTypeRef::uint_128(),
                        "LeadingZeroCount".into(),
                        FnSig::new([Type::U128], Type::U128),
                        true
                    ),
                    [handle_operand(&args[0].node, ctx)]
                )),
                ctx,
            )
        }
        _ => todo!("Can't `ctlz`  type {tpe:?} yet!"),
    };
    place_set(
        destination,
        conv_u32!(sub!(
            call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "LeadingZeroCount".into(),
                    FnSig::new(&[Type::U64], Type::I32),
                    true,
                ),
                [conv_u64!(handle_operand(&args[0].node, ctx))]
            ),
            sub
        )),
        ctx,
    )
}
pub fn cttz<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctlz` MUST take in exactly 1 argument!"
    );
    let bit_operations =
        DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
            .with_valuetype(false);
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);
    let bit_operations = Some(bit_operations);
    let operand = handle_operand(&args[0].node, ctx);
    match tpe {
        Type::I16 | Type::I8 => place_set(
            destination,
            conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::I32], Type::I32),
                    true,
                ),
                [conv_i32!(operand)]
            )),
            ctx,
        ),
        Type::U16 | Type::U8 => place_set(
            destination,
            conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::U32], Type::I32),
                    true,
                ),
                [conv_u32!(operand)]
            )),
            ctx,
        ),
        _ => place_set(
            destination,
            conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[tpe], Type::I32),
                    true,
                ),
                [operand]
            )),
            ctx,
        ),
    }
}
pub fn rotate_left<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        2,
        "The intrinsic `rotate_left` MUST take in exactly 2 arguments!"
    );
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
    );
    let val_tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let val_tpe = ctx.type_from_cache(val_tpe);
    let val = handle_operand(&args[0].node, ctx);
    let rot = handle_operand(&args[0].node, ctx);
    match val_tpe {
        Type::U8 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(8) - rot))
            ),
            ctx,
        ),
        Type::U16 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(16) - rot))
            ),
            ctx,
        ),
        Type::U32 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(32) - rot))
            ),
            ctx,
        ),
        Type::U64 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u64!(64) - rot))
            ),
            ctx,
        ),
        Type::USize => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(
                    Box::new(val),
                    Box::new(size_of!(Type::USize) * ldc_i32!(8) - rot)
                )
            ),
            ctx,
        ),
        Type::I8 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(8) - rot))
            ),
            ctx,
        ),
        Type::I16 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(16) - rot))
            ),
            ctx,
        ),
        Type::I32 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(32) - rot))
            ),
            ctx,
        ),
        Type::I64 => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_i64!(64) - rot))
            ),
            ctx,
        ),
        Type::ISize => place_set(
            destination,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(
                    Box::new(val),
                    Box::new(size_of!(Type::ISize) * ldc_i32!(8) - rot)
                )
            ),
            ctx,
        ),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
pub fn rotate_right<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
    );
    let val_tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let val_tpe = ctx.type_from_cache(val_tpe);
    let val = handle_operand(&args[0].node, ctx);
    let rot = handle_operand(&args[0].node, ctx);
    match val_tpe {
        Type::U8 => place_set(
            destination,
            or!(
                CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shl(Box::new(val), Box::new(ldc_u32!(8) - rot))
            ),
            ctx,
        ),
        Type::U16 => place_set(
            destination,
            or!(
                CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shl(Box::new(val), Box::new(ldc_u32!(16) - rot))
            ),
            ctx,
        ),
        Type::U32 => place_set(
            destination,
            or!(
                CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shl(Box::new(val), Box::new(ldc_u32!(32) - rot))
            ),
            ctx,
        ),
        Type::U64 => place_set(
            destination,
            or!(
                CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shl(Box::new(val), Box::new(ldc_u32!(64) - rot))
            ),
            ctx,
        ),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
