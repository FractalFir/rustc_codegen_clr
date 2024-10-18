#[allow(dead_code)]
use super::{Assembly, ClassRef, Exporter, FnSig, Type};
use std::io::Write;
#[derive(Default)]
pub struct CillyIRExpoter {}
impl Exporter for CillyIRExpoter {
    type Error = std::io::Error;

    fn export(&self, asm: &super::Assembly, target: &std::path::Path) -> Result<(), Self::Error> {
        let il_path = target.with_extension("rs");

        if let Err(err) = std::fs::remove_file(&il_path) {
            match err.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => {
                    panic!("Could not remove tmp file because {err:?}")
                }
            }
        };
        let mut il_out = std::io::BufWriter::new(std::fs::File::create(&il_path)?);
        writeln!(il_out, "fn add_cilly(asm:&mut Assembly){{")?;
        for def in asm.class_defs().values() {
            let name = &asm[def.name()];
            let escaped_name = escape_class_name(name);
            writeln!(il_out, "fn {escaped_name}(asm:&mut Assembly){{",)?;
            let extends = if let Some(extends) = def.extends() {
                class_ref(asm.class_ref(extends), asm)
            } else {
                "None".into()
            };
            let fields: String = def
                .fields()
                .iter()
                .map(|(tpe, name, offset)| {
                    let tpe = tpe_to(tpe, asm);
                    let name = &asm[*name];
                    let offset = if let Some(offset) = offset {
                        format!("Some(NonZeroU32::new({offset}).unwrap())")
                    } else {
                        "None".into()
                    };
                    format!("({tpe},{{asm.alloc_string({name:?})}},{offset})")
                })
                .intersperse(",".to_owned())
                .collect();
            let fields = format!("vec![{fields}]");
            let static_fields: String = def
                .static_fields()
                .iter()
                .map(|(tpe, name, thread_local)| {
                    let tpe = tpe_to(tpe, asm);
                    let name = &asm[*name];
                    format!("({tpe},{{asm.alloc_string({name:?})}},{thread_local})")
                })
                .intersperse(",".to_owned())
                .collect();
            let static_fields = format!("vec![{static_fields}]");
            let methods = "vec![]";
            let access = "Access::Public";
            let explict_size = if let Some(explict_size) = def.explict_size() {
                format!("Some(NonZeroU32::new({explict_size}).unwrap())")
            } else {
                "None".into()
            };
            writeln!(
                il_out,
                "let {escaped_name} = ClassDef::new(asm.alloc_string({name:?}),{is_valuetype},{generics},{extends},{fields},{static_fields},{methods},{access},{explict_size});",
                is_valuetype = def.is_valuetype(),
                generics = def.generics(),
            )?;
            // let mut class = crate::v2::class::ClassDef::new();
            writeln!(il_out, "}}")?;
        }
        writeln!(il_out, "}}")?;
        let _ = std::process::Command::new("rustfmt").arg(il_path).output();
        Ok(())
    }
}
fn tpe_to(tpe: &Type, asm: &Assembly) -> String {
    match tpe {
        Type::Ptr(inner) => format!("{{asm.nptr({inner})}}", inner = tpe_to(&asm[*inner], asm)),
        Type::Ref(inner) => format!("{{asm.nref({inner})}}", inner = tpe_to(&asm[*inner], asm)),
        Type::ClassRef(cref) => format!(
            "Type::ClassRef({{asm.alloc_class_ref({cref})}})",
            cref = class_ref(asm.class_ref(*cref), asm)
        ),
        Type::PlatformGeneric(_, _) => todo!(),
        Type::PlatformString
        | Type::PlatformChar
        | Type::PlatformObject
        | Type::Float(_)
        | Type::Int(_)
        | Type::Bool
        | Type::Void => format!("{tpe:?}"),
        Type::PlatformArray { .. } => todo!(),
        Type::FnPtr(sig) => format!("Type::FnPtr({sig})", sig = sig_to(asm[*sig].clone(), asm)),
        Type::SMIDVector(_) => panic!("SMID is not supported when dumping cilly IR"),
    }
}
fn sig_to(sig: FnSig, asm: &Assembly) -> String {
    let inputs: String = sig
        .inputs()
        .iter()
        .map(|input| tpe_to(input, asm))
        .intersperse(",".into())
        .collect();
    format!(
        "{{asm.sig([{inputs}],{output})}}",
        output = tpe_to(sig.output(), asm)
    )
}
fn escape_class_name(name: &str) -> String {
    name.replace('.', "dot")
}
fn class_ref(cref: &ClassRef, asm: &Assembly) -> String {
    let name = &asm[cref.name()];
    let ref_asm = if let Some(ref_asm) = cref.asm() {
        format!("Some(asm.alloc_string({:?}))", &asm[ref_asm])
    } else {
        "None".into()
    };
    let generics: String = cref
        .generics()
        .iter()
        .map(|tpe| tpe_to(tpe, asm))
        .intersperse(",".into())
        .collect::<String>();
    format!("{{let name = asm.alloc_string({name:?}); let ref_asm = {ref_asm};let generics = vec![{generics}].into(); let cref = ClassRef::new(name,ref_asm,{is_valuetype},generics);}}",is_valuetype = cref.is_valuetype())
}
