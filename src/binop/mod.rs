use cilly::call_site::CallSite;
use cilly::cil_node::CILNode;
use cilly::cil_root::CILRoot;
use cilly::field_desc::FieldDescriptor;
use cilly::{DotnetTypeRef, Type};
use rustc_hir::lang_items::LangItem;
use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, List, ParamEnv, Ty, TyCtxt, TyKind, UintTy};

use crate::r#type::TyCache;
use cilly::fn_sig::FnSig;

pub mod bitop;
pub mod checked;
pub mod cmp;
pub mod shift;
use bitop::{bit_and_unchecked, bit_or_unchecked, bit_xor_unchecked};
use cilly::{
    call, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8, div, eq, gt_un, ldc_i32, lt_un, rem,
    rem_un, size_of, sub,
};

use cmp::{eq_unchecked, gt_unchecked, lt_unchecked, ne_unchecked};
use shift::{shl_checked, shl_unchecked, shr_checked, shr_unchecked};

use self::checked::{add_signed, add_unsigned, sub_signed, sub_unsigned};
/// Preforms an unchecked binary operation.
pub(crate) fn binop<'tyctx>(
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
        BinOp::AddWithOverflow => {
            if ty_a.is_signed() {
                add_signed(&ops_a, &ops_b, ty_a, tyctx, method_instance, tycache)
            } else {
                add_unsigned(&ops_a, &ops_b, ty_a, tyctx, method_instance, tycache)
            }
        }
        BinOp::Add | BinOp::AddUnchecked => {
            add_unchecked(ty_a, ty_b, tyctx, &method_instance, tycache, ops_a, ops_b)
        }
        BinOp::SubWithOverflow => {
            if ty_a.is_signed() {
                sub_signed(&ops_a, &ops_b, ty_a, tyctx, method_instance, tycache)
            } else {
                sub_unsigned(&ops_a, &ops_b, ty_a, tyctx, method_instance, tycache)
            }
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
        BinOp::MulWithOverflow => {
            checked::mul(&ops_a, &ops_b, ty_a, tyctx, method_instance, tycache)
        }
        BinOp::Div => div_unchecked(ty_a, ty_b, tycache, &method_instance, tyctx, ops_a, ops_b),

        BinOp::Ge => match ty_a.kind() {
            // Unordered, to handle NaNs propely
            TyKind::Float(_) => eq!(lt_un!(ops_a, ops_b), ldc_i32!(0)),
            _ => eq!(lt_unchecked(ty_a, ops_a, ops_b), ldc_i32!(0)),
        },
        BinOp::Le => match ty_a.kind() {
            // Unordered, to handle NaNs propely
            TyKind::Float(_) => eq!(gt_un!(ops_a, ops_b), ldc_i32!(0)),
            _ => eq!(gt_unchecked(ty_a, ops_a, ops_b), ldc_i32!(0)),
        },
        BinOp::Offset => {
            let pointed_ty = if let TyKind::RawPtr(inner, _) = ty_a.kind() {
                *inner
            } else {
                todo!("Can't offset pointer of type {ty_a:?}");
            };
            let pointed_ty = crate::utilis::monomorphize(&method_instance, pointed_ty, tyctx);
            let pointed_ty =
                Box::new(tycache.type_from_cache(pointed_ty, tyctx, method_instance));
            ops_a + ops_b * conv_isize!(size_of!(pointed_ty))
        }
        BinOp::Cmp => {
            let ordering = tyctx
                .get_lang_items(())
                .get(LangItem::OrderingEnum)
                .unwrap();
            let ordering =
                Instance::resolve(tyctx, ParamEnv::reveal_all(), ordering, List::empty())
                    .unwrap()
                    .unwrap();
            let ordering_ty = ordering.ty(tyctx, ParamEnv::reveal_all());
            let ordering_type = tycache.type_from_cache(ordering_ty, tyctx, method_instance);
            let lt = -lt_unchecked(ty_a, ops_a.clone(), ops_b.clone());
            let gt = gt_unchecked(ty_a, ops_a, ops_b);
            let res = lt | gt;
            CILNode::TemporaryLocal(Box::new((
                ordering_type.clone(),
                [CILRoot::SetField {
                    addr: CILNode::LoadAddresOfTMPLocal,
                    value: res,
                    desc: FieldDescriptor::new(
                        ordering_type.as_dotnet().unwrap(),
                        Type::I8,
                        "value__".into(),
                    ),
                }]
                .into(),
                CILNode::LoadTMPLocal,
            )))
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
                let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
                let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::int_128(),
                        "op_Addition".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                        true,
                    ),
                    [ops_a, ops_b]
                )
            } else {
                ops_a + ops_b
            }
        }
        TyKind::Uint(uint_ty) => {
            if let UintTy::U128 = uint_ty {
                let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
                let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::uint_128(),
                        "op_Addition".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                        true,
                    ),
                    [ops_a, ops_b]
                )
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
        TyKind::Float(_) => ops_a + ops_b,
        _ => todo!("can't add numbers of types {ty_a} and {ty_b}"),
    }
}
/// Preforms unchecked subtraction
pub fn sub_unchecked<'tyctx>(
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
                let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
                let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::int_128(),
                        "op_Subtraction".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], ty_a),
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
                let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
                let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
                call!(
                    CallSite::new_extern(
                        DotnetTypeRef::uint_128(),
                        "op_Subtraction".into(),
                        FnSig::new(&[ty_a.clone(), ty_b], ty_a),
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
            let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
            let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Modulus".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
            let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Modulus".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                    true,
                ),
                [ops_a, ops_b]
            )
        }
        TyKind::Int(_) | TyKind::Char | TyKind::Float(_) => rem!(ops_a, ops_b),
        TyKind::Uint(_) => rem_un!(ops_a, ops_b),

        _ => todo!(),
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
            let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
            let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
            let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Multiply".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        _ => operand_a * operand_b,
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
            let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
            let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Division".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        TyKind::Uint(UintTy::U128) => {
            let ty_a = tycache.type_from_cache(ty_a, tyctx, *method_instance);
            let ty_b = tycache.type_from_cache(ty_b, tyctx, *method_instance);
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Division".into(),
                    FnSig::new(&[ty_a.clone(), ty_b], ty_a),
                    true,
                ),
                [operand_a, operand_b]
            )
        }
        TyKind::Uint(_) => CILNode::DivUn(operand_a.into(), operand_b.into()),
        TyKind::Int(_) | TyKind::Char | TyKind::Float(_) => div!(operand_a, operand_b),
        _ => todo!(),
    }
}
