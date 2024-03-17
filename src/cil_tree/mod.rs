use crate::{cil::CILOp, r#type::Type};

use self::cil_root::CILRoot;
use rustc_middle::ty::TyCtxt;
pub mod cil_node;
pub mod cil_root;
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
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
    pub fn into_ops(&self) -> Vec<CILOp> {

        self.tree.into_ops()
    }

    pub(crate) fn fix_for_exception_handler(&mut self, id: u32) {
        self.tree.fix_for_exception_handler(id);
    }

    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        self.tree.targets(targets)
    }
    pub fn shed_trees(self)->Vec<Self>{
        self.tree.shed_trees().into_iter().map(|tree|tree.into()).collect()
    }
    pub fn tree(&self) -> &CILRoot {
        &self.tree
    }

    pub fn opt(&mut self) {
        self.tree.opt()
    }

    pub(crate) fn allocate_tmps(&mut self, locals: &mut Vec<(Option<Box<str>>, Type)>) {
        self.tree.allocate_tmps(None, locals);
    }

    pub(crate) fn resolve_global_allocations(
        &mut self,
        arg: &mut crate::assembly::Assembly,
        tyctx: TyCtxt,
    ) {
        self.tree.resolve_global_allocations(arg, tyctx);
    }
}
pub fn append_vec(mut vec: Vec<CILOp>, by: CILOp) -> Vec<CILOp> {
    vec.push(by);
    vec
}
