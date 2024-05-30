#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
pub mod field_desc;
pub mod r#type;
pub use r#type::*;
pub type IString = Box<str>;
pub mod dotnet_type;
pub use dotnet_type::*;
pub mod fn_sig;
pub use fn_sig::*;
pub mod access_modifier;
pub mod asm;
pub mod asm_exporter;
pub mod basic_block;
pub mod c_exporter;
pub mod call_site;
pub mod cil_iter;
pub mod cil_iter_mut;
pub mod cil_node;
pub mod cil_root;
pub mod cil_tree;
pub mod entrypoint;
pub mod ilasm_exporter;
pub mod ilasm_op;
pub mod method;
pub mod static_field_desc;
pub mod type_def;
#[must_use]
/// Returns the name of a fixed-size array
pub fn arr_name(element_count: usize, element: &Type) -> IString {
    let element_name = mangle(element);
    format!("Arr{element_count}_{element_name}",).into()
}
/// Returns a mangled type name.
/// # Panics
/// Panics when a genetic managed array is used.
pub fn mangle(tpe: &Type) -> std::borrow::Cow<'static, str> {
    match tpe {
        Type::Bool => "b".into(),
        Type::Void => "v".into(),
        Type::U8 => "u8".into(),
        Type::U16 => "u16".into(),
        Type::U32 => "u32".into(),
        Type::U64 => "u64".into(),
        Type::U128 => "u128".into(),
        Type::USize => "us".into(),
        Type::I8 => "i8".into(),
        Type::I16 => "i16".into(),
        Type::I32 => "i32".into(),
        Type::I64 => "i64".into(),
        Type::I128 => "i128".into(),
        Type::ISize => "is".into(),
        Type::F16 => "f16".into(),
        Type::F32 => "f32".into(),
        Type::F64 => "f64".into(),
        Type::Ptr(inner) => format!("p{inner}", inner = mangle(inner)).into(),
        Type::DotnetType(tpe) => {
            assert!(
                tpe.generics().is_empty(),
                "Arrays of generic .NET types not supported yet"
            );
            tpe.name_path().replace('.', "_").into()
        }
        Type::ManagedArray { element, dims } => format!("a{}{}", dims, mangle(element)).into(),
        Type::DotnetChar => "c".into(),
        Type::GenericArg(_) => todo!("Can't mangle generic type arg"),
        Type::FnDef(name) => format!("fn{}{}", name.len(), name).into(),
        Type::Unresolved => "un".into(),
        Type::DelegatePtr(sig) => format!(
            "d{output}{input_count}{input_string}",
            output = mangle(sig.output()),
            input_count = sig.inputs().len(),
            input_string = sig.inputs().iter().map(mangle).collect::<String>()
        )
        .into(),
        Type::ManagedReference(inner) => format!("m{inner}", inner = mangle(inner)).into(),
        Type::Foreign => "g".into(),
        Type::CallGenericArg(_) => "l".into(),
        Type::MethodGenericArg(_) => "h".into(),
        //_ => todo!("Can't mangle type {tpe:?}"),
    }
}
#[must_use]
pub fn mem_checks() -> bool {
    *crate::MEM_CHECKS
}
use lazy_static::lazy_static;
lazy_static! {
    #[doc = "Tells codegen to insert memory consistency checks after each call. If INSERT_MIR_DEBUG_COMMENTS is enabled, the consistency checks will be run also after each MIR statement."]pub static ref MEM_CHECKS:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(MEM_CHECKS){
            Some(value)
        }else {
            None
        }).is_some_and(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(MEM_CHECKS),value),
        })
    };
}
#[derive(Clone, Copy)]
pub enum IlasmFlavour {
    Clasic,
    Modern,
}
