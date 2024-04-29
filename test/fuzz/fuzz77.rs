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
pub fn fn0(mut _1: u128,mut _2: char,mut _3: u16,mut _4: i8,mut _5: i16,mut _6: u32,mut _7: i64,mut _8: i128,mut _9: usize) -> [i8; 2] {
mir! {
type RET = [i8; 2];
let _10: char;
let _11: f32;
let _12: *mut u64;
let _13: u8;
let _14: ();
let _15: ();
{
_1 = !109975159139082179055953717581085931195_u128;
_1 = (-69_i8) as u128;
Call(_4 = fn1(_1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_4,_4];
_8 = 163821500631997722583137592381606237842_i128 + 76231525628093012210409038188742330707_i128;
_10 = '\u{8f064}';
_7 = !4051297953926172480_i64;
_9 = 3_usize >> _4;
_1 = !199171403372081323351567079723334625690_u128;
RET = [_4,_4];
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(0_usize, 1_usize, Move(_1), 10_usize, Move(_10), 8_usize, Move(_8), 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: u128,mut _9: u128,mut _10: u128,mut _11: u128,mut _12: u128,mut _13: u128) -> i8 {
mir! {
type RET = i8;
let _14: Adt45;
let _15: (u64, i32, bool);
let _16: *mut u32;
let _17: Adt54;
let _18: i8;
let _19: [u8; 5];
let _20: [i64; 7];
let _21: u8;
let _22: f32;
let _23: Adt53;
let _24: u8;
let _25: bool;
let _26: char;
let _27: isize;
let _28: f32;
let _29: isize;
let _30: Adt44;
let _31: u32;
let _32: [u16; 1];
let _33: f64;
let _34: Adt46;
let _35: Adt53;
let _36: isize;
let _37: f32;
let _38: bool;
let _39: u64;
let _40: ();
let _41: ();
{
_12 = 44845_u16 as u128;
_15.0 = 2950621400567413664_u64 & 782237172657071616_u64;
_8 = 22439_u16 as u128;
RET = 45_i8;
_2 = _11;
_3 = _4;
_1 = (-63_isize) as u128;
Goto(bb1)
}
bb1 = {
_2 = (-3597838869748394608_i64) as u128;
RET = (-19_i8) << _15.0;
_2 = _11;
_5 = 512593261_i32 as u128;
_9 = _8;
_9 = _3 ^ _11;
_15.0 = 26234_i16 as u64;
_2 = _9;
_15.2 = !true;
_13 = 31481_i16 as u128;
_4 = _8 + _3;
_12 = _4 | _5;
Goto(bb2)
}
bb2 = {
_15.1 = (-985828583_i32);
_5 = !_4;
_5 = _13;
_5 = !_1;
_7 = _9 << _5;
_7 = _15.0 as u128;
_10 = _11;
_1 = _4 << _9;
_15 = (12937391800601390746_u64, 1098868144_i32, true);
_1 = !_12;
_15.1 = 694504362_i32 - 79660278_i32;
_1 = _15.1 as u128;
_7 = _10;
_9 = _6;
RET = !57_i8;
_6 = !_4;
_1 = _4;
_20 = [5971773644397330391_i64,1904553454422469787_i64,(-1369982531549272894_i64),4258358418924270561_i64,(-6012620297359939260_i64),(-8445876742744388933_i64),(-2526945645662216802_i64)];
Call(_9 = fn2(_15, _15, _15.0, _15, RET, _20, _4, _20, _15.1, _3, _20, _15.1, _15.0, _15.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = (3769000353660781518_u64, (-1712028147_i32), true);
_10 = _15.0 as u128;
_3 = _10;
_9 = _2 - _11;
_23.fld0 = _10 == _12;
_22 = 8915428716412423591_i64 as f32;
_21 = 218_u8;
_18 = !RET;
match _15.1 {
0 => bb1,
1 => bb4,
2 => bb5,
340282366920938463463374607430056183309 => bb7,
_ => bb6
}
}
bb4 = {
_15.1 = (-985828583_i32);
_5 = !_4;
_5 = _13;
_5 = !_1;
_7 = _9 << _5;
_7 = _15.0 as u128;
_10 = _11;
_1 = _4 << _9;
_15 = (12937391800601390746_u64, 1098868144_i32, true);
_1 = !_12;
_15.1 = 694504362_i32 - 79660278_i32;
_1 = _15.1 as u128;
_7 = _10;
_9 = _6;
RET = !57_i8;
_6 = !_4;
_1 = _4;
_20 = [5971773644397330391_i64,1904553454422469787_i64,(-1369982531549272894_i64),4258358418924270561_i64,(-6012620297359939260_i64),(-8445876742744388933_i64),(-2526945645662216802_i64)];
Call(_9 = fn2(_15, _15, _15.0, _15, RET, _20, _4, _20, _15.1, _3, _20, _15.1, _15.0, _15.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2 = (-3597838869748394608_i64) as u128;
RET = (-19_i8) << _15.0;
_2 = _11;
_5 = 512593261_i32 as u128;
_9 = _8;
_9 = _3 ^ _11;
_15.0 = 26234_i16 as u64;
_2 = _9;
_15.2 = !true;
_13 = 31481_i16 as u128;
_4 = _8 + _3;
_12 = _4 | _5;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_2 = _10 - _12;
_19 = [_21,_21,_21,_21,_21];
RET = !_18;
_9 = _22 as u128;
RET = _15.1 as i8;
_5 = _23.fld0 as u128;
_15 = (7247787357804696447_u64, (-847530196_i32), _23.fld0);
_19 = [_21,_21,_21,_21,_21];
_25 = _3 <= _5;
_19 = [_21,_21,_21,_21,_21];
_9 = _12;
_15.2 = !_23.fld0;
_12 = !_5;
Goto(bb8)
}
bb8 = {
_23.fld0 = _25;
_20 = [(-8266680992097790509_i64),5007647581527617847_i64,3828104377726055580_i64,7760798286831520940_i64,(-398613805675386643_i64),(-6847245872403771118_i64),(-8438266592488563237_i64)];
_5 = _12;
_26 = '\u{f78d4}';
_15.0 = 268310259337370787_u64 ^ 6958549639924949027_u64;
RET = _18 * _18;
_3 = !_13;
_12 = _3;
_19 = [_21,_21,_21,_21,_21];
_13 = _12;
_15.0 = !10451038099261337838_u64;
_8 = _2;
_23 = Adt53 { fld0: _15.2 };
_27 = (-9223372036854775808_isize);
_23.fld0 = !_25;
RET = _18;
_19 = [_21,_21,_21,_21,_21];
_23.fld0 = _2 != _2;
Goto(bb9)
}
bb9 = {
_15.1 = 2038072099_i32;
_22 = RET as f32;
_15.1 = _18 as i32;
_24 = _21;
_11 = _15.2 as u128;
_8 = !_7;
_1 = _2;
_1 = _6 >> _6;
_18 = RET;
_31 = 658214254_u32;
_1 = !_8;
_21 = (-103731200255812545021080605367692454630_i128) as u8;
_35.fld0 = _23.fld0;
_4 = _31 as u128;
_28 = 8576483541370283578_usize as f32;
_29 = !_27;
_18 = 0_usize as i8;
_15 = (3621982219644815676_u64, (-180395997_i32), _35.fld0);
_1 = _15.0 as u128;
_8 = _10;
_15.2 = _22 >= _22;
_3 = _1 - _8;
_31 = !666781065_u32;
_28 = -_22;
_11 = _10;
_35.fld0 = _23.fld0;
match _15.1 {
0 => bb10,
340282366920938463463374607431587815459 => bb12,
_ => bb11
}
}
bb10 = {
_23.fld0 = _25;
_20 = [(-8266680992097790509_i64),5007647581527617847_i64,3828104377726055580_i64,7760798286831520940_i64,(-398613805675386643_i64),(-6847245872403771118_i64),(-8438266592488563237_i64)];
_5 = _12;
_26 = '\u{f78d4}';
_15.0 = 268310259337370787_u64 ^ 6958549639924949027_u64;
RET = _18 * _18;
_3 = !_13;
_12 = _3;
_19 = [_21,_21,_21,_21,_21];
_13 = _12;
_15.0 = !10451038099261337838_u64;
_8 = _2;
_23 = Adt53 { fld0: _15.2 };
_27 = (-9223372036854775808_isize);
_23.fld0 = !_25;
RET = _18;
_19 = [_21,_21,_21,_21,_21];
_23.fld0 = _2 != _2;
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
RET = _18 >> _15.1;
_11 = _15.1 as u128;
_1 = _3 >> _15.1;
_16 = core::ptr::addr_of_mut!(_31);
_15.2 = !_35.fld0;
_33 = _28 as f64;
_32 = [1315_u16];
_15.0 = !1818192399248664057_u64;
_25 = !_15.2;
_13 = _6 * _9;
_36 = _29;
_23.fld0 = !_15.2;
_31 = 3270948877_u32;
_37 = _22;
Goto(bb13)
}
bb13 = {
Call(_40 = dump_var(1_usize, 5_usize, Move(_5), 32_usize, Move(_32), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_40 = dump_var(1_usize, 26_usize, Move(_26), 7_usize, Move(_7), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_40 = dump_var(1_usize, 15_usize, Move(_15), 21_usize, Move(_21), 13_usize, Move(_13), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(1_usize, 18_usize, Move(_18), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: (u64, i32, bool),mut _2: (u64, i32, bool),mut _3: u64,mut _4: (u64, i32, bool),mut _5: i8,mut _6: [i64; 7],mut _7: u128,mut _8: [i64; 7],mut _9: i32,mut _10: u128,mut _11: [i64; 7],mut _12: i32,mut _13: u64,mut _14: u64) -> u128 {
mir! {
type RET = u128;
let _15: Adt58;
let _16: bool;
let _17: *const [i8; 2];
let _18: isize;
let _19: i32;
let _20: (*mut *mut char,);
let _21: *mut [i64; 7];
let _22: [i8; 2];
let _23: Adt53;
let _24: bool;
let _25: [bool; 8];
let _26: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8);
let _27: Adt57;
let _28: [bool; 8];
let _29: f32;
let _30: u128;
let _31: [u8; 5];
let _32: f64;
let _33: isize;
let _34: *mut u64;
let _35: usize;
let _36: Adt49;
let _37: (&'static u16, [i8; 2], u8, bool);
let _38: f32;
let _39: ();
let _40: ();
{
_10 = '\u{e7c55}' as u128;
_6 = _11;
_3 = _14;
RET = !_10;
_3 = _13 | _1.0;
_1 = (_13, _2.1, _2.2);
_2 = _4;
RET = 7410_i16 as u128;
Goto(bb1)
}
bb1 = {
_6 = [6925608964444618121_i64,9010742774283522125_i64,4966268327848039825_i64,(-8698311913731584613_i64),(-1193762922845908417_i64),(-662171349707922973_i64),(-7440618862791060579_i64)];
_2 = (_13, _12, _1.2);
RET = _10;
RET = _10 + _10;
_13 = !_3;
_4.2 = !_1.2;
_12 = 124487954006666753483284813044789789769_i128 as i32;
_13 = !_14;
_4.0 = !_14;
_4.2 = _1.2 | _1.2;
_12 = 30_u8 as i32;
_13 = !_3;
_4.2 = !_1.2;
_11 = [7599555918287430424_i64,(-5496146314075589544_i64),(-1501565018552144109_i64),(-3881006825249927553_i64),3912345732763666528_i64,(-1392435200021481155_i64),(-6908893142429346227_i64)];
_1.2 = _4.2;
_4.2 = !_1.2;
_12 = -_9;
_2 = (_3, _4.1, _4.2);
_10 = '\u{30c9b}' as u128;
_14 = 9223372036854775807_isize as u64;
_2.1 = _3 as i32;
_16 = _4.1 < _9;
_18 = _4.1 as isize;
Call(RET = core::intrinsics::bswap(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = -82_i8;
_9 = _2.1;
_2.2 = !_16;
_4.1 = !_12;
_10 = RET;
_12 = 5_usize as i32;
_8 = [(-7359574657918272235_i64),5289062160448808309_i64,(-4239196739128235213_i64),5106804296017003664_i64,(-7017536216149168546_i64),2946656220784778701_i64,(-3997160900337974981_i64)];
_3 = _13 / _1.0;
_15 = Adt58::Variant0 { fld0: _2.1 };
_19 = _4.1 ^ _9;
_8 = [4544055508188076980_i64,(-3733913352537049011_i64),(-6935869156809760394_i64),(-5230970574487747930_i64),(-2757702856067598406_i64),(-4375865913274257268_i64),(-307041612793870216_i64)];
_1.0 = _18 as u64;
_4.1 = _2.2 as i32;
_13 = _3 | _3;
_7 = RET;
_10 = _7;
_3 = Field::<i32>(Variant(_15, 0), 0) as u64;
_21 = core::ptr::addr_of_mut!(_6);
_17 = core::ptr::addr_of!(_22);
RET = _10;
Call(_1.1 = fn3(_3, _21, _9, _3, _1.2, _8, _13, _7, Move(_15)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_21 = core::ptr::addr_of_mut!((*_21));
(*_17) = [_5,_5];
_3 = !_13;
_3 = !_13;
_4 = (_3, _2.1, _2.2);
_13 = 26617_u16 as u64;
_2 = (_4.0, _1.1, _4.2);
_18 = 9223372036854775807_isize;
(*_17) = [_5,_5];
_11 = [(-3548169834362138993_i64),(-2745915742964408732_i64),(-6973477550294155198_i64),7921706443138655102_i64,575671442060789289_i64,3412830683923140910_i64,(-6526376641853074046_i64)];
(*_21) = _8;
RET = _4.2 as u128;
_7 = 5379828949668412601273720981252061660_i128 as u128;
_22 = [_5,_5];
RET = !_10;
match _18 {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb4 = {
_1.2 = _2.2 >= _16;
_11 = (*_21);
_13 = _18 as u64;
_6 = _8;
_5 = (-88_i8) ^ 13_i8;
_13 = !_4.0;
_23.fld0 = !_16;
_17 = core::ptr::addr_of!((*_17));
_21 = core::ptr::addr_of_mut!((*_21));
Goto(bb5)
}
bb5 = {
_16 = _4.2 ^ _1.2;
_2.2 = _23.fld0;
_11 = _6;
_10 = !RET;
_2.0 = _4.0 & _13;
_6 = _8;
_7 = 18150855430743489643_usize as u128;
_4.1 = !_2.1;
_1.2 = _23.fld0;
_4.2 = !_23.fld0;
_1.2 = _4.2;
_10 = _7;
RET = _10;
_17 = core::ptr::addr_of!(_22);
_4.1 = -_2.1;
_20.0 = core::ptr::addr_of_mut!(_26.3.1);
_25 = [_1.2,_4.2,_16,_23.fld0,_16,_16,_16,_2.2];
_26.1 = !_18;
_17 = core::ptr::addr_of!((*_17));
_22 = [_5,_5];
_4 = _2;
_2.2 = !_16;
_1.1 = -_2.1;
_19 = _1.1 | _1.1;
RET = _7 * _10;
_6 = _11;
_29 = 62078_u16 as f32;
match _18 {
9223372036854775807 => bb6,
_ => bb4
}
}
bb6 = {
_26.3.0 = RET * RET;
_27.fld2.1 = _2.0 as i32;
_29 = 2408776355718367790_i64 as f32;
_27.fld4 = [_5,_5];
_27.fld2.2 = _4.0 == _4.0;
_1.0 = _13 ^ _3;
RET = _5 as u128;
_10 = _26.3.0 | _7;
_1 = _2;
_24 = _27.fld2.1 > _19;
_20.0 = core::ptr::addr_of_mut!(_26.3.1);
_4.0 = '\u{46fde}' as u64;
_4.2 = _24;
_12 = -_1.1;
_7 = _2.0 as u128;
_26.1 = -_18;
_9 = _19 + _19;
match _18 {
0 => bb5,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
9223372036854775807 => bb13,
_ => bb12
}
}
bb7 = {
_16 = _4.2 ^ _1.2;
_2.2 = _23.fld0;
_11 = _6;
_10 = !RET;
_2.0 = _4.0 & _13;
_6 = _8;
_7 = 18150855430743489643_usize as u128;
_4.1 = !_2.1;
_1.2 = _23.fld0;
_4.2 = !_23.fld0;
_1.2 = _4.2;
_10 = _7;
RET = _10;
_17 = core::ptr::addr_of!(_22);
_4.1 = -_2.1;
_20.0 = core::ptr::addr_of_mut!(_26.3.1);
_25 = [_1.2,_4.2,_16,_23.fld0,_16,_16,_16,_2.2];
_26.1 = !_18;
_17 = core::ptr::addr_of!((*_17));
_22 = [_5,_5];
_4 = _2;
_2.2 = !_16;
_1.1 = -_2.1;
_19 = _1.1 | _1.1;
RET = _7 * _10;
_6 = _11;
_29 = 62078_u16 as f32;
match _18 {
9223372036854775807 => bb6,
_ => bb4
}
}
bb8 = {
_1.2 = _2.2 >= _16;
_11 = (*_21);
_13 = _18 as u64;
_6 = _8;
_5 = (-88_i8) ^ 13_i8;
_13 = !_4.0;
_23.fld0 = !_16;
_17 = core::ptr::addr_of!((*_17));
_21 = core::ptr::addr_of_mut!((*_21));
Goto(bb5)
}
bb9 = {
_21 = core::ptr::addr_of_mut!((*_21));
(*_17) = [_5,_5];
_3 = !_13;
_3 = !_13;
_4 = (_3, _2.1, _2.2);
_13 = 26617_u16 as u64;
_2 = (_4.0, _1.1, _4.2);
_18 = 9223372036854775807_isize;
(*_17) = [_5,_5];
_11 = [(-3548169834362138993_i64),(-2745915742964408732_i64),(-6973477550294155198_i64),7921706443138655102_i64,575671442060789289_i64,3412830683923140910_i64,(-6526376641853074046_i64)];
(*_21) = _8;
RET = _4.2 as u128;
_7 = 5379828949668412601273720981252061660_i128 as u128;
_22 = [_5,_5];
RET = !_10;
match _18 {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb10 = {
_5 = -82_i8;
_9 = _2.1;
_2.2 = !_16;
_4.1 = !_12;
_10 = RET;
_12 = 5_usize as i32;
_8 = [(-7359574657918272235_i64),5289062160448808309_i64,(-4239196739128235213_i64),5106804296017003664_i64,(-7017536216149168546_i64),2946656220784778701_i64,(-3997160900337974981_i64)];
_3 = _13 / _1.0;
_15 = Adt58::Variant0 { fld0: _2.1 };
_19 = _4.1 ^ _9;
_8 = [4544055508188076980_i64,(-3733913352537049011_i64),(-6935869156809760394_i64),(-5230970574487747930_i64),(-2757702856067598406_i64),(-4375865913274257268_i64),(-307041612793870216_i64)];
_1.0 = _18 as u64;
_4.1 = _2.2 as i32;
_13 = _3 | _3;
_7 = RET;
_10 = _7;
_3 = Field::<i32>(Variant(_15, 0), 0) as u64;
_21 = core::ptr::addr_of_mut!(_6);
_17 = core::ptr::addr_of!(_22);
RET = _10;
Call(_1.1 = fn3(_3, _21, _9, _3, _1.2, _8, _13, _7, Move(_15)), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_6 = [6925608964444618121_i64,9010742774283522125_i64,4966268327848039825_i64,(-8698311913731584613_i64),(-1193762922845908417_i64),(-662171349707922973_i64),(-7440618862791060579_i64)];
_2 = (_13, _12, _1.2);
RET = _10;
RET = _10 + _10;
_13 = !_3;
_4.2 = !_1.2;
_12 = 124487954006666753483284813044789789769_i128 as i32;
_13 = !_14;
_4.0 = !_14;
_4.2 = _1.2 | _1.2;
_12 = 30_u8 as i32;
_13 = !_3;
_4.2 = !_1.2;
_11 = [7599555918287430424_i64,(-5496146314075589544_i64),(-1501565018552144109_i64),(-3881006825249927553_i64),3912345732763666528_i64,(-1392435200021481155_i64),(-6908893142429346227_i64)];
_1.2 = _4.2;
_4.2 = !_1.2;
_12 = -_9;
_2 = (_3, _4.1, _4.2);
_10 = '\u{30c9b}' as u128;
_14 = 9223372036854775807_isize as u64;
_2.1 = _3 as i32;
_16 = _4.1 < _9;
_18 = _4.1 as isize;
Call(RET = core::intrinsics::bswap(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_1 = (_2.0, _19, _16);
_12 = !_27.fld2.1;
(*_17) = [_5,_5];
_24 = _2.2;
_4.0 = _3 | _2.0;
_24 = _4.2;
_26.2 = [_5,_5];
_27.fld1.0 = core::ptr::addr_of_mut!(_27.fld0.1);
_22 = [_5,_5];
_29 = _5 as f32;
_31 = [0_u8,206_u8,31_u8,179_u8,103_u8];
_27.fld2.0 = _4.0 | _4.0;
_32 = 95_u8 as f64;
_26.0 = core::ptr::addr_of!(_27.fld0);
(*_21) = _11;
_28 = [_4.2,_16,_4.2,_1.2,_24,_4.2,_24,_1.2];
_26.1 = (-8059782319177125213_i64) as isize;
Call(_19 = core::intrinsics::transmute(_27.fld2.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_20.0 = core::ptr::addr_of_mut!(_27.fld0.1);
_21 = core::ptr::addr_of_mut!((*_21));
_1.0 = _26.1 as u64;
_26.2 = [_5,_5];
_33 = _18 << _27.fld2.1;
_26.1 = _32 as isize;
_34 = core::ptr::addr_of_mut!(_2.0);
_35 = !3_usize;
_23 = Adt53 { fld0: _2.2 };
_1 = (_13, _9, _27.fld2.2);
_24 = !_1.2;
_25 = _28;
_30 = _7 - _7;
_27.fld1.0 = core::ptr::addr_of_mut!(_27.fld0.1);
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(2_usize, 30_usize, Move(_30), 4_usize, Move(_4), 28_usize, Move(_28), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(2_usize, 31_usize, Move(_31), 8_usize, Move(_8), 35_usize, Move(_35), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(2_usize, 11_usize, Move(_11), 10_usize, Move(_10), 22_usize, Move(_22), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u64,mut _2: *mut [i64; 7],mut _3: i32,mut _4: u64,mut _5: bool,mut _6: [i64; 7],mut _7: u64,mut _8: u128,mut _9: Adt58) -> i32 {
mir! {
type RET = i32;
let _10: u128;
let _11: u64;
let _12: i64;
let _13: i8;
let _14: *mut [i64; 7];
let _15: f32;
let _16: isize;
let _17: char;
let _18: f32;
let _19: isize;
let _20: f32;
let _21: ();
let _22: ();
{
_6 = [153833011761533509_i64,(-8976531396593905764_i64),9214166741804334595_i64,(-9087155691226497039_i64),107179212322068608_i64,(-6601564039811292619_i64),8778243021764115704_i64];
_7 = '\u{70803}' as u64;
(*_2) = _6;
RET = _3;
(*_2) = [(-1142256241666532773_i64),7313076532183183350_i64,3148532721471651018_i64,7865110277354625215_i64,(-2032357203899891782_i64),5206913225508136038_i64,6936529378758252939_i64];
_6 = [1674446936606455602_i64,461228651833022863_i64,(-1866458383367146543_i64),(-6907642675504098699_i64),(-5506242872418900985_i64),9114765548258677169_i64,(-7082598975120064441_i64)];
_3 = '\u{2b791}' as i32;
RET = Field::<i32>(Variant(_9, 0), 0);
_3 = Field::<i32>(Variant(_9, 0), 0);
_4 = !_1;
_10 = !_8;
_2 = core::ptr::addr_of_mut!((*_2));
RET = 104_u8 as i32;
_5 = false;
_2 = core::ptr::addr_of_mut!(_6);
_11 = 22086_i16 as u64;
_10 = !_8;
Goto(bb1)
}
bb1 = {
place!(Field::<i32>(Variant(_9, 0), 0)) = _3;
RET = Field::<i32>(Variant(_9, 0), 0);
_8 = !_10;
_7 = _1;
_5 = true;
_5 = _4 < _1;
_11 = _1;
SetDiscriminant(_9, 3);
_11 = !_1;
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)).1 = RET;
_14 = _2;
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)).2 = _5;
_6 = [7986730065882685409_i64,9222022532546711621_i64,2733664531358808738_i64,(-5919919420048111598_i64),7193020258123795222_i64,5031718118413709920_i64,8320992370288922173_i64];
Call(_4 = fn4(Field::<(u64, i32, bool)>(Variant(_9, 3), 3).2, _5, _2, RET, (*_2), _5, _10, Field::<(u64, i32, bool)>(Variant(_9, 3), 3).1, _2, _7, (*_2), _3, (*_14)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = 24790_u16 as u128;
_3 = RET - RET;
_17 = '\u{b2393}';
_17 = '\u{ca39b}';
_4 = !_7;
_15 = 4079002643_u32 as f32;
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)) = (_7, _3, _5);
_16 = !9223372036854775807_isize;
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)).1 = _3 + _3;
_1 = 57065_u16 as u64;
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)).1 = _17 as i32;
(*_2) = [5645398315020787151_i64,(-6389589370330580433_i64),727762770095664304_i64,(-555032325636195268_i64),5555836627519381203_i64,(-5313163528946383443_i64),8270930997736217669_i64];
_1 = Field::<(u64, i32, bool)>(Variant(_9, 3), 3).0 + _7;
place!(Field::<u64>(Variant(_9, 3), 6)) = Field::<(u64, i32, bool)>(Variant(_9, 3), 3).0 << _1;
_11 = !Field::<(u64, i32, bool)>(Variant(_9, 3), 3).0;
Goto(bb3)
}
bb3 = {
place!(Field::<[bool; 8]>(Variant(_9, 3), 5)) = [_5,_5,_5,Field::<(u64, i32, bool)>(Variant(_9, 3), 3).2,Field::<(u64, i32, bool)>(Variant(_9, 3), 3).2,Field::<(u64, i32, bool)>(Variant(_9, 3), 3).2,_5,Field::<(u64, i32, bool)>(Variant(_9, 3), 3).2];
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)).2 = _5 | _5;
_4 = _7 & Field::<(u64, i32, bool)>(Variant(_9, 3), 3).0;
RET = !_3;
_11 = Field::<(u64, i32, bool)>(Variant(_9, 3), 3).0;
place!(Field::<(u64, i32, bool)>(Variant(_9, 3), 3)).2 = !_5;
_12 = (-4507665074538885153_i64);
RET = 21450137956093866505054418440331614714_i128 as i32;
_1 = _4;
RET = _3;
Goto(bb4)
}
bb4 = {
Call(_21 = dump_var(3_usize, 6_usize, Move(_6), 12_usize, Move(_12), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_21 = dump_var(3_usize, 4_usize, Move(_4), 3_usize, Move(_3), 22_usize, _22, 22_usize, _22), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: bool,mut _3: *mut [i64; 7],mut _4: i32,mut _5: [i64; 7],mut _6: bool,mut _7: u128,mut _8: i32,mut _9: *mut [i64; 7],mut _10: u64,mut _11: [i64; 7],mut _12: i32,mut _13: [i64; 7]) -> u64 {
mir! {
type RET = u64;
let _14: f64;
let _15: Adt53;
let _16: [u16; 1];
let _17: i128;
let _18: ();
let _19: ();
{
_2 = !_1;
_2 = !_1;
_5 = (*_9);
RET = _12 as u64;
_8 = '\u{2fb95}' as i32;
_3 = core::ptr::addr_of_mut!((*_3));
_7 = 61116177246994206363359941389041317619_u128;
_5 = [4645972977101524697_i64,2304488549453679768_i64,5534056495115436397_i64,(-1460043550307793696_i64),2363939692057711057_i64,2167254694440739453_i64,6897842415103196041_i64];
_2 = _1;
_5 = (*_9);
_5 = [(-6584787333693978672_i64),5112782606587352399_i64,(-1214410099103453768_i64),4802951676840415709_i64,2637206672803985472_i64,4880511691839041568_i64,(-4539728744898597141_i64)];
_14 = _7 as f64;
_10 = RET;
RET = !_10;
_10 = RET - RET;
(*_9) = _13;
RET = _7 as u64;
_14 = (-122_isize) as f64;
Call((*_3) = fn5(_9, _1, _3, _5, _5, _1, _3, _9, _6, _12, _12, _3, _9, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (-6_i8) as u128;
_9 = _3;
_9 = _3;
_12 = _4 * _4;
_2 = !_1;
_15.fld0 = !_1;
_15.fld0 = _1;
_9 = core::ptr::addr_of_mut!((*_9));
_7 = 338908662830615224572976127349931736541_u128;
_1 = _10 < _10;
RET = _10 * _10;
_8 = !_12;
_14 = _7 as f64;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(4_usize, 8_usize, Move(_8), 13_usize, Move(_13), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(4_usize, 11_usize, Move(_11), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut [i64; 7],mut _2: bool,mut _3: *mut [i64; 7],mut _4: [i64; 7],mut _5: [i64; 7],mut _6: bool,mut _7: *mut [i64; 7],mut _8: *mut [i64; 7],mut _9: bool,mut _10: i32,mut _11: i32,mut _12: *mut [i64; 7],mut _13: *mut [i64; 7],mut _14: i32) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _15: [u16; 1];
let _16: (&'static u16, [i8; 2], u8, bool);
let _17: u32;
let _18: isize;
let _19: i16;
let _20: [u8; 5];
let _21: Adt49;
let _22: Adt53;
let _23: [i8; 2];
let _24: [u8; 5];
let _25: (u64, i32, bool);
let _26: (u64, i32, bool);
let _27: *const f64;
let _28: i64;
let _29: (u64, i32, bool);
let _30: u64;
let _31: char;
let _32: [i64; 7];
let _33: [bool; 8];
let _34: ();
let _35: ();
{
_13 = _3;
_6 = !_2;
_13 = core::ptr::addr_of_mut!(_4);
_9 = _2 > _6;
_9 = !_2;
Goto(bb1)
}
bb1 = {
_14 = 152479992371389003_usize as i32;
_17 = 89182520747766739161544591957322635220_i128 as u32;
_16.1 = [(-75_i8),(-17_i8)];
_16.3 = _6 < _2;
RET = (*_13);
_1 = _13;
(*_13) = [(-5337998430607864333_i64),7739288917753444114_i64,(-5172518800284672819_i64),(-3094735693291168387_i64),(-6205946028597504142_i64),(-5462008418991907888_i64),(-7057057052104885466_i64)];
(*_1) = [(-3222513556406243969_i64),(-174304035718539025_i64),583778059932083630_i64,6823677983029811061_i64,5962632449521159358_i64,(-4663197454554599656_i64),7767434959525266699_i64];
_6 = _16.3 & _16.3;
RET = (*_1);
_6 = !_9;
(*_13) = [2726492956775184136_i64,(-172169438318634824_i64),(-6735773538668445198_i64),(-5256930870748328421_i64),3522697997821582987_i64,(-7321285343511049330_i64),7662390543836326211_i64];
_16.3 = _6 >= _6;
_4 = [5313525107427573140_i64,26828700740222303_i64,231791562274979986_i64,(-1496235215786252906_i64),(-1600942881507678448_i64),425744546249911018_i64,3282883865436066484_i64];
_18 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_11 = !_10;
_1 = core::ptr::addr_of_mut!(_5);
_16.2 = 97_u8;
_16.2 = 198_u8 << _18;
_16.2 = 224_u8 >> _18;
_14 = !_10;
(*_1) = [(-8388857075426944918_i64),6005275884610773509_i64,1855070073258598600_i64,6150302227080513578_i64,1570185050738828265_i64,9135480458109382653_i64,7731192573602429482_i64];
(*_13) = [(-8442295844495061725_i64),(-7992929103006385531_i64),(-1284336985450072511_i64),(-3879857497789652568_i64),6686547844696091703_i64,8919329267832496993_i64,(-6630246537492004368_i64)];
_20 = [_16.2,_16.2,_16.2,_16.2,_16.2];
Call(_16.1 = fn6(_16.3, _11, _20, _16.3, (*_1), _16.3, _6, _13, _16.3, (*_1), _8, _1, _13, _8, _3, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16.2 = 137_u8 ^ 69_u8;
_14 = _11;
_2 = _16.3;
_9 = _2;
(*_13) = (*_1);
(*_13) = [(-8766030993540662061_i64),552419889570472972_i64,4797492883632232548_i64,5205881566831585742_i64,(-6865839868517402433_i64),8698882797344906367_i64,9187949715350704747_i64];
_13 = core::ptr::addr_of_mut!((*_1));
_6 = _9;
(*_1) = _4;
_4 = [7497139791169995081_i64,(-4782010744218761050_i64),(-6031462932254480105_i64),(-7287614328584085835_i64),(-5973651733213550649_i64),7903461340267989333_i64,(-5108441108033798403_i64)];
_10 = (-20379_i16) as i32;
_19 = (-3_i16) << _10;
_9 = _6;
Goto(bb3)
}
bb3 = {
_1 = _3;
_19 = 1796_i16;
_5 = [(-192682486162181349_i64),(-7081884257614318008_i64),8945318554494973773_i64,(-8177675949672573709_i64),6443099208794532122_i64,6292026252778047647_i64,7145528528825851986_i64];
Call(_16.2 = fn16(_1, _1, _8, _9, _2, _2, _9, _14, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = !_14;
_9 = _16.3;
_1 = _3;
_22 = Adt53 { fld0: _2 };
_4 = [(-6894395202145586261_i64),(-2672506677830572676_i64),3931317032366914141_i64,(-6943733129428307164_i64),3420690716576499983_i64,492335680937085056_i64,1896256069777833064_i64];
_23 = _16.1;
(*_13) = [4785251018767864730_i64,(-4435383189442719248_i64),181257679897228238_i64,(-6664678663815404141_i64),(-4579893532227394304_i64),(-9198744619930498246_i64),2899903367400060737_i64];
_17 = 1980615455_u32 * 3358204227_u32;
RET = (*_13);
_7 = _12;
_13 = _3;
_24 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_16.3 = _9 <= _22.fld0;
_15 = [16676_u16];
_1 = core::ptr::addr_of_mut!(RET);
_3 = core::ptr::addr_of_mut!(_4);
_18 = _17 as isize;
_26 = (957106076874598131_u64, _11, _2);
_23 = _16.1;
(*_1) = _4;
_4 = [(-8244166611251650132_i64),(-6722333483901818096_i64),(-1041906261143853588_i64),8440920242299754102_i64,2034247063322860182_i64,9124173357935670813_i64,2900850607810625452_i64];
RET = [(-5191637480876704271_i64),6560573944346553299_i64,3523693829144354493_i64,7126958554212531057_i64,(-6004137451763875713_i64),(-7313759574743115116_i64),4053062227877913948_i64];
_17 = 1759197198_u32 ^ 859032941_u32;
_1 = core::ptr::addr_of_mut!((*_3));
match _26.0 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
957106076874598131 => bb9,
_ => bb8
}
}
bb5 = {
_1 = _3;
_19 = 1796_i16;
_5 = [(-192682486162181349_i64),(-7081884257614318008_i64),8945318554494973773_i64,(-8177675949672573709_i64),6443099208794532122_i64,6292026252778047647_i64,7145528528825851986_i64];
Call(_16.2 = fn16(_1, _1, _8, _9, _2, _2, _9, _14, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_16.2 = 137_u8 ^ 69_u8;
_14 = _11;
_2 = _16.3;
_9 = _2;
(*_13) = (*_1);
(*_13) = [(-8766030993540662061_i64),552419889570472972_i64,4797492883632232548_i64,5205881566831585742_i64,(-6865839868517402433_i64),8698882797344906367_i64,9187949715350704747_i64];
_13 = core::ptr::addr_of_mut!((*_1));
_6 = _9;
(*_1) = _4;
_4 = [7497139791169995081_i64,(-4782010744218761050_i64),(-6031462932254480105_i64),(-7287614328584085835_i64),(-5973651733213550649_i64),7903461340267989333_i64,(-5108441108033798403_i64)];
_10 = (-20379_i16) as i32;
_19 = (-3_i16) << _10;
_9 = _6;
Goto(bb3)
}
bb7 = {
_14 = 152479992371389003_usize as i32;
_17 = 89182520747766739161544591957322635220_i128 as u32;
_16.1 = [(-75_i8),(-17_i8)];
_16.3 = _6 < _2;
RET = (*_13);
_1 = _13;
(*_13) = [(-5337998430607864333_i64),7739288917753444114_i64,(-5172518800284672819_i64),(-3094735693291168387_i64),(-6205946028597504142_i64),(-5462008418991907888_i64),(-7057057052104885466_i64)];
(*_1) = [(-3222513556406243969_i64),(-174304035718539025_i64),583778059932083630_i64,6823677983029811061_i64,5962632449521159358_i64,(-4663197454554599656_i64),7767434959525266699_i64];
_6 = _16.3 & _16.3;
RET = (*_1);
_6 = !_9;
(*_13) = [2726492956775184136_i64,(-172169438318634824_i64),(-6735773538668445198_i64),(-5256930870748328421_i64),3522697997821582987_i64,(-7321285343511049330_i64),7662390543836326211_i64];
_16.3 = _6 >= _6;
_4 = [5313525107427573140_i64,26828700740222303_i64,231791562274979986_i64,(-1496235215786252906_i64),(-1600942881507678448_i64),425744546249911018_i64,3282883865436066484_i64];
_18 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_11 = !_10;
_1 = core::ptr::addr_of_mut!(_5);
_16.2 = 97_u8;
_16.2 = 198_u8 << _18;
_16.2 = 224_u8 >> _18;
_14 = !_10;
(*_1) = [(-8388857075426944918_i64),6005275884610773509_i64,1855070073258598600_i64,6150302227080513578_i64,1570185050738828265_i64,9135480458109382653_i64,7731192573602429482_i64];
(*_13) = [(-8442295844495061725_i64),(-7992929103006385531_i64),(-1284336985450072511_i64),(-3879857497789652568_i64),6686547844696091703_i64,8919329267832496993_i64,(-6630246537492004368_i64)];
_20 = [_16.2,_16.2,_16.2,_16.2,_16.2];
Call(_16.1 = fn6(_16.3, _11, _20, _16.3, (*_1), _16.3, _6, _13, _16.3, (*_1), _8, _1, _13, _8, _3, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_14 = !_11;
_19 = -(-15383_i16);
_26.0 = 3121505909832086847_u64;
_2 = !_16.3;
_22.fld0 = _9;
_5 = (*_1);
_26.0 = 14667844705194242349_u64;
RET = (*_1);
_1 = _12;
_24 = _20;
_26.2 = _2 | _2;
RET = [(-1434225931787686794_i64),(-835716832542508797_i64),8343821852987030971_i64,(-8372223493563830013_i64),7678029381754426840_i64,3669250499679682780_i64,4742163291710576150_i64];
_6 = _26.2;
_25 = (_26.0, _11, _26.2);
_14 = _10;
_17 = 39489904_u32 + 1687150418_u32;
_15 = [37121_u16];
RET = _5;
Goto(bb10)
}
bb10 = {
_16.1 = [(-22_i8),(-43_i8)];
_16.2 = 241_u8 & 109_u8;
_12 = core::ptr::addr_of_mut!(_5);
_26.0 = _25.0;
Call(_19 = fn19(_26.1, _13, _22, _25.0, _16.3, _25), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_25.1 = _18 as i32;
_20 = _24;
_28 = 8261266208853146732_i64 >> _19;
_26 = (_25.0, _11, _6);
_16.1 = [(-49_i8),(-67_i8)];
Goto(bb12)
}
bb12 = {
(*_3) = _5;
_14 = _25.1 >> _19;
_29 = _26;
_4 = [_28,_28,_28,_28,_28,_28,_28];
RET = [_28,_28,_28,_28,_28,_28,_28];
_2 = _26.2 & _25.2;
_4 = (*_12);
_22 = Adt53 { fld0: _26.2 };
_17 = _14 as u32;
RET = _4;
_31 = '\u{e3dbb}';
_22 = Adt53 { fld0: _6 };
_25.2 = !_22.fld0;
(*_12) = [_28,_28,_28,_28,_28,_28,_28];
_9 = _2;
RET = [_28,_28,_28,_28,_28,_28,_28];
_5 = [_28,_28,_28,_28,_28,_28,_28];
_15 = [683_u16];
Goto(bb13)
}
bb13 = {
Call(_34 = dump_var(5_usize, 23_usize, Move(_23), 17_usize, Move(_17), 20_usize, Move(_20), 31_usize, Move(_31)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_34 = dump_var(5_usize, 19_usize, Move(_19), 14_usize, Move(_14), 25_usize, Move(_25), 9_usize, Move(_9)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_34 = dump_var(5_usize, 2_usize, Move(_2), 10_usize, Move(_10), 35_usize, _35, 35_usize, _35), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: i32,mut _3: [u8; 5],mut _4: bool,mut _5: [i64; 7],mut _6: bool,mut _7: bool,mut _8: *mut [i64; 7],mut _9: bool,mut _10: [i64; 7],mut _11: *mut [i64; 7],mut _12: *mut [i64; 7],mut _13: *mut [i64; 7],mut _14: *mut [i64; 7],mut _15: *mut [i64; 7],mut _16: i32) -> [i8; 2] {
mir! {
type RET = [i8; 2];
let _17: [bool; 8];
let _18: Adt50;
let _19: (u64, i32, bool);
let _20: u64;
let _21: [bool; 8];
let _22: char;
let _23: f32;
let _24: *const (u128, *mut char, *mut i64);
let _25: i32;
let _26: isize;
let _27: Adt47;
let _28: Adt47;
let _29: u128;
let _30: isize;
let _31: (*const (u128, *mut char, *mut i64), [i8; 2], *const [i8; 2], *const (u128, *mut char, *mut i64), bool);
let _32: bool;
let _33: isize;
let _34: usize;
let _35: bool;
let _36: bool;
let _37: [u16; 1];
let _38: [u8; 5];
let _39: f64;
let _40: *mut *mut char;
let _41: *const (u128, *mut char, *mut i64);
let _42: char;
let _43: bool;
let _44: [u16; 1];
let _45: *mut u64;
let _46: *mut u32;
let _47: Adt47;
let _48: ();
let _49: ();
{
_14 = core::ptr::addr_of_mut!(_10);
_9 = _1 >= _1;
_13 = _15;
RET = [(-62_i8),114_i8];
_17 = [_9,_1,_1,_9,_4,_6,_1,_9];
(*_8) = [6047439232801209378_i64,(-555778300954149105_i64),954678938814529663_i64,(-1153769653915693468_i64),(-4078878489601207039_i64),(-6483146475788844329_i64),(-6587036651688350983_i64)];
_3 = [231_u8,24_u8,194_u8,28_u8,40_u8];
_13 = core::ptr::addr_of_mut!((*_12));
Call(_10 = fn7(_9, _9, _17), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = !_9;
_17 = [_1,_7,_7,_9,_7,_9,_4,_7];
_1 = _7;
(*_12) = [(-5584008623661503672_i64),54811769639447572_i64,3206018626373065634_i64,4515123689776989642_i64,(-4782616583267938668_i64),(-6349123789858537212_i64),7869415757146038673_i64];
_7 = !_1;
_1 = !_7;
RET = [16_i8,(-72_i8)];
(*_8) = (*_12);
_10 = (*_12);
_7 = _1 > _9;
(*_14) = [2046240086039518808_i64,(-4636860469614387163_i64),(-1431756715184450738_i64),(-6048059243010607706_i64),7273401934622324482_i64,3301574369059224513_i64,(-8040844111351438639_i64)];
(*_14) = [(-474333522763494164_i64),5254828497727654506_i64,(-6363490513547966574_i64),3879702953150674689_i64,(-3362300972388899674_i64),(-2605749489094272257_i64),3102567002509250074_i64];
_7 = !_1;
_10 = [(-8735685274382933843_i64),3769976870846162762_i64,1895134785650814811_i64,8404978819679889503_i64,4185654801167641273_i64,(-6684569391707546334_i64),(-2632110396450306786_i64)];
_8 = core::ptr::addr_of_mut!((*_14));
RET = [62_i8,16_i8];
(*_12) = (*_8);
(*_14) = (*_13);
_5 = [701886969122548162_i64,5237927827172664512_i64,(-2985941255609409670_i64),(-2777061536578654948_i64),6379019065607078966_i64,5125790893540463274_i64,697944776627765813_i64];
Goto(bb2)
}
bb2 = {
_15 = core::ptr::addr_of_mut!(_5);
_13 = _12;
_8 = core::ptr::addr_of_mut!((*_8));
_14 = _12;
(*_13) = [6342128608770844237_i64,314234133232557659_i64,(-7671120057986537388_i64),(-6171324806728984287_i64),6261117277360407700_i64,8456936722500724997_i64,7698097373455095033_i64];
_5 = [3240632019975092448_i64,(-1598495799714336088_i64),(-6437809732349392151_i64),8147199289526543191_i64,1746356648611847452_i64,(-8318432279901168905_i64),(-8129156879635855029_i64)];
(*_15) = [(-44841630573400794_i64),(-2802937641077149516_i64),(-815065263150630286_i64),(-7952891793532698819_i64),1602046268967163790_i64,2718432083154514691_i64,(-531297015066690419_i64)];
_8 = core::ptr::addr_of_mut!((*_8));
_5 = (*_8);
_5 = [(-4381625145900827040_i64),357662374686289959_i64,6578701168015645581_i64,(-6281655778306095890_i64),(-8062397674109518992_i64),(-4920045810611663908_i64),(-9101187633187805265_i64)];
(*_8) = [(-6868200053781950098_i64),3423372099100015331_i64,(-705115824957300489_i64),(-5837160960460804132_i64),(-4415398442610477579_i64),(-9221331309583288558_i64),7804116525527923773_i64];
(*_14) = [635439918960766464_i64,3264911218443253785_i64,122154728456213904_i64,3548166875373462892_i64,3730703462414716238_i64,(-3095738313177749525_i64),(-246818686265956344_i64)];
_10 = [(-769328967561151239_i64),9108807000373570237_i64,4587607408175069213_i64,8601300476892170414_i64,4258933484599027223_i64,(-5310622073609306352_i64),(-115707628450338356_i64)];
(*_15) = (*_12);
_17 = [_9,_9,_7,_9,_1,_1,_7,_9];
_2 = -_16;
(*_13) = [9073598368770829132_i64,7306502867112475849_i64,5930721890160587342_i64,7947791082263726992_i64,6839301437726431246_i64,(-5272129301868370466_i64),8512953404611821097_i64];
(*_8) = (*_13);
_3 = [55_u8,218_u8,7_u8,20_u8,117_u8];
(*_12) = [1495892781024866965_i64,(-5711461291856918615_i64),(-6682789850502638409_i64),1284416311650056612_i64,1952375410608522859_i64,2492761431648288767_i64,(-2058629961323527033_i64)];
(*_8) = [(-9071886931313266915_i64),(-762515160043829994_i64),(-7502180586678603258_i64),650060488250407997_i64,(-7279885231426196968_i64),(-2045362710111529344_i64),(-3136334225193273793_i64)];
_19.1 = _2 & _16;
_19.0 = '\u{90593}' as u64;
_11 = core::ptr::addr_of_mut!((*_12));
(*_11) = [(-7222223707411930383_i64),2206522799254480553_i64,(-2595452408278306497_i64),(-960863646117126864_i64),(-6505518088824199808_i64),5386451376793738069_i64,(-5902931428959203354_i64)];
(*_13) = [7338762284879404766_i64,2724392485955155267_i64,(-4084970273242278303_i64),(-7804815579452750670_i64),(-8185629298247574984_i64),5678425604226398835_i64,7835309439070008399_i64];
Goto(bb3)
}
bb3 = {
(*_11) = [2316147951971091571_i64,2465687278573723132_i64,6097775433010778521_i64,(-4349015965263662435_i64),2068985045324665187_i64,4855976097028888664_i64,(-6102384605779810158_i64)];
_5 = (*_11);
_19.1 = 312958470264979650919740280392462774794_u128 as i32;
(*_13) = [1970121018984250301_i64,(-5888019190155343289_i64),1630834663447161258_i64,8264178839299584311_i64,(-2224217641673021084_i64),(-2217973429095045656_i64),(-8454509423529939611_i64)];
(*_15) = [4882551784926602027_i64,(-6160223486732584335_i64),(-5819169927868390724_i64),(-2770291757500319714_i64),3394555604730104730_i64,(-1149954335658569064_i64),8698867565624442090_i64];
(*_14) = [(-8872774457630151259_i64),(-4410678243893199594_i64),7550565599464531418_i64,5945282660436365596_i64,7555363395265586327_i64,2618978337862945728_i64,5169048771529868336_i64];
_22 = '\u{3c818}';
_5 = (*_12);
_3 = [129_u8,97_u8,155_u8,246_u8,58_u8];
_20 = _19.0;
(*_12) = [(-6484868637902062940_i64),6898785996531312956_i64,4802898960227366006_i64,6820939162669462166_i64,1227378743449493771_i64,763955729625672313_i64,(-1158255785403857351_i64)];
(*_14) = [4445205329025212336_i64,(-4069509952638994935_i64),(-2292120991218279335_i64),8100113583053295603_i64,2235898251448259549_i64,(-5805319762772822654_i64),3622753134008448807_i64];
(*_13) = [8047448736839293870_i64,(-8817307773443627642_i64),(-1122404323509648326_i64),108782392142032088_i64,(-7859559111402806673_i64),2152805899433753439_i64,8940475983762669623_i64];
_4 = !_7;
(*_15) = (*_12);
_8 = _11;
(*_15) = [(-241907243548444010_i64),(-4738878005939448897_i64),7026243187387755646_i64,4943564766514707061_i64,6994621861615541587_i64,(-87949766911737072_i64),3492807809057939970_i64];
_16 = _2;
_6 = _9;
(*_8) = [7481787967363661582_i64,2385605180894903436_i64,(-1612841705504027214_i64),1498855575416328043_i64,(-3667537314891795873_i64),(-2684192305181656353_i64),8384021552402272342_i64];
(*_11) = [(-7154978285937246930_i64),(-7921090401632981447_i64),(-6525188285165968533_i64),7416665690989814147_i64,(-1913040085364699104_i64),(-2733693459041788217_i64),8137081223018287484_i64];
Goto(bb4)
}
bb4 = {
_5 = (*_13);
_3 = [223_u8,140_u8,136_u8,181_u8,214_u8];
_19 = (_20, _2, _9);
(*_13) = [(-3914543172539861647_i64),8617833063681904702_i64,8470213727225699362_i64,1505852897809926793_i64,(-3014254497155920651_i64),(-3054133749132055554_i64),(-4035909325440900845_i64)];
_19 = (_20, _2, _9);
_25 = _16;
_19.2 = !_9;
RET = [(-19_i8),1_i8];
_28 = Adt47::Variant1 { fld0: 1400224482_u32 };
_3 = [148_u8,35_u8,194_u8,47_u8,181_u8];
place!(Field::<u32>(Variant(_28, 1), 0)) = 43854_u16 as u32;
_22 = '\u{6de2c}';
_23 = 53406_u16 as f32;
_6 = _19.2 ^ _4;
_31.2 = core::ptr::addr_of!(RET);
_26 = !(-9223372036854775808_isize);
(*_11) = _10;
RET = [61_i8,89_i8];
_31.4 = _7 >= _1;
_20 = _19.0;
_20 = !_19.0;
Goto(bb5)
}
bb5 = {
_15 = _8;
(*_12) = _5;
(*_8) = [(-5754289268238319039_i64),6456484731750846578_i64,(-3849702171400413750_i64),(-3642252326383895412_i64),2641526265933538542_i64,866004540717406676_i64,(-4923195625782303700_i64)];
_19.2 = _6 > _31.4;
_3 = [182_u8,52_u8,107_u8,122_u8,106_u8];
_10 = (*_15);
_1 = _9;
_30 = _6 as isize;
_32 = _4;
_6 = _19.2 > _31.4;
(*_8) = _5;
_9 = _31.4 | _32;
_29 = (-110_i8) as u128;
_21 = [_1,_19.2,_19.2,_4,_31.4,_19.2,_4,_31.4];
_23 = _30 as f32;
_26 = !_30;
RET = [54_i8,41_i8];
_4 = !_6;
SetDiscriminant(_28, 1);
_28 = Adt47::Variant2 { fld0: _29,fld1: 13360_u16 };
_19.2 = _32;
(*_11) = _5;
Goto(bb6)
}
bb6 = {
(*_11) = [5182997976324317891_i64,3928431070644660677_i64,7444247366657361166_i64,(-8949951369806168275_i64),(-922855171573320850_i64),6236168250071624186_i64,4715170848206696840_i64];
_25 = !_19.1;
(*_12) = [(-613851139333252472_i64),2943690883396936186_i64,1655072177942710461_i64,3161652565132684495_i64,284141783067775668_i64,(-5739564081965959390_i64),6323225341321725771_i64];
_19.0 = !_20;
_30 = 63244_u16 as isize;
_28 = Adt47::Variant2 { fld0: _29,fld1: 55893_u16 };
_28 = Adt47::Variant3 { fld0: _19.0 };
_31.2 = core::ptr::addr_of!(_31.1);
_19.1 = _25;
(*_12) = [(-2413545145730302104_i64),(-8343532725549002107_i64),7705596084803679818_i64,424030949746503587_i64,3562717883035041012_i64,(-2250391353762758910_i64),(-696846205088842488_i64)];
_10 = [(-2594719386697392029_i64),4376979818056560409_i64,8105444007057354946_i64,(-5484245290613477864_i64),4372051629194790494_i64,(-1921524153459430156_i64),(-7721378524757202027_i64)];
_22 = '\u{48945}';
SetDiscriminant(_28, 0);
_28 = Adt47::Variant2 { fld0: _29,fld1: 2521_u16 };
Goto(bb7)
}
bb7 = {
_5 = _10;
(*_13) = _5;
_37 = [61997_u16];
(*_11) = [(-1336207018452009704_i64),(-6987837798126316363_i64),3171696485921125257_i64,(-1571475974756931518_i64),6264905963808827191_i64,4360733301090898766_i64,(-156094536961093383_i64)];
_12 = _8;
_32 = _9;
_5 = (*_15);
_36 = _6 == _19.2;
_35 = _31.4;
(*_11) = _5;
place!(Field::<u16>(Variant(_28, 2), 1)) = _22 as u16;
RET = [75_i8,(-29_i8)];
SetDiscriminant(_28, 3);
_1 = _36 ^ _6;
RET = [93_i8,(-45_i8)];
(*_13) = [8450426331200324795_i64,4838129235648508634_i64,(-1585606070920786656_i64),(-8132561847378241612_i64),(-4465627797895118864_i64),(-756532030639746248_i64),(-1838563425973163598_i64)];
_8 = core::ptr::addr_of_mut!((*_13));
Goto(bb8)
}
bb8 = {
_1 = _35 & _9;
(*_8) = [2417072662458482552_i64,7180313569295765114_i64,(-2045716845624678247_i64),(-8023158278347040895_i64),(-8318088169180948304_i64),(-3182700230409654443_i64),5229484156125796639_i64];
(*_13) = [8152001391271692256_i64,7470334986875119545_i64,(-5415567238207151379_i64),3810808094542768503_i64,2715381535780058174_i64,(-8716464986638922414_i64),(-798968795968092031_i64)];
(*_12) = [(-2298461463757993974_i64),5758965406888875842_i64,9106058899278588518_i64,(-2420387617023603967_i64),5190973948908725961_i64,(-2657955660409686700_i64),(-290182593026898096_i64)];
_36 = !_32;
_35 = !_9;
(*_12) = [6430326710654579143_i64,(-250293252844906633_i64),(-2370390996005570095_i64),8257970538104270283_i64,(-4291423003201636322_i64),(-1906859161637390876_i64),(-3465659150897625512_i64)];
_23 = 27_u8 as f32;
_38 = [2_u8,174_u8,70_u8,10_u8,94_u8];
_30 = !_26;
place!(Field::<u64>(Variant(_28, 3), 0)) = _20;
_21 = [_6,_32,_6,_9,_35,_1,_6,_32];
_31.4 = !_6;
Goto(bb9)
}
bb9 = {
_31.1 = RET;
_27 = Adt47::Variant1 { fld0: 1105148010_u32 };
(*_8) = [(-355446951243326040_i64),(-7876371235840057302_i64),6900245905840762892_i64,(-1168062610080291796_i64),8664640444368311653_i64,(-4164350490773948692_i64),(-8672996784020794640_i64)];
_2 = _25;
_5 = [3260822906725198188_i64,(-3456030455248028532_i64),3880899877429336722_i64,(-896115218180102708_i64),(-6372606723030929853_i64),5818884453139324853_i64,8131478075448132469_i64];
Goto(bb10)
}
bb10 = {
_37 = [7780_u16];
_27 = Adt47::Variant0 { fld0: _26,fld1: _19.0 };
Goto(bb11)
}
bb11 = {
_26 = Field::<isize>(Variant(_27, 0), 0);
_25 = _16;
SetDiscriminant(_28, 0);
_33 = Field::<isize>(Variant(_27, 0), 0);
_7 = !_4;
_2 = 4319226577007038418_i64 as i32;
_17 = [_7,_19.2,_31.4,_32,_36,_31.4,_36,_4];
_17 = [_35,_19.2,_4,_4,_9,_32,_31.4,_31.4];
(*_11) = [(-483641846551877354_i64),(-6636266918257547507_i64),(-3601742349414683898_i64),(-5202416347899560421_i64),4259067383111103814_i64,2450993445424395782_i64,(-1142140777929570077_i64)];
_31.2 = core::ptr::addr_of!(RET);
_20 = !_19.0;
_42 = _22;
_26 = _30;
_29 = 96075872073083006104932513815452142357_u128;
SetDiscriminant(_27, 2);
_15 = core::ptr::addr_of_mut!((*_8));
place!(Field::<isize>(Variant(_28, 0), 0)) = !_30;
_23 = _19.0 as f32;
_1 = !_9;
match _29 {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
96075872073083006104932513815452142357 => bb18,
_ => bb17
}
}
bb12 = {
_15 = core::ptr::addr_of_mut!(_5);
_13 = _12;
_8 = core::ptr::addr_of_mut!((*_8));
_14 = _12;
(*_13) = [6342128608770844237_i64,314234133232557659_i64,(-7671120057986537388_i64),(-6171324806728984287_i64),6261117277360407700_i64,8456936722500724997_i64,7698097373455095033_i64];
_5 = [3240632019975092448_i64,(-1598495799714336088_i64),(-6437809732349392151_i64),8147199289526543191_i64,1746356648611847452_i64,(-8318432279901168905_i64),(-8129156879635855029_i64)];
(*_15) = [(-44841630573400794_i64),(-2802937641077149516_i64),(-815065263150630286_i64),(-7952891793532698819_i64),1602046268967163790_i64,2718432083154514691_i64,(-531297015066690419_i64)];
_8 = core::ptr::addr_of_mut!((*_8));
_5 = (*_8);
_5 = [(-4381625145900827040_i64),357662374686289959_i64,6578701168015645581_i64,(-6281655778306095890_i64),(-8062397674109518992_i64),(-4920045810611663908_i64),(-9101187633187805265_i64)];
(*_8) = [(-6868200053781950098_i64),3423372099100015331_i64,(-705115824957300489_i64),(-5837160960460804132_i64),(-4415398442610477579_i64),(-9221331309583288558_i64),7804116525527923773_i64];
(*_14) = [635439918960766464_i64,3264911218443253785_i64,122154728456213904_i64,3548166875373462892_i64,3730703462414716238_i64,(-3095738313177749525_i64),(-246818686265956344_i64)];
_10 = [(-769328967561151239_i64),9108807000373570237_i64,4587607408175069213_i64,8601300476892170414_i64,4258933484599027223_i64,(-5310622073609306352_i64),(-115707628450338356_i64)];
(*_15) = (*_12);
_17 = [_9,_9,_7,_9,_1,_1,_7,_9];
_2 = -_16;
(*_13) = [9073598368770829132_i64,7306502867112475849_i64,5930721890160587342_i64,7947791082263726992_i64,6839301437726431246_i64,(-5272129301868370466_i64),8512953404611821097_i64];
(*_8) = (*_13);
_3 = [55_u8,218_u8,7_u8,20_u8,117_u8];
(*_12) = [1495892781024866965_i64,(-5711461291856918615_i64),(-6682789850502638409_i64),1284416311650056612_i64,1952375410608522859_i64,2492761431648288767_i64,(-2058629961323527033_i64)];
(*_8) = [(-9071886931313266915_i64),(-762515160043829994_i64),(-7502180586678603258_i64),650060488250407997_i64,(-7279885231426196968_i64),(-2045362710111529344_i64),(-3136334225193273793_i64)];
_19.1 = _2 & _16;
_19.0 = '\u{90593}' as u64;
_11 = core::ptr::addr_of_mut!((*_12));
(*_11) = [(-7222223707411930383_i64),2206522799254480553_i64,(-2595452408278306497_i64),(-960863646117126864_i64),(-6505518088824199808_i64),5386451376793738069_i64,(-5902931428959203354_i64)];
(*_13) = [7338762284879404766_i64,2724392485955155267_i64,(-4084970273242278303_i64),(-7804815579452750670_i64),(-8185629298247574984_i64),5678425604226398835_i64,7835309439070008399_i64];
Goto(bb3)
}
bb13 = {
_31.1 = RET;
_27 = Adt47::Variant1 { fld0: 1105148010_u32 };
(*_8) = [(-355446951243326040_i64),(-7876371235840057302_i64),6900245905840762892_i64,(-1168062610080291796_i64),8664640444368311653_i64,(-4164350490773948692_i64),(-8672996784020794640_i64)];
_2 = _25;
_5 = [3260822906725198188_i64,(-3456030455248028532_i64),3880899877429336722_i64,(-896115218180102708_i64),(-6372606723030929853_i64),5818884453139324853_i64,8131478075448132469_i64];
Goto(bb10)
}
bb14 = {
_7 = !_9;
_17 = [_1,_7,_7,_9,_7,_9,_4,_7];
_1 = _7;
(*_12) = [(-5584008623661503672_i64),54811769639447572_i64,3206018626373065634_i64,4515123689776989642_i64,(-4782616583267938668_i64),(-6349123789858537212_i64),7869415757146038673_i64];
_7 = !_1;
_1 = !_7;
RET = [16_i8,(-72_i8)];
(*_8) = (*_12);
_10 = (*_12);
_7 = _1 > _9;
(*_14) = [2046240086039518808_i64,(-4636860469614387163_i64),(-1431756715184450738_i64),(-6048059243010607706_i64),7273401934622324482_i64,3301574369059224513_i64,(-8040844111351438639_i64)];
(*_14) = [(-474333522763494164_i64),5254828497727654506_i64,(-6363490513547966574_i64),3879702953150674689_i64,(-3362300972388899674_i64),(-2605749489094272257_i64),3102567002509250074_i64];
_7 = !_1;
_10 = [(-8735685274382933843_i64),3769976870846162762_i64,1895134785650814811_i64,8404978819679889503_i64,4185654801167641273_i64,(-6684569391707546334_i64),(-2632110396450306786_i64)];
_8 = core::ptr::addr_of_mut!((*_14));
RET = [62_i8,16_i8];
(*_12) = (*_8);
(*_14) = (*_13);
_5 = [701886969122548162_i64,5237927827172664512_i64,(-2985941255609409670_i64),(-2777061536578654948_i64),6379019065607078966_i64,5125790893540463274_i64,697944776627765813_i64];
Goto(bb2)
}
bb15 = {
_5 = _10;
(*_13) = _5;
_37 = [61997_u16];
(*_11) = [(-1336207018452009704_i64),(-6987837798126316363_i64),3171696485921125257_i64,(-1571475974756931518_i64),6264905963808827191_i64,4360733301090898766_i64,(-156094536961093383_i64)];
_12 = _8;
_32 = _9;
_5 = (*_15);
_36 = _6 == _19.2;
_35 = _31.4;
(*_11) = _5;
place!(Field::<u16>(Variant(_28, 2), 1)) = _22 as u16;
RET = [75_i8,(-29_i8)];
SetDiscriminant(_28, 3);
_1 = _36 ^ _6;
RET = [93_i8,(-45_i8)];
(*_13) = [8450426331200324795_i64,4838129235648508634_i64,(-1585606070920786656_i64),(-8132561847378241612_i64),(-4465627797895118864_i64),(-756532030639746248_i64),(-1838563425973163598_i64)];
_8 = core::ptr::addr_of_mut!((*_13));
Goto(bb8)
}
bb16 = {
(*_11) = [5182997976324317891_i64,3928431070644660677_i64,7444247366657361166_i64,(-8949951369806168275_i64),(-922855171573320850_i64),6236168250071624186_i64,4715170848206696840_i64];
_25 = !_19.1;
(*_12) = [(-613851139333252472_i64),2943690883396936186_i64,1655072177942710461_i64,3161652565132684495_i64,284141783067775668_i64,(-5739564081965959390_i64),6323225341321725771_i64];
_19.0 = !_20;
_30 = 63244_u16 as isize;
_28 = Adt47::Variant2 { fld0: _29,fld1: 55893_u16 };
_28 = Adt47::Variant3 { fld0: _19.0 };
_31.2 = core::ptr::addr_of!(_31.1);
_19.1 = _25;
(*_12) = [(-2413545145730302104_i64),(-8343532725549002107_i64),7705596084803679818_i64,424030949746503587_i64,3562717883035041012_i64,(-2250391353762758910_i64),(-696846205088842488_i64)];
_10 = [(-2594719386697392029_i64),4376979818056560409_i64,8105444007057354946_i64,(-5484245290613477864_i64),4372051629194790494_i64,(-1921524153459430156_i64),(-7721378524757202027_i64)];
_22 = '\u{48945}';
SetDiscriminant(_28, 0);
_28 = Adt47::Variant2 { fld0: _29,fld1: 2521_u16 };
Goto(bb7)
}
bb17 = {
(*_11) = [2316147951971091571_i64,2465687278573723132_i64,6097775433010778521_i64,(-4349015965263662435_i64),2068985045324665187_i64,4855976097028888664_i64,(-6102384605779810158_i64)];
_5 = (*_11);
_19.1 = 312958470264979650919740280392462774794_u128 as i32;
(*_13) = [1970121018984250301_i64,(-5888019190155343289_i64),1630834663447161258_i64,8264178839299584311_i64,(-2224217641673021084_i64),(-2217973429095045656_i64),(-8454509423529939611_i64)];
(*_15) = [4882551784926602027_i64,(-6160223486732584335_i64),(-5819169927868390724_i64),(-2770291757500319714_i64),3394555604730104730_i64,(-1149954335658569064_i64),8698867565624442090_i64];
(*_14) = [(-8872774457630151259_i64),(-4410678243893199594_i64),7550565599464531418_i64,5945282660436365596_i64,7555363395265586327_i64,2618978337862945728_i64,5169048771529868336_i64];
_22 = '\u{3c818}';
_5 = (*_12);
_3 = [129_u8,97_u8,155_u8,246_u8,58_u8];
_20 = _19.0;
(*_12) = [(-6484868637902062940_i64),6898785996531312956_i64,4802898960227366006_i64,6820939162669462166_i64,1227378743449493771_i64,763955729625672313_i64,(-1158255785403857351_i64)];
(*_14) = [4445205329025212336_i64,(-4069509952638994935_i64),(-2292120991218279335_i64),8100113583053295603_i64,2235898251448259549_i64,(-5805319762772822654_i64),3622753134008448807_i64];
(*_13) = [8047448736839293870_i64,(-8817307773443627642_i64),(-1122404323509648326_i64),108782392142032088_i64,(-7859559111402806673_i64),2152805899433753439_i64,8940475983762669623_i64];
_4 = !_7;
(*_15) = (*_12);
_8 = _11;
(*_15) = [(-241907243548444010_i64),(-4738878005939448897_i64),7026243187387755646_i64,4943564766514707061_i64,6994621861615541587_i64,(-87949766911737072_i64),3492807809057939970_i64];
_16 = _2;
_6 = _9;
(*_8) = [7481787967363661582_i64,2385605180894903436_i64,(-1612841705504027214_i64),1498855575416328043_i64,(-3667537314891795873_i64),(-2684192305181656353_i64),8384021552402272342_i64];
(*_11) = [(-7154978285937246930_i64),(-7921090401632981447_i64),(-6525188285165968533_i64),7416665690989814147_i64,(-1913040085364699104_i64),(-2733693459041788217_i64),8137081223018287484_i64];
Goto(bb4)
}
bb18 = {
_15 = core::ptr::addr_of_mut!(_10);
_1 = !_35;
(*_12) = [543831722050215753_i64,773328650943272477_i64,(-5871586345709331786_i64),3234034412502997719_i64,6223897003070446502_i64,5834835608439209965_i64,(-4168719794911059255_i64)];
_42 = _22;
(*_12) = [4003785288928193539_i64,(-4216116811585519582_i64),(-1913065107416895423_i64),(-8125828638023023858_i64),5674717592539976068_i64,(-5615471644577255121_i64),5788989551277175286_i64];
_23 = 9269_i16 as f32;
_31.2 = core::ptr::addr_of!(_31.1);
(*_13) = [8861203450810965584_i64,7281512037208979215_i64,(-427614716329721853_i64),5084067174507511254_i64,(-3116388557698222992_i64),(-1986265941804796054_i64),(-313425447017650944_i64)];
_39 = _16 as f64;
_31.4 = _6 > _32;
(*_14) = [250477740204747071_i64,4015578514295848858_i64,2200353307106487922_i64,(-3885350872737643342_i64),(-5440569632707219999_i64),4796089246203132182_i64,(-1391574592939747520_i64)];
_19.1 = _16;
Goto(bb19)
}
bb19 = {
Call(_48 = dump_var(6_usize, 22_usize, Move(_22), 2_usize, Move(_2), 26_usize, Move(_26), 17_usize, Move(_17)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(6_usize, 33_usize, Move(_33), 10_usize, Move(_10), 16_usize, Move(_16), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_48 = dump_var(6_usize, 25_usize, Move(_25), 4_usize, Move(_4), 35_usize, Move(_35), 32_usize, Move(_32)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_48 = dump_var(6_usize, 1_usize, Move(_1), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: bool,mut _3: [bool; 8]) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _4: [i64; 7];
let _5: [u8; 5];
let _6: [i8; 2];
let _7: (u64, i32, bool);
let _8: [i8; 2];
let _9: f32;
let _10: isize;
let _11: Adt53;
let _12: Adt52;
let _13: u64;
let _14: f64;
let _15: [u16; 1];
let _16: f32;
let _17: Adt52;
let _18: u32;
let _19: f32;
let _20: Adt50;
let _21: i128;
let _22: Adt46;
let _23: ();
let _24: ();
{
RET = [1820180212383597577_i64,7168881493368299540_i64,6779243234594073149_i64,(-2137734559123742044_i64),1048595607359129455_i64,8982932747091318140_i64,4331395702526070402_i64];
_3 = [_1,_2,_2,_2,_2,_1,_1,_2];
_2 = _1 < _1;
_4 = RET;
_3 = [_1,_2,_1,_1,_1,_2,_2,_1];
_3 = [_1,_2,_2,_1,_2,_2,_2,_2];
RET = [4154651129468310670_i64,8333038875380427920_i64,(-7169254257786926799_i64),(-187411821597564579_i64),2688849986927583449_i64,2251359986676809438_i64,7903809365109676175_i64];
_1 = _2 | _2;
RET = _4;
_3 = [_2,_1,_2,_2,_2,_2,_1,_1];
RET = [8921185902382524112_i64,943029387815724942_i64,(-8318711926002851604_i64),(-5808028926549894288_i64),(-3702891166767928268_i64),(-4337227994324982571_i64),7787358119217386806_i64];
Call(RET = fn8(_4, _2, _2, _3, _1, _1, _3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-631590510492732184_i64),(-5814164521444224353_i64),7594185029687685057_i64,5634921011290604112_i64,4646612664294123846_i64,4652152077226002734_i64,(-1215644277894608802_i64)];
_4 = [(-5298452460410544694_i64),5764729426713034771_i64,289695817623806918_i64,8016908314566495223_i64,(-4459310890256514594_i64),(-8189526036936638840_i64),6272260280162500703_i64];
_4 = [(-3662338884610598625_i64),7761552803144986296_i64,(-7301997332134277210_i64),2789865085963412572_i64,5930543066235354871_i64,7497608829291069219_i64,(-8738209808440895095_i64)];
_5 = [145_u8,47_u8,172_u8,247_u8,107_u8];
_6 = [(-31_i8),(-40_i8)];
_4 = RET;
_7.2 = _2;
_4 = [2397435666098190653_i64,(-3509236066775929399_i64),8139389741709334706_i64,7070013559264490049_i64,(-1483923971094176261_i64),1120998015293090289_i64,(-3698251346997405556_i64)];
_7.1 = 1764100574_i32;
_7 = (3674617237725682395_u64, 2012084663_i32, _2);
_7.2 = _1 <= _1;
RET = _4;
_8 = _6;
_4 = [(-5877532540273469687_i64),(-6662350726034108242_i64),(-7917825597223325365_i64),6864387395053094655_i64,(-1597526213433685309_i64),(-6226881026745390862_i64),(-7015908797492595746_i64)];
match _7.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
3674617237725682395 => bb10,
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
_2 = !_1;
_8 = [125_i8,65_i8];
RET = [2057887574406868732_i64,2346549670360858056_i64,(-7344875280033326367_i64),(-2873562712602851348_i64),(-1214813063427474050_i64),(-3022894951751894722_i64),4368376890302067848_i64];
_7.0 = (-258088217756321875_i64) as u64;
RET = [1341441412496196144_i64,4689401748534431154_i64,(-8670846274146307648_i64),(-3220734459303712877_i64),719074943959901229_i64,(-2807086757060192665_i64),(-667080644388557973_i64)];
_7.2 = !_2;
_7.2 = _7.1 != _7.1;
_8 = _6;
_9 = 42364653938422170963704747942873494744_u128 as f32;
_9 = _7.1 as f32;
_10 = (-9223372036854775808_isize) >> _7.1;
_11.fld0 = _7.2 ^ _2;
_9 = _7.1 as f32;
_7.1 = (-236267508_i32) | 979145336_i32;
_6 = [(-126_i8),1_i8];
_4 = [3509185905322262096_i64,4138702036877668837_i64,(-439571410275120361_i64),1022465820015136716_i64,(-2141707942233042353_i64),2446043107249474808_i64,6972301127116804205_i64];
_1 = _7.2;
_7.1 = 2_usize as i32;
_7.2 = _11.fld0;
_3 = [_1,_11.fld0,_1,_2,_7.2,_1,_7.2,_2];
_15 = [29036_u16];
_14 = 45241847378881275651979416009612630938_i128 as f64;
Goto(bb11)
}
bb11 = {
_2 = _9 == _9;
_7.0 = !10489831740325387313_u64;
_16 = -_9;
_15 = [48840_u16];
_10 = (-8_isize) << _7.0;
_7.1 = '\u{e6460}' as i32;
RET = [(-4360165405484940706_i64),(-4626209948776245715_i64),(-8905654606814896013_i64),12660830634569633_i64,3810861956256638330_i64,(-8665560046402885230_i64),1593251866155185976_i64];
_2 = !_1;
_4 = [(-5593486449155884547_i64),2731959182163238884_i64,7927191136011749432_i64,4643293783338742794_i64,(-8156740837831900461_i64),(-651630699430543482_i64),2438290109814765044_i64];
_5 = [123_u8,210_u8,216_u8,205_u8,232_u8];
Goto(bb12)
}
bb12 = {
_13 = (-19390_i16) as u64;
_7.2 = !_1;
_8 = [(-30_i8),(-45_i8)];
Goto(bb13)
}
bb13 = {
_10 = 9223372036854775807_isize;
RET = _4;
_14 = 71_i8 as f64;
_7 = (_13, 1327749877_i32, _1);
_1 = _7.1 == _7.1;
Goto(bb14)
}
bb14 = {
_19 = _16 - _9;
_13 = _9 as u64;
_7 = (_13, (-928399214_i32), _11.fld0);
_1 = _7.0 <= _13;
_11.fld0 = _1;
_9 = _19;
_8 = [45_i8,94_i8];
RET = [(-989020438069021797_i64),(-8404019307549651935_i64),4188260977368184724_i64,3912458874152779889_i64,2723481733393020220_i64,1242111420372686276_i64,7693085654809728686_i64];
_7 = (_13, (-1558877666_i32), _2);
_14 = 43513_u16 as f64;
_18 = '\u{395cc}' as u32;
RET = _4;
_2 = !_7.2;
_16 = _9;
_3 = [_7.2,_2,_11.fld0,_2,_11.fld0,_7.2,_11.fld0,_1];
_7.0 = _13 * _13;
RET = _4;
_5 = [147_u8,167_u8,41_u8,120_u8,59_u8];
_7.2 = !_1;
_18 = 288012418_u32 + 1257327918_u32;
RET = [(-8810398076846345991_i64),5834627747535672947_i64,(-2760026735667046134_i64),5351468005246715662_i64,(-3997915219739009623_i64),8657696366338462618_i64,(-7860906437293944168_i64)];
_7.1 = (-20288268_i32) << _13;
_6 = [(-37_i8),126_i8];
_1 = _11.fld0 & _11.fld0;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(7_usize, 3_usize, Move(_3), 6_usize, Move(_6), 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i64; 7],mut _2: bool,mut _3: bool,mut _4: [bool; 8],mut _5: bool,mut _6: bool,mut _7: [bool; 8],mut _8: bool) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _9: [u16; 1];
let _10: i8;
let _11: Adt57;
let _12: f32;
let _13: usize;
let _14: (*mut *mut char,);
let _15: Adt47;
let _16: u8;
let _17: [i8; 2];
let _18: (u128, *mut char, *mut i64);
let _19: (u64, i32, bool);
let _20: char;
let _21: Adt47;
let _22: Adt53;
let _23: *const [i8; 2];
let _24: f64;
let _25: [u16; 1];
let _26: [u8; 5];
let _27: [bool; 8];
let _28: [i64; 7];
let _29: [i64; 7];
let _30: bool;
let _31: ();
let _32: ();
{
_8 = _3 >= _5;
RET = [1995713880359086221_i64,(-6382313447663900826_i64),(-903245658728131750_i64),(-2679992660458589524_i64),(-8767372256422348484_i64),(-2151128678105317086_i64),8031678031221524072_i64];
Call(_9 = fn9(_6, _1, _4, _2, _7, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.fld0.0 = 128882804605935042428306327877984280270_u128 * 136971203907816865773766041922730842595_u128;
_11.fld2.2 = _3 < _5;
_11.fld1.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_13 = 5_usize | 3549634672131687867_usize;
_11.fld2 = (12328898068753372287_u64, 2018932335_i32, _3);
_8 = !_11.fld2.2;
_11.fld2.2 = _11.fld2.1 == _11.fld2.1;
_9 = [23753_u16];
_11.fld1.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_11.fld4 = [(-40_i8),45_i8];
_11.fld2.0 = 3517171120670855442_u64 << _11.fld2.1;
_11.fld2.1 = 575433178_i32;
Goto(bb2)
}
bb2 = {
_11.fld2.1 = 55_i8 as i32;
_10 = (-80_i8) | (-86_i8);
_6 = _5 != _8;
_11.fld4 = [_10,_10];
Goto(bb3)
}
bb3 = {
_11.fld2.1 = '\u{67bf}' as i32;
_12 = (-9150_i16) as f32;
_13 = _5 as usize;
_14.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_11.fld1.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_14.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_5 = _3;
_11.fld2.1 = (-217593639_i32) + 1373512923_i32;
_1 = [(-4528486926242101091_i64),726453842438619177_i64,2089442460282227687_i64,(-3010605000784510777_i64),7811382333425827768_i64,4758377052730294589_i64,(-8914580518806442267_i64)];
_11.fld2 = (15401198340711249110_u64, (-798832007_i32), _3);
_16 = !80_u8;
match _11.fld2.0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
15401198340711249110 => bb11,
_ => bb10
}
}
bb4 = {
_11.fld2.1 = 55_i8 as i32;
_10 = (-80_i8) | (-86_i8);
_6 = _5 != _8;
_11.fld4 = [_10,_10];
Goto(bb3)
}
bb5 = {
_11.fld0.0 = 128882804605935042428306327877984280270_u128 * 136971203907816865773766041922730842595_u128;
_11.fld2.2 = _3 < _5;
_11.fld1.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_13 = 5_usize | 3549634672131687867_usize;
_11.fld2 = (12328898068753372287_u64, 2018932335_i32, _3);
_8 = !_11.fld2.2;
_11.fld2.2 = _11.fld2.1 == _11.fld2.1;
_9 = [23753_u16];
_11.fld1.0 = core::ptr::addr_of_mut!(_11.fld0.1);
_11.fld4 = [(-40_i8),45_i8];
_11.fld2.0 = 3517171120670855442_u64 << _11.fld2.1;
_11.fld2.1 = 575433178_i32;
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
_5 = !_2;
_11.fld2 = (15706154350029031629_u64, (-1825869895_i32), _2);
_16 = 161_u8;
_10 = _13 as i8;
_7 = [_8,_3,_2,_8,_5,_2,_2,_3];
_2 = _11.fld2.2 < _5;
_11.fld2.1 = 1389453870_u32 as i32;
_2 = _8 | _3;
_11.fld2.2 = !_3;
_11.fld2 = (14338665646557648551_u64, 1581799271_i32, _6);
_11.fld2.2 = _11.fld2.1 < _11.fld2.1;
_18.0 = _11.fld0.0 - _11.fld0.0;
Goto(bb12)
}
bb12 = {
_5 = _11.fld2.2 & _3;
match _11.fld2.1 {
1581799271 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_6 = _2 ^ _2;
_21 = Adt47::Variant1 { fld0: 1792049922_u32 };
_5 = !_8;
place!(Field::<u32>(Variant(_21, 1), 0)) = !909566434_u32;
_17 = [_10,_10];
_22.fld0 = !_2;
_18.0 = !_11.fld0.0;
_19.0 = _11.fld2.0;
_11.fld2.1 = (-1190745017_i32) * 293145444_i32;
SetDiscriminant(_21, 3);
_11.fld2.0 = _19.0 + _19.0;
_4 = [_22.fld0,_2,_5,_3,_8,_2,_8,_8];
_20 = '\u{109543}';
_11.fld2.2 = !_22.fld0;
_25 = [45678_u16];
_16 = 175_u8 + 187_u8;
_18.0 = !_11.fld0.0;
_28 = [8069401492413688412_i64,(-5887720346137760010_i64),2699944844512705872_i64,7781481051492073759_i64,(-9178743371386742311_i64),(-2731472761840273470_i64),3774086492718331865_i64];
_24 = (-24863_i16) as f64;
_11.fld0.1 = core::ptr::addr_of_mut!(_20);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(8_usize, 2_usize, Move(_2), 25_usize, Move(_25), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(8_usize, 16_usize, Move(_16), 4_usize, Move(_4), 20_usize, Move(_20), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: [i64; 7],mut _3: [bool; 8],mut _4: bool,mut _5: [bool; 8],mut _6: bool) -> [u16; 1] {
mir! {
type RET = [u16; 1];
let _7: Adt53;
let _8: u16;
let _9: bool;
let _10: *mut u16;
let _11: [bool; 8];
let _12: *mut u64;
let _13: bool;
let _14: [bool; 8];
let _15: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8);
let _16: Adt45;
let _17: u64;
let _18: isize;
let _19: f32;
let _20: Adt48;
let _21: [i8; 2];
let _22: *mut *mut char;
let _23: *mut u64;
let _24: f32;
let _25: [i64; 7];
let _26: Adt43;
let _27: [i8; 2];
let _28: bool;
let _29: [u16; 1];
let _30: i32;
let _31: Adt52;
let _32: bool;
let _33: Adt45;
let _34: [bool; 8];
let _35: f64;
let _36: i8;
let _37: [i64; 7];
let _38: ();
let _39: ();
{
_3 = _5;
RET = [20907_u16];
_6 = _4 | _1;
_4 = !_6;
_9 = !_6;
_8 = !41746_u16;
_3 = [_4,_4,_9,_1,_9,_9,_9,_1];
_11 = [_9,_6,_4,_1,_9,_1,_9,_6];
_7 = Adt53 { fld0: _9 };
RET = [_8];
_1 = !_9;
_6 = _1 < _1;
_5 = [_7.fld0,_1,_1,_9,_6,_6,_6,_7.fld0];
_1 = _7.fld0 ^ _7.fld0;
_9 = _7.fld0;
_3 = _5;
RET = [_8];
Goto(bb1)
}
bb1 = {
_10 = core::ptr::addr_of_mut!(_8);
_2 = [(-6018172619226521150_i64),(-4698350898313031496_i64),3264616914979778236_i64,(-5235940694268568787_i64),(-3465638926804028006_i64),4028095058581847662_i64,(-885187867872089203_i64)];
RET = [(*_10)];
_8 = !77_u16;
_10 = core::ptr::addr_of_mut!((*_10));
_6 = _1 | _1;
RET = [_8];
_4 = _7.fld0;
_1 = _6 | _7.fld0;
_5 = [_9,_4,_1,_9,_1,_1,_9,_7.fld0];
_11 = [_4,_6,_6,_6,_4,_4,_6,_1];
_15.3.0 = 217524068459840747939356921426319594680_u128 | 312612683372026052571899984656866201058_u128;
_15.0 = core::ptr::addr_of!(_15.3);
_15.1 = 2322505224_u32 as isize;
_5 = [_6,_9,_1,_6,_1,_9,_6,_1];
_7 = Adt53 { fld0: _1 };
RET = [(*_10)];
_6 = _7.fld0 != _9;
_12 = core::ptr::addr_of_mut!(_17);
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = 3076780889236684369_u64 + 7075425835690901657_u64;
_8 = 46066_u16;
_14 = [_9,_9,_9,_1,_4,_1,_1,_1];
RET = [_8];
_8 = 16270_u16;
_5 = _11;
Call(_15 = fn10(_1, _5, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_10) = _15.3.0 as u16;
_13 = _4;
_11 = [_6,_9,_9,_13,_13,_1,_1,_9];
_11 = [_9,_6,_1,_9,_7.fld0,_4,_9,_13];
_3 = [_1,_6,_1,_4,_7.fld0,_9,_13,_6];
Goto(bb3)
}
bb3 = {
_18 = _15.1;
_1 = _6 == _9;
_14 = [_4,_6,_1,_1,_6,_9,_1,_4];
_4 = _6 ^ _6;
_3 = _5;
_9 = _1 ^ _6;
_17 = 273237662579550566_u64;
_1 = !_7.fld0;
_10 = core::ptr::addr_of_mut!((*_10));
_8 = 7963_u16 + 28411_u16;
match (*_12) {
0 => bb1,
1 => bb2,
2 => bb4,
273237662579550566 => bb6,
_ => bb5
}
}
bb4 = {
(*_10) = _15.3.0 as u16;
_13 = _4;
_11 = [_6,_9,_9,_13,_13,_1,_1,_9];
_11 = [_9,_6,_1,_9,_7.fld0,_4,_9,_13];
_3 = [_1,_6,_1,_4,_7.fld0,_9,_13,_6];
Goto(bb3)
}
bb5 = {
_10 = core::ptr::addr_of_mut!(_8);
_2 = [(-6018172619226521150_i64),(-4698350898313031496_i64),3264616914979778236_i64,(-5235940694268568787_i64),(-3465638926804028006_i64),4028095058581847662_i64,(-885187867872089203_i64)];
RET = [(*_10)];
_8 = !77_u16;
_10 = core::ptr::addr_of_mut!((*_10));
_6 = _1 | _1;
RET = [_8];
_4 = _7.fld0;
_1 = _6 | _7.fld0;
_5 = [_9,_4,_1,_9,_1,_1,_9,_7.fld0];
_11 = [_4,_6,_6,_6,_4,_4,_6,_1];
_15.3.0 = 217524068459840747939356921426319594680_u128 | 312612683372026052571899984656866201058_u128;
_15.0 = core::ptr::addr_of!(_15.3);
_15.1 = 2322505224_u32 as isize;
_5 = [_6,_9,_1,_6,_1,_9,_6,_1];
_7 = Adt53 { fld0: _1 };
RET = [(*_10)];
_6 = _7.fld0 != _9;
_12 = core::ptr::addr_of_mut!(_17);
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = 3076780889236684369_u64 + 7075425835690901657_u64;
_8 = 46066_u16;
_14 = [_9,_9,_9,_1,_4,_1,_1,_1];
RET = [_8];
_8 = 16270_u16;
_5 = _11;
Call(_15 = fn10(_1, _5, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_15.0 = core::ptr::addr_of!(_15.3);
_17 = 4266565156266132743_u64 & 9632503334038543829_u64;
(*_12) = 18136459676709022418_u64 >> _15.3.0;
_4 = _9 >= _13;
_19 = 1_usize as f32;
_15.0 = core::ptr::addr_of!(_15.3);
_15.4 = (*_12) as i8;
_7.fld0 = !_13;
(*_12) = !5182775450328723533_u64;
_8 = 2905068049_u32 as u16;
_15.0 = core::ptr::addr_of!(_15.3);
_11 = _3;
_13 = _9;
_6 = _7.fld0;
_15.2 = [_15.4,_15.4];
_15.4 = -108_i8;
_4 = _13;
Call(_17 = core::intrinsics::transmute(_5), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = [_1,_4,_1,_1,_6,_9,_6,_6];
_12 = core::ptr::addr_of_mut!((*_12));
_15.2 = [_15.4,_15.4];
_7.fld0 = !_13;
_7 = Adt53 { fld0: _13 };
_13 = !_6;
_12 = core::ptr::addr_of_mut!((*_12));
_8 = 15485_u16;
_19 = _8 as f32;
_15.4 = _18 as i8;
_19 = _8 as f32;
match (*_10) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
15485 => bb10,
_ => bb9
}
}
bb8 = {
_18 = _15.1;
_1 = _6 == _9;
_14 = [_4,_6,_1,_1,_6,_9,_1,_4];
_4 = _6 ^ _6;
_3 = _5;
_9 = _1 ^ _6;
_17 = 273237662579550566_u64;
_1 = !_7.fld0;
_10 = core::ptr::addr_of_mut!((*_10));
_8 = 7963_u16 + 28411_u16;
match (*_12) {
0 => bb1,
1 => bb2,
2 => bb4,
273237662579550566 => bb6,
_ => bb5
}
}
bb9 = {
_10 = core::ptr::addr_of_mut!(_8);
_2 = [(-6018172619226521150_i64),(-4698350898313031496_i64),3264616914979778236_i64,(-5235940694268568787_i64),(-3465638926804028006_i64),4028095058581847662_i64,(-885187867872089203_i64)];
RET = [(*_10)];
_8 = !77_u16;
_10 = core::ptr::addr_of_mut!((*_10));
_6 = _1 | _1;
RET = [_8];
_4 = _7.fld0;
_1 = _6 | _7.fld0;
_5 = [_9,_4,_1,_9,_1,_1,_9,_7.fld0];
_11 = [_4,_6,_6,_6,_4,_4,_6,_1];
_15.3.0 = 217524068459840747939356921426319594680_u128 | 312612683372026052571899984656866201058_u128;
_15.0 = core::ptr::addr_of!(_15.3);
_15.1 = 2322505224_u32 as isize;
_5 = [_6,_9,_1,_6,_1,_9,_6,_1];
_7 = Adt53 { fld0: _1 };
RET = [(*_10)];
_6 = _7.fld0 != _9;
_12 = core::ptr::addr_of_mut!(_17);
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = 3076780889236684369_u64 + 7075425835690901657_u64;
_8 = 46066_u16;
_14 = [_9,_9,_9,_1,_4,_1,_1,_1];
RET = [_8];
_8 = 16270_u16;
_5 = _11;
Call(_15 = fn10(_1, _5, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_15.0 = core::ptr::addr_of!(_15.3);
(*_10) = 5511967170469217409_i64 as u16;
_5 = [_1,_13,_6,_4,_13,_1,_7.fld0,_6];
RET = [(*_10)];
_19 = _15.3.0 as f32;
_21 = [_15.4,_15.4];
_7.fld0 = _1;
_4 = _7.fld0 ^ _9;
_15.1 = _18 - _18;
_22 = core::ptr::addr_of_mut!(_15.3.1);
_13 = !_4;
_22 = core::ptr::addr_of_mut!(_15.3.1);
_15.0 = core::ptr::addr_of!(_15.3);
(*_10) = 48148_u16;
_15.2 = _21;
_23 = core::ptr::addr_of_mut!((*_12));
(*_10) = !10795_u16;
(*_10) = 107344247886620658218578209186648236138_i128 as u16;
Call((*_23) = fn11(_7, _9, _4, _15.3.0, _3, _7, _15, _15.0, _15.0, _6, _15.3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_13 = _19 < _19;
RET = [(*_10)];
_5 = _14;
_15.0 = core::ptr::addr_of!(_15.3);
_15.4 = 7_i8 * (-116_i8);
_15.0 = core::ptr::addr_of!(_15.3);
(*_23) = !8757046655354521631_u64;
RET = [(*_10)];
_15.2 = _21;
(*_23) = _15.1 as u64;
_19 = 37_u8 as f32;
_25 = [1418677305111412118_i64,(-549537946384994787_i64),2618618484761528657_i64,2787980134948697355_i64,6342104780549834040_i64,(-3957512092646428779_i64),2464744353125102149_i64];
_2 = [(-8552365560478030244_i64),(-8912802313250274822_i64),5915835572750475531_i64,519163264219515534_i64,7789984818234159911_i64,(-4314393133693281687_i64),(-2548929887311483588_i64)];
_11 = [_1,_6,_6,_6,_1,_4,_13,_7.fld0];
_15.0 = core::ptr::addr_of!(_15.3);
_22 = core::ptr::addr_of_mut!((*_22));
Goto(bb12)
}
bb12 = {
RET = [(*_10)];
(*_12) = 4117519898_u32 as u64;
_17 = !1730165862323187251_u64;
_18 = _17 as isize;
RET = [_8];
_24 = -_19;
_27 = _15.2;
_5 = [_4,_7.fld0,_9,_4,_1,_9,_13,_1];
_10 = core::ptr::addr_of_mut!((*_10));
(*_12) = 314858736817525170_u64;
_7.fld0 = _1 > _6;
_14 = [_4,_13,_9,_7.fld0,_6,_9,_6,_4];
(*_10) = 4285_u16;
_28 = _9 != _7.fld0;
_19 = _24 + _24;
_3 = _14;
_8 = !28013_u16;
(*_12) = _24 as u64;
_21 = [_15.4,_15.4];
_10 = core::ptr::addr_of_mut!((*_10));
_27 = _15.2;
_15.0 = core::ptr::addr_of!(_15.3);
_28 = _9;
Goto(bb13)
}
bb13 = {
(*_12) = _15.3.0 as u64;
_15.2 = [_15.4,_15.4];
_29 = RET;
(*_10) = 29266_u16;
_15.3.0 = !78928239660268031280021605485022790752_u128;
_2 = [3451608016760572211_i64,(-2005848875738913292_i64),387407688173825270_i64,(-5398095726435970634_i64),5394143636155263267_i64,8417935473106168952_i64,2412279303248591768_i64];
_1 = _9 >= _13;
_7 = Adt53 { fld0: _6 };
_29 = [(*_10)];
_4 = _6;
_3 = _11;
_31 = Adt52::Variant1 { fld0: _22 };
(*_23) = !13430313437102287394_u64;
_12 = _23;
_2 = [(-3207041206250877006_i64),(-8089020430255060083_i64),4921398131112634845_i64,(-4852270541246635779_i64),(-6304942584446068578_i64),3146705832719016267_i64,5389276350474045485_i64];
_4 = _28;
_15.2 = [_15.4,_15.4];
match (*_10) {
0 => bb4,
1 => bb2,
2 => bb11,
3 => bb14,
4 => bb15,
5 => bb16,
29266 => bb18,
_ => bb17
}
}
bb14 = {
_11 = [_1,_4,_1,_1,_6,_9,_6,_6];
_12 = core::ptr::addr_of_mut!((*_12));
_15.2 = [_15.4,_15.4];
_7.fld0 = !_13;
_7 = Adt53 { fld0: _13 };
_13 = !_6;
_12 = core::ptr::addr_of_mut!((*_12));
_8 = 15485_u16;
_19 = _8 as f32;
_15.4 = _18 as i8;
_19 = _8 as f32;
match (*_10) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
15485 => bb10,
_ => bb9
}
}
bb15 = {
_10 = core::ptr::addr_of_mut!(_8);
_2 = [(-6018172619226521150_i64),(-4698350898313031496_i64),3264616914979778236_i64,(-5235940694268568787_i64),(-3465638926804028006_i64),4028095058581847662_i64,(-885187867872089203_i64)];
RET = [(*_10)];
_8 = !77_u16;
_10 = core::ptr::addr_of_mut!((*_10));
_6 = _1 | _1;
RET = [_8];
_4 = _7.fld0;
_1 = _6 | _7.fld0;
_5 = [_9,_4,_1,_9,_1,_1,_9,_7.fld0];
_11 = [_4,_6,_6,_6,_4,_4,_6,_1];
_15.3.0 = 217524068459840747939356921426319594680_u128 | 312612683372026052571899984656866201058_u128;
_15.0 = core::ptr::addr_of!(_15.3);
_15.1 = 2322505224_u32 as isize;
_5 = [_6,_9,_1,_6,_1,_9,_6,_1];
_7 = Adt53 { fld0: _1 };
RET = [(*_10)];
_6 = _7.fld0 != _9;
_12 = core::ptr::addr_of_mut!(_17);
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = 3076780889236684369_u64 + 7075425835690901657_u64;
_8 = 46066_u16;
_14 = [_9,_9,_9,_1,_4,_1,_1,_1];
RET = [_8];
_8 = 16270_u16;
_5 = _11;
Call(_15 = fn10(_1, _5, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_15.0 = core::ptr::addr_of!(_15.3);
(*_10) = 5511967170469217409_i64 as u16;
_5 = [_1,_13,_6,_4,_13,_1,_7.fld0,_6];
RET = [(*_10)];
_19 = _15.3.0 as f32;
_21 = [_15.4,_15.4];
_7.fld0 = _1;
_4 = _7.fld0 ^ _9;
_15.1 = _18 - _18;
_22 = core::ptr::addr_of_mut!(_15.3.1);
_13 = !_4;
_22 = core::ptr::addr_of_mut!(_15.3.1);
_15.0 = core::ptr::addr_of!(_15.3);
(*_10) = 48148_u16;
_15.2 = _21;
_23 = core::ptr::addr_of_mut!((*_12));
(*_10) = !10795_u16;
(*_10) = 107344247886620658218578209186648236138_i128 as u16;
Call((*_23) = fn11(_7, _9, _4, _15.3.0, _3, _7, _15, _15.0, _15.0, _6, _15.3), ReturnTo(bb11), UnwindUnreachable())
}
bb17 = {
_10 = core::ptr::addr_of_mut!(_8);
_2 = [(-6018172619226521150_i64),(-4698350898313031496_i64),3264616914979778236_i64,(-5235940694268568787_i64),(-3465638926804028006_i64),4028095058581847662_i64,(-885187867872089203_i64)];
RET = [(*_10)];
_8 = !77_u16;
_10 = core::ptr::addr_of_mut!((*_10));
_6 = _1 | _1;
RET = [_8];
_4 = _7.fld0;
_1 = _6 | _7.fld0;
_5 = [_9,_4,_1,_9,_1,_1,_9,_7.fld0];
_11 = [_4,_6,_6,_6,_4,_4,_6,_1];
_15.3.0 = 217524068459840747939356921426319594680_u128 | 312612683372026052571899984656866201058_u128;
_15.0 = core::ptr::addr_of!(_15.3);
_15.1 = 2322505224_u32 as isize;
_5 = [_6,_9,_1,_6,_1,_9,_6,_1];
_7 = Adt53 { fld0: _1 };
RET = [(*_10)];
_6 = _7.fld0 != _9;
_12 = core::ptr::addr_of_mut!(_17);
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = 3076780889236684369_u64 + 7075425835690901657_u64;
_8 = 46066_u16;
_14 = [_9,_9,_9,_1,_4,_1,_1,_1];
RET = [_8];
_8 = 16270_u16;
_5 = _11;
Call(_15 = fn10(_1, _5, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_4 = _1 >= _1;
_7 = Adt53 { fld0: _1 };
_22 = Field::<*mut *mut char>(Variant(_31, 1), 0);
(*_12) = !9389484102143798791_u64;
_3 = _14;
_35 = 3_usize as f64;
_36 = _15.4;
Goto(bb19)
}
bb19 = {
Call(_38 = dump_var(9_usize, 17_usize, Move(_17), 29_usize, Move(_29), 2_usize, Move(_2), 36_usize, Move(_36)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(9_usize, 21_usize, Move(_21), 6_usize, Move(_6), 27_usize, Move(_27), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_38 = dump_var(9_usize, 5_usize, Move(_5), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: bool,mut _2: [bool; 8],mut _3: Adt53,mut _4: bool) -> (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8) {
mir! {
type RET = (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8);
let _5: char;
let _6: i128;
let _7: f64;
let _8: Adt51;
let _9: (*const (u128, *mut char, *mut i64), [i8; 2], *const [i8; 2], *const (u128, *mut char, *mut i64), bool);
let _10: f32;
let _11: *mut [i64; 7];
let _12: Adt47;
let _13: char;
let _14: [u16; 1];
let _15: [u8; 5];
let _16: (u64, i32, bool);
let _17: [i8; 2];
let _18: *mut u64;
let _19: i16;
let _20: [u8; 5];
let _21: f64;
let _22: f64;
let _23: f32;
let _24: isize;
let _25: char;
let _26: (u64, i32, bool);
let _27: [u8; 5];
let _28: char;
let _29: u32;
let _30: u32;
let _31: (*mut *mut char,);
let _32: [i64; 7];
let _33: [bool; 8];
let _34: [bool; 8];
let _35: (u64, i32, bool);
let _36: Adt48;
let _37: (u64, i32, bool);
let _38: [u16; 1];
let _39: [i64; 7];
let _40: i64;
let _41: bool;
let _42: Adt53;
let _43: isize;
let _44: (u64, i32, bool);
let _45: u16;
let _46: Adt52;
let _47: [i64; 7];
let _48: i32;
let _49: (u128, *mut char, *mut i64);
let _50: f32;
let _51: [bool; 8];
let _52: ();
let _53: ();
{
RET.2 = [19_i8,(-26_i8)];
RET.1 = 9223372036854775807_isize;
RET.4 = RET.1 as i8;
RET.3.0 = !155872343430411746513464085048317237518_u128;
RET.0 = core::ptr::addr_of!(RET.3);
RET.2 = [RET.4,RET.4];
RET.3.1 = core::ptr::addr_of_mut!(_5);
RET.3.1 = core::ptr::addr_of_mut!(_5);
_5 = '\u{f5bc2}';
RET.1 = -(-9223372036854775808_isize);
RET.0 = core::ptr::addr_of!(RET.3);
RET.4 = 76_i8;
match RET.4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
76 => bb9,
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
RET.4 = -36_i8;
RET.3.1 = core::ptr::addr_of_mut!(_5);
_6 = 7542696876596391754_u64 as i128;
_9.3 = RET.0;
RET.1 = (-9223372036854775808_isize) << RET.3.0;
_7 = (-15895_i16) as f64;
RET.3.1 = core::ptr::addr_of_mut!(_5);
RET.3.1 = core::ptr::addr_of_mut!(_5);
_4 = _3.fld0;
_9.1 = [RET.4,RET.4];
RET.0 = _9.3;
_3 = Adt53 { fld0: _4 };
_9.0 = _9.3;
_9.2 = core::ptr::addr_of!(RET.2);
RET.1 = (-10175_i16) as isize;
_9.4 = _3.fld0 != _3.fld0;
_1 = _3.fld0 | _4;
RET.0 = core::ptr::addr_of!(RET.3);
Goto(bb10)
}
bb10 = {
RET.1 = 18008_i16 as isize;
_3 = Adt53 { fld0: _9.4 };
RET.3.1 = core::ptr::addr_of_mut!(_13);
RET.1 = 9223372036854775807_isize << RET.4;
_13 = _5;
_4 = _1;
RET.0 = _9.0;
_9.1 = [RET.4,RET.4];
_14 = [7052_u16];
_10 = 11129_i16 as f32;
RET.4 = -(-32_i8);
_12 = Adt47::Variant0 { fld0: RET.1,fld1: 16544840581590895667_u64 };
_9.4 = _3.fld0 != _1;
RET.0 = core::ptr::addr_of!(RET.3);
_9.3 = _9.0;
RET.3.1 = core::ptr::addr_of_mut!(_13);
_16 = (6405433740600790589_u64, 193429841_i32, _1);
_16 = (16018630935427174149_u64, 886330156_i32, _9.4);
RET.0 = core::ptr::addr_of!(RET.3);
RET.3.0 = !273307089116836274705507439703721947534_u128;
_20 = [59_u8,35_u8,39_u8,161_u8,172_u8];
_2 = [_9.4,_9.4,_3.fld0,_16.2,_1,_3.fld0,_4,_9.4];
Goto(bb11)
}
bb11 = {
_18 = core::ptr::addr_of_mut!(_16.0);
_6 = (*_18) as i128;
RET.0 = _9.0;
_9.4 = _4;
RET.1 = Field::<isize>(Variant(_12, 0), 0);
_13 = _5;
_9.3 = _9.0;
_19 = 248_u8 as i16;
_12 = Adt47::Variant3 { fld0: (*_18) };
_15 = [162_u8,148_u8,63_u8,37_u8,27_u8];
_7 = RET.1 as f64;
_7 = _10 as f64;
_17 = [RET.4,RET.4];
RET.4 = 39_i8 & 61_i8;
_9.0 = core::ptr::addr_of!(RET.3);
_9.1 = _17;
_22 = -_7;
RET.0 = core::ptr::addr_of!(RET.3);
_13 = _5;
_9.0 = core::ptr::addr_of!(RET.3);
Goto(bb12)
}
bb12 = {
(*_18) = !Field::<u64>(Variant(_12, 3), 0);
RET.4 = 71_i8;
RET.0 = _9.3;
_16.0 = Field::<u64>(Variant(_12, 3), 0) * Field::<u64>(Variant(_12, 3), 0);
_18 = core::ptr::addr_of_mut!(_16.0);
_3.fld0 = !_1;
_1 = !_4;
RET.2 = [RET.4,RET.4];
_5 = _13;
_9.1 = _17;
_23 = 4190643467_u32 as f32;
_9.3 = RET.0;
_9.0 = RET.0;
_20 = [234_u8,102_u8,27_u8,170_u8,16_u8];
_1 = !_16.2;
_4 = _1;
Goto(bb13)
}
bb13 = {
_2 = [_16.2,_1,_9.4,_3.fld0,_4,_16.2,_1,_1];
_9.3 = core::ptr::addr_of!(RET.3);
_3.fld0 = _6 < _6;
_14 = [3971_u16];
_15 = [35_u8,210_u8,173_u8,49_u8,27_u8];
_16 = (Field::<u64>(Variant(_12, 3), 0), (-1531793108_i32), _3.fld0);
_5 = _13;
_9.4 = _16.2;
_5 = _13;
_13 = _5;
RET.0 = _9.3;
_24 = RET.1;
_28 = _5;
RET.4 = (-87_i8) ^ 71_i8;
RET.3.0 = !80508929950368587586396090400038661632_u128;
_21 = _22;
_26.2 = _9.4 | _9.4;
RET.4 = (-18_i8);
RET.4 = 111_i8;
_28 = _5;
SetDiscriminant(_12, 0);
_26 = (_16.0, _16.1, _9.4);
_16 = _26;
RET.0 = _9.0;
_9.0 = _9.3;
_22 = -_21;
_16.2 = !_9.4;
RET.3.1 = core::ptr::addr_of_mut!(_28);
(*_18) = _26.0;
match (*_18) {
0 => bb4,
1 => bb2,
2 => bb12,
3 => bb14,
4 => bb15,
16018630935427174149 => bb17,
_ => bb16
}
}
bb14 = {
(*_18) = !Field::<u64>(Variant(_12, 3), 0);
RET.4 = 71_i8;
RET.0 = _9.3;
_16.0 = Field::<u64>(Variant(_12, 3), 0) * Field::<u64>(Variant(_12, 3), 0);
_18 = core::ptr::addr_of_mut!(_16.0);
_3.fld0 = !_1;
_1 = !_4;
RET.2 = [RET.4,RET.4];
_5 = _13;
_9.1 = _17;
_23 = 4190643467_u32 as f32;
_9.3 = RET.0;
_9.0 = RET.0;
_20 = [234_u8,102_u8,27_u8,170_u8,16_u8];
_1 = !_16.2;
_4 = _1;
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_18 = core::ptr::addr_of_mut!((*_18));
place!(Field::<isize>(Variant(_12, 0), 0)) = RET.1;
RET.1 = _26.1 as isize;
match _26.0 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
16018630935427174149 => bb25,
_ => bb24
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
RET.4 = -36_i8;
RET.3.1 = core::ptr::addr_of_mut!(_5);
_6 = 7542696876596391754_u64 as i128;
_9.3 = RET.0;
RET.1 = (-9223372036854775808_isize) << RET.3.0;
_7 = (-15895_i16) as f64;
RET.3.1 = core::ptr::addr_of_mut!(_5);
RET.3.1 = core::ptr::addr_of_mut!(_5);
_4 = _3.fld0;
_9.1 = [RET.4,RET.4];
RET.0 = _9.3;
_3 = Adt53 { fld0: _4 };
_9.0 = _9.3;
_9.2 = core::ptr::addr_of!(RET.2);
RET.1 = (-10175_i16) as isize;
_9.4 = _3.fld0 != _3.fld0;
_1 = _3.fld0 | _4;
RET.0 = core::ptr::addr_of!(RET.3);
Goto(bb10)
}
bb21 = {
_2 = [_16.2,_1,_9.4,_3.fld0,_4,_16.2,_1,_1];
_9.3 = core::ptr::addr_of!(RET.3);
_3.fld0 = _6 < _6;
_14 = [3971_u16];
_15 = [35_u8,210_u8,173_u8,49_u8,27_u8];
_16 = (Field::<u64>(Variant(_12, 3), 0), (-1531793108_i32), _3.fld0);
_5 = _13;
_9.4 = _16.2;
_5 = _13;
_13 = _5;
RET.0 = _9.3;
_24 = RET.1;
_28 = _5;
RET.4 = (-87_i8) ^ 71_i8;
RET.3.0 = !80508929950368587586396090400038661632_u128;
_21 = _22;
_26.2 = _9.4 | _9.4;
RET.4 = (-18_i8);
RET.4 = 111_i8;
_28 = _5;
SetDiscriminant(_12, 0);
_26 = (_16.0, _16.1, _9.4);
_16 = _26;
RET.0 = _9.0;
_9.0 = _9.3;
_22 = -_21;
_16.2 = !_9.4;
RET.3.1 = core::ptr::addr_of_mut!(_28);
(*_18) = _26.0;
match (*_18) {
0 => bb4,
1 => bb2,
2 => bb12,
3 => bb14,
4 => bb15,
16018630935427174149 => bb17,
_ => bb16
}
}
bb22 = {
Return()
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_25 = _13;
RET.3.1 = core::ptr::addr_of_mut!(_25);
RET.4 = 1445769032_u32 as i8;
_26.1 = _16.1 & _16.1;
_9.3 = RET.0;
_1 = _3.fld0 != _4;
_23 = _10;
match _16.1 {
0 => bb24,
1 => bb9,
2 => bb16,
3 => bb20,
4 => bb12,
5 => bb26,
6 => bb27,
340282366920938463463374607430236418348 => bb29,
_ => bb28
}
}
bb26 = {
Return()
}
bb27 = {
_2 = [_16.2,_1,_9.4,_3.fld0,_4,_16.2,_1,_1];
_9.3 = core::ptr::addr_of!(RET.3);
_3.fld0 = _6 < _6;
_14 = [3971_u16];
_15 = [35_u8,210_u8,173_u8,49_u8,27_u8];
_16 = (Field::<u64>(Variant(_12, 3), 0), (-1531793108_i32), _3.fld0);
_5 = _13;
_9.4 = _16.2;
_5 = _13;
_13 = _5;
RET.0 = _9.3;
_24 = RET.1;
_28 = _5;
RET.4 = (-87_i8) ^ 71_i8;
RET.3.0 = !80508929950368587586396090400038661632_u128;
_21 = _22;
_26.2 = _9.4 | _9.4;
RET.4 = (-18_i8);
RET.4 = 111_i8;
_28 = _5;
SetDiscriminant(_12, 0);
_26 = (_16.0, _16.1, _9.4);
_16 = _26;
RET.0 = _9.0;
_9.0 = _9.3;
_22 = -_21;
_16.2 = !_9.4;
RET.3.1 = core::ptr::addr_of_mut!(_28);
(*_18) = _26.0;
match (*_18) {
0 => bb4,
1 => bb2,
2 => bb12,
3 => bb14,
4 => bb15,
16018630935427174149 => bb17,
_ => bb16
}
}
bb28 = {
Return()
}
bb29 = {
_20 = [234_u8,86_u8,86_u8,135_u8,95_u8];
place!(Field::<isize>(Variant(_12, 0), 0)) = RET.1;
RET.4 = (-5_i8);
_27 = _20;
_3 = Adt53 { fld0: _1 };
_9.3 = core::ptr::addr_of!(RET.3);
_14 = [59377_u16];
_28 = _5;
_9.0 = core::ptr::addr_of!(RET.3);
_14 = [7389_u16];
_30 = !3155039069_u32;
_6 = (-138499402665079407107429515784153764354_i128) * 120082641366984670585532592742214439494_i128;
RET.3.0 = !93718589640464503532433525520746968100_u128;
RET.0 = core::ptr::addr_of!(RET.3);
_1 = (*_18) == (*_18);
_3.fld0 = _4;
_16.0 = _26.0 + _26.0;
RET.3.1 = core::ptr::addr_of_mut!(_25);
_26.2 = _4;
_29 = _30;
RET.3.0 = !320483537008501504225284097225673390614_u128;
_14 = [1975_u16];
place!(Field::<isize>(Variant(_12, 0), 0)) = RET.1 ^ RET.1;
place!(Field::<u64>(Variant(_12, 0), 1)) = (*_18) % _26.0;
place!(Field::<u64>(Variant(_12, 0), 1)) = _26.0 | (*_18);
RET.2 = _9.1;
match _26.0 {
0 => bb30,
1 => bb31,
2 => bb32,
3 => bb33,
4 => bb34,
16018630935427174149 => bb36,
_ => bb35
}
}
bb30 = {
Return()
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
_18 = core::ptr::addr_of_mut!((*_18));
place!(Field::<isize>(Variant(_12, 0), 0)) = RET.1;
RET.1 = _26.1 as isize;
match _26.0 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
16018630935427174149 => bb25,
_ => bb24
}
}
bb34 = {
Return()
}
bb35 = {
Return()
}
bb36 = {
RET.1 = Field::<isize>(Variant(_12, 0), 0);
_27 = _15;
_31.0 = core::ptr::addr_of_mut!(RET.3.1);
_2 = [_9.4,_16.2,_4,_4,_9.4,_16.2,_4,_1];
_16.2 = _9.4;
RET.0 = _9.0;
_22 = -_21;
RET.1 = Field::<isize>(Variant(_12, 0), 0) * Field::<isize>(Variant(_12, 0), 0);
RET.4 = _19 as i8;
_20 = [95_u8,220_u8,154_u8,139_u8,86_u8];
_4 = !_3.fld0;
_33 = [_3.fld0,_4,_16.2,_16.2,_3.fld0,_9.4,_16.2,_16.2];
SetDiscriminant(_12, 1);
_9.4 = _3.fld0 | _4;
_11 = core::ptr::addr_of_mut!(_32);
_17 = [RET.4,RET.4];
_26.1 = _16.1 >> (*_18);
RET.3.0 = 275983761706913299460714916154701123769_u128;
_17 = _9.1;
_3 = Adt53 { fld0: _26.2 };
_4 = !_26.2;
Goto(bb37)
}
bb37 = {
Goto(bb38)
}
bb38 = {
RET.1 = _24 - _24;
_10 = _23;
_32 = [109600265442811461_i64,(-5471930083184375744_i64),3194287293154210744_i64,4386711069827062886_i64,6936324580163141444_i64,(-4847301359272051809_i64),8443524216209108522_i64];
RET.3.0 = !96126367768248056194636544270971512573_u128;
_20 = [14_u8,12_u8,147_u8,136_u8,206_u8];
_10 = 63406_u16 as f32;
RET.2 = _9.1;
(*_18) = _26.0 & _26.0;
_33 = [_9.4,_3.fld0,_4,_4,_16.2,_1,_4,_9.4];
_10 = _23;
_3.fld0 = _16.2;
_34 = _2;
_35.2 = !_4;
_37.0 = !(*_18);
_2 = _33;
RET.3.0 = 294664769664106119971641029741041125334_u128 >> _26.1;
_16.0 = _26.1 as u64;
_26 = ((*_18), _16.1, _3.fld0);
_40 = (-6663947515399699790_i64);
_24 = -RET.1;
_26.1 = _16.1;
_40 = 5419294837493074122_usize as i64;
match _16.1 {
0 => bb19,
1 => bb35,
2 => bb39,
3 => bb40,
4 => bb41,
5 => bb42,
340282366920938463463374607430236418348 => bb44,
_ => bb43
}
}
bb39 = {
Return()
}
bb40 = {
Return()
}
bb41 = {
Return()
}
bb42 = {
Return()
}
bb43 = {
Return()
}
bb44 = {
_9.0 = core::ptr::addr_of!(RET.3);
(*_11) = [_40,_40,_40,_40,_40,_40,_40];
_12 = Adt47::Variant2 { fld0: RET.3.0,fld1: 30900_u16 };
Goto(bb45)
}
bb45 = {
RET.3.0 = Field::<u128>(Variant(_12, 2), 0);
RET.4 = _19 as i8;
_39 = [_40,_40,_40,_40,_40,_40,_40];
(*_18) = _37.0;
_24 = RET.1;
_5 = _25;
_7 = _22;
RET.1 = _24;
_16.1 = !_26.1;
_6 = (*_18) as i128;
RET.3.0 = Field::<u128>(Variant(_12, 2), 0) * Field::<u128>(Variant(_12, 2), 0);
_35.1 = 14142852001173714073_usize as i32;
RET.3.2 = core::ptr::addr_of_mut!(_40);
RET.3.2 = core::ptr::addr_of_mut!(_40);
_19 = _40 as i16;
RET.4 = _23 as i8;
_45 = !53080_u16;
_34 = [_4,_9.4,_9.4,_9.4,_1,_35.2,_1,_3.fld0];
_25 = _5;
_26.0 = (*_18);
_23 = _10 * _10;
_44.0 = !(*_18);
Goto(bb46)
}
bb46 = {
Call(_52 = dump_var(10_usize, 33_usize, Move(_33), 39_usize, Move(_39), 34_usize, Move(_34), 45_usize, Move(_45)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_52 = dump_var(10_usize, 29_usize, Move(_29), 19_usize, Move(_19), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_52 = dump_var(10_usize, 14_usize, Move(_14), 40_usize, Move(_40), 26_usize, Move(_26), 24_usize, Move(_24)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: Adt53,mut _2: bool,mut _3: bool,mut _4: u128,mut _5: [bool; 8],mut _6: Adt53,mut _7: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),mut _8: *const (u128, *mut char, *mut i64),mut _9: *const (u128, *mut char, *mut i64),mut _10: bool,mut _11: (u128, *mut char, *mut i64)) -> u64 {
mir! {
type RET = u64;
let _12: char;
let _13: (u64, i32, bool);
let _14: Adt55;
let _15: isize;
let _16: [u16; 1];
let _17: f64;
let _18: [u16; 1];
let _19: [i8; 2];
let _20: (u64, i32, bool);
let _21: Adt46;
let _22: *mut *mut u128;
let _23: [i8; 2];
let _24: i8;
let _25: Adt52;
let _26: u8;
let _27: i32;
let _28: isize;
let _29: bool;
let _30: f64;
let _31: Adt47;
let _32: (u64, i32, bool);
let _33: bool;
let _34: f32;
let _35: ();
let _36: ();
{
_2 = _11.0 > (*_8).0;
_13.1 = -1812768786_i32;
(*_9).2 = _11.2;
_7.3.2 = _11.2;
_10 = _3;
_7.2 = [_7.4,_7.4];
_13.1 = (-1081295743_i32);
Goto(bb1)
}
bb1 = {
_7.3 = (*_8);
(*_9) = (_11.0, _7.3.1, _11.2);
_9 = _7.0;
_7.2 = [_7.4,_7.4];
(*_8).0 = _4;
(*_9).2 = _11.2;
_15 = _7.1 - _7.1;
Call(_10 = fn12(_1, _6, (*_8), (*_9).0, _9, _7, (*_8), _7, (*_8), _11.0, _9, _7.3, _5, (*_9), (*_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 5426571040242022130_u64;
_1.fld0 = _10 ^ _3;
_8 = _9;
(*_9).1 = core::ptr::addr_of_mut!(_12);
_13.0 = RET;
Goto(bb3)
}
bb3 = {
(*_9).2 = _11.2;
_16 = [52674_u16];
_18 = [38136_u16];
_12 = '\u{bfc4f}';
_7.3.0 = !_4;
_12 = '\u{4474}';
_20 = (RET, _13.1, _3);
_3 = !_1.fld0;
(*_9).0 = _4;
_1 = _6;
_7.3.2 = (*_9).2;
_8 = _7.0;
(*_9).1 = _7.3.1;
_9 = _7.0;
_7.2 = [_7.4,_7.4];
_20.0 = _7.4 as u64;
(*_8) = _7.3;
RET = _20.0;
_17 = (*_9).0 as f64;
_3 = !_20.2;
match _20.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607430686915713 => bb9,
_ => bb8
}
}
bb4 = {
RET = 5426571040242022130_u64;
_1.fld0 = _10 ^ _3;
_8 = _9;
(*_9).1 = core::ptr::addr_of_mut!(_12);
_13.0 = RET;
Goto(bb3)
}
bb5 = {
_7.3 = (*_8);
(*_9) = (_11.0, _7.3.1, _11.2);
_9 = _7.0;
_7.2 = [_7.4,_7.4];
(*_8).0 = _4;
(*_9).2 = _11.2;
_15 = _7.1 - _7.1;
Call(_10 = fn12(_1, _6, (*_8), (*_9).0, _9, _7, (*_8), _7, (*_8), _11.0, _9, _7.3, _5, (*_9), (*_8)), ReturnTo(bb2), UnwindUnreachable())
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
_20 = (RET, _13.1, _6.fld0);
_27 = _20.1 & _20.1;
(*_8).0 = _4;
_13.0 = _20.0 ^ RET;
_6 = Adt53 { fld0: _10 };
RET = !_20.0;
_28 = _15;
(*_8).1 = _11.1;
_6 = Adt53 { fld0: _3 };
_15 = _28 | _28;
(*_8) = (_11.0, _11.1, _7.3.2);
(*_8).2 = _7.3.2;
_1.fld0 = !_3;
_32.2 = !_1.fld0;
_13.0 = !RET;
_32.1 = _27 + _20.1;
_7.2 = [_7.4,_7.4];
_29 = !_1.fld0;
_13 = _20;
(*_8).1 = core::ptr::addr_of_mut!(_12);
_14 = Adt55::Variant0 { fld0: _32.2,fld1: 57_u8,fld2: (*_9).2,fld3: _20,fld4: _1 };
_27 = _20.1 * _32.1;
_18 = [14178_u16];
_24 = _7.4 << (*_8).0;
(*_8).2 = _11.2;
_32 = Field::<(u64, i32, bool)>(Variant(_14, 0), 3);
match _13.1 {
0 => bb6,
1 => bb10,
2 => bb11,
340282366920938463463374607430686915713 => bb13,
_ => bb12
}
}
bb10 = {
RET = 5426571040242022130_u64;
_1.fld0 = _10 ^ _3;
_8 = _9;
(*_9).1 = core::ptr::addr_of_mut!(_12);
_13.0 = RET;
Goto(bb3)
}
bb11 = {
Return()
}
bb12 = {
RET = 5426571040242022130_u64;
_1.fld0 = _10 ^ _3;
_8 = _9;
(*_9).1 = core::ptr::addr_of_mut!(_12);
_13.0 = RET;
Goto(bb3)
}
bb13 = {
_23 = _7.2;
_7 = (_8, _15, _23, _11, _24);
_7.3.0 = !(*_9).0;
(*_8).0 = _11.0;
Call((*_8).2 = fn13(_11.0, _11, _7, Field::<Adt53>(Variant(_14, 0), 4).fld0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11.2 = (*_8).2;
(*_8).2 = _11.2;
_14 = Adt55::Variant0 { fld0: _29,fld1: 193_u8,fld2: (*_9).2,fld3: _20,fld4: _6 };
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(11_usize, 3_usize, Move(_3), 28_usize, Move(_28), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(11_usize, 32_usize, Move(_32), 20_usize, Move(_20), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: Adt53,mut _2: Adt53,mut _3: (u128, *mut char, *mut i64),mut _4: u128,mut _5: *const (u128, *mut char, *mut i64),mut _6: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),mut _7: (u128, *mut char, *mut i64),mut _8: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),mut _9: (u128, *mut char, *mut i64),mut _10: u128,mut _11: *const (u128, *mut char, *mut i64),mut _12: (u128, *mut char, *mut i64),mut _13: [bool; 8],mut _14: (u128, *mut char, *mut i64),mut _15: (u128, *mut char, *mut i64)) -> bool {
mir! {
type RET = bool;
let _16: ();
let _17: ();
{
_9.1 = _3.1;
RET = !_1.fld0;
_7.2 = _6.3.2;
(*_5).1 = _7.1;
_8.1 = -_6.1;
(*_5).2 = _12.2;
(*_11).0 = _4;
(*_11).1 = _7.1;
_5 = core::ptr::addr_of!(_3);
_1 = _2;
_9.2 = (*_5).2;
_8.1 = (-1322000993_i32) as isize;
_6.1 = 103_u8 as isize;
_8.3 = (_14.0, (*_5).1, _6.3.2);
_8.0 = _11;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(12_usize, 4_usize, Move(_4), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: u128,mut _2: (u128, *mut char, *mut i64),mut _3: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),mut _4: bool) -> *mut i64 {
mir! {
type RET = *mut i64;
let _5: Adt49;
let _6: char;
let _7: f64;
let _8: isize;
let _9: bool;
let _10: i128;
let _11: f64;
let _12: [bool; 8];
let _13: usize;
let _14: *mut *mut u128;
let _15: bool;
let _16: bool;
let _17: Adt58;
let _18: bool;
let _19: (u64, i32, bool);
let _20: [u8; 5];
let _21: *const (u128, *mut char, *mut i64);
let _22: isize;
let _23: [u8; 5];
let _24: [i64; 7];
let _25: isize;
let _26: i8;
let _27: isize;
let _28: Adt47;
let _29: i16;
let _30: isize;
let _31: isize;
let _32: [u8; 5];
let _33: Adt48;
let _34: [bool; 8];
let _35: Adt44;
let _36: [u16; 1];
let _37: bool;
let _38: [u16; 1];
let _39: u64;
let _40: ();
let _41: ();
{
RET = _2.2;
_3.3.2 = RET;
_2.2 = RET;
_3.2 = [_3.4,_3.4];
_3.4 = !(-100_i8);
_1 = 15963423652535265527_u64 as u128;
_3.0 = core::ptr::addr_of!(_2);
_3.0 = core::ptr::addr_of!(_3.3);
RET = _2.2;
_3.0 = core::ptr::addr_of!(_3.3);
RET = _3.3.2;
_2.1 = _3.3.1;
_3.0 = core::ptr::addr_of!(_3.3);
Goto(bb1)
}
bb1 = {
_2.0 = _3.4 as u128;
_6 = '\u{4bc5d}';
_3.3.1 = core::ptr::addr_of_mut!(_6);
_3.3 = (_2.0, _2.1, RET);
_7 = 14106396801498103817_usize as f64;
_2.1 = core::ptr::addr_of_mut!(_6);
_2.1 = _3.3.1;
_6 = '\u{512f1}';
_6 = '\u{6d08c}';
_2 = _3.3;
_3.0 = core::ptr::addr_of!(_2);
_3.4 = (-116_i8) >> _3.1;
_3.2 = [_3.4,_3.4];
_3.3.1 = core::ptr::addr_of_mut!(_6);
_3.3.0 = _1;
_2.2 = _3.3.2;
_3.4 = (-12_i8) << _3.1;
_3.1 = -9223372036854775807_isize;
_8 = _3.1 * _3.1;
RET = _2.2;
_3.3.2 = _2.2;
Goto(bb2)
}
bb2 = {
_3.0 = core::ptr::addr_of!(_3.3);
_2.0 = _3.3.0 ^ _3.3.0;
_13 = 7_usize & 10574526636416375082_usize;
_7 = _13 as f64;
_10 = (-129885015769740353151416098349031422481_i128);
_2 = (_1, _3.3.1, _3.3.2);
RET = _3.3.2;
_4 = false ^ false;
_3.3 = (_2.0, _2.1, _2.2);
_3.3.0 = !_1;
Call(_3.3.0 = fn14(_2.1, _2, _3.4, _2.2, _3.4, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3.0 = core::ptr::addr_of!(_2);
_2 = (_1, _3.3.1, _3.3.2);
RET = _2.2;
_15 = !_4;
_3.2 = [_3.4,_3.4];
_9 = _4;
_2.0 = _1;
_1 = _2.0;
_7 = 251_u8 as f64;
_4 = _9;
_7 = _3.4 as f64;
_3.3 = _2;
_16 = !_4;
_3.4 = (-68_i8) & (-89_i8);
_3.4 = 27_i8 << _13;
_1 = _2.0 - _2.0;
_2.0 = _3.3.0;
_19.2 = !_16;
_19.0 = 15138732121842438268_u64 << _3.4;
_11 = _7 - _7;
_2.1 = _3.3.1;
_16 = _8 == _8;
_2.0 = _1;
_3.3.2 = _2.2;
_19 = (6725139693778355036_u64, (-1573960313_i32), _16);
_19.1 = (-166845511_i32) - 827096470_i32;
match _19.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
6725139693778355036 => bb8,
_ => bb7
}
}
bb4 = {
_3.0 = core::ptr::addr_of!(_3.3);
_2.0 = _3.3.0 ^ _3.3.0;
_13 = 7_usize & 10574526636416375082_usize;
_7 = _13 as f64;
_10 = (-129885015769740353151416098349031422481_i128);
_2 = (_1, _3.3.1, _3.3.2);
RET = _3.3.2;
_4 = false ^ false;
_3.3 = (_2.0, _2.1, _2.2);
_3.3.0 = !_1;
Call(_3.3.0 = fn14(_2.1, _2, _3.4, _2.2, _3.4, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2.0 = _3.4 as u128;
_6 = '\u{4bc5d}';
_3.3.1 = core::ptr::addr_of_mut!(_6);
_3.3 = (_2.0, _2.1, RET);
_7 = 14106396801498103817_usize as f64;
_2.1 = core::ptr::addr_of_mut!(_6);
_2.1 = _3.3.1;
_6 = '\u{512f1}';
_6 = '\u{6d08c}';
_2 = _3.3;
_3.0 = core::ptr::addr_of!(_2);
_3.4 = (-116_i8) >> _3.1;
_3.2 = [_3.4,_3.4];
_3.3.1 = core::ptr::addr_of_mut!(_6);
_3.3.0 = _1;
_2.2 = _3.3.2;
_3.4 = (-12_i8) << _3.1;
_3.1 = -9223372036854775807_isize;
_8 = _3.1 * _3.1;
RET = _2.2;
_3.3.2 = _2.2;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_6 = '\u{59a2d}';
_15 = _9 == _16;
_7 = -_11;
_13 = _11 as usize;
_13 = 15954191634238754467_usize & 14609808135724434579_usize;
_2.1 = _3.3.1;
_17 = Adt58::Variant0 { fld0: _19.1 };
_2 = (_1, _3.3.1, RET);
_3.3.2 = _2.2;
_9 = !_4;
_8 = _3.1 >> _3.4;
_7 = _11;
_19 = (5610839769143987082_u64, Field::<i32>(Variant(_17, 0), 0), _16);
_3.3.0 = 56882_u16 as u128;
_9 = _7 > _7;
_3.3.1 = core::ptr::addr_of_mut!(_6);
_20 = [61_u8,194_u8,20_u8,148_u8,105_u8];
_12 = [_9,_15,_9,_19.2,_9,_9,_15,_9];
RET = _3.3.2;
RET = _2.2;
match _19.0 {
0 => bb7,
1 => bb9,
5610839769143987082 => bb11,
_ => bb10
}
}
bb9 = {
_2.0 = _3.4 as u128;
_6 = '\u{4bc5d}';
_3.3.1 = core::ptr::addr_of_mut!(_6);
_3.3 = (_2.0, _2.1, RET);
_7 = 14106396801498103817_usize as f64;
_2.1 = core::ptr::addr_of_mut!(_6);
_2.1 = _3.3.1;
_6 = '\u{512f1}';
_6 = '\u{6d08c}';
_2 = _3.3;
_3.0 = core::ptr::addr_of!(_2);
_3.4 = (-116_i8) >> _3.1;
_3.2 = [_3.4,_3.4];
_3.3.1 = core::ptr::addr_of_mut!(_6);
_3.3.0 = _1;
_2.2 = _3.3.2;
_3.4 = (-12_i8) << _3.1;
_3.1 = -9223372036854775807_isize;
_8 = _3.1 * _3.1;
RET = _2.2;
_3.3.2 = _2.2;
Goto(bb2)
}
bb10 = {
_3.0 = core::ptr::addr_of!(_2);
_2 = (_1, _3.3.1, _3.3.2);
RET = _2.2;
_15 = !_4;
_3.2 = [_3.4,_3.4];
_9 = _4;
_2.0 = _1;
_1 = _2.0;
_7 = 251_u8 as f64;
_4 = _9;
_7 = _3.4 as f64;
_3.3 = _2;
_16 = !_4;
_3.4 = (-68_i8) & (-89_i8);
_3.4 = 27_i8 << _13;
_1 = _2.0 - _2.0;
_2.0 = _3.3.0;
_19.2 = !_16;
_19.0 = 15138732121842438268_u64 << _3.4;
_11 = _7 - _7;
_2.1 = _3.3.1;
_16 = _8 == _8;
_2.0 = _1;
_3.3.2 = _2.2;
_19 = (6725139693778355036_u64, (-1573960313_i32), _16);
_19.1 = (-166845511_i32) - 827096470_i32;
match _19.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
6725139693778355036 => bb8,
_ => bb7
}
}
bb11 = {
_7 = (-12982_i16) as f64;
_22 = _8;
_18 = !_9;
_3.3.2 = _2.2;
_1 = _3.3.0 & _2.0;
place!(Field::<i32>(Variant(_17, 0), 0)) = _10 as i32;
Goto(bb12)
}
bb12 = {
_27 = _8 ^ _22;
_3.3.0 = 1596_u16 as u128;
_4 = !_18;
_24 = [(-5739620201218580752_i64),3997944628300588103_i64,5722530640401981299_i64,1302256539959707347_i64,4851850602547252146_i64,(-1701513699575793806_i64),(-5017122748977888062_i64)];
_3.4 = (-93_i8);
_15 = _4 < _9;
_12 = [_15,_9,_4,_9,_16,_9,_9,_9];
_19.1 = _19.0 as i32;
_3.3.2 = _2.2;
_28 = Adt47::Variant0 { fld0: _27,fld1: _19.0 };
_1 = 2536245737_u32 as u128;
_29 = _10 as i16;
place!(Field::<isize>(Variant(_28, 0), 0)) = _22;
_19.1 = Field::<i32>(Variant(_17, 0), 0);
_3.3.1 = _2.1;
_2.0 = 836260862_u32 as u128;
_4 = _9;
_3.3.1 = _2.1;
_26 = _3.4 * _3.4;
_19 = (Field::<u64>(Variant(_28, 0), 1), Field::<i32>(Variant(_17, 0), 0), _18);
_15 = _9;
Call(_15 = fn15(_18, _11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3.3.2 = RET;
_19.0 = Field::<u64>(Variant(_28, 0), 1);
_7 = -_11;
_19.0 = Field::<u64>(Variant(_28, 0), 1) * Field::<u64>(Variant(_28, 0), 1);
_22 = -_8;
_28 = Adt47::Variant1 { fld0: 2746190839_u32 };
_3.3.2 = _2.2;
SetDiscriminant(_17, 3);
_1 = _2.0 * _2.0;
RET = _2.2;
place!(Field::<(u64, i32, bool)>(Variant(_17, 3), 3)) = (_19.0, _19.1, _19.2);
Goto(bb14)
}
bb14 = {
_18 = _15 ^ _4;
_7 = -_11;
place!(Field::<(*mut *mut char,)>(Variant(_17, 3), 2)).0 = core::ptr::addr_of_mut!(_3.3.1);
_23 = [22_u8,119_u8,14_u8,233_u8,124_u8];
RET = _3.3.2;
place!(Field::<i128>(Variant(_17, 3), 7)) = _10;
_17 = Adt58::Variant0 { fld0: _19.1 };
_19.1 = !Field::<i32>(Variant(_17, 0), 0);
_1 = _26 as u128;
_21 = core::ptr::addr_of!(_2);
_19.1 = _9 as i32;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(13_usize, 8_usize, Move(_8), 15_usize, Move(_15), 24_usize, Move(_24), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(13_usize, 1_usize, Move(_1), 22_usize, Move(_22), 23_usize, Move(_23), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(13_usize, 26_usize, Move(_26), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *mut char,mut _2: (u128, *mut char, *mut i64),mut _3: i8,mut _4: *mut i64,mut _5: i8,mut _6: f64) -> u128 {
mir! {
type RET = u128;
let _7: [bool; 8];
let _8: (u64, i32, bool);
let _9: Adt46;
let _10: Adt43;
let _11: Adt47;
let _12: f32;
let _13: bool;
let _14: [bool; 8];
let _15: (u64, i32, bool);
let _16: Adt53;
let _17: char;
let _18: u16;
let _19: i8;
let _20: i16;
let _21: ();
let _22: ();
{
RET = _2.0 * _2.0;
(*_1) = '\u{10375d}';
_6 = 15584_u16 as f64;
_2.0 = RET;
_7 = [true,false,true,true,false,false,true,false];
Goto(bb1)
}
bb1 = {
_2.2 = _4;
_2.0 = 181_u8 as u128;
_8.0 = _3 as u64;
(*_1) = '\u{9d89a}';
_8 = (4452066216272117889_u64, 1275244897_i32, false);
_4 = _2.2;
_7 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_7 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_6 = 9189805164675343308_i64 as f64;
(*_1) = '\u{5f880}';
_2.0 = RET << _8.1;
_5 = -_3;
(*_1) = '\u{82094}';
_8.0 = !18370062133689107711_u64;
(*_1) = '\u{102a86}';
_8 = (4726238560046917552_u64, 1968955269_i32, true);
Goto(bb2)
}
bb2 = {
Goto(bb3)
}
bb3 = {
RET = _2.0 + _2.0;
_2.0 = _8.2 as u128;
_8.2 = !false;
RET = _2.0;
_8.0 = !9305326612489549786_u64;
_8.2 = _3 != _5;
(*_1) = '\u{6ca6c}';
_12 = 0_usize as f32;
_2.1 = core::ptr::addr_of_mut!((*_1));
Goto(bb4)
}
bb4 = {
_2 = (RET, _1, _4);
_8.2 = !true;
_7 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_8.2 = false;
_8 = (2494931700212434902_u64, (-1773843836_i32), true);
_2.2 = _4;
_2.1 = _1;
_5 = _3 & _3;
_11 = Adt47::Variant0 { fld0: 9223372036854775807_isize,fld1: _8.0 };
place!(Field::<u64>(Variant(_11, 0), 1)) = _8.2 as u64;
place!(Field::<isize>(Variant(_11, 0), 0)) = _8.2 as isize;
SetDiscriminant(_11, 0);
_11 = Adt47::Variant2 { fld0: RET,fld1: 28375_u16 };
_6 = (-31523_i16) as f64;
match _8.1 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
340282366920938463463374607429994367620 => bb10,
_ => bb9
}
}
bb5 = {
RET = _2.0 + _2.0;
_2.0 = _8.2 as u128;
_8.2 = !false;
RET = _2.0;
_8.0 = !9305326612489549786_u64;
_8.2 = _3 != _5;
(*_1) = '\u{6ca6c}';
_12 = 0_usize as f32;
_2.1 = core::ptr::addr_of_mut!((*_1));
Goto(bb4)
}
bb6 = {
Goto(bb3)
}
bb7 = {
_2.2 = _4;
_2.0 = 181_u8 as u128;
_8.0 = _3 as u64;
(*_1) = '\u{9d89a}';
_8 = (4452066216272117889_u64, 1275244897_i32, false);
_4 = _2.2;
_7 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_7 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_6 = 9189805164675343308_i64 as f64;
(*_1) = '\u{5f880}';
_2.0 = RET << _8.1;
_5 = -_3;
(*_1) = '\u{82094}';
_8.0 = !18370062133689107711_u64;
(*_1) = '\u{102a86}';
_8 = (4726238560046917552_u64, 1968955269_i32, true);
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_2.0 = !RET;
place!(Field::<u16>(Variant(_11, 2), 1)) = 4198_u16 * 39989_u16;
_2 = (RET, _1, _4);
(*_1) = '\u{c3d9f}';
place!(Field::<u16>(Variant(_11, 2), 1)) = !20840_u16;
_11 = Adt47::Variant2 { fld0: _2.0,fld1: 63064_u16 };
RET = 3454010561_u32 as u128;
_13 = _8.2;
_3 = _5 ^ _5;
_11 = Adt47::Variant1 { fld0: 2537537274_u32 };
place!(Field::<u32>(Variant(_11, 1), 0)) = !3458441097_u32;
_14 = _7;
match _8.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463463374607429994367620 => bb11,
_ => bb8
}
}
bb11 = {
_15 = (_8.0, _8.1, _13);
_8 = _15;
_5 = _3 - _3;
Goto(bb12)
}
bb12 = {
_15.0 = !_8.0;
_15.0 = (*_1) as u64;
_15.1 = -_8.1;
_15.2 = _8.2;
(*_1) = '\u{69de1}';
_11 = Adt47::Variant2 { fld0: _2.0,fld1: 39155_u16 };
place!(Field::<u16>(Variant(_11, 2), 1)) = 52817_u16;
_7 = _14;
_15.2 = !_13;
_8.0 = _15.0;
_17 = (*_1);
_15.1 = _8.1 | _8.1;
SetDiscriminant(_11, 1);
_16.fld0 = _8.2 ^ _15.2;
_2.2 = _4;
match _8.1 {
0 => bb1,
1 => bb7,
2 => bb11,
340282366920938463463374607429994367620 => bb13,
_ => bb5
}
}
bb13 = {
_14 = [_16.fld0,_13,_16.fld0,_13,_15.2,_13,_8.2,_13];
_2 = (RET, _1, _4);
_19 = _5 - _3;
_17 = (*_1);
_18 = 20579_u16;
_11 = Adt47::Variant2 { fld0: RET,fld1: _18 };
_16.fld0 = _15.2;
_2.1 = core::ptr::addr_of_mut!((*_1));
_15 = (_8.0, _8.1, _13);
_8.0 = _15.0;
(*_1) = _17;
_3 = _19 - _19;
Call(_3 = core::intrinsics::transmute(_19), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15 = _8;
_16 = Adt53 { fld0: _8.2 };
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(14_usize, 17_usize, Move(_17), 3_usize, Move(_3), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(14_usize, 14_usize, Move(_14), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: bool,mut _2: f64) -> bool {
mir! {
type RET = bool;
let _3: f32;
let _4: ();
let _5: ();
{
RET = _1;
_1 = !RET;
_1 = RET < RET;
_1 = !RET;
RET = !_1;
_1 = !RET;
_1 = RET;
_1 = RET;
RET = _1;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *mut [i64; 7],mut _2: *mut [i64; 7],mut _3: *mut [i64; 7],mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: i32,mut _9: *mut [i64; 7]) -> u8 {
mir! {
type RET = u8;
let _10: f32;
let _11: [i64; 7];
let _12: f32;
let _13: [u8; 5];
let _14: f32;
let _15: *const [i8; 2];
let _16: char;
let _17: usize;
let _18: u32;
let _19: isize;
let _20: *mut *mut char;
let _21: char;
let _22: char;
let _23: *mut [i64; 7];
let _24: [bool; 8];
let _25: i16;
let _26: i8;
let _27: [u8; 5];
let _28: i8;
let _29: Adt53;
let _30: bool;
let _31: u32;
let _32: [bool; 8];
let _33: ();
let _34: ();
{
_5 = _6 != _4;
RET = 77605151718359303722496272227338626191_i128 as u8;
(*_9) = [1797912688574114833_i64,617153447553529484_i64,5332166459027910180_i64,(-6032355735873161631_i64),(-3341772886475322078_i64),(-8846224955668463358_i64),8110133322800295723_i64];
(*_9) = [3280310731393516261_i64,(-6847024035045305943_i64),598311771639272913_i64,2519850800485981272_i64,(-8055389975372172698_i64),89829070989802858_i64,(-949775704649491993_i64)];
RET = 55_i8 as u8;
_9 = _2;
_9 = _2;
_10 = 114662436074516815266726633186131042831_u128 as f32;
_2 = _9;
_5 = _4 == _6;
_12 = _10;
_14 = 9223372036854775807_isize as f32;
RET = _14 as u8;
RET = 159_u8;
_9 = _2;
_2 = _3;
_11 = [6430219939576108101_i64,4978346880485884333_i64,222967408092432114_i64,1813476246738405728_i64,1552664887826024099_i64,(-1741873202094332336_i64),(-8976988856982583226_i64)];
_3 = _2;
_13 = [RET,RET,RET,RET,RET];
Goto(bb1)
}
bb1 = {
_5 = !_7;
RET = 70_u8;
_9 = core::ptr::addr_of_mut!(_11);
(*_9) = [(-4226389265518154723_i64),(-7710149518461738554_i64),(-2601059573005267639_i64),(-5138842279453614001_i64),(-8762576648562241390_i64),3109959580488928139_i64,7401995751344179192_i64];
_8 = -2030546194_i32;
_12 = RET as f32;
_3 = _2;
_10 = -_12;
RET = (-47708532651346064634065161882778575614_i128) as u8;
_2 = core::ptr::addr_of_mut!(_11);
_9 = _3;
_10 = -_14;
_16 = '\u{6c308}';
_1 = core::ptr::addr_of_mut!(_11);
_10 = _14 * _14;
_14 = -_12;
_16 = '\u{263ff}';
(*_1) = [(-1632782098561766132_i64),(-1198198905594101336_i64),(-6485343175070414822_i64),4590971142111106930_i64,(-939970444584151373_i64),(-5111346924515499233_i64),1092319367102109455_i64];
(*_2) = [2499312544224323033_i64,(-226227598018974088_i64),(-7165827309847924675_i64),(-2725538629630383156_i64),3934983825814011021_i64,(-3599248445955455129_i64),(-9101002659117059905_i64)];
_12 = -_14;
_8 = _16 as i32;
(*_2) = [5883842078679489014_i64,(-7566399538063958710_i64),(-5032904302514278837_i64),(-7611470232756426129_i64),(-2280840603723289669_i64),2955017426094499579_i64,(-621794097094085889_i64)];
RET = 180_u8;
Goto(bb2)
}
bb2 = {
_1 = core::ptr::addr_of_mut!(_11);
_8 = -(-138795861_i32);
_13 = [RET,RET,RET,RET,RET];
(*_1) = [5422755659428945457_i64,(-7707010511907614791_i64),7893197561056738743_i64,(-8807063137748684803_i64),2912110064808428887_i64,9098052591276638643_i64,(-7425411180669853072_i64)];
_18 = 2683113521_u32 << RET;
_10 = _12;
_10 = 45201369720862268639512208122733355941_i128 as f32;
_19 = (-9223372036854775808_isize) - 5_isize;
match RET {
180 => bb3,
_ => bb1
}
}
bb3 = {
_19 = (-85_isize) + 8_isize;
match RET {
180 => bb5,
_ => bb4
}
}
bb4 = {
_1 = core::ptr::addr_of_mut!(_11);
_8 = -(-138795861_i32);
_13 = [RET,RET,RET,RET,RET];
(*_1) = [5422755659428945457_i64,(-7707010511907614791_i64),7893197561056738743_i64,(-8807063137748684803_i64),2912110064808428887_i64,9098052591276638643_i64,(-7425411180669853072_i64)];
_18 = 2683113521_u32 << RET;
_10 = _12;
_10 = 45201369720862268639512208122733355941_i128 as f32;
_19 = (-9223372036854775808_isize) - 5_isize;
match RET {
180 => bb3,
_ => bb1
}
}
bb5 = {
_14 = _10;
_4 = _6 <= _7;
_21 = _16;
(*_2) = [5756192328688767933_i64,2236848641874191761_i64,4804606936161047007_i64,983270413810324342_i64,8185931053214728336_i64,(-2629051717171057234_i64),9010101412149084445_i64];
(*_1) = [7179437855064916506_i64,(-3625161016076019537_i64),5468507234598964356_i64,(-3664792529753176924_i64),(-2588012585140276182_i64),(-4262959644151162469_i64),(-3051569598828597233_i64)];
_2 = _9;
_22 = _16;
_12 = -_14;
_11 = [6437446830257274787_i64,7311978757522288420_i64,(-5182021331926254941_i64),17231049520567717_i64,(-8694039223713300034_i64),(-4721656875682145385_i64),(-4318118909044028457_i64)];
_2 = core::ptr::addr_of_mut!((*_1));
_9 = core::ptr::addr_of_mut!((*_2));
_17 = 14994037962586889654_usize;
(*_2) = [8701852526964707355_i64,519060762951012747_i64,6509962763641872443_i64,(-4912865060866096492_i64),1632739293971634084_i64,(-3189726998797311017_i64),(-2519859176042823985_i64)];
(*_9) = [3013933060903736057_i64,2626496194444261453_i64,(-5735013975442471245_i64),4278355153350646694_i64,(-3841806977403241790_i64),(-8245166706870051624_i64),(-2611628202906464164_i64)];
_11 = [6021831491907250078_i64,(-1445621245341646469_i64),(-802373349504532642_i64),6229271020891326302_i64,(-201906755947748611_i64),(-6331955581103156621_i64),3731155701338336848_i64];
_10 = -_14;
_14 = _10 * _12;
_13 = [RET,RET,RET,RET,RET];
_5 = _4;
_21 = _16;
_5 = _4 < _7;
_2 = _9;
_16 = _22;
_24 = [_7,_4,_7,_5,_4,_4,_7,_4];
(*_2) = [(-7535915035751347905_i64),6212631718776142555_i64,(-3477885192196609927_i64),(-5764740486488092157_i64),(-6137241265487290569_i64),966175908099715454_i64,(-3815897754519887654_i64)];
Call(_14 = fn17(_24, _16, _2, _5, _5, (*_2), _5, (*_9), _4, _7, _3, _24, _6, _5, _24, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_23 = core::ptr::addr_of_mut!(_11);
(*_9) = [(-2826376019814685575_i64),(-753943633638626258_i64),1552708612477830362_i64,4222842330157961947_i64,(-968311762615918539_i64),(-4643456377085497103_i64),4480733301471533380_i64];
(*_1) = [(-717955103430448061_i64),(-2469286052420416375_i64),(-4280034362322408475_i64),(-1303701100020858107_i64),(-5074894487582091592_i64),3123473923139604076_i64,7386341621556498118_i64];
_4 = !_5;
_19 = !(-115_isize);
_21 = _22;
_19 = 4739559129925986748_i64 as isize;
_13 = [RET,RET,RET,RET,RET];
_12 = _14;
_18 = !533323623_u32;
(*_23) = [4132691802103001398_i64,5983513190967327114_i64,8307304503020380589_i64,7813747682698226700_i64,(-7565227099205243488_i64),8679309183490286733_i64,5767645426398170843_i64];
_21 = _16;
_27 = _13;
_1 = _3;
_14 = -_10;
_17 = 3_usize;
_27 = [RET,_13[_17],_13[_17],_13[_17],_13[_17]];
(*_2) = [1525363537236810361_i64,5566106688008029318_i64,7746787029860184094_i64,(-8440993762339588887_i64),6264370250196148487_i64,7276770462168364930_i64,6165436732966748078_i64];
Call((*_9)[_17] = core::intrinsics::transmute(_24), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_30 = !_4;
Call(RET = core::intrinsics::transmute(_30), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10 = -_12;
(*_9) = [(-4504278911443110030_i64),(-341769549482009347_i64),3229233979590229978_i64,(-5523684725922073985_i64),4997351346181860139_i64,(-5403494932919604130_i64),66973672968291898_i64];
_9 = core::ptr::addr_of_mut!((*_23));
_28 = (-42_i8);
(*_9)[_17] = 2449290872671120563_i64 * 5671946204318130621_i64;
_25 = RET as i16;
_11[_17] = 7103978753568038433_i64;
_26 = _14 as i8;
_3 = _23;
_16 = _21;
_12 = _14 - _14;
(*_3)[_17] = (-7622820593177279555_i64) - 3963003695324235699_i64;
_32 = [_7,_4,_6,_4,_24[_17],_6,_30,_30];
Goto(bb9)
}
bb9 = {
Call(_33 = dump_var(16_usize, 24_usize, Move(_24), 7_usize, Move(_7), 8_usize, Move(_8), 13_usize, Move(_13)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_33 = dump_var(16_usize, 4_usize, Move(_4), 18_usize, Move(_18), 28_usize, Move(_28), 22_usize, Move(_22)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_33 = dump_var(16_usize, 21_usize, Move(_21), 11_usize, Move(_11), 34_usize, _34, 34_usize, _34), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [bool; 8],mut _2: char,mut _3: *mut [i64; 7],mut _4: bool,mut _5: bool,mut _6: [i64; 7],mut _7: bool,mut _8: [i64; 7],mut _9: bool,mut _10: bool,mut _11: *mut [i64; 7],mut _12: [bool; 8],mut _13: bool,mut _14: bool,mut _15: [bool; 8],mut _16: [bool; 8]) -> f32 {
mir! {
type RET = f32;
let _17: f64;
let _18: isize;
let _19: char;
let _20: Adt53;
let _21: Adt53;
let _22: f64;
let _23: u64;
let _24: u16;
let _25: Adt53;
let _26: usize;
let _27: Adt47;
let _28: isize;
let _29: Adt53;
let _30: isize;
let _31: Adt42;
let _32: [i64; 7];
let _33: u64;
let _34: f64;
let _35: [i64; 7];
let _36: *mut i64;
let _37: u16;
let _38: Adt53;
let _39: isize;
let _40: char;
let _41: f32;
let _42: *const isize;
let _43: u32;
let _44: isize;
let _45: ();
let _46: ();
{
_11 = _3;
_8 = [3841754653505006802_i64,2241135757155218792_i64,3029138649186456187_i64,3838116846045097481_i64,(-8375385594276468402_i64),(-7633502178662023445_i64),(-682677353347586676_i64)];
_12 = [_13,_5,_5,_14,_7,_9,_10,_5];
_1 = [_5,_4,_5,_14,_14,_5,_14,_7];
_14 = _7;
RET = (-5445_i16) as f32;
(*_11) = _6;
_9 = !_4;
_10 = !_14;
(*_11) = _8;
_12 = [_9,_4,_5,_5,_10,_9,_4,_10];
RET = 280906135803901752760229303012322173595_u128 as f32;
Goto(bb1)
}
bb1 = {
(*_3) = [(-1789209362876557716_i64),(-1793954597526350722_i64),946003218200269517_i64,(-7284521831071942634_i64),(-3863105083265501586_i64),(-5089744971372621041_i64),2253056962277706895_i64];
RET = 8_i8 as f32;
_15 = [_14,_14,_9,_5,_5,_5,_14,_7];
_1 = _16;
_14 = !_9;
_15 = _1;
_5 = !_7;
_20 = Adt53 { fld0: _7 };
_10 = _14;
_8 = (*_11);
_10 = !_9;
_13 = _10 ^ _14;
RET = 2533309929733841158_i64 as f32;
RET = 4197814563650598457_i64 as f32;
_1 = [_10,_7,_14,_7,_5,_9,_9,_9];
_3 = _11;
_5 = _20.fld0 ^ _20.fld0;
_15 = [_5,_4,_13,_9,_4,_7,_14,_14];
_13 = _5 | _5;
_20 = Adt53 { fld0: _5 };
_13 = _14;
_18 = (-9223372036854775808_isize);
_20 = Adt53 { fld0: _13 };
_4 = _9;
_13 = !_5;
_17 = 56385003544433732108768501080077796510_u128 as f64;
Goto(bb2)
}
bb2 = {
_11 = _3;
_17 = 891094969_i32 as f64;
Goto(bb3)
}
bb3 = {
_21.fld0 = _13;
RET = 110_i8 as f32;
_14 = _21.fld0;
_7 = !_10;
_19 = _2;
_14 = !_9;
_12 = [_21.fld0,_20.fld0,_13,_14,_13,_4,_20.fld0,_9];
_10 = _13;
_11 = core::ptr::addr_of_mut!((*_11));
_17 = (-126814935706354421011991239782242585725_i128) as f64;
_7 = _10 & _5;
match _18 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb4 = {
_11 = _3;
_17 = 891094969_i32 as f64;
Goto(bb3)
}
bb5 = {
(*_3) = [(-1789209362876557716_i64),(-1793954597526350722_i64),946003218200269517_i64,(-7284521831071942634_i64),(-3863105083265501586_i64),(-5089744971372621041_i64),2253056962277706895_i64];
RET = 8_i8 as f32;
_15 = [_14,_14,_9,_5,_5,_5,_14,_7];
_1 = _16;
_14 = !_9;
_15 = _1;
_5 = !_7;
_20 = Adt53 { fld0: _7 };
_10 = _14;
_8 = (*_11);
_10 = !_9;
_13 = _10 ^ _14;
RET = 2533309929733841158_i64 as f32;
RET = 4197814563650598457_i64 as f32;
_1 = [_10,_7,_14,_7,_5,_9,_9,_9];
_3 = _11;
_5 = _20.fld0 ^ _20.fld0;
_15 = [_5,_4,_13,_9,_4,_7,_14,_14];
_13 = _5 | _5;
_20 = Adt53 { fld0: _5 };
_13 = _14;
_18 = (-9223372036854775808_isize);
_20 = Adt53 { fld0: _13 };
_4 = _9;
_13 = !_5;
_17 = 56385003544433732108768501080077796510_u128 as f64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_5 = _4;
_1 = [_21.fld0,_21.fld0,_4,_21.fld0,_9,_5,_9,_10];
_23 = !4243343947692147012_u64;
_16 = _1;
_16 = _12;
_20.fld0 = _10;
_3 = core::ptr::addr_of_mut!((*_11));
(*_3) = [(-8358754820537748364_i64),8423711401285645803_i64,(-4416364791349947171_i64),3229697835866950836_i64,(-46273151878934303_i64),(-7830740879657282698_i64),(-293037692563744230_i64)];
RET = 3455826372_u32 as f32;
_26 = 2_usize;
_18 = 1160657235_u32 as isize;
(*_3)[_26] = !_8[_26];
(*_11)[_26] = -_8[_26];
RET = (-28370_i16) as f32;
_9 = _16[_26];
_8 = (*_11);
_15[_26] = _16[_26] == _9;
_15 = _12;
_26 = 10589568713684405656_usize * 0_usize;
_9 = _7;
_17 = 44_i8 as f64;
_27 = Adt47::Variant2 { fld0: 182458034168859433120146155705350010695_u128,fld1: 9536_u16 };
Goto(bb9)
}
bb9 = {
_23 = _26 as u64;
_24 = 4736_u16;
_17 = RET as f64;
_14 = !_10;
_21 = _20;
_18 = (-9223372036854775808_isize);
_16 = _12;
_14 = _13;
_27 = Adt47::Variant0 { fld0: _18,fld1: _23 };
_6 = [(-7195143203522326371_i64),(-7061434493002847551_i64),4870626101597980590_i64,(-8585262411487306133_i64),7886207516942241332_i64,(-2020235669618835911_i64),(-7491034465509692570_i64)];
_27 = Adt47::Variant2 { fld0: 175466342848567719498338090142741709754_u128,fld1: _24 };
_27 = Adt47::Variant0 { fld0: _18,fld1: _23 };
_1 = _15;
_21 = Adt53 { fld0: _7 };
_21.fld0 = _7 | _5;
Goto(bb10)
}
bb10 = {
(*_3) = [1917446348681494630_i64,(-647558918724161924_i64),1703018389442369175_i64,(-4076654512609301543_i64),4715220515827861981_i64,(-5724977680562818355_i64),(-693605570761640657_i64)];
_10 = !_9;
_27 = Adt47::Variant1 { fld0: 508169856_u32 };
_10 = !_20.fld0;
_28 = _20.fld0 as isize;
RET = 3476290821_u32 as f32;
_32 = [5450707766133006587_i64,1794281681401237618_i64,(-9081582207077123915_i64),(-6438475453184707472_i64),(-1309579181441584228_i64),(-5456451489244774360_i64),742975771579347449_i64];
_11 = _3;
RET = _24 as f32;
(*_11) = [8135059379406522400_i64,6956980220160444109_i64,4442752317038159055_i64,3429430388167913503_i64,8202547820732380799_i64,(-4155229581066949393_i64),(-6251044258295508636_i64)];
_32 = [(-3420497861867025562_i64),195635893747581457_i64,(-1451663035348402312_i64),5678697711949202582_i64,6254894845734712793_i64,(-177912464464266254_i64),111519896358719194_i64];
_30 = _28;
_20.fld0 = _9;
_27 = Adt47::Variant2 { fld0: 96351318308000274311422533274046660061_u128,fld1: _24 };
_32 = [(-956674550964480270_i64),(-3464869473607644737_i64),2142381963181606633_i64,8371803336179415612_i64,(-4053520619285348409_i64),(-3196538069077130643_i64),(-8763821692144069388_i64)];
place!(Field::<u128>(Variant(_27, 2), 0)) = !261628019215565427609149206420223869189_u128;
_10 = _7 | _9;
_20 = Adt53 { fld0: _10 };
Call(_34 = core::intrinsics::fmaf64(_17, _17, _17), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_5 = !_20.fld0;
_4 = _5;
_6 = (*_11);
_18 = _28;
_23 = 12761286211675233744_u64 & 17708052877165572090_u64;
_22 = _17 - _17;
_32 = [5739889134329754561_i64,5040336928875117810_i64,(-5229338697952800108_i64),2428633656600547472_i64,(-8096072345681405608_i64),(-6625026875857892563_i64),6938959115602789763_i64];
_24 = _14 as u16;
SetDiscriminant(_27, 0);
_6 = [820173374731338858_i64,9153687017720595458_i64,8097614840237450364_i64,4308449217507123562_i64,3840050225056838987_i64,1918845028463574964_i64,473029653113102939_i64];
place!(Field::<u64>(Variant(_27, 0), 1)) = _26 as u64;
_14 = _9 != _5;
_30 = !_18;
_18 = !_30;
_21 = _20;
Call(_21.fld0 = fn18(_14, _18, _4, _20.fld0, _5, _9, _16, _18, _7, _12, _4, _9), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7 = _13;
_21.fld0 = _10 <= _4;
_21 = Adt53 { fld0: _20.fld0 };
_19 = _2;
_1 = _12;
_27 = Adt47::Variant0 { fld0: _18,fld1: _23 };
RET = 7979106798509428499_i64 as f32;
place!(Field::<u64>(Variant(_27, 0), 1)) = _23 | _23;
_22 = -_17;
_25 = Adt53 { fld0: _10 };
_25 = Adt53 { fld0: _10 };
_33 = !Field::<u64>(Variant(_27, 0), 1);
_6 = [2167278434438803149_i64,8943365353439779692_i64,(-7564442281697775899_i64),(-3935965765319044468_i64),5167832582408338633_i64,(-8239431055432493429_i64),(-4781877331688109654_i64)];
(*_11) = [6166361938709280209_i64,6229444447352295592_i64,8089698822662632598_i64,(-5203677132406741099_i64),(-2142369243108181645_i64),(-3635112068435545130_i64),(-5323358052284073324_i64)];
_11 = _3;
_29 = _21;
_29 = _21;
_30 = _28;
_7 = _25.fld0 < _21.fld0;
_40 = _19;
(*_11) = [(-3184262430395267309_i64),(-1985270971493549329_i64),2416696605739560648_i64,1044595655318439979_i64,(-6181219303783545588_i64),6587914660299190316_i64,675154994094730384_i64];
_42 = core::ptr::addr_of!(_30);
_32 = [6996632824559855398_i64,(-6128161701958352438_i64),(-2560144791739799598_i64),252868682130334296_i64,7220158054915079978_i64,1406258386682034520_i64,(-3459296885759315808_i64)];
Goto(bb13)
}
bb13 = {
_12 = _16;
(*_42) = _28;
(*_3) = _6;
_18 = (*_42);
_18 = -(*_42);
(*_11) = [5708756496853477630_i64,6353371610117941091_i64,2353698825885258947_i64,260036331038549331_i64,5584378988126596710_i64,1448612467434456044_i64,(-6998422007745704892_i64)];
(*_42) = _18 | _18;
_12 = [_7,_4,_4,_14,_29.fld0,_25.fld0,_7,_10];
Goto(bb14)
}
bb14 = {
_25.fld0 = _21.fld0 ^ _7;
_35 = [(-6424254698079040598_i64),5063643936086793278_i64,651581042547502851_i64,(-580419411287434198_i64),1071440742627095546_i64,(-1528116595903916856_i64),(-2599380235257748117_i64)];
_21 = Adt53 { fld0: _20.fld0 };
_21.fld0 = _14 | _4;
_25.fld0 = _14 & _21.fld0;
_38 = Adt53 { fld0: _4 };
(*_3) = [1664923208534063877_i64,7874068848056950925_i64,2704383683381377456_i64,(-2777379466294539983_i64),(-2535235387233028919_i64),6980191843363523965_i64,3162129513577756553_i64];
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(17_usize, 2_usize, Move(_2), 40_usize, Move(_40), 16_usize, Move(_16), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(17_usize, 28_usize, Move(_28), 33_usize, Move(_33), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(17_usize, 14_usize, Move(_14), 10_usize, Move(_10), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: bool,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: [bool; 8],mut _8: isize,mut _9: bool,mut _10: [bool; 8],mut _11: bool,mut _12: bool) -> bool {
mir! {
type RET = bool;
let _13: [u8; 5];
let _14: Adt47;
let _15: *mut u128;
let _16: ();
let _17: ();
{
_9 = _11 == _6;
_10 = _7;
_4 = !_9;
_3 = _1 == _9;
_5 = _4;
Goto(bb1)
}
bb1 = {
RET = _6 | _11;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(18_usize, 2_usize, Move(_2), 8_usize, Move(_8), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_16 = dump_var(18_usize, 12_usize, Move(_12), 6_usize, Move(_6), 17_usize, _17, 17_usize, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i32,mut _2: *mut [i64; 7],mut _3: Adt53,mut _4: u64,mut _5: bool,mut _6: (u64, i32, bool)) -> i16 {
mir! {
type RET = i16;
let _7: *mut *mut u128;
let _8: Adt53;
let _9: [u16; 1];
let _10: *mut u32;
let _11: i16;
let _12: bool;
let _13: [i8; 2];
let _14: Adt50;
let _15: [u16; 1];
let _16: [i8; 2];
let _17: char;
let _18: isize;
let _19: u32;
let _20: isize;
let _21: Adt53;
let _22: bool;
let _23: char;
let _24: [u16; 1];
let _25: usize;
let _26: Adt47;
let _27: Adt53;
let _28: ();
let _29: ();
{
_1 = _6.1;
_6.2 = _5;
_5 = _3.fld0;
RET = 88081257897518209907840606748240702821_i128 as i16;
_5 = _6.2;
_6.1 = _1 >> _1;
_4 = (-57_isize) as u64;
RET = _3.fld0 as i16;
_3.fld0 = _5 ^ _6.2;
_5 = _6.2 | _3.fld0;
_6.2 = !_3.fld0;
RET = (-5708_i16) - (-12978_i16);
_6.2 = _5 >= _5;
_4 = !_6.0;
_6.1 = 108028951842125168902763800943896552072_i128 as i32;
_6.1 = _1 + _1;
_3 = Adt53 { fld0: _5 };
RET = (-88_i8) as i16;
Goto(bb1)
}
bb1 = {
_6 = (_4, _1, _3.fld0);
_3 = Adt53 { fld0: _5 };
_3 = Adt53 { fld0: _6.2 };
_6.0 = !_4;
_6.0 = _4 - _4;
_3 = Adt53 { fld0: _6.2 };
_4 = _6.0;
_6.2 = !_3.fld0;
_6.2 = _5;
RET = (-11761_i16) << _6.0;
_6.1 = -_1;
_3 = Adt53 { fld0: _6.2 };
_3.fld0 = !_6.2;
_5 = !_6.2;
_8 = Adt53 { fld0: _5 };
Goto(bb2)
}
bb2 = {
_5 = _3.fld0;
_6.2 = !_5;
_6.0 = _4 & _4;
_6.2 = !_5;
Goto(bb3)
}
bb3 = {
_8 = _3;
_8.fld0 = _3.fld0;
_6.0 = !_4;
_6.1 = _8.fld0 as i32;
RET = -18552_i16;
_6.1 = !_1;
_3.fld0 = _8.fld0;
_3.fld0 = _5;
_8 = Adt53 { fld0: _3.fld0 };
_8 = Adt53 { fld0: _3.fld0 };
_6 = (_4, _1, _8.fld0);
RET = !(-18148_i16);
_1 = !_6.1;
_5 = _6.2 == _6.2;
_9 = [36778_u16];
_3 = Adt53 { fld0: _6.2 };
_9 = [22487_u16];
_11 = RET;
_5 = _6.2 == _3.fld0;
_6.2 = !_3.fld0;
Goto(bb4)
}
bb4 = {
_13 = [(-102_i8),119_i8];
_6.0 = !_4;
_11 = RET << _6.1;
RET = _11 << _1;
_8 = _3;
_5 = _6.2;
_8.fld0 = _6.2;
RET = _11;
_9 = [31199_u16];
RET = _11;
_6 = (_4, _1, _5);
_17 = '\u{91782}';
_3 = _8;
_17 = '\u{e0fd7}';
_4 = !_6.0;
_13 = [112_i8,107_i8];
_17 = '\u{89ae6}';
_15 = _9;
Goto(bb5)
}
bb5 = {
_17 = '\u{d2096}';
_11 = RET;
_16 = [49_i8,(-126_i8)];
_6.1 = 100588063132155202304946115206949665184_i128 as i32;
_6.0 = _4 & _4;
RET = !_11;
_6 = (_4, _1, _8.fld0);
_11 = RET + RET;
_6.0 = _4 - _4;
_20 = _3.fld0 as isize;
RET = _11 << _20;
_5 = _6.2 > _3.fld0;
_18 = _20 ^ _20;
_4 = !_6.0;
_3.fld0 = _5;
_13 = _16;
_19 = 2238273410_u32 >> _20;
RET = -_11;
_12 = _6.2;
RET = _11 | _11;
_16 = [119_i8,125_i8];
_24 = _15;
Goto(bb6)
}
bb6 = {
Call(_28 = dump_var(19_usize, 5_usize, Move(_5), 13_usize, Move(_13), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_28 = dump_var(19_usize, 24_usize, Move(_24), 9_usize, Move(_9), 12_usize, Move(_12), 29_usize, _29), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(267171020054842468723881621579746655136_u128), std::hint::black_box('\u{8c979}'), std::hint::black_box(42187_u16), std::hint::black_box((-41_i8)), std::hint::black_box((-23705_i16)), std::hint::black_box(2626033230_u32), std::hint::black_box(2235052479094670389_i64), std::hint::black_box(23147497015775763150164778610578918_i128), std::hint::black_box(2016868944123175217_usize));
                
            }
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),
fld1: char,
fld2: *mut u32,
fld3: i8,
fld4: i16,
fld5: f32,
fld6: i64,

},
Variant1{
fld0: *mut *mut u128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *const [i8; 2],
fld1: *mut [i64; 7],
fld2: *mut u16,
fld3: (u64, i32, bool),
fld4: *const (u128, *mut char, *mut i64),
fld5: i32,
fld6: usize,

},
Variant1{
fld0: [bool; 8],
fld1: i64,
fld2: (*mut *mut char,),
fld3: u32,
fld4: *mut u32,

},
Variant2{
fld0: (u64, i32, bool),
fld1: *mut [i64; 7],
fld2: u8,

},
Variant3{
fld0: *mut u32,
fld1: char,
fld2: isize,
fld3: i8,
fld4: (*mut *mut char,),
fld5: i32,
fld6: usize,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i8; 2],

},
Variant1{
fld0: bool,
fld1: *const isize,
fld2: Adt43,
fld3: [i64; 7],
fld4: f64,
fld5: *mut u128,
fld6: i64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut i64,
fld1: Adt43,

},
Variant1{
fld0: f32,
fld1: *mut u16,
fld2: [u8; 5],
fld3: *mut i64,
fld4: i16,
fld5: i32,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [u8; 5],
fld1: *mut *mut u128,
fld2: (u64, i32, bool),
fld3: *mut i64,
fld4: *mut *mut char,
fld5: u32,

},
Variant1{
fld0: *const isize,
fld1: char,
fld2: (u64, i32, bool),
fld3: Adt45,
fld4: [u8; 5],
fld5: i32,
fld6: *mut u128,
fld7: *mut u16,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: isize,
fld1: u64,

},
Variant1{
fld0: u32,

},
Variant2{
fld0: u128,
fld1: u16,

},
Variant3{
fld0: u64,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u8,
fld1: *mut [i64; 7],
fld2: Adt42,
fld3: u128,
fld4: *const f64,
fld5: (u128, *mut char, *mut i64),
fld6: *const [i8; 2],

},
Variant1{
fld0: i64,
fld1: char,
fld2: u128,
fld3: u32,
fld4: i16,
fld5: Adt43,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: i8,
fld1: char,
fld2: *mut u64,

},
Variant1{
fld0: *const (u128, *mut char, *mut i64),
fld1: *mut u128,
fld2: *const [i8; 2],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut u64,

},
Variant1{
fld0: *const (u128, *mut char, *mut i64),
fld1: (*const (u128, *mut char, *mut i64), [i8; 2], *const [i8; 2], *const (u128, *mut char, *mut i64), bool),
fld2: u64,
fld3: (u128, *mut char, *mut i64),
fld4: i16,
fld5: i32,
fld6: i64,
fld7: Adt47,

},
Variant2{
fld0: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),
fld1: *const isize,
fld2: isize,
fld3: i64,
fld4: (*mut *mut char,),
fld5: *const (u128, *mut char, *mut i64),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: *mut u32,
fld2: usize,
fld3: (*const (u128, *mut char, *mut i64), [i8; 2], *const [i8; 2], *const (u128, *mut char, *mut i64), bool),
fld4: *mut u128,
fld5: *const f64,

},
Variant1{
fld0: *mut i64,
fld1: u8,
fld2: [u16; 1],
fld3: *const isize,

},
Variant2{
fld0: f32,
fld1: *const (u128, *mut char, *mut i64),
fld2: isize,
fld3: i8,
fld4: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),
fld5: (u128, *mut char, *mut i64),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: bool,
fld1: u32,
fld2: [i8; 2],
fld3: *mut u32,

},
Variant1{
fld0: *mut *mut char,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: bool,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: u8,
fld1: u64,
fld2: (*const (u128, *mut char, *mut i64), [i8; 2], *const [i8; 2], *const (u128, *mut char, *mut i64), bool),
fld3: Adt47,
fld4: i16,
fld5: Adt43,
fld6: *const [i8; 2],

},
Variant1{
fld0: *mut u16,
fld1: u128,
fld2: [bool; 8],
fld3: *const f64,
fld4: [i8; 2],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: bool,
fld1: u8,
fld2: *mut i64,
fld3: (u64, i32, bool),
fld4: Adt53,

},
Variant1{
fld0: *const (u128, *mut char, *mut i64),
fld1: (u64, i32, bool),
fld2: isize,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: bool,
fld1: char,
fld2: [u8; 5],
fld3: (u64, i32, bool),
fld4: [i64; 7],
fld5: u8,
fld6: Adt44,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: (u128, *mut char, *mut i64),
fld1: (*mut *mut char,),
fld2: (u64, i32, bool),
fld3: Adt43,
fld4: [i8; 2],
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: i32,

},
Variant1{
fld0: Adt50,
fld1: *mut u16,
fld2: Adt55,
fld3: (*const (u128, *mut char, *mut i64), isize, [i8; 2], (u128, *mut char, *mut i64), i8),
fld4: f32,
fld5: [bool; 8],

},
Variant2{
fld0: (*const (u128, *mut char, *mut i64), [i8; 2], *const [i8; 2], *const (u128, *mut char, *mut i64), bool),
fld1: Adt45,
fld2: Adt48,

},
Variant3{
fld0: Adt50,
fld1: Adt45,
fld2: (*mut *mut char,),
fld3: (u64, i32, bool),
fld4: Adt54,
fld5: [bool; 8],
fld6: u64,
fld7: i128,

}}

