use crate::r#type::tycache::TyCache;
use cilly::call_site::CallSite;
use cilly::cil_node::CILNode;
use cilly::fn_sig::FnSig;
use cilly::{DotnetTypeRef, Type};

use rustc_middle::mir::{Operand, UnOp};
use rustc_middle::ty::{Instance, IntTy, TyCtxt, TyKind, UintTy};

/// Implements an unary operation, such as negation.
pub fn unop<'ctx>(
    unnop: UnOp,
    operand: &Operand<'ctx>,
    tcx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
    tycache: &mut TyCache,
) -> CILNode {
    let parrent_node =
        crate::operand::handle_operand(operand, tcx, method, method_instance, tycache);
    let ty = operand.ty(&method.local_decls, tcx);
    match unnop {
        UnOp::Neg => match ty.kind() {
            TyKind::Int(IntTy::I128) => CILNode::Call {
                site: CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_UnaryNegation".into(),
                    FnSig::new(&[Type::I128], Type::I128),
                    true,
                ),
                args: [parrent_node].into(),
            },
            TyKind::Int(IntTy::I8) => CILNode::Neg(CILNode::ConvI8(parrent_node.into()).into()),
            TyKind::Int(IntTy::I16) => CILNode::Neg(CILNode::ConvI16(parrent_node.into()).into()),
            TyKind::Uint(UintTy::U128) => CILNode::Call {
                site: CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_UnaryNegation".into(),
                    FnSig::new(&[Type::U128], Type::U128),
                    true,
                ),
                args: [parrent_node].into(),
            },
            _ => CILNode::Neg(parrent_node.into()),
        },
        UnOp::Not => match ty.kind() {
            TyKind::Bool => CILNode::Eq(CILNode::LdcI32(0).into(), parrent_node.into()),
            TyKind::Uint(UintTy::U128) => CILNode::Call {
                site: CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_OnesComplement".into(),
                    FnSig::new(&[Type::U128], Type::U128),
                    true,
                ),
                args: [parrent_node].into(),
            },
            TyKind::Int(IntTy::I128) => CILNode::Call {
                site: CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_OnesComplement".into(),
                    FnSig::new(&[Type::I128], Type::I128),
                    true,
                ),
                args: [parrent_node].into(),
            },

            //TyKind::U128 => ops.extend([CILOp::LdcI32(0), CILOp::Eq]),
            _ => CILNode::Not(parrent_node.into()),
        },
    }
}
