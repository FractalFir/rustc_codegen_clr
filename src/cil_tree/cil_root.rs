use crate::{cil::CILOp, cil_tree::cil_node::CILNode};

use super::append_vec;
pub(crate) enum CILRoot {
    STLoc { local: u32, tree: CILNode },
    BTrue { target: u32, ops: CILNode },
    GoTo { target: u32 },
}
impl CILRoot {
    pub fn flatten(&self) -> Vec<CILOp> {
        match self {
            Self::STLoc { local, tree } => append_vec(tree.flatten(), CILOp::STLoc(*local)),
            Self::BTrue { target, ops } => append_vec(ops.flatten(), CILOp::BTrue(*target)),
            Self::GoTo { target } => vec![CILOp::GoTo(*target)],
        }
    }
}
