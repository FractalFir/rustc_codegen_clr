#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    invalid_value
)]
#![no_std]
include!("../common.rs");

fn main() {
    let v6 = !black_box(15566726440841019736_u64);
    test_eq!(v6, 2880017632868531879_u64);
    let v12 = black_box(v6) as f32;
    test_eq!(v12, 2.8800176e18);
    let v8 = black_box(v12) + black_box(v12);
    test_eq!(v8, 5.760035e18);
    let v17 = black_box(v8) as i8;
    test_eq!(v17, 127);
    let v4 = -black_box(v17);
    test_eq!(v4, -127);
}
