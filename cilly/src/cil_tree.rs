use serde::{Deserialize, Serialize};

use crate::{cil_node::ValidationContext, cil_root::CILRoot, method::Method, IString, Type};

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
        let trees: Vec<Self> = self
            .tree
            .sheed_trees()
            .into_iter()
            .map(std::convert::Into::into)
            .collect();

        trees
    }
    /// Retunrs the root of this tree.
    #[must_use]
    pub fn root(&self) -> &CILRoot {
        &self.tree
    }
    /// Optimizes this tree
    pub fn opt(&mut self, opt_count: &mut usize) {
        self.tree.opt(opt_count);
    }
    /// Allocates the temporary variables this tree uses.
    pub fn allocate_tmps(&mut self, locals: &mut Vec<(Option<IString>, Type)>) {
        self.tree.allocate_tmps(None, locals);
    }
    pub fn validate(&self, vctx: ValidationContext) -> Result<(), String> {
        self.root().validate(vctx, None)
    }
    // TODO: remember to make this recompute tree metadtata when it is added
    pub fn root_mut(&mut self) -> &mut CILRoot {
        &mut self.tree
    }
}

#[test]
fn test_sheed() {
    use crate::cil_node::CILNode;
    let node = CILNode::SubTrees(Box::new((
        [CILRoot::STLoc {
            local: 11,
            tree: CILNode::TemporaryLocal(Box::new((
                Type::DotnetType(Box::new(crate::DotnetTypeRef::new::<&str, _>(
                    None,
                    "core.ptr.metadata.PtrComponents.h4c1f0d773746020e",
                ))),
                Box::new([CILRoot::SetTMPLocal {
                    value: CILNode::LDLoc(1),
                }]),
                CILNode::LdObj {
                    ptr: Box::new(CILNode::LoadAddresOfTMPLocal),
                    obj: Box::new(Type::DotnetType(Box::new(crate::DotnetTypeRef::new::<
                        &str,
                        _,
                    >(
                        None,
                        "core.ptr.metadata.PtrComponents.h4c1f0d773746020e",
                    )))),
                },
            ))),
        }]
        .into(),
        Box::new(CILNode::LdObj {
            ptr: Box::new(CILNode::LDLocA(11)),
            obj: Box::new(Type::DotnetType(Box::new(crate::DotnetTypeRef::new::<
                &str,
                _,
            >(
                None,
                "core.ptr.metadata.PtrComponents.h4c1f0d773746020e",
            )))),
        }),
    )));
    let tree = CILTree {
        tree: CILRoot::STLoc {
            local: 7,
            tree: node,
        },
    };
    let _trees = tree.shed_trees();
}
#[test]
fn test_alloc() {
    use crate::cil_node::CILNode;
    let mut tree: CILNode = CILNode::TemporaryLocal(Box::new((
        Type::U8,
        [CILRoot::SetTMPLocal {
            value: CILNode::TemporaryLocal(Box::new((Type::U8, [].into(), CILNode::LDLoc(0)))),
        }]
        .into(),
        CILNode::LDLoc(0),
    )));

    let mut locs = vec![];
    tree.allocate_tmps(None, &mut locs);
}
