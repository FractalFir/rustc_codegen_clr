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
    unused_imports
)]
#![no_std]
include!("../common.rs");
static mut INT32: i32 = 0;
fn main() {
    let zero = unsafe { INT32 };
    test_eq!(zero, 0);
    unsafe { INT32 += 1 };
    let one = unsafe { INT32 };
    test_eq!(one, 1);
    unsafe { INT32 += 1 };
    let two = unsafe { INT32 };
    test_eq!(two, 2);
    unsafe { INT32 *= two };
    let four = unsafe { INT32 };
    test_eq!(four, 4);
}
