/// Cached type handler
pub(crate) mod tycache;
/// A representation of a primitve type or a reference.
pub mod r#type;
/// Contains a reperesentation of a non-primitve .NET type(class,struct)
pub(crate) mod type_def;
pub use cilly::Type;
pub use r#type::*;
pub use tycache::*;
pub use type_def::*;
use rustc_middle::ty::{FloatTy, IntTy,  UintTy};
pub fn mangle(tpe: &Type) -> std::borrow::Cow<'static, str> {
    cilly::mangle(tpe)
}

pub fn from_int(int_tpe: &IntTy) -> Type {
    match int_tpe {
        IntTy::I8 => Type::I8,
        IntTy::I16 => Type::I16,
        IntTy::I32 => Type::I32,
        IntTy::I64 => Type::I64,
        IntTy::I128 => Type::I128,
        IntTy::Isize => Type::ISize,
    }
}


pub fn from_uint(uint_tpe: &UintTy) -> Type {
    match uint_tpe {
        UintTy::U8 => Type::U8,
        UintTy::U16 => Type::U16,
        UintTy::U32 => Type::U32,
        UintTy::U64 => Type::U64,
        UintTy::U128 => Type::U128,
        UintTy::Usize => Type::USize,
    }
}


pub fn from_float(float: &FloatTy) -> Type {
    match float {
        FloatTy::F16 => Type::F16,
        FloatTy::F32 => Type::F32,
        FloatTy::F64 => Type::F64,
        FloatTy::F128 => todo!("Can't hanlde 128 bit floats yet!"),
    }
}
