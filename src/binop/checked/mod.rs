use crate::{
    call, cil_tree::cil_node::CILNode, cil_tree::cil_root::CILRoot, conv_usize, r#type::TyCache,
    and, casts, conv_i64, conv_isize, conv_u64, gt, gt_un, ldc_u32, lt, or,
    size_of,
    cil::{CallSite, FieldDescriptor},
    function_sig::FnSig,
    r#type::{DotnetTypeRef, Type},
};
use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

pub fn binop_checked<'tyctx>(
    binop: BinOp,
    operand_a: &Operand<'tyctx>,
    operand_b: &Operand<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let ops_a = crate::operand::handle_operand(operand_a, tyctx, method, method_instance, tycache);
    let ops_b = crate::operand::handle_operand(operand_b, tyctx, method, method_instance, tycache);
    let ty_a = operand_a.ty(&method.local_decls, tyctx);
    let ty_b = operand_b.ty(&method.local_decls, tyctx);
    debug_assert_eq!(ty_a, ty_b);
    match binop {
        BinOp::Sub => if ty_a.is_signed() {
            sub_signed(ops_a, ops_b, ty_a, tyctx, method_instance, tycache) }else{
            sub_unsigned(ops_a, ops_b, ty_a, tyctx, method_instance, tycache)
            }
        BinOp::Add => if ty_a.is_signed() {
            add_signed(ops_a, ops_b, ty_a, tyctx, method_instance, tycache)} else {
            add_unsigned(ops_a, ops_b, ty_a, tyctx, method_instance, tycache)
            }
        ,
        // TODO: Chekced multiplcation is NOT checked
        BinOp::Mul => mul(ops_a, ops_b, ty_a, tyctx, method_instance, tycache),
        _ => todo!("Can't handle checked binop {binop:?}"),
    }
}
pub fn result_tuple(tpe: Type, out_of_range: CILNode, val: CILNode) -> CILNode {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    CILNode::TemporaryLocal(Box::new((
        tuple.clone().into(),
        [
            CILRoot::SetField {
                addr: CILNode::LoadAddresOfTMPLocal,
                value: out_of_range,
                desc: FieldDescriptor::new(tuple.clone(), Type::Bool, "Item2".into()),
            },
            CILRoot::SetField {
                addr: CILNode::LoadAddresOfTMPLocal,
                value: val,
                desc: FieldDescriptor::new(tuple.clone(), tpe, "Item1".into()),
            },
        ]
        .into(),
        CILNode::LoadTMPLocal,
    )))
    //CILNode::T
}
fn zero(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8 | UintTy::U16 | UintTy::U32) => crate::ldc_u32!(0),
        TyKind::Int(IntTy::I8 | IntTy::I16 | IntTy::I32) => crate::ldc_i32!(0),
        TyKind::Uint(UintTy::U64) => crate::ldc_u64!(0),
        TyKind::Int(IntTy::I64) => crate::ldc_i64!(0),
        TyKind::Uint(UintTy::Usize) => conv_usize!(size_of!(Type::USize)),
        TyKind::Int(IntTy::Isize) => conv_isize!(size_of!(Type::USize)),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "op_Implicit".into(),
                FnSig::new(&[Type::U32], &Type::U128),
                true
            ),
            [ldc_u32!(0)]
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "op_Implicit".into(),
                FnSig::new(&[Type::I32], &Type::I128),
                true
            ),
            [ldc_u32!(0)]
        ),
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn min(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => crate::ldc_u32!(u32::from(u8::MIN)),
        TyKind::Uint(UintTy::U16) => crate::ldc_u32!(u32::from(u16::MIN)),
        TyKind::Uint(UintTy::U32) => crate::ldc_u32!(u32::MIN),
        TyKind::Int(IntTy::I8) => crate::ldc_i32!(i32::from(i8::MIN)),
        TyKind::Int(IntTy::I16) => crate::ldc_i32!(i32::from(i16::MIN)),
        TyKind::Int(IntTy::I32) => crate::ldc_i32!(i32::MIN),
        TyKind::Uint(UintTy::U64) => crate::ldc_u64!(u64::MIN),
        TyKind::Int(IntTy::I64) => crate::ldc_i64!(i64::MIN),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::usize_type(),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::isize_type(),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::U128),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::I128),
                true
            ),
            []
        ),
        _ => todo!("Can't get min of {ty:?}"),
    }
}
fn max(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => crate::ldc_u32!(u32::from(u8::MAX)),
        TyKind::Uint(UintTy::U16) => crate::ldc_u32!(u32::from(u16::MAX)),
        TyKind::Uint(UintTy::U32) => crate::ldc_u32!(u32::MAX),
        TyKind::Int(IntTy::I8) => crate::ldc_i32!(i32::from(i8::MAX)),
        TyKind::Int(IntTy::I16) => crate::ldc_i32!(i32::from(i16::MAX)),
        TyKind::Int(IntTy::I32) => crate::ldc_i32!(i32::MAX),
        TyKind::Uint(UintTy::U64) => crate::ldc_u64!(u64::MAX),
        TyKind::Int(IntTy::I64) => crate::ldc_i64!(i64::MAX),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::usize_type(),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new_extern(
                DotnetTypeRef::isize_type(),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::uint_128(),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::U128),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::new_extern(
                DotnetTypeRef::int_128(),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::I128),
                true
            ),
            []
        ),
        _ => todo!("Can't get max of {ty:?}"),
    }
}
/*
result_tuple(
            crate::r#type::simple_tuple(&[
                tycache
                    .type_from_cache(ty_a, tyctx, Some(method_instance))
                    .clone(),
                Type::Bool,
            ])
            .into(),
            ldc_i32!(0),
            super::mul_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b),
        ), */
