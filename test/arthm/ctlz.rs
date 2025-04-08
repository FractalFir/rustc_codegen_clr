#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#[allow(dead_code)]
include!("../common.rs");
fn main() {
    const A: u128 = 0b0101_1111_u128;
    test_eq!(A.trailing_ones(), 5);
    unsafe{printf(c"%u\n".as_ptr(),(!A).leading_ones() as u32)};
    test_eq!((!A).leading_ones(), u128::BITS - 7);

    test_eq!(A.reverse_bits().leading_ones(), 5);

    test_eq!((!0_u128).leading_ones(), u128::BITS);
    test_eq!((!0_u128).trailing_ones(), u128::BITS);

    test_eq!(((!0_u128) << 1).trailing_ones(), 0);
    test_eq!(((!0_u128) >> 1).leading_ones(), 0);

    test_eq!(((!0_u128) << 1).leading_ones(), u128::BITS - 1);
    test_eq!(((!0_u128) >> 1).trailing_ones(), u128::BITS - 1);

    test_eq!(0_u128.leading_ones(), 0);
    test_eq!(0_u128.trailing_ones(), 0);

    const X: u128 = 0b0010_1100_u128;
    test_eq!(X.leading_ones(), 0);
    test_eq!(X.trailing_ones(), 0);
}