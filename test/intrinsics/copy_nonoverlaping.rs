#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
use core::ptr::addr_of;
use core::ptr::addr_of_mut;
#[inline(never)]
fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
    unsafe { core::ptr::copy_nonoverlapping::<T>(src, dst, count) };
}
#[inline(never)]
fn write_bytes<T>(dst: *mut T, val: u8, count: usize) {
    unsafe { core::intrinsics::write_bytes::<T>(dst, val, count) };
}
fn main() {
    let a = black_box(0xDEAD_C0FE_u32);
    Put::putnl(a);
    let mut b = black_box(0xBEEF_BABE_u32);
    test_eq!(black_box(core::mem::size_of::<u32>()), 4);
    copy_nonoverlapping::<u32>(black_box(addr_of!(a)), black_box(addr_of_mut!(b)), 1);
    test_eq!(b, 0xDEAD_C0FE_u32);
    Put::putnl(a);
    test_eq!(a, 0xDEAD_C0FE_u32);
    let mut a = black_box(0xBEEF_BABE_u32);
    write_bytes::<u32>(black_box(addr_of_mut!(a)), 0xAB, 1);
    test_eq!(a, 0xABAB_ABAB_u32);
}
