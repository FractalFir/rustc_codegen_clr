use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{
    and, call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_i16, conv_i32,
    conv_i8, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8, conv_usize, div, fn_sig::FnSig,
    ldc_i32, ldc_i64, ldc_u32, ldc_u64, or, rem_un, size_of, sub, DotnetTypeRef, Type,
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
    let bit_operations = DotnetTypeRef::bit_operations();
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
            Type::I64 => conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::U64], Type::I32),
                    true,
                ),
                [conv_u64!(operand)]
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

            Type::U8 | Type::U16 | Type::I8 | Type::I16 | Type::I32 => conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::U32], Type::I32),
                    true,
                ),
                [conv_u32!(operand)]
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
            Type::ISize => conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "PopCount".into(),
                    FnSig::new(&[Type::USize], Type::I32),
                    true,
                ),
                [conv_isize!(operand)]
            )),
            Type::U128 => crate::casts::int_to_int(
                Type::U128,
                &Type::U32,
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::uint_128().clone(),
                        "PopCount".into(),
                        FnSig::new(&[Type::U128], Type::U128),
                        true,
                    ),
                    [operand]
                ),
            ),
            Type::I128 => crate::casts::int_to_int(
                Type::I128,
                &Type::U32,
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::int_128().clone(),
                        "PopCount".into(),
                        FnSig::new(&[Type::I128], Type::I128),
                        true,
                    ),
                    [operand]
                ),
            ),
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
    let bit_operations = DotnetTypeRef::bit_operations();
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
    let bit_operations = DotnetTypeRef::bit_operations();
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);
    let bit_operations = Some(bit_operations);
    let operand = handle_operand(&args[0].node, ctx);
    match tpe {
        Type::I8 => {
            let value_calc = conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::I32], Type::I32),
                    true,
                ),
                [conv_i32!(operand)]
            ));
            place_set(
                destination,
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::math(),
                        "Min".into(),
                        FnSig::new([Type::U32, Type::U32], Type::U32),
                        true
                    ),
                    [value_calc, ldc_u32!(i8::BITS)]
                ),
                ctx,
            )
        }
        Type::I16 => {
            let value_calc = conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::I32], Type::I32),
                    true,
                ),
                [conv_i32!(operand)]
            ));
            place_set(
                destination,
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::math(),
                        "Min".into(),
                        FnSig::new([Type::U32, Type::U32], Type::U32),
                        true
                    ),
                    [value_calc, ldc_u32!(i16::BITS)]
                ),
                ctx,
            )
        }
        Type::U8 => {
            let value_calc = conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::U32], Type::I32),
                    true,
                ),
                [conv_u32!(operand)]
            ));
            place_set(
                destination,
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::math(),
                        "Min".into(),
                        FnSig::new([Type::U32, Type::U32], Type::U32),
                        true
                    ),
                    [value_calc, ldc_u32!(u8::BITS)]
                ),
                ctx,
            )
        }
        Type::U16 => {
            let value_calc = conv_u32!(call!(
                CallSite::boxed(
                    bit_operations.clone(),
                    "TrailingZeroCount".into(),
                    FnSig::new(&[Type::U32], Type::I32),
                    true,
                ),
                [conv_u32!(operand)]
            ));
            place_set(
                destination,
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::math(),
                        "Min".into(),
                        FnSig::new([Type::U32, Type::U32], Type::U32),
                        true
                    ),
                    [value_calc, ldc_u32!(u16::BITS)]
                ),
                ctx,
            )
        }
        Type::I128 => place_set(
            destination,
            conv_u32!(call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "TrailingZeroCount".into(),
                    FnSig::new([Type::I128], Type::I128),
                    true
                ),
                [handle_operand(&args[0].node, ctx)]
            )),
            ctx,
        ),
        Type::U128 => place_set(
            destination,
            conv_u32!(call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "TrailingZeroCount".into(),
                    FnSig::new([Type::U128], Type::U128),
                    true
                ),
                [handle_operand(&args[0].node, ctx)]
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
    let val_tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let val_tpe = ctx.type_from_cache(val_tpe);
    let val = handle_operand(&args[0].node, ctx);
    let rot = handle_operand(&args[1].node, ctx);
    match val_tpe {
        Type::U8 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::byte(),
                    "RotateLeft".into(),
                    FnSig::new([Type::U8, Type::I32], Type::U8),
                    true
                ),
                [val, conv_i32!(rot)]
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
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateLeft".into(),
                    FnSig::new([Type::U32, Type::I32], Type::U32),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::U64 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateLeft".into(),
                    FnSig::new([Type::U64, Type::I32], Type::U64),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::USize => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateLeft".into(),
                    FnSig::new([Type::USize, Type::I32], Type::USize),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I8 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::sbyte(),
                    "RotateLeft".into(),
                    FnSig::new([Type::I8, Type::I32], Type::I8),
                    true
                ),
                [val, conv_i32!(rot)]
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
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateLeft".into(),
                    FnSig::new([Type::U32, Type::I32], Type::U32),
                    true
                ),
                [conv_u32!(val), conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I64 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateLeft".into(),
                    FnSig::new([Type::U64, Type::I32], Type::U32),
                    true
                ),
                [conv_u64!(val), conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::ISize => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateLeft".into(),
                    FnSig::new([Type::USize, Type::I32], Type::U32),
                    true
                ),
                [conv_usize!(val), conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::U128 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "RotateLeft".into(),
                    FnSig::new([Type::U128, Type::I32], Type::U128),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I128 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "RotateLeft".into(),
                    FnSig::new([Type::I128, Type::I32], Type::I128),
                    true
                ),
                [val, conv_i32!(rot)]
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
        2,
        "The  `rotate_right` MUST take in exactly 2 arguments!"
    );
    let val_tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let val_tpe = ctx.type_from_cache(val_tpe);
    let val = handle_operand(&args[0].node, ctx);
    let rot = handle_operand(&args[1].node, ctx);
    match val_tpe {
        Type::U16 => place_set(
            destination,
            or!(
                CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                CILNode::Shl(Box::new(val), Box::new(ldc_u32!(16) - rot))
            ),
            ctx,
        ),
        Type::U8 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::byte(),
                    "RotateRight".into(),
                    FnSig::new([Type::U8, Type::I32], Type::U8),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::U32 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateRight".into(),
                    FnSig::new([Type::U32, Type::I32], Type::U32),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::U64 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateRight".into(),
                    FnSig::new([Type::U64, Type::I32], Type::U64),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::USize => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateRight".into(),
                    FnSig::new([Type::USize, Type::I32], Type::USize),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I8 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::sbyte(),
                    "RotateRight".into(),
                    FnSig::new([Type::I8, Type::I32], Type::I8),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I32 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateRight".into(),
                    FnSig::new([Type::U32, Type::I32], Type::U32),
                    true
                ),
                [conv_u32!(val), conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I64 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateRight".into(),
                    FnSig::new([Type::U64, Type::I32], Type::U32),
                    true
                ),
                [conv_u64!(val), conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::ISize => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::bit_operations(),
                    "RotateRight".into(),
                    FnSig::new([Type::USize, Type::I32], Type::U32),
                    true
                ),
                [conv_usize!(val), conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::U128 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "RotateRight".into(),
                    FnSig::new([Type::U128, Type::I32], Type::U128),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        Type::I128 => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "RotateRight".into(),
                    FnSig::new([Type::I128, Type::I32], Type::I128),
                    true
                ),
                [val, conv_i32!(rot)]
            ),
            ctx,
        ),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
