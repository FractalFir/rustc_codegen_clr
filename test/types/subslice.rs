#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
pub fn main() {
    let a = [0; 8];
    let b = black_box(&a[..]);
    let c = &b[1..4];
    if c.len() != 3 {
        core::intrinsics::abort();
    }
    let c = &b[..4];
    if c.len() != 4 {
        core::intrinsics::abort();
    }
    let c = &b[4..];
    if c.len() != 4 {
        core::intrinsics::abort();
    }
    let c = &b[2..b.len() - 3];
}
