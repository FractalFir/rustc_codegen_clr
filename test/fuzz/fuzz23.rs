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
pub fn fn0(mut _1: i128,mut _2: usize,mut _3: u8,mut _4: i16) -> i8 {
mir! {
type RET = i8;
let _5: &'static *mut i64;
let _6: [u32; 4];
let _7: bool;
let _8: f32;
let _9: ((i16, usize),);
let _10: usize;
let _11: &'static &'static i16;
let _12: bool;
let _13: Adt46;
let _14: f64;
let _15: &'static *mut *mut u8;
let _16: (*mut i128, (&'static i16,), char);
let _17: *const *mut i128;
let _18: Adt73;
let _19: i32;
let _20: u16;
let _21: u32;
let _22: *mut ([char; 7], *mut u8);
let _23: ((i16, usize),);
let _24: &'static bool;
let _25: f32;
let _26: Adt24;
let _27: u32;
let _28: f32;
let _29: f32;
let _30: i64;
let _31: isize;
let _32: Adt60;
let _33: i16;
let _34: isize;
let _35: Adt60;
let _36: &'static (*mut i128, (&'static i16,), char);
let _37: Adt24;
let _38: *mut u8;
let _39: isize;
let _40: isize;
let _41: &'static *const usize;
let _42: &'static (*mut i128, (&'static i16,), char);
let _43: ();
let _44: ();
{
_1 = (-142773245257915745737091771527994650452_i128);
_2 = !5_usize;
_3 = 243_u8 * 103_u8;
_4 = 7421_i16 - 1013_i16;
RET = 121_i8 << _3;
RET = 77_i8 - (-87_i8);
_3 = 94_u8;
_3 = 427472562_i32 as u8;
_4 = !(-8030_i16);
RET = 31_i8;
_6 = [2623117114_u32,2280056733_u32,214877406_u32,3643575263_u32];
_1 = !(-39343415281269521744569745172973507822_i128);
_1 = (-135374535334805700165523819606521973171_i128) ^ (-66145064441605730163162987137267105023_i128);
RET = !(-25_i8);
_1 = 56745408549442345764644200132879944090_i128;
_1 = (-78876993267709645575410160588709632924_i128);
_2 = 5_usize;
RET = 64_i8;
_7 = _3 < _3;
_4 = RET as i16;
RET = _2 as i8;
_9.0 = (_4, _2);
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.0.1 = _2;
_6 = [2729451194_u32,1286591907_u32,777137567_u32,4128944865_u32];
_3 = !36_u8;
_7 = !true;
_9.0.1 = 23756_u16 as usize;
_3 = !75_u8;
_3 = 32_u8;
_9.0.0 = _4 - _4;
_9.0.0 = -_4;
_8 = _3 as f32;
_9.0.1 = !_2;
_4 = !_9.0.0;
_1 = 134358271833364427104049675428431094620_i128 >> RET;
_4 = _9.0.0;
_7 = RET >= RET;
_4 = !_9.0.0;
_12 = _7;
_12 = _8 <= _8;
_12 = _7;
_7 = _12 >= _12;
RET = (-4_i8);
RET = 96_i8 << _3;
Goto(bb2)
}
bb2 = {
_16.1.0 = &_9.0.0;
_10 = 10025_u16 as usize;
RET = 118_i8 >> _2;
_7 = _12;
_16.2 = '\u{66aa1}';
_16.1.0 = &_9.0.0;
_10 = _2;
Call(_3 = fn1(_7, _9, _12, _16.2, _9, RET, _6, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _2 ^ _9.0.1;
_11 = &_16.1.0;
_16.0 = core::ptr::addr_of_mut!(_1);
_17 = core::ptr::addr_of!(_16.0);
_20 = !26403_u16;
_9.0.0 = _4;
_21 = !2888596436_u32;
_1 = (-53699924249094279296831472265514122718_i128);
match _2 {
0 => bb1,
1 => bb4,
2 => bb5,
5 => bb7,
_ => bb6
}
}
bb4 = {
_16.1.0 = &_9.0.0;
_10 = 10025_u16 as usize;
RET = 118_i8 >> _2;
_7 = _12;
_16.2 = '\u{66aa1}';
_16.1.0 = &_9.0.0;
_10 = _2;
Call(_3 = fn1(_7, _9, _12, _16.2, _9, RET, _6, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_9.0.1 = _2;
_6 = [2729451194_u32,1286591907_u32,777137567_u32,4128944865_u32];
_3 = !36_u8;
_7 = !true;
_9.0.1 = 23756_u16 as usize;
_3 = !75_u8;
_3 = 32_u8;
_9.0.0 = _4 - _4;
_9.0.0 = -_4;
_8 = _3 as f32;
_9.0.1 = !_2;
_4 = !_9.0.0;
_1 = 134358271833364427104049675428431094620_i128 >> RET;
_4 = _9.0.0;
_7 = RET >= RET;
_4 = !_9.0.0;
_12 = _7;
_12 = _8 <= _8;
_12 = _7;
_7 = _12 >= _12;
RET = (-4_i8);
RET = 96_i8 << _3;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_16.2 = '\u{2ee48}';
match _3 {
0 => bb6,
141 => bb9,
_ => bb8
}
}
bb8 = {
_16.1.0 = &_9.0.0;
_10 = 10025_u16 as usize;
RET = 118_i8 >> _2;
_7 = _12;
_16.2 = '\u{66aa1}';
_16.1.0 = &_9.0.0;
_10 = _2;
Call(_3 = fn1(_7, _9, _12, _16.2, _9, RET, _6, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_9.0.0 = -_4;
_11 = &(*_11);
_16.0 = core::ptr::addr_of_mut!(_1);
_11 = &(*_11);
_10 = !_2;
_19 = (-555717301_i32) ^ 1820368165_i32;
_11 = &(*_11);
_24 = &_7;
_7 = _12;
_6 = [_21,_21,_21,_21];
(*_17) = core::ptr::addr_of_mut!(_1);
RET = 16302554260943492215_u64 as i8;
_23.0.0 = _4;
(*_17) = core::ptr::addr_of_mut!(_1);
_10 = 9223372036854775807_isize as usize;
_27 = !_21;
_17 = core::ptr::addr_of!(_16.0);
_23.0.1 = !_2;
_2 = 289762743185734385489222176676345804704_u128 as usize;
_23.0.1 = _9.0.1;
_25 = -_8;
_16.1.0 = &_23.0.0;
Goto(bb10)
}
bb10 = {
_11 = &_16.1.0;
_24 = &_12;
_7 = _12 ^ _12;
_14 = _4 as f64;
_10 = _9.0.1;
_2 = _7 as usize;
_2 = _23.0.1 - _23.0.1;
_23.0.1 = !_2;
_16.2 = '\u{9ce0c}';
_1 = (-116675537285903675916883015995012892096_i128) * 58898377859755932246703402304651846114_i128;
_24 = &_7;
_23.0.1 = _2 << _3;
Goto(bb11)
}
bb11 = {
_28 = _20 as f32;
_17 = core::ptr::addr_of!((*_17));
_4 = _23.0.0 - _23.0.0;
_24 = &(*_24);
_8 = 218894508927087774057333379827454701985_u128 as f32;
_7 = !_12;
_31 = (-9223372036854775808_isize);
_23 = (_9.0,);
_2 = _9.0.1 * _10;
_30 = _2 as i64;
_20 = !37955_u16;
_9.0.0 = _4;
_23.0.0 = _9.0.0 << _19;
RET = 44_i8 * 68_i8;
_16.0 = core::ptr::addr_of_mut!(_1);
_14 = _20 as f64;
(*_17) = core::ptr::addr_of_mut!(_1);
_21 = 137437101813494357098585009605645583668_u128 as u32;
_25 = _28 + _28;
_4 = !_23.0.0;
Goto(bb12)
}
bb12 = {
_9.0.1 = _10 << _9.0.0;
(*_17) = core::ptr::addr_of_mut!(_1);
_9.0 = _23.0;
_12 = !_7;
_26 = Adt24 { fld0: _19 };
_14 = _19 as f64;
_23.0 = (_9.0.0, _9.0.1);
RET = 190328584570457628916170782675678715117_u128 as i8;
_17 = core::ptr::addr_of!((*_17));
_26 = Adt24 { fld0: _19 };
_16.0 = core::ptr::addr_of_mut!(_1);
Goto(bb13)
}
bb13 = {
_8 = _27 as f32;
_34 = _31;
_23.0.1 = _2 * _10;
_33 = _30 as i16;
_16.1.0 = &_9.0.0;
_30 = 7172305491507426238_i64 * (-5401319653978696644_i64);
(*_17) = core::ptr::addr_of_mut!(_1);
_11 = &_16.1.0;
_12 = _7;
_31 = -_34;
_37.fld0 = -_26.fld0;
_9 = (_23.0,);
_14 = _23.0.1 as f64;
_14 = _27 as f64;
_19 = !_37.fld0;
_14 = _1 as f64;
_23.0.1 = _2;
_17 = core::ptr::addr_of!((*_17));
_4 = _23.0.0;
_10 = _23.0.1;
_38 = core::ptr::addr_of_mut!(_3);
_4 = _9.0.0 ^ _9.0.0;
(*_17) = core::ptr::addr_of_mut!(_1);
match _34 {
0 => bb9,
1 => bb12,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb14 = {
_16.2 = '\u{2ee48}';
match _3 {
0 => bb6,
141 => bb9,
_ => bb8
}
}
bb15 = {
_30 = -4510293829406881806_i64;
_16.1.0 = &_9.0.0;
_30 = _10 as i64;
_37 = Move(_26);
_25 = _28 - _28;
_23 = (_9.0,);
_26 = Adt24 { fld0: _19 };
_10 = _23.0.1 * _23.0.1;
_34 = _30 as isize;
_24 = &_7;
_34 = _31 << _4;
Goto(bb16)
}
bb16 = {
Call(_43 = dump_var(0_usize, 7_usize, Move(_7), 4_usize, Move(_4), 6_usize, Move(_6), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(0_usize, 30_usize, Move(_30), 34_usize, Move(_34), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(0_usize, 2_usize, Move(_2), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: ((i16, usize),),mut _3: bool,mut _4: char,mut _5: ((i16, usize),),mut _6: i8,mut _7: [u32; 4],mut _8: bool) -> u8 {
mir! {
type RET = u8;
let _9: isize;
let _10: [u8; 5];
let _11: (*mut *mut u8, Adt46);
let _12: isize;
let _13: &'static (*mut i128, (&'static i16,), char);
let _14: *const f64;
let _15: *mut u8;
let _16: f64;
let _17: *mut bool;
let _18: isize;
let _19: *const *mut u8;
let _20: &'static char;
let _21: [u64; 6];
let _22: bool;
let _23: usize;
let _24: [u64; 7];
let _25: isize;
let _26: [u8; 3];
let _27: u16;
let _28: [i32; 2];
let _29: f64;
let _30: char;
let _31: usize;
let _32: *mut u8;
let _33: (i16, usize);
let _34: i16;
let _35: f64;
let _36: &'static bool;
let _37: [i32; 1];
let _38: i8;
let _39: isize;
let _40: ();
let _41: ();
{
_5.0.0 = _2.0.0;
_5.0.0 = _5.0.1 as i16;
RET = !85_u8;
_7 = [3186087767_u32,3587226029_u32,1701711859_u32,4169210182_u32];
_4 = '\u{d39c9}';
_5.0.0 = _2.0.0 << _6;
_7 = [550292939_u32,930893879_u32,964046422_u32,1906179433_u32];
_4 = '\u{23eb9}';
_2.0 = (_5.0.0, _5.0.1);
_7 = [859439602_u32,3013115739_u32,3497325686_u32,2081158321_u32];
_12 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_2.0.1 = _2.0.0 as usize;
RET = !36_u8;
_12 = 9223372036854775807_isize << _2.0.1;
_4 = '\u{7741e}';
RET = _1 as u8;
_3 = _8;
RET = 212_u8;
_15 = core::ptr::addr_of_mut!(RET);
_8 = _12 > _12;
_5.0.1 = _2.0.1;
_8 = _1 | _3;
_9 = !_12;
_12 = _9 << (*_15);
_5.0.1 = 3232074176127978805_u64 as usize;
Goto(bb1)
}
bb1 = {
_5 = (_2.0,);
_8 = _3;
_10 = [RET,(*_15),RET,(*_15),(*_15)];
_6 = (-96_i8) << _5.0.1;
_9 = _4 as isize;
_2.0 = (_5.0.0, _5.0.1);
_10 = [RET,RET,(*_15),RET,RET];
_2.0.0 = _5.0.0;
Call(RET = fn2(_6, _8, _12, _2.0.1, _2.0, _7, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2.0.0 = _5.0.0 & _5.0.0;
_14 = core::ptr::addr_of!(_16);
_11.0 = core::ptr::addr_of_mut!(_15);
_16 = _2.0.1 as f64;
_18 = _6 as isize;
_6 = 32_i8 | 64_i8;
RET = !195_u8;
_7 = [3159116294_u32,2781334624_u32,993750221_u32,3926201124_u32];
_2 = (_5.0,);
_8 = _2.0.1 <= _5.0.1;
_5 = _2;
_17 = core::ptr::addr_of_mut!(_8);
_2 = (_5.0,);
RET = 152683283000028043813121926661550536001_i128 as u8;
(*_17) = _1;
RET = 10_u8;
_9 = -_18;
_5.0.0 = _2.0.0 ^ _2.0.0;
(*_17) = !_3;
_3 = !_8;
_5.0.0 = (-133939108909000248228813056206837528072_i128) as i16;
Goto(bb3)
}
bb3 = {
_5.0.0 = _2.0.0 * _2.0.0;
_17 = core::ptr::addr_of_mut!(_1);
_18 = _3 as isize;
_5 = (_2.0,);
_2.0.1 = _5.0.1;
_5 = _2;
(*_17) = _3;
_19 = core::ptr::addr_of!(_15);
_5 = (_2.0,);
_17 = core::ptr::addr_of_mut!(_3);
(*_14) = _5.0.1 as f64;
_24 = [12659198478623524627_u64,15649015511401113961_u64,7275780235840257042_u64,12492126363271221914_u64,4865379277782513343_u64,15986687139404596044_u64,788170178623789747_u64];
_2.0.1 = !_5.0.1;
_9 = _12 * _12;
_6 = (-40_i8) * (-88_i8);
_2.0.0 = _5.0.0;
match (*_15) {
0 => bb4,
1 => bb5,
2 => bb6,
10 => bb8,
_ => bb7
}
}
bb4 = {
_2.0.0 = _5.0.0 & _5.0.0;
_14 = core::ptr::addr_of!(_16);
_11.0 = core::ptr::addr_of_mut!(_15);
_16 = _2.0.1 as f64;
_18 = _6 as isize;
_6 = 32_i8 | 64_i8;
RET = !195_u8;
_7 = [3159116294_u32,2781334624_u32,993750221_u32,3926201124_u32];
_2 = (_5.0,);
_8 = _2.0.1 <= _5.0.1;
_5 = _2;
_17 = core::ptr::addr_of_mut!(_8);
_2 = (_5.0,);
RET = 152683283000028043813121926661550536001_i128 as u8;
(*_17) = _1;
RET = 10_u8;
_9 = -_18;
_5.0.0 = _2.0.0 ^ _2.0.0;
(*_17) = !_3;
_3 = !_8;
_5.0.0 = (-133939108909000248228813056206837528072_i128) as i16;
Goto(bb3)
}
bb5 = {
_5 = (_2.0,);
_8 = _3;
_10 = [RET,(*_15),RET,(*_15),(*_15)];
_6 = (-96_i8) << _5.0.1;
_9 = _4 as isize;
_2.0 = (_5.0.0, _5.0.1);
_10 = [RET,RET,(*_15),RET,RET];
_2.0.0 = _5.0.0;
Call(RET = fn2(_6, _8, _12, _2.0.1, _2.0, _7, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_20 = &_4;
_21 = [10712075093597193415_u64,7323213008155451702_u64,3010865278008990831_u64,10694397109784299310_u64,4018212426844734549_u64,4020278907739290454_u64];
_12 = _18;
_22 = !(*_17);
(*_19) = core::ptr::addr_of_mut!((*_15));
match (*_15) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb10,
10 => bb12,
_ => bb11
}
}
bb9 = {
_2.0.0 = _5.0.0 & _5.0.0;
_14 = core::ptr::addr_of!(_16);
_11.0 = core::ptr::addr_of_mut!(_15);
_16 = _2.0.1 as f64;
_18 = _6 as isize;
_6 = 32_i8 | 64_i8;
RET = !195_u8;
_7 = [3159116294_u32,2781334624_u32,993750221_u32,3926201124_u32];
_2 = (_5.0,);
_8 = _2.0.1 <= _5.0.1;
_5 = _2;
_17 = core::ptr::addr_of_mut!(_8);
_2 = (_5.0,);
RET = 152683283000028043813121926661550536001_i128 as u8;
(*_17) = _1;
RET = 10_u8;
_9 = -_18;
_5.0.0 = _2.0.0 ^ _2.0.0;
(*_17) = !_3;
_3 = !_8;
_5.0.0 = (-133939108909000248228813056206837528072_i128) as i16;
Goto(bb3)
}
bb10 = {
_5.0.0 = _2.0.0 * _2.0.0;
_17 = core::ptr::addr_of_mut!(_1);
_18 = _3 as isize;
_5 = (_2.0,);
_2.0.1 = _5.0.1;
_5 = _2;
(*_17) = _3;
_19 = core::ptr::addr_of!(_15);
_5 = (_2.0,);
_17 = core::ptr::addr_of_mut!(_3);
(*_14) = _5.0.1 as f64;
_24 = [12659198478623524627_u64,15649015511401113961_u64,7275780235840257042_u64,12492126363271221914_u64,4865379277782513343_u64,15986687139404596044_u64,788170178623789747_u64];
_2.0.1 = !_5.0.1;
_9 = _12 * _12;
_6 = (-40_i8) * (-88_i8);
_2.0.0 = _5.0.0;
match (*_15) {
0 => bb4,
1 => bb5,
2 => bb6,
10 => bb8,
_ => bb7
}
}
bb11 = {
_5 = (_2.0,);
_8 = _3;
_10 = [RET,(*_15),RET,(*_15),(*_15)];
_6 = (-96_i8) << _5.0.1;
_9 = _4 as isize;
_2.0 = (_5.0.0, _5.0.1);
_10 = [RET,RET,(*_15),RET,RET];
_2.0.0 = _5.0.0;
Call(RET = fn2(_6, _8, _12, _2.0.1, _2.0, _7, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_25 = _9 & _18;
_6 = !(-91_i8);
RET = !49_u8;
_11.0 = core::ptr::addr_of_mut!((*_19));
_3 = (*_14) < _16;
_27 = !51189_u16;
_2.0.0 = -_5.0.0;
_7 = [2331412164_u32,2669987937_u32,685429001_u32,479768446_u32];
_28 = [481494109_i32,(-459023110_i32)];
(*_19) = core::ptr::addr_of_mut!(RET);
_5 = (_2.0,);
_24 = [1052589700890268354_u64,7446696773300683393_u64,3396934841251597248_u64,8249300847165553990_u64,3490645905330059968_u64,8217649160869764096_u64,12142687857597801723_u64];
_21 = [10700290091688960282_u64,18196225604356654519_u64,17955948772559085458_u64,16304576128306734973_u64,16945785172709202938_u64,999137697977499192_u64];
_6 = _25 as i8;
(*_17) = _8;
_12 = _25;
_12 = 16276604562245169880_u64 as isize;
_26 = [(*_15),(*_15),(*_15)];
_9 = !_18;
_16 = 2464562401_u32 as f64;
_2 = (_5.0,);
_24 = [17142656668146349568_u64,17231237856292284342_u64,7688019510251878758_u64,9714104693231528088_u64,7695724870338659671_u64,5602375393223039276_u64,15675049712653093305_u64];
_2.0 = (_5.0.0, _5.0.1);
_29 = -_16;
(*_14) = -_29;
_14 = core::ptr::addr_of!(_16);
_11.0 = core::ptr::addr_of_mut!(_32);
Goto(bb13)
}
bb13 = {
_2.0.0 = RET as i16;
_11.0 = core::ptr::addr_of_mut!((*_19));
_22 = _18 >= _25;
(*_15) = 141_u8;
_20 = &(*_20);
_18 = -_25;
_1 = !(*_17);
_33.1 = _5.0.1 ^ _2.0.1;
_11.1 = Adt46::Variant2 { fld0: Move(_15),fld1: (-1255257394_i32),fld2: _25 };
_31 = _33.1 * _2.0.1;
_31 = _33.1;
_30 = _4;
_3 = _8 | _8;
_23 = _31 - _2.0.1;
_33.1 = _2.0.1 * _23;
_33.1 = _2.0.1;
_2.0.1 = !_31;
_36 = &_3;
(*_14) = _27 as f64;
_37 = [350817291_i32];
place!(Field::<*mut u8>(Variant(_11.1, 2), 0)) = core::ptr::addr_of_mut!(RET);
_30 = (*_20);
_19 = core::ptr::addr_of!((*_19));
place!(Field::<*mut u8>(Variant(_11.1, 2), 0)) = core::ptr::addr_of_mut!(RET);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
141 => bb14,
_ => bb11
}
}
bb14 = {
_33.0 = !_5.0.0;
_5.0 = _2.0;
_2.0.0 = -_33.0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(1_usize, 27_usize, Move(_27), 22_usize, Move(_22), 6_usize, Move(_6), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(1_usize, 8_usize, Move(_8), 3_usize, Move(_3), 24_usize, Move(_24), 37_usize, Move(_37)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(1_usize, 2_usize, Move(_2), 10_usize, Move(_10), 31_usize, Move(_31), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i8,mut _2: bool,mut _3: isize,mut _4: usize,mut _5: (i16, usize),mut _6: [u32; 4],mut _7: ((i16, usize),)) -> u8 {
mir! {
type RET = u8;
let _8: char;
let _9: f64;
let _10: &'static *const usize;
let _11: [i8; 1];
let _12: Adt46;
let _13: isize;
let _14: i8;
let _15: (*mut *mut u8, Adt46);
let _16: i8;
let _17: *mut i8;
let _18: u8;
let _19: f32;
let _20: &'static *mut *mut u8;
let _21: [i8; 2];
let _22: [u64; 6];
let _23: usize;
let _24: char;
let _25: ((i16, usize),);
let _26: u128;
let _27: &'static *mut *mut u8;
let _28: isize;
let _29: [u8; 5];
let _30: *mut bool;
let _31: *mut isize;
let _32: i64;
let _33: char;
let _34: isize;
let _35: &'static *mut *mut u8;
let _36: (*mut i128, (&'static i16,), char);
let _37: i8;
let _38: [u8; 3];
let _39: f32;
let _40: (&'static i16,);
let _41: [u128; 2];
let _42: ();
let _43: ();
{
_7.0.1 = 21069_u16 as usize;
_11 = [_1];
_7.0.0 = _5.0;
RET = 91_u8 | 26_u8;
_7 = (_5,);
_13 = _3;
RET = 1393969194_i32 as u8;
_1 = -79_i8;
_5 = (_7.0.0, _7.0.1);
_13 = RET as isize;
_3 = '\u{9395c}' as isize;
Call(_7 = fn3(_5.1, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = (_7.0.0, _7.0.1);
_9 = _5.1 as f64;
_7.0.0 = _9 as i16;
_13 = _3 ^ _3;
_9 = 8185927054413669967_i64 as f64;
_2 = false;
_9 = 38737_u16 as f64;
_14 = (-1699033740_i32) as i8;
_9 = (-5325629904321646299989683211667497484_i128) as f64;
_7 = (_5,);
_8 = '\u{f108d}';
_1 = 4748913778481307480_i64 as i8;
RET = 25_u8 | 100_u8;
_8 = '\u{d55d0}';
_5.1 = _4 | _4;
_13 = 1871819016362695411_i64 as isize;
_7.0.0 = _5.0;
_16 = _1 >> _1;
_8 = '\u{5796b}';
_11 = [_1];
_7.0 = (_5.0, _5.1);
Goto(bb2)
}
bb2 = {
_5 = _7.0;
_2 = RET == RET;
_1 = _16 - _16;
_7 = (_5,);
_7.0 = _5;
_5 = (_7.0.0, _7.0.1);
_16 = (-1933122481_i32) as i8;
_2 = false;
_8 = '\u{388e6}';
_8 = '\u{65e77}';
_4 = _5.1;
_7 = (_5,);
_9 = 51399_u16 as f64;
_7.0 = (_5.0, _5.1);
_16 = _14;
_11 = [_16];
_18 = RET >> _4;
_2 = !true;
_13 = !_3;
_5.1 = _4 | _4;
_7.0.0 = _5.1 as i16;
_5.1 = !_4;
_19 = 276333034888683224013577558619016008744_u128 as f32;
Goto(bb3)
}
bb3 = {
_17 = core::ptr::addr_of_mut!(_1);
_8 = '\u{d2d3b}';
_1 = -_14;
_5 = (_7.0.0, _4);
_7.0.0 = _5.0 - _5.0;
_7.0.1 = _9 as usize;
_6 = [3225900667_u32,4138381800_u32,2238015460_u32,430549490_u32];
_22 = [14856583845391132280_u64,16283023877552573105_u64,16191542995796492566_u64,2097414407110848963_u64,17540170252678758106_u64,3233939288729782934_u64];
_7.0.1 = (-1002050453327324216_i64) as usize;
_20 = &_15.0;
_21 = [_16,(*_17)];
_24 = _8;
_3 = _13;
_8 = _24;
_11 = [_1];
_18 = RET;
Goto(bb4)
}
bb4 = {
_7.0.0 = 52356_u16 as i16;
_17 = core::ptr::addr_of_mut!((*_17));
RET = _24 as u8;
_14 = (*_17);
_7.0.1 = _5.1;
_19 = _5.1 as f32;
_19 = 3476117305208538761_i64 as f32;
(*_17) = _16;
_13 = _3;
_25.0 = (_5.0, _5.1);
_7 = _25;
_5.0 = _25.0.0 ^ _7.0.0;
_19 = RET as f32;
_4 = _14 as usize;
_18 = RET;
_25.0 = (_7.0.0, _5.1);
_25.0.0 = _5.0 << _5.0;
_24 = _8;
_25.0 = _5;
RET = (-45763968266928539616079655073726976416_i128) as u8;
_23 = (-1655949985_i32) as usize;
Goto(bb5)
}
bb5 = {
_20 = &(*_20);
_26 = _9 as u128;
_23 = RET as usize;
_24 = _8;
_7.0.0 = -_5.0;
_18 = _7.0.1 as u8;
_5.0 = 2041942800_u32 as i16;
_3 = _13 & _13;
_13 = -_3;
_3 = _13 | _13;
_25.0.0 = (-970021120_i32) as i16;
_20 = &_15.0;
_16 = (-95212234862851152202977514194250958359_i128) as i8;
_22 = [10692441027379843931_u64,3953653599293145006_u64,11820624758724336880_u64,7975502174750285213_u64,8362910648579944676_u64,16981893304877829521_u64];
_17 = core::ptr::addr_of_mut!((*_17));
_8 = _24;
_24 = _8;
_7.0.0 = _5.0;
Goto(bb6)
}
bb6 = {
_28 = _3 | _3;
_16 = (*_17);
_7 = (_25.0,);
_6 = [739946113_u32,1999652932_u32,2030089168_u32,2323787688_u32];
Goto(bb7)
}
bb7 = {
_16 = (*_17);
Goto(bb8)
}
bb8 = {
_18 = RET | RET;
_25 = _7;
RET = _18 - _18;
Goto(bb9)
}
bb9 = {
_6 = [625181311_u32,653021169_u32,3592256662_u32,162892415_u32];
_4 = _7.0.1;
_5 = (_25.0.0, _4);
_11 = [_1];
_6 = [1520850635_u32,4262145399_u32,2078420936_u32,284145600_u32];
_28 = _19 as isize;
(*_17) = !_16;
_26 = !97355187189007578260525182194905679549_u128;
_11 = [_14];
_27 = &(*_20);
_25 = _7;
_22 = [15487092030678295667_u64,2071130524244155674_u64,8138852067760145843_u64,15385934091630632335_u64,15119045764273109507_u64,2332285872373822460_u64];
_16 = (*_17) * (*_17);
_11 = [_1];
_29 = [RET,RET,RET,RET,_18];
_7.0.1 = _5.1 & _5.1;
_6 = [4091762621_u32,1516767867_u32,285314311_u32,1457641626_u32];
_20 = &(*_27);
_7 = (_5,);
_7 = _25;
_26 = !299434665227732751771242571823979958394_u128;
Goto(bb10)
}
bb10 = {
_25.0.1 = _25.0.0 as usize;
_21 = [_16,_16];
_4 = _7.0.1 & _5.1;
RET = _18;
_26 = 37012174682518064774044963906413001283_u128;
_6 = [2832121796_u32,2688064338_u32,1178341568_u32,3629212016_u32];
_5.0 = -_7.0.0;
(*_17) = _16 + _16;
RET = _18 << _3;
_7 = (_25.0,);
_5.1 = !_4;
_24 = _8;
_24 = _8;
_24 = _8;
_14 = -_16;
_31 = core::ptr::addr_of_mut!(_28);
_7.0.1 = _4 | _4;
match _26 {
37012174682518064774044963906413001283 => bb12,
_ => bb11
}
}
bb11 = {
_6 = [625181311_u32,653021169_u32,3592256662_u32,162892415_u32];
_4 = _7.0.1;
_5 = (_25.0.0, _4);
_11 = [_1];
_6 = [1520850635_u32,4262145399_u32,2078420936_u32,284145600_u32];
_28 = _19 as isize;
(*_17) = !_16;
_26 = !97355187189007578260525182194905679549_u128;
_11 = [_14];
_27 = &(*_20);
_25 = _7;
_22 = [15487092030678295667_u64,2071130524244155674_u64,8138852067760145843_u64,15385934091630632335_u64,15119045764273109507_u64,2332285872373822460_u64];
_16 = (*_17) * (*_17);
_11 = [_1];
_29 = [RET,RET,RET,RET,_18];
_7.0.1 = _5.1 & _5.1;
_6 = [4091762621_u32,1516767867_u32,285314311_u32,1457641626_u32];
_20 = &(*_27);
_7 = (_5,);
_7 = _25;
_26 = !299434665227732751771242571823979958394_u128;
Goto(bb10)
}
bb12 = {
_17 = core::ptr::addr_of_mut!((*_17));
_22 = [15720341225031082886_u64,3989221732680936590_u64,9200822763086999982_u64,9269905361312890222_u64,5141527329511268252_u64,2626725715105303164_u64];
_4 = _7.0.1 + _5.1;
_5.1 = _26 as usize;
_2 = !true;
_7 = (_25.0,);
_33 = _24;
_19 = 1436585020343530651_u64 as f32;
_2 = false;
_2 = !true;
RET = _18;
_8 = _33;
_30 = core::ptr::addr_of_mut!(_2);
_26 = !214738514672943203023219887489510172266_u128;
_7.0 = (_25.0.0, _4);
_20 = &(*_27);
_25.0.0 = _7.0.0;
(*_31) = _3;
Goto(bb13)
}
bb13 = {
_25.0.1 = !_4;
_36.2 = _33;
_33 = _36.2;
_5 = _25.0;
_37 = !(*_17);
_7.0.0 = !_5.0;
_20 = &(*_27);
_5 = (_7.0.0, _4);
_13 = !_3;
_26 = 228423453932410065283964433445792721654_u128;
_3 = (*_31);
_7.0.1 = _5.1;
_25 = (_5,);
_20 = &(*_27);
_25.0.1 = _4 + _4;
_34 = _13;
_26 = 146473965447044414699051077367332138494_u128;
Goto(bb14)
}
bb14 = {
_17 = core::ptr::addr_of_mut!((*_17));
_34 = !_13;
_1 = _16;
_6 = [1989006269_u32,3773391441_u32,2622617748_u32,4215699202_u32];
_39 = -_19;
_20 = &(*_27);
_33 = _8;
_24 = _36.2;
_7.0 = (_25.0.0, _25.0.1);
(*_31) = _34 >> _7.0.1;
_17 = core::ptr::addr_of_mut!(_16);
_13 = (*_31) - (*_31);
_37 = _13 as i8;
_3 = -_13;
_29 = [_18,RET,RET,_18,_18];
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(2_usize, 4_usize, Move(_4), 2_usize, Move(_2), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(2_usize, 6_usize, Move(_6), 29_usize, Move(_29), 21_usize, Move(_21), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(2_usize, 8_usize, Move(_8), 11_usize, Move(_11), 22_usize, Move(_22), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: usize,mut _2: [u32; 4],mut _3: isize) -> ((i16, usize),) {
mir! {
type RET = ((i16, usize),);
let _4: ([char; 7], *mut u8);
let _5: u16;
let _6: [usize; 2];
let _7: &'static i8;
let _8: *mut i128;
let _9: [u128; 1];
let _10: i8;
let _11: &'static [char; 7];
let _12: *mut i8;
let _13: Adt60;
let _14: [i8; 2];
let _15: f64;
let _16: [i32; 2];
let _17: f32;
let _18: f32;
let _19: f64;
let _20: &'static i16;
let _21: &'static &'static (*mut i128, (&'static i16,), char);
let _22: Adt73;
let _23: char;
let _24: f32;
let _25: isize;
let _26: [isize; 2];
let _27: &'static *mut *mut u8;
let _28: *const usize;
let _29: [u128; 1];
let _30: *const usize;
let _31: char;
let _32: (*mut *mut u8, Adt46);
let _33: char;
let _34: i128;
let _35: ();
let _36: ();
{
RET.0 = ((-20266_i16), _1);
RET.0.1 = 15_u8 as usize;
RET.0 = (3783_i16, _1);
RET.0 = (18975_i16, _1);
RET.0 = ((-26388_i16), _1);
RET.0 = (5963_i16, _1);
_3 = 9223372036854775807_isize;
_4.0 = ['\u{26381}','\u{ac1c7}','\u{dc9ce}','\u{c8a17}','\u{1a3fd}','\u{8d47d}','\u{f06b4}'];
_2 = [2798964966_u32,1371450733_u32,849037780_u32,3718706568_u32];
_5 = !50824_u16;
_6 = [_1,RET.0.1];
RET.0 = (14591_i16, _1);
RET.0 = ((-31397_i16), _1);
RET.0.1 = !_1;
RET.0.1 = !_1;
_3 = (-9223372036854775808_isize);
_1 = RET.0.1 >> RET.0.1;
RET.0 = ((-11576_i16), _1);
RET.0.1 = _1;
_2 = [985034554_u32,2603950020_u32,305123992_u32,2587151249_u32];
_2 = [3370043774_u32,1090040032_u32,1594486196_u32,2858684787_u32];
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
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
_2 = [2761336911_u32,720691514_u32,583917954_u32,1643065196_u32];
_10 = 27_i8;
RET.0.0 = -7241_i16;
_11 = &_4.0;
RET.0 = (16747_i16, _1);
RET.0 = (11983_i16, _1);
_3 = !(-82_isize);
RET.0.1 = (-1792035298_i32) as usize;
_2 = [1489325583_u32,4235395755_u32,3242054257_u32,2157995150_u32];
_5 = !25270_u16;
_15 = 13749916795753977792_u64 as f64;
RET.0.1 = !_1;
_16 = [(-64358999_i32),(-588987553_i32)];
_17 = _5 as f32;
_18 = 2794838491927387408_u64 as f32;
RET.0 = (15691_i16, _1);
RET.0 = ((-15506_i16), _1);
RET.0 = ((-22473_i16), _1);
_5 = _18 as u16;
_5 = !27803_u16;
_11 = &(*_11);
RET.0 = ((-4402_i16), _1);
_7 = &_10;
match RET.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
340282366920938463463374607431768207054 => bb9,
_ => bb8
}
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_19 = _15 + _15;
_18 = -_17;
_14 = [_10,_10];
Call(_16 = fn4(Move(_7), _6, RET.0.0, RET.0.0, RET, _18, _3, RET.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7 = &_10;
_2 = [854366620_u32,2536601019_u32,3417798724_u32,86472363_u32];
_18 = _17 - _17;
_2 = [2821179175_u32,3806089301_u32,1505286674_u32,4212333307_u32];
_5 = !33542_u16;
_1 = _5 as usize;
_5 = 12939571594712726716_u64 as u16;
_4.0 = ['\u{64fb5}','\u{6225a}','\u{10394c}','\u{a951f}','\u{80bc3}','\u{65a45}','\u{3a680}'];
_18 = -_17;
_16 = [891921110_i32,(-1183707402_i32)];
RET.0.0 = !(-13360_i16);
RET.0 = ((-14625_i16), _1);
RET.0.0 = 2351_i16 - 5539_i16;
_15 = -_19;
RET.0.1 = _1 - _1;
_1 = !RET.0.1;
_2 = [2393898985_u32,3422986689_u32,3011315965_u32,3243601015_u32];
_9 = [209020083930082801040273266704485050717_u128];
_20 = &RET.0.0;
_12 = core::ptr::addr_of_mut!((*_7));
_4.0 = ['\u{47483}','\u{247b}','\u{7ca9d}','\u{52549}','\u{b77e8}','\u{3ffca}','\u{e6b00}'];
RET.0 = (880_i16, _1);
RET.0 = (12413_i16, _1);
_15 = _19 * _19;
_4.0 = ['\u{fbaf0}','\u{8d428}','\u{20d4f}','\u{9e6ae}','\u{6810e}','\u{f2d12}','\u{1b589}'];
_11 = &_4.0;
match RET.0.0 {
0 => bb11,
12413 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_18 = _17;
RET.0.1 = _1;
_3 = 16443240782181285078_u64 as isize;
_25 = -_3;
RET.0.1 = !_1;
_26 = [_3,_25];
_15 = _19 - _19;
_25 = _3 - _3;
_3 = _18 as isize;
_25 = _3;
_11 = &_4.0;
_5 = 19275_u16;
_12 = core::ptr::addr_of_mut!((*_12));
_17 = 28438991258330132087968955854303685397_u128 as f32;
_26 = [_25,_25];
_7 = &(*_12);
_7 = &(*_12);
_1 = !RET.0.1;
_29 = [151263620143099457034834867787070021214_u128];
_12 = core::ptr::addr_of_mut!((*_7));
_11 = &(*_11);
_5 = (-593451114_i32) as u16;
_26 = [_3,_25];
RET.0.0 = 29773_i16;
_23 = '\u{507cd}';
_10 = -103_i8;
match RET.0.0 {
0 => bb8,
1 => bb4,
2 => bb14,
3 => bb15,
29773 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_25 = _3 - _3;
_27 = &_32.0;
_31 = _23;
_28 = core::ptr::addr_of!(RET.0.1);
_28 = core::ptr::addr_of!(RET.0.1);
_32.0 = core::ptr::addr_of_mut!(_4.1);
_1 = (*_28) >> _25;
_19 = -_15;
_30 = Move(_28);
_25 = _3;
_15 = -_19;
_28 = Move(_30);
_25 = _3 * _3;
_10 = 88_i8;
RET.0.1 = _1 - _1;
RET.0 = (31195_i16, _1);
RET.0.0 = -(-17500_i16);
_25 = !_3;
_25 = !_3;
_20 = &RET.0.0;
_27 = &_32.0;
_12 = core::ptr::addr_of_mut!(_10);
_14 = [(*_12),(*_12)];
_15 = 2576330316_u32 as f64;
_6 = [RET.0.1,_1];
_19 = _15;
RET.0.0 = (-15536_i16) & (-27927_i16);
_6 = [_1,_1];
_30 = core::ptr::addr_of!(_1);
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(3_usize, 1_usize, Move(_1), 31_usize, Move(_31), 10_usize, Move(_10), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(3_usize, 6_usize, Move(_6), 14_usize, Move(_14), 16_usize, Move(_16), 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: &'static i8,mut _2: [usize; 2],mut _3: i16,mut _4: i16,mut _5: ((i16, usize),),mut _6: f32,mut _7: isize,mut _8: (i16, usize)) -> [i32; 2] {
mir! {
type RET = [i32; 2];
let _9: u32;
let _10: [i8; 2];
let _11: char;
let _12: f32;
let _13: u128;
let _14: f64;
let _15: u8;
let _16: u64;
let _17: [u64; 6];
let _18: &'static (*mut i128, (&'static i16,), char);
let _19: [u64; 7];
let _20: *mut i64;
let _21: *mut Adt24;
let _22: [isize; 5];
let _23: [i32; 2];
let _24: u128;
let _25: bool;
let _26: u128;
let _27: isize;
let _28: [u64; 6];
let _29: i64;
let _30: [u128; 2];
let _31: &'static bool;
let _32: *mut bool;
let _33: ();
let _34: ();
{
_8.0 = _3 - _5.0.0;
RET = [(-1649624628_i32),1850561451_i32];
_8.0 = -_3;
_8.1 = (-3380076288515862003_i64) as usize;
_5.0.1 = _8.1;
_5.0.1 = _8.1 ^ _8.1;
_10 = [84_i8,54_i8];
_9 = !265727197_u32;
_6 = 2062003152_i32 as f32;
_8.0 = _4 ^ _4;
_5.0.1 = _8.1;
_5 = (_8,);
RET = [(-494430522_i32),383381734_i32];
_9 = 1517795943_u32 << _4;
_8.1 = !_5.0.1;
RET = [(-1146636613_i32),(-80921456_i32)];
_11 = '\u{a24ae}';
_8 = _5.0;
Call(_5.0.1 = fn5(_5.0.0, _5.0.0, _9, _8.0, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [2025211733_i32,(-1347306408_i32)];
_3 = _4;
match _4 {
0 => bb2,
340282366920938463463374607431768207054 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_13 = !4878019354644934456473966573047248680_u128;
_7 = 9223372036854775807_isize - (-123_isize);
_12 = _6;
_5.0 = _8;
_5.0.0 = _6 as i16;
_12 = _6 + _6;
_5.0 = (_8.0, _8.1);
_8.0 = _4;
_14 = (-1518635206_i32) as f64;
_8 = _5.0;
_5 = (_8,);
_5.0.1 = _8.1 & _8.1;
_8.1 = _5.0.1 & _5.0.1;
_15 = !42_u8;
_10 = [(-104_i8),18_i8];
_5 = (_8,);
_8.0 = 957855597_i32 as i16;
_5.0 = (_3, _8.1);
_5 = (_8,);
RET = [(-1454581256_i32),(-692034331_i32)];
_4 = _3;
Goto(bb5)
}
bb5 = {
_8.0 = _3 + _4;
_16 = 2321428588670341845_u64;
_14 = _15 as f64;
_5.0 = (_4, _8.1);
_19 = [_16,_16,_16,_16,_16,_16,_16];
_12 = _13 as f32;
_5.0.1 = _8.1 * _8.1;
_12 = -_6;
_7 = 6_isize * (-9223372036854775808_isize);
_8.1 = !_5.0.1;
_11 = '\u{2d23b}';
_16 = !4024325414396140053_u64;
_4 = _8.0;
_8.1 = 62943_u16 as usize;
_7 = (-9223372036854775808_isize) & 92_isize;
_9 = 2636375204_u32;
_11 = '\u{60b25}';
_17 = [_16,_16,_16,_16,_16,_16];
_5.0.0 = !_4;
_11 = '\u{6da1e}';
_11 = '\u{af690}';
_12 = 36294_u16 as f32;
_5 = (_8,);
match _3 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
340282366920938463463374607431768207054 => bb11,
_ => bb10
}
}
bb6 = {
_13 = !4878019354644934456473966573047248680_u128;
_7 = 9223372036854775807_isize - (-123_isize);
_12 = _6;
_5.0 = _8;
_5.0.0 = _6 as i16;
_12 = _6 + _6;
_5.0 = (_8.0, _8.1);
_8.0 = _4;
_14 = (-1518635206_i32) as f64;
_8 = _5.0;
_5 = (_8,);
_5.0.1 = _8.1 & _8.1;
_8.1 = _5.0.1 & _5.0.1;
_15 = !42_u8;
_10 = [(-104_i8),18_i8];
_5 = (_8,);
_8.0 = 957855597_i32 as i16;
_5.0 = (_3, _8.1);
_5 = (_8,);
RET = [(-1454581256_i32),(-692034331_i32)];
_4 = _3;
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
RET = [2025211733_i32,(-1347306408_i32)];
_3 = _4;
match _4 {
0 => bb2,
340282366920938463463374607431768207054 => bb4,
_ => bb3
}
}
bb10 = {
Return()
}
bb11 = {
_13 = _6 as u128;
_5.0.1 = _8.1;
RET = [969656642_i32,1763937379_i32];
_3 = !_5.0.0;
_3 = _8.0 + _4;
_8 = (_3, _5.0.1);
_5.0.1 = !_8.1;
_12 = -_6;
_8.0 = _4 + _4;
_8 = (_3, _5.0.1);
_25 = false;
_27 = _7 & _7;
_7 = _12 as isize;
_22 = [_7,_7,_7,_27,_27];
_19 = [_16,_16,_16,_16,_16,_16,_16];
_28 = [_16,_16,_16,_16,_16,_16];
match _9 {
2636375204 => bb13,
_ => bb12
}
}
bb12 = {
RET = [2025211733_i32,(-1347306408_i32)];
_3 = _4;
match _4 {
0 => bb2,
340282366920938463463374607431768207054 => bb4,
_ => bb3
}
}
bb13 = {
_16 = 1311516422873230083_u64;
_14 = 1069049038_i32 as f64;
_23 = [406314599_i32,(-988714648_i32)];
_16 = !5364189930968906082_u64;
RET = [(-625532037_i32),(-1009520256_i32)];
_31 = &_25;
_17 = [_16,_16,_16,_16,_16,_16];
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
2636375204 => bb15,
_ => bb14
}
}
bb14 = {
_13 = !4878019354644934456473966573047248680_u128;
_7 = 9223372036854775807_isize - (-123_isize);
_12 = _6;
_5.0 = _8;
_5.0.0 = _6 as i16;
_12 = _6 + _6;
_5.0 = (_8.0, _8.1);
_8.0 = _4;
_14 = (-1518635206_i32) as f64;
_8 = _5.0;
_5 = (_8,);
_5.0.1 = _8.1 & _8.1;
_8.1 = _5.0.1 & _5.0.1;
_15 = !42_u8;
_10 = [(-104_i8),18_i8];
_5 = (_8,);
_8.0 = 957855597_i32 as i16;
_5.0 = (_3, _8.1);
_5 = (_8,);
RET = [(-1454581256_i32),(-692034331_i32)];
_4 = _3;
Goto(bb5)
}
bb15 = {
_10 = [(-41_i8),(-37_i8)];
_5.0.1 = 84_i8 as usize;
_6 = _12 - _12;
_16 = 2109121910097704349_u64 - 5732673249510736586_u64;
_19 = [_16,_16,_16,_16,_16,_16,_16];
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(4_usize, 22_usize, Move(_22), 13_usize, Move(_13), 28_usize, Move(_28), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(4_usize, 16_usize, Move(_16), 2_usize, Move(_2), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(4_usize, 11_usize, Move(_11), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i16,mut _2: i16,mut _3: u32,mut _4: i16,mut _5: i16) -> usize {
mir! {
type RET = usize;
let _6: u32;
let _7: [i8; 1];
let _8: f32;
let _9: [u8; 3];
let _10: ([char; 7], *mut u8);
let _11: f32;
let _12: isize;
let _13: isize;
let _14: [u128; 1];
let _15: (Adt47, [u128; 1], &'static f64);
let _16: i8;
let _17: Adt60;
let _18: &'static i8;
let _19: ([char; 7], *mut u8);
let _20: char;
let _21: u8;
let _22: char;
let _23: char;
let _24: f64;
let _25: &'static &'static (*mut i128, (&'static i16,), char);
let _26: i8;
let _27: ();
let _28: ();
{
_3 = 2239663086_u32 | 3974881960_u32;
RET = 29948125830986056298643712882490025225_u128 as usize;
_4 = _2;
_1 = _4 >> _4;
_1 = RET as i16;
_2 = -_4;
_3 = 50258_u16 as u32;
_6 = _3 << _2;
_7 = [105_i8];
_2 = _4 + _4;
_1 = -_2;
_3 = 105_i8 as u32;
RET = 9223372036854775807_isize as usize;
RET = 3597649855231448996_usize & 18048547353732705463_usize;
_4 = (-8037183378307506843_i64) as i16;
_4 = _1;
_8 = 228_u8 as f32;
_5 = _1;
_1 = !_2;
_5 = !_1;
_3 = _2 as u32;
RET = 1347834593624084236_usize ^ 4_usize;
_2 = _1 - _1;
Goto(bb1)
}
bb1 = {
RET = !6_usize;
_7 = [41_i8];
_5 = -_1;
_10.0 = ['\u{79bbc}','\u{b4da7}','\u{e9cba}','\u{f6f10}','\u{d343d}','\u{f9890}','\u{d47f}'];
_3 = _6;
_7 = [(-101_i8)];
RET = 7769672095887175763_u64 as usize;
_5 = _1;
_3 = 9223372036854775807_isize as u32;
_5 = '\u{e764c}' as i16;
_10.0 = ['\u{7a253}','\u{e4e7c}','\u{efbb7}','\u{ce2d4}','\u{c781a}','\u{f07b6}','\u{64c75}'];
_7 = [(-69_i8)];
_10.0 = ['\u{e0af7}','\u{4118c}','\u{254c9}','\u{e5cce}','\u{85fc9}','\u{ea3b9}','\u{4fcaa}'];
_10.0 = ['\u{de170}','\u{101ce5}','\u{c572d}','\u{a637f}','\u{30980}','\u{d9d8b}','\u{5a6c0}'];
_7 = [(-77_i8)];
RET = !5073291111492019507_usize;
_9 = [70_u8,90_u8,58_u8];
_10.0 = ['\u{3fa8c}','\u{872e9}','\u{4c6f9}','\u{646b8}','\u{17fd6}','\u{a052e}','\u{f2000}'];
_1 = !_2;
_11 = -_8;
RET = 15681080092732933927_usize - 2_usize;
RET = 2262720721969983945_usize;
_4 = -_2;
_6 = _3;
_8 = _11;
_6 = _3 - _3;
_4 = !_1;
_1 = _2;
_8 = -_11;
_11 = _8 * _8;
Goto(bb2)
}
bb2 = {
RET = 13797247984332438266_usize;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
13797247984332438266 => bb8,
_ => bb7
}
}
bb3 = {
RET = !6_usize;
_7 = [41_i8];
_5 = -_1;
_10.0 = ['\u{79bbc}','\u{b4da7}','\u{e9cba}','\u{f6f10}','\u{d343d}','\u{f9890}','\u{d47f}'];
_3 = _6;
_7 = [(-101_i8)];
RET = 7769672095887175763_u64 as usize;
_5 = _1;
_3 = 9223372036854775807_isize as u32;
_5 = '\u{e764c}' as i16;
_10.0 = ['\u{7a253}','\u{e4e7c}','\u{efbb7}','\u{ce2d4}','\u{c781a}','\u{f07b6}','\u{64c75}'];
_7 = [(-69_i8)];
_10.0 = ['\u{e0af7}','\u{4118c}','\u{254c9}','\u{e5cce}','\u{85fc9}','\u{ea3b9}','\u{4fcaa}'];
_10.0 = ['\u{de170}','\u{101ce5}','\u{c572d}','\u{a637f}','\u{30980}','\u{d9d8b}','\u{5a6c0}'];
_7 = [(-77_i8)];
RET = !5073291111492019507_usize;
_9 = [70_u8,90_u8,58_u8];
_10.0 = ['\u{3fa8c}','\u{872e9}','\u{4c6f9}','\u{646b8}','\u{17fd6}','\u{a052e}','\u{f2000}'];
_1 = !_2;
_11 = -_8;
RET = 15681080092732933927_usize - 2_usize;
RET = 2262720721969983945_usize;
_4 = -_2;
_6 = _3;
_8 = _11;
_6 = _3 - _3;
_4 = !_1;
_1 = _2;
_8 = -_11;
_11 = _8 * _8;
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
_11 = _8;
RET = 6_usize - 3_usize;
_6 = _3;
RET = 30685_u16 as usize;
_4 = _2;
_6 = _3;
_8 = 113781275631081292387453755372880672363_i128 as f32;
_10.0 = ['\u{10fc80}','\u{bbf3c}','\u{10f588}','\u{af861}','\u{1f0cf}','\u{ebaa}','\u{5d380}'];
_13 = 57585_u16 as isize;
_10.0 = ['\u{64f9d}','\u{97c47}','\u{b54ca}','\u{9e5c8}','\u{df8fc}','\u{597}','\u{3d336}'];
_9 = [92_u8,240_u8,41_u8];
Call(_6 = fn6(_4, _1, _4, _1, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = _3;
_14 = [10371797169132482222457925809054912285_u128];
RET = 927670339645561537_usize & 4_usize;
_7 = [21_i8];
Goto(bb10)
}
bb10 = {
_2 = _1;
Goto(bb11)
}
bb11 = {
RET = 50302923656630006681051779743898622552_i128 as usize;
_13 = -(-9223372036854775808_isize);
_1 = _2 << _4;
_12 = (-3034334998444502349_i64) as isize;
_9 = [188_u8,119_u8,147_u8];
_2 = _1;
_2 = 212_u8 as i16;
_4 = RET as i16;
_10.0 = ['\u{10ba8f}','\u{7a1f5}','\u{8719f}','\u{c2799}','\u{7adb9}','\u{1cba9}','\u{85aff}'];
_1 = _4 ^ _4;
Goto(bb12)
}
bb12 = {
_6 = (-14_i8) as u32;
_3 = !_6;
_5 = _1 + _4;
_15.1 = [318082414734958060060544858500811500623_u128];
_14 = [205106312950778538052517719213354544024_u128];
_14 = _15.1;
_6 = _3;
_6 = 6319652492512291722_i64 as u32;
_3 = !_6;
_16 = -(-124_i8);
_3 = _6;
_6 = _3;
_19.0 = _10.0;
_19.0 = _10.0;
_21 = 26999_u16 as u8;
_19.1 = core::ptr::addr_of_mut!(_21);
_4 = _1;
_3 = RET as u32;
RET = 5_usize & 3_usize;
_8 = 4481227788727713605_i64 as f32;
_10.1 = core::ptr::addr_of_mut!(_21);
_8 = _11;
Goto(bb13)
}
bb13 = {
_19 = Move(_10);
_5 = _4 - _4;
_16 = (-74_i8);
match _16 {
0 => bb12,
1 => bb2,
2 => bb10,
3 => bb9,
4 => bb5,
5 => bb14,
6 => bb15,
340282366920938463463374607431768211382 => bb17,
_ => bb16
}
}
bb14 = {
_6 = (-14_i8) as u32;
_3 = !_6;
_5 = _1 + _4;
_15.1 = [318082414734958060060544858500811500623_u128];
_14 = [205106312950778538052517719213354544024_u128];
_14 = _15.1;
_6 = _3;
_6 = 6319652492512291722_i64 as u32;
_3 = !_6;
_16 = -(-124_i8);
_3 = _6;
_6 = _3;
_19.0 = _10.0;
_19.0 = _10.0;
_21 = 26999_u16 as u8;
_19.1 = core::ptr::addr_of_mut!(_21);
_4 = _1;
_3 = RET as u32;
RET = 5_usize & 3_usize;
_8 = 4481227788727713605_i64 as f32;
_10.1 = core::ptr::addr_of_mut!(_21);
_8 = _11;
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
RET = !6_usize;
_7 = [41_i8];
_5 = -_1;
_10.0 = ['\u{79bbc}','\u{b4da7}','\u{e9cba}','\u{f6f10}','\u{d343d}','\u{f9890}','\u{d47f}'];
_3 = _6;
_7 = [(-101_i8)];
RET = 7769672095887175763_u64 as usize;
_5 = _1;
_3 = 9223372036854775807_isize as u32;
_5 = '\u{e764c}' as i16;
_10.0 = ['\u{7a253}','\u{e4e7c}','\u{efbb7}','\u{ce2d4}','\u{c781a}','\u{f07b6}','\u{64c75}'];
_7 = [(-69_i8)];
_10.0 = ['\u{e0af7}','\u{4118c}','\u{254c9}','\u{e5cce}','\u{85fc9}','\u{ea3b9}','\u{4fcaa}'];
_10.0 = ['\u{de170}','\u{101ce5}','\u{c572d}','\u{a637f}','\u{30980}','\u{d9d8b}','\u{5a6c0}'];
_7 = [(-77_i8)];
RET = !5073291111492019507_usize;
_9 = [70_u8,90_u8,58_u8];
_10.0 = ['\u{3fa8c}','\u{872e9}','\u{4c6f9}','\u{646b8}','\u{17fd6}','\u{a052e}','\u{f2000}'];
_1 = !_2;
_11 = -_8;
RET = 15681080092732933927_usize - 2_usize;
RET = 2262720721969983945_usize;
_4 = -_2;
_6 = _3;
_8 = _11;
_6 = _3 - _3;
_4 = !_1;
_1 = _2;
_8 = -_11;
_11 = _8 * _8;
Goto(bb2)
}
bb17 = {
_18 = &_16;
_12 = !_13;
_1 = _2 & _5;
_15.2 = &_24;
_10.0 = ['\u{8608f}','\u{53ea3}','\u{50e22}','\u{88cb0}','\u{6a0bc}','\u{9ed9e}','\u{a02f4}'];
_1 = -_5;
_19.0 = ['\u{8b399}','\u{7928c}','\u{f3610}','\u{20483}','\u{e6e95}','\u{769fa}','\u{7229a}'];
_10 = Move(_19);
_20 = '\u{5818f}';
_9 = [_21,_21,_21];
_24 = _12 as f64;
_14 = _15.1;
_8 = _11;
_1 = !_5;
_15.2 = &_24;
_13 = (*_18) as isize;
_21 = !237_u8;
_12 = false as isize;
_22 = _20;
_9 = [_21,_21,_21];
_2 = _1;
_16 = (-127_i8);
_5 = _4 * _4;
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(5_usize, 4_usize, Move(_4), 2_usize, Move(_2), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(5_usize, 20_usize, Move(_20), 22_usize, Move(_22), 9_usize, Move(_9), 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16) -> u32 {
mir! {
type RET = u32;
let _6: (*mut i128, (&'static i16,), char);
let _7: [u128; 1];
let _8: u16;
let _9: f64;
let _10: f64;
let _11: ([char; 7], *const *mut i128, &'static bool);
let _12: f32;
let _13: u16;
let _14: char;
let _15: f64;
let _16: isize;
let _17: ((i16, usize),);
let _18: i8;
let _19: char;
let _20: f64;
let _21: [u128; 2];
let _22: &'static *const [char; 7];
let _23: char;
let _24: f64;
let _25: &'static bool;
let _26: ();
let _27: ();
{
RET = !1048689614_u32;
_5 = _2;
Call(_2 = fn7(_3, _1, _4, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 221790536460031983523173910291557663854_u128 as i16;
_1 = -_5;
_1 = !_5;
RET = 891166085_u32 - 759403783_u32;
RET = 3019278115_u32;
_2 = !_5;
_3 = -_5;
_2 = -_1;
_6.1.0 = &_4;
_3 = !_1;
RET = '\u{7884a}' as u32;
RET = 3191781740_u32;
_7 = [114465831239155000181633014416665797267_u128];
_5 = _3 + _3;
_6.2 = '\u{8e3e0}';
_3 = (-9223372036854775808_isize) as i16;
Goto(bb2)
}
bb2 = {
_5 = _2;
_7 = [110377049209308323720727938848350243358_u128];
_5 = _1 + _2;
_8 = 40380_u16 << _5;
_4 = 270596463090039119_u64 as i16;
_5 = _2 << _8;
_1 = _5;
_6.2 = '\u{c6ed3}';
_6.1.0 = &_1;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3191781740 => bb7,
_ => bb6
}
}
bb3 = {
_4 = 221790536460031983523173910291557663854_u128 as i16;
_1 = -_5;
_1 = !_5;
RET = 891166085_u32 - 759403783_u32;
RET = 3019278115_u32;
_2 = !_5;
_3 = -_5;
_2 = -_1;
_6.1.0 = &_4;
_3 = !_1;
RET = '\u{7884a}' as u32;
RET = 3191781740_u32;
_7 = [114465831239155000181633014416665797267_u128];
_5 = _3 + _3;
_6.2 = '\u{8e3e0}';
_3 = (-9223372036854775808_isize) as i16;
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
_8 = !14062_u16;
_6.1.0 = &_1;
_1 = _5;
RET = 879617253_u32 - 3439922368_u32;
Goto(bb8)
}
bb8 = {
_6.2 = '\u{8084e}';
_6.2 = '\u{818e1}';
_5 = !_2;
_1 = _2;
Call(_6.2 = fn8(_1, _1, _1, _2, _1, _2, _2, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_4 = RET as i16;
_9 = 161_u8 as f64;
_7 = [25635107317866193041344428079244881125_u128];
_7 = [125792624061645338917295585790352768057_u128];
_8 = 7256_u16;
_1 = 5539774963729140911_u64 as i16;
RET = !3686824064_u32;
_6.1.0 = &_4;
_8 = 59540_u16 + 62212_u16;
_1 = -_2;
_8 = (-4962693111590323227_i64) as u16;
_4 = -_1;
_3 = _4 ^ _2;
_7 = [199516624942180965375983646372407043180_u128];
_6.1.0 = &_1;
_6.2 = '\u{c64}';
_1 = _4;
_11.1 = core::ptr::addr_of!(_6.0);
_12 = (-1945636051_i32) as f32;
_6.1.0 = &_1;
RET = 1080216519_u32 ^ 3484804650_u32;
Goto(bb10)
}
bb10 = {
_6.2 = '\u{b7df9}';
_10 = -_9;
_2 = _3 >> _3;
_1 = _3;
_8 = !2730_u16;
_4 = _3 + _5;
_9 = _12 as f64;
_13 = _8;
_10 = _9 * _9;
_6.2 = '\u{c3931}';
_11.0 = [_6.2,_6.2,_6.2,_6.2,_6.2,_6.2,_6.2];
Goto(bb11)
}
bb11 = {
RET = 1231809098_i32 as u32;
_3 = -_4;
_8 = !_13;
_11.0 = [_6.2,_6.2,_6.2,_6.2,_6.2,_6.2,_6.2];
_6.2 = '\u{6505e}';
_14 = _6.2;
_16 = RET as isize;
_8 = true as u16;
Goto(bb12)
}
bb12 = {
_6.2 = _14;
_11.1 = core::ptr::addr_of!(_6.0);
_17.0 = (_2, 14231300138032000192_usize);
_18 = true as i8;
_16 = (-9223372036854775808_isize);
_17.0.1 = 7_usize | 7_usize;
_4 = _1 + _2;
_11.1 = core::ptr::addr_of!(_6.0);
_11.1 = core::ptr::addr_of!(_6.0);
_5 = !_2;
_16 = _18 as isize;
_2 = 1951202461341542512_i64 as i16;
_14 = _6.2;
_6.2 = _14;
_9 = _10;
RET = 10447000091847498403_u64 as u32;
RET = 2605931078_u32 - 2382990894_u32;
_6.2 = _14;
_15 = -_10;
_5 = _3 >> _4;
_11.1 = core::ptr::addr_of!(_6.0);
_11.0 = [_14,_14,_14,_6.2,_14,_14,_14];
_19 = _6.2;
_13 = _8;
_3 = _4;
_11.1 = core::ptr::addr_of!(_6.0);
Goto(bb13)
}
bb13 = {
_10 = _9 * _15;
_17.0 = (_5, 2_usize);
RET = 477099621_u32;
_12 = (-1273336408682695902_i64) as f32;
_11.1 = core::ptr::addr_of!(_6.0);
_8 = 4735011107186698594_u64 as u16;
_20 = -_15;
_20 = _10;
_20 = _10 * _9;
_11.1 = core::ptr::addr_of!(_6.0);
_5 = !_3;
_6.1.0 = &_1;
_17.0 = (_3, 18377638066628419322_usize);
_21 = [187428199319877561137377783300832032808_u128,205573120962773971750208841111029857057_u128];
_17.0 = (_1, 4_usize);
Goto(bb14)
}
bb14 = {
_4 = _1 & _17.0.0;
_7 = [193292379385157831932293091994628754532_u128];
_4 = -_1;
_12 = (-98170725_i32) as f32;
_15 = (-5542475002741736539_i64) as f64;
_11.0 = [_19,_19,_6.2,_14,_14,_19,_14];
_14 = _6.2;
_6.1.0 = &_4;
_19 = _14;
RET = 2820077097_u32;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(6_usize, 14_usize, Move(_14), 19_usize, Move(_19), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(6_usize, 18_usize, Move(_18), 8_usize, Move(_8), 21_usize, Move(_21), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16) -> i16 {
mir! {
type RET = i16;
let _5: (&'static i16,);
let _6: [i8; 1];
let _7: *const *mut u8;
let _8: [i8; 1];
let _9: ();
let _10: ();
{
_3 = -_2;
_3 = _2;
RET = !_1;
_2 = _4 & _1;
_2 = -_4;
RET = 3_usize as i16;
RET = -_2;
_5.0 = &_2;
RET = _1;
RET = !_4;
_4 = RET;
_4 = _2;
RET = _4 << _3;
_6 = [(-127_i8)];
_6 = [(-23_i8)];
_8 = [10_i8];
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 6_usize, Move(_6), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: i16,mut _8: i16) -> char {
mir! {
type RET = char;
let _9: [u32; 4];
let _10: &'static *mut i64;
let _11: *mut ([char; 7], *mut u8);
let _12: [isize; 2];
let _13: *const f64;
let _14: *mut i128;
let _15: Adt39;
let _16: *mut isize;
let _17: [char; 4];
let _18: isize;
let _19: char;
let _20: f64;
let _21: f32;
let _22: ([i32; 8],);
let _23: [u8; 3];
let _24: [i8; 2];
let _25: bool;
let _26: *const usize;
let _27: u32;
let _28: *mut i64;
let _29: ();
let _30: ();
{
_2 = _4 * _4;
RET = '\u{178c1}';
_8 = true as i16;
RET = '\u{d9c63}';
_1 = -_4;
_6 = 147_u8 as i16;
_5 = 3185722206_u32 as i16;
_9 = [713602505_u32,2009086154_u32,4143651226_u32,1478679027_u32];
_6 = -_4;
_7 = 63318788909893554247983109227935845255_i128 as i16;
_8 = -_2;
_5 = _8;
RET = '\u{b6c6d}';
_9 = [685705700_u32,2550408014_u32,2837246067_u32,2072360108_u32];
Goto(bb1)
}
bb1 = {
_12 = [(-40_isize),9223372036854775807_isize];
_12 = [(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _1 ^ _1;
_5 = _6 - _3;
_7 = _4 << _1;
_12 = [48_isize,119_isize];
_15.fld0.1 = 18315465275614950027_usize << _7;
_8 = !_5;
_15.fld0 = (_4, 5399013663661319729_usize);
_8 = _4;
_12 = [(-75_isize),9223372036854775807_isize];
_7 = _3;
_15.fld0 = (_8, 1_usize);
_4 = 120_i8 as i16;
_15.fld0.0 = (-42817748729092764829969290925732398882_i128) as i16;
_6 = -_1;
match _15.fld0.1 {
0 => bb2,
1 => bb5,
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
_4 = !_3;
Goto(bb6)
}
bb6 = {
_15.fld0.0 = -_4;
RET = '\u{1e0bd}';
_7 = !_5;
RET = '\u{47534}';
_17 = [RET,RET,RET,RET];
_15.fld0 = (_7, 16984671587350544412_usize);
_16 = core::ptr::addr_of_mut!(_18);
(*_16) = 9223372036854775807_isize ^ 9223372036854775807_isize;
_18 = !(-9223372036854775808_isize);
_1 = -_15.fld0.0;
(*_16) = 9223372036854775807_isize ^ 9223372036854775807_isize;
_5 = -_8;
_15.fld0 = (_7, 17472216863935891524_usize);
(*_16) = 9_isize;
_15.fld0 = (_7, 268329898477936905_usize);
_15.fld0.1 = RET as usize;
_5 = _15.fld0.1 as i16;
_20 = 3054260978_u32 as f64;
(*_16) = _20 as isize;
Call(_11 = fn9(_15.fld0.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = 56_u8 as i16;
_3 = _2;
_17 = [RET,RET,RET,RET];
_3 = _15.fld0.1 as i16;
_21 = 7_u8 as f32;
_21 = 2248010289_u32 as f32;
_17 = [RET,RET,RET,RET];
(*_16) = !61_isize;
_2 = _15.fld0.0 - _15.fld0.0;
_15.fld0 = (_1, 896199770806632407_usize);
_19 = RET;
RET = _19;
_19 = RET;
_6 = _18 as i16;
RET = _19;
_7 = _2;
match _15.fld0.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
896199770806632407 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
_12 = [(-40_isize),9223372036854775807_isize];
_12 = [(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _1 ^ _1;
_5 = _6 - _3;
_7 = _4 << _1;
_12 = [48_isize,119_isize];
_15.fld0.1 = 18315465275614950027_usize << _7;
_8 = !_5;
_15.fld0 = (_4, 5399013663661319729_usize);
_8 = _4;
_12 = [(-75_isize),9223372036854775807_isize];
_7 = _3;
_15.fld0 = (_8, 1_usize);
_4 = 120_i8 as i16;
_15.fld0.0 = (-42817748729092764829969290925732398882_i128) as i16;
_6 = -_1;
match _15.fld0.1 {
0 => bb2,
1 => bb5,
_ => bb4
}
}
bb10 = {
_23 = [35_u8,59_u8,164_u8];
_19 = RET;
_15.fld0.1 = RET as usize;
RET = _19;
_21 = _15.fld0.1 as f32;
_5 = _1;
_4 = 1916236584_i32 as i16;
_8 = !_15.fld0.0;
_22.0 = [158573453_i32,419125113_i32,(-198903325_i32),1905197401_i32,1467110448_i32,(-1735919624_i32),1620149072_i32,(-1281447826_i32)];
_22.0 = [1537542303_i32,(-4586179_i32),(-2003861096_i32),530279083_i32,1437968675_i32,304816827_i32,1682230935_i32,(-1295589065_i32)];
RET = _19;
_3 = !_5;
_8 = _5 - _15.fld0.0;
_6 = false as i16;
_19 = RET;
_8 = !_5;
(*_16) = true as isize;
_26 = core::ptr::addr_of!(_15.fld0.1);
(*_16) = (-9223372036854775808_isize);
_26 = core::ptr::addr_of!((*_26));
_13 = core::ptr::addr_of!(_20);
(*_13) = (-5749371588061059630021120108589171476_i128) as f64;
_19 = RET;
_6 = _3;
_15.fld0 = (_3, 5931842188628419304_usize);
_23 = [49_u8,240_u8,230_u8];
match (*_26) {
0 => bb6,
1 => bb11,
2 => bb12,
3 => bb13,
5931842188628419304 => bb15,
_ => bb14
}
}
bb11 = {
_12 = [(-40_isize),9223372036854775807_isize];
_12 = [(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _1 ^ _1;
_5 = _6 - _3;
_7 = _4 << _1;
_12 = [48_isize,119_isize];
_15.fld0.1 = 18315465275614950027_usize << _7;
_8 = !_5;
_15.fld0 = (_4, 5399013663661319729_usize);
_8 = _4;
_12 = [(-75_isize),9223372036854775807_isize];
_7 = _3;
_15.fld0 = (_8, 1_usize);
_4 = 120_i8 as i16;
_15.fld0.0 = (-42817748729092764829969290925732398882_i128) as i16;
_6 = -_1;
match _15.fld0.1 {
0 => bb2,
1 => bb5,
_ => bb4
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_12 = [(-40_isize),9223372036854775807_isize];
_12 = [(-9223372036854775808_isize),9223372036854775807_isize];
_8 = _1 ^ _1;
_5 = _6 - _3;
_7 = _4 << _1;
_12 = [48_isize,119_isize];
_15.fld0.1 = 18315465275614950027_usize << _7;
_8 = !_5;
_15.fld0 = (_4, 5399013663661319729_usize);
_8 = _4;
_12 = [(-75_isize),9223372036854775807_isize];
_7 = _3;
_15.fld0 = (_8, 1_usize);
_4 = 120_i8 as i16;
_15.fld0.0 = (-42817748729092764829969290925732398882_i128) as i16;
_6 = -_1;
match _15.fld0.1 {
0 => bb2,
1 => bb5,
_ => bb4
}
}
bb15 = {
RET = _19;
_25 = false;
_1 = _6 | _8;
_24 = [100_i8,39_i8];
_6 = _8 & _15.fld0.0;
_2 = _20 as i16;
_1 = _3 >> _5;
(*_26) = !10860435015888463097_usize;
_25 = false;
_2 = (-5570716341744257062_i64) as i16;
_10 = &_28;
_1 = _6;
_1 = _6;
_17 = [RET,RET,_19,_19];
RET = _19;
_19 = RET;
_15.fld0.0 = _1;
(*_26) = (-50_i8) as usize;
(*_13) = _21 as f64;
_18 = !9223372036854775807_isize;
_9 = [1197580461_u32,608732040_u32,2246093594_u32,1714708567_u32];
_16 = core::ptr::addr_of_mut!((*_16));
_6 = -_1;
_24 = [(-11_i8),1_i8];
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(8_usize, 18_usize, Move(_18), 1_usize, Move(_1), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(8_usize, 25_usize, Move(_25), 24_usize, Move(_24), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i16) -> *mut ([char; 7], *mut u8) {
mir! {
type RET = *mut ([char; 7], *mut u8);
let _2: i32;
let _3: char;
let _4: [isize; 5];
let _5: isize;
let _6: [isize; 5];
let _7: u64;
let _8: ([char; 7], *mut u8);
let _9: Adt47;
let _10: u128;
let _11: bool;
let _12: f64;
let _13: [u32; 4];
let _14: bool;
let _15: Adt24;
let _16: bool;
let _17: &'static &'static i16;
let _18: Adt47;
let _19: char;
let _20: char;
let _21: *mut ([char; 7], *mut u8);
let _22: f32;
let _23: u64;
let _24: isize;
let _25: (i16, usize);
let _26: isize;
let _27: bool;
let _28: ();
let _29: ();
{
_1 = 32497_i16;
_1 = -(-7623_i16);
Goto(bb1)
}
bb1 = {
_1 = !(-19354_i16);
_3 = '\u{7a86f}';
_1 = 52828_u16 as i16;
_3 = '\u{87cb0}';
_5 = (-109_isize) << _1;
_2 = !(-45903205_i32);
_4 = [_5,_5,_5,_5,_5];
_2 = !(-59119409_i32);
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_7 = 59_u8 as u64;
RET = core::ptr::addr_of_mut!(_8);
_6 = [_5,_5,_5,_5,_5];
RET = core::ptr::addr_of_mut!((*RET));
_2 = 0_usize as i32;
_3 = '\u{d9529}';
RET = core::ptr::addr_of_mut!((*RET));
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_4 = [_5,_5,_5,_5,_5];
_5 = -9223372036854775807_isize;
_6 = _4;
_3 = '\u{7e11e}';
_3 = '\u{13512}';
Goto(bb2)
}
bb2 = {
_4 = [_5,_5,_5,_5,_5];
RET = core::ptr::addr_of_mut!((*RET));
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb3)
}
bb3 = {
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_1 = _2 as i16;
_3 = '\u{7a246}';
_7 = !8047002702915738817_u64;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [_5,_5,_5,_5,_5];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_6 = _4;
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_5 = 9223372036854775807_isize << _2;
_2 = !(-503285225_i32);
_2 = 784390622_i32 | 1480548792_i32;
Call((*RET) = fn10(_3, _4, _3, _1, _7, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb5)
}
bb5 = {
_5 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = core::ptr::addr_of_mut!(_8);
_7 = 12768197785356817896_u64 >> _1;
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_11 = _1 <= _1;
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
RET = core::ptr::addr_of_mut!(_8);
_5 = (-9223372036854775808_isize) << _2;
_10 = 78260673291366594066325672644225307556_u128;
RET = core::ptr::addr_of_mut!(_8);
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_4 = [_5,_5,_5,_5,_5];
_1 = -18423_i16;
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
78260673291366594066325672644225307556 => bb9,
_ => bb8
}
}
bb6 = {
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb5)
}
bb7 = {
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_1 = _2 as i16;
_3 = '\u{7a246}';
_7 = !8047002702915738817_u64;
RET = core::ptr::addr_of_mut!((*RET));
_4 = [_5,_5,_5,_5,_5];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_6 = _4;
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_5 = 9223372036854775807_isize << _2;
_2 = !(-503285225_i32);
_2 = 784390622_i32 | 1480548792_i32;
Call((*RET) = fn10(_3, _4, _3, _1, _7, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1 = !(-19354_i16);
_3 = '\u{7a86f}';
_1 = 52828_u16 as i16;
_3 = '\u{87cb0}';
_5 = (-109_isize) << _1;
_2 = !(-45903205_i32);
_4 = [_5,_5,_5,_5,_5];
_2 = !(-59119409_i32);
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_7 = 59_u8 as u64;
RET = core::ptr::addr_of_mut!(_8);
_6 = [_5,_5,_5,_5,_5];
RET = core::ptr::addr_of_mut!((*RET));
_2 = 0_usize as i32;
_3 = '\u{d9529}';
RET = core::ptr::addr_of_mut!((*RET));
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_4 = [_5,_5,_5,_5,_5];
_5 = -9223372036854775807_isize;
_6 = _4;
_3 = '\u{7e11e}';
_3 = '\u{13512}';
Goto(bb2)
}
bb9 = {
_10 = (-3561593342611507633_i64) as u128;
RET = core::ptr::addr_of_mut!((*RET));
_11 = !true;
_12 = _5 as f64;
_15.fld0 = !_2;
_13 = [1466735058_u32,74766871_u32,2392315835_u32,2661848043_u32];
RET = core::ptr::addr_of_mut!(_8);
_14 = _11;
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
RET = core::ptr::addr_of_mut!((*RET));
_12 = 7174909208588961738_i64 as f64;
_1 = _10 as i16;
_13 = [516652930_u32,477401168_u32,4034400985_u32,982623068_u32];
_5 = (-9223372036854775808_isize) + 9223372036854775807_isize;
Goto(bb10)
}
bb10 = {
_1 = (-25585_i16) * (-833_i16);
_10 = 31715_u16 as u128;
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_5 = -(-9223372036854775808_isize);
RET = core::ptr::addr_of_mut!(_8);
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_2 = _10 as i32;
_6 = _4;
_1 = _12 as i16;
_15.fld0 = -_2;
RET = core::ptr::addr_of_mut!((*RET));
_13 = [634505757_u32,230746738_u32,3753853272_u32,2973582790_u32];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
_6 = [_5,_5,_5,_5,_5];
(*RET).0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb11)
}
bb11 = {
_13 = [183613292_u32,311534511_u32,3040136062_u32,671788740_u32];
_8.0 = [_3,_3,_3,_3,_3,_3,_3];
_16 = !_11;
_19 = _3;
_13 = [995999406_u32,527288961_u32,884849497_u32,2938170790_u32];
_4 = [_5,_5,_5,_5,_5];
(*RET).0 = [_3,_19,_3,_19,_3,_19,_3];
_10 = 133156894580311854585516814192945832444_u128 | 204040764145982686468013967573107552907_u128;
_20 = _3;
_10 = 31823337549671772693049912887327206149_i128 as u128;
_15 = Adt24 { fld0: _2 };
(*RET).0 = [_3,_19,_19,_19,_3,_20,_20];
_4 = [_5,_5,_5,_5,_5];
_8.0 = [_3,_20,_3,_20,_3,_20,_19];
_8.0 = [_20,_19,_20,_20,_3,_20,_20];
_19 = _20;
_8.0 = [_3,_3,_20,_19,_3,_3,_3];
_21 = core::ptr::addr_of_mut!((*RET));
_5 = _14 as isize;
_3 = _20;
_1 = _16 as i16;
_23 = (-102663376896005066331031830529002781924_i128) as u64;
(*RET).0 = [_20,_19,_20,_19,_3,_3,_3];
_3 = _20;
_12 = 77731021634691214771662995019581139278_i128 as f64;
_8.0 = [_3,_20,_19,_19,_20,_20,_20];
Call(_1 = core::intrinsics::bswap((-31398_i16)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*RET).0 = [_20,_20,_3,_20,_19,_19,_19];
Goto(bb13)
}
bb13 = {
_2 = _15.fld0 | _15.fld0;
_22 = _7 as f32;
(*RET).0 = [_3,_19,_19,_3,_3,_3,_3];
_4 = _6;
_13 = [1925061817_u32,3118983631_u32,257265064_u32,1731538960_u32];
(*RET).0 = [_19,_20,_19,_3,_3,_3,_19];
_16 = _14;
_2 = _19 as i32;
_3 = _20;
_19 = _3;
_12 = _1 as f64;
_11 = !_16;
(*RET).0 = [_3,_3,_19,_3,_19,_3,_20];
_13 = [4045042302_u32,794162023_u32,1760243806_u32,206626294_u32];
_14 = !_16;
_5 = (-9223372036854775808_isize) | 9223372036854775807_isize;
(*_21).0 = [_20,_20,_19,_19,_3,_20,_20];
_22 = _10 as f32;
_14 = !_11;
(*RET).0 = [_19,_19,_19,_20,_19,_19,_3];
_7 = !_23;
_2 = -_15.fld0;
_21 = Move(RET);
_4 = [_5,_5,_5,_5,_5];
_11 = _14;
RET = Move(_21);
RET = core::ptr::addr_of_mut!(_8);
_1 = -(-29593_i16);
_10 = 52115888037252733090981085158865746000_u128;
_19 = _20;
Goto(bb14)
}
bb14 = {
_21 = Move(RET);
_13 = [270499579_u32,2509243706_u32,306214516_u32,978029858_u32];
_25.1 = 2_usize >> _10;
_15 = Adt24 { fld0: _2 };
RET = core::ptr::addr_of_mut!(_8);
_24 = !_5;
_11 = !_16;
_13 = [3517889797_u32,1932215752_u32,3349900212_u32,4074890473_u32];
_24 = _5 & _5;
_19 = _20;
RET = core::ptr::addr_of_mut!(_8);
_11 = !_16;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(9_usize, 5_usize, Move(_5), 20_usize, Move(_20), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(9_usize, 24_usize, Move(_24), 19_usize, Move(_19), 1_usize, Move(_1), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: char,mut _2: [isize; 5],mut _3: char,mut _4: i16,mut _5: u64,mut _6: isize,mut _7: isize) -> ([char; 7], *mut u8) {
mir! {
type RET = ([char; 7], *mut u8);
let _8: isize;
let _9: u8;
let _10: *const [char; 7];
let _11: isize;
let _12: [isize; 2];
let _13: &'static &'static i16;
let _14: *mut *mut u8;
let _15: (&'static i16,);
let _16: i128;
let _17: *mut i8;
let _18: *mut Adt24;
let _19: [char; 4];
let _20: &'static i8;
let _21: isize;
let _22: &'static &'static bool;
let _23: (u128, (&'static i16,));
let _24: isize;
let _25: isize;
let _26: u128;
let _27: f32;
let _28: ();
let _29: ();
{
_3 = _1;
_2 = [_7,_6,_7,_6,_6];
_8 = -_6;
RET.0 = [_3,_3,_3,_3,_1,_1,_1];
_1 = _3;
RET.1 = core::ptr::addr_of_mut!(_9);
RET.0 = [_3,_3,_3,_1,_1,_3,_1];
RET.0 = [_1,_1,_3,_1,_1,_3,_3];
_10 = core::ptr::addr_of!(RET.0);
_10 = core::ptr::addr_of!((*_10));
Goto(bb1)
}
bb1 = {
_9 = 131_u8;
RET.0 = [_1,_3,_1,_1,_1,_1,_3];
_6 = -_7;
_12 = [_8,_8];
RET.1 = core::ptr::addr_of_mut!(_9);
(*_10) = [_1,_1,_3,_3,_3,_3,_1];
match _9 {
0 => bb2,
131 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_10 = core::ptr::addr_of!((*_10));
_1 = _3;
RET.1 = core::ptr::addr_of_mut!(_9);
Call(_4 = fn11((*_10), _6, _2, _6, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET.0 = [_3,_1,_3,_1,_3,_3,_3];
Goto(bb6)
}
bb6 = {
_12 = [_7,_8];
_15.0 = &_4;
_11 = 308364221878771435679913282222655801778_u128 as isize;
_15.0 = &_4;
_13 = &_15.0;
_1 = _3;
Goto(bb7)
}
bb7 = {
(*_10) = [_1,_3,_1,_1,_1,_1,_1];
_5 = false as u64;
RET.0 = [_1,_1,_1,_1,_3,_3,_3];
_6 = _7 >> _4;
RET.0 = [_1,_1,_1,_3,_1,_3,_3];
_16 = _5 as i128;
RET.0 = [_1,_3,_1,_3,_3,_3,_3];
_21 = 6_usize as isize;
_2 = [_7,_6,_6,_6,_7];
_19 = [_3,_1,_3,_3];
_8 = _6 * _7;
_6 = _7 ^ _7;
_19 = [_3,_1,_1,_1];
_1 = _3;
_3 = _1;
RET.0 = [_3,_3,_3,_3,_1,_1,_1];
match _9 {
0 => bb5,
1 => bb6,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
131 => bb14,
_ => bb13
}
}
bb8 = {
_12 = [_7,_8];
_15.0 = &_4;
_11 = 308364221878771435679913282222655801778_u128 as isize;
_15.0 = &_4;
_13 = &_15.0;
_1 = _3;
Goto(bb7)
}
bb9 = {
RET.0 = [_3,_1,_3,_1,_3,_3,_3];
Goto(bb6)
}
bb10 = {
_10 = core::ptr::addr_of!((*_10));
_1 = _3;
RET.1 = core::ptr::addr_of_mut!(_9);
Call(_4 = fn11((*_10), _6, _2, _6, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_9 = 131_u8;
RET.0 = [_1,_3,_1,_1,_1,_1,_3];
_6 = -_7;
_12 = [_8,_8];
RET.1 = core::ptr::addr_of_mut!(_9);
(*_10) = [_1,_1,_3,_3,_3,_3,_1];
match _9 {
0 => bb2,
131 => bb4,
_ => bb3
}
}
bb14 = {
_15.0 = &_4;
_21 = _11;
_10 = core::ptr::addr_of!((*_10));
_1 = _3;
_15.0 = &_4;
_15.0 = &_4;
_15.0 = &_4;
_9 = 188_u8 | 0_u8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(10_usize, 1_usize, Move(_1), 3_usize, Move(_3), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(10_usize, 6_usize, Move(_6), 5_usize, Move(_5), 21_usize, Move(_21), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [char; 7],mut _2: isize,mut _3: [isize; 5],mut _4: isize,mut _5: isize) -> i16 {
mir! {
type RET = i16;
let _6: [isize; 2];
let _7: *mut ([char; 7], *mut u8);
let _8: i16;
let _9: f64;
let _10: ((i16, usize),);
let _11: Adt58;
let _12: u8;
let _13: *mut u8;
let _14: [u8; 5];
let _15: isize;
let _16: bool;
let _17: [u64; 6];
let _18: [u32; 4];
let _19: isize;
let _20: *mut isize;
let _21: ();
let _22: ();
{
_5 = _4;
Goto(bb1)
}
bb1 = {
_3 = [_2,_5,_5,_4,_2];
RET = 27936_i16 << _2;
_5 = _2;
_2 = 232171673226678326181941397674304627690_u128 as isize;
_1 = ['\u{8cb5}','\u{87fe0}','\u{2bd95}','\u{e15e2}','\u{f255e}','\u{9d106}','\u{f8fb9}'];
_5 = !_2;
_6 = [_5,_5];
RET = 12209_i16 >> _2;
_8 = RET ^ RET;
_9 = 70_i8 as f64;
_3 = [_4,_2,_2,_5,_4];
_4 = _2 << RET;
_6 = [_2,_5];
_1 = ['\u{83d33}','\u{1a63b}','\u{ecf33}','\u{4deae}','\u{1174f}','\u{9815d}','\u{ee4c4}'];
Goto(bb2)
}
bb2 = {
_5 = _2 - _4;
_6 = [_4,_5];
_4 = _5;
Call(_8 = fn12(_5, _1, _1, _5, _4, _6, _1, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = 164930008399388344700040075309102149195_i128 as i16;
_2 = _8 as isize;
_1 = ['\u{dcd47}','\u{fec32}','\u{d108a}','\u{db104}','\u{c9e68}','\u{3c708}','\u{25f47}'];
_8 = !RET;
_2 = -_5;
_10.0.0 = -RET;
_8 = RET >> RET;
_6 = [_4,_4];
_10.0.0 = -RET;
_11.fld0 = Adt24 { fld0: 2014797128_i32 };
_10.0 = (_8, 1_usize);
_9 = 59643_u16 as f64;
_5 = _4;
_12 = 98_u8;
_4 = !_5;
_2 = (-16_i8) as isize;
_11.fld2 = [_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0];
_8 = RET - _10.0.0;
match _11.fld0.fld0 {
0 => bb1,
1 => bb4,
2014797128 => bb6,
_ => bb5
}
}
bb4 = {
_5 = _2 - _4;
_6 = [_4,_5];
_4 = _5;
Call(_8 = fn12(_5, _1, _1, _5, _4, _6, _1, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3 = [_2,_5,_5,_4,_2];
RET = 27936_i16 << _2;
_5 = _2;
_2 = 232171673226678326181941397674304627690_u128 as isize;
_1 = ['\u{8cb5}','\u{87fe0}','\u{2bd95}','\u{e15e2}','\u{f255e}','\u{9d106}','\u{f8fb9}'];
_5 = !_2;
_6 = [_5,_5];
RET = 12209_i16 >> _2;
_8 = RET ^ RET;
_9 = 70_i8 as f64;
_3 = [_4,_2,_2,_5,_4];
_4 = _2 << RET;
_6 = [_2,_5];
_1 = ['\u{83d33}','\u{1a63b}','\u{ecf33}','\u{4deae}','\u{1174f}','\u{9815d}','\u{ee4c4}'];
Goto(bb2)
}
bb6 = {
_13 = core::ptr::addr_of_mut!(_12);
_11.fld1 = core::ptr::addr_of_mut!((*_13));
_11.fld3 = [10544714632069443699_u64,16574666958204334677_u64,6109107399846847769_u64,5752691420863377522_u64,12514234971645104622_u64,12681093917201065735_u64];
_9 = (-47_i8) as f64;
_1 = ['\u{cf57c}','\u{52879}','\u{b5570}','\u{86538}','\u{252b8}','\u{35d4e}','\u{9e4ad}'];
(*_13) = !185_u8;
_15 = _5 ^ _5;
_5 = _15;
_10.0 = (_8, 6_usize);
_15 = -_4;
_14 = [(*_13),(*_13),_12,_12,(*_13)];
_3 = [_5,_5,_5,_15,_5];
_18 = [2606405542_u32,1528545838_u32,1878579659_u32,4156878786_u32];
_10.0.0 = RET << _10.0.1;
_19 = _5;
_1 = ['\u{31978}','\u{9158a}','\u{7efef}','\u{1323}','\u{155a9}','\u{edb6e}','\u{5c16b}'];
RET = -_8;
_10.0.0 = RET * _8;
RET = _10.0.0 - _8;
_11.fld2 = [_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0,_11.fld0.fld0];
_17 = [17913427363962427675_u64,12780778303869459242_u64,12747660845929009191_u64,6226492769937870787_u64,15475110114692998076_u64,4120665706813104114_u64];
Goto(bb7)
}
bb7 = {
Call(_21 = dump_var(11_usize, 3_usize, Move(_3), 2_usize, Move(_2), 15_usize, Move(_15), 12_usize, Move(_12)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_21 = dump_var(11_usize, 17_usize, Move(_17), 8_usize, Move(_8), 6_usize, Move(_6), 22_usize, _22), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: [char; 7],mut _3: [char; 7],mut _4: isize,mut _5: isize,mut _6: [isize; 2],mut _7: [char; 7],mut _8: [isize; 5],mut _9: isize) -> i16 {
mir! {
type RET = i16;
let _10: Adt60;
let _11: [i32; 8];
let _12: [i32; 7];
let _13: i128;
let _14: [u64; 7];
let _15: bool;
let _16: &'static (*mut i128, (&'static i16,), char);
let _17: bool;
let _18: ((i16, usize),);
let _19: [u64; 6];
let _20: [u8; 3];
let _21: i64;
let _22: *const *mut i128;
let _23: u8;
let _24: ();
let _25: ();
{
RET = (-3461_i16) + (-1393_i16);
_5 = -_1;
_2 = _3;
Call(_8 = fn13(_5, _7, _6, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _5 & _4;
RET = !2752_i16;
_1 = '\u{5234e}' as isize;
_3 = ['\u{6947f}','\u{5a937}','\u{a41e0}','\u{c0f50}','\u{f631e}','\u{43b8c}','\u{453a1}'];
_6 = [_9,_4];
_3 = ['\u{9a1e5}','\u{58121}','\u{13b57}','\u{c2e0e}','\u{10f05f}','\u{53b18}','\u{17530}'];
_4 = !_5;
Call(_8 = fn16(_9, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [_5,_9,_4,_4,_5];
_8 = [_9,_5,_1,_4,_5];
Goto(bb3)
}
bb3 = {
_5 = _4 >> _9;
_2 = ['\u{1b22a}','\u{7995b}','\u{7d99}','\u{57027}','\u{922e1}','\u{343ed}','\u{10ae74}'];
_4 = _5;
RET = -(-23629_i16);
RET = (-19351_i16);
RET = 10819_i16;
RET = (-1660_i16) - 28533_i16;
Goto(bb4)
}
bb4 = {
_8 = [_5,_5,_4,_5,_4];
_9 = _4 + _4;
_4 = !_5;
_2 = _3;
_2 = ['\u{71762}','\u{c6b22}','\u{93035}','\u{d7865}','\u{594e6}','\u{6a4cc}','\u{e3ea7}'];
RET = (-20342_i16);
RET = 54332_u16 as i16;
_8 = [_9,_4,_9,_4,_9];
_6 = [_4,_4];
_11 = [(-1369763740_i32),(-1072084636_i32),95175544_i32,(-2019033902_i32),1157134370_i32,(-342413880_i32),(-552618104_i32),549709296_i32];
_3 = ['\u{ccc6a}','\u{34766}','\u{efa3a}','\u{d80be}','\u{628cc}','\u{56fb4}','\u{9be3}'];
_2 = _3;
_11 = [603689649_i32,617744448_i32,447848176_i32,(-1113239043_i32),1577079473_i32,1632068563_i32,(-1798721677_i32),1651726738_i32];
_4 = -_5;
_7 = _2;
_1 = (-2412829807543306467022228205847696987_i128) as isize;
_7 = ['\u{1fbb9}','\u{9e83a}','\u{f78fc}','\u{d5129}','\u{705a}','\u{106da7}','\u{3547a}'];
_7 = ['\u{a3cb}','\u{c7db3}','\u{e5c1}','\u{d78b0}','\u{c6d24}','\u{4cd71}','\u{40123}'];
_2 = ['\u{cb29a}','\u{24094}','\u{e3bfd}','\u{b2ba6}','\u{97614}','\u{9aad8}','\u{7489}'];
_2 = ['\u{868e5}','\u{f2bbc}','\u{72b20}','\u{d9abb}','\u{c823c}','\u{101538}','\u{e368d}'];
Goto(bb5)
}
bb5 = {
_5 = !_9;
Call(_9 = fn19(), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14 = [7794487883908706864_u64,1145844874995950614_u64,16363197094782517604_u64,16282583366388976586_u64,14039315520653673860_u64,8370745062878339915_u64,16159600440676521426_u64];
_9 = _5 - _4;
_13 = -(-115351298711777980703910289913868787951_i128);
_4 = _9;
_5 = _9 - _9;
_12 = [858286664_i32,(-4113301_i32),(-1597913417_i32),(-890222493_i32),549704647_i32,906605861_i32,576211844_i32];
_14 = [14446858709673114935_u64,10068339969738158849_u64,15664107073133113855_u64,15621390999877625536_u64,13478742083890622572_u64,18137517981147560743_u64,2300436130515496559_u64];
_1 = !_4;
_14 = [13493762962720122503_u64,12023711010918194231_u64,6903144336479346685_u64,15561208236087746397_u64,17748261891286828647_u64,16630919406030814568_u64,85977090522156048_u64];
_15 = true;
_11 = [1222425617_i32,370130565_i32,(-1276264174_i32),1393975177_i32,(-140045292_i32),(-1556785951_i32),524625251_i32,(-630594657_i32)];
_14 = [9605216276165856012_u64,7523654565373373375_u64,10404946316443402146_u64,311647358817251943_u64,1585002271361474982_u64,15306379642246172820_u64,15442675515833439542_u64];
_4 = !_1;
_18.0.0 = RET;
_18.0.0 = -RET;
_17 = _15;
_14 = [2614683906256947435_u64,3278981924591998488_u64,6394104773410154706_u64,15930252576723469763_u64,7969077700530815038_u64,16466355878362877511_u64,2582708354090848907_u64];
_15 = !_17;
_18.0.1 = 8207576884585452100_usize;
_3 = ['\u{2a1fe}','\u{fcbe6}','\u{e7009}','\u{db79e}','\u{36bc9}','\u{7cbfd}','\u{5e2fd}'];
_9 = _5;
_1 = _9;
_18.0 = (RET, 4490141746501733168_usize);
_17 = _15;
_6 = [_5,_9];
match _18.0.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
4490141746501733168 => bb9,
_ => bb8
}
}
bb7 = {
_5 = _4 >> _9;
_2 = ['\u{1b22a}','\u{7995b}','\u{7d99}','\u{57027}','\u{922e1}','\u{343ed}','\u{10ae74}'];
_4 = _5;
RET = -(-23629_i16);
RET = (-19351_i16);
RET = 10819_i16;
RET = (-1660_i16) - 28533_i16;
Goto(bb4)
}
bb8 = {
_8 = [_5,_5,_4,_5,_4];
_9 = _4 + _4;
_4 = !_5;
_2 = _3;
_2 = ['\u{71762}','\u{c6b22}','\u{93035}','\u{d7865}','\u{594e6}','\u{6a4cc}','\u{e3ea7}'];
RET = (-20342_i16);
RET = 54332_u16 as i16;
_8 = [_9,_4,_9,_4,_9];
_6 = [_4,_4];
_11 = [(-1369763740_i32),(-1072084636_i32),95175544_i32,(-2019033902_i32),1157134370_i32,(-342413880_i32),(-552618104_i32),549709296_i32];
_3 = ['\u{ccc6a}','\u{34766}','\u{efa3a}','\u{d80be}','\u{628cc}','\u{56fb4}','\u{9be3}'];
_2 = _3;
_11 = [603689649_i32,617744448_i32,447848176_i32,(-1113239043_i32),1577079473_i32,1632068563_i32,(-1798721677_i32),1651726738_i32];
_4 = -_5;
_7 = _2;
_1 = (-2412829807543306467022228205847696987_i128) as isize;
_7 = ['\u{1fbb9}','\u{9e83a}','\u{f78fc}','\u{d5129}','\u{705a}','\u{106da7}','\u{3547a}'];
_7 = ['\u{a3cb}','\u{c7db3}','\u{e5c1}','\u{d78b0}','\u{c6d24}','\u{4cd71}','\u{40123}'];
_2 = ['\u{cb29a}','\u{24094}','\u{e3bfd}','\u{b2ba6}','\u{97614}','\u{9aad8}','\u{7489}'];
_2 = ['\u{868e5}','\u{f2bbc}','\u{72b20}','\u{d9abb}','\u{c823c}','\u{101538}','\u{e368d}'];
Goto(bb5)
}
bb9 = {
_18.0.0 = 5749644907890985824_u64 as i16;
_19 = [1197245895405705145_u64,16541603805698633491_u64,5064853277597172660_u64,16991887166428677081_u64,5517230948385873627_u64,15781649527130215006_u64];
_5 = _9;
_2 = ['\u{ac9fc}','\u{ba03}','\u{d0989}','\u{105e61}','\u{ff00}','\u{191af}','\u{aae2e}'];
_3 = _7;
_3 = ['\u{d523e}','\u{8875c}','\u{12438}','\u{4b674}','\u{c7889}','\u{a064e}','\u{4a500}'];
_18.0 = (RET, 9371112896658341414_usize);
RET = _18.0.0 << _1;
_6 = [_5,_9];
_7 = _3;
_2 = ['\u{f4ffe}','\u{81689}','\u{40b25}','\u{6b6cb}','\u{ca401}','\u{69f7e}','\u{d0aa5}'];
_12 = [495389198_i32,1653760567_i32,1679026403_i32,(-2033672848_i32),(-469838445_i32),969439633_i32,(-1032575971_i32)];
_2 = ['\u{4c7ad}','\u{5ea68}','\u{edaee}','\u{c993a}','\u{f588}','\u{100b48}','\u{bdc18}'];
_18.0.1 = !5706426247013532889_usize;
_13 = (-34_i8) as i128;
_3 = ['\u{d1d53}','\u{cf056}','\u{d55fb}','\u{6f61}','\u{da6b8}','\u{59f02}','\u{1006f1}'];
_4 = _9 << _1;
_18.0 = (RET, 7915184141240082929_usize);
_18.0 = (RET, 3_usize);
_3 = ['\u{d09ce}','\u{bb8e3}','\u{9cd4f}','\u{31551}','\u{e566f}','\u{38d8a}','\u{f7fed}'];
_19 = [14682950876728455783_u64,16899092682014645882_u64,12672826140307948441_u64,12257096685421911745_u64,18036520570155128723_u64,1014712966307172936_u64];
_15 = _17;
_1 = 302063051956191866193517584774544771678_u128 as isize;
_2 = ['\u{e4749}','\u{5ddb2}','\u{8fd4e}','\u{739d6}','\u{c7ae1}','\u{d375}','\u{cc97}'];
_2 = ['\u{a30d6}','\u{e4d97}','\u{3e235}','\u{142c}','\u{6b65c}','\u{6c44a}','\u{13a89}'];
RET = !_18.0.0;
_4 = _5 | _5;
Goto(bb10)
}
bb10 = {
Call(_24 = dump_var(12_usize, 13_usize, Move(_13), 8_usize, Move(_8), 12_usize, Move(_12), 18_usize, Move(_18)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_24 = dump_var(12_usize, 7_usize, Move(_7), 3_usize, Move(_3), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: [char; 7],mut _3: [isize; 2],mut _4: isize) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _5: u128;
let _6: usize;
let _7: f32;
let _8: bool;
let _9: i32;
let _10: [usize; 2];
let _11: f64;
let _12: i16;
let _13: &'static char;
let _14: *mut bool;
let _15: Adt39;
let _16: ();
let _17: ();
{
_1 = -_4;
_2 = ['\u{9c5e2}','\u{35d8b}','\u{91729}','\u{3d734}','\u{c363b}','\u{d75f4}','\u{dc56b}'];
_4 = _1;
RET = [_1,_1,_1,_4,_1];
_7 = 149258333540378039563094468168752356890_u128 as f32;
_6 = 1487021730557789067_usize;
_4 = 41886276597478468976253171847230354655_u128 as isize;
RET = [_1,_1,_1,_4,_1];
_5 = !38563822659394376102820694624670682917_u128;
RET = [_1,_1,_1,_1,_4];
_8 = true;
_3 = [_1,_1];
RET = [_1,_1,_1,_1,_1];
RET = [_4,_1,_1,_1,_4];
Call(_1 = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_1,_1,_1,_1,_1];
RET = [_1,_1,_1,_1,_1];
_3 = [_1,_1];
_4 = -_1;
RET = [_1,_1,_1,_1,_1];
_3 = [_1,_1];
RET = [_4,_4,_1,_4,_4];
_5 = 180821193469281706389013359310376789692_u128;
_10 = [_6,_6];
_9 = -(-1549069946_i32);
_7 = 31322_i16 as f32;
RET = [_4,_4,_4,_1,_4];
RET = [_4,_4,_4,_4,_4];
_2 = ['\u{fb7c5}','\u{e50d3}','\u{b74b8}','\u{f3388}','\u{102dd5}','\u{ab928}','\u{10b128}'];
_6 = 2_usize - 0_usize;
_9 = (-1134642655_i32) * (-1571763227_i32);
_4 = _5 as isize;
_2 = ['\u{5882}','\u{f8431}','\u{5dc7a}','\u{ee727}','\u{45c16}','\u{5b32a}','\u{a8b76}'];
_10 = [_6,_6];
Call(_4 = fn14(RET, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = [_4,_4];
RET = [_4,_4,_4,_1,_4];
_11 = (-151340814363919673322347667510344359986_i128) as f64;
_9 = 989540650_u32 as i32;
_2 = ['\u{481b7}','\u{f1d0e}','\u{804e8}','\u{58141}','\u{d3de7}','\u{a5e42}','\u{10277}'];
_11 = 22695_u16 as f64;
_2 = ['\u{a5908}','\u{839fb}','\u{109174}','\u{3ec16}','\u{10aef9}','\u{9c6c6}','\u{1074f}'];
_3 = [_1,_4];
_8 = false;
_1 = (-1_i8) as isize;
RET = [_1,_4,_4,_4,_4];
_2 = ['\u{cb18}','\u{29bed}','\u{1a166}','\u{d6326}','\u{106b16}','\u{41f3d}','\u{e394c}'];
RET = [_4,_4,_1,_4,_4];
_12 = 1021772936_u32 as i16;
_6 = 3268489528823356425_usize * 9593334296064269775_usize;
_10 = [_6,_6];
_14 = core::ptr::addr_of_mut!(_8);
RET = [_4,_4,_4,_4,_4];
RET = [_4,_4,_4,_4,_4];
_9 = (-726301602_i32) + (-709750937_i32);
(*_14) = _4 > _4;
Goto(bb3)
}
bb3 = {
Call(_16 = dump_var(13_usize, 6_usize, Move(_6), 3_usize, Move(_3), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_16 = dump_var(13_usize, 4_usize, Move(_4), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [isize; 5],mut _2: isize,mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: f32;
let _6: [i32; 1];
let _7: f64;
let _8: [i8; 1];
let _9: &'static i16;
let _10: f64;
let _11: u8;
let _12: char;
let _13: ((i16, usize),);
let _14: isize;
let _15: *mut i8;
let _16: isize;
let _17: Adt39;
let _18: isize;
let _19: isize;
let _20: ();
let _21: ();
{
_3 = (-3867_i16) as isize;
RET = _4 | _4;
_3 = !RET;
RET = 255_u8 as isize;
RET = '\u{638ac}' as isize;
_3 = !RET;
_2 = _4;
RET = _2;
_4 = (-19049_i16) as isize;
RET = (-221153264_i32) as isize;
_5 = 1713545485_i32 as f32;
_2 = RET - RET;
_6 = [(-66211436_i32)];
_2 = _4;
_4 = !RET;
_3 = _2;
_2 = 6_usize as isize;
_3 = _2 | RET;
_5 = 17092_u16 as f32;
_2 = _3 | RET;
RET = _4;
_4 = !_2;
_3 = !RET;
RET = true as isize;
Goto(bb1)
}
bb1 = {
RET = 1603884111603534946_i64 as isize;
_7 = 75_i8 as f64;
_1 = [_4,_3,RET,_2,_2];
_5 = 489_i16 as f32;
_7 = _5 as f64;
_1 = [_2,_4,_2,_2,_2];
_7 = _5 as f64;
_3 = 70_i8 as isize;
_7 = _4 as f64;
_4 = _2 * _2;
_1 = [_2,_4,_4,_3,_4];
_5 = (-1362076264729057719_i64) as f32;
_6 = [(-1235240962_i32)];
_7 = (-694632049_i32) as f64;
_8 = [(-68_i8)];
_6 = [1013889847_i32];
_5 = 6623_u16 as f32;
_4 = _2 ^ _3;
_1 = [_2,_4,_2,_4,_2];
_8 = [113_i8];
Goto(bb2)
}
bb2 = {
RET = !_4;
_2 = RET;
_7 = 3023644475501758540_usize as f64;
RET = _2;
_5 = RET as f32;
_4 = -_2;
_7 = 35976913058914827969551248698974651175_i128 as f64;
_6 = [1050595347_i32];
_10 = _7 + _7;
_10 = _5 as f64;
_3 = 17258_i16 as isize;
_8 = [109_i8];
_11 = !46_u8;
_6 = [968728572_i32];
_4 = 30299683752905440302025552370277086740_i128 as isize;
_12 = '\u{7a7b1}';
_5 = _11 as f32;
_4 = !RET;
_1 = [_4,_3,RET,_2,_4];
_2 = _4 & _4;
_1 = [RET,_2,_3,_2,_2];
_3 = _2;
_3 = !RET;
_13.0.0 = -(-21134_i16);
Goto(bb3)
}
bb3 = {
_13.0.1 = true as usize;
_13.0.0 = 29156_i16 * (-325_i16);
_10 = -_7;
_8 = [16_i8];
_5 = (-81472964823577082003540828277149314830_i128) as f32;
_3 = _4;
_10 = _7 - _7;
_13.0 = ((-10265_i16), 15369232666293135340_usize);
_1 = [_2,_2,_2,RET,_2];
_13.0.1 = 6_usize | 10080135377621718749_usize;
_8 = [17_i8];
_8 = [(-12_i8)];
_4 = !_3;
_7 = (-1305903648_i32) as f64;
_8 = [110_i8];
_7 = 8279491284173723076_u64 as f64;
_3 = -_2;
_1 = [_2,_3,_3,RET,_4];
_9 = &_13.0.0;
_7 = _10 - _10;
match _13.0.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431768201191 => bb11,
_ => bb10
}
}
bb4 = {
RET = !_4;
_2 = RET;
_7 = 3023644475501758540_usize as f64;
RET = _2;
_5 = RET as f32;
_4 = -_2;
_7 = 35976913058914827969551248698974651175_i128 as f64;
_6 = [1050595347_i32];
_10 = _7 + _7;
_10 = _5 as f64;
_3 = 17258_i16 as isize;
_8 = [109_i8];
_11 = !46_u8;
_6 = [968728572_i32];
_4 = 30299683752905440302025552370277086740_i128 as isize;
_12 = '\u{7a7b1}';
_5 = _11 as f32;
_4 = !RET;
_1 = [_4,_3,RET,_2,_4];
_2 = _4 & _4;
_1 = [RET,_2,_3,_2,_2];
_3 = _2;
_3 = !RET;
_13.0.0 = -(-21134_i16);
Goto(bb3)
}
bb5 = {
RET = 1603884111603534946_i64 as isize;
_7 = 75_i8 as f64;
_1 = [_4,_3,RET,_2,_2];
_5 = 489_i16 as f32;
_7 = _5 as f64;
_1 = [_2,_4,_2,_2,_2];
_7 = _5 as f64;
_3 = 70_i8 as isize;
_7 = _4 as f64;
_4 = _2 * _2;
_1 = [_2,_4,_4,_3,_4];
_5 = (-1362076264729057719_i64) as f32;
_6 = [(-1235240962_i32)];
_7 = (-694632049_i32) as f64;
_8 = [(-68_i8)];
_6 = [1013889847_i32];
_5 = 6623_u16 as f32;
_4 = _2 ^ _3;
_1 = [_2,_4,_2,_4,_2];
_8 = [113_i8];
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
Return()
}
bb11 = {
_14 = _2;
_11 = _3 as u8;
RET = 83192188740884538107116421672081869786_u128 as isize;
_13.0.1 = (-797979772386117590_i64) as usize;
_3 = _2;
_6 = [954068199_i32];
_13.0.1 = _12 as usize;
_17.fld0.1 = 26506_u16 as usize;
_4 = _2 - _14;
_13.0.1 = !_17.fld0.1;
_17 = Adt39 { fld0: _13.0 };
_14 = -_4;
_17.fld0.1 = _13.0.1;
_1 = [_14,_4,_14,_2,_4];
_2 = _17.fld0.1 as isize;
_17.fld0 = ((*_9), _13.0.1);
_6 = [(-1777802962_i32)];
_16 = _4;
_13 = (_17.fld0,);
Call(_17.fld0.1 = fn15(_13.0.0, _1, _1, _4, _16, _4, _3, _8, _1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_8 = [1_i8];
_7 = _4 as f64;
_17.fld0 = (_13.0.0, _13.0.1);
_9 = &_17.fld0.0;
_5 = _17.fld0.1 as f32;
_8 = [(-38_i8)];
_17.fld0.0 = _13.0.0;
match _13.0.0 {
340282366920938463463374607431768201191 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_5 = 153144627876636202484668864209818850171_i128 as f32;
_17 = Adt39 { fld0: _13.0 };
_16 = -_4;
_17 = Adt39 { fld0: _13.0 };
_13.0 = _17.fld0;
RET = -_4;
_11 = !109_u8;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(14_usize, 14_usize, Move(_14), 2_usize, Move(_2), 12_usize, Move(_12), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(14_usize, 13_usize, Move(_13), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i16,mut _2: [isize; 5],mut _3: [isize; 5],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [i8; 1],mut _9: [isize; 5]) -> usize {
mir! {
type RET = usize;
let _10: &'static [char; 7];
let _11: [i32; 2];
let _12: isize;
let _13: Adt47;
let _14: u32;
let _15: *mut i64;
let _16: *const usize;
let _17: i64;
let _18: char;
let _19: *const f64;
let _20: [char; 4];
let _21: (bool, &'static *const [char; 7], Adt46, (&'static i16,));
let _22: char;
let _23: ();
let _24: ();
{
RET = 5_usize & 2571031157911271871_usize;
_5 = _4;
_5 = !_6;
_6 = _5;
_6 = _4;
RET = !6_usize;
_2 = _9;
_3 = _9;
_8 = [(-112_i8)];
_7 = !_5;
RET = !755388784466259956_usize;
_8 = [(-84_i8)];
_9 = _3;
RET = 5_usize;
_9 = [_5,_4,_4,_7,_4];
_6 = _7 ^ _5;
_7 = (-247151963_i32) as isize;
RET = 5_usize + 7_usize;
_8 = [(-18_i8)];
_2 = [_6,_6,_6,_5,_4];
_8 = [(-90_i8)];
RET = !1_usize;
Goto(bb1)
}
bb1 = {
_4 = -_6;
_9 = [_6,_4,_6,_6,_4];
_7 = 54_i8 as isize;
_6 = !_4;
_8 = [(-97_i8)];
_1 = (-3663_i16) << _4;
_4 = !_6;
_4 = -_6;
_8 = [39_i8];
RET = 2195714893929648356_usize;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2195714893929648356 => bb10,
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
_5 = _4 & _6;
RET = !3_usize;
RET = _1 as usize;
_12 = 2967203336_u32 as isize;
_12 = _6;
RET = 29869_u16 as usize;
_6 = _5 - _5;
_1 = (-211256684_i32) as i16;
_7 = (-1813855646_i32) as isize;
_12 = _6 ^ _5;
_9 = _2;
_9 = _2;
_4 = -_12;
_3 = _2;
Goto(bb11)
}
bb11 = {
RET = !3757894233804088662_usize;
_6 = (-331861411_i32) as isize;
RET = !2013466085578211247_usize;
_4 = _5;
_5 = _4;
_11 = [584919864_i32,(-263396829_i32)];
RET = (-1855401081_i32) as usize;
_2 = [_4,_12,_12,_4,_5];
_5 = -_12;
RET = 35_u8 as usize;
_14 = 3189295768_u32;
_11 = [(-1532639834_i32),627821116_i32];
_5 = 84_u8 as isize;
_4 = '\u{b1a84}' as isize;
_12 = _6;
_7 = _12;
_8 = [(-27_i8)];
_3 = [_5,_4,_5,_12,_4];
RET = !2_usize;
_3 = _2;
_2 = [_4,_4,_4,_4,_5];
_16 = core::ptr::addr_of!(RET);
_2 = [_6,_7,_12,_12,_7];
_4 = 1203459961305426547_u64 as isize;
match _14 {
0 => bb4,
1 => bb6,
2 => bb5,
3 => bb12,
4 => bb13,
3189295768 => bb15,
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
_4 = -_6;
_9 = [_6,_4,_6,_6,_4];
_7 = 54_i8 as isize;
_6 = !_4;
_8 = [(-97_i8)];
_1 = (-3663_i16) << _4;
_4 = !_6;
_4 = -_6;
_8 = [39_i8];
RET = 2195714893929648356_usize;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2195714893929648356 => bb10,
_ => bb9
}
}
bb15 = {
_2 = [_7,_7,_6,_7,_5];
_1 = '\u{10b21d}' as i16;
_1 = 9553_u16 as i16;
RET = 0_usize;
_3 = _9;
_5 = _9[RET] + _9[RET];
_3[RET] = !_5;
_5 = RET as isize;
_5 = -_9[RET];
_8[RET] = 155230474579117015102711412905960485792_u128 as i8;
_6 = true as isize;
_4 = true as isize;
_17 = (-3581827888142810870_i64);
_11[RET] = -(-1498024603_i32);
_20[RET] = '\u{10cc39}';
_15 = core::ptr::addr_of_mut!(_17);
_1 = (-26034_i16) + 5314_i16;
_2[RET] = _5 * _5;
_1 = (-7503_i16) << _2[RET];
_21.3.0 = &_1;
_15 = core::ptr::addr_of_mut!(_17);
_21.3.0 = &_1;
_1 = !(-1553_i16);
RET = 2_usize - 393329677988606504_usize;
_16 = core::ptr::addr_of!(RET);
_11 = [759741796_i32,(-656348832_i32)];
_17 = 8688430345467630186_i64;
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(15_usize, 3_usize, Move(_3), 17_usize, Move(_17), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(15_usize, 1_usize, Move(_1), 8_usize, Move(_8), 24_usize, _24, 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: [char; 7]) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _4: *mut Adt24;
let _5: &'static *mut i128;
let _6: &'static f64;
let _7: f64;
let _8: &'static *const [char; 7];
let _9: Adt58;
let _10: &'static i8;
let _11: [u32; 4];
let _12: *const f64;
let _13: *mut ([char; 7], *mut u8);
let _14: (i16, usize);
let _15: *mut i8;
let _16: Adt39;
let _17: [i32; 8];
let _18: [char; 7];
let _19: f32;
let _20: ();
let _21: ();
{
RET = [_1,_2,_2,_1,_1];
_1 = 11006343923980706283_u64 as isize;
Goto(bb1)
}
bb1 = {
RET = [_2,_2,_2,_1,_1];
_1 = !_2;
RET = [_2,_2,_2,_1,_1];
RET = [_1,_1,_2,_1,_2];
RET = [_2,_2,_2,_1,_2];
_2 = _1 << _1;
RET = [_2,_2,_1,_2,_2];
Goto(bb2)
}
bb2 = {
_2 = _1;
RET = [_2,_1,_2,_1,_2];
_1 = _2 * _2;
_2 = -_1;
RET = [_1,_1,_1,_1,_1];
_2 = 17044627895364121581_usize as isize;
_7 = (-21_i8) as f64;
RET = [_1,_2,_1,_1,_1];
_2 = -_1;
_2 = !_1;
RET = [_2,_1,_2,_1,_2];
_2 = -_1;
_6 = &_7;
Goto(bb3)
}
bb3 = {
_2 = _1;
RET = [_2,_2,_2,_1,_2];
RET = [_2,_1,_1,_2,_1];
_2 = 206265657082521929524266864946567269984_u128 as isize;
_7 = _2 as f64;
_3 = ['\u{afa27}','\u{aebb7}','\u{ade74}','\u{2a72d}','\u{92a8e}','\u{db623}','\u{f2dbf}'];
RET = [_1,_1,_1,_1,_1];
RET = [_1,_1,_1,_1,_1];
_1 = _2 * _2;
_1 = 34293026398094756797791250654975527375_i128 as isize;
_3 = ['\u{eb8d6}','\u{e3544}','\u{f74f1}','\u{4235e}','\u{519d1}','\u{8371c}','\u{8520e}'];
Goto(bb4)
}
bb4 = {
_3 = ['\u{bfd0b}','\u{ec35f}','\u{5f660}','\u{b0c44}','\u{f93c}','\u{64ce1}','\u{1f354}'];
_2 = _1 * _1;
RET = [_1,_2,_1,_2,_2];
_2 = -_1;
_7 = 11826688591743490559_u64 as f64;
_1 = (-108304985_i32) as isize;
_7 = (-636539780_i32) as f64;
RET = [_2,_1,_2,_2,_1];
_2 = _1 | _1;
_1 = 1229380171_i32 as isize;
Goto(bb5)
}
bb5 = {
RET = [_2,_2,_1,_2,_2];
_6 = &_7;
_2 = 2207356444_u32 as isize;
_1 = _2;
_9.fld0 = Adt24 { fld0: (-852898436_i32) };
_9.fld0 = Adt24 { fld0: (-507831078_i32) };
_7 = 28341_i16 as f64;
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
_7 = 252_u16 as f64;
_2 = _1 | _1;
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
_9.fld3 = [1888113008141405608_u64,9262764596980524128_u64,12156331860725252761_u64,14812557813958465298_u64,12168382987977922284_u64,4023989396749650909_u64];
_6 = &_7;
RET = [_2,_2,_2,_2,_2];
RET = [_1,_2,_2,_2,_2];
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
_11 = [169675550_u32,2339889866_u32,98837758_u32,2797946551_u32];
_4 = core::ptr::addr_of_mut!(_9.fld0);
_2 = '\u{ea0f8}' as isize;
_3 = ['\u{9d3e8}','\u{e6a92}','\u{e5d0f}','\u{702c6}','\u{8d269}','\u{444a7}','\u{1f70e}'];
_1 = 126_u8 as isize;
Call(_9.fld0.fld0 = fn17(Move(_6), _2, _9.fld3, _3, RET, _9.fld2, Move(_4), _11, _9.fld3, (*_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = (-124913674140442814120190485988067039565_i128) as f64;
_2 = _1;
_6 = &_7;
_9.fld0 = Adt24 { fld0: 102833762_i32 };
_14 = ((-7752_i16), 11358408145863730939_usize);
_14.1 = !9549156670841440812_usize;
_11 = [187811347_u32,2420687012_u32,1346695202_u32,797788546_u32];
_12 = core::ptr::addr_of!((*_6));
RET = [_2,_2,_2,_1,_1];
RET = [_1,_1,_2,_2,_1];
_14 = ((-13065_i16), 3_usize);
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
match _14.1 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb10,
_ => bb9
}
}
bb7 = {
RET = [_2,_2,_1,_2,_2];
_6 = &_7;
_2 = 2207356444_u32 as isize;
_1 = _2;
_9.fld0 = Adt24 { fld0: (-852898436_i32) };
_9.fld0 = Adt24 { fld0: (-507831078_i32) };
_7 = 28341_i16 as f64;
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
_7 = 252_u16 as f64;
_2 = _1 | _1;
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
_9.fld3 = [1888113008141405608_u64,9262764596980524128_u64,12156331860725252761_u64,14812557813958465298_u64,12168382987977922284_u64,4023989396749650909_u64];
_6 = &_7;
RET = [_2,_2,_2,_2,_2];
RET = [_1,_2,_2,_2,_2];
_9.fld2 = [_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0,_9.fld0.fld0];
_11 = [169675550_u32,2339889866_u32,98837758_u32,2797946551_u32];
_4 = core::ptr::addr_of_mut!(_9.fld0);
_2 = '\u{ea0f8}' as isize;
_3 = ['\u{9d3e8}','\u{e6a92}','\u{e5d0f}','\u{702c6}','\u{8d269}','\u{444a7}','\u{1f70e}'];
_1 = 126_u8 as isize;
Call(_9.fld0.fld0 = fn17(Move(_6), _2, _9.fld3, _3, RET, _9.fld2, Move(_4), _11, _9.fld3, (*_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
RET = [_2,_2,_2,_1,_1];
_1 = !_2;
RET = [_2,_2,_2,_1,_1];
RET = [_1,_1,_2,_1,_2];
RET = [_2,_2,_2,_1,_2];
_2 = _1 << _1;
RET = [_2,_2,_1,_2,_2];
Goto(bb2)
}
bb9 = {
_2 = _1;
RET = [_2,_2,_2,_1,_2];
RET = [_2,_1,_1,_2,_1];
_2 = 206265657082521929524266864946567269984_u128 as isize;
_7 = _2 as f64;
_3 = ['\u{afa27}','\u{aebb7}','\u{ade74}','\u{2a72d}','\u{92a8e}','\u{db623}','\u{f2dbf}'];
RET = [_1,_1,_1,_1,_1];
RET = [_1,_1,_1,_1,_1];
_1 = _2 * _2;
_1 = 34293026398094756797791250654975527375_i128 as isize;
_3 = ['\u{eb8d6}','\u{e3544}','\u{f74f1}','\u{4235e}','\u{519d1}','\u{8371c}','\u{8520e}'];
Goto(bb4)
}
bb10 = {
_11 = [933161722_u32,2462674458_u32,2998884497_u32,1150809322_u32];
_9.fld0 = Adt24 { fld0: 1591010064_i32 };
_3 = ['\u{e27af}','\u{10a644}','\u{391c7}','\u{e76fa}','\u{2c61f}','\u{ff6f}','\u{7dcb1}'];
_16.fld0.1 = _14.0 as usize;
_4 = core::ptr::addr_of_mut!(_9.fld0);
(*_4).fld0 = 204_u8 as i32;
_1 = -_2;
_16 = Adt39 { fld0: _14 };
(*_4).fld0 = 109267295_i32;
_16.fld0.0 = !_14.0;
(*_4).fld0 = 223366362278025606_i64 as i32;
match _16.fld0.1 {
0 => bb11,
3 => bb13,
_ => bb12
}
}
bb11 = {
_2 = _1;
RET = [_2,_2,_2,_1,_2];
RET = [_2,_1,_1,_2,_1];
_2 = 206265657082521929524266864946567269984_u128 as isize;
_7 = _2 as f64;
_3 = ['\u{afa27}','\u{aebb7}','\u{ade74}','\u{2a72d}','\u{92a8e}','\u{db623}','\u{f2dbf}'];
RET = [_1,_1,_1,_1,_1];
RET = [_1,_1,_1,_1,_1];
_1 = _2 * _2;
_1 = 34293026398094756797791250654975527375_i128 as isize;
_3 = ['\u{eb8d6}','\u{e3544}','\u{f74f1}','\u{4235e}','\u{519d1}','\u{8371c}','\u{8520e}'];
Goto(bb4)
}
bb12 = {
RET = [_2,_2,_2,_1,_1];
_1 = !_2;
RET = [_2,_2,_2,_1,_1];
RET = [_1,_1,_2,_1,_2];
RET = [_2,_2,_2,_1,_2];
_2 = _1 << _1;
RET = [_2,_2,_1,_2,_2];
Goto(bb2)
}
bb13 = {
_16.fld0.1 = !_14.1;
Goto(bb14)
}
bb14 = {
(*_4) = Adt24 { fld0: 462374368_i32 };
_17 = _9.fld2;
(*_4).fld0 = _2 as i32;
_14.0 = _16.fld0.0;
_17 = [(*_4).fld0,(*_4).fld0,_9.fld0.fld0,(*_4).fld0,(*_4).fld0,_9.fld0.fld0,_9.fld0.fld0,(*_4).fld0];
RET = [_1,_1,_1,_2,_1];
_9.fld2 = [_9.fld0.fld0,(*_4).fld0,(*_4).fld0,(*_4).fld0,(*_4).fld0,(*_4).fld0,(*_4).fld0,(*_4).fld0];
_2 = '\u{ef8b3}' as isize;
_16.fld0 = (_14.0, _14.1);
_16.fld0.0 = _14.0 >> _14.1;
_14.0 = _16.fld0.0 | _16.fld0.0;
_2 = _16.fld0.0 as isize;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(16_usize, 11_usize, Move(_11), 3_usize, Move(_3), 17_usize, Move(_17), 21_usize, _21), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: &'static f64,mut _2: isize,mut _3: [u64; 6],mut _4: [char; 7],mut _5: [isize; 5],mut _6: [i32; 8],mut _7: *mut Adt24,mut _8: [u32; 4],mut _9: [u64; 6],mut _10: f64) -> i32 {
mir! {
type RET = i32;
let _11: [char; 4];
let _12: f64;
let _13: isize;
let _14: u32;
let _15: &'static &'static bool;
let _16: u32;
let _17: *const usize;
let _18: bool;
let _19: [isize; 5];
let _20: isize;
let _21: [usize; 2];
let _22: char;
let _23: (*mut i128, (&'static i16,), char);
let _24: [u32; 4];
let _25: &'static &'static (*mut i128, (&'static i16,), char);
let _26: i8;
let _27: *mut bool;
let _28: &'static *mut *mut u8;
let _29: f64;
let _30: ((i16, usize),);
let _31: Adt60;
let _32: f64;
let _33: u16;
let _34: [u64; 6];
let _35: [u64; 6];
let _36: isize;
let _37: (i16, usize);
let _38: f32;
let _39: f32;
let _40: i32;
let _41: *mut isize;
let _42: Adt47;
let _43: [i32; 7];
let _44: ();
let _45: ();
{
_1 = &_10;
RET = _2 as i32;
RET = -747076979_i32;
Goto(bb1)
}
bb1 = {
_2 = 90_u8 as isize;
_11 = ['\u{d8282}','\u{bbee0}','\u{7bb77}','\u{c936a}'];
RET = 66_u8 as i32;
_9 = _3;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [_2,_2,_2,_2,_2];
RET = 1653335502_i32;
_2 = RET as isize;
_1 = &(*_1);
_1 = &(*_1);
RET = -(-474672394_i32);
_8 = [598797772_u32,3134272965_u32,417572762_u32,2975170613_u32];
_10 = (-12741928931656027151138637023625021718_i128) as f64;
_1 = &_10;
_3 = [11090084929015279007_u64,11036151922295471158_u64,15752145499852725670_u64,11531269577239377459_u64,13163984910437254658_u64,1047335440871551502_u64];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{48e30}','\u{d8f25}','\u{92a0}','\u{10c93c}'];
RET = 1787922575_i32;
_4 = ['\u{c8469}','\u{104d78}','\u{2cdf1}','\u{59aca}','\u{33fbc}','\u{8f019}','\u{b739e}'];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = [5570864072082451858_u64,11452658866511589057_u64,17336436453805645490_u64,5404995506014659820_u64,11257284710651862511_u64,4176437702264710316_u64];
_3 = [8883420490087589333_u64,15212235510697431303_u64,13059962498460193333_u64,11418730760406003685_u64,13336690953345875896_u64,8153656416841877503_u64];
_2 = !(-9223372036854775808_isize);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{588b4}','\u{d00}','\u{aa10c}','\u{82be5}'];
_12 = (*_1);
_12 = -_10;
_1 = &(*_1);
Goto(bb2)
}
bb2 = {
_10 = _12;
_3 = [10512992492263219052_u64,16781263618439131628_u64,12263114884004834879_u64,16404514171408817733_u64,6538062605074831976_u64,3251753858406080511_u64];
_11 = ['\u{e37e2}','\u{bf639}','\u{38db9}','\u{77ec6}'];
_9 = _3;
_1 = &_12;
_10 = (*_1) - _12;
RET = (-1629594666_i32) | 169484260_i32;
_11 = ['\u{da0d0}','\u{5433c}','\u{c54f2}','\u{bbeb7}'];
RET = 1471278425_i32 - (-799131238_i32);
_5 = [_2,_2,_2,_2,_2];
_11 = ['\u{1085a5}','\u{5730}','\u{86c19}','\u{45d3d}'];
_11 = ['\u{a5786}','\u{e7751}','\u{4f1f7}','\u{de6d2}'];
_9 = [16340336141788116855_u64,3142197592312743413_u64,2306608865479889811_u64,12342184569251050422_u64,12485189132430404477_u64,14274200531653632328_u64];
RET = 780101436_i32;
_5 = [_2,_2,_2,_2,_2];
_2 = -(-9223372036854775808_isize);
RET = 124410447_u32 as i32;
Goto(bb3)
}
bb3 = {
_2 = (-9223372036854775808_isize);
_14 = (-128_i8) as u32;
_9 = [14303941164871283292_u64,1335961845741474593_u64,12307897042313518304_u64,13120698719044824875_u64,15938606172177529179_u64,283243699386131908_u64];
_2 = 111_isize;
_13 = RET as isize;
_14 = 1096713441_u32;
_8 = [_14,_14,_14,_14];
_9 = _3;
_18 = true;
_5 = [_13,_2,_13,_13,_13];
_16 = !_14;
RET = 89136570_i32 * (-850825438_i32);
RET = (-1823876964_i32) * 1097432801_i32;
_9 = _3;
_5 = [_2,_13,_13,_2,_13];
_16 = _14 - _14;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [11600260177723480529_u64,4313982785320897006_u64,6949828981030530827_u64,6358227016570997275_u64,16740689266657527892_u64,14630569393768998869_u64];
_14 = _16 << RET;
_11 = ['\u{bf8b3}','\u{69975}','\u{39fe4}','\u{f0541}'];
_20 = !_2;
_20 = !_2;
_19 = [_13,_13,_20,_20,_13];
_5 = [_13,_2,_2,_20,_13];
match _2 {
0 => bb2,
1 => bb4,
2 => bb5,
111 => bb7,
_ => bb6
}
}
bb4 = {
_10 = _12;
_3 = [10512992492263219052_u64,16781263618439131628_u64,12263114884004834879_u64,16404514171408817733_u64,6538062605074831976_u64,3251753858406080511_u64];
_11 = ['\u{e37e2}','\u{bf639}','\u{38db9}','\u{77ec6}'];
_9 = _3;
_1 = &_12;
_10 = (*_1) - _12;
RET = (-1629594666_i32) | 169484260_i32;
_11 = ['\u{da0d0}','\u{5433c}','\u{c54f2}','\u{bbeb7}'];
RET = 1471278425_i32 - (-799131238_i32);
_5 = [_2,_2,_2,_2,_2];
_11 = ['\u{1085a5}','\u{5730}','\u{86c19}','\u{45d3d}'];
_11 = ['\u{a5786}','\u{e7751}','\u{4f1f7}','\u{de6d2}'];
_9 = [16340336141788116855_u64,3142197592312743413_u64,2306608865479889811_u64,12342184569251050422_u64,12485189132430404477_u64,14274200531653632328_u64];
RET = 780101436_i32;
_5 = [_2,_2,_2,_2,_2];
_2 = -(-9223372036854775808_isize);
RET = 124410447_u32 as i32;
Goto(bb3)
}
bb5 = {
_2 = 90_u8 as isize;
_11 = ['\u{d8282}','\u{bbee0}','\u{7bb77}','\u{c936a}'];
RET = 66_u8 as i32;
_9 = _3;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [_2,_2,_2,_2,_2];
RET = 1653335502_i32;
_2 = RET as isize;
_1 = &(*_1);
_1 = &(*_1);
RET = -(-474672394_i32);
_8 = [598797772_u32,3134272965_u32,417572762_u32,2975170613_u32];
_10 = (-12741928931656027151138637023625021718_i128) as f64;
_1 = &_10;
_3 = [11090084929015279007_u64,11036151922295471158_u64,15752145499852725670_u64,11531269577239377459_u64,13163984910437254658_u64,1047335440871551502_u64];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{48e30}','\u{d8f25}','\u{92a0}','\u{10c93c}'];
RET = 1787922575_i32;
_4 = ['\u{c8469}','\u{104d78}','\u{2cdf1}','\u{59aca}','\u{33fbc}','\u{8f019}','\u{b739e}'];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = [5570864072082451858_u64,11452658866511589057_u64,17336436453805645490_u64,5404995506014659820_u64,11257284710651862511_u64,4176437702264710316_u64];
_3 = [8883420490087589333_u64,15212235510697431303_u64,13059962498460193333_u64,11418730760406003685_u64,13336690953345875896_u64,8153656416841877503_u64];
_2 = !(-9223372036854775808_isize);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{588b4}','\u{d00}','\u{aa10c}','\u{82be5}'];
_12 = (*_1);
_12 = -_10;
_1 = &(*_1);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{5a7fb}','\u{6f667}','\u{994f}','\u{ae3de}'];
_20 = -_2;
_2 = _13;
_12 = _10;
_1 = &_12;
_4 = ['\u{79024}','\u{10cbd0}','\u{2d5f8}','\u{12357}','\u{70d01}','\u{a6941}','\u{3bed9}'];
_23.2 = '\u{5e2f9}';
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_20 = _13 + _2;
Call(RET = core::intrinsics::transmute(_23.2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18 = false;
RET = -390018950_i32;
_10 = _12 + (*_1);
RET = 1142216306_i32;
match RET {
0 => bb9,
1142216306 => bb11,
_ => bb10
}
}
bb9 = {
_10 = _12;
_3 = [10512992492263219052_u64,16781263618439131628_u64,12263114884004834879_u64,16404514171408817733_u64,6538062605074831976_u64,3251753858406080511_u64];
_11 = ['\u{e37e2}','\u{bf639}','\u{38db9}','\u{77ec6}'];
_9 = _3;
_1 = &_12;
_10 = (*_1) - _12;
RET = (-1629594666_i32) | 169484260_i32;
_11 = ['\u{da0d0}','\u{5433c}','\u{c54f2}','\u{bbeb7}'];
RET = 1471278425_i32 - (-799131238_i32);
_5 = [_2,_2,_2,_2,_2];
_11 = ['\u{1085a5}','\u{5730}','\u{86c19}','\u{45d3d}'];
_11 = ['\u{a5786}','\u{e7751}','\u{4f1f7}','\u{de6d2}'];
_9 = [16340336141788116855_u64,3142197592312743413_u64,2306608865479889811_u64,12342184569251050422_u64,12485189132430404477_u64,14274200531653632328_u64];
RET = 780101436_i32;
_5 = [_2,_2,_2,_2,_2];
_2 = -(-9223372036854775808_isize);
RET = 124410447_u32 as i32;
Goto(bb3)
}
bb10 = {
Return()
}
bb11 = {
_21 = [6_usize,4_usize];
_22 = _23.2;
_27 = core::ptr::addr_of_mut!(_18);
_21 = [7968081905339351332_usize,3911642568585374341_usize];
_2 = _20 * _13;
_22 = _23.2;
_24 = [_16,_14,_16,_14];
_16 = _14 - _14;
(*_27) = !true;
RET = (-445324494_i32) & (-210185634_i32);
_14 = _16;
_9 = [9446638835361083381_u64,17409438978693591677_u64,6415056935629627255_u64,6250229857701017704_u64,17820789123659650426_u64,2755369317955098305_u64];
_29 = _10;
_13 = 48491_u16 as isize;
_27 = core::ptr::addr_of_mut!((*_27));
_30.0.0 = (-32116_i16);
_4 = [_22,_22,_22,_23.2,_22,_23.2,_23.2];
_17 = core::ptr::addr_of!(_30.0.1);
_23.1.0 = &_30.0.0;
_27 = core::ptr::addr_of_mut!((*_27));
_16 = _14 + _14;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_33 = _20 as u16;
_5 = [_20,_2,_20,_2,_2];
_30.0.1 = 12318308771975018952_usize;
Call(RET = fn18(Move(_23.1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_27) = !false;
_12 = _30.0.1 as f64;
_16 = 4787598586381202702_u64 as u32;
_37 = _30.0;
_38 = 154345735061836888654913489555436159686_i128 as f32;
_11 = [_23.2,_23.2,_22,_22];
(*_27) = !true;
_34 = [116866914834821877_u64,14827003165596327609_u64,10465736108785457513_u64,13306409917213804846_u64,3624025762980857956_u64,14037121671588044699_u64];
(*_17) = _37.1 & _37.1;
match _37.0 {
0 => bb6,
1 => bb3,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463463374607431768179340 => bb19,
_ => bb18
}
}
bb13 = {
_21 = [6_usize,4_usize];
_22 = _23.2;
_27 = core::ptr::addr_of_mut!(_18);
_21 = [7968081905339351332_usize,3911642568585374341_usize];
_2 = _20 * _13;
_22 = _23.2;
_24 = [_16,_14,_16,_14];
_16 = _14 - _14;
(*_27) = !true;
RET = (-445324494_i32) & (-210185634_i32);
_14 = _16;
_9 = [9446638835361083381_u64,17409438978693591677_u64,6415056935629627255_u64,6250229857701017704_u64,17820789123659650426_u64,2755369317955098305_u64];
_29 = _10;
_13 = 48491_u16 as isize;
_27 = core::ptr::addr_of_mut!((*_27));
_30.0.0 = (-32116_i16);
_4 = [_22,_22,_22,_23.2,_22,_23.2,_23.2];
_17 = core::ptr::addr_of!(_30.0.1);
_23.1.0 = &_30.0.0;
_27 = core::ptr::addr_of_mut!((*_27));
_16 = _14 + _14;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_33 = _20 as u16;
_5 = [_20,_2,_20,_2,_2];
_30.0.1 = 12318308771975018952_usize;
Call(RET = fn18(Move(_23.1)), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_2 = (-9223372036854775808_isize);
_14 = (-128_i8) as u32;
_9 = [14303941164871283292_u64,1335961845741474593_u64,12307897042313518304_u64,13120698719044824875_u64,15938606172177529179_u64,283243699386131908_u64];
_2 = 111_isize;
_13 = RET as isize;
_14 = 1096713441_u32;
_8 = [_14,_14,_14,_14];
_9 = _3;
_18 = true;
_5 = [_13,_2,_13,_13,_13];
_16 = !_14;
RET = 89136570_i32 * (-850825438_i32);
RET = (-1823876964_i32) * 1097432801_i32;
_9 = _3;
_5 = [_2,_13,_13,_2,_13];
_16 = _14 - _14;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [11600260177723480529_u64,4313982785320897006_u64,6949828981030530827_u64,6358227016570997275_u64,16740689266657527892_u64,14630569393768998869_u64];
_14 = _16 << RET;
_11 = ['\u{bf8b3}','\u{69975}','\u{39fe4}','\u{f0541}'];
_20 = !_2;
_20 = !_2;
_19 = [_13,_13,_20,_20,_13];
_5 = [_13,_2,_2,_20,_13];
match _2 {
0 => bb2,
1 => bb4,
2 => bb5,
111 => bb7,
_ => bb6
}
}
bb16 = {
_2 = 90_u8 as isize;
_11 = ['\u{d8282}','\u{bbee0}','\u{7bb77}','\u{c936a}'];
RET = 66_u8 as i32;
_9 = _3;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [_2,_2,_2,_2,_2];
RET = 1653335502_i32;
_2 = RET as isize;
_1 = &(*_1);
_1 = &(*_1);
RET = -(-474672394_i32);
_8 = [598797772_u32,3134272965_u32,417572762_u32,2975170613_u32];
_10 = (-12741928931656027151138637023625021718_i128) as f64;
_1 = &_10;
_3 = [11090084929015279007_u64,11036151922295471158_u64,15752145499852725670_u64,11531269577239377459_u64,13163984910437254658_u64,1047335440871551502_u64];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{48e30}','\u{d8f25}','\u{92a0}','\u{10c93c}'];
RET = 1787922575_i32;
_4 = ['\u{c8469}','\u{104d78}','\u{2cdf1}','\u{59aca}','\u{33fbc}','\u{8f019}','\u{b739e}'];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = [5570864072082451858_u64,11452658866511589057_u64,17336436453805645490_u64,5404995506014659820_u64,11257284710651862511_u64,4176437702264710316_u64];
_3 = [8883420490087589333_u64,15212235510697431303_u64,13059962498460193333_u64,11418730760406003685_u64,13336690953345875896_u64,8153656416841877503_u64];
_2 = !(-9223372036854775808_isize);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{588b4}','\u{d00}','\u{aa10c}','\u{82be5}'];
_12 = (*_1);
_12 = -_10;
_1 = &(*_1);
Goto(bb2)
}
bb17 = {
_2 = 90_u8 as isize;
_11 = ['\u{d8282}','\u{bbee0}','\u{7bb77}','\u{c936a}'];
RET = 66_u8 as i32;
_9 = _3;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [_2,_2,_2,_2,_2];
RET = 1653335502_i32;
_2 = RET as isize;
_1 = &(*_1);
_1 = &(*_1);
RET = -(-474672394_i32);
_8 = [598797772_u32,3134272965_u32,417572762_u32,2975170613_u32];
_10 = (-12741928931656027151138637023625021718_i128) as f64;
_1 = &_10;
_3 = [11090084929015279007_u64,11036151922295471158_u64,15752145499852725670_u64,11531269577239377459_u64,13163984910437254658_u64,1047335440871551502_u64];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{48e30}','\u{d8f25}','\u{92a0}','\u{10c93c}'];
RET = 1787922575_i32;
_4 = ['\u{c8469}','\u{104d78}','\u{2cdf1}','\u{59aca}','\u{33fbc}','\u{8f019}','\u{b739e}'];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = [5570864072082451858_u64,11452658866511589057_u64,17336436453805645490_u64,5404995506014659820_u64,11257284710651862511_u64,4176437702264710316_u64];
_3 = [8883420490087589333_u64,15212235510697431303_u64,13059962498460193333_u64,11418730760406003685_u64,13336690953345875896_u64,8153656416841877503_u64];
_2 = !(-9223372036854775808_isize);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = ['\u{588b4}','\u{d00}','\u{aa10c}','\u{82be5}'];
_12 = (*_1);
_12 = -_10;
_1 = &(*_1);
Goto(bb2)
}
bb18 = {
Return()
}
bb19 = {
(*_27) = true;
_38 = (*_17) as f32;
(*_27) = true;
_2 = 150598000429344319822372840495224044993_u128 as isize;
_27 = core::ptr::addr_of_mut!((*_27));
_2 = (-43_i8) as isize;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [11430601011181435978_u64,5691392396310539376_u64,7328205797345735126_u64,10476760529658559187_u64,17163792665734565972_u64,617308185827230936_u64];
_26 = 39_i8;
_30 = (_37,);
_23.1.0 = &_30.0.0;
_30 = (_37,);
_3 = _9;
_18 = false;
_12 = _29;
_27 = core::ptr::addr_of_mut!(_18);
_23.1.0 = &_30.0.0;
_30.0 = (_37.0, _37.1);
(*_17) = _33 as usize;
_30.0.0 = _37.0;
_14 = 19492538015795330465341127054863287152_u128 as u32;
Goto(bb20)
}
bb20 = {
Call(_44 = dump_var(17_usize, 6_usize, Move(_6), 24_usize, Move(_24), 13_usize, Move(_13), 21_usize, Move(_21)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_44 = dump_var(17_usize, 5_usize, Move(_5), 19_usize, Move(_19), 16_usize, Move(_16), 37_usize, Move(_37)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_44 = dump_var(17_usize, 26_usize, Move(_26), 4_usize, Move(_4), 8_usize, Move(_8), 45_usize, _45), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: (&'static i16,)) -> i32 {
mir! {
type RET = i32;
let _2: *const usize;
let _3: f64;
let _4: char;
let _5: f32;
let _6: i16;
let _7: bool;
let _8: char;
let _9: &'static *const usize;
let _10: *const *mut u8;
let _11: u128;
let _12: [char; 7];
let _13: char;
let _14: *mut i64;
let _15: *mut ([char; 7], *mut u8);
let _16: f64;
let _17: f32;
let _18: char;
let _19: &'static *const usize;
let _20: [i32; 7];
let _21: &'static [char; 7];
let _22: i128;
let _23: isize;
let _24: u64;
let _25: (Adt47, [u128; 1], &'static f64);
let _26: u8;
let _27: i8;
let _28: ();
let _29: ();
{
RET = 33726_u16 as i32;
RET = !(-775147530_i32);
RET = -(-1003471265_i32);
RET = 1459190244_i32 | 2062480752_i32;
RET = (-1695341893_i32) - 790213436_i32;
RET = 2613539144_u32 as i32;
RET = (-2139641917_i32);
RET = 654616716_i32 | 1251668756_i32;
RET = (-1030474403_i32);
_3 = 291305274343608871327649794674372345247_u128 as f64;
RET = 569525486_i32 * (-1920406882_i32);
RET = 4_usize as i32;
_4 = '\u{1e0a9}';
RET = _3 as i32;
RET = (-443100082_i32) >> 9223372036854775807_isize;
_3 = 2153285512_u32 as f64;
RET = 162720686_i32 & (-1072566322_i32);
RET = (-471247173_i32) & (-1094750721_i32);
_3 = 9421_i16 as f64;
RET = 1390709637_i32;
RET = !557188764_i32;
_3 = 118461632148800379658360980964017392555_u128 as f64;
RET = 2458329118855724156_i64 as i32;
RET = (-2030332112_i32);
_4 = '\u{ffd70}';
_4 = '\u{736a4}';
RET = (-1018360636_i32) << 3886605597_u32;
RET = (-123_i8) as i32;
Goto(bb1)
}
bb1 = {
RET = _4 as i32;
RET = 141_u8 as i32;
_4 = '\u{98f4a}';
_3 = 3058100688766628705_u64 as f64;
RET = (-905226123_i32) | 1823355539_i32;
_1.0 = &_6;
_4 = '\u{2e18}';
Goto(bb2)
}
bb2 = {
_5 = 9223372036854775807_isize as f32;
_3 = 107_u8 as f64;
_1.0 = &_6;
_1.0 = &_6;
_6 = (-31205_i16) << RET;
_7 = !false;
_7 = !false;
RET = 1041558121_i32;
Goto(bb3)
}
bb3 = {
RET = (-121_isize) as i32;
RET = !1084888752_i32;
_8 = _4;
_1.0 = &_6;
Goto(bb4)
}
bb4 = {
_4 = _8;
_4 = _8;
_3 = 2_i8 as f64;
_9 = &_2;
_1.0 = &_6;
_3 = (-9223372036854775808_isize) as f64;
_9 = &(*_9);
_5 = 9462594194026936135_u64 as f32;
_8 = _4;
_12 = [_8,_4,_4,_4,_8,_8,_8];
_8 = _4;
_1.0 = &_6;
_7 = _5 != _5;
RET = (-812020055_i32);
_3 = 63_u8 as f64;
_8 = _4;
_3 = 98633062237540770867977713761163473839_u128 as f64;
_5 = 48_i8 as f32;
_9 = &(*_9);
_1.0 = &_6;
RET = 0_usize as i32;
Goto(bb5)
}
bb5 = {
_6 = 173286426776025505496888772361388364514_u128 as i16;
_11 = 97799448215545653634091663415796511522_u128 + 40788460305318740642004965739176525943_u128;
_13 = _8;
_5 = 145877849045522776790163693143483025144_i128 as f32;
RET = !1145202828_i32;
RET = (-331533208_i32);
_13 = _4;
_13 = _8;
_1.0 = &_6;
RET = 93_i8 as i32;
_9 = &_2;
_13 = _4;
_4 = _8;
_16 = -_3;
RET = !(-615126130_i32);
Goto(bb6)
}
bb6 = {
_9 = &(*_9);
_17 = _5;
_12 = [_8,_8,_8,_8,_13,_13,_4];
RET = 1708000646_i32 * 564497911_i32;
_18 = _4;
_19 = &_2;
_16 = 10880263214303846978_usize as f64;
_19 = &(*_19);
_9 = &(*_9);
Goto(bb7)
}
bb7 = {
_11 = 111242732246307484068069185996536287747_u128;
_18 = _4;
_13 = _4;
_19 = &(*_19);
_13 = _4;
_19 = &(*_9);
_9 = &(*_19);
_17 = -_5;
_22 = !114636458162412092726184274027030446356_i128;
_3 = -_16;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
111242732246307484068069185996536287747 => bb9,
_ => bb8
}
}
bb8 = {
RET = _4 as i32;
RET = 141_u8 as i32;
_4 = '\u{98f4a}';
_3 = 3058100688766628705_u64 as f64;
RET = (-905226123_i32) | 1823355539_i32;
_1.0 = &_6;
_4 = '\u{2e18}';
Goto(bb2)
}
bb9 = {
_20 = [RET,RET,RET,RET,RET,RET,RET];
_1.0 = &_6;
_17 = _5;
_21 = &_12;
_23 = 13992853093927394176_u64 as isize;
_19 = &(*_9);
_11 = 154506512560855914139037041120004441508_u128;
_11 = 335580193108667164787593836430456537071_u128;
RET = (-2045927918_i32) >> _22;
_22 = _16 as i128;
_19 = &(*_9);
_18 = _8;
RET = 149695038_i32;
_23 = !9223372036854775807_isize;
_11 = 23234416958252584191592484680333194701_u128 + 260046790625663960421459709253209704544_u128;
_11 = !268323659756250116121193570836284198615_u128;
_7 = true;
_17 = _23 as f32;
_13 = _8;
_21 = &(*_21);
_4 = _8;
_16 = _3;
_9 = &(*_19);
_8 = _18;
_18 = _13;
match RET {
0 => bb1,
1 => bb5,
2 => bb4,
3 => bb10,
149695038 => bb12,
_ => bb11
}
}
bb10 = {
_9 = &(*_9);
_17 = _5;
_12 = [_8,_8,_8,_8,_13,_13,_4];
RET = 1708000646_i32 * 564497911_i32;
_18 = _4;
_19 = &_2;
_16 = 10880263214303846978_usize as f64;
_19 = &(*_19);
_9 = &(*_9);
Goto(bb7)
}
bb11 = {
_6 = 173286426776025505496888772361388364514_u128 as i16;
_11 = 97799448215545653634091663415796511522_u128 + 40788460305318740642004965739176525943_u128;
_13 = _8;
_5 = 145877849045522776790163693143483025144_i128 as f32;
RET = !1145202828_i32;
RET = (-331533208_i32);
_13 = _4;
_13 = _8;
_1.0 = &_6;
RET = 93_i8 as i32;
_9 = &_2;
_13 = _4;
_4 = _8;
_16 = -_3;
RET = !(-615126130_i32);
Goto(bb6)
}
bb12 = {
_26 = 79_u8 & 218_u8;
_25.1 = [_11];
_5 = -_17;
_3 = _16 * _16;
RET = 155320751_i32;
match RET {
0 => bb1,
1 => bb6,
2 => bb13,
3 => bb14,
155320751 => bb16,
_ => bb15
}
}
bb13 = {
_6 = 173286426776025505496888772361388364514_u128 as i16;
_11 = 97799448215545653634091663415796511522_u128 + 40788460305318740642004965739176525943_u128;
_13 = _8;
_5 = 145877849045522776790163693143483025144_i128 as f32;
RET = !1145202828_i32;
RET = (-331533208_i32);
_13 = _4;
_13 = _8;
_1.0 = &_6;
RET = 93_i8 as i32;
_9 = &_2;
_13 = _4;
_4 = _8;
_16 = -_3;
RET = !(-615126130_i32);
Goto(bb6)
}
bb14 = {
_4 = _8;
_4 = _8;
_3 = 2_i8 as f64;
_9 = &_2;
_1.0 = &_6;
_3 = (-9223372036854775808_isize) as f64;
_9 = &(*_9);
_5 = 9462594194026936135_u64 as f32;
_8 = _4;
_12 = [_8,_4,_4,_4,_8,_8,_8];
_8 = _4;
_1.0 = &_6;
_7 = _5 != _5;
RET = (-812020055_i32);
_3 = 63_u8 as f64;
_8 = _4;
_3 = 98633062237540770867977713761163473839_u128 as f64;
_5 = 48_i8 as f32;
_9 = &(*_9);
_1.0 = &_6;
RET = 0_usize as i32;
Goto(bb5)
}
bb15 = {
_11 = 111242732246307484068069185996536287747_u128;
_18 = _4;
_13 = _4;
_19 = &(*_19);
_13 = _4;
_19 = &(*_9);
_9 = &(*_19);
_17 = -_5;
_22 = !114636458162412092726184274027030446356_i128;
_3 = -_16;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
111242732246307484068069185996536287747 => bb9,
_ => bb8
}
}
bb16 = {
_20 = [RET,RET,RET,RET,RET,RET,RET];
_27 = (-39_i8) << _22;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(18_usize, 22_usize, Move(_22), 7_usize, Move(_7), 27_usize, Move(_27), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(18_usize, 6_usize, Move(_6), 23_usize, Move(_23), 29_usize, _29, 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19() -> isize {
mir! {
type RET = isize;
let _1: i128;
let _2: i128;
let _3: &'static *const usize;
let _4: i16;
let _5: i8;
let _6: [u64; 7];
let _7: bool;
let _8: bool;
let _9: i64;
let _10: (i16, usize);
let _11: &'static &'static bool;
let _12: i128;
let _13: isize;
let _14: Adt58;
let _15: f64;
let _16: isize;
let _17: char;
let _18: [isize; 2];
let _19: char;
let _20: u128;
let _21: [u8; 5];
let _22: &'static char;
let _23: ([char; 7], *const *mut i128, &'static bool);
let _24: f64;
let _25: isize;
let _26: ([i32; 8],);
let _27: f32;
let _28: ();
let _29: ();
{
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize | 102_isize;
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
RET = 187_u8 as isize;
Call(RET = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 9223372036854775807_isize | 41_isize;
_1 = (-48807818161840319159937149321203719992_i128) ^ 54870369172024989322712682104631761161_i128;
RET = -(-9223372036854775808_isize);
_2 = 4845117336633183614_u64 as i128;
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_2 = _1 << RET;
_2 = _1 << RET;
Goto(bb2)
}
bb2 = {
RET = 55_isize;
RET = 70_isize;
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
RET = 9223372036854775807_isize;
_4 = !7529_i16;
_5 = (-75_i8) & 30_i8;
_6 = [155914622130929946_u64,11957389844662841678_u64,12562225679465085194_u64,17407449860467791461_u64,8695685241266563457_u64,2389934433345283274_u64,13706845279290825099_u64];
_1 = _2;
_1 = RET as i128;
_7 = RET <= RET;
_2 = _1;
_8 = _7 & _7;
_1 = -_2;
RET = 9223372036854775807_isize * (-9223372036854775808_isize);
RET = (-9223372036854775808_isize);
_9 = (-3689584917407772271_i64);
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463459685022514360439185 => bb8,
_ => bb7
}
}
bb3 = {
RET = 9223372036854775807_isize | 41_isize;
_1 = (-48807818161840319159937149321203719992_i128) ^ 54870369172024989322712682104631761161_i128;
RET = -(-9223372036854775808_isize);
_2 = 4845117336633183614_u64 as i128;
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_2 = _1 << RET;
_2 = _1 << RET;
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
_6 = [1116752580009080829_u64,11200777936037155837_u64,13102911889881464884_u64,15026842639724612513_u64,16246590670106545534_u64,18367443722257063778_u64,3276308591799716550_u64];
_1 = !_2;
_9 = '\u{ce48b}' as i64;
_10.1 = _8 as usize;
_10.0 = _4;
_4 = _10.0;
_2 = _4 as i128;
match RET {
0 => bb4,
340282366920938463454151235394913435648 => bb9,
_ => bb2
}
}
bb9 = {
_10.0 = _4;
_13 = RET << _5;
_4 = '\u{cade0}' as i16;
_2 = !_1;
_7 = _8 == _8;
_1 = -_2;
_9 = (-9173884684726915965_i64) & 6232126041451638523_i64;
_5 = (-21_i8) & 75_i8;
_2 = _9 as i128;
_6 = [130562781607897149_u64,11298813327382734031_u64,1042753230703713926_u64,14887864310180885796_u64,11016840002618249921_u64,8043335810020006038_u64,7255552906943851323_u64];
_12 = -_2;
_13 = _1 as isize;
_9 = 58262_u16 as i64;
RET = _13 * _13;
_4 = !_10.0;
_2 = _12;
Goto(bb10)
}
bb10 = {
_14.fld2 = [(-890520197_i32),1153246295_i32,1310370686_i32,1564385882_i32,537147469_i32,1159434124_i32,537793484_i32,(-1304791175_i32)];
_4 = -_10.0;
_14.fld0.fld0 = (-1109685466_i32) ^ (-2077069290_i32);
_9 = 9146245707268008539_i64;
_6 = [6844652591802966498_u64,18047418055185963253_u64,15003134202507871014_u64,7623425841191673629_u64,15951159345133187750_u64,13378059253988393228_u64,14339370979439615540_u64];
RET = _9 as isize;
_1 = _2 - _12;
_12 = 2982725173_u32 as i128;
_14.fld0 = Adt24 { fld0: (-462587292_i32) };
_9 = (-7764940076176364471_i64) | (-1820940690601474283_i64);
_14.fld0.fld0 = (-752775901_i32) - 690171052_i32;
_6 = [401370060364766864_u64,3914701192064669553_u64,7679295852558615095_u64,10623099959419167564_u64,5139919844107837087_u64,6109238391648578123_u64,17483265710759460953_u64];
_14.fld2 = [_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0];
_14.fld0.fld0 = 1741453410_u32 as i32;
RET = _13 + _13;
_5 = 106_i8 - 47_i8;
RET = -_13;
_16 = RET - _13;
Goto(bb11)
}
bb11 = {
_16 = _13;
_15 = 233608905038063038736986327648134886276_u128 as f64;
Call(RET = core::intrinsics::bswap(_13), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_17 = '\u{3b130}';
_16 = -RET;
_8 = _7;
_12 = -_2;
RET = 137_u8 as isize;
_14.fld0.fld0 = -(-232872538_i32);
_10.1 = 1_usize & 1_usize;
_13 = _10.1 as isize;
_10.1 = _7 as usize;
_14.fld3 = [12857417591591303246_u64,9938103484648856621_u64,11714678802493883559_u64,11703536070916612349_u64,6161557608125455045_u64,6202146880885102747_u64];
_14.fld0 = Adt24 { fld0: (-1819629780_i32) };
_6 = [9974209679419033210_u64,11561338481034701664_u64,8689774079329048922_u64,17327372331299850199_u64,16680286666277291552_u64,10102620253931387681_u64,6775662023593938586_u64];
_10.0 = _4 + _4;
_5 = (-102_i8);
_5 = _2 as i8;
_14.fld0.fld0 = (-314383695_i32) & 1929811429_i32;
_9 = _7 as i64;
_8 = _7;
RET = !_13;
_18 = [RET,RET];
_10.0 = 1458973345_u32 as i16;
_13 = _16;
_8 = _7;
_5 = 42_i8 + 88_i8;
_19 = _17;
_14.fld3 = [9681566506579763185_u64,6760702028456088060_u64,14036524321695847780_u64,8259633121542460566_u64,13190959392072736298_u64,14009424393531347961_u64];
_13 = _16 * RET;
_16 = _5 as isize;
_8 = _7;
_8 = !_7;
Goto(bb13)
}
bb13 = {
_14.fld3 = [1504568415304482226_u64,9885792545053338146_u64,16976650619871381764_u64,10321005747202858968_u64,367255943573219807_u64,7493750463502226846_u64];
_14.fld0 = Adt24 { fld0: (-7278139_i32) };
_12 = _1 << _2;
_19 = _17;
_10.0 = 211428011138603913576993038262974616211_u128 as i16;
_2 = _12;
_17 = _19;
_13 = RET * _16;
_14.fld2 = [_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0,_14.fld0.fld0];
RET = _15 as isize;
_14.fld0.fld0 = 1726992053_i32;
_18 = [_16,RET];
_2 = _12;
_18 = [_16,_16];
_10.1 = 7052843166245002562_usize;
_16 = _13 * _13;
_1 = _9 as i128;
match _10.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb11,
4 => bb9,
5 => bb6,
7052843166245002562 => bb15,
_ => bb14
}
}
bb14 = {
_10.0 = _4;
_13 = RET << _5;
_4 = '\u{cade0}' as i16;
_2 = !_1;
_7 = _8 == _8;
_1 = -_2;
_9 = (-9173884684726915965_i64) & 6232126041451638523_i64;
_5 = (-21_i8) & 75_i8;
_2 = _9 as i128;
_6 = [130562781607897149_u64,11298813327382734031_u64,1042753230703713926_u64,14887864310180885796_u64,11016840002618249921_u64,8043335810020006038_u64,7255552906943851323_u64];
_12 = -_2;
_13 = _1 as isize;
_9 = 58262_u16 as i64;
RET = _13 * _13;
_4 = !_10.0;
_2 = _12;
Goto(bb10)
}
bb15 = {
_7 = _8 ^ _8;
_10.1 = _9 as usize;
_12 = _1;
_16 = _15 as isize;
_20 = !324920715333366375165674067945789501184_u128;
_12 = _2;
RET = !_13;
_17 = _19;
_10.1 = 6_usize ^ 2_usize;
_2 = -_1;
_17 = _19;
_22 = &_19;
_22 = &(*_22);
_1 = _12 - _12;
_13 = RET;
_23.0 = [_17,_19,_17,(*_22),(*_22),(*_22),_17];
_8 = !_7;
_11 = &_23.2;
RET = _13 << _9;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(19_usize, 17_usize, Move(_17), 13_usize, Move(_13), 20_usize, Move(_20), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(19_usize, 18_usize, Move(_18), 8_usize, Move(_8), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(106932791198093372281568345866048086292_i128), std::hint::black_box(2314330249581282336_usize), std::hint::black_box(59_u8), std::hint::black_box((-8442_i16)));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt24{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt24 {
fld0: i32,
}
impl PrintFDebug for Adt28{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt28{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt28 {
fld0: *mut i128,
fld1: i16,
}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt39{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt39 {
fld0: (i16, usize),
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f64,
fld1: Adt28,
fld2: *const usize,
fld3: u32,
fld4: *mut i128,

},
Variant1{
fld0: *mut isize,
fld1: [isize; 5],
fld2: usize,
fld3: u16,
fld4: [usize; 2],

},
Variant2{
fld0: *mut u8,
fld1: i32,
fld2: isize,

},
Variant3{
fld0: usize,
fld1: char,
fld2: [usize; 2],
fld3: i8,
fld4: [u64; 7],
fld5: i32,
fld6: f32,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: i128,
fld1: [u64; 6],

},
Variant1{
fld0: [usize; 2],
fld1: *const [char; 7],
fld2: i32,

},
Variant2{
fld0: u16,
fld1: Adt39,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: Adt24,
fld1: *mut u8,
fld2: [i32; 8],
fld3: [u64; 6],
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: u8,
fld1: *mut i64,
fld2: isize,
fld3: *mut isize,
fld4: f64,
fld5: [u64; 7],

},
Variant1{
fld0: bool,
fld1: char,
fld2: [u64; 7],
fld3: [char; 7],
fld4: Adt58,
fld5: [u32; 4],
fld6: *mut *mut u8,
fld7: i128,

},
Variant2{
fld0: Adt47,
fld1: [u128; 1],

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: [i32; 8],
fld1: i64,

},
Variant1{
fld0: *mut *mut u8,
fld1: *mut ([char; 7], *mut u8),
fld2: [i8; 2],
fld3: i128,
fld4: [u64; 7],

}}