pub fn bitreverse_u8(byte: CILNode) -> CILNode {
    conv_u8!(rem_un!(
        (and!(
            conv_u64!(byte) * ldc_u64!(0x0002_0202_0202),
            ldc_u64!(0x0108_8442_2010)
        )),
        ldc_u64!(1023)
    ))
}
fn bitreverse_u16(ushort: CILNode) -> CILNode {
    conv_u16!(bitreverse_u8(conv_u8!(ushort.clone()))) * conv_u16!(ldc_u32!(256))
        + conv_u16!(bitreverse_u8(conv_u8!(div!(
            ushort,
            conv_u16!(ldc_u32!(256))
        ))))
}
pub fn bitreverse<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The  `bitreverse` MUST take in exactly 1 argument!"
    );
    let val_tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let val_tpe = ctx.type_from_cache(val_tpe);
    let val = handle_operand(&args[0].node, ctx);
    place_set(
        destination,
        match val_tpe {
            Type::U8 => bitreverse_u8(val),
            Type::I8 => conv_i8!(bitreverse_u8(val)),
            Type::U16 => bitreverse_u16(val),
            Type::I16 => conv_i16!(bitreverse_u16(conv_u16!(val))),
            Type::U32 => call!(
                CallSite::builtin(
                    "bitreverse_u32".into(),
                    FnSig::new(&[Type::U32], Type::U32),
                    true
                ),
                [val]
            ),
            Type::I32 => crate::casts::int_to_int(
                Type::U32,
                &Type::I32,
                call!(
                    CallSite::builtin(
                        "bitreverse_u32".into(),
                        FnSig::new(&[Type::U32], Type::U32),
                        true
                    ),
                    [crate::casts::int_to_int(Type::I32, &Type::U32, val)]
                ),
            ),
            Type::U64 => call!(
                CallSite::builtin(
                    "bitreverse_u64".into(),
                    FnSig::new(&[Type::U64], Type::U64),
                    true
                ),
                [val]
            ),
            Type::I64 => crate::casts::int_to_int(
                Type::U64,
                &Type::I64,
                call!(
                    CallSite::builtin(
                        "bitreverse_u64".into(),
                        FnSig::new(&[Type::U64], Type::U64),
                        true
                    ),
                    [crate::casts::int_to_int(Type::I64, &Type::U64, val)]
                ),
            ),
            Type::U128 => call!(
                CallSite::builtin(
                    "bitreverse_u128".into(),
                    FnSig::new(&[Type::U128], Type::U128,),
                    true
                ),
                [val]
            ),
            Type::I128 => crate::casts::int_to_int(
                Type::U128,
                &Type::I128,
                call!(
                    CallSite::builtin(
                        "bitreverse_u128".into(),
                        FnSig::new(&[Type::U128], Type::U128,),
                        true
                    ),
                    [crate::casts::int_to_int(Type::I128, &Type::U128, val)]
                ),
            ),

            _ => todo!("can't yet bitreverse {val_tpe:?}"),
        },
        ctx,
    )
}
