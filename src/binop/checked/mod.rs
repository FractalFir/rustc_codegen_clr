use crate::{assembly::MethodCompileCtx, casts};
use cilly::{
    and, call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_i64, conv_isize,
    conv_u64, conv_usize, field_desc::FieldDescriptor, gt, gt_un, ldc_i32, ldc_i64, ldc_u32,
    ldc_u64, lt, mul, or, size_of, DotnetTypeRef, FnSig, Type,
};
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};

pub fn result_tuple(tpe: Type, out_of_range: CILNode, val: CILNode) -> CILNode {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    CILNode::TemporaryLocal(Box::new((
        tuple.clone().into(),
        [
            CILRoot::SetField {
                addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                value: Box::new(out_of_range),
                desc: Box::new(FieldDescriptor::new(
                    tuple.clone(),
                    Type::Bool,
                    "Item2".into(),
                )),
            },
            CILRoot::SetField {
                addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                value: Box::new(val),
                desc: Box::new(FieldDescriptor::new(tuple.clone(), tpe, "Item1".into())),
            },
        ]
        .into(),
        CILNode::LoadTMPLocal,
    )))
    //CILNode::T
}
pub fn zero(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(0),
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(0),
        TyKind::Uint(UintTy::U32) => ldc_u32!(0),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(0),
        TyKind::Int(IntTy::I32) => ldc_i32!(0),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(0),
        TyKind::Uint(UintTy::U64) => ldc_u64!(0),
        TyKind::Int(IntTy::I64) => ldc_i64!(0),
        TyKind::Uint(UintTy::Usize) => conv_usize!(ldc_u32!(0)),
        TyKind::Int(IntTy::Isize) => conv_isize!(ldc_u32!(0)),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Implicit".into(),
                FnSig::new(&[Type::U32], Type::U128),
                true
            ),
            [ldc_u32!(0)]
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Implicit".into(),
                FnSig::new(&[Type::I32], Type::I128),
                true
            ),
            [ldc_u32!(0)]
        ),
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn min(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(u8::MIN),
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(u16::MIN),
        TyKind::Uint(UintTy::U32) => ldc_u32!(u32::MIN),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(i8::MIN),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(i16::MIN),
        TyKind::Int(IntTy::I32) => ldc_i32!(i32::MIN),
        TyKind::Uint(UintTy::U64) => ldc_u64!(u64::MIN),
        TyKind::Int(IntTy::I64) => ldc_i64!(i64::MIN),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::usize_type(),
                "get_MinValue".into(),
                FnSig::new(&[], Type::USize),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::isize_type(),
                "get_MinValue".into(),
                FnSig::new(&[], Type::ISize),
                true
            ),
            []
        ),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "get_MinValue".into(),
                FnSig::new(&[], Type::U128),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "get_MinValue".into(),
                FnSig::new(&[], Type::I128),
                true
            ),
            []
        ),
        _ => todo!("Can't get min of {ty:?}"),
    }
}
fn max(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(u8::MAX),
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(u16::MAX),
        TyKind::Uint(UintTy::U32) => ldc_u32!(u32::MAX),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(i8::MIN),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(i16::MIN),
        TyKind::Int(IntTy::I32) => ldc_i32!(i32::MAX),
        TyKind::Uint(UintTy::U64) => ldc_u64!(u64::MAX),
        TyKind::Int(IntTy::I64) => ldc_i64!(i64::MAX),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::usize_type(),
                "get_MaxValue".into(),
                FnSig::new(&[], Type::USize),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::isize_type(),
                "get_MaxValue".into(),
                FnSig::new(&[], Type::ISize),
                true
            ),
            []
        ),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "get_MaxValue".into(),
                FnSig::new(&[], Type::U128),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "get_MaxValue".into(),
                FnSig::new(&[], Type::I128),
                true
            ),
            []
        ),
        _ => todo!("Can't get max of {ty:?}"),
    }
}

