use crate::r#type::tycache::TyCache;
use cilly::{
    basic_block::BasicBlock, method::Method, Type
};
use rustc_middle::ty::TyCtxt;

   
    pub fn maxstack(method:&Method) -> usize {
        crate::utilis::max_stack(
            method.blocks()
                .iter()
                .flat_map(crate::basic_block::into_ops)
                .collect::<Vec<_>>()
                .as_ref(),
            *method.sig().output() == Type::Void,
        ) + 10
    }
    
    pub(crate) fn resolve_global_allocations(
        method:&mut Method,
        arg: &mut crate::assembly::Assembly,
        tyctx: TyCtxt,
        tycache: &mut TyCache,
    ) {
        method.blocks_mut()
            .iter_mut()
            .flat_map(BasicBlock::trees_mut)
            .for_each(|tree| {
                crate::cil_tree::resolve_global_allocations_tree(tree, arg, tyctx, tycache)
            });
    }
    
