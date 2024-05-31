#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    never_type
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code,unused_imports,unused_mut,private_interfaces,non_upper_case_globals,unused_unsafe,improper_ctypes)]
#![no_std]


include!("../common.rs");   
fn thread_start(data:*mut ()){
    unsafe{printf("Hello from a launched thread!\n\0".as_ptr() as *const i8)};
}
extern "C"{
    fn pthread_create(thread:&mut isize,attr:&(),start_routine:fn(*mut ()),data:*mut ())->i32;
}
fn main() {
    let mut thread = 0;
    let attr = ();
    let mut data = 0_i32;
    unsafe{test_eq!(pthread_create(&mut thread,&attr,thread_start,core::ptr::addr_of_mut!(data) as *mut ()),0)};
   
}
