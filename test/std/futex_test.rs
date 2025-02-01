#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#[allow(dead_code)]
mod futex;
include!("../common.rs");
use futex::*;
fn main() {
    let once = Once::new();
    let mut val = 0;
    once.call(false, &mut |_| {
        val += 1;
    });
    if val != 1 {
        core::intrinsics::abort();
    }
}
