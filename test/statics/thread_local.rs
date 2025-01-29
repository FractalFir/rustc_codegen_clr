#![feature(
    thread_local,
    lang_items,
    adt_const_params,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![no_std]
#![allow(unused_variables, internal_features, incomplete_features, dead_code)]
include!("../common.rs");
#[thread_local]
pub static FOO: [&str; 1] = ["Hello"];
unsafe extern "C"{
    fn sigemptyset(_:&mut [u64;20])->i32;
    fn sigaction(signal:i32,new:&mut [u64;20],old:usize)->i32;
}
fn main() {
    test_eq!("Hello", black_box(FOO[black_box(0)]));
    let mut set  = [0;20]; 
    test_eq!(unsafe{sigemptyset(&mut set)},0);
    test_eq!(unsafe{sigaction(1,&mut set,0)},0);
}
