#![feature(core_intrinsics)]
#![allow(arithmetic_overflow,overflowing_literals)]
//#![deny(dead_code)]
#![deny(unused_variables)]
use std::ffi::{c_char, c_int};


extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
trait PrintFDebug {
    unsafe fn printf_debug(&self);
}
impl PrintFDebug for i64 {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for char {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u64);
    }
}
impl PrintFDebug for usize {
    unsafe fn printf_debug(&self) {
        printf("%lu\0".as_ptr() as *const c_char, *self as usize);
    }
}
impl PrintFDebug for i32 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self);
    }
}

impl PrintFDebug for u8 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u8 as c_int);
    }
}
impl PrintFDebug for u128{
    unsafe fn printf_debug(&self){
        printf("%lx%lx\0".as_ptr() as *const c_char, (*self >> 64) as u64,*self as u64);
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
impl PrintFDebug for u64 {
    unsafe fn printf_debug(&self) {
        printf("%lu\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for () {
    unsafe fn printf_debug(&self) {
        printf("()\0".as_ptr() as *const c_char);
    }
}
impl PrintFDebug for u32 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for u16 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u16 as c_int);
    }
}
impl PrintFDebug for isize {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self as isize);
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


fn fn9(mut _1: isize) -> char {
    let mut ret:char;
    let mut _2: isize;
    let mut _4: char = 'a';

    let mut _6: i32;
    let mut _7: isize;
    let mut _8: bool;
    
    let mut _10: ([u64; 3], isize);
    let mut _11: u32;
    let mut _12: [u8; 6];

    let mut _14: char;
    let mut _15: bool;

    let mut _17: f64;

    let mut _22: usize;
    let mut _24: u32;
    let mut _25: u16 = 0;
    let mut _26: isize;

    let mut _34: () = ();
    ret = '\u{c114e}';
    _2 = -_1;
    _6 = 1375518148_i32 + 1246297573_i32;
    ret = _4;
    _1 = _2;
    _4 = ret;
    _7 = !_2;
    _1 = -_7;
    ret = _4;
    _7 = 2570451067848647683_u64 as isize;
    _4 = ret;
    ret = _4;

    ret = _4;
    _8 = !false;
    _12 = [86_u8,110_u8,230_u8,12_u8,67_u8,115_u8];
    _2 = !_1;
    _1 = _7;
    _17 = 174_u8 as f64;
    _15 = !_8;
    _8 = _15;
    _11 = 1853534658_u32;
    _14 = ret;
    _4 = ret;
    _14 = _4;
    _15 = _8;
    ret = _14;
    _2 = _15 as isize;
    _1 = !_2;
    _14 = ret;
    _22 = !0_usize;
    _17 = _22 as f64;
    _26 = 69_i8 as isize;

    _24 = _17 as u32;
    _22 = 8437857178953584601_usize;
    dump_var(9_usize, 24_usize, _24, 14_usize, _14, 1_usize, _1, 2_usize, _2);
    return ret;
}

pub fn main() {
    let mut _17 = 8437857178953584601_usize as f64;
    if _17 != core::hint::black_box(8437857178953585000.0){
        core::intrinsics::abort();
    }
    let mut _24 = _17 as u32;
    if _24 != core::hint::black_box(4294967295){
        core::intrinsics::abort();
    }
    core::hint::black_box(fn9(core::hint::black_box(1081299219633787456)));
}
