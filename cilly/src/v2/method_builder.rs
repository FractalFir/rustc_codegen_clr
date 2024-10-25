use std::ops::{Deref, DerefMut};

use super::{
    basic_block::BlockId, method::LocalId, Assembly, BasicBlock, IntoAsmIndex, MethodDef,
    MethodImpl, StringIdx, TypeIdx,
};

pub struct MethodBuilder<'asm> {
    asm: &'asm mut Assembly,
    def: MethodDef,
    curr_block: BlockId,
}
impl Deref for MethodBuilder<'_> {
    type Target = Assembly;

    fn deref(&self) -> &Self::Target {
        self.asm
    }
}
impl DerefMut for MethodBuilder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.asm
    }
}
impl MethodBuilder<'_> {
    pub fn new_block(&mut self) -> BlockId {
        let MethodImpl::MethodBody { blocks, locals: _ } = self.def.implementation_mut() else {
            panic!("Attempted to add a new basic block to a method with an invalid or unresolved body:{:?},",self.def.implementation());
        };
        let block_id: BlockId = blocks.len().try_into().expect("Block cap exceeded!");
        blocks.push(BasicBlock::new(vec![], block_id, None));
        self.curr_block = block_id;
        block_id
    }
    pub fn tmp_local(
        &mut self,
        name: Option<impl IntoAsmIndex<StringIdx>>,
        tpe: impl IntoAsmIndex<TypeIdx>,
    ) -> LocalId {
        let name = name.map(|inner| inner.into_idx(self));
        let tpe = tpe.into_idx(self);
        let MethodImpl::MethodBody { blocks: _, locals } = self.def.implementation_mut() else {
            panic!("Attempted to add a local variable a method with an invalid or unresolved body:{:?},",self.def.implementation());
        };
        let new_local = locals.len();
        locals.push((name, tpe));
        new_local.try_into().expect("Local variable cap exceeded.")
    }
}
