use ordered_float::OrderedFloat;

use super::{CILNode, StringIdx};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Const {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(u64),
    PlatformString(StringIdx),
    Bool(bool),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
}
impl From<Const> for CILNode {
    fn from(value: Const) -> Self {
        Self::Const(value)
    }
}
