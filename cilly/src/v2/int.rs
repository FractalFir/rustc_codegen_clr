use serde::{Deserialize, Serialize};

use super::Type;

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
}
