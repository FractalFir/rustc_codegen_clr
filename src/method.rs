use std::ops::DerefMut;

use crate::r#type::tycache::TyCache;
use cilly::{
    asm::Assembly, basic_block::BasicBlock, cil_iter_mut::CILIterElemMut, cil_node::CILNode,
    method::Method,
};
use rustc_middle::ty::TyCtxt;

pub(crate) fn resolve_global_allocations(
    method: &mut Method,
    asm: &mut Assembly,
    tcx: TyCtxt,
    tycache: &mut TyCache,
) {
    let mut blocks = method.blocks_mut();
    let mut tmp: Vec<_> = blocks
        .iter_mut()
        .flat_map(|block| block.tree_iter())
        .map(|tree| tree.root_mut())
        .collect();
    tmp.iter_mut()
        .flat_map(|root| root.deref_mut().into_iter())
        .for_each(|elem| {
            if let CILIterElemMut::Node(node) = elem {
                match node {
                    CILNode::LoadGlobalAllocPtr { alloc_id } => {
                        *node = crate::assembly::add_allocation(asm, *alloc_id, tcx, tycache);
                    }
                    CILNode::PointerToConstValue(bytes) => {
                        *node = CILNode::LDStaticField(Box::new(crate::assembly::add_const_value(
                            asm, **bytes, tcx,
                        )));
                    }
                    _ => (),
                }
            }
        });
}
