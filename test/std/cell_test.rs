#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    never_type
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code,unused_imports,unused_mut,private_interfaces,non_upper_case_globals)]
#![no_std]
#[allow(dead_code)]
use core::cell::Cell;
include!("../common.rs");   
fn cell_test(){
    let cell = Cell::new(black_box(64));
    test_eq!(cell.get(),64);
    cell.set(black_box(33));
    test_eq!(cell.get(),33);
}
fn main() {
    cell_test();
}
