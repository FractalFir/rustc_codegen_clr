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

use core::intrinsics::cosf32;
use core::intrinsics::cosf64;
use core::intrinsics::sinf32;
use core::intrinsics::sinf64;

fn main() {
    let x = 2.0 * core::f32::consts::PI;
    let abs_difference = (cosf32(x) - 1.0).abs();
    test!(abs_difference <= black_box(f32::EPSILON));
    let x = 2.0 * core::f64::consts::PI;
    let abs_difference = (cosf64(x) - 1.0).abs();
    test!(abs_difference <= black_box(f64::EPSILON));
    let x = 2.0 * core::f32::consts::FRAC_PI_2;
    let abs_difference = (sinf32(x) - 1.0).abs();
    test!(abs_difference <= black_box(f32::EPSILON));
    let x = 2.0 * core::f64::consts::FRAC_PI_2;
    let abs_difference = (sinf64(x) - 1.0).abs();
    test!(abs_difference <= black_box(f64::EPSILON));
}
