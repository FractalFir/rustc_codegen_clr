#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
struct MockArgs<'a> {
    pieces: &'a [&'static str],
}
impl<'a> MockArgs<'a> {
    pub fn estimated_capacity(&self) -> usize {
        let pieces_length: usize = self.pieces.iter().map(|x| x.len()).sum();
        return pieces_length.checked_mul(2).unwrap_or(0);
    }
}
fn main() {
    let a = [1, 2, 3];
    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, x| acc + x);

    test_eq!(sum, 6);
    let pieces = black_box(["Hello ", "this ", "is ", "an iterator!"]);

    let args = MockArgs { pieces: &pieces };
    let cap = black_box(args).estimated_capacity();
    test_eq!(cap, 52);
}
