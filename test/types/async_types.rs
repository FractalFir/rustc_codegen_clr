#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn main() {
    let x = black_box(async_fn2(8, black_box(9.9)));
}
async fn async_fn(a: i32, b: f32) -> f32 {
    a as f32 + black_box(b)
}
async fn async_fn2(a: i32, b: f32) -> f32 {
   b
}
