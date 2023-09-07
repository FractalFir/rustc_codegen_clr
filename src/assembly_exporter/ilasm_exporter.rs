use crate::{
    assembly_exporter::AssemblyExportError, base_ir::BaseIR, clr_method::CLRMethod, types::Type,
    IString,
};

use super::AssemblyExporter;
#[must_use]
pub(crate) struct ILASMExporter {
    asm_name: IString,
    types: Vec<Type>,
    methods: Vec<CLRMethod>,
}
impl AssemblyExporter for ILASMExporter {
    fn init(asm_name: &str) -> Self {
        Self {
            asm_name: asm_name.into(),
            types: Vec::with_capacity(0x100),
            methods: vec![],
        }
    }
    fn add_type(&mut self, tpe: &Type) {
        match tpe {
            Type::U8 | Type::I8 => (),
            Type::U16 | Type::I16 => (),
            Type::U32 | Type::I32 | Type::F32 => (),
            Type::U64 | Type::I64 | Type::F64 => (),
            _ => {
                let _ = self.types.push(tpe.clone());
            }
        }
    }
    fn add_method(&mut self, method: CLRMethod) {
        self.methods.push(method);
    }
    fn finalize(self, final_path: &std::path::Path) -> Result<(), AssemblyExportError> {
        use std::io::Write;
        //println!("final_path:{final_path:?}");
        let directory = absolute_path(final_path)
            .map_err(|io| AssemblyExportError::CouldNotCanonalizePath(io, final_path.to_owned()))?
            .parent()
            .expect("Can't get the target directory")
            .to_owned();

        let mut out_path = directory.clone();
        out_path.set_file_name(final_path.file_name().expect("Target file has no name!"));
        let out_path = out_path.with_extension(
            final_path
                .extension()
                .expect("target file has no extension!"),
        ); //final_path.expect("Could not canonialize path!");

        let cil_path = out_path.with_extension("il");
        let cil = self.get_cil().expect("Could not get cil");
        println!("cil_path:{cil_path:?}");
        std::fs::File::create(&cil_path)
            .expect("Could not create file")
            .write_all(cil.as_bytes())
            .expect("Could not write bytes");
        let asm_type = "/dll";
        let target = format!(
            "/output:{out_path}",
            out_path = out_path.clone().to_string_lossy()
        );
        let args: [String; 3] = [
            asm_type.into(),
            target,
            cil_path.clone().to_string_lossy().to_string(),
        ];
        let out = std::process::Command::new("ilasm")
            .args(args)
            .output()
            .expect("failed run ilasm process");
        let stdout = String::from_utf8(out.stdout).unwrap();
        if !stdout.contains("\nOperation completed successfully\n") {
            let err = format!(
                "stdout:{} stderr:{}",
                stdout,
                String::from_utf8(out.stderr).unwrap()
            );
            return Err(AssemblyExportError::ExporterError(err.into()));
        }
        Ok(())
    }
}
fn type_cil(tpe: &Type) -> Result<IString, super::AssemblyExportError> {
    match tpe {
        Type::Struct { name, fields } => {
            let mut field_iter = fields.iter();
            let mut field_string = String::new();
            if let Some(first) = field_iter.next() {
                let type_name = escaped_type_name(&first.tpe);
                let field_name = &first.name;
                field_string.push_str(&format!(".field {type_name} {field_name}\n\t"));
            }
            for field in field_iter {
                let type_name = escaped_type_name(&field.tpe);
                let field_name = &field.name;
                field_string.push_str(&format!(".field {type_name} {field_name}\n\t"));
            }
            Ok(format!(".class public sequential ansi sealed beforefieldinit '{name}' extends [System.Runtime]System.ValueType{{{field_string}}}").into())
        }
        Type::Array { element, length } => {
            let name = format!("Arr{length}_{element}", element = type_name(element));
            let mut fields = String::with_capacity((*length as usize) * 10);
            //TODO: use a better approach for creating arrays!
            for index in 0..(*length) {
                fields.push_str(&format!(
                    ".field {element} i{index}\n\t",
                    element = escaped_type_name(element)
                ));
            }
            Ok(format!(".class public sequential ansi sealed beforefieldinit '{name}' extends [System.Runtime]System.ValueType{{{fields}}}").into())
        }
        _ => Ok("".into()),
    }
}
fn absolute_path(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    if path.has_root() {
        Ok(path.to_owned())
    } else {
        let mut abs_path = std::env::current_dir()?;
        abs_path.extend(path);
        Ok(abs_path)
    }
}
impl ILASMExporter {
    fn version(&self) -> (u8, u8, u8, u8) {
        (0, 0, 0, 0)
    }
    fn get_cil(&self) -> Result<IString, super::AssemblyExportError> {
        let types: String = self
            .types
            .iter()
            .map(|tpe| {
                format!(
                    "\t{s}\n",
                    s = type_cil(tpe).expect("Could not create struct CIL")
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
            ".assembly {name}{{\n\t.ver {version}\n}}{types}{methods}",
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
        let mut inputs_iter = method.sig().inputs.iter();
        let mut args = String::new();
        if let Some(firts_arg) = inputs_iter.next() {
            args.push_str(&escaped_type_name(firts_arg));
        }
        for arg in inputs_iter {
            args.push(',');
            args.push_str(&escaped_type_name(arg));
        }
        // Call
        let locals = locals_cli(method.locals());
        let ops = self.ops_cil(method.ops());
        let entrypoint = if method.has_attribute(&crate::clr_method::MethodAttribute::EntryPoint) {
            ".entrypoint\n"
        } else {
            ""
        };
        format!(".method {visibility} hidebysig static {ret} {name}({args}){{\n{entrypoint}{locals}\n\t{ops}\n}}")
            .into()
    }
}
fn locals_cli(locals: &[Type]) -> IString {
    if locals.len() == 0 {
        return "".into();
    }
    let mut local_string = ".locals init(".to_string();
    let mut local_iter = locals.iter().enumerate();
    if let Some((_, first)) = local_iter.next() {
        if *first != Type::Void {
            local_string.push_str(&format!("[0] {tpe}", tpe = escaped_type_name(first)));
        } else {
            local_string.push_str(&"[0] void*//Rust void");
        }
    }
    for (index, tpe) in local_iter {
        local_string.push_str(",\n\t");
        if *tpe != Type::Void {
            local_string.push_str(&format!("[{index}] {tpe}", tpe = escaped_type_name(tpe)));
        } else {
            local_string.push_str(&format!("[{index}] void*//Rust void"));
        }
    }
    local_string.push(')');
    local_string.into()
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
            } else if u8::try_from(*argnum).is_ok() {
                format!("ldarg.s {argnum}")
            } else {
                format!("ldarg {argnum}")
            }
        }
        BaseIR::LDArgA(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("ldarga.s {argnum}")
            } else {
                format!("ldarga {argnum}")
            }
        }
        BaseIR::STArg(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("starg.s {argnum}")
            } else {
                format!("starg {argnum}")
            }
        }
        //Locals
        BaseIR::LDLoc(argnum) => {
            if *argnum < 4 {
                format!("ldloc.{argnum}")
            } else if u8::try_from(*argnum).is_ok() {
                format!("ldloc.s {argnum}")
            } else {
                format!("ldloc {argnum}")
            }
        }
        BaseIR::LDLocA(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("ldloca.s {argnum}")
            } else {
                format!("ldloca {argnum}")
            }
        }
        BaseIR::STLoc(argnum) => {
            if *argnum < 4 {
                format!("stloc.{argnum}")
            } else if u8::try_from(*argnum).is_ok() {
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
            } else if i8::try_from(*value).is_ok() {
                format!("ldc.i4.s {value}\n\t")
            } else if i32::try_from(*value).is_ok() {
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
            } else if i8::try_from(*value).is_ok() {
                format!("ldc.i4.s {value}")
            } else {
                format!("ldc.i4 {value}")
            }
        }
        BaseIR::LDConstString(string) => format!("ldstr \"{string}\""),
        BaseIR::NewObj(call_site) => {
            //CallSite{ assembly, name, signature, is_static } = call_site;
            let mut inputs_iter = call_site.signature.inputs.iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&escaped_type_name(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&escaped_type_name(arg));
            }
            assert!(!call_site.is_static, "object constructor can't be static!");
            let owner_name = match &call_site.owner {
                Some(owner) => format!("{}::", escaped_type_name(&owner)),
                None => "".into(),
            };

            format!(
                "newobj instance {output} {owner_name}{function_name}({input_string})",
                function_name = call_site.name,
                output = escaped_type_name(&call_site.signature.output)
            )
        }
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
        BaseIR::Eq => "ceq".into(),
        BaseIR::Lt => "clt".into(),
        //Seting Pointers
        BaseIR::LDIndI => "ldind.i".into(),
        BaseIR::LDIndIn(n) => format!("ldind.i{n}"),
        BaseIR::LDIndR4 => "ldind.r4".to_string(),
        BaseIR::LDIndR8 => "ldind.r8".to_string(),
        BaseIR::STObj(tpe) => format!("stobj {name}", name = variable_arg_type_name(tpe)),
        // Geting pointers
        BaseIR::STIndI => "stind.i".into(),
        BaseIR::STIndIn(n) => format!("stind.i{n}"),
        BaseIR::LDObj(tpe) => format!("ldobj {name}", name = variable_arg_type_name(tpe)),
        BaseIR::STIndR4 => "stind.r4".to_string(),
        BaseIR::STIndR8 => "stind.r8".to_string(),
        //Fileds
        BaseIR::LDField(field_descriptor) => {
            let field = field_descriptor
                .owner
                .field(field_descriptor.variant, field_descriptor.field_index);
            format!(
                "ldfld {field_type} {field_parent}::{field_name}",
                field_parent = escaped_type_name(&field_descriptor.owner),
                field_type = variable_arg_type_name(&field.tpe),
                field_name = field.name,
            )
        }
        BaseIR::LDFieldAdress(field_descriptor) => {
            let field = field_descriptor
                .owner
                .field(field_descriptor.variant, field_descriptor.field_index);
            format!(
                "ldflda {field_type} {field_parent}::{field_name}",
                field_parent = escaped_type_name(&field_descriptor.owner),
                field_type = variable_arg_type_name(&field.tpe),
                field_name = field.name,
            )
        }
        BaseIR::STField(field_descriptor) => {
            let field = field_descriptor
                .owner
                .field(field_descriptor.variant, field_descriptor.field_index);
            format!(
                "stfld {field_type} {field_parent}::{field_name}",
                field_parent = escaped_type_name(&field_descriptor.owner),
                field_type = variable_arg_type_name(&field.tpe),
                field_name = field.name,
            )
        }
        //Conversions
        BaseIR::ConvI => "conv.i".into(),
        BaseIR::ConvU => "conv.u".into(),
        BaseIR::ConvI8 => "conv.i1".into(),
        BaseIR::ConvU8 => "conv.u1".into(),
        BaseIR::ConvI16 => "conv.i2".into(),
        BaseIR::ConvU16 => "conv.u2".into(),
        BaseIR::ConvI32 => "conv.i4".into(),
        BaseIR::ConvU32 => "conv.u4".into(),
        BaseIR::ConvF32 => "conv.r4".into(),
        BaseIR::ConvI64 => "conv.i8".into(),
        BaseIR::ConvU64 => "conv.u8".into(),
        BaseIR::ConvF64 => "conv.r8".into(),
        //Checked convetions
        BaseIR::ConvI16Checked => "conv.ovf.i2".into(),
        BaseIR::ConvI32Checked => "conv.ovf.i4".into(),
        //Calls
        BaseIR::Call(call_site) => {
            //assert!(sig.inputs.is_empty());
            let mut inputs_iter = call_site.signature.inputs.iter();
            let mut input_string = String::new();
            if let Some(firts_arg) = inputs_iter.next() {
                input_string.push_str(&escaped_type_name(firts_arg));
            }
            for arg in inputs_iter {
                input_string.push(',');
                input_string.push_str(&escaped_type_name(arg));
            }
            let prefix = if call_site.is_static { "" } else { "instance" };
            let owner_name = match &call_site.owner {
                Some(owner) => format!("{}::", escaped_type_name(&owner)),
                None => "".into(),
            };
            //println!("inputs:{inputs:?} input_string: {input_string}",inputs = call_site.signature.inputs);
            format!(
                "\tcall {prefix} {output} {owner_name}{function_name}({input_string})\n",
                function_name = call_site.name,
                output = escaped_type_name(&call_site.signature.output)
            )
        }
        //Type info
        BaseIR::SizeOf(tpe) => format!("sizeof {name}", name = escaped_type_name(tpe)),
        //Debuging
        BaseIR::DebugComment(comment) => format!("//{comment}"),
        BaseIR::Nop => "nop".into(),

        //Other
        BaseIR::InitObj(name) => format!("initobj {name}"),
        BaseIR::Volatile(inner) => format!("volatile. {inner}", inner = op_cil(inner, call_prefix)),
        BaseIR::Pop => "pop".into(),
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
        Type::Ptr(inner) => format!("{inner}*", inner = escaped_type_name(inner)),
        Type::Array { element, length } => {
            format!("Arr{length}_{element}", element = type_name(element))
        }
        Type::ExternType { asm, name } => {
            if asm.is_empty() {
                name.to_string()
            } else {
                format!("[{asm}]{name}")
            }
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
    let type_cil = type_cil(&Type::Struct {
        name: "Empty".into(),
        fields: [].into(),
    })
    .expect("Could not create proper struct CIL");
    assert_eq!(
        type_cil.as_ref(),
        ".class public sequential ansi sealed beforefieldinit 'Empty' extends [System.Runtime]System.ValueType{}"
    );
}

#[test]
fn empty_method_to_cil() {
    let ilasm = ILASMExporter::init("mock_assembly");
    let sig = crate::FunctionSignature::new(&[], &<()>::clr_tpe());
    let empty = CLRMethod::from_raw(&[BaseIR::Return], &[], "empty", sig);
    let method_cil = ilasm.method_cil(&empty);
    assert_eq!(
        method_cil.as_ref(),
        ".method public hidebysig static void empty(){\n\n\tret\n\t\n}"
    );
}
#[test]
fn ilasm_exporter_add_struct() {
    use crate::types::FieldType;
    let mut ilasm = ILASMExporter::init("mock_assembly");
    let vec3 = &Type::Struct {
        name: "Vec3".into(),
        fields: [
            FieldType {
                name: "x".into(),
                tpe: Type::F32,
            },
            FieldType {
                name: "y".into(),
                tpe: Type::F32,
            },
            FieldType {
                name: "z".into(),
                tpe: Type::F32,
            },
        ]
        .into(),
    };
    ilasm.add_type(vec3);
}
