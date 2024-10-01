use serde::{Deserialize, Serialize};

use super::{
    cilnode::MethodKind, Assembly, CILNode, ClassRef, ClassRefIdx, Const, MethodRef, Type,
};

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
    /// Returns a reference to a class representing this type.
    pub fn class(&self, asm: &mut Assembly) -> ClassRefIdx {
        match self {
            Int::U8 => ClassRef::byte(asm),
            Int::U16 => ClassRef::uint16(asm),
            Int::U32 => ClassRef::uint32(asm),
            Int::U64 => ClassRef::uint64(asm),
            Int::U128 => ClassRef::uint_128(asm),
            Int::USize => ClassRef::usize_type(asm),
            Int::I8 => ClassRef::sbyte(asm),
            Int::I16 => ClassRef::int16(asm),
            Int::I32 => ClassRef::int32(asm),
            Int::I64 => ClassRef::int64(asm),
            Int::I128 => ClassRef::int_128(asm),
            Int::ISize => ClassRef::isize_type(asm),
        }
    }
    /// Returns the unsigned version of this type.
    /// ```
    /// # use cilly::Int;
    /// assert_eq!(Int::I8.as_unsigned(),Int::U8);
    /// assert_eq!(Int::I128.as_unsigned(),Int::U128.as_unsigned());
    /// ```
    pub fn as_unsigned(&self) -> Self {
        match self {
            Int::I8 | Int::U8 => Int::U8,
            Int::I16 | Int::U16 => Int::U16,
            Int::I32 | Int::U32 => Int::U32,
            Int::I64 | Int::U64 => Int::U64,
            Int::I128 | Int::U128 => Int::U128,
            Int::ISize | Int::USize => Int::USize,
        }
    }
    /// Returns the signed version of this type.
    /// ```
    /// # use cilly::Int;
    /// assert_eq!(Int::U8.as_signed(),Int::I8);
    /// assert_eq!(Int::U128.as_signed(),Int::I128.as_signed());
    /// ```
    pub fn as_signed(&self) -> Self {
        match self {
            Int::I8 | Int::U8 => Int::I8,
            Int::I16 | Int::U16 => Int::I16,
            Int::I32 | Int::U32 => Int::I32,
            Int::I64 | Int::U64 => Int::I64,
            Int::I128 | Int::U128 => Int::I128,
            Int::ISize | Int::USize => Int::ISize,
        }
    }

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
    /// Returns the size of this int in byte, or None if the int is `isize` or `usize`
    pub fn size(&self) -> Option<u8> {
        match self {
            Int::U8 | Int::I8 => Some(1),
            Int::U16 | Int::I16 => Some(2),
            Int::U32 | Int::I32 => Some(4),
            Int::U64 | Int::I64 => Some(8),
            Int::U128 | Int::I128 => Some(16),
            Int::USize | Int::ISize => None,
        }
    }
}
#[test]
fn is_signed() {
    let unsigned = [Int::U8, Int::U16, Int::U32, Int::U64, Int::U128, Int::USize];
    let signed = [Int::I8, Int::I16, Int::I32, Int::I64, Int::I128, Int::ISize];
    for signed in signed {
        assert!(signed.is_signed());
    }
    for unsigned in unsigned {
        assert!(!unsigned.is_signed());
    }
}
#[test]
fn name() {
    assert_eq!(Int::USize.name(), "usize");
    assert_eq!(Int::ISize.name(), "isize");
    assert_eq!(Int::U128.name(), "u128");
    assert_eq!(Int::I128.name(), "i128");
    assert_eq!(Int::U64.name(), "u64");
    assert_eq!(Int::I64.name(), "i64");
    assert_eq!(Int::U32.name(), "u32");
    assert_eq!(Int::I32.name(), "i32");
    assert_eq!(Int::U16.name(), "u16");
    assert_eq!(Int::I16.name(), "i16");
    assert_eq!(Int::U8.name(), "u8");
    assert_eq!(Int::I8.name(), "i8");
}
