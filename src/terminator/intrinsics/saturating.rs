use crate::{assembly::MethodCompileCtx, operand::handle_operand, place::place_set};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_i16, conv_i32, conv_i64,
    conv_i8, ldc_i32, ldc_i64, lt_un, DotnetTypeRef, FnSig, Type,
};

use rustc_middle::{
    mir::{Operand, Place},
    ty::Instance,
};
use rustc_span::source_map::Spanned;
pub fn saturating_add<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    let a = handle_operand(&args[0].node, ctx);
    let b = handle_operand(&args[1].node, ctx);
    let a_type = ctx.type_from_cache(
        ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        ),
    );
    let calc = match a_type {
        Type::USize | Type::U64 | Type::U32 | Type::U16 | Type::U8 => {
            let sum = a.clone() + b.clone();
            let or = a | b;
            let flag = lt_un!(sum.clone(), or);
            let max = crate::r#type::max_value(&a_type);
            CILNode::select(a_type, max, sum, flag)
        }
        Type::I32 => {
            let a = conv_i64!(a);
            let b = conv_i64!(b);
            let diff = a + b;
            let diff_capped = call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Clamp".into(),
                    FnSig::new(&[Type::I64, Type::I64, Type::I64], Type::I64),
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
        Type::I16 => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a + b;
            let diff_capped = call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Clamp".into(),
                    FnSig::new(&[Type::I32, Type::I32, Type::I32], Type::I32),
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
        Type::I8 => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a + b;
            let diff_capped = call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Clamp".into(),
                    FnSig::new(&[Type::I32, Type::I32, Type::I32], Type::I32),
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
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
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
        Type::U128 | Type::U64 | Type::U32 | Type::U16 | Type::U8 | Type::USize => {
            let undeflow = crate::binop::cmp::lt_unchecked(a_ty, a.clone(), b.clone());
            let diff = crate::binop::sub_unchecked(a_ty, a_ty, ctx, a, b);
            let zero = crate::binop::checked::zero(a_ty);
            CILNode::select(a_type, zero, diff, undeflow)
        }
        Type::I32 => {
            let a = conv_i64!(a);
            let b = conv_i64!(b);
            let diff = a - b;
            let diff_capped = call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Clamp".into(),
                    FnSig::new(&[Type::I64, Type::I64, Type::I64], Type::I64),
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
        Type::I16 => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a - b;
            let diff_capped = call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Clamp".into(),
                    FnSig::new(&[Type::I32, Type::I32, Type::I32], Type::I32),
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
        Type::I8 => {
            let a = conv_i32!(a);
            let b = conv_i32!(b);
            let diff = a - b;
            let diff_capped = call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Clamp".into(),
                    FnSig::new(&[Type::I32, Type::I32, Type::I32], Type::I32),
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
