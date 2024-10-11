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
impl PrintFDebug for usize {
    unsafe fn printf_debug(&self) {
        printf(c"%lu".as_ptr(), *self as usize);
    }
}
impl PrintFDebug for u64 {
    unsafe fn printf_debug(&self) {
        printf(c"%lu".as_ptr(), *self);
    }
}
impl PrintFDebug for () {
    unsafe fn printf_debug(&self) {
        printf(c"()".as_ptr());
    }
}
impl PrintFDebug for u32 {
    unsafe fn printf_debug(&self) {
        printf(c"%u".as_ptr(), *self);
    }
}
impl PrintFDebug for u16 {
    unsafe fn printf_debug(&self) {
        printf(c"%u".as_ptr(), *self as u16 as c_int);
    }
}
impl PrintFDebug for f64{
    unsafe fn printf_debug(&self){
        printf(c"%f".as_ptr(),*self as core::ffi::c_double);
    }
}
impl<A: PrintFDebug> PrintFDebug for (A,) {
    unsafe fn printf_debug(&self) {
        printf(c"(".as_ptr());
        self.0.printf_debug();
        printf(c",)".as_ptr());
    }
}
impl<A: PrintFDebug, B: PrintFDebug> PrintFDebug for (A, B) {
    unsafe fn printf_debug(&self) {
        printf(c"(".as_ptr());
        self.0.printf_debug();
        printf(c",".as_ptr());
        self.1.printf_debug();
        printf(c")".as_ptr());
    }
}
impl<T: PrintFDebug, const N: usize> PrintFDebug for [T; N] {
    unsafe fn printf_debug(&self) {
        printf(c"[".as_ptr());
        for b in self {
            b.printf_debug();
            printf(c",".as_ptr());
        }
        printf(c"]".as_ptr());
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
        printf(c"fn%u:_%u = ".as_ptr(), f, var0);
        val0.printf_debug();
        printf(c"\n_%u = ".as_ptr(), var1);
        val1.printf_debug();
        printf(c"\n_%u = ".as_ptr(), var2);
        val2.printf_debug();
        printf(c"\n_%u = ".as_ptr(), var3);
        val3.printf_debug();
        printf(c"\n".as_ptr());
    }
}


fn fn9(mut _1: isize) -> char {
    let mut _17: f64;
    let mut _22: usize;
    let mut _24: u32;
    _22 = !0_usize;
    _17 = _22 as f64;
    _24 = _17 as u32;
    _22 = !0_usize;
    dump_var(9_usize, 24_usize, _24, 22_usize, _22, 17_usize, _17, 2_usize,  ());
    return '\0';
}

pub fn main() {
    let mut _17 = core::hint::black_box(!0_usize) as f64;
    if _17 != core::hint::black_box(18446744073709552000.0){
        core::intrinsics::abort();
    }
    /*
    let mut _24 = _17 as u32;
    if _24 != core::hint::black_box(4294967295){
        core::intrinsics::abort();
    }
    core::hint::black_box(fn9(core::hint::black_box(1081299219633787456))); */
}
