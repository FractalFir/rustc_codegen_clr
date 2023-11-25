use crate::{cil_op::CILOp, r#type::TyCache};
use rustc_middle::{
    mir::{Body, CopyNonOverlapping, NonDivergingIntrinsic, Statement, StatementKind},
    ty::{Instance, TyCtxt},
};
pub fn handle_statement<'tcx>(
    statement: &Statement<'tcx>,
    tyctx: TyCtxt<'tcx>,
    method: &Body<'tcx>,
    method_instance: Instance<'tcx>,
    type_cache: &mut TyCache,
) -> Vec<CILOp> {
    let kind = &statement.kind;
    let res = match kind {
        StatementKind::StorageLive(_local) => {
            vec![]
        }
        StatementKind::StorageDead(_local) => {
            vec![]
        }
        StatementKind::Assign(palce_rvalue) => {
            let place = palce_rvalue.as_ref().0;
            let rvalue = &palce_rvalue.as_ref().1;
            let rvalue_ops = crate::rvalue::handle_rvalue(
                rvalue,
                tyctx,
                &place,
                method,
                method_instance,
                type_cache,
            );
            crate::place::place_set(
                &place,
                tyctx,
                rvalue_ops,
                method,
                method_instance,
                type_cache,
            )
        }
        StatementKind::Intrinsic(non_diverging_intirinsic) => {
            match non_diverging_intirinsic.as_ref() {
                NonDivergingIntrinsic::Assume(_) => vec![],
                NonDivergingIntrinsic::CopyNonOverlapping(CopyNonOverlapping {
                    src,
                    dst,
                    count,
                }) => {
                    let dst_op = crate::operand::handle_operand(
                        dst,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    );
                    let src_op = crate::operand::handle_operand(
                        src,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    );
                    let count_op = crate::operand::handle_operand(
                        count,
                        tyctx,
                        method,
                        method_instance,
                        type_cache,
                    );
                    let src_ty = src.ty(method, tyctx);
                    let ptr_type = type_cache.type_from_cache(src_ty, tyctx, Some(method_instance));
                    let pointed = if let crate::r#type::Type::Ptr(pointed) = ptr_type {
                        pointed
                    } else {
                        panic!("Copy nonoverlaping called with non-pointer type {src_ty:?}");
                    };
                    let mut res: Vec<_> =
                        [dst_op, src_op, count_op].into_iter().flatten().collect();
                    res.push(CILOp::SizeOf(pointed));
                    res.push(CILOp::Mul);
                    res.push(CILOp::CpBlk);
                    res
                }
                _ => {
                    todo!("Can't handle non-diverging intrinsics {non_diverging_intirinsic:?} yet!")
                }
            }
        }
        _ => todo!("Unsuported statement kind {kind:?}"),
    };
    res
}
