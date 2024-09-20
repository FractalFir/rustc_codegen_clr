use std::ops::DerefMut;

use cilly::{cil_iter_mut::CILIterElemMut, cil_node::CILNode, method::Method};

use crate::fn_ctx::MethodCompileCtx;

pub(crate) fn resolve_global_allocations(method: &mut Method, ctx: &mut MethodCompileCtx<'_, '_>) {
    let mut blocks = method.blocks_mut();
    let mut tmp: Vec<_> = blocks
        .iter_mut()
        .flat_map(cilly::basic_block::BasicBlock::tree_iter)
        .map(cilly::cil_tree::CILTree::root_mut)
        .collect();
    tmp.iter_mut()
        .flat_map(|root| root.deref_mut().into_iter())
        .for_each(|elem| {
            if let CILIterElemMut::Node(node) = elem {
                match node {
                    CILNode::LoadGlobalAllocPtr { alloc_id } => {
                        let (tcx, asm) = ctx.tcx_and_asm();
                        *node = crate::assembly::add_allocation(*alloc_id, asm, tcx);
                    }
                    CILNode::PointerToConstValue(bytes) => {
                        let asm = ctx.asm_mut();
                        *node = CILNode::AddressOfStaticField(Box::new(
                            crate::assembly::add_const_value(asm, **bytes),
                        ));
                    }
                    _ => (),
                }
            }
        });
}
