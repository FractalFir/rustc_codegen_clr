/// Cached type handler
pub(crate) mod tycache;
/// A representation of a primitve type or a reference.
pub mod r#type;
/// Contains a reperesentation of a non-primitve .NET type(class,struct)
pub(crate) mod type_def;

pub use r#type::*;
pub use tycache::*;
pub use type_def::*;
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
        Type::F32 => "f32".into(),
        Type::F64 => "f64".into(),
        Type::Ptr(inner) => format!("p{inner}", inner = mangle(inner)).into(),
        Type::DotnetType(tpe) => {
            assert!(
                tpe.generics().is_empty(),
                "Arrays of generic .NET types not supported yet"
            );
            tpe.name_path().replace(".", "_").into()
        }
        Type::DotnetArray(arr)=>format!("a{}{}",arr.dimensions,mangle(&arr.element)).into(),
        Type::DotnetChar=>"c".into(),
        Type::GenericArg(_)=>todo!("Can't mangle generic type arg"),
        Type::FnDef(name)=>format!("fn{}{}",name.len(),name).into(),
        Type::Unresolved=>"un".into(),
        _ => todo!("Can't mangle type {tpe:?}"),
    }
}
