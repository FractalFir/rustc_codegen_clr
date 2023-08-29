use std::collections::HashMap;

use crate::{base_ir::BaseIR, clr_method::CLRMethod, variable::VariableType, IString};

use super::{AssemblyExporter, StructType};
#[must_use]
pub(crate) struct ILASMExporter {
    asm_name: IString,
    structs: HashMap<IString, StructType>,
}
impl AssemblyExporter for ILASMExporter {
    fn init(asm_name: &str) -> Self {
        Self {
            asm_name: asm_name.into(),
            structs: HashMap::new(),
        }
    }

    fn add_struct(&mut self, struct_type: StructType) {
        self.structs
            .entry(struct_type.name().into())
            .or_insert(struct_type);
    }

    fn add_enum_type(
        &mut self,
        name: &str,
        enum_variants: &[super::EnumVariant],
        generics: &super::GenericType,
    ) -> Result<(), super::AssemblyExportError> {
        todo!()
    }

    fn add_array_type(
        &mut self,
        element: &crate::variable::VariableType,
        length: usize,
    ) -> Result<(), super::AssemblyExportError> {
        todo!()
    }

    fn add_method(
        &mut self,
        symbol: &str,
        method: &super::Method,
    ) -> Result<(), super::AssemblyExportError> {
        todo!()
    }

    fn finalize(self, final_path: &std::path::Path) -> Result<(), super::AssemblyExportError> {
        use std::io::Write;
        let cil_path = final_path.with_extension("il");
        let cil = self.get_cil()?;
        std::fs::File::create(&cil_path)?.write_all(cil.as_bytes())?;
        let asm_type = "/dll";
        let args:[String;2] = [asm_type.into(), cil_path.canonicalize()?.to_string_lossy().to_string()];
        println!("args:{args:?}");
        std::process::Command::new("ialsm")
        .args(args)
        .output()
        .expect("failed run ilasm process");
        todo!()
    }
}
impl ILASMExporter {
    fn version(&self) -> (u8, u8, u8, u8) {
        (0, 0, 0, 0)
    }
    fn field_cil(&self, field: &(IString, VariableType)) -> String {
        todo!()
    }
    fn struct_cil(&self, strct: &StructType) -> Result<IString, super::AssemblyExportError> {
        const STRUCT_MODIFIERS: &str = "sequential ansii sealed beforefieldinit";
        let visibility = "public";
        let name = strct.name();
        let fields: String = strct
            .fields()
            .iter()
            .map(|field| format!("\t{fld}\n", fld = self.field_cil(field)))
            .collect();
        Ok(format!(".class {visibility} {STRUCT_MODIFIERS} '{name}'{{{fields}}}").into())
    }
    fn get_cil(&self) -> Result<IString, super::AssemblyExportError> {
        let structs: String = self
            .structs
            .iter()
            .map(|(_, strct)| {
                format!(
                    "\t{s}\n",
                    s = self.struct_cil(strct).expect("Could not create struct CIL")
                )
            })
            .collect();
        let version = self.version();
        let version = format!("{}:{}:{}:{}", version.0, version.1, version.2, version.3);
        let final_cil = format!(
            ".assembly {name}{{\n\t.ver {version}\n}}{structs}",
            name = self.asm_name
        );
        Ok(final_cil.into())
    }
    fn inter_call_prefix(&self) -> &str {
        ""
    }
    fn ops_cil(&self, ops: &[BaseIR]) -> IString {
        todo!();
    }
    fn method_cil(&self, method: CLRMethod) -> IString {
        let name = method.name();
        let visibility = "public";
        let ret = var_name(method.sig().output());
        let args = "";
        let locals = "";
        let ops = "";
        format!(".method {visibility} hidebysig static {ret} {name}({args}){{{locals}\n{ops}}}")
            .into()
    }
}
fn var_name(var: &VariableType) -> IString {
    match var {
        VariableType::Void => "void".into(),
        _ => todo!("unhandled var type "),
    }
}
#[cfg(test)]
use crate::AsVartype;
#[test]
fn init_ilasm_exporter() {
    let _ilasm = ILASMExporter::init("mock_assembly");
}
#[test]
fn empty_asm_to_cil() {
    let ilasm = ILASMExporter::init("mock_assembly");
    let cil = ilasm.get_cil().expect("Could not create CIL assembly!");
    assert_eq!(cil.as_ref(), ".assembly mock_assembly{\n\t.ver 0:0:0:0\n}");
}
#[test]
fn empty_struct_to_cil() {
    let ilasm = ILASMExporter::init("mock_assembly");
    let struct_cil = ilasm
        .struct_cil(&StructType::new("Empty", &[]))
        .expect("Could not create proper struct CIL");
    assert_eq!(
        struct_cil.as_ref(),
        ".class public sequential ansii sealed beforefieldinit 'Empty'{}"
    );
}

#[test]
fn empty_method_to_cil() {
    
    let ilasm = ILASMExporter::init("mock_assembly");
    let sig = crate::FunctionSignature::new(&[<()>::vtpe()], &<()>::vtpe());
    let empty = CLRMethod::from_raw(&[BaseIR::Return], &[], "empty", sig);
    let method_cil = ilasm.method_cil(empty);
    assert_eq!(
        method_cil.as_ref(),
        ".method public hidebysig static void empty(){\n\tret\n}"
    );
}
#[test]
fn ilasm_exporter_add_struct() {
    let mut ilasm = ILASMExporter::init("mock_assembly");
    let fields = &[
        ("x".into(), f32::vtpe()),
        ("y".into(), f32::vtpe()),
        ("z".into(), f32::vtpe()),
    ];
    let vec3 = StructType::new("Vector3", fields);
    ilasm.add_struct(vec3);
}
