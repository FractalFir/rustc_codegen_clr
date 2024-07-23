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
use core::mem::offset_of;
type Tuple = (u64, [i128; 4], u32, f64);
#[derive(Default)]
struct Quad<T: Default> {
    a: T,
    b: T,
    c: T,
    d: T,
}
fn main() {
    test_eq!(black_box(offset_of!(Tuple, 0)), 64);
    test_eq!(offset_of!(Tuple, 1), 0);
    test_eq!(offset_of!(Tuple, 2), 72);
    test_eq!(offset_of!(Tuple, 3), 80);
}
