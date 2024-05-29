use crate::r#type::tycache::TyCache;
use cilly::{asm::Assembly, basic_block::BasicBlock, method::Method, Type};
use rustc_middle::ty::TyCtxt;


pub(crate) fn resolve_global_allocations(
    method: &mut Method,
    arg: &mut Assembly,
    tyctx: TyCtxt,
    tycache: &mut TyCache,
) {
    method
        .blocks_mut()
        .iter_mut()
        .flat_map(BasicBlock::trees_mut)
        .for_each(|tree| {
            crate::cil_tree::resolve_global_allocations_tree(tree, arg, tyctx, tycache)
        });
}
