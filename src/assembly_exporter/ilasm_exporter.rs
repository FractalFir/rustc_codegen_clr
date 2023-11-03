use super::AssemblyExporter;
use crate::{
    access_modifier::AccessModifer,
    assembly_exporter::AssemblyExportError,
    method::Method,
    r#type::{DotnetTypeRef, Type},
    type_def::TypeDef,
};
use std::{borrow::Cow, io::Write, ops::Deref};
#[must_use]
pub struct ILASMExporter {
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
    fn finalize(
        self,
        final_path: &std::path::Path,
        is_dll: bool,
    ) -> Result<(), AssemblyExportError> {
        //println!("final_path:{final_path:?}");
        let directory = absolute_path(final_path)
            .map_err(|io| AssemblyExportError::CouldNotCanonalizePath(io, final_path.to_owned()))?
            .parent()
            .expect("Can't get the target directory")
            .to_owned();

        let mut out_path = directory.clone();
        out_path.set_file_name(final_path.file_name().expect("Target file has no name!"));
        if let Some(ext) = final_path.extension() {
            out_path = out_path.with_extension(ext);
        }
        //final_path.expect("Could not canonialize path!");

        let cil_path = out_path.with_extension("il");
        let cil = self.encoded_asm;
        println!("cil_path:{cil_path:?}");
        std::fs::File::create(&cil_path)
            .expect("Could not create file")
            .write_all(&cil)
            .expect("Could not write bytes");
        let asm_type = if is_dll { "-dll" } else { "-exe" };
        let target = format!(
            "-output:{out_path}",
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
        let stdout = String::from_utf8_lossy(&out.stdout);
        if !stdout.contains("\nOperation completed successfully\n") {
            let err = format!(
                "stdout:{} stderr:{}",
                stdout,
                String::from_utf8_lossy(&out.stderr)
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
    writeln!(w, "\n.class {access} {name}{generics} extends {extended}{{")?;
    for inner_type in tpe.inner_types() {
        type_def_cli(w, inner_type)?;
    }
    let _field_string = String::new();
    if let Some(offsets) = tpe.explicit_offsets() {
        for ((field_name, field_type), offset) in tpe.fields().iter().zip(offsets.iter()) {
            writeln!(
                w,
                "\t.field [{offset}] public {field_type_name} {field_name}",
                field_type_name = prefixed_type_cil(field_type)
            )?;
        }
    } else {
        for (field_name, field_type) in tpe.fields() {
            writeln!(
                w,
                "\t.field public {field_type_name} {field_name}",
                field_type_name = prefixed_type_cil(field_type)
            )?;
        }
    }
    for (_, method) in tpe.methods().enumerate() {
        method_cil(w, method)?;
    }
    writeln!(w, "}}")?;
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
fn method_cil(w: &mut impl Write, method: &Method) -> std::io::Result<()> {
    let access = if let AccessModifer::Private = method.access() {
        "private"
    } else {
        "public"
    };
    let static_inst = if method.is_static() {
        "static"
    } else {
        "instance"
    };
    let output = output_type_cil(method.sig().output());
    let name = method.name();
    write!(
        w,
        ".method {access} hidebysig {static_inst} {output} {name}"
    )?;
    args_cli(w, method.explicit_inputs())?;
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
            escaped_type = arg_type_cil(local)
        )?;
    }
    for (local_id, local) in locals_iter {
        write!(
            w,
            ",\n\t\t[{local_id}] {escaped_type}",
            escaped_type = arg_type_cil(local)
        )?;
    }
    writeln!(w, "\n\t)")?;
    println!("{name}:\n\n");
    for op in method.get_ops() {
        //println!("{op:?}");
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
        CILOp::BNe(id) => format!("bne.un bb_{id}").into(),
        CILOp::BGe(id) => format!("bge bb_{id}").into(),
        CILOp::BLt(id) => format!("blt bb_{id}").into(),
        CILOp::BLe(id) => format!("ble bb_{id}").into(),
        CILOp::BZero(id) => format!("brzero bb_{id}").into(),
        CILOp::Call(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&call_arg_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&call_arg_type_cil(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let owner_name = match &call_site.class() {
                    Some(owner) => {
                        format!("{}::", prefixed_type_cil(&owner.deref().clone().into()))
                    }
                    None => String::new(),
                };
                //println!("inputs:{inputs:?} input_string: {input_string}",inputs = call_site.signature.inputs);
                format!(
                    "call {prefix} {output} {owner_name} {function_name}({input_string})",
                    function_name = call_site.name(),
                    output = call_output_type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        CILOp::CallVirt(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&call_arg_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&call_arg_type_cil(arg));
                }
                let prefix = if call_site.is_static() {
                    ""
                } else {
                    "instance"
                };
                let owner_name = match &call_site.class() {
                    Some(owner) => {
                        format!("{}::", prefixed_type_cil(&owner.deref().clone().into()))
                    }
                    None => String::new(),
                };
                //println!("inputs:{inputs:?} input_string: {input_string}",inputs = call_site.signature.inputs);
                format!(
                    "callvirt {prefix} {output} {owner_name} {function_name}({input_string})",
                    function_name = call_site.name(),
                    output = call_output_type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        //Arthmetics
        CILOp::Add => "add".into(),
        CILOp::AddOvf => "add.ovf".into(),
        CILOp::AddOvfUn => "add.ovf.un".into(),
        CILOp::Sub => "sub".into(),
        CILOp::SubOvf => "sub.ovf".into(),
        CILOp::SubOvfUn => "sub.ovf.un".into(),
        CILOp::Mul => "mul".into(),
        CILOp::MulOvf => "mul.ovf".into(),
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
        CILOp::LdNull => "ldnull".into(), 
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
        CILOp::ConvF64(checked) => {
            if *checked {
                "conv.ovf.r8".into()
            } else {
                "conv.r8".into()
            }
        }
        // Pointer stuff
        CILOp::LDIndI8 => "ldind.i1".into(),
        CILOp::LDIndI16 => "ldind.i2".into(),
        CILOp::LDIndI32 => "ldind.i4".into(),
        CILOp::LDIndI64 => "ldind.i8".into(),
        CILOp::LDIndF32 => "ldind.r4".into(),
        CILOp::LDIndF64 => "ldind.r8".into(),
        CILOp::LDIndRef => "ldind.ref".into(),
        CILOp::STIndI8 => "stind.i1".into(),
        CILOp::STIndI16 => "stind.i2".into(),
        CILOp::STIndI32 => "stind.i4".into(),
        CILOp::STIndI64 => "stind.i8".into(),
        CILOp::STIndF32 => "stind.r4".into(),
        CILOp::STIndF64 => "stind.r8".into(),
        CILOp::LDIndISize => "ldind.i".into(),
        CILOp::STIndISize => "stind.i".into(),
        CILOp::LocAlloc => "localloc".into(),
        //OOP
        CILOp::SizeOf(tpe) => format!("sizeof {tpe}", tpe = prefixed_type_cil(tpe)).into(),
        CILOp::Throw => "throw".into(),
        CILOp::Rethrow => "rethrow".into(),
        CILOp::LdStr(str) => format!("ldstr {str:?}").into(),
        CILOp::LdObj(obj) => format!(
            "ldobj {tpe}",
            tpe = prefixed_field_type_cil(&obj.as_ref().clone())
        )
        .into(),
        CILOp::STObj(obj) => format!(
            "stobj {tpe}",
            tpe = prefixed_field_type_cil(&obj.as_ref().clone())
        )
        .into(),
        CILOp::LDField(descr) => format!(
            "ldfld {prefixed_type} {owner}::{field_name}",
            prefixed_type = prefixed_field_type_cil(descr.tpe()),
            owner = prefixed_field_type_cil(&descr.owner().clone().into()),
            field_name = descr.name()
        )
        .into(),
        CILOp::LDFieldAdress(descr) => format!(
            "ldflda {prefixed_type} {owner}::{field_name}",
            prefixed_type = prefixed_field_type_cil(descr.tpe()),
            owner = prefixed_field_type_cil(&descr.owner().clone().into()),
            field_name = descr.name()
        )
        .into(),
        CILOp::STField(descr) => format!(
            "stfld {prefixed_type} {owner}::{field_name}",
            prefixed_type = prefixed_field_type_cil(descr.tpe()),
            owner = prefixed_field_type_cil(&descr.owner().clone().into()),
            field_name = descr.name()
        )
        .into(),
        CILOp::NewObj(call_site) => {
            if call_site.is_nop() {
                "".into()
            } else {
                //assert!(sig.inputs.is_empty());
                let mut inputs_iter = call_site.explicit_inputs().iter();
                let mut input_string = String::new();
                if let Some(firts_arg) = inputs_iter.next() {
                    input_string.push_str(&call_arg_type_cil(firts_arg));
                }
                for arg in inputs_iter {
                    input_string.push(',');
                    input_string.push_str(&call_arg_type_cil(arg));
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
                    output = output_type_cil(call_site.signature().output())
                )
                .into()
            }
        }
        CILOp::Nop => "nop".into(),
        CILOp::NewTMPLocal(_) | CILOp::FreeTMPLocal | CILOp::LoadAddresOfTMPLocal | CILOp::SetTMPLocal | CILOp::LoadTMPLocal | CILOp::LoadUnderTMPLocal(_) =>
         panic!("CRITICAL INTERNAL ERROR: OP '{op:?}' is syntetic(internal only) and should have been substituted before being emmited!"),
         CILOp::LoadLocalAllocPtr { alloc_id } => panic!("CRITICAL INTERNAL ERROR:Allocation {alloc_id} was not resolved to a static."),
        CILOp::Pop => "pop".into(),
        CILOp::Dup => "dup".into(),
        CILOp::LDStaticField(static_field) => {
            todo!("Can't load static field {static_field:?}");
        } //_ => todo!("Unsuported op {op:?}"),
    }
}
fn output_type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "void".into(),
        _ => prefixed_type_cil(tpe),
    }
}
fn call_output_type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Void => "void".into(),
        _ => prefixed_field_type_cil(tpe),
    }
}
fn arg_type_cil(tpe: &Type) -> Cow<'static, str> {
    prefixed_type_cil(tpe)
}
fn call_arg_type_cil(tpe: &Type) -> Cow<'static, str> {
    prefixed_field_type_cil(tpe)
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
    let prefix = dotnet_type.tpe_prefix();
    format!("{prefix} {asm}{name}{generics}")
}
fn type_cil(tpe: &Type) -> Cow<'static, str> {
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
        Type::Ptr(inner) => format!("{inner}*", inner = type_cil(inner)).into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli(dotnet_type).into(),
        //Special type
        Type::Unresolved => "Unresolved".into(),
        Type::Bool => "bool".into(),
        Type::DotnetChar => "char".into(),
        Type::GenericArg(idx) => format!("!G{idx}").into(),
        Type::Foreign => "valuetype Foreign".into(),
        Type::DotnetArray(array) => {
            let arr = if array.dimensions > 0 {
                (0..(array.dimensions - 1)).map(|_| ",").collect::<String>()
            } else {
                "".into()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(&array.element)).into()
        } //_ => todo!("Unsuported type {tpe:?}"),
        Type::FnDef(_site) => "FnDef".into(),
    }
}
fn field_type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Ptr(inner) => format!("{inner}*", inner = type_cil(inner)).into(),
        Type::GenericArg(id) => format!("!{id}").into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli_generics_unescaped(dotnet_type).into(),
        _ => prefixed_type_cil(tpe),
    }
}
fn prefixed_field_type_cil(tpe: &Type) -> Cow<'static, str> {
    match tpe {
        Type::Ptr(inner) => format!("{inner}*", inner = prefixed_field_type_cil(inner)).into(),
        Type::GenericArg(id) => format!("!{id}").into(),
        Type::DotnetType(dotnet_type) => dotnet_type_ref_cli_generics_unescaped(dotnet_type).into(),
        _ => prefixed_type_cil(tpe),
    }
}
fn prefixed_type_cil(tpe: &Type) -> Cow<'static, str> {
    let prefixed_type = match tpe {
        Type::Void => "valuetype RustVoid".into(),
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
        Type::I128 => "valuetype [System.Runtime]System.Int128".into(),
        Type::U128 => "valuetype [System.Runtime]System.UInt128".into(),
        Type::ISize => "native int".into(),
        Type::USize => "native uint".into(),
        Type::Ptr(inner) => format!("{inner}*", inner = prefixed_type_cil(inner)).into(),
        Type::DotnetType(dotnet_type) => {
            let prefix = dotnet_type.tpe_prefix();
            format!("{prefix} {}", dotnet_type_ref_cli(dotnet_type)).into()
        }
        Type::FnDef(_site) => "valuetype FnDef".into(),
        //Special type
        Type::Unresolved => "valuetype Unresolved".into(),
        Type::Foreign => "valuetype Foreign".into(),
        Type::Bool => "bool".into(),
        Type::DotnetChar => "char".into(),
        Type::GenericArg(idx) => format!("!G{idx}").into(),
        Type::DotnetArray(array) => {
            let arr = if array.dimensions > 0 {
                (0..(array.dimensions - 1)).map(|_| ",").collect::<String>()
            } else {
                "".into()
            };
            format!("{tpe}[{arr}]", tpe = type_cil(&array.element)).into()
        } //_ => todo!("Unsuported type {tpe:?}"),
    };
    println!("prefixed_type:{prefixed_type}, type:{tpe:?}");
    prefixed_type
}
fn args_cli(w: &mut impl Write, args: &[Type]) -> std::io::Result<()> {
    let mut args = args.iter();
    write!(w, "(")?;
    if let Some(first_arg) = args.next() {
        write!(w, "{type_cil}", type_cil = arg_type_cil(first_arg))?;
    }
    for arg in args {
        write!(w, ",{type_cil}", type_cil = arg_type_cil(arg))?;
    }
    write!(w, ")")?;
    Ok(())
}
fn call_args_cli(w: &mut impl Write, args: &[Type]) -> std::io::Result<()> {
    let mut args = args.iter();
    write!(w, "(")?;
    if let Some(first_arg) = args.next() {
        write!(w, "{type_cil}", type_cil = call_arg_type_cil(first_arg))?;
    }
    for arg in args {
        write!(w, ",{type_cil}", type_cil = call_arg_type_cil(arg))?;
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
                "{type_cil}",
                type_cil = prefixed_field_type_cil(first_generic)
            ));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(
                ",{type_cil}",
                type_cil = prefixed_field_type_cil(arg)
            ));
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
                "{type_cil}",
                type_cil = field_type_cil(first_generic)
            ));
        }
        for arg in generic_iter {
            garg_string.push_str(&format!(",{type_cil}", type_cil = field_type_cil(arg)));
        }
        format!("<{garg_string}>").into()
    }
}
#[test]
fn generic_prefix() {
    let generic = Type::GenericArg(0);
    assert_eq!("!G0", &prefixed_type_cil(&generic));
    assert_eq!("!0", &prefixed_field_type_cil(&generic));
}
#[test]
fn tuple_type() {
    let generic = crate::r#type::tuple_type(&[Type::I8, Type::U8]).into();
    assert_eq!(
        "valuetype [System.Runtime]System.ValueTuple`2<int8,uint8>",
        &prefixed_type_cil(&generic)
    );
    assert_eq!(
        "valuetype [System.Runtime]System.ValueTuple`2<int8,uint8>",
        &prefixed_field_type_cil(&generic)
    );
}
