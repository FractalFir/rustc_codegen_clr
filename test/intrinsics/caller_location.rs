#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
#![no_std]
extern crate core;
include!("../common.rs");
fn main() {
    let cloc = black_box(core::intrinsics::caller_location());
    let file = cloc.file();
    let fcopy = unsafe { malloc(file.len() + 1) as *mut u8 };
    unsafe {
        core::ptr::copy_nonoverlapping(file.as_ptr(), fcopy, file.len());
        *fcopy.offset((file.len()) as isize) = b'\0';
    }
    unsafe {
        printf(
            "%s:%d:%d\n\0".as_ptr() as *const core::ffi::c_char,
            fcopy,
            cloc.line(),
            cloc.column(),
        );
    }
}
