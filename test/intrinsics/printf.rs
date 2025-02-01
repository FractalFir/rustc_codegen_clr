#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
include!("../common.rs");
fn main() {
    unsafe { printf("Using printf. num0:%i\0".as_ptr() as *const _, 64) };
    unsafe {
        printf(
            "Using printf. bob num0:%f num1:%i\0".as_ptr() as *const _,
            64.7,
            55,
        )
    };
}
