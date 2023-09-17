use crate::cil_op::CILOp;
use rustc_middle::mir::{
    interpret::ConstValue, interpret::Scalar, BinOp, Body, CastKind, Constant, ConstantKind,
    Operand, Statement, StatementKind,
};
use rustc_middle::{
    mir::{Place, Rvalue},
    ty::TyCtxt,
};
pub fn handle_rvalue<'tyctx>(
    rvalue: &Rvalue<'tyctx>,
    tcx: TyCtxt<'tyctx>,
    target_location: &Place<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
) -> Vec<CILOp> {
    match rvalue {
        Rvalue::Use(operand) => crate::operand::handle_operand(operand, tcx, method),
        Rvalue::Ref(_region, _kind, place) => crate::place::place_adress(place, tcx, method),
        Rvalue::AddressOf(_mutability, place) => crate::place::place_adress(place, tcx, method),
        Rvalue::Cast(CastKind::PointerCoercion(_) | CastKind::PtrToPtr, operand, _) => {
            crate::operand::handle_operand(operand, tcx, method)
        }
        Rvalue::Cast(kind, operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        Rvalue::BinaryOp(binop, operands) => {
            crate::binop::binop_unchecked(*binop, &operands.0, &operands.1, tcx, method)
        }
        _ => todo!("Unhandled RValue {rvalue:?}"),
    }
}
