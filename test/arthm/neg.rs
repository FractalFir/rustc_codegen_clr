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
    test_eq!((black_box(0b1111111_i8) + 2) as i32, 4 as i32);

    test_eq!(black_box(2_u8) + 2, 4);
    test_eq!(black_box(2_i16) + 2, 4);
    test_eq!(black_box(2_u16) + 2, 4);
    test_eq!(black_box(2_i32) + 2, 4);
    test_eq!(black_box(2_u32) + 2, 4);
}
