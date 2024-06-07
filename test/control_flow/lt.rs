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
    unused_imports
)]
#![no_std]
include!("../common.rs");

fn main() {
    test_eq!(branch(4), true);
    test_eq!(branch2(1, 0), true);
    test_eq!(branch3(4), true);
}

fn branch(a: i32) -> bool {
    if black_box(a) < 3 {
        return false;
    }
    if black_box(a) > 6 {
        return false;
    }
    if black_box(a) == 3 || black_box(a) >= 5 {
        return false;
    }
    if black_box(a) != 4 {
        return false;
    }
    return true;
}
fn branch2(a: i32, b: i32) -> bool {
    if black_box(a) < black_box(b) {
        return false;
    }
    return true;
}
fn branch3(a: u32) -> bool {
    if black_box(a) < 3 {
        return false;
    }
    if black_box(a) > 6 {
        return false;
    }
    if black_box(a) == 3 || black_box(a) >= 5 {
        return false;
    }
    if black_box(a) != 4 {
        return false;
    }
    return true;
}
