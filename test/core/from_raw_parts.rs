#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params,
    ptr_metadata
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
use core::ptr::NonNull;
use core::ptr;
use core::ptr::*;
use std::fmt::Display;
include!("../common.rs");
fn main(){
    let mut value = 5_u32;/*  */
    let address = &mut value as *mut _ as *mut ();
    let trait_object: &dyn Display = &mut value;
    let vtable = metadata(trait_object);
    let trait_object = NonNull::from(trait_object);
    std::hint::black_box::<*const dyn Display>(ptr::from_raw_parts(address, vtable));// == trait_object.as_ptr() as *const _);
   test_eq!(ptr::from_raw_parts(address, vtable), trait_object.as_ptr() as *const _);
    test_eq!(ptr::from_raw_parts_mut(address, vtable), trait_object.as_ptr());
    test_eq!(NonNull::from_raw_parts(NonNull::new(address).unwrap(), vtable), trait_object);

    let mut array = [5_u32, 5, 5, 5, 5];
    let address = &mut array as *mut _ as *mut ();
    let array_ptr = NonNull::from(&mut array);
    let slice_ptr = NonNull::from(&mut array[..]);

   test_eq!(ptr::from_raw_parts(address, ()), array_ptr.as_ptr() as *const _);
    test_eq!(ptr::from_raw_parts_mut(address, ()), array_ptr.as_ptr());
    test_eq!(NonNull::from_raw_parts(NonNull::new(address).unwrap(), ()), array_ptr);

    test_eq!(ptr::from_raw_parts(address, 5_usize), slice_ptr.as_ptr() as *const _);
    test_eq!(ptr::from_raw_parts_mut(address, 5), slice_ptr.as_ptr());
    test_eq!(NonNull::from_raw_parts(NonNull::new(address).unwrap(), 5), slice_ptr);

    }