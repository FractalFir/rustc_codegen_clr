#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    ptr_metadata,
    strict_provenance,
    unsized_const_params,
    portable_simd
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
include!("../common.rs");
use core::simd::Simd;
fn main() {
    test_eq!(
        black_box(Simd::from_array([4, 6, 8, 10])),
        Simd::from_array([4, 6, 8, 10])
    );
    let a = Simd::from_array([0, 1, 2, 3]);
    let b = Simd::from_array([4, 5, 6, 7]);
    test_eq!(a + b, Simd::from_array([4, 6, 8, 10]));
    let a = Simd::from_array([4, 5, 6, 7]);
    let b = Simd::from_array([0, 1, 2, 3]);
    test_eq!(a - b, Simd::from_array([4, 4, 4, 4]));
}
