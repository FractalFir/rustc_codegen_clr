#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
#[allow(dead_code)]
struct Test<T>{
    data:T,
}
include!("../common.rs");
fn main(){
    let a:i8 = black_box(-127);
    let b = a as u64;
    test_eq!(b, 18446744073709551489_u64);
}
