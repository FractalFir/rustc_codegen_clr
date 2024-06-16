use crate::{operand::handle_operand, place::place_set, r#type::tycache::TyCache};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_u32, conv_u64,
    conv_usize, fn_sig::FnSig, ldc_i32, ldc_u32, or, size_of, sub, DotnetTypeRef, Type,
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, TyCtxt},
};
use rustc_span::source_map::Spanned;
pub fn ctpop<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctpop` MUST take in exactly 1 argument!"
    );
    let tpe = type_cache.type_from_cache(
        crate::utilis::monomorphize(
            &method_instance,
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
            tyctx,
        ),
        tyctx,
        method_instance,
    );
    let bit_operations =
        DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
            .with_valuetype(false);
    let bit_operations = Some(bit_operations);
    let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    place_set(
        destination,
        tyctx,
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
        body,
        method_instance,
        type_cache,
    )
}
pub fn ctlz<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
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

    let tpe = crate::utilis::monomorphize(
        &method_instance,
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
        tyctx,
    );
    let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
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
                tyctx,
                conv_u32!(call!(
                    CallSite::new_extern(
                        DotnetTypeRef::int_128(),
                        "LeadingZeroCount".into(),
                        FnSig::new([Type::I128], Type::I128),
                        true
                    ),
                    [handle_operand(
                        &args[0].node,
                        tyctx,
                        body,
                        method_instance,
                        type_cache
                    )]
                )),
                body,
                method_instance,
                type_cache,
            )
        }
        Type::U128 => {
            return place_set(
                destination,
                tyctx,
                conv_u32!(call!(
                    CallSite::new_extern(
                        DotnetTypeRef::uint_128(),
                        "LeadingZeroCount".into(),
                        FnSig::new([Type::U128], Type::U128),
                        true
                    ),
                    [handle_operand(
                        &args[0].node,
                        tyctx,
                        body,
                        method_instance,
                        type_cache
                    )]
                )),
                body,
                method_instance,
                type_cache,
            )
        }
        _ => todo!("Can't `ctlz`  type {tpe:?} yet!"),
    };
    place_set(
        destination,
        tyctx,
        conv_u32!(sub!(
            call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "LeadingZeroCount".into(),
                    FnSig::new(&[Type::U64], Type::I32),
                    true,
                ),
                [conv_u64!(handle_operand(
                    &args[0].node,
                    tyctx,
                    body,
                    method_instance,
                    type_cache
                ))]
            ),
            sub
        )),
        body,
        method_instance,
        type_cache,
    )
}
pub fn cttz<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctlz` MUST take in exactly 1 argument!"
    );
    let bit_operations =
        DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
            .with_valuetype(false);
    let tpe = crate::utilis::monomorphize(
        &method_instance,
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
        tyctx,
    );
    let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
    let bit_operations = Some(bit_operations);
    let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);

    place_set(
        destination,
        tyctx,
        conv_u32!(call!(
            CallSite::boxed(
                bit_operations.clone(),
                "TrailingZeroCount".into(),
                FnSig::new(&[tpe], Type::I32),
                true,
            ),
            [operand]
        )),
        body,
        method_instance,
        type_cache,
    )
}
pub fn rotate_left<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
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
    let val_tpe = crate::utilis::monomorphize(
        &method_instance,
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
        tyctx,
    );
    let val_tpe = type_cache.type_from_cache(val_tpe, tyctx, method_instance);
    let val = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    let rot = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    match val_tpe {
        Type::U8 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(8) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::U16 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(16) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::U32 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(32) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::U64 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(64) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::USize => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::ShrUn(
                    Box::new(val),
                    Box::new(size_of!(Type::USize) * ldc_i32!(8) - rot)
                )
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::I8 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(8) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::I16 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(16) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::I32 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(32) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::I64 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(Box::new(val), Box::new(ldc_u32!(64) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        Type::ISize => place_set(
            destination,
            tyctx,
            or!(
                CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shr(
                    Box::new(val),
                    Box::new(size_of!(Type::ISize) * ldc_i32!(8) - rot)
                )
            ),
            body,
            method_instance,
            type_cache,
        ),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
pub fn rotate_right<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
    );
    let val_tpe = crate::utilis::monomorphize(
        &method_instance,
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
        tyctx,
    );
    let val_tpe = type_cache.type_from_cache(val_tpe, tyctx, method_instance);
    let val = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    let rot = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    match val_tpe {
        Type::U32 => place_set(
            destination,
            tyctx,
            or!(
                CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shl(Box::new(val), Box::new(ldc_u32!(32) - rot))
            ),
            body,
            method_instance,
            type_cache,
        ),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
