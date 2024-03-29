#![deny(unused_must_use)]
//use assembly::Assembly;
use lazy_static::*;
use rustc_codegen_clr::{
    assembly::Assembly,
    basic_block::BasicBlock,
    cil::CallSite,
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    function_sig::FnSig,
    method::{Method, MethodType},
    r#type::{DotnetTypeRef, Type},
    *,
};
mod cmd;
mod export;
mod load;

use std::{env, io::Write};

enum AOTCompileMode {
    NoAOT,
    MonoAOT,
    FullMonoAOT,
}
impl AOTCompileMode {
    pub fn compile(self, path: &str) {
        match self {
            Self::NoAOT => (),
            Self::MonoAOT => {
                let out = std::process::Command::new("mono")
                    .arg("--aot")
                    .arg("-O=all")
                    .arg(path)
                    .output()
                    .expect("failed run mono AOT process");
                if !out.stderr.is_empty() {
                    panic!("Could not run AOT!");
                }
            }
            Self::FullMonoAOT => {
                let out = std::process::Command::new("mono")
                    .arg("--aot=full")
                    .arg("-O=all")
                    .arg(path)
                    .output()
                    .expect("failed run mono AOT process");
                if !out.stderr.is_empty() {
                    panic!("Could not run AOT!");
                }
            }
        }
    }
}
fn aot_compile_mode(args: &[String]) -> AOTCompileMode {
    if let Some(aot_idx) = args.iter().position(|arg| arg == "--aot_mode") {
        let aot_idx = aot_idx + 1;
        let aot = args
            .get(aot_idx)
            .expect("ERROR: \"--aot_mode\" provided, but no AOT mode set!");
        match aot.as_str() {
            "no" | "none" | "no_aot" | "no-aot" => AOTCompileMode::NoAOT,
            "mono" | "mono_aot" | "mono-aot" => AOTCompileMode::MonoAOT,
            "mono_full" | "mono-full" | "mono_full_aot" | "mono-full-aot" => {
                AOTCompileMode::FullMonoAOT
            }
            _ => panic!("Unknown AOT mode:{aot:?}"),
        }
    } else {
        AOTCompileMode::NoAOT
    }
}
fn patch_missing_method(call_site: &cil::CallSite) -> method::Method {
    let sig = call_site.signature().clone();
    let method = method::Method::new(
        access_modifier::AccessModifer::Private,
        MethodType::Static,
        sig,
        call_site.name(),
        vec![],
        vec![BasicBlock::new(
            vec![CILRoot::throw(&format!(
                "Tried to invoke missing method {name}",
                name = call_site.name()
            ))
            .into()],
            0,
            None,
        )],
    );

    let ops = rustc_codegen_clr::cil::CILOp::throw_msg(&format!(
        "Tried to invoke missing method {name}",
        name = call_site.name()
    ));
    //method.set_ops(ops.into());
    method
}
fn autopatch(asm: &mut Assembly) {
    let asm_sites = asm.call_sites();
    let call_sites = asm_sites
        .iter()
        .filter(|call| call.is_static() && call.class().is_none())
        .filter(|call| !asm.contains_fn(call));
    let mut patched = std::collections::HashMap::new();
    let mut externs = Vec::new();
    for call in call_sites {
        let name = call.name();
        if name == "malloc" {
            patched.insert(
                call.clone(),
                Method::new(
                    access_modifier::AccessModifer::Private,
                    MethodType::Static,
                    call.signature().clone(),
                    "malloc",
                    vec![],
                    vec![BasicBlock::new(
                        vec![CILRoot::Ret {
                            tree: CILNode::Call {
                                args: [CILNode::LDArg(0)].into(),
                                site: CallSite::boxed(
                                    DotnetTypeRef::marshal().into(),
                                    "AllocHGlobal".into(),
                                    FnSig::new(&[Type::ISize], &Type::ISize),
                                    true,
                                ),
                            },
                        }
                        .into()],
                        0,
                        None,
                    )],
                ),
            );
            continue;
        }
        if name == "free" {
            patched.insert(
                call.clone(),
                Method::new(
                    access_modifier::AccessModifer::Private,
                    MethodType::Static,
                    call.signature().clone(),
                    "free",
                    vec![],
                    vec![BasicBlock::new(
                        vec![CILRoot::Ret {
                            tree: CILNode::Call {
                                args: [CILNode::LDArg(0)].into(),
                                site: CallSite::boxed(
                                    DotnetTypeRef::marshal().into(),
                                    "FreeHGlobal".into(),
                                    FnSig::new(&[Type::ISize], &Type::Void),
                                    true,
                                ),
                            },
                        }
                        .into()],
                        0,
                        None,
                    )],
                ),
            );
            continue;
        }
        if name == "realloc" {
            patched.insert(
                call.clone(),
                Method::new(
                    access_modifier::AccessModifer::Private,
                    MethodType::Static,
                    call.signature().clone(),
                    "realloc",
                    vec![],
                    vec![BasicBlock::new(
                        vec![CILRoot::Ret {
                            tree: CILNode::Call {
                                args: [CILNode::LDArg(0), CILNode::LDArg(1)].into(),
                                site: CallSite::boxed(
                                    DotnetTypeRef::marshal().into(),
                                    "ReAllocHGlobal".into(),
                                    FnSig::new(&[Type::ISize, Type::ISize], &Type::ISize),
                                    true,
                                ),
                            },
                        }
                        .into()],
                        0,
                        None,
                    )],
                ),
            );
            continue;
        }
        if rustc_codegen_clr::native_pastrough::LIBC_FNS
            .iter()
            .any(|libc_fn| *libc_fn == name)
        {
            externs.push((
                call.name().into(),
                call.signature().to_owned(),
                get_libc().into(),
            ));
            continue;
        }
        if !patched.contains_key(call) {
            patched.insert(call.clone(), patch_missing_method(call));
        }
    }
    externs
        .into_iter()
        .for_each(|(name, sig, lib)| asm.add_extern_fn(name, sig, lib));
    patched
        .values()
        .for_each(|method| asm.add_method(method.clone()));
}
fn add_mandatory_statics(asm: &mut Assembly) {
    asm.add_static(Type::U8, "__rust_alloc_error_handler_should_panic");
    asm.add_static(Type::U8, "__rust_no_alloc_shim_is_unstable");
    asm.add_static(Type::Ptr(Type::Ptr(Type::U8.into()).into()), "environ");
}
fn get_libc()->&'static str{
    LIBC.as_ref()
}
lazy_static!{
    static ref LIBC:String = get_libc_();
}
#[cfg(target_os = "linux")]
fn get_libc_() -> String {
    let mut libc = None;
    for entry in std::fs::read_dir("/lib").unwrap() {
        let entry = if let Ok(entry) = entry {
            entry
        } else {
            continue;
        };
        if entry.metadata().unwrap().is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("libc.so.") {
                libc = Some(name);
            }
        }
    }
    for entry in std::fs::read_dir("/lib64").unwrap() {
        let entry = if let Ok(entry) = entry {
            entry
        } else {
            continue;
        };
        if entry.metadata().unwrap().is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("libc.so.") {
                libc = Some(name);
            }
        }
    }
    libc.unwrap_or("libc.so.6".into())
    //todo!()
}
#[cfg(target_os = "windows")]
fn get_libc() ->&'static str {
    "ucrtbase.dll"
}
#[cfg(target_os = "macos")]
fn get_libc() -> &'static str{
    "libSystem.B.dylib"
}
fn main() {
    // Parse command line arguments

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    // Input\/output files
    let to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".bc")).collect();
    let ar_to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".rlib")).collect();
    let output = &args[1 + args
        .iter()
        .position(|arg| arg == "-o")
        .expect("No output file!")];
    // Configs
    let aot_compile_mode = aot_compile_mode(args);
    let cargo_support = args.iter().any(|arg| arg.contains("--cargo-support"));

    // Load assemblies from files

    let mut final_assembly = load::load_assemblies(to_link.as_slice(), ar_to_link.as_slice());

    if !*rustc_codegen_clr::config::ABORT_ON_ERROR {
        autopatch(&mut final_assembly);
    }
    let is_lib = output.contains(".dll") || output.contains(".so") || output.contains(".o");
    add_mandatory_statics(&mut final_assembly);
    if !is_lib {
        final_assembly.eliminate_dead_code();
    }
    if *config::C_MODE {
        println!(
            "The codegen is now running in C mode. It will emmit C source files and build them."
        );
        type EXPORTER = rustc_codegen_clr::assembly_exporter::c_exporter::CExporter;
        use rustc_codegen_clr::assembly_exporter::AssemblyExporter;
        EXPORTER::export_assembly(&final_assembly, output.as_ref(), is_lib).unwrap();
        return;
    }
    // Run ILASM
    export::export_assembly(&final_assembly, output, is_lib).expect("Assembly export faliure!");

    // Run AOT compiler
    aot_compile_mode.compile(output);
    let path: std::path::PathBuf = output.into();
    //      Cargo integration

    if cargo_support {
        let bootstrap = format!(
            include_str!("dotnet_jumpstart.rs"),
            exec_file = path.file_name().unwrap().to_string_lossy()
        );
        let bootstrap_path = path.with_extension("rs");
        let mut bootstrap_file = std::fs::File::create(&bootstrap_path).unwrap();
        bootstrap_file.write_all(bootstrap.as_bytes()).unwrap();
        let path = std::env::var("PATH").unwrap();
        let out = std::process::Command::new("rustc")
            .arg("-O")
            .arg(bootstrap_path)
            .arg("-o")
            .arg(output)
            .env_clear()
            .env("PATH", path)
            .output()
            .unwrap();
        if !out.stderr.is_empty() {
            panic!("{}", String::from_utf8(out.stderr).unwrap());
        }
    }

    //todo!()
}
