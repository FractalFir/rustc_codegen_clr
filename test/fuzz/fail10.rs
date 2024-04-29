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
fn main() {
    std::hint::black_box(fn4());
    std::hint::black_box(fn0());
    std::hint::black_box(fn11([false;5],[false;5],'b',[0;5]));
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [bool; 5], mut _2: [bool; 5], mut _3: char, mut _4: [u128; 5]) -> () {
    mir! {
    type RET = ();
    let _5: Adt21;

    let _10: [i128; 4];
    let _12: u64;
   
    let tmp:u128;
    let tmp2:f64;
    let tmp3:u32;
    let _16: i16;
    let _21: isize;
    let _23: isize;

    let _30: ();
    let _31: ();
    {
    _10 = [63888660423821798871942303771400419639_i128,48190258137206506221152964915320086556_i128,136701057553463430869935423946978219310_i128,107106853136610033988274367737192123988_i128];
    _12 = 7818556801315723626_usize as u64; 
    tmp2 = 9223372036854775807_isize as f64;
    tmp = 3934180252_u32 as u128;
    let tmp4 = (-46_i8) as isize;
    _5 = Adt21::Variant2 { fld0: tmp,fld1: tmp2,fld2: tmp4};  
    _23 = _12 as isize;

    Call(_30 = dump_var(11_usize, 10_usize, Move(_10), 23_usize, Move(_23), 12_usize, _12, 30_usize, _30), ReturnTo(t1), UnwindUnreachable())
    }
    t1 = {
    _23 = tmp as isize;
    Call(_30 = dump_var(11_usize, 5_usize, Move(_5), 23_usize, Move(_23), 5_usize, _5, 30_usize, _30), ReturnTo(t0), UnwindUnreachable())
    }
    t0 = {
    Return()
    }

    }
}
#[derive(Copy, Clone)]
pub enum Adt21 {
    Variant0 {
        fld0: u64,
        fld1: isize,
    },
    Variant1 {
        fld0: u16,
    },
    Variant2 {
        fld0: u128,
        fld1: f64,
        fld2: isize,
    },
    Variant3 {
        fld0: u16,
        fld1: char,
        fld2: f32,
        fld3: u32,
        fld4: [i16; 4],
    },
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
            Self::Variant0 { fld0, fld1 } => {
                unsafe { printf("Variant0{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
                unsafe { printf("fld1:\0".as_ptr() as *const c_char) };
                fld1.printf_debug();
                unsafe { printf(",\0".as_ptr() as *const c_char) };
            }
            Self::Variant1 { fld0 } => {
                unsafe { printf("Variant1{\0".as_ptr() as *const c_char) };
                unsafe { printf("fld0:\0".as_ptr() as *const c_char) };
                fld0.printf_debug();
                unsafe { printf(",".as_ptr() as *const c_char) };
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
impl PrintFDebug for u8{
    unsafe fn printf_debug(&self){
        printf("%u\0".as_ptr() as *const c_char,*self as u8 as c_int);
    }
} 
impl PrintFDebug for bool{
    unsafe fn printf_debug(&self){
        if *self{
            printf("true\0".as_ptr() as *const c_char);
        }
        else{
            printf("false\0".as_ptr() as *const c_char);
        }
    }
}
impl PrintFDebug for (){
    unsafe fn printf_debug(&self){
        printf("()\0".as_ptr() as *const c_char);
    }
} 
impl PrintFDebug for u128{
    unsafe fn printf_debug(&self){
        printf("%lx%lx\0".as_ptr() as *const c_char, (*self >> 64) as u64,*self as u64);
    }
}
impl<T:PrintFDebug> PrintFDebug for &T{
    unsafe fn printf_debug(&self){
        (**self).printf_debug();
    }
}
impl PrintFDebug for char{
    unsafe fn printf_debug(&self){
        printf("%u\0".as_ptr() as *const c_char,*self as u32);
    }
}
impl PrintFDebug for u16{
    unsafe fn printf_debug(&self){
        printf("%u\0".as_ptr() as *const c_char,*self as u16 as c_int);
    }
} 
impl PrintFDebug for f32{
    unsafe fn printf_debug(&self){
        printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
    }
}