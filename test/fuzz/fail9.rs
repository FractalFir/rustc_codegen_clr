#![recursion_limit = "1024"]

extern crate core;
use core::ffi::c_char;
use core::ffi::c_int;

pub fn fn8(
   
) {
    let _6: isize;
    let mut _8: (isize,);
    _6 = -9223372036854775808_isize;
    _8 = (_6,);
    dump_var(8_usize, 8_usize, _8, 8_usize, _8, 6_usize, _6, 17_usize, ());
}

fn main() {
    fn8();
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
extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
trait PrintFDebug {
    unsafe fn printf_debug(&self);
}
impl PrintFDebug for () {
    unsafe fn printf_debug(&self) {
        printf(c"()".as_ptr());
    }
}
impl PrintFDebug for bool {
    unsafe fn printf_debug(&self) {
        if *self {
            printf(c"true".as_ptr());
        } else {
            printf(c"false".as_ptr());
        }
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
impl PrintFDebug for u32 {
    unsafe fn printf_debug(&self) {
        printf(c"%u".as_ptr(), *self);
    }
}
impl<A: PrintFDebug> PrintFDebug for (A,) {
    unsafe fn printf_debug(&self) {
        printf(c"(".as_ptr());
        self.0.printf_debug();
        printf(c",)".as_ptr());
    }
}
impl PrintFDebug for isize {
    unsafe fn printf_debug(&self) {
        printf(c"%li".as_ptr(), *self as isize);
    }
}
