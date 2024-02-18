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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: usize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i128) -> u8 {
mir! {
type RET = u8;
let _8: isize;
let _9: isize;
let _10: isize;
let _11: &'static u64;
let _12: isize;
let _13: isize;
let _14: (Adt31, u8);
let _15: *const Adt63;
let _16: [u16; 1];
let _17: f32;
let _18: i64;
let _19: [u16; 1];
let _20: u8;
let _21: [u32; 8];
let _22: isize;
let _23: *const Adt26;
let _24: &'static *mut Adt38;
let _25: &'static [i128; 6];
let _26: (u32, *const Adt26, *mut i32, u128);
let _27: u128;
let _28: char;
let _29: u128;
let _30: f32;
let _31: Adt53;
let _32: *const [u16; 4];
let _33: ();
let _34: ();
{
_5 = (-8459_i16);
_2 = '\u{ceeaf}';
RET = !131_u8;
_7 = 140320716308732728866508424223450358238_i128 << RET;
_4 = (-85_i8);
_2 = '\u{69e75}';
_5 = 28281_i16;
_3 = 2_usize;
_1 = true;
_1 = _2 >= _2;
_6 = !(-1792075138_i32);
_1 = !false;
_1 = false;
_3 = !12268096086490794060_usize;
_1 = true;
_3 = 0_usize >> RET;
_6 = !880225152_i32;
_6 = 978394924_i32;
RET = _7 as u8;
RET = _1 as u8;
RET = 194_u8;
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = (-1108810877_i32) & (-1617187182_i32);
_1 = true ^ false;
_6 = -(-881085604_i32);
_4 = _6 as i8;
_1 = true & true;
_7 = 17853641436372380670_u64 as i128;
RET = 170_u8;
RET = _7 as u8;
RET = 138_u8;
RET = 81_u8 * 97_u8;
_8 = -(-9223372036854775808_isize);
_8 = -9223372036854775807_isize;
_6 = _3 as i32;
_6 = (-1301962836_i32);
_10 = 11523755704784396090_u64 as isize;
_4 = _7 as i8;
RET = !63_u8;
RET = 83_u8 << _6;
Call(_7 = fn1(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = (-107_i8) & (-9_i8);
_9 = _2 as isize;
_3 = _7 as usize;
_7 = -(-144113802375156012457600453410634601791_i128);
_7 = !(-82452871086979585828475762488058106675_i128);
_7 = -14083273272927257102284818074309372470_i128;
_4 = -118_i8;
_2 = '\u{45ad9}';
_5 = -(-15217_i16);
_8 = !_9;
RET = _7 as u8;
_10 = _8;
_4 = -89_i8;
_2 = '\u{3e3c0}';
_6 = 898634763_i32;
_3 = 896017581459489506_usize + 7109258397130222390_usize;
_3 = 242483356605130526_u64 as usize;
_8 = _6 as isize;
_6 = -(-332852947_i32);
RET = 17894_u16 as u8;
_8 = _10;
_8 = -_10;
_6 = (-909275343_i32);
Call(RET = core::intrinsics::bswap(172_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = 823645166_i32;
_13 = _8 >> _6;
_14.1 = RET - RET;
_6 = -(-1717059819_i32);
Goto(bb4)
}
bb4 = {
RET = !_14.1;
Goto(bb5)
}
bb5 = {
RET = 20532_u16 as u8;
_8 = _10;
_18 = !920357778501284533_i64;
_17 = 185440675023690791676763011070935131241_u128 as f32;
_1 = RET == _14.1;
_5 = 2214496563_u32 as i16;
_5 = (-29411_i16);
_10 = _17 as isize;
_17 = _18 as f32;
_19 = [49206_u16];
_6 = 1774128119_i32 << RET;
_16 = [32250_u16];
_8 = _9 | _13;
_10 = _8;
Goto(bb6)
}
bb6 = {
_6 = 220730234_i32;
_22 = _8;
_20 = !_14.1;
_2 = '\u{243b8}';
_17 = 505287881401840023_u64 as f32;
_20 = _14.1 ^ _14.1;
_18 = _8 as i64;
_4 = _10 as i8;
_12 = _22 * _22;
_27 = 275731776659500860253343046642175097803_u128 | 141184342623718378880899235909490382473_u128;
_7 = 74761624423621662505220772416766855151_i128;
Goto(bb7)
}
bb7 = {
_1 = !false;
_26.0 = _17 as u32;
_28 = _2;
_8 = !_22;
RET = _27 as u8;
_3 = !7_usize;
RET = _20 * _14.1;
_8 = _12;
_27 = !120325347653950701876857727180894236337_u128;
_27 = 138065095533507783265551852332946838860_u128 * 117717648121748277650131061264446321801_u128;
_13 = 5382631251821935299_u64 as isize;
_29 = !_27;
RET = _5 as u8;
_29 = _27;
_19 = _16;
_6 = 6681637238682564507_u64 as i32;
_26.0 = 1427292407_u32 & 2080176333_u32;
_17 = _26.0 as f32;
match _5 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463463374607431768182045 => bb14,
_ => bb13
}
}
bb8 = {
_6 = 220730234_i32;
_22 = _8;
_20 = !_14.1;
_2 = '\u{243b8}';
_17 = 505287881401840023_u64 as f32;
_20 = _14.1 ^ _14.1;
_18 = _8 as i64;
_4 = _10 as i8;
_12 = _22 * _22;
_27 = 275731776659500860253343046642175097803_u128 | 141184342623718378880899235909490382473_u128;
_7 = 74761624423621662505220772416766855151_i128;
Goto(bb7)
}
bb9 = {
RET = 20532_u16 as u8;
_8 = _10;
_18 = !920357778501284533_i64;
_17 = 185440675023690791676763011070935131241_u128 as f32;
_1 = RET == _14.1;
_5 = 2214496563_u32 as i16;
_5 = (-29411_i16);
_10 = _17 as isize;
_17 = _18 as f32;
_19 = [49206_u16];
_6 = 1774128119_i32 << RET;
_16 = [32250_u16];
_8 = _9 | _13;
_10 = _8;
Goto(bb6)
}
bb10 = {
RET = !_14.1;
Goto(bb5)
}
bb11 = {
_6 = 823645166_i32;
_13 = _8 >> _6;
_14.1 = RET - RET;
_6 = -(-1717059819_i32);
Goto(bb4)
}
bb12 = {
_4 = (-107_i8) & (-9_i8);
_9 = _2 as isize;
_3 = _7 as usize;
_7 = -(-144113802375156012457600453410634601791_i128);
_7 = !(-82452871086979585828475762488058106675_i128);
_7 = -14083273272927257102284818074309372470_i128;
_4 = -118_i8;
_2 = '\u{45ad9}';
_5 = -(-15217_i16);
_8 = !_9;
RET = _7 as u8;
_10 = _8;
_4 = -89_i8;
_2 = '\u{3e3c0}';
_6 = 898634763_i32;
_3 = 896017581459489506_usize + 7109258397130222390_usize;
_3 = 242483356605130526_u64 as usize;
_8 = _6 as isize;
_6 = -(-332852947_i32);
RET = 17894_u16 as u8;
_8 = _10;
_8 = -_10;
_6 = (-909275343_i32);
Call(RET = core::intrinsics::bswap(172_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_6 = (-1108810877_i32) & (-1617187182_i32);
_1 = true ^ false;
_6 = -(-881085604_i32);
_4 = _6 as i8;
_1 = true & true;
_7 = 17853641436372380670_u64 as i128;
RET = 170_u8;
RET = _7 as u8;
RET = 138_u8;
RET = 81_u8 * 97_u8;
_8 = -(-9223372036854775808_isize);
_8 = -9223372036854775807_isize;
_6 = _3 as i32;
_6 = (-1301962836_i32);
_10 = 11523755704784396090_u64 as isize;
_4 = _7 as i8;
RET = !63_u8;
RET = 83_u8 << _6;
Call(_7 = fn1(), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_17 = _18 as f32;
_30 = _18 as f32;
_1 = !true;
_20 = _4 as u8;
_26.3 = !_27;
RET = _17 as u8;
_31.fld1 = _1 as usize;
_31.fld7.fld5 = _2 as i32;
_21 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_27 = _26.3;
_31.fld6 = core::ptr::addr_of!(_31.fld7);
_26.3 = !_29;
_28 = _2;
_31.fld7.fld5 = _6 ^ _6;
_9 = _5 as isize;
_31.fld7.fld2 = _13;
_28 = _2;
_17 = _30 + _30;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 19_usize, Move(_19), 18_usize, Move(_18), 28_usize, Move(_28), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 9_usize, Move(_9), 20_usize, Move(_20), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 6_usize, Move(_6), 4_usize, Move(_4), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1() -> i128 {
mir! {
type RET = i128;
let _1: f32;
let _2: [i64; 3];
let _3: (Adt31, u8);
let _4: bool;
let _5: bool;
let _6: *const &'static [i128; 6];
let _7: bool;
let _8: *const f32;
let _9: i16;
let _10: u32;
let _11: Adt40;
let _12: Adt31;
let _13: u128;
let _14: [u32; 8];
let _15: Adt40;
let _16: [i32; 6];
let _17: f64;
let _18: *const &'static [i128; 6];
let _19: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _20: *const (Adt38,);
let _21: i8;
let _22: isize;
let _23: [i128; 2];
let _24: isize;
let _25: [isize; 2];
let _26: [i64; 6];
let _27: f64;
let _28: bool;
let _29: isize;
let _30: &'static u64;
let _31: u8;
let _32: [usize; 5];
let _33: bool;
let _34: (f32, [isize; 8], [bool; 8]);
let _35: *const (u32, *const Adt26, *mut i32, u128);
let _36: u32;
let _37: ();
let _38: ();
{
RET = -(-57316238346901239727431663947858863088_i128);
RET = !(-101057215767150739257347080409901014627_i128);
RET = (-36814863767766217174455192541880531561_i128) & 60777167148976531166709589545749534089_i128;
_1 = 596499662_i32 as f32;
RET = 61182953837082992398863880087330212959_i128 & 40412545249195419186802903329048453401_i128;
_1 = (-3739_i16) as f32;
RET = !69081623262858347484898130413652895636_i128;
_1 = (-13154_i16) as f32;
RET = (-22_i8) as i128;
_1 = 267025682480199308155515505136316437432_u128 as f32;
_2 = [8285986780159647548_i64,(-7083000805186496160_i64),(-1040373092508468667_i64)];
_2 = [8710578330262143204_i64,1223023213493877490_i64,4268814599774188267_i64];
RET = _1 as i128;
RET = 54012937692863387841782804156549819149_u128 as i128;
_3.1 = !246_u8;
RET = (-101812812722734087863988907502982989516_i128) ^ (-60737461804623242896385058749339869454_i128);
_4 = false;
_3.1 = 71_u8 | 249_u8;
_2 = [1839869543424776659_i64,(-401730304567856816_i64),(-785532635947149178_i64)];
_1 = 9223372036854775807_isize as f32;
_5 = !_4;
Goto(bb1)
}
bb1 = {
_3.1 = 42410_u16 as u8;
_4 = _5 & _5;
_4 = !_5;
_3.1 = 62_u8;
_5 = !_4;
_5 = _4;
_7 = _4;
_1 = 13135_u16 as f32;
_1 = _3.1 as f32;
_3.1 = 209_u8;
_8 = core::ptr::addr_of!(_1);
_9 = 23342_i16 + (-23152_i16);
_7 = !_5;
_1 = 548466185_u32 as f32;
RET = 7_usize as i128;
_3.1 = (*_8) as u8;
RET = 3421902936520685794762716481489793842_u128 as i128;
_1 = 6841310590962716455_u64 as f32;
_10 = 3981617879_u32;
_4 = _5 | _7;
RET = !121441821827369453325064049170947222676_i128;
RET = 46998512586970742984198324018180808755_i128;
_9 = RET as i16;
(*_8) = _10 as f32;
_3.1 = _9 as u8;
Call(_9 = fn2(RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = -95773682488371985742073951093261043601_i128;
_10 = 13247_u16 as u32;
(*_8) = _3.1 as f32;
(*_8) = (-2084374429_i32) as f32;
_1 = 8304925491130923592_usize as f32;
_7 = !_4;
_7 = !_4;
_10 = !2765036653_u32;
Call(_9 = core::intrinsics::bswap(17153_i16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = 31548719596363324165805362460599245097_u128;
_9 = (-13660_i16) << _13;
_10 = RET as u32;
RET = 88810296791775216422108259337520807832_i128 | 119450932529974079258280165249540063312_i128;
_2 = [(-1205340951607417711_i64),(-8941774223843644207_i64),(-2680803427887392316_i64)];
(*_8) = 53_i8 as f32;
_9 = 29157_i16;
_10 = 1563231975_u32 * 4008408053_u32;
(*_8) = 25_i8 as f32;
_11.fld1 = '\u{c6b36}';
_3.1 = 216_u8;
RET = 3633811941781317856942778891300690833_i128;
_14 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = 2005335616_u32;
_1 = 29_i8 as f32;
_8 = core::ptr::addr_of!((*_8));
_13 = !310634069486445901194598144307010899994_u128;
_4 = _5;
_17 = _1 as f64;
Call(_13 = fn3(_7, _14, _10, _9, _17, _1, _10, _9, _9, _5, RET, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = _7;
RET = 50877_u16 as i128;
_9 = 12101_i16;
_7 = _4;
_2 = [4799393758411599084_i64,(-6785564430728517521_i64),(-2801802793578517005_i64)];
_7 = _5;
_8 = core::ptr::addr_of!(_1);
_15.fld1 = _11.fld1;
_15.fld1 = _11.fld1;
_11.fld1 = _15.fld1;
_2 = [6920883212017384940_i64,(-7649434928035962884_i64),3943029948983060914_i64];
_13 = _10 as u128;
_9 = (-20423_i16) >> _3.1;
_16 = [1996222653_i32,606244371_i32,(-1651760266_i32),(-1239929292_i32),280795054_i32,(-1476017390_i32)];
_19.2.0 = !90_i8;
RET = !(-105158899453298161370994614900552566030_i128);
_11.fld1 = _15.fld1;
Goto(bb5)
}
bb5 = {
(*_8) = _9 as f32;
_19.0 = _9 * _9;
_19.0 = _9 >> _10;
_3.1 = 150_u8;
_9 = _19.0;
_19.0 = _9 - _9;
(*_8) = _10 as f32;
_19.0 = _9;
_11.fld0 = Adt26::Variant2 { fld0: _13 };
_5 = _7;
_19.1 = (-1497511869_i32) as f64;
_21 = _19.2.0;
_14 = [_10,_10,_10,_10,_10,_10,_10,_10];
match _3.1 {
0 => bb4,
1 => bb3,
2 => bb6,
150 => bb8,
_ => bb7
}
}
bb6 = {
_3.1 = 42410_u16 as u8;
_4 = _5 & _5;
_4 = !_5;
_3.1 = 62_u8;
_5 = !_4;
_5 = _4;
_7 = _4;
_1 = 13135_u16 as f32;
_1 = _3.1 as f32;
_3.1 = 209_u8;
_8 = core::ptr::addr_of!(_1);
_9 = 23342_i16 + (-23152_i16);
_7 = !_5;
_1 = 548466185_u32 as f32;
RET = 7_usize as i128;
_3.1 = (*_8) as u8;
RET = 3421902936520685794762716481489793842_u128 as i128;
_1 = 6841310590962716455_u64 as f32;
_10 = 3981617879_u32;
_4 = _5 | _7;
RET = !121441821827369453325064049170947222676_i128;
RET = 46998512586970742984198324018180808755_i128;
_9 = RET as i16;
(*_8) = _10 as f32;
_3.1 = _9 as u8;
Call(_9 = fn2(RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_13 = 31548719596363324165805362460599245097_u128;
_9 = (-13660_i16) << _13;
_10 = RET as u32;
RET = 88810296791775216422108259337520807832_i128 | 119450932529974079258280165249540063312_i128;
_2 = [(-1205340951607417711_i64),(-8941774223843644207_i64),(-2680803427887392316_i64)];
(*_8) = 53_i8 as f32;
_9 = 29157_i16;
_10 = 1563231975_u32 * 4008408053_u32;
(*_8) = 25_i8 as f32;
_11.fld1 = '\u{c6b36}';
_3.1 = 216_u8;
RET = 3633811941781317856942778891300690833_i128;
_14 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = 2005335616_u32;
_1 = 29_i8 as f32;
_8 = core::ptr::addr_of!((*_8));
_13 = !310634069486445901194598144307010899994_u128;
_4 = _5;
_17 = _1 as f64;
Call(_13 = fn3(_7, _14, _10, _9, _17, _1, _10, _9, _9, _5, RET, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_15.fld0 = Adt26::Variant2 { fld0: Field::<u128>(Variant(_11.fld0, 2), 0) };
RET = _11.fld1 as i128;
SetDiscriminant(_15.fld0, 3);
_17 = (-9223372036854775808_isize) as f64;
place!(Field::<[u32; 2]>(Variant(_15.fld0, 3), 3)) = [_10,_10];
(*_8) = (-7303373150940982998_i64) as f32;
_21 = _19.2.0 | _19.2.0;
_24 = (-15_isize) << _13;
place!(Field::<(i16, usize, u32)>(Variant(_15.fld0, 3), 1)).1 = !5_usize;
(*_8) = _17 as f32;
_4 = !_7;
place!(Field::<i32>(Variant(_15.fld0, 3), 5)) = 238799344_i32;
_3.1 = 229_u8 << _21;
_8 = core::ptr::addr_of!(place!(Field::<f32>(Variant(_15.fld0, 3), 0)));
_5 = Field::<u128>(Variant(_11.fld0, 2), 0) <= Field::<u128>(Variant(_11.fld0, 2), 0);
_7 = !_5;
SetDiscriminant(_11.fld0, 2);
place!(Field::<[u32; 2]>(Variant(_15.fld0, 3), 3)) = [_10,_10];
_22 = _24;
place!(Field::<u128>(Variant(_15.fld0, 3), 4)) = _13;
_9 = -_19.0;
_25 = [_24,_22];
place!(Field::<(i16, usize, u32)>(Variant(_15.fld0, 3), 1)).2 = _10;
_19.3 = RET as u16;
_2 = [7743467068995129627_i64,(-3549441697463547969_i64),3894324083922797314_i64];
_19.2.1 = [_19.2.0,_21,_21,_21,_21,_19.2.0];
(*_8) = Field::<u128>(Variant(_15.fld0, 3), 4) as f32;
Goto(bb9)
}
bb9 = {
_17 = _19.1;
Goto(bb10)
}
bb10 = {
_26 = [(-2664801540839034864_i64),8392118590837290241_i64,7624805612634841147_i64,(-100275654415881480_i64),(-7551201358057548471_i64),8867754269676171931_i64];
_25 = [_22,_22];
_22 = !_24;
place!(Field::<[u32; 2]>(Variant(_15.fld0, 3), 3)) = [Field::<(i16, usize, u32)>(Variant(_15.fld0, 3), 1).2,_10];
_19.0 = _9 | _9;
_2 = [7326202563778927629_i64,3235085913399689269_i64,(-8102488338759712262_i64)];
_21 = -_19.2.0;
RET = !(-30178034719719354655316502471755717019_i128);
_8 = core::ptr::addr_of!((*_8));
_22 = !_24;
_26 = [(-7581303232139132455_i64),9034197688104610528_i64,(-8088228923380100320_i64),1203767047461491181_i64,(-161799339810327737_i64),661765782192499853_i64];
RET = !71175081925389089932046692434828473636_i128;
_23 = [RET,RET];
place!(Field::<u128>(Variant(_11.fld0, 2), 0)) = !Field::<u128>(Variant(_15.fld0, 3), 4);
_9 = _19.0;
_21 = _19.2.0 - _19.2.0;
_5 = _4;
place!(Field::<f32>(Variant(_15.fld0, 3), 0)) = -_1;
RET = -45525262944125193553503648846079214408_i128;
_15 = Move(_11);
_19.1 = _17 + _17;
_19.1 = _17 * _17;
RET = _10 as i128;
_34.1 = [_22,_24,_24,_22,_24,_24,_24,_22];
match _10 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
2005335616 => bb16,
_ => bb15
}
}
bb11 = {
_17 = _19.1;
Goto(bb10)
}
bb12 = {
_5 = _7;
RET = 50877_u16 as i128;
_9 = 12101_i16;
_7 = _4;
_2 = [4799393758411599084_i64,(-6785564430728517521_i64),(-2801802793578517005_i64)];
_7 = _5;
_8 = core::ptr::addr_of!(_1);
_15.fld1 = _11.fld1;
_15.fld1 = _11.fld1;
_11.fld1 = _15.fld1;
_2 = [6920883212017384940_i64,(-7649434928035962884_i64),3943029948983060914_i64];
_13 = _10 as u128;
_9 = (-20423_i16) >> _3.1;
_16 = [1996222653_i32,606244371_i32,(-1651760266_i32),(-1239929292_i32),280795054_i32,(-1476017390_i32)];
_19.2.0 = !90_i8;
RET = !(-105158899453298161370994614900552566030_i128);
_11.fld1 = _15.fld1;
Goto(bb5)
}
bb13 = {
_13 = 31548719596363324165805362460599245097_u128;
_9 = (-13660_i16) << _13;
_10 = RET as u32;
RET = 88810296791775216422108259337520807832_i128 | 119450932529974079258280165249540063312_i128;
_2 = [(-1205340951607417711_i64),(-8941774223843644207_i64),(-2680803427887392316_i64)];
(*_8) = 53_i8 as f32;
_9 = 29157_i16;
_10 = 1563231975_u32 * 4008408053_u32;
(*_8) = 25_i8 as f32;
_11.fld1 = '\u{c6b36}';
_3.1 = 216_u8;
RET = 3633811941781317856942778891300690833_i128;
_14 = [_10,_10,_10,_10,_10,_10,_10,_10];
_10 = 2005335616_u32;
_1 = 29_i8 as f32;
_8 = core::ptr::addr_of!((*_8));
_13 = !310634069486445901194598144307010899994_u128;
_4 = _5;
_17 = _1 as f64;
Call(_13 = fn3(_7, _14, _10, _9, _17, _1, _10, _9, _9, _5, RET, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_3.1 = 42410_u16 as u8;
_4 = _5 & _5;
_4 = !_5;
_3.1 = 62_u8;
_5 = !_4;
_5 = _4;
_7 = _4;
_1 = 13135_u16 as f32;
_1 = _3.1 as f32;
_3.1 = 209_u8;
_8 = core::ptr::addr_of!(_1);
_9 = 23342_i16 + (-23152_i16);
_7 = !_5;
_1 = 548466185_u32 as f32;
RET = 7_usize as i128;
_3.1 = (*_8) as u8;
RET = 3421902936520685794762716481489793842_u128 as i128;
_1 = 6841310590962716455_u64 as f32;
_10 = 3981617879_u32;
_4 = _5 | _7;
RET = !121441821827369453325064049170947222676_i128;
RET = 46998512586970742984198324018180808755_i128;
_9 = RET as i16;
(*_8) = _10 as f32;
_3.1 = _9 as u8;
Call(_9 = fn2(RET, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
(*_8) = _9 as f32;
_19.0 = _9 * _9;
_19.0 = _9 >> _10;
_3.1 = 150_u8;
_9 = _19.0;
_19.0 = _9 - _9;
(*_8) = _10 as f32;
_19.0 = _9;
_11.fld0 = Adt26::Variant2 { fld0: _13 };
_5 = _7;
_19.1 = (-1497511869_i32) as f64;
_21 = _19.2.0;
_14 = [_10,_10,_10,_10,_10,_10,_10,_10];
match _3.1 {
0 => bb4,
1 => bb3,
2 => bb6,
150 => bb8,
_ => bb7
}
}
bb16 = {
_16 = [1924089894_i32,(-1456398088_i32),(-1680470389_i32),(-174160571_i32),(-719164434_i32),1717119144_i32];
_27 = _19.1;
_28 = _3.1 < _3.1;
_34.0 = _1 * _1;
_22 = -_24;
_34.1 = [_24,_24,_24,_22,_22,_24,_22,_24];
_26 = [(-6843595582678751901_i64),(-3169139705270295891_i64),(-8615669359334721294_i64),(-4480395278240026328_i64),5040000490771204328_i64,5975475523448607204_i64];
_2 = [3978951220296318525_i64,(-4721859595946252347_i64),5001701816470138505_i64];
_11.fld0 = Move(_15.fld0);
SetDiscriminant(_11.fld0, 3);
place!(Field::<[u32; 2]>(Variant(_11.fld0, 3), 3)) = [_10,_10];
place!(Field::<(i16, usize, u32)>(Variant(_11.fld0, 3), 1)).1 = _3.1 as usize;
place!(Field::<(i16, usize, u32)>(Variant(_11.fld0, 3), 1)).0 = _9 | _9;
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(1_usize, 24_usize, Move(_24), 25_usize, Move(_25), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(1_usize, 16_usize, Move(_16), 23_usize, Move(_23), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: f32) -> i16 {
mir! {
type RET = i16;
let _3: (i16, usize, u32);
let _4: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _5: *const [u16; 4];
let _6: usize;
let _7: char;
let _8: [i128; 6];
let _9: isize;
let _10: (Adt31, u8);
let _11: f64;
let _12: bool;
let _13: i8;
let _14: [i128; 6];
let _15: ();
let _16: ();
{
_1 = 64761835658602616221013589514863769758_i128 << 122_u8;
_2 = 74_u8 as f32;
RET = !16991_i16;
RET = (-8548_i16) * (-12190_i16);
RET = _1 as i16;
RET = 899_i16 * (-8845_i16);
RET = (-2070460454_i32) as i16;
_1 = (-76292647075077535171000125331917315040_i128);
_2 = 8357936526994099855_u64 as f32;
_1 = !23445535322832223510868476818968938770_i128;
_4.1 = 3_usize as f64;
RET = 11858287812997962169_u64 as i16;
_2 = (-117_i8) as f32;
RET = -18445_i16;
_2 = 14341337938892031633_u64 as f32;
_3 = (RET, 11214651850970012131_usize, 2009767382_u32);
match _3.1 {
0 => bb1,
11214651850970012131 => bb3,
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
_4.1 = 2532980010156273469_u64 as f64;
_3.0 = RET;
RET = 14239450216734195862_u64 as i16;
_4.0 = RET;
_1 = 69210801931223435724102279001866804424_i128;
RET = -_4.0;
_1 = (-133283243652094112890387385676822779224_i128) * (-166062269751989036561163394501792951484_i128);
_3.1 = 101_u8 as usize;
RET = 183_u8 as i16;
_4.2.1 = [41_i8,88_i8,(-98_i8),100_i8,2_i8,4_i8];
RET = 11726239419953420522_u64 as i16;
_3.1 = 10391487698040987128_usize;
_4.2.0 = _3.1 as i8;
_4.1 = _4.2.0 as f64;
_1 = (-9223372036854775808_isize) as i128;
_4.2.0 = 68_i8;
match _4.2.0 {
0 => bb4,
1 => bb5,
2 => bb6,
68 => bb8,
_ => bb7
}
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
_3 = (RET, 2_usize, 3025739275_u32);
_2 = _4.1 as f32;
match _3.2 {
3025739275 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_3.0 = _2 as i16;
RET = -_4.0;
_2 = _3.1 as f32;
_3.2 = 804467612_u32 ^ 602095335_u32;
_4.3 = !63288_u16;
_3.1 = 10844757662636594117_usize - 1835707097006153940_usize;
RET = _3.1 as i16;
_4.1 = _1 as f64;
_3.2 = !2719032720_u32;
_4.2.0 = 103_i8 | (-104_i8);
_3.1 = 3465096218284801251_usize;
_4.3 = 52424_u16 ^ 46356_u16;
match _3.1 {
0 => bb5,
3465096218284801251 => bb11,
_ => bb6
}
}
bb11 = {
RET = _3.0;
RET = 1_u8 as i16;
_4.2.0 = 3538458356978288131_u64 as i8;
_4.0 = _3.0 & _3.0;
_4.3 = !32832_u16;
RET = _4.0 ^ _3.0;
_4.2.1 = [_4.2.0,_4.2.0,_4.2.0,_4.2.0,_4.2.0,_4.2.0];
_6 = !_3.1;
RET = -_4.0;
RET = _4.0;
_2 = 1116594551_i32 as f32;
_3.2 = !987191526_u32;
_8 = [_1,_1,_1,_1,_1,_1];
_7 = '\u{7790d}';
_4.2.1 = [_4.2.0,_4.2.0,_4.2.0,_4.2.0,_4.2.0,_4.2.0];
_3.1 = !_6;
_4.2.0 = !34_i8;
_9 = 1_isize;
_3 = (_4.0, _6, 354872878_u32);
RET = _4.3 as i16;
_3.1 = _6 | _6;
_4.0 = _3.0 + _3.0;
_3.0 = !_4.0;
RET = _3.0 - _4.0;
_6 = _1 as usize;
_3.2 = _1 as u32;
match _9 {
0 => bb1,
2 => bb6,
3 => bb5,
4 => bb12,
1 => bb14,
_ => bb13
}
}
bb12 = {
_3 = (RET, 2_usize, 3025739275_u32);
_2 = _4.1 as f32;
match _3.2 {
3025739275 => bb10,
_ => bb9
}
}
bb13 = {
Return()
}
bb14 = {
_4.0 = 1166911356_i32 as i16;
_3.1 = _6 >> RET;
_4.3 = 16489_u16 + 8556_u16;
_4.2.1 = [_4.2.0,_4.2.0,_4.2.0,_4.2.0,_4.2.0,_4.2.0];
_3.1 = _6 << _4.3;
_8 = [_1,_1,_1,_1,_1,_1];
_7 = '\u{100caa}';
RET = _3.0;
_3.1 = _6 ^ _6;
_10.1 = 205_u8 >> _3.1;
_3 = (_4.0, _6, 982172799_u32);
RET = _4.0 & _4.0;
_1 = 147564123439353866318074711045596051317_i128;
_12 = !false;
_12 = !false;
_3.0 = (-632741930_i32) as i16;
_9 = _2 as isize;
_11 = _4.1 * _4.1;
_3 = (RET, _6, 1570983299_u32);
_11 = -_4.1;
_4.3 = 47324_u16;
_2 = _4.0 as f32;
_11 = _4.1 + _4.1;
_9 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(2_usize, 8_usize, Move(_8), 12_usize, Move(_12), 9_usize, Move(_9), 16_usize, _16), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: bool,mut _2: [u32; 8],mut _3: u32,mut _4: i16,mut _5: f64,mut _6: f32,mut _7: u32,mut _8: i16,mut _9: i16,mut _10: bool,mut _11: i128,mut _12: [i64; 3]) -> u128 {
mir! {
type RET = u128;
let _13: i16;
let _14: [i16; 6];
let _15: *const [u16; 4];
let _16: Adt40;
let _17: (u32, *const Adt26, *mut i32, u128);
let _18: bool;
let _19: f64;
let _20: i32;
let _21: &'static [i64; 6];
let _22: &'static &'static u8;
let _23: isize;
let _24: u64;
let _25: Adt40;
let _26: i64;
let _27: usize;
let _28: f32;
let _29: &'static u8;
let _30: *const f32;
let _31: (i16, usize, u32);
let _32: [u16; 1];
let _33: i64;
let _34: f64;
let _35: [i64; 6];
let _36: &'static [i64; 6];
let _37: *mut i32;
let _38: char;
let _39: ();
let _40: ();
{
_2 = [_3,_3,_7,_7,_3,_7,_3,_3];
_9 = _8;
RET = (-720878149_i32) as u128;
_6 = 313206361879269030_i64 as f32;
_1 = _10;
_9 = _4;
_10 = _1;
_7 = !_3;
_2 = [_3,_7,_3,_3,_7,_7,_3,_7];
_11 = -104346160141987243261002286862069674745_i128;
_3 = _7;
_2 = [_3,_3,_3,_7,_7,_3,_3,_7];
_2 = [_3,_3,_7,_3,_3,_3,_3,_7];
_2 = [_3,_3,_3,_7,_7,_7,_7,_7];
_14 = [_8,_8,_9,_4,_8,_8];
_7 = _3 >> _8;
_14 = [_8,_9,_4,_8,_9,_4];
_9 = -_4;
RET = 298944878792584276423036281408610863973_u128;
_14 = [_4,_8,_8,_9,_9,_4];
RET = !116819253516735331934032107044476118458_u128;
_4 = _9 - _9;
_12 = [1864519875517592956_i64,(-1952728617214648991_i64),(-8773646138422168294_i64)];
match _8 {
29157 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_13 = _4 << _4;
_14 = [_13,_13,_9,_13,_13,_13];
_17.1 = core::ptr::addr_of!(_16.fld0);
_17.3 = !RET;
_11 = -(-56833449846521049031084118700531043461_i128);
match _8 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
29157 => bb9,
_ => bb8
}
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
_19 = _11 as f64;
RET = _17.3 | _17.3;
_10 = _13 >= _13;
_11 = (-1403481121_i32) as i128;
_17.1 = core::ptr::addr_of!(_16.fld0);
_14 = [_13,_4,_13,_4,_4,_4];
_1 = _13 > _13;
_10 = _1;
_14 = [_13,_4,_13,_13,_13,_8];
_6 = 10004_u16 as f32;
_18 = _19 <= _5;
_19 = -_5;
_17.1 = core::ptr::addr_of!(_16.fld0);
_8 = _13 - _13;
_9 = _4 | _4;
_12 = [2033843229202643114_i64,(-963727810806990298_i64),(-7348182677791253539_i64)];
RET = 158_u8 as u128;
_20 = _8 as i32;
_3 = _7 + _7;
_19 = 4_usize as f64;
_14 = [_4,_4,_13,_8,_13,_9];
_2 = [_7,_7,_3,_3,_3,_7,_7,_3];
Call(_2 = fn4(_12, _19, _1, _8, _9, _14, _8, _8, _1, _8, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = !_17.3;
_19 = _5;
_18 = _10;
_17.1 = core::ptr::addr_of!(_16.fld0);
_13 = _1 as i16;
_6 = _19 as f32;
_20 = 1016794004_i32;
_5 = _19 * _19;
_9 = !_8;
_17.3 = RET * RET;
_6 = 1131800285055863724_usize as f32;
_16.fld1 = '\u{3cb8b}';
_5 = 27266_u16 as f64;
_17.0 = !_7;
_18 = !_10;
_17.2 = core::ptr::addr_of_mut!(_20);
RET = _17.3 + _17.3;
_24 = 1436163892335890820_u64 - 782839554158849890_u64;
_10 = !_1;
RET = 2748248170259547327_i64 as u128;
_11 = -(-152827006157343697079805604737280958360_i128);
match _20 {
1016794004 => bb11,
_ => bb9
}
}
bb11 = {
_26 = 7972758034990582887_i64;
_3 = _17.0 ^ _7;
RET = _17.3 | _17.3;
_23 = _19 as isize;
_16.fld1 = '\u{831c3}';
_5 = _19;
_20 = !(-852026795_i32);
_6 = _23 as f32;
_27 = !0_usize;
_18 = _8 > _13;
_30 = core::ptr::addr_of!(_28);
Goto(bb12)
}
bb12 = {
_25.fld1 = _16.fld1;
_17.0 = _27 as u32;
RET = _17.3;
(*_30) = 181_u8 as f32;
_25.fld0 = Adt26::Variant2 { fld0: RET };
_12 = [_26,_26,_26];
_17.3 = RET;
_23 = _26 as isize;
(*_30) = _6 * _6;
_27 = _9 as usize;
_11 = (-19631011602196424858073203168539786553_i128) ^ 85793867474504669282260598283778162910_i128;
_4 = !_8;
_12 = [_26,_26,_26];
_22 = &_29;
_28 = _6 + _6;
_5 = _19 * _19;
_16.fld0 = Adt26::Variant2 { fld0: _17.3 };
_28 = -_6;
RET = _17.3 + _17.3;
_14 = [_9,_9,_9,_13,_8,_4];
_4 = _9 >> _8;
_26 = _5 as i64;
RET = Field::<u128>(Variant(_25.fld0, 2), 0);
_16.fld0 = Move(_25.fld0);
_3 = _25.fld1 as u32;
_14 = [_4,_4,_9,_9,_8,_13];
(*_30) = _6 + _6;
Call(_4 = core::intrinsics::transmute(_8), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_30 = core::ptr::addr_of!(_28);
SetDiscriminant(_16.fld0, 2);
_30 = core::ptr::addr_of!(_28);
RET = _17.3 + _17.3;
_31.0 = !_4;
_17.0 = _3;
_19 = _5 + _5;
_10 = _8 < _9;
_5 = _19;
_28 = -_6;
_1 = _26 == _26;
_23 = -9223372036854775807_isize;
_31.2 = _7;
_23 = _26 as isize;
_20 = 348722485_i32 ^ 2059304977_i32;
RET = _17.3 | _17.3;
Goto(bb14)
}
bb14 = {
place!(Field::<u128>(Variant(_16.fld0, 2), 0)) = _17.3 ^ RET;
_35 = [_26,_26,_26,_26,_26,_26];
_23 = _24 as isize;
_25.fld1 = _16.fld1;
_21 = &_35;
_35 = [_26,_26,_26,_26,_26,_26];
_9 = !_4;
_8 = !_9;
_25 = Adt40 { fld0: Move(_16.fld0),fld1: _16.fld1 };
_6 = (*_30) + (*_30);
_16.fld0 = Move(_25.fld0);
(*_30) = -_6;
_25.fld0 = Move(_16.fld0);
_7 = !_31.2;
_37 = core::ptr::addr_of_mut!(_20);
_16 = Adt40 { fld0: Move(_25.fld0),fld1: _25.fld1 };
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(3_usize, 12_usize, Move(_12), 27_usize, Move(_27), 18_usize, Move(_18), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(3_usize, 7_usize, Move(_7), 26_usize, Move(_26), 3_usize, Move(_3), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(3_usize, 2_usize, Move(_2), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [i64; 3],mut _2: f64,mut _3: bool,mut _4: i16,mut _5: i16,mut _6: [i16; 6],mut _7: i16,mut _8: i16,mut _9: bool,mut _10: i16,mut _11: i16) -> [u32; 8] {
mir! {
type RET = [u32; 8];
let _12: f32;
let _13: isize;
let _14: (i32, [isize; 8]);
let _15: isize;
let _16: [i16; 8];
let _17: [i64; 3];
let _18: *const f64;
let _19: Adt63;
let _20: u8;
let _21: [u8; 7];
let _22: u8;
let _23: u64;
let _24: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _25: (&'static &'static u8,);
let _26: *const [u16; 4];
let _27: ([i128; 6], i128, &'static u64, [i16; 6]);
let _28: isize;
let _29: &'static f32;
let _30: [char; 3];
let _31: *mut Adt38;
let _32: f32;
let _33: isize;
let _34: ();
let _35: ();
{
RET = [4103776967_u32,1762225912_u32,1412785507_u32,271732720_u32,3206628933_u32,1866036192_u32,160248787_u32,2594761784_u32];
_11 = _7 & _4;
_12 = (-9223372036854775808_isize) as f32;
_4 = _11;
_6 = [_8,_4,_11,_7,_5,_4];
_3 = _9;
_6 = [_11,_5,_11,_11,_8,_11];
_3 = _9;
_6 = [_5,_11,_4,_7,_10,_11];
_9 = _3;
_3 = _9;
_9 = _3;
_15 = 9223372036854775807_isize;
_4 = 233118776849965588965477841147949049819_u128 as i16;
_15 = _12 as isize;
Goto(bb1)
}
bb1 = {
_14.0 = -(-412937507_i32);
_7 = _14.0 as i16;
_11 = _10 >> _5;
_12 = 73_u8 as f32;
_2 = _14.0 as f64;
_12 = (-140272839305979643388195976952401050351_i128) as f32;
_1 = [(-5180524796069994001_i64),(-5800281429658175578_i64),(-5516568516911062916_i64)];
RET = [4098746733_u32,3291501950_u32,2710281403_u32,1448357019_u32,3578104643_u32,3364867358_u32,327750976_u32,1296083997_u32];
_17 = [(-6768554322768522407_i64),(-4926420730899152599_i64),511425776557471570_i64];
_6 = [_10,_11,_10,_11,_11,_8];
_5 = -_11;
_13 = _15;
_6 = [_5,_5,_5,_11,_11,_8];
_16 = [_5,_5,_10,_11,_11,_10,_8,_8];
_14.1 = [_15,_13,_15,_15,_13,_15,_15,_15];
_5 = _11 >> _11;
_16 = [_11,_5,_5,_11,_5,_5,_5,_8];
_15 = !_13;
_13 = _15;
_18 = core::ptr::addr_of!(_2);
_3 = _11 > _11;
Call(_1 = fn5(_10, _5, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = [_5,_8,_5,_5,_5,_5,_5,_4];
_12 = (*_18) as f32;
(*_18) = 226459498157556337564560844589284213438_u128 as f64;
_14.0 = 1446954373_i32;
_6 = [_10,_8,_11,_11,_5,_10];
RET = [3992776937_u32,2123665530_u32,226983306_u32,1549473818_u32,3653783456_u32,2304670835_u32,2813139291_u32,1116363001_u32];
_16 = [_11,_5,_11,_10,_5,_5,_5,_8];
_10 = _8 >> _11;
RET = [1283705984_u32,707260542_u32,1747528237_u32,348819876_u32,1102906485_u32,118869283_u32,1477923649_u32,1546677138_u32];
_4 = _5;
_3 = !_9;
_6 = [_11,_10,_5,_10,_10,_10];
_17 = _1;
_4 = -_10;
_12 = 3742359043_u32 as f32;
_17 = [(-3792498019999331543_i64),7931300182055120179_i64,(-845669636771858967_i64)];
_9 = _3;
RET = [2906983441_u32,2785114416_u32,2756750082_u32,3711050728_u32,1223758386_u32,3077828043_u32,4074599944_u32,134781503_u32];
_16 = [_10,_4,_8,_4,_10,_8,_8,_8];
match _14.0 {
0 => bb3,
1 => bb4,
2 => bb5,
1446954373 => bb7,
_ => bb6
}
}
bb3 = {
_14.0 = -(-412937507_i32);
_7 = _14.0 as i16;
_11 = _10 >> _5;
_12 = 73_u8 as f32;
_2 = _14.0 as f64;
_12 = (-140272839305979643388195976952401050351_i128) as f32;
_1 = [(-5180524796069994001_i64),(-5800281429658175578_i64),(-5516568516911062916_i64)];
RET = [4098746733_u32,3291501950_u32,2710281403_u32,1448357019_u32,3578104643_u32,3364867358_u32,327750976_u32,1296083997_u32];
_17 = [(-6768554322768522407_i64),(-4926420730899152599_i64),511425776557471570_i64];
_6 = [_10,_11,_10,_11,_11,_8];
_5 = -_11;
_13 = _15;
_6 = [_5,_5,_5,_11,_11,_8];
_16 = [_5,_5,_10,_11,_11,_10,_8,_8];
_14.1 = [_15,_13,_15,_15,_13,_15,_15,_15];
_5 = _11 >> _11;
_16 = [_11,_5,_5,_11,_5,_5,_5,_8];
_15 = !_13;
_13 = _15;
_18 = core::ptr::addr_of!(_2);
_3 = _11 > _11;
Call(_1 = fn5(_10, _5, _3, RET, _3), ReturnTo(bb2), UnwindUnreachable())
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
_20 = _14.0 as u8;
_4 = _10;
RET = [4009616592_u32,1678312231_u32,3539465290_u32,3989645463_u32,909098143_u32,960206632_u32,3867739516_u32,3778279584_u32];
_8 = _4 >> _10;
_1 = [989205687328365602_i64,7929955167597340135_i64,4607798096906155553_i64];
_4 = _5 + _7;
_13 = (*_18) as isize;
_16 = [_10,_5,_10,_8,_8,_8,_10,_5];
Goto(bb8)
}
bb8 = {
_18 = core::ptr::addr_of!((*_18));
_12 = 12123861790563025971_u64 as f32;
_13 = 39_i8 as isize;
_14.0 = 462318768_i32;
_14.1 = [_15,_13,_15,_13,_15,_13,_13,_13];
_4 = _5 * _10;
_22 = !_20;
_15 = _13 ^ _13;
_11 = _10 + _10;
RET = [952215528_u32,1744375718_u32,4047788160_u32,1637170649_u32,712022316_u32,4226274952_u32,454005250_u32,2299026460_u32];
_8 = 136645138080084390869438072270331673381_u128 as i16;
_1 = [(-1475427195320621802_i64),425136770117602126_i64,(-5336312735280433987_i64)];
_11 = _4 + _4;
_12 = _14.0 as f32;
_5 = !_10;
_15 = -_13;
RET = [1121969943_u32,3423128309_u32,1420923134_u32,2919137484_u32,3431614701_u32,1496893835_u32,3597451207_u32,672596720_u32];
_24.1 = (*_18);
_11 = _4;
_20 = _22;
_14.0 = 1916171065_i32;
match _14.0 {
0 => bb6,
1916171065 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_20 = _22;
RET = [2453356504_u32,2444120513_u32,2463079888_u32,3845975828_u32,2321292007_u32,3323424_u32,66111304_u32,2075218804_u32];
_9 = !_3;
_24.2.1 = [(-17_i8),78_i8,29_i8,(-13_i8),(-24_i8),(-116_i8)];
_24.3 = !15617_u16;
(*_18) = -_24.1;
_2 = 5072027306028765834972008737853344125_u128 as f64;
_22 = _20 | _20;
(*_18) = -_24.1;
_24.3 = 21246_u16 - 49793_u16;
_13 = -_15;
_10 = -_4;
_22 = !_20;
_17 = [4014386894863355796_i64,240907859761541652_i64,1432340609513950332_i64];
_13 = _15;
_11 = !_5;
_5 = _10 << _10;
RET = [1349539440_u32,2448906589_u32,4203958272_u32,932712626_u32,1276967288_u32,1994621609_u32,3078518700_u32,3016192389_u32];
_17 = _1;
_24.1 = _2;
_11 = (*_18) as i16;
Goto(bb11)
}
bb11 = {
_15 = _13;
_27.0 = [(-55212922260805638587407704664012883200_i128),65354573108553841333135428155766896450_i128,(-62385022784666053438083384085914262294_i128),(-75187927492013617757473879123204442455_i128),(-27854540771584706915064067217642107549_i128),(-62321528446683959093398666807668641673_i128)];
_27.1 = 69656478900248097655050315880754644152_i128;
_29 = &_12;
_14.1 = [_15,_13,_15,_13,_13,_13,_13,_13];
_9 = !_3;
_11 = -_5;
_24.2.1 = [18_i8,115_i8,104_i8,(-47_i8),(-88_i8),106_i8];
_23 = 10083212345597867603_u64;
_28 = _27.1 as isize;
_2 = (-6482073808163271389_i64) as f64;
_8 = _5;
_12 = _27.1 as f32;
_24.1 = _2 - (*_18);
_6 = [_11,_5,_10,_5,_5,_4];
RET = [3972994528_u32,684860343_u32,3092615664_u32,1537311788_u32,3499795413_u32,2992407473_u32,30428420_u32,1055056948_u32];
_11 = -_10;
_24.3 = 14142_u16;
_27.3 = [_11,_10,_10,_5,_4,_5];
_23 = 18160398385300721804_u64;
_27.0 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
RET = [3525731241_u32,1815236538_u32,1956971899_u32,978960234_u32,1111917623_u32,3818520721_u32,448873013_u32,2887338118_u32];
(*_18) = -_24.1;
_29 = &_32;
match _27.1 {
0 => bb1,
1 => bb9,
69656478900248097655050315880754644152 => bb12,
_ => bb7
}
}
bb12 = {
(*_18) = _24.1 * _24.1;
_32 = 3889061375_u32 as f32;
_18 = core::ptr::addr_of!(_24.1);
Goto(bb13)
}
bb13 = {
_33 = !_28;
_32 = _12 - _12;
_20 = _22 << _10;
_7 = !_8;
RET = [2322076589_u32,3162251266_u32,4233766744_u32,623584518_u32,2003388415_u32,2265552450_u32,194748302_u32,569880802_u32];
_8 = _7 << _10;
_15 = -_13;
(*_18) = _2;
_14.1 = [_13,_15,_15,_13,_33,_33,_33,_28];
RET = [3489981726_u32,1499373096_u32,2382362793_u32,1943284536_u32,4209486206_u32,965883012_u32,978546856_u32,1206168758_u32];
Goto(bb14)
}
bb14 = {
_27.0 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
_20 = _22 >> _5;
_24.0 = _10 >> _5;
_2 = _33 as f64;
_8 = _7;
_28 = _9 as isize;
_27.0 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
_24.1 = _2;
_24.2.0 = (-111_i8) | 68_i8;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(4_usize, 20_usize, Move(_20), 22_usize, Move(_22), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(4_usize, 5_usize, Move(_5), 33_usize, Move(_33), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(4_usize, 7_usize, Move(_7), 14_usize, Move(_14), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i16,mut _2: i16,mut _3: bool,mut _4: [u32; 8],mut _5: bool) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _6: i32;
let _7: (f32, [isize; 8], [bool; 8]);
let _8: *const (u32, *const Adt26, *mut i32, u128);
let _9: i128;
let _10: usize;
let _11: &'static *const Adt63;
let _12: *const [u16; 4];
let _13: *const f64;
let _14: &'static u8;
let _15: char;
let _16: isize;
let _17: bool;
let _18: [u16; 1];
let _19: [u32; 8];
let _20: (Adt31, u8);
let _21: [u32; 2];
let _22: isize;
let _23: [i128; 6];
let _24: [isize; 2];
let _25: *const (u32, *const Adt26, *mut i32, u128);
let _26: *const Adt26;
let _27: char;
let _28: isize;
let _29: (i8, [i8; 6], *const Adt17, Adt22);
let _30: *const Adt63;
let _31: bool;
let _32: isize;
let _33: f64;
let _34: (&'static &'static u8,);
let _35: f32;
let _36: [i16; 6];
let _37: char;
let _38: *mut i32;
let _39: &'static i8;
let _40: u64;
let _41: i128;
let _42: isize;
let _43: [isize; 2];
let _44: isize;
let _45: usize;
let _46: u64;
let _47: isize;
let _48: f64;
let _49: u16;
let _50: isize;
let _51: isize;
let _52: ();
let _53: ();
{
_3 = _5;
_3 = !_5;
_3 = !_5;
_3 = _5 | _5;
_2 = -_1;
_5 = _3;
_1 = _2 + _2;
_5 = !_3;
_6 = 119947710585500044357869317577527108553_i128 as i32;
RET = [5902026099575802669_i64,(-202065240866133131_i64),8528931317307469857_i64];
_5 = !_3;
_3 = _5;
_7.0 = 127_i8 as f32;
_2 = _1;
_2 = '\u{2397a}' as i16;
_2 = _1;
_2 = _7.0 as i16;
_3 = _5 <= _5;
RET = [4064575556496977467_i64,(-1049212951163362982_i64),8972391644308719057_i64];
RET = [(-4317277332083427809_i64),7611120283339923775_i64,5075213858674929253_i64];
RET = [(-2511627222860544437_i64),(-2599719307952276661_i64),(-6787139034557545441_i64)];
_6 = !(-1266690406_i32);
_9 = !(-110614815954051996178450497075524684605_i128);
_10 = !12212559471217157640_usize;
_7.1 = [9223372036854775807_isize,(-52_isize),(-107_isize),(-60_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = 2854336124_u32 as i128;
_5 = _3 > _3;
Call(RET = fn6(_3, _3, _5, _5, _1, _5, _5, _5, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _3 < _3;
_3 = _5;
_3 = _5;
_9 = 123233091627393205004408050064079530777_i128 - (-34885275582322429763559143728200273891_i128);
_5 = !_3;
RET = [(-2971497423552208976_i64),1333722759247165549_i64,(-4041463264524442905_i64)];
RET = [6075966253449642635_i64,(-8857737755717039001_i64),4972745739866161129_i64];
_6 = -(-497842867_i32);
_7.2 = [_3,_5,_5,_3,_3,_3,_5,_5];
_2 = _1;
_6 = _9 as i32;
RET = [452060610637661044_i64,(-4655312699169926784_i64),6663323491020003050_i64];
_5 = _3 == _3;
_7.2 = [_3,_3,_5,_5,_5,_3,_3,_5];
_9 = (-9223372036854775808_isize) as i128;
_1 = 241_u8 as i16;
_7.2 = [_3,_5,_3,_3,_5,_3,_5,_3];
_6 = !1670210335_i32;
_15 = '\u{b99cf}';
Goto(bb2)
}
bb2 = {
_17 = _5;
_17 = _5;
_7.0 = _10 as f32;
_17 = _5;
_7.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-124_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_10 = !1_usize;
Goto(bb3)
}
bb3 = {
_10 = 16689238667813174640_usize ^ 0_usize;
_7.2 = [_3,_5,_17,_5,_3,_17,_17,_17];
_2 = 2556240273525566577_i64 as i16;
_17 = _3 & _3;
_7.2 = [_3,_5,_5,_3,_3,_3,_5,_17];
_14 = &_20.1;
_16 = 13603_u16 as isize;
RET = [1689378241931843299_i64,(-4727739159990754341_i64),(-98031619284776710_i64)];
_19 = [2909608025_u32,2601815613_u32,4163900109_u32,3228701637_u32,3011564420_u32,3562006835_u32,531236542_u32,2487979407_u32];
_7.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_15 = '\u{d4adc}';
_23 = [_9,_9,_9,_9,_9,_9];
RET = [(-2578069742906831417_i64),(-5660400914213270507_i64),(-5022865034695041029_i64)];
_21 = [1248378714_u32,993966163_u32];
_22 = 239_u8 as isize;
_3 = _17;
_19 = _4;
_19 = [3923738618_u32,277443322_u32,3494242090_u32,286106314_u32,3087874279_u32,3884444389_u32,1166790389_u32,668046468_u32];
_7.1 = [_22,_16,_16,_16,_22,_22,_22,_16];
_5 = !_3;
_18 = [46837_u16];
_3 = _17;
Goto(bb4)
}
bb4 = {
_17 = _5 == _3;
_3 = !_17;
RET = [9006569955274592393_i64,326597376760962622_i64,(-2786992946171676935_i64)];
_7.2 = [_5,_3,_5,_3,_17,_3,_5,_5];
_18 = [36478_u16];
_20.1 = !253_u8;
_27 = _15;
Call(_16 = fn9(_7, _5, _3, _5, _7, _3, _7.2, _7.2, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = _7.0 as isize;
Goto(bb6)
}
bb6 = {
_31 = !_5;
_21 = [3810098975_u32,3535155436_u32];
_2 = 339402125295027574871978023958656373102_u128 as i16;
_7.1 = [_22,_22,_16,_16,_22,_22,_22,_22];
_28 = !_16;
_33 = _6 as f64;
_19 = [2186767918_u32,3465885894_u32,3680799893_u32,676455650_u32,3669649137_u32,771703078_u32,3066565514_u32,862573088_u32];
_7.2 = [_31,_5,_5,_17,_5,_5,_17,_5];
RET = [406961251722089387_i64,(-131998241095543824_i64),(-6420153696445780412_i64)];
_20.1 = (-6773899267404561497_i64) as u8;
_11 = &_30;
_18 = [21967_u16];
_34.0 = &_14;
_13 = core::ptr::addr_of!(_33);
_16 = _10 as isize;
_36 = [_1,_2,_2,_2,_2,_2];
_7.2 = [_5,_31,_5,_17,_31,_31,_31,_5];
_13 = core::ptr::addr_of!((*_13));
_31 = _7.0 <= _7.0;
Goto(bb7)
}
bb7 = {
_5 = _17 == _3;
RET = [(-1400793348780234467_i64),2696487444395843233_i64,(-3494365127459456317_i64)];
_35 = _7.0 + _7.0;
_7.0 = _35;
_32 = _16;
_37 = _15;
_24 = [_28,_22];
(*_13) = 5769151381100993691_u64 as f64;
_39 = &_29.0;
_7.1 = [_22,_16,_32,_16,_22,_32,_32,_32];
_33 = _20.1 as f64;
RET = [613809511052694388_i64,7612397891741820752_i64,199022128196007285_i64];
_1 = _2;
_15 = _37;
_38 = core::ptr::addr_of_mut!(_6);
_31 = _3 & _17;
Goto(bb8)
}
bb8 = {
_24 = [_22,_32];
_40 = 2624440604_u32 as u64;
_5 = _17 != _17;
_5 = _3 & _3;
_23 = [_9,_9,_9,_9,_9,_9];
_29.1 = [(-104_i8),108_i8,90_i8,26_i8,(-59_i8),50_i8];
_28 = _16;
_1 = -_2;
_40 = 2474818972965744200_u64 & 16875015122982793674_u64;
(*_38) = 812939266_i32;
_2 = 56_i8 as i16;
_20.1 = 23_u8 & 189_u8;
_16 = _3 as isize;
_21 = [2932911600_u32,85846123_u32];
_2 = _9 as i16;
_10 = 2_usize - 7023733866656080052_usize;
_42 = 7175522127855938205_i64 as isize;
_16 = _1 as isize;
RET = [(-5230534723720670100_i64),(-3736253340785235501_i64),(-180724704896924945_i64)];
_29.0 = (-114_i8) - (-108_i8);
Goto(bb9)
}
bb9 = {
_39 = &_29.0;
_44 = !_32;
_42 = _44;
_41 = _9;
(*_38) = !1144357166_i32;
_9 = 4248688388_u32 as i128;
_9 = -_41;
_1 = (*_13) as i16;
_22 = !_44;
_17 = !_3;
_2 = _1 & _1;
_4 = _19;
_16 = _44 << _6;
_17 = !_3;
_3 = _31 | _5;
_29.0 = 47_i8;
_4 = [3165785666_u32,2031727784_u32,1597631856_u32,3339184456_u32,136583087_u32,249608984_u32,1198900578_u32,2519253408_u32];
Goto(bb10)
}
bb10 = {
_7.1 = [_28,_16,_32,_16,_16,_44,_22,_32];
(*_38) = -509316951_i32;
_16 = _41 as isize;
_38 = core::ptr::addr_of_mut!((*_38));
_47 = !_42;
(*_38) = 1794322614_i32;
_44 = !_22;
_41 = _9;
_40 = 16550074257280656447_u64;
_28 = 25819_u16 as isize;
RET = [(-3796768483923171740_i64),7207015888022741354_i64,(-6030404558184927387_i64)];
_38 = core::ptr::addr_of_mut!((*_38));
_28 = _42;
_20.1 = !35_u8;
_36 = [_2,_1,_1,_2,_2,_2];
_29.1 = [_29.0,_29.0,_29.0,_29.0,_29.0,_29.0];
RET = [(-6869961482843013092_i64),(-7689181995243967574_i64),73882759024368007_i64];
_19 = [1647609494_u32,131961948_u32,1052249739_u32,4037029144_u32,1527418298_u32,2023464490_u32,3507857293_u32,801745598_u32];
Call(_49 = core::intrinsics::transmute(_2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_4 = [1442515362_u32,1913832423_u32,670291974_u32,1441161014_u32,2506020817_u32,32811769_u32,1873528169_u32,4289093326_u32];
_36 = [_1,_2,_2,_1,_2,_2];
Goto(bb12)
}
bb12 = {
_18 = [_49];
_5 = _17 == _17;
_44 = !_42;
_51 = _2 as isize;
_28 = _10 as isize;
_32 = _42;
_48 = (*_38) as f64;
(*_38) = 521550563_i32 - 1098749460_i32;
_33 = _48;
match _40 {
16550074257280656447 => bb14,
_ => bb13
}
}
bb13 = {
_24 = [_22,_32];
_40 = 2624440604_u32 as u64;
_5 = _17 != _17;
_5 = _3 & _3;
_23 = [_9,_9,_9,_9,_9,_9];
_29.1 = [(-104_i8),108_i8,90_i8,26_i8,(-59_i8),50_i8];
_28 = _16;
_1 = -_2;
_40 = 2474818972965744200_u64 & 16875015122982793674_u64;
(*_38) = 812939266_i32;
_2 = 56_i8 as i16;
_20.1 = 23_u8 & 189_u8;
_16 = _3 as isize;
_21 = [2932911600_u32,85846123_u32];
_2 = _9 as i16;
_10 = 2_usize - 7023733866656080052_usize;
_42 = 7175522127855938205_i64 as isize;
_16 = _1 as isize;
RET = [(-5230534723720670100_i64),(-3736253340785235501_i64),(-180724704896924945_i64)];
_29.0 = (-114_i8) - (-108_i8);
Goto(bb9)
}
bb14 = {
_18 = [_49];
_41 = (-3642958524430023773_i64) as i128;
Goto(bb15)
}
bb15 = {
Call(_52 = dump_var(5_usize, 49_usize, Move(_49), 9_usize, Move(_9), 23_usize, Move(_23), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_52 = dump_var(5_usize, 16_usize, Move(_16), 36_usize, Move(_36), 42_usize, Move(_42), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_52 = dump_var(5_usize, 6_usize, Move(_6), 3_usize, Move(_3), 28_usize, Move(_28), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_52 = dump_var(5_usize, 1_usize, Move(_1), 44_usize, Move(_44), 51_usize, Move(_51), 53_usize, _53), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: i16,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _11: &'static [i64; 6];
let _12: (&'static [i64; 3], f32, *const (u32, *const Adt26, *mut i32, u128));
let _13: u32;
let _14: (f32, [isize; 8], [bool; 8]);
let _15: [i32; 6];
let _16: (i128, *const Adt17, (Adt38,));
let _17: (i32, Adt17, u64);
let _18: u32;
let _19: u64;
let _20: (Adt31, u8);
let _21: f64;
let _22: &'static &'static u8;
let _23: *mut *const Adt63;
let _24: [i64; 3];
let _25: &'static u8;
let _26: ([i128; 6], i128, &'static u64, [i16; 6]);
let _27: [i128; 6];
let _28: &'static i8;
let _29: i32;
let _30: (f32, [isize; 8], [bool; 8]);
let _31: Adt38;
let _32: Adt26;
let _33: (Adt31, u8);
let _34: *const (Adt38,);
let _35: u8;
let _36: Adt31;
let _37: ();
let _38: ();
{
_5 = (-4276_i16);
_8 = !_3;
_8 = _2 <= _1;
_1 = _6;
_7 = !_3;
_8 = !_9;
_5 = 27101_i16;
RET = [1327731615341420626_i64,9046442994157218108_i64,5404630081495827936_i64];
_12.1 = 6040700699042051106_u64 as f32;
_7 = _4 ^ _6;
RET = [9147010229548980615_i64,(-5079639291006769611_i64),6076919905945780792_i64];
RET = [(-2251366082700498155_i64),2237451688123401035_i64,(-1143138379387795690_i64)];
_12.0 = &RET;
_12.1 = (-658013423_i32) as f32;
_3 = _7;
_7 = !_10;
_2 = _9 >= _3;
_1 = !_10;
_9 = _2;
RET = [(-5033201337951523281_i64),(-4294972526960569879_i64),6097773453754232940_i64];
_3 = _10;
_8 = _2 == _6;
Goto(bb1)
}
bb1 = {
_15 = [(-681968159_i32),(-783834525_i32),1411966791_i32,(-1530346392_i32),(-1194837235_i32),1359569171_i32];
_12.1 = 222_u8 as f32;
_17.1.fld4 = _5 - _5;
_16.2.0.fld2.1 = !0_u8;
_16.2.0.fld1.0 = -91_i8;
_6 = _2 > _8;
_16.2.0.fld3 = _16.2.0.fld1.0 ^ _16.2.0.fld1.0;
_17.1 = Adt17 { fld0: 74301986357156105423517085347473534075_u128,fld1: '\u{d4f18}',fld2: (-9223372036854775808_isize),fld3: _16.2.0.fld3,fld4: _5,fld5: 648583181_i32 };
_16.2.0.fld2.1 = 54_u8;
_3 = !_8;
_17.0 = _16.2.0.fld2.1 as i32;
_9 = !_4;
_16.1 = core::ptr::addr_of!(_17.1);
Goto(bb2)
}
bb2 = {
_12.1 = _17.0 as f32;
_6 = _4 == _8;
_18 = 3909522683_u32;
_17.0 = _17.1.fld5 << _17.1.fld0;
_17.1.fld1 = '\u{6374d}';
_17.1.fld1 = '\u{e572b}';
_17.1.fld1 = '\u{6a20a}';
_19 = 4_usize as u64;
match _17.1.fld5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
648583181 => bb8,
_ => bb7
}
}
bb3 = {
_15 = [(-681968159_i32),(-783834525_i32),1411966791_i32,(-1530346392_i32),(-1194837235_i32),1359569171_i32];
_12.1 = 222_u8 as f32;
_17.1.fld4 = _5 - _5;
_16.2.0.fld2.1 = !0_u8;
_16.2.0.fld1.0 = -91_i8;
_6 = _2 > _8;
_16.2.0.fld3 = _16.2.0.fld1.0 ^ _16.2.0.fld1.0;
_17.1 = Adt17 { fld0: 74301986357156105423517085347473534075_u128,fld1: '\u{d4f18}',fld2: (-9223372036854775808_isize),fld3: _16.2.0.fld3,fld4: _5,fld5: 648583181_i32 };
_16.2.0.fld2.1 = 54_u8;
_3 = !_8;
_17.0 = _16.2.0.fld2.1 as i32;
_9 = !_4;
_16.1 = core::ptr::addr_of!(_17.1);
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
_17.0 = _17.1.fld5;
_12.1 = _19 as f32;
_10 = _9 != _1;
_16.2.0.fld1.1 = [_16.2.0.fld3,_17.1.fld3,_16.2.0.fld3,_17.1.fld3,_16.2.0.fld3,_17.1.fld3];
_2 = !_1;
RET = [(-3456079295252588829_i64),6322999594512952183_i64,(-5795739699471847365_i64)];
_17.1.fld2 = 59625_u16 as isize;
_12.0 = &RET;
_20.1 = _16.2.0.fld2.1;
_14.1 = [_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2];
_1 = !_6;
_12.1 = _18 as f32;
_14.2 = [_10,_7,_8,_8,_6,_9,_8,_4];
_16.2.0.fld1.0 = !_16.2.0.fld3;
_2 = _9 < _9;
_5 = _17.1.fld4 >> _17.1.fld5;
_16.2.0.fld1.2 = core::ptr::addr_of!(_17.1);
_5 = !_17.1.fld4;
_4 = !_2;
RET = [7660694237455443105_i64,5374999706431669042_i64,(-4195459259690850340_i64)];
match _17.1.fld0 {
0 => bb4,
1 => bb2,
74301986357156105423517085347473534075 => bb10,
_ => bb9
}
}
bb9 = {
_15 = [(-681968159_i32),(-783834525_i32),1411966791_i32,(-1530346392_i32),(-1194837235_i32),1359569171_i32];
_12.1 = 222_u8 as f32;
_17.1.fld4 = _5 - _5;
_16.2.0.fld2.1 = !0_u8;
_16.2.0.fld1.0 = -91_i8;
_6 = _2 > _8;
_16.2.0.fld3 = _16.2.0.fld1.0 ^ _16.2.0.fld1.0;
_17.1 = Adt17 { fld0: 74301986357156105423517085347473534075_u128,fld1: '\u{d4f18}',fld2: (-9223372036854775808_isize),fld3: _16.2.0.fld3,fld4: _5,fld5: 648583181_i32 };
_16.2.0.fld2.1 = 54_u8;
_3 = !_8;
_17.0 = _16.2.0.fld2.1 as i32;
_9 = !_4;
_16.1 = core::ptr::addr_of!(_17.1);
Goto(bb2)
}
bb10 = {
_14.0 = _12.1;
_16.2.0.fld1.1 = [_16.2.0.fld3,_17.1.fld3,_17.1.fld3,_16.2.0.fld3,_16.2.0.fld1.0,_17.1.fld3];
_14.1 = [_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2];
_7 = !_8;
_16.2.0.fld3 = (-33094110022900142660939745047543119168_i128) as i8;
_16.0 = !43337737419185611840907087291520369535_i128;
_12.1 = -_14.0;
_13 = _18 * _18;
_24 = RET;
_21 = _16.2.0.fld1.0 as f64;
_8 = _6 & _10;
_17.1.fld3 = _16.2.0.fld3;
_16.2.0.fld0 = [_17.1.fld4,_17.1.fld4,_5,_17.1.fld4,_17.1.fld4,_17.1.fld4];
_1 = _10 | _2;
_17.2 = _19;
_17.0 = _2 as i32;
_16.0 = _21 as i128;
_17.1.fld0 = 191332299966555115620833036928944618779_u128 & 153072946770201539174105127173034129259_u128;
_13 = 5503158169864215176_i64 as u32;
Call(_13 = fn7(_10, _7, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_25 = &_16.2.0.fld2.1;
_17.2 = _19 << _13;
_16.2.0.fld1.1 = [_16.2.0.fld3,_16.2.0.fld1.0,_16.2.0.fld1.0,_17.1.fld3,_16.2.0.fld1.0,_16.2.0.fld3];
_9 = _4 & _4;
_17.1.fld4 = _13 as i16;
_21 = _12.1 as f64;
_26.2 = &_17.2;
_10 = _13 >= _13;
_17.1 = Adt17 { fld0: 51619517098770614260517727941127586073_u128,fld1: '\u{244bb}',fld2: 36_isize,fld3: _16.2.0.fld3,fld4: _5,fld5: _17.0 };
_26.1 = _16.0 * _16.0;
_17.1.fld2 = !(-71_isize);
_1 = _17.1.fld1 <= _17.1.fld1;
_25 = &(*_25);
_17.1 = Adt17 { fld0: 339479221793729340293460533123303176234_u128,fld1: '\u{c8f6}',fld2: (-9223372036854775808_isize),fld3: _16.2.0.fld3,fld4: _5,fld5: _17.0 };
_30.2 = _14.2;
RET = [9142184989308826733_i64,4913271728829186576_i64,(-4263315717308731666_i64)];
_16.2.0.fld0 = [_17.1.fld4,_17.1.fld4,_5,_5,_17.1.fld4,_17.1.fld4];
_30 = (_12.1, _14.1, _14.2);
_16.2.0.fld3 = !_16.2.0.fld1.0;
_28 = &_16.2.0.fld3;
_25 = &(*_25);
_17.2 = 3_usize as u64;
_8 = !_2;
_17.1.fld0 = 95238856275500110457714433442883556968_u128 >> _17.1.fld2;
_26.2 = &_19;
_30 = (_12.1, _14.1, _14.2);
_16.0 = _17.1.fld0 as i128;
Call(_17.1.fld3 = core::intrinsics::bswap(_16.2.0.fld3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22 = &_25;
_9 = _7 < _4;
_16.2.0.fld1.2 = core::ptr::addr_of!(_17.1);
_26.2 = &_19;
_14 = (_30.0, _30.1, _30.2);
_16.2.0.fld1.2 = core::ptr::addr_of!(_17.1);
_17.1.fld4 = _5 | _5;
_10 = _9;
_1 = _2 | _3;
_30 = (_12.1, _14.1, _14.2);
_16.2.0.fld1.0 = _16.2.0.fld3;
_31.fld0 = _16.2.0.fld0;
_12.1 = _14.0 + _30.0;
RET = [8281016796420833689_i64,(-6193689135532914121_i64),(-1406675727253564002_i64)];
_31.fld1.2 = core::ptr::addr_of!(_17.1);
_14.0 = _30.0;
_31.fld3 = !_17.1.fld3;
_9 = !_7;
match _17.1.fld2 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
340282366920938463454151235394913435648 => bb20,
_ => bb19
}
}
bb13 = {
_25 = &_16.2.0.fld2.1;
_17.2 = _19 << _13;
_16.2.0.fld1.1 = [_16.2.0.fld3,_16.2.0.fld1.0,_16.2.0.fld1.0,_17.1.fld3,_16.2.0.fld1.0,_16.2.0.fld3];
_9 = _4 & _4;
_17.1.fld4 = _13 as i16;
_21 = _12.1 as f64;
_26.2 = &_17.2;
_10 = _13 >= _13;
_17.1 = Adt17 { fld0: 51619517098770614260517727941127586073_u128,fld1: '\u{244bb}',fld2: 36_isize,fld3: _16.2.0.fld3,fld4: _5,fld5: _17.0 };
_26.1 = _16.0 * _16.0;
_17.1.fld2 = !(-71_isize);
_1 = _17.1.fld1 <= _17.1.fld1;
_25 = &(*_25);
_17.1 = Adt17 { fld0: 339479221793729340293460533123303176234_u128,fld1: '\u{c8f6}',fld2: (-9223372036854775808_isize),fld3: _16.2.0.fld3,fld4: _5,fld5: _17.0 };
_30.2 = _14.2;
RET = [9142184989308826733_i64,4913271728829186576_i64,(-4263315717308731666_i64)];
_16.2.0.fld0 = [_17.1.fld4,_17.1.fld4,_5,_5,_17.1.fld4,_17.1.fld4];
_30 = (_12.1, _14.1, _14.2);
_16.2.0.fld3 = !_16.2.0.fld1.0;
_28 = &_16.2.0.fld3;
_25 = &(*_25);
_17.2 = 3_usize as u64;
_8 = !_2;
_17.1.fld0 = 95238856275500110457714433442883556968_u128 >> _17.1.fld2;
_26.2 = &_19;
_30 = (_12.1, _14.1, _14.2);
_16.0 = _17.1.fld0 as i128;
Call(_17.1.fld3 = core::intrinsics::bswap(_16.2.0.fld3), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_14.0 = _12.1;
_16.2.0.fld1.1 = [_16.2.0.fld3,_17.1.fld3,_17.1.fld3,_16.2.0.fld3,_16.2.0.fld1.0,_17.1.fld3];
_14.1 = [_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2];
_7 = !_8;
_16.2.0.fld3 = (-33094110022900142660939745047543119168_i128) as i8;
_16.0 = !43337737419185611840907087291520369535_i128;
_12.1 = -_14.0;
_13 = _18 * _18;
_24 = RET;
_21 = _16.2.0.fld1.0 as f64;
_8 = _6 & _10;
_17.1.fld3 = _16.2.0.fld3;
_16.2.0.fld0 = [_17.1.fld4,_17.1.fld4,_5,_17.1.fld4,_17.1.fld4,_17.1.fld4];
_1 = _10 | _2;
_17.2 = _19;
_17.0 = _2 as i32;
_16.0 = _21 as i128;
_17.1.fld0 = 191332299966555115620833036928944618779_u128 & 153072946770201539174105127173034129259_u128;
_13 = 5503158169864215176_i64 as u32;
Call(_13 = fn7(_10, _7, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_12.1 = _17.0 as f32;
_6 = _4 == _8;
_18 = 3909522683_u32;
_17.0 = _17.1.fld5 << _17.1.fld0;
_17.1.fld1 = '\u{6374d}';
_17.1.fld1 = '\u{e572b}';
_17.1.fld1 = '\u{6a20a}';
_19 = 4_usize as u64;
match _17.1.fld5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
648583181 => bb8,
_ => bb7
}
}
bb16 = {
_17.0 = _17.1.fld5;
_12.1 = _19 as f32;
_10 = _9 != _1;
_16.2.0.fld1.1 = [_16.2.0.fld3,_17.1.fld3,_16.2.0.fld3,_17.1.fld3,_16.2.0.fld3,_17.1.fld3];
_2 = !_1;
RET = [(-3456079295252588829_i64),6322999594512952183_i64,(-5795739699471847365_i64)];
_17.1.fld2 = 59625_u16 as isize;
_12.0 = &RET;
_20.1 = _16.2.0.fld2.1;
_14.1 = [_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2,_17.1.fld2];
_1 = !_6;
_12.1 = _18 as f32;
_14.2 = [_10,_7,_8,_8,_6,_9,_8,_4];
_16.2.0.fld1.0 = !_16.2.0.fld3;
_2 = _9 < _9;
_5 = _17.1.fld4 >> _17.1.fld5;
_16.2.0.fld1.2 = core::ptr::addr_of!(_17.1);
_5 = !_17.1.fld4;
_4 = !_2;
RET = [7660694237455443105_i64,5374999706431669042_i64,(-4195459259690850340_i64)];
match _17.1.fld0 {
0 => bb4,
1 => bb2,
74301986357156105423517085347473534075 => bb10,
_ => bb9
}
}
bb17 = {
Return()
}
bb18 = {
_15 = [(-681968159_i32),(-783834525_i32),1411966791_i32,(-1530346392_i32),(-1194837235_i32),1359569171_i32];
_12.1 = 222_u8 as f32;
_17.1.fld4 = _5 - _5;
_16.2.0.fld2.1 = !0_u8;
_16.2.0.fld1.0 = -91_i8;
_6 = _2 > _8;
_16.2.0.fld3 = _16.2.0.fld1.0 ^ _16.2.0.fld1.0;
_17.1 = Adt17 { fld0: 74301986357156105423517085347473534075_u128,fld1: '\u{d4f18}',fld2: (-9223372036854775808_isize),fld3: _16.2.0.fld3,fld4: _5,fld5: 648583181_i32 };
_16.2.0.fld2.1 = 54_u8;
_3 = !_8;
_17.0 = _16.2.0.fld2.1 as i32;
_9 = !_4;
_16.1 = core::ptr::addr_of!(_17.1);
Goto(bb2)
}
bb19 = {
Return()
}
bb20 = {
_33.1 = _30.0 as u8;
_20.1 = (-8382534483437821982_i64) as u8;
_25 = &_16.2.0.fld2.1;
_16.1 = Move(_16.2.0.fld1.2);
_16.1 = core::ptr::addr_of!(_17.1);
_8 = !_7;
_7 = _3;
_25 = &(*_25);
_34 = core::ptr::addr_of!(_16.2);
_18 = (*_25) as u32;
_21 = _16.0 as f64;
_31.fld0 = [_17.1.fld4,_5,_17.1.fld4,_17.1.fld4,_5,_17.1.fld4];
(*_34).0.fld1.3 = Adt22::Variant1 { fld0: _19,fld1: Move(_17),fld2: _17.1.fld5 };
(*_34).0.fld1.2 = core::ptr::addr_of!(_17.1);
_16.2.0.fld1.2 = core::ptr::addr_of!(place!(Field::<(i32, Adt17, u64)>(Variant((*_34).0.fld1.3, 1), 1)).1);
_16.2.0.fld2.1 = _33.1;
_3 = _1;
_30.1 = [Field::<(i32, Adt17, u64)>(Variant((*_34).0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant(_16.2.0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant((*_34).0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant((*_34).0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant(_16.2.0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant((*_34).0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant((*_34).0.fld1.3, 1), 1).1.fld2,Field::<(i32, Adt17, u64)>(Variant(_16.2.0.fld1.3, 1), 1).1.fld2];
_31.fld4 = core::ptr::addr_of!(_32);
Goto(bb21)
}
bb21 = {
Call(_37 = dump_var(6_usize, 18_usize, Move(_18), 5_usize, Move(_5), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_37 = dump_var(6_usize, 3_usize, Move(_3), 19_usize, Move(_19), 2_usize, Move(_2), 38_usize, _38), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: bool,mut _3: bool) -> u32 {
mir! {
type RET = u32;
let _4: (u32, *const Adt26, *mut i32, u128);
let _5: &'static [i64; 3];
let _6: bool;
let _7: f64;
let _8: isize;
let _9: ();
let _10: ();
{
RET = 188_u8 as u32;
_1 = !_2;
_2 = !_3;
_3 = _1 & _2;
_3 = _1 >= _1;
RET = 1614447026_u32 + 1797556968_u32;
_3 = _1 <= _2;
_2 = !_1;
Goto(bb1)
}
bb1 = {
_1 = _2;
_3 = _2 != _2;
_3 = _1 != _1;
_3 = _2 >= _1;
_2 = _3;
RET = !1915281052_u32;
_4.3 = 215_u8 as u128;
_4.3 = 271531990499023840244215074995216461044_u128 >> RET;
RET = !3359753212_u32;
_2 = _1;
_2 = _3 & _1;
_4.0 = RET;
_3 = _2;
Call(_3 = fn8(_1, _2, _1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = !_1;
_4.0 = _2 as u32;
RET = _4.0;
_2 = !_1;
RET = _4.0;
_1 = _2;
_6 = !_3;
_2 = !_1;
_2 = _6;
_1 = _6;
Goto(bb3)
}
bb3 = {
Call(_9 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 10_usize, _10, 10_usize, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: [bool; 8];
let _7: [i128; 6];
let _8: i128;
let _9: [u16; 4];
let _10: &'static i8;
let _11: &'static f32;
let _12: ();
let _13: ();
{
_1 = !_5;
_2 = !_1;
_1 = !_3;
_5 = !_3;
_6 = [_1,_4,_2,_4,_1,_2,_4,_1];
RET = _4;
_2 = RET == RET;
RET = _3;
RET = _4;
_2 = !_1;
_1 = !_4;
_7 = [(-150640341205130847933324263281539022165_i128),74159182438124163843855812460332655825_i128,(-103249179793773243777433801631625165490_i128),23050622627910045575059063996549881290_i128,19459851223767757783040856696750280899_i128,(-152324008974739825524665607049440764973_i128)];
_8 = 9619_u16 as i128;
_8 = -135765404549990424133362557251928021533_i128;
_7 = [_8,_8,_8,_8,_8,_8];
_5 = !_4;
_5 = !_2;
_2 = !_1;
_8 = (-27424277532102794667230320276753101925_i128) << 117692349620125936245945527557957012818_u128;
_3 = !RET;
_8 = 4584472592009944288_u64 as i128;
_5 = RET != _3;
RET = _1 == _1;
_2 = _3 ^ RET;
_9 = [13716_u16,25445_u16,18599_u16,42905_u16];
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(8_usize, 8_usize, Move(_8), 1_usize, Move(_1), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: (f32, [isize; 8], [bool; 8]),mut _2: bool,mut _3: bool,mut _4: bool,mut _5: (f32, [isize; 8], [bool; 8]),mut _6: bool,mut _7: [bool; 8],mut _8: [bool; 8],mut _9: (f32, [isize; 8], [bool; 8])) -> isize {
mir! {
type RET = isize;
let _10: f64;
let _11: u8;
let _12: i16;
let _13: [i8; 2];
let _14: *const Adt63;
let _15: (i8, [i8; 6], *const Adt17, Adt22);
let _16: isize;
let _17: [i64; 6];
let _18: &'static &'static u8;
let _19: bool;
let _20: [u32; 2];
let _21: isize;
let _22: [i8; 2];
let _23: Adt31;
let _24: isize;
let _25: u8;
let _26: i64;
let _27: u128;
let _28: &'static isize;
let _29: [i8; 2];
let _30: bool;
let _31: [i128; 2];
let _32: bool;
let _33: [i128; 2];
let _34: [isize; 2];
let _35: u16;
let _36: *const (Adt38,);
let _37: i64;
let _38: isize;
let _39: (i8, [i8; 6], *const Adt17, Adt22);
let _40: [u8; 7];
let _41: f64;
let _42: bool;
let _43: (Adt38,);
let _44: &'static &'static u8;
let _45: [i32; 6];
let _46: (i8, [i8; 6], *const Adt17, Adt22);
let _47: *mut i32;
let _48: *const Adt26;
let _49: u16;
let _50: ();
let _51: ();
{
RET = !(-60_isize);
_9.0 = _1.0;
RET = 41_isize;
_7 = [_3,_6,_2,_3,_4,_4,_3,_3];
_5.2 = _8;
_5 = (_1.0, _9.1, _9.2);
_1 = _5;
_1.0 = -_5.0;
_5.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9.2 = _8;
RET = 9223372036854775807_isize - 116_isize;
_8 = _7;
_1 = _5;
_5.2 = _9.2;
_1.2 = [_4,_3,_2,_6,_6,_4,_4,_6];
_3 = _4 <= _2;
_6 = !_2;
Goto(bb1)
}
bb1 = {
_5.1 = _1.1;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
_2 = !_6;
_3 = _2 < _2;
_8 = [_3,_6,_3,_6,_6,_3,_2,_2];
_9.2 = [_3,_6,_2,_2,_6,_3,_4,_2];
_5.0 = _1.0 + _9.0;
_10 = 34740_u16 as f64;
_1 = (_5.0, _5.1, _5.2);
_9.0 = -_5.0;
_1.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5.2 = [_6,_2,_6,_4,_3,_4,_4,_4];
_13 = [90_i8,5_i8];
Goto(bb2)
}
bb2 = {
_5.2 = _1.2;
RET = -9223372036854775807_isize;
_15.0 = -117_i8;
_1 = _5;
_5.0 = _9.0;
_9.0 = 1040019997_i32 as f32;
_7 = [_4,_3,_3,_2,_2,_2,_4,_3];
Call(_5.2 = fn10(_1, _9.2, _8, _7, _8, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _2;
_3 = _4 > _6;
_5.2 = _9.2;
_5 = (_9.0, _1.1, _7);
_10 = (-2549_i16) as f64;
_9.2 = [_6,_4,_3,_6,_6,_3,_6,_3];
_12 = -(-19698_i16);
_1 = (_9.0, _9.1, _5.2);
_1.0 = RET as f32;
_1.2 = _8;
_3 = RET <= RET;
_16 = RET ^ RET;
_9.2 = [_2,_6,_4,_2,_3,_2,_2,_6];
_8 = [_6,_4,_4,_2,_6,_2,_4,_2];
_15.1 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_15.0 = !29_i8;
_6 = !_2;
_4 = _2 <= _2;
_4 = _6 < _2;
_9.1 = _5.1;
_9.1 = [_16,_16,RET,_16,_16,_16,_16,_16];
_15.0 = 109_i8;
Goto(bb4)
}
bb4 = {
_11 = _6 as u8;
_7 = _1.2;
_2 = !_4;
_12 = (-5449974795869617494_i64) as i16;
match _15.0 {
0 => bb1,
1 => bb2,
109 => bb5,
_ => bb3
}
}
bb5 = {
_9 = (_1.0, _5.1, _1.2);
_1 = (_9.0, _9.1, _9.2);
RET = _16 & _16;
_1.0 = _5.0;
RET = _16;
_11 = 5195685461322931186_usize as u8;
_20 = [1656209478_u32,3362551688_u32];
_9.0 = _1.0 * _1.0;
_20 = [1412335321_u32,358903361_u32];
_16 = RET ^ RET;
_20 = [2946477042_u32,2748910424_u32];
_12 = 3006484176_u32 as i16;
_15.1 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_9 = _5;
_16 = RET;
_12 = '\u{dcce3}' as i16;
_15.0 = !12_i8;
RET = _12 as isize;
_10 = _11 as f64;
_10 = 291882554766736053033657536384340369301_u128 as f64;
_17 = [8270176145842503341_i64,(-3309514943820930963_i64),(-1796367677599216061_i64),(-2264686150705867585_i64),(-8499283409564941245_i64),(-5244579820792379697_i64)];
_10 = _12 as f64;
_12 = !(-9645_i16);
_10 = 1_usize as f64;
_22 = [_15.0,_15.0];
_17 = [(-3285202986254412125_i64),(-9174836141245430942_i64),8112634866081212810_i64,4054717831233805173_i64,(-3431013562896222004_i64),3268341146864804429_i64];
_1 = (_5.0, _5.1, _7);
_19 = _6 < _2;
_1.1 = [_16,RET,_16,_16,_16,_16,_16,RET];
Call(_8 = fn11(_1.2, _5, _2, _2, _9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1.0 = _9.0 * _5.0;
_17 = [2598177312163440378_i64,1655721494288494936_i64,(-3933324101457703970_i64),5224005101716813741_i64,(-2352307017556071330_i64),(-245085117500401608_i64)];
_21 = _15.0 as isize;
_9 = (_1.0, _1.1, _5.2);
_5 = (_1.0, _9.1, _8);
_13 = [_15.0,_15.0];
_6 = _5.0 == _1.0;
_1.2 = [_2,_4,_4,_4,_19,_19,_19,_4];
_4 = _19;
_1.1 = _5.1;
_13 = _22;
_1 = (_9.0, _9.1, _9.2);
_20 = [865338106_u32,486224222_u32];
_15.1 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_10 = 1555503059_u32 as f64;
_8 = [_2,_4,_19,_4,_19,_4,_19,_4];
_3 = !_4;
_1.2 = [_2,_4,_2,_3,_19,_19,_2,_3];
Call(_8 = fn12(_9, _3, _5.2, _5, _4, _7, _9, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16 = _21 | _21;
_1.0 = 5126792246134941924_i64 as f32;
_22 = [_15.0,_15.0];
_6 = _3 ^ _19;
_21 = _16 - _16;
_25 = !_11;
_7 = _1.2;
_7 = _8;
_24 = _21 - _16;
RET = 209369453829674881903672630265595030962_u128 as isize;
_26 = 5227506571808182360_u64 as i64;
_27 = 43884883253781955764250944450372758180_i128 as u128;
_29 = _13;
_30 = _19 ^ _6;
_9 = _5;
_5.0 = -_9.0;
_13 = [_15.0,_15.0];
_5.1 = _9.1;
_26 = 47335_u16 as i64;
Goto(bb8)
}
bb8 = {
_9.1 = _5.1;
_20 = [4021953276_u32,1121399445_u32];
_27 = 1319452474_i32 as u128;
_19 = _4;
_28 = &_24;
_1.1 = [(*_28),_21,_24,_16,(*_28),_21,_21,_16];
_1.1 = [_24,(*_28),_16,_21,_24,(*_28),_24,_24];
Goto(bb9)
}
bb9 = {
_33 = [(-49449775242238233273220910288684154976_i128),161223805678218488541197250946586306175_i128];
_13 = [_15.0,_15.0];
_37 = _5.0 as i64;
_13 = [_15.0,_15.0];
_5.2 = _7;
_33 = [115396060033898923320440596646808166430_i128,(-25798727307200775008935783202911938486_i128)];
_15.1 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_34 = [(*_28),(*_28)];
_35 = _11 as u16;
_8 = _9.2;
_5.0 = _9.0;
_38 = (*_28);
_12 = 2650_i16;
_9.0 = _5.0 - _5.0;
_32 = _30;
_39.0 = !_15.0;
Goto(bb10)
}
bb10 = {
_19 = _30;
_30 = _4 > _4;
_16 = !(*_28);
_13 = [_39.0,_15.0];
_40 = [_25,_25,_11,_25,_25,_11,_11];
_38 = -(*_28);
_42 = _3 != _19;
_41 = _10 * _10;
_1 = (_5.0, _9.1, _5.2);
_33 = [(-121206693552615259760346424673906469871_i128),44274491720272546398296125177660047652_i128];
_35 = 63085_u16 | 15453_u16;
Call(_34 = fn13(Move(_28), _9.2, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Goto(bb12)
}
bb12 = {
_39.1 = _15.1;
_43.0.fld3 = (-597379960_i32) as i8;
_26 = _12 as i64;
_43.0.fld0 = [_12,_12,_12,_12,_12,_12];
_30 = !_4;
_46.1 = [_43.0.fld3,_15.0,_39.0,_39.0,_43.0.fld3,_15.0];
_2 = _19;
_17 = [_37,_37,_37,_26,_37,_26];
match _12 {
0 => bb1,
1 => bb5,
2 => bb4,
3 => bb13,
4 => bb14,
5 => bb15,
2650 => bb17,
_ => bb16
}
}
bb13 = {
Goto(bb12)
}
bb14 = {
_19 = _30;
_30 = _4 > _4;
_16 = !(*_28);
_13 = [_39.0,_15.0];
_40 = [_25,_25,_11,_25,_25,_11,_11];
_38 = -(*_28);
_42 = _3 != _19;
_41 = _10 * _10;
_1 = (_5.0, _9.1, _5.2);
_33 = [(-121206693552615259760346424673906469871_i128),44274491720272546398296125177660047652_i128];
_35 = 63085_u16 | 15453_u16;
Call(_34 = fn13(Move(_28), _9.2, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_5.1 = _1.1;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
_2 = !_6;
_3 = _2 < _2;
_8 = [_3,_6,_3,_6,_6,_3,_2,_2];
_9.2 = [_3,_6,_2,_2,_6,_3,_4,_2];
_5.0 = _1.0 + _9.0;
_10 = 34740_u16 as f64;
_1 = (_5.0, _5.1, _5.2);
_9.0 = -_5.0;
_1.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5.2 = [_6,_2,_6,_4,_3,_4,_4,_4];
_13 = [90_i8,5_i8];
Goto(bb2)
}
bb16 = {
_1.0 = _9.0 * _5.0;
_17 = [2598177312163440378_i64,1655721494288494936_i64,(-3933324101457703970_i64),5224005101716813741_i64,(-2352307017556071330_i64),(-245085117500401608_i64)];
_21 = _15.0 as isize;
_9 = (_1.0, _1.1, _5.2);
_5 = (_1.0, _9.1, _8);
_13 = [_15.0,_15.0];
_6 = _5.0 == _1.0;
_1.2 = [_2,_4,_4,_4,_19,_19,_19,_4];
_4 = _19;
_1.1 = _5.1;
_13 = _22;
_1 = (_9.0, _9.1, _9.2);
_20 = [865338106_u32,486224222_u32];
_15.1 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_10 = 1555503059_u32 as f64;
_8 = [_2,_4,_19,_4,_19,_4,_19,_4];
_3 = !_4;
_1.2 = [_2,_4,_2,_3,_19,_19,_2,_3];
Call(_8 = fn12(_9, _3, _5.2, _5, _4, _7, _9, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
_25 = _30 as u8;
_25 = _11 << _24;
_4 = _42;
_43.0.fld2.1 = _11 & _11;
_40 = [_25,_11,_25,_25,_25,_25,_25];
Goto(bb18)
}
bb18 = {
Call(_50 = dump_var(9_usize, 7_usize, Move(_7), 26_usize, Move(_26), 13_usize, Move(_13), 30_usize, Move(_30)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(9_usize, 27_usize, Move(_27), 38_usize, Move(_38), 42_usize, Move(_42), 29_usize, Move(_29)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_50 = dump_var(9_usize, 4_usize, Move(_4), 22_usize, Move(_22), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_50 = dump_var(9_usize, 3_usize, Move(_3), 11_usize, Move(_11), 51_usize, _51, 51_usize, _51), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (f32, [isize; 8], [bool; 8]),mut _2: [bool; 8],mut _3: [bool; 8],mut _4: [bool; 8],mut _5: [bool; 8],mut _6: [bool; 8]) -> [bool; 8] {
mir! {
type RET = [bool; 8];
let _7: [u32; 2];
let _8: ();
let _9: ();
{
_1.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),118_isize,(-123_isize),(-62_isize),9223372036854775807_isize];
RET = _2;
_7 = [2227783858_u32,1746940036_u32];
_3 = [true,false,true,true,false,false,false,true];
_1.0 = (-1328067856_i32) as f32;
_1.0 = 12628263048578641789_usize as f32;
_5 = _1.2;
_7 = [3677273027_u32,785082399_u32];
_1.0 = 4_usize as f32;
_2 = _5;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(10_usize, 5_usize, Move(_5), 6_usize, Move(_6), 3_usize, Move(_3), 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [bool; 8],mut _2: (f32, [isize; 8], [bool; 8]),mut _3: bool,mut _4: bool,mut _5: (f32, [isize; 8], [bool; 8])) -> [bool; 8] {
mir! {
type RET = [bool; 8];
let _6: i32;
let _7: i128;
let _8: Adt38;
let _9: (i128, *const Adt17, (Adt38,));
let _10: ();
let _11: ();
{
_4 = _3 & _3;
_4 = _3;
_2.0 = 1_usize as f32;
RET = [_3,_3,_4,_3,_4,_4,_3,_4];
_5 = _2;
_2 = _5;
_2 = (_5.0, _5.1, _1);
RET = [_4,_4,_4,_3,_4,_3,_3,_3];
_6 = 1682871892_i32 * (-1932743474_i32);
_8.fld0 = [(-16603_i16),22190_i16,(-3197_i16),(-11940_i16),(-15238_i16),(-11683_i16)];
_9.2.0.fld1.0 = -110_i8;
_9.2.0.fld1.1 = [_9.2.0.fld1.0,_9.2.0.fld1.0,_9.2.0.fld1.0,_9.2.0.fld1.0,_9.2.0.fld1.0,_9.2.0.fld1.0];
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(11_usize, 6_usize, Move(_6), 4_usize, Move(_4), 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (f32, [isize; 8], [bool; 8]),mut _2: bool,mut _3: [bool; 8],mut _4: (f32, [isize; 8], [bool; 8]),mut _5: bool,mut _6: [bool; 8],mut _7: (f32, [isize; 8], [bool; 8]),mut _8: bool) -> [bool; 8] {
mir! {
type RET = [bool; 8];
let _9: i32;
let _10: *mut i32;
let _11: ();
let _12: ();
{
_7.0 = -_4.0;
_1.2 = _7.2;
_1.2 = [_2,_5,_5,_5,_8,_2,_2,_8];
RET = [_2,_5,_8,_2,_8,_2,_5,_2];
_1.0 = -_7.0;
_1 = _4;
_5 = _2 < _8;
_5 = _2;
RET = [_8,_2,_2,_5,_2,_8,_8,_2];
_4 = _7;
_4.1 = [(-84_isize),9223372036854775807_isize,9223372036854775807_isize,(-16_isize),(-102_isize),9223372036854775807_isize,46_isize,(-9223372036854775808_isize)];
_3 = _4.2;
_8 = _5 < _5;
_8 = !_2;
_1 = (_7.0, _4.1, _4.2);
_9 = _1.0 as i32;
RET = [_5,_5,_2,_8,_8,_8,_2,_2];
_2 = _5;
_10 = core::ptr::addr_of_mut!(_9);
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(12_usize, 9_usize, Move(_9), 3_usize, Move(_3), 5_usize, Move(_5), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: &'static isize,mut _2: [bool; 8],mut _3: bool) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _4: &'static *mut Adt38;
let _5: char;
let _6: isize;
let _7: isize;
let _8: *mut *const Adt63;
let _9: &'static [i64; 6];
let _10: [u32; 2];
let _11: [u8; 7];
let _12: &'static [i64; 6];
let _13: isize;
let _14: isize;
let _15: f64;
let _16: bool;
let _17: isize;
let _18: *mut *const Adt63;
let _19: (isize, Adt38, bool);
let _20: (Adt38,);
let _21: &'static isize;
let _22: [u8; 7];
let _23: f64;
let _24: usize;
let _25: [i16; 6];
let _26: bool;
let _27: f32;
let _28: i32;
let _29: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _30: Adt26;
let _31: *mut *const Adt63;
let _32: f64;
let _33: (i16, usize, u32);
let _34: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _35: [char; 3];
let _36: *mut i32;
let _37: u32;
let _38: isize;
let _39: (i16, usize, u32);
let _40: i64;
let _41: [i16; 8];
let _42: bool;
let _43: f32;
let _44: *const (u32, *const Adt26, *mut i32, u128);
let _45: (i16, usize, u32);
let _46: i64;
let _47: ();
let _48: ();
{
RET = [(-9223372036854775808_isize),9223372036854775807_isize];
RET = [9223372036854775807_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize];
RET = [9223372036854775807_isize,9223372036854775807_isize];
_3 = !true;
_3 = !false;
_3 = false;
_3 = true;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = false;
_3 = false;
RET = [9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [77_isize,9223372036854775807_isize];
_3 = !false;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET = [9223372036854775807_isize,9223372036854775807_isize];
RET = [(-105_isize),(-9223372036854775808_isize)];
_3 = 9223372036854775807_isize <= (-9223372036854775808_isize);
_3 = !false;
_3 = false;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_3 = true;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_5 = '\u{8b6fa}';
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,9223372036854775807_isize];
Goto(bb1)
}
bb1 = {
_1 = &_6;
_1 = &(*_1);
_5 = '\u{8975f}';
_7 = 2380333702409154034_u64 as isize;
_7 = 19_isize;
_1 = &(*_1);
_3 = true;
RET = [_7,_7];
RET = [_7,_7];
_6 = !_7;
RET = [_6,_6];
_5 = '\u{2cd74}';
RET = [_7,_6];
_1 = &_6;
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_7 = -_6;
_10 = [2951021144_u32,1985586983_u32];
RET = [_7,(*_1)];
_5 = '\u{23983}';
_11 = [42_u8,4_u8,116_u8,254_u8,35_u8,162_u8,107_u8];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
RET = [(*_1),(*_1)];
RET = [_6,(*_1)];
_11 = [204_u8,107_u8,103_u8,186_u8,117_u8,85_u8,148_u8];
Goto(bb2)
}
bb2 = {
RET = [(*_1),(*_1)];
_6 = _7;
_1 = &_6;
RET = [(*_1),_6];
RET = [_6,_6];
_6 = _7 & _7;
_6 = _7;
RET = [_7,_6];
_16 = !_3;
_1 = &_7;
_13 = _6;
_14 = -_6;
_6 = _14 << _14;
_5 = '\u{3520b}';
Call(_8 = fn14(Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = '\u{e7885}';
_14 = !_6;
_6 = !_13;
_3 = _16;
_17 = 1167458229_u32 as isize;
_14 = -_17;
_19.1.fld2.1 = 93_u8 ^ 165_u8;
_10 = [1746243120_u32,2212277179_u32];
_19.1.fld3 = (-124_i8) + 110_i8;
_20.0.fld2.1 = !_19.1.fld2.1;
_20.0.fld3 = _19.1.fld3;
_19.1.fld1.1 = [_19.1.fld3,_19.1.fld3,_20.0.fld3,_19.1.fld3,_20.0.fld3,_19.1.fld3];
Goto(bb4)
}
bb4 = {
_19.1.fld1.0 = -_19.1.fld3;
_1 = &_13;
_20.0.fld0 = [27884_i16,(-32319_i16),(-18435_i16),(-11968_i16),27605_i16,(-15908_i16)];
_20.0.fld1.0 = _20.0.fld3;
_20.0.fld3 = _19.1.fld3 & _19.1.fld3;
_20.0.fld1.1 = [_19.1.fld3,_20.0.fld3,_19.1.fld3,_19.1.fld3,_20.0.fld3,_19.1.fld1.0];
_17 = (*_1);
_16 = _3;
_20.0.fld1.0 = 1957996027_u32 as i8;
_16 = _3;
_13 = -_14;
_19.1.fld1.0 = _20.0.fld3 * _20.0.fld1.0;
_20.0.fld1.1 = [_20.0.fld3,_20.0.fld3,_19.1.fld1.0,_20.0.fld1.0,_19.1.fld1.0,_20.0.fld1.0];
Goto(bb5)
}
bb5 = {
_19.2 = _20.0.fld2.1 == _20.0.fld2.1;
_16 = !_19.2;
_20.0.fld1.1 = _19.1.fld1.1;
_10 = [3631043368_u32,4291118786_u32];
_2 = [_19.2,_16,_16,_19.2,_19.2,_19.2,_16,_19.2];
_19.2 = _19.1.fld2.1 == _20.0.fld2.1;
RET = [_6,_7];
_13 = _17 - _7;
_19.0 = _6;
_11 = [_20.0.fld2.1,_19.1.fld2.1,_19.1.fld2.1,_20.0.fld2.1,_20.0.fld2.1,_19.1.fld2.1,_20.0.fld2.1];
_1 = &_14;
_3 = !_19.2;
Goto(bb6)
}
bb6 = {
_23 = 2_usize as f64;
_20.0.fld1.0 = _20.0.fld3;
_19.1.fld0 = _20.0.fld0;
_16 = _19.2;
_22 = [_19.1.fld2.1,_20.0.fld2.1,_20.0.fld2.1,_20.0.fld2.1,_19.1.fld2.1,_20.0.fld2.1,_19.1.fld2.1];
_21 = &_6;
_2 = [_3,_19.2,_3,_3,_19.2,_16,_16,_3];
Goto(bb7)
}
bb7 = {
_19.2 = _3;
_10 = [2209489020_u32,1873374806_u32];
Goto(bb8)
}
bb8 = {
_5 = '\u{96921}';
_19.0 = 397704500348569238_usize as isize;
_20.0.fld1.0 = -_20.0.fld3;
_20.0.fld1.1 = [_20.0.fld1.0,_19.1.fld1.0,_19.1.fld1.0,_20.0.fld3,_19.1.fld3,_19.1.fld1.0];
_20.0.fld1.1 = _19.1.fld1.1;
_24 = (-22575_i16) as usize;
_25 = [(-21851_i16),16348_i16,21194_i16,12054_i16,(-21414_i16),(-9338_i16)];
_20.0.fld0 = [(-10128_i16),9110_i16,1946_i16,25040_i16,(-5629_i16),(-22810_i16)];
_20.0.fld0 = [(-17468_i16),(-2474_i16),18390_i16,23487_i16,7296_i16,23352_i16];
_19.0 = _13;
_19.1.fld1.0 = _20.0.fld3;
_22 = [_19.1.fld2.1,_19.1.fld2.1,_20.0.fld2.1,_20.0.fld2.1,_19.1.fld2.1,_19.1.fld2.1,_19.1.fld2.1];
_20.0.fld1.1 = _19.1.fld1.1;
_19.1.fld1.0 = -_20.0.fld3;
_19.1.fld0 = _20.0.fld0;
RET = [_7,(*_21)];
_19.1.fld1.1 = _20.0.fld1.1;
Goto(bb9)
}
bb9 = {
_25 = [21166_i16,(-13925_i16),13831_i16,19759_i16,(-23845_i16),(-14844_i16)];
_5 = '\u{3fc53}';
_15 = _23 + _23;
_14 = _13 << _20.0.fld1.0;
_23 = _15;
_21 = &_13;
_1 = &_6;
_21 = Move(_1);
_6 = 7228315380567103533_i64 as isize;
_2 = [_3,_16,_19.2,_16,_16,_3,_3,_19.2];
Goto(bb10)
}
bb10 = {
_19.2 = !_16;
_3 = !_16;
_19.1.fld4 = core::ptr::addr_of!(_30);
_29.0 = 31406_i16;
_13 = _17;
_17 = _23 as isize;
_34.2.1 = _20.0.fld1.1;
_20.0.fld3 = !_19.1.fld3;
Goto(bb11)
}
bb11 = {
_36 = core::ptr::addr_of_mut!(_28);
_20.0.fld0 = [_29.0,_29.0,_29.0,_29.0,_29.0,_29.0];
_19.0 = _14 | _17;
match _29.0 {
0 => bb6,
1 => bb4,
2 => bb9,
31406 => bb13,
_ => bb12
}
}
bb12 = {
_19.2 = _3;
_10 = [2209489020_u32,1873374806_u32];
Goto(bb8)
}
bb13 = {
_37 = 641117093_u32;
_19.1.fld1.1 = [_19.1.fld3,_20.0.fld3,_20.0.fld1.0,_19.1.fld1.0,_19.1.fld3,_20.0.fld1.0];
_27 = _37 as f32;
_33.0 = _29.0;
_1 = &_14;
_33 = (_29.0, _24, _37);
_34.1 = _23 + _23;
_11 = [_20.0.fld2.1,_20.0.fld2.1,_19.1.fld2.1,_19.1.fld2.1,_19.1.fld2.1,_20.0.fld2.1,_20.0.fld2.1];
_34.3 = !52181_u16;
_35 = [_5,_5,_5];
_37 = _33.2;
match _37 {
0 => bb11,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb8,
5 => bb12,
641117093 => bb15,
_ => bb14
}
}
bb14 = {
_19.1.fld1.0 = -_19.1.fld3;
_1 = &_13;
_20.0.fld0 = [27884_i16,(-32319_i16),(-18435_i16),(-11968_i16),27605_i16,(-15908_i16)];
_20.0.fld1.0 = _20.0.fld3;
_20.0.fld3 = _19.1.fld3 & _19.1.fld3;
_20.0.fld1.1 = [_19.1.fld3,_20.0.fld3,_19.1.fld3,_19.1.fld3,_20.0.fld3,_19.1.fld1.0];
_17 = (*_1);
_16 = _3;
_20.0.fld1.0 = 1957996027_u32 as i8;
_16 = _3;
_13 = -_14;
_19.1.fld1.0 = _20.0.fld3 * _20.0.fld1.0;
_20.0.fld1.1 = [_20.0.fld3,_20.0.fld3,_19.1.fld1.0,_20.0.fld1.0,_19.1.fld1.0,_20.0.fld1.0];
Goto(bb5)
}
bb15 = {
_34.0 = -_29.0;
_34.0 = _29.0;
_19.1.fld0 = [_29.0,_29.0,_34.0,_29.0,_33.0,_33.0];
_33.1 = _24;
_38 = -(*_1);
_28 = (-1696246805965068874_i64) as i32;
_33.1 = _24;
_5 = '\u{69eb1}';
_37 = _33.2 >> _19.0;
_20.0.fld2.1 = !_19.1.fld2.1;
_34.2.1 = [_20.0.fld3,_19.1.fld1.0,_19.1.fld1.0,_19.1.fld1.0,_19.1.fld1.0,_20.0.fld1.0];
_29.3 = !_34.3;
_25 = [_34.0,_33.0,_33.0,_29.0,_33.0,_29.0];
_45.0 = !_33.0;
_1 = &_13;
_29.2.0 = _19.1.fld1.0 << _37;
RET = [_7,_19.0];
_28 = 1756222003_i32 | 1436386275_i32;
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(13_usize, 37_usize, Move(_37), 13_usize, Move(_13), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(13_usize, 10_usize, Move(_10), 33_usize, Move(_33), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(13_usize, 22_usize, Move(_22), 48_usize, _48, 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static isize) -> *mut *const Adt63 {
mir! {
type RET = *mut *const Adt63;
let _2: [usize; 5];
let _3: f64;
let _4: [usize; 5];
let _5: i128;
let _6: Adt63;
let _7: [u32; 8];
let _8: f32;
let _9: *const [u16; 4];
let _10: [i16; 8];
let _11: &'static isize;
let _12: usize;
let _13: *mut Adt38;
let _14: isize;
let _15: char;
let _16: i8;
let _17: &'static [i64; 3];
let _18: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _19: bool;
let _20: isize;
let _21: u32;
let _22: (i16, usize, u32);
let _23: &'static isize;
let _24: usize;
let _25: *mut Adt38;
let _26: u8;
let _27: [usize; 5];
let _28: [bool; 8];
let _29: char;
let _30: u64;
let _31: char;
let _32: Adt26;
let _33: f32;
let _34: i16;
let _35: bool;
let _36: isize;
let _37: [char; 3];
let _38: isize;
let _39: *mut *const Adt63;
let _40: f32;
let _41: &'static [i128; 6];
let _42: isize;
let _43: [i32; 6];
let _44: u128;
let _45: f64;
let _46: f64;
let _47: usize;
let _48: bool;
let _49: i128;
let _50: u8;
let _51: i64;
let _52: f64;
let _53: (Adt31, u8);
let _54: f32;
let _55: *const f32;
let _56: i16;
let _57: [u32; 2];
let _58: *const [u16; 4];
let _59: [i64; 3];
let _60: isize;
let _61: *const Adt63;
let _62: char;
let _63: [i128; 2];
let _64: bool;
let _65: isize;
let _66: Adt22;
let _67: Adt26;
let _68: isize;
let _69: [isize; 2];
let _70: Adt40;
let _71: [u16; 4];
let _72: [u8; 7];
let _73: (i32, Adt17, u64);
let _74: ();
let _75: ();
{
_2 = [1_usize,17262744896946074993_usize,7_usize,5_usize,0_usize];
_2 = [433564898184963857_usize,6_usize,5_usize,1_usize,2_usize];
_2 = [2_usize,4_usize,1_usize,1_usize,14066174094695838977_usize];
_2 = [703278945426242923_usize,127687495617024395_usize,3046792396509275291_usize,6_usize,3_usize];
_2 = [12716973285575079339_usize,1_usize,3_usize,0_usize,1814164153358444592_usize];
_2 = [0_usize,15440776874097345513_usize,6468129864297734382_usize,1_usize,4_usize];
_2 = [17672934800908176424_usize,2395516581942058068_usize,7_usize,8801267550995650181_usize,1_usize];
_2 = [6_usize,7_usize,6062848342383674836_usize,4485296383075297184_usize,679500212641313442_usize];
_2 = [1907923168332378039_usize,7_usize,2_usize,6_usize,9325835704712638907_usize];
_3 = 320259436123446184332060791866207578056_u128 as f64;
_3 = 60029_u16 as f64;
_3 = 7287479524369268875_i64 as f64;
Goto(bb1)
}
bb1 = {
_3 = 4116185901_u32 as f64;
_3 = (-122_i8) as f64;
_3 = 35960_u16 as f64;
_3 = 334023610671787443640067637060181770390_u128 as f64;
_3 = 54_i8 as f64;
_4 = _2;
_2 = [6_usize,5_usize,3_usize,18073442463689354770_usize,7596065382811179044_usize];
_2 = [11590138475277750383_usize,5_usize,1_usize,0_usize,366482103377625929_usize];
_2 = [5811091278529243015_usize,17175666948876929059_usize,6_usize,5_usize,10414717229658797378_usize];
_4 = [7_usize,17187358173165272764_usize,18292477699606404662_usize,15691827006901718267_usize,7_usize];
_3 = 160550592936354133153552753339015028418_i128 as f64;
_3 = 155_u8 as f64;
_2 = [3_usize,2_usize,6_usize,16461535491502600858_usize,10691542715259296965_usize];
_4 = [0_usize,0_usize,7_usize,1_usize,5896811029721847304_usize];
_4 = [16135248129127505981_usize,4_usize,12901290298739421571_usize,0_usize,0_usize];
_4 = _2;
_2 = [3_usize,17516282110803937281_usize,6_usize,392423364033488682_usize,3893990473328160581_usize];
_4 = _2;
_4 = [0_usize,14216988049155542215_usize,5_usize,3842197648381553518_usize,7_usize];
_4 = [0_usize,0_usize,3_usize,10311255083661833815_usize,18003287737009836307_usize];
_2 = [12119544419634909043_usize,16012305621858365368_usize,445939004619065272_usize,2151491006095427635_usize,2_usize];
_3 = 2913709726027075176_i64 as f64;
_5 = (-158142187751476958551410215465370273089_i128) ^ (-92341760100598006411468137937025953721_i128);
Goto(bb2)
}
bb2 = {
_2 = [9499399490378478169_usize,3_usize,2_usize,0_usize,18295246892362276080_usize];
_5 = !41680898325582798541402327066947653910_i128;
_4 = [6_usize,5_usize,6_usize,7_usize,1_usize];
_7 = [1878568602_u32,1942206446_u32,2138855147_u32,482892587_u32,772587733_u32,4260966667_u32,2521996823_u32,3948829887_u32];
_5 = 228_u8 as i128;
_7 = [2281349825_u32,1860033409_u32,3007231696_u32,1052752440_u32,1982364864_u32,3005039584_u32,2455306612_u32,935750563_u32];
_2 = _4;
_2 = [11855670898696395180_usize,17488600342384356927_usize,15986135327681522112_usize,5_usize,2_usize];
_7 = [2099887486_u32,4256788724_u32,609697431_u32,4072884105_u32,477104442_u32,1099817382_u32,2322464339_u32,2292880373_u32];
_7 = [3671976889_u32,1324631527_u32,455545106_u32,688812937_u32,4015366261_u32,2977467854_u32,3681623075_u32,2029589957_u32];
_8 = 18112674882733023681_u64 as f32;
_2 = _4;
_8 = (-73_i8) as f32;
_4 = [0_usize,3_usize,3903209224168923598_usize,4485718881752366850_usize,0_usize];
Goto(bb3)
}
bb3 = {
_4 = [4_usize,8271578210787672678_usize,6_usize,1_usize,2784926356285119314_usize];
_7 = [721550199_u32,4065699760_u32,210981272_u32,293508880_u32,2840198880_u32,576409499_u32,224299199_u32,2810668076_u32];
_7 = [1637211194_u32,1580828099_u32,4029581714_u32,3408752437_u32,1779588117_u32,740132194_u32,1883020299_u32,4242270350_u32];
_10 = [(-3091_i16),15154_i16,1516_i16,(-29639_i16),7131_i16,27813_i16,(-11620_i16),(-25755_i16)];
_10 = [31214_i16,17292_i16,8434_i16,21050_i16,21387_i16,1202_i16,8946_i16,9881_i16];
_2 = [1651701080089764700_usize,1496195029422592112_usize,18415232766907271070_usize,1_usize,2654645956058978001_usize];
_2 = _4;
Goto(bb4)
}
bb4 = {
Goto(bb5)
}
bb5 = {
_5 = (-5264753314306990736_i64) as i128;
_7 = [3871578684_u32,2648261836_u32,3509376845_u32,1139648425_u32,511480857_u32,3074666689_u32,3533050929_u32,3090166829_u32];
_5 = (-9223372036854775808_isize) as i128;
_8 = (-591745479640365196_i64) as f32;
_10 = [31555_i16,2449_i16,24807_i16,(-7574_i16),(-148_i16),(-25222_i16),(-5519_i16),(-8636_i16)];
_10 = [628_i16,(-23612_i16),22208_i16,24724_i16,24254_i16,(-30090_i16),10340_i16,19667_i16];
_2 = _4;
_4 = [4_usize,5_usize,7_usize,7_usize,4_usize];
_3 = 8778933_i32 as f64;
_5 = (-144349674236076413976757583828079538818_i128);
_3 = (-952860053559340782_i64) as f64;
_7 = [935119343_u32,1788179101_u32,697220604_u32,3752862426_u32,2205725481_u32,4035986317_u32,370875776_u32,2694874450_u32];
_4 = [12898679375377993265_usize,10778494517197245693_usize,4_usize,5510844309238629714_usize,17751502992213138930_usize];
Call(_2 = fn15(_7, _10, _4, _4, _7, _4, _7, _7, _10, _10, _4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = [1983748091_u32,273281419_u32,3605787897_u32,1893058799_u32,4208324053_u32,16146900_u32,2923918210_u32,185353996_u32];
_2 = [5_usize,0_usize,214610831873175799_usize,6_usize,3_usize];
_4 = [244217543078997424_usize,9146237023345780353_usize,5_usize,3_usize,4_usize];
_8 = 213852472824766507217803807542118180774_u128 as f32;
_8 = 0_usize as f32;
_10 = [2550_i16,(-26588_i16),(-30714_i16),20923_i16,25336_i16,(-23594_i16),11223_i16,(-10825_i16)];
_2 = [14409583396933394915_usize,6_usize,17801067926518047756_usize,4_usize,4_usize];
_3 = 14030696394368462257_u64 as f64;
_1 = &_14;
_8 = (-1248960360_i32) as f32;
_1 = &_14;
_1 = &(*_1);
_3 = 10715016369963809401_u64 as f64;
_1 = &(*_1);
_10 = [1800_i16,(-13968_i16),(-6089_i16),(-14755_i16),(-17539_i16),30018_i16,(-20739_i16),12783_i16];
_12 = !5_usize;
Goto(bb7)
}
bb7 = {
_11 = &(*_1);
_1 = &_14;
_2 = [_12,_12,_12,_12,_12];
_2 = [_12,_12,_12,_12,_12];
_1 = &_14;
_8 = 9223372036854775807_isize as f32;
_16 = -(-41_i8);
_15 = '\u{9d3ef}';
_10 = [2984_i16,10669_i16,(-14404_i16),(-2654_i16),30643_i16,(-23387_i16),8210_i16,13210_i16];
_1 = &(*_11);
_18.1 = _3;
_18.0 = 21803_i16;
_14 = 297474400066787133403072785964811407997_u128 as isize;
_11 = &_14;
_18.0 = (-26798_i16);
_4 = _2;
_10 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
Goto(bb8)
}
bb8 = {
_18.2.1 = [_16,_16,_16,_16,_16,_16];
_7 = [2352217203_u32,4262301059_u32,1810436896_u32,3580342883_u32,3804270832_u32,3876806078_u32,3964637787_u32,862157164_u32];
_18.2.1 = [_16,_16,_16,_16,_16,_16];
_16 = (-103_i8) * (-14_i8);
_18.0 = 48069_u16 as i16;
_14 = false as isize;
_21 = 1925404152_u32 & 1111832289_u32;
_21 = 3854040544_u32;
_18.2.0 = _16;
_2 = _4;
_2 = [_12,_12,_12,_12,_12];
_20 = _14;
Goto(bb9)
}
bb9 = {
_1 = &_14;
_2 = [_12,_12,_12,_12,_12];
_3 = _18.1;
_4 = [_12,_12,_12,_12,_12];
_22.1 = _12;
_22.2 = _21 | _21;
_20 = !(*_1);
_18.3 = 40940_u16 | 38757_u16;
_4 = [_22.1,_22.1,_22.1,_22.1,_12];
_2 = [_22.1,_12,_22.1,_12,_12];
_18.2.0 = _15 as i8;
_10 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
Goto(bb10)
}
bb10 = {
_19 = true;
_14 = _20 + _20;
_26 = !78_u8;
_23 = &_20;
_22.0 = !_18.0;
_18.2.1 = [_16,_16,_16,_16,_16,_16];
_22.2 = _21;
_20 = _8 as isize;
_28 = [_19,_19,_19,_19,_19,_19,_19,_19];
_8 = _26 as f32;
_14 = 14761994439538784123_u64 as isize;
_16 = !_18.2.0;
Goto(bb11)
}
bb11 = {
_22.2 = _21 * _21;
_1 = &_20;
_18.1 = _3 - _3;
_3 = _8 as f64;
_14 = (*_1) * (*_1);
_23 = &(*_1);
_7 = [_22.2,_22.2,_22.2,_22.2,_22.2,_21,_22.2,_22.2];
_2 = _4;
_10 = [_18.0,_18.0,_18.0,_18.0,_22.0,_22.0,_22.0,_18.0];
_4 = [_22.1,_22.1,_22.1,_22.1,_12];
_2 = _4;
_12 = _22.1;
_3 = -_18.1;
match _5 {
0 => bb9,
1 => bb12,
195932692684862049486617023603688672638 => bb14,
_ => bb13
}
}
bb12 = {
Goto(bb5)
}
bb13 = {
_3 = 4116185901_u32 as f64;
_3 = (-122_i8) as f64;
_3 = 35960_u16 as f64;
_3 = 334023610671787443640067637060181770390_u128 as f64;
_3 = 54_i8 as f64;
_4 = _2;
_2 = [6_usize,5_usize,3_usize,18073442463689354770_usize,7596065382811179044_usize];
_2 = [11590138475277750383_usize,5_usize,1_usize,0_usize,366482103377625929_usize];
_2 = [5811091278529243015_usize,17175666948876929059_usize,6_usize,5_usize,10414717229658797378_usize];
_4 = [7_usize,17187358173165272764_usize,18292477699606404662_usize,15691827006901718267_usize,7_usize];
_3 = 160550592936354133153552753339015028418_i128 as f64;
_3 = 155_u8 as f64;
_2 = [3_usize,2_usize,6_usize,16461535491502600858_usize,10691542715259296965_usize];
_4 = [0_usize,0_usize,7_usize,1_usize,5896811029721847304_usize];
_4 = [16135248129127505981_usize,4_usize,12901290298739421571_usize,0_usize,0_usize];
_4 = _2;
_2 = [3_usize,17516282110803937281_usize,6_usize,392423364033488682_usize,3893990473328160581_usize];
_4 = _2;
_4 = [0_usize,14216988049155542215_usize,5_usize,3842197648381553518_usize,7_usize];
_4 = [0_usize,0_usize,3_usize,10311255083661833815_usize,18003287737009836307_usize];
_2 = [12119544419634909043_usize,16012305621858365368_usize,445939004619065272_usize,2151491006095427635_usize,2_usize];
_3 = 2913709726027075176_i64 as f64;
_5 = (-158142187751476958551410215465370273089_i128) ^ (-92341760100598006411468137937025953721_i128);
Goto(bb2)
}
bb14 = {
_22.0 = _18.0 + _18.0;
_24 = _12 - _22.1;
_22.1 = _24;
_7 = [_22.2,_22.2,_22.2,_22.2,_21,_21,_21,_22.2];
_11 = &(*_1);
_30 = _26 as u64;
_7 = [_21,_21,_21,_21,_22.2,_22.2,_22.2,_22.2];
_27 = _2;
_15 = '\u{40d18}';
Goto(bb15)
}
bb15 = {
_4 = [_24,_12,_22.1,_24,_22.1];
_21 = _22.2 >> _12;
_19 = (*_11) <= _14;
_8 = _5 as f32;
_1 = &(*_11);
_11 = Move(_23);
_8 = _22.1 as f32;
_7 = [_22.2,_21,_21,_21,_21,_21,_22.2,_21];
_22.2 = _15 as u32;
_8 = _21 as f32;
_2 = _27;
_4 = [_22.1,_12,_24,_22.1,_22.1];
_18.2.1 = [_16,_18.2.0,_18.2.0,_18.2.0,_16,_18.2.0];
_3 = _18.1 - _18.1;
_29 = _15;
_18.2.0 = _16 + _16;
match _5 {
0 => bb10,
1 => bb2,
2 => bb11,
3 => bb8,
4 => bb12,
5 => bb6,
195932692684862049486617023603688672638 => bb17,
_ => bb16
}
}
bb16 = {
_11 = &(*_1);
_1 = &_14;
_2 = [_12,_12,_12,_12,_12];
_2 = [_12,_12,_12,_12,_12];
_1 = &_14;
_8 = 9223372036854775807_isize as f32;
_16 = -(-41_i8);
_15 = '\u{9d3ef}';
_10 = [2984_i16,10669_i16,(-14404_i16),(-2654_i16),30643_i16,(-23387_i16),8210_i16,13210_i16];
_1 = &(*_11);
_18.1 = _3;
_18.0 = 21803_i16;
_14 = 297474400066787133403072785964811407997_u128 as isize;
_11 = &_14;
_18.0 = (-26798_i16);
_4 = _2;
_10 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
Goto(bb8)
}
bb17 = {
_31 = _15;
_26 = _5 as u8;
_18.3 = 28821_u16 | 59445_u16;
_21 = _22.2 >> _22.1;
_30 = 3783984546691197964_u64;
_21 = _22.2;
_34 = _30 as i16;
_33 = _22.0 as f32;
_23 = &_14;
_21 = _22.2 & _22.2;
_18.3 = 18193_u16 >> _22.1;
_24 = _12;
_31 = _29;
_18.2.0 = _16;
_22.2 = _21 * _21;
_18.3 = !42726_u16;
_34 = -_22.0;
_29 = _31;
_8 = _33;
_11 = &(*_23);
_37 = [_31,_29,_15];
_18.2.1 = [_18.2.0,_18.2.0,_18.2.0,_18.2.0,_18.2.0,_16];
_33 = _8;
_35 = _19 ^ _19;
match _30 {
3783984546691197964 => bb19,
_ => bb18
}
}
bb18 = {
_2 = [9499399490378478169_usize,3_usize,2_usize,0_usize,18295246892362276080_usize];
_5 = !41680898325582798541402327066947653910_i128;
_4 = [6_usize,5_usize,6_usize,7_usize,1_usize];
_7 = [1878568602_u32,1942206446_u32,2138855147_u32,482892587_u32,772587733_u32,4260966667_u32,2521996823_u32,3948829887_u32];
_5 = 228_u8 as i128;
_7 = [2281349825_u32,1860033409_u32,3007231696_u32,1052752440_u32,1982364864_u32,3005039584_u32,2455306612_u32,935750563_u32];
_2 = _4;
_2 = [11855670898696395180_usize,17488600342384356927_usize,15986135327681522112_usize,5_usize,2_usize];
_7 = [2099887486_u32,4256788724_u32,609697431_u32,4072884105_u32,477104442_u32,1099817382_u32,2322464339_u32,2292880373_u32];
_7 = [3671976889_u32,1324631527_u32,455545106_u32,688812937_u32,4015366261_u32,2977467854_u32,3681623075_u32,2029589957_u32];
_8 = 18112674882733023681_u64 as f32;
_2 = _4;
_8 = (-73_i8) as f32;
_4 = [0_usize,3_usize,3903209224168923598_usize,4485718881752366850_usize,0_usize];
Goto(bb3)
}
bb19 = {
_27 = [_22.1,_12,_22.1,_22.1,_12];
_12 = _24;
_10 = [_34,_22.0,_34,_34,_22.0,_18.0,_22.0,_34];
_29 = _31;
_26 = 194_u8 ^ 247_u8;
_35 = !_19;
_18.0 = _34 >> _34;
_23 = Move(_11);
_29 = _15;
_8 = _33;
_18.0 = _3 as i16;
_10 = [_22.0,_18.0,_34,_34,_22.0,_22.0,_18.0,_34];
_16 = _18.0 as i8;
_36 = (*_1) | _14;
_22.1 = !_12;
_4 = _27;
_10 = [_18.0,_22.0,_18.0,_18.0,_22.0,_18.0,_34,_18.0];
_3 = _18.1;
_22.2 = _21;
_40 = _18.3 as f32;
_20 = -_14;
_1 = &_20;
Goto(bb20)
}
bb20 = {
_15 = _29;
_30 = !11933580065036783601_u64;
_42 = _36 | (*_1);
_35 = _19;
_5 = 54016571727192878817268666553948548677_i128 ^ 60571564479213721403067284166622950514_i128;
_42 = -_36;
_23 = &(*_1);
_4 = [_22.1,_22.1,_24,_24,_22.1];
_8 = -_33;
_15 = _31;
_32 = Adt26::Variant2 { fld0: 301249407315495070918829078850265340911_u128 };
place!(Field::<u128>(Variant(_32, 2), 0)) = _36 as u128;
_14 = -(*_1);
_1 = Move(_23);
_37 = [_31,_15,_15];
_18.2.1 = [_16,_16,_16,_16,_16,_16];
_45 = -_3;
_12 = _22.1;
_24 = !_12;
_21 = _22.2 - _22.2;
_18.2.0 = _16 - _16;
_7 = [_22.2,_21,_22.2,_22.2,_22.2,_21,_21,_22.2];
place!(Field::<u128>(Variant(_32, 2), 0)) = 196748369472901430340689530248873209595_u128;
SetDiscriminant(_32, 1);
Goto(bb21)
}
bb21 = {
_23 = &_42;
place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).2 = !_30;
_4 = _2;
_42 = _20;
_27 = [_22.1,_22.1,_12,_12,_24];
_40 = _26 as f32;
_32 = Adt26::Variant2 { fld0: 221449192497735534638217489666635889707_u128 };
_34 = !_18.0;
Call(_43 = fn18(_14, _24, _37, _42, _3, _29, _3, _16, _36, _34), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_8 = _26 as f32;
_28 = [_19,_19,_19,_35,_19,_19,_19,_19];
_44 = !249815362658466214287368730035531639358_u128;
_22 = (_18.0, _24, _21);
_15 = _29;
_11 = &_14;
_27 = _4;
Goto(bb23)
}
bb23 = {
_49 = _18.3 as i128;
_21 = _22.2;
_24 = _22.1;
_33 = (-34593513_i32) as f32;
_51 = (-7035850655935112217_i64) | (-17077726822676372_i64);
_18.3 = 502743671_i32 as u16;
_32 = Adt26::Variant2 { fld0: _44 };
_22 = (_18.0, _24, _21);
_7 = [_22.2,_21,_22.2,_21,_21,_22.2,_22.2,_22.2];
_27 = [_24,_12,_22.1,_24,_12];
_18.0 = _34 & _22.0;
Goto(bb24)
}
bb24 = {
_31 = _29;
_30 = 16056075697422768429_u64 * 15603794058207822383_u64;
Goto(bb25)
}
bb25 = {
_50 = _26;
_27 = [_24,_22.1,_24,_24,_22.1];
SetDiscriminant(_32, 1);
place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1.fld5 = _35 as i32;
_1 = &(*_11);
_18.2.2 = core::ptr::addr_of!(place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1);
_40 = _24 as f32;
_37 = [_29,_29,_31];
_26 = !_50;
_37 = [_31,_29,_31];
_24 = _22.1;
_21 = _22.2;
_14 = _36 ^ _36;
_7 = [_22.2,_22.2,_22.2,_21,_22.2,_21,_21,_21];
_22 = (_34, _12, _21);
place!(Field::<[u32; 2]>(Variant(_32, 1), 7)) = [_21,_21];
_37 = [_29,_29,_29];
_4 = [_12,_24,_24,_24,_12];
_36 = _14;
_37 = [_29,_15,_31];
_52 = _45;
_19 = _21 >= _21;
Goto(bb26)
}
bb26 = {
_24 = _22.1 ^ _12;
_38 = _19 as isize;
place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1 = Adt17 { fld0: _44,fld1: _15,fld2: _38,fld3: _16,fld4: _18.0,fld5: 372177562_i32 };
_10 = [_18.0,_34,_22.0,_34,Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1).1.fld4,_18.0,_18.0,Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1).1.fld4];
_12 = _22.1 >> _18.0;
place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1.fld3 = _18.2.0 & _18.2.0;
_45 = _30 as f64;
_56 = Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1).1.fld4 >> _12;
_51 = !(-4279439616213238747_i64);
_46 = -_45;
_23 = &place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1.fld2;
_11 = &(*_23);
Goto(bb27)
}
bb27 = {
_55 = core::ptr::addr_of!(_8);
(*_55) = -_33;
place!(Field::<[u32; 2]>(Variant(_32, 1), 7)) = [_22.2,_22.2];
_1 = Move(_11);
_11 = Move(_23);
_56 = Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1).1.fld4 + _18.0;
_53.1 = !_50;
_45 = -_3;
_51 = 3895271868871089438_i64;
place!(Field::<*const Adt17>(Variant(_32, 1), 2)) = core::ptr::addr_of!(place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1);
_49 = _5;
place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1.fld2 = _42;
_57 = [_21,_21];
_23 = &place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1.fld2;
_5 = !_49;
_55 = core::ptr::addr_of!(_40);
_61 = core::ptr::addr_of!(_6);
place!(Field::<i16>(Variant(_32, 1), 4)) = _56 * _56;
_18.2.2 = core::ptr::addr_of!(place!(Field::<(i32, Adt17, u64)>(Variant(_32, 1), 1)).1);
_10 = [Field::<i16>(Variant(_32, 1), 4),_56,_56,Field::<i16>(Variant(_32, 1), 4),_22.0,_56,Field::<i16>(Variant(_32, 1), 4),Field::<i16>(Variant(_32, 1), 4)];
_59 = [_51,_51,_51];
_36 = _38 + _14;
place!(Field::<u8>(Variant(_32, 1), 5)) = _22.1 as u8;
_7 = [_21,_21,_22.2,_21,_21,_21,_22.2,_21];
_36 = _38 >> _18.0;
_32 = Adt26::Variant2 { fld0: _44 };
match _51 {
0 => bb11,
1 => bb7,
2 => bb3,
3 => bb24,
4 => bb16,
5 => bb19,
3895271868871089438 => bb29,
_ => bb28
}
}
bb28 = {
_2 = [9499399490378478169_usize,3_usize,2_usize,0_usize,18295246892362276080_usize];
_5 = !41680898325582798541402327066947653910_i128;
_4 = [6_usize,5_usize,6_usize,7_usize,1_usize];
_7 = [1878568602_u32,1942206446_u32,2138855147_u32,482892587_u32,772587733_u32,4260966667_u32,2521996823_u32,3948829887_u32];
_5 = 228_u8 as i128;
_7 = [2281349825_u32,1860033409_u32,3007231696_u32,1052752440_u32,1982364864_u32,3005039584_u32,2455306612_u32,935750563_u32];
_2 = _4;
_2 = [11855670898696395180_usize,17488600342384356927_usize,15986135327681522112_usize,5_usize,2_usize];
_7 = [2099887486_u32,4256788724_u32,609697431_u32,4072884105_u32,477104442_u32,1099817382_u32,2322464339_u32,2292880373_u32];
_7 = [3671976889_u32,1324631527_u32,455545106_u32,688812937_u32,4015366261_u32,2977467854_u32,3681623075_u32,2029589957_u32];
_8 = 18112674882733023681_u64 as f32;
_2 = _4;
_8 = (-73_i8) as f32;
_4 = [0_usize,3_usize,3903209224168923598_usize,4485718881752366850_usize,0_usize];
Goto(bb3)
}
bb29 = {
_62 = _15;
_34 = _51 as i16;
_7 = [_21,_22.2,_21,_22.2,_22.2,_21,_21,_22.2];
match _51 {
0 => bb1,
1 => bb18,
2 => bb27,
3895271868871089438 => bb30,
_ => bb24
}
}
bb30 = {
_63 = [_49,_5];
_38 = _31 as isize;
_65 = -_36;
(*_55) = _8;
_53.0 = Adt31::Variant0 { fld0: _22,fld1: Move(_32) };
_19 = _35;
_1 = &_42;
_47 = _40 as usize;
_18.3 = 63974_u16;
_59 = [_51,_51,_51];
_54 = -_33;
place!(Field::<u128>(Variant(place!(Field::<Adt26>(Variant(_53.0, 0), 1)), 2), 0)) = _44 | _44;
_30 = !8396590385563298074_u64;
_23 = &_38;
_19 = _45 == _3;
_10 = [_18.0,_56,_56,Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0).0,Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0).0,_18.0,_22.0,_18.0];
_3 = _46 + _52;
_18.0 = Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0).0;
_23 = Move(_1);
place!(Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0)).1 = _12;
_30 = 5686285998083353914_u64;
_7 = [_22.2,Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0).2,_22.2,_22.2,_21,_22.2,Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0).2,Field::<(i16, usize, u32)>(Variant(_53.0, 0), 0).2];
_19 = _35;
_5 = _49 + _49;
_18.2.0 = Field::<u128>(Variant(Field::<Adt26>(Variant(_53.0, 0), 1), 2), 0) as i8;
match _30 {
5686285998083353914 => bb31,
_ => bb3
}
}
bb31 = {
SetDiscriminant(_53.0, 0);
_48 = _35;
_29 = _31;
_7 = [_22.2,_22.2,_21,_22.2,_22.2,_21,_22.2,_21];
_22.2 = _21 << _65;
_28 = [_48,_35,_35,_19,_19,_35,_35,_48];
_32 = Adt26::Variant2 { fld0: _44 };
_15 = _31;
_47 = _12;
_18.2.3 = Adt22::Variant3 { fld0: _26,fld1: _63,fld2: _22,fld3: _44,fld4: _30,fld5: 1389991635_i32,fld6: _51,fld7: _49 };
place!(Field::<u64>(Variant(_18.2.3, 3), 4)) = _49 as u64;
_18.3 = _3 as u16;
_64 = !_35;
_22.1 = _24 << _22.2;
_22.2 = !Field::<(i16, usize, u32)>(Variant(_18.2.3, 3), 2).2;
_38 = _65;
_28 = [_19,_48,_35,_64,_48,_19,_64,_48];
place!(Field::<(i16, usize, u32)>(Variant(_18.2.3, 3), 2)).2 = _22.2;
_52 = _18.0 as f64;
Goto(bb32)
}
bb32 = {
_51 = !Field::<i64>(Variant(_18.2.3, 3), 6);
place!(Field::<(i16, usize, u32)>(Variant(_18.2.3, 3), 2)).0 = _52 as i16;
RET = core::ptr::addr_of_mut!(_61);
_61 = core::ptr::addr_of!(_6);
_39 = core::ptr::addr_of_mut!(_61);
_9 = core::ptr::addr_of!(_71);
_66 = Adt22::Variant3 { fld0: _50,fld1: Field::<[i128; 2]>(Variant(_18.2.3, 3), 1),fld2: Field::<(i16, usize, u32)>(Variant(_18.2.3, 3), 2),fld3: Field::<u128>(Variant(_18.2.3, 3), 3),fld4: _30,fld5: 591838951_i32,fld6: _51,fld7: _49 };
(*RET) = core::ptr::addr_of!((*_61));
place!(Field::<u8>(Variant(_66, 3), 0)) = !_26;
_53.1 = Field::<(i16, usize, u32)>(Variant(_18.2.3, 3), 2).0 as u8;
_22.1 = !Field::<(i16, usize, u32)>(Variant(_18.2.3, 3), 2).1;
_70.fld0 = Move(_32);
_22.1 = _12 ^ _24;
_64 = _18.1 <= _52;
place!(Field::<i32>(Variant(_18.2.3, 3), 5)) = 1883330893_i32 ^ 980286127_i32;
_46 = -_3;
place!(Field::<Adt26>(Variant(_53.0, 0), 1)) = Move(_70.fld0);
Goto(bb33)
}
bb33 = {
Call(_74 = dump_var(14_usize, 49_usize, Move(_49), 62_usize, Move(_62), 47_usize, Move(_47), 50_usize, Move(_50)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_74 = dump_var(14_usize, 27_usize, Move(_27), 2_usize, Move(_2), 19_usize, Move(_19), 34_usize, Move(_34)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_74 = dump_var(14_usize, 37_usize, Move(_37), 5_usize, Move(_5), 15_usize, Move(_15), 36_usize, Move(_36)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_74 = dump_var(14_usize, 59_usize, Move(_59), 56_usize, Move(_56), 16_usize, Move(_16), 31_usize, Move(_31)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_74 = dump_var(14_usize, 10_usize, Move(_10), 22_usize, Move(_22), 64_usize, Move(_64), 29_usize, Move(_29)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [u32; 8],mut _2: [i16; 8],mut _3: [usize; 5],mut _4: [usize; 5],mut _5: [u32; 8],mut _6: [usize; 5],mut _7: [u32; 8],mut _8: [u32; 8],mut _9: [i16; 8],mut _10: [i16; 8],mut _11: [usize; 5]) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _12: isize;
let _13: char;
let _14: char;
let _15: u16;
let _16: f64;
let _17: &'static u8;
let _18: f32;
let _19: f32;
let _20: *const Adt26;
let _21: ();
let _22: ();
{
_5 = [2722473362_u32,3132632097_u32,2256149357_u32,765763006_u32,979321122_u32,3857339509_u32,3428724421_u32,1272393361_u32];
_6 = [1_usize,5_usize,3_usize,9501964439732974103_usize,7_usize];
RET = _6;
_11 = _3;
_9 = _10;
_11 = [0_usize,10410505944932354363_usize,15334899794005070511_usize,13778826066337506540_usize,11753578807410704045_usize];
_3 = [311676469481920346_usize,7_usize,14576568056051063865_usize,5_usize,3_usize];
_6 = [4_usize,8171744616035742239_usize,3_usize,3_usize,1_usize];
Call(_11 = fn16(_8, _10, _7, _4, _9, _10, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = [(-25068_i16),8893_i16,1554_i16,(-26759_i16),31209_i16,(-8971_i16),(-4735_i16),8043_i16];
_11 = [8868321077381398584_usize,1_usize,1599789569048281360_usize,4_usize,1799020278547056958_usize];
_1 = _7;
_3 = [4_usize,7_usize,10973348183156073398_usize,5_usize,3_usize];
_7 = _1;
_1 = [251891899_u32,2472971041_u32,3155028735_u32,811314173_u32,286272542_u32,95908887_u32,3355991733_u32,3812694645_u32];
_6 = [2329146408751604337_usize,5_usize,6_usize,7_usize,6_usize];
_3 = [2_usize,1_usize,6_usize,0_usize,5_usize];
_9 = _2;
_1 = _8;
RET = [16653765361784856988_usize,12713969912345031011_usize,6_usize,5_usize,1_usize];
_9 = [4197_i16,20003_i16,7289_i16,8444_i16,(-19344_i16),15970_i16,(-17734_i16),(-29678_i16)];
_2 = _9;
RET = _3;
RET = [12352828488210162461_usize,7_usize,17097634504713139675_usize,6357591692495461020_usize,1_usize];
_10 = _2;
_2 = _10;
_12 = 9223372036854775807_isize;
_3 = [17748565706838521313_usize,9989610260039176839_usize,6_usize,4_usize,0_usize];
_7 = _8;
_2 = _10;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
_ => bb6
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
_1 = _5;
_9 = [(-30859_i16),(-7439_i16),24428_i16,22971_i16,(-14775_i16),2498_i16,(-13208_i16),(-28126_i16)];
_6 = [3_usize,5156513213927721670_usize,11646638560523815607_usize,16338579286992295242_usize,1_usize];
_9 = [(-19598_i16),21657_i16,14945_i16,30440_i16,27473_i16,29898_i16,(-6974_i16),120_i16];
_8 = [133817116_u32,2939401724_u32,3031283925_u32,4071454661_u32,427764570_u32,2142335948_u32,507754465_u32,2694082443_u32];
_2 = [(-1989_i16),(-5902_i16),12761_i16,11082_i16,14767_i16,3242_i16,30567_i16,(-31659_i16)];
_9 = [(-5752_i16),(-15033_i16),(-1411_i16),(-2236_i16),(-32016_i16),11129_i16,(-16736_i16),(-12148_i16)];
_8 = [1350768316_u32,1441595996_u32,43901638_u32,4271712231_u32,4149745262_u32,3095587987_u32,2528902015_u32,3752811494_u32];
_9 = _10;
_3 = RET;
_6 = [15972953849059676388_usize,0_usize,5_usize,7_usize,0_usize];
RET = [5_usize,14605752321508711660_usize,4_usize,16408299676773469294_usize,0_usize];
_1 = [677396473_u32,3441929937_u32,127712648_u32,4131339087_u32,3847609392_u32,3463029819_u32,3759403913_u32,4105747388_u32];
_10 = [30212_i16,29549_i16,8294_i16,28806_i16,3460_i16,(-32000_i16),13384_i16,28970_i16];
_7 = [4126869721_u32,13373510_u32,3035284211_u32,918272717_u32,3478576986_u32,3740855462_u32,982499842_u32,3836893425_u32];
_3 = [1_usize,8045337488584704064_usize,4_usize,14153130911600215073_usize,12007608203502440545_usize];
_7 = [3347965504_u32,2337470341_u32,3516896279_u32,2944945092_u32,3759041200_u32,3584470581_u32,2179556723_u32,2660547388_u32];
_13 = '\u{1067a1}';
_11 = _6;
_6 = [4_usize,16774451388891673861_usize,6_usize,3_usize,4639268677471409763_usize];
_8 = [1663703002_u32,2589748915_u32,3333932535_u32,3712319083_u32,869563647_u32,2240297710_u32,3959741042_u32,3999202510_u32];
Goto(bb8)
}
bb8 = {
_8 = _7;
_3 = [6_usize,5814289791648194024_usize,9690398049744825786_usize,14321595098870161579_usize,5845842229621127948_usize];
_3 = RET;
_15 = 26229_u16;
_4 = [5_usize,12161989613228702935_usize,3_usize,5_usize,9876979814211255408_usize];
_5 = _1;
_11 = [4_usize,6755977745448818294_usize,468914682826302522_usize,5_usize,12281448344209345211_usize];
_14 = _13;
_8 = [4291199325_u32,394126624_u32,3797223074_u32,1418403183_u32,3549391873_u32,2034280958_u32,4288826804_u32,729648740_u32];
_16 = 1386098876_i32 as f64;
_10 = [(-5678_i16),(-28719_i16),14034_i16,(-13202_i16),20918_i16,19470_i16,(-21782_i16),(-30340_i16)];
_3 = [6025275676248432932_usize,3_usize,15197200407844565790_usize,3_usize,429608519768025492_usize];
_9 = [(-22844_i16),10400_i16,(-13209_i16),(-14282_i16),23542_i16,30583_i16,18017_i16,(-2926_i16)];
match _15 {
0 => bb7,
1 => bb9,
2 => bb10,
26229 => bb12,
_ => bb11
}
}
bb9 = {
_1 = _5;
_9 = [(-30859_i16),(-7439_i16),24428_i16,22971_i16,(-14775_i16),2498_i16,(-13208_i16),(-28126_i16)];
_6 = [3_usize,5156513213927721670_usize,11646638560523815607_usize,16338579286992295242_usize,1_usize];
_9 = [(-19598_i16),21657_i16,14945_i16,30440_i16,27473_i16,29898_i16,(-6974_i16),120_i16];
_8 = [133817116_u32,2939401724_u32,3031283925_u32,4071454661_u32,427764570_u32,2142335948_u32,507754465_u32,2694082443_u32];
_2 = [(-1989_i16),(-5902_i16),12761_i16,11082_i16,14767_i16,3242_i16,30567_i16,(-31659_i16)];
_9 = [(-5752_i16),(-15033_i16),(-1411_i16),(-2236_i16),(-32016_i16),11129_i16,(-16736_i16),(-12148_i16)];
_8 = [1350768316_u32,1441595996_u32,43901638_u32,4271712231_u32,4149745262_u32,3095587987_u32,2528902015_u32,3752811494_u32];
_9 = _10;
_3 = RET;
_6 = [15972953849059676388_usize,0_usize,5_usize,7_usize,0_usize];
RET = [5_usize,14605752321508711660_usize,4_usize,16408299676773469294_usize,0_usize];
_1 = [677396473_u32,3441929937_u32,127712648_u32,4131339087_u32,3847609392_u32,3463029819_u32,3759403913_u32,4105747388_u32];
_10 = [30212_i16,29549_i16,8294_i16,28806_i16,3460_i16,(-32000_i16),13384_i16,28970_i16];
_7 = [4126869721_u32,13373510_u32,3035284211_u32,918272717_u32,3478576986_u32,3740855462_u32,982499842_u32,3836893425_u32];
_3 = [1_usize,8045337488584704064_usize,4_usize,14153130911600215073_usize,12007608203502440545_usize];
_7 = [3347965504_u32,2337470341_u32,3516896279_u32,2944945092_u32,3759041200_u32,3584470581_u32,2179556723_u32,2660547388_u32];
_13 = '\u{1067a1}';
_11 = _6;
_6 = [4_usize,16774451388891673861_usize,6_usize,3_usize,4639268677471409763_usize];
_8 = [1663703002_u32,2589748915_u32,3333932535_u32,3712319083_u32,869563647_u32,2240297710_u32,3959741042_u32,3999202510_u32];
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_4 = [2857919017806598416_usize,5_usize,1_usize,0_usize,13728287067846795875_usize];
RET = [8228727938209017552_usize,0_usize,15803451246993467207_usize,11850170109442632277_usize,9112818731502064671_usize];
_13 = _14;
RET = [6942347551542377923_usize,2650527445280172001_usize,3_usize,4_usize,0_usize];
RET = [0_usize,6455866821683612740_usize,18334468052269242763_usize,3980561541670251359_usize,9489945202258373613_usize];
_3 = [17009491631039263133_usize,0_usize,7_usize,5_usize,6_usize];
_16 = 13937303445523279914_u64 as f64;
_12 = (-9223372036854775808_isize);
_6 = _4;
_7 = [1470497961_u32,396943820_u32,653324654_u32,453416958_u32,1030754222_u32,1716863599_u32,3819209106_u32,1103613324_u32];
_9 = [17938_i16,24360_i16,(-4211_i16),9449_i16,25602_i16,17708_i16,23489_i16,16773_i16];
_8 = [3071060296_u32,3835452229_u32,1098249267_u32,1637161467_u32,2405577765_u32,3631320647_u32,693893314_u32,1290428105_u32];
_18 = 140765215040382361573350832493169452538_i128 as f32;
_15 = 60058_u16;
_14 = _13;
_6 = [2_usize,1_usize,4622582104685463698_usize,2_usize,1_usize];
_3 = [7556938377265861749_usize,10203580545995223999_usize,978892771769751401_usize,7861400282654240698_usize,18115606567339681068_usize];
_5 = [1617353304_u32,1128135412_u32,1201208286_u32,4016155285_u32,1013291107_u32,1200450678_u32,3962959395_u32,1620096800_u32];
_18 = 1791643762_i32 as f32;
_12 = 13_isize & 9223372036854775807_isize;
_10 = _2;
_8 = [1169936015_u32,1500741456_u32,1183647485_u32,755985134_u32,144307031_u32,1759292857_u32,3159895855_u32,3880212392_u32];
_5 = [1721096498_u32,1588759885_u32,818709340_u32,620679090_u32,3187353476_u32,3140117725_u32,2576842469_u32,1933335922_u32];
_1 = _8;
_2 = [(-23840_i16),(-22535_i16),4414_i16,(-25685_i16),19864_i16,25687_i16,(-18844_i16),(-9100_i16)];
match _15 {
0 => bb1,
1 => bb6,
60058 => bb13,
_ => bb11
}
}
bb13 = {
_16 = 52848480_i32 as f64;
_10 = [(-11979_i16),8329_i16,(-3526_i16),(-14044_i16),(-32393_i16),(-4084_i16),(-13279_i16),(-23862_i16)];
_8 = _5;
RET = [12071445211636938644_usize,9024314396716454115_usize,3_usize,15283698497442162988_usize,2_usize];
_16 = _12 as f64;
_6 = [6617911820409805803_usize,10054221489852732375_usize,2741773871844514824_usize,1432320576439300713_usize,7050246675266712679_usize];
_19 = _18;
match _15 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb5,
4 => bb14,
5 => bb15,
6 => bb16,
60058 => bb18,
_ => bb17
}
}
bb14 = {
_8 = _7;
_3 = [6_usize,5814289791648194024_usize,9690398049744825786_usize,14321595098870161579_usize,5845842229621127948_usize];
_3 = RET;
_15 = 26229_u16;
_4 = [5_usize,12161989613228702935_usize,3_usize,5_usize,9876979814211255408_usize];
_5 = _1;
_11 = [4_usize,6755977745448818294_usize,468914682826302522_usize,5_usize,12281448344209345211_usize];
_14 = _13;
_8 = [4291199325_u32,394126624_u32,3797223074_u32,1418403183_u32,3549391873_u32,2034280958_u32,4288826804_u32,729648740_u32];
_16 = 1386098876_i32 as f64;
_10 = [(-5678_i16),(-28719_i16),14034_i16,(-13202_i16),20918_i16,19470_i16,(-21782_i16),(-30340_i16)];
_3 = [6025275676248432932_usize,3_usize,15197200407844565790_usize,3_usize,429608519768025492_usize];
_9 = [(-22844_i16),10400_i16,(-13209_i16),(-14282_i16),23542_i16,30583_i16,18017_i16,(-2926_i16)];
match _15 {
0 => bb7,
1 => bb9,
2 => bb10,
26229 => bb12,
_ => bb11
}
}
bb15 = {
_2 = [(-25068_i16),8893_i16,1554_i16,(-26759_i16),31209_i16,(-8971_i16),(-4735_i16),8043_i16];
_11 = [8868321077381398584_usize,1_usize,1599789569048281360_usize,4_usize,1799020278547056958_usize];
_1 = _7;
_3 = [4_usize,7_usize,10973348183156073398_usize,5_usize,3_usize];
_7 = _1;
_1 = [251891899_u32,2472971041_u32,3155028735_u32,811314173_u32,286272542_u32,95908887_u32,3355991733_u32,3812694645_u32];
_6 = [2329146408751604337_usize,5_usize,6_usize,7_usize,6_usize];
_3 = [2_usize,1_usize,6_usize,0_usize,5_usize];
_9 = _2;
_1 = _8;
RET = [16653765361784856988_usize,12713969912345031011_usize,6_usize,5_usize,1_usize];
_9 = [4197_i16,20003_i16,7289_i16,8444_i16,(-19344_i16),15970_i16,(-17734_i16),(-29678_i16)];
_2 = _9;
RET = _3;
RET = [12352828488210162461_usize,7_usize,17097634504713139675_usize,6357591692495461020_usize,1_usize];
_10 = _2;
_2 = _10;
_12 = 9223372036854775807_isize;
_3 = [17748565706838521313_usize,9989610260039176839_usize,6_usize,4_usize,0_usize];
_7 = _8;
_2 = _10;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET = _3;
_13 = _14;
RET = [17695004947863988345_usize,0_usize,3_usize,0_usize,0_usize];
_6 = RET;
_11 = [1985729015723031574_usize,2_usize,4_usize,4_usize,12469676272638271186_usize];
_2 = [(-19367_i16),3035_i16,(-2221_i16),30399_i16,9408_i16,17945_i16,32742_i16,(-2861_i16)];
_15 = 2_usize as u16;
_13 = _14;
Goto(bb19)
}
bb19 = {
Call(_21 = dump_var(15_usize, 1_usize, Move(_1), 10_usize, Move(_10), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_21 = dump_var(15_usize, 12_usize, Move(_12), 3_usize, Move(_3), 9_usize, Move(_9), 22_usize, _22), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [u32; 8],mut _2: [i16; 8],mut _3: [u32; 8],mut _4: [usize; 5],mut _5: [i16; 8],mut _6: [i16; 8],mut _7: [i16; 8]) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _8: bool;
let _9: bool;
let _10: f64;
let _11: [i16; 8];
let _12: [i8; 6];
let _13: u128;
let _14: f32;
let _15: f64;
let _16: &'static u64;
let _17: char;
let _18: Adt31;
let _19: isize;
let _20: bool;
let _21: *const &'static [i128; 6];
let _22: (u32, *const Adt26, *mut i32, u128);
let _23: f64;
let _24: *const Adt63;
let _25: (i32, [isize; 8]);
let _26: (&'static &'static u8,);
let _27: (i16, f64, (i8, [i8; 6], *const Adt17, Adt22), u16);
let _28: bool;
let _29: isize;
let _30: i128;
let _31: (f32, [isize; 8], [bool; 8]);
let _32: *mut Adt38;
let _33: (isize, Adt38, bool);
let _34: [u16; 4];
let _35: *const &'static [i128; 6];
let _36: isize;
let _37: u16;
let _38: [i128; 6];
let _39: ();
let _40: ();
{
RET = [7_usize,2_usize,7_usize,15272381656836626593_usize,5_usize];
_4 = [5_usize,7_usize,6_usize,6429795878464858293_usize,7163275603837485016_usize];
RET = [7_usize,5_usize,4_usize,1388093547918358817_usize,3_usize];
_7 = [(-5084_i16),(-12646_i16),25017_i16,(-26692_i16),(-11613_i16),24798_i16,22402_i16,(-1387_i16)];
Goto(bb1)
}
bb1 = {
_1 = [4096820732_u32,927545838_u32,1384419519_u32,1501009096_u32,4061962363_u32,456460887_u32,3518747473_u32,1634888969_u32];
_7 = _5;
_1 = [3212283507_u32,281981306_u32,698119950_u32,4214256899_u32,319506063_u32,2628454640_u32,1878727704_u32,1482111183_u32];
_4 = [5079569371958515829_usize,1_usize,5_usize,5047063406905953827_usize,3_usize];
_2 = [17924_i16,808_i16,17453_i16,11434_i16,(-19534_i16),6083_i16,25377_i16,18792_i16];
RET = _4;
_6 = [6328_i16,26211_i16,(-19199_i16),(-27420_i16),14932_i16,(-31258_i16),(-25537_i16),(-10351_i16)];
_3 = [1092356966_u32,3962905643_u32,1161557785_u32,4181829183_u32,3348734491_u32,2648670541_u32,2186903687_u32,2385576740_u32];
_1 = _3;
RET = _4;
_2 = [19765_i16,(-3191_i16),(-23650_i16),(-28230_i16),25330_i16,24311_i16,28173_i16,(-6066_i16)];
_5 = [15873_i16,(-27379_i16),(-19523_i16),(-20627_i16),(-6832_i16),15438_i16,2332_i16,(-12921_i16)];
RET = _4;
_6 = [7472_i16,15608_i16,23166_i16,6384_i16,(-5650_i16),2759_i16,7905_i16,21330_i16];
_6 = [7623_i16,(-11098_i16),(-21674_i16),(-15616_i16),26532_i16,28716_i16,(-28112_i16),(-20436_i16)];
RET = _4;
_1 = [603975980_u32,3376704039_u32,3248295728_u32,2248227941_u32,68861947_u32,1817249426_u32,3896415462_u32,3512191606_u32];
Goto(bb2)
}
bb2 = {
_7 = [(-10278_i16),(-4763_i16),10543_i16,(-26090_i16),(-27786_i16),17266_i16,(-20431_i16),28299_i16];
_3 = [98523030_u32,1406749031_u32,193708590_u32,685940730_u32,3616207563_u32,232900062_u32,2206801096_u32,2746992942_u32];
_4 = RET;
RET = _4;
_2 = [11987_i16,3066_i16,(-27469_i16),(-30868_i16),(-4606_i16),25923_i16,(-20749_i16),6949_i16];
_8 = !false;
_7 = [6057_i16,(-22441_i16),32295_i16,11338_i16,6779_i16,26702_i16,13193_i16,(-7223_i16)];
_6 = [12907_i16,18163_i16,(-19488_i16),32614_i16,(-11628_i16),(-26585_i16),(-25772_i16),(-1227_i16)];
Goto(bb3)
}
bb3 = {
_5 = [(-29745_i16),(-31580_i16),21104_i16,(-2473_i16),6967_i16,(-14313_i16),32490_i16,(-9492_i16)];
_6 = _2;
_9 = _8 == _8;
_4 = RET;
_3 = _1;
_6 = _2;
_6 = [(-19574_i16),(-3615_i16),(-5586_i16),10507_i16,17038_i16,(-9207_i16),6618_i16,(-14640_i16)];
_2 = [(-19885_i16),(-20203_i16),10403_i16,(-31028_i16),(-30556_i16),(-23293_i16),18610_i16,(-14656_i16)];
_2 = _5;
_6 = [(-9829_i16),18874_i16,29455_i16,8681_i16,9547_i16,(-12532_i16),4976_i16,(-24974_i16)];
_9 = _8;
_8 = _9 < _9;
_4 = [4_usize,11726799363409917255_usize,3829108077458160448_usize,0_usize,832327938778408499_usize];
_7 = [30051_i16,(-23894_i16),6266_i16,27968_i16,7417_i16,2383_i16,31079_i16,15777_i16];
_11 = [(-16102_i16),(-17919_i16),16342_i16,8388_i16,30591_i16,28377_i16,16042_i16,17217_i16];
_11 = _2;
_10 = 13790_u16 as f64;
RET = _4;
_2 = _7;
_9 = _8 ^ _8;
_12 = [(-46_i8),(-74_i8),119_i8,(-20_i8),115_i8,115_i8];
_2 = _5;
_3 = [3269424825_u32,160803423_u32,2080639602_u32,782585119_u32,2351320271_u32,2212298187_u32,3647568070_u32,1759470645_u32];
RET = [11297621086371532220_usize,4_usize,17086358068995479249_usize,1_usize,5_usize];
_8 = _9 > _9;
_11 = _5;
_11 = [3008_i16,(-427_i16),(-13283_i16),8948_i16,(-29084_i16),(-10233_i16),14988_i16,31393_i16];
_7 = [(-10506_i16),(-32694_i16),29872_i16,12156_i16,31435_i16,6279_i16,23632_i16,(-27310_i16)];
_4 = [1438542080970128361_usize,5_usize,3433600921316194315_usize,2_usize,12377054363989812040_usize];
Goto(bb4)
}
bb4 = {
_4 = [14057089766866922572_usize,1_usize,7953335409036233741_usize,16739272119584465230_usize,11781901649908151560_usize];
_11 = [14908_i16,4282_i16,207_i16,20215_i16,(-30337_i16),(-431_i16),(-26550_i16),(-9828_i16)];
_8 = _9 ^ _9;
_10 = 2674813766103184760_u64 as f64;
_10 = 11090871917017825574_u64 as f64;
_7 = [20745_i16,(-28365_i16),16310_i16,3642_i16,11402_i16,9834_i16,(-16588_i16),(-16282_i16)];
Goto(bb5)
}
bb5 = {
_7 = [(-15035_i16),3465_i16,18112_i16,(-30737_i16),31502_i16,32368_i16,(-30023_i16),7872_i16];
_7 = [19540_i16,17218_i16,15316_i16,(-5618_i16),29644_i16,(-21794_i16),27907_i16,(-18903_i16)];
_2 = [16339_i16,23740_i16,12648_i16,(-3572_i16),26242_i16,(-23334_i16),9332_i16,(-27225_i16)];
_1 = _3;
_2 = _6;
_2 = [(-20215_i16),(-23171_i16),(-24513_i16),(-24514_i16),7777_i16,(-29441_i16),26549_i16,(-11613_i16)];
_13 = 119965926484616268452044964460863801342_u128 ^ 337261569051894257747793888548973724725_u128;
_4 = [7_usize,1354993106881648774_usize,16558366914743214673_usize,6_usize,5_usize];
_14 = 4204363370_u32 as f32;
_10 = 7680_i16 as f64;
_13 = 1720_u16 as u128;
RET = [5_usize,0_usize,0_usize,483766323252930436_usize,1_usize];
_14 = 9223372036854775807_isize as f32;
_5 = [2816_i16,7879_i16,2268_i16,17750_i16,14399_i16,(-16072_i16),32267_i16,10982_i16];
_10 = 39561_u16 as f64;
_19 = !9223372036854775807_isize;
_14 = _13 as f32;
_8 = _13 == _13;
_12 = [(-82_i8),25_i8,49_i8,(-87_i8),8_i8,(-8_i8)];
_8 = _9;
_19 = !(-9223372036854775808_isize);
RET = [5_usize,17573204512194801254_usize,9382642681193588079_usize,0_usize,6930487121729962787_usize];
_15 = 42_i8 as f64;
Call(_21 = fn17(), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8 = _9 ^ _9;
_17 = '\u{745e3}';
_13 = 5312143312467754223219505000521297911_u128 | 271989773740217734295238851452970327080_u128;
_7 = [(-20030_i16),14419_i16,(-17834_i16),(-27148_i16),925_i16,13564_i16,(-9436_i16),(-26021_i16)];
_17 = '\u{ac42}';
_20 = !_9;
Goto(bb7)
}
bb7 = {
_2 = [26610_i16,(-14578_i16),233_i16,1194_i16,(-13901_i16),24359_i16,(-21086_i16),(-11943_i16)];
_22.0 = 216_u8 as u32;
_22.3 = _13 * _13;
_13 = !_22.3;
_22.3 = _14 as u128;
_12 = [104_i8,(-65_i8),68_i8,70_i8,(-70_i8),38_i8];
_11 = [(-23036_i16),(-25641_i16),28864_i16,12473_i16,(-8974_i16),24242_i16,9293_i16,(-12816_i16)];
_5 = [(-26257_i16),(-13123_i16),(-1141_i16),24876_i16,(-4228_i16),31926_i16,(-389_i16),29254_i16];
_9 = _8;
_23 = _15;
_25.0 = !2039984778_i32;
_25.0 = (-1729319191_i32);
_25.0 = -(-751983426_i32);
_1 = _3;
_27.2.0 = (-28_i8) ^ (-1_i8);
Goto(bb8)
}
bb8 = {
_27.1 = -_23;
Goto(bb9)
}
bb9 = {
_25.1 = [_19,_19,_19,_19,_19,_19,_19,_19];
_1 = _3;
_20 = _14 == _14;
RET = [1653523045436954670_usize,6_usize,3771906458591522884_usize,8680392631787955453_usize,3_usize];
_27.2.1 = [_27.2.0,_27.2.0,_27.2.0,_27.2.0,_27.2.0,_27.2.0];
_11 = [10469_i16,(-20786_i16),32423_i16,(-9095_i16),514_i16,(-25663_i16),17540_i16,(-16765_i16)];
_27.3 = !23217_u16;
_29 = !_19;
_11 = [2597_i16,(-21590_i16),9038_i16,(-29926_i16),(-18124_i16),(-15941_i16),(-23480_i16),4293_i16];
_4 = [10035663581805663044_usize,4_usize,32793299219091696_usize,0_usize,5_usize];
RET = [17870064672907073691_usize,7403128558410312952_usize,1_usize,15892913935446122412_usize,3_usize];
_25.1 = [_19,_29,_29,_19,_29,_19,_29,_19];
_5 = _11;
_27.2.0 = (-32_i8);
_28 = _22.3 == _13;
_19 = 2_usize as isize;
_3 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_31.1 = _25.1;
_31.0 = _25.0 as f32;
_4 = [7_usize,0_usize,6_usize,3_usize,1001384640461438730_usize];
_31.1 = [_29,_29,_29,_19,_19,_19,_29,_19];
_25.1 = _31.1;
match _27.2.0 {
0 => bb5,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463463374607431768211424 => bb14,
_ => bb13
}
}
bb10 = {
_27.1 = -_23;
Goto(bb9)
}
bb11 = {
_1 = [4096820732_u32,927545838_u32,1384419519_u32,1501009096_u32,4061962363_u32,456460887_u32,3518747473_u32,1634888969_u32];
_7 = _5;
_1 = [3212283507_u32,281981306_u32,698119950_u32,4214256899_u32,319506063_u32,2628454640_u32,1878727704_u32,1482111183_u32];
_4 = [5079569371958515829_usize,1_usize,5_usize,5047063406905953827_usize,3_usize];
_2 = [17924_i16,808_i16,17453_i16,11434_i16,(-19534_i16),6083_i16,25377_i16,18792_i16];
RET = _4;
_6 = [6328_i16,26211_i16,(-19199_i16),(-27420_i16),14932_i16,(-31258_i16),(-25537_i16),(-10351_i16)];
_3 = [1092356966_u32,3962905643_u32,1161557785_u32,4181829183_u32,3348734491_u32,2648670541_u32,2186903687_u32,2385576740_u32];
_1 = _3;
RET = _4;
_2 = [19765_i16,(-3191_i16),(-23650_i16),(-28230_i16),25330_i16,24311_i16,28173_i16,(-6066_i16)];
_5 = [15873_i16,(-27379_i16),(-19523_i16),(-20627_i16),(-6832_i16),15438_i16,2332_i16,(-12921_i16)];
RET = _4;
_6 = [7472_i16,15608_i16,23166_i16,6384_i16,(-5650_i16),2759_i16,7905_i16,21330_i16];
_6 = [7623_i16,(-11098_i16),(-21674_i16),(-15616_i16),26532_i16,28716_i16,(-28112_i16),(-20436_i16)];
RET = _4;
_1 = [603975980_u32,3376704039_u32,3248295728_u32,2248227941_u32,68861947_u32,1817249426_u32,3896415462_u32,3512191606_u32];
Goto(bb2)
}
bb12 = {
_7 = [(-10278_i16),(-4763_i16),10543_i16,(-26090_i16),(-27786_i16),17266_i16,(-20431_i16),28299_i16];
_3 = [98523030_u32,1406749031_u32,193708590_u32,685940730_u32,3616207563_u32,232900062_u32,2206801096_u32,2746992942_u32];
_4 = RET;
RET = _4;
_2 = [11987_i16,3066_i16,(-27469_i16),(-30868_i16),(-4606_i16),25923_i16,(-20749_i16),6949_i16];
_8 = !false;
_7 = [6057_i16,(-22441_i16),32295_i16,11338_i16,6779_i16,26702_i16,13193_i16,(-7223_i16)];
_6 = [12907_i16,18163_i16,(-19488_i16),32614_i16,(-11628_i16),(-26585_i16),(-25772_i16),(-1227_i16)];
Goto(bb3)
}
bb13 = {
_4 = [14057089766866922572_usize,1_usize,7953335409036233741_usize,16739272119584465230_usize,11781901649908151560_usize];
_11 = [14908_i16,4282_i16,207_i16,20215_i16,(-30337_i16),(-431_i16),(-26550_i16),(-9828_i16)];
_8 = _9 ^ _9;
_10 = 2674813766103184760_u64 as f64;
_10 = 11090871917017825574_u64 as f64;
_7 = [20745_i16,(-28365_i16),16310_i16,3642_i16,11402_i16,9834_i16,(-16588_i16),(-16282_i16)];
Goto(bb5)
}
bb14 = {
_32 = core::ptr::addr_of_mut!(_33.1);
_19 = !_29;
_33.1.fld3 = 167_u8 as i8;
(*_32).fld1.0 = _33.1.fld3;
_32 = core::ptr::addr_of_mut!((*_32));
_37 = _27.3 >> (*_32).fld1.0;
_10 = -_23;
_11 = _6;
_35 = Move(_21);
(*_32).fld2.1 = !153_u8;
(*_32).fld1.1 = [(*_32).fld3,_27.2.0,_27.2.0,_33.1.fld1.0,(*_32).fld1.0,_33.1.fld3];
_34 = [_37,_37,_27.3,_37];
_33.0 = _19 >> (*_32).fld3;
_27.3 = _37 & _37;
(*_32).fld0 = [(-5343_i16),(-6896_i16),(-22643_i16),(-30390_i16),(-27332_i16),(-23246_i16)];
(*_32).fld1.0 = !(*_32).fld3;
(*_32).fld2.1 = 223_u8 >> _33.0;
(*_32).fld2.1 = 32317_i16 as u8;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(16_usize, 19_usize, Move(_19), 28_usize, Move(_28), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(16_usize, 2_usize, Move(_2), 37_usize, Move(_37), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(16_usize, 20_usize, Move(_20), 17_usize, Move(_17), 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17() -> *const &'static [i128; 6] {
mir! {
type RET = *const &'static [i128; 6];
let _1: f32;
let _2: bool;
let _3: Adt31;
let _4: [u8; 4];
let _5: (i32, [isize; 8]);
let _6: *const [u16; 4];
let _7: i32;
let _8: &'static u8;
let _9: char;
let _10: [isize; 2];
let _11: [i8; 6];
let _12: u128;
let _13: *const (Adt38,);
let _14: f64;
let _15: [bool; 2];
let _16: &'static i8;
let _17: f64;
let _18: &'static &'static u8;
let _19: (i8, [i8; 6], *const Adt17, Adt22);
let _20: isize;
let _21: u32;
let _22: *const [u16; 4];
let _23: [bool; 2];
let _24: (f32, [isize; 8], [bool; 8]);
let _25: [i128; 2];
let _26: bool;
let _27: [i128; 6];
let _28: [u32; 8];
let _29: f64;
let _30: [char; 3];
let _31: u64;
let _32: Adt17;
let _33: &'static *mut Adt38;
let _34: i128;
let _35: isize;
let _36: *const [u16; 4];
let _37: *const f32;
let _38: f64;
let _39: char;
let _40: (&'static [i64; 3], f32, *const (u32, *const Adt26, *mut i32, u128));
let _41: *mut i32;
let _42: isize;
let _43: (&'static [i64; 3], f32, *const (u32, *const Adt26, *mut i32, u128));
let _44: isize;
let _45: (&'static [i64; 3], f32, *const (u32, *const Adt26, *mut i32, u128));
let _46: *const Adt63;
let _47: usize;
let _48: i128;
let _49: char;
let _50: [i16; 6];
let _51: [i64; 6];
let _52: f64;
let _53: *const Adt26;
let _54: *const f64;
let _55: f32;
let _56: bool;
let _57: bool;
let _58: *const Adt17;
let _59: f64;
let _60: f64;
let _61: isize;
let _62: [i8; 6];
let _63: isize;
let _64: char;
let _65: u16;
let _66: char;
let _67: isize;
let _68: isize;
let _69: *mut Adt38;
let _70: isize;
let _71: *mut i32;
let _72: &'static *const Adt63;
let _73: isize;
let _74: isize;
let _75: &'static [i128; 6];
let _76: i8;
let _77: (i32, [isize; 8]);
let _78: *const Adt26;
let _79: ();
let _80: ();
{
_1 = 202_u8 as f32;
_2 = false | false;
_1 = 16566_i16 as f32;
_2 = !true;
_1 = 46089_u16 as f32;
_1 = (-687628584932981342_i64) as f32;
_1 = 1637104131_i32 as f32;
_1 = 4559259996652418746_u64 as f32;
_2 = !true;
_2 = !false;
Goto(bb1)
}
bb1 = {
_2 = _1 < _1;
_1 = 4_usize as f32;
_1 = 26086_i16 as f32;
_1 = (-9223372036854775808_isize) as f32;
_2 = false;
_1 = 2_usize as f32;
Goto(bb2)
}
bb2 = {
_2 = _1 >= _1;
_5.0 = (-1737306658_i32);
_4 = [55_u8,177_u8,137_u8,157_u8];
_5.0 = '\u{f57a7}' as i32;
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,41_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 != _1;
_5.1 = [9223372036854775807_isize,19_isize,(-103_isize),9223372036854775807_isize,18_isize,99_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !true;
_5.1 = [(-58_isize),101_isize,9223372036854775807_isize,(-9223372036854775808_isize),124_isize,40_isize,(-26_isize),125_isize];
_2 = true;
Goto(bb3)
}
bb3 = {
_5.1 = [(-46_isize),19_isize,(-30_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),113_isize,9223372036854775807_isize];
_1 = _5.0 as f32;
_1 = 46586_u16 as f32;
_1 = _5.0 as f32;
_4 = [12_u8,83_u8,241_u8,238_u8];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-51_isize),9223372036854775807_isize,62_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = !_5.0;
_2 = true;
_4 = [97_u8,74_u8,16_u8,25_u8];
_2 = !false;
_4 = [254_u8,176_u8,209_u8,134_u8];
_7 = _5.0 | _5.0;
_5.1 = [(-80_isize),9223372036854775807_isize,0_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),51_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _7 <= _7;
_1 = 1642091449_u32 as f32;
_7 = _5.0;
_5.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-127_isize),(-57_isize),(-9223372036854775808_isize),(-25_isize),(-94_isize),(-26_isize)];
_5.1 = [9223372036854775807_isize,9223372036854775807_isize,(-80_isize),121_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _7;
_1 = 30634_u16 as f32;
Goto(bb4)
}
bb4 = {
_2 = false;
_9 = '\u{9db4a}';
_9 = '\u{d83ac}';
_2 = _9 <= _9;
_5.1 = [9223372036854775807_isize,(-47_isize),(-11_isize),96_isize,9223372036854775807_isize,(-107_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{da5b8}';
_4 = [64_u8,170_u8,150_u8,43_u8];
_5.0 = _7;
_7 = 15166288426535030355_usize as i32;
_5.0 = _7 & _7;
_2 = !false;
_4 = [119_u8,100_u8,132_u8,43_u8];
_5.0 = _7 + _7;
_1 = (-9223372036854775808_isize) as f32;
_5.1 = [(-54_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,120_isize,9223372036854775807_isize,26_isize];
_1 = 185630275521473572246971821696549558823_u128 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{20522}';
_2 = !true;
_7 = 22375_u16 as i32;
_5.0 = -_7;
_10 = [(-9223372036854775808_isize),25_isize];
Goto(bb5)
}
bb5 = {
_1 = 8651_i16 as f32;
_5.1 = [115_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),75_isize,35_isize,(-9223372036854775808_isize)];
_5.0 = _1 as i32;
_5.0 = 12626367720158535078381368251164459909_u128 as i32;
_11 = [94_i8,2_i8,121_i8,117_i8,105_i8,109_i8];
Goto(bb6)
}
bb6 = {
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = 3635407212_u32 as f32;
_11 = [(-111_i8),(-109_i8),81_i8,(-77_i8),(-64_i8),(-111_i8)];
_11 = [34_i8,68_i8,72_i8,(-58_i8),(-33_i8),(-26_i8)];
_5.0 = -_7;
_2 = false;
_4 = [185_u8,207_u8,119_u8,150_u8];
_5.0 = _7;
_9 = '\u{583cb}';
_1 = 15441934374147651486_usize as f32;
Goto(bb7)
}
bb7 = {
_12 = 272514297127941596773701712677958424_u128;
_9 = '\u{8d6ce}';
_5.1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-93_isize),(-64_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{b7da0}';
_4 = [254_u8,177_u8,145_u8,74_u8];
_11 = [29_i8,(-39_i8),62_i8,46_i8,119_i8,82_i8];
_4 = [205_u8,226_u8,100_u8,104_u8];
_4 = [74_u8,154_u8,45_u8,140_u8];
_5.1 = [(-9223372036854775808_isize),57_isize,(-72_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),62_isize];
_10 = [20_isize,(-110_isize)];
_10 = [9_isize,(-9223372036854775808_isize)];
_11 = [(-51_i8),99_i8,126_i8,(-52_i8),(-12_i8),(-123_i8)];
_5.0 = _7;
_9 = '\u{caf0}';
_14 = (-84_i8) as f64;
_5.0 = _7;
_7 = -_5.0;
_5.1 = [(-52_isize),62_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,83_isize,9223372036854775807_isize];
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
272514297127941596773701712677958424 => bb9,
_ => bb8
}
}
bb8 = {
_2 = _1 < _1;
_1 = 4_usize as f32;
_1 = 26086_i16 as f32;
_1 = (-9223372036854775808_isize) as f32;
_2 = false;
_1 = 2_usize as f32;
Goto(bb2)
}
bb9 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb10 = {
_16 = &_19.0;
_1 = (-5131629559851951552_i64) as f32;
_10 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_17 = _14;
_2 = !false;
_4 = [153_u8,67_u8,78_u8,252_u8];
_19.1 = [(*_16),(*_16),(*_16),_19.0,(*_16),(*_16)];
_16 = &_19.0;
_16 = &(*_16);
_10 = [(-115_isize),73_isize];
_17 = _14;
_10 = [88_isize,(-9223372036854775808_isize)];
Call(_19.1 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_10 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_17 = _1 as f64;
_5.0 = 48218453519444491205407948163538239976_i128 as i32;
_16 = &_19.0;
_19.0 = 91_i8 >> _7;
_20 = 9049334161399795722_i64 as isize;
_14 = 178006590533605529_i64 as f64;
_20 = 9223372036854775807_isize;
_21 = 1253937164_u32 >> _12;
_16 = &_19.0;
_4 = [26_u8,204_u8,20_u8,223_u8];
_12 = 239461241880445555154794917048852485158_u128;
_14 = _17;
_20 = _2 as isize;
_11 = [(*_16),(*_16),(*_16),(*_16),(*_16),(*_16)];
_1 = _12 as f32;
_21 = 2644248865_u32 * 1080770997_u32;
match _12 {
0 => bb8,
1 => bb6,
239461241880445555154794917048852485158 => bb13,
_ => bb12
}
}
bb12 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb13 = {
_19.1 = [_19.0,(*_16),_19.0,(*_16),_19.0,(*_16)];
_11 = [_19.0,(*_16),(*_16),(*_16),(*_16),(*_16)];
_2 = !false;
_23 = [_2,_2];
_17 = -_14;
_20 = !(-19_isize);
_10 = [_20,_20];
_5.0 = _7;
_14 = _17;
_1 = (-32302_i16) as f32;
_4 = [30_u8,69_u8,179_u8,78_u8];
_20 = -9223372036854775807_isize;
_7 = !_5.0;
_20 = -(-9223372036854775808_isize);
_9 = '\u{b49b7}';
_2 = false;
_4 = [148_u8,42_u8,197_u8,148_u8];
_24.1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_5.1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_24.2 = [_2,_2,_2,_2,_2,_2,_2,_2];
_18 = &_8;
_19.0 = _7 as i8;
match _12 {
0 => bb14,
239461241880445555154794917048852485158 => bb16,
_ => bb15
}
}
bb14 = {
_2 = _1 < _1;
_1 = 4_usize as f32;
_1 = 26086_i16 as f32;
_1 = (-9223372036854775808_isize) as f32;
_2 = false;
_1 = 2_usize as f32;
Goto(bb2)
}
bb15 = {
_2 = false;
_9 = '\u{9db4a}';
_9 = '\u{d83ac}';
_2 = _9 <= _9;
_5.1 = [9223372036854775807_isize,(-47_isize),(-11_isize),96_isize,9223372036854775807_isize,(-107_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{da5b8}';
_4 = [64_u8,170_u8,150_u8,43_u8];
_5.0 = _7;
_7 = 15166288426535030355_usize as i32;
_5.0 = _7 & _7;
_2 = !false;
_4 = [119_u8,100_u8,132_u8,43_u8];
_5.0 = _7 + _7;
_1 = (-9223372036854775808_isize) as f32;
_5.1 = [(-54_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,120_isize,9223372036854775807_isize,26_isize];
_1 = 185630275521473572246971821696549558823_u128 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{20522}';
_2 = !true;
_7 = 22375_u16 as i32;
_5.0 = -_7;
_10 = [(-9223372036854775808_isize),25_isize];
Goto(bb5)
}
bb16 = {
_10 = [_20,_20];
_24.2 = [_2,_2,_2,_2,_2,_2,_2,_2];
_24.2 = [_2,_2,_2,_2,_2,_2,_2,_2];
_27 = [128870161161584448985858567778135629270_i128,51702968642401114245611016928287591933_i128,46604422443992362758717667184679573939_i128,90600044340206918211272214226499554732_i128,91386255406562713827482596743690528797_i128,14273046217936183268636317272242976192_i128];
_16 = &_19.0;
_18 = &(*_18);
_25 = [(-54229959581407543964418453440419068137_i128),(-163227412367437881581885619118841679492_i128)];
_24.0 = (*_16) as f32;
_24.0 = 254_u8 as f32;
Goto(bb17)
}
bb17 = {
_32.fld0 = !_12;
_31 = 16809225759810101617_u64;
_11 = _19.1;
_2 = false;
_7 = _5.0;
_5.1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_29 = _14;
_17 = _29 + _29;
_35 = _20;
_5 = (_7, _24.1);
_32 = Adt17 { fld0: _12,fld1: _9,fld2: _35,fld3: _19.0,fld4: 14182_i16,fld5: _7 };
_19.1 = [_32.fld3,(*_16),(*_16),(*_16),_19.0,(*_16)];
_18 = &(*_18);
_32.fld2 = _35 + _35;
_28 = [_21,_21,_21,_21,_21,_21,_21,_21];
_30 = [_9,_9,_9];
_19.2 = core::ptr::addr_of!(_32);
_19.2 = core::ptr::addr_of!(_32);
_5 = (_32.fld5, _24.1);
_24.0 = _1 * _1;
_18 = &(*_18);
_32.fld5 = -_5.0;
Goto(bb18)
}
bb18 = {
match _32.fld4 {
0 => bb19,
1 => bb20,
2 => bb21,
3 => bb22,
14182 => bb24,
_ => bb23
}
}
bb19 = {
_2 = _1 >= _1;
_5.0 = (-1737306658_i32);
_4 = [55_u8,177_u8,137_u8,157_u8];
_5.0 = '\u{f57a7}' as i32;
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,41_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 != _1;
_5.1 = [9223372036854775807_isize,19_isize,(-103_isize),9223372036854775807_isize,18_isize,99_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !true;
_5.1 = [(-58_isize),101_isize,9223372036854775807_isize,(-9223372036854775808_isize),124_isize,40_isize,(-26_isize),125_isize];
_2 = true;
Goto(bb3)
}
bb20 = {
_10 = [_20,_20];
_24.2 = [_2,_2,_2,_2,_2,_2,_2,_2];
_24.2 = [_2,_2,_2,_2,_2,_2,_2,_2];
_27 = [128870161161584448985858567778135629270_i128,51702968642401114245611016928287591933_i128,46604422443992362758717667184679573939_i128,90600044340206918211272214226499554732_i128,91386255406562713827482596743690528797_i128,14273046217936183268636317272242976192_i128];
_16 = &_19.0;
_18 = &(*_18);
_25 = [(-54229959581407543964418453440419068137_i128),(-163227412367437881581885619118841679492_i128)];
_24.0 = (*_16) as f32;
_24.0 = 254_u8 as f32;
Goto(bb17)
}
bb21 = {
_16 = &_19.0;
_1 = (-5131629559851951552_i64) as f32;
_10 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_17 = _14;
_2 = !false;
_4 = [153_u8,67_u8,78_u8,252_u8];
_19.1 = [(*_16),(*_16),(*_16),_19.0,(*_16),(*_16)];
_16 = &_19.0;
_16 = &(*_16);
_10 = [(-115_isize),73_isize];
_17 = _14;
_10 = [88_isize,(-9223372036854775808_isize)];
Call(_19.1 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb22 = {
_2 = _1 < _1;
_1 = 4_usize as f32;
_1 = 26086_i16 as f32;
_1 = (-9223372036854775808_isize) as f32;
_2 = false;
_1 = 2_usize as f32;
Goto(bb2)
}
bb23 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb24 = {
_32.fld1 = _9;
_29 = _14 - _17;
_5.1 = _24.1;
_34 = _14 as i128;
_14 = -_29;
_18 = &(*_18);
_14 = 20777_u16 as f64;
_32.fld5 = _7 + _5.0;
_14 = -_29;
_34 = (-3531003179000649531147362483553082583_i128);
_9 = _32.fld1;
_41 = core::ptr::addr_of_mut!(_32.fld5);
_5 = ((*_41), _24.1);
_35 = _32.fld2 * _32.fld2;
_34 = _32.fld4 as i128;
_21 = 1086140366_u32;
_2 = true;
_32.fld4 = !22403_i16;
_18 = &(*_18);
_32.fld1 = _9;
_14 = _17 * _29;
Goto(bb25)
}
bb25 = {
_39 = _9;
_19.1 = [_32.fld3,_32.fld3,_32.fld3,_32.fld3,_32.fld3,(*_16)];
_10 = [_35,_32.fld2];
_23 = _15;
_25 = [_34,_34];
_27 = [_34,_34,_34,_34,_34,_34];
_32.fld5 = _32.fld0 as i32;
_24.0 = -_1;
_26 = _19.0 == (*_16);
_20 = _35;
_32 = Adt17 { fld0: _12,fld1: _9,fld2: _20,fld3: (*_16),fld4: 14031_i16,fld5: _5.0 };
_5.0 = _32.fld5 << _31;
_29 = -_17;
_44 = _32.fld1 as isize;
_15 = [_2,_26];
_11 = [(*_16),(*_16),_19.0,(*_16),(*_16),(*_16)];
_32.fld5 = _7;
match _32.fld4 {
0 => bb4,
1 => bb9,
2 => bb26,
14031 => bb28,
_ => bb27
}
}
bb26 = {
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = 3635407212_u32 as f32;
_11 = [(-111_i8),(-109_i8),81_i8,(-77_i8),(-64_i8),(-111_i8)];
_11 = [34_i8,68_i8,72_i8,(-58_i8),(-33_i8),(-26_i8)];
_5.0 = -_7;
_2 = false;
_4 = [185_u8,207_u8,119_u8,150_u8];
_5.0 = _7;
_9 = '\u{583cb}';
_1 = 15441934374147651486_usize as f32;
Goto(bb7)
}
bb27 = {
_1 = 8651_i16 as f32;
_5.1 = [115_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),75_isize,35_isize,(-9223372036854775808_isize)];
_5.0 = _1 as i32;
_5.0 = 12626367720158535078381368251164459909_u128 as i32;
_11 = [94_i8,2_i8,121_i8,117_i8,105_i8,109_i8];
Goto(bb6)
}
bb28 = {
_30 = [_32.fld1,_39,_9];
_26 = _2 ^ _2;
_43.1 = _1 + _24.0;
_37 = core::ptr::addr_of!(_45.1);
_48 = -_34;
_40.1 = _1;
match _32.fld4 {
0 => bb1,
1 => bb17,
2 => bb3,
3 => bb12,
4 => bb5,
5 => bb22,
6 => bb7,
14031 => bb29,
_ => bb27
}
}
bb29 = {
_51 = [5982990478456510970_i64,(-7127765637952661048_i64),(-5250674900594201365_i64),(-1735497242752740001_i64),1903951084627387212_i64,(-2077989370489036442_i64)];
_30 = [_32.fld1,_9,_39];
_29 = _14;
_7 = (*_41);
_44 = _21 as isize;
(*_37) = 23_u8 as f32;
_49 = _9;
_15 = _23;
_43.1 = _24.0 * (*_37);
_52 = 2_usize as f64;
_24.1 = [_20,_44,_20,_20,_35,_20,_20,_32.fld2];
_19.1 = _11;
_15 = [_26,_26];
_19.1 = [_19.0,_19.0,(*_16),_32.fld3,(*_16),(*_16)];
_35 = _32.fld2;
(*_41) = _20 as i32;
_24.0 = _40.1 + _1;
_5 = (_32.fld5, _24.1);
_14 = _29;
Goto(bb30)
}
bb30 = {
_54 = core::ptr::addr_of!(_38);
(*_41) = _5.0;
(*_37) = _24.0 + _43.1;
_57 = _26;
_56 = !_2;
_28 = [_21,_21,_21,_21,_21,_21,_21,_21];
_28 = [_21,_21,_21,_21,_21,_21,_21,_21];
_54 = core::ptr::addr_of!((*_54));
_49 = _39;
_47 = !16673401357563971248_usize;
_32 = Adt17 { fld0: _12,fld1: _39,fld2: _20,fld3: (*_16),fld4: (-25075_i16),fld5: _5.0 };
_17 = _29;
_43.1 = _45.1 + _45.1;
_18 = &(*_18);
_35 = _20;
(*_54) = _14;
_57 = !_26;
_27 = [_34,_48,_34,_34,_48,_48];
_7 = !_32.fld5;
(*_41) = _48 as i32;
_43.1 = _24.0 * _45.1;
_14 = (*_54) * _17;
_41 = core::ptr::addr_of_mut!(_5.0);
_32.fld5 = _57 as i32;
_19.0 = !_32.fld3;
match _32.fld4 {
0 => bb10,
1 => bb31,
2 => bb32,
3 => bb33,
4 => bb34,
340282366920938463463374607431768186381 => bb36,
_ => bb35
}
}
bb31 = {
_1 = 8651_i16 as f32;
_5.1 = [115_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),75_isize,35_isize,(-9223372036854775808_isize)];
_5.0 = _1 as i32;
_5.0 = 12626367720158535078381368251164459909_u128 as i32;
_11 = [94_i8,2_i8,121_i8,117_i8,105_i8,109_i8];
Goto(bb6)
}
bb32 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb33 = {
_2 = _1 >= _1;
_5.0 = (-1737306658_i32);
_4 = [55_u8,177_u8,137_u8,157_u8];
_5.0 = '\u{f57a7}' as i32;
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,41_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 != _1;
_5.1 = [9223372036854775807_isize,19_isize,(-103_isize),9223372036854775807_isize,18_isize,99_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !true;
_5.1 = [(-58_isize),101_isize,9223372036854775807_isize,(-9223372036854775808_isize),124_isize,40_isize,(-26_isize),125_isize];
_2 = true;
Goto(bb3)
}
bb34 = {
_32.fld0 = !_12;
_31 = 16809225759810101617_u64;
_11 = _19.1;
_2 = false;
_7 = _5.0;
_5.1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_29 = _14;
_17 = _29 + _29;
_35 = _20;
_5 = (_7, _24.1);
_32 = Adt17 { fld0: _12,fld1: _9,fld2: _35,fld3: _19.0,fld4: 14182_i16,fld5: _7 };
_19.1 = [_32.fld3,(*_16),(*_16),(*_16),_19.0,(*_16)];
_18 = &(*_18);
_32.fld2 = _35 + _35;
_28 = [_21,_21,_21,_21,_21,_21,_21,_21];
_30 = [_9,_9,_9];
_19.2 = core::ptr::addr_of!(_32);
_19.2 = core::ptr::addr_of!(_32);
_5 = (_32.fld5, _24.1);
_24.0 = _1 * _1;
_18 = &(*_18);
_32.fld5 = -_5.0;
Goto(bb18)
}
bb35 = {
_16 = &_19.0;
_1 = (-5131629559851951552_i64) as f32;
_10 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_17 = _14;
_2 = !false;
_4 = [153_u8,67_u8,78_u8,252_u8];
_19.1 = [(*_16),(*_16),(*_16),_19.0,(*_16),(*_16)];
_16 = &_19.0;
_16 = &(*_16);
_10 = [(-115_isize),73_isize];
_17 = _14;
_10 = [88_isize,(-9223372036854775808_isize)];
Call(_19.1 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb36 = {
_63 = _45.1 as isize;
_10 = [_63,_35];
_20 = _63;
_47 = 2_usize;
_62 = [_19.1[_47],_11[_47],_19.0,_11[_47],_11[_47],_11[_47]];
_50 = [_32.fld4,_32.fld4,_32.fld4,_32.fld4,_32.fld4,_32.fld4];
_5 = (_7, _24.1);
_32.fld1 = _49;
_38 = _17;
match _4[_47] {
0 => bb23,
1 => bb5,
2 => bb37,
3 => bb38,
4 => bb39,
5 => bb40,
197 => bb42,
_ => bb41
}
}
bb37 = {
_2 = false;
_9 = '\u{9db4a}';
_9 = '\u{d83ac}';
_2 = _9 <= _9;
_5.1 = [9223372036854775807_isize,(-47_isize),(-11_isize),96_isize,9223372036854775807_isize,(-107_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{da5b8}';
_4 = [64_u8,170_u8,150_u8,43_u8];
_5.0 = _7;
_7 = 15166288426535030355_usize as i32;
_5.0 = _7 & _7;
_2 = !false;
_4 = [119_u8,100_u8,132_u8,43_u8];
_5.0 = _7 + _7;
_1 = (-9223372036854775808_isize) as f32;
_5.1 = [(-54_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,120_isize,9223372036854775807_isize,26_isize];
_1 = 185630275521473572246971821696549558823_u128 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{20522}';
_2 = !true;
_7 = 22375_u16 as i32;
_5.0 = -_7;
_10 = [(-9223372036854775808_isize),25_isize];
Goto(bb5)
}
bb38 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb39 = {
_30 = [_32.fld1,_39,_9];
_26 = _2 ^ _2;
_43.1 = _1 + _24.0;
_37 = core::ptr::addr_of!(_45.1);
_48 = -_34;
_40.1 = _1;
match _32.fld4 {
0 => bb1,
1 => bb17,
2 => bb3,
3 => bb12,
4 => bb5,
5 => bb22,
6 => bb7,
14031 => bb29,
_ => bb27
}
}
bb40 = {
_2 = _1 >= _1;
_5.0 = (-1737306658_i32);
_4 = [55_u8,177_u8,137_u8,157_u8];
_5.0 = '\u{f57a7}' as i32;
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,41_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 != _1;
_5.1 = [9223372036854775807_isize,19_isize,(-103_isize),9223372036854775807_isize,18_isize,99_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !true;
_5.1 = [(-58_isize),101_isize,9223372036854775807_isize,(-9223372036854775808_isize),124_isize,40_isize,(-26_isize),125_isize];
_2 = true;
Goto(bb3)
}
bb41 = {
_1 = 8651_i16 as f32;
_5.1 = [115_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),75_isize,35_isize,(-9223372036854775808_isize)];
_5.0 = _1 as i32;
_5.0 = 12626367720158535078381368251164459909_u128 as i32;
_11 = [94_i8,2_i8,121_i8,117_i8,105_i8,109_i8];
Goto(bb6)
}
bb42 = {
_5.1 = [_20,_20,_32.fld2,_35,_24.1[_47],_20,_20,_63];
_51[_47] = 3325211277469088448_i64 & (-5449846656287550199_i64);
_62[_47] = _7 as i8;
_19.0 = _11[_47] + _19.1[_47];
_11 = _62;
_52 = _14 * _29;
(*_54) = _14;
_37 = core::ptr::addr_of!((*_37));
Call(_32.fld2 = core::intrinsics::bswap(_20), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_10 = [_63,_63];
_31 = 4675043617199038855_u64;
_50 = [_32.fld4,_32.fld4,_32.fld4,_32.fld4,_32.fld4,_32.fld4];
_58 = Move(_19.2);
_19.1 = _11;
_62 = _19.1;
_63 = _19.1[_47] as isize;
_50 = [_32.fld4,_32.fld4,_32.fld4,_32.fld4,_32.fld4,_32.fld4];
_59 = (*_54) * _29;
_63 = _32.fld5 as isize;
_19.2 = Move(_58);
_37 = core::ptr::addr_of!(_55);
_51[_47] = !7185504917881215828_i64;
_62[_47] = _11[_47];
_4 = [202_u8,208_u8,17_u8,51_u8];
_35 = !_24.1[_47];
_60 = _59 + (*_54);
_42 = !_5.1[_47];
(*_54) = _12 as f64;
_14 = _29;
_65 = !41314_u16;
_58 = Move(_19.2);
_19.0 = _11[_47] + _62[_47];
_70 = _44 << _4[_47];
_24.2[_47] = !_57;
match _50[_47] {
0 => bb44,
1 => bb45,
2 => bb46,
3 => bb47,
4 => bb48,
340282366920938463463374607431768186381 => bb50,
_ => bb49
}
}
bb44 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb45 = {
_1 = 8651_i16 as f32;
_5.1 = [115_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),75_isize,35_isize,(-9223372036854775808_isize)];
_5.0 = _1 as i32;
_5.0 = 12626367720158535078381368251164459909_u128 as i32;
_11 = [94_i8,2_i8,121_i8,117_i8,105_i8,109_i8];
Goto(bb6)
}
bb46 = {
_32.fld0 = !_12;
_31 = 16809225759810101617_u64;
_11 = _19.1;
_2 = false;
_7 = _5.0;
_5.1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_29 = _14;
_17 = _29 + _29;
_35 = _20;
_5 = (_7, _24.1);
_32 = Adt17 { fld0: _12,fld1: _9,fld2: _35,fld3: _19.0,fld4: 14182_i16,fld5: _7 };
_19.1 = [_32.fld3,(*_16),(*_16),(*_16),_19.0,(*_16)];
_18 = &(*_18);
_32.fld2 = _35 + _35;
_28 = [_21,_21,_21,_21,_21,_21,_21,_21];
_30 = [_9,_9,_9];
_19.2 = core::ptr::addr_of!(_32);
_19.2 = core::ptr::addr_of!(_32);
_5 = (_32.fld5, _24.1);
_24.0 = _1 * _1;
_18 = &(*_18);
_32.fld5 = -_5.0;
Goto(bb18)
}
bb47 = {
_39 = _9;
_19.1 = [_32.fld3,_32.fld3,_32.fld3,_32.fld3,_32.fld3,(*_16)];
_10 = [_35,_32.fld2];
_23 = _15;
_25 = [_34,_34];
_27 = [_34,_34,_34,_34,_34,_34];
_32.fld5 = _32.fld0 as i32;
_24.0 = -_1;
_26 = _19.0 == (*_16);
_20 = _35;
_32 = Adt17 { fld0: _12,fld1: _9,fld2: _20,fld3: (*_16),fld4: 14031_i16,fld5: _5.0 };
_5.0 = _32.fld5 << _31;
_29 = -_17;
_44 = _32.fld1 as isize;
_15 = [_2,_26];
_11 = [(*_16),(*_16),_19.0,(*_16),(*_16),(*_16)];
_32.fld5 = _7;
match _32.fld4 {
0 => bb4,
1 => bb9,
2 => bb26,
14031 => bb28,
_ => bb27
}
}
bb48 = {
_11 = [123_i8,98_i8,(-120_i8),(-87_i8),10_i8,(-67_i8)];
_14 = _1 as f64;
_15 = [_2,_2];
_5.1 = [(-9223372036854775808_isize),(-123_isize),(-114_isize),9223372036854775807_isize,40_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = -_7;
_2 = true;
_7 = _5.0;
_5.0 = _7 ^ _7;
_15 = [_2,_2];
_10 = [(-11_isize),(-5_isize)];
_1 = _14 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = _12 as i32;
_4 = [78_u8,252_u8,80_u8,140_u8];
_12 = 9721080462697136796_usize as u128;
_11 = [68_i8,4_i8,(-103_i8),(-61_i8),(-69_i8),(-12_i8)];
_10 = [(-9223372036854775808_isize),9223372036854775807_isize];
_9 = '\u{5a0a8}';
_5.1 = [(-118_isize),(-9223372036854775808_isize),124_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_10 = [9223372036854775807_isize,(-24_isize)];
_5.1 = [(-9223372036854775808_isize),9223372036854775807_isize,91_isize,(-9223372036854775808_isize),55_isize,9223372036854775807_isize,(-101_isize),(-9223372036854775808_isize)];
_15 = [_2,_2];
_18 = &_8;
_11 = [39_i8,(-86_i8),88_i8,32_i8,115_i8,90_i8];
_15 = [_2,_2];
_11 = [58_i8,(-90_i8),(-92_i8),(-5_i8),82_i8,77_i8];
_19.0 = 55028363210224183450916726214470113962_i128 as i8;
_12 = 87590620053833035746111840147055625645_u128 ^ 323233781466478999884686607315139738550_u128;
Goto(bb10)
}
bb49 = {
_16 = &_19.0;
_1 = (-5131629559851951552_i64) as f32;
_10 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_17 = _14;
_2 = !false;
_4 = [153_u8,67_u8,78_u8,252_u8];
_19.1 = [(*_16),(*_16),(*_16),_19.0,(*_16),(*_16)];
_16 = &_19.0;
_16 = &(*_16);
_10 = [(-115_isize),73_isize];
_17 = _14;
_10 = [88_isize,(-9223372036854775808_isize)];
Call(_19.1 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb50 = {
(*_37) = _47 as f32;
_9 = _39;
_19.2 = Move(_58);
_56 = !_57;
match _50[_47] {
0 => bb47,
1 => bb40,
2 => bb48,
3 => bb4,
4 => bb51,
5 => bb52,
6 => bb53,
340282366920938463463374607431768186381 => bb55,
_ => bb54
}
}
bb51 = {
_2 = _1 >= _1;
_5.0 = (-1737306658_i32);
_4 = [55_u8,177_u8,137_u8,157_u8];
_5.0 = '\u{f57a7}' as i32;
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,41_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 != _1;
_5.1 = [9223372036854775807_isize,19_isize,(-103_isize),9223372036854775807_isize,18_isize,99_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !true;
_5.1 = [(-58_isize),101_isize,9223372036854775807_isize,(-9223372036854775808_isize),124_isize,40_isize,(-26_isize),125_isize];
_2 = true;
Goto(bb3)
}
bb52 = {
_2 = _1 >= _1;
_5.0 = (-1737306658_i32);
_4 = [55_u8,177_u8,137_u8,157_u8];
_5.0 = '\u{f57a7}' as i32;
_5.1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,41_isize,124_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1 != _1;
_5.1 = [9223372036854775807_isize,19_isize,(-103_isize),9223372036854775807_isize,18_isize,99_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !true;
_5.1 = [(-58_isize),101_isize,9223372036854775807_isize,(-9223372036854775808_isize),124_isize,40_isize,(-26_isize),125_isize];
_2 = true;
Goto(bb3)
}
bb53 = {
_2 = false;
_9 = '\u{9db4a}';
_9 = '\u{d83ac}';
_2 = _9 <= _9;
_5.1 = [9223372036854775807_isize,(-47_isize),(-11_isize),96_isize,9223372036854775807_isize,(-107_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{da5b8}';
_4 = [64_u8,170_u8,150_u8,43_u8];
_5.0 = _7;
_7 = 15166288426535030355_usize as i32;
_5.0 = _7 & _7;
_2 = !false;
_4 = [119_u8,100_u8,132_u8,43_u8];
_5.0 = _7 + _7;
_1 = (-9223372036854775808_isize) as f32;
_5.1 = [(-54_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,120_isize,9223372036854775807_isize,26_isize];
_1 = 185630275521473572246971821696549558823_u128 as f32;
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = '\u{20522}';
_2 = !true;
_7 = 22375_u16 as i32;
_5.0 = -_7;
_10 = [(-9223372036854775808_isize),25_isize];
Goto(bb5)
}
bb54 = {
_51 = [5982990478456510970_i64,(-7127765637952661048_i64),(-5250674900594201365_i64),(-1735497242752740001_i64),1903951084627387212_i64,(-2077989370489036442_i64)];
_30 = [_32.fld1,_9,_39];
_29 = _14;
_7 = (*_41);
_44 = _21 as isize;
(*_37) = 23_u8 as f32;
_49 = _9;
_15 = _23;
_43.1 = _24.0 * (*_37);
_52 = 2_usize as f64;
_24.1 = [_20,_44,_20,_20,_35,_20,_20,_32.fld2];
_19.1 = _11;
_15 = [_26,_26];
_19.1 = [_19.0,_19.0,(*_16),_32.fld3,(*_16),(*_16)];
_35 = _32.fld2;
(*_41) = _20 as i32;
_24.0 = _40.1 + _1;
_5 = (_32.fld5, _24.1);
_14 = _29;
Goto(bb30)
}
bb55 = {
_33 = &_69;
_31 = _35 as u64;
_54 = core::ptr::addr_of!(_17);
(*_41) = !_32.fld5;
_33 = &(*_33);
_71 = Move(_41);
_68 = _42 & _35;
_12 = _32.fld0 % _32.fld0;
_73 = !_20;
_17 = -_59;
_9 = _32.fld1;
_72 = &_46;
match _50[_47] {
0 => bb42,
1 => bb2,
2 => bb11,
3 => bb20,
4 => bb14,
5 => bb46,
340282366920938463463374607431768186381 => bb56,
_ => bb43
}
}
bb56 = {
_24.2 = [_56,_56,_26,_57,_26,_57,_57,_26];
_66 = _30[_47];
_32.fld2 = _63 | _70;
_50[_47] = _4[_47] as i16;
_73 = -_5.1[_47];
_24.0 = _43.1;
_5.1[_47] = _42 + _32.fld2;
_75 = &_27;
_24.2 = [_56,_57,_56,_56,_26,_26,_57,_56];
(*_54) = _43.1 as f64;
_35 = _19.0 as isize;
_41 = Move(_71);
_66 = _49;
RET = core::ptr::addr_of!(_75);
_32.fld0 = _12 * _12;
_77.0 = _5.0;
_8 = &_4[_47];
_64 = _30[_47];
_58 = core::ptr::addr_of!(_32);
_11 = [_19.1[_47],_19.0,_19.0,_19.0,(*_58).fld3,_62[_47]];
(*_58).fld3 = -_19.0;
_72 = &(*_72);
_32.fld3 = !_19.0;
Goto(bb57)
}
bb57 = {
Call(_79 = dump_var(17_usize, 21_usize, Move(_21), 39_usize, Move(_39), 64_usize, Move(_64), 49_usize, Move(_49)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_79 = dump_var(17_usize, 4_usize, Move(_4), 2_usize, Move(_2), 27_usize, Move(_27), 42_usize, Move(_42)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_79 = dump_var(17_usize, 10_usize, Move(_10), 5_usize, Move(_5), 25_usize, Move(_25), 57_usize, Move(_57)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_79 = dump_var(17_usize, 68_usize, Move(_68), 44_usize, Move(_44), 65_usize, Move(_65), 66_usize, Move(_66)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_79 = dump_var(17_usize, 51_usize, Move(_51), 11_usize, Move(_11), 48_usize, Move(_48), 80_usize, _80), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: usize,mut _3: [char; 3],mut _4: isize,mut _5: f64,mut _6: char,mut _7: f64,mut _8: i8,mut _9: isize,mut _10: i16) -> [i32; 6] {
mir! {
type RET = [i32; 6];
let _11: char;
let _12: &'static i8;
let _13: [bool; 2];
let _14: bool;
let _15: *const Adt17;
let _16: f32;
let _17: *const Adt63;
let _18: f64;
let _19: ([i64; 6],);
let _20: Adt63;
let _21: isize;
let _22: usize;
let _23: &'static [i64; 6];
let _24: f64;
let _25: [usize; 5];
let _26: char;
let _27: *const (u32, *const Adt26, *mut i32, u128);
let _28: ([i64; 6],);
let _29: [i32; 6];
let _30: *const [u16; 4];
let _31: char;
let _32: *const f32;
let _33: Adt38;
let _34: char;
let _35: &'static [i64; 3];
let _36: [isize; 2];
let _37: u16;
let _38: (Adt38,);
let _39: isize;
let _40: [bool; 8];
let _41: ();
let _42: ();
{
_9 = _1;
_1 = _4 + _4;
RET = [(-38678122_i32),(-2073556332_i32),(-2004561098_i32),(-958300282_i32),(-1969437195_i32),1243997893_i32];
_1 = !_9;
_1 = _2 as isize;
_12 = &_8;
Goto(bb1)
}
bb1 = {
_11 = _6;
RET = [1388488578_i32,1205534411_i32,766171509_i32,(-394941112_i32),(-1949736687_i32),(-1786675716_i32)];
Goto(bb2)
}
bb2 = {
_12 = &(*_12);
_2 = 0_usize >> _1;
_13 = [false,true];
_2 = (-2050145798_i32) as usize;
_13 = [true,false];
RET = [303878213_i32,12625300_i32,1533767047_i32,2102108942_i32,(-933820878_i32),(-1548833405_i32)];
Goto(bb3)
}
bb3 = {
_5 = -_7;
_3 = [_11,_11,_11];
_12 = &(*_12);
_14 = _6 < _6;
_9 = _1 | _1;
_13 = [_14,_14];
_11 = _6;
_13 = [_14,_14];
_12 = &(*_12);
_2 = 2_usize ^ 10244440828833363835_usize;
_12 = &(*_12);
_1 = _9;
_2 = !1_usize;
_6 = _11;
_10 = 20396_i16 - 27433_i16;
_6 = _11;
_7 = _5 * _5;
_16 = _4 as f32;
_8 = !(-66_i8);
RET = [934764596_i32,(-1686174332_i32),(-932464533_i32),(-717795410_i32),(-1125464308_i32),568010278_i32];
_1 = _7 as isize;
Goto(bb4)
}
bb4 = {
_16 = 10356889299491282320_u64 as f32;
_10 = 120008785354374167831261923908650997291_i128 as i16;
_5 = -_7;
_9 = _11 as isize;
_18 = -_5;
RET = [234870534_i32,(-1167693875_i32),2114353365_i32,(-873432166_i32),(-1874665235_i32),1473302125_i32];
_9 = (-3214678763803527484_i64) as isize;
_12 = &_8;
_14 = false ^ false;
_7 = -_5;
_14 = false ^ true;
_22 = _2 - _2;
_5 = _18 * _7;
_5 = _18 - _7;
_22 = _18 as usize;
_17 = core::ptr::addr_of!(_20);
Call(_10 = core::intrinsics::bswap((-27250_i16)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = !_22;
_3 = [_6,_11,_11];
_17 = core::ptr::addr_of!(_20);
_6 = _11;
_8 = !46_i8;
_3 = [_11,_6,_6];
_13 = [_14,_14];
_18 = _1 as f64;
_5 = _7;
_18 = _5;
_14 = !false;
_7 = _8 as f64;
_18 = (-7439650827312085012_i64) as f64;
_1 = 5635069807391910425_i64 as isize;
_14 = true;
_21 = _9 * _9;
_8 = 1868509655_i32 as i8;
_2 = _22 - _22;
_11 = _6;
_13 = [_14,_14];
_16 = _2 as f32;
_10 = 30765_i16 - (-31812_i16);
_26 = _11;
_13 = [_14,_14];
_19.0 = [1490920872340519048_i64,(-5692590452234389397_i64),(-4460654404521233018_i64),(-5837571749113726676_i64),8498683535461275392_i64,(-8267006098229782283_i64)];
_1 = _22 as isize;
_5 = _7;
Goto(bb6)
}
bb6 = {
_3 = [_11,_11,_6];
_5 = _7;
_2 = !_22;
RET = [1267885574_i32,125030975_i32,302080843_i32,411631070_i32,2029362859_i32,(-448040777_i32)];
_12 = &_8;
_3 = [_6,_11,_11];
_24 = _7;
_21 = _1 & _9;
_28.0 = _19.0;
_7 = -_5;
_2 = !_22;
RET = [1578752341_i32,(-384148082_i32),538233665_i32,(-587588982_i32),1687287757_i32,48315063_i32];
_11 = _6;
Goto(bb7)
}
bb7 = {
_18 = 90_u8 as f64;
_22 = !_2;
_19.0 = [(-5612197567475886131_i64),8058646932239185676_i64,(-6051326739128900041_i64),(-1222612199829670909_i64),(-3925570778863777421_i64),4216081894929931179_i64];
_2 = _22;
_13 = [_14,_14];
_3 = [_26,_6,_6];
Goto(bb8)
}
bb8 = {
RET = [596364963_i32,(-268894948_i32),495919269_i32,507785837_i32,1857655191_i32,(-94351861_i32)];
Goto(bb9)
}
bb9 = {
_11 = _6;
_10 = 25483_i16;
_23 = &_19.0;
_5 = (*_12) as f64;
_3 = [_11,_11,_6];
_31 = _11;
_21 = _1 << _1;
_25 = [_22,_2,_2,_2,_2];
_19.0 = [(-920700350670989463_i64),(-5242427886952546030_i64),7675825209356657154_i64,(-5207904214250229755_i64),(-5408991137741673111_i64),(-3591524750158175024_i64)];
_4 = 131556334267102922654544679641079784180_i128 as isize;
_29 = [(-1631425276_i32),1726405652_i32,814028573_i32,454816100_i32,514692066_i32,(-312838885_i32)];
_5 = _18 - _18;
_33.fld3 = 8589131694902038122_i64 as i8;
_33.fld2.1 = 221_u8;
_28.0 = [6417900113923447566_i64,8344073853354367964_i64,(-9150918979075654662_i64),1208022228901444298_i64,6434611948130899742_i64,5766093153636177687_i64];
_33.fld0 = [_10,_10,_10,_10,_10,_10];
_13 = [_14,_14];
_10 = 227857325614978482029061971671163328837_u128 as i16;
_33.fld1.1 = [(*_12),(*_12),(*_12),(*_12),(*_12),_33.fld3];
_31 = _6;
RET = [(-813053334_i32),(-290820891_i32),2061390069_i32,(-1690935986_i32),(-969103644_i32),(-377553682_i32)];
_33.fld1.0 = !(*_12);
_11 = _31;
match _33.fld2.1 {
0 => bb1,
1 => bb3,
2 => bb10,
221 => bb12,
_ => bb11
}
}
bb10 = {
RET = [596364963_i32,(-268894948_i32),495919269_i32,507785837_i32,1857655191_i32,(-94351861_i32)];
Goto(bb9)
}
bb11 = {
_12 = &(*_12);
_2 = 0_usize >> _1;
_13 = [false,true];
_2 = (-2050145798_i32) as usize;
_13 = [true,false];
RET = [303878213_i32,12625300_i32,1533767047_i32,2102108942_i32,(-933820878_i32),(-1548833405_i32)];
Goto(bb3)
}
bb12 = {
_4 = _21;
_14 = !true;
_29 = [216334368_i32,1573787504_i32,(-1537384504_i32),(-1024676851_i32),(-1629430144_i32),(-82910622_i32)];
_21 = _4;
_22 = !_2;
_16 = _10 as f32;
_14 = !false;
_26 = _11;
_36 = [_21,_21];
_8 = _33.fld3;
Call(_8 = core::intrinsics::bswap(_33.fld1.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_12 = &_33.fld1.0;
_23 = &_28.0;
match _33.fld2.1 {
0 => bb1,
1 => bb14,
221 => bb16,
_ => bb15
}
}
bb14 = {
_11 = _6;
_10 = 25483_i16;
_23 = &_19.0;
_5 = (*_12) as f64;
_3 = [_11,_11,_6];
_31 = _11;
_21 = _1 << _1;
_25 = [_22,_2,_2,_2,_2];
_19.0 = [(-920700350670989463_i64),(-5242427886952546030_i64),7675825209356657154_i64,(-5207904214250229755_i64),(-5408991137741673111_i64),(-3591524750158175024_i64)];
_4 = 131556334267102922654544679641079784180_i128 as isize;
_29 = [(-1631425276_i32),1726405652_i32,814028573_i32,454816100_i32,514692066_i32,(-312838885_i32)];
_5 = _18 - _18;
_33.fld3 = 8589131694902038122_i64 as i8;
_33.fld2.1 = 221_u8;
_28.0 = [6417900113923447566_i64,8344073853354367964_i64,(-9150918979075654662_i64),1208022228901444298_i64,6434611948130899742_i64,5766093153636177687_i64];
_33.fld0 = [_10,_10,_10,_10,_10,_10];
_13 = [_14,_14];
_10 = 227857325614978482029061971671163328837_u128 as i16;
_33.fld1.1 = [(*_12),(*_12),(*_12),(*_12),(*_12),_33.fld3];
_31 = _6;
RET = [(-813053334_i32),(-290820891_i32),2061390069_i32,(-1690935986_i32),(-969103644_i32),(-377553682_i32)];
_33.fld1.0 = !(*_12);
_11 = _31;
match _33.fld2.1 {
0 => bb1,
1 => bb3,
2 => bb10,
221 => bb12,
_ => bb11
}
}
bb15 = {
_11 = _6;
RET = [1388488578_i32,1205534411_i32,766171509_i32,(-394941112_i32),(-1949736687_i32),(-1786675716_i32)];
Goto(bb2)
}
bb16 = {
_14 = _18 >= _18;
_6 = _26;
_31 = _11;
_10 = (-2069841721706432391_i64) as i16;
_21 = _4;
_37 = 48539408835078220022538628417908601520_i128 as u16;
_33.fld3 = _33.fld1.0 << _4;
_14 = false;
_33.fld1.0 = -_8;
_5 = 7077985585350520057_i64 as f64;
_12 = &_38.0.fld1.0;
_22 = !_2;
_23 = &_19.0;
_33.fld3 = -_8;
_19.0 = [1297866412926369823_i64,5530147046324352858_i64,2230222405530171121_i64,(-4896593859233024805_i64),(-7047141559018093303_i64),(-8906839163868031837_i64)];
_1 = _21;
_17 = core::ptr::addr_of!((*_17));
_38.0.fld1.0 = _33.fld3 & _33.fld1.0;
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(18_usize, 28_usize, Move(_28), 29_usize, Move(_29), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(18_usize, 31_usize, Move(_31), 36_usize, Move(_36), 9_usize, Move(_9), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(18_usize, 11_usize, Move(_11), 1_usize, Move(_1), 42_usize, _42, 42_usize, _42), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{40800}'), std::hint::black_box(13686154633398635105_usize), std::hint::black_box((-104_i8)), std::hint::black_box((-9103_i16)), std::hint::black_box(292191761_i32), std::hint::black_box((-124547159119144347485346898114518117909_i128)));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt17{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt17 {
fld0: u128,
fld1: char,
fld2: isize,
fld3: i8,
fld4: i16,
fld5: i32,
}
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: u32,
fld1: f64,
fld2: (i32, Adt17, u64),
fld3: u128,
fld4: Adt17,
fld5: [i128; 2],
fld6: i64,

},
Variant1{
fld0: u64,
fld1: (i32, Adt17, u64),
fld2: i32,

},
Variant2{
fld0: bool,
fld1: i32,
fld2: Adt17,
fld3: usize,
fld4: (i32, Adt17, u64),

},
Variant3{
fld0: u8,
fld1: [i128; 2],
fld2: (i16, usize, u32),
fld3: u128,
fld4: u64,
fld5: i32,
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt26{
	unsafe fn printf_debug(&self){unsafe{printf("Adt26::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt26 {
Variant0{
fld0: bool,
fld1: u8,
fld2: (i16, usize, u32),
fld3: i8,
fld4: Adt17,

},
Variant1{
fld0: Adt22,
fld1: (i32, Adt17, u64),
fld2: *const Adt17,
fld3: u16,
fld4: i16,
fld5: u8,
fld6: u128,
fld7: [u32; 2],

},
Variant2{
fld0: u128,

},
Variant3{
fld0: f32,
fld1: (i16, usize, u32),
fld2: isize,
fld3: [u32; 2],
fld4: u128,
fld5: i32,
fld6: *const Adt17,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: (i16, usize, u32),
fld1: Adt26,

},
Variant1{
fld0: usize,
fld1: [i16; 6],
fld2: u64,
fld3: i8,
fld4: (i16, usize, u32),
fld5: i32,
fld6: u8,
fld7: Adt17,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: [i16; 6],
fld1: (i8, [i8; 6], *const Adt17, Adt22),
fld2: (Adt31, u8),
fld3: i8,
fld4: *const Adt26,
}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: Adt26,
fld1: char,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: u16,
fld1: usize,
fld2: isize,
fld3: i8,
fld4: *mut Adt38,
fld5: i32,
fld6: *const Adt17,
fld7: Adt17,
}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: (i32, Adt17, u64),
fld1: *const Adt26,
fld2: (i8, [i8; 6], *const Adt17, Adt22),
fld3: Adt17,

},
Variant1{
fld0: u32,
fld1: (*const Adt26, [i128; 6], [i8; 6], [u32; 2]),
fld2: *const Adt26,
fld3: Adt53,
fld4: Adt31,
fld5: Adt22,
fld6: *const f32,
fld7: (u32, *const Adt26, *mut i32, u128),

},
Variant2{
fld0: usize,
fld1: Adt17,
fld2: isize,
fld3: i8,
fld4: [u16; 1],
fld5: Adt40,
fld6: *const Adt26,

},
Variant3{
fld0: [i8; 6],
fld1: u8,
fld2: (Adt31, u8),
fld3: [bool; 8],
fld4: Adt26,
fld5: i32,
fld6: *const Adt26,
fld7: i128,

}}

