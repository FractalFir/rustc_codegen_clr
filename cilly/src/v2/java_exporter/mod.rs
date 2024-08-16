use std::io::Write;

use crate::v2::MethodImpl;

use super::{
    cilroot::BranchCond, int, Assembly, CILIter, CILIterElem, CILNode, ClassRefIdx, Exporter,
    NodeIdx, RootIdx, Type,
};
use lazy_static::*;
lazy_static! {
    #[doc = "Specifies the path to the java bytecode assembler."]
    pub static ref JAVA_ASM_PATH:String = {
        std::env::vars().find_map(|(key,value)|if key == "JAVA_ASM_PATH"{Some(value)}else{None}).unwrap_or("krak2".into())
    };
}
pub struct JavaExporter {
    is_lib: bool,
}
impl JavaExporter {
    pub fn new(is_lib: bool) -> Self {
        Self { is_lib }
    }

    fn export_to_write(&self, asm: &super::Assembly, out: &mut impl Write) -> std::io::Result<()> {
        // Iterate trough all types
        for class_def in asm.iter_class_defs() {
            let vis = match class_def.access() {
                crate::v2::Access::Extern | crate::v2::Access::Public => "public",
                crate::v2::Access::Private => "private",
            };
            let sealed = if class_def.is_valuetype() {
                "final"
            } else {
                ""
            };
            let extends = if let Some(parrent) = class_def.extends() {
                simple_class_ref(parrent, asm)
            } else {
                "java/lang/Object".into()
            };

            let name = asm.get_string(class_def.name()).replace(".", "/");
            writeln!(out, ".class {vis} {sealed} {name}\n.super {extends}")?;
            // Export size
            if let Some(size) = class_def.explict_size() {
                writeln!(out, ".method public static sizeof : ()J\n.code stack 1 locals 0\nldc {size}\nlreturn\n.end code\n.end method", size = size.get())?;
            }
            // Export all fields
            for (tpe, name, offset) in class_def.fields() {
                let name = asm.get_string(*name);

                if let Some(offset) = offset {
                    writeln!(
                        out,
                        ".method public static get_{name}_Address : (J)J\n.code stack 3 locals 1\nlload_0\nldc {offset}\nladd\nlreturn\n.end code\n.end method"
                    )
                } else {
                    writeln!(
                        out,
                        ".field public {name} {tpe}",
                        tpe = type_string(*tpe, asm)
                    )
                }?;
            }
            /*// Export all static fields
            for (tpe, name, thread_local) in class_def.static_fields() {
                let name = asm.get_string(*name);
                let tpe = non_void_type_il(tpe, asm);

                writeln!(out, ".field static {tpe} '{name}'")?;
                if *thread_local {
                    writeln!(out,".custom instance void [System.Runtime]System.ThreadStaticAttribute::.ctor() = (01 00 00 00)")?;
                };
            }
            // Export all methods

            for method_id in class_def.methods() {
                let method = asm.method_def(*method_id);
                let vis = match method.access() {
                    crate::v2::Access::Extern | crate::v2::Access::Public => "public",
                    crate::v2::Access::Private => "private",
                };
                let kind = match method.kind() {
                    crate::v2::cilnode::MethodKind::Static => "static",
                    crate::v2::cilnode::MethodKind::Instance => "instance",
                    crate::v2::cilnode::MethodKind::Virtual => "virtual instance",
                    crate::v2::cilnode::MethodKind::Constructor => {
                        "rtspecialname specialname"
                    }
                };
                let pinvoke = if let MethodImpl::Extern {
                    lib,
                    preserve_errno,
                } = method.implementation()
                {
                    let lib = asm.get_string(*lib);
                    if *preserve_errno {
                        format!("pinvokeimpl(\"{lib}\" cdecl lasterr)")
                    } else {
                        format!("pinvokeimpl(\"{lib}\" cdecl)")
                    }
                } else {
                    "".into()
                };
                let name = asm.get_string(method.name());
                let sig = asm.get_sig(method.sig());
                let ret = type_il(sig.output(), asm);
                assert_eq!(method.arg_names().len(), sig.inputs().len(), "{name:?}");
                let inputs = match method.kind() {
                    crate::v2::cilnode::MethodKind::Static => sig.inputs(),
                    crate::v2::cilnode::MethodKind::Instance
                    | crate::v2::cilnode::MethodKind::Virtual
                    | crate::v2::cilnode::MethodKind::Constructor => &sig.inputs()[1..],
                };

                let inputs: String = inputs
                    .iter()
                    .zip(method.arg_names())
                    .map(|(tpe, name)| match name {
                        Some(name) => {
                            format!("{} '{}'", non_void_type_il(tpe, asm), asm.get_string(*name))
                        }
                        None => non_void_type_il(tpe, asm),
                    })
                    .intersperse(",".to_string())
                    .collect();
                let preservesig = if method.implementation().is_extern(){
                    "preservesig"
                }else{""};
                writeln!(
                    out,
                    ".method {vis} hidebysig {kind} {pinvoke} {ret} '{name}'({inputs}) cil managed {preservesig}{{// Method ID {method_id:?}"
                )?;
                let stack_size = match method.implementation() {
                    MethodImpl::MethodBody { blocks, .. } => blocks
                        .iter()
                        .flat_map(|block| block.roots().iter())
                        .map(|root| {
                            crate::v2::CILIter::new(asm.get_root(*root).clone(), asm).count() + 4
                        })
                        .max()
                        .unwrap_or(0),
                    MethodImpl::Extern { .. } => 0,
                    MethodImpl::AliasFor(_) => todo!(),
                    MethodImpl::Missing => 3,
                };
                if stack_size > 6 {
                    writeln!(out, ".maxstack {stack_size}")?;
                }
                if **name == *"entrypoint" {
                    writeln!(out, ".entrypoint")?;
                }
                // Export the implementation
                self.export_method_imp(asm, out, method.resolved_implementation(asm), name)?;
                writeln!(out, "}}")?;
            }*/
            writeln!(out, ".end class")?;
        }

        Ok(())
    }
}

