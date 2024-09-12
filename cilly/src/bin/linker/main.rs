#![deny(unused_must_use)]
#![allow(clippy::module_name_repetitions)]
use cilly::{
    asm::DEAD_CODE_ELIMINATION,
    call_site::CallSite,
    conv_usize,
    libc_fns::{self, LIBC_FNS, LIBC_MODIFIES_ERRNO},
    v2::{
        asm::{MissingMethodPatcher, ILASM_FLAVOUR},
        cilnode::MethodKind,
        Assembly, BasicBlock, CILNode, CILRoot, ClassDef, ClassRef, Const, FnSig, IlasmFlavour,
        Int, MethodImpl, Type,
    },
};
//use assembly::Assembly;
use lazy_static::lazy_static;

mod cmd;
mod export;
mod load;
mod native_passtrough;
mod patch;
use fxhash::FxHashMap;
use patch::call_alias;
use std::{env, io::Write, num::NonZeroU32, path::Path};
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
}

lazy_static! {
    static ref LIBC: String = get_libc_();
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
    /*
       {
           let msg = final_assembly.alloc_string("Starting constant initialization");
           let msg = final_assembly.alloc_node(Const::PlatformString(msg));
           let console = ClassRef::console(&mut final_assembly);
           let fn_name = final_assembly.alloc_string("WriteLine");
           let mref = final_assembly.class_ref(console).clone().static_mref(
               &[Type::PlatformString],
               Type::Void,
               fn_name,
               &mut final_assembly,
           );
           let stat = final_assembly.alloc_root(CILRoot::Call(Box::new((mref, [msg].into()))));
           final_assembly.add_cctor(&[stat]);
       }
    */
    let path: std::path::PathBuf = output_file_path.into();

    let is_lib = output_file_path.contains(".dll")
        || output_file_path.contains(".so")
        || output_file_path.contains(".o");

    let mut externs: FxHashMap<_, _> = LIBC_FNS
        .iter()
        .map(|fn_name| (*fn_name, LIBC.to_string()))
        .collect();

    let modifies_errno = LIBC_MODIFIES_ERRNO.iter().copied().collect();
    if let Some(f128_support) = libc_fns::f128_support_lib() {
        let f128_support = f128_support.to_str().to_owned().unwrap();
        externs.extend(
            libc_fns::F128_SYMBOLS
                .iter()
                .map(|fn_name| (*fn_name, f128_support.to_owned())),
        );
    }
    let mathf = LIBM.to_owned();
    externs.extend(
        libc_fns::LIBM_FNS
            .iter()
            .map(|fn_name| (*fn_name, mathf.to_owned())),
    );
    let mut overrides: MissingMethodPatcher = FxHashMap::default();
    overrides.insert(
        final_assembly.alloc_string("pthread_atfork"),
        Box::new(|_, asm: &mut Assembly| {
            let ret_val = asm.alloc_node(Const::I32(0));
            let blocks = vec![BasicBlock::new(
                vec![asm.alloc_root(CILRoot::Ret(ret_val))],
                1,
                None,
            )];
            MethodImpl::MethodBody {
                blocks,
                locals: vec![],
            }
        }),
    );
    // Override allocator
    {
        // Get the marshal class
        let marshal = ClassRef::marshal(&mut final_assembly);
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
    if !*PANIC_MANAGED_BT {
        overrides.insert(
            final_assembly.alloc_string("_Unwind_RaiseException"),
            Box::new(|_, asm| {
                let rust_exception = asm.alloc_string("RustException");
                MethodImpl::MethodBody {
                    blocks: vec![cilly::v2::BasicBlock::from_v1(
                        &cilly::basic_block::BasicBlock::new(
                            vec![cilly::cil_root::CILRoot::Throw(
                                cilly::cil_node::CILNode::NewObj(Box::new(
                                    cilly::cil_node::CallOpArgs {
                                        args: Box::new([conv_usize!(
                                            cilly::cil_node::CILNode::LDArg(0)
                                        )]),
                                        site: Box::new(CallSite::new(
                                            Some(asm.alloc_class_ref(ClassRef::new(
                                                rust_exception,
                                                None,
                                                false,
                                                [].into(),
                                            ))),
                                            ".ctor".into(),
                                            FnSig::new(
                                                Box::new([
                                                    Type::ClassRef(asm.alloc_class_ref(
                                                        ClassRef::new(
                                                            rust_exception,
                                                            None,
                                                            false,
                                                            [].into(),
                                                        ),
                                                    )),
                                                    Type::Int(Int::USize),
                                                ]),
                                                Type::Void,
                                            ),
                                            false,
                                        )),
                                    },
                                )),
                            )
                            .into()],
                            0,
                            None,
                        ),
                        asm,
                    )],
                    locals: vec![],
                }
            }),
        );
    }

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
        final_assembly.alloc_string("fork"),
        Box::new(|_, asm| {
            let ret_val = asm.alloc_node(Const::I32(-1));
            let blocks = vec![BasicBlock::new(
                vec![asm.alloc_root(CILRoot::Ret(ret_val))],
                0,
                None,
            )];
            MethodImpl::MethodBody {
                blocks,
                locals: vec![],
            }
        }),
    );
    overrides.insert(
        final_assembly.alloc_string("__cxa_thread_atexit_impl"),
        Box::new(|_, asm| {
            let blocks = vec![BasicBlock::new(
                vec![asm.alloc_root(CILRoot::VoidRet)],
                0,
                None,
            )];
            MethodImpl::MethodBody {
                blocks,
                locals: vec![],
            }
        }),
    );
    cilly::v2::builtins::atomics::generate_all_atomics(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::casts::insert_casts(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::select::generate_int_selects(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::insert_heap(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::instert_threading(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::insert_swap_at_generic(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::insert_bounds_check(&mut final_assembly, &mut overrides);
    cilly::v2::builtins::math::math(&mut final_assembly, &mut overrides);
    // Ensure the cctor and tcctor exist!
    let _ = final_assembly.tcctor();
    let _ = final_assembly.cctor();
    let float128 = final_assembly.alloc_string("f128");
    final_assembly.class_def(ClassDef::new(
        float128,
        true,
        0,
        None,
        vec![],
        vec![],
        cilly::v2::Access::Public,
        NonZeroU32::new(16),
    ));

    final_assembly.patch_missing_methods(externs, modifies_errno, overrides);

    add_mandatory_statics(&mut final_assembly);
    if *DEAD_CODE_ELIMINATION {
        println!("Eliminating dead code");
        final_assembly.eliminate_dead_code();
    }
    let mut fuel = final_assembly.fuel_from_env().fraction(0.1);
    final_assembly.opt(&mut fuel);
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
    if let Err(err) = std::fs::remove_file(output_file_path) {
        match err.kind() {
            std::io::ErrorKind::NotFound => (),
            _ => {
                panic!("Could not remove tmp file because {err:?}")
            }
        }
    };
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
                output_file_path = fpath.file_stem().unwrap().to_string_lossy()
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
    #[doc = "Tells the codegen to emmit Java source files."]pub static ref JAVA_MODE:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(JAVA_MODE){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(JAVA_MODE),value),
        }).unwrap_or(false)
    };
}
lazy_static! {
    #[doc = "Tells the codegen to throw exceptions on panics"]pub static ref PANIC_MANAGED_BT:bool = {
        std::env::vars().find_map(|(key,value)|if key == stringify!(PANIC_MANAGED_BT){
            Some(value)
        }else {
            None
        }).map(|value|match value.as_ref(){
            "0"|"false"|"False"|"FALSE" => false,"1"|"true"|"True"|"TRUE" => true,_ => panic!("Boolean enviroment variable {} has invalid value {}",stringify!(PANIC_MANAGED_BT),value),
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
                                Some(ClassRef::marshal()),
                                "GetLastWin32Error".into(),
                                FnSig::new(&[], Type::Int(Int::I32)),
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
