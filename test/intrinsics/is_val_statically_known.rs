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

use core::intrinsics::is_val_statically_known;

fn main() {
    test_eq!(is_val_statically_known(42), true);
    test_eq!(is_val_statically_known([42]), false);
    test_eq!(is_val_statically_known("42"), false);
    test_eq!(is_val_statically_known(FortyTwo {}), false);
    test_eq!(is_val_statically_known(FortyTwo {}.forty_two()), false);
}

#[derive(Copy, Clone)]
struct FortyTwo {}

trait Return {
    fn forty_two(&self) -> usize;
}

impl Return for FortyTwo {
    fn forty_two(&self) -> usize {
        42
    }
}
