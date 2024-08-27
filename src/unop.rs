use crate::assembly::MethodCompileCtx;
use crate::r#type::get_type;
use cilly::call_site::CallSite;
use cilly::cil_node::CILNode;
use cilly::field_desc::FieldDescriptor;
use cilly::fn_sig::FnSig;
use cilly::v2::{ClassRef, Int};
use cilly::{call, ld_field, Type};

use rustc_middle::mir::{Operand, UnOp};
use rustc_middle::ty::{IntTy, TyKind, UintTy};

/// Implements an unary operation, such as negation.
pub fn unop<'tcx>(
    unnop: UnOp,
    operand: &Operand<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let parrent_node = crate::operand::handle_operand(operand, ctx);
    let ty = operand.ty(&ctx.body().local_decls, ctx.tcx());
    match unnop {
        UnOp::Neg => match ty.kind() {
            TyKind::Int(IntTy::I128) => call!(
                CallSite::boxed(
                    ClassRef::int_128(ctx.asm_mut()).into(),
                    "op_UnaryNegation".into(),
                    FnSig::new(&[Type::Int(Int::I128)], Type::Int(Int::I128)),
                    true,
                ),
                [parrent_node]
            ),
            TyKind::Int(IntTy::I8) => CILNode::Neg(CILNode::ConvI8(parrent_node.into()).into()),
            TyKind::Int(IntTy::I16) => CILNode::Neg(CILNode::ConvI16(parrent_node.into()).into()),
            TyKind::Uint(UintTy::U128) => call!(
                CallSite::boxed(
                    ClassRef::uint_128(ctx.asm_mut()).into(),
                    "op_UnaryNegation".into(),
                    FnSig::new(&[Type::Int(Int::U128)], Type::Int(Int::U128)),
                    true,
                ),
                [parrent_node]
            ),
            _ => CILNode::Neg(parrent_node.into()),
        },
        UnOp::Not => match ty.kind() {
            TyKind::Bool => CILNode::Eq(CILNode::LdFalse.into(), parrent_node.into()),
            TyKind::Uint(UintTy::U128) => call!(
                CallSite::boxed(
                    ClassRef::uint_128(ctx.asm_mut()).into(),
                    "op_OnesComplement".into(),
                    FnSig::new(&[Type::Int(Int::U128)], Type::Int(Int::U128)),
                    true,
                ),
                [parrent_node]
            ),
            TyKind::Int(IntTy::I128) => call!(
                CallSite::boxed(
                    ClassRef::int_128(ctx.asm_mut()).into(),
                    "op_OnesComplement".into(),
                    FnSig::new(&[Type::Int(Int::I128)], Type::Int(Int::I128)),
                    true,
                ),
                [parrent_node]
            ),

            //TyKind::U128 => ops.extend([CILOp::LdcI32(0), CILOp::Eq]),
            _ => CILNode::Not(parrent_node.into()),
        },
        rustc_middle::mir::UnOp::PtrMetadata => {
            let tpe = get_type(ty, ctx)
                .as_class_ref()
                .expect("Invalid pointer type");
            ld_field!(
                parrent_node,
                FieldDescriptor::new(
                    tpe,
                    cilly::v2::Type::Int(cilly::v2::Int::USize),
                    crate::METADATA.into()
                )
            )
        }
    }
}
