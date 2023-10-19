#![feature(lang_items)]
#![allow(internal_features)]
#![no_std]
#![feature(start)]
#![feature(associated_type_defaults)]
#![feature(core_intrinsics)]
include!("../common.rs");
fn main(){
    let mut i = black_box(0);
    for _ in 0..1234{
        i+=1;
    }
    test_eq!(black_box(i),1234);
}
