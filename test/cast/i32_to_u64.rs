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
    let a: i32 = black_box(-451078156);
    let b = a as u64;
    test_eq!(b, 18446744073258473460_u64);
}
