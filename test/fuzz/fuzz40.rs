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
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(
    mut _1: bool,
    mut _2: char,
    mut _3: isize,
    mut _4: i8,
    mut _5: i16,
    mut _6: i32,
    mut _7: i64,
    mut _8: i128,
    mut _9: usize,
    mut _10: u8,
    mut _11: u16,
    mut _12: u32,
    mut _13: u64,
    mut _14: u128,
) -> char {
    mir! {
    type RET = char;
    let _15: f64;
    let _16: isize;
    let _17: bool;
    let _18: [u16; 1];
    let _19: u128;
    let _20: (usize, i16);
    let _21: ([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2]);
    let _22: u16;
    let _23: isize;
    let _24: [u32; 1];
    let _25: char;
    let _26: Adt49;
    let _27: *mut char;
    let _28: [i8; 4];
    let _29: i8;
    let _30: f64;
    let _31: [i8; 4];
    let _32: i8;
    let _33: bool;
    let _34: i16;
    let _35: char;
    let _36: *mut [usize; 3];
    let _37: isize;
    let _38: ();
    let _39: ();
    {
    _14 = 15540701945132180763520386292698418847_u128;
    _4 = (-119_i8);
    _4 = 12403294203989809041_usize as i8;
    _11 = 9223372036854775807_isize as u16;
    _8 = 124831002390335584661064896542571465323_i128 >> _14;
    RET = '\u{cefde}';
    _2 = RET;
    _4 = (-27_i8) * (-102_i8);
    _13 = false as u64;
    _6 = (-1764731621_i32) << _11;
    _1 = false;
    _10 = 152_u8;
    _12 = _1 as u32;
    _4 = RET as i8;
    _16 = -(-9223372036854775808_isize);
    _4 = _13 as i8;
    _2 = RET;
    _15 = _14 as f64;
    _7 = 990885744023306610_i64 * (-1022997841955059729_i64);
    Call(_13 = fn1(_8, _14, _10, RET, _10, _16, _12, _7, _10, _8, RET, _2, _11, _7, _8), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _9 = !1_usize;
    Goto(bb2)
    }
    bb2 = {
    RET = _2;
    _1 = false ^ true;
    RET = _2;
    _1 = true;
    RET = _2;
    _15 = _10 as f64;
    _6 = 703662390_i32;
    _15 = _6 as f64;
    _5 = 25320_i16;
    _1 = _6 < _6;
    _17 = !_1;
    _3 = !_16;
    _1 = !_17;
    _8 = 137570056830126694766299120506923598124_i128;
    _9 = 16148078843379474481_usize << _3;
    _8 = 74923269481472097721674156737945320573_i128 * (-1427998670707178727257177390793037678_i128);
    _4 = (-32_i8);
    _18 = [_11];
    _14 = 1027704236157957251101696866304004778_u128 >> _11;
    _5 = -(-30677_i16);
    _1 = _17 | _17;
    RET = _2;
    _10 = 113_u8 * 205_u8;
    RET = _2;
    RET = _2;
    _18 = [_11];
    _19 = _14 & _14;
    _20.1 = _5 + _5;
    _2 = RET;
    Goto(bb3)
    }
    bb3 = {
    _7 = _16 as i64;
    _3 = _16 & _16;
    _16 = _3 ^ _3;
    _8 = 146527812405356881671437695463125180634_i128;
    _19 = _14 >> _11;
    _20.1 = _5 | _5;
    _20.0 = !_9;
    _5 = !_20.1;
    _21.0 = [_12,_12,_12,_12,_12,_12];
    _9 = !_20.0;
    _9 = _16 as usize;
    _21.1.0 = _4;
    _6 = (-567906856_i32) | 1762876016_i32;
    _21.4 = [_13,_13];
    RET = _2;
    _17 = _1 ^ _1;
    _8 = (-150892302329386502490657709812070314246_i128);
    Goto(bb4)
    }
    bb4 = {
    _20.0 = _9 + _9;
    _21.1.1 = _1 as isize;
    _21.1.1 = !_16;
    _20.1 = -_5;
    _10 = !153_u8;
    RET = _2;
    _22 = _11 - _11;
    _21.4 = [_13,_13];
    _21.1.2 = [_6,_6];
    _21.1.2 = [_6,_6];
    _23 = _17 as isize;
    RET = _2;
    _21.3 = [_20.1,_5,_5,_5,_5,_20.1];
    _21.1.0 = _15 as i8;
    _5 = RET as i16;
    _21.4 = [_13,_13];
    RET = _2;
    _17 = _1;
    _3 = _21.1.1 >> _16;
    _24 = [_12];
    _25 = RET;
    _22 = _11;
    RET = _2;
    _21.3 = [_5,_20.1,_20.1,_20.1,_20.1,_5];
    _18 = [_11];
    match _4 {
    0 => bb3,
    1 => bb2,
    2 => bb5,
    3 => bb6,
    4 => bb7,
    5 => bb8,
    340282366920938463463374607431768211424 => bb10,
    _ => bb9
    }
    }
    bb5 = {
    _7 = _16 as i64;
    _3 = _16 & _16;
    _16 = _3 ^ _3;
    _8 = 146527812405356881671437695463125180634_i128;
    _19 = _14 >> _11;
    _20.1 = _5 | _5;
    _20.0 = !_9;
    _5 = !_20.1;
    _21.0 = [_12,_12,_12,_12,_12,_12];
    _9 = !_20.0;
    _9 = _16 as usize;
    _21.1.0 = _4;
    _6 = (-567906856_i32) | 1762876016_i32;
    _21.4 = [_13,_13];
    RET = _2;
    _17 = _1 ^ _1;
    _8 = (-150892302329386502490657709812070314246_i128);
    Goto(bb4)
    }
    bb6 = {
    RET = _2;
    _1 = false ^ true;
    RET = _2;
    _1 = true;
    RET = _2;
    _15 = _10 as f64;
    _6 = 703662390_i32;
    _15 = _6 as f64;
    _5 = 25320_i16;
    _1 = _6 < _6;
    _17 = !_1;
    _3 = !_16;
    _1 = !_17;
    _8 = 137570056830126694766299120506923598124_i128;
    _9 = 16148078843379474481_usize << _3;
    _8 = 74923269481472097721674156737945320573_i128 * (-1427998670707178727257177390793037678_i128);
    _4 = (-32_i8);
    _18 = [_11];
    _14 = 1027704236157957251101696866304004778_u128 >> _11;
    _5 = -(-30677_i16);
    _1 = _17 | _17;
    RET = _2;
    _10 = 113_u8 * 205_u8;
    RET = _2;
    RET = _2;
    _18 = [_11];
    _19 = _14 & _14;
    _20.1 = _5 + _5;
    _2 = RET;
    Goto(bb3)
    }
    bb7 = {
    _9 = !1_usize;
    Goto(bb2)
    }
    bb8 = {
    Return()
    }
    bb9 = {
    Return()
    }
    bb10 = {
    _18 = [_11];
    _23 = _16 - _3;
    _18 = [_22];
    _28 = [_21.1.0,_4,_4,_4];
    RET = _25;
    RET = _25;
    _24 = [_12];
    _29 = _21.1.0 ^ _21.1.0;
    _18 = [_22];
    _24 = [_12];
    _14 = _19;
    _19 = _25 as u128;
    _21.3 = [_20.1,_20.1,_20.1,_20.1,_5,_20.1];
    _20.1 = _5;
    _21.1.0 = _29;
    _30 = -_15;
    _1 = _20.0 <= _20.0;
    _22 = _11;
    RET = _25;
    Goto(bb11)
    }
    bb11 = {
    _25 = RET;
    _12 = 1941435211_u32;
    _3 = _20.1 as isize;
    RET = _25;
    _19 = !_14;
    _16 = _23 + _21.1.1;
    _21.3 = [_20.1,_5,_20.1,_5,_20.1,_20.1];
    _13 = 15570787547221982195_u64;
    _10 = _30 as u8;
    _16 = !_21.1.1;
    _13 = 12314583507640013199_u64 + 1415807745869619490_u64;
    RET = _25;
    _21.4 = [_13,_13];
    _8 = !(-50075874878524572647218834319393871449_i128);
    _18 = [_11];
    _5 = _20.1 ^ _20.1;
    _29 = !_4;
    _13 = 2447027841316352743_u64;
    _17 = !_1;
    _31 = _28;
    _23 = _8 as isize;
    _16 = -_21.1.1;
    _2 = _25;
    _34 = _21.1.1 as i16;
    _15 = _12 as f64;
    _20.0 = _9 & _9;
    _34 = _5;
    _33 = !_17;
    _11 = _22;
    match _12 {
    0 => bb12,
    1 => bb13,
    2 => bb14,
    3 => bb15,
    4 => bb16,
    5 => bb17,
    6 => bb18,
    1941435211 => bb20,
    _ => bb19
    }
    }
    bb12 = {
    _18 = [_11];
    _23 = _16 - _3;
    _18 = [_22];
    _28 = [_21.1.0,_4,_4,_4];
    RET = _25;
    RET = _25;
    _24 = [_12];
    _29 = _21.1.0 ^ _21.1.0;
    _18 = [_22];
    _24 = [_12];
    _14 = _19;
    _19 = _25 as u128;
    _21.3 = [_20.1,_20.1,_20.1,_20.1,_5,_20.1];
    _20.1 = _5;
    _21.1.0 = _29;
    _30 = -_15;
    _1 = _20.0 <= _20.0;
    _22 = _11;
    RET = _25;
    Goto(bb11)
    }
    bb13 = {
    Return()
    }
    bb14 = {
    _9 = !1_usize;
    Goto(bb2)
    }
    bb15 = {
    _9 = !1_usize;
    Goto(bb2)
    }
    bb16 = {
    RET = _2;
    _1 = false ^ true;
    RET = _2;
    _1 = true;
    RET = _2;
    _15 = _10 as f64;
    _6 = 703662390_i32;
    _15 = _6 as f64;
    _5 = 25320_i16;
    _1 = _6 < _6;
    _17 = !_1;
    _3 = !_16;
    _1 = !_17;
    _8 = 137570056830126694766299120506923598124_i128;
    _9 = 16148078843379474481_usize << _3;
    _8 = 74923269481472097721674156737945320573_i128 * (-1427998670707178727257177390793037678_i128);
    _4 = (-32_i8);
    _18 = [_11];
    _14 = 1027704236157957251101696866304004778_u128 >> _11;
    _5 = -(-30677_i16);
    _1 = _17 | _17;
    RET = _2;
    _10 = 113_u8 * 205_u8;
    RET = _2;
    RET = _2;
    _18 = [_11];
    _19 = _14 & _14;
    _20.1 = _5 + _5;
    _2 = RET;
    Goto(bb3)
    }
    bb17 = {
    RET = _2;
    _1 = false ^ true;
    RET = _2;
    _1 = true;
    RET = _2;
    _15 = _10 as f64;
    _6 = 703662390_i32;
    _15 = _6 as f64;
    _5 = 25320_i16;
    _1 = _6 < _6;
    _17 = !_1;
    _3 = !_16;
    _1 = !_17;
    _8 = 137570056830126694766299120506923598124_i128;
    _9 = 16148078843379474481_usize << _3;
    _8 = 74923269481472097721674156737945320573_i128 * (-1427998670707178727257177390793037678_i128);
    _4 = (-32_i8);
    _18 = [_11];
    _14 = 1027704236157957251101696866304004778_u128 >> _11;
    _5 = -(-30677_i16);
    _1 = _17 | _17;
    RET = _2;
    _10 = 113_u8 * 205_u8;
    RET = _2;
    RET = _2;
    _18 = [_11];
    _19 = _14 & _14;
    _20.1 = _5 + _5;
    _2 = RET;
    Goto(bb3)
    }
    bb18 = {
    _20.0 = _9 + _9;
    _21.1.1 = _1 as isize;
    _21.1.1 = !_16;
    _20.1 = -_5;
    _10 = !153_u8;
    RET = _2;
    _22 = _11 - _11;
    _21.4 = [_13,_13];
    _21.1.2 = [_6,_6];
    _21.1.2 = [_6,_6];
    _23 = _17 as isize;
    RET = _2;
    _21.3 = [_20.1,_5,_5,_5,_5,_20.1];
    _21.1.0 = _15 as i8;
    _5 = RET as i16;
    _21.4 = [_13,_13];
    RET = _2;
    _17 = _1;
    _3 = _21.1.1 >> _16;
    _24 = [_12];
    _25 = RET;
    _22 = _11;
    RET = _2;
    _21.3 = [_5,_20.1,_20.1,_20.1,_20.1,_5];
    _18 = [_11];
    match _4 {
    0 => bb3,
    1 => bb2,
    2 => bb5,
    3 => bb6,
    4 => bb7,
    5 => bb8,
    340282366920938463463374607431768211424 => bb10,
    _ => bb9
    }
    }
    bb19 = {
    _7 = _16 as i64;
    _3 = _16 & _16;
    _16 = _3 ^ _3;
    _8 = 146527812405356881671437695463125180634_i128;
    _19 = _14 >> _11;
    _20.1 = _5 | _5;
    _20.0 = !_9;
    _5 = !_20.1;
    _21.0 = [_12,_12,_12,_12,_12,_12];
    _9 = !_20.0;
    _9 = _16 as usize;
    _21.1.0 = _4;
    _6 = (-567906856_i32) | 1762876016_i32;
    _21.4 = [_13,_13];
    RET = _2;
    _17 = _1 ^ _1;
    _8 = (-150892302329386502490657709812070314246_i128);
    Goto(bb4)
    }
    bb20 = {
    _6 = _4 as i32;
    RET = _2;
    _1 = _33 > _33;
    _11 = !_22;
    _16 = !_3;
    _30 = _15 - _15;
    _17 = !_1;
    _18 = [_22];
    _6 = (-1847047986_i32) + (-1284504990_i32);
    Goto(bb21)
    }
    bb21 = {
    Call(_38 = dump_var(0_usize, 23_usize, Move(_23), 24_usize, Move(_24), 12_usize, Move(_12), 33_usize, Move(_33)), ReturnTo(bb22), UnwindUnreachable())
    }
    bb22 = {
    Call(_38 = dump_var(0_usize, 29_usize, Move(_29), 14_usize, Move(_14), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb23), UnwindUnreachable())
    }
    bb23 = {
    Call(_38 = dump_var(0_usize, 8_usize, Move(_8), 16_usize, Move(_16), 34_usize, Move(_34), 4_usize, Move(_4)), ReturnTo(bb24), UnwindUnreachable())
    }
    bb24 = {
    Call(_38 = dump_var(0_usize, 19_usize, Move(_19), 3_usize, Move(_3), 39_usize, _39, 39_usize, _39), ReturnTo(bb25), UnwindUnreachable())
    }
    bb25 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(
    mut _1: i128,
    mut _2: u128,
    mut _3: u8,
    mut _4: char,
    mut _5: u8,
    mut _6: isize,
    mut _7: u32,
    mut _8: i64,
    mut _9: u8,
    mut _10: i128,
    mut _11: char,
    mut _12: char,
    mut _13: u16,
    mut _14: i64,
    mut _15: i128,
) -> u64 {
    mir! {
    type RET = u64;
    let _16: char;
    let _17: [u32; 1];
    let _18: Adt58;
    let _19: [i16; 6];
    let _20: [i32; 2];
    let _21: usize;
    let _22: bool;
    let _23: i8;
    let _24: i8;
    let _25: [i16; 6];
    let _26: bool;
    let _27: u64;
    let _28: [u64; 6];
    let _29: [char; 7];
    let _30: Adt48;
    let _31: u128;
    let _32: [u32; 6];
    let _33: u128;
    let _34: ();
    let _35: ();
    {
    _1 = _15 ^ _10;
    Call(_15 = fn2(_8, _12, _3, _13, _11, _3, _1, _1, _4), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _11 = _4;
    _9 = !_5;
    _11 = _12;
    _14 = _8;
    _2 = 16382_i16 as u128;
    _4 = _12;
    RET = _4 as u64;
    _3 = _5;
    RET = 17141053976243200912_u64;
    _4 = _12;
    _9 = _3;
    _4 = _12;
    _4 = _12;
    _2 = !251818735733616076137568076645258222327_u128;
    _6 = _7 as isize;
    _7 = 768696295_u32;
    _13 = 51901_u16;
    _9 = !_5;
    _19 = [(-26240_i16),(-24535_i16),(-21404_i16),(-32425_i16),(-13552_i16),1672_i16];
    _14 = -_8;
    Goto(bb2)
    }
    bb2 = {
    _5 = _9 % _3;
    _8 = -_14;
    RET = 2_usize as u64;
    RET = _13 as u64;
    _10 = _15 >> _15;
    _5 = _9;
    Goto(bb3)
    }
    bb3 = {
    _19 = [(-16410_i16),4562_i16,31597_i16,(-2695_i16),30554_i16,13990_i16];
    _16 = _4;
    _7 = 3388351778153402086_usize as u32;
    _12 = _11;
    _5 = RET as u8;
    _2 = _6 as u128;
    _17 = [_7];
    _17 = [_7];
    _1 = _10 - _15;
    _5 = _3 ^ _3;
    _5 = _3;
    _15 = -_10;
    _15 = _1;
    _4 = _11;
    _22 = true ^ false;
    _13 = 46028_u16;
    _2 = !3834323706563861448515307835601011386_u128;
    _4 = _12;
    _20 = [1374513825_i32,696714401_i32];
    _2 = 198018724077699114435610956095881637428_u128;
    _12 = _16;
    _4 = _16;
    match _3 {
    152 => bb5,
    _ => bb4
    }
    }
    bb4 = {
    _5 = _9 % _3;
    _8 = -_14;
    RET = 2_usize as u64;
    RET = _13 as u64;
    _10 = _15 >> _15;
    _5 = _9;
    Goto(bb3)
    }
    bb5 = {
    _6 = 2_usize as isize;
    _3 = !_5;
    _19 = [23498_i16,10648_i16,8949_i16,20984_i16,30645_i16,2167_i16];
    RET = 16670878310020939789_usize as u64;
    _19 = [32189_i16,(-11093_i16),(-4936_i16),(-26764_i16),15707_i16,17870_i16];
    _20 = [1010793535_i32,1657266972_i32];
    _24 = -29_i8;
    _1 = !_15;
    _21 = 0_usize >> _15;
    _2 = 17075443224423905852900209460491209038_u128;
    _19 = [(-12152_i16),32623_i16,(-20709_i16),21863_i16,31558_i16,(-24753_i16)];
    _11 = _12;
    _8 = _14 + _14;
    _4 = _16;
    _3 = _5;
    _21 = _1 as usize;
    _12 = _11;
    _26 = _22;
    _22 = _26;
    _21 = 25938_i16 as usize;
    _15 = _10 >> _1;
    _25 = _19;
    _21 = 5_usize + 3828000844807470376_usize;
    _20 = [347270078_i32,527041850_i32];
    _22 = _26;
    _14 = _8;
    match _5 {
    0 => bb1,
    1 => bb2,
    2 => bb4,
    152 => bb7,
    _ => bb6
    }
    }
    bb6 = {
    _5 = _9 % _3;
    _8 = -_14;
    RET = 2_usize as u64;
    RET = _13 as u64;
    _10 = _15 >> _15;
    _5 = _9;
    Goto(bb3)
    }
    bb7 = {
    _21 = !3_usize;
    _8 = -_14;
    _28 = [RET,RET,RET,RET,RET,RET];
    _27 = RET;
    _4 = _16;
    _7 = !510050983_u32;
    _13 = 22754_u16;
    _30 = Adt48 { fld0: _2 };
    _5 = !_3;
    _9 = _16 as u8;
    _9 = !_3;
    _28 = [RET,RET,_27,_27,RET,_27];
    _14 = _8;
    RET = _27;
    _11 = _4;
    _2 = _30.fld0;
    _20 = [1776258687_i32,(-1281725741_i32)];
    _23 = _24;
    _26 = !_22;
    _2 = _21 as u128;
    _9 = _5;
    _22 = !_26;
    _31 = _2;
    _7 = 689374596_i32 as u32;
    _10 = _1;
    _19 = [1929_i16,1450_i16,(-27055_i16),(-10693_i16),30200_i16,(-20457_i16)];
    match _30.fld0 {
    0 => bb8,
    1 => bb9,
    2 => bb10,
    3 => bb11,
    4 => bb12,
    5 => bb13,
    6 => bb14,
    17075443224423905852900209460491209038 => bb16,
    _ => bb15
    }
    }
    bb8 = {
    _5 = _9 % _3;
    _8 = -_14;
    RET = 2_usize as u64;
    RET = _13 as u64;
    _10 = _15 >> _15;
    _5 = _9;
    Goto(bb3)
    }
    bb9 = {
    _6 = 2_usize as isize;
    _3 = !_5;
    _19 = [23498_i16,10648_i16,8949_i16,20984_i16,30645_i16,2167_i16];
    RET = 16670878310020939789_usize as u64;
    _19 = [32189_i16,(-11093_i16),(-4936_i16),(-26764_i16),15707_i16,17870_i16];
    _20 = [1010793535_i32,1657266972_i32];
    _24 = -29_i8;
    _1 = !_15;
    _21 = 0_usize >> _15;
    _2 = 17075443224423905852900209460491209038_u128;
    _19 = [(-12152_i16),32623_i16,(-20709_i16),21863_i16,31558_i16,(-24753_i16)];
    _11 = _12;
    _8 = _14 + _14;
    _4 = _16;
    _3 = _5;
    _21 = _1 as usize;
    _12 = _11;
    _26 = _22;
    _22 = _26;
    _21 = 25938_i16 as usize;
    _15 = _10 >> _1;
    _25 = _19;
    _21 = 5_usize + 3828000844807470376_usize;
    _20 = [347270078_i32,527041850_i32];
    _22 = _26;
    _14 = _8;
    match _5 {
    0 => bb1,
    1 => bb2,
    2 => bb4,
    152 => bb7,
    _ => bb6
    }
    }
    bb10 = {
    _5 = _9 % _3;
    _8 = -_14;
    RET = 2_usize as u64;
    RET = _13 as u64;
    _10 = _15 >> _15;
    _5 = _9;
    Goto(bb3)
    }
    bb11 = {
    _19 = [(-16410_i16),4562_i16,31597_i16,(-2695_i16),30554_i16,13990_i16];
    _16 = _4;
    _7 = 3388351778153402086_usize as u32;
    _12 = _11;
    _5 = RET as u8;
    _2 = _6 as u128;
    _17 = [_7];
    _17 = [_7];
    _1 = _10 - _15;
    _5 = _3 ^ _3;
    _5 = _3;
    _15 = -_10;
    _15 = _1;
    _4 = _11;
    _22 = true ^ false;
    _13 = 46028_u16;
    _2 = !3834323706563861448515307835601011386_u128;
    _4 = _12;
    _20 = [1374513825_i32,696714401_i32];
    _2 = 198018724077699114435610956095881637428_u128;
    _12 = _16;
    _4 = _16;
    match _3 {
    152 => bb5,
    _ => bb4
    }
    }
    bb12 = {
    _5 = _9 % _3;
    _8 = -_14;
    RET = 2_usize as u64;
    RET = _13 as u64;
    _10 = _15 >> _15;
    _5 = _9;
    Goto(bb3)
    }
    bb13 = {
    _11 = _4;
    _9 = !_5;
    _11 = _12;
    _14 = _8;
    _2 = 16382_i16 as u128;
    _4 = _12;
    RET = _4 as u64;
    _3 = _5;
    RET = 17141053976243200912_u64;
    _4 = _12;
    _9 = _3;
    _4 = _12;
    _4 = _12;
    _2 = !251818735733616076137568076645258222327_u128;
    _6 = _7 as isize;
    _7 = 768696295_u32;
    _13 = 51901_u16;
    _9 = !_5;
    _19 = [(-26240_i16),(-24535_i16),(-21404_i16),(-32425_i16),(-13552_i16),1672_i16];
    _14 = -_8;
    Goto(bb2)
    }
    bb14 = {
    Return()
    }
    bb15 = {
    Return()
    }
    bb16 = {
    _33 = !_30.fld0;
    _28 = [RET,_27,_27,RET,RET,RET];
    _26 = _22 & _22;
    Goto(bb17)
    }
    bb17 = {
    Call(_34 = dump_var(1_usize, 33_usize, Move(_33), 7_usize, Move(_7), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Call(_34 = dump_var(1_usize, 20_usize, Move(_20), 6_usize, Move(_6), 24_usize, Move(_24), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Call(_34 = dump_var(1_usize, 31_usize, Move(_31), 27_usize, Move(_27), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
    }
    bb20 = {
    Call(_34 = dump_var(1_usize, 23_usize, Move(_23), 22_usize, Move(_22), 35_usize, _35, 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
    }
    bb21 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(
    mut _1: i64,
    mut _2: char,
    mut _3: u8,
    mut _4: u16,
    mut _5: char,
    mut _6: u8,
    mut _7: i128,
    mut _8: i128,
    mut _9: char,
) -> i128 {
    mir! {
    type RET = i128;
    let _10: f32;
    let _11: *const f32;
    let _12: f64;
    let _13: &'static u16;
    let _14: f64;
    let _15: [char; 7];
    let _16: u32;
    let _17: i32;
    let _18: Adt56;
    let _19: f32;
    let _20: *const f32;
    let _21: char;
    let _22: u128;
    let _23: ();
    let _24: ();
    {
    RET = 10477_i16 as i128;
    _7 = -_8;
    _1 = -(-8221550624463596169_i64);
    Call(_5 = fn3(_8, _8, _8, _9, _8, _7, _2), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _2 = _9;
    _5 = _2;
    _6 = _3;
    _6 = (-9223372036854775808_isize) as u8;
    _9 = _5;
    _10 = 5_usize as f32;
    _4 = !58269_u16;
    _1 = -1473414006073810620_i64;
    _11 = core::ptr::addr_of!(_10);
    (*_11) = 3140_i16 as f32;
    _8 = _7 + _7;
    _10 = 7_usize as f32;
    _9 = _2;
    _3 = _6;
    _7 = _8 >> _8;
    _12 = _1 as f64;
    _9 = _2;
    _8 = _12 as i128;
    _11 = core::ptr::addr_of!(_10);
    _10 = _12 as f32;
    _9 = _5;
    _12 = 6842_i16 as f64;
    RET = _1 as i128;
    RET = _3 as i128;
    Goto(bb2)
    }
    bb2 = {
    _5 = _2;
    Call((*_11) = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
    }
    bb3 = {
    _12 = 1206734964_u32 as f64;
    _10 = (-84_i8) as f32;
    _5 = _9;
    _7 = RET ^ RET;
    _11 = core::ptr::addr_of!((*_11));
    _11 = core::ptr::addr_of!((*_11));
    _13 = &_4;
    _5 = _9;
    RET = _7;
    _9 = _2;
    _6 = _3;
    (*_11) = 303229151764163213401125116567626245652_u128 as f32;
    _13 = &_4;
    (*_11) = (*_13) as f32;
    _6 = _3 >> (*_13);
    (*_11) = _7 as f32;
    _3 = !_6;
    _12 = 18240625041671567596_usize as f64;
    _13 = &(*_13);
    _8 = RET >> (*_13);
    _2 = _5;
    (*_11) = _7 as f32;
    _2 = _9;
    RET = _7 << _7;
    RET = _12 as i128;
    Goto(bb4)
    }
    bb4 = {
    _3 = _6 + _6;
    _12 = 2245999255_u32 as f64;
    _15 = [_5,_2,_5,_2,_2,_5,_2];
    RET = _8 << _8;
    _8 = -_7;
    _5 = _2;
    RET = _7 << _8;
    _5 = _2;
    _16 = 0_usize as u32;
    _16 = !2337077479_u32;
    _11 = core::ptr::addr_of!((*_11));
    Goto(bb5)
    }
    bb5 = {
    _11 = core::ptr::addr_of!((*_11));
    _9 = _5;
    _2 = _9;
    _7 = !RET;
    (*_11) = 4376451369490650_usize as f32;
    _17 = (-1742424140_i32) >> RET;
    _4 = !26761_u16;
    (*_11) = 9223372036854775807_isize as f32;
    _9 = _2;
    _9 = _2;
    _2 = _9;
    _12 = 128758534574032610167744525323042338719_u128 as f64;
    Goto(bb6)
    }
    bb6 = {
    RET = 14972185993002165805_usize as i128;
    _6 = !_3;
    _13 = &_4;
    _8 = _7 << _7;
    _12 = 4_usize as f64;
    RET = (-10943_i16) as i128;
    _17 = 1204255088_i32;
    _14 = _12 + _12;
    _20 = _11;
    _5 = _2;
    RET = _8 & _8;
    _22 = !339732327058290431906056677625909644195_u128;
    _16 = !1511187878_u32;
    _6 = _3;
    Goto(bb7)
    }
    bb7 = {
    Call(_23 = dump_var(2_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb8), UnwindUnreachable())
    }
    bb8 = {
    Call(_23 = dump_var(2_usize, 15_usize, Move(_15), 2_usize, Move(_2), 24_usize, _24, 24_usize, _24), ReturnTo(bb9), UnwindUnreachable())
    }
    bb9 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(
    mut _1: i128,
    mut _2: i128,
    mut _3: i128,
    mut _4: char,
    mut _5: i128,
    mut _6: i128,
    mut _7: char,
) -> char {
    mir! {
    type RET = char;
    let _8: f64;
    let _9: Adt55;
    let _10: i128;
    let _11: (i8, isize, [i32; 2]);
    let _12: Adt49;
    let _13: [i8; 4];
    let _14: f64;
    let _15: u128;
    let _16: Adt49;
    let _17: Adt61;
    let _18: Adt54;
    let _19: isize;
    let _20: isize;
    let _21: f64;
    let _22: i16;
    let _23: (&'static u16, &'static u16, i64, (u64,));
    let _24: i128;
    let _25: Adt52;
    let _26: isize;
    let _27: isize;
    let _28: isize;
    let _29: isize;
    let _30: ();
    let _31: ();
    {
    _4 = _7;
    _5 = _3 >> _3;
    _8 = 12744857079585336045_u64 as f64;
    _3 = _5 << _1;
    _5 = _6;
    RET = _4;
    RET = _4;
    _7 = RET;
    _8 = 2_usize as f64;
    _6 = !_2;
    _11.0 = (-116_i8) >> _2;
    _5 = _6 - _3;
    _5 = _3;
    _10 = _5;
    _8 = (-9223372036854775808_isize) as f64;
    _7 = _4;
    _6 = _5 ^ _2;
    _11.1 = 9223372036854775807_isize;
    _3 = _1;
    _2 = _8 as i128;
    _8 = 1670869088_u32 as f64;
    _3 = 245520150062387072868421983975989307874_u128 as i128;
    _11.2 = [(-341902157_i32),(-406481566_i32)];
    _11.2 = [601297413_i32,92283047_i32];
    Goto(bb1)
    }
    bb1 = {
    _5 = true as i128;
    _11.0 = _4 as i8;
    _3 = _10 * _6;
    _5 = _3 << _10;
    _15 = 165484128065225364393910054624991150915_u128 + 144385349198335903191137432040137710205_u128;
    _15 = !213248491750386472698782748076652754081_u128;
    _11.2 = [1413429052_i32,(-1120007192_i32)];
    Goto(bb2)
    }
    bb2 = {
    _11.2 = [(-1313811_i32),(-524715823_i32)];
    _11.2 = [(-798815786_i32),514727120_i32];
    _14 = -_8;
    _5 = _6 << _3;
    _4 = _7;
    _5 = -_2;
    RET = _4;
    RET = _4;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    RET = _4;
    _15 = !281952905726534246166958927750081308951_u128;
    _15 = 3068_u16 as u128;
    _4 = RET;
    _6 = _1;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    _10 = 3474425937183653945_u64 as i128;
    Call(_11.2 = fn4(_3, _3, _1), ReturnTo(bb3), UnwindUnreachable())
    }
    bb3 = {
    _10 = !_3;
    RET = _4;
    _11.1 = false as isize;
    RET = _7;
    _14 = _8;
    RET = _4;
    _6 = _3;
    _1 = -_6;
    _11.0 = (-106_i8) | 2_i8;
    _5 = !_6;
    _11.0 = 20_i8;
    _5 = _1 * _10;
    RET = _7;
    _3 = !_1;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    _11.1 = 118_isize;
    _1 = (-143881149_i32) as i128;
    _19 = _8 as isize;
    _15 = 121734192847483010178524099200054824497_u128 | 132490806770013464589782726459154496245_u128;
    _5 = _6 - _2;
    RET = _4;
    _10 = _3;
    _11.1 = _19 | _19;
    _19 = _11.1 | _11.1;
    _11.2 = [(-1237115459_i32),(-1053711276_i32)];
    _2 = 43616_u16 as i128;
    RET = _4;
    match _11.0 {
    0 => bb1,
    1 => bb2,
    2 => bb4,
    3 => bb5,
    4 => bb6,
    5 => bb7,
    6 => bb8,
    20 => bb10,
    _ => bb9
    }
    }
    bb4 = {
    _11.2 = [(-1313811_i32),(-524715823_i32)];
    _11.2 = [(-798815786_i32),514727120_i32];
    _14 = -_8;
    _5 = _6 << _3;
    _4 = _7;
    _5 = -_2;
    RET = _4;
    RET = _4;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    RET = _4;
    _15 = !281952905726534246166958927750081308951_u128;
    _15 = 3068_u16 as u128;
    _4 = RET;
    _6 = _1;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    _10 = 3474425937183653945_u64 as i128;
    Call(_11.2 = fn4(_3, _3, _1), ReturnTo(bb3), UnwindUnreachable())
    }
    bb5 = {
    _5 = true as i128;
    _11.0 = _4 as i8;
    _3 = _10 * _6;
    _5 = _3 << _10;
    _15 = 165484128065225364393910054624991150915_u128 + 144385349198335903191137432040137710205_u128;
    _15 = !213248491750386472698782748076652754081_u128;
    _11.2 = [1413429052_i32,(-1120007192_i32)];
    Goto(bb2)
    }
    bb6 = {
    Return()
    }
    bb7 = {
    Return()
    }
    bb8 = {
    Return()
    }
    bb9 = {
    Return()
    }
    bb10 = {
    _23.2 = 7971020981448906133_i64 ^ (-7439357878710643105_i64);
    _11.0 = (-59_i8) | 2_i8;
    _23.3 = (12873723809863651933_u64,);
    Call(_11.0 = core::intrinsics::bswap(57_i8), ReturnTo(bb11), UnwindUnreachable())
    }
    bb11 = {
    _22 = (-16521_i16);
    RET = _4;
    _8 = -_14;
    Goto(bb12)
    }
    bb12 = {
    _11.2 = [841961563_i32,(-861543118_i32)];
    match _22 {
    0 => bb11,
    1 => bb13,
    2 => bb14,
    3 => bb15,
    4 => bb16,
    340282366920938463463374607431768194935 => bb18,
    _ => bb17
    }
    }
    bb13 = {
    _22 = (-16521_i16);
    RET = _4;
    _8 = -_14;
    Goto(bb12)
    }
    bb14 = {
    Return()
    }
    bb15 = {
    _10 = !_3;
    RET = _4;
    _11.1 = false as isize;
    RET = _7;
    _14 = _8;
    RET = _4;
    _6 = _3;
    _1 = -_6;
    _11.0 = (-106_i8) | 2_i8;
    _5 = !_6;
    _11.0 = 20_i8;
    _5 = _1 * _10;
    RET = _7;
    _3 = !_1;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    _11.1 = 118_isize;
    _1 = (-143881149_i32) as i128;
    _19 = _8 as isize;
    _15 = 121734192847483010178524099200054824497_u128 | 132490806770013464589782726459154496245_u128;
    _5 = _6 - _2;
    RET = _4;
    _10 = _3;
    _11.1 = _19 | _19;
    _19 = _11.1 | _11.1;
    _11.2 = [(-1237115459_i32),(-1053711276_i32)];
    _2 = 43616_u16 as i128;
    RET = _4;
    match _11.0 {
    0 => bb1,
    1 => bb2,
    2 => bb4,
    3 => bb5,
    4 => bb6,
    5 => bb7,
    6 => bb8,
    20 => bb10,
    _ => bb9
    }
    }
    bb16 = {
    Return()
    }
    bb17 = {
    _11.2 = [(-1313811_i32),(-524715823_i32)];
    _11.2 = [(-798815786_i32),514727120_i32];
    _14 = -_8;
    _5 = _6 << _3;
    _4 = _7;
    _5 = -_2;
    RET = _4;
    RET = _4;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    RET = _4;
    _15 = !281952905726534246166958927750081308951_u128;
    _15 = 3068_u16 as u128;
    _4 = RET;
    _6 = _1;
    _13 = [_11.0,_11.0,_11.0,_11.0];
    _10 = 3474425937183653945_u64 as i128;
    Call(_11.2 = fn4(_3, _3, _1), ReturnTo(bb3), UnwindUnreachable())
    }
    bb18 = {
    _27 = _23.2 as isize;
    _11.0 = 86_i8;
    _23.3 = (11927816265128853579_u64,);
    _15 = 275781802793224854357929196921779982793_u128;
    Goto(bb19)
    }
    bb19 = {
    Call(_30 = dump_var(3_usize, 1_usize, Move(_1), 3_usize, Move(_3), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
    }
    bb20 = {
    Call(_30 = dump_var(3_usize, 7_usize, Move(_7), 22_usize, Move(_22), 13_usize, Move(_13), 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
    }
    bb21 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i128, mut _2: i128, mut _3: i128) -> [i32; 2] {
    mir! {
    type RET = [i32; 2];
    let _4: usize;
    let _5: [u16; 1];
    let _6: *mut (i8, isize, [i32; 2]);
    let _7: char;
    let _8: [i32; 2];
    let _9: f64;
    let _10: f32;
    let _11: [u64; 2];
    let _12: f32;
    let _13: f64;
    let _14: bool;
    let _15: u128;
    let _16: Adt49;
    let _17: (u64,);
    let _18: (u64,);
    let _19: (&'static u16, &'static u16, i64, (u64,));
    let _20: Adt51;
    let _21: (usize, i16);
    let _22: (u64,);
    let _23: [isize; 3];
    let _24: bool;
    let _25: [u16; 1];
    let _26: Adt59;
    let _27: usize;
    let _28: f32;
    let _29: Adt48;
    let _30: Adt58;
    let _31: isize;
    let _32: isize;
    let _33: [u32; 1];
    let _34: isize;
    let _35: bool;
    let _36: Adt57;
    let _37: [isize; 3];
    let _38: f32;
    let _39: [i8; 4];
    let _40: [i8; 4];
    let _41: [u64; 6];
    let _42: (u64,);
    let _43: ();
    let _44: ();
    {
    RET = [(-1578259229_i32),1403019478_i32];
    RET = [1133456658_i32,1298645909_i32];
    _1 = _2 << _2;
    _7 = '\u{68ac}';
    _2 = _1;
    _5 = [44596_u16];
    _3 = _1;
    _1 = !_2;
    _4 = !2_usize;
    _3 = 2021084273_u32 as i128;
    _7 = '\u{fbaca}';
    RET = [(-995379179_i32),1631131437_i32];
    Call(_7 = fn5(_1, _2, _1, _2, _1, _1, _2, RET, _2, _1), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _7 = '\u{4baec}';
    _8 = [181649679_i32,1415429396_i32];
    _7 = '\u{cdecd}';
    _5 = [33145_u16];
    _1 = !_2;
    RET = [(-2113592511_i32),1836489640_i32];
    RET = [(-819681865_i32),1985659949_i32];
    _1 = -_2;
    _9 = 87_i8 as f64;
    _4 = !2_usize;
    _8 = [886798508_i32,(-1573474752_i32)];
    _3 = -_2;
    _9 = 686035865_u32 as f64;
    RET = _8;
    _4 = !615560844117894091_usize;
    _3 = _7 as i128;
    _1 = _7 as i128;
    _1 = 130_u8 as i128;
    _11 = [8523559742123510932_u64,10201266017652900411_u64];
    RET = [1317831802_i32,1564961644_i32];
    Goto(bb2)
    }
    bb2 = {
    RET = _8;
    _5 = [17050_u16];
    _7 = '\u{88ee}';
    _3 = _2;
    _12 = 118045926483997816560205314038471851444_u128 as f32;
    _1 = !_2;
    _9 = 83_u8 as f64;
    _4 = 3_usize >> _3;
    _7 = '\u{ba6d9}';
    _5 = [22407_u16];
    _9 = 137_u8 as f64;
    _11 = [2704059505048577628_u64,9380780861980473264_u64];
    _10 = _12 + _12;
    _9 = (-7924441990320024407_i64) as f64;
    _9 = 9223372036854775807_isize as f64;
    _11 = [7082505501233434165_u64,10817738833180982587_u64];
    _12 = -_10;
    _13 = 19786_u16 as f64;
    _10 = _12;
    _7 = '\u{cb70c}';
    _15 = !298224967084030364973901531439640176351_u128;
    _9 = _13;
    Goto(bb3)
    }
    bb3 = {
    _7 = '\u{b4666}';
    _3 = _2 << _1;
    _5 = [20355_u16];
    _5 = [44012_u16];
    _3 = 3202566052324929387_i64 as i128;
    _2 = -_1;
    _3 = false as i128;
    _14 = _4 <= _4;
    _3 = -_1;
    _8 = [389376194_i32,12313538_i32];
    _4 = !11829883978217416772_usize;
    Call(_12 = core::intrinsics::transmute(_7), ReturnTo(bb4), UnwindUnreachable())
    }
    bb4 = {
    _2 = _1 + _1;
    _10 = _12 * _12;
    _7 = '\u{f0786}';
    _3 = _1;
    _11 = [1817802045285870086_u64,10084644903622342821_u64];
    _13 = _9;
    Goto(bb5)
    }
    bb5 = {
    _12 = _10;
    _17.0 = 17598590029429663213_u64;
    _11 = [_17.0,_17.0];
    RET = _8;
    RET = [(-56559964_i32),381590887_i32];
    Call(_1 = core::intrinsics::bswap(_2), ReturnTo(bb6), UnwindUnreachable())
    }
    bb6 = {
    _17 = (14266062888741506388_u64,);
    _12 = -_10;
    _17.0 = 10_i8 as u64;
    _8 = [(-1275085274_i32),(-1351780782_i32)];
    _10 = _13 as f32;
    _14 = !true;
    RET = _8;
    _17.0 = 3392410210446211971_u64 - 7666443432870755316_u64;
    RET = _8;
    _8 = [(-1677481811_i32),(-9710858_i32)];
    _1 = _3;
    _19.2 = 6400123547704241598_i64 >> _1;
    _19.3 = (_17.0,);
    RET = [(-244349551_i32),(-2120817913_i32)];
    _12 = _4 as f32;
    _14 = true;
    _19.3.0 = 111_i8 as u64;
    _7 = '\u{4e864}';
    _7 = '\u{bcc9f}';
    _5 = [24526_u16];
    _18 = (_19.3.0,);
    Call(RET = core::intrinsics::transmute(_19.2), ReturnTo(bb7), UnwindUnreachable())
    }
    bb7 = {
    _19.3 = (_17.0,);
    _19.3.0 = 9_u8 as u64;
    _19.3 = _17;
    _21.1 = (-26348_i16);
    _3 = _2 >> _2;
    _5 = [60183_u16];
    _22.0 = _17.0;
    _18 = _19.3;
    _18.0 = _17.0 * _22.0;
    _18 = (_19.3.0,);
    _7 = '\u{4ee87}';
    _17.0 = _18.0;
    _19.3.0 = !_17.0;
    _22 = _18;
    _11 = [_18.0,_17.0];
    _2 = -_3;
    _21.1 = 19116_i16 * 31931_i16;
    _17 = (_22.0,);
    RET = _8;
    _12 = _10;
    _21.1 = (-27658_i16) * (-7860_i16);
    Goto(bb8)
    }
    bb8 = {
    _10 = _12 * _12;
    _21.0 = _4 + _4;
    _11 = [_22.0,_19.3.0];
    _17.0 = _19.3.0 << _3;
    _24 = _14;
    RET = [472414271_i32,(-1243046350_i32)];
    _23 = [(-100_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
    _12 = 3003_u16 as f32;
    _25 = [29364_u16];
    _19.2 = (-6302166524102625562_i64) & 3999116838854713666_i64;
    _29.fld0 = _15 << _3;
    _9 = _13 + _13;
    _19.3 = (_17.0,);
    _25 = _5;
    _28 = -_12;
    Goto(bb9)
    }
    bb9 = {
    _5 = [8522_u16];
    _10 = -_28;
    _17.0 = !_19.3.0;
    _23 = [(-57_isize),9223372036854775807_isize,9223372036854775807_isize];
    _5 = [64917_u16];
    _21 = (_4, (-11217_i16));
    _24 = _14;
    Call(_2 = core::intrinsics::transmute(_29.fld0), ReturnTo(bb10), UnwindUnreachable())
    }
    bb10 = {
    _3 = _2 | _1;
    _18.0 = _17.0 & _17.0;
    _16 = Adt49::Variant1 { fld0: _21.0,fld1: _17 };
    _23 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
    _16 = Adt49::Variant1 { fld0: _4,fld1: _19.3 };
    _21.1 = 11969_i16;
    RET = [(-1475547521_i32),(-131538791_i32)];
    _21.1 = (-6937_i16) ^ (-23648_i16);
    _19.3.0 = 9223372036854775807_isize as u64;
    _19.3.0 = _19.2 as u64;
    _5 = [54813_u16];
    _21 = (_4, 2720_i16);
    _21.1 = -17945_i16;
    _1 = _21.1 as i128;
    _19.3 = (_17.0,);
    Goto(bb11)
    }
    bb11 = {
    _1 = _3;
    _31 = (-9223372036854775808_isize) + 7_isize;
    _8 = [1286148804_i32,(-768714711_i32)];
    _7 = '\u{c3f09}';
    _19.3.0 = !Field::<(u64,)>(Variant(_16, 1), 1).0;
    _19.3.0 = _17.0 & Field::<(u64,)>(Variant(_16, 1), 1).0;
    _3 = _1 | _2;
    _13 = _9;
    _24 = _19.3.0 == _18.0;
    _18 = (_17.0,);
    _17.0 = _19.3.0;
    _10 = _28;
    _18 = (Field::<(u64,)>(Variant(_16, 1), 1).0,);
    _19.2 = 2171174656497305871_i64 - 6457464207194320227_i64;
    _17.0 = _13 as u64;
    place!(Field::<usize>(Variant(_16, 1), 0)) = _4;
    _18.0 = Field::<(u64,)>(Variant(_16, 1), 1).0 << _2;
    place!(Field::<(u64,)>(Variant(_16, 1), 1)).0 = !_18.0;
    SetDiscriminant(_16, 0);
    _18 = _19.3;
    _29.fld0 = _15 << _1;
    _33 = [3021932847_u32];
    _10 = _12;
    place!(Field::<(u64,)>(Variant(_16, 0), 1)).0 = !_19.3.0;
    Goto(bb12)
    }
    bb12 = {
    _17 = (_18.0,);
    _25 = _5;
    _18.0 = Field::<(u64,)>(Variant(_16, 0), 1).0;
    _3 = _2;
    _19.3.0 = _17.0;
    _32 = _31 << _3;
    Goto(bb13)
    }
    bb13 = {
    _23 = [_32,_32,_32];
    _34 = _32 >> _3;
    RET = [1491614885_i32,(-582166096_i32)];
    Goto(bb14)
    }
    bb14 = {
    _2 = _3;
    _18.0 = Field::<(u64,)>(Variant(_16, 0), 1).0 ^ _19.3.0;
    _15 = _29.fld0;
    _17 = (Field::<(u64,)>(Variant(_16, 0), 1).0,);
    _17 = (Field::<(u64,)>(Variant(_16, 0), 1).0,);
    RET = [(-1800358966_i32),(-127592233_i32)];
    _16 = Adt49::Variant1 { fld0: _21.0,fld1: _19.3 };
    _12 = 27_i8 as f32;
    _16 = Adt49::Variant1 { fld0: _21.0,fld1: _18 };
    RET = [1620971855_i32,1217388065_i32];
    _38 = -_28;
    _32 = _34 << _15;
    RET = [221914413_i32,(-2144522179_i32)];
    _31 = (-99_i8) as isize;
    _36 = Adt57::Variant0 { fld0: _34 };
    _36 = Adt57::Variant0 { fld0: _32 };
    _24 = _14 & _14;
    _15 = _21.1 as u128;
    _13 = 154_u8 as f64;
    _21.0 = _19.2 as usize;
    _31 = _24 as isize;
    _30 = Adt58::Variant1 { fld0: Move(_16),fld1: _19.3.0 };
    _39 = [6_i8,11_i8,(-96_i8),(-111_i8)];
    _15 = (-515810951_i32) as u128;
    _10 = 70_i8 as f32;
    Goto(bb15)
    }
    bb15 = {
    Call(_43 = dump_var(4_usize, 24_usize, Move(_24), 33_usize, Move(_33), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    Call(_43 = dump_var(4_usize, 17_usize, Move(_17), 5_usize, Move(_5), 31_usize, Move(_31), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_43 = dump_var(4_usize, 32_usize, Move(_32), 21_usize, Move(_21), 25_usize, Move(_25), 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(
    mut _1: i128,
    mut _2: i128,
    mut _3: i128,
    mut _4: i128,
    mut _5: i128,
    mut _6: i128,
    mut _7: i128,
    mut _8: [i32; 2],
    mut _9: i128,
    mut _10: i128,
) -> char {
    mir! {
    type RET = char;
    let _11: isize;
    let _12: Adt48;
    let _13: (f32, i64, (f64, i16, &'static u16), f64);
    let _14: (i8, isize, [i32; 2]);
    let _15: isize;
    let _16: f32;
    let _17: Adt57;
    let _18: char;
    let _19: Adt57;
    let _20: i64;
    let _21: u8;
    let _22: Adt49;
    let _23: isize;
    let _24: (&'static u16, &'static u16, i64, (u64,));
    let _25: [u16; 6];
    let _26: isize;
    let _27: ();
    let _28: ();
    {
    _6 = !_3;
    _5 = 11341_i16 as i128;
    _7 = -_10;
    _2 = (-3186624755631077292_i64) as i128;
    RET = '\u{93ff2}';
    _4 = _7;
    _5 = _4 >> _4;
    _12.fld0 = 53727589978928571531466727610395590931_u128;
    _11 = -78_isize;
    _1 = -_7;
    _6 = 51172_u16 as i128;
    _12 = Adt48 { fld0: 211224835983300249648660854127311625084_u128 };
    _12 = Adt48 { fld0: 145852330683432649504413531092934152943_u128 };
    _11 = (-9223372036854775808_isize) * 9223372036854775807_isize;
    _8 = [1194201561_i32,1331447659_i32];
    _8 = [(-622243148_i32),1528901160_i32];
    _9 = _3 * _4;
    _7 = _10;
    _1 = _11 as i128;
    _13.3 = _11 as f64;
    _5 = _12.fld0 as i128;
    _3 = _7 - _7;
    _9 = _7 << _4;
    _2 = !_10;
    Call(_8 = fn6(_3, _2, _3, _4, _10, _3), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _13.3 = 54053_u16 as f64;
    _4 = 61_u8 as i128;
    _12.fld0 = 74482176874800772785125282225777651112_u128;
    _12.fld0 = 237147640081551456626371169489278139525_u128;
    _3 = _7;
    _13.0 = 3534599156_u32 as f32;
    _12 = Adt48 { fld0: 300717838562802159757373795472369808689_u128 };
    _14.0 = -97_i8;
    _7 = -_9;
    _13.2.0 = _11 as f64;
    RET = '\u{44f56}';
    match _12.fld0 {
    0 => bb2,
    1 => bb3,
    300717838562802159757373795472369808689 => bb5,
    _ => bb4
    }
    }
    bb2 = {
    Return()
    }
    bb3 = {
    Return()
    }
    bb4 = {
    Return()
    }
    bb5 = {
    _14 = (78_i8, _11, _8);
    RET = '\u{8b39d}';
    _12.fld0 = 1930468855_i32 as u128;
    _12.fld0 = 79621784776946531771467669393258810838_u128;
    _13.2.0 = _13.3 + _13.3;
    Goto(bb6)
    }
    bb6 = {
    _5 = _3;
    _13.2.0 = _13.0 as f64;
    _12 = Adt48 { fld0: 322356931855143932908351130657290738423_u128 };
    _15 = _14.1;
    RET = '\u{82f62}';
    Goto(bb7)
    }
    bb7 = {
    _12.fld0 = 161808125812977455972274639158080264909_u128;
    _12 = Adt48 { fld0: 63402887081196074302715484665227422704_u128 };
    _18 = RET;
    _20 = (-7365006818591321668_i64) << _5;
    _6 = _2 ^ _2;
    _18 = RET;
    _12.fld0 = 285194137396223050811709003239273331214_u128;
    _14.1 = 183_u8 as isize;
    _7 = false as i128;
    _5 = _2;
    _15 = _11 - _14.1;
    _10 = !_6;
    match _14.0 {
    0 => bb5,
    1 => bb2,
    2 => bb8,
    3 => bb9,
    4 => bb10,
    78 => bb12,
    _ => bb11
    }
    }
    bb8 = {
    _13.3 = 54053_u16 as f64;
    _4 = 61_u8 as i128;
    _12.fld0 = 74482176874800772785125282225777651112_u128;
    _12.fld0 = 237147640081551456626371169489278139525_u128;
    _3 = _7;
    _13.0 = 3534599156_u32 as f32;
    _12 = Adt48 { fld0: 300717838562802159757373795472369808689_u128 };
    _14.0 = -97_i8;
    _7 = -_9;
    _13.2.0 = _11 as f64;
    RET = '\u{44f56}';
    match _12.fld0 {
    0 => bb2,
    1 => bb3,
    300717838562802159757373795472369808689 => bb5,
    _ => bb4
    }
    }
    bb9 = {
    Return()
    }
    bb10 = {
    Return()
    }
    bb11 = {
    Return()
    }
    bb12 = {
    _7 = !_3;
    _10 = _2 ^ _5;
    _13.1 = _6 as i64;
    _14.1 = !_11;
    _7 = _13.1 as i128;
    RET = _18;
    _10 = _3;
    _14.1 = _12.fld0 as isize;
    _3 = 5531817773799776295_usize as i128;
    _13.2.1 = -(-8888_i16);
    _20 = false as i64;
    _5 = _7 | _9;
    _9 = -_7;
    _7 = _5;
    _11 = _14.1;
    _13.0 = _12.fld0 as f32;
    _21 = 254_u8 & 35_u8;
    match _14.0 {
    0 => bb4,
    1 => bb13,
    2 => bb14,
    3 => bb15,
    78 => bb17,
    _ => bb16
    }
    }
    bb13 = {
    _12.fld0 = 161808125812977455972274639158080264909_u128;
    _12 = Adt48 { fld0: 63402887081196074302715484665227422704_u128 };
    _18 = RET;
    _20 = (-7365006818591321668_i64) << _5;
    _6 = _2 ^ _2;
    _18 = RET;
    _12.fld0 = 285194137396223050811709003239273331214_u128;
    _14.1 = 183_u8 as isize;
    _7 = false as i128;
    _5 = _2;
    _15 = _11 - _14.1;
    _10 = !_6;
    match _14.0 {
    0 => bb5,
    1 => bb2,
    2 => bb8,
    3 => bb9,
    4 => bb10,
    78 => bb12,
    _ => bb11
    }
    }
    bb14 = {
    Return()
    }
    bb15 = {
    Return()
    }
    bb16 = {
    _5 = _3;
    _13.2.0 = _13.0 as f64;
    _12 = Adt48 { fld0: 322356931855143932908351130657290738423_u128 };
    _15 = _14.1;
    RET = '\u{82f62}';
    Goto(bb7)
    }
    bb17 = {
    _24.2 = _13.1;
    Goto(bb18)
    }
    bb18 = {
    Call(_27 = dump_var(5_usize, 14_usize, Move(_14), 11_usize, Move(_11), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Call(_27 = dump_var(5_usize, 2_usize, Move(_2), 7_usize, Move(_7), 21_usize, Move(_21), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
    }
    bb20 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(
    mut _1: i128,
    mut _2: i128,
    mut _3: i128,
    mut _4: i128,
    mut _5: i128,
    mut _6: i128,
) -> [i32; 2] {
    mir! {
    type RET = [i32; 2];
    let _7: [u16; 6];
    let _8: [u32; 6];
    let _9: (u64,);
    let _10: [i16; 6];
    let _11: &'static u16;
    let _12: f64;
    let _13: [u64; 6];
    let _14: f32;
    let _15: [i8; 4];
    let _16: *mut (i8, isize, [i32; 2]);
    let _17: [usize; 3];
    let _18: [u16; 1];
    let _19: [usize; 3];
    let _20: [i16; 6];
    let _21: *const f32;
    let _22: (f32, i64, (f64, i16, &'static u16), f64);
    let _23: char;
    let _24: u64;
    let _25: f32;
    let _26: i16;
    let _27: *const f32;
    let _28: i64;
    let _29: Adt47;
    let _30: f64;
    let _31: (u64,);
    let _32: u8;
    let _33: ();
    let _34: ();
    {
    RET = [(-1589379567_i32),(-874896907_i32)];
    RET = [1753479264_i32,1906818417_i32];
    _6 = -_2;
    RET = [169850622_i32,(-1057162717_i32)];
    _8 = [1853999028_u32,2290131528_u32,3631227613_u32,1287936066_u32,732265921_u32,2752140276_u32];
    RET = [(-1339307677_i32),(-1264891166_i32)];
    _8 = [3642461568_u32,2008318861_u32,269352573_u32,1525959359_u32,3942266941_u32,209752100_u32];
    _9.0 = 11415271656136277423_u64 >> _4;
    _10 = [(-7806_i16),5800_i16,(-11443_i16),3989_i16,(-12638_i16),(-20662_i16)];
    _1 = -_6;
    _8 = [4127323723_u32,3294781299_u32,2719312177_u32,1896124736_u32,651065197_u32,905651980_u32];
    _2 = 177_u8 as i128;
    _7 = [23968_u16,49751_u16,6610_u16,63228_u16,15727_u16,45816_u16];
    _2 = _5;
    _3 = _9.0 as i128;
    _5 = _3 >> _1;
    _9 = (7392193430345207621_u64,);
    _5 = _2;
    _9.0 = !16096412635652653282_u64;
    _9.0 = 565852319_i32 as u64;
    _9 = (10234016496530167766_u64,);
    _9.0 = !15582648012173830466_u64;
    Goto(bb1)
    }
    bb1 = {
    _4 = !_2;
    _1 = _5;
    Call(_3 = fn7(_4, _4, _6, _5), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    _5 = !_6;
    RET = [199591607_i32,1562249842_i32];
    _6 = 6606241025634403821640776567507270972_u128 as i128;
    _9 = (10334610177193879692_u64,);
    _8 = [1744975649_u32,3889029315_u32,251023406_u32,1860085162_u32,833759942_u32,1666511726_u32];
    match _9.0 {
    0 => bb3,
    1 => bb4,
    2 => bb5,
    3 => bb6,
    10334610177193879692 => bb8,
    _ => bb7
    }
    }
    bb3 = {
    _4 = !_2;
    _1 = _5;
    Call(_3 = fn7(_4, _4, _6, _5), ReturnTo(bb2), UnwindUnreachable())
    }
    bb4 = {
    Return()
    }
    bb5 = {
    Return()
    }
    bb6 = {
    Return()
    }
    bb7 = {
    Return()
    }
    bb8 = {
    _6 = _3 ^ _2;
    _8 = [3060339693_u32,1661672921_u32,349394821_u32,3503149103_u32,875381719_u32,3053461325_u32];
    _9.0 = !4046344710841816426_u64;
    RET = [(-1594896299_i32),(-936124598_i32)];
    _9.0 = 920743303478655654_u64;
    _1 = _5 | _6;
    _13 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
    _15 = [63_i8,(-10_i8),(-30_i8),(-20_i8)];
    _9.0 = (-734047205_i32) as u64;
    _13 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
    RET = [1090805240_i32,1263982410_i32];
    RET = [278607316_i32,552710710_i32];
    _7 = [61527_u16,28576_u16,4486_u16,1418_u16,5389_u16,42586_u16];
    _15 = [54_i8,39_i8,(-42_i8),112_i8];
    _9 = (14560318465726261766_u64,);
    _1 = (-3711443664188698761_i64) as i128;
    _5 = _6;
    Call(_16 = fn8(_4, _5, _5, _10, _5, _6, _5, _4, _5, _3), ReturnTo(bb9), UnwindUnreachable())
    }
    bb9 = {
    _14 = 2835211276551018927_i64 as f32;
    _12 = 39135291034369792709639531133519354922_u128 as f64;
    _9.0 = !283128926885740949_u64;
    _18 = [530_u16];
    _19 = [2_usize,3_usize,17521982045846062131_usize];
    _6 = _2;
    _17 = [6627211430521579028_usize,8316925537613360996_usize,1769415501826153620_usize];
    _15 = [88_i8,101_i8,17_i8,(-15_i8)];
    _5 = _4 | _3;
    _1 = 63_isize as i128;
    _7 = [28198_u16,1900_u16,28970_u16,43550_u16,16860_u16,13052_u16];
    _13 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
    _9.0 = !2795571663444259176_u64;
    _22.2.1 = 31869_i16 | 32250_i16;
    _20 = [_22.2.1,_22.2.1,_22.2.1,_22.2.1,_22.2.1,_22.2.1];
    _18 = [48726_u16];
    _22.2.0 = _22.2.1 as f64;
    _8 = [738367048_u32,2841604051_u32,2448550100_u32,1480493786_u32,2755728416_u32,1548732002_u32];
    _20 = [_22.2.1,_22.2.1,_22.2.1,_22.2.1,_22.2.1,_22.2.1];
    Goto(bb10)
    }
    bb10 = {
    _22.3 = _22.2.0;
    _8 = [3943561575_u32,1642729888_u32,3313874420_u32,987673999_u32,551271329_u32,2361575781_u32];
    _22.3 = 2600687676308385526_i64 as f64;
    Goto(bb11)
    }
    bb11 = {
    RET = [(-163000837_i32),571101642_i32];
    _5 = !_6;
    RET = [689291475_i32,733938980_i32];
    _12 = _6 as f64;
    _6 = !_3;
    _5 = !_4;
    _9.0 = '\u{354e6}' as u64;
    _1 = _4 + _6;
    _22.2.1 = -18476_i16;
    _26 = !_22.2.1;
    RET = [1928405460_i32,(-1521282173_i32)];
    _1 = _14 as i128;
    _14 = _12 as f32;
    _22.0 = 58_u8 as f32;
    _1 = _6;
    _9.0 = 7429184930090228469_u64 >> _4;
    Goto(bb12)
    }
    bb12 = {
    _17 = _19;
    _21 = core::ptr::addr_of!(_14);
    _23 = '\u{cca2f}';
    _7 = [44624_u16,45980_u16,11170_u16,23063_u16,28916_u16,20097_u16];
    _25 = (*_21) * _14;
    _10 = [_26,_22.2.1,_22.2.1,_22.2.1,_26,_22.2.1];
    _21 = core::ptr::addr_of!((*_21));
    _27 = _21;
    _2 = _4 ^ _6;
    _20 = [_26,_22.2.1,_22.2.1,_22.2.1,_26,_26];
    Goto(bb13)
    }
    bb13 = {
    _22.2.0 = -_12;
    Call(RET = fn13(_2, _21, _3, (*_27), _1, _12, _2, _5, _27, _9, _2, (*_27)), ReturnTo(bb14), UnwindUnreachable())
    }
    bb14 = {
    _22.3 = -_12;
    _13 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
    _24 = _9.0 - _9.0;
    _12 = 4419704516666916942_i64 as f64;
    _22.3 = -_22.2.0;
    _28 = (-1346665341861090995_i64);
    (*_27) = _25;
    Goto(bb15)
    }
    bb15 = {
    Call(_33 = dump_var(6_usize, 18_usize, Move(_18), 13_usize, Move(_13), 7_usize, Move(_7), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    Call(_33 = dump_var(6_usize, 17_usize, Move(_17), 6_usize, Move(_6), 15_usize, Move(_15), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_33 = dump_var(6_usize, 9_usize, Move(_9), 24_usize, Move(_24), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i128, mut _2: i128, mut _3: i128, mut _4: i128) -> i128 {
    mir! {
    type RET = i128;
    let _5: [usize; 3];
    let _6: ();
    let _7: ();
    {
    _4 = _3;
    _2 = _4;
    RET = _3 + _4;
    RET = (-1380045916_i32) as i128;
    RET = _2;
    _3 = _1;
    RET = _1;
    _2 = !_3;
    _4 = _1 + RET;
    Goto(bb1)
    }
    bb1 = {
    Call(_6 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(
    mut _1: i128,
    mut _2: i128,
    mut _3: i128,
    mut _4: [i16; 6],
    mut _5: i128,
    mut _6: i128,
    mut _7: i128,
    mut _8: i128,
    mut _9: i128,
    mut _10: i128,
) -> *mut (i8, isize, [i32; 2]) {
    mir! {
    type RET = *mut (i8, isize, [i32; 2]);
    let _11: isize;
    let _12: Adt48;
    let _13: [u64; 2];
    let _14: bool;
    let _15: u64;
    let _16: i32;
    let _17: char;
    let _18: Adt49;
    let _19: u128;
    let _20: [u16; 6];
    let _21: f32;
    let _22: Adt58;
    let _23: f32;
    let _24: (&'static u16, &'static u16, i64, (u64,));
    let _25: f32;
    let _26: f32;
    let _27: [i8; 4];
    let _28: u16;
    let _29: Adt48;
    let _30: &'static u16;
    let _31: [i32; 2];
    let _32: (i8, isize, [i32; 2]);
    let _33: [u16; 1];
    let _34: isize;
    let _35: isize;
    let _36: *mut (i8, isize, [i32; 2]);
    let _37: char;
    let _38: [isize; 3];
    let _39: u8;
    let _40: Adt54;
    let _41: [i16; 6];
    let _42: [u32; 6];
    let _43: [i32; 2];
    let _44: u16;
    let _45: (u64,);
    let _46: i32;
    let _47: [i8; 4];
    let _48: (usize, i16);
    let _49: ();
    let _50: ();
    {
    _10 = _2 & _5;
    _6 = 16662850356701524914_u64 as i128;
    _6 = _8 & _5;
    _5 = _3 >> _9;
    _4 = [(-20770_i16),17880_i16,(-8602_i16),(-32618_i16),(-18182_i16),(-10990_i16)];
    _4 = [(-24497_i16),22902_i16,(-27702_i16),(-13993_i16),(-29318_i16),(-29634_i16)];
    _6 = 4061343449_u32 as i128;
    _10 = _1;
    _1 = !_2;
    _11 = -9223372036854775807_isize;
    _7 = !_10;
    _10 = 149917549055091883722484631228470634181_u128 as i128;
    _9 = _7;
    _2 = (-14362_i16) as i128;
    _2 = _5 >> _5;
    _6 = -_2;
    _6 = _5;
    Goto(bb1)
    }
    bb1 = {
    _2 = _8 >> _7;
    _12.fld0 = 28469519975369120905996713103278089193_u128 | 318398821257771577384814848109109619988_u128;
    _2 = _5 << _6;
    _3 = !_1;
    _15 = 10391195934208514933_u64;
    _4 = [5639_i16,(-23193_i16),(-28201_i16),(-31619_i16),(-3161_i16),(-27733_i16)];
    _8 = _2;
    _6 = -_2;
    _5 = _9 ^ _6;
    _6 = _1 * _5;
    _15 = true as u64;
    Call(_15 = core::intrinsics::bswap(16768160325669389490_u64), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    _12.fld0 = 582146912_i32 as u128;
    _1 = 2701505782_u32 as i128;
    Goto(bb3)
    }
    bb3 = {
    _10 = !_9;
    _12.fld0 = 330323162144807822380855112857865515570_u128 >> _10;
    _8 = _3 * _2;
    _13 = [_15,_15];
    _7 = _5;
    Goto(bb4)
    }
    bb4 = {
    _2 = '\u{8d5a6}' as i128;
    _8 = _6;
    _6 = _10 & _5;
    _16 = (-734285402_i32) * 464034254_i32;
    _7 = _10 ^ _3;
    _3 = -_7;
    _16 = (-3008554423984786503_i64) as i32;
    _6 = _16 as i128;
    _14 = _10 == _5;
    _15 = 146_u8 as u64;
    _8 = !_3;
    _6 = _8;
    _3 = 3970255187083766732_i64 as i128;
    _7 = (-7442_i16) as i128;
    _11 = -(-9223372036854775808_isize);
    _12.fld0 = 211282759339872725714837240414675275424_u128 - 181278494371414775758743029700938033977_u128;
    Call(RET = fn9(_5, _14, _14), ReturnTo(bb5), UnwindUnreachable())
    }
    bb5 = {
    _10 = -_5;
    _8 = -_10;
    _4 = [(-3016_i16),13837_i16,1140_i16,22968_i16,6143_i16,(-6954_i16)];
    _17 = '\u{b5453}';
    _2 = -_9;
    _5 = !_8;
    _11 = 173_u8 as isize;
    _10 = _5;
    _5 = _12.fld0 as i128;
    _8 = !_6;
    _11 = _16 as isize;
    _17 = '\u{19327}';
    _15 = 12645729856430501189_u64;
    _13 = [_15,_15];
    _15 = 9418445182209344984_u64 & 13863234017693838292_u64;
    _1 = _6 << _8;
    _12.fld0 = 237714776716249557726939618111735612035_u128;
    _12.fld0 = 109396486016935857898289629620932542082_u128 + 44090720872114990974336401349000720870_u128;
    _15 = !14245088847411834381_u64;
    _6 = _10 + _1;
    _4 = [13743_i16,(-32191_i16),(-23987_i16),9688_i16,25661_i16,(-18335_i16)];
    _5 = 83_i8 as i128;
    _9 = _6 * _10;
    _16 = (-581453472_i32) ^ (-1394278166_i32);
    _20 = [40554_u16,65079_u16,31383_u16,62513_u16,48808_u16,3534_u16];
    _5 = -_9;
    Goto(bb6)
    }
    bb6 = {
    _17 = '\u{edb3f}';
    _11 = 4182345578231684695_i64 as isize;
    _5 = !_9;
    _19 = _12.fld0;
    _19 = !_12.fld0;
    _12 = Adt48 { fld0: _19 };
    _9 = -_6;
    _11 = _15 as isize;
    _2 = _1;
    _17 = '\u{3b989}';
    _5 = !_8;
    _4 = [31026_i16,(-1010_i16),9288_i16,10690_i16,(-2881_i16),529_i16];
    _1 = -_8;
    _5 = _10;
    _12.fld0 = _19;
    _19 = !_12.fld0;
    _14 = false;
    _4 = [25588_i16,(-1058_i16),3980_i16,16459_i16,18277_i16,6442_i16];
    _5 = _1;
    _2 = (-64_i8) as i128;
    _21 = _15 as f32;
    Call(_4 = fn11(_1, _6, _1, _6, _20, _5, _10, _1), ReturnTo(bb7), UnwindUnreachable())
    }
    bb7 = {
    _12 = Adt48 { fld0: _19 };
    _20 = [4284_u16,3691_u16,28999_u16,34819_u16,17362_u16,59623_u16];
    _15 = 6434159824554862656_u64 + 18118353227604837077_u64;
    _24.2 = !(-1016756513784898783_i64);
    _23 = _21;
    _24.3 = (_15,);
    _2 = _24.2 as i128;
    _18 = Adt49::Variant1 { fld0: 3_usize,fld1: _24.3 };
    _27 = [92_i8,(-101_i8),(-25_i8),109_i8];
    _24.1 = &_28;
    _8 = _5 >> _9;
    Call(_11 = core::intrinsics::bswap(78_isize), ReturnTo(bb8), UnwindUnreachable())
    }
    bb8 = {
    _18 = Adt49::Variant1 { fld0: 1_usize,fld1: _24.3 };
    _9 = -_8;
    _28 = !51643_u16;
    _25 = _21 + _21;
    _17 = '\u{16835}';
    _28 = 58854_u16 >> _1;
    _12 = Adt48 { fld0: _19 };
    _29 = Adt48 { fld0: _19 };
    _21 = _25;
    _15 = 42_i8 as u64;
    _14 = false ^ false;
    _26 = _19 as f32;
    _8 = 2826763863_u32 as i128;
    place!(Field::<usize>(Variant(_18, 1), 0)) = !6_usize;
    _9 = _1 >> _28;
    Goto(bb9)
    }
    bb9 = {
    SetDiscriminant(_18, 0);
    _7 = _1;
    _32.2 = [_16,_16];
    _2 = _7 ^ _10;
    _29.fld0 = _19 & _12.fld0;
    _24.1 = &_28;
    _24.0 = Move(_24.1);
    _12 = Adt48 { fld0: _29.fld0 };
    _24.0 = &_28;
    _33 = [_28];
    _9 = -_6;
    _24.2 = (-2744277688562542509_i64);
    _23 = _25;
    _24.0 = &_28;
    place!(Field::<(u64,)>(Variant(_18, 0), 1)) = (_24.3.0,);
    _24.1 = &_28;
    _32.1 = _11;
    _37 = _17;
    _24.0 = &_28;
    _24.3.0 = _14 as u64;
    _19 = _12.fld0 | _12.fld0;
    _4 = [(-5730_i16),(-20822_i16),(-9558_i16),(-2925_i16),(-30565_i16),(-24099_i16)];
    _14 = _1 != _7;
    match _24.2 {
    0 => bb1,
    1 => bb2,
    2 => bb8,
    3 => bb5,
    4 => bb10,
    340282366920938463460630329743205668947 => bb12,
    _ => bb11
    }
    }
    bb10 = {
    _12.fld0 = 582146912_i32 as u128;
    _1 = 2701505782_u32 as i128;
    Goto(bb3)
    }
    bb11 = {
    _2 = '\u{8d5a6}' as i128;
    _8 = _6;
    _6 = _10 & _5;
    _16 = (-734285402_i32) * 464034254_i32;
    _7 = _10 ^ _3;
    _3 = -_7;
    _16 = (-3008554423984786503_i64) as i32;
    _6 = _16 as i128;
    _14 = _10 == _5;
    _15 = 146_u8 as u64;
    _8 = !_3;
    _6 = _8;
    _3 = 3970255187083766732_i64 as i128;
    _7 = (-7442_i16) as i128;
    _11 = -(-9223372036854775808_isize);
    _12.fld0 = 211282759339872725714837240414675275424_u128 - 181278494371414775758743029700938033977_u128;
    Call(RET = fn9(_5, _14, _14), ReturnTo(bb5), UnwindUnreachable())
    }
    bb12 = {
    _39 = _37 as u8;
    _6 = _24.3.0 as i128;
    _6 = _28 as i128;
    _8 = _32.1 as i128;
    _38 = [_11,_11,_32.1];
    _24.3.0 = Field::<(u64,)>(Variant(_18, 0), 1).0 * Field::<(u64,)>(Variant(_18, 0), 1).0;
    place!(Field::<[char; 7]>(Variant(_18, 0), 0)) = [_37,_17,_37,_17,_37,_37,_37];
    _35 = !_32.1;
    _24.2 = 7862549895086090477_i64 << _6;
    _29.fld0 = 3449561614_u32 as u128;
    Goto(bb13)
    }
    bb13 = {
    _24.0 = Move(_24.1);
    _30 = &_28;
    _11 = _35;
    place!(Field::<(u64,)>(Variant(_18, 0), 1)) = (_24.3.0,);
    _32.2 = [_16,_16];
    _24.1 = &(*_30);
    _24 = (Move(_30), Move(_30), (-4945463414536885910_i64), Field::<(u64,)>(Variant(_18, 0), 1));
    _22 = Adt58::Variant1 { fld0: Move(_18),fld1: Field::<(u64,)>(Variant(_18, 0), 1).0 };
    _26 = _21 + _25;
    _12.fld0 = _19 + _19;
    _3 = _9;
    _12.fld0 = _16 as u128;
    _42 = [1557649993_u32,3916506032_u32,328247390_u32,293234306_u32,643200307_u32,2075381065_u32];
    _14 = false;
    _43 = [_16,_16];
    _10 = _1;
    match _24.2 {
    0 => bb7,
    1 => bb14,
    2 => bb15,
    3 => bb16,
    4 => bb17,
    340282366920938463458429144017231325546 => bb19,
    _ => bb18
    }
    }
    bb14 = {
    _10 = -_5;
    _8 = -_10;
    _4 = [(-3016_i16),13837_i16,1140_i16,22968_i16,6143_i16,(-6954_i16)];
    _17 = '\u{b5453}';
    _2 = -_9;
    _5 = !_8;
    _11 = 173_u8 as isize;
    _10 = _5;
    _5 = _12.fld0 as i128;
    _8 = !_6;
    _11 = _16 as isize;
    _17 = '\u{19327}';
    _15 = 12645729856430501189_u64;
    _13 = [_15,_15];
    _15 = 9418445182209344984_u64 & 13863234017693838292_u64;
    _1 = _6 << _8;
    _12.fld0 = 237714776716249557726939618111735612035_u128;
    _12.fld0 = 109396486016935857898289629620932542082_u128 + 44090720872114990974336401349000720870_u128;
    _15 = !14245088847411834381_u64;
    _6 = _10 + _1;
    _4 = [13743_i16,(-32191_i16),(-23987_i16),9688_i16,25661_i16,(-18335_i16)];
    _5 = 83_i8 as i128;
    _9 = _6 * _10;
    _16 = (-581453472_i32) ^ (-1394278166_i32);
    _20 = [40554_u16,65079_u16,31383_u16,62513_u16,48808_u16,3534_u16];
    _5 = -_9;
    Goto(bb6)
    }
    bb15 = {
    _2 = '\u{8d5a6}' as i128;
    _8 = _6;
    _6 = _10 & _5;
    _16 = (-734285402_i32) * 464034254_i32;
    _7 = _10 ^ _3;
    _3 = -_7;
    _16 = (-3008554423984786503_i64) as i32;
    _6 = _16 as i128;
    _14 = _10 == _5;
    _15 = 146_u8 as u64;
    _8 = !_3;
    _6 = _8;
    _3 = 3970255187083766732_i64 as i128;
    _7 = (-7442_i16) as i128;
    _11 = -(-9223372036854775808_isize);
    _12.fld0 = 211282759339872725714837240414675275424_u128 - 181278494371414775758743029700938033977_u128;
    Call(RET = fn9(_5, _14, _14), ReturnTo(bb5), UnwindUnreachable())
    }
    bb16 = {
    _12 = Adt48 { fld0: _19 };
    _20 = [4284_u16,3691_u16,28999_u16,34819_u16,17362_u16,59623_u16];
    _15 = 6434159824554862656_u64 + 18118353227604837077_u64;
    _24.2 = !(-1016756513784898783_i64);
    _23 = _21;
    _24.3 = (_15,);
    _2 = _24.2 as i128;
    _18 = Adt49::Variant1 { fld0: 3_usize,fld1: _24.3 };
    _27 = [92_i8,(-101_i8),(-25_i8),109_i8];
    _24.1 = &_28;
    _8 = _5 >> _9;
    Call(_11 = core::intrinsics::bswap(78_isize), ReturnTo(bb8), UnwindUnreachable())
    }
    bb17 = {
    SetDiscriminant(_18, 0);
    _7 = _1;
    _32.2 = [_16,_16];
    _2 = _7 ^ _10;
    _29.fld0 = _19 & _12.fld0;
    _24.1 = &_28;
    _24.0 = Move(_24.1);
    _12 = Adt48 { fld0: _29.fld0 };
    _24.0 = &_28;
    _33 = [_28];
    _9 = -_6;
    _24.2 = (-2744277688562542509_i64);
    _23 = _25;
    _24.0 = &_28;
    place!(Field::<(u64,)>(Variant(_18, 0), 1)) = (_24.3.0,);
    _24.1 = &_28;
    _32.1 = _11;
    _37 = _17;
    _24.0 = &_28;
    _24.3.0 = _14 as u64;
    _19 = _12.fld0 | _12.fld0;
    _4 = [(-5730_i16),(-20822_i16),(-9558_i16),(-2925_i16),(-30565_i16),(-24099_i16)];
    _14 = _1 != _7;
    match _24.2 {
    0 => bb1,
    1 => bb2,
    2 => bb8,
    3 => bb5,
    4 => bb10,
    340282366920938463460630329743205668947 => bb12,
    _ => bb11
    }
    }
    bb18 = {
    _17 = '\u{edb3f}';
    _11 = 4182345578231684695_i64 as isize;
    _5 = !_9;
    _19 = _12.fld0;
    _19 = !_12.fld0;
    _12 = Adt48 { fld0: _19 };
    _9 = -_6;
    _11 = _15 as isize;
    _2 = _1;
    _17 = '\u{3b989}';
    _5 = !_8;
    _4 = [31026_i16,(-1010_i16),9288_i16,10690_i16,(-2881_i16),529_i16];
    _1 = -_8;
    _5 = _10;
    _12.fld0 = _19;
    _19 = !_12.fld0;
    _14 = false;
    _4 = [25588_i16,(-1058_i16),3980_i16,16459_i16,18277_i16,6442_i16];
    _5 = _1;
    _2 = (-64_i8) as i128;
    _21 = _15 as f32;
    Call(_4 = fn11(_1, _6, _1, _6, _20, _5, _10, _1), ReturnTo(bb7), UnwindUnreachable())
    }
    bb19 = {
    _9 = _10 - _1;
    _32.2 = _43;
    _24.2 = (-6017147892756706072_i64) - (-4781499786960099957_i64);
    _3 = _39 as i128;
    _10 = _5;
    _41 = [31278_i16,(-22165_i16),(-23358_i16),(-2881_i16),(-26979_i16),19417_i16];
    _36 = core::ptr::addr_of_mut!(_32);
    (*_36) = (76_i8, _11, _43);
    _24.1 = &_28;
    Goto(bb20)
    }
    bb20 = {
    Call(_49 = dump_var(8_usize, 10_usize, Move(_10), 33_usize, Move(_33), 8_usize, Move(_8), 41_usize, Move(_41)), ReturnTo(bb21), UnwindUnreachable())
    }
    bb21 = {
    Call(_49 = dump_var(8_usize, 43_usize, Move(_43), 5_usize, Move(_5), 39_usize, Move(_39), 4_usize, Move(_4)), ReturnTo(bb22), UnwindUnreachable())
    }
    bb22 = {
    Call(_49 = dump_var(8_usize, 15_usize, Move(_15), 2_usize, Move(_2), 35_usize, Move(_35), 42_usize, Move(_42)), ReturnTo(bb23), UnwindUnreachable())
    }
    bb23 = {
    Call(_49 = dump_var(8_usize, 7_usize, Move(_7), 28_usize, Move(_28), 50_usize, _50, 50_usize, _50), ReturnTo(bb24), UnwindUnreachable())
    }
    bb24 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i128, mut _2: bool, mut _3: bool) -> *mut (i8, isize, [i32; 2]) {
    mir! {
    type RET = *mut (i8, isize, [i32; 2]);
    let _4: *mut char;
    let _5: i64;
    let _6: [u16; 1];
    let _7: f64;
    let _8: Adt52;
    let _9: isize;
    let _10: Adt55;
    let _11: f32;
    let _12: Adt49;
    let _13: bool;
    let _14: u8;
    let _15: isize;
    let _16: (i8, isize, [i32; 2]);
    let _17: [u64; 6];
    let _18: bool;
    let _19: ();
    let _20: ();
    {
    _3 = _2 ^ _2;
    _2 = _3;
    _3 = !_2;
    _3 = _2;
    _1 = 4198899297305691009582053051134664685_i128 >> 230084169357296591129000753530937932184_u128;
    _2 = _3 < _3;
    _1 = 6507153397718929236555329336924740448_i128;
    _3 = !_2;
    _1 = 2147889417699104045_u64 as i128;
    _1 = -25339933833008714933345048510761487889_i128;
    _3 = _2 | _2;
    _2 = _3 != _3;
    _3 = _2 | _2;
    _5 = (-118_i8) as i64;
    _1 = (-167532154606436839435580528178846949742_i128);
    _1 = 5349043031984704461220494800832709764_i128 - 106309181570222247669757091844080478454_i128;
    _5 = 6622955962840022716_i64;
    _2 = _3 ^ _3;
    _2 = _3 != _3;
    _2 = _3 != _3;
    match _5 {
    0 => bb1,
    1 => bb2,
    6622955962840022716 => bb4,
    _ => bb3
    }
    }
    bb1 = {
    Return()
    }
    bb2 = {
    Return()
    }
    bb3 = {
    Return()
    }
    bb4 = {
    _6 = [26496_u16];
    _2 = _3;
    _2 = _3;
    _3 = _2;
    _7 = 8512_u16 as f64;
    _1 = 47345340986495566523014199963851719561_i128 ^ (-165825386757203039252554921324354290428_i128);
    _7 = 1123300645_i32 as f64;
    _7 = 4_usize as f64;
    _7 = _1 as f64;
    _7 = 3061_i16 as f64;
    _2 = !_3;
    _6 = [36306_u16];
    _2 = _3;
    _5 = (-5581600475435057365_i64);
    _13 = !_3;
    _2 = !_13;
    _6 = [57282_u16];
    _11 = 91701445236336115273693590650029309869_u128 as f32;
    _7 = _11 as f64;
    Call(_1 = fn10(_3, _3, _2, _13, _2, _13, _13, _13, _13, _2, _3, _13, _3, _2), ReturnTo(bb5), UnwindUnreachable())
    }
    bb5 = {
    _13 = _1 != _1;
    _6 = [28808_u16];
    match _5 {
    0 => bb1,
    1 => bb2,
    2 => bb6,
    3 => bb7,
    4 => bb8,
    5 => bb9,
    6 => bb10,
    340282366920938463457793006956333154091 => bb12,
    _ => bb11
    }
    }
    bb6 = {
    _6 = [26496_u16];
    _2 = _3;
    _2 = _3;
    _3 = _2;
    _7 = 8512_u16 as f64;
    _1 = 47345340986495566523014199963851719561_i128 ^ (-165825386757203039252554921324354290428_i128);
    _7 = 1123300645_i32 as f64;
    _7 = 4_usize as f64;
    _7 = _1 as f64;
    _7 = 3061_i16 as f64;
    _2 = !_3;
    _6 = [36306_u16];
    _2 = _3;
    _5 = (-5581600475435057365_i64);
    _13 = !_3;
    _2 = !_13;
    _6 = [57282_u16];
    _11 = 91701445236336115273693590650029309869_u128 as f32;
    _7 = _11 as f64;
    Call(_1 = fn10(_3, _3, _2, _13, _2, _13, _13, _13, _13, _2, _3, _13, _3, _2), ReturnTo(bb5), UnwindUnreachable())
    }
    bb7 = {
    Return()
    }
    bb8 = {
    Return()
    }
    bb9 = {
    Return()
    }
    bb10 = {
    Return()
    }
    bb11 = {
    Return()
    }
    bb12 = {
    _1 = _13 as i128;
    _15 = 9223372036854775807_isize;
    _5 = '\u{e18f}' as i64;
    _7 = 7_usize as f64;
    _9 = _15 | _15;
    _15 = _9 | _9;
    _13 = _3;
    _7 = _1 as f64;
    _6 = [4523_u16];
    _1 = (-53490072947972345429694995029460343079_i128);
    _1 = (-167219884455879287942426553055556564082_i128);
    _7 = 5139_u16 as f64;
    _9 = !_15;
    RET = core::ptr::addr_of_mut!(_16);
    (*RET).1 = !_15;
    (*RET).2 = [1752413001_i32,(-1962495045_i32)];
    _7 = 1218887959025828605_usize as f64;
    match _1 {
    0 => bb11,
    1 => bb13,
    173062482465059175520948054376211647374 => bb15,
    _ => bb14
    }
    }
    bb13 = {
    Return()
    }
    bb14 = {
    Return()
    }
    bb15 = {
    _16.0 = 94_i8 & (-14_i8);
    _7 = 18112438071823760349_usize as f64;
    _9 = (*RET).1;
    _6 = [879_u16];
    (*RET).0 = 111_i8;
    _6 = [25245_u16];
    (*RET).0 = (-32657_i16) as i8;
    _16.2 = [(-1088168347_i32),62879104_i32];
    (*RET).1 = _1 as isize;
    _2 = !_13;
    _1 = !(-166087515699115914086489635875584863364_i128);
    RET = core::ptr::addr_of_mut!(_16);
    Goto(bb16)
    }
    bb16 = {
    Call(_19 = dump_var(9_usize, 5_usize, Move(_5), 13_usize, Move(_13), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(
    mut _1: bool,
    mut _2: bool,
    mut _3: bool,
    mut _4: bool,
    mut _5: bool,
    mut _6: bool,
    mut _7: bool,
    mut _8: bool,
    mut _9: bool,
    mut _10: bool,
    mut _11: bool,
    mut _12: bool,
    mut _13: bool,
    mut _14: bool,
) -> i128 {
    mir! {
    type RET = i128;
    let _15: [i32; 2];
    let _16: [i8; 4];
    let _17: ();
    let _18: ();
    {
    _11 = _1;
    _6 = _13 < _5;
    _5 = _10 == _8;
    _12 = _1 < _13;
    RET = !142128484146120200053925349467975939181_i128;
    _1 = _4;
    RET = (-151237107904829966153250622682146083019_i128);
    _2 = _10 >= _7;
    _13 = _8 == _14;
    _2 = _14;
    _14 = _11 >= _4;
    RET = _3 as i128;
    _15 = [282791163_i32,(-1603551397_i32)];
    Goto(bb1)
    }
    bb1 = {
    Call(_17 = dump_var(10_usize, 15_usize, Move(_15), 11_usize, Move(_11), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    Call(_17 = dump_var(10_usize, 9_usize, Move(_9), 4_usize, Move(_4), 10_usize, Move(_10), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
    }
    bb3 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(
    mut _1: i128,
    mut _2: i128,
    mut _3: i128,
    mut _4: i128,
    mut _5: [u16; 6],
    mut _6: i128,
    mut _7: i128,
    mut _8: i128,
) -> [i16; 6] {
    mir! {
    type RET = [i16; 6];
    let _9: isize;
    let _10: [i32; 2];
    let _11: u16;
    let _12: [u64; 2];
    let _13: *mut u64;
    let _14: f64;
    let _15: (&'static u16, &'static u16, i64, (u64,));
    let _16: [u16; 1];
    let _17: Adt57;
    let _18: Adt61;
    let _19: [u32; 1];
    let _20: *mut char;
    let _21: isize;
    let _22: i64;
    let _23: usize;
    let _24: [char; 7];
    let _25: u64;
    let _26: i16;
    let _27: f32;
    let _28: Adt55;
    let _29: char;
    let _30: char;
    let _31: u32;
    let _32: char;
    let _33: u8;
    let _34: i16;
    let _35: i8;
    let _36: ();
    let _37: ();
    {
    RET = [(-7571_i16),742_i16,24979_i16,(-25512_i16),(-20268_i16),22929_i16];
    _8 = -_4;
    RET = [26151_i16,(-4341_i16),(-5871_i16),32672_i16,(-23401_i16),(-16162_i16)];
    _7 = _1 - _6;
    RET = [25101_i16,(-7330_i16),22556_i16,(-5613_i16),28843_i16,12978_i16];
    _6 = _3;
    _6 = _1 & _7;
    RET = [(-31095_i16),15549_i16,(-3049_i16),14107_i16,2425_i16,(-31955_i16)];
    _8 = 42_isize as i128;
    _4 = _6 + _7;
    _2 = _6 & _6;
    Goto(bb1)
    }
    bb1 = {
    _6 = -_2;
    RET = [(-12728_i16),(-880_i16),(-23353_i16),29868_i16,22175_i16,29646_i16];
    _6 = _7 & _4;
    RET = [31328_i16,(-32280_i16),12058_i16,(-6436_i16),15395_i16,(-12213_i16)];
    _9 = -78_isize;
    _9 = (-9223372036854775808_isize);
    _10 = [(-1982105321_i32),(-1663974409_i32)];
    RET = [(-12719_i16),26814_i16,(-9863_i16),(-5698_i16),(-5661_i16),31378_i16];
    _2 = 308_u16 as i128;
    _3 = 232227528996250920210693403892973656713_u128 as i128;
    _6 = 3920355494351497547_u64 as i128;
    _9 = false as isize;
    Goto(bb2)
    }
    bb2 = {
    _11 = '\u{ef198}' as u16;
    _4 = (-24392_i16) as i128;
    _11 = 30537_u16;
    _2 = _7;
    _15.3 = (13353289947282390786_u64,);
    RET = [19400_i16,(-881_i16),(-28474_i16),540_i16,7785_i16,14113_i16];
    RET = [4780_i16,(-28085_i16),(-20079_i16),28252_i16,(-19093_i16),(-11425_i16)];
    _15.3.0 = 12582755471606247650_u64 >> _7;
    RET = [(-7946_i16),(-11769_i16),(-29537_i16),(-30570_i16),(-31482_i16),32716_i16];
    _6 = 1053906696_u32 as i128;
    _14 = 39_u8 as f64;
    _16 = [_11];
    Goto(bb3)
    }
    bb3 = {
    _15.2 = _1 as i64;
    _5 = [_11,_11,_11,_11,_11,_11];
    _15.0 = &_11;
    _15.2 = 937328828_i32 as i64;
    _12 = [_15.3.0,_15.3.0];
    _3 = !_1;
    _15.1 = Move(_15.0);
    _1 = _2 | _3;
    _8 = _7 << _7;
    Goto(bb4)
    }
    bb4 = {
    RET = [(-16809_i16),12279_i16,(-32687_i16),(-11835_i16),18916_i16,22849_i16];
    _16 = [_11];
    _5 = [_11,_11,_11,_11,_11,_11];
    _10 = [(-562523600_i32),(-187442086_i32)];
    RET = [(-9939_i16),(-18779_i16),(-18344_i16),31250_i16,9328_i16,(-30333_i16)];
    _15.3 = (13805432672072396612_u64,);
    _17 = Adt57::Variant0 { fld0: _9 };
    _15.0 = &_11;
    _9 = -Field::<isize>(Variant(_17, 0), 0);
    _15.0 = &_11;
    RET = [(-24268_i16),(-8818_i16),(-120_i16),2810_i16,19534_i16,16595_i16];
    _16 = [_11];
    _3 = _1;
    _19 = [2484951430_u32];
    _15.3 = (14408198540479979702_u64,);
    _5 = [_11,_11,_11,_11,_11,_11];
    _15.3 = (5327718182701346748_u64,);
    _15.3.0 = 201_u8 as u64;
    Goto(bb5)
    }
    bb5 = {
    _11 = !278_u16;
    Call(_4 = fn12(_1, _1, _2, _3, _3, _8, _7, _3), ReturnTo(bb6), UnwindUnreachable())
    }
    bb6 = {
    _16 = [_11];
    _11 = (-22_i8) as u16;
    _15.3 = (13048427514278133776_u64,);
    _15.2 = 1827673990991480441_i64;
    _15.1 = &_11;
    RET = [(-1143_i16),8429_i16,(-2153_i16),16906_i16,(-21732_i16),(-21876_i16)];
    _7 = _4 ^ _4;
    _19 = [745896193_u32];
    _1 = '\u{5ca67}' as i128;
    _1 = _11 as i128;
    _10 = [340426391_i32,(-1933904605_i32)];
    _16 = [_11];
    _15.0 = &_11;
    _5 = [_11,_11,_11,_11,_11,_11];
    _14 = 11106_i16 as f64;
    _13 = core::ptr::addr_of_mut!(_15.3.0);
    RET = [19969_i16,(-23233_i16),(-1231_i16),(-24847_i16),(-7485_i16),(-31393_i16)];
    _15.1 = &_11;
    match (*_13) {
    0 => bb1,
    1 => bb2,
    13048427514278133776 => bb8,
    _ => bb7
    }
    }
    bb7 = {
    _15.2 = _1 as i64;
    _5 = [_11,_11,_11,_11,_11,_11];
    _15.0 = &_11;
    _15.2 = 937328828_i32 as i64;
    _12 = [_15.3.0,_15.3.0];
    _3 = !_1;
    _15.1 = Move(_15.0);
    _1 = _2 | _3;
    _8 = _7 << _7;
    Goto(bb4)
    }
    bb8 = {
    _23 = _11 as usize;
    _21 = Field::<isize>(Variant(_17, 0), 0);
    _15.1 = &_11;
    _2 = _4 | _7;
    match _15.2 {
    0 => bb1,
    1 => bb2,
    2 => bb6,
    3 => bb9,
    1827673990991480441 => bb11,
    _ => bb10
    }
    }
    bb9 = {
    _11 = !278_u16;
    Call(_4 = fn12(_1, _1, _2, _3, _3, _8, _7, _3), ReturnTo(bb6), UnwindUnreachable())
    }
    bb10 = {
    _16 = [_11];
    _11 = (-22_i8) as u16;
    _15.3 = (13048427514278133776_u64,);
    _15.2 = 1827673990991480441_i64;
    _15.1 = &_11;
    RET = [(-1143_i16),8429_i16,(-2153_i16),16906_i16,(-21732_i16),(-21876_i16)];
    _7 = _4 ^ _4;
    _19 = [745896193_u32];
    _1 = '\u{5ca67}' as i128;
    _1 = _11 as i128;
    _10 = [340426391_i32,(-1933904605_i32)];
    _16 = [_11];
    _15.0 = &_11;
    _5 = [_11,_11,_11,_11,_11,_11];
    _14 = 11106_i16 as f64;
    _13 = core::ptr::addr_of_mut!(_15.3.0);
    RET = [19969_i16,(-23233_i16),(-1231_i16),(-24847_i16),(-7485_i16),(-31393_i16)];
    _15.1 = &_11;
    match (*_13) {
    0 => bb1,
    1 => bb2,
    13048427514278133776 => bb8,
    _ => bb7
    }
    }
    bb11 = {
    _7 = -_4;
    _22 = _15.2;
    _21 = _9;
    _15.1 = Move(_15.0);
    _4 = _2;
    _15.1 = &_11;
    _11 = 3536_u16 | 47698_u16;
    _3 = _4;
    _21 = !Field::<isize>(Variant(_17, 0), 0);
    _30 = '\u{c5c71}';
    _10 = [(-1119515107_i32),(-1413476225_i32)];
    _3 = 48_i8 as i128;
    _26 = _30 as i16;
    _12 = [_15.3.0,_15.3.0];
    _13 = core::ptr::addr_of_mut!(_15.3.0);
    place!(Field::<isize>(Variant(_17, 0), 0)) = _21 << _4;
    _11 = 57788_u16 | 16247_u16;
    (*_13) = !14910044561704975904_u64;
    _13 = core::ptr::addr_of_mut!(_25);
    (*_13) = _15.3.0 * _15.3.0;
    _5 = [_11,_11,_11,_11,_11,_11];
    _13 = core::ptr::addr_of_mut!(_15.3.0);
    _27 = _11 as f32;
    Goto(bb12)
    }
    bb12 = {
    RET = [_26,_26,_26,_26,_26,_26];
    _1 = _8 * _8;
    match _22 {
    0 => bb8,
    1 => bb5,
    2 => bb13,
    1827673990991480441 => bb15,
    _ => bb14
    }
    }
    bb13 = {
    _11 = !278_u16;
    Call(_4 = fn12(_1, _1, _2, _3, _3, _8, _7, _3), ReturnTo(bb6), UnwindUnreachable())
    }
    bb14 = {
    _11 = '\u{ef198}' as u16;
    _4 = (-24392_i16) as i128;
    _11 = 30537_u16;
    _2 = _7;
    _15.3 = (13353289947282390786_u64,);
    RET = [19400_i16,(-881_i16),(-28474_i16),540_i16,7785_i16,14113_i16];
    RET = [4780_i16,(-28085_i16),(-20079_i16),28252_i16,(-19093_i16),(-11425_i16)];
    _15.3.0 = 12582755471606247650_u64 >> _7;
    RET = [(-7946_i16),(-11769_i16),(-29537_i16),(-30570_i16),(-31482_i16),32716_i16];
    _6 = 1053906696_u32 as i128;
    _14 = 39_u8 as f64;
    _16 = [_11];
    Goto(bb3)
    }
    bb15 = {
    _27 = _26 as f32;
    SetDiscriminant(_17, 1);
    _31 = 3659878991_u32 * 3827342270_u32;
    place!(Field::<[u16; 1]>(Variant(_17, 1), 1)) = _16;
    _14 = _31 as f64;
    _19 = [_31];
    _6 = !_7;
    RET = [_26,_26,_26,_26,_26,_26];
    place!(Field::<u128>(Variant(_17, 1), 3)) = _27 as u128;
    _11 = 43498_u16 >> _2;
    _29 = _30;
    place!(Field::<[u16; 1]>(Variant(_17, 1), 1)) = [_11];
    Goto(bb16)
    }
    bb16 = {
    Call(_36 = dump_var(11_usize, 6_usize, Move(_6), 22_usize, Move(_22), 29_usize, Move(_29), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_36 = dump_var(11_usize, 3_usize, Move(_3), 1_usize, Move(_1), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Call(_36 = dump_var(11_usize, 25_usize, Move(_25), 9_usize, Move(_9), 26_usize, Move(_26), 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(
    mut _1: i128,
    mut _2: i128,
    mut _3: i128,
    mut _4: i128,
    mut _5: i128,
    mut _6: i128,
    mut _7: i128,
    mut _8: i128,
) -> i128 {
    mir! {
    type RET = i128;
    let _9: (usize, i16);
    let _10: isize;
    let _11: [usize; 3];
    let _12: [isize; 3];
    let _13: ();
    let _14: ();
    {
    _5 = !_4;
    _2 = !_7;
    RET = -_8;
    RET = _7 ^ _6;
    _1 = -_7;
    _7 = 7608447726099297074_i64 as i128;
    _9 = (3_usize, 10872_i16);
    RET = -_3;
    _9 = (5003848826339903674_usize, 11139_i16);
    _5 = _6 | _8;
    _12 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
    Goto(bb1)
    }
    bb1 = {
    Call(_13 = dump_var(12_usize, 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    Call(_13 = dump_var(12_usize, 9_usize, Move(_9), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
    }
    bb3 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(
    mut _1: i128,
    mut _2: *const f32,
    mut _3: i128,
    mut _4: f32,
    mut _5: i128,
    mut _6: f64,
    mut _7: i128,
    mut _8: i128,
    mut _9: *const f32,
    mut _10: (u64,),
    mut _11: i128,
    mut _12: f32,
) -> [i32; 2] {
    mir! {
    type RET = [i32; 2];
    let _13: Adt45;
    let _14: Adt53;
    let _15: [isize; 3];
    let _16: Adt51;
    let _17: (i8, isize, [i32; 2]);
    let _18: bool;
    let _19: bool;
    let _20: Adt48;
    let _21: f32;
    let _22: Adt60;
    let _23: [u32; 6];
    let _24: u128;
    let _25: Adt60;
    let _26: (usize, i16);
    let _27: [u32; 1];
    let _28: Adt46;
    let _29: u128;
    let _30: [i8; 4];
    let _31: u32;
    let _32: Adt57;
    let _33: Adt45;
    let _34: [u32; 6];
    let _35: [i32; 2];
    let _36: i8;
    let _37: bool;
    let _38: i8;
    let _39: f32;
    let _40: (u64,);
    let _41: *mut u64;
    let _42: [u64; 6];
    let _43: [i16; 6];
    let _44: char;
    let _45: char;
    let _46: ();
    let _47: ();
    {
    _12 = (-13222_i16) as f32;
    _13.fld4 = _1;
    _13.fld2 = (*_2) as f64;
    _13.fld0 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
    _10.0 = !2649420980804701839_u64;
    _13.fld2 = _6 + _6;
    _13.fld1 = !2466622108_u32;
    (*_2) = _4 + _4;
    _1 = _11;
    _9 = _2;
    (*_9) = _4 - _4;
    _9 = core::ptr::addr_of!(_4);
    _4 = 35786_u16 as f32;
    RET = [2074244437_i32,(-1707429112_i32)];
    _7 = -_8;
    _6 = _13.fld2;
    _13.fld4 = !_11;
    _6 = _8 as f64;
    Goto(bb1)
    }
    bb1 = {
    _13.fld4 = !_3;
    _15 = [(-9223372036854775808_isize),79_isize,(-9223372036854775808_isize)];
    RET = [(-398157395_i32),(-466812464_i32)];
    RET = [445867317_i32,(-1256371574_i32)];
    _4 = 4477036104690678892207518052600344481_u128 as f32;
    _13.fld6 = 42479_u16;
    _18 = _1 == _1;
    _13.fld3 = core::ptr::addr_of_mut!(_17);
    _13.fld5 = ['\u{2742d}','\u{d6dda}','\u{6b3e1}','\u{9d1c2}','\u{e1941}','\u{97f2e}','\u{d4d67}'];
    _17.1 = (-76_isize) ^ (-99_isize);
    _3 = _11;
    _20 = Adt48 { fld0: 230916170379271553781355286579822410292_u128 };
    _20.fld0 = 319085712923797088857215979626271556980_u128 * 241622435382281862348149103499530020473_u128;
    _17.0 = 5_i8;
    Call(_17.1 = core::intrinsics::transmute(RET), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    _9 = core::ptr::addr_of!((*_9));
    Call(_16 = fn14(_2, (*_2)), ReturnTo(bb3), UnwindUnreachable())
    }
    bb3 = {
    SetDiscriminant(_16, 3);
    place!(Field::<[u32; 1]>(Variant(_16, 3), 1)) = [_13.fld1];
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.1 = _17.1;
    _5 = _3 << _7;
    _12 = (*_2);
    _26 = (3742874574083289497_usize, 3229_i16);
    _2 = core::ptr::addr_of!((*_2));
    _17.2 = RET;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).3 = [_26.1,_26.1,_26.1,_26.1,_26.1,_26.1];
    (*_9) = (*_2);
    _17.0 = 55_i8 * 15_i8;
    (*_9) = (*_2);
    _17.1 = !Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1;
    _10.0 = !11735820780262577102_u64;
    _17.0 = (-106_i8);
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.0 = _17.0 ^ _17.0;
    place!(Field::<u64>(Variant(_16, 3), 3)) = _10.0;
    _21 = (*_2) * _12;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).0 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
    RET = _17.2;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = [_17.1,_17.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1];
    place!(Field::<[u64; 2]>(Variant(_16, 3), 7)) = [Field::<u64>(Variant(_16, 3), 3),_10.0];
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.2 = _17.2;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = _15;
    _26 = (18424986576568404600_usize, (-1220_i16));
    _27 = Field::<[u32; 1]>(Variant(_16, 3), 1);
    Goto(bb4)
    }
    bb4 = {
    _26.0 = 6367095677913542078_usize;
    RET = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.2;
    _13.fld2 = -_6;
    _19 = _18 | _18;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.1 = _17.1;
    _3 = _8 ^ _13.fld4;
    _26 = (3_usize, 31215_i16);
    _13.fld2 = _17.0 as f64;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.2 = RET;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.1 = _17.1 ^ _17.1;
    _20 = Adt48 { fld0: 244364396593278315503132910777339169524_u128 };
    _6 = _13.fld2;
    _18 = !_19;
    _17.1 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1;
    _13.fld4 = _1 * _1;
    _24 = !_20.fld0;
    place!(Field::<[u32; 1]>(Variant(_16, 3), 1)) = _27;
    _23 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
    _33.fld5 = ['\u{8ba7a}','\u{2a74f}','\u{10170b}','\u{c703e}','\u{7459}','\u{af88}','\u{ad0c8}'];
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).2 = core::ptr::addr_of!(_4);
    Call(place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.0 = core::intrinsics::bswap(_17.0), ReturnTo(bb5), UnwindUnreachable())
    }
    bb5 = {
    _13.fld0 = [Field::<u64>(Variant(_16, 3), 3),_10.0,Field::<u64>(Variant(_16, 3), 3),_10.0,_10.0,Field::<u64>(Variant(_16, 3), 3)];
    _20.fld0 = !_24;
    _33.fld6 = 49_u8 as u16;
    place!(Field::<[i8; 4]>(Variant(_16, 3), 5)) = [_17.0,_17.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0];
    place!(Field::<[u64; 2]>(Variant(_16, 3), 7)) = [Field::<u64>(Variant(_16, 3), 3),_10.0];
    place!(Field::<[u32; 1]>(Variant(_16, 3), 1)) = _27;
    _30 = [Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0];
    (*_9) = (*_2);
    _6 = _13.fld2;
    _33 = _13;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = [Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1,_17.1,_17.1];
    _13.fld6 = !_33.fld6;
    _33.fld0 = [Field::<u64>(Variant(_16, 3), 3),_10.0,Field::<u64>(Variant(_16, 3), 3),Field::<u64>(Variant(_16, 3), 3),_10.0,Field::<u64>(Variant(_16, 3), 3)];
    _34 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).0;
    _18 = !_19;
    _13.fld3 = core::ptr::addr_of_mut!(_17);
    _13.fld5 = ['\u{6c44c}','\u{2ce76}','\u{c2df2}','\u{1012f5}','\u{7411a}','\u{10ef50}','\u{80033}'];
    _8 = _33.fld4 >> _11;
    place!(Field::<[i8; 4]>(Variant(_16, 3), 5)) = _30;
    _13 = Adt45 { fld0: _33.fld0,fld1: _33.fld1,fld2: _33.fld2,fld3: _33.fld3,fld4: _1,fld5: _33.fld5,fld6: _33.fld6 };
    _33.fld3 = core::ptr::addr_of_mut!(_17);
    _13.fld4 = _26.1 as i128;
    place!(Field::<f64>(Variant(_16, 3), 0)) = -_33.fld2;
    _33 = Adt45 { fld0: _13.fld0,fld1: _13.fld1,fld2: _13.fld2,fld3: _13.fld3,fld4: _11,fld5: _13.fld5,fld6: _13.fld6 };
    _36 = -_17.0;
    Goto(bb6)
    }
    bb6 = {
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.2 = [(-757826894_i32),296465017_i32];
    _34 = _23;
    _34 = [_13.fld1,_13.fld1,_33.fld1,_33.fld1,_13.fld1,_13.fld1];
    _18 = _19 & _19;
    _15 = [_17.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1];
    _29 = _24 >> _1;
    _4 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0 as f32;
    _33.fld1 = _13.fld1;
    place!(Field::<u64>(Variant(_16, 3), 3)) = _13.fld1 as u64;
    _37 = _18 <= _18;
    _8 = _1;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).4 = Field::<[u64; 2]>(Variant(_16, 3), 7);
    _9 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).2;
    _35 = [(-1648314598_i32),1393649126_i32];
    Goto(bb7)
    }
    bb7 = {
    _42 = [_10.0,_10.0,_10.0,Field::<u64>(Variant(_16, 3), 3),Field::<u64>(Variant(_16, 3), 3),_10.0];
    (*_9) = _21;
    _17.0 = !Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0;
    (*_2) = _4;
    _41 = core::ptr::addr_of_mut!(_40.0);
    place!(Field::<f64>(Variant(_16, 3), 0)) = _29 as f64;
    match _26.1 {
    0 => bb8,
    1 => bb9,
    2 => bb10,
    3 => bb11,
    4 => bb12,
    5 => bb13,
    31215 => bb15,
    _ => bb14
    }
    }
    bb8 = {
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.2 = [(-757826894_i32),296465017_i32];
    _34 = _23;
    _34 = [_13.fld1,_13.fld1,_33.fld1,_33.fld1,_13.fld1,_13.fld1];
    _18 = _19 & _19;
    _15 = [_17.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1];
    _29 = _24 >> _1;
    _4 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0 as f32;
    _33.fld1 = _13.fld1;
    place!(Field::<u64>(Variant(_16, 3), 3)) = _13.fld1 as u64;
    _37 = _18 <= _18;
    _8 = _1;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).4 = Field::<[u64; 2]>(Variant(_16, 3), 7);
    _9 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).2;
    _35 = [(-1648314598_i32),1393649126_i32];
    Goto(bb7)
    }
    bb9 = {
    _13.fld0 = [Field::<u64>(Variant(_16, 3), 3),_10.0,Field::<u64>(Variant(_16, 3), 3),_10.0,_10.0,Field::<u64>(Variant(_16, 3), 3)];
    _20.fld0 = !_24;
    _33.fld6 = 49_u8 as u16;
    place!(Field::<[i8; 4]>(Variant(_16, 3), 5)) = [_17.0,_17.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0];
    place!(Field::<[u64; 2]>(Variant(_16, 3), 7)) = [Field::<u64>(Variant(_16, 3), 3),_10.0];
    place!(Field::<[u32; 1]>(Variant(_16, 3), 1)) = _27;
    _30 = [Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0];
    (*_9) = (*_2);
    _6 = _13.fld2;
    _33 = _13;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = [Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1,_17.1,_17.1];
    _13.fld6 = !_33.fld6;
    _33.fld0 = [Field::<u64>(Variant(_16, 3), 3),_10.0,Field::<u64>(Variant(_16, 3), 3),Field::<u64>(Variant(_16, 3), 3),_10.0,Field::<u64>(Variant(_16, 3), 3)];
    _34 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).0;
    _18 = !_19;
    _13.fld3 = core::ptr::addr_of_mut!(_17);
    _13.fld5 = ['\u{6c44c}','\u{2ce76}','\u{c2df2}','\u{1012f5}','\u{7411a}','\u{10ef50}','\u{80033}'];
    _8 = _33.fld4 >> _11;
    place!(Field::<[i8; 4]>(Variant(_16, 3), 5)) = _30;
    _13 = Adt45 { fld0: _33.fld0,fld1: _33.fld1,fld2: _33.fld2,fld3: _33.fld3,fld4: _1,fld5: _33.fld5,fld6: _33.fld6 };
    _33.fld3 = core::ptr::addr_of_mut!(_17);
    _13.fld4 = _26.1 as i128;
    place!(Field::<f64>(Variant(_16, 3), 0)) = -_33.fld2;
    _33 = Adt45 { fld0: _13.fld0,fld1: _13.fld1,fld2: _13.fld2,fld3: _13.fld3,fld4: _11,fld5: _13.fld5,fld6: _13.fld6 };
    _36 = -_17.0;
    Goto(bb6)
    }
    bb10 = {
    _26.0 = 6367095677913542078_usize;
    RET = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.2;
    _13.fld2 = -_6;
    _19 = _18 | _18;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.1 = _17.1;
    _3 = _8 ^ _13.fld4;
    _26 = (3_usize, 31215_i16);
    _13.fld2 = _17.0 as f64;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.2 = RET;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.1 = _17.1 ^ _17.1;
    _20 = Adt48 { fld0: 244364396593278315503132910777339169524_u128 };
    _6 = _13.fld2;
    _18 = !_19;
    _17.1 = Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1;
    _13.fld4 = _1 * _1;
    _24 = !_20.fld0;
    place!(Field::<[u32; 1]>(Variant(_16, 3), 1)) = _27;
    _23 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
    _33.fld5 = ['\u{8ba7a}','\u{2a74f}','\u{10170b}','\u{c703e}','\u{7459}','\u{af88}','\u{ad0c8}'];
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).2 = core::ptr::addr_of!(_4);
    Call(place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.0 = core::intrinsics::bswap(_17.0), ReturnTo(bb5), UnwindUnreachable())
    }
    bb11 = {
    SetDiscriminant(_16, 3);
    place!(Field::<[u32; 1]>(Variant(_16, 3), 1)) = [_13.fld1];
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.1 = _17.1;
    _5 = _3 << _7;
    _12 = (*_2);
    _26 = (3742874574083289497_usize, 3229_i16);
    _2 = core::ptr::addr_of!((*_2));
    _17.2 = RET;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).3 = [_26.1,_26.1,_26.1,_26.1,_26.1,_26.1];
    (*_9) = (*_2);
    _17.0 = 55_i8 * 15_i8;
    (*_9) = (*_2);
    _17.1 = !Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1;
    _10.0 = !11735820780262577102_u64;
    _17.0 = (-106_i8);
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.0 = _17.0 ^ _17.0;
    place!(Field::<u64>(Variant(_16, 3), 3)) = _10.0;
    _21 = (*_2) * _12;
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).0 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
    RET = _17.2;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = [_17.1,_17.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1];
    place!(Field::<[u64; 2]>(Variant(_16, 3), 7)) = [Field::<u64>(Variant(_16, 3), 3),_10.0];
    place!(Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2)).1.2 = _17.2;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = _15;
    _26 = (18424986576568404600_usize, (-1220_i16));
    _27 = Field::<[u32; 1]>(Variant(_16, 3), 1);
    Goto(bb4)
    }
    bb12 = {
    _9 = core::ptr::addr_of!((*_9));
    Call(_16 = fn14(_2, (*_2)), ReturnTo(bb3), UnwindUnreachable())
    }
    bb13 = {
    _13.fld4 = !_3;
    _15 = [(-9223372036854775808_isize),79_isize,(-9223372036854775808_isize)];
    RET = [(-398157395_i32),(-466812464_i32)];
    RET = [445867317_i32,(-1256371574_i32)];
    _4 = 4477036104690678892207518052600344481_u128 as f32;
    _13.fld6 = 42479_u16;
    _18 = _1 == _1;
    _13.fld3 = core::ptr::addr_of_mut!(_17);
    _13.fld5 = ['\u{2742d}','\u{d6dda}','\u{6b3e1}','\u{9d1c2}','\u{e1941}','\u{97f2e}','\u{d4d67}'];
    _17.1 = (-76_isize) ^ (-99_isize);
    _3 = _11;
    _20 = Adt48 { fld0: 230916170379271553781355286579822410292_u128 };
    _20.fld0 = 319085712923797088857215979626271556980_u128 * 241622435382281862348149103499530020473_u128;
    _17.0 = 5_i8;
    Call(_17.1 = core::intrinsics::transmute(RET), ReturnTo(bb2), UnwindUnreachable())
    }
    bb14 = {
    Return()
    }
    bb15 = {
    _40 = (_10.0,);
    (*_2) = (*_9);
    _26 = (16816199655110410219_usize, (-4940_i16));
    _37 = !_18;
    _11 = _5 & _3;
    place!(Field::<[isize; 3]>(Variant(_16, 3), 6)) = [_17.1,Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.1,_17.1];
    _31 = _33.fld1 | _13.fld1;
    _17.0 = -Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_16, 3), 2).1.0;
    place!(Field::<u128>(Variant(_16, 3), 4)) = 1169953796_i32 as u128;
    _4 = _26.0 as f32;
    _24 = _29 * _29;
    _27 = [_13.fld1];
    _18 = !_37;
    place!(Field::<u128>(Variant(_16, 3), 4)) = _11 as u128;
    _42 = _33.fld0;
    SetDiscriminant(_16, 1);
    _17.0 = _36;
    _13.fld5 = _33.fld5;
    _31 = _33.fld1 + _33.fld1;
    _17.0 = !_36;
    place!(Field::<(i64, *mut (i8, isize, [i32; 2]))>(Variant(_16, 1), 4)).1 = _13.fld3;
    _27 = [_33.fld1];
    place!(Field::<(u64,)>(Variant(_16, 1), 0)).0 = _40.0;
    _35 = [(-323140731_i32),1509549410_i32];
    _5 = _8 << _7;
    _13 = _33;
    _34 = [_31,_13.fld1,_13.fld1,_31,_31,_31];
    _24 = _29;
    place!(Field::<[i16; 6]>(Variant(_16, 1), 2)) = [_26.1,_26.1,_26.1,_26.1,_26.1,_26.1];
    Goto(bb16)
    }
    bb16 = {
    Call(_46 = dump_var(13_usize, 19_usize, Move(_19), 40_usize, Move(_40), 1_usize, Move(_1), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_46 = dump_var(13_usize, 11_usize, Move(_11), 29_usize, Move(_29), 30_usize, Move(_30), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Call(_46 = dump_var(13_usize, 10_usize, Move(_10), 18_usize, Move(_18), 42_usize, Move(_42), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *const f32, mut _2: f32) -> Adt51 {
    mir! {
    type RET = Adt51;
    let _3: [i8; 4];
    let _4: &'static u16;
    let _5: bool;
    let _6: [i8; 4];
    let _7: (u64,);
    let _8: i64;
    let _9: i16;
    let _10: [usize; 3];
    let _11: [u16; 1];
    let _12: Adt57;
    let _13: isize;
    let _14: [usize; 3];
    let _15: i8;
    let _16: [i8; 4];
    let _17: Adt48;
    let _18: char;
    let _19: isize;
    let _20: [u32; 1];
    let _21: [u16; 6];
    let _22: u128;
    let _23: f64;
    let _24: isize;
    let _25: isize;
    let _26: Adt50;
    let _27: f32;
    let _28: [u16; 1];
    let _29: bool;
    let _30: Adt51;
    let _31: (u64,);
    let _32: isize;
    let _33: isize;
    let _34: ();
    let _35: ();
    {
    (*_1) = (-25743_i16) as f32;
    (*_1) = 67_u8 as f32;
    _1 = core::ptr::addr_of!(_2);
    _2 = 1356030466_u32 as f32;
    (*_1) = (-5603883_i32) as f32;
    (*_1) = 28_u8 as f32;
    (*_1) = 181_u8 as f32;
    (*_1) = 7348647392720940775_usize as f32;
    _1 = core::ptr::addr_of!((*_1));
    (*_1) = 5387746221920276380927366978671774992_u128 as f32;
    _1 = core::ptr::addr_of!((*_1));
    (*_1) = 59_i8 as f32;
    (*_1) = 12548322258289073850_usize as f32;
    Goto(bb1)
    }
    bb1 = {
    _2 = (-6546210655611176940_i64) as f32;
    (*_1) = 3963160296_u32 as f32;
    (*_1) = 216518851022474119999865235866514419831_u128 as f32;
    _5 = !false;
    _3 = [103_i8,35_i8,(-36_i8),6_i8];
    (*_1) = 3658290056_u32 as f32;
    (*_1) = (-613195026_i32) as f32;
    (*_1) = 9223372036854775807_isize as f32;
    _2 = 9223372036854775807_isize as f32;
    _2 = 7593005565846802648_i64 as f32;
    _6 = [115_i8,(-47_i8),20_i8,44_i8];
    _2 = 2_usize as f32;
    _7 = (9533681651950081296_u64,);
    _7.0 = 13132604744244250823_u64;
    _3 = _6;
    _6 = _3;
    _2 = 6468080263118317799_i64 as f32;
    _2 = 9223372036854775807_isize as f32;
    (*_1) = (-93_i8) as f32;
    _5 = false & true;
    _8 = -7954880152924121766_i64;
    (*_1) = 1002639704_u32 as f32;
    (*_1) = (-9223372036854775808_isize) as f32;
    _6 = _3;
    Goto(bb2)
    }
    bb2 = {
    (*_1) = 723090983_i32 as f32;
    _3 = _6;
    _10 = [2_usize,12504707825948881170_usize,4_usize];
    _1 = core::ptr::addr_of!(_2);
    _2 = _7.0 as f32;
    _12 = Adt57::Variant0 { fld0: (-9223372036854775808_isize) };
    _10 = [18181240803994997982_usize,2523532650744410174_usize,2_usize];
    _9 = (*_1) as i16;
    _3 = _6;
    _9 = 13296_u16 as i16;
    place!(Field::<isize>(Variant(_12, 0), 0)) = 7_isize;
    _1 = core::ptr::addr_of!(_2);
    place!(Field::<isize>(Variant(_12, 0), 0)) = 9223372036854775807_isize;
    _2 = _7.0 as f32;
    _8 = (-7194645651269610342_i64) & 5804033140961251338_i64;
    _5 = true | true;
    _2 = Field::<isize>(Variant(_12, 0), 0) as f32;
    (*_1) = 50134_u16 as f32;
    _2 = 212_u8 as f32;
    _8 = !(-8257922947752563895_i64);
    _6 = _3;
    (*_1) = 3_usize as f32;
    _2 = _7.0 as f32;
    (*_1) = 421650997_i32 as f32;
    _7.0 = _5 as u64;
    _8 = -1561890245560642263_i64;
    _10 = [6_usize,8201451797824720897_usize,1188293714234836952_usize];
    _5 = false;
    _11 = [64744_u16];
    place!(Field::<isize>(Variant(_12, 0), 0)) = !9223372036854775807_isize;
    Goto(bb3)
    }
    bb3 = {
    _14 = [10982089301118576156_usize,16033747806715121045_usize,6893930746115607984_usize];
    _10 = [0_usize,5_usize,961591312106404919_usize];
    _7 = (7320909291293564545_u64,);
    _5 = false | true;
    _7.0 = !6979680148660177050_u64;
    Call((*_1) = fn15(_3, _1, _14, _10, _9, _10, _10, _5, _6, _1, _6, _6, _10, _5), ReturnTo(bb4), UnwindUnreachable())
    }
    bb4 = {
    _15 = 67_i8 | (-87_i8);
    _13 = Field::<isize>(Variant(_12, 0), 0);
    _8 = (-5058600112527921181_i64);
    _7.0 = !10128563659403651713_u64;
    (*_1) = 11067805094478011310_usize as f32;
    _3 = [_15,_15,_15,_15];
    _3 = [_15,_15,_15,_15];
    _7 = (3611125809344470393_u64,);
    Goto(bb5)
    }
    bb5 = {
    (*_1) = 6_usize as f32;
    _10 = [3423160766584337904_usize,4135400509690571946_usize,3941345065635975002_usize];
    _2 = 16173_u16 as f32;
    match _7.0 {
    3611125809344470393 => bb6,
    _ => bb1
    }
    }
    bb6 = {
    _6 = [_15,_15,_15,_15];
    _9 = (-6995_i16) * 11317_i16;
    _3 = _6;
    _6 = [_15,_15,_15,_15];
    SetDiscriminant(_12, 3);
    _7 = (9924448543409338408_u64,);
    _17 = Adt48 { fld0: 135527642251541924305476855338879663033_u128 };
    place!(Field::<Adt49>(Variant(_12, 3), 2)) = Adt49::Variant1 { fld0: 1_usize,fld1: _7 };
    _16 = [_15,_15,_15,_15];
    _14 = _10;
    _2 = 2729_u16 as f32;
    place!(Field::<(u64,)>(Variant(place!(Field::<Adt49>(Variant(_12, 3), 2)), 1), 1)).0 = _7.0;
    _11 = [62394_u16];
    _1 = core::ptr::addr_of!((*_1));
    place!(Field::<u64>(Variant(_12, 3), 1)) = (-144387937461328619429501105476840031272_i128) as u64;
    _12 = Adt57::Variant0 { fld0: _13 };
    _1 = core::ptr::addr_of!((*_1));
    _7.0 = _5 as u64;
    _1 = core::ptr::addr_of!(_2);
    _6 = _3;
    _17 = Adt48 { fld0: 100461820387874785849831580400608830695_u128 };
    Goto(bb7)
    }
    bb7 = {
    _16 = [_15,_15,_15,_15];
    place!(Field::<isize>(Variant(_12, 0), 0)) = _7.0 as isize;
    Goto(bb8)
    }
    bb8 = {
    _17 = Adt48 { fld0: 146360180668039934654199452513903862574_u128 };
    _6 = [_15,_15,_15,_15];
    _20 = [1634911880_u32];
    _11 = [41726_u16];
    _18 = '\u{5ac9}';
    (*_1) = _9 as f32;
    _7 = (14003846801937048136_u64,);
    _19 = _13 & Field::<isize>(Variant(_12, 0), 0);
    _20 = [622798839_u32];
    _16 = [_15,_15,_15,_15];
    (*_1) = 74819025380825562125275105427696580446_i128 as f32;
    _8 = _7.0 as i64;
    SetDiscriminant(_12, 2);
    place!(Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0)).1 = _19 + _19;
    place!(Field::<u32>(Variant(_12, 2), 1)) = 3791771514_u32 & 157310807_u32;
    _20 = [Field::<u32>(Variant(_12, 2), 1)];
    match _17.fld0 {
    146360180668039934654199452513903862574 => bb10,
    _ => bb9
    }
    }
    bb9 = {
    _16 = [_15,_15,_15,_15];
    place!(Field::<isize>(Variant(_12, 0), 0)) = _7.0 as isize;
    Goto(bb8)
    }
    bb10 = {
    _12 = Adt57::Variant0 { fld0: _13 };
    (*_1) = _17.fld0 as f32;
    _15 = (-58_i8) << _9;
    Goto(bb11)
    }
    bb11 = {
    _21 = [53240_u16,56328_u16,11064_u16,36340_u16,26650_u16,13276_u16];
    _20 = [1744293406_u32];
    _1 = core::ptr::addr_of!((*_1));
    _22 = !_17.fld0;
    (*_1) = _8 as f32;
    SetDiscriminant(_12, 2);
    place!(Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0)).0 = -_15;
    place!(Field::<u32>(Variant(_12, 2), 1)) = Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0).0 as u32;
    place!(Field::<Adt48>(Variant(_12, 2), 4)) = _17;
    _23 = _13 as f64;
    _3 = _16;
    _2 = Field::<u32>(Variant(_12, 2), 1) as f32;
    _25 = _19 - _19;
    place!(Field::<u32>(Variant(_12, 2), 1)) = _18 as u32;
    _7 = (14862514317315804114_u64,);
    place!(Field::<i32>(Variant(_12, 2), 5)) = Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0).0 as i32;
    (*_1) = _22 as f32;
    place!(Field::<(f32, *mut [usize; 3], [i16; 6])>(Variant(_12, 2), 6)).2 = [_9,_9,_9,_9,_9,_9];
    _18 = '\u{10cb63}';
    place!(Field::<(f32, *mut [usize; 3], [i16; 6])>(Variant(_12, 2), 6)).1 = core::ptr::addr_of_mut!(_10);
    place!(Field::<(f32, *mut [usize; 3], [i16; 6])>(Variant(_12, 2), 6)).2 = [_9,_9,_9,_9,_9,_9];
    _20 = [Field::<u32>(Variant(_12, 2), 1)];
    place!(Field::<Adt48>(Variant(_12, 2), 4)) = _17;
    _19 = -_25;
    match Field::<Adt48>(Variant(_12, 2), 4).fld0 {
    0 => bb1,
    1 => bb2,
    2 => bb3,
    146360180668039934654199452513903862574 => bb12,
    _ => bb7
    }
    }
    bb12 = {
    _16 = [Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0).0,_15,_15,_15];
    _27 = _2;
    _19 = _15 as isize;
    (*_1) = 96157572784179786048516515399155487106_i128 as f32;
    place!(Field::<[u16; 6]>(Variant(_12, 2), 3)) = [28168_u16,6316_u16,25150_u16,19180_u16,47007_u16,35081_u16];
    (*_1) = _27;
    (*_1) = _27 * _27;
    place!(Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0)).2 = [Field::<i32>(Variant(_12, 2), 5),Field::<i32>(Variant(_12, 2), 5)];
    _27 = -(*_1);
    _27 = _2;
    _19 = _7.0 as isize;
    place!(Field::<u128>(Variant(_12, 2), 2)) = _17.fld0;
    _3 = [_15,_15,Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0).0,Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0).0];
    Goto(bb13)
    }
    bb13 = {
    (*_1) = _27;
    _21 = Field::<[u16; 6]>(Variant(_12, 2), 3);
    _21 = [39886_u16,18315_u16,39886_u16,43158_u16,532_u16,42810_u16];
    _24 = _13 + _25;
    _7 = (15501598105901819766_u64,);
    _7.0 = 8870647507119752223_u64 ^ 11287564106340717685_u64;
    _19 = _9 as isize;
    place!(Field::<Adt48>(Variant(_12, 2), 4)) = Adt48 { fld0: Field::<u128>(Variant(_12, 2), 2) };
    place!(Field::<[u16; 6]>(Variant(_12, 2), 3)) = [52521_u16,22323_u16,22853_u16,26269_u16,4678_u16,43591_u16];
    place!(Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0)).0 = _15;
    place!(Field::<Adt48>(Variant(_12, 2), 4)) = Adt48 { fld0: _22 };
    _17.fld0 = _22 + _22;
    _20 = [Field::<u32>(Variant(_12, 2), 1)];
    Goto(bb14)
    }
    bb14 = {
    _9 = (-15199_i16) << _22;
    _7 = (3673660764919471753_u64,);
    _33 = _8 as isize;
    _13 = _24;
    _17 = Adt48 { fld0: Field::<u128>(Variant(_12, 2), 2) };
    _9 = Field::<Adt48>(Variant(_12, 2), 4).fld0 as i16;
    Call(_30 = fn19(_13, _21, Field::<u32>(Variant(_12, 2), 1), _21, Field::<[u16; 6]>(Variant(_12, 2), 3), (*_1), Field::<i32>(Variant(_12, 2), 5), _7.0, _25, (*_1), _7, Field::<u32>(Variant(_12, 2), 1), _14, _27), ReturnTo(bb15), UnwindUnreachable())
    }
    bb15 = {
    _17.fld0 = _5 as u128;
    place!(Field::<Adt51>(Variant(_12, 2), 7)) = Adt51::Variant3 { fld0: Field::<f64>(Variant(_30, 3), 0),fld1: Field::<[u32; 1]>(Variant(_30, 3), 1),fld2: Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_30, 3), 2),fld3: Field::<u64>(Variant(_30, 3), 3),fld4: Field::<u128>(Variant(_30, 3), 4),fld5: _3,fld6: Field::<[isize; 3]>(Variant(_30, 3), 6),fld7: Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_30, 3), 2).4 };
    place!(Field::<(i8, isize, [i32; 2])>(Variant(_12, 2), 0)).0 = -_15;
    _18 = '\u{98ae6}';
    place!(Field::<(f32, *mut [usize; 3], [i16; 6])>(Variant(_12, 2), 6)).0 = (*_1);
    _29 = Field::<u128>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 4) > Field::<u128>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 4);
    _14 = _10;
    _23 = Field::<f64>(Variant(_30, 3), 0) - Field::<f64>(Variant(_30, 3), 0);
    _14 = [8217220514219096231_usize,16337997711272219726_usize,7_usize];
    RET = Adt51::Variant3 { fld0: Field::<f64>(Variant(_30, 3), 0),fld1: Field::<[u32; 1]>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 1),fld2: Field::<([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2])>(Variant(_30, 3), 2),fld3: Field::<u64>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 3),fld4: Field::<u128>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 4),fld5: Field::<[i8; 4]>(Variant(_30, 3), 5),fld6: Field::<[isize; 3]>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 6),fld7: Field::<[u64; 2]>(Variant(Field::<Adt51>(Variant(_12, 2), 7), 3), 7) };
    Goto(bb16)
    }
    bb16 = {
    Call(_34 = dump_var(14_usize, 16_usize, Move(_16), 10_usize, Move(_10), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_34 = dump_var(14_usize, 19_usize, Move(_19), 11_usize, Move(_11), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Call(_34 = dump_var(14_usize, 22_usize, Move(_22), 29_usize, Move(_29), 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(
    mut _1: [i8; 4],
    mut _2: *const f32,
    mut _3: [usize; 3],
    mut _4: [usize; 3],
    mut _5: i16,
    mut _6: [usize; 3],
    mut _7: [usize; 3],
    mut _8: bool,
    mut _9: [i8; 4],
    mut _10: *const f32,
    mut _11: [i8; 4],
    mut _12: [i8; 4],
    mut _13: [usize; 3],
    mut _14: bool,
) -> f32 {
    mir! {
    type RET = f32;
    let _15: isize;
    let _16: Adt49;
    let _17: u8;
    let _18: [usize; 3];
    let _19: u32;
    let _20: [usize; 3];
    let _21: [i16; 6];
    let _22: f64;
    let _23: Adt52;
    let _24: *mut [i16; 6];
    let _25: [usize; 3];
    let _26: (usize, i16);
    let _27: f64;
    let _28: usize;
    let _29: i16;
    let _30: ();
    let _31: ();
    {
    _7 = [7_usize,4916551452868077030_usize,1_usize];
    _6 = [10793397769772971787_usize,1_usize,11417688326533134353_usize];
    _5 = 2170_i16 | (-28064_i16);
    _2 = _10;
    _9 = [(-81_i8),(-7_i8),(-112_i8),48_i8];
    _11 = [124_i8,(-15_i8),(-55_i8),102_i8];
    _3 = [12072059881585091719_usize,9514878778478143468_usize,2203264490381150234_usize];
    _1 = [85_i8,74_i8,(-97_i8),(-82_i8)];
    _5 = _8 as i16;
    _11 = _1;
    _2 = core::ptr::addr_of!(RET);
    _15 = (-9223372036854775808_isize) * (-77_isize);
    _14 = _15 == _15;
    _2 = _10;
    RET = 2608117627155476959_usize as f32;
    _5 = (-635_i16);
    _12 = _1;
    _13 = [3_usize,2_usize,11003959531895227035_usize];
    _10 = _2;
    _13 = _3;
    _3 = [3_usize,3986485773704429588_usize,7_usize];
    _10 = _2;
    _8 = !_14;
    _8 = _14;
    _7 = _3;
    RET = _5 as f32;
    _12 = _11;
    _13 = [1_usize,538643477902621994_usize,1_usize];
    match _5 {
    0 => bb1,
    1 => bb2,
    2 => bb3,
    3 => bb4,
    4 => bb5,
    5 => bb6,
    340282366920938463463374607431768210821 => bb8,
    _ => bb7
    }
    }
    bb1 = {
    Return()
    }
    bb2 = {
    Return()
    }
    bb3 = {
    Return()
    }
    bb4 = {
    Return()
    }
    bb5 = {
    Return()
    }
    bb6 = {
    Return()
    }
    bb7 = {
    Return()
    }
    bb8 = {
    _3 = [1143639327642390812_usize,6177393345636679152_usize,2_usize];
    _12 = [31_i8,44_i8,123_i8,116_i8];
    RET = 52_i8 as f32;
    _14 = !_8;
    _18 = [7795957252562007755_usize,5791147338097626188_usize,2_usize];
    _13 = _3;
    _8 = _14;
    _17 = !121_u8;
    _9 = [73_i8,(-116_i8),15_i8,34_i8];
    _9 = [(-55_i8),117_i8,(-50_i8),58_i8];
    _11 = [(-72_i8),52_i8,46_i8,15_i8];
    _15 = 7_isize;
    _15 = (-85_isize) | 9223372036854775807_isize;
    RET = (-650733270_i32) as f32;
    match _5 {
    0 => bb1,
    1 => bb6,
    2 => bb7,
    3 => bb4,
    340282366920938463463374607431768210821 => bb9,
    _ => bb5
    }
    }
    bb9 = {
    _4 = [9277970004679596759_usize,4_usize,14234201351004776653_usize];
    _9 = _12;
    _18 = _3;
    _20 = _7;
    _14 = _8;
    _5 = (-7742_i16) & 13723_i16;
    _14 = _8;
    _1 = [(-60_i8),(-73_i8),127_i8,89_i8];
    _12 = [(-91_i8),(-77_i8),(-38_i8),32_i8];
    RET = 2315481579_u32 as f32;
    Goto(bb10)
    }
    bb10 = {
    _9 = _1;
    RET = 220213028125126887913623954672933645548_u128 as f32;
    _1 = _12;
    Call(RET = fn16(_20, _15, _15, _4), ReturnTo(bb11), UnwindUnreachable())
    }
    bb11 = {
    _20 = _7;
    _2 = _10;
    _6 = [4_usize,4_usize,4_usize];
    _6 = [2_usize,10405874208326831883_usize,3709587369289691120_usize];
    _4 = [3978096643404849404_usize,6_usize,6_usize];
    _19 = !4236976592_u32;
    Goto(bb12)
    }
    bb12 = {
    _4 = [4_usize,8338006156185184771_usize,12604563759315787916_usize];
    _20 = _4;
    _10 = core::ptr::addr_of!(RET);
    _13 = _7;
    (*_10) = _15 as f32;
    (*_10) = 0_usize as f32;
    RET = 109085210450829690_usize as f32;
    _11 = _1;
    _22 = 8695414982982610185_u64 as f64;
    _1 = [44_i8,(-75_i8),(-18_i8),(-71_i8)];
    _21 = [_5,_5,_5,_5,_5,_5];
    Goto(bb13)
    }
    bb13 = {
    RET = (-6393407041147736494_i64) as f32;
    _18 = [6279385497293046667_usize,10931779459137590827_usize,17120978480479104769_usize];
    _12 = _11;
    _17 = !8_u8;
    (*_10) = _19 as f32;
    Goto(bb14)
    }
    bb14 = {
    _10 = _2;
    _8 = _14 & _14;
    _2 = _10;
    _8 = !_14;
    _26.1 = -_5;
    _24 = core::ptr::addr_of_mut!(_21);
    _11 = [(-32_i8),(-62_i8),12_i8,46_i8];
    _6 = _7;
    _3 = _7;
    _26.1 = _5 + _5;
    _12 = _1;
    _7 = [14560569033915850362_usize,0_usize,0_usize];
    _22 = _19 as f64;
    _25 = [5446925045158115468_usize,9975043663136428380_usize,15209474993941194210_usize];
    RET = (-136826762612260373963047850128933251377_i128) as f32;
    _1 = [97_i8,(-11_i8),22_i8,(-77_i8)];
    _4 = _13;
    _26.1 = 108_i8 as i16;
    Goto(bb15)
    }
    bb15 = {
    Call(_30 = dump_var(15_usize, 1_usize, Move(_1), 19_usize, Move(_19), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    Call(_30 = dump_var(15_usize, 14_usize, Move(_14), 17_usize, Move(_17), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_30 = dump_var(15_usize, 15_usize, Move(_15), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [usize; 3], mut _2: isize, mut _3: isize, mut _4: [usize; 3]) -> f32 {
    mir! {
    type RET = f32;
    let _5: [u32; 6];
    let _6: *mut u64;
    let _7: i64;
    let _8: f64;
    let _9: [i8; 4];
    let _10: i32;
    let _11: f32;
    let _12: i128;
    let _13: i64;
    let _14: Adt52;
    let _15: bool;
    let _16: usize;
    let _17: f64;
    let _18: bool;
    let _19: usize;
    let _20: isize;
    let _21: isize;
    let _22: *mut u64;
    let _23: *mut u64;
    let _24: Adt53;
    let _25: &'static u16;
    let _26: u16;
    let _27: Adt49;
    let _28: isize;
    let _29: (f32, i64, (f64, i16, &'static u16), f64);
    let _30: f32;
    let _31: isize;
    let _32: [i16; 6];
    let _33: [i32; 2];
    let _34: char;
    let _35: char;
    let _36: i128;
    let _37: Adt55;
    let _38: char;
    let _39: [i32; 2];
    let _40: *mut [i16; 6];
    let _41: bool;
    let _42: Adt61;
    let _43: u32;
    let _44: f64;
    let _45: ();
    let _46: ();
    {
    _5 = [1715051855_u32,4248757379_u32,2105131343_u32,2661153917_u32,2891185224_u32,3715244400_u32];
    _4 = _1;
    _5 = [390335128_u32,3009453619_u32,50710877_u32,3349474894_u32,86363953_u32,4178372921_u32];
    RET = (-9_i8) as f32;
    RET = 27064114280369593977027836015653712096_u128 as f32;
    _3 = _2 & _2;
    _3 = _2;
    _1 = [7_usize,9531830074099486089_usize,6_usize];
    _1 = [3_usize,6_usize,5825000533951084393_usize];
    _2 = -_3;
    _1 = [17515802040372247461_usize,5_usize,6_usize];
    _5 = [1147782803_u32,2217039506_u32,1938663624_u32,1821825402_u32,2753929306_u32,1562288490_u32];
    _5 = [1683803440_u32,3018476555_u32,3605593257_u32,3749482568_u32,2845104135_u32,1509282309_u32];
    Call(_1 = fn17(_5, _5, _4), ReturnTo(bb1), UnwindUnreachable())
    }
    bb1 = {
    _4 = [6830782020935072131_usize,4_usize,12175001904461048122_usize];
    _5 = [122315383_u32,711171210_u32,598941616_u32,1798847056_u32,715709680_u32,180309803_u32];
    _3 = _2 << _2;
    _7 = (-5453468151527195870_i64) << _3;
    _5 = [2433683468_u32,2963176101_u32,79350387_u32,3434864984_u32,1913842419_u32,3210108396_u32];
    Call(_5 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
    }
    bb2 = {
    RET = (-848815547_i32) as f32;
    _8 = 7_usize as f64;
    Goto(bb3)
    }
    bb3 = {
    _7 = !(-4713673023633688558_i64);
    _7 = !7289559249622000133_i64;
    _4 = [4_usize,1_usize,1_usize];
    _8 = 5_usize as f64;
    _7 = 4621264221848367315_i64;
    _1 = _4;
    _2 = -_3;
    RET = _7 as f32;
    _2 = 150227494810392166816856926281614643385_u128 as isize;
    RET = 9525663837168261203_usize as f32;
    _9 = [(-94_i8),(-98_i8),(-41_i8),88_i8];
    _3 = _2 << _7;
    RET = 14888534217300310551_u64 as f32;
    _2 = _3;
    _2 = -_3;
    _10 = (-1483159506_i32) << _7;
    _7 = (-9026561499484925921_i64);
    _5 = [3469754560_u32,460435365_u32,2750540844_u32,1650789735_u32,891842679_u32,46121883_u32];
    Goto(bb4)
    }
    bb4 = {
    _8 = 9694236185005791740_usize as f64;
    RET = 78_i8 as f32;
    Goto(bb5)
    }
    bb5 = {
    _5 = [1978034771_u32,2028665837_u32,2334220239_u32,1114903292_u32,3357111613_u32,147887798_u32];
    _7 = 121_u8 as i64;
    _5 = [152657598_u32,568538337_u32,1486413318_u32,1246237854_u32,3152445962_u32,3022623644_u32];
    RET = (-112_i8) as f32;
    _12 = !85684925902328470183775154509142848063_i128;
    _4 = _1;
    _10 = -(-205825443_i32);
    _2 = 2188121841_u32 as isize;
    _5 = [1736016887_u32,1620946208_u32,613079167_u32,3851807601_u32,3671892656_u32,744826963_u32];
    _8 = 17009441417159826334_usize as f64;
    _1 = _4;
    _9 = [89_i8,118_i8,(-52_i8),(-18_i8)];
    _4 = [13250639747474881422_usize,17797603188784044615_usize,3951795756978422529_usize];
    _3 = -_2;
    _10 = (-9970874_i32) << _12;
    _4 = [0_usize,17702768220766405643_usize,5688248809629279022_usize];
    RET = _8 as f32;
    _10 = !915381691_i32;
    _5 = [3991715916_u32,3916529724_u32,1273160690_u32,2158527645_u32,4244386267_u32,819199892_u32];
    _4 = [6644320489306674999_usize,2_usize,2208751918023272313_usize];
    _7 = !8047994671795069365_i64;
    _13 = !_7;
    _7 = 5_usize as i64;
    _9 = [(-54_i8),(-22_i8),111_i8,(-72_i8)];
    _5 = [3397541887_u32,3468864051_u32,2397254122_u32,3838849010_u32,3828462582_u32,3138122438_u32];
    _12 = !(-94742629423750005155563390698639758527_i128);
    Goto(bb6)
    }
    bb6 = {
    _11 = -RET;
    _4 = [5_usize,3_usize,5_usize];
    _7 = !_13;
    _10 = -233992716_i32;
    _11 = 51_i8 as f32;
    _7 = 5_u8 as i64;
    _5 = [2643167202_u32,2853259567_u32,3558511223_u32,2935759426_u32,2327136310_u32,26319516_u32];
    _10 = 1994169112_i32;
    _16 = 1_usize + 5_usize;
    _7 = _13;
    _2 = 14733395808824073351_u64 as isize;
    RET = _11;
    _5 = [4066914248_u32,3587568032_u32,750244606_u32,3577592317_u32,34656469_u32,1499087907_u32];
    _20 = 5125_i16 as isize;
    _9 = [98_i8,86_i8,(-39_i8),(-82_i8)];
    _7 = 25995_u16 as i64;
    _1 = [_16,_16,_16];
    _17 = -_8;
    match _10 {
    1994169112 => bb7,
    _ => bb4
    }
    }
    bb7 = {
    _10 = 967317558_i32;
    _1 = [_16,_16,_16];
    _15 = !false;
    _20 = !_3;
    _7 = _13;
    _21 = _2 >> _3;
    Goto(bb8)
    }
    bb8 = {
    _19 = _16;
    _9 = [84_i8,66_i8,(-110_i8),(-66_i8)];
    _11 = RET;
    _3 = _21;
    _3 = -_21;
    _15 = !false;
    _11 = RET;
    _9 = [(-99_i8),(-118_i8),(-43_i8),104_i8];
    _11 = -RET;
    _18 = _15;
    _19 = _16;
    _26 = 5092412630552602964_u64 as u16;
    _1 = [_16,_16,_16];
    RET = -_11;
    _1 = [_19,_16,_16];
    _1 = _4;
    RET = _11 + _11;
    _20 = 315163899807358406951102221488499839810_u128 as isize;
    _25 = &_26;
    Call(_20 = core::intrinsics::bswap(_21), ReturnTo(bb9), UnwindUnreachable())
    }
    bb9 = {
    _13 = !_7;
    _1 = [_19,_19,_19];
    _20 = _10 as isize;
    _29.2.1 = '\u{995b7}' as i16;
    _29.0 = RET;
    _28 = _20 << (*_25);
    _29.2.0 = -_17;
    _16 = _19;
    _4 = [_16,_16,_19];
    _7 = _13;
    _1 = [_19,_16,_19];
    _7 = _13;
    _4 = _1;
    _16 = !_19;
    _11 = (*_25) as f32;
    _17 = (-37_i8) as f64;
    _17 = _29.2.0 + _8;
    _26 = 39174_u16 + 14831_u16;
    Goto(bb10)
    }
    bb10 = {
    _15 = _20 == _21;
    _32 = [_29.2.1,_29.2.1,_29.2.1,_29.2.1,_29.2.1,_29.2.1];
    _9 = [(-18_i8),(-19_i8),(-119_i8),(-41_i8)];
    RET = _11;
    _17 = _21 as f64;
    _28 = !_3;
    _16 = !_19;
    _17 = _28 as f64;
    _1 = [_16,_16,_16];
    _19 = _16 >> _28;
    _29.3 = -_17;
    _7 = _18 as i64;
    _11 = _29.0 - _29.0;
    _1 = [_16,_19,_19];
    _33 = [_10,_10];
    _36 = _12 + _12;
    _29.1 = _26 as i64;
    Goto(bb11)
    }
    bb11 = {
    _21 = -_3;
    _30 = _11;
    _30 = -_11;
    _10 = 2094677657_i32 | (-445164874_i32);
    _10 = 368662328_i32 | (-919755325_i32);
    RET = 121_i8 as f32;
    _26 = 11187_u16 << _28;
    _4 = [_19,_19,_19];
    RET = _30;
    _39 = _33;
    _38 = '\u{e6baa}';
    _31 = _26 as isize;
    _34 = _38;
    _39 = [_10,_10];
    Call(_29.2.0 = core::intrinsics::transmute(_19), ReturnTo(bb12), UnwindUnreachable())
    }
    bb12 = {
    _25 = &_26;
    RET = _11 - _30;
    _33 = _39;
    _17 = _29.2.0;
    _29.0 = _11 * RET;
    _32 = [_29.2.1,_29.2.1,_29.2.1,_29.2.1,_29.2.1,_29.2.1];
    _16 = _19 >> _31;
    _41 = _18;
    Goto(bb13)
    }
    bb13 = {
    _10 = 811496782_i32 + 59845766_i32;
    _29.2.0 = _8 - _8;
    _32 = [_29.2.1,_29.2.1,_29.2.1,_29.2.1,_29.2.1,_29.2.1];
    _39 = [_10,_10];
    _9 = [(-3_i8),(-32_i8),4_i8,(-23_i8)];
    _29.1 = _7;
    Goto(bb14)
    }
    bb14 = {
    _38 = _34;
    _2 = 160_u8 as isize;
    _33 = [_10,_10];
    _29.2.0 = _29.3;
    _41 = _15;
    _5 = [2131541829_u32,994696465_u32,3637964520_u32,4197735929_u32,3014802028_u32,3372108508_u32];
    _29.2.2 = &_26;
    _21 = !_28;
    _25 = &_26;
    _39 = [_10,_10];
    _29.2.0 = -_8;
    _9 = [103_i8,(-112_i8),101_i8,94_i8];
    Goto(bb15)
    }
    bb15 = {
    Call(_45 = dump_var(16_usize, 5_usize, Move(_5), 10_usize, Move(_10), 7_usize, Move(_7), 39_usize, Move(_39)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    Call(_45 = dump_var(16_usize, 28_usize, Move(_28), 20_usize, Move(_20), 18_usize, Move(_18), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_45 = dump_var(16_usize, 2_usize, Move(_2), 4_usize, Move(_4), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Call(_45 = dump_var(16_usize, 38_usize, Move(_38), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [u32; 6], mut _2: [u32; 6], mut _3: [usize; 3]) -> [usize; 3] {
    mir! {
    type RET = [usize; 3];
    let _4: (i8, isize, [i32; 2]);
    let _5: char;
    let _6: (u64,);
    let _7: char;
    let _8: i64;
    let _9: [i32; 2];
    let _10: i32;
    let _11: [u16; 1];
    let _12: char;
    let _13: u8;
    let _14: Adt47;
    let _15: i8;
    let _16: usize;
    let _17: [char; 7];
    let _18: Adt47;
    let _19: [u32; 6];
    let _20: (u64,);
    let _21: [u64; 6];
    let _22: Adt54;
    let _23: ();
    let _24: ();
    {
    RET = _3;
    _3 = RET;
    _1 = [987325378_u32,3225336032_u32,3971052197_u32,2642003686_u32,2491320388_u32,2184917989_u32];
    RET = [5_usize,4_usize,3095618285522037787_usize];
    RET = [7780840011766870407_usize,4956326611253302801_usize,0_usize];
    _2 = [4222726483_u32,4070907312_u32,2014043756_u32,974631139_u32,4068049743_u32,2706136031_u32];
    RET = _3;
    _2 = [354699615_u32,1082823913_u32,3399305038_u32,2313174801_u32,4245057248_u32,1835109551_u32];
    RET = _3;
    _4.0 = 66_isize as i8;
    _4.1 = 102_isize;
    RET = [3_usize,3_usize,5406641355795498929_usize];
    RET = [16149004497601867560_usize,0_usize,6_usize];
    _1 = [2427690949_u32,715837071_u32,2180884886_u32,2225829245_u32,3752402963_u32,3139441606_u32];
    _3 = [2_usize,10704762881235155751_usize,7_usize];
    RET = _3;
    match _4.1 {
    102 => bb2,
    _ => bb1
    }
    }
    bb1 = {
    Return()
    }
    bb2 = {
    _1 = _2;
    _4.1 = 17187751958326637382_usize as isize;
    _4.1 = 8_u8 as isize;
    _6 = (15927742390943874104_u64,);
    _5 = '\u{1027b1}';
    RET = [6_usize,14884277905441404505_usize,2800071840782683925_usize];
    _6.0 = 3089299978958268297_u64;
    RET = [14825564384643836754_usize,9898264360687990106_usize,869885098444087215_usize];
    _1 = [1244113445_u32,1479202606_u32,3229526734_u32,1906008232_u32,967488669_u32,594436693_u32];
    RET = _3;
    _5 = '\u{aa4d4}';
    RET = [10339670819974352192_usize,7_usize,8553820838735270514_usize];
    _6 = (5688329901552132269_u64,);
    _6.0 = !6857857808536567268_u64;
    _2 = [3973743334_u32,3307631336_u32,2714181945_u32,125954189_u32,4268328640_u32,1261998450_u32];
    _7 = _5;
    _4.2 = [(-504505007_i32),(-204309662_i32)];
    _5 = _7;
    RET = [6_usize,10693094887953238632_usize,10313276247357214576_usize];
    _5 = _7;
    _5 = _7;
    _1 = [633018262_u32,1182991328_u32,879217647_u32,77660581_u32,2868614399_u32,2979506232_u32];
    _3 = [13883141607580766474_usize,2370768859076646548_usize,8400655897549243679_usize];
    Goto(bb3)
    }
    bb3 = {
    _6.0 = !2592944202214363349_u64;
    _6.0 = 11318913286514061915_u64;
    RET = [4608228942194438868_usize,8144523251064977534_usize,6_usize];
    _3 = RET;
    _8 = 7460489957423857966_i64 - (-6145543877343047704_i64);
    _6 = (9617630427270737992_u64,);
    _6.0 = 12397004196230776802_u64 ^ 9779238319007123932_u64;
    _4.2 = [(-2139538038_i32),(-1481208115_i32)];
    Call(_4.0 = fn18(_2, _1, _2, _2, _7, _6, _1, RET, _6, _4.2), ReturnTo(bb4), UnwindUnreachable())
    }
    bb4 = {
    _5 = _7;
    _8 = 8177295457092904620_i64;
    _10 = 2157190832784714701_usize as i32;
    _2 = [1041475924_u32,1694585743_u32,3515545803_u32,919555795_u32,3761483626_u32,3064038874_u32];
    RET = [15165337418700366886_usize,1544101381829410053_usize,14053781133614056611_usize];
    _3 = [5_usize,3544462425244515310_usize,4_usize];
    RET = [574540052326207574_usize,14478253490310460141_usize,7_usize];
    _10 = 1443_u16 as i32;
    _8 = -3200859464860142917_i64;
    _6 = (1309322670003044177_u64,);
    _4.0 = !107_i8;
    _4.0 = -66_i8;
    _8 = (-6795331739824572554_i64);
    _11 = [38724_u16];
    _8 = 151_u8 as i64;
    RET = [1_usize,12917987856937238002_usize,3_usize];
    _4.2 = [_10,_10];
    RET = [10177478466798493390_usize,3_usize,6798008094783298722_usize];
    _9 = [_10,_10];
    match _6.0 {
    0 => bb1,
    1 => bb2,
    1309322670003044177 => bb6,
    _ => bb5
    }
    }
    bb5 = {
    _1 = _2;
    _4.1 = 17187751958326637382_usize as isize;
    _4.1 = 8_u8 as isize;
    _6 = (15927742390943874104_u64,);
    _5 = '\u{1027b1}';
    RET = [6_usize,14884277905441404505_usize,2800071840782683925_usize];
    _6.0 = 3089299978958268297_u64;
    RET = [14825564384643836754_usize,9898264360687990106_usize,869885098444087215_usize];
    _1 = [1244113445_u32,1479202606_u32,3229526734_u32,1906008232_u32,967488669_u32,594436693_u32];
    RET = _3;
    _5 = '\u{aa4d4}';
    RET = [10339670819974352192_usize,7_usize,8553820838735270514_usize];
    _6 = (5688329901552132269_u64,);
    _6.0 = !6857857808536567268_u64;
    _2 = [3973743334_u32,3307631336_u32,2714181945_u32,125954189_u32,4268328640_u32,1261998450_u32];
    _7 = _5;
    _4.2 = [(-504505007_i32),(-204309662_i32)];
    _5 = _7;
    RET = [6_usize,10693094887953238632_usize,10313276247357214576_usize];
    _5 = _7;
    _5 = _7;
    _1 = [633018262_u32,1182991328_u32,879217647_u32,77660581_u32,2868614399_u32,2979506232_u32];
    _3 = [13883141607580766474_usize,2370768859076646548_usize,8400655897549243679_usize];
    Goto(bb3)
    }
    bb6 = {
    _10 = (-532932605_i32) >> _8;
    _4 = ((-125_i8), 9223372036854775807_isize, _9);
    _5 = _7;
    _11 = [36794_u16];
    _11 = [50780_u16];
    _5 = _7;
    _5 = _7;
    _12 = _5;
    _4 = ((-60_i8), (-81_isize), _9);
    RET = _3;
    _3 = RET;
    _4 = ((-23_i8), 9223372036854775807_isize, _9);
    _16 = 3_usize * 11762347959841175020_usize;
    _16 = 1824619836_u32 as usize;
    _4.0 = (-106_i8) ^ 48_i8;
    _13 = 130_u8;
    match _4.1 {
    0 => bb1,
    1 => bb2,
    2 => bb3,
    3 => bb4,
    4 => bb5,
    5 => bb7,
    6 => bb8,
    9223372036854775807 => bb10,
    _ => bb9
    }
    }
    bb7 = {
    _1 = _2;
    _4.1 = 17187751958326637382_usize as isize;
    _4.1 = 8_u8 as isize;
    _6 = (15927742390943874104_u64,);
    _5 = '\u{1027b1}';
    RET = [6_usize,14884277905441404505_usize,2800071840782683925_usize];
    _6.0 = 3089299978958268297_u64;
    RET = [14825564384643836754_usize,9898264360687990106_usize,869885098444087215_usize];
    _1 = [1244113445_u32,1479202606_u32,3229526734_u32,1906008232_u32,967488669_u32,594436693_u32];
    RET = _3;
    _5 = '\u{aa4d4}';
    RET = [10339670819974352192_usize,7_usize,8553820838735270514_usize];
    _6 = (5688329901552132269_u64,);
    _6.0 = !6857857808536567268_u64;
    _2 = [3973743334_u32,3307631336_u32,2714181945_u32,125954189_u32,4268328640_u32,1261998450_u32];
    _7 = _5;
    _4.2 = [(-504505007_i32),(-204309662_i32)];
    _5 = _7;
    RET = [6_usize,10693094887953238632_usize,10313276247357214576_usize];
    _5 = _7;
    _5 = _7;
    _1 = [633018262_u32,1182991328_u32,879217647_u32,77660581_u32,2868614399_u32,2979506232_u32];
    _3 = [13883141607580766474_usize,2370768859076646548_usize,8400655897549243679_usize];
    Goto(bb3)
    }
    bb8 = {
    Return()
    }
    bb9 = {
    _6.0 = !2592944202214363349_u64;
    _6.0 = 11318913286514061915_u64;
    RET = [4608228942194438868_usize,8144523251064977534_usize,6_usize];
    _3 = RET;
    _8 = 7460489957423857966_i64 - (-6145543877343047704_i64);
    _6 = (9617630427270737992_u64,);
    _6.0 = 12397004196230776802_u64 ^ 9779238319007123932_u64;
    _4.2 = [(-2139538038_i32),(-1481208115_i32)];
    Call(_4.0 = fn18(_2, _1, _2, _2, _7, _6, _1, RET, _6, _4.2), ReturnTo(bb4), UnwindUnreachable())
    }
    bb10 = {
    _4.2 = _9;
    _6 = (15980222912742407981_u64,);
    _11 = [16156_u16];
    Goto(bb11)
    }
    bb11 = {
    _6 = (10594950559171190002_u64,);
    _4.1 = (-9223372036854775808_isize) >> _8;
    _7 = _12;
    _6.0 = _8 as u64;
    _4.2 = [_10,_10];
    _16 = !2_usize;
    _4.1 = 9223372036854775807_isize;
    _5 = _12;
    _2 = [1622299261_u32,3232630253_u32,4139692090_u32,3857064987_u32,1694237280_u32,2452528184_u32];
    _5 = _12;
    _2 = _1;
    _12 = _5;
    _4 = ((-39_i8), (-9223372036854775808_isize), _9);
    _6.0 = 11438813772005974379_u64 >> _16;
    _15 = _4.0;
    RET = [_16,_16,_16];
    _4.1 = (-111_isize) >> _16;
    _19 = [1027592063_u32,2556276640_u32,2906301854_u32,296217480_u32,2096154989_u32,3512930906_u32];
    _11 = [12861_u16];
    match _4.0 {
    0 => bb1,
    1 => bb2,
    2 => bb9,
    3 => bb7,
    340282366920938463463374607431768211417 => bb13,
    _ => bb12
    }
    }
    bb12 = {
    _4.2 = _9;
    _6 = (15980222912742407981_u64,);
    _11 = [16156_u16];
    Goto(bb11)
    }
    bb13 = {
    _20 = _6;
    _17 = [_7,_5,_5,_5,_7,_12,_12];
    _9 = [_10,_10];
    _4.2 = [_10,_10];
    Goto(bb14)
    }
    bb14 = {
    _4 = (_15, 85_isize, _9);
    RET = _3;
    RET = [_16,_16,_16];
    _10 = -409521780_i32;
    _16 = !2_usize;
    RET = [_16,_16,_16];
    _12 = _5;
    _5 = _12;
    _19 = [428001426_u32,4203828201_u32,1983964226_u32,2299225587_u32,591182643_u32,1248887880_u32];
    _10 = false as i32;
    _21 = [_20.0,_6.0,_6.0,_6.0,_20.0,_6.0];
    _20 = (_6.0,);
    _6 = _20;
    _15 = -_4.0;
    _5 = _7;
    _11 = [11461_u16];
    _17 = [_5,_12,_12,_7,_7,_12,_5];
    Goto(bb15)
    }
    bb15 = {
    Call(_23 = dump_var(17_usize, 4_usize, Move(_4), 9_usize, Move(_9), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    Call(_23 = dump_var(17_usize, 7_usize, Move(_7), 12_usize, Move(_12), 21_usize, Move(_21), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_23 = dump_var(17_usize, 15_usize, Move(_15), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(
    mut _1: [u32; 6],
    mut _2: [u32; 6],
    mut _3: [u32; 6],
    mut _4: [u32; 6],
    mut _5: char,
    mut _6: (u64,),
    mut _7: [u32; 6],
    mut _8: [usize; 3],
    mut _9: (u64,),
    mut _10: [i32; 2],
) -> i8 {
    mir! {
    type RET = i8;
    let _11: f32;
    let _12: u128;
    let _13: [u32; 6];
    let _14: ([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2]);
    let _15: i8;
    let _16: f32;
    let _17: [u64; 6];
    let _18: Adt58;
    let _19: *mut u64;
    let _20: char;
    let _21: Adt51;
    let _22: i32;
    let _23: [usize; 3];
    let _24: [u64; 6];
    let _25: i128;
    let _26: f32;
    let _27: [char; 7];
    let _28: Adt58;
    let _29: bool;
    let _30: Adt59;
    let _31: u8;
    let _32: [i32; 2];
    let _33: f64;
    let _34: Adt53;
    let _35: isize;
    let _36: char;
    let _37: i64;
    let _38: usize;
    let _39: (usize, i16);
    let _40: f32;
    let _41: &'static u16;
    let _42: [i32; 2];
    let _43: *const f32;
    let _44: [usize; 3];
    let _45: char;
    let _46: isize;
    let _47: bool;
    let _48: i16;
    let _49: f32;
    let _50: f64;
    let _51: (f64, i16, &'static u16);
    let _52: bool;
    let _53: u128;
    let _54: (i8, isize, [i32; 2]);
    let _55: isize;
    let _56: ();
    let _57: ();
    {
    _9.0 = !_6.0;
    _2 = _4;
    _5 = '\u{ffe66}';
    _6.0 = 100789011544866159577839844151667791037_u128 as u64;
    _11 = (-298433768_i32) as f32;
    _5 = '\u{81d50}';
    _1 = [3602760781_u32,3691091126_u32,3891646742_u32,4270983787_u32,2076400052_u32,602200415_u32];
    _8 = [7_usize,7_usize,15892387097016747793_usize];
    _1 = [4022733140_u32,5358023_u32,3169020449_u32,11188251_u32,3153615807_u32,26989305_u32];
    _12 = _9.0 as u128;
    RET = (-7342074923759464092_i64) as i8;
    _11 = RET as f32;
    _4 = [4244342139_u32,1404523445_u32,3891972029_u32,132607316_u32,2164755447_u32,828224719_u32];
    _6.0 = _9.0 >> _12;
    _6 = (_9.0,);
    _3 = [1485012075_u32,4148012016_u32,3414715287_u32,1410356644_u32,1579837145_u32,32024722_u32];
    _3 = [237848575_u32,1337374902_u32,317229667_u32,2754835944_u32,1938915983_u32,1959127378_u32];
    _13 = [2469109553_u32,4118296036_u32,2863791603_u32,379492031_u32,208152633_u32,1060473001_u32];
    RET = 17_i8 << _9.0;
    RET = 76_i8;
    _5 = '\u{40384}';
    Goto(bb1)
    }
    bb1 = {
    _14.1.2 = _10;
    _2 = [65711320_u32,1887252061_u32,3489849271_u32,1154819256_u32,2364533414_u32,807303951_u32];
    _1 = [1261906773_u32,1165295373_u32,3829475637_u32,1629562143_u32,262113298_u32,562537524_u32];
    _11 = 32324_i16 as f32;
    _14.2 = core::ptr::addr_of!(_11);
    _14.0 = _1;
    _16 = _11;
    RET = 50_i8;
    _14.4 = [_9.0,_9.0];
    _14.1 = (RET, (-9223372036854775808_isize), _10);
    RET = _14.1.0;
    _8 = [18286955035878474063_usize,17214626571857575198_usize,4_usize];
    _14.1.2 = [343527530_i32,2023387697_i32];
    _14.3 = [19400_i16,(-30861_i16),(-32065_i16),628_i16,10619_i16,(-20695_i16)];
    _14.1 = (RET, 118_isize, _10);
    _14.1.1 = _5 as isize;
    Goto(bb2)
    }
    bb2 = {
    _17 = [_6.0,_6.0,_9.0,_9.0,_6.0,_6.0];
    _14.1.2 = _10;
    _1 = [2301379442_u32,1497765529_u32,959893264_u32,20531692_u32,3002517960_u32,1441698003_u32];
    _6 = (_9.0,);
    _2 = [952012783_u32,3171576481_u32,3958253147_u32,2630192235_u32,2370516953_u32,4210601712_u32];
    _14.3 = [(-13758_i16),(-16376_i16),32568_i16,3948_i16,15698_i16,23524_i16];
    _2 = [880794904_u32,569606626_u32,2999436837_u32,1023124686_u32,2813245629_u32,877095713_u32];
    _3 = _7;
    _14.0 = _1;
    RET = _14.1.0 << _12;
    _15 = RET;
    _6.0 = _9.0;
    _11 = RET as f32;
    _11 = _16;
    _14.1.1 = 37_isize;
    _10 = [(-1189664983_i32),(-1843283046_i32)];
    _4 = [141152315_u32,923508307_u32,120272167_u32,1969181250_u32,995427701_u32,2851504134_u32];
    _3 = [211482038_u32,163409766_u32,279988996_u32,1454374220_u32,4150984696_u32,1932031123_u32];
    RET = _9.0 as i8;
    _8 = [14390592704379642575_usize,4_usize,3_usize];
    _15 = -RET;
    _19 = core::ptr::addr_of_mut!(_6.0);
    Goto(bb3)
    }
    bb3 = {
    _9.0 = _12 as u64;
    _14.1.0 = _15;
    _14.1 = (_15, 11_isize, _10);
    _14.1.0 = !_15;
    _11 = 3575816120343523077_usize as f32;
    _12 = 17838865695349035861415612676655428851_u128;
    _15 = _14.1.0;
    _14.4 = [_6.0,(*_19)];
    _14.1 = (RET, 77_isize, _10);
    _14.3 = [(-5004_i16),12421_i16,22482_i16,12569_i16,(-10242_i16),(-25974_i16)];
    _20 = _5;
    _14.1.2 = _10;
    _23 = [6_usize,3_usize,395896769273730665_usize];
    _14.1.2 = [(-2037438097_i32),129464118_i32];
    _24 = [_6.0,(*_19),(*_19),_6.0,(*_19),(*_19)];
    _24 = _17;
    _14.2 = core::ptr::addr_of!(_11);
    _6 = _9;
    _14.2 = core::ptr::addr_of!(_11);
    _14.1.0 = RET << _14.1.1;
    _11 = _16;
    _25 = 2634808092_u32 as i128;
    RET = 14592_u16 as i8;
    _14.1.2 = [1314227839_i32,(-1230256135_i32)];
    _24 = [_6.0,(*_19),(*_19),(*_19),_9.0,(*_19)];
    _17 = [_9.0,_9.0,(*_19),(*_19),_6.0,_9.0];
    match _14.1.1 {
    77 => bb4,
    _ => bb1
    }
    }
    bb4 = {
    _14.1 = (_15, (-9223372036854775808_isize), _10);
    _7 = [817960025_u32,689343615_u32,3154906647_u32,4151610072_u32,2318151614_u32,3392866160_u32];
    _20 = _5;
    _22 = (-1972617172_i32);
    _6 = (_9.0,);
    RET = (-8113742857339171785_i64) as i8;
    _12 = 90868112699439434745300678521369580925_u128 + 164911135118120485340313874964970305649_u128;
    _6 = _9;
    _24 = _17;
    _14.2 = core::ptr::addr_of!(_26);
    _17 = [_9.0,_9.0,(*_19),(*_19),_6.0,_9.0];
    _7 = _13;
    _25 = !74294347985186634710872439362888971158_i128;
    _9 = ((*_19),);
    _29 = !true;
    _12 = 294007189894582756907151907208305363129_u128 + 246290092631499944622326591982133099065_u128;
    _9.0 = _20 as u64;
    _14.2 = core::ptr::addr_of!(_26);
    Goto(bb5)
    }
    bb5 = {
    _23 = [9962416739660636065_usize,374308292397877780_usize,14696349222271309011_usize];
    _13 = _14.0;
    _14.1 = (RET, 9223372036854775807_isize, _10);
    match _22 {
    0 => bb4,
    1 => bb6,
    2 => bb7,
    3 => bb8,
    340282366920938463463374607429795594284 => bb10,
    _ => bb9
    }
    }
    bb6 = {
    _14.1 = (_15, (-9223372036854775808_isize), _10);
    _7 = [817960025_u32,689343615_u32,3154906647_u32,4151610072_u32,2318151614_u32,3392866160_u32];
    _20 = _5;
    _22 = (-1972617172_i32);
    _6 = (_9.0,);
    RET = (-8113742857339171785_i64) as i8;
    _12 = 90868112699439434745300678521369580925_u128 + 164911135118120485340313874964970305649_u128;
    _6 = _9;
    _24 = _17;
    _14.2 = core::ptr::addr_of!(_26);
    _17 = [_9.0,_9.0,(*_19),(*_19),_6.0,_9.0];
    _7 = _13;
    _25 = !74294347985186634710872439362888971158_i128;
    _9 = ((*_19),);
    _29 = !true;
    _12 = 294007189894582756907151907208305363129_u128 + 246290092631499944622326591982133099065_u128;
    _9.0 = _20 as u64;
    _14.2 = core::ptr::addr_of!(_26);
    Goto(bb5)
    }
    bb7 = {
    _9.0 = _12 as u64;
    _14.1.0 = _15;
    _14.1 = (_15, 11_isize, _10);
    _14.1.0 = !_15;
    _11 = 3575816120343523077_usize as f32;
    _12 = 17838865695349035861415612676655428851_u128;
    _15 = _14.1.0;
    _14.4 = [_6.0,(*_19)];
    _14.1 = (RET, 77_isize, _10);
    _14.3 = [(-5004_i16),12421_i16,22482_i16,12569_i16,(-10242_i16),(-25974_i16)];
    _20 = _5;
    _14.1.2 = _10;
    _23 = [6_usize,3_usize,395896769273730665_usize];
    _14.1.2 = [(-2037438097_i32),129464118_i32];
    _24 = [_6.0,(*_19),(*_19),_6.0,(*_19),(*_19)];
    _24 = _17;
    _14.2 = core::ptr::addr_of!(_11);
    _6 = _9;
    _14.2 = core::ptr::addr_of!(_11);
    _14.1.0 = RET << _14.1.1;
    _11 = _16;
    _25 = 2634808092_u32 as i128;
    RET = 14592_u16 as i8;
    _14.1.2 = [1314227839_i32,(-1230256135_i32)];
    _24 = [_6.0,(*_19),(*_19),(*_19),_9.0,(*_19)];
    _17 = [_9.0,_9.0,(*_19),(*_19),_6.0,_9.0];
    match _14.1.1 {
    77 => bb4,
    _ => bb1
    }
    }
    bb8 = {
    _17 = [_6.0,_6.0,_9.0,_9.0,_6.0,_6.0];
    _14.1.2 = _10;
    _1 = [2301379442_u32,1497765529_u32,959893264_u32,20531692_u32,3002517960_u32,1441698003_u32];
    _6 = (_9.0,);
    _2 = [952012783_u32,3171576481_u32,3958253147_u32,2630192235_u32,2370516953_u32,4210601712_u32];
    _14.3 = [(-13758_i16),(-16376_i16),32568_i16,3948_i16,15698_i16,23524_i16];
    _2 = [880794904_u32,569606626_u32,2999436837_u32,1023124686_u32,2813245629_u32,877095713_u32];
    _3 = _7;
    _14.0 = _1;
    RET = _14.1.0 << _12;
    _15 = RET;
    _6.0 = _9.0;
    _11 = RET as f32;
    _11 = _16;
    _14.1.1 = 37_isize;
    _10 = [(-1189664983_i32),(-1843283046_i32)];
    _4 = [141152315_u32,923508307_u32,120272167_u32,1969181250_u32,995427701_u32,2851504134_u32];
    _3 = [211482038_u32,163409766_u32,279988996_u32,1454374220_u32,4150984696_u32,1932031123_u32];
    RET = _9.0 as i8;
    _8 = [14390592704379642575_usize,4_usize,3_usize];
    _15 = -RET;
    _19 = core::ptr::addr_of_mut!(_6.0);
    Goto(bb3)
    }
    bb9 = {
    _14.1.2 = _10;
    _2 = [65711320_u32,1887252061_u32,3489849271_u32,1154819256_u32,2364533414_u32,807303951_u32];
    _1 = [1261906773_u32,1165295373_u32,3829475637_u32,1629562143_u32,262113298_u32,562537524_u32];
    _11 = 32324_i16 as f32;
    _14.2 = core::ptr::addr_of!(_11);
    _14.0 = _1;
    _16 = _11;
    RET = 50_i8;
    _14.4 = [_9.0,_9.0];
    _14.1 = (RET, (-9223372036854775808_isize), _10);
    RET = _14.1.0;
    _8 = [18286955035878474063_usize,17214626571857575198_usize,4_usize];
    _14.1.2 = [343527530_i32,2023387697_i32];
    _14.3 = [19400_i16,(-30861_i16),(-32065_i16),628_i16,10619_i16,(-20695_i16)];
    _14.1 = (RET, 118_isize, _10);
    _14.1.1 = _5 as isize;
    Goto(bb2)
    }
    bb10 = {
    _6 = _9;
    _25 = 32131358511702517867428215298098512187_i128;
    _14.4 = [_9.0,(*_19)];
    _26 = _11 + _16;
    _33 = _25 as f64;
    _16 = -_26;
    _12 = 16050_u16 as u128;
    _35 = _14.1.1;
    _20 = _5;
    _36 = _5;
    _6 = (_9.0,);
    _14.4 = [_6.0,(*_19)];
    _32 = [_22,_22];
    _14.3 = [20388_i16,28519_i16,19633_i16,31557_i16,(-3348_i16),23113_i16];
    _4 = [264914361_u32,1057583229_u32,2026704769_u32,880167660_u32,4136962923_u32,1240018983_u32];
    _2 = [2976582967_u32,693089375_u32,4038014550_u32,3707772433_u32,3494216250_u32,2083650835_u32];
    _22 = !(-605584472_i32);
    _16 = _33 as f32;
    _1 = _13;
    _14.4 = [(*_19),_9.0];
    Goto(bb11)
    }
    bb11 = {
    _13 = [896842869_u32,3781094938_u32,2206602732_u32,2428928413_u32,1617329204_u32,3227955226_u32];
    _27 = [_20,_20,_36,_5,_20,_36,_20];
    _1 = [3617598520_u32,2215000954_u32,232909221_u32,2344461220_u32,3902987011_u32,1929790674_u32];
    _9.0 = _25 as u64;
    _29 = true;
    _38 = 5_usize;
    _14.1 = (RET, _35, _32);
    match _14.3[_38] {
    0 => bb7,
    1 => bb10,
    2 => bb5,
    23113 => bb12,
    _ => bb6
    }
    }
    bb12 = {
    _14.1.2 = [_22,_22];
    _27[_38] = _20;
    _39.1 = -_14.3[_38];
    _14.0[_38] = _2[_38];
    _10 = _14.1.2;
    _29 = true;
    (*_19) = _17[_38] * _17[_38];
    _14.1.1 = !_35;
    _24 = [(*_19),(*_19),(*_19),_6.0,(*_19),(*_19)];
    _39.0 = _38 & _38;
    _29 = _11 <= _26;
    _7[_38] = _13[_38] / _14.0[_38];
    _6 = _9;
    _24[_38] = !_6.0;
    _39 = (_38, _14.3[_38]);
    _37 = 99_u8 as i64;
    _12 = 4992328519258423053645852768063790490_u128;
    _4[_38] = _5 as u32;
    _20 = _5;
    Goto(bb13)
    }
    bb13 = {
    _13 = _14.0;
    _14.1.2 = [_22,_22];
    _37 = 2397479959317583303_i64;
    _37 = !(-3279092008031810162_i64);
    _47 = !_29;
    _36 = _27[_38];
    _14.1 = (_15, _35, _32);
    _14.1.0 = RET * _15;
    _2[_38] = _39.0 as u32;
    _9 = (_24[_38],);
    (*_19) = _35 as u64;
    _7[_38] = _3[_38];
    _9.0 = (*_19);
    _3[_38] = _1[_38];
    _46 = _35 | _35;
    _3 = [_7[_38],_1[_38],_13[_38],_14.0[_38],_7[_38],_7[_38]];
    _14.0[_38] = (*_19) as u32;
    _22 = -(-1651314419_i32);
    _45 = _27[_38];
    _44 = [_38,_38,_38];
    _2 = [_3[_38],_13[_38],_14.0[_38],_1[_38],_3[_38],_3[_38]];
    match _14.1.1 {
    9223372036854775807 => bb14,
    _ => bb5
    }
    }
    bb14 = {
    RET = _37 as i8;
    _44 = _23;
    _17 = [_9.0,(*_19),(*_19),_9.0,_9.0,_6.0];
    _51.0 = _46 as f64;
    _39.0 = _38;
    _14.0 = [_2[_38],_1[_38],_7[_38],_13[_38],_1[_38],_1[_38]];
    _53 = !_12;
    _14.1.0 = RET & _15;
    _14.0 = [_13[_38],_3[_38],_2[_38],_7[_38],_3[_38],_2[_38]];
    _51.1 = _22 as i16;
    _54.2 = [_22,_22];
    _10 = [_22,_22];
    _20 = _36;
    _38 = _39.0;
    _38 = _39.0;
    _40 = _11 * _16;
    _24[_38] = _53 as u64;
    _27 = [_36,_20,_36,_45,_45,_36,_5];
    _12 = !_53;
    _49 = _40 * _40;
    Goto(bb15)
    }
    bb15 = {
    Call(_56 = dump_var(18_usize, 27_usize, Move(_27), 47_usize, Move(_47), 15_usize, Move(_15), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
    }
    bb16 = {
    Call(_56 = dump_var(18_usize, 44_usize, Move(_44), 10_usize, Move(_10), 36_usize, Move(_36), 39_usize, Move(_39)), ReturnTo(bb17), UnwindUnreachable())
    }
    bb17 = {
    Call(_56 = dump_var(18_usize, 45_usize, Move(_45), 29_usize, Move(_29), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
    }
    bb18 = {
    Call(_56 = dump_var(18_usize, 20_usize, Move(_20), 22_usize, Move(_22), 3_usize, Move(_3), 53_usize, Move(_53)), ReturnTo(bb19), UnwindUnreachable())
    }
    bb19 = {
    Return()
    }

    }
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(
    mut _1: isize,
    mut _2: [u16; 6],
    mut _3: u32,
    mut _4: [u16; 6],
    mut _5: [u16; 6],
    mut _6: f32,
    mut _7: i32,
    mut _8: u64,
    mut _9: isize,
    mut _10: f32,
    mut _11: (u64,),
    mut _12: u32,
    mut _13: [usize; 3],
    mut _14: f32,
) -> Adt51 {
    mir! {
    type RET = Adt51;
    let _15: i128;
    let _16: isize;
    let _17: f32;
    let _18: Adt52;
    let _19: (u64,);
    let _20: i64;
    let _21: isize;
    let _22: [usize; 3];
    let _23: [u64; 6];
    let _24: f64;
    let _25: Adt48;
    let _26: char;
    let _27: i32;
    let _28: [i16; 6];
    let _29: i8;
    let _30: char;
    let _31: f32;
    let _32: [u32; 1];
    let _33: i64;
    let _34: bool;
    let _35: i128;
    let _36: f32;
    let _37: isize;
    let _38: [i16; 6];
    let _39: [u64; 6];
    let _40: char;
    let _41: Adt55;
    let _42: (i8, isize, [i32; 2]);
    let _43: [i8; 4];
    let _44: isize;
    let _45: [isize; 3];
    let _46: Adt58;
    let _47: f64;
    let _48: char;
    let _49: [u16; 6];
    let _50: Adt52;
    let _51: Adt54;
    let _52: [isize; 3];
    let _53: ([u32; 6], (i8, isize, [i32; 2]), *const f32, [i16; 6], [u64; 2]);
    let _54: [char; 7];
    let _55: f32;
    let _56: Adt48;
    let _57: [i16; 6];
    let _58: [u32; 1];
    let _59: Adt54;
    let _60: &'static u16;
    let _61: (i8, isize, [i32; 2]);
    let _62: Adt45;
    let _63: usize;
    let _64: [u32; 1];
    let _65: Adt49;
    let _66: u32;
    let _67: Adt57;
    let _68: i16;
    let _69: [u64; 6];
    let _70: i32;
    let _71: f32;
    let _72: [u64; 2];
    let _73: [u64; 6];
    let _74: i128;
    let _75: [u64; 2];
    let _76: ();
    let _77: ();
    {
    _12 = !_3;
    _7 = _6 as i32;
    _15 = -(-124688207174041064161793565599348108254_i128);
    _9 = _10 as isize;
    _14 = (-83_i8) as f32;
    _11 = (_8,);
    _11.0 = !_8;
    _9 = -_1;
    _3 = _12 + _12;
    _3 = _12;
    _9 = _1 | _1;
    _10 = -_6;
    _15 = -(-42977780532690247158260910688035795369_i128);
    _17 = 54_i8 as f32;
    _4 = [9842_u16,2960_u16,28763_u16,63074_u16,57642_u16,8408_u16];
    _6 = -_17;
    _9 = _7 as isize;
    _4 = _2;
    _4 = _2;
    _20 = (-1385815734513644346_i64);
    _19.0 = !_11.0;
    Goto(bb1)
    }
    bb1 = {
    _20 = (-28804_i16) as i64;
    _9 = -_1;
    _1 = _9;
    _3 = _12;
    _8 = _11.0 - _11.0;
    _10 = -_17;
    Goto(bb2)
    }
    bb2 = {
    _11.0 = _8;
    _21 = _9;
    _21 = _9 * _1;
    _16 = _21 << _7;
    _2 = [15798_u16,29827_u16,44207_u16,16385_u16,13317_u16,47192_u16];
    _16 = -_21;
    _6 = _10;
    _23 = [_19.0,_8,_19.0,_8,_8,_11.0];
    _19.0 = !_11.0;
    _15 = 148968271305454753907333711756179992153_i128 * 86558404896745946801237049389475553028_i128;
    _24 = _1 as f64;
    _25 = Adt48 { fld0: 154041900012660831496077323712384024112_u128 };
    _21 = _16 ^ _16;
    _21 = !_16;
    match _25.fld0 {
    0 => bb1,
    1 => bb3,
    2 => bb4,
    3 => bb5,
    4 => bb6,
    5 => bb7,
    6 => bb8,
    154041900012660831496077323712384024112 => bb10,
    _ => bb9
    }
    }
    bb3 = {
    _20 = (-28804_i16) as i64;
    _9 = -_1;
    _1 = _9;
    _3 = _12;
    _8 = _11.0 - _11.0;
    _10 = -_17;
    Goto(bb2)
    }
    bb4 = {
    Return()
    }
    bb5 = {
    Return()
    }
    bb6 = {
    Return()
    }
    bb7 = {
    Return()
    }
    bb8 = {
    Return()
    }
    bb9 = {
    Return()
    }
    bb10 = {
    _7 = _24 as i32;
    _20 = !(-1189022191762710648_i64);
    _2 = [34685_u16,932_u16,62953_u16,10933_u16,51963_u16,7523_u16];
    _10 = 55355_u16 as f32;
    _16 = _21;
    _23 = [_8,_11.0,_19.0,_8,_8,_19.0];
    _8 = _11.0 >> _7;
    _9 = _21;
    _25.fld0 = 195243742959630619227387868200886832370_u128;
    _23 = [_8,_8,_8,_8,_8,_8];
    _10 = _6 + _17;
    _15 = 47718898465055979598989819963014841569_i128;
    _11 = (_8,);
    _27 = !_7;
    _15 = 151258444403684597247619488016791590080_i128 + 98573120279591923023606860344394075136_i128;
    _1 = _16;
    _26 = '\u{3c3ca}';
    _3 = !_12;
    _13 = [0_usize,2_usize,14098013492852193400_usize];
    _27 = _7 - _7;
    _3 = _25.fld0 as u32;
    _20 = 188_u8 as i64;
    _5 = _2;
    _25.fld0 = 284528058439106054695208589554821455507_u128;
    _29 = 21_i8 >> _27;
    Goto(bb11)
    }
    bb11 = {
    _25.fld0 = !285967843788595781117409006853855364392_u128;
    _11.0 = _8;
    _3 = !_12;
    _24 = _29 as f64;
    _11 = _19;
    _31 = _14;
    _2 = _5;
    _11.0 = !_8;
    _7 = _27 | _27;
    _17 = _31;
    Goto(bb12)
    }
    bb12 = {
    _24 = _3 as f64;
    _11.0 = _8;
    _2 = _4;
    _20 = _26 as i64;
    _1 = _15 as isize;
    _30 = _26;
    _24 = _29 as f64;
    _26 = _30;
    _24 = _20 as f64;
    _24 = _29 as f64;
    _22 = _13;
    _25.fld0 = _29 as u128;
    _4 = _2;
    _26 = _30;
    _17 = _31 * _10;
    _17 = _14 - _14;
    _29 = 8_i8;
    _25 = Adt48 { fld0: 208353176932794128921214160930070785705_u128 };
    _22 = _13;
    _6 = -_10;
    _10 = _14 - _6;
    _36 = _17;
    match _29 {
    0 => bb7,
    1 => bb2,
    2 => bb3,
    3 => bb13,
    4 => bb14,
    8 => bb16,
    _ => bb15
    }
    }
    bb13 = {
    _25.fld0 = !285967843788595781117409006853855364392_u128;
    _11.0 = _8;
    _3 = !_12;
    _24 = _29 as f64;
    _11 = _19;
    _31 = _14;
    _2 = _5;
    _11.0 = !_8;
    _7 = _27 | _27;
    _17 = _31;
    Goto(bb12)
    }
    bb14 = {
    _7 = _24 as i32;
    _20 = !(-1189022191762710648_i64);
    _2 = [34685_u16,932_u16,62953_u16,10933_u16,51963_u16,7523_u16];
    _10 = 55355_u16 as f32;
    _16 = _21;
    _23 = [_8,_11.0,_19.0,_8,_8,_19.0];
    _8 = _11.0 >> _7;
    _9 = _21;
    _25.fld0 = 195243742959630619227387868200886832370_u128;
    _23 = [_8,_8,_8,_8,_8,_8];
    _10 = _6 + _17;
    _15 = 47718898465055979598989819963014841569_i128;
    _11 = (_8,);
    _27 = !_7;
    _15 = 151258444403684597247619488016791590080_i128 + 98573120279591923023606860344394075136_i128;
    _1 = _16;
    _26 = '\u{3c3ca}';
    _3 = !_12;
    _13 = [0_usize,2_usize,14098013492852193400_usize];
    _27 = _7 - _7;
    _3 = _25.fld0 as u32;
    _20 = 188_u8 as i64;
    _5 = _2;
    _25.fld0 = 284528058439106054695208589554821455507_u128;
    _29 = 21_i8 >> _27;
    Goto(bb11)
    }
    bb15 = {
    Return()
    }
    bb16 = {
    _12 = _3 + _3;
    _34 = _21 > _21;
    _35 = _15 - _15;
    _9 = _20 as isize;
    _15 = _35;
    _15 = -_35;
    _16 = _21 & _1;
    _32 = [_12];
    _22 = _13;
    _21 = _16;
    _7 = 197_u8 as i32;
    _20 = _34 as i64;
    Goto(bb17)
    }
    bb17 = {
    _16 = _21;
    _42.1 = _16 << _20;
    _25 = Adt48 { fld0: 107750656822822299150915417090009809213_u128 };
    _17 = _36 * _36;
    _5 = [34904_u16,35356_u16,3602_u16,9587_u16,39200_u16,61048_u16];
    _33 = (-26501_i16) as i64;
    _15 = _35;
    _22 = [1_usize,4388440195645029973_usize,8192915282512875126_usize];
    _25.fld0 = _20 as u128;
    _19.0 = _11.0 << _27;
    _7 = _27;
    _38 = [(-16038_i16),(-6912_i16),3142_i16,534_i16,(-10077_i16),(-12953_i16)];
    _1 = _21;
    _9 = _1 | _42.1;
    Goto(bb18)
    }
    bb18 = {
    _35 = _15;
    _47 = _20 as f64;
    _25.fld0 = _10 as u128;
    _29 = !(-53_i8);
    _16 = _34 as isize;
    _14 = _10 + _10;
    _26 = _30;
    _2 = [17856_u16,10079_u16,32893_u16,38997_u16,25743_u16,3745_u16];
    _43 = [_29,_29,_29,_29];
    _3 = _12;
    _25 = Adt48 { fld0: 208724469406485025004566613994856844291_u128 };
    _42.0 = -_29;
    _3 = _10 as u32;
    _27 = _7;
    _45 = [_9,_21,_9];
    _35 = !_15;
    _39 = [_8,_11.0,_8,_8,_19.0,_11.0];
    _28 = [22980_i16,(-10056_i16),(-31218_i16),21211_i16,22540_i16,1790_i16];
    _16 = _21 << _11.0;
    _26 = _30;
    _49 = _2;
    _30 = _26;
    _31 = _36;
    _42.1 = _47 as isize;
    _44 = _42.1;
    _40 = _30;
    Goto(bb19)
    }
    bb19 = {
    _27 = _7 << _35;
    _15 = _44 as i128;
    _43 = [_29,_42.0,_42.0,_29];
    _11.0 = _20 as u64;
    _8 = _17 as u64;
    _42.2 = [_7,_7];
    _49 = [57001_u16,57826_u16,22684_u16,40377_u16,36738_u16,37938_u16];
    _12 = _3;
    _29 = _15 as i8;
    _17 = -_31;
    _33 = _20;
    _34 = false;
    _35 = _15 - _15;
    Goto(bb20)
    }
    bb20 = {
    _32 = [_3];
    _48 = _40;
    _22 = _13;
    Goto(bb21)
    }
    bb21 = {
    _10 = -_14;
    _19.0 = _11.0;
    _38 = [(-20732_i16),2193_i16,5246_i16,(-21028_i16),18503_i16,15693_i16];
    _53.4 = [_11.0,_19.0];
    _54 = [_40,_48,_26,_40,_40,_26,_26];
    _20 = _33 + _33;
    _25.fld0 = _9 as u128;
    _9 = _21 + _44;
    _36 = _27 as f32;
    _24 = -_47;
    _56 = Adt48 { fld0: _25.fld0 };
    _53.3 = [(-8878_i16),(-23003_i16),23674_i16,(-6209_i16),10715_i16,13866_i16];
    _48 = _26;
    _5 = _49;

    _11.0 = 0;
    _43 = [_29,_29,_29,_29];
    _17 = _14 - _14;
    Goto(bb22)
    }
    bb22 = {
    _29 = -_42.0;
    _35 = _15;
    _21 = _16;
    _53.1.1 = _12 as isize;
    _53.1.1 = _42.1;
    _48 = _26;
    _16 = _42.1;
    _44 = _53.1.1;
    _17 = -_36;
    _53.1 = (_42.0, _16, _42.2);
    _25 = _56;
    _1 = -_9;
    _37 = _34 as isize;
    _33 = _20 + _20;
    _19.0 = !0;
    _36 = _19.0 as f32;
    _9 = _42.1;
    _47 = 6_usize as f64;
    _26 = _40;
    _20 = _33;
    _52 = [_42.1,_44,_53.1.1];
    _19.0 = _36 as u64;
    _29 = !_53.1.0;
    _57 = [30667_i16,5246_i16,15917_i16,11327_i16,(-14311_i16),(-17046_i16)];
    _55 = _44 as f32;
    _61.2 = [_27,_27];
    Goto(bb23)
    }
    bb23 = {
    _53.1.0 = _42.0 * _29;
    _11.0 = _34 as u64;
    _24 = _47;
    _62.fld3 = core::ptr::addr_of_mut!(_53.1);
    _42.2 = _61.2;
    _42.1 = _53.1.1 | _44;
    _62.fld1 = _12 >> _20;
    _26 = _30;
    _14 = _53.1.0 as f32;
    _53.1.2 = _42.2;
    _42 = (_53.1.0, _53.1.1, _61.2);
    _6 = _55 - _55;
    _61.2 = [_27,_27];
    _62.fld2 = _17 as f64;
    _16 = -_53.1.1;
    _45 = _52;
    _53.2 = core::ptr::addr_of!(_6);
    _26 = _40;
    _62.fld3 = core::ptr::addr_of_mut!(_53.1);
    _56 = _25;
    _53.0 = [_62.fld1,_62.fld1,_62.fld1,_62.fld1,_62.fld1,_62.fld1];
    _53.1.1 = _42.1 * _44;
    _6 = -_36;
    _61.2 = [_27,_7];
    _8 = _19.0;
    Goto(bb24)
    }
    bb24 = {
    _53.1.1 = _44;
    _61.0 = -_53.1.0;
    _52 = [_53.1.1,_53.1.1,_9];
    _48 = _26;
    _11 = _19;
    _10 = _36 - _36;
    _53.1.2 = [_27,_7];
    _43 = [_53.1.0,_42.0,_61.0,_42.0];
    _57 = _53.3;
    _64 = [_62.fld1];
    _43 = [_61.0,_42.0,_61.0,_61.0];
    _37 = _1 ^ _1;
    Goto(bb25)
    }
    bb25 = {
    _28 = _53.3;
    _36 = _6 + _10;
    _61.2 = _53.1.2;
    _55 = -_10;
    _53.1 = (_42.0, _37, _61.2);
    RET = Adt51::Variant3 { fld0: _62.fld2,fld1: _64,fld2: _53,fld3: _8,fld4: _25.fld0,fld5: _43,fld6: _52,fld7: _53.4 };
    SetDiscriminant(RET, 2);
    _48 = _40;
    _47 = _62.fld2;
    _25.fld0 = !_56.fld0;
    _62.fld0 = [_19.0,_19.0,_11.0,_19.0,_19.0,_19.0];
    _61.1 = _10 as isize;
    _47 = 195_u8 as f64;
    _53.1.2 = _42.2;
    _24 = _27 as f64;
    _22 = _13;
    _24 = _62.fld2;
    _30 = _40;
    _6 = -_55;
    _56.fld0 = _25.fld0 << _20;
    place!(Field::<*mut [i16; 6]>(Variant(RET, 2), 0)) = core::ptr::addr_of_mut!(_57);
    place!(Field::<*mut [i16; 6]>(Variant(RET, 2), 0)) = core::ptr::addr_of_mut!(_53.3);
    _63 = !6_usize;
    _53.4 = [_19.0,_8];
    _25.fld0 = _56.fld0 << _56.fld0;
    Goto(bb26)
    }
    bb26 = {
    _4 = [50793_u16,26164_u16,64179_u16,53764_u16,10455_u16,46605_u16];
    _20 = _33;
    Goto(bb27)
    }
    bb27 = {
    _53.1.2 = [_27,_27];
    Goto(bb28)
    }
    bb28 = {
    _70 = _7;
    _17 = _63 as f32;
    _42.1 = _63 as isize;
    _57 = [24186_i16,(-15540_i16),(-24165_i16),7719_i16,32364_i16,(-7083_i16)];
    _55 = _10;
    _15 = _35 + _35;
    _49 = [42480_u16,48678_u16,9378_u16,3010_u16,51929_u16,4345_u16];
    _9 = -_21;
    _60 = &_62.fld6;
    _27 = _55 as i32;
    _62.fld4 = _15;
    _14 = 92_u8 as f32;
    _53.2 = core::ptr::addr_of!(_31);
    _21 = _1 >> _25.fld0;
    RET = Adt51::Variant3 { fld0: _62.fld2,fld1: _64,fld2: _53,fld3: _8,fld4: _56.fld0,fld5: _43,fld6: _45,fld7: _53.4 };
    Goto(bb29)
    }
    bb29 = {
    Call(_76 = dump_var(19_usize, 64_usize, Move(_64), 3_usize, Move(_3), 38_usize, Move(_38), 22_usize, Move(_22)), ReturnTo(bb30), UnwindUnreachable())
    }
    bb30 = {
    Call(_76 = dump_var(19_usize, 57_usize, Move(_57), 7_usize, Move(_7), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb31), UnwindUnreachable())
    }
    bb31 = {
    Call(_76 = dump_var(19_usize, 28_usize, Move(_28), 43_usize, Move(_43), 30_usize, Move(_30), 52_usize, Move(_52)), ReturnTo(bb32), UnwindUnreachable())
    }
    bb32 = {
    Call(_76 = dump_var(19_usize, 33_usize, Move(_33), 23_usize, Move(_23), 29_usize, Move(_29), 70_usize, Move(_70)), ReturnTo(bb33), UnwindUnreachable())
    }
    bb33 = {
    Call(_76 = dump_var(19_usize, 26_usize, Move(_26), 44_usize, Move(_44), 9_usize, Move(_9), 54_usize, Move(_54)), ReturnTo(bb34), UnwindUnreachable())
    }
    bb34 = {
    Call(_76 = dump_var(19_usize, 21_usize, Move(_21), 34_usize, Move(_34), 77_usize, _77, 77_usize, _77), ReturnTo(bb35), UnwindUnreachable())
    }
    bb35 = {
    Return()
    }

    }
}
pub fn main() {
    fn0(
        std::hint::black_box(true),
        std::hint::black_box('\u{be289}'),
        std::hint::black_box(9223372036854775807_isize),
        std::hint::black_box(24_i8),
        std::hint::black_box(13914_i16),
        std::hint::black_box((-1963126301_i32)),
        std::hint::black_box((-3407012583659461294_i64)),
        std::hint::black_box((-50464232047376664440829169328549646287_i128)),
        std::hint::black_box(0_usize),
        std::hint::black_box(101_u8),
        std::hint::black_box(38232_u16),
        std::hint::black_box(2904913413_u32),
        std::hint::black_box(17095853152942221982_u64),
        std::hint::black_box(291092824882136121120749181999185014710_u128),
    );
}
impl PrintFDebug for Adt45 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt45{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub struct Adt45 {
    fld0: [u64; 6],
    fld1: u32,
    fld2: f64,
    fld3: *mut (i8, isize, [i32; 2]),
    fld4: i128,
    fld5: [char; 7],
    fld6: u16,
}
impl PrintFDebug for Adt46 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt46::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant3 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant3{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt46 {
    Variant0 {
        fld0: usize,
        fld1: (i8, isize, [i32; 2]),
        fld2: *mut char,
        fld3: u128,
        fld4: i16,
    },
    Variant1 {
        fld0: (i64, *mut (i8, isize, [i32; 2])),
        fld1: Adt45,
        fld2: [i32; 2],
        fld3: *mut u64,
        fld4: f32,
        fld5: u32,
        fld6: *mut char,
    },
    Variant2 {
        fld0: [u16; 1],
        fld1: [i16; 6],
        fld2: i8,
    },
    Variant3 {
        fld0: u16,
        fld1: (u64,),
        fld2: [usize; 3],
        fld3: u64,
        fld4: *mut u64,
        fld5: u8,
        fld6: [u32; 6],
    },
}
impl PrintFDebug for Adt47 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt47::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant3 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant3{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt47 {
    Variant0 {
        fld0: [u32; 1],
        fld1: Adt46,
        fld2: (
            [u32; 6],
            (i8, isize, [i32; 2]),
            *const f32,
            [i16; 6],
            [u64; 2],
        ),
        fld3: i8,
        fld4: [u64; 2],
        fld5: [u32; 6],
    },
    Variant1 {
        fld0: [i32; 2],
        fld1: [u64; 2],
        fld2: u32,
        fld3: [i16; 6],
        fld4: *mut [i16; 6],
        fld5: *const f32,
    },
    Variant2 {
        fld0: i64,
        fld1: *mut [usize; 3],
        fld2: (i64, *mut (i8, isize, [i32; 2])),
        fld3: i128,
        fld4: i16,
        fld5: (usize, i16),
    },
    Variant3 {
        fld0: [i8; 4],
        fld1: [char; 7],
        fld2: i32,
        fld3: i8,
        fld4: [u16; 6],
    },
}
impl PrintFDebug for Adt48 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt48{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub struct Adt48 {
    fld0: u128,
}
impl PrintFDebug for Adt49 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt49::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 { fld0, fld1 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 { fld0, fld1 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt49 {
    Variant0 { fld0: [char; 7], fld1: (u64,) },
    Variant1 { fld0: usize, fld1: (u64,) },
}
impl PrintFDebug for Adt50 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt50::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant3 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant3{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt50 {
    Variant0 {
        fld0: [u16; 6],
        fld1: *mut [usize; 3],
        fld2: u128,
        fld3: f32,
    },
    Variant1 {
        fld0: [isize; 3],
        fld1: [char; 7],
        fld2: *const f32,
        fld3: Adt45,
        fld4: [u64; 6],
    },
    Variant2 {
        fld0: [usize; 3],
        fld1: (f32, *mut [usize; 3], [i16; 6]),
        fld2: [u64; 6],
    },
    Variant3 {
        fld0: i32,
        fld1: Adt49,
        fld2: isize,
    },
}
impl PrintFDebug for Adt51 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt51::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 { fld0, fld1 } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant3 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
                fld7,
            } => {
                unsafe { printf("Variant3{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld7:\0".as_ptr() as *const c_char) };
                fld7.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt51 {
    Variant0 {
        fld0: f32,
        fld1: [isize; 3],
        fld2: isize,
        fld3: i8,
        fld4: (u64,),
        fld5: u32,
        fld6: *mut [i16; 6],
    },
    Variant1 {
        fld0: (u64,),
        fld1: u16,
        fld2: [i16; 6],
        fld3: u32,
        fld4: (i64, *mut (i8, isize, [i32; 2])),
    },
    Variant2 {
        fld0: *mut [i16; 6],
        fld1: Adt50,
    },
    Variant3 {
        fld0: f64,
        fld1: [u32; 1],
        fld2: (
            [u32; 6],
            (i8, isize, [i32; 2]),
            *const f32,
            [i16; 6],
            [u64; 2],
        ),
        fld3: u64,
        fld4: u128,
        fld5: [i8; 4],
        fld6: [isize; 3],
        fld7: [u64; 2],
    },
}
impl PrintFDebug for Adt52 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt52::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt52 {
    Variant0 {
        fld0: [isize; 3],
        fld1: i128,
        fld2: *mut [i16; 6],
    },
    Variant1 {
        fld0: Adt51,
        fld1: Adt48,
        fld2: (i8, isize, [i32; 2]),
    },
}
impl PrintFDebug for Adt53 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt53::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
                fld7,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld7:\0".as_ptr() as *const c_char) };
                fld7.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
                fld7,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld7:\0".as_ptr() as *const c_char) };
                fld7.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt53 {
    Variant0 {
        fld0: Adt47,
        fld1: (u64,),
        fld2: *mut [i16; 6],
        fld3: i8,
        fld4: Adt51,
        fld5: f64,
        fld6: Adt49,
        fld7: (i8, isize, [i32; 2]),
    },
    Variant1 {
        fld0: Adt49,
        fld1: Adt47,
        fld2: *mut [usize; 3],
        fld3: [i32; 2],
        fld4: f32,
    },
    Variant2 {
        fld0: (u64,),
        fld1: usize,
        fld2: u16,
        fld3: (i8, isize, [i32; 2]),
        fld4: *mut [usize; 3],
        fld5: f32,
        fld6: Adt48,
        fld7: [u32; 1],
    },
}
impl PrintFDebug for Adt54 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt54::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 { fld0, fld1 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt54 {
    Variant0 {
        fld0: [usize; 3],
        fld1: *mut [usize; 3],
    },
    Variant1 {
        fld0: bool,
        fld1: i128,
        fld2: Adt47,
        fld3: f64,
        fld4: [i32; 2],
        fld5: *mut [i16; 6],
    },
}
impl PrintFDebug for Adt55 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt55::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt55 {
    Variant0 {
        fld0: [usize; 3],
        fld1: (i8, isize, [i32; 2]),
        fld2: isize,
        fld3: i64,
        fld4: (
            [u32; 6],
            (i8, isize, [i32; 2]),
            *const f32,
            [i16; 6],
            [u64; 2],
        ),
    },
    Variant1 {
        fld0: u32,
        fld1: [i32; 2],
        fld2: [u16; 1],
        fld3: i8,
        fld4: [u32; 6],
        fld5: *mut [i16; 6],
    },
}
impl PrintFDebug for Adt56 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt56::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 { fld0 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 { fld0, fld1 } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt56 {
    Variant0 {
        fld0: u32,
        fld1: [char; 7],
        fld2: *mut [i16; 6],
    },
    Variant1 {
        fld0: i16,
    },
    Variant2 {
        fld0: bool,
        fld1: f32,
    },
}
impl PrintFDebug for Adt57 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt57::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 { fld0 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
                fld7,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld7:\0".as_ptr() as *const c_char) };
                fld7.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant3 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant3{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt57 {
    Variant0 {
        fld0: isize,
    },
    Variant1 {
        fld0: f64,
        fld1: [u16; 1],
        fld2: isize,
        fld3: u128,
    },
    Variant2 {
        fld0: (i8, isize, [i32; 2]),
        fld1: u32,
        fld2: u128,
        fld3: [u16; 6],
        fld4: Adt48,
        fld5: i32,
        fld6: (f32, *mut [usize; 3], [i16; 6]),
        fld7: Adt51,
    },
    Variant3 {
        fld0: *mut [i16; 6],
        fld1: u64,
        fld2: Adt49,
        fld3: Adt52,
        fld4: Adt47,
    },
}
impl PrintFDebug for Adt58 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt58::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 { fld0, fld1 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt58 {
    Variant0 {
        fld0: u128,
        fld1: Adt55,
        fld2: [usize; 3],
        fld3: Adt46,
    },
    Variant1 {
        fld0: Adt49,
        fld1: u64,
    },
    Variant2 {
        fld0: bool,
        fld1: [u64; 2],
        fld2: [i32; 2],
        fld3: Adt48,
        fld4: Adt45,
    },
}
impl PrintFDebug for Adt59 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt59::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt59 {
    Variant0 {
        fld0: Adt47,
        fld1: i128,
        fld2: Adt52,
        fld3: (i64, *mut (i8, isize, [i32; 2])),
        fld4: Adt53,
        fld5: [char; 7],
        fld6: Adt50,
    },
    Variant1 {
        fld0: bool,
        fld1: Adt46,
        fld2: Adt56,
        fld3: i8,
        fld4: *mut u64,
        fld5: Adt52,
        fld6: Adt54,
    },
    Variant2 {
        fld0: (i8, isize, [i32; 2]),
        fld1: [u16; 6],
        fld2: isize,
        fld3: Adt46,
        fld4: *mut char,
        fld5: Adt49,
        fld6: i64,
    },
}
impl PrintFDebug for Adt60 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt60::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 {
                fld0,
                fld1,
                fld2,
                fld3,
            } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 { fld0, fld1, fld2 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
                fld6,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld6:\0".as_ptr() as *const c_char) };
                fld6.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt60 {
    Variant0 {
        fld0: *mut [usize; 3],
        fld1: [u64; 6],
        fld2: Adt57,
        fld3: i128,
    },
    Variant1 {
        fld0: u16,
        fld1: Adt51,
        fld2: [u16; 6],
    },
    Variant2 {
        fld0: (f32, *mut [usize; 3], [i16; 6]),
        fld1: (
            [u32; 6],
            (i8, isize, [i32; 2]),
            *const f32,
            [i16; 6],
            [u64; 2],
        ),
        fld2: [char; 7],
        fld3: u128,
        fld4: Adt50,
        fld5: *mut (i8, isize, [i32; 2]),
        fld6: [u32; 6],
    },
}
impl PrintFDebug for Adt61 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt61::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant0 { fld0, fld1 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
                fld5,
            } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld5:\0".as_ptr() as *const c_char) };
                fld5.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant2 {
                fld0,
                fld1,
                fld2,
                fld3,
                fld4,
            } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld3:\0".as_ptr() as *const c_char) };
                fld3.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld4:\0".as_ptr() as *const c_char) };
                fld4.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant3 { fld0, fld1 } => {
                unsafe { printf("Variant3{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt61 {
    Variant0 {
        fld0: Adt56,
        fld1: f32,
    },
    Variant1 {
        fld0: Adt57,
        fld1: *mut char,
        fld2: [u32; 1],
        fld3: Adt54,
        fld4: f32,
        fld5: *mut [usize; 3],
    },
    Variant2 {
        fld0: u16,
        fld1: *mut (i8, isize, [i32; 2]),
        fld2: Adt46,
        fld3: i128,
        fld4: *mut [usize; 3],
    },
    Variant3 {
        fld0: bool,
        fld1: [isize; 3],
    },
}
