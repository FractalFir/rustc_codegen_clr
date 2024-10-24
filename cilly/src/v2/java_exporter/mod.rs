// This is a WIP experiment.
#![allow(dead_code, unused_imports)]
use std::io::Write;

use crate::v2::MethodImpl;

use super::{
    cilroot::BranchCond, Assembly, CILIter, CILIterElem, CILNode, ClassRefIdx, Exporter, NodeIdx,
    RootIdx, Type,
};

#[doc = "Specifies the path to the java bytecode assembler."]
pub static JAVA_ASM_PATH: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::vars()
        .find_map(|(key, value)| {
            if key == "JAVA_ASM_PATH" {
                Some(value)
            } else {
                None
            }
        })
        .unwrap_or("krak2".into())
});

pub struct JavaExporter {
    is_lib: bool,
}
impl JavaExporter {
    #[must_use]
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

            let name = asm[class_def.name()].replace('.', "/");
            writeln!(out, ".class {vis} {sealed} {name}\n.super {extends}")?;
            // Export size
            if let Some(size) = class_def.explict_size() {
                writeln!(out, ".method public static sizeof : ()J\n.code stack 1 locals 0\nldc {size}\nlreturn\n.end code\n.end method", size = size.get())?;
            }
            // Export all fields
            for (tpe, name, offset) in class_def.fields() {
                let name = &asm[*name];

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
        assert!(
            !(stderr.contains("Error") || stdout.contains("Error")),
            "stdout:{} stderr:{} cmd:{cmd:?}",
            stdout,
            String::from_utf8_lossy(&out.stderr)
        );

        Ok(())
    }
}
fn simple_class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = asm[cref.name()].replace('.', "/");
    if let Some(assembly) = cref.asm() {
        let assembly = asm[assembly].replace('.', "/");
        format!("{assembly}/{name}")
    } else {
        name
    }
}
pub(crate) fn class_ref(cref: ClassRefIdx, asm: &Assembly) -> String {
    let cref = asm.class_ref(cref);
    let name = &asm[cref.name()];
    let prefix = if cref.is_valuetype() {
        "valuetype"
    } else {
        "class"
    };
    let generic_list = if cref.generics().is_empty() {
        String::new()
    } else {
        format!(
            "<{generics}>",
            generics = cref
                .generics()
                .iter()
                .map(|tpe| type_string(*tpe, asm))
                .intersperse(",".to_string())
                .collect::<String>()
        )
    };
    let generic_postfix = if cref.generics().is_empty() {
        String::new()
    } else {
        format!("`{}", cref.generics().len())
    };
    if let Some(assembly) = cref.asm() {
        let assembly = &asm[assembly];
        format!("{prefix} [{assembly}]'{name}{generic_postfix}'{generic_list}")
    } else {
        format!("{prefix} '{name}{generic_postfix}'{generic_list}")
    }
}

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
        Type::PlatformArray { elem, dims } => format!(
            "{arr}{elem}",
            arr = (0..dims.get()).map(|_| "[").collect::<String>(),
            elem = type_string(asm[elem], asm)
        ),
        Type::FnPtr(_) => "J".into(),
        Type::SMIDVector(_) => panic!("SMID is not supported in Java"),
    }
}
/*
// Type your code here, or load an example.
import java.lang.foreign.*;
import  java.lang.System;
class Memory {
    static MemorySegment globalMem;
    static int bitWidth;
    static{
        bitWidth = 8;
        globalMem = MemorySegment.ofAddress(0).reinterpret(1l<<(bitWidth * 8));
    }
    static int LoadI32(long ptr){
        return globalMem.get(ValueLayout.JAVA_INT,ptr);
    }
    static long LoadI64(long ptr){
        return globalMem.get(ValueLayout.JAVA_LONG,ptr);
    }
}
*/
