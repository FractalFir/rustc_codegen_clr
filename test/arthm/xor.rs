#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]

include!("../common.rs");
fn main(){
    let a = -5737_i16;
    let b = a ^ a;
    test_eq!(black_box(b),0);
    // Test addition of different types
}