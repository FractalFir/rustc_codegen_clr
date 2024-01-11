#![deny(unused_must_use)]
//use assembly::Assembly;
use rustc_codegen_clr::{assembly::Assembly, r#type::Type, *};
use std::{env, io::Write};

fn load_ar(r: &mut impl std::io::Read) -> std::io::Result<assembly::Assembly> {
    use ar::Archive;
    use std::io::Read;
    let mut final_assembly = assembly::Assembly::empty();
    let mut archive = Archive::new(r);
    // Iterate over all entries in the archive:
    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result.unwrap();
        let name = String::from_utf8_lossy(entry.header().identifier());
        if name.contains(".bc") {
            let mut asm_bytes = Vec::with_capacity(0x100);
            entry
                .read_to_end(&mut asm_bytes)
                .expect("ERROR: Could not load the assembly file!");
            let assembly = postcard::from_bytes(&asm_bytes)
                .expect("ERROR:Could not decode the assembly file!");
            final_assembly = final_assembly.join(assembly);
        }
    }
    Ok(final_assembly)
}
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
        true,
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
    for call in call_sites {
        if !patched.contains_key(call) {
            patched.insert(call.clone(), patch_missing_method(call));
        }
    }
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
    use std::io::Read;
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".bc")).collect();
    let ar_to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".rlib")).collect();
    let output = &args[1 + args
        .iter()
        .position(|arg| arg == "-o")
        .expect("No output file!")];
    let mut final_assembly = assembly::Assembly::empty();
    for asm_path in &to_link {
        let mut asm_file =
            std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
        let mut asm_bytes = Vec::with_capacity(0x100);
        asm_file
            .read_to_end(&mut asm_bytes)
            .expect("ERROR: Could not load the assembly file!");
        let assembly =
            postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!");
        final_assembly = final_assembly.join(assembly);
    }
    for asm_path in &ar_to_link {
        let mut asm_file =
            std::fs::File::open(asm_path).expect("ERROR: Could not open the assembly file!");
        let assembly = load_ar(&mut asm_file).expect("Could not open archive");
        final_assembly = final_assembly.join(assembly);
    }
    //final_assembly.add_array_types();
    //
    if !*rustc_codegen_clr::config::ABORT_ON_ERROR {
        autopatch(&mut final_assembly);
    }

    use rustc_codegen_clr::assembly_exporter::AssemblyExporter;
    let path = output;
    let is_lib = output.contains(".dll") || output.contains(".so") || output.contains(".o");
    add_mandatory_statics(&mut final_assembly);
    // Run ILASM
    rustc_codegen_clr::assembly_exporter::ilasm_exporter::ILASMExporter::export_assembly(
        &final_assembly,
        path.as_ref(),
        is_lib,
    )
    .expect("Assembly export faliure!");
    // Run AOT compiler
    let aot_compile_mode = aot_compile_mode(args);
    aot_compile_mode.compile(path);
    //      Cargo integration
    if args.iter().any(|arg| arg.contains("--cargo-support")) {
        // Moves the final path up one directory.
        let path = std::path::Path::new(output);
        let executable_file_name = path.file_name().unwrap().to_str().unwrap();
        let mut path = path
            .parent()
            .expect("Cargo integration issue. Are you using a custom cargo target directory?")
            .to_owned();
        path.set_file_name(executable_file_name);
        // Sets the file extension to `.exe`
        let mut executable_path = path.clone();
        executable_path.set_extension("exe");
        let executable_file_name = executable_path.file_name().unwrap().to_str().unwrap();

        #[cfg(not(target_os = "windows"))]
        {
            // Creates a laucher shell script
            use std::os::unix::fs::PermissionsExt;
            let launcher_script =
                format!("#!/bin/sh\npushd $(dirname $0)\ndotnet ./{executable_file_name}\npopd\n");
            let mut launcher_path = path.clone();
            let launcher_file_name = launcher_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .rsplit_once('-')
                .unwrap()
                .0
                .to_owned();
            launcher_path.set_file_name(launcher_file_name);
            launcher_path.set_extension("");
            let mut launcher_file = std::fs::File::create(&launcher_path).unwrap();
            let metadata = launcher_file.metadata().unwrap();
            let mut permissions = metadata.permissions();

            permissions.set_mode(0o755);
            launcher_file.set_permissions(permissions).unwrap();
            write!(
                launcher_file,
                "{launcher_script} executable_path:{executable_path:?}"
            )
            .unwrap();
            let mut runtimeconfig = executable_path.clone();
            runtimeconfig.set_extension("runtimeconfig.json");
            let mut runtimeconfig_file = std::fs::File::create(&runtimeconfig).unwrap();
            write!(
                runtimeconfig_file,
                "{}",
                rustc_codegen_clr::compile_test::get_runtime_config()
            )
            .unwrap();
            std::fs::copy(path, executable_path).unwrap();
            //panic!("launcher_path:{launcher_path:?}");
        }
    }
    //todo!()
}
