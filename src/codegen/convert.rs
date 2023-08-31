use crate::{base_ir::BaseIR, types::Type};

pub(crate) fn convert_as(src: &Type, dest: &Type) -> Vec<BaseIR> {
    if matches!(src, Type::I128 | Type::U128) {
        todo!("Casts involving i128/u128 are not supported yet.")
    }
    match dest {
        Type::I8 => vec![BaseIR::ConvI8],
        Type::U8 => vec![BaseIR::ConvU8],
        Type::I16 => vec![BaseIR::ConvI16],
        Type::U16 => vec![BaseIR::ConvU16],
        Type::F32 => vec![BaseIR::ConvF32],
        Type::I32 => vec![BaseIR::ConvI32],
        Type::U32 => vec![BaseIR::ConvU32],
        Type::F64 => vec![BaseIR::ConvF64],
        Type::I64 => vec![BaseIR::ConvI64],
        Type::U64 => vec![BaseIR::ConvU64],
        Type::ISize => vec![BaseIR::ConvI],
        Type::USize => vec![BaseIR::ConvU],
        Type::Bool => vec![BaseIR::Eq, BaseIR::LDConstI32(0), BaseIR::Eq],
        Type::Ref(_) => vec![BaseIR::ConvI],
        Type::Ptr(_) => vec![BaseIR::ConvI],
        Type::Void => {
            panic!("tried to convert type {src:?} to void type using `as` convertion")
        }
        Type::Array { .. } => panic!(
            "tried to convert type {src:?} to array type(\"{dest:?}\") using `as` convertion"
        ),
        Type::Slice { .. } => panic!(
            "tried to convert type {src:?} to slice type(\"{dest:?}\") using `as` convertion"
        ),
        Type::StrSlice => {
            panic!("tried to convert type {src:?} to string slice type, using `as` convertion")
        }
        Type::Struct{..} => panic!(
            "tried to convert type {src:?} to struct type(\"{dest:?}\") using `as` convertion"
        ),
        /* 
        Type::Enum{..} => {
            panic!("tried to convert type {src:?} to enum type(\"{dest:?}\") using `as` convertion")
        }*/
        Type::Tuple(_) => panic!(
            "tried to convert type {src:?} to tuple type(\"{dest:?}\") using `as` convertion"
        ),
        Type::ResolvedGenric{..} => todo!("Convertions can't yet handle resolved generic types"),
        Type::I128 | Type::U128 => {
            todo!("Casts involving i128/u128 are not supported yet.")
        }
        Type::GenericParam { index } => todo!("Can't handle converting to generic using as"),
    }
}
