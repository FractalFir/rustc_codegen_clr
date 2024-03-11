#![feature(core_intrinsics)]
#![allow(arithmetic_overflow)]
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
#[derive(Copy, Clone)]
pub enum Adt39 {
    Variant0 {
        fld0: (u64,),
        fld1: (char,),
        fld2: isize,
        fld3: u16,
        fld4: i128,
    },
    Variant1 {
        fld0: (u8, Adt22, f64),
        fld1: [u64; 1],
        fld2: u128,
        fld3: (char,),
        fld4: [isize; 8],
        fld5: *const i32,
        fld6: u64,
    },
    Variant2 {
        fld0: u8,
        fld1: (u8,),
        fld2: f64,
        fld3: Adt22,
    },
}
impl Adt39{
    const fn default()->Self{
        Self::Variant0 {
            fld0: (0,),
            fld1: ('\0',),
            fld2: 0,
            fld3: 0,
            fld4: 0,
        }
    }
}
impl Default for Adt39{
    fn default()->Self{
        Self::default()
    }
}
#[derive(Copy, Clone,Default)]
pub struct Adt22 {
    fld0: u16,
}
unsafe fn fn12() -> *const i32 {
    let RET: *const i32;
    let mut _5: char;
    let mut _11: f32;
    //let mut _12: (&Adt39,);
    let mut _13: i32 = <i32>::default();
    let mut _14: usize;
    let mut _17: i64;

   
    _5 = '\u{3db66}';
    _17 = -6951135566362712794_i64;
    _14 = (-2147483648_i32) as usize;
    _13 = !451078155_i32;
     dump_var(
        12_usize,
        17_usize,
        (_17),
        5_usize,
        (_5),
        13_usize,
        (_13),
        14_usize,
        (_14),
    );
    RET = core::ptr::addr_of!(_13);
    // bb18
    return RET;
}

pub fn main() {
    unsafe {
        core::hint::black_box(fn12());
    }
}
