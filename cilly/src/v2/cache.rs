/*
use std::marker::PhantomData;

use super::{bimap::IntoBiMapIndex, Assembly, NodeIdx};

/// A cached information about some data.
#[derive(Clone)]
pub struct CachedAssemblyInfo<Key, Cached, Function> {
    data: Vec<Option<Cached>>,
    function: PhantomData<Function>,
    key: PhantomData<Key>,
}
impl<Key, Cached, Function> Default for CachedAssemblyInfo<Key, Cached, Function> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            function: Default::default(),
            key: Default::default(),
        }
    }
}
impl<Key: IntoBiMapIndex, Cached: Clone, Function: CachedAssemblyFunction<Key, Cached>>
    CachedAssemblyInfo<Key, Cached, Function>
{
    pub fn get(
        &mut self,
        cached: &mut CachedAssemblyInfo<Key, Cached, Function>,
        asm: &mut Assembly,
        key: Key,
    ) -> Cached {
        let index = key.as_bimap_index().get() as usize - 1;
        // 1st expand the internal data if that is needed
        if self.data.len() <= index {
            self.data.extend((self.data.len()..=index).map(|_| None));
        }
        assert!(!self.data.len() <= index);
        match &self.data[index] {
            Some(val) => val.clone(),
            None => {
                self.data[index] = Some(CachedAssemblyFunction::not_in_cache(cached, asm, key));
                self.data[index].clone().unwrap()
            }
        }
    }
}
pub trait CachedAssemblyFunction<Key: IntoBiMapIndex, Cached>: Sized {
    fn not_in_cache(
        cached: &mut CachedAssemblyInfo<Key, Cached, Self>,
        asm: &mut Assembly,
        key: Key,
    ) -> Cached;
}
/// Cached information about the stack usage of a [`CILNode`]
#[derive(Clone, Copy)]
pub struct StackUsage;
impl CachedAssemblyFunction<NodeIdx, NonMaxU32> for StackUsage {
    fn not_in_cache(
        cached: &mut CachedAssemblyInfo<NodeIdx, NonMaxU32, Self>,
        asm: &mut Assembly,
        key: NodeIdx,
    ) -> NonMaxU32 {
        match asm.get_node(key) {
            super::CILNode::Const(_) => todo!(),
            super::CILNode::BinOp(_, _, _) => todo!(),
            super::CILNode::UnOp(_, _) => todo!(),
            super::CILNode::LdLoc(_)
            | super::CILNode::LdLocA(_)
            | super::CILNode::LdArg(_)
            | super::CILNode::LdArgA(_) => NonMaxU32::new(1),
            super::CILNode::Call(_) => todo!(),
            super::CILNode::IntCast {
                input,
                target,
                extend,
            } => todo!(),
            super::CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => todo!(),
            super::CILNode::RefToPtr(_) => todo!(),
            super::CILNode::PtrCast(_, _) => todo!(),
            super::CILNode::LdFieldAdress { addr, field } => todo!(),
            super::CILNode::LdField { addr, field } => todo!(),
            super::CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => todo!(),
            super::CILNode::SizeOf(_) => NonMaxU32::new(1),
            super::CILNode::GetException => NonMaxU32::new(1),
            super::CILNode::IsInst(_, _) => todo!(),
            super::CILNode::CheckedCast(_, _) => todo!(),
            super::CILNode::CallI(_) => todo!(),
            super::CILNode::LocAlloc { size } => todo!(),
            super::CILNode::LdStaticField(_) => todo!(),
            super::CILNode::LdFtn(_) => todo!(),
            super::CILNode::LdTypeToken(_) => todo!(),
            super::CILNode::LdLen(_) => todo!(),
            super::CILNode::LocAllocAlgined { tpe, align } => todo!(),
            super::CILNode::LdElelemRef { array, index } => todo!(),
            super::CILNode::UnboxAny { object, tpe } => todo!(),
        }
    }
}
/// A U32 intiger, which is not u32::MAX.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NonMaxU32(std::num::NonZeroU32);
impl NonMaxU32 {
    pub fn new(val: u32) -> Self {
        assert_ne!(val, u32::MAX);
        NonMaxU32(std::num::NonZeroU32::new(val - 1).unwrap())
    }
    pub fn get(self) -> u32 {
        self.0.get() + 1
    }
}
*/
