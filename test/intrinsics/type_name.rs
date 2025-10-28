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

use core::intrinsics::type_name;

include!("../common.rs");

fn main() {
    test_eq!(
        type_name::<Option<u32>>(),
        black_box("core::option::Option<core::primitive::u32>")
    );
}
