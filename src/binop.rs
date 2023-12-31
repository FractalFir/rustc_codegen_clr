use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil::{CILOp, CallSite};
use crate::function_sig::FnSig;
use crate::r#type::{DotnetTypeRef, TyCache};
/// Preforms an unchecked binary operation.
pub(crate) fn binop_unchecked<'tyctx>(
    binop: BinOp,
    operand_a: &Operand<'tyctx>,
    operand_b: &Operand<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    let ops_a = crate::operand::handle_operand(operand_a, tyctx, method, method_instance, tycache);
    let ops_b = crate::operand::handle_operand(operand_b, tyctx, method, method_instance, tycache);
    let ty_a = operand_a.ty(&method.local_decls, tyctx);
    let ty_b = operand_b.ty(&method.local_decls, tyctx);
    match binop {
        BinOp::Add | BinOp::AddUnchecked => [
            ops_a,
            ops_b,
            add_unchecked(ty_a, ty_b, tyctx, &method_instance, tycache),
        ]
        .into_iter()
        .flatten()
        .collect(),
        BinOp::Sub | BinOp::SubUnchecked => [
            ops_a,
            ops_b,
            sub_unchecked(ty_a, ty_b, tyctx, &method_instance, tycache),
        ]
        .into_iter()
        .flatten()
        .collect(),
        BinOp::Ne => [ops_a, ops_b, ne_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Eq => [ops_a, ops_b, eq_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Lt => [ops_a, ops_b, lt_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Gt => [ops_a, ops_b, gt_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::BitAnd => [ops_a, ops_b, bit_and_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::BitOr => [ops_a, ops_b, bit_or_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::BitXor => [ops_a, ops_b, bit_xor_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Rem => [ops_a, ops_b, rem_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Shl | BinOp::ShlUnchecked => [ops_a, ops_b, shl_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Shr | BinOp::ShrUnchecked => [ops_a, ops_b, shr_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Mul | BinOp::MulUnchecked => [ops_a, ops_b, mul_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Div => [ops_a, ops_b, div_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Ge => [
            ops_a,
            ops_b,
            lt_unchecked(ty_a, ty_b),
            vec![CILOp::LdcI32(0), CILOp::Eq],
        ]
        .into_iter()
        .flatten()
        .collect(),
        BinOp::Le => [
            ops_a,
            ops_b,
            gt_unchecked(ty_a, ty_b),
            vec![CILOp::LdcI32(0), CILOp::Eq],
        ]
        .into_iter()
        .flatten()
        .collect(),
        BinOp::Offset => {
            let pointed_ty = if let TyKind::RawPtr(inner_and_mut) = ty_a.kind() {
                inner_and_mut.ty
            } else {
                todo!("Can't offset pointer of type {ty_a:?}");
            };
            let pointed_ty = crate::utilis::monomorphize(&method_instance, pointed_ty, tyctx);
            let pointed_ty =
                Box::new(tycache.type_from_cache(pointed_ty, tyctx, Some(method_instance)));
            [
                ops_a,
                ops_b,
                vec![CILOp::SizeOf(pointed_ty), CILOp::Mul, CILOp::Add],
            ]
            .into_iter()
            .flatten()
            .collect()
        } //_ => todo!("Unsupported bionp {binop:?}"),
    }
}
/// Preforms unchecked addition
fn add_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: &Instance<'tyctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                vec![CILOp::Call(
                    CallSite::new(
                        Some(DotnetTypeRef::int_128()),
                        "op_Addition".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    )
                    .into(),
                )]
            } else {
                vec![CILOp::Add]
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                vec![CILOp::Call(
                    CallSite::new(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Addition".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    )
                    .into(),
                )]
            } else {
                match uint_ty {
                    UintTy::U8 => vec![CILOp::Add, CILOp::ConvU8(false)],
                    UintTy::U16 => vec![CILOp::Add, CILOp::ConvU16(false)],
                    UintTy::U32 => vec![CILOp::Add, CILOp::ConvU32(false)],
                    UintTy::U64 => vec![CILOp::Add, CILOp::ConvU64(false)],
                    _ => vec![CILOp::Add],
                }
            }
        }
        TyKind::Float(_) => vec![CILOp::Add],
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
/// Preforms unchecked subtraction
fn sub_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: &Instance<'tyctx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                vec![CILOp::Call(
                    CallSite::new(
                        Some(DotnetTypeRef::int_128()),
                        "op_Subtraction".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    )
                    .into(),
                )]
            } else {
                vec![CILOp::Sub]
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                vec![CILOp::Call(
                    CallSite::new(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Subtraction".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    )
                    .into(),
                )]
            } else {
                vec![CILOp::Sub]
            }
        }
        TyKind::Float(_) => vec![CILOp::Sub],
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
fn ne_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Eq, CILOp::LdcI32(0), CILOp::Eq]
}
fn eq_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Eq]
}
fn lt_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Lt]
}
fn gt_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Gt]
}
fn bit_and_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::And]
}
fn bit_or_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Or]
}
fn bit_xor_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::XOr]
}
fn rem_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Rem]
}
fn shr_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Shr]
}
fn shl_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Shl]
}
fn mul_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Mul]
}
fn div_unchecked<'tyctx>(_ty_a: Ty<'tyctx>, _ty_b: Ty<'tyctx>) -> Vec<CILOp> {
    vec![CILOp::Div]
}
