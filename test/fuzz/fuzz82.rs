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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u32,mut _9: usize,mut _10: u8,mut _11: u128) -> f64 {
mir! {
type RET = f64;
let _12: *const (i128, i64, i8, i8);
let _13: *mut Adt23;
let _14: Adt62;
let _15: [char; 3];
let _16: f64;
let _17: [u8; 4];
let _18: isize;
let _19: &'static [isize; 7];
let _20: u32;
let _21: i64;
let _22: usize;
let _23: [u32; 4];
let _24: i64;
let _25: Adt23;
let _26: *mut &'static (i128, i64, i8, i8);
let _27: bool;
let _28: i64;
let _29: (isize, f32, u8, u16);
let _30: &'static *const i16;
let _31: char;
let _32: ();
let _33: ();
{
_4 = !(-55_i8);
RET = _4 as f64;
RET = 1455019907949170075_i64 as f64;
_8 = !3343071577_u32;
_4 = (-121_i8) ^ (-10_i8);
_10 = 110_u8;
_10 = 152_u8;
_9 = 16305028130265723004_usize ^ 6_usize;
_2 = '\u{d6429}';
_3 = (-9223372036854775808_isize);
_11 = 179009680320397760516993003350890663064_u128 | 216555165229687781817188310691274859147_u128;
_7 = 6412263348425892689_i64;
_5 = !20403_i16;
_3 = (-9223372036854775808_isize) << _9;
_7 = (-1743776582814803473_i64) | 2022313842184002401_i64;
_1 = 14483858442090802547_u64;
_10 = 23_u8 | 203_u8;
_1 = 6662309513410876708_u64;
_1 = 12726690292788414217_u64;
Goto(bb1)
}
bb1 = {
_9 = _2 as usize;
RET = _5 as f64;
_4 = (-98241004517320179991951834207979377590_i128) as i8;
match _1 {
0 => bb2,
1 => bb3,
12726690292788414217 => bb5,
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
RET = 40384_u16 as f64;
_6 = -(-1186536729_i32);
_10 = !204_u8;
_1 = RET as u64;
_15 = [_2,_2,_2];
_16 = RET * RET;
_9 = !4_usize;
_4 = _6 as i8;
Call(_11 = core::intrinsics::bswap(63114214675051711964601156570483448633_u128), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = RET - RET;
_4 = _5 as i8;
Goto(bb7)
}
bb7 = {
_17 = [_10,_10,_10,_10];
Call(_6 = fn1(_10, _3, _3, _11, _3, _8, _7, _10, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = '\u{9413e}';
_15 = [_2,_2,_2];
_16 = RET + RET;
_9 = 4904954232256300411_usize * 9742182705037306358_usize;
_8 = !61582074_u32;
_17 = [_10,_10,_10,_10];
_6 = (-1999722529_i32);
_7 = _9 as i64;
RET = _16 - _16;
_11 = !77720894087565719074343465676771250587_u128;
_11 = !251616032999666690411129909794824266704_u128;
_15 = [_2,_2,_2];
_11 = 52353636972587694127953739140214720326_u128;
_10 = 91_u8;
_6 = (-773242912_i32) & 612633419_i32;
_11 = _1 as u128;
_9 = !1198841322039187302_usize;
_3 = (-9223372036854775808_isize);
_7 = -2712935955693811620_i64;
_7 = (-917141663995485672_i64) ^ (-4039605880174124268_i64);
_18 = -_3;
_16 = -RET;
_5 = (-17123_i16);
_15 = [_2,_2,_2];
RET = -_16;
match _5 {
0 => bb7,
1 => bb2,
340282366920938463463374607431768194333 => bb9,
_ => bb5
}
}
bb9 = {
_20 = 49433_u16 as u32;
_8 = _20;
_17 = [_10,_10,_10,_10];
Goto(bb10)
}
bb10 = {
_4 = -24_i8;
Goto(bb11)
}
bb11 = {
_17 = [_10,_10,_10,_10];
RET = _16;
_8 = _4 as u32;
_8 = _7 as u32;
_2 = '\u{aa965}';
_16 = RET - RET;
_11 = 34724284540135322031130270582630667184_u128 | 267657180156310265709489017782651096021_u128;
Goto(bb12)
}
bb12 = {
_15 = [_2,_2,_2];
_7 = 5219122642620619090_i64;
_24 = _7 ^ _7;
_10 = _9 as u8;
_10 = 255_u8 & 214_u8;
_16 = -RET;
_23 = [_8,_8,_8,_8];
_25.fld0 = [_10];
_6 = _24 as i32;
_22 = _9 * _9;
_22 = _9;
_7 = -_24;
_17 = [_10,_10,_10,_10];
_13 = core::ptr::addr_of_mut!(_25);
_9 = !_22;
_1 = 368132818205194914_u64;
Goto(bb13)
}
bb13 = {
_17 = [_10,_10,_10,_10];
_6 = _9 as i32;
Call(_23 = fn19((*_13).fld0, (*_13).fld0, _9, (*_13).fld0, Move(_13), _3, _17), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17 = [_10,_10,_10,_10];
_15 = [_2,_2,_2];
_21 = !_24;
_29.0 = _18;
_7 = _24 * _24;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(0_usize, 9_usize, Move(_9), 22_usize, Move(_22), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(0_usize, 15_usize, Move(_15), 5_usize, Move(_5), 21_usize, Move(_21), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(0_usize, 7_usize, Move(_7), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u8,mut _2: isize,mut _3: isize,mut _4: u128,mut _5: isize,mut _6: u32,mut _7: i64,mut _8: u8,mut _9: f64) -> i32 {
mir! {
type RET = i32;
let _10: ([i32; 6], usize, usize);
let _11: u16;
let _12: bool;
let _13: f64;
let _14: *const (i128, i64, i8, i8);
let _15: [bool; 7];
let _16: u8;
let _17: &'static [u8; 4];
let _18: Adt42;
let _19: u64;
let _20: (&'static u128, [bool; 7]);
let _21: i8;
let _22: [bool; 6];
let _23: isize;
let _24: f32;
let _25: &'static *const i16;
let _26: isize;
let _27: [u32; 4];
let _28: [bool; 6];
let _29: u32;
let _30: f64;
let _31: [usize; 2];
let _32: u8;
let _33: ([u8; 4], (Adt42, Adt54, *mut Adt23, char), char);
let _34: ([i32; 6], usize, usize);
let _35: isize;
let _36: char;
let _37: bool;
let _38: ((isize, f32, u8, u16), u128, usize, [u8; 1]);
let _39: (&'static u128, [bool; 7]);
let _40: *const ((char, u32, u32),);
let _41: Adt73;
let _42: Adt47;
let _43: Adt78;
let _44: *mut i16;
let _45: (u32, u8, i128);
let _46: *mut &'static i8;
let _47: (&'static u128, *const i16);
let _48: ();
let _49: ();
{
RET = 64_i8 as i32;
Call(_2 = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = '\u{b45b0}' as i32;
_6 = 2666479992_u32;
_9 = _7 as f64;
_1 = _8;
_5 = _3 | _2;
_2 = _3 << _3;
_10.2 = '\u{5ac47}' as usize;
_4 = 38173500072347220213878680011499938263_u128;
_10.1 = _10.2;
_1 = _8;
_6 = 3132994309_u32 ^ 1244155174_u32;
_10.0 = [RET,RET,RET,RET,RET,RET];
_10.0 = [RET,RET,RET,RET,RET,RET];
RET = -(-1989276623_i32);
Call(RET = fn2(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _3 >> _3;
_10.0 = [RET,RET,RET,RET,RET,RET];
_5 = -_2;
_10.2 = !_10.1;
_10.0 = [RET,RET,RET,RET,RET,RET];
_9 = _10.2 as f64;
_10.2 = !_10.1;
_7 = (-432038229627481572_i64);
_11 = 23251_u16 + 48647_u16;
_11 = _8 as u16;
_1 = _6 as u8;
_2 = _11 as isize;
_7 = !5230956068406173703_i64;
_7 = (-624249995548410488_i64) >> _3;
_5 = -_3;
_12 = !true;
_4 = 203915553092105415333775944933838447338_u128 ^ 106034743469728005794291908603707314597_u128;
match RET {
340282366920938463463374607430759808475 => bb4,
_ => bb3
}
}
bb3 = {
RET = '\u{b45b0}' as i32;
_6 = 2666479992_u32;
_9 = _7 as f64;
_1 = _8;
_5 = _3 | _2;
_2 = _3 << _3;
_10.2 = '\u{5ac47}' as usize;
_4 = 38173500072347220213878680011499938263_u128;
_10.1 = _10.2;
_1 = _8;
_6 = 3132994309_u32 ^ 1244155174_u32;
_10.0 = [RET,RET,RET,RET,RET,RET];
_10.0 = [RET,RET,RET,RET,RET,RET];
RET = -(-1989276623_i32);
Call(RET = fn2(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_9 = (-23402126817201003067232053047896774164_i128) as f64;
_6 = _11 as u32;
_13 = RET as f64;
_8 = !_1;
_10.2 = _10.1;
_15 = [_12,_12,_12,_12,_12,_12,_12];
_8 = !_1;
_2 = _11 as isize;
_9 = _13;
_8 = _1;
_9 = -_13;
_6 = 796791911_u32 + 3666552146_u32;
_1 = _8;
RET = -(-18781998_i32);
_12 = true;
_10.1 = !_10.2;
_4 = _6 as u128;
_15 = [_12,_12,_12,_12,_12,_12,_12];
_10.0 = [RET,RET,RET,RET,RET,RET];
Call(_3 = fn3(_10.0, _1, _1, RET, _1, _5, RET, _10.2, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = 302424707_i32 << _7;
_11 = !5175_u16;
_13 = _1 as f64;
_8 = !_1;
_16 = !_8;
_3 = !_2;
_6 = 3388024914_u32;
RET = 1085343426_i32;
_1 = _8 * _16;
_5 = _7 as isize;
_5 = _10.2 as isize;
_6 = 3325375228_u32 * 805151666_u32;
_12 = !true;
_8 = _1;
_5 = _2;
RET = !(-1088095033_i32);
Goto(bb6)
}
bb6 = {
_4 = !335431282997460776425110487908593993506_u128;
_3 = _5 ^ _2;
_4 = !114980371665457254744648933120897675925_u128;
_7 = _4 as i64;
_5 = _3 & _3;
RET = -(-1923945828_i32);
_15 = [_12,_12,_12,_12,_12,_12,_12];
_6 = 4240488327_u32;
_9 = 10422574917114152356_u64 as f64;
_2 = _5;
_5 = '\u{12c65}' as isize;
_10.0 = [RET,RET,RET,RET,RET,RET];
_11 = RET as u16;
_10.2 = _10.1;
match _6 {
0 => bb4,
1 => bb5,
4240488327 => bb7,
_ => bb3
}
}
bb7 = {
_19 = !2612917501455230372_u64;
_12 = true;
_19 = 3098237138123755580_u64;
RET = 1769137019_i32 - 1285312710_i32;
_10.0 = [RET,RET,RET,RET,RET,RET];
_11 = 34066_u16;
_3 = _2 - _2;
RET = _19 as i32;
_24 = _7 as f32;
_19 = !119694309330191774_u64;
_10.1 = 10495_i16 as usize;
_22 = [_12,_12,_12,_12,_12,_12];
_15 = [_12,_12,_12,_12,_12,_12,_12];
_24 = _3 as f32;
match _6 {
4240488327 => bb9,
_ => bb8
}
}
bb8 = {
_4 = !335431282997460776425110487908593993506_u128;
_3 = _5 ^ _2;
_4 = !114980371665457254744648933120897675925_u128;
_7 = _4 as i64;
_5 = _3 & _3;
RET = -(-1923945828_i32);
_15 = [_12,_12,_12,_12,_12,_12,_12];
_6 = 4240488327_u32;
_9 = 10422574917114152356_u64 as f64;
_2 = _5;
_5 = '\u{12c65}' as isize;
_10.0 = [RET,RET,RET,RET,RET,RET];
_11 = RET as u16;
_10.2 = _10.1;
match _6 {
0 => bb4,
1 => bb5,
4240488327 => bb7,
_ => bb3
}
}
bb9 = {
_11 = !35845_u16;
_2 = _3;
_3 = !_2;
_21 = -66_i8;
_27 = [_6,_6,_6,_6];
_22 = [_12,_12,_12,_12,_12,_12];
_13 = _9 + _9;
_21 = 97_i8;
_12 = true;
_3 = -_2;
_23 = 16815_i16 as isize;
_9 = _13 - _13;
_28 = [_12,_12,_12,_12,_12,_12];
_18 = Adt42::Variant0 { fld0: _27 };
place!(Field::<[u32; 4]>(Variant(_18, 0), 0)) = [_6,_6,_6,_6];
_16 = _8 + _8;
_20.1 = [_12,_12,_12,_12,_12,_12,_12];
_4 = 197716744860358846219347880715692176561_u128;
_10.1 = _10.2;
_9 = -_13;
_19 = 18049330861563051485_u64 | 7440986912860591267_u64;
_24 = _11 as f32;
Call(_23 = core::intrinsics::transmute(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_20.1 = _15;
_7 = !7906666584097744962_i64;
_15 = [_12,_12,_12,_12,_12,_12,_12];
_20.0 = &_4;
_6 = 3026624018_u32;
_5 = _2 + _3;
_8 = _5 as u8;
place!(Field::<[u32; 4]>(Variant(_18, 0), 0)) = [_6,_6,_6,_6];
_5 = _4 as isize;
_32 = _16 - _8;
_26 = -_3;
_24 = _32 as f32;
place!(Field::<[u32; 4]>(Variant(_18, 0), 0)) = [_6,_6,_6,_6];
_33.2 = '\u{a53aa}';
RET = 1836302670_i32;
_11 = 26773_u16 & 59772_u16;
_30 = _9 - _9;
_34.0 = _10.0;
_34.1 = _24 as usize;
_34 = (_10.0, _10.2, _10.2);
_21 = !(-117_i8);
_16 = _8;
RET = 918269826_i32;
_34.1 = _21 as usize;
Goto(bb11)
}
bb11 = {
SetDiscriminant(_18, 0);
_15 = [_12,_12,_12,_12,_12,_12,_12];
_10.1 = !_34.1;
_10.0 = [RET,RET,RET,RET,RET,RET];
_34.2 = !_10.2;
_10.2 = _33.2 as usize;
_8 = _32 << _23;
_31 = [_34.2,_10.1];
RET = 1805690787_i32 & 1142618971_i32;
_20.1 = [_12,_12,_12,_12,_12,_12,_12];
_12 = _16 >= _8;
_4 = 148613883041446059335069835618500814749_u128 << _16;
RET = -(-1857674273_i32);
_33.1.1 = Adt54::Variant2 { fld0: _7 };
_17 = &_33.0;
_34 = _10;
_8 = _32 | _32;
_27 = [_6,_6,_6,_6];
_33.1.0 = Adt42::Variant0 { fld0: _27 };
_16 = _8 ^ _1;
_34.0 = [RET,RET,RET,RET,RET,RET];
_10.2 = _34.1 ^ _34.2;
_33.0 = [_32,_8,_8,_16];
_18 = Move(_33.1.0);
Goto(bb12)
}
bb12 = {
place!(Field::<i64>(Variant(_33.1.1, 2), 0)) = _7 * _7;
_21 = 28_i8 * (-121_i8);
_10.1 = !_34.2;
_33.1.3 = _33.2;
_24 = _16 as f32;
_22 = [_12,_12,_12,_12,_12,_12];
_35 = _19 as isize;
_16 = _32;
_33.1.0 = Move(_18);
_10.1 = _10.2 ^ _10.2;
_9 = _13;
_1 = _8 | _32;
_28 = [_12,_12,_12,_12,_12,_12];
_5 = _3 | _2;
_26 = _21 as isize;
_35 = _5;
_13 = _30;
_33.1.3 = _33.2;
_10.1 = _10.2;
_7 = Field::<i64>(Variant(_33.1.1, 2), 0) * Field::<i64>(Variant(_33.1.1, 2), 0);
_36 = _33.2;
_34.0 = [RET,RET,RET,RET,RET,RET];
_1 = _8;
_38.3 = [_8];
_38.0.2 = _1 >> _1;
match _6 {
0 => bb5,
1 => bb2,
3026624018 => bb13,
_ => bb11
}
}
bb13 = {
_37 = _12;
_24 = _10.2 as f32;
_12 = _37;
_34.2 = _10.2 << _32;
RET = 52865119915839518930439879272964563927_i128 as i32;
SetDiscriminant(_33.1.1, 2);
_2 = _7 as isize;
_37 = !_12;
_7 = !5095063254799181447_i64;
_18 = Adt42::Variant2 { fld0: _7,fld1: (-13897_i16),fld2: _38.3 };
place!(Field::<[u8; 1]>(Variant(_18, 2), 2)) = [_32];
place!(Field::<i16>(Variant(_18, 2), 1)) = _33.1.3 as i16;
_22 = [_37,_37,_37,_12,_37,_37];
_10.2 = !_34.2;
_38.0.1 = RET as f32;
_20.1 = [_37,_37,_12,_37,_37,_37,_12];
_36 = _33.2;
_42 = Adt47::Variant1 { fld0: Field::<[u8; 1]>(Variant(_18, 2), 2),fld1: Move(_18) };
_8 = !_16;
_38.0.1 = -_24;
_28 = [_37,_12,_37,_12,_12,_12];
_17 = &_33.0;
_34.1 = _19 as usize;
place!(Field::<[u8; 1]>(Variant(_42, 1), 0)) = [_38.0.2];
Call(_16 = core::intrinsics::transmute(_12), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_29 = !_6;
_38.0.2 = !_16;
place!(Field::<i64>(Variant(_33.1.1, 2), 0)) = _19 as i64;
_39.1 = _15;
_38.2 = _34.2;
_33.0 = [_38.0.2,_16,_16,_16];
_20.0 = &_4;
_34.2 = _38.2 << _16;
_8 = _4 as u8;
_34.1 = _34.2;
_38.2 = _10.2 + _10.2;
_18 = Move(Field::<Adt42>(Variant(_42, 1), 1));
_10.1 = _34.2;
_31 = [_10.1,_10.1];
_19 = !78129053652366761_u64;
_6 = _10.1 as u32;
_30 = -_9;
_36 = _33.1.3;
_38.0.0 = _7 as isize;
_20.0 = &_38.1;
_45.0 = _6 - _6;
_45.0 = _6 ^ _6;
_44 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_18, 2), 1)));
_25 = &_47.1;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(1_usize, 12_usize, Move(_12), 35_usize, Move(_35), 23_usize, Move(_23), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(1_usize, 7_usize, Move(_7), 8_usize, Move(_8), 3_usize, Move(_3), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(1_usize, 10_usize, Move(_10), 21_usize, Move(_21), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(1_usize, 16_usize, Move(_16), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i64) -> i32 {
mir! {
type RET = i32;
let _2: (f64, u16, i8);
let _3: (*const u8, [bool; 8], (isize, f32, u8, u16), f32);
let _4: (Adt42, Adt54, *mut Adt23, char);
let _5: ((char, u32, u32),);
let _6: u32;
let _7: bool;
let _8: [u8; 1];
let _9: f32;
let _10: *const [bool; 7];
let _11: ((isize, f32, u8, u16), u128, usize, [u8; 1]);
let _12: char;
let _13: &'static &'static *const i16;
let _14: f64;
let _15: [u8; 1];
let _16: usize;
let _17: i128;
let _18: u128;
let _19: *const (i128, i64, i8, i8);
let _20: *const u128;
let _21: *mut &'static (i128, i64, i8, i8);
let _22: [u8; 4];
let _23: u64;
let _24: Adt47;
let _25: f64;
let _26: *const [i32; 6];
let _27: u64;
let _28: ();
let _29: ();
{
RET = 120496407_i32;
RET = 3047751114_u32 as i32;
_1 = (-6739527161814139536_i64) << RET;
RET = 401972325_i32 | 21916461_i32;
RET = 1164742761_i32;
RET = 1554009308_u32 as i32;
RET = (-1999905776_i32) << _1;
_2.1 = 57247_u16 & 58591_u16;
_1 = (-9209305482037112891_i64) >> RET;
_3.2.1 = 237_u8 as f32;
_2.2 = 66_i8;
_5.0 = ('\u{4de24}', 1323187704_u32, 1246098454_u32);
_6 = _3.2.1 as u32;
_3.3 = _3.2.1;
RET = (-1148299300_i32);
_1 = (-7924558737247364605_i64) - (-4833508978619341681_i64);
_4.3 = _5.0.0;
_3.1 = [false,true,false,true,true,false,true,false];
_3.1 = [false,false,true,false,true,false,false,false];
_7 = true;
_9 = _3.2.1 + _3.3;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607430619912156 => bb7,
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
_7 = false;
_11.0.2 = 101_u8;
_11.0.0 = 9223372036854775807_isize & 9223372036854775807_isize;
_11.0.3 = !_2.1;
_3.3 = _11.0.0 as f32;
_3.0 = core::ptr::addr_of!(_3.2.2);
_6 = !_5.0.2;
_3.2.1 = -_3.3;
_1 = 474225840947075497_i64;
_11.0.1 = 7531075640941766146_u64 as f32;
_8 = [_11.0.2];
Goto(bb8)
}
bb8 = {
_4.3 = _5.0.0;
_5.0 = (_4.3, _6, _6);
_5.0.0 = _4.3;
_3.2.0 = _11.0.0 - _11.0.0;
match _1 {
0 => bb7,
1 => bb9,
474225840947075497 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_9 = 302427881436295098543988576488087435505_u128 as f32;
_5.0.1 = _6 << _5.0.2;
_6 = !_5.0.1;
RET = !415346333_i32;
_12 = _5.0.0;
_3.2.0 = RET as isize;
_4.3 = _5.0.0;
_15 = [_11.0.2];
_9 = -_3.3;
_11.0.2 = 4_u8;
_3.2 = _11.0;
_5.0.2 = _6 | _6;
_3.0 = core::ptr::addr_of!(_11.0.2);
_16 = !2_usize;
_11 = (_3.2, 143189854578935258778358821737651724713_u128, _16, _15);
RET = (-1008402981_i32);
_3.1 = [_7,_7,_7,_7,_7,_7,_7,_7];
_2.1 = !_3.2.3;
Call(_16 = core::intrinsics::transmute(_11.0.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = (-7997557154205623145_i64) << _5.0.2;
_11.0 = (_3.2.0, _9, _3.2.2, _3.2.3);
_11.0.0 = !_3.2.0;
_3.2.3 = _2.1 & _11.0.3;
_16 = _11.2;
_20 = core::ptr::addr_of!(_18);
_11.3 = [_11.0.2];
_5.0.1 = _5.0.2 >> _11.1;
_1 = RET as i64;
_17 = _11.1 as i128;
_4.1 = Adt54::Variant0 { fld0: RET };
_5.0 = (_12, _6, _6);
_4.1 = Adt54::Variant2 { fld0: _1 };
_3.2.2 = _11.1 as u8;
(*_20) = _9 as u128;
_15 = _11.3;
_3.1 = [_7,_7,_7,_7,_7,_7,_7,_7];
_3.2.0 = _11.0.0 + _11.0.0;
_9 = _3.2.1 * _3.3;
_11.2 = _16 >> _3.2.3;
_11.0.2 = _11.2 as u8;
_9 = _3.3;
match RET {
340282366920938463463374607430759808475 => bb13,
_ => bb9
}
}
bb13 = {
_2.0 = _2.1 as f64;
match _11.1 {
0 => bb7,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
143189854578935258778358821737651724713 => bb15,
_ => bb14
}
}
bb14 = {
_4.3 = _5.0.0;
_5.0 = (_4.3, _6, _6);
_5.0.0 = _4.3;
_3.2.0 = _11.0.0 - _11.0.0;
match _1 {
0 => bb7,
1 => bb9,
474225840947075497 => bb11,
_ => bb10
}
}
bb15 = {
_5.0.1 = _5.0.2;
_4.0 = Adt42::Variant1 { fld0: Move(_3.0) };
_17 = (-21797302719810235341512608834020651883_i128) | 87191764766048871174372241851319735535_i128;
_11.1 = 10458782258737085879_u64 as u128;
_8 = [_3.2.2];
_5.0.0 = _12;
_2.2 = _3.2.3 as i8;
_2.0 = (*_20) as f64;
_7 = _5.0.1 >= _5.0.1;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(2_usize, 6_usize, Move(_6), 15_usize, Move(_15), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(2_usize, 5_usize, Move(_5), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i32; 6],mut _2: u8,mut _3: u8,mut _4: i32,mut _5: u8,mut _6: isize,mut _7: i32,mut _8: usize,mut _9: u8) -> isize {
mir! {
type RET = isize;
let _10: (f32,);
let _11: u32;
let _12: bool;
let _13: [i32; 6];
let _14: Adt23;
let _15: (f64, u16, i8);
let _16: [bool; 7];
let _17: char;
let _18: [u8; 4];
let _19: isize;
let _20: bool;
let _21: &'static &'static *const i16;
let _22: f32;
let _23: ();
let _24: ();
{
RET = _6;
RET = -_6;
RET = _6;
_2 = !_9;
_5 = _2 >> _9;
Goto(bb1)
}
bb1 = {
_8 = 12552978040801057059_usize;
RET = _6 + _6;
_5 = !_2;
RET = _6;
_8 = !8477306079325854382_usize;
_9 = _8 as u8;
Call(_8 = core::intrinsics::bswap(2346833830835861860_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = !_5;
_7 = _4 << _5;
_2 = _5 << _7;
_9 = !_2;
_10.0 = _5 as f32;
_11 = 3471028180_u32;
_11 = RET as u32;
_6 = RET + RET;
_11 = 502507437_u32;
_8 = _11 as usize;
RET = _6;
RET = _8 as isize;
_12 = false;
_2 = _3 & _9;
_8 = 2_usize;
_3 = !_9;
RET = _6;
Call(_3 = fn4(_6, _11, _11, RET, RET, _6, RET, _7, _6, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15.1 = !34899_u16;
_15.2 = (-106_i8) + 118_i8;
_12 = _9 == _5;
_13 = _1;
_3 = _2;
_4 = _7;
_3 = 9117275288473971882_i64 as u8;
_8 = 2_usize - 16531854535956132333_usize;
_7 = !_4;
Call(_5 = core::intrinsics::transmute(_9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8 = (-9628_i16) as usize;
_6 = !RET;
Goto(bb5)
}
bb5 = {
_15.1 = !41479_u16;
_15.0 = _15.2 as f64;
_13 = _1;
_1 = [_4,_7,_4,_7,_7,_7];
_9 = 8137871260473157992_i64 as u8;
_5 = !_2;
_14.fld0 = [_2];
_9 = _2;
_15.1 = '\u{7727}' as u16;
_7 = !_4;
_9 = _15.2 as u8;
_5 = _2 * _2;
_13 = [_4,_4,_4,_7,_4,_7];
_17 = '\u{9cd45}';
_19 = _6;
_18 = [_2,_5,_5,_9];
_15.2 = _4 as i8;
_7 = (-4208_i16) as i32;
_4 = (-3951_i16) as i32;
_3 = _5;
_7 = _4;
_14.fld0 = [_3];
_13 = _1;
Call(_14 = fn18(_2, RET, _12, _17, RET, _19, _5, _15.2, _18, _3, _6, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = [_12,_12,_12,_12,_12,_12,_12];
_1 = [_4,_4,_4,_7,_4,_7];
_5 = _2;
_7 = -_4;
_12 = false;
_1 = [_7,_7,_4,_7,_7,_7];
_7 = _4 + _4;
_18 = [_2,_3,_3,_3];
_8 = 3842915264527969185_usize * 2262527934646287329_usize;
_5 = _15.0 as u8;
_8 = !0_usize;
_18 = [_2,_2,_3,_3];
_8 = 262376359402150757969949570166585915976_u128 as usize;
_20 = _12;
_11 = !2861255785_u32;
_2 = 13144_i16 as u8;
_17 = '\u{ea053}';
_15.1 = 54086_u16;
_15.2 = 90_i8 | 56_i8;
RET = _6 - _19;
Goto(bb7)
}
bb7 = {
Call(_23 = dump_var(3_usize, 1_usize, Move(_1), 5_usize, Move(_5), 20_usize, Move(_20), 2_usize, Move(_2)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_23 = dump_var(3_usize, 12_usize, Move(_12), 9_usize, Move(_9), 7_usize, Move(_7), 11_usize, Move(_11)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: u32,mut _3: u32,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: i32,mut _9: isize,mut _10: (f32,)) -> u8 {
mir! {
type RET = u8;
let _11: char;
let _12: [bool; 7];
let _13: [u8; 1];
let _14: &'static f64;
let _15: [isize; 4];
let _16: [bool; 7];
let _17: &'static isize;
let _18: (Adt42, Adt54, *mut Adt23, char);
let _19: isize;
let _20: (f64, u16, i8);
let _21: f32;
let _22: *mut &'static i8;
let _23: (usize, (Adt42, Adt54, *mut Adt23, char), *const [bool; 7], ((char, u32, u32),));
let _24: ((char, u32, u32),);
let _25: &'static (&'static u128, [bool; 7]);
let _26: f64;
let _27: (i8, [u32; 4], *const u8);
let _28: isize;
let _29: *const bool;
let _30: (usize, &'static u128, ([u32; 4],), i8);
let _31: *const u128;
let _32: *mut u8;
let _33: f32;
let _34: Adt75;
let _35: u64;
let _36: *const [i32; 6];
let _37: [i32; 6];
let _38: ([isize; 4],);
let _39: ();
let _40: ();
{
_9 = '\u{a03e8}' as isize;
_7 = 13_i8 as isize;
_8 = 651102107_i32 | 283788344_i32;
_1 = _7;
_6 = -_4;
_7 = '\u{106d57}' as isize;
RET = 230_u8;
_4 = (-61_i8) as isize;
Call(RET = fn5(_6, _1, _9, _5, _2, _9, _2, _3, _4, _10.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = [true,false,true,false,true,true,false];
_3 = 85704494442848023433240709762911548901_i128 as u32;
_9 = _5;
_2 = _3 | _3;
_13 = [RET];
_2 = _3 << _6;
_9 = !_6;
_5 = 7192_i16 as isize;
_7 = _6;
_8 = 1762833772_i32;
_1 = _6;
_2 = !_3;
_1 = _8 as isize;
_5 = _6;
_9 = _5;
Goto(bb2)
}
bb2 = {
_7 = 88763134246729719905239334636116288061_i128 as isize;
_1 = _6;
RET = 7_u8 * 93_u8;
_2 = !_3;
match _8 {
0 => bb3,
1 => bb4,
1762833772 => bb6,
_ => bb5
}
}
bb3 = {
_12 = [true,false,true,false,true,true,false];
_3 = 85704494442848023433240709762911548901_i128 as u32;
_9 = _5;
_2 = _3 | _3;
_13 = [RET];
_2 = _3 << _6;
_9 = !_6;
_5 = 7192_i16 as isize;
_7 = _6;
_8 = 1762833772_i32;
_1 = _6;
_2 = !_3;
_1 = _8 as isize;
_5 = _6;
_9 = _5;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_7 = _1 ^ _5;
Call(_6 = core::intrinsics::bswap(_7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = 79_u8;
_4 = _1;
_18.3 = '\u{3a4c2}';
_17 = &_6;
_7 = (*_17);
_2 = 1155954806963467964306950487694158713_u128 as u32;
_18.1 = Adt54::Variant0 { fld0: _8 };
place!(Field::<i32>(Variant(_18.1, 0), 0)) = _8 * _8;
_6 = _4;
_6 = _7 ^ _1;
RET = _2 as u8;
_15 = [_6,_7,_6,_7];
_7 = _6 >> _8;
_10.0 = _8 as f32;
_10.0 = _6 as f32;
_13 = [RET];
_18.0 = Adt42::Variant2 { fld0: (-92262989386212213_i64),fld1: (-29563_i16),fld2: _13 };
_19 = RET as isize;
match _8 {
0 => bb8,
1762833772 => bb10,
_ => bb9
}
}
bb8 = {
_7 = 88763134246729719905239334636116288061_i128 as isize;
_1 = _6;
RET = 7_u8 * 93_u8;
_2 = !_3;
match _8 {
0 => bb3,
1 => bb4,
1762833772 => bb6,
_ => bb5
}
}
bb9 = {
Return()
}
bb10 = {
_13 = Field::<[u8; 1]>(Variant(_18.0, 2), 2);
_20.2 = 28_i8 - (-87_i8);
place!(Field::<i64>(Variant(_18.0, 2), 0)) = 2574296073386030341_i64 + 375474764251319291_i64;
_4 = 12226373547364207424_usize as isize;
place!(Field::<i16>(Variant(_18.0, 2), 1)) = !(-807_i16);
place!(Field::<i64>(Variant(_18.0, 2), 0)) = 4822659575475629378_i64 + 8886850268676141524_i64;
_14 = &_20.0;
_23.3.0.0 = _18.3;
_8 = Field::<i32>(Variant(_18.1, 0), 0);
_14 = &(*_14);
_23.3.0 = (_18.3, _2, _2);
_23.1.0 = Move(_18.0);
_24 = (_23.3.0,);
_17 = &_6;
_23.1.1 = Adt54::Variant2 { fld0: Field::<i64>(Variant(_23.1.0, 2), 0) };
_20.0 = Field::<i64>(Variant(_23.1.1, 2), 0) as f64;
_18.0 = Move(_23.1.0);
_12 = [false,true,true,false,true,false,true];
_2 = !_23.3.0.2;
Goto(bb11)
}
bb11 = {
_23.1.1 = Adt54::Variant0 { fld0: _8 };
_23.1.3 = _24.0.0;
_13 = Field::<[u8; 1]>(Variant(_18.0, 2), 2);
_20.1 = 27952_u16 | 16540_u16;
_23.1.1 = Move(_18.1);
_3 = _24.0.1 + _24.0.2;
_8 = Field::<i32>(Variant(_23.1.1, 0), 0);
_23.3.0 = (_24.0.0, _2, _3);
_15 = [_6,_5,_1,_7];
_20.0 = _4 as f64;
Goto(bb12)
}
bb12 = {
_23.3.0.1 = _23.1.3 as u32;
SetDiscriminant(_18.0, 1);
_18.1 = Move(_23.1.1);
_23.0 = !6_usize;
_27.0 = _20.2;
_20.1 = 37440_u16;
SetDiscriminant(_18.1, 0);
_8 = _10.0 as i32;
_11 = _23.1.3;
_24.0.0 = _18.3;
place!(Field::<*const u8>(Variant(_18.0, 1), 0)) = core::ptr::addr_of!(RET);
_28 = _9 * (*_17);
_23.1.3 = _18.3;
place!(Field::<i32>(Variant(_18.1, 0), 0)) = _8;
_23.3 = (_24.0,);
_24 = (_23.3.0,);
_23.1.0 = Adt42::Variant2 { fld0: 3983050436312036665_i64,fld1: (-28867_i16),fld2: _13 };
_2 = _24.0.2;
_23.3.0.0 = _24.0.0;
_32 = core::ptr::addr_of_mut!(RET);
_24 = _23.3;
_30.2.0 = [_24.0.2,_24.0.2,_3,_2];
_14 = &_26;
_1 = _20.2 as isize;
_2 = _24.0.1 - _3;
match _20.1 {
0 => bb4,
37440 => bb14,
_ => bb13
}
}
bb13 = {
_12 = [true,false,true,false,true,true,false];
_3 = 85704494442848023433240709762911548901_i128 as u32;
_9 = _5;
_2 = _3 | _3;
_13 = [RET];
_2 = _3 << _6;
_9 = !_6;
_5 = 7192_i16 as isize;
_7 = _6;
_8 = 1762833772_i32;
_1 = _6;
_2 = !_3;
_1 = _8 as isize;
_5 = _6;
_9 = _5;
Goto(bb2)
}
bb14 = {
place!(Field::<[u8; 1]>(Variant(_23.1.0, 2), 2)) = [(*_32)];
_27.1 = [_24.0.1,_23.3.0.2,_23.3.0.1,_2];
_27 = (_20.2, _30.2.0, Move(Field::<*const u8>(Variant(_18.0, 1), 0)));
_23.2 = core::ptr::addr_of!(_16);
_14 = &(*_14);
(*_32) = 146_u8;
_23.1.3 = _18.3;
_8 = _2 as i32;
_5 = _28;
SetDiscriminant(_18.1, 1);
_30.3 = _20.1 as i8;
_5 = _20.0 as isize;
_11 = _23.3.0.0;
_32 = core::ptr::addr_of_mut!((*_32));
place!(Field::<u64>(Variant(_18.1, 1), 2)) = 18081630596944303133_u64 << _9;
place!(Field::<i16>(Variant(_18.1, 1), 4)) = (-31472_i16);
place!(Field::<u64>(Variant(_18.1, 1), 2)) = 6334934903321991726_u64 | 15804143938371383977_u64;
_30.0 = _23.3.0.0 as usize;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(4_usize, 9_usize, Move(_9), 11_usize, Move(_11), 28_usize, Move(_28), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(4_usize, 2_usize, Move(_2), 15_usize, Move(_15), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: u32,mut _6: isize,mut _7: u32,mut _8: u32,mut _9: isize,mut _10: f32) -> u8 {
mir! {
type RET = u8;
let _11: f32;
let _12: f32;
let _13: *const i16;
let _14: (isize, f32, u8, u16);
let _15: *mut u8;
let _16: &'static [isize; 7];
let _17: [bool; 7];
let _18: isize;
let _19: u32;
let _20: bool;
let _21: i16;
let _22: &'static [isize; 4];
let _23: &'static (&'static u128, [bool; 7]);
let _24: u8;
let _25: char;
let _26: bool;
let _27: *mut Adt23;
let _28: Adt23;
let _29: &'static [isize; 4];
let _30: &'static *const i16;
let _31: f64;
let _32: &'static (&'static u128, [bool; 7]);
let _33: [usize; 2];
let _34: u32;
let _35: (&'static u128, [bool; 7]);
let _36: i16;
let _37: isize;
let _38: (char, u32, u32);
let _39: *const *const [bool; 7];
let _40: (i128, i64, i8, i8);
let _41: f32;
let _42: isize;
let _43: (Adt42, Adt54, *mut Adt23, char);
let _44: f64;
let _45: (Adt42, Adt54, *mut Adt23, char);
let _46: char;
let _47: *const *const [bool; 7];
let _48: f32;
let _49: [usize; 2];
let _50: ();
let _51: ();
{
_4 = _3 << _7;
_3 = _1 - _2;
_6 = _3;
_1 = _3 - _3;
_7 = !_5;
RET = 7778619182077896458_i64 as u8;
_3 = 1201762744902288856_i64 as isize;
_8 = _5 & _7;
_10 = 134103561448475047293328465972142657280_i128 as f32;
_11 = _10 + _10;
_10 = _11 + _11;
RET = !51_u8;
_6 = -_1;
_9 = _6 | _4;
_12 = _11;
_9 = (-544331125_i32) as isize;
_5 = !_7;
Call(_14.1 = fn6(_6, _1, _1, _1, _1, _1, _6, _6, _6, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = _14.1 * _10;
_14.1 = 15536664185428914288_usize as f32;
_15 = core::ptr::addr_of_mut!(_14.2);
_6 = _1;
_2 = _6;
_14 = (_1, _12, RET, 55945_u16);
_4 = _2 << _1;
_7 = _8 + _8;
_18 = -_14.0;
_14.0 = _18 & _2;
_6 = _1 - _18;
_5 = _7 | _8;
_17 = [true,true,false,false,true,true,false];
_6 = _14.0;
match _14.3 {
55945 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_13 = core::ptr::addr_of!(_21);
RET = (*_15);
_1 = _6 - _18;
_2 = _6 - _18;
_9 = _4;
_14.0 = -_2;
_10 = 2691005449910748983_i64 as f32;
_6 = -_4;
RET = _14.2;
_18 = (-160853831518904247415404298890348398557_i128) as isize;
(*_13) = (-24144_i16) << _4;
_6 = _14.3 as isize;
_18 = RET as isize;
RET = (*_15) ^ (*_15);
_2 = _6;
_14.1 = _10 + _11;
(*_13) = (-10467_i16) + 20421_i16;
_14.1 = _12 * _11;
match _14.3 {
0 => bb4,
55945 => bb6,
_ => bb5
}
}
bb4 = {
Return()
}
bb5 = {
_11 = _14.1 * _10;
_14.1 = 15536664185428914288_usize as f32;
_15 = core::ptr::addr_of_mut!(_14.2);
_6 = _1;
_2 = _6;
_14 = (_1, _12, RET, 55945_u16);
_4 = _2 << _1;
_7 = _8 + _8;
_18 = -_14.0;
_14.0 = _18 & _2;
_6 = _1 - _18;
_5 = _7 | _8;
_17 = [true,true,false,false,true,true,false];
_6 = _14.0;
match _14.3 {
55945 => bb3,
_ => bb2
}
}
bb6 = {
_18 = _6 * _9;
RET = (*_15);
_20 = false;
RET = _14.2 - (*_15);
_28.fld0 = [RET];
_4 = _9 * _18;
_7 = _8;
_20 = true | true;
_28.fld0 = [(*_15)];
(*_13) = (-32468_i16) & (-26805_i16);
_2 = _4;
_26 = _18 > _14.0;
_5 = 50387340477914953381476470810903838498_u128 as u32;
_5 = !_7;
_14.3 = 53967_u16;
RET = _14.2 - _14.2;
_5 = _7;
_20 = !_26;
_6 = _14.0 << _4;
_14.2 = RET << _9;
_28.fld0 = [_14.2];
_19 = !_7;
Goto(bb7)
}
bb7 = {
(*_15) = RET;
_15 = core::ptr::addr_of_mut!((*_15));
_18 = _26 as isize;
_14.1 = _11;
_24 = !(*_15);
_25 = '\u{a9c28}';
_5 = _14.0 as u32;
_14.2 = !_24;
RET = _24 ^ (*_15);
_25 = '\u{e6fb7}';
_31 = 102_i8 as f64;
_3 = _18 << _5;
_26 = !_20;
(*_15) = 11140235574317324676_usize as u8;
_21 = 1683467830445340646_u64 as i16;
_10 = _12;
(*_13) = -268_i16;
_30 = &_13;
_4 = -_18;
match _14.3 {
0 => bb4,
1 => bb5,
53967 => bb9,
_ => bb8
}
}
bb8 = {
_11 = _14.1 * _10;
_14.1 = 15536664185428914288_usize as f32;
_15 = core::ptr::addr_of_mut!(_14.2);
_6 = _1;
_2 = _6;
_14 = (_1, _12, RET, 55945_u16);
_4 = _2 << _1;
_7 = _8 + _8;
_18 = -_14.0;
_14.0 = _18 & _2;
_6 = _1 - _18;
_5 = _7 | _8;
_17 = [true,true,false,false,true,true,false];
_6 = _14.0;
match _14.3 {
55945 => bb3,
_ => bb2
}
}
bb9 = {
_6 = _3 | _4;
_5 = (*_13) as u32;
_14.1 = 133012378406126544826485800684920824666_u128 as f32;
_14.0 = _3;
_14.2 = _24;
_14.3 = 57680_u16;
_33 = [5099054573535287367_usize,9547168024772195085_usize];
_14.3 = 63103_u16 ^ 40577_u16;
_7 = _5;
_34 = 192688319751412275427589620167732793334_u128 as u32;
_12 = -_11;
_8 = _19 << _14.0;
_3 = _31 as isize;
_4 = _6;
_28.fld0 = [RET];
(*_15) = RET;
_25 = '\u{5845f}';
_36 = _14.3 as i16;
_27 = core::ptr::addr_of_mut!(_28);
_28.fld0 = [_14.2];
_27 = core::ptr::addr_of_mut!((*_27));
Goto(bb10)
}
bb10 = {
RET = (*_15) * (*_15);
_10 = _11;
_14.0 = _2;
_38.0 = _25;
(*_27).fld0 = [RET];
_20 = !_26;
_12 = -_11;
_27 = core::ptr::addr_of_mut!(_28);
(*_13) = -_36;
_14 = (_6, _12, RET, 22207_u16);
_6 = !_18;
_38 = (_25, _8, _8);
_40.1 = !(-5723100326861209965_i64);
_40.2 = _8 as i8;
_40.2 = (-77_i8) - 25_i8;
(*_27).fld0 = [_14.2];
RET = !_14.2;
(*_27).fld0 = [(*_15)];
_37 = _14.0 - _14.0;
match _14.3 {
0 => bb2,
1 => bb11,
22207 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
_11 = _14.1 * _10;
_14.1 = 15536664185428914288_usize as f32;
_15 = core::ptr::addr_of_mut!(_14.2);
_6 = _1;
_2 = _6;
_14 = (_1, _12, RET, 55945_u16);
_4 = _2 << _1;
_7 = _8 + _8;
_18 = -_14.0;
_14.0 = _18 & _2;
_6 = _1 - _18;
_5 = _7 | _8;
_17 = [true,true,false,false,true,true,false];
_6 = _14.0;
match _14.3 {
55945 => bb3,
_ => bb2
}
}
bb13 = {
(*_27).fld0 = [RET];
RET = _14.2;
_37 = _18 + _18;
_32 = &_35;
_14.2 = RET;
_41 = _10;
_19 = !_38.2;
RET = (*_15);
_2 = _37;
_35.1 = [_20,_20,_20,_20,_26,_26,_26];
(*_27).fld0 = [(*_15)];
_42 = 70825441779469632771818551720783953476_u128 as isize;
_14.1 = -_11;
_43.3 = _25;
_14.2 = !RET;
(*_27).fld0 = [RET];
_43.2 = core::ptr::addr_of_mut!((*_27));
_7 = _26 as u32;
_46 = _38.0;
_37 = _14.0;
_28.fld0 = [(*_15)];
_3 = _14.0;
_40 = (104902575227335267496462424039550943708_i128, 6244091754086184246_i64, 120_i8, 16_i8);
_15 = core::ptr::addr_of_mut!(RET);
Call(_45.3 = fn17(Move(_32), Move(_30), _37, _6, _14.3, _9, _18, _2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_34 = _40.1 as u32;
(*_13) = -_36;
_2 = _8 as isize;
_25 = _43.3;
_45.1 = Adt54::Variant0 { fld0: (-1846688075_i32) };
_7 = _10 as u32;
_25 = _38.0;
(*_15) = _24;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(5_usize, 18_usize, Move(_18), 4_usize, Move(_4), 17_usize, Move(_17), 40_usize, Move(_40)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(5_usize, 20_usize, Move(_20), 21_usize, Move(_21), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(5_usize, 25_usize, Move(_25), 38_usize, Move(_38), 34_usize, Move(_34), 46_usize, Move(_46)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> f32 {
mir! {
type RET = f32;
let _12: bool;
let _13: char;
let _14: ((char, u32, u32),);
let _15: [char; 1];
let _16: *mut [u8; 1];
let _17: [i128; 4];
let _18: (i128, i64, i8, i8);
let _19: [i128; 4];
let _20: u8;
let _21: u128;
let _22: (*const u8, [bool; 8], (isize, f32, u8, u16), f32);
let _23: ([isize; 4],);
let _24: *const u128;
let _25: f64;
let _26: usize;
let _27: &'static (&'static u128, [bool; 7]);
let _28: (usize, &'static u128, ([u32; 4],), i8);
let _29: f64;
let _30: f32;
let _31: isize;
let _32: i128;
let _33: char;
let _34: bool;
let _35: i16;
let _36: *const ((char, u32, u32),);
let _37: usize;
let _38: ();
let _39: ();
{
_8 = -_9;
_10 = 1603851476441817498_u64 as isize;
_9 = _5;
RET = 23440_i16 as f32;
_2 = _5;
Goto(bb1)
}
bb1 = {
_11 = _6 * _4;
_5 = !_11;
RET = (-123_i8) as f32;
_10 = _11 | _7;
_14.0.0 = '\u{ad617}';
_9 = _3;
RET = 145518172834314130911629037713283281261_u128 as f32;
Goto(bb2)
}
bb2 = {
_14.0.2 = 1832256167_u32 * 191027963_u32;
_3 = _10;
_14.0.0 = '\u{981ae}';
_15 = [_14.0.0];
Call(_3 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = (-148471716393624900865997144330968256539_i128) as isize;
_18.0 = !(-56373362873280909355934290975061400535_i128);
_18.0 = (-12173433688518969154407860688776554047_i128);
_7 = 641280296457318413_u64 as isize;
_18.2 = 125_i8 * (-67_i8);
_9 = _10 & _3;
_13 = _14.0.0;
_17 = [_18.0,_18.0,_18.0,_18.0];
_8 = _9;
_22.1 = [true,false,true,true,false,true,false,true];
_22.2 = (_9, RET, 69_u8, 40193_u16);
_12 = false;
_17 = [_18.0,_18.0,_18.0,_18.0];
_14.0.2 = (-26757_i16) as u32;
_6 = _11 * _8;
_6 = (-6059079134533604391_i64) as isize;
_24 = core::ptr::addr_of!(_21);
_18.2 = 50_i8 & 97_i8;
Goto(bb4)
}
bb4 = {
_8 = -_9;
_14.0.0 = _13;
_14.0.2 = _12 as u32;
_22.3 = 16065869577771651240_u64 as f32;
RET = _22.2.1 - _22.2.1;
_19 = [_18.0,_18.0,_18.0,_18.0];
_18 = ((-103273806927704965121944967441233605887_i128), (-5203324676270045817_i64), 17_i8, (-35_i8));
_22.2.3 = (-1544574920_i32) as u16;
_25 = 254235124167807523855257825736318754386_u128 as f64;
_7 = _11;
RET = _22.2.1;
_22.2 = (_3, RET, 178_u8, 54162_u16);
_18.0 = (-26224_i16) as i128;
_22.0 = core::ptr::addr_of!(_20);
_14.0.1 = _14.0.2 | _14.0.2;
(*_24) = _12 as u128;
Goto(bb5)
}
bb5 = {
_14.0.0 = _13;
_13 = _14.0.0;
_18.2 = -_18.3;
_22.2.2 = 187_u8 * 59_u8;
RET = _22.2.1;
_8 = _10 & _9;
_22.2 = (_3, _22.3, 157_u8, 31823_u16);
_18.0 = _22.2.3 as i128;
Goto(bb6)
}
bb6 = {
_25 = _22.3 as f64;
_22.2.3 = _10 as u16;
_11 = -_22.2.0;
_2 = 18660_i16 as isize;
_30 = RET;
_22.2.3 = 33247_u16;
_24 = core::ptr::addr_of!((*_24));
_22.2 = (_9, _30, 162_u8, 51559_u16);
_10 = -_22.2.0;
_2 = !_10;
_28.2.0 = [_14.0.2,_14.0.2,_14.0.1,_14.0.1];
_14.0.1 = (-1077120900_i32) as u32;
(*_24) = _8 as u128;
_2 = _10;
_4 = !_2;
RET = _22.2.1;
_2 = _22.2.0 ^ _8;
_28.0 = 4_usize;
_22.2 = (_10, _22.3, 34_u8, 42981_u16);
match _22.2.3 {
0 => bb1,
1 => bb2,
2 => bb7,
42981 => bb9,
_ => bb8
}
}
bb7 = {
_14.0.2 = 1832256167_u32 * 191027963_u32;
_3 = _10;
_14.0.0 = '\u{981ae}';
_15 = [_14.0.0];
Call(_3 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_9 = (-148471716393624900865997144330968256539_i128) as isize;
_18.0 = !(-56373362873280909355934290975061400535_i128);
_18.0 = (-12173433688518969154407860688776554047_i128);
_7 = 641280296457318413_u64 as isize;
_18.2 = 125_i8 * (-67_i8);
_9 = _10 & _3;
_13 = _14.0.0;
_17 = [_18.0,_18.0,_18.0,_18.0];
_8 = _9;
_22.1 = [true,false,true,true,false,true,false,true];
_22.2 = (_9, RET, 69_u8, 40193_u16);
_12 = false;
_17 = [_18.0,_18.0,_18.0,_18.0];
_14.0.2 = (-26757_i16) as u32;
_6 = _11 * _8;
_6 = (-6059079134533604391_i64) as isize;
_24 = core::ptr::addr_of!(_21);
_18.2 = 50_i8 & 97_i8;
Goto(bb4)
}
bb9 = {
_22.0 = core::ptr::addr_of!(_20);
_14.0 = (_13, 968471023_u32, 1415085431_u32);
_28.0 = !6_usize;
_28.2.0 = [_14.0.2,_14.0.1,_14.0.2,_14.0.1];
_7 = _2 ^ _9;
_18.0 = !75467863256343494607694600902103912802_i128;
_20 = !_22.2.2;
_12 = _22.2.0 > _10;
_25 = _14.0.2 as f64;
Call(_9 = fn7(Move(_24), _22.2, _7, _2, _3, (*_24), (*_24), _2, _22.2.3, _18.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_22.3 = -_22.2.1;
_28.3 = _18.2 & _18.2;
match _22.2.2 {
0 => bb3,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
34 => bb18,
_ => bb17
}
}
bb11 = {
_22.0 = core::ptr::addr_of!(_20);
_14.0 = (_13, 968471023_u32, 1415085431_u32);
_28.0 = !6_usize;
_28.2.0 = [_14.0.2,_14.0.1,_14.0.2,_14.0.1];
_7 = _2 ^ _9;
_18.0 = !75467863256343494607694600902103912802_i128;
_20 = !_22.2.2;
_12 = _22.2.0 > _10;
_25 = _14.0.2 as f64;
Call(_9 = fn7(Move(_24), _22.2, _7, _2, _3, (*_24), (*_24), _2, _22.2.3, _18.3), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_9 = (-148471716393624900865997144330968256539_i128) as isize;
_18.0 = !(-56373362873280909355934290975061400535_i128);
_18.0 = (-12173433688518969154407860688776554047_i128);
_7 = 641280296457318413_u64 as isize;
_18.2 = 125_i8 * (-67_i8);
_9 = _10 & _3;
_13 = _14.0.0;
_17 = [_18.0,_18.0,_18.0,_18.0];
_8 = _9;
_22.1 = [true,false,true,true,false,true,false,true];
_22.2 = (_9, RET, 69_u8, 40193_u16);
_12 = false;
_17 = [_18.0,_18.0,_18.0,_18.0];
_14.0.2 = (-26757_i16) as u32;
_6 = _11 * _8;
_6 = (-6059079134533604391_i64) as isize;
_24 = core::ptr::addr_of!(_21);
_18.2 = 50_i8 & 97_i8;
Goto(bb4)
}
bb13 = {
_14.0.2 = 1832256167_u32 * 191027963_u32;
_3 = _10;
_14.0.0 = '\u{981ae}';
_15 = [_14.0.0];
Call(_3 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_25 = _22.3 as f64;
_22.2.3 = _10 as u16;
_11 = -_22.2.0;
_2 = 18660_i16 as isize;
_30 = RET;
_22.2.3 = 33247_u16;
_24 = core::ptr::addr_of!((*_24));
_22.2 = (_9, _30, 162_u8, 51559_u16);
_10 = -_22.2.0;
_2 = !_10;
_28.2.0 = [_14.0.2,_14.0.2,_14.0.1,_14.0.1];
_14.0.1 = (-1077120900_i32) as u32;
(*_24) = _8 as u128;
_2 = _10;
_4 = !_2;
RET = _22.2.1;
_2 = _22.2.0 ^ _8;
_28.0 = 4_usize;
_22.2 = (_10, _22.3, 34_u8, 42981_u16);
match _22.2.3 {
0 => bb1,
1 => bb2,
2 => bb7,
42981 => bb9,
_ => bb8
}
}
bb15 = {
_14.0.0 = _13;
_13 = _14.0.0;
_18.2 = -_18.3;
_22.2.2 = 187_u8 * 59_u8;
RET = _22.2.1;
_8 = _10 & _9;
_22.2 = (_3, _22.3, 157_u8, 31823_u16);
_18.0 = _22.2.3 as i128;
Goto(bb6)
}
bb16 = {
_8 = -_9;
_14.0.0 = _13;
_14.0.2 = _12 as u32;
_22.3 = 16065869577771651240_u64 as f32;
RET = _22.2.1 - _22.2.1;
_19 = [_18.0,_18.0,_18.0,_18.0];
_18 = ((-103273806927704965121944967441233605887_i128), (-5203324676270045817_i64), 17_i8, (-35_i8));
_22.2.3 = (-1544574920_i32) as u16;
_25 = 254235124167807523855257825736318754386_u128 as f64;
_7 = _11;
RET = _22.2.1;
_22.2 = (_3, RET, 178_u8, 54162_u16);
_18.0 = (-26224_i16) as i128;
_22.0 = core::ptr::addr_of!(_20);
_14.0.1 = _14.0.2 | _14.0.2;
(*_24) = _12 as u128;
Goto(bb5)
}
bb17 = {
_9 = (-148471716393624900865997144330968256539_i128) as isize;
_18.0 = !(-56373362873280909355934290975061400535_i128);
_18.0 = (-12173433688518969154407860688776554047_i128);
_7 = 641280296457318413_u64 as isize;
_18.2 = 125_i8 * (-67_i8);
_9 = _10 & _3;
_13 = _14.0.0;
_17 = [_18.0,_18.0,_18.0,_18.0];
_8 = _9;
_22.1 = [true,false,true,true,false,true,false,true];
_22.2 = (_9, RET, 69_u8, 40193_u16);
_12 = false;
_17 = [_18.0,_18.0,_18.0,_18.0];
_14.0.2 = (-26757_i16) as u32;
_6 = _11 * _8;
_6 = (-6059079134533604391_i64) as isize;
_24 = core::ptr::addr_of!(_21);
_18.2 = 50_i8 & 97_i8;
Goto(bb4)
}
bb18 = {
_2 = _11;
_33 = _13;
_25 = 5393_i16 as f64;
_22.3 = RET * _22.2.1;
_22.2.3 = !23432_u16;
_28.0 = 4_usize;
_28.1 = &_21;
_33 = _13;
_14.0.0 = _13;
_18.3 = !_28.3;
_23.0 = [_10,_8,_10,_9];
_18.1 = _18.0 as i64;
Goto(bb19)
}
bb19 = {
Call(_38 = dump_var(6_usize, 14_usize, Move(_14), 2_usize, Move(_2), 19_usize, Move(_19), 20_usize, Move(_20)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(6_usize, 33_usize, Move(_33), 6_usize, Move(_6), 21_usize, Move(_21), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_38 = dump_var(6_usize, 9_usize, Move(_9), 10_usize, Move(_10), 5_usize, Move(_5), 39_usize, _39), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *const u128,mut _2: (isize, f32, u8, u16),mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u128,mut _7: u128,mut _8: isize,mut _9: u16,mut _10: i8) -> isize {
mir! {
type RET = isize;
let _11: i8;
let _12: &'static i8;
let _13: ();
let _14: ();
{
_9 = !_2.3;
_3 = (-154350738624464819353083661376763813017_i128) as isize;
Call(_3 = fn8(_9, _4, _4, _2.0, _2.2, _8, _2.3, _4, _4, _2.0, _2.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.2 = !221_u8;
Call(_1 = fn11(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _4 * _2.0;
_9 = !_2.3;
_2.2 = !224_u8;
_4 = RET;
_5 = _8 * _8;
_4 = (-22674_i16) as isize;
_2.0 = _8 * RET;
_2.3 = !_9;
_6 = _7 ^ _7;
_7 = (-4123111551702403215_i64) as u128;
_1 = core::ptr::addr_of!(_7);
_12 = &_11;
(*_1) = !_6;
_10 = 26_i8 + 43_i8;
_1 = core::ptr::addr_of!((*_1));
Goto(bb3)
}
bb3 = {
Call(_13 = dump_var(7_usize, 8_usize, Move(_8), 6_usize, Move(_6), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u16,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: u8,mut _6: isize,mut _7: u16,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: u16) -> isize {
mir! {
type RET = isize;
let _12: bool;
let _13: i128;
let _14: f64;
let _15: Adt42;
let _16: Adt23;
let _17: [bool; 7];
let _18: [u32; 4];
let _19: usize;
let _20: [i32; 6];
let _21: (usize, (Adt42, Adt54, *mut Adt23, char), *const [bool; 7], ((char, u32, u32),));
let _22: ([u32; 4],);
let _23: &'static isize;
let _24: i8;
let _25: bool;
let _26: isize;
let _27: i8;
let _28: Adt78;
let _29: (i8, [u32; 4], *const u8);
let _30: ();
let _31: ();
{
_4 = 172130434423953860488438125999231859826_u128 as isize;
_8 = !_2;
RET = _8;
RET = (-27739_i16) as isize;
_1 = true as u16;
_10 = _2;
_13 = 165246030028673546085261391219631237338_i128;
Call(_14 = fn9(_5, _2, _7, _7, _10, _11, _8, _2, _8, _9, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 4_usize as isize;
_2 = _10;
Goto(bb2)
}
bb2 = {
_18 = [2757943025_u32,2040741324_u32,93514398_u32,3633314910_u32];
_7 = _11 % _11;
_16.fld0 = [_5];
_1 = !_11;
_15 = Adt42::Variant0 { fld0: _18 };
_3 = !_10;
_17 = [true,false,true,true,false,true,false];
_13 = -(-46225918099349047458277444031132916064_i128);
_13 = -(-103554842093456030509788350223560843475_i128);
_16.fld0 = [_5];
_20 = [(-60978456_i32),1668748843_i32,1031399472_i32,(-1427005879_i32),(-1212378069_i32),1480587320_i32];
_19 = !8368506338658116212_usize;
_6 = 2665755768_u32 as isize;
Call(_16.fld0 = fn10(_14, _11, _11, _9, _8, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = !_2;
_4 = _3;
_21.1.0 = Adt42::Variant0 { fld0: _18 };
_3 = _10 - _10;
_22.0 = _18;
_9 = -_2;
_17 = [true,true,true,true,false,false,true];
_21.3.0.0 = '\u{55c24}';
_15 = Move(_21.1.0);
_21.3.0.2 = 3077736416_u32;
_11 = _7;
_21.3.0.2 = !3372696049_u32;
_21.0 = !_19;
_15 = Adt42::Variant2 { fld0: 6838447583782293526_i64,fld1: 30298_i16,fld2: _16.fld0 };
place!(Field::<[u8; 1]>(Variant(_15, 2), 2)) = [_5];
place!(Field::<[u8; 1]>(Variant(_15, 2), 2)) = [_5];
place!(Field::<i16>(Variant(_15, 2), 1)) = 19193_i16 + 17934_i16;
_16 = Adt23 { fld0: Field::<[u8; 1]>(Variant(_15, 2), 2) };
_17 = [true,true,true,true,true,true,false];
match _5 {
0 => bb2,
34 => bb5,
_ => bb4
}
}
bb4 = {
_4 = 4_usize as isize;
_2 = _10;
Goto(bb2)
}
bb5 = {
_22.0 = [_21.3.0.2,_21.3.0.2,_21.3.0.2,_21.3.0.2];
_21.3.0 = ('\u{97046}', 3618247533_u32, 1679148740_u32);
_21.1.1 = Adt54::Variant2 { fld0: 9089435622242233257_i64 };
_19 = !_21.0;
RET = true as isize;
place!(Field::<i64>(Variant(_15, 2), 0)) = 7472661975746228882_i64 >> _8;
_6 = _8 + _2;
_24 = 0_i8 * (-74_i8);
place!(Field::<i64>(Variant(_21.1.1, 2), 0)) = _21.3.0.1 as i64;
_21.2 = core::ptr::addr_of!(_17);
place!(Field::<i16>(Variant(_15, 2), 1)) = (-4487_i16) >> _10;
SetDiscriminant(_15, 0);
_14 = _24 as f64;
_1 = _7;
_23 = &_9;
_6 = -(*_23);
_18 = [_21.3.0.2,_21.3.0.2,_21.3.0.2,_21.3.0.2];
_21.1.2 = core::ptr::addr_of_mut!(_16);
_7 = _1 * _11;
_16.fld0 = [_5];
_21.3.0.1 = _3 as u32;
_21.0 = 1438081726_i32 as usize;
_21.3.0 = ('\u{c9068}', 3685283130_u32, 4216045707_u32);
_6 = _4;
_18 = [_21.3.0.2,_21.3.0.1,_21.3.0.1,_21.3.0.1];
Goto(bb6)
}
bb6 = {
_16.fld0 = [_5];
_21.3.0.1 = _21.3.0.2 - _21.3.0.2;
_23 = &_2;
_5 = 2_u8;
_18 = [_21.3.0.1,_21.3.0.2,_21.3.0.1,_21.3.0.2];
_21.2 = core::ptr::addr_of!(_17);
_21.1.0 = Adt42::Variant0 { fld0: _18 };
_21.2 = core::ptr::addr_of!(_17);
_1 = _7 * _7;
_13 = (-145215517695368092568880880983857233340_i128) * 119824476720826394346675045633325329793_i128;
_12 = _9 <= _2;
_20 = [1553358966_i32,(-1047742685_i32),(-1810831752_i32),282362047_i32,1309867345_i32,57950565_i32];
_21.1.3 = _21.3.0.0;
place!(Field::<i64>(Variant(_21.1.1, 2), 0)) = -362153703702637377_i64;
_22 = (_18,);
_5 = 162_u8 & 105_u8;
_8 = _6 << _11;
_25 = !_12;
_7 = _21.3.0.1 as u16;
_3 = 111690793731425293079805412542494754180_u128 as isize;
_13 = !74011152419690144752767842998454376037_i128;
place!(Field::<[u32; 4]>(Variant(_21.1.0, 0), 0)) = [_21.3.0.1,_21.3.0.1,_21.3.0.1,_21.3.0.2];
_24 = 42_i8;
_16.fld0 = [_5];
_15 = Adt42::Variant0 { fld0: Field::<[u32; 4]>(Variant(_21.1.0, 0), 0) };
_12 = !_25;
_21.1.1 = Adt54::Variant0 { fld0: 1211772129_i32 };
Goto(bb7)
}
bb7 = {
_8 = -_9;
_21.2 = core::ptr::addr_of!(_17);
_4 = _21.3.0.0 as isize;
_1 = 1557427366_i32 as u16;
_23 = &(*_23);
_20 = [(-307789102_i32),(-1447764057_i32),1144928278_i32,(-816345322_i32),817216714_i32,471181177_i32];
_16.fld0 = [_5];
place!(Field::<[u32; 4]>(Variant(_15, 0), 0)) = _18;
RET = 4889396047252178852_i64 as isize;
_8 = (*_23);
_21.1.3 = _21.3.0.0;
SetDiscriminant(_21.1.0, 2);
_1 = _11 - _11;
_26 = _9;
_21.1.0 = Adt42::Variant2 { fld0: (-2913897076985784841_i64),fld1: (-4673_i16),fld2: _16.fld0 };
place!(Field::<[u32; 4]>(Variant(_15, 0), 0)) = [_21.3.0.1,_21.3.0.1,_21.3.0.1,_21.3.0.1];
_13 = 123929401878237661374100372904782812143_i128 | (-114413928449275215345277793202917742036_i128);
_21.1.0 = Move(_15);
_25 = !_12;
SetDiscriminant(_21.1.0, 0);
_24 = _21.3.0.1 as i8;
_21.3.0 = (_21.1.3, 1497082464_u32, 504465496_u32);
RET = _10 - _9;
_3 = -_8;
_2 = _11 as isize;
_29.0 = -_24;
Goto(bb8)
}
bb8 = {
Call(_30 = dump_var(8_usize, 7_usize, Move(_7), 1_usize, Move(_1), 22_usize, Move(_22), 18_usize, Move(_18)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_30 = dump_var(8_usize, 6_usize, Move(_6), 19_usize, Move(_19), 10_usize, Move(_10), 25_usize, Move(_25)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_30 = dump_var(8_usize, 9_usize, Move(_9), 20_usize, Move(_20), 31_usize, _31, 31_usize, _31), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u8,mut _2: isize,mut _3: u16,mut _4: u16,mut _5: isize,mut _6: u16,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: u16) -> f64 {
mir! {
type RET = f64;
let _12: char;
let _13: char;
let _14: u64;
let _15: char;
let _16: u32;
let _17: *mut u8;
let _18: u16;
let _19: ();
let _20: ();
{
_6 = 286741065_i32 as u16;
_11 = _8 as u16;
_7 = -_5;
_7 = -_2;
_1 = !12_u8;
RET = 6830324534145598057_usize as f64;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
42981 => bb9,
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
_7 = 163489154_u32 as isize;
_12 = '\u{191d1}';
_9 = _5 & _2;
RET = _8 as f64;
RET = _5 as f64;
_14 = !7419969862233170141_u64;
_13 = _12;
_2 = _10;
_10 = _9;
_6 = _2 as u16;
_16 = !237630803_u32;
_16 = 1689932148_u32 + 1963930496_u32;
_15 = _12;
_15 = _12;
Goto(bb10)
}
bb10 = {
Call(_19 = dump_var(9_usize, 8_usize, Move(_8), 5_usize, Move(_5), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_19 = dump_var(9_usize, 9_usize, Move(_9), 4_usize, Move(_4), 10_usize, Move(_10), 13_usize, Move(_13)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: f64,mut _2: u16,mut _3: u16,mut _4: isize,mut _5: isize,mut _6: isize) -> [u8; 1] {
mir! {
type RET = [u8; 1];
let _7: [isize; 7];
let _8: isize;
let _9: bool;
let _10: f64;
let _11: [isize; 4];
let _12: &'static [u8; 4];
let _13: i64;
let _14: &'static i8;
let _15: ((char, u32, u32),);
let _16: (&'static u128, [bool; 7]);
let _17: &'static [isize; 4];
let _18: char;
let _19: f32;
let _20: (*const u8, [bool; 8], (isize, f32, u8, u16), f32);
let _21: isize;
let _22: i128;
let _23: f32;
let _24: f32;
let _25: bool;
let _26: f64;
let _27: isize;
let _28: isize;
let _29: isize;
let _30: ();
let _31: ();
{
_3 = _2;
Goto(bb1)
}
bb1 = {
_6 = _5;
RET = [42_u8];
_7 = [_5,_6,_6,_6,_6,_5,_4];
RET = [14_u8];
_7 = [_5,_6,_6,_4,_4,_6,_4];
RET = [207_u8];
RET = [122_u8];
RET = [233_u8];
_2 = _3 * _3;
RET = [217_u8];
_6 = 5735747871159066401904218667131744516_u128 as isize;
_3 = _2 * _2;
_8 = _4;
RET = [203_u8];
_8 = _4;
RET = [138_u8];
_7 = [_5,_5,_4,_5,_8,_4,_4];
_5 = _8 >> _8;
Goto(bb2)
}
bb2 = {
_9 = false;
_3 = _2 << _5;
_13 = 22_u8 as i64;
_9 = !false;
_11 = [_8,_4,_8,_5];
_11 = [_8,_5,_5,_5];
_5 = '\u{d2323}' as isize;
_2 = _3 + _3;
_4 = _8;
_8 = !_4;
RET = [91_u8];
_9 = true;
_8 = _4;
RET = [70_u8];
_2 = _3;
_11 = [_8,_8,_4,_8];
Goto(bb3)
}
bb3 = {
_13 = !(-6834189393266501774_i64);
_11 = [_8,_4,_4,_4];
_10 = -_1;
_11 = [_4,_4,_4,_4];
_15.0.2 = !1253910935_u32;
_15.0.1 = !_15.0.2;
_2 = !_3;
RET = [195_u8];
RET = [159_u8];
_10 = 12023443587211892833_u64 as f64;
_6 = -_8;
_13 = (-5229996296261649413_i64);
_6 = _5;
_13 = 6257420828559938257_i64;
RET = [95_u8];
_13 = (-5683493120601385944_i64);
_15.0.2 = _15.0.1;
_8 = (-75_i8) as isize;
_10 = _2 as f64;
_4 = _5 - _5;
RET = [11_u8];
_15.0 = ('\u{ae20}', 1486122972_u32, 464224899_u32);
_1 = _10;
_15.0.2 = _15.0.1 & _15.0.1;
_7 = [_6,_4,_6,_8,_5,_6,_5];
_19 = _2 as f32;
Call(_4 = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = -_4;
_5 = !_6;
RET = [213_u8];
RET = [35_u8];
_7 = [_6,_4,_6,_8,_4,_4,_4];
_19 = _3 as f32;
_15.0.2 = _15.0.1 / _15.0.1;
_15.0.1 = _15.0.2;
_6 = _5;
_3 = !_2;
match _13 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463457691114311166825512 => bb9,
_ => bb8
}
}
bb5 = {
_13 = !(-6834189393266501774_i64);
_11 = [_8,_4,_4,_4];
_10 = -_1;
_11 = [_4,_4,_4,_4];
_15.0.2 = !1253910935_u32;
_15.0.1 = !_15.0.2;
_2 = !_3;
RET = [195_u8];
RET = [159_u8];
_10 = 12023443587211892833_u64 as f64;
_6 = -_8;
_13 = (-5229996296261649413_i64);
_6 = _5;
_13 = 6257420828559938257_i64;
RET = [95_u8];
_13 = (-5683493120601385944_i64);
_15.0.2 = _15.0.1;
_8 = (-75_i8) as isize;
_10 = _2 as f64;
_4 = _5 - _5;
RET = [11_u8];
_15.0 = ('\u{ae20}', 1486122972_u32, 464224899_u32);
_1 = _10;
_15.0.2 = _15.0.1 & _15.0.1;
_7 = [_6,_4,_6,_8,_5,_6,_5];
_19 = _2 as f32;
Call(_4 = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_9 = false;
_3 = _2 << _5;
_13 = 22_u8 as i64;
_9 = !false;
_11 = [_8,_4,_8,_5];
_11 = [_8,_5,_5,_5];
_5 = '\u{d2323}' as isize;
_2 = _3 + _3;
_4 = _8;
_8 = !_4;
RET = [91_u8];
_9 = true;
_8 = _4;
RET = [70_u8];
_2 = _3;
_11 = [_8,_8,_4,_8];
Goto(bb3)
}
bb7 = {
_6 = _5;
RET = [42_u8];
_7 = [_5,_6,_6,_6,_6,_5,_4];
RET = [14_u8];
_7 = [_5,_6,_6,_4,_4,_6,_4];
RET = [207_u8];
RET = [122_u8];
RET = [233_u8];
_2 = _3 * _3;
RET = [217_u8];
_6 = 5735747871159066401904218667131744516_u128 as isize;
_3 = _2 * _2;
_8 = _4;
RET = [203_u8];
_8 = _4;
RET = [138_u8];
_7 = [_5,_5,_4,_5,_8,_4,_4];
_5 = _8 >> _8;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_20.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17 = &_11;
_22 = 155722208794395266804877685290998298484_i128 << _3;
_18 = _15.0.0;
_11 = [_6,_4,_5,_4];
_20.2.2 = 44_u8 * 10_u8;
_6 = _8;
_16.1 = [_9,_9,_9,_9,_9,_9,_9];
_21 = 256967297107027289573809195029982034470_u128 as isize;
_15.0.2 = _15.0.1 + _15.0.1;
_19 = 11152838438575776607_u64 as f32;
_20.0 = core::ptr::addr_of!(_20.2.2);
match _13 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463457691114311166825512 => bb16,
_ => bb15
}
}
bb10 = {
Return()
}
bb11 = {
_6 = _5;
RET = [42_u8];
_7 = [_5,_6,_6,_6,_6,_5,_4];
RET = [14_u8];
_7 = [_5,_6,_6,_4,_4,_6,_4];
RET = [207_u8];
RET = [122_u8];
RET = [233_u8];
_2 = _3 * _3;
RET = [217_u8];
_6 = 5735747871159066401904218667131744516_u128 as isize;
_3 = _2 * _2;
_8 = _4;
RET = [203_u8];
_8 = _4;
RET = [138_u8];
_7 = [_5,_5,_4,_5,_8,_4,_4];
_5 = _8 >> _8;
Goto(bb2)
}
bb12 = {
_6 = _5;
RET = [42_u8];
_7 = [_5,_6,_6,_6,_6,_5,_4];
RET = [14_u8];
_7 = [_5,_6,_6,_4,_4,_6,_4];
RET = [207_u8];
RET = [122_u8];
RET = [233_u8];
_2 = _3 * _3;
RET = [217_u8];
_6 = 5735747871159066401904218667131744516_u128 as isize;
_3 = _2 * _2;
_8 = _4;
RET = [203_u8];
_8 = _4;
RET = [138_u8];
_7 = [_5,_5,_4,_5,_8,_4,_4];
_5 = _8 >> _8;
Goto(bb2)
}
bb13 = {
_9 = false;
_3 = _2 << _5;
_13 = 22_u8 as i64;
_9 = !false;
_11 = [_8,_4,_8,_5];
_11 = [_8,_5,_5,_5];
_5 = '\u{d2323}' as isize;
_2 = _3 + _3;
_4 = _8;
_8 = !_4;
RET = [91_u8];
_9 = true;
_8 = _4;
RET = [70_u8];
_2 = _3;
_11 = [_8,_8,_4,_8];
Goto(bb3)
}
bb14 = {
_6 = -_4;
_5 = !_6;
RET = [213_u8];
RET = [35_u8];
_7 = [_6,_4,_6,_8,_4,_4,_4];
_19 = _3 as f32;
_15.0.2 = _15.0.1 / _15.0.1;
_15.0.1 = _15.0.2;
_6 = _5;
_3 = !_2;
match _13 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463457691114311166825512 => bb9,
_ => bb8
}
}
bb15 = {
_13 = !(-6834189393266501774_i64);
_11 = [_8,_4,_4,_4];
_10 = -_1;
_11 = [_4,_4,_4,_4];
_15.0.2 = !1253910935_u32;
_15.0.1 = !_15.0.2;
_2 = !_3;
RET = [195_u8];
RET = [159_u8];
_10 = 12023443587211892833_u64 as f64;
_6 = -_8;
_13 = (-5229996296261649413_i64);
_6 = _5;
_13 = 6257420828559938257_i64;
RET = [95_u8];
_13 = (-5683493120601385944_i64);
_15.0.2 = _15.0.1;
_8 = (-75_i8) as isize;
_10 = _2 as f64;
_4 = _5 - _5;
RET = [11_u8];
_15.0 = ('\u{ae20}', 1486122972_u32, 464224899_u32);
_1 = _10;
_15.0.2 = _15.0.1 & _15.0.1;
_7 = [_6,_4,_6,_8,_5,_6,_5];
_19 = _2 as f32;
Call(_4 = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb16 = {
_20.2.0 = 1_usize as isize;
_20.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_20.3 = _19;
_3 = _2 + _2;
_18 = _15.0.0;
_10 = _1;
_2 = _3 - _3;
_15.0.1 = 1850040483682329679_u64 as u32;
_20.3 = 31_i8 as f32;
_2 = _3;
_25 = _9;
_7 = [_8,_4,_8,_21,_8,_20.2.0,_6];
_24 = _20.3;
_23 = _24 - _20.3;
_27 = _4;
_15.0.2 = _15.0.1 & _15.0.1;
_2 = !_3;
_27 = _4 ^ _5;
_27 = _6 | _20.2.0;
_2 = _3 | _3;
_17 = &_11;
RET = [_20.2.2];
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(10_usize, 5_usize, Move(_5), 2_usize, Move(_2), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(10_usize, 18_usize, Move(_18), 4_usize, Move(_4), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> *const u128 {
mir! {
type RET = *const u128;
let _1: ((isize, f32, u8, u16), u128, usize, [u8; 1]);
let _2: i32;
let _3: *const [bool; 7];
let _4: isize;
let _5: bool;
let _6: isize;
let _7: i64;
let _8: usize;
let _9: *mut &'static (i128, i64, i8, i8);
let _10: bool;
let _11: [char; 1];
let _12: bool;
let _13: f64;
let _14: usize;
let _15: isize;
let _16: isize;
let _17: f64;
let _18: Adt54;
let _19: Adt78;
let _20: u64;
let _21: *const (i128, i64, i8, i8);
let _22: f32;
let _23: (*const u8, [bool; 8], (isize, f32, u8, u16), f32);
let _24: char;
let _25: isize;
let _26: i128;
let _27: [bool; 6];
let _28: u8;
let _29: f32;
let _30: i16;
let _31: f64;
let _32: [i128; 4];
let _33: ();
let _34: ();
{
_1.0.0 = 132567187543515877794371826218002994290_i128 as isize;
_1.1 = 27218236094788589961021308523436941106_u128 | 248535805328374498115358150835592680511_u128;
_1.0.0 = (-8_isize) | 9223372036854775807_isize;
_1.3 = [132_u8];
_1.0.1 = 1681867668_u32 as f32;
RET = core::ptr::addr_of!(_1.1);
_1.0.0 = 48_isize | (-9223372036854775808_isize);
RET = core::ptr::addr_of!(_1.1);
_1.0.3 = 38316_u16 >> (*RET);
_2 = true as i32;
Call(_1.0.0 = fn12(_1.0.3, Move(RET)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.3 = [58_u8];
RET = core::ptr::addr_of!(_1.1);
_1.3 = [123_u8];
(*RET) = 185841990396301198875898976402387562827_u128 + 339540458760193553885447603453558620702_u128;
RET = core::ptr::addr_of!(_1.1);
Goto(bb2)
}
bb2 = {
(*RET) = 190401753798576157897596323779032577931_u128 - 279229559440960648406391897424059650211_u128;
_1.0.2 = 71_u8 * 100_u8;
_1.0.0 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_1.0.2 = 228_u8;
_1.0.1 = (-7172261846191614503_i64) as f32;
_1.0.0 = 2_isize & 9223372036854775807_isize;
_5 = true | true;
(*RET) = 316425351834672844978630914172224514885_u128 * 24199407760951001083770039657262564374_u128;
RET = core::ptr::addr_of!((*RET));
_1.2 = (-53230045525994870730181753226381941842_i128) as usize;
RET = core::ptr::addr_of!(_1.1);
_1.0.0 = _1.0.1 as isize;
(*RET) = '\u{3debe}' as u128;
_2 = 313408207_i32 >> _1.0.3;
RET = core::ptr::addr_of!((*RET));
_5 = true;
_5 = !false;
Call(_3 = fn16(Move(RET), _1, _1.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1.0.3 = !34734_u16;
RET = core::ptr::addr_of!(_1.1);
_7 = -6105546842762671941_i64;
_4 = !_1.0.0;
_1.0.1 = _1.0.2 as f32;
_2 = 1590924112_i32 | (-837327574_i32);
_6 = -_4;
_8 = _1.2;
_8 = !_1.2;
_10 = _5;
(*RET) = 23879_i16 as u128;
_1.3 = [_1.0.2];
(*RET) = 942850787_u32 as u128;
_1.0.1 = (-169135915886121027040708174925555151436_i128) as f32;
_13 = 1535087830_u32 as f64;
_7 = !(-7679435942432902248_i64);
_11 = ['\u{f8968}'];
RET = core::ptr::addr_of!((*RET));
_4 = _6;
_1.3 = [_1.0.2];
(*RET) = _1.2 as u128;
_1.2 = _8;
_6 = _1.0.1 as isize;
_1.1 = 1874_i16 as u128;
Goto(bb4)
}
bb4 = {
_6 = _1.0.0 + _4;
_1.0.3 = 112986002016688702139705658270745956327_i128 as u16;
_1.0.2 = 102_u8 << _6;
_4 = (-30_i8) as isize;
_7 = _1.0.2 as i64;
Goto(bb5)
}
bb5 = {
_1.0.3 = 37190_u16;
(*RET) = 302547684616408583370210034960777672608_u128 & 83034080196974044516216561376271182246_u128;
_12 = !_10;
(*RET) = !339392518506841837224975870020123924926_u128;
(*RET) = _7 as u128;
_1.0.2 = _1.2 as u8;
_6 = (*RET) as isize;
_17 = _13;
_6 = !_4;
_1.0.1 = _6 as f32;
_14 = _8 << (*RET);
_1.0.0 = _6;
(*RET) = _1.0.3 as u128;
(*RET) = _1.0.1 as u128;
(*RET) = 329012433754943812753671256492350165421_u128 + 21886145788339319998369114837411011302_u128;
(*RET) = 221261260550990639787111927695781206994_u128 | 179832036744729267961779336573663776597_u128;
_6 = _4;
_4 = 8709621283079981500_u64 as isize;
_1.0.2 = 143_u8 + 237_u8;
_7 = 4050212390111620822_i64 ^ 3833147359138716787_i64;
_1.2 = !_14;
(*RET) = 127328711892076567462200810852186675698_u128 | 117680439579811590395485131077426326852_u128;
(*RET) = 23122241215800158780936164486342041187_u128 >> _1.2;
_1.0.3 = !3061_u16;
_1.2 = _1.1 as usize;
_6 = _12 as isize;
_14 = _1.2 & _1.2;
_1.2 = _14;
Goto(bb6)
}
bb6 = {
(*RET) = 287673174714593508344829140194261235613_u128;
match _1.1 {
287673174714593508344829140194261235613 => bb8,
_ => bb7
}
}
bb7 = {
_6 = _1.0.0 + _4;
_1.0.3 = 112986002016688702139705658270745956327_i128 as u16;
_1.0.2 = 102_u8 << _6;
_4 = (-30_i8) as isize;
_7 = _1.0.2 as i64;
Goto(bb5)
}
bb8 = {
_5 = !_10;
(*RET) = _12 as u128;
_10 = !_5;
RET = core::ptr::addr_of!((*RET));
_15 = _7 as isize;
_8 = _1.2 ^ _1.2;
_13 = _1.2 as f64;
_1.1 = 103256286371404336300542680638279539687_u128;
_14 = _1.2;
RET = core::ptr::addr_of!((*RET));
_1.1 = _7 as u128;
(*RET) = 8948061881685113657_u64 as u128;
_1.0.1 = (-87_i8) as f32;
_17 = _13 - _13;
(*RET) = !120238328143165123973157938466023132717_u128;
_18 = Adt54::Variant0 { fld0: _2 };
SetDiscriminant(_18, 0);
_4 = _1.0.0;
(*RET) = !121109309470223775272012713936374394456_u128;
_14 = _8;
_1.0.1 = 1898689692_u32 as f32;
_1.0.0 = _6;
_20 = 22812_i16 as u64;
_7 = (-6719049499333618583_i64) & (-6883446904701714472_i64);
Goto(bb9)
}
bb9 = {
(*RET) = !310341598674161012906866208518928138946_u128;
_14 = _8;
(*RET) = !282914826585224967969372800745336528924_u128;
(*RET) = _2 as u128;
Goto(bb10)
}
bb10 = {
_1.0.2 = !143_u8;
(*RET) = 263692051375832538036757499959455560945_u128 << _6;
_4 = _15 | _1.0.0;
_1.0.3 = 45983_u16 << (*RET);
_10 = _12;
_10 = !_5;
_1.0.3 = !2814_u16;
_23.3 = -_1.0.1;
_23.2.1 = _23.3 * _1.0.1;
(*RET) = _1.0.3 as u128;
_7 = 4995010024083159282_i64 << _15;
(*RET) = 168763500472602625321734188706858391889_u128 ^ 227941797203320351410684308061561004690_u128;
_23.0 = core::ptr::addr_of!(_1.0.2);
_4 = -_1.0.0;
_23.2.2 = _15 as u8;
Goto(bb11)
}
bb11 = {
_23.2.3 = _1.0.3;
_23.2.2 = !_1.0.2;
_18 = Adt54::Variant2 { fld0: _7 };
_23.1 = [_12,_12,_10,_5,_12,_12,_10,_12];
_6 = _23.2.2 as isize;
_4 = _13 as isize;
_11 = ['\u{b30c9}'];
(*RET) = !25869424274195379070230269469497413602_u128;
_23.3 = _4 as f32;
(*RET) = 238018786348223924729199958079097247015_u128 & 273676975550465002847266955736551844110_u128;
_1.0.3 = !_23.2.3;
_16 = _4 | _4;
Goto(bb12)
}
bb12 = {
_13 = _17 - _17;
_6 = _16;
_24 = '\u{fce63}';
_1.1 = 153527781528256844556716321234106417940_u128;
_1.3 = [_23.2.2];
SetDiscriminant(_18, 0);
_25 = _16 + _6;
_12 = !_5;
_12 = _17 <= _17;
_18 = Adt54::Variant0 { fld0: _2 };
(*RET) = 161732959301315278497673698275819601044_u128 - 131250001128283913656771526440453293762_u128;
_23.0 = core::ptr::addr_of!(_1.0.2);
place!(Field::<i32>(Variant(_18, 0), 0)) = _2 | _2;
_23.3 = 3785785211_u32 as f32;
_10 = _16 > _16;
_1.1 = 9941752213650761965388922619542375689_u128 & 301910364730211848559168608230911173394_u128;
_1.0 = (_25, _23.3, _23.2.2, _23.2.3);
Goto(bb13)
}
bb13 = {
_20 = !18298260907730445290_u64;
_4 = _25 << Field::<i32>(Variant(_18, 0), 0);
_12 = _10;
_22 = _23.2.1;
SetDiscriminant(_18, 1);
place!(Field::<i16>(Variant(_18, 1), 4)) = 7863_i16;
_23.1 = [_12,_10,_10,_10,_10,_12,_10,_10];
match Field::<i16>(Variant(_18, 1), 4) {
0 => bb7,
1 => bb12,
2 => bb3,
3 => bb11,
4 => bb5,
5 => bb8,
7863 => bb15,
_ => bb14
}
}
bb14 = {
_1.0.3 = 37190_u16;
(*RET) = 302547684616408583370210034960777672608_u128 & 83034080196974044516216561376271182246_u128;
_12 = !_10;
(*RET) = !339392518506841837224975870020123924926_u128;
(*RET) = _7 as u128;
_1.0.2 = _1.2 as u8;
_6 = (*RET) as isize;
_17 = _13;
_6 = !_4;
_1.0.1 = _6 as f32;
_14 = _8 << (*RET);
_1.0.0 = _6;
(*RET) = _1.0.3 as u128;
(*RET) = _1.0.1 as u128;
(*RET) = 329012433754943812753671256492350165421_u128 + 21886145788339319998369114837411011302_u128;
(*RET) = 221261260550990639787111927695781206994_u128 | 179832036744729267961779336573663776597_u128;
_6 = _4;
_4 = 8709621283079981500_u64 as isize;
_1.0.2 = 143_u8 + 237_u8;
_7 = 4050212390111620822_i64 ^ 3833147359138716787_i64;
_1.2 = !_14;
(*RET) = 127328711892076567462200810852186675698_u128 | 117680439579811590395485131077426326852_u128;
(*RET) = 23122241215800158780936164486342041187_u128 >> _1.2;
_1.0.3 = !3061_u16;
_1.2 = _1.1 as usize;
_6 = _12 as isize;
_14 = _1.2 & _1.2;
_1.2 = _14;
Goto(bb6)
}
bb15 = {
_15 = !_16;
place!(Field::<[isize; 4]>(Variant(_18, 1), 6)) = [_16,_6,_16,_4];
_23.2.0 = _22 as isize;
_23.2 = (_6, _23.3, _1.0.2, _1.0.3);
place!(Field::<i8>(Variant(_18, 1), 3)) = 120_i8 & (-60_i8);
_25 = _16;
_8 = !_14;
_1.0 = (_16, _23.2.1, _23.2.2, _23.2.3);
place!(Field::<u128>(Variant(_18, 1), 0)) = !(*RET);
(*RET) = Field::<u128>(Variant(_18, 1), 0);
_6 = _16;
_4 = _15;
_23.1 = [_10,_10,_12,_12,_10,_12,_10,_12];
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(11_usize, 2_usize, Move(_2), 8_usize, Move(_8), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(11_usize, 24_usize, Move(_24), 7_usize, Move(_7), 4_usize, Move(_4), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u16,mut _2: *const u128) -> isize {
mir! {
type RET = isize;
let _3: [char; 3];
let _4: f64;
let _5: f64;
let _6: (f32,);
let _7: u8;
let _8: usize;
let _9: &'static *const i16;
let _10: i128;
let _11: f32;
let _12: *const u8;
let _13: [u32; 4];
let _14: i128;
let _15: Adt42;
let _16: isize;
let _17: isize;
let _18: u128;
let _19: &'static (&'static u128, [bool; 7]);
let _20: ([u32; 4],);
let _21: u8;
let _22: ([u32; 4],);
let _23: isize;
let _24: isize;
let _25: (i8, [u32; 4], *const u8);
let _26: f64;
let _27: bool;
let _28: [char; 3];
let _29: &'static (i128, i64, i8, i8);
let _30: Adt62;
let _31: i32;
let _32: bool;
let _33: &'static u128;
let _34: ();
let _35: ();
{
RET = 1080646222_u32 as isize;
RET = 3769843495255942478799271003212895098_i128 as isize;
RET = false as isize;
RET = (-12_isize) * (-9223372036854775808_isize);
RET = !126_isize;
RET = true as isize;
RET = (-9223372036854775808_isize);
_1 = !63585_u16;
_3 = ['\u{61547}','\u{321a0}','\u{9736}'];
Call(_4 = fn13(_3, _3, Move(_2), RET, RET, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_6.0 = _1 as f32;
_5 = 302229672572912443624470109415385928259_u128 as f64;
_7 = 131_u8 + 206_u8;
_8 = _1 as usize;
_5 = _1 as f64;
_6.0 = _4 as f32;
_6.0 = 2_i8 as f32;
_7 = 121_u8;
_4 = _5 * _5;
RET = -9223372036854775807_isize;
RET = !51_isize;
Call(_2 = fn15(_7, _3, _3, _6.0, _3, RET, _3, _4, _7, _6, RET, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = 55_isize;
_4 = 6873736273828731477_u64 as f64;
RET = 9223372036854775807_isize - 9223372036854775807_isize;
_1 = 39741_u16;
_6.0 = 141066446878389209357347668462267572485_u128 as f32;
_11 = _6.0;
RET = _8 as isize;
_10 = -(-9094748371666560981127764056927488938_i128);
_8 = _5 as usize;
_7 = 195_u8;
_8 = !2_usize;
Goto(bb4)
}
bb4 = {
_4 = _5;
_14 = !_10;
_11 = _6.0 * _6.0;
_1 = 7244_u16;
_1 = (-679032033683143575_i64) as u16;
_7 = _1 as u8;
_10 = _14;
RET = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_7 = _1 as u8;
_14 = _10;
_5 = _4;
_12 = core::ptr::addr_of!(_7);
Goto(bb5)
}
bb5 = {
_3 = ['\u{a224a}','\u{962fa}','\u{ecdc9}'];
RET = 31_isize | (-9223372036854775808_isize);
_11 = -_6.0;
_4 = -_5;
_14 = _10 << RET;
_16 = false as isize;
_2 = core::ptr::addr_of!(_18);
_13 = [706589796_u32,1524641610_u32,1625114929_u32,3145827711_u32];
_15 = Adt42::Variant1 { fld0: Move(_12) };
Call((*_2) = core::intrinsics::transmute(_13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = 28021_i16 as u8;
RET = _5 as isize;
_12 = Move(Field::<*const u8>(Variant(_15, 1), 0));
_23 = RET | RET;
_4 = -_5;
_23 = 4248783848_u32 as isize;
_8 = 8707934655384886985_usize;
_25.0 = (-103_i8) >> (*_2);
_24 = !_23;
_22.0 = _13;
_25 = ((-14_i8), _13, Move(_12));
_21 = true as u8;
_22.0 = _13;
_6.0 = -_11;
Call(_20.0 = core::intrinsics::transmute(_22.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_25.0 = 90_i8;
_26 = _4 + _4;
_25.0 = (-126_i8) << _18;
_14 = _16 as i128;
_28 = ['\u{186b0}','\u{8f703}','\u{dd865}'];
_26 = _5;
_25.0 = 29_i8;
_18 = 170794909716483632828411486178388449177_u128;
match (*_2) {
0 => bb6,
1 => bb2,
2 => bb5,
3 => bb8,
4 => bb9,
5 => bb10,
170794909716483632828411486178388449177 => bb12,
_ => bb11
}
}
bb8 = {
_7 = 28021_i16 as u8;
RET = _5 as isize;
_12 = Move(Field::<*const u8>(Variant(_15, 1), 0));
_23 = RET | RET;
_4 = -_5;
_23 = 4248783848_u32 as isize;
_8 = 8707934655384886985_usize;
_25.0 = (-103_i8) >> (*_2);
_24 = !_23;
_22.0 = _13;
_25 = ((-14_i8), _13, Move(_12));
_21 = true as u8;
_22.0 = _13;
_6.0 = -_11;
Call(_20.0 = core::intrinsics::transmute(_22.0), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_3 = ['\u{a224a}','\u{962fa}','\u{ecdc9}'];
RET = 31_isize | (-9223372036854775808_isize);
_11 = -_6.0;
_4 = -_5;
_14 = _10 << RET;
_16 = false as isize;
_2 = core::ptr::addr_of!(_18);
_13 = [706589796_u32,1524641610_u32,1625114929_u32,3145827711_u32];
_15 = Adt42::Variant1 { fld0: Move(_12) };
Call((*_2) = core::intrinsics::transmute(_13), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_6.0 = _1 as f32;
_5 = 302229672572912443624470109415385928259_u128 as f64;
_7 = 131_u8 + 206_u8;
_8 = _1 as usize;
_5 = _1 as f64;
_6.0 = _4 as f32;
_6.0 = 2_i8 as f32;
_7 = 121_u8;
_4 = _5 * _5;
RET = -9223372036854775807_isize;
RET = !51_isize;
Call(_2 = fn15(_7, _3, _3, _6.0, _3, RET, _3, _4, _7, _6, RET, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
RET = 55_isize;
_4 = 6873736273828731477_u64 as f64;
RET = 9223372036854775807_isize - 9223372036854775807_isize;
_1 = 39741_u16;
_6.0 = 141066446878389209357347668462267572485_u128 as f32;
_11 = _6.0;
RET = _8 as isize;
_10 = -(-9094748371666560981127764056927488938_i128);
_8 = _5 as usize;
_7 = 195_u8;
_8 = !2_usize;
Goto(bb4)
}
bb12 = {
_6.0 = -_11;
_27 = !true;
_28 = _3;
_22 = (_20.0,);
_17 = !_24;
_17 = _27 as isize;
_10 = _14 * _14;
_5 = -_4;
_18 = !321590288844775159622349950814568686665_u128;
_12 = core::ptr::addr_of!(_7);
_15 = Adt42::Variant1 { fld0: Move(_25.2) };
RET = _23;
(*_2) = 153508953338569242968036017903683979338_u128 + 153039629964634734105122229271912548872_u128;
_21 = (*_12) | _7;
_6 = (_11,);
_8 = _27 as usize;
place!(Field::<*const u8>(Variant(_15, 1), 0)) = core::ptr::addr_of!(_7);
_11 = _6.0 * _6.0;
_25.2 = Move(Field::<*const u8>(Variant(_15, 1), 0));
(*_2) = 224109647322730436376916287575700764029_u128;
(*_12) = !_21;
Goto(bb13)
}
bb13 = {
(*_2) = 98540660252367040231738689051868429526_u128 ^ 226823883080731615137781919513241704556_u128;
(*_2) = 146739003908308708325070062748173358342_u128 ^ 59264086525789272021441282061326489374_u128;
Goto(bb14)
}
bb14 = {
_7 = !_21;
_24 = !_16;
_25.1 = _20.0;
_6 = (_11,);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(12_usize, 28_usize, Move(_28), 27_usize, Move(_27), 3_usize, Move(_3), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(12_usize, 7_usize, Move(_7), 20_usize, Move(_20), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [char; 3],mut _2: [char; 3],mut _3: *const u128,mut _4: isize,mut _5: isize,mut _6: u16,mut _7: u16) -> f64 {
mir! {
type RET = f64;
let _8: char;
let _9: ([u32; 4],);
let _10: bool;
let _11: (*mut [u8; 1], (isize, f32, u8, u16), *mut i16, Adt62);
let _12: &'static [u8; 4];
let _13: char;
let _14: [u8; 4];
let _15: char;
let _16: ();
let _17: ();
{
_4 = _5;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb7,
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
_6 = 112_i8 as u16;
_7 = 462919230_u32 as u16;
_5 = 140430955429736960146355964916481134416_u128 as isize;
_6 = _7;
_1 = _2;
RET = 6_usize as f64;
_1 = ['\u{b062}','\u{8911b}','\u{56da2}'];
RET = 175_u8 as f64;
_2 = ['\u{19591}','\u{1cef1}','\u{d4ddc}'];
_8 = '\u{b5eb8}';
_6 = _7;
_5 = !_4;
_6 = !_7;
_5 = _4 & _4;
_9.0 = [1142155880_u32,4068070624_u32,4149472701_u32,2038562739_u32];
match _4 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
4 => bb8,
340282366920938463454151235394913435648 => bb10,
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
_4 = 34301938925606403537683274401276230797_i128 as isize;
RET = 4724760819893795881_usize as f64;
_6 = !_7;
_6 = _7 & _7;
_4 = _5 | _5;
Goto(bb11)
}
bb11 = {
_4 = !_5;
_5 = _4 & _4;
_4 = _5 << _6;
_8 = '\u{44dad}';
Call(_6 = fn14(_9.0, _1, _9.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2 = [_8,_8,_8];
_10 = false;
_9.0 = [443471503_u32,3608855142_u32,3785311279_u32,626940805_u32];
_10 = _4 == _4;
_10 = _6 <= _6;
RET = 4169921994272896054_u64 as f64;
_6 = RET as u16;
_6 = _7;
RET = 751838456_i32 as f64;
_9.0 = [1212377065_u32,3596784780_u32,2338498410_u32,3967824109_u32];
_8 = '\u{e3e6c}';
_9.0 = [4037484632_u32,1525553304_u32,3512615738_u32,2549675783_u32];
_11.1.0 = 5076162220940142181_usize as isize;
_11.1.0 = _5 >> _4;
RET = 121376074175059042029093078024939528132_u128 as f64;
_1 = _2;
_8 = '\u{9ca20}';
_11.1.2 = !2_u8;
_13 = _8;
_12 = &_14;
_11.1.1 = 3618309094035303982_usize as f32;
Goto(bb13)
}
bb13 = {
_8 = _13;
_6 = _7;
Goto(bb14)
}
bb14 = {
_5 = !_4;
_12 = &(*_12);
_7 = (-50902120201134840658470106475337463358_i128) as u16;
_11.1.3 = _7 >> _6;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(13_usize, 1_usize, Move(_1), 5_usize, Move(_5), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_16 = dump_var(13_usize, 9_usize, Move(_9), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [u32; 4],mut _2: [char; 3],mut _3: [u32; 4]) -> u16 {
mir! {
type RET = u16;
let _4: i128;
let _5: isize;
let _6: f32;
let _7: u16;
let _8: i128;
let _9: &'static [u8; 4];
let _10: Adt23;
let _11: i64;
let _12: Adt75;
let _13: (char, u32, u32);
let _14: ();
let _15: ();
{
_2 = ['\u{2fa26}','\u{7141c}','\u{72492}'];
RET = 52235_u16;
_1 = [660358613_u32,986634721_u32,3118891769_u32,2819383159_u32];
RET = 17677_u16 ^ 59890_u16;
RET = 18042_u16 << 4_usize;
_3 = [2703115306_u32,899717283_u32,3647929374_u32,814282852_u32];
RET = (-3706_i16) as u16;
_1 = _3;
RET = !2594_u16;
RET = 21839_u16;
_4 = 95621541665288236480917261273468343864_i128;
_2 = ['\u{2a0e3}','\u{679e6}','\u{b10a4}'];
_2 = ['\u{de837}','\u{b8bc9}','\u{5d56e}'];
_6 = 275342024805895792979622453688686387353_u128 as f32;
RET = 27_u8 as u16;
_2 = ['\u{104fb6}','\u{10c5c7}','\u{2261e}'];
_4 = 106505190639607571552649630512549004429_i128;
_7 = RET;
_5 = !9223372036854775807_isize;
_4 = 71508900111421843283231823474812963799_i128;
RET = !_7;
_1 = [1241945998_u32,2976089503_u32,1158016322_u32,788984371_u32];
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
71508900111421843283231823474812963799 => bb8,
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
_5 = -9223372036854775807_isize;
RET = _7;
_8 = _4 << _7;
_8 = !_4;
_8 = 15133428388357218443_u64 as i128;
_8 = 46_i8 as i128;
match _4 {
0 => bb5,
1 => bb2,
2 => bb4,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
71508900111421843283231823474812963799 => bb14,
_ => bb13
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
Return()
}
bb13 = {
Return()
}
bb14 = {
_6 = 105132049118577502908207444445887526008_u128 as f32;
RET = _7 ^ _7;
_7 = !RET;
RET = _7;
_10.fld0 = [224_u8];
RET = _7;
_11 = 6853212525685991267_i64;
RET = !_7;
_13.2 = 888037925_u32 << _7;
_11 = 4753214923618007978_i64;
_13.0 = '\u{62fe2}';
_13.2 = 11636_i16 as u32;
_11 = _6 as i64;
_2 = [_13.0,_13.0,_13.0];
_4 = !_8;
Goto(bb15)
}
bb15 = {
Call(_14 = dump_var(14_usize, 7_usize, Move(_7), 11_usize, Move(_11), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u8,mut _2: [char; 3],mut _3: [char; 3],mut _4: f32,mut _5: [char; 3],mut _6: isize,mut _7: [char; 3],mut _8: f64,mut _9: u8,mut _10: (f32,),mut _11: isize,mut _12: isize) -> *const u128 {
mir! {
type RET = *const u128;
let _13: [u8; 1];
let _14: char;
let _15: *const u8;
let _16: isize;
let _17: (u32, u8, i128);
let _18: Adt62;
let _19: *const u128;
let _20: i64;
let _21: ([u32; 4],);
let _22: &'static (i128, i64, i8, i8);
let _23: f32;
let _24: i64;
let _25: [isize; 4];
let _26: [isize; 4];
let _27: Adt75;
let _28: isize;
let _29: ([u8; 4], (Adt42, Adt54, *mut Adt23, char), char);
let _30: f64;
let _31: char;
let _32: (u32, u8, i128);
let _33: [i32; 6];
let _34: &'static i8;
let _35: isize;
let _36: ();
let _37: ();
{
_11 = _6;
_12 = _6;
_9 = !_1;
_10.0 = 39362_u16 as f32;
_14 = '\u{68888}';
_15 = core::ptr::addr_of!(_9);
_14 = '\u{66b99}';
_9 = _1;
_6 = _12;
_17.1 = (*_15) % (*_15);
_17.2 = (-117127929212979936327835359569208582415_i128);
_17.1 = !(*_15);
_6 = _11;
_20 = (-8927771446585006685_i64) << _6;
_14 = '\u{60b68}';
_17.0 = false as u32;
_23 = _4;
_5 = _2;
match (*_15) {
0 => bb1,
1 => bb2,
2 => bb3,
121 => bb5,
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
_11 = _6 & _6;
_7 = [_14,_14,_14];
_17.0 = _8 as u32;
_9 = (-26698_i16) as u8;
_11 = 83844784586476686588248268907024407738_u128 as isize;
match _1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
121 => bb14,
_ => bb13
}
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
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_23 = 3_usize as f32;
_10.0 = -_4;
_7 = [_14,_14,_14];
_2 = _3;
_8 = 3781_i16 as f64;
_17.2 = -13061491247038490734125559371083648982_i128;
_17.2 = (-105705868911157936190443150698939368188_i128);
_13 = [(*_15)];
_24 = _20;
_24 = _20;
_12 = _11;
_17.1 = _9 * (*_15);
_24 = !_20;
_8 = _23 as f64;
_10 = (_23,);
_17.2 = 152019759091415651697780453340955474357_i128 | (-9251730346747674613974091629649470425_i128);
_13 = [(*_15)];
_13 = [_1];
_10.0 = _17.2 as f32;
_13 = [_17.1];
match _1 {
0 => bb6,
1 => bb2,
2 => bb15,
3 => bb16,
121 => bb18,
_ => bb17
}
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
_21.0 = [_17.0,_17.0,_17.0,_17.0];
_24 = _20;
match _1 {
0 => bb14,
1 => bb17,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
121 => bb25,
_ => bb24
}
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
_23 = 3_usize as f32;
_10.0 = -_4;
_7 = [_14,_14,_14];
_2 = _3;
_8 = 3781_i16 as f64;
_17.2 = -13061491247038490734125559371083648982_i128;
_17.2 = (-105705868911157936190443150698939368188_i128);
_13 = [(*_15)];
_24 = _20;
_24 = _20;
_12 = _11;
_17.1 = _9 * (*_15);
_24 = !_20;
_8 = _23 as f64;
_10 = (_23,);
_17.2 = 152019759091415651697780453340955474357_i128 | (-9251730346747674613974091629649470425_i128);
_13 = [(*_15)];
_13 = [_1];
_10.0 = _17.2 as f32;
_13 = [_17.1];
match _1 {
0 => bb6,
1 => bb2,
2 => bb15,
3 => bb16,
121 => bb18,
_ => bb17
}
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_25 = [_6,_11,_12,_12];
_1 = _9;
_12 = _11;
_10 = (_23,);
_10.0 = _23 * _23;
_25 = [_6,_11,_6,_11];
_10.0 = _23;
_17.2 = -(-61697372488804665881919555817428812015_i128);
_20 = _24;
_26 = [_12,_6,_12,_12];
_14 = '\u{1382b}';
_24 = _20 | _20;
_17.2 = 2319942079532127213544782258213028769_i128 + (-75593235419963601527289300981133625898_i128);
_24 = -_20;
_20 = -_24;
_16 = _20 as isize;
_26 = _25;
_9 = _16 as u8;
_4 = _23 - _23;
_5 = _3;
_21.0 = [_17.0,_17.0,_17.0,_17.0];
_23 = -_4;
_3 = _2;
(*_15) = _1;
Goto(bb26)
}
bb26 = {
_11 = 100_i8 as isize;
_17.0 = 6_usize as u32;
_17.2 = _23 as i128;
_5 = [_14,_14,_14];
_7 = [_14,_14,_14];
_5 = [_14,_14,_14];
Goto(bb27)
}
bb27 = {
_6 = (*_15) as isize;
_28 = _12;
_17.2 = -(-66828871477434795949458102516159020172_i128);
_17.2 = 100726278409636208045254423158593556438_i128;
_13 = [(*_15)];
_5 = [_14,_14,_14];
_29.1.0 = Adt42::Variant1 { fld0: Move(_15) };
_3 = _7;
_13 = [_17.1];
_3 = [_14,_14,_14];
_29.2 = _14;
_28 = _12;
_9 = _17.1 - _17.1;
_12 = _16 << _6;
_11 = 17646055230132486589_u64 as isize;
_16 = _14 as isize;
_7 = _2;
_15 = core::ptr::addr_of!(_17.1);
match _17.2 {
0 => bb11,
1 => bb24,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
100726278409636208045254423158593556438 => bb33,
_ => bb32
}
}
bb28 = {
_23 = 3_usize as f32;
_10.0 = -_4;
_7 = [_14,_14,_14];
_2 = _3;
_8 = 3781_i16 as f64;
_17.2 = -13061491247038490734125559371083648982_i128;
_17.2 = (-105705868911157936190443150698939368188_i128);
_13 = [(*_15)];
_24 = _20;
_24 = _20;
_12 = _11;
_17.1 = _9 * (*_15);
_24 = !_20;
_8 = _23 as f64;
_10 = (_23,);
_17.2 = 152019759091415651697780453340955474357_i128 | (-9251730346747674613974091629649470425_i128);
_13 = [(*_15)];
_13 = [_1];
_10.0 = _17.2 as f32;
_13 = [_17.1];
match _1 {
0 => bb6,
1 => bb2,
2 => bb15,
3 => bb16,
121 => bb18,
_ => bb17
}
}
bb29 = {
_11 = _6 & _6;
_7 = [_14,_14,_14];
_17.0 = _8 as u32;
_9 = (-26698_i16) as u8;
_11 = 83844784586476686588248268907024407738_u128 as isize;
match _1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
121 => bb14,
_ => bb13
}
}
bb30 = {
_21.0 = [_17.0,_17.0,_17.0,_17.0];
_24 = _20;
match _1 {
0 => bb14,
1 => bb17,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
121 => bb25,
_ => bb24
}
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
_17.1 = _9 | _9;
_4 = _10.0;
place!(Field::<*const u8>(Variant(_29.1.0, 1), 0)) = core::ptr::addr_of!((*_15));
SetDiscriminant(_29.1.0, 2);
place!(Field::<i64>(Variant(_29.1.0, 2), 0)) = _20;
(*_15) = _9;
place!(Field::<i16>(Variant(_29.1.0, 2), 1)) = 38972_u16 as i16;
_28 = _11 + _12;
_23 = -_4;
_17 = (2173811385_u32, _1, 151213692182236230195883103394032305364_i128);
_13 = [_1];
_28 = _12 >> _24;
place!(Field::<[u8; 1]>(Variant(_29.1.0, 2), 2)) = _13;
_20 = -_24;
(*_15) = _17.2 as u8;
_29.1.3 = _14;
(*_15) = _9 << _9;
_9 = _17.1 >> _17.1;
place!(Field::<[u8; 1]>(Variant(_29.1.0, 2), 2)) = [_9];
_17 = (2131052556_u32, _9, 98258654636737885658687940526859256486_i128);
match _17.2 {
0 => bb21,
1 => bb34,
2 => bb35,
3 => bb36,
4 => bb37,
5 => bb38,
98258654636737885658687940526859256486 => bb40,
_ => bb39
}
}
bb34 = {
_23 = 3_usize as f32;
_10.0 = -_4;
_7 = [_14,_14,_14];
_2 = _3;
_8 = 3781_i16 as f64;
_17.2 = -13061491247038490734125559371083648982_i128;
_17.2 = (-105705868911157936190443150698939368188_i128);
_13 = [(*_15)];
_24 = _20;
_24 = _20;
_12 = _11;
_17.1 = _9 * (*_15);
_24 = !_20;
_8 = _23 as f64;
_10 = (_23,);
_17.2 = 152019759091415651697780453340955474357_i128 | (-9251730346747674613974091629649470425_i128);
_13 = [(*_15)];
_13 = [_1];
_10.0 = _17.2 as f32;
_13 = [_17.1];
match _1 {
0 => bb6,
1 => bb2,
2 => bb15,
3 => bb16,
121 => bb18,
_ => bb17
}
}
bb35 = {
Return()
}
bb36 = {
_25 = [_6,_11,_12,_12];
_1 = _9;
_12 = _11;
_10 = (_23,);
_10.0 = _23 * _23;
_25 = [_6,_11,_6,_11];
_10.0 = _23;
_17.2 = -(-61697372488804665881919555817428812015_i128);
_20 = _24;
_26 = [_12,_6,_12,_12];
_14 = '\u{1382b}';
_24 = _20 | _20;
_17.2 = 2319942079532127213544782258213028769_i128 + (-75593235419963601527289300981133625898_i128);
_24 = -_20;
_20 = -_24;
_16 = _20 as isize;
_26 = _25;
_9 = _16 as u8;
_4 = _23 - _23;
_5 = _3;
_21.0 = [_17.0,_17.0,_17.0,_17.0];
_23 = -_4;
_3 = _2;
(*_15) = _1;
Goto(bb26)
}
bb37 = {
Return()
}
bb38 = {
Return()
}
bb39 = {
_6 = (*_15) as isize;
_28 = _12;
_17.2 = -(-66828871477434795949458102516159020172_i128);
_17.2 = 100726278409636208045254423158593556438_i128;
_13 = [(*_15)];
_5 = [_14,_14,_14];
_29.1.0 = Adt42::Variant1 { fld0: Move(_15) };
_3 = _7;
_13 = [_17.1];
_3 = [_14,_14,_14];
_29.2 = _14;
_28 = _12;
_9 = _17.1 - _17.1;
_12 = _16 << _6;
_11 = 17646055230132486589_u64 as isize;
_16 = _14 as isize;
_7 = _2;
_15 = core::ptr::addr_of!(_17.1);
match _17.2 {
0 => bb11,
1 => bb24,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
100726278409636208045254423158593556438 => bb33,
_ => bb32
}
}
bb40 = {
_30 = _8;
_32.1 = (*_15) * (*_15);
_16 = -_12;
_29.1.1 = Adt54::Variant2 { fld0: Field::<i64>(Variant(_29.1.0, 2), 0) };
_26 = [_12,_28,_12,_28];
_17.1 = _9 - _32.1;
_10 = (_4,);
(*_15) = !_32.1;
_3 = [_29.2,_29.1.3,_29.2];
_14 = _29.2;
_9 = !_17.1;
_29.2 = _14;
_5 = [_14,_29.1.3,_29.2];
_32.0 = !_17.0;
_5 = [_29.2,_29.1.3,_14];
SetDiscriminant(_29.1.0, 0);
SetDiscriminant(_29.1.1, 1);
_32 = (_17.0, (*_15), _17.2);
_8 = _30 - _30;
_16 = _11 * _28;
_29.1.0 = Adt42::Variant2 { fld0: _20,fld1: 17917_i16,fld2: _13 };
RET = core::ptr::addr_of!(place!(Field::<u128>(Variant(_29.1.1, 1), 0)));
(*RET) = false as u128;
place!(Field::<i64>(Variant(_29.1.0, 2), 0)) = _24;
Goto(bb41)
}
bb41 = {
Call(_36 = dump_var(15_usize, 2_usize, Move(_2), 14_usize, Move(_14), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_36 = dump_var(15_usize, 21_usize, Move(_21), 5_usize, Move(_5), 16_usize, Move(_16), 9_usize, Move(_9)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_36 = dump_var(15_usize, 25_usize, Move(_25), 28_usize, Move(_28), 37_usize, _37, 37_usize, _37), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *const u128,mut _2: ((isize, f32, u8, u16), u128, usize, [u8; 1]),mut _3: [u8; 1]) -> *const [bool; 7] {
mir! {
type RET = *const [bool; 7];
let _4: ((isize, f32, u8, u16), u128, usize, [u8; 1]);
let _5: char;
let _6: (i8, [u32; 4], *const u8);
let _7: Adt23;
let _8: u64;
let _9: *mut &'static i8;
let _10: Adt54;
let _11: bool;
let _12: [u8; 1];
let _13: i128;
let _14: char;
let _15: [usize; 2];
let _16: *const [i32; 6];
let _17: &'static (&'static u128, [bool; 7]);
let _18: isize;
let _19: *const bool;
let _20: *mut [u8; 1];
let _21: isize;
let _22: [isize; 7];
let _23: bool;
let _24: f64;
let _25: *const *const [bool; 7];
let _26: *const u128;
let _27: i32;
let _28: u16;
let _29: f64;
let _30: u16;
let _31: u64;
let _32: f64;
let _33: u64;
let _34: char;
let _35: bool;
let _36: *mut i16;
let _37: i32;
let _38: i128;
let _39: [char; 3];
let _40: *mut Adt23;
let _41: [i128; 4];
let _42: i8;
let _43: bool;
let _44: f32;
let _45: isize;
let _46: (*mut [u8; 1], (isize, f32, u8, u16), *mut i16, Adt62);
let _47: *mut [u8; 1];
let _48: isize;
let _49: isize;
let _50: &'static (i128, i64, i8, i8);
let _51: (char, u32, u32);
let _52: f64;
let _53: usize;
let _54: f64;
let _55: &'static [u8; 4];
let _56: &'static f64;
let _57: f32;
let _58: *mut [u8; 1];
let _59: [i128; 4];
let _60: isize;
let _61: &'static u128;
let _62: Adt78;
let _63: (&'static u128, [bool; 7]);
let _64: f32;
let _65: f64;
let _66: Adt23;
let _67: [i32; 6];
let _68: Adt75;
let _69: ();
let _70: ();
{
_2.0.0 = (-9223372036854775808_isize);
_2.2 = (-1165226741_i32) as usize;
_2.0.1 = (-9650999882176140491661864999069405026_i128) as f32;
_1 = core::ptr::addr_of!(_4.1);
match _2.0.0 {
340282366920938463454151235394913435648 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2.3 = [_2.0.2];
_4.0.1 = _2.0.1;
_4.1 = !_2.1;
_5 = '\u{5e3c6}';
_1 = core::ptr::addr_of!((*_1));
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
_4 = _2;
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
(*_1) = !_2.1;
Goto(bb3)
}
bb3 = {
_2.3 = [_4.0.2];
_2.0.2 = _4.0.2 & _4.0.2;
_6.2 = core::ptr::addr_of!(_2.0.2);
_1 = core::ptr::addr_of!(_4.1);
_2.1 = !_4.1;
_7 = Adt23 { fld0: _2.3 };
_1 = core::ptr::addr_of!((*_1));
_4.1 = 14160069353472956506_u64 as u128;
_8 = 15982674616262352226_u64 & 14379823050358220902_u64;
_2.2 = _4.2;
_4.1 = _2.1 ^ _2.1;
_4.3 = [_2.0.2];
_6.0 = (-55_i8) | (-95_i8);
_4.2 = !_2.2;
_6.2 = core::ptr::addr_of!(_2.0.2);
_4.0.3 = _2.0.3;
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
_4.0.3 = (-1581884393_i32) as u16;
_6.0 = -(-42_i8);
_4.0 = _2.0;
match _2.0.0 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb4 = {
_2.3 = [_2.0.2];
_4.0.1 = _2.0.1;
_4.1 = !_2.1;
_5 = '\u{5e3c6}';
_1 = core::ptr::addr_of!((*_1));
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
_4 = _2;
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
(*_1) = !_2.1;
Goto(bb3)
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
_6.1 = [2358399620_u32,2634479854_u32,963398275_u32,2630492367_u32];
_7.fld0 = _4.3;
_6.1 = [702407535_u32,3172279565_u32,2893740724_u32,4250658736_u32];
_4.2 = _2.2 * _2.2;
_2.2 = _4.2 * _4.2;
Goto(bb9)
}
bb9 = {
_2.0.1 = _4.0.1;
Goto(bb10)
}
bb10 = {
_12 = [_2.0.2];
_2.0.2 = !_4.0.2;
_7 = Adt23 { fld0: _3 };
(*_1) = false as u128;
_2.0.1 = _2.1 as f32;
_2.0.2 = _4.0.2 - _4.0.2;
_14 = _5;
_13 = 15850_i16 as i128;
_13 = 26192847238065757713255270832416534010_i128;
_5 = _14;
_15 = [_4.2,_4.2];
_7 = Adt23 { fld0: _2.3 };
_10 = Adt54::Variant2 { fld0: 8354242204984672967_i64 };
_12 = [_2.0.2];
_2.0.3 = _4.0.3;
_6.0 = _5 as i8;
_4.1 = !_2.1;
(*_1) = _2.1 | _2.1;
match _2.0.0 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb11 = {
_2.0.1 = _4.0.1;
Goto(bb10)
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
Return()
}
bb16 = {
_2.3 = [_2.0.2];
_4.0.1 = _2.0.1;
_4.1 = !_2.1;
_5 = '\u{5e3c6}';
_1 = core::ptr::addr_of!((*_1));
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
_4 = _2;
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
(*_1) = !_2.1;
Goto(bb3)
}
bb17 = {
_2.3 = [_4.0.2];
_2.0.2 = _4.0.2 & _4.0.2;
_6.2 = core::ptr::addr_of!(_2.0.2);
_1 = core::ptr::addr_of!(_4.1);
_2.1 = !_4.1;
_7 = Adt23 { fld0: _2.3 };
_1 = core::ptr::addr_of!((*_1));
_4.1 = 14160069353472956506_u64 as u128;
_8 = 15982674616262352226_u64 & 14379823050358220902_u64;
_2.2 = _4.2;
_4.1 = _2.1 ^ _2.1;
_4.3 = [_2.0.2];
_6.0 = (-55_i8) | (-95_i8);
_4.2 = !_2.2;
_6.2 = core::ptr::addr_of!(_2.0.2);
_4.0.3 = _2.0.3;
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
_4.0.3 = (-1581884393_i32) as u16;
_6.0 = -(-42_i8);
_4.0 = _2.0;
match _2.0.0 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb18 = {
_2.3 = [_2.0.2];
_4.0.1 = _2.0.1;
_4.1 = !_2.1;
_5 = '\u{5e3c6}';
_1 = core::ptr::addr_of!((*_1));
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
_4 = _2;
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _2.0.3);
(*_1) = !_2.1;
Goto(bb3)
}
bb19 = {
_13 = (-122141435876828594471573110797741475583_i128);
_11 = true;
_2.1 = (*_1);
_2.0.1 = 4850_i16 as f32;
(*_1) = _2.1 >> _2.2;
_7 = Adt23 { fld0: _2.3 };
_19 = core::ptr::addr_of!(_11);
_21 = -_4.0.0;
_2.0.0 = _21 >> _2.2;
_2.0.0 = 1025087506_u32 as isize;
place!(Field::<i64>(Variant(_10, 2), 0)) = _4.0.0 as i64;
_2.0 = (_4.0.0, _4.0.1, _4.0.2, _4.0.3);
_20 = core::ptr::addr_of_mut!(_4.3);
_18 = -_21;
_15 = [_2.2,_2.2];
(*_19) = false;
_2.2 = _4.2;
_20 = core::ptr::addr_of_mut!(_3);
_20 = core::ptr::addr_of_mut!((*_20));
(*_20) = [_2.0.2];
Goto(bb20)
}
bb20 = {
_4.0.0 = _8 as isize;
_22 = [_21,_21,_4.0.0,_21,_4.0.0,_21,_18];
SetDiscriminant(_10, 2);
_11 = !true;
_4 = (_2.0, _2.1, _2.2, (*_20));
_18 = -_21;
_2.2 = !_4.2;
_1 = core::ptr::addr_of!((*_1));
_4.2 = _6.0 as usize;
_12 = [_4.0.2];
_2.0 = (_21, _4.0.1, _4.0.2, _4.0.3);
(*_19) = false;
_2.1 = _4.1;
_4.2 = _8 as usize;
(*_19) = false;
(*_19) = _4.0.2 != _2.0.2;
_8 = 17105455487942068526_u64 & 10574974842152615546_u64;
_6.0 = (-91_i8);
_20 = core::ptr::addr_of_mut!(_4.3);
_4 = (_2.0, _2.1, _2.2, _12);
Goto(bb21)
}
bb21 = {
_7.fld0 = (*_20);
_4.3 = [_2.0.2];
_4.0.2 = _2.0.2;
_2.0.1 = _4.0.1 + _4.0.1;
_2.1 = (*_1);
_25 = core::ptr::addr_of!(RET);
_28 = (-770351985_i32) as u16;
_20 = core::ptr::addr_of_mut!(_3);
_1 = core::ptr::addr_of!(_2.1);
_22 = [_2.0.0,_21,_21,_21,_18,_2.0.0,_21];
_32 = (-6669061012002509239_i64) as f64;
_20 = core::ptr::addr_of_mut!(_12);
_29 = -_32;
_6.1 = [945346793_u32,2056380738_u32,2560876922_u32,407160508_u32];
_34 = _14;
_30 = _4.0.2 as u16;
_2.0.3 = _30 >> (*_1);
_15 = [_4.2,_4.2];
_3 = (*_20);
Goto(bb22)
}
bb22 = {
_26 = Move(_1);
_2.0.1 = -_4.0.1;
_2.0.1 = (-382967670_i32) as f32;
_15 = [_4.2,_4.2];
Goto(bb23)
}
bb23 = {
_2.0 = (_21, _4.0.1, _4.0.2, _4.0.3);
_24 = _32 * _29;
_41 = [_13,_13,_13,_13];
_12 = [_4.0.2];
_4.0 = (_2.0.0, _2.0.1, _2.0.2, _30);
_4.0.3 = _2.0.3 - _2.0.3;
_23 = !(*_19);
_19 = core::ptr::addr_of!(_43);
_4.0.1 = _2.0.1;
_27 = _24 as i32;
_6.2 = core::ptr::addr_of!(_2.0.2);
_45 = _18;
_39 = [_5,_5,_14];
_1 = Move(_26);
_2 = _4;
_47 = Move(_20);
_46.3 = Adt62::Variant0 { fld0: Move(_6),fld1: _7.fld0,fld2: _18 };
Goto(bb24)
}
bb24 = {
place!(Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0)).1 = [3930200251_u32,3586486848_u32,2812353652_u32,3138235885_u32];
place!(Field::<i64>(Variant(_10, 2), 0)) = (-3185857186180423843_i64) << _2.0.3;
_13 = (-163217455706734060586671460357838153296_i128) | 100682057951625197543667481327841091566_i128;
_37 = _27;
_38 = -_13;
_7 = Adt23 { fld0: _12 };
_8 = 13391119636548814872_u64 << _4.2;
place!(Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0)).1 = [4088131315_u32,3804643921_u32,3242679348_u32,2502387136_u32];
_6.1 = [2371520791_u32,974989231_u32,738749588_u32,30700596_u32];
_2 = _4;
SetDiscriminant(_46.3, 0);
(*_19) = _11;
_46.1.2 = _38 as u8;
_2.1 = _4.1 >> _2.0.3;
_2.0 = (_4.0.0, _4.0.1, _4.0.2, _28);
_29 = _24 * _24;
_5 = _34;
_7 = Adt23 { fld0: _4.3 };
_32 = _29;
Goto(bb25)
}
bb25 = {
_4 = (_2.0, _2.1, _2.2, _12);
_33 = _8;
place!(Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0)).0 = (-59_i8);
_37 = _43 as i32;
_39 = [_14,_5,_34];
_41 = [_13,_13,_13,_13];
_46.1 = (_4.0.0, _2.0.1, _4.0.2, _2.0.3);
_7.fld0 = [_2.0.2];
Goto(bb26)
}
bb26 = {
_25 = core::ptr::addr_of!((*_25));
_46.0 = core::ptr::addr_of_mut!(_4.3);
(*_19) = !_11;
_39 = [_34,_5,_14];
_2.0.1 = _46.1.1;
_45 = _4.0.0;
_32 = _2.0.2 as f64;
_1 = core::ptr::addr_of!(_2.1);
_5 = _14;
place!(Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0)).2 = core::ptr::addr_of!(_4.0.2);
_47 = core::ptr::addr_of_mut!(_7.fld0);
(*_1) = !_4.1;
_11 = (*_1) != (*_1);
_46.1.3 = _4.0.3 + _30;
place!(Field::<isize>(Variant(_46.3, 0), 2)) = _46.1.0;
place!(Field::<[u8; 1]>(Variant(_46.3, 0), 1)) = [_4.0.2];
_33 = _8 | _8;
_39 = [_14,_5,_14];
match Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0).0 {
0 => bb25,
1 => bb24,
2 => bb9,
3 => bb14,
4 => bb20,
5 => bb6,
340282366920938463463374607431768211397 => bb27,
_ => bb7
}
}
bb27 = {
_51.2 = !2596379121_u32;
_41 = [_38,_38,_38,_38];
_2.2 = _4.2 & _4.2;
(*_19) = _11;
_51.2 = 2817265978_u32;
_8 = _33;
SetDiscriminant(_10, 0);
_24 = -_32;
_23 = (*_19);
place!(Field::<i32>(Variant(_10, 0), 0)) = _37 | _37;
_15 = [_2.2,_2.2];
_26 = Move(_1);
_35 = _23;
_7 = Adt23 { fld0: _12 };
(*_19) = _35;
_4.0.3 = Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0).0 as u16;
_26 = core::ptr::addr_of!(_2.1);
SetDiscriminant(_10, 1);
_14 = _34;
_53 = _4.2 ^ _2.2;
_14 = _5;
_54 = (-906898085916121813_i64) as f64;
_51.0 = _14;
Goto(bb28)
}
bb28 = {
_52 = _53 as f64;
_46.2 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_10, 1), 4)));
_42 = Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0).0;
_4.0.2 = !_46.1.2;
place!(Field::<(i8, [u32; 4], *const u8)>(Variant(_46.3, 0), 0)).1 = [_51.2,_51.2,_51.2,_51.2];
_16 = core::ptr::addr_of!(place!(Field::<[i32; 6]>(Variant(_10, 1), 7)));
_37 = _27 >> _33;
_26 = core::ptr::addr_of!(_2.1);
_40 = core::ptr::addr_of_mut!(_7);
_12 = [_4.0.2];
_2.1 = _4.1 >> _8;
(*_40) = Adt23 { fld0: _3 };
(*_16) = [_37,_37,_37,_37,_37,_37];
_46.1.0 = (-16352_i16) as isize;
SetDiscriminant(_46.3, 2);
_11 = (*_19);
_51.2 = 1313368311_u32 - 2278800778_u32;
_31 = _8;
_15 = [_2.2,_53];
_35 = (*_19) ^ (*_19);
(*_47) = [_2.0.2];
match _42 {
0 => bb14,
1 => bb11,
2 => bb9,
3 => bb8,
4 => bb27,
340282366920938463463374607431768211397 => bb29,
_ => bb6
}
}
bb29 = {
_37 = _27 - _27;
_20 = core::ptr::addr_of_mut!(_12);
_4.1 = _42 as u128;
_33 = _8 | _8;
_47 = core::ptr::addr_of_mut!(_7.fld0);
_16 = core::ptr::addr_of!((*_16));
place!(Field::<(u32, u8, i128)>(Variant(_46.3, 2), 3)).2 = _38;
_60 = _2.0.0 ^ _18;
match _42 {
340282366920938463463374607431768211397 => bb31,
_ => bb30
}
}
bb30 = {
_13 = (-122141435876828594471573110797741475583_i128);
_11 = true;
_2.1 = (*_1);
_2.0.1 = 4850_i16 as f32;
(*_1) = _2.1 >> _2.2;
_7 = Adt23 { fld0: _2.3 };
_19 = core::ptr::addr_of!(_11);
_21 = -_4.0.0;
_2.0.0 = _21 >> _2.2;
_2.0.0 = 1025087506_u32 as isize;
place!(Field::<i64>(Variant(_10, 2), 0)) = _4.0.0 as i64;
_2.0 = (_4.0.0, _4.0.1, _4.0.2, _4.0.3);
_20 = core::ptr::addr_of_mut!(_4.3);
_18 = -_21;
_15 = [_2.2,_2.2];
(*_19) = false;
_2.2 = _4.2;
_20 = core::ptr::addr_of_mut!(_3);
_20 = core::ptr::addr_of_mut!((*_20));
(*_20) = [_2.0.2];
Goto(bb20)
}
bb31 = {
_25 = core::ptr::addr_of!((*_25));
_30 = !_46.1.3;
_2 = (_46.1, _4.1, _53, _7.fld0);
_61 = &_4.1;
_10 = Adt54::Variant0 { fld0: _37 };
place!(Field::<Adt54>(Variant(_46.3, 2), 2)) = Adt54::Variant2 { fld0: 3250336156134165208_i64 };
_1 = core::ptr::addr_of!(_2.1);
_36 = Move(_46.2);
_29 = 21618_i16 as f64;
RET = core::ptr::addr_of!(_63.1);
_58 = Move(_20);
_4.0.0 = _21;
_20 = core::ptr::addr_of_mut!(_12);
(*_20) = (*_40).fld0;
_4.0.1 = _2.0.1 - _2.0.1;
_11 = !(*_19);
_30 = _46.1.3 - _28;
SetDiscriminant(_10, 2);
(*_1) = _4.1;
_51 = (_5, 3138862630_u32, 158212209_u32);
_7.fld0 = [_2.0.2];
_12 = [_4.0.2];
Goto(bb32)
}
bb32 = {
Call(_69 = dump_var(16_usize, 41_usize, Move(_41), 5_usize, Move(_5), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_69 = dump_var(16_usize, 53_usize, Move(_53), 45_usize, Move(_45), 34_usize, Move(_34), 38_usize, Move(_38)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_69 = dump_var(16_usize, 18_usize, Move(_18), 3_usize, Move(_3), 43_usize, Move(_43), 28_usize, Move(_28)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_69 = dump_var(16_usize, 35_usize, Move(_35), 8_usize, Move(_8), 70_usize, _70, 70_usize, _70), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: &'static (&'static u128, [bool; 7]),mut _2: &'static *const i16,mut _3: isize,mut _4: isize,mut _5: u16,mut _6: isize,mut _7: isize,mut _8: isize) -> char {
mir! {
type RET = char;
let _9: ([u8; 4], (Adt42, Adt54, *mut Adt23, char), char);
let _10: char;
let _11: [usize; 2];
let _12: Adt62;
let _13: char;
let _14: *const (i128, i64, i8, i8);
let _15: *mut &'static (i128, i64, i8, i8);
let _16: f32;
let _17: (*const u8, [bool; 8], (isize, f32, u8, u16), f32);
let _18: &'static *const i16;
let _19: *const *const [bool; 7];
let _20: char;
let _21: &'static (i128, i64, i8, i8);
let _22: i8;
let _23: *const (i128, i64, i8, i8);
let _24: *const u8;
let _25: ();
let _26: ();
{
_4 = _7 & _3;
RET = '\u{10c58f}';
RET = '\u{fe156}';
_3 = _7 << _4;
RET = '\u{e7058}';
_7 = 3976104664045013373_usize as isize;
_4 = _6;
_4 = _3;
RET = '\u{16aae}';
_6 = -_4;
_9.2 = RET;
_10 = RET;
RET = _10;
RET = _10;
Goto(bb1)
}
bb1 = {
_9.1.3 = RET;
_4 = -_6;
_4 = _8;
_8 = _6 ^ _4;
_9.0 = [22_u8,16_u8,198_u8,253_u8];
_9.1.3 = _9.2;
_9.1.3 = RET;
RET = _9.1.3;
_11 = [6549451073792396964_usize,17613953355291342892_usize];
_11 = [3_usize,7_usize];
_5 = 28414_u16;
_9.0 = [136_u8,191_u8,49_u8,29_u8];
_9.0 = [247_u8,116_u8,216_u8,44_u8];
_7 = _4;
RET = _10;
RET = _9.1.3;
_8 = _3 - _6;
_9.0 = [172_u8,234_u8,57_u8,178_u8];
_9.1.3 = _9.2;
_13 = _10;
_9.2 = _9.1.3;
_6 = !_7;
_9.1.1 = Adt54::Variant0 { fld0: 1029775292_i32 };
RET = _9.2;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
28414 => bb8,
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
RET = _9.1.3;
_3 = _6;
_6 = !_4;
_9.1.3 = _13;
place!(Field::<i32>(Variant(_9.1.1, 0), 0)) = 755623039_i32;
_10 = RET;
_7 = _6 + _4;
match Field::<i32>(Variant(_9.1.1, 0), 0) {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb9,
5 => bb10,
6 => bb11,
755623039 => bb13,
_ => bb12
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
Return()
}
bb13 = {
_6 = _8 * _4;
_8 = _6;
_10 = RET;
_9.1.3 = _10;
place!(Field::<i32>(Variant(_9.1.1, 0), 0)) = (-455294298_i32);
_5 = (-56_i8) as u16;
_17.0 = core::ptr::addr_of!(_17.2.2);
SetDiscriminant(_9.1.1, 2);
_17.2.0 = 202520575678052260315012054883976986705_u128 as isize;
_16 = _5 as f32;
_4 = -_7;
place!(Field::<i64>(Variant(_9.1.1, 2), 0)) = 3283068542384762614_i64;
_20 = RET;
_17.2.0 = 24838_i16 as isize;
_11 = [1_usize,5_usize];
_17.3 = -_16;
_15 = core::ptr::addr_of_mut!(_21);
_13 = RET;
_9.0 = [6_u8,96_u8,99_u8,94_u8];
_9.1.3 = _9.2;
_17.1 = [true,true,true,true,true,true,true,false];
_22 = -(-25_i8);
_17.2 = (_8, _17.3, 185_u8, _5);
_8 = _3 << _4;
_8 = -_17.2.0;
match _17.2.2 {
185 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
RET = _13;
RET = _13;
_9.2 = _9.1.3;
_10 = RET;
_20 = _9.1.3;
_8 = 3291911003534574411_usize as isize;
SetDiscriminant(_9.1.1, 2);
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(17_usize, 6_usize, Move(_6), 3_usize, Move(_3), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_25 = dump_var(17_usize, 10_usize, Move(_10), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u8,mut _2: isize,mut _3: bool,mut _4: char,mut _5: isize,mut _6: isize,mut _7: u8,mut _8: i8,mut _9: [u8; 4],mut _10: u8,mut _11: isize,mut _12: isize) -> Adt23 {
mir! {
type RET = Adt23;
let _13: ([u8; 4], (Adt42, Adt54, *mut Adt23, char), char);
let _14: ();
let _15: ();
{
_1 = _10 + _7;
Goto(bb1)
}
bb1 = {
_1 = _2 as u8;
_12 = _5 >> _11;
_12 = (-1066968610_i32) as isize;
_10 = _1 - _7;
RET.fld0 = [_10];
_13.2 = _4;
_12 = _2;
RET.fld0 = [_7];
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(18_usize, 3_usize, Move(_3), 5_usize, Move(_5), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_14 = dump_var(18_usize, 11_usize, Move(_11), 9_usize, Move(_9), 15_usize, _15, 15_usize, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: [u8; 1],mut _2: [u8; 1],mut _3: usize,mut _4: [u8; 1],mut _5: *mut Adt23,mut _6: isize,mut _7: [u8; 4]) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _8: i64;
let _9: [isize; 4];
let _10: bool;
let _11: *mut u8;
let _12: (f64, u16, i8);
let _13: (char, u32, u32);
let _14: isize;
let _15: usize;
let _16: isize;
let _17: *const u8;
let _18: u8;
let _19: bool;
let _20: *mut i16;
let _21: (isize, f32, u8, u16);
let _22: Adt23;
let _23: bool;
let _24: f32;
let _25: (*const u8, [bool; 8], (isize, f32, u8, u16), f32);
let _26: isize;
let _27: *mut &'static (i128, i64, i8, i8);
let _28: i32;
let _29: (char, u32, u32);
let _30: i16;
let _31: f64;
let _32: &'static [isize; 4];
let _33: ();
let _34: ();
{
_2 = [168_u8];
_7 = [162_u8,136_u8,88_u8,84_u8];
_3 = (-149432111902045526948577889978271772364_i128) as usize;
_2 = [100_u8];
RET = [2998527985_u32,1543926254_u32,3327128497_u32,3878666927_u32];
_8 = (-6867520335061700692_i64) | (-3015657036495480442_i64);
_3 = !7_usize;
_2 = [109_u8];
_9 = [_6,_6,_6,_6];
_3 = 4341691653839519240_usize;
RET = [1691633826_u32,2663544048_u32,2929296486_u32,210841194_u32];
_7 = [14_u8,159_u8,191_u8,197_u8];
_7 = [208_u8,221_u8,106_u8,166_u8];
_7 = [92_u8,197_u8,142_u8,31_u8];
_7 = [144_u8,85_u8,49_u8,37_u8];
RET = [1937889630_u32,2845060409_u32,3279150713_u32,3371305702_u32];
_4 = [51_u8];
_9 = [_6,_6,_6,_6];
_10 = false;
_9 = [_6,_6,_6,_6];
_10 = false;
_10 = !true;
RET = [2168924211_u32,775450451_u32,3514060435_u32,4088919619_u32];
RET = [3527906287_u32,2119413717_u32,3387593888_u32,3509963857_u32];
RET = [4025650526_u32,3392206602_u32,107967826_u32,2490423907_u32];
_4 = [39_u8];
_9 = [_6,_6,_6,_6];
_4 = [62_u8];
_1 = [88_u8];
Goto(bb1)
}
bb1 = {
_1 = [40_u8];
_7 = [255_u8,41_u8,190_u8,54_u8];
_7 = [254_u8,57_u8,133_u8,209_u8];
RET = [1272286637_u32,3838601512_u32,1988115026_u32,3680237721_u32];
_4 = [27_u8];
_6 = (-23_isize) >> _8;
_4 = [21_u8];
_1 = [60_u8];
_1 = _4;
RET = [423738072_u32,2876325370_u32,2868652319_u32,2240572832_u32];
_3 = 7566837271500655560_usize;
_12.0 = (-775242025_i32) as f64;
_12.1 = 56762_u16 - 35429_u16;
_13.0 = '\u{7d814}';
_13 = ('\u{65a76}', 897869205_u32, 934470571_u32);
_1 = [190_u8];
_12.2 = -17_i8;
_9 = [_6,_6,_6,_6];
RET = [_13.2,_13.1,_13.2,_13.2];
_12.1 = 46585_u16;
_12.1 = 12667_u16 + 621_u16;
Goto(bb2)
}
bb2 = {
_10 = false & false;
_12.0 = _13.1 as f64;
_4 = [251_u8];
_6 = 9223372036854775807_isize;
_1 = _4;
_1 = [206_u8];
_12.2 = 12381_i16 as i8;
_14 = _10 as isize;
_1 = [149_u8];
_2 = [123_u8];
RET = [_13.1,_13.2,_13.2,_13.1];
_6 = _14 * _14;
_8 = 1622427141240889146_i64;
_10 = !false;
_8 = 4807975452473316670_i64 & (-1366209642203214538_i64);
_13.2 = _13.1 + _13.1;
_13 = ('\u{c93ab}', 1107767061_u32, 3760372389_u32);
_2 = [146_u8];
Goto(bb3)
}
bb3 = {
_14 = 310902349306970276281270800321634783906_u128 as isize;
_12.0 = 111788530112735376985292453112298991078_i128 as f64;
_2 = _4;
RET = [_13.1,_13.1,_13.1,_13.1];
_13.1 = _13.2 ^ _13.2;
Goto(bb4)
}
bb4 = {
_4 = [38_u8];
_8 = 278374228733319911_i64;
_14 = _6 + _6;
_18 = _12.2 as u8;
_17 = core::ptr::addr_of!(_18);
_17 = core::ptr::addr_of!(_18);
_14 = !_6;
_14 = 10107885209347393316_u64 as isize;
_11 = core::ptr::addr_of_mut!((*_17));
_16 = 19810_i16 as isize;
_12.1 = _10 as u16;
_3 = 7_usize * 13603740844725848410_usize;
_13.0 = '\u{cda75}';
(*_11) = 182_u8 >> _6;
(*_11) = !255_u8;
RET = [_13.2,_13.2,_13.2,_13.2];
_14 = _8 as isize;
_9 = [_6,_6,_16,_6];
_12.0 = _13.1 as f64;
_17 = core::ptr::addr_of!(_21.2);
(*_17) = (*_11);
_18 = _21.2 >> _21.2;
_10 = !false;
Goto(bb5)
}
bb5 = {
(*_17) = !(*_11);
_19 = !_10;
_13.1 = _19 as u32;
(*_11) = (*_17);
_4 = [_21.2];
_7 = [(*_11),_21.2,_21.2,_21.2];
match _13.2 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
3760372389 => bb10,
_ => bb9
}
}
bb6 = {
_4 = [38_u8];
_8 = 278374228733319911_i64;
_14 = _6 + _6;
_18 = _12.2 as u8;
_17 = core::ptr::addr_of!(_18);
_17 = core::ptr::addr_of!(_18);
_14 = !_6;
_14 = 10107885209347393316_u64 as isize;
_11 = core::ptr::addr_of_mut!((*_17));
_16 = 19810_i16 as isize;
_12.1 = _10 as u16;
_3 = 7_usize * 13603740844725848410_usize;
_13.0 = '\u{cda75}';
(*_11) = 182_u8 >> _6;
(*_11) = !255_u8;
RET = [_13.2,_13.2,_13.2,_13.2];
_14 = _8 as isize;
_9 = [_6,_6,_16,_6];
_12.0 = _13.1 as f64;
_17 = core::ptr::addr_of!(_21.2);
(*_17) = (*_11);
_18 = _21.2 >> _21.2;
_10 = !false;
Goto(bb5)
}
bb7 = {
_14 = 310902349306970276281270800321634783906_u128 as isize;
_12.0 = 111788530112735376985292453112298991078_i128 as f64;
_2 = _4;
RET = [_13.1,_13.1,_13.1,_13.1];
_13.1 = _13.2 ^ _13.2;
Goto(bb4)
}
bb8 = {
_10 = false & false;
_12.0 = _13.1 as f64;
_4 = [251_u8];
_6 = 9223372036854775807_isize;
_1 = _4;
_1 = [206_u8];
_12.2 = 12381_i16 as i8;
_14 = _10 as isize;
_1 = [149_u8];
_2 = [123_u8];
RET = [_13.1,_13.2,_13.2,_13.1];
_6 = _14 * _14;
_8 = 1622427141240889146_i64;
_10 = !false;
_8 = 4807975452473316670_i64 & (-1366209642203214538_i64);
_13.2 = _13.1 + _13.1;
_13 = ('\u{c93ab}', 1107767061_u32, 3760372389_u32);
_2 = [146_u8];
Goto(bb3)
}
bb9 = {
_1 = [40_u8];
_7 = [255_u8,41_u8,190_u8,54_u8];
_7 = [254_u8,57_u8,133_u8,209_u8];
RET = [1272286637_u32,3838601512_u32,1988115026_u32,3680237721_u32];
_4 = [27_u8];
_6 = (-23_isize) >> _8;
_4 = [21_u8];
_1 = [60_u8];
_1 = _4;
RET = [423738072_u32,2876325370_u32,2868652319_u32,2240572832_u32];
_3 = 7566837271500655560_usize;
_12.0 = (-775242025_i32) as f64;
_12.1 = 56762_u16 - 35429_u16;
_13.0 = '\u{7d814}';
_13 = ('\u{65a76}', 897869205_u32, 934470571_u32);
_1 = [190_u8];
_12.2 = -17_i8;
_9 = [_6,_6,_6,_6];
RET = [_13.2,_13.1,_13.2,_13.2];
_12.1 = 46585_u16;
_12.1 = 12667_u16 + 621_u16;
Goto(bb2)
}
bb10 = {
_6 = _14 >> _12.2;
_22.fld0 = [_21.2];
_9 = [_6,_6,_14,_6];
_23 = _19 ^ _10;
_23 = !_10;
_25.2.3 = !_12.1;
(*_17) = (*_11);
_25.2.1 = _3 as f32;
RET = [_13.2,_13.2,_13.1,_13.1];
RET = [_13.2,_13.2,_13.2,_13.2];
_21.3 = _12.1 + _12.1;
_24 = _13.1 as f32;
_22 = Adt23 { fld0: _4 };
_25.2.0 = !_16;
(*_11) = 205265435237303922683777725783253870228_u128 as u8;
_17 = core::ptr::addr_of!(_21.2);
_21.2 = !(*_11);
match _13.2 {
0 => bb1,
1 => bb2,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
3760372389 => bb17,
_ => bb16
}
}
bb11 = {
_1 = [40_u8];
_7 = [255_u8,41_u8,190_u8,54_u8];
_7 = [254_u8,57_u8,133_u8,209_u8];
RET = [1272286637_u32,3838601512_u32,1988115026_u32,3680237721_u32];
_4 = [27_u8];
_6 = (-23_isize) >> _8;
_4 = [21_u8];
_1 = [60_u8];
_1 = _4;
RET = [423738072_u32,2876325370_u32,2868652319_u32,2240572832_u32];
_3 = 7566837271500655560_usize;
_12.0 = (-775242025_i32) as f64;
_12.1 = 56762_u16 - 35429_u16;
_13.0 = '\u{7d814}';
_13 = ('\u{65a76}', 897869205_u32, 934470571_u32);
_1 = [190_u8];
_12.2 = -17_i8;
_9 = [_6,_6,_6,_6];
RET = [_13.2,_13.1,_13.2,_13.2];
_12.1 = 46585_u16;
_12.1 = 12667_u16 + 621_u16;
Goto(bb2)
}
bb12 = {
_14 = 310902349306970276281270800321634783906_u128 as isize;
_12.0 = 111788530112735376985292453112298991078_i128 as f64;
_2 = _4;
RET = [_13.1,_13.1,_13.1,_13.1];
_13.1 = _13.2 ^ _13.2;
Goto(bb4)
}
bb13 = {
_14 = 310902349306970276281270800321634783906_u128 as isize;
_12.0 = 111788530112735376985292453112298991078_i128 as f64;
_2 = _4;
RET = [_13.1,_13.1,_13.1,_13.1];
_13.1 = _13.2 ^ _13.2;
Goto(bb4)
}
bb14 = {
_4 = [38_u8];
_8 = 278374228733319911_i64;
_14 = _6 + _6;
_18 = _12.2 as u8;
_17 = core::ptr::addr_of!(_18);
_17 = core::ptr::addr_of!(_18);
_14 = !_6;
_14 = 10107885209347393316_u64 as isize;
_11 = core::ptr::addr_of_mut!((*_17));
_16 = 19810_i16 as isize;
_12.1 = _10 as u16;
_3 = 7_usize * 13603740844725848410_usize;
_13.0 = '\u{cda75}';
(*_11) = 182_u8 >> _6;
(*_11) = !255_u8;
RET = [_13.2,_13.2,_13.2,_13.2];
_14 = _8 as isize;
_9 = [_6,_6,_16,_6];
_12.0 = _13.1 as f64;
_17 = core::ptr::addr_of!(_21.2);
(*_17) = (*_11);
_18 = _21.2 >> _21.2;
_10 = !false;
Goto(bb5)
}
bb15 = {
(*_17) = !(*_11);
_19 = !_10;
_13.1 = _19 as u32;
(*_11) = (*_17);
_4 = [_21.2];
_7 = [(*_11),_21.2,_21.2,_21.2];
match _13.2 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
3760372389 => bb10,
_ => bb9
}
}
bb16 = {
_4 = [38_u8];
_8 = 278374228733319911_i64;
_14 = _6 + _6;
_18 = _12.2 as u8;
_17 = core::ptr::addr_of!(_18);
_17 = core::ptr::addr_of!(_18);
_14 = !_6;
_14 = 10107885209347393316_u64 as isize;
_11 = core::ptr::addr_of_mut!((*_17));
_16 = 19810_i16 as isize;
_12.1 = _10 as u16;
_3 = 7_usize * 13603740844725848410_usize;
_13.0 = '\u{cda75}';
(*_11) = 182_u8 >> _6;
(*_11) = !255_u8;
RET = [_13.2,_13.2,_13.2,_13.2];
_14 = _8 as isize;
_9 = [_6,_6,_16,_6];
_12.0 = _13.1 as f64;
_17 = core::ptr::addr_of!(_21.2);
(*_17) = (*_11);
_18 = _21.2 >> _21.2;
_10 = !false;
Goto(bb5)
}
bb17 = {
_29.2 = _13.2 * _13.2;
_12.1 = _21.3;
_22.fld0 = [_21.2];
Goto(bb18)
}
bb18 = {
Call(_33 = dump_var(19_usize, 3_usize, Move(_3), 16_usize, Move(_16), 19_usize, Move(_19), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(19_usize, 9_usize, Move(_9), 18_usize, Move(_18), 10_usize, Move(_10), 34_usize, _34), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(9906133640603750400_u64), std::hint::black_box('\u{5c712}'), std::hint::black_box(71_isize), std::hint::black_box((-119_i8)), std::hint::black_box((-22997_i16)), std::hint::black_box(193124390_i32), std::hint::black_box(7167072046709517560_i64), std::hint::black_box(1999524726_u32), std::hint::black_box(3_usize), std::hint::black_box(186_u8), std::hint::black_box(308151100700005803678582178802377388191_u128));
                
            }
impl PrintFDebug for Adt23{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt23{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt23 {
fld0: [u8; 1],
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: [u32; 4],

},
Variant1{
fld0: *const u8,

},
Variant2{
fld0: i64,
fld1: i16,
fld2: [u8; 1],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const u128,
fld1: (isize, f32, u8, u16),
fld2: isize,
fld3: ([isize; 4],),

},
Variant1{
fld0: [u8; 1],
fld1: Adt42,

},
Variant2{
fld0: u8,
fld1: Adt23,
fld2: [char; 1],
fld3: [bool; 7],
fld4: [isize; 7],

},
Variant3{
fld0: (*const u8, [bool; 8], (isize, f32, u8, u16), f32),
fld1: (f64, u16, i8),
fld2: isize,
fld3: u64,
fld4: u8,
fld5: [bool; 8],
fld6: *const bool,
fld7: i128,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: i32,

},
Variant1{
fld0: u128,
fld1: char,
fld2: u64,
fld3: i8,
fld4: i16,
fld5: Adt47,
fld6: [isize; 4],
fld7: [i32; 6],

},
Variant2{
fld0: i64,

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: (i8, [u32; 4], *const u8),
fld1: [u8; 1],
fld2: isize,

},
Variant1{
fld0: bool,
fld1: *const (i128, i64, i8, i8),
fld2: [isize; 4],
fld3: ((char, u32, u32),),

},
Variant2{
fld0: *const u128,
fld1: [char; 1],
fld2: Adt54,
fld3: (u32, u8, i128),
fld4: *const bool,

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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: Adt47,
fld1: i16,
fld2: u128,

},
Variant1{
fld0: bool,
fld1: u32,

},
Variant2{
fld0: *const [bool; 7],
fld1: char,

},
Variant3{
fld0: Adt23,
fld1: u8,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){unsafe{printf("Adt75::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: u32,
fld1: [bool; 6],
fld2: [char; 3],

},
Variant1{
fld0: bool,
fld1: [isize; 7],
fld2: u16,
fld3: u128,
fld4: ([u32; 4],),

}}
impl PrintFDebug for Adt78{
	unsafe fn printf_debug(&self){unsafe{printf("Adt78::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt78 {
Variant0{
fld0: u128,
fld1: [isize; 4],
fld2: *const [i32; 6],
fld3: (i128, i64, i8, i8),
fld4: Adt73,
fld5: (i8, [u32; 4], *const u8),

},
Variant1{
fld0: Adt23,
fld1: ((isize, f32, u8, u16), u128, usize, [u8; 1]),

},
Variant2{
fld0: ([i32; 6], usize, usize),
fld1: [isize; 7],
fld2: Adt47,
fld3: *const u128,
fld4: u8,
fld5: (u32, u8, i128),

}}

