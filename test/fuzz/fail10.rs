#![recursion_limit = "1024"]
#![feature(custom_mir, core_intrinsics, const_hash)]
#![allow(
    unused_parens,
    arithmetic_overflow,
    overflowing_literals,
    internal_features
)]
extern crate core;
use core::intrinsics::mir::*;
fn main() {
    std::hint::black_box(fn4());
    std::hint::black_box(fn0());
    std::hint::black_box(fn11([false; 5], [false; 5], 'b', [0; 5]));
    std::hint::black_box(fn2());
    std::hint::black_box(fnkt());

    std::hint::black_box(bob(1, 2));
}

fn fnkt() -> isize {
    let mut RET: isize = 0;
    let _11 = std::hint::black_box(std::hint::black_box(2206060000_u32)) as isize;
    dump_var(
        1_usize,
        11_usize,
        _11,
        32_usize,
        (),
        0xFF_usize,
        (),
        0xFF_usize,
        (),
    );
    RET
}

//#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2() {
    let _2 = std::hint::black_box(214779980854582670730034556426665328640.000000) as isize;
    dump_var(
        2_usize,
        0xFF_usize,
        (),
        0xFF_usize,
        (),
        2_usize,
        (_2),
        0xFF_usize,
        (),
    );
}
//#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(
    mut _1: u32,
    mut _2: u32,
    mut _3: (u64, i128, isize),
    mut _4: i128,
    mut _5: (u64, i128, isize),
    mut _6: u32,
    mut _7: (u64, i128, isize),
    mut _8: u32,
) -> i128 {
    let _9: i64;
    let _10: *mut &'static u16;
    let _11: (u32, u8);

    _1 = '\u{bef75}' as u32;
    _2 = _6 ^ _6;
    _3.0 = _7.0 & _7.0;
    _8 = '\u{1b75}' as u32;

    let ret = !_4;
    _7.2 = !_5.2;
    _7.2 = '\u{1c53f}' as isize;

    dump_var(
        3_usize,
        3_usize,
        (_3),
        4_usize,
        (_4),
        6_usize,
        (_6),
        2_usize,
        (_2),
    );
    return ret;
}

use core::ffi::c_char;
use core::ffi::c_int;

fn fn4() -> () {
    let mut _5 = 6.6;
    _5 = 16692680020138226209_u64 as f64;
    unsafe {
        printf(
            "%f\n\0".as_ptr() as *const c_char,
            _5 as core::ffi::c_double,
        )
    };
}
pub fn fn0() -> () {
    let _5 = 2279710617_u32 as i64;
    unsafe {
        printf("%li\n\0".as_ptr() as *const c_char, _5);
    }
}

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}

pub fn fn11(mut _1: [bool; 5], mut _2: [bool; 5], mut _3: char, mut _4: [u128; 5]) -> () {
    let _5: Adt21;

    let _10: [i128; 4];
    let _12: u64;

    let tmp: u128;
    let tmp2: f64;

    let _21: isize;

    {
        _10 = [
            63888660423821798871942303771400419639_i128,
            48190258137206506221152964915320086556_i128,
            136701057553463430869935423946978219310_i128,
            107106853136610033988274367737192123988_i128,
        ];
        _12 = 7818556801315723626_usize as u64;
        tmp2 = 9223372036854775807_isize as f64;
        tmp = 3934180252_u32 as u128;
        let tmp4 = (-46_i8) as isize;
        _5 = Adt21::Variant2 {
            fld0: tmp,

            fld2: tmp4,
        };
        let _23 = _12 as isize;
        dump_var(
            11_usize,
            10_usize,
            (_10),
            23_usize,
            (_23),
            12_usize,
            _12,
            30_usize,
            (),
        )
    }
}
#[derive(Copy, Clone)]
pub enum Adt21 {
    Variant2 { fld0: u128, fld2: isize },
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
trait PrintFDebug {
    unsafe fn printf_debug(&self);
}
impl PrintFDebug for Adt21 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt21::\0".as_ptr() as *const c_char) };
        match self {
            Self::Variant2 { fld0, fld2 } => {
                unsafe { printf("Variant2{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };

                unsafe { printf("fld2:\0".as_ptr() as *const c_char) };
                fld2.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
        }
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
impl PrintFDebug for i128 {
    unsafe fn printf_debug(&self) {
        u128::printf_debug(&(*self as u128));
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
impl PrintFDebug for isize {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self as isize);
    }
}
impl PrintFDebug for u64 {
    unsafe fn printf_debug(&self) {
        printf("%lu\0".as_ptr() as *const c_char, *self);
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
impl PrintFDebug for u32 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self);
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
impl PrintFDebug for i16 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self as i16 as c_int);
    }
}
impl PrintFDebug for u8 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u8 as c_int);
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
impl PrintFDebug for u128 {
    unsafe fn printf_debug(&self) {
        printf(
            "%lx%lx\0".as_ptr() as *const c_char,
            (*self >> 64) as u64,
            *self as u64,
        );
    }
}
impl<T: PrintFDebug> PrintFDebug for &T {
    unsafe fn printf_debug(&self) {
        (**self).printf_debug();
    }
}
impl PrintFDebug for char {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u32);
    }
}
impl PrintFDebug for u16 {
    unsafe fn printf_debug(&self) {
        printf("%u\0".as_ptr() as *const c_char, *self as u16 as c_int);
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

impl PrintFDebug for i64 {
    unsafe fn printf_debug(&self) {
        printf("%li\0".as_ptr() as *const c_char, *self);
    }
}
impl PrintFDebug for i32 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self);
    }
}

impl PrintFDebug for Adt60 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt60{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone, Default)]
pub struct Adt60 {
    fld0: u32,

    fld2: (([i64; 2], [i64; 2]), f32, (i32, i128)),
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
impl PrintFDebug for i8 {
    unsafe fn printf_debug(&self) {
        printf("%i\0".as_ptr() as *const c_char, *self as i8 as c_int);
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
impl<A: PrintFDebug> PrintFDebug for (A,) {
    unsafe fn printf_debug(&self) {
        printf("(\0".as_ptr() as *const c_char);
        self.0.printf_debug();
        printf(",)\0".as_ptr() as *const c_char);
    }
}
impl PrintFDebug for usize {
    unsafe fn printf_debug(&self) {
        printf("%lu\0".as_ptr() as *const c_char, *self as usize);
    }
}

fn bob(mut _1: isize, mut _2: i8){
    let b = std::hint::black_box(-9.000000_f32) as usize;
    dump_var(0xFF_usize, 1_usize, (), 2_usize, (b),3_usize, (), 0xFF_usize, ());
    
   
}
