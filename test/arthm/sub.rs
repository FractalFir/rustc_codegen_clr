#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
#[allow(dead_code)]
struct Test<T>{
    data:T,
}
include!("../common.rs");
fn main(){
    // Test subtraction of different types
    test_eq!(black_box(4_u8) - 2, 2);
    test_eq!(black_box(4_i8) - 2, 2);
    test_eq!(black_box(0_i8) - 2, -2);

    test_eq!(black_box(4_u16) - 2, 2);
    test_eq!(black_box(4_i16) - 2, 2);
    test_eq!(black_box(0_i16) - 2, -2);

    test_eq!(black_box(4_u32) - 2, 2);
    test_eq!(black_box(4_i32) - 2, 2);
    test_eq!(black_box(0_i32) - 2, -2);
    // 64 bit checked binops don't work in mono
    //test_eq!(black_box(4_u64) - 2, 2);
    //test_eq!(black_box(4_i64) - 2, 2);
    //test_eq!(black_box(0_i64) - 2, -2);
}
