use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil::CallSite;
use crate::cil_tree::cil_node::CILNode;
use crate::function_sig::FnSig;
use crate::r#type::{DotnetTypeRef, TyCache, Type};

use crate::{and, call, or, xor};
pub fn bit_and_unchecked<'tyctx>(
    ty_a: Ty<'tyctx>,
    ty_b: Ty<'tyctx>,
    tycache: &mut TyCache,
    method_instance: &Instance<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    operand_a: CILNode,
    operand_b: CILNode,
) -> CILNode {
    let type_b = tycache.type_from_cache(ty_b, tyctx, Some(*method_instance));
    match ty_a.kind() {
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::boxed(
                DotnetTypeRef::uint_128().into(),
                "op_BitwiseAnd".into(),
                FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            ),
            [
                operand_a,
                crate::casts::int_to_int(type_b.clone(), Type::U128, operand_b)
            ]
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::boxed(
                DotnetTypeRef::int_128().into(),
                "op_BitwiseAnd".into(),
                FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            ),
            [
                operand_a,
                crate::casts::int_to_int(type_b.clone(), Type::I128, operand_b)
            ]
        ),
        _ => and!(operand_a, operand_b),
    }
}
pub fn bit_or_unchecked<'tyctx>(
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
                    "op_BitwiseOr".into(),
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
                    "op_BitwiseOr".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        _ => or!(operand_a, operand_b),
    }
}
pub fn bit_xor_unchecked<'tyctx>(
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
                    "op_ExclusiveOr".into(),
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
                    "op_ExclusiveOr".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], &ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        _ => xor!(ops_a, ops_b),
    }
}
