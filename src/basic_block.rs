// This is a WIP module an is currently unused
#![allow(unused)]
#![allow(clippy::all)]
use crate::cil::CILOp;
use rustc_middle::mir::UnwindAction;
#[derive(Debug)]
struct BasicBlock {
    unwind_action: UnwindAction,
    ops: Vec<CILOp>,
}
impl BasicBlock {
    pub fn new(ops: Vec<CILOp>, unwind_action: UnwindAction) -> Self {
        Self { unwind_action, ops }
    }
    pub fn may_merge(&self, other: &Self) -> bool {
        match (self.unwind_action, other.unwind_action) {
            (UnwindAction::Continue, UnwindAction::Continue) => true,
            (UnwindAction::Continue, UnwindAction::Unreachable)
            | (UnwindAction::Unreachable, UnwindAction::Continue) => true,
            (UnwindAction::Unreachable, UnwindAction::Unreachable) => true,
            (UnwindAction::Cleanup(a), UnwindAction::Cleanup(b)) => a == b,
            (UnwindAction::Terminate(_), UnwindAction::Terminate(_)) => true,
            (_, _) => false,
        }
    }
    pub fn into_ops(bbs: &[Self]) -> Vec<CILOp> {
        todo!("Can't convert basic blocks {bbs:?} into ops yet!");
    }
}
#[test]
fn exceptions() {
    use crate::{cil::CallSite, function_sig::FnSig, r#type::Type};
    let bb0 = BasicBlock::new(
        vec![
            CILOp::LDArg(0),
            CILOp::LDArg(1),
            CILOp::Add,
            CILOp::Dup,
            CILOp::STLoc(0),
            CILOp::Call(CallSite::boxed(
                None,
                "malloc".into(),
                FnSig::new(&[Type::ISize], &Type::Ptr(Type::Void.into())),
                true,
            )),
            CILOp::GoTo(1),
        ],
        UnwindAction::Continue,
    );
    let bb1 = BasicBlock::new(
        vec![
            CILOp::STLoc(1),
            CILOp::LDLoc(1),
            CILOp::Call(CallSite::boxed(
                None,
                "do_sth_with_a_buff".into(),
                FnSig::new(&[Type::Ptr(Type::Void.into())], &Type::Void),
                true,
            )),
            CILOp::GoTo(2),
        ],
        UnwindAction::Cleanup(3_u32.into()),
    );
    let bb2 = BasicBlock::new(
        vec![
            CILOp::LDLoc(1),
            CILOp::Call(CallSite::boxed(
                None,
                "free".into(),
                FnSig::new(&[Type::Ptr(Type::Void.into())], &Type::Void),
                true,
            )),
            CILOp::Ret,
        ],
        UnwindAction::Continue,
    );
    let bb3 = BasicBlock::new(
        vec![
            CILOp::LDLoc(1),
            CILOp::Call(CallSite::boxed(
                None,
                "free".into(),
                FnSig::new(&[Type::Ptr(Type::Void.into())], &Type::Void),
                true,
            )),
            CILOp::Rethrow,
        ],
        UnwindAction::Continue,
    );
    let bbs = vec![bb0, bb1, bb2, bb3];
    let ops = BasicBlock::into_ops(&bbs);
}
