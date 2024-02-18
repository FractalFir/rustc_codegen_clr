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
pub fn fn0(mut _1: u16,mut _2: u128,mut _3: u32,mut _4: u8) -> i8 {
mir! {
type RET = i8;
let _5: bool;
let _6: [i64; 2];
let _7: isize;
let _8: usize;
let _9: *mut &'static i32;
let _10: ([bool; 7], Adt79, u32, &'static char);
let _11: Adt58;
let _12: [u8; 8];
let _13: ((usize, *const i8, f32, Adt22),);
let _14: u8;
let _15: i128;
let _16: f32;
let _17: [i32; 2];
let _18: char;
let _19: *mut usize;
let _20: usize;
let _21: &'static *const f64;
let _22: &'static [char; 3];
let _23: u32;
let _24: [bool; 1];
let _25: u8;
let _26: u16;
let _27: char;
let _28: i8;
let _29: [i32; 2];
let _30: isize;
let _31: *mut *mut char;
let _32: Adt58;
let _33: (u16,);
let _34: bool;
let _35: char;
let _36: *const f64;
let _37: f32;
let _38: i32;
let _39: isize;
let _40: ();
let _41: ();
{
RET = 64104070574275488207851482217770115055_i128 as i8;
_1 = 21451_u16 - 28273_u16;
_2 = !267782652577650043184901727525223215648_u128;
_4 = 33_u8 ^ 107_u8;
_7 = !(-80_isize);
_1 = 58712_u16;
_3 = !1619766106_u32;
RET = (-128_i8) << _4;
RET = -54_i8;
RET = _1 as i8;
_5 = true;
_4 = 3_u8 | 167_u8;
_3 = 1740356501_u32;
_1 = !19873_u16;
_7 = 10_isize << RET;
_4 = 92_u8;
_7 = 9223372036854775807_isize;
match _4 {
0 => bb1,
1 => bb2,
92 => bb4,
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
RET = -95_i8;
_6 = [4867026899983841871_i64,(-3828117795381756752_i64)];
_2 = !51601590178205527157434681960647992970_u128;
RET = 33_i8;
_2 = !22379054453960883114520330117423429137_u128;
_7 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_1 = !27477_u16;
RET = 121_i8;
_4 = !233_u8;
RET = -14_i8;
_3 = !4230808237_u32;
RET = 90596026158837642400318469812706323536_i128 as i8;
_5 = _1 > _1;
_5 = !false;
_4 = 5024_i16 as u8;
_8 = !7752324339696702306_usize;
_6 = [4111191350970102350_i64,(-3120498045638955216_i64)];
_8 = !2_usize;
Goto(bb5)
}
bb5 = {
_1 = !17114_u16;
RET = _4 as i8;
_5 = true;
_10.1.fld2.fld0.fld1 = '\u{f3c94}';
_10.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_10.1.fld1.3.fld1);
_10.1.fld2.fld0.fld3 = _3 ^ _3;
_10.1.fld1.1 = core::ptr::addr_of!(RET);
Call(_10.1.fld1.3.fld0 = core::intrinsics::transmute(_6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10.1.fld2.fld0.fld3 = _3 | _3;
_10.1.fld1.3.fld0 = -161820414122545405443583588054168534968_i128;
_10.1.fld1.1 = core::ptr::addr_of!(RET);
Goto(bb7)
}
bb7 = {
_10.1.fld1.3.fld5 = _7 as i32;
_10.1.fld4 = !22992_i16;
_10.1.fld1.0 = _8 & _8;
_10.0 = [_5,_5,_5,_5,_5,_5,_5];
_10.1.fld1.0 = _7 as usize;
_14 = !_4;
_10.1.fld1.2 = _10.1.fld1.0 as f32;
_10.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_10.1.fld2.fld0.fld1);
_13.0.3.fld6 = _10.1.fld1.3.fld0 as f64;
_13.0.3.fld4 = Move(_10.1.fld2.fld0.fld4);
_13.0.3.fld5 = _10.1.fld1.3.fld5 & _10.1.fld1.3.fld5;
Call(_15 = fn1(Move(_13.0.3.fld4), _1, _10.1.fld2.fld0.fld3, _7, _6, _6, _8, _1, _13.0.3.fld5, _10.1.fld1.2, _13.0.3.fld5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_10.1.fld2.fld0.fld1);
_10.1.fld1.3.fld4 = Move(_10.1.fld2.fld0.fld4);
_10.1.fld1.3.fld6 = _13.0.3.fld6;
_10.1.fld1.0 = 8416777120284855139_i64 as usize;
_10.3 = &_10.1.fld2.fld0.fld1;
_10.1.fld3 = [_7,_7];
_10.1.fld1.3.fld0 = _10.1.fld1.3.fld6 as i128;
_10.1.fld1.3.fld0 = !_15;
_10.2 = _10.1.fld2.fld0.fld3;
_13.0.3.fld1 = _10.1.fld2.fld0.fld1;
_10.1.fld0 = _3 | _10.2;
_16 = -_10.1.fld1.2;
_17 = [_13.0.3.fld5,_10.1.fld1.3.fld5];
_10.1.fld1.0 = _8 + _8;
_13.0.3.fld6 = _8 as f64;
Goto(bb9)
}
bb9 = {
_10.1.fld1.2 = _16 * _16;
_13.0.3.fld3 = !_10.2;
_17 = [_10.1.fld1.3.fld5,_10.1.fld1.3.fld5];
_13.0.3.fld4 = core::ptr::addr_of_mut!(_10.1.fld2.fld0.fld1);
_1 = 56028_u16;
_10.1.fld4 = (-32536_i16) << _10.1.fld0;
_5 = false;
_17 = [_13.0.3.fld5,_13.0.3.fld5];
_13.0.3.fld5 = _10.1.fld1.0 as i32;
_13.0.3.fld2 = _10.1.fld4 as usize;
_18 = _13.0.3.fld1;
_10.1.fld2.fld0.fld1 = _18;
_10.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_10.1.fld2.fld0.fld1);
_10.1.fld2.fld0.fld2 = _13.0.3.fld2;
_10.1.fld5 = [_14,_4,_4,_14,_14,_4,_14,_14];
_13.0.3 = Adt22 { fld0: _15,fld1: _18,fld2: _10.1.fld1.0,fld3: _10.1.fld0,fld4: Move(_10.1.fld1.3.fld4),fld5: _10.1.fld1.3.fld5,fld6: _10.1.fld1.3.fld6 };
_10.1.fld0 = _3 << _10.1.fld4;
_10.1.fld2.fld0.fld5 = -_13.0.3.fld5;
_3 = _13.0.3.fld3 & _10.1.fld0;
_10.1.fld4 = 25343_i16;
_10.1.fld2.fld0.fld6 = -_13.0.3.fld6;
_20 = _7 as usize;
_10.1.fld1.3 = Adt22 { fld0: _15,fld1: _10.1.fld2.fld0.fld1,fld2: _10.1.fld1.0,fld3: _10.2,fld4: Move(_13.0.3.fld4),fld5: _10.1.fld2.fld0.fld5,fld6: _10.1.fld2.fld0.fld6 };
Goto(bb10)
}
bb10 = {
_18 = _13.0.3.fld1;
_1 = 53763_u16;
_8 = !_10.1.fld1.3.fld2;
_10.1.fld2.fld0 = Move(_10.1.fld1.3);
_10.1.fld1.2 = -_16;
_3 = !_10.1.fld2.fld0.fld3;
_10.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_18);
_13.0 = (_10.1.fld1.0, Move(_10.1.fld1.1), _16, Move(_10.1.fld2.fld0));
_13.0.3.fld0 = !_15;
_10.1.fld1.3.fld4 = Move(_13.0.3.fld4);
_10.1.fld2.fld0.fld1 = _18;
match _10.1.fld4 {
25343 => bb11,
_ => bb3
}
}
bb11 = {
_10.1.fld1.3.fld5 = _13.0.3.fld5 ^ _13.0.3.fld5;
_10.0 = [_5,_5,_5,_5,_5,_5,_5];
_13.0.3.fld2 = _13.0.0 << _3;
_10.1.fld1.1 = core::ptr::addr_of!(RET);
match _10.1.fld4 {
0 => bb6,
25343 => bb12,
_ => bb9
}
}
bb12 = {
_14 = !_4;
_10.3 = &_13.0.3.fld1;
_7 = 9223372036854775807_isize >> _10.1.fld0;
_10.1.fld1.3.fld1 = _18;
_10.1.fld1.1 = core::ptr::addr_of!(RET);
_5 = !true;
_10.1.fld2.fld0.fld4 = Move(_10.1.fld1.3.fld4);
_10.1.fld1.3.fld0 = _13.0.3.fld0 * _13.0.3.fld0;
_10.1.fld2.fld0.fld1 = _13.0.3.fld1;
_10.1.fld2.fld0.fld0 = _10.1.fld1.3.fld0;
_19 = core::ptr::addr_of_mut!(_10.1.fld1.3.fld2);
_10.1.fld5 = [_14,_14,_14,_4,_14,_4,_14,_14];
(*_19) = _13.0.3.fld2;
_13.0.3.fld5 = !_10.1.fld1.3.fld5;
_10.1.fld2.fld0.fld1 = _13.0.3.fld1;
_10.1.fld1.3.fld3 = _10.1.fld4 as u32;
_10.1.fld1.3.fld1 = _10.1.fld2.fld0.fld1;
_13.0.3.fld0 = !_10.1.fld1.3.fld0;
_13.0.2 = _10.1.fld1.2;
Goto(bb13)
}
bb13 = {
_7 = (-9223372036854775808_isize);
_10.1.fld1.3.fld5 = _13.0.3.fld5;
_19 = core::ptr::addr_of_mut!(_20);
_10.1.fld2.fld0.fld6 = -_13.0.3.fld6;
_3 = _13.0.3.fld3;
_10.1.fld0 = 4136258386289427113_i64 as u32;
_10.1.fld2.fld0.fld2 = (*_19);
_13.0.3.fld1 = _18;
_10.1.fld1.3.fld0 = -_13.0.3.fld0;
_10.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_27);
_10.1.fld0 = _10.1.fld1.3.fld3 | _13.0.3.fld3;
_30 = _7;
_27 = _10.1.fld2.fld0.fld1;
_28 = RET & RET;
_23 = !_10.1.fld0;
_20 = _15 as usize;
_10.1.fld2.fld0.fld5 = _2 as i32;
_10.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_10.1.fld1.3.fld1);
_10.2 = _13.0.3.fld3;
_10.1.fld1.3.fld0 = _4 as i128;
_10.1.fld1.3.fld3 = _10.1.fld0;
Call(_10.1.fld2.fld0.fld2 = fn17(_20, _10.1.fld2.fld0.fld0, _6, Move(_10.1.fld2.fld0.fld4), _10.1.fld1.3.fld3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_26 = !_1;
_10.3 = &_13.0.3.fld1;
_10.1.fld1.2 = _10.1.fld4 as f32;
_13.0.3.fld3 = !_23;
_10.1.fld2.fld0 = Adt22 { fld0: _10.1.fld1.3.fld0,fld1: _18,fld2: _10.1.fld1.3.fld2,fld3: _10.1.fld1.3.fld3,fld4: Move(_10.1.fld1.3.fld4),fld5: _13.0.3.fld5,fld6: _13.0.3.fld6 };
_10.1.fld1.3.fld6 = -_13.0.3.fld6;
_15 = _13.0.3.fld0 << _13.0.3.fld5;
_1 = _26 & _26;
_10.1.fld1.3.fld5 = _13.0.3.fld5;
_10.1.fld2.fld0.fld6 = _10.1.fld1.3.fld6;
_15 = _13.0.3.fld0 + _10.1.fld1.3.fld0;
_13.0.3.fld0 = _5 as i128;
_34 = !_5;
_28 = RET;
_10.1.fld1 = (_10.1.fld2.fld0.fld2, Move(_13.0.1), _16, Move(_10.1.fld2.fld0));
_10.1.fld2.fld0.fld5 = !_13.0.3.fld5;
_36 = core::ptr::addr_of!(_10.1.fld1.3.fld6);
_10.1.fld2.fld0.fld3 = !_10.1.fld0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(0_usize, 1_usize, Move(_1), 8_usize, Move(_8), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(0_usize, 20_usize, Move(_20), 18_usize, Move(_18), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(0_usize, 30_usize, Move(_30), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: *mut char,mut _2: u16,mut _3: u32,mut _4: isize,mut _5: [i64; 2],mut _6: [i64; 2],mut _7: usize,mut _8: u16,mut _9: i32,mut _10: f32,mut _11: i32) -> i128 {
mir! {
type RET = i128;
let _12: *mut char;
let _13: [i32; 1];
let _14: (i32, usize, i8);
let _15: [u32; 2];
let _16: isize;
let _17: *mut u128;
let _18: *mut i64;
let _19: (isize, u32);
let _20: char;
let _21: (u16,);
let _22: (&'static bool, u128);
let _23: f32;
let _24: [bool; 1];
let _25: u128;
let _26: i128;
let _27: [isize; 2];
let _28: [i32; 1];
let _29: f64;
let _30: bool;
let _31: i16;
let _32: isize;
let _33: [u8; 8];
let _34: (*mut char, Adt38, (i32, usize, i8));
let _35: ();
let _36: ();
{
_11 = -_9;
_8 = _3 as u16;
RET = 2692257702271638440131911381988891712_i128;
_6 = [2170301174637462947_i64,(-6155489808278044773_i64)];
_12 = Move(_1);
_4 = false as isize;
_1 = Move(_12);
_12 = Move(_1);
Goto(bb1)
}
bb1 = {
_10 = _7 as f32;
_2 = 181_u8 as u16;
_1 = Move(_12);
_3 = 1515644083_u32 * 1446409595_u32;
_8 = 4951816194649312779_i64 as u16;
_9 = !_11;
_13 = [_11];
_14.2 = 15_i8;
_14.1 = _7 | _7;
_13 = [_11];
_6 = [6926032793364182910_i64,(-2525198030526142185_i64)];
_12 = Move(_1);
_4 = (-9223372036854775808_isize) ^ 50_isize;
RET = 18247706212262041971_u64 as i128;
_14.2 = -(-65_i8);
_7 = _14.1 & _14.1;
_14.0 = _8 as i32;
_14.2 = 99_i8 >> _9;
_7 = _14.1 >> _9;
RET = 154_u8 as i128;
_14 = (_11, _7, (-20_i8));
_7 = !_14.1;
_14 = (_11, _7, 50_i8);
_14 = (_11, _7, 107_i8);
_15 = [_3,_3];
Call(_12 = fn2(_14.2, _14.1, _13, _14.2, _14.2, _5, _14.0, _14, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = _4 as f32;
RET = _3 as i128;
RET = 84558916345855591129931639448110320727_i128 * 112733440920548619885648618605421558151_i128;
_11 = _9;
_13 = [_11];
_10 = _4 as f32;
_14.2 = (-9607_i16) as i8;
_2 = _8 * _8;
_10 = _9 as f32;
_6 = _5;
_14.2 = _8 as i8;
_14 = (_11, _7, 34_i8);
_7 = RET as usize;
_5 = _6;
_8 = !_2;
_7 = _14.1;
_4 = !9223372036854775807_isize;
_16 = _4 << _8;
_14.1 = _7;
_13 = [_9];
_1 = Move(_12);
_14.0 = !_9;
_16 = _14.2 as isize;
Goto(bb3)
}
bb3 = {
_12 = core::ptr::addr_of_mut!(_20);
_10 = _8 as f32;
(*_12) = '\u{a8781}';
_11 = 124757862655946527751220673593714474415_u128 as i32;
_3 = !2521292817_u32;
RET = !106234076405981080750740230512262601402_i128;
_5 = [8260811457864916549_i64,(-5435309932134616548_i64)];
_13 = [_14.0];
_7 = 12460610858256998822003032736004176232_u128 as usize;
_14.2 = 5556759678259118937_u64 as i8;
_6 = [4015125955962474949_i64,(-7680349393599682294_i64)];
_20 = '\u{5c5d2}';
Goto(bb4)
}
bb4 = {
_1 = core::ptr::addr_of_mut!((*_12));
_9 = _14.0;
(*_12) = '\u{d3b7}';
_13 = [_9];
_14 = (_11, _7, (-121_i8));
_7 = _14.1;
_3 = 2968564327_u32;
Call(_11 = fn15(_16, _2, _16, _13, _2, _16, (*_1), _16, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = [_11];
_3 = 18004398_u32 & 1533009017_u32;
RET = (-137495154246277293204934737914385486593_i128) * 119178114214246047855888014494474163537_i128;
_21.0 = _14.2 as u16;
_16 = false as isize;
_13 = [_11];
_10 = 9351247198759546544_u64 as f32;
_11 = _7 as i32;
_12 = Move(_1);
_14 = (_9, _7, (-17_i8));
match _14.2 {
0 => bb3,
1 => bb4,
2 => bb6,
340282366920938463463374607431768211439 => bb8,
_ => bb7
}
}
bb6 = {
_10 = _4 as f32;
RET = _3 as i128;
RET = 84558916345855591129931639448110320727_i128 * 112733440920548619885648618605421558151_i128;
_11 = _9;
_13 = [_11];
_10 = _4 as f32;
_14.2 = (-9607_i16) as i8;
_2 = _8 * _8;
_10 = _9 as f32;
_6 = _5;
_14.2 = _8 as i8;
_14 = (_11, _7, 34_i8);
_7 = RET as usize;
_5 = _6;
_8 = !_2;
_7 = _14.1;
_4 = !9223372036854775807_isize;
_16 = _4 << _8;
_14.1 = _7;
_13 = [_9];
_1 = Move(_12);
_14.0 = !_9;
_16 = _14.2 as isize;
Goto(bb3)
}
bb7 = {
_10 = _7 as f32;
_2 = 181_u8 as u16;
_1 = Move(_12);
_3 = 1515644083_u32 * 1446409595_u32;
_8 = 4951816194649312779_i64 as u16;
_9 = !_11;
_13 = [_11];
_14.2 = 15_i8;
_14.1 = _7 | _7;
_13 = [_11];
_6 = [6926032793364182910_i64,(-2525198030526142185_i64)];
_12 = Move(_1);
_4 = (-9223372036854775808_isize) ^ 50_isize;
RET = 18247706212262041971_u64 as i128;
_14.2 = -(-65_i8);
_7 = _14.1 & _14.1;
_14.0 = _8 as i32;
_14.2 = 99_i8 >> _9;
_7 = _14.1 >> _9;
RET = 154_u8 as i128;
_14 = (_11, _7, (-20_i8));
_7 = !_14.1;
_14 = (_11, _7, 50_i8);
_14 = (_11, _7, 107_i8);
_15 = [_3,_3];
Call(_12 = fn2(_14.2, _14.1, _13, _14.2, _14.2, _5, _14.0, _14, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
RET = _20 as i128;
_4 = _16 | _16;
_9 = _14.2 as i32;
_14.1 = !_7;
_6 = _5;
_10 = _3 as f32;
_7 = _14.1;
_9 = -_11;
_16 = !_4;
_14 = (_9, _7, (-7_i8));
_10 = RET as f32;
_23 = _10;
_8 = !_2;
_25 = 296392675441901479304187885602748543197_u128 - 225140520872771748387235600478606482546_u128;
_25 = 318564835163963237632684155845209867961_u128 | 315188678684611349893503146328002242460_u128;
_22.1 = !_25;
_1 = Move(_12);
_24 = [true];
_9 = _14.0 ^ _11;
_8 = _21.0 | _2;
_24 = [true];
_19 = (_4, _3);
_10 = -_23;
_24 = [false];
_16 = _19.0;
_28 = _13;
Call(_19.0 = core::intrinsics::transmute(_4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = _23;
_26 = RET >> _3;
_8 = 6_u8 as u16;
_5 = _6;
_10 = _14.2 as f32;
_13 = [_14.0];
_19.1 = _3 ^ _3;
_1 = core::ptr::addr_of_mut!(_20);
_28 = [_11];
_13 = _28;
_2 = _14.0 as u16;
Call(_14.2 = fn16(_19.0, _28, _15, _15, _19, _19.0, _14.1, _19, _9, _19, _20), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19.1 = _3 | _3;
_14.1 = _7;
RET = _26;
RET = -_26;
_14 = (_9, _7, (-91_i8));
_17 = core::ptr::addr_of_mut!(_25);
(*_17) = _22.1;
_31 = 24256_i16;
_29 = (*_17) as f64;
_8 = !_21.0;
_3 = (*_1) as u32;
_25 = _22.1 & _22.1;
_21 = (_2,);
_8 = _26 as u16;
_10 = _22.1 as f32;
RET = _9 as i128;
_14 = (_11, _7, (-77_i8));
_22.0 = &_30;
_29 = 892469243416893840_u64 as f64;
_15 = [_19.1,_19.1];
_10 = -_23;
_17 = core::ptr::addr_of_mut!((*_17));
_29 = (*_17) as f64;
_23 = _14.2 as f32;
_20 = '\u{b15e}';
match _14.2 {
0 => bb1,
1 => bb6,
2 => bb8,
3 => bb5,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607431768211379 => bb15,
_ => bb14
}
}
bb11 = {
_10 = _7 as f32;
_2 = 181_u8 as u16;
_1 = Move(_12);
_3 = 1515644083_u32 * 1446409595_u32;
_8 = 4951816194649312779_i64 as u16;
_9 = !_11;
_13 = [_11];
_14.2 = 15_i8;
_14.1 = _7 | _7;
_13 = [_11];
_6 = [6926032793364182910_i64,(-2525198030526142185_i64)];
_12 = Move(_1);
_4 = (-9223372036854775808_isize) ^ 50_isize;
RET = 18247706212262041971_u64 as i128;
_14.2 = -(-65_i8);
_7 = _14.1 & _14.1;
_14.0 = _8 as i32;
_14.2 = 99_i8 >> _9;
_7 = _14.1 >> _9;
RET = 154_u8 as i128;
_14 = (_11, _7, (-20_i8));
_7 = !_14.1;
_14 = (_11, _7, 50_i8);
_14 = (_11, _7, 107_i8);
_15 = [_3,_3];
Call(_12 = fn2(_14.2, _14.1, _13, _14.2, _14.2, _5, _14.0, _14, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_1 = core::ptr::addr_of_mut!((*_12));
_9 = _14.0;
(*_12) = '\u{d3b7}';
_13 = [_9];
_14 = (_11, _7, (-121_i8));
_7 = _14.1;
_3 = 2968564327_u32;
Call(_11 = fn15(_16, _2, _16, _13, _2, _16, (*_1), _16, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_12 = core::ptr::addr_of_mut!(_20);
_10 = _8 as f32;
(*_12) = '\u{a8781}';
_11 = 124757862655946527751220673593714474415_u128 as i32;
_3 = !2521292817_u32;
RET = !106234076405981080750740230512262601402_i128;
_5 = [8260811457864916549_i64,(-5435309932134616548_i64)];
_13 = [_14.0];
_7 = 12460610858256998822003032736004176232_u128 as usize;
_14.2 = 5556759678259118937_u64 as i8;
_6 = [4015125955962474949_i64,(-7680349393599682294_i64)];
_20 = '\u{5c5d2}';
Goto(bb4)
}
bb14 = {
_10 = _4 as f32;
RET = _3 as i128;
RET = 84558916345855591129931639448110320727_i128 * 112733440920548619885648618605421558151_i128;
_11 = _9;
_13 = [_11];
_10 = _4 as f32;
_14.2 = (-9607_i16) as i8;
_2 = _8 * _8;
_10 = _9 as f32;
_6 = _5;
_14.2 = _8 as i8;
_14 = (_11, _7, 34_i8);
_7 = RET as usize;
_5 = _6;
_8 = !_2;
_7 = _14.1;
_4 = !9223372036854775807_isize;
_16 = _4 << _8;
_14.1 = _7;
_13 = [_9];
_1 = Move(_12);
_14.0 = !_9;
_16 = _14.2 as isize;
Goto(bb3)
}
bb15 = {
_9 = !_14.0;
_23 = 88_u8 as f32;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(1_usize, 5_usize, Move(_5), 26_usize, Move(_26), 2_usize, Move(_2), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(1_usize, 21_usize, Move(_21), 20_usize, Move(_20), 16_usize, Move(_16), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(1_usize, 14_usize, Move(_14), 24_usize, Move(_24), 36_usize, _36, 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i8,mut _2: usize,mut _3: [i32; 1],mut _4: i8,mut _5: i8,mut _6: [i64; 2],mut _7: i32,mut _8: (i32, usize, i8),mut _9: [i64; 2]) -> *mut char {
mir! {
type RET = *mut char;
let _10: isize;
let _11: f64;
let _12: f64;
let _13: isize;
let _14: u128;
let _15: isize;
let _16: (i128,);
let _17: *mut u128;
let _18: u8;
let _19: [i64; 2];
let _20: *mut char;
let _21: [i32; 2];
let _22: (i8, (i32, usize, i8), (u128,), u128);
let _23: &'static [u64; 2];
let _24: f32;
let _25: isize;
let _26: bool;
let _27: bool;
let _28: i128;
let _29: i128;
let _30: (isize, u32);
let _31: (u16,);
let _32: (&'static bool, u128);
let _33: f32;
let _34: (i8, (i32, usize, i8), (u128,), u128);
let _35: isize;
let _36: *mut usize;
let _37: f32;
let _38: ([bool; 7], Adt79, u32, &'static char);
let _39: u32;
let _40: [i64; 2];
let _41: f32;
let _42: (isize, u32);
let _43: (usize, *const i8, f32, Adt22);
let _44: *mut (usize, *const i8, f32, Adt22);
let _45: f32;
let _46: i32;
let _47: [i32; 2];
let _48: f64;
let _49: &'static [char; 3];
let _50: &'static bool;
let _51: f32;
let _52: ();
let _53: ();
{
_8 = (_7, _2, _1);
_4 = _1 << _7;
_8.0 = _7 - _7;
_3 = [_8.0];
Call(_3 = fn3(_8.2, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !_8.1;
_2 = _8.1;
_8.1 = _2;
_5 = _1;
_5 = -_1;
_4 = _5;
_10 = (-54715807821911479_i64) as isize;
_8.0 = !_7;
_10 = 35_isize + (-9223372036854775808_isize);
_4 = _1;
_8.2 = 116094458_u32 as i8;
_12 = 107433774926725310942364277337995695181_i128 as f64;
_1 = _4;
_8 = (_7, _2, _5);
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
107 => bb8,
_ => bb7
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
_13 = _10 & _10;
_8 = (_7, _2, _5);
_14 = 126_u8 as u128;
_12 = _14 as f64;
_12 = (-7046_i16) as f64;
_2 = !_8.1;
_8.2 = _4 - _5;
_8.1 = !_2;
_15 = _13;
_8 = (_7, _2, _1);
_5 = _12 as i8;
_11 = _12 * _12;
_10 = _15 >> _4;
_1 = _8.2;
_8.2 = -_1;
_8.1 = !_2;
_2 = _8.1;
_8.2 = (-2746272074896156210_i64) as i8;
_16 = ((-57269933402580107934659228849110825799_i128),);
_14 = _2 as u128;
_12 = 5978332987381816577_i64 as f64;
_3 = [_7];
_2 = _8.1 ^ _8.1;
_1 = _4;
_14 = !61858099548668740800908983270984092270_u128;
match _4 {
0 => bb9,
1 => bb10,
107 => bb12,
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
_2 = !_8.1;
_2 = _8.1;
_8.1 = _2;
_5 = _1;
_5 = -_1;
_4 = _5;
_10 = (-54715807821911479_i64) as isize;
_8.0 = !_7;
_10 = 35_isize + (-9223372036854775808_isize);
_4 = _1;
_8.2 = 116094458_u32 as i8;
_12 = 107433774926725310942364277337995695181_i128 as f64;
_1 = _4;
_8 = (_7, _2, _5);
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
107 => bb8,
_ => bb7
}
}
bb12 = {
_4 = _5;
_5 = -_1;
_16.0 = 70792035511302750587499758523743349008_i128 << _7;
_8.2 = _5 * _1;
_4 = _5;
_16.0 = _14 as i128;
_12 = _11 - _11;
_8.2 = !_4;
_9 = [(-6246911976797807820_i64),(-6354980591349372540_i64)];
_18 = 188_u8 * 176_u8;
_11 = _12;
_10 = _15;
_9 = [(-3484878819165506027_i64),3223994251413267416_i64];
_17 = core::ptr::addr_of_mut!(_14);
_9 = [146194061344999059_i64,(-6810310008117706743_i64)];
(*_17) = 256100189794357406808897325954845640134_u128;
_8.1 = _2;
(*_17) = 103569391350975976768445286649571471174_u128;
_8.2 = -_5;
_8.1 = _2;
match _1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
107 => bb19,
_ => bb18
}
}
bb13 = {
_2 = !_8.1;
_2 = _8.1;
_8.1 = _2;
_5 = _1;
_5 = -_1;
_4 = _5;
_10 = (-54715807821911479_i64) as isize;
_8.0 = !_7;
_10 = 35_isize + (-9223372036854775808_isize);
_4 = _1;
_8.2 = 116094458_u32 as i8;
_12 = 107433774926725310942364277337995695181_i128 as f64;
_1 = _4;
_8 = (_7, _2, _5);
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
107 => bb8,
_ => bb7
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_13 = _10 & _10;
_8 = (_7, _2, _5);
_14 = 126_u8 as u128;
_12 = _14 as f64;
_12 = (-7046_i16) as f64;
_2 = !_8.1;
_8.2 = _4 - _5;
_8.1 = !_2;
_15 = _13;
_8 = (_7, _2, _1);
_5 = _12 as i8;
_11 = _12 * _12;
_10 = _15 >> _4;
_1 = _8.2;
_8.2 = -_1;
_8.1 = !_2;
_2 = _8.1;
_8.2 = (-2746272074896156210_i64) as i8;
_16 = ((-57269933402580107934659228849110825799_i128),);
_14 = _2 as u128;
_12 = 5978332987381816577_i64 as f64;
_3 = [_7];
_2 = _8.1 ^ _8.1;
_1 = _4;
_14 = !61858099548668740800908983270984092270_u128;
match _4 {
0 => bb9,
1 => bb10,
107 => bb12,
_ => bb11
}
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_9 = [(-4905663112954064843_i64),2165187401386390553_i64];
_3 = [_7];
_1 = !_8.2;
_14 = !87523331183391325899141406022032205047_u128;
_19 = _6;
_3 = [_8.0];
_12 = _11 + _11;
_22.0 = _18 as i8;
_13 = -_15;
_14 = 2932817205923096244486745418148471719_u128 >> _5;
_14 = 188451450410315325175957066676464383505_u128;
_22.1.2 = _5 >> _16.0;
_22.1 = (_8.0, _2, _4);
_22.2.0 = !(*_17);
_22.0 = _1;
_17 = core::ptr::addr_of_mut!(_22.3);
_3 = [_8.0];
_3 = [_7];
_15 = _13 >> _5;
_8.2 = _5;
_14 = _5 as u128;
_1 = _12 as i8;
Goto(bb20)
}
bb20 = {
_8.1 = !_2;
_22.3 = _22.2.0;
_12 = _11;
_22.2.0 = _22.3 + _14;
_22.1.1 = !_2;
_22.1 = (_7, _8.1, _8.2);
_8.2 = -_4;
_18 = 9892757264461685262_u64 as u8;
_22.2 = (_14,);
(*_17) = (-3301530124814663000_i64) as u128;
(*_17) = _22.2.0;
_26 = false;
(*_17) = _22.2.0;
_2 = !_8.1;
_8 = (_7, _2, _22.0);
_21 = [_8.0,_7];
_24 = _8.0 as f32;
Goto(bb21)
}
bb21 = {
_18 = 227_u8 >> _22.2.0;
_2 = _26 as usize;
_17 = core::ptr::addr_of_mut!(_14);
_8 = _22.1;
_9 = [8653657085612607153_i64,(-258998551313590632_i64)];
_22.1 = _8;
_2 = !_8.1;
_4 = _5;
_30 = (_15, 3190345063_u32);
_29 = _16.0 >> (*_17);
_1 = -_22.0;
_30 = (_13, 3485325354_u32);
_11 = _12;
_29 = _16.0 & _16.0;
Goto(bb22)
}
bb22 = {
_22.1 = (_7, _2, _22.0);
_8 = _22.1;
_24 = _18 as f32;
_16 = (_29,);
_10 = _15;
_6 = _9;
_30 = (_15, 1321918438_u32);
_22.1.1 = _24 as usize;
_29 = _16.0 << _13;
_22.1.0 = _8.0 >> _14;
Call(_34.0 = core::intrinsics::transmute(_18), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_34.1.2 = _1 - _34.0;
_9 = [8159491687210104863_i64,5203587359298226469_i64];
_14 = !_22.2.0;
_21 = [_22.1.0,_8.0];
_33 = -_24;
_35 = -_10;
_25 = -_10;
_22.3 = (*_17);
_18 = !137_u8;
_34.2.0 = _22.3;
_8.1 = !_2;
_30.0 = '\u{866de}' as isize;
_8.1 = _22.1.1;
_3 = [_7];
_32.0 = &_26;
_6 = [(-9042826143423348674_i64),(-8512501904499389851_i64)];
_34.1.2 = _18 as i8;
_13 = _25 & _35;
_5 = _22.1.2 >> _10;
_32.1 = !_34.2.0;
_8.2 = -_5;
_3 = [_22.1.0];
Goto(bb24)
}
bb24 = {
_28 = 33514_u16 as i128;
_38.1.fld1.3.fld6 = -_11;
_38.2 = _30.1 | _30.1;
_38.1.fld1.3.fld1 = '\u{c418a}';
_36 = core::ptr::addr_of_mut!(_38.1.fld1.0);
_38.1.fld2.fld0.fld3 = _38.2 / _30.1;
_11 = _12;
_38.1.fld1.3.fld0 = -_29;
_22.2 = (_14,);
_18 = 156_u8 - 15_u8;
_34.3 = _22.3 >> (*_17);
_38.1.fld1.3.fld6 = _11 - _11;
_34.1.2 = _5;
_38.1.fld1.0 = !_8.1;
match _30.1 {
0 => bb4,
1 => bb8,
2 => bb9,
1321918438 => bb26,
_ => bb25
}
}
bb25 = {
_8.1 = !_2;
_22.3 = _22.2.0;
_12 = _11;
_22.2.0 = _22.3 + _14;
_22.1.1 = !_2;
_22.1 = (_7, _8.1, _8.2);
_8.2 = -_4;
_18 = 9892757264461685262_u64 as u8;
_22.2 = (_14,);
(*_17) = (-3301530124814663000_i64) as u128;
(*_17) = _22.2.0;
_26 = false;
(*_17) = _22.2.0;
_2 = !_8.1;
_8 = (_7, _2, _22.0);
_21 = [_8.0,_7];
_24 = _8.0 as f32;
Goto(bb21)
}
bb26 = {
_28 = _29 * _38.1.fld1.3.fld0;
_34.2 = (_14,);
_22.1.2 = (-7458534079503720551_i64) as i8;
_38.1.fld1.1 = core::ptr::addr_of!(_8.2);
_43.3.fld6 = -_38.1.fld1.3.fld6;
_38.1.fld1.2 = _33 - _24;
_38.1.fld1.3.fld5 = _22.1.0;
_43.1 = Move(_38.1.fld1.1);
_25 = _13 | _10;
_35 = _38.1.fld1.3.fld5 as isize;
_20 = core::ptr::addr_of_mut!(_43.3.fld1);
_34 = (_22.0, _22.1, _22.2, (*_17));
_29 = _28 << _13;
_38.0 = [_26,_26,_26,_26,_26,_26,_26];
_4 = _5 << _25;
_16 = (_29,);
match _30.1 {
0 => bb1,
1 => bb5,
2 => bb15,
3 => bb27,
4 => bb28,
1321918438 => bb30,
_ => bb29
}
}
bb27 = {
_2 = !_8.1;
_2 = _8.1;
_8.1 = _2;
_5 = _1;
_5 = -_1;
_4 = _5;
_10 = (-54715807821911479_i64) as isize;
_8.0 = !_7;
_10 = 35_isize + (-9223372036854775808_isize);
_4 = _1;
_8.2 = 116094458_u32 as i8;
_12 = 107433774926725310942364277337995695181_i128 as f64;
_1 = _4;
_8 = (_7, _2, _5);
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
107 => bb8,
_ => bb7
}
}
bb28 = {
Return()
}
bb29 = {
_2 = !_8.1;
_2 = _8.1;
_8.1 = _2;
_5 = _1;
_5 = -_1;
_4 = _5;
_10 = (-54715807821911479_i64) as isize;
_8.0 = !_7;
_10 = 35_isize + (-9223372036854775808_isize);
_4 = _1;
_8.2 = 116094458_u32 as i8;
_12 = 107433774926725310942364277337995695181_i128 as f64;
_1 = _4;
_8 = (_7, _2, _5);
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
107 => bb8,
_ => bb7
}
}
bb30 = {
(*_20) = _38.1.fld1.3.fld1;
_26 = _38.1.fld1.0 != _38.1.fld1.0;
_38.1.fld1.3.fld4 = core::ptr::addr_of_mut!((*_20));
_12 = _43.3.fld6;
_43.3.fld4 = core::ptr::addr_of_mut!(_43.3.fld1);
_38.1.fld5 = [_18,_18,_18,_18,_18,_18,_18,_18];
_43.2 = -_24;
_38.1.fld2.fld0.fld5 = !_8.0;
_7 = -_38.1.fld1.3.fld5;
RET = core::ptr::addr_of_mut!((*_20));
_38.1.fld1.3.fld3 = !_38.1.fld2.fld0.fld3;
_43.3.fld5 = _29 as i32;
_38.1.fld2.fld0.fld6 = _2 as f64;
_38.2 = _30.1;
_38.1.fld3 = [_25,_10];
_50 = &_26;
Goto(bb31)
}
bb31 = {
Call(_52 = dump_var(2_usize, 26_usize, Move(_26), 22_usize, Move(_22), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_52 = dump_var(2_usize, 10_usize, Move(_10), 19_usize, Move(_19), 8_usize, Move(_8), 28_usize, Move(_28)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_52 = dump_var(2_usize, 34_usize, Move(_34), 4_usize, Move(_4), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i8,mut _2: i32) -> [i32; 1] {
mir! {
type RET = [i32; 1];
let _3: u128;
let _4: &'static (*mut &'static i32, (usize, *mut char));
let _5: u8;
let _6: [isize; 2];
let _7: isize;
let _8: *const *mut (usize, *const i8, f32, Adt22);
let _9: ([bool; 7], Adt79, u32, &'static char);
let _10: isize;
let _11: *const f64;
let _12: [bool; 7];
let _13: u8;
let _14: &'static (u128,);
let _15: u16;
let _16: f32;
let _17: isize;
let _18: [i64; 2];
let _19: i8;
let _20: (i32, usize, i8);
let _21: isize;
let _22: u16;
let _23: isize;
let _24: &'static i32;
let _25: i64;
let _26: *mut i32;
let _27: (isize, u32);
let _28: bool;
let _29: &'static [u64; 2];
let _30: bool;
let _31: u8;
let _32: i16;
let _33: i8;
let _34: [isize; 2];
let _35: [u64; 2];
let _36: *mut u128;
let _37: f64;
let _38: isize;
let _39: f64;
let _40: u8;
let _41: &'static (*mut &'static i32, (usize, *mut char));
let _42: u64;
let _43: (usize, *const i8, f32, Adt22);
let _44: f32;
let _45: ();
let _46: ();
{
_1 = 28_i8 & (-40_i8);
RET = [_2];
RET = [_2];
RET = [_2];
_2 = !(-1242141145_i32);
RET = [_2];
_1 = 114_i8;
_2 = 915876430_i32;
_1 = 16_i8;
_3 = 200146254218417636870397973907044369679_u128;
RET = [_2];
_2 = 16988660954008032369_usize as i32;
RET = [_2];
RET = [_2];
_1 = (-1952145795960666463_i64) as i8;
_5 = !5_u8;
_2 = false as i32;
RET = [_2];
_6 = [9223372036854775807_isize,(-86_isize)];
_5 = 2087650216617441618_u64 as u8;
_6 = [9223372036854775807_isize,9223372036854775807_isize];
_1 = (-27_i8) & (-7_i8);
Call(_2 = core::intrinsics::bswap((-413309873_i32)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _2 as u8;
_5 = !166_u8;
RET = [_2];
RET = [_2];
_2 = _1 as i32;
_7 = 1274257402_u32 as isize;
RET = [_2];
_3 = 233011482011954975377121433791083987844_u128;
_3 = 160224990517831563495987941304468360370_u128;
_7 = -(-106_isize);
_1 = 83_i8;
_6 = [_7,_7];
_9.1.fld4 = 15877_i16 + 23319_i16;
_9.1.fld0 = !2829968523_u32;
_9.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_9.1.fld1.3.fld1);
_9.1.fld1.3.fld1 = '\u{377df}';
_3 = 152684348624591281172502375778868520106_u128 >> _7;
_10 = _7;
_9.1.fld1.2 = _10 as f32;
_9.1.fld1.3.fld6 = 3_usize as f64;
_9.1.fld2.fld0.fld5 = _2;
Goto(bb2)
}
bb2 = {
_13 = _1 as u8;
RET = [_2];
_9.1.fld1.3.fld4 = Move(_9.1.fld2.fld0.fld4);
_11 = core::ptr::addr_of!(_9.1.fld2.fld0.fld6);
_12 = [false,true,false,true,true,false,true];
_9.1.fld2.fld0 = Adt22 { fld0: 151515305297561715314858362038854712677_i128,fld1: _9.1.fld1.3.fld1,fld2: 7742560498943516804_usize,fld3: _9.1.fld0,fld4: Move(_9.1.fld1.3.fld4),fld5: _2,fld6: _9.1.fld1.3.fld6 };
_9.1.fld1.0 = _10 as usize;
_13 = _5;
_9.3 = &_9.1.fld2.fld0.fld1;
_13 = !_5;
_9.1.fld1.3.fld3 = _9.1.fld2.fld0.fld3 & _9.1.fld0;
_5 = _13;
_9.1.fld3 = [_10,_10];
_9.2 = 7976768356787225020_i64 as u32;
_9.1.fld1.3.fld6 = _9.1.fld2.fld0.fld6;
_9.1.fld1.3.fld4 = Move(_9.1.fld2.fld0.fld4);
_9.1.fld2.fld0 = Adt22 { fld0: (-110061567631698127792098244894523046044_i128),fld1: _9.1.fld1.3.fld1,fld2: _9.1.fld1.0,fld3: _9.1.fld0,fld4: Move(_9.1.fld1.3.fld4),fld5: _2,fld6: _9.1.fld1.3.fld6 };
_13 = _5;
_9.1.fld1.3.fld1 = _9.1.fld2.fld0.fld1;
_9.1.fld5 = [_13,_13,_5,_5,_5,_5,_13,_13];
RET = [_2];
_9.1.fld2.fld0.fld6 = _1 as f64;
Call(_9.1.fld2.fld0.fld6 = fn4(_9.1.fld2.fld0.fld5, _9.1.fld1.2, _9.1.fld2.fld0.fld5, _9.1.fld2.fld0.fld5, _9.1.fld2.fld0.fld0, _3, _9.1.fld2.fld0.fld0, _6, _9.1.fld2.fld0.fld0, _2, _9.1.fld1.2, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.1.fld2.fld0.fld6 = _9.1.fld1.2 as f64;
_9.0 = [false,false,true,false,false,false,true];
_9.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_9.1.fld1.3.fld1);
_7 = 11129885731357585640_u64 as isize;
_9.1.fld1.3.fld0 = -_9.1.fld2.fld0.fld0;
_9.1.fld3 = [_10,_7];
RET = [_9.1.fld2.fld0.fld5];
_18 = [(-7495061062397406958_i64),(-5257331601623999251_i64)];
_9.1.fld1.2 = _5 as f32;
_5 = _13 * _13;
_9.3 = &_9.1.fld1.3.fld1;
_9.0 = [true,false,false,false,false,true,false];
_9.1.fld2.fld0.fld0 = _9.1.fld2.fld0.fld5 as i128;
_9.1.fld2.fld0.fld6 = _5 as f64;
_9.0 = [false,true,false,false,true,false,true];
_12 = _9.0;
_9.1.fld1.0 = _9.1.fld2.fld0.fld2 + _9.1.fld2.fld0.fld2;
Goto(bb4)
}
bb4 = {
_16 = _9.1.fld2.fld0.fld5 as f32;
RET = [_2];
_20.2 = _9.1.fld1.2 as i8;
_9.1.fld1.3.fld0 = -_9.1.fld2.fld0.fld0;
_12 = [false,false,true,false,false,true,false];
_19 = _9.2 as i8;
_20.0 = -_9.1.fld2.fld0.fld5;
_20 = (_9.1.fld2.fld0.fld5, _9.1.fld1.0, _1);
_18 = [4661736253615629335_i64,(-8145687928161045435_i64)];
_9.1.fld1.3.fld0 = _19 as i128;
_21 = _10;
_9.1.fld1.3 = Move(_9.1.fld2.fld0);
_17 = _7;
_9.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_9.1.fld1.3.fld1);
_16 = -_9.1.fld1.2;
_9.1.fld5 = [_13,_5,_13,_5,_13,_5,_13,_13];
_20 = (_2, _9.1.fld1.3.fld2, _19);
_10 = 3145_u16 as isize;
_9.1.fld2.fld0.fld0 = _9.1.fld1.3.fld0;
(*_11) = _9.1.fld1.3.fld6 * _9.1.fld1.3.fld6;
RET = [_9.1.fld1.3.fld5];
(*_11) = _9.2 as f64;
match _1 {
0 => bb3,
1 => bb2,
83 => bb6,
_ => bb5
}
}
bb5 = {
_9.1.fld2.fld0.fld6 = _9.1.fld1.2 as f64;
_9.0 = [false,false,true,false,false,false,true];
_9.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_9.1.fld1.3.fld1);
_7 = 11129885731357585640_u64 as isize;
_9.1.fld1.3.fld0 = -_9.1.fld2.fld0.fld0;
_9.1.fld3 = [_10,_7];
RET = [_9.1.fld2.fld0.fld5];
_18 = [(-7495061062397406958_i64),(-5257331601623999251_i64)];
_9.1.fld1.2 = _5 as f32;
_5 = _13 * _13;
_9.3 = &_9.1.fld1.3.fld1;
_9.0 = [true,false,false,false,false,true,false];
_9.1.fld2.fld0.fld0 = _9.1.fld2.fld0.fld5 as i128;
_9.1.fld2.fld0.fld6 = _5 as f64;
_9.0 = [false,true,false,false,true,false,true];
_12 = _9.0;
_9.1.fld1.0 = _9.1.fld2.fld0.fld2 + _9.1.fld2.fld0.fld2;
Goto(bb4)
}
bb6 = {
_28 = !true;
_9.1.fld5 = [_5,_5,_5,_5,_13,_5,_5,_5];
_19 = _1;
_9.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_9.1.fld2.fld0.fld1);
_17 = _28 as isize;
_9.3 = &_9.1.fld1.3.fld1;
_9.1.fld2.fld0.fld1 = _9.1.fld1.3.fld1;
_9.1.fld2.fld0.fld3 = _9.2;
_13 = _5;
_9.1.fld0 = _9.2;
_9.1.fld0 = _9.1.fld2.fld0.fld3;
_12 = [_28,_28,_28,_28,_28,_28,_28];
_19 = _1;
_20.0 = _28 as i32;
RET = [_2];
_13 = !_5;
_9.1.fld1.3.fld2 = _9.1.fld1.0 | _9.1.fld1.0;
_9.1.fld2.fld0 = Move(_9.1.fld1.3);
_13 = _5 ^ _5;
_9.1.fld2.fld0.fld6 = _7 as f64;
_22 = !54342_u16;
_9.1.fld2.fld0.fld6 = _21 as f64;
RET = [_2];
_10 = !_17;
Call(_15 = core::intrinsics::bswap(_22), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9.1.fld3 = [_21,_7];
Goto(bb8)
}
bb8 = {
_20 = (_2, _9.1.fld2.fld0.fld2, _19);
_9.1.fld1.3.fld2 = !_9.1.fld2.fld0.fld2;
_25 = -(-8815488469586478402_i64);
Goto(bb9)
}
bb9 = {
_28 = !false;
_32 = _9.1.fld4;
_27 = (_21, _9.2);
_20.2 = _19;
_31 = _5 - _5;
_24 = &_9.1.fld2.fld0.fld5;
_18 = [_25,_25];
_9.1.fld1.3.fld6 = _9.1.fld2.fld0.fld6 - _9.1.fld2.fld0.fld6;
_9.1.fld1.3 = Move(_9.1.fld2.fld0);
_9.1.fld1.1 = core::ptr::addr_of!(_33);
_20.2 = _1 << _9.1.fld1.3.fld0;
_31 = _5 - _5;
_9.1.fld5 = [_13,_13,_5,_31,_31,_13,_5,_5];
match _19 {
83 => bb11,
_ => bb10
}
}
bb10 = {
_9.1.fld2.fld0.fld6 = _9.1.fld1.2 as f64;
_9.0 = [false,false,true,false,false,false,true];
_9.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_9.1.fld1.3.fld1);
_7 = 11129885731357585640_u64 as isize;
_9.1.fld1.3.fld0 = -_9.1.fld2.fld0.fld0;
_9.1.fld3 = [_10,_7];
RET = [_9.1.fld2.fld0.fld5];
_18 = [(-7495061062397406958_i64),(-5257331601623999251_i64)];
_9.1.fld1.2 = _5 as f32;
_5 = _13 * _13;
_9.3 = &_9.1.fld1.3.fld1;
_9.0 = [true,false,false,false,false,true,false];
_9.1.fld2.fld0.fld0 = _9.1.fld2.fld0.fld5 as i128;
_9.1.fld2.fld0.fld6 = _5 as f64;
_9.0 = [false,true,false,false,true,false,true];
_12 = _9.0;
_9.1.fld1.0 = _9.1.fld2.fld0.fld2 + _9.1.fld2.fld0.fld2;
Goto(bb4)
}
bb11 = {
_9.1.fld1.3.fld5 = _2 | _2;
_30 = _20.1 == _9.1.fld1.3.fld2;
_32 = _9.1.fld4 | _9.1.fld4;
_9.2 = !_9.1.fld0;
_22 = 45050_u16 << _5;
_1 = _20.2;
_9.1.fld1.1 = core::ptr::addr_of!(_20.2);
_9.1.fld1.3.fld3 = _9.1.fld4 as u32;
_27.0 = _7;
_9.1.fld1.3.fld5 = _20.0 * _20.0;
_9.1.fld2.fld0.fld2 = _20.1;
_9.1.fld4 = 12781182908136832891_u64 as i16;
_9.1.fld2.fld0.fld6 = _9.1.fld1.3.fld6;
_31 = _5;
_2 = _9.1.fld1.3.fld5;
_9.2 = _9.1.fld0;
_24 = &_20.0;
_38 = _17 - _7;
_36 = core::ptr::addr_of_mut!(_3);
_26 = core::ptr::addr_of_mut!((*_24));
_9.1.fld1.3.fld2 = (*_36) as usize;
_37 = _9.1.fld1.3.fld6;
_9.1.fld5 = [_5,_5,_5,_31,_5,_13,_13,_13];
_9.1.fld1.3.fld3 = _27.1;
_9.3 = &_9.1.fld1.3.fld1;
Goto(bb12)
}
bb12 = {
_9.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_9.1.fld2.fld0.fld1);
_9.1.fld1.2 = -_16;
_27 = (_7, _9.1.fld1.3.fld3);
_9.1.fld2.fld0.fld1 = _9.1.fld1.3.fld1;
_39 = -(*_11);
Call(_9.1.fld2.fld0 = fn14(Move(_24), Move(_9.3), (*_36), _30, Move(_36), _38), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_32 = _13 as i16;
_23 = -_10;
_37 = _9.1.fld1.2 as f64;
_9.1.fld1.3.fld6 = _37;
_9.1.fld1.3.fld0 = 195046563509989824_u64 as i128;
_43.3.fld2 = _20.1;
_9.3 = &_9.1.fld2.fld0.fld1;
_20.0 = -_9.1.fld2.fld0.fld5;
_25 = 1635601740145897622_i64 - (-5880963504041387688_i64);
_29 = &_35;
_11 = core::ptr::addr_of!(_37);
_27 = (_7, _9.1.fld0);
_9.1.fld5 = [_13,_13,_13,_5,_13,_13,_5,_5];
_43.3.fld3 = _9.1.fld2.fld0.fld2 as u32;
_25 = (-8286762679614080323_i64) - 1241611385508134218_i64;
Call(_9.1.fld1.3.fld3 = core::intrinsics::bswap(_27.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_32 = -_9.1.fld4;
_9.1.fld1.3.fld6 = -_39;
_9.1.fld1.3.fld1 = _9.1.fld2.fld0.fld1;
_9.1.fld2.fld0.fld0 = _9.1.fld1.2 as i128;
_43.3.fld1 = _9.1.fld2.fld0.fld1;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(3_usize, 13_usize, Move(_13), 5_usize, Move(_5), 6_usize, Move(_6), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(3_usize, 19_usize, Move(_19), 7_usize, Move(_7), 15_usize, Move(_15), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(3_usize, 10_usize, Move(_10), 30_usize, Move(_30), 17_usize, Move(_17), 38_usize, Move(_38)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i32,mut _2: f32,mut _3: i32,mut _4: i32,mut _5: i128,mut _6: u128,mut _7: i128,mut _8: [isize; 2],mut _9: i128,mut _10: i32,mut _11: f32,mut _12: u128) -> f64 {
mir! {
type RET = f64;
let _13: *mut i8;
let _14: Adt30;
let _15: u8;
let _16: u16;
let _17: *mut usize;
let _18: Adt58;
let _19: [bool; 1];
let _20: &'static *mut (usize, *const i8, f32, Adt22);
let _21: &'static Adt58;
let _22: u128;
let _23: f64;
let _24: u128;
let _25: [u8; 8];
let _26: u8;
let _27: [i32; 1];
let _28: [bool; 7];
let _29: [u8; 8];
let _30: char;
let _31: [char; 3];
let _32: isize;
let _33: isize;
let _34: Adt30;
let _35: isize;
let _36: f32;
let _37: [char; 3];
let _38: ();
let _39: ();
{
_5 = _9 * _9;
_4 = _5 as i32;
_5 = -_9;
_9 = !_7;
RET = _12 as f64;
_9 = _12 as i128;
_12 = (-5427793064286774914_i64) as u128;
_9 = 107874978_u32 as i128;
_8 = [(-86_isize),9223372036854775807_isize];
_11 = -_2;
RET = 9223372036854775807_isize as f64;
Call(_10 = fn5(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _5;
_12 = 16876308916778331204_u64 as u128;
_4 = _1;
_14.fld3 = [(-778367777501663252_i64),2787303316026034543_i64];
_2 = _11;
match _7 {
230220799289240335671276362537245165412 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_15 = 138_u8;
RET = 12852048053901341438_u64 as f64;
_14.fld1 = RET as u8;
_3 = _12 as i32;
_3 = !_10;
_1 = _10 * _4;
_15 = (-1526616269506554834_i64) as u8;
_4 = '\u{29723}' as i32;
Goto(bb4)
}
bb4 = {
_4 = !_10;
RET = 2941230614791875280_i64 as f64;
RET = 74_isize as f64;
_8 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = 5675817848609590392_i64 as i128;
match _7 {
0 => bb3,
1 => bb2,
230220799289240335671276362537245165412 => bb6,
_ => bb5
}
}
bb5 = {
_9 = _5;
_12 = 16876308916778331204_u64 as u128;
_4 = _1;
_14.fld3 = [(-778367777501663252_i64),2787303316026034543_i64];
_2 = _11;
match _7 {
230220799289240335671276362537245165412 => bb3,
_ => bb2
}
}
bb6 = {
_15 = (-6442298225141494214_i64) as u8;
Call(_12 = fn6(_5, _7, _4, _1, _4, _1, _7, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = 7924864602884529982_i64 as f64;
_1 = _4 + _3;
_1 = !_4;
Call(_5 = fn9(_1, _1, _7, _7, _3, _4, _11, _1, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_19 = [false];
_21 = &_18;
_21 = &_18;
_19 = [false];
_14.fld2 = core::ptr::addr_of_mut!(_6);
_15 = _2 as u8;
_19 = [false];
_14.fld1 = !_15;
Goto(bb9)
}
bb9 = {
_2 = -_11;
_12 = RET as u128;
_8 = [17_isize,79_isize];
_2 = _11 + _11;
_8 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_12 = !_6;
_1 = 11834_i16 as i32;
_7 = _5;
_3 = _4 << _5;
_7 = RET as i128;
_22 = _12;
_4 = (-9223372036854775808_isize) as i32;
_9 = _5;
_6 = _2 as u128;
_7 = _9 | _5;
_24 = !_12;
RET = (-8515601799481448654_i64) as f64;
_19 = [true];
_15 = !_14.fld1;
Goto(bb10)
}
bb10 = {
_5 = 9223372036854775807_isize as i128;
_26 = _14.fld1 ^ _14.fld1;
_24 = _22;
_21 = &(*_21);
RET = (-15678_i16) as f64;
_27 = [_3];
_12 = !_24;
_27 = [_10];
_1 = !_3;
_29 = [_15,_14.fld1,_14.fld1,_26,_14.fld1,_26,_15,_26];
_23 = -RET;
_16 = RET as u16;
_2 = _11;
RET = _23;
_1 = _3;
_14.fld3 = [6091667285008311762_i64,(-845441090419846454_i64)];
RET = _23 * _23;
_21 = &(*_21);
Call(_9 = fn11(_7, _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_25 = _29;
_8 = [9223372036854775807_isize,123_isize];
_9 = !_7;
_23 = -RET;
_26 = _11 as u8;
_28 = [false,true,true,true,true,true,false];
_3 = _10;
_32 = 78_isize;
_22 = !_24;
_24 = _6;
RET = _2 as f64;
_14.fld1 = _15;
_1 = !_3;
_7 = !_9;
_19 = [false];
_27 = [_10];
_10 = -_1;
RET = _11 as f64;
_25 = _29;
Call(_15 = core::intrinsics::transmute(_14.fld1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _23 - _23;
_33 = _12 as isize;
_16 = !4051_u16;
_21 = &(*_21);
_1 = _10 * _3;
RET = _23;
_29 = _25;
_16 = 4896_u16 & 4303_u16;
_2 = (-379660045287688334_i64) as f32;
_9 = -_7;
match _32 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb6,
78 => bb14,
_ => bb13
}
}
bb13 = {
_4 = !_10;
RET = 2941230614791875280_i64 as f64;
RET = 74_isize as f64;
_8 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_9 = 5675817848609590392_i64 as i128;
match _7 {
0 => bb3,
1 => bb2,
230220799289240335671276362537245165412 => bb6,
_ => bb5
}
}
bb14 = {
_7 = -_9;
_11 = _2;
_22 = !_12;
_3 = -_1;
_24 = _11 as u128;
_30 = '\u{b0f61}';
_14.fld2 = core::ptr::addr_of_mut!(_6);
_14.fld1 = _15 << _9;
_34.fld3 = [(-8652391214997075085_i64),1380474921149818744_i64];
_34.fld3 = [(-1593471451838189720_i64),2407719059555504424_i64];
_5 = _9 * _7;
RET = _23;
_6 = _14.fld1 as u128;
_8 = [_32,_33];
_35 = _32;
_3 = _2 as i32;
_8 = [_32,_32];
_25 = [_14.fld1,_14.fld1,_14.fld1,_14.fld1,_14.fld1,_14.fld1,_14.fld1,_14.fld1];
_32 = _35;
RET = -_23;
_8 = [_35,_32];
_31 = [_30,_30,_30];
_6 = !_12;
RET = _5 as f64;
_29 = _25;
_5 = !_9;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(4_usize, 27_usize, Move(_27), 12_usize, Move(_12), 31_usize, Move(_31), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(4_usize, 6_usize, Move(_6), 19_usize, Move(_19), 35_usize, Move(_35), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(4_usize, 7_usize, Move(_7), 32_usize, Move(_32), 10_usize, Move(_10), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5() -> i32 {
mir! {
type RET = i32;
let _1: char;
let _2: *const *mut (usize, *const i8, f32, Adt22);
let _3: (*mut i32, i8, *mut u128, usize);
let _4: f64;
let _5: i32;
let _6: (*const usize,);
let _7: [i32; 1];
let _8: f64;
let _9: Adt22;
let _10: [u32; 2];
let _11: f32;
let _12: Adt58;
let _13: bool;
let _14: f32;
let _15: (i32, usize, i8);
let _16: isize;
let _17: ((usize, *const i8, f32, Adt22),);
let _18: ((*mut i32, i8, *mut u128, usize),);
let _19: isize;
let _20: f32;
let _21: i32;
let _22: (u128,);
let _23: char;
let _24: isize;
let _25: i32;
let _26: (i8, (i32, usize, i8), (u128,), u128);
let _27: bool;
let _28: f64;
let _29: bool;
let _30: char;
let _31: (isize, u32);
let _32: [char; 3];
let _33: (usize, *const i8, f32, Adt22);
let _34: u64;
let _35: (*mut char, Adt38, (i32, usize, i8));
let _36: ();
let _37: ();
{
RET = 272844318_i32 + (-1005798991_i32);
RET = !(-2000091129_i32);
RET = -1040284238_i32;
RET = 1286975262_i32 * 1204428031_i32;
RET = (-1784373975_i32) ^ 1448162911_i32;
_1 = '\u{77d02}';
_1 = '\u{26174}';
RET = (-34_i8) as i32;
RET = 114522733655704589633583570242920526618_i128 as i32;
_1 = '\u{6f8d}';
_1 = '\u{2a8d7}';
RET = (-717868072_i32);
_1 = '\u{73ad3}';
RET = 1719822495_i32;
RET = (-164348260_i32) >> (-19977_i16);
_3.3 = !15702704628001051330_usize;
_1 = '\u{da905}';
RET = 245419415103357670769278855711508608123_u128 as i32;
RET = !2113307409_i32;
_3.3 = RET as usize;
RET = (-7390617_i32) ^ (-1628560121_i32);
Goto(bb1)
}
bb1 = {
_3.0 = core::ptr::addr_of_mut!(_5);
_3.3 = !7_usize;
_1 = '\u{f8417}';
_3.1 = 6794_u16 as i8;
_3.3 = 1_usize >> _3.1;
_4 = _3.1 as f64;
_3.0 = core::ptr::addr_of_mut!(_5);
RET = (-1861232474_i32);
RET = -(-527445188_i32);
_5 = !RET;
_3.0 = core::ptr::addr_of_mut!(RET);
_7 = [RET];
_3.1 = (-115_i8) | (-1_i8);
_3.3 = 0_usize;
_3.1 = (-102_i8);
_8 = 3838946260_u32 as f64;
RET = true as i32;
_3.3 = 2_usize;
_8 = -_4;
RET = _5;
_8 = -_4;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = -_5;
_9.fld1 = _1;
_9.fld2 = _3.3;
_3.0 = core::ptr::addr_of_mut!(_9.fld5);
_3.1 = (-43_i8);
_9.fld2 = _3.3 / _3.3;
_7 = [_5];
_9.fld1 = _1;
_9.fld3 = 364313336_u32;
_7 = [RET];
_9.fld5 = -_5;
_9.fld5 = RET;
_9.fld5 = RET << _9.fld2;
_3.1 = -2_i8;
_9.fld6 = _8;
_4 = (-7456070925858724497_i64) as f64;
_10 = [_9.fld3,_9.fld3];
_4 = _9.fld6 * _9.fld6;
RET = true as i32;
Goto(bb3)
}
bb3 = {
_10 = [_9.fld3,_9.fld3];
_7 = [_9.fld5];
_7 = [RET];
_9.fld0 = 22702007135672259706187745425953943821_i128;
_9.fld5 = _5;
_3.3 = _9.fld1 as usize;
_3.0 = core::ptr::addr_of_mut!(RET);
_9.fld5 = _5 << _3.1;
RET = _9.fld5 & _5;
_9.fld5 = -_5;
_6.0 = core::ptr::addr_of!(_3.3);
_4 = _8;
_4 = _8 - _9.fld6;
_6.0 = core::ptr::addr_of!(_3.3);
_1 = _9.fld1;
_9.fld4 = core::ptr::addr_of_mut!(_9.fld1);
_7 = [_9.fld5];
_9.fld3 = 1733799877_u32 + 657635198_u32;
RET = _9.fld5;
_4 = _8;
_3.1 = !(-91_i8);
_3.0 = core::ptr::addr_of_mut!(_9.fld5);
_3.3 = _9.fld2 & _9.fld2;
_9.fld3 = 2668134082_u32 ^ 2351577791_u32;
_13 = _3.3 == _3.3;
match _9.fld0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
22702007135672259706187745425953943821 => bb9,
_ => bb8
}
}
bb4 = {
RET = -_5;
_9.fld1 = _1;
_9.fld2 = _3.3;
_3.0 = core::ptr::addr_of_mut!(_9.fld5);
_3.1 = (-43_i8);
_9.fld2 = _3.3 / _3.3;
_7 = [_5];
_9.fld1 = _1;
_9.fld3 = 364313336_u32;
_7 = [RET];
_9.fld5 = -_5;
_9.fld5 = RET;
_9.fld5 = RET << _9.fld2;
_3.1 = -2_i8;
_9.fld6 = _8;
_4 = (-7456070925858724497_i64) as f64;
_10 = [_9.fld3,_9.fld3];
_4 = _9.fld6 * _9.fld6;
RET = true as i32;
Goto(bb3)
}
bb5 = {
_3.0 = core::ptr::addr_of_mut!(_5);
_3.3 = !7_usize;
_1 = '\u{f8417}';
_3.1 = 6794_u16 as i8;
_3.3 = 1_usize >> _3.1;
_4 = _3.1 as f64;
_3.0 = core::ptr::addr_of_mut!(_5);
RET = (-1861232474_i32);
RET = -(-527445188_i32);
_5 = !RET;
_3.0 = core::ptr::addr_of_mut!(RET);
_7 = [RET];
_3.1 = (-115_i8) | (-1_i8);
_3.3 = 0_usize;
_3.1 = (-102_i8);
_8 = 3838946260_u32 as f64;
RET = true as i32;
_3.3 = 2_usize;
_8 = -_4;
RET = _5;
_8 = -_4;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
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
_9.fld6 = _9.fld0 as f64;
_9.fld6 = _8;
_11 = _9.fld6 as f32;
_15 = (_9.fld5, _9.fld2, _3.1);
_4 = -_9.fld6;
RET = _9.fld5 >> _3.3;
_9.fld5 = -RET;
_9.fld5 = RET;
_9.fld5 = 17530_u16 as i32;
_3.0 = core::ptr::addr_of_mut!(_5);
_15.2 = _3.1 * _3.1;
_6.0 = core::ptr::addr_of!(_15.1);
_9.fld2 = !_3.3;
match _9.fld0 {
0 => bb8,
1 => bb7,
2 => bb3,
22702007135672259706187745425953943821 => bb11,
_ => bb10
}
}
bb10 = {
RET = -_5;
_9.fld1 = _1;
_9.fld2 = _3.3;
_3.0 = core::ptr::addr_of_mut!(_9.fld5);
_3.1 = (-43_i8);
_9.fld2 = _3.3 / _3.3;
_7 = [_5];
_9.fld1 = _1;
_9.fld3 = 364313336_u32;
_7 = [RET];
_9.fld5 = -_5;
_9.fld5 = RET;
_9.fld5 = RET << _9.fld2;
_3.1 = -2_i8;
_9.fld6 = _8;
_4 = (-7456070925858724497_i64) as f64;
_10 = [_9.fld3,_9.fld3];
_4 = _9.fld6 * _9.fld6;
RET = true as i32;
Goto(bb3)
}
bb11 = {
_16 = (-9223372036854775808_isize);
_11 = RET as f32;
_17.0.3.fld3 = _16 as u32;
_9.fld5 = RET & RET;
_7 = [_9.fld5];
_17.0.3.fld0 = -_9.fld0;
_1 = _9.fld1;
_9.fld0 = _17.0.3.fld0;
_18.0.1 = _15.2 - _15.2;
_9.fld1 = _1;
_17.0.3.fld1 = _1;
_17.0.3.fld2 = (-4138_i16) as usize;
_3.1 = !_15.2;
_17.0.1 = core::ptr::addr_of!(_15.2);
Goto(bb12)
}
bb12 = {
_17.0.1 = core::ptr::addr_of!(_18.0.1);
_10 = [_9.fld3,_17.0.3.fld3];
_18.0.1 = _1 as i8;
_22 = (227301783990344987162585350315093899671_u128,);
_23 = _17.0.3.fld1;
_9.fld1 = _1;
_15.1 = !_17.0.3.fld2;
_17.0.3.fld1 = _9.fld1;
_17.0.2 = _11;
_17.0.3 = Move(_9);
_18.0.0 = core::ptr::addr_of_mut!(_17.0.3.fld5);
_17.0.2 = 42984_u16 as f32;
_3.2 = core::ptr::addr_of_mut!(_22.0);
_7 = [_17.0.3.fld5];
_17.0.3.fld4 = core::ptr::addr_of_mut!(_23);
_17.0.0 = _3.3;
_9.fld6 = _4;
_18.0 = (Move(_3.0), _3.1, Move(_3.2), _17.0.3.fld2);
RET = (-7575613287048333174_i64) as i32;
_9.fld2 = _16 as usize;
Goto(bb13)
}
bb13 = {
_3.2 = core::ptr::addr_of_mut!(_26.3);
_17.0.3.fld4 = core::ptr::addr_of_mut!(_17.0.3.fld1);
_24 = _16;
_17.0.1 = core::ptr::addr_of!(_3.1);
_9.fld0 = !_17.0.3.fld0;
_28 = 131_u8 as f64;
_9.fld4 = Move(_17.0.3.fld4);
_14 = -_11;
_28 = _17.0.3.fld6 - _9.fld6;
_26.1.1 = _17.0.3.fld5 as usize;
_9.fld1 = _17.0.3.fld1;
_26.1.2 = _15.2;
_26 = (_15.2, _15, _22, _22.0);
_25 = !_17.0.3.fld5;
_3.3 = _26.3 as usize;
_28 = _9.fld6;
_17.0.3.fld4 = Move(_9.fld4);
_26.1.2 = -_18.0.1;
_15.2 = _9.fld6 as i8;
Call(RET = core::intrinsics::transmute(_7), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_26.1.2 = _18.0.1;
_17.0.3.fld0 = _9.fld0 + _9.fld0;
_3.0 = core::ptr::addr_of_mut!(_15.0);
_17.0.3.fld4 = core::ptr::addr_of_mut!(_9.fld1);
_31.1 = !_17.0.3.fld3;
_15 = (RET, _17.0.3.fld2, _3.1);
_33.3.fld1 = _9.fld1;
_15.1 = !_17.0.3.fld2;
_18.0.0 = Move(_3.0);
_15.1 = _18.0.3 + _17.0.0;
_26.0 = _17.0.3.fld0 as i8;
_17.0.3.fld4 = core::ptr::addr_of_mut!(_30);
_9.fld2 = 25259_i16 as usize;
_17.0.3.fld1 = _23;
_33.3.fld1 = _17.0.3.fld1;
_9.fld2 = _17.0.0 << _15.0;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(5_usize, 25_usize, Move(_25), 13_usize, Move(_13), 5_usize, Move(_5), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(5_usize, 26_usize, Move(_26), 10_usize, Move(_10), 37_usize, _37, 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i128,mut _2: i128,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: i128,mut _8: i32) -> u128 {
mir! {
type RET = u128;
let _9: char;
let _10: [char; 3];
let _11: i8;
let _12: [i64; 2];
let _13: char;
let _14: bool;
let _15: [char; 3];
let _16: [char; 3];
let _17: *const usize;
let _18: isize;
let _19: char;
let _20: f32;
let _21: i8;
let _22: ();
let _23: ();
{
_6 = _3 | _8;
RET = 7903135742870783246_usize as u128;
RET = 328874064705588260264356616235180931227_u128 - 8014995307378813835249560163388784998_u128;
_5 = -_8;
_9 = '\u{f8ab6}';
_9 = '\u{56070}';
RET = 235459419544660400064765967576074508555_u128;
_8 = _3;
_2 = _7 | _7;
RET = 326240463373540386580361790830958598018_u128;
_8 = _5 >> _4;
_3 = !_4;
_8 = _4 ^ _4;
_1 = (-67_isize) as i128;
_3 = 118_i8 as i32;
RET = 60321520576929772115249535595543595512_u128 | 336679533370505849708243012243392311100_u128;
_10 = [_9,_9,_9];
_5 = -_8;
_5 = _8;
_1 = _2 << _4;
_3 = _6 & _8;
_4 = _5;
_8 = _4 - _5;
Goto(bb1)
}
bb1 = {
_10 = [_9,_9,_9];
_7 = _1;
_13 = _9;
_3 = 31872_u16 as i32;
_8 = _6;
_13 = _9;
_8 = !_5;
_9 = _13;
_5 = _4;
_12 = [(-8385358466295511794_i64),1658029358988823763_i64];
_4 = _6;
_14 = true;
RET = 130756126346012114163816915858697404736_u128 - 194548926928686706253391316165348923077_u128;
_1 = !_7;
_16 = _10;
Call(RET = core::intrinsics::bswap(14926610702728947501676121682090265196_u128), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = [_13,_13,_13];
Call(_6 = fn7(_2, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _2;
_7 = (-179_i16) as i128;
_5 = _8 - _8;
_1 = _2;
RET = 53328248101270271226159065967018904508_u128 + 24055440402342155621151580041747266707_u128;
_6 = _5 + _8;
_11 = (-59_i8) | 125_i8;
_12 = [4395668708431202030_i64,3719046633667827099_i64];
_20 = 103_u8 as f32;
_19 = _9;
RET = _1 as u128;
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(6_usize, 16_usize, Move(_16), 6_usize, Move(_6), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_22 = dump_var(6_usize, 8_usize, Move(_8), 9_usize, Move(_9), 19_usize, Move(_19), 10_usize, Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i128,mut _2: i128) -> i32 {
mir! {
type RET = i32;
let _3: [u64; 2];
let _4: ((*mut i32, i8, *mut u128, usize),);
let _5: char;
let _6: (*mut i32, i8, *mut u128, usize);
let _7: f64;
let _8: [bool; 7];
let _9: isize;
let _10: [u32; 2];
let _11: [bool; 4];
let _12: char;
let _13: isize;
let _14: (u8, isize, Adt38);
let _15: *const usize;
let _16: f64;
let _17: [isize; 2];
let _18: ();
let _19: ();
{
RET = 81623614769546172863726525161618642282_u128 as i32;
_1 = _2;
_2 = _1 * _1;
RET = !(-558233436_i32);
_3 = [2904066270731827112_u64,7177709937664255929_u64];
RET = (-1322555730_i32) + 422974574_i32;
_3 = [507698792751802892_u64,17318737867030254914_u64];
Call(_1 = fn8(_2, _2, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = -1911799500_i32;
_4.0.0 = core::ptr::addr_of_mut!(RET);
_4.0.0 = core::ptr::addr_of_mut!(RET);
_6.3 = (-9223372036854775808_isize) as usize;
_5 = '\u{30bbc}';
Goto(bb2)
}
bb2 = {
_5 = '\u{f79f1}';
_7 = 3955001933_u32 as f64;
_4.0.1 = 85_i8 >> _1;
_2 = _1 ^ _1;
_5 = '\u{eff9b}';
_9 = 9223372036854775807_isize;
match _9 {
0 => bb3,
1 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb3 = {
RET = -1911799500_i32;
_4.0.0 = core::ptr::addr_of_mut!(RET);
_4.0.0 = core::ptr::addr_of_mut!(RET);
_6.3 = (-9223372036854775808_isize) as usize;
_5 = '\u{30bbc}';
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
RET = 1017649026_i32;
_8 = [true,false,false,true,false,false,true];
Call(RET = core::intrinsics::bswap((-1053473310_i32)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = [true,false,true,false];
_8 = [true,false,false,true,true,false,true];
_6.1 = _4.0.1;
_10 = [4175534805_u32,1595024570_u32];
RET = 2922820091989540716_u64 as i32;
_4.0.1 = -_6.1;
_9 = !9223372036854775807_isize;
RET = !(-1068901477_i32);
_8 = [false,true,true,true,true,false,true];
_12 = _5;
_3 = [14563640566428190246_u64,6521404641024490408_u64];
_4.0.3 = 12972932354231818336_u64 as usize;
_6.0 = Move(_4.0.0);
_7 = 269078341083788814_u64 as f64;
_4.0.0 = core::ptr::addr_of_mut!(RET);
Goto(bb8)
}
bb8 = {
_10 = [2204829825_u32,1813678978_u32];
_13 = _9 * _9;
_12 = _5;
_11 = [true,false,true,true];
_4.0.1 = !_6.1;
_8 = [false,true,false,true,false,true,true];
_4.0.3 = _6.3;
_8 = [false,false,true,true,false,false,false];
_4.0.3 = !_6.3;
_3 = [126107768676799161_u64,11282671800999833178_u64];
_14.0 = 188_u8 - 223_u8;
_14.2.fld0 = 1215345211_u32 as f32;
_14.1 = !_9;
_14.0 = 46_u8 * 248_u8;
RET = _4.0.1 as i32;
_16 = -_7;
_9 = _14.1 * _13;
RET = _2 as i32;
_14.2.fld1 = core::ptr::addr_of!(_4.0.1);
_3 = [2251732947441209917_u64,15378295457764024280_u64];
_14.0 = _14.1 as u8;
_6.3 = _2 as usize;
Goto(bb9)
}
bb9 = {
Call(_18 = dump_var(7_usize, 10_usize, Move(_10), 13_usize, Move(_13), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_18 = dump_var(7_usize, 5_usize, Move(_5), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i128,mut _2: i128,mut _3: i128,mut _4: i128) -> i128 {
mir! {
type RET = i128;
let _5: ();
let _6: ();
{
_3 = (-3472117564753271219_i64) as i128;
_2 = !_1;
RET = !_1;
RET = _4;
Goto(bb1)
}
bb1 = {
Call(_5 = dump_var(8_usize, 2_usize, Move(_2), 1_usize, Move(_1), 6_usize, _6, 6_usize, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i32,mut _2: i32,mut _3: i128,mut _4: i128,mut _5: i32,mut _6: i32,mut _7: f32,mut _8: i32,mut _9: i128) -> i128 {
mir! {
type RET = i128;
let _10: (*const usize,);
let _11: u8;
let _12: Adt22;
let _13: [char; 3];
let _14: *mut &'static i32;
let _15: *mut i32;
let _16: isize;
let _17: (i8, (i32, usize, i8), (u128,), u128);
let _18: &'static bool;
let _19: &'static [char; 3];
let _20: bool;
let _21: f32;
let _22: (u8, isize, Adt38);
let _23: i8;
let _24: isize;
let _25: char;
let _26: &'static *mut (usize, *const i8, f32, Adt22);
let _27: char;
let _28: ();
let _29: ();
{
_7 = (-9223372036854775808_isize) as f32;
RET = 17585_u16 as i128;
_11 = !208_u8;
RET = _3 | _9;
_6 = _8 - _8;
_8 = _2 & _2;
_12.fld2 = 2_usize ^ 48787742051680505_usize;
_10.0 = core::ptr::addr_of!(_12.fld2);
_4 = !_9;
_5 = _8 | _6;
_12.fld4 = core::ptr::addr_of_mut!(_12.fld1);
_12.fld2 = 94577997922605116013474239337919809953_u128 as usize;
_12.fld4 = core::ptr::addr_of_mut!(_12.fld1);
_12.fld5 = (-8910733169913810170_i64) as i32;
_9 = _4 >> _1;
_4 = !RET;
_12.fld0 = _11 as i128;
RET = _3 | _9;
_9 = _4 + RET;
_12.fld5 = _6 >> RET;
_12.fld1 = '\u{78548}';
_2 = -_12.fld5;
_10.0 = core::ptr::addr_of!(_12.fld2);
RET = _3;
Goto(bb1)
}
bb1 = {
_1 = _5 & _12.fld5;
_13 = [_12.fld1,_12.fld1,_12.fld1];
_12.fld1 = '\u{cef9d}';
_1 = 78_i8 as i32;
_8 = 13328_i16 as i32;
Call(_8 = fn10(_9, _3, _2, RET, _5, _6, _12.fld5, _4, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.2 = (157238185470061004843349480063380238998_u128,);
_17.3 = !_17.2.0;
_6 = RET as i32;
_4 = _9;
Goto(bb3)
}
bb3 = {
_6 = _2 * _8;
_7 = (-21256_i16) as f32;
_12.fld0 = !_9;
_19 = &_13;
_9 = -_4;
_19 = &(*_19);
_17.1.1 = !_12.fld2;
_16 = !9223372036854775807_isize;
_17.3 = !_17.2.0;
_12.fld6 = 3783250945_u32 as f64;
match _17.2.0 {
157238185470061004843349480063380238998 => bb4,
_ => bb1
}
}
bb4 = {
_20 = true;
_17.1 = (_5, _12.fld2, 53_i8);
_7 = 559_u16 as f32;
_22.2.fld0 = -_7;
_3 = -_12.fld0;
_10.0 = core::ptr::addr_of!(_12.fld2);
_22.2.fld1 = core::ptr::addr_of!(_23);
_23 = _17.1.2 << _5;
_5 = _22.2.fld0 as i32;
RET = !_4;
_17.1.1 = _12.fld2;
_9 = _12.fld0;
_2 = !_12.fld5;
_17.1 = (_8, _12.fld2, _23);
_11 = !90_u8;
_12.fld6 = _7 as f64;
_1 = _16 as i32;
_22.1 = _16 - _16;
_12.fld2 = _17.1.1 << _9;
_17.2.0 = _17.3 + _17.3;
_10.0 = core::ptr::addr_of!(_17.1.1);
_15 = core::ptr::addr_of_mut!(_2);
_3 = !RET;
_27 = _12.fld1;
_25 = _12.fld1;
Goto(bb5)
}
bb5 = {
Call(_28 = dump_var(9_usize, 1_usize, Move(_1), 13_usize, Move(_13), 27_usize, Move(_27), 6_usize, Move(_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_28 = dump_var(9_usize, 2_usize, Move(_2), 9_usize, Move(_9), 20_usize, Move(_20), 29_usize, _29), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i128,mut _2: i128,mut _3: i32,mut _4: i128,mut _5: i32,mut _6: i32,mut _7: i32,mut _8: i128,mut _9: i32) -> i32 {
mir! {
type RET = i32;
let _10: (usize, *mut char);
let _11: [bool; 7];
let _12: &'static bool;
let _13: ();
let _14: ();
{
_9 = _3;
_7 = _6 >> _9;
RET = _5 | _9;
RET = _3 ^ _7;
_4 = _1 - _1;
_4 = !_2;
_1 = _8 ^ _8;
_10.0 = 95_u8 as usize;
_9 = _7;
_4 = !_8;
_10.0 = 3_i16 as usize;
_9 = _7;
_2 = (-30498_i16) as i128;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(10_usize, 7_usize, Move(_7), 5_usize, Move(_5), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i128,mut _2: i32) -> i128 {
mir! {
type RET = i128;
let _3: *mut usize;
let _4: *const i8;
let _5: isize;
let _6: [u64; 2];
let _7: isize;
let _8: isize;
let _9: char;
let _10: &'static (*mut &'static i32, (usize, *mut char));
let _11: f32;
let _12: (i16, (i32, usize, i8), *mut *mut char);
let _13: [bool; 1];
let _14: isize;
let _15: (u16,);
let _16: (i32, usize, i8);
let _17: Adt34;
let _18: bool;
let _19: f64;
let _20: [bool; 4];
let _21: (Adt38, usize, char, u8);
let _22: &'static [i32; 1];
let _23: bool;
let _24: f32;
let _25: isize;
let _26: i128;
let _27: ();
let _28: ();
{
RET = _1;
_2 = 6156405595577538939_i64 as i32;
RET = !_1;
RET = _1;
RET = _1;
_1 = RET;
_2 = (-49_i8) as i32;
RET = 117_i8 as i128;
_1 = RET * RET;
_1 = RET;
RET = _1;
_6 = [11790501446468956028_u64,4390975535068706000_u64];
_2 = (-1173270463_i32) >> RET;
_6 = [16967169627072493719_u64,1312983083118927982_u64];
_5 = RET as isize;
Call(_7 = fn12(_2, _1, _6, _6, _6, _1, _6, _6, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = !_5;
RET = _1;
RET = !_1;
RET = _1;
_8 = _5;
_7 = 3226698686993828823_usize as isize;
_2 = 5584486352726694613_i64 as i32;
RET = 7504366238936666224_i64 as i128;
RET = _1;
_5 = !_7;
_7 = -_8;
_5 = _7 - _8;
_2 = (-493314042_i32);
_6 = [10110309252278890495_u64,16225310455660268273_u64];
Goto(bb2)
}
bb2 = {
RET = !_1;
_6 = [7954210446642930567_u64,8412271474169915680_u64];
_12.0 = 16950_i16;
_12.1.0 = _2 >> _5;
RET = _1;
_12.1.1 = 186_u8 as usize;
_13 = [true];
_13 = [false];
_6 = [17465630341486867023_u64,13432706614622597904_u64];
RET = -_1;
_15 = (42051_u16,);
_13 = [true];
_12.0 = 879_i16 >> _12.1.0;
_4 = core::ptr::addr_of!(_12.1.2);
_16.0 = _2;
(*_4) = (-21_i8) + (-63_i8);
_11 = _15.0 as f32;
_12.1.0 = _16.0 | _2;
RET = _1;
_9 = '\u{a8d8b}';
_17.fld0.2.0 = 225916382550873397583740466122070396380_u128 & 37487612186626751820096614735756670261_u128;
_17.fld0.1 = _12.1;
_17.fld3.0 = !_12.0;
_16.2 = _12.1.2 << (*_4);
match _16.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431274897414 => bb10,
_ => bb9
}
}
bb3 = {
_7 = !_5;
RET = _1;
RET = !_1;
RET = _1;
_8 = _5;
_7 = 3226698686993828823_usize as isize;
_2 = 5584486352726694613_i64 as i32;
RET = 7504366238936666224_i64 as i128;
RET = _1;
_5 = !_7;
_7 = -_8;
_5 = _7 - _8;
_2 = (-493314042_i32);
_6 = [10110309252278890495_u64,16225310455660268273_u64];
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
_17.fld2.fld5 = _16.0 << _17.fld0.1.2;
_16.1 = _12.1.1 & _12.1.1;
_16.2 = _12.1.2;
_17.fld3.0 = 3956004191507638798_u64 as i16;
_17.fld4.fld1 = _9 as u8;
_6 = [1320996492736996838_u64,17095734369152952820_u64];
_17.fld2.fld3 = 4036991840_u32;
_6 = [15766767107023777375_u64,7527962430677415326_u64];
_17.fld0.1 = (_12.1.0, _16.1, (*_4));
_17.fld2.fld4 = core::ptr::addr_of_mut!(_17.fld1);
_17.fld0.1 = (_12.1.0, _16.1, (*_4));
_17.fld0.3 = _17.fld0.2.0 >> (*_4);
_17.fld5 = core::ptr::addr_of!((*_4));
_17.fld0.3 = _17.fld0.2.0;
_17.fld1 = _9;
_21.0.fld0 = _11;
_18 = _17.fld2.fld5 == _12.1.0;
_14 = _8;
_3 = core::ptr::addr_of_mut!(_17.fld0.1.1);
_3 = core::ptr::addr_of_mut!(_17.fld2.fld2);
(*_3) = _17.fld0.1.1;
_5 = _7;
_13 = [_18];
_21.3 = _17.fld2.fld3 as u8;
Goto(bb11)
}
bb11 = {
_17.fld2.fld6 = _12.1.2 as f64;
_17.fld1 = _9;
match _15.0 {
0 => bb12,
1 => bb13,
42051 => bb15,
_ => bb14
}
}
bb12 = {
_17.fld2.fld5 = _16.0 << _17.fld0.1.2;
_16.1 = _12.1.1 & _12.1.1;
_16.2 = _12.1.2;
_17.fld3.0 = 3956004191507638798_u64 as i16;
_17.fld4.fld1 = _9 as u8;
_6 = [1320996492736996838_u64,17095734369152952820_u64];
_17.fld2.fld3 = 4036991840_u32;
_6 = [15766767107023777375_u64,7527962430677415326_u64];
_17.fld0.1 = (_12.1.0, _16.1, (*_4));
_17.fld2.fld4 = core::ptr::addr_of_mut!(_17.fld1);
_17.fld0.1 = (_12.1.0, _16.1, (*_4));
_17.fld0.3 = _17.fld0.2.0 >> (*_4);
_17.fld5 = core::ptr::addr_of!((*_4));
_17.fld0.3 = _17.fld0.2.0;
_17.fld1 = _9;
_21.0.fld0 = _11;
_18 = _17.fld2.fld5 == _12.1.0;
_14 = _8;
_3 = core::ptr::addr_of_mut!(_17.fld0.1.1);
_3 = core::ptr::addr_of_mut!(_17.fld2.fld2);
(*_3) = _17.fld0.1.1;
_5 = _7;
_13 = [_18];
_21.3 = _17.fld2.fld3 as u8;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_7 = !_5;
RET = _1;
RET = !_1;
RET = _1;
_8 = _5;
_7 = 3226698686993828823_usize as isize;
_2 = 5584486352726694613_i64 as i32;
RET = 7504366238936666224_i64 as i128;
RET = _1;
_5 = !_7;
_7 = -_8;
_5 = _7 - _8;
_2 = (-493314042_i32);
_6 = [10110309252278890495_u64,16225310455660268273_u64];
Goto(bb2)
}
bb15 = {
_5 = (-1228284931221900521_i64) as isize;
_24 = -_11;
_17.fld1 = _9;
_17.fld3.1.0 = _17.fld0.1.0 | _17.fld2.fld5;
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(11_usize, 1_usize, Move(_1), 5_usize, Move(_5), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(11_usize, 13_usize, Move(_13), 15_usize, Move(_15), 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i32,mut _2: i128,mut _3: [u64; 2],mut _4: [u64; 2],mut _5: [u64; 2],mut _6: i128,mut _7: [u64; 2],mut _8: [u64; 2],mut _9: i128) -> isize {
mir! {
type RET = isize;
let _10: [u8; 8];
let _11: bool;
let _12: isize;
let _13: isize;
let _14: ([bool; 7], Adt79, u32, &'static char);
let _15: *mut i64;
let _16: &'static bool;
let _17: isize;
let _18: f64;
let _19: *mut i32;
let _20: f64;
let _21: i128;
let _22: Adt43;
let _23: (i128,);
let _24: &'static *mut (usize, *const i8, f32, Adt22);
let _25: isize;
let _26: i32;
let _27: [i32; 2];
let _28: *const *mut (usize, *const i8, f32, Adt22);
let _29: &'static [u64; 2];
let _30: f64;
let _31: (usize, *const i8, f32, Adt22);
let _32: i8;
let _33: u16;
let _34: isize;
let _35: u32;
let _36: (char, *const f64, *const f64);
let _37: [i64; 2];
let _38: (i32, usize, i8);
let _39: [i32; 2];
let _40: *mut i64;
let _41: i128;
let _42: (i8, (i32, usize, i8), (u128,), u128);
let _43: &'static (*mut &'static i32, (usize, *mut char));
let _44: (u128,);
let _45: i16;
let _46: i64;
let _47: f32;
let _48: u32;
let _49: &'static *mut (usize, *const i8, f32, Adt22);
let _50: u128;
let _51: &'static [i32; 1];
let _52: [u32; 2];
let _53: isize;
let _54: Adt58;
let _55: [i32; 1];
let _56: [bool; 7];
let _57: bool;
let _58: usize;
let _59: *mut u128;
let _60: &'static [bool; 1];
let _61: ();
let _62: ();
{
_7 = [12352251857410259781_u64,13382627437252142542_u64];
RET = (-29344_i16) as isize;
_3 = [8018306799655020780_u64,13164088906715801764_u64];
_1 = !(-1975269442_i32);
_6 = _2;
_3 = [622070937141377689_u64,3130854715735940097_u64];
_3 = _7;
_2 = _6 * _6;
_1 = true as i32;
_5 = [5301633039736105949_u64,3006498972169440849_u64];
_5 = [12081118179348470635_u64,3306813905118153440_u64];
_7 = [7322347654650109644_u64,16631902849866403651_u64];
Goto(bb1)
}
bb1 = {
_3 = _8;
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_1 = false as i32;
_2 = -_6;
_9 = _2;
_8 = [14063445329180602641_u64,4677604881070165018_u64];
RET = (-4040_i16) as isize;
_6 = _9 * _9;
_9 = _6 - _6;
_1 = -51661200_i32;
Goto(bb2)
}
bb2 = {
RET = (-73_isize);
_4 = [2781622140684272939_u64,8219541285593330686_u64];
_11 = !true;
_4 = [12418298505464188560_u64,13153501138675676671_u64];
_12 = RET * RET;
_13 = 277858365246986904426266143045681526476_u128 as isize;
_14.3 = &_14.1.fld1.3.fld1;
_14.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_14.1.fld2.fld0.fld1);
_14.2 = 1507243276_u32 - 3095767779_u32;
_14.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_14.1.fld2.fld0.fld1);
_14.1.fld1.3.fld1 = '\u{150fe}';
_14.1.fld2.fld0.fld3 = _14.1.fld1.3.fld1 as u32;
RET = -_12;
_11 = false ^ true;
_12 = -RET;
_10 = [222_u8,106_u8,79_u8,178_u8,159_u8,114_u8,138_u8,31_u8];
_14.1.fld2.fld0.fld0 = _14.1.fld1.3.fld1 as i128;
_14.1.fld3 = [RET,RET];
_14.3 = &_14.1.fld1.3.fld1;
_14.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_14.1.fld2.fld0.fld1);
Goto(bb3)
}
bb3 = {
_14.1.fld5 = [210_u8,152_u8,152_u8,82_u8,218_u8,48_u8,238_u8,153_u8];
_14.1.fld2.fld0.fld1 = _14.1.fld1.3.fld1;
_14.1.fld1.3.fld3 = _14.2 - _14.1.fld2.fld0.fld3;
_14.1.fld1.3.fld0 = _6;
_12 = RET;
_14.1.fld2.fld0.fld3 = 102_u8 as u32;
_14.1.fld1.3.fld5 = _1 & _1;
_10 = [52_u8,196_u8,64_u8,116_u8,63_u8,143_u8,191_u8,44_u8];
RET = _12 ^ _12;
RET = _12 & _12;
_14.1.fld2.fld0.fld6 = (-5054_i16) as f64;
_14.1.fld1.0 = 5_usize;
_14.1.fld1.0 = _14.1.fld2.fld0.fld6 as usize;
_14.1.fld2.fld0.fld2 = _14.1.fld1.3.fld5 as usize;
_14.1.fld2.fld0.fld5 = !_1;
_14.1.fld4 = (-11148_i16) * 18892_i16;
_9 = _2;
_4 = [2678929792556048217_u64,16801871373548675173_u64];
_17 = 7512780673351717993_u64 as isize;
_11 = true;
Goto(bb4)
}
bb4 = {
_14.1.fld1.3 = Adt22 { fld0: _2,fld1: _14.1.fld2.fld0.fld1,fld2: _14.1.fld2.fld0.fld2,fld3: _14.1.fld2.fld0.fld3,fld4: Move(_14.1.fld2.fld0.fld4),fld5: _1,fld6: _14.1.fld2.fld0.fld6 };
_14.1.fld2.fld0.fld4 = Move(_14.1.fld1.3.fld4);
_14.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_14.1.fld2.fld0.fld1);
_19 = core::ptr::addr_of_mut!(_1);
_14.1.fld1.3.fld3 = !_14.2;
_10 = _14.1.fld5;
_14.1.fld1.3.fld4 = Move(_14.1.fld2.fld0.fld4);
_5 = [12978337292439459828_u64,3232098835777475247_u64];
_14.1.fld1.2 = _14.1.fld2.fld0.fld6 as f32;
_1 = _14.1.fld2.fld0.fld5;
_16 = &_11;
_16 = &(*_16);
_25 = RET;
_5 = [17325312352765459272_u64,3403735797140770583_u64];
_26 = _14.1.fld2.fld0.fld5;
_29 = &_5;
_14.1.fld2.fld0 = Adt22 { fld0: _2,fld1: _14.1.fld1.3.fld1,fld2: _14.1.fld1.3.fld2,fld3: _14.1.fld1.3.fld3,fld4: Move(_14.1.fld1.3.fld4),fld5: (*_19),fld6: _14.1.fld1.3.fld6 };
_23 = (_14.1.fld1.3.fld0,);
Goto(bb5)
}
bb5 = {
_18 = _14.1.fld1.3.fld6;
_4 = _3;
_27 = [_14.1.fld1.3.fld5,(*_19)];
_14.1.fld1.3.fld2 = !_14.1.fld1.0;
_22 = Adt43::Variant0 { fld0: (*_16),fld1: _14.1.fld1.3.fld1,fld2: 207184873249786616109355492251386516987_u128,fld3: 94_i8,fld4: _14.1.fld1.3.fld2,fld5: _14.1.fld1.2,fld6: _14.1.fld1.3.fld6 };
_14.1.fld1.3 = Adt22 { fld0: _14.1.fld2.fld0.fld0,fld1: _14.1.fld2.fld0.fld1,fld2: _14.1.fld2.fld0.fld2,fld3: _14.2,fld4: Move(_14.1.fld2.fld0.fld4),fld5: _26,fld6: _14.1.fld2.fld0.fld6 };
_9 = -_14.1.fld2.fld0.fld0;
place!(Field::<char>(Variant(_22, 0), 1)) = _14.1.fld1.3.fld1;
_31.3.fld4 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_22, 0), 1)));
_21 = _6 + _14.1.fld2.fld0.fld0;
_14.1.fld2.fld0 = Adt22 { fld0: _6,fld1: Field::<char>(Variant(_22, 0), 1),fld2: Field::<usize>(Variant(_22, 0), 4),fld3: _14.1.fld1.3.fld3,fld4: Move(_31.3.fld4),fld5: _1,fld6: _18 };
_30 = Field::<f64>(Variant(_22, 0), 6) - _18;
_16 = &_11;
_12 = _13;
_14.1.fld0 = 44460_u16 as u32;
_16 = &(*_16);
_32 = !89_i8;
_33 = !41729_u16;
Goto(bb6)
}
bb6 = {
_14.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_14.1.fld1.3.fld1);
_9 = !_14.1.fld2.fld0.fld0;
_31.3.fld4 = Move(_14.1.fld1.3.fld4);
_14.1.fld1.3 = Adt22 { fld0: _14.1.fld2.fld0.fld0,fld1: Field::<char>(Variant(_22, 0), 1),fld2: Field::<usize>(Variant(_22, 0), 4),fld3: _14.2,fld4: Move(_31.3.fld4),fld5: _1,fld6: _18 };
Goto(bb7)
}
bb7 = {
_5 = [1585612186307276441_u64,493292508137183195_u64];
_14.1.fld5 = [3_u8,222_u8,90_u8,212_u8,239_u8,233_u8,198_u8,50_u8];
_14.1.fld1.3.fld5 = -_14.1.fld2.fld0.fld5;
_36.2 = core::ptr::addr_of!(_14.1.fld2.fld0.fld6);
Goto(bb8)
}
bb8 = {
_3 = _5;
_14.1.fld3 = [RET,RET];
place!(Field::<u128>(Variant(_22, 0), 2)) = 30947972262651070076730559123396723168_u128 + 227291948336406670809190411063930649118_u128;
place!(Field::<bool>(Variant(_22, 0), 0)) = (*_16);
_29 = &_7;
_38.0 = _14.1.fld1.3.fld5;
_31.3.fld4 = Move(_14.1.fld2.fld0.fld4);
_38.2 = _32;
_31.3.fld0 = -_6;
_36.0 = Field::<char>(Variant(_22, 0), 1);
RET = -_13;
_38.1 = _14.1.fld1.0;
place!(Field::<char>(Variant(_22, 0), 1)) = _36.0;
Call(_14.1.fld2.fld0.fld5 = fn13(Move(_29), _6, Move(_36.2), _14.1.fld3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_19) = _26 >> _2;
_14.1.fld3 = [_17,RET];
_1 = _38.0 * _38.0;
_31.0 = _38.1;
_42.1.0 = (*_19) - _14.1.fld1.3.fld5;
(*_19) = 3_u8 as i32;
_17 = _12 >> _14.1.fld0;
_14.1.fld4 = (*_16) as i16;
_16 = &place!(Field::<bool>(Variant(_22, 0), 0));
Goto(bb10)
}
bb10 = {
place!(Field::<bool>(Variant(_22, 0), 0)) = _14.1.fld1.3.fld1 > _36.0;
_14.1.fld2.fld0.fld6 = -_30;
_16 = &_11;
_38.1 = !_14.1.fld1.0;
_14.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_36.0);
_14.1.fld0 = _14.1.fld1.3.fld3 & _14.1.fld2.fld0.fld3;
_14.1.fld2.fld0 = Adt22 { fld0: _31.3.fld0,fld1: _36.0,fld2: _38.1,fld3: _14.1.fld0,fld4: Move(_31.3.fld4),fld5: (*_19),fld6: _30 };
_39 = [_14.1.fld1.3.fld5,_42.1.0];
_38.1 = _14.1.fld1.0;
_14.1.fld2.fld0.fld4 = core::ptr::addr_of_mut!(_14.1.fld1.3.fld1);
_42.1.0 = _26;
_42.2 = (Field::<u128>(Variant(_22, 0), 2),);
_27 = _39;
_14.1.fld2.fld0.fld5 = _14.1.fld1.3.fld5 + _1;
_9 = _21 | _21;
_14.1.fld4 = 27493_i16;
_48 = !_14.1.fld2.fld0.fld3;
_23 = (_9,);
_38 = (_14.1.fld2.fld0.fld5, _31.0, _32);
_14.1.fld2.fld0.fld2 = _31.0 ^ _14.1.fld1.0;
_14.1.fld1.3.fld5 = Field::<char>(Variant(_22, 0), 1) as i32;
Goto(bb11)
}
bb11 = {
_14.1.fld1.2 = -Field::<f32>(Variant(_22, 0), 5);
_19 = core::ptr::addr_of_mut!((*_19));
_7 = [12826362241052545963_u64,15626392380812742383_u64];
_14.1.fld1.3 = Adt22 { fld0: _21,fld1: _14.1.fld2.fld0.fld1,fld2: _14.1.fld1.0,fld3: _48,fld4: Move(_14.1.fld2.fld0.fld4),fld5: _1,fld6: _30 };
_1 = _14.1.fld2.fld0.fld5 * _14.1.fld1.3.fld5;
_38 = ((*_19), _14.1.fld1.0, _32);
_2 = Field::<u128>(Variant(_22, 0), 2) as i128;
match _14.1.fld4 {
0 => bb12,
27493 => bb14,
_ => bb13
}
}
bb12 = {
_14.1.fld5 = [210_u8,152_u8,152_u8,82_u8,218_u8,48_u8,238_u8,153_u8];
_14.1.fld2.fld0.fld1 = _14.1.fld1.3.fld1;
_14.1.fld1.3.fld3 = _14.2 - _14.1.fld2.fld0.fld3;
_14.1.fld1.3.fld0 = _6;
_12 = RET;
_14.1.fld2.fld0.fld3 = 102_u8 as u32;
_14.1.fld1.3.fld5 = _1 & _1;
_10 = [52_u8,196_u8,64_u8,116_u8,63_u8,143_u8,191_u8,44_u8];
RET = _12 ^ _12;
RET = _12 & _12;
_14.1.fld2.fld0.fld6 = (-5054_i16) as f64;
_14.1.fld1.0 = 5_usize;
_14.1.fld1.0 = _14.1.fld2.fld0.fld6 as usize;
_14.1.fld2.fld0.fld2 = _14.1.fld1.3.fld5 as usize;
_14.1.fld2.fld0.fld5 = !_1;
_14.1.fld4 = (-11148_i16) * 18892_i16;
_9 = _2;
_4 = [2678929792556048217_u64,16801871373548675173_u64];
_17 = 7512780673351717993_u64 as isize;
_11 = true;
Goto(bb4)
}
bb13 = {
_14.1.fld1.3.fld4 = core::ptr::addr_of_mut!(_14.1.fld1.3.fld1);
_9 = !_14.1.fld2.fld0.fld0;
_31.3.fld4 = Move(_14.1.fld1.3.fld4);
_14.1.fld1.3 = Adt22 { fld0: _14.1.fld2.fld0.fld0,fld1: Field::<char>(Variant(_22, 0), 1),fld2: Field::<usize>(Variant(_22, 0), 4),fld3: _14.2,fld4: Move(_31.3.fld4),fld5: _1,fld6: _18 };
Goto(bb7)
}
bb14 = {
_42.1.1 = _31.0;
_14.1.fld2.fld0.fld1 = _14.1.fld1.3.fld1;
_31.3.fld4 = core::ptr::addr_of_mut!(_14.1.fld1.3.fld1);
_23.0 = _14.1.fld1.3.fld0;
_31.3 = Move(_14.1.fld1.3);
_14.1.fld1.1 = core::ptr::addr_of!(_38.2);
_42.2.0 = Field::<u128>(Variant(_22, 0), 2);
_42.1 = (_14.1.fld2.fld0.fld5, _38.1, _32);
_14.3 = &_36.0;
_45 = !_14.1.fld4;
_14.1.fld0 = _14.2;
_31.3.fld0 = _14.1.fld2.fld0.fld3 as i128;
_31.3.fld5 = _30 as i32;
RET = _25 - _17;
_57 = (*_16) & (*_16);
_38 = _42.1;
_31.3.fld1 = _14.1.fld2.fld0.fld1;
_47 = Field::<f32>(Variant(_22, 0), 5) * Field::<f32>(Variant(_22, 0), 5);
place!(Field::<i8>(Variant(_22, 0), 3)) = 12810125845647007036_u64 as i8;
_51 = &_55;
_23.0 = (*_19) as i128;
_8 = [4286145194834973409_u64,15781825161762366048_u64];
_14.2 = !_48;
_42.1.2 = _38.2;
_59 = core::ptr::addr_of_mut!(_42.2.0);
Goto(bb15)
}
bb15 = {
Call(_61 = dump_var(12_usize, 45_usize, Move(_45), 17_usize, Move(_17), 33_usize, Move(_33), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_61 = dump_var(12_usize, 39_usize, Move(_39), 7_usize, Move(_7), 10_usize, Move(_10), 48_usize, Move(_48)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_61 = dump_var(12_usize, 27_usize, Move(_27), 26_usize, Move(_26), 12_usize, Move(_12), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_61 = dump_var(12_usize, 1_usize, Move(_1), 62_usize, _62, 62_usize, _62, 62_usize, _62), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: &'static [u64; 2],mut _2: i128,mut _3: *const f64,mut _4: [isize; 2]) -> i32 {
mir! {
type RET = i32;
let _5: (u8, isize, Adt38);
let _6: u64;
let _7: *const usize;
let _8: i32;
let _9: (i8, (i32, usize, i8), (u128,), u128);
let _10: [u8; 8];
let _11: (u128,);
let _12: bool;
let _13: char;
let _14: [isize; 2];
let _15: f32;
let _16: i16;
let _17: (usize, *const i8, f32, Adt22);
let _18: [bool; 1];
let _19: isize;
let _20: char;
let _21: u128;
let _22: *mut (usize, *const i8, f32, Adt22);
let _23: char;
let _24: isize;
let _25: u8;
let _26: *mut &'static i32;
let _27: f32;
let _28: &'static Adt58;
let _29: isize;
let _30: ();
let _31: ();
{
RET = (-1266641909_i32) * 9445396_i32;
RET = '\u{3cf9c}' as i32;
_4 = [(-79_isize),64_isize];
_4 = [(-9223372036854775808_isize),44_isize];
_5.1 = false as isize;
RET = _2 as i32;
Goto(bb1)
}
bb1 = {
_5.2.fld0 = 169_u8 as f32;
_5.0 = 172_u8;
_2 = 4698560284556665084249774217107875894_i128 >> RET;
RET = 1909589506_i32;
match RET {
0 => bb2,
1 => bb3,
1909589506 => bb5,
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
RET = -449982120_i32;
Goto(bb6)
}
bb6 = {
_4 = [_5.1,_5.1];
_5.2.fld0 = 5443365922572273498_i64 as f32;
RET = 1614826313528256752_i64 as i32;
_5.2.fld0 = 923768449_u32 as f32;
_5.0 = 61841_u16 as u8;
RET = (-1487950181_i32);
_4 = [_5.1,_5.1];
RET = !1861594567_i32;
RET = _5.0 as i32;
_4 = [_5.1,_5.1];
Goto(bb7)
}
bb7 = {
RET = !1770077683_i32;
_5.2.fld0 = 8527398603965996851_usize as f32;
_6 = !16318406458935240103_u64;
_6 = !10993660483502620881_u64;
_2 = (-8213689870686972995503666805987731962_i128);
_8 = RET - RET;
_6 = 62076629528215799_u64 + 510012740761357050_u64;
_9.1.2 = _5.0 as i8;
_8 = RET >> _2;
_9.2.0 = 22190855518854145061737750403133756543_u128 * 104867674697522869333189372134504059136_u128;
_7 = core::ptr::addr_of!(_9.1.1);
_10 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_9.1.2 = (-72_i8);
_9.2 = (290378372284353848659293429400125731447_u128,);
_12 = !false;
_5.2.fld1 = core::ptr::addr_of!(_9.0);
_13 = '\u{a73d3}';
_12 = true;
_8 = RET | RET;
Call(_14 = core::intrinsics::transmute(_4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9.0 = -_9.1.2;
_9.1.0 = _8;
_9.3 = (-7553_i16) as u128;
_9.3 = _9.2.0;
(*_7) = 2_usize & 9289428081680463323_usize;
_12 = !false;
RET = _9.1.2 as i32;
_5.1 = (-28_isize);
_9.1 = (RET, 17073547679280063540_usize, _9.0);
_17.3.fld3 = 1829318480_u32 & 54982333_u32;
_5.1 = _5.0 as isize;
_5.1 = 9223372036854775807_isize;
_9.1.2 = _17.3.fld3 as i8;
_13 = '\u{1cf17}';
_9.0 = _2 as i8;
_17.0 = (*_7) + (*_7);
_16 = (-2748_i16) + (-8592_i16);
_9.1 = (RET, _17.0, _9.0);
_11 = (_9.2.0,);
_17.3.fld4 = core::ptr::addr_of_mut!(_13);
_5.2.fld0 = _11.0 as f32;
_9.2.0 = _11.0;
_9.1 = (RET, _17.0, _9.0);
RET = _9.1.0 | _9.1.0;
match _9.2.0 {
0 => bb7,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb9,
290378372284353848659293429400125731447 => bb11,
_ => bb10
}
}
bb9 = {
RET = -449982120_i32;
Goto(bb6)
}
bb10 = {
_4 = [_5.1,_5.1];
_5.2.fld0 = 5443365922572273498_i64 as f32;
RET = 1614826313528256752_i64 as i32;
_5.2.fld0 = 923768449_u32 as f32;
_5.0 = 61841_u16 as u8;
RET = (-1487950181_i32);
_4 = [_5.1,_5.1];
RET = !1861594567_i32;
RET = _5.0 as i32;
_4 = [_5.1,_5.1];
Goto(bb7)
}
bb11 = {
(*_7) = _17.0 >> RET;
_17.1 = core::ptr::addr_of!(_9.0);
_18 = [_12];
_17.3.fld5 = !_9.1.0;
RET = _9.1.0 >> _17.0;
_5.2.fld0 = _5.0 as f32;
_17.3.fld0 = _2;
_12 = false;
_17.3.fld0 = _17.3.fld3 as i128;
_9.3 = _11.0;
_19 = !_5.1;
_22 = core::ptr::addr_of_mut!(_17);
(*_22).3.fld2 = (*_7) * (*_7);
(*_22).3.fld1 = _13;
(*_7) = (*_22).0;
_9.0 = _9.1.2 * _9.1.2;
_8 = _17.3.fld0 as i32;
_17.3.fld6 = _6 as f64;
(*_22).0 = _17.3.fld2;
Goto(bb12)
}
bb12 = {
(*_22).0 = (*_22).3.fld2;
_13 = (*_22).3.fld1;
(*_22).3.fld6 = _19 as f64;
(*_22).2 = RET as f32;
_8 = _17.3.fld5;
match _9.3 {
0 => bb9,
1 => bb5,
2 => bb13,
3 => bb14,
290378372284353848659293429400125731447 => bb16,
_ => bb15
}
}
bb13 = {
(*_7) = _17.0 >> RET;
_17.1 = core::ptr::addr_of!(_9.0);
_18 = [_12];
_17.3.fld5 = !_9.1.0;
RET = _9.1.0 >> _17.0;
_5.2.fld0 = _5.0 as f32;
_17.3.fld0 = _2;
_12 = false;
_17.3.fld0 = _17.3.fld3 as i128;
_9.3 = _11.0;
_19 = !_5.1;
_22 = core::ptr::addr_of_mut!(_17);
(*_22).3.fld2 = (*_7) * (*_7);
(*_22).3.fld1 = _13;
(*_7) = (*_22).0;
_9.0 = _9.1.2 * _9.1.2;
_8 = _17.3.fld0 as i32;
_17.3.fld6 = _6 as f64;
(*_22).0 = _17.3.fld2;
Goto(bb12)
}
bb14 = {
_4 = [_5.1,_5.1];
_5.2.fld0 = 5443365922572273498_i64 as f32;
RET = 1614826313528256752_i64 as i32;
_5.2.fld0 = 923768449_u32 as f32;
_5.0 = 61841_u16 as u8;
RET = (-1487950181_i32);
_4 = [_5.1,_5.1];
RET = !1861594567_i32;
RET = _5.0 as i32;
_4 = [_5.1,_5.1];
Goto(bb7)
}
bb15 = {
_5.2.fld0 = 169_u8 as f32;
_5.0 = 172_u8;
_2 = 4698560284556665084249774217107875894_i128 >> RET;
RET = 1909589506_i32;
match RET {
0 => bb2,
1 => bb3,
1909589506 => bb5,
_ => bb4
}
}
bb16 = {
_17.1 = Move(_5.2.fld1);
(*_22).3.fld3 = 3131740107_u32;
(*_22).3.fld5 = RET + RET;
_4 = [_19,_5.1];
(*_22).0 = _9.2.0 as usize;
(*_22).3.fld6 = (*_22).3.fld3 as f64;
(*_22).3.fld4 = core::ptr::addr_of_mut!(_17.3.fld1);
_9.2 = (_11.0,);
(*_22).3.fld1 = _13;
(*_22).3.fld5 = !RET;
(*_22).3.fld5 = _8 | _8;
_16 = (*_22).3.fld6 as i16;
_5.2.fld0 = _16 as f32;
_10 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_22 = core::ptr::addr_of_mut!((*_22));
_14 = [_19,_5.1];
_15 = (*_22).2;
_17.3.fld6 = _11.0 as f64;
_9.3 = _11.0 - _9.2.0;
_9.1.2 = _9.0 & _9.0;
_9.0 = 22225_u16 as i8;
_5.1 = _19;
(*_22).3.fld0 = _2 | _2;
(*_22).3.fld2 = (*_7);
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(13_usize, 13_usize, Move(_13), 16_usize, Move(_16), 18_usize, Move(_18), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(13_usize, 19_usize, Move(_19), 12_usize, Move(_12), 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static i32,mut _2: &'static char,mut _3: u128,mut _4: bool,mut _5: *mut u128,mut _6: isize) -> Adt22 {
mir! {
type RET = Adt22;
let _7: [bool; 4];
let _8: isize;
let _9: char;
let _10: f64;
let _11: *mut u128;
let _12: f64;
let _13: i16;
let _14: isize;
let _15: char;
let _16: usize;
let _17: isize;
let _18: [char; 3];
let _19: [u64; 2];
let _20: ();
let _21: ();
{
RET.fld5 = (-48123743_i32);
Goto(bb1)
}
bb1 = {
_8 = _6 * _6;
RET.fld2 = 5_usize | 825765677793357327_usize;
RET.fld4 = core::ptr::addr_of_mut!(RET.fld1);
RET.fld3 = 11_u8 as u32;
_9 = '\u{ab1f3}';
RET.fld2 = 16949320456697981833_usize;
RET.fld5 = -(-450981651_i32);
RET.fld1 = _9;
RET.fld1 = _9;
RET.fld0 = RET.fld1 as i128;
match RET.fld2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
16949320456697981833 => bb10,
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
_4 = _3 >= _3;
RET.fld2 = !0_usize;
RET.fld6 = (-18912_i16) as f64;
RET.fld4 = core::ptr::addr_of_mut!(RET.fld1);
_11 = Move(_5);
RET.fld4 = core::ptr::addr_of_mut!(RET.fld1);
RET.fld6 = RET.fld5 as f64;
RET.fld3 = !2437623086_u32;
_1 = &RET.fld5;
_2 = &_9;
RET.fld6 = RET.fld0 as f64;
_12 = _6 as f64;
RET.fld6 = -_12;
RET.fld0 = !(-16881261294020142704024241622886033345_i128);
_4 = _6 >= _6;
_11 = core::ptr::addr_of_mut!(_3);
_1 = &(*_1);
_7 = [_4,_4,_4,_4];
_7 = [_4,_4,_4,_4];
_5 = core::ptr::addr_of_mut!((*_11));
_12 = -RET.fld6;
RET.fld2 = RET.fld3 as usize;
(*_5) = 5030515685829531880_u64 as u128;
_13 = (-23974_i16);
_14 = _6 + _8;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb8,
340282366920938463463374607431768187482 => bb11,
_ => bb6
}
}
bb11 = {
_16 = !RET.fld2;
_4 = _13 != _13;
_4 = true;
RET.fld0 = 203_u8 as i128;
Goto(bb12)
}
bb12 = {
RET.fld2 = _14 as usize;
RET.fld0 = RET.fld2 as i128;
RET.fld3 = 145799881_u32 + 585859034_u32;
RET.fld0 = RET.fld5 as i128;
_6 = 8239394456479165432_u64 as isize;
_11 = core::ptr::addr_of_mut!(_3);
_12 = RET.fld6;
RET.fld3 = 6402560767177782554_i64 as u32;
RET.fld1 = _9;
(*_5) = !167780702796307650872221803444325888579_u128;
RET.fld6 = -_12;
RET.fld3 = !1746154145_u32;
_14 = _8;
_5 = core::ptr::addr_of_mut!(_3);
RET.fld3 = 2029319092_u32;
RET.fld6 = (-90_i8) as f64;
(*_5) = !87482057676047045395745065986187245857_u128;
_7 = [_4,_4,_4,_4];
_3 = 90927991842158289750150841642376600452_u128 + 57465495718653669013625842874506349577_u128;
RET.fld4 = core::ptr::addr_of_mut!((*_2));
RET.fld5 = 263806829_i32 << (*_11);
RET.fld6 = -_12;
RET.fld3 = !53010795_u32;
Goto(bb13)
}
bb13 = {
_3 = 181766333352193454717251185663217854478_u128;
_9 = RET.fld1;
_4 = RET.fld3 >= RET.fld3;
RET.fld5 = 1077829944_i32 & 850586304_i32;
RET.fld6 = _12;
RET.fld6 = RET.fld2 as f64;
(*_5) = 57997448153132386696427889664616846104_u128;
(*_5) = 149814732630286490809710018438109355486_u128;
RET.fld6 = -_12;
RET.fld2 = 13343551195916305176_u64 as usize;
RET.fld5 = !(-1123926127_i32);
_12 = RET.fld6 + RET.fld6;
RET.fld5 = 683177885_i32;
_15 = RET.fld1;
RET.fld2 = _16 + _16;
RET.fld1 = _9;
_7 = [_4,_4,_4,_4];
RET.fld2 = RET.fld3 as usize;
_10 = RET.fld6;
RET.fld0 = -54256052717795508414416759607150209867_i128;
_9 = _15;
Goto(bb14)
}
bb14 = {
_8 = !_14;
_12 = RET.fld6;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(14_usize, 9_usize, Move(_9), 15_usize, Move(_15), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(14_usize, 16_usize, Move(_16), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: u16,mut _3: isize,mut _4: [i32; 1],mut _5: u16,mut _6: isize,mut _7: char,mut _8: isize,mut _9: [i64; 2]) -> i32 {
mir! {
type RET = i32;
let _10: ((u128,), *const i8);
let _11: u32;
let _12: (char, *const f64, *const f64);
let _13: i32;
let _14: [i32; 2];
let _15: f32;
let _16: Adt38;
let _17: [i64; 2];
let _18: isize;
let _19: ((usize, *const i8, f32, Adt22),);
let _20: u16;
let _21: &'static i32;
let _22: [u64; 2];
let _23: ();
let _24: ();
{
_2 = 14734_i16 as u16;
_9 = [(-5951267557632751924_i64),(-5887328214676548524_i64)];
_7 = '\u{8b182}';
RET = 215944999_i32 + 1805637873_i32;
_9 = [7229767034019123873_i64,7920582491553924678_i64];
Goto(bb1)
}
bb1 = {
_10.0 = (311793400057486648888975783239648220455_u128,);
_10.0 = (304763695414141321752324603799112089304_u128,);
_1 = 9352698918702063883_u64 as isize;
_4 = [RET];
RET = 31_u8 as i32;
_1 = _10.0.0 as isize;
RET = 14471658325556424020_u64 as i32;
_10.0.0 = !124242343872509118620272369552171794229_u128;
_5 = _2;
_10.0.0 = !189785029605336157976353188797770072498_u128;
_10.0.0 = 69924920542274336380382114251841100559_u128 + 9855167777844499926597985609286887119_u128;
_9 = [(-2450144199199028672_i64),5912822461358526043_i64];
_12.0 = _7;
_12.0 = _7;
_8 = _3;
RET = 1349225099_i32;
_9 = [4918261143892920304_i64,5807036357068507629_i64];
RET = !1483905279_i32;
_11 = 112_u8 as u32;
_1 = -_8;
_11 = 3716408365_u32 * 1280238874_u32;
_14 = [RET,RET];
_10.0 = (275705433539211527592801078118692333695_u128,);
match _10.0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
275705433539211527592801078118692333695 => bb6,
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
_6 = _8;
RET = 187276194_i32;
_9 = [(-6797207714791862103_i64),(-5145054629116576026_i64)];
_9 = [3632772029345962552_i64,2462541873346108308_i64];
_15 = _2 as f32;
_11 = 4032083890_u32;
_2 = false as u16;
_13 = -RET;
_11 = 3629658576_u32;
_2 = !_5;
_8 = _1;
RET = _13;
_11 = 3253950903_u32 | 4073764722_u32;
_5 = _15 as u16;
_16.fld0 = _15 - _15;
_17 = [(-6543686604725049425_i64),(-5313781191762848640_i64)];
_8 = _1;
_8 = -_1;
_7 = _12.0;
_9 = [(-2910474816109949123_i64),7194730690092512117_i64];
Goto(bb7)
}
bb7 = {
_13 = _6 as i32;
RET = _11 as i32;
_6 = -_1;
_9 = [(-5831375430599681983_i64),(-2621873286565884823_i64)];
_11 = 2696143640_u32;
_14 = [_13,_13];
Goto(bb8)
}
bb8 = {
_1 = !_3;
_12.1 = core::ptr::addr_of!(_19.0.3.fld6);
_19.0.3.fld2 = 3_usize;
_18 = false as isize;
_12.0 = _7;
_7 = _12.0;
_19.0.3.fld5 = 2_i8 as i32;
_20 = (-28056888571357360722485515167806564677_i128) as u16;
_19.0.3.fld4 = core::ptr::addr_of_mut!(_12.0);
_3 = _1 ^ _8;
_12.2 = core::ptr::addr_of!(_19.0.3.fld6);
_19.0.3.fld3 = true as u32;
RET = _13 + _13;
_10.0 = (35584598547145971559898036875159854586_u128,);
_2 = _5;
_15 = -_16.fld0;
_7 = _12.0;
_19.0.3.fld1 = _12.0;
_17 = _9;
_2 = _20 >> _3;
_14 = [RET,RET];
_2 = !_5;
_12.0 = _7;
_6 = _3 >> _1;
_9 = _17;
Goto(bb9)
}
bb9 = {
Call(_23 = dump_var(15_usize, 3_usize, Move(_3), 18_usize, Move(_18), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_23 = dump_var(15_usize, 6_usize, Move(_6), 11_usize, Move(_11), 17_usize, Move(_17), 24_usize, _24), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: [i32; 1],mut _3: [u32; 2],mut _4: [u32; 2],mut _5: (isize, u32),mut _6: isize,mut _7: usize,mut _8: (isize, u32),mut _9: i32,mut _10: (isize, u32),mut _11: char) -> i8 {
mir! {
type RET = i8;
let _12: ((u128,), *const i8);
let _13: i128;
let _14: [bool; 1];
let _15: (*mut i32, i8, *mut u128, usize);
let _16: [bool; 7];
let _17: [bool; 4];
let _18: *const u32;
let _19: f64;
let _20: u16;
let _21: i8;
let _22: f64;
let _23: char;
let _24: isize;
let _25: f32;
let _26: bool;
let _27: u8;
let _28: i16;
let _29: *mut u128;
let _30: *mut u128;
let _31: *mut char;
let _32: (*mut i32, i8, *mut u128, usize);
let _33: bool;
let _34: *const i8;
let _35: [i32; 2];
let _36: f32;
let _37: char;
let _38: isize;
let _39: char;
let _40: usize;
let _41: char;
let _42: i64;
let _43: &'static i8;
let _44: (Adt38, usize, char, u8);
let _45: ();
let _46: ();
{
_11 = '\u{10671d}';
_10.0 = 29957_u16 as isize;
_12.0.0 = !90027683115618932283262600295211675913_u128;
_7 = 5_usize * 5_usize;
_8.1 = !_10.1;
RET = -(-41_i8);
RET = -(-56_i8);
_8.0 = -_6;
_8 = (_5.0, _10.1);
_8.0 = -_10.0;
_15.0 = core::ptr::addr_of_mut!(_9);
_15.0 = core::ptr::addr_of_mut!(_9);
Goto(bb1)
}
bb1 = {
_15.3 = !_7;
_14 = [true];
_12.0 = (191571802783373355977861989339313318014_u128,);
_17 = [false,true,true,true];
_9 = 527274879_i32;
_15.2 = core::ptr::addr_of_mut!(_12.0.0);
_15.1 = RET - RET;
RET = 9587323872100251528_u64 as i8;
_10.0 = _5.0 >> _7;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
527274879 => bb6,
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
_15.2 = core::ptr::addr_of_mut!(_12.0.0);
_10.1 = !_5.1;
RET = _15.1;
_2 = [_9];
_7 = !_15.3;
_15.1 = RET << _10.0;
_4 = [_5.1,_8.1];
_13 = !(-71045429937136912862016196373973805585_i128);
_12.0 = (188039014093731036834974224954145683880_u128,);
_3 = [_10.1,_10.1];
_18 = core::ptr::addr_of!(_5.1);
_9 = !561579868_i32;
_1 = -_6;
_20 = _8.1 as u16;
_12.1 = core::ptr::addr_of!(_21);
_19 = _20 as f64;
_6 = _10.0;
_10.1 = !(*_18);
_18 = core::ptr::addr_of!(_10.1);
_5.1 = _8.1;
Goto(bb7)
}
bb7 = {
_22 = -_19;
_10 = (_5.0, _8.1);
_5.0 = _15.3 as isize;
_4 = [(*_18),_8.1];
Goto(bb8)
}
bb8 = {
_15.3 = !_7;
_10 = (_1, _8.1);
_5.0 = _1;
RET = _15.1;
_19 = _22 * _22;
_10.0 = !_6;
_5 = (_6, (*_18));
_10.1 = _5.1;
_9 = -(-461262430_i32);
_5 = (_1, _10.1);
_22 = _19;
_23 = _11;
_12.0 = (168802748130056906581982644347101316080_u128,);
_30 = Move(_15.2);
_5.1 = !(*_18);
_23 = _11;
_26 = RET >= _15.1;
_21 = 6_u8 as i8;
_17 = [_26,_26,_26,_26];
_20 = 1546_u16 & 17166_u16;
Goto(bb9)
}
bb9 = {
_18 = core::ptr::addr_of!(_10.1);
_14 = [_26];
_11 = _23;
Goto(bb10)
}
bb10 = {
_34 = Move(_12.1);
_19 = _21 as f64;
_5.0 = _6;
_12.1 = Move(_34);
_15.1 = _20 as i8;
(*_18) = !_8.1;
_6 = _5.0;
_14 = [_26];
_27 = !237_u8;
Call(_35 = core::intrinsics::transmute(_3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = _21 << _8.1;
_26 = !false;
_8.1 = 6023821335125966543_u64 as u32;
_5.1 = !(*_18);
_12.0.0 = 268457013701484977970754736864349125851_u128 - 106937976851134072223034599396499837226_u128;
_8.1 = _10.1;
_32 = (Move(_15.0), RET, Move(_30), _15.3);
_37 = _11;
_18 = core::ptr::addr_of!(_8.1);
_5.1 = (*_18);
_8.1 = (-7107022036176568145_i64) as u32;
_12.0 = (305748988457307797794840870247995524591_u128,);
_1 = _10.0;
_9 = !(-2030136159_i32);
_25 = (-6256322135398154947_i64) as f32;
_15.2 = Move(_32.2);
_34 = Move(_12.1);
_10.0 = _6;
_10.1 = _20 as u32;
match _12.0.0 {
0 => bb8,
1 => bb5,
305748988457307797794840870247995524591 => bb12,
_ => bb3
}
}
bb12 = {
_7 = 25080_i16 as usize;
_38 = 13152190703151497794_u64 as isize;
_38 = _1 * _10.0;
_16 = [_26,_26,_26,_26,_26,_26,_26];
_12.1 = Move(_34);
Goto(bb13)
}
bb13 = {
_30 = Move(_15.2);
_6 = -_38;
_34 = Move(_12.1);
_32.1 = _12.0.0 as i8;
_32.3 = _15.3 * _7;
_34 = core::ptr::addr_of!(_32.1);
_32.2 = Move(_30);
_28 = RET as i16;
_18 = core::ptr::addr_of!((*_18));
_10.1 = 351904980734670505_i64 as u32;
_15 = (Move(_32.0), RET, Move(_32.2), _32.3);
_8.0 = _1;
_8 = (_10.0, _5.1);
_12.0.0 = 278931537902020691996145784307006485743_u128 | 333819149843430397315419902138158418209_u128;
_28 = -(-21791_i16);
_21 = _32.3 as i8;
_12.0 = (129683542400402181311290573147872899967_u128,);
_1 = _6;
_6 = _8.1 as isize;
_14 = [_26];
_21 = _15.1;
_12.0 = (180406951639838833130653387318909668467_u128,);
(*_34) = _25 as i8;
match _12.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb11,
180406951639838833130653387318909668467 => bb14,
_ => bb6
}
}
bb14 = {
_21 = -_15.1;
_44.3 = _27 | _27;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(16_usize, 17_usize, Move(_17), 35_usize, Move(_35), 21_usize, Move(_21), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(16_usize, 5_usize, Move(_5), 13_usize, Move(_13), 6_usize, Move(_6), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(16_usize, 37_usize, Move(_37), 4_usize, Move(_4), 27_usize, Move(_27), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: usize,mut _2: i128,mut _3: [i64; 2],mut _4: *mut char,mut _5: u32) -> usize {
mir! {
type RET = usize;
let _6: usize;
let _7: f64;
let _8: ([bool; 7], Adt79, u32, &'static char);
let _9: Adt38;
let _10: f64;
let _11: u128;
let _12: *mut i8;
let _13: isize;
let _14: &'static (u128,);
let _15: [isize; 2];
let _16: *const i8;
let _17: char;
let _18: f64;
let _19: &'static i32;
let _20: (u128,);
let _21: *const usize;
let _22: i64;
let _23: i128;
let _24: [bool; 1];
let _25: isize;
let _26: *const usize;
let _27: Adt22;
let _28: [i32; 2];
let _29: f32;
let _30: ();
let _31: ();
{
_2 = (-160804457811723962108384299768653307851_i128) | (-149065938669686553144303803256965782503_i128);
RET = !_1;
_1 = RET;
RET = !_1;
RET = _1;
RET = _1;
_1 = 33_i8 as usize;
_2 = (-136476482898550442098468786595618746556_i128) * (-115313590820748834753671843173982702586_i128);
_5 = 20461997209948457628818400350889532097_u128 as u32;
_6 = RET * RET;
RET = _5 as usize;
_5 = 375205448_u32;
_6 = _1 - _1;
Goto(bb1)
}
bb1 = {
_8.1.fld1.3.fld0 = 242_u8 as i128;
_9.fld0 = 10741808256910631339_u64 as f32;
_10 = 93644061252949091_u64 as f64;
_8.1.fld2.fld0.fld0 = (-113_i8) as i128;
_8.1.fld2.fld0.fld1 = '\u{f2a60}';
_8.1.fld2.fld0.fld2 = !_1;
_8.1.fld5 = [236_u8,194_u8,164_u8,135_u8,99_u8,32_u8,176_u8,168_u8];
_8.1.fld2.fld1 = Adt43::Variant0 { fld0: true,fld1: _8.1.fld2.fld0.fld1,fld2: 18689305007096575892329643216978936137_u128,fld3: 85_i8,fld4: _6,fld5: _9.fld0,fld6: _10 };
_8.1.fld1.3.fld6 = 49_u8 as f64;
Goto(bb2)
}
bb2 = {
_8.1.fld3 = [(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2)) = 281236093223955755847557675233144943347_u128;
_8.1.fld2.fld0.fld4 = Move(_4);
_8.1.fld1.3.fld4 = Move(_8.1.fld2.fld0.fld4);
_11 = Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2);
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _1 > _6;
_8.1.fld2.fld0.fld3 = _8.1.fld2.fld0.fld1 as u32;
place!(Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4)) = _6;
_8.1.fld1.3.fld6 = -_10;
_8.1.fld2.fld0.fld6 = -_8.1.fld1.3.fld6;
_7 = -_8.1.fld1.3.fld6;
_3 = [(-8108100903866352219_i64),(-8304532222735014430_i64)];
_8.1.fld0 = 5315053538765253040_u64 as u32;
_8.2 = _8.1.fld2.fld0.fld3;
_2 = -_8.1.fld1.3.fld0;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
_8.1.fld2.fld0.fld5 = (-1193822264_i32);
_8.1.fld2.fld0 = Adt22 { fld0: _2,fld1: Field::<char>(Variant(_8.1.fld2.fld1, 0), 1),fld2: Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4),fld3: _5,fld4: Move(_8.1.fld1.3.fld4),fld5: (-1107012601_i32),fld6: _10 };
_8.0 = [Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0),Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0),Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0),Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0),Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0),Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0),Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)];
place!(Field::<f64>(Variant(_8.1.fld2.fld1, 0), 6)) = _10 * _7;
_8.1.fld1.3.fld5 = _8.1.fld2.fld0.fld5 >> _8.2;
_8.1.fld2.fld1 = Adt43::Variant0 { fld0: true,fld1: _8.1.fld2.fld0.fld1,fld2: _11,fld3: 55_i8,fld4: _6,fld5: _9.fld0,fld6: _8.1.fld2.fld0.fld6 };
_8.1.fld2.fld0.fld1 = Field::<char>(Variant(_8.1.fld2.fld1, 0), 1);
match Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2) {
0 => bb1,
281236093223955755847557675233144943347 => bb4,
_ => bb3
}
}
bb3 = {
_8.1.fld1.3.fld0 = 242_u8 as i128;
_9.fld0 = 10741808256910631339_u64 as f32;
_10 = 93644061252949091_u64 as f64;
_8.1.fld2.fld0.fld0 = (-113_i8) as i128;
_8.1.fld2.fld0.fld1 = '\u{f2a60}';
_8.1.fld2.fld0.fld2 = !_1;
_8.1.fld5 = [236_u8,194_u8,164_u8,135_u8,99_u8,32_u8,176_u8,168_u8];
_8.1.fld2.fld1 = Adt43::Variant0 { fld0: true,fld1: _8.1.fld2.fld0.fld1,fld2: 18689305007096575892329643216978936137_u128,fld3: 85_i8,fld4: _6,fld5: _9.fld0,fld6: _10 };
_8.1.fld1.3.fld6 = 49_u8 as f64;
Goto(bb2)
}
bb4 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb5 = {
_8.1.fld2.fld0.fld3 = !_5;
_17 = _8.1.fld2.fld0.fld1;
_8.1.fld1.3 = Adt22 { fld0: _2,fld1: _17,fld2: _1,fld3: _8.1.fld0,fld4: Move(_8.1.fld2.fld0.fld4),fld5: _8.1.fld2.fld0.fld5,fld6: Field::<f64>(Variant(_8.1.fld2.fld1, 0), 6) };
_8.1.fld0 = _8.2;
_8.1.fld1.3.fld6 = _7;
_8.1.fld0 = _8.1.fld1.3.fld3 & _8.1.fld1.3.fld3;
_8.1.fld0 = !_8.1.fld1.3.fld3;
_20.0 = Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2);
_8.3 = &_17;
_8.1.fld2.fld0.fld4 = Move(_8.1.fld1.3.fld4);
match _8.1.fld1.3.fld5 {
340282366920938463463374607430661198855 => bb7,
_ => bb6
}
}
bb6 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb7 = {
_19 = &_8.1.fld2.fld0.fld5;
_12 = core::ptr::addr_of_mut!(place!(Field::<i8>(Variant(_8.1.fld2.fld1, 0), 3)));
_8.1.fld1.3.fld0 = _8.1.fld1.3.fld3 as i128;
_15 = [(-9223372036854775808_isize),80_isize];
_8.1.fld1.3 = Move(_8.1.fld2.fld0);
_8.1.fld2.fld0.fld3 = !_8.1.fld0;
_9.fld0 = Field::<f32>(Variant(_8.1.fld2.fld1, 0), 5);
_20 = (Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2),);
_8.1.fld1.2 = -Field::<f32>(Variant(_8.1.fld2.fld1, 0), 5);
_7 = _8.1.fld1.3.fld6 * Field::<f64>(Variant(_8.1.fld2.fld1, 0), 6);
_15 = [(-120_isize),9223372036854775807_isize];
_8.1.fld2.fld0.fld2 = _10 as usize;
_8.1.fld4 = (-7333_i16);
_9.fld1 = core::ptr::addr_of!((*_12));
_8.1.fld1.0 = Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4) * Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4);
_8.1.fld1.3.fld4 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)));
place!(Field::<i8>(Variant(_8.1.fld2.fld1, 0), 3)) = Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4) as i8;
_8.1.fld5 = [207_u8,100_u8,69_u8,25_u8,142_u8,89_u8,25_u8,124_u8];
RET = !_8.1.fld1.0;
place!(Field::<i8>(Variant(_8.1.fld2.fld1, 0), 3)) = (-53_i8);
RET = _8.1.fld4 as usize;
_20.0 = !Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2);
_8.2 = _8.1.fld4 as u32;
_4 = core::ptr::addr_of_mut!(_17);
_6 = _8.1.fld1.0;
_19 = &_8.1.fld2.fld0.fld5;
match _8.1.fld1.3.fld5 {
0 => bb8,
340282366920938463463374607430661198855 => bb10,
_ => bb9
}
}
bb8 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb9 = {
_8.1.fld2.fld0.fld3 = !_5;
_17 = _8.1.fld2.fld0.fld1;
_8.1.fld1.3 = Adt22 { fld0: _2,fld1: _17,fld2: _1,fld3: _8.1.fld0,fld4: Move(_8.1.fld2.fld0.fld4),fld5: _8.1.fld2.fld0.fld5,fld6: Field::<f64>(Variant(_8.1.fld2.fld1, 0), 6) };
_8.1.fld0 = _8.2;
_8.1.fld1.3.fld6 = _7;
_8.1.fld0 = _8.1.fld1.3.fld3 & _8.1.fld1.3.fld3;
_8.1.fld0 = !_8.1.fld1.3.fld3;
_20.0 = Field::<u128>(Variant(_8.1.fld2.fld1, 0), 2);
_8.3 = &_17;
_8.1.fld2.fld0.fld4 = Move(_8.1.fld1.3.fld4);
match _8.1.fld1.3.fld5 {
340282366920938463463374607430661198855 => bb7,
_ => bb6
}
}
bb10 = {
_6 = Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4) - _8.1.fld1.0;
(*_12) = 112_u8 as i8;
SetDiscriminant(_8.1.fld2.fld1, 0);
_9.fld0 = -_8.1.fld1.2;
match _8.1.fld1.3.fld5 {
0 => bb5,
1 => bb9,
2 => bb3,
3 => bb6,
340282366920938463463374607430661198855 => bb12,
_ => bb11
}
}
bb11 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb12 = {
_14 = &_20;
_6 = !RET;
RET = !_8.1.fld1.0;
_24 = [false];
_8.0 = [false,false,false,false,true,false,false];
_3 = [(-1215340796518647715_i64),(-2708831824219524287_i64)];
RET = _1;
_13 = 16558_u16 as isize;
RET = _2 as usize;
_8.1.fld2.fld0 = Adt22 { fld0: _2,fld1: (*_4),fld2: _8.1.fld1.3.fld2,fld3: _8.2,fld4: Move(_4),fld5: _8.1.fld1.3.fld5,fld6: _7 };
_3 = [3609401032543199999_i64,5825717843860424266_i64];
_18 = _8.1.fld2.fld0.fld6 + _8.1.fld2.fld0.fld6;
place!(Field::<f64>(Variant(_8.1.fld2.fld1, 0), 6)) = _8.1.fld2.fld0.fld6 - _18;
RET = _18 as usize;
place!(Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4)) = _8.1.fld1.0 >> _8.1.fld1.3.fld5;
_8.1.fld1.2 = _9.fld0;
_2 = _8.1.fld1.3.fld0 << _8.1.fld1.0;
_8.1.fld4 = (-17821_i16) - (-3169_i16);
_16 = Move(_9.fld1);
place!(Field::<usize>(Variant(_8.1.fld2.fld1, 0), 4)) = (*_14).0 as usize;
_8.1.fld2.fld0 = Adt22 { fld0: _2,fld1: _8.1.fld1.3.fld1,fld2: _1,fld3: _8.2,fld4: Move(_8.1.fld1.3.fld4),fld5: _8.1.fld1.3.fld5,fld6: _8.1.fld1.3.fld6 };
_19 = &_8.1.fld1.3.fld5;
match (*_19) {
0 => bb7,
1 => bb11,
2 => bb5,
3 => bb4,
4 => bb13,
5 => bb14,
340282366920938463463374607430661198855 => bb16,
_ => bb15
}
}
bb13 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb14 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb15 = {
_2 = -_8.1.fld2.fld0.fld0;
_8.3 = &_8.1.fld1.3.fld1;
_17 = _8.1.fld2.fld0.fld1;
place!(Field::<char>(Variant(_8.1.fld2.fld1, 0), 1)) = _8.1.fld2.fld0.fld1;
place!(Field::<bool>(Variant(_8.1.fld2.fld1, 0), 0)) = _8.1.fld2.fld0.fld6 < _8.1.fld1.3.fld6;
_8.1.fld0 = _8.1.fld2.fld0.fld3 >> _1;
_8.1.fld2.fld0.fld2 = 38_i8 as usize;
Goto(bb5)
}
bb16 = {
_8.1.fld0 = !_8.2;
_21 = core::ptr::addr_of!(_6);
_7 = -_8.1.fld2.fld0.fld6;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(17_usize, 20_usize, Move(_20), 15_usize, Move(_15), 17_usize, Move(_17), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(17_usize, 24_usize, Move(_24), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(63328_u16), std::hint::black_box(177146538990543251339453450199235467837_u128), std::hint::black_box(1933034134_u32), std::hint::black_box(48_u8));
                
            }
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt22{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt22 {
fld0: i128,
fld1: char,
fld2: usize,
fld3: u32,
fld4: *mut char,
fld5: i32,
fld6: f64,
}
impl PrintFDebug for Adt30{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt30{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt30 {
fld0: *mut *mut char,
fld1: u8,
fld2: *mut u128,
fld3: [i64; 2],
}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt34{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt34 {
fld0: (i8, (i32, usize, i8), (u128,), u128),
fld1: char,
fld2: Adt22,
fld3: (i16, (i32, usize, i8), *mut *mut char),
fld4: Adt30,
fld5: *const i8,
}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: f32,
fld1: *const i8,
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: char,
fld2: u128,
fld3: i8,
fld4: usize,
fld5: f32,
fld6: f64,

},
Variant1{
fld0: u8,
fld1: (*mut i32, i8, *mut u128, usize),
fld2: (i16, (i32, usize, i8), *mut *mut char),
fld3: (i32, usize, i8),
fld4: i16,
fld5: i32,
fld6: i64,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [isize; 2],
fld1: [bool; 4],
fld2: *mut (usize, *const i8, f32, Adt22),
fld3: f64,
fld4: (Adt38, usize, char, u8),
fld5: [u32; 2],
fld6: (usize, *const i8, f32, Adt22),

},
Variant1{
fld0: u64,
fld1: *mut *mut char,
fld2: *const i8,

}}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt69{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt69 {
fld0: Adt22,
fld1: Adt43,
}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt79{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt79 {
fld0: u32,
fld1: (usize, *const i8, f32, Adt22),
fld2: Adt69,
fld3: [isize; 2],
fld4: i16,
fld5: [u8; 8],
}

