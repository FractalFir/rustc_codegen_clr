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
    let a: i16 = black_box(-23535);
    let b = a as u64;
    test_eq!(b, 18446744073709528081_u64);
}
