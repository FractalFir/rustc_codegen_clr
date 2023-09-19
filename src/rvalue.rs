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
    
    let res = match rvalue {
        Rvalue::Use(operand) => crate::operand::handle_operand(operand, tcx, method),
        Rvalue::CopyForDeref(place) => crate::place::place_get(place, tcx, method),
        Rvalue::Ref(_region, _kind, place) => crate::place::place_adress(place, tcx, method),
        Rvalue::AddressOf(_mutability, place) => crate::place::place_adress(place, tcx, method),
        Rvalue::Cast(CastKind::PointerCoercion(_) | CastKind::PtrToPtr, operand, _) => crate::operand::handle_operand(operand, tcx, method),
        Rvalue::BinaryOp(binop, operands) => {
            crate::binop::binop_unchecked(*binop, &operands.0, &operands.1, tcx, method)
        }
        Rvalue::UnaryOp(binop, operand) => crate::unop::unop(*binop, operand, tcx, method),
        Rvalue::Cast(CastKind::IntToInt, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target,tcx);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src,tcx);
            [crate::operand::handle_operand(operand, tcx, method),crate::casts::int_to_int(src,target)].into_iter().flatten().collect()
        }
        Rvalue::Cast(CastKind::FloatToInt, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target,tcx);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src,tcx);
            [crate::operand::handle_operand(operand, tcx, method),crate::casts::float_to_int(src,target)].into_iter().flatten().collect()
        }
        Rvalue::Cast(CastKind::IntToFloat, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target,tcx);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src,tcx);
            [crate::operand::handle_operand(operand, tcx, method),crate::casts::int_to_float(src,target)].into_iter().flatten().collect()
        }
        Rvalue::Cast(kind, operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        _ => todo!("Unhandled RValue {rvalue:?}"),
    };
    res
}
