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
    use core::any::Any;
    fn modify_if_u32(s: &mut (dyn Any + Send)) {
        if let Some(num) = s.downcast_mut::<u32>() {
            *num = 42;
        }
    }
    let mut x = black_box(10u32);
    let mut s = black_box("starlord");
    modify_if_u32(&mut x);
    modify_if_u32(&mut s);
    test_eq!(x, 42);
    test_eq!(s, "starlord");
}
