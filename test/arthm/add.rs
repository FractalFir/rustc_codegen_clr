#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
#[allow(dead_code)]
struct Test<T>{
    data:T,
}
include!("../common.rs");
fn main(){
    // Test addition of different types
    test_eq!((black_box(2_i8) + 2) as i32, 4 as i32);

    test_eq!(black_box(2_u8) + 2, 4);
    test_eq!(black_box(2_i16) + 2, 4);
    test_eq!(black_box(2_u16) + 2, 4);
    test_eq!(black_box(2_i32)+ 2, 4);
    test_eq!(black_box(2_u32) + 2, 4);
    
    test_eq!(black_box(2_u64) + 2, 4);
    // Signed 64 bit checked add DOES NOT WORK in mono.
    test_eq!(black_box(2_i64)+ 2, 4);
    
    test_eq!(black_box(2_u128)+ 2, 4);
    #[cfg(debug_assertions)]
    {
        test_eq!(black_box(2_i128)+ 2, 4);
    }
    //let slice:&mut [u8] = black_box(unsafe{core::slice::from_raw_parts_mut(black_box(core::ptr::null_mut()),64)});

    test_eq!(black_box(2.0_f32) + 2.0, 4.0);
    test_eq!(black_box(2.0_f64) + 2.0, 4.0);
    // Test addition of negative values
    test_eq!(black_box(2_i8) + black_box(-2), 0);
    test_eq!(black_box(2_i16) + black_box(-2), 0);
    test_eq!(black_box(2_i32) + black_box(-2), 0);
    //test_eq!(black_box(2_i64) + black_box(-2), 0);
    test_eq!(black_box(2_f32) + black_box(-2.0), 0.0);
    test_eq!(black_box(2_f64) + black_box(-2.0), 0.0);
    // Test overflows 
    test_eq!(black_box(255_u8).wrapping_add(1_u8), 0_u8);
    test_eq!(black_box(127_i8).wrapping_add(1_i8),-128_i8);
    test_eq!(black_box(65_535u16).wrapping_add(1_u16), 0_u16);
    test_eq!(black_box(32_767i16).wrapping_add(1_i16),-32_768i16);
    //test_eq!(black_box(4_294_967_295u32).wrapping_add(1_u32), 0_u32);
    //test_eq!(black_box(2_147_483_647i32).wrapping_add(1_i32),-2_147_483_648i32);
    //test_eq!(black_box(18_446_744_073_709_551_615u64).wrapping_add(1_u64), 0_u64);
    //test_eq!(black_box(9_223_372_036_854_775_807i64).wrapping_add(1_i64),-9_223_372_036_854_775_808i64);
    //let ptr:*mut Test<i32> = core::ptr::null_mut();
    //black_box(ptr);
}
