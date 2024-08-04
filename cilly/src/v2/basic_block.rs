use serde::{Deserialize, Serialize};

use super::{Assembly, CILRoot, RootIdx};
use crate::basic_block::BasicBlock as V1Block;
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct BasicBlock {
    roots: Vec<RootIdx>,
    block_id: u32,
    handler: Option<Box<[Self]>>,
}

impl BasicBlock {
    pub fn new(roots: Vec<RootIdx>, block_id: u32, handler: Option<Box<[Self]>>) -> Self {
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
    pub fn handler(&self) -> Option<&[BasicBlock]> {
        self.handler.as_ref().map(|b| b.as_ref())
    }
    pub fn handler_mut(&mut self) -> Option<&mut [BasicBlock]> {
        self.handler.as_mut().map(|b| b.as_mut())
    }
    pub fn roots_mut(&mut self) -> &mut Vec<RootIdx> {
        &mut self.roots
    }
    pub fn handler_and_root_mut(&mut self) -> (Option<&mut [BasicBlock]>, &mut Vec<RootIdx>) {
        (self.handler.as_mut().map(|b| b.as_mut()), &mut self.roots)
    }
}
impl BasicBlock {
    pub fn from_v1(v1: &V1Block, asm: &mut Assembly) -> Self {
        let handler: Option<Box<[Self]>> = v1.handler().map(|handler| {
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
