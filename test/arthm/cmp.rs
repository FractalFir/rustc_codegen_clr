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
    test_eq!(black_box(2_i8) < 3,true);
    test_eq!(black_box(2_i8) <= 2,true);
    test_eq!(black_box(2_i8) <= 3,true);
    test_eq!(black_box(3_i8) <= 2,false);

   
    
    
   
}
