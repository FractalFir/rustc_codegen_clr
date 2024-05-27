use serde::{Deserialize, Serialize};

use crate::{cil_root::CILRoot, method::Method, Type};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// A root of a CIL Tree with metadata about local variables it reads/writes into.  
pub struct CILTree {
    tree: CILRoot,
}
impl From<CILRoot> for CILTree {
    fn from(tree: CILRoot) -> Self {
        Self { tree }
    }
}
impl CILTree {
    pub fn fix_for_exception_handler(&mut self, id: u32) {
        self.tree.fix_for_exception_handler(id);
    }
    /// Returns a list of blocks this object may jump to.
    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        self.tree.targets(targets);
    }
    /// Converts a tree with subtrees into multiple trees.
    #[must_use]
    pub fn shed_trees(self) -> Vec<Self> {
        self.tree
            .shed_trees()
            .into_iter()
            .map(std::convert::Into::into)
            .collect()
    }
    /// Retunrs the root of this tree.
    #[must_use]
    pub fn root(&self) -> &CILRoot {
        &self.tree
    }
    /// Optimizes this tree
    pub fn opt(&mut self) {
        self.tree.opt();
    }
    /// Allocates the temporary variables this tree uses.
    pub fn allocate_tmps(&mut self, locals: &mut Vec<(Option<Box<str>>, Type)>) {
        self.tree.allocate_tmps(None, locals);
    }
    pub fn validate(&self, method: &Method) -> Result<(), String> {
        //self.tree.validate(method)
        todo!("method:{method:?}");
    }
    // TODO: remember to make this recompute tree metadtata when it is added
    pub fn root_mut(&mut self) -> &mut CILRoot {
        &mut self.tree
    }
}
