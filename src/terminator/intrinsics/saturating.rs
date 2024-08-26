use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_i16, conv_i32, conv_i64,
    conv_i8, ldc_i32, ldc_i64, ClassRef, FnSig, Type,
};

use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn saturating_add<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
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
        Type::Int(Int::USize)
        | Type::Int(Int::U128)
        | Type::Int(Int::U64)
        | Type::Int(Int::U32)
        | Type::Int(Int::U16)
        | Type::Int(Int::U8) => {
            let sum = crate::binop::add_unchecked(a_ty, a_ty, ctx, a.clone(), b.clone());
            let or = crate::binop::bitop::bit_or_unchecked(a_ty, a_ty, ctx, a.clone(), b.clone());
            let flag = crate::binop::cmp::lt_unchecked(a_ty, sum.clone(), or.clone());
            let max = crate::r#type::max_value(&a_type);
            CILNode::select(a_type, max, sum, flag)
        }
        Type::Int(Int::I32) => {
            let a = conv_i64!(a);
            let b = conv_i64!(b);
            let diff = a + b;
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::math(),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I64),
                            Type::Int(Int::I64),
                            Type::Int(Int::I64)
                        ],
                        Type::Int(Int::I64)
                    ),
                    true
                ),
                [
                    diff,
                    ldc_i64!(i64::from(i32::MIN)),
                    ldc_i64!(i64::from(i32::MAX))
                ]
            );
            conv_i32!(diff_capped)
        }

        Type::Int(Int::I64) => {
            let a = crate::casts::int_to_int(Type::Int(Int::I64), &Type::Int(Int::I128), a);
            let b = crate::casts::int_to_int(Type::Int(Int::I64), &Type::Int(Int::I128), b);
            let diff = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_Addition".into(),
                    FnSig::new(
                        &[Type::Int(Int::I128), Type::Int(Int::I128)],
                        Type::Int(Int::I128)
                    ),
                    true,
                ),
                [a, b]
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I128),
                            Type::Int(Int::I128),
                            Type::Int(Int::I128)
                        ],
                        Type::Int(Int::I128)
                    ),
                    true
                ),
                [
                    diff,
                    CILNode::const_i128(i128::from(i64::MIN) as u128),
                    CILNode::const_i128(i128::from(i64::MAX) as u128),
                ]
            );
            crate::casts::int_to_int(Type::Int(Int::I128), &Type::Int(Int::I64), diff_capped)
        }

        Type::Int(Int::ISize) => {
            let a = crate::casts::int_to_int(Type::Int(Int::ISize), &Type::Int(Int::I128), a);
            let b = crate::casts::int_to_int(Type::Int(Int::ISize), &Type::Int(Int::I128), b);
            let diff = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_Addition".into(),
                    FnSig::new(
                        &[Type::Int(Int::I128), Type::Int(Int::I128)],
                        Type::Int(Int::I128)
                    ),
                    true,
                ),
                [a, b]
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I128),
                            Type::Int(Int::I128),
                            Type::Int(Int::I128)
                        ],
                        Type::Int(Int::I128)
                    ),
                    true
                ),
                [
                    diff,
                    // TODO: this assumes isize::MAX == i64::MAX
                    CILNode::const_i128(i128::from(i64::MIN) as u128),
                    CILNode::const_i128(i128::from(i64::MAX) as u128),
                ]
            );
            crate::casts::int_to_int(Type::Int(Int::I128), &Type::Int(Int::ISize), diff_capped)
        }
        Type::Int(Int::I16) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a + b;
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::math(),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I32),
                            Type::Int(Int::I32),
                            Type::Int(Int::I32)
                        ],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [
                    diff,
                    ldc_i32!(i32::from(i16::MIN)),
                    ldc_i32!(i32::from(i16::MAX))
                ]
            );
            conv_i16!(diff_capped)
        }
        Type::Int(Int::I8) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a + b;
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::math(),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I32),
                            Type::Int(Int::I32),
                            Type::Int(Int::I32)
                        ],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [
                    diff,
                    ldc_i32!(i32::from(i8::MIN)),
                    ldc_i32!(i32::from(i8::MAX))
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
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
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
        Type::Int(Int::U128)
        | Type::Int(Int::U64)
        | Type::Int(Int::U32)
        | Type::Int(Int::U16)
        | Type::Int(Int::U8)
        | Type::Int(Int::USize) => {
            let undeflow = crate::binop::cmp::lt_unchecked(a_ty, a.clone(), b.clone());
            let diff = crate::binop::sub_unchecked(a_ty, a_ty, ctx, a, b);
            let zero = crate::binop::checked::zero(a_ty);
            CILNode::select(a_type, zero, diff, undeflow)
        }
        Type::Int(Int::I64) => {
            let a = crate::casts::int_to_int(Type::Int(Int::I64), &Type::Int(Int::I128), a);
            let b = crate::casts::int_to_int(Type::Int(Int::I64), &Type::Int(Int::I128), b);
            let diff = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_Subtraction".into(),
                    FnSig::new(
                        &[Type::Int(Int::I128), Type::Int(Int::I128)],
                        Type::Int(Int::I128)
                    ),
                    true,
                ),
                [a, b]
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I128),
                            Type::Int(Int::I128),
                            Type::Int(Int::I128)
                        ],
                        Type::Int(Int::I128)
                    ),
                    true
                ),
                [
                    diff,
                    CILNode::const_i128(i128::from(i64::MIN) as u128),
                    CILNode::const_i128(i128::from(i64::MAX) as u128),
                ]
            );
            crate::casts::int_to_int(Type::Int(Int::I128), &Type::Int(Int::I64), diff_capped)
        }
        Type::Int(Int::ISize) => {
            let a = crate::casts::int_to_int(Type::Int(Int::ISize), &Type::Int(Int::I128), a);
            let b = crate::casts::int_to_int(Type::Int(Int::ISize), &Type::Int(Int::I128), b);
            let diff = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_Subtraction".into(),
                    FnSig::new(
                        &[Type::Int(Int::I128), Type::Int(Int::I128)],
                        Type::Int(Int::I128)
                    ),
                    true,
                ),
                [a, b]
            );
            #[allow(clippy::cast_sign_loss)]
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I128),
                            Type::Int(Int::I128),
                            Type::Int(Int::I128)
                        ],
                        Type::Int(Int::I128)
                    ),
                    true
                ),
                [
                    diff,
                    // TODO: this assumes isize::MAX == i64::MAX
                    CILNode::const_i128(i128::from(i64::MIN) as u128),
                    CILNode::const_i128(i128::from(i64::MAX) as u128),
                ]
            );
            crate::casts::int_to_int(Type::Int(Int::I128), &Type::Int(Int::ISize), diff_capped)
        }
        Type::Int(Int::I32) => {
            let a = conv_i64!(a);
            let b = conv_i64!(b);
            let diff = a - b;
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::math(),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I64),
                            Type::Int(Int::I64),
                            Type::Int(Int::I64)
                        ],
                        Type::Int(Int::I64)
                    ),
                    true
                ),
                [
                    diff,
                    ldc_i64!(i64::from(i32::MIN)),
                    ldc_i64!(i64::from(i32::MAX))
                ]
            );
            conv_i32!(diff_capped)
        }
        Type::Int(Int::I16) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a - b;
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::math(),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I32),
                            Type::Int(Int::I32),
                            Type::Int(Int::I32)
                        ],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [
                    diff,
                    ldc_i32!(i32::from(i16::MIN)),
                    ldc_i32!(i32::from(i16::MAX))
                ]
            );
            conv_i16!(diff_capped)
        }
        Type::Int(Int::I8) => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a - b;
            let diff_capped = call!(
                CallSite::new_extern(
                    ClassRef::math(),
                    "Clamp".into(),
                    FnSig::new(
                        &[
                            Type::Int(Int::I32),
                            Type::Int(Int::I32),
                            Type::Int(Int::I32)
                        ],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [
                    diff,
                    ldc_i32!(i32::from(i8::MIN)),
                    ldc_i32!(i32::from(i8::MAX))
                ]
            );
            conv_i8!(diff_capped)
        }
        _ => todo!("Can't use the intrinsic `saturating_sub` on {a_type:?}"),
    };
    place_set(destination, calc, ctx)
}
