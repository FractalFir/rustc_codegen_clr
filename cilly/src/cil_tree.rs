use serde::{Deserialize, Serialize};

use crate::{cil_root::V1Root, v2::method::LocalDef};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// A root of a CIL Tree with metadata about local variables it reads/writes into.  
pub struct CILTree {
    //tree: InterCow<CILRoot>,
    tree: V1Root,
}
impl From<V1Root> for CILTree {
    fn from(tree: V1Root) -> Self {
        Self {
            //tree: InterCow::new(tree),
            tree,
        }
    }
}
impl CILTree {
    pub fn fix_for_exception_handler(&mut self, id: u32) {
        //self.tree.borrow_mut().fix_for_exception_handler(id);
        self.tree.fix_for_exception_handler(id);
    }
    /// Returns a list of blocks this object may jump to.
    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        self.tree.targets(targets);
    }
    /// Converts a tree with subtrees into multiple trees.
    #[must_use]
    pub fn shed_trees(self) -> Vec<Self> {
        vec![self]
    }
    /// Retunrs the root of this tree.
    #[must_use]
    pub fn root(&self) -> &V1Root {
        &self.tree
    }

    /// Allocates the temporary variables this tree uses.
    pub fn allocate_tmps(&mut self, locals: &mut Vec<LocalDef>) {}

    // TODO: remember to make this recompute tree metadtata when it is added
    pub fn root_mut(&mut self) -> &mut V1Root {
        //self.tree.borrow_mut()
        &mut self.tree
    }
}
