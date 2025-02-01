#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn main() {
    let strig = unsafe {
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(malloc(64) as *mut _, 64))
    };
    black_box(strig);
}
