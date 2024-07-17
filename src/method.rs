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
    method
        .blocks_mut()
        .iter_mut()
        .flat_map(BasicBlock::iter_cil_mut)
        .for_each(|elem| {
            if let CILIterElemMut::Node(node) = elem {
                match node {
                    CILNode::LoadGlobalAllocPtr { alloc_id } => {
                        *node =
                            crate::assembly::add_allocation(asm, *alloc_id, tcx, tycache).into();
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
