#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code,unused_unsafe,unused_imports)]
#![no_std]
include!("../common.rs");
fn main(){
    unsafe{
        let mut buff = malloc(64);
        test_ne!(buff,0_usize as *mut _);
        buff = realloc(buff,128);
        test_ne!(buff,0_usize as *mut _);
        free(buff);
    }
}
