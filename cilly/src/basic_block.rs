use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    cil_iter::CILIterElem, cil_iter_mut::CILIterElemMut, cil_node::ValidationContext,
    cil_root::CILRoot, cil_tree::CILTree, ilasm_op::DepthSetting, IlasmFlavour,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
/// A block of ops that is a valid jump target, and is protected by an exception handler.
pub struct BasicBlock {
    trees: Vec<CILTree>,
    id: u32,
    handler: Option<Handler>,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Handler {
    RawID(u32),
    Blocks(Vec<BasicBlock>),
}

impl Handler {
    pub fn as_blocks_mut(&mut self) -> Option<&mut Vec<BasicBlock>> {
        if let Self::Blocks(v) = self {
            Some(v)
        } else {
            None
        }
    }
    #[must_use]
    pub fn as_blocks(&self) -> Option<&[BasicBlock]> {
        if let Self::Blocks(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
fn find_bb(id: u32, bbs: &[BasicBlock]) -> &BasicBlock {
    bbs.iter().find(|bb| bb.id() == id).unwrap()
}
fn block_gc(entrypoint: u32, bbs: &[BasicBlock]) -> Vec<BasicBlock> {
    //debug_assert!(crate::utilis::is_sorted(bbs.iter(),|a,b|a.id + 1 == b.id));
    let mut alive: HashSet<u32> = HashSet::new();
    let mut resurecting = HashSet::new();
    let mut to_resurect = HashSet::new();
    to_resurect.insert(entrypoint);
    while !to_resurect.is_empty() {
        alive.extend(&resurecting);
        resurecting.clear();
        resurecting.extend(&to_resurect);
        to_resurect.clear();
        for (target, sub_target) in resurecting
            .iter()
            .flat_map(|bb| find_bb(*bb, bbs).targets())
        {
            assert_eq!(
                sub_target, 0,
                "No block can have subblocks before the exception handler resolving phase!"
            );
            if !alive.contains(&target) && !resurecting.contains(&target) {
                to_resurect.insert(target);
            }
        }
    }
    alive.extend(&resurecting);
    bbs.iter()
        .filter(|bb| alive.contains(&bb.id))
        .cloned()
        .collect()
}
impl BasicBlock {
    /// Converts all trees containing sub-trees into multiple trees.
    pub fn sheed_trees(&mut self) {
        // Get the current trees
        let trees = swap_out(&mut self.trees);
        self.trees = trees.into_iter().flat_map(CILTree::shed_trees).collect();
        if let Some(handler) = self.handler.as_mut() {
            handler
                .as_blocks_mut()
                .unwrap()
                .iter_mut()
                .for_each(Self::sheed_trees);
        }
    }
    pub fn final_uncond_jump(&self) -> Option<u32> {
        self.trees()
            .last()
            .into_iter()
            .filter_map(|tree| match tree.root() {
                CILRoot::GoTo { target, sub_target } => {
                    if *sub_target != 0 {
                        None
                    } else {
                        Some(*target)
                    }
                }
                _ => None,
            })
            .next()
    }
    pub fn resolve_exception_handlers(&mut self, handler_bbs: &[Self]) {
        let Some(handler) = &self.handler else {
            return;
        };
        let Handler::RawID(handler_id) = handler else {
            panic!("Tired to double-resolve ");
        };
        // Get alive blovks
        let mut handler = block_gc(*handler_id, handler_bbs);
        // Fix up handler jumps
        for bb in &mut handler {
            bb.trees
                .iter_mut()
                .for_each(|tree| tree.fix_for_exception_handler(self.id()));
        }
        // Insert the "jumpstarter"
        handler.insert(
            0,
            Self::new(
                vec![CILRoot::GoTo {
                    target: self.id(),
                    sub_target: *handler_id,
                }
                .into()],
                u32::MAX,
                None,
            ),
        );
        // Generate launching pads for cross-block branches!
        let id = self.id();
        let targets = self.targets();
        let targets: HashSet<_> = targets.iter().collect();
        for (target, sub_target) in targets {
            assert_eq!(*sub_target, 0);
            self.trees.push(
                CILRoot::JumpingPad {
                    target: *target,
                    source: id,
                }
                .into(),
            );
        }
        // Change branches to use lanuching pads.

        self.trees
            .iter_mut()
            .for_each(|tree| tree.fix_for_exception_handler(id));
        self.handler = Some(Handler::Blocks(handler));
    }
    /// Creates a new basic block with id `id`, made up from `trees` and with exception handler `handler`.
    #[must_use]
    pub fn new(trees: Vec<CILTree>, id: u32, handler: Option<Handler>) -> Self {
        Self { trees, id, handler }
    }
    /// Returns a list of basic blocks this baisc block targets.
    #[must_use]
    pub fn targets(&self) -> Vec<(u32, u32)> {
        let mut targets = Vec::new();
        self.trees
            .iter()
            .for_each(|tree| tree.targets(&mut targets));
        targets
    }

    /// Returns the id of this block.
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }
    /// Returns a mutable reference to the trees that make up this block.
    pub fn trees_mut(&mut self) -> &mut Vec<CILTree> {
        &mut self.trees
    }
    /// Returns a mutable reference to the trees that make up this block, and its exception handler.
    pub fn all_trees_mut(&mut self) -> impl Iterator<Item = &mut CILTree> {
        let trees = self.trees.iter_mut();
        let blocks = if let Some(handler) = &mut self.handler {
            handler.as_blocks_mut().expect("ERROR: tried to iterate trough the handler of a block with unresolved handlers.").iter_mut()
        } else {
            std::slice::IterMut::default()
        };
        trees.chain(blocks.flat_map(|block| block.trees_mut()))
    }
    /// Returns a reference to the trees that make up this block.
    #[must_use]
    pub fn trees(&self) -> &[CILTree] {
        &self.trees
    }
    /// Returns a iterator over `CILIterElem`
    pub fn iter_cil(&self) -> impl Iterator<Item = CILIterElem> {
        let handler_bbs = self
            .handler
            .iter()
            .filter_map(|handler| handler.as_blocks())
            .flatten();
        let sref: &Self = self;
        let self_blocks = Some(sref).into_iter();
        let block_iter = self_blocks.chain(handler_bbs);
        block_iter
            .flat_map(|block| block.trees.iter())
            .flat_map(|tree| tree.root().into_iter())
    }
    /// Returns a iterator over `CILIterElemMut`
    pub fn iter_cil_mut(&mut self) -> impl Iterator<Item = CILIterElemMut> {
        let handler_bbs = self
            .handler
            .iter_mut()
            .filter_map(|handler| handler.as_blocks_mut())
            .flatten()
            .flat_map(|block| block.trees.iter_mut());
        let self_blocks = self.trees.iter_mut();
        let block_iter = self_blocks.chain(handler_bbs);
        block_iter.flat_map(|tree| tree.root_mut().into_iter())
    }

    #[must_use]
    pub fn handler(&self) -> Option<&Handler> {
        self.handler.as_ref()
    }
    pub fn validate(&self, method: ValidationContext) -> Result<(), String> {
        let errs: Vec<String> = self
            .trees()
            .iter()
            .filter_map(|tree| {
                match tree
                    .validate(method)
                    .map_err(|err| format!("{tree:?}:\n\n{err}"))
                {
                    Ok(()) => None,
                    Err(err) => Some(err),
                }
            })
            .collect::<Vec<_>>();
        if !errs.is_empty() {
            return Err(errs[0].clone());
        }
        Ok(())
    }
}
pub fn export(
    out: &mut impl std::fmt::Write,
    block: &BasicBlock,
    depth: DepthSetting,
    flavour: IlasmFlavour,
) -> std::fmt::Result {
    let this_depth = if block.handler().is_some() {
        write!(out, ".try{{").unwrap();
        depth.incremented()
    } else {
        depth
    };
    // Basic block
    writeln!(out, "bb_{id}_0:", id = block.id()).unwrap();
    for tree in block.trees() {
        crate::ilasm_op::export_root(out, tree.root(), this_depth, flavour).unwrap();
    }
    if let Some(handler) = block.handler() {
        let handler = handler.as_blocks().unwrap();
        write!(out, "}}catch [System.Runtime]System.Object{{\npop").unwrap();
        DepthSetting::with_pading().pad(out).unwrap();
        for handler_block in handler {
            writeln!(
                out,
                "bb_{main_id}_{sub_id}:",
                main_id = block.id(),
                sub_id = handler_block.id()
            )
            .unwrap();
            for tree in handler_block.trees() {
                crate::ilasm_op::export_root(out, tree.root(), this_depth, flavour).unwrap();
            }
            this_depth.pad(out).unwrap();
        }

        write!(out, "}}")?;
    }
    //eprintln!("{string}");
    depth.pad(out)?;
    Ok(())
}
#[inline]
pub fn swap_out<T>(val: &mut Vec<T>) -> Vec<T> {
    let mut tmp = Vec::new();
    std::mem::swap(&mut tmp, val);
    tmp
}
