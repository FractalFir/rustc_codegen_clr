use crate::assembly::MethodCompileCtx;
use cilly::{
    and, call,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_i16, conv_i32, conv_i8, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8, rem_un,
    v2::{cilnode::MethodKind, ClassRef, MethodRef},
    Assembly, Int, Type,
};
use rustc_codegen_clr_place::place_set;
use rustc_codegen_clr_type::GetTypeExt;
use rustc_codgen_clr_operand::handle_operand;
use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
fn ctpop_small_int(asm: &mut cilly::v2::Assembly, operand: CILNode, int: Int) -> CILNode {
    assert!(int.size().is_none_or(|size| size <= 8));
    let mref = MethodRef::new(
        ClassRef::bit_operations(asm),
        asm.alloc_string("PopCount"),
        asm.sig([Type::Int(int)], Type::Int(Int::I32)),
        MethodKind::Static,
        vec![].into(),
    );
    conv_u32!(call!(asm.alloc_methodref(mref), [operand]))
}
pub fn ctpop<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
    let operand = handle_operand(&args[0].node, ctx);
    place_set(
        destination,
        match tpe {
            Type::Int(Int::U64) => ctpop_small_int(ctx, operand, Int::U64),
            Type::Int(Int::I64) => ctpop_small_int(ctx, conv_u64!(operand), Int::U64),
            Type::Int(Int::U32) => ctpop_small_int(ctx, operand, Int::U32),
            Type::Int(Int::U8 | Int::U16 | Int::I8 | Int::I16 | Int::I32) => {
                ctpop_small_int(ctx, conv_u32!(operand), Int::U32)
            }
            Type::Int(Int::USize) => ctpop_small_int(ctx, operand, Int::USize),
            Type::Int(Int::ISize) => ctpop_small_int(ctx, conv_isize!(operand), Int::USize),
            Type::Int(Int::U128) => {
                let mref = MethodRef::new(
                    ClassRef::uint_128(ctx),
                    ctx.alloc_string("PopCount"),
                    ctx.sig([Type::Int(Int::U128)], Type::Int(Int::U128)),
                    MethodKind::Static,
                    vec![].into(),
                );
                crate::casts::int_to_int(
                    Type::Int(Int::U128),
                    Type::Int(Int::U32),
                    call!(ctx.alloc_methodref(mref), [operand]),
                    ctx,
                )
            }
            Type::Int(Int::I128) => {
                let mref = MethodRef::new(
                    ClassRef::int_128(ctx),
                    ctx.alloc_string("PopCount"),
                    ctx.sig([Type::Int(Int::I128)], Type::Int(Int::I128)),
                    MethodKind::Static,
                    vec![].into(),
                );
                crate::casts::int_to_int(
                    Type::Int(Int::I128),
                    Type::Int(Int::U32),
                    call!(ctx.alloc_methodref(mref), [operand]),
                    ctx,
                )
            }
            _ => todo!("Unsported pop count type {tpe:?}"),
        },
        ctx,
    )
}
pub fn ctlz<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctlz` MUST take in exactly 1 argument!"
    );

    let tpe = ctx.type_from_cache(
        ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        ),
    );
    // TODO: this assumes a 64 bit system!
    let sub = match tpe {
        Type::Int(int @ (Int::ISize | Int::USize)) => {
            let mref = MethodRef::new(
                ClassRef::bit_operations(ctx),
                ctx.alloc_string("LeadingZeroCount"),
                ctx.sig([Type::Int(int)], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            return place_set(
                destination,
                conv_u32!(call!(
                    ctx.alloc_methodref(mref),
                    [handle_operand(&args[0].node, ctx)]
                )),
                ctx,
            );
        }
        Type::Ptr(_) => {
            let mref = MethodRef::new(
                ClassRef::bit_operations(ctx),
                ctx.alloc_string("LeadingZeroCount"),
                ctx.sig([Type::Int(Int::USize)], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            return place_set(
                destination,
                conv_u32!(call!(
                    ctx.alloc_methodref(mref),
                    [handle_operand(&args[0].node, ctx)]
                )),
                ctx,
            );
        }
        Type::Int(Int::I64 | Int::U64) => CILNode::V2(ctx.alloc_node(0_i32)),
        Type::Int(Int::I32 | Int::U32) => CILNode::V2(ctx.alloc_node(32_i32)),
        Type::Int(Int::I16 | Int::U16) => CILNode::V2(ctx.alloc_node(48_i32)),
        Type::Int(Int::I8 | Int::U8) => CILNode::V2(ctx.alloc_node(56_i32)),
        Type::Int(Int::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("LeadingZeroCount"),
                ctx.sig([Type::Int(Int::I128)], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            return place_set(
                destination,
                conv_u32!(call!(
                    ctx.alloc_methodref(mref),
                    [handle_operand(&args[0].node, ctx)]
                )),
                ctx,
            );
        }
        Type::Int(Int::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("LeadingZeroCount"),
                ctx.sig([Type::Int(Int::U128)], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            return place_set(
                destination,
                conv_u32!(call!(
                    ctx.alloc_methodref(mref),
                    [handle_operand(&args[0].node, ctx)]
                )),
                ctx,
            );
        }
        _ => todo!("Can't `ctlz`  type {tpe:?} yet!"),
    };
    let mref = MethodRef::new(
        ClassRef::bit_operations(ctx),
        ctx.alloc_string("LeadingZeroCount"),
        ctx.sig([Type::Int(Int::U64)], Type::Int(Int::I32)),
        MethodKind::Static,
        vec![].into(),
    );
    place_set(
        destination,
        conv_u32!(CILNode::Sub(
            Box::new(call!(
                ctx.alloc_methodref(mref),
                [conv_u64!(handle_operand(&args[0].node, ctx))]
            )),
            Box::new(sub)
        )),
        ctx,
    )
}
pub fn cttz<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `ctlz` MUST take in exactly 1 argument!"
    );
    let bit_operations = ClassRef::bit_operations(ctx);
    let tpe = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    let tpe = ctx.type_from_cache(tpe);
    let operand = handle_operand(&args[0].node, ctx);
    match tpe {
        Type::Int(Int::I8) => {
            let ttc = MethodRef::new(
                bit_operations,
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([Type::Int(Int::I32)], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(ctx.alloc_methodref(ttc), [conv_i32!(operand)]));
            let min = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Min"),
                ctx.sig(
                    [Type::Int(Int::U32), Type::Int(Int::U32)],
                    Type::Int(Int::U32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(min),
                    [value_calc, CILNode::V2(ctx.alloc_node(i8::BITS))]
                ),
                ctx,
            )
        }
        Type::Int(Int::I16) => {
            let mref = MethodRef::new(
                bit_operations,
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([Type::Int(Int::I32)], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(ctx.alloc_methodref(mref), [conv_i32!(operand)]));
            let min = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Min"),
                ctx.sig(
                    [Type::Int(Int::U32), Type::Int(Int::U32)],
                    Type::Int(Int::U32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(min),
                    [value_calc, CILNode::V2(ctx.alloc_node(i16::BITS))]
                ),
                ctx,
            )
        }
        Type::Int(Int::U8) => {
            let mref = MethodRef::new(
                bit_operations,
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([Type::Int(Int::U32)], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(ctx.alloc_methodref(mref), [conv_u32!(operand)]));
            let min = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Min"),
                ctx.sig(
                    [Type::Int(Int::U32), Type::Int(Int::U32)],
                    Type::Int(Int::U32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(min),
                    [value_calc, CILNode::V2(ctx.alloc_node(u8::BITS))]
                ),
                ctx,
            )
        }
        Type::Int(Int::U16) => {
            let mref = MethodRef::new(
                bit_operations,
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([Type::Int(Int::U32)], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(ctx.alloc_methodref(mref), [conv_u32!(operand)]));
            let min = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Min"),
                ctx.sig(
                    [Type::Int(Int::U32), Type::Int(Int::U32)],
                    Type::Int(Int::U32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(min),
                    [value_calc, CILNode::V2(ctx.alloc_node(u16::BITS))]
                ),
                ctx,
            )
        }
        Type::Int(Int::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([Type::Int(Int::I128)], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(
                ctx.alloc_methodref(mref),
                [handle_operand(&args[0].node, ctx)]
            ));
            place_set(destination, value_calc, ctx)
        }
        Type::Int(Int::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([Type::Int(Int::U128)], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(
                ctx.alloc_methodref(mref),
                [handle_operand(&args[0].node, ctx)]
            ));
            place_set(destination, value_calc, ctx)
        }
        _ => {
            let mref = MethodRef::new(
                bit_operations,
                ctx.alloc_string("TrailingZeroCount"),
                ctx.sig([tpe], Type::Int(Int::I32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = conv_u32!(call!(ctx.alloc_methodref(mref), [operand]));
            place_set(destination, value_calc, ctx)
        }
    }
}
pub fn rotate_left<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        2,
        "The  `rotate_left` MUST take in exactly 2 arguments!"
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
        Type::Int(
            int @ (Int::U8
            | Int::I8
            | Int::U16
            | Int::I16
            | Int::U32
            | Int::I32
            | Int::U64
            | Int::I64
            | Int::U128
            | Int::I128
            | Int::USize
            | Int::ISize),
        ) => place_set(destination, rol_int(val, conv_i32!(rot), int, ctx), ctx),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
pub fn rol_int(val: CILNode, rot: CILNode, int: Int, asm: &mut cilly::v2::Assembly) -> CILNode {
    let mref = MethodRef::new(
        int.class(asm),
        asm.alloc_string("RotateLeft"),
        asm.sig([Type::Int(int), Type::Int(Int::I32)], Type::Int(int)),
        MethodKind::Static,
        vec![].into(),
    );
    call!(asm.alloc_methodref(mref), [val, rot])
}
pub fn ror_int(val: CILNode, rot: CILNode, int: Int, asm: &mut cilly::v2::Assembly) -> CILNode {
    let mref = MethodRef::new(
        int.class(asm),
        asm.alloc_string("RotateRight"),
        asm.sig([Type::Int(int), Type::Int(Int::I32)], Type::Int(int)),
        MethodKind::Static,
        vec![].into(),
    );
    call!(asm.alloc_methodref(mref), [val, rot])
}
pub fn rotate_right<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
        Type::Int(
            int @ (Int::U8
            | Int::I8
            | Int::U16
            | Int::I16
            | Int::U32
            | Int::I32
            | Int::U64
            | Int::I64
            | Int::U128
            | Int::I128
            | Int::USize
            | Int::ISize),
        ) => place_set(destination, ror_int(val, conv_i32!(rot), int, ctx), ctx),
        _ => todo!("Can't ror {val_tpe:?}"),
    }
}
pub fn bitreverse_u8(byte: CILNode, asm: &mut Assembly) -> CILNode {
    conv_u8!(rem_un!(
        (and!(
            conv_u64!(byte) * CILNode::V2(asm.alloc_node(0x0002_0202_0202_u64)),
            CILNode::V2(asm.alloc_node(0x0108_8442_2010_u64))
        )),
        CILNode::V2(asm.alloc_node(1023_u64))
    ))
}
fn bitreverse_u16(ushort: CILNode, asm: &mut Assembly) -> CILNode {
    conv_u16!(bitreverse_u8(conv_u8!(ushort.clone()), asm)) * CILNode::V2(asm.alloc_node(256_u16))
        + conv_u16!(bitreverse_u8(
            conv_u8!(CILNode::Div(
                Box::new(ushort),
                Box::new(CILNode::V2(asm.alloc_node(256_u16)))
            )),
            asm
        ))
}
pub fn bitreverse_int(val: CILNode, int: Int, asm: &mut cilly::v2::Assembly) -> CILNode {
    let mref = MethodRef::new(
        *asm.main_module(),
        asm.alloc_string(format!("bitreverse_{}", int.as_unsigned().name())),
        asm.sig([Type::Int(int.as_unsigned())], Type::Int(int.as_unsigned())),
        MethodKind::Static,
        vec![].into(),
    );
    crate::casts::int_to_int(
        int.as_unsigned().into(),
        int.into(),
        call!(
            asm.alloc_methodref(mref),
            [crate::casts::int_to_int(
                int.into(),
                int.as_unsigned().into(),
                val,
                asm
            )]
        ),
        asm,
    )
}
pub fn bitreverse<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
            Type::Int(Int::U8) => bitreverse_u8(val, ctx),
            Type::Int(Int::I8) => conv_i8!(bitreverse_u8(val, ctx)),
            Type::Int(Int::U16) => bitreverse_u16(val, ctx),
            Type::Int(Int::I16) => conv_i16!(bitreverse_u16(conv_u16!(val), ctx)),
            Type::Int(
                int @ (Int::I32 | Int::U32 | Int::I64 | Int::U64 | Int::U128 | Int::I128),
            ) => bitreverse_int(val, int, ctx),
            _ => todo!("can't yet bitreverse {val_tpe:?}"),
        },
        ctx,
    )
}
