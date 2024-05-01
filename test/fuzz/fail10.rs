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
    //fn8();
}


fn fn8(
) {
    let mut _9: Adt45 = Adt45::default();
    let mut _14: [u8; 7] = [14;7];
    let tmp = _14;
    _9.fld0 = tmp;
    _9.fld1 = std::hint::black_box((['5';2],['5';7],['5';2]));
    let _9 = std::hint::black_box(_9);
    _14 = _9.fld0;
    dump_var(8_usize, 14_usize, (_14), 9_usize, (_9), 0xFF_usize, (), 0xFF_usize, ());    
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

fn bob(mut _1: isize, mut _2: i8) {
    let b = std::hint::black_box(-9.000000_f32) as usize;
    dump_var(
        0xFF_usize,
        1_usize,
        (),
        2_usize,
        (b),
        3_usize,
        (),
        0xFF_usize,
        (),
    );
}
impl PrintFDebug for Adt45 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt45{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone,Default)]
pub struct Adt45 {
    fld0: [u8; 7],
    fld1: ([char; 2], [char; 7], [char; 2]),

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
            Self::Variant1 { fld0 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
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
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt51 {
    Variant0 {
        fld0: ([char; 2], [char; 7], [char; 2]),
        fld1: *const (u32, *const u128, isize),
        fld2: i64,
        fld3: Adt44,
        fld4: Adt42,
    },
    Variant1 {
        fld0: i8,
    },
    Variant2 {
        fld0: [char; 2],
        fld1: char,
        fld2: *mut [u128; 7],
    },
}
impl PrintFDebug for Adt54 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt54{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub struct Adt54 {
    fld0: usize,
    fld1: *const i32,
    fld2: *const u128,
    fld3: f64,
    fld4: *mut f64,
    fld5: Adt47,
    fld6: u128,
}

impl PrintFDebug for Adt48 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt48::\0".as_ptr() as *const c_char) };
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
pub enum Adt48 {
    Variant0 {
        fld0: (*const i128,),
        fld1: u64,
        fld2: *const (u32, *const u128, isize),
        fld3: *mut i16,
        fld4: *const i32,
        fld5: f32,
        fld6: i64,
        fld7: (u32, *const u128, isize),
    },
    Variant1 {
        fld0: *mut [u128; 7],
        fld1: f64,
        fld2: [u128; 7],
        fld3: u16,
        fld4: *mut *const i128,
        fld5: i32,
    },
}
impl PrintFDebug for Adt44 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt44{ ".as_ptr() as *const c_char) };
        unsafe { printf("}\0".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub struct Adt44 {
    fld0: [u32; 4],
}
impl PrintFDebug for Adt42 {
    unsafe fn printf_debug(&self) {
        unsafe { printf("Adt42::\0".as_ptr() as *const c_char) };
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
        }
        unsafe { printf("\0}".as_ptr() as *const c_char) };
    }
}
#[derive(Copy, Clone)]
pub enum Adt42 {
    Variant0 {
        fld0: ([char; 2], [char; 7], [char; 2]),
        fld1: *mut *const i128,
        fld2: u32,
        fld3: usize,
        fld4: f32,
        fld5: [char; 7],
    },
    Variant1 {
        fld0: bool,
        fld1: [char; 2],
        fld2: *mut *const i128,
        fld3: *mut f64,
        fld4: ([char; 2], [char; 7], [char; 2]),
        fld5: [u8; 7],
    },
    Variant2 {
        fld0: (isize, i32, *const i32, i128),
        fld1: (
            isize,
            *mut *const i128,
            isize,
            u8,
            [u128; 7],
            (*const i128,),
        ),
        fld2: *mut i16,
    },
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: i64,
fld1: ([char; 2],),
fld2: [i64; 5],
fld3: i8,
fld4: *mut i16,
}
impl<T:PrintFDebug> PrintFDebug for *mut T{
    unsafe fn printf_debug(&self){
        unsafe{(**self).printf_debug()};
    }
}
impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
    unsafe fn printf_debug(&self){
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
impl<T:PrintFDebug> PrintFDebug for *const T{
    unsafe fn printf_debug(&self){
        unsafe{(**self).printf_debug()};
    }
}