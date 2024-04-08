#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code,unused_unsafe)]
#![no_std]
include!("../common.rs");

#[derive(Default)]
struct Quad<T:Default>{
    a:T,
    b:T,
    c:T,
    d:T
}
fn main(){
    unsafe{
   let alloc = __rust_alloc(4,8);
   *(alloc as *mut u32) = black_box(0xDEAD_C0FE);
   test_eq!(*(alloc as *mut u32),0xDEAD_C0FE);
   let alloc = __rust_realloc(alloc,4,8,8);
   test_eq!(*(alloc as *mut u32),0xDEAD_C0FE);
   *(alloc as *mut u64) = black_box(0xBEFF_C0FE_DEAD_BABE);
   test_eq!(*(alloc as *mut u64),0xBEFF_C0FE_DEAD_BABE);
   __rust_dealloc(alloc,8,8);
    }
}
    