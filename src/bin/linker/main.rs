#![deny(unused_must_use)]
//use assembly::Assembly;
use lazy_static::*;
use load::LinkableFile;
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

use std::{collections::HashMap, env, io::Write};
struct NativePastroughInfo {
    defs: HashMap<IString, AString>,
}
impl NativePastroughInfo {
    pub fn new() -> Self {
        Self {
            defs: HashMap::new(),
        }
    }
    pub fn insert(&mut self, k: IString, v: impl Into<AString>) -> Option<AString> {
        self.defs.insert(k, v.into())
    }

    pub fn get(&self, k: &str) -> Option<&AString> {
        self.defs.get(k)
    }
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
    eprintln!(" missing method {name}.", name = call_site.name());
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
    method
}
fn autopatch(asm: &mut Assembly, native_pastrough: &NativePastroughInfo) {
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
        //#[cfg(not(target_os = "linux"))]
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
        if let Some(lib) = native_pastrough.get(name) {
            externs.push((
                call.name().into(),
                call.signature().to_owned(),
                lib.as_ref().clone(),
            ));
            eprintln!("Adding symbol {name} from shared lib {lib}.");
            continue;
        }
        if !patched.contains_key(call) {
            eprintln!("Unknown symbol {name}.");
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
fn get_libc() -> &'static str {
    LIBC.as_ref()
}
lazy_static! {
    static ref LIBC: String = get_libc_();
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
                libc = Some(entry.path().to_str().unwrap().to_owned());
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
                libc = Some(entry.path().to_str().unwrap().to_owned());
            }
        }
    }
    libc.unwrap()
    //todo!()
}
#[cfg(target_os = "windows")]
fn get_libc() -> &'static str {
    "ucrtbase.dll"
}
#[cfg(target_os = "macos")]
fn get_libc() -> &'static str {
    "libSystem.B.dylib"
}
// Detects all the link directiores provided by the linker,
fn link_directories2(args: &[String]) -> Vec<String> {
    let mut directories = Vec::new();
    let mut after_l = false;

    for string in args.iter() {
        if after_l {
            directories.push(string.into());
            after_l = false;
        } else if *string == "-L" {
            directories.push(string.into());
            after_l = true;
        }
    }
    directories
}
// Gets the name of a file without an extension
fn file_stem(file: &str) -> String {
    std::path::Path::new(file)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
// Gets the name of a file without an extension
fn file_dir(file: &str) -> String {
    std::path::Path::new(file)
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
// Adds the shared library at `file_path` to the native passtrough list. Also generates the info  neccessary for creating PInvoke declarations used to call the functions within them.
// Uses `nm` to get function names from a `.so` file, so it is not cross-platform.
#[cfg(target_os = "linux")]
fn add_shared(file_path: &str, native_pastrough: &mut NativePastroughInfo) {
    let nm = std::process::Command::new("nm")
        .arg("-D")
        .arg(file_path)
        .output()
        .unwrap();
    //let file_path = AString::new(format!("{}.{}",file_stem(file_path),file_ext(file_path)).into());
    //eprintln!("file_path:{file_path}");
    let file_path = AString::new(file_path.into());
    if !nm.stderr.is_empty() {
        eprintln!("nm_error:{}", String::from_utf8_lossy(&nm.stderr));
    }
    for line in String::from_utf8_lossy(&nm.stdout).to_string().lines() {
        let mut line_parts = line.split_whitespace();
        if line_parts.clone().count() != 3 {
            continue;
        }
        let _offset = line_parts.next().unwrap();
        let sym_ty = line_parts.next().unwrap();
        let sym_name = line_parts.next().unwrap();
        if sym_ty == "t" || sym_ty == "T" {
            native_pastrough.insert(sym_name.to_string().into(), file_path.clone());
        }
    }
}
// This function should get all the function names from `file_path`, and insert them into `native_pastrough`, with the lib name in such a form, that the .NET runtime is able to handle it.
// DO NOT USE ABSOLUTE PATHS AS THE LIB NAME. IT WILL WORK, BUT WILL NOT BE PORTABLE.
#[cfg(not(target_os = "linux"))]
fn add_shared(file_path: &str, native_pastrough: &mut NativePastroughInfo) {
    panic!("Native passtrough not supported on this platfrom.")
}
/// Compiles all the linked object files into one shared lib, and then generates the info neccessary for creating PInvoke declarations used to call the functions within them.  
/// Uses `gcc`, so may not work on other platforms.
fn handle_native_passtrough(
    args: &[String],
    linkables: &[LinkableFile],
    output_file_path: &str,
    native_pastrough: &mut NativePastroughInfo,
) {
    let mut link = std::process::Command::new("gcc");
    link.arg("--shared");
    link.arg("-fPIC");
    link.arg("-g");
    let dir = file_dir(output_file_path);
    for linkable in linkables {
        std::fs::File::create(format!("{dir}/{}.o", linkable.name()))
            .unwrap()
            .write_all(linkable.file())
            .unwrap();
        link.arg(format!("{dir}/{}.o", linkable.name()));
    }
    link.args(link_directories2(args));
    std::fs::File::create(format!("{dir}/rustc_defs.c")).unwrap().write_all(b"#include <stdlib.h>\n#include <string.h>\n#include <stdint.h>\n#include <stdio.h>
    #ifdef _MSC_VER
    #include <malloc.h>
    void* __rust_alloc(size_t size, size_t align){return _aligned_malloc(align,size);}
    void __rust_dealloc(void* ptr, size_t size, size_t align){_aligned_free(ptr);return;}
    void* __rust_realloc(void* ptr, size_t old_size, size_t align, size_t size){return _aligned_realloc(ptr,size,align);}
    #else
    void* __rust_alloc(size_t size, size_t align){return aligned_alloc(align,size);}
    void __rust_dealloc(void* ptr, size_t size, size_t align){free(ptr);return;}
    void* __rust_realloc(void* ptr, size_t old_size, size_t align, size_t size){
        void* new_alloc = __rust_alloc(size,align);
        memcpy(new_alloc,ptr,old_size);
        __rust_dealloc(ptr,align,old_size);
        return new_alloc;
    }
    #endif
    
    void* __rust_alloc_zeroed(size_t size, size_t align){char* alc = __rust_alloc(size,align);memset(alc,0,size);return alc;}
    uint8_t __rust_no_alloc_shim_is_unstable = 0;
    uint8_t __rust_alloc_error_handler_should_panic = 1;
    void __rust_alloc_error_handler(size_t size, size_t align){printf(\"Allocation of size %x an align %x has failed. Aborting.\\n\",size,align); abort();}
    ").unwrap();
    link.arg(format!("{dir}/rustc_defs.c"));
    //link.args(args.iter().filter(|arg| !arg.contains(".bc") && !arg.contains("static") && !arg.contains("symbols")&& !arg.contains("-nodefaultlibs")  && !arg.contains("-pie")  && !arg.contains("-o") && !arg.contains(".exe") ));
    link.arg("-o");

    let out_fname = file_stem(output_file_path);

    let rustlibs = format!("{dir}/rust_native_{out_fname}.so");
    link.arg(&rustlibs);
    let link_res = link
        .output()
        .expect("Could not launch `gcc` to link native libs.");
    if !link_res.stderr.is_empty() {
        let estring = String::from_utf8_lossy(&link_res.stderr);
        if estring.contains("fatal error: no input files") {
            // Nothing to link, just return without adding the shared lib to the `native_pastrough` list.
            return;
        }
        eprintln!("native linker error:{estring}",);
    }

    add_shared(
        &format!("{dir}/rust_native_{out_fname}.so"),
        native_pastrough,
    );
}
fn main() {
    // Parse command line arguments

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    // Input\/output files
    let to_link: Vec<_> = args.iter().filter(|arg| arg.contains(".bc")).collect();
    let ar_to_link: Vec<_> = args
        .iter()
        .filter(|arg| arg.contains(".rlib"))
        .cloned()
        .collect();

    //ar_to_link.extend(link_dir_files(args));
    let output_file_path = &args[1 + args
        .iter()
        .position(|arg| arg == "-o")
        .expect("No output file!")];
    // Configs
    let aot_compile_mode = aot_compile_mode(args);
    let cargo_support = args.iter().any(|arg| arg.contains("--cargo-support"));

    // Load assemblies from files

    let (mut final_assembly, linkables) =
        load::load_assemblies(to_link.as_slice(), ar_to_link.as_slice());

    let mut native_pastrough = NativePastroughInfo::new();
    #[cfg(target_os = "linux")]
    {
        add_shared(get_libc(), &mut native_pastrough);
    }
    if *crate::config::NATIVE_PASSTROUGH {
        handle_native_passtrough(args, &linkables, output_file_path, &mut native_pastrough);
    }

    if !*rustc_codegen_clr::config::ABORT_ON_ERROR {
        autopatch(&mut final_assembly, &native_pastrough);
    }
    let is_lib = output_file_path.contains(".dll")
        || output_file_path.contains(".so")
        || output_file_path.contains(".o");
    add_mandatory_statics(&mut final_assembly);
    if !is_lib {
        final_assembly.eliminate_dead_code();
    }
    if *config::C_MODE {
        println!(
            "The codegen is now running in C mode. It will emmit C source files and build them."
        );
        type Exporter = rustc_codegen_clr::assembly_exporter::c_exporter::CExporter;
        use rustc_codegen_clr::assembly_exporter::AssemblyExporter;
        Exporter::export_assembly(&final_assembly, output_file_path.as_ref(), is_lib).unwrap();
        return;
    }

    // Run ILASM
    export::export_assembly(&final_assembly, output_file_path, is_lib)
        .expect("Assembly export faliure!");

    // Run AOT compiler
    aot_compile_mode.compile(output_file_path);
    let path: std::path::PathBuf = output_file_path.into();
    //      Cargo integration

    if cargo_support {
        let bootstrap = format!(
            include_str!("dotnet_jumpstart.rs"),
            exec_file = path.file_name().unwrap().to_string_lossy(),
            has_native_companion = *crate::config::NATIVE_PASSTROUGH,
            native_companion_file = if *crate::config::NATIVE_PASSTROUGH {
                format!(
                    "rust_native_{output_file_path}.so",
                    output_file_path = file_stem(output_file_path)
                )
            } else {
                "".to_string()
            }
        );
        let bootstrap_path = path.with_extension("rs");
        let mut bootstrap_file = std::fs::File::create(&bootstrap_path).unwrap();
        bootstrap_file.write_all(bootstrap.as_bytes()).unwrap();
        let path = std::env::var("PATH").unwrap();
        let out = std::process::Command::new("rustc")
            .arg("-O")
            .arg(bootstrap_path)
            .arg("-o")
            .arg(output_file_path)
            .env_clear()
            .env("PATH", path)
            .output()
            .unwrap();
        if !out.stderr.is_empty() {
            panic!("{}", String::from_utf8(out.stderr).unwrap());
        }
    }
    //todo!();
}
