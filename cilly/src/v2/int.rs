use serde::{Deserialize, Serialize};

use super::{cilnode::MethodKind, Assembly, CILNode, ClassRef, Const, MethodRef, Type};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Int {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}
impl From<Int> for Type {
    fn from(value: Int) -> Self {
        Self::Int(value)
    }
}
impl Int {
    /// Returns the minimum value of this int.
    pub fn min(&self, asm: &mut Assembly) -> CILNode {
        match self {
            Int::U8 => Const::U8(u8::MIN).into(),
            Int::U16 => Const::U16(u16::MIN).into(),
            Int::U32 => Const::U32(u32::MIN).into(),
            Int::U64 => Const::U64(u64::MIN).into(),
            Int::U128 => {
                let min_value = asm.alloc_string("get_MinValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::uint_128(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        min_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
            Int::USize => {
                let min_value = asm.alloc_string("get_MinValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::usize_type(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        min_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
            Int::I8 => Const::I8(i8::MIN).into(),
            Int::I16 => Const::I16(i16::MIN).into(),
            Int::I32 => Const::I32(i32::MIN).into(),
            Int::I64 => Const::I64(i64::MIN).into(),
            Int::I128 => {
                let min_value = asm.alloc_string("get_MinValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::uint_128(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        min_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
            Int::ISize => {
                let min_value = asm.alloc_string("get_MinValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::isize_type(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        min_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
        }
    }
    /// Returns the maximum value of this int.
    pub fn max(&self, asm: &mut Assembly) -> CILNode {
        match self {
            Int::U8 => Const::U8(u8::MAX).into(),
            Int::U16 => Const::U16(u16::MAX).into(),
            Int::U32 => Const::U32(u32::MAX).into(),
            Int::U64 => Const::U64(u64::MAX).into(),
            Int::U128 => {
                let max_value = asm.alloc_string("get_MaxValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::uint_128(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        max_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
            Int::USize => {
                let max_value = asm.alloc_string("get_MaxValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::usize_type(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        max_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
            Int::I8 => Const::I8(i8::MAX).into(),
            Int::I16 => Const::I16(i16::MAX).into(),
            Int::I32 => Const::I32(i32::MAX).into(),
            Int::I64 => Const::I64(i64::MAX).into(),
            Int::I128 => {
                let max_value = asm.alloc_string("get_MaxValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::uint_128(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        max_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
            Int::ISize => {
                let max_value = asm.alloc_string("get_MaxValue");
                let sig = asm.sig([], Type::Int(*self));
                let class = ClassRef::isize_type(asm);
                CILNode::Call(Box::new((
                    asm.alloc_methodref(MethodRef::new(
                        class,
                        max_value,
                        sig,
                        MethodKind::Static,
                        [].into(),
                    )),
                    [].into(),
                )))
            }
        }
    }
    /// Returns a short name of this int.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Int::U8 => "u8",
            Int::U16 => "u16",
            Int::U32 => "u32",
            Int::U64 => "u64",
            Int::U128 => "u128",
            Int::USize => "usize",
            Int::I8 => "i8",
            Int::I16 => "i16",
            Int::I32 => "i32",
            Int::I64 => "i64",
            Int::I128 => "i128",
            Int::ISize => "isize",
        }
    }
    /// Checks if this int is signed.
    #[must_use]
    pub fn is_signed(&self) -> bool {
        match self {
            Int::U8 | Int::U16 | Int::U32 | Int::U64 | Int::U128 | Int::USize => false,
            Int::I8 | Int::I16 | Int::I32 | Int::I64 | Int::I128 | Int::ISize => true,
        }
    }
}
