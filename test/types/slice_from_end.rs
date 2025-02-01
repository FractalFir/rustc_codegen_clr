#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    ascii_char,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn main() {
    let oslice = b"a";
    test_eq!(oslice.len(), black_box(1));
    test_eq!(oslice[0], b'a');
    test_eq!(*oslice.last().unwrap(), b'a');
}
