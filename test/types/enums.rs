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

#[derive(Clone, Copy)]
enum Animal {
    Cow(u8),
    Dog(u64),
}
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SimpleEnum {
    A,
    B,
    C,
    D,
    E,
    F,
}
#[allow(dead_code)]
fn simple_enum() {
    let simple_enum = SimpleEnum::A;
    let simple_enum = black_box(simple_enum);
    test_eq!(simple_enum, SimpleEnum::A);
    test_ne!(simple_enum, SimpleEnum::B);
    let some_0_usize = black_box(Some(0_usize));
    match some_0_usize {
        Some(some) => {
            black_box(some);
        }
        None => black_box(()),
    }
}
fn main() {
    simple_enum();
    //let maybe:*mut Maybe = core::ptr::null_mut();
    //test_eq!(maybe,core::ptr::null_mut());
    let maybe: *mut Option<i32> = unsafe { malloc(8) }.cast();
    let tag: *mut i32 = maybe.cast();
    unsafe { *tag = 0 };
    if let Some(_) = unsafe { *maybe } {
        core::intrinsics::abort();
    }
    black_box(maybe);
    let maybe = Some(8);
    black_box(maybe);
    let end = black_box(90);
    let range = 65..end;
    black_box(&range);
    let mut iter = range.into_iter();
    black_box(&iter);
    let first = iter.next();
    black_box(&first);
    for i in 65..black_box(90) {
        let msg = (0x00_00_00_00_00_00_0a_i64 << 8) | i;
        unsafe { puts(core::ptr::addr_of!(msg).cast()) };
    }
    let animal = Animal::Cow(black_box(8));
    black_box(animal);
    test_eq!(core::mem::size_of::<Option<u8>>(), 2);
}
