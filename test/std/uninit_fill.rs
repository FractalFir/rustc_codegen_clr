#![feature(
    maybe_uninit_fill,
    unsized_const_params,
    core_intrinsics,
    lang_items,
    start
)]
#![allow(
    unused_variables,
    incomplete_features,
    unused_imports,
    dead_code,
    internal_features
)]
#![no_std]
include!("../common.rs");
extern "C" {
    fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
}
use core::mem::MaybeUninit;
fn main() {
    let mut dst = [MaybeUninit::new(255); 64];
    let expect = [0; 64];

    assert_eq!(MaybeUninit::fill(&mut dst, 0), &expect);
}
