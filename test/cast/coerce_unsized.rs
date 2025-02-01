#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params,
    unsize,
    coerce_unsized
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
struct Coercable<'a, T: ?Sized> {
    rf: &'a T,
    next: usize,
}
fn main() {
    let arr: [i32; 8] = [0, 1, 2, 4, 5, 6, 7, 8];
    let coercable: Coercable<'_, [i32; 8]> = black_box(Coercable {
        rf: &arr,
        next: usize::MAX,
    });
    let coerced = coercable as Coercable<'_, [i32]>;
    test_eq!(coerced.next, usize::MAX);
}
impl<'a, 'b, A: ?Sized, B: core::marker::Unsize<A> + ?Sized>
    core::ops::CoerceUnsized<Coercable<'a, A>> for Coercable<'a, B>
{
}
