use crate::{cil::{CILOp, CallSite, FieldDescriptor}, cil_tree::cil_node::CILNode};

use super::append_vec;
#[derive(Clone,Debug)]
pub(crate) enum CILRoot {
    STLoc { local: u32, tree: CILNode },
    BTrue { target: u32, ops: CILNode },
    GoTo { target: u32 },
    Call{site:CallSite,args:Box<[CILNode]>},
    SetField{addr:CILNode,value:CILNode,desc:FieldDescriptor},
}
impl CILRoot {
    pub fn flatten(&self) -> Vec<CILOp> {
        match self {
            Self::STLoc { local, tree } => append_vec(tree.flatten(), CILOp::STLoc(*local)),
            Self::BTrue { target, ops } => append_vec(ops.flatten(), CILOp::BTrue(*target)),
            Self::GoTo { target } => vec![CILOp::GoTo(*target)],
            Self::Call{site,args}=>{
                let mut args:Vec<_> = args.iter().flat_map(|arg|arg.flatten()).collect();
                args.push(CILOp::Call(site.clone().into()));
                args
            },
            Self::SetField{addr,value: root,desc}=>{
                let mut res = addr.flatten();
                res.extend(root.flatten());
                res.push(CILOp::STField(desc.clone().into()));
                res
            }
        }
    }
}
