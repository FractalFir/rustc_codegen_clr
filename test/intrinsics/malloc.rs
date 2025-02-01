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
    unused_unsafe,
    unused_imports
)]
include!("../common.rs");
fn main() {
    unsafe {
        let mut buff = malloc(64);
        test_ne!(buff, 0_usize as *mut _);
        buff = realloc(buff, 128);
        test_ne!(buff, 0_usize as *mut _);
        free(buff);
        let tmp = __rust_alloc(64, 8);
    }
}
