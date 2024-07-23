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
pub fn main() {
    let a = [0; 8];
    let b = black_box(&a[..]);
    let c = black_box(&((*b)[7]));
    let a = [b; 8];
    let b = black_box(&a[..]);
    let c = black_box(&((*b)[7]));
}
