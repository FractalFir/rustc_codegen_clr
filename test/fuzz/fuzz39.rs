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
pub fn fn0(mut _1: bool,mut _2: u16,mut _3: usize,mut _4: i8,mut _5: i128,mut _6: u64,mut _7: i64) -> u8 {
mir! {
type RET = u8;
let _8: f32;
let _9: f64;
let _10: ((isize, f32, isize, [bool; 3]), [bool; 3]);
let _11: char;
let _12: *mut char;
let _13: isize;
let _14: *const isize;
let _15: isize;
let _16: u128;
let _17: Adt48;
let _18: (i128,);
let _19: i128;
let _20: (i16,);
let _21: Adt58;
let _22: ([char; 1], u128);
let _23: bool;
let _24: i64;
let _25: (f32, i16, u64, i32, (i128,));
let _26: Adt53;
let _27: Adt53;
let _28: Adt54;
let _29: isize;
let _30: ();
let _31: ();
{
_2 = 7314_u16;
_3 = !6_usize;
_4 = (-82_i8) >> _2;
_7 = 220142332_i32 as i64;
_5 = !(-161721375399424391188917865129913626548_i128);
_4 = (-11_i8) * 65_i8;
Call(_8 = fn1(_7, _7, _2, _4, _4, _2, _3, _3, _4, _4, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 39_u8 | 134_u8;
_8 = (-24784_i16) as f32;
_9 = _5 as f64;
_2 = !64427_u16;
_6 = !3996180392332548262_u64;
Goto(bb2)
}
bb2 = {
_10.1 = [false,true,false];
_6 = 14856770411306082795_u64 ^ 17410508885163602519_u64;
RET = !193_u8;
_2 = 49179_u16 | 49797_u16;
_7 = 2100785358367733129_i64 & 8226145211671679031_i64;
_10.0.0 = '\u{10861e}' as isize;
_10.0.1 = _8 * _8;
Goto(bb3)
}
bb3 = {
_10.0.3 = [false,false,true];
RET = 85_u8;
_9 = 17906_i16 as f64;
_10.1 = [true,false,true];
_10.0.2 = 17521_i16 as isize;
_10.0.1 = _8;
_10.0.2 = _9 as isize;
_11 = '\u{ac619}';
_10.1 = _10.0.3;
RET = _3 as u8;
_6 = !17904048780711968146_u64;
_3 = 2_usize + 17487997224999679235_usize;
_3 = 988959737217440354_usize;
_3 = !4890048564427777477_usize;
_13 = _10.0.2 - _10.0.2;
RET = _5 as u8;
Goto(bb4)
}
bb4 = {
_10.0.3 = [false,false,true];
_6 = 4706188643138039235_u64 ^ 14304842274224249401_u64;
_10.0 = (_13, _8, _13, _10.1);
_3 = !0_usize;
_5 = (-44188066030314976060915575873299852629_i128);
_5 = -95566929568446047138151847282444795821_i128;
_6 = !2261779152868433370_u64;
Goto(bb5)
}
bb5 = {
_10.0.0 = _10.0.2 << _4;
_11 = '\u{6c291}';
_10.0.2 = -_10.0.0;
_11 = '\u{b6709}';
_12 = core::ptr::addr_of_mut!(_11);
_11 = '\u{d2737}';
_2 = 65180_u16;
_12 = core::ptr::addr_of_mut!(_11);
_10.0.0 = _10.0.2;
(*_12) = '\u{149b6}';
_10.0.1 = _8 + _8;
_8 = _10.0.1 - _10.0.1;
_2 = !46884_u16;
_6 = _8 as u64;
_7 = _10.0.2 as i64;
_1 = true & false;
_10.0 = (_13, _8, _13, _10.1);
_10.0.1 = -_8;
_12 = core::ptr::addr_of_mut!(_11);
_10.0.3 = [_1,_1,_1];
(*_12) = '\u{d442b}';
_10.1 = _10.0.3;
_8 = _10.0.1 * _10.0.1;
_4 = -14_i8;
_9 = _3 as f64;
_10.0 = (_13, _8, _13, _10.1);
_7 = _11 as i64;
_10.0.3 = [_1,_1,_1];
_9 = 14145_i16 as f64;
Goto(bb6)
}
bb6 = {
(*_12) = '\u{750af}';
_10.0.1 = _8;
RET = 66_u8 ^ 40_u8;
_8 = _10.0.1;
_6 = _1 as u64;
_10.0.0 = _9 as isize;
_10.0.0 = -_10.0.2;
_2 = 32161_u16;
_9 = (-1763782191_i32) as f64;
_6 = 15772712729095447990_u64;
_1 = true;
_14 = core::ptr::addr_of!(_15);
_1 = _10.0.0 < _10.0.0;
_3 = 10371824909328237968_usize + 16611500074146521019_usize;
_15 = _2 as isize;
_16 = !254549111801173326122249237424121186242_u128;
_10.1 = [_1,_1,_1];
(*_14) = _7 as isize;
_10.0.0 = _15 * _13;
_17.fld1.1 = _16 - _16;
_17.fld5.fld2.0 = [_10.0.2,(*_14),_10.0.0,_10.0.0];
RET = _6 as u8;
_11 = '\u{1090cb}';
_17.fld5.fld2.3.0 = [_1,_1,_1];
match _6 {
0 => bb7,
1 => bb8,
2 => bb9,
15772712729095447990 => bb11,
_ => bb10
}
}
bb7 = {
RET = 39_u8 | 134_u8;
_8 = (-24784_i16) as f32;
_9 = _5 as f64;
_2 = !64427_u16;
_6 = !3996180392332548262_u64;
Goto(bb2)
}
bb8 = {
_10.0.3 = [false,false,true];
_6 = 4706188643138039235_u64 ^ 14304842274224249401_u64;
_10.0 = (_13, _8, _13, _10.1);
_3 = !0_usize;
_5 = (-44188066030314976060915575873299852629_i128);
_5 = -95566929568446047138151847282444795821_i128;
_6 = !2261779152868433370_u64;
Goto(bb5)
}
bb9 = {
_10.0.3 = [false,false,true];
RET = 85_u8;
_9 = 17906_i16 as f64;
_10.1 = [true,false,true];
_10.0.2 = 17521_i16 as isize;
_10.0.1 = _8;
_10.0.2 = _9 as isize;
_11 = '\u{ac619}';
_10.1 = _10.0.3;
RET = _3 as u8;
_6 = !17904048780711968146_u64;
_3 = 2_usize + 17487997224999679235_usize;
_3 = 988959737217440354_usize;
_3 = !4890048564427777477_usize;
_13 = _10.0.2 - _10.0.2;
RET = _5 as u8;
Goto(bb4)
}
bb10 = {
_10.1 = [false,true,false];
_6 = 14856770411306082795_u64 ^ 17410508885163602519_u64;
RET = !193_u8;
_2 = 49179_u16 | 49797_u16;
_7 = 2100785358367733129_i64 & 8226145211671679031_i64;
_10.0.0 = '\u{10861e}' as isize;
_10.0.1 = _8 * _8;
Goto(bb3)
}
bb11 = {
_19 = !_5;
_17.fld4.3 = 2512295_u32 as i32;
_17.fld1.0 = [(*_12)];
_10.1 = [_1,_1,_1];
_16 = _15 as u128;
_17.fld6 = _9 + _9;
_20 = (16551_i16,);
_1 = false;
_22.0 = [_11];
_17.fld5.fld0 = (_17.fld5.fld2.3.0,);
_17.fld1.1 = !_16;
(*_14) = !_10.0.0;
_17.fld3 = _4;
_20 = (19906_i16,);
(*_12) = '\u{10344e}';
_20.0 = 4954_i16;
_8 = _10.0.1 - _10.0.1;
_17.fld1.0 = _22.0;
_16 = _17.fld4.3 as u128;
_20 = ((-10149_i16),);
_18 = (_19,);
_25.0 = -_8;
_17.fld5.fld2.2 = [RET];
_17.fld4.0 = _8;
_20.0 = !(-19703_i16);
Goto(bb12)
}
bb12 = {
_17.fld0 = -_17.fld4.0;
_17.fld5.fld0 = (_17.fld5.fld2.3.0,);
_25.3 = 2493340670_u32 as i32;
_17.fld4.2 = _6 | _6;
_17.fld4 = (_8, _20.0, _6, _25.3, _18);
_17.fld1.0 = _22.0;
_21 = Adt58 { fld0: _6 };
_26.fld2.2.3 = _10.0.3;
_27.fld1 = (*_12);
_26.fld2.1.3 = _17.fld5.fld2.3.0;
_26.fld2.2.2 = -_10.0.0;
_26.fld2.2 = ((*_14), _8, _13, _17.fld5.fld0.0);
_27.fld0 = [1752016908_u32,3427334152_u32,335732269_u32,2679493952_u32,2151328340_u32,640186443_u32];
_26.fld2.1.3 = _17.fld5.fld0.0;
_17.fld5.fld2.3 = _17.fld5.fld0;
match _17.fld4.2 {
15772712729095447990 => bb14,
_ => bb13
}
}
bb13 = {
_10.1 = [false,true,false];
_6 = 14856770411306082795_u64 ^ 17410508885163602519_u64;
RET = !193_u8;
_2 = 49179_u16 | 49797_u16;
_7 = 2100785358367733129_i64 & 8226145211671679031_i64;
_10.0.0 = '\u{10861e}' as isize;
_10.0.1 = _8 * _8;
Goto(bb3)
}
bb14 = {
_27.fld2.1.0 = _26.fld2.2.0 - (*_14);
_22.1 = _16;
_25.1 = _17.fld4.1 + _20.0;
_26.fld2.2.1 = _17.fld4.0 - _25.0;
_23 = _1;
_25.3 = !_17.fld4.3;
_18.0 = _17.fld0 as i128;
_27.fld0 = [1710393457_u32,4146522433_u32,1079085370_u32,3027075779_u32,3849391757_u32,4223417305_u32];
_27.fld2.1.0 = !_10.0.0;
_26.fld2.1.1 = _17.fld4.0 * _17.fld4.0;
_20.0 = 4288527040_u32 as i16;
_17.fld4.4.0 = _25.1 as i128;
_17.fld6 = -_9;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(0_usize, 4_usize, Move(_4), 2_usize, Move(_2), 20_usize, Move(_20), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(0_usize, 19_usize, Move(_19), 13_usize, Move(_13), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i64,mut _2: i64,mut _3: u16,mut _4: i8,mut _5: i8,mut _6: u16,mut _7: usize,mut _8: usize,mut _9: i8,mut _10: i8,mut _11: u16) -> f32 {
mir! {
type RET = f32;
let _12: u16;
let _13: (char, i8);
let _14: Adt44;
let _15: [bool; 3];
let _16: Adt52;
let _17: i32;
let _18: isize;
let _19: i64;
let _20: (char, i8);
let _21: bool;
let _22: Adt51;
let _23: bool;
let _24: isize;
let _25: i32;
let _26: isize;
let _27: ();
let _28: ();
{
_5 = _4;
RET = (-242004563_i32) as f32;
_3 = !_6;
_8 = _7;
RET = 33829724672188468938103210339780500690_i128 as f32;
RET = 79_u8 as f32;
_1 = -_2;
_1 = _2;
_4 = -_10;
_8 = _7;
_6 = 127449248714107509620842971766428204372_u128 as u16;
_3 = 14031198812634487559_u64 as u16;
_8 = _7 | _7;
_9 = _5 - _5;
_1 = -_2;
_2 = !_1;
Call(_8 = fn2(RET, _11, _9, _9, _9, _2, _5, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _4 | _9;
_4 = _3 as i8;
RET = 30112_i16 as f32;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
7314 => bb6,
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
RET = 429688885_i32 as f32;
_9 = !_5;
_6 = !_11;
RET = (-9223372036854775808_isize) as f32;
RET = (-151333065521614123493662662297887318077_i128) as f32;
_3 = 65601146143914347161673385047087206916_u128 as u16;
_6 = _3;
_4 = _5;
_12 = !_11;
_13 = ('\u{3c032}', _4);
_2 = _1;
_13.0 = '\u{9f303}';
_13 = ('\u{52071}', _9);
RET = 227_u8 as f32;
_10 = 25988_i16 as i8;
_3 = _12 % _11;
_11 = !_3;
_6 = _5 as u16;
_13.1 = !_5;
_12 = _6;
_7 = _8 + _8;
_2 = _1;
_6 = _13.0 as u16;
_12 = _6;
_15 = [true,false,false];
_5 = 149389248702506353840103945583345579205_i128 as i8;
_12 = _6 | _6;
_8 = _7 * _7;
Goto(bb7)
}
bb7 = {
_13.0 = '\u{57eaa}';
_5 = _4 * _13.1;
_4 = -_5;
_12 = !_6;
_9 = _1 as i8;
_14.fld0 = (_15,);
_15 = [false,true,true];
_14.fld0.0 = [true,true,true];
_14.fld2.2 = [162_u8];
_17 = (-1701187936_i32) ^ 364714342_i32;
_1 = _17 as i64;
_11 = _6 + _12;
_14.fld0 = (_15,);
_3 = _11;
_3 = 26292_i16 as u16;
_20.1 = !_4;
_11 = !_12;
_14.fld2.3 = _14.fld0;
_18 = !9223372036854775807_isize;
_14.fld2.1 = [40_u8,91_u8,66_u8,217_u8];
_19 = _1;
_12 = _17 as u16;
Goto(bb8)
}
bb8 = {
_2 = 316346784922770576845939277814430361502_u128 as i64;
_14.fld1 = _14.fld2.2;
_14.fld2.3.0 = [true,false,true];
_18 = 9223372036854775807_isize | 9223372036854775807_isize;
_6 = _12 | _11;
_8 = _7 & _7;
_13.1 = !_4;
_21 = !true;
RET = (-19653_i16) as f32;
_14.fld2.0 = [_18,_18,_18,_18];
_8 = _2 as usize;
_20 = (_13.0, _13.1);
_21 = _5 != _5;
_14.fld2.2 = _14.fld1;
_20.0 = _13.0;
_1 = !_19;
_14.fld0.0 = _14.fld2.3.0;
Goto(bb9)
}
bb9 = {
_13.0 = _20.0;
_4 = 111656112786736350436990634360971809193_i128 as i8;
RET = _5 as f32;
_15 = _14.fld2.3.0;
_23 = _21;
_2 = -_1;
_14.fld2.1 = [138_u8,169_u8,149_u8,100_u8];
_22.fld2 = _18;
_22.fld0 = _6 as i16;
_9 = _17 as i8;
_2 = _19 << _22.fld0;
_20.0 = _13.0;
Goto(bb10)
}
bb10 = {
Call(_27 = dump_var(1_usize, 19_usize, Move(_19), 4_usize, Move(_4), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_27 = dump_var(1_usize, 1_usize, Move(_1), 15_usize, Move(_15), 9_usize, Move(_9), 18_usize, Move(_18)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_27 = dump_var(1_usize, 11_usize, Move(_11), 20_usize, Move(_20), 28_usize, _28, 28_usize, _28), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: f32,mut _2: u16,mut _3: i8,mut _4: i8,mut _5: i8,mut _6: i64,mut _7: i8,mut _8: i8) -> usize {
mir! {
type RET = usize;
let _9: ((isize, f32, isize, [bool; 3]), [bool; 3]);
let _10: u64;
let _11: u8;
let _12: i16;
let _13: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3]));
let _14: Adt51;
let _15: isize;
let _16: ([char; 1], u128);
let _17: char;
let _18: f32;
let _19: &'static u8;
let _20: f32;
let _21: isize;
let _22: Adt47;
let _23: ((isize, f32, isize, [bool; 3]), [bool; 3]);
let _24: i128;
let _25: *const ([bool; 3],);
let _26: u8;
let _27: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _28: f32;
let _29: isize;
let _30: char;
let _31: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _32: Adt58;
let _33: i128;
let _34: isize;
let _35: Adt56;
let _36: [u8; 4];
let _37: u8;
let _38: u64;
let _39: char;
let _40: Adt47;
let _41: Adt44;
let _42: u64;
let _43: ([char; 1], u128);
let _44: bool;
let _45: &'static u8;
let _46: Adt52;
let _47: u16;
let _48: i32;
let _49: isize;
let _50: f32;
let _51: ();
let _52: ();
{
_1 = 9223372036854775807_isize as f32;
RET = 1111827544_i32 as usize;
_9.0.2 = !98_isize;
_6 = _4 as i64;
_7 = _5 + _4;
_9.1 = [true,false,true];
_9.0.0 = false as isize;
_8 = !_7;
_9.0 = (9223372036854775807_isize, _1, 9223372036854775807_isize, _9.1);
RET = 3212801540498401883_usize;
_10 = !10125231732316253028_u64;
_10 = 440618731_u32 as u64;
_9.0 = (9223372036854775807_isize, _1, (-121_isize), _9.1);
_1 = _9.0.1 * _9.0.1;
Goto(bb1)
}
bb1 = {
_7 = -_8;
_8 = _7;
_9.0.1 = _2 as f32;
_12 = 28216_i16;
_13.1.0 = _9.0.0 >> _12;
_6 = RET as i64;
_13.2.1 = _1 * _1;
_13.1.2 = _9.0.0;
_13.2.0 = -_13.1.2;
_13.2.1 = 4119957790_u32 as f32;
_9.0.0 = _9.0.1 as isize;
_13.1.3 = [false,true,false];
RET = 163743034_u32 as usize;
_9.0.2 = _13.1.0;
Call(_13.2.2 = fn3(_9, _9, _7, _8, _13.1.2, _13.1.2, _7, _9, _13.1.2, _3, _8, _9.0, _13.1.2, _13.1.2, _9, _13.2.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _12 as i8;
_2 = _12 as u16;
_8 = -_3;
_10 = !1219642487666759315_u64;
_9.0.1 = RET as f32;
_14.fld0 = _12;
_9.0.0 = _2 as isize;
_2 = 48764_u16;
_2 = !18375_u16;
_16.0 = ['\u{b5b70}'];
_13.0 = core::ptr::addr_of_mut!(_16.1);
Goto(bb3)
}
bb3 = {
_9.0.0 = true as isize;
_13.1.1 = RET as f32;
_9.0.0 = _13.1.0;
_13.2 = (_13.1.0, _1, _9.0.2, _13.1.3);
_14.fld1 = core::ptr::addr_of_mut!(_16.1);
Call(_16.1 = fn4(_13.2.0, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9.0 = (_13.2.2, _1, _13.2.2, _13.2.3);
_8 = 2072462189_i32 as i8;
_18 = _13.2.1;
Goto(bb5)
}
bb5 = {
_13.2 = (_9.0.2, _9.0.1, _9.0.2, _9.0.3);
_12 = !_14.fld0;
_10 = 7148386521808468003_u64;
_14.fld2 = _13.1.2 - _13.2.0;
_23.0.2 = -_13.1.2;
_4 = _7;
_11 = 22_u8 << _13.2.0;
_13.1.3 = [false,true,false];
Goto(bb6)
}
bb6 = {
_23.1 = [false,false,true];
_23.0.0 = _13.2.0;
_22.fld0 = [_11];
_1 = _13.2.1 - _18;
_20 = _1 * _9.0.1;
_6 = '\u{4b849}' as i64;
_9.0.1 = _6 as f32;
_2 = 39342_u16 ^ 10523_u16;
Call(_13.1.2 = fn16(_11, Move(_14), _13.1.3, _9.0, _8, _1, _5), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
match _16.1 {
0 => bb1,
1 => bb5,
2 => bb4,
219821316853589604606929928981876601706 => bb9,
_ => bb8
}
}
bb8 = {
_7 = _12 as i8;
_2 = _12 as u16;
_8 = -_3;
_10 = !1219642487666759315_u64;
_9.0.1 = RET as f32;
_14.fld0 = _12;
_9.0.0 = _2 as isize;
_2 = 48764_u16;
_2 = !18375_u16;
_16.0 = ['\u{b5b70}'];
_13.0 = core::ptr::addr_of_mut!(_16.1);
Goto(bb3)
}
bb9 = {
_13.1.0 = !_9.0.0;
_11 = 110_u8;
_31.0 = [_9.0.0,_13.1.2,_13.2.0,_13.1.0];
_14.fld0 = 2132171227_u32 as i16;
_22.fld0 = [_11];
_30 = '\u{80127}';
_22.fld5 = (_30, _3);
_14 = Adt51 { fld0: _12,fld1: _13.0,fld2: _13.2.0 };
_10 = !5384913049400451751_u64;
_27.3.0 = [false,false,false];
_22.fld5 = (_30, _3);
_13.0 = _14.fld1;
_31.1 = [_11,_11,_11,_11];
_22.fld5.0 = _30;
Goto(bb10)
}
bb10 = {
_9.0 = _13.1;
match _16.1 {
219821316853589604606929928981876601706 => bb12,
_ => bb11
}
}
bb11 = {
_7 = _12 as i8;
_2 = _12 as u16;
_8 = -_3;
_10 = !1219642487666759315_u64;
_9.0.1 = RET as f32;
_14.fld0 = _12;
_9.0.0 = _2 as isize;
_2 = 48764_u16;
_2 = !18375_u16;
_16.0 = ['\u{b5b70}'];
_13.0 = core::ptr::addr_of_mut!(_16.1);
Goto(bb3)
}
bb12 = {
_28 = 78306373095187724831539589716806456343_i128 as f32;
_22.fld2 = !894615277_u32;
_12 = _14.fld0 - _14.fld0;
_8 = _5 ^ _22.fld5.1;
_34 = _11 as isize;
_22.fld1 = _14.fld1;
_12 = !_14.fld0;
_13.0 = core::ptr::addr_of_mut!(_16.1);
_13 = (_14.fld1, _9.0, _9.0);
_13.2 = (_23.0.0, _20, _9.0.2, _9.1);
_9.0.0 = false as isize;
_13.2.2 = _9.0.2 | _9.0.2;
_27.2 = _22.fld0;
_15 = _14.fld2;
_22.fld3 = _11 * _11;
_31.3 = (_13.2.3,);
_24 = 169968900195078471657011656204553660241_i128 - 100378334298729354537894303662193478028_i128;
_11 = _22.fld3 & _22.fld3;
_10 = 14170906449565791974_u64;
Goto(bb13)
}
bb13 = {
_23.0 = (_9.0.2, _13.2.1, _13.2.2, _31.3.0);
_13.1.0 = _13.1.2 + _14.fld2;
_33 = _24 >> _23.0.0;
_37 = _11;
_22.fld1 = _13.0;
_40.fld1 = _22.fld1;
_9.0.3 = [true,true,true];
_41.fld0 = (_31.3.0,);
_29 = _33 as isize;
_22.fld2 = 4198070878_u32;
_43 = _16;
_36 = [_37,_11,_22.fld3,_22.fld3];
match _16.1 {
0 => bb11,
1 => bb2,
2 => bb4,
3 => bb14,
4 => bb15,
219821316853589604606929928981876601706 => bb17,
_ => bb16
}
}
bb14 = {
_28 = 78306373095187724831539589716806456343_i128 as f32;
_22.fld2 = !894615277_u32;
_12 = _14.fld0 - _14.fld0;
_8 = _5 ^ _22.fld5.1;
_34 = _11 as isize;
_22.fld1 = _14.fld1;
_12 = !_14.fld0;
_13.0 = core::ptr::addr_of_mut!(_16.1);
_13 = (_14.fld1, _9.0, _9.0);
_13.2 = (_23.0.0, _20, _9.0.2, _9.1);
_9.0.0 = false as isize;
_13.2.2 = _9.0.2 | _9.0.2;
_27.2 = _22.fld0;
_15 = _14.fld2;
_22.fld3 = _11 * _11;
_31.3 = (_13.2.3,);
_24 = 169968900195078471657011656204553660241_i128 - 100378334298729354537894303662193478028_i128;
_11 = _22.fld3 & _22.fld3;
_10 = 14170906449565791974_u64;
Goto(bb13)
}
bb15 = {
_9.0 = (_13.2.2, _1, _13.2.2, _13.2.3);
_8 = 2072462189_i32 as i8;
_18 = _13.2.1;
Goto(bb5)
}
bb16 = {
match _16.1 {
0 => bb1,
1 => bb5,
2 => bb4,
219821316853589604606929928981876601706 => bb9,
_ => bb8
}
}
bb17 = {
_9.0.2 = _8 as isize;
_41.fld2.1 = _36;
_31.2 = [_22.fld3];
_16 = _43;
_31.1 = [_22.fld3,_37,_22.fld3,_37];
_23.0 = _13.1;
_39 = _22.fld5.0;
_41.fld2.3.0 = [false,false,true];
_13.2 = (_13.1.0, _20, _9.0.2, _31.3.0);
_25 = core::ptr::addr_of!(_31.3);
_13.1.3 = [true,true,true];
Goto(bb18)
}
bb18 = {
Call(_51 = dump_var(2_usize, 4_usize, Move(_4), 36_usize, Move(_36), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_51 = dump_var(2_usize, 5_usize, Move(_5), 30_usize, Move(_30), 29_usize, Move(_29), 43_usize, Move(_43)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_51 = dump_var(2_usize, 16_usize, Move(_16), 2_usize, Move(_2), 33_usize, Move(_33), 52_usize, _52), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: ((isize, f32, isize, [bool; 3]), [bool; 3]),mut _2: ((isize, f32, isize, [bool; 3]), [bool; 3]),mut _3: i8,mut _4: i8,mut _5: isize,mut _6: isize,mut _7: i8,mut _8: ((isize, f32, isize, [bool; 3]), [bool; 3]),mut _9: isize,mut _10: i8,mut _11: i8,mut _12: (isize, f32, isize, [bool; 3]),mut _13: isize,mut _14: isize,mut _15: ((isize, f32, isize, [bool; 3]), [bool; 3]),mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: Adt58;
let _18: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _19: *mut f64;
let _20: f64;
let _21: char;
let _22: usize;
let _23: (i16,);
let _24: f32;
let _25: ();
let _26: ();
{
_8.0.3 = [false,true,false];
_15.0.2 = _13;
_6 = -_2.0.0;
_7 = 15177955881188178160_usize as i8;
_12.0 = _15.0.2 >> _8.0.2;
_15.0.3 = _2.1;
_8.1 = [false,false,false];
_10 = _7;
_2.0.0 = -_8.0.2;
_5 = _2.0.0 | _15.0.2;
_15.0 = (_1.0.2, _12.1, _1.0.2, _2.1);
_7 = -_11;
_2.0 = _1.0;
_12.1 = (-1518175678_i32) as f32;
_12.0 = _5;
_12.0 = _5 * _9;
RET = 27188_u16 as isize;
_15.0.0 = !_9;
RET = -_12.2;
Goto(bb1)
}
bb1 = {
_1 = (_12, _8.0.3);
_15.0.3 = [false,true,false];
_1.0 = _12;
_2.0.2 = _14 ^ RET;
RET = 4005381955_u32 as isize;
_2.0.2 = 4385827530602052164_u64 as isize;
_13 = -_8.0.2;
_1.0.0 = _12.0 >> _13;
_12.1 = -_8.0.1;
match _14 {
0 => bb2,
1 => bb3,
9223372036854775807 => bb5,
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
_2.0.0 = _9;
_12 = (_15.0.2, _2.0.1, _1.0.0, _2.1);
_13 = -_1.0.2;
_1.0.3 = [true,false,true];
_13 = (-21729231693834483979541104066595605761_i128) as isize;
_15.0.1 = _12.1;
_8 = (_1.0, _2.0.3);
_18.3 = (_15.1,);
_7 = _3;
_15.0 = (_12.0, _8.0.1, _2.0.0, _15.1);
_12 = (_2.0.0, _15.0.1, _16, _15.0.3);
_17.fld0 = 14479210334581607431_u64;
RET = _16 | _12.2;
_24 = -_1.0.1;
_13 = -_1.0.2;
_8.0.3 = [true,true,true];
_8.0 = (_15.0.2, _12.1, _15.0.0, _1.0.3);
_17.fld0 = 13422044940972655622_u64;
_23 = (29905_i16,);
Goto(bb6)
}
bb6 = {
Call(_25 = dump_var(3_usize, 13_usize, Move(_13), 23_usize, Move(_23), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_25 = dump_var(3_usize, 10_usize, Move(_10), 4_usize, Move(_4), 26_usize, _26, 26_usize, _26), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3]))) -> u128 {
mir! {
type RET = u128;
let _3: (i128,);
let _4: (i16,);
let _5: isize;
let _6: ([bool; 3],);
let _7: isize;
let _8: i16;
let _9: [bool; 3];
let _10: char;
let _11: [u64; 8];
let _12: (f32, i16, u64, i32, (i128,));
let _13: (f32, i16, u64, i32, (i128,));
let _14: Adt59;
let _15: Adt45;
let _16: i16;
let _17: u32;
let _18: Adt54;
let _19: isize;
let _20: isize;
let _21: isize;
let _22: bool;
let _23: bool;
let _24: [u32; 6];
let _25: *const ([bool; 3],);
let _26: Adt55;
let _27: Adt55;
let _28: Adt59;
let _29: ();
let _30: ();
{
RET = 257698983747173602996993179331551056867_u128 >> _2.2.0;
_2.1.0 = 25455_u16 as isize;
_2.2 = (_1, _2.1.1, _2.1.2, _2.1.3);
_2.1.1 = _2.2.0 as f32;
_2.2.2 = -_2.1.2;
RET = 219727136927958429922054431511405169954_u128;
RET = (-5590350220521041521_i64) as u128;
_2.1 = (_2.2.0, _2.2.1, _2.2.2, _2.2.3);
_2.0 = core::ptr::addr_of_mut!(RET);
_2.2.3 = [false,false,true];
_4.0 = 6291_i16;
_2.1 = _2.2;
_2.2.2 = _2.2.0;
_2.1.0 = -_1;
RET = 9980138360828127701_u64 as u128;
Goto(bb1)
}
bb1 = {
_2.1 = (_2.2.0, _2.2.1, _2.2.2, _2.2.3);
_6.0 = _2.1.3;
_10 = '\u{c0214}';
_2.1.3 = [true,true,true];
_9 = [false,true,true];
match _4.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6291 => bb9,
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
_2.1.0 = _2.1.1 as isize;
_4 = ((-16645_i16),);
_2.2.0 = _2.1.2 * _2.2.2;
RET = 137461484519868590518366315928678907081_u128 & 105074481225753031122024398443867357856_u128;
_2.0 = core::ptr::addr_of_mut!(RET);
_8 = !_4.0;
_3.0 = 51_i8 as i128;
_5 = _1;
_2.2.0 = 16266999247483259510_u64 as isize;
_2.2 = (_1, _2.1.1, _1, _9);
Call(_6.0 = fn5(_2.2, _2.1, _1, _2, _2.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Goto(bb11)
}
bb11 = {
_6.0 = _9;
_12.4 = _3;
_9 = _2.1.3;
_13 = (_2.2.1, _8, 15342006884697515178_u64, (-802624218_i32), _3);
_2.2.0 = _5;
_2.2 = (_1, _2.1.1, _1, _9);
_10 = '\u{7c7f9}';
_4 = (_8,);
_12.0 = _2.1.1;
_1 = _5 | _2.1.2;
_2.1 = _2.2;
_11 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_2.2.3 = _9;
_1 = _2.1.2 ^ _2.1.2;
_7 = _1;
_13.3 = !(-1244944012_i32);
_5 = !_7;
_7 = _2.2.0;
_3.0 = 59_i8 as i128;
_1 = _2.2.0 & _2.1.0;
_12.4 = (_3.0,);
_2.0 = core::ptr::addr_of_mut!(RET);
_12.2 = !_13.2;
_3.0 = -_13.4.0;
Call(_12.1 = core::intrinsics::transmute(_8), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = [true,true,false];
_3.0 = true as i128;
_12.0 = _13.0;
_12.1 = !_4.0;
_10 = '\u{f6304}';
_2.1.1 = (-4259281101346495487_i64) as f32;
_13 = (_2.2.1, _4.0, _12.2, (-1346357984_i32), _3);
_12.1 = _8;
RET = 219821316853589604606929928981876601706_u128;
_16 = _13.1;
_5 = _2.1.2;
_9 = _2.2.3;
_11 = [_12.2,_13.2,_12.2,_12.2,_13.2,_12.2,_13.2,_12.2];
_2.2.0 = _13.3 as isize;
_1 = !_2.1.2;
_19 = !_2.2.0;
_10 = '\u{e54d7}';
_2.2.3 = [false,true,false];
match _13.3 {
0 => bb1,
1 => bb10,
2 => bb9,
3 => bb4,
4 => bb7,
340282366920938463463374607430421853472 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_12.3 = _13.3 + _13.3;
_25 = core::ptr::addr_of!(_6);
_13 = (_2.1.1, _4.0, _12.2, _12.3, _3);
_13.4 = _3;
_2.2.1 = 45669_u16 as f32;
_2.1.2 = _19 ^ _19;
_2.2.0 = _4.0 as isize;
_27.fld0.3 = [true,true,true];
_26.fld2 = _12.3 << _19;
_26.fld1.0 = [_10];
_27.fld0.3 = [false,true,false];
_27.fld0.3 = [true,true,true];
(*_25).0 = _2.2.3;
_2.1.2 = _2.2.2;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(4_usize, 1_usize, Move(_1), 19_usize, Move(_19), 11_usize, Move(_11), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(4_usize, 6_usize, Move(_6), 4_usize, Move(_4), 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (isize, f32, isize, [bool; 3]),mut _2: (isize, f32, isize, [bool; 3]),mut _3: isize,mut _4: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3])),mut _5: *mut u128) -> [bool; 3] {
mir! {
type RET = [bool; 3];
let _6: f64;
let _7: Adt52;
let _8: Adt58;
let _9: *mut [i32; 3];
let _10: (f32, i16, u64, i32, (i128,));
let _11: *const ([bool; 3],);
let _12: *mut char;
let _13: ([bool; 3],);
let _14: Adt55;
let _15: f64;
let _16: Adt60;
let _17: Adt57;
let _18: isize;
let _19: *mut f64;
let _20: ((isize, f32, isize, [bool; 3]), [bool; 3]);
let _21: Adt59;
let _22: char;
let _23: i16;
let _24: (bool, *const ([bool; 3],), [u8; 4], u128);
let _25: [u8; 4];
let _26: (isize, f32, isize, [bool; 3]);
let _27: u16;
let _28: Adt45;
let _29: i128;
let _30: ([char; 1], u128);
let _31: ();
let _32: ();
{
RET = _2.3;
_1.1 = 59734_u16 as f32;
_4.0 = _5;
_2.1 = _1.1;
_4.2.2 = _3;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_2.3 = _4.1.3;
_3 = _1.0;
_1.0 = _4.2.0;
RET = _4.2.3;
_2.0 = _4.2.2 + _4.2.2;
_2.3 = [true,true,true];
_4.1.0 = _4.2.0;
_5 = core::ptr::addr_of_mut!((*_5));
Goto(bb3)
}
bb3 = {
_1.2 = 2486593487_u32 as isize;
(*_5) = 181189610108791159511718127623767094792_u128;
_2.2 = _2.0 + _2.0;
_4 = (_5, _2, _1);
Goto(bb4)
}
bb4 = {
RET = _2.3;
RET = [false,true,true];
_4 = (_5, _2, _1);
_1.0 = 1265465791_i32 as isize;
_6 = _4.1.2 as f64;
(*_5) = 221460441597807460411080524523417765679_u128 & 323455614670477053875492355796337955707_u128;
(*_5) = 316263344555557979502514561503950148918_u128 << _4.1.2;
_8.fld0 = 13483092329486395878_u64;
_2.2 = _4.2.0 >> _4.1.2;
_6 = _8.fld0 as f64;
_4.1.0 = _2.2;
_2 = _4.1;
(*_5) = 174215897515006224814006508160229827614_u128;
_1.0 = (-1482883434_i32) as isize;
_2.3 = _4.1.3;
_4.1 = (_2.2, _2.1, _2.2, _2.3);
_8 = Adt58 { fld0: 10226709938557323979_u64 };
RET = [true,false,true];
_4.2.3 = [true,true,true];
(*_5) = 126017396485832965198112166523371582203_u128;
RET = _4.1.3;
_4.2.2 = _4.2.0;
_2.1 = _1.1 * _4.2.1;
_4.1.3 = [true,true,false];
_4.2.2 = _2.2;
_8 = Adt58 { fld0: 17962602670190650109_u64 };
_4.2.0 = -_4.2.2;
_5 = core::ptr::addr_of_mut!((*_5));
Goto(bb5)
}
bb5 = {
_8.fld0 = !13535317670957750178_u64;
RET = _1.3;
_10.2 = _8.fld0;
(*_5) = 33949716384068985544065078193107366591_u128 >> _4.2.0;
_10.2 = _8.fld0 - _8.fld0;
_4.2.0 = _1.1 as isize;
_6 = (*_5) as f64;
_5 = core::ptr::addr_of_mut!((*_5));
_2.1 = _4.2.1;
_4.2.3 = [false,false,true];
_10.3 = 160_u8 as i32;
_2 = _4.1;
_8.fld0 = !_10.2;
Goto(bb6)
}
bb6 = {
_4.2.0 = -_4.1.0;
(*_5) = _6 as u128;
_10.3 = 1121340540_i32;
_1.2 = _3 << _2.2;
_5 = _4.0;
_10.4 = (111202638445807133604519478796793177381_i128,);
_4.1.1 = -_4.2.1;
_10.3 = _2.2 as i32;
_1.3 = [true,false,false];
(*_5) = !244804643546091290346175463880906745854_u128;
Call(_6 = fn6(_4, _1.2, _4.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4.2.1 = _1.1 - _4.1.1;
_4.0 = _5;
_8 = Adt58 { fld0: _10.2 };
_14.fld0.2 = _4.2.2;
_2.3 = [false,false,true];
_14.fld1.1 = (*_5) - (*_5);
_10.0 = _4.2.1 + _4.2.1;
_14.fld0.0 = (*_5) as isize;
_8 = Adt58 { fld0: _10.2 };
_1.0 = _4.2.2 * _4.1.0;
_8.fld0 = _10.2;
_4.1.3 = _1.3;
_4.1.1 = _10.0 - _4.2.1;
_1.2 = _4.1.0 - _2.0;
RET = [true,true,false];
_4.2 = (_2.2, _2.1, _1.2, _1.3);
_14.fld1.1 = !(*_5);
_8 = Adt58 { fld0: _10.2 };
Call(_4.2.2 = fn15(_1.0, _2, _2.0, _4.1.0, _10.0, _1.2, _1, _1.0, _4.1, _4.1, _1.2, _3, _4.2.0, _2.0, _4.1.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14.fld1.1 = (*_5) ^ (*_5);
_1.0 = _2.2 - _2.2;
_11 = core::ptr::addr_of!(_13);
(*_11) = (_4.2.3,);
_10.3 = _14.fld1.1 as i32;
_4.1.2 = _2.0 | _4.2.2;
_14.fld0 = (_1.2, _10.0, _1.0, (*_11).0);
(*_11).0 = [false,true,true];
_4 = (_5, _1, _14.fld0);
_19 = core::ptr::addr_of_mut!(_6);
_14.fld0 = _1;
_14.fld3 = (*_5) as i8;
_4.2.1 = -_4.1.1;
_4.2.3 = [false,false,true];
_11 = core::ptr::addr_of!((*_11));
_1.2 = _14.fld3 as isize;
_20.0 = (_1.2, _10.0, _14.fld0.0, _2.3);
_14.fld2 = _10.3;
Goto(bb9)
}
bb9 = {
RET = _14.fld0.3;
_8 = Adt58 { fld0: _10.2 };
_6 = (-5814811223397399573_i64) as f64;
_4.1.1 = -_10.0;
(*_11) = (_2.3,);
_26.3 = [true,true,true];
_2.3 = [true,true,false];
_26 = _4.2;
_2.2 = -_26.0;
Goto(bb10)
}
bb10 = {
_10.0 = _4.2.1;
_26.2 = _26.0 * _4.2.0;
match _10.4.0 {
111202638445807133604519478796793177381 => bb11,
_ => bb2
}
}
bb11 = {
_11 = core::ptr::addr_of!((*_11));
_1.3 = (*_11).0;
_10.2 = _8.fld0 & _8.fld0;
_2.2 = _26.0 & _4.1.0;
_14.fld0.0 = _2.2 << _4.2.2;
_20.0.2 = !_4.2.2;
_20.0.3 = (*_11).0;
_14.fld1.0 = ['\u{a4256}'];
_4.1.3 = [false,true,false];
_14.fld0.2 = _4.1.2;
_10.3 = -_14.fld2;
_20.1 = [false,true,true];
_12 = core::ptr::addr_of_mut!(_22);
match _10.4.0 {
0 => bb10,
1 => bb7,
2 => bb6,
3 => bb12,
4 => bb13,
5 => bb14,
111202638445807133604519478796793177381 => bb16,
_ => bb15
}
}
bb12 = {
_10.0 = _4.2.1;
_26.2 = _26.0 * _4.2.0;
match _10.4.0 {
111202638445807133604519478796793177381 => bb11,
_ => bb2
}
}
bb13 = {
_8.fld0 = !13535317670957750178_u64;
RET = _1.3;
_10.2 = _8.fld0;
(*_5) = 33949716384068985544065078193107366591_u128 >> _4.2.0;
_10.2 = _8.fld0 - _8.fld0;
_4.2.0 = _1.1 as isize;
_6 = (*_5) as f64;
_5 = core::ptr::addr_of_mut!((*_5));
_2.1 = _4.2.1;
_4.2.3 = [false,false,true];
_10.3 = 160_u8 as i32;
_2 = _4.1;
_8.fld0 = !_10.2;
Goto(bb6)
}
bb14 = {
_14.fld1.1 = (*_5) ^ (*_5);
_1.0 = _2.2 - _2.2;
_11 = core::ptr::addr_of!(_13);
(*_11) = (_4.2.3,);
_10.3 = _14.fld1.1 as i32;
_4.1.2 = _2.0 | _4.2.2;
_14.fld0 = (_1.2, _10.0, _1.0, (*_11).0);
(*_11).0 = [false,true,true];
_4 = (_5, _1, _14.fld0);
_19 = core::ptr::addr_of_mut!(_6);
_14.fld0 = _1;
_14.fld3 = (*_5) as i8;
_4.2.1 = -_4.1.1;
_4.2.3 = [false,false,true];
_11 = core::ptr::addr_of!((*_11));
_1.2 = _14.fld3 as isize;
_20.0 = (_1.2, _10.0, _14.fld0.0, _2.3);
_14.fld2 = _10.3;
Goto(bb9)
}
bb15 = {
_2.3 = _4.1.3;
_3 = _1.0;
_1.0 = _4.2.0;
RET = _4.2.3;
_2.0 = _4.2.2 + _4.2.2;
_2.3 = [true,true,true];
_4.1.0 = _4.2.0;
_5 = core::ptr::addr_of_mut!((*_5));
Goto(bb3)
}
bb16 = {
_10.4 = (57084856542903326566679764747599377868_i128,);
_14.fld1.1 = !(*_5);
_19 = core::ptr::addr_of_mut!((*_19));
_14.fld0.1 = _4.2.1 + _20.0.1;
_13 = (_4.2.3,);
_24.3 = _14.fld1.1 << _2.0;
_30 = (_14.fld1.0, _24.3);
_4.1 = (_2.2, _20.0.1, _2.0, _2.3);
(*_12) = '\u{10882c}';
_14.fld0 = (_26.0, _20.0.1, _2.2, (*_11).0);
_4.2.3 = [false,false,false];
_20.0.2 = (-20904_i16) as isize;
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(5_usize, 22_usize, Move(_22), 30_usize, Move(_30), 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3])),mut _2: isize,mut _3: isize) -> f64 {
mir! {
type RET = f64;
let _4: bool;
let _5: i16;
let _6: (f32, i16, u64, i32, (i128,));
let _7: *mut (char, i8);
let _8: u128;
let _9: (i16,);
let _10: f32;
let _11: Adt44;
let _12: f32;
let _13: (i128,);
let _14: isize;
let _15: [u64; 8];
let _16: char;
let _17: char;
let _18: (isize, f32, isize, [bool; 3]);
let _19: &'static u8;
let _20: u8;
let _21: isize;
let _22: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _23: ();
let _24: ();
{
_1.2 = _1.1;
_1.1.0 = (-1954923804_i32) as isize;
_1.1 = _1.2;
_1.1.1 = (-96_i8) as f32;
_1.1.2 = _2;
_1.1 = (_2, _1.2.1, _1.2.2, _1.2.3);
RET = (-22229_i16) as f64;
_1.1.0 = _1.1.2;
RET = 12_i8 as f64;
_1.2.1 = _1.1.1 - _1.1.1;
_1.1 = (_3, _1.2.1, _3, _1.2.3);
_1.2.1 = _1.1.1;
_2 = _1.1.0;
Goto(bb1)
}
bb1 = {
RET = (-82585842254437196736218395556732852870_i128) as f64;
_4 = false;
_1.2 = _1.1;
_1.1.1 = _1.2.1;
_5 = -(-7905_i16);
_6.4.0 = (-7316024648503732447362708844839249651_i128) >> _3;
_6.2 = _2 as u64;
_3 = _1.1.2 & _2;
RET = (-85_i8) as f64;
_6.4.0 = -(-153505532446032371692602124552190829821_i128);
_3 = -_2;
_4 = _1.2.2 >= _1.2.2;
_6.0 = _1.2.1 - _1.1.1;
_1.1.3 = [_4,_4,_4];
_1.1.0 = _1.2.0;
Goto(bb2)
}
bb2 = {
_1.1.3 = _1.2.3;
_1.1.2 = _1.2.0 | _3;
_1.2 = (_2, _6.0, _1.1.2, _1.1.3);
RET = 2867086091627733239_i64 as f64;
_4 = true & true;
_1.2.1 = _6.0;
_6.4.0 = (-99442090948006858278069570187272687398_i128);
_1.1.1 = -_1.2.1;
_6.1 = _5;
_1.1.0 = _1.1.2 | _1.2.0;
_1.1.3 = _1.2.3;
_1.1.1 = _6.0 * _1.2.1;
_6.1 = _5 ^ _5;
_1.1.3 = [_4,_4,_4];
_1.2.1 = _1.1.1 + _1.1.1;
_6.4.0 = -118705188626724884631933734318653058471_i128;
_4 = _1.2.2 == _1.1.0;
_1.1.3 = _1.2.3;
_1.1.1 = RET as f32;
_6.4 = ((-41322970745103044773802186917949400134_i128),);
Goto(bb3)
}
bb3 = {
_1.2.3 = _1.1.3;
_6.3 = 2041347064_i32 | (-783919936_i32);
Call(_1.2.1 = fn7(_1.1.0, _1.1.2, _1.2.2, _1.1, _4, _4, _1.1, _2, _1.1.0, _1.1.0, _1.1, RET, _1.1.2, _1.2.0, _3, _1.2.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1.2.0 = _1.1.0 >> _1.2.2;
_1.2.2 = _1.1.0;
_1.2 = (_1.1.0, _6.0, _2, _1.1.3);
_1.1 = (_1.2.0, _6.0, _1.2.0, _1.2.3);
_2 = 2892569963_u32 as isize;
match _6.4.0 {
0 => bb3,
298959396175835418689572420513818811322 => bb5,
_ => bb2
}
}
bb5 = {
_11.fld0.0 = [_4,_4,_4];
_8 = 27393829645427984922891799656289911848_u128 ^ 103218853661238824194098259167114459431_u128;
_11.fld2.1 = [157_u8,57_u8,178_u8,61_u8];
_6.0 = -_1.2.1;
_11.fld2.0 = [_1.2.0,_1.2.0,_1.2.0,_1.1.2];
_11.fld2.3.0 = _11.fld0.0;
_6.2 = 17363930426282525779_u64;
_1.1.2 = !_1.2.0;
_13 = (_6.4.0,);
RET = 182_u8 as f64;
_11.fld2.1 = [89_u8,198_u8,187_u8,191_u8];
match _6.4.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
298959396175835418689572420513818811322 => bb12,
_ => bb11
}
}
bb6 = {
_1.2.0 = _1.1.0 >> _1.2.2;
_1.2.2 = _1.1.0;
_1.2 = (_1.1.0, _6.0, _2, _1.1.3);
_1.1 = (_1.2.0, _6.0, _1.2.0, _1.2.3);
_2 = 2892569963_u32 as isize;
match _6.4.0 {
0 => bb3,
298959396175835418689572420513818811322 => bb5,
_ => bb2
}
}
bb7 = {
_1.2.3 = _1.1.3;
_6.3 = 2041347064_i32 | (-783919936_i32);
Call(_1.2.1 = fn7(_1.1.0, _1.1.2, _1.2.2, _1.1, _4, _4, _1.1, _2, _1.1.0, _1.1.0, _1.1, RET, _1.1.2, _1.2.0, _3, _1.2.2), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1.1.3 = _1.2.3;
_1.1.2 = _1.2.0 | _3;
_1.2 = (_2, _6.0, _1.1.2, _1.1.3);
RET = 2867086091627733239_i64 as f64;
_4 = true & true;
_1.2.1 = _6.0;
_6.4.0 = (-99442090948006858278069570187272687398_i128);
_1.1.1 = -_1.2.1;
_6.1 = _5;
_1.1.0 = _1.1.2 | _1.2.0;
_1.1.3 = _1.2.3;
_1.1.1 = _6.0 * _1.2.1;
_6.1 = _5 ^ _5;
_1.1.3 = [_4,_4,_4];
_1.2.1 = _1.1.1 + _1.1.1;
_6.4.0 = -118705188626724884631933734318653058471_i128;
_4 = _1.2.2 == _1.1.0;
_1.1.3 = _1.2.3;
_1.1.1 = RET as f32;
_6.4 = ((-41322970745103044773802186917949400134_i128),);
Goto(bb3)
}
bb9 = {
RET = (-82585842254437196736218395556732852870_i128) as f64;
_4 = false;
_1.2 = _1.1;
_1.1.1 = _1.2.1;
_5 = -(-7905_i16);
_6.4.0 = (-7316024648503732447362708844839249651_i128) >> _3;
_6.2 = _2 as u64;
_3 = _1.1.2 & _2;
RET = (-85_i8) as f64;
_6.4.0 = -(-153505532446032371692602124552190829821_i128);
_3 = -_2;
_4 = _1.2.2 >= _1.2.2;
_6.0 = _1.2.1 - _1.1.1;
_1.1.3 = [_4,_4,_4];
_1.1.0 = _1.2.0;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_5 = _6.1 + _6.1;
_6.4.0 = _13.0;
_11.fld2.3.0 = _11.fld0.0;
_6.4.0 = 149_u8 as i128;
_11.fld2.2 = [165_u8];
_9.0 = !_6.1;
_11.fld0.0 = [_4,_4,_4];
_8 = 228_u8 as u128;
_1.0 = core::ptr::addr_of_mut!(_8);
_11.fld0.0 = [_4,_4,_4];
_12 = _6.0;
_1.1.2 = _1.2.0;
match _6.2 {
0 => bb11,
1 => bb8,
2 => bb7,
17363930426282525779 => bb13,
_ => bb6
}
}
bb13 = {
_1.2.2 = (-110_i8) as isize;
_13 = (_6.4.0,);
Call(_13.0 = fn14(_1.1.0, _11.fld2.0, _1.1, _1.1.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11.fld2.1 = [144_u8,54_u8,79_u8,166_u8];
_1.2.3 = [_4,_4,_4];
_14 = !_1.1.0;
_13.0 = _6.4.0;
_11.fld2.3 = (_1.2.3,);
_1.1.1 = _6.0 - _6.0;
_11.fld2.3 = (_11.fld0.0,);
_10 = _1.1.1 - _12;
_13.0 = RET as i128;
_1.2.3 = [_4,_4,_4];
_11.fld0 = _11.fld2.3;
_9.0 = !_5;
RET = 681143089_u32 as f64;
_18.3 = [_4,_4,_4];
_18.1 = -_10;
_1.2.1 = 47815_u16 as f32;
_1.1.0 = _14;
_10 = _12 * _18.1;
_8 = _5 as u128;
_18.0 = _1.2.0 + _1.1.0;
_11.fld1 = [220_u8];
_1.0 = core::ptr::addr_of_mut!(_8);
_1.0 = core::ptr::addr_of_mut!(_8);
_13 = _6.4;
_1.1.0 = !_1.2.0;
_21 = RET as isize;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(6_usize, 3_usize, Move(_3), 4_usize, Move(_4), 5_usize, Move(_5), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: (isize, f32, isize, [bool; 3]),mut _5: bool,mut _6: bool,mut _7: (isize, f32, isize, [bool; 3]),mut _8: isize,mut _9: isize,mut _10: isize,mut _11: (isize, f32, isize, [bool; 3]),mut _12: f64,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> f32 {
mir! {
type RET = f32;
let _17: (bool, *const ([bool; 3],), [u8; 4], u128);
let _18: char;
let _19: Adt47;
let _20: bool;
let _21: char;
let _22: f64;
let _23: [u32; 6];
let _24: [u8; 1];
let _25: i8;
let _26: Adt56;
let _27: [u8; 1];
let _28: f32;
let _29: u8;
let _30: [i32; 3];
let _31: Adt55;
let _32: *mut u128;
let _33: [u32; 6];
let _34: i32;
let _35: i128;
let _36: Adt53;
let _37: isize;
let _38: ();
let _39: ();
{
_6 = _5 | _5;
_1 = _11.2 * _15;
_3 = _16;
_7.1 = _4.1 * _4.1;
_4 = _7;
_11.2 = 11829_u16 as isize;
_17.2 = [223_u8,199_u8,39_u8,172_u8];
_7.3 = _11.3;
_7.3 = _4.3;
_15 = (-515113657_i32) as isize;
_7.2 = !_10;
_7.0 = _16;
_19.fld1 = core::ptr::addr_of_mut!(_17.3);
_2 = -_3;
_21 = '\u{4e1c}';
_6 = !_5;
_6 = _5 ^ _5;
_17.0 = _6;
_1 = _10 * _11.0;
_5 = _17.0;
_19.fld3 = !88_u8;
_6 = _17.0 | _5;
RET = -_7.1;
_8 = !_14;
Goto(bb1)
}
bb1 = {
_7.1 = -_11.1;
_4.3 = [_5,_5,_6];
_17.3 = 125988431305465258631608280231559370994_u128 ^ 337465428600703800036110641593361698776_u128;
_6 = _5;
_23 = [3720230697_u32,2362476629_u32,3924285473_u32,235796724_u32,2695967277_u32,2426902491_u32];
_19.fld2 = 62058_u16 as u32;
_7.2 = _11.0;
_11.3 = _4.3;
_11 = (_7.0, _7.1, _4.0, _4.3);
_7.3 = _4.3;
_4.2 = -_16;
_7.2 = _4.2 + _7.0;
_13 = _7.0 >> _1;
_6 = _17.0 & _5;
Goto(bb2)
}
bb2 = {
_19.fld5.1 = !(-121_i8);
_15 = _14 + _4.0;
_2 = _10;
_17.3 = !272151042346217246673052240349048926103_u128;
_31.fld0.1 = 23307_u16 as f32;
_20 = _11.2 != _3;
_2 = _19.fld3 as isize;
_4 = _11;
_5 = _20;
_31.fld0.2 = _7.2;
_31.fld0.1 = _11.1 + _11.1;
_21 = '\u{80e36}';
_31.fld0.2 = _16 | _7.2;
_19.fld5 = (_21, 87_i8);
_29 = _6 as u8;
_7.2 = _14 - _7.0;
_31.fld1.0 = [_21];
Call(_31.fld3 = fn8(_6, _7.2, _11.2, _23, _15, _9, _1, _10, _9, _7.0, _7.0, _4, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = (_11.2, _11.1, _11.2, _11.3);
_19.fld0 = [_29];
_31.fld3 = _19.fld5.1 & _19.fld5.1;
_31.fld1.1 = _17.3;
_31.fld0.0 = _8 * _15;
_11.0 = !_16;
_28 = _29 as f32;
_31.fld1.0 = [_19.fld5.0];
_36.fld2.1 = (_11.0, _31.fld0.1, _8, _4.3);
_36.fld2.2.1 = (-99328751143687270092342150778489087922_i128) as f32;
_22 = -_12;
_19.fld5 = (_21, _31.fld3);
_11 = (_4.2, _28, _7.0, _4.3);
_31.fld2 = _19.fld2 as i32;
_36.fld2.1.2 = -_1;
_36.fld2.0 = _19.fld1;
_36.fld2 = (_19.fld1, _7, _11);
RET = -_36.fld2.2.1;
_22 = (-186948391122336648244260599671761156_i128) as f64;
_31.fld2 = -(-876454094_i32);
_31.fld0.1 = _11.1 * _11.1;
_7.0 = -_1;
Goto(bb4)
}
bb4 = {
Call(_38 = dump_var(7_usize, 2_usize, Move(_2), 15_usize, Move(_15), 21_usize, Move(_21), 20_usize, Move(_20)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_38 = dump_var(7_usize, 9_usize, Move(_9), 6_usize, Move(_6), 8_usize, Move(_8), 29_usize, Move(_29)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: [u32; 6],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: (isize, f32, isize, [bool; 3]),mut _13: isize,mut _14: isize) -> i8 {
mir! {
type RET = i8;
let _15: i128;
let _16: u16;
let _17: u8;
let _18: [i32; 3];
let _19: usize;
let _20: Adt48;
let _21: isize;
let _22: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _23: Adt57;
let _24: char;
let _25: Adt48;
let _26: ((isize, f32, isize, [bool; 3]), [bool; 3]);
let _27: i8;
let _28: (char, i8);
let _29: char;
let _30: f32;
let _31: bool;
let _32: ();
let _33: ();
{
_6 = !_11;
_7 = -_8;
_12.0 = -_10;
_4 = [525962468_u32,2862516058_u32,3429249135_u32,527041084_u32,3533314981_u32,3029433805_u32];
_15 = (-100563569017025867325442842280433662680_i128);
_12.0 = _8;
_1 = false;
_4 = [2377949547_u32,2271233687_u32,294862441_u32,1854158594_u32,805970799_u32,710073366_u32];
RET = 2676174460_u32 as i8;
RET = 97_i8;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
239718797903912596137931765151334548776 => bb7,
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
_11 = _12.0 | _12.2;
_16 = 3747_u16;
_15 = 3842750145559594192_u64 as i128;
_12.1 = 3254681586_u32 as f32;
RET = (-43_i8) >> _3;
_14 = _12.0 | _7;
_13 = _6 + _5;
_12.3 = [_1,_1,_1];
_14 = !_13;
_8 = _6;
_8 = 16800972266145085940_u64 as isize;
RET = _1 as i8;
_13 = _14 & _6;
_12.2 = _9;
_17 = !200_u8;
_11 = _12.0;
_7 = _10 ^ _13;
_12.3 = [_1,_1,_1];
_13 = _12.0 | _3;
_17 = !182_u8;
Call(_12.0 = fn9(_9, _5, _11, _13, _11, _11, _11, _10, _13), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = [536967375_u32,642916443_u32,1804409129_u32,3369425821_u32,1724966370_u32,294897682_u32];
_17 = _15 as u8;
_12.3 = [_1,_1,_1];
_12.1 = 12185_i16 as f32;
_11 = _2 - _10;
RET = 76_i8;
_7 = _3;
_14 = _9 ^ _9;
_16 = 42486_u16;
_2 = _12.2;
_20.fld5.fld2.0 = [_3,_12.0,_10,_11];
_20.fld4.1 = 17206203058745568657_u64 as i16;
_20.fld4.4 = (_15,);
_20.fld5.fld2.3 = (_12.3,);
_20.fld3 = -RET;
_20.fld4.0 = _12.2 as f32;
Call(_4 = fn10(_5, _13, _10, _7, _3, _6, _12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = -_7;
_18 = [827183079_i32,(-1974922983_i32),(-335604565_i32)];
_17 = _20.fld4.1 as u8;
_2 = !_10;
_12.1 = RET as f32;
_20.fld4.0 = -_12.1;
_2 = _14 & _14;
_20.fld0 = 13885088425776236648_u64 as f32;
_20.fld4.2 = !1063536728633750469_u64;
RET = !_20.fld3;
_19 = 9024438463514519262_usize * 15588722162032172593_usize;
_13 = !_7;
_13 = _20.fld4.0 as isize;
_21 = _14;
_19 = 12944499196798920593_usize ^ 14666435685858573956_usize;
_20.fld5.fld2.0 = [_14,_21,_12.2,_6];
RET = _1 as i8;
_17 = !159_u8;
_20.fld5.fld1 = [_17];
_20.fld5.fld1 = [_17];
_13 = _14 - _12.0;
_20.fld5.fld2.3.0 = [_1,_1,_1];
_22.1 = [_17,_17,_17,_17];
match _16 {
42486 => bb10,
_ => bb8
}
}
bb10 = {
_12.0 = RET as isize;
_15 = _3 as i128;
_20.fld5.fld0 = _20.fld5.fld2.3;
_22.3.0 = _12.3;
_20.fld5.fld0.0 = [_1,_1,_1];
RET = 2041018038_u32 as i8;
_25.fld5.fld2 = (_20.fld5.fld2.0, _22.1, _20.fld5.fld1, _22.3);
_25.fld4.1 = _20.fld4.1 & _20.fld4.1;
_22.2 = [_17];
_25.fld5.fld2.3 = (_22.3.0,);
_20.fld5.fld2.3 = (_22.3.0,);
_20.fld5.fld2.1 = _25.fld5.fld2.1;
_22.3 = (_20.fld5.fld0.0,);
_20.fld4.2 = 4318771265228343228_u64 * 15460050239427768325_u64;
_22 = _25.fld5.fld2;
_25.fld0 = _20.fld4.0;
_25.fld5.fld2.1 = [_17,_17,_17,_17];
_20.fld5.fld2.2 = [_17];
_20.fld5.fld1 = _22.2;
_22.2 = [_17];
_7 = -_11;
_26.1 = _20.fld5.fld2.3.0;
_20.fld6 = 4244524569484183556_i64 as f64;
_20.fld1.1 = 37822226293849108009184648926367555296_u128 ^ 109648878108696422270641399063733547997_u128;
_6 = _12.2;
_25.fld1.1 = !_20.fld1.1;
_20.fld2 = [144264335_u32,1592110458_u32,89552488_u32,763929398_u32,1014002906_u32,1900274643_u32];
Goto(bb11)
}
bb11 = {
_20.fld2 = _4;
_20.fld2 = [281289961_u32,112659858_u32,994354210_u32,1840715993_u32,1338829594_u32,2899098454_u32];
_9 = _11;
_25.fld1.1 = _20.fld1.1 | _20.fld1.1;
_25.fld4.3 = !1930884923_i32;
_20.fld1.0 = ['\u{b8784}'];
_25.fld4.4 = (_15,);
_20.fld4.0 = _20.fld1.1 as f32;
_25.fld4.4.0 = -_15;
_22.0 = _20.fld5.fld2.0;
_25.fld5.fld2.2 = [_17];
RET = _20.fld3 >> _2;
_28.0 = '\u{4c523}';
_25.fld2 = [2790808212_u32,4166617992_u32,3779180686_u32,3884971723_u32,2519332550_u32,2292710455_u32];
_20.fld4.1 = _25.fld4.1;
_25.fld5.fld0.0 = [_1,_1,_1];
_30 = _25.fld4.3 as f32;
_20.fld4.4 = _25.fld4.4;
_26.1 = [_1,_1,_1];
Goto(bb12)
}
bb12 = {
Call(_32 = dump_var(8_usize, 7_usize, Move(_7), 22_usize, Move(_22), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_32 = dump_var(8_usize, 10_usize, Move(_10), 2_usize, Move(_2), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_32 = dump_var(8_usize, 4_usize, Move(_4), 21_usize, Move(_21), 33_usize, _33, 33_usize, _33), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: [i32; 3];
let _11: f64;
let _12: *mut f64;
let _13: isize;
let _14: ();
let _15: ();
{
_10 = [1888614837_i32,(-1632165362_i32),(-725864356_i32)];
RET = 4154955326371250010_i64 as isize;
_7 = _2 << _4;
_8 = _4;
_5 = (-130481966594584195456889624968954598591_i128) as isize;
_4 = _8 * _1;
_1 = _7 + _9;
_1 = _6;
_11 = 8828354697182819732855843526818514587_u128 as f64;
_11 = _6 as f64;
_5 = !_7;
RET = 103_u8 as isize;
_6 = '\u{c231c}' as isize;
_4 = 193174635127221371940835209228781437252_u128 as isize;
_7 = _9 * _8;
_11 = 707197326_u32 as f64;
RET = _7 * _8;
_8 = 230860206955198790402170397482896204645_u128 as isize;
_13 = RET * _5;
_12 = core::ptr::addr_of_mut!(_11);
_4 = _3 - RET;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(9_usize, 3_usize, Move(_3), 5_usize, Move(_5), 13_usize, Move(_13), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(9_usize, 7_usize, Move(_7), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: (isize, f32, isize, [bool; 3])) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _8: f32;
let _9: isize;
let _10: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _11: &'static u8;
let _12: [i32; 3];
let _13: [isize; 4];
let _14: isize;
let _15: *const ([bool; 3],);
let _16: Adt58;
let _17: isize;
let _18: [u64; 8];
let _19: u32;
let _20: [u32; 6];
let _21: ([bool; 3],);
let _22: &'static u8;
let _23: [bool; 3];
let _24: [u32; 6];
let _25: u64;
let _26: Adt48;
let _27: Adt58;
let _28: Adt55;
let _29: Adt56;
let _30: i32;
let _31: char;
let _32: ([bool; 3],);
let _33: Adt51;
let _34: *mut (char, i8);
let _35: char;
let _36: u64;
let _37: ();
let _38: ();
{
_1 = 3_usize as isize;
_3 = -_7.0;
_7.3 = [true,false,true];
RET = [2534455647_u32,1419546875_u32,985144133_u32,202214937_u32,3912151271_u32,3182688246_u32];
_7.1 = (-1726_i16) as f32;
_5 = 444402003_i32 as isize;
RET = [3900462472_u32,3689984966_u32,76727524_u32,62796247_u32,1766592676_u32,1351428389_u32];
RET = [4110053753_u32,2313349970_u32,210103600_u32,3202894018_u32,1510948271_u32,3848572191_u32];
Call(_7.1 = fn11(_2, _6, _2, _2, _2, _7.2, _7.0, _4, _3, _2, _2, _2, _7.2, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _4 + _7.0;
_9 = (-8065850171946352389_i64) as isize;
_8 = -_7.1;
_1 = _7.2 << _2;
_7.1 = _8;
_7.1 = -_8;
_9 = _4 | _3;
_1 = _2 << _4;
_10.3 = (_7.3,);
_4 = _7.2 << _6;
_1 = _7.0 + _6;
RET = [3040026523_u32,3513019489_u32,2028415919_u32,2028955496_u32,2820202570_u32,1654050999_u32];
_6 = -_7.0;
_2 = (-33_i8) as isize;
_10.2 = [235_u8];
_16 = Adt58 { fld0: 13560237346800917437_u64 };
_4 = _6 | _1;
_10.1 = [213_u8,254_u8,238_u8,43_u8];
_7.0 = -_4;
_13 = [_3,_4,_7.0,_4];
_12 = [(-1813835547_i32),1676901307_i32,(-1235951275_i32)];
RET = [1588972273_u32,4233226215_u32,2554533780_u32,110455080_u32,2384155814_u32,41659279_u32];
Call(_7.2 = core::intrinsics::transmute(_7.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.0 = [_3,_9,_7.0,_3];
_9 = -_7.0;
_15 = core::ptr::addr_of!(_10.3);
_12 = [2096038870_i32,(-1030052942_i32),1224088245_i32];
_9 = -_3;
_18 = [_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0];
_14 = _6;
_12 = [(-1405332893_i32),2047509679_i32,(-788501640_i32)];
_8 = _7.1;
_7.1 = _8;
RET = [3695827361_u32,3980050225_u32,2045316496_u32,2257901089_u32,549023933_u32,1680006669_u32];
_17 = _3 * _14;
_10.3 = (_7.3,);
_16.fld0 = !6420477217004955943_u64;
_10.3.0 = [false,false,false];
_16.fld0 = !7546307515938416887_u64;
_18 = [_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0,_16.fld0];
_7.3 = [false,true,false];
_15 = core::ptr::addr_of!(_10.3);
(*_15) = (_7.3,);
_5 = _16.fld0 as isize;
_5 = !_7.0;
_20 = RET;
_21.0 = [true,true,true];
_15 = core::ptr::addr_of!((*_15));
Call(_10.0 = fn12(_13, _6, _7.0, _7.1, _6, _3, _6, _14, _14, _14, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10.2 = [37_u8];
(*_15).0 = [true,true,true];
_14 = 314854427_i32 as isize;
_10.2 = [169_u8];
_3 = _7.2;
_10.3 = _21;
_13 = _10.0;
_21.0 = (*_15).0;
_25 = 7305616618525522024_i64 as u64;
_1 = _9 & _7.0;
_7.2 = _7.0 ^ _1;
Goto(bb4)
}
bb4 = {
_14 = !_4;
_16 = Adt58 { fld0: _25 };
_26.fld5.fld0 = (_10.3.0,);
_26.fld5.fld2.3.0 = [true,true,true];
_10.2 = [72_u8];
_26.fld5.fld2.2 = _10.2;
(*_15).0 = _21.0;
_2 = _4;
RET = _20;
_26.fld5.fld2.0 = [_3,_7.2,_1,_1];
_10.1 = [154_u8,13_u8,141_u8,29_u8];
_7 = (_14, _8, _17, _26.fld5.fld0.0);
(*_15).0 = [false,false,true];
_26.fld5.fld2.3 = (_26.fld5.fld0.0,);
_10.2 = _26.fld5.fld2.2;
_23 = [true,false,false];
Goto(bb5)
}
bb5 = {
_7.1 = _7.2 as f32;
_15 = core::ptr::addr_of!(_26.fld5.fld2.3);
_24 = [2986211789_u32,3447071698_u32,2853174270_u32,3638218341_u32,1731113419_u32,1516061441_u32];
_26.fld4.4 = ((-10059419781287412441785157785305413866_i128),);
_9 = (-117_i8) as isize;
_26.fld0 = _7.1 - _7.1;
match _26.fld4.4.0 {
330222947139651051021589449646462797590 => bb7,
_ => bb6
}
}
bb6 = {
_2 = _4 + _7.0;
_9 = (-8065850171946352389_i64) as isize;
_8 = -_7.1;
_1 = _7.2 << _2;
_7.1 = _8;
_7.1 = -_8;
_9 = _4 | _3;
_1 = _2 << _4;
_10.3 = (_7.3,);
_4 = _7.2 << _6;
_1 = _7.0 + _6;
RET = [3040026523_u32,3513019489_u32,2028415919_u32,2028955496_u32,2820202570_u32,1654050999_u32];
_6 = -_7.0;
_2 = (-33_i8) as isize;
_10.2 = [235_u8];
_16 = Adt58 { fld0: 13560237346800917437_u64 };
_4 = _6 | _1;
_10.1 = [213_u8,254_u8,238_u8,43_u8];
_7.0 = -_4;
_13 = [_3,_4,_7.0,_4];
_12 = [(-1813835547_i32),1676901307_i32,(-1235951275_i32)];
RET = [1588972273_u32,4233226215_u32,2554533780_u32,110455080_u32,2384155814_u32,41659279_u32];
Call(_7.2 = core::intrinsics::transmute(_7.0), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_26.fld4.3 = _26.fld4.4.0 as i32;
_20 = RET;
_26.fld5.fld2.1 = [95_u8,88_u8,204_u8,48_u8];
_26.fld5.fld2.3 = (_10.3.0,);
_26.fld4.3 = (-273187690_i32) * (-592433940_i32);
_26.fld1.0 = ['\u{e6dc2}'];
_12 = [_26.fld4.3,_26.fld4.3,_26.fld4.3];
_26.fld3 = '\u{78d34}' as i8;
_15 = core::ptr::addr_of!(_26.fld5.fld2.3);
_26.fld4.4 = (86845933293974722246305307549661583156_i128,);
_18 = [_25,_25,_25,_16.fld0,_25,_25,_16.fld0,_25];
_13 = [_1,_2,_7.0,_7.2];
_26.fld1.1 = 36508870583141422855163377128456967224_u128;
_28.fld0 = _7;
_25 = !_16.fld0;
_26.fld5.fld2.1 = [201_u8,63_u8,153_u8,23_u8];
_28.fld1.1 = _26.fld1.1 / _26.fld1.1;
_10 = (_13, _26.fld5.fld2.1, _26.fld5.fld2.2, _26.fld5.fld0);
_30 = _26.fld4.3 + _26.fld4.3;
_23 = [false,false,true];
_24 = RET;
_26.fld5.fld2.0 = [_5,_4,_3,_28.fld0.0];
_19 = !827901178_u32;
_10.0 = [_7.2,_4,_28.fld0.2,_1];
_27.fld0 = !_25;
_26.fld4.4 = (130703211091092832744340647710091317469_i128,);
match _26.fld1.1 {
36508870583141422855163377128456967224 => bb8,
_ => bb1
}
}
bb8 = {
Call(_26.fld4.4 = fn13(_2, _26.fld5.fld2.0, _10.0, _2, _10, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_28.fld3 = -_26.fld3;
_15 = core::ptr::addr_of!(_21);
_6 = _4;
_32.0 = [false,false,true];
_31 = '\u{9c5dd}';
_7.0 = _28.fld0.1 as isize;
_26.fld4.1 = _19 as i16;
_28.fld0.0 = _4 + _1;
match _26.fld1.1 {
0 => bb4,
1 => bb2,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
36508870583141422855163377128456967224 => bb15,
_ => bb14
}
}
bb10 = {
Call(_26.fld4.4 = fn13(_2, _26.fld5.fld2.0, _10.0, _2, _10, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_26.fld4.3 = _26.fld4.4.0 as i32;
_20 = RET;
_26.fld5.fld2.1 = [95_u8,88_u8,204_u8,48_u8];
_26.fld5.fld2.3 = (_10.3.0,);
_26.fld4.3 = (-273187690_i32) * (-592433940_i32);
_26.fld1.0 = ['\u{e6dc2}'];
_12 = [_26.fld4.3,_26.fld4.3,_26.fld4.3];
_26.fld3 = '\u{78d34}' as i8;
_15 = core::ptr::addr_of!(_26.fld5.fld2.3);
_26.fld4.4 = (86845933293974722246305307549661583156_i128,);
_18 = [_25,_25,_25,_16.fld0,_25,_25,_16.fld0,_25];
_13 = [_1,_2,_7.0,_7.2];
_26.fld1.1 = 36508870583141422855163377128456967224_u128;
_28.fld0 = _7;
_25 = !_16.fld0;
_26.fld5.fld2.1 = [201_u8,63_u8,153_u8,23_u8];
_28.fld1.1 = _26.fld1.1 / _26.fld1.1;
_10 = (_13, _26.fld5.fld2.1, _26.fld5.fld2.2, _26.fld5.fld0);
_30 = _26.fld4.3 + _26.fld4.3;
_23 = [false,false,true];
_24 = RET;
_26.fld5.fld2.0 = [_5,_4,_3,_28.fld0.0];
_19 = !827901178_u32;
_10.0 = [_7.2,_4,_28.fld0.2,_1];
_27.fld0 = !_25;
_26.fld4.4 = (130703211091092832744340647710091317469_i128,);
match _26.fld1.1 {
36508870583141422855163377128456967224 => bb8,
_ => bb1
}
}
bb12 = {
_2 = _4 + _7.0;
_9 = (-8065850171946352389_i64) as isize;
_8 = -_7.1;
_1 = _7.2 << _2;
_7.1 = _8;
_7.1 = -_8;
_9 = _4 | _3;
_1 = _2 << _4;
_10.3 = (_7.3,);
_4 = _7.2 << _6;
_1 = _7.0 + _6;
RET = [3040026523_u32,3513019489_u32,2028415919_u32,2028955496_u32,2820202570_u32,1654050999_u32];
_6 = -_7.0;
_2 = (-33_i8) as isize;
_10.2 = [235_u8];
_16 = Adt58 { fld0: 13560237346800917437_u64 };
_4 = _6 | _1;
_10.1 = [213_u8,254_u8,238_u8,43_u8];
_7.0 = -_4;
_13 = [_3,_4,_7.0,_4];
_12 = [(-1813835547_i32),1676901307_i32,(-1235951275_i32)];
RET = [1588972273_u32,4233226215_u32,2554533780_u32,110455080_u32,2384155814_u32,41659279_u32];
Call(_7.2 = core::intrinsics::transmute(_7.0), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_7.1 = _7.2 as f32;
_15 = core::ptr::addr_of!(_26.fld5.fld2.3);
_24 = [2986211789_u32,3447071698_u32,2853174270_u32,3638218341_u32,1731113419_u32,1516061441_u32];
_26.fld4.4 = ((-10059419781287412441785157785305413866_i128),);
_9 = (-117_i8) as isize;
_26.fld0 = _7.1 - _7.1;
match _26.fld4.4.0 {
330222947139651051021589449646462797590 => bb7,
_ => bb6
}
}
bb14 = {
_14 = !_4;
_16 = Adt58 { fld0: _25 };
_26.fld5.fld0 = (_10.3.0,);
_26.fld5.fld2.3.0 = [true,true,true];
_10.2 = [72_u8];
_26.fld5.fld2.2 = _10.2;
(*_15).0 = _21.0;
_2 = _4;
RET = _20;
_26.fld5.fld2.0 = [_3,_7.2,_1,_1];
_10.1 = [154_u8,13_u8,141_u8,29_u8];
_7 = (_14, _8, _17, _26.fld5.fld0.0);
(*_15).0 = [false,false,true];
_26.fld5.fld2.3 = (_26.fld5.fld0.0,);
_10.2 = _26.fld5.fld2.2;
_23 = [true,false,false];
Goto(bb5)
}
bb15 = {
_19 = 1_usize as u32;
_33.fld0 = 87_u8 as i16;
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(10_usize, 32_usize, Move(_32), 24_usize, Move(_24), 31_usize, Move(_31), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(10_usize, 10_usize, Move(_10), 25_usize, Move(_25), 9_usize, Move(_9), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(10_usize, 12_usize, Move(_12), 23_usize, Move(_23), 20_usize, Move(_20), 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> f32 {
mir! {
type RET = f32;
let _16: u8;
let _17: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3]));
let _18: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _19: f32;
let _20: bool;
let _21: Adt55;
let _22: ();
let _23: ();
{
_7 = 19508_i16 as isize;
_13 = 32399_u16 as isize;
RET = 230_u8 as f32;
_5 = _2 - _12;
_14 = _15;
_2 = _3;
_17.2.2 = -_15;
RET = _14 as f32;
_8 = -_15;
_17.2.0 = _15 << _3;
_18.2 = [22_u8];
_17.2.3 = [false,false,true];
_11 = _17.2.0;
_21.fld0.2 = -_15;
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(11_usize, 11_usize, Move(_11), 8_usize, Move(_8), 15_usize, Move(_15), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(11_usize, 5_usize, Move(_5), 3_usize, Move(_3), 4_usize, Move(_4), 23_usize, _23), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [isize; 4],mut _2: isize,mut _3: isize,mut _4: f32,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: (isize, f32, isize, [bool; 3])) -> [isize; 4] {
mir! {
type RET = [isize; 4];
let _12: ();
let _13: ();
{
RET = _1;
_2 = 27051_u16 as isize;
_3 = _10;
_4 = 13061349662912560309863453463794443685_i128 as f32;
_4 = 6_usize as f32;
_10 = !_11.0;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(12_usize, 1_usize, Move(_1), 9_usize, Move(_9), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: [isize; 4],mut _3: [isize; 4],mut _4: isize,mut _5: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],)),mut _6: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],))) -> (i128,) {
mir! {
type RET = (i128,);
let _7: [bool; 3];
let _8: f32;
let _9: (char, i8);
let _10: isize;
let _11: Adt48;
let _12: (bool, *const ([bool; 3],), [u8; 4], u128);
let _13: char;
let _14: i64;
let _15: i32;
let _16: Adt53;
let _17: char;
let _18: *mut u128;
let _19: *mut f64;
let _20: char;
let _21: i16;
let _22: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _23: char;
let _24: f32;
let _25: f32;
let _26: ();
let _27: ();
{
_3 = [_1,_1,_1,_4];
_5.2 = [125_u8];
_5.0 = [_1,_1,_4,_1];
_5.0 = _6.0;
RET = (23323973412057708177531787465192037092_i128,);
_5.0 = _6.0;
_7 = [true,false,true];
_5.3.0 = _7;
_8 = (-7437542426163313713_i64) as f32;
_6.3 = (_5.3.0,);
_6.3.0 = [true,false,true];
_7 = _6.3.0;
Goto(bb1)
}
bb1 = {
_8 = RET.0 as f32;
RET.0 = 3696_u16 as i128;
_2 = _6.0;
_5.1 = [18_u8,209_u8,86_u8,209_u8];
_6.0 = [_4,_4,_4,_1];
_6.1 = [232_u8,66_u8,70_u8,220_u8];
Call(_3 = core::intrinsics::transmute(_5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.2 = [202_u8];
_6 = (_3, _5.1, _5.2, _5.3);
_1 = 122_u8 as isize;
_6.3.0 = [true,true,true];
_5.3 = (_7,);
_5.2 = [146_u8];
_3 = [_4,_4,_4,_4];
_6.1 = [176_u8,187_u8,5_u8,58_u8];
_10 = 337152838657876096446741973584290953515_u128 as isize;
RET.0 = 93351755826113365580758683239173434599_i128;
RET = ((-38523232843646853913217595063401290840_i128),);
RET.0 = !7337257583553042116025847769700675542_i128;
_5.0 = [_4,_4,_4,_4];
_11.fld2 = [4166028888_u32,3814600352_u32,2847612545_u32,3446172912_u32,4265933577_u32,2384533141_u32];
_7 = [false,false,true];
_11.fld1.0 = ['\u{c750e}'];
_9.1 = 67_i8;
_11.fld5.fld2 = (_3, _6.1, _5.2, _5.3);
_4 = (-6027707438595100412_i64) as isize;
_3 = [_1,_1,_1,_4];
_10 = !_1;
_11.fld1.1 = (-1062238136_i32) as u128;
_6.2 = _11.fld5.fld2.2;
_11.fld5.fld2.3.0 = [true,true,true];
_1 = _4 - _10;
_11.fld4.1 = _9.1 as i16;
_11.fld5.fld0 = _5.3;
match _9.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
67 => bb10,
_ => bb9
}
}
bb3 = {
_8 = RET.0 as f32;
RET.0 = 3696_u16 as i128;
_2 = _6.0;
_5.1 = [18_u8,209_u8,86_u8,209_u8];
_6.0 = [_4,_4,_4,_1];
_6.1 = [232_u8,66_u8,70_u8,220_u8];
Call(_3 = core::intrinsics::transmute(_5.0), ReturnTo(bb2), UnwindUnreachable())
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
_11.fld1.0 = ['\u{c97c9}'];
_6 = (_2, _5.1, _11.fld5.fld2.2, _11.fld5.fld0);
_5.0 = [_4,_4,_10,_10];
_12.2 = _5.1;
RET = (39303996531665321678735208426790316217_i128,);
_11.fld4.1 = (-10873_i16);
_11.fld4.4 = (RET.0,);
_11.fld4.4.0 = RET.0 >> _11.fld1.1;
_12.2 = _6.1;
_12.3 = 4243409927821506147_u64 as u128;
_6 = (_2, _12.2, _5.2, _5.3);
_3 = [_1,_1,_4,_1];
_11.fld3 = 11789940384244490123_usize as i8;
_6.3 = _5.3;
_11.fld5.fld1 = [183_u8];
_11.fld5.fld2.0 = _2;
_1 = _4 & _4;
_8 = 34981_u16 as f32;
_11.fld4.3 = 1925_u16 as i32;
_13 = '\u{9b69f}';
_11.fld5 = Adt44 { fld0: _6.3,fld1: _6.2,fld2: _6 };
_12.3 = _11.fld1.1;
_11.fld4.0 = _8 * _8;
_9.1 = _11.fld3;
_11.fld4 = (_8, (-28740_i16), 14454137535754257533_u64, 1215933707_i32, RET);
Goto(bb11)
}
bb11 = {
_12.1 = core::ptr::addr_of!(_11.fld5.fld0);
_5 = (_2, _12.2, _6.2, _6.3);
_16.fld2.2 = (_10, _8, _1, _5.3.0);
_12.2 = [133_u8,45_u8,141_u8,141_u8];
_8 = _9.1 as f32;
_17 = _13;
_5.0 = _6.0;
match _11.fld4.4.0 {
0 => bb1,
1 => bb6,
39303996531665321678735208426790316217 => bb12,
_ => bb4
}
}
bb12 = {
_16.fld2.0 = core::ptr::addr_of_mut!(_12.3);
_11.fld4.0 = _8 + _16.fld2.2.1;
_11.fld5.fld2.0 = _2;
_12.1 = core::ptr::addr_of!(_11.fld5.fld0);
_5 = _6;
_11.fld4.1 = 14826_i16 & (-31329_i16);
Goto(bb13)
}
bb13 = {
_11.fld4.1 = 10615_i16;
RET = _11.fld4.4;
_6.3.0 = [true,false,false];
_11.fld3 = _11.fld4.0 as i8;
_16.fld2.2.2 = _4;
_11.fld5.fld2 = (_6.0, _5.1, _6.2, _5.3);
_11.fld0 = _16.fld2.2.1 * _11.fld4.0;
_19 = core::ptr::addr_of_mut!(_11.fld6);
_16.fld2.2.1 = -_8;
match _11.fld4.3 {
0 => bb10,
1215933707 => bb14,
_ => bb2
}
}
bb14 = {
_9 = (_17, _11.fld3);
_11.fld4.0 = -_11.fld0;
_11.fld5.fld0.0 = [true,true,true];
_2 = _11.fld5.fld2.0;
_16.fld1 = _9.0;
_18 = _16.fld2.0;
_19 = core::ptr::addr_of_mut!(_11.fld6);
_16.fld0 = _11.fld2;
_5.3.0 = [false,false,true];
_11.fld5.fld2.3.0 = [true,true,false];
_5.3 = (_11.fld5.fld0.0,);
_16.fld2.2.0 = _11.fld4.3 as isize;
_22.3 = _5.3;
_11.fld5.fld0 = (_6.3.0,);
_16.fld1 = _13;
_9.1 = _11.fld3 - _11.fld3;
_16.fld2.1.0 = _16.fld2.2.0 >> _16.fld2.2.0;
_14 = !484094033470938464_i64;
_17 = _16.fld1;
_24 = _11.fld4.0;
_22 = _11.fld5.fld2;
_6.3.0 = [false,false,false];
(*_19) = _9.1 as f64;
_7 = _16.fld2.2.3;
_23 = _16.fld1;
RET.0 = _11.fld4.4.0 >> _11.fld4.3;
_3 = [_16.fld2.1.0,_1,_16.fld2.2.0,_16.fld2.1.0];
RET.0 = 5_usize as i128;
_19 = core::ptr::addr_of_mut!((*_19));
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(13_usize, 2_usize, Move(_2), 17_usize, Move(_17), 4_usize, Move(_4), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(13_usize, 3_usize, Move(_3), 6_usize, Move(_6), 23_usize, Move(_23), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: [isize; 4],mut _3: (isize, f32, isize, [bool; 3]),mut _4: isize) -> i128 {
mir! {
type RET = i128;
let _5: [u32; 6];
let _6: Adt51;
let _7: f64;
let _8: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],));
let _9: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3]));
let _10: *mut [i32; 3];
let _11: f64;
let _12: (bool, *const ([bool; 3],), [u8; 4], u128);
let _13: isize;
let _14: Adt60;
let _15: u32;
let _16: f64;
let _17: u16;
let _18: f64;
let _19: [char; 1];
let _20: i32;
let _21: (i128,);
let _22: i32;
let _23: Adt58;
let _24: ([char; 1], u128);
let _25: ([char; 1], u128);
let _26: isize;
let _27: u128;
let _28: [u32; 6];
let _29: Adt49;
let _30: u8;
let _31: isize;
let _32: f64;
let _33: [isize; 4];
let _34: char;
let _35: *mut f64;
let _36: Adt44;
let _37: [i32; 3];
let _38: ();
let _39: ();
{
RET = 85_u8 as i128;
_2 = [_4,_1,_3.0,_3.0];
Goto(bb1)
}
bb1 = {
_2 = [_1,_3.2,_1,_3.2];
_1 = !_3.0;
_6.fld0 = -16696_i16;
_6.fld0 = (-15390_i16) >> _3.0;
_4 = 2071801364_u32 as isize;
_6.fld0 = (-30135_i16) + 18519_i16;
_5 = [2873673406_u32,137838498_u32,2437533088_u32,3235001804_u32,3868976373_u32,34472040_u32];
_3.3 = [true,true,false];
_3.1 = 188_u8 as f32;
_4 = 1453549601286488830_i64 as isize;
_5 = [1730605167_u32,1157882933_u32,2415687664_u32,2094379861_u32,3341052792_u32,3721224348_u32];
_2 = [_3.2,_3.0,_1,_3.0];
_6.fld2 = 139945259_u32 as isize;
_6.fld0 = 1912662007_i32 as i16;
RET = !(-87140398096408725295583783977507721753_i128);
_3.2 = (-124_i8) as isize;
_2 = [_1,_1,_1,_3.0];
_2 = [_3.0,_4,_1,_3.0];
RET = -12374163465518407507346868456979579149_i128;
_6.fld2 = _3.0;
_6.fld0 = !(-26335_i16);
_8.0 = [_1,_3.0,_3.0,_1];
RET = 92765615329641275796963024946218760904_i128;
_7 = (-2022010506_i32) as f64;
_8.3 = (_3.3,);
Goto(bb2)
}
bb2 = {
_8.3 = (_3.3,);
_8.3.0 = [true,false,true];
_8.1 = [139_u8,229_u8,14_u8,225_u8];
_2 = [_3.0,_3.0,_3.0,_6.fld2];
_9.2.0 = 1904928934128523213_u64 as isize;
_9.2.2 = _3.0;
_9.2.2 = _1 - _1;
RET = 87875512694801892216431677673032700023_i128 - 69430305050696053463725990215588756162_i128;
_9.1.1 = _7 as f32;
_8.2 = [170_u8];
_6.fld0 = 6406_i16 ^ 21658_i16;
_3.0 = 64694727146439088_u64 as isize;
RET = (-37275944213512406901742036475902449506_i128);
_3.3 = [true,false,true];
_9.2.2 = _3.2 | _6.fld2;
_9.1.0 = 3_usize as isize;
_3.2 = _1 << _6.fld2;
_6.fld1 = core::ptr::addr_of_mut!(_12.3);
_9.2.3 = [false,true,true];
_4 = 73_i8 as isize;
_2 = [_1,_3.2,_9.2.2,_3.2];
_9.1.1 = -_3.1;
Goto(bb3)
}
bb3 = {
_6.fld1 = core::ptr::addr_of_mut!(_12.3);
_8.0 = [_3.2,_3.2,_6.fld2,_6.fld2];
_6.fld1 = core::ptr::addr_of_mut!(_12.3);
_3.1 = _9.1.1 - _9.1.1;
_7 = 1168483171_u32 as f64;
_9.1.2 = !_3.2;
_2 = _8.0;
_8.3 = (_9.2.3,);
_12.2 = [130_u8,75_u8,49_u8,121_u8];
_8.1 = _12.2;
_16 = _7;
_9.1 = (_9.2.2, _3.1, _1, _8.3.0);
_9.1.1 = -_3.1;
_3.1 = _9.1.1;
RET = 116612551870300707668017208410625817509_i128;
_1 = _6.fld0 as isize;
Goto(bb4)
}
bb4 = {
_1 = _9.1.0 << _9.1.2;
_12.1 = core::ptr::addr_of!(_8.3);
_3.2 = -_9.2.2;
_9.2.2 = _9.1.2;
_3.2 = _9.1.2 - _9.1.2;
RET = (-41237682171513872281517142620359383593_i128) * 100326801095760037959259281165731249669_i128;
Call(_9.2.0 = core::intrinsics::transmute(_1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = (-168955226043880330687240405347263013915_i128);
_12.0 = true;
_19 = ['\u{57753}'];
_12.2 = [88_u8,113_u8,183_u8,60_u8];
_8.2 = [45_u8];
_9.1.3 = [_12.0,_12.0,_12.0];
_8.0 = [_9.2.0,_9.2.0,_9.1.0,_1];
_20 = (-658892721_i32) * 1641481716_i32;
_9.1.2 = -_9.1.0;
_4 = _9.1.0 & _9.2.0;
_12.2 = [60_u8,69_u8,196_u8,28_u8];
_25.1 = 87240889918757913066993341611343907912_u128 >> _9.1.2;
_24 = (_19, _25.1);
_14 = Adt60::Variant0 { fld0: _19 };
_6.fld2 = _1;
_25.0 = ['\u{10de68}'];
_24 = (_19, _25.1);
SetDiscriminant(_14, 1);
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.2.2 = 60157_u16 as isize;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.0 = core::ptr::addr_of_mut!(_24.1);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
171327140877058132776134202084505197541 => bb7,
_ => bb6
}
}
bb6 = {
_2 = [_1,_3.2,_1,_3.2];
_1 = !_3.0;
_6.fld0 = -16696_i16;
_6.fld0 = (-15390_i16) >> _3.0;
_4 = 2071801364_u32 as isize;
_6.fld0 = (-30135_i16) + 18519_i16;
_5 = [2873673406_u32,137838498_u32,2437533088_u32,3235001804_u32,3868976373_u32,34472040_u32];
_3.3 = [true,true,false];
_3.1 = 188_u8 as f32;
_4 = 1453549601286488830_i64 as isize;
_5 = [1730605167_u32,1157882933_u32,2415687664_u32,2094379861_u32,3341052792_u32,3721224348_u32];
_2 = [_3.2,_3.0,_1,_3.0];
_6.fld2 = 139945259_u32 as isize;
_6.fld0 = 1912662007_i32 as i16;
RET = !(-87140398096408725295583783977507721753_i128);
_3.2 = (-124_i8) as isize;
_2 = [_1,_1,_1,_3.0];
_2 = [_3.0,_4,_1,_3.0];
RET = -12374163465518407507346868456979579149_i128;
_6.fld2 = _3.0;
_6.fld0 = !(-26335_i16);
_8.0 = [_1,_3.0,_3.0,_1];
RET = 92765615329641275796963024946218760904_i128;
_7 = (-2022010506_i32) as f64;
_8.3 = (_3.3,);
Goto(bb2)
}
bb7 = {
_17 = 44640_u16;
_6.fld2 = 13692963072073075798_u64 as isize;
_18 = _16;
_9.0 = Field::<Adt53>(Variant(_14, 1), 2).fld2.0;
_9.2.1 = _9.1.1 - _9.1.1;
_3.2 = _9.1.2;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.2.1 = _9.2.1;
_22 = !_20;
Call(_12.3 = core::intrinsics::transmute(_24.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld1 = '\u{cd2f5}';
_7 = _18;
_24.0 = _25.0;
_17 = 61847_u16 << _1;
_9.1.1 = 4016006710_u32 as f32;
_9.1.2 = 53_u8 as isize;
_21 = (RET,);
_13 = 3629088668_u32 as isize;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.2.3 = [_12.0,_12.0,_12.0];
_11 = -_18;
_26 = _9.1.0 & _4;
_6.fld2 = _1;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.1.3 = [_12.0,_12.0,_12.0];
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.1.2 = _1 + _9.2.0;
_3.3 = _9.2.3;
_18 = _16;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.2.2 = -Field::<Adt53>(Variant(_14, 1), 2).fld2.1.2;
_12.1 = core::ptr::addr_of!(_8.3);
_18 = _7 * _16;
_20 = _22 - _22;
place!(Field::<f64>(Variant(_14, 1), 1)) = _18 + _16;
Call(_27 = core::intrinsics::bswap(_12.3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2 = _9;
_6 = Adt51 { fld0: (-12541_i16),fld1: Field::<Adt53>(Variant(_14, 1), 2).fld2.0,fld2: _26 };
_9.1.0 = -_26;
_9.2.0 = -_4;
_23.fld0 = !5891652961458578621_u64;
match _6.fld0 {
0 => bb4,
1 => bb2,
2 => bb5,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607431768198915 => bb15,
_ => bb14
}
}
bb10 = {
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld1 = '\u{cd2f5}';
_7 = _18;
_24.0 = _25.0;
_17 = 61847_u16 << _1;
_9.1.1 = 4016006710_u32 as f32;
_9.1.2 = 53_u8 as isize;
_21 = (RET,);
_13 = 3629088668_u32 as isize;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.2.3 = [_12.0,_12.0,_12.0];
_11 = -_18;
_26 = _9.1.0 & _4;
_6.fld2 = _1;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.1.3 = [_12.0,_12.0,_12.0];
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.1.2 = _1 + _9.2.0;
_3.3 = _9.2.3;
_18 = _16;
place!(Field::<Adt53>(Variant(_14, 1), 2)).fld2.2.2 = -Field::<Adt53>(Variant(_14, 1), 2).fld2.1.2;
_12.1 = core::ptr::addr_of!(_8.3);
_18 = _7 * _16;
_20 = _22 - _22;
place!(Field::<f64>(Variant(_14, 1), 1)) = _18 + _16;
Call(_27 = core::intrinsics::bswap(_12.3), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_8.3 = (_3.3,);
_8.3.0 = [true,false,true];
_8.1 = [139_u8,229_u8,14_u8,225_u8];
_2 = [_3.0,_3.0,_3.0,_6.fld2];
_9.2.0 = 1904928934128523213_u64 as isize;
_9.2.2 = _3.0;
_9.2.2 = _1 - _1;
RET = 87875512694801892216431677673032700023_i128 - 69430305050696053463725990215588756162_i128;
_9.1.1 = _7 as f32;
_8.2 = [170_u8];
_6.fld0 = 6406_i16 ^ 21658_i16;
_3.0 = 64694727146439088_u64 as isize;
RET = (-37275944213512406901742036475902449506_i128);
_3.3 = [true,false,true];
_9.2.2 = _3.2 | _6.fld2;
_9.1.0 = 3_usize as isize;
_3.2 = _1 << _6.fld2;
_6.fld1 = core::ptr::addr_of_mut!(_12.3);
_9.2.3 = [false,true,true];
_4 = 73_i8 as isize;
_2 = [_1,_3.2,_9.2.2,_3.2];
_9.1.1 = -_3.1;
Goto(bb3)
}
bb12 = {
_2 = [_1,_3.2,_1,_3.2];
_1 = !_3.0;
_6.fld0 = -16696_i16;
_6.fld0 = (-15390_i16) >> _3.0;
_4 = 2071801364_u32 as isize;
_6.fld0 = (-30135_i16) + 18519_i16;
_5 = [2873673406_u32,137838498_u32,2437533088_u32,3235001804_u32,3868976373_u32,34472040_u32];
_3.3 = [true,true,false];
_3.1 = 188_u8 as f32;
_4 = 1453549601286488830_i64 as isize;
_5 = [1730605167_u32,1157882933_u32,2415687664_u32,2094379861_u32,3341052792_u32,3721224348_u32];
_2 = [_3.2,_3.0,_1,_3.0];
_6.fld2 = 139945259_u32 as isize;
_6.fld0 = 1912662007_i32 as i16;
RET = !(-87140398096408725295583783977507721753_i128);
_3.2 = (-124_i8) as isize;
_2 = [_1,_1,_1,_3.0];
_2 = [_3.0,_4,_1,_3.0];
RET = -12374163465518407507346868456979579149_i128;
_6.fld2 = _3.0;
_6.fld0 = !(-26335_i16);
_8.0 = [_1,_3.0,_3.0,_1];
RET = 92765615329641275796963024946218760904_i128;
_7 = (-2022010506_i32) as f64;
_8.3 = (_3.3,);
Goto(bb2)
}
bb13 = {
_2 = [_1,_3.2,_1,_3.2];
_1 = !_3.0;
_6.fld0 = -16696_i16;
_6.fld0 = (-15390_i16) >> _3.0;
_4 = 2071801364_u32 as isize;
_6.fld0 = (-30135_i16) + 18519_i16;
_5 = [2873673406_u32,137838498_u32,2437533088_u32,3235001804_u32,3868976373_u32,34472040_u32];
_3.3 = [true,true,false];
_3.1 = 188_u8 as f32;
_4 = 1453549601286488830_i64 as isize;
_5 = [1730605167_u32,1157882933_u32,2415687664_u32,2094379861_u32,3341052792_u32,3721224348_u32];
_2 = [_3.2,_3.0,_1,_3.0];
_6.fld2 = 139945259_u32 as isize;
_6.fld0 = 1912662007_i32 as i16;
RET = !(-87140398096408725295583783977507721753_i128);
_3.2 = (-124_i8) as isize;
_2 = [_1,_1,_1,_3.0];
_2 = [_3.0,_4,_1,_3.0];
RET = -12374163465518407507346868456979579149_i128;
_6.fld2 = _3.0;
_6.fld0 = !(-26335_i16);
_8.0 = [_1,_3.0,_3.0,_1];
RET = 92765615329641275796963024946218760904_i128;
_7 = (-2022010506_i32) as f64;
_8.3 = (_3.3,);
Goto(bb2)
}
bb14 = {
_1 = _9.1.0 << _9.1.2;
_12.1 = core::ptr::addr_of!(_8.3);
_3.2 = -_9.2.2;
_9.2.2 = _9.1.2;
_3.2 = _9.1.2 - _9.1.2;
RET = (-41237682171513872281517142620359383593_i128) * 100326801095760037959259281165731249669_i128;
Call(_9.2.0 = core::intrinsics::transmute(_1), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_35 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_14, 1), 1)));
_24.0 = _25.0;
_9.2.2 = !_6.fld2;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(14_usize, 5_usize, Move(_5), 22_usize, Move(_22), 13_usize, Move(_13), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(14_usize, 21_usize, Move(_21), 25_usize, Move(_25), 26_usize, Move(_26), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: (isize, f32, isize, [bool; 3]),mut _3: isize,mut _4: isize,mut _5: f32,mut _6: isize,mut _7: (isize, f32, isize, [bool; 3]),mut _8: isize,mut _9: (isize, f32, isize, [bool; 3]),mut _10: (isize, f32, isize, [bool; 3]),mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> isize {
mir! {
type RET = isize;
let _16: *mut u128;
let _17: bool;
let _18: ();
let _19: ();
{
_9.0 = !_1;
_9 = (_6, _7.1, _6, _7.3);
_10.1 = _5 + _9.1;
_13 = -_6;
RET = !_10.0;
_7 = (_9.0, _10.1, _9.0, _9.3);
_9.1 = _5;
_10 = _9;
RET = _10.0 * _8;
_6 = -_15;
_9 = (_13, _2.1, _10.2, _7.3);
_2.0 = !_10.2;
_15 = 1941720071_i32 as isize;
_10.1 = _9.1;
_6 = 1809773015_i32 as isize;
_13 = 1344893115010613976_u64 as isize;
_7.3 = [true,false,true];
_4 = _1 | _2.2;
_7.3 = [true,true,true];
_9.3 = [false,true,false];
_1 = _2.0 ^ _11;
_9.0 = RET | _4;
RET = !_7.2;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(15_usize, 11_usize, Move(_11), 14_usize, Move(_14), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(15_usize, 1_usize, Move(_1), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u8,mut _2: Adt51,mut _3: [bool; 3],mut _4: (isize, f32, isize, [bool; 3]),mut _5: i8,mut _6: f32,mut _7: i8) -> isize {
mir! {
type RET = isize;
let _8: Adt52;
let _9: Adt45;
let _10: u64;
let _11: ();
let _12: ();
{
_4.3 = [true,false,true];
_1 = (-5889191730662373745_i64) as u8;
_4 = (_2.fld2, _6, _2.fld2, _3);
match _2.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
28216 => bb8,
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
_2.fld0 = 15706_i16 * (-23718_i16);
RET = _2.fld2 & _2.fld2;
RET = _2.fld2;
_4.0 = 6774951686108595780_i64 as isize;
_3 = [false,true,true];
_7 = _5;
_3 = [false,false,false];
_7 = _5;
_4.3 = [false,false,false];
RET = _4.2;
_2.fld2 = _4.2 << _4.0;
_6 = _4.1;
RET = -_4.2;
Goto(bb9)
}
bb9 = {
Call(_11 = dump_var(16_usize, 3_usize, Move(_3), 1_usize, Move(_1), 12_usize, _12, 12_usize, _12), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(23771_u16), std::hint::black_box(4_usize), std::hint::black_box(78_i8), std::hint::black_box((-58516410011499530930118163861120929347_i128)), std::hint::black_box(16418671511491985622_u64), std::hint::black_box(2553873607875856965_i64));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: ([bool; 3],),
fld1: [u8; 1],
fld2: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],)),
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (f32, i16, u64, i32, (i128,)),
fld1: u32,
fld2: *const ([bool; 3],),
fld3: [u32; 6],
fld4: u64,

},
Variant1{
fld0: bool,
fld1: *const isize,
fld2: *mut char,
fld3: Adt44,
fld4: ([char; 1], u128),
fld5: [u8; 1],
fld6: [char; 1],

},
Variant2{
fld0: bool,
fld1: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3])),
fld2: ((isize, f32, isize, [bool; 3]), [bool; 3]),
fld3: f32,
fld4: u16,
fld5: u8,
fld6: u64,

},
Variant3{
fld0: u8,
fld1: *const isize,
fld2: (isize, f32, isize, [bool; 3]),
fld3: f32,
fld4: [u32; 6],
fld5: (i128,),
fld6: u128,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: usize,
fld1: [u8; 1],
fld2: *mut char,
fld3: [u64; 8],
fld4: ([bool; 3],),
fld5: i32,
fld6: [isize; 4],
fld7: f32,

},
Variant1{
fld0: i128,
fld1: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3])),
fld2: [bool; 3],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: [u8; 1],
fld1: *mut u128,
fld2: u32,
fld3: u8,
fld4: u64,
fld5: (char, i8),
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: f32,
fld1: ([char; 1], u128),
fld2: [u32; 6],
fld3: i8,
fld4: (f32, i16, u64, i32, (i128,)),
fld5: Adt44,
fld6: f64,
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
fld0: bool,
fld1: *mut [i32; 3],

},
Variant1{
fld0: u128,
fld1: [u8; 4],
fld2: Adt44,
fld3: *mut [i32; 3],
fld4: (i16,),
fld5: i64,

},
Variant2{
fld0: Adt46,
fld1: *mut f64,
fld2: ([char; 1], u128),
fld3: Adt48,
fld4: i16,
fld5: [isize; 4],
fld6: *mut [i32; 3],
fld7: [u64; 8],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f64,
fld1: Adt46,
fld2: i128,
fld3: (bool, *const ([bool; 3],), [u8; 4], u128),
fld4: (f32, i16, u64, i32, (i128,)),
fld5: *mut f64,

},
Variant1{
fld0: u64,
fld1: (i128,),
fld2: [u8; 4],
fld3: f64,
fld4: (char, i8),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: i16,
fld1: *mut u128,
fld2: isize,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: bool,
fld1: usize,
fld2: i128,
fld3: i64,
fld4: ((isize, f32, isize, [bool; 3]), [bool; 3]),

},
Variant1{
fld0: Adt48,
fld1: Adt45,
fld2: [isize; 4],
fld3: Adt47,
fld4: (i128,),
fld5: i128,

},
Variant2{
fld0: [u32; 6],
fld1: (i16,),
fld2: [char; 1],
fld3: Adt48,
fld4: u8,
fld5: i32,
fld6: [u8; 4],
fld7: Adt47,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: [u32; 6],
fld1: char,
fld2: (*mut u128, (isize, f32, isize, [bool; 3]), (isize, f32, isize, [bool; 3])),
fld3: i8,
fld4: Adt49,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt46,
fld1: ([bool; 3],),

},
Variant1{
fld0: *mut [i32; 3],

},
Variant2{
fld0: f64,
fld1: u32,
fld2: *mut f64,
fld3: u16,
fld4: Adt53,
fld5: [u64; 8],
fld6: Adt50,
fld7: i128,

},
Variant3{
fld0: [u64; 8],
fld1: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],)),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: (isize, f32, isize, [bool; 3]),
fld1: ([char; 1], u128),
fld2: i32,
fld3: i8,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (bool, *const ([bool; 3],), [u8; 4], u128),
fld1: (i16,),
fld2: Adt55,
fld3: *const ([bool; 3],),
fld4: u128,
fld5: usize,

},
Variant1{
fld0: (char, i8),
fld1: *mut char,
fld2: i32,
fld3: *mut u128,
fld4: Adt45,

},
Variant2{
fld0: bool,
fld1: i128,
fld2: Adt54,
fld3: (isize, f32, isize, [bool; 3]),
fld4: [u32; 6],
fld5: ((isize, f32, isize, [bool; 3]), [bool; 3]),

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt54,
fld1: f64,
fld2: (char, i8),
fld3: f32,

},
Variant1{
fld0: ([char; 1], u128),
fld1: u16,
fld2: Adt47,
fld3: [u64; 8],
fld4: i16,
fld5: [isize; 4],
fld6: Adt53,
fld7: f64,

},
Variant2{
fld0: Adt47,
fld1: (isize, f32, isize, [bool; 3]),
fld2: ([isize; 4], [u8; 4], [u8; 1], ([bool; 3],)),
fld3: *mut f64,
fld4: *const isize,
fld5: u128,

},
Variant3{
fld0: Adt49,
fld1: f64,
fld2: u16,
fld3: [bool; 3],
fld4: [i32; 3],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: u64,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: i32,
fld1: i128,
fld2: ([bool; 3],),
fld3: Adt45,

},
Variant1{
fld0: bool,
fld1: [u64; 8],
fld2: [char; 1],
fld3: Adt46,
fld4: [isize; 4],
fld5: Adt50,
fld6: [u8; 1],

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: [char; 1],

},
Variant1{
fld0: [bool; 3],
fld1: f64,
fld2: Adt53,

}}

