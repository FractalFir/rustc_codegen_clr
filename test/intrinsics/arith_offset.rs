#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    let mut a = [0,1,2,3,4,5,6,7];
    let aptr = &mut a as *mut i32;
    let aptr = black_box(aptr);
    test_eq!(unsafe{*aptr. wrapping_offset(1)},1);
    test_eq!(unsafe{*aptr. wrapping_offset(5)},5);
    test_eq!(unsafe{*aptr. wrapping_offset(7)},7);
}
