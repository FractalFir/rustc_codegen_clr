#![deny(unused_must_use)]
//use assembly::Assembly;
use rustc_codegen_clr::{
    assembly::Assembly, config::USE_CECIL_EXPORTER, method::MethodType, r#type::Type, *,
};
mod cmd;
mod load;
mod export;
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
    let mut method = method::Method::new(
        access_modifier::AccessModifer::Private,
        MethodType::Static,
        sig,
        call_site.name(),
        vec![],
    );

    let ops = rustc_codegen_clr::cil::CILOp::throw_msg(&format!(
        "Tried to invoke missing method {name}",
        name = call_site.name()
    ));
    method.set_ops(ops.into());
    method
}
fn autopatch(asm: &mut Assembly) {
    let call_sites = asm
        .call_sites()
        .filter(|call| call.is_static() && call.class().is_none())
        .filter(|call| !asm.contains_fn(call));
    let mut patched = std::collections::HashMap::new();
    let mut externs = Vec::new();
    for call in call_sites {
        if call.name() == "printf" {
            externs.push((
                call.name().into(),
                call.signature().to_owned(),
                "/lib64/libc.so.6".into(),
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

fn main() {

    // Parse command line arguments

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    // Input\/output files
    let to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".bc")).collect();
    let ar_to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".rlib").into()).collect();
    let output = &args[1 + args
        .iter()
        .position(|arg| arg == "-o")
        .expect("No output file!")];
    // Configs
    let aot_compile_mode = aot_compile_mode(args);
    let cargo_support =  args.iter().any(|arg| arg.contains("--cargo-support"));

    // Load assemblies from files

    let mut final_assembly = load::load_assemblies(to_link.as_slice(),ar_to_link.as_slice());

    if !*rustc_codegen_clr::config::ABORT_ON_ERROR {
        autopatch(&mut final_assembly);
    }
    let is_lib = output.contains(".dll") || output.contains(".so") || output.contains(".o");
    add_mandatory_statics(&mut final_assembly);
    // Run ILASM
    export::export_assembly(&final_assembly, output, is_lib).expect("Assembly export faliure!");
  

    // Run AOT compiler
    aot_compile_mode.compile(output);
    let path:std::path::PathBuf = output.into();
    //      Cargo integration
    
    if cargo_support {
        let bootstrap = format!(include_str!("dotnet_jumpstart.rs"),exec_file = path.file_name().unwrap().to_string_lossy());
        let bootstrap_path = path.with_extension("rs");
        let mut bootstrap_file = std::fs::File::create(&bootstrap_path).unwrap();
        bootstrap_file.write_all(bootstrap.as_bytes()).unwrap();
        let path = std::env::var("PATH").unwrap();
        let out = std::process::Command::new("rustc").arg("-O").arg(bootstrap_path).arg("-o").arg(output).env_clear().env("PATH", path).output().unwrap();
        if out.stderr.len() != 0{
            panic!("{}",String::from_utf8(out.stderr).unwrap());
        }
       
    }
   
    //todo!()
}
