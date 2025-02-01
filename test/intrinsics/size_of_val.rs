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
    unused_unsafe
)]
include!("../common.rs");

#[derive(Default)]
struct Quad<T: Default> {
    a: T,
    b: T,
    c: T,
    d: T,
}
trait Marker {}
impl<T: core::default::Default> Marker for Quad<T> {}
fn main() {
    use core::mem;

    test_eq!(4, mem::size_of_val(&5i32));

    let x: [u8; 13] = [0; 13];
    let y: &[u8] = &x;
    test_eq!(13, mem::size_of_val(y));

    let quad: Quad<i32> = Quad {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
    };

    let dynv = black_box(&quad as &dyn Marker);
    test_eq!(mem::size_of_val(dynv), core::mem::size_of::<Quad<i32>>());
    test_eq!(black_box(64_usize).is_power_of_two(), true);
    test_eq!(black_box(8_usize).is_power_of_two(), true);
}
