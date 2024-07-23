#![deny(unused_must_use)]
#![allow(clippy::module_name_repetitions)]
use aot::aot_compile_mode;
use cilly::{
    access_modifier,
    asm::{Assembly, CILLY_V2},
    basic_block::BasicBlock,
    c_exporter::CExporter,
    call,
    call_site::CallSite,
    cil_node::{CILNode, CallOpArgs},
    cil_root::CILRoot,
    conv_usize,
    ilasm_exporter::ILASM_FLAVOUR,
    ldc_i32,
    method::{Method, MethodType},
    DotnetTypeRef, FnSig, IString, IlasmFlavour, Type,
};
//use assembly::Assembly;
use lazy_static::lazy_static;

mod cmd;
mod export;
use cilly::libc_fns;
mod load;
mod native_passtrough;
mod patch;
use fxhash::{FxBuildHasher, FxHashMap};
use native_passtrough::{add_shared, handle_native_passtrough, NativePastroughInfo};
use std::{env, io::Write, path::Path};
mod aot;
fn patch_missing_method(call_site: &cilly::call_site::CallSite) -> Method {
    eprintln!(" missing method {name}.", name = call_site.name());
    let sig = call_site.signature().clone();
    let method = Method::new(
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
        vec![],
    );
    method
}
/// Replaces `malloc` with a direct call to `AllocHGlobal`
fn override_malloc(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
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
                    tree: call!(
                        CallSite::boxed(
                            DotnetTypeRef::marshal().into(),
                            "AllocHGlobal".into(),
                            FnSig::new(&[Type::ISize], Type::ISize),
                            true,
                        ),
                        [CILNode::LDArg(0).cast_ptr(Type::ISize)]
                    )
                    .cast_ptr(call.signature().output().clone()),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("size".into())],
        ),
    );
}

/// Replaces calls to `pthread_atfork` with nops.
/// TODO: this can cause issues.
fn override_pthread_atfork(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "pthread_atfork",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret { tree: ldc_i32!(0) }.into()],
                0,
                None,
            )],
            vec![
                Some("prepare".into()),
                Some("parent".into()),
                Some("child".into()),
            ],
        ),
    );
}

