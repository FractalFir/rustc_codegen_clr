#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params,
    ptr_mask,
    strict_provenance
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
include!("../common.rs");
extern crate core;

use core::intrinsics::ptr_mask;

fn main() {
    let v = 17_u32;
    let ptr: *const u32 = &v;

    // `u32` is 4 bytes aligned,
    // which means that lower 2 bits are always 0.
    let tag_mask = 0b11;
    let mask = !tag_mask;

    // We can store something in these lower bits
    let tagged_ptr = ptr.map_addr(|a| a | 0b10);

    // Get the "tag" back
    let tag = tagged_ptr.addr() & tag_mask;
    assert_eq!(tag, black_box(0b10));

    // Note that `tagged_ptr` is unaligned, it's UB to read from it.
    // To get original pointer `mask` can be used:
    let masked_ptr = ptr_mask(tagged_ptr, mask);
    assert_eq!(unsafe { *masked_ptr }, black_box(17));
}
