#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
use core::ptr::addr_of_mut;
use core::ptr::addr_of;

fn main(){
    let a = black_box(0xDEAD_C0FE_u32);
    Put::putnl(a);
    let mut b = black_box(0xBEEF_BABE_u32);
    test_eq!(black_box(core::mem::size_of::<u32>()),4);
    unsafe{core::ptr::copy_nonoverlapping(black_box(addr_of!(a)),black_box(addr_of_mut!(b)),1)};
    test_eq!(b,0xDEAD_C0FE_u32);
    Put::putnl(a);
    test_eq!(a,0xDEAD_C0FE_u32);
}
    