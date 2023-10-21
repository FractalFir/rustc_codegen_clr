#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    let mut strig = unsafe{str::from_utf8_unchecked(slice::from_raw_parts(malloc(64) as *mut _,64))};
}
