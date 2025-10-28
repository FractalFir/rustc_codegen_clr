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
    test_eq!(is_val_statically_known(42), black_box(true));
    test_eq!(is_val_statically_known([42]), black_box(false));
    test_eq!(is_val_statically_known("42"), black_box(false));
    test_eq!(is_val_statically_known(FortyTwo {}), black_box(false));
    test_eq!(
        is_val_statically_known(FortyTwo {}.forty_two()),
        black_box(false)
    );
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
