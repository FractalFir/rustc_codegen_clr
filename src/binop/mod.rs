use self::checked::{add_signed, add_unsigned, sub_signed, sub_unsigned};
use crate::assembly::MethodCompileCtx;
use bitop::{bit_and_unchecked, bit_or_unchecked, bit_xor_unchecked};
use cilly::{
    call,
    cil_node::V1Node,
    conv_i8, conv_u16, conv_u32, conv_u64, conv_u8, eq, gt_un, lt_un, rem, rem_un, IntoAsmIndex,
    Type,
    {cilnode::MethodKind, Float, Int, MethodRef},
};
use cmp::{eq_unchecked, gt_unchecked, lt_unchecked, ne_unchecked};
use rustc_codegen_clr_type::{utilis::instance_try_resolve, GetTypeExt};

use rustc_codgen_clr_operand::handle_operand;
use rustc_hir::lang_items::LangItem;
use rustc_middle::{
    mir::{BinOp, Operand},
    ty::{FloatTy, IntTy, List, Ty, TyKind, UintTy},
};
use shift::{shl_checked, shl_unchecked, shr_checked, shr_unchecked};

pub mod bitop;
pub mod checked;
pub mod cmp;
pub mod shift;

/// Preforms an unchecked binary operation.
pub(crate) fn binop<'tcx>(
    binop: BinOp,
    operand_a: &Operand<'tcx>,
    operand_b: &Operand<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Node {
    let ops_a = handle_operand(operand_a, ctx);
    let ops_b = handle_operand(operand_b, ctx);
    let ty_a = operand_a.ty(&ctx.body().local_decls, ctx.tcx());
    let ty_b = operand_b.ty(&ctx.body().local_decls, ctx.tcx());
    match binop {
        BinOp::AddWithOverflow => {
            if ty_a.is_signed() {
                add_signed(&ops_a, &ops_b, ty_a, ctx)
            } else {
                add_unsigned(&ops_a, &ops_b, ty_a, ctx)
            }
        }
        BinOp::Add | BinOp::AddUnchecked => add_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::SubWithOverflow => {
            if ty_a.is_signed() {
                sub_signed(&ops_a, &ops_b, ty_a, ctx)
            } else {
                sub_unsigned(&ops_a, &ops_b, ty_a, ctx)
            }
        }
        BinOp::Sub | BinOp::SubUnchecked => sub_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::Ne => ne_unchecked(ty_a, ops_a, ops_b, ctx),
        BinOp::Eq => eq_unchecked(ty_a, ops_a, ops_b, ctx),
        BinOp::Lt => lt_unchecked(ty_a, ops_a, ops_b, ctx),
        BinOp::Gt => gt_unchecked(ty_a, ops_a, ops_b, ctx),
        BinOp::BitAnd => bit_and_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::BitOr => bit_or_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::BitXor => bit_xor_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::Rem => rem_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::Shl => shl_checked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::ShlUnchecked => shl_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::Shr => shr_checked(ty_a, ty_b, ctx, ops_a, ops_b),
        BinOp::ShrUnchecked => shr_unchecked(ty_a, ty_b, ctx, ops_a, ops_b),

        BinOp::Mul | BinOp::MulUnchecked => mul_unchecked(ty_a, ctx, ops_a, ops_b),
        BinOp::MulWithOverflow => checked::mul(&ops_a, &ops_b, ty_a, ctx),
        BinOp::Div => div_unchecked(ty_a, ctx, ops_a, ops_b),

        BinOp::Ge => match ty_a.kind() {
            // Unordered, to handle NaNs propely
            TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
                eq!(lt_un!(ops_a, ops_b), V1Node::V2(ctx.alloc_node(false)))
            }
            TyKind::Float(FloatTy::F128) => {
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string("__getf2"),
                    ctx.sig(
                        [Type::Float(Float::F128), Type::Float(Float::F128)],
                        Type::Bool,
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
            }
            _ => eq!(
                lt_unchecked(ty_a, ops_a, ops_b, ctx),
                V1Node::V2(ctx.alloc_node(false))
            ),
        },
        BinOp::Le => match ty_a.kind() {
            // Unordered, to handle NaNs propely
            TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
                eq!(gt_un!(ops_a, ops_b), V1Node::V2(ctx.alloc_node(false)))
            }
            TyKind::Float(FloatTy::F128) => {
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string("__letf2"),
                    ctx.sig(
                        [Type::Float(Float::F128), Type::Float(Float::F128)],
                        Type::Bool,
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
            }
            _ => eq!(
                gt_unchecked(ty_a, ops_a, ops_b, ctx),
                V1Node::V2(ctx.alloc_node(false))
            ),
        },
        BinOp::Offset => {
            let pointed_ty = if let TyKind::RawPtr(inner, _) = ty_a.kind() {
                *inner
            } else {
                todo!("Can't offset pointer of type {ty_a:?}");
            };
            let pointed_ty = ctx.monomorphize(pointed_ty);
            let layout = ctx.layout_of(pointed_ty);
            if layout.is_zst() {
                ops_a
            } else {
                let pointed_type = ctx.type_from_cache(pointed_ty);
                let offset_tpe = ctx.type_from_cache(ty_b);
                ops_a
                    + ops_b
                        * crate::casts::int_to_int(
                            Type::Int(Int::U64),
                            offset_tpe,
                            V1Node::V2(ctx.size_of(pointed_type).into_idx(ctx)),
                            ctx,
                        )
            }
        }
        BinOp::Cmp => {
            let ordering = ctx
                .tcx()
                .get_lang_items(())
                .get(LangItem::OrderingEnum)
                .unwrap();
            let ordering = instance_try_resolve(ordering, ctx.tcx(), List::empty());
            let ordering_ty = ordering.ty(
                ctx.tcx(),
                rustc_middle::ty::TypingEnv::fully_monomorphized(),
            );
            let ordering_type = ctx.type_from_cache(ordering_ty);
            let lt = -conv_i8!(lt_unchecked(ty_a, ops_a.clone(), ops_b.clone(), ctx));
            let gt = conv_i8!(gt_unchecked(ty_a, ops_a, ops_b, ctx));
            let res = lt | gt;

            res.transmute_on_stack(Type::Int(Int::I8), ordering_type, ctx)
        }
    }
}
/// Preforms unchecked addition
pub fn add_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    ty_b: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: V1Node,
    ops_b: V1Node,
) -> V1Node {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string("add_i128"),
                    ctx.sig(
                        [Type::Int(Int::I128), Type::Int(Int::I128)],
                        Type::Int(Int::I128),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
            } else {
                ops_a + ops_b
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string("add_u128"),
                    ctx.sig(
                        [Type::Int(Int::U128), Type::Int(Int::U128)],
                        Type::Int(Int::U128),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
            } else {
                match uint_ty {
                    UintTy::U8 => conv_u8!(ops_a + ops_b),
                    UintTy::U16 => conv_u16!(ops_a + ops_b),
                    UintTy::U32 => conv_u32!(ops_a + ops_b),
                    UintTy::U64 => conv_u64!(ops_a + ops_b),
                    _ => ops_a + ops_b,
                }
            }
        }
        TyKind::Float(FloatTy::F32 | FloatTy::F64) => ops_a + ops_b,
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("__addtf3"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Float(Float::F128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b,])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("add_f16"),
                ctx.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Float(Float::F16),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b,])
        }
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
/// Preforms unchecked subtraction
pub fn sub_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    ty_b: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: V1Node,
    ops_b: V1Node,
) -> V1Node {
    match ty_a.kind() {
        TyKind::Int(int_ty) => {
            if let IntTy::I128 = int_ty {
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string("sub_i128"),
                    ctx.sig(
                        [Type::Int(Int::I128), Type::Int(Int::I128)],
                        Type::Int(Int::I128),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
            } else {
                V1Node::Sub(Box::new(ops_a), Box::new(ops_b))
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let mref = MethodRef::new(
                    *ctx.main_module(),
                    ctx.alloc_string("sub_u128"),
                    ctx.sig(
                        [Type::Int(Int::U128), Type::Int(Int::U128)],
                        Type::Int(Int::U128),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
            } else {
                V1Node::Sub(Box::new(ops_a), Box::new(ops_b))
            }
        }
        TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            V1Node::Sub(Box::new(ops_a), Box::new(ops_b))
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("__subtf3"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Float(Float::F128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("sub_f16"),
                ctx.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Float(Float::F16),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b,])
        }
        _ => todo!("can't sub numbers of types {ty_a} and {ty_b}"),
    }
}

fn rem_unchecked<'tcx>(
    ty_a: Ty<'tcx>,
    _ty_b: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: V1Node,
    ops_b: V1Node,
) -> V1Node {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("mod_i128"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
        }
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("mod_u128"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::U128)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
        }
        TyKind::Int(_) | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            rem!(ops_a, ops_b)
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("fmodl"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Float(Float::F128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("mod_f16"),
                ctx.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Float(Float::F16),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [ops_a, ops_b,])
        }
        TyKind::Uint(_) => rem_un!(ops_a, ops_b),

        _ => todo!(),
    }
}

fn mul_unchecked<'tcx>(
    ty_a: Ty<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
    operand_a: V1Node,
    operand_b: V1Node,
) -> V1Node {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("mul_i128"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("mul_u128"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::U128)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("__multf3"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Float(Float::F128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("mul_f16"),
                ctx.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Float(Float::F16),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b,])
        }
        _ => operand_a * operand_b,
    }
}
fn div_unchecked<'tcx>(
    ty_a: Ty<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
    operand_a: V1Node,
    operand_b: V1Node,
) -> V1Node {
    match ty_a.kind() {
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("div_i128"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("div_u128"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::U128)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Uint(_) => V1Node::DivUn(operand_a.into(), operand_b.into()),
        TyKind::Int(_) | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            V1Node::Div(Box::new(operand_a), Box::new(operand_b))
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("__divtf3"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Float(Float::F128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("div_f16"),
                ctx.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Float(Float::F16),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(ctx.alloc_methodref(mref), [operand_a, operand_b,])
        }
        _ => todo!(),
    }
}
