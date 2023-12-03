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
    let mut string = String::with_capacity(100);
    string.push('H');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');
    string.push('!');
    string.push('\0');
    std::hint::black_box(&string);
    unsafe{puts(string.as_ptr())}
}
