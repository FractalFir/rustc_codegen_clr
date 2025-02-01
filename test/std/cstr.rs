#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports,
    unused_mut,
    private_interfaces,
    non_upper_case_globals
)]
#![allow(dead_code)]
use core::ffi::CStr;
include!("../common.rs");
fn main() {
    let cstr = CStr::from_bytes_until_nul(b"Hi bob!\0").unwrap();
    unsafe { printf(cstr.as_ptr()) };
}
