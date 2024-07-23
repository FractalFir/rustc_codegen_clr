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
extern crate core;

use core::ptr::addr_of_mut;

fn main() {
    let mut u: u32 = black_box(20);
    test_eq!(
        unsafe { core::intrinsics::atomic_xsub_release(addr_of_mut!(u), 10) },
        20
    );
    let mut u: u32 = black_box(20);
    let (val, is_eq) =
        unsafe { core::intrinsics::atomic_cxchgweak_acquire_relaxed(addr_of_mut!(u), 20_u32, 10) };
    test_eq!(val, 20_u32);
    test_eq!(u, 10_u32);
    //test_eq!(is_eq,true);
    let (val, is_eq) =
        unsafe { core::intrinsics::atomic_cxchgweak_acquire_relaxed(addr_of_mut!(u), 10_u32, 20) };
    test_eq!(val, 10_u32);
    //test_eq!(is_eq,true);
}
