#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
#![no_std]
use core::any::TypeId;
include!("../common.rs");
fn main() {
    test_eq!(TypeId::of::<i32>(), TypeId::of::<i32>());
    test_ne!(TypeId::of::<i32>(), TypeId::of::<u32>());
    test_eq!(TypeId::of::<u32>(), TypeId::of::<u32>());
    test_ne!(TypeId::of::<i128>(), TypeId::of::<f32>());
}
