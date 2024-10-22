use serde::{Deserialize, Serialize};

use super::{
    hashable::{HashableF32, HashableF64},
    CILNode, ClassRefIdx, Float, Int, StringIdx, Type,
};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Const {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(u64),
    PlatformString(StringIdx),
    Bool(bool),
    F32(HashableF32),
    F64(HashableF64),
    Null(ClassRefIdx),
}
impl Const {
    pub(crate) fn get_type(&self) -> Type {
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
