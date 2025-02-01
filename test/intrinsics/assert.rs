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
    assert_eq!(8, black_box(8));
    assert_ne!(7, black_box(8));
    assert_eq!(8.0, black_box(8.0));
    assert_ne!(7.999, black_box(8.0));
}
