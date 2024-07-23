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
#[allow(dead_code)]
struct Test<T> {
    data: T,
}
include!("../common.rs");
fn main() {
    test_eq!(!black_box(true), black_box(false));
    test_eq!(!black_box(0b1111_0000u8), black_box(0b0000_1111u8));
    test_eq!(!black_box(-66), black_box(65i8));
}
