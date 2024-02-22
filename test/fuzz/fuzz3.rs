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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _15: *mut isize;
let _16: u8;
let _17: [u32; 8];
let _18: u64;
let _19: Adt53;
let _20: u8;
let _21: i128;
let _22: [u32; 8];
let _23: ([u32; 1], u32, u8);
let _24: [u32; 1];
let _25: (i16,);
let _26: i8;
let _27: *mut isize;
let _28: [bool; 6];
let _29: u128;
let _30: i16;
let _31: *mut *mut *const i128;
let _32: u16;
let _33: [i128; 5];
let _34: [char; 4];
let _35: *mut *const i128;
let _36: [char; 4];
let _37: [i128; 5];
let _38: char;
let _39: u8;
let _40: i64;
let _41: isize;
let _42: &'static f64;
let _43: ();
let _44: ();
{
_12 = 2406548343_u32 * 1139234934_u32;
_14 = (-53_isize) as u128;
_9 = 7_usize;
_7 = 3866471358681855109_i64 | (-5541233624075935626_i64);
_13 = 6320421768169382220_u64 & 748199249179466249_u64;
_2 = '\u{b842a}';
RET = [(-109_isize),88_isize,(-9_isize)];
_4 = 122_i8;
RET = [(-120_isize),(-49_isize),122_isize];
_10 = !213_u8;
_6 = true as i32;
Goto(bb1)
}
bb1 = {
_12 = 857001969_u32;
_6 = 1670868138_i32 + 1487261145_i32;
_8 = 6729055135822275399093505040039861696_i128 * (-41979505578651861791188131442469424327_i128);
_11 = 11350_u16 + 55910_u16;
_9 = 10225507307617099593_usize;
_8 = 52396285534752018821308216660710540624_i128;
Goto(bb2)
}
bb2 = {
_3 = 9223372036854775807_isize | (-9223372036854775808_isize);
_13 = _2 as u64;
_15 = core::ptr::addr_of_mut!(_3);
_12 = _4 as u32;
(*_15) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_11 = 60928_u16;
_16 = _10 >> (*_15);
(*_15) = _12 as isize;
_7 = _10 as i64;
_2 = '\u{744ea}';
(*_15) = (-9223372036854775808_isize) ^ (-33_isize);
RET = [(*_15),(*_15),_3];
_1 = true;
_7 = _9 as i64;
_3 = 103_isize << _11;
_15 = core::ptr::addr_of_mut!((*_15));
_9 = _4 as usize;
_2 = '\u{a76f8}';
(*_15) = (-9223372036854775808_isize) & (-95_isize);
_5 = (-24553_i16);
_1 = !true;
_17 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = [(*_15),(*_15),_3];
_11 = 41975_u16 * 23704_u16;
_1 = false ^ false;
_12 = _5 as u32;
(*_15) = !(-9223372036854775808_isize);
Goto(bb3)
}
bb3 = {
RET = [(*_15),_3,(*_15)];
(*_15) = !(-26_isize);
_13 = _9 as u64;
_5 = 6566_i16;
_7 = !(-1053240669360810567_i64);
_22 = _17;
_18 = !_13;
_6 = 1161742645_i32 * 1700329498_i32;
(*_15) = 22_isize - (-61_isize);
_23.2 = _11 as u8;
_24 = [_12];
_21 = _9 as i128;
_5 = 26286_i16 * (-20650_i16);
_22 = _17;
_16 = _10;
_1 = false;
_15 = core::ptr::addr_of_mut!(_3);
_7 = !(-6878644175386182665_i64);
_19.fld0 = [_21,_8,_21,_8,_8];
_4 = 39_i8;
_15 = core::ptr::addr_of_mut!((*_15));
_3 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_14 = 316551771070869668232169561924655937219_u128 << _21;
_15 = core::ptr::addr_of_mut!((*_15));
Goto(bb4)
}
bb4 = {
_15 = core::ptr::addr_of_mut!(_3);
_25.0 = _9 as i16;
_20 = _16;
_19.fld0 = [_8,_21,_8,_21,_21];
_3 = 76_isize;
_7 = (-6706435277361611391_i64) + 8805424957428938456_i64;
(*_15) = 112_isize;
_16 = _23.2;
_14 = 64322527333969514706792969423366724155_u128 - 290727618261431730144595170221439967731_u128;
_14 = !30360963099347441539844555488955332879_u128;
_28 = [_1,_1,_1,_1,_1,_1];
_13 = !_18;
RET = [(*_15),(*_15),(*_15)];
RET = [(*_15),(*_15),(*_15)];
_23.1 = _12;
_3 = 9223372036854775807_isize & 9223372036854775807_isize;
_11 = 15820_u16 | 19659_u16;
_26 = _4;
_6 = _1 as i32;
_33 = [_8,_8,_8,_21,_21];
(*_15) = _1 as isize;
Goto(bb5)
}
bb5 = {
_26 = _11 as i8;
_18 = !_13;
_29 = _14 - _14;
_25.0 = _9 as i16;
_23.1 = _12;
_6 = _1 as i32;
_23.2 = _16 + _10;
_4 = _9 as i8;
_33 = [_8,_8,_21,_8,_8];
_21 = -_8;
_32 = _11;
_2 = '\u{640e1}';
_17 = [_23.1,_12,_12,_12,_23.1,_23.1,_12,_12];
_21 = _6 as i128;
_23.0 = _24;
_22 = [_23.1,_23.1,_12,_23.1,_23.1,_12,_23.1,_12];
(*_15) = -48_isize;
_13 = _7 as u64;
_10 = _23.2 + _20;
_20 = _16 | _16;
_8 = _21 >> _12;
_19 = Adt53 { fld0: _33 };
_1 = !true;
Call(_32 = fn1(_23, _10, _20, (*_15), _15, _6, _7, _33, _23.2, _10, _13, _16, _14, _8, _5, _20), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19.fld0 = [_8,_21,_21,_8,_8];
_36 = [_2,_2,_2,_2];
_19.fld0 = _33;
_23.0 = _24;
_4 = (*_15) as i8;
_33 = [_21,_8,_21,_21,_8];
_23 = (_24, _12, _16);
_19.fld0 = [_8,_8,_8,_21,_8];
_1 = !false;
_19.fld0 = [_8,_8,_8,_8,_21];
_10 = !_20;
_27 = core::ptr::addr_of_mut!(_3);
_23 = (_24, _12, _20);
_32 = _11 | _11;
_9 = 15801968115147491253_usize - 1_usize;
_14 = _29;
_23.2 = _2 as u8;
Call(_25 = fn17(_27, (*_15), _2, _27, _27, (*_15), _13, (*_15), RET, _32, _3, (*_27), _27, _15, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = [_3,(*_15),_3];
_6 = _11 as i32;
_31 = core::ptr::addr_of_mut!(_35);
_8 = _21;
_23.0 = [_12];
_20 = _11 as u8;
_32 = !_11;
_7 = 8047258483964420727_i64;
_1 = _23.2 <= _10;
_34 = _36;
_6 = (-1479868789_i32);
_37 = [_8,_21,_8,_8,_21];
_4 = _6 as i8;
_20 = _10;
RET = [(*_15),(*_27),(*_27)];
(*_15) = 97_isize | 9223372036854775807_isize;
(*_27) = 9223372036854775807_isize;
RET = [(*_15),(*_15),(*_15)];
_11 = _32 ^ _32;
_27 = _15;
_23.1 = _12 + _12;
_23.2 = _20 * _20;
_36 = [_2,_2,_2,_2];
_40 = !_7;
RET = [(*_15),(*_27),_3];
_33 = [_8,_8,_8,_21,_21];
_27 = core::ptr::addr_of_mut!((*_15));
_23.2 = !_20;
match (*_27) {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb9,
9223372036854775807 => bb11,
_ => bb10
}
}
bb8 = {
_12 = 857001969_u32;
_6 = 1670868138_i32 + 1487261145_i32;
_8 = 6729055135822275399093505040039861696_i128 * (-41979505578651861791188131442469424327_i128);
_11 = 11350_u16 + 55910_u16;
_9 = 10225507307617099593_usize;
_8 = 52396285534752018821308216660710540624_i128;
Goto(bb2)
}
bb9 = {
_26 = _11 as i8;
_18 = !_13;
_29 = _14 - _14;
_25.0 = _9 as i16;
_23.1 = _12;
_6 = _1 as i32;
_23.2 = _16 + _10;
_4 = _9 as i8;
_33 = [_8,_8,_21,_8,_8];
_21 = -_8;
_32 = _11;
_2 = '\u{640e1}';
_17 = [_23.1,_12,_12,_12,_23.1,_23.1,_12,_12];
_21 = _6 as i128;
_23.0 = _24;
_22 = [_23.1,_23.1,_12,_23.1,_23.1,_12,_23.1,_12];
(*_15) = -48_isize;
_13 = _7 as u64;
_10 = _23.2 + _20;
_20 = _16 | _16;
_8 = _21 >> _12;
_19 = Adt53 { fld0: _33 };
_1 = !true;
Call(_32 = fn1(_23, _10, _20, (*_15), _15, _6, _7, _33, _23.2, _10, _13, _16, _14, _8, _5, _20), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_15 = core::ptr::addr_of_mut!(_3);
_25.0 = _9 as i16;
_20 = _16;
_19.fld0 = [_8,_21,_8,_21,_21];
_3 = 76_isize;
_7 = (-6706435277361611391_i64) + 8805424957428938456_i64;
(*_15) = 112_isize;
_16 = _23.2;
_14 = 64322527333969514706792969423366724155_u128 - 290727618261431730144595170221439967731_u128;
_14 = !30360963099347441539844555488955332879_u128;
_28 = [_1,_1,_1,_1,_1,_1];
_13 = !_18;
RET = [(*_15),(*_15),(*_15)];
RET = [(*_15),(*_15),(*_15)];
_23.1 = _12;
_3 = 9223372036854775807_isize & 9223372036854775807_isize;
_11 = 15820_u16 | 19659_u16;
_26 = _4;
_6 = _1 as i32;
_33 = [_8,_8,_8,_21,_21];
(*_15) = _1 as isize;
Goto(bb5)
}
bb11 = {
_39 = _10 + _16;
_6 = (-2000587359_i32) << _13;
_38 = _2;
(*_15) = _12 as isize;
(*_15) = _8 as isize;
_38 = _2;
_10 = !_16;
_30 = !_5;
_39 = _20 << _16;
_24 = [_23.1];
_17 = _22;
RET = [(*_15),(*_27),(*_15)];
_21 = _8;
_28 = [_1,_1,_1,_1,_1,_1];
_38 = _2;
_4 = !_26;
_3 = (-9223372036854775808_isize);
Call(_6 = fn18(_2, _8, _8, _24, _1, _3, _23.2, (*_15), _1, _11, _25, _16, _15, _20, _13, _12), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = [_3,(*_15),(*_15)];
_23.2 = _32 as u8;
Goto(bb13)
}
bb13 = {
Call(_43 = dump_var(0_usize, 40_usize, Move(_40), 18_usize, Move(_18), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_43 = dump_var(0_usize, 6_usize, Move(_6), 21_usize, Move(_21), 36_usize, Move(_36), 17_usize, Move(_17)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_43 = dump_var(0_usize, 33_usize, Move(_33), 20_usize, Move(_20), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(0_usize, 24_usize, Move(_24), 37_usize, Move(_37), 22_usize, Move(_22), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(0_usize, 34_usize, Move(_34), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: ([u32; 1], u32, u8),mut _2: u8,mut _3: u8,mut _4: isize,mut _5: *mut isize,mut _6: i32,mut _7: i64,mut _8: [i128; 5],mut _9: u8,mut _10: u8,mut _11: u64,mut _12: u8,mut _13: u128,mut _14: i128,mut _15: i16,mut _16: u8) -> u16 {
mir! {
type RET = u16;
let _17: bool;
let _18: bool;
let _19: Adt53;
let _20: isize;
let _21: [i128; 3];
let _22: [i128; 3];
let _23: Adt42;
let _24: &'static f64;
let _25: [u32; 8];
let _26: Adt50;
let _27: [i128; 5];
let _28: i8;
let _29: (i16,);
let _30: [isize; 3];
let _31: Adt49;
let _32: (*const i128,);
let _33: Adt49;
let _34: (u128,);
let _35: isize;
let _36: f64;
let _37: (u128, u8);
let _38: isize;
let _39: bool;
let _40: [i128; 5];
let _41: [i128; 3];
let _42: Adt42;
let _43: isize;
let _44: isize;
let _45: [bool; 6];
let _46: i128;
let _47: f64;
let _48: ();
let _49: ();
{
_3 = !_1.2;
_8 = [_14,_14,_14,_14,_14];
_3 = _10 | _16;
_11 = 5730771170971470345_u64 ^ 1883028570625215995_u64;
RET = 16744_u16 ^ 62596_u16;
_12 = _2 >> _13;
_7 = (-108_i8) as i64;
_19.fld0 = [_14,_14,_14,_14,_14];
RET = 37050_u16;
_17 = !false;
_10 = _3 ^ _3;
Call(_11 = core::intrinsics::transmute((*_5)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_12;
_16 = _12;
_13 = _12 as u128;
RET = 38249_u16 * 61279_u16;
_1.1 = 2209446673_u32 << _10;
RET = 40945_u16;
_10 = _16 | _3;
_19.fld0 = [_14,_14,_14,_14,_14];
_18 = _17 ^ _17;
_19.fld0 = [_14,_14,_14,_14,_14];
_10 = _3;
_12 = !_2;
Call(_17 = fn2(_1, _5, _1, _6, _10, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _1.2 + _10;
RET = 2760_u16 | 15961_u16;
_1.2 = _3;
_19.fld0 = _8;
_7 = (-8204031919647102745_i64) | 6190269922660357106_i64;
_1.1 = 4136421743_u32;
_21 = [_14,_14,_14];
_10 = _11 as u8;
_19 = Adt53 { fld0: _8 };
_8 = [_14,_14,_14,_14,_14];
_12 = !_9;
_18 = _17;
_22 = [_14,_14,_14];
_12 = (*_5) as u8;
_5 = core::ptr::addr_of_mut!(_4);
_21 = [_14,_14,_14];
_19.fld0 = _8;
_13 = 117335142953386145794543685293439587108_u128 >> _10;
_19 = Adt53 { fld0: _8 };
match _1.1 {
0 => bb1,
1 => bb3,
4136421743 => bb5,
_ => bb4
}
}
bb3 = {
_3 = !_12;
_16 = _12;
_13 = _12 as u128;
RET = 38249_u16 * 61279_u16;
_1.1 = 2209446673_u32 << _10;
RET = 40945_u16;
_10 = _16 | _3;
_19.fld0 = [_14,_14,_14,_14,_14];
_18 = _17 ^ _17;
_19.fld0 = [_14,_14,_14,_14,_14];
_10 = _3;
_12 = !_2;
Call(_17 = fn2(_1, _5, _1, _6, _10, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_14 = 42274546857149987023726072395488332620_i128;
_4 = (-73_isize) & (-13_isize);
_11 = 6844620601011532179_u64 | 1095377643246656406_u64;
_20 = (*_5) * _4;
_1.2 = _2 | _3;
Goto(bb6)
}
bb6 = {
_17 = _18 == _18;
_3 = _9;
_6 = -(-1460134730_i32);
RET = !6884_u16;
RET = _13 as u16;
Goto(bb7)
}
bb7 = {
_2 = _1.2 & _1.2;
_22 = [_14,_14,_14];
(*_5) = _20 + _20;
_7 = _17 as i64;
_19 = Adt53 { fld0: _8 };
_18 = _7 <= _7;
_16 = _15 as u8;
_20 = !(*_5);
_1.0 = [_1.1];
_10 = _15 as u8;
_17 = _7 != _7;
_20 = (*_5) >> _2;
_3 = _2;
_1.2 = _2 - _2;
RET = _1.2 as u16;
(*_5) = (-113_i8) as isize;
_6 = (-907328177_i32) << _7;
RET = !21096_u16;
_19.fld0 = [_14,_14,_14,_14,_14];
_12 = _2 >> _1.1;
_25 = [_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1];
_14 = (-3136092221323292611108434812046202604_i128);
_17 = _18;
_11 = _7 as u64;
Goto(bb8)
}
bb8 = {
_2 = _3;
_19.fld0 = [_14,_14,_14,_14,_14];
_3 = (-74_i8) as u8;
match _14 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb6,
4 => bb9,
5 => bb10,
337146274699615170852266172619722008852 => bb12,
_ => bb11
}
}
bb9 = {
_2 = _1.2 & _1.2;
_22 = [_14,_14,_14];
(*_5) = _20 + _20;
_7 = _17 as i64;
_19 = Adt53 { fld0: _8 };
_18 = _7 <= _7;
_16 = _15 as u8;
_20 = !(*_5);
_1.0 = [_1.1];
_10 = _15 as u8;
_17 = _7 != _7;
_20 = (*_5) >> _2;
_3 = _2;
_1.2 = _2 - _2;
RET = _1.2 as u16;
(*_5) = (-113_i8) as isize;
_6 = (-907328177_i32) << _7;
RET = !21096_u16;
_19.fld0 = [_14,_14,_14,_14,_14];
_12 = _2 >> _1.1;
_25 = [_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1];
_14 = (-3136092221323292611108434812046202604_i128);
_17 = _18;
_11 = _7 as u64;
Goto(bb8)
}
bb10 = {
_3 = !_12;
_16 = _12;
_13 = _12 as u128;
RET = 38249_u16 * 61279_u16;
_1.1 = 2209446673_u32 << _10;
RET = 40945_u16;
_10 = _16 | _3;
_19.fld0 = [_14,_14,_14,_14,_14];
_18 = _17 ^ _17;
_19.fld0 = [_14,_14,_14,_14,_14];
_10 = _3;
_12 = !_2;
Call(_17 = fn2(_1, _5, _1, _6, _10, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
_13 = 122703030910325281057984338871687046818_u128;
_29.0 = _15 | _15;
_31.fld0 = 17191989994910203856_usize as isize;
_14 = (-75723593742528890179838477790286697487_i128);
_20 = !_4;
_29.0 = !_15;
_7 = _18 as i64;
_12 = _1.2;
_16 = !_1.2;
_8 = [_14,_14,_14,_14,_14];
_1.1 = 552954475_u32 * 1455508144_u32;
_21 = _22;
_1.2 = _18 as u8;
_1.2 = _2 ^ _12;
_4 = !_20;
Goto(bb13)
}
bb13 = {
RET = 966603199302331924_usize as u16;
_11 = 14577966794253872987_u64 ^ 3948027782627095079_u64;
RET = 42575_u16;
RET = 27134_u16;
_32.0 = core::ptr::addr_of!(_14);
_31 = Adt49 { fld0: (*_5) };
(*_5) = _31.fld0 << _2;
_14 = 56700603968339592884117646286988949609_i128;
_8 = _19.fld0;
_28 = -(-74_i8);
_24 = &_36;
_4 = _20;
_31.fld0 = _4 >> _6;
_14 = !(-112490657022610818504393243483544190512_i128);
_5 = core::ptr::addr_of_mut!((*_5));
_34.0 = !_13;
_31 = Adt49 { fld0: _20 };
_4 = '\u{106db4}' as isize;
_21 = _22;
_14 = (-81560034260828592643235480299746375254_i128);
_34.0 = _13 >> _6;
(*_5) = -_20;
_7 = 9182778054685976718_i64 >> _34.0;
_3 = _12;
_37.1 = _2;
match _13 {
0 => bb10,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
122703030910325281057984338871687046818 => bb19,
_ => bb18
}
}
bb14 = {
_2 = _1.2 & _1.2;
_22 = [_14,_14,_14];
(*_5) = _20 + _20;
_7 = _17 as i64;
_19 = Adt53 { fld0: _8 };
_18 = _7 <= _7;
_16 = _15 as u8;
_20 = !(*_5);
_1.0 = [_1.1];
_10 = _15 as u8;
_17 = _7 != _7;
_20 = (*_5) >> _2;
_3 = _2;
_1.2 = _2 - _2;
RET = _1.2 as u16;
(*_5) = (-113_i8) as isize;
_6 = (-907328177_i32) << _7;
RET = !21096_u16;
_19.fld0 = [_14,_14,_14,_14,_14];
_12 = _2 >> _1.1;
_25 = [_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1,_1.1];
_14 = (-3136092221323292611108434812046202604_i128);
_17 = _18;
_11 = _7 as u64;
Goto(bb8)
}
bb15 = {
_17 = _18 == _18;
_3 = _9;
_6 = -(-1460134730_i32);
RET = !6884_u16;
RET = _13 as u16;
Goto(bb7)
}
bb16 = {
_3 = !_12;
_16 = _12;
_13 = _12 as u128;
RET = 38249_u16 * 61279_u16;
_1.1 = 2209446673_u32 << _10;
RET = 40945_u16;
_10 = _16 | _3;
_19.fld0 = [_14,_14,_14,_14,_14];
_18 = _17 ^ _17;
_19.fld0 = [_14,_14,_14,_14,_14];
_10 = _3;
_12 = !_2;
Call(_17 = fn2(_1, _5, _1, _6, _10, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_14 = 42274546857149987023726072395488332620_i128;
_4 = (-73_isize) & (-13_isize);
_11 = 6844620601011532179_u64 | 1095377643246656406_u64;
_20 = (*_5) * _4;
_1.2 = _2 | _3;
Goto(bb6)
}
bb18 = {
_2 = _3;
_19.fld0 = [_14,_14,_14,_14,_14];
_3 = (-74_i8) as u8;
match _14 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb6,
4 => bb9,
5 => bb10,
337146274699615170852266172619722008852 => bb12,
_ => bb11
}
}
bb19 = {
_23 = Adt42::Variant2 { fld0: _5,fld1: _1.1 };
_36 = _34.0 as f64;
SetDiscriminant(_23, 2);
_17 = _7 != _7;
_8 = [_14,_14,_14,_14,_14];
_10 = !_1.2;
_1.2 = !_37.1;
(*_5) = RET as isize;
_34.0 = _13;
_44 = !(*_5);
_23 = Adt42::Variant2 { fld0: _5,fld1: _1.1 };
_10 = !_12;
_30 = [_4,_4,_4];
_35 = _17 as isize;
_6 = (-624373337_i32);
_1.2 = _16;
_4 = _15 as isize;
SetDiscriminant(_23, 3);
_27 = [_14,_14,_14,_14,_14];
_3 = _36 as u8;
_19 = Adt53 { fld0: _27 };
Goto(bb20)
}
bb20 = {
Call(_48 = dump_var(1_usize, 12_usize, Move(_12), 27_usize, Move(_27), 21_usize, Move(_21), 6_usize, Move(_6)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_48 = dump_var(1_usize, 9_usize, Move(_9), 7_usize, Move(_7), 30_usize, Move(_30), 2_usize, Move(_2)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_48 = dump_var(1_usize, 44_usize, Move(_44), 34_usize, Move(_34), 10_usize, Move(_10), 20_usize, Move(_20)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_48 = dump_var(1_usize, 18_usize, Move(_18), 13_usize, Move(_13), 49_usize, _49, 49_usize, _49), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: ([u32; 1], u32, u8),mut _2: *mut isize,mut _3: ([u32; 1], u32, u8),mut _4: i32,mut _5: u8,mut _6: ([u32; 1], u32, u8),mut _7: ([u32; 1], u32, u8)) -> bool {
mir! {
type RET = bool;
let _8: Adt47;
let _9: (u128, u8);
let _10: [isize; 3];
let _11: isize;
let _12: Adt53;
let _13: [u32; 8];
let _14: bool;
let _15: u128;
let _16: ([bool; 6],);
let _17: bool;
let _18: (u128,);
let _19: [i128; 3];
let _20: (u128,);
let _21: [i128; 3];
let _22: u64;
let _23: [i128; 3];
let _24: ([bool; 6],);
let _25: [u32; 1];
let _26: ([bool; 6],);
let _27: (*mut *const i128, (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8)), *mut *const i128, i8);
let _28: *mut isize;
let _29: [i128; 3];
let _30: Adt49;
let _31: u32;
let _32: (*mut *const i128, (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8)), *mut *const i128, i8);
let _33: Adt49;
let _34: Adt57;
let _35: bool;
let _36: ();
let _37: ();
{
_1 = (_3.0, _3.1, _5);
Goto(bb1)
}
bb1 = {
_3 = _6;
_1.0 = [_3.1];
_3 = _7;
RET = true;
_1.0 = _6.0;
_1.0 = _7.0;
_9.1 = _6.2 ^ _6.2;
// AB
_7.2 = _5 * _3.2;
Goto(bb2)
}
bb2 = {
_6.0 = _3.0;
_7.0 = [_6.1];
_9 = (97994727048049401489739998723875258884_u128, _3.2);
_10 = [(*_2),(*_2),(*_2)];
_6.2 = _1.2;
_3.1 = _7.1 * _6.1;
_6 = (_7.0, _7.1, _7.2);
_7.0 = [_7.1];
_1.2 = _7.2 >> _7.1;
_9.1 = 15708501360848342251_usize as u8;
_6.0 = _1.0;
_7.0 = [_6.1];
_7.0 = [_3.1];
_9.0 = 261256631850469980701994569656674459748_u128 >> _6.1;
Goto(bb3)
}
bb3 = {
_7.1 = !_1.1;
_3.1 = !_7.1;
Goto(bb4)
}
bb4 = {
_14 = !RET;
_12.fld0 = [(-115029976568926920200430181327619825045_i128),(-130784223541897636873219751864339831235_i128),(-128606549451075746488593647077244342424_i128),(-89802986318784366579942244525781033558_i128),55728318364685533198122379823981631776_i128];
Goto(bb5)
}
bb5 = {
_11 = _4 as isize;
Call(_4 = fn3(_7.1, _9, _1.1, _7, _7.2, _1.2, _6.1, _7, _9.0, _6, _7.1, _7.1, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
(*_2) = (-16178_i16) as isize;
Call(_5 = core::intrinsics::transmute(_1.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = _1.2 <= _7.2;
(*_2) = -_11;
_17 = !_14;
_3.0 = _7.0;
_15 = (-19234_i16) as u128;
_18 = (_9.0,);
_2 = core::ptr::addr_of_mut!((*_2));
_15 = _18.0;
_18 = (_15,);
_6.0 = _7.0;
(*_2) = _11;
_2 = core::ptr::addr_of_mut!((*_2));
_6 = (_3.0, _1.1, _7.2);
_18 = (_15,);
_9 = (_18.0, _3.2);
_21 = [(-92825279561992178995610821699436600433_i128),(-9919174091891249837483145625363112487_i128),(-135965004355609607002256868169241231108_i128)];
_14 = _17 | _17;
_7.0 = [_7.1];
_5 = _1.2;
_20 = _18;
_11 = (*_2) << _5;
_17 = _11 != _11;
_18.0 = _9.0;
_18.0 = _20.0;
_20 = (_9.0,);
_19 = [(-48257765854502478534606480884050109140_i128),(-157588713419553878024441413373726592546_i128),(-105755080923828088764987246456028933035_i128)];
Call(_16.0 = fn10(_3.0, _17, _3, _14, _11, _9, _6, _14, _18, _14), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_2) = -_11;
_20 = (_15,);
_1 = (_6.0, _3.1, _5);
_22 = 17346563026497918716_u64;
_22 = !6643155675622954406_u64;
_9.1 = '\u{c420f}' as u8;
_5 = !_1.2;
_17 = _14;
_10 = [(*_2),_11,(*_2)];
Call(_24.0 = fn11(_1.1, _20.0, _2, _17, _11, _6, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16 = _24;
_24.0 = [_17,_14,_14,_17,_17,_17];
_13 = [_6.1,_7.1,_6.1,_1.1,_3.1,_7.1,_7.1,_6.1];
_7 = (_1.0, _1.1, _1.2);
_18 = (_15,);
_23 = [25521567139636301312608273669344217075_i128,(-124359111430322573161260459982283859297_i128),(-114022676919385362043690398447537868142_i128)];
_7.0 = _6.0;
_1 = (_7.0, _7.1, _6.2);
Call(_23 = fn12(_16, _7.0, _17, _7.2, _16.0, (*_2), _5, _15, _24, _7.1, _14, _2, _3.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1.1 = !_7.1;
_27.1.0 = _7.1;
_1 = _3;
_7 = _1;
_26 = (_24.0,);
_22 = 15861138587441539198_u64 >> _6.1;
_27.1.2.2 = _5;
_26.0 = [_14,_17,_17,_14,_17,_14];
(*_2) = _11 << _11;
_19 = _23;
_21 = [93603692867248421599776890838110329765_i128,61279391600647177217798242182450167184_i128,2227256724166441351989600439069501612_i128];
Goto(bb11)
}
bb11 = {
_32.1.2.2 = _27.1.2.2;
_27.1.1 = (_1.0, _6.1, _5);
_32.1.2.1 = !_27.1.0;
Call(_27.1.1.1 = fn15(_3, _6, _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_30.fld0 = (*_2);
Goto(bb13)
}
bb13 = {
_5 = _32.1.2.2 & _32.1.2.2;
_32.1.2.0 = _7.0;
_27.1 = (_32.1.2.1, _7, _6);
_9 = (_18.0, _5);
_9.0 = _15 + _20.0;
_16.0 = [_14,_14,_14,_14,_17,_14];
_32.1.2.1 = _3.1;
Call(_32.1.2 = fn16(_2, _27.1.2.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7.0 = _1.0;
_32.1.2.2 = _9.1;
RET = !_14;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(2_usize, 15_usize, Move(_15), 5_usize, Move(_5), 17_usize, Move(_17), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(2_usize, 11_usize, Move(_11), 24_usize, Move(_24), 1_usize, Move(_1), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(2_usize, 22_usize, Move(_22), 20_usize, Move(_20), 9_usize, Move(_9), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u32,mut _2: (u128, u8),mut _3: u32,mut _4: ([u32; 1], u32, u8),mut _5: u8,mut _6: u8,mut _7: u32,mut _8: ([u32; 1], u32, u8),mut _9: u128,mut _10: ([u32; 1], u32, u8),mut _11: u32,mut _12: u32,mut _13: ([u32; 1], u32, u8)) -> i32 {
mir! {
type RET = i32;
let _14: f32;
let _15: (*const i128,);
let _16: char;
let _17: char;
let _18: ([u32; 1], u32, u8);
let _19: char;
let _20: isize;
let _21: [u32; 8];
let _22: (u16,);
let _23: [isize; 3];
let _24: Adt47;
let _25: f32;
let _26: (i32, *mut bool);
let _27: [u16; 5];
let _28: *mut bool;
let _29: *mut isize;
let _30: Adt51;
let _31: f64;
let _32: Adt49;
let _33: Adt53;
let _34: (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8));
let _35: u128;
let _36: f32;
let _37: [u32; 8];
let _38: Adt55;
let _39: bool;
let _40: [char; 4];
let _41: ();
let _42: ();
{
_9 = _2.0;
_4.2 = 17802531650253905073445749146020411186_i128 as u8;
_3 = _8.1;
_10.0 = [_12];
RET = (-39_i8) as i32;
_13 = (_10.0, _7, _6);
_4.2 = _12 as u8;
_11 = 31501_u16 as u32;
_7 = _8.2 as u32;
RET = 1374114334_i32;
_4.0 = [_10.1];
_13.2 = _4.2 ^ _6;
_4.0 = [_7];
_5 = _4.2 - _6;
_13.2 = _8.2 * _5;
_10 = (_8.0, _12, _4.2);
_1 = _3 >> _5;
_5 = 82_i8 as u8;
_13 = (_8.0, _1, _10.2);
_4.1 = _1 >> _1;
_10.2 = (-22802499329187445995664948601679556705_i128) as u8;
Call(_13.2 = core::intrinsics::transmute(_4.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18.0 = [_4.1];
_21 = [_1,_3,_4.1,_4.1,_12,_12,_10.1,_4.1];
_10.2 = 43416971638836367337726232282897502314_i128 as u8;
_9 = _2.0;
_21 = [_13.1,_10.1,_10.1,_3,_4.1,_7,_4.1,_1];
_9 = _2.0;
_6 = RET as u8;
_4.2 = _8.2;
_13 = _10;
_20 = 5_isize | 9223372036854775807_isize;
_4.2 = !_8.2;
_8.1 = !_10.1;
Call(_7 = fn4(_4, _21, _1, _18.0, _8.0, _18.0, _4, _4.1, _4, _21, _4.1, _4.1, _18.0, _21), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _8;
_22 = (22976_u16,);
_18.0 = [_4.1];
_8 = (_10.0, _1, _4.2);
_13.2 = _8.2;
_4.1 = !_7;
_13.0 = [_13.1];
_22.0 = 10018_u16 - 53578_u16;
_2.0 = !_9;
_21 = [_4.1,_13.1,_8.1,_8.1,_4.1,_1,_10.1,_8.1];
_3 = _8.1 << _12;
_10.1 = !_7;
_10.0 = [_8.1];
_13 = _4;
_4.1 = '\u{2c27b}' as u32;
_7 = _2.1 as u32;
Call(_6 = core::intrinsics::bswap(_4.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13.1 = (-147533165815880972532511893283683079633_i128) as u32;
_18.0 = [_3];
_8.0 = _4.0;
_5 = _13.2 + _8.2;
_22 = (27116_u16,);
_14 = RET as f32;
_10.1 = _3;
_5 = _13.2;
_21 = [_8.1,_10.1,_3,_10.1,_12,_8.1,_1,_1];
_16 = '\u{933a9}';
_24 = Adt47::Variant0 { fld0: _22 };
_10.2 = _5 * _13.2;
Goto(bb4)
}
bb4 = {
_12 = _10.1 & _3;
_4.1 = _10.1;
match RET {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
1374114334 => bb12,
_ => bb11
}
}
bb5 = {
_13.1 = (-147533165815880972532511893283683079633_i128) as u32;
_18.0 = [_3];
_8.0 = _4.0;
_5 = _13.2 + _8.2;
_22 = (27116_u16,);
_14 = RET as f32;
_10.1 = _3;
_5 = _13.2;
_21 = [_8.1,_10.1,_3,_10.1,_12,_8.1,_1,_1];
_16 = '\u{933a9}';
_24 = Adt47::Variant0 { fld0: _22 };
_10.2 = _5 * _13.2;
Goto(bb4)
}
bb6 = {
_4 = _8;
_22 = (22976_u16,);
_18.0 = [_4.1];
_8 = (_10.0, _1, _4.2);
_13.2 = _8.2;
_4.1 = !_7;
_13.0 = [_13.1];
_22.0 = 10018_u16 - 53578_u16;
_2.0 = !_9;
_21 = [_4.1,_13.1,_8.1,_8.1,_4.1,_1,_10.1,_8.1];
_3 = _8.1 << _12;
_10.1 = !_7;
_10.0 = [_8.1];
_13 = _4;
_4.1 = '\u{2c27b}' as u32;
_7 = _2.1 as u32;
Call(_6 = core::intrinsics::bswap(_4.2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_18.0 = [_4.1];
_21 = [_1,_3,_4.1,_4.1,_12,_12,_10.1,_4.1];
_10.2 = 43416971638836367337726232282897502314_i128 as u8;
_9 = _2.0;
_21 = [_13.1,_10.1,_10.1,_3,_4.1,_7,_4.1,_1];
_9 = _2.0;
_6 = RET as u8;
_4.2 = _8.2;
_13 = _10;
_20 = 5_isize | 9223372036854775807_isize;
_4.2 = !_8.2;
_8.1 = !_10.1;
Call(_7 = fn4(_4, _21, _1, _18.0, _8.0, _18.0, _4, _4.1, _4, _21, _4.1, _4.1, _18.0, _21), ReturnTo(bb2), UnwindUnreachable())
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
_14 = 30117_i16 as f32;
_8.1 = _16 as u32;
_18.2 = _20 as u8;
_25 = _14 * _14;
SetDiscriminant(_24, 2);
_16 = '\u{9743e}';
place!(Field::<([u32; 1], u32, u8)>(Variant(_24, 2), 0)).2 = _8.2 & _8.2;
_5 = Field::<([u32; 1], u32, u8)>(Variant(_24, 2), 0).2 + Field::<([u32; 1], u32, u8)>(Variant(_24, 2), 0).2;
_26.0 = RET ^ RET;
_6 = !_4.2;
match RET {
0 => bb11,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1374114334 => bb13,
_ => bb9
}
}
bb13 = {
_13.0 = [_12];
_18 = (_13.0, _10.1, _6);
_11 = _12;
_1 = _10.1 * _10.1;
place!(Field::<([u32; 1], u32, u8)>(Variant(_24, 2), 0)).1 = 9029262055810103680_i64 as u32;
_2.0 = _9;
_1 = !_18.1;
RET = !_26.0;
_23 = [_20,_20,_20];
_10.1 = !_1;
_13 = _10;
_18.0 = [_10.1];
place!(Field::<f64>(Variant(_24, 2), 3)) = 11189617354932511828_usize as f64;
_13.2 = !_18.2;
_18 = _4;
_27 = [_22.0,_22.0,_22.0,_22.0,_22.0];
_6 = !_18.2;
_10.2 = _5 + _5;
_34.2.2 = _14 as u8;
_34.2.2 = !_4.2;
_13 = (_10.0, _18.1, _34.2.2);
_32 = Adt49 { fld0: _20 };
_4 = _8;
Goto(bb14)
}
bb14 = {
_34.2.0 = [_1];
_21 = [_1,_3,_11,_11,_1,_1,_3,_11];
_16 = '\u{17cca}';
_23 = [_32.fld0,_32.fld0,_20];
_34.2 = _18;
_22.0 = 63010_u16 & 62813_u16;
_22 = (33273_u16,);
_13 = (_18.0, _12, _10.2);
_36 = _25 - _25;
_35 = !_2.0;
_2.1 = false as u8;
place!(Field::<[u32; 1]>(Variant(_24, 2), 1)) = _4.0;
_19 = _16;
_10 = _8;
_33.fld0 = [150678916048732714502077481999169096184_i128,164699261654572859523936571022260844248_i128,(-78273243917457118008475611820754846965_i128),52415734445066200721603059430819680474_i128,67490720863914954521390775981593069403_i128];
_32.fld0 = _20;
_2.0 = _14 as u128;
_28 = core::ptr::addr_of_mut!(_39);
_18.0 = [_12];
_34.1 = (Field::<[u32; 1]>(Variant(_24, 2), 1), _34.2.1, _13.2);
place!(Field::<[u32; 1]>(Variant(_24, 2), 1)) = [_34.1.1];
_24 = Adt47::Variant0 { fld0: _22 };
_18 = _34.1;
_29 = core::ptr::addr_of_mut!(_32.fld0);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(3_usize, 8_usize, Move(_8), 16_usize, Move(_16), 4_usize, Move(_4), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(3_usize, 11_usize, Move(_11), 22_usize, Move(_22), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(3_usize, 27_usize, Move(_27), 1_usize, Move(_1), 3_usize, Move(_3), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: ([u32; 1], u32, u8),mut _2: [u32; 8],mut _3: u32,mut _4: [u32; 1],mut _5: [u32; 1],mut _6: [u32; 1],mut _7: ([u32; 1], u32, u8),mut _8: u32,mut _9: ([u32; 1], u32, u8),mut _10: [u32; 8],mut _11: u32,mut _12: u32,mut _13: [u32; 1],mut _14: [u32; 8]) -> u32 {
mir! {
type RET = u32;
let _15: Adt46;
let _16: bool;
let _17: isize;
let _18: f32;
let _19: f32;
let _20: &'static f64;
let _21: f32;
let _22: [i128; 5];
let _23: u8;
let _24: Adt58;
let _25: u128;
let _26: u8;
let _27: *mut *mut *const i128;
let _28: Adt49;
let _29: bool;
let _30: *mut *const i128;
let _31: (i32, *mut bool);
let _32: u8;
let _33: f32;
let _34: ();
let _35: ();
{
_9.1 = (-130138433676810260957139309995216521800_i128) as u32;
_4 = [_12];
_1.1 = !_8;
_9.1 = _8 - _8;
_14 = [_7.1,_7.1,_11,_12,_1.1,_11,_12,_8];
Goto(bb1)
}
bb1 = {
_7.0 = [_1.1];
_17 = 9223372036854775807_isize | 7_isize;
_9 = (_6, _12, _7.2);
_8 = 66703628967571857310560906491163430192_u128 as u32;
_12 = _9.1;
Goto(bb2)
}
bb2 = {
_2 = [_7.1,_12,_8,_3,_12,_7.1,_7.1,_1.1];
_9 = _7;
_12 = _11 << _3;
_13 = [_3];
_21 = 11623546699115956933_u64 as f32;
_16 = !false;
_11 = 1_usize as u32;
_7.2 = '\u{44a28}' as u8;
_7 = _9;
_10 = [_12,_9.1,_12,_12,_7.1,_12,_12,_12];
_12 = !_1.1;
_9.2 = _1.2 | _1.2;
_7 = _9;
_7.1 = 5167961703093691690_usize as u32;
_1 = _7;
_8 = _9.1 << _7.2;
_19 = 16128_u16 as f32;
_21 = -_19;
_7.0 = [_9.1];
_11 = _8 & _12;
RET = 21726_i16 as u32;
_2 = _10;
_1 = (_6, _8, _7.2);
Call(_22 = fn5(_7, _4, _9.1, _12, _1, _1, _6, _12, _6, _9, _12, _4, _1, _10, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.0 = [_8];
_19 = (-26126_i16) as f32;
Call(_5 = fn8(_14, _9.1, _7, _12, _6, _8, _9.1, _7, _2, _9.1, _4, _4, _9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10 = [_8,_11,_12,_12,_11,_3,_3,_8];
_10 = [_3,_9.1,_12,_1.1,_9.1,_3,_8,_1.1];
_18 = 14121_i16 as f32;
_22 = [(-19096567812419351056743849420979030439_i128),(-115484966940890361791946427372921090451_i128),(-109515250907572771100650302923973754357_i128),94807829254718691029981529514674781630_i128,6794613757360386743352678986447861687_i128];
_7.0 = [_8];
_14 = [_11,_11,_12,_8,_1.1,_9.1,_12,_11];
_5 = [_8];
_26 = _9.2 + _1.2;
_9 = _7;
_18 = -_19;
_1.2 = _17 as u8;
_28.fld0 = _17;
_1.1 = _3 - _8;
_9.0 = [_1.1];
_9 = _7;
Call(_7.1 = fn9(_11, _5, _1.1, _9, _7.0, _1.0, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = !false;
_1.1 = !_3;
_26 = _7.2;
_9.0 = [_11];
_7.2 = _3 as u8;
Goto(bb6)
}
bb6 = {
_29 = _16 ^ _16;
_12 = _8;
_7 = _9;
_23 = _26;
_16 = _8 < _11;
_4 = _9.0;
_9.2 = _26 << _12;
_27 = core::ptr::addr_of_mut!(_30);
_13 = [_11];
_26 = !_23;
_22 = [(-91438909205900011947159180051414903694_i128),29861738829447544246355724893074773004_i128,(-73688286870839293381098036027552113596_i128),(-124938419878951327808417260638721882537_i128),103032402679568013846678685848351638291_i128];
_18 = -_19;
_26 = _23 + _7.2;
_21 = _19 + _18;
_17 = -_28.fld0;
_22 = [39865077387667809403853641092236899535_i128,(-128000963649441453332124660927216480729_i128),(-51190264239935167459033197289129616462_i128),(-129397845039837586096423489677741035722_i128),(-140883158245640361969527832994127581637_i128)];
_12 = _1.1;
_8 = 1093927688_i32 as u32;
_18 = 12999348324252806021_u64 as f32;
_13 = [_11];
_14 = [_3,_12,_12,_12,_1.1,_11,_12,_11];
_1 = _9;
_1.1 = _11 * _3;
_18 = _21 * _21;
_31.1 = core::ptr::addr_of_mut!(_29);
Goto(bb7)
}
bb7 = {
_10 = [_1.1,_12,_3,_11,_1.1,_3,_3,_11];
_11 = !_12;
_7.2 = _1.2 << _1.2;
_9.0 = [_1.1];
_4 = _13;
_21 = _18 - _18;
_1.0 = [_12];
_10 = [_1.1,_1.1,_1.1,_1.1,_12,_12,_3,_11];
_11 = !_3;
RET = _3;
_27 = core::ptr::addr_of_mut!((*_27));
_9 = (_1.0, _1.1, _7.2);
Goto(bb8)
}
bb8 = {
Call(_34 = dump_var(4_usize, 13_usize, Move(_13), 16_usize, Move(_16), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_34 = dump_var(4_usize, 23_usize, Move(_23), 26_usize, Move(_26), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_34 = dump_var(4_usize, 3_usize, Move(_3), 4_usize, Move(_4), 35_usize, _35, 35_usize, _35), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: ([u32; 1], u32, u8),mut _2: [u32; 1],mut _3: u32,mut _4: u32,mut _5: ([u32; 1], u32, u8),mut _6: ([u32; 1], u32, u8),mut _7: [u32; 1],mut _8: u32,mut _9: [u32; 1],mut _10: ([u32; 1], u32, u8),mut _11: u32,mut _12: [u32; 1],mut _13: ([u32; 1], u32, u8),mut _14: [u32; 8],mut _15: [u32; 1]) -> [i128; 5] {
mir! {
type RET = [i128; 5];
let _16: *mut *const i128;
let _17: Adt48;
let _18: i16;
let _19: isize;
let _20: ([u32; 1], u32, u8);
let _21: u32;
let _22: [i128; 5];
let _23: u16;
let _24: Adt49;
let _25: f32;
let _26: isize;
let _27: Adt51;
let _28: isize;
let _29: bool;
let _30: char;
let _31: ([u32; 1], u32, u8);
let _32: isize;
let _33: f64;
let _34: Adt46;
let _35: [u16; 5];
let _36: (i16,);
let _37: isize;
let _38: bool;
let _39: [i128; 3];
let _40: Adt53;
let _41: ();
let _42: ();
{
_12 = _2;
_10.0 = _15;
_8 = _13.1 | _5.1;
_5 = (_9, _6.1, _1.2);
_13 = (_6.0, _6.1, _1.2);
_14 = [_8,_11,_10.1,_6.1,_11,_5.1,_3,_4];
_13.0 = [_10.1];
_1 = (_2, _5.1, _6.2);
_13 = (_15, _4, _6.2);
_1.1 = !_8;
_3 = _10.1 - _11;
RET = [86615879178555051760742129953894885987_i128,125563340411271947959205309397991399488_i128,(-22047667110846955348588653413849960518_i128),(-115066914787863800649044795815785334467_i128),75395028579181147946390972672444912332_i128];
_6 = (_2, _3, _13.2);
_10.0 = [_13.1];
Goto(bb1)
}
bb1 = {
_6.2 = _1.2;
_13.1 = _5.1 >> _1.1;
_5 = (_13.0, _10.1, _13.2);
_6.2 = !_13.2;
_6 = _10;
_1 = _5;
Call(_6.2 = fn6(_13.0, _10.1, _14, _10.0, _10.0, _10.0, _6.1, _15, _1.1, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.1 = !_5.1;
_1 = (_5.0, _4, _6.2);
RET = [(-105327639048032927989911470521810732833_i128),77436370302205846583207089995188708274_i128,22426647173351299826149038666266903326_i128,(-26079300625973257185615765951044803375_i128),128063259454572908873657543761018395435_i128];
_13.2 = _6.2;
_4 = !_3;
_6.2 = _1.2 << _11;
_4 = 1413295416996876282_u64 as u32;
_6 = (_12, _1.1, _13.2);
_18 = 11525_i16 * (-18236_i16);
_10.2 = 9223372036854775807_isize as u8;
RET = [120396817838404231102907720646063418386_i128,79600549464035268301973163605385469037_i128,148938196660147936979861720009434155634_i128,95323069927739481243287044078442951579_i128,(-41803003034099782378895866510493931819_i128)];
_6 = (_9, _10.1, _1.2);
Call(_5.0 = fn7(_7, _13, _14, _1.1, _11, _13.2, _7, _10, _14, _6.2, _12, _1.0, _6.2, _2, _8, _13.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = (_6.0, _8, _5.2);
_23 = true as u16;
_14 = [_10.1,_8,_6.1,_13.1,_1.1,_13.1,_10.1,_1.1];
_10.1 = _11;
_6.0 = [_10.1];
_6.0 = [_5.1];
_20 = _13;
Call(_26 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _11;
_5.2 = !_20.2;
_10 = (_5.0, _8, _20.2);
_5 = (_2, _11, _1.2);
_11 = !_10.1;
_21 = _5.1;
_20 = (_1.0, _4, _10.2);
RET = [(-88159968120317851899793685631269115496_i128),(-4752065920125450996339640187936034083_i128),71302051679621249567141080979015512670_i128,119174267593379844615241218381323235455_i128,78324981051903636287342052337880926490_i128];
_22 = RET;
_13.2 = !_5.2;
_25 = 297899578_i32 as f32;
_19 = _25 as isize;
_22 = [15266060674488668100620573921170480856_i128,(-158300967666468414896327816337949382613_i128),(-128758198968638531247707197154721048859_i128),74423793721724248222169053938446295350_i128,(-69091779061803356284582575017745740790_i128)];
_24.fld0 = !_19;
_20.2 = _10.2;
_1.0 = [_10.1];
_29 = !true;
_13.2 = !_1.2;
_5.0 = [_3];
_21 = _11 - _11;
_1.0 = [_21];
RET = _22;
_13.0 = [_13.1];
Goto(bb5)
}
bb5 = {
_22 = RET;
_1 = (_13.0, _20.1, _13.2);
_6.0 = [_20.1];
_5 = _10;
_15 = _5.0;
_1 = (_9, _21, _10.2);
_21 = '\u{56b10}' as u32;
_18 = (-7395_i16);
_6.2 = !_13.2;
_31.0 = [_5.1];
_3 = _20.2 as u32;
_31.1 = 7394531232747778241_u64 as u32;
_1.0 = [_20.1];
_31.1 = (-35340897385615762615926291986881242845_i128) as u32;
_13.2 = 13654349774706184936_u64 as u8;
_6 = (_31.0, _1.1, _1.2);
_13.2 = (-95_i8) as u8;
match _18 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463463374607431768204061 => bb14,
_ => bb13
}
}
bb6 = {
_4 = _11;
_5.2 = !_20.2;
_10 = (_5.0, _8, _20.2);
_5 = (_2, _11, _1.2);
_11 = !_10.1;
_21 = _5.1;
_20 = (_1.0, _4, _10.2);
RET = [(-88159968120317851899793685631269115496_i128),(-4752065920125450996339640187936034083_i128),71302051679621249567141080979015512670_i128,119174267593379844615241218381323235455_i128,78324981051903636287342052337880926490_i128];
_22 = RET;
_13.2 = !_5.2;
_25 = 297899578_i32 as f32;
_19 = _25 as isize;
_22 = [15266060674488668100620573921170480856_i128,(-158300967666468414896327816337949382613_i128),(-128758198968638531247707197154721048859_i128),74423793721724248222169053938446295350_i128,(-69091779061803356284582575017745740790_i128)];
_24.fld0 = !_19;
_20.2 = _10.2;
_1.0 = [_10.1];
_29 = !true;
_13.2 = !_1.2;
_5.0 = [_3];
_21 = _11 - _11;
_1.0 = [_21];
RET = _22;
_13.0 = [_13.1];
Goto(bb5)
}
bb7 = {
_10 = (_6.0, _8, _5.2);
_23 = true as u16;
_14 = [_10.1,_8,_6.1,_13.1,_1.1,_13.1,_10.1,_1.1];
_10.1 = _11;
_6.0 = [_10.1];
_6.0 = [_5.1];
_20 = _13;
Call(_26 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1.1 = !_5.1;
_1 = (_5.0, _4, _6.2);
RET = [(-105327639048032927989911470521810732833_i128),77436370302205846583207089995188708274_i128,22426647173351299826149038666266903326_i128,(-26079300625973257185615765951044803375_i128),128063259454572908873657543761018395435_i128];
_13.2 = _6.2;
_4 = !_3;
_6.2 = _1.2 << _11;
_4 = 1413295416996876282_u64 as u32;
_6 = (_12, _1.1, _13.2);
_18 = 11525_i16 * (-18236_i16);
_10.2 = 9223372036854775807_isize as u8;
RET = [120396817838404231102907720646063418386_i128,79600549464035268301973163605385469037_i128,148938196660147936979861720009434155634_i128,95323069927739481243287044078442951579_i128,(-41803003034099782378895866510493931819_i128)];
_6 = (_9, _10.1, _1.2);
Call(_5.0 = fn7(_7, _13, _14, _1.1, _11, _13.2, _7, _10, _14, _6.2, _12, _1.0, _6.2, _2, _8, _13.2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_6.2 = _1.2;
_13.1 = _5.1 >> _1.1;
_5 = (_13.0, _10.1, _13.2);
_6.2 = !_13.2;
_6 = _10;
_1 = _5;
Call(_6.2 = fn6(_13.0, _10.1, _14, _10.0, _10.0, _10.0, _6.1, _15, _1.1, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_18 = -(-2363_i16);
_11 = !_1.1;
_5.0 = [_1.1];
_13.2 = !_5.2;
_1.0 = [_8];
_31 = (_15, _10.1, _10.2);
_33 = 327402818900781130209665162888629502227_u128 as f64;
_31.0 = [_4];
_6.2 = !_31.2;
_13.2 = !_1.2;
_3 = _11 & _8;
_23 = _24.fld0 as u16;
_18 = -456_i16;
_28 = _19 << _6.2;
_19 = _28 * _28;
_10.0 = _6.0;
_9 = _12;
_13.2 = _6.2 - _31.2;
_7 = _9;
_1.2 = !_5.2;
_23 = _5.2 as u16;
_10.0 = [_20.1];
_24.fld0 = -_28;
_20.1 = _11 << _1.1;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(5_usize, 14_usize, Move(_14), 26_usize, Move(_26), 20_usize, Move(_20), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(5_usize, 4_usize, Move(_4), 23_usize, Move(_23), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(5_usize, 15_usize, Move(_15), 11_usize, Move(_11), 29_usize, Move(_29), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [u32; 1],mut _2: u32,mut _3: [u32; 8],mut _4: [u32; 1],mut _5: [u32; 1],mut _6: [u32; 1],mut _7: u32,mut _8: [u32; 1],mut _9: u32,mut _10: [u32; 1]) -> u8 {
mir! {
type RET = u8;
let _11: (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8));
let _12: u8;
let _13: isize;
let _14: usize;
let _15: isize;
let _16: *const i128;
let _17: isize;
let _18: isize;
let _19: bool;
let _20: isize;
let _21: f64;
let _22: Adt55;
let _23: f32;
let _24: ();
let _25: ();
{
_9 = _2;
_9 = 15333409702853259527_u64 as u32;
_3 = [_2,_7,_7,_2,_2,_2,_7,_7];
_11.2 = (_1, _2, 95_u8);
_11.0 = _2 + _11.2.1;
_11.2.2 = 178_u8 * 182_u8;
_1 = [_2];
_11.0 = _11.2.1 >> _11.2.1;
_11.2.2 = 43_u8 << _11.2.1;
_11.1.2 = (-146292959286310019983049142792099518844_i128) as u8;
_8 = _10;
_11.2.1 = _2;
_3 = [_11.0,_11.0,_11.2.1,_11.2.1,_11.2.1,_2,_11.0,_7];
_8 = [_11.2.1];
_11.1.2 = !_11.2.2;
_11.1.1 = _2 ^ _11.2.1;
_11.1.2 = _11.2.2;
_3 = [_11.2.1,_2,_11.0,_2,_11.2.1,_2,_11.0,_11.0];
_1 = [_11.0];
_4 = [_11.2.1];
_11.2.1 = _2;
_13 = false as isize;
RET = (-1079706821_i32) as u8;
Goto(bb1)
}
bb1 = {
_3 = [_11.1.1,_11.2.1,_11.1.1,_11.1.1,_11.0,_11.1.1,_11.2.1,_11.2.1];
_11.1 = _11.2;
_14 = 5_usize >> _11.1.2;
_13 = false as isize;
_15 = _13;
_11.0 = _7;
_3 = [_11.1.1,_2,_7,_2,_11.1.1,_2,_11.0,_7];
_8 = _11.1.0;
Goto(bb2)
}
bb2 = {
_14 = !5_usize;
_15 = _13 >> _11.1.1;
_11.2 = (_8, _11.0, _11.1.2);
_11.1.1 = _14 as u32;
_18 = _15 - _15;
_15 = 73157829946619040530634864126418840082_i128 as isize;
_9 = (-1923495625_i32) as u32;
_11.2.1 = _2 - _7;
_11.1.0 = [_11.2.1];
_11.1.1 = _11.2.1 | _7;
_12 = 1461110064_i32 as u8;
_11.2.1 = _11.2.2 as u32;
_19 = _11.0 == _11.0;
_11.2.1 = _2 | _11.0;
_11.2.0 = _8;
Call(_14 = core::intrinsics::bswap(5_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_18 = _13;
RET = _19 as u8;
_11.2.1 = _11.1.1;
_11.0 = _2 << _2;
_8 = [_2];
_3 = [_11.0,_11.1.1,_11.0,_2,_7,_7,_11.2.1,_11.0];
_20 = -_13;
_18 = _13;
_11.2.2 = RET ^ RET;
_11.2 = (_5, _11.0, RET);
_4 = [_11.0];
_17 = !_15;
_11.1 = (_5, _7, RET);
_11.2.1 = !_2;
_19 = !true;
_8 = [_11.0];
_17 = !_20;
_11.1.0 = _8;
_18 = _13;
_11.0 = !_11.2.1;
_12 = _11.1.2 * _11.1.2;
_12 = RET;
_11.0 = RET as u32;
_4 = [_2];
_15 = _17 >> _11.0;
_11.2.1 = _11.0;
_12 = !RET;
_3 = [_7,_7,_7,_11.0,_11.0,_2,_11.0,_7];
Goto(bb4)
}
bb4 = {
Call(_24 = dump_var(6_usize, 12_usize, Move(_12), 9_usize, Move(_9), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_24 = dump_var(6_usize, 5_usize, Move(_5), 4_usize, Move(_4), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_24 = dump_var(6_usize, 10_usize, Move(_10), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [u32; 1],mut _2: ([u32; 1], u32, u8),mut _3: [u32; 8],mut _4: u32,mut _5: u32,mut _6: u8,mut _7: [u32; 1],mut _8: ([u32; 1], u32, u8),mut _9: [u32; 8],mut _10: u8,mut _11: [u32; 1],mut _12: [u32; 1],mut _13: u8,mut _14: [u32; 1],mut _15: u32,mut _16: u8) -> [u32; 1] {
mir! {
type RET = [u32; 1];
let _17: i16;
let _18: [char; 4];
let _19: f32;
let _20: ();
let _21: ();
{
RET = [_2.1];
_2.2 = !_16;
_13 = _6;
_15 = _8.1 >> _6;
_2 = (_12, _4, _10);
_9 = [_2.1,_8.1,_15,_15,_4,_15,_8.1,_2.1];
_8.0 = _14;
_7 = [_8.1];
_15 = _2.1;
_10 = _13;
_2.2 = _10 ^ _6;
RET = [_8.1];
_12 = [_5];
_18 = ['\u{8c5d1}','\u{76de3}','\u{be5a7}','\u{d8355}'];
_2.1 = _5;
_11 = _7;
_14 = _2.0;
_19 = (-11233_i16) as f32;
_17 = (-5161_i16) & (-28772_i16);
_8 = (_1, _2.1, _13);
_16 = 5657079172345417832_u64 as u8;
_2 = (_14, _15, _8.2);
RET = _8.0;
_18 = ['\u{40b5b}','\u{b090b}','\u{d25d6}','\u{c481e}'];
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(7_usize, 15_usize, Move(_15), 13_usize, Move(_13), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(7_usize, 8_usize, Move(_8), 12_usize, Move(_12), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(7_usize, 4_usize, Move(_4), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [u32; 8],mut _2: u32,mut _3: ([u32; 1], u32, u8),mut _4: u32,mut _5: [u32; 1],mut _6: u32,mut _7: u32,mut _8: ([u32; 1], u32, u8),mut _9: [u32; 8],mut _10: u32,mut _11: [u32; 1],mut _12: [u32; 1],mut _13: ([u32; 1], u32, u8)) -> [u32; 1] {
mir! {
type RET = [u32; 1];
let _14: ();
let _15: ();
{
_13.0 = [_4];
_4 = _2;
_3.1 = _4 * _10;
_12 = [_2];
_8.0 = _5;
_8.2 = 6_i8 as u8;
RET = _5;
_8.0 = [_3.1];
_8 = (_11, _7, _13.2);
RET = [_13.1];
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(8_usize, 7_usize, Move(_7), 13_usize, Move(_13), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(8_usize, 9_usize, Move(_9), 2_usize, Move(_2), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u32,mut _2: [u32; 1],mut _3: u32,mut _4: ([u32; 1], u32, u8),mut _5: [u32; 1],mut _6: [u32; 1],mut _7: [u32; 8]) -> u32 {
mir! {
type RET = u32;
let _8: f64;
let _9: isize;
let _10: Adt42;
let _11: u128;
let _12: ();
let _13: ();
{
_4.1 = (-2141872945_i32) as u32;
_4.2 = 180_u8 - 221_u8;
_4.0 = [_1];
RET = _3;
_4.2 = 235_u8 ^ 181_u8;
_3 = _1 << _1;
RET = !_1;
_4.1 = 30529468849191563164386056450877628449_i128 as u32;
RET = _3 >> _1;
RET = true as u32;
_5 = _2;
_4 = (_6, _1, 168_u8);
_5 = [_3];
_1 = 76_i8 as u32;
_7 = [_4.1,_4.1,_4.1,_3,_3,_3,_3,_3];
_1 = _3 * _3;
_2 = [_1];
_4.0 = _6;
_7 = [_3,_1,_3,_4.1,_1,_4.1,_4.1,_4.1];
Goto(bb1)
}
bb1 = {
_6 = _5;
_2 = _4.0;
RET = _1;
_11 = 299868649012757003278216371138715117890_u128 >> _1;
_3 = _4.1;
_9 = 121_isize;
_8 = _4.1 as f64;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(9_usize, 6_usize, Move(_6), 9_usize, Move(_9), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [u32; 1],mut _2: bool,mut _3: ([u32; 1], u32, u8),mut _4: bool,mut _5: isize,mut _6: (u128, u8),mut _7: ([u32; 1], u32, u8),mut _8: bool,mut _9: (u128,),mut _10: bool) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _11: f64;
let _12: ();
let _13: ();
{
_9 = (_6.0,);
_4 = _2;
_10 = _4;
_8 = !_10;
_6 = (_9.0, _7.2);
_6 = (_9.0, _7.2);
_3.1 = !_7.1;
RET = [_4,_2,_8,_10,_4,_2];
_9.0 = 123964506532504878889275915660275536241_i128 as u128;
RET = [_10,_10,_8,_4,_8,_4];
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(10_usize, 1_usize, Move(_1), 6_usize, Move(_6), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_12 = dump_var(10_usize, 7_usize, Move(_7), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u32,mut _2: u128,mut _3: *mut isize,mut _4: bool,mut _5: isize,mut _6: ([u32; 1], u32, u8),mut _7: [isize; 3]) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _8: ();
let _9: ();
{
_6.2 = (-139025031078302011429166313388497462216_i128) as u8;
_6.2 = 213_u8 >> _2;
_6.0 = [_6.1];
RET = [_4,_4,_4,_4,_4,_4];
RET = [_4,_4,_4,_4,_4,_4];
_1 = _6.1 + _6.1;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(11_usize, 1_usize, Move(_1), 2_usize, Move(_2), 6_usize, Move(_6), 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: ([bool; 6],),mut _2: [u32; 1],mut _3: bool,mut _4: u8,mut _5: [bool; 6],mut _6: isize,mut _7: u8,mut _8: u128,mut _9: ([bool; 6],),mut _10: u32,mut _11: bool,mut _12: *mut isize,mut _13: [u32; 1]) -> [i128; 3] {
mir! {
type RET = [i128; 3];
let _14: *mut isize;
let _15: [i128; 5];
let _16: f32;
let _17: *const i128;
let _18: bool;
let _19: (*const i128,);
let _20: isize;
let _21: (u16,);
let _22: (i16,);
let _23: f64;
let _24: f64;
let _25: [u32; 1];
let _26: ([u32; 1], u32, u8);
let _27: isize;
let _28: isize;
let _29: bool;
let _30: isize;
let _31: [bool; 6];
let _32: [i128; 3];
let _33: bool;
let _34: bool;
let _35: Adt49;
let _36: *mut isize;
let _37: f32;
let _38: f32;
let _39: f64;
let _40: (*const i128,);
let _41: ();
let _42: ();
{
_9.0 = [_11,_11,_3,_3,_11,_11];
Call(_7 = fn13((*_12), _3, _9, _1.0, (*_12), _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !2518595380_u32;
_4 = !_7;
_9.0 = [_3,_11,_11,_3,_3,_3];
RET = [19514957070097512252392428655527283239_i128,168053731932556569492417574310288176434_i128,32384693634836798840019275271571923174_i128];
_4 = !_7;
_8 = (-16266_i16) as u128;
_5 = [_3,_3,_3,_11,_3,_3];
_7 = !_4;
_13 = [_10];
_6 = (*_12);
_6 = (*_12) >> (*_12);
_6 = (*_12) >> (*_12);
_15 = [(-30269524841377228428281768377159604232_i128),(-15181691668027159067169493602997577484_i128),(-72982218306893256038307468118268779232_i128),(-42390088315051387452513781811038666703_i128),(-702029178423627006603356407200477240_i128)];
_13 = _2;
_1 = _9;
_14 = _12;
(*_12) = _6;
(*_12) = _6 - _6;
_13 = [_10];
_16 = (-92_i8) as f32;
_10 = 1247401311_u32;
_3 = !_11;
_20 = -(*_14);
_2 = _13;
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
1247401311 => bb6,
_ => bb5
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
Return()
}
bb6 = {
(*_12) = _6;
_9.0 = _5;
(*_14) = 36794_u16 as isize;
_18 = _11 | _3;
_11 = _20 > _20;
_5 = _1.0;
_18 = !_11;
(*_14) = _20;
_21.0 = 96989332338729075385729514008925574026_i128 as u16;
_20 = _21.0 as isize;
_15 = [99189116326388959855942686081246224273_i128,(-116995364873730866254055648910785970415_i128),(-70479110760786946345706275149029533226_i128),115952905950527376125021288969086691720_i128,131255209483074757753983217836638520683_i128];
_1.0 = [_18,_11,_11,_11,_11,_11];
_13 = [_10];
_12 = core::ptr::addr_of_mut!(_6);
_7 = _4 | _4;
_22.0 = !13156_i16;
_9 = (_1.0,);
_1 = (_9.0,);
RET = [116712106082321817404674574480245460916_i128,45508008215293571257447077587164784174_i128,111365325042919505326032728349640246744_i128];
_23 = (-66_i8) as f64;
(*_14) = (*_12) << _6;
_7 = !_4;
_22 = (3787_i16,);
_1.0 = [_11,_11,_18,_11,_3,_18];
(*_12) = !(*_14);
match _10 {
0 => bb7,
1 => bb8,
1247401311 => bb10,
_ => bb9
}
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
_14 = core::ptr::addr_of_mut!((*_14));
_7 = _4 >> (*_14);
_20 = (*_14);
RET = [82291996429400220302315559414743334261_i128,(-143067900199483172834839522676908956769_i128),8241434944262724313647578601091357191_i128];
_25 = [_10];
_12 = _14;
_16 = (-6064246077558212389_i64) as f32;
_22.0 = (-17657_i16);
_2 = _25;
RET = [154924883657879574504628081351200306361_i128,34770816408414649315381644341704460300_i128,(-74226824012987341134430689030989285584_i128)];
_18 = !_11;
_10 = 85392167_u32 >> _6;
Goto(bb11)
}
bb11 = {
_4 = !_7;
_12 = core::ptr::addr_of_mut!(_27);
_18 = !_3;
match _22.0 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb9,
340282366920938463463374607431768193799 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_9 = (_1.0,);
(*_12) = _6 + _6;
_26.2 = !_7;
(*_12) = (-68_i8) as isize;
_5 = [_11,_3,_11,_11,_18,_11];
_26.0 = [_10];
_28 = _6 * (*_14);
_2 = _26.0;
_14 = core::ptr::addr_of_mut!((*_12));
_13 = [_10];
_4 = !_26.2;
_1 = _9;
_6 = -_20;
_26.1 = _10;
_15 = [(-142757832197215755135613011231047687354_i128),(-164386707661143003449903865638660641390_i128),84804630766446420652000297243876236676_i128,128751339167156475932449389502679506518_i128,(-7983099689749237156284931757417617195_i128)];
(*_14) = '\u{ea099}' as isize;
_21 = (54543_u16,);
_28 = -_6;
_26.1 = _10 - _10;
_15 = [51230696322993315533206698487312772961_i128,13172076979841772673729104323345881847_i128,150967176018320162450917746931504172187_i128,105080848283843586555185067302908086407_i128,(-89033539923218613368141775452473093197_i128)];
_22 = (5416_i16,);
_6 = _20 << _7;
_27 = _7 as isize;
_24 = _23;
(*_12) = _6 & _20;
_22 = ((-26535_i16),);
match _22.0 {
0 => bb1,
1 => bb7,
2 => bb5,
3 => bb14,
340282366920938463463374607431768184921 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_4 = !_7;
_12 = core::ptr::addr_of_mut!(_27);
_18 = !_3;
match _22.0 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb9,
340282366920938463463374607431768193799 => bb13,
_ => bb12
}
}
bb16 = {
_22.0 = 9613_i16;
_23 = _16 as f64;
RET = [(-64905514383644889961797169074555000225_i128),(-113921521003236787298651977400556472428_i128),135263698770446189334001724970062180803_i128];
_26.2 = _4;
_20 = (*_14);
_31 = _5;
_28 = _20;
_13 = [_10];
_30 = 6612359441562549681_u64 as isize;
_26.2 = (-11139341233397679797454218833623524409_i128) as u8;
_5 = [_11,_18,_3,_3,_3,_11];
_37 = -_16;
_1.0 = [_3,_3,_18,_18,_18,_3];
_32 = [47628574784039263472572262421990456507_i128,144243776009381060831745612941611737663_i128,97779463725856761073230568195347163676_i128];
_16 = _37 * _37;
_4 = _7;
_26.0 = [_26.1];
_25 = [_10];
_7 = 6681321041410323317_i64 as u8;
_9.0 = _1.0;
_20 = _28 + (*_12);
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(12_usize, 3_usize, Move(_3), 6_usize, Move(_6), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(12_usize, 21_usize, Move(_21), 5_usize, Move(_5), 1_usize, Move(_1), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(12_usize, 7_usize, Move(_7), 20_usize, Move(_20), 18_usize, Move(_18), 11_usize, Move(_11)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: bool,mut _3: ([bool; 6],),mut _4: [bool; 6],mut _5: isize,mut _6: [u32; 1]) -> u8 {
mir! {
type RET = u8;
let _7: Adt49;
let _8: isize;
let _9: Adt52;
let _10: (u16,);
let _11: f64;
let _12: f64;
let _13: u8;
let _14: char;
let _15: u16;
let _16: f64;
let _17: [i128; 5];
let _18: *mut *const i128;
let _19: i128;
let _20: Adt42;
let _21: f32;
let _22: f64;
let _23: u16;
let _24: f32;
let _25: i128;
let _26: i8;
let _27: u64;
let _28: bool;
let _29: isize;
let _30: (u128,);
let _31: i16;
let _32: *mut bool;
let _33: *mut *mut *const i128;
let _34: [i128; 3];
let _35: f32;
let _36: bool;
let _37: f64;
let _38: ();
let _39: ();
{
_5 = '\u{5bbf6}' as isize;
_4 = [_2,_2,_2,_2,_2,_2];
_5 = 1258573699_u32 as isize;
_4 = _3.0;
RET = 1243906507_i32 as u8;
_3 = (_4,);
_2 = true | true;
_2 = false;
RET = !92_u8;
_6 = [1035586_u32];
_2 = false;
_1 = -_5;
_8 = _5;
_7 = Adt49 { fld0: _1 };
RET = 100_u8;
_7.fld0 = -_8;
_8 = _5;
_4 = _3.0;
_3 = (_4,);
_5 = _1 ^ _8;
_10.0 = '\u{af5bd}' as u16;
_10 = (56321_u16,);
_3 = (_4,);
_12 = 679875180_i32 as f64;
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
56321 => bb7,
_ => bb6
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
RET = 234_u8;
_8 = 70612011098578151_usize as isize;
_5 = _7.fld0;
_1 = 245543775592948634098081495075849839263_u128 as isize;
_1 = _7.fld0 | _8;
_3.0 = [_2,_2,_2,_2,_2,_2];
RET = 4_u8 + 1_u8;
_12 = 1_usize as f64;
RET = _12 as u8;
_5 = _1 >> _7.fld0;
_9 = Adt52::Variant0 { fld0: 3725367532_u32 };
_6 = [3642987203_u32];
_2 = !true;
_10.0 = 28307_u16 - 64727_u16;
_6 = [1902923527_u32];
_13 = !RET;
_7.fld0 = 0_usize as isize;
_1 = -_8;
_6 = [619752128_u32];
_14 = '\u{129e7}';
_14 = '\u{f5802}';
_4 = _3.0;
_2 = !true;
_1 = _5 ^ _8;
_13 = RET << _8;
_14 = '\u{faa2f}';
Call(_12 = fn14(_1, Move(_7), _10, _5, _5, _5, _1, _1, _10, _3, _1, _13, _1, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = _5 & _5;
_8 = _1 << _13;
_6 = [3739272912_u32];
_7.fld0 = _8;
_15 = _10.0;
_12 = 1743272354_i32 as f64;
_4 = [_2,_2,_2,_2,_2,_2];
_7.fld0 = !_8;
_4 = [_2,_2,_2,_2,_2,_2];
_7.fld0 = (-1095776235_i32) as isize;
_12 = _1 as f64;
Goto(bb9)
}
bb9 = {
_10.0 = _15;
_2 = !false;
_11 = _12 * _12;
_16 = _12 + _11;
_15 = 2897903283_u32 as u16;
_1 = !_8;
RET = _13 ^ _13;
_13 = RET;
_7 = Adt49 { fld0: _1 };
_14 = '\u{66ff0}';
_17 = [10260694939025159380392570480936879328_i128,159107259778661632494208680739165788671_i128,(-146858372282105556021299602480989809059_i128),114158535701703191407071213908171703643_i128,(-112405848420802950792715166638803048471_i128)];
_10.0 = _15;
_19 = 30900397_i32 as i128;
_2 = !false;
_21 = _19 as f32;
_9 = Adt52::Variant0 { fld0: 1487904208_u32 };
_3.0 = [_2,_2,_2,_2,_2,_2];
_10.0 = 13423457804317494436_u64 as u16;
_8 = _1;
_22 = _19 as f64;
_21 = _15 as f32;
_7.fld0 = _8 | _8;
_3 = (_4,);
place!(Field::<u32>(Variant(_9, 0), 0)) = _7.fld0 as u32;
_3.0 = [_2,_2,_2,_2,_2,_2];
_13 = RET;
Goto(bb10)
}
bb10 = {
_6 = [Field::<u32>(Variant(_9, 0), 0)];
_8 = !_1;
_5 = _7.fld0;
Goto(bb11)
}
bb11 = {
_27 = 13791655141600166663_u64;
_27 = 16396144023633819565_u64;
_26 = (-81_i8);
_2 = !true;
_15 = RET as u16;
_13 = RET;
_12 = -_16;
_7.fld0 = _1 ^ _5;
_22 = _13 as f64;
_2 = _16 >= _16;
_24 = _21;
SetDiscriminant(_9, 3);
place!(Field::<Adt49>(Variant(_9, 3), 2)).fld0 = _8;
_1 = _8;
_23 = _15;
_5 = (-1972535001_i32) as isize;
_3 = (_4,);
match _26 {
340282366920938463463374607431768211375 => bb13,
_ => bb12
}
}
bb12 = {
_6 = [Field::<u32>(Variant(_9, 0), 0)];
_8 = !_1;
_5 = _7.fld0;
Goto(bb11)
}
bb13 = {
_29 = _22 as isize;
place!(Field::<Adt49>(Variant(_9, 3), 2)) = Move(_7);
place!(Field::<([bool; 6],)>(Variant(_9, 3), 3)).0 = [_2,_2,_2,_2,_2,_2];
place!(Field::<([u32; 1], u32, u8)>(Variant(_9, 3), 0)).2 = RET;
_2 = _16 <= _12;
place!(Field::<Adt49>(Variant(_9, 3), 2)).fld0 = -_5;
_30 = (130142454499512000077467624806825182907_u128,);
place!(Field::<([u32; 1], u32, u8)>(Variant(_9, 3), 0)).1 = !2453318804_u32;
_30 = (264203525769322509251898599715591064313_u128,);
_25 = !_19;
place!(Field::<([u32; 1], u32, u8)>(Variant(_9, 3), 0)).1 = 7203_i16 as u32;
_14 = '\u{24dd3}';
place!(Field::<*const i128>(Variant(_9, 3), 4)) = core::ptr::addr_of!(_25);
_28 = _16 == _12;
_7 = Adt49 { fld0: _8 };
_26 = !(-50_i8);
match _27 {
0 => bb11,
1 => bb7,
2 => bb14,
16396144023633819565 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_10.0 = _15;
_2 = !false;
_11 = _12 * _12;
_16 = _12 + _11;
_15 = 2897903283_u32 as u16;
_1 = !_8;
RET = _13 ^ _13;
_13 = RET;
_7 = Adt49 { fld0: _1 };
_14 = '\u{66ff0}';
_17 = [10260694939025159380392570480936879328_i128,159107259778661632494208680739165788671_i128,(-146858372282105556021299602480989809059_i128),114158535701703191407071213908171703643_i128,(-112405848420802950792715166638803048471_i128)];
_10.0 = _15;
_19 = 30900397_i32 as i128;
_2 = !false;
_21 = _19 as f32;
_9 = Adt52::Variant0 { fld0: 1487904208_u32 };
_3.0 = [_2,_2,_2,_2,_2,_2];
_10.0 = 13423457804317494436_u64 as u16;
_8 = _1;
_22 = _19 as f64;
_21 = _15 as f32;
_7.fld0 = _8 | _8;
_3 = (_4,);
place!(Field::<u32>(Variant(_9, 0), 0)) = _7.fld0 as u32;
_3.0 = [_2,_2,_2,_2,_2,_2];
_13 = RET;
Goto(bb10)
}
bb16 = {
_4 = [_2,_2,_28,_2,_2,_2];
_13 = !Field::<([u32; 1], u32, u8)>(Variant(_9, 3), 0).2;
_5 = _11 as isize;
place!(Field::<i32>(Variant(_9, 3), 5)) = 751220654_i32;
_26 = (-6_i8);
place!(Field::<[bool; 6]>(Variant(_9, 3), 7)) = [_28,_2,_2,_28,_28,_2];
_3 = (Field::<([bool; 6],)>(Variant(_9, 3), 3).0,);
_31 = (-6309_i16);
_6 = [Field::<([u32; 1], u32, u8)>(Variant(_9, 3), 0).1];
place!(Field::<Adt49>(Variant(_9, 3), 2)).fld0 = -_1;
_25 = -_19;
_23 = !_15;
_12 = -_16;
_28 = _2;
_6 = [Field::<([u32; 1], u32, u8)>(Variant(_9, 3), 0).1];
_21 = _27 as f32;
_14 = '\u{e4a7f}';
_5 = -_1;
_36 = _2 & _2;
Goto(bb17)
}
bb17 = {
Call(_38 = dump_var(13_usize, 25_usize, Move(_25), 1_usize, Move(_1), 29_usize, Move(_29), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(13_usize, 28_usize, Move(_28), 5_usize, Move(_5), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(13_usize, 2_usize, Move(_2), 27_usize, Move(_27), 13_usize, Move(_13), 39_usize, _39), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: Adt49,mut _3: (u16,),mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: (u16,),mut _10: ([bool; 6],),mut _11: isize,mut _12: u8,mut _13: isize,mut _14: isize) -> f64 {
mir! {
type RET = f64;
let _15: *mut &'static f64;
let _16: f32;
let _17: char;
let _18: Adt46;
let _19: (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8));
let _20: i128;
let _21: f32;
let _22: Adt42;
let _23: i128;
let _24: i32;
let _25: bool;
let _26: f32;
let _27: u8;
let _28: Adt50;
let _29: ();
let _30: ();
{
_5 = true as isize;
_8 = true as isize;
_4 = _11;
_3.0 = _9.0 * _9.0;
RET = 270856265247755311818440405236198340491_u128 as f64;
_7 = _1;
_12 = !82_u8;
_6 = _3.0 as isize;
Goto(bb1)
}
bb1 = {
_7 = -_11;
_9.0 = _3.0;
_3 = (_9.0,);
_2 = Adt49 { fld0: _13 };
_17 = '\u{9c545}';
_19.1.0 = [2187951940_u32];
_4 = _7;
_19.1.2 = !_12;
_19.2.1 = 3712482431_u32;
_13 = 14348966298208205713_u64 as isize;
_14 = _3.0 as isize;
_10.0 = [true,true,true,false,false,false];
_11 = _6;
_5 = -_6;
_19.2.2 = _12;
_20 = 65_i8 as i128;
_3.0 = _9.0;
_19.2.2 = _14 as u8;
_8 = _13;
_7 = _2.fld0;
_5 = !_13;
_16 = _20 as f32;
Call(_1 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-8411424140810156566_i64) as f64;
_8 = _1 + _7;
_6 = !_1;
_10.0 = [false,false,true,false,false,false];
_19.1.1 = !_19.2.1;
RET = 7107606588123466470_i64 as f64;
match _19.2.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
3712482431 => bb8,
_ => bb7
}
}
bb3 = {
_7 = -_11;
_9.0 = _3.0;
_3 = (_9.0,);
_2 = Adt49 { fld0: _13 };
_17 = '\u{9c545}';
_19.1.0 = [2187951940_u32];
_4 = _7;
_19.1.2 = !_12;
_19.2.1 = 3712482431_u32;
_13 = 14348966298208205713_u64 as isize;
_14 = _3.0 as isize;
_10.0 = [true,true,true,false,false,false];
_11 = _6;
_5 = -_6;
_19.2.2 = _12;
_20 = 65_i8 as i128;
_3.0 = _9.0;
_19.2.2 = _14 as u8;
_8 = _13;
_7 = _2.fld0;
_5 = !_13;
_16 = _20 as f32;
Call(_1 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
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
_16 = (-50_i8) as f32;
_1 = _8 | _8;
_19.2.1 = !_19.1.1;
_19.0 = _19.2.1 >> _1;
_8 = (-6732579740151223757_i64) as isize;
Goto(bb9)
}
bb9 = {
_4 = _14 << _19.2.1;
_19.0 = !_19.2.1;
_9 = _3;
_19.2 = _19.1;
_11 = _2.fld0 | _4;
_19.2 = (_19.1.0, _19.0, _12);
_19.1.0 = [_19.2.1];
_5 = -_1;
_19.2 = (_19.1.0, _19.1.1, _12);
_21 = -_16;
_19.1.1 = _19.0 - _19.2.1;
_21 = _16;
_17 = '\u{22363}';
_19.1.0 = _19.2.0;
Goto(bb10)
}
bb10 = {
_7 = _13;
_14 = _5 ^ _5;
_25 = _14 == _7;
_11 = _4 | _5;
_19.1.2 = _19.2.2 * _12;
_10.0 = [_25,_25,_25,_25,_25,_25];
_19.1.2 = _12;
_5 = !_14;
_19.2.1 = !_19.1.1;
_19.1 = _19.2;
_5 = (-15917_i16) as isize;
_11 = _14;
_6 = _11;
RET = _6 as f64;
_8 = _17 as isize;
_1 = _14;
_9.0 = !_3.0;
_27 = !_19.1.2;
_17 = '\u{41bbe}';
_5 = _1 + _14;
_6 = _5;
_2.fld0 = !_5;
_8 = _5 ^ _5;
_19.1.0 = _19.2.0;
_23 = _20 * _20;
_2.fld0 = _17 as isize;
Goto(bb11)
}
bb11 = {
Call(_29 = dump_var(14_usize, 6_usize, Move(_6), 4_usize, Move(_4), 19_usize, Move(_19), 5_usize, Move(_5)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_29 = dump_var(14_usize, 23_usize, Move(_23), 9_usize, Move(_9), 10_usize, Move(_10), 25_usize, Move(_25)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_29 = dump_var(14_usize, 8_usize, Move(_8), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: ([u32; 1], u32, u8),mut _2: ([u32; 1], u32, u8),mut _3: ([bool; 6],)) -> u32 {
mir! {
type RET = u32;
let _4: Adt53;
let _5: Adt58;
let _6: f32;
let _7: (u16,);
let _8: Adt49;
let _9: i128;
let _10: ();
let _11: ();
{
_2 = (_1.0, _1.1, _1.2);
RET = _2.1;
RET = 5_usize as u32;
_1.0 = [_1.1];
_2.1 = _1.1;
_4.fld0 = [142298349127519401929144727645256420630_i128,107508537862061610579038021442980731843_i128,19165146597552357445526118826548691115_i128,(-135908933246137398128908446288811467113_i128),(-49372616118854267213649975547101640389_i128)];
_1.2 = !_2.2;
_2.0 = _1.0;
_3.0 = [false,true,false,false,true,true];
_2 = (_1.0, _1.1, _1.2);
_2.2 = !_1.2;
_1.1 = !_2.1;
RET = !_1.1;
_3.0 = [false,true,false,false,false,true];
_2.0 = [_1.1];
_1.1 = false as u32;
_4.fld0 = [19078323949587971967796010949012413305_i128,169959032069679400168229832760627961005_i128,116787754581259619317016519755497251292_i128,(-93781246027009294653450236671901291564_i128),53425729374932499149075394350535718397_i128];
_1 = _2;
_1 = _2;
_2.1 = 101184066733706315602709147745991606550_u128 as u32;
_1.2 = _2.2;
_6 = 39183527756022335030338928956618864097_u128 as f32;
_1.1 = RET << RET;
_3.0 = [true,true,false,false,true,true];
_7 = (55779_u16,);
_7 = (18201_u16,);
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(15_usize, 7_usize, Move(_7), 2_usize, Move(_2), 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *mut isize,mut _2: [u32; 1]) -> ([u32; 1], u32, u8) {
mir! {
type RET = ([u32; 1], u32, u8);
let _3: ();
let _4: ();
{
RET = (_2, 2810753233_u32, 180_u8);
RET.2 = 247685508763969572129589665307089586655_u128 as u8;
(*_1) = 9223372036854775807_isize - 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut isize,mut _2: isize,mut _3: char,mut _4: *mut isize,mut _5: *mut isize,mut _6: isize,mut _7: u64,mut _8: isize,mut _9: [isize; 3],mut _10: u16,mut _11: isize,mut _12: isize,mut _13: *mut isize,mut _14: *mut isize,mut _15: i128) -> (i16,) {
mir! {
type RET = (i16,);
let _16: Adt56;
let _17: f32;
let _18: Adt51;
let _19: (u128,);
let _20: (i32, *mut bool);
let _21: [i128; 3];
let _22: f32;
let _23: [u32; 8];
let _24: Adt54;
let _25: [u32; 8];
let _26: Adt53;
let _27: f64;
let _28: char;
let _29: (u128,);
let _30: (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8));
let _31: Adt46;
let _32: i8;
let _33: u8;
let _34: u8;
let _35: (i16,);
let _36: Adt45;
let _37: [char; 4];
let _38: isize;
let _39: *mut bool;
let _40: char;
let _41: ();
let _42: ();
{
(*_13) = !_12;
RET = (29725_i16,);
_8 = -(*_5);
_2 = !_11;
_6 = 120_u8 as isize;
RET = (15148_i16,);
_14 = core::ptr::addr_of_mut!((*_14));
Goto(bb1)
}
bb1 = {
(*_13) = _12;
(*_13) = _7 as isize;
_16 = Adt56::Variant0 { fld0: RET };
RET = Field::<(i16,)>(Variant(_16, 0), 0);
_14 = core::ptr::addr_of_mut!((*_5));
(*_1) = _8;
RET = Field::<(i16,)>(Variant(_16, 0), 0);
(*_1) = _8;
_2 = (*_4) & (*_5);
_4 = core::ptr::addr_of_mut!((*_4));
_8 = (*_5) * (*_1);
_17 = _8 as f32;
match RET.0 {
0 => bb2,
1 => bb3,
15148 => bb5,
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
_8 = (*_14);
(*_5) = -_8;
_4 = _13;
(*_1) = -_11;
(*_14) = _11;
RET = Field::<(i16,)>(Variant(_16, 0), 0);
Call((*_14) = core::intrinsics::transmute(_11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13 = core::ptr::addr_of_mut!((*_5));
(*_13) = _6;
_22 = _17 * _17;
SetDiscriminant(_16, 3);
_23 = [2841950350_u32,1856896365_u32,911612922_u32,565437891_u32,674729464_u32,336641518_u32,792236863_u32,754805386_u32];
_2 = _11;
Call(_2 = core::intrinsics::transmute((*_13)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Goto(bb8)
}
bb8 = {
(*_5) = _6 + _2;
(*_1) = _8;
_1 = core::ptr::addr_of_mut!((*_14));
place!(Field::<u128>(Variant(_16, 3), 2)) = 11630548648972440136_usize as u128;
_20.0 = -1437045298_i32;
_19 = (Field::<u128>(Variant(_16, 3), 2),);
_15 = 189_u8 as i128;
place!(Field::<[bool; 6]>(Variant(_16, 3), 0)) = [false,false,false,false,true,false];
_8 = (*_14);
_11 = (*_14) + (*_5);
_7 = 8875890869492423093_u64 & 2227651313462595171_u64;
(*_13) = RET.0 as isize;
_21 = [_15,_15,_15];
(*_5) = _6 & _11;
RET = (31747_i16,);
RET = ((-7387_i16),);
_26.fld0 = [_15,_15,_15,_15,_15];
_19.0 = Field::<u128>(Variant(_16, 3), 2) - Field::<u128>(Variant(_16, 3), 2);
_11 = !(*_5);
(*_5) = _11;
_21 = [_15,_15,_15];
_27 = 73_u8 as f64;
_25 = [3162734886_u32,64002232_u32,2849057670_u32,1067087415_u32,2185133096_u32,334729747_u32,1869488351_u32,2241940204_u32];
_2 = 14407424279999966262_usize as isize;
_28 = _3;
Goto(bb9)
}
bb9 = {
_23 = [1512352014_u32,3181041289_u32,1352358928_u32,3120545848_u32,1166718301_u32,2158667945_u32,1285857999_u32,3060433418_u32];
_7 = 18405924834937957111_u64;
_9 = [_6,_11,(*_14)];
_9 = [_8,(*_13),(*_13)];
_7 = 8832981806836524794_u64;
_10 = _19.0 as u16;
_22 = -_17;
place!(Field::<u8>(Variant(_16, 3), 1)) = 80_u8 >> (*_1);
(*_4) = _22 as isize;
_19.0 = Field::<u128>(Variant(_16, 3), 2) & Field::<u128>(Variant(_16, 3), 2);
_30.2.2 = true as u8;
_15 = (-159583995791713853053537051511317734117_i128);
_20.0 = (-667524106_i32) + (-1608215390_i32);
_26.fld0 = [_15,_15,_15,_15,_15];
(*_5) = !_11;
_10 = 21284_u16;
_15 = (-74842118335907002541258307363982581396_i128);
_1 = core::ptr::addr_of_mut!((*_5));
(*_13) = _11;
RET = ((-30567_i16),);
_33 = !Field::<u8>(Variant(_16, 3), 1);
_30.1.0 = [1185306239_u32];
RET.0 = (-1444108481630530639_i64) as i16;
_32 = 47_i8 * 33_i8;
_2 = (*_14) & (*_1);
_30.2 = (_30.1.0, 269173323_u32, Field::<u8>(Variant(_16, 3), 1));
Goto(bb10)
}
bb10 = {
_35 = RET;
_30.1.1 = 8978358050649818733_i64 as u32;
_29.0 = _19.0;
_30.1 = (_30.2.0, _30.2.1, _30.2.2);
_30.1.1 = _30.2.1;
match _30.2.1 {
0 => bb8,
269173323 => bb11,
_ => bb4
}
}
bb11 = {
SetDiscriminant(_16, 1);
_12 = !(*_1);
_6 = (*_1) * _2;
_6 = _30.1.1 as isize;
_22 = _17;
_19.0 = !_29.0;
place!(Field::<i32>(Variant(_16, 1), 5)) = -_20.0;
place!(Field::<i32>(Variant(_16, 1), 5)) = _10 as i32;
_34 = _33 & _30.1.2;
(*_1) = -_12;
_35 = RET;
Goto(bb12)
}
bb12 = {
_28 = _3;
Goto(bb13)
}
bb13 = {
_16 = Adt56::Variant0 { fld0: _35 };
SetDiscriminant(_16, 0);
Goto(bb14)
}
bb14 = {
_35.0 = !RET.0;
(*_4) = !_11;
_19.0 = !_29.0;
place!(Field::<(i16,)>(Variant(_16, 0), 0)) = (RET.0,);
RET.0 = _35.0 >> _6;
RET = (Field::<(i16,)>(Variant(_16, 0), 0).0,);
_7 = !8440385032695827356_u64;
_26.fld0 = [_15,_15,_15,_15,_15];
_2 = !(*_5);
_6 = (*_4);
_11 = -_8;
RET = (_35.0,);
(*_5) = _6 << _30.2.1;
(*_5) = _2;
RET = (_35.0,);
_30.1 = (_30.2.0, _30.2.1, _33);
_29 = (_19.0,);
RET = _35;
SetDiscriminant(_16, 3);
place!(Field::<[bool; 6]>(Variant(_16, 3), 0)) = [false,true,false,false,false,false];
_17 = _32 as f32;
_38 = (*_5);
(*_5) = !_11;
_20.0 = (-139807718_i32) << _34;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(17_usize, 11_usize, Move(_11), 32_usize, Move(_32), 21_usize, Move(_21), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(17_usize, 29_usize, Move(_29), 8_usize, Move(_8), 35_usize, Move(_35), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(17_usize, 9_usize, Move(_9), 34_usize, Move(_34), 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: char,mut _2: i128,mut _3: i128,mut _4: [u32; 1],mut _5: bool,mut _6: isize,mut _7: u8,mut _8: isize,mut _9: bool,mut _10: u16,mut _11: (i16,),mut _12: u8,mut _13: *mut isize,mut _14: u8,mut _15: u64,mut _16: u32) -> i32 {
mir! {
type RET = i32;
let _17: isize;
let _18: Adt46;
let _19: [u16; 5];
let _20: u8;
let _21: isize;
let _22: isize;
let _23: i64;
let _24: f32;
let _25: isize;
let _26: usize;
let _27: isize;
let _28: [u16; 5];
let _29: *mut *const i128;
let _30: [isize; 3];
let _31: (u16,);
let _32: (i16,);
let _33: [u16; 5];
let _34: [u16; 5];
let _35: &'static f64;
let _36: ([bool; 6],);
let _37: Adt47;
let _38: u128;
let _39: u64;
let _40: Adt49;
let _41: Adt52;
let _42: ();
let _43: ();
{
_3 = _2;
_12 = _7 << _3;
_15 = 7_usize as u64;
RET = 836741114_i32 >> _7;
(*_13) = -_8;
_17 = _6 * _6;
_2 = _3 - _3;
_2 = _3;
_14 = 14176793941527545415_usize as u8;
RET = -405234517_i32;
_4 = [_16];
_7 = !_12;
_11.0 = -(-18785_i16);
_15 = 17913317638409375391_u64;
_8 = _17;
_11.0 = (-18969_i16);
_7 = !_14;
_21 = _17 | _8;
_20 = !_12;
_10 = 19605_u16;
RET = -290574430_i32;
_11.0 = 19442_i16 << _20;
_22 = 6_usize as isize;
RET = -(-1191158427_i32);
Goto(bb1)
}
bb1 = {
_14 = _1 as u8;
_17 = (*_13) + (*_13);
_19 = [_10,_10,_10,_10,_10];
_24 = _20 as f32;
(*_13) = _17 | _21;
_20 = _7 ^ _12;
_19 = [_10,_10,_10,_10,_10];
_19 = [_10,_10,_10,_10,_10];
_24 = (*_13) as f32;
(*_13) = !_22;
_11 = (29428_i16,);
_9 = !_5;
_25 = _24 as isize;
_26 = _5 as usize;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
17913317638409375391 => bb6,
_ => bb5
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
Return()
}
bb6 = {
_11.0 = !(-30331_i16);
_21 = !_17;
_15 = !17309216317039465187_u64;
_30 = [_25,_25,_25];
_1 = '\u{bb1ab}';
RET = 1323707889_i32 + 946595400_i32;
_7 = _12 >> _25;
_20 = 41_i8 as u8;
RET = _21 as i32;
_15 = _25 as u64;
_28 = [_10,_10,_10,_10,_10];
_5 = !_9;
_22 = _25 * _25;
Goto(bb7)
}
bb7 = {
_2 = _3;
_5 = _9;
_31.0 = _7 as u16;
_17 = _25;
_12 = 6084217764976516893_i64 as u8;
_21 = _22 + _22;
_26 = _15 as usize;
_2 = _31.0 as i128;
_9 = _5;
_32.0 = _1 as i16;
_28 = [_31.0,_31.0,_31.0,_31.0,_31.0];
_25 = -_21;
_19 = _28;
_17 = _9 as isize;
_10 = !_31.0;
_11 = (_32.0,);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb8,
4 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb8 = {
_11.0 = !(-30331_i16);
_21 = !_17;
_15 = !17309216317039465187_u64;
_30 = [_25,_25,_25];
_1 = '\u{bb1ab}';
RET = 1323707889_i32 + 946595400_i32;
_7 = _12 >> _25;
_20 = 41_i8 as u8;
RET = _21 as i32;
_15 = _25 as u64;
_28 = [_10,_10,_10,_10,_10];
_5 = !_9;
_22 = _25 * _25;
Goto(bb7)
}
bb9 = {
_14 = _1 as u8;
_17 = (*_13) + (*_13);
_19 = [_10,_10,_10,_10,_10];
_24 = _20 as f32;
(*_13) = _17 | _21;
_20 = _7 ^ _12;
_19 = [_10,_10,_10,_10,_10];
_19 = [_10,_10,_10,_10,_10];
_24 = (*_13) as f32;
(*_13) = !_22;
_11 = (29428_i16,);
_9 = !_5;
_25 = _24 as isize;
_26 = _5 as usize;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
17913317638409375391 => bb6,
_ => bb5
}
}
bb10 = {
Return()
}
bb11 = {
RET = _26 as i32;
_8 = _21;
_31 = (_10,);
_27 = !_21;
_4 = [_16];
_33 = _28;
_28 = [_10,_31.0,_10,_10,_10];
_5 = _8 < _8;
Goto(bb12)
}
bb12 = {
_23 = 54343908684643156566380056560491474243_u128 as i64;
Goto(bb13)
}
bb13 = {
_12 = _7;
RET = (-1998420903_i32);
_21 = _25 ^ _25;
_14 = _24 as u8;
RET = (-748078264_i32);
_2 = !_3;
_14 = _7 | _12;
_33 = [_31.0,_31.0,_10,_31.0,_31.0];
_24 = _16 as f32;
(*_13) = _21 & _21;
_7 = _21 as u8;
_25 = !_21;
_22 = -(*_13);
_32.0 = _16 as i16;
RET = 79592380844961669804215024911178936734_u128 as i32;
match _6 {
0 => bb7,
1 => bb12,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb14 = {
_23 = 54343908684643156566380056560491474243_u128 as i64;
Goto(bb13)
}
bb15 = {
_14 = _1 as u8;
_17 = (*_13) + (*_13);
_19 = [_10,_10,_10,_10,_10];
_24 = _20 as f32;
(*_13) = _17 | _21;
_20 = _7 ^ _12;
_19 = [_10,_10,_10,_10,_10];
_19 = [_10,_10,_10,_10,_10];
_24 = (*_13) as f32;
(*_13) = !_22;
_11 = (29428_i16,);
_9 = !_5;
_25 = _24 as isize;
_26 = _5 as usize;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
17913317638409375391 => bb6,
_ => bb5
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_2 = _3;
_5 = _9;
_31.0 = _7 as u16;
_17 = _25;
_12 = 6084217764976516893_i64 as u8;
_21 = _22 + _22;
_26 = _15 as usize;
_2 = _31.0 as i128;
_9 = _5;
_32.0 = _1 as i16;
_28 = [_31.0,_31.0,_31.0,_31.0,_31.0];
_25 = -_21;
_19 = _28;
_17 = _9 as isize;
_10 = !_31.0;
_11 = (_32.0,);
match _6 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb8,
4 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb19 = {
_23 = (-3250411349289505366_i64);
(*_13) = _25 & _25;
(*_13) = _22 ^ _22;
_34 = [_10,_31.0,_31.0,_10,_31.0];
_23 = _7 as i64;
_8 = !(*_13);
_23 = _26 as i64;
_33 = [_10,_31.0,_31.0,_10,_10];
_24 = _31.0 as f32;
_37 = Adt47::Variant0 { fld0: _31 };
_32.0 = RET as i16;
_38 = 97177698277081836296156141073498246640_u128 ^ 130564063155880317533470239328079516515_u128;
_40.fld0 = _32.0 as isize;
Goto(bb20)
}
bb20 = {
Call(_42 = dump_var(18_usize, 5_usize, Move(_5), 14_usize, Move(_14), 19_usize, Move(_19), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_42 = dump_var(18_usize, 28_usize, Move(_28), 12_usize, Move(_12), 1_usize, Move(_1), 23_usize, Move(_23)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_42 = dump_var(18_usize, 27_usize, Move(_27), 17_usize, Move(_17), 38_usize, Move(_38), 20_usize, Move(_20)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_42 = dump_var(18_usize, 33_usize, Move(_33), 34_usize, Move(_34), 30_usize, Move(_30), 43_usize, _43), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{8f9d2}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box((-55_i8)), std::hint::black_box(31452_i16), std::hint::black_box((-815448975_i32)), std::hint::black_box((-586460390995864098_i64)), std::hint::black_box((-58794338528584575681612389131742121176_i128)), std::hint::black_box(6_usize), std::hint::black_box(245_u8), std::hint::black_box(2925_u16), std::hint::black_box(3575564625_u32), std::hint::black_box(16696044470602531556_u64), std::hint::black_box(163801573208050959415059199384679010606_u128));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: [isize; 3],
fld2: [u16; 5],
fld3: (i16,),
fld4: [i128; 3],
fld5: [bool; 6],
fld6: i64,
fld7: (u128, u8),

},
Variant1{
fld0: u16,
fld1: i64,
fld2: (u128,),
fld3: *mut *const i128,

},
Variant2{
fld0: *mut isize,
fld1: u32,

},
Variant3{
fld0: [char; 4],
fld1: u128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: f32,
fld1: (u16,),
fld2: [bool; 6],
fld3: [i128; 3],
fld4: [char; 4],
fld5: i32,

},
Variant1{
fld0: isize,
fld1: [u16; 5],

},
Variant2{
fld0: *const i128,
fld1: u128,
fld2: *mut bool,
fld3: u64,
fld4: (*const i128,),
fld5: (i32, *mut bool),
fld6: ([u32; 1], u32, u8),
fld7: (u128, u8),

},
Variant3{
fld0: (i32, *mut bool),
fld1: f32,
fld2: isize,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: (u128, u8),
fld1: i128,
fld2: isize,
fld3: u8,
fld4: f32,
fld5: u128,

},
Variant1{
fld0: (u128,),
fld1: (u128, u8),
fld2: [u16; 5],
fld3: *mut bool,
fld4: [bool; 6],
fld5: *mut *mut *const i128,

},
Variant2{
fld0: bool,
fld1: [isize; 3],
fld2: i64,
fld3: *mut isize,
fld4: [char; 4],

},
Variant3{
fld0: u16,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: f32,
fld1: ([u32; 1], u32, u8),
fld2: *mut isize,

},
Variant1{
fld0: Adt43,
fld1: u8,
fld2: ([u32; 1], u32, u8),
fld3: usize,
fld4: [char; 4],
fld5: (u128, u8),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: [u16; 5],
fld2: *mut isize,
fld3: f32,
fld4: f64,
fld5: Adt44,

},
Variant1{
fld0: f32,
fld1: char,
fld2: isize,
fld3: ([u32; 1], u32, u8),
fld4: (i32, *mut bool),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (u16,),

},
Variant1{
fld0: u32,
fld1: char,
fld2: *mut *const i128,
fld3: i8,
fld4: (u128,),

},
Variant2{
fld0: ([u32; 1], u32, u8),
fld1: [u32; 1],
fld2: *mut bool,
fld3: f64,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: f32,
fld1: (u16,),
fld2: isize,
fld3: [i128; 5],
fld4: Adt42,
fld5: (u128,),
fld6: u128,

},
Variant1{
fld0: *mut *mut *const i128,

},
Variant2{
fld0: ([bool; 6],),
fld1: usize,

},
Variant3{
fld0: *mut *mut *const i128,
fld1: (*const i128,),
fld2: [u32; 1],
fld3: f64,
fld4: i16,
fld5: ([bool; 6],),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: isize,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt44,
fld1: u128,
fld2: i64,
fld3: i128,
fld4: (i16,),
fld5: (u128,),

},
Variant1{
fld0: u16,
fld1: Adt47,
fld2: usize,
fld3: ([u32; 1], u32, u8),
fld4: u64,

},
Variant2{
fld0: [u16; 5],
fld1: [char; 4],
fld2: (*const i128,),
fld3: i8,
fld4: [bool; 6],

},
Variant3{
fld0: (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8)),
fld1: *const i128,
fld2: i16,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (*const i128,),
fld1: u8,

},
Variant1{
fld0: bool,
fld1: (i32, *mut bool),
fld2: ([u32; 1], u32, u8),

},
Variant2{
fld0: *mut *const i128,
fld1: [u16; 5],
fld2: [isize; 3],
fld3: (*const i128,),
fld4: *mut isize,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u32,

},
Variant1{
fld0: Adt44,
fld1: [char; 4],

},
Variant2{
fld0: Adt49,
fld1: [u16; 5],

},
Variant3{
fld0: ([u32; 1], u32, u8),
fld1: Adt51,
fld2: Adt49,
fld3: ([bool; 6],),
fld4: *const i128,
fld5: i32,
fld6: Adt43,
fld7: [bool; 6],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: [i128; 5],
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt45,
fld1: Adt49,
fld2: Adt48,
fld3: [isize; 3],
fld4: (u16,),
fld5: u64,

},
Variant1{
fld0: Adt46,
fld1: char,
fld2: u16,
fld3: Adt52,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: Adt43,
fld1: usize,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (i16,),

},
Variant1{
fld0: Adt44,
fld1: Adt42,
fld2: u8,
fld3: *const i128,
fld4: Adt52,
fld5: i32,
fld6: [char; 4],

},
Variant2{
fld0: f32,
fld1: i128,
fld2: Adt53,
fld3: [isize; 3],
fld4: [u16; 5],
fld5: *mut isize,
fld6: Adt52,

},
Variant3{
fld0: [bool; 6],
fld1: u8,
fld2: u128,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (u16,),
fld1: Adt47,
fld2: [bool; 6],

},
Variant1{
fld0: Adt50,
fld1: Adt44,
fld2: [i128; 3],
fld3: (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8)),
fld4: Adt52,
fld5: [i128; 5],
fld6: Adt54,

},
Variant2{
fld0: (*const i128,),
fld1: (*mut *const i128, (u32, ([u32; 1], u32, u8), ([u32; 1], u32, u8)), *mut *const i128, i8),
fld2: i64,
fld3: [char; 4],
fld4: ([u32; 1], u32, u8),

},
Variant3{
fld0: *mut *const i128,
fld1: [i128; 5],
fld2: u32,
fld3: usize,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: i32,
fld1: u128,
fld2: Adt51,
fld3: i8,
fld4: [i128; 3],

},
Variant1{
fld0: Adt50,

},
Variant2{
fld0: Adt51,
fld1: Adt43,
fld2: [bool; 6],
fld3: (u16,),
fld4: [u16; 5],
fld5: i32,
fld6: [u32; 1],
fld7: [u32; 8],

}}

