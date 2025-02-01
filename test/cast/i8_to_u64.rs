#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn main() {
    let a: i8 = black_box(-127);
    let b = a as u64;
    test_eq!(b, 18446744073709551489_u64);
}
