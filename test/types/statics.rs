#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
include!("../common.rs");

static mut INT32: i32 = -1;
static mut BYTE: u8 = 0;
static mut SHORT: u16 = 0;
static ARR: [u8; 3] = [0; 3];
fn main() {
    //unsafe { printf(c"INT32:%x\n".as_ptr(), &INT32 as *const _ as usize) };
    //unsafe { printf(c"BYTE:%x\n".as_ptr(), &BYTE as *const _ as usize) };
    //unsafe { printf(c"SHORT:%x\n".as_ptr(), &SHORT as *const _ as usize) };
    unsafe { INT32 += 1 };
    let zero = unsafe { INT32 };
    test_eq!(zero, 0);
    unsafe { INT32 += 1 };
    let one = unsafe { INT32 };
    test_eq!(one, 1);

    unsafe { INT32 += 1 };
    let two = unsafe { INT32 };
    test_eq!(two, 2);
    unsafe { INT32 *= two };
    let four = unsafe { INT32 };
    test_eq!(four, 4);
    #[allow(static_mut_refs)]
    unsafe {
        *black_box(&mut BYTE) = 64
    };
    unsafe { test_eq!(BYTE, 64) };
    #[allow(static_mut_refs)]
    unsafe {
        *black_box(&mut SHORT) = 128
    };
    unsafe { test_eq!(SHORT, 128) };
    test_eq!(ARR[0], 0);
}
