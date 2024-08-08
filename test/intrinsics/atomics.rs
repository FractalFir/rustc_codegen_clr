#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params,
    strict_provenance,
    strict_provenance_atomic_ptr
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::SeqCst;

include!("../common.rs");
extern crate core;

use core::ptr::addr_of_mut;

fn main() {
    let mut u: u32 = black_box(20);
    test_eq!(
        unsafe { core::intrinsics::atomic_xsub_release(addr_of_mut!(u), 10) },
        20
    );
    let mut u: u32 = black_box(20);
    let (val, is_eq) =
        unsafe { core::intrinsics::atomic_cxchgweak_acquire_relaxed(addr_of_mut!(u), 20_u32, 10) };
    test_eq!(val, 20_u32);
    test_eq!(u, 10_u32);
    //test_eq!(is_eq,true);
    let (val, is_eq) =
        unsafe { core::intrinsics::atomic_cxchgweak_acquire_relaxed(addr_of_mut!(u), 10_u32, 20) };
    test_eq!(val, 10_u32);
    ptr_bitops_tagging();
}
fn ptr_bitops_tagging() {
    #[repr(align(16))]
    struct Tagme(#[allow(dead_code)] u128);

    let tagme = Tagme(1000);
    let ptr = &tagme as *const Tagme as *mut Tagme;
    let atom: AtomicPtr<Tagme> = AtomicPtr::new(ptr);

    const MASK_TAG: usize = 0b1111;
    const MASK_PTR: usize = !MASK_TAG;
    unsafe {
        printf(
            c"The 16 byte aligned tagme struct is located at an address of %p\n".as_ptr(),
            ptr.addr(),
        )
    };
    test_eq!(ptr.addr() & MASK_TAG, 0);

    test_eq!(atom.fetch_or(0b0111, SeqCst), ptr);
    test_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b111));

    test_eq!(
        atom.fetch_and(MASK_PTR | 0b0010, SeqCst),
        ptr.map_addr(|a| a | 0b111)
    );
    test_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b0010));
    // XOR not yet supported
    /*test_eq!(atom.fetch_xor(0b1011, SeqCst), ptr.map_addr(|a| a | 0b0010));
    test_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b1001));

    test_eq!(
        atom.fetch_and(MASK_PTR, SeqCst),
        ptr.map_addr(|a| a | 0b1001)
    );
    test_eq!(atom.load(SeqCst), ptr);*/
}
