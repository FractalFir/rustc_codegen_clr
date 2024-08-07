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
extern crate core;

use core::intrinsics::sqrtf32;
use core::intrinsics::sqrtf64;
use core::intrinsics::powf32;
use core::intrinsics::powf64;
use core::intrinsics::powif32;
use core::intrinsics::powif64;
use core::intrinsics::exp2f32;
use core::intrinsics::exp2f64;

fn main() {
    let positive = 4.0_f32;
    let negative = -4.0_f32;
    let negative_zero = -0.0_f32;

    test_eq!(sqrtf32(positive), black_box(2.0));
    test!(sqrtf32(negative).is_nan());
    test_eq!(sqrtf32(negative_zero), black_box(negative_zero));

    let positive = 4.0_f64;
    let negative = -4.0_f64;
    let negative_zero = -0.0_f64;

    test_eq!(sqrtf64(positive), black_box(2.0));
    test!(sqrtf64(negative).is_nan());
    test_eq!(sqrtf64(negative_zero), black_box(negative_zero));

    let x = 2.0_f32;
    let abs_difference = (powf32(2.0) - (x * x)).abs();
    test!(abs_difference <= black_box(f32::EPSILON));
    let x = 2.0_f64;
    let abs_difference = (powf64(2.0) - (x * x)).abs();
    test!(abs_difference <= black_box(f64::EPSILON));
    let x = 2.0_f32;
    let abs_difference = (powif32(2) - (x * x)).abs();
    test!(abs_difference <= black_box(f32::EPSILON));
    let x = 2.0_f64;
    let abs_difference = (powif64(2) - (x * x)).abs();
    test!(abs_difference <= black_box(f64::EPSILON));

    let f = 2.0f32;
    // 2^2 - 4 == 0
    let abs_difference = (exp2f32(f) - 4.0).abs();
    test!(abs_difference <= black_box(f32::EPSILON));
    let f = 2.0f64;
    // 2^2 - 4 == 0
    let abs_difference = (exp2f64(f) - 4.0).abs();
    test!(abs_difference <= black_box(f64::EPSILON));
}
