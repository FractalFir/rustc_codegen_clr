#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    let ptr = unsafe{malloc(64) as *mut _};
    black_box(ptr);
    let slice:&mut [u8] = unsafe{core::slice::from_raw_parts_mut(ptr,64)};
    let len = slice.len();
    black_box(slice);
}
