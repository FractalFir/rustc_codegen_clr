use super::AssemblyExporter;
use crate::{
    access_modifier::AccessModifer,
    assembly_exporter::AssemblyExportError,
    method::Method,
    r#type::{DotnetTypeRef, Type},
    type_def::TypeDef,
};
use std::{borrow::Cow, io::Write};
#[must_use]
pub(crate) struct ILASMExporter {
    encoded_asm: Vec<u8>,
}
impl std::io::Write for ILASMExporter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.encoded_asm.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.encoded_asm.flush()
    }
}
impl AssemblyExporter for ILASMExporter {
    fn init(asm_name: &str) -> Self {
        let mut encoded_asm = Vec::with_capacity(0x1_00);
        write!(encoded_asm, ".assembly {asm_name}{{}}").expect("Write error!");
        Self { encoded_asm }
    }
    fn add_type(&mut self, tpe: &TypeDef) {
        type_def_cli(&mut self.encoded_asm, tpe).expect("Error");
        //let _ = self.types.push(tpe.clone());
    }
    fn add_method(&mut self, method: &Method) {
        method_cil(&mut self.encoded_asm, method).expect("Error");
    }
    fn finalize(self, final_path: &std::path::Path) -> Result<(), AssemblyExportError> {
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
        let cil = self.encoded_asm;
        println!("cil_path:{cil_path:?}");
        std::fs::File::create(&cil_path)
            .expect("Could not create file")
            .write_all(&cil)
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

fn type_def_cli(w: &mut impl Write, tpe: &TypeDef) -> Result<(), super::AssemblyExportError> {
    let name = tpe.name();
    let mut generics = String::new();
    if tpe.gargc() != 0 {
        generics.push('<');
    }
    for i in 0..tpe.gargc() {
        if i != 0 {
            generics.push(',');
        }
        generics.push_str(&format!("G{i}"));
    }
    if tpe.gargc() != 0 {
        generics.push('>');
    }
    let extended = if let Some(_extended) = tpe.extends() {
        todo!("Can't handle inheretence yet!");
    } else {
        "[System.Runtime]System.ValueType"
    };
    let access = if let AccessModifer::Public = tpe.access_modifier() {
        "public"
    } else {
        "private"
    };
    writeln!(w, "\n.class {access} {name}{generics} extends {extended}{{");
    for inner_type in tpe.inner_types() {
        type_def_cli(w, inner_type);
    }
    let _field_string = String::new();
    if let Some(offsets) = tpe.explicit_offsets() {
        for ((field_name, field_type), offset) in tpe.fields().iter().zip(offsets.iter()) {
            writeln!(
                w,
                "\t.field [{offset}] public {field_type_name} {field_name}",
                field_type_name = prefixed_type_cli(field_type)
            );
        }
    } else {
        for (field_name, field_type) in tpe.fields() {
            writeln!(
                w,
                "\t.field public {field_type_name} {field_name}",
                field_type_name = prefixed_type_cli(field_type)
            );
        }
    }
    writeln!(w, "}}");
    Ok(())
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
fn method_cil(mut w: impl Write, method: &Method) -> std::io::Result<()> {
    let access = if let AccessModifer::Private = method.access() {
        "private"
    } else {
        "public"
    };
    let output = output_type_cli(method.sig().output());
    let name = method.name();
    write!(w, ".method {access} static hidebysig {output} {name}")?;
    args_cli(&mut w, method.sig().inputs())?;
    writeln!(w, "{{")?;
    if method.is_entrypoint() {
        writeln!(w, ".entrypoint")?;
    }
    writeln!(w, "\t.locals (")?;
    let mut locals_iter = method.locals().iter().enumerate();
    if let Some((local_id, local)) = locals_iter.next() {
        write!(
            w,
            "\t\t[{local_id}] {escaped_type}",
            escaped_type = arg_type_cli(local)
        )?;
    }
    for (local_id, local) in locals_iter {
        write!(
            w,
            ",\n\t\t[{local_id}] {escaped_type}",
            escaped_type = arg_type_cli(local)
        )?;
    }
    writeln!(w, "\n\t)")?;
    println!("{name}:\n\n");
    for op in method.get_ops() {
        println!("{op:?}");
        writeln!(w, "\t{op_cli}", op_cli = op_cli(op))?;
    }
    writeln!(w, "}}")
}
fn op_cli(op: &crate::cil_op::CILOp) -> Cow<'static, str> {
    use crate::cil_op::CILOp;
    match op {
        //Control flow
        CILOp::Ret => "ret".into(),
        CILOp::Label(id) => format!("bb_{id}:").into(),
        CILOp::GoTo(id) => format!("br bb_{id}").into(),
        CILOp::BEq(id) => format!("beq bb_{id}").into(),
        CILOp::BGe(id) => format!("bge bb_{id}").into(),
        CILOp::BZero(id) => format!("brzero bb_{id}").into(),
        CILOp::Call(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.signature().inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&arg_type_cli(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&arg_type_cli(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let owner_name = match &call_site.class() {
                    Some(owner) => format!("{}::", dotnet_type_ref_cli(owner)),
                    None => String::new(),
                };
                //println!("inputs:{inputs:?} input_string: {input_string}",inputs = call_site.signature.inputs);
                format!(
                    "call {prefix} {output} {owner_name}{function_name}({input_string})",
                    function_name = call_site.name(),
                    output = output_type_cli(call_site.signature().output())
                )
                .into()
            }
        }
        //Arthmetics
        CILOp::Add => "add".into(),
        CILOp::Sub => "sub".into(),
        CILOp::Mul => "mul".into(),
        CILOp::Div => "div".into(),
        CILOp::Rem => "rem".into(),
        CILOp::Neg => "neg".into(),
        //Bitwise
        CILOp::And => "and".into(),
        CILOp::Or => "or".into(),
        CILOp::XOr => "xor".into(),
        CILOp::Not => "not".into(),
        //Bitshifts
        CILOp::Shl => "shl".into(),
        CILOp::Shr => "shr".into(),
        //Comparisons
        CILOp::Gt => "cgt".into(),
        CILOp::Eq => "ceq".into(),
        CILOp::Lt => "clt".into(),
        //Arguments
        CILOp::LDArg(argnum) => {
            if *argnum < 8 {
                format!("ldarg.{argnum}").into()
            } else if u8::try_from(*argnum).is_ok() {
                format!("ldarg.s {argnum}").into()
            } else {
                format!("ldarg {argnum}").into()
            }
        }
        CILOp::LDArgA(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("ldarga.s {argnum}").into()
            } else {
                format!("ldarga {argnum}").into()
            }
        }
        CILOp::STArg(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("starg.s {argnum}").into()
            } else {
                format!("starg {argnum}").into()
            }
        }
        //Locals
        CILOp::LDLoc(argnum) => {
            if *argnum < 4 {
                format!("ldloc.{argnum}").into()
            } else if u8::try_from(*argnum).is_ok() {
                format!("ldloc.s {argnum}").into()
            } else {
                format!("ldloc {argnum}").into()
            }
        }
        CILOp::LDLocA(argnum) => {
            if u8::try_from(*argnum).is_ok() {
                format!("ldloca.s {argnum}").into()
            } else {
                format!("ldloca {argnum}").into()
            }
        }
        CILOp::STLoc(argnum) => {
            if *argnum < 4 {
                format!("stloc.{argnum}").into()
            } else if u8::try_from(*argnum).is_ok() {
                format!("stloc.s {argnum}").into()
            } else {
                format!("stloc {argnum}").into()
            }
        }
        //Constant
        CILOp::LdcI32(value) => {
            if *value == -1 {
                "ldc.i4.m1".into()
            } else if *value <= 8 && *value >= 0 {
                format!("ldc.i4.{value}").into()
            } else if i8::try_from(*value).is_ok() {
                format!("ldc.i4.s {value}").into()
            } else {
                format!("ldc.i4 {value}").into()
            }
        }
        CILOp::LdcI64(value) => {
            if *value == -1 {
                "ldc.i4.m1".into()
            } else if *value <= 8 && *value >= 0 {
                format!("ldc.i4.{value}").into()
            } else if i8::try_from(*value).is_ok() {
                format!("ldc.i4.s {value}\n\t").into()
            } else if i32::try_from(*value).is_ok() {
                format!("ldc.i4 {value}").into()
            } else {
                format!("ldc.i8 {value}").into()
            }
        }
        CILOp::LdcF32(f32const) => format!("ldc.r4 {f32const}").into(),
        CILOp::LdcF64(f64const) => format!("ldc.r8 {f64const}").into(),
        //Debug
        CILOp::Comment(comment) => format!("//{comment}").into(),
        //Convertions
        CILOp::ConvISize(checked) => {
            if *checked {
                "conv.ovf.i".into()
            } else {
                "conv.i".into()
            }
        }
        CILOp::ConvI8(checked) => {
            if *checked {
                "conv.ovf.i1".into()
            } else {
                "conv.i1".into()
            }
        }
        CILOp::ConvI16(checked) => {
            if *checked {
                "conv.ovf.i2".into()
            } else {
                "conv.i2".into()
            }
        }
        CILOp::ConvI32(checked) => {
            if *checked {
                "conv.ovf.i4".into()
            } else {
                "conv.i4".into()
            }
        }
        CILOp::ConvI64(checked) => {
            if *checked {
                "conv.ovf.i8".into()
            } else {
                "conv.i8".into()
            }
        }
        CILOp::ConvUSize(checked) => {
            if *checked {
                "conv.ovf.u".into()
            } else {
                "conv.u".into()
            }
        }
        CILOp::ConvU8(checked) => {
            if *checked {
                "conv.ovf.u1".into()
            } else {
                "conv.u1".into()
            }
        }
        CILOp::ConvU16(checked) => {
            if *checked {
                "conv.ovf.u2".into()
            } else {
                "conv.u2".into()
            }
        }
        CILOp::ConvU32(checked) => {
            if *checked {
                "conv.ovf.u4".into()
            } else {
                "conv.u4".into()
            }
        }
        CILOp::ConvU64(checked) => {
            if *checked {
                "conv.ovf.u8".into()
            } else {
                "conv.u8".into()
            }
        }
        CILOp::ConvF32(checked) => {
            if *checked {
                "conv.ovf.r4".into()
            } else {
                "conv.r4".into()
            }
        }
        // Pointer stuff
        CILOp::LDIndI8 => "ldind.i1".into(),
        CILOp::STIndI8 => "stind.i1".into(),
        CILOp::LDIndISize => "ldind.i".into(),
        CILOp::STIndISize => "stind.i".into(),
        //OOP
        CILOp::SizeOf(tpe) => format!("sizeof {tpe}", tpe = prefixed_type_cli(tpe)).into(),
        CILOp::Throw => "throw".into(),
        CILOp::LdStr(str) => format!("ldstr {str:?}").into(),
        CILOp::LdObj(obj) => {
            format!("ldobj valuetype {tpe}", tpe = dotnet_type_ref_cli(obj)).into()
        }
        CILOp::STObj(obj) => {
            format!("stobj valuetype {tpe}", tpe = dotnet_type_ref_cli(obj)).into()
        }
        CILOp::LDField(descr) => format!(
            "ldfld {prefixed_type} valuetype {owner}::{field_name}",
            prefixed_type = preifxed_field_type_cli(descr.tpe()),
            owner = dotnet_type_ref_cli(descr.owner()),
            field_name = descr.name()
        )
        .into(),
        CILOp::LDFieldAdress(descr) => format!(
            "ldflda {prefixed_type} valuetype {owner}::{field_name}",
            prefixed_type = preifxed_field_type_cli(descr.tpe()),
            owner = dotnet_type_ref_cli(descr.owner()),
            field_name = descr.name()
        )
        .into(),
        CILOp::STField(descr) => format!(
            "stfld {prefixed_type} valuetype {owner}::{field_name}",
            prefixed_type = preifxed_field_type_cli(descr.tpe()),
            owner = dotnet_type_ref_cli(descr.owner()),
            field_name = descr.name()
        )
        .into(),
        CILOp::NewObj(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.signature().inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&arg_type_cli(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&arg_type_cli(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let owner_name = match &call_site.class() {
                    Some(owner) => format!("{}::", dotnet_type_ref_cli(owner)),
                    None => String::new(),
                };
                //println!("inputs:{inputs:?} input_string: {input_string}",inputs = call_site.signature.inputs);
                format!(
                    "newobj {prefix} {output} {owner_name}{function_name}({input_string})",
                    function_name = call_site.name(),
                    output = output_type_cli(call_site.signature().output())
                )
                .into()
            }
        }
        //Stack
        CILOp::Pop => "pop".into(),
        CILOp::Dup => "dup".into(),
        _ => todo!("Unsuported op {op:?}"),
    }
}
fn output_type_cli(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "void".into(),
        _ => prefixed_type_cli(tpe),
    }
}
fn arg_type_cli(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::DotnetType(_) => format!("valuetype {tpe}", tpe = type_cli(tpe)).into(),
        Type::Ptr(inner) => format!("{inner}*", inner = arg_type_cli(inner)).into(),
        _ => type_cli(tpe),
    }
}
fn dotnet_type_ref_cli(dotnet_type: &DotnetTypeRef) -> String {
    let asm = if let Some(asm_ref) = dotnet_type.asm() {
        format!("[{asm_ref}]")
    } else {
        String::new()
    };
    let name = dotnet_type.name_path();
    let generics = generics_str(dotnet_type.generics());
    format!("{asm}{name}{generics}")
}
fn dotnet_type_ref_cli_generics_unescaped(dotnet_type: &DotnetTypeRef) -> String {
    let asm = if let Some(asm_ref) = dotnet_type.asm() {
        format!("[{asm_ref}]")
    } else {
        String::new()
    };
    let name = dotnet_type.name_path();
    let generics = generics_ident_str(dotnet_type.generics());
    format!("{asm}{name}{generics}")
}
fn type_cli(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "RustVoid".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::I16 => "int16".into(),
        Type::U16 => "uint16".into(),
        Type::F32 => "float32".into(),
        Type::I32 => "int32".into(),
        Type::U32 => "uint32".into(),
        Type::F64 => "float64".into(),
        Type::I64 => "int64".into(),
        Type::U64 => "uint64".into(),
        Type::I128 => "[System.Rutnime]System.Int128".into(),
        Type::U128 => "[System.Rutnime]System.UInt128".into(),
        Type::ISize => "native int".into(),
        Type::USize => "native uint".into(),
        Type::Ptr(inner) => format!("{inner}*", inner = type_cli(inner)).into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli(dotnet_type).into(),
        //Special type
        Type::Unresolved => "Unresolved".into(),
        Type::Bool => "bool".into(),
        Type::DotnetChar => "char".into(),
        Type::GenericArg(idx) => format!("!G{idx}").into(),
        Type::Foreign => "valuetype Foreign".into(),
        //_ => todo!("Unsuported type {tpe:?}"),
    }
}
fn field_type_cli(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Ptr(inner) => format!("{inner}*", inner = type_cli(inner)).into(),
        Type::GenericArg(id) => format!("!{id}").into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli_generics_unescaped(dotnet_type).into(),
        _ => prefixed_type_cli(tpe),
    }
}
fn preifxed_field_type_cli(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Ptr(inner) => format!("{inner}*", inner = type_cli(inner)).into(),
        Type::GenericArg(id) => format!("!{id}").into(),
        Type::DotnetType(dotnet_type) => format!(
            "valuetype {}",
            dotnet_type_ref_cli_generics_unescaped(dotnet_type)
        )
        .into(),
        _ => prefixed_type_cli(tpe),
    }
}
fn prefixed_type_cli(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "RustVoid".into(),
        Type::I8 => "int8".into(),
        Type::U8 => "uint8".into(),
        Type::I16 => "int16".into(),
        Type::U16 => "uint16".into(),
        Type::F32 => "float32".into(),
        Type::I32 => "int32".into(),
        Type::U32 => "uint32".into(),
        Type::F64 => "float64".into(),
        Type::I64 => "int64".into(),
        Type::U64 => "uint64".into(),
        Type::I128 => "valuetype [System.Rutnime]System.Int128".into(),
        Type::U128 => "valuetype [System.Rutnime]System.UInt128".into(),
        Type::ISize => "native int".into(),
        Type::USize => "native uint".into(),
        Type::Ptr(inner) => format!("{inner}*", inner = prefixed_type_cli(inner)).into(),
        Type::DotnetType(dotnet_type) => {
            format!("valuetype {}", dotnet_type_ref_cli(dotnet_type)).into()
        }
        //Special type
        Type::Unresolved => "valuetype Unresolved".into(),
        Type::Foreign => "valuetype Foreign".into(),
        Type::Bool => "bool".into(),
        Type::DotnetChar => "char".into(),
        Type::GenericArg(idx) => format!("!G{idx}").into(),
        //_ => todo!("Unsuported type {tpe:?}"),
    }
}
fn args_cli(w: &mut impl Write, args: &[Type]) -> std::io::Result<()> {
    let mut args = args.iter();
    write!(w, "(")?;
    if let Some(first_arg) = args.next() {
        write!(w, "{type_cli}", type_cli = arg_type_cli(first_arg))?;
    }
    for arg in args {
        write!(w, ",{type_cli}", type_cli = arg_type_cli(arg))?;
    }
    write!(w, ")")?;
    Ok(())
}
fn generics_str(generics: &[Type]) -> Cow<'static, str> {
    if generics.is_empty() {
        "".into()
    } else {
        let mut garg_string = String::new();
        let mut generic_iter = generics.iter();
        if let Some(first_generic) = generic_iter.next() {
            garg_string.push_str(&format!(
                "{type_cli}",
                type_cli = prefixed_type_cli(first_generic)
            ));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(",{type_cli}", type_cli = prefixed_type_cli(arg)));
        }
        format!("<{garg_string}>").into()
    }
}
fn generics_ident_str(generics: &[Type]) -> Cow<'static, str> {
    if generics.is_empty() {
        "".into()
    } else {
        let mut garg_string = String::new();
        let mut generic_iter = generics.iter();
        if let Some(first_generic) = generic_iter.next() {
            garg_string.push_str(&format!(
                "{type_cli}",
                type_cli = field_type_cli(first_generic)
            ));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(",{type_cli}", type_cli = field_type_cli(arg)));
        }
        format!("<{garg_string}>").into()
    }
}
