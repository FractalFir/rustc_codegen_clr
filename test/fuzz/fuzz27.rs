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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: u64,mut _5: u8,mut _6: i32,mut _7: i64,mut _8: i128) -> char {
mir! {
type RET = char;
let _9: Adt56;
let _10: char;
let _11: [u64; 5];
let _12: (char, *const u128, u32, u16);
let _13: isize;
let _14: i128;
let _15: f64;
let _16: [i64; 8];
let _17: [u128; 2];
let _18: Adt51;
let _19: u8;
let _20: [usize; 2];
let _21: char;
let _22: isize;
let _23: i8;
let _24: u8;
let _25: ((char, *const u128, u32, u16),);
let _26: f64;
let _27: isize;
let _28: ([i64; 8],);
let _29: [u16; 1];
let _30: f32;
let _31: ([u16; 1], f64, u8);
let _32: [i64; 8];
let _33: ();
let _34: ();
{
RET = '\u{f8f8f}';
_5 = 242_u8;
_2 = RET;
_7 = (-7681348597144257154_i64) - 3897404491194240931_i64;
_3 = 231320268369266346420342807035650871247_u128 & 259298111501041644688640022315001988305_u128;
RET = _2;
_3 = (-103915033562143492919733529477176547022_i128) as u128;
_6 = !(-2077319736_i32);
RET = _2;
_3 = true as u128;
_4 = 15064456635386496865_u64;
_7 = 6351759885418033050_i64 | (-287823032579014422_i64);
_4 = _7 as u64;
_8 = !112506222557503274761037079422727049133_i128;
_10 = RET;
_7 = !(-8116803858121528867_i64);
_8 = 2333600192_u32 as i128;
_1 = true;
_10 = RET;
_3 = !338999865072220716725024550447077472939_u128;
_8 = 14563_i16 as i128;
RET = _10;
_6 = 537143070_i32 | 246023555_i32;
_3 = 106685089218723701777035946741547438115_u128 >> _7;
_6 = (-2098900035_i32) << _7;
_10 = _2;
_7 = 4310659933189890297_i64;
_10 = _2;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
4310659933189890297 => bb5,
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
_4 = (-9223372036854775808_isize) as u64;
_10 = RET;
RET = _2;
_4 = !15701643270380085680_u64;
_1 = !false;
_7 = 1023944208095590979_i64;
_8 = 163167832619197316484165017746938163915_i128 & (-39858312323723735042880563151664660507_i128);
_3 = _4 as u128;
_12.2 = !273064059_u32;
_12.1 = core::ptr::addr_of!(_3);
_11 = [_4,_4,_4,_4,_4];
RET = _10;
_2 = _10;
_12.2 = 27329_i16 as u32;
_6 = 445262444_i32;
_12.0 = _10;
_5 = !230_u8;
_12.1 = core::ptr::addr_of!(_3);
match _6 {
0 => bb1,
1 => bb2,
445262444 => bb6,
_ => bb4
}
}
bb6 = {
_6 = (-97_i8) as i32;
_12.0 = _2;
_14 = _8 << _6;
_5 = 142_u8 + 47_u8;
_4 = 2497780875883377832_u64;
_11 = [_4,_4,_4,_4,_4];
_12.2 = 7487_i16 as u32;
RET = _2;
_7 = (-1399839384125273303_i64);
RET = _10;
_10 = RET;
_4 = 9679586563755763285_u64 - 16921908840260579127_u64;
_16 = [_7,_7,_7,_7,_7,_7,_7,_7];
_8 = _14 | _14;
_12.1 = core::ptr::addr_of!(_3);
Call(_12.0 = fn1(_6, _8, _14), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = _12.0;
_8 = _14;
_15 = _6 as f64;
_7 = (-2760481290645700635_i64) * (-8398363886007686585_i64);
RET = _2;
_2 = RET;
_12.3 = 62084_u16;
_12.1 = core::ptr::addr_of!(_3);
_17 = [_3,_3];
_16 = [_7,_7,_7,_7,_7,_7,_7,_7];
_12.3 = 48180_u16 << _14;
_15 = _5 as f64;
_5 = 9_u8 >> _8;
_19 = !_5;
Goto(bb8)
}
bb8 = {
_12.0 = _2;
_20 = [5672208543631138675_usize,13540537278223239757_usize];
_12.3 = 42597_u16 ^ 41162_u16;
RET = _12.0;
_10 = _12.0;
_5 = _12.3 as u8;
_23 = 9223372036854775807_isize as i8;
_19 = !_5;
_25.0 = (_2, _12.1, _12.2, _12.3);
_14 = _8 & _8;
RET = _25.0.0;
_12.3 = _25.0.3 * _25.0.3;
_26 = _15;
_3 = _26 as u128;
_8 = _14 + _14;
_10 = _2;
_15 = (-23490_i16) as f64;
Goto(bb9)
}
bb9 = {
_21 = _12.0;
_25 = (_12,);
_22 = _10 as isize;
_10 = _2;
_20 = [5_usize,638231216673048814_usize];
_24 = !_5;
_10 = _21;
_1 = !false;
_27 = !_22;
_15 = _26 * _26;
_7 = 7227631399716775282_i64 ^ 739417305036072665_i64;
RET = _10;
_25 = (_12,);
_15 = _26;
RET = _2;
_19 = !_24;
_4 = 3612768582499076694_u64 ^ 14250610909739787896_u64;
_10 = RET;
_11 = [_4,_4,_4,_4,_4];
_12 = (_25.0.0, _25.0.1, _25.0.2, _25.0.3);
_6 = (-1238900017_i32) + 1417968807_i32;
_21 = _12.0;
_11 = [_4,_4,_4,_4,_4];
Goto(bb10)
}
bb10 = {
_26 = _15 - _15;
_16 = [_7,_7,_7,_7,_7,_7,_7,_7];
_6 = (-112839165_i32);
_12.3 = !_25.0.3;
_1 = !true;
_12.0 = _25.0.0;
_21 = _2;
_15 = _14 as f64;
match _6 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb11,
4 => bb12,
340282366920938463463374607431655372291 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
_12.0 = _2;
_20 = [5672208543631138675_usize,13540537278223239757_usize];
_12.3 = 42597_u16 ^ 41162_u16;
RET = _12.0;
_10 = _12.0;
_5 = _12.3 as u8;
_23 = 9223372036854775807_isize as i8;
_19 = !_5;
_25.0 = (_2, _12.1, _12.2, _12.3);
_14 = _8 & _8;
RET = _25.0.0;
_12.3 = _25.0.3 * _25.0.3;
_26 = _15;
_3 = _26 as u128;
_8 = _14 + _14;
_10 = _2;
_15 = (-23490_i16) as f64;
Goto(bb9)
}
bb13 = {
_6 = (-97_i8) as i32;
_12.0 = _2;
_14 = _8 << _6;
_5 = 142_u8 + 47_u8;
_4 = 2497780875883377832_u64;
_11 = [_4,_4,_4,_4,_4];
_12.2 = 7487_i16 as u32;
RET = _2;
_7 = (-1399839384125273303_i64);
RET = _10;
_10 = RET;
_4 = 9679586563755763285_u64 - 16921908840260579127_u64;
_16 = [_7,_7,_7,_7,_7,_7,_7,_7];
_8 = _14 | _14;
_12.1 = core::ptr::addr_of!(_3);
Call(_12.0 = fn1(_6, _8, _14), ReturnTo(bb7), UnwindUnreachable())
}
bb14 = {
_25.0 = (_10, _12.1, _12.2, _12.3);
_28.0 = _16;
_14 = _8 - _8;
_25.0 = _12;
_25.0.0 = _21;
_1 = false;
_25.0.1 = core::ptr::addr_of!(_3);
_26 = _15;
_12 = (_2, _25.0.1, _25.0.2, _25.0.3);
_16 = [_7,_7,_7,_7,_7,_7,_7,_7];
_17 = [_3,_3];
_25.0.0 = _10;
RET = _2;
_12.1 = _25.0.1;
_12.2 = _25.0.2 * _25.0.2;
_21 = RET;
_31.1 = _26 - _15;
_29 = [_12.3];
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 5_usize, Move(_5), 7_usize, Move(_7), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 10_usize, Move(_10), 4_usize, Move(_4), 3_usize, Move(_3), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 24_usize, Move(_24), 22_usize, Move(_22), 2_usize, Move(_2), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i32,mut _2: i128,mut _3: i128) -> char {
mir! {
type RET = char;
let _4: [u128; 2];
let _5: [u128; 2];
let _6: [u16; 1];
let _7: Adt56;
let _8: i8;
let _9: f64;
let _10: [usize; 2];
let _11: [u128; 2];
let _12: [u64; 5];
let _13: char;
let _14: [i16; 4];
let _15: Adt49;
let _16: [u16; 1];
let _17: *mut [usize; 2];
let _18: u128;
let _19: char;
let _20: [i16; 4];
let _21: i64;
let _22: ([i64; 8],);
let _23: *mut f32;
let _24: [i64; 8];
let _25: f64;
let _26: ((char, *const u128, u32, u16),);
let _27: f32;
let _28: [i64; 8];
let _29: char;
let _30: f32;
let _31: char;
let _32: usize;
let _33: Adt57;
let _34: ();
let _35: ();
{
RET = '\u{291bd}';
RET = '\u{5b9d2}';
RET = '\u{a0f4a}';
_1 = (-2110228642_i32);
_4 = [83644414715314580389240635923854687565_u128,120245741448097670628723029684195260499_u128];
_2 = _3 << _1;
Call(_2 = fn2(RET, _3, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [194982825120563662287484246769305429066_u128,252643826739463328541270360919856407889_u128];
Goto(bb2)
}
bb2 = {
_5 = [49837696009070737001008818870509665482_u128,45394544640243062708803062686563342877_u128];
_6 = [41663_u16];
_8 = (-115_i8);
_10 = [5_usize,9625644010079271289_usize];
RET = '\u{a4fa8}';
_10 = [2_usize,6332006537106767051_usize];
_9 = 18415668501610684029_u64 as f64;
RET = '\u{8e936}';
_3 = _2 * _2;
_2 = true as i128;
_8 = _1 as i8;
RET = '\u{8b268}';
RET = '\u{4330e}';
_4 = [51014640962922834799953075035466402607_u128,245246151879083673238695513706815401911_u128];
_9 = 3167598703463843054_i64 as f64;
_8 = 25131_u16 as i8;
_5 = [114974588424320692481770392509845223241_u128,108138683606868349380575557798731690473_u128];
_8 = true as i8;
_12 = [10231760857792385379_u64,17969379584267931280_u64,6737437556378883596_u64,18345675672879090825_u64,12225044050382792246_u64];
RET = '\u{a90c9}';
RET = '\u{c68f3}';
_10 = [0_usize,0_usize];
match _1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607429657982814 => bb11,
_ => bb10
}
}
bb3 = {
_5 = [194982825120563662287484246769305429066_u128,252643826739463328541270360919856407889_u128];
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
Return()
}
bb11 = {
_5 = _4;
_1 = 73_u8 as i32;
RET = '\u{86e78}';
_11 = [240785512644550274581536429135325309894_u128,229292622972252600793214011264532404780_u128];
_16 = [63704_u16];
_3 = _8 as i128;
_10 = [6_usize,17295327807856256091_usize];
_8 = -(-35_i8);
_13 = RET;
RET = _13;
_1 = (-2030873968_i32);
_3 = 8368622730195748475_u64 as i128;
_8 = -49_i8;
_9 = 16054_u16 as f64;
_17 = core::ptr::addr_of_mut!(_10);
_18 = 121205520777285070617179522397126572051_u128;
_11 = [_18,_18];
_14 = [18763_i16,(-23793_i16),(-5716_i16),(-3804_i16)];
_10 = [4_usize,0_usize];
_6 = [23122_u16];
_3 = _2;
_20 = [1648_i16,21065_i16,21512_i16,782_i16];
RET = _13;
_6 = [35682_u16];
Goto(bb12)
}
bb12 = {
_4 = _5;
RET = _13;
RET = _13;
_12 = [75031933271511180_u64,12601405268297193900_u64,2451504116548808377_u64,12644091609504668462_u64,7652791412345712351_u64];
_11 = _4;
RET = _13;
_24 = [5965084752797840442_i64,696143652980019549_i64,(-4984774435397263238_i64),1120702241365095825_i64,196574760821088680_i64,(-8781188414712032057_i64),8090577004691806422_i64,(-6750237602218479704_i64)];
_12 = [17528566842739404092_u64,8113217987872644579_u64,8032599559558251036_u64,2742653394807184028_u64,15239863188914650294_u64];
_1 = 316836987_i32 + 1553861823_i32;
_22 = (_24,);
_11 = _4;
_22.0 = [6804661188901775044_i64,(-3450768260684274483_i64),(-2776495219043447635_i64),5411029204535592179_i64,571175001406457110_i64,4512267546498792955_i64,(-1650636280281548694_i64),(-7635164527688274887_i64)];
RET = _13;
_11 = [_18,_18];
_13 = RET;
_14 = [(-5601_i16),26607_i16,(-8461_i16),359_i16];
_19 = RET;
_9 = _1 as f64;
_20 = [29567_i16,31902_i16,(-1785_i16),8570_i16];
_14 = _20;
_21 = -(-4569998614867335472_i64);
(*_17) = [5299694440756881367_usize,3_usize];
_9 = (-56_isize) as f64;
(*_17) = [11061419036667788124_usize,6_usize];
Goto(bb13)
}
bb13 = {
_23 = core::ptr::addr_of_mut!(_27);
_26.0.0 = _13;
_21 = (-7639571634734785388_i64);
_2 = _3;
_28 = _22.0;
(*_17) = [15620754424353932136_usize,1215101937948994136_usize];
_26.0.0 = _19;
_27 = 17739342117295375770_u64 as f32;
_26.0.3 = 62169_u16;
_30 = _27;
_11 = _4;
_21 = -8167792499864802365_i64;
_2 = -_3;
_26.0.1 = core::ptr::addr_of!(_18);
match _18 {
121205520777285070617179522397126572051 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_26.0.0 = _13;
_27 = -_30;
(*_17) = [2_usize,6_usize];
_17 = core::ptr::addr_of_mut!((*_17));
_6 = [_26.0.3];
_32 = 5011524559366381951_usize + 17064702299811211236_usize;
_26.0.1 = core::ptr::addr_of!(_18);
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(1_usize, 4_usize, Move(_4), 28_usize, Move(_28), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(1_usize, 14_usize, Move(_14), 22_usize, Move(_22), 12_usize, Move(_12), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(1_usize, 3_usize, Move(_3), 32_usize, Move(_32), 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: char,mut _2: i128,mut _3: [u128; 2],mut _4: [u128; 2]) -> i128 {
mir! {
type RET = i128;
let _5: ([i64; 8],);
let _6: ((*mut &'static i64,), i8);
let _7: u32;
let _8: u16;
let _9: [i64; 8];
let _10: Adt56;
let _11: [u16; 1];
let _12: char;
let _13: [usize; 2];
let _14: Adt48;
let _15: isize;
let _16: i32;
let _17: *const usize;
let _18: ([u16; 1], f64, u8);
let _19: bool;
let _20: char;
let _21: u128;
let _22: f32;
let _23: u64;
let _24: [u8; 4];
let _25: isize;
let _26: Adt41;
let _27: i8;
let _28: u32;
let _29: ();
let _30: ();
{
_4 = [30976094378779336520306220856809542827_u128,10679404047182610838937116854943953051_u128];
RET = 23049_i16 as i128;
_2 = _1 as i128;
_4 = [10424828115016337170834220291540587346_u128,212650535893029292546120513508514728068_u128];
RET = 8824_i16 as i128;
_4 = [129381229005538847354342614360341210407_u128,134439780704008037485111140066385361664_u128];
_3 = [57012322859561913124487911186131112665_u128,123138817229535927999860692130096546291_u128];
RET = !_2;
_3 = [206566385263907597278108534827805496310_u128,203499898736583351437984417053883779427_u128];
_2 = RET | RET;
_5.0 = [(-6988156924608028545_i64),(-7277999979521522521_i64),(-7578521160691882252_i64),59387364838275192_i64,(-3045564645172199319_i64),(-1384082271492526706_i64),8871980333199832960_i64,(-8415483133706673500_i64)];
_6.1 = -(-7_i8);
_4 = _3;
_4 = _3;
_5.0 = [(-8404694427824062333_i64),2793626468945747611_i64,7610516620135268524_i64,(-8998537884924564258_i64),3947999866498255378_i64,(-4004433089059253636_i64),(-4354403114170529575_i64),6629449132029470162_i64];
_1 = '\u{d938a}';
RET = _2 - _2;
_2 = 58053_u16 as i128;
RET = _2 ^ _2;
_4 = [15034249233474315627721441827739809768_u128,258259092092764025596571312870731076413_u128];
_7 = 1450608955_u32;
_8 = 32582_u16;
match _8 {
0 => bb1,
32582 => bb3,
_ => bb2
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
_5.0 = [(-2458764812879932854_i64),1314650298293718106_i64,(-1484835811000662221_i64),(-5254399575630974195_i64),9085420501459686363_i64,(-407742431278626465_i64),(-5584485590961268333_i64),(-7579439165390434531_i64)];
_5.0 = [894713823138043635_i64,490120886373955889_i64,(-1216422261069841377_i64),(-8479024958461791041_i64),(-8714357264649813655_i64),8366285242260439811_i64,5506208156559951033_i64,4268087851703095808_i64];
_6.1 = -126_i8;
_8 = 44547_u16 | 30245_u16;
_7 = 1509629605_u32;
_3 = _4;
_5.0 = [(-1833519953739411591_i64),7608089978271916354_i64,6338371352068207441_i64,(-3788818023668787049_i64),6344887225484831973_i64,(-5725994675034772178_i64),(-5119101658734102052_i64),(-2188576917766166260_i64)];
_3 = _4;
_7 = _6.1 as u32;
_9 = [6898865866072815920_i64,(-1413041065181251638_i64),1615657996034319397_i64,(-8414432595820308693_i64),5763099488057778377_i64,(-4360691479271586532_i64),8608800095630650884_i64,9211593863723780988_i64];
_9 = _5.0;
_6.1 = (-48_i8) ^ 78_i8;
_1 = '\u{1e927}';
_3 = [300810534501122259598583402710056992275_u128,3305379145378964576618590790685565970_u128];
_5.0 = [6169646231997146679_i64,5943292627030859820_i64,254086940819983103_i64,3591564763858667726_i64,(-3456310541316315326_i64),(-8103634760139739806_i64),(-3561193060399024721_i64),1682247318299834670_i64];
_1 = '\u{2e435}';
_5 = (_9,);
_6.1 = 51_i8 ^ (-56_i8);
_2 = !RET;
_5 = (_9,);
_9 = [1364441308036354298_i64,1162360640538015900_i64,(-7556430606153530393_i64),(-7894927710926770940_i64),(-3167626438444989441_i64),3614201625490155996_i64,7900553956780180766_i64,(-4139372329371768669_i64)];
_4 = _3;
_1 = '\u{8f4b9}';
RET = _2 << _8;
_8 = 42642_u16 | 11636_u16;
RET = _8 as i128;
Call(_12 = fn3(_5.0, _2, RET, _1, _9, _6.1, _5.0, _2, _3, _5.0, _5, _9, _2, _8, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = _12 as u32;
_1 = _12;
_4 = [25176528509772436472002427952047325583_u128,79181563750268929473061586884863952991_u128];
RET = _2;
_9 = _5.0;
_6.1 = 4_i8 & (-73_i8);
_1 = _12;
_8 = 26195_u16;
_7 = !1236534742_u32;
match _8 {
26195 => bb5,
_ => bb3
}
}
bb5 = {
_4 = [66276691205980128923380094788060065495_u128,311335364111711512812496832978594423128_u128];
_4 = _3;
_2 = RET;
_14.fld0 = core::ptr::addr_of_mut!(_4);
_12 = _1;
_14.fld1.2 = 77_u8;
_14.fld1.3 = (-9223372036854775808_isize) << _14.fld1.2;
_14.fld1.0 = _7;
_14.fld0 = core::ptr::addr_of_mut!(_3);
_14.fld1.1 = [7334661752943410519_i64,(-2586281615406771147_i64),1963108491972184447_i64,(-52401303334359283_i64),(-4226818300371582421_i64),(-5343099162524443538_i64),3773142459130312412_i64,3038169000504614116_i64];
_5.0 = [(-1672446030660941379_i64),855752214377560140_i64,4813310988496935646_i64,7309516642470668088_i64,(-4382011884250759524_i64),5734363524515265770_i64,(-8846857319417291010_i64),4512064382031135665_i64];
Goto(bb6)
}
bb6 = {
_4 = _3;
_8 = 30541_u16;
_14.fld0 = core::ptr::addr_of_mut!(_4);
_14.fld1.2 = 91_u8 & 202_u8;
_7 = !_14.fld1.0;
_13 = [3_usize,5_usize];
_14.fld2 = _4;
_14.fld0 = core::ptr::addr_of_mut!(_4);
_14.fld1.1 = [3639173839173596308_i64,(-2969731774277263765_i64),(-2187733457819137341_i64),(-6232642377112039293_i64),1881081493013563788_i64,(-829053533358243228_i64),1763663127800948801_i64,(-1107402907410745924_i64)];
Goto(bb7)
}
bb7 = {
_1 = _12;
_7 = _14.fld1.0 - _14.fld1.0;
_5.0 = [(-6981275714765465644_i64),(-937159619686640255_i64),3636748829968748112_i64,1226328763476291018_i64,(-951750009718553455_i64),(-8126528087730510001_i64),6406495665093099942_i64,782139264105715580_i64];
_19 = !true;
_8 = 26084_u16;
_8 = 314321992801178037245267389509613126591_u128 as u16;
_22 = 12075_i16 as f32;
_14.fld1.1 = [3926317895358271747_i64,(-4234275277178931518_i64),5549598455731895763_i64,(-1637215368339041726_i64),7580156476352912608_i64,4245218214873267942_i64,(-2434643360986451032_i64),4883080166270070542_i64];
_23 = 16976088446549384768_u64 & 9565589028868586059_u64;
_6.1 = (-80_i8) - (-51_i8);
_21 = !109858139844200923064117687939038065830_u128;
_18.2 = _14.fld1.3 as u8;
_18.1 = _8 as f64;
_19 = true;
_25 = _21 as isize;
_18.2 = _1 as u8;
_9 = _5.0;
_14.fld1.3 = _25;
RET = -_2;
Call(_16 = core::intrinsics::bswap((-747693342_i32)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14.fld1.2 = !_18.2;
_18.0 = [_8];
_11 = _18.0;
_2 = -RET;
_9 = _5.0;
_6.1 = (-39_i8);
_2 = _6.1 as i128;
_20 = _12;
_2 = _1 as i128;
Goto(bb9)
}
bb9 = {
_9 = [(-1775782427087119178_i64),(-306894737921827105_i64),(-4195853996131761695_i64),5323269151345165483_i64,3319680598331432447_i64,4742481492427017553_i64,(-1396238690996401619_i64),(-8012865289619567741_i64)];
_4 = [_21,_21];
_23 = 1837216578822587352_u64;
_22 = 21322091_i32 as f32;
Goto(bb10)
}
bb10 = {
_15 = -_14.fld1.3;
_3 = [_21,_21];
_14.fld1.3 = _8 as isize;
_15 = _14.fld1.3;
_24 = [_18.2,_14.fld1.2,_18.2,_14.fld1.2];
_14.fld2 = _3;
_4 = [_21,_21];
Goto(bb11)
}
bb11 = {
_24 = [_18.2,_18.2,_18.2,_14.fld1.2];
_15 = _25 + _14.fld1.3;
_27 = _6.1;
_21 = _22 as u128;
_1 = _20;
_8 = 21083_u16 >> _15;
_4 = [_21,_21];
_14.fld1.2 = !_18.2;
_11 = [_8];
_13 = [12604676636238765589_usize,7963947611824025997_usize];
match _6.1 {
0 => bb12,
340282366920938463463374607431768211417 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_7 = _12 as u32;
_1 = _12;
_4 = [25176528509772436472002427952047325583_u128,79181563750268929473061586884863952991_u128];
RET = _2;
_9 = _5.0;
_6.1 = 4_i8 & (-73_i8);
_1 = _12;
_8 = 26195_u16;
_7 = !1236534742_u32;
match _8 {
26195 => bb5,
_ => bb3
}
}
bb14 = {
_25 = _15 ^ _15;
_15 = _8 as isize;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 9_usize, Move(_9), 16_usize, Move(_16), 24_usize, Move(_24), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 19_usize, Move(_19), 13_usize, Move(_13), 20_usize, Move(_20), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(2_usize, 1_usize, Move(_1), 5_usize, Move(_5), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i64; 8],mut _2: i128,mut _3: i128,mut _4: char,mut _5: [i64; 8],mut _6: i8,mut _7: [i64; 8],mut _8: i128,mut _9: [u128; 2],mut _10: [i64; 8],mut _11: ([i64; 8],),mut _12: [i64; 8],mut _13: i128,mut _14: u16,mut _15: ([i64; 8],),mut _16: ([i64; 8],)) -> char {
mir! {
type RET = char;
let _17: [u16; 1];
let _18: Adt48;
let _19: char;
let _20: f64;
let _21: isize;
let _22: [u8; 4];
let _23: u16;
let _24: [u16; 1];
let _25: f64;
let _26: ([u16; 1], f64, u8);
let _27: isize;
let _28: usize;
let _29: [u8; 4];
let _30: [usize; 2];
let _31: [u16; 1];
let _32: u128;
let _33: f32;
let _34: ((*mut &'static i64,), i8);
let _35: isize;
let _36: ([u16; 1], f64, u8);
let _37: isize;
let _38: [u64; 5];
let _39: f32;
let _40: bool;
let _41: f32;
let _42: char;
let _43: [i16; 4];
let _44: char;
let _45: (char, *const u128, u32, u16);
let _46: ((*mut &'static i64,), i8);
let _47: ();
let _48: ();
{
_4 = '\u{c54ea}';
_15.0 = [(-57436230559847271_i64),(-1457150467327914940_i64),(-3312395660659035786_i64),(-6238791003428489094_i64),(-8142397841419063392_i64),411867328336212458_i64,(-8250046623837404002_i64),426707110804600871_i64];
_17 = [_14];
_3 = _13;
_16 = (_11.0,);
_15.0 = [(-2429185252546602275_i64),(-6167078848957135748_i64),(-3520358700396325628_i64),(-5166805472355760358_i64),2783565657729823677_i64,7932072734407312595_i64,3156599373393338366_i64,6327297535887321406_i64];
_11 = (_15.0,);
_8 = -_2;
_2 = _13 * _8;
_4 = '\u{db58a}';
Call(_13 = fn4(_16, _15, _15.0, _15, _8, _7, _15, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _4;
RET = _4;
_12 = _10;
_5 = _16.0;
_14 = 4111_u16;
RET = _4;
_12 = [(-8126292341885572777_i64),4823773161881149321_i64,4029894868748317103_i64,(-5947410376723882722_i64),(-349491976702149786_i64),(-6852325143031688130_i64),(-1668091173881912840_i64),(-3678338410389947483_i64)];
_11.0 = _1;
_13 = _2;
RET = _4;
_14 = 47545_u16;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_14 = 52340_u16;
_5 = _16.0;
_9 = [197667219405167414495018224221922603355_u128,213409211544369533052990085547755109067_u128];
_18.fld1.1 = [8026846188411041843_i64,1427883315973834965_i64,(-260385913194583418_i64),3215485849193664529_i64,(-5703263216811168226_i64),5492430739155496847_i64,(-3347255860308656100_i64),9036596830527320008_i64];
_18.fld1.3 = !(-40_isize);
_11 = _16;
_18.fld2 = [335053715264901917507468933419599070517_u128,78377601995722927245524154182150122540_u128];
_18.fld1.0 = !2257299799_u32;
_18.fld1.1 = [(-6321617631625631125_i64),(-8661687217750814451_i64),4556822867428054645_i64,(-9189584664870200710_i64),5734922922773831527_i64,8181442035629509259_i64,7579449943249501408_i64,5212337373036228134_i64];
_18.fld1.3 = _18.fld1.0 as isize;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_6 = 26_i8;
Goto(bb2)
}
bb2 = {
_4 = RET;
_2 = !_13;
_17 = [_14];
RET = _4;
Goto(bb3)
}
bb3 = {
_13 = _2;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_11 = (_5,);
_18.fld1.1 = [(-2795114459460598843_i64),829107425206479180_i64,7739806673934280117_i64,8676280905846677739_i64,(-8872029352737941915_i64),(-2127751106234129015_i64),(-32191378843652858_i64),(-2240041884967686035_i64)];
_18.fld1.1 = _10;
_15 = _16;
_4 = RET;
_18.fld1.0 = 24365112279649540700406160380971670852_u128 as u32;
_21 = !_18.fld1.3;
_4 = RET;
_18.fld1.1 = [(-6826715014786613225_i64),(-3221340046697071525_i64),6281280406486731544_i64,1660679753848693564_i64,5428482009915022846_i64,(-1582360918913482153_i64),282570996251480004_i64,(-1823684818692852184_i64)];
_1 = _11.0;
_15.0 = [(-4365602473125522980_i64),2444314337438396029_i64,3867036046122317388_i64,1909933795024341159_i64,2492292201913075790_i64,(-4898562943740199952_i64),(-7656842239422387837_i64),(-2477572198193858299_i64)];
_11 = (_5,);
_18.fld1.0 = !108599174_u32;
_13 = 325569786717899774759442774575497243143_u128 as i128;
Goto(bb4)
}
bb4 = {
_7 = _15.0;
_1 = [2648253282978239147_i64,3520400390875056510_i64,(-8956810791867450633_i64),(-9047008840211669616_i64),4978999316712253375_i64,(-4007786837607190743_i64),4010610466653502309_i64,7666463516313002540_i64];
_6 = 1886990106863371820_u64 as i8;
_23 = !_14;
_19 = _4;
_10 = _18.fld1.1;
_16 = (_10,);
_10 = [(-3711606975292248878_i64),8465201500457187711_i64,8066421300764981963_i64,1579835302591951361_i64,8366006486554809459_i64,(-7454626094968108041_i64),954788230162773410_i64,5824485685233751113_i64];
_1 = [5210111518095512506_i64,(-7531504565734709287_i64),371676682659429965_i64,3013139128096600418_i64,8036792767958693776_i64,7385858550774502028_i64,2899131490697944525_i64,3753882906216156487_i64];
_25 = 7782192495421933496_i64 as f64;
_24 = [_14];
_21 = _18.fld1.3 >> _2;
RET = _19;
_18.fld1.2 = !7_u8;
_26.2 = !_18.fld1.2;
_20 = _21 as f64;
_22 = [_26.2,_18.fld1.2,_26.2,_18.fld1.2];
_16 = _15;
_26.0 = [_23];
RET = _19;
_26.2 = _18.fld1.2 >> _21;
_26 = (_24, _20, _18.fld1.2);
RET = _19;
_27 = !_21;
_14 = _23;
_19 = RET;
_14 = _23;
Goto(bb5)
}
bb5 = {
_17 = [_14];
_28 = !5_usize;
_18.fld2 = [119761316164031391380042152130304389175_u128,96487804055204868477410902558006359528_u128];
_21 = -_27;
_23 = _14;
RET = _19;
_26.2 = !_18.fld1.2;
_21 = -_27;
_21 = _18.fld1.3;
_17 = _24;
_13 = -_3;
Goto(bb6)
}
bb6 = {
_17 = [_14];
_18.fld0 = core::ptr::addr_of_mut!(_9);
_11 = _15;
RET = _19;
_7 = [7882868055812540948_i64,(-6441973258886393906_i64),4138227020239262702_i64,8856381988665699025_i64,1720739167594207777_i64,3525060810584349116_i64,4389343425158891688_i64,(-748089953324873661_i64)];
_27 = !_18.fld1.3;
_31 = _17;
RET = _4;
Call(_18.fld1.3 = core::intrinsics::bswap(_21), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_25 = _28 as f64;
_15.0 = [1885409398540121851_i64,4369023796636520001_i64,(-3743564321876124806_i64),7631096221850870340_i64,(-6384186253628160664_i64),9117445223105219732_i64,(-3499832585045195711_i64),(-8844277813437425630_i64)];
_19 = _4;
_13 = !_2;
_26.1 = -_20;
_18.fld1.1 = _11.0;
_26.0 = [_23];
Goto(bb8)
}
bb8 = {
RET = _19;
_2 = _13;
_3 = _28 as i128;
_10 = _15.0;
_18.fld1.0 = 4078854021_u32;
_18.fld1.1 = [8941601109645786816_i64,6566832525922731513_i64,(-7333064717619058477_i64),4519523334211650716_i64,8913353315146230220_i64,(-3349629929185916908_i64),1357708168789254028_i64,(-8414730783361631626_i64)];
_32 = 192449063296643230880475970586020614612_u128 + 158034690987351466313004848767254453297_u128;
_18.fld0 = core::ptr::addr_of_mut!(_9);
_14 = _23 | _23;
_16 = (_7,);
_21 = -_27;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_15.0 = [5293232636492590449_i64,(-3770233992696358554_i64),(-686971130089403419_i64),(-1050417523091824527_i64),(-4189387004300046359_i64),8494586396783032772_i64,(-1047422968421539607_i64),(-2693284354188212048_i64)];
_24 = _26.0;
_14 = _23 | _23;
_36 = (_17, _20, _18.fld1.2);
_36.2 = _26.2 >> _2;
_16.0 = [(-2095188195620809869_i64),(-6220189649280224544_i64),(-4839797019003937236_i64),(-8069139237948241178_i64),(-3919243052123232440_i64),1698718508518204244_i64,(-2064831913565438312_i64),540375937057590436_i64];
_35 = _21 * _27;
_18.fld1.2 = !_26.2;
RET = _4;
_33 = 24729_i16 as f32;
Goto(bb9)
}
bb9 = {
_26.1 = -_36.1;
_18.fld1.3 = -_21;
_30 = [_28,_28];
_23 = _14;
_17 = _26.0;
_16.0 = [5635147984753027820_i64,875551343001149286_i64,2583510116845727185_i64,(-8792576565195539368_i64),8672248461666198118_i64,(-4129981687029701804_i64),7587646089354798426_i64,4796561303146498957_i64];
_30 = [_28,_28];
_41 = _33 - _33;
_36.0 = [_14];
_11 = (_10,);
_23 = _14 & _14;
_26.1 = _41 as f64;
_40 = !false;
_36.1 = _20 * _20;
_34.1 = 6693783521191423150_u64 as i8;
_9 = [_32,_32];
_9 = [_32,_32];
_29 = _22;
_37 = _21;
_40 = true;
RET = _4;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
RET = _19;
_19 = _4;
Goto(bb10)
}
bb10 = {
_21 = _35;
match _18.fld1.0 {
0 => bb4,
1 => bb6,
2 => bb8,
3 => bb11,
4 => bb12,
5 => bb13,
4078854021 => bb15,
_ => bb14
}
}
bb11 = {
_17 = [_14];
_28 = !5_usize;
_18.fld2 = [119761316164031391380042152130304389175_u128,96487804055204868477410902558006359528_u128];
_21 = -_27;
_23 = _14;
RET = _19;
_26.2 = !_18.fld1.2;
_21 = -_27;
_21 = _18.fld1.3;
_17 = _24;
_13 = -_3;
Goto(bb6)
}
bb12 = {
_4 = RET;
_2 = !_13;
_17 = [_14];
RET = _4;
Goto(bb3)
}
bb13 = {
_13 = _2;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_11 = (_5,);
_18.fld1.1 = [(-2795114459460598843_i64),829107425206479180_i64,7739806673934280117_i64,8676280905846677739_i64,(-8872029352737941915_i64),(-2127751106234129015_i64),(-32191378843652858_i64),(-2240041884967686035_i64)];
_18.fld1.1 = _10;
_15 = _16;
_4 = RET;
_18.fld1.0 = 24365112279649540700406160380971670852_u128 as u32;
_21 = !_18.fld1.3;
_4 = RET;
_18.fld1.1 = [(-6826715014786613225_i64),(-3221340046697071525_i64),6281280406486731544_i64,1660679753848693564_i64,5428482009915022846_i64,(-1582360918913482153_i64),282570996251480004_i64,(-1823684818692852184_i64)];
_1 = _11.0;
_15.0 = [(-4365602473125522980_i64),2444314337438396029_i64,3867036046122317388_i64,1909933795024341159_i64,2492292201913075790_i64,(-4898562943740199952_i64),(-7656842239422387837_i64),(-2477572198193858299_i64)];
_11 = (_5,);
_18.fld1.0 = !108599174_u32;
_13 = 325569786717899774759442774575497243143_u128 as i128;
Goto(bb4)
}
bb14 = {
RET = _4;
RET = _4;
_12 = _10;
_5 = _16.0;
_14 = 4111_u16;
RET = _4;
_12 = [(-8126292341885572777_i64),4823773161881149321_i64,4029894868748317103_i64,(-5947410376723882722_i64),(-349491976702149786_i64),(-6852325143031688130_i64),(-1668091173881912840_i64),(-3678338410389947483_i64)];
_11.0 = _1;
_13 = _2;
RET = _4;
_14 = 47545_u16;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_14 = 52340_u16;
_5 = _16.0;
_9 = [197667219405167414495018224221922603355_u128,213409211544369533052990085547755109067_u128];
_18.fld1.1 = [8026846188411041843_i64,1427883315973834965_i64,(-260385913194583418_i64),3215485849193664529_i64,(-5703263216811168226_i64),5492430739155496847_i64,(-3347255860308656100_i64),9036596830527320008_i64];
_18.fld1.3 = !(-40_isize);
_11 = _16;
_18.fld2 = [335053715264901917507468933419599070517_u128,78377601995722927245524154182150122540_u128];
_18.fld1.0 = !2257299799_u32;
_18.fld1.1 = [(-6321617631625631125_i64),(-8661687217750814451_i64),4556822867428054645_i64,(-9189584664870200710_i64),5734922922773831527_i64,8181442035629509259_i64,7579449943249501408_i64,5212337373036228134_i64];
_18.fld1.3 = _18.fld1.0 as isize;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_6 = 26_i8;
Goto(bb2)
}
bb15 = {
_19 = _4;
_14 = _32 as u16;
_31 = [_14];
_38 = [7843822692783610932_u64,1509547012275347903_u64,9589944052276103711_u64,8633842707748923347_u64,17083570896666398009_u64];
_36.0 = [_14];
_45.1 = core::ptr::addr_of!(_32);
_14 = (-3176562309898510123_i64) as u16;
_1 = [(-2874464470021915403_i64),8585784768954797080_i64,(-4854820345094189044_i64),1681698321227613753_i64,(-4500171510205345058_i64),5728984022440301420_i64,2643526424741692499_i64,5965911424960419501_i64];
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(3_usize, 38_usize, Move(_38), 12_usize, Move(_12), 3_usize, Move(_3), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(3_usize, 8_usize, Move(_8), 13_usize, Move(_13), 4_usize, Move(_4), 40_usize, Move(_40)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(3_usize, 30_usize, Move(_30), 21_usize, Move(_21), 24_usize, Move(_24), 32_usize, Move(_32)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_47 = dump_var(3_usize, 23_usize, Move(_23), 17_usize, Move(_17), 35_usize, Move(_35), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: ([i64; 8],),mut _2: ([i64; 8],),mut _3: [i64; 8],mut _4: ([i64; 8],),mut _5: i128,mut _6: [i64; 8],mut _7: ([i64; 8],),mut _8: [i64; 8]) -> i128 {
mir! {
type RET = i128;
let _9: bool;
let _10: *const u32;
let _11: ([i64; 8],);
let _12: Adt57;
let _13: [u8; 4];
let _14: ([i64; 8],);
let _15: char;
let _16: [i16; 4];
let _17: f64;
let _18: i128;
let _19: [u16; 1];
let _20: [i64; 8];
let _21: Adt52;
let _22: i128;
let _23: *const usize;
let _24: Adt43;
let _25: [i64; 8];
let _26: f32;
let _27: f32;
let _28: ([i64; 8],);
let _29: [i16; 4];
let _30: i128;
let _31: isize;
let _32: [i16; 4];
let _33: Adt50;
let _34: i32;
let _35: Adt52;
let _36: ([i64; 8],);
let _37: [u64; 5];
let _38: u32;
let _39: f64;
let _40: Adt45;
let _41: usize;
let _42: i128;
let _43: [u128; 2];
let _44: f64;
let _45: ();
let _46: ();
{
RET = _5 * _5;
_5 = (-1541_i16) as i128;
_5 = RET + RET;
_7.0 = _6;
_8 = _6;
_1 = _4;
_9 = true ^ true;
_2.0 = _1.0;
RET = !_5;
_1 = (_3,);
_11 = _2;
_2 = (_6,);
_9 = RET != RET;
_2.0 = [(-3602235122981375907_i64),(-8811258438055309194_i64),(-8029319699737715309_i64),(-7399534942235884199_i64),2163023755367072455_i64,1303371263334127563_i64,6460859809696923337_i64,(-1626776475771830639_i64)];
_1 = (_3,);
_2 = (_1.0,);
_4 = (_8,);
Goto(bb1)
}
bb1 = {
_4.0 = [7222753425356270123_i64,1807031191850288744_i64,1211535914299088037_i64,(-2859320690120855236_i64),(-9071575405009584977_i64),(-8715304180842589738_i64),(-3257195917574486499_i64),7444677628986290280_i64];
_9 = !false;
_1.0 = [(-4407434240022631829_i64),(-4204815828931752784_i64),4420656111997598125_i64,835847013375953764_i64,(-8105747645214796679_i64),8391087239820062883_i64,(-132528096114034795_i64),(-4589107683213744215_i64)];
_2 = (_1.0,);
Call(_2 = fn5(_1, _11.0, _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = [176_u8,94_u8,216_u8,218_u8];
_5 = RET;
_14 = _7;
_1.0 = [5338029098851054807_i64,(-7766001372192996683_i64),(-7827416966108887739_i64),(-3672719963156237552_i64),(-3416451716151686897_i64),7982168390821593022_i64,(-2909034114450282650_i64),(-2300360406413296970_i64)];
_7 = (_14.0,);
_7 = (_1.0,);
_6 = _7.0;
_4.0 = [117365679280414961_i64,6631538423717051197_i64,(-9117379672530570859_i64),8752435021955360347_i64,2902624516535653165_i64,(-7738427100903737876_i64),1706846968306381723_i64,6214380232984046518_i64];
_7.0 = _3;
_4 = (_7.0,);
_18 = _5;
_5 = -_18;
_18 = RET;
Goto(bb3)
}
bb3 = {
_11.0 = _2.0;
_16 = [(-11411_i16),(-6616_i16),(-12749_i16),(-20851_i16)];
_2 = (_14.0,);
Goto(bb4)
}
bb4 = {
RET = -_5;
Goto(bb5)
}
bb5 = {
_7 = _14;
_11.0 = _6;
_3 = [(-4458009740269629196_i64),2008682018387164111_i64,6330240672343172450_i64,(-8890469598874870512_i64),7792001511398184425_i64,(-2831207773335796029_i64),6493900451555184784_i64,3713218029405467439_i64];
_5 = _18;
_2.0 = [7794844733826938777_i64,5686787618646124060_i64,(-6319714025377613504_i64),(-4821379230605748627_i64),(-5068196316145547521_i64),(-6155358876204869225_i64),(-4431807845795000898_i64),4839046344002723083_i64];
Goto(bb6)
}
bb6 = {
_9 = !false;
_9 = false;
_15 = '\u{29460}';
_7.0 = [2367759149541567625_i64,(-3185754901545959880_i64),5094109537053770954_i64,5301582187037316093_i64,(-468712800098064054_i64),4094670648718326085_i64,(-6217095513078059443_i64),3130215008781998832_i64];
_4.0 = [6768543821794251439_i64,8889995985252359680_i64,7147568290472821095_i64,(-4681482191059401866_i64),(-2375888498929351187_i64),2090093831085900725_i64,6475528856945684705_i64,442179935705864246_i64];
_4 = (_6,);
_2.0 = _14.0;
_1 = (_6,);
_19 = [60320_u16];
_7.0 = _14.0;
_2 = (_7.0,);
_5 = !RET;
_18 = RET;
_7 = _2;
_9 = !false;
Goto(bb7)
}
bb7 = {
_17 = 13304_u16 as f64;
_7.0 = _6;
_19 = [7762_u16];
RET = -_18;
_5 = _17 as i128;
_6 = [(-6085565566211307270_i64),(-1179040946538566579_i64),(-2253284120578470674_i64),(-1673071322914526181_i64),2742166803693898951_i64,994097986481174720_i64,(-1851089098441474598_i64),8684062615111498417_i64];
_20 = [1751090291110397011_i64,(-5257465794353425385_i64),6537119585500890116_i64,(-7352185858796482154_i64),(-1002692178436012633_i64),6961675039968095226_i64,3385800788556990927_i64,8115223926563308789_i64];
_6 = [3181027462448361838_i64,2918151152902886734_i64,8140166573282784983_i64,(-2598036180430264344_i64),1395088254511799943_i64,8454336409986916895_i64,8875440198393218030_i64,(-4717015795156438579_i64)];
_14 = (_20,);
_1 = (_11.0,);
_11.0 = [4514778640886343372_i64,4767050246076548073_i64,(-4175941438264758456_i64),(-7901474678589416558_i64),(-7190717282744240980_i64),(-1005933292531442487_i64),5441415787730562547_i64,240200188793694480_i64];
_10 = core::ptr::addr_of!(_24.fld3.2);
_24.fld3.0 = _15;
_2.0 = _14.0;
_11.0 = [8022612033597600409_i64,8596228364024665263_i64,5778152547635893701_i64,4331375683976747730_i64,(-7373501455058910427_i64),1059077974225604350_i64,(-65206995674215295_i64),(-2283151567864011321_i64)];
_7 = (_11.0,);
Call(_24.fld3.3 = core::intrinsics::transmute(_19), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_26 = 26820_i16 as f32;
_3 = _11.0;
_22 = _18 * _18;
_26 = 3567256619_u32 as f32;
_22 = 201_u8 as i128;
_14.0 = [5493547053269277013_i64,5963593125951183091_i64,(-2585183044845754227_i64),(-6697020291682178011_i64),(-3174825971177678735_i64),3081097472217782860_i64,689417886722674140_i64,(-1502603413436456898_i64)];
_14 = (_3,);
_10 = core::ptr::addr_of!(_24.fld3.2);
_9 = false | false;
_11.0 = [2749700230694519337_i64,1696125766322969868_i64,1471963002216183631_i64,(-3374673125928215863_i64),(-6697356714874025202_i64),1825554571020213421_i64,3199554610934928617_i64,(-7529711868957648304_i64)];
_7.0 = [3080199822352795001_i64,(-3574302675203875663_i64),3116462052345589799_i64,7418007311271760854_i64,(-1218275495917571235_i64),680201689205907692_i64,2533034514279461876_i64,4154020815502075586_i64];
_15 = _24.fld3.0;
_24.fld1 = -(-23340_i16);
Goto(bb9)
}
bb9 = {
_17 = 13264373685005400066_usize as f64;
_2.0 = [1042764828649111902_i64,(-8674965810134795586_i64),(-1631359220093443536_i64),1617722608030543222_i64,3106408345706314656_i64,(-1123431273386231909_i64),(-5694193904764547145_i64),(-3034914679458506677_i64)];
_24.fld3.3 = !50554_u16;
_24.fld3.3 = 60579_u16 + 51175_u16;
_4.0 = [7309557238775346633_i64,1485545379973272573_i64,2854222416194115922_i64,(-6241270884769575990_i64),(-5206634476868950100_i64),8342213186286861727_i64,5990928037353694922_i64,1600449626327981663_i64];
_5 = (-119_i8) as i128;
_11.0 = [(-2149173426948730463_i64),(-6492395181438197892_i64),9136901614344036198_i64,2930237026086943780_i64,5969269961629743688_i64,(-3952531082310216737_i64),(-8557434357995502971_i64),1837176395478438613_i64];
_24.fld3.3 = 16788_u16 & 51401_u16;
Goto(bb10)
}
bb10 = {
_13 = [219_u8,1_u8,27_u8,100_u8];
_27 = _18 as f32;
_24.fld3.3 = 168_u8 as u16;
_16 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
_25 = [5562629831155294406_i64,3428002977907723775_i64,(-273961886407061542_i64),(-879979075273738999_i64),1872286432759746403_i64,(-3647247284732641341_i64),(-7327438409454145032_i64),3532819459546871433_i64];
_29 = _16;
Call(_14.0 = core::intrinsics::transmute(_4.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_32 = _29;
_17 = 325786720937243976857579039463885967733_u128 as f64;
_1 = (_20,);
_12 = Adt57::Variant3 { fld0: _27,fld1: _4.0 };
_24.fld3.2 = !2628120423_u32;
_4.0 = [(-5147003934638069942_i64),(-977867275678325015_i64),221560463362328931_i64,4214394035152873155_i64,1331525991943705645_i64,(-1339149786926319560_i64),1792729216037915879_i64,(-3055695649251561800_i64)];
_20 = [(-1692349373191347956_i64),(-865532142068939350_i64),8432165700708862157_i64,(-2712662582495379480_i64),(-8920003072473278169_i64),8406962758423989303_i64,3624984117849704737_i64,(-5957573108266817510_i64)];
_32 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
_14.0 = [1294791482998868143_i64,(-3803924255521628907_i64),(-1108208197773501924_i64),2881594162770398954_i64,(-7534846785676130352_i64),4800381066780650850_i64,6157082068367992489_i64,4155423848496102647_i64];
_10 = core::ptr::addr_of!((*_10));
_16 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
_11.0 = [(-6225463464430797738_i64),(-4026661349724025555_i64),7128339489777085927_i64,(-8134866703569805843_i64),(-4758328849113041156_i64),(-6604996482571729132_i64),6978861658318394477_i64,(-3473443820425236390_i64)];
_22 = _18;
_16 = _32;
_32 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
_29 = _16;
_14.0 = [(-5292860529622537999_i64),1699962737776456687_i64,6157275195800855009_i64,7456197272802252220_i64,4315946235655145089_i64,(-7531784686707801351_i64),(-3936026479354092157_i64),(-6318511046237709550_i64)];
_31 = _24.fld3.0 as isize;
_11.0 = _7.0;
_10 = core::ptr::addr_of!(_24.fld3.2);
_24.fld2 = _16;
(*_10) = 3593548965_u32;
Goto(bb12)
}
bb12 = {
SetDiscriminant(_12, 1);
_7 = (_6,);
place!(Field::<Adt43>(Variant(_12, 1), 1)).fld2 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
place!(Field::<Adt43>(Variant(_12, 1), 1)).fld3.0 = _15;
_26 = _27;
_24.fld3.2 = _17 as u32;
_24.fld3.2 = _24.fld3.0 as u32;
_13 = [1_u8,114_u8,90_u8,191_u8];
_34 = !(-2030252586_i32);
_8 = _11.0;
_2.0 = [(-6754856167685205204_i64),(-1394186713950140196_i64),8556085968196450311_i64,3278388171236374008_i64,5772347426208747700_i64,(-5925442071861198790_i64),3843809184301394268_i64,7079569081413907103_i64];
place!(Field::<Adt43>(Variant(_12, 1), 1)).fld0 = _19;
_36 = _11;
_7.0 = [9088516300519044068_i64,8741007946708716312_i64,(-4190862729090780459_i64),6509242285994091957_i64,(-8768668107957469137_i64),5492748037356045819_i64,3617518638971055236_i64,336019533005864651_i64];
place!(Field::<Adt43>(Variant(_12, 1), 1)).fld3.2 = !(*_10);
_30 = _18 & _22;
_28.0 = [258083077889227084_i64,(-8690133376284334008_i64),7273391793481317967_i64,(-4213252927838817762_i64),(-2531968845165529004_i64),6861459172439898523_i64,(-4845278153006004929_i64),(-2878276122603779511_i64)];
_2 = (_3,);
_24.fld3.0 = Field::<Adt43>(Variant(_12, 1), 1).fld3.0;
_33.fld2 = _13;
_3 = _28.0;
_30 = _22;
_8 = _6;
Goto(bb13)
}
bb13 = {
(*_10) = !Field::<Adt43>(Variant(_12, 1), 1).fld3.2;
place!(Field::<Adt43>(Variant(_12, 1), 1)).fld3.0 = _15;
_24.fld3.3 = !38142_u16;
_18 = _30 & RET;
place!(Field::<bool>(Variant(_12, 1), 0)) = _9;
_7 = (_3,);
_6 = _8;
_38 = (*_10) << (*_10);
_17 = 13966920325371659298_usize as f64;
_10 = core::ptr::addr_of!((*_10));
_24.fld3.2 = _31 as u32;
_24.fld3.3 = 18476_u16 << _30;
_38 = (-8345692919506772837_i64) as u32;
_24.fld2 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
(*_10) = Field::<Adt43>(Variant(_12, 1), 1).fld3.2 + Field::<Adt43>(Variant(_12, 1), 1).fld3.2;
_29 = _32;
_36.0 = [(-8487164119628700582_i64),(-7613037850298937640_i64),(-6660587375565398348_i64),1274272528955549401_i64,6619798034568631747_i64,(-3561906661271365218_i64),3101341557149823908_i64,6115848938689869632_i64];
_18 = -_30;
_24.fld3.0 = Field::<Adt43>(Variant(_12, 1), 1).fld3.0;
_39 = _17;
_9 = RET >= _22;
place!(Field::<Adt43>(Variant(_12, 1), 1)).fld3.3 = 4_usize as u16;
_41 = !5_usize;
Goto(bb14)
}
bb14 = {
_34 = !(-1650110573_i32);
_43 = [263594789030548712120395469395504120633_u128,248592673074248787787653084053599690482_u128];
_29 = [_24.fld1,_24.fld1,_24.fld1,_24.fld1];
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(4_usize, 4_usize, Move(_4), 19_usize, Move(_19), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(4_usize, 8_usize, Move(_8), 43_usize, Move(_43), 38_usize, Move(_38), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(4_usize, 30_usize, Move(_30), 18_usize, Move(_18), 16_usize, Move(_16), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(4_usize, 36_usize, Move(_36), 28_usize, Move(_28), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: ([i64; 8],),mut _2: [i64; 8],mut _3: ([i64; 8],),mut _4: ([i64; 8],)) -> ([i64; 8],) {
mir! {
type RET = ([i64; 8],);
let _5: i8;
let _6: [i16; 4];
let _7: [i64; 8];
let _8: [i64; 8];
let _9: u16;
let _10: Adt56;
let _11: u8;
let _12: bool;
let _13: char;
let _14: isize;
let _15: u128;
let _16: u128;
let _17: [u8; 4];
let _18: f64;
let _19: ();
let _20: ();
{
_3.0 = [(-6287438330177644714_i64),7855173871533551970_i64,(-4979819887383656382_i64),(-3681618032348844152_i64),(-6341433262231323019_i64),(-9195887372459877735_i64),2450591692740463234_i64,(-9004527278349843449_i64)];
_2 = _4.0;
_4.0 = [927115009423577343_i64,(-191986873746453550_i64),(-4128162784237605616_i64),7457473716926382056_i64,3493165845584775344_i64,2003677030365918867_i64,5435118740207609369_i64,5680346190173684053_i64];
_1 = (_2,);
_1 = _4;
_4.0 = _3.0;
RET.0 = [(-2529440962592545777_i64),8223826493483170436_i64,467392689440343460_i64,8750357933834988752_i64,4396031433668582849_i64,(-6745884806204160395_i64),(-8782383472239618157_i64),2109542284982943996_i64];
_1 = _4;
_3.0 = [(-877857364057399877_i64),(-5576345339939958848_i64),(-5698658993222025787_i64),6935976338775294473_i64,(-1022645463352066994_i64),5570879769063965946_i64,(-1984039172046018846_i64),6191870294781558154_i64];
_4 = (_1.0,);
RET.0 = _2;
_3 = RET;
RET.0 = [(-99315724266911580_i64),(-1357120337436039955_i64),7993813700498314631_i64,(-4403552296241059177_i64),(-6472921172237785254_i64),6086500071497815452_i64,(-4590213367832664739_i64),(-7902623937151855402_i64)];
RET = (_2,);
_1.0 = [8078118160980647749_i64,1493739093981362462_i64,4732722257913790425_i64,7434229460161684356_i64,6143937383031033980_i64,(-5174565595589246732_i64),2682293618417270135_i64,4579134481694968653_i64];
_1 = (RET.0,);
_3 = (RET.0,);
_5 = !(-127_i8);
RET = (_3.0,);
_1 = (_2,);
_5 = 183_u8 as i8;
RET.0 = [(-3650280467888810387_i64),7229419302427279885_i64,7430991168085420489_i64,(-8177278986018787748_i64),(-8111221856166884319_i64),6329000912454193103_i64,2636829420774451008_i64,3550559769885195637_i64];
_3.0 = [3173937766868497369_i64,(-5406059234487647631_i64),951631661518732539_i64,700165875851456182_i64,(-6808321409126891401_i64),(-1710324234618903383_i64),(-670825984088566572_i64),(-131870529908413321_i64)];
_3.0 = [(-5244524111379637653_i64),1466381112184763821_i64,(-1085245987566143328_i64),780499048892324026_i64,(-4138471844245915982_i64),(-7747679270173309505_i64),(-3888528051210892262_i64),4038728184941923786_i64];
Goto(bb1)
}
bb1 = {
RET = (_1.0,);
_5 = (-13_i8) | (-128_i8);
_6 = [1640_i16,14135_i16,(-1321_i16),(-23147_i16)];
RET.0 = [(-5610481705200616439_i64),3282550299486420982_i64,8947151938506758964_i64,4383393806888048762_i64,(-4951057034621641642_i64),3705082752638043251_i64,1840907796076067322_i64,8762933618003675147_i64];
Call(RET.0 = fn6(_2, _4.0, _3, _1, _3.0, _3.0, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _3;
_9 = !12565_u16;
_1.0 = [(-1197614314415806650_i64),(-8358938261455021900_i64),6552036766165562098_i64,3544959368274903258_i64,7174047976043315380_i64,3253298045203046040_i64,(-7987830849269325631_i64),(-775725183771273718_i64)];
_7 = [(-5242673251646686338_i64),(-1298796148778783944_i64),(-1676988140951963260_i64),(-4977502011358221575_i64),7177386560069333612_i64,(-9142539325762930304_i64),254205864880040532_i64,2709665280240268609_i64];
RET.0 = [(-2596402179108653996_i64),1474173990466921934_i64,7232363983833177877_i64,(-5624298870784083589_i64),6065761624524641153_i64,(-1325845077439792639_i64),(-37471342889936798_i64),6921045958472012426_i64];
_1.0 = [(-2127056916018549439_i64),3098027981019215628_i64,(-497495967649386471_i64),(-1882256853048811663_i64),6235085065871229217_i64,4962454587716459201_i64,5426778782723583084_i64,(-1608485645104947836_i64)];
RET = (_3.0,);
_2 = [4671990311362143116_i64,3230184574808687956_i64,(-4382047693376836585_i64),7665580952640820336_i64,(-1923515485342480907_i64),5320748084409629367_i64,(-8410612878538365628_i64),(-4971656339824602692_i64)];
_5 = (-74_i8);
_6 = [19306_i16,23603_i16,23677_i16,(-2926_i16)];
_3 = (_7,);
_8 = [6356177889593841094_i64,4041861252279727063_i64,7838581903766315879_i64,1154262324096585683_i64,2515465944743478933_i64,1861789339542145090_i64,43335445594602589_i64,1214987263379019385_i64];
_1.0 = _7;
_3.0 = [(-6639184388979369558_i64),(-3815850913551824860_i64),4571974208057547558_i64,5834077736291722489_i64,(-8069938380888062059_i64),(-447068500036465750_i64),(-2167115890110688045_i64),4791574351604212134_i64];
RET = (_1.0,);
_2 = [(-2954430268624997601_i64),6145440977405915799_i64,(-6608597909641972194_i64),5127800557570295515_i64,(-8804759174684683831_i64),(-8137193271141255083_i64),(-7582770705635828915_i64),4606528596758565665_i64];
RET.0 = [(-9006802876428287783_i64),(-6185272462582010006_i64),6151407237233371378_i64,1496440870242379942_i64,(-6974271817187868858_i64),(-7604627251224987164_i64),(-1970753772139259412_i64),(-4775157344185273116_i64)];
RET = _4;
RET = (_7,);
RET = _3;
_4 = (_2,);
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768211382 => bb11,
_ => bb10
}
}
bb3 = {
RET = (_1.0,);
_5 = (-13_i8) | (-128_i8);
_6 = [1640_i16,14135_i16,(-1321_i16),(-23147_i16)];
RET.0 = [(-5610481705200616439_i64),3282550299486420982_i64,8947151938506758964_i64,4383393806888048762_i64,(-4951057034621641642_i64),3705082752638043251_i64,1840907796076067322_i64,8762933618003675147_i64];
Call(RET.0 = fn6(_2, _4.0, _3, _1, _3.0, _3.0, _2), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb11 = {
_9 = 10152_u16;
_4.0 = [(-5153632007868212327_i64),8767226218548298800_i64,7181952862611797863_i64,9168983785968278670_i64,6461060041638596572_i64,96038415683272039_i64,6724497150477348424_i64,(-8023904210178567390_i64)];
_8 = [(-1150839577950168348_i64),(-1420798466743956860_i64),8537731143381132452_i64,(-8021355820983286197_i64),4467710335200915688_i64,2465806330639579647_i64,(-3566600750500640678_i64),(-5954165476870037787_i64)];
_7 = _8;
_1 = RET;
_8 = _1.0;
_4.0 = [(-8054805498486723337_i64),8412209206095375884_i64,(-2778054117723198587_i64),7585955170643053922_i64,(-236987818224288116_i64),1902552735596409574_i64,(-1833707998470485890_i64),(-1304498225122791217_i64)];
_11 = _9 as u8;
_4 = (RET.0,);
RET = (_3.0,);
_3 = RET;
_9 = 63958_u16 + 40347_u16;
_11 = 2043748427_i32 as u8;
_4 = _3;
_11 = 88_u8 | 69_u8;
RET = (_2,);
_11 = 301773962457400605091612700805240981796_u128 as u8;
_11 = 129_u8;
_14 = 111_isize;
_11 = !95_u8;
RET.0 = _7;
RET.0 = _4.0;
_13 = '\u{8a59e}';
_13 = '\u{14218}';
_8 = RET.0;
_12 = true;
_1.0 = [5386803415475405256_i64,(-1406448600331178176_i64),(-3783795527206137835_i64),(-1961749649074570585_i64),(-2882726858409688794_i64),5484509492081811349_i64,(-8357728900927477342_i64),3151847809696927027_i64];
match _5 {
0 => bb12,
1 => bb13,
340282366920938463463374607431768211382 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
RET = _3;
_9 = !12565_u16;
_1.0 = [(-1197614314415806650_i64),(-8358938261455021900_i64),6552036766165562098_i64,3544959368274903258_i64,7174047976043315380_i64,3253298045203046040_i64,(-7987830849269325631_i64),(-775725183771273718_i64)];
_7 = [(-5242673251646686338_i64),(-1298796148778783944_i64),(-1676988140951963260_i64),(-4977502011358221575_i64),7177386560069333612_i64,(-9142539325762930304_i64),254205864880040532_i64,2709665280240268609_i64];
RET.0 = [(-2596402179108653996_i64),1474173990466921934_i64,7232363983833177877_i64,(-5624298870784083589_i64),6065761624524641153_i64,(-1325845077439792639_i64),(-37471342889936798_i64),6921045958472012426_i64];
_1.0 = [(-2127056916018549439_i64),3098027981019215628_i64,(-497495967649386471_i64),(-1882256853048811663_i64),6235085065871229217_i64,4962454587716459201_i64,5426778782723583084_i64,(-1608485645104947836_i64)];
RET = (_3.0,);
_2 = [4671990311362143116_i64,3230184574808687956_i64,(-4382047693376836585_i64),7665580952640820336_i64,(-1923515485342480907_i64),5320748084409629367_i64,(-8410612878538365628_i64),(-4971656339824602692_i64)];
_5 = (-74_i8);
_6 = [19306_i16,23603_i16,23677_i16,(-2926_i16)];
_3 = (_7,);
_8 = [6356177889593841094_i64,4041861252279727063_i64,7838581903766315879_i64,1154262324096585683_i64,2515465944743478933_i64,1861789339542145090_i64,43335445594602589_i64,1214987263379019385_i64];
_1.0 = _7;
_3.0 = [(-6639184388979369558_i64),(-3815850913551824860_i64),4571974208057547558_i64,5834077736291722489_i64,(-8069938380888062059_i64),(-447068500036465750_i64),(-2167115890110688045_i64),4791574351604212134_i64];
RET = (_1.0,);
_2 = [(-2954430268624997601_i64),6145440977405915799_i64,(-6608597909641972194_i64),5127800557570295515_i64,(-8804759174684683831_i64),(-8137193271141255083_i64),(-7582770705635828915_i64),4606528596758565665_i64];
RET.0 = [(-9006802876428287783_i64),(-6185272462582010006_i64),6151407237233371378_i64,1496440870242379942_i64,(-6974271817187868858_i64),(-7604627251224987164_i64),(-1970753772139259412_i64),(-4775157344185273116_i64)];
RET = _4;
RET = (_7,);
RET = _3;
_4 = (_2,);
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768211382 => bb11,
_ => bb10
}
}
bb15 = {
_4 = _1;
_3.0 = [(-6685009123058649616_i64),1381660199418462499_i64,(-5206301544993240905_i64),(-2271737695107994490_i64),3718090361858555785_i64,(-1368825884784879023_i64),(-4755851758283746059_i64),(-6775538203859885352_i64)];
_14 = 98970023525706886084391808122710955806_i128 as isize;
RET.0 = [4235046379225738574_i64,4570933845523602871_i64,626476387580199658_i64,(-3053468538368360782_i64),2744852919246430470_i64,926737862839390159_i64,(-6256471223837749699_i64),(-4031288216501425714_i64)];
_7 = [(-2193087735608028006_i64),(-2395183550990853988_i64),(-1702266556537248410_i64),2959647806142917166_i64,(-3916033057644503625_i64),2770357769440212214_i64,(-8564799456891502922_i64),2981392750377577033_i64];
_6 = [28497_i16,24741_i16,(-4906_i16),29633_i16];
_2 = [(-4393743824717950152_i64),3051637971384808239_i64,(-4571177467019042301_i64),(-7429644061937574883_i64),(-5612406381514606570_i64),681731809928654482_i64,(-5463905460965860274_i64),3688582546617459882_i64];
RET.0 = [3148784978318086379_i64,7014441297610362655_i64,(-2335601275326121414_i64),(-3409180083779226919_i64),1940717312657274036_i64,(-2389238026985238441_i64),7769720337553422208_i64,54490752138290141_i64];
_2 = _4.0;
RET = (_2,);
_12 = _5 >= _5;
_9 = 6009_u16 & 9570_u16;
RET.0 = _1.0;
_16 = !241123542434747248986639046839581230186_u128;
_13 = '\u{8ac12}';
_7 = _4.0;
RET.0 = [(-8849166229711295154_i64),(-349582070338992809_i64),(-4493934363467181220_i64),(-3000249518623421170_i64),(-1585649280750230529_i64),2389044867490124648_i64,1366594499847010325_i64,(-7634924567528187226_i64)];
_4.0 = [(-8995045039143707422_i64),5643086610474908031_i64,(-6734212993084748662_i64),(-7113697852321596726_i64),614285935466484639_i64,2526308895185052264_i64,(-3528704306310823131_i64),(-738768074261050678_i64)];
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(5_usize, 6_usize, Move(_6), 14_usize, Move(_14), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(5_usize, 3_usize, Move(_3), 12_usize, Move(_12), 11_usize, Move(_11), 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i64; 8],mut _2: [i64; 8],mut _3: ([i64; 8],),mut _4: ([i64; 8],),mut _5: [i64; 8],mut _6: [i64; 8],mut _7: [i64; 8]) -> [i64; 8] {
mir! {
type RET = [i64; 8];
let _8: [i16; 4];
let _9: f64;
let _10: u16;
let _11: bool;
let _12: *mut [usize; 2];
let _13: [i16; 4];
let _14: Adt49;
let _15: ([u16; 1], f64, u8);
let _16: f32;
let _17: usize;
let _18: char;
let _19: isize;
let _20: isize;
let _21: Adt55;
let _22: Adt50;
let _23: Adt54;
let _24: i16;
let _25: Adt46;
let _26: (u32, [i64; 8], u8, isize, *const i16);
let _27: [u64; 5];
let _28: f32;
let _29: Adt45;
let _30: isize;
let _31: i32;
let _32: ([u16; 1], f64, u8);
let _33: Adt56;
let _34: i128;
let _35: [i16; 4];
let _36: [i64; 8];
let _37: (*mut &'static i64,);
let _38: [u128; 2];
let _39: isize;
let _40: [i64; 8];
let _41: Adt45;
let _42: f64;
let _43: ();
let _44: ();
{
RET = _4.0;
_7 = _4.0;
_4 = (_3.0,);
Goto(bb1)
}
bb1 = {
_7 = [1571403799268156170_i64,(-7661146380938167690_i64),5962942842739019294_i64,4655013099892189098_i64,(-5603854041806666212_i64),4467688996340327579_i64,(-1726829284883460540_i64),6775784610596879213_i64];
_4 = (_3.0,);
_8 = [16074_i16,(-25864_i16),12381_i16,(-18547_i16)];
_5 = [5126983254954315151_i64,5419359885759416207_i64,4694551107167854383_i64,5067046724236777814_i64,(-2997255154363881359_i64),7304049697780711820_i64,1346401928684769376_i64,4636825583458902407_i64];
_5 = [8887408757580066163_i64,4393277443517582465_i64,(-8550305968846550336_i64),(-6840055353550640425_i64),1017459295420285182_i64,(-4989489206141246493_i64),(-1750819267980170876_i64),(-2858425696053883151_i64)];
_6 = [7722471963628825704_i64,(-5118448256413017484_i64),(-700107217432308324_i64),(-7753104637108335909_i64),(-7862147274980284022_i64),2499041351277001363_i64,(-7711515775531180245_i64),(-6375938285443839920_i64)];
Goto(bb2)
}
bb2 = {
_7 = [(-2985078593131995045_i64),(-1586499205403607098_i64),(-1816741014291897233_i64),(-2380400451695805016_i64),(-2494056459553184545_i64),(-7388822242575935575_i64),(-547290545586751799_i64),(-6586286764283105100_i64)];
_8 = [18791_i16,15093_i16,(-5405_i16),460_i16];
_5 = [(-8163567017079898378_i64),8046506902270740570_i64,(-6197385467974863081_i64),(-3483370215706841082_i64),1507977490627055706_i64,(-4280298622499862277_i64),7674140475605553493_i64,84911088706569453_i64];
_9 = 6038242850642104379_u64 as f64;
_4 = _3;
_3 = (_7,);
_4.0 = [(-9081977214155853160_i64),8546568868469642595_i64,(-7934352275090926292_i64),738215793169063196_i64,(-2864853550953923581_i64),(-2043447634318616131_i64),2033904388555258641_i64,299520200179092648_i64];
_5 = [6910166484574955535_i64,9043150220520369964_i64,2526432163572925518_i64,(-3187900842050940237_i64),(-3781589040247204969_i64),(-4261308665681065609_i64),(-7269645639535748860_i64),(-4590240973472251778_i64)];
_8 = [29077_i16,(-23250_i16),15359_i16,12823_i16];
_5 = [6980749781537406817_i64,217805736638019036_i64,(-8618094499639946381_i64),6845873984322101375_i64,579580832150851878_i64,5928569099949243058_i64,(-2453864677874203130_i64),(-8977892523758609659_i64)];
_4.0 = [(-4608673625618128095_i64),5566506956414902629_i64,3275859750506789720_i64,8422675425783003661_i64,(-5830255413083457559_i64),8087357299313454291_i64,1952701920385216076_i64,7504000985760415660_i64];
RET = [(-485187797150403454_i64),(-1046605755968379229_i64),(-8800918273375941722_i64),(-4738453293196032777_i64),(-2025608390056605922_i64),(-2229676260154037841_i64),5384219533241283676_i64,(-6239040901117873843_i64)];
_4.0 = [2878597044679573312_i64,(-5540897187548345239_i64),8082866727842117508_i64,1483784106878010225_i64,(-5088150321607227834_i64),5620572253192173775_i64,(-1023728847973586626_i64),7649821016646080275_i64];
_3 = (_4.0,);
RET = _5;
Call(RET = fn7(_4, _2, _1, _6, _4.0, _4.0, _4.0, _8, _4.0, _7, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _4;
_6 = _7;
_1 = RET;
_1 = [263500607897021180_i64,4799941935074376805_i64,(-7015970484174032152_i64),809642491315595856_i64,(-2008154958131580865_i64),(-8440315181383691818_i64),7749704218382368146_i64,7467066889413802051_i64];
_11 = false;
RET = _6;
_7 = [8637441829649488543_i64,5105905283095082424_i64,(-3597355978445976996_i64),(-1328197079333431807_i64),(-3909868213057559201_i64),(-5056111859151857835_i64),5396756237697677646_i64,(-8472484905354386331_i64)];
RET = [(-1039294913024616387_i64),8036454126411764586_i64,4762503012302434770_i64,(-1232968138968398329_i64),7995597166511522360_i64,179023431532677331_i64,(-5790461305070457456_i64),(-1294083820687586865_i64)];
_3 = (_2,);
_10 = 20076_u16;
_3 = (_2,);
_9 = 45983802252121887170996420021889313781_u128 as f64;
_4.0 = _1;
_4.0 = [(-7060760931650086824_i64),(-17189403632041620_i64),2358121188910159786_i64,(-5969828000395492128_i64),1637753709061653061_i64,1829411733469653128_i64,7819617971847709751_i64,3063111714919427780_i64];
_9 = 87_u8 as f64;
_3 = _4;
_5 = _2;
_7 = [(-8873821290117492843_i64),(-1017506095377168134_i64),1599540656064676229_i64,5105132263000730174_i64,7300248024279881659_i64,(-7418667587777650343_i64),7464857128432211289_i64,8541045930694322014_i64];
_2 = [6119068669152808605_i64,234114679465796828_i64,(-7438824937124063913_i64),(-1440271280658325582_i64),(-6053725208764679458_i64),(-7376083780580075888_i64),(-8134381658445275370_i64),(-1863301995551186637_i64)];
_4.0 = _2;
_4.0 = [(-1225600112930308419_i64),(-7565675544244081238_i64),(-8624190699351439382_i64),(-7251975073999766157_i64),(-7084673143831398836_i64),5328174751772675346_i64,2079112190718489836_i64,364683949925786654_i64];
_4.0 = [(-4933673328449854224_i64),(-5240640579123355146_i64),8534698684589812124_i64,(-4457901894922315018_i64),6009718299828048307_i64,(-2404128201583316681_i64),(-6816103414544815287_i64),2958763904890245354_i64];
Goto(bb4)
}
bb4 = {
RET = _7;
_8 = [9055_i16,752_i16,2787_i16,6281_i16];
_8 = [(-16785_i16),(-30330_i16),(-31322_i16),14077_i16];
_4.0 = _5;
_10 = 36756_u16 + 49626_u16;
_3.0 = _6;
_6 = [2877202724118016472_i64,546269738855329921_i64,7183313853928230961_i64,8860069375809607204_i64,(-5741358243478691546_i64),(-1443888730520478118_i64),5306338316953841300_i64,5436604998064697265_i64];
_16 = 4158340952_u32 as f32;
_13 = [982_i16,(-28995_i16),31751_i16,(-29817_i16)];
_13 = [12445_i16,25801_i16,25858_i16,(-12360_i16)];
_15.1 = (-1738229224_i32) as f64;
_8 = [(-30452_i16),(-6901_i16),(-18922_i16),28024_i16];
Goto(bb5)
}
bb5 = {
_3 = (_7,);
RET = [8359216366190707373_i64,(-6939913906399378982_i64),(-4832228554318888732_i64),(-1879526963401021347_i64),5394698745275679109_i64,(-5679894095657821321_i64),(-4861305819684467231_i64),6610723217278586354_i64];
_5 = _6;
Call(_15 = fn8(_3, _7, _3.0, _3.0, _3, _3, _4, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = _4;
_17 = 7_usize & 1803979053546357730_usize;
_6 = [(-9031469914121242910_i64),7780653064498346858_i64,6827053486164502138_i64,(-8742329222401391919_i64),(-1049693719986737210_i64),(-4555785773126301245_i64),(-497009551872598542_i64),6510483765388680075_i64];
_2 = [4900320235512982170_i64,1468136077167002734_i64,1106969203511456876_i64,2090523237920990722_i64,(-1514502574167182110_i64),(-6414294823691238367_i64),8353385410805150102_i64,(-9050507157715511093_i64)];
Goto(bb7)
}
bb7 = {
_10 = 15096_u16;
Goto(bb8)
}
bb8 = {
_19 = 9223372036854775807_isize - 9223372036854775807_isize;
_11 = true;
_2 = [7072539375088741182_i64,338993438591541633_i64,(-9070961630371253591_i64),1573899001106214586_i64,(-4374740292122436002_i64),3714106563119996035_i64,(-1582230121549570692_i64),(-3579374924772569428_i64)];
_8 = [15709_i16,6665_i16,25689_i16,22930_i16];
RET = [(-6668281024850666782_i64),4279548955380626541_i64,505554338088128964_i64,8233997089649810889_i64,(-1682920168619213056_i64),3714777924213853107_i64,7747518531824016758_i64,(-8389637702070253856_i64)];
_18 = '\u{6dc64}';
RET = [(-6624217741984802709_i64),(-200961781820135036_i64),7807204676635557361_i64,(-2944459265031532373_i64),(-467955846840238471_i64),2483373225783174147_i64,(-6753274820786590648_i64),2790327256455185220_i64];
_7 = [(-3050227387776694189_i64),6993648145989223582_i64,7252686436879824273_i64,(-4820436914263990976_i64),5760481218587589785_i64,(-32472072906983124_i64),8498210485204826559_i64,8210937272977148594_i64];
_17 = 5_usize;
_15.0 = [_10];
_8 = _13;
_10 = 36979_u16 << _3.0[_17];
_22.fld0 = Adt47::Variant0 { fld0: _4.0 };
place!(Field::<[i64; 8]>(Variant(_22.fld0, 0), 0))[_17] = _7[_17];
_18 = '\u{82f06}';
_7 = _5;
_8 = _13;
_15.1 = _9 + _9;
_3 = (_4.0,);
_24 = _16 as i16;
_26.0 = !3244526176_u32;
_26.2 = _15.2 << _6[_17];
_4.0[_17] = _3.0[_17];
Goto(bb9)
}
bb9 = {
_13 = [_24,_24,_24,_24];
_19 = (-99_isize);
RET[_17] = _19 as i64;
_26.1[_17] = _6[_17] * RET[_17];
_5 = [_3.0[_17],_4.0[_17],RET[_17],_2[_17],_2[_17],Field::<[i64; 8]>(Variant(_22.fld0, 0), 0)[_17],_7[_17],_3.0[_17]];
_22.fld3 = [_24,_24,_24,_24];
_6[_17] = _2[_17];
_20 = _11 as isize;
_4 = _3;
_8 = [_24,_24,_24,_24];
_3 = (Field::<[i64; 8]>(Variant(_22.fld0, 0), 0),);
_26.4 = core::ptr::addr_of!(_24);
SetDiscriminant(_22.fld0, 1);
RET = _7;
place!(Field::<([u16; 1], f64, u8)>(Variant(_22.fld0, 1), 4)).1 = -_15.1;
match _7[_17] {
0 => bb7,
1 => bb2,
2 => bb6,
3 => bb4,
340282366920938463461930718701247733338 => bb11,
_ => bb10
}
}
bb10 = {
_7 = [(-2985078593131995045_i64),(-1586499205403607098_i64),(-1816741014291897233_i64),(-2380400451695805016_i64),(-2494056459553184545_i64),(-7388822242575935575_i64),(-547290545586751799_i64),(-6586286764283105100_i64)];
_8 = [18791_i16,15093_i16,(-5405_i16),460_i16];
_5 = [(-8163567017079898378_i64),8046506902270740570_i64,(-6197385467974863081_i64),(-3483370215706841082_i64),1507977490627055706_i64,(-4280298622499862277_i64),7674140475605553493_i64,84911088706569453_i64];
_9 = 6038242850642104379_u64 as f64;
_4 = _3;
_3 = (_7,);
_4.0 = [(-9081977214155853160_i64),8546568868469642595_i64,(-7934352275090926292_i64),738215793169063196_i64,(-2864853550953923581_i64),(-2043447634318616131_i64),2033904388555258641_i64,299520200179092648_i64];
_5 = [6910166484574955535_i64,9043150220520369964_i64,2526432163572925518_i64,(-3187900842050940237_i64),(-3781589040247204969_i64),(-4261308665681065609_i64),(-7269645639535748860_i64),(-4590240973472251778_i64)];
_8 = [29077_i16,(-23250_i16),15359_i16,12823_i16];
_5 = [6980749781537406817_i64,217805736638019036_i64,(-8618094499639946381_i64),6845873984322101375_i64,579580832150851878_i64,5928569099949243058_i64,(-2453864677874203130_i64),(-8977892523758609659_i64)];
_4.0 = [(-4608673625618128095_i64),5566506956414902629_i64,3275859750506789720_i64,8422675425783003661_i64,(-5830255413083457559_i64),8087357299313454291_i64,1952701920385216076_i64,7504000985760415660_i64];
RET = [(-485187797150403454_i64),(-1046605755968379229_i64),(-8800918273375941722_i64),(-4738453293196032777_i64),(-2025608390056605922_i64),(-2229676260154037841_i64),5384219533241283676_i64,(-6239040901117873843_i64)];
_4.0 = [2878597044679573312_i64,(-5540897187548345239_i64),8082866727842117508_i64,1483784106878010225_i64,(-5088150321607227834_i64),5620572253192173775_i64,(-1023728847973586626_i64),7649821016646080275_i64];
_3 = (_4.0,);
RET = _5;
Call(RET = fn7(_4, _2, _1, _6, _4.0, _4.0, _4.0, _8, _4.0, _7, _3, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_3.0 = [_26.1[_17],_4.0[_17],_2[_17],_5[_17],_2[_17],_1[_17],_26.1[_17],_1[_17]];
_2[_17] = !_7[_17];
place!(Field::<Adt43>(Variant(_22.fld0, 1), 0)).fld3.2 = _26.0 - _26.0;
_5 = [_4.0[_17],_26.1[_17],_7[_17],_26.1[_17],_2[_17],_4.0[_17],_3.0[_17],_3.0[_17]];
_30 = _19;
_6[_17] = _7[_17] >> _4.0[_17];
place!(Field::<[u16; 1]>(Variant(_22.fld0, 1), 3)) = [_10];
_17 = 15725583588062192325_usize | 15739944498923479512_usize;
_31 = (-484144078_i32) - 847285142_i32;
_26.1 = [(-2017776057375223269_i64),6735963499775176206_i64,(-2549124809217610217_i64),(-7505602504032655737_i64),(-7565692110802890256_i64),6948503462980224827_i64,(-5967767390414172231_i64),(-4396692486224001319_i64)];
_10 = 27643_u16 | 4076_u16;
_26.1 = _6;
_22.fld2 = [_26.2,_26.2,_15.2,_26.2];
place!(Field::<[u16; 1]>(Variant(_22.fld0, 1), 3)) = _15.0;
place!(Field::<([u16; 1], f64, u8)>(Variant(_22.fld0, 1), 4)).2 = _26.2;
_26.4 = core::ptr::addr_of!(place!(Field::<Adt43>(Variant(_22.fld0, 1), 0)).fld1);
_7 = [1235260983471324181_i64,9207198243223650417_i64,(-6436531273983660084_i64),5648113774404751145_i64,(-1654916044856791805_i64),(-125287286384891297_i64),(-8310115431733464801_i64),6515267078623427294_i64];
Goto(bb12)
}
bb12 = {
place!(Field::<Adt43>(Variant(_22.fld0, 1), 0)).fld2 = _13;
_26.1 = [(-2317880024867116924_i64),3127427858754991468_i64,(-6673018987864301489_i64),(-7268275564808123670_i64),7427466656270862262_i64,5807873559586366833_i64,(-1765122537074253633_i64),(-5810503298765146202_i64)];
_22.fld0 = Adt47::Variant0 { fld0: _7 };
_11 = true;
SetDiscriminant(_22.fld0, 0);
_31 = 1129957279_i32 - 467618736_i32;
_26.0 = 2252187953_u32;
_26.3 = _31 as isize;
_17 = 18396656383708229761_usize >> _26.0;
_6 = _1;
_32.2 = _9 as u8;
_34 = _17 as i128;
_31 = 1817933391_i32;
_35 = [_24,_24,_24,_24];
_28 = _16;
_22.fld0 = Adt47::Variant0 { fld0: RET };
Goto(bb13)
}
bb13 = {
_28 = _34 as f32;
_24 = (-32118_i16);
place!(Field::<[i64; 8]>(Variant(_22.fld0, 0), 0)) = [(-2995285633218992242_i64),5270362791671696087_i64,717030937430726757_i64,(-7434385054483935792_i64),(-4798528523548363655_i64),(-2707678827013029712_i64),(-1759428563585914744_i64),8603856318294229365_i64];
RET = [419247829041883156_i64,(-6374139153421918913_i64),8738773827333880417_i64,6012690653713852108_i64,(-4689325165473983467_i64),8258589563688494869_i64,8562096997413450962_i64,6396645119728782041_i64];
_17 = (-41_i8) as usize;
_34 = _11 as i128;
place!(Field::<[i64; 8]>(Variant(_22.fld0, 0), 0)) = _5;
_26.2 = 6016314521831507509_i64 as u8;
SetDiscriminant(_22.fld0, 0);
_2 = [(-597576318132985219_i64),(-645012372730721684_i64),4934455372783248143_i64,5016515059955528068_i64,(-7084414512196704037_i64),4258020152209382545_i64,4418092533548470525_i64,6889038496977941743_i64];
_38 = [245572727462725789152722218792200743936_u128,319670216467925267565036917789854328363_u128];
_32.0 = _15.0;
_19 = !_20;
_19 = _20 ^ _30;
_31 = 1488937569_i32;
_22.fld2 = [_15.2,_15.2,_32.2,_26.2];
_2 = _26.1;
_42 = _17 as f64;
_34 = (-23136506695436369751495462457264318418_i128) << _10;
_30 = !_19;
Goto(bb14)
}
bb14 = {
_22.fld2 = [_32.2,_32.2,_26.2,_15.2];
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(6_usize, 34_usize, Move(_34), 35_usize, Move(_35), 31_usize, Move(_31), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(6_usize, 11_usize, Move(_11), 30_usize, Move(_30), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(6_usize, 3_usize, Move(_3), 19_usize, Move(_19), 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: ([i64; 8],),mut _2: [i64; 8],mut _3: [i64; 8],mut _4: [i64; 8],mut _5: [i64; 8],mut _6: [i64; 8],mut _7: [i64; 8],mut _8: [i16; 4],mut _9: [i64; 8],mut _10: [i64; 8],mut _11: ([i64; 8],),mut _12: ([i64; 8],)) -> [i64; 8] {
mir! {
type RET = [i64; 8];
let _13: isize;
let _14: f32;
let _15: [u8; 4];
let _16: bool;
let _17: usize;
let _18: f64;
let _19: ([i64; 8],);
let _20: f32;
let _21: bool;
let _22: [u64; 5];
let _23: Adt57;
let _24: f64;
let _25: isize;
let _26: char;
let _27: [i16; 4];
let _28: [u8; 4];
let _29: [u64; 5];
let _30: char;
let _31: Adt42;
let _32: bool;
let _33: ();
let _34: ();
{
_7 = [1751805859950126510_i64,(-7346278976094255057_i64),5545620108939909586_i64,(-3297118291904657656_i64),6514208704465089534_i64,(-7123218855554945827_i64),(-5189319084291275553_i64),4770284762340276864_i64];
_12.0 = [(-3632382662172293312_i64),9219052203207590632_i64,(-5468734783731571018_i64),6171978351098933536_i64,1934540456643178774_i64,(-1822102362573701568_i64),(-2225345204622222312_i64),186636235595837986_i64];
_6 = [8931767132043994597_i64,(-2472762798344933208_i64),810075397160519878_i64,7342776766785186676_i64,6840263299121656364_i64,1985466651473964517_i64,8875625884895853874_i64,(-7836756731171426723_i64)];
_12 = (_5,);
_11 = (_9,);
RET = [(-436690252572354482_i64),2446750700904545483_i64,(-8900725643434900388_i64),(-2371474970977071411_i64),(-5969217817781237668_i64),5300015679300307820_i64,4306343672474177606_i64,(-2005903600317209433_i64)];
_10 = [5230826875307094047_i64,9045843936560618658_i64,(-3138029241818272696_i64),(-8464818471879520157_i64),3915800258069309165_i64,8371054553481792858_i64,3549721845591354772_i64,(-7188466158245108479_i64)];
_3 = _7;
_11 = (_1.0,);
_11 = _12;
_8 = [(-1051_i16),(-1935_i16),(-19747_i16),(-7940_i16)];
_13 = (-9223372036854775808_isize);
_8 = [(-14572_i16),(-29138_i16),(-8595_i16),(-31929_i16)];
_2 = [6198424792290546950_i64,(-7572047757730443815_i64),(-181772474810594410_i64),6251665040158216594_i64,(-3712543676075975901_i64),(-5389398780702789745_i64),(-2970851274601434241_i64),(-8509033332234281276_i64)];
_13 = !(-9223372036854775808_isize);
_14 = 58_i8 as f32;
_7 = [3687722823046301509_i64,(-3531552614310148103_i64),864018824215047772_i64,3195977746205683472_i64,(-2679715143696281955_i64),2683649103156627013_i64,8225576842552730249_i64,(-2473072311024322369_i64)];
_3 = [8479252778347408680_i64,2797482848876216882_i64,5126932626714787494_i64,(-8279537754183157178_i64),1824853378756109859_i64,3487150584673437311_i64,(-4664810190571847731_i64),(-1763680565448052809_i64)];
_3 = [(-2554481717519372854_i64),3967071357350704538_i64,(-8984849553105473531_i64),(-8951286549825259339_i64),3858174876101130834_i64,(-1218557279707752366_i64),7219523143287042640_i64,(-8549261522520687724_i64)];
_3 = [(-7741468755310476642_i64),(-3598800560093459325_i64),7411581343084541229_i64,(-8731760294684519785_i64),(-7126560160780640780_i64),(-3811744104526521249_i64),(-5165534776844906474_i64),(-5034772684589256993_i64)];
_5 = RET;
_12.0 = [7289997269376918506_i64,(-8556191285833912659_i64),(-1170819076088695584_i64),(-5729804720083284228_i64),(-4820528512085061912_i64),9061489124686647467_i64,(-4923411164632930118_i64),2303004959275653708_i64];
Goto(bb1)
}
bb1 = {
_5 = _11.0;
RET = [8033970738251273814_i64,(-2202594824089572137_i64),2772444278746896657_i64,7493255166554076530_i64,562726415850503664_i64,3655063216795815790_i64,8655888190953899873_i64,(-3257718716355230639_i64)];
_19 = (_3,);
_17 = 7073862126383667901_usize;
_19 = (_9,);
_7 = [(-1572362476070217195_i64),8372318646750469979_i64,(-3438438011262746316_i64),7980283495616052576_i64,(-1087664994287035222_i64),8703799446421598347_i64,(-7820080655402250724_i64),2475216536650675481_i64];
_18 = 85_u8 as f64;
match _17 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
7073862126383667901 => bb10,
_ => bb9
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
_16 = true;
_3 = _6;
_6 = [(-2068760528783170606_i64),(-6072290260279844385_i64),1158434140566710189_i64,(-1301552727852119767_i64),1812308378667080050_i64,(-411322024443418972_i64),(-4562862852087345947_i64),(-6261531193824821112_i64)];
_12.0 = _10;
_12.0 = [(-8025220092320345427_i64),(-518123651363684036_i64),(-4712915349956354810_i64),(-809770026153758232_i64),4522206152581763492_i64,(-3965131405441618902_i64),2003861161645542374_i64,7954036329815482812_i64];
_2 = [8561029392849470356_i64,(-5178520907921814216_i64),746192308559905445_i64,(-6625221208450071_i64),(-4218895540050655167_i64),2312972650827786559_i64,(-8134116844820601321_i64),(-4033794853550654111_i64)];
_14 = 164933498794490327900734609824433593764_i128 as f32;
_15 = [137_u8,99_u8,48_u8,92_u8];
_15 = [49_u8,133_u8,99_u8,12_u8];
_1.0 = [(-7323487347725692016_i64),3751217965044499624_i64,(-957925681919706631_i64),4141863267642094419_i64,(-6896749572487503181_i64),8720156592454943084_i64,6588545960893552208_i64,(-5423156994727585285_i64)];
_11.0 = [(-929636412343441753_i64),6083714862704391977_i64,(-2427400756854625310_i64),812470486659853660_i64,962502841757797297_i64,(-5055305250447732813_i64),(-5044931896725818274_i64),7813849658531406989_i64];
_7 = [(-7763797787970607694_i64),1404771979519205650_i64,(-6640653098330367703_i64),(-7182279603528525118_i64),(-2176467431924418057_i64),1971421746251309395_i64,(-1286855031759995899_i64),1596871951262571349_i64];
_21 = _16 >= _16;
_12.0 = [(-5693545074049390806_i64),(-1259372435338407441_i64),(-9079643506972199718_i64),6912813537702596025_i64,(-3391205707382947121_i64),(-7156058357673983897_i64),(-359950184072504361_i64),(-8923442878277868306_i64)];
_2 = [6030250361215520526_i64,(-8046670858073409426_i64),1938766184963462880_i64,(-1237391458604955674_i64),(-3270337531591031885_i64),(-1205266287681938510_i64),1021268474840161710_i64,6161962674371114806_i64];
_20 = _17 as f32;
RET = [(-6341277137373938241_i64),(-5599171153323831547_i64),7902657567303475734_i64,(-6449807778066946640_i64),(-426530165554337319_i64),6978289933808834880_i64,(-1167900531312049879_i64),(-2879074155235755358_i64)];
_5 = _9;
RET = _10;
Goto(bb11)
}
bb11 = {
_22 = [15401550126710787076_u64,15141630235453485423_u64,11251651368075212589_u64,10194253825864897428_u64,2932514004110089841_u64];
_24 = _18;
_3 = [7373690634462497198_i64,(-5583021778085217582_i64),3884164550322397244_i64,3392358143398774401_i64,(-2521481119609590778_i64),3167012028828160971_i64,(-3456259498674361516_i64),(-8214016428407547158_i64)];
_24 = -_18;
_10 = _11.0;
_11 = (_2,);
_18 = -_24;
_8 = [12864_i16,8075_i16,10967_i16,(-5958_i16)];
_23 = Adt57::Variant3 { fld0: _14,fld1: _11.0 };
_12.0 = _5;
_17 = 7289_i16 as usize;
_6 = [1561080396418421267_i64,2898477269492583273_i64,4911861459692858015_i64,(-5711930057386298463_i64),(-3208892278747856867_i64),4078702619681736675_i64,(-5837102270709229858_i64),(-7259978501883114455_i64)];
_1 = (_4,);
_9 = [6037435372734516202_i64,(-4730042428121780790_i64),5060692832203958413_i64,1249850563462987844_i64,(-2885153200289016789_i64),(-5854891313775368474_i64),(-107713490403304514_i64),2822431215732161502_i64];
_19 = (_11.0,);
_23 = Adt57::Variant0 { fld0: 243_u8 };
place!(Field::<u8>(Variant(_23, 0), 0)) = 32_u8 << _17;
_2 = [(-2155140727467384068_i64),343068046768510120_i64,(-2723406409034650069_i64),5543772494042931518_i64,(-2311891572197733016_i64),5406552014359269483_i64,5912186288362277157_i64,(-3995116012125458294_i64)];
_14 = _20 - _20;
_16 = !_21;
_26 = '\u{fa6}';
_20 = _14 - _14;
_23 = Adt57::Variant0 { fld0: 28_u8 };
_23 = Adt57::Variant0 { fld0: 140_u8 };
_17 = 3265636397889805929_usize;
_7 = _9;
_6 = _12.0;
match _17 {
0 => bb12,
1 => bb13,
3265636397889805929 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_7 = RET;
_11.0 = _7;
_12 = _19;
_22 = [9549249985244544713_u64,12831307973799240215_u64,12353178547315010936_u64,10827015101835614106_u64,7082733425543831342_u64];
place!(Field::<u8>(Variant(_23, 0), 0)) = _17 as u8;
_12 = (_19.0,);
_17 = 292034627026081395218512687707251553474_u128 as usize;
_14 = 52347_u16 as f32;
RET = [(-3795723188586337389_i64),676487291402562254_i64,8276398530288805882_i64,1549398457590631529_i64,4613349108104733684_i64,(-160913284061390588_i64),4105710043795712005_i64,4714009574982782077_i64];
_13 = (-9223372036854775808_isize);
_11 = _1;
_8 = [10181_i16,4333_i16,20265_i16,8788_i16];
_18 = _24;
_20 = _14;
_10 = [264646972592224149_i64,1240653937090557458_i64,(-3393736155108237923_i64),(-2629961862351304000_i64),5423436435131453602_i64,(-6336346117704392023_i64),(-6547047848056869649_i64),(-7300306362622181713_i64)];
_27 = [(-22218_i16),9920_i16,41_i16,25727_i16];
_12 = _1;
_1 = (_10,);
_20 = _14 + _14;
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(7_usize, 17_usize, Move(_17), 22_usize, Move(_22), 7_usize, Move(_7), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(7_usize, 26_usize, Move(_26), 19_usize, Move(_19), 9_usize, Move(_9), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(7_usize, 8_usize, Move(_8), 13_usize, Move(_13), 34_usize, _34, 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: ([i64; 8],),mut _2: [i64; 8],mut _3: [i64; 8],mut _4: [i64; 8],mut _5: ([i64; 8],),mut _6: ([i64; 8],),mut _7: ([i64; 8],),mut _8: [i64; 8]) -> ([u16; 1], f64, u8) {
mir! {
type RET = ([u16; 1], f64, u8);
let _9: isize;
let _10: char;
let _11: bool;
let _12: *const usize;
let _13: isize;
let _14: u16;
let _15: f32;
let _16: u128;
let _17: i8;
let _18: [u8; 4];
let _19: Adt47;
let _20: ([i64; 8],);
let _21: ([u16; 1], f64, u8);
let _22: Adt52;
let _23: [u64; 5];
let _24: *mut [usize; 2];
let _25: u16;
let _26: u16;
let _27: *const i16;
let _28: isize;
let _29: ([i64; 8],);
let _30: u64;
let _31: Adt48;
let _32: Adt49;
let _33: isize;
let _34: Adt50;
let _35: i64;
let _36: [u64; 5];
let _37: i64;
let _38: [u128; 2];
let _39: f64;
let _40: f32;
let _41: f64;
let _42: [i64; 8];
let _43: f64;
let _44: [u64; 5];
let _45: f64;
let _46: [u64; 5];
let _47: Adt43;
let _48: f64;
let _49: bool;
let _50: [usize; 2];
let _51: isize;
let _52: ([u16; 1], f64, u8);
let _53: u32;
let _54: ([u16; 1], f64, u8);
let _55: ();
let _56: ();
{
RET.0 = [41973_u16];
_6 = (_5.0,);
_7 = _6;
_3 = _7.0;
_5 = (_7.0,);
RET.0 = [53895_u16];
_4 = _8;
_5.0 = _4;
RET.1 = 42833_u16 as f64;
Goto(bb1)
}
bb1 = {
_6.0 = _8;
RET.2 = !19_u8;
Goto(bb2)
}
bb2 = {
_6.0 = [(-8222820695345668317_i64),(-7222570736803333309_i64),2857466235796053028_i64,4195422352108564703_i64,(-6561551458081305200_i64),(-6808906642350643143_i64),(-6361628508572887400_i64),(-11033623145305533_i64)];
_2 = [(-8307770098390336357_i64),(-7916739571041184918_i64),(-3903045563021701538_i64),1598860667126883247_i64,1776736363733170089_i64,(-6964843463769777478_i64),4500465022593349755_i64,8375311344912668875_i64];
RET.1 = RET.2 as f64;
_1 = _5;
_7 = (_6.0,);
_6 = (_1.0,);
_2 = _4;
_1 = (_5.0,);
RET.1 = 2368204802_u32 as f64;
RET.2 = !86_u8;
_1 = (_5.0,);
RET.1 = 764728689_i32 as f64;
_1.0 = [(-1866834251393602932_i64),1338155982932907326_i64,(-8562396015934903772_i64),1927366190658654958_i64,5156011801158883403_i64,7201757820864786800_i64,(-1018129970617755009_i64),(-7814101368046224048_i64)];
Goto(bb3)
}
bb3 = {
RET.1 = 9223372036854775807_isize as f64;
_6.0 = [(-5996784390544503646_i64),5607794775024376330_i64,(-6783914762389552299_i64),(-4348844008826775694_i64),(-6135796855848867364_i64),6427673379166447813_i64,(-68007835723306534_i64),8593476044104780405_i64];
RET.0 = [5105_u16];
_11 = false ^ false;
_7 = (_4,);
_6.0 = [(-3491224351189253788_i64),5473099214852347012_i64,(-7764730886545903440_i64),(-7091575884024078681_i64),3545284777657030367_i64,3660674489324226977_i64,(-432340339193249293_i64),1393201104402542435_i64];
_6 = (_2,);
_10 = '\u{b0ac7}';
_9 = (-9223372036854775808_isize);
RET.0 = [46966_u16];
_13 = _9 * _9;
_9 = _11 as isize;
_5.0 = [(-2563921702755970014_i64),7434651359839185599_i64,587777933958879326_i64,(-3149146826945560364_i64),(-6781497550604054829_i64),1789315082602642145_i64,(-1269680894092450465_i64),(-8676099491194770084_i64)];
_11 = true;
RET.1 = _9 as f64;
_5 = (_7.0,);
Goto(bb4)
}
bb4 = {
_5.0 = [(-3653336244590921977_i64),(-7029541033077794641_i64),2466404268808976583_i64,(-8875399300060940107_i64),(-7674882540625533464_i64),6234580603131330154_i64,(-4485751820393986719_i64),(-7611870714940659056_i64)];
_1 = _6;
_1.0 = [1501983045603382479_i64,4886742556224407078_i64,3300727219078637794_i64,7562046630784226389_i64,(-3960813924374311265_i64),3441241404797382601_i64,7862753380700702282_i64,522237824180272397_i64];
_6 = (_2,);
_1 = (_3,);
_3 = [7790557578132772310_i64,(-2613848634958856116_i64),(-7646000532561784349_i64),(-3478721237119616724_i64),9084053612280925034_i64,6376385280533662128_i64,(-5333583362990285457_i64),5128513679301213016_i64];
RET.0 = [60776_u16];
_7 = (_3,);
RET.1 = RET.2 as f64;
RET.0 = [37529_u16];
RET.0 = [44535_u16];
_4 = [(-7475265311857090224_i64),(-1528384813445336961_i64),6945732271676165668_i64,(-6463364194541658535_i64),(-3861025448951981809_i64),(-2493380588802073653_i64),(-8499649716160016754_i64),(-9053966425730018001_i64)];
_6 = (_7.0,);
Goto(bb5)
}
bb5 = {
_13 = -_9;
_6 = (_2,);
_13 = _9 >> _9;
_14 = 18938_u16 * 32905_u16;
_9 = 4374072173591420592_i64 as isize;
RET.2 = 40_u8;
_11 = false;
RET.0 = [_14];
_10 = '\u{91711}';
_15 = _14 as f32;
_17 = 87_i8;
_21.0 = RET.0;
_17 = !(-58_i8);
_16 = 703861770_u32 as u128;
_8 = [6518424710325902637_i64,(-495885483920784719_i64),7522823790592302118_i64,(-7035954946500631185_i64),(-7602414670689579555_i64),4067053888281648700_i64,3618448451293633017_i64,(-5202222086612846216_i64)];
_13 = 30628_i16 as isize;
Goto(bb6)
}
bb6 = {
_16 = _13 as u128;
_9 = _13 ^ _13;
RET.1 = 8238_i16 as f64;
_10 = '\u{a4da6}';
RET.2 = _15 as u8;
_23 = [17696194292057283443_u64,9889133807764667312_u64,8038526473863593083_u64,15263211651414824394_u64,2423062119465062825_u64];
_7 = (_4,);
_6.0 = _3;
RET.0 = _21.0;
_15 = _14 as f32;
_1 = (_5.0,);
RET.0 = _21.0;
Goto(bb7)
}
bb7 = {
_13 = !_9;
_25 = _13 as u16;
_3 = [(-8678213220580165819_i64),1111622818291427599_i64,6149767942369225829_i64,(-1801019440887135237_i64),8628177394357694655_i64,4660158765679745381_i64,(-3036171328348804020_i64),3408164521593274305_i64];
_10 = '\u{31b48}';
_25 = !_14;
_10 = '\u{97ee5}';
_21 = (RET.0, RET.1, RET.2);
RET.0 = [_14];
_3 = _7.0;
_19 = Adt47::Variant0 { fld0: _3 };
RET.2 = _21.2 | _21.2;
RET.1 = -_21.1;
_2 = Field::<[i64; 8]>(Variant(_19, 0), 0);
_21.1 = 16383249013542446783_u64 as f64;
SetDiscriminant(_19, 1);
place!(Field::<Adt43>(Variant(_19, 1), 0)).fld0 = [_14];
_11 = !true;
RET.0 = [_25];
_19 = Adt47::Variant0 { fld0: _3 };
_5 = (_6.0,);
_11 = _15 == _15;
Goto(bb8)
}
bb8 = {
_29.0 = [3264880649762226092_i64,5611850008248646017_i64,7422101899902303027_i64,(-1685130814284972677_i64),3381251587064683343_i64,4159542944728902127_i64,9170561334902076401_i64,(-1434567666315012395_i64)];
_11 = true;
_2 = [6431383927954481257_i64,(-3889358576660894557_i64),(-7773242735131984152_i64),7360917329481775578_i64,(-6042004170011123040_i64),3598065718200572029_i64,(-5916962046342540578_i64),3197280065910807423_i64];
_21.0 = RET.0;
_19 = Adt47::Variant0 { fld0: _29.0 };
_6 = _5;
_20.0 = _1.0;
_3 = [(-4039957745826157519_i64),4682567853503524676_i64,2224201888654651862_i64,1180638779613323108_i64,(-537121037949237703_i64),2619914115524589306_i64,(-2503421899386073266_i64),(-4580563603835544080_i64)];
_31.fld1.3 = !_13;
Call(_13 = core::intrinsics::bswap(_9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = (_21.0, _21.1, _21.2);
_1.0 = [(-684865155920929727_i64),(-470303714756387842_i64),(-7921843266104539909_i64),(-311693174851419813_i64),(-740743188953980082_i64),(-423650287170103192_i64),7717030937999308878_i64,932302401463919081_i64];
RET.0 = _21.0;
RET.0 = [_25];
Goto(bb10)
}
bb10 = {
_31.fld1.2 = _17 as u8;
_2 = _8;
_16 = 208822869013217071344830980913919749486_u128 << _13;
_8 = Field::<[i64; 8]>(Variant(_19, 0), 0);
SetDiscriminant(_19, 0);
_31.fld1.1 = _8;
_21.0 = RET.0;
_31.fld0 = core::ptr::addr_of_mut!(_31.fld2);
_14 = _25;
_33 = RET.1 as isize;
_34.fld2 = [_31.fld1.2,_31.fld1.2,_31.fld1.2,RET.2];
_23 = [1277036250468938297_u64,4494386690440019586_u64,10276020074710849443_u64,6591822024464055079_u64,11418652829178891739_u64];
place!(Field::<[i64; 8]>(Variant(_19, 0), 0)) = [(-7591767925360124628_i64),770503965991193798_i64,(-8604629678021798131_i64),5817837715044260772_i64,(-1946833958178863510_i64),6334564029615942179_i64,8164040599011712760_i64,3571414551469620123_i64];
_15 = _25 as f32;
_31.fld1.1 = [(-1366714928610794635_i64),2615359855277921723_i64,7666051658987537030_i64,(-4014350816616606037_i64),1183032255682753270_i64,(-4469792607706922729_i64),(-56068557800388185_i64),4338248371057477511_i64];
_21.0 = RET.0;
_39 = _21.1 - RET.1;
Call(_29 = fn9(_6, _13, _6.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_39 = -_21.1;
_34.fld0 = Adt47::Variant0 { fld0: _1.0 };
_29.0 = [(-6968450791389116108_i64),(-4078286094181342871_i64),9208700918118586866_i64,(-2388923127988594009_i64),8907872023970805567_i64,2886799584870013359_i64,(-2505469649830058528_i64),3741146798550691535_i64];
_31.fld1.0 = 2414960338_u32;
place!(Field::<[i64; 8]>(Variant(_34.fld0, 0), 0)) = [5527703674361716976_i64,7907172446247390761_i64,5181512503518902486_i64,2918184313005079991_i64,7455700487605428186_i64,6519080675169113830_i64,(-3725802132207962551_i64),6098438439721671991_i64];
_31.fld2 = [_16,_16];
_36 = _23;
_10 = '\u{dbaf7}';
_5.0 = [2475995064938948330_i64,(-225426609251431495_i64),5331139152421806958_i64,1337504202750205601_i64,(-654873610116120010_i64),8867443480346563458_i64,(-3271052538645585412_i64),(-603428126081718433_i64)];
_38 = [_16,_16];
_31.fld0 = core::ptr::addr_of_mut!(_38);
match _31.fld1.0 {
0 => bb10,
2414960338 => bb13,
_ => bb12
}
}
bb12 = {
RET = (_21.0, _21.1, _21.2);
_1.0 = [(-684865155920929727_i64),(-470303714756387842_i64),(-7921843266104539909_i64),(-311693174851419813_i64),(-740743188953980082_i64),(-423650287170103192_i64),7717030937999308878_i64,932302401463919081_i64];
RET.0 = _21.0;
RET.0 = [_25];
Goto(bb10)
}
bb13 = {
SetDiscriminant(_19, 0);
_2 = [(-5618081539433923594_i64),(-4560909305331509991_i64),2415608923851550567_i64,2624750083368117671_i64,(-1995892065659049456_i64),(-378166730936455994_i64),(-2893850091899817801_i64),(-8343576335576438669_i64)];
RET.2 = !_31.fld1.2;
_35 = (-3217903048841089218_i64);
_3 = [_35,_35,_35,_35,_35,_35,_35,_35];
_42 = _29.0;
RET.2 = _31.fld1.2;
_43 = -_21.1;
_28 = 9852060396050351332_u64 as isize;
_20 = (Field::<[i64; 8]>(Variant(_34.fld0, 0), 0),);
RET = _21;
_46 = [10116170875362334609_u64,11992821478634860014_u64,1642612864959055960_u64,13557888761616639970_u64,18167958637141813358_u64];
RET.0 = _21.0;
_3 = [_35,_35,_35,_35,_35,_35,_35,_35];
SetDiscriminant(_34.fld0, 1);
_35 = !299073519793542753_i64;
_30 = 8263388623896830520_u64 + 11963408851515104547_u64;
place!(Field::<Adt43>(Variant(_34.fld0, 1), 0)).fld1 = (-20802_i16) - 4696_i16;
_31.fld1.2 = _21.2 & _21.2;
place!(Field::<Adt43>(Variant(_34.fld0, 1), 0)).fld3.1 = core::ptr::addr_of!(_16);
_47.fld3.0 = _10;
_6.0 = _1.0;
place!(Field::<Adt43>(Variant(_34.fld0, 1), 0)).fld2 = [Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1,Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1,Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1,Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1];
_21.1 = _43 - _43;
_10 = _47.fld3.0;
_28 = _31.fld1.3 ^ _31.fld1.3;
_31.fld1.1 = _8;
_22 = Adt52::Variant3 { fld0: _15,fld1: 89175368411154393351588898582737893899_i128 };
_49 = _11;
match _31.fld1.0 {
0 => bb8,
1 => bb12,
2 => bb14,
3 => bb15,
4 => bb16,
2414960338 => bb18,
_ => bb17
}
}
bb14 = {
_6.0 = [(-8222820695345668317_i64),(-7222570736803333309_i64),2857466235796053028_i64,4195422352108564703_i64,(-6561551458081305200_i64),(-6808906642350643143_i64),(-6361628508572887400_i64),(-11033623145305533_i64)];
_2 = [(-8307770098390336357_i64),(-7916739571041184918_i64),(-3903045563021701538_i64),1598860667126883247_i64,1776736363733170089_i64,(-6964843463769777478_i64),4500465022593349755_i64,8375311344912668875_i64];
RET.1 = RET.2 as f64;
_1 = _5;
_7 = (_6.0,);
_6 = (_1.0,);
_2 = _4;
_1 = (_5.0,);
RET.1 = 2368204802_u32 as f64;
RET.2 = !86_u8;
_1 = (_5.0,);
RET.1 = 764728689_i32 as f64;
_1.0 = [(-1866834251393602932_i64),1338155982932907326_i64,(-8562396015934903772_i64),1927366190658654958_i64,5156011801158883403_i64,7201757820864786800_i64,(-1018129970617755009_i64),(-7814101368046224048_i64)];
Goto(bb3)
}
bb15 = {
_6.0 = _8;
RET.2 = !19_u8;
Goto(bb2)
}
bb16 = {
_31.fld1.2 = _17 as u8;
_2 = _8;
_16 = 208822869013217071344830980913919749486_u128 << _13;
_8 = Field::<[i64; 8]>(Variant(_19, 0), 0);
SetDiscriminant(_19, 0);
_31.fld1.1 = _8;
_21.0 = RET.0;
_31.fld0 = core::ptr::addr_of_mut!(_31.fld2);
_14 = _25;
_33 = RET.1 as isize;
_34.fld2 = [_31.fld1.2,_31.fld1.2,_31.fld1.2,RET.2];
_23 = [1277036250468938297_u64,4494386690440019586_u64,10276020074710849443_u64,6591822024464055079_u64,11418652829178891739_u64];
place!(Field::<[i64; 8]>(Variant(_19, 0), 0)) = [(-7591767925360124628_i64),770503965991193798_i64,(-8604629678021798131_i64),5817837715044260772_i64,(-1946833958178863510_i64),6334564029615942179_i64,8164040599011712760_i64,3571414551469620123_i64];
_15 = _25 as f32;
_31.fld1.1 = [(-1366714928610794635_i64),2615359855277921723_i64,7666051658987537030_i64,(-4014350816616606037_i64),1183032255682753270_i64,(-4469792607706922729_i64),(-56068557800388185_i64),4338248371057477511_i64];
_21.0 = RET.0;
_39 = _21.1 - RET.1;
Call(_29 = fn9(_6, _13, _6.0), ReturnTo(bb11), UnwindUnreachable())
}
bb17 = {
RET.1 = 9223372036854775807_isize as f64;
_6.0 = [(-5996784390544503646_i64),5607794775024376330_i64,(-6783914762389552299_i64),(-4348844008826775694_i64),(-6135796855848867364_i64),6427673379166447813_i64,(-68007835723306534_i64),8593476044104780405_i64];
RET.0 = [5105_u16];
_11 = false ^ false;
_7 = (_4,);
_6.0 = [(-3491224351189253788_i64),5473099214852347012_i64,(-7764730886545903440_i64),(-7091575884024078681_i64),3545284777657030367_i64,3660674489324226977_i64,(-432340339193249293_i64),1393201104402542435_i64];
_6 = (_2,);
_10 = '\u{b0ac7}';
_9 = (-9223372036854775808_isize);
RET.0 = [46966_u16];
_13 = _9 * _9;
_9 = _11 as isize;
_5.0 = [(-2563921702755970014_i64),7434651359839185599_i64,587777933958879326_i64,(-3149146826945560364_i64),(-6781497550604054829_i64),1789315082602642145_i64,(-1269680894092450465_i64),(-8676099491194770084_i64)];
_11 = true;
RET.1 = _9 as f64;
_5 = (_7.0,);
Goto(bb4)
}
bb18 = {
_29 = (_2,);
_31.fld1.4 = core::ptr::addr_of!(place!(Field::<Adt43>(Variant(_34.fld0, 1), 0)).fld1);
_10 = _47.fld3.0;
_47.fld3.0 = _10;
_31.fld1.2 = _21.2 + RET.2;
place!(Field::<Adt43>(Variant(_34.fld0, 1), 0)).fld2 = [Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1,Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1,Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1,Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1];
_24 = core::ptr::addr_of_mut!(_50);
_8 = [_35,_35,_35,_35,_35,_35,_35,_35];
_47.fld3 = (_10, Field::<Adt43>(Variant(_34.fld0, 1), 0).fld3.1, _31.fld1.0, _14);
(*_24) = [4_usize,2_usize];
place!(Field::<[i64; 8]>(Variant(_19, 0), 0)) = _31.fld1.1;
_6.0 = [_35,_35,_35,_35,_35,_35,_35,_35];
_1 = _20;
_44 = _46;
place!(Field::<Adt44>(Variant(_34.fld0, 1), 1)) = Adt44::Variant0 { fld0: (*_24),fld1: Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1 };
_48 = RET.1;
place!(Field::<i16>(Variant(place!(Field::<Adt44>(Variant(_34.fld0, 1), 1)), 0), 1)) = Field::<Adt43>(Variant(_34.fld0, 1), 0).fld1;
_6 = _20;
_26 = _47.fld3.3;
_6.0 = [_35,_35,_35,_35,_35,_35,_35,_35];
_3 = [_35,_35,_35,_35,_35,_35,_35,_35];
place!(Field::<Adt43>(Variant(_34.fld0, 1), 0)).fld3 = _47.fld3;
place!(Field::<i128>(Variant(_22, 3), 1)) = 11996685514214744722_usize as i128;
_24 = core::ptr::addr_of_mut!(place!(Field::<[usize; 2]>(Variant(place!(Field::<Adt44>(Variant(_34.fld0, 1), 1)), 0), 0)));
_36 = [_30,_30,_30,_30,_30];
_51 = _13 | _31.fld1.3;
Goto(bb19)
}
bb19 = {
Call(_55 = dump_var(8_usize, 25_usize, Move(_25), 9_usize, Move(_9), 3_usize, Move(_3), 49_usize, Move(_49)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_55 = dump_var(8_usize, 11_usize, Move(_11), 17_usize, Move(_17), 13_usize, Move(_13), 2_usize, Move(_2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_55 = dump_var(8_usize, 36_usize, Move(_36), 14_usize, Move(_14), 50_usize, Move(_50), 51_usize, Move(_51)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_55 = dump_var(8_usize, 1_usize, Move(_1), 5_usize, Move(_5), 6_usize, Move(_6), 26_usize, Move(_26)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: ([i64; 8],),mut _2: isize,mut _3: [i64; 8]) -> ([i64; 8],) {
mir! {
type RET = ([i64; 8],);
let _4: [u128; 2];
let _5: [u16; 1];
let _6: *mut [usize; 2];
let _7: isize;
let _8: isize;
let _9: i64;
let _10: [u128; 2];
let _11: *const i16;
let _12: u16;
let _13: u32;
let _14: ((char, *const u128, u32, u16),);
let _15: char;
let _16: f32;
let _17: [u64; 5];
let _18: isize;
let _19: f64;
let _20: isize;
let _21: ((*mut &'static i64,), i8);
let _22: i128;
let _23: [usize; 2];
let _24: [u8; 4];
let _25: ([i64; 8],);
let _26: [i16; 4];
let _27: u32;
let _28: [u64; 5];
let _29: f32;
let _30: [i64; 8];
let _31: Adt42;
let _32: i16;
let _33: i32;
let _34: [u8; 4];
let _35: Adt52;
let _36: [u16; 1];
let _37: char;
let _38: *const usize;
let _39: ();
let _40: ();
{
_4 = [136721548350532577987340952307962108637_u128,41680192178421790219506259023056938618_u128];
RET.0 = [(-1466752710448894704_i64),(-2968147541060260746_i64),4116224407051255479_i64,(-8465564913791767378_i64),(-1290498477724886543_i64),(-7699060766203897044_i64),587792894187851014_i64,(-7408274895213615898_i64)];
RET.0 = [(-8612318849377822204_i64),8967275465382845237_i64,8622650834071419583_i64,(-8788701503746956486_i64),4406312479974413760_i64,5936873357974834797_i64,3893011456060816314_i64,6509057684334850705_i64];
RET = (_3,);
_4 = [217391785945825396455609747307239692293_u128,200169652475296533375148295727681453345_u128];
RET = _1;
_1 = (_3,);
RET = (_1.0,);
_2 = 9223372036854775807_isize;
match _2 {
0 => bb1,
9223372036854775807 => bb3,
_ => bb2
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
RET = _1;
RET = _1;
RET = _1;
_2 = !(-103_isize);
_5 = [57798_u16];
RET = (_1.0,);
_1 = (_3,);
_5 = [18619_u16];
_5 = [39540_u16];
_1 = (RET.0,);
RET.0 = [8025557360856672478_i64,(-8575059740222268177_i64),(-6600293776817845209_i64),8692258281153674312_i64,5810407497909196563_i64,(-3281286651361527550_i64),5491105488378363919_i64,(-348704030079215489_i64)];
_2 = 49_isize << 3707646195_u32;
_3 = [5679780827430631778_i64,5866173857984146384_i64,7612252037356559911_i64,(-4441745621771638582_i64),7948873771365274036_i64,(-252824280923337064_i64),7177736147430706339_i64,8262457893892041833_i64];
RET.0 = [8747957130694267643_i64,5649012059146453066_i64,(-2660001260032866089_i64),(-704610786624604251_i64),(-2805829867213402446_i64),(-13397917642303211_i64),4056606647143051400_i64,(-4007908845066368171_i64)];
RET = (_1.0,);
RET.0 = [(-703798353384491509_i64),(-5149508603413341940_i64),3264713325475670977_i64,(-1026542031747703702_i64),2180001439040415059_i64,(-5230909124465024609_i64),(-5141096914600222340_i64),8494605065690026697_i64];
RET.0 = [58716295631940446_i64,7949366507989649707_i64,(-4766383303549303741_i64),3475407935280312283_i64,(-1082534881214332598_i64),(-5391519329946966214_i64),262625728757337304_i64,(-8987771509050135856_i64)];
_1 = (_3,);
_8 = _2;
RET = _1;
_5 = [27275_u16];
_9 = -(-1258024626660521376_i64);
RET = _1;
_4 = [96917674070235363132676090825911698637_u128,144609681409215790745707052996593783070_u128];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = [29336_u16];
_10 = [328724906071999264393058288719625052448_u128,218433119856578431747398482158464663466_u128];
_4 = [92383553600968466773555839528078182778_u128,13804843957074125097890437632584052417_u128];
Call(_5 = fn10(_1, _8, _3, _8, _3, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = _2;
RET = (_3,);
RET = (_3,);
RET = (_3,);
RET = (_3,);
_12 = true as u16;
_9 = 2929923475879305338_i64;
_1 = (_3,);
_13 = 2949724251_u32 - 302175084_u32;
_1 = (_3,);
_2 = _7 >> _9;
_12 = 220650932359297066944849518054593369316_u128 as u16;
_14.0.2 = _13 + _13;
RET = (_1.0,);
_10 = [235605562021361926351770183556599848142_u128,51355365943344682864091133320788196390_u128];
_14.0.0 = '\u{4971b}';
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
RET = (_3,);
_14.0.3 = (-64_i8) as u16;
_15 = _14.0.0;
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = [_12];
_12 = !_14.0.3;
_14.0.3 = _12;
_13 = _2 as u32;
_9 = 11_u8 as i64;
Goto(bb5)
}
bb5 = {
_13 = _14.0.2;
_13 = !_14.0.2;
_14.0.2 = 0_usize as u32;
_1 = (RET.0,);
_15 = _14.0.0;
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_14.0.2 = _13;
_7 = _2;
_15 = _14.0.0;
_12 = _14.0.3;
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_8 = 7360468850039496645_usize as isize;
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_13 = 89_i8 as u32;
_5 = [_12];
_1.0 = RET.0;
_16 = (-9645_i16) as f32;
_1 = (RET.0,);
_17 = [4032584956987165772_u64,16082734301448952117_u64,3638356088961853380_u64,11642273595130165684_u64,9647578334212317458_u64];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_8 = !_2;
_2 = !_8;
_5 = [_12];
Call(_20 = core::intrinsics::transmute(_7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = _4;
_8 = _2 | _2;
_5 = [_14.0.3];
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = (_3,);
RET.0 = _3;
_9 = _14.0.2 as i64;
_12 = _14.0.3;
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17 = [13393700689180027446_u64,4205119888569357766_u64,12665977245791856468_u64,18178257535436871486_u64,7324812400463355983_u64];
_21.1 = (-61_i8) * (-2_i8);
_21.1 = 87_i8;
_7 = !_8;
_16 = 283229330420092503220091024809279003865_u128 as f32;
_14.0.3 = _14.0.2 as u16;
_19 = 155_u8 as f64;
_22 = _14.0.3 as i128;
_14.0.2 = _13 * _13;
_10 = [240718939344877943510678002124507165148_u128,156018370996139299501215567204029120901_u128];
_3 = RET.0;
_7 = _20 >> _9;
_14.0.2 = _13;
_16 = _14.0.2 as f32;
Goto(bb7)
}
bb7 = {
_23 = [7_usize,6041222320012987016_usize];
_25.0 = _1.0;
_22 = 46116418073342383065448345716349320599_i128 | (-20658676694925980746492225070063829146_i128);
_17 = [15679437853219762953_u64,10848415146981662305_u64,10263501224992220844_u64,7201261347441883565_u64,12214149105876151909_u64];
_23 = [1_usize,4810416224856377092_usize];
RET = (_25.0,);
_24 = [154_u8,31_u8,27_u8,177_u8];
_4 = _10;
RET = _1;
_21.1 = -(-128_i8);
_6 = core::ptr::addr_of_mut!(_23);
RET = (_3,);
_17 = [13354551372551617885_u64,1580004027914977178_u64,11087281862103709392_u64,8089441999216778179_u64,15997040597235027432_u64];
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_27 = _13 - _13;
_24 = [135_u8,101_u8,138_u8,247_u8];
_1.0 = _25.0;
_13 = _27;
_26 = [10161_i16,(-30722_i16),31840_i16,(-24456_i16)];
Goto(bb8)
}
bb8 = {
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_28 = [11098413609600216716_u64,16267609646365615547_u64,3051275304158279565_u64,16707102133845837015_u64,3347375107986217489_u64];
_7 = !_8;
_22 = 176_u8 as i128;
_28 = _17;
_15 = _14.0.0;
_1 = (RET.0,);
RET = (_3,);
_25 = (RET.0,);
_22 = 1776547339119855744920536678030161448_i128;
_12 = !_14.0.3;
_30 = [_9,_9,_9,_9,_9,_9,_9,_9];
(*_6) = [0_usize,9566138286214101812_usize];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_16 = _14.0.2 as f32;
_29 = _16 - _16;
_27 = _12 as u32;
_25.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_31.fld2 = core::ptr::addr_of_mut!(_10);
_5 = [_12];
_2 = -_8;
_13 = 1_usize as u32;
match _22 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1776547339119855744920536678030161448 => bb10,
_ => bb9
}
}
bb9 = {
_13 = _14.0.2;
_13 = !_14.0.2;
_14.0.2 = 0_usize as u32;
_1 = (RET.0,);
_15 = _14.0.0;
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_14.0.2 = _13;
_7 = _2;
_15 = _14.0.0;
_12 = _14.0.3;
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_8 = 7360468850039496645_usize as isize;
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_13 = 89_i8 as u32;
_5 = [_12];
_1.0 = RET.0;
_16 = (-9645_i16) as f32;
_1 = (RET.0,);
_17 = [4032584956987165772_u64,16082734301448952117_u64,3638356088961853380_u64,11642273595130165684_u64,9647578334212317458_u64];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_8 = !_2;
_2 = !_8;
_5 = [_12];
Call(_20 = core::intrinsics::transmute(_7), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_6 = core::ptr::addr_of_mut!(_23);
_17 = [10064436123366155883_u64,16153285220795376644_u64,488873040003773299_u64,7223848042792681199_u64,3309304389291167809_u64];
match _22 {
0 => bb3,
1 => bb8,
1776547339119855744920536678030161448 => bb12,
_ => bb11
}
}
bb11 = {
_23 = [7_usize,6041222320012987016_usize];
_25.0 = _1.0;
_22 = 46116418073342383065448345716349320599_i128 | (-20658676694925980746492225070063829146_i128);
_17 = [15679437853219762953_u64,10848415146981662305_u64,10263501224992220844_u64,7201261347441883565_u64,12214149105876151909_u64];
_23 = [1_usize,4810416224856377092_usize];
RET = (_25.0,);
_24 = [154_u8,31_u8,27_u8,177_u8];
_4 = _10;
RET = _1;
_21.1 = -(-128_i8);
_6 = core::ptr::addr_of_mut!(_23);
RET = (_3,);
_17 = [13354551372551617885_u64,1580004027914977178_u64,11087281862103709392_u64,8089441999216778179_u64,15997040597235027432_u64];
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_27 = _13 - _13;
_24 = [135_u8,101_u8,138_u8,247_u8];
_1.0 = _25.0;
_13 = _27;
_26 = [10161_i16,(-30722_i16),31840_i16,(-24456_i16)];
Goto(bb8)
}
bb12 = {
_25.0 = _30;
_4 = [321160662406779683130995724572912105710_u128,157743123775231283923060764675567395760_u128];
_4 = [242824154942905816176260290894536703018_u128,265851312537461380535768478491390125450_u128];
_16 = _29;
(*_6) = [7_usize,0_usize];
_33 = !(-759393173_i32);
_31.fld2 = core::ptr::addr_of_mut!(_4);
_36 = [_12];
match _22 {
0 => bb4,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
1776547339119855744920536678030161448 => bb19,
_ => bb18
}
}
bb13 = {
_23 = [7_usize,6041222320012987016_usize];
_25.0 = _1.0;
_22 = 46116418073342383065448345716349320599_i128 | (-20658676694925980746492225070063829146_i128);
_17 = [15679437853219762953_u64,10848415146981662305_u64,10263501224992220844_u64,7201261347441883565_u64,12214149105876151909_u64];
_23 = [1_usize,4810416224856377092_usize];
RET = (_25.0,);
_24 = [154_u8,31_u8,27_u8,177_u8];
_4 = _10;
RET = _1;
_21.1 = -(-128_i8);
_6 = core::ptr::addr_of_mut!(_23);
RET = (_3,);
_17 = [13354551372551617885_u64,1580004027914977178_u64,11087281862103709392_u64,8089441999216778179_u64,15997040597235027432_u64];
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_27 = _13 - _13;
_24 = [135_u8,101_u8,138_u8,247_u8];
_1.0 = _25.0;
_13 = _27;
_26 = [10161_i16,(-30722_i16),31840_i16,(-24456_i16)];
Goto(bb8)
}
bb14 = {
Return()
}
bb15 = {
RET = _1;
RET = _1;
RET = _1;
_2 = !(-103_isize);
_5 = [57798_u16];
RET = (_1.0,);
_1 = (_3,);
_5 = [18619_u16];
_5 = [39540_u16];
_1 = (RET.0,);
RET.0 = [8025557360856672478_i64,(-8575059740222268177_i64),(-6600293776817845209_i64),8692258281153674312_i64,5810407497909196563_i64,(-3281286651361527550_i64),5491105488378363919_i64,(-348704030079215489_i64)];
_2 = 49_isize << 3707646195_u32;
_3 = [5679780827430631778_i64,5866173857984146384_i64,7612252037356559911_i64,(-4441745621771638582_i64),7948873771365274036_i64,(-252824280923337064_i64),7177736147430706339_i64,8262457893892041833_i64];
RET.0 = [8747957130694267643_i64,5649012059146453066_i64,(-2660001260032866089_i64),(-704610786624604251_i64),(-2805829867213402446_i64),(-13397917642303211_i64),4056606647143051400_i64,(-4007908845066368171_i64)];
RET = (_1.0,);
RET.0 = [(-703798353384491509_i64),(-5149508603413341940_i64),3264713325475670977_i64,(-1026542031747703702_i64),2180001439040415059_i64,(-5230909124465024609_i64),(-5141096914600222340_i64),8494605065690026697_i64];
RET.0 = [58716295631940446_i64,7949366507989649707_i64,(-4766383303549303741_i64),3475407935280312283_i64,(-1082534881214332598_i64),(-5391519329946966214_i64),262625728757337304_i64,(-8987771509050135856_i64)];
_1 = (_3,);
_8 = _2;
RET = _1;
_5 = [27275_u16];
_9 = -(-1258024626660521376_i64);
RET = _1;
_4 = [96917674070235363132676090825911698637_u128,144609681409215790745707052996593783070_u128];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = [29336_u16];
_10 = [328724906071999264393058288719625052448_u128,218433119856578431747398482158464663466_u128];
_4 = [92383553600968466773555839528078182778_u128,13804843957074125097890437632584052417_u128];
Call(_5 = fn10(_1, _8, _3, _8, _3, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb16 = {
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_28 = [11098413609600216716_u64,16267609646365615547_u64,3051275304158279565_u64,16707102133845837015_u64,3347375107986217489_u64];
_7 = !_8;
_22 = 176_u8 as i128;
_28 = _17;
_15 = _14.0.0;
_1 = (RET.0,);
RET = (_3,);
_25 = (RET.0,);
_22 = 1776547339119855744920536678030161448_i128;
_12 = !_14.0.3;
_30 = [_9,_9,_9,_9,_9,_9,_9,_9];
(*_6) = [0_usize,9566138286214101812_usize];
RET.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_16 = _14.0.2 as f32;
_29 = _16 - _16;
_27 = _12 as u32;
_25.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_31.fld2 = core::ptr::addr_of_mut!(_10);
_5 = [_12];
_2 = -_8;
_13 = 1_usize as u32;
match _22 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1776547339119855744920536678030161448 => bb10,
_ => bb9
}
}
bb17 = {
Return()
}
bb18 = {
_10 = _4;
_8 = _2 | _2;
_5 = [_14.0.3];
_3 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = (_3,);
RET.0 = _3;
_9 = _14.0.2 as i64;
_12 = _14.0.3;
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17 = [13393700689180027446_u64,4205119888569357766_u64,12665977245791856468_u64,18178257535436871486_u64,7324812400463355983_u64];
_21.1 = (-61_i8) * (-2_i8);
_21.1 = 87_i8;
_7 = !_8;
_16 = 283229330420092503220091024809279003865_u128 as f32;
_14.0.3 = _14.0.2 as u16;
_19 = 155_u8 as f64;
_22 = _14.0.3 as i128;
_14.0.2 = _13 * _13;
_10 = [240718939344877943510678002124507165148_u128,156018370996139299501215567204029120901_u128];
_3 = RET.0;
_7 = _20 >> _9;
_14.0.2 = _13;
_16 = _14.0.2 as f32;
Goto(bb7)
}
bb19 = {
_14.0.0 = _15;
_30 = [_9,_9,_9,_9,_9,_9,_9,_9];
(*_6) = [7_usize,7_usize];
_35 = Adt52::Variant2 { fld0: _27,fld1: _4,fld2: _14.0.3 };
_6 = core::ptr::addr_of_mut!(_23);
_1 = (_25.0,);
_34 = [186_u8,43_u8,35_u8,79_u8];
SetDiscriminant(_35, 2);
place!(Field::<u32>(Variant(_35, 2), 0)) = _13 & _27;
RET.0 = _30;
_8 = _2;
Goto(bb20)
}
bb20 = {
Call(_39 = dump_var(9_usize, 34_usize, Move(_34), 26_usize, Move(_26), 24_usize, Move(_24), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(9_usize, 25_usize, Move(_25), 23_usize, Move(_23), 30_usize, Move(_30), 33_usize, Move(_33)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_39 = dump_var(9_usize, 2_usize, Move(_2), 12_usize, Move(_12), 27_usize, Move(_27), 22_usize, Move(_22)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: ([i64; 8],),mut _2: isize,mut _3: [i64; 8],mut _4: isize,mut _5: [i64; 8],mut _6: ([i64; 8],)) -> [u16; 1] {
mir! {
type RET = [u16; 1];
let _7: ([u16; 1], f64, u8);
let _8: [i16; 4];
let _9: isize;
let _10: [u128; 2];
let _11: *const u32;
let _12: isize;
let _13: char;
let _14: [u16; 1];
let _15: [i64; 8];
let _16: isize;
let _17: char;
let _18: ([i64; 8],);
let _19: f64;
let _20: *mut [u128; 2];
let _21: isize;
let _22: Adt50;
let _23: f64;
let _24: ([i64; 8],);
let _25: i16;
let _26: ((char, *const u128, u32, u16),);
let _27: u128;
let _28: [u8; 4];
let _29: isize;
let _30: isize;
let _31: [u8; 4];
let _32: ([i64; 8],);
let _33: [u64; 5];
let _34: Adt51;
let _35: ([u16; 1], f64, u8);
let _36: Adt57;
let _37: ((*mut &'static i64,), i8);
let _38: isize;
let _39: i8;
let _40: [u8; 4];
let _41: ();
let _42: ();
{
RET = [53080_u16];
_4 = (-17_i8) as isize;
_3 = [(-1507102882129102438_i64),(-6635036981560163963_i64),(-2830211273661473798_i64),4015865684223010048_i64,(-1919029525864442441_i64),7381392624142725787_i64,4204027006245494442_i64,(-2802325605088246656_i64)];
_4 = (-650389919_i32) as isize;
_7.0 = RET;
_1.0 = _6.0;
RET = _7.0;
_6 = _1;
_7.2 = 72_u8;
_1 = _6;
_1 = _6;
_6 = _1;
_8 = [30528_i16,4040_i16,(-30974_i16),245_i16];
_6.0 = [(-1719467868187345637_i64),4308762418428149806_i64,7069077767013585356_i64,1900550230438144664_i64,7778988806282959962_i64,(-6711697988239202076_i64),(-3647156882904486391_i64),(-4806084415736698699_i64)];
_5 = _1.0;
_7.1 = 1205967845_u32 as f64;
_7.2 = 204_u8 & 98_u8;
RET = _7.0;
RET = [46709_u16];
_7.1 = (-8328340851425297080_i64) as f64;
_2 = (-32411_i16) as isize;
_10 = [71044470277111451636984014271877815133_u128,175244349545044526719507059897340799817_u128];
_4 = _2 * _2;
Goto(bb1)
}
bb1 = {
_12 = _4 - _2;
RET = _7.0;
_6 = _1;
_5 = [(-3729546433498745797_i64),632855232583634782_i64,(-6351643956285081134_i64),379907465594411778_i64,(-2568030730163285109_i64),365022885567112388_i64,5019498111391415319_i64,4384178702322034915_i64];
_13 = '\u{6c38b}';
_7.0 = RET;
_7.2 = 189_u8;
_7.2 = 3639955908704865837_i64 as u8;
_7.2 = 62_u8;
_1.0 = [(-8305985608230825078_i64),(-168886896509684803_i64),5353502043711925349_i64,9166083625245604436_i64,5784218908613537205_i64,(-880822436050979824_i64),(-6115502810590166798_i64),(-8443187007523073060_i64)];
_1 = (_6.0,);
_7.2 = 158_u8;
_8 = [7420_i16,(-19775_i16),(-11783_i16),28059_i16];
_2 = (-63_i8) as isize;
_7.1 = (-64_i8) as f64;
_10 = [109059841497175307568102727818933556009_u128,293991696718519206318784058434344212569_u128];
_1.0 = [3010995350760331102_i64,(-6143689592400687743_i64),1650040420404375778_i64,(-5598155454434030252_i64),4920036255387835128_i64,(-6311305283745238094_i64),3519703220766783029_i64,(-3588362338617946247_i64)];
_1.0 = [(-5360069266511741273_i64),(-456884495126306459_i64),5701043941016023552_i64,2309523260225510715_i64,7614627039375020937_i64,(-4637893721582976828_i64),465099009906590052_i64,1024802992265873499_i64];
_14 = [48317_u16];
match _7.2 {
0 => bb2,
1 => bb3,
2 => bb4,
158 => bb6,
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
_1.0 = _3;
_7.2 = 111_u8 & 155_u8;
_1.0 = [2639257976740418493_i64,(-4396750102933977455_i64),(-6571986847017003542_i64),(-2121230276356919000_i64),4655330111453466750_i64,(-5696977201266796807_i64),311297959424272667_i64,(-5072553397519377833_i64)];
_18.0 = [(-3244805152964249099_i64),(-5467432539153831220_i64),(-4335150915655297784_i64),7172289304406726690_i64,2740211108256353755_i64,(-5752359521179589927_i64),(-7935519278745967747_i64),(-85012744394280037_i64)];
_17 = _13;
_6.0 = [(-1270446283120018886_i64),3542911173281250387_i64,(-8864896234263380837_i64),8811098126191139746_i64,(-605789350102012904_i64),553218092293306888_i64,(-2362193508967113200_i64),976900506777292167_i64];
Goto(bb7)
}
bb7 = {
_14 = RET;
_15 = _18.0;
_4 = 2_usize as isize;
_6.0 = [1941553830255643826_i64,(-498421395827696643_i64),1529828371216413055_i64,(-2254793238700152684_i64),6476599798010536363_i64,1625039290585443501_i64,(-8041626829287152805_i64),901999894056780995_i64];
_4 = _12;
_14 = _7.0;
_1.0 = [(-7456243314501842132_i64),2065333135379666896_i64,8202811569982826286_i64,(-4531601485126281608_i64),3456332570544987643_i64,(-5942683617320385585_i64),(-5821511792882154754_i64),2729256611479696868_i64];
_19 = _7.1;
_20 = core::ptr::addr_of_mut!(_10);
_5 = _15;
_20 = core::ptr::addr_of_mut!(_10);
_5 = [4199075835708189237_i64,8877331258847058773_i64,971518132754488133_i64,(-2456526730581840310_i64),(-7122948683992299867_i64),4514860560562857255_i64,6653970282663345246_i64,(-2776969502091769374_i64)];
RET = _14;
_1 = _6;
_7.1 = _19 - _19;
Goto(bb8)
}
bb8 = {
_22.fld3 = [(-24122_i16),10976_i16,(-9874_i16),(-10026_i16)];
(*_20) = [106280441387335108131979174156560050216_u128,270847513756534798928470075156190200610_u128];
_1 = (_6.0,);
_21 = !_4;
RET = _14;
_1.0 = _18.0;
_8 = [(-27140_i16),16461_i16,(-17429_i16),19024_i16];
_22.fld2 = [_7.2,_7.2,_7.2,_7.2];
_7.2 = 170_u8 + 141_u8;
RET = _7.0;
_23 = _19 + _7.1;
_18 = (_15,);
_1 = (_6.0,);
_22.fld3 = [32318_i16,8631_i16,13033_i16,(-22665_i16)];
_2 = 31_i8 as isize;
_2 = _12;
_21 = _2;
Goto(bb9)
}
bb9 = {
_13 = _17;
_3 = [5613656424552175192_i64,8388204032824607925_i64,1142774113258255267_i64,(-2858064043729307453_i64),(-7141887136582845514_i64),(-8795836212399427573_i64),(-2747654663216185762_i64),5465934454453586701_i64];
_10 = [61996172940587366359502553162024997279_u128,63072440716412751182716360125069126438_u128];
_20 = core::ptr::addr_of_mut!(_10);
Call(_25 = fn11(_21, _1, _12, _20, _15, _5, _18, _18, _1, _18, _4, _4, _18, _6.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_20) = [289005516902112741822833358888998890288_u128,188191502437225211891711878620140936355_u128];
_16 = _2;
_24.0 = _3;
_23 = _7.1;
_25 = (-3313_i16) << _12;
(*_20) = [186564950387278250005556038742816702824_u128,163707302930547581505093783100072925935_u128];
_16 = _2;
_1 = _6;
_26.0.0 = _17;
_18 = (_24.0,);
RET = _14;
_26.0.1 = core::ptr::addr_of!(_27);
_26.0.0 = _13;
_3 = [5701383919773994356_i64,(-5180673470468656564_i64),1479089603947910267_i64,6098799543603916405_i64,(-9009675603373131176_i64),(-4869503200090393081_i64),5559695746666423829_i64,5510875971906123419_i64];
_9 = (-1204498370_i32) as isize;
_7.0 = [10279_u16];
_16 = (-45_i8) as isize;
_3 = [4137248754067425340_i64,1903790620678503327_i64,3381220879413153097_i64,4161598998248742328_i64,(-5359803193093680526_i64),(-3760016885544420_i64),(-4402547443040589764_i64),(-72796084482751568_i64)];
(*_20) = [230044977842322839803065851206457017596_u128,2364917766496062909946703977107961780_u128];
Goto(bb11)
}
bb11 = {
_3 = [8646087990834977828_i64,1450301581381486257_i64,454482977259798168_i64,(-3618737627817853401_i64),(-4792344403298903347_i64),4958220519211770100_i64,4863949751371520524_i64,(-4747598736119181614_i64)];
_19 = -_23;
_27 = 24717828136319099804553829217698443194_u128 >> _21;
RET = [11507_u16];
RET = [61227_u16];
_7.0 = _14;
_26.0.2 = 1773668028_u32;
_20 = core::ptr::addr_of_mut!((*_20));
_28 = _22.fld2;
_22.fld2 = [_7.2,_7.2,_7.2,_7.2];
_24.0 = _18.0;
_26.0.2 = 3468421647_u32;
Goto(bb12)
}
bb12 = {
_27 = 280360970466992528097177965319746706800_u128;
_28 = [_7.2,_7.2,_7.2,_7.2];
_12 = _25 as isize;
_24.0 = _1.0;
_30 = _2 | _9;
_7.2 = 36_i8 as u8;
_29 = _30 - _4;
_8 = [_25,_25,_25,_25];
_2 = !_29;
_2 = _29 << _25;
_7.2 = 98_u8;
_26.0.0 = _13;
RET = [63853_u16];
_26.0.3 = 22995_u16 ^ 34382_u16;
_18 = (_3,);
_22.fld2 = [_7.2,_7.2,_7.2,_7.2];
_18 = _24;
RET = [_26.0.3];
_13 = _26.0.0;
(*_20) = [_27,_27];
_23 = -_7.1;
_26.0.2 = 2181053317_u32;
Goto(bb13)
}
bb13 = {
_13 = _26.0.0;
_11 = core::ptr::addr_of!(_26.0.2);
(*_11) = 10143814528485453750_u64 as u32;
_8 = [_25,_25,_25,_25];
_27 = 63547298790348679309182378033810040673_u128;
Goto(bb14)
}
bb14 = {
_5 = [5185992614162801595_i64,7081246216127506533_i64,7875384162885481402_i64,(-7949692971474543149_i64),5346582398820260689_i64,(-5071787780085099123_i64),7521268922106384818_i64,2824278340680428274_i64];
_11 = core::ptr::addr_of!(_26.0.2);
_6.0 = [6898186544445788481_i64,8286117818139641121_i64,(-8980768218308744369_i64),(-4874111429965858744_i64),7787824313453478203_i64,(-6804160994456688456_i64),(-6369422211808461963_i64),(-8571420746928497823_i64)];
_30 = -_29;
_32.0 = _6.0;
(*_20) = [_27,_27];
_30 = !_2;
_18.0 = [4326793512556253382_i64,(-1094921845596404068_i64),5660517350517266018_i64,5959560884563838267_i64,(-3445153826880331591_i64),643495901544382689_i64,(-3977682796194517852_i64),8050666657437464005_i64];
_6.0 = _3;
_27 = !61620620585342793045757549622936656186_u128;
_3 = [5423489564265273576_i64,9048181784905232777_i64,4721849654225268040_i64,(-8263273371733259036_i64),4087230056578691135_i64,7442685045961990096_i64,(-3761660612077901443_i64),2908697955677944541_i64];
_1 = (_6.0,);
_35.2 = !_7.2;
_32.0 = _3;
_34 = Adt51::Variant0 { fld0: _26.0.3,fld1: RET,fld2: _26.0 };
RET = Field::<[u16; 1]>(Variant(_34, 0), 1);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(10_usize, 30_usize, Move(_30), 27_usize, Move(_27), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(10_usize, 28_usize, Move(_28), 17_usize, Move(_17), 25_usize, Move(_25), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(10_usize, 15_usize, Move(_15), 6_usize, Move(_6), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: ([i64; 8],),mut _3: isize,mut _4: *mut [u128; 2],mut _5: [i64; 8],mut _6: [i64; 8],mut _7: ([i64; 8],),mut _8: ([i64; 8],),mut _9: ([i64; 8],),mut _10: ([i64; 8],),mut _11: isize,mut _12: isize,mut _13: ([i64; 8],),mut _14: [i64; 8]) -> i16 {
mir! {
type RET = i16;
let _15: ([i64; 8],);
let _16: f32;
let _17: i32;
let _18: ([i64; 8],);
let _19: u32;
let _20: [usize; 2];
let _21: i64;
let _22: [u16; 1];
let _23: f32;
let _24: char;
let _25: bool;
let _26: f32;
let _27: char;
let _28: [i64; 8];
let _29: ();
let _30: ();
{
_10 = (_9.0,);
_13.0 = [1863180542070760210_i64,6403867116082237156_i64,(-6936365981740587330_i64),(-7945023227500494052_i64),(-1601801709258832590_i64),(-2874998023466217981_i64),3801855749123255383_i64,1273172889302564314_i64];
_4 = core::ptr::addr_of_mut!((*_4));
_2 = _7;
_2 = _13;
_13 = (_6,);
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = [42428655461121761551245371584555150940_u128,20453548028270479970811481585171553564_u128];
_9.0 = _5;
_8 = (_5,);
_13 = _8;
Call(RET = fn12(_13, _14, _10, _7.0, _13, _12, _2, _8, _5, _7.0, _1, _5, _1, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = core::ptr::addr_of_mut!((*_4));
_15.0 = [(-7920220152451431005_i64),(-371714932884699975_i64),3449866567638304687_i64,8332499051458686902_i64,9144871777997567297_i64,8771496326765372098_i64,(-1437393179659557935_i64),(-8592130539952732253_i64)];
_10 = (_6,);
_10 = (_5,);
_16 = (-226066025090435127_i64) as f32;
_11 = !_1;
Goto(bb2)
}
bb2 = {
_3 = -_12;
_7.0 = [(-6628927373672011831_i64),6322940633055972389_i64,(-5422208669302393334_i64),3200776540980221972_i64,3465600522825976736_i64,(-6571401031861735920_i64),8935926204013244858_i64,(-3833813931988327582_i64)];
_9 = _13;
_5 = [7336638129330783346_i64,4375417539016116077_i64,(-8859245221018385411_i64),3038537876495850965_i64,(-7025689773055268138_i64),(-1842939150253192462_i64),9085750576682711778_i64,(-2840778640530083419_i64)];
_17 = !(-700872138_i32);
_7 = (_14,);
_9 = _8;
(*_4) = [72888961604273330874095220419531117330_u128,186860295164407033366302560439617534577_u128];
_15.0 = _9.0;
_16 = 49_i8 as f32;
_2 = (_13.0,);
_11 = _12 << _1;
_15 = (_13.0,);
_2 = _10;
_5 = _2.0;
_18 = _8;
_1 = _12 ^ _11;
_13 = (_7.0,);
_6 = [5511620173658855328_i64,(-2229212762449505355_i64),111315776841815333_i64,(-2794317153278886017_i64),8556567422239220688_i64,3184564055909841193_i64,7980381341605984539_i64,(-2382895112987725915_i64)];
_21 = _16 as i64;
_15.0 = _9.0;
_1 = -_12;
_11 = _3;
_2 = (_8.0,);
_22 = [9505_u16];
_20 = [5_usize,4601496901682297864_usize];
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768181208 => bb10,
_ => bb9
}
}
bb3 = {
_4 = core::ptr::addr_of_mut!((*_4));
_15.0 = [(-7920220152451431005_i64),(-371714932884699975_i64),3449866567638304687_i64,8332499051458686902_i64,9144871777997567297_i64,8771496326765372098_i64,(-1437393179659557935_i64),(-8592130539952732253_i64)];
_10 = (_6,);
_10 = (_5,);
_16 = (-226066025090435127_i64) as f32;
_11 = !_1;
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
_14 = _8.0;
_2.0 = _5;
_8.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_24 = '\u{add78}';
_18 = (_13.0,);
_19 = 30_u8 as u32;
_9 = (_18.0,);
_15.0 = _2.0;
_14 = _2.0;
match RET {
0 => bb3,
1 => bb6,
340282366920938463463374607431768181208 => bb12,
_ => bb11
}
}
bb11 = {
_4 = core::ptr::addr_of_mut!((*_4));
_15.0 = [(-7920220152451431005_i64),(-371714932884699975_i64),3449866567638304687_i64,8332499051458686902_i64,9144871777997567297_i64,8771496326765372098_i64,(-1437393179659557935_i64),(-8592130539952732253_i64)];
_10 = (_6,);
_10 = (_5,);
_16 = (-226066025090435127_i64) as f32;
_11 = !_1;
Goto(bb2)
}
bb12 = {
_15 = (_9.0,);
_23 = _16;
(*_4) = [318018496539064807806099142434244846693_u128,32856965815835732254753364412757509515_u128];
_1 = _12 & _11;
_6 = [_21,_21,_21,_21,_21,_21,_21,_21];
match RET {
0 => bb1,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
340282366920938463463374607431768181208 => bb20,
_ => bb19
}
}
bb13 = {
_4 = core::ptr::addr_of_mut!((*_4));
_15.0 = [(-7920220152451431005_i64),(-371714932884699975_i64),3449866567638304687_i64,8332499051458686902_i64,9144871777997567297_i64,8771496326765372098_i64,(-1437393179659557935_i64),(-8592130539952732253_i64)];
_10 = (_6,);
_10 = (_5,);
_16 = (-226066025090435127_i64) as f32;
_11 = !_1;
Goto(bb2)
}
bb14 = {
_4 = core::ptr::addr_of_mut!((*_4));
_15.0 = [(-7920220152451431005_i64),(-371714932884699975_i64),3449866567638304687_i64,8332499051458686902_i64,9144871777997567297_i64,8771496326765372098_i64,(-1437393179659557935_i64),(-8592130539952732253_i64)];
_10 = (_6,);
_10 = (_5,);
_16 = (-226066025090435127_i64) as f32;
_11 = !_1;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_3 = -_12;
_7.0 = [(-6628927373672011831_i64),6322940633055972389_i64,(-5422208669302393334_i64),3200776540980221972_i64,3465600522825976736_i64,(-6571401031861735920_i64),8935926204013244858_i64,(-3833813931988327582_i64)];
_9 = _13;
_5 = [7336638129330783346_i64,4375417539016116077_i64,(-8859245221018385411_i64),3038537876495850965_i64,(-7025689773055268138_i64),(-1842939150253192462_i64),9085750576682711778_i64,(-2840778640530083419_i64)];
_17 = !(-700872138_i32);
_7 = (_14,);
_9 = _8;
(*_4) = [72888961604273330874095220419531117330_u128,186860295164407033366302560439617534577_u128];
_15.0 = _9.0;
_16 = 49_i8 as f32;
_2 = (_13.0,);
_11 = _12 << _1;
_15 = (_13.0,);
_2 = _10;
_5 = _2.0;
_18 = _8;
_1 = _12 ^ _11;
_13 = (_7.0,);
_6 = [5511620173658855328_i64,(-2229212762449505355_i64),111315776841815333_i64,(-2794317153278886017_i64),8556567422239220688_i64,3184564055909841193_i64,7980381341605984539_i64,(-2382895112987725915_i64)];
_21 = _16 as i64;
_15.0 = _9.0;
_1 = -_12;
_11 = _3;
_2 = (_8.0,);
_22 = [9505_u16];
_20 = [5_usize,4601496901682297864_usize];
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768181208 => bb10,
_ => bb9
}
}
bb18 = {
Return()
}
bb19 = {
_4 = core::ptr::addr_of_mut!((*_4));
_15.0 = [(-7920220152451431005_i64),(-371714932884699975_i64),3449866567638304687_i64,8332499051458686902_i64,9144871777997567297_i64,8771496326765372098_i64,(-1437393179659557935_i64),(-8592130539952732253_i64)];
_10 = (_6,);
_10 = (_5,);
_16 = (-226066025090435127_i64) as f32;
_11 = !_1;
Goto(bb2)
}
bb20 = {
_11 = 23160_u16 as isize;
_19 = 1449860145_u32 | 2639711795_u32;
_22 = [35011_u16];
_23 = _16 * _16;
_13 = _10;
_8.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_17 = (-1351000923_i32);
_8.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_13.0 = _15.0;
_8 = (_2.0,);
_13.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_25 = true;
_15 = _8;
_15 = _9;
(*_4) = [204242296008807517965111681908128385675_u128,230186371690736930519087403030968541646_u128];
_21 = 122289542883486489215468288019338945869_i128 as i64;
RET = 245_u8 as i16;
_7 = _9;
_17 = (-109_i8) as i32;
_1 = !_12;
_8.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_19 = 4166959547_u32;
_26 = _23 - _23;
_9 = _18;
RET = (-32082_i16);
(*_4) = [337982738472093885820341885717341911998_u128,318450031635000477847375809718833142912_u128];
_15.0 = _9.0;
_10.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
Goto(bb21)
}
bb21 = {
Call(_29 = dump_var(11_usize, 7_usize, Move(_7), 24_usize, Move(_24), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_29 = dump_var(11_usize, 19_usize, Move(_19), 22_usize, Move(_22), 25_usize, Move(_25), 10_usize, Move(_10)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_29 = dump_var(11_usize, 3_usize, Move(_3), 9_usize, Move(_9), 14_usize, Move(_14), 30_usize, _30), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: ([i64; 8],),mut _2: [i64; 8],mut _3: ([i64; 8],),mut _4: [i64; 8],mut _5: ([i64; 8],),mut _6: isize,mut _7: ([i64; 8],),mut _8: ([i64; 8],),mut _9: [i64; 8],mut _10: [i64; 8],mut _11: isize,mut _12: [i64; 8],mut _13: isize,mut _14: isize) -> i16 {
mir! {
type RET = i16;
let _15: (char, *const u128, u32, u16);
let _16: isize;
let _17: [i64; 8];
let _18: Adt53;
let _19: isize;
let _20: isize;
let _21: *const u128;
let _22: ([u16; 1], f64, u8);
let _23: [i16; 4];
let _24: f64;
let _25: char;
let _26: isize;
let _27: [u128; 2];
let _28: f64;
let _29: u32;
let _30: ([u16; 1], f64, u8);
let _31: ();
let _32: ();
{
_6 = _14 << _14;
_2 = [618906380439877249_i64,(-52476435795913646_i64),2610079973452415812_i64,(-5679201386217403424_i64),1161414526982037086_i64,(-1733722700275725549_i64),7439844042904005859_i64,(-9015577671698480561_i64)];
_5.0 = [(-3623117872883971112_i64),7327871739246367913_i64,8666230302008040438_i64,(-787838152563768796_i64),(-5369247282637173218_i64),(-5061447807905360672_i64),(-1249500921895829984_i64),5444929047614983316_i64];
_1 = (_5.0,);
_8 = _3;
_5.0 = [5695399496517639688_i64,3482548070623541927_i64,(-5914283250157949224_i64),(-7024192656085649216_i64),5527981292450954674_i64,(-2187761835241500581_i64),(-1164332560722318612_i64),6589044447373301625_i64];
_8.0 = [(-4078279308428888075_i64),(-9140978761696224797_i64),(-8108679639005755937_i64),(-8954443624319071152_i64),6071691212616942372_i64,(-77048279042173554_i64),(-1953306044324213641_i64),7442882600175545552_i64];
_6 = '\u{4df3a}' as isize;
RET = 2571_i16;
_3.0 = [(-7933121338348935014_i64),(-4608960699087316325_i64),(-435765919395094213_i64),(-6651504060589636077_i64),2362897645826623500_i64,(-7232496413090477362_i64),7766842094690498663_i64,(-1725311835873286353_i64)];
_15.0 = '\u{3dabf}';
_13 = RET as isize;
_6 = _11;
Call(_15.2 = fn13(_7.0, _1, _8, _5.0, _2, _14, _5.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.0 = [5939989827670138474_i64,(-1596235449814442086_i64),(-3062829021767022199_i64),(-2061186702698668569_i64),(-8356244643827398290_i64),699348094207474210_i64,(-3541284988282521329_i64),(-2978557252973136229_i64)];
_16 = 6298441163246976464_i64 as isize;
_7 = (_2,);
_2 = [6457910128494006951_i64,3776740988902154521_i64,(-2170041914135820276_i64),(-4955301040750295275_i64),(-1403315988455459686_i64),(-3665039179043140643_i64),(-2196366605554802019_i64),7636976940973839811_i64];
_15.2 = !1944310839_u32;
_12 = _2;
_12 = [4663650128140268686_i64,842050675356544723_i64,(-7526566323149634721_i64),5967248643570323725_i64,7758619338158344319_i64,(-3116283928292617819_i64),2965919914385521435_i64,(-4075856358961497218_i64)];
_5 = (_8.0,);
_7 = _8;
_12 = _5.0;
_5 = (_12,);
_8.0 = _7.0;
_6 = -_16;
_19 = _11;
_10 = [(-5341981279723799019_i64),1885339232260020795_i64,(-4255192012860019736_i64),(-8656249029461027633_i64),4835394682167856843_i64,1201167167897636877_i64,(-8300497682944198669_i64),5640280545289935700_i64];
_15.3 = !6041_u16;
_11 = _19 * _13;
_6 = 316160906033694147764091920948056689020_u128 as isize;
_19 = _13;
_20 = !_14;
RET = 314289063967722478966384531708590446059_u128 as i16;
_16 = _11 - _14;
Goto(bb2)
}
bb2 = {
_10 = [(-3168641824820245911_i64),2164404106232417966_i64,6216792981349201116_i64,(-5870100853370239111_i64),7131367614203694431_i64,5091879950473079109_i64,201410782904050087_i64,527980436565966079_i64];
_8 = (_4,);
_8.0 = [627039813998339238_i64,2079112103914294932_i64,(-6854721063459442861_i64),3130910696766854463_i64,(-4935895036964526115_i64),(-739904551809914313_i64),(-136810682151258480_i64),(-4063032440988258192_i64)];
_7.0 = [(-3235923687076842438_i64),(-5777315400503903986_i64),1766033865430636905_i64,(-4706581788855843215_i64),1701055189446867381_i64,(-464144780466630531_i64),8321279941398380112_i64,(-6125681904408446701_i64)];
_22.1 = RET as f64;
_3 = _1;
_15.0 = '\u{d31cc}';
_22.2 = _16 as u8;
_9 = _2;
_11 = _6;
_8 = (_9,);
Goto(bb3)
}
bb3 = {
_1.0 = _8.0;
_15.3 = 34132_u16;
_3.0 = _9;
_14 = _19 & _20;
_23 = [RET,RET,RET,RET];
_12 = [9132116131786615000_i64,(-2085556260842586993_i64),(-7086758214145423941_i64),5136079903995617164_i64,(-3316232413275500005_i64),(-1553273443401088440_i64),1840026352306819359_i64,(-6095512272305464727_i64)];
match _15.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
34132 => bb9,
_ => bb8
}
}
bb4 = {
_10 = [(-3168641824820245911_i64),2164404106232417966_i64,6216792981349201116_i64,(-5870100853370239111_i64),7131367614203694431_i64,5091879950473079109_i64,201410782904050087_i64,527980436565966079_i64];
_8 = (_4,);
_8.0 = [627039813998339238_i64,2079112103914294932_i64,(-6854721063459442861_i64),3130910696766854463_i64,(-4935895036964526115_i64),(-739904551809914313_i64),(-136810682151258480_i64),(-4063032440988258192_i64)];
_7.0 = [(-3235923687076842438_i64),(-5777315400503903986_i64),1766033865430636905_i64,(-4706581788855843215_i64),1701055189446867381_i64,(-464144780466630531_i64),8321279941398380112_i64,(-6125681904408446701_i64)];
_22.1 = RET as f64;
_3 = _1;
_15.0 = '\u{d31cc}';
_22.2 = _16 as u8;
_9 = _2;
_11 = _6;
_8 = (_9,);
Goto(bb3)
}
bb5 = {
_3.0 = [5939989827670138474_i64,(-1596235449814442086_i64),(-3062829021767022199_i64),(-2061186702698668569_i64),(-8356244643827398290_i64),699348094207474210_i64,(-3541284988282521329_i64),(-2978557252973136229_i64)];
_16 = 6298441163246976464_i64 as isize;
_7 = (_2,);
_2 = [6457910128494006951_i64,3776740988902154521_i64,(-2170041914135820276_i64),(-4955301040750295275_i64),(-1403315988455459686_i64),(-3665039179043140643_i64),(-2196366605554802019_i64),7636976940973839811_i64];
_15.2 = !1944310839_u32;
_12 = _2;
_12 = [4663650128140268686_i64,842050675356544723_i64,(-7526566323149634721_i64),5967248643570323725_i64,7758619338158344319_i64,(-3116283928292617819_i64),2965919914385521435_i64,(-4075856358961497218_i64)];
_5 = (_8.0,);
_7 = _8;
_12 = _5.0;
_5 = (_12,);
_8.0 = _7.0;
_6 = -_16;
_19 = _11;
_10 = [(-5341981279723799019_i64),1885339232260020795_i64,(-4255192012860019736_i64),(-8656249029461027633_i64),4835394682167856843_i64,1201167167897636877_i64,(-8300497682944198669_i64),5640280545289935700_i64];
_15.3 = !6041_u16;
_11 = _19 * _13;
_6 = 316160906033694147764091920948056689020_u128 as isize;
_19 = _13;
_20 = !_14;
RET = 314289063967722478966384531708590446059_u128 as i16;
_16 = _11 - _14;
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
_8 = (_3.0,);
_3 = (_5.0,);
_15.0 = '\u{c747c}';
_5.0 = [817738306426948798_i64,(-8096283793202065557_i64),(-6515745175512941820_i64),1474504363895510761_i64,3707542400916683633_i64,2776441547660852018_i64,(-4445967642688380809_i64),1957605426485389131_i64];
_2 = [(-4508388645227635795_i64),2720422026018142959_i64,(-6282093563509392237_i64),(-7515362214810215791_i64),6280094585466304288_i64,(-3899433815428126251_i64),5229677177619252582_i64,(-4186554032941624815_i64)];
_7.0 = [(-3607346431852179671_i64),(-2791335210446813985_i64),(-1258935772577397437_i64),(-8119125113347005938_i64),(-479121706457242688_i64),(-3102135075002740603_i64),(-2682496098630306314_i64),7407256720319843210_i64];
_17 = [397763449357065727_i64,1321812012857173837_i64,5930932133043353062_i64,(-1970157416104672674_i64),(-2846433852937190583_i64),7465882028781013146_i64,(-3430935291978015877_i64),7361900145988233412_i64];
RET = (-30248_i16);
match _15.3 {
0 => bb3,
34132 => bb11,
_ => bb10
}
}
bb10 = {
_1.0 = _8.0;
_15.3 = 34132_u16;
_3.0 = _9;
_14 = _19 & _20;
_23 = [RET,RET,RET,RET];
_12 = [9132116131786615000_i64,(-2085556260842586993_i64),(-7086758214145423941_i64),5136079903995617164_i64,(-3316232413275500005_i64),(-1553273443401088440_i64),1840026352306819359_i64,(-6095512272305464727_i64)];
match _15.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
34132 => bb9,
_ => bb8
}
}
bb11 = {
_11 = -_14;
_12 = [295060860321973755_i64,(-7237440655975898673_i64),(-6953198563892128873_i64),(-8214277870196680099_i64),(-4618511882497273719_i64),3345950744589016483_i64,2937834384453806664_i64,8729483154109525753_i64];
_15.2 = 750866665_u32 ^ 109715457_u32;
_14 = _13 & _16;
_8.0 = _5.0;
_1 = (_10,);
_19 = !_14;
_5.0 = _9;
_6 = _19;
_8.0 = _1.0;
match RET {
0 => bb5,
340282366920938463463374607431768181208 => bb12,
_ => bb10
}
}
bb12 = {
_24 = _11 as f64;
_16 = _11 & _19;
_25 = _15.0;
_27 = [53017078638951913801757493911965933735_u128,60518457457527884862264177204851790410_u128];
_22.2 = 8573995444785691766_i64 as u8;
_23 = [RET,RET,RET,RET];
_24 = 2437662527036885853_u64 as f64;
_23 = [RET,RET,RET,RET];
_22.1 = -_24;
match _15.3 {
0 => bb10,
1 => bb2,
34132 => bb13,
_ => bb9
}
}
bb13 = {
_4 = _8.0;
_15.3 = !53660_u16;
_4 = _9;
_25 = _15.0;
_7 = (_5.0,);
_6 = -_16;
_8 = (_17,);
_8.0 = [(-1070679556200533467_i64),804400885352184347_i64,747267689418157957_i64,2372237430299283663_i64,7348465829244654349_i64,(-4052330011698379639_i64),(-6243038057878767595_i64),7739208989099366813_i64];
match RET {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
340282366920938463463374607431768181208 => bb20,
_ => bb19
}
}
bb14 = {
Return()
}
bb15 = {
_11 = -_14;
_12 = [295060860321973755_i64,(-7237440655975898673_i64),(-6953198563892128873_i64),(-8214277870196680099_i64),(-4618511882497273719_i64),3345950744589016483_i64,2937834384453806664_i64,8729483154109525753_i64];
_15.2 = 750866665_u32 ^ 109715457_u32;
_14 = _13 & _16;
_8.0 = _5.0;
_1 = (_10,);
_19 = !_14;
_5.0 = _9;
_6 = _19;
_8.0 = _1.0;
match RET {
0 => bb5,
340282366920938463463374607431768181208 => bb12,
_ => bb10
}
}
bb16 = {
_1.0 = _8.0;
_15.3 = 34132_u16;
_3.0 = _9;
_14 = _19 & _20;
_23 = [RET,RET,RET,RET];
_12 = [9132116131786615000_i64,(-2085556260842586993_i64),(-7086758214145423941_i64),5136079903995617164_i64,(-3316232413275500005_i64),(-1553273443401088440_i64),1840026352306819359_i64,(-6095512272305464727_i64)];
match _15.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
34132 => bb9,
_ => bb8
}
}
bb17 = {
_3.0 = [5939989827670138474_i64,(-1596235449814442086_i64),(-3062829021767022199_i64),(-2061186702698668569_i64),(-8356244643827398290_i64),699348094207474210_i64,(-3541284988282521329_i64),(-2978557252973136229_i64)];
_16 = 6298441163246976464_i64 as isize;
_7 = (_2,);
_2 = [6457910128494006951_i64,3776740988902154521_i64,(-2170041914135820276_i64),(-4955301040750295275_i64),(-1403315988455459686_i64),(-3665039179043140643_i64),(-2196366605554802019_i64),7636976940973839811_i64];
_15.2 = !1944310839_u32;
_12 = _2;
_12 = [4663650128140268686_i64,842050675356544723_i64,(-7526566323149634721_i64),5967248643570323725_i64,7758619338158344319_i64,(-3116283928292617819_i64),2965919914385521435_i64,(-4075856358961497218_i64)];
_5 = (_8.0,);
_7 = _8;
_12 = _5.0;
_5 = (_12,);
_8.0 = _7.0;
_6 = -_16;
_19 = _11;
_10 = [(-5341981279723799019_i64),1885339232260020795_i64,(-4255192012860019736_i64),(-8656249029461027633_i64),4835394682167856843_i64,1201167167897636877_i64,(-8300497682944198669_i64),5640280545289935700_i64];
_15.3 = !6041_u16;
_11 = _19 * _13;
_6 = 316160906033694147764091920948056689020_u128 as isize;
_19 = _13;
_20 = !_14;
RET = 314289063967722478966384531708590446059_u128 as i16;
_16 = _11 - _14;
Goto(bb2)
}
bb18 = {
Return()
}
bb19 = {
_3.0 = [5939989827670138474_i64,(-1596235449814442086_i64),(-3062829021767022199_i64),(-2061186702698668569_i64),(-8356244643827398290_i64),699348094207474210_i64,(-3541284988282521329_i64),(-2978557252973136229_i64)];
_16 = 6298441163246976464_i64 as isize;
_7 = (_2,);
_2 = [6457910128494006951_i64,3776740988902154521_i64,(-2170041914135820276_i64),(-4955301040750295275_i64),(-1403315988455459686_i64),(-3665039179043140643_i64),(-2196366605554802019_i64),7636976940973839811_i64];
_15.2 = !1944310839_u32;
_12 = _2;
_12 = [4663650128140268686_i64,842050675356544723_i64,(-7526566323149634721_i64),5967248643570323725_i64,7758619338158344319_i64,(-3116283928292617819_i64),2965919914385521435_i64,(-4075856358961497218_i64)];
_5 = (_8.0,);
_7 = _8;
_12 = _5.0;
_5 = (_12,);
_8.0 = _7.0;
_6 = -_16;
_19 = _11;
_10 = [(-5341981279723799019_i64),1885339232260020795_i64,(-4255192012860019736_i64),(-8656249029461027633_i64),4835394682167856843_i64,1201167167897636877_i64,(-8300497682944198669_i64),5640280545289935700_i64];
_15.3 = !6041_u16;
_11 = _19 * _13;
_6 = 316160906033694147764091920948056689020_u128 as isize;
_19 = _13;
_20 = !_14;
RET = 314289063967722478966384531708590446059_u128 as i16;
_16 = _11 - _14;
Goto(bb2)
}
bb20 = {
_2 = [(-4426061328711607914_i64),(-5659430467504159476_i64),8306824135414716522_i64,270994670590084839_i64,4448660987981989920_i64,5633568398755746411_i64,2775003746801315739_i64,5802591597365984599_i64];
_6 = _19 - _19;
_22.0 = [_15.3];
_7 = _1;
_27 = [75031423619013459369164815523070991698_u128,40846931564462386326612747052613026078_u128];
_22.1 = _24;
_29 = 264102657976240468498134815024393969301_u128 as u32;
_16 = _14 ^ _6;
Goto(bb21)
}
bb21 = {
Call(_31 = dump_var(12_usize, 23_usize, Move(_23), 9_usize, Move(_9), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_31 = dump_var(12_usize, 25_usize, Move(_25), 5_usize, Move(_5), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_31 = dump_var(12_usize, 14_usize, Move(_14), 6_usize, Move(_6), 2_usize, Move(_2), 32_usize, _32), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i64; 8],mut _2: ([i64; 8],),mut _3: ([i64; 8],),mut _4: [i64; 8],mut _5: [i64; 8],mut _6: isize,mut _7: [i64; 8]) -> u32 {
mir! {
type RET = u32;
let _8: [u16; 1];
let _9: Adt49;
let _10: Adt48;
let _11: u16;
let _12: Adt55;
let _13: [u128; 2];
let _14: isize;
let _15: i128;
let _16: isize;
let _17: &'static i64;
let _18: ([i64; 8],);
let _19: Adt43;
let _20: u16;
let _21: isize;
let _22: ([i64; 8],);
let _23: ([i64; 8],);
let _24: i16;
let _25: Adt50;
let _26: ();
let _27: ();
{
_1 = _4;
_2 = (_4,);
_7 = [3277739444429249371_i64,(-4477632558323164192_i64),(-8398423590310640760_i64),(-7859000133712988712_i64),343406876237123896_i64,3716071113270490059_i64,(-8344452907353847518_i64),(-7060833839309693100_i64)];
RET = 8556_u16 as u32;
_4 = [8953267432351608714_i64,5653960130688119234_i64,6434550361562671794_i64,9106621242894548040_i64,(-889496849044368753_i64),5872297578947824646_i64,(-6897289555322869888_i64),2753077225515255215_i64];
_8 = [51229_u16];
RET = 3975767823_u32;
RET = 564464795_u32;
_10.fld1.2 = 79_u8 + 64_u8;
_10.fld0 = core::ptr::addr_of_mut!(_10.fld2);
_10.fld1.0 = !RET;
_10.fld1.3 = _6;
RET = !_10.fld1.0;
_10.fld1.2 = 50_u8;
_2.0 = [83081152325372639_i64,7416428079213532461_i64,6792901517735941411_i64,3420535696736661859_i64,(-1045848448363309849_i64),1889390390003310641_i64,2613081558308425794_i64,(-4547343521793764419_i64)];
_2 = (_1,);
_1 = [3361305353910255702_i64,(-3344030448587584700_i64),1611421319473246822_i64,(-1957022944342464342_i64),(-4777110434533320826_i64),1035109876773078053_i64,(-949171073056031430_i64),7978383089719866542_i64];
_4 = _5;
RET = _10.fld1.0 ^ _10.fld1.0;
_10.fld2 = [235330956980271214584644558219316485381_u128,135766313448475357363633897294097700441_u128];
_2.0 = _7;
RET = !_10.fld1.0;
match _10.fld1.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
50 => bb9,
_ => bb8
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
Return()
}
bb9 = {
_2.0 = _5;
_3 = (_4,);
_5 = [(-7757224073961638602_i64),6475556860410777918_i64,3806961213385803982_i64,(-7597947092050106219_i64),520714543637635921_i64,6495863281106087990_i64,(-5834429079291066810_i64),5959578030094713965_i64];
_8 = [16485_u16];
_10.fld1.2 = 237_u8;
_10.fld1.2 = (-79976747950258066249479520943294272810_i128) as u8;
_8 = [61348_u16];
_2 = (_5,);
Call(_10.fld1.2 = fn14(_10.fld1.3, _5, _6, _6, _3.0, _1, _3, _3, _5, _6, _5, _3.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = _7;
_3.0 = _4;
_4 = [(-8889526247048917010_i64),3529248170622731492_i64,(-3067710895437857429_i64),1895987991683350838_i64,(-396091846430484757_i64),(-5152542159116798077_i64),4707110840667446669_i64,(-5206847999204311061_i64)];
_7 = [(-6089931385259756087_i64),(-1135125173020268132_i64),3464930798437303814_i64,(-7048040175963733316_i64),(-7895425141239510230_i64),(-7157193917523302950_i64),1097031282884543446_i64,1987108910162894_i64];
_7 = _2.0;
_5 = _1;
_10.fld0 = core::ptr::addr_of_mut!(_13);
_8 = [20063_u16];
RET = _10.fld1.0;
Goto(bb11)
}
bb11 = {
_1 = _7;
_10.fld1.1 = [(-6223384620807073586_i64),1672051460087257191_i64,4468204499935191845_i64,(-4271504586944389453_i64),7786426502696145678_i64,4011803284375415690_i64,3566954057657630057_i64,(-3863307470429820989_i64)];
_10.fld1.0 = _6 as u32;
_2.0 = [660716085418907645_i64,3002461012179697683_i64,178416537464603847_i64,(-4301803536056298201_i64),9105337882677718703_i64,(-7459096181948002146_i64),(-2877327463058872607_i64),9122776175363479778_i64];
_10.fld1.3 = 64927_u16 as isize;
_3.0 = [(-4151043982608266559_i64),7136350653054799422_i64,(-6182067524051793939_i64),(-3662444944662440299_i64),3286025614162066355_i64,(-4063764399305312328_i64),(-7100497110324690891_i64),1252600468353434882_i64];
_11 = 42469_u16 | 55880_u16;
_8 = [_11];
_7 = [(-7308231543010647512_i64),(-7528036833631407584_i64),3530195223319122857_i64,6157186175683283191_i64,3771036599642588627_i64,(-7458595392832013301_i64),4993507747422244660_i64,(-6205727020992238191_i64)];
_14 = '\u{75424}' as isize;
_7 = [(-3774518636644508928_i64),(-7985439084534363075_i64),639816857796304679_i64,5039897611648628569_i64,1997743207875350178_i64,8693726899945614960_i64,7119878396544431212_i64,3285615487095079653_i64];
Goto(bb12)
}
bb12 = {
_2 = (_3.0,);
_13 = [139941706283016166674882955356824010932_u128,205124475013948878548407519466797458502_u128];
_13 = _10.fld2;
_15 = 91502674913784732159268801203257878825_i128 >> _6;
_10.fld1.1 = [9158214238548525746_i64,1781572588900792928_i64,2909602815836603021_i64,331339809211364469_i64,8581413568597501195_i64,6849397520995923169_i64,(-1575965087107314985_i64),6374307967504101965_i64];
RET = _10.fld1.0;
_7 = [(-8951092229987233178_i64),(-7158960282961796870_i64),3386585786416958515_i64,(-3035603521804474041_i64),(-7135050823017571343_i64),(-3711556987432720450_i64),(-1102224481460467721_i64),8489840627498777594_i64];
_13 = _10.fld2;
_16 = _14 << _6;
_10.fld1.1 = [(-6835997226778501429_i64),2847556816179049789_i64,6584216770045559076_i64,9216813308222129143_i64,252679509825261689_i64,1488017380426412326_i64,751699848523827823_i64,(-7967637775625325275_i64)];
_2.0 = _3.0;
_16 = _14;
_5 = [4317946736664696986_i64,3721579740013572276_i64,(-150573663891999320_i64),2342099695481704800_i64,(-1068247196825146037_i64),7827809723355922292_i64,1848400948908920412_i64,(-7347841450648141652_i64)];
_7 = [5785585696389190513_i64,(-1851657573407117794_i64),370420197497438003_i64,7791449893233153507_i64,(-2673253074974052925_i64),1341694267037280889_i64,160710789892812112_i64,(-4863452232950085080_i64)];
_4 = [4898193618666346249_i64,7713765255214120063_i64,8733300305492976740_i64,(-8160065916801340604_i64),(-453551385408076066_i64),4743930162604088173_i64,(-5521179902859578275_i64),2624617125558292506_i64];
_8 = [_11];
_10.fld0 = core::ptr::addr_of_mut!(_10.fld2);
_14 = _6 + _10.fld1.3;
_4 = [9142870949913319857_i64,4136511528997535365_i64,6157928014808820370_i64,(-4864002209718667351_i64),(-1177923754228160717_i64),7735647872536370777_i64,2230287803802645527_i64,(-2761694993183524119_i64)];
_10.fld1.0 = !RET;
_1 = [(-6376405312488275678_i64),(-3001642520093018816_i64),(-5003582032543396829_i64),(-3746860518508877630_i64),6959395524839281663_i64,(-7527262680356637513_i64),3366186841531692705_i64,239336885601847984_i64];
_3 = (_5,);
_1 = _7;
_16 = !_14;
Goto(bb13)
}
bb13 = {
_15 = 4034402687050485904378467745282029115_i128;
_19.fld2 = [14752_i16,15082_i16,(-30150_i16),(-15568_i16)];
_19.fld0 = [_11];
_10.fld1.0 = !RET;
_4 = [(-691226323700354009_i64),89891084391271624_i64,2962368436128442983_i64,(-6029607174449685321_i64),(-454528715027360385_i64),512365560281359753_i64,4969330010110639598_i64,6281641986601450018_i64];
_4 = _3.0;
_7 = [(-446482954090248932_i64),9085656770235340136_i64,5179688791821503488_i64,30256596961367492_i64,(-2787390455974401930_i64),(-5084622174367051356_i64),(-3967102251003816003_i64),(-7901180769836391962_i64)];
_5 = [7157206200315950065_i64,4316102507400939688_i64,6708661115849984093_i64,(-3327140706423705090_i64),(-5299341919431535250_i64),(-8597150018610265294_i64),2103955799156994971_i64,4718197019669991341_i64];
_19.fld3.2 = !RET;
_10.fld1.0 = !_19.fld3.2;
_18.0 = [545161825573591710_i64,(-5064995716552527771_i64),(-1901672534282990638_i64),(-4779547027291976956_i64),7453006117602589320_i64,(-4977555882604071887_i64),(-6349522937025522709_i64),7633071926242906025_i64];
match _15 {
0 => bb8,
1 => bb2,
2 => bb7,
3 => bb11,
4 => bb5,
4034402687050485904378467745282029115 => bb14,
_ => bb6
}
}
bb14 = {
_4 = _10.fld1.1;
_10.fld2 = [72576239314800819376507083395538755487_u128,212597473723106554029522187983722376088_u128];
_2.0 = [(-766339720774873133_i64),(-6417148687168535127_i64),2526312265400163266_i64,(-739358017132867609_i64),(-3360920682608350989_i64),(-7257775607962621708_i64),1736237339803484718_i64,2146109997502511438_i64];
_21 = _14;
_3 = (_18.0,);
_3 = (_4,);
_22.0 = [(-7365420476074512892_i64),8922818496935438825_i64,(-7191794344848667248_i64),304900698969959998_i64,(-6347082506204887051_i64),(-719990482860623797_i64),(-1634156278072343954_i64),(-3997003716979034254_i64)];
_10.fld1.2 = !211_u8;
_19.fld1 = false as i16;
_3 = (_5,);
_5 = [8074225740934467522_i64,3827565624402909310_i64,900238654079947548_i64,1541502729921694275_i64,(-8871793583555298838_i64),(-2597718658224746858_i64),(-3087142478465758541_i64),(-8559388404694136524_i64)];
_19.fld3.3 = true as u16;
_10.fld0 = core::ptr::addr_of_mut!(_13);
_5 = _1;
_16 = _10.fld1.3 | _21;
_10.fld1.0 = true as u32;
_19.fld3.0 = '\u{d949d}';
_10.fld0 = core::ptr::addr_of_mut!(_13);
_10.fld1.4 = core::ptr::addr_of!(_19.fld1);
_6 = _14;
_10.fld1.3 = _14 & _6;
_3.0 = _4;
_24 = (-3739966543838512928_i64) as i16;
_1 = _10.fld1.1;
_1 = _5;
_19.fld3.3 = !_11;
_11 = !_19.fld3.3;
_25.fld2 = [_10.fld1.2,_10.fld1.2,_10.fld1.2,_10.fld1.2];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(13_usize, 8_usize, Move(_8), 4_usize, Move(_4), 14_usize, Move(_14), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(13_usize, 5_usize, Move(_5), 1_usize, Move(_1), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: [i64; 8],mut _3: isize,mut _4: isize,mut _5: [i64; 8],mut _6: [i64; 8],mut _7: ([i64; 8],),mut _8: ([i64; 8],),mut _9: [i64; 8],mut _10: isize,mut _11: [i64; 8],mut _12: [i64; 8]) -> u8 {
mir! {
type RET = u8;
let _13: ([u16; 1], f64, u8);
let _14: isize;
let _15: ([i64; 8],);
let _16: [usize; 2];
let _17: ([u16; 1], f64, u8);
let _18: ([i64; 8],);
let _19: [usize; 2];
let _20: ([i64; 8],);
let _21: [u8; 4];
let _22: isize;
let _23: isize;
let _24: [i64; 8];
let _25: isize;
let _26: char;
let _27: bool;
let _28: u128;
let _29: usize;
let _30: ([i64; 8],);
let _31: isize;
let _32: bool;
let _33: isize;
let _34: (*mut &'static i64,);
let _35: [usize; 2];
let _36: i128;
let _37: i128;
let _38: Adt50;
let _39: [usize; 2];
let _40: Adt57;
let _41: u32;
let _42: i16;
let _43: Adt56;
let _44: isize;
let _45: [u64; 5];
let _46: ();
let _47: ();
{
RET = 41_u8;
_8.0 = _12;
_1 = _3 >> _10;
_8.0 = _5;
_10 = 14740062933274553497_usize as isize;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
41 => bb5,
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
RET = 103_u8;
_5 = [1188051768030536567_i64,(-3058476237724742297_i64),2559072893711506292_i64,8875298498025941473_i64,561454560289568193_i64,(-144260605643602983_i64),(-934101633031798873_i64),(-2273549764753769725_i64)];
_11 = [829655253364157742_i64,(-5860268160763893603_i64),(-5175179080815365171_i64),(-1824303640589990327_i64),3750370013210534748_i64,4755285571436471507_i64,2723248855405640007_i64,8274729529131143195_i64];
_4 = _1;
_4 = _1 | _1;
_8 = (_11,);
_12 = _6;
_7 = _8;
_13.0 = [35743_u16];
_8 = (_2,);
RET = 7812565449378251498_u64 as u8;
Call(_3 = core::intrinsics::transmute(_4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14 = 11366688540884508096_u64 as isize;
_1 = 14813171473578609987_usize as isize;
_5 = [2115021804071705187_i64,5665793401420351784_i64,6038972881102425984_i64,7216347417008932152_i64,(-3106180944465386115_i64),9049514174618692114_i64,(-7375943281108522451_i64),(-8579918889515214069_i64)];
_9 = [(-507660820150414129_i64),(-2197427945595288864_i64),(-5765739108414198451_i64),3342273183623194500_i64,4029151526192708215_i64,1224200714451166594_i64,3863531826933871045_i64,(-6009505186440464442_i64)];
_8.0 = [6826224157597154070_i64,3082920652942579954_i64,(-4163404086767748079_i64),4562044618665353379_i64,(-8109489101086267123_i64),(-5237866857696575717_i64),(-4012936889269245415_i64),(-1224569177438498749_i64)];
_13.1 = 2360476449814598617819750129024338720_i128 as f64;
Call(_15.0 = core::intrinsics::transmute(_12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = '\u{3720c}' as isize;
RET = 9211976074198956142_u64 as u8;
_4 = 152667546799187651206865485160388368944_u128 as isize;
_15.0 = _11;
_6 = [8559971004626249217_i64,(-7434965638395479245_i64),(-4902841148570161211_i64),9104065397729542011_i64,(-6769005024474261547_i64),3768574355617979062_i64,(-6232242418558104824_i64),3534531863001417613_i64];
_13.2 = RET;
_18.0 = [5872776042705010251_i64,(-553885506657403301_i64),8140713746563031970_i64,(-6458418742584964418_i64),(-3327662789800203715_i64),7734932147637252356_i64,(-3273392891751618973_i64),1539738277132262507_i64];
_15.0 = [6158689007776789492_i64,4051591611037932082_i64,(-6413799623388531559_i64),(-6893068278622704586_i64),7457306653251582094_i64,122895401837443179_i64,7894277127931712109_i64,(-7893372304529626867_i64)];
_11 = _2;
_3 = 1038199301_i32 as isize;
_9 = [(-7402987898127540766_i64),(-2465016886402203945_i64),9026915134814442470_i64,(-4328079393716421779_i64),(-6364461485687247710_i64),(-5137308113108142301_i64),4441205280520870679_i64,791247876731335829_i64];
_17.0 = [24013_u16];
_8.0 = [(-3955994765827230138_i64),(-1588600311539086635_i64),8996521184953815152_i64,2349532013208624478_i64,(-7773620557383526695_i64),1274677781118855402_i64,3948982797222331953_i64,3487149830753677867_i64];
_19 = [1367036540104975754_usize,6_usize];
_19 = [16533655039936098074_usize,2_usize];
_17.1 = _13.1 - _13.1;
_17 = _13;
_8 = (_5,);
_11 = [(-8847335224621966141_i64),2453824216145024936_i64,4237276704379157773_i64,(-1019312576803207783_i64),(-2236460193390966820_i64),5539334181073169789_i64,(-3993699540675309872_i64),(-1252603643270088680_i64)];
_18.0 = [(-5025814233066484245_i64),(-8263080216907062660_i64),8913383572730769394_i64,9022863400066623183_i64,(-339175182237000451_i64),5728195832456606645_i64,(-6851198700925024982_i64),(-7011723432390533470_i64)];
Call(_3 = fn15(_18, _2, _11, _13.2, _17, _18.0, _12, _19), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = !_1;
_13.2 = !_17.2;
_17.2 = _13.2;
_15 = (_2,);
_19 = [17829432599224625872_usize,4_usize];
_11 = [(-7489306189627075627_i64),(-976610688086007324_i64),(-7869195476087912113_i64),(-2041479936476123288_i64),462124414661119617_i64,(-458727569647538817_i64),(-7452903608635795666_i64),5283619689666235221_i64];
_17.0 = _13.0;
_7 = _8;
_24 = _5;
_1 = _3 >> _3;
_20.0 = [(-6682805730202113977_i64),2261903499859836151_i64,7894813710529113044_i64,5249570803275117899_i64,641683801335438101_i64,(-9059034419902601617_i64),(-4243486943335807713_i64),2061539701200892763_i64];
_17 = _13;
_18 = (_9,);
_12 = _20.0;
_9 = [5250927576305749559_i64,8851115512863428530_i64,(-2110331922900895852_i64),8885041247302865644_i64,(-7598678457801479350_i64),1258063013713425367_i64,4742157075848096223_i64,(-216479478805254568_i64)];
_17 = _13;
_12 = [(-721989731788892323_i64),(-4063100298270624916_i64),(-5997602911453772658_i64),2908106304860363290_i64,6955154383091007523_i64,595052361422073908_i64,6822210606041635778_i64,3384110513457228034_i64];
_15 = (_9,);
_8 = (_20.0,);
_23 = _3 | _1;
_3 = _1;
_16 = _19;
_15.0 = [(-1114866652407789096_i64),7579699836169207879_i64,(-4990456480052938645_i64),2464341297685152777_i64,3010360417354139925_i64,(-1292693553305679816_i64),(-3270917067244057413_i64),(-7706578998565385183_i64)];
_29 = 7_usize;
Call(_4 = core::intrinsics::bswap(_23), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = [_15.0[_29],_11[_29],_24[_29],_11[_29],_8.0[_29],_7.0[_29],_5[_29],_18.0[_29]];
_14 = _29 as isize;
_11 = [_2[_29],_18.0[_29],_2[_29],_8.0[_29],_5[_29],_20.0[_29],_18.0[_29],_20.0[_29]];
match _7.0[_29] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463454794688542252997387 => bb10,
_ => bb8
}
}
bb10 = {
_8.0[_29] = _18.0[_29] >> _3;
_10 = _17.1 as isize;
_15.0 = _8.0;
_27 = !true;
_20 = _8;
_7 = (_24,);
RET = 17285298605982107765_u64 as u8;
_14 = _3;
_27 = _14 > _3;
_18 = _20;
_13.1 = 146870847399638209149025819904079245525_i128 as f64;
_18 = (_8.0,);
_16 = [_29,_29];
_32 = !_27;
_11[_29] = !_12[_29];
Goto(bb11)
}
bb11 = {
_3 = !_14;
_13.1 = _17.1;
_12[_29] = _11[_29] & _15.0[_29];
_6[_29] = _12[_29];
_22 = !_1;
_7.0[_29] = !_12[_29];
_36 = !(-77495085413737125734213544757869132227_i128);
_30.0 = [_6[_29],_11[_29],_15.0[_29],_18.0[_29],_12[_29],_15.0[_29],_18.0[_29],_15.0[_29]];
_18.0 = _8.0;
_26 = '\u{1788f}';
_5 = [_15.0[_29],_6[_29],_15.0[_29],_18.0[_29],_6[_29],_15.0[_29],_18.0[_29],_11[_29]];
_31 = (-2078484639_i32) as isize;
_15.0 = _20.0;
_13 = _17;
_7 = (_8.0,);
_13.1 = _17.1;
_35 = [_29,_29];
_15.0 = [_6[_29],_2[_29],_18.0[_29],_20.0[_29],_18.0[_29],_18.0[_29],_6[_29],_8.0[_29]];
_18 = (_15.0,);
_9[_29] = _30.0[_29] + _15.0[_29];
_30 = (_24,);
_16 = _19;
_17.2 = RET + RET;
_33 = !_1;
_10 = _33 & _3;
Call(_23 = core::intrinsics::bswap(_1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2[_29] = _9[_29] + _12[_29];
_24 = _20.0;
_8.0[_29] = _12[_29] - _2[_29];
_4 = _22 - _14;
_38.fld0 = Adt47::Variant0 { fld0: _15.0 };
_25 = _4;
_29 = 0_usize | 6_usize;
_27 = _4 <= _4;
_2 = [(-6347297188915947689_i64),3278302803376411936_i64,7285112246161068836_i64,2442585756109468006_i64,1016610516871843949_i64,5648048377914149950_i64,3277998670126443778_i64,(-5814584220191950152_i64)];
_22 = _1;
RET = (-29384_i16) as u8;
_29 = 6_usize ^ 14365330587751515271_usize;
_42 = RET as i16;
_33 = -_10;
_40 = Adt57::Variant0 { fld0: _17.2 };
_35 = _19;
_38.fld0 = Adt47::Variant0 { fld0: _6 };
_33 = _1 & _1;
Goto(bb13)
}
bb13 = {
place!(Field::<u8>(Variant(_40, 0), 0)) = !_17.2;
_15.0 = [5529340968202951828_i64,(-1517892884368254639_i64),(-3744282353852353528_i64),(-5930574831202605408_i64),(-5012958921481213254_i64),(-3272128676800007692_i64),3423770191870114910_i64,3657224670783161624_i64];
_11 = [8775428982358346139_i64,2444511608497506226_i64,(-3792442488694541000_i64),1178835825607275028_i64,4747473819998535528_i64,(-7588967655742234484_i64),457087471542839022_i64,(-4618607417772970133_i64)];
Goto(bb14)
}
bb14 = {
_6 = [6432891390975206038_i64,3474938075325816147_i64,(-6542728520376679732_i64),(-7004475682859670322_i64),584839430130380765_i64,7696389703139427969_i64,(-8673342303626287655_i64),(-5859626286007926213_i64)];
_38.fld3 = [_42,_42,_42,_42];
_21 = [RET,RET,RET,RET];
_30 = (_15.0,);
_37 = _36 | _36;
RET = _13.2 | _17.2;
_13 = (_17.0, _17.1, Field::<u8>(Variant(_40, 0), 0));
_17.0 = _13.0;
_36 = _37 << _33;
_41 = 104552886_u32 - 839019256_u32;
RET = _17.2 >> _4;
_13.2 = RET - RET;
_30 = (Field::<[i64; 8]>(Variant(_38.fld0, 0), 0),);
_22 = !_14;
place!(Field::<u8>(Variant(_40, 0), 0)) = _36 as u8;
SetDiscriminant(_38.fld0, 0);
_20.0 = [(-1259247147023013343_i64),7613382147768863564_i64,(-7611921843445162973_i64),(-2101755928056108176_i64),7044196749474187082_i64,6769292437444657940_i64,(-523035474660169308_i64),1015543640881356520_i64];
_36 = 171642641070575363418698845752935872110_u128 as i128;
_41 = !682942648_u32;
_20 = (_15.0,);
_39 = [_29,_29];
_21 = [_13.2,_13.2,_13.2,_13.2];
_22 = _25 - _14;
_17.2 = _13.2 >> _33;
RET = 4003906624416428787_i64 as u8;
_45 = [17473117568150088791_u64,12727771530941016281_u64,13408976820356953224_u64,6884456374151979219_u64,3820943946734999739_u64];
_9 = [(-8702225616493095396_i64),(-8222082991152681881_i64),3787501477705296985_i64,(-587505145224808053_i64),(-5700973330558838429_i64),339781612313833117_i64,(-71843850291995888_i64),(-4027285380957796322_i64)];
_22 = _14;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(14_usize, 1_usize, Move(_1), 37_usize, Move(_37), 42_usize, Move(_42), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(14_usize, 5_usize, Move(_5), 29_usize, Move(_29), 20_usize, Move(_20), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(14_usize, 27_usize, Move(_27), 24_usize, Move(_24), 4_usize, Move(_4), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(14_usize, 2_usize, Move(_2), 26_usize, Move(_26), 33_usize, Move(_33), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(14_usize, 8_usize, Move(_8), 31_usize, Move(_31), 47_usize, _47, 47_usize, _47), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: ([i64; 8],),mut _2: [i64; 8],mut _3: [i64; 8],mut _4: u8,mut _5: ([u16; 1], f64, u8),mut _6: [i64; 8],mut _7: [i64; 8],mut _8: [usize; 2]) -> isize {
mir! {
type RET = isize;
let _9: usize;
let _10: ([u16; 1], f64, u8);
let _11: f32;
let _12: [u8; 4];
let _13: f64;
let _14: [u128; 2];
let _15: f64;
let _16: char;
let _17: f32;
let _18: isize;
let _19: char;
let _20: isize;
let _21: [i16; 4];
let _22: char;
let _23: [u16; 1];
let _24: u8;
let _25: [u128; 2];
let _26: u64;
let _27: isize;
let _28: *mut [u128; 2];
let _29: [u8; 4];
let _30: u128;
let _31: *const usize;
let _32: char;
let _33: ((*mut &'static i64,), i8);
let _34: bool;
let _35: (u32, [i64; 8], u8, isize, *const i16);
let _36: [u128; 2];
let _37: u128;
let _38: Adt49;
let _39: ([u16; 1], f64, u8);
let _40: ([i64; 8],);
let _41: i128;
let _42: [u8; 4];
let _43: [usize; 2];
let _44: Adt55;
let _45: [u16; 1];
let _46: [u128; 2];
let _47: u128;
let _48: ();
let _49: ();
{
_5.0 = [45874_u16];
_5.1 = 333106911965772834442758981278471025235_u128 as f64;
_1.0 = _2;
_3 = [7279597927440997353_i64,(-1972510618101965182_i64),(-1485434762274937995_i64),(-8589251402488343503_i64),1626653929897530504_i64,3926947730326690899_i64,(-8247954035484593266_i64),(-2308410771594618488_i64)];
_9 = 16416132402241998987_usize | 15323016146058920347_usize;
_5.2 = _4 << _9;
RET = 9223372036854775807_isize;
_2 = [(-177261402736789712_i64),4660172799793569251_i64,2484164394392130348_i64,(-4528798801465881770_i64),(-1011517654794659020_i64),7000157262460568731_i64,(-587684337466909125_i64),(-4327397998145131535_i64)];
_9 = 3_usize;
_1.0 = [_6[_9],_6[_9],_2[_9],_2[_9],_6[_9],_6[_9],_3[_9],_6[_9]];
Goto(bb1)
}
bb1 = {
_3 = [_7[_9],_1.0[_9],_6[_9],_1.0[_9],_7[_9],_7[_9],_2[_9],_6[_9]];
_9 = 2_usize;
_7 = _1.0;
_7 = [_2[_9],_6[_9],_1.0[_9],_6[_9],_1.0[_9],_6[_9],_1.0[_9],_1.0[_9]];
_5.2 = 31829649_u32 as u8;
_3 = _6;
_6[_9] = !_2[_9];
_2[_9] = _6[_9];
_3 = [_7[_9],_1.0[_9],_1.0[_9],_7[_9],_1.0[_9],_1.0[_9],_6[_9],_6[_9]];
_1.0[_9] = _7[_9] ^ _3[_9];
_9 = !128627053086486913_usize;
_11 = RET as f32;
RET = 27261_i16 as isize;
_5.2 = _4;
_10.0 = _5.0;
_1.0 = _2;
_10.1 = _5.1 - _5.1;
_10 = (_5.0, _5.1, _5.2);
_5.0 = [39907_u16];
_10 = _5;
_10.1 = _9 as f64;
_10 = (_5.0, _5.1, _4);
_5.2 = _4;
Goto(bb2)
}
bb2 = {
_5.1 = _10.1 + _10.1;
_10.2 = _5.2;
_10.2 = !_5.2;
_3 = [(-9127495830095333136_i64),(-3297657038254914726_i64),(-3376430499759186333_i64),(-9195059366760522231_i64),2527509428265925187_i64,(-469852695619951346_i64),1590992703736457367_i64,530196944548487697_i64];
_2 = _1.0;
_3 = [(-783291099922339721_i64),(-4349266267259609788_i64),9220335080056285135_i64,(-7736952339626383156_i64),7359588849717325427_i64,2358109863258364153_i64,3859408994295813225_i64,(-5810250018267269346_i64)];
_5.0 = [1256_u16];
_5.0 = [5675_u16];
_10.2 = 307650125604413024702218379751966240593_u128 as u8;
_10.2 = _4;
_16 = '\u{4133}';
_6 = [(-3435528058600379582_i64),5626678196144526327_i64,(-1887971083895706275_i64),(-4096847412078865933_i64),3342202518430956311_i64,(-8152187947096843160_i64),(-5652091866593861519_i64),2058675513629629827_i64];
_10.0 = [7960_u16];
_10.1 = _5.1;
_4 = !_5.2;
_10.0 = [14267_u16];
Goto(bb3)
}
bb3 = {
_14 = [204584167823545637793623358500456285445_u128,4328193961083422033593037946431014109_u128];
_12 = [_5.2,_5.2,_10.2,_5.2];
_17 = 5158048270556159414_u64 as f32;
_5 = (_10.0, _10.1, _10.2);
_15 = -_10.1;
_13 = -_15;
_13 = _15;
Goto(bb4)
}
bb4 = {
_3 = _1.0;
_10.2 = !_4;
_5.1 = _9 as f64;
_1 = (_2,);
_7 = [(-8979464177653555110_i64),1588125744483896302_i64,(-2092971655112054958_i64),(-3950944789884321366_i64),(-4072578444834506364_i64),293849874295718092_i64,(-4552340370282801614_i64),(-8973680855295770798_i64)];
_10.2 = _5.2 | _4;
_10.0 = _5.0;
_10.1 = _15;
RET = !78_isize;
_2 = [(-7189592878669703926_i64),7993091886002973501_i64,7848229090810319939_i64,8358602213198680636_i64,(-2680888328316519130_i64),(-1295796683926065948_i64),(-3944633465993834513_i64),7916147636423709760_i64];
_10 = _5;
_21 = [(-15687_i16),(-4987_i16),(-9540_i16),12752_i16];
_18 = RET;
_12 = [_4,_10.2,_10.2,_10.2];
_17 = _13 as f32;
_5 = _10;
_19 = _16;
_5.1 = 5250092115987967437_u64 as f64;
Goto(bb5)
}
bb5 = {
_10.1 = _11 as f64;
_20 = _18 ^ _18;
_14 = [117190040274029197873665731350492251753_u128,46854647675283308480914814780038511905_u128];
_21 = [(-30137_i16),14517_i16,(-22704_i16),31090_i16];
_21 = [13795_i16,14362_i16,21809_i16,(-9997_i16)];
_17 = _11 + _11;
_7 = [8641058061666477545_i64,(-8838890026762822851_i64),(-7732056077396820688_i64),(-7664040475495013411_i64),3790829940166917324_i64,8458999322327128860_i64,(-8679653431567434845_i64),(-9179951774149599503_i64)];
_6 = [3752199328401069566_i64,722995373392914875_i64,5380847460325385727_i64,(-8653899925651253198_i64),(-5832290523316534051_i64),(-8348962081387960328_i64),(-699799356955257100_i64),2546718819647907637_i64];
_18 = !_20;
_4 = _5.2 ^ _5.2;
_15 = (-1920220349_i32) as f64;
_3 = _6;
_22 = _16;
_5 = (_10.0, _13, _10.2);
_8 = [_9,_9];
_22 = _16;
_10.2 = 1173277_i32 as u8;
_20 = -_18;
RET = !_20;
_25 = [21684743322268662506562854995604701697_u128,293488782320246649187215080975315024745_u128];
_6 = [(-7974065277585800531_i64),6469686810260594533_i64,(-3623812574274872705_i64),(-6336124221955779928_i64),(-6403424782377755458_i64),2546598352904225585_i64,8690459785062565070_i64,(-2818977748090853101_i64)];
_16 = _22;
Goto(bb6)
}
bb6 = {
_24 = _15 as u8;
RET = _20;
_3 = [(-4279169049910691799_i64),(-5021569996592051261_i64),8899190283323214554_i64,(-2595718190060259732_i64),318692911450605319_i64,5093306035009255285_i64,7622693990054820422_i64,8931006775712660542_i64];
_10 = (_5.0, _5.1, _4);
_10 = (_5.0, _5.1, _4);
_5.0 = _10.0;
_10.1 = 4602835941518527824_u64 as f64;
_1 = (_7,);
_7 = [(-6584267040689560044_i64),2076818183608646069_i64,(-4210000836028095599_i64),4630145929200940331_i64,7211473533980151627_i64,(-343062278112829410_i64),(-7697710498164425188_i64),(-3909622876770986612_i64)];
_5 = (_10.0, _15, _4);
_28 = core::ptr::addr_of_mut!(_14);
_13 = _15;
_26 = !230690136118425821_u64;
_5.2 = _24 | _24;
Goto(bb7)
}
bb7 = {
_29 = _12;
_26 = 7321743642174272628_u64;
_23 = _10.0;
_8 = [_9,_9];
_9 = !11267482685893521905_usize;
_7 = [(-5121632520218538584_i64),(-4553062667601943126_i64),7789046331678075896_i64,4071642286252573689_i64,2950186042350259990_i64,(-8531459245431181603_i64),(-3892073553765560410_i64),(-2810895428280932983_i64)];
_16 = _19;
Call(_31 = fn16(_5.2, _19, _1, _10.2, _2, _1, _20, _5, _22), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_27 = _20;
_10.1 = -_13;
_16 = _22;
_30 = 101826152756019071640080867954270156503_u128 + 253118042253930870055766374582859933948_u128;
_5.1 = _10.1;
_5 = (_10.0, _15, _10.2);
Goto(bb9)
}
bb9 = {
_32 = _22;
_6 = [4081803434452423707_i64,5300238491606570633_i64,869331316283507927_i64,(-5301734107252340830_i64),307498505135129746_i64,642564305880926638_i64,(-8834341595113575085_i64),6920070449331709506_i64];
_15 = _20 as f64;
_5.2 = _4;
_10.2 = _5.2 << _18;
_5.0 = [4688_u16];
_4 = _24;
_32 = _16;
_15 = 480381657_u32 as f64;
_5.0 = [58122_u16];
_18 = _20 | RET;
_6 = [(-3925371974912174861_i64),127069631921256049_i64,1471824913450252017_i64,8896122722864194432_i64,(-7260008627451101324_i64),3874135152001270174_i64,335344133896515171_i64,8359201866182480004_i64];
_5.2 = _30 as u8;
Goto(bb10)
}
bb10 = {
_18 = _20 * RET;
_33.1 = !(-60_i8);
_33.1 = !(-99_i8);
_10.1 = -_13;
_24 = _10.2;
_11 = -_17;
_10.1 = _15;
_7 = _1.0;
Goto(bb11)
}
bb11 = {
_5.1 = _13 + _15;
_1 = (_6,);
_20 = _18;
_14 = [_30,_30];
_34 = _9 != _9;
_5.0 = _23;
_5.2 = _24 + _24;
_4 = _26 as u8;
_34 = true ^ false;
_35.1 = [(-3763036653895712157_i64),(-5652798303508951017_i64),(-7524262111060843740_i64),(-3277216252785204659_i64),3242868000955115222_i64,6792683806417167626_i64,(-7512563994791422382_i64),7196352938770463211_i64];
_10.0 = _5.0;
RET = _18 * _27;
_8 = [_9,_9];
(*_28) = [_30,_30];
_5 = (_23, _15, _10.2);
_27 = _20 - _20;
_21 = [(-14196_i16),(-11490_i16),(-14320_i16),6521_i16];
Goto(bb12)
}
bb12 = {
_31 = core::ptr::addr_of!(_9);
_35.3 = _27;
_37 = _30;
_35.0 = 3799891762_u32 | 515842342_u32;
_35.0 = !3638556746_u32;
_33.1 = -18_i8;
_40 = _1;
_27 = _18 ^ _18;
_11 = -_17;
_39.0 = [13920_u16];
_1.0 = [5019864700279755033_i64,4143843752803468377_i64,(-2583190137071309941_i64),(-2873060177074474202_i64),1747693781025865333_i64,1393066475154302588_i64,5234203853723600594_i64,(-5070825530208291677_i64)];
_34 = !true;
_5.1 = -_15;
(*_31) = _33.1 as usize;
_1.0 = [5473713368794446934_i64,6357147008844327546_i64,8681323250042776576_i64,902378748275622227_i64,(-2535748555455446568_i64),(-5199756977731887422_i64),2202161149996205492_i64,5316858948579245883_i64];
(*_28) = [_30,_37];
_40 = (_35.1,);
_13 = _5.1 + _10.1;
_28 = core::ptr::addr_of_mut!((*_28));
(*_31) = !17948423276597518877_usize;
_23 = [19320_u16];
_30 = _5.1 as u128;
_14 = [_30,_30];
_26 = 13526858779648904784_u64;
_22 = _32;
(*_28) = [_37,_37];
_12 = _29;
Goto(bb13)
}
bb13 = {
_26 = 3861266313711905719_u64;
_29 = [_10.2,_10.2,_10.2,_5.2];
_13 = _35.0 as f64;
_5.2 = !_10.2;
_5.1 = 720371739078729506_i64 as f64;
_24 = _5.2 | _10.2;
RET = _27;
(*_31) = 7_usize - 7974351815081277587_usize;
_39.1 = _13;
_4 = _24;
_37 = _30 + _30;
_39.2 = _34 as u8;
_21 = [(-10057_i16),27480_i16,25946_i16,(-6530_i16)];
match _26 {
0 => bb7,
1 => bb8,
2 => bb3,
3 => bb4,
3861266313711905719 => bb14,
_ => bb11
}
}
bb14 = {
_16 = _19;
_36 = _25;
_43 = _8;
_5 = (_39.0, _10.1, _10.2);
_35.1 = _6;
_5.0 = [64594_u16];
_35.2 = !_24;
_28 = core::ptr::addr_of_mut!(_14);
_6 = [1806657451960099678_i64,(-3761626056116018825_i64),(-533148557944271278_i64),(-5665285636331804397_i64),8390189309520827068_i64,(-3886029586794278665_i64),6904688371499451736_i64,4536800501484603174_i64];
RET = -_35.3;
_5.2 = _24;
RET = -_35.3;
_36 = _14;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(15_usize, 36_usize, Move(_36), 6_usize, Move(_6), 9_usize, Move(_9), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(15_usize, 8_usize, Move(_8), 40_usize, Move(_40), 1_usize, Move(_1), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(15_usize, 14_usize, Move(_14), 23_usize, Move(_23), 20_usize, Move(_20), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(15_usize, 25_usize, Move(_25), 34_usize, Move(_34), 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u8,mut _2: char,mut _3: ([i64; 8],),mut _4: u8,mut _5: [i64; 8],mut _6: ([i64; 8],),mut _7: isize,mut _8: ([u16; 1], f64, u8),mut _9: char) -> *const usize {
mir! {
type RET = *const usize;
let _10: (char, *const u128, u32, u16);
let _11: f32;
let _12: Adt52;
let _13: *mut [usize; 2];
let _14: Adt46;
let _15: Adt47;
let _16: u32;
let _17: bool;
let _18: ([u16; 1], f64, u8);
let _19: Adt53;
let _20: isize;
let _21: isize;
let _22: [u64; 5];
let _23: isize;
let _24: f32;
let _25: i128;
let _26: *mut f32;
let _27: [i16; 4];
let _28: char;
let _29: [u128; 2];
let _30: (u32, [i64; 8], u8, isize, *const i16);
let _31: Adt49;
let _32: bool;
let _33: i64;
let _34: i8;
let _35: [i16; 4];
let _36: f64;
let _37: Adt42;
let _38: [u128; 2];
let _39: f64;
let _40: ();
let _41: ();
{
_6.0 = [(-6608593946561057747_i64),(-328556849928371951_i64),2338920066694324943_i64,(-1483701098949777807_i64),(-6306530792324210410_i64),(-5979623527099099310_i64),(-4756242735865545723_i64),(-2716027830672854510_i64)];
_8.1 = 4795_i16 as f64;
_4 = _1;
_1 = !_8.2;
_1 = _8.2 + _8.2;
_6 = (_3.0,);
_8.1 = 1015938266_i32 as f64;
_8.0 = [54231_u16];
_4 = _1;
_10.2 = 1717960848_u32;
match _10.2 {
1717960848 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_10.0 = _2;
_6.0 = [(-4093908255041944787_i64),6403717604010900091_i64,(-8999584409431071414_i64),(-499920605868925804_i64),5789327142333454085_i64,4530418400163586130_i64,278415106999513504_i64,(-6566821000219113949_i64)];
_1 = !_4;
_6 = (_3.0,);
_4 = _1 ^ _8.2;
_8.2 = !_1;
_6.0 = [(-3221295392730393075_i64),(-5411621910042770476_i64),5771411927867385033_i64,72955153465486446_i64,6543005006742678003_i64,7107744069780984789_i64,5073305654955754171_i64,(-7475431000801035290_i64)];
_8.0 = [60370_u16];
_10.2 = _1 as u32;
_1 = _4;
_6.0 = _5;
_10.2 = !4279560708_u32;
_10.2 = 1603175912_u32 - 3831452186_u32;
_10.0 = _2;
_2 = _10.0;
Call(_4 = fn17(_8, _8, _6, _6.0, _2, _6.0, _7, _6, _3.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _4;
_2 = _10.0;
_10.3 = 29682_u16 & 26341_u16;
_10.0 = _9;
_6.0 = _3.0;
_11 = 12378192430522441750_u64 as f32;
_4 = _8.2 * _1;
_8.2 = _4;
_3.0 = [7772687046807062958_i64,(-7103788032150903185_i64),(-762280690249097122_i64),8139137136755577355_i64,7102042494785360671_i64,(-684879228659894686_i64),(-4770432955111738178_i64),(-3404823716252000588_i64)];
_4 = _8.2;
Call(_7 = fn19(_8.2, _8, _1, _6.0, _6.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3.0 = _5;
_8.1 = 146125735573236154081941357598433251197_u128 as f64;
Goto(bb5)
}
bb5 = {
_3 = (_5,);
_1 = (-189157026_i32) as u8;
_6.0 = _3.0;
_8.0 = [_10.3];
_8.0 = [_10.3];
_8.0 = [_10.3];
_4 = _8.2;
_10.0 = _9;
_8.1 = 15867_i16 as f64;
_15 = Adt47::Variant0 { fld0: _5 };
_5 = [(-4252891432704891464_i64),(-1710676303866317673_i64),(-4599899589364128519_i64),(-2489571974156044491_i64),7856936944589068914_i64,6477384220874838245_i64,7139980684324464553_i64,(-3534053458630435921_i64)];
_8.0 = [_10.3];
_6.0 = _5;
_2 = _9;
_3.0 = [(-7242475962842045689_i64),(-4047515678426668202_i64),8121996152306805958_i64,2922880083394162673_i64,1216582509579132067_i64,3198237344209310268_i64,(-526385310279950759_i64),(-3231828386031739007_i64)];
_11 = 50_i8 as f32;
_9 = _2;
_16 = (-27617_i16) as u32;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = [(-6994161594404329367_i64),(-1234850321253896977_i64),(-369530587861138605_i64),6805449759503429242_i64,(-2722579550420662646_i64),3015191076463945108_i64,(-6011046918427580231_i64),(-7394803008714102813_i64)];
_6.0 = _5;
_3.0 = [91297960351704825_i64,(-2869634897753273851_i64),4951098819378452299_i64,(-1840391670351809728_i64),(-6317073376367194899_i64),7485838567479134615_i64,(-2224807501656137135_i64),5270249598172602556_i64];
_7 = _8.1 as isize;
Goto(bb6)
}
bb6 = {
_3.0 = [(-3676887901991046840_i64),6794142420340284993_i64,(-3116640046307107357_i64),9094869514633420983_i64,1573655800480769710_i64,6446936103554824672_i64,(-5720428031485608814_i64),(-6849784409613029662_i64)];
_10.3 = !5320_u16;
SetDiscriminant(_15, 0);
_4 = 324756473369618441801146512342727677716_u128 as u8;
_16 = !_10.2;
_2 = _9;
_3.0 = [(-7274619995548173782_i64),(-4898631719871177576_i64),(-2912376021416218764_i64),(-7257621485067301336_i64),(-715449470073421873_i64),(-5210996833677765934_i64),5919392986003587758_i64,(-3041907880876062443_i64)];
_1 = _8.2 << _8.2;
_18.2 = _8.2;
_16 = _10.2;
_8.2 = !_1;
Goto(bb7)
}
bb7 = {
_5 = _6.0;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = [3246325722662907122_i64,(-3124736476084994182_i64),7954319460522440685_i64,6522867760336731517_i64,2818334929270280119_i64,(-4518394129421743135_i64),(-5114666041365647944_i64),(-6595181893570908092_i64)];
_6.0 = Field::<[i64; 8]>(Variant(_15, 0), 0);
_4 = false as u8;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = _5;
SetDiscriminant(_15, 1);
_18.2 = _1 + _8.2;
_20 = _7 ^ _7;
_9 = _2;
place!(Field::<Adt43>(Variant(_15, 1), 0)).fld0 = _8.0;
_10.3 = 39427_u16 | 44022_u16;
place!(Field::<Adt43>(Variant(_15, 1), 0)).fld2 = [15650_i16,(-6922_i16),(-19457_i16),22583_i16];
_10.2 = !_16;
_18 = (Field::<Adt43>(Variant(_15, 1), 0).fld0, _8.1, _8.2);
place!(Field::<Adt43>(Variant(_15, 1), 0)).fld3.3 = _10.3;
_8.0 = [Field::<Adt43>(Variant(_15, 1), 0).fld3.3];
_9 = _2;
_15 = Adt47::Variant0 { fld0: _5 };
_18.2 = !_1;
_8.2 = _18.2 >> _1;
_8.0 = [_10.3];
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = _5;
_11 = _10.3 as f32;
Call(_18.1 = core::intrinsics::transmute(_20), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_8.1 = _18.1 * _18.1;
_8.2 = !_18.2;
_8.0 = [_10.3];
_17 = !false;
_2 = _9;
_8 = (_18.0, _18.1, _1);
_10.3 = !31056_u16;
_4 = 4128594727228466202_u64 as u8;
_12 = Adt52::Variant3 { fld0: _11,fld1: (-46979787751518111104090008600883338794_i128) };
Call(_8.1 = core::intrinsics::fmaf64(_18.1, _18.1, _18.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = _8.2 - _8.2;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = [(-1470774924116900711_i64),6647350869289402345_i64,6866734981746788363_i64,8141525795474032569_i64,(-227232000923885492_i64),4282402796354678582_i64,7972515489127216587_i64,8988487850718012892_i64];
_10.3 = _17 as u16;
place!(Field::<f32>(Variant(_12, 3), 0)) = (-81801299129657461986376560700157628542_i128) as f32;
_10.3 = 55348647141202119069973410725430466236_i128 as u16;
_12 = Adt52::Variant0 { fld0: _10.3,fld1: _2,fld2: 6372036014109336307_i64 };
_11 = (-353886785_i32) as f32;
_8.0 = [Field::<u16>(Variant(_12, 0), 0)];
_18.0 = _8.0;
_1 = _8.2 >> _18.2;
_20 = _7 | _7;
_9 = _2;
_17 = !false;
_1 = 9040745229713688815_usize as u8;
_8.1 = _18.1 * _18.1;
_6.0 = [(-3121390473875005465_i64),(-8126241275274851906_i64),4430279564503013197_i64,2982086994111652628_i64,2634106943217377004_i64,(-4719508109144832042_i64),(-7978030027747066200_i64),7482144127156198312_i64];
Call(_16 = core::intrinsics::transmute(Field::<char>(Variant(_12, 0), 1)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<u16>(Variant(_12, 0), 0)) = _10.3 - _10.3;
_20 = _7;
_8.1 = Field::<u16>(Variant(_12, 0), 0) as f64;
_21 = _20 & _20;
_1 = _8.2 | _8.2;
_10.3 = !Field::<u16>(Variant(_12, 0), 0);
Goto(bb11)
}
bb11 = {
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = [994857034937964567_i64,3306087862190146790_i64,(-8981730785036356498_i64),818666925709326248_i64,5524926674665175392_i64,(-7118293149515168365_i64),1905057709562201635_i64,(-4330660391821844369_i64)];
_1 = _4;
_23 = 4_usize as isize;
_1 = (-5469384430150568531_i64) as u8;
_11 = _8.2 as f32;
_18.1 = _8.1 * _8.1;
_21 = Field::<u16>(Variant(_12, 0), 0) as isize;
place!(Field::<i64>(Variant(_12, 0), 2)) = _21 as i64;
_18.2 = !_8.2;
_17 = !true;
_8.0 = _18.0;
SetDiscriminant(_15, 0);
_8 = (_18.0, _18.1, _18.2);
_23 = _20;
place!(Field::<i64>(Variant(_12, 0), 2)) = 5717936580141082458_i64;
_18.2 = _17 as u8;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = _5;
_22 = [3191707467925515469_u64,6394558197027553842_u64,3397325657751729389_u64,11001314715622786488_u64,1924892069169732721_u64];
_10.3 = Field::<u16>(Variant(_12, 0), 0);
_2 = _10.0;
_6 = _3;
_3 = (_5,);
_4 = !_8.2;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = [Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2),Field::<i64>(Variant(_12, 0), 2)];
_4 = _8.2 | _8.2;
_24 = _11;
_21 = _20;
_22 = [15410983725675350671_u64,12849542245076547189_u64,17536881721544222043_u64,12753927009003680259_u64,18405526892958233429_u64];
Goto(bb12)
}
bb12 = {
_18.2 = _4 + _4;
_27 = [(-5507_i16),(-23284_i16),(-28253_i16),(-4209_i16)];
_29 = [103495076437491723498805944604369562222_u128,208353945199258787759614138575241419875_u128];
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = _3.0;
place!(Field::<char>(Variant(_12, 0), 1)) = _2;
_10.0 = _9;
SetDiscriminant(_12, 1);
SetDiscriminant(_15, 1);
_30.2 = !_8.2;
_10.2 = _9 as u32;
_6.0 = [(-2565715396355350431_i64),(-1492294452598073267_i64),5841265747782529845_i64,539014202120890992_i64,4926033909198701316_i64,235220391926018612_i64,7131679713683193187_i64,(-6919194137475227677_i64)];
_30.3 = 1941194794_i32 as isize;
Goto(bb13)
}
bb13 = {
_4 = _18.2;
place!(Field::<isize>(Variant(_12, 1), 2)) = !_20;
_32 = !_17;
_30.0 = _10.2 | _10.2;
_3.0 = [(-3300190685819733512_i64),(-5922250686803730331_i64),9188576703805791984_i64,7243644986633917641_i64,8044562117485911195_i64,(-7135817938866757659_i64),(-519111867480792401_i64),3407736002301350943_i64];
place!(Field::<([u16; 1], f64, u8)>(Variant(_15, 1), 4)).0 = [_10.3];
_30.4 = core::ptr::addr_of!(place!(Field::<Adt43>(Variant(_15, 1), 0)).fld1);
_32 = _17 & _17;
Goto(bb14)
}
bb14 = {
_23 = !_21;
_30.0 = _16;
_3 = _6;
place!(Field::<([u16; 1], f64, u8)>(Variant(_15, 1), 4)).1 = _18.1;
_26 = core::ptr::addr_of_mut!(_24);
_3.0 = [(-2374584789137364342_i64),68466248668469464_i64,(-731096146976602088_i64),7941023924474685451_i64,3149336730087775489_i64,2110320148639737891_i64,5518301809552624675_i64,(-7217400564673909686_i64)];
Goto(bb15)
}
bb15 = {
place!(Field::<u32>(Variant(_15, 1), 2)) = _16;
_21 = -_30.3;
place!(Field::<usize>(Variant(_12, 1), 1)) = 1072661250950136854_usize;
_30.4 = core::ptr::addr_of!(place!(Field::<Adt43>(Variant(_15, 1), 0)).fld1);
_25 = 163066498127451150194642748638751637925_i128;
place!(Field::<([u16; 1], f64, u8)>(Variant(_15, 1), 4)) = (_8.0, _18.1, _18.2);
_8.0 = [_10.3];
_15 = Adt47::Variant0 { fld0: _3.0 };
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_12, 1), 1)));
place!(Field::<*mut i32>(Variant(_12, 1), 3)) = core::ptr::addr_of_mut!(_37.fld1);
_5 = [(-1145736292688733739_i64),(-2059549543025273799_i64),2303941794096558138_i64,2214836429970483793_i64,(-8656123744363770593_i64),4247395470919505179_i64,(-8166201081095023339_i64),(-8164039869188154395_i64)];
_6.0 = Field::<[i64; 8]>(Variant(_15, 0), 0);
(*RET) = !16758419600167601348_usize;
place!(Field::<[i64; 8]>(Variant(_15, 0), 0)) = _6.0;
_36 = Field::<usize>(Variant(_12, 1), 1) as f64;
_2 = _9;
_27 = [(-9829_i16),12154_i16,(-24886_i16),(-8747_i16)];
_35 = _27;
_4 = _18.2;
_22 = [17174448189576946240_u64,18237779219696631725_u64,949290166796757005_u64,3946526956711312551_u64,3671463455905896465_u64];
Goto(bb16)
}
bb16 = {
Call(_40 = dump_var(16_usize, 25_usize, Move(_25), 9_usize, Move(_9), 3_usize, Move(_3), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 6_usize, Move(_6), 35_usize, Move(_35), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(16_usize, 7_usize, Move(_7), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: ([u16; 1], f64, u8),mut _2: ([u16; 1], f64, u8),mut _3: ([i64; 8],),mut _4: [i64; 8],mut _5: char,mut _6: [i64; 8],mut _7: isize,mut _8: ([i64; 8],),mut _9: [i64; 8]) -> u8 {
mir! {
type RET = u8;
let _10: *mut f32;
let _11: [i16; 4];
let _12: &'static i64;
let _13: usize;
let _14: [u64; 5];
let _15: (*mut &'static i64,);
let _16: i64;
let _17: *mut [u128; 2];
let _18: i64;
let _19: u64;
let _20: isize;
let _21: u8;
let _22: isize;
let _23: u16;
let _24: f64;
let _25: char;
let _26: (u32, [i64; 8], u8, isize, *const i16);
let _27: f64;
let _28: f32;
let _29: ([i64; 8],);
let _30: bool;
let _31: [i64; 8];
let _32: [u16; 1];
let _33: ([i64; 8],);
let _34: f32;
let _35: Adt41;
let _36: Adt49;
let _37: [u8; 4];
let _38: [i64; 8];
let _39: ([u16; 1], f64, u8);
let _40: ([i64; 8],);
let _41: ();
let _42: ();
{
_5 = '\u{2db24}';
_2.0 = [33912_u16];
_3 = (_8.0,);
_2 = _1;
_5 = '\u{44900}';
_5 = '\u{aef3f}';
RET = !_1.2;
RET = _2.2 >> _7;
RET = _2.2 * _2.2;
_4 = [6294252709756225088_i64,(-5202686737308494487_i64),(-572678471113494982_i64),8093850786113359553_i64,7976526274043570523_i64,8585633502140347074_i64,(-3906448549219748323_i64),7520108721641794277_i64];
RET = (-5107813119033345257_i64) as u8;
_9 = [1225213285715705421_i64,(-6483566054956836410_i64),2304640243074644833_i64,(-2843809302252709456_i64),5549912212090588594_i64,2116989792718773077_i64,1977542259310118562_i64,(-2958657481184114485_i64)];
_2.0 = [23272_u16];
_2.2 = true as u8;
_4 = [7050360188409384109_i64,(-9126354726003750691_i64),(-4163369327706206480_i64),7580169293153664748_i64,6741041300863188669_i64,(-8479579465554070888_i64),189473498180368751_i64,(-1726297890052942814_i64)];
_3.0 = [(-7766703970539925067_i64),(-6671959060462361982_i64),2862481442781288582_i64,(-4043908286032472048_i64),(-8347283186670915316_i64),3019071047877125232_i64,(-1798644064957819590_i64),6261375725705602596_i64];
_2.2 = RET & _1.2;
_1 = _2;
_3 = (_6,);
_2.0 = [30650_u16];
_9 = _3.0;
_1.2 = 1653377213_i32 as u8;
_2 = _1;
_4 = [448232546177305835_i64,4901440950827822375_i64,(-190959557755137840_i64),(-880437799607184592_i64),6154699642270804369_i64,(-1728128667784108805_i64),2702628098762227416_i64,(-6012664072511996056_i64)];
_7 = _1.1 as isize;
_2 = (_1.0, _1.1, RET);
Goto(bb1)
}
bb1 = {
_3.0 = [(-7852963291443921928_i64),4468106087548713575_i64,(-1114473728826834933_i64),7813703398457491500_i64,(-470571145744160490_i64),(-1398881137877236478_i64),2924116488629006972_i64,(-1716356460386761583_i64)];
Goto(bb2)
}
bb2 = {
_1 = (_2.0, _2.1, _2.2);
_5 = '\u{ee1bf}';
_1 = (_2.0, _2.1, RET);
Goto(bb3)
}
bb3 = {
_4 = [(-7445010516774522389_i64),3294908888491674316_i64,(-7861221881284753970_i64),6201489425151504874_i64,(-8757386763053624133_i64),(-1977652264859979234_i64),6532229573509820814_i64,133057822211587803_i64];
_3 = (_8.0,);
_8.0 = [(-2088215638407219918_i64),(-3462252634877796366_i64),8683089115726074048_i64,3219074459275608_i64,(-3313110368593454990_i64),6986216538626321961_i64,(-5623401676643792654_i64),(-2750942796553887234_i64)];
_3.0 = _9;
_1.0 = _2.0;
_11 = [(-17121_i16),3790_i16,14636_i16,10624_i16];
_2.0 = [30879_u16];
_11 = [17953_i16,4589_i16,234_i16,(-5921_i16)];
_2.2 = RET >> RET;
_7 = !27_isize;
_3 = _8;
_1.0 = [13782_u16];
_11 = [(-14366_i16),19591_i16,22296_i16,(-15504_i16)];
RET = !_2.2;
_8 = _3;
_1.0 = [24924_u16];
Call(_2.1 = fn18(_3, _3, _7, _4, _9, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13 = 12648856252438493863_usize << _2.2;
_14 = [5564726092881848869_u64,17025811130243426723_u64,10487361160270383417_u64,10678180227807710736_u64,5767357834016843952_u64];
_7 = -39_isize;
_9 = [329823521336624848_i64,(-3815405554150829210_i64),4102873281820862265_i64,4683198720260093587_i64,(-702419750264449162_i64),725609615486095148_i64,(-8445590031097836865_i64),(-3171091811761239031_i64)];
_1.2 = 127448756054090421167452426661150384344_u128 as u8;
_13 = 2_usize - 3_usize;
_11 = [24009_i16,26783_i16,(-20649_i16),28740_i16];
_6 = _8.0;
_15.0 = core::ptr::addr_of_mut!(_12);
Goto(bb5)
}
bb5 = {
_12 = &_16;
_2 = (_1.0, _1.1, RET);
_1.2 = RET * RET;
_8 = (_3.0,);
_13 = 13978635983012051519_usize * 3_usize;
_8.0 = _6;
_1.0 = [31285_u16];
_8 = (_9,);
_2 = _1;
_1 = (_2.0, _2.1, _2.2);
_16 = !(-681946424844394957_i64);
_5 = '\u{d328b}';
_14 = [16353995886370174793_u64,2180023729044085978_u64,6862151968067440224_u64,13589939630007536623_u64,4107733704279846712_u64];
_12 = &_18;
_3.0 = _8.0;
_2 = _1;
RET = _1.2;
_1 = _2;
Goto(bb6)
}
bb6 = {
_4 = [_16,_16,_16,_16,_16,_16,_16,_16];
_14 = [602493319309852179_u64,4785949914381711963_u64,4514440713872284557_u64,12180201807631612572_u64,6146017818648582398_u64];
_18 = _16;
_20 = _7 >> _1.2;
_12 = &_16;
_19 = 25629_i16 as u64;
_1.0 = [26814_u16];
RET = _1.2 * _2.2;
_2.2 = 377557640_u32 as u8;
_15.0 = core::ptr::addr_of_mut!(_12);
Goto(bb7)
}
bb7 = {
_20 = 92970125076175144756330645492903378501_u128 as isize;
_3 = _8;
RET = (-2105306393_i32) as u8;
_2.2 = 316874254357679403607737839956044215807_u128 as u8;
_1.0 = [33892_u16];
_24 = 61_i8 as f64;
_4 = [(*_12),(*_12),_18,_18,_18,_18,_16,(*_12)];
_26.1 = _8.0;
Goto(bb8)
}
bb8 = {
RET = _1.2;
_2.2 = !RET;
_23 = RET as u16;
_2.0 = [_23];
_8 = (_3.0,);
_8 = (_26.1,);
_18 = 3173363963_u32 as i64;
_9 = [_16,(*_12),(*_12),(*_12),_18,(*_12),_16,(*_12)];
_26.3 = (-1852542899_i32) as isize;
_26.2 = RET;
_31 = _6;
_25 = _5;
_20 = _26.3 | _26.3;
Goto(bb9)
}
bb9 = {
_30 = !false;
_32 = [_23];
_29 = _3;
_28 = 15_i8 as f32;
_18 = (*_12);
Goto(bb10)
}
bb10 = {
_2 = (_1.0, _24, _26.2);
_33.0 = [(*_12),(*_12),_18,(*_12),_18,(*_12),(*_12),_16];
RET = !_26.2;
_1.2 = RET;
_15.0 = core::ptr::addr_of_mut!(_12);
_21 = _2.2 + _1.2;
_25 = _5;
_2 = (_32, _1.1, _21);
_12 = &(*_12);
_2.1 = _13 as f64;
RET = !_1.2;
_4 = [_18,(*_12),(*_12),(*_12),(*_12),_16,(*_12),(*_12)];
_33.0 = [(*_12),_16,(*_12),(*_12),(*_12),(*_12),(*_12),_16];
_19 = 7849787854784021771_u64;
_1.2 = _21;
RET = _26.2 >> _21;
_25 = _5;
_4 = [(*_12),(*_12),(*_12),(*_12),(*_12),_16,_18,(*_12)];
_12 = &(*_12);
_3.0 = _29.0;
_39.2 = _1.2 * RET;
_39.2 = !_26.2;
_31 = [(*_12),(*_12),_16,(*_12),_16,_16,(*_12),_18];
_14 = [_19,_19,_19,_19,_19];
Goto(bb11)
}
bb11 = {
Call(_41 = dump_var(17_usize, 19_usize, Move(_19), 4_usize, Move(_4), 23_usize, Move(_23), 13_usize, Move(_13)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_41 = dump_var(17_usize, 33_usize, Move(_33), 14_usize, Move(_14), 29_usize, Move(_29), 11_usize, Move(_11)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_41 = dump_var(17_usize, 32_usize, Move(_32), 9_usize, Move(_9), 20_usize, Move(_20), 42_usize, _42), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: ([i64; 8],),mut _2: ([i64; 8],),mut _3: isize,mut _4: [i64; 8],mut _5: [i64; 8],mut _6: char) -> f64 {
mir! {
type RET = f64;
let _7: [u128; 2];
let _8: ((*mut &'static i64,), i8);
let _9: ([u16; 1], f64, u8);
let _10: char;
let _11: char;
let _12: usize;
let _13: [u64; 5];
let _14: char;
let _15: [u8; 4];
let _16: [i64; 8];
let _17: [i16; 4];
let _18: i32;
let _19: [i64; 8];
let _20: ();
let _21: ();
{
_1 = (_4,);
_3 = 34_isize;
_5 = [(-3239248046211588438_i64),8889473610296973867_i64,(-5754091487098344860_i64),(-5780609986202981168_i64),(-1639158172739187929_i64),(-7383342825575244812_i64),850752154488595753_i64,2799925834605979628_i64];
RET = (-45_i8) as f64;
_1.0 = [(-3506333940229411216_i64),(-1036553780073751413_i64),7378133589348127968_i64,5759386387432568099_i64,(-357295440974984744_i64),8777276905430489162_i64,(-7223822660018869175_i64),3105469559766878633_i64];
_3 = (-104_isize);
_1 = (_5,);
_2.0 = [(-4743102483942942867_i64),(-351615345217326274_i64),(-8528314084678238661_i64),6109338730772233520_i64,(-5087831133163093549_i64),8646418131002419983_i64,(-6400585022043391122_i64),(-3519473217675859152_i64)];
_3 = (-1074572444_i32) as isize;
_1 = (_2.0,);
_9.2 = 197_u8;
_1.0 = [(-8102603665910368281_i64),6246765786478610674_i64,(-7134342496461306409_i64),7977846375580754614_i64,172194832186730986_i64,(-8777571980630245996_i64),(-5238352747948670636_i64),6763511467153814777_i64];
_9.2 = 198_u8;
_1.0 = [(-8854113743389340316_i64),(-3993331009038708765_i64),1450365742132670631_i64,7201502676078552425_i64,4802848753494868002_i64,8630279700207228185_i64,3013649049725730746_i64,(-4482195517547324121_i64)];
_5 = [6089553706769861838_i64,482215242143887560_i64,(-2310701897128625765_i64),5315116733188800240_i64,(-7061751856848752691_i64),(-1678307492487672311_i64),(-3572297451884654159_i64),7665832755809157493_i64];
_9.1 = -RET;
_5 = [(-7774630345148107919_i64),(-2088369719774495214_i64),(-1965619907911740999_i64),(-6344254070665569999_i64),5318255586738259964_i64,(-358056254495338774_i64),2407400481041632933_i64,922271545933663742_i64];
Goto(bb1)
}
bb1 = {
_3 = 9223372036854775807_isize & (-9223372036854775808_isize);
_9.0 = [43095_u16];
_6 = '\u{3f43}';
_2.0 = [264778673051493977_i64,1395332268270589190_i64,8322839730086396304_i64,(-6076323491873856556_i64),2965228251933639329_i64,3848985135096214716_i64,(-6885884187681512123_i64),(-8832617447142732378_i64)];
_6 = '\u{8eaa9}';
_6 = '\u{76256}';
_9.0 = [30300_u16];
_1 = (_2.0,);
RET = -_9.1;
_2 = (_5,);
_11 = _6;
_10 = _11;
_11 = _10;
Goto(bb2)
}
bb2 = {
_9.2 = 253_u8;
_9.0 = [41638_u16];
_6 = _11;
RET = -_9.1;
_8.1 = 231704066993280453118897331649121318341_u128 as i8;
_13 = [16195900689605403650_u64,13993610363481722427_u64,9557770559249913970_u64,10792594559246269392_u64,6412941640790717791_u64];
_12 = 17244005050178035079_usize;
_3 = 9223372036854775807_isize;
match _12 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
17244005050178035079 => bb11,
_ => bb10
}
}
bb3 = {
_3 = 9223372036854775807_isize & (-9223372036854775808_isize);
_9.0 = [43095_u16];
_6 = '\u{3f43}';
_2.0 = [264778673051493977_i64,1395332268270589190_i64,8322839730086396304_i64,(-6076323491873856556_i64),2965228251933639329_i64,3848985135096214716_i64,(-6885884187681512123_i64),(-8832617447142732378_i64)];
_6 = '\u{8eaa9}';
_6 = '\u{76256}';
_9.0 = [30300_u16];
_1 = (_2.0,);
RET = -_9.1;
_2 = (_5,);
_11 = _6;
_10 = _11;
_11 = _10;
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
Return()
}
bb11 = {
_9.0 = [19616_u16];
_1 = _2;
_14 = _10;
_14 = _6;
RET = _8.1 as f64;
_1 = _2;
_9.2 = _9.1 as u8;
_1 = (_2.0,);
_1 = _2;
_13 = [2427219389040351490_u64,11347707480348458883_u64,202784513960409253_u64,6121524704394061302_u64,3246690850688695416_u64];
_2.0 = _5;
_5 = [(-4128972523475531817_i64),(-7463182625127500829_i64),(-5232902091464855685_i64),(-5431469506967644688_i64),(-6548899244233011027_i64),(-3698098461084436004_i64),(-642893656711260743_i64),3151542546151971746_i64];
_9.0 = [53204_u16];
_2.0 = _5;
_6 = _14;
_13 = [17788889557697822978_u64,10508009458135747192_u64,157671352848247086_u64,12909819963201332605_u64,8625837512885622209_u64];
_12 = 4_usize + 1_usize;
_11 = _10;
_11 = _10;
_4 = _1.0;
Goto(bb12)
}
bb12 = {
Goto(bb13)
}
bb13 = {
_1.0 = [(-4485483707507884615_i64),7723146873923225518_i64,6163657002026721346_i64,(-1096247341558021128_i64),(-1679076653786851736_i64),3026312721145755541_i64,(-6817496639012222452_i64),355022750854677909_i64];
_9.0 = [15295_u16];
_9.0 = [54979_u16];
_14 = _6;
RET = (-979369825_i32) as f64;
_13 = [5533059120451584563_u64,4815925718079789676_u64,16194732390699823560_u64,14980355237128651321_u64,2222075121273987689_u64];
_11 = _6;
_3 = true as isize;
_8.1 = (-31_i8);
_17 = [13023_i16,25546_i16,(-25179_i16),12174_i16];
_2.0 = [2893053330953665320_i64,(-7292728306728539689_i64),4520125954776394227_i64,(-4735625762061745886_i64),4005445250153204136_i64,339208057282550687_i64,7761418170916567466_i64,(-1281326808526519445_i64)];
_9.2 = 42_u8 + 237_u8;
_9.0 = [53139_u16];
_15 = [_9.2,_9.2,_9.2,_9.2];
Call(_2.0 = core::intrinsics::transmute(_4), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_9.2 = !206_u8;
_2 = _1;
_9.2 = !68_u8;
_6 = _14;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(18_usize, 17_usize, Move(_17), 10_usize, Move(_10), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(18_usize, 12_usize, Move(_12), 11_usize, Move(_11), 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: u8,mut _2: ([u16; 1], f64, u8),mut _3: u8,mut _4: [i64; 8],mut _5: [i64; 8]) -> isize {
mir! {
type RET = isize;
let _6: isize;
let _7: [u8; 4];
let _8: [i64; 8];
let _9: *const usize;
let _10: f64;
let _11: u128;
let _12: [u16; 1];
let _13: [u128; 2];
let _14: [u128; 2];
let _15: ((*mut &'static i64,), i8);
let _16: ();
let _17: ();
{
RET = -86_isize;
Goto(bb1)
}
bb1 = {
_2.0 = [19192_u16];
_2.2 = !_1;
_4 = _5;
RET = 105_isize;
RET = 100_isize;
_2.1 = 88453280886607657832244045986002817220_u128 as f64;
_5 = [(-7187069223846958822_i64),4683857714468186480_i64,(-7220767285973870382_i64),4343304927548724499_i64,(-3383830899146424328_i64),(-824710673786527161_i64),3723079350805856044_i64,2152486066260827597_i64];
_2.0 = [21241_u16];
_7 = [_3,_3,_1,_2.2];
RET = (-9223372036854775808_isize);
_7 = [_1,_2.2,_1,_1];
_6 = !RET;
_2.2 = _1 + _1;
_8 = [(-3146919046710439431_i64),(-1245055869963172266_i64),4323327955516012850_i64,(-6752576755907007284_i64),1160799837981474894_i64,(-5235876120824964398_i64),6601686648989290653_i64,(-1267663724545279825_i64)];
_7 = [_2.2,_2.2,_2.2,_3];
_8 = [(-2053752885650390614_i64),2394882408695722583_i64,(-1963181686643481976_i64),608205180475132770_i64,8381910982831501144_i64,(-3462187412617848953_i64),81678878082636135_i64,6885570840772916249_i64];
RET = !_6;
_4 = [(-7394089241536519636_i64),(-5484823297514464512_i64),4529774004538171812_i64,9211624444154162453_i64,6267286151011440047_i64,(-8721157736432186091_i64),(-226274120316190254_i64),1346614954951535421_i64];
Goto(bb2)
}
bb2 = {
_7 = [_2.2,_2.2,_2.2,_1];
_2.0 = [34909_u16];
_2.1 = _3 as f64;
_5 = [(-6154660374574198809_i64),(-1232795222142658132_i64),7714751752192694098_i64,(-6451377434108318237_i64),5475811329774724785_i64,(-6012390307571707963_i64),61283713076971321_i64,(-5970325154973317715_i64)];
_1 = !_3;
_2.2 = 8362631630780319788_usize as u8;
_2.2 = _3;
RET = _6;
RET = -_6;
Goto(bb3)
}
bb3 = {
_7 = [_2.2,_2.2,_2.2,_2.2];
_7 = [_2.2,_2.2,_1,_2.2];
_1 = '\u{4d732}' as u8;
Goto(bb4)
}
bb4 = {
RET = !_6;
_5 = _8;
RET = _6;
_11 = (-5297088249324637116_i64) as u128;
_8 = [(-5445993715823384747_i64),2613797668635999645_i64,(-1347274344021851114_i64),(-822649643051766231_i64),(-3961872455408492145_i64),8545039048168286981_i64,(-312021224476441616_i64),2831233697013066997_i64];
_7 = [_2.2,_3,_3,_3];
_6 = -RET;
_11 = !12320819495888807509739752464380043481_u128;
_2.2 = _11 as u8;
RET = _6 + _6;
_2.0 = [16186_u16];
_8 = _5;
_2.1 = 6556501631866054301_u64 as f64;
_2.0 = [53041_u16];
_2.2 = !_3;
_3 = (-66477959665156446913216449668698647542_i128) as u8;
_10 = -_2.1;
_1 = !_2.2;
RET = _6;
_4 = _8;
RET = '\u{10c5c}' as isize;
_5 = [(-5191744716868075455_i64),413151728579408404_i64,8110137586065481743_i64,3210087667863581388_i64,8465677935304917946_i64,5168065512981798644_i64,7354468762839724420_i64,(-5908631899844751597_i64)];
_1 = _2.2 | _2.2;
_12 = _2.0;
Goto(bb5)
}
bb5 = {
_11 = 285686939687990893792394712552423066343_u128;
_7 = [_2.2,_1,_1,_1];
_10 = _2.1 - _2.1;
RET = !_6;
RET = _6 ^ _6;
_1 = !_2.2;
_1 = !_2.2;
_7 = [_1,_1,_2.2,_2.2];
_6 = RET;
_1 = !_2.2;
_2.0 = [56082_u16];
_3 = _1;
RET = _6 << _6;
RET = -_6;
_2.0 = _12;
_13 = [_11,_11];
_11 = 272062133148058954057246045529625871498_u128 + 62889588016177303331423035892335707248_u128;
RET = _6 << _1;
_14 = [_11,_11];
Goto(bb6)
}
bb6 = {
Call(_16 = dump_var(19_usize, 11_usize, Move(_11), 6_usize, Move(_6), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_16 = dump_var(19_usize, 4_usize, Move(_4), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{549ed}'), std::hint::black_box(283246534541431320976876428323193687213_u128), std::hint::black_box(10502660830949645840_u64), std::hint::black_box(64_u8), std::hint::black_box(1063020513_i32), std::hint::black_box(1242789258494990520_i64), std::hint::black_box((-58824456648076164532271545619988924622_i128)));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *const u32,
fld1: *mut [usize; 2],
fld2: [u16; 1],
fld3: u64,

},
Variant1{
fld0: u32,
fld1: [u8; 4],
fld2: [i64; 8],
fld3: i8,
fld4: [u128; 2],
fld5: i64,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: [u64; 5],
fld1: i32,
fld2: *mut [u128; 2],
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: [u16; 1],
fld1: i16,
fld2: [i16; 4],
fld3: (char, *const u128, u32, u16),
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [usize; 2],
fld1: i16,

},
Variant1{
fld0: bool,
fld1: [u128; 2],
fld2: ((char, *const u128, u32, u16),),
fld3: f32,
fld4: u128,
fld5: i32,
fld6: u8,
fld7: *const u128,

},
Variant2{
fld0: i8,
fld1: Adt42,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (u32, [i64; 8], u8, isize, *const i16),
fld1: [i64; 8],
fld2: *const u128,

},
Variant1{
fld0: *mut f32,
fld1: *const u32,
fld2: f32,
fld3: i8,
fld4: ([u16; 1], f64, u8),
fld5: Adt43,
fld6: usize,
fld7: i128,

},
Variant2{
fld0: (u32, [i64; 8], u8, isize, *const i16),
fld1: Adt41,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [u16; 1],
fld1: *mut f32,
fld2: isize,
fld3: [u128; 2],
fld4: Adt44,
fld5: i32,
fld6: *mut [u128; 2],

},
Variant1{
fld0: bool,
fld1: usize,
fld2: *const u32,
fld3: ([i64; 8],),
fld4: [u64; 5],
fld5: ([u16; 1], f64, u8),
fld6: *mut [u128; 2],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [i64; 8],

},
Variant1{
fld0: Adt43,
fld1: Adt44,
fld2: u32,
fld3: [u16; 1],
fld4: ([u16; 1], f64, u8),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: *mut [u128; 2],
fld1: (u32, [i64; 8], u8, isize, *const i16),
fld2: [u128; 2],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u32,
fld1: Adt44,

},
Variant1{
fld0: *const u128,
fld1: *const usize,
fld2: u64,
fld3: f32,
fld4: i16,
fld5: Adt43,
fld6: Adt45,
fld7: *mut f32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: Adt47,
fld1: Adt45,
fld2: [u8; 4],
fld3: [i16; 4],
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u16,
fld1: [u16; 1],
fld2: (char, *const u128, u32, u16),

},
Variant1{
fld0: ([i64; 8],),

},
Variant2{
fld0: bool,
fld1: Adt49,
fld2: f64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u16,
fld1: char,
fld2: i64,

},
Variant1{
fld0: [u16; 1],
fld1: usize,
fld2: isize,
fld3: *mut i32,
fld4: [u8; 4],

},
Variant2{
fld0: u32,
fld1: [u128; 2],
fld2: u16,

},
Variant3{
fld0: f32,
fld1: i128,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [i16; 4],
fld1: f32,
fld2: Adt48,
fld3: [u128; 2],
fld4: ([u16; 1], f64, u8),

},
Variant1{
fld0: *mut [usize; 2],
fld1: ([i64; 8],),
fld2: *const u32,

},
Variant2{
fld0: [i16; 4],
fld1: Adt43,
fld2: *const usize,
fld3: *mut f32,
fld4: [u64; 5],
fld5: i32,
fld6: *const u32,
fld7: i128,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt43,
fld1: [u8; 4],
fld2: [i64; 8],
fld3: i8,
fld4: i16,
fld5: Adt49,
fld6: Adt48,
fld7: Adt50,

},
Variant1{
fld0: [u128; 2],
fld1: char,
fld2: i64,
fld3: [u64; 5],
fld4: u64,
fld5: (u32, [i64; 8], u8, isize, *const i16),

},
Variant2{
fld0: f64,
fld1: i16,
fld2: f32,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt47,
fld1: u32,
fld2: *mut f32,
fld3: [u128; 2],
fld4: *mut [usize; 2],
fld5: i32,
fld6: i64,
fld7: Adt48,

},
Variant1{
fld0: (char, *const u128, u32, u16),
fld1: u64,
fld2: (u32, [i64; 8], u8, isize, *const i16),

},
Variant2{
fld0: Adt46,
fld1: char,
fld2: Adt43,

},
Variant3{
fld0: [u16; 1],
fld1: Adt45,
fld2: Adt52,
fld3: Adt41,
fld4: u64,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt46,
fld1: [usize; 2],
fld2: Adt52,
fld3: [i64; 8],
fld4: Adt50,
fld5: Adt42,
fld6: [i16; 4],

},
Variant1{
fld0: u32,
fld1: Adt42,
fld2: i128,
fld3: i8,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u8,

},
Variant1{
fld0: bool,
fld1: Adt43,
fld2: i128,
fld3: Adt52,
fld4: Adt55,

},
Variant2{
fld0: Adt46,
fld1: *mut [usize; 2],

},
Variant3{
fld0: f32,
fld1: [i64; 8],

}}

