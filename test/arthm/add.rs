#![feature(lang_items)]
#![allow(internal_features)]
#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]
include!("../common.rs");
fn main(){
    // Test addition of different types
    test_eq!(black_box(2_i8) + 2, 4);

    test_eq!(black_box(2_u8) + 2, 4);
    test_eq!(black_box(2_i16) + 2, 4);
    test_eq!(black_box(2_u16) + 2, 4);
    test_eq!(black_box(2_i32)+ 2, 4);
    test_eq!(black_box(2_u32) + 2, 4);
    test_eq!(black_box(2_i64)+ 2, 4);
    test_eq!(black_box(2_u64) + 2, 4);
    test_eq!(black_box(2.0_f32) + 2.0, 4.0);
    test_eq!(black_box(2.0_f64) + 2.0, 4.0);
    // Test addition of negative values
    test_eq!(black_box(2_i8) + black_box(-2), 0);
    test_eq!(black_box(2_i16) + black_box(-2), 0);
    test_eq!(black_box(2_i32) + black_box(-2), 0);
    test_eq!(black_box(2_i64) + black_box(-2), 0);
    test_eq!(black_box(2_f32) + black_box(-2.0), 0.0);
    test_eq!(black_box(2_f64) + black_box(-2.0), 0.0);
    // Test overflows 
    test_eq!(black_box(0xFF_u8).wrapping_add(1), 0);
    test_eq!(black_box(127_i8).wrapping_add(1), -128);
}
