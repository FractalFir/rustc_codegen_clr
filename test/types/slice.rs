#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    let ptr = unsafe{malloc(64) as *mut _};
    black_box(ptr);
    let slice:&mut [u8] = unsafe{core::slice::from_raw_parts_mut(ptr,64)};
    let len = slice.len();
    let first = slice[black_box(0)];
    Put::putnl(len as u64);
    Put::putnl(first);
    slice[black_box(0)] = 'H' as u8;
    slice[black_box(1)] = 'e' as u8;
    slice[black_box(2)] = 'l' as u8;
    slice[black_box(3)] = 'l' as u8;
    slice[black_box(4)] = 'o' as u8;
    slice[black_box(5)] = '.' as u8;
    slice[black_box(6)] = 0;
    unsafe{puts(ptr)};
    black_box(slice);
}
