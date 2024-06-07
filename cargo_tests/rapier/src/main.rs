#![feature(fmt_internals,sync_unsafe_cell,numfmt,core_intrinsics,flt2dec,no_sanitize,extern_types,specialization,maybe_uninit_uninit_array,maybe_uninit_slice,never_type,exposed_provenance,trusted_len)]
#![feature(adt_const_params,test,os_str_display,extend_one,fs_try_exists,strict_provenance)]
use std::fmt::Debug;

// /mod path;

use std::fs::File;
use std::hint;
use std::io::{self, Write, Read};
use std::net::TcpStream;

//use term::terminfo::searcher::get_dbpath_for_term;

fn main() {
    let mut test = std::hint::black_box(<[_]>::into_vec(std::boxed::Box::new([b'a'])));
    eprintln!("At init length is {}. cap is {}!",test.len(),test.capacity());
    unsafe{println!("test bytes:{:?}",&*(&test as *const _ as *const [usize;core::mem::size_of::<Vec<u8>>()/core::mem::size_of::<usize>()]))};
    println!("test_addr:{:x}",core::ptr::addr_of!(test) as usize);
    test.reserve(10);
    unsafe{println!("test bytes:{:?}",&*(&test as *const _ as *const [usize;core::mem::size_of::<Vec<u8>>()/core::mem::size_of::<usize>()]))};
    println!("test_addr:{:x}",core::ptr::addr_of!(test) as usize);
    // Should be 1, but is 0.
    eprintln!("After reserve length is {}. cap is {}!",test.len(),test.capacity());
}
fn pass(seld:&mut Vec<u8>){
    seld.reserve(10);
    println!("self.len is :{}",seld.len());
}
fn vec_len(seld:&mut Vec<u8>)->&mut usize{
    unsafe{&mut *(seld as *mut _ as *mut usize).add(2)}
}
fn extend_trusted(seld:&mut Vec<u8>, iterator: impl core::iter::Iterator<Item = u8>) {
    let (low, high) = iterator.size_hint();
    if let Some(additional) = high {
        debug_assert_eq!(
            low,
            additional,
            "TrustedLen iterator's size hint is not exact: {:?}",
            (low, high)
        );
        seld.reserve(additional);
        unsafe {
            let ptr = seld.as_mut_ptr();
            println!("self.len is :{}",seld.len());
            let mut local_len = SetLenOnDrop::new(vec_len(seld));
            iterator.for_each(move |element| {
                core::ptr::write(ptr.add(local_len.current_len()), element);
                // Since the loop executes user code which can panic we have to update
                // the length every step to correctly drop what we've written.
                // NB can't overflow since we would have had to alloc the address space
                local_len.increment_len(1);
            });
        }
    } else {
        // Per TrustedLen contract a `None` upper bound means that the iterator length
        // truly exceeds usize::MAX, which would eventually lead to a capacity overflow anyway.
        // Since the other branch already panics eagerly (via `reserve()`) we do the same here.
        // This avoids additional codegen for a fallback code path which would eventually
        // panic anyway.
        panic!("capacity overflow");
    }
}
pub struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}

impl<'a> SetLenOnDrop<'a> {
    #[inline]
    pub fn new(len: &'a mut usize) -> Self {
        eprintln!("Creating. len is:{}",len);
        SetLenOnDrop { local_len: *len, len }
    }

    #[inline]
    pub fn increment_len(&mut self, increment: usize) {
        eprintln!("Incrementing len {}",increment);
        self.local_len += increment;
    }

    #[inline]
    pub fn current_len(&self) -> usize {
        self.local_len
    }
}

impl Drop for SetLenOnDrop<'_> {
    #[inline]
    fn drop(&mut self) {
        eprintln!("Setting len to {}",self.local_len);
        *self.len = self.local_len;
    }
}