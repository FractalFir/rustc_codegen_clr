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
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_unsafe
)]
#![no_std]
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::SeqCst;

include!("../common.rs");
extern crate core;
extern "C" {
    fn atomic_xor_u32(addr: &mut u32, xorand: u32) -> u32;
    fn atomic_nand_u32(addr: &mut u32, xorand: u32) -> u32;
    //fn atomic_cmpxchng_i32(addr: *mut i32, bytes: i32) -> i32;
}
use core::ptr::addr_of_mut;
//fn compare_exchange_byte(addr:&mut u8, byte:u8)->u8
fn main() {
    let mut u: u32 = black_box(20);
    let sub_old = unsafe { core::intrinsics::atomic_xsub_release(addr_of_mut!(u), 10) };
    unsafe { printf(c"sub_old:%lx\n".as_ptr(), sub_old) };
    test_eq!(sub_old, 20);
    let mut u: u32 = black_box(20);
    let (val, is_eq) =
        unsafe { core::intrinsics::atomic_cxchgweak_acquire_relaxed(addr_of_mut!(u), 20_u32, 10) };
    test_eq!(val, 20_u32);
    test_eq!(u, 10_u32);
    //test_eq!(is_eq,true);
    let (val, is_eq) =
        unsafe { core::intrinsics::atomic_cxchgweak_acquire_relaxed(addr_of_mut!(u), 10_u32, 20) };
    test_eq!(val, 10_u32);
    let mut tmp = 0xFF_u32;
    unsafe { test_eq!(atomic_xor_u32(&mut tmp, 0x0A), 0xFF_u32) };
    test_eq!(tmp, 0xFF ^ 0x0A);
    let mut tmp = 0xFF_u32;
    unsafe { test_eq!(atomic_nand_u32(&mut tmp, 0x0A), 0xFF_u32) };
    test_eq!(tmp, !(0xFF & 0x0A));
    ptr_bitops_tagging();
    let atomic = core::sync::atomic::AtomicUsize::new(0);
    test_eq!(atomic.load(core::sync::atomic::Ordering::Relaxed), 0);
    let atomic_old = atomic.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    unsafe { printf(c"atomic_old:%lx\n".as_ptr(), atomic_old as u64) };
    test_eq!(atomic_old, 0);
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
    test_eq!(atom.fetch_xor(0b1011, SeqCst), ptr.map_addr(|a| a | 0b0010));
    test_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b1001));

    test_eq!(
        atom.fetch_and(MASK_PTR, SeqCst),
        ptr.map_addr(|a| a | 0b1001)
    );
    test_eq!(atom.load(SeqCst), ptr);
}
