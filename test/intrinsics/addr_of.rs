#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
use core::ptr::addr_of_mut;
use core::ptr::addr_of;
#[derive(Default)]
struct Quad<T:Default>{
    a:T,
    b:T,
    c:T,
    d:T
}
fn main(){
    let quad = Quad::<u8>::default();
    test_eq!(black_box(addr_of!(quad) as usize + 2),addr_of!(quad.c) as usize);
    let mut quad = Quad::<Quad::<u8>>::default();
    // Test field offsets
    test_eq!(black_box(addr_of!(quad) as usize + 10),addr_of!(quad.c.c) as usize);
    test_eq!(black_box(addr_of!(quad) as usize + 2),addr_of!(quad.a.c) as usize);
    test_eq!(black_box(addr_of_mut!(quad) as usize + 10),addr_of_mut!(quad.c.c) as usize);
    test_eq!(black_box(addr_of_mut!(quad) as usize + 2),addr_of_mut!(quad.a.c) as usize);
    // Check field addres is not 0
    test_ne!(addr_of!(quad.c.c) as usize,0);
    // Check the value of the field is propely initialzed. Check that changing it trough a pointer works.
    test_eq!(black_box(quad.c.c), 0);
    unsafe{*black_box(addr_of_mut!(quad.c.c)) = 138};
    test_eq!(black_box(quad.c.c), 138);
    // Checks that the pointer roundtripped.
    let ptr_val = black_box(addr_of_mut!(quad.c.c)) as isize;
    let ptr_val_2 = black_box(ptr_val) + 224488;
    let ptr_val_3 = black_box(ptr_val_2) - 114444;
    let ptr_val_4 = black_box(ptr_val_3) - 110044;
    test_eq!(black_box(quad.c.c), 138);
    unsafe{*black_box(ptr_val_4 as *mut u8) = 0xAA};
    test_eq!(black_box(quad.c.c), 0xAA);
    // Test if pointer offsets work correctly
    test_eq!(unsafe{black_box(addr_of_mut!(quad.a).offset(2)) as usize + 2},addr_of_mut!(quad.c.c) as usize);
    let f0 = 7818556801315723626_usize as u64;
    let f1 = [25218654463224818122969828049712073135_i128,120712558264094237432810624595822131062_i128,114515168747267207915719546104105726720_i128,(-146246673762081125525637774717577019298_i128)];
    let f2 = 3117618922_u32 - 210026923_u32;
    let f3 = 9223372036854775807_isize as f64;
    let a:(u64, [i128; 4], u32, f64) = (f0,f1,f2,f3);
    test_eq!(black_box(addr_of!(a) as usize + 64),addr_of!(a.0) as usize);
    test_eq!(black_box(addr_of!(a) as usize + 0),addr_of!(a.1) as usize);
    test_eq!(black_box(addr_of!(a) as usize + 72),addr_of!(a.2) as usize);
    test_eq!(black_box(addr_of!(a) as usize + 80),addr_of!(a.3) as usize);
    let a = black_box(a);
    test_eq!(a.0,f0);
}
    