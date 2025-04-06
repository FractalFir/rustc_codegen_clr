#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params,
    pointer_is_aligned_to
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
#[inline(never)]
#[no_mangle]
fn assert_ne(a:f64,b:f64){
    test!(!((a,) <= (b,)));
}
fn main(){
    assert_ne(1.0, f64::NAN);
    //test!(!((1.0f64, 2.0f64) <= (f64::NAN, 3.0)));
    
}