/// Replaces `free` with a direct call to `FreeHGlobal`
fn override_free(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
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
                    tree: call!(
                        CallSite::boxed(
                            DotnetTypeRef::marshal().into(),
                            "FreeHGlobal".into(),
                            FnSig::new(&[Type::ISize], Type::Void),
                            true,
                        ),
                        [CILNode::LDArg(0)]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("free".into())],
        ),
    );
}
/// Replaces `realloc` with a direct call to `ReAllocHGlobal`
fn override_realloc(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
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
                    tree: call!(
                        CallSite::boxed(
                            DotnetTypeRef::marshal().into(),
                            "ReAllocHGlobal".into(),
                            FnSig::new(&[Type::ISize, Type::ISize], Type::ISize),
                            true,
                        ),
                        [CILNode::LDArg(0), CILNode::LDArg(1)]
                    ),
                }
                .into()],
                0,
                None,
            )],
            vec![Some("ptr".into()), Some("new_size".into())],
        ),
    );
}
/// Replaces `_Unwind_RaiseException` with a throw of a special object.
fn override_raise_exception(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "_Unwind_RaiseException",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Throw(CILNode::NewObj(Box::new(CallOpArgs {
                    args: Box::new([conv_usize!(CILNode::LDArg(0))]),
                    site: Box::new(CallSite::new(
                        Some(
                            DotnetTypeRef::new::<&str, _>(None, "RustException")
                                .with_valuetype(false),
                        ),
                        ".ctor".into(),
                        FnSig::new(
                            &[
                                Type::DotnetType(Box::new(
                                    DotnetTypeRef::new::<&str, _>(None, "RustException")
                                        .with_valuetype(false),
                                )),
                                Type::USize,
                            ],
                            Type::Void,
                        ),
                        false,
                    )),
                })))
                .into()],
                0,
                None,
            )],
            vec![Some("ptr".into())],
        ),
    );
}
/// Replaces `_UnwindBacktrace` with a nop.
fn override_backtrace(patched: &mut FxHashMap<CallSite, Method>, call: &CallSite) {
    patched.insert(
        call.clone(),
        Method::new(
            access_modifier::AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "_Unwind_Backtrace",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::debug("called _Unwind_Backtrace").into(),
                    CILRoot::Ret { tree: ldc_i32!(4) }.into(),
                ],
                0,
                None,
            )],
            vec![Some("trace".into()), Some("trace_arg".into())],
        ),
    );
}
fn autopatch(asm: &mut Assembly, native_pastrough: &NativePastroughInfo) {
    let asm_sites = asm.call_sites();
    let call_sites = asm_sites
        .iter()
        .filter(|call| call.is_static() && call.class().is_none())
        .filter(|call| !asm.contains_fn(call));
    let mut patched = FxHashMap::with_hasher(FxBuildHasher::default());
    let mut externs = Vec::new();
    for call in call_sites {
        let name = call.name();
        if name == "malloc" {
            override_malloc(&mut patched, call);
            continue;
        }
        if name == "free" {
            override_free(&mut patched, call);
            continue;
        }
        if name == "realloc" {
            override_realloc(&mut patched, call);
            continue;
        }
        if name == "pthread_atfork" {
            override_pthread_atfork(&mut patched, call);
            continue;
        }
        if name == "pthread_create" {
            patch::override_pthread_create(&mut patched, call);
            continue;
        }
        if name == "pthread_detach" {
            patch::override_pthread_detach(&mut patched, call);
            continue;
        }
        if name == "pthread_attr_init" {
            patch::override_pthread_attr_init(&mut patched, call);
            continue;
        }
        if name == "pthread_attr_setstacksize" {
            patch::override_pthread_attr_setstacksize(&mut patched, call);
            continue;
        }
        if name == "pthread_attr_destroy" {
            patch::pthread_attr_destroy(&mut patched, call);
            continue;
        }
        if name == "pthread_self" {
            patch::pthread_self(&mut patched, call);
            continue;
        }
        if name == "pthread_setname_np" {
            patch::pthread_setname_np(&mut patched, call);
            continue;
        }
        if name == "pthread_join" {
            patch::override_pthread_join(&mut patched, call);
            continue;
        }

        if name == "_Unwind_RaiseException" {
            override_raise_exception(&mut patched, call);
            continue;
        }
        if name == "_Unwind_Backtrace" {
            override_backtrace(&mut patched, call);
            continue;
        }
        //#[cfg(not(target_os = "linux"))]
        if libc_fns::LIBC_FNS.iter().any(|libc_fn| *libc_fn == name) {
            externs.push((
                call.name().into(),
                call.signature().to_owned(),
                get_libc(),
                libc_fns::LIBC_MODIFIES_ERRNO
                    .iter()
                    .any(|libc_fn| *libc_fn == name),
            ));
            continue;
        }
        if libc_fns::LIBM_FNS.iter().any(|libc_fn| *libc_fn == name) {
            externs.push((
                call.name().into(),
                call.signature().to_owned(),
                get_libm(),
                false,
            ));
            continue;
        }
        if let Some(lib) = native_pastrough.get(name) {
            externs.push((
                call.name().into(),
                call.signature().to_owned(),
                lib.as_ref(),
                false,
            ));
            continue;
        }

        if !patched.contains_key(call) {
            patched.insert((*call).clone(), patch_missing_method(call));
        }
    }
    externs
        .into_iter()
        .for_each(|(name, sig, lib, preserve_errno)| {
            asm.add_extern_fn(name, sig, lib.into(), preserve_errno)
        });
    patched
        .values()
        .for_each(|method| asm.add_method(method.clone()));
}
fn add_mandatory_statics(asm: &mut Assembly) {
    asm.add_static(Type::U8, "__rust_alloc_error_handler_should_panic", false);
    asm.add_static(Type::U8, "__rust_no_alloc_shim_is_unstable", false);
    asm.add_static(
        Type::Ptr(Type::Ptr(Type::U8.into()).into()),
        "environ",
        false,
    );
}

