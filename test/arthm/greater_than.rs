#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]

include!("../common.rs");
fn main(){
    let a = 27340181294056582_usize;
    let b = 18419403892415495033_usize;
    test_eq!(black_box(a) > b,false);
    // Test addition of different types
}