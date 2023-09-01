use crate::{base_ir::BaseIR, clr_method::CLRMethod, types::Type, IString};

use super::{AssemblyExporter, ClassInfo};
#[must_use]
pub(crate) struct ILASMExporter {
    asm_name: IString,
    structs: Vec<ClassInfo>,
    methods: Vec<CLRMethod>,
}
impl AssemblyExporter for ILASMExporter {
    fn init(asm_name: &str) -> Self {
        Self {
            asm_name: asm_name.into(),
            structs: vec![],
            methods: vec![],
        }
    }

    fn add_class(&mut self, struct_type: ClassInfo) {
        self.structs.push(struct_type)
    }
    fn add_method(&mut self, method: CLRMethod) {
        self.methods.push(method);
    }
    fn finalize(self, final_path: &std::path::Path) -> Result<(), super::AssemblyExportError> {
        use std::io::Write;
        //println!("final_path:{final_path:?}");
        let directory = final_path
            .parent()
            .expect("Can't get the target directory")
            .canonicalize()?;

        let mut out_path = directory.clone();
        out_path.set_file_name(final_path.file_name().expect("Target file has no name!"));
        let out_path = out_path.with_extension(
            final_path
                .extension()
                .expect("target file has no extension!"),
        ); //final_path.expect("Could not canonialize path!");

        let cil_path = out_path.with_extension("il");
        let cil = self.get_cil()?;
        println!("cil_path:{cil_path:?}");
        std::fs::File::create(&cil_path)
            .expect("Could not create file")
            .write_all(cil.as_bytes())?;
        let asm_type = "/dll";
        let target = format!(
            "/output:{out_path}",
            out_path = out_path.clone().to_string_lossy().to_string()
        );
        let args: [String; 3] = [
            asm_type.into(),
            target,
            cil_path.clone().to_string_lossy().to_string(),
        ];
        std::process::Command::new("ilasm")
            .args(args)
            .output()
            .expect("failed run ilasm process");
        Ok(())
    }
}
impl ILASMExporter {
    fn version(&self) -> (u8, u8, u8, u8) {
        (0, 0, 0, 0)
    }
    fn field_cil(&self, field: &(IString, Type)) -> String {
        format!(
            ".field {type_name} {field_name}",
            type_name = escaped_type_name(&field.1),
            field_name = field.0
        )
    }
    fn struct_cil(&self, strct: &ClassInfo) -> Result<IString, super::AssemblyExportError> {
        const STRUCT_MODIFIERS: &str = "sequential ansi sealed beforefieldinit";
        let visibility = "public";
        let name = strct.name();
        let fields: String = strct
            .fields()
            .iter()
            .map(|field| format!("\t{fld}\n", fld = self.field_cil(field)))
            .collect();
        let extends = {
            let extends = strct.extends();
            let assembly_ref = match extends.0.as_ref() {
                Some(assembly_ref) => format!("[{assembly_ref}]"),
                None => "".into(),
            };
            format!("{assembly_ref}'{type_string}'", type_string = extends.1)
        };

        Ok(
            format!(
                ".class {visibility} {STRUCT_MODIFIERS} '{name}' extends {extends}{{{fields}}}"
            )
            .into(),
        )
    }
    fn get_cil(&self) -> Result<IString, super::AssemblyExportError> {
        let structs: String = self
            .structs
            .iter()
            .map(|strct| {
                format!(
                    "\t{s}\n",
                    s = self.struct_cil(strct).expect("Could not create struct CIL")
                )
            })
            .collect();
        let methods: String = self
            .methods
            .iter()
            .map(|meth| self.method_cil(meth))
            .collect();
        let version = self.version();
        let version = format!("{}:{}:{}:{}", version.0, version.1, version.2, version.3);
        let final_cil = format!(
            ".assembly {name}{{\n\t.ver {version}\n}}{structs}{methods}",
            name = self.asm_name
        );
        Ok(final_cil.into())
    }
    fn call_prefix(&self) -> &str {
        ""
    }
    fn ops_cil(&self, ops: &[BaseIR]) -> String {
        ops.iter()
            .map(|op| op_cil(op, self.call_prefix()))
            .flat_map(|op_str| [op_str, "\n\t".into()])
            .collect()
    }
    fn method_cil(&self, method: &CLRMethod) -> IString {
        let name = method.name();
        let visibility = "public";
        let ret = variable_arg_type_name(method.sig().output());
        let args = "";
        let locals = "";
        let ops = self.ops_cil(method.ops());
        format!(".method {visibility} hidebysig static {ret} {name}({args}){{{locals}\n\t{ops}\n}}")
            .into()
    }
}
fn op_cil(op: &BaseIR, call_prefix: &str) -> String {
    match op {
        //Controll flow
        BaseIR::BBLabel { bb_id } => format!("bb_{bb_id}:"),
        BaseIR::BEq { target } => format!("beq bb_{target}"),
        BaseIR::GoTo { target } => format!("br bb_{target}"),
        BaseIR::Return => "ret".into(),
        BaseIR::Throw => "throw".into(),
        //Arguments
        BaseIR::LDArg(argnum) => {
            if *argnum < 8 {
                format!("ldarg.{argnum}")
            } else if *argnum <= u8::MAX as u32 {
                format!("ldarg.s {argnum}")
            } else {
                format!("ldarg {argnum}")
            }
        }
        BaseIR::LDArgA(argnum) => {
            if *argnum <= u8::MAX as u32 {
                format!("ldarga.s {argnum}")
            } else {
                format!("ldarga {argnum}")
            }
        }
        BaseIR::STArg(argnum) => {
            if *argnum <= u8::MAX as u32 {
                format!("starg.s {argnum}")
            } else {
                format!("starg {argnum}")
            }
        }
        //Locals
        BaseIR::LDLoc(argnum) => {
            if *argnum < 4 {
                format!("ldloc.{argnum}")
            } else if *argnum <= u8::MAX as u32 {
                format!("ldloc.s {argnum}")
            } else {
                format!("ldloc {argnum}")
            }
        }
        BaseIR::LDLocA(argnum) => {
            if *argnum <= u8::MAX as u32 {
                format!("ldloc.s {argnum}")
            } else {
                format!("ldloc {argnum}")
            }
        }
        BaseIR::STLoc(argnum) => {
            if *argnum < 4 {
                format!("stloc.{argnum}")
            } else if *argnum <= u8::MAX as u32 {
                format!("stloc.s {argnum}")
            } else {
                format!("stloc {argnum}")
            }
        }
        //Constants
        BaseIR::LDConstI64(value) => {
            if *value == -1 {
                "ldc.i4.m1".into()
            } else if *value <= 8 && *value >= 0 {
                format!("ldc.i4.{value}")
            } else if *value <= i8::MAX as i64 && *value >= i8::MIN as i64 {
                format!("ldc.i4.s {value}")
            } else if *value <= i32::MAX as i64 && *value >= i32::MIN as i64 {
                format!("ldc.i4 {value}")
            } else {
                format!("ldc.i8 {value}")
            }
        }
        BaseIR::LDConstF32(f32const) => format!("ldc.r4 {f32const}"),
        BaseIR::LDConstI32(value) => {
            if *value == -1 {
                "ldc.i4.m1".into()
            } else if *value <= 8 && *value >= 0 {
                format!("ldc.i4.{value}")
            } else if *value <= i8::MAX as i32 && *value >= i8::MIN as i32 {
                format!("ldc.i4.s {value}")
            } else {
                format!("ldc.i4 {value}")
            }
        }
        BaseIR::LDConstString(string) => format!("ldstr \"{string}\""),
        BaseIR::NewObj { ctor_fn } => format!("newobj instance {ctor_fn}"),
        //Arthmetics
        BaseIR::Add => "add".into(),
        BaseIR::Sub => "sub".into(),
        BaseIR::Mul => "mul".into(),
        BaseIR::Div => "div".into(),
        BaseIR::Rem => "rem".into(),
        //Bitwise
        BaseIR::And => "and".into(),
        BaseIR::Or => "or".into(),
        BaseIR::Xor => "xor".into(),
        BaseIR::Not => "not".into(),
        //Bitshifts
        BaseIR::Shl => "shl".into(),
        BaseIR::Shr => "shr".into(),
        //Comparisons
        BaseIR::Gt => "cgt".into(),
        BaseIR::Ge => "cge".into(),
        BaseIR::Eq => "ceq".into(),
        BaseIR::Lt => "clt".into(),
        BaseIR::Le => "cle".into(),
        //Seting Pointers
        BaseIR::LDIndI => "ldind.i".into(),
        BaseIR::LDIndIn(n) => format!("ldind.i{n}"),
        BaseIR::LDIndR4 => format!("ldind.r4"),
        BaseIR::LDIndR8 => format!("ldind.r8"),
        BaseIR::STObj(name) => format!("stobj valuetype {name}"),
        // Geting pointers
        BaseIR::STIndI => "stind.i".into(),
        BaseIR::STIndIn(n) => format!("stind.i{n}"),
        BaseIR::LDObj(name) => format!("ldobj valuetype {name}"),
        BaseIR::STIndR4 => format!("stind.r4"),
        BaseIR::STIndR8 => format!("stind.r8"),
        //Fileds
        BaseIR::LDField {
            field_parent,
            field_name,
            field_type,
        } => {
            format!(
                "ldfld {field_type} '{field_parent}'::{field_name}",
                field_type = variable_arg_type_name(field_type),
            )
        }
        BaseIR::LDFieldAdress {
            field_parent,
            field_name,
            field_type,
        } => {
            format!(
                "ldflda {field_type} '{field_parent}'::{field_name}",
                field_type = variable_arg_type_name(field_type),
            )
        }
        BaseIR::STField {
            field_parent,
            field_name,
            field_type,
        } => {
            format!(
                "stfld {field_type} '{field_parent}'::{field_name}",
                field_type = variable_arg_type_name(field_type),
            )
        }
        //Conversions
        BaseIR::ConvI => "conv.i".into(),
        BaseIR::ConvU => "conv.u".into(),
        BaseIR::ConvI8 => "conv.i8".into(),
        BaseIR::ConvU8 => "conv.u8".into(),
        BaseIR::ConvI16 => "conv.i16".into(),
        BaseIR::ConvU16 => "conv.i16".into(),
        BaseIR::ConvI32 => "conv.i32".into(),
        BaseIR::ConvU32 => "conv.i32".into(),
        BaseIR::ConvF32 => "conv.r4".into(),
        BaseIR::ConvI64 => "conv.i64".into(),
        BaseIR::ConvU64 => "conv.i64".into(),
        BaseIR::ConvF64 => "conv.r8".into(),
        //Checked convetions
        BaseIR::ConvI16Checked => "conv.ovf.i2".into(),
        BaseIR::ConvI32Checked => "conv.ovf.i4".into(),
        //Calls
        BaseIR::Call { sig, function_name } => {
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = sig.inputs.iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&escaped_type_name(&firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&escaped_type_name(&arg));
            }
            format!(
                "\tcall instance {output} {function_name}({input_string})\n",
                output = escaped_type_name(&sig.output)
            )
        }
        BaseIR::CallStatic { sig, function_name } => {
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = sig.inputs.iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&escaped_type_name(&firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&escaped_type_name(&arg));
            }
            format!(
                "\tcall {output} {call_prefix}{function_name}({input_string})\n",
                output = escaped_type_name(&sig.output)
            )
        }
        //Type info
        BaseIR::SizeOf(name) => format!("sizeof {name}"),
        //Debuging
        BaseIR::DebugComment(comment) => format!("//{comment}"),
        BaseIR::Nop => "nop".into(),
        //Other
        BaseIR::InitObj(name) => format!("initobj {name}"),
        //_=>todo!("unsuported op:{op:?}."),
    }
}
fn variable_arg_type_name(var: &Type) -> IString {
    match var {
        Type::Struct { .. } => {
            format!("valuetype {typename}", typename = escaped_type_name(var)).into()
        }
        /*
        Type::Enum(_) => {
            format!("valuetype {typename}", typename = escaped_type_name(var)).into()
        }*/
        _ => escaped_type_name(var),
    }
}
fn escaped_type_name(var: &Type) -> IString {
    match var {
        Type::Struct { .. } => format!("'{name}'", name = type_name(var)).into(),
        //Type::Enum(name) => format!("'{name}'").into(),
        _ => type_name(var),
    }
}
fn type_name(var: &Type) -> IString {
    match var {
        Type::Struct { name, .. } => name.replace("::", "."), //TODO: handle generic arguments
        //Type::Enum(name) => name.replace("::", "."),
        Type::Void => "void".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::I16 => "int16".into(),
        Type::U16 => "uint16".into(),
        Type::I32 => "int32".into(),
        Type::U32 => "uint32".into(),
        Type::F32 => "float32".into(),
        Type::I64 => "int64".into(),
        Type::U64 => "uint64".into(),
        Type::F64 => "float64".into(),
        Type::I128 => "[System.Runtime]System.Int128".into(),
        Type::U128 => "[System.Runtime]System.UInt128".into(),
        Type::ISize => "native int".into(),
        Type::USize => "native uint".into(),
        Type::Bool => "bool".into(),
        Type::Ref(inner) => format!("{inner}*", inner = escaped_type_name(inner)),
        Type::Array { element, length } => {
            format!("Arr{length}_{element}", element = type_name(element))
        }
        _ => todo!("unhandled var type {var:?}"),
    }
    .into()
}
#[cfg(test)]
use crate::types::ToCLRType;
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
        .struct_cil(&ClassInfo::new("Empty", &[]))
        .expect("Could not create proper struct CIL");
    assert_eq!(
        struct_cil.as_ref(),
        ".class public sequential ansi sealed beforefieldinit 'Empty' extends [System.Runtime]'System.ValueType'{}"
    );
}

#[test]
fn empty_method_to_cil() {
    let ilasm = ILASMExporter::init("mock_assembly");
    let sig = crate::FunctionSignature::new(&[<()>::clr_tpe()], &<()>::clr_tpe());
    let empty = CLRMethod::from_raw(&[BaseIR::Return], &[], "empty", sig);
    let method_cil = ilasm.method_cil(&empty);
    assert_eq!(
        method_cil.as_ref(),
        ".method public hidebysig static void empty(){\n\tret\n\t\n}"
    );
}
#[test]
fn ilasm_exporter_add_struct() {
    let mut ilasm = ILASMExporter::init("mock_assembly");
    let fields = &[
        ("x".into(), f32::clr_tpe()),
        ("y".into(), f32::clr_tpe()),
        ("z".into(), f32::clr_tpe()),
    ];
    let vec3 = ClassInfo::new("Vector3", fields);
    ilasm.add_class(vec3);
}
