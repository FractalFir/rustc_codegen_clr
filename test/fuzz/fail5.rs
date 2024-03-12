#![recursion_limit = "1024"]
#![feature(custom_mir, core_intrinsics, const_hash)]
#![allow(
    unused_parens,
    unused_assignments,
    overflowing_literals,
    internal_features
)]
//#![deny(dead_code)]
extern crate core;
use core::intrinsics::mir::*;
#[derive(Copy, Clone)]
pub enum Adt44 {
    Variant0 {
        fld0: Adt37,
        fld1: *mut u128,
        fld2: i128,
        fld3: (u8, u64, (u8, i64, u64)),
        fld4: usize,
        fld5: u8,
        fld6: [usize; 2],
    },
    Variant1 {
        fld0: [i64; 6],
        fld1: char,
    },
    Variant2 {
        fld0: *const usize,
        fld1: *mut [i64; 6],
    },
    Variant3 {
        fld0: *const *const isize,
        fld1: [usize; 2],
        fld2: u8,
        fld3: i8,
        fld4: i16,
        fld5: *mut u128,
        fld6: Adt27,
        fld7: i128,
    },
}
#[derive(Copy, Clone)]
pub enum Adt27 {
    Variant0 {
        fld0: (i128, char, i32, u32),
        fld1: i128,
        fld2: i16,
        fld3: u16,
    },
    Variant1 {
        fld0: *const isize,
        fld1: *const i16,
        fld2: (u128, [i64; 6]),
    },
    Variant2 {
        fld0: f64,
        fld1: *const isize,
        fld2: isize,
        fld3: i8,
        fld4: (i128, char, i32, u32),
        fld5: (u128, [i64; 6]),
    },
}
#[derive(Copy, Clone)]
pub enum Adt37 {
    Variant0 {
        fld0: f64,
        fld1: usize,
        fld2: [i64; 6],
        fld3: i8,
        fld4: *const i128,
    },
    Variant1 {
        fld0: i128,
        fld1: *const *const isize,
        fld2: *const i16,
    },
}
use std::ffi::{c_char, c_int};


#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: char) -> i128 {
    mir! {
    type RET = i128;
    let _3: Adt44;
    let _5: f64;
    let _11: ();
    let _12: ();
    {

    let arr = [328435303712648957_i64,(-2628818876864231610_i64),(-745956930460735314_i64),(-6077391193087132748_i64),4617320432413299220_i64,(-1532122738878591976_i64)];
    let val2 =  963426893_u32 as i128;
    _5 = val2 as f64;
    _3 = Adt44::Variant1 { fld0: arr,fld1: '\u{5a92f}' };

    Call(RET = fn9(Move(_3), arr, arr, _1, arr, _5, _1), ReturnTo(bb7), UnwindUnreachable())
    }
    bb7 = {
    let val = 6565145176971011887_i64 | 6565145176971011887_i64;
    RET = (-142109229527252706957065360219398948335_i128) >> val;
    Return()
    }

    }
}

fn fn9(
    mut _1: Adt44,
    mut _2: [i64; 6],
    mut _3: [i64; 6],
    mut _4: char,
    mut _5: [i64; 6],
    mut _6: f64,
    mut _7: char,
) -> i128 {
    core::hint::black_box(64_i128)
}
pub fn main() {
    fn8(std::hint::black_box(('U')));
}