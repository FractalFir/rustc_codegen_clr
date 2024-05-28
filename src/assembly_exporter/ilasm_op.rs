use std::borrow::Cow;

use cilly::{asm_exporter::escape_class_name, method::Method, DotnetTypeRef, IlasmFlavour, Type};

pub fn non_void_type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "valuetype RustVoid".into(),
        _ => type_cil(tpe),
    }
}
pub fn type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::DelegatePtr(sig) => {
            let mut inputs_iter = sig.inputs().iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&non_void_type_cil(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&non_void_type_cil(arg));
            }
            format!(
                "method {output}*({input_string})",
                output = type_cil(sig.output())
            )
            .into()
        }
        Type::FnDef(name) => format!("valuetype 'fn_{name}'").into(),
        Type::Void => "void".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::F16 => "valuetype [System.Runtime]System.Numerics.Half".into(),
        Type::I16 => "int16".into(),
        Type::U16 => "uint16".into(),
        Type::F32 => "float32".into(),
        Type::I32 => "int32".into(),
        Type::U32 => "uint32".into(),
        Type::F64 => "float64".into(),
        Type::I64 => "int64".into(),
        Type::U64 => "uint64".into(),
        Type::I128 => "valuetype [System.Runtime]System.Int128".into(),
        Type::U128 => "valuetype [System.Runtime]System.UInt128".into(),
        Type::ISize => "native int".into(),
        Type::USize => "native uint".into(),
        Type::Ptr(inner) => format!("{inner}*", inner = type_cil(inner)).into(),
        Type::ManagedReference(inner) => format!("{inner}&", inner = type_cil(inner)).into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli(dotnet_type).into(),
        //Special type
        Type::Unresolved => "valuetype Unresolved".into(),
        Type::Bool => "bool".into(),
        Type::DotnetChar => "char".into(),
        Type::GenericArg(idx) => format!("!{idx}").into(),
        Type::CallGenericArg(idx) => format!("!!{idx}").into(),
        Type::Foreign => "valuetype Foreign".into(),
        Type::ManagedArray { element, dims } => {
            let dims = Into::<u8>::into(*dims);
            let arr = if dims > 0_u8 {
                (0..(dims - 1)).map(|_| ",").collect::<String>()
            } else {
                String::new()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(element)).into()
        } //_ => todo!("Unsuported type {tpe:?}"),
        Type::MethodGenericArg(idx) => format!("!!{idx}").into(),
    }
}
pub fn dotnet_type_ref_cli(dotnet_type: &DotnetTypeRef) -> String {
    let prefix = dotnet_type.tpe_prefix();
    if Some("System.Runtime") == dotnet_type.asm()
        && "System.String" == dotnet_type.name_path()
        && !dotnet_type.is_valuetype()
    {
        return "string".into();
    }
    let asm = if let Some(asm_ref) = dotnet_type.asm() {
        format!("[{asm_ref}]")
    } else {
        String::new()
    };
    let name = dotnet_type.name_path();
    let name = if *crate::config::ESCAPE_NAMES {
        escape_class_name(name)
    } else {
        name.into()
    };
    let generics = generics_str(dotnet_type.generics());
    format!("{prefix} {asm}'{name}'{generics}")
}
fn generics_str(generics: &[Type]) -> Cow<'static, str> {
    if generics.is_empty() {
        "".into()
    } else {
        let mut garg_string = String::new();
        let mut generic_iter = generics.iter();
        if let Some(first_generic) = generic_iter.next() {
            garg_string.push_str(&format!("{type_cil}", type_cil = type_cil(first_generic)));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(",{type_cil}", type_cil = type_cil(arg)));
        }
        format!("<{garg_string}>").into()
    }
}
