#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    invalid_value
)]
#![no_std]
include!("../common.rs");
use core::mem::MaybeUninit;
#[derive(Copy, Clone)]
pub struct Adt47 {
    fld1: i128,
    fld2: u64,
    fld3: i8,
    ptr: (*mut i8,),
    fld5: ((isize,),[bool; 5]),
}
#[inline(never)]
pub unsafe fn fn7(_2: i128, out: *mut i8) -> usize {
    let mut adt: Adt47 = unsafe { MaybeUninit::uninit().assume_init() };
    // Precondition
    test_eq!(core::ptr::addr_of!(adt) as usize + 16,core::ptr::addr_of!(adt.fld1) as usize);
    test_eq!(core::ptr::addr_of!(adt) as usize + 40,core::ptr::addr_of!(adt.ptr) as usize);
    // Failing test
    adt.ptr.0 = out;
    test_eq!(adt.ptr.0,out);
    adt.fld1 = 0x0123456789ABCDEF_i128;
    black_box(adt);
    test_eq!(adt.ptr.0,out);
    return 0;
}
fn main() {
    let mut var = 0;
    let res = unsafe { fn7(1, core::ptr::addr_of_mut!(var)) };
}
