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

use core::intrinsics::fabsf32;
use core::intrinsics::fabsf64;
use core::intrinsics::nearbyintf32;
use core::intrinsics::nearbyintf64;
use core::intrinsics::rintf32;
use core::intrinsics::rintf64;
use core::intrinsics::roundevenf32;
use core::intrinsics::roundevenf64;
use core::intrinsics::roundf32;
use core::intrinsics::roundf64;
use core::intrinsics::floorf32;
use core::intrinsics::floorf64;
use core::intrinsics::ceilf32;
use core::intrinsics::ceilf64;

fn main() {
    let x = 3.5_f32;
    let y = -3.5_f32;
    test_eq!(fabsf32(x), black_box(x));
    test_eq!(fabsf32(y), black_box(-y));
    test!(fabsf32(f32::NAN.is_nan()));
    let x = 3.5_f64;
    let y = -3.5_f64;
    test_eq!(fabsf64(x), black_box(x));
    test_eq!(fabsf64(y), black_box(-y));
    test!(fabsf64(f64::NAN.is_nan()));
    test_eq!(nearbyintf32(2.5f32), black_box(2.0));
    test_eq!(nearbyintf32(3.5f32), black_box(4.0));
    test_eq!(nearbyintf64(2.5f64), black_box(2.0));
    test_eq!(nearbyintf64(3.5f64), black_box(4.0));
    let f = 3.3_f32;
    let g = -3.3_f32;
    let h = 3.5_f32;
    let i = 4.5_f32;
    test_eq!(rintf32(f), black_box(3.0));
    test_eq!(rintf32(g), black_box(-3.0));
    test_eq!(rintf32(h), black_box(4.0));
    test_eq!(rintf32(i), black_box(4.0));
    let f = 3.3_f64;
    let g = -3.3_f64;
    let h = 3.5_f64;
    let i = 4.5_f64;
    test_eq!(rintf64(f), black_box(3.0));
    test_eq!(rintf64(g), black_box(-3.0));
    test_eq!(rintf64(h), black_box(4.0));
    test_eq!(rintf64(i), black_box(4.0));
    let f = 3.3_f32;
    let g = -3.3_f32;
    let h = 3.5_f32;
    let i = 4.5_f32;
    test_eq!(roundevenf32(f), black_box(3.0));
    test_eq!(roundevenf32(g), black_box(-3.0));
    test_eq!(roundevenf32(h), black_box(4.0));
    test_eq!(roundevenf32(i), black_box(4.0));
    let f = 3.3_f64;
    let g = -3.3_f64;
    let h = 3.5_f64;
    let i = 4.5_f64;
    test_eq!(roundevenf64(f), black_box(3.0));
    test_eq!(roundevenf64(g), black_box(-3.0));
    test_eq!(roundevenf64(h), black_box(4.0));
    test_eq!(roundevenf64(i), black_box(4.0));
    let f = 3.3_f32;
    let g = -3.3_f32;
    let h = -3.7_f32;
    let i = 3.5_f32;
    let j = 4.5_f32;
    test_eq!(roundf32(f), black_box(3.0));
    test_eq!(roundf32(g), black_box(-3.0));
    test_eq!(roundf32(h), black_box(-4.0));
    test_eq!(roundf32(i), black_box(4.0));
    test_eq!(roundf32(j), black_box(5.0));
    let f = 3.3_f64;
    let g = -3.3_f64;
    let h = -3.7_f64;
    let i = 3.5_f64;
    let j = 4.5_f64;
    test_eq!(roundf64(f), black_box(3.0));
    test_eq!(roundf64(g), black_box(-3.0));
    test_eq!(roundf64(h), black_box(-4.0));
    test_eq!(roundf64(i), black_box(4.0));
    test_eq!(roundf64(j), black_box(5.0));
    let f = 3.01_f32;
    let g = 4.0_f32;
    test_eq!(ceilf32(f), black_box(4.0));
    test_eq!(ceilf32(g), black_box(4.0));
    let f = 3.01_f64;
    let g = 4.0_f64;
    test_eq!(ceilf64(f), black_box(4.0));
    test_eq!(ceilf64(g), black_box(4.0));
    let f = 3.7_f32;
    let g = 3.0_f32;
    let h = -3.7_f32;
    test_eq!(floorf32(f), black_box(3.0));
    test_eq!(floorf32(g), black_box(3.0));
    test_eq!(floorf32(h), black_box(-4.0));
    let f = 3.7_f64;
    let g = 3.0_f64;
    let h = -3.7_f64;
    test_eq!(floorf64(f), black_box(3.0));
    test_eq!(floorf64(g), black_box(3.0));
    test_eq!(floorf64(h), black_box(-4.0));
}
