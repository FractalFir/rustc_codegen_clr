use crate::cil::CILOp;

use self::cil_root::CILRoot;

pub mod cil_node;
mod cil_root;
pub struct CILTree {
    tree: CILRoot,
}
impl CILTree {
    pub fn flatten(&self) -> Vec<CILOp> {
        self.tree.flatten()
    }
}
pub fn append_vec(mut vec: Vec<CILOp>, by: CILOp) -> Vec<CILOp> {
    vec.push(by);
    vec
}
