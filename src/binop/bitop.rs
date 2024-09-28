use crate::assembly::MethodCompileCtx;
use cilly::{
    and, call,
    call_site::CallSite,
    cil_node::CILNode,
    or,
    v2::{ClassRef, FnSig, Int},
    xor, Type,
};
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};

pub fn bit_and_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    ty_b: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    operand_a: CILNode,
    operand_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(ty_b);
    match ty_a.kind() {
        TyKind::Uint(UintTy::U128) => call!(
            CallSite::boxed(
                ClassRef::uint_128(ctx).into(),
                "op_BitwiseAnd".into(),
                FnSig::new(
                    [Type::Int(Int::U128), Type::Int(Int::U128)].into(),
                    Type::Int(Int::U128)
                ),
                true,
            ),
            [
                operand_a,
                crate::casts::int_to_int(type_b, Type::Int(Int::U128), operand_b, ctx)
            ]
        ),
        TyKind::Int(IntTy::I128) => call!(
            CallSite::boxed(
                ClassRef::int_128(ctx).into(),
                "op_BitwiseAnd".into(),
                FnSig::new(
                    [Type::Int(Int::I128), Type::Int(Int::I128)].into(),
                    Type::Int(Int::I128)
                ),
                true,
            ),
            [
                operand_a,
                crate::casts::int_to_int(type_b, Type::Int(Int::I128), operand_b, ctx)
            ]
        ),
        _ => and!(operand_a, operand_b),
    }
}
pub fn bit_or_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    ty_b: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    operand_a: CILNode,
    operand_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let ty_a = ctx.type_from_cache(ty_a);
            let ty_b = ctx.type_from_cache(ty_b);
            call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx),
                    "op_BitwiseOr".into(),
                    FnSig::new([ty_a, ty_b].into(), ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = ctx.type_from_cache(ty_a);
            let ty_b = ctx.type_from_cache(ty_b);
            call!(
                CallSite::new_extern(
                    ClassRef::uint_128(ctx),
                    "op_BitwiseOr".into(),
                    FnSig::new([ty_a, ty_b].into(), ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        _ => or!(operand_a, operand_b),
    }
}
pub fn bit_xor_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    ty_b: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let ty_a = ctx.type_from_cache(ty_a);
            let ty_b = ctx.type_from_cache(ty_b);
            call!(
                CallSite::new_extern(
                    ClassRef::int_128(ctx),
                    "op_ExclusiveOr".into(),
                    FnSig::new([ty_a, ty_b].into(), ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = ctx.type_from_cache(ty_a);
            let ty_b = ctx.type_from_cache(ty_b);
            call!(
                CallSite::new_extern(
                    ClassRef::uint_128(ctx),
                    "op_ExclusiveOr".into(),
                    FnSig::new([ty_a, ty_b].into(), ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        _ => xor!(ops_a, ops_b),
    }
}
