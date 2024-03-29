use crate::cil::{CallSite, FieldDescriptor};
use crate::function_sig::FnSig;
use crate::r#type::{DotnetTypeRef, Type};
use crate::{
    call, cil_tree::cil_node::CILNode, cil_tree::cil_root::CILRoot, conv_usize, r#type::TyCache, size_of,
};
use crate::{and, conv_isize, ldc_u32, or};
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
        BinOp::Sub => match ty_a.is_signed() {
            true => sub_signed(ops_a, ops_b, ty_a, tyctx, method_instance, tycache),
            false => sub_unsigned(ops_a, ops_b, ty_a, tyctx, method_instance, tycache),
        },
        BinOp::Add => match ty_a.is_signed() {
            true => add_signed(ops_a, ops_b, ty_a, tyctx, method_instance, tycache),
            false => add_unsigned(ops_a, ops_b, ty_a, tyctx, method_instance, tycache),
        },
        // TODO: Chekced multiplcation is NOT checked
        BinOp::Mul => mul(ops_a, ops_b, ty_a, tyctx, method_instance, tycache),
        _ => todo!("Can't handle checked binop {binop:?}"),
    }
}
pub fn result_tuple<'tyctx>(tpe: Type, out_of_range: CILNode, val: CILNode) -> CILNode {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    CILNode::TemporaryLocal(Box::new((
        tuple.clone().into(),
        [
            CILRoot::SetField {
                addr: CILNode::LoadAddresOfTMPLocal,
                value: out_of_range,
                desc: FieldDescriptor::new(tuple.clone(), Type::Bool, "Item2".into()),
            }
            .into(),
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
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn min(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => crate::ldc_u32!(u8::MIN as u32),
        TyKind::Uint(UintTy::U16) => crate::ldc_u32!(u16::MIN as u32),
        TyKind::Uint(UintTy::U32) => crate::ldc_u32!(u32::MIN as u32),
        TyKind::Int(IntTy::I8) => crate::ldc_i32!(i8::MIN as i32),
        TyKind::Int(IntTy::I16) => crate::ldc_i32!(i16::MIN as i32),
        TyKind::Int(IntTy::I32) => crate::ldc_i32!(i32::MIN as i32),
        TyKind::Uint(UintTy::U64) => crate::ldc_u64!(u64::MIN as u64),
        TyKind::Int(IntTy::I64) => crate::ldc_i64!(i64::MIN as i64),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new(
                Some(DotnetTypeRef::usize_type()),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new(
                Some(DotnetTypeRef::usize_type()),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn max(ty: Ty) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => crate::ldc_u32!(u8::MAX as u32),
        TyKind::Uint(UintTy::U16) => crate::ldc_u32!(u16::MAX as u32),
        TyKind::Uint(UintTy::U32) => crate::ldc_u32!(u32::MAX as u32),
        TyKind::Int(IntTy::I8) => crate::ldc_i32!(i8::MAX as i32),
        TyKind::Int(IntTy::I16) => crate::ldc_i32!(i16::MAX as i32),
        TyKind::Int(IntTy::I32) => crate::ldc_i32!(i32::MAX as i32),
        TyKind::Uint(UintTy::U64) => crate::ldc_u64!(u64::MAX as u64),
        TyKind::Int(IntTy::I64) => crate::ldc_i64!(i64::MAX as i64),
        TyKind::Uint(UintTy::Usize) => call!(
            CallSite::new(
                Some(DotnetTypeRef::usize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        TyKind::Int(IntTy::Isize) => call!(
            CallSite::new(
                Some(DotnetTypeRef::usize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true
            ),
            []
        ),
        _ => todo!("Can't get zero of {ty:?}"),
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
    result_tuple(
        tpe,
        ldc_u32!(0),
        super::mul_unchecked(ty, ty, tycache, &method_instance, tyctx, ops_a, ops_b),
    )
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
        or!(
            super::cmp::lt_unchecked(ty, res.clone(), ops_a.clone()),
            super::cmp::lt_unchecked(ty, res.clone(), ops_b.clone())
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
