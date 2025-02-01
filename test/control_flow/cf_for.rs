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
    let mut i = black_box(0);
    for _ in 0..1234 {
        i += 1;
        unsafe { printf(c"i:%i\n".as_ptr(), i) };
    }
    test_eq!(black_box(i), 1234);
}
