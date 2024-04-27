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
extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}
trait PrintFDebug {
    unsafe fn printf_debug(&self);
}
impl PrintFDebug for () {
    unsafe fn printf_debug(&self) {
        printf("()\0".as_ptr() as *const c_char);
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
impl<A: PrintFDebug> PrintFDebug for (A,) {
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",)\0".as_ptr() as *const c_char);
    }
}
impl PrintFDebug for isize {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self as isize);
    }
}
