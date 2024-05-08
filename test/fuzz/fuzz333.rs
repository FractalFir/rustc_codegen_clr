#![allow(dead_code,unused_variables)]#![recursion_limit = "1024"]
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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> f64 {
mir! {
type RET = f64;
let _15: f32;
let _16: (i16, u64, i16, u64, bool, char);
let _17: u32;
let _18: *mut u16;
let _19: i32;
let _20: f32;
let _21: isize;
let _22: f32;
let _23: char;
let _24: u128;
let _25: isize;
let _26: isize;
let _27: i32;
let _28: Adt50;
let _29: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _30: Adt57;
let _31: i16;
let _32: [char; 1];
let _33: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _34: bool;
let _35: isize;
let _36: [char; 1];
let _37: (i64, i32, f64);
let _38: [u64; 6];
let _39: i16;
let _40: isize;
let _41: [u16; 2];
let _42: char;
let _43: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _44: [u128; 7];
let _45: Adt57;
let _46: i128;
let _47: ();
let _48: ();
{
RET = 138_u8 as f64;
_14 = !119222049154625081304642677102789456386_u128;
_8 = (-97_i8) as i128;
_2 = '\u{8638d}';
Call(_7 = fn1(RET, _2, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = RET as u32;
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = RET as i32;
_17 = _12 - _12;
_15 = 65250_u16 as f32;
_16.3 = 1213018366537008043_u64 & 8051106530663854883_u64;
RET = _7 as f64;
_16.1 = _16.3 + _16.3;
_16.4 = !false;
_10 = 1_usize as u8;
_16.4 = true;
_12 = _17;
_16.2 = 27103_i16;
_8 = (-5833070212545322611535590861547648330_i128) * 83949761869825166033950193676892294284_i128;
_16.5 = _2;
_1 = !_16.4;
RET = _14 as f64;
_4 = (-125_i8) * 6_i8;
_17 = _12 | _12;
_10 = _16.4 as u8;
_1 = _16.4;
_1 = _16.4;
Call(_11 = core::intrinsics::transmute(_16.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_18 = core::ptr::addr_of_mut!(_11);
_16.0 = _16.2;
_16.5 = _2;
(*_18) = 65315_u16;
_16.1 = _16.3 & _16.3;
_19 = _6;
_5 = _16.0;
RET = _8 as f64;
_16.0 = _17 as i16;
_17 = _12;
_16.4 = _1 ^ _1;
_19 = _6 & _6;
_16.0 = _16.2 | _16.2;
_18 = core::ptr::addr_of_mut!((*_18));
_16 = (_5, 10623326501109001401_u64, _5, 13612696033315499159_u64, _1, _2);
_15 = RET as f32;
(*_18) = !11942_u16;
_5 = _16.0 << _4;
_4 = 87_i8;
_9 = 4_usize;
_7 = -5789609505858819622_i64;
_18 = core::ptr::addr_of_mut!((*_18));
_7 = (-3626849339262909819_i64);
(*_18) = 56582_u16 ^ 2407_u16;
_6 = _19;
match _7 {
0 => bb4,
340282366920938463459747758092505301637 => bb6,
_ => bb5
}
}
bb4 = {
_6 = RET as i32;
_17 = _12 - _12;
_15 = 65250_u16 as f32;
_16.3 = 1213018366537008043_u64 & 8051106530663854883_u64;
RET = _7 as f64;
_16.1 = _16.3 + _16.3;
_16.4 = !false;
_10 = 1_usize as u8;
_16.4 = true;
_12 = _17;
_16.2 = 27103_i16;
_8 = (-5833070212545322611535590861547648330_i128) * 83949761869825166033950193676892294284_i128;
_16.5 = _2;
_1 = !_16.4;
RET = _14 as f64;
_4 = (-125_i8) * 6_i8;
_17 = _12 | _12;
_10 = _16.4 as u8;
_1 = _16.4;
_1 = _16.4;
Call(_11 = core::intrinsics::transmute(_16.2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_12 = RET as u32;
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_4 = _7 as i8;
(*_18) = 24569_u16 * 28586_u16;
_15 = _16.0 as f32;
(*_18) = 420_u16;
_11 = !42343_u16;
_22 = _15;
_15 = -_22;
_20 = -_22;
_22 = _15 - _15;
_16.3 = _16.1;
_16.2 = _5 * _5;
Goto(bb7)
}
bb7 = {
(*_18) = 395_u16 ^ 63787_u16;
_16 = (_5, 4303202621292839232_u64, _5, 6957210463688726905_u64, _1, _2);
_1 = !_16.4;
_26 = !9223372036854775807_isize;
_9 = 12589494952713627890_usize + 1_usize;
_25 = _9 as isize;
_15 = _22 * _22;
_13 = _16.4 as u64;
_4 = (-66_i8);
_27 = _19;
_6 = _19;
_16.2 = _5;
_28.fld4 = _16.1;
_5 = _16.2;
_5 = _16.2 * _16.0;
_29.1 = [_16.1,_16.1,_16.3,_16.3,_16.3,_16.1];
_28.fld3.fld3.1 = _16.3 + _16.1;
Goto(bb8)
}
bb8 = {
_28.fld3.fld1 = RET as i16;
_28.fld3.fld1 = _16.0;
_29.1 = [_16.3,_16.3,_28.fld4,_16.3,_28.fld4,_28.fld4];
_28.fld1 = _29.1;
_28.fld3.fld0.2 = [_11,(*_18)];
_16.4 = _28.fld4 <= _16.1;
_29.0.3 = _16.1;
_29.0.4 = !_16.4;
_33.0.5 = _2;
_33.4 = _14 | _14;
_28.fld3.fld0.0 = _9 | _9;
_10 = 110_u8;
_28.fld0.2 = _11;
RET = _12 as f64;
_29.0.0 = !_16.2;
_16 = (_28.fld3.fld1, _28.fld3.fld3.1, _29.0.0, _28.fld3.fld3.1, _1, _2);
_19 = _6;
_28.fld3.fld3.0 = _10 as i16;
_28.fld3.fld3.5 = _2;
_28.fld3.fld3 = _16;
_29.4 = !_14;
RET = _9 as f64;
_29.2 = !_12;
_2 = _16.5;
_28.fld1 = _29.1;
Goto(bb9)
}
bb9 = {
_33.0.3 = _28.fld3.fld3.3 % _29.0.3;
_33.0.5 = _16.5;
_32 = [_16.5];
_28.fld3.fld0.0 = _9 - _9;
_37 = (_7, _19, RET);
_3 = _2 as isize;
_29.0.5 = _2;
_16.4 = _29.0.4;
_37.1 = _19;
_29.0 = (_5, _16.3, _28.fld3.fld3.0, _28.fld3.fld3.1, _16.4, _2);
_12 = _17 + _29.2;
_28.fld2 = Adt44::Variant0 { fld0: _29.0.0,fld1: _28.fld3.fld3.3,fld2: _28.fld3.fld0.0,fld3: _4 };
_33.0.5 = _28.fld3.fld3.5;
_27 = _37.1;
match _28.fld4 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
4 => bb10,
4303202621292839232 => bb12,
_ => bb11
}
}
bb10 = {
_12 = RET as u32;
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_4 = _7 as i8;
(*_18) = 24569_u16 * 28586_u16;
_15 = _16.0 as f32;
(*_18) = 420_u16;
_11 = !42343_u16;
_22 = _15;
_15 = -_22;
_20 = -_22;
_22 = _15 - _15;
_16.3 = _16.1;
_16.2 = _5 * _5;
Goto(bb7)
}
bb12 = {
_19 = _27;
_33.3 = _10 + _10;
_31 = !_29.0.2;
SetDiscriminant(_28.fld2, 2);
_28.fld3.fld2 = _32;
_35 = _33.0.5 as isize;
_37.1 = _12 as i32;
_29.0 = _28.fld3.fld3;
_16.1 = _37.2 as u64;
_28.fld0.1 = (*_18) as i64;
_20 = _4 as f32;
_28.fld0.0 = (*_18);
_28.fld3.fld3.2 = _8 as i16;
_28.fld0 = (_11, _37.0, _11);
_28.fld3.fld3.0 = !_16.0;
_28.fld0 = (_11, _37.0, (*_18));
_33.0.0 = _5;
_29.0.4 = !_16.4;
_21 = !_25;
_29.0.5 = _28.fld3.fld3.5;
_14 = _10 as u128;
_26 = _25;
_27 = _22 as i32;
_33.0.1 = _29.0.3 * _28.fld3.fld3.3;
match _37.0 {
0 => bb1,
340282366920938463459747758092505301637 => bb14,
_ => bb13
}
}
bb13 = {
_28.fld3.fld1 = RET as i16;
_28.fld3.fld1 = _16.0;
_29.1 = [_16.3,_16.3,_28.fld4,_16.3,_28.fld4,_28.fld4];
_28.fld1 = _29.1;
_28.fld3.fld0.2 = [_11,(*_18)];
_16.4 = _28.fld4 <= _16.1;
_29.0.3 = _16.1;
_29.0.4 = !_16.4;
_33.0.5 = _2;
_33.4 = _14 | _14;
_28.fld3.fld0.0 = _9 | _9;
_10 = 110_u8;
_28.fld0.2 = _11;
RET = _12 as f64;
_29.0.0 = !_16.2;
_16 = (_28.fld3.fld1, _28.fld3.fld3.1, _29.0.0, _28.fld3.fld3.1, _1, _2);
_19 = _6;
_28.fld3.fld3.0 = _10 as i16;
_28.fld3.fld3.5 = _2;
_28.fld3.fld3 = _16;
_29.4 = !_14;
RET = _9 as f64;
_29.2 = !_12;
_2 = _16.5;
_28.fld1 = _29.1;
Goto(bb9)
}
bb14 = {
_28.fld3.fld0.2 = [(*_18),_28.fld0.0];
_33 = (_28.fld3.fld3, _28.fld1, _12, _10, _29.4);
_29.1 = [_16.3,_16.3,_33.0.3,_29.0.3,_33.0.3,_33.0.1];
_16.4 = _29.0.4;
_19 = _29.0.4 as i32;
_28.fld3.fld3.5 = _29.0.5;
_29 = (_16, _28.fld1, _12, _10, _14);
_28.fld3.fld3.2 = -_33.0.2;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(0_usize, 13_usize, Move(_13), 25_usize, Move(_25), 31_usize, Move(_31), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(0_usize, 11_usize, Move(_11), 9_usize, Move(_9), 33_usize, Move(_33), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(0_usize, 29_usize, Move(_29), 14_usize, Move(_14), 16_usize, Move(_16), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(0_usize, 2_usize, Move(_2), 48_usize, _48, 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: f64,mut _2: char,mut _3: i128) -> i64 {
mir! {
type RET = i64;
let _4: [u8; 7];
let _5: f32;
let _6: Adt44;
let _7: [char; 1];
let _8: char;
let _9: isize;
let _10: i64;
let _11: Adt42;
let _12: (i16, u64, i16, u64, bool, char);
let _13: (i64, i32, f64);
let _14: u128;
let _15: [u16; 2];
let _16: [isize; 6];
let _17: (i64, i32, f64);
let _18: isize;
let _19: isize;
let _20: isize;
let _21: f64;
let _22: i16;
let _23: bool;
let _24: [isize; 6];
let _25: [char; 1];
let _26: [u16; 2];
let _27: (i64, i32, f64);
let _28: isize;
let _29: isize;
let _30: *mut *const f64;
let _31: f32;
let _32: ();
let _33: ();
{
RET = (-1862555209_i32) as i64;
RET = !2978883428705732703_i64;
_1 = 22087_i16 as f64;
RET = 7469067570071337310_i64;
_3 = 138924407888664610422382143887634307467_i128;
RET = -6060649836288376761_i64;
_2 = '\u{b01f3}';
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
138924407888664610422382143887634307467 => bb8,
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
_4 = [31_u8,168_u8,206_u8,144_u8,194_u8,10_u8,33_u8];
_2 = '\u{86757}';
_2 = '\u{24320}';
_5 = 12367054446267217604_usize as f32;
_5 = 9223372036854775807_isize as f32;
_3 = -(-2470373463611871436907500404882659161_i128);
RET = 37830585883108285851213870278730556181_u128 as i64;
RET = 4303690078226237811_i64;
_5 = _3 as f32;
_6 = Adt44::Variant2 { fld0: 38_i8 };
Call(RET = fn2(_5, _4, _5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<i8>(Variant(_6, 2), 0)) = !(-40_i8);
_3 = 184_u8 as i128;
place!(Field::<i8>(Variant(_6, 2), 0)) = (-26_i8) - (-8_i8);
RET = 2816357457091443724_i64 * 7074186349664575302_i64;
Goto(bb10)
}
bb10 = {
_5 = Field::<i8>(Variant(_6, 2), 0) as f32;
place!(Field::<i8>(Variant(_6, 2), 0)) = (-124_i8) - (-61_i8);
_5 = 15570198611087308435_usize as f32;
_10 = !RET;
_12.5 = _2;
RET = _10 - _10;
_6 = Adt44::Variant0 { fld0: 6366_i16,fld1: 14466861400985795060_u64,fld2: 7_usize,fld3: 51_i8 };
place!(Field::<u64>(Variant(_6, 0), 1)) = 11679979185029749727_u64 | 13875636447287874680_u64;
_12 = ((-24364_i16), Field::<u64>(Variant(_6, 0), 1), 31476_i16, Field::<u64>(Variant(_6, 0), 1), true, _2);
Goto(bb11)
}
bb11 = {
_13.2 = _1;
_5 = (-9223372036854775808_isize) as f32;
_13.0 = 214_u8 as i64;
_7 = [_12.5];
_12.4 = !true;
_9 = 9223372036854775807_isize << _12.0;
_8 = _12.5;
_12.5 = _8;
place!(Field::<i16>(Variant(_6, 0), 0)) = _12.2;
Goto(bb12)
}
bb12 = {
_17.1 = !942506554_i32;
_16 = [_9,_9,_9,_9,_9,_9];
_9 = 35_isize;
_7 = [_12.5];
_12.0 = Field::<i16>(Variant(_6, 0), 0) * _12.2;
_17.0 = -RET;
_8 = _12.5;
_20 = _9 ^ _9;
_19 = _20 * _20;
_13.1 = _5 as i32;
RET = 7_usize as i64;
_15 = [44786_u16,27190_u16];
_3 = 88352838964955687308545668401685430606_i128 - 135599335927275233478912609576188301175_i128;
_5 = _12.3 as f32;
_17.1 = _9 as i32;
_7 = [_8];
_17.2 = _1;
_3 = !(-4808506133767491465190027809478435313_i128);
_12.2 = _5 as i16;
RET = _10 & _17.0;
Goto(bb13)
}
bb13 = {
_18 = -_20;
RET = -_13.0;
place!(Field::<usize>(Variant(_6, 0), 2)) = !716237858077895559_usize;
place!(Field::<usize>(Variant(_6, 0), 2)) = 3064216991778199874_usize;
_7 = [_2];
_19 = _9 + _9;
_22 = _12.0 * Field::<i16>(Variant(_6, 0), 0);
place!(Field::<i8>(Variant(_6, 0), 3)) = -(-76_i8);
_16 = [_18,_19,_20,_19,_9,_18];
_6 = Adt44::Variant0 { fld0: _22,fld1: _12.1,fld2: 1820185577824756603_usize,fld3: (-1_i8) };
_24 = _16;
_28 = _22 as isize;
_13.0 = RET;
Call(_17.0 = core::intrinsics::bswap(RET), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_19 = _13.2 as isize;
_27.2 = _17.0 as f64;
_3 = _5 as i128;
_17.1 = _28 as i32;
_27.1 = -_17.1;
_27 = (_17.0, _17.1, _17.2);
_12.3 = !_12.1;
place!(Field::<u64>(Variant(_6, 0), 1)) = 35785_u16 as u64;
_12.0 = Field::<i16>(Variant(_6, 0), 0) >> _28;
_29 = _5 as isize;
_14 = !58144322738368579039835878799201470371_u128;
_18 = _28 & _28;
_25 = [_12.5];
_20 = _19;
_12.4 = true;
_14 = 177646560077789311248419936357173556410_u128 & 7019987954392478167194354649705430179_u128;
place!(Field::<usize>(Variant(_6, 0), 2)) = !14664833829524506882_usize;
_14 = 49747437087760181167449394203134630171_u128;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(1_usize, 16_usize, Move(_16), 22_usize, Move(_22), 3_usize, Move(_3), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(1_usize, 24_usize, Move(_24), 14_usize, Move(_14), 10_usize, Move(_10), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(1_usize, 19_usize, Move(_19), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f32,mut _2: [u8; 7],mut _3: f32) -> i64 {
mir! {
type RET = i64;
let _4: [u8; 7];
let _5: char;
let _6: (i64, i32, f64);
let _7: *mut u16;
let _8: Adt48;
let _9: bool;
let _10: [char; 1];
let _11: (u16,);
let _12: [u16; 2];
let _13: &'static char;
let _14: i128;
let _15: (usize, u8, [u16; 2]);
let _16: u8;
let _17: u8;
let _18: f32;
let _19: [isize; 6];
let _20: &'static char;
let _21: (&'static char, (u64, i32, usize));
let _22: *mut usize;
let _23: *mut u16;
let _24: (i16, u64, i16, u64, bool, char);
let _25: isize;
let _26: [u128; 7];
let _27: i8;
let _28: isize;
let _29: Adt43;
let _30: Adt41;
let _31: i16;
let _32: [u16; 2];
let _33: [u8; 7];
let _34: isize;
let _35: u16;
let _36: bool;
let _37: Adt55;
let _38: f32;
let _39: ();
let _40: ();
{
RET = 95892117163407118489237186677704985252_u128 as i64;
_2 = [187_u8,102_u8,211_u8,135_u8,197_u8,211_u8,32_u8];
_2 = [176_u8,136_u8,74_u8,213_u8,41_u8,122_u8,178_u8];
RET = (-1295622704777937827_i64) << (-9223372036854775808_isize);
RET = !(-10517385048826818_i64);
RET = (-4358245592024817201_i64);
_4 = _2;
RET = 697065673_u32 as i64;
_4 = [75_u8,205_u8,128_u8,4_u8,3_u8,125_u8,15_u8];
_1 = _3;
RET = !(-2143936575124683479_i64);
RET = 8280392446879280225_i64 * 156692475241968327_i64;
RET = 2300850941348194889_i64 >> (-1962725543_i32);
_1 = _3 * _3;
_1 = _3;
_5 = '\u{52aa1}';
_2 = [183_u8,7_u8,58_u8,153_u8,224_u8,36_u8,131_u8];
RET = _5 as i64;
_4 = [160_u8,86_u8,206_u8,134_u8,124_u8,159_u8,205_u8];
RET = (-6184526158111133436_i64);
RET = 829881775594001100_u64 as i64;
Goto(bb1)
}
bb1 = {
RET = !(-6463707729042360921_i64);
_3 = -_1;
Goto(bb2)
}
bb2 = {
_6.1 = (-1113473627_i32);
RET = 6911954203042746860_i64 >> _6.1;
_4 = [75_u8,230_u8,252_u8,39_u8,41_u8,248_u8,166_u8];
_6.0 = RET;
RET = _6.0;
_2 = [144_u8,157_u8,38_u8,71_u8,80_u8,129_u8,118_u8];
_7 = core::ptr::addr_of_mut!(_11.0);
RET = _6.0;
_6.2 = 966064885_u32 as f64;
_6.0 = RET;
_9 = true;
_9 = _6.1 != _6.1;
_11 = (22323_u16,);
_6.2 = _6.1 as f64;
_4 = [114_u8,152_u8,27_u8,94_u8,32_u8,130_u8,128_u8];
_7 = core::ptr::addr_of_mut!((*_7));
_11.0 = 36356_u16;
(*_7) = !5702_u16;
_10 = [_5];
_2 = [57_u8,125_u8,31_u8,99_u8,90_u8,194_u8,141_u8];
_3 = -_1;
_11 = (5588_u16,);
Call(_11.0 = fn3(_4, _4, _6.2, _4, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.2 = 3164852513184408925_u64 as f64;
_3 = 9223372036854775807_isize as f32;
_11.0 = 58375_u16 & 11233_u16;
RET = _6.0 | _6.0;
_4 = _2;
_9 = true & true;
_6.1 = 112460922555506370828791081118591393322_u128 as i32;
(*_7) = 26251_u16;
_6.1 = _6.2 as i32;
(*_7) = 58_i8 as u16;
_9 = _6.2 > _6.2;
_13 = &_5;
RET = _6.0 * _6.0;
_4 = _2;
_4 = _2;
_4 = [78_u8,70_u8,28_u8,134_u8,40_u8,202_u8,27_u8];
RET = _6.0;
Goto(bb4)
}
bb4 = {
_12 = [(*_7),(*_7)];
_17 = 4160028848843122005_usize as u8;
_6.1 = _9 as i32;
_15.2 = [(*_7),(*_7)];
_16 = (-9223372036854775808_isize) as u8;
_7 = core::ptr::addr_of_mut!(_11.0);
_2 = _4;
_1 = -_3;
_5 = '\u{2c3f0}';
_18 = _3 * _1;
_12 = [(*_7),_11.0];
Goto(bb5)
}
bb5 = {
_4 = [_17,_17,_17,_17,_17,_16,_16];
_11 = (26294_u16,);
Goto(bb6)
}
bb6 = {
_13 = &_5;
_15.1 = 280343941617588433882838036053227064638_u128 as u8;
_7 = core::ptr::addr_of_mut!((*_7));
_15.2 = [_11.0,_11.0];
_15 = (2_usize, _17, _12);
_21.0 = &(*_13);
_21.1.2 = 20841_i16 as usize;
_21.1 = (3517272006375165573_u64, _6.1, _15.0);
Goto(bb7)
}
bb7 = {
_14 = !39558614508094256194737393557616959165_i128;
_7 = core::ptr::addr_of_mut!((*_7));
_15.1 = _16 >> _21.1.0;
_21.1.0 = !15644586646480847699_u64;
_2 = [_17,_17,_15.1,_15.1,_15.1,_15.1,_15.1];
_2 = [_15.1,_15.1,_17,_17,_15.1,_15.1,_16];
_21.1.0 = (*_13) as u64;
_21.0 = Move(_13);
(*_7) = 2697166040_u32 as u16;
_15.2 = _12;
Goto(bb8)
}
bb8 = {
_6.0 = _15.0 as i64;
_15.2 = _12;
_27 = -(-62_i8);
_26 = [182310609795911085876610073850528534442_u128,114172489432716384129229914455527649456_u128,32308383279784219269314501706174466680_u128,49494854363878423065319570419114237011_u128,222921605033506332455884218717428692788_u128,104781993963127312628946509056542124795_u128,216661356196246027786272619203278813938_u128];
_23 = core::ptr::addr_of_mut!((*_7));
_26 = [178927474675684231743440303259161650756_u128,144784919331904061692135706176940281072_u128,26689126036585396365411663325529442323_u128,189120168609210699427648372403625455754_u128,251103621409676782756146617631665688790_u128,155520834568909225424653856831881647071_u128,157972756248905792655473187185864023039_u128];
_24.0 = 1205519022_u32 as i16;
(*_23) = _9 as u16;
_24 = (30111_i16, _21.1.0, (-11395_i16), _21.1.0, _9, _5);
_24.3 = _27 as u64;
RET = -_6.0;
_22 = core::ptr::addr_of_mut!(_15.0);
_21.1.1 = _6.1;
Goto(bb9)
}
bb9 = {
_18 = _1;
_29.fld2 = [_5];
_1 = -_3;
_11 = (23316_u16,);
_28 = 1253134198_u32 as isize;
_12 = [(*_7),(*_23)];
_29.fld0.1 = !_15.1;
_24.3 = _21.1.0 - _21.1.0;
_29.fld0.1 = _15.1 << _17;
_24 = (14594_i16, _21.1.0, 13470_i16, _21.1.0, _9, _5);
_10 = _29.fld2;
_29.fld3.2 = _6.0 as i16;
_11.0 = 39071_u16;
_20 = &_24.5;
_29.fld0.0 = (*_22) >> _29.fld0.1;
_14 = -75643094871412859438453877439848356074_i128;
_13 = &(*_20);
_29.fld3.3 = _24.1 ^ _21.1.0;
match (*_22) {
0 => bb6,
1 => bb10,
2 => bb12,
_ => bb11
}
}
bb10 = {
_6.0 = _15.0 as i64;
_15.2 = _12;
_27 = -(-62_i8);
_26 = [182310609795911085876610073850528534442_u128,114172489432716384129229914455527649456_u128,32308383279784219269314501706174466680_u128,49494854363878423065319570419114237011_u128,222921605033506332455884218717428692788_u128,104781993963127312628946509056542124795_u128,216661356196246027786272619203278813938_u128];
_23 = core::ptr::addr_of_mut!((*_7));
_26 = [178927474675684231743440303259161650756_u128,144784919331904061692135706176940281072_u128,26689126036585396365411663325529442323_u128,189120168609210699427648372403625455754_u128,251103621409676782756146617631665688790_u128,155520834568909225424653856831881647071_u128,157972756248905792655473187185864023039_u128];
_24.0 = 1205519022_u32 as i16;
(*_23) = _9 as u16;
_24 = (30111_i16, _21.1.0, (-11395_i16), _21.1.0, _9, _5);
_24.3 = _27 as u64;
RET = -_6.0;
_22 = core::ptr::addr_of_mut!(_15.0);
_21.1.1 = _6.1;
Goto(bb9)
}
bb11 = {
_12 = [(*_7),(*_7)];
_17 = 4160028848843122005_usize as u8;
_6.1 = _9 as i32;
_15.2 = [(*_7),(*_7)];
_16 = (-9223372036854775808_isize) as u8;
_7 = core::ptr::addr_of_mut!(_11.0);
_2 = _4;
_1 = -_3;
_5 = '\u{2c3f0}';
_18 = _3 * _1;
_12 = [(*_7),_11.0];
Goto(bb5)
}
bb12 = {
_29.fld3.5 = _24.5;
_30 = Adt41::Variant1 { fld0: _24.3,fld1: _22,fld2: _28,fld3: _11 };
_26 = [267275446483840726837838210370721973629_u128,157193512785217985922457613232387100120_u128,51161709543954547224942874394142316894_u128,159117488983247791699811308281603230455_u128,31370537378626312420799310754249280028_u128,16154592364187217035992083068937227522_u128,65279478563869846670072477105329347130_u128];
RET = _14 as i64;
_30 = Adt41::Variant0 { fld0: _21.1 };
_25 = !_28;
SetDiscriminant(_30, 1);
place!(Field::<(u16,)>(Variant(_30, 1), 3)).0 = (*_23);
place!(Field::<*mut usize>(Variant(_30, 1), 1)) = core::ptr::addr_of_mut!(_21.1.2);
(*_23) = !Field::<(u16,)>(Variant(_30, 1), 3).0;
_21.1.1 = -_6.1;
_4 = [_15.1,_29.fld0.1,_15.1,_15.1,_29.fld0.1,_29.fld0.1,_15.1];
_29.fld3.1 = _24.1 & _24.1;
match _24.2 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb9,
4 => bb13,
13470 => bb15,
_ => bb14
}
}
bb13 = {
_6.1 = (-1113473627_i32);
RET = 6911954203042746860_i64 >> _6.1;
_4 = [75_u8,230_u8,252_u8,39_u8,41_u8,248_u8,166_u8];
_6.0 = RET;
RET = _6.0;
_2 = [144_u8,157_u8,38_u8,71_u8,80_u8,129_u8,118_u8];
_7 = core::ptr::addr_of_mut!(_11.0);
RET = _6.0;
_6.2 = 966064885_u32 as f64;
_6.0 = RET;
_9 = true;
_9 = _6.1 != _6.1;
_11 = (22323_u16,);
_6.2 = _6.1 as f64;
_4 = [114_u8,152_u8,27_u8,94_u8,32_u8,130_u8,128_u8];
_7 = core::ptr::addr_of_mut!((*_7));
_11.0 = 36356_u16;
(*_7) = !5702_u16;
_10 = [_5];
_2 = [57_u8,125_u8,31_u8,99_u8,90_u8,194_u8,141_u8];
_3 = -_1;
_11 = (5588_u16,);
Call(_11.0 = fn3(_4, _4, _6.2, _4, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
RET = !(-6463707729042360921_i64);
_3 = -_1;
Goto(bb2)
}
bb15 = {
(*_7) = !Field::<(u16,)>(Variant(_30, 1), 3).0;
_6.2 = _27 as f64;
_6.1 = _3 as i32;
_4 = [_29.fld0.1,_15.1,_15.1,_29.fld0.1,_15.1,_29.fld0.1,_16];
_13 = &_5;
_19 = [_25,_28,_25,_25,_28,_28];
(*_23) = Field::<(u16,)>(Variant(_30, 1), 3).0;
place!(Field::<isize>(Variant(_30, 1), 2)) = _28 << _29.fld3.2;
_31 = -_24.0;
_25 = 14348193484697216125229914895569601573_u128 as isize;
_4 = [_29.fld0.1,_29.fld0.1,_15.1,_15.1,_15.1,_29.fld0.1,_29.fld0.1];
_29 = Adt43 { fld0: _15,fld1: _31,fld2: _10,fld3: _24 };
_6.0 = RET;
_29.fld3.5 = _24.5;
_21.1.2 = _29.fld0.0 - (*_22);
_13 = Move(_20);
_38 = _15.1 as f32;
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(2_usize, 4_usize, Move(_4), 10_usize, Move(_10), 19_usize, Move(_19), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(2_usize, 16_usize, Move(_16), 14_usize, Move(_14), 24_usize, Move(_24), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(2_usize, 5_usize, Move(_5), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: f64,mut _4: [u8; 7],mut _5: [u8; 7]) -> u16 {
mir! {
type RET = u16;
let _6: i64;
let _7: (u64, u128);
let _8: Adt41;
let _9: f32;
let _10: Adt53;
let _11: f32;
let _12: bool;
let _13: Adt44;
let _14: [u64; 6];
let _15: isize;
let _16: Adt45;
let _17: [u128; 7];
let _18: ();
let _19: ();
{
_2 = _1;
_4 = [67_u8,171_u8,209_u8,66_u8,156_u8,83_u8,22_u8];
_2 = _1;
_3 = 32137_u16 as f64;
_2 = _5;
_4 = _5;
_6 = -(-299069410404919454_i64);
_1 = [123_u8,94_u8,24_u8,50_u8,149_u8,160_u8,51_u8];
_2 = [120_u8,28_u8,1_u8,4_u8,31_u8,149_u8,47_u8];
_6 = (-8330078759462724204_i64) & (-3330113203562605720_i64);
_6 = (-26_i8) as i64;
Goto(bb1)
}
bb1 = {
RET = !11053_u16;
_3 = (-3438_i16) as f64;
Call(RET = fn4(_2, _4, _3, _5, _4, _4, _2, _2, _4, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = 39_i8 as f64;
_7.0 = 1691044638779559773_u64 & 5035746674194531904_u64;
Goto(bb3)
}
bb3 = {
_7 = (13609560119541117771_u64, 318964348627802220427196168545379860513_u128);
_6 = (-8712115953949240464_i64);
RET = 219_u8 as u16;
_6 = _7.1 as i64;
_1 = _2;
_7.0 = _3 as u64;
_4 = [17_u8,127_u8,198_u8,63_u8,204_u8,45_u8,70_u8];
_3 = (-90016408190123427615580674970171483401_i128) as f64;
_7 = (14047102479715085811_u64, 96896990236918566408759808945195675072_u128);
_7.0 = 2548683176771279400_u64;
RET = !48609_u16;
_7.1 = 109981316396084150128698292795438543541_u128;
_1 = _2;
_6 = -(-3065137391741211829_i64);
_2 = [187_u8,66_u8,29_u8,196_u8,230_u8,63_u8,91_u8];
_3 = _7.0 as f64;
_7 = (15352322031198376988_u64, 126281547213468790071631404035990655957_u128);
_4 = [76_u8,153_u8,182_u8,104_u8,154_u8,84_u8,133_u8];
_7.0 = 17837628849700669095_u64 & 1479884972320101162_u64;
_1 = [210_u8,151_u8,167_u8,10_u8,11_u8,112_u8,253_u8];
_7.1 = 97873294075289432882834389269313265039_u128 >> RET;
_7 = (14155702991536628061_u64, 217978197993231756924662868745533152329_u128);
match _7.0 {
0 => bb4,
1 => bb5,
14155702991536628061 => bb7,
_ => bb6
}
}
bb4 = {
_3 = 39_i8 as f64;
_7.0 = 1691044638779559773_u64 & 5035746674194531904_u64;
Goto(bb3)
}
bb5 = {
RET = !11053_u16;
_3 = (-3438_i16) as f64;
Call(RET = fn4(_2, _4, _3, _5, _4, _4, _2, _2, _4, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
RET = 38027_u16 >> _7.1;
_7.0 = 7627905571126449422_u64 >> RET;
_2 = _4;
_7.1 = 51273561282104666727793513236789213587_u128 + 248657001071553797983034387867402170891_u128;
RET = !19837_u16;
RET = !41645_u16;
_7.0 = 4650308945296053160_u64 & 12225602203853307663_u64;
_4 = [148_u8,129_u8,190_u8,152_u8,12_u8,245_u8,211_u8];
_6 = (-25777275175771727649315808925928290831_i128) as i64;
_7.0 = 730334727886932516_u64 ^ 885810063958907123_u64;
_7.1 = 134185466565511127160331609796367158607_u128 | 126674512369872376802497576281973160195_u128;
Goto(bb8)
}
bb8 = {
_7.0 = _7.1 as u64;
RET = 31866_u16 | 28608_u16;
_11 = RET as f32;
_2 = [51_u8,209_u8,192_u8,233_u8,44_u8,203_u8,72_u8];
_5 = [17_u8,222_u8,28_u8,178_u8,150_u8,11_u8,39_u8];
_3 = 3866_i16 as f64;
_9 = _11;
_5 = [202_u8,219_u8,137_u8,182_u8,238_u8,57_u8,207_u8];
_4 = _1;
_2 = [51_u8,92_u8,28_u8,186_u8,175_u8,253_u8,219_u8];
_3 = 2385823694_u32 as f64;
_2 = [79_u8,106_u8,243_u8,11_u8,37_u8,181_u8,99_u8];
_6 = (-1342281748933227285_i64);
_6 = !(-6847261939414005420_i64);
_7.0 = !10017004568141637716_u64;
_3 = _7.0 as f64;
_9 = _3 as f32;
_11 = -_9;
Goto(bb9)
}
bb9 = {
_3 = (-9223372036854775808_isize) as f64;
_6 = 5973127950540933029_i64 ^ (-2330616768495196378_i64);
RET = 2568_u16;
_4 = [214_u8,83_u8,150_u8,58_u8,172_u8,147_u8,107_u8];
RET = 7475_u16 | 50398_u16;
_4 = [53_u8,126_u8,123_u8,83_u8,137_u8,55_u8,230_u8];
_3 = 2102479016_i32 as f64;
_3 = (-9223372036854775808_isize) as f64;
_4 = [211_u8,91_u8,113_u8,77_u8,158_u8,69_u8,84_u8];
_1 = [211_u8,80_u8,18_u8,178_u8,48_u8,117_u8,181_u8];
Goto(bb10)
}
bb10 = {
_9 = _11;
_9 = _11 + _11;
_7 = (11695605352804841_u64, 281480500507523595596972168261816062004_u128);
_13 = Adt44::Variant2 { fld0: 89_i8 };
_2 = _1;
_2 = [68_u8,41_u8,74_u8,19_u8,119_u8,74_u8,47_u8];
place!(Field::<i8>(Variant(_13, 2), 0)) = 24530_i16 as i8;
RET = 43109_u16;
_2 = [179_u8,15_u8,13_u8,86_u8,125_u8,182_u8,238_u8];
_9 = _11 - _11;
_15 = _11 as isize;
_9 = RET as f32;
_4 = _2;
_9 = -_11;
SetDiscriminant(_13, 0);
RET = 24472_u16 ^ 38473_u16;
place!(Field::<usize>(Variant(_13, 0), 2)) = 4_usize * 4_usize;
_11 = _9;
_1 = _5;
_12 = _11 != _11;
place!(Field::<u64>(Variant(_13, 0), 1)) = _7.0;
RET = _6 as u16;
place!(Field::<i16>(Variant(_13, 0), 0)) = -32635_i16;
_16.fld5 = (_7.0, (-1511065198_i32), Field::<usize>(Variant(_13, 0), 2));
_5 = [236_u8,21_u8,65_u8,3_u8,11_u8,93_u8,21_u8];
place!(Field::<u64>(Variant(_13, 0), 1)) = !_7.0;
match _16.fld5.0 {
0 => bb3,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
11695605352804841 => bb18,
_ => bb17
}
}
bb11 = {
RET = !11053_u16;
_3 = (-3438_i16) as f64;
Call(RET = fn4(_2, _4, _3, _5, _4, _4, _2, _2, _4, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_3 = 39_i8 as f64;
_7.0 = 1691044638779559773_u64 & 5035746674194531904_u64;
Goto(bb3)
}
bb13 = {
RET = 38027_u16 >> _7.1;
_7.0 = 7627905571126449422_u64 >> RET;
_2 = _4;
_7.1 = 51273561282104666727793513236789213587_u128 + 248657001071553797983034387867402170891_u128;
RET = !19837_u16;
RET = !41645_u16;
_7.0 = 4650308945296053160_u64 & 12225602203853307663_u64;
_4 = [148_u8,129_u8,190_u8,152_u8,12_u8,245_u8,211_u8];
_6 = (-25777275175771727649315808925928290831_i128) as i64;
_7.0 = 730334727886932516_u64 ^ 885810063958907123_u64;
_7.1 = 134185466565511127160331609796367158607_u128 | 126674512369872376802497576281973160195_u128;
Goto(bb8)
}
bb14 = {
Return()
}
bb15 = {
RET = !11053_u16;
_3 = (-3438_i16) as f64;
Call(RET = fn4(_2, _4, _3, _5, _4, _4, _2, _2, _4, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_3 = 39_i8 as f64;
_7.0 = 1691044638779559773_u64 & 5035746674194531904_u64;
Goto(bb3)
}
bb17 = {
_7 = (13609560119541117771_u64, 318964348627802220427196168545379860513_u128);
_6 = (-8712115953949240464_i64);
RET = 219_u8 as u16;
_6 = _7.1 as i64;
_1 = _2;
_7.0 = _3 as u64;
_4 = [17_u8,127_u8,198_u8,63_u8,204_u8,45_u8,70_u8];
_3 = (-90016408190123427615580674970171483401_i128) as f64;
_7 = (14047102479715085811_u64, 96896990236918566408759808945195675072_u128);
_7.0 = 2548683176771279400_u64;
RET = !48609_u16;
_7.1 = 109981316396084150128698292795438543541_u128;
_1 = _2;
_6 = -(-3065137391741211829_i64);
_2 = [187_u8,66_u8,29_u8,196_u8,230_u8,63_u8,91_u8];
_3 = _7.0 as f64;
_7 = (15352322031198376988_u64, 126281547213468790071631404035990655957_u128);
_4 = [76_u8,153_u8,182_u8,104_u8,154_u8,84_u8,133_u8];
_7.0 = 17837628849700669095_u64 & 1479884972320101162_u64;
_1 = [210_u8,151_u8,167_u8,10_u8,11_u8,112_u8,253_u8];
_7.1 = 97873294075289432882834389269313265039_u128 >> RET;
_7 = (14155702991536628061_u64, 217978197993231756924662868745533152329_u128);
match _7.0 {
0 => bb4,
1 => bb5,
14155702991536628061 => bb7,
_ => bb6
}
}
bb18 = {
_16.fld3.1 = 210_u8;
Goto(bb19)
}
bb19 = {
Call(_18 = dump_var(3_usize, 6_usize, Move(_6), 4_usize, Move(_4), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: f64,mut _4: [u8; 7],mut _5: [u8; 7],mut _6: [u8; 7],mut _7: [u8; 7],mut _8: [u8; 7],mut _9: [u8; 7],mut _10: [u8; 7],mut _11: [u8; 7]) -> u16 {
mir! {
type RET = u16;
let _12: char;
let _13: Adt47;
let _14: f64;
let _15: Adt56;
let _16: bool;
let _17: Adt43;
let _18: u64;
let _19: isize;
let _20: Adt51;
let _21: u64;
let _22: f32;
let _23: (u16, i64, u16);
let _24: f64;
let _25: Adt52;
let _26: (&'static char, (u64, i32, usize));
let _27: (i16, u64, i16, u64, bool, char);
let _28: char;
let _29: Adt45;
let _30: Adt57;
let _31: u32;
let _32: [u16; 7];
let _33: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _34: u8;
let _35: u128;
let _36: isize;
let _37: *mut i64;
let _38: Adt48;
let _39: Adt47;
let _40: [u16; 7];
let _41: char;
let _42: i32;
let _43: f32;
let _44: (u16, i64, u16);
let _45: (u64, u128);
let _46: Adt55;
let _47: Adt50;
let _48: [u128; 7];
let _49: ();
let _50: ();
{
_9 = [161_u8,59_u8,235_u8,174_u8,103_u8,111_u8,34_u8];
_2 = [198_u8,13_u8,248_u8,217_u8,82_u8,10_u8,215_u8];
RET = !36031_u16;
_7 = _8;
_10 = [107_u8,155_u8,156_u8,109_u8,221_u8,5_u8,165_u8];
_1 = [14_u8,129_u8,84_u8,148_u8,248_u8,84_u8,217_u8];
_8 = [184_u8,77_u8,220_u8,209_u8,118_u8,51_u8,116_u8];
_12 = '\u{1e363}';
_8 = [175_u8,152_u8,127_u8,82_u8,33_u8,102_u8,4_u8];
_6 = [116_u8,61_u8,155_u8,154_u8,162_u8,37_u8,255_u8];
_7 = _9;
_12 = '\u{a23a5}';
Goto(bb1)
}
bb1 = {
_12 = '\u{b5940}';
_5 = [90_u8,169_u8,164_u8,166_u8,172_u8,15_u8,127_u8];
_6 = [131_u8,150_u8,20_u8,184_u8,12_u8,227_u8,42_u8];
_2 = _4;
Call(_7 = fn5(_10, _1, _11, _10, _10, _10, _8, _5, _11, _1, _6, _6, _9, _11, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [44_u8,149_u8,245_u8,116_u8,18_u8,61_u8,9_u8];
_7 = _2;
_2 = [55_u8,119_u8,3_u8,179_u8,165_u8,149_u8,152_u8];
_3 = 9223372036854775807_isize as f64;
_1 = [220_u8,26_u8,82_u8,43_u8,104_u8,98_u8,137_u8];
_4 = _1;
RET = 19379_u16 & 17824_u16;
_5 = [85_u8,123_u8,210_u8,71_u8,220_u8,240_u8,187_u8];
_2 = [29_u8,129_u8,245_u8,251_u8,166_u8,128_u8,180_u8];
RET = 44300_u16 & 43950_u16;
_15.fld1 = _12;
_5 = _1;
_1 = [124_u8,45_u8,200_u8,42_u8,24_u8,7_u8,136_u8];
_15.fld0 = 203469343361833229696589882794804914757_u128;
_16 = RET == RET;
_15.fld1 = _12;
_6 = [18_u8,222_u8,171_u8,86_u8,180_u8,80_u8,31_u8];
_17.fld3.2 = 17538878259041145880_u64 as i16;
_17.fld3.2 = 22456_i16 >> RET;
match _15.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
203469343361833229696589882794804914757 => bb6,
_ => bb5
}
}
bb3 = {
_12 = '\u{b5940}';
_5 = [90_u8,169_u8,164_u8,166_u8,172_u8,15_u8,127_u8];
_6 = [131_u8,150_u8,20_u8,184_u8,12_u8,227_u8,42_u8];
_2 = _4;
Call(_7 = fn5(_10, _1, _11, _10, _10, _10, _8, _5, _11, _1, _6, _6, _9, _11, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_17.fld1 = 246_u8 as i16;
_17.fld3.4 = _16 >= _16;
_17.fld0.1 = _17.fld3.4 as u8;
_17.fld3.4 = !_16;
_6 = [_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1];
_17.fld3 = (_17.fld1, 9384780542817540837_u64, _17.fld1, 9576655313861205838_u64, _16, _12);
_16 = _17.fld3.4;
RET = 2774379701006776484_i64 as u16;
_17.fld3.2 = (-128420002537357608219768890466760499398_i128) as i16;
_17.fld2 = [_15.fld1];
_16 = _17.fld3.4 ^ _17.fld3.4;
_14 = _3;
_11 = [_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1];
_17.fld3.0 = 1983136793_i32 as i16;
_17.fld3.4 = _16;
_14 = _3 * _3;
RET = !58555_u16;
_9 = [_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1];
_8 = _9;
_18 = 472831615672541499_i64 as u64;
_17.fld0.0 = 0_usize + 17064727855832401918_usize;
RET = 22734_u16;
_17.fld3.2 = 108401598066405392131985787147788901084_i128 as i16;
_17.fld0.0 = 2_usize;
_12 = _15.fld1;
_16 = _17.fld3.4 ^ _17.fld3.4;
Goto(bb7)
}
bb7 = {
_21 = !_17.fld3.3;
_5 = _11;
RET = !47981_u16;
_21 = _17.fld3.1 << _17.fld3.1;
_14 = _3;
_4 = _6;
_6 = _7;
_17.fld3.3 = _18;
_17.fld3.0 = !_17.fld3.2;
_12 = _17.fld3.5;
Call(_17.fld0.0 = core::intrinsics::bswap(9156713204252465613_usize), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15.fld0 = !309889370221946196567297028625828193796_u128;
Call(_11 = fn16(_9, _17.fld3.4, _8, _17.fld1, _4, _16, _17.fld3.1, _3, _10, _16, _16, _21, _8, _17.fld3.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_23.2 = RET;
_18 = _21 / _17.fld3.1;
_17.fld2 = [_12];
_16 = !_17.fld3.4;
RET = (-9223372036854775808_isize) as u16;
_22 = 3022371146_u32 as f32;
_14 = _3 * _3;
_23.0 = !_23.2;
_3 = _14;
_23 = (RET, (-7769882532212075156_i64), RET);
_25.fld2 = core::ptr::addr_of_mut!(_25.fld4.fld3.fld0.0);
_25.fld4.fld3.fld3.5 = _15.fld1;
match _23.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
340282366920938463455604724899556136300 => bb10,
_ => bb6
}
}
bb10 = {
_25.fld5 = _17.fld0.1;
_25.fld4.fld1 = [_21,_21,_18,_18,_21,_18];
_14 = -_3;
_25.fld4.fld3.fld3.2 = _17.fld1 + _17.fld3.0;
_25.fld4.fld3.fld3 = _17.fld3;
_15.fld2 = 2045772472_u32 as u8;
_25.fld4.fld3.fld3 = (_17.fld3.0, _17.fld3.1, _17.fld1, _18, _17.fld3.4, _15.fld1);
_19 = (-9223372036854775808_isize) + 85_isize;
_25.fld2 = core::ptr::addr_of_mut!(_17.fld0.0);
_17.fld3.3 = _15.fld0 as u64;
_15.fld3 = core::ptr::addr_of_mut!(_25.fld4.fld0.1);
_3 = _22 as f64;
_4 = _9;
_22 = _19 as f32;
_5 = [_17.fld0.1,_25.fld5,_15.fld2,_17.fld0.1,_17.fld0.1,_25.fld5,_17.fld0.1];
_25.fld4.fld3.fld3.0 = 3151819420_u32 as i16;
_29.fld5.0 = _25.fld4.fld3.fld3.3;
_29.fld3.0 = _17.fld0.0;
_26.1.1 = 1557646006_i32 >> _25.fld4.fld3.fld3.3;
_14 = _3;
_16 = _17.fld3.4;
_25.fld4.fld3.fld0.1 = _25.fld5 & _25.fld5;
_15.fld2 = _17.fld0.1;
_25.fld4.fld3.fld0.2 = [_23.0,_23.0];
_15.fld0 = _17.fld0.1 as u128;
_29.fld5.2 = _14 as usize;
Call(_29.fld4 = fn17(_25.fld4.fld3.fld3.3, _25.fld4.fld3.fld3.3, _17.fld1, _6, _17.fld3.1, _25.fld4.fld3.fld3, _29.fld5.0, _25.fld4.fld3.fld3.1, _19, _18, _25.fld4.fld3.fld3.4, _25.fld4.fld3.fld3.5, _26.1.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_25.fld4.fld3.fld2 = _29.fld4;
_19 = (-4_isize) >> _25.fld5;
_17.fld0 = (_29.fld5.2, _25.fld4.fld3.fld0.1, _25.fld4.fld3.fld0.2);
_29.fld0 = core::ptr::addr_of_mut!(_23.0);
_27.5 = _25.fld4.fld3.fld3.5;
_7 = [_17.fld0.1,_25.fld4.fld3.fld0.1,_25.fld4.fld3.fld0.1,_25.fld5,_17.fld0.1,_25.fld4.fld3.fld0.1,_15.fld2];
_25.fld4.fld3.fld0.0 = !_29.fld5.2;
_6 = [_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_25.fld4.fld3.fld0.1,_15.fld2,_25.fld5];
_17.fld3.0 = _17.fld3.2 | _25.fld4.fld3.fld3.0;
_27.4 = _29.fld3.0 != _29.fld5.2;
_29.fld6 = _23.1 >> _17.fld3.2;
_16 = _25.fld4.fld3.fld3.4;
_25.fld4.fld4 = !_17.fld3.1;
_10 = [_25.fld4.fld3.fld0.1,_15.fld2,_25.fld4.fld3.fld0.1,_17.fld0.1,_17.fld0.1,_25.fld5,_17.fld0.1];
_9 = _5;
RET = _25.fld4.fld3.fld3.4 as u16;
_25.fld4.fld0.2 = (-58_i8) as u16;
_26.0 = &_25.fld4.fld3.fld3.5;
_29.fld4 = [_27.5];
_25.fld4.fld3.fld3.4 = _17.fld3.4;
_17.fld3.0 = _17.fld3.2;
_27 = (_17.fld3.0, _21, _25.fld4.fld3.fld3.2, _21, _16, _15.fld1);
_24 = -_14;
_29.fld3.2 = _25.fld4.fld3.fld0.2;
_27.3 = !_18;
_25.fld4.fld0.0 = RET;
_15.fld2 = _17.fld0.1;
_29.fld1 = _15.fld1;
_6 = [_17.fld0.1,_15.fld2,_15.fld2,_17.fld0.1,_25.fld4.fld3.fld0.1,_25.fld4.fld3.fld0.1,_17.fld0.1];
Goto(bb12)
}
bb12 = {
_32 = [_25.fld4.fld0.0,_25.fld4.fld0.0,RET,RET,_25.fld4.fld0.2,_25.fld4.fld0.0,_25.fld4.fld0.2];
_25.fld4.fld3.fld3.2 = _27.2;
_17.fld3.2 = _27.0 << _25.fld5;
_17.fld3 = (_25.fld4.fld3.fld3.2, _25.fld4.fld3.fld3.3, _27.2, _25.fld4.fld3.fld3.1, _25.fld4.fld3.fld3.4, _25.fld4.fld3.fld3.5);
_15.fld2 = !_17.fld0.1;
_25.fld4.fld3.fld3.3 = _15.fld0 as u64;
_29.fld3.1 = _17.fld0.1 | _25.fld5;
_32 = [RET,_25.fld4.fld0.0,RET,_25.fld4.fld0.0,_25.fld4.fld0.0,_25.fld4.fld0.0,_25.fld4.fld0.0];
_36 = _19;
_9 = [_25.fld5,_29.fld3.1,_17.fld0.1,_15.fld2,_29.fld3.1,_29.fld3.1,_29.fld3.1];
_33.0.5 = _17.fld3.5;
_33.0.1 = _29.fld3.1 as u64;
_17.fld0.1 = (-115721805881338707557285905318930786696_i128) as u8;
_10 = [_15.fld2,_25.fld4.fld3.fld0.1,_25.fld4.fld3.fld0.1,_15.fld2,_25.fld4.fld3.fld0.1,_15.fld2,_29.fld3.1];
_37 = core::ptr::addr_of_mut!(_29.fld6);
_25.fld4.fld3.fld2 = [_17.fld3.5];
_33.4 = (*_37) as u128;
_25.fld2 = core::ptr::addr_of_mut!(_29.fld3.0);
_17.fld3 = (_25.fld4.fld3.fld3.0, _29.fld5.0, _27.0, _27.3, _16, _33.0.5);
_25.fld0 = [_19,_36,_19,_36,_36,_36];
Goto(bb13)
}
bb13 = {
_25.fld4.fld0 = (RET, (*_37), RET);
_25.fld5 = (-95_i8) as u8;
_44.1 = (*_37) >> _15.fld0;
_25.fld5 = !_29.fld3.1;
_17.fld3.0 = _25.fld4.fld3.fld3.0 | _27.2;
_27.3 = !_18;
_26.1.2 = _23.2 as usize;
_45 = (_27.3, _15.fld0);
_17.fld0.1 = _36 as u8;
_38 = Adt48::Variant1 { fld0: _17,fld1: _12,fld2: _10,fld3: _29.fld0 };
Goto(bb14)
}
bb14 = {
_37 = _15.fld3;
_14 = _24;
_25.fld1 = core::ptr::addr_of_mut!(_23.0);
_33.4 = Field::<Adt43>(Variant(_38, 1), 0).fld1 as u128;
_25.fld4.fld0.1 = _44.1 << _25.fld4.fld3.fld0.1;
_27.3 = Field::<Adt43>(Variant(_38, 1), 0).fld3.1;
_44.0 = RET;
_42 = _25.fld4.fld3.fld0.0 as i32;
_26.1 = (_25.fld4.fld4, _42, _17.fld0.0);
_33.3 = _19 as u8;
_17.fld2 = Field::<Adt43>(Variant(_38, 1), 0).fld2;
_29.fld3 = _25.fld4.fld3.fld0;
_25.fld4.fld3.fld3.4 = Field::<Adt43>(Variant(_38, 1), 0).fld3.4;
_47.fld0.1 = -(*_37);
_35 = _45.1;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(4_usize, 23_usize, Move(_23), 9_usize, Move(_9), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(4_usize, 7_usize, Move(_7), 35_usize, Move(_35), 11_usize, Move(_11), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(4_usize, 10_usize, Move(_10), 36_usize, Move(_36), 32_usize, Move(_32), 50_usize, _50), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: [u8; 7],mut _4: [u8; 7],mut _5: [u8; 7],mut _6: [u8; 7],mut _7: [u8; 7],mut _8: [u8; 7],mut _9: [u8; 7],mut _10: [u8; 7],mut _11: [u8; 7],mut _12: [u8; 7],mut _13: [u8; 7],mut _14: [u8; 7],mut _15: [u8; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _16: (i16, u64, i16, u64, bool, char);
let _17: *mut [u8; 7];
let _18: Adt56;
let _19: isize;
let _20: (u64, i32, usize);
let _21: f64;
let _22: char;
let _23: char;
let _24: u64;
let _25: (u16, i64, u16);
let _26: Adt51;
let _27: [u16; 7];
let _28: (u16,);
let _29: ();
let _30: ();
{
_6 = [180_u8,113_u8,224_u8,39_u8,105_u8,124_u8,59_u8];
_4 = [105_u8,143_u8,43_u8,152_u8,57_u8,41_u8,64_u8];
_4 = _10;
_2 = _7;
_14 = [115_u8,14_u8,53_u8,1_u8,231_u8,246_u8,118_u8];
Call(_13 = fn6(_14, _9, _1, _2, _11, _4, _7, _8, _5, _10, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_16.2 = (-21405_i16);
_12 = [34_u8,232_u8,164_u8,104_u8,185_u8,131_u8,112_u8];
RET = [173_u8,89_u8,105_u8,112_u8,219_u8,15_u8,187_u8];
_16.3 = !542948117263123903_u64;
_16.1 = !_16.3;
_8 = [46_u8,46_u8,168_u8,213_u8,99_u8,130_u8,68_u8];
_18.fld1 = '\u{92621}';
_7 = _3;
_18.fld0 = 12132988643935579567897659539357678037_u128 ^ 245960525361234790903071082654252321397_u128;
_5 = _12;
_18.fld0 = false as u128;
_13 = RET;
_16.1 = !_16.3;
_4 = _3;
_16 = ((-19123_i16), 14224845907492240803_u64, 1825_i16, 4018603362550520260_u64, true, _18.fld1);
_17 = core::ptr::addr_of_mut!(_3);
_16.2 = _18.fld0 as i16;
_18.fld1 = _16.5;
_18.fld2 = 211_u8 + 247_u8;
_13 = _10;
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_1 = _14;
Goto(bb2)
}
bb2 = {
_6 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
(*_17) = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_20.2 = 1702243048505960627_usize ^ 5_usize;
_14 = _8;
_18.fld0 = _16.4 as u128;
_4 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_9 = _12;
RET = _5;
_20.2 = _18.fld0 as usize;
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_8 = _7;
_20 = (_16.1, (-1280621510_i32), 5_usize);
_20 = (_16.1, 1961669831_i32, 7837272038457140429_usize);
_9 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
RET = _11;
_10 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
match _16.0 {
0 => bb3,
1 => bb4,
340282366920938463463374607431768192333 => bb6,
_ => bb5
}
}
bb3 = {
_16.2 = (-21405_i16);
_12 = [34_u8,232_u8,164_u8,104_u8,185_u8,131_u8,112_u8];
RET = [173_u8,89_u8,105_u8,112_u8,219_u8,15_u8,187_u8];
_16.3 = !542948117263123903_u64;
_16.1 = !_16.3;
_8 = [46_u8,46_u8,168_u8,213_u8,99_u8,130_u8,68_u8];
_18.fld1 = '\u{92621}';
_7 = _3;
_18.fld0 = 12132988643935579567897659539357678037_u128 ^ 245960525361234790903071082654252321397_u128;
_5 = _12;
_18.fld0 = false as u128;
_13 = RET;
_16.1 = !_16.3;
_4 = _3;
_16 = ((-19123_i16), 14224845907492240803_u64, 1825_i16, 4018603362550520260_u64, true, _18.fld1);
_17 = core::ptr::addr_of_mut!(_3);
_16.2 = _18.fld0 as i16;
_18.fld1 = _16.5;
_18.fld2 = 211_u8 + 247_u8;
_13 = _10;
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_1 = _14;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_4 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_20.0 = _16.3 ^ _16.1;
_20.1 = !1445409814_i32;
_5 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_16.5 = _18.fld1;
_22 = _18.fld1;
_18.fld0 = 14961519607152294065306956736516080298_u128 << _20.2;
_14 = _15;
Goto(bb7)
}
bb7 = {
_12 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_2 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_16 = (1435_i16, _20.0, 18804_i16, _20.0, false, _22);
_5 = _15;
_24 = _16.1;
match _16.2 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb8,
4 => bb9,
5 => bb10,
18804 => bb12,
_ => bb11
}
}
bb8 = {
_4 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_20.0 = _16.3 ^ _16.1;
_20.1 = !1445409814_i32;
_5 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_16.5 = _18.fld1;
_22 = _18.fld1;
_18.fld0 = 14961519607152294065306956736516080298_u128 << _20.2;
_14 = _15;
Goto(bb7)
}
bb9 = {
_16.2 = (-21405_i16);
_12 = [34_u8,232_u8,164_u8,104_u8,185_u8,131_u8,112_u8];
RET = [173_u8,89_u8,105_u8,112_u8,219_u8,15_u8,187_u8];
_16.3 = !542948117263123903_u64;
_16.1 = !_16.3;
_8 = [46_u8,46_u8,168_u8,213_u8,99_u8,130_u8,68_u8];
_18.fld1 = '\u{92621}';
_7 = _3;
_18.fld0 = 12132988643935579567897659539357678037_u128 ^ 245960525361234790903071082654252321397_u128;
_5 = _12;
_18.fld0 = false as u128;
_13 = RET;
_16.1 = !_16.3;
_4 = _3;
_16 = ((-19123_i16), 14224845907492240803_u64, 1825_i16, 4018603362550520260_u64, true, _18.fld1);
_17 = core::ptr::addr_of_mut!(_3);
_16.2 = _18.fld0 as i16;
_18.fld1 = _16.5;
_18.fld2 = 211_u8 + 247_u8;
_13 = _10;
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_1 = _14;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_6 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
(*_17) = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_20.2 = 1702243048505960627_usize ^ 5_usize;
_14 = _8;
_18.fld0 = _16.4 as u128;
_4 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_9 = _12;
RET = _5;
_20.2 = _18.fld0 as usize;
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_8 = _7;
_20 = (_16.1, (-1280621510_i32), 5_usize);
_20 = (_16.1, 1961669831_i32, 7837272038457140429_usize);
_9 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
RET = _11;
_10 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
match _16.0 {
0 => bb3,
1 => bb4,
340282366920938463463374607431768192333 => bb6,
_ => bb5
}
}
bb12 = {
_21 = _16.0 as f64;
_3 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_9 = _13;
_9 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_19 = (-9223372036854775808_isize);
match _16.2 {
0 => bb3,
1 => bb9,
2 => bb13,
18804 => bb15,
_ => bb14
}
}
bb13 = {
_12 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_2 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_16 = (1435_i16, _20.0, 18804_i16, _20.0, false, _22);
_5 = _15;
_24 = _16.1;
match _16.2 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb8,
4 => bb9,
5 => bb10,
18804 => bb12,
_ => bb11
}
}
bb14 = {
_16.2 = (-21405_i16);
_12 = [34_u8,232_u8,164_u8,104_u8,185_u8,131_u8,112_u8];
RET = [173_u8,89_u8,105_u8,112_u8,219_u8,15_u8,187_u8];
_16.3 = !542948117263123903_u64;
_16.1 = !_16.3;
_8 = [46_u8,46_u8,168_u8,213_u8,99_u8,130_u8,68_u8];
_18.fld1 = '\u{92621}';
_7 = _3;
_18.fld0 = 12132988643935579567897659539357678037_u128 ^ 245960525361234790903071082654252321397_u128;
_5 = _12;
_18.fld0 = false as u128;
_13 = RET;
_16.1 = !_16.3;
_4 = _3;
_16 = ((-19123_i16), 14224845907492240803_u64, 1825_i16, 4018603362550520260_u64, true, _18.fld1);
_17 = core::ptr::addr_of_mut!(_3);
_16.2 = _18.fld0 as i16;
_18.fld1 = _16.5;
_18.fld2 = 211_u8 + 247_u8;
_13 = _10;
_7 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_1 = _14;
Goto(bb2)
}
bb15 = {
_22 = _18.fld1;
_18.fld2 = 89_u8;
_25.1 = !(-7166032728196881323_i64);
_25.1 = (-3361033797256361646_i64) ^ 785360642022477603_i64;
_8 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_14 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
_18.fld1 = _16.5;
_13 = [_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2,_18.fld2];
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(5_usize, 20_usize, Move(_20), 13_usize, Move(_13), 1_usize, Move(_1), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(5_usize, 6_usize, Move(_6), 3_usize, Move(_3), 4_usize, Move(_4), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(5_usize, 9_usize, Move(_9), 12_usize, Move(_12), 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: [u8; 7],mut _4: [u8; 7],mut _5: [u8; 7],mut _6: [u8; 7],mut _7: [u8; 7],mut _8: [u8; 7],mut _9: [u8; 7],mut _10: [u8; 7],mut _11: [u8; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _12: char;
let _13: char;
let _14: char;
let _15: i32;
let _16: &'static char;
let _17: Adt42;
let _18: u16;
let _19: [u8; 7];
let _20: Adt44;
let _21: f32;
let _22: isize;
let _23: *mut *const f64;
let _24: bool;
let _25: usize;
let _26: u64;
let _27: *const f64;
let _28: [u128; 7];
let _29: Adt43;
let _30: i64;
let _31: isize;
let _32: Adt42;
let _33: char;
let _34: bool;
let _35: u8;
let _36: Adt57;
let _37: (i16, u64, i16, u64, bool, char);
let _38: [u64; 6];
let _39: bool;
let _40: isize;
let _41: u128;
let _42: [u64; 6];
let _43: u16;
let _44: char;
let _45: ();
let _46: ();
{
_4 = _7;
RET = _2;
_12 = '\u{54729}';
_1 = [91_u8,132_u8,201_u8,38_u8,51_u8,239_u8,252_u8];
_11 = [76_u8,231_u8,154_u8,77_u8,173_u8,153_u8,92_u8];
_10 = [248_u8,205_u8,239_u8,93_u8,13_u8,167_u8,206_u8];
_7 = _3;
_12 = '\u{462a0}';
_12 = '\u{f0b4c}';
_14 = _12;
_4 = _10;
_4 = [229_u8,179_u8,84_u8,205_u8,29_u8,6_u8,88_u8];
_15 = (-57_i8) as i32;
_3 = [246_u8,130_u8,89_u8,16_u8,137_u8,92_u8,31_u8];
_3 = _10;
_6 = [68_u8,116_u8,56_u8,201_u8,246_u8,88_u8,119_u8];
_12 = _14;
_16 = &_12;
_18 = 50314_u16 + 25274_u16;
_18 = 5114475766408979807_usize as u16;
Goto(bb1)
}
bb1 = {
_11 = [221_u8,181_u8,146_u8,5_u8,234_u8,25_u8,143_u8];
Call(_5 = fn7(_2, _7, _11, _8, _10, Move(_16), _9, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = &_12;
_19 = [41_u8,32_u8,144_u8,67_u8,182_u8,230_u8,111_u8];
_19 = [161_u8,12_u8,211_u8,28_u8,181_u8,69_u8,105_u8];
Goto(bb3)
}
bb3 = {
_15 = 1188670830_u32 as i32;
_12 = _14;
_7 = _8;
RET = _5;
_13 = _12;
_13 = _12;
_4 = RET;
_15 = 603006589_i32 + 1929772879_i32;
_19 = [226_u8,216_u8,181_u8,104_u8,149_u8,18_u8,68_u8];
_9 = [207_u8,194_u8,76_u8,40_u8,178_u8,156_u8,172_u8];
_25 = 7_usize;
_12 = _14;
_2 = RET;
_25 = 1_usize + 4198577351232318616_usize;
_26 = !7342829868808637359_u64;
_2 = [110_u8,119_u8,168_u8,113_u8,137_u8,33_u8,149_u8];
RET = [91_u8,161_u8,20_u8,129_u8,206_u8,74_u8,196_u8];
_4 = _5;
_15 = !2007056944_i32;
_10 = _11;
_3 = [179_u8,2_u8,63_u8,63_u8,108_u8,30_u8,127_u8];
_22 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_12 = _14;
Call(_21 = core::intrinsics::transmute(_15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_3 = core::intrinsics::transmute(_6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_29.fld3.0 = 6982_i16;
_29.fld2 = [_14];
_29.fld3.3 = _22 as u64;
_29.fld3 = (30181_i16, _26, (-8788_i16), _26, false, _13);
_7 = [21_u8,163_u8,186_u8,105_u8,217_u8,203_u8,17_u8];
_7 = _11;
_20 = Adt44::Variant2 { fld0: 49_i8 };
_19 = [91_u8,8_u8,75_u8,186_u8,149_u8,118_u8,26_u8];
_2 = _10;
_30 = !1256347093853825636_i64;
_28 = [145257453309840418258062109571740516341_u128,89674587131150022817147701285862788897_u128,336855994316685915201640479288534891195_u128,223453747215426499463319901062562969809_u128,16950256688050646811259322397357651730_u128,39420228738714005686504245949774987973_u128,321510137664271091072749494884699540620_u128];
_4 = [61_u8,7_u8,202_u8,191_u8,215_u8,25_u8,218_u8];
_29.fld0.0 = _25;
_20 = Adt44::Variant2 { fld0: (-115_i8) };
_25 = !_29.fld0.0;
_21 = 680779432_u32 as f32;
match _29.fld3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
30181 => bb10,
_ => bb9
}
}
bb6 = {
Call(_3 = core::intrinsics::transmute(_6), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_15 = 1188670830_u32 as i32;
_12 = _14;
_7 = _8;
RET = _5;
_13 = _12;
_13 = _12;
_4 = RET;
_15 = 603006589_i32 + 1929772879_i32;
_19 = [226_u8,216_u8,181_u8,104_u8,149_u8,18_u8,68_u8];
_9 = [207_u8,194_u8,76_u8,40_u8,178_u8,156_u8,172_u8];
_25 = 7_usize;
_12 = _14;
_2 = RET;
_25 = 1_usize + 4198577351232318616_usize;
_26 = !7342829868808637359_u64;
_2 = [110_u8,119_u8,168_u8,113_u8,137_u8,33_u8,149_u8];
RET = [91_u8,161_u8,20_u8,129_u8,206_u8,74_u8,196_u8];
_4 = _5;
_15 = !2007056944_i32;
_10 = _11;
_3 = [179_u8,2_u8,63_u8,63_u8,108_u8,30_u8,127_u8];
_22 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_12 = _14;
Call(_21 = core::intrinsics::transmute(_15), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_16 = &_12;
_19 = [41_u8,32_u8,144_u8,67_u8,182_u8,230_u8,111_u8];
_19 = [161_u8,12_u8,211_u8,28_u8,181_u8,69_u8,105_u8];
Goto(bb3)
}
bb9 = {
_11 = [221_u8,181_u8,146_u8,5_u8,234_u8,25_u8,143_u8];
Call(_5 = fn7(_2, _7, _11, _8, _10, Move(_16), _9, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_24 = _29.fld3.0 > _29.fld3.2;
_31 = _22;
_34 = _24;
_29.fld0.2 = [_18,_18];
_2 = _5;
_29.fld0.1 = 248_u8 * 110_u8;
place!(Field::<i8>(Variant(_20, 2), 0)) = 123_i8;
_14 = _29.fld3.5;
_29.fld3.1 = _26;
_2 = _19;
_18 = 52899_u16 | 3777_u16;
_16 = &_14;
_35 = _22 as u8;
_19 = [_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1];
_29.fld2 = [_29.fld3.5];
_23 = core::ptr::addr_of_mut!(_27);
_29.fld0.2 = [_18,_18];
_3 = _1;
_29.fld3.3 = _15 as u64;
RET = [_35,_35,_29.fld0.1,_35,_35,_29.fld0.1,_35];
_29.fld0.1 = _35 ^ _35;
place!(Field::<i8>(Variant(_20, 2), 0)) = 97_i8;
_10 = [_29.fld0.1,_35,_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_35];
_29.fld3.4 = _34;
_19 = [_29.fld0.1,_29.fld0.1,_35,_35,_29.fld0.1,_35,_35];
_18 = !32701_u16;
_19 = _7;
Goto(bb11)
}
bb11 = {
_29.fld0.1 = _35 & _35;
SetDiscriminant(_20, 2);
_6 = [_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1];
_7 = [_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_35,_29.fld0.1,_35];
_37.3 = 95_i8 as u64;
place!(Field::<i8>(Variant(_20, 2), 0)) = _25 as i8;
_25 = _29.fld0.0;
_22 = _29.fld3.0 as isize;
_28 = [324000236125309324271389911872143121664_u128,313344719335753434773322465821098710246_u128,195672897909074737027372950861972815377_u128,214396604743501340403840260852552674050_u128,309273221140167545608227506798860551088_u128,332749030417326449937601937322518265740_u128,300876681969409491961793070680942927159_u128];
_29.fld2 = [_14];
_12 = (*_16);
SetDiscriminant(_20, 1);
_16 = &(*_16);
_30 = (-807199209761110880_i64);
_29.fld1 = _29.fld3.2;
_33 = _12;
Goto(bb12)
}
bb12 = {
_4 = _8;
_23 = core::ptr::addr_of_mut!((*_23));
_3 = [_35,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1,_35,_29.fld0.1];
_5 = [_35,_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_29.fld0.1,_35];
_29.fld3.1 = _29.fld0.1 as u64;
_37.1 = _29.fld3.1;
_39 = _34 == _34;
Goto(bb13)
}
bb13 = {
_29.fld0.0 = !_25;
_24 = _29.fld3.4 != _39;
match _29.fld1 {
0 => bb4,
1 => bb7,
2 => bb14,
340282366920938463463374607431768202668 => bb16,
_ => bb15
}
}
bb14 = {
_24 = _29.fld3.0 > _29.fld3.2;
_31 = _22;
_34 = _24;
_29.fld0.2 = [_18,_18];
_2 = _5;
_29.fld0.1 = 248_u8 * 110_u8;
place!(Field::<i8>(Variant(_20, 2), 0)) = 123_i8;
_14 = _29.fld3.5;
_29.fld3.1 = _26;
_2 = _19;
_18 = 52899_u16 | 3777_u16;
_16 = &_14;
_35 = _22 as u8;
_19 = [_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1,_29.fld0.1];
_29.fld2 = [_29.fld3.5];
_23 = core::ptr::addr_of_mut!(_27);
_29.fld0.2 = [_18,_18];
_3 = _1;
_29.fld3.3 = _15 as u64;
RET = [_35,_35,_29.fld0.1,_35,_35,_29.fld0.1,_35];
_29.fld0.1 = _35 ^ _35;
place!(Field::<i8>(Variant(_20, 2), 0)) = 97_i8;
_10 = [_29.fld0.1,_35,_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_35];
_29.fld3.4 = _34;
_19 = [_29.fld0.1,_29.fld0.1,_35,_35,_29.fld0.1,_35,_35];
_18 = !32701_u16;
_19 = _7;
Goto(bb11)
}
bb15 = {
_11 = [221_u8,181_u8,146_u8,5_u8,234_u8,25_u8,143_u8];
Call(_5 = fn7(_2, _7, _11, _8, _10, Move(_16), _9, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_29.fld0.2 = [_18,_18];
_30 = -5935981997086795149_i64;
_29.fld3 = (_29.fld1, _26, _29.fld1, _37.3, _34, _13);
_16 = &(*_16);
_3 = [_29.fld0.1,_29.fld0.1,_35,_29.fld0.1,_29.fld0.1,_35,_29.fld0.1];
place!(Field::<[u128; 7]>(Variant(_20, 1), 3)) = [305677828877321062865965348824092414207_u128,195059866594580949573774086148180785517_u128,326516291755056934975845372476931146176_u128,168507251864253482851811727252933711024_u128,141824434745918588161526697884287213350_u128,218685133938463256984265686335664807150_u128,154729383108201861700775729053915723899_u128];
_37.2 = 20303210069437113665991287051258294210_u128 as i16;
_42 = [_29.fld3.3,_37.1,_26,_37.1,_37.1,_37.3];
place!(Field::<char>(Variant(_20, 1), 1)) = _29.fld3.5;
Goto(bb17)
}
bb17 = {
Call(_45 = dump_var(6_usize, 14_usize, Move(_14), 6_usize, Move(_6), 18_usize, Move(_18), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(6_usize, 22_usize, Move(_22), 15_usize, Move(_15), 34_usize, Move(_34), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(6_usize, 8_usize, Move(_8), 2_usize, Move(_2), 9_usize, Move(_9), 39_usize, Move(_39)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(6_usize, 12_usize, Move(_12), 7_usize, Move(_7), 46_usize, _46, 46_usize, _46), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: [u8; 7],mut _4: [u8; 7],mut _5: [u8; 7],mut _6: &'static char,mut _7: [u8; 7],mut _8: [u8; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _9: i16;
let _10: *mut u16;
let _11: char;
let _12: f32;
let _13: (i16, u64, i16, u64, bool, char);
let _14: (u16,);
let _15: f32;
let _16: char;
let _17: u128;
let _18: Adt42;
let _19: usize;
let _20: *mut usize;
let _21: u8;
let _22: char;
let _23: i8;
let _24: isize;
let _25: (u16,);
let _26: Adt53;
let _27: (i16, u64, i16, u64, bool, char);
let _28: i128;
let _29: i32;
let _30: bool;
let _31: [char; 1];
let _32: f32;
let _33: i16;
let _34: (i64, i32, f64);
let _35: ();
let _36: ();
{
_8 = [70_u8,113_u8,249_u8,14_u8,194_u8,61_u8,9_u8];
_7 = _1;
RET = _3;
_5 = [73_u8,131_u8,237_u8,184_u8,181_u8,49_u8,177_u8];
_2 = _7;
_6 = &(*_6);
_7 = _5;
_3 = [48_u8,129_u8,167_u8,237_u8,112_u8,81_u8,6_u8];
_2 = [196_u8,108_u8,50_u8,154_u8,84_u8,43_u8,65_u8];
_4 = [61_u8,32_u8,119_u8,111_u8,200_u8,133_u8,62_u8];
_6 = &(*_6);
Goto(bb1)
}
bb1 = {
_6 = &(*_6);
_9 = (-29311_i16) & 5683_i16;
_3 = [153_u8,147_u8,223_u8,184_u8,125_u8,224_u8,32_u8];
_6 = &(*_6);
_6 = &(*_6);
_5 = [18_u8,15_u8,155_u8,236_u8,95_u8,57_u8,148_u8];
_9 = (-82775367143172745073706547239870418052_i128) as i16;
RET = [52_u8,72_u8,137_u8,189_u8,156_u8,115_u8,86_u8];
_2 = [157_u8,114_u8,145_u8,109_u8,12_u8,226_u8,50_u8];
_6 = &(*_6);
_4 = [162_u8,32_u8,101_u8,245_u8,68_u8,66_u8,223_u8];
Call(_1 = fn8(_5, _8, _8, _8, RET, _8, _2, RET, _7, _7, _2, _4, (*_6), _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = &(*_6);
_5 = _8;
Goto(bb3)
}
bb3 = {
_6 = &(*_6);
RET = [138_u8,177_u8,155_u8,128_u8,164_u8,21_u8,242_u8];
_5 = [100_u8,21_u8,45_u8,174_u8,23_u8,48_u8,56_u8];
_8 = [197_u8,148_u8,245_u8,194_u8,71_u8,165_u8,94_u8];
_2 = [148_u8,108_u8,86_u8,160_u8,90_u8,31_u8,56_u8];
_8 = RET;
_7 = RET;
RET = _2;
_6 = &(*_6);
_5 = [44_u8,8_u8,248_u8,16_u8,23_u8,67_u8,253_u8];
_3 = [143_u8,115_u8,125_u8,130_u8,46_u8,231_u8,57_u8];
Goto(bb4)
}
bb4 = {
_6 = &(*_6);
_1 = [90_u8,210_u8,8_u8,213_u8,233_u8,49_u8,172_u8];
_11 = (*_6);
_11 = (*_6);
_13 = (_9, 15559362017100416526_u64, _9, 6529706281581079035_u64, true, (*_6));
_1 = [76_u8,49_u8,184_u8,238_u8,115_u8,180_u8,37_u8];
_11 = (*_6);
_13.4 = !true;
_6 = &(*_6);
_12 = 244489092788284964160266357599630796904_u128 as f32;
_5 = _1;
_14 = (11638_u16,);
_14.0 = !63925_u16;
_13.0 = _13.2 >> _13.3;
_8 = [142_u8,228_u8,245_u8,199_u8,80_u8,177_u8,74_u8];
_14.0 = 54650_u16;
_13.3 = !_13.1;
RET = [86_u8,71_u8,192_u8,55_u8,129_u8,145_u8,138_u8];
_7 = _8;
_12 = 4050770941209223775_i64 as f32;
_15 = _12 - _12;
_10 = core::ptr::addr_of_mut!(_14.0);
_6 = &_13.5;
_8 = [45_u8,22_u8,158_u8,96_u8,21_u8,47_u8,107_u8];
_14 = (35003_u16,);
_13.0 = (-59_i8) as i16;
Goto(bb5)
}
bb5 = {
_4 = [248_u8,178_u8,90_u8,171_u8,42_u8,179_u8,164_u8];
_6 = &(*_6);
_13.1 = !_13.3;
_13.1 = 1926348525_i32 as u64;
_13.1 = _13.3 ^ _13.3;
_2 = _1;
_1 = [209_u8,125_u8,236_u8,117_u8,4_u8,200_u8,46_u8];
_13 = (_9, 10838345277771935611_u64, _9, 10545679230166447017_u64, true, _11);
(*_10) = 63153_u16 ^ 949_u16;
_19 = 456535259937457678_usize;
_13 = (_9, 16486636594587338432_u64, _9, 16717343474708822895_u64, false, _11);
_13 = (_9, 15262048619462433737_u64, _9, 2404712990160464100_u64, true, _11);
_11 = _13.5;
_8 = [160_u8,24_u8,114_u8,178_u8,96_u8,70_u8,185_u8];
_6 = &_16;
_24 = 9223372036854775807_isize;
_22 = _11;
_25.0 = (*_10);
_23 = !(-68_i8);
_13.0 = _13.2;
_13.1 = !_13.3;
_17 = (*_10) as u128;
match _13.3 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
2404712990160464100 => bb10,
_ => bb9
}
}
bb6 = {
_6 = &(*_6);
_1 = [90_u8,210_u8,8_u8,213_u8,233_u8,49_u8,172_u8];
_11 = (*_6);
_11 = (*_6);
_13 = (_9, 15559362017100416526_u64, _9, 6529706281581079035_u64, true, (*_6));
_1 = [76_u8,49_u8,184_u8,238_u8,115_u8,180_u8,37_u8];
_11 = (*_6);
_13.4 = !true;
_6 = &(*_6);
_12 = 244489092788284964160266357599630796904_u128 as f32;
_5 = _1;
_14 = (11638_u16,);
_14.0 = !63925_u16;
_13.0 = _13.2 >> _13.3;
_8 = [142_u8,228_u8,245_u8,199_u8,80_u8,177_u8,74_u8];
_14.0 = 54650_u16;
_13.3 = !_13.1;
RET = [86_u8,71_u8,192_u8,55_u8,129_u8,145_u8,138_u8];
_7 = _8;
_12 = 4050770941209223775_i64 as f32;
_15 = _12 - _12;
_10 = core::ptr::addr_of_mut!(_14.0);
_6 = &_13.5;
_8 = [45_u8,22_u8,158_u8,96_u8,21_u8,47_u8,107_u8];
_14 = (35003_u16,);
_13.0 = (-59_i8) as i16;
Goto(bb5)
}
bb7 = {
_6 = &(*_6);
RET = [138_u8,177_u8,155_u8,128_u8,164_u8,21_u8,242_u8];
_5 = [100_u8,21_u8,45_u8,174_u8,23_u8,48_u8,56_u8];
_8 = [197_u8,148_u8,245_u8,194_u8,71_u8,165_u8,94_u8];
_2 = [148_u8,108_u8,86_u8,160_u8,90_u8,31_u8,56_u8];
_8 = RET;
_7 = RET;
RET = _2;
_6 = &(*_6);
_5 = [44_u8,8_u8,248_u8,16_u8,23_u8,67_u8,253_u8];
_3 = [143_u8,115_u8,125_u8,130_u8,46_u8,231_u8,57_u8];
Goto(bb4)
}
bb8 = {
_6 = &(*_6);
_5 = _8;
Goto(bb3)
}
bb9 = {
_6 = &(*_6);
_9 = (-29311_i16) & 5683_i16;
_3 = [153_u8,147_u8,223_u8,184_u8,125_u8,224_u8,32_u8];
_6 = &(*_6);
_6 = &(*_6);
_5 = [18_u8,15_u8,155_u8,236_u8,95_u8,57_u8,148_u8];
_9 = (-82775367143172745073706547239870418052_i128) as i16;
RET = [52_u8,72_u8,137_u8,189_u8,156_u8,115_u8,86_u8];
_2 = [157_u8,114_u8,145_u8,109_u8,12_u8,226_u8,50_u8];
_6 = &(*_6);
_4 = [162_u8,32_u8,101_u8,245_u8,68_u8,66_u8,223_u8];
Call(_1 = fn8(_5, _8, _8, _8, RET, _8, _2, RET, _7, _7, _2, _4, (*_6), _7), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_10 = core::ptr::addr_of_mut!(_25.0);
_16 = _13.5;
_5 = [164_u8,61_u8,170_u8,132_u8,188_u8,218_u8,40_u8];
_25 = (_14.0,);
Call(_3 = fn14(_13.5, _13, _4, _13, _16, RET, _13.3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7 = _2;
_27.4 = _13.4;
_13.1 = !_13.3;
_21 = 32_u8;
_25.0 = !_14.0;
_23 = (-5137044780589648611_i64) as i8;
_20 = core::ptr::addr_of_mut!(_19);
_28 = (-41551602058195008331615153474638880479_i128);
_9 = _13.0 - _13.0;
_3 = _7;
match (*_20) {
0 => bb8,
1 => bb12,
456535259937457678 => bb14,
_ => bb13
}
}
bb12 = {
_10 = core::ptr::addr_of_mut!(_25.0);
_16 = _13.5;
_5 = [164_u8,61_u8,170_u8,132_u8,188_u8,218_u8,40_u8];
_25 = (_14.0,);
Call(_3 = fn14(_13.5, _13, _4, _13, _16, RET, _13.3), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_6 = &(*_6);
_5 = _8;
Goto(bb3)
}
bb14 = {
_4 = _5;
_23 = 98_i8;
_31 = [_11];
_14.0 = (*_10) - (*_10);
_27.4 = _13.4;
_6 = &_16;
_25 = (_14.0,);
_9 = _23 as i16;
_13.5 = _16;
(*_10) = !_14.0;
_28 = _24 as i128;
_30 = _27.4;
(*_20) = 4_usize - 6_usize;
_14.0 = (*_10);
_13.1 = _28 as u64;
_13.4 = _30;
_13 = (_9, 1945258626035885042_u64, _9, 15711636970714783967_u64, _27.4, _11);
_7 = [_21,_21,_21,_21,_21,_21,_21];
_31 = [_13.5];
_25.0 = _28 as u16;
_13.3 = _13.1 & _13.1;
_29 = (-661763957_i32) & 384440863_i32;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(7_usize, 4_usize, Move(_4), 30_usize, Move(_30), 16_usize, Move(_16), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(7_usize, 23_usize, Move(_23), 28_usize, Move(_28), 22_usize, Move(_22), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(7_usize, 29_usize, Move(_29), 8_usize, Move(_8), 3_usize, Move(_3), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: [u8; 7],mut _4: [u8; 7],mut _5: [u8; 7],mut _6: [u8; 7],mut _7: [u8; 7],mut _8: [u8; 7],mut _9: [u8; 7],mut _10: [u8; 7],mut _11: [u8; 7],mut _12: [u8; 7],mut _13: char,mut _14: [u8; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _15: [u64; 6];
let _16: *mut i64;
let _17: [isize; 6];
let _18: &'static char;
let _19: *mut u16;
let _20: u32;
let _21: (usize, u8, [u16; 2]);
let _22: bool;
let _23: f64;
let _24: Adt53;
let _25: i16;
let _26: Adt49;
let _27: &'static char;
let _28: i8;
let _29: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _30: Adt50;
let _31: bool;
let _32: Adt43;
let _33: [char; 1];
let _34: f32;
let _35: usize;
let _36: isize;
let _37: u16;
let _38: char;
let _39: bool;
let _40: (usize, u8, [u16; 2]);
let _41: bool;
let _42: [char; 1];
let _43: [u16; 2];
let _44: Adt47;
let _45: [u64; 6];
let _46: u64;
let _47: char;
let _48: bool;
let _49: [isize; 6];
let _50: char;
let _51: ();
let _52: ();
{
RET = [113_u8,227_u8,31_u8,114_u8,191_u8,52_u8,84_u8];
_2 = [64_u8,184_u8,164_u8,156_u8,166_u8,98_u8,228_u8];
_15 = [3379804868640082595_u64,1965627053145457435_u64,6958929065698658177_u64,5925901609374201317_u64,16081729011857619927_u64,8229411240059620158_u64];
RET = [143_u8,130_u8,193_u8,9_u8,231_u8,183_u8,241_u8];
_14 = [20_u8,67_u8,76_u8,123_u8,114_u8,195_u8,161_u8];
RET = [89_u8,118_u8,124_u8,32_u8,160_u8,113_u8,164_u8];
_5 = [43_u8,11_u8,124_u8,135_u8,193_u8,222_u8,184_u8];
_8 = [93_u8,162_u8,98_u8,129_u8,42_u8,227_u8,187_u8];
_3 = _12;
_11 = [146_u8,54_u8,12_u8,45_u8,139_u8,168_u8,118_u8];
_15 = [13931218390090605291_u64,2757108603629344892_u64,2239320924858167770_u64,6598293307501605084_u64,11773337114269507871_u64,5305190449603861605_u64];
RET = [218_u8,4_u8,94_u8,18_u8,190_u8,225_u8,139_u8];
_4 = [227_u8,236_u8,165_u8,125_u8,85_u8,142_u8,183_u8];
_11 = [78_u8,83_u8,71_u8,91_u8,178_u8,205_u8,168_u8];
_10 = _8;
_17 = [(-9223372036854775808_isize),9223372036854775807_isize,(-122_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = _7;
_14 = [75_u8,164_u8,169_u8,125_u8,128_u8,121_u8,61_u8];
RET = _11;
Call(_6 = fn9(_2, _12, _4, _17, _9, _12, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_20 = 2338551692_u32 << (-5813_i16);
_15 = [17825395236135788664_u64,15291292295768188706_u64,97516229681638191_u64,12445790249403540323_u64,15578057888348111211_u64,10681244193042735270_u64];
_6 = [98_u8,198_u8,166_u8,80_u8,97_u8,96_u8,229_u8];
_6 = [1_u8,123_u8,23_u8,136_u8,213_u8,40_u8,226_u8];
_11 = _4;
_21.2 = [59327_u16,8521_u16];
_21.1 = 154_u8;
_5 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_8 = _10;
_3 = _8;
_17 = [(-9223372036854775808_isize),(-9223372036854775808_isize),84_isize,11_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_21.1 = 81_u8 * 206_u8;
_8 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_7 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_11 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
Goto(bb2)
}
bb2 = {
_21.2 = [61980_u16,61126_u16];
RET = _14;
_21.1 = 2249014310037688770_u64 as u8;
_23 = 7793953831287316577959971334657204779_u128 as f64;
_14 = RET;
_22 = false;
_23 = 31649_i16 as f64;
Goto(bb3)
}
bb3 = {
_21.1 = !212_u8;
_21.0 = 11156362775076550818_usize - 14068892657616694449_usize;
_21.2 = [61851_u16,58426_u16];
_25 = (-26819_i16);
_20 = 210074453_u32;
_20 = 3718006332_u32;
_2 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_18 = &_13;
_13 = '\u{35f85}';
_9 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_18 = &_13;
_20 = _21.1 as u32;
match _25 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768184637 => bb8,
_ => bb7
}
}
bb4 = {
_21.2 = [61980_u16,61126_u16];
RET = _14;
_21.1 = 2249014310037688770_u64 as u8;
_23 = 7793953831287316577959971334657204779_u128 as f64;
_14 = RET;
_22 = false;
_23 = 31649_i16 as f64;
Goto(bb3)
}
bb5 = {
_20 = 2338551692_u32 << (-5813_i16);
_15 = [17825395236135788664_u64,15291292295768188706_u64,97516229681638191_u64,12445790249403540323_u64,15578057888348111211_u64,10681244193042735270_u64];
_6 = [98_u8,198_u8,166_u8,80_u8,97_u8,96_u8,229_u8];
_6 = [1_u8,123_u8,23_u8,136_u8,213_u8,40_u8,226_u8];
_11 = _4;
_21.2 = [59327_u16,8521_u16];
_21.1 = 154_u8;
_5 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_8 = _10;
_3 = _8;
_17 = [(-9223372036854775808_isize),(-9223372036854775808_isize),84_isize,11_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_21.1 = 81_u8 * 206_u8;
_8 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_7 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_11 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_8 = _1;
_21.1 = 132_u8 & 102_u8;
_23 = (-729895393_i32) as f64;
_13 = '\u{3053}';
_13 = '\u{2ceff}';
_18 = &_13;
RET = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
RET = _12;
_5 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_6 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_1 = _3;
_14 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
Goto(bb9)
}
bb9 = {
_9 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_3 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_7 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_21.1 = 19_isize as u8;
_27 = &(*_18);
_3 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_5 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_28 = 36_i8 << _20;
Goto(bb10)
}
bb10 = {
_29.0.4 = _22;
_27 = Move(_18);
_21.0 = !11674146212261374102_usize;
_7 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_28 = (-37_i8);
_30.fld0.1 = _28 as i64;
Goto(bb11)
}
bb11 = {
_29.0.0 = _25 & _25;
_29.3 = _21.1;
_30.fld1 = [9395165224734434511_u64,12339314774820781153_u64,8013451760344447197_u64,15546256622603025367_u64,10511152265023126815_u64,9895498894297968547_u64];
_21.2 = [41816_u16,18242_u16];
RET = [_21.1,_21.1,_21.1,_29.3,_29.3,_21.1,_21.1];
_29.0 = (_25, 630116662113340042_u64, _25, 14029456484705552523_u64, _22, (*_27));
_29.0.2 = _25 ^ _29.0.0;
_29.0 = (_25, 11407402171560732184_u64, _25, 13069420492817037090_u64, _22, (*_27));
_32.fld3.2 = _25;
_30.fld3.fld0.1 = _21.1;
_30.fld3.fld3.0 = _29.0.2;
_30.fld4 = _32.fld3.2 as u64;
_32.fld0.2 = [5930_u16,1153_u16];
_29.4 = 76825303315368201379569451196572794863_u128;
_31 = _22;
_30.fld3.fld3.2 = _20 as i16;
Goto(bb12)
}
bb12 = {
_30.fld3.fld3.4 = _31;
_32.fld3.0 = _32.fld3.2;
_29.0.1 = _30.fld4 + _29.0.3;
_30.fld0.2 = 21158_u16;
_32.fld3.3 = _29.0.1;
_11 = [_30.fld3.fld0.1,_30.fld3.fld0.1,_21.1,_30.fld3.fld0.1,_30.fld3.fld0.1,_30.fld3.fld0.1,_21.1];
_19 = core::ptr::addr_of_mut!(_30.fld0.0);
_9 = [_29.3,_29.3,_21.1,_29.3,_21.1,_21.1,_30.fld3.fld0.1];
_29.0.4 = _30.fld3.fld3.0 == _29.0.0;
_30.fld3.fld3.4 = !_31;
_32.fld3 = (_25, _29.0.1, _25, _29.0.3, _29.0.4, (*_27));
_30.fld3.fld1 = _20 as i16;
_29.0.1 = !_29.0.3;
_16 = core::ptr::addr_of_mut!(_30.fld0.1);
_2 = [_29.3,_21.1,_21.1,_21.1,_30.fld3.fld0.1,_21.1,_21.1];
_12 = _10;
_30.fld0 = (43298_u16, (-4570277334964072689_i64), 50292_u16);
match _30.fld0.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463458804330096804138767 => bb13,
_ => bb9
}
}
bb13 = {
_30.fld3.fld1 = !_30.fld3.fld3.0;
_40 = _21;
_30.fld3.fld3.2 = _30.fld3.fld3.0 ^ _30.fld3.fld3.0;
_29.0.2 = _30.fld3.fld3.2;
_39 = _31;
_41 = _29.0.3 >= _29.0.1;
_30.fld3.fld3.3 = !_29.0.3;
_33 = [(*_27)];
_18 = &_13;
_41 = !_39;
_30.fld3.fld0.0 = _30.fld3.fld3.3 as usize;
_29.2 = !_20;
_32.fld0.0 = _40.0 & _30.fld3.fld0.0;
_8 = _12;
_25 = _30.fld3.fld3.0;
_30.fld3.fld0 = (_32.fld0.0, _21.1, _21.2);
_21.2 = [(*_19),(*_19)];
Call(_32.fld3.0 = core::intrinsics::transmute(_29.0.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_35 = _32.fld0.0 - _40.0;
_30.fld3.fld1 = -_30.fld3.fld3.2;
_27 = &(*_27);
_32.fld2 = _33;
_34 = _28 as f32;
_29.0.3 = _30.fld4 * _32.fld3.1;
_30.fld3.fld3.1 = _29.0.3;
_29.0.3 = _30.fld3.fld3.3;
_29.3 = !_40.1;
_25 = _29.0.2;
_8 = _12;
_29.0.1 = _32.fld3.3;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(8_usize, 12_usize, Move(_12), 1_usize, Move(_1), 10_usize, Move(_10), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(8_usize, 9_usize, Move(_9), 25_usize, Move(_25), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(8_usize, 17_usize, Move(_17), 33_usize, Move(_33), 40_usize, Move(_40), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(8_usize, 15_usize, Move(_15), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: [u8; 7],mut _4: [isize; 6],mut _5: [u8; 7],mut _6: [u8; 7],mut _7: [u8; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _8: i64;
let _9: isize;
let _10: [u128; 7];
let _11: bool;
let _12: isize;
let _13: f64;
let _14: f32;
let _15: char;
let _16: u64;
let _17: bool;
let _18: Adt41;
let _19: *const f64;
let _20: u16;
let _21: f32;
let _22: Adt49;
let _23: (usize, u8, [u16; 2]);
let _24: [u64; 6];
let _25: Adt43;
let _26: *mut *const f64;
let _27: Adt48;
let _28: *mut usize;
let _29: (u16, i64, u16);
let _30: bool;
let _31: f32;
let _32: Adt48;
let _33: char;
let _34: Adt56;
let _35: f32;
let _36: Adt51;
let _37: i16;
let _38: f32;
let _39: Adt55;
let _40: isize;
let _41: i16;
let _42: char;
let _43: ();
let _44: ();
{
_5 = _3;
_2 = _5;
RET = [12_u8,30_u8,249_u8,220_u8,5_u8,206_u8,109_u8];
RET = [122_u8,85_u8,252_u8,182_u8,126_u8,155_u8,174_u8];
_6 = [66_u8,137_u8,204_u8,135_u8,177_u8,121_u8,242_u8];
_8 = (-8109800707765187144_i64);
_6 = [103_u8,253_u8,197_u8,37_u8,206_u8,204_u8,142_u8];
_1 = [90_u8,254_u8,122_u8,1_u8,165_u8,141_u8,179_u8];
_8 = (-493110989056984270_i64) >> 253631129941529258680673241320736970052_u128;
_4 = [(-117_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-31_isize),41_isize];
_2 = [110_u8,115_u8,192_u8,127_u8,137_u8,85_u8,44_u8];
RET = _5;
_3 = RET;
RET = [14_u8,75_u8,209_u8,21_u8,189_u8,182_u8,96_u8];
RET = [98_u8,180_u8,40_u8,36_u8,176_u8,81_u8,98_u8];
_1 = _6;
_7 = [67_u8,238_u8,44_u8,224_u8,122_u8,216_u8,179_u8];
RET = [226_u8,191_u8,122_u8,248_u8,185_u8,171_u8,165_u8];
_5 = _2;
RET = [53_u8,152_u8,50_u8,199_u8,202_u8,129_u8,198_u8];
RET = [100_u8,154_u8,57_u8,18_u8,125_u8,4_u8,160_u8];
_9 = (-86_isize) * (-9223372036854775808_isize);
_8 = (-53970423436698218_i64) | (-6378668096156505368_i64);
_8 = !(-6491965154434901591_i64);
RET = [108_u8,106_u8,134_u8,102_u8,165_u8,165_u8,103_u8];
RET = [73_u8,108_u8,69_u8,235_u8,248_u8,252_u8,63_u8];
Call(_1 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = (-625480662922497104_i64);
_2 = [70_u8,40_u8,22_u8,198_u8,125_u8,170_u8,125_u8];
_1 = [211_u8,230_u8,112_u8,35_u8,184_u8,247_u8,47_u8];
Goto(bb2)
}
bb2 = {
_5 = [11_u8,49_u8,87_u8,72_u8,139_u8,147_u8,63_u8];
_9 = 12535971168770595247_usize as isize;
RET = [60_u8,1_u8,240_u8,212_u8,92_u8,128_u8,21_u8];
_12 = _8 as isize;
_5 = [247_u8,243_u8,43_u8,123_u8,77_u8,120_u8,162_u8];
_11 = false | true;
_13 = _12 as f64;
_3 = [138_u8,235_u8,75_u8,171_u8,178_u8,190_u8,163_u8];
_8 = 1908800274_u32 as i64;
_9 = 10500202277331162368_u64 as isize;
_9 = _13 as isize;
_13 = 2029725005_i32 as f64;
_11 = !true;
_14 = _8 as f32;
_1 = _3;
Goto(bb3)
}
bb3 = {
_4 = [_9,_9,_12,_9,_9,_12];
_10 = [281170454515366517814001519102418539775_u128,86921059945531840204317857390842412703_u128,57843968044431778809984644376708346935_u128,115164875572662886528603046092126269813_u128,161842592752641666246801927346148796383_u128,128371744472680763164658120488542168521_u128,135899444994160981881874927167179580810_u128];
_2 = [181_u8,254_u8,182_u8,45_u8,19_u8,84_u8,181_u8];
_11 = false ^ false;
_5 = [176_u8,247_u8,158_u8,81_u8,127_u8,178_u8,217_u8];
_7 = [225_u8,212_u8,156_u8,88_u8,89_u8,69_u8,200_u8];
_6 = [26_u8,20_u8,222_u8,233_u8,54_u8,249_u8,41_u8];
_5 = [218_u8,82_u8,157_u8,74_u8,251_u8,54_u8,231_u8];
_7 = [90_u8,67_u8,165_u8,170_u8,170_u8,75_u8,212_u8];
_11 = !false;
_11 = true;
Goto(bb4)
}
bb4 = {
_3 = [29_u8,255_u8,6_u8,194_u8,225_u8,18_u8,151_u8];
_2 = _1;
_4 = [_12,_9,_9,_9,_12,_12];
_4 = [_12,_12,_12,_9,_12,_9];
_6 = [246_u8,105_u8,57_u8,143_u8,55_u8,184_u8,73_u8];
_15 = '\u{1818a}';
_1 = [246_u8,192_u8,230_u8,60_u8,10_u8,16_u8,238_u8];
_5 = _1;
_10 = [298719907616412106130433847246621189647_u128,172978883367237551668493328841356620535_u128,91520639330100387988080193873787707327_u128,212424883016500462676934579383748846254_u128,36900371225447899648193406889915147926_u128,123790472788415698248420188625711966879_u128,237897958318839642391674458392557273436_u128];
_2 = [2_u8,81_u8,19_u8,254_u8,196_u8,117_u8,17_u8];
_4 = [_9,_12,_12,_12,_9,_12];
_16 = !7979591711074803216_u64;
_2 = [9_u8,193_u8,235_u8,216_u8,86_u8,51_u8,180_u8];
_11 = false;
_11 = _14 < _14;
RET = _6;
_17 = _11;
_16 = !7435085276176148809_u64;
_16 = 1909562058404014372_u64 ^ 12319580694043668543_u64;
_15 = '\u{3adf3}';
_3 = _5;
_5 = _2;
_4 = [_12,_9,_12,_12,_9,_9];
Goto(bb5)
}
bb5 = {
_15 = '\u{941a2}';
_20 = !9663_u16;
_5 = _1;
_10 = [39558470288539224725975279099130372714_u128,253333554507547843269217942499570136851_u128,216583916654291702755719131454651967679_u128,231781721255310306386812587530388783172_u128,263544122747257403169433458670187054175_u128,4152745747052414986588542237274333311_u128,176689477418381921161670179740649389605_u128];
_8 = 6516223979560929398_i64;
_1 = _5;
_14 = 3632989978_u32 as f32;
_9 = !_12;
_13 = _14 as f64;
_16 = 7609257191092374642_u64;
_8 = 1202185970398279995_i64;
_13 = 33_u8 as f64;
_14 = _16 as f32;
_13 = _8 as f64;
RET = [199_u8,37_u8,168_u8,77_u8,132_u8,103_u8,189_u8];
_21 = _14;
_8 = !(-3625828856707198366_i64);
_20 = 63181_u16 | 45105_u16;
_8 = _20 as i64;
_14 = -_21;
_6 = [114_u8,200_u8,71_u8,189_u8,117_u8,229_u8,142_u8];
_20 = !18659_u16;
_8 = 4919863123685443288_i64;
match _8 {
0 => bb4,
4919863123685443288 => bb7,
_ => bb6
}
}
bb6 = {
_3 = [29_u8,255_u8,6_u8,194_u8,225_u8,18_u8,151_u8];
_2 = _1;
_4 = [_12,_9,_9,_9,_12,_12];
_4 = [_12,_12,_12,_9,_12,_9];
_6 = [246_u8,105_u8,57_u8,143_u8,55_u8,184_u8,73_u8];
_15 = '\u{1818a}';
_1 = [246_u8,192_u8,230_u8,60_u8,10_u8,16_u8,238_u8];
_5 = _1;
_10 = [298719907616412106130433847246621189647_u128,172978883367237551668493328841356620535_u128,91520639330100387988080193873787707327_u128,212424883016500462676934579383748846254_u128,36900371225447899648193406889915147926_u128,123790472788415698248420188625711966879_u128,237897958318839642391674458392557273436_u128];
_2 = [2_u8,81_u8,19_u8,254_u8,196_u8,117_u8,17_u8];
_4 = [_9,_12,_12,_12,_9,_12];
_16 = !7979591711074803216_u64;
_2 = [9_u8,193_u8,235_u8,216_u8,86_u8,51_u8,180_u8];
_11 = false;
_11 = _14 < _14;
RET = _6;
_17 = _11;
_16 = !7435085276176148809_u64;
_16 = 1909562058404014372_u64 ^ 12319580694043668543_u64;
_15 = '\u{3adf3}';
_3 = _5;
_5 = _2;
_4 = [_12,_9,_12,_12,_9,_9];
Goto(bb5)
}
bb7 = {
_19 = core::ptr::addr_of!(_13);
_20 = !34821_u16;
_14 = -_21;
_9 = (-20532_i16) as isize;
_3 = [41_u8,240_u8,13_u8,159_u8,4_u8,218_u8,216_u8];
RET = [74_u8,212_u8,233_u8,222_u8,51_u8,38_u8,230_u8];
_3 = [127_u8,190_u8,171_u8,144_u8,236_u8,33_u8,75_u8];
_12 = _9 ^ _9;
_2 = [61_u8,181_u8,176_u8,193_u8,160_u8,228_u8,13_u8];
_17 = _11 | _11;
_11 = _13 >= _13;
_2 = [81_u8,229_u8,8_u8,206_u8,174_u8,109_u8,150_u8];
Goto(bb8)
}
bb8 = {
_23.0 = 10049298879160051700_usize;
Goto(bb9)
}
bb9 = {
_16 = 10327227000424813738_u64;
_24 = [_16,_16,_16,_16,_16,_16];
_25.fld0.1 = !177_u8;
_25.fld1 = -(-14021_i16);
_3 = RET;
_7 = [_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1];
RET = [_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1];
_25.fld3.3 = _16;
_19 = core::ptr::addr_of!(_13);
_25.fld3.0 = _25.fld1;
Goto(bb10)
}
bb10 = {
_25.fld3.5 = _15;
_25.fld3.3 = _23.0 as u64;
_12 = _16 as isize;
_1 = _2;
_26 = core::ptr::addr_of_mut!(_19);
_21 = _14 * _14;
_25.fld3.1 = _25.fld3.3 ^ _25.fld3.3;
_23.2 = [_20,_20];
_25.fld0 = (_23.0, 2_u8, _23.2);
_29.2 = _25.fld3.0 as u16;
_19 = core::ptr::addr_of!((*_19));
_29.0 = _14 as u16;
_29.1 = _8;
_25.fld3.5 = _15;
_25.fld0.1 = 113_u8;
_25.fld3.5 = _15;
Call(_2 = fn10((*_26), _15, (*_26), _5, _3, _7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = _25.fld3.1 ^ _25.fld3.3;
_30 = !_17;
_25.fld0.2 = [_29.0,_20];
_28 = core::ptr::addr_of_mut!(_23.0);
_25.fld3.1 = _25.fld3.3;
_25.fld3.2 = !_25.fld3.0;
_25.fld3.3 = !_25.fld3.1;
_19 = core::ptr::addr_of!(_13);
_23 = _25.fld0;
_25.fld3.1 = !_16;
_33 = _25.fld3.5;
_25.fld2 = [_25.fld3.5];
_34.fld2 = !_23.1;
_29 = (_20, _8, _20);
_19 = core::ptr::addr_of!((*_19));
_1 = _6;
_7 = [_34.fld2,_23.1,_23.1,_23.1,_34.fld2,_23.1,_25.fld0.1];
_30 = !_17;
(*_26) = core::ptr::addr_of!(_13);
_2 = _1;
_25.fld3.1 = (*_19) as u64;
match _23.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb5,
10049298879160051700 => bb13,
_ => bb12
}
}
bb12 = {
_3 = [29_u8,255_u8,6_u8,194_u8,225_u8,18_u8,151_u8];
_2 = _1;
_4 = [_12,_9,_9,_9,_12,_12];
_4 = [_12,_12,_12,_9,_12,_9];
_6 = [246_u8,105_u8,57_u8,143_u8,55_u8,184_u8,73_u8];
_15 = '\u{1818a}';
_1 = [246_u8,192_u8,230_u8,60_u8,10_u8,16_u8,238_u8];
_5 = _1;
_10 = [298719907616412106130433847246621189647_u128,172978883367237551668493328841356620535_u128,91520639330100387988080193873787707327_u128,212424883016500462676934579383748846254_u128,36900371225447899648193406889915147926_u128,123790472788415698248420188625711966879_u128,237897958318839642391674458392557273436_u128];
_2 = [2_u8,81_u8,19_u8,254_u8,196_u8,117_u8,17_u8];
_4 = [_9,_12,_12,_12,_9,_12];
_16 = !7979591711074803216_u64;
_2 = [9_u8,193_u8,235_u8,216_u8,86_u8,51_u8,180_u8];
_11 = false;
_11 = _14 < _14;
RET = _6;
_17 = _11;
_16 = !7435085276176148809_u64;
_16 = 1909562058404014372_u64 ^ 12319580694043668543_u64;
_15 = '\u{3adf3}';
_3 = _5;
_5 = _2;
_4 = [_12,_9,_12,_12,_9,_9];
Goto(bb5)
}
bb13 = {
_34.fld1 = _25.fld3.5;
_23.0 = _25.fld0.0;
_25.fld0.2 = [_20,_29.2];
_24 = [_25.fld3.3,_25.fld3.1,_25.fld3.3,_25.fld3.1,_25.fld3.3,_16];
_25.fld2 = [_33];
_5 = [_25.fld0.1,_23.1,_23.1,_25.fld0.1,_23.1,_23.1,_34.fld2];
_25.fld2 = [_25.fld3.5];
_2 = [_34.fld2,_23.1,_25.fld0.1,_25.fld0.1,_34.fld2,_34.fld2,_23.1];
_4 = [_9,_12,_12,_9,_12,_9];
_34.fld0 = 21064199469270177010980260759021298807_u128;
_10 = [_34.fld0,_34.fld0,_34.fld0,_34.fld0,_34.fld0,_34.fld0,_34.fld0];
_25.fld3.0 = !_25.fld3.2;
_20 = _29.0;
Goto(bb14)
}
bb14 = {
_14 = -_21;
_25.fld3.3 = _12 as u64;
_29.1 = !_8;
_34.fld3 = core::ptr::addr_of_mut!(_29.1);
RET = _6;
RET = [_23.1,_34.fld2,_23.1,_25.fld0.1,_34.fld2,_34.fld2,_23.1];
_15 = _25.fld3.5;
_32 = Adt48::Variant3 { fld0: _6 };
_25.fld3.5 = _34.fld1;
_11 = !_30;
(*_19) = _34.fld0 as f64;
_23.2 = [_20,_29.2];
_7 = Field::<[u8; 7]>(Variant(_32, 3), 0);
_25.fld1 = _25.fld3.2 << _29.0;
_17 = !_30;
_25.fld3.3 = _16;
(*_26) = core::ptr::addr_of!((*_19));
_24 = [_16,_25.fld3.3,_25.fld3.3,_25.fld3.1,_16,_25.fld3.3];
(*_26) = core::ptr::addr_of!((*_19));
_5 = Field::<[u8; 7]>(Variant(_32, 3), 0);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(9_usize, 7_usize, Move(_7), 5_usize, Move(_5), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(9_usize, 11_usize, Move(_11), 3_usize, Move(_3), 10_usize, Move(_10), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(9_usize, 16_usize, Move(_16), 15_usize, Move(_15), 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *const f64,mut _2: char,mut _3: *const f64,mut _4: [u8; 7],mut _5: [u8; 7],mut _6: [u8; 7]) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _7: (u64, i32, usize);
let _8: Adt51;
let _9: f32;
let _10: u128;
let _11: char;
let _12: u32;
let _13: isize;
let _14: isize;
let _15: Adt54;
let _16: char;
let _17: u128;
let _18: u32;
let _19: i16;
let _20: isize;
let _21: bool;
let _22: [u16; 2];
let _23: *mut i64;
let _24: &'static char;
let _25: (u64, i32, usize);
let _26: [u64; 6];
let _27: f64;
let _28: Adt50;
let _29: u8;
let _30: ();
let _31: ();
{
_7.0 = 5412593502528218081_u64 ^ 9344381258177336107_u64;
_7 = (13912717941004510406_u64, 629822246_i32, 5_usize);
(*_1) = (-1280713336478752278_i64) as f64;
_6 = _4;
_4 = _5;
(*_3) = _7.2 as f64;
_2 = '\u{b24e0}';
_7.1 = 2118563498_u32 as i32;
Call(RET = fn11(_6, (*_1), (*_1), _7, _7.0, (*_1), (*_3), _7.2, _5, _6, _1, _4, _4, _5, _7.2, (*_1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = '\u{bea22}';
_3 = core::ptr::addr_of!((*_3));
RET = [141_u8,43_u8,72_u8,132_u8,42_u8,166_u8,221_u8];
_7.0 = 143_u8 as u64;
_4 = [246_u8,162_u8,122_u8,123_u8,83_u8,130_u8,68_u8];
_2 = '\u{b514a}';
RET = [124_u8,145_u8,13_u8,86_u8,237_u8,144_u8,160_u8];
Goto(bb2)
}
bb2 = {
(*_1) = _7.2 as f64;
RET = [145_u8,156_u8,230_u8,90_u8,120_u8,58_u8,237_u8];
_7.0 = 5091600809820910679_u64;
(*_1) = 92717099674576184585176704304715511151_i128 as f64;
_4 = [151_u8,51_u8,89_u8,11_u8,18_u8,46_u8,176_u8];
_2 = '\u{cb477}';
(*_1) = _7.0 as f64;
Goto(bb3)
}
bb3 = {
RET = _5;
_3 = core::ptr::addr_of!((*_1));
(*_3) = 2089266385_u32 as f64;
_2 = '\u{7bd51}';
_13 = (-9223372036854775808_isize);
match _13 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb4 = {
(*_1) = _7.2 as f64;
RET = [145_u8,156_u8,230_u8,90_u8,120_u8,58_u8,237_u8];
_7.0 = 5091600809820910679_u64;
(*_1) = 92717099674576184585176704304715511151_i128 as f64;
_4 = [151_u8,51_u8,89_u8,11_u8,18_u8,46_u8,176_u8];
_2 = '\u{cb477}';
(*_1) = _7.0 as f64;
Goto(bb3)
}
bb5 = {
_2 = '\u{bea22}';
_3 = core::ptr::addr_of!((*_3));
RET = [141_u8,43_u8,72_u8,132_u8,42_u8,166_u8,221_u8];
_7.0 = 143_u8 as u64;
_4 = [246_u8,162_u8,122_u8,123_u8,83_u8,130_u8,68_u8];
_2 = '\u{b514a}';
RET = [124_u8,145_u8,13_u8,86_u8,237_u8,144_u8,160_u8];
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
_5 = RET;
RET = [40_u8,196_u8,66_u8,206_u8,180_u8,252_u8,137_u8];
_7.2 = !6_usize;
_10 = 150444522738645651014710996305078708388_u128 >> _7.0;
_3 = core::ptr::addr_of!((*_3));
_11 = _2;
_14 = _13;
_7.2 = 3_usize & 4_usize;
Goto(bb12)
}
bb12 = {
_7.0 = 9448303102645808004_u64 & 16445851306131491886_u64;
_5 = _6;
(*_1) = _7.0 as f64;
_13 = _14 << _7.1;
_7.1 = -248145952_i32;
_9 = (*_1) as f32;
_7.0 = !15087262470458693504_u64;
_7.2 = 14680920556448091723_usize;
_17 = !_10;
_7.2 = 8777754013598945722_usize & 2511300723198710624_usize;
_18 = !2371692839_u32;
_6 = _5;
_16 = _11;
_22 = [33215_u16,28418_u16];
_19 = 28396_i16;
_19 = !(-23244_i16);
_7.0 = 2961231043237733754_u64;
_24 = &_16;
RET = [32_u8,180_u8,191_u8,133_u8,253_u8,5_u8,216_u8];
_13 = _19 as isize;
Goto(bb13)
}
bb13 = {
_3 = core::ptr::addr_of!((*_1));
_2 = (*_24);
_25 = _7;
_8 = Adt51::Variant0 { fld0: _1,fld1: _9 };
_11 = (*_24);
SetDiscriminant(_8, 2);
match _14 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb14 = {
_7.0 = 9448303102645808004_u64 & 16445851306131491886_u64;
_5 = _6;
(*_1) = _7.0 as f64;
_13 = _14 << _7.1;
_7.1 = -248145952_i32;
_9 = (*_1) as f32;
_7.0 = !15087262470458693504_u64;
_7.2 = 14680920556448091723_usize;
_17 = !_10;
_7.2 = 8777754013598945722_usize & 2511300723198710624_usize;
_18 = !2371692839_u32;
_6 = _5;
_16 = _11;
_22 = [33215_u16,28418_u16];
_19 = 28396_i16;
_19 = !(-23244_i16);
_7.0 = 2961231043237733754_u64;
_24 = &_16;
RET = [32_u8,180_u8,191_u8,133_u8,253_u8,5_u8,216_u8];
_13 = _19 as isize;
Goto(bb13)
}
bb15 = {
RET = _5;
_3 = core::ptr::addr_of!((*_1));
(*_3) = 2089266385_u32 as f64;
_2 = '\u{7bd51}';
_13 = (-9223372036854775808_isize);
match _13 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_5 = [109_u8,243_u8,239_u8,0_u8,97_u8,158_u8,104_u8];
_7.1 = _25.1;
_28.fld0.2 = 46671_u16 >> _7.1;
place!(Field::<Adt45>(Variant(_8, 2), 2)).fld5.0 = !_25.0;
_28.fld3.fld0 = (_7.2, 45_u8, _22);
_18 = !1968047971_u32;
place!(Field::<Adt45>(Variant(_8, 2), 2)).fld4 = [(*_24)];
_8 = Adt51::Variant0 { fld0: _3,fld1: _9 };
_28.fld3.fld0 = (_25.2, 35_u8, _22);
_28.fld3.fld0.1 = 173_u8;
_24 = &_16;
_28.fld3.fld2 = [(*_24)];
Goto(bb20)
}
bb20 = {
Call(_30 = dump_var(10_usize, 18_usize, Move(_18), 2_usize, Move(_2), 11_usize, Move(_11), 10_usize, Move(_10)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_30 = dump_var(10_usize, 5_usize, Move(_5), 25_usize, Move(_25), 6_usize, Move(_6), 31_usize, _31), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [u8; 7],mut _2: f64,mut _3: f64,mut _4: (u64, i32, usize),mut _5: u64,mut _6: f64,mut _7: f64,mut _8: usize,mut _9: [u8; 7],mut _10: [u8; 7],mut _11: *const f64,mut _12: [u8; 7],mut _13: [u8; 7],mut _14: [u8; 7],mut _15: usize,mut _16: f64) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _17: [u128; 7];
let _18: u32;
let _19: (usize, u8, [u16; 2]);
let _20: Adt43;
let _21: u64;
let _22: (i64, i32, f64);
let _23: u16;
let _24: u32;
let _25: u128;
let _26: u8;
let _27: Adt49;
let _28: *mut &'static char;
let _29: usize;
let _30: [isize; 6];
let _31: ();
let _32: ();
{
_14 = _1;
_16 = (*_11) - _2;
_14 = _10;
RET = [_9[_8],_9[_8],_10[_8],_9[_8],_12[_8],_1[_8],_1[_8]];
_1[_8] = RET[_8];
_4 = (_5, 125506943_i32, _8);
_13 = [_14[_8],_10[_8],_14[_8],_1[_8],_9[_8],RET[_8],_14[_8]];
_14 = [RET[_8],_9[_8],RET[_8],RET[_8],RET[_8],_1[_8],_9[_8]];
_9 = [_12[_8],_12[_8],_1[_8],_12[_8],_10[_8],_14[_8],_12[_8]];
RET[_8] = !_12[_8];
_13 = _9;
_15 = _4.1 as usize;
_4.2 = (-8362764520238572483_i64) as usize;
_17 = [128699415383184748143647572644688855013_u128,91924261678371101644329966907007614528_u128,323904994506811546378507462171151713464_u128,124689668973947874182277537599746504118_u128,196849986735713811075186125453709123465_u128,79276065828029038001155893027375702788_u128,139863167681345271047005535118418614571_u128];
_13 = _14;
_12 = [_14[_8],_13[_8],_14[_8],_1[_8],_10[_8],_1[_8],_1[_8]];
Goto(bb1)
}
bb1 = {
_2 = _7;
_18 = 2804045309_u32 * 2853831541_u32;
_19.0 = _8 | _15;
_19.2 = [62760_u16,47460_u16];
_10 = [_9[_8],_13[_8],_12[_8],_9[_8],_13[_8],_14[_8],_12[_8]];
_4.2 = !_15;
RET[_8] = (-104653343686110020335054356792511484030_i128) as u8;
_8 = _15;
_16 = (*_11) * _2;
_18 = 3400747023_u32 + 3380200271_u32;
_12 = [212_u8,79_u8,13_u8,53_u8,193_u8,48_u8,10_u8];
RET = [95_u8,25_u8,128_u8,72_u8,228_u8,137_u8,196_u8];
_17 = [166105481977528417393601572030447099979_u128,103603369443092102833311470075780067316_u128,126243281612795144648560230577747387367_u128,13892576065773101578073138667505358233_u128,125156381446254434554018796849741685536_u128,238051440046800686112664218979146299941_u128,275799744152708633996151316097292148313_u128];
_4 = (_5, (-23011289_i32), _15);
_20.fld3.1 = _5 ^ _4.0;
_20.fld3.3 = _5;
_20.fld0 = (_19.0, 36_u8, _19.2);
_19.1 = (-1908834819020606612_i64) as u8;
_9 = _12;
match _4.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431745200167 => bb7,
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
_20.fld3.4 = _15 <= _20.fld0.0;
_20.fld2 = ['\u{119a4}'];
_5 = 16977_u16 as u64;
_3 = _2;
_4.1 = 268381467_i32 + 1675900719_i32;
_20.fld1 = 24513_i16 ^ 10419_i16;
_19 = _20.fld0;
_21 = _20.fld3.1 & _4.0;
_11 = core::ptr::addr_of!(_6);
_4.2 = _20.fld0.0;
_10 = _13;
(*_11) = _18 as f64;
match _4.0 {
0 => bb8,
13912717941004510406 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_22.2 = -_2;
_22.0 = _4.1 as i64;
_5 = _21;
_14 = [_19.1,_19.1,_20.fld0.1,_20.fld0.1,_19.1,_19.1,_19.1];
_20.fld0.2 = [8922_u16,47112_u16];
(*_11) = _18 as f64;
_20.fld1 = 28475_u16 as i16;
_20.fld3.5 = '\u{bc84e}';
_14 = [_19.1,_19.1,_19.1,_20.fld0.1,_19.1,_20.fld0.1,_20.fld0.1];
_25 = 65522511443769442193809331088204551123_u128 + 20810667446193364864075756541087427929_u128;
_25 = (-54_i8) as u128;
_16 = _2 - _3;
_4 = (_21, (-1242956831_i32), _20.fld0.0);
_20.fld3.0 = _20.fld0.1 as i16;
_20.fld3.0 = -_20.fld1;
_7 = _6 - _2;
_11 = core::ptr::addr_of!(_16);
_24 = !_18;
_20.fld3.5 = '\u{e12e0}';
Call(_7 = fn12(_13, _12, _10, _20.fld0.1, _20.fld2, _20.fld1, _19.0, _4.0, _20.fld2, _4.2, _4.0, _19), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_19.1 = 23_isize as u8;
_3 = _22.2;
_9 = [_20.fld0.1,_20.fld0.1,_20.fld0.1,_19.1,_20.fld0.1,_20.fld0.1,_19.1];
_4.0 = !_5;
_10 = _9;
_1 = [_20.fld0.1,_20.fld0.1,_20.fld0.1,_20.fld0.1,_20.fld0.1,_20.fld0.1,_20.fld0.1];
_18 = _24;
_20.fld0.1 = _20.fld3.0 as u8;
_20.fld3.0 = _20.fld1;
match _4.1 {
0 => bb9,
1 => bb2,
2 => bb5,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607430525254625 => bb16,
_ => bb15
}
}
bb12 = {
_22.2 = -_2;
_22.0 = _4.1 as i64;
_5 = _21;
_14 = [_19.1,_19.1,_20.fld0.1,_20.fld0.1,_19.1,_19.1,_19.1];
_20.fld0.2 = [8922_u16,47112_u16];
(*_11) = _18 as f64;
_20.fld1 = 28475_u16 as i16;
_20.fld3.5 = '\u{bc84e}';
_14 = [_19.1,_19.1,_19.1,_20.fld0.1,_19.1,_20.fld0.1,_20.fld0.1];
_25 = 65522511443769442193809331088204551123_u128 + 20810667446193364864075756541087427929_u128;
_25 = (-54_i8) as u128;
_16 = _2 - _3;
_4 = (_21, (-1242956831_i32), _20.fld0.0);
_20.fld3.0 = _20.fld0.1 as i16;
_20.fld3.0 = -_20.fld1;
_7 = _6 - _2;
_11 = core::ptr::addr_of!(_16);
_24 = !_18;
_20.fld3.5 = '\u{e12e0}';
Call(_7 = fn12(_13, _12, _10, _20.fld0.1, _20.fld2, _20.fld1, _19.0, _4.0, _20.fld2, _4.2, _4.0, _19), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_18 = 9223372036854775807_isize as u32;
_20.fld3.4 = true | true;
_22.1 = _4.1 & _4.1;
(*_11) = -_2;
_25 = 194288823084598783983664493879191577179_u128;
_22.0 = !2368300601099914609_i64;
_13 = [_19.1,_20.fld0.1,_20.fld0.1,_19.1,_20.fld0.1,_20.fld0.1,_20.fld0.1];
(*_11) = _22.2 + _7;
_12 = _10;
_25 = 155697892968797358056978466056746169171_u128 & 211303414047518441250656400706691024822_u128;
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(11_usize, 25_usize, Move(_25), 5_usize, Move(_5), 21_usize, Move(_21), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(11_usize, 18_usize, Move(_18), 1_usize, Move(_1), 8_usize, Move(_8), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [u8; 7],mut _2: [u8; 7],mut _3: [u8; 7],mut _4: u8,mut _5: [char; 1],mut _6: i16,mut _7: usize,mut _8: u64,mut _9: [char; 1],mut _10: usize,mut _11: u64,mut _12: (usize, u8, [u16; 2])) -> f64 {
mir! {
type RET = f64;
let _13: (i16, u64, i16, u64, bool, char);
let _14: Adt42;
let _15: Adt44;
let _16: isize;
let _17: char;
let _18: (u16,);
let _19: f32;
let _20: (i16, u64, i16, u64, bool, char);
let _21: Adt51;
let _22: (usize, u8, [u16; 2]);
let _23: isize;
let _24: Adt53;
let _25: [isize; 6];
let _26: (u16,);
let _27: f32;
let _28: ();
let _29: ();
{
_12.1 = _4 | _4;
RET = (-632441856_i32) as f64;
_13.2 = -_6;
_12.1 = _4 >> _4;
RET = (-9223372036854775808_isize) as f64;
_12.2 = [19589_u16,60570_u16];
_13.5 = '\u{3f85f}';
_13.4 = false;
RET = (-778714408401320283_i64) as f64;
RET = 283906598279918285661469424230406789355_u128 as f64;
_13 = (_6, _11, _6, _11, false, '\u{e189}');
_17 = _13.5;
_15 = Adt44::Variant2 { fld0: 113_i8 };
_18.0 = _6 as u16;
Goto(bb1)
}
bb1 = {
RET = _11 as f64;
_13.5 = _17;
place!(Field::<i8>(Variant(_15, 2), 0)) = -(-43_i8);
_2 = _1;
_12.0 = !_7;
_5 = [_17];
_16 = Field::<i8>(Variant(_15, 2), 0) as isize;
_10 = _7;
_6 = Field::<i8>(Variant(_15, 2), 0) as i16;
_13.2 = _6;
_20.2 = !_13.0;
_11 = !_13.1;
SetDiscriminant(_15, 1);
_20.4 = !_13.4;
_13.2 = _13.0 >> _12.1;
_18.0 = 56770_u16 + 11204_u16;
_2 = [_12.1,_12.1,_12.1,_12.1,_4,_4,_4];
_12.2 = [_18.0,_18.0];
_18 = (50624_u16,);
_20.3 = _11 << _4;
_13.1 = _13.3 << _16;
_6 = _10 as i16;
place!(Field::<u32>(Variant(_15, 1), 2)) = 3767020001_u32;
_19 = _4 as f32;
_20.0 = 22765690638644896162574572412347026765_u128 as i16;
_4 = _12.1;
_18.0 = 4482_u16 + 11671_u16;
Goto(bb2)
}
bb2 = {
_9 = [_17];
_13.3 = !_8;
_20.4 = !_13.4;
_13.0 = 6505089238924905889_i64 as i16;
_20 = (_13.2, _13.1, _6, _13.1, _13.4, _13.5);
_2 = [_12.1,_12.1,_4,_12.1,_12.1,_12.1,_12.1];
_20 = _13;
match Field::<u32>(Variant(_15, 1), 2) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
3767020001 => bb11,
_ => bb10
}
}
bb3 = {
RET = _11 as f64;
_13.5 = _17;
place!(Field::<i8>(Variant(_15, 2), 0)) = -(-43_i8);
_2 = _1;
_12.0 = !_7;
_5 = [_17];
_16 = Field::<i8>(Variant(_15, 2), 0) as isize;
_10 = _7;
_6 = Field::<i8>(Variant(_15, 2), 0) as i16;
_13.2 = _6;
_20.2 = !_13.0;
_11 = !_13.1;
SetDiscriminant(_15, 1);
_20.4 = !_13.4;
_13.2 = _13.0 >> _12.1;
_18.0 = 56770_u16 + 11204_u16;
_2 = [_12.1,_12.1,_12.1,_12.1,_4,_4,_4];
_12.2 = [_18.0,_18.0];
_18 = (50624_u16,);
_20.3 = _11 << _4;
_13.1 = _13.3 << _16;
_6 = _10 as i16;
place!(Field::<u32>(Variant(_15, 1), 2)) = 3767020001_u32;
_19 = _4 as f32;
_20.0 = 22765690638644896162574572412347026765_u128 as i16;
_4 = _12.1;
_18.0 = 4482_u16 + 11671_u16;
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
_13.4 = _4 > _4;
_22.0 = !_7;
_5 = [_17];
place!(Field::<char>(Variant(_15, 1), 1)) = _13.5;
_2 = _1;
_9 = _5;
_7 = (-1458092681_i32) as usize;
_13.5 = _20.5;
Call(place!(Field::<bool>(Variant(_15, 1), 0)) = fn13(_12, _5, _20.4), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _19 as f64;
_13.1 = !_20.3;
_13.0 = _20.2 - _20.2;
_20.4 = _6 < _20.2;
_13 = _20;
_13.5 = _17;
_20.0 = _20.1 as i16;
_20.3 = !_8;
place!(Field::<char>(Variant(_15, 1), 1)) = _20.5;
_25 = [_16,_16,_16,_16,_16,_16];
_23 = _16;
_16 = Field::<u32>(Variant(_15, 1), 2) as isize;
_13.2 = _18.0 as i16;
_26.0 = 2568607150688762920_i64 as u16;
_2 = [_12.1,_4,_4,_12.1,_4,_12.1,_4];
_20.4 = _20.2 > _20.2;
_22.2 = _12.2;
_25 = [_23,_23,_23,_23,_16,_23];
_22.1 = _12.1 | _4;
_13 = (_20.2, _20.3, _6, _20.3, Field::<bool>(Variant(_15, 1), 0), _20.5);
place!(Field::<bool>(Variant(_15, 1), 0)) = !_13.4;
place!(Field::<bool>(Variant(_15, 1), 0)) = !_13.4;
_19 = 35184008467317660_i64 as f32;
_19 = _23 as f32;
Goto(bb13)
}
bb13 = {
Call(_28 = dump_var(12_usize, 11_usize, Move(_11), 10_usize, Move(_10), 16_usize, Move(_16), 25_usize, Move(_25)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_28 = dump_var(12_usize, 6_usize, Move(_6), 12_usize, Move(_12), 3_usize, Move(_3), 23_usize, Move(_23)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_28 = dump_var(12_usize, 17_usize, Move(_17), 22_usize, Move(_22), 29_usize, _29, 29_usize, _29), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: (usize, u8, [u16; 2]),mut _2: [char; 1],mut _3: bool) -> bool {
mir! {
type RET = bool;
let _4: u64;
let _5: Adt44;
let _6: i16;
let _7: ();
let _8: ();
{
RET = _3;
RET = _1.1 == _1.1;
_4 = !12436475180713764569_u64;
_4 = 8685902714236156882_u64;
_3 = RET;
_3 = RET != RET;
_1.2 = [24419_u16,44967_u16];
RET = _3;
_6 = (-3096_i16) * (-8188_i16);
_1.0 = 6087462575629336556_usize;
_1.0 = 5_usize;
_6 = 10730_i16;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(13_usize, 4_usize, Move(_4), 1_usize, Move(_1), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: char,mut _2: (i16, u64, i16, u64, bool, char),mut _3: [u8; 7],mut _4: (i16, u64, i16, u64, bool, char),mut _5: char,mut _6: [u8; 7],mut _7: u64) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _8: [isize; 6];
let _9: (u64, i32, usize);
let _10: Adt52;
let _11: isize;
let _12: isize;
let _13: isize;
let _14: u16;
let _15: Adt50;
let _16: usize;
let _17: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _18: (i64, i32, f64);
let _19: Adt47;
let _20: (u64, i32, usize);
let _21: bool;
let _22: isize;
let _23: f32;
let _24: Adt44;
let _25: isize;
let _26: [u16; 2];
let _27: u128;
let _28: f64;
let _29: isize;
let _30: [char; 1];
let _31: [u128; 7];
let _32: isize;
let _33: f64;
let _34: f64;
let _35: Adt46;
let _36: ();
let _37: ();
{
RET = [239_u8,200_u8,233_u8,29_u8,179_u8,191_u8,88_u8];
_4.1 = _4.2 as u64;
_2.5 = _4.5;
_6 = _3;
RET = [120_u8,163_u8,160_u8,51_u8,21_u8,84_u8,209_u8];
_4 = (_2.2, _2.1, _2.0, _7, _2.4, _5);
RET = [69_u8,133_u8,192_u8,138_u8,188_u8,129_u8,75_u8];
_8 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4.3 = _4.1;
_3 = [35_u8,228_u8,200_u8,222_u8,12_u8,12_u8,105_u8];
_2.3 = _4.3 ^ _2.1;
_3 = RET;
RET = [10_u8,198_u8,108_u8,117_u8,25_u8,111_u8,167_u8];
_8 = [9223372036854775807_isize,(-15_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
Call(_9.2 = core::intrinsics::bswap(2_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.fld4.fld3.fld3.2 = _2.0 << _7;
_10.fld4.fld3.fld0.2 = [63236_u16,20688_u16];
_10.fld4.fld3.fld2 = [_5];
_10.fld3 = [118_u8,47_u8,250_u8,176_u8,235_u8,98_u8,20_u8];
_2 = (_4.2, _4.3, _10.fld4.fld3.fld3.2, _4.3, _4.4, _4.5);
_2.0 = _10.fld4.fld3.fld3.2 * _2.2;
_10.fld4.fld3.fld3.5 = _5;
_10.fld4.fld0 = (12147_u16, 6448933384248974706_i64, 1317_u16);
_11 = 9223372036854775807_isize;
_10.fld4.fld3.fld3.3 = !_2.1;
_10.fld4.fld3.fld1 = _2.0;
_4.4 = _2.4 ^ _2.4;
_2 = (_10.fld4.fld3.fld1, _4.3, _10.fld4.fld3.fld1, _7, _4.4, _4.5);
_10.fld4.fld1 = [_7,_2.1,_10.fld4.fld3.fld3.3,_2.1,_10.fld4.fld3.fld3.3,_7];
_8 = [_11,_11,_11,_11,_11,_11];
_15.fld3.fld1 = 155962790464736522623767991573432224766_i128 as i16;
_6 = [251_u8,53_u8,222_u8,213_u8,55_u8,175_u8,123_u8];
_15.fld3.fld3.3 = _2.3 - _10.fld4.fld3.fld3.3;
_2.3 = _11 as u64;
match _7 {
0 => bb2,
2404712990160464100 => bb4,
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
_15.fld3.fld3.1 = _7;
_15.fld0.2 = !_10.fld4.fld0.0;
_10.fld4.fld0.1 = -(-1542068269328240714_i64);
_11 = _10.fld4.fld0.1 as isize;
_15.fld3.fld0.2 = [_10.fld4.fld0.0,_10.fld4.fld0.0];
_15.fld3.fld0 = (16577301253631986513_usize, 80_u8, _10.fld4.fld3.fld0.2);
_17.2 = !3104159252_u32;
_10.fld4.fld3.fld3.0 = _10.fld4.fld3.fld1;
_9.1 = (-114_i8) as i32;
_15.fld0.1 = _10.fld4.fld0.1 << _10.fld4.fld3.fld3.0;
_10.fld0 = [_11,_11,_11,_11,_11,_11];
_10.fld4.fld3.fld0 = (_15.fld3.fld0.0, _15.fld3.fld0.1, _15.fld3.fld0.2);
_4 = (_10.fld4.fld3.fld1, _15.fld3.fld3.3, _2.2, _15.fld3.fld3.3, _2.4, _5);
match _15.fld3.fld0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
16577301253631986513 => bb8,
_ => bb7
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_10.fld4.fld3.fld3.2 = _2.0 << _7;
_10.fld4.fld3.fld0.2 = [63236_u16,20688_u16];
_10.fld4.fld3.fld2 = [_5];
_10.fld3 = [118_u8,47_u8,250_u8,176_u8,235_u8,98_u8,20_u8];
_2 = (_4.2, _4.3, _10.fld4.fld3.fld3.2, _4.3, _4.4, _4.5);
_2.0 = _10.fld4.fld3.fld3.2 * _2.2;
_10.fld4.fld3.fld3.5 = _5;
_10.fld4.fld0 = (12147_u16, 6448933384248974706_i64, 1317_u16);
_11 = 9223372036854775807_isize;
_10.fld4.fld3.fld3.3 = !_2.1;
_10.fld4.fld3.fld1 = _2.0;
_4.4 = _2.4 ^ _2.4;
_2 = (_10.fld4.fld3.fld1, _4.3, _10.fld4.fld3.fld1, _7, _4.4, _4.5);
_10.fld4.fld1 = [_7,_2.1,_10.fld4.fld3.fld3.3,_2.1,_10.fld4.fld3.fld3.3,_7];
_8 = [_11,_11,_11,_11,_11,_11];
_15.fld3.fld1 = 155962790464736522623767991573432224766_i128 as i16;
_6 = [251_u8,53_u8,222_u8,213_u8,55_u8,175_u8,123_u8];
_15.fld3.fld3.3 = _2.3 - _10.fld4.fld3.fld3.3;
_2.3 = _11 as u64;
match _7 {
0 => bb2,
2404712990160464100 => bb4,
_ => bb3
}
}
bb8 = {
_15.fld2 = Adt44::Variant0 { fld0: _4.0,fld1: _15.fld3.fld3.3,fld2: _15.fld3.fld0.0,fld3: 113_i8 };
Goto(bb9)
}
bb9 = {
_17.2 = !1200241913_u32;
_17.0.0 = Field::<i16>(Variant(_15.fld2, 0), 0) | _10.fld4.fld3.fld3.0;
place!(Field::<usize>(Variant(_15.fld2, 0), 2)) = _10.fld4.fld3.fld0.0;
_2.2 = _17.0.0;
_10.fld4.fld3.fld3.4 = _2.4;
place!(Field::<usize>(Variant(_15.fld2, 0), 2)) = _15.fld0.1 as usize;
_4.1 = _15.fld0.2 as u64;
_17.0 = _2;
_10.fld4.fld0.1 = _10.fld4.fld3.fld0.1 as i64;
_15.fld1 = _10.fld4.fld1;
_10.fld4.fld0.2 = _10.fld4.fld0.0 & _10.fld4.fld0.0;
_1 = _2.5;
_10.fld4.fld3.fld3.4 = _15.fld3.fld3.3 >= Field::<u64>(Variant(_15.fld2, 0), 1);
_8 = _10.fld0;
_15.fld3.fld1 = _15.fld0.1 as i16;
_15.fld3.fld2 = [_5];
Goto(bb10)
}
bb10 = {
_7 = _10.fld4.fld3.fld3.4 as u64;
_15.fld1 = [_17.0.1,_10.fld4.fld3.fld3.3,_10.fld4.fld3.fld3.3,_15.fld3.fld3.1,_4.3,Field::<u64>(Variant(_15.fld2, 0), 1)];
match _15.fld3.fld3.1 {
2404712990160464100 => bb11,
_ => bb7
}
}
bb11 = {
_20 = (_15.fld3.fld3.3, _9.1, Field::<usize>(Variant(_15.fld2, 0), 2));
_13 = _11 - _11;
_9 = (_4.3, _20.1, Field::<usize>(Variant(_15.fld2, 0), 2));
_15.fld4 = _4.3 ^ _4.1;
_10.fld4.fld2 = Adt44::Variant2 { fld0: 120_i8 };
_10.fld4.fld3.fld0.2 = [_10.fld4.fld0.2,_10.fld4.fld0.0];
_16 = !_20.2;
Goto(bb12)
}
bb12 = {
RET = [_15.fld3.fld0.1,_15.fld3.fld0.1,_15.fld3.fld0.1,_15.fld3.fld0.1,_10.fld4.fld3.fld0.1,_10.fld4.fld3.fld0.1,_15.fld3.fld0.1];
_14 = _10.fld4.fld0.0;
_17.0.4 = _10.fld4.fld3.fld3.4;
_10.fld4.fld3.fld0.1 = _15.fld3.fld0.1;
_15.fld3.fld3.4 = !_4.4;
_10.fld4.fld3.fld3.0 = _17.0.0 | _4.2;
_17.1 = [_9.0,_10.fld4.fld3.fld3.3,_7,_15.fld3.fld3.3,_10.fld4.fld3.fld3.3,_15.fld4];
_10.fld4.fld3.fld3.5 = _1;
Goto(bb13)
}
bb13 = {
_15.fld2 = Adt44::Variant2 { fld0: 34_i8 };
_1 = _4.5;
_10.fld4.fld4 = _7;
_10.fld4.fld3.fld3.2 = _2.0 + _2.0;
_2.2 = _10.fld4.fld3.fld3.0 << _15.fld3.fld1;
_20 = _9;
_18.0 = _10.fld4.fld0.1 | _15.fld0.1;
_15.fld0.2 = !_10.fld4.fld0.2;
_29 = -_13;
_2.3 = _20.0 / _15.fld3.fld3.1;
_10.fld0 = [_13,_11,_13,_11,_13,_29];
_4 = _2;
_10.fld4.fld3.fld3.0 = _10.fld4.fld0.1 as i16;
_15.fld3 = Adt43 { fld0: _10.fld4.fld3.fld0,fld1: _4.2,fld2: _10.fld4.fld3.fld2,fld3: _2 };
_20.2 = _15.fld3.fld0.0 % _10.fld4.fld3.fld0.0;
_10.fld4.fld0.2 = !_15.fld0.2;
_9.2 = _16 * _20.2;
Call(_9.1 = fn15(_15.fld3.fld3.2, _15.fld3.fld3.2, _20.2, _10.fld0, _10.fld4.fld3.fld0.1, _4.2, _2.2, _15.fld3, _17.0.5, _17.0.2, _2.0, _18.0, _17.0.2, _17.0.1, _17.0, _15.fld3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17.4 = 2579518282259863999284827232355836158_u128;
_15.fld1 = _17.1;
_10.fld4.fld3.fld3.1 = _7 + _15.fld4;
_21 = _2.2 > _15.fld3.fld1;
_22 = _29 + _13;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(14_usize, 21_usize, Move(_21), 3_usize, Move(_3), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(14_usize, 20_usize, Move(_20), 2_usize, Move(_2), 7_usize, Move(_7), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i16,mut _2: i16,mut _3: usize,mut _4: [isize; 6],mut _5: u8,mut _6: i16,mut _7: i16,mut _8: Adt43,mut _9: char,mut _10: i16,mut _11: i16,mut _12: i64,mut _13: i16,mut _14: u64,mut _15: (i16, u64, i16, u64, bool, char),mut _16: Adt43) -> i32 {
mir! {
type RET = i32;
let _17: f32;
let _18: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _19: *mut i64;
let _20: bool;
let _21: i32;
let _22: usize;
let _23: u8;
let _24: u64;
let _25: (u64, i32, usize);
let _26: [u16; 2];
let _27: char;
let _28: [u64; 6];
let _29: *mut &'static char;
let _30: ();
let _31: ();
{
_15.1 = !_8.fld3.3;
_16.fld0 = _8.fld0;
_15.5 = _9;
_18.3 = !_8.fld0.1;
_17 = _12 as f32;
_16.fld3.2 = _16.fld3.4 as i16;
_15.4 = _16.fld3.0 < _11;
_8.fld1 = _1;
_18.0.5 = _15.5;
_16.fld0.2 = [10545_u16,49118_u16];
_8.fld0 = _16.fld0;
_18.4 = 282801786970361117828929187116522215394_u128 + 87994314207976276132217068780502375459_u128;
_16.fld0.2 = [42765_u16,41848_u16];
_8.fld3 = (_16.fld1, _16.fld3.3, _13, _16.fld3.3, _15.4, _9);
_23 = _8.fld0.1 % _16.fld0.1;
_8.fld3.2 = _8.fld0.0 as i16;
_16.fld1 = _12 as i16;
_18.0.3 = _12 as u64;
_16.fld3.3 = _18.0.3;
_16.fld0 = (_3, _8.fld0.1, _8.fld0.2);
_18.0.2 = _7 + _10;
Goto(bb1)
}
bb1 = {
_16.fld3.2 = !_6;
_8.fld3 = (_10, _18.0.3, _8.fld1, _16.fld3.3, _16.fld3.4, _18.0.5);
_25.1 = 773360126_i32 >> _16.fld3.2;
_19 = core::ptr::addr_of_mut!(_12);
_19 = core::ptr::addr_of_mut!((*_19));
_21 = !_25.1;
_2 = _8.fld1;
_16.fld0.1 = !_23;
_13 = _16.fld3.2 | _16.fld3.2;
_8.fld3.1 = !_18.0.3;
_18.0.2 = _8.fld3.2;
(*_19) = 1358850680677774476_i64 + (-5232853390516896797_i64);
_18.0.0 = _18.3 as i16;
_25.2 = !_16.fld0.0;
_12 = !3107768042287308530_i64;
_16.fld3 = (_8.fld3.0, _18.0.3, _18.0.2, _8.fld3.1, _15.4, _18.0.5);
_6 = _15.0;
_17 = _25.1 as f32;
_16.fld3.0 = _13 - _13;
_2 = !_11;
RET = _8.fld3.3 as i32;
_18.2 = 6113_u16 as u32;
_25 = (_15.1, RET, _3);
Goto(bb2)
}
bb2 = {
Call(_30 = dump_var(15_usize, 4_usize, Move(_4), 13_usize, Move(_13), 5_usize, Move(_5), 21_usize, Move(_21)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_30 = dump_var(15_usize, 25_usize, Move(_25), 3_usize, Move(_3), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [u8; 7],mut _2: bool,mut _3: [u8; 7],mut _4: i16,mut _5: [u8; 7],mut _6: bool,mut _7: u64,mut _8: f64,mut _9: [u8; 7],mut _10: bool,mut _11: bool,mut _12: u64,mut _13: [u8; 7],mut _14: i16) -> [u8; 7] {
mir! {
type RET = [u8; 7];
let _15: &'static char;
let _16: *mut &'static char;
let _17: Adt51;
let _18: Adt44;
let _19: f32;
let _20: i64;
let _21: [u64; 6];
let _22: &'static char;
let _23: f64;
let _24: *mut &'static char;
let _25: char;
let _26: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128);
let _27: i64;
let _28: *const f64;
let _29: char;
let _30: isize;
let _31: isize;
let _32: [u16; 2];
let _33: f64;
let _34: (u16,);
let _35: isize;
let _36: [char; 1];
let _37: ();
let _38: ();
{
_8 = 30788_u16 as f64;
RET = [90_u8,110_u8,62_u8,135_u8,117_u8,120_u8,170_u8];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
9384780542817540837 => bb9,
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
RET = _9;
_7 = (-2128872660_i32) as u64;
_3 = _1;
_4 = (-132213525358569504776461038482998130898_i128) as i16;
_14 = !_4;
_1 = _13;
_6 = _11 == _10;
_3 = _13;
_1 = [21_u8,219_u8,36_u8,129_u8,149_u8,208_u8,181_u8];
_14 = !_4;
RET = [111_u8,206_u8,183_u8,138_u8,1_u8,8_u8,241_u8];
_14 = -_4;
_11 = _2;
_13 = [186_u8,137_u8,239_u8,66_u8,41_u8,183_u8,210_u8];
_13 = [137_u8,158_u8,236_u8,123_u8,21_u8,150_u8,122_u8];
_13 = [211_u8,107_u8,192_u8,186_u8,43_u8,30_u8,53_u8];
_16 = core::ptr::addr_of_mut!(_15);
_12 = !_7;
_9 = [178_u8,43_u8,193_u8,254_u8,83_u8,83_u8,31_u8];
_5 = RET;
_20 = -(-1610648232745582772_i64);
_19 = 63_i8 as f32;
Goto(bb10)
}
bb10 = {
_5 = [236_u8,137_u8,149_u8,74_u8,209_u8,135_u8,203_u8];
_6 = !_2;
_7 = _12 + _12;
_4 = -_14;
_13 = [251_u8,78_u8,48_u8,252_u8,243_u8,136_u8,36_u8];
_12 = _7 - _7;
_16 = core::ptr::addr_of_mut!((*_16));
_11 = _2 != _10;
_5 = RET;
_2 = !_11;
_7 = _12;
_21 = [_12,_12,_12,_12,_12,_7];
RET = [176_u8,9_u8,249_u8,172_u8,215_u8,149_u8,156_u8];
_11 = !_6;
_10 = !_2;
_20 = 141495437140436588_i64 + 5462790277638723751_i64;
_10 = _11;
_2 = _12 != _12;
_3 = _9;
_16 = core::ptr::addr_of_mut!(_15);
_24 = core::ptr::addr_of_mut!((*_16));
_12 = _7;
_25 = '\u{e3e30}';
RET = [180_u8,34_u8,10_u8,97_u8,164_u8,64_u8,133_u8];
Call(_3 = core::intrinsics::transmute(_9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = _4;
_9 = [208_u8,87_u8,176_u8,119_u8,128_u8,133_u8,103_u8];
Goto(bb12)
}
bb12 = {
_26.0.1 = !_12;
_14 = _4 << _7;
(*_24) = &_26.0.5;
_26.0.0 = 157386926560788897220746008938099648750_u128 as i16;
_26.2 = 3324930801_u32 + 689411329_u32;
_12 = !_26.0.1;
_26.0.1 = _7 + _12;
_9 = _5;
_7 = _26.0.1 << _14;
_27 = -_20;
_6 = _2;
_26.0.3 = _7;
Goto(bb13)
}
bb13 = {
_7 = _12 << _14;
_4 = 39_isize as i16;
(*_24) = &_25;
_9 = RET;
_23 = -_8;
_26.0.4 = !_6;
(*_24) = &_25;
_8 = -_23;
_22 = &_26.0.5;
_22 = &(*_22);
_26.0.3 = !_7;
_27 = _20;
(*_16) = &(*_15);
(*_16) = &(*_22);
_21 = [_26.0.3,_26.0.3,_12,_7,_26.0.1,_26.0.1];
Call(_26.0.2 = core::intrinsics::transmute(_26.0.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15 = &(*_22);
_1 = [225_u8,20_u8,198_u8,240_u8,108_u8,101_u8,89_u8];
_7 = (-104_i8) as u64;
_28 = core::ptr::addr_of!(_23);
_9 = RET;
_24 = core::ptr::addr_of_mut!((*_24));
_27 = (-9223372036854775808_isize) as i64;
_11 = !_2;
_4 = !_14;
(*_28) = -_8;
_12 = _26.0.1 | _26.0.3;
_33 = 1986484606_i32 as f64;
_13 = [105_u8,212_u8,2_u8,58_u8,180_u8,77_u8,145_u8];
_35 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_22 = &(*_15);
(*_16) = &_25;
_14 = _4 + _4;
_31 = _35;
_7 = _26.0.3 * _12;
_26.0.2 = _14;
_9 = _3;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(16_usize, 2_usize, Move(_2), 12_usize, Move(_12), 14_usize, Move(_14), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(16_usize, 10_usize, Move(_10), 1_usize, Move(_1), 25_usize, Move(_25), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(16_usize, 4_usize, Move(_4), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u64,mut _2: u64,mut _3: i16,mut _4: [u8; 7],mut _5: u64,mut _6: (i16, u64, i16, u64, bool, char),mut _7: u64,mut _8: u64,mut _9: isize,mut _10: u64,mut _11: bool,mut _12: char,mut _13: i32) -> [char; 1] {
mir! {
type RET = [char; 1];
let _14: u16;
let _15: *mut i64;
let _16: *mut [u8; 7];
let _17: u128;
let _18: usize;
let _19: i128;
let _20: Adt55;
let _21: Adt56;
let _22: bool;
let _23: [u16; 7];
let _24: f64;
let _25: *mut [u8; 7];
let _26: f32;
let _27: i128;
let _28: bool;
let _29: bool;
let _30: [u16; 7];
let _31: (u16,);
let _32: f64;
let _33: ();
let _34: ();
{
_9 = !61_isize;
_6.0 = !_3;
_8 = !_6.1;
_10 = !_6.3;
_6.5 = _12;
_10 = _6.0 as u64;
_11 = !_6.4;
match _6.1 {
9384780542817540837 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_1 = 191_u8 as u64;
Goto(bb3)
}
bb3 = {
_6 = (_3, _7, _3, _2, _11, _12);
_6.1 = _7;
_6.2 = _3;
_3 = 53_u8 as i16;
_6.5 = _12;
_2 = _7 + _6.3;
_13 = 2187607650687658935_i64 as i32;
_4 = [96_u8,70_u8,232_u8,41_u8,194_u8,144_u8,160_u8];
_6.1 = _2;
_6.4 = _6.1 < _2;
_16 = core::ptr::addr_of_mut!(_4);
_5 = !_2;
_5 = !_6.1;
_2 = !_5;
_13 = _6.4 as i32;
_9 = 9223372036854775807_isize * 9223372036854775807_isize;
_13 = _8 as i32;
RET = [_12];
_1 = _2 * _6.3;
_6 = (_3, _7, _3, _1, _11, _12);
_6 = (_3, _5, _3, _5, _11, _12);
_17 = _11 as u128;
_17 = !284662192602184858415647053175255540966_u128;
_1 = !_2;
_11 = _6.4;
_1 = _6.1 + _6.1;
Goto(bb4)
}
bb4 = {
_7 = (-85680788867038744300687628147596603206_i128) as u64;
_6.0 = _3;
_6.2 = (-2941712313671343774_i64) as i16;
RET = [_6.5];
_6.1 = _12 as u64;
_3 = 21625_u16 as i16;
_6.1 = _2;
_12 = _6.5;
_16 = core::ptr::addr_of_mut!((*_16));
_4 = [21_u8,247_u8,73_u8,88_u8,63_u8,233_u8,128_u8];
_17 = 85260826233385259109744486692214048530_u128;
_16 = core::ptr::addr_of_mut!((*_16));
_2 = !_1;
_18 = 12325090695493508109_usize;
_6 = (_3, _2, _3, _2, _11, _12);
_13 = !1694889231_i32;
_6.0 = !_3;
_11 = !_6.4;
_8 = _18 as u64;
_6.1 = _5;
_16 = core::ptr::addr_of_mut!((*_16));
_22 = _6.4 | _6.4;
_14 = 56652_u16 ^ 20567_u16;
_6.2 = -_3;
match _18 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
12325090695493508109 => bb10,
_ => bb9
}
}
bb5 = {
_6 = (_3, _7, _3, _2, _11, _12);
_6.1 = _7;
_6.2 = _3;
_3 = 53_u8 as i16;
_6.5 = _12;
_2 = _7 + _6.3;
_13 = 2187607650687658935_i64 as i32;
_4 = [96_u8,70_u8,232_u8,41_u8,194_u8,144_u8,160_u8];
_6.1 = _2;
_6.4 = _6.1 < _2;
_16 = core::ptr::addr_of_mut!(_4);
_5 = !_2;
_5 = !_6.1;
_2 = !_5;
_13 = _6.4 as i32;
_9 = 9223372036854775807_isize * 9223372036854775807_isize;
_13 = _8 as i32;
RET = [_12];
_1 = _2 * _6.3;
_6 = (_3, _7, _3, _1, _11, _12);
_6 = (_3, _5, _3, _5, _11, _12);
_17 = _11 as u128;
_17 = !284662192602184858415647053175255540966_u128;
_1 = !_2;
_11 = _6.4;
_1 = _6.1 + _6.1;
Goto(bb4)
}
bb6 = {
_1 = 191_u8 as u64;
Goto(bb3)
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
_6 = (_3, _1, _3, _2, _22, _12);
_6.3 = !_2;
_21.fld0 = _17 & _17;
_13 = 956755455_i32;
_5 = _6.3;
RET = [_12];
_6.4 = !_22;
Goto(bb11)
}
bb11 = {
_2 = !_1;
_9 = 9223372036854775807_isize ^ 20_isize;
RET = [_6.5];
(*_16) = [164_u8,12_u8,141_u8,171_u8,187_u8,55_u8,94_u8];
_6.3 = _1 ^ _1;
_19 = _22 as i128;
_6.2 = _6.0 & _6.0;
_6.0 = _6.2 << _6.3;
_21.fld1 = _6.5;
_5 = 77_i8 as u64;
(*_16) = [244_u8,253_u8,213_u8,45_u8,94_u8,192_u8,169_u8];
RET = [_21.fld1];
_9 = 110_isize;
_24 = _6.0 as f64;
_4 = [246_u8,89_u8,36_u8,228_u8,219_u8,105_u8,250_u8];
RET = [_21.fld1];
_18 = 2_usize - 631508078322557893_usize;
_7 = _6.3 ^ _1;
_8 = !_6.3;
_17 = !_21.fld0;
_6 = (_3, _7, _3, _1, _22, _12);
_6 = (_3, _1, _3, _1, _22, _21.fld1);
_1 = _6.3;
_13 = -(-1792551559_i32);
_21.fld2 = !245_u8;
_17 = _24 as u128;
_6.1 = _11 as u64;
_6 = (_3, _2, _3, _1, _22, _21.fld1);
_4 = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
(*_16) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
_17 = _21.fld0;
Goto(bb12)
}
bb12 = {
_25 = core::ptr::addr_of_mut!((*_16));
_3 = _6.2;
_16 = _25;
_21.fld1 = _12;
(*_25) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
(*_16) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
_17 = _9 as u128;
_17 = _11 as u128;
RET = [_21.fld1];
_6.5 = _12;
_11 = !_22;
_1 = _13 as u64;
(*_16) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
_13 = (-2021577751_i32);
_9 = 9223372036854775807_isize;
_27 = _19;
_13 = -1738237575_i32;
_13 = 383914459_i32 - 1204709_i32;
Goto(bb13)
}
bb13 = {
_26 = (-89_i8) as f32;
_24 = _26 as f64;
_16 = core::ptr::addr_of_mut!((*_16));
_6.1 = _2 - _6.3;
_19 = _27;
_6.3 = _2;
_26 = _7 as f32;
_4 = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
(*_25) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
_6.5 = _21.fld1;
_22 = !_6.4;
_22 = _6.4;
_1 = _3 as u64;
(*_16) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
_21.fld1 = _12;
_28 = _6.4;
_10 = _6.3 * _7;
_21.fld0 = _17;
RET = [_12];
_8 = _7;
_21.fld2 = 58_u8;
(*_25) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
match _21.fld2 {
0 => bb11,
1 => bb14,
2 => bb15,
58 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
_2 = !_1;
_9 = 9223372036854775807_isize ^ 20_isize;
RET = [_6.5];
(*_16) = [164_u8,12_u8,141_u8,171_u8,187_u8,55_u8,94_u8];
_6.3 = _1 ^ _1;
_19 = _22 as i128;
_6.2 = _6.0 & _6.0;
_6.0 = _6.2 << _6.3;
_21.fld1 = _6.5;
_5 = 77_i8 as u64;
(*_16) = [244_u8,253_u8,213_u8,45_u8,94_u8,192_u8,169_u8];
RET = [_21.fld1];
_9 = 110_isize;
_24 = _6.0 as f64;
_4 = [246_u8,89_u8,36_u8,228_u8,219_u8,105_u8,250_u8];
RET = [_21.fld1];
_18 = 2_usize - 631508078322557893_usize;
_7 = _6.3 ^ _1;
_8 = !_6.3;
_17 = !_21.fld0;
_6 = (_3, _7, _3, _1, _22, _12);
_6 = (_3, _1, _3, _1, _22, _21.fld1);
_1 = _6.3;
_13 = -(-1792551559_i32);
_21.fld2 = !245_u8;
_17 = _24 as u128;
_6.1 = _11 as u64;
_6 = (_3, _2, _3, _1, _22, _21.fld1);
_4 = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
(*_16) = [_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_21.fld2];
_17 = _21.fld0;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
_31.0 = _14;
Goto(bb18)
}
bb18 = {
Call(_33 = dump_var(17_usize, 1_usize, Move(_1), 22_usize, Move(_22), 28_usize, Move(_28), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(17_usize, 8_usize, Move(_8), 9_usize, Move(_9), 27_usize, Move(_27), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(17_usize, 12_usize, Move(_12), 3_usize, Move(_3), 34_usize, _34, 34_usize, _34), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{8e4f}'), std::hint::black_box((-121_isize)), std::hint::black_box((-36_i8)), std::hint::black_box((-27133_i16)), std::hint::black_box((-800266828_i32)), std::hint::black_box((-6454885325066692894_i64)), std::hint::black_box((-127834718574731707000918699746939557023_i128)), std::hint::black_box(10722309146459583091_usize), std::hint::black_box(247_u8), std::hint::black_box(41495_u16), std::hint::black_box(1799075644_u32), std::hint::black_box(3428779245612304747_u64), std::hint::black_box(87082945816580105533032789951923873855_u128));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: (u64, i32, usize),

},
Variant1{
fld0: u64,
fld1: *mut usize,
fld2: isize,
fld3: (u16,),

},
Variant2{
fld0: u64,
fld1: f32,
fld2: i32,
fld3: [u8; 7],
fld4: [u64; 6],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128),
fld1: [u128; 7],
fld2: isize,
fld3: (u64, u128),
fld4: i16,
fld5: i128,
fld6: [u8; 7],

},
Variant1{
fld0: i64,
fld1: [u64; 6],
fld2: u16,
fld3: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128),
fld4: u64,

},
Variant2{
fld0: (i16, u64, i16, u64, bool, char),
fld1: (u16,),
fld2: [u128; 7],
fld3: (u16, i64, u16),

},
Variant3{
fld0: [u64; 6],
fld1: Adt41,
fld2: *mut u16,
fld3: (usize, u8, [u16; 2]),
fld4: u32,
fld5: i32,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: (usize, u8, [u16; 2]),
fld1: i16,
fld2: [char; 1],
fld3: (i16, u64, i16, u64, bool, char),
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: i16,
fld1: u64,
fld2: usize,
fld3: i8,

},
Variant1{
fld0: bool,
fld1: char,
fld2: u32,
fld3: [u128; 7],
fld4: u8,
fld5: *mut i64,

},
Variant2{
fld0: i8,

},
Variant3{
fld0: (u64, i32, usize),
fld1: (i64, i32, f64),
fld2: f32,
fld3: i128,
fld4: Adt43,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *mut u16,
fld1: char,
fld2: *const f64,
fld3: (usize, u8, [u16; 2]),
fld4: [char; 1],
fld5: (u64, i32, usize),
fld6: i64,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: char,

},
Variant1{
fld0: Adt43,
fld1: char,
fld2: (u16,),
fld3: Adt41,
fld4: (i64, i32, f64),
fld5: u8,
fld6: i64,
fld7: [u16; 2],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u16; 2],
fld1: Adt45,
fld2: *mut i64,
fld3: i16,

},
Variant1{
fld0: ((i16, u64, i16, u64, bool, char), [u64; 6], u32, u8, u128),
fld1: u128,

},
Variant2{
fld0: (i64, i32, f64),
fld1: Adt44,
fld2: (u16,),
fld3: i8,
fld4: *mut *const f64,
fld5: *mut [u8; 7],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt46,
fld1: Adt47,
fld2: isize,
fld3: (u64, i32, usize),
fld4: Adt45,

},
Variant1{
fld0: Adt43,
fld1: char,
fld2: [u8; 7],
fld3: *mut u16,

},
Variant2{
fld0: [u64; 6],
fld1: u16,
fld2: i32,

},
Variant3{
fld0: [u8; 7],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt41,
fld1: *mut *const f64,
fld2: f64,
fld3: i8,
fld4: u8,

},
Variant1{
fld0: (u16, i64, u16),
fld1: (i64, i32, f64),
fld2: (i16, u64, i16, u64, bool, char),
fld3: i8,
fld4: *const f64,
fld5: usize,

},
Variant2{
fld0: bool,
fld1: Adt43,
fld2: (i16, u64, i16, u64, bool, char),
fld3: [isize; 6],
fld4: i16,
fld5: f32,
fld6: u64,
fld7: (usize, u8, [u16; 2]),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: (u16, i64, u16),
fld1: [u64; 6],
fld2: Adt44,
fld3: Adt43,
fld4: u64,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *const f64,
fld1: f32,

},
Variant1{
fld0: i128,
fld1: (i16, u64, i16, u64, bool, char),
fld2: [u64; 6],
fld3: Adt47,
fld4: [char; 1],
fld5: (u16, i64, u16),

},
Variant2{
fld0: [isize; 6],
fld1: u32,
fld2: Adt45,

},
Variant3{
fld0: f64,
fld1: Adt42,
fld2: i128,
fld3: i8,
fld4: i16,
fld5: i32,
fld6: i64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: [isize; 6],
fld1: *mut u16,
fld2: *mut usize,
fld3: [u8; 7],
fld4: Adt50,
fld5: u8,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: (i16, u64, i16, u64, bool, char),
fld1: *const f64,

},
Variant1{
fld0: Adt44,
fld1: u16,
fld2: i128,
fld3: *mut usize,

},
Variant2{
fld0: *mut [u8; 7],

},
Variant3{
fld0: (u64, i32, usize),

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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u16; 7],
fld1: char,
fld2: *mut *const f64,
fld3: u16,
fld4: (u16,),
fld5: i32,
fld6: Adt52,
fld7: [u64; 6],

},
Variant1{
fld0: (i64, i32, f64),
fld1: u64,
fld2: Adt43,
fld3: Adt50,
fld4: Adt53,

},
Variant2{
fld0: bool,
fld1: Adt52,
fld2: Adt51,
fld3: (u64, i32, usize),
fld4: usize,
fld5: (u16, i64, u16),

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt47,
fld1: Adt54,
fld2: [u64; 6],
fld3: (u64, i32, usize),
fld4: i16,
fld5: Adt41,
fld6: u8,
fld7: i128,

},
Variant1{
fld0: Adt54,
fld1: *mut [u8; 7],
fld2: Adt48,
fld3: [u8; 7],
fld4: [u16; 2],
fld5: i32,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: u128,
fld1: char,
fld2: u8,
fld3: *mut i64,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (i16, u64, i16, u64, bool, char),
fld1: *mut u16,

},
Variant1{
fld0: [u64; 6],

},
Variant2{
fld0: Adt48,
fld1: u8,

}}

