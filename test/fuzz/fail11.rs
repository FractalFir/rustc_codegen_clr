#![recursion_limit = "1024"]
#![feature(custom_mir, core_intrinsics, const_hash)]
#![allow(
    unused_parens,
    unused_assignments,
    overflowing_literals,
    internal_features
)]
#![allow(dead_code, unused_imports, unused_mut)]

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
        if self.is_nan() {
            printf("NaN\0".as_ptr() as *const c_char);
        } else {
            printf(
                "%f\0".as_ptr() as *const c_char,
                *self as core::ffi::c_double,
            );
        }
    }
}
impl PrintFDebug for f64 {
    unsafe fn printf_debug(&self) {
        if self.is_nan() {
            printf("NaN\0".as_ptr() as *const c_char);
        } else {
            printf(
                "%f\0".as_ptr() as *const c_char,
                *self as core::ffi::c_double,
            );
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

fn fn6() {
    let tmp5 = Adt50 {
        fld0: 50,
        fld1: [false; 6],
    };

    unsafe {
        printf(
            "offset of fld0:%p. size of fld0:%x\n\0".as_ptr() as *const i8,
            core::ptr::addr_of!(tmp5.fld0) as usize - core::ptr::addr_of!(tmp5) as usize,
            core::mem::size_of_val(&tmp5.fld0) as u32,
        );
        printf(
            "offset of fld1:%p. size of fld1:%x\n\0".as_ptr() as *const i8,
            core::ptr::addr_of!(tmp5.fld1) as usize - core::ptr::addr_of!(tmp5) as usize,
            core::mem::size_of_val(&tmp5.fld1) as u32,
        );
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

    billy();
    fn1();
    fn16(
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        '\u{4a457}',
        [8681242669872074330, 7, 5],
        [8681242669872074330, 7, 5],
        81040590000000000000000000000000000000.0,
        '\u{4a457}',
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        (8.104059e37,),
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
        [
            -28,
            9223372036854775807,
            9223372036854775807,
            -9223372036854775808,
            85,
            9223372036854775807,
        ],
    );
}

impl PrintFDebug for Adt50 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt50{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone, Debug, Default)]
pub struct Adt50 {
    fld0: i16,
    fld1: [bool; 6],
}

fn fn9() {
    let mut _11: [usize; 5];
    let two = std::hint::black_box(2);
    let not_two = std::hint::black_box(!two);
    let tmp = not_two / two;
    _11 = [tmp, tmp, tmp, tmp, tmp];
    dump_var(
        9_usize,
        9_usize,
        (tmp),
        1_usize,
        (two),
        2_usize,
        not_two,
        0xFF_usize,
        (),
    );
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1() -> f64 {
    mir! {
    type RET = f64;
    let _1: (i16, bool);
    let _2: (i16, bool);
    let _3: (i16, bool);
    let _4: (i16, bool);
    let _31: ();

    {
    _1 = Checked((-2150_i16) + (-21040_i16));
    _2 = Checked((2150_i16) + (-21040_i16));
    _3 = Checked((2150_i16) + (21040_i16));
    _4 = Checked((-2150_i16) + (21040_i16));
    Call(_31 = dump_var(1_usize, 1_usize, Move(_1), 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    Return()
    }

    }
}

fn billy() {
    let tmp = (-1886154176_i64) >> std::hint::black_box(35_usize);
    dump_var(
        16_usize,
        18_usize,
        (tmp),
        23_usize,
        (),
        23_usize,
        (),
        23_usize,
        (),
    );
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(
    mut _1: [isize; 6],
    mut _2: [isize; 6],
    mut _3: [isize; 6],
    mut _4: char,
    mut _5: [usize; 3],
    mut _6: [usize; 3],
    mut _7: f32,
    mut _8: char,
    mut _9: [isize; 6],
    mut _10: (f32,),
    mut _11: [isize; 6],
    mut _12: [isize; 6],
    mut _13: [isize; 6],
    mut _14: [isize; 6],
    mut _15: [isize; 6],
    mut _16: [isize; 6],
) -> u16 {
    mir! {
    type RET = u16;
    let _17: isize;

    let _19: char;
    let _20: [char; 7];
    let _21: [char; 4];
    let _22: [i8; 5];
    let _23: [char; 7];
    let _24: (f32,);
    let _25: isize;
    let _26: i128;
    let _27: isize;
    let _28: i8;
    let _29: f64;
    let _30: [usize; 3];

    let _32: *const *mut [i8; 5];
    let _33: Adt45;
    let _34: f32;

    let _36: i64;
    let _37: i64;
    let _38: char;
    let _39: f64;
    let _40: char;

    let _42: (bool, i32);
    let _43: [char; 8];

    let _45: bool;

    let _47: [char; 8];
    let _48: u128;
    let _49: char;
    let _50: ();
    let _51: ();
    {
    _9 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-116_isize)];
    _8 = _4;
    RET = (-111516441711422647304073867959032708798_i128) as u16;
    RET = 50185_u16 ^ 26460_u16;
    _14 = [(-27_isize),(-9223372036854775808_isize),9223372036854775807_isize,67_isize,76_isize,9223372036854775807_isize];
    _1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),78_isize,9223372036854775807_isize];
    _10 = (_7,);
    _9 = [4_isize,(-98_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-93_isize),9223372036854775807_isize];
    RET = 61160_u16 + 58952_u16;
    _7 = _10.0;
    _1 = _14;

    _16 = [9223372036854775807_isize,(-30_isize),(-111_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
    _3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
    _10 = (_7,);
    _17 = 280238355645298633612051627014440070336_u128 as isize;

    _10.0 = _7;
    _14 = [_17,_17,_17,_17,_17,_17];
    _5 = [1861619566001845757_usize,1_usize,8760626428140770463_usize];
    _20 = [_8,_4,_8,_8,_4,_8,_4];
    RET = _4 as u16;
    _14 = _2;
    _14 = [_17,_17,_17,_17,_17,_17];
    _5 = _6;
    _10 = (_7,);
    _8 = _4;

    _9 = _11;
    _1 = [_17,_17,_17,_17,_17,_17];
    _2 = [_17,_17,_17,_17,_17,_17];
    _8 = _4;
    _9 = [_17,_17,_17,_17,_17,_17];
    _10 = (_7,);
    _21 = [_8,_4,_8,_4];
    RET = !40643_u16;
    _4 = _8;
    _10.0 = _7;
    _15 = _3;
    _15 = [_17,_17,_17,_17,_17,_17];
    _23 = _20;

    _10.0 = _7 * _7;
    _24 = _10;
    _10 = (_7,);
    RET = 472_u16 | 32604_u16;
    _19 = _4;
    _10 = (_24.0,);
    _9 = [_17,_17,_17,_17,_17,_17];
    _19 = _8;
    _24.0 = -_10.0;
    _24.0 = _10.0;

    _2 = [_17,_17,_17,_17,_17,_17];

    _23 = [_4,_4,_8,_4,_19,_4,_19];
    _2 = _14;
    _3 = [_17,_17,_17,_17,_17,_17];
    _24.0 = _10.0;
    _19 = _8;
    _3 = _12;
    _25 = _17;
    _20 = [_8,_19,_4,_8,_19,_8,_19];
    _4 = _8;
    _28 = !(-16_i8);
    _26 = _28 as i128;
    _24 = (_10.0,);
    _27 = _17 - _25;
    _12 = [_25,_25,_25,_17,_27,_17];
    _10.0 = _24.0;
    _23 = [_8,_4,_19,_4,_4,_8,_19];
    _23 = _20;

    _13 = [_25,_17,_27,_27,_27,_27];
    _12 = [_17,_17,_25,_17,_25,_17];
    _28 = (-76_i8) << _25;
    _28 = _26 as i8;
    _2 = [_27,_17,_17,_27,_27,_27];
    _17 = 14134_i16 as isize;
    _8 = _4;

    _28 = (-79_i8);
    _29 = 833620195993400214_u64 as f64;
    _20 = [_4,_8,_19,_8,_19,_19,_19];

    _23 = _20;
    _33.fld0.0 = _10.0 + _24.0;

    _22 = [_28,_28,_28,_28,_28];
    _33.fld3 = RET;
    _33.fld1 = core::ptr::addr_of!(_26);
    _6 = [2721556065459894470_usize,15654237979308234671_usize,4562457919922719912_usize];
    _20 = [_19,_4,_19,_4,_8,_8,_4];
    _7 = _33.fld0.0;
    _17 = _27 & _27;
    RET = _33.fld3 * _33.fld3;
    _19 = _4;
    _33.fld3 = RET | RET;
    _33.fld3 = RET >> RET;
    _29 = (-1977566323656098150_i64) as f64;
    _5 = [7311357670104946435_usize,4_usize,1_usize];
    _21 = [_8,_19,_19,_4];
    _12 = [_17,_17,_17,_27,_17,_27];
    _9 = _3;
    _33.fld3 = RET & RET;
    _33.fld2 = core::ptr::addr_of_mut!(_32);
    _33.fld1 = core::ptr::addr_of!(_26);
    _7 = -_33.fld0.0;
    _3 = [_17,_27,_27,_27,_27,_17];
    _33.fld0 = (_7,);

    _4 = _19;

    _33.fld0.0 = _10.0;
    _30 = [5_usize,4365508176095717335_usize,6_usize];
    _26 = (-802775440_i32) as i128;
    _19 = _4;
    _24 = _10;
    _2 = [_17,_25,_17,_17,_27,_27];
    _29 = 324605307_u32 as f64;
    _33.fld2 = core::ptr::addr_of_mut!(_32);
    _12 = [_25,_17,_17,_27,_17,_27];
    _10.0 = _7 - _24.0;
    _1 = [_25,_17,_17,_17,_17,_27];
    _10.0 = 5_usize as f32;
    _20 = [_8,_8,_8,_19,_19,_19,_19];
    _14 = [_27,_17,_17,_17,_17,_17];

    _9 = _14;
    _30 = [1851276586247578748_usize,14956850829093726079_usize,6_usize];
    _33.fld0 = (_7,);
    _15 = [_17,_27,_25,_27,_17,_27];
    _25 = (-5112_i16) as isize;
    _33.fld0.0 = _7 + _24.0;
    _23 = [_8,_19,_19,_19,_19,_8,_8];
    _14 = _3;
    _34 = _10.0 + _33.fld0.0;

    _2 = _12;
    _8 = _19;

    _38 = _4;
    _30 = _6;
    _39 = 923209211668991756_u64 as f64;
    _27 = _25 | _17;
    _16 = [_27,_27,_27,_17,_17,_17];

    _42.0 = !true;
    RET = (-687246327_i32) as u16;
    _33.fld0 = _24;
    RET = _33.fld3;
    _10.0 = 215_u8 as f32;

    _21 = [_38,_38,_38,_4];
    _23 = [_8,_19,_38,_8,_4,_19,_38];
    _42.1 = -1549505805_i32;
    _22 = [_28,_28,_28,_28,_28];
    _43 = [_8,_19,_4,_4,_38,_8,_19,_19];
    _12 = [_17,_27,_25,_17,_27,_17];
    _36 = 5925437046627578812_i64;
    _39 = -_29;

    _28 = (-123_i8) * (-104_i8);
    RET = _33.fld3;
    _24.0 = -f32::NAN;

    let tmp = -f32::NAN;
    _45 = tmp >= _24.0;
    _16 = _1;
    Goto(bb20)
    }
    bb20 = {
    Call(_50 = dump_var(16_usize, 45_usize, Move(_45), 34_usize, Move(_34), 24_usize, Move(_34), 50_usize, Move(_50)), ReturnTo(bb21), UnwindUnreachable())
    }
    bb21 = {

    Return()
    }

    }
}
impl PrintFDebug for Adt45 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt45{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub struct Adt45 {
    fld0: (f32,),
    fld1: *const i128,
    fld2: *mut *const *mut [i8; 5],
    fld3: u16,
    fld4: *const [char; 7],
}
