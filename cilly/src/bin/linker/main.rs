#![deny(unused_must_use)]
#![allow(clippy::module_name_repetitions)]
use cilly::{
    asm::DEAD_CODE_ELIMINATION,
    call_site::CallSite,
    conv_usize,
    libc_fns::{LIBC_FNS, LIBC_MODIFIES_ERRNO},
    v2::{
        asm::{MissingMethodPatcher, ILASM_FLAVOUR},
        cilnode::MethodKind,
        BasicBlock, CILNode, CILRoot, ClassRef, IlasmFlavour, Int, MethodImpl, Type,
    },
    DotnetTypeRef, FnSig,
};
//use assembly::Assembly;
use lazy_static::lazy_static;

mod cmd;
mod export;
mod load;
mod native_passtrough;
mod patch;
use fxhash::FxHashMap;
use patch::{builtin_call, call_alias};
use std::{env, io::Write, path::Path};
mod aot;

fn add_mandatory_statics(asm: &mut cilly::v2::Assembly) {
    let main_module = asm.main_module();
    asm.add_static(
        cilly::v2::Type::Int(cilly::v2::Int::U8),
        "__rust_alloc_error_handler_should_panic",
        false,
        main_module,
    );
    asm.add_static(
        cilly::v2::Type::Int(cilly::v2::Int::U8),
        "__rust_no_alloc_shim_is_unstable",
        false,
        main_module,
    );
    let inner = asm.nptr(Type::Int(cilly::v2::Int::U8));
    let tpe = asm.nptr(inner);
    asm.add_static(tpe, "environ", false, main_module);
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

    let cargo_support = args.iter().any(|arg| arg.contains("--cargo-support"));

    // Load assemblies from files

    let (mut final_assembly, _) = load::load_assemblies(to_link.as_slice(), ar_to_link.as_slice());

    let path: std::path::PathBuf = output_file_path.into();

    let is_lib = output_file_path.contains(".dll")
        || output_file_path.contains(".so")
        || output_file_path.contains(".o");

    let externs = LIBC_FNS
        .iter()
        .map(|fn_name| (*fn_name, LIBC.as_ref()))
        .collect();
    let modifies_errno = LIBC_MODIFIES_ERRNO.iter().copied().collect();
    let mut overrides: MissingMethodPatcher = FxHashMap::default();
    // Override allocator
    {
        // Get the marshal class
        let marshal = ClassRef::from_v1(&DotnetTypeRef::marshal(), &mut final_assembly);
        let marshal = final_assembly.alloc_class_ref(marshal);
        // Overrides calls to malloc
        let sig = final_assembly.sig([Type::Int(Int::ISize)], Type::Int(Int::ISize));
        let allochglobal =
            final_assembly.new_methodref(marshal, "AllocHGlobal", sig, MethodKind::Static, []);
        let mref = final_assembly.get_mref(allochglobal).clone();
        call_alias(&mut overrides, &mut final_assembly, "malloc", mref);
        // Overrides calls to realloc
        let sig = final_assembly.sig(
            [Type::Int(Int::ISize), Type::Int(Int::ISize)],
            Type::Int(Int::ISize),
        );
        let realloc =
            final_assembly.new_methodref(marshal, "ReAllocHGlobal", sig, MethodKind::Static, []);
        let mref = final_assembly.get_mref(realloc).clone();
        call_alias(&mut overrides, &mut final_assembly, "realloc", mref);
        // Overrides calls to free
        let sig = final_assembly.sig([Type::Int(Int::ISize)], Type::Void);
        let allochglobal =
            final_assembly.new_methodref(marshal, "FreeHGlobal", sig, MethodKind::Static, []);
        let mref = final_assembly.get_mref(allochglobal).clone();
        call_alias(&mut overrides, &mut final_assembly, "free", mref);
    }
    // Override pthreads
    {
        // Override pthread create
        let void_ptr = final_assembly.nptr(Type::Void);

        let fn_ptr = final_assembly.fn_ptr([void_ptr], void_ptr);
        let isize_ptr = final_assembly.nptr(Type::Int(Int::ISize));
        let sig = final_assembly.sig([isize_ptr, void_ptr, fn_ptr, void_ptr], Type::Int(Int::I32));
        builtin_call(&mut overrides, &mut final_assembly, "pthread_create", sig);
        let sig = final_assembly.sig([isize_ptr], Type::Int(Int::I32));
        builtin_call(
            &mut overrides,
            &mut final_assembly,
            "pthread_attr_init",
            sig,
        );
        builtin_call(
            &mut overrides,
            &mut final_assembly,
            "pthread_attr_destroy",
            sig,
        );
        let sig = final_assembly.sig([isize_ptr, Type::Int(Int::USize)], Type::Int(Int::I32));
        builtin_call(
            &mut overrides,
            &mut final_assembly,
            "pthread_attr_setstacksize",
            sig,
        );
        let void_ptr_ptr = final_assembly.nptr(void_ptr);
        let sig = final_assembly.sig([Type::Int(Int::ISize), void_ptr_ptr], Type::Int(Int::I32));
        builtin_call(&mut overrides, &mut final_assembly, "pthread_join", sig);
        let sig = final_assembly.sig([], Type::Int(Int::ISize));
        builtin_call(&mut overrides, &mut final_assembly, "pthread_self", sig);
    }
    overrides.insert(
        final_assembly.alloc_string("_Unwind_RaiseException"),
        Box::new(|_, asm| MethodImpl::MethodBody {
            blocks: vec![cilly::v2::BasicBlock::from_v1(
                &cilly::basic_block::BasicBlock::new(
                    vec![
                        cilly::cil_root::CILRoot::Throw(cilly::cil_node::CILNode::NewObj(
                            Box::new(cilly::cil_node::CallOpArgs {
                                args: Box::new([conv_usize!(cilly::cil_node::CILNode::LDArg(0))]),
                                site: Box::new(CallSite::new(
                                    Some(
                                        DotnetTypeRef::new::<&str, _>(None, "RustException")
                                            .with_valuetype(false),
                                    ),
                                    ".ctor".into(),
                                    FnSig::new(
                                        &[
                                            cilly::r#type::Type::DotnetType(Box::new(
                                                DotnetTypeRef::new::<&str, _>(
                                                    None,
                                                    "RustException",
                                                )
                                                .with_valuetype(false),
                                            )),
                                            cilly::r#type::Type::USize,
                                        ],
                                        cilly::r#type::Type::Void,
                                    ),
                                    false,
                                )),
                            }),
                        ))
                        .into(),
                    ],
                    0,
                    None,
                ),
                asm,
            )],
            locals: vec![],
        }),
    );
    overrides.insert(
        final_assembly.alloc_string("_Unwind_Backtrace"),
        Box::new(|mref, asm| {
            // 1 Get the output of the method.
            let mref = asm.get_mref(mref);
            let sig = asm.get_sig(mref.sig()).clone();
            let output = sig.output();
            // 2. Create one local of the output type
            let loc_name = asm.alloc_string("uninit");
            let locals = vec![(Some(loc_name), asm.alloc_type(*output))];
            // 3. Create CIL returning an uninitalized value of this type. TODO: even tough this value is shortly discarded on the Rust side, this is UB. Consider zero-initializing it.
            let loc = asm.alloc_node(CILNode::LdLoc(0));
            let ret = asm.alloc_root(CILRoot::Ret(loc));
            let blocks = vec![BasicBlock::new(vec![ret], 0, None)];
            MethodImpl::MethodBody { blocks, locals }
        }),
    );
    overrides.insert(
        final_assembly.alloc_string("_Unwind_DeleteException"),
        Box::new(|_, asm| {
            let ret = asm.alloc_root(CILRoot::VoidRet);
            let blocks = vec![BasicBlock::new(vec![ret], 0, None)];
            MethodImpl::MethodBody {
                blocks,
                locals: vec![],
            }
        }),
    );
    overrides.insert(
        final_assembly.alloc_string("rust_begin_unwind"),
        Box::new(|_, asm| {
            MethodImpl::AliasFor(
                *asm.find_methods_named("rust_begin_unwind")
                    .unwrap()
                    .next()
                    .unwrap(),
            )
        }),
    );
    final_assembly.patch_missing_methods(externs, modifies_errno, overrides);

    add_mandatory_statics(&mut final_assembly);
    if *DEAD_CODE_ELIMINATION {
        println!("Eliminating dead code");
        final_assembly.eliminate_dead_code();
    }

    final_assembly
        .save_tmp(&mut std::fs::File::create(path.with_extension("cilly2")).unwrap())
        .unwrap();
    if *C_MODE {
        final_assembly.export(
            &path,
            cilly::v2::il_exporter::ILExporter::new(*ILASM_FLAVOUR, is_lib),
        );
    } else if *JS_MODE {
        todo!();
    } else if *JAVA_MODE {
        final_assembly.export(&path, cilly::v2::java_exporter::JavaExporter::new(is_lib));
        if cargo_support {
            let bootstrap =
                bootstrap_source(&path.with_extension("jar"), path.to_str().unwrap(), "java");
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
        final_assembly.export(
            &path,
            cilly::v2::il_exporter::ILExporter::new(*ILASM_FLAVOUR, is_lib),
        );
        if cargo_support {
            let bootstrap = bootstrap_source(
                &path.with_extension("exe"),
                path.to_str().unwrap(),
                "dotnet",
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
lazy_static! {
    #[doc = "Tells the codegen to emmit JS source files."]pub static ref JAVA_MODE:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(JAVA_MODE){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(JAVA_MODE),value),
        }).unwrap_or(false)
    };
}
/*
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
}*/
