use serde::{Deserialize, Serialize};

use super::{opt, Assembly, CILNode, CILRoot, RootIdx};
use crate::basic_block::BasicBlock as V1Block;
pub type BlockId = u32;
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct BasicBlock {
    roots: Vec<RootIdx>,
    block_id: BlockId,
    handler: Option<Vec<Self>>,
}

impl BasicBlock {
    pub fn targets<'block, 'asm: 'block>(
        &'block self,
        asm: &'asm Assembly,
    ) -> impl Iterator<Item = BlockId> + 'block {
        self.roots().iter().filter_map(|root| {
            match asm.get_root(*root) {
                CILRoot::Branch(info) => {
                    let (target, sub_target, _) = info.as_ref();
                    //Some(*sub_target)
                    //(eprintln!("{target} {sub_target}");
                    if *sub_target == 0 {
                        Some(*target)
                    } else {
                        Some(*sub_target)
                    }
                }
                CILRoot::ExitSpecialRegion { target, .. } => Some(*target),
                _ => None,
            }
        })
    }
    #[must_use]
    pub fn new(roots: Vec<RootIdx>, block_id: BlockId, handler: Option<Vec<Self>>) -> Self {
        Self {
            roots,
            block_id,
            handler,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RootIdx] {
        &self.roots
    }

    #[must_use]
    pub fn block_id(&self) -> BlockId {
        self.block_id
    }
    pub fn iter_roots(&self) -> impl Iterator<Item = RootIdx> + '_ {
        let handler_iter: Box<dyn Iterator<Item = RootIdx>> = match self.handler() {
            Some(handler) => Box::new(handler.iter().flat_map(BasicBlock::iter_roots)),
            None => Box::new(std::iter::empty()),
        };
        self.roots().iter().copied().chain(handler_iter)
    }
    /// Remaps all the roots in this block using `root_map` and `node_root`
    /// Iterates trough the roots of this block and its handlers
    pub fn iter_roots_mut(&mut self) -> impl Iterator<Item = &mut RootIdx> + '_ {
        let handler_iter: Box<dyn Iterator<Item = &mut RootIdx>> = match self.handler.as_mut() {
            Some(handler) => Box::new(handler.iter_mut().flat_map(BasicBlock::iter_roots_mut)),
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
            *root = asm.alloc_root(val);
        });
    }
    #[must_use]
    /// Returns an immutable reference to this blocks handler.
    /// ```
    /// # use cilly::v2::BasicBlock;
    /// let block = BasicBlock::new(vec![],0,Some(vec![BasicBlock::new(vec![],1,None)]));
    /// assert_eq!(block.handler().unwrap().len(),1);
    /// ```
    pub fn handler(&self) -> Option<&[BasicBlock]> {
        self.handler.as_ref().map(std::convert::AsRef::as_ref)
    }
    /// Returns a mutable reference to this blocks handler.
    /// ```
    /// # use cilly::v2::BasicBlock;
    /// let mut block = BasicBlock::new(vec![],0,Some(vec![BasicBlock::new(vec![],1,None)]));
    /// assert_eq!(block.handler_mut().unwrap().len(),1);
    /// // Add another block to this handler
    /// block.handler_mut().unwrap().push(BasicBlock::new(vec![],2,None));
    /// assert_eq!(block.handler_mut().unwrap().len(),2);
    /// ```
    pub fn handler_mut(&mut self) -> Option<&mut Vec<BasicBlock>> {
        self.handler.as_mut()
    }
    pub fn roots_mut(&mut self) -> &mut Vec<RootIdx> {
        &mut self.roots
    }
    pub fn handler_and_root_mut(&mut self) -> (Option<&mut [BasicBlock]>, &mut Vec<RootIdx>) {
        (
            self.handler.as_mut().map(std::convert::AsMut::as_mut),
            &mut self.roots,
        )
    }
    /// Checks if this basic block consists of nothing more than an unconditional jump to another block.
    /// ```
    /// # use cilly::*;
    /// # use cilly::v2::BasicBlock;
    /// # let mut asm = Assembly::default();
    /// # let mut void_ret = asm.alloc_root(CILRoot::VoidRet);
    /// # let mut rethrow = asm.alloc_root(CILRoot::ReThrow);
    /// # let mut val = asm.alloc_node(0);
    /// # let mut do_sth = asm.alloc_root(CILRoot::StLoc(0,val));
    /// let target = 11;
    /// let mut jump = asm.alloc_root(CILRoot::Branch(Box::new((target,0,None))));
    /// assert_eq!(BasicBlock::new(vec![],0,None).is_direct_jump(&asm),None);
    /// assert_eq!(BasicBlock::new(vec![void_ret],0,None).is_direct_jump(&asm),None);
    /// assert_eq!(BasicBlock::new(vec![jump],0,None).is_direct_jump(&asm),Some((target,0)));
    /// assert_eq!(BasicBlock::new(vec![do_sth,jump],0,None).is_direct_jump(&asm),None);
    /// ```
    #[must_use]
    pub fn is_direct_jump(&self, asm: &Assembly) -> Option<(BlockId, BlockId)> {
        let mut meningfull_root = self.meaningfull_roots(asm);
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
    /// ```
    /// # use cilly::*;
    /// # use cilly::v2::BasicBlock;
    /// # let mut asm = Assembly::default();
    /// # let mut void_ret = asm.alloc_root(CILRoot::VoidRet);
    /// # let mut rethrow = asm.alloc_root(CILRoot::ReThrow);
    /// # let mut val = asm.alloc_node(0);
    /// # let mut do_sth = asm.alloc_root(CILRoot::StLoc(0,val));
    /// assert!(!BasicBlock::new(vec![],0,None).is_only_rethrow(&asm));
    /// assert!(!BasicBlock::new(vec![void_ret],0,None).is_only_rethrow(&asm));
    /// assert!(BasicBlock::new(vec![rethrow],0,None).is_only_rethrow(&asm));
    /// assert!(!BasicBlock::new(vec![do_sth,rethrow],0,None).is_only_rethrow(&asm));
    /// ```
    #[must_use]
    pub fn is_only_rethrow(&self, asm: &Assembly) -> bool {
        let mut meningfull_root = self.meaningfull_roots(asm);
        let Some(root) = meningfull_root.next() else {
            return false;
        };
        CILRoot::ReThrow == *asm.get_root(root) && meningfull_root.next().is_none()
    }

    pub fn meaningfull_roots<'s, 'asm: 's>(
        &'s self,
        asm: &'asm Assembly,
    ) -> impl Iterator<Item = RootIdx> + 's {
        self.iter_roots().filter(move |root| {
            !matches!(
                asm.get_root(*root),
                CILRoot::Nop | CILRoot::SourceFileInfo { .. }
            )
        })
    }
    /// Removes this blocks handler.
    /// ```
    /// # use cilly::v2::BasicBlock;
    /// # let mut asm = cilly::v2::Assembly::default();
    /// let mut block = BasicBlock::new(vec![],0,Some(vec![BasicBlock::new(vec![],1,None)]));
    /// assert!(block.handler().is_some());
    /// // Add another block to this handler
    /// block.remove_handler(&mut asm);
    /// assert!(block.handler().is_none());
    /// ```
    pub fn remove_handler(&mut self, asm: &mut Assembly) {
        self.handler = None;
        self.roots_mut().iter_mut().for_each(|root| {
            if let CILRoot::ExitSpecialRegion { target, source: _ } = asm[*root] {
                *root = asm.alloc_root(CILRoot::Branch(Box::new((target, 0, None))));
            }
        });
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
#[test]
fn is_direct_jump() {
    let asm = &mut Assembly::default();
    let block = BasicBlock::new(vec![], 0, None);
    // A Block which is empty is not a direwct jump anywhere.'
    assert!(block.is_direct_jump(asm).is_none());
}
#[test]
fn is_only_rethrow() {
    let asm = &mut Assembly::default();
    let block = BasicBlock::new(vec![], 0, None);
    // A Block which is empty is not a rethrow.
    assert!(!block.is_only_rethrow(asm));
    let rethrow = asm.alloc_root(CILRoot::ReThrow);
    let block = BasicBlock::new(vec![rethrow], 0, None);
    // A Block which is just a rethrow is, well, a rethrow.
    assert!(block.is_only_rethrow(asm));
    let dbg_break = asm.alloc_root(CILRoot::Break);
    let block = BasicBlock::new(vec![dbg_break, rethrow], 0, None);
    // A dbg break has side effects, this should return false
    assert!(!block.is_only_rethrow(asm));
    let dbg_break = asm.alloc_root(CILRoot::Break);
    let block = BasicBlock::new(vec![rethrow, dbg_break], 0, None);
    // A dbf break has side effects, this should return false
    assert!(!block.is_only_rethrow(asm));
}
