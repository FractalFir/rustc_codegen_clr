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
#[repr(align(4096))]
struct Aligned {
    val: u8,
}
fn main() {
    let mut aligned = Aligned { val: black_box(0) };
    black_box(&mut aligned);
    test_eq!(core::ptr::addr_of_mut!(aligned) as usize % 4096, 0);
}
