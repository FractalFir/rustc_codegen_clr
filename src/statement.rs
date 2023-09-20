use crate::cil_op::CILOp;
use rustc_middle::{
    mir::{Body, Statement, StatementKind},
    ty::TyCtxt,
};
pub fn handle_statement<'tctx>(
    statement: &Statement<'tctx>,
    body: &'tctx Body<'tctx>,
    tyctx: TyCtxt<'tctx>,
    method: &rustc_middle::mir::Body<'tctx>,
) -> Vec<CILOp> {
    let kind = &statement.kind;
    let res = match kind {
        StatementKind::StorageLive(local) => {
            vec![]
        }
        StatementKind::StorageDead(local) => {
            vec![]
        }
        StatementKind::Assign(palce_rvalue) => {
            let place = palce_rvalue.as_ref().0;
            let rvalue = &palce_rvalue.as_ref().1;
            let rvalue_ops = crate::rvalue::handle_rvalue(&rvalue, tyctx, &place, method);
            crate::place::place_set(&place, tyctx, rvalue_ops, method)
        }
        _ => todo!("Unsuported statement kind {kind:?}"),
    };
    res
}
