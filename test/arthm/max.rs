
#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    test_eq!(black_box(core::cmp::max(black_box(67_usize),black_box(171_usize))),171_usize);
}

