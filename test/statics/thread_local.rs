#![feature(thread_local,lang_items,adt_const_params,core_intrinsics,start)]
#![no_std]
#![allow(unused_variables,internal_features,incomplete_features,dead_code)]
include!("../common.rs");
#[thread_local]
pub static FOO: [&str; 1] = [ "Hello" ];
fn main() {
    test_eq!("Hello" , black_box(FOO[black_box(0)]));
}
