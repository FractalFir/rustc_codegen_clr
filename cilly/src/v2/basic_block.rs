use serde::{Deserialize, Serialize};

use super::{opt, Assembly, CILNode, CILRoot, RootIdx};
use crate::basic_block::BasicBlock as V1Block;
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct BasicBlock {
    roots: Vec<RootIdx>,
    block_id: u32,
    handler: Option<Vec<Self>>,
}

impl BasicBlock {
    pub fn new(roots: Vec<RootIdx>, block_id: u32, handler: Option<Vec<Self>>) -> Self {
        Self {
            roots,
            block_id,
            handler,
        }
    }

    pub fn roots(&self) -> &[RootIdx] {
        &self.roots
    }

    pub fn block_id(&self) -> u32 {
        self.block_id
    }
    pub fn iter_roots(&self) -> impl Iterator<Item = RootIdx> + '_ {
        let handler_iter: Box<dyn Iterator<Item = RootIdx>> = match self.handler() {
            Some(handler) => Box::new(handler.iter().flat_map(|block| block.iter_roots())),
            None => Box::new(std::iter::empty()),
        };
        self.roots().iter().copied().chain(handler_iter)
    }
    /// Remaps all the roots in this block using `root_map` and `node_root`
    /// Iterates trough the roots of this block and its handlers
    pub fn iter_roots_mut(&mut self) -> impl Iterator<Item = &mut RootIdx> + '_ {
        let handler_iter: Box<dyn Iterator<Item = &mut RootIdx>> =
            match self.handler.as_mut().map(|b| b) {
                Some(handler) => {
                    Box::new(handler.iter_mut().flat_map(|block| block.iter_roots_mut()))
                }
                None => Box::new(std::iter::empty()),
            };
        self.roots.iter_mut().chain(handler_iter)
    }
    /// Modifies all nodes and roots in this `BasicBlock`
    pub fn map_roots(
        &mut self,
        asm: &mut Assembly,
        root_map: &mut impl Fn(CILRoot, &mut Assembly) -> CILRoot,
        node_map: &mut impl Fn(CILNode, &mut Assembly) -> CILNode,
    ) {
        self.iter_roots_mut().for_each(|root| {
            let get_root = asm.get_root(*root).clone();
            let val = get_root.map(asm, root_map, node_map);
            *root = asm.alloc_root(val)
        })
    }
    pub fn handler(&self) -> Option<&[BasicBlock]> {
        self.handler.as_ref().map(|b| b.as_ref())
    }
    pub fn handler_mut(&mut self) -> Option<&mut Vec<BasicBlock>> {
        self.handler.as_mut().map(|b| b.as_mut())
    }
    pub fn roots_mut(&mut self) -> &mut Vec<RootIdx> {
        &mut self.roots
    }
    pub fn handler_and_root_mut(&mut self) -> (Option<&mut [BasicBlock]>, &mut Vec<RootIdx>) {
        (self.handler.as_mut().map(|b| b.as_mut()), &mut self.roots)
    }
    /// Checks if this basic block consists of nothing more than an unconditional jump to another block
    pub fn is_direct_jump(&self, asm: &Assembly) -> Option<(u32, u32)> {
        let mut meningfull_root = self
            .iter_roots()
            .filter(|root| !matches!(asm.get_root(*root), CILRoot::SourceFileInfo { .. }));
        let root = meningfull_root.next()?;
        let CILRoot::Branch(binfo) = asm.get_root(root) else {
            return None;
        };
        if opt::is_branch_unconditional(binfo) && meningfull_root.next().is_none() {
            Some((binfo.0, binfo.1))
        } else {
            None
        }
    }
    /// Checks if this basic block consists of nothing more thaan an uncondtional rethrow
    pub fn is_only_rethrow(&self, asm: &Assembly) -> bool {
        let mut meningfull_root = self
            .iter_roots()
            .filter(|root| !matches!(asm.get_root(*root), CILRoot::SourceFileInfo { .. }));
        let Some(root) = meningfull_root.next() else {
            return false;
        };
        CILRoot::ReThrow == *asm.get_root(root) && meningfull_root.next().is_none()
    }

    pub fn remove_handler(&mut self) {
        self.handler = None;
    }
}
impl BasicBlock {
    pub fn from_v1(v1: &V1Block, asm: &mut Assembly) -> Self {
        let handler: Option<Vec<Self>> = v1.handler().map(|handler| {
            handler
                .as_blocks()
                .unwrap()
                .iter()
                .map(|block| Self::from_v1(block, asm))
                .collect()
        });
        Self::new(
            v1.trees()
                .iter()
                .map(|root| {
                    let root = CILRoot::from_v1(root.root(), asm);
                    asm.alloc_root(root)
                })
                .collect(),
            v1.id(),
            handler,
        )
    }
}
