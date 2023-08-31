use crate::{base_ir::BaseIR, variable::VariableType};

pub(crate) fn convert_as(src: &VariableType, dest: &VariableType) -> Vec<BaseIR> {
    if matches!(src, VariableType::I128 | VariableType::U128) {
        todo!("Casts involving i128/u128 are not supported yet.")
    }
    match dest {
        VariableType::I8 => vec![BaseIR::ConvI8],
        VariableType::U8 => vec![BaseIR::ConvU8],
        VariableType::I16 => vec![BaseIR::ConvI16],
        VariableType::U16 => vec![BaseIR::ConvU16],
        VariableType::F32 => vec![BaseIR::ConvF32],
        VariableType::I32 => vec![BaseIR::ConvI32],
        VariableType::U32 => vec![BaseIR::ConvU32],
        VariableType::F64 => vec![BaseIR::ConvF64],
        VariableType::I64 => vec![BaseIR::ConvI64],
        VariableType::U64 => vec![BaseIR::ConvU64],
        VariableType::ISize => vec![BaseIR::ConvI],
        VariableType::USize => vec![BaseIR::ConvU],
        VariableType::Bool => vec![BaseIR::Eq, BaseIR::LDConstI32(0), BaseIR::Eq],
        VariableType::Ref(_) => vec![BaseIR::ConvI],
        VariableType::RefMut(_) => vec![BaseIR::ConvI],
        VariableType::Pointer(_) => vec![BaseIR::ConvI],
        VariableType::Void => {
            panic!("tried to convert type {src:?} to void type using `as` convertion")
        }
        VariableType::Array { .. } => panic!(
            "tried to convert type {src:?} to array type(\"{dest:?}\") using `as` convertion"
        ),
        VariableType::Slice { .. } => panic!(
            "tried to convert type {src:?} to slice type(\"{dest:?}\") using `as` convertion"
        ),
        VariableType::StrSlice => {
            panic!("tried to convert type {src:?} to string slice type, using `as` convertion")
        }
        VariableType::Struct(_) => panic!(
            "tried to convert type {src:?} to struct type(\"{dest:?}\") using `as` convertion"
        ),
        VariableType::Enum(_) => {
            panic!("tried to convert type {src:?} to enum type(\"{dest:?}\") using `as` convertion")
        }
        VariableType::Tuple(_) => panic!(
            "tried to convert type {src:?} to tuple type(\"{dest:?}\") using `as` convertion"
        ),
        VariableType::Generic(_) => panic!(
            "tried to convert type {src:?} to genric type(\"{dest:?}\") using `as` convertion"
        ),
        VariableType::I128 | VariableType::U128 => {
            todo!("Casts involving i128/u128 are not supported yet.")
        }
    }
}
