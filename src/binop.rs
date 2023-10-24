use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil_op::CILOp;
/// Preforms an unchecked binary operation.
pub(crate) fn binop_unchecked<'tcx>(
    binop: BinOp,
    operand_a: &Operand<'tcx>,
    operand_b: &Operand<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let ops_a = crate::operand::handle_operand(operand_a, tcx, method, method_instance);
    let ops_b = crate::operand::handle_operand(operand_b, tcx, method, method_instance);
    let ty_a = operand_a.ty(&method.local_decls, tcx);
    let ty_b = operand_b.ty(&method.local_decls, tcx);
    match binop {
        BinOp::Add | BinOp::AddUnchecked => [ops_a, ops_b, add_unchecked(ty_a, ty_b)]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Sub | BinOp::SubUnchecked => [ops_a, ops_b, sub_unchecked(ty_a, ty_b)]
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
            let pointed_ty = crate::utilis::monomorphize(&method_instance, pointed_ty, tcx);
            let pointed_ty = Box::new(crate::r#type::Type::from_ty(
                pointed_ty,
                tcx,
                &method_instance,
            ));
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
fn add_unchecked<'tcx>(ty_a: Ty<'tcx>, ty_b: Ty<'tcx>) -> Vec<CILOp> {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                todo!("Can't add 128 bit intigers yet!");
            } else {
                vec![CILOp::Add]
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                todo!("Can't add 128 bit intigers yet!");
            } else {
                vec![CILOp::Add]
            }
        }
        TyKind::Float(_) => vec![CILOp::Add],
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
/// Preforms unchecked subtraction
fn sub_unchecked<'tcx>(ty_a: Ty<'tcx>, ty_b: Ty<'tcx>) -> Vec<CILOp> {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                todo!("Can't add 128 bit intigers yet!");
            } else {
                vec![CILOp::Sub]
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                todo!("Can't add 128 bit intigers yet!");
            } else {
                vec![CILOp::Sub]
            }
        }
        TyKind::Float(_) => vec![CILOp::Sub],
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
fn ne_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Eq, CILOp::LdcI32(0), CILOp::Eq]
}
fn eq_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Eq]
}
fn lt_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Lt]
}
fn gt_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Lt]
}
fn bit_and_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::And]
}
fn bit_or_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Or]
}
fn bit_xor_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::XOr]
}
fn rem_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Rem]
}
fn shr_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Shr]
}
fn shl_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Shl]
}
fn mul_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Mul]
}
fn div_unchecked<'tcx>(_ty_a: Ty<'tcx>, _ty_b: Ty<'tcx>) -> Vec<CILOp> {
    vec![CILOp::Div]
}
