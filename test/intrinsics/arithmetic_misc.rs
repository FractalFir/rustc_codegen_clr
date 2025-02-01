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

use core::intrinsics::copysignf32;
use core::intrinsics::copysignf64;
use core::intrinsics::fmaf32;
use core::intrinsics::fmaf64;
use core::intrinsics::maxnumf32;
use core::intrinsics::maxnumf64;
use core::intrinsics::minnumf32;
use core::intrinsics::minnumf64;

fn main() {
    let x = 1.0_f32;
    let y = 2.0_f32;
    test_eq!(maxnumf32(x, y), black_box(y));
    let x = 1.0_f64;
    let y = 2.0_f64;
    test_eq!(maxnumf64(x, y), black_box(y));
    let x = 1.0_f32;
    let y = 2.0_f32;
    test_eq!(minnumf32(x, y), black_box(x));
    let x = 1.0_f64;
    let y = 2.0_f64;
    test_eq!(minnumf64(x, y), black_box(x));

    let m = 10.0_f32;
    let x = 4.0_f32;
    let b = 60.0_f32;
    let result = unsafe { fmaf32(m, x, b) };
    test_eq!(result, black_box(100.0));
    test_eq!(m * x + b, black_box(100.0));
    let one_plus_eps = 1.0_f32 + f32::EPSILON;
    let one_minus_eps = 1.0_f32 - f32::EPSILON;
    let minus_one = -1.0_f32;
    let result = unsafe { fmaf32(one_plus_eps, one_minus_eps, minus_one) };
    // The exact result (1 + eps) * (1 - eps) = 1 - eps * eps.
    test_eq!(result, black_box(-f32::EPSILON * f32::EPSILON));
    // Different rounding with the non-fused multiply and add.
    test_eq!(one_plus_eps * one_minus_eps + minus_one, black_box(0.0));
    let m = 10.0_f64;
    let x = 4.0_f64;
    let b = 60.0_f64;
    let result = unsafe { fmaf64(m, x, b) };
    test_eq!(result, black_box(100.0));
    test_eq!(m * x + b, black_box(100.0));
    let one_plus_eps = 1.0_f64 + f64::EPSILON;
    let one_minus_eps = 1.0_f64 - f64::EPSILON;
    let minus_one = -1.0_f64;
    let result = unsafe { fmaf64(one_plus_eps, one_minus_eps, minus_one) };
    // The exact result (1 + eps) * (1 - eps) = 1 - eps * eps.
    test_eq!(result, black_box(-f64::EPSILON * f64::EPSILON));
    // Different rounding with the non-fused multiply and add.
    test_eq!(one_plus_eps * one_minus_eps + minus_one, black_box(0.0));

    let f = 3.5_f32;
    test_eq!(unsafe { copysignf32(f, 0.42) }, black_box(3.5_f32));
    test_eq!(unsafe { copysignf32(f, -0.42) }, black_box(-3.5_f32));
    test_eq!(unsafe { copysignf32(-f, 0.42) }, black_box(3.5_f32));
    test_eq!(unsafe { copysignf32(-f, -0.42) }, black_box(-3.5_f32));
    test!(unsafe { copysignf32(f32::NAN, 1.0) }.is_nan());
    let f = 3.5_f64;
    test_eq!(unsafe { copysignf64(f, 0.42) }, black_box(3.5_f64));
    test_eq!(unsafe { copysignf64(f, -0.42) }, black_box(-3.5_f64));
    test_eq!(unsafe { copysignf64(-f, 0.42) }, black_box(3.5_f64));
    test_eq!(unsafe { copysignf64(-f, -0.42) }, black_box(-3.5_f64));
    test!(unsafe { copysignf64(f64::NAN, 1.0) }.is_nan());
    test_eq!(unsafe { copysignf64(-f, -0.0) }, black_box(-3.5_f64));
}
