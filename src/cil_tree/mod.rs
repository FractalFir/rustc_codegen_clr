use crate::cil::CILOp;

use self::cil_root::CILRoot;

pub mod cil_node;
pub mod cil_root;
#[derive(Clone, Debug)]
pub struct CILTree {
    tree: CILRoot,
}
impl From<CILRoot> for CILTree {
    fn from(tree: CILRoot) -> Self {
        Self { tree }
    }
}
impl From<CILRoot> for Vec<CILTree> {
    fn from(tree: CILRoot) -> Self {
        vec![CILTree { tree }]
    }
}
impl CILTree {
    pub fn flatten(&self) -> Vec<CILOp> {
        self.tree.flatten()
    }

    pub(crate) fn fix_for_exception_handler(&mut self, id: u32) {
        self.tree.fix_for_exception_handler(id);
    }

    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        self.tree.targets(targets)
    }
}
pub fn append_vec(mut vec: Vec<CILOp>, by: CILOp) -> Vec<CILOp> {
    vec.push(by);
    vec
}
