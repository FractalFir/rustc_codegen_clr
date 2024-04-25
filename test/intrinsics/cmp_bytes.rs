#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    let a = b"Hello, Bob!\n\0";
    let b = "Hello, Bob!\n\0";
    test_eq!(a,black_box(b).as_bytes());
    let a:&[u8] = &b"Hello, Bob!\n\0"[..];
    let b:&[u8] = &b"Hello, Bob!\n\0"[..];
    test_eq!(a,black_box(b));
    test_ne!(a,black_box(&b"Hello, Bill!\n\0"[..]));
}
    