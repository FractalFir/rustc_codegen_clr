use crate::{cil::CILOp, cil_tree::cil_node::CILNode};
pub(crate) enum CILRoot {
    STLoc { local: u32, tree: CILNode },
}
impl CILRoot {
    pub fn flatten(&self) -> Vec<CILOp> {
        match self {
            Self::STLoc { local, tree } => {
                let mut ops = vec![CILOp::STLoc(*local)];
                ops.extend(tree.flatten());
                ops
            }
        }
    }
}
