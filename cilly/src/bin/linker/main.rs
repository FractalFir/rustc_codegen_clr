#![deny(unused_must_use)]
#![allow(clippy::module_name_repetitions)]
use cilly::{
    cilnode::IsPure,
    config, conv_usize,
    libc_fns::{self, LIBC_FNS, LIBC_MODIFIES_ERRNO},
    MethodRef, DEAD_CODE_ELIMINATION,
    {
        asm::{MissingMethodPatcher, ILASM_FLAVOUR},
        cilnode::MethodKind,
        Assembly, BasicBlock, CILNode, CILRoot, ClassDef, ClassRef, Const, IlasmFlavour, Int,
        MethodImpl, Type,
    },
};
mod load;
mod native_passtrough;
mod patch;
use fxhash::FxHashMap;
use patch::call_alias;
use std::{
    env,
    io::Write,
    num::NonZeroU32,
    path::{Path, PathBuf},
};
mod aot;

fn add_mandatory_statics(asm: &mut cilly::Assembly) {
    let main_module = asm.main_module();
    asm.add_static(
        cilly::Type::Int(cilly::Int::U8),
        "__rust_alloc_error_handler_should_panic",
        false,
        main_module,
    );
    asm.add_static(
        cilly::Type::Int(cilly::Int::U8),
        "__rust_no_alloc_shim_is_unstable",
        false,
        main_module,
    );
}
static FORCE_FAIL: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::env::var("FORCE_FAIL").is_ok());
static LIBC: std::sync::LazyLock<String> = std::sync::LazyLock::new(get_libc_);
static LIBM: std::sync::LazyLock<String> = std::sync::LazyLock::new(get_libm_);
static BACKUP_STD: std::sync::LazyLock<Option<PathBuf>> = std::sync::LazyLock::new(|| {
    std::env::vars()
        .filter_map(|(key, value)| {
            if key == "BACKUP_STD" {
                Some(value.into())
            } else {
                None
            }
        })
        .next()
});
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
    "msvcrt.dll".to_string()
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
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_out_path(args: &[String]) -> &str {
    &args[1 + args
        .iter()
        .position(|arg| *arg == "-o")
        .unwrap_or_else(|| panic!("No output file! {args:?}"))]
}
#[cfg(target_os = "windows")]
fn get_out_path<'a>(args: &'a [String]) -> &'a str {
    args.iter()
        .filter_map(|arg| arg.strip_prefix("/OUT:"))
        .next()
        .expect(&format!("No output file! {args:?}"))
}
fn link_dir(path: &Path, ar_to_link: &mut Vec<String>) {
    let dir = std::fs::read_dir(path).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() && entry.file_name().to_str().unwrap().contains(".rlib") {
            ar_to_link.push(entry.path().to_str().unwrap().to_owned());
            eprintln!("Linking file {:?}.", entry.file_name());
        }
    }
}
// Links a prebuilt std if none present
fn link_backup_std(to_link: &[&String], ar_to_link: &mut Vec<String>, backup: &Path) {
    if !to_link.iter().chain(to_link).any(|linkable| {
        linkable.contains("std") | linkable.contains("core") | linkable.contains("alloc")
    }) {
        link_dir(backup, ar_to_link);
    }
}
fn extract_libs(args: &[String]) -> Vec<String> {
    args.iter()
        .filter(|arg| arg[..2] == *"-l")
        .map(|arg| arg.to_string())
        .collect()
}
fn extract_dirs(args: &[String]) -> Vec<String> {
    args.iter()
        .filter(|arg| arg[..2] == *"-B")
        .map(|arg| arg.to_string())
        .collect()
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
    let mut ar_to_link: Vec<_> = args
        .iter()
        .filter(|arg| arg.contains(".rlib"))
        .cloned()
        .collect();

    if let Some(backup_std) = BACKUP_STD.as_ref() {
        link_backup_std(&to_link[..], &mut ar_to_link, backup_std);
    }
    //ar_to_link.extend(link_dir_files(args));
    let output_file_path = get_out_path(args);
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
           let stat = final_assembly.alloc_root(CILRoot::call(((mref, [msg].into()))));
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
    externs.extend(
        libc_fns::GB_FNS
            .iter()
            .map(|fn_name| (*fn_name, "gameboy".to_owned())),
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
    if !*C_MODE {
        // Get the marshal class
        let marshal = ClassRef::marshal(&mut final_assembly);
        // Overrides calls to malloc
        let sig = final_assembly.sig([Type::Int(Int::ISize)], Type::Int(Int::ISize));
        let allochglobal =
            final_assembly.new_methodref(marshal, "AllocHGlobal", sig, MethodKind::Static, []);
        let mref = final_assembly[allochglobal].clone();
        call_alias(&mut overrides, &mut final_assembly, "malloc", mref);
        // Overrides calls to realloc
        let sig = final_assembly.sig(
            [Type::Int(Int::ISize), Type::Int(Int::ISize)],
            Type::Int(Int::ISize),
        );
        let realloc =
            final_assembly.new_methodref(marshal, "ReAllocHGlobal", sig, MethodKind::Static, []);
        let mref = final_assembly[realloc].clone();
        call_alias(&mut overrides, &mut final_assembly, "realloc", mref);
        // Overrides calls to free
        let sig = final_assembly.sig([Type::Int(Int::ISize)], Type::Void);
        let allochglobal =
            final_assembly.new_methodref(marshal, "FreeHGlobal", sig, MethodKind::Static, []);
        let mref = final_assembly[allochglobal].clone();
        call_alias(&mut overrides, &mut final_assembly, "free", mref);
    } else {
        let void_ptr = final_assembly.nptr(Type::Void);
        let sig = final_assembly.sig(
            [void_ptr, void_ptr, void_ptr, void_ptr],
            Type::Int(Int::I32),
        );
        let main_module = final_assembly.main_module();
        let allochglobal = final_assembly.new_methodref(
            *main_module,
            "pthread_create_wrapper",
            sig,
            MethodKind::Static,
            [],
        );
        let mref = final_assembly[allochglobal].clone();
        externs.insert("pthread_create_wrapper", LIBC.clone());
        call_alias(&mut overrides, &mut final_assembly, "pthread_create", mref);
    }
    if !*PANIC_MANAGED_BT {
        overrides.insert(
            final_assembly.alloc_string("_Unwind_RaiseException"),
            Box::new(|_, asm| {
                let rust_exception = asm.alloc_string("RustException");
                let exception_class =
                    asm.alloc_class_ref(ClassRef::new(rust_exception, None, false, [].into()));
                let exception_ctor = MethodRef::new(
                    asm.alloc_class_ref(ClassRef::new(rust_exception, None, false, [].into())),
                    asm.alloc_string(".ctor"),
                    asm.sig(
                        [Type::ClassRef(exception_class), Type::Int(Int::USize)],
                        Type::Void,
                    ),
                    MethodKind::Constructor,
                    vec![].into(),
                );
                MethodImpl::MethodBody {
                    blocks: vec![cilly::BasicBlock::from_v1(
                        &cilly::basic_block::BasicBlock::new(
                            vec![cilly::cil_root::CILRoot::Throw(
                                cilly::cil_node::CILNode::NewObj(Box::new(
                                    cilly::cil_node::CallOpArgs {
                                        args: Box::new([conv_usize!(
                                            cilly::cil_node::CILNode::LDArg(0)
                                        )]),
                                        site: asm.alloc_methodref(exception_ctor),
                                        is_pure: IsPure::NOT,
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
    if !*C_MODE {
        overrides.insert(
            final_assembly.alloc_string("_Unwind_Backtrace"),
            Box::new(|mref, asm| {
                // 1 Get the output of the method.
                let mref = &asm[mref];
                let sig = asm[mref.sig()].clone();
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
    }

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
    cilly::builtins::select::generate_int_selects(&mut final_assembly, &mut overrides);
    cilly::builtins::insert_swap_at_generic(&mut final_assembly, &mut overrides);
    cilly::builtins::insert_bounds_check(&mut final_assembly, &mut overrides);
    cilly::builtins::casts::insert_casts(&mut final_assembly, &mut overrides);
    cilly::builtins::insert_heap(&mut final_assembly, &mut overrides, *C_MODE);
    cilly::builtins::int128::generate_int128_ops(&mut final_assembly, &mut overrides, *C_MODE);
    cilly::builtins::int128::i128_mul_ovf_check(&mut final_assembly, &mut overrides);
    cilly::builtins::f16::generate_f16_ops(&mut final_assembly, &mut overrides, *C_MODE);
    cilly::builtins::atomics::generate_all_atomics(&mut final_assembly, &mut overrides);
    cilly::builtins::stack_addr(&mut final_assembly, &mut overrides);
    cilly::builtins::transmute(&mut final_assembly, &mut overrides);
    cilly::builtins::create_slice(&mut final_assembly, &mut overrides);
    cilly::builtins::ovf_check_tuple(&mut final_assembly, &mut overrides);
    cilly::builtins::uninit_val(&mut final_assembly, &mut overrides);

    cilly::builtins::math::bitreverse(&mut final_assembly, &mut overrides);

    if *C_MODE {
        cilly::builtins::insert_exeception_stub(&mut final_assembly, &mut overrides);
        externs.insert("__dso_handle", LIBC.clone());
        externs.insert("_mm_malloc", LIBC.clone());
        externs.insert("_mm_free", LIBC.clone());
        externs.insert("abort", LIBC.clone());
        for fnc in [
            "pthread_getattr_np",
            "pthread_attr_getguardsize",
            "pthread_attr_getstack",
            "pthread_attr_destroy",
            "pthread_self",
            "pthread_create",
            "pthread_detach",
            "pthread_attr_setstacksize",
            "pthread_attr_init",
            "pthread_setname_np",
            "pthread_key_create",
            "pthread_key_delete",
            "pthread_join",
            "pthread_setspecific",
            "ldexpf",
        ] {
            externs.insert(fnc, LIBC.clone());
        }
        overrides.insert(
            final_assembly.alloc_string("argc_argv_init"),
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
    } else {
        cilly::builtins::instert_threading(&mut final_assembly, &mut overrides);
        cilly::builtins::math::math(&mut final_assembly, &mut overrides);
        cilly::builtins::simd::simd(&mut final_assembly, &mut overrides);
        cilly::builtins::insert_exception(&mut final_assembly, &mut overrides);
        cilly::builtins::argc_argv_init(&mut final_assembly, &mut overrides);
    }

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
        cilly::Access::Public,
        NonZeroU32::new(16),
        NonZeroU32::new(16),
        true,
    ));

    final_assembly.patch_missing_methods(&externs, &modifies_errno, &overrides);
    final_assembly.patch_missing_methods(&externs, &modifies_errno, &overrides);

    add_mandatory_statics(&mut final_assembly);

    if *DEAD_CODE_ELIMINATION {
        println!("Eliminating dead code");
        final_assembly.eliminate_dead_code();
    }
    let mut fuel = final_assembly.fuel_from_env().fraction(0.25);
    final_assembly.opt(&mut fuel);
    final_assembly.eliminate_dead_code();
    final_assembly.fix_aligement();
    final_assembly
        .save_tmp(&mut std::fs::File::create(path.with_extension("cilly2")).unwrap())
        .unwrap();
    let libs = extract_libs(args);
    let dirs = extract_dirs(args);
    if *FORCE_FAIL {
        panic!("FORCE_FAIL");
    }
    if *C_MODE {
        let cexport = cilly::c_exporter::CExporter::new(is_lib, libs, dirs);

        final_assembly.export(&path, cexport);
    } else if *JAVA_MODE {
        final_assembly.export(&path, cilly::java_exporter::JavaExporter::new(is_lib));
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
            cilly::il_exporter::ILExporter::new(*ILASM_FLAVOUR, is_lib),
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
config!(NATIVE_PASSTROUGH, bool, false);
config!(ABORT_ON_ERROR, bool, false);
config!(C_MODE, bool, false);
config!(JAVA_MODE, bool, false);
config!(PANIC_MANAGED_BT, bool, false);
/*
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
}*/