pub fn mul<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    //(b > 0 && a < INT_MIN + b) || (b < 0 && a > INT_MAX + b);
    let tpe = ctx.type_from_cache(ty);
    let mul = super::mul_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());
    let ovf = match ty.kind() {
        // Work without promotions
        TyKind::Uint(UintTy::U8 | UintTy::U16) => gt_un!(mul.clone(), max(ty)),
        TyKind::Int(IntTy::I8 | IntTy::I16) => {
            or!(gt!(mul.clone(), max(ty)), lt!(mul.clone(), min(ty)))
        }
        // Works with 32 -> 64 size promotions
        TyKind::Uint(UintTy::U32) => {
            let mul = mul!(conv_u64!(ops_a.clone()), conv_u64!(ops_b.clone()));
            gt_un!(mul.clone(), conv_u64!(max(ty)))
        }
        TyKind::Int(IntTy::I32) => {
            let mul = mul!(conv_i64!(ops_a.clone()), conv_i64!(ops_b.clone()));
            or!(
                gt!(mul.clone(), conv_i64!(max(ty))),
                lt!(mul.clone(), conv_i64!(min(ty)))
            )
        }
        // Use 128 bit ints, not supported in mono.
        TyKind::Uint(UintTy::U64) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::U128),
                    true
                ),
                [
                    casts::int_to_int(Type::U64, &Type::U128, ops_a.clone()),
                    casts::int_to_int(Type::U64, &Type::U128, ops_b.clone())
                ]
            );
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::U64, &Type::U128, max(ty))
                ]
            )
        }
        TyKind::Int(IntTy::I64) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::I128),
                    true
                ),
                [
                    casts::int_to_int(Type::I64, &Type::I128, ops_a.clone()),
                    casts::int_to_int(Type::I64, &Type::I128, ops_b.clone())
                ]
            );
            let gt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::I64, &Type::I128, max(ty))
                ]
            );
            let lt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_LessThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::I64, &Type::I128, min(ty))
                ]
            );
            or!(gt, lt)
        }

        TyKind::Uint(UintTy::Usize) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::U128),
                    true
                ),
                [
                    casts::int_to_int(Type::USize, &Type::U128, ops_a.clone()),
                    casts::int_to_int(Type::USize, &Type::U128, ops_b.clone())
                ]
            );

            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::USize, &Type::U128, max(ty))
                ]
            )
        }
        TyKind::Int(IntTy::Isize) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::I128),
                    true
                ),
                [
                    casts::int_to_int(Type::ISize, &Type::I128, ops_a.clone()),
                    casts::int_to_int(Type::ISize, &Type::I128, ops_b.clone())
                ]
            );
            let gt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::ISize, &Type::I128, max(ty))
                ]
            );
            let lt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_LessThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::ISize, &Type::I128, min(ty))
                ]
            );
            or!(gt, lt)
        }
        _ => {
            eprintln!("WARINING: can't checked mul type {ty:?}");
            CILNode::LdFalse
        }
    };
    result_tuple(tpe, ovf, mul)
}
pub fn sub_signed<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    result_tuple(
        tpe,
        or!(
            and!(
                super::cmp::gt_unchecked(ty, ops_b.clone(), zero(ty)),
                super::cmp::lt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(ty, ty, ctx, min(ty), ops_b.clone())
                )
            ),
            and!(
                super::cmp::lt_unchecked(ty, ops_b.clone(), zero(ty)),
                super::cmp::gt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(ty, ty, ctx, max(ty), ops_b.clone())
                )
            )
        ),
        super::sub_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
    )
}
pub fn sub_unsigned<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    result_tuple(
        tpe,
        super::cmp::lt_unchecked(ty, ops_a.clone(), ops_b.clone()),
        super::sub_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
    )
}
pub fn add_unsigned<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    let res = super::add_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());

    result_tuple(
        tpe,
        super::cmp::lt_unchecked(
            ty,
            res.clone(),
            super::bit_or_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
        ),
        res,
    )
}
pub fn add_signed<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    let res = super::add_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());
    result_tuple(
        tpe,
        or!(
            and!(
                super::lt_unchecked(ty, ops_a.clone(), zero(ty)),
                and!(
                    super::lt_unchecked(ty, ops_b.clone(), zero(ty)),
                    super::gt_unchecked(ty, res.clone(), zero(ty))
                )
            ),
            and!(
                super::gt_unchecked(ty, ops_a.clone(), zero(ty)),
                and!(
                    super::gt_unchecked(ty, ops_b.clone(), zero(ty)),
                    super::lt_unchecked(ty, res.clone(), zero(ty))
                )
            )
        ),
        res,
    )
}
