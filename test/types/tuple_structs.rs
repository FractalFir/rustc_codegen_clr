#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
struct TupleStyleStruct(u32, u8);
include!("../common.rs");
fn main() {
    let mut tst = black_box(TupleStyleStruct(black_box(8), 1));
    tst.0 += tst.1 as u32;
    black_box(tst);
}
