#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
extern crate core;
use core::intrinsics::transmute;
fn main() {
    let slice = unsafe { transmute::<&str, &[u8]>(".NET") };
    test_eq!(black_box(slice), &[46, 78, 69, 84]);

    fn forty_two() -> i32 {
        42
    }
    let pointer = forty_two as *const ();
    let function = unsafe {
        transmute::<*const (), fn() -> i32>(pointer)
    };
    test_eq!(function(), black_box(42));

    let ptr_i32: &mut i32 = &mut 3;
    let transmuted_to_u32: &mut u32 = unsafe {
        transmute::<&mut i32, &mut u32>(ptr_i32)
    };
    let ptr_u32: &mut u32 = &mut 3;
    test_eq!(transmuted_to_u32, black_box(ptr_u32));
}
