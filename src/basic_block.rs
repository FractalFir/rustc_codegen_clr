use rustc_middle::mir::UnwindAction;
use crate::{cil::CILOp, cil_tree::CILTree};

#[derive(Clone,Debug)]
pub struct BasicBlock{
    trees:Vec<CILTree>,
    id:u32,
    handler:Option<u32>,
}
pub fn handler_for_block(block_data:&rustc_middle::mir::BasicBlockData)->Option<u32>{
    let term = (&block_data.terminator).as_ref()?;
    let unwind = term.unwind()?;
    handler_from_action(unwind)
}
pub fn handler_from_action(action:&UnwindAction)->Option<u32>{
    match action{
        UnwindAction::Continue => None,
        UnwindAction:: Cleanup(handler)=>Some(handler.as_u32()),
        // This is triggered during double panics and panic corssing FFI boundaries. 
        // TODO: This is incorrect, since it does nothing when it should terminate this program.
        UnwindAction:: Terminate(_reason)=>None,
        // Reaching this is UB, so we can do whatever here
        // continuing unwinding seems like an OK option.
        UnwindAction::Unreachable => None,
    }
}
impl BasicBlock{
    pub fn new(trees: Vec<CILTree>, id: u32, handler: Option<u32>) -> Self {
        Self { trees, id, handler }
    }

    pub fn flatten(&self) -> Vec<CILOp> {
        let mut ops = vec![CILOp::BlockStart(self.id)];
        ops.extend(self.trees.iter().flat_map(|tree|tree.flatten()));
        ops.push(CILOp::BlockEnd(self.id));
        ops
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn handler(&self) -> Option<u32> {
        self.handler
    }
    
}