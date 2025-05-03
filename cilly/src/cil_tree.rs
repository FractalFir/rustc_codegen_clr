use serde::{Deserialize, Serialize};

use crate::{cil_root::CILRoot, v2::method::LocalDef};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// A root of a CIL Tree with metadata about local variables it reads/writes into.  
pub struct CILTree {
    //tree: InterCow<CILRoot>,
    tree: CILRoot,
}
impl From<CILRoot> for CILTree {
    fn from(tree: CILRoot) -> Self {
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
    pub fn root(&self) -> &CILRoot {
        &self.tree
    }

    /// Allocates the temporary variables this tree uses.
    pub fn allocate_tmps(&mut self, locals: &mut Vec<LocalDef>) {
        // self.tree.borrow_mut().allocate_tmps(None, locals);
        self.tree.allocate_tmps(None, locals);
    }

    // TODO: remember to make this recompute tree metadtata when it is added
    pub fn root_mut(&mut self) -> &mut CILRoot {
        //self.tree.borrow_mut()
        &mut self.tree
    }
}
