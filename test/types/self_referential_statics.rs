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
    improper_ctypes_definitions
)]
#![no_std]
include!("../common.rs");
#[allow(dead_code)]
static A: u8 = 10;
static B: u8 = 11;
static NUMBER_LISTS: [&u8; 2] = [&A, &B];
fn main() {
    test_eq!(*black_box(NUMBER_LISTS[black_box(0)]), 10);
    test_eq!(*black_box(NUMBER_LISTS[black_box(1)]), 11);
}
