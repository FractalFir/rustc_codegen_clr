use std::any::type_name;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use super::{CILNode, StringIdx};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
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

impl std::hash::Hash for Const {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        type_name::<Self>().hash(state);
        match self {
            Const::I8(val) => val.hash(state),
            Const::I16(val) => val.hash(state),
            Const::I32(val) => val.hash(state),
            Const::I64(val) => val.hash(state),
            Const::ISize(val) => val.hash(state),
            Const::U8(val) => val.hash(state),
            Const::U16(val) => val.hash(state),
            Const::U32(val) => val.hash(state),
            Const::U64(val) => val.hash(state),
            Const::USize(val) => val.hash(state),
            Const::PlatformString(val) => val.hash(state),
            Const::Bool(val) => val.hash(state),
            Const::F32(val) => val.hash(state),
            Const::F64(val) => val.hash(state),
        }
    }
}
impl From<Const> for CILNode {
    fn from(value: Const) -> Self {
        Self::Const(value)
    }
}
