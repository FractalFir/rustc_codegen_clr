#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code,improper_ctypes_definitions,improper_ctypes)]
#![no_std]
#![feature(unboxed_closures)]
include!("../common.rs");

fn main(){
    //let val = 0xDEAD_BEFF_DEAD_C0FFE_BEFF_BABE_u128;
    //black_box(val);
    let val = black_box(0xDEAD_BEFF_DEAD_C0FFE_BEFF_BABE_i128);
    //black_box(val);
    let x = match val {
        8=>black_box(8),
        7=>black_box(7),
        _=>black_box(11),
    };
    black_box(x);
}
