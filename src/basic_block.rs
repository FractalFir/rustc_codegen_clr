use cilly::basic_block::{BasicBlock, Handler};
use rustc_middle::mir::BasicBlockData;
use rustc_middle::mir::UnwindAction;
use rustc_middle::{
    mir::{BasicBlocks, Body, TerminatorKind},
    ty::{Instance, InstanceDef, TyCtxt},
};

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
//  We match same arms on purpose here.
#[allow(clippy::match_same_arms)]
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
