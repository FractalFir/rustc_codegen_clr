#![recursion_limit = "1024"]
#![feature(custom_mir, core_intrinsics, const_hash)]
#![allow(
    unused_parens,
    unused_assignments,
    overflowing_literals,
    internal_features
)]
extern crate core;
use core::intrinsics::mir::*;

use std::ffi::{c_char, c_int};

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
trait PrintFDebug {
    unsafe fn printf_debug(&self);
}
impl<T: PrintFDebug> PrintFDebug for *const T {
    unsafe fn printf_debug(&self) {
        unsafe { (**self).printf_debug() };
    }
}
impl<T: PrintFDebug> PrintFDebug for *mut T {
    unsafe fn printf_debug(&self) {
        unsafe { (**self).printf_debug() };
    }
}
impl<T: PrintFDebug> PrintFDebug for &T {
    unsafe fn printf_debug(&self) {
        (**self).printf_debug();
    }
}
impl<T: PrintFDebug> PrintFDebug for &mut T {
    unsafe fn printf_debug(&self) {
        (**self).printf_debug();
    }
}
impl PrintFDebug for i8 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self as i8 as c_int);
    }
}
impl PrintFDebug for u8 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u8 as c_int);
    }
}
impl PrintFDebug for i16 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self as i16 as c_int);
    }
}
impl PrintFDebug for u16 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u16 as c_int);
    }
}
impl PrintFDebug for i32 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for f32 {
    unsafe fn printf_debug(&self) {
        printf(
            "%f\0".as_ptr() as *const c_char,
            *self as core::ffi::c_double,
        );
    }
}
impl PrintFDebug for f64 {
    unsafe fn printf_debug(&self) {
        printf(
            "%f\0".as_ptr() as *const c_char,
            *self as core::ffi::c_double,
        );
    }
}
impl<T: PrintFDebug, const N: usize> PrintFDebug for [T; N] {
    unsafe fn printf_debug(&self) {
        printf("[\0".as_ptr() as *const c_char);
        for b in self {
            b.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
        }
        printf("]\0".as_ptr() as *const c_char);
    }
}
impl PrintFDebug for u32 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for char {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u64);
    }
}
impl PrintFDebug for i64 {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for u64 {
    unsafe fn printf_debug(&self) {
        printf("%lu\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for i128 {
    unsafe fn printf_debug(&self) {
        u128::printf_debug(&(*self as u128));
    }
}
impl PrintFDebug for u128 {
    unsafe fn printf_debug(&self) {
        printf(
            "%lx%lx\0".as_ptr() as *const c_char,
            (*self >> 64) as u64,
            *self as u64,
        );
    }
}
impl PrintFDebug for isize {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self as isize);
    }
}
impl PrintFDebug for usize {
    unsafe fn printf_debug(&self) {
        printf("%lu\0".as_ptr() as *const c_char, *self as usize);
    }
}
impl PrintFDebug for bool {
    unsafe fn printf_debug(&self) {
        if *self {
            printf("true\0".as_ptr() as *const c_char);
        } else {
            printf("false\0".as_ptr() as *const c_char);
        }
    }
}
impl PrintFDebug for () {
    unsafe fn printf_debug(&self) {
        printf("()\0".as_ptr() as *const c_char);
    }
}
impl<A: PrintFDebug> PrintFDebug for (A,) {
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",)\0".as_ptr() as *const c_char);
    }
}
impl<A: PrintFDebug, B: PrintFDebug> PrintFDebug for (A, B) {
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<A: PrintFDebug, B: PrintFDebug, C: PrintFDebug> PrintFDebug for (A, B, C) {
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<A: PrintFDebug, B: PrintFDebug, C: PrintFDebug, D: PrintFDebug> PrintFDebug for (A, B, C, D) {
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<A: PrintFDebug, B: PrintFDebug, C: PrintFDebug, D: PrintFDebug, E: PrintFDebug> PrintFDebug
    for (A, B, C, D, E)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
        G: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F, G)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.6.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
        G: PrintFDebug,
        H: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F, G, H)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.6.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.7.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
        G: PrintFDebug,
        H: PrintFDebug,
        I: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F, G, H, I)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.6.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.7.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.8.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
        G: PrintFDebug,
        H: PrintFDebug,
        I: PrintFDebug,
        J: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F, G, H, I, J)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.6.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.7.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.8.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.9.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
        G: PrintFDebug,
        H: PrintFDebug,
        I: PrintFDebug,
        J: PrintFDebug,
        K: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F, G, H, I, J, K)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.6.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.7.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.8.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.9.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.10.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
impl<
        A: PrintFDebug,
        B: PrintFDebug,
        C: PrintFDebug,
        D: PrintFDebug,
        E: PrintFDebug,
        F: PrintFDebug,
        G: PrintFDebug,
        H: PrintFDebug,
        I: PrintFDebug,
        J: PrintFDebug,
        K: PrintFDebug,
        L: PrintFDebug,
    > PrintFDebug for (A, B, C, D, E, F, G, H, I, J, K, L)
{
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.1.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.2.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.3.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.4.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.5.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.6.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.7.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.8.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.9.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.10.printf_debug();
        printf(",\0".as_ptr() as *const c_char);
        self.11.printf_debug();
        printf(")\0".as_ptr() as *const c_char);
    }
}
#[inline(never)]
fn dump_var(
    f: usize,
    var0: usize,
    val0: impl PrintFDebug,
    var1: usize,
    val1: impl PrintFDebug,
    var2: usize,
    val2: impl PrintFDebug,
    var3: usize,
    val3: impl PrintFDebug,
) {
    unsafe {
        printf("fn%u:_%u = \0".as_ptr() as *const c_char, f, var0);
        val0.printf_debug();
        printf("\n_%u = \0".as_ptr() as *const c_char, var1);
        val1.printf_debug();
        printf("\n_%u = \0".as_ptr() as *const c_char, var2);
        val2.printf_debug();
        printf("\n_%u = \0".as_ptr() as *const c_char, var3);
        val3.printf_debug();
        printf("\n\0".as_ptr() as *const c_char);
    }
}

fn fn6(
  
) {

   
    let tmp5 = Adt50 {
        fld0: 50,
        fld1: [false; 6],
    };
 
    
    unsafe{
        printf("offset of fld0:%p. size of fld0:%x\n\0".as_ptr() as *const i8, core::ptr::addr_of!(tmp5.fld0) as usize - core::ptr::addr_of!(tmp5) as usize,core::mem::size_of_val(&tmp5.fld0) as u32);
        printf("offset of fld1:%p. size of fld1:%x\n\0".as_ptr() as *const i8, core::ptr::addr_of!(tmp5.fld1) as usize - core::ptr::addr_of!(tmp5) as usize,core::mem::size_of_val(&tmp5.fld1) as u32);
    }
    dump_var(
        6_usize,
        0_usize,
        (tmp5.fld0),
        1_usize,
        (tmp5.fld1),
        0xFF_usize,
        (),
        0xFF_usize,
        (),
    );
}

pub fn main() {
    fn6();
    fn9();
}

impl PrintFDebug for Adt50 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt50{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone, Debug,Default)]
pub struct Adt50 {
    fld0: i16,
    fld1: [bool; 6],
}

fn fn9() {

let mut _11: [usize; 5];




let two = std::hint::black_box(2);
let not_two = std::hint::black_box(!two); 
let tmp = not_two / two;
_11 = [tmp,tmp,tmp,tmp,tmp];



dump_var(9_usize, 9_usize, (tmp), 1_usize, (two), 2_usize, not_two, 0xFF_usize, ());



}
