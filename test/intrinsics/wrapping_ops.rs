#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
extern crate core;

use core::intrinsics::wrapping_add;
use core::intrinsics::wrapping_sub;
use core::intrinsics::wrapping_mul;

fn main() {
    test_eq!(wrapping_add(200u32, 55), black_box(255));
    test_eq!(wrapping_add(200u32, u32::MAX), black_box(199));
    test_eq!(wrapping_sub(100u32, 100), black_box(0));
    test_eq!(wrapping_sub(100u32, u32::MAX), black_box(101));
    test_eq!(wrapping_mul(10u8, 12), black_box(120));
    test_eq!(wrapping_mul(25u8, 12), black_box(44));
}
