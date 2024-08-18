#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
include!("../common.rs");
use core::mem::MaybeUninit;
fn main() {
    let mut x = MaybeUninit::<u8>::uninit();
    x.write(black_box(89));
    let x = unsafe { x.assume_init() };
    test_eq!(x, 89);
    test_buff();
}
fn test_buff() {
    let buf: &mut [MaybeUninit<u8>] = &mut [MaybeUninit::uninit(); DEFAULT_BUF_SIZE];
    black_box(buf);
}
const DEFAULT_BUF_SIZE: usize = 8_192usize;
