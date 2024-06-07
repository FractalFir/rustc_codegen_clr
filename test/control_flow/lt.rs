#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
#![no_std]
include!("../common.rs");

fn main() {
    test_eq!(branch(4), true)
}

fn branch(a: i32) -> bool {
    if black_box(a) < 5 {
        return true;
    }
    return false;
}
