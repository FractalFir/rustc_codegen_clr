use std::collections::HashSet;

use crate::{
    cil::CILOp,
    cil_tree::{cil_root::CILRoot, CILTree},
};
use rustc_middle::mir::BasicBlockData;
use rustc_middle::mir::UnwindAction;
use rustc_middle::{
    mir::{BasicBlocks, Body, TerminatorKind},
    ty::{Instance, InstanceDef, TyCtxt},
};
use serde::{Deserialize, Serialize};
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
}

pub(crate) fn handler_for_block<'tyctx>(
    block_data: &BasicBlockData,
    blocks: &BasicBlocks<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: &Instance<'tyctx>,
    method: &Body<'tyctx>,
) -> Option<Handler> {
    let term = block_data.terminator.as_ref()?;
    let unwind = term.unwind()?;
    Some(Handler::RawID(simplify_handler(
        handler_from_action(*unwind),
        blocks,
        tyctx,
        method_instance,
        method,
    )?))
}
#[allow(clippy::match_same_arms)]
fn simplify_handler<'tyctx>(
    handler: Option<u32>,
    blocks: &BasicBlocks<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method_instance: &Instance<'tyctx>,
    method: &Body<'tyctx>,
) -> Option<u32> {
    let handler = handler?;
    if !blocks[handler.into()].statements.is_empty() {
        return Some(handler);
    }
    match blocks[handler.into()].terminator.as_ref()?.kind {
        TerminatorKind::Goto { target } => simplify_handler(
            Some(target.as_u32()),
            blocks,
            tyctx,
            method_instance,
            method,
        ),
        // Reaching Unreachable is UB, so we can do whatever, including doing nothing :).
        TerminatorKind::UnwindResume | TerminatorKind::Unreachable => None,
        TerminatorKind::Return => panic!("Interal error: cleanup(unwind) block returns!"),
        // This block drops, so we **have** to execute it
        TerminatorKind::Drop {
            place,
            target,
            unwind: _,
            replace: _,
        } => {
            let ty =
                crate::utilis::monomorphize(method_instance, place.ty(method, tyctx).ty, tyctx);

            let drop_instance = Instance::resolve_drop_in_place(tyctx, ty).polymorphize(tyctx);
            if let InstanceDef::DropGlue(_, None) = drop_instance.def {
                //Empty drop, nothing needs to happen.
                simplify_handler(
                    Some(target.as_u32()),
                    blocks,
                    tyctx,
                    method_instance,
                    method,
                )
            } else {
                Some(handler)
            }
        }
        TerminatorKind::CoroutineDrop { .. } => Some(handler),
        // This block calls, so we **have** to execute it
        // TODO: consider checking if this call has side effects!
        TerminatorKind::Call { .. } => Some(handler),
        // This block asserts, so it *could* double-panics, so we **have** to execute it
        TerminatorKind::Assert { .. } => Some(handler),
        TerminatorKind::Yield { .. } => {
            panic!("Interal error: cleanup(unwind) block yelds(returns)!")
        }
        // False targets should not be present.
        TerminatorKind::FalseEdge { .. } | TerminatorKind::FalseUnwind { .. } => {
            panic!("False bb termiantor after drop elaboration!")
        }
        // Iniline ASM could do **anything** so it can never be skipped.
        TerminatorKind::InlineAsm { .. } => Some(handler),
        // We *don't* know which target is taken, so we can't skip it
        // TODO: consider checking all sub-targets and removing impossible ones?
        TerminatorKind::SwitchInt { .. } => Some(handler),
        // We can't skip a termiantor which aborts.
        TerminatorKind::UnwindTerminate(_) => Some(handler),
    }
}
/// Convert an `UnwindAction` into an id of the block this will jump into during an exception.
pub(crate) fn handler_from_action(action: UnwindAction) -> Option<u32> {
    match action {
        UnwindAction::Continue => None,
        UnwindAction::Cleanup(handler) => Some(handler.as_u32()),
        // This is triggered during double panics and panic corssing FFI boundaries.
        // TODO: This is incorrect, since it does nothing when it should terminate this program.
        UnwindAction::Terminate(_reason) => None,
        // Reaching this is UB, so we can do whatever here
        // continuing unwinding seems like an OK option.
        UnwindAction::Unreachable => None,
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
        self.trees = self
            .trees
            .clone()
            .into_iter()
            .flat_map(super::cil_tree::CILTree::shed_trees)
            .collect();
        if let Some(handler) = self.handler.as_mut() {
            handler
                .as_blocks_mut()
                .unwrap()
                .iter_mut()
                .for_each(BasicBlock::sheed_trees);
        }
    }
    pub(crate) fn resolve_exception_handlers(&mut self, handler_bbs: &[BasicBlock]) {
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
            BasicBlock::new(
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
        for (target, sub_target) in self.targets() {
            assert_eq!(sub_target, 0);
            self.trees.push(
                CILRoot::JumpingPad {
                    ops: Box::new([CILOp::Label(id, target), CILOp::Leave(target)]),
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
    fn flatten_inner(&self, id: u32, sub_id: u32) -> Vec<CILOp> {
        let mut ops = vec![CILOp::Label(id, sub_id)];
        if self.handler.is_some() {
            ops.push(CILOp::BeginTry);
        };
        ops.extend(
            self.trees
                .iter()
                .flat_map(super::cil_tree::CILTree::into_ops),
        );
        if let Some(handler) = &self.handler {
            ops.push(CILOp::BeginCatch);
            ops.push(CILOp::Pop);
            let Handler::Blocks(blocks) = handler else {
                panic!("Unresolved eception handler blocks!")
            };
            for block in blocks {
                ops.extend(block.flatten_inner(self.id, block.id));
            }
            ops.push(CILOp::EndTry);
        }
        ops
    }
    /// Converts this basic block into a list of ops.
    #[must_use]
    pub fn into_ops(&self) -> Vec<CILOp> {
        self.flatten_inner(self.id(), 0)
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
    /// Returns a reference to the trees that make up this block.
    #[must_use]
    pub fn trees(&self) -> &[CILTree] {
        &self.trees
    }
}
