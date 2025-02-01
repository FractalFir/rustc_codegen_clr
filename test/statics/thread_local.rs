#![feature(
    thread_local,
    lang_items,
    adt_const_params,
    core_intrinsics,
    unsized_const_params
)]
#![allow(unused_variables, internal_features, incomplete_features, dead_code)]
include!("../common.rs");
#[thread_local]
pub static FOO: [&str; 1] = ["Hello"];
unsafe extern "C"{
    fn sigemptyset(_:&mut [u64;20])->i32;
    fn sigaction(signal:i32,new:&mut [u64;20],old:usize)->i32;
}
fn main() {
    test_eq!(b"Hello", black_box(FOO[black_box(0)].as_bytes()));
    let mut set  = [0;20]; 
    test_eq!(unsafe{sigemptyset(&mut set)},0);
    test_eq!(unsafe{sigaction(1,&mut set,0)},0);
}
