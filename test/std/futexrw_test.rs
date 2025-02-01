#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    negative_impls,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports,
    unused_mut,
    private_interfaces,
    non_upper_case_globals
)]
#[allow(dead_code)]
mod futexrw;
extern crate core;
use futexrw::*;
//include!("../common.rs");

#[allow(dead_code)]
extern "C" {
    fn puts(msg: *const u8);
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
    fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
    fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
fn main() {
    let rwlock = RwLock::new(0_u32);
    for i in 1..101_u32 {
        //unsafe{printf("Index is:%u\n\0".as_ptr() as *const i8,i)};
        *(rwlock.write().unwrap()) += 1;
        let value = *rwlock.read().unwrap();
        if value != i {
            unsafe {
                printf(
                    "RWlock value invalid. val:%u, should be %u \n\0".as_ptr() as *const i8,
                    value,
                    i,
                )
            };
            //unsafe { core::intrinsics::abort() };
        } else {
            unsafe {
                printf(
                    "RWlock value is ok. val:%u, should be %u \n\0".as_ptr() as *const i8,
                    value,
                    i,
                )
            };
        }
    }
    let val = rwlock.into_inner().unwrap();
    if val != 100 {
        unsafe {
            printf(
                "RWlock value invalid. val:%u  \n\0".as_ptr() as *const i8,
                val,
            )
        };
        unsafe { core::intrinsics::abort() };
    };
    unsafe { printf("RWlock value is OK! val:%u \n\0".as_ptr() as *const i8, val) };
}
