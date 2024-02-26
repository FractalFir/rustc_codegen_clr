use crate::{
    cil::CILOp,
    cil_tree::{cil_root::CILRoot, CILTree},
};
use rustc_middle::{
    mir::{Body,BasicBlocks, Operand, SwitchTargets, Terminator, TerminatorKind},
    ty::{Instance, Ty, TyCtxt, TyKind,InstanceDef},
};
use rustc_middle::mir::UnwindAction;
use rustc_middle::mir::BasicBlockData;
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

pub fn handler_for_block<'tyctx>(block_data: &BasicBlockData,blocks:&BasicBlocks<'tyctx>,tyctx:TyCtxt<'tyctx>,method_instance:&Instance<'tyctx>,method:&Body<'tyctx>) -> Option<Handler> {
    let term = (&block_data.terminator).as_ref()?;
    let unwind = term.unwind()?;
    Some(Handler::RawID(simplify_handler(handler_from_action(unwind),blocks,tyctx,method_instance,method)?))
}
fn simplify_handler<'tyctx>(handler:Option<u32>,blocks:&BasicBlocks<'tyctx>,tyctx:TyCtxt<'tyctx>,method_instance:&Instance<'tyctx>,method:&Body<'tyctx>)->Option<u32>{
    let handler = handler?;
    if !blocks[handler.into()].statements.is_empty(){
        return Some(handler);
    }
    match (&blocks[handler.into()]).terminator.as_ref()?.kind{
        TerminatorKind::Goto { target }=> simplify_handler(Some(target.as_u32()), blocks,tyctx,method_instance,method),
        TerminatorKind::UnwindResume => None,
        TerminatorKind::Return=> panic!("Interal error: cleanup(unwind) block returns!"),
        // Reaching this is UB, so we can do whatever, including doing nothing :).
        TerminatorKind::Unreachable => None,
        // This block drops, so we **have** to execute it
        TerminatorKind::Drop{place,
        target,
        unwind: _,
        replace: _,
    } => {
        let ty = crate::utilis::monomorphize(&method_instance, place.ty(method, tyctx).ty, tyctx);

        let drop_instance = Instance::resolve_drop_in_place(tyctx, ty).polymorphize(tyctx);
        if let InstanceDef::DropGlue(_, None) = drop_instance.def {
            //Empty drop, nothing needs to happen.
            simplify_handler(Some(target.as_u32()), blocks,tyctx,method_instance,method)
        }
        else{
            Some(handler)
        }
    }
        TerminatorKind::CoroutineDrop{..} => Some(handler),
        // This block calls, so we **have** to execute it
        // TODO: consider checking if this call has side effects!
        TerminatorKind::Call { .. } => Some(handler),
        // This block asserts, so it *could* double-panics, so we **have** to execute it
        TerminatorKind::Assert{..} => Some(handler),
        TerminatorKind::Yield {..}=> panic!("Interal error: cleanup(unwind) block yelds(returns)!"),
        // False targets should not be present.
        TerminatorKind::FalseEdge{..} | TerminatorKind::FalseUnwind{..} => panic!("False bb termiantor after drop elaboration!"),
        // Iniline ASM could do **anything** so it can never be skipped. 
        TerminatorKind::InlineAsm{..}=> Some(handler),
        // We *don't* know which target is taken, so we can't skip it
        // TODO: consider checking all sub-targets and removing impossible ones?
        TerminatorKind::SwitchInt { .. } =>Some(handler),
        // We can't skip a termiantor which aborts. 
        TerminatorKind::UnwindTerminate(_) => Some(handler),
    }
}
pub fn handler_from_action(action: &UnwindAction) -> Option<u32> {
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
