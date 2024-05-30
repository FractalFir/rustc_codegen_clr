use crate::r#type::tycache::TyCache;
use cilly::{asm::Assembly, cil_iter_mut::CILIterElemMut, cil_node::CILNode, method::Method};
use rustc_middle::ty::TyCtxt;

pub(crate) fn resolve_global_allocations(
    method: &mut Method,
    asm: &mut Assembly,
    tyctx: TyCtxt,
    tycache: &mut TyCache,
) {
    method
        .blocks_mut()
        .iter_mut()
        .flat_map(|block| block.iter_cil_mut())
        .for_each(|elem| match elem {
            CILIterElemMut::Node(node) => match node {
                CILNode::LoadGlobalAllocPtr { alloc_id } => {
                    *node = CILNode::LDStaticField(
                        crate::assembly::add_allocation(asm, *alloc_id, tyctx, tycache).into(),
                    )
                }
                CILNode::PointerToConstValue(bytes) => {
                    *node = CILNode::LDStaticField(Box::new(crate::assembly::add_const_value(
                        asm, *bytes, tyctx,
                    )))
                }
                _ => (),
            },
            _ => (),
        });
}
