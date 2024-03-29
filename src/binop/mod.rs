use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil::CallSite;
use crate::cil_tree::cil_node::CILNode;
use crate::function_sig::FnSig;
use crate::r#type::{DotnetTypeRef, TyCache};

pub mod bitop;
pub mod checked;
pub mod cmp;
pub mod shift;
use crate::{
    add, call, conv_u16, conv_u32, conv_u64, conv_u8, conv_usize, div, eq, ldc_i32, mul, rem,
    rem_un, size_of, sub,
};
use bitop::*;
pub use checked::binop_checked;
use cmp::*;
use shift::*;
/// Preforms an unchecked binary operation.
pub(crate) fn binop_unchecked<'tyctx>(
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
    match binop {
        BinOp::Add | BinOp::AddUnchecked => {
            add_unchecked(ty_a, ty_b, tyctx, &method_instance, tycache, ops_a, ops_b)
        }

        BinOp::Sub | BinOp::SubUnchecked => {
            sub_unchecked(ty_a, ty_b, tyctx, &method_instance, tycache, ops_a, ops_b)
        }
        BinOp::Ne => ne_unchecked(ty_a, ops_a, ops_b),
        BinOp::Eq => eq_unchecked(ty_a, ops_a, ops_b),
        BinOp::Lt => lt_unchecked(ty_a, ops_a, ops_b),
        BinOp::Gt => gt_unchecked(ty_a, ops_a, ops_b),
        BinOp::BitAnd => {
            bit_and_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b)
        }
        BinOp::BitOr => {
            bit_or_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b)
        }
        BinOp::BitXor => {
            bit_xor_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b)
        }
        BinOp::Rem => rem_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b),
        BinOp::Shl => shl_checked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b),
        BinOp::ShlUnchecked => {
            shl_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b)
        }
        BinOp::Shr => shr_checked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b),
        BinOp::ShrUnchecked => {
            shr_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b)
        }

        BinOp::Mul | BinOp::MulUnchecked => {
            mul_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b)
        }

        BinOp::Div => div_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b),

        BinOp::Ge => eq!(lt_unchecked(ty_a, ops_a, ops_b), ldc_i32!(0)),
        BinOp::Le => eq!(gt_unchecked(ty_a, ops_a, ops_b), ldc_i32!(0)),
        BinOp::Offset => {
            let pointed_ty = if let TyKind::RawPtr(inner,_) = ty_a.kind() {
                *inner
            } else {
                todo!("Can't offset pointer of type {ty_a:?}");
            };
            let pointed_ty = crate::utilis::monomorphize(&method_instance, pointed_ty, tyctx);
            let pointed_ty =
                Box::new(tycache.type_from_cache(pointed_ty, tyctx, Some(method_instance)));
            add!(ops_a, mul!(ops_b, conv_usize!(size_of!(pointed_ty))))
        }
    }
}
/// Preforms unchecked addition
pub fn add_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: &Instance<'tyctx>,
    tycache: &mut TyCache,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                call!(
                    CallSite::new(
                        Some(DotnetTypeRef::int_128()),
                        "op_Addition".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    ),
                    [ops_a, ops_b]
                )
            } else {
                add!(ops_a, ops_b)
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                call!(
                    CallSite::new(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Addition".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    ),
                    [ops_a, ops_b]
                )
            } else {
                match uint_ty {
                    UintTy::U8 => conv_u8!(add!(ops_a, ops_b)),
                    UintTy::U16 => conv_u16!(add!(ops_a, ops_b)),
                    UintTy::U32 => conv_u32!(add!(ops_a, ops_b)),
                    UintTy::U64 => conv_u64!(add!(ops_a, ops_b)),
                    _ => add!(ops_a, ops_b),
                }
            }
        }
        TyKind::Float(_) => add!(ops_a, ops_b),
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
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                call!(
                    CallSite::new(
                        Some(DotnetTypeRef::int_128()),
                        "op_Subtraction".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    ),
                    [ops_a, ops_b]
                )
            } else {
                sub!(ops_a, ops_b)
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
                let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
                call!(
                    CallSite::new(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Subtraction".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                        true,
                    ),
                    [ops_a, ops_b]
                )
            } else {
                sub!(ops_a, ops_b)
            }
        }
        TyKind::Float(_) => sub!(ops_a, ops_b),
        _ => todo!("can't sub numbers of types {ty_a} and {ty_b}"),
    }
}

fn rem_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tycache: &mut TyCache,
    method_instance: &Instance<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
            let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Modulus".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
            let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Modulus".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        TyKind::Int(_) => rem!(ops_a, ops_b),
        TyKind::Uint(_) => rem_un!(ops_a, ops_b),
        _ => rem!(ops_a, ops_b),
    }
}

fn mul_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tycache: &mut TyCache,
    method_instance: &Instance<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    operand_a: CILNode,
    operand_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
            let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Multiply".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
            let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Multiply".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        _ => mul!(operand_a, operand_b),
    }
}
fn div_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tycache: &mut TyCache,
    method_instance: &Instance<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    operand_a: CILNode,
    operand_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
            let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::int_128()),
                    "op_Division".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, Some(*method_instance));
            let ty_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Division".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        _ => div!(operand_a, operand_b),
    }
}
