use serde::{Deserialize, Serialize};

use super::{
    hashable::{HashableF32, HashableF64},
    CILNode, ClassRefIdx, Float, Int, StringIdx, Type,
};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
/// A constant cillyIR value.
pub enum Const {
    /// Constant i8 value.
    I8(i8),
    /// Constant i16 value.
    I16(i16),
    /// Constant i32 value.
    I32(i32),
    /// Constant i64 value.
    I64(i64),
    /// Constant i128 value.
    I128(i128),
    /// Constant isize value.
    ISize(i64),
    /// Constant u8 value.
    U8(u8),
    /// Constant u16 value.
    U16(u16),
    /// Constant u32 value.
    U32(u32),
    /// Constant u64 value.
    U64(u64),
    /// Constant u128 value.
    U128(u128),
    /// Constant usize value.
    USize(u64),
    /// A reference to an immutable, platform-specific representation of a string.
    /// There is no guarrantees about the encoding of this type.
    PlatformString(StringIdx),
    /// A boolean value
    Bool(bool),
    /// A representation of a single-precision floating-point value. No guarateess are given about the exact bitpattern of NaNs.  
    F32(HashableF32),
    /// A representation of a double-precision floating-point value. No guarateess are given about the exact bitpattern of NaNs.
    F64(HashableF64),
    /// A "null" reference to a platform-specifc managed object of type `class`.
    Null(ClassRefIdx),
}
impl Const {
    /// Retrives the type of this value.
    pub fn get_type(&self) -> Type {
        match self {
            Const::I8(_) => Type::Int(Int::I8),
            Const::I16(_) => Type::Int(Int::I16),
            Const::I32(_) => Type::Int(Int::I32),
            Const::I64(_) => Type::Int(Int::I64),
            Const::I128(_) => Type::Int(Int::I128),
            Const::ISize(_) => Type::Int(Int::ISize),
            Const::U8(_) => Type::Int(Int::U8),
            Const::U16(_) => Type::Int(Int::U16),
            Const::U32(_) => Type::Int(Int::U32),
            Const::U64(_) => Type::Int(Int::U64),
            Const::USize(_) => Type::Int(Int::USize),
            Const::U128(_) => Type::Int(Int::U128),
            Const::PlatformString(_) => Type::PlatformString,
            Const::Bool(_) => Type::Bool,
            Const::F32(_) => Type::Float(Float::F32),
            Const::F64(_) => Type::Float(Float::F64),
            Const::Null(tpe) => Type::ClassRef(*tpe),
        }
    }
    /// Checks if the value is zero.
    pub fn is_zero(&self) -> bool {
        match self {
            Const::I8(val) => *val == 0,
            Const::I16(val) => *val == 0,
            Const::I32(val) => *val == 0,
            Const::I64(val) => *val == 0,
            Const::I128(val) => *val == 0,
            Const::ISize(val) => *val == 0,
            Const::U8(val) => *val == 0,
            Const::U16(val) => *val == 0,
            Const::U32(val) => *val == 0,
            Const::U64(val) => *val == 0,
            Const::U128(val) => *val == 0,
            Const::USize(val) => *val == 0,
            Const::PlatformString(_) => false,
            Const::Bool(_) => false,
            Const::F32(val) => **val == 0.0,
            Const::F64(val) => **val == 0.0,
            Const::Null(_) => true,
        }
    }
    /// Checks if the value is exactly one.
    pub fn is_one(&self) -> bool {
        match self {
            Const::I8(val) => *val == 1,
            Const::I16(val) => *val == 1,
            Const::I32(val) => *val == 1,
            Const::I64(val) => *val == 1,
            Const::I128(val) => *val == 1,
            Const::ISize(val) => *val == 1,
            Const::U8(val) => *val == 1,
            Const::U16(val) => *val == 1,
            Const::U32(val) => *val == 1,
            Const::U64(val) => *val == 1,
            Const::U128(val) => *val == 1,
            Const::USize(val) => *val == 1,
            Const::PlatformString(_) => false,
            Const::Bool(_) => false,
            Const::F32(val) => **val == 1.0,
            Const::F64(val) => **val == 1.0,
            Const::Null(_) => true,
        }
    }
}

impl From<Const> for CILNode {
    fn from(value: Const) -> Self {
        Self::Const(Box::new(value))
    }
}
macro_rules! const_impl {
    ($ty:ty,$name:ident) => {
        impl From<$ty> for Const {
            fn from(value: $ty) -> Self {
                Const::$name(value.try_into().unwrap())
            }
        }
        impl From<$ty> for CILNode {
            fn from(value: $ty) -> Self {
                Const::$name(value.try_into().unwrap()).into()
            }
        }
    };
}
const_impl! {bool, Bool}
const_impl! {u8, U8}
const_impl! {u16, U16}
const_impl! {u32, U32}
const_impl! {u64, U64}
const_impl! {u128, U128}
const_impl! {usize, USize}
const_impl! {i8, I8}
const_impl! {i16, I16}
const_impl! {i32, I32}
const_impl! {i64, I64}
const_impl! {i128, I128}
const_impl! {isize, ISize}
