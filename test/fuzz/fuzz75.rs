#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, const_hash)]
    #![allow(unused_parens, unused_assignments, overflowing_literals,internal_features)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::ffi::{c_char, c_int};

    extern "C" {
        fn printf(fmt: *const c_char, ...) -> c_int;
    }
    trait PrintFDebug{
        unsafe fn printf_debug(&self);
    }
    impl<T:PrintFDebug> PrintFDebug for *const T{
        unsafe fn printf_debug(&self){
            unsafe{(**self).printf_debug()};
        }
    }
    impl<T:PrintFDebug> PrintFDebug for *mut T{
        unsafe fn printf_debug(&self){
            unsafe{(**self).printf_debug()};
        }
    }
    impl<T:PrintFDebug> PrintFDebug for &T{
        unsafe fn printf_debug(&self){
            (**self).printf_debug();
        }
    }
    impl<T:PrintFDebug> PrintFDebug for &mut T{
        unsafe fn printf_debug(&self){
            (**self).printf_debug();
        }
    }
    impl PrintFDebug for i8{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf("[\0".as_ptr() as *const c_char);
            for b in self{
                b.printf_debug();
                printf(",\0".as_ptr() as *const c_char);
            }
            printf("]\0".as_ptr() as *const c_char);
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf("%li\0".as_ptr() as *const c_char,*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf("%lu\0".as_ptr() as *const c_char,*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf("%lx%lx\0".as_ptr() as *const c_char, (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf("%li\0".as_ptr() as *const c_char,*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf("%lu\0".as_ptr() as *const c_char,*self as usize);
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
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",)\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
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
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
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
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
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
            printf(")\0".as_ptr() as *const c_char);
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
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
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
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
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
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
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
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
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
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
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
        var0: usize, val0: impl PrintFDebug,
        var1: usize, val1: impl PrintFDebug,
        var2: usize, val2: impl PrintFDebug,
        var3: usize, val3: impl PrintFDebug,
    ) {
        unsafe{
            printf("fn%u:_%u = \0".as_ptr() as *const c_char,f,var0);
            val0.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var1);
            val1.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var2);
            val2.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var3);
            val3.printf_debug();
            printf("\n\0".as_ptr() as *const c_char);
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: i32,mut _2: i128) -> f64 {
mir! {
type RET = f64;
let _3: (*const [u64; 2], isize, *const u8, [isize; 6]);
let _4: (i32, bool, i8, i32);
let _5: [u8; 6];
let _6: [u64; 2];
let _7: (char, usize);
let _8: ((*mut Adt23, u128, [isize; 6]), *const ([u16; 5], (char, usize), *mut Adt23, *mut Adt23), &'static (bool,));
let _9: bool;
let _10: [u64; 2];
let _11: (*mut Adt23, u128, [isize; 6]);
let _12: f32;
let _13: bool;
let _14: i16;
let _15: [i8; 2];
let _16: isize;
let _17: (u32, f64, i16);
let _18: u128;
let _19: [char; 1];
let _20: (bool,);
let _21: bool;
let _22: bool;
let _23: f32;
let _24: *const ([u16; 5], (char, usize), *mut Adt23, *mut Adt23);
let _25: i32;
let _26: Adt31;
let _27: char;
let _28: f64;
let _29: &'static f64;
let _30: ();
let _31: ();
{
_1 = (-126573887253675851841737760971437570304_i128) as i32;
RET = _1 as f64;
RET = 12306_i16 as f64;
_2 = 7159555085759806869415825334442729917_i128;
_2 = -(-18144153092064863963106531373720851591_i128);
RET = _1 as f64;
RET = 1231833366284329318_i64 as f64;
RET = _1 as f64;
_3.3 = [120_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4.0 = (-113_i8) as i32;
Goto(bb1)
}
bb1 = {
_4.2 = (-38_i8) - (-30_i8);
_4 = (_1, true, 127_i8, _1);
_4.3 = (-187_i16) as i32;
_4.1 = false;
_4 = (_1, false, (-122_i8), _1);
Call(_4.0 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.2 = 198161417427954046441916824610979373573_u128 as i8;
_3.1 = 35804_u16 as isize;
_3.3 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_5 = [64_u8,83_u8,249_u8,150_u8,162_u8,234_u8];
_7 = ('\u{c1e0b}', 3_usize);
_4.2 = 23_i8 & (-89_i8);
_1 = (-6681068902277855891_i64) as i32;
_3.0 = core::ptr::addr_of!(_6);
_7 = ('\u{58bc8}', 5_usize);
_6 = [9368608717481993115_u64,10997973334126737128_u64];
_7.0 = '\u{3498f}';
_3.3 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_4.2 = 15_i8;
RET = 2385020289_u32 as f64;
_8.0.2 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
match _4.2 {
0 => bb3,
1 => bb4,
15 => bb6,
_ => bb5
}
}
bb3 = {
_4.2 = (-38_i8) - (-30_i8);
_4 = (_1, true, 127_i8, _1);
_4.3 = (-187_i16) as i32;
_4.1 = false;
_4 = (_1, false, (-122_i8), _1);
Call(_4.0 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_8.0.2 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_5 = [37_u8,254_u8,16_u8,206_u8,26_u8,15_u8];
_3.3 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
RET = _2 as f64;
_4.2 = 83_u8 as i8;
RET = 61133_u16 as f64;
_10 = _6;
_8.0.1 = 280451154542077244032474876445687324213_u128 << _4.0;
_12 = RET as f32;
_3.3 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_7 = ('\u{77736}', 9554296509748573984_usize);
_9 = !_4.1;
_10 = _6;
_9 = _4.3 > _4.0;
_5 = [201_u8,223_u8,119_u8,105_u8,116_u8,62_u8];
_7.0 = '\u{21542}';
_4 = (_1, _9, (-96_i8), _1);
_4.0 = _1 << _4.2;
_5 = [165_u8,250_u8,29_u8,158_u8,239_u8,151_u8];
_2 = 163774704511026219343898457314123593588_i128;
Call(_9 = fn1(_8.0.2, _4, _8.0.2, _1, _7.0, _4.0, _7.1, _4, _7, _7.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = (_1, _9, (-92_i8), _1);
_15 = [_4.2,_4.2];
_7.0 = '\u{3b2f9}';
_4 = (_1, _9, (-22_i8), _1);
_11.2 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_12 = _3.1 as f32;
_7.0 = '\u{82d5}';
_5 = [227_u8,113_u8,120_u8,9_u8,241_u8,235_u8];
_13 = _9;
_5 = [105_u8,190_u8,116_u8,85_u8,214_u8,125_u8];
_3.0 = core::ptr::addr_of!(_10);
_14 = (-8316_i16) * (-26440_i16);
match _4.2 {
340282366920938463463374607431768211434 => bb8,
_ => bb1
}
}
bb8 = {
_12 = _8.0.1 as f32;
_7.0 = '\u{d9e13}';
_3.0 = core::ptr::addr_of!(_6);
_3.3 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_2 = (-142411728935897467785908462708967027447_i128) - 102140929812610285173246460774569725987_i128;
_17.0 = !1346577661_u32;
_8.0.2 = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.1];
_17.2 = _3.1 as i16;
_10 = [8565858032635020593_u64,946473336451737844_u64];
_7 = ('\u{cbbc9}', 3_usize);
_8.0.1 = 155292243982253031404257073068210229272_u128 - 150020289395183051641248501442428679433_u128;
_17.2 = _14;
_14 = _17.2;
_11.1 = !_8.0.1;
_2 = 33141293049506498723645579104871808189_i128;
_7.0 = '\u{9abd0}';
_17.1 = RET;
_11.1 = _8.0.1 - _8.0.1;
_7.0 = '\u{ff58c}';
_16 = _17.0 as isize;
_3.3 = [_16,_3.1,_16,_3.1,_3.1,_3.1];
Call(_8.0 = fn2(_15, _4.2, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11.2 = [_16,_3.1,_3.1,_16,_16,_16];
_7 = ('\u{69338}', 2_usize);
_12 = _4.2 as f32;
_10 = [4531790115159831039_u64,7311894602927294314_u64];
_15 = [_4.2,_4.2];
_4 = (_1, _13, (-113_i8), _1);
_4.3 = _4.0 + _1;
_7.0 = '\u{bd876}';
_3.1 = -_16;
_16 = _3.1 + _3.1;
_8.0.2 = _11.2;
_4.1 = !_13;
_7.1 = !3_usize;
_4.0 = !_4.3;
_5 = [181_u8,191_u8,252_u8,96_u8,19_u8,93_u8];
_9 = _4.2 < _4.2;
_17.1 = RET;
_18 = _8.0.1 * _11.1;
_17.1 = RET;
_19 = [_7.0];
_8.0.2 = _3.3;
_12 = _4.2 as f32;
_8.2 = &_20;
_22 = _9 ^ _4.1;
_18 = !_11.1;
_5 = [23_u8,23_u8,30_u8,162_u8,122_u8,125_u8];
_14 = -_17.2;
Goto(bb10)
}
bb10 = {
_4.2 = -87_i8;
_14 = _17.2 - _17.2;
_4.1 = !_22;
_3.3 = [_3.1,_16,_16,_3.1,_16,_16];
_22 = _9 > _13;
match _2 {
33141293049506498723645579104871808189 => bb12,
_ => bb11
}
}
bb11 = {
_4.2 = (-38_i8) - (-30_i8);
_4 = (_1, true, 127_i8, _1);
_4.3 = (-187_i16) as i32;
_4.1 = false;
_4 = (_1, false, (-122_i8), _1);
Call(_4.0 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_4.0 = -_1;
_23 = -_12;
_21 = !_13;
_3.3 = [_16,_3.1,_3.1,_16,_16,_16];
_20 = (_9,);
_7.0 = '\u{df5b5}';
_17.1 = RET * RET;
_8.2 = &_20;
_17.2 = _14 & _14;
_11 = (Move(_8.0.0), _8.0.1, _3.3);
RET = 3719129509324216150_i64 as f64;
Goto(bb13)
}
bb13 = {
_8.0 = (Move(_11.0), _11.1, _3.3);
_27 = _7.0;
_22 = _4.1 == _4.1;
_9 = _4.1;
match _11.1 {
220862569184014418797799008843873663971 => bb14,
_ => bb6
}
}
bb14 = {
_6 = [14447240713781965170_u64,17724274653616015689_u64];
_20.0 = !_22;
_11.1 = _18;
_25 = _2 as i32;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(0_usize, 15_usize, Move(_15), 2_usize, Move(_2), 20_usize, Move(_20), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(0_usize, 19_usize, Move(_19), 7_usize, Move(_7), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(0_usize, 27_usize, Move(_27), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [isize; 6],mut _2: (i32, bool, i8, i32),mut _3: [isize; 6],mut _4: i32,mut _5: char,mut _6: i32,mut _7: usize,mut _8: (i32, bool, i8, i32),mut _9: (char, usize),mut _10: usize) -> bool {
mir! {
type RET = bool;
let _11: u32;
let _12: [i128; 4];
let _13: Adt23;
let _14: u128;
let _15: [i128; 4];
let _16: f64;
let _17: u16;
let _18: f64;
let _19: i8;
let _20: usize;
let _21: u128;
let _22: ();
let _23: ();
{
_8.2 = _2.2;
_8.2 = _2.2;
_8.0 = _8.3;
_5 = _9.0;
_4 = !_6;
_2 = (_6, _8.1, _8.2, _6);
_6 = _2.3;
_11 = 6401487793238195895_u64 as u32;
_2.1 = _11 > _11;
_9.0 = _5;
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),15_isize,113_isize];
_4 = _6 | _2.0;
_4 = _2.0 - _2.3;
_7 = _9.1;
_14 = _4 as u128;
_6 = _4 + _2.3;
_13 = Adt23::Variant1 { fld0: _2.2 };
_9.1 = _10 ^ _10;
_11 = !1980511701_u32;
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211360 => bb5,
_ => bb4
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
_10 = (-8_isize) as usize;
_11 = !2501443639_u32;
_2.0 = -_2.3;
_13 = Adt23::Variant3 { fld0: _4,fld1: (-4703_i16),fld2: _9.1,fld3: 63536_u16 };
_8.1 = !_2.1;
_4 = _2.0;
place!(Field::<i16>(Variant(_13, 3), 1)) = (-17423_i16) - 13877_i16;
place!(Field::<i16>(Variant(_13, 3), 1)) = (-22633_i16);
_12 = [(-103836848936672698071586100000201855265_i128),(-36599370653615895602110331069478672856_i128),65374171563976034091311698266624725141_i128,49761093714134673805063840093531673145_i128];
place!(Field::<u16>(Variant(_13, 3), 3)) = _9.1 as u16;
_8.3 = _4 - Field::<i32>(Variant(_13, 3), 0);
_3 = _1;
_2.1 = _8.3 != _8.3;
_2.0 = _8.3 | Field::<i32>(Variant(_13, 3), 0);
place!(Field::<i16>(Variant(_13, 3), 1)) = _6 as i16;
_8.1 = !_2.1;
_11 = 3167055920_u32;
_2.2 = _8.2 + _8.2;
_2.0 = -_6;
_16 = _14 as f64;
_8.2 = !_2.2;
Goto(bb6)
}
bb6 = {
RET = !_2.1;
_18 = _16;
place!(Field::<i32>(Variant(_13, 3), 0)) = -_2.0;
_15 = [(-55031569703716007329276873837028859352_i128),(-101139670886410769319916734364013168593_i128),103002608423402734342954432767009657014_i128,51071341132104314997659885553366537334_i128];
place!(Field::<u16>(Variant(_13, 3), 3)) = 17138_u16 + 10514_u16;
_6 = _2.0;
SetDiscriminant(_13, 1);
_9 = (_5, _7);
_8.1 = RET ^ _2.1;
_14 = 120865234677558771106971145379576595499_u128 ^ 193762789507605641470682654417927443296_u128;
_12 = [(-64588922122485888821747513229227759110_i128),(-56229515063671313317237316368194224666_i128),3155052565394203979378565878279888451_i128,146438010113787962639364464254880570759_i128];
_11 = 772515218_u32 << _8.3;
_13 = Adt23::Variant3 { fld0: _6,fld1: (-8271_i16),fld2: _9.1,fld3: 48458_u16 };
_21 = _14;
_16 = _18 + _18;
_14 = !_21;
Goto(bb7)
}
bb7 = {
Call(_22 = dump_var(1_usize, 1_usize, Move(_1), 10_usize, Move(_10), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_22 = dump_var(1_usize, 4_usize, Move(_4), 21_usize, Move(_21), 3_usize, Move(_3), 23_usize, _23), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [i8; 2],mut _2: i8,mut _3: (i32, bool, i8, i32)) -> (*mut Adt23, u128, [isize; 6]) {
mir! {
type RET = (*mut Adt23, u128, [isize; 6]);
let _4: *mut Adt23;
let _5: u16;
let _6: (isize, u64, [u64; 2], &'static u16);
let _7: i128;
let _8: Adt23;
let _9: f32;
let _10: [i128; 4];
let _11: Adt82;
let _12: &'static (char, usize);
let _13: f32;
let _14: Adt23;
let _15: ();
let _16: ();
{
_1 = [_3.2,_3.2];
RET.1 = 13386_i16 as u128;
RET.2 = [37_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-75_isize),(-9223372036854775808_isize)];
_3 = ((-1824153946_i32), true, _2, (-1694270550_i32));
_3.2 = _2;
_2 = _3.2 * _3.2;
_3.2 = _2 - _2;
RET.2 = [(-46_isize),74_isize,9223372036854775807_isize,73_isize,90_isize,61_isize];
RET.2 = [(-18_isize),9223372036854775807_isize,(-2_isize),121_isize,9223372036854775807_isize,9223372036854775807_isize];
_3.0 = -_3.3;
RET.1 = 297603520854113266007401433770445399360_u128;
Goto(bb1)
}
bb1 = {
_5 = !31219_u16;
_6.1 = 7139291930916592995_u64 + 12475288010439531268_u64;
_1 = [_3.2,_3.2];
_3.1 = false & true;
RET.2 = [(-96_isize),(-9223372036854775808_isize),(-45_isize),7_isize,(-9223372036854775808_isize),9223372036854775807_isize];
RET.2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,66_isize];
_6.1 = !11463791719003421113_u64;
RET.2 = [59_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-10_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_6.3 = &_5;
Call(_3.0 = fn3(_1, Move(_6.3), _3.3, _1, _3.2, _2, _2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.1 = 297566144660709737637899364898369719183_u128 >> _3.2;
_6.2 = [_6.1,_6.1];
_3.2 = _2 | _2;
RET.1 = (-7114589388982022299_i64) as u128;
_3.3 = _3.0;
_3.1 = false;
_6.3 = &_5;
RET.0 = core::ptr::addr_of_mut!(_8);
_4 = core::ptr::addr_of_mut!(_8);
_8 = Adt23::Variant3 { fld0: _3.3,fld1: (-11062_i16),fld2: 0_usize,fld3: _5 };
(*_4) = Adt23::Variant3 { fld0: _3.3,fld1: (-6637_i16),fld2: 8002088462136229483_usize,fld3: _5 };
place!(Field::<i16>(Variant((*_4), 3), 1)) = 10946_i16 << _2;
(*_4) = Adt23::Variant1 { fld0: _2 };
RET.1 = 9223372036854775807_isize as u128;
Call(RET.0 = fn4(Move(_6.3), Move(_4), _2, _8, (*_4), _8, (*_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = 180_u8 as f32;
RET.0 = core::ptr::addr_of_mut!(_8);
RET.0 = core::ptr::addr_of_mut!(_8);
_6.3 = &_5;
_6.3 = &_5;
_6.0 = (-9223372036854775808_isize) & 9223372036854775807_isize;
place!(Field::<i8>(Variant(_8, 1), 0)) = _3.2;
RET.1 = 220862569184014418797799008843873663971_u128;
_6.3 = &_5;
place!(Field::<i8>(Variant(_8, 1), 0)) = _6.1 as i8;
_3.1 = _3.3 >= _3.3;
_3.2 = _2;
_3.2 = _2;
_3.2 = (-7432340261087414292_i64) as i8;
_4 = core::ptr::addr_of_mut!(_14);
_14 = Adt23::Variant3 { fld0: _3.3,fld1: 32270_i16,fld2: 4442345537738309481_usize,fld3: _5 };
RET.2 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
place!(Field::<usize>(Variant((*_4), 3), 2)) = 6485056212136950429_usize;
RET.2 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
Goto(bb4)
}
bb4 = {
Call(_15 = dump_var(2_usize, 5_usize, Move(_5), 2_usize, Move(_2), 16_usize, _16, 16_usize, _16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [i8; 2],mut _2: &'static u16,mut _3: i32,mut _4: [i8; 2],mut _5: i8,mut _6: i8,mut _7: i8,mut _8: [i8; 2]) -> i32 {
mir! {
type RET = i32;
let _9: isize;
let _10: [i128; 4];
let _11: f32;
let _12: ();
let _13: ();
{
_5 = !_6;
_5 = 19988_i16 as i8;
_5 = _7;
_1 = [_6,_5];
RET = 973024971_u32 as i32;
_10 = [(-148397860280438162681644311631346938010_i128),81968401501031838283736818949727760708_i128,(-23043591914902665068108904485700150120_i128),107784558796288510749601955369827876743_i128];
_9 = !9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_7 = _5;
_6 = _5;
_7 = _5;
_10 = [(-83561592628620674660382126832605184523_i128),(-41588397795968411542458096842224900970_i128),137468268097848063127848364376956379576_i128,(-1157191218816140577589755186878808313_i128)];
_6 = -_5;
_4 = [_7,_7];
_1 = [_6,_7];
_8 = [_7,_6];
_9 = (-4674_i16) as isize;
_11 = 22_u8 as f32;
_9 = (-29598_i16) as isize;
RET = -_3;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(3_usize, 3_usize, Move(_3), 4_usize, Move(_4), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: &'static u16,mut _2: *mut Adt23,mut _3: i8,mut _4: Adt23,mut _5: Adt23,mut _6: Adt23,mut _7: Adt23) -> *mut Adt23 {
mir! {
type RET = *mut Adt23;
let _8: [i64; 5];
let _9: isize;
let _10: char;
let _11: &'static u16;
let _12: (u32, f64, i16);
let _13: *const (([u16; 5], (char, usize), *mut Adt23, *mut Adt23), *const u8, *const u16, Adt23);
let _14: Adt23;
let _15: bool;
let _16: isize;
let _17: [char; 1];
let _18: [i16; 2];
let _19: isize;
let _20: ();
let _21: ();
{
_4 = _6;
_6 = _7;
_5 = _7;
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of_mut!(_7);
_9 = 9223372036854775807_isize - (-9223372036854775808_isize);
_10 = '\u{d3ee6}';
place!(Field::<i8>(Variant(_7, 1), 0)) = !Field::<i8>(Variant(_6, 1), 0);
_5 = (*RET);
_7 = _6;
_5 = Adt23::Variant3 { fld0: 1745290656_i32,fld1: 28064_i16,fld2: 1_usize,fld3: 47952_u16 };
place!(Field::<usize>(Variant(_5, 3), 2)) = 3820088163720072077_usize >> Field::<i8>(Variant((*RET), 1), 0);
(*RET) = _6;
_12.1 = Field::<usize>(Variant(_5, 3), 2) as f64;
place!(Field::<i32>(Variant(_5, 3), 0)) = 129706049918847694849969445986370830251_u128 as i32;
_2 = Move(RET);
_11 = &place!(Field::<u16>(Variant(_5, 3), 3));
Goto(bb2)
}
bb2 = {
_8 = [(-1372430360836816811_i64),(-801746089513872245_i64),(-4726850047816415713_i64),7590453460110370836_i64,(-4691353362555587280_i64)];
_11 = &(*_11);
place!(Field::<i16>(Variant(_5, 3), 1)) = 70905039822173076276400782814261752603_i128 as i16;
_12.2 = Field::<i16>(Variant(_5, 3), 1) * Field::<i16>(Variant(_5, 3), 1);
place!(Field::<u16>(Variant(_5, 3), 3)) = !31502_u16;
_12.2 = Field::<i16>(Variant(_5, 3), 1) + Field::<i16>(Variant(_5, 3), 1);
_1 = &place!(Field::<u16>(Variant(_5, 3), 3));
place!(Field::<i8>(Variant(_4, 1), 0)) = _3;
_12.2 = (*_1) as i16;
_2 = core::ptr::addr_of_mut!(_4);
place!(Field::<usize>(Variant(_5, 3), 2)) = !3_usize;
_11 = &(*_1);
_16 = _9 & _9;
place!(Field::<i8>(Variant(_6, 1), 0)) = -Field::<i8>(Variant(_7, 1), 0);
place!(Field::<usize>(Variant(_5, 3), 2)) = 522300569_u32 as usize;
_3 = Field::<i8>(Variant((*_2), 1), 0);
_2 = core::ptr::addr_of_mut!((*_2));
_16 = _9;
(*_2) = _6;
place!(Field::<i8>(Variant((*_2), 1), 0)) = Field::<i8>(Variant(_6, 1), 0) << _3;
Goto(bb3)
}
bb3 = {
place!(Field::<i8>(Variant(_6, 1), 0)) = 17486058313492268257_u64 as i8;
RET = core::ptr::addr_of_mut!(_6);
place!(Field::<i8>(Variant((*_2), 1), 0)) = Field::<i8>(Variant(_7, 1), 0);
place!(Field::<i8>(Variant((*RET), 1), 0)) = Field::<i8>(Variant((*_2), 1), 0) | Field::<i8>(Variant((*_2), 1), 0);
_17 = [_10];
_16 = _9;
_18 = [Field::<i16>(Variant(_5, 3), 1),_12.2];
_12.0 = !3374652484_u32;
place!(Field::<usize>(Variant(_5, 3), 2)) = 14363831477544789320_usize ^ 2_usize;
_12.2 = Field::<i16>(Variant(_5, 3), 1) & Field::<i16>(Variant(_5, 3), 1);
Goto(bb4)
}
bb4 = {
Call(_20 = dump_var(4_usize, 9_usize, Move(_9), 16_usize, Move(_16), 10_usize, Move(_10), 21_usize, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(1208229835_i32), std::hint::black_box((-128385072770910914022251804689070849623_i128)));
                
            }
impl PrintFDebug for Adt23{
	unsafe fn printf_debug(&self){unsafe{printf("Adt23::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt23 {
Variant0{
fld0: bool,
fld1: (u32, f64, i16),
fld2: i16,

},
Variant1{
fld0: i8,

},
Variant2{
fld0: i8,
fld1: (char, usize),

},
Variant3{
fld0: i32,
fld1: i16,
fld2: usize,
fld3: u16,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: bool,
fld1: u16,
fld2: isize,
fld3: i128,
fld4: [char; 1],
fld5: (Adt23, f64, (*mut f32, f32, [char; 1], u8), bool),

},
Variant1{
fld0: bool,
fld1: (i32, bool, i8, i32),
fld2: Adt23,
fld3: usize,
fld4: i16,
fld5: [char; 1],
fld6: (isize, char),
fld7: i128,

},
Variant2{
fld0: u8,
fld1: char,
fld2: *mut Adt23,
fld3: i8,
fld4: (Adt23, f64, (*mut f32, f32, [char; 1], u8), bool),
fld5: *mut f32,
fld6: [char; 1],
fld7: u64,

}}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf("Adt37::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: u8,
fld1: f32,
fld2: *const [u64; 2],

},
Variant1{
fld0: u128,
fld1: u16,

},
Variant2{
fld0: i8,
fld1: (isize, char),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: i8,

},
Variant1{
fld0: [bool; 1],
fld1: (u32, f64, i16),
fld2: Adt31,
fld3: *const u8,
fld4: [i8; 2],
fld5: [char; 1],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [char; 1],
fld1: i8,
fld2: u16,

},
Variant1{
fld0: u128,
fld1: (u32, f64, i16),
fld2: (char, usize),

},
Variant2{
fld0: Adt51,
fld1: Adt31,
fld2: usize,
fld3: u32,
fld4: i16,
fld5: u8,
fld6: (u128, Adt31, u8, char),

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: (*mut Adt23, u128, [isize; 6]),
fld1: (Adt23, f64, (*mut f32, f32, [char; 1], u8), bool),
fld2: (bool,),

},
Variant1{
fld0: bool,
fld1: u32,
fld2: isize,
fld3: f64,
fld4: usize,
fld5: f32,
fld6: *const [i64; 5],
fld7: [i128; 6],

},
Variant2{
fld0: *const (*mut f32, f32, [char; 1], u8),
fld1: (*mut f32, Adt56),
fld2: u64,

},
Variant3{
fld0: (i32, bool, i8, i32),
fld1: *mut Adt23,
fld2: [isize; 6],
fld3: f32,
fld4: u64,
fld5: *const (*mut f32, f32, [char; 1], u8),
fld6: Adt51,
fld7: [i128; 6],

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt74{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt74 {
fld0: *const Adt31,
fld1: ([u16; 5], (char, usize), *mut Adt23, *mut Adt23),
fld2: ((*mut f32, f32, [char; 1], u8), i8, [char; 1]),
fld3: Adt31,
fld4: [bool; 1],
fld5: [u64; 2],
fld6: (bool,),
fld7: usize,
}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){unsafe{printf("Adt82::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt82 {
Variant0{
fld0: u128,
fld1: f64,
fld2: *const ([u16; 5], (char, usize), *mut Adt23, *mut Adt23),

},
Variant1{
fld0: *mut [i8; 2],

}}

