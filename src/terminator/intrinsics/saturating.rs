use crate::assembly::MethodCompileCtx;
use cilly::{
    call,
    cil_node::V1Node,
    cil_root::CILRoot,
    cilnode::MethodKind,
    conv_i16, conv_i32, conv_i64, conv_i8, MethodRef, Type, {ClassRef, Int},
};
use rustc_codegen_clr_place::place_set;
use rustc_codegen_clr_type::{utilis::max_value, GetTypeExt};

use rustc_codgen_clr_operand::handle_operand;
use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn saturating_add<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    let a = handle_operand(&args[0].node, ctx);
    let b = handle_operand(&args[1].node, ctx);
    let a_ty = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("saturating_sub works only on types!"),
    );
    let a_type = ctx.type_from_cache(
        ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        ),
    );
    let calc = match a_type {
        Type::Int(Int::USize | Int::U128 | Int::U64 | Int::U32 | Int::U16 | Int::U8) => {
            let sum = crate::binop::add_unchecked(a_ty, a_ty, ctx, a.clone(), b.clone());
            let or = crate::binop::bitop::bit_or_unchecked(a_ty, a_ty, ctx, a.clone(), b.clone());
            let flag = crate::binop::cmp::lt_unchecked(a_ty, sum.clone(), or.clone(), ctx);
            let max = max_value(&a_type, ctx);
            V1Node::select(a_type, max, sum, flag, ctx)
        }
        Type::Int(Int::I32) => {
            let a = conv_i64!(a);
            let b = conv_i64!(b);
            let diff = a + b;
            let clamp = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I64),
                        Type::Int(Int::I64),
                        Type::Int(Int::I64),
                    ],
                    Type::Int(Int::I64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    V1Node::V2(ctx.alloc_node(i64::from(i32::MIN))),
                    V1Node::V2(ctx.alloc_node(i64::from(i32::MAX)))
                ]
            );
            conv_i32!(diff_capped)
        }

        Type::Int(Int::I64) => {
            let a = crate::casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), a, ctx);
            let b = crate::casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), b, ctx);
            let add = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_Addition"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff = call!(ctx.alloc_methodref(add), [a, b]);
            let clamp = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                    ],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    V1Node::const_i128(i128::from(i64::MIN) as u128, ctx),
                    V1Node::const_i128(i128::from(i64::MAX) as u128, ctx),
                ]
            );
            crate::casts::int_to_int(Type::Int(Int::I128), Type::Int(Int::I64), diff_capped, ctx)
        }

        Type::Int(Int::ISize) => {
            let a = crate::casts::int_to_int(Type::Int(Int::ISize), Type::Int(Int::I128), a, ctx);
            let b = crate::casts::int_to_int(Type::Int(Int::ISize), Type::Int(Int::I128), b, ctx);
            let sum = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_Addition"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff = call!(ctx.alloc_methodref(sum), [a, b]);
            let clamp = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                    ],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    // TODO: this assumes isize::MAX == i64::MAX
                    V1Node::const_i128(i128::from(i64::MIN) as u128, ctx),
                    V1Node::const_i128(i128::from(i64::MAX) as u128, ctx),
                ]
            );
            crate::casts::int_to_int(
                Type::Int(Int::I128),
                Type::Int(Int::ISize),
                diff_capped,
                ctx,
            )
        }
        Type::Int(Int::I16) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a + b;
            let mref = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                    ],
                    Type::Int(Int::I32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff_capped = call!(
                ctx.alloc_methodref(mref),
                [
                    diff,
                    V1Node::V2(ctx.alloc_node(i32::from(i16::MIN))),
                    V1Node::V2(ctx.alloc_node(i32::from(i16::MAX)))
                ]
            );
            conv_i16!(diff_capped)
        }
        Type::Int(Int::I8) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a + b;
            let mref = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                    ],
                    Type::Int(Int::I32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff_capped = call!(
                ctx.alloc_methodref(mref),
                [
                    diff,
                    V1Node::V2(ctx.alloc_node(i32::from(i8::MIN))),
                    V1Node::V2(ctx.alloc_node(i32::from(i8::MAX)))
                ]
            );
            conv_i8!(diff_capped)
        }
        _ => todo!("Can't use the intrinsic `saturating_add` on {a_type:?}"),
    };
    place_set(destination, calc, ctx)
}
pub fn saturating_sub<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    let a = handle_operand(&args[0].node, ctx);
    let b = handle_operand(&args[1].node, ctx);
    let a_ty = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("saturating_sub works only on types!"),
    );
    let a_type = ctx.type_from_cache(a_ty);
    let calc = match a_type {
        Type::Int(Int::U128 | Int::U64 | Int::U32 | Int::U16 | Int::U8 | Int::USize) => {
            let undeflow = crate::binop::cmp::lt_unchecked(a_ty, a.clone(), b.clone(), ctx);
            let diff = crate::binop::sub_unchecked(a_ty, a_ty, ctx, a, b);
            let zero = crate::binop::checked::zero(a_ty, ctx);
            V1Node::select(a_type, zero, diff, undeflow, ctx)
        }
        Type::Int(Int::I64) => {
            let a = crate::casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), a, ctx);
            let b = crate::casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), b, ctx);
            let sub = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_Subtraction"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff = call!(ctx.alloc_methodref(sub), [a, b]);
            let clamp = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                    ],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    V1Node::const_i128(i128::from(i64::MIN) as u128, ctx),
                    V1Node::const_i128(i128::from(i64::MAX) as u128, ctx),
                ]
            );
            crate::casts::int_to_int(Type::Int(Int::I128), Type::Int(Int::I64), diff_capped, ctx)
        }
        Type::Int(Int::ISize) => {
            let a = crate::casts::int_to_int(Type::Int(Int::ISize), Type::Int(Int::I128), a, ctx);
            let b = crate::casts::int_to_int(Type::Int(Int::ISize), Type::Int(Int::I128), b, ctx);
            let sub = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_Subtraction"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff = call!(ctx.alloc_methodref(sub), [a, b]);
            let clamp = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                        Type::Int(Int::I128),
                    ],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    // TODO: this assumes isize::MAX == i64::MAX
                    V1Node::const_i128(i128::from(i64::MIN) as u128, ctx),
                    V1Node::const_i128(i128::from(i64::MAX) as u128, ctx),
                ]
            );
            crate::casts::int_to_int(
                Type::Int(Int::I128),
                Type::Int(Int::ISize),
                diff_capped,
                ctx,
            )
        }
        Type::Int(Int::I32) => {
            let a = conv_i64!(a);
            let b = conv_i64!(b);
            let diff = a - b;
            let clamp = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I64),
                        Type::Int(Int::I64),
                        Type::Int(Int::I64),
                    ],
                    Type::Int(Int::I64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    V1Node::V2(ctx.alloc_node(i64::from(i32::MIN))),
                    V1Node::V2(ctx.alloc_node(i64::from(i32::MAX)))
                ]
            );
            conv_i32!(diff_capped)
        }
        Type::Int(Int::I16) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a - b;
            let clamp = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                    ],
                    Type::Int(Int::I32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    V1Node::V2(ctx.alloc_node(i32::from(i16::MIN))),
                    V1Node::V2(ctx.alloc_node(i32::from(i16::MAX)))
                ]
            );
            conv_i16!(diff_capped)
        }
        Type::Int(Int::I8) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a - b;
            let clamp = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Clamp"),
                ctx.sig(
                    [
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                        Type::Int(Int::I32),
                    ],
                    Type::Int(Int::I32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let diff_capped = call!(
                ctx.alloc_methodref(clamp),
                [
                    diff,
                    V1Node::V2(ctx.alloc_node(i32::from(i8::MIN))),
                    V1Node::V2(ctx.alloc_node(i32::from(i8::MAX)))
                ]
            );
            conv_i8!(diff_capped)
        }
        _ => todo!("Can't use the intrinsic `saturating_sub` on {a_type:?}"),
    };
    place_set(destination, calc, ctx)
}