impl Exporter for JavaExporter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        // The IL file should be next to the target
        let il_path = target.with_extension("j");
        let mut il_out = std::io::BufWriter::new(std::fs::File::create(&il_path)?);
        self.export_to_write(asm, &mut il_out)?;
        // Needed to ensure the IL file is valid!
        il_out.flush().unwrap();
        drop(il_out);
        let jar_out = target.with_extension("jar");

        let mut cmd = std::process::Command::new(JAVA_ASM_PATH.clone());
        cmd.arg("asm").arg(il_path)
        .arg("--out")
        .arg(jar_out)
        // .arg("-FOLD") saves up on space, consider enabling.
        ;
        let out = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("Error") || stdout.contains("Error") {
            panic!(
                "stdout:{} stderr:{} cmd:{cmd:?}",
                stdout,
                String::from_utf8_lossy(&out.stderr)
            );
        }

        Ok(())
    }
}
fn simple_class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = asm.get_string(cref.name()).replace(".", "/");
    if let Some(assembly) = cref.asm() {
        let assembly = asm.get_string(assembly).replace(".", "/");
        format!("{assembly}/{name}")
    } else {
        name
    }
}
pub(crate) fn class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = asm.get_string(cref.name());
    let prefix = if cref.is_valuetype() {
        "valuetype"
    } else {
        "class"
    };
    let generic_list = if cref.generics().is_empty() {
        "".into()
    } else {
        format!(
            "<{generics}>",
            generics = cref
                .generics()
                .iter()
                .map(|tpe| type_il(tpe, asm))
                .intersperse(",".to_string())
                .collect::<String>()
        )
    };
    let generic_postfix = if cref.generics().is_empty() {
        "".into()
    } else {
        format!("`{}", cref.generics().len())
    };
    if let Some(assembly) = cref.asm() {
        let assembly = asm.get_string(assembly);
        format!("{prefix} [{assembly}]'{name}{generic_postfix}'{generic_list}")
    } else {
        format!("{prefix} '{name}{generic_postfix}'{generic_list}")
    }
}
fn non_void_type_il(tpe: &Type, asm: &Assembly) -> String {
    match tpe {
        Type::Void => "valuetype RustVoid".into(),
        _ => type_il(tpe, asm),
    }
}
fn type_il(tpe: &Type, asm: &Assembly) -> String {
    match tpe {
        Type::Ptr(inner) => format!("{}*", type_il(asm.get_type(*inner), asm)),
        Type::Ref(inner) => format!("{}&", type_il(asm.get_type(*inner), asm)),
        Type::Int(int) => match int {
            super::Int::U8 => "uint8".into(),
            super::Int::U16 => "uint16".into(),
            super::Int::U32 => "uint32".into(),
            super::Int::U64 => "uint64".into(),
            super::Int::U128 => "valuetype [System.Runtime]System.UInt128".into(),
            super::Int::USize => "native uint".into(),
            super::Int::I8 => "int8".into(),
            super::Int::I16 => "int16".into(),
            super::Int::I32 => "int32".into(),
            super::Int::I64 => "int64".into(),
            super::Int::I128 => "valuetype [System.Runtime]System.Int128".into(),
            super::Int::ISize => "native int".into(),
        },
        Type::ClassRef(cref) => class_ref(*cref, asm),
        Type::Float(float) => match float {
            super::Float::F16 => todo!(),
            super::Float::F32 => "float32".into(),
            super::Float::F64 => "float64".into(),
            super::Float::F128 => todo!(),
        },
        Type::PlatformChar => "char".into(),
        Type::PlatformGeneric(arg, generic) => match generic {
            super::tpe::GenericKind::MethodGeneric => todo!(),
            super::tpe::GenericKind::CallGeneric => format!("!!{arg}"),
            super::tpe::GenericKind::TypeGeneric => format!("!{arg}"),
        },
        Type::Bool => "bool".into(),
        Type::Void => "void".into(),
        Type::PlatformArray { elem, dims } => format!(
            "{elem}[{dims}]",
            elem = type_il(asm.get_type(*elem), asm),
            dims = (1..(dims.get())).map(|_| ',').collect::<String>()
        ),
        Type::FnPtr(sig) => {
            let sig = asm.get_sig(*sig);
            format!(
                "method {output}*({inputs})",
                output = type_il(sig.output(), asm),
                inputs = sig
                    .inputs()
                    .iter()
                    .map(|tpe| non_void_type_il(tpe, asm))
                    .intersperse(",".to_string())
                    .collect::<String>(),
            )
        }
        Type::PlatformString => "string".into(),
        Type::PlatformObject => "object".into(),
    }
}
/*
compile_test::aligned::stable::debug
    compile_test::aligned::stable::release
    compile_test::any::stable::debug
    compile_test::any::stable::release
    compile_test::arg_test::stable::debug
    compile_test::arg_test::stable::release
    compile_test::assert::stable::debug
    compile_test::assert::stable::release
    compile_test::assign::stable::debug
    compile_test::assign::stable::release
    compile_test::binops::stable::debug
    compile_test::binops::stable::release
    compile_test::branches::stable::debug
    compile_test::branches::stable::release
    compile_test::caller_location::stable::debug
    compile_test::caller_location::stable::release
    compile_test::calls::stable::debug
    compile_test::calls::stable::release
    compile_test::casts::stable::debug
    compile_test::casts::stable::release
    compile_test::catch::stable::debug
    compile_test::catch::stable::release
    compile_test::closure::stable::debug
    compile_test::closure::stable::release
    compile_test::cmp::stable::debug
    compile_test::cmp::stable::release
    compile_test::copy_nonoverlaping::stable::debug
    compile_test::copy_nonoverlaping::stable::release
    compile_test::dyns::stable::debug
    compile_test::dyns::stable::release
    compile_test::empty_string_slice::stable::debug
    compile_test::empty_string_slice::stable::release
    compile_test::fn_ptr::stable::debug
    compile_test::fn_ptr::stable::release
    compile_test::fold::stable::debug
    compile_test::fold::stable::release
    compile_test::fuzz100::stable::debug
    compile_test::fuzz16::stable::debug
    compile_test::fuzz16::stable::release
    compile_test::fuzz43::stable::debug
    compile_test::fuzz4::stable::debug
    compile_test::fuzz4::stable::release
    compile_test::fuzz67::stable::debug
    compile_test::fuzz67::stable::release
    compile_test::fuzz80::stable::debug
    compile_test::fuzz88::stable::debug
    compile_test::fuzz88::stable::release
    compile_test::fuzz94::stable::debug
    compile_test::fuzz94::stable::release
    compile_test::fuzz9::stable::debug
    compile_test::fuzz9::stable::release
    compile_test::identity::stable::debug
    compile_test::identity::stable::release
    compile_test::interop::stable::debug
    compile_test::interop::stable::release
    compile_test::mutithreading::stable::debug
    compile_test::mutithreading::stable::release
    compile_test::recursive::stable::debug
    compile_test::recursive::stable::release
    compile_test::references::stable::debug
    compile_test::references::stable::release
    compile_test::slice::stable::debug
    compile_test::slice::stable::release
    compile_test::slice_from_end::stable::debug
    compile_test::slice_from_end::stable::release
    compile_test::slice_index_ref::stable::debug
    compile_test::slice_index_ref::stable::release
    compile_test::tlocal_key_test::stable::debug
    compile_test::tlocal_key_test::stable::release
    compile_test::tuple::stable::debug
    compile_test::tuple::stable::release
    compile_test::type_id::stable::debug
    compile_test::type_id::stable::release
    compile_test::types::stable::debug
    compile_test::types::stable::release
    compile_test::vec::stable::debug
    compile_test::vec::stable::release

*/
fn type_string(tpe: Type, asm: &Assembly) -> String {
    match tpe {
        Type::Ptr(_) => "J".into(),
        Type::Ref(_) => "J".into(),
        Type::Int(_) => "I".into(),
        Type::ClassRef(cref) => {
            if asm.class_ref(cref).is_valuetype() {
                "J".into()
            } else {
                todo!();
            }
        }
        Type::Float(_) => "F".into(),
        Type::PlatformString => todo!(),
        Type::PlatformChar => "C".into(),
        Type::PlatformGeneric(_, _) => todo!(),
        Type::PlatformObject => todo!(),
        Type::Bool => "B".into(),
        Type::Void => "V".into(),
        Type::PlatformArray { elem, dims } => todo!(),
        Type::FnPtr(_) => "J".into(),
    }
}
