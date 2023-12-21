/// Cached type handler
pub(crate) mod tycache;
/// A representation of a primitve type or a reference.
pub mod r#type;
/// Contains a reperesentation of a non-primitve .NET type(class,struct)
pub(crate) mod type_def;

pub use r#type::*;
pub use tycache::*;
pub use type_def::*;
pub fn mangle(tpe:&Type)->std::borrow::Cow<'static,str>{
    match tpe{
        Type::Void => "v".into(),
        Type::U8=>"u8".into(),
        Type::DotnetType(tpe)=>{
            assert!(tpe.generics().is_empty(),"Arrays of generic .NET types not supported yet");
            tpe.name_path().replace(".", "_").into()
        }
        _=>todo!("Can't mangle type {tpe:?}"),
    }
}