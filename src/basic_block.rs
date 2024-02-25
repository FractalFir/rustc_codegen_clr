use crate::{
    cil::CILOp,
    cil_tree::{cil_root::CILRoot, CILTree},
};
use rustc_middle::mir::UnwindAction;

#[derive(Clone, Debug)]
pub struct BasicBlock {
    trees: Vec<CILTree>,
    id: u32,
    handler: Option<Handler>,
}
#[derive(Clone, Debug)]
pub enum Handler {
    RawID(u32),
    Blocks(Vec<BasicBlock>),
}

pub fn handler_for_block(block_data: &rustc_middle::mir::BasicBlockData) -> Option<Handler> {
    let term = (&block_data.terminator).as_ref()?;
    let unwind = term.unwind()?;
    handler_from_action(unwind)
}
pub fn handler_from_action(action: &UnwindAction) -> Option<Handler> {
    match action {
        UnwindAction::Continue => None,
        UnwindAction::Cleanup(handler) => Some(Handler::RawID(handler.as_u32())),
        // This is triggered during double panics and panic corssing FFI boundaries.
        // TODO: This is incorrect, since it does nothing when it should terminate this program.
        UnwindAction::Terminate(_reason) => None,
        // Reaching this is UB, so we can do whatever here
        // continuing unwinding seems like an OK option.
        UnwindAction::Unreachable => None,
    }
}
impl BasicBlock {
    pub fn resolve_exception_handlers(&mut self, handler_bbs: &[BasicBlock]) {
        let handler = if let Some(handler) = &self.handler {
            handler
        } else {
            return;
        };
        let handler_id = if let Handler::RawID(handler_id) = handler {
            handler_id
        } else {
            panic!("Tired to double-resolve ");
        };
        //let handler_entrypoint = handler_bbs.iter().find(|bb|bb.id == *handler_id).unwrap().clone();
        let mut handler = Vec::new();
        handler.push(BasicBlock::new(
            vec![CILRoot::GoTo {
                target: self.id(),
                sub_target: *handler_id,
            }
            .into()],
            u32::MAX,
            None,
        ));
        // Fix up handler jumps
        for bb in handler_bbs {
            let mut cloned = bb.clone();
            cloned
                .trees
                .iter_mut()
                .for_each(|tree| tree.fix_for_exception_handler(self.id()));
            handler.push(cloned);
        }
        // Get cross-block jumps
        let mut targets = Vec::new();
        self.trees
            .iter()
            .for_each(|tree| tree.targets(&mut targets));
        // Generate launching pads for cross-block branches!
        let id = self.id();
        for (target, sub_target) in targets {
            assert_eq!(sub_target, 0);
            self.trees.push(
                CILRoot::Raw {
                    ops: Box::new([CILOp::Label(id, target), CILOp::Leave(target)]),
                }
                .into(),
            )
        }
        // Change branches to use lanuching pads.

        self.trees
            .iter_mut()
            .for_each(|tree| tree.fix_for_exception_handler(id));
        self.handler = Some(Handler::Blocks(handler));
    }
    pub fn new(trees: Vec<CILTree>, id: u32, handler: Option<Handler>) -> Self {
        Self { trees, id, handler }
    }
    pub fn flatten_inner(&self, id: u32, sub_id: u32) -> Vec<CILOp> {
        let mut ops = vec![CILOp::Label(id, sub_id)];
        if let Some(_) = self.handler {
            ops.push(CILOp::BeginTry);
        };
        ops.extend(self.trees.iter().flat_map(|tree| tree.flatten()));
        if let Some(handler) = &self.handler {
            ops.push(CILOp::BeginCatch);
            ops.push(CILOp::Pop);
            let blocks = if let Handler::Blocks(blocks) = handler {
                blocks
            } else {
                panic!("Unresolved eception handler blocks!")
            };
            for block in blocks {
                ops.extend(block.flatten_inner(self.id, block.id));
            }
            ops.push(CILOp::EndTry)
        }
        ops
    }
    pub fn flatten(&self) -> Vec<CILOp> {
        self.flatten_inner(self.id(), 0)
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
