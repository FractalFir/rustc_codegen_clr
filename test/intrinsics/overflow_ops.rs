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

use core::intrinsics::add_with_overflow;
use core::intrinsics::mul_with_overflow;
use core::intrinsics::sub_with_overflow;

fn main() {
    test_eq!(add_with_overflow(5u32, 2), black_box((7, false)));
    test_eq!(add_with_overflow(u32::MAX, 1), black_box((0, true)));
    test_eq!(sub_with_overflow(5u32, 2), black_box((3, false)));
    test_eq!(sub_with_overflow(0u32, 1), black_box((u32::MAX, true)));
    test_eq!(mul_with_overflow(5u32, 2), black_box((10, false)));
    test_eq!(
        mul_with_overflow(1_000_000_000u32, 10),
        black_box((1410065408, true))
    );
}
