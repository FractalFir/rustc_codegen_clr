#![feature(slice_ptr_get)]
#![feature(allocator_api)]
use core::alloc::Allocator;
use alloc::alloc::handle_alloc_error;
use alloc::alloc::Global;
use alloc::alloc::Layout;

extern crate alloc;
pub unsafe fn exchange_malloc(size: usize, align: usize) -> *mut u8 {
    let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
    let res = match Global.allocate(layout) {
        Ok(ptr) => ptr.as_mut_ptr(),
        Err(_) => handle_alloc_error(layout),
    };
    if res == std::ptr::null_mut(){
        core::intrinsics::abort();
    }
    //unsafe{super::printf("Allocated buffer of size %p algined to %p at address %p\n\r\n\r\0".as_ptr() as *const i8,size,align,res)};
    unsafe{super::printf("Allocating pointer at address %p.\n\0".as_ptr() as *const i8,res)};
    unsafe{super::printf("The align is %p.\n\0".as_ptr() as *const i8,align)};
    res
}

pub fn exchange_malloc_test() {
    let mut _2: usize;
    let mut _3: usize;
    let mut _4: *mut u8;
    let mut _6: *mut f64;
    let mut _7: *const ();
    let mut _8: usize;
    let mut _9: usize;
    let mut _10: usize;
    let mut _11: usize;
    let mut _12: bool;
    _2 = core::mem::size_of::<f64>();
    _3 = core::mem::align_of::<f64>();
    unsafe{_4 = exchange_malloc(_2, _3)};
    if _4 == std::ptr::null_mut(){
        core::intrinsics::abort();
    }
    _7 = _4 as *const ();
    _8 = _7 as usize;
    _9 = core::mem::align_of::<f64>();
    _10 = _9 - 1_usize;
    _11 = _8 & _10;
    _12 = _11 == 0_usize;
    _6 = _4 as *mut f64;
    unsafe{super::printf("Allocated ptr at address %p\n\0".as_ptr() as *const i8,_6)};
    if _6 == std::ptr::null_mut(){
        core::intrinsics::abort();
    }
    unsafe{*_6 = 64.0};
    //assert!(_12, "misaligned pointer dereference: address must be a multiple of {} but is {}", _9, _8);
    return;
}


