#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
#![allow(dead_code, unused_imports)]
include!("../common.rs");

use core::ptr;
use core::sync::atomic::{AtomicIsize, AtomicPtr, Ordering};

// The system-provided argc and argv, which we store in static memory
// here so that we can defer the work of parsing them until its actually
// needed.
//
// Note that we never mutate argv/argc, the argv array, or the argv
// strings, which allows the code in this file to be very simple.
static ARGC: AtomicIsize = AtomicIsize::new(0);
static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(ptr::null_mut());
extern "C" {
    fn fork() -> i32;
    static mut environ: *mut *mut i8;
}
unsafe fn really_init(argc: isize, argv: *const *const u8) {
    // These don't need to be ordered with each other or other stores,
    // because they only hold the unmodified system-provide argv/argc.
    ARGC.store(argc, Ordering::Relaxed);
    ARGV.store(argv as *mut _, Ordering::Relaxed);
}

#[inline(always)]
pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
    // On Linux-GNU, we rely on `ARGV_INIT_ARRAY` below to initialize
    // `ARGC` and `ARGV`. But in Miri that does not actually happen so we
    // still initialize here.
    #[cfg(any(miri, not(all(target_os = "linux", target_env = "gnu"))))]
    really_init(_argc, _argv);
}

/// glibc passes argc, argv, and envp to functions in .init_array, as a non-standard extension.
/// This allows `std::env::args` to work even in a `cdylib`, as it does on macOS and Windows.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[used]
#[link_section = ".init_array.00099"]
static ARGV_INIT_ARRAY: extern "C" fn(core::ffi::c_int, *const *const u8, *const *const u8) = {
    extern "C" fn init_wrapper(
        argc: core::ffi::c_int,
        argv: *const *const u8,
        _envp: *const *const u8,
    ) {
        unsafe {
            really_init(argc as isize, argv);
        }
    }
    init_wrapper
};

#[no_mangle]
fn load_environ() -> *mut *mut i8 {
    unsafe { environ }
}
fn main() {
    test_ne!(ARGV.load(Ordering::Relaxed), ptr::null_mut());
    unsafe {
        printf(c"%p\n".as_ptr(), load_environ());
    };
    let mut i = 0_isize;
    unsafe {
        while !(*environ.offset(i)).is_null() {
            let fresh0 = i;
            i = i + 1;
            printf(
                b"%s\n\0" as *const u8 as *const i8,
                *environ.offset(fresh0 as isize),
            );
        }
    }
    // Ensure that fork returns -1, to indicate it is unsupported.
    test_eq!(unsafe { fork() }, -1);
}
