#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
include!("../common.rs");
extern "C" {
    static __rust_no_alloc_shim_is_unstable: u8;
}

fn test_ref_deref() {
    black_box(unsafe { core::ptr::read_volatile(&__rust_no_alloc_shim_is_unstable) });
}
fn main() {
    let two = black_box(2);
    black_box(test_ref_deref());
    let three = black_box(3);
}
