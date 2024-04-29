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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u64) -> u128 {
mir! {
type RET = u128;
let _13: isize;
let _14: u32;
let _15: Adt56;
let _16: (isize,);
let _17: ([i64; 6], u8);
let _18: (char, i64, i64, &'static u16, [usize; 5]);
let _19: ((bool, usize), u32, isize, ((isize,), i16));
let _20: [i64; 6];
let _21: [usize; 5];
let _22: isize;
let _23: usize;
let _24: [u64; 6];
let _25: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16);
let _26: u128;
let _27: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16);
let _28: char;
let _29: i64;
let _30: bool;
let _31: [isize; 7];
let _32: bool;
let _33: isize;
let _34: isize;
let _35: [u64; 6];
let _36: f64;
let _37: u128;
let _38: [isize; 7];
let _39: bool;
let _40: ([i64; 6], u8);
let _41: Adt58;
let _42: ((bool, usize), u32, isize, ((isize,), i16));
let _43: ();
let _44: ();
{
_2 = '\u{dddab}';
_3 = 9223372036854775807_isize;
RET = 21272_u16 as u128;
Goto(bb1)
}
bb1 = {
_6 = -(-1725733469_i32);
_3 = !123_isize;
_8 = _3 as i128;
_7 = 245_u8 as i64;
_14 = (-13418_i16) as u32;
_4 = RET as i8;
_6 = 1918187311_i32 * 1161559676_i32;
_15.fld0 = _14 as f64;
_10 = 121_u8;
_5 = (-3431_i16) << _14;
_6 = 630449573_i32 & 1973773112_i32;
_1 = true;
_9 = 17930539699454732209_usize;
_14 = 1903625607_u32;
_2 = '\u{be691}';
_12 = 10773435723002913041_u64;
match _12 {
10773435723002913041 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_4 = _2 as i8;
_7 = (-1854422612139262953_i64);
_19.0.0 = _1;
_18.2 = _7;
_17.0 = [_7,_7,_18.2,_18.2,_7,_7];
_18.1 = _7 << _9;
_18.2 = _18.1 & _7;
_19.1 = !_14;
_19.3.0.0 = !_3;
RET = 288546547380531443613375314933625882891_u128;
_16 = (_19.3.0.0,);
_19.3.0.0 = _18.2 as isize;
_15.fld0 = _10 as f64;
_12 = 12079559863678995294_u64;
_19.0 = (_1, _9);
_18.0 = _2;
_19.3.0.0 = _3;
_17.1 = _8 as u8;
Goto(bb4)
}
bb4 = {
_4 = !(-3_i8);
_17.1 = _10 + _10;
_18.2 = !_18.1;
_1 = _19.0.0 | _19.0.0;
_18.0 = _2;
_21 = [_19.0.1,_9,_19.0.1,_9,_9];
_19.3.1 = !_5;
_14 = _19.1 << _17.1;
_18.4 = [_9,_9,_9,_9,_19.0.1];
_19.3.0.0 = _12 as isize;
_18.3 = &_11;
_15.fld0 = _3 as f64;
Goto(bb5)
}
bb5 = {
_15.fld1 = core::ptr::addr_of!(_19.0.0);
_19.3 = (_16, _5);
_19.0.0 = _1 & _1;
_25.1.0 = core::ptr::addr_of_mut!(_6);
_19.3.0.0 = -_16.0;
_19.0 = (_1, _9);
RET = !188066498028174561080990179883428742078_u128;
_27.0 = _16;
_26 = !RET;
_27.1.0 = core::ptr::addr_of_mut!(_6);
_23 = _9 % _19.0.1;
_29 = _12 as i64;
_2 = _18.0;
_11 = _15.fld0 as u16;
_27.1.1 = core::ptr::addr_of_mut!(_25.1.0);
_19.2 = _8 as isize;
_25.1 = (_27.1.0, _27.1.1, RET, _15.fld1);
_19.3.0.0 = _6 as isize;
_27.1 = (_25.1.0, _25.1.1, _25.1.2, _15.fld1);
Goto(bb6)
}
bb6 = {
_27.1.0 = _25.1.0;
_25.0 = (_19.2,);
_23 = _19.0.1 >> _19.3.1;
_25.1.1 = _27.1.1;
_27.1.1 = _25.1.1;
_27.0.0 = _19.3.0.0;
_27.0.0 = _25.0.0;
_18.1 = _29;
_19.0 = (_1, _23);
_15.fld0 = _27.0.0 as f64;
_27 = (_19.3.0, _25.1, _19.3.1);
_25.2 = !_27.2;
_27.1.2 = RET & _26;
_27.2 = _18.1 as i16;
_8 = _27.2 as i128;
_13 = _15.fld0 as isize;
_16 = _19.3.0;
_27.1.3 = _15.fld1;
RET = _26;
_18.0 = _2;
_3 = _16.0;
_19.3.0 = (_13,);
_11 = 46829_u16;
_19.0 = (_1, _23);
_27.0.0 = -_3;
_30 = !_19.0.0;
_1 = !_19.0.0;
_9 = _23;
Call(_27.1.2 = fn1(_16.0, _12, _25.1.0, _14, _25.2, _27.0.0, _17.1, _18.4, _19.3.1, _12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_19.0.1 = _9;
_6 = !(-734382303_i32);
_27.1.3 = _15.fld1;
_27 = _25;
_27.0.0 = _19.3.0.0 * _16.0;
_27.1.1 = _25.1.1;
_18.4 = _21;
_15.fld0 = _17.1 as f64;
match _10 {
0 => bb5,
1 => bb6,
121 => bb9,
_ => bb8
}
}
bb8 = {
_4 = _2 as i8;
_7 = (-1854422612139262953_i64);
_19.0.0 = _1;
_18.2 = _7;
_17.0 = [_7,_7,_18.2,_18.2,_7,_7];
_18.1 = _7 << _9;
_18.2 = _18.1 & _7;
_19.1 = !_14;
_19.3.0.0 = !_3;
RET = 288546547380531443613375314933625882891_u128;
_16 = (_19.3.0.0,);
_19.3.0.0 = _18.2 as isize;
_15.fld0 = _10 as f64;
_12 = 12079559863678995294_u64;
_19.0 = (_1, _9);
_18.0 = _2;
_19.3.0.0 = _3;
_17.1 = _8 as u8;
Goto(bb4)
}
bb9 = {
_33 = _15.fld0 as isize;
_10 = _8 as u8;
_18.2 = !_18.1;
_20 = _17.0;
_22 = _27.0.0;
_5 = !_25.2;
_33 = _19.3.0.0;
_25.1.1 = core::ptr::addr_of_mut!(_27.1.0);
_32 = !_19.0.0;
_32 = !_19.0.0;
_19.3.0 = (_13,);
_25.1.3 = core::ptr::addr_of!(_1);
_38 = [_25.0.0,_27.0.0,_3,_33,_33,_33,_13];
_25.1.1 = _27.1.1;
_25.2 = _5;
_37 = _26 >> _25.1.2;
_26 = _25.1.2;
_25.0.0 = _3 ^ _33;
_25.0 = (_22,);
_40 = _17;
_4 = -(-49_i8);
_39 = _19.0.0 & _1;
Call(_30 = fn6(_16, _4, _11, _19.3, _27.1.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_31 = _38;
_19.0.1 = !_9;
_18.4 = _21;
_42.0 = (_19.0.0, _9);
_35 = [_12,_12,_12,_12,_12,_12];
_17 = _40;
_19.3.0 = (_25.0.0,);
match _12 {
0 => bb3,
1 => bb4,
2 => bb11,
3 => bb12,
4 => bb13,
12079559863678995294 => bb15,
_ => bb14
}
}
bb11 = {
_33 = _15.fld0 as isize;
_10 = _8 as u8;
_18.2 = !_18.1;
_20 = _17.0;
_22 = _27.0.0;
_5 = !_25.2;
_33 = _19.3.0.0;
_25.1.1 = core::ptr::addr_of_mut!(_27.1.0);
_32 = !_19.0.0;
_32 = !_19.0.0;
_19.3.0 = (_13,);
_25.1.3 = core::ptr::addr_of!(_1);
_38 = [_25.0.0,_27.0.0,_3,_33,_33,_33,_13];
_25.1.1 = _27.1.1;
_25.2 = _5;
_37 = _26 >> _25.1.2;
_26 = _25.1.2;
_25.0.0 = _3 ^ _33;
_25.0 = (_22,);
_40 = _17;
_4 = -(-49_i8);
_39 = _19.0.0 & _1;
Call(_30 = fn6(_16, _4, _11, _19.3, _27.1.3), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_4 = _2 as i8;
_7 = (-1854422612139262953_i64);
_19.0.0 = _1;
_18.2 = _7;
_17.0 = [_7,_7,_18.2,_18.2,_7,_7];
_18.1 = _7 << _9;
_18.2 = _18.1 & _7;
_19.1 = !_14;
_19.3.0.0 = !_3;
RET = 288546547380531443613375314933625882891_u128;
_16 = (_19.3.0.0,);
_19.3.0.0 = _18.2 as isize;
_15.fld0 = _10 as f64;
_12 = 12079559863678995294_u64;
_19.0 = (_1, _9);
_18.0 = _2;
_19.3.0.0 = _3;
_17.1 = _8 as u8;
Goto(bb4)
}
bb13 = {
_19.0.1 = _9;
_6 = !(-734382303_i32);
_27.1.3 = _15.fld1;
_27 = _25;
_27.0.0 = _19.3.0.0 * _16.0;
_27.1.1 = _25.1.1;
_18.4 = _21;
_15.fld0 = _17.1 as f64;
match _10 {
0 => bb5,
1 => bb6,
121 => bb9,
_ => bb8
}
}
bb14 = {
_4 = !(-3_i8);
_17.1 = _10 + _10;
_18.2 = !_18.1;
_1 = _19.0.0 | _19.0.0;
_18.0 = _2;
_21 = [_19.0.1,_9,_19.0.1,_9,_9];
_19.3.1 = !_5;
_14 = _19.1 << _17.1;
_18.4 = [_9,_9,_9,_9,_19.0.1];
_19.3.0.0 = _12 as isize;
_18.3 = &_11;
_15.fld0 = _3 as f64;
Goto(bb5)
}
bb15 = {
_29 = _18.1;
_7 = _18.1 + _29;
_36 = -_15.fld0;
_42.3.0 = (_19.3.0.0,);
_35 = [_12,_12,_12,_12,_12,_12];
_42.0.0 = _19.0.0;
_30 = _32;
Goto(bb16)
}
bb16 = {
Call(_43 = dump_var(0_usize, 10_usize, Move(_10), 19_usize, Move(_19), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(0_usize, 7_usize, Move(_7), 20_usize, Move(_20), 35_usize, Move(_35), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(0_usize, 38_usize, Move(_38), 13_usize, Move(_13), 16_usize, Move(_16), 31_usize, Move(_31)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_43 = dump_var(0_usize, 8_usize, Move(_8), 40_usize, Move(_40), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: u64,mut _3: *mut i32,mut _4: u32,mut _5: i16,mut _6: isize,mut _7: u8,mut _8: [usize; 5],mut _9: i16,mut _10: u64) -> u128 {
mir! {
type RET = u128;
let _11: (f64,);
let _12: isize;
let _13: (char, i64, i64, &'static u16, [usize; 5]);
let _14: (isize,);
let _15: (f64,);
let _16: [usize; 5];
let _17: bool;
let _18: Adt58;
let _19: char;
let _20: (isize,);
let _21: Adt47;
let _22: char;
let _23: (bool, usize);
let _24: *mut u64;
let _25: Adt53;
let _26: [isize; 7];
let _27: isize;
let _28: [i64; 6];
let _29: ();
let _30: ();
{
_2 = _10;
RET = _4 as u128;
_6 = -_1;
_1 = !_6;
_3 = core::ptr::addr_of_mut!((*_3));
RET = 255358157942451228259533876924352582742_u128;
_4 = !3981253283_u32;
(*_3) = true as i32;
_4 = 1541298362_u32;
RET = 39332610415211997914440965054560619757_u128;
_13.2 = 5144052295284700342_i64 - 8137969796978758003_i64;
_9 = _13.2 as i16;
_13.1 = _13.2;
Goto(bb1)
}
bb1 = {
_2 = !_10;
RET = !706153420560835716047778661168363312_u128;
_13.0 = '\u{97fe6}';
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1541298362 => bb9,
_ => bb8
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
_14 = (_1,);
_14.0 = _1;
_13.1 = (-88208050561450572874233357240056914328_i128) as i64;
(*_3) = (-1670850344_i32) ^ 918250970_i32;
_15.0 = _4 as f64;
_16 = [7917376498488726686_usize,6_usize,4_usize,4_usize,10068890210831244233_usize];
_13.2 = _13.1 + _13.1;
_8 = _16;
Call(_9 = fn2(_14.0, _1, _3, _14, _3, _6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_14.0 = (*_3) as isize;
_13.4 = _16;
_4 = !3808792190_u32;
RET = !216845666591898507305996873248140265796_u128;
_8 = [7_usize,2_usize,17640654682714578159_usize,1_usize,13944508089880079438_usize];
_17 = !true;
_20 = _14;
_1 = _2 as isize;
_11 = (_15.0,);
_20 = (_6,);
_15.0 = _11.0 - _11.0;
_12 = _1 ^ _1;
_14 = (_12,);
RET = _4 as u128;
_16 = _13.4;
Call((*_3) = fn3(RET, _16, _20.0, _7, _13.4, _8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_13.2 = 53009_u16 as i64;
_20.0 = -_12;
Goto(bb12)
}
bb12 = {
_11.0 = -_15.0;
_7 = 191_u8;
_11.0 = 13460_u16 as f64;
_12 = -_14.0;
_9 = _4 as i16;
_11.0 = _15.0 - _15.0;
_15.0 = _11.0 - _11.0;
(*_3) = _11.0 as i32;
_19 = _13.0;
RET = _17 as u128;
_6 = _1;
_9 = -_5;
_10 = !_2;
_22 = _19;
_4 = (*_3) as u32;
RET = 224495472299069138021972710833600554791_u128;
_22 = _19;
_13.2 = _14.0 as i64;
_11 = (_15.0,);
_9 = !_5;
_10 = _2;
_15 = (_11.0,);
_2 = _10 << _20.0;
_13.1 = !_13.2;
Goto(bb13)
}
bb13 = {
_10 = !_2;
_7 = 95_u8 * 213_u8;
_14.0 = _1;
_13.4 = [5_usize,7_usize,3_usize,4301057569327562557_usize,6_usize];
_14.0 = _6;
_13.4 = [11930970968081750377_usize,0_usize,12949469900542383234_usize,6_usize,6_usize];
_23.0 = _17 | _17;
_13.1 = _13.2;
_22 = _13.0;
_7 = !208_u8;
_23.1 = _22 as usize;
_23.0 = _11.0 > _15.0;
_10 = _4 as u64;
_12 = _2 as isize;
_14.0 = _19 as isize;
RET = 59147183534113263133569175344849104161_u128 | 272681148865997315839122481776693823843_u128;
_5 = !_9;
_15 = _11;
(*_3) = 829296312_i32;
_25.fld0.2 = 70_i8 as u32;
Call(_21 = fn4(_14.0, _12, _23.0, _15, _23.0, _9, _2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_25.fld2.0.0 = _14.0;
_25.fld1 = (_11.0,);
_10 = _2;
_20 = (_12,);
_25.fld0.0 = -_15.0;
_25.fld1.0 = -_11.0;
_13.2 = _13.1;
_8 = [_23.1,_23.1,_23.1,_23.1,_23.1];
_26 = [_6,_20.0,_20.0,_12,_12,_12,_20.0];
_23.0 = _17;
_23.0 = !_17;
_11.0 = _7 as f64;
_25.fld0.2 = _4;
_19 = _13.0;
_25.fld2.2 = _9;
_10 = _2;
_9 = -_25.fld2.2;
place!(Field::<(f64,)>(Variant(_21, 2), 0)).0 = _15.0 - _25.fld1.0;
_25.fld2.1.3 = core::ptr::addr_of!(_17);
_1 = 37_i8 as isize;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(1_usize, 2_usize, Move(_2), 10_usize, Move(_10), 26_usize, Move(_26), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(1_usize, 8_usize, Move(_8), 9_usize, Move(_9), 23_usize, Move(_23), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(1_usize, 16_usize, Move(_16), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: *mut i32,mut _4: (isize,),mut _5: *mut i32,mut _6: isize) -> i16 {
mir! {
type RET = i16;
let _7: [char; 2];
let _8: (f64,);
let _9: char;
let _10: Adt47;
let _11: Adt53;
let _12: *mut i32;
let _13: i8;
let _14: i64;
let _15: (isize,);
let _16: ((bool, usize), u32, isize, ((isize,), i16));
let _17: (bool, usize);
let _18: bool;
let _19: (f64,);
let _20: (f64,);
let _21: i32;
let _22: f64;
let _23: [char; 2];
let _24: Adt48;
let _25: bool;
let _26: [usize; 5];
let _27: i128;
let _28: &'static u16;
let _29: ();
let _30: ();
{
RET = (-25235_i16);
(*_5) = !(-1537858872_i32);
(*_3) = !(-1915800247_i32);
RET = _4.0 as i16;
_4 = (_6,);
RET = 2337130001310142472_u64 as i16;
_6 = _2 - _2;
_6 = -_4.0;
_1 = _2 * _4.0;
(*_3) = 61_i8 as i32;
(*_5) = (-1480293574_i32) >> _1;
_5 = _3;
_8.0 = 61545_u16 as f64;
_7 = ['\u{b7f1f}','\u{a06e4}'];
_11.fld0.0 = 162154372881964426285802770627422521020_i128 as f64;
_11.fld2.0 = (_1,);
_5 = core::ptr::addr_of_mut!((*_5));
_11.fld0.2 = !1246410712_u32;
_7 = ['\u{a8539}','\u{10cb2e}'];
_11.fld2.1.1 = core::ptr::addr_of_mut!(_11.fld2.1.0);
_11.fld2.2 = 4_usize as i16;
_12 = _3;
_14 = !(-2196689694160320979_i64);
Call((*_5) = core::intrinsics::transmute(_11.fld0.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.0 = -_11.fld2.0.0;
RET = _11.fld2.2;
(*_12) = _4.0 as i32;
Goto(bb2)
}
bb2 = {
_11.fld2.1.0 = _3;
_13 = !(-42_i8);
_15 = _4;
_11.fld2.1.2 = 151379779494611824451115002191580626006_u128;
_5 = _11.fld2.1.0;
_4.0 = 53678_u16 as isize;
_11.fld0 = (_8.0, _7, 718033227_u32);
Goto(bb3)
}
bb3 = {
_11.fld0 = (_8.0, _7, 185996185_u32);
_9 = '\u{a3a72}';
RET = _13 as i16;
_11.fld2.1.2 = 191980771575394483279078626889506521846_u128 & 19119046302081769539471073331007305059_u128;
_11.fld2.0.0 = -_4.0;
_11.fld1 = _8;
(*_12) = !684868026_i32;
_11.fld0.0 = -_8.0;
_16.0.1 = !11790958933571757869_usize;
_11.fld0.1 = [_9,_9];
Goto(bb4)
}
bb4 = {
_16.3.0.0 = !_6;
_11.fld0 = (_11.fld1.0, _7, 206934251_u32);
_14 = 2228894831651357135_i64 | (-7538657203599471059_i64);
_7 = [_9,_9];
_4 = (_15.0,);
_11.fld2.1.0 = core::ptr::addr_of_mut!((*_5));
(*_5) = !2077219973_i32;
_16.2 = _4.0 | _4.0;
_13 = _11.fld0.2 as i8;
(*_3) = 354008564_i32 & 111747110_i32;
_1 = !_11.fld2.0.0;
_11.fld2.1.3 = core::ptr::addr_of!(_16.0.0);
_15 = _4;
_10 = Adt47::Variant2 { fld0: _11.fld1,fld1: _11.fld1.0 };
_4 = (_6,);
_8.0 = _13 as f64;
_11.fld0.2 = _16.2 as u32;
_11.fld2.1.1 = core::ptr::addr_of_mut!(_5);
_16.3 = (_15, _11.fld2.2);
RET = _16.3.1;
SetDiscriminant(_10, 0);
(*_5) = (-1627062110_i32);
_17.1 = _16.0.1 & _16.0.1;
_11.fld2.1.2 = 1472_u16 as u128;
_11.fld1 = (_11.fld0.0,);
place!(Field::<bool>(Variant(_10, 0), 0)) = false;
place!(Field::<u16>(Variant(_10, 0), 1)) = 9835_u16 | 53995_u16;
_6 = _16.3.0.0 << (*_12);
_9 = '\u{53a11}';
Goto(bb5)
}
bb5 = {
_16.0.0 = Field::<bool>(Variant(_10, 0), 0);
_8.0 = _11.fld0.0 + _11.fld1.0;
_11.fld2.1.0 = _5;
_11.fld2.1.2 = 299077174372531364668977905566661881202_u128;
_11.fld2.0 = (_15.0,);
_8 = _11.fld1;
_17 = (_16.0.0, _16.0.1);
(*_12) = 2044724132_i32;
place!(Field::<bool>(Variant(_10, 0), 0)) = !_17.0;
_17 = (_16.0.0, _16.0.1);
_16.3 = (_11.fld2.0, RET);
_11.fld2.2 = (*_5) as i16;
match (*_3) {
0 => bb1,
1 => bb4,
2 => bb3,
2044724132 => bb7,
_ => bb6
}
}
bb6 = {
_16.3.0.0 = !_6;
_11.fld0 = (_11.fld1.0, _7, 206934251_u32);
_14 = 2228894831651357135_i64 | (-7538657203599471059_i64);
_7 = [_9,_9];
_4 = (_15.0,);
_11.fld2.1.0 = core::ptr::addr_of_mut!((*_5));
(*_5) = !2077219973_i32;
_16.2 = _4.0 | _4.0;
_13 = _11.fld0.2 as i8;
(*_3) = 354008564_i32 & 111747110_i32;
_1 = !_11.fld2.0.0;
_11.fld2.1.3 = core::ptr::addr_of!(_16.0.0);
_15 = _4;
_10 = Adt47::Variant2 { fld0: _11.fld1,fld1: _11.fld1.0 };
_4 = (_6,);
_8.0 = _13 as f64;
_11.fld0.2 = _16.2 as u32;
_11.fld2.1.1 = core::ptr::addr_of_mut!(_5);
_16.3 = (_15, _11.fld2.2);
RET = _16.3.1;
SetDiscriminant(_10, 0);
(*_5) = (-1627062110_i32);
_17.1 = _16.0.1 & _16.0.1;
_11.fld2.1.2 = 1472_u16 as u128;
_11.fld1 = (_11.fld0.0,);
place!(Field::<bool>(Variant(_10, 0), 0)) = false;
place!(Field::<u16>(Variant(_10, 0), 1)) = 9835_u16 | 53995_u16;
_6 = _16.3.0.0 << (*_12);
_9 = '\u{53a11}';
Goto(bb5)
}
bb7 = {
_11.fld0 = (_8.0, _7, 173365379_u32);
_16.0.0 = !_17.0;
(*_5) = (-2053405028_i32) - (-562708338_i32);
(*_12) = (-96740244_i32);
_11.fld1.0 = _11.fld0.0;
(*_12) = !(-673794119_i32);
_3 = _5;
_10 = Adt47::Variant2 { fld0: _11.fld1,fld1: _8.0 };
_16.3.0.0 = (*_3) as isize;
_9 = '\u{131f8}';
_11.fld2.1.2 = (*_5) as u128;
_11.fld2.1.1 = core::ptr::addr_of_mut!(_12);
SetDiscriminant(_10, 1);
_9 = '\u{c9ffd}';
place!(Field::<((bool, usize), u32, isize, ((isize,), i16))>(Variant(_10, 1), 6)).0.1 = _17.1;
place!(Field::<([i64; 6], u8)>(Variant(_10, 1), 2)).0 = [_14,_14,_14,_14,_14,_14];
_16.2 = !_11.fld2.0.0;
place!(Field::<([i64; 6], u8)>(Variant(_10, 1), 2)).0 = [_14,_14,_14,_14,_14,_14];
_11.fld0.1 = [_9,_9];
(*_12) = !(-749740548_i32);
_12 = _5;
_20.0 = -_11.fld0.0;
place!(Field::<(f64, [char; 2], u32)>(Variant(_10, 1), 4)) = (_11.fld1.0, _7, _11.fld0.2);
match Field::<(f64, [char; 2], u32)>(Variant(_10, 1), 4).2 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
173365379 => bb14,
_ => bb13
}
}
bb8 = {
_16.3.0.0 = !_6;
_11.fld0 = (_11.fld1.0, _7, 206934251_u32);
_14 = 2228894831651357135_i64 | (-7538657203599471059_i64);
_7 = [_9,_9];
_4 = (_15.0,);
_11.fld2.1.0 = core::ptr::addr_of_mut!((*_5));
(*_5) = !2077219973_i32;
_16.2 = _4.0 | _4.0;
_13 = _11.fld0.2 as i8;
(*_3) = 354008564_i32 & 111747110_i32;
_1 = !_11.fld2.0.0;
_11.fld2.1.3 = core::ptr::addr_of!(_16.0.0);
_15 = _4;
_10 = Adt47::Variant2 { fld0: _11.fld1,fld1: _11.fld1.0 };
_4 = (_6,);
_8.0 = _13 as f64;
_11.fld0.2 = _16.2 as u32;
_11.fld2.1.1 = core::ptr::addr_of_mut!(_5);
_16.3 = (_15, _11.fld2.2);
RET = _16.3.1;
SetDiscriminant(_10, 0);
(*_5) = (-1627062110_i32);
_17.1 = _16.0.1 & _16.0.1;
_11.fld2.1.2 = 1472_u16 as u128;
_11.fld1 = (_11.fld0.0,);
place!(Field::<bool>(Variant(_10, 0), 0)) = false;
place!(Field::<u16>(Variant(_10, 0), 1)) = 9835_u16 | 53995_u16;
_6 = _16.3.0.0 << (*_12);
_9 = '\u{53a11}';
Goto(bb5)
}
bb9 = {
_16.0.0 = Field::<bool>(Variant(_10, 0), 0);
_8.0 = _11.fld0.0 + _11.fld1.0;
_11.fld2.1.0 = _5;
_11.fld2.1.2 = 299077174372531364668977905566661881202_u128;
_11.fld2.0 = (_15.0,);
_8 = _11.fld1;
_17 = (_16.0.0, _16.0.1);
(*_12) = 2044724132_i32;
place!(Field::<bool>(Variant(_10, 0), 0)) = !_17.0;
_17 = (_16.0.0, _16.0.1);
_16.3 = (_11.fld2.0, RET);
_11.fld2.2 = (*_5) as i16;
match (*_3) {
0 => bb1,
1 => bb4,
2 => bb3,
2044724132 => bb7,
_ => bb6
}
}
bb10 = {
_16.3.0.0 = !_6;
_11.fld0 = (_11.fld1.0, _7, 206934251_u32);
_14 = 2228894831651357135_i64 | (-7538657203599471059_i64);
_7 = [_9,_9];
_4 = (_15.0,);
_11.fld2.1.0 = core::ptr::addr_of_mut!((*_5));
(*_5) = !2077219973_i32;
_16.2 = _4.0 | _4.0;
_13 = _11.fld0.2 as i8;
(*_3) = 354008564_i32 & 111747110_i32;
_1 = !_11.fld2.0.0;
_11.fld2.1.3 = core::ptr::addr_of!(_16.0.0);
_15 = _4;
_10 = Adt47::Variant2 { fld0: _11.fld1,fld1: _11.fld1.0 };
_4 = (_6,);
_8.0 = _13 as f64;
_11.fld0.2 = _16.2 as u32;
_11.fld2.1.1 = core::ptr::addr_of_mut!(_5);
_16.3 = (_15, _11.fld2.2);
RET = _16.3.1;
SetDiscriminant(_10, 0);
(*_5) = (-1627062110_i32);
_17.1 = _16.0.1 & _16.0.1;
_11.fld2.1.2 = 1472_u16 as u128;
_11.fld1 = (_11.fld0.0,);
place!(Field::<bool>(Variant(_10, 0), 0)) = false;
place!(Field::<u16>(Variant(_10, 0), 1)) = 9835_u16 | 53995_u16;
_6 = _16.3.0.0 << (*_12);
_9 = '\u{53a11}';
Goto(bb5)
}
bb11 = {
_11.fld0 = (_8.0, _7, 185996185_u32);
_9 = '\u{a3a72}';
RET = _13 as i16;
_11.fld2.1.2 = 191980771575394483279078626889506521846_u128 & 19119046302081769539471073331007305059_u128;
_11.fld2.0.0 = -_4.0;
_11.fld1 = _8;
(*_12) = !684868026_i32;
_11.fld0.0 = -_8.0;
_16.0.1 = !11790958933571757869_usize;
_11.fld0.1 = [_9,_9];
Goto(bb4)
}
bb12 = {
_11.fld2.1.0 = _3;
_13 = !(-42_i8);
_15 = _4;
_11.fld2.1.2 = 151379779494611824451115002191580626006_u128;
_5 = _11.fld2.1.0;
_4.0 = 53678_u16 as isize;
_11.fld0 = (_8.0, _7, 718033227_u32);
Goto(bb3)
}
bb13 = {
_4.0 = -_11.fld2.0.0;
RET = _11.fld2.2;
(*_12) = _4.0 as i32;
Goto(bb2)
}
bb14 = {
_8 = (_11.fld0.0,);
_25 = !_17.0;
_16.0.1 = Field::<((bool, usize), u32, isize, ((isize,), i16))>(Variant(_10, 1), 6).0.1 | Field::<((bool, usize), u32, isize, ((isize,), i16))>(Variant(_10, 1), 6).0.1;
place!(Field::<*mut i32>(Variant(_10, 1), 3)) = _12;
_16.3.1 = 13864553057126386145_u64 as i16;
place!(Field::<((bool, usize), u32, isize, ((isize,), i16))>(Variant(_10, 1), 6)) = (_17, _11.fld0.2, _15.0, _16.3);
place!(Field::<f64>(Variant(_10, 1), 0)) = -_8.0;
_15.0 = _16.2;
place!(Field::<f64>(Variant(_10, 1), 0)) = _8.0 + _11.fld0.0;
(*_5) = 1611158600_i32;
_11.fld2.1.0 = _3;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 6_usize, Move(_6), 25_usize, Move(_25), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 2_usize, Move(_2), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u128,mut _2: [usize; 5],mut _3: isize,mut _4: u8,mut _5: [usize; 5],mut _6: [usize; 5]) -> i32 {
mir! {
type RET = i32;
let _7: (f64,);
let _8: (f64, [char; 2], u32);
let _9: ((f64,),);
let _10: Adt56;
let _11: f64;
let _12: (f64,);
let _13: i8;
let _14: f64;
let _15: f32;
let _16: (bool, usize);
let _17: Adt57;
let _18: *mut u64;
let _19: isize;
let _20: ();
let _21: ();
{
_1 = !313219127304380996633515200662623575389_u128;
RET = 29699_i16 as i32;
_1 = 41617136391979405797058057427539295919_u128 - 246633367646300893090122135042682991712_u128;
_5 = [11637048876920165386_usize,1_usize,6_usize,18081294001102214038_usize,6_usize];
_5 = [6_usize,4069732506123560250_usize,2648672086326274714_usize,1_usize,3_usize];
_1 = 333148264479630820143833449503725866426_u128;
_5 = [5_usize,0_usize,0_usize,4_usize,17220526693839787415_usize];
_8.1 = ['\u{2bf8a}','\u{76d98}'];
_8.2 = 1461939276_u32 | 1334042405_u32;
_5 = [4_usize,5_usize,1_usize,1261473474534691044_usize,6823414148051314458_usize];
_10.fld0 = 52842_u16 as f64;
_9.0.0 = -_10.fld0;
_5 = [7_usize,3_usize,0_usize,5_usize,2135694324498457781_usize];
RET = !1224105093_i32;
_8.0 = _10.fld0 * _9.0.0;
_8.0 = -_9.0.0;
_11 = 10116_i16 as f64;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
333148264479630820143833449503725866426 => bb8,
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
_8.1 = ['\u{c1ea2}','\u{58420}'];
_12.0 = -_8.0;
_10.fld0 = _8.2 as f64;
_12.0 = 10196320883699156863_u64 as f64;
_12 = _9.0;
_7.0 = _8.0 - _10.fld0;
_1 = (-6657827016482780065_i64) as u128;
_12 = (_10.fld0,);
_10.fld0 = _7.0 - _7.0;
_12.0 = _10.fld0;
_7 = (_12.0,);
_12 = _7;
_10.fld0 = RET as f64;
_10.fld0 = _12.0;
_6 = _5;
_11 = _7.0 * _7.0;
_7.0 = -_10.fld0;
_4 = 163_u8;
_11 = _12.0;
RET = (-1417501391_i32) & (-1688131999_i32);
RET = 1060702046_i32;
RET = -(-1604441801_i32);
_4 = 80_u8 * 150_u8;
_9.0 = _12;
_1 = 267222740480605336705656196470611005714_u128 + 159641723213407233862179108645437098312_u128;
_8.2 = 1_usize as u32;
_12 = (_7.0,);
_5 = [0_usize,8303113261526439320_usize,2_usize,15609844758878365978_usize,3223907808595937143_usize];
_4 = 149_u8;
RET = !(-613342484_i32);
match _4 {
0 => bb9,
1 => bb10,
149 => bb12,
_ => bb11
}
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
_9.0 = (_10.fld0,);
_10.fld0 = RET as f64;
Goto(bb13)
}
bb13 = {
RET = !(-641471831_i32);
_3 = -9223372036854775807_isize;
_7 = (_11,);
_10.fld0 = _7.0;
_15 = (-4_i8) as f32;
_16.0 = true;
_4 = 192_u8;
match _4 {
0 => bb12,
1 => bb14,
2 => bb15,
3 => bb16,
192 => bb18,
_ => bb17
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
Return()
}
bb18 = {
_12 = (_10.fld0,);
_6 = _5;
_8.2 = 20802180_u32;
_14 = -_12.0;
_11 = (-79_i8) as f64;
_9 = (_7,);
_16 = (true, 7_usize);
_19 = !_3;
_15 = RET as f32;
_8.0 = _8.2 as f64;
_16.0 = true ^ true;
_14 = _12.0;
Goto(bb19)
}
bb19 = {
Call(_20 = dump_var(3_usize, 3_usize, Move(_3), 5_usize, Move(_5), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: (f64,),mut _5: bool,mut _6: i16,mut _7: u64) -> Adt47 {
mir! {
type RET = Adt47;
let _8: f64;
let _9: (f64,);
let _10: u128;
let _11: i32;
let _12: (char, i64, i64, &'static u16, [usize; 5]);
let _13: ((bool, usize), u32, isize, ((isize,), i16));
let _14: i8;
let _15: [usize; 8];
let _16: Adt45;
let _17: usize;
let _18: *mut f32;
let _19: ((f64,),);
let _20: *mut *const u8;
let _21: *const u8;
let _22: [char; 2];
let _23: isize;
let _24: *mut i32;
let _25: ();
let _26: ();
{
_7 = '\u{d4e0}' as u64;
_1 = (-1869480559193444052_i64) as isize;
_1 = _2 >> _2;
_4.0 = (-85386404949560659363558635486211624256_i128) as f64;
_7 = !14665708257115606206_u64;
_8 = -_4.0;
_6 = !(-13522_i16);
RET = Adt47::Variant2 { fld0: _4,fld1: _4.0 };
place!(Field::<f64>(Variant(RET, 2), 1)) = Field::<(f64,)>(Variant(RET, 2), 0).0 * Field::<(f64,)>(Variant(RET, 2), 0).0;
place!(Field::<(f64,)>(Variant(RET, 2), 0)).0 = 88391934781968542553549822385825310812_i128 as f64;
RET = Adt47::Variant2 { fld0: _4,fld1: _4.0 };
_2 = _5 as isize;
_5 = _3;
_2 = _1 - _1;
place!(Field::<(f64,)>(Variant(RET, 2), 0)) = _4;
_1 = -_2;
_8 = _4.0 * Field::<(f64,)>(Variant(RET, 2), 0).0;
_9 = (_4.0,);
_3 = _2 > _1;
place!(Field::<(f64,)>(Variant(RET, 2), 0)).0 = -_9.0;
Call(_4.0 = fn5(Move(RET), _2, _3, _3, _1, _3, _5, _5, _1, _3, _3, _2, _5, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = 17162868331749009189_u64 ^ 12876565218267242978_u64;
_5 = !_3;
_7 = 3411063367816836879_u64 ^ 2008915876293564196_u64;
RET = Adt47::Variant2 { fld0: _4,fld1: _9.0 };
_12.4 = [17626806192738493047_usize,4132407852512690816_usize,5_usize,3_usize,2081421705625313415_usize];
place!(Field::<(f64,)>(Variant(RET, 2), 0)) = _4;
place!(Field::<f64>(Variant(RET, 2), 1)) = -Field::<(f64,)>(Variant(RET, 2), 0).0;
_13.2 = _2;
SetDiscriminant(RET, 3);
_13.3.1 = (-1665016706_i32) as i16;
Goto(bb2)
}
bb2 = {
_13.3.0.0 = _13.2;
_4.0 = -_8;
_5 = _3 & _3;
_9.0 = _8 - _8;
_13.0 = (_3, 2_usize);
place!(Field::<i128>(Variant(RET, 3), 3)) = _13.0.1 as i128;
_12.4 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.2 = 311112060471981338_i64 as isize;
_4.0 = _9.0;
_12.1 = 1440409721730133565_i64 & (-7642081412443519594_i64);
_13.0.0 = !_3;
_10 = 21615500519123823221312819784554500408_u128 * 164478648422208381473916299256713934099_u128;
place!(Field::<[usize; 5]>(Variant(RET, 3), 0)) = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
Goto(bb3)
}
bb3 = {
_13.0.0 = _2 >= _13.3.0.0;
_4.0 = _8 * _9.0;
_5 = _3;
_13.1 = _13.0.0 as u32;
_13.1 = 3174858072_u32;
_15 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
place!(Field::<i128>(Variant(RET, 3), 3)) = (-76334852807049657272552495525803288248_i128) | 170115478875881997155418760185923823262_i128;
_8 = _4.0 * _4.0;
place!(Field::<*const bool>(Variant(RET, 3), 2)) = core::ptr::addr_of!(_13.0.0);
_11 = 94_i8 as i32;
_19 = (_4,);
_13.0.1 = 9157066957594882266_usize;
_10 = !11994721893430116362424739054337324650_u128;
_12.2 = -_12.1;
_13.3.0.0 = _1;
place!(Field::<i128>(Variant(RET, 3), 3)) = 105065903496814054048178497140694665607_i128 ^ (-147172962899788205347855814689147990511_i128);
place!(Field::<[usize; 5]>(Variant(RET, 3), 0)) = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
match _13.1 {
0 => bb1,
3174858072 => bb4,
_ => bb2
}
}
bb4 = {
_4.0 = -_8;
RET = Adt47::Variant2 { fld0: _19.0,fld1: _8 };
_3 = _5 ^ _13.0.0;
_13.1 = !1883781815_u32;
_13.0 = (_3, 1944917188716736462_usize);
place!(Field::<f64>(Variant(RET, 2), 1)) = _8 + _4.0;
_22 = ['\u{140b3}','\u{17aa0}'];
_20 = core::ptr::addr_of_mut!(_21);
_22 = ['\u{2f757}','\u{d9ef9}'];
place!(Field::<f64>(Variant(RET, 2), 1)) = (-12_i8) as f64;
_9 = (Field::<(f64,)>(Variant(RET, 2), 0).0,);
_11 = 546617512_i32 << _13.0.1;
_23 = _13.3.0.0 + _2;
Goto(bb5)
}
bb5 = {
Call(_25 = dump_var(4_usize, 15_usize, Move(_15), 23_usize, Move(_23), 22_usize, Move(_22), 2_usize, Move(_2)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_25 = dump_var(4_usize, 1_usize, Move(_1), 5_usize, Move(_5), 26_usize, _26, 26_usize, _26), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: Adt47,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: isize,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: isize,mut _10: bool,mut _11: bool,mut _12: isize,mut _13: bool,mut _14: bool) -> f64 {
mir! {
type RET = f64;
let _15: char;
let _16: u128;
let _17: f32;
let _18: char;
let _19: Adt58;
let _20: [u64; 6];
let _21: isize;
let _22: i16;
let _23: isize;
let _24: ();
let _25: ();
{
_11 = !_3;
_13 = !_4;
RET = -Field::<(f64,)>(Variant(_1, 2), 0).0;
place!(Field::<(f64,)>(Variant(_1, 2), 0)).0 = 486374676_i32 as f64;
_14 = _8 != _3;
place!(Field::<(f64,)>(Variant(_1, 2), 0)).0 = Field::<f64>(Variant(_1, 2), 1);
place!(Field::<(f64,)>(Variant(_1, 2), 0)) = (Field::<f64>(Variant(_1, 2), 1),);
place!(Field::<(f64,)>(Variant(_1, 2), 0)).0 = 678277987_u32 as f64;
place!(Field::<(f64,)>(Variant(_1, 2), 0)) = (Field::<f64>(Variant(_1, 2), 1),);
place!(Field::<(f64,)>(Variant(_1, 2), 0)) = (Field::<f64>(Variant(_1, 2), 1),);
_18 = '\u{15a7}';
_15 = _18;
RET = -Field::<f64>(Variant(_1, 2), 1);
_8 = _9 > _9;
_6 = _10 == _7;
_3 = _11;
RET = Field::<(f64,)>(Variant(_1, 2), 0).0 + Field::<f64>(Variant(_1, 2), 1);
place!(Field::<f64>(Variant(_1, 2), 1)) = Field::<(f64,)>(Variant(_1, 2), 0).0;
_17 = RET as f32;
_16 = !215795829791032878492136550874572735961_u128;
place!(Field::<(f64,)>(Variant(_1, 2), 0)) = (RET,);
SetDiscriminant(_1, 3);
Goto(bb1)
}
bb1 = {
place!(Field::<i128>(Variant(_1, 3), 3)) = 177_u8 as i128;
_18 = _15;
_11 = _5 <= _2;
_16 = 298681726375141555933355663952927490673_u128;
place!(Field::<*const bool>(Variant(_1, 3), 2)) = core::ptr::addr_of!(_7);
_20 = [15522877769234355506_u64,11154352824463395022_u64,7662292271611889301_u64,5028117820358360951_u64,11280366364036281585_u64,2566092088522965580_u64];
place!(Field::<[usize; 5]>(Variant(_1, 3), 0)) = [1_usize,16968969203443317196_usize,16917764692700398284_usize,1_usize,1_usize];
place!(Field::<i128>(Variant(_1, 3), 3)) = (-123141059689880047492127026383781521776_i128);
_13 = _7 > _8;
_13 = _4;
_15 = _18;
_15 = _18;
_17 = _2 as f32;
_22 = 30600_i16 ^ 17512_i16;
_20 = [14341710908377684480_u64,3749529185335623440_u64,9880527542030481693_u64,4586436172164476658_u64,4371265162539792816_u64,1065692880302550163_u64];
_6 = !_11;
_21 = _9;
_5 = 8958_u16 as isize;
_3 = !_10;
_16 = !227271467711601973653451943795476160534_u128;
place!(Field::<*const bool>(Variant(_1, 3), 2)) = core::ptr::addr_of!(_10);
RET = _17 as f64;
_23 = _2;
_11 = _14;
Goto(bb2)
}
bb2 = {
Call(_24 = dump_var(5_usize, 22_usize, Move(_22), 2_usize, Move(_2), 13_usize, Move(_13), 20_usize, Move(_20)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_24 = dump_var(5_usize, 3_usize, Move(_3), 12_usize, Move(_12), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_24 = dump_var(5_usize, 9_usize, Move(_9), 11_usize, Move(_11), 25_usize, _25, 25_usize, _25), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: (isize,),mut _2: i8,mut _3: u16,mut _4: ((isize,), i16),mut _5: *const bool) -> bool {
mir! {
type RET = bool;
let _6: ([i64; 6], u8);
let _7: ([i64; 6], u8);
let _8: f64;
let _9: (isize,);
let _10: [usize; 8];
let _11: (bool, usize);
let _12: *mut *const u8;
let _13: isize;
let _14: ([i64; 6], u8);
let _15: [char; 2];
let _16: char;
let _17: u32;
let _18: char;
let _19: u32;
let _20: (bool, usize);
let _21: [i64; 6];
let _22: i8;
let _23: bool;
let _24: isize;
let _25: (f64, [char; 2], u32);
let _26: bool;
let _27: f64;
let _28: ();
let _29: ();
{
RET = !(*_5);
_4 = (_1, (-9199_i16));
_6.0 = [8649049546165769963_i64,(-5122468962928577869_i64),(-4933544726710152766_i64),1759489809630580174_i64,4385486446144295437_i64,(-109296299013912707_i64)];
_4.0.0 = -_1.0;
_3 = 1485146824_u32 as u16;
_4.0 = _1;
_4.0.0 = _1.0;
(*_5) = _4.1 > _4.1;
_1.0 = _4.0.0;
(*_5) = RET ^ RET;
_8 = _2 as f64;
_3 = 2118_u16;
Goto(bb1)
}
bb1 = {
_2 = (-97_i8);
Call(_7.1 = core::intrinsics::bswap(195_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = 40_u8 as f64;
_7.0 = _6.0;
_9 = (_4.0.0,);
_2 = (-81_i8);
_4.1 = (-23563_i16) + 6026_i16;
_10 = [7_usize,3_usize,10133083122363042655_usize,12445859587778494293_usize,2_usize,12244566270333757781_usize,5696253336246992896_usize,17952779409706383864_usize];
_9 = (_4.0.0,);
_4.0.0 = _1.0 - _9.0;
_6 = (_7.0, 244_u8);
_7.0 = [(-3055772553166736268_i64),5941517855408611865_i64,(-7363331699791776569_i64),(-9221167494759761282_i64),1591410200520691026_i64,(-7941905330989562214_i64)];
_6 = _7;
_7.0 = [4651268216442366458_i64,8646221181154503559_i64,(-5892592117055487212_i64),1812361411249992075_i64,8737944313482272348_i64,6724921049576867705_i64];
RET = (*_5);
_2 = (-143087489646596593458435205762931969844_i128) as i8;
_8 = 9827870935556281320_u64 as f64;
_8 = (-6060233056122345781_i64) as f64;
_10 = [4_usize,1377261953773257402_usize,9907717601497015401_usize,6_usize,10988974279998498762_usize,1_usize,1_usize,6_usize];
Call(_4.0.0 = fn7(_6.0, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = core::ptr::addr_of!((*_5));
_2 = (-78_i8);
_4.1 = 24356_i16 >> _4.0.0;
_6.0 = [(-5667258331122678010_i64),8023202424484673172_i64,1985996269126856951_i64,(-6510072234830632403_i64),(-6100263944322758382_i64),1016612017146569462_i64];
_4.0.0 = -_9.0;
_11.0 = !(*_5);
Call(_6.1 = fn11(_10, _7.0, _10, _5, _4.0, _2, _7, _9.0, _4, _5, _9.0, _11.0, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.0 = [6631398780614885574_i64,(-1572693483252822438_i64),7322742505010171435_i64,2581385009259613536_i64,(-2453750843813908733_i64),2826510706015163694_i64];
_11.1 = 7_usize & 11202057979192919893_usize;
_13 = RET as isize;
_9.0 = !_1.0;
_6 = (_7.0, 2_u8);
_6 = (_14.0, 216_u8);
_4.1 = !22203_i16;
_2 = 42_i8;
RET = (*_5);
_13 = -_9.0;
Goto(bb5)
}
bb5 = {
(*_5) = !_11.0;
_7.0 = [(-6148975036756475695_i64),4163066088059669312_i64,(-589179364938565410_i64),8174069432388347334_i64,(-6333412812961618809_i64),(-7402838725575736685_i64)];
_13 = _4.0.0;
_4.0.0 = _1.0;
_4.0 = (_9.0,);
_7.1 = !_6.1;
(*_5) = RET;
_6.1 = _4.0.0 as u8;
_16 = '\u{7fe3d}';
_15 = [_16,_16];
_14 = _6;
_7.0 = [2235760566621536302_i64,(-6658564569185070129_i64),(-8263455710633296668_i64),6992801041298136692_i64,(-8803238577972455944_i64),(-2083415869934434691_i64)];
_11.1 = 5578950975055484471_usize;
RET = !_11.0;
_14.1 = _11.1 as u8;
_11.0 = RET;
_14.0 = _6.0;
_17 = 2217456381_u32 >> _7.1;
_9.0 = -_13;
_14.0 = [5781633398979345346_i64,(-224635358738034916_i64),(-7434668086758184922_i64),(-1719688102875196254_i64),2889003790505803192_i64,2372342766287639450_i64];
_3 = (*_5) as u16;
_1.0 = _13 & _4.0.0;
_6.0 = [(-8360638413314234308_i64),4536339714278441142_i64,(-2577628310296949585_i64),5721883116169481543_i64,1527313622893571831_i64,8369866169060652146_i64];
_9 = (_13,);
_10 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
Goto(bb6)
}
bb6 = {
_6 = _14;
_10 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_18 = _16;
_2 = _14.1 as i8;
_7.1 = _6.1 ^ _6.1;
_8 = _17 as f64;
RET = !(*_5);
_15 = [_16,_16];
_18 = _16;
_14.1 = !_7.1;
_4 = (_1, 26515_i16);
(*_5) = !RET;
_11.0 = (*_5);
(*_5) = _6.1 == _7.1;
RET = (*_5);
_14.0 = [8442738707682491128_i64,2780808996233601983_i64,(-8618310028653470672_i64),(-872423299358683687_i64),6867528445772547252_i64,(-7127897933048650037_i64)];
_19 = !_17;
_20.0 = !RET;
_16 = _18;
_7.1 = !_6.1;
_22 = -_2;
RET = !(*_5);
_2 = _22;
_1 = (_4.0.0,);
_4.0 = (_13,);
_19 = _17 * _17;
(*_5) = _11.0 & _11.0;
_5 = core::ptr::addr_of!(_20.0);
match _4.1 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb4,
26515 => bb8,
_ => bb7
}
}
bb7 = {
_5 = core::ptr::addr_of!((*_5));
_2 = (-78_i8);
_4.1 = 24356_i16 >> _4.0.0;
_6.0 = [(-5667258331122678010_i64),8023202424484673172_i64,1985996269126856951_i64,(-6510072234830632403_i64),(-6100263944322758382_i64),1016612017146569462_i64];
_4.0.0 = -_9.0;
_11.0 = !(*_5);
Call(_6.1 = fn11(_10, _7.0, _10, _5, _4.0, _2, _7, _9.0, _4, _5, _9.0, _11.0, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_18 = _16;
_6.1 = !_14.1;
_23 = !(*_5);
Goto(bb9)
}
bb9 = {
(*_5) = !_11.0;
Call(_20 = fn14(_4.0, _16, _14.0, _7, _19, _11.0, _19, _19, _5, _14.0, _14.0, _14, _6.0, _4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_20 = (_11.0, _11.1);
Goto(bb11)
}
bb11 = {
_11.0 = _19 > _17;
_11.1 = 3484910292557868793_i64 as usize;
_24 = _13 & _1.0;
_20 = (_11.0, _11.1);
_20.1 = !_11.1;
_25.2 = _17 | _17;
_18 = _16;
(*_5) = _17 < _17;
_7 = (_6.0, _14.1);
_18 = _16;
_8 = 129531975196245114561777144967637494410_i128 as f64;
match _4.1 {
0 => bb10,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
26515 => bb17,
_ => bb16
}
}
bb12 = {
_2 = (-97_i8);
Call(_7.1 = core::intrinsics::bswap(195_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
(*_5) = !_11.0;
Call(_20 = fn14(_4.0, _16, _14.0, _7, _19, _11.0, _19, _19, _5, _14.0, _14.0, _14, _6.0, _4), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
(*_5) = !_11.0;
_7.0 = [(-6148975036756475695_i64),4163066088059669312_i64,(-589179364938565410_i64),8174069432388347334_i64,(-6333412812961618809_i64),(-7402838725575736685_i64)];
_13 = _4.0.0;
_4.0.0 = _1.0;
_4.0 = (_9.0,);
_7.1 = !_6.1;
(*_5) = RET;
_6.1 = _4.0.0 as u8;
_16 = '\u{7fe3d}';
_15 = [_16,_16];
_14 = _6;
_7.0 = [2235760566621536302_i64,(-6658564569185070129_i64),(-8263455710633296668_i64),6992801041298136692_i64,(-8803238577972455944_i64),(-2083415869934434691_i64)];
_11.1 = 5578950975055484471_usize;
RET = !_11.0;
_14.1 = _11.1 as u8;
_11.0 = RET;
_14.0 = _6.0;
_17 = 2217456381_u32 >> _7.1;
_9.0 = -_13;
_14.0 = [5781633398979345346_i64,(-224635358738034916_i64),(-7434668086758184922_i64),(-1719688102875196254_i64),2889003790505803192_i64,2372342766287639450_i64];
_3 = (*_5) as u16;
_1.0 = _13 & _4.0.0;
_6.0 = [(-8360638413314234308_i64),4536339714278441142_i64,(-2577628310296949585_i64),5721883116169481543_i64,1527313622893571831_i64,8369866169060652146_i64];
_9 = (_13,);
_10 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
Goto(bb6)
}
bb15 = {
_8 = 40_u8 as f64;
_7.0 = _6.0;
_9 = (_4.0.0,);
_2 = (-81_i8);
_4.1 = (-23563_i16) + 6026_i16;
_10 = [7_usize,3_usize,10133083122363042655_usize,12445859587778494293_usize,2_usize,12244566270333757781_usize,5696253336246992896_usize,17952779409706383864_usize];
_9 = (_4.0.0,);
_4.0.0 = _1.0 - _9.0;
_6 = (_7.0, 244_u8);
_7.0 = [(-3055772553166736268_i64),5941517855408611865_i64,(-7363331699791776569_i64),(-9221167494759761282_i64),1591410200520691026_i64,(-7941905330989562214_i64)];
_6 = _7;
_7.0 = [4651268216442366458_i64,8646221181154503559_i64,(-5892592117055487212_i64),1812361411249992075_i64,8737944313482272348_i64,6724921049576867705_i64];
RET = (*_5);
_2 = (-143087489646596593458435205762931969844_i128) as i8;
_8 = 9827870935556281320_u64 as f64;
_8 = (-6060233056122345781_i64) as f64;
_10 = [4_usize,1377261953773257402_usize,9907717601497015401_usize,6_usize,10988974279998498762_usize,1_usize,1_usize,6_usize];
Call(_4.0.0 = fn7(_6.0, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_5 = core::ptr::addr_of!((*_5));
_2 = (-78_i8);
_4.1 = 24356_i16 >> _4.0.0;
_6.0 = [(-5667258331122678010_i64),8023202424484673172_i64,1985996269126856951_i64,(-6510072234830632403_i64),(-6100263944322758382_i64),1016612017146569462_i64];
_4.0.0 = -_9.0;
_11.0 = !(*_5);
Call(_6.1 = fn11(_10, _7.0, _10, _5, _4.0, _2, _7, _9.0, _4, _5, _9.0, _11.0, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb17 = {
_22 = _2;
_7.0 = [(-1877161805370808416_i64),(-2499515178824602189_i64),(-2151524205331803780_i64),(-1300166896012946554_i64),(-4373752074576015933_i64),7250646495377018245_i64];
_20.1 = _3 as usize;
_19 = _17 ^ _17;
_11 = ((*_5), _20.1);
_11.0 = !_20.0;
_25.1 = [_16,_16];
_20 = (_11.0, _11.1);
Goto(bb18)
}
bb18 = {
Call(_28 = dump_var(6_usize, 24_usize, Move(_24), 13_usize, Move(_13), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(6_usize, 20_usize, Move(_20), 15_usize, Move(_15), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(6_usize, 3_usize, Move(_3), 1_usize, Move(_1), 29_usize, _29, 29_usize, _29), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [i64; 6],mut _2: (isize,)) -> isize {
mir! {
type RET = isize;
let _3: Adt53;
let _4: [usize; 8];
let _5: bool;
let _6: f64;
let _7: &'static u16;
let _8: (f64, [char; 2], u32);
let _9: isize;
let _10: isize;
let _11: isize;
let _12: u8;
let _13: char;
let _14: &'static u16;
let _15: (bool, usize);
let _16: Adt45;
let _17: ((*mut i32, *mut *mut i32, u128, *const bool), i128, u128);
let _18: &'static u16;
let _19: i8;
let _20: [char; 2];
let _21: [i64; 6];
let _22: [isize; 7];
let _23: Adt44;
let _24: char;
let _25: u64;
let _26: *mut *mut i32;
let _27: [usize; 5];
let _28: [i64; 6];
let _29: isize;
let _30: [char; 2];
let _31: ((bool, usize), u32, isize, ((isize,), i16));
let _32: (char, i64, i64, &'static u16, [usize; 5]);
let _33: *mut *mut i32;
let _34: ([i64; 6], u8);
let _35: isize;
let _36: i64;
let _37: bool;
let _38: char;
let _39: f32;
let _40: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16);
let _41: ((*mut i32, *mut *mut i32, u128, *const bool), i128, u128);
let _42: Adt60;
let _43: [char; 2];
let _44: Adt49;
let _45: [i64; 6];
let _46: f64;
let _47: isize;
let _48: ();
let _49: ();
{
_2.0 = (-9223372036854775808_isize) - (-59_isize);
RET = 17250441099677606720_u64 as isize;
_3.fld2.0 = _2;
RET = 657903803_i32 as isize;
_3.fld2.0.0 = 28600_u16 as isize;
_3.fld2.0 = (_2.0,);
_3.fld1.0 = 4239022852332868914200454308212043826_u128 as f64;
Goto(bb1)
}
bb1 = {
_3.fld0.0 = -_3.fld1.0;
_3.fld0.1 = ['\u{76ed8}','\u{d24fc}'];
_3.fld0.2 = _3.fld2.0.0 as u32;
RET = 58_u8 as isize;
_3.fld2.1.1 = core::ptr::addr_of_mut!(_3.fld2.1.0);
_2.0 = 49_i8 as isize;
_3.fld1.0 = _3.fld0.0 + _3.fld0.0;
_3.fld2.0 = (_2.0,);
_3.fld2.1.1 = core::ptr::addr_of_mut!(_3.fld2.1.0);
Goto(bb2)
}
bb2 = {
_3.fld1.0 = -_3.fld0.0;
_3.fld0.0 = _3.fld1.0 - _3.fld1.0;
RET = -_3.fld2.0.0;
_2.0 = !_3.fld2.0.0;
_3.fld1.0 = RET as f64;
_3.fld0.2 = 718844465_u32;
_6 = -_3.fld0.0;
_3.fld2.1.3 = core::ptr::addr_of!(_5);
_3.fld2.1.2 = 85557133369196720328415062193192899448_u128;
_4 = [932485128055908059_usize,6_usize,4_usize,0_usize,6_usize,16127413178149656493_usize,11607095747646514063_usize,14620359965858682427_usize];
_5 = _3.fld2.0.0 != RET;
_8 = (_3.fld0.0, _3.fld0.1, _3.fld0.2);
_2.0 = _3.fld2.0.0 * RET;
_9 = RET ^ _2.0;
_3.fld2.2 = 6154_i16 + 25974_i16;
Call(_3.fld2.1 = fn8(_3.fld2.2, _2.0, _2, _1, _5, _8.0, _9, _1, _8.1, _4, _3.fld2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3.fld2.0.0 = !_2.0;
_3.fld0 = _8;
_12 = _3.fld2.2 as u8;
_11 = _8.2 as isize;
RET = _11 | _3.fld2.0.0;
_3.fld2.0.0 = !_9;
_1 = [7275111859569282806_i64,6128837375914040147_i64,(-7212437460116499571_i64),2248611867292395929_i64,(-8951087627266737748_i64),(-354491601266758425_i64)];
RET = _3.fld2.0.0 >> _9;
Goto(bb4)
}
bb4 = {
_3.fld0.2 = _8.2;
_3.fld2.1.2 = 1829222940_i32 as u128;
RET = !_3.fld2.0.0;
_3.fld2.0.0 = 1551339880643257213_usize as isize;
_8 = (_3.fld1.0, _3.fld0.1, _3.fld0.2);
_10 = _2.0;
_2 = _3.fld2.0;
_3.fld2.1.2 = 7222507688601650927_i64 as u128;
_13 = '\u{7ab1}';
RET = _3.fld2.0.0;
_15.1 = !828464413915025641_usize;
_3.fld0.0 = -_6;
_3.fld2.2 = 8406_i16 & (-17012_i16);
RET = _10 | _10;
_1 = [6395677633599903466_i64,6894057150818942525_i64,(-5629085595519558129_i64),(-8873772923135664308_i64),4993459472651397241_i64,(-902430959325276033_i64)];
_10 = -RET;
_3.fld1 = (_8.0,);
_3.fld2.0.0 = -RET;
Goto(bb5)
}
bb5 = {
_17.1 = 38075517038660288186275553825832599392_i128 >> _11;
_17.2 = 10090889256881821853_u64 as u128;
_8 = (_3.fld1.0, _3.fld0.1, _3.fld0.2);
_17.0 = _3.fld2.1;
_3.fld2.0.0 = -_9;
_3.fld2.1.3 = _17.0.3;
_20 = [_13,_13];
_19 = !(-20_i8);
_17.2 = _3.fld2.1.2 >> _3.fld2.2;
_17.2 = !_17.0.2;
_15.0 = _3.fld2.2 == _3.fld2.2;
_3.fld0.0 = -_6;
_17 = (_3.fld2.1, 32121548535088169880013652076790925793_i128, _3.fld2.1.2);
_3.fld2 = (_2, _17.0, (-22922_i16));
_8.1 = _20;
Goto(bb6)
}
bb6 = {
_17.0.1 = core::ptr::addr_of_mut!(_17.0.0);
_17.0.3 = _3.fld2.1.3;
_3.fld0.2 = _17.1 as u32;
_5 = _15.0 | _15.0;
_3.fld1.0 = -_3.fld0.0;
_3.fld0.0 = -_3.fld1.0;
_8 = (_3.fld1.0, _3.fld0.1, _3.fld0.2);
_3.fld2.0 = (_10,);
_17.2 = _17.1 as u128;
_3.fld0.1 = [_13,_13];
_25 = 1793293064365851666_u64 >> _3.fld0.2;
_20 = [_13,_13];
_3.fld0 = (_6, _8.1, _8.2);
_8.2 = _3.fld0.2;
_3.fld2.1.2 = _17.2;
_2 = _3.fld2.0;
_10 = _19 as isize;
_17 = (_3.fld2.1, 31804086213422692452341611481581567672_i128, _3.fld2.1.2);
Goto(bb7)
}
bb7 = {
_6 = _3.fld1.0 * _3.fld0.0;
_17 = (_3.fld2.1, (-123180904176157259387725315665902089141_i128), _3.fld2.1.2);
_3.fld0.1 = [_13,_13];
_12 = _8.2 as u8;
_8.0 = _15.1 as f64;
_28 = [3850841558607541315_i64,3578458735777144241_i64,(-6566343837037386593_i64),(-6148780671871105030_i64),(-7213523124594109503_i64),6920363934400427045_i64];
match _17.1 {
217101462744781204075649291765866122315 => bb9,
_ => bb8
}
}
bb8 = {
_3.fld0.0 = -_3.fld1.0;
_3.fld0.1 = ['\u{76ed8}','\u{d24fc}'];
_3.fld0.2 = _3.fld2.0.0 as u32;
RET = 58_u8 as isize;
_3.fld2.1.1 = core::ptr::addr_of_mut!(_3.fld2.1.0);
_2.0 = 49_i8 as isize;
_3.fld1.0 = _3.fld0.0 + _3.fld0.0;
_3.fld2.0 = (_2.0,);
_3.fld2.1.1 = core::ptr::addr_of_mut!(_3.fld2.1.0);
Goto(bb2)
}
bb9 = {
RET = _9 * _9;
_8.1 = [_13,_13];
_9 = RET;
_3.fld2 = (_2, _17.0, 12518_i16);
_17.0 = (_3.fld2.1.0, _3.fld2.1.1, _17.2, _3.fld2.1.3);
_15.0 = _5;
_21 = _1;
_27 = [_15.1,_15.1,_15.1,_15.1,_15.1];
_10 = _2.0;
_9 = -_2.0;
_26 = core::ptr::addr_of_mut!(_17.0.0);
_3.fld0 = (_6, _8.1, _8.2);
_2 = (_9,);
_31.3.1 = _3.fld2.2;
_15.0 = _5;
_3.fld0.2 = _8.2 - _8.2;
_31.3.1 = -_3.fld2.2;
_32.1 = 8338905514899173487_i64;
_31.0.0 = _5;
Goto(bb10)
}
bb10 = {
_31.1 = RET as u32;
_29 = _3.fld0.0 as isize;
_32.0 = _13;
_3.fld2.1.3 = _17.0.3;
_4 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_25 = 13912676165155352058_u64;
_31.3 = (_2, _3.fld2.2);
_22 = [_10,_9,RET,_10,_29,_2.0,_9];
_31.2 = _17.1 as isize;
_32.4 = [_15.1,_15.1,_15.1,_15.1,_15.1];
_15.1 = 3157921414822128026_usize;
_8.2 = _6 as u32;
match _25 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
13912676165155352058 => bb11,
_ => bb8
}
}
bb11 = {
_17.0 = (_3.fld2.1.0, _26, _3.fld2.1.2, _3.fld2.1.3);
_8.0 = _6;
_15.0 = _17.1 < _17.1;
_31.0 = (_15.0, _15.1);
_25 = 1325168359644453169_u64;
_35 = _31.2;
_17.0.3 = core::ptr::addr_of!(_15.0);
_39 = _3.fld0.2 as f32;
_31.3.1 = _3.fld2.2 << _17.0.2;
_2.0 = _10;
_2.0 = -_11;
_26 = core::ptr::addr_of_mut!(_17.0.0);
_4 = [_31.0.1,_15.1,_15.1,_31.0.1,_15.1,_15.1,_31.0.1,_15.1];
_3.fld2.1.1 = _17.0.1;
_27 = [_31.0.1,_31.0.1,_15.1,_31.0.1,_31.0.1];
_34.1 = _3.fld2.1.2 as u8;
_22 = [_35,_31.2,_31.2,_35,_29,_31.3.0.0,_35];
_17.0 = (_3.fld2.1.0, _26, _17.2, _3.fld2.1.3);
Call(_8 = fn10(_15.0, _3.fld2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_37 = !_15.0;
_26 = core::ptr::addr_of_mut!(_3.fld2.1.0);
_11 = _31.2 & _35;
_6 = _3.fld0.0;
_31.2 = !_11;
_40.1.3 = _3.fld2.1.3;
_40.1.1 = _17.0.1;
_2 = (_11,);
_41.0.2 = _17.2 & _17.0.2;
_5 = !_37;
_17.1 = (-168574808828580412083439589089592590398_i128) >> _31.3.1;
_17.1 = _34.1 as i128;
_32.2 = _32.1 >> _11;
_11 = !_31.2;
_41.0 = _3.fld2.1;
_41.0.3 = _40.1.3;
_3.fld0 = (_6, _20, _8.2);
_43 = [_32.0,_13];
_3.fld2.1.1 = core::ptr::addr_of_mut!((*_26));
_33 = _3.fld2.1.1;
_4 = [_15.1,_31.0.1,_31.0.1,_15.1,_15.1,_15.1,_15.1,_31.0.1];
_3.fld2.1.2 = !_17.0.2;
_15.0 = _5 == _37;
match _3.fld2.2 {
0 => bb7,
1 => bb3,
2 => bb13,
3 => bb14,
12518 => bb16,
_ => bb15
}
}
bb13 = {
_3.fld0.0 = -_3.fld1.0;
_3.fld0.1 = ['\u{76ed8}','\u{d24fc}'];
_3.fld0.2 = _3.fld2.0.0 as u32;
RET = 58_u8 as isize;
_3.fld2.1.1 = core::ptr::addr_of_mut!(_3.fld2.1.0);
_2.0 = 49_i8 as isize;
_3.fld1.0 = _3.fld0.0 + _3.fld0.0;
_3.fld2.0 = (_2.0,);
_3.fld2.1.1 = core::ptr::addr_of_mut!(_3.fld2.1.0);
Goto(bb2)
}
bb14 = {
_31.1 = RET as u32;
_29 = _3.fld0.0 as isize;
_32.0 = _13;
_3.fld2.1.3 = _17.0.3;
_4 = [_15.1,_15.1,_15.1,_15.1,_15.1,_15.1,_15.1,_15.1];
_25 = 13912676165155352058_u64;
_31.3 = (_2, _3.fld2.2);
_22 = [_10,_9,RET,_10,_29,_2.0,_9];
_31.2 = _17.1 as isize;
_32.4 = [_15.1,_15.1,_15.1,_15.1,_15.1];
_15.1 = 3157921414822128026_usize;
_8.2 = _6 as u32;
match _25 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
13912676165155352058 => bb11,
_ => bb8
}
}
bb15 = {
_3.fld0.2 = _8.2;
_3.fld2.1.2 = 1829222940_i32 as u128;
RET = !_3.fld2.0.0;
_3.fld2.0.0 = 1551339880643257213_usize as isize;
_8 = (_3.fld1.0, _3.fld0.1, _3.fld0.2);
_10 = _2.0;
_2 = _3.fld2.0;
_3.fld2.1.2 = 7222507688601650927_i64 as u128;
_13 = '\u{7ab1}';
RET = _3.fld2.0.0;
_15.1 = !828464413915025641_usize;
_3.fld0.0 = -_6;
_3.fld2.2 = 8406_i16 & (-17012_i16);
RET = _10 | _10;
_1 = [6395677633599903466_i64,6894057150818942525_i64,(-5629085595519558129_i64),(-8873772923135664308_i64),4993459472651397241_i64,(-902430959325276033_i64)];
_10 = -RET;
_3.fld1 = (_8.0,);
_3.fld2.0.0 = -RET;
Goto(bb5)
}
bb16 = {
_9 = _11;
_3.fld1.0 = _3.fld0.0;
_40.1.0 = _41.0.0;
Goto(bb17)
}
bb17 = {
Call(_48 = dump_var(7_usize, 37_usize, Move(_37), 22_usize, Move(_22), 28_usize, Move(_28), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(7_usize, 9_usize, Move(_9), 5_usize, Move(_5), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(7_usize, 31_usize, Move(_31), 4_usize, Move(_4), 10_usize, Move(_10), 49_usize, _49), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i16,mut _2: isize,mut _3: (isize,),mut _4: [i64; 6],mut _5: bool,mut _6: f64,mut _7: isize,mut _8: [i64; 6],mut _9: [char; 2],mut _10: [usize; 8],mut _11: (isize,)) -> (*mut i32, *mut *mut i32, u128, *const bool) {
mir! {
type RET = (*mut i32, *mut *mut i32, u128, *const bool);
let _12: f64;
let _13: (f64, [char; 2], u32);
let _14: [char; 2];
let _15: Adt47;
let _16: Adt44;
let _17: (f64, [char; 2], u32);
let _18: isize;
let _19: [usize; 5];
let _20: u64;
let _21: usize;
let _22: ();
let _23: ();
{
RET.3 = core::ptr::addr_of!(_5);
RET.2 = !282759789897516675290925900130263276612_u128;
_11.0 = _6 as isize;
_13 = (_6, _9, 4044316916_u32);
RET.1 = core::ptr::addr_of_mut!(RET.0);
_6 = 8225222706944310326_u64 as f64;
RET.2 = 306866560112076759229309589705749811768_u128;
RET.2 = 202018774398979895150784233029080182148_u128 + 61411374296201478829227208375842463720_u128;
_13.2 = 0_usize as u32;
_3 = (_7,);
RET.1 = core::ptr::addr_of_mut!(RET.0);
RET.3 = core::ptr::addr_of!(_5);
_2 = _11.0 | _11.0;
_5 = false | false;
_13.1 = ['\u{1afa5}','\u{1dbec}'];
_12 = _13.0 - _13.0;
_13.2 = !3114000005_u32;
_11 = (_7,);
_1 = (-13581_i16);
_17.0 = _6 - _6;
_17 = (_12, _9, _13.2);
Goto(bb1)
}
bb1 = {
_9 = _13.1;
_11.0 = _13.2 as isize;
_17.0 = _12;
_5 = _3.0 == _11.0;
_6 = _17.0;
_6 = _17.0;
RET.3 = core::ptr::addr_of!(_5);
_2 = _3.0;
_18 = !_7;
_18 = _7 & _3.0;
_17 = (_6, _13.1, _13.2);
_17.1 = _9;
_13.1 = ['\u{9c9e1}','\u{e29bb}'];
_18 = _3.0;
_6 = (-149188703_i32) as f64;
_2 = (-153079195931468005196671843258855896426_i128) as isize;
_8 = [5806954750372053728_i64,6248658263524228361_i64,2913794794392952846_i64,3378710973457182283_i64,4569224921240107302_i64,(-2592843926345361749_i64)];
_14 = ['\u{ea476}','\u{f416e}'];
_4 = [(-1977380163173681859_i64),(-6860770766365782798_i64),7746792602655109170_i64,(-1068727435098221311_i64),(-4889983376641310011_i64),73623224201220066_i64];
_11.0 = _3.0 ^ _3.0;
_18 = (-111062026714118303943533271033280377882_i128) as isize;
Goto(bb2)
}
bb2 = {
_11.0 = _7;
_15 = Adt47::Variant0 { fld0: _5,fld1: 28166_u16,fld2: 184497653_i32,fld3: _8 };
_4 = _8;
_17.0 = _6;
place!(Field::<u16>(Variant(_15, 0), 1)) = 42210_u16;
_5 = _13.2 == _17.2;
RET.3 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_15, 0), 0)));
RET.2 = 224660474371181791264037139953429514282_u128;
_12 = _13.0;
_12 = -_6;
_13 = _17;
_5 = Field::<bool>(Variant(_15, 0), 0) | Field::<bool>(Variant(_15, 0), 0);
_9 = _14;
_1 = -(-16421_i16);
_1 = (-18645_i16);
RET.2 = 1851988246239190106_u64 as u128;
Call(_12 = fn9(Field::<bool>(Variant(_15, 0), 0), _13.2, _4, _2, RET.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _5 as isize;
RET.0 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_15, 0), 2)));
_3.0 = _11.0;
RET.1 = core::ptr::addr_of_mut!(RET.0);
place!(Field::<bool>(Variant(_15, 0), 0)) = _7 != _7;
_11 = (_7,);
_9 = ['\u{f1c22}','\u{c344c}'];
_8 = Field::<[i64; 6]>(Variant(_15, 0), 3);
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(8_usize, 8_usize, Move(_8), 1_usize, Move(_1), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_22 = dump_var(8_usize, 3_usize, Move(_3), 4_usize, Move(_4), 23_usize, _23, 23_usize, _23), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: u32,mut _3: [i64; 6],mut _4: isize,mut _5: u128) -> f64 {
mir! {
type RET = f64;
let _6: isize;
let _7: Adt52;
let _8: Adt44;
let _9: isize;
let _10: f32;
let _11: isize;
let _12: Adt52;
let _13: [usize; 5];
let _14: ((f64,),);
let _15: u64;
let _16: [isize; 7];
let _17: isize;
let _18: isize;
let _19: isize;
let _20: isize;
let _21: u64;
let _22: f64;
let _23: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16);
let _24: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16);
let _25: (isize,);
let _26: i64;
let _27: isize;
let _28: f64;
let _29: Adt48;
let _30: f64;
let _31: u16;
let _32: [isize; 7];
let _33: Adt51;
let _34: (isize,);
let _35: [char; 2];
let _36: (f64,);
let _37: (*mut i32, *mut *mut i32, u128, *const bool);
let _38: Adt56;
let _39: ();
let _40: ();
{
RET = (-10_i8) as f64;
_1 = true;
_1 = true;
_1 = !false;
_4 = 119_u8 as isize;
_6 = _4;
RET = 18012_u16 as f64;
_6 = !_4;
_1 = false | true;
Goto(bb1)
}
bb1 = {
RET = _2 as f64;
Goto(bb2)
}
bb2 = {
_6 = _4 - _4;
_3 = [(-8322242669109926996_i64),7043261056100963061_i64,(-5828171131538376790_i64),(-583145103287212458_i64),482629745443997208_i64,1189086597623230002_i64];
_5 = 30285712897136098238780414037926593235_u128;
_1 = false;
_2 = (-26529189391601274610355113065546400598_i128) as u32;
RET = 17_u8 as f64;
_1 = _5 != _5;
RET = 17223_u16 as f64;
_4 = !_6;
RET = 48034_u16 as f64;
_4 = _2 as isize;
_1 = true;
RET = _2 as f64;
RET = 81_u8 as f64;
_2 = 6701209660338711788_usize as u32;
_7.fld1 = [8262097241511172520_u64,7239571574837424152_u64,8218651382691251546_u64,1837856426115305089_u64,13310130594163789093_u64,17387432087910774757_u64];
RET = _2 as f64;
_6 = _4 ^ _4;
_3 = [6741666326270616477_i64,(-8231317239310835365_i64),(-878484722310925462_i64),(-5712014850084479890_i64),(-8953576958356273901_i64),8081127179790881706_i64];
_1 = _4 <= _6;
_9 = _2 as isize;
_2 = !3464947952_u32;
RET = _2 as f64;
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
30285712897136098238780414037926593235 => bb7,
_ => bb6
}
}
bb3 = {
RET = _2 as f64;
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
_2 = 1828496748_u32 + 242891246_u32;
_3 = [(-6759702068862963727_i64),4385120922029710306_i64,(-7449925610142831597_i64),8985829269417775345_i64,6662992049114009832_i64,(-9202266819212414258_i64)];
_3 = [(-6283926147060591169_i64),(-5824277535375368207_i64),4300356281021928282_i64,(-6341084386835851052_i64),3211719807596933230_i64,(-3422530223213580394_i64)];
RET = 2396218737792162067_i64 as f64;
_10 = _5 as f32;
_7.fld1 = [2337305352569524768_u64,7424900248816360506_u64,14340938279170765137_u64,14239381373092661360_u64,9427935747466673138_u64,6421863460597814142_u64];
_6 = _4 + _9;
_4 = -_6;
_2 = 3377150746_u32;
_11 = _6 | _4;
_10 = _5 as f32;
RET = 54579_u16 as f64;
_1 = _11 <= _4;
_10 = RET as f32;
_2 = !1034999984_u32;
_2 = 468080499_u32 & 1590537407_u32;
_9 = !_4;
_9 = 2834533611094419701_usize as isize;
_6 = 2376147539839333890_usize as isize;
Goto(bb8)
}
bb8 = {
_3 = [(-6992736995212667349_i64),1703923522661449780_i64,(-1804439646501566112_i64),6832897325913747624_i64,(-6602193104859584535_i64),(-5463780962875832551_i64)];
_14.0 = (RET,);
_13 = [4_usize,3_usize,7_usize,1_usize,2331231907694860470_usize];
_5 = 73270435667986446578967895402374333210_u128;
_12.fld1 = [6972575862059088127_u64,8792734638924389828_u64,17105061775671824829_u64,5513408811653246247_u64,1744707009162931704_u64,14175423768256108919_u64];
RET = -_14.0.0;
_11 = -_9;
_7.fld1 = [4162720112569805193_u64,11157145637187944999_u64,15293116889968855609_u64,7539880548784615853_u64,7277966879575787961_u64,18065683870033657476_u64];
_7.fld1 = [14781020116823797811_u64,8079183426943522025_u64,14162658713041628783_u64,7341539402621039132_u64,15082226366880448184_u64,5570267954097525707_u64];
_15 = 18139015120127562358_usize as u64;
_14.0 = (RET,);
_9 = !_4;
_5 = 254246615811519956977712328525195256907_u128 & 128012024759482895749538861267531431_u128;
_6 = !_4;
_2 = 3646102243_u32;
_4 = !_9;
_13 = [7_usize,10539293432890916166_usize,8664235253048933844_usize,6_usize,104730017485697443_usize];
RET = -_14.0.0;
_10 = _9 as f32;
_13 = [7_usize,7_usize,10701980568254148855_usize,9248679417088166735_usize,18264320253845054282_usize];
_6 = _4 | _9;
_9 = _6 | _6;
_7.fld1 = [_15,_15,_15,_15,_15,_15];
_16 = [_9,_9,_6,_6,_9,_9,_6];
_16 = [_9,_6,_9,_9,_9,_4,_6];
RET = -_14.0.0;
Call(_6 = core::intrinsics::bswap(_9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = _14.0.0 * _14.0.0;
RET = -_14.0.0;
_2 = !3618642262_u32;
_12.fld1 = [_15,_15,_15,_15,_15,_15];
_4 = 0_usize as isize;
_15 = 734844700167648654_u64;
_17 = _9;
_15 = 15149546239690996058_u64 ^ 9533335144663357285_u64;
_13 = [6_usize,2_usize,3_usize,3_usize,1110842706041500426_usize];
_15 = (-836895402122585343_i64) as u64;
_14.0 = (RET,);
RET = _14.0.0;
_10 = 12329620239797796193_usize as f32;
_3 = [1712470615899270832_i64,(-9041357359275099793_i64),(-8701551679633051676_i64),(-4689174398018120823_i64),8279251659187361249_i64,2228476563673204490_i64];
_17 = -_6;
_6 = _4;
_14.0.0 = RET;
_8 = Adt44::Variant0 { fld0: _16,fld1: _15 };
_12 = Adt52 { fld0: _8,fld1: _7.fld1 };
_7 = Adt52 { fld0: _8,fld1: _12.fld1 };
_5 = 80700290960276228942704194052250983180_u128;
_5 = _1 as u128;
Goto(bb10)
}
bb10 = {
_20 = '\u{57786}' as isize;
_18 = !_17;
_13 = [2_usize,12997365056824938659_usize,11117210176684194067_usize,1_usize,3103991087442690810_usize];
_16 = Field::<[isize; 7]>(Variant(_8, 0), 0);
RET = _14.0.0 * _14.0.0;
_12.fld0 = _7.fld0;
RET = _14.0.0;
_6 = _18 - _18;
_12.fld0 = Adt44::Variant0 { fld0: Field::<[isize; 7]>(Variant(_7.fld0, 0), 0),fld1: Field::<u64>(Variant(_8, 0), 1) };
place!(Field::<u64>(Variant(_7.fld0, 0), 1)) = !_15;
_24.1.2 = _5 & _5;
_3 = [(-1559318203529094085_i64),(-7044399833705723917_i64),(-2270760310747997907_i64),(-7381819532410009092_i64),(-8399706216839078529_i64),(-7404818165894966225_i64)];
_23.0.0 = _18 >> _6;
place!(Field::<[isize; 7]>(Variant(_12.fld0, 0), 0)) = [_6,_9,_23.0.0,_18,_18,_23.0.0,_23.0.0];
place!(Field::<[isize; 7]>(Variant(_8, 0), 0)) = [_6,_4,_6,_9,_23.0.0,_9,_23.0.0];
_23.1.2 = _24.1.2;
_24.1.1 = core::ptr::addr_of_mut!(_24.1.0);
_16 = Field::<[isize; 7]>(Variant(_12.fld0, 0), 0);
_23.1.2 = 52075_u16 as u128;
_17 = _23.0.0 | _9;
_1 = !true;
_12 = Move(_7);
_1 = _4 != _23.0.0;
Goto(bb11)
}
bb11 = {
RET = -_14.0.0;
_22 = -RET;
_23.1.2 = _1 as u128;
Goto(bb12)
}
bb12 = {
_15 = !Field::<u64>(Variant(_8, 0), 1);
SetDiscriminant(_8, 0);
place!(Field::<u64>(Variant(_8, 0), 1)) = Field::<u64>(Variant(_12.fld0, 0), 1) + _15;
_24.1.1 = core::ptr::addr_of_mut!(_23.1.0);
_9 = _23.0.0;
_23.1.2 = _24.1.2 + _24.1.2;
_24.1.3 = core::ptr::addr_of!(_1);
_30 = -_14.0.0;
_24.2 = 112_u8 as i16;
_23.0.0 = _18 + _9;
_23.1.2 = _5;
_24.0 = (_23.0.0,);
_15 = !Field::<u64>(Variant(_8, 0), 1);
place!(Field::<[isize; 7]>(Variant(_8, 0), 0)) = [_17,_23.0.0,_6,_9,_6,_9,_18];
Goto(bb13)
}
bb13 = {
_24.1.1 = core::ptr::addr_of_mut!(_24.1.0);
_17 = _24.1.2 as isize;
_23.1.3 = _24.1.3;
_23.1.3 = core::ptr::addr_of!(_1);
_26 = (-3318519435379482152_i64) >> _9;
_21 = _15;
_27 = !_9;
_7.fld1 = _12.fld1;
_23.1.1 = core::ptr::addr_of_mut!(_24.1.0);
SetDiscriminant(_12.fld0, 0);
_10 = _24.1.2 as f32;
_7.fld0 = Adt44::Variant0 { fld0: _16,fld1: Field::<u64>(Variant(_8, 0), 1) };
_12.fld0 = _7.fld0;
_28 = _14.0.0;
_24.0 = _23.0;
_23.0 = (_24.0.0,);
_24.0 = _23.0;
_36 = (_28,);
_4 = !_24.0.0;
RET = _22;
SetDiscriminant(_8, 1);
Goto(bb14)
}
bb14 = {
_34 = _24.0;
_10 = 1963665299_i32 as f32;
_17 = _24.0.0 - _4;
_32 = [_17,_11,_6,_17,_17,_23.0.0,_9];
_23.2 = _24.2;
_37.2 = (-48_i8) as u128;
_4 = 449275504_i32 as isize;
place!(Field::<[isize; 7]>(Variant(_12.fld0, 0), 0)) = _32;
_19 = _24.0.0;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(9_usize, 9_usize, Move(_9), 6_usize, Move(_6), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(9_usize, 13_usize, Move(_13), 11_usize, Move(_11), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(9_usize, 19_usize, Move(_19), 27_usize, Move(_27), 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: bool,mut _2: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16)) -> (f64, [char; 2], u32) {
mir! {
type RET = (f64, [char; 2], u32);
let _3: char;
let _4: *mut f32;
let _5: u16;
let _6: isize;
let _7: (isize,);
let _8: ((isize,), i16);
let _9: ((isize,), i16);
let _10: isize;
let _11: u8;
let _12: usize;
let _13: u128;
let _14: i32;
let _15: (isize,);
let _16: i64;
let _17: u128;
let _18: i64;
let _19: f32;
let _20: Adt58;
let _21: [u64; 6];
let _22: (bool, usize);
let _23: isize;
let _24: [u64; 6];
let _25: f32;
let _26: [char; 2];
let _27: i128;
let _28: Adt44;
let _29: (isize,);
let _30: char;
let _31: u8;
let _32: i8;
let _33: f32;
let _34: bool;
let _35: i32;
let _36: [usize; 5];
let _37: char;
let _38: ();
let _39: ();
{
RET.1 = ['\u{10afff}','\u{7611a}'];
RET.0 = 8573535444975061872_u64 as f64;
RET.0 = _2.1.2 as f64;
RET.1 = ['\u{ac795}','\u{10bfdd}'];
_3 = '\u{ddd99}';
RET.2 = 541610295_u32;
_2.2 = _2.0.0 as i16;
RET.1 = [_3,_3];
_1 = !true;
RET.0 = _2.0.0 as f64;
RET.0 = (-93078079203162339690940031868420486682_i128) as f64;
RET.2 = !1649132359_u32;
RET.1 = [_3,_3];
_2.1.3 = core::ptr::addr_of!(_1);
_2.0 = ((-9223372036854775808_isize),);
RET.1 = [_3,_3];
_5 = !59571_u16;
_2.0.0 = 9223372036854775807_isize;
RET.0 = 4319305557031863543_u64 as f64;
match _2.0.0 {
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
_7.0 = _2.0.0 ^ _2.0.0;
_7 = (_2.0.0,);
_7 = _2.0;
_2.1.2 = !233905280676036901155322227223449716760_u128;
RET.1 = [_3,_3];
RET.2 = 1355806480_u32;
_7 = _2.0;
_2.1.3 = core::ptr::addr_of!(_1);
_8.0.0 = !_7.0;
_2.2 = -4291_i16;
_9 = (_7, _2.2);
Goto(bb4)
}
bb4 = {
_2.0.0 = 13554282650393337078_u64 as isize;
_8.0 = (_7.0,);
RET.1 = [_3,_3];
_10 = _2.1.2 as isize;
_1 = RET.0 == RET.0;
_11 = !193_u8;
_10 = !_8.0.0;
_9.1 = _2.2 | _2.2;
_11 = 90_u8 & 59_u8;
_1 = _7.0 >= _9.0.0;
_9.0.0 = _10;
_9.1 = _2.2;
_12 = 7_usize;
_8.1 = _2.2;
_2.0 = _9.0;
_8.1 = (-2244083138253712776_i64) as i16;
_11 = !244_u8;
_14 = (-1761424557_i32) + (-1719792440_i32);
Goto(bb5)
}
bb5 = {
_15.0 = _7.0 << _7.0;
_2.0 = (_10,);
_2.0.0 = _7.0;
RET.1 = [_3,_3];
_9 = (_7, _2.2);
_12 = _5 as usize;
_14 = 235740265_i32 - 1392151504_i32;
_6 = _8.0.0;
_7.0 = _8.1 as isize;
_2.1.1 = core::ptr::addr_of_mut!(_2.1.0);
_9.0.0 = _14 as isize;
_2.1.2 = 312118578079924096968408294373009681683_u128;
_13 = !_2.1.2;
_5 = 23997_u16;
_8.1 = _7.0 as i16;
Goto(bb6)
}
bb6 = {
_8 = (_9.0, _9.1);
_5 = 59469_u16;
_9.0.0 = !_2.0.0;
_7.0 = _8.0.0 + _2.0.0;
_6 = _10 + _8.0.0;
_4 = core::ptr::addr_of_mut!(_19);
_1 = true | true;
_13 = RET.0 as u128;
_16 = _7.0 as i64;
_12 = !14199889799753399412_usize;
_11 = 28_u8 ^ 86_u8;
_2.1.3 = core::ptr::addr_of!(_1);
_10 = RET.2 as isize;
_4 = core::ptr::addr_of_mut!((*_4));
RET.1 = [_3,_3];
_16 = !8003867835137066196_i64;
_15 = _7;
_9.0 = _15;
_7 = (_8.0.0,);
_8.0.0 = _7.0 ^ _6;
Call(_9.0.0 = core::intrinsics::bswap(_8.0.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = !false;
RET.0 = 155768288832485412462788084792935054421_i128 as f64;
_15 = _8.0;
_15.0 = _9.0.0;
_8 = (_7, _9.1);
_17 = _2.1.2;
(*_4) = _11 as f32;
_2.2 = -_9.1;
_3 = '\u{34b34}';
_8.1 = _9.1;
_17 = !_13;
_22 = (_1, _12);
_22.0 = !_1;
_2.0.0 = _7.0;
_17 = _13;
_9.1 = RET.2 as i16;
_24 = [10324531353252007674_u64,7594182989026029266_u64,12042688491468331789_u64,2808839468713628801_u64,17563603798054396116_u64,16857820842868819753_u64];
_9.0 = (_8.0.0,);
_2.0.0 = -_9.0.0;
_11 = !167_u8;
match RET.2 {
0 => bb1,
1 => bb4,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
1355806480 => bb13,
_ => bb12
}
}
bb8 = {
_8 = (_9.0, _9.1);
_5 = 59469_u16;
_9.0.0 = !_2.0.0;
_7.0 = _8.0.0 + _2.0.0;
_6 = _10 + _8.0.0;
_4 = core::ptr::addr_of_mut!(_19);
_1 = true | true;
_13 = RET.0 as u128;
_16 = _7.0 as i64;
_12 = !14199889799753399412_usize;
_11 = 28_u8 ^ 86_u8;
_2.1.3 = core::ptr::addr_of!(_1);
_10 = RET.2 as isize;
_4 = core::ptr::addr_of_mut!((*_4));
RET.1 = [_3,_3];
_16 = !8003867835137066196_i64;
_15 = _7;
_9.0 = _15;
_7 = (_8.0.0,);
_8.0.0 = _7.0 ^ _6;
Call(_9.0.0 = core::intrinsics::bswap(_8.0.0), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_15.0 = _7.0 << _7.0;
_2.0 = (_10,);
_2.0.0 = _7.0;
RET.1 = [_3,_3];
_9 = (_7, _2.2);
_12 = _5 as usize;
_14 = 235740265_i32 - 1392151504_i32;
_6 = _8.0.0;
_7.0 = _8.1 as isize;
_2.1.1 = core::ptr::addr_of_mut!(_2.1.0);
_9.0.0 = _14 as isize;
_2.1.2 = 312118578079924096968408294373009681683_u128;
_13 = !_2.1.2;
_5 = 23997_u16;
_8.1 = _7.0 as i16;
Goto(bb6)
}
bb10 = {
_2.0.0 = 13554282650393337078_u64 as isize;
_8.0 = (_7.0,);
RET.1 = [_3,_3];
_10 = _2.1.2 as isize;
_1 = RET.0 == RET.0;
_11 = !193_u8;
_10 = !_8.0.0;
_9.1 = _2.2 | _2.2;
_11 = 90_u8 & 59_u8;
_1 = _7.0 >= _9.0.0;
_9.0.0 = _10;
_9.1 = _2.2;
_12 = 7_usize;
_8.1 = _2.2;
_2.0 = _9.0;
_8.1 = (-2244083138253712776_i64) as i16;
_11 = !244_u8;
_14 = (-1761424557_i32) + (-1719792440_i32);
Goto(bb5)
}
bb11 = {
_7.0 = _2.0.0 ^ _2.0.0;
_7 = (_2.0.0,);
_7 = _2.0;
_2.1.2 = !233905280676036901155322227223449716760_u128;
RET.1 = [_3,_3];
RET.2 = 1355806480_u32;
_7 = _2.0;
_2.1.3 = core::ptr::addr_of!(_1);
_8.0.0 = !_7.0;
_2.2 = -4291_i16;
_9 = (_7, _2.2);
Goto(bb4)
}
bb12 = {
Return()
}
bb13 = {
RET.1 = [_3,_3];
_9.0.0 = !_8.0.0;
_17 = !_2.1.2;
_2.0 = (_15.0,);
_9.0 = (_6,);
_5 = (-146402806732100428196010697402330217816_i128) as u16;
_11 = 97_u8 & 30_u8;
_19 = _22.1 as f32;
_24 = [16057308071344901041_u64,15584165873593756337_u64,14895464588847274921_u64,4956746676822505392_u64,10439658190941531684_u64,15611570642029964831_u64];
_26 = [_3,_3];
_25 = _8.1 as f32;
_12 = _22.1;
_23 = RET.2 as isize;
_19 = _5 as f32;
_5 = !17628_u16;
_5 = !38173_u16;
_13 = _2.1.2 & _2.1.2;
_8.1 = _9.1;
_15.0 = RET.2 as isize;
_21 = [8326917632065631538_u64,6402798702758412903_u64,5220225071528456383_u64,8387671334329007729_u64,5055842960861248681_u64,8950578438401786587_u64];
_2.0.0 = 55575466888616360101343575039623207101_i128 as isize;
_33 = _19;
RET.0 = _16 as f64;
_8.0.0 = _9.0.0 - _9.0.0;
_24 = [11443473734137287221_u64,15676354905182972538_u64,16205070960833396410_u64,15775401536893502403_u64,10779635387379499165_u64,4826683246435211532_u64];
_12 = _22.1 & _22.1;
Goto(bb14)
}
bb14 = {
_12 = _22.1;
RET.0 = _16 as f64;
_8.0 = (_9.0.0,);
_35 = (-7802953259522610357339188128731780504_i128) as i32;
_18 = _14 as i64;
RET.2 = _13 as u32;
_22 = (_1, _12);
_24 = [1318134019366052403_u64,14358645876329627399_u64,682183313790873643_u64,18272074990027088518_u64,6388179427602707273_u64,4929763050174426415_u64];
_22.0 = _1;
_2.1.0 = core::ptr::addr_of_mut!(_14);
_8.0.0 = _6;
_7 = (_10,);
_5 = 5587_u16 | 41536_u16;
_7 = (_10,);
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(10_usize, 21_usize, Move(_21), 3_usize, Move(_3), 24_usize, Move(_24), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(10_usize, 17_usize, Move(_17), 16_usize, Move(_16), 11_usize, Move(_11), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(10_usize, 6_usize, Move(_6), 10_usize, Move(_10), 8_usize, Move(_8), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [usize; 8],mut _2: [i64; 6],mut _3: [usize; 8],mut _4: *const bool,mut _5: (isize,),mut _6: i8,mut _7: ([i64; 6], u8),mut _8: isize,mut _9: ((isize,), i16),mut _10: *const bool,mut _11: isize,mut _12: bool,mut _13: ([i64; 6], u8)) -> u8 {
mir! {
type RET = u8;
let _14: isize;
let _15: bool;
let _16: [isize; 7];
let _17: i32;
let _18: f32;
let _19: u64;
let _20: [usize; 5];
let _21: u32;
let _22: i64;
let _23: (f64,);
let _24: Adt57;
let _25: ();
let _26: ();
{
_2 = [3935842279633737226_i64,6665295251767571402_i64,4898232011704220818_i64,(-962474385567434056_i64),(-8824052899658096399_i64),2279902464879041377_i64];
(*_10) = !_12;
(*_10) = _12;
RET = 322487267_i32 as u8;
_2 = _13.0;
_1 = _3;
(*_10) = _12;
(*_10) = _12;
_12 = _9.1 <= _9.1;
_3 = [2_usize,6_usize,3_usize,3_usize,7_usize,5436424458634246035_usize,7_usize,3_usize];
_11 = _5.0 * _5.0;
_6 = !(-127_i8);
_4 = core::ptr::addr_of!(_12);
_5.0 = _8 >> RET;
_10 = core::ptr::addr_of!(_12);
_10 = core::ptr::addr_of!((*_10));
_9.0 = (_11,);
_13 = (_7.0, RET);
_3 = [7_usize,2_usize,4_usize,10596184590137178489_usize,5721376780551394694_usize,15667966580239157421_usize,0_usize,3378394595648852733_usize];
Call(_9 = fn12(_4, _10, _11, (*_10), (*_10), _5, _3, (*_10), (*_10), _12, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _7.0;
_9.0 = _5;
_10 = core::ptr::addr_of!(_15);
_16 = [_11,_11,_5.0,_11,_8,_11,_8];
_7.1 = RET;
_9 = (_5, (-27621_i16));
_7.1 = !_13.1;
RET = _7.1;
_4 = core::ptr::addr_of!((*_4));
_1 = _3;
_16 = [_8,_9.0.0,_9.0.0,_11,_5.0,_11,_5.0];
_13.0 = [(-1750589333288595208_i64),8055124322859634022_i64,8494947733807981409_i64,(-2635519927444307430_i64),(-950014391086849595_i64),(-8180026423313869379_i64)];
(*_10) = _12 <= (*_4);
_8 = _11 * _11;
_11 = 1824648924_i32 as isize;
_14 = _8;
_7.0 = _13.0;
_13.1 = _7.1;
Goto(bb2)
}
bb2 = {
(*_10) = (*_4);
_10 = core::ptr::addr_of!((*_10));
match _9.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768183835 => bb9,
_ => bb8
}
}
bb3 = {
_2 = _7.0;
_9.0 = _5;
_10 = core::ptr::addr_of!(_15);
_16 = [_11,_11,_5.0,_11,_8,_11,_8];
_7.1 = RET;
_9 = (_5, (-27621_i16));
_7.1 = !_13.1;
RET = _7.1;
_4 = core::ptr::addr_of!((*_4));
_1 = _3;
_16 = [_8,_9.0.0,_9.0.0,_11,_5.0,_11,_5.0];
_13.0 = [(-1750589333288595208_i64),8055124322859634022_i64,8494947733807981409_i64,(-2635519927444307430_i64),(-950014391086849595_i64),(-8180026423313869379_i64)];
(*_10) = _12 <= (*_4);
_8 = _11 * _11;
_11 = 1824648924_i32 as isize;
_14 = _8;
_7.0 = _13.0;
_13.1 = _7.1;
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
_7 = _13;
_5.0 = !_8;
_1 = [15666877093936201931_usize,1_usize,3313123725279366373_usize,6191524435539460744_usize,6_usize,6_usize,9222417627954318630_usize,1989351310909521003_usize];
_5 = _9.0;
match _9.1 {
0 => bb5,
1 => bb10,
340282366920938463463374607431768183835 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_9.0 = (_14,);
match _9.1 {
0 => bb5,
1 => bb13,
2 => bb14,
340282366920938463463374607431768183835 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_2 = _7.0;
_9.0 = _5;
_10 = core::ptr::addr_of!(_15);
_16 = [_11,_11,_5.0,_11,_8,_11,_8];
_7.1 = RET;
_9 = (_5, (-27621_i16));
_7.1 = !_13.1;
RET = _7.1;
_4 = core::ptr::addr_of!((*_4));
_1 = _3;
_16 = [_8,_9.0.0,_9.0.0,_11,_5.0,_11,_5.0];
_13.0 = [(-1750589333288595208_i64),8055124322859634022_i64,8494947733807981409_i64,(-2635519927444307430_i64),(-950014391086849595_i64),(-8180026423313869379_i64)];
(*_10) = _12 <= (*_4);
_8 = _11 * _11;
_11 = 1824648924_i32 as isize;
_14 = _8;
_7.0 = _13.0;
_13.1 = _7.1;
Goto(bb2)
}
bb16 = {
_18 = (-572833119211293669_i64) as f32;
(*_10) = (*_4);
_2 = [2253766092808232885_i64,(-306034511458024509_i64),6543548779774907676_i64,(-555965466950986789_i64),(-1977156133755998345_i64),(-1788894137550491054_i64)];
_1 = _3;
_7.1 = !_13.1;
_19 = 10707075203469112965_u64 * 7277331361382136295_u64;
_5.0 = 64477_u16 as isize;
_7.0 = [(-6810340580069613702_i64),1058817270288222504_i64,255700220295577969_i64,2096894284880258650_i64,4949694498127860090_i64,7053749297711684268_i64];
_14 = !_5.0;
_7.0 = _2;
_19 = 13860760443943319802_u64 ^ 10575639431961064338_u64;
_4 = core::ptr::addr_of!(_12);
RET = _7.1 ^ _13.1;
RET = !_7.1;
_9.1 = !(-2546_i16);
_6 = RET as i8;
(*_4) = !(*_10);
(*_4) = _15;
_2 = [(-2393021864001601606_i64),527627584394835175_i64,(-8610399822062099502_i64),(-9189112629776001169_i64),5828480702489508273_i64,9002978768171935805_i64];
_4 = core::ptr::addr_of!((*_10));
RET = _9.1 as u8;
_9.0 = (_8,);
_22 = !6490007337415824917_i64;
_6 = -(-79_i8);
_2 = _13.0;
_23.0 = _14 as f64;
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(11_usize, 3_usize, Move(_3), 8_usize, Move(_8), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_25 = dump_var(11_usize, 14_usize, Move(_14), 9_usize, Move(_9), 22_usize, Move(_22), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *const bool,mut _2: *const bool,mut _3: isize,mut _4: bool,mut _5: bool,mut _6: (isize,),mut _7: [usize; 8],mut _8: bool,mut _9: bool,mut _10: bool,mut _11: isize) -> ((isize,), i16) {
mir! {
type RET = ((isize,), i16);
let _12: Adt46;
let _13: *const u8;
let _14: *mut f32;
let _15: ([i64; 6], u8);
let _16: Adt51;
let _17: Adt60;
let _18: [isize; 7];
let _19: (f64,);
let _20: char;
let _21: usize;
let _22: [char; 2];
let _23: ([i64; 6], u8);
let _24: f32;
let _25: (*mut i32, *mut *mut i32, u128, *const bool);
let _26: ((f64,),);
let _27: Adt53;
let _28: Adt58;
let _29: f32;
let _30: Adt60;
let _31: f64;
let _32: ([i64; 6], u8);
let _33: [usize; 5];
let _34: (isize,);
let _35: isize;
let _36: [u64; 6];
let _37: ([i64; 6], u8);
let _38: ((isize,), i16);
let _39: [char; 2];
let _40: ((f64,),);
let _41: ();
let _42: ();
{
RET.1 = -5451_i16;
RET = (_6, 22044_i16);
_2 = core::ptr::addr_of!((*_2));
_2 = _1;
_8 = (*_1) >= _5;
_1 = core::ptr::addr_of!((*_1));
_11 = _3 + _6.0;
RET = (_6, (-16544_i16));
(*_1) = _10 != _4;
(*_1) = !_4;
(*_1) = _5 | _9;
RET.0 = (_11,);
RET.0 = _6;
(*_1) = _4 != _5;
_8 = !(*_1);
_10 = (*_1);
_6.0 = -_11;
RET.0.0 = _6.0;
(*_2) = _4 < _8;
_8 = _10;
_3 = '\u{1d73c}' as isize;
match RET.1 {
0 => bb1,
1 => bb2,
340282366920938463463374607431768194912 => bb4,
_ => bb3
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
_15.0 = [(-4605271238826566998_i64),(-4813810306501414567_i64),(-6903464337973520595_i64),(-1803323415629426225_i64),(-1970951672052941075_i64),(-9027607557166515682_i64)];
RET.0 = (_6.0,);
match RET.1 {
340282366920938463463374607431768194912 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_11 = _6.0;
_7 = [7_usize,7728356855426214774_usize,1711237200321905733_usize,14478938967729668817_usize,18353812229276522291_usize,5692132261661787008_usize,5880834045330232090_usize,8144533240427160858_usize];
RET.1 = (-6158_i16) - (-12758_i16);
_6 = (_11,);
_1 = core::ptr::addr_of!((*_1));
(*_1) = _5 < _10;
_7 = [4_usize,383526755871838637_usize,17087446958475563937_usize,6_usize,14245202194105635647_usize,3059392645127843374_usize,1_usize,2_usize];
_2 = core::ptr::addr_of!(_4);
(*_2) = _5;
_18 = [_6.0,_11,_11,_11,RET.0.0,_11,_3];
_21 = 6_usize;
RET = (_6, (-19987_i16));
_20 = '\u{28822}';
_18 = [_6.0,_11,RET.0.0,_6.0,RET.0.0,_6.0,_11];
_12 = Adt46::Variant0 { fld0: _20 };
RET.0.0 = _11;
_15.1 = 210_u8 >> RET.0.0;
SetDiscriminant(_12, 0);
_23 = (_15.0, _15.1);
Call((*_1) = fn13(_10, _10, _1, _7[_21], RET.0.0, _4, _1, _7[_21]), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = _1;
place!(Field::<char>(Variant(_12, 0), 0)) = _20;
_6.0 = 272112621373035216736192806810161388523_u128 as isize;
_23.0 = [(-6232798166435114966_i64),1846775310479481253_i64,(-15583699100993638_i64),(-7654363503657065968_i64),577575851183096978_i64,(-8328433899625276821_i64)];
_19.0 = 1172154964_u32 as f64;
_14 = core::ptr::addr_of_mut!(_24);
_25.3 = core::ptr::addr_of!(_4);
_27.fld0.1 = [_20,_20];
Goto(bb8)
}
bb8 = {
RET.0 = (_11,);
Goto(bb9)
}
bb9 = {
_31 = _19.0;
_26.0.0 = _19.0 + _19.0;
_27.fld2.0 = (_11,);
_6.0 = RET.0.0;
Goto(bb10)
}
bb10 = {
RET = (_6, 18092_i16);
_27.fld0.1 = [Field::<char>(Variant(_12, 0), 0),_20];
_24 = 105_i8 as f32;
_10 = _8 <= _9;
_13 = core::ptr::addr_of!(_15.1);
_25.1 = core::ptr::addr_of_mut!(_25.0);
SetDiscriminant(_12, 0);
(*_14) = 49_i8 as f32;
match RET.1 {
0 => bb6,
1 => bb2,
18092 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_7 = [_21,_21,_21,_21,_21,_21,_21,_21];
(*_13) = _20 as u8;
_32.0 = [7713141317556328646_i64,(-1510786843213133128_i64),(-5131144774590060306_i64),3727114983887498112_i64,6317409000853811382_i64,(-592458404676442716_i64)];
_33 = [_21,_21,_21,_21,_21];
_27.fld2.1.2 = 111511047025971434517952898836672752545_u128 * 281822900998667271134125820976522722117_u128;
_27.fld2.2 = _21 as i16;
Goto(bb13)
}
bb13 = {
_27.fld2.1.2 = !133153210621455399972461196336690513485_u128;
_13 = core::ptr::addr_of!(_37.1);
_36 = [5877693468468612209_u64,1630185501563856568_u64,2911118644894835058_u64,6713189151395083204_u64,10103964249230286450_u64,2410155575176252992_u64];
_25.1 = core::ptr::addr_of_mut!(_27.fld2.1.0);
Goto(bb14)
}
bb14 = {
_20 = '\u{4d33}';
_36 = [12201405992394375809_u64,16631158590781525244_u64,3178717513773002326_u64,13851513632068051318_u64,14928002491667028116_u64,13046942362160332151_u64];
_15.0 = _23.0;
_25.2 = !_27.fld2.1.2;
_27.fld1 = (_31,);
_24 = _6.0 as f32;
_29 = (*_14) * (*_14);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(12_usize, 18_usize, Move(_18), 20_usize, Move(_20), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(12_usize, 33_usize, Move(_33), 11_usize, Move(_11), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: bool,mut _3: *const bool,mut _4: usize,mut _5: isize,mut _6: bool,mut _7: *const bool,mut _8: usize) -> bool {
mir! {
type RET = bool;
let _9: i8;
let _10: [u64; 6];
let _11: [i64; 6];
let _12: ();
let _13: ();
{
RET = !_1;
_8 = _4 * _4;
_4 = _8;
RET = !_2;
_5 = (-97_isize) * 42_isize;
_4 = 31795342_i32 as usize;
RET = _1;
_7 = core::ptr::addr_of!(RET);
_10 = [10005037159554175392_u64,1515401351292638108_u64,5027302367146999945_u64,3272145489158440523_u64,12164511146962512927_u64,334587921969995279_u64];
_2 = (*_7);
_10 = [2177703219450928038_u64,13931463077453780121_u64,159195333302470782_u64,7106668613943564575_u64,17531843477624236761_u64,5416450622129575661_u64];
_7 = core::ptr::addr_of!(_6);
_1 = !(*_7);
_2 = !RET;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(13_usize, 5_usize, Move(_5), 2_usize, Move(_2), 10_usize, Move(_10), 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: (isize,),mut _2: char,mut _3: [i64; 6],mut _4: ([i64; 6], u8),mut _5: u32,mut _6: bool,mut _7: u32,mut _8: u32,mut _9: *const bool,mut _10: [i64; 6],mut _11: [i64; 6],mut _12: ([i64; 6], u8),mut _13: [i64; 6],mut _14: ((isize,), i16)) -> (bool, usize) {
mir! {
type RET = (bool, usize);
let _15: isize;
let _16: i32;
let _17: f32;
let _18: i8;
let _19: [isize; 7];
let _20: (bool, usize);
let _21: ();
let _22: ();
{
RET = (_6, 2_usize);
_14.0 = (_1.0,);
_15 = RET.0 as isize;
_7 = _5 ^ _5;
_7 = _5;
_12.0 = _10;
_12.0 = [8069648764589826641_i64,(-5509285823165586202_i64),(-7214905129409224769_i64),(-5544776770026785693_i64),984946147992648632_i64,(-6898243729897510808_i64)];
_14 = (_1, 27376_i16);
RET = (_6, 16817879800703453049_usize);
RET.1 = !3_usize;
_1 = (_15,);
_13 = [7475764275647023577_i64,(-2190089382325190287_i64),2406242602851484315_i64,7659577068236300713_i64,(-2874378014250468879_i64),(-685656707019799365_i64)];
_4.1 = _12.1;
_6 = RET.0 & RET.0;
_14.1 = -(-7619_i16);
_3 = [2950839198719048601_i64,(-4911608915390208563_i64),(-3199049235176521020_i64),4196533766106459149_i64,8682183255199731982_i64,6937732717138072969_i64];
_2 = '\u{18777}';
RET.0 = _4.1 >= _4.1;
_15 = _14.1 as isize;
RET.0 = _6;
_14 = (_1, 29045_i16);
_9 = core::ptr::addr_of!(RET.0);
RET.1 = !4384768323017502143_usize;
_8 = 62315254323441947926922933459475395257_u128 as u32;
_4.1 = _12.1;
_10 = [931488523457220617_i64,(-3697056410920718645_i64),(-4707997777705284861_i64),675698585289756004_i64,(-2548928823077842649_i64),(-6377874968974065080_i64)];
match _14.1 {
0 => bb1,
29045 => bb3,
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
RET.0 = _6 <= _6;
Goto(bb4)
}
bb4 = {
Call(_21 = dump_var(14_usize, 11_usize, Move(_11), 12_usize, Move(_12), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_21 = dump_var(14_usize, 6_usize, Move(_6), 13_usize, Move(_13), 8_usize, Move(_8), 22_usize, _22), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{9282f}'), std::hint::black_box((-62_isize)), std::hint::black_box((-99_i8)), std::hint::black_box((-4453_i16)), std::hint::black_box(1360348306_i32), std::hint::black_box((-7230033652888106399_i64)), std::hint::black_box((-96116110429419088790334098214143340036_i128)), std::hint::black_box(5_usize), std::hint::black_box(94_u8), std::hint::black_box(42929_u16), std::hint::black_box(10926199855937587033_u64));
                
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [isize; 7],
fld1: u64,

},
Variant1{
fld0: *mut i32,
fld1: u8,
fld2: [u64; 6],
fld3: f32,
fld4: *mut f32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u16,
fld1: char,
fld2: *mut *mut i32,
fld3: [isize; 7],
fld4: *mut u64,
fld5: f64,
fld6: i64,
fld7: (*mut i32, *mut *mut i32, u128, *const bool),

},
Variant1{
fld0: [isize; 7],
fld1: ((f64,),),

},
Variant2{
fld0: Adt44,
fld1: [usize; 8],
fld2: *const u8,
fld3: *mut *const u8,
fld4: i16,
fld5: [isize; 7],
fld6: ((f64,),),
fld7: i128,

}}
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
fld0: *mut u64,
fld1: ((*mut i32, *mut *mut i32, u128, *const bool), i128, u128),
fld2: ((f64,),),
fld3: Adt45,
fld4: [char; 2],
fld5: [u64; 6],
fld6: i64,
fld7: usize,

},
Variant2{
fld0: i128,
fld1: (bool, usize),

},
Variant3{
fld0: [usize; 5],
fld1: *mut *mut i32,
fld2: *const bool,
fld3: i8,
fld4: u8,
fld5: Adt45,
fld6: usize,
fld7: (*mut i32, *mut *mut i32, u128, *const bool),

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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: u16,
fld2: i32,
fld3: [i64; 6],

},
Variant1{
fld0: f64,
fld1: [usize; 8],
fld2: ([i64; 6], u8),
fld3: *mut i32,
fld4: (f64, [char; 2], u32),
fld5: u32,
fld6: ((bool, usize), u32, isize, ((isize,), i16)),
fld7: *mut f32,

},
Variant2{
fld0: (f64,),
fld1: f64,

},
Variant3{
fld0: [usize; 5],
fld1: Adt45,
fld2: *const bool,
fld3: i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const u8,
fld1: char,

},
Variant1{
fld0: bool,
fld1: *mut u64,
fld2: isize,
fld3: [isize; 7],
fld4: (bool, usize),
fld5: u32,
fld6: ([i64; 6], u8),
fld7: Adt47,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
fld0: ((isize,), i16),
fld1: u128,
fld2: isize,
fld3: Adt46,
fld4: f32,
fld5: *const bool,

},
Variant1{
fld0: [i64; 6],
fld1: *const u8,
fld2: Adt46,
fld3: [u64; 6],
fld4: ([i64; 6], u8),
fld5: (*mut i32, *mut *mut i32, u128, *const bool),
fld6: *const bool,
fld7: u128,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt45,
fld1: i64,
fld2: Adt44,
fld3: ((isize,), i16),

},
Variant1{
fld0: f32,
fld1: [isize; 7],
fld2: i64,
fld3: (bool, usize),
fld4: *mut *mut i32,
fld5: u16,

},
Variant2{
fld0: Adt44,
fld1: char,
fld2: f32,
fld3: i16,

},
Variant3{
fld0: (f64,),
fld1: ((*mut i32, *mut *mut i32, u128, *const bool), i128, u128),
fld2: [usize; 5],
fld3: (((f64,),), i128, *mut f32, i8, [usize; 5]),
fld4: [usize; 8],
fld5: Adt45,
fld6: Adt44,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: ((*mut i32, *mut *mut i32, u128, *const bool), i128, u128),
fld1: (bool, usize),
fld2: isize,
fld3: (f64, [char; 2], u32),
fld4: Adt48,
fld5: ([i64; 6], u8),
fld6: i64,
fld7: u128,

},
Variant1{
fld0: ((bool, usize), u32, isize, ((isize,), i16)),

},
Variant2{
fld0: (isize,),
fld1: *mut *const u8,
fld2: f32,
fld3: [usize; 8],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: Adt44,
fld1: [u64; 6],
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: (f64, [char; 2], u32),
fld1: (f64,),
fld2: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16),
fld3: Adt51,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: Adt52,
fld1: char,
fld2: *const u8,
fld3: Adt49,
fld4: [char; 2],
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt52,

},
Variant1{
fld0: Adt46,
fld1: [usize; 8],
fld2: u128,
fld3: [i64; 6],
fld4: ((f64,),),
fld5: *mut f32,

},
Variant2{
fld0: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16),
fld1: *mut *mut i32,
fld2: ((f64,),),
fld3: u128,
fld4: *mut *const u8,
fld5: [i64; 6],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: f64,
fld1: *const bool,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: i128,
fld1: Adt51,
fld2: u64,
fld3: [char; 2],
fld4: i16,
fld5: [i64; 6],

},
Variant1{
fld0: Adt55,
fld1: (isize,),

},
Variant2{
fld0: f32,
fld1: usize,
fld2: Adt47,
fld3: Adt49,
fld4: *mut f32,
fld5: ((f64,),),
fld6: u8,
fld7: [char; 2],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt50,

},
Variant1{
fld0: *mut *const u8,
fld1: i16,
fld2: Adt44,

},
Variant2{
fld0: [char; 2],
fld1: char,
fld2: (((f64,),), i128, *mut f32, i8, [usize; 5]),
fld3: Adt55,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: u128,

},
Variant1{
fld0: [i64; 6],
fld1: Adt46,
fld2: [isize; 7],
fld3: i8,
fld4: *mut *mut i32,

},
Variant2{
fld0: u128,
fld1: Adt56,
fld2: usize,
fld3: Adt53,
fld4: [i64; 6],

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: [usize; 5],
fld1: [i64; 6],

},
Variant1{
fld0: Adt57,
fld1: char,
fld2: u8,
fld3: ((bool, usize), u32, isize, ((isize,), i16)),

},
Variant2{
fld0: [usize; 8],
fld1: *mut f32,
fld2: u128,
fld3: ((isize,), (*mut i32, *mut *mut i32, u128, *const bool), i16),
fld4: i16,
fld5: i32,

}}

