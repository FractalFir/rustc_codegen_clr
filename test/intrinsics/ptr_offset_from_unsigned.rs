#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn test_ptr_offset_from_unsigned() {
    // Define two pointers
    let ptr1: *const u8 = black_box(0x1000 as *const u8);
    let ptr2: *const u8 = 0x1004 as *const u8;

    // Calculate the offset between ptr2 and ptr1
    let offset = unsafe { core::intrinsics::ptr_offset_from_unsigned(ptr2, ptr1) };

    // Expected result: 4 bytes (because ptr2 is 4 bytes ahead of ptr1)
    Put::putnl(1);
    if offset != 4 {
        Put::putnl(offset as u64);
        core::intrinsics::abort();
    }

    // Test with another example
    let ptr3: *const u8 = core::intrinsics::black_box(0x2000 as *const u8);
    let ptr4: *const u8 = 0x1000 as *const u8;

    let offset2 = unsafe { core::intrinsics::ptr_offset_from_unsigned(ptr3, ptr4) };
    Put::putnl(2);
    // Expected result: 4096 bytes (because ptr4 is 4096 bytes ahead of ptr3)
    if offset2 != 4096 {
        Put::putnl(offset2 as u64);
        core::intrinsics::abort();
    }

    // Additional test case: Pointers are equal
    let ptr5: *const String = core::intrinsics::black_box(0x3000 as *const String);

    let offset3 = unsafe {
        core::intrinsics::ptr_offset_from_unsigned(ptr5, core::intrinsics::black_box(ptr5))
    };
    Put::putnl(3);
    // Expected result: 0 (since both pointers are the same)
    if offset3 != 0 {
        Put::putnl(offset3 as u64);
        core::intrinsics::abort();
    }
}
fn main() {
    test_ptr_offset_from_unsigned();
}
