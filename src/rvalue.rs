use crate::cil_op::CILOp;
use rustc_middle::mir::{
    interpret::ConstValue, interpret::Scalar, BinOp, Body, CastKind, Constant, ConstantKind,
    Operand, Statement, StatementKind,
};
use rustc_middle::{
    mir::{Place, Rvalue},
    ty::{Instance,TyCtxt},
};
pub fn handle_rvalue<'tcx>(
    rvalue: &Rvalue<'tcx>,
    tcx: TyCtxt<'tcx>,
    target_location: &Place<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let res = match rvalue {
        Rvalue::Use(operand) => crate::operand::handle_operand(operand, tcx, method,method_instance),
        Rvalue::CopyForDeref(place) => crate::place::place_get(place, tcx, method,method_instance),
        Rvalue::Ref(_region, _kind, place) => crate::place::place_adress(place, tcx, method,method_instance),
        Rvalue::AddressOf(_mutability, place) => crate::place::place_adress(place, tcx, method,method_instance),
        Rvalue::Cast(CastKind::PointerCoercion(_) | CastKind::PtrToPtr, operand, _) => {
            crate::operand::handle_operand(operand, tcx, method,method_instance)
        }
        Rvalue::BinaryOp(binop, operands) => {
            crate::binop::binop_unchecked(*binop, &operands.0, &operands.1, tcx, method,method_instance)
        }
        Rvalue::UnaryOp(binop, operand) => crate::unop::unop(*binop, operand, tcx, method,method_instance),
        Rvalue::Cast(CastKind::IntToInt, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target, tcx);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src, tcx);
            [
                crate::operand::handle_operand(operand, tcx, method,method_instance),
                crate::casts::int_to_int(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Cast(CastKind::FloatToInt, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target, tcx);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src, tcx);
            [
                crate::operand::handle_operand(operand, tcx, method,method_instance),
                crate::casts::float_to_int(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Cast(CastKind::IntToFloat, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target, tcx);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src, tcx);
            [
                crate::operand::handle_operand(operand, tcx, method,method_instance),
                crate::casts::int_to_float(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Aggregate(aggregate_kind,field_index)=>crate::aggregate::handle_aggregate(tcx, target_location, method, aggregate_kind.as_ref(), field_index,method_instance),
        Rvalue::Cast(kind, operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        _ => todo!("Unhandled RValue {rvalue:?}"),
    };
    res
}
