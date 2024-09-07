use cilly::{IString, Type};

use crate::{AllocID, InterpreterState};

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum Value {
    Undef,
    StringArray(Box<[IString]>),
    String(IString),
    USize(usize),
    ISize(isize),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Ptr(AllocID, u32),
    Bool(bool),
    F32(f32),
    F64(f64),
    ValueType(AllocID),
}

impl Value {
    pub fn default_for_type(tpe: &Type, state: &mut InterpreterState) -> Value {
        match tpe {
            Type::ClassRef(tpe) => {
                let size = state.asm.sizeof_tpedef(tpe.as_ref());
                Value::ValueType(state.alloc(size.get() as usize))
            }
            _ => Value::Undef,
        }
    }
    pub fn set(&mut self, other: Self, state: &mut InterpreterState) {
        let _ = state;
        match self {
            Value::Undef
            | Value::StringArray(_)
            | Value::String(_)
            | Value::USize(_)
            | Value::ISize(_)
            | Value::I8(_)
            | Value::U8(_)
            | Value::I16(_)
            | Value::U16(_)
            | Value::I32(_)
            | Value::U32(_)
            | Value::I64(_)
            | Value::U64(_)
            | Value::I128(_)
            | Value::U128(_)
            | Value::Ptr(_, _)
            | Value::Bool(_)
            | Value::F32(_)
            | Value::F64(_) => *self = other,
            Value::ValueType(_alloc) => todo!("Can't yet assign a value of type ValueType"),
        }
    }
    pub fn pass_as_arg(self, state: &mut InterpreterState) -> Self {
        let _ = state;
        match self {
            Value::Undef
            | Value::StringArray(_)
            | Value::String(_)
            | Value::USize(_)
            | Value::ISize(_)
            | Value::I8(_)
            | Value::U8(_)
            | Value::I16(_)
            | Value::U16(_)
            | Value::I32(_)
            | Value::U32(_)
            | Value::I64(_)
            | Value::U64(_)
            | Value::I128(_)
            | Value::U128(_)
            | Value::Ptr(_, _)
            | Value::Bool(_)
            | Value::F32(_)
            | Value::F64(_) => self.clone(),
            Value::ValueType(_alloc) => todo!("Can't yet pass a valuetype as an arg"),
        }
    }
    pub fn as_usize(&self) -> Option<usize> {
        if let Self::USize(v) = self {
            Some(*v)
        } else if let Self::ISize(v) = self {
            Some(*v as usize)
        } else if let Self::I32(v) = self {
            Some(*v as isize as usize)
        } else if let Self::U32(v) = self {
            Some(*v as usize)
        } else if let Self::Ptr(alloc, offset) = self {
            Some((((*alloc as u64) << (32)) + *offset as u64) as usize)
        } else {
            None
        }
    }
    pub fn as_ptr(&self) -> Option<(AllocID, u32)> {
        if let Self::Ptr(id, offset) = self {
            Some((*id, *offset))
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&IString> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_u64(&self) -> Option<&u64> {
        if let Self::U64(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        if let Self::I8(v) = self {
            Some(*v)
        } else if let Self::U8(v) = self {
            Some(*v as i8)
        } else if let Self::U32(v) = self {
            assert!(*v < 256);
            Some(*v as u8 as i8)
        } else {
            None
        }
    }
}
