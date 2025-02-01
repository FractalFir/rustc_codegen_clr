#![feature(
    lang_items,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params,
    f16
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn main() {
    // 1st. Check that a const f16 can be created.
    let zero = black_box(0_f16);
    let one = black_box(1_f16);
    let two = black_box(2_f16);
    // Check that addtion works
    test_eq!(black_box(one + one), two);
    // Check that subtraction works
    test_eq!(black_box(one - one), zero);
    // Check that multiplaction works
    test_eq!(black_box(one * one), one);
    // Check that division works
    test_eq!(black_box(two / one), two);
}