fn get_libc() -> &'static str {
    LIBC.as_ref()
}
lazy_static! {
    static ref LIBC: String = get_libc_();
}
fn get_libm() -> &'static str {
    LIBM.as_ref()
}
lazy_static! {
    static ref LIBM: String = get_libm_();
}
#[cfg(target_os = "linux")]
fn get_libc_() -> String {
    let mut libc = None;
    for entry in std::fs::read_dir("/lib").unwrap() {
        let Ok(entry) = entry else {
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
        let Ok(entry) = entry else {
            continue;
        };
        if entry.metadata().unwrap().is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("libc.so.") {
                libc = Some(entry.path().to_str().unwrap().to_owned());
            }
        }
    }
    libc.unwrap_or("libc.so.6".into())
    //todo!()
}
#[cfg(target_os = "linux")]
fn get_libm_() -> String {
    let mut libc = None;
    for entry in std::fs::read_dir("/lib").unwrap() {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.metadata().unwrap().is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("libm.so.") {
                libc = Some(entry.path().to_str().unwrap().to_owned());
            }
        }
    }
    for entry in std::fs::read_dir("/lib64").unwrap() {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.metadata().unwrap().is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("libm.so.") {
                libc = Some(entry.path().to_str().unwrap().to_owned());
            }
        }
    }
    libc.unwrap_or("libm.so.6".into())
    //todo!()
}
#[cfg(target_os = "windows")]
fn get_libc_() -> String {
    "ucrtbase.dll".to_string()
}
#[cfg(target_os = "windows")]
fn get_libm_() -> String {
    "ucrtbase.dll".to_string()
}
#[cfg(target_os = "macos")]
fn get_libc_() -> String {
    "libSystem.B.dylib".to_string()
}
#[cfg(target_os = "macos")]
fn get_libm_() -> String {
    "libSystem.B.dylib".to_string()
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

fn main() {
    // Parse command line arguments

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    // Input\/output files
    let to_link: Vec<_> = args
        .iter()
        .filter(|arg| arg.contains(".bc") || arg.contains(".cilly"))
        .collect();
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
    final_assembly.resolve_initializers();
    final_assembly.resolve_method_aliases();
    // Aplly certain fixes/workarounds to the final assembly
    override_errno(&mut final_assembly);
    patch::patch_all(&mut final_assembly);
    let mut native_pastrough = NativePastroughInfo::new();
    #[cfg(target_os = "linux")]
    {
        if *NATIVE_PASSTROUGH {
            add_shared(get_libc(), &mut native_pastrough);
        }
    }
    if *NATIVE_PASSTROUGH {
        handle_native_passtrough(args, &linkables, output_file_path, &mut native_pastrough);
    }

    if !*ABORT_ON_ERROR {
        autopatch(&mut final_assembly, &native_pastrough);
    }
    let is_lib = output_file_path.contains(".dll")
        || output_file_path.contains(".so")
        || output_file_path.contains(".o");
    add_mandatory_statics(&mut final_assembly);

    if !is_lib && !*KEEP_DEAD_CODE {
        final_assembly.eliminate_dead_code();
    }
    if *C_MODE {
        type Exporter = cilly::c_exporter::CExporter;
        use cilly::asm_exporter::AssemblyExporter;
        println!(
            "The codegen is now running in C mode. It will emmit C source files and build them."
        );

        Exporter::export_assembly(
            CExporter::init("rust_module"),
            &final_assembly,
            output_file_path.as_ref(),
            is_lib,
            false,
        )
        .unwrap();
    } else if *JS_MODE {
        type Exporter = cilly::js_exporter::JSExporter;
        use cilly::asm_exporter::AssemblyExporter;
        Exporter::export_assembly(
            Exporter::default(),
            &final_assembly,
            output_file_path.as_ref(),
            is_lib,
            false,
        )
        .expect("Assembly export faliure!");
        let path: std::path::PathBuf = output_file_path.into();
        final_assembly
            .save_tmp(&mut std::fs::File::create(path.with_extension("cilly")).unwrap())
            .unwrap();
        // Run AOT compiler
        aot_compile_mode.compile(output_file_path);

        //      Cargo integration

        if cargo_support {
            let bootstrap = bootstrap_source(&path, output_file_path, "node");
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
            assert!(
                out.stderr.is_empty(),
                "{}",
                String::from_utf8(out.stderr).unwrap()
            );
        }
    } else {
        // Run ILASM
        export::export_assembly(&final_assembly, output_file_path, is_lib)
            .expect("Assembly export faliure!");
        let path: std::path::PathBuf = output_file_path.into();
        if *CILLY_V2 {
            cilly::v2::Assembly::from_v1(&final_assembly)
                .save_tmp(&mut std::fs::File::create(path.with_extension("cilly2")).unwrap())
                .unwrap()
        }
        final_assembly
            .save_tmp(&mut std::fs::File::create(path.with_extension("cilly")).unwrap())
            .unwrap();
        // Run AOT compiler
        aot_compile_mode.compile(output_file_path);

        //      Cargo integration

        if cargo_support {
            let bootstrap = bootstrap_source(&path, output_file_path, "dotnet");
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
            assert!(
                out.stderr.is_empty(),
                "{}",
                String::from_utf8(out.stderr).unwrap()
            );
        }
    }

    //todo!();
}
fn bootstrap_source(fpath: &Path, output_file_path: &str, jumpstart_cmd: &str) -> String {
    format!(
        include_str!("dotnet_jumpstart.rs"),
        jumpstart_cmd = jumpstart_cmd,
        exec_file = fpath.file_name().unwrap().to_string_lossy(),
        has_native_companion = *NATIVE_PASSTROUGH,
        has_pdb = match *ILASM_FLAVOUR {
            IlasmFlavour::Clasic => false,
            IlasmFlavour::Modern => true,
        },
        pdb_file = match *ILASM_FLAVOUR {
            IlasmFlavour::Clasic => String::new(),
            IlasmFlavour::Modern => format!(
                "{output_file_path}.pdb",
                output_file_path = fpath.file_name().unwrap().to_string_lossy()
            ),
        },
        native_companion_file = if *NATIVE_PASSTROUGH {
            format!(
                "rust_native_{output_file_path}.so",
                output_file_path = file_stem(output_file_path)
            )
        } else {
            String::new()
        }
    )
}
lazy_static! {
    #[doc = "Tells the codegen compile linked static libraries into a shared library, which will be bundled with the .NET executable."]pub static ref NATIVE_PASSTROUGH:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(NATIVE_PASSTROUGH){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(NATIVE_PASSTROUGH),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    #[doc = "Should the codegen stop working when ecountering an error, or try to press on, replacing unusuported code with exceptions throws?"]pub static ref ABORT_ON_ERROR:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(ABORT_ON_ERROR){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(ABORT_ON_ERROR),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    #[doc = "Tells the codegen to emmit C source files."]pub static ref C_MODE:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(C_MODE){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(C_MODE),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    #[doc = "Tells the linker to not remove any dead code."]pub static ref KEEP_DEAD_CODE:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(KEEP_DEAD_CODE){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(KEEP_DEAD_CODE),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    #[doc = "Tells the codegen to emmit JS source files."]pub static ref JS_MODE:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(JS_MODE){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(JS_MODE),value),
        }).unwrap_or(false)
    };
}
fn override_errno(asm: &mut Assembly) {
    for method in asm.methods_mut() {
        if method.name().contains("errno")
            && method.name().contains("os")
            && method.name().contains("unix")
            && method.name().contains("pal")
            && method.name().contains("sys")
            && method.name().contains("std")
        {
            *method = Method::new(
                access_modifier::AccessModifer::Private,
                MethodType::Static,
                method.call_site().signature().clone(),
                method.name(),
                vec![],
                vec![BasicBlock::new(
                    vec![CILRoot::Ret {
                        tree: cilly::call!(
                            CallSite::new(
                                Some(DotnetTypeRef::marshal()),
                                "GetLastWin32Error".into(),
                                FnSig::new(&[], Type::I32),
                                true
                            ),
                            []
                        ),
                    }
                    .into()],
                    0,
                    None,
                )],
                vec![],
            );
        }
    }
}
/*
0x00007ffff74bacc6 in TypeHandle::IsValueType() const () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
(gdb) bt
#0  0x00007ffff74bacc6 in TypeHandle::IsValueType() const () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#1  0x00007ffff7436856 in CEEInfo::isValueClass(CORINFO_CLASS_STRUCT_*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#2  0x00007ffff46be348 in ClassLayout::Create(Compiler*, CORINFO_CLASS_STRUCT_*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#3  0x00007ffff46bf5a9 in ClassLayoutTable::GetObjLayoutIndex(Compiler*, CORINFO_CLASS_STRUCT_*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#4  0x00007ffff46be2fa in Compiler::typGetObjLayout(CORINFO_CLASS_STRUCT_*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#5  0x00007ffff477ac5f in Compiler::lvaSetStruct(unsigned int, CORINFO_CLASS_STRUCT_*, bool) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#6  0x00007ffff479d756 in Lowering::SpillStructCallResult(GenTreeCall*) const () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#7  0x00007ffff4797c6b in Lowering::LowerStoreSingleRegCallStruct(GenTreeBlk*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#8  0x00007ffff4795e22 in Lowering::LowerNode(GenTree*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#9  0x00007ffff479fc8b in Lowering::DoPhase() () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#10 0x00007ffff47e45bc in Phase::Run() () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#11 0x00007ffff46d0b02 in Compiler::compCompile(void**, unsigned int*, JitFlags*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#12 0x00007ffff46d332a in Compiler::compCompileHelper(CORINFO_MODULE_STRUCT_*, ICorJitInfo*, CORINFO_METHOD_INFO*, void**, unsigned int*, JitFlags*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#13 0x00007ffff46d1d8a in Compiler::compCompile(CORINFO_MODULE_STRUCT_*, void**, unsigned int*, JitFlags*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#14 0x00007ffff46d3d1b in jitNativeCode(CORINFO_METHOD_STRUCT_*, CORINFO_MODULE_STRUCT_*, ICorJitInfo*, CORINFO_METHOD_INFO*, void**, unsigned int*, JitFlags*, void*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#15 0x00007ffff46d8ad8 in CILJit::compileMethod(ICorJitInfo*, CORINFO_METHOD_INFO*, unsigned int, unsigned char**, unsigned int*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libclrjit.so
#16 0x00007ffff7445cfa in invokeCompileMethodHelper(EEJitManager*, CEEInfo*, CORINFO_METHOD_INFO*, CORJIT_FLAGS, unsigned char**, unsigned int*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#17 0x00007ffff7445f14 in invokeCompileMethod(EEJitManager*, CEEInfo*, CORINFO_METHOD_INFO*, CORJIT_FLAGS, unsigned char**, unsigned int*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#18 0x00007ffff7446ab7 in UnsafeJitFunction(PrepareCodeConfig*, COR_ILMETHOD_DECODER*, CORJIT_FLAGS*, unsigned int*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#19 0x00007ffff74843ee in MethodDesc::JitCompileCodeLocked(PrepareCodeConfig*, COR_ILMETHOD_DECODER*, ListLockEntryBase<NativeCodeVersion>*, unsigned int*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#20 0x00007ffff7483c8f in MethodDesc::JitCompileCodeLockedEventWrapper(PrepareCodeConfig*, ListLockEntryBase<NativeCodeVersion>*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#21 0x00007ffff74833e8 in MethodDesc::JitCompileCode(PrepareCodeConfig*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#22 0x00007ffff7482da2 in MethodDesc::PrepareILBasedCode(PrepareCodeConfig*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#23 0x00007ffff73ef178 in CodeVersionManager::PublishVersionableCodeIfNecessary(MethodDesc*, CallerGCMode, bool*, bool*) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#24 0x00007ffff748766b in MethodDesc::DoPrestub(MethodTable*, CallerGCMode) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#25 0x00007ffff7487163 in PreStubWorker () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#26 0x00007ffff76b8a00 in ThePreStub () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#27 0x00007ffff76b7aef in CallDescrWorkerInternal () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#28 0x00007ffff74e3c96 in DispatchCallDebuggerWrapper(CallDescrData*, int) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#29 0x00007ffff74e3ec7 in DispatchCallSimple(unsigned long*, unsigned int, unsigned long, unsigned int) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#30 0x00007ffff7462efb in MethodTable::RunClassInitEx(Object**) () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#31 0x00007ffff7463409 in MethodTable::DoRunClassInitThrowing() () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#32 0x00007ffff7544739 in JIT_GetSharedNonGCStaticBase_Helper () from /usr/lib64/dotnet/shared/Microsoft.NETCore.App/8.0.4/libcoreclr.so
#33 0x00007fff78f4196e in ?? ()
#34 0x00007fff00000000 in ?? ()
#35 0x0000000000000000 in ?? ()

*/