pub fn mul<'tyctx>(
    ops_a: CILNode,
    ops_b: CILNode,
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    //(b > 0 && a < INT_MIN + b) || (b < 0 && a > INT_MAX + b);
    let tpe = tycache.type_from_cache(ty, tyctx, Some(method_instance));
    let mul = super::mul_unchecked(
        ty,
        ty,
        tycache,
        &method_instance,
        tyctx,
        ops_a.clone(),
        ops_b.clone(),
    );
    let ovf = match ty.kind() {
        // Work without promotions
        TyKind::Uint(UintTy::U8 | UintTy::U16) => gt_un!(mul.clone(), max(ty)),
        TyKind::Int(IntTy::I8 | IntTy::I16) => {
            or!(gt!(mul.clone(), max(ty)), lt!(mul.clone(), min(ty)))
        }
        // Works with 32 -> 64 size promotions
        TyKind::Uint(UintTy::U32) => {
            let mul = crate::mul!(conv_u64!(ops_a), conv_u64!(ops_b));
            gt_un!(mul.clone(), conv_u64!(max(ty)))
        }
        TyKind::Int(IntTy::I32) => {
            let mul = crate::mul!(conv_i64!(ops_a), conv_i64!(ops_b));
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
                    FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                    true
                ),
                [
                    casts::int_to_int(Type::U64, Type::U128, ops_a),
                    casts::int_to_int(Type::U64, Type::U128, ops_b)
                ]
            );
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::U128, Type::U128], &Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::U64, Type::U128, max(ty))
                ]
            )
        }
        TyKind::Int(IntTy::I64) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                    true
                ),
                [
                    casts::int_to_int(Type::I64, Type::I128, ops_a),
                    casts::int_to_int(Type::I64, Type::I128, ops_b)
                ]
            );
            let gt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], &Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::I64, Type::I128, max(ty))
                ]
            );
            let lt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_LessThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], &Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::I64, Type::I128, min(ty))
                ]
            );
            or!(gt, lt)
        }

        TyKind::Uint(UintTy::Usize) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                    true
                ),
                [
                    casts::int_to_int(Type::USize, Type::U128, ops_a),
                    casts::int_to_int(Type::USize, Type::U128, ops_b)
                ]
            );

            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::U128, Type::U128], &Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::USize, Type::U128, max(ty))
                ]
            )
        }
        TyKind::Int(IntTy::Isize) => {
            let mul = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                    true
                ),
                [
                    casts::int_to_int(Type::ISize, Type::I128, ops_a),
                    casts::int_to_int(Type::ISize, Type::I128, ops_b)
                ]
            );
            let gt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], &Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::ISize, Type::I128, max(ty))
                ]
            );
            let lt = call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_LessThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], &Type::Bool),
                    true
                ),
                [
                    mul.clone(),
                    casts::int_to_int(Type::ISize, Type::I128, min(ty))
                ]
            );
            or!(gt, lt)
        }
        _ => {
            eprintln!("WARINING: can't checked mul type {ty:?}");
            ldc_u32!(0)
        }
    };
    result_tuple(tpe, ovf, mul)
}
pub fn sub_signed<'tyctx>(
    ops_a: CILNode,
    ops_b: CILNode,
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    //(b > 0 && a < INT_MIN + b) || (b < 0 && a > INT_MAX + b);
    let tpe = tycache.type_from_cache(ty, tyctx, Some(method_instance));
    result_tuple(
        tpe,
        or!(
            and!(
                super::cmp::gt_unchecked(ty, ops_b.clone(), zero(ty)),
                super::cmp::lt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(
                        ty,
                        ty,
                        tyctx,
                        &method_instance,
                        tycache,
                        min(ty),
                        ops_b.clone()
                    )
                )
            ),
            and!(
                super::cmp::lt_unchecked(ty, ops_b.clone(), zero(ty)),
                super::cmp::gt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(
                        ty,
                        ty,
                        tyctx,
                        &method_instance,
                        tycache,
                        max(ty),
                        ops_b.clone()
                    )
                )
            )
        ),
        super::sub_unchecked(ty, ty, tyctx, &method_instance, tycache, ops_a, ops_b),
    )
}
pub fn sub_unsigned<'tyctx>(
    ops_a: CILNode,
    ops_b: CILNode,
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let tpe = tycache.type_from_cache(ty, tyctx, Some(method_instance));
    result_tuple(
        tpe,
        super::cmp::lt_unchecked(ty, ops_a.clone(), ops_b.clone()),
        super::sub_unchecked(ty, ty, tyctx, &method_instance, tycache, ops_a, ops_b),
    )
}
pub fn add_unsigned<'tyctx>(
    ops_a: CILNode,
    ops_b: CILNode,
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let tpe = tycache.type_from_cache(ty, tyctx, Some(method_instance));
    let res = super::add_unchecked(
        ty,
        ty,
        tyctx,
        &method_instance,
        tycache,
        ops_a.clone(),
        ops_b.clone(),
    );

    result_tuple(
        tpe,
        super::cmp::lt_unchecked(
            ty,
            res.clone(),
            super::bit_or_unchecked(
                ty,
                ty,
                tycache,
                &method_instance,
                tyctx,
                ops_a.clone(),
                ops_b.clone(),
            ),
        ),
        res,
    )
}
pub fn add_signed<'tyctx>(
    ops_a: CILNode,
    ops_b: CILNode,
    ty: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let tpe = tycache.type_from_cache(ty, tyctx, Some(method_instance));
    let res = super::add_unchecked(
        ty,
        ty,
        tyctx,
        &method_instance,
        tycache,
        ops_a.clone(),
        ops_b.clone(),
    );
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
                super::gt_unchecked(ty, ops_a, zero(ty)),
                and!(
                    super::gt_unchecked(ty, ops_b, zero(ty)),
                    super::lt_unchecked(ty, res.clone(), zero(ty))
                )
            )
        ),
        res,
    )
}
