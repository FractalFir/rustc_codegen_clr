use crate::assembly::MethodCompileCtx;
use cilly::{
    and, call,
    cil_node::CILNode,
    or, xor, Type,
    {cilnode::MethodKind, ClassRef, Int, MethodRef},
};
use rustc_codegen_clr_type::GetTypeExt;
use rustc_middle::span_bug;
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
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_BitwiseAnd"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::U128)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    operand_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::U128), operand_b, ctx)
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_BitwiseAnd"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    operand_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I128), operand_b, ctx)
                ]
            )
        }
        TyKind::RawPtr(..) => span_bug!(ctx.span(), "bitand of ptr"),
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
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_BitwiseOr"),
                ctx.sig([ty_a, ty_b], ty_a),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = ctx.type_from_cache(ty_a);
            let ty_b = ctx.type_from_cache(ty_b);
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_BitwiseOr"),
                ctx.sig([ty_a, ty_b], ty_a),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
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
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_ExclusiveOr"),
                ctx.sig([ty_a, ty_b], ty_a),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = ctx.type_from_cache(ty_a);
            let ty_b = ctx.type_from_cache(ty_b);
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_ExclusiveOr"),
                ctx.sig([ty_a, ty_b], ty_a),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
        }
        _ => xor!(ops_a, ops_b),
    }
}
