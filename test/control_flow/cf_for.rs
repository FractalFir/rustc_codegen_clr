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
fn main() {
    let mut i = black_box(0);
    for _ in 0..1234 {
        i += 1;
        rustc_clr_interop_managed_call1_::<
            "System.Console",
            "System.Console",
            false,
            "WriteLine",
            true,
            (),
            i32,
        >(i);
    }
    test_eq!(black_box(i), 1234);
}
