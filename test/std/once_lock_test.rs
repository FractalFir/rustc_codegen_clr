#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    never_type,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
#[allow(dead_code)]
mod futex;
mod once_lock;
include!("../common.rs");
use once_lock::*;
fn main() {
    static COMPUTATION: OnceLock<u8> = OnceLock::new();
    if let Some(_) = COMPUTATION.get() {
        core::intrinsics::abort();
    }
    if *COMPUTATION
        .get_or_try_init(|| Ok::<u8, ()>(black_box(77)))
        .unwrap()
        != 77
    {
        core::intrinsics::abort();
    }
    if let Some(val) = COMPUTATION.get() {
        unsafe { printf("val is:%d\n\0".as_ptr() as *const i8, *val as u32) };
    }
}
