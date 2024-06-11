use crate::{asm_exporter::escape_class_name, Type};
use std::fmt::{Result, Write};

fn variable(out: &mut impl Write, tpe: &Type, name: &str) -> Result {
    match tpe {
        Type::Void => panic!("ERROR: void variable declared!"),
        Type::Bool => write!(out, "bool {name}"),
        Type::F16 => todo!("f16 not supported yet!"),
        Type::F32 => write!(out, "float {name}"),
        Type::F64 => write!(out, "double {name}"),
        Type::U8 => write!(out, "uint8_t {name}"),
        Type::U16 => write!(out, "uint16_t {name}"),
        Type::U32 => write!(out, "uint32_t {name}"),
        Type::U64 => write!(out, "uint64_t {name}"),
        Type::U128 => write!(out, "unsigned __int128_t {name}"),
        Type::USize => write!(out, "uintptr_t"),
        Type::I8 => write!(out, "int8_t {name}"),
        Type::I16 => write!(out, "int16_t {name}"),
        Type::I32 => write!(out, "int32_t {name}"),
        Type::I64 => write!(out, "int64_t {name}"),
        Type::I128 => write!(out, "__int128_t {name}"),
        Type::ISize => write!(out, "intptr_t"),
        Type::DotnetType(tpe) => {
            assert!(tpe.generics().is_empty());
            write!(
                out,
                "{asm}{name}",
                asm = escape_class_name(tpe.asm().unwrap_or("")),
                name = escape_class_name(tpe.name_path())
            )
        }
        Type::Ptr(_) => todo!(),
        Type::ManagedReference(mref) => todo!(),

        Type::Foreign => todo!(),
        Type::GenericArg(_) => todo!(),
        Type::CallGenericArg(_) => todo!(),
        Type::DotnetChar => todo!(),

        Type::DelegatePtr(_) => todo!(),
        Type::MethodGenericArg(_) => todo!(),
        Type::ManagedArray { element, dims } => todo!(),
    }
}
#[test]
fn c_vars() {
    fn vstr(tpe: &Type, name: &str) -> String {
        let mut string = String::new();
        variable(&mut string, tpe, name).unwrap();
        string
    }
    assert_eq!(vstr(&Type::Bool, "bl"), "bool bl");
    assert_eq!(vstr(&Type::F32, "f"), "float f");
    assert_eq!(vstr(&Type::F64, "d"), "double d");
}
