#![allow(internal_features,unused_imports,incomplete_features,unused_variables,dead_code,improper_ctypes_definitions)]
#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
//#![no_std]
use std::time::Instant;
//use mycorrhiza::{start,panic_handler};
//panic_handler!{}
//start!{}
//#[lang = "eh_personality"]
//fn rust_eh_personality() {}
extern "C"{
    fn puts(msg:*const u8);
}
fn main() {
    let mut vec:Vec<u8> = Vec::new();
    vec.push(0x48);
    vec.push(0x65);
    vec.push(0x6C);
    vec.push(0x6C);
    vec.push(0x6F);
    vec.push(0x21);
    vec.push(0x00);
    std::hint::black_box(&vec);
    unsafe{puts(vec.as_ptr())}
}
