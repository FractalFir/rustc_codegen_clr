use crate::{assembly::MethodCompileCtx, casts};
use cilly::{
    and, call,
    call_site::CallSite,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u64, conv_usize, gt, gt_un, ldc_i32,
    ldc_i64, ldc_u32, ldc_u64, lt, mul, or,
    v2::{Assembly, ClassRef, FieldDesc, FnSig, Int},
    Type,
};
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};

pub fn result_tuple(tpe: Type, out_of_range: CILNode, val: CILNode, asm: &mut Assembly) -> CILNode {
    let tuple = crate::r#type::simple_tuple(&[tpe, Type::Bool], asm);
    let item2 = asm.alloc_string("Item2");
    let item1 = asm.alloc_string("Item1");
    CILNode::TemporaryLocal(Box::new((
        tuple.into(),
        [
            CILRoot::SetField {
                addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                value: Box::new(out_of_range),
                desc: asm.alloc_field(FieldDesc::new(tuple, item2, Type::Bool)),
            },
            CILRoot::SetField {
                addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                value: Box::new(val),
                desc: asm.alloc_field(FieldDesc::new(tuple, item1, tpe)),
            },
        ]
        .into(),
        CILNode::LoadTMPLocal,
    )))
    //CILNode::T
}
pub fn zero(ty: Ty, asm: &mut Assembly) -> CILNode {
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
                ClassRef::uint_128(asm),
                "op_Implicit".into(),
                FnSig::new([Type::Int(Int::U32)].into(), Type::Int(Int::U128)),
                true
            ),
            [ldc_u32!(0)]
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "op_Implicit".into(),
                FnSig::new([Type::Int(Int::I32)].into(), Type::Int(Int::I128)),
                true
            ),
            [ldc_i32!(0)]
        ),
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn min(ty: Ty, asm: &mut Assembly) -> CILNode {
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
                ClassRef::usize_type(asm),
                "get_MinValue".into(),
                FnSig::new([].into(), Type::Int(Int::USize)),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new_extern(
                ClassRef::isize_type(asm),
                "get_MinValue".into(),
                FnSig::new([].into(), Type::Int(Int::ISize)),
                true
            ),
            []
        ),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "get_MinValue".into(),
                FnSig::new([].into(), Type::Int(Int::U128)),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "get_MinValue".into(),
                FnSig::new([].into(), Type::Int(Int::I128)),
                true
            ),
            []
        ),
        _ => todo!("Can't get min of {ty:?}"),
    }
}
fn max(ty: Ty, asm: &mut Assembly) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(u8::MAX),
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(u16::MAX),
        TyKind::Uint(UintTy::U32) => ldc_u32!(u32::MAX),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(i8::MAX),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(i16::MAX),
        TyKind::Int(IntTy::I32) => ldc_i32!(i32::MAX),
        TyKind::Uint(UintTy::U64) => ldc_u64!(u64::MAX),
        TyKind::Int(IntTy::I64) => ldc_i64!(i64::MAX),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new_extern(
                ClassRef::usize_type(asm),
                "get_MaxValue".into(),
                FnSig::new([].into(), Type::Int(Int::USize)),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new_extern(
                ClassRef::isize_type(asm),
                "get_MaxValue".into(),
                FnSig::new([].into(), Type::Int(Int::ISize)),
                true
            ),
            []
        ),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                ClassRef::uint_128(asm),
                "get_MaxValue".into(),
                FnSig::new([].into(), Type::Int(Int::U128)),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                ClassRef::int_128(asm),
                "get_MaxValue".into(),
                FnSig::new([].into(), Type::Int(Int::I128)),
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
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    //(b > 0 && a < INT_MIN + b) || (b < 0 && a > INT_MAX + b);
    let tpe = ctx.type_from_cache(ty);
    let mul = super::mul_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());
    let ovf = match ty.kind() {
        // Work without promotions
        TyKind::Uint(UintTy::U8 | UintTy::U16) => gt_un!(mul.clone(), max(ty, ctx.asm_mut())),
        TyKind::Int(IntTy::I8 | IntTy::I16) => {
            or!(
                gt!(mul.clone(), max(ty, ctx.asm_mut())),
                lt!(mul.clone(), min(ty, ctx.asm_mut()))
            )
        }
        // Works with 32 -> 64 size promotions
        TyKind::Uint(UintTy::U32) => {
            let mul = mul!(conv_u64!(ops_a.clone()), conv_u64!(ops_b.clone()));
            gt_un!(mul.clone(), conv_u64!(max(ty, ctx.asm_mut())))
        }
        TyKind::Int(IntTy::I32) => {
            let mul = mul!(conv_i64!(ops_a.clone()), conv_i64!(ops_b.clone()));
            or!(
                gt!(mul.clone(), conv_i64!(max(ty, ctx.asm_mut()))),
                lt!(mul.clone(), conv_i64!(min(ty, ctx.asm_mut())))
            )
        }
        // Use 128 bit ints, not supported in mono.
        TyKind::Uint(UintTy::U64) => {
            let mul = call!(
                CallSite::new_extern(
                    ClassRef::uint_128(ctx.asm_mut()),
                    "op_Multiply".into(),
                    FnSig::new(
                        [Type::Int(Int::U128), Type::Int(Int::U128)].into(),
                        Type::Int(Int::U128)
                    ),
                    true
                ),
                [
                    casts::int_to_int(
                        Type::Int(Int::U64),
                        Type::Int(Int::U128),
                        ops_a.clone(),
                        ctx.asm_mut()
                    ),
                    casts::int_to_int(
                        Type::Int(Int::U64),
                        Type::Int(Int::U128),
                        ops_b.clone(),
                        ctx.asm_mut()
                    )
                ]
            );
            call!(
                CallSite::new_extern(
                    ClassRef::uint_128(ctx.asm_mut()),
                    "op_GreaterThan".into(),
                    FnSig::new(
                        [Type::Int(Int::U128), Type::Int(Int::U128)].into(),
                        Type::Bool
                    ),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::U64),
                        Type::Int(Int::U128),
                        max(ty, ctx.asm_mut()),
                        ctx.asm_mut()
                    )
                ]
            )
        }
        TyKind::Int(IntTy::I64) => {
            let mul = call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx.asm_mut()),
                    "op_Multiply".into(),
                    FnSig::new(
                        [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                        Type::Int(Int::I128)
                    ),
                    true
                ),
                [
                    casts::int_to_int(
                        Type::Int(Int::I64),
                        Type::Int(Int::I128),
                        ops_a.clone(),
                        ctx.asm_mut()
                    ),
                    casts::int_to_int(
                        Type::Int(Int::I64),
                        Type::Int(Int::I128),
                        ops_b.clone(),
                        ctx.asm_mut()
                    )
                ]
            );
            let gt = call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx.asm_mut()),
                    "op_GreaterThan".into(),
                    FnSig::new(
                        [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                        Type::Bool
                    ),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::I64),
                        Type::Int(Int::I128),
                        max(ty, ctx.asm_mut()),
                        ctx.asm_mut()
                    )
                ]
            );
            let lt = call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx.asm_mut()),
                    "op_LessThan".into(),
                    FnSig::new(
                        [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                        Type::Bool
                    ),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::I64),
                        Type::Int(Int::I128),
                        min(ty, ctx.asm_mut()),
                        ctx.asm_mut()
                    )
                ]
            );
            or!(gt, lt)
        }

        TyKind::Uint(UintTy::Usize) => {
            let mul = call!(
                CallSite::new_extern(
                    ClassRef::uint_128(ctx.asm_mut()),
                    "op_Multiply".into(),
                    FnSig::new(
                        [Type::Int(Int::U128), Type::Int(Int::U128)].into(),
                        Type::Int(Int::U128)
                    ),
                    true
                ),
                [
                    casts::int_to_int(
                        Type::Int(Int::USize),
                        Type::Int(Int::U128),
                        ops_a.clone(),
                        ctx.asm_mut()
                    ),
                    casts::int_to_int(
                        Type::Int(Int::USize),
                        Type::Int(Int::U128),
                        ops_b.clone(),
                        ctx.asm_mut()
                    )
                ]
            );

            call!(
                CallSite::new_extern(
                    ClassRef::uint_128(ctx.asm_mut()),
                    "op_GreaterThan".into(),
                    FnSig::new(
                        [Type::Int(Int::U128), Type::Int(Int::U128)].into(),
                        Type::Bool
                    ),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::USize),
                        Type::Int(Int::U128),
                        max(ty, ctx.asm_mut()),
                        ctx.asm_mut()
                    )
                ]
            )
        }
        TyKind::Int(IntTy::Isize) => {
            let mul = call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx.asm_mut()),
                    "op_Multiply".into(),
                    FnSig::new(
                        [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                        Type::Int(Int::I128)
                    ),
                    true
                ),
                [
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        ops_a.clone(),
                        ctx.asm_mut()
                    ),
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        ops_b.clone(),
                        ctx.asm_mut()
                    )
                ]
            );
            let gt = call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx.asm_mut()),
                    "op_GreaterThan".into(),
                    FnSig::new(
                        [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                        Type::Bool
                    ),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        max(ty, ctx.asm_mut()),
                        ctx.asm_mut()
                    )
                ]
            );
            let lt = call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx.asm_mut()),
                    "op_LessThan".into(),
                    FnSig::new(
                        [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                        Type::Bool
                    ),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        min(ty, ctx.asm_mut()),
                        ctx.asm_mut()
                    )
                ]
            );
            or!(gt, lt)
        }
        _ => {
            eprintln!("WARINING: can't checked mul type {ty:?}");
            CILNode::LdFalse
        }
    };
    result_tuple(tpe, ovf, mul, ctx.asm_mut())
}
pub fn sub_signed<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    let min = min(ty, ctx.asm_mut());
    let max = max(ty, ctx.asm_mut());
    result_tuple(
        tpe,
        or!(
            and!(
                super::cmp::gt_unchecked(ty, ops_b.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut()),
                super::cmp::lt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(ty, ty, ctx, min, ops_b.clone()),
                    ctx.asm_mut()
                )
            ),
            and!(
                super::cmp::lt_unchecked(ty, ops_b.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut()),
                super::cmp::gt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(ty, ty, ctx, max, ops_b.clone()),
                    ctx.asm_mut()
                )
            )
        ),
        super::sub_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
        ctx.asm_mut(),
    )
}
pub fn sub_unsigned<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    result_tuple(
        tpe,
        super::cmp::lt_unchecked(ty, ops_a.clone(), ops_b.clone(), ctx.asm_mut()),
        super::sub_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
        ctx.asm_mut(),
    )
}
pub fn add_unsigned<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    let res = super::add_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());

    result_tuple(
        tpe,
        super::cmp::lt_unchecked(
            ty,
            res.clone(),
            super::bit_or_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
            ctx.asm_mut(),
        ),
        res,
        ctx.asm_mut(),
    )
}
pub fn add_signed<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    match ty.kind() {
        TyKind::Int(IntTy::I8) => {
            let sum = conv_i16!(ops_a.clone()) + conv_i16!(ops_b.clone());
            return result_tuple(
                tpe,
                or!(
                    lt!(sum.clone(), conv_i16!(ldc_i32!(i8::MIN.into()))),
                    gt!(sum.clone(), conv_i16!(ldc_i32!(i8::MAX.into())))
                ),
                conv_i8!(sum),
                ctx.asm_mut(),
            );
        }
        TyKind::Int(IntTy::I16) => {
            let sum = conv_i32!(ops_a.clone()) + conv_i32!(ops_b.clone());
            return result_tuple(
                tpe,
                or!(
                    lt!(sum.clone(), (ldc_i32!(i16::MIN.into()))),
                    gt!(sum.clone(), (ldc_i32!(i16::MAX.into())))
                ),
                conv_i16!(sum),
                ctx.asm_mut(),
            );
        }
        TyKind::Int(IntTy::I32) => {
            let sum = conv_i64!(ops_a.clone()) + conv_i64!(ops_b.clone());
            let out_of_range = or!(
                lt!(sum.clone(), conv_i64!(ldc_i32!(i32::MIN))),
                gt!(sum.clone(), conv_i64!(ldc_i32!(i32::MAX)))
            );
            return result_tuple(tpe, out_of_range, conv_i32!(sum), ctx.asm_mut());
        }
        _ => (),
    }
    let res = super::add_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());
    result_tuple(
        tpe,
        or!(
            and!(
                super::lt_unchecked(ty, ops_a.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut()),
                and!(
                    super::lt_unchecked(ty, ops_b.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut()),
                    super::gt_unchecked(ty, res.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut())
                )
            ),
            and!(
                super::gt_unchecked(ty, ops_a.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut()),
                and!(
                    super::gt_unchecked(ty, ops_b.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut()),
                    super::lt_unchecked(ty, res.clone(), zero(ty, ctx.asm_mut()), ctx.asm_mut())
                )
            )
        ),
        res,
        ctx.asm_mut(),
    )
}
