use crate::r#type::tycache::TyCache;
use cilly::call_site::CallSite;
use cilly::cil_node::CILNode;
use cilly::field_desc::FieldDescriptor;
use cilly::fn_sig::FnSig;
use cilly::{call, ld_field, DotnetTypeRef, Type};

use rustc_middle::mir::{Operand, UnOp};
use rustc_middle::ty::{Instance, IntTy, TyCtxt, TyKind, UintTy};

/// Implements an unary operation, such as negation.
pub fn unop<'ctx>(
    unnop: UnOp,
    operand: &Operand<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let parrent_node =
        crate::operand::handle_operand(operand, tyctx, method, method_instance, tycache);
    let ty = operand.ty(&method.local_decls, tyctx);
    match unnop {
        UnOp::Neg => match ty.kind() {
            TyKind::Int(IntTy::I128) => call!(
                CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_UnaryNegation".into(),
                    FnSig::new(&[Type::I128], Type::I128),
                    true,
                ),
                [parrent_node]
            ),
            TyKind::Int(IntTy::I8) => CILNode::Neg(CILNode::ConvI8(parrent_node.into()).into()),
            TyKind::Int(IntTy::I16) => CILNode::Neg(CILNode::ConvI16(parrent_node.into()).into()),
            TyKind::Uint(UintTy::U128) => call!(
                CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_UnaryNegation".into(),
                    FnSig::new(&[Type::U128], Type::U128),
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
                    DotnetTypeRef::uint_128().into(),
                    "op_OnesComplement".into(),
                    FnSig::new(&[Type::U128], Type::U128),
                    true,
                ),
                [parrent_node]
            ),
            TyKind::Int(IntTy::I128) => call!(
                CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_OnesComplement".into(),
                    FnSig::new(&[Type::I128], Type::I128),
                    true,
                ),
                [parrent_node]
            ),

            //TyKind::U128 => ops.extend([CILOp::LdcI32(0), CILOp::Eq]),
            _ => CILNode::Not(parrent_node.into()),
        },
        rustc_middle::mir::UnOp::PtrMetadata => {
            let tpe = tycache
                .type_from_cache(ty, tyctx, method_instance)
                .as_dotnet()
                .unwrap();
            ld_field!(
                parrent_node,
                FieldDescriptor::new(tpe, Type::USize, "metadata".into())
            )
        }
    }
}
