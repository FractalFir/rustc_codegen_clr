#![allow(dead_code)]
use cilly::IString;
use fxhash::{FxBuildHasher, FxHashMap};

use crate::file_stem;
use crate::load::LinkableFile;

use std::io::Write;
pub struct NativePastroughInfo {
    defs: FxHashMap<IString, IString>,
}
impl NativePastroughInfo {
    pub fn new() -> Self {
        Self {
            defs: FxHashMap::with_hasher(FxBuildHasher::default()),
        }
    }
    pub fn insert(&mut self, k: IString, v: impl Into<IString>) -> Option<IString> {
        self.defs.insert(k, v.into())
    }

    pub fn get(&self, k: &str) -> Option<&IString> {
        self.defs.get(k)
    }
}
/// Compiles all the linked object files into one shared lib, and then generates the info neccessary for creating `PInvoke` declarations used to call the functions within them.  
/// Uses `gcc`, so may not work on other platforms.
pub fn handle_native_passtrough(
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
// Adds the shared library at `file_path` to the native passtrough list. Also generates the info  neccessary for creating PInvoke declarations used to call the functions within them.
// Uses `nm` to get function names from a `.so` file, so it is not cross-platform.
#[cfg(target_os = "linux")]
pub fn add_shared(file_path: &str, native_pastrough: &mut NativePastroughInfo) {
    use cilly::IString;

    let nm = std::process::Command::new("nm")
        .arg("-D")
        .arg(file_path)
        .output()
        .unwrap();
    //let file_path = AString::new(format!("{}.{}",file_stem(file_path),file_ext(file_path)).into());
    //eprintln!("file_path:{file_path}");
    let file_path: IString = file_path.into();
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
pub fn add_shared(file_path: &str, native_pastrough: &mut NativePastroughInfo) {
    panic!("Native passtrough not supported on this platfrom.")
}
// Detects all the link directiores provided by the linker,
fn link_directories2(args: &[String]) -> Vec<String> {
    let mut directories = Vec::new();
    let mut after_l = false;

    for string in args {
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
