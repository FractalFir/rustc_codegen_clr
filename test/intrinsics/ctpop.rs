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
fn main() {
    let n = 0b01001100u32;
    test_eq!(n.count_ones(), 3);
    let n = 0b01001100usize;
    test_eq!(n.count_ones(), 3);
}
