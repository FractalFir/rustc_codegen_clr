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
#[allow(dead_code)]
use core::cmp::Ordering;
include!("../common.rs");
fn main() {
    // Test addition of different types
    /*test_eq!(black_box(2_i8) < 3,true);
    test_eq!(black_box(2_i8) <= 2,true);
    test_eq!(black_box(2_i8) <= 3,true);
    test_eq!(black_box(3_i8) <= 2,false);*/
    test_eq!(min(black_box(6_usize), 0_usize), 0_usize);
}
pub fn min<T: Ord>(a: T, b: T) -> T {
    // Broken
    min_by(a, b, T::cmp)
    // OK
    //min_by(a,b,|a,b|a.cmp(b))
}
pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
    unsafe { printf("Preparing to call the closure!\n\0".as_ptr() as *const core::ffi::c_char) };
    let res = compare(&v1, &v2);
    unsafe { printf("Called the closure!\n\0".as_ptr() as *const core::ffi::c_char) };
    match res {
        Ordering::Less | Ordering::Equal => v1,
        Ordering::Greater => v2,
    }
}
