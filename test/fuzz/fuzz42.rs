#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics)]
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
            printf(c"%i".as_ptr(),*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf(c"[".as_ptr());
            for b in self{
                b.printf_debug();
                printf(c",".as_ptr());
            }
            printf(c"]".as_ptr());
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf(c"%lx%lx".as_ptr(), (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self as usize);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf(c"true".as_ptr());
            }
            else{
                printf(c"false".as_ptr());
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf(c"()".as_ptr());
        }
    } 
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",)".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c",".as_ptr());
            self.11.printf_debug();
            printf(c")".as_ptr());
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
            printf(c"fn%u:_%u = ".as_ptr(),f,var0);
            val0.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var1);
            val1.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var2);
            val2.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var3);
            val3.printf_debug();
            printf(c"\n".as_ptr());
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> char {
mir! {
type RET = char;
let _15: (i16,);
let _16: [isize; 5];
let _17: usize;
let _18: f64;
let _19: (i8,);
let _20: u8;
let _21: u16;
let _22: [i8; 7];
let _23: [i64; 2];
let _24: ([u8; 2],);
let _25: isize;
let _26: *const *mut i16;
let _27: char;
let _28: i8;
let _29: Adt50;
let _30: [i16; 4];
let _31: [i64; 4];
let _32: [char; 7];
let _33: Adt42;
let _34: u16;
let _35: [char; 7];
let _36: f64;
let _37: [isize; 5];
let _38: ();
let _39: ();
{
RET = '\u{1822d}';
_2 = RET;
_9 = 2365028114_u32 as usize;
_15.0 = RET as i16;
_8 = 20235_u16 as i128;
RET = _2;
_6 = (-1188080860_i32) & 1571059134_i32;
_17 = _15.0 as usize;
_3 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_13 = 2343375937927834608_u64;
_4 = (-83_i8) | (-37_i8);
_18 = _3 as f64;
_19.0 = _4;
_14 = 315446560541821683947713931113361233450_u128 << _8;
_15.0 = 28991_i16;
_18 = 56447_u16 as f64;
_12 = !188633307_u32;
_10 = _13 as u8;
match _13 {
0 => bb1,
1 => bb2,
2343375937927834608 => bb4,
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
RET = _2;
_15.0 = (-3456_i16);
RET = _2;
_22 = [_19.0,_4,_19.0,_4,_4,_19.0,_19.0];
_8 = (-29512883914758566842551033121038333167_i128);
RET = _2;
_24.0 = [_10,_10];
_13 = _6 as u64;
_24.0 = [_10,_10];
_9 = !_17;
_1 = !false;
match _8 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
310769483006179896620823574310729878289 => bb10,
_ => bb9
}
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
_14 = 82322343564886556421875056868272663068_u128;
_21 = 26348_u16 * 15117_u16;
_15.0 = _21 as i16;
_9 = !_17;
_19.0 = _12 as i8;
_20 = _10 >> _13;
_25 = !_3;
_14 = !140040906545111643667498297190311265883_u128;
_23 = [(-5919813704280724282_i64),(-5183484064858204655_i64)];
_5 = _25 as i16;
_17 = _3 as usize;
_8 = (-6986883813094693837865633844955603095_i128) >> _13;
RET = _2;
RET = _2;
_3 = _25 * _25;
_18 = _25 as f64;
_6 = 5838399106814939152_i64 as i32;
_7 = -2238115030000218307_i64;
_5 = _15.0;
_16 = [_25,_25,_3,_3,_3];
_2 = RET;
_2 = RET;
_16 = [_25,_3,_25,_3,_3];
_16 = [_3,_3,_3,_3,_25];
_7 = _6 as i64;
_10 = _20;
_18 = _8 as f64;
_12 = !1893459769_u32;
_20 = _18 as u8;
Call(_11 = fn1(_21, _8, RET, _23, _5, _19.0, _24.0, _3, _10, _15.0, RET, _18, _8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = _2;
_7 = 3882511144158059224_i64 ^ 8145966681307826844_i64;
_15.0 = -_5;
_14 = _20 as u128;
_1 = !true;
_8 = !47188419237556591943787867198961788155_i128;
_4 = _19.0 * _19.0;
_1 = true;
_30 = [_5,_5,_15.0,_15.0];
RET = _2;
_30 = [_5,_5,_15.0,_15.0];
_34 = _11 | _11;
_27 = RET;
_13 = 2022260430636756536_u64 ^ 18124654407630616103_u64;
_12 = 2170316612_u32;
_6 = _13 as i32;
_10 = _20 & _20;
_14 = !218809554222385285930480706967660193345_u128;
RET = _27;
Goto(bb12)
}
bb12 = {
_28 = _17 as i8;
_32 = [RET,_27,_27,_2,_2,_27,_27];
_27 = _2;
_7 = 8643005719220213200_i64;
_25 = _3 | _3;
_27 = RET;
RET = _2;
Goto(bb13)
}
bb13 = {
_17 = _9 & _9;
_24.0 = [_20,_20];
_1 = !false;
_19.0 = _28;
_28 = _4;
_21 = _17 as u16;
_22 = [_28,_19.0,_28,_28,_19.0,_28,_4];
RET = _27;
_35 = [RET,RET,_27,_27,RET,_2,_2];
_36 = _18 + _18;
_11 = _25 as u16;
_13 = !7653074253106821704_u64;
_19 = (_28,);
_33.fld2 = -_25;
_34 = _11 & _21;
_33.fld1 = core::ptr::addr_of!(_17);
match _12 {
2170316612 => bb14,
_ => bb4
}
}
bb14 = {
_4 = _19.0 >> _33.fld2;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(0_usize, 16_usize, Move(_16), 13_usize, Move(_13), 21_usize, Move(_21), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(0_usize, 6_usize, Move(_6), 1_usize, Move(_1), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(0_usize, 12_usize, Move(_12), 7_usize, Move(_7), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(0_usize, 14_usize, Move(_14), 24_usize, Move(_24), 34_usize, Move(_34), 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u16,mut _2: i128,mut _3: char,mut _4: [i64; 2],mut _5: i16,mut _6: i8,mut _7: [u8; 2],mut _8: isize,mut _9: u8,mut _10: i16,mut _11: char,mut _12: f64,mut _13: i128) -> u16 {
mir! {
type RET = u16;
let _14: [u8; 1];
let _15: f64;
let _16: bool;
let _17: [u8; 1];
let _18: [i16; 4];
let _19: bool;
let _20: (i8,);
let _21: (i8,);
let _22: isize;
let _23: [i64; 2];
let _24: Adt44;
let _25: Adt46;
let _26: u8;
let _27: i128;
let _28: Adt47;
let _29: i32;
let _30: bool;
let _31: f32;
let _32: [char; 7];
let _33: [u8; 2];
let _34: Adt55;
let _35: Adt48;
let _36: bool;
let _37: i8;
let _38: ();
let _39: ();
{
_3 = _11;
_5 = _10 & _10;
_7 = [_9,_9];
_6 = !(-22_i8);
_5 = _10 >> _13;
RET = _1;
_2 = _13;
_11 = _3;
_1 = 3290114558_u32 as u16;
_6 = 95_i8;
_3 = _11;
_9 = 119_u8;
_7 = [_9,_9];
_9 = 233_u8 * 52_u8;
RET = _1 & _1;
_5 = !_10;
_10 = _5;
_14 = [_9];
_6 = (-108_i8);
_5 = -_10;
_14 = [_9];
_11 = _3;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211348 => bb7,
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
_5 = !_10;
_13 = _2;
RET = _3 as u16;
_8 = 9223372036854775807_isize;
_17 = [_9];
_7 = [_9,_9];
RET = _5 as u16;
_15 = -_12;
_14 = [_9];
_17 = [_9];
_11 = _3;
_17 = _14;
_16 = true & true;
_6 = (-89_i8) * 44_i8;
_9 = 1729635575_i32 as u8;
_3 = _11;
_12 = _15;
_12 = _15;
_15 = _12;
_2 = 1249757625_u32 as i128;
Call(_6 = fn2(_8, _17, _15, _11, _4, _12, RET, _7, _5, _11, _15, _17, _8, _4, _3, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_8 = 53_isize << _10;
_12 = _15 + _15;
_7 = [_9,_9];
_10 = _5 >> _8;
RET = _1 ^ _1;
_19 = _16 & _16;
_4 = [(-2178179555265726198_i64),8939763908347714862_i64];
_3 = _11;
_6 = 918926156_u32 as i8;
_6 = -106_i8;
_20 = (_6,);
_17 = _14;
_19 = !_16;
RET = _16 as u16;
_1 = RET >> _10;
_14 = [_9];
Goto(bb9)
}
bb9 = {
RET = _10 as u16;
_5 = 1593663308_u32 as i16;
RET = _10 as u16;
_17 = [_9];
Goto(bb10)
}
bb10 = {
_8 = (-30_isize) << _1;
_17 = _14;
_11 = _3;
_5 = _10 * _10;
_10 = _2 as i16;
RET = _1;
_21 = _20;
_3 = _11;
_3 = _11;
_21.0 = _6 & _20.0;
_20 = (_21.0,);
Goto(bb11)
}
bb11 = {
_13 = _16 as i128;
_17 = [_9];
_9 = !126_u8;
_7 = [_9,_9];
_20.0 = _21.0;
_19 = _16;
_2 = _13 | _13;
_14 = [_9];
Call(_9 = core::intrinsics::transmute(_6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6 = 318941234524319801_i64 as i8;
_16 = !_19;
_20 = (_21.0,);
_20 = (_21.0,);
_13 = _2;
_18 = [_5,_5,_5,_5];
_26 = _9 * _9;
_19 = _26 == _26;
_1 = !RET;
RET = _1;
Goto(bb13)
}
bb13 = {
_2 = !_13;
_4 = [6501259957847927126_i64,8045839852403213638_i64];
_3 = _11;
_3 = _11;
_29 = (-1123737436_i32) * 1933318662_i32;
_7 = [_9,_9];
_29 = -1131887247_i32;
_23 = [(-1829298456399240668_i64),(-2537607291219812153_i64)];
_22 = _8 >> _9;
_21 = (_6,);
_33 = [_9,_26];
_2 = _3 as i128;
_17 = [_9];
_14 = [_26];
_6 = -_20.0;
_9 = _26 << _13;
Call(_33 = core::intrinsics::transmute(RET), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10 = _5 | _5;
_24 = Adt44::Variant0 { fld0: 0_usize };
_31 = _29 as f32;
_26 = !_9;
_22 = _8 ^ _8;
_37 = _21.0;
_18 = [_10,_10,_5,_5];
_6 = _37 + _20.0;
_4 = [1759606569225655192_i64,1065876252570544193_i64];
_2 = _13;
_33 = _7;
_30 = _16;
_30 = _19;
_32 = [_11,_11,_11,_11,_3,_3,_3];
_8 = _22 * _22;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(1_usize, 30_usize, Move(_30), 6_usize, Move(_6), 26_usize, Move(_26), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(1_usize, 4_usize, Move(_4), 33_usize, Move(_33), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(1_usize, 20_usize, Move(_20), 2_usize, Move(_2), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(1_usize, 8_usize, Move(_8), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: [u8; 1],mut _3: f64,mut _4: char,mut _5: [i64; 2],mut _6: f64,mut _7: u16,mut _8: [u8; 2],mut _9: i16,mut _10: char,mut _11: f64,mut _12: [u8; 1],mut _13: isize,mut _14: [i64; 2],mut _15: char,mut _16: bool) -> i8 {
mir! {
type RET = i8;
let _17: u8;
let _18: i8;
let _19: isize;
let _20: [char; 7];
let _21: usize;
let _22: *const u128;
let _23: bool;
let _24: *const usize;
let _25: char;
let _26: [char; 7];
let _27: isize;
let _28: i64;
let _29: [isize; 2];
let _30: i32;
let _31: *const i32;
let _32: i64;
let _33: [u8; 1];
let _34: u8;
let _35: Adt50;
let _36: Adt45;
let _37: u64;
let _38: ();
let _39: ();
{
_13 = -_1;
_3 = -_6;
_17 = _15 as u8;
_1 = !_13;
_3 = -_11;
RET = 112_i8;
_18 = -RET;
_6 = _13 as f64;
_2 = _12;
_18 = (-4317544670685514836_i64) as i8;
_4 = _10;
_15 = _10;
Goto(bb1)
}
bb1 = {
_17 = 239_u8 - 77_u8;
_5 = [(-7996805845140818090_i64),(-7888733619735777792_i64)];
_7 = !19474_u16;
_2 = _12;
_21 = (-148535287043138829074000489323148685881_i128) as usize;
_4 = _15;
_2 = [_17];
_24 = core::ptr::addr_of!(_21);
_19 = _1;
_10 = _15;
_18 = RET;
_7 = 61145_u16 & 7209_u16;
_23 = !_16;
_17 = (*_24) as u8;
_16 = _23 >= _23;
_1 = !_19;
match _18 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
112 => bb7,
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
_19 = _1 * _13;
_12 = [_17];
_10 = _15;
RET = _19 as i8;
RET = _18 * _18;
_10 = _4;
RET = (*_24) as i8;
_2 = [_17];
_20 = [_4,_10,_15,_15,_15,_4,_10];
RET = -_18;
_26 = [_4,_4,_4,_15,_15,_4,_10];
_24 = core::ptr::addr_of!((*_24));
_6 = -_3;
RET = -_18;
_19 = _16 as isize;
(*_24) = 5_usize;
_8 = [_17,_17];
_16 = _23 | _23;
_5 = [3786217512499126615_i64,(-7105426750164765159_i64)];
_5 = _14;
_26[_21] = _20[_21];
_4 = _15;
(*_24) = 2_usize * 6_usize;
_1 = (-797047634_i32) as isize;
_17 = 165_u8 | 18_u8;
_21 = _3 as usize;
match _18 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
112 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_25 = _15;
_28 = (-81887317890012110894664198067032437106_i128) as i64;
_18 = !RET;
RET = -_18;
_3 = 257043147952917411785980607485315693975_u128 as f64;
_29 = [_19,_13];
_26 = _20;
_20 = [_25,_25,_25,_10,_15,_25,_25];
_6 = _17 as f64;
_4 = _25;
_31 = core::ptr::addr_of!(_30);
(*_31) = (-1622462403_i32) + 869522235_i32;
Call(_25 = fn3(_28, _19, _13, _10, _13, _31), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = _25;
_16 = _23;
Goto(bb11)
}
bb11 = {
_12 = [_17];
_6 = _28 as f64;
_18 = RET << (*_31);
_27 = _28 as isize;
_9 = 27913_i16 | 29060_i16;
_17 = !18_u8;
_20 = _26;
_17 = 58_u8;
_9 = -19506_i16;
_13 = _1;
_4 = _15;
_32 = -_28;
_4 = _25;
_26 = _20;
_20 = [_4,_25,_25,_15,_15,_15,_25];
_32 = -_28;
_19 = _27;
_3 = _11 + _11;
_7 = !21650_u16;
_8 = [_17,_17];
(*_31) = (-1862802184_i32) | (-95752198_i32);
_32 = _28 | _28;
_12 = [_17];
_14 = [_32,_28];
_31 = core::ptr::addr_of!(_30);
RET = 72976935985524637209735846440770399359_i128 as i8;
match _17 {
58 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_5 = _14;
_26 = _20;
_3 = -_6;
_25 = _10;
_31 = core::ptr::addr_of!((*_31));
_34 = _17 & _17;
_13 = _19 ^ _1;
(*_24) = 815734840804618434_usize;
Goto(bb14)
}
bb14 = {
_16 = _23 ^ _23;
_10 = _4;
_18 = RET - RET;
_7 = _11 as u16;
_3 = _11;
RET = _25 as i8;
_18 = -RET;
_27 = _1;
_11 = _3;
_9 = (-15043_i16);
_10 = _15;
_5 = [_32,_32];
_34 = !_17;
_11 = _3;
_30 = 33708515179144257866829107876606210221_i128 as i32;
_33 = _2;
_24 = core::ptr::addr_of!((*_24));
_9 = -12437_i16;
(*_31) = (-93466431_i32);
_1 = _13;
(*_31) = -(-818111550_i32);
_1 = _16 as isize;
_29 = [_13,_19];
_9 = !21145_i16;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(2_usize, 20_usize, Move(_20), 2_usize, Move(_2), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(2_usize, 21_usize, Move(_21), 29_usize, Move(_29), 34_usize, Move(_34), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(2_usize, 7_usize, Move(_7), 28_usize, Move(_28), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(2_usize, 26_usize, Move(_26), 4_usize, Move(_4), 39_usize, _39, 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: i64,mut _2: isize,mut _3: isize,mut _4: char,mut _5: isize,mut _6: *const i32) -> char {
mir! {
type RET = char;
let _7: *const u128;
let _8: bool;
let _9: (i16,);
let _10: ([u8; 2],);
let _11: bool;
let _12: bool;
let _13: [isize; 2];
let _14: Adt54;
let _15: char;
let _16: [u64; 7];
let _17: *const i32;
let _18: i128;
let _19: [i16; 4];
let _20: bool;
let _21: [u8; 2];
let _22: i32;
let _23: isize;
let _24: (i8,);
let _25: [i16; 4];
let _26: f32;
let _27: isize;
let _28: Adt44;
let _29: ();
let _30: ();
{
RET = _4;
_6 = core::ptr::addr_of!((*_6));
(*_6) = (-300585400_i32);
(*_6) = (-450588980_i32);
_1 = 171166830132957700128864934001697741628_u128 as i64;
_1 = -2258391324328647691_i64;
_1 = (-4157218056205766559_i64);
_6 = core::ptr::addr_of!((*_6));
(*_6) = _3 as i32;
_2 = 8639_i16 as isize;
RET = _4;
_5 = _3 >> (*_6);
RET = _4;
RET = _4;
_2 = _1 as isize;
_5 = _3;
(*_6) = (-1164257512_i32);
RET = _4;
(*_6) = (-656668888_i32);
Call(_3 = fn4((*_6), (*_6), (*_6)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = RET;
_3 = !_5;
_5 = (*_6) as isize;
RET = _4;
match (*_6) {
0 => bb2,
340282366920938463463374607431111542568 => bb4,
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
_8 = true;
_8 = _3 <= _5;
_1 = 31082_i16 as i64;
_6 = core::ptr::addr_of!((*_6));
_8 = false;
_8 = (*_6) > (*_6);
_11 = _8;
_12 = !_8;
_11 = _3 > _3;
_9 = (29452_i16,);
_12 = (*_6) == (*_6);
_9.0 = !4552_i16;
_2 = 16585541349760542968_u64 as isize;
_10.0 = [119_u8,118_u8];
_2 = _5 & _3;
_10.0 = [148_u8,218_u8];
_9.0 = _8 as i16;
_2 = _5;
_12 = !_8;
(*_6) = (-1864225702_i32);
_12 = _8;
Goto(bb5)
}
bb5 = {
_3 = _5 ^ _5;
_10.0 = [28_u8,83_u8];
_6 = core::ptr::addr_of!((*_6));
_12 = _8 | _11;
(*_6) = 252712171_i32 * 520332464_i32;
RET = _4;
_13 = [_3,_5];
_17 = _6;
_2 = _3;
(*_17) = 1889269134_i32;
Call(_10.0 = core::intrinsics::transmute(_9.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = !_2;
_12 = _8;
_5 = _3;
_3 = !_2;
(*_6) = !2041487644_i32;
_15 = _4;
_15 = RET;
RET = _4;
_4 = RET;
_16 = [5560431865460093457_u64,13513215724050097317_u64,16671786629480243740_u64,9186928104254768121_u64,6515599921179767623_u64,2767119956493061226_u64,11581357479330123638_u64];
_10.0 = [188_u8,1_u8];
_2 = 62077939337650972440493486220407739595_u128 as isize;
_17 = _6;
_10.0 = [113_u8,136_u8];
Goto(bb7)
}
bb7 = {
RET = _4;
_2 = 12093601822038106496_usize as isize;
_18 = 6111774271560308785411783860754049511_i128;
(*_17) = !1618246984_i32;
_8 = !_11;
Goto(bb8)
}
bb8 = {
(*_17) = _1 as i32;
_9.0 = (-9461_i16);
(*_6) = !1036919299_i32;
_11 = _8;
_4 = RET;
_12 = _11 | _11;
_12 = _8;
_22 = (*_17) - (*_17);
_20 = _11 ^ _11;
_11 = !_20;
_15 = RET;
_21 = [209_u8,27_u8];
match _18 {
0 => bb3,
6111774271560308785411783860754049511 => bb9,
_ => bb6
}
}
bb9 = {
_8 = _11;
RET = _15;
_21 = _10.0;
_16 = [14107965392689730385_u64,10450879955057460176_u64,16549742733031865770_u64,14506830841788545603_u64,3271212988927815230_u64,4084077190913622522_u64,8965680290017579458_u64];
_13 = [_3,_3];
_22 = -(*_6);
_5 = _3;
_3 = _2 - _2;
_18 = (-54197512718265835699892806508437844054_i128);
_10 = (_21,);
_12 = _11 | _20;
Goto(bb10)
}
bb10 = {
_23 = 4762570307344833997_u64 as isize;
_10.0 = [28_u8,149_u8];
_1 = 4045301044_u32 as i64;
(*_6) = _22 >> _3;
_6 = _17;
_3 = _8 as isize;
(*_6) = -_22;
(*_6) = _22 >> _5;
_26 = (*_6) as f32;
_24 = (27_i8,);
_15 = RET;
Goto(bb11)
}
bb11 = {
(*_6) = 2108807975_u32 as i32;
_9.0 = -26139_i16;
match _18 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb12,
286084854202672627763481800923330367402 => bb14,
_ => bb13
}
}
bb12 = {
_23 = 4762570307344833997_u64 as isize;
_10.0 = [28_u8,149_u8];
_1 = 4045301044_u32 as i64;
(*_6) = _22 >> _3;
_6 = _17;
_3 = _8 as isize;
(*_6) = -_22;
(*_6) = _22 >> _5;
_26 = (*_6) as f32;
_24 = (27_i8,);
_15 = RET;
Goto(bb11)
}
bb13 = {
_3 = _5 ^ _5;
_10.0 = [28_u8,83_u8];
_6 = core::ptr::addr_of!((*_6));
_12 = _8 | _11;
(*_6) = 252712171_i32 * 520332464_i32;
RET = _4;
_13 = [_3,_5];
_17 = _6;
_2 = _3;
(*_17) = 1889269134_i32;
Call(_10.0 = core::intrinsics::transmute(_9.0), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_26 = 126716182975018512370503657876393823767_u128 as f32;
(*_17) = 1753593458_u32 as i32;
_21 = [171_u8,61_u8];
_22 = (*_6);
_21 = _10.0;
_23 = !_3;
RET = _4;
_9 = ((-16086_i16),);
_8 = _20;
_11 = _20 & _20;
_27 = 42623_u16 as isize;
_27 = _3;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(3_usize, 18_usize, Move(_18), 21_usize, Move(_21), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(3_usize, 8_usize, Move(_8), 22_usize, Move(_22), 15_usize, Move(_15), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(3_usize, 24_usize, Move(_24), 4_usize, Move(_4), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i32,mut _2: i32,mut _3: i32) -> isize {
mir! {
type RET = isize;
let _4: [u8; 1];
let _5: [u128; 3];
let _6: bool;
let _7: u8;
let _8: [isize; 2];
let _9: &'static char;
let _10: (i16,);
let _11: (i8,);
let _12: isize;
let _13: ();
let _14: ();
{
_3 = _2 << _2;
RET = !(-34_isize);
_4 = [161_u8];
RET = 9223372036854775807_isize;
_2 = -_3;
_3 = _1;
RET = -(-9223372036854775808_isize);
_5 = [153074858272575948689669417077745737966_u128,278371087248400624745901702207589833317_u128,200283753656912638474275386759354557399_u128];
RET = -9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_2 = _1 * _1;
_5 = [87542926669375375959877832939457353365_u128,141816958164642398447054233991620227994_u128,320289438847457138403921617542627934424_u128];
_4 = [67_u8];
RET = 9063778668498481761_usize as isize;
RET = _3 as isize;
Goto(bb1)
}
bb1 = {
_1 = 4_usize as i32;
_6 = true;
RET = 9223372036854775807_isize;
_8 = [RET,RET];
_7 = !130_u8;
_4 = [_7];
RET = -(-107_isize);
_4 = [_7];
_1 = _2 & _3;
Goto(bb2)
}
bb2 = {
_6 = !false;
_4 = [_7];
_6 = false;
_2 = _3;
_4 = [_7];
_10.0 = -11945_i16;
_5 = [108569633644195271723585725839257561971_u128,82937257429037051532660403057580923499_u128,138176917845555108374757585843329670167_u128];
_2 = -_1;
_3 = _10.0 as i32;
_10 = ((-3484_i16),);
_6 = _1 >= _2;
_8 = [RET,RET];
_8 = [RET,RET];
_3 = _1;
_11 = (70_i8,);
_11.0 = (-2_i8) << _3;
_10 = ((-7044_i16),);
_10.0 = (-21621_i16) * (-29993_i16);
_4 = [_7];
Call(RET = fn5(_11.0, _6, _5, _3, _6, _6, _3, _1, _8, _11, _1, _3, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = !true;
Goto(bb4)
}
bb4 = {
Call(_13 = dump_var(4_usize, 6_usize, Move(_6), 5_usize, Move(_5), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_13 = dump_var(4_usize, 10_usize, Move(_10), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i8,mut _2: bool,mut _3: [u128; 3],mut _4: i32,mut _5: bool,mut _6: bool,mut _7: i32,mut _8: i32,mut _9: [isize; 2],mut _10: (i8,),mut _11: i32,mut _12: i32,mut _13: (i16,)) -> isize {
mir! {
type RET = isize;
let _14: i8;
let _15: *const [isize; 5];
let _16: f32;
let _17: u16;
let _18: *mut i16;
let _19: f32;
let _20: usize;
let _21: Adt54;
let _22: [u128; 3];
let _23: f32;
let _24: isize;
let _25: isize;
let _26: bool;
let _27: isize;
let _28: Adt42;
let _29: [u64; 7];
let _30: [u8; 2];
let _31: *const u128;
let _32: ();
let _33: ();
{
_13 = ((-20421_i16),);
_5 = _6;
_11 = -_4;
_7 = _11 << _12;
_9 = [(-9223372036854775808_isize),9223372036854775807_isize];
_14 = -_10.0;
RET = (-53_isize) * 9223372036854775807_isize;
RET = (-15_isize);
_16 = 1_usize as f32;
_3 = [190772968941223699772445855459414113624_u128,89676018376483731741153785988449099113_u128,91573224740443824502696194126563588439_u128];
_16 = 63600802574536328333920515987868451013_u128 as f32;
_17 = !41854_u16;
_14 = -_10.0;
_9 = [RET,RET];
_11 = _8;
_3 = [133707168059520900869911092677720251200_u128,32398950966220851336910300697461743724_u128,276106043857128353195947813748788185541_u128];
_7 = !_8;
_5 = _12 != _12;
_6 = !_2;
_16 = _8 as f32;
_10.0 = _1 - _14;
_14 = _16 as i8;
Call(_13 = fn6(_2, _1, _6, _12, _4, _14, _5, _10.0, _14, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _4 | _7;
_3 = [225147960821555984446801630520473501115_u128,8856897095050731791875030182879289249_u128,336109044715334598859053735986951344384_u128];
_8 = _13.0 as i32;
_8 = _4 & _7;
_5 = _2 & _6;
_5 = _6 > _6;
_14 = !_10.0;
_9 = [RET,RET];
RET = 115_isize >> _10.0;
_1 = _14 << RET;
_19 = _16 * _16;
_10.0 = _1 - _1;
_6 = _2;
_7 = -_4;
_14 = _10.0 | _10.0;
match _13.0 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768210208 => bb6,
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
_6 = !_5;
_10 = (_14,);
_9 = [RET,RET];
_5 = !_2;
_16 = _19;
_20 = !3668997388000265104_usize;
_7 = _8 + _11;
_6 = _8 >= _12;
_19 = 52188236611124732397041962680529632453_u128 as f32;
_5 = !_2;
_14 = _10.0;
_1 = _5 as i8;
_10 = (_14,);
_22 = [260747327180600982056210641633009042398_u128,238175340795441592122073266648630318973_u128,190028929154851705322948386495133562996_u128];
_23 = _16;
_17 = !1700_u16;
_20 = 3_usize;
_1 = 974739337_u32 as i8;
_11 = _4;
_9 = [RET,RET];
_8 = _4;
match _20 {
0 => bb1,
1 => bb2,
2 => bb3,
4 => bb5,
3 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_18 = core::ptr::addr_of_mut!(_13.0);
_18 = core::ptr::addr_of_mut!(_13.0);
_24 = 250_u8 as isize;
_18 = core::ptr::addr_of_mut!(_13.0);
_14 = (-76883102013177517754966871287123217273_i128) as i8;
_7 = _11;
_26 = _10.0 != _10.0;
_12 = 15910580548271027770_u64 as i32;
_28.fld2 = RET - RET;
_11 = !_8;
_1 = !_10.0;
_16 = _23;
_29[_20] = 17500652518152698813_u64;
_2 = _10.0 <= _1;
_13 = ((-2148_i16),);
(*_18) = !18563_i16;
_30 = [37_u8,190_u8];
_1 = _7 as i8;
_18 = core::ptr::addr_of_mut!((*_18));
RET = !_28.fld2;
_4 = _11 ^ _11;
_25 = !RET;
_29[_20] = 7643003071902719659_u64;
Goto(bb9)
}
bb9 = {
Call(_32 = dump_var(5_usize, 6_usize, Move(_6), 22_usize, Move(_22), 9_usize, Move(_9), 26_usize, Move(_26)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_32 = dump_var(5_usize, 5_usize, Move(_5), 20_usize, Move(_20), 24_usize, Move(_24), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_32 = dump_var(5_usize, 12_usize, Move(_12), 25_usize, Move(_25), 33_usize, _33, 33_usize, _33), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: i8,mut _3: bool,mut _4: i32,mut _5: i32,mut _6: i8,mut _7: bool,mut _8: i8,mut _9: i8,mut _10: [u128; 3]) -> (i16,) {
mir! {
type RET = (i16,);
let _11: u128;
let _12: [u128; 3];
let _13: Adt45;
let _14: i8;
let _15: [i64; 2];
let _16: [u64; 7];
let _17: Adt54;
let _18: i32;
let _19: bool;
let _20: u8;
let _21: [u128; 3];
let _22: f64;
let _23: isize;
let _24: isize;
let _25: Adt51;
let _26: Adt44;
let _27: f64;
let _28: [isize; 5];
let _29: [i8; 7];
let _30: [u128; 3];
let _31: f32;
let _32: *const u128;
let _33: *const [isize; 5];
let _34: f32;
let _35: usize;
let _36: ();
let _37: ();
{
RET = (4591_i16,);
_12 = [36476102159399813760536596936460127157_u128,280064221052085752041502135201349765601_u128,160016892659214542632368324096657418253_u128];
RET = (28611_i16,);
_2 = -_9;
_2 = !_8;
_11 = '\u{6fae1}' as u128;
_3 = _8 >= _6;
RET.0 = (-16924_i16);
_5 = _4 | _4;
_3 = _9 != _8;
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768194532 => bb8,
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
_5 = -_4;
_14 = _6 | _2;
Call(_4 = fn7(_14, _3, _8, _14, _7, _1, _2, _8, _8, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET.0 = (-23877_i16);
_8 = _9;
_6 = _2 - _2;
_14 = _6;
_15 = [(-7790743058647037559_i64),(-758243052848180731_i64)];
_7 = _14 < _6;
RET.0 = -(-30848_i16);
RET.0 = 1040004473102155714_i64 as i16;
_16 = [9258747228289750091_u64,17940939213199535160_u64,5113672249311803643_u64,18196005748143375372_u64,7394289125618082457_u64,7520264014595669634_u64,7492683558301871160_u64];
_8 = _2 ^ _14;
_15 = [(-370564458268449055_i64),(-694124423797107629_i64)];
_16 = [9844367633811872291_u64,5128242787350362217_u64,16631905177933051137_u64,16591880596903863987_u64,366658889626717818_u64,15067067435338785212_u64,15465432851200464756_u64];
_9 = _6 + _8;
_8 = _9 + _9;
_10 = [_11,_11,_11];
RET.0 = (-7080_i16);
_6 = -_9;
_12 = _10;
RET = (13262_i16,);
_16 = [8166295255929191045_u64,1547022034193122341_u64,13092278690229469170_u64,1710931221965086443_u64,3167073772715232448_u64,14753762799938373418_u64,845853940012397480_u64];
RET.0 = (-32499_i16);
_7 = _3 ^ _3;
_8 = 93_isize as i8;
_18 = 30440_u16 as i32;
RET = ((-22867_i16),);
Call(RET.0 = core::intrinsics::bswap((-6731_i16)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = (26886_i16,);
_8 = !_6;
_20 = 12626456549700108602_usize as u8;
_9 = _8 | _8;
_9 = _6 & _8;
_12 = [_11,_11,_11];
_8 = !_6;
_19 = _1 | _7;
_7 = _19;
Call(_18 = core::intrinsics::transmute(_4), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_5 = _4 + _18;
_4 = _5;
RET.0 = _11 as i16;
_1 = _7;
_23 = 9223372036854775807_isize;
_24 = _23;
_5 = _2 as i32;
_15 = [(-6914868030656326986_i64),279407420801814280_i64];
_2 = -_9;
_1 = _19 ^ _7;
_3 = !_1;
RET.0 = 7926_i16 + 24652_i16;
_22 = _11 as f64;
_10 = [_11,_11,_11];
_8 = !_9;
RET = ((-23918_i16),);
_24 = !_23;
_15 = [8430988645535106167_i64,(-2321470514776899630_i64)];
_3 = !_19;
Call(_14 = fn9(_2, _6, _8, _18, _8), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = 3755889309_u32 as i8;
RET.0 = 8794_i16;
_21 = _10;
_12 = [_11,_11,_11];
_18 = _4 ^ _4;
_4 = -_18;
_28 = [_23,_23,_24,_24,_23];
Call(_27 = fn10(_6, _1, _2, _18, _8, _7, _18, _8, _18, _12), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20 = 2_usize as u8;
_8 = 1136625908_u32 as i8;
_23 = _24;
_19 = _4 != _4;
_1 = _19;
_10 = [_11,_11,_11];
_7 = _19 | _19;
RET = (13964_i16,);
_16 = [10313863762004745213_u64,15597178514172299456_u64,17448469519385217866_u64,676557303553282610_u64,9774090137990140856_u64,6740871013820170077_u64,13495070741176501035_u64];
_7 = !_1;
Goto(bb14)
}
bb14 = {
_12 = _21;
RET = ((-18135_i16),);
_16 = [2133868569934257767_u64,14493555877658740192_u64,1194545590904543146_u64,5723775916510356535_u64,5999059096977183721_u64,297436298754305451_u64,587377222876884936_u64];
_29 = [_2,_2,_2,_14,_6,_14,_2];
_11 = 186577974685502732178387307318946137701_u128 * 154407875731449564921979259530862623027_u128;
_30 = [_11,_11,_11];
_19 = _7;
_8 = _6;
_3 = !_19;
_5 = RET.0 as i32;
_10 = _30;
_33 = core::ptr::addr_of!(_28);
RET.0 = (-1248_i16);
_10 = [_11,_11,_11];
_4 = -_18;
_24 = 110697784694302121522463691097496019826_i128 as isize;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(6_usize, 12_usize, Move(_12), 11_usize, Move(_11), 5_usize, Move(_5), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(6_usize, 4_usize, Move(_4), 21_usize, Move(_21), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(6_usize, 18_usize, Move(_18), 15_usize, Move(_15), 2_usize, Move(_2), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i8,mut _2: bool,mut _3: i8,mut _4: i8,mut _5: bool,mut _6: bool,mut _7: i8,mut _8: i8,mut _9: i8,mut _10: i8) -> i32 {
mir! {
type RET = i32;
let _11: ([u8; 2],);
let _12: f64;
let _13: Adt44;
let _14: [i64; 3];
let _15: [i64; 2];
let _16: i128;
let _17: ();
let _18: ();
{
_3 = _8 * _9;
_9 = _7;
_5 = !_6;
_2 = !_5;
Call(_10 = fn8(_1, _4, _1, _2, _1, _7, _8, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 362965634_i32;
_2 = !_5;
_4 = _3;
_11.0 = [143_u8,129_u8];
RET = 2916377608_u32 as i32;
_1 = _8;
_8 = !_1;
_11.0 = [105_u8,200_u8];
_4 = _3 + _1;
_7 = _10;
_5 = _6;
_12 = 8661613986559095240_i64 as f64;
_10 = _3;
_9 = _4;
RET = (-1857142418_i32);
_5 = !_6;
_4 = !_7;
RET = 366750543650377676_u64 as i32;
_8 = _3;
_2 = _4 <= _7;
_3 = _4 << _7;
_4 = _8;
RET = 939982745_i32 >> _10;
_4 = -_7;
_1 = -_4;
RET = (-118662111_i32) << _3;
Goto(bb2)
}
bb2 = {
Call(_17 = dump_var(7_usize, 1_usize, Move(_1), 9_usize, Move(_9), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_17 = dump_var(7_usize, 4_usize, Move(_4), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: bool,mut _5: i8,mut _6: i8,mut _7: i8,mut _8: i8) -> i8 {
mir! {
type RET = i8;
let _9: i128;
let _10: i128;
let _11: Adt50;
let _12: bool;
let _13: Adt43;
let _14: ();
let _15: ();
{
_3 = (-31823_i16) as i8;
_3 = (-9223372036854775808_isize) as i8;
RET = _1;
_4 = false & false;
RET = _2 << _2;
_5 = !RET;
_6 = _2 * _7;
_7 = RET | _1;
_4 = true;
_9 = 166075565824543378369746433568298632516_i128;
_4 = true | false;
_6 = 55_u8 as i8;
_7 = _2;
_10 = !_9;
RET = -_1;
RET = 36832_u16 as i8;
_10 = !_9;
_8 = _3;
_9 = 2094_i16 as i128;
Goto(bb1)
}
bb1 = {
_3 = -_5;
_3 = _5;
_3 = 18309971694407123020_u64 as i8;
RET = _5 & _5;
RET = _5;
_9 = _10;
_10 = (-29532_i16) as i128;
RET = _2;
_10 = RET as i128;
RET = !_1;
_10 = _9 - _9;
_5 = !_2;
_7 = 8289950496532934349_u64 as i8;
_8 = _1;
_9 = _10;
_5 = -_8;
_12 = _4;
_3 = _8 + _1;
RET = _5 ^ _3;
_5 = RET + RET;
_6 = _12 as i8;
_6 = _2 * RET;
_7 = _6;
_6 = (-6225_i16) as i8;
_10 = _9 + _9;
_3 = _7;
_6 = !_5;
_1 = !_6;
_4 = _12 & _12;
_6 = !_1;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(8_usize, 1_usize, Move(_1), 2_usize, Move(_2), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_14 = dump_var(8_usize, 8_usize, Move(_8), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: i32,mut _5: i8) -> i8 {
mir! {
type RET = i8;
let _6: ();
let _7: ();
{
_5 = _3;
Call(RET = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _3 - RET;
_4 = 761905668_i32 >> _1;
Goto(bb2)
}
bb2 = {
Call(_6 = dump_var(9_usize, 5_usize, Move(_5), 1_usize, Move(_1), 7_usize, _7, 7_usize, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i8,mut _2: bool,mut _3: i8,mut _4: i32,mut _5: i8,mut _6: bool,mut _7: i32,mut _8: i8,mut _9: i32,mut _10: [u128; 3]) -> f64 {
mir! {
type RET = f64;
let _11: Adt44;
let _12: f64;
let _13: [isize; 5];
let _14: [i64; 2];
let _15: u64;
let _16: Adt54;
let _17: [i64; 4];
let _18: Adt49;
let _19: i32;
let _20: f32;
let _21: (i8,);
let _22: u8;
let _23: f32;
let _24: [u8; 2];
let _25: Adt48;
let _26: [i64; 2];
let _27: [i64; 3];
let _28: i8;
let _29: char;
let _30: [isize; 5];
let _31: *const *mut i16;
let _32: [u8; 2];
let _33: [isize; 2];
let _34: u128;
let _35: f64;
let _36: f32;
let _37: Adt57;
let _38: f64;
let _39: (i8,);
let _40: [u8; 2];
let _41: [isize; 2];
let _42: Adt45;
let _43: i128;
let _44: [isize; 5];
let _45: ();
let _46: ();
{
RET = 27812_i16 as f64;
_12 = RET * RET;
_8 = _1;
_9 = 3464066936_u32 as i32;
_9 = _4 >> _5;
RET = _12;
RET = -_12;
_3 = _1;
RET = _12 * _12;
_7 = (-9223372036854775808_isize) as i32;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = -_4;
_5 = 8396742199205017498_i64 as i8;
_12 = -RET;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_9 = _4 * _4;
Goto(bb2)
}
bb2 = {
_15 = !17809606514661829718_u64;
_15 = _9 as u64;
_3 = _1 | _8;
RET = _12;
_14 = [7029986233045250356_i64,1790821888344271521_i64];
_5 = _1 * _1;
_9 = _5 as i32;
_10 = [295004943013105995033466819344207986603_u128,142544712417095996570245444633952490500_u128,326785432889467976878144886633107001707_u128];
_10 = [171925337197091123586410094919233627835_u128,280980893464763982010202893481074348985_u128,14084200200397715920926837940489834711_u128];
_1 = _3 ^ _5;
RET = -_12;
_17 = [7867459719147112667_i64,8903117184452023782_i64,7008908470168047300_i64,9149887282137742682_i64];
RET = -_12;
_21 = (_1,);
_8 = -_1;
_1 = _8 | _8;
_1 = !_21.0;
_3 = !_8;
_17 = [(-7793197087417849444_i64),(-5679417435209600691_i64),(-6444070230195860946_i64),(-6094792576421698741_i64)];
_21 = (_1,);
Call(_5 = fn11(_9, _15, _4, _14, _21.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = (-3146_i16) as f64;
_19 = _9 << _4;
_10 = [286387229231817419249896387944466955642_u128,327094647704082222274720305014868174409_u128,236675797188619392134336804658708722466_u128];
_20 = 142197327474090325666768467494897442650_i128 as f32;
_23 = 9209643500161808782_i64 as f32;
_17 = [(-3737482157326222392_i64),(-2592390246267230602_i64),(-7690179420159484272_i64),1350710700129802264_i64];
_10 = [288049484291975235253973333960520974891_u128,200467479475235903602015330170811446109_u128,252321942935695223652633550717549652420_u128];
Goto(bb4)
}
bb4 = {
_4 = _9 * _9;
_1 = _21.0 + _3;
Call(_17 = fn12(_21, _15, _4, _4, _5, _21.0, _19, _15, _3, _1, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_27 = [(-2559149151227466545_i64),(-8830348906638555238_i64),(-2402611591353429527_i64)];
_21.0 = !_5;
_3 = _21.0 & _1;
_17 = [3151460061353664806_i64,8673271089429619434_i64,(-6930801940267168228_i64),6001908387892666622_i64];
_18 = Adt49::Variant1 { fld0: _21 };
_6 = _3 < _3;
_13 = [42_isize,9223372036854775807_isize,9223372036854775807_isize,97_isize,9223372036854775807_isize];
_12 = _3 as f64;
_2 = _5 > _1;
_19 = _4 | _9;
_9 = _19 * _19;
_27 = [9147257138734545085_i64,(-1440928823964897363_i64),(-4287135477705548805_i64)];
_6 = _2 & _2;
place!(Field::<(i8,)>(Variant(_18, 1), 0)).0 = _21.0;
_22 = 157_u8;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,(-40_isize),92_isize,9223372036854775807_isize];
_27 = [1720109475542202369_i64,5186399949614157740_i64,3321557645001853578_i64];
_2 = _6 | _6;
_12 = RET;
_6 = !_2;
_28 = _21.0;
_4 = !_9;
match _22 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
157 => bb12,
_ => bb11
}
}
bb6 = {
_4 = _9 * _9;
_1 = _21.0 + _3;
Call(_17 = fn12(_21, _15, _4, _4, _5, _21.0, _19, _15, _3, _1, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_12 = (-3146_i16) as f64;
_19 = _9 << _4;
_10 = [286387229231817419249896387944466955642_u128,327094647704082222274720305014868174409_u128,236675797188619392134336804658708722466_u128];
_20 = 142197327474090325666768467494897442650_i128 as f32;
_23 = 9209643500161808782_i64 as f32;
_17 = [(-3737482157326222392_i64),(-2592390246267230602_i64),(-7690179420159484272_i64),1350710700129802264_i64];
_10 = [288049484291975235253973333960520974891_u128,200467479475235903602015330170811446109_u128,252321942935695223652633550717549652420_u128];
Goto(bb4)
}
bb8 = {
_15 = !17809606514661829718_u64;
_15 = _9 as u64;
_3 = _1 | _8;
RET = _12;
_14 = [7029986233045250356_i64,1790821888344271521_i64];
_5 = _1 * _1;
_9 = _5 as i32;
_10 = [295004943013105995033466819344207986603_u128,142544712417095996570245444633952490500_u128,326785432889467976878144886633107001707_u128];
_10 = [171925337197091123586410094919233627835_u128,280980893464763982010202893481074348985_u128,14084200200397715920926837940489834711_u128];
_1 = _3 ^ _5;
RET = -_12;
_17 = [7867459719147112667_i64,8903117184452023782_i64,7008908470168047300_i64,9149887282137742682_i64];
RET = -_12;
_21 = (_1,);
_8 = -_1;
_1 = _8 | _8;
_1 = !_21.0;
_3 = !_8;
_17 = [(-7793197087417849444_i64),(-5679417435209600691_i64),(-6444070230195860946_i64),(-6094792576421698741_i64)];
_21 = (_1,);
Call(_5 = fn11(_9, _15, _4, _14, _21.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_9 = -_4;
_5 = 8396742199205017498_i64 as i8;
_12 = -RET;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_9 = _4 * _4;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_28 = (-9223372036854775808_isize) as i8;
_14 = [4617768138404967201_i64,(-6334520155132024813_i64)];
_5 = _19 as i8;
_30 = _13;
place!(Field::<(i8,)>(Variant(_18, 1), 0)) = (_3,);
_23 = _15 as f32;
_1 = !_3;
_5 = _8 - _3;
_32 = [_22,_22];
_29 = '\u{541d1}';
_3 = !_5;
_33 = [9223372036854775807_isize,9223372036854775807_isize];
_22 = 142_u8 ^ 163_u8;
_2 = !_6;
place!(Field::<(i8,)>(Variant(_18, 1), 0)) = (_21.0,);
_6 = !_2;
_29 = '\u{cecc}';
_19 = _4 << _5;
_8 = _21.0;
place!(Field::<(i8,)>(Variant(_18, 1), 0)).0 = _5 & _21.0;
_29 = '\u{308aa}';
Call(_26 = fn14(_21.0, _2, _1, Move(_18), _23, RET, _9, _21.0, _15, _23), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_32 = [_22,_22];
_5 = !_8;
_17 = [(-7727874229161701100_i64),(-2269315922342331037_i64),(-6916333877012538229_i64),8296025486276123790_i64];
_14 = _26;
_5 = _1 ^ _8;
_10 = [67502329753603436760486310966790039266_u128,328497915682415055859541527143926961194_u128,188542224296812036359236439781284958826_u128];
_34 = 327500564904768388460076060118288998855_u128;
_35 = _12;
_36 = _23;
_25 = Adt48::Variant0 { fld0: _15 };
_26 = [(-331587981125501090_i64),(-6817725033494290959_i64)];
_25 = Adt48::Variant0 { fld0: _15 };
_21 = (_3,);
_1 = _3;
SetDiscriminant(_25, 0);
_24 = _32;
_2 = _6;
_39 = (_28,);
_38 = -RET;
_28 = -_5;
_8 = _1 << _1;
_22 = 3_usize as u8;
_21.0 = _5 ^ _8;
_12 = _28 as f64;
match _34 {
0 => bb3,
1 => bb9,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
327500564904768388460076060118288998855 => bb19,
_ => bb18
}
}
bb14 = {
_28 = (-9223372036854775808_isize) as i8;
_14 = [4617768138404967201_i64,(-6334520155132024813_i64)];
_5 = _19 as i8;
_30 = _13;
place!(Field::<(i8,)>(Variant(_18, 1), 0)) = (_3,);
_23 = _15 as f32;
_1 = !_3;
_5 = _8 - _3;
_32 = [_22,_22];
_29 = '\u{541d1}';
_3 = !_5;
_33 = [9223372036854775807_isize,9223372036854775807_isize];
_22 = 142_u8 ^ 163_u8;
_2 = !_6;
place!(Field::<(i8,)>(Variant(_18, 1), 0)) = (_21.0,);
_6 = !_2;
_29 = '\u{cecc}';
_19 = _4 << _5;
_8 = _21.0;
place!(Field::<(i8,)>(Variant(_18, 1), 0)).0 = _5 & _21.0;
_29 = '\u{308aa}';
Call(_26 = fn14(_21.0, _2, _1, Move(_18), _23, RET, _9, _21.0, _15, _23), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
_9 = -_4;
_5 = 8396742199205017498_i64 as i8;
_12 = -RET;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_9 = _4 * _4;
Goto(bb2)
}
bb16 = {
_15 = !17809606514661829718_u64;
_15 = _9 as u64;
_3 = _1 | _8;
RET = _12;
_14 = [7029986233045250356_i64,1790821888344271521_i64];
_5 = _1 * _1;
_9 = _5 as i32;
_10 = [295004943013105995033466819344207986603_u128,142544712417095996570245444633952490500_u128,326785432889467976878144886633107001707_u128];
_10 = [171925337197091123586410094919233627835_u128,280980893464763982010202893481074348985_u128,14084200200397715920926837940489834711_u128];
_1 = _3 ^ _5;
RET = -_12;
_17 = [7867459719147112667_i64,8903117184452023782_i64,7008908470168047300_i64,9149887282137742682_i64];
RET = -_12;
_21 = (_1,);
_8 = -_1;
_1 = _8 | _8;
_1 = !_21.0;
_3 = !_8;
_17 = [(-7793197087417849444_i64),(-5679417435209600691_i64),(-6444070230195860946_i64),(-6094792576421698741_i64)];
_21 = (_1,);
Call(_5 = fn11(_9, _15, _4, _14, _21.0), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_9 = -_4;
_5 = 8396742199205017498_i64 as i8;
_12 = -RET;
_13 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_9 = _4 * _4;
Goto(bb2)
}
bb18 = {
_15 = !17809606514661829718_u64;
_15 = _9 as u64;
_3 = _1 | _8;
RET = _12;
_14 = [7029986233045250356_i64,1790821888344271521_i64];
_5 = _1 * _1;
_9 = _5 as i32;
_10 = [295004943013105995033466819344207986603_u128,142544712417095996570245444633952490500_u128,326785432889467976878144886633107001707_u128];
_10 = [171925337197091123586410094919233627835_u128,280980893464763982010202893481074348985_u128,14084200200397715920926837940489834711_u128];
_1 = _3 ^ _5;
RET = -_12;
_17 = [7867459719147112667_i64,8903117184452023782_i64,7008908470168047300_i64,9149887282137742682_i64];
RET = -_12;
_21 = (_1,);
_8 = -_1;
_1 = _8 | _8;
_1 = !_21.0;
_3 = !_8;
_17 = [(-7793197087417849444_i64),(-5679417435209600691_i64),(-6444070230195860946_i64),(-6094792576421698741_i64)];
_21 = (_1,);
Call(_5 = fn11(_9, _15, _4, _14, _21.0), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_19 = _36 as i32;
_25 = Adt48::Variant0 { fld0: _15 };
SetDiscriminant(_25, 0);
_6 = _2 & _2;
_22 = 78_u8;
_40 = [_22,_22];
_14 = [3366640746625520882_i64,(-336934792662127447_i64)];
_21 = (_8,);
_23 = _19 as f32;
_34 = 255546476479962831442796322844732369674_u128 | 6881010062641098363777022347283433933_u128;
RET = _38 - _12;
_21.0 = _8 & _1;
_10 = [_34,_34,_34];
_9 = _4;
_40 = _24;
_2 = _6 & _6;
RET = _35 + _12;
_29 = '\u{f6195}';
_14 = _26;
_5 = 4221_u16 as i8;
Goto(bb20)
}
bb20 = {
Call(_45 = dump_var(10_usize, 29_usize, Move(_29), 28_usize, Move(_28), 40_usize, Move(_40), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_45 = dump_var(10_usize, 34_usize, Move(_34), 3_usize, Move(_3), 30_usize, Move(_30), 21_usize, Move(_21)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_45 = dump_var(10_usize, 19_usize, Move(_19), 24_usize, Move(_24), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_45 = dump_var(10_usize, 39_usize, Move(_39), 2_usize, Move(_2), 46_usize, _46, 46_usize, _46), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i32,mut _2: u64,mut _3: i32,mut _4: [i64; 2],mut _5: i8) -> i8 {
mir! {
type RET = i8;
let _6: [i64; 2];
let _7: ();
let _8: ();
{
_4 = [(-8726089146538846520_i64),(-5976128925072819655_i64)];
RET = -_5;
_3 = _1;
_2 = 7941119584000757187_u64;
_4 = [(-2944704425999667983_i64),3630250070326815958_i64];
_5 = !RET;
_4 = [5614490528058122131_i64,5997894579262109328_i64];
_1 = -_3;
RET = _5;
_4 = [(-524477260568766314_i64),3045966736843010891_i64];
_2 = 2045990636070392059_u64;
_5 = (-5215472168761179317_i64) as i8;
_2 = 13848698113312752586_u64 - 12271201596948357859_u64;
_4 = [(-1096475915837994872_i64),8652854802348330237_i64];
_6 = [(-5334530510681036515_i64),(-3463626090059643649_i64)];
RET = _5 >> _1;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(11_usize, 6_usize, Move(_6), 3_usize, Move(_3), 1_usize, Move(_1), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (i8,),mut _2: u64,mut _3: i32,mut _4: i32,mut _5: i8,mut _6: i8,mut _7: i32,mut _8: u64,mut _9: i8,mut _10: i8,mut _11: (i8,)) -> [i64; 4] {
mir! {
type RET = [i64; 4];
let _12: bool;
let _13: i32;
let _14: f32;
let _15: u32;
let _16: i8;
let _17: isize;
let _18: i8;
let _19: [u128; 3];
let _20: [u8; 1];
let _21: bool;
let _22: Adt47;
let _23: isize;
let _24: [u8; 1];
let _25: i128;
let _26: i16;
let _27: [i8; 7];
let _28: Adt53;
let _29: isize;
let _30: u8;
let _31: u128;
let _32: bool;
let _33: char;
let _34: Adt53;
let _35: Adt43;
let _36: [u64; 7];
let _37: u32;
let _38: u32;
let _39: isize;
let _40: [i8; 7];
let _41: Adt49;
let _42: *const u128;
let _43: [i64; 2];
let _44: ([u8; 2],);
let _45: [isize; 2];
let _46: u8;
let _47: isize;
let _48: [isize; 2];
let _49: Adt43;
let _50: isize;
let _51: ();
let _52: ();
{
RET = [2722283390330200580_i64,(-8452202250198121294_i64),(-1276628498658124459_i64),260848814734479902_i64];
_1.0 = (-107_isize) as i8;
_9 = !_11.0;
_8 = _2 + _2;
_11 = (_9,);
RET = [(-677145171630327337_i64),9124445582742038837_i64,74190495020382201_i64,8091695186339524332_i64];
_14 = 144174830640150278412807551342129032049_i128 as f32;
_5 = _11.0 | _9;
RET = [(-3103120281314918132_i64),(-7144034854668098575_i64),2091656552344698424_i64,6821567917638318954_i64];
Goto(bb1)
}
bb1 = {
_5 = _9 | _9;
_9 = '\u{5adb8}' as i8;
_10 = _11.0;
_11.0 = _10 + _6;
_16 = _11.0 << _10;
_7 = 233_u8 as i32;
_6 = _16 + _11.0;
_9 = _16 ^ _16;
_16 = !_5;
_12 = !true;
_17 = 87_isize;
_15 = 2270836656_u32 * 43342346_u32;
_12 = _9 >= _5;
_5 = _9 | _16;
_10 = _5;
_15 = 8974475770944411534_usize as u32;
_21 = _12;
_21 = _4 < _3;
_8 = _17 as u64;
Goto(bb2)
}
bb2 = {
_14 = 44872_u16 as f32;
_13 = -_4;
_9 = _6;
match _17 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
87 => bb8,
_ => bb7
}
}
bb3 = {
_5 = _9 | _9;
_9 = '\u{5adb8}' as i8;
_10 = _11.0;
_11.0 = _10 + _6;
_16 = _11.0 << _10;
_7 = 233_u8 as i32;
_6 = _16 + _11.0;
_9 = _16 ^ _16;
_16 = !_5;
_12 = !true;
_17 = 87_isize;
_15 = 2270836656_u32 * 43342346_u32;
_12 = _9 >= _5;
_5 = _9 | _16;
_10 = _5;
_15 = 8974475770944411534_usize as u32;
_21 = _12;
_21 = _4 < _3;
_8 = _17 as u64;
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
_17 = (-9223372036854775808_isize) << _5;
_11 = (_5,);
_2 = !_8;
_11 = (_16,);
_7 = _4;
_13 = -_7;
_10 = _9;
_3 = _4 >> _6;
_7 = _3;
_20 = [9_u8];
_19 = [305209057535849429053757982004935938988_u128,178125477462750700816791652222932589761_u128,339203157817644476398784458704427656706_u128];
_16 = _6 | _10;
_14 = _17 as f32;
_2 = !_8;
_1.0 = _9;
_9 = _5;
_18 = _6;
_20 = [128_u8];
_23 = _17;
_5 = _10 << _13;
_17 = _23 - _23;
Call(_6 = core::intrinsics::transmute(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11.0 = _8 as i8;
_16 = _10;
_26 = 21487_i16 >> _17;
_1.0 = !_16;
_18 = _9 + _9;
_11.0 = !_18;
_29 = _23 ^ _17;
_16 = _6;
_7 = _13 ^ _4;
_10 = _9 & _16;
_27 = [_6,_16,_6,_1.0,_18,_1.0,_11.0];
_2 = 137157949906716195878945651326469307930_u128 as u64;
_25 = _12 as i128;
_17 = _29;
_20 = [239_u8];
RET = [(-106118301219606286_i64),3128678403339946187_i64,(-6699815952021234693_i64),(-7420079504346456501_i64)];
_16 = '\u{e38cd}' as i8;
_19 = [248020928743998488351840513862263867752_u128,188777150243981446611898929450095026364_u128,280240319727948176432905810921154789060_u128];
_23 = _12 as isize;
Call(_32 = fn13(_23, _1.0, _7, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_12 = _21 & _21;
_4 = -_7;
_14 = 47_u8 as f32;
_18 = _3 as i8;
_6 = !_10;
_27 = [_11.0,_5,_11.0,_6,_9,_1.0,_5];
_24 = [68_u8];
_19 = [254971214291757853525794205340666138215_u128,326440980817637395975252897635398053472_u128,121579786182972441712331068769505010174_u128];
Call(_15 = core::intrinsics::bswap(861532490_u32), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = _9;
_31 = 90418433193744240575317790684111467900_u128;
_31 = _15 as u128;
RET = [(-3718081667289877497_i64),(-4847039832522659017_i64),5355407935487376097_i64,(-2224865261522302893_i64)];
_16 = !_6;
RET = [6406396641553149226_i64,(-1861081491641489197_i64),1773380575811289505_i64,258206649305133490_i64];
_30 = 240_u8;
_1.0 = 31974_u16 as i8;
_23 = -_17;
_1.0 = _29 as i8;
_11 = (_6,);
_15 = _25 as u32;
_20 = [_30];
_23 = -_29;
_8 = !_2;
_9 = _23 as i8;
_30 = _26 as u8;
_12 = _21;
_11.0 = _9;
_13 = _4 | _3;
_6 = -_5;
Goto(bb12)
}
bb12 = {
_11 = (_16,);
_29 = _23;
_19 = [_31,_31,_31];
_4 = _7;
_10 = _5 * _9;
_39 = _29;
_33 = '\u{ba24e}';
_33 = '\u{cf326}';
_10 = _1.0;
_33 = '\u{90f75}';
RET = [7935475851558018500_i64,2876351396479148870_i64,3149671476721968361_i64,8148443133007253663_i64];
_11 = _1;
_42 = core::ptr::addr_of!(_31);
_5 = _15 as i8;
_30 = _33 as u8;
_11.0 = _18 & _18;
_30 = _8 as u8;
_17 = !_23;
_3 = _7 - _4;
_40 = [_5,_16,_1.0,_16,_18,_1.0,_1.0];
_26 = 27993_i16;
_43 = [(-7116779888952437741_i64),8233127106381136639_i64];
_14 = _16 as f32;
_42 = core::ptr::addr_of!(_31);
match _26 {
0 => bb9,
1 => bb11,
2 => bb6,
27993 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_15 = 1175658884_u32 - 2084557830_u32;
_45 = [_29,_29];
_39 = _17;
_6 = -_9;
_16 = !_1.0;
_3 = _31 as i32;
_36 = [_2,_2,_8,_8,_8,_2,_8];
_6 = _18;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(12_usize, 23_usize, Move(_23), 13_usize, Move(_13), 10_usize, Move(_10), 45_usize, Move(_45)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(12_usize, 24_usize, Move(_24), 2_usize, Move(_2), 21_usize, Move(_21), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(12_usize, 1_usize, Move(_1), 19_usize, Move(_19), 36_usize, Move(_36), 32_usize, Move(_32)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(12_usize, 30_usize, Move(_30), 3_usize, Move(_3), 33_usize, Move(_33), 31_usize, Move(_31)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_51 = dump_var(12_usize, 12_usize, Move(_12), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: i8,mut _3: i32,mut _4: i16) -> bool {
mir! {
type RET = bool;
let _5: [isize; 2];
let _6: bool;
let _7: f32;
let _8: ();
let _9: ();
{
RET = !true;
_1 = 2266684875084237425_i64 as isize;
RET = true & true;
_3 = (-149518981_i32);
RET = true ^ false;
_2 = RET as i8;
_3 = !(-41982683_i32);
_3 = !(-879681782_i32);
_3 = -(-1364861714_i32);
_5 = [_1,_1];
RET = false;
_3 = !1345228572_i32;
_1 = (-9223372036854775808_isize);
_5 = [_1,_1];
RET = !false;
_5 = [_1,_1];
_3 = 3997328499_u32 as i32;
Goto(bb1)
}
bb1 = {
_2 = -(-99_i8);
_2 = RET as i8;
_5 = [_1,_1];
RET = _4 != _4;
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(13_usize, 5_usize, Move(_5), 4_usize, Move(_4), 9_usize, _9, 9_usize, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i8,mut _2: bool,mut _3: i8,mut _4: Adt49,mut _5: f32,mut _6: f64,mut _7: i32,mut _8: i8,mut _9: u64,mut _10: f32) -> [i64; 2] {
mir! {
type RET = [i64; 2];
let _11: f64;
let _12: i64;
let _13: [i64; 4];
let _14: (i16,);
let _15: u128;
let _16: [u8; 1];
let _17: u128;
let _18: [i16; 4];
let _19: f32;
let _20: f64;
let _21: f64;
let _22: Adt54;
let _23: u8;
let _24: ([u8; 2],);
let _25: isize;
let _26: f32;
let _27: char;
let _28: *const [isize; 5];
let _29: f32;
let _30: u32;
let _31: u64;
let _32: u128;
let _33: f32;
let _34: f32;
let _35: u16;
let _36: f32;
let _37: u64;
let _38: ([u8; 2],);
let _39: i128;
let _40: [i64; 3];
let _41: (i16,);
let _42: Adt58;
let _43: (i8,);
let _44: [i16; 4];
let _45: (i8,);
let _46: f64;
let _47: char;
let _48: i128;
let _49: usize;
let _50: ();
let _51: ();
{
_3 = -_1;
_9 = !14050221531566215760_u64;
_8 = !_3;
_7 = (-831972739_i32) * 424207015_i32;
_8 = -_1;
_3 = _1 | Field::<(i8,)>(Variant(_4, 1), 0).0;
_11 = _7 as f64;
_5 = -_10;
RET = [(-3299441210568539847_i64),(-6350297250110815984_i64)];
place!(Field::<(i8,)>(Variant(_4, 1), 0)) = (_1,);
RET = [8702661370144947146_i64,5058660749745235792_i64];
RET = [72972345859852621_i64,(-4510116982042094068_i64)];
Goto(bb1)
}
bb1 = {
RET = [(-1532111701450057883_i64),(-5621049642104275903_i64)];
_3 = -Field::<(i8,)>(Variant(_4, 1), 0).0;
_13 = [(-561178669958975841_i64),(-1067481123122256659_i64),5446474108371412167_i64,(-8670405422667106737_i64)];
_9 = !12572678295172501804_u64;
_12 = 1502676608883477375_i64 + (-6305174920805612470_i64);
_6 = _11 - _11;
_5 = _6 as f32;
_6 = (-9223372036854775808_isize) as f64;
SetDiscriminant(_4, 0);
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{c8e08}';
_15 = !175448476339974419142024701856514311304_u128;
_11 = _6 * _6;
_2 = false;
_10 = _12 as f32;
_2 = !false;
_6 = _11;
_15 = !287179639492642245367474594756078250243_u128;
_14 = ((-20944_i16),);
_15 = !89509969898910532574297151550248718481_u128;
_15 = 230383707465953702542751000635330052386_u128;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = 9223372036854775807_isize + 51_isize;
_12 = (-41138482671108997805853458761702271303_i128) as i64;
_14.0 = _10 as i16;
_3 = _8 >> _1;
_5 = _10 * _10;
Goto(bb2)
}
bb2 = {
_18 = [_14.0,_14.0,_14.0,_14.0];
_16 = [27_u8];
_17 = _15;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{c2aab}';
_19 = -_5;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{6ba50}';
Call(place!(Field::<usize>(Variant(_4, 0), 0)) = core::intrinsics::transmute(Field::<Adt42>(Variant(_4, 0), 4).fld2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [_12,_12];
RET = [_12,_12];
_11 = -_6;
_3 = _9 as i8;
_2 = true;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
place!(Field::<[u128; 3]>(Variant(_4, 0), 5)) = [_15,_15,_15];
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = 114_isize ^ (-6_isize);
_21 = -_6;
place!(Field::<[i16; 4]>(Variant(_4, 0), 3)) = _18;
_10 = _7 as f32;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{d505d}';
_23 = 91_u8 * 34_u8;
_5 = _10 * _19;
_2 = _8 <= _8;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = -77_isize;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_11 = -_21;
_11 = -_6;
_5 = _19;
RET = [_12,_12];
_20 = -_6;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{6603e}';
_18 = [_14.0,_14.0,_14.0,_14.0];
Goto(bb4)
}
bb4 = {
_25 = _14.0 as isize;
_12 = (-5067621142100521647_i64);
place!(Field::<[u128; 3]>(Variant(_4, 0), 5)) = [_15,_15,_15];
_27 = Field::<char>(Variant(_4, 0), 1);
_5 = _19 - _19;
_27 = Field::<char>(Variant(_4, 0), 1);
_20 = _21;
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld1 = Field::<*const usize>(Variant(_4, 0), 2);
_2 = false;
_23 = Field::<usize>(Variant(_4, 0), 0) as u8;
_16 = [_23];
_23 = !47_u8;
_16 = [_23];
_3 = -_1;
_25 = Field::<Adt42>(Variant(_4, 0), 4).fld2;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = Field::<Adt42>(Variant(_4, 0), 4).fld1;
_20 = _11;
_32 = _15 ^ _15;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = _25 | _25;
_24.0 = [_23,_23];
_13 = [_12,_12,_12,_12];
_8 = _12 as i8;
_5 = _19;
_10 = _12 as f32;
Goto(bb5)
}
bb5 = {
_3 = _1;
_17 = _32 - _15;
_7 = -905174914_i32;
_1 = Field::<usize>(Variant(_4, 0), 0) as i8;
_14 = (31250_i16,);
_32 = _17 | _15;
_3 = _2 as i8;
_27 = Field::<char>(Variant(_4, 0), 1);
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_7 = -(-430735288_i32);
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_9 = _14.0 as u64;
_10 = _9 as f32;
RET = [_12,_12];
_26 = _7 as f32;
_7 = 1777979837_i32 + (-1086165411_i32);
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
_15 = _32 * _17;
_24.0 = [_23,_23];
Call(_35 = fn15(_32, Field::<Adt42>(Variant(_4, 0), 4).fld2, _1, Field::<Adt42>(Variant(_4, 0), 4).fld2, Field::<*const usize>(Variant(_4, 0), 2)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_37 = _9;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld1 = Field::<*const usize>(Variant(_4, 0), 2);
_33 = _5;
_12 = (-3049694712333866857_i64);
_9 = _37;
_38 = _24;
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
_18 = [_14.0,_14.0,_14.0,_14.0];
match _12 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463460324912719434344599 => bb12,
_ => bb11
}
}
bb7 = {
_3 = _1;
_17 = _32 - _15;
_7 = -905174914_i32;
_1 = Field::<usize>(Variant(_4, 0), 0) as i8;
_14 = (31250_i16,);
_32 = _17 | _15;
_3 = _2 as i8;
_27 = Field::<char>(Variant(_4, 0), 1);
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_7 = -(-430735288_i32);
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_9 = _14.0 as u64;
_10 = _9 as f32;
RET = [_12,_12];
_26 = _7 as f32;
_7 = 1777979837_i32 + (-1086165411_i32);
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
_15 = _32 * _17;
_24.0 = [_23,_23];
Call(_35 = fn15(_32, Field::<Adt42>(Variant(_4, 0), 4).fld2, _1, Field::<Adt42>(Variant(_4, 0), 4).fld2, Field::<*const usize>(Variant(_4, 0), 2)), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_25 = _14.0 as isize;
_12 = (-5067621142100521647_i64);
place!(Field::<[u128; 3]>(Variant(_4, 0), 5)) = [_15,_15,_15];
_27 = Field::<char>(Variant(_4, 0), 1);
_5 = _19 - _19;
_27 = Field::<char>(Variant(_4, 0), 1);
_20 = _21;
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld1 = Field::<*const usize>(Variant(_4, 0), 2);
_2 = false;
_23 = Field::<usize>(Variant(_4, 0), 0) as u8;
_16 = [_23];
_23 = !47_u8;
_16 = [_23];
_3 = -_1;
_25 = Field::<Adt42>(Variant(_4, 0), 4).fld2;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = Field::<Adt42>(Variant(_4, 0), 4).fld1;
_20 = _11;
_32 = _15 ^ _15;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = _25 | _25;
_24.0 = [_23,_23];
_13 = [_12,_12,_12,_12];
_8 = _12 as i8;
_5 = _19;
_10 = _12 as f32;
Goto(bb5)
}
bb9 = {
RET = [_12,_12];
RET = [_12,_12];
_11 = -_6;
_3 = _9 as i8;
_2 = true;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
place!(Field::<[u128; 3]>(Variant(_4, 0), 5)) = [_15,_15,_15];
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = 114_isize ^ (-6_isize);
_21 = -_6;
place!(Field::<[i16; 4]>(Variant(_4, 0), 3)) = _18;
_10 = _7 as f32;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{d505d}';
_23 = 91_u8 * 34_u8;
_5 = _10 * _19;
_2 = _8 <= _8;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = -77_isize;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_11 = -_21;
_11 = -_6;
_5 = _19;
RET = [_12,_12];
_20 = -_6;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{6603e}';
_18 = [_14.0,_14.0,_14.0,_14.0];
Goto(bb4)
}
bb10 = {
_18 = [_14.0,_14.0,_14.0,_14.0];
_16 = [27_u8];
_17 = _15;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{c2aab}';
_19 = -_5;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{6ba50}';
Call(place!(Field::<usize>(Variant(_4, 0), 0)) = core::intrinsics::transmute(Field::<Adt42>(Variant(_4, 0), 4).fld2), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
RET = [(-1532111701450057883_i64),(-5621049642104275903_i64)];
_3 = -Field::<(i8,)>(Variant(_4, 1), 0).0;
_13 = [(-561178669958975841_i64),(-1067481123122256659_i64),5446474108371412167_i64,(-8670405422667106737_i64)];
_9 = !12572678295172501804_u64;
_12 = 1502676608883477375_i64 + (-6305174920805612470_i64);
_6 = _11 - _11;
_5 = _6 as f32;
_6 = (-9223372036854775808_isize) as f64;
SetDiscriminant(_4, 0);
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{c8e08}';
_15 = !175448476339974419142024701856514311304_u128;
_11 = _6 * _6;
_2 = false;
_10 = _12 as f32;
_2 = !false;
_6 = _11;
_15 = !287179639492642245367474594756078250243_u128;
_14 = ((-20944_i16),);
_15 = !89509969898910532574297151550248718481_u128;
_15 = 230383707465953702542751000635330052386_u128;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = 9223372036854775807_isize + 51_isize;
_12 = (-41138482671108997805853458761702271303_i128) as i64;
_14.0 = _10 as i16;
_3 = _8 >> _1;
_5 = _10 * _10;
Goto(bb2)
}
bb12 = {
_6 = _21;
_31 = _37;
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
_34 = _23 as f32;
_36 = _5 * _19;
_24 = (_38.0,);
_3 = !_8;
match _12 {
0 => bb9,
340282366920938463460324912719434344599 => bb13,
_ => bb5
}
}
bb13 = {
_20 = _1 as f64;
_24.0 = [_23,_23];
_29 = _33 + _36;
_14.0 = (-27315_i16) & 13939_i16;
_38.0 = [_23,_23];
_2 = !true;
_20 = _35 as f64;
_42.fld0 = core::ptr::addr_of_mut!(_14.0);
_7 = (-2925975_i32);
_14 = ((-9676_i16),);
RET = [_12,_12];
_32 = _1 as u128;
place!(Field::<[i16; 4]>(Variant(_4, 0), 3)) = _18;
_17 = !_32;
_27 = Field::<char>(Variant(_4, 0), 1);
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = _25;
_32 = !_15;
_7 = 168941261711779285594095430752100178765_i128 as i32;
match _14.0 {
0 => bb1,
1 => bb2,
2 => bb12,
3 => bb4,
4 => bb11,
5 => bb14,
340282366920938463463374607431768201780 => bb16,
_ => bb15
}
}
bb14 = {
RET = [_12,_12];
RET = [_12,_12];
_11 = -_6;
_3 = _9 as i8;
_2 = true;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
place!(Field::<[u128; 3]>(Variant(_4, 0), 5)) = [_15,_15,_15];
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = 114_isize ^ (-6_isize);
_21 = -_6;
place!(Field::<[i16; 4]>(Variant(_4, 0), 3)) = _18;
_10 = _7 as f32;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{d505d}';
_23 = 91_u8 * 34_u8;
_5 = _10 * _19;
_2 = _8 <= _8;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = -77_isize;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
place!(Field::<*const usize>(Variant(_4, 0), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4, 0), 0)));
_11 = -_21;
_11 = -_6;
_5 = _19;
RET = [_12,_12];
_20 = -_6;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{6603e}';
_18 = [_14.0,_14.0,_14.0,_14.0];
Goto(bb4)
}
bb15 = {
_25 = _14.0 as isize;
_12 = (-5067621142100521647_i64);
place!(Field::<[u128; 3]>(Variant(_4, 0), 5)) = [_15,_15,_15];
_27 = Field::<char>(Variant(_4, 0), 1);
_5 = _19 - _19;
_27 = Field::<char>(Variant(_4, 0), 1);
_20 = _21;
place!(Field::<char>(Variant(_4, 0), 1)) = _27;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld1 = Field::<*const usize>(Variant(_4, 0), 2);
_2 = false;
_23 = Field::<usize>(Variant(_4, 0), 0) as u8;
_16 = [_23];
_23 = !47_u8;
_16 = [_23];
_3 = -_1;
_25 = Field::<Adt42>(Variant(_4, 0), 4).fld2;
place!(Field::<*const usize>(Variant(_4, 0), 2)) = Field::<Adt42>(Variant(_4, 0), 4).fld1;
_20 = _11;
_32 = _15 ^ _15;
place!(Field::<Adt42>(Variant(_4, 0), 4)).fld2 = _25 | _25;
_24.0 = [_23,_23];
_13 = [_12,_12,_12,_12];
_8 = _12 as i8;
_5 = _19;
_10 = _12 as f32;
Goto(bb5)
}
bb16 = {
place!(Field::<usize>(Variant(_4, 0), 0)) = _15 as usize;
_14 = (21031_i16,);
_41.0 = _14.0 >> _32;
_16 = [_23];
_44 = _18;
_31 = _2 as u64;
_43 = (_8,);
_35 = !48822_u16;
_14 = _41;
Goto(bb17)
}
bb17 = {
Call(_50 = dump_var(14_usize, 15_usize, Move(_15), 13_usize, Move(_13), 25_usize, Move(_25), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(14_usize, 41_usize, Move(_41), 31_usize, Move(_31), 9_usize, Move(_9), 35_usize, Move(_35)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(14_usize, 37_usize, Move(_37), 18_usize, Move(_18), 38_usize, Move(_38), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u128,mut _2: isize,mut _3: i8,mut _4: isize,mut _5: *const usize) -> u16 {
mir! {
type RET = u16;
let _6: *const [isize; 5];
let _7: u64;
let _8: [i8; 7];
let _9: f64;
let _10: [i16; 4];
let _11: Adt51;
let _12: i8;
let _13: [i64; 4];
let _14: u128;
let _15: isize;
let _16: f32;
let _17: isize;
let _18: i8;
let _19: i32;
let _20: ();
let _21: ();
{
RET = 13138_u16;
RET = 7918_u16 | 63702_u16;
RET = 33384_u16 - 37386_u16;
_3 = false as i8;
_5 = core::ptr::addr_of!((*_5));
RET = (-127283620585414209206026581306265397233_i128) as u16;
(*_5) = 1436413301_u32 as usize;
_3 = 150_u8 as i8;
_1 = !318103100660437690536831188235884833909_u128;
_4 = _2 * _2;
Goto(bb1)
}
bb1 = {
(*_5) = 7648557601254817011_usize << _4;
_7 = !12830469531343963039_u64;
_4 = _2 ^ _2;
_5 = core::ptr::addr_of!((*_5));
_8 = [_3,_3,_3,_3,_3,_3,_3];
(*_5) = !0_usize;
RET = 5669_u16 | 49066_u16;
(*_5) = 15528728388553691798_usize;
_7 = !17290798324747570957_u64;
_2 = RET as isize;
RET = !12537_u16;
_1 = 123409135005437390183303644867511806771_u128 & 306485232673342561797003676486946445530_u128;
_1 = !166853506532729253686385743786709225499_u128;
(*_5) = 14126314157973824611_usize;
RET = !33875_u16;
_5 = core::ptr::addr_of!((*_5));
(*_5) = 17214208396991926245_usize;
_7 = 17406472475490304640_u64 * 3974903206718048952_u64;
_7 = !3123720925645108017_u64;
_7 = 77_u8 as u64;
_1 = 58019277784761130515208366239814105100_u128;
_8 = [_3,_3,_3,_3,_3,_3,_3];
_2 = _4;
_7 = 922653568255199447_u64 & 8010257918435312526_u64;
_2 = _4;
_7 = '\u{6d936}' as u64;
match (*_5) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
17214208396991926245 => bb9,
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
RET = 18165_u16 << _1;
(*_5) = 5_usize;
_8 = [_3,_3,_3,_3,_3,_3,_3];
(*_5) = 4_usize >> _4;
_10 = [(-5563_i16),(-26448_i16),31488_i16,10120_i16];
_5 = core::ptr::addr_of!((*_5));
_7 = !17176405525909589865_u64;
_9 = _3 as f64;
(*_5) = !1_usize;
RET = 28452_u16;
_5 = core::ptr::addr_of!((*_5));
_1 = 159864145759811500720924696041522080139_u128;
_3 = _1 as i8;
(*_5) = 7_usize;
_4 = !_2;
_8 = [_3,_3,_3,_3,_3,_3,_3];
_5 = core::ptr::addr_of!((*_5));
Call(_8 = fn16(_4, (*_5), _7, _5, _2, _2, _4, _7, RET, _4, (*_5), (*_5), _1, _2, _4, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2 = _4;
_2 = _3 as isize;
_3 = -32_i8;
_9 = _3 as f64;
_7 = !16914446218748150336_u64;
_9 = _7 as f64;
_5 = core::ptr::addr_of!((*_5));
_1 = _4 as u128;
_12 = 23_u8 as i8;
_7 = 9520586583636296440_u64 * 15333205611150928738_u64;
_10 = [(-14615_i16),(-20410_i16),25934_i16,30968_i16];
_12 = !_3;
_12 = -_3;
_8 = [_12,_3,_3,_12,_3,_3,_12];
match (*_5) {
0 => bb7,
1 => bb6,
2 => bb3,
7 => bb11,
_ => bb4
}
}
bb11 = {
_9 = (-786003815_i32) as f64;
_10 = [(-28622_i16),(-25596_i16),24031_i16,31474_i16];
_5 = core::ptr::addr_of!((*_5));
_8 = [_3,_12,_12,_3,_12,_12,_3];
_7 = (*_5) as u64;
RET = (-1190883673724819592_i64) as u16;
_2 = !_4;
_3 = _12;
_7 = 17238800160356029778_u64;
_8 = [_3,_12,_12,_12,_12,_12,_12];
RET = 35146_u16;
_12 = _1 as i8;
_13 = [(-979441126757766559_i64),4598638954134576110_i64,(-3761622718271837483_i64),(-1923166767997932134_i64)];
_8 = [_12,_12,_12,_12,_12,_3,_12];
(*_5) = !14737635180607877042_usize;
match RET {
0 => bb10,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb6,
35146 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_7 = 7594924557971693863_u64 | 996879175432978681_u64;
_1 = 90464263533879896416342294511384510326_u128 ^ 311881878326798353873482978071333321239_u128;
(*_5) = 99747317480459251_usize;
RET = _9 as u16;
_1 = 202106934525536718121810281462630885970_u128 | 163460039266936392546600232401183013428_u128;
RET = _1 as u16;
_1 = !300628733162093643950695483011043252518_u128;
_9 = (-1958864324_i32) as f64;
_15 = _7 as isize;
_3 = _12;
_15 = _2 >> _4;
_16 = 1338317426_u32 as f32;
_15 = -_2;
_16 = _9 as f32;
match (*_5) {
0 => bb12,
1 => bb4,
2 => bb14,
3 => bb15,
4 => bb16,
99747317480459251 => bb18,
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
_2 = _4;
_2 = _3 as isize;
_3 = -32_i8;
_9 = _3 as f64;
_7 = !16914446218748150336_u64;
_9 = _7 as f64;
_5 = core::ptr::addr_of!((*_5));
_1 = _4 as u128;
_12 = 23_u8 as i8;
_7 = 9520586583636296440_u64 * 15333205611150928738_u64;
_10 = [(-14615_i16),(-20410_i16),25934_i16,30968_i16];
_12 = !_3;
_12 = -_3;
_8 = [_12,_3,_3,_12,_3,_3,_12];
match (*_5) {
0 => bb7,
1 => bb6,
2 => bb3,
7 => bb11,
_ => bb4
}
}
bb17 = {
Return()
}
bb18 = {
_15 = _2 ^ _2;
_10 = [(-30768_i16),(-5714_i16),(-19094_i16),2612_i16];
_16 = _1 as f32;
_13 = [(-1395950038169648945_i64),(-7817802240214787747_i64),(-8605314534171141082_i64),(-8652854416933807816_i64)];
RET = !57642_u16;
_16 = 57063620091301023814052011924860795921_i128 as f32;
(*_5) = 6304_i16 as usize;
_17 = _7 as isize;
_9 = (-12767_i16) as f64;
_15 = _4;
_10 = [9123_i16,2909_i16,22854_i16,(-28067_i16)];
_1 = false as u128;
_19 = 1158428994_i32;
_5 = core::ptr::addr_of!((*_5));
_16 = RET as f32;
_17 = -_2;
Goto(bb19)
}
bb19 = {
Call(_20 = dump_var(15_usize, 1_usize, Move(_1), 12_usize, Move(_12), 2_usize, Move(_2), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_20 = dump_var(15_usize, 17_usize, Move(_17), 7_usize, Move(_7), 21_usize, _21, 21_usize, _21), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: usize,mut _3: u64,mut _4: *const usize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: u64,mut _9: u16,mut _10: isize,mut _11: usize,mut _12: usize,mut _13: u128,mut _14: isize,mut _15: isize,mut _16: f64) -> [i8; 7] {
mir! {
type RET = [i8; 7];
let _17: [i16; 4];
let _18: *mut [i64; 3];
let _19: u16;
let _20: char;
let _21: ([u8; 2],);
let _22: Adt53;
let _23: i16;
let _24: i8;
let _25: usize;
let _26: [u64; 7];
let _27: char;
let _28: [char; 7];
let _29: i64;
let _30: f64;
let _31: Adt58;
let _32: [i16; 4];
let _33: u128;
let _34: i16;
let _35: f32;
let _36: u16;
let _37: i32;
let _38: i128;
let _39: ([u8; 2],);
let _40: ();
let _41: ();
{
_16 = (-1388513012_i32) as f64;
_8 = 1662850020_u32 as u64;
RET = [12_i8,26_i8,62_i8,(-88_i8),126_i8,55_i8,81_i8];
_2 = _11 % (*_4);
_2 = !(*_4);
_7 = _14 * _5;
_17 = [1671_i16,(-24313_i16),13631_i16,21746_i16];
Goto(bb1)
}
bb1 = {
_2 = (-5_i8) as usize;
_3 = !_8;
_6 = _1 - _10;
_12 = _11;
_4 = core::ptr::addr_of!(_2);
_14 = !_6;
_2 = !_11;
_10 = (-114599697910420098466649781059258485168_i128) as isize;
_1 = _14;
_7 = _6;
(*_4) = _3 as usize;
_3 = 43_u8 as u64;
_14 = !_1;
_15 = !_1;
_8 = _3 * _3;
_3 = !_8;
_2 = _3 as usize;
_1 = !_14;
Goto(bb2)
}
bb2 = {
_16 = (-27904986438845341057191809405776743829_i128) as f64;
_12 = !(*_4);
match _9 {
0 => bb1,
28452 => bb4,
_ => bb3
}
}
bb3 = {
_2 = (-5_i8) as usize;
_3 = !_8;
_6 = _1 - _10;
_12 = _11;
_4 = core::ptr::addr_of!(_2);
_14 = !_6;
_2 = !_11;
_10 = (-114599697910420098466649781059258485168_i128) as isize;
_1 = _14;
_7 = _6;
(*_4) = _3 as usize;
_3 = 43_u8 as u64;
_14 = !_1;
_15 = !_1;
_8 = _3 * _3;
_3 = !_8;
_2 = _3 as usize;
_1 = !_14;
Goto(bb2)
}
bb4 = {
_7 = (-59115943084041173130244168909734659800_i128) as isize;
_19 = _9 * _9;
_19 = 146_u8 as u16;
_2 = _12;
_17 = [(-13015_i16),(-9497_i16),8160_i16,(-19960_i16)];
(*_4) = _11 ^ _12;
_12 = _15 as usize;
(*_4) = _12;
_17 = [17480_i16,(-23277_i16),5797_i16,(-24396_i16)];
_11 = _9 as usize;
_5 = _1 * _15;
_6 = _14;
RET = [(-20_i8),7_i8,(-118_i8),(-11_i8),(-126_i8),34_i8,74_i8];
_4 = core::ptr::addr_of!(_11);
_2 = _12 >> _5;
_17 = [(-867_i16),(-21805_i16),20523_i16,(-5410_i16)];
_7 = _1 * _10;
_8 = _3 + _3;
_15 = _14;
(*_4) = _2 << _2;
_8 = !_3;
match _13 {
0 => bb5,
159864145759811500720924696041522080139 => bb7,
_ => bb6
}
}
bb5 = {
_2 = (-5_i8) as usize;
_3 = !_8;
_6 = _1 - _10;
_12 = _11;
_4 = core::ptr::addr_of!(_2);
_14 = !_6;
_2 = !_11;
_10 = (-114599697910420098466649781059258485168_i128) as isize;
_1 = _14;
_7 = _6;
(*_4) = _3 as usize;
_3 = 43_u8 as u64;
_14 = !_1;
_15 = !_1;
_8 = _3 * _3;
_3 = !_8;
_2 = _3 as usize;
_1 = !_14;
Goto(bb2)
}
bb6 = {
_16 = (-27904986438845341057191809405776743829_i128) as f64;
_12 = !(*_4);
match _9 {
0 => bb1,
28452 => bb4,
_ => bb3
}
}
bb7 = {
_8 = _19 as u64;
RET = [102_i8,(-107_i8),124_i8,(-52_i8),70_i8,25_i8,0_i8];
_15 = _5 - _7;
_10 = true as isize;
_11 = _2;
match _9 {
0 => bb2,
1 => bb8,
2 => bb9,
28452 => bb11,
_ => bb10
}
}
bb8 = {
_2 = (-5_i8) as usize;
_3 = !_8;
_6 = _1 - _10;
_12 = _11;
_4 = core::ptr::addr_of!(_2);
_14 = !_6;
_2 = !_11;
_10 = (-114599697910420098466649781059258485168_i128) as isize;
_1 = _14;
_7 = _6;
(*_4) = _3 as usize;
_3 = 43_u8 as u64;
_14 = !_1;
_15 = !_1;
_8 = _3 * _3;
_3 = !_8;
_2 = _3 as usize;
_1 = !_14;
Goto(bb2)
}
bb9 = {
_2 = (-5_i8) as usize;
_3 = !_8;
_6 = _1 - _10;
_12 = _11;
_4 = core::ptr::addr_of!(_2);
_14 = !_6;
_2 = !_11;
_10 = (-114599697910420098466649781059258485168_i128) as isize;
_1 = _14;
_7 = _6;
(*_4) = _3 as usize;
_3 = 43_u8 as u64;
_14 = !_1;
_15 = !_1;
_8 = _3 * _3;
_3 = !_8;
_2 = _3 as usize;
_1 = !_14;
Goto(bb2)
}
bb10 = {
_7 = (-59115943084041173130244168909734659800_i128) as isize;
_19 = _9 * _9;
_19 = 146_u8 as u16;
_2 = _12;
_17 = [(-13015_i16),(-9497_i16),8160_i16,(-19960_i16)];
(*_4) = _11 ^ _12;
_12 = _15 as usize;
(*_4) = _12;
_17 = [17480_i16,(-23277_i16),5797_i16,(-24396_i16)];
_11 = _9 as usize;
_5 = _1 * _15;
_6 = _14;
RET = [(-20_i8),7_i8,(-118_i8),(-11_i8),(-126_i8),34_i8,74_i8];
_4 = core::ptr::addr_of!(_11);
_2 = _12 >> _5;
_17 = [(-867_i16),(-21805_i16),20523_i16,(-5410_i16)];
_7 = _1 * _10;
_8 = _3 + _3;
_15 = _14;
(*_4) = _2 << _2;
_8 = !_3;
match _13 {
0 => bb5,
159864145759811500720924696041522080139 => bb7,
_ => bb6
}
}
bb11 = {
_21.0 = [227_u8,126_u8];
_2 = _11;
_12 = 162349120466375595232511845851445876585_i128 as usize;
_21.0 = [128_u8,254_u8];
_5 = -_7;
_14 = _15 + _15;
_22 = Adt53::Variant0 { fld0: _21,fld1: (*_4) };
_25 = (*_4) >> (*_4);
_13 = 1665936454654889741_i64 as u128;
_3 = _9 as u64;
_24 = !(-76_i8);
Goto(bb12)
}
bb12 = {
place!(Field::<([u8; 2],)>(Variant(_22, 0), 0)).0 = _21.0;
_1 = '\u{79b90}' as isize;
_23 = 29189_i16;
SetDiscriminant(_22, 1);
_25 = _11 >> _5;
_19 = (-1697273480_i32) as u16;
_14 = _7 ^ _15;
_21.0 = [233_u8,35_u8];
_22 = Adt53::Variant0 { fld0: _21,fld1: (*_4) };
_12 = (*_4);
_9 = !_19;
_4 = core::ptr::addr_of!(_25);
_29 = -8779905406163837805_i64;
_10 = _14 ^ _15;
_27 = '\u{3fc99}';
_16 = _19 as f64;
_3 = !_8;
match _23 {
29189 => bb13,
_ => bb1
}
}
bb13 = {
_27 = '\u{b858c}';
_11 = !Field::<usize>(Variant(_22, 0), 1);
SetDiscriminant(_22, 0);
_13 = 32915217679262938841898236982732204238_u128;
place!(Field::<usize>(Variant(_22, 0), 1)) = 2348599320_u32 as usize;
_31.fld0 = core::ptr::addr_of_mut!(_23);
place!(Field::<([u8; 2],)>(Variant(_22, 0), 0)) = (_21.0,);
place!(Field::<usize>(Variant(_22, 0), 1)) = 295559833_u32 as usize;
_7 = _14;
_9 = _19 | _19;
_8 = _3;
_10 = _15 >> _15;
place!(Field::<([u8; 2],)>(Variant(_22, 0), 0)) = (_21.0,);
_11 = (*_4) << _10;
_1 = !_15;
_13 = 85185090956090406129832411261822231876_u128 | 53737386010713509668438434257093734174_u128;
_27 = '\u{a3233}';
place!(Field::<([u8; 2],)>(Variant(_22, 0), 0)).0 = _21.0;
(*_4) = !_11;
_14 = _1 * _7;
_21.0 = [215_u8,228_u8];
_23 = 146759580718773134017166369356371029466_i128 as i16;
_30 = _16 - _16;
Call(_35 = fn17(_14, _25, _7, _10, _14, _4), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = [_24,_24,_24,_24,_24,_24,_24];
_33 = _1 as u128;
_34 = _1 as i16;
_37 = _8 as i32;
_38 = _29 as i128;
_9 = !_19;
_39.0 = [88_u8,176_u8];
_16 = _30;
_39.0 = Field::<([u8; 2],)>(Variant(_22, 0), 0).0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(16_usize, 23_usize, Move(_23), 12_usize, Move(_12), 25_usize, Move(_25), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(16_usize, 9_usize, Move(_9), 39_usize, Move(_39), 10_usize, Move(_10), 37_usize, Move(_37)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 33_usize, Move(_33), 3_usize, Move(_3), 5_usize, Move(_5), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(16_usize, 34_usize, Move(_34), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: usize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: *const usize) -> f32 {
mir! {
type RET = f32;
let _7: [char; 7];
let _8: Adt53;
let _9: [u8; 1];
let _10: Adt48;
let _11: isize;
let _12: (i16,);
let _13: f32;
let _14: *const usize;
let _15: [u128; 3];
let _16: Adt43;
let _17: usize;
let _18: [u64; 7];
let _19: (i8,);
let _20: Adt53;
let _21: [u128; 3];
let _22: isize;
let _23: bool;
let _24: *const usize;
let _25: ();
let _26: ();
{
RET = (-41_i8) as f32;
_5 = _1;
_5 = _4 + _1;
_4 = 126054190709061422577343389504096563235_u128 as isize;
RET = 3088408132473252145_i64 as f32;
_1 = !_5;
Goto(bb1)
}
bb1 = {
RET = _2 as f32;
(*_6) = _2 ^ _2;
_3 = _5 - _1;
(*_6) = _2 | _2;
_6 = core::ptr::addr_of!((*_6));
_2 = (*_6) | (*_6);
_1 = 324967822_u32 as isize;
_7 = ['\u{9dfbf}','\u{5008f}','\u{5e863}','\u{721cb}','\u{21341}','\u{7a322}','\u{7d654}'];
_6 = core::ptr::addr_of!((*_6));
(*_6) = _3 as usize;
RET = 80121405086678677906690356366652215894_i128 as f32;
_7 = ['\u{c039a}','\u{4a62a}','\u{10fe80}','\u{53a36}','\u{107b5}','\u{4dc04}','\u{675f5}'];
_3 = (-38_i8) as isize;
_3 = _4 & _5;
RET = 9837748650926560730_u64 as f32;
_9 = [100_u8];
_5 = _3 * _3;
(*_6) = _2;
_3 = _5 << _5;
_2 = RET as usize;
_2 = !(*_6);
(*_6) = 459570495_i32 as usize;
(*_6) = RET as usize;
Call(_10 = fn18(_5, _5, _3, _3, _5, _5, _2, _3, _3, _3, _5, _3, _5, _2, _3, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = ['\u{30d31}','\u{b2354}','\u{21682}','\u{a02cf}','\u{5a9ac}','\u{4c6c0}','\u{250e9}'];
_13 = (-7845756055845064327_i64) as f32;
RET = -_13;
_5 = !_3;
_12 = (30763_i16,);
_9 = [212_u8];
SetDiscriminant(_10, 0);
place!(Field::<u64>(Variant(_10, 0), 0)) = !17221950390868812575_u64;
_14 = core::ptr::addr_of!((*_6));
match _12.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
30763 => bb9,
_ => bb8
}
}
bb3 = {
RET = _2 as f32;
(*_6) = _2 ^ _2;
_3 = _5 - _1;
(*_6) = _2 | _2;
_6 = core::ptr::addr_of!((*_6));
_2 = (*_6) | (*_6);
_1 = 324967822_u32 as isize;
_7 = ['\u{9dfbf}','\u{5008f}','\u{5e863}','\u{721cb}','\u{21341}','\u{7a322}','\u{7d654}'];
_6 = core::ptr::addr_of!((*_6));
(*_6) = _3 as usize;
RET = 80121405086678677906690356366652215894_i128 as f32;
_7 = ['\u{c039a}','\u{4a62a}','\u{10fe80}','\u{53a36}','\u{107b5}','\u{4dc04}','\u{675f5}'];
_3 = (-38_i8) as isize;
_3 = _4 & _5;
RET = 9837748650926560730_u64 as f32;
_9 = [100_u8];
_5 = _3 * _3;
(*_6) = _2;
_3 = _5 << _5;
_2 = RET as usize;
_2 = !(*_6);
(*_6) = 459570495_i32 as usize;
(*_6) = RET as usize;
Call(_10 = fn18(_5, _5, _3, _3, _5, _5, _2, _3, _3, _3, _5, _3, _5, _2, _3, _2), ReturnTo(bb2), UnwindUnreachable())
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
_15 = [211744213999042580139679583220299689775_u128,339232686435256544374881582147035303121_u128,271631359786743335978289133642126196553_u128];
(*_6) = _2;
_7 = ['\u{100b22}','\u{12487}','\u{6c46b}','\u{dbc48}','\u{5339}','\u{226c0}','\u{48ab1}'];
_13 = 2475855870_u32 as f32;
_3 = !_5;
Goto(bb10)
}
bb10 = {
(*_6) = _2 << _5;
_2 = (*_6) * (*_6);
RET = 2226352446_u32 as f32;
_17 = _2;
(*_14) = !_17;
_15 = [284203512973773197161696300488702753758_u128,92196255768723608213776464819691588051_u128,94666727673472225569808147799079612858_u128];
SetDiscriminant(_10, 0);
_1 = !_5;
_18 = [2751030615812270712_u64,4848808398514588119_u64,9060907129558508761_u64,4651555952516479993_u64,577908261572424833_u64,5538699040042742070_u64,17674244855602364443_u64];
_1 = _5;
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb5,
5 => bb11,
6 => bb12,
30763 => bb14,
_ => bb13
}
}
bb11 = {
_7 = ['\u{30d31}','\u{b2354}','\u{21682}','\u{a02cf}','\u{5a9ac}','\u{4c6c0}','\u{250e9}'];
_13 = (-7845756055845064327_i64) as f32;
RET = -_13;
_5 = !_3;
_12 = (30763_i16,);
_9 = [212_u8];
SetDiscriminant(_10, 0);
place!(Field::<u64>(Variant(_10, 0), 0)) = !17221950390868812575_u64;
_14 = core::ptr::addr_of!((*_6));
match _12.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
30763 => bb9,
_ => bb8
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_5 = -_3;
(*_14) = _17;
RET = 57006_u16 as f32;
_22 = !_3;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(17_usize, 15_usize, Move(_15), 17_usize, Move(_17), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(17_usize, 7_usize, Move(_7), 12_usize, Move(_12), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: usize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: usize,mut _15: isize,mut _16: usize) -> Adt48 {
mir! {
type RET = Adt48;
let _17: i8;
let _18: Adt50;
let _19: u8;
let _20: *const usize;
let _21: [i64; 2];
let _22: isize;
let _23: isize;
let _24: bool;
let _25: usize;
let _26: [u8; 1];
let _27: isize;
let _28: u128;
let _29: [i16; 4];
let _30: [isize; 5];
let _31: isize;
let _32: [i64; 2];
let _33: u8;
let _34: Adt53;
let _35: usize;
let _36: f64;
let _37: (i8,);
let _38: usize;
let _39: u64;
let _40: Adt52;
let _41: isize;
let _42: (*mut i16, *const [isize; 5], ([u8; 2],), *mut [i64; 3]);
let _43: f32;
let _44: u8;
let _45: [u8; 1];
let _46: (i8,);
let _47: u64;
let _48: [u8; 1];
let _49: char;
let _50: isize;
let _51: bool;
let _52: [char; 7];
let _53: f32;
let _54: f32;
let _55: [i64; 3];
let _56: [u8; 1];
let _57: [isize; 5];
let _58: [i64; 2];
let _59: [char; 7];
let _60: char;
let _61: [char; 7];
let _62: u32;
let _63: isize;
let _64: *const *mut i16;
let _65: usize;
let _66: Adt43;
let _67: Adt44;
let _68: ();
let _69: ();
{
_16 = (-128781803865833507034821661799384502050_i128) as usize;
_14 = !_7;
Goto(bb1)
}
bb1 = {
_4 = _5;
_10 = -_13;
_13 = (-2015_i16) as isize;
_10 = _9;
_15 = !_11;
_11 = 9492141689290955630_u64 as isize;
_17 = 87_i8 * (-86_i8);
_3 = _12 * _5;
_9 = _14 as isize;
_3 = 525416571_u32 as isize;
_13 = _17 as isize;
_1 = -_10;
_17 = false as i8;
_10 = _9;
_3 = _15 | _15;
_8 = (-287460194980221087_i64) as isize;
_2 = -_1;
_19 = _15 as u8;
Goto(bb2)
}
bb2 = {
_8 = -_6;
_21 = [1671127266799764746_i64,7562132808109012597_i64];
RET = Adt48::Variant0 { fld0: 6844625962237393191_u64 };
_21 = [1597192174873477387_i64,9001295629860365541_i64];
place!(Field::<u64>(Variant(RET, 0), 0)) = 17369661350181243737_u64;
_8 = -_10;
_6 = _19 as isize;
_19 = 231_u8;
RET = Adt48::Variant0 { fld0: 4016467899339842882_u64 };
_16 = (-109525551979874907890738643975794300441_i128) as usize;
place!(Field::<u64>(Variant(RET, 0), 0)) = 18155807592108796688_u64 | 7592471607923525782_u64;
match _19 {
0 => bb3,
1 => bb4,
231 => bb6,
_ => bb5
}
}
bb3 = {
_4 = _5;
_10 = -_13;
_13 = (-2015_i16) as isize;
_10 = _9;
_15 = !_11;
_11 = 9492141689290955630_u64 as isize;
_17 = 87_i8 * (-86_i8);
_3 = _12 * _5;
_9 = _14 as isize;
_3 = 525416571_u32 as isize;
_13 = _17 as isize;
_1 = -_10;
_17 = false as i8;
_10 = _9;
_3 = _15 | _15;
_8 = (-287460194980221087_i64) as isize;
_2 = -_1;
_19 = _15 as u8;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_23 = _9 + _5;
_23 = true as isize;
_13 = _5 * _5;
_14 = (-24803_i16) as usize;
_20 = core::ptr::addr_of!(_16);
Goto(bb7)
}
bb7 = {
_3 = true as isize;
_19 = 90_u8 + 203_u8;
place!(Field::<u64>(Variant(RET, 0), 0)) = !11008139199086611300_u64;
_20 = core::ptr::addr_of!(_25);
_22 = _15 >> _12;
_14 = _7 & _7;
_5 = _10;
_7 = _14 >> _1;
_24 = !false;
_23 = _19 as isize;
SetDiscriminant(RET, 1);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = [_13,_5,_8,_22,_22];
_22 = _17 as isize;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = core::ptr::addr_of!(place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4);
_28 = 204476645303380202096682258951868565166_u128 << _5;
_14 = _7;
_2 = _13;
_7 = _14 + _14;
_29 = [(-25920_i16),(-12770_i16),15214_i16,4397_i16];
_25 = !_14;
_23 = _9 | _12;
RET = Adt48::Variant0 { fld0: 16254997660681746187_u64 };
Goto(bb8)
}
bb8 = {
_22 = _1 & _23;
_15 = !_12;
Goto(bb9)
}
bb9 = {
_26 = [_19];
_27 = -_22;
(*_20) = _24 as usize;
_2 = (-3563212105578614588_i64) as isize;
_32 = [6463170582940974641_i64,1880549088248086469_i64];
Call(_30 = fn19(_15, _15, _5, _9, _27, _23, _1, _12, _4, _13, _10, _1, _15, _13, _28, _4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = Adt48::Variant0 { fld0: 4701673510857437159_u64 };
_6 = _4;
_32 = _21;
place!(Field::<u64>(Variant(RET, 0), 0)) = (-6836856414022673273_i64) as u64;
_30 = [_27,_13,_8,_1,_1];
Goto(bb11)
}
bb11 = {
_5 = _13;
_24 = false;
_7 = _17 as usize;
_20 = core::ptr::addr_of!(_14);
_30 = [_27,_23,_15,_10,_5];
_4 = _15 * _27;
_16 = _14;
_20 = core::ptr::addr_of!(_38);
_9 = Field::<u64>(Variant(RET, 0), 0) as isize;
_37 = (_17,);
_3 = _19 as isize;
_10 = _22 * _27;
_25 = _16;
_28 = 95663294101062935047141883983186204331_u128 & 294736592227837503617994333472779143638_u128;
_25 = _24 as usize;
_23 = !_27;
(*_20) = _14 * _14;
_21 = [(-2688217507331826863_i64),(-6539293644594377708_i64)];
_14 = '\u{139eb}' as usize;
_12 = _24 as isize;
Goto(bb12)
}
bb12 = {
_20 = core::ptr::addr_of!(_14);
_9 = (-1448521837_i32) as isize;
_25 = _38 - _38;
_39 = Field::<u64>(Variant(RET, 0), 0) - Field::<u64>(Variant(RET, 0), 0);
_37 = (_17,);
_26 = [_19];
_40 = Adt52::Variant1 { fld0: _20,fld1: _32 };
_28 = 7926_i16 as u128;
_38 = _25;
_40 = Adt52::Variant0 { fld0: _23 };
_10 = -_22;
RET = Adt48::Variant0 { fld0: _39 };
_8 = _15 ^ _1;
_19 = 221_u8;
match _19 {
0 => bb6,
1 => bb10,
2 => bb4,
3 => bb13,
4 => bb14,
5 => bb15,
221 => bb17,
_ => bb16
}
}
bb13 = {
Return()
}
bb14 = {
_3 = true as isize;
_19 = 90_u8 + 203_u8;
place!(Field::<u64>(Variant(RET, 0), 0)) = !11008139199086611300_u64;
_20 = core::ptr::addr_of!(_25);
_22 = _15 >> _12;
_14 = _7 & _7;
_5 = _10;
_7 = _14 >> _1;
_24 = !false;
_23 = _19 as isize;
SetDiscriminant(RET, 1);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = [_13,_5,_8,_22,_22];
_22 = _17 as isize;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = core::ptr::addr_of!(place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4);
_28 = 204476645303380202096682258951868565166_u128 << _5;
_14 = _7;
_2 = _13;
_7 = _14 + _14;
_29 = [(-25920_i16),(-12770_i16),15214_i16,4397_i16];
_25 = !_14;
_23 = _9 | _12;
RET = Adt48::Variant0 { fld0: 16254997660681746187_u64 };
Goto(bb8)
}
bb15 = {
_26 = [_19];
_27 = -_22;
(*_20) = _24 as usize;
_2 = (-3563212105578614588_i64) as isize;
_32 = [6463170582940974641_i64,1880549088248086469_i64];
Call(_30 = fn19(_15, _15, _5, _9, _27, _23, _1, _12, _4, _13, _10, _1, _15, _13, _28, _4), ReturnTo(bb10), UnwindUnreachable())
}
bb16 = {
_4 = _5;
_10 = -_13;
_13 = (-2015_i16) as isize;
_10 = _9;
_15 = !_11;
_11 = 9492141689290955630_u64 as isize;
_17 = 87_i8 * (-86_i8);
_3 = _12 * _5;
_9 = _14 as isize;
_3 = 525416571_u32 as isize;
_13 = _17 as isize;
_1 = -_10;
_17 = false as i8;
_10 = _9;
_3 = _15 | _15;
_8 = (-287460194980221087_i64) as isize;
_2 = -_1;
_19 = _15 as u8;
Goto(bb2)
}
bb17 = {
_16 = '\u{f5cb2}' as usize;
_29 = [29875_i16,32574_i16,(-129_i16),19295_i16];
_38 = _37.0 as usize;
(*_20) = _25;
SetDiscriminant(RET, 1);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = core::ptr::addr_of!(place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4);
_33 = Field::<isize>(Variant(_40, 0), 0) as u8;
_37 = (_17,);
_6 = _4 >> (*_20);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).5 = _17;
place!(Field::<bool>(Variant(RET, 1), 0)) = _24;
_40 = Adt52::Variant0 { fld0: _5 };
_37.0 = _17;
_36 = _33 as f64;
_35 = !(*_20);
_28 = !136807858921824121433191404883098711534_u128;
place!(Field::<bool>(Variant(RET, 1), 0)) = !_24;
_4 = -_23;
Goto(bb18)
}
bb18 = {
_40 = Adt52::Variant0 { fld0: _1 };
_10 = -_5;
_23 = !_15;
_12 = _22;
_27 = !_23;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = _30;
SetDiscriminant(_40, 0);
_42.1 = core::ptr::addr_of!(place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4);
_8 = _15 * _5;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = _30;
_41 = !_23;
_42.2.0 = [_33,_33];
_5 = _13;
_6 = -_10;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).1 = 1204544256_u32 as f32;
_20 = core::ptr::addr_of!(_7);
_16 = !_35;
_40 = Adt52::Variant0 { fld0: _27 };
_45 = [_33];
_42.2.0 = [_33,_33];
match _19 {
0 => bb6,
1 => bb2,
2 => bb16,
221 => bb20,
_ => bb19
}
}
bb19 = {
_22 = _1 & _23;
_15 = !_12;
Goto(bb9)
}
bb20 = {
_2 = (-143084824246197946604706822934183374183_i128) as isize;
_43 = -Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).1;
_29 = [(-2612_i16),21567_i16,23202_i16,1697_i16];
SetDiscriminant(_40, 1);
_3 = _23;
_1 = -_22;
place!(Field::<[i64; 2]>(Variant(_40, 1), 1)) = [7475536956910459290_i64,(-2276316595441254343_i64)];
_36 = _28 as f64;
_15 = _23;
_40 = Adt52::Variant1 { fld0: _20,fld1: _21 };
_46 = _37;
_28 = _24 as u128;
_6 = _27;
SetDiscriminant(_40, 0);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = _42.1;
_37.0 = Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).5;
_14 = _16 >> _13;
_45 = [_33];
_29 = [26889_i16,(-16937_i16),19767_i16,(-19867_i16)];
_45 = _26;
place!(Field::<bool>(Variant(RET, 1), 0)) = !_24;
_7 = _24 as usize;
match _19 {
0 => bb21,
1 => bb22,
221 => bb24,
_ => bb23
}
}
bb21 = {
_3 = true as isize;
_19 = 90_u8 + 203_u8;
place!(Field::<u64>(Variant(RET, 0), 0)) = !11008139199086611300_u64;
_20 = core::ptr::addr_of!(_25);
_22 = _15 >> _12;
_14 = _7 & _7;
_5 = _10;
_7 = _14 >> _1;
_24 = !false;
_23 = _19 as isize;
SetDiscriminant(RET, 1);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = [_13,_5,_8,_22,_22];
_22 = _17 as isize;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = core::ptr::addr_of!(place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4);
_28 = 204476645303380202096682258951868565166_u128 << _5;
_14 = _7;
_2 = _13;
_7 = _14 + _14;
_29 = [(-25920_i16),(-12770_i16),15214_i16,4397_i16];
_25 = !_14;
_23 = _9 | _12;
RET = Adt48::Variant0 { fld0: 16254997660681746187_u64 };
Goto(bb8)
}
bb22 = {
_3 = true as isize;
_19 = 90_u8 + 203_u8;
place!(Field::<u64>(Variant(RET, 0), 0)) = !11008139199086611300_u64;
_20 = core::ptr::addr_of!(_25);
_22 = _15 >> _12;
_14 = _7 & _7;
_5 = _10;
_7 = _14 >> _1;
_24 = !false;
_23 = _19 as isize;
SetDiscriminant(RET, 1);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = [_13,_5,_8,_22,_22];
_22 = _17 as isize;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = core::ptr::addr_of!(place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4);
_28 = 204476645303380202096682258951868565166_u128 << _5;
_14 = _7;
_2 = _13;
_7 = _14 + _14;
_29 = [(-25920_i16),(-12770_i16),15214_i16,4397_i16];
_25 = !_14;
_23 = _9 | _12;
RET = Adt48::Variant0 { fld0: 16254997660681746187_u64 };
Goto(bb8)
}
bb23 = {
_4 = _5;
_10 = -_13;
_13 = (-2015_i16) as isize;
_10 = _9;
_15 = !_11;
_11 = 9492141689290955630_u64 as isize;
_17 = 87_i8 * (-86_i8);
_3 = _12 * _5;
_9 = _14 as isize;
_3 = 525416571_u32 as isize;
_13 = _17 as isize;
_1 = -_10;
_17 = false as i8;
_10 = _9;
_3 = _15 | _15;
_8 = (-287460194980221087_i64) as isize;
_2 = -_1;
_19 = _15 as u8;
Goto(bb2)
}
bb24 = {
_35 = !_14;
_1 = _5 << _16;
(*_20) = _16 << _5;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).5 = _46.0 << _3;
place!(Field::<isize>(Variant(_40, 0), 0)) = _4 * _6;
_46 = (Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).5,);
(*_20) = _35;
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).1 = _43;
_45 = [_33];
SetDiscriminant(_40, 1);
_22 = _24 as isize;
_3 = _43 as isize;
_41 = _12 * _12;
_1 = _27 - _41;
_23 = 2686_i16 as isize;
_51 = !Field::<bool>(Variant(RET, 1), 0);
_50 = _27;
place!(Field::<bool>(Variant(RET, 1), 0)) = _35 != _35;
_46.0 = Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).5;
_38 = !_7;
_3 = _28 as isize;
match _19 {
0 => bb25,
221 => bb27,
_ => bb26
}
}
bb25 = {
Return()
}
bb26 = {
Return()
}
bb27 = {
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).5 = _46.0 >> _1;
_47 = _39;
_37.0 = (-136063273723065569930255279341507345257_i128) as i8;
_29 = [(-32495_i16),32652_i16,(-23544_i16),(-22234_i16)];
_14 = !_38;
_37 = _46;
_32 = [1026789840930991411_i64,(-8563442662725040419_i64)];
_23 = _10 | _10;
_10 = _23 - _1;
_30 = Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).4;
_5 = _12;
_37 = (_46.0,);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).5 = 72594537581407255531982569390514357318_i128 as i8;
(*_20) = _25;
Goto(bb28)
}
bb28 = {
_55 = [7336691252364132187_i64,743121815489365905_i64,(-5465913911188891928_i64)];
_44 = !_33;
_38 = (*_20);
_54 = 38541_u16 as f32;
_45 = _26;
match _19 {
0 => bb16,
1 => bb23,
2 => bb19,
3 => bb11,
4 => bb12,
221 => bb30,
_ => bb29
}
}
bb29 = {
_2 = (-143084824246197946604706822934183374183_i128) as isize;
_43 = -Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).1;
_29 = [(-2612_i16),21567_i16,23202_i16,1697_i16];
SetDiscriminant(_40, 1);
_3 = _23;
_1 = -_22;
place!(Field::<[i64; 2]>(Variant(_40, 1), 1)) = [7475536956910459290_i64,(-2276316595441254343_i64)];
_36 = _28 as f64;
_15 = _23;
_40 = Adt52::Variant1 { fld0: _20,fld1: _21 };
_46 = _37;
_28 = _24 as u128;
_6 = _27;
SetDiscriminant(_40, 0);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = _42.1;
_37.0 = Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).5;
_14 = _16 >> _13;
_45 = [_33];
_29 = [26889_i16,(-16937_i16),19767_i16,(-19867_i16)];
_45 = _26;
place!(Field::<bool>(Variant(RET, 1), 0)) = !_24;
_7 = _24 as usize;
match _19 {
0 => bb21,
1 => bb22,
221 => bb24,
_ => bb23
}
}
bb30 = {
_20 = core::ptr::addr_of!(_16);
_4 = _27;
_33 = (-99396699_i32) as u8;
_27 = _15;
_37.0 = _46.0;
_12 = 8296753615280017486_i64 as isize;
_37 = _46;
_7 = _25;
_48 = _45;
match _19 {
0 => bb1,
1 => bb5,
2 => bb31,
221 => bb33,
_ => bb32
}
}
bb31 = {
_2 = (-143084824246197946604706822934183374183_i128) as isize;
_43 = -Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).1;
_29 = [(-2612_i16),21567_i16,23202_i16,1697_i16];
SetDiscriminant(_40, 1);
_3 = _23;
_1 = -_22;
place!(Field::<[i64; 2]>(Variant(_40, 1), 1)) = [7475536956910459290_i64,(-2276316595441254343_i64)];
_36 = _28 as f64;
_15 = _23;
_40 = Adt52::Variant1 { fld0: _20,fld1: _21 };
_46 = _37;
_28 = _24 as u128;
_6 = _27;
SetDiscriminant(_40, 0);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).2 = _42.1;
_37.0 = Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).5;
_14 = _16 >> _13;
_45 = [_33];
_29 = [26889_i16,(-16937_i16),19767_i16,(-19867_i16)];
_45 = _26;
place!(Field::<bool>(Variant(RET, 1), 0)) = !_24;
_7 = _24 as usize;
match _19 {
0 => bb21,
1 => bb22,
221 => bb24,
_ => bb23
}
}
bb32 = {
_4 = _5;
_10 = -_13;
_13 = (-2015_i16) as isize;
_10 = _9;
_15 = !_11;
_11 = 9492141689290955630_u64 as isize;
_17 = 87_i8 * (-86_i8);
_3 = _12 * _5;
_9 = _14 as isize;
_3 = 525416571_u32 as isize;
_13 = _17 as isize;
_1 = -_10;
_17 = false as i8;
_10 = _9;
_3 = _15 | _15;
_8 = (-287460194980221087_i64) as isize;
_2 = -_1;
_19 = _15 as u8;
Goto(bb2)
}
bb33 = {
_62 = 16328706_u32 + 216511889_u32;
_59 = ['\u{8f0d0}','\u{1d85f}','\u{25515}','\u{d920a}','\u{f4c9f}','\u{8970a}','\u{90d7f}'];
_46 = (_37.0,);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).4 = [_15,_41,_5,_27,_27];
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).5 = _23 as i8;
_61 = ['\u{91cd}','\u{25fcc}','\u{cc126}','\u{7b82a}','\u{27d68}','\u{10188e}','\u{b6e0a}'];
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).3 = core::ptr::addr_of_mut!(_55);
place!(Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1)).1 = -_54;
_49 = '\u{477f0}';
place!(Field::<bool>(Variant(RET, 1), 0)) = _41 <= _41;
_30 = Field::<(*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8)>(Variant(RET, 1), 1).4;
_32 = _21;
place!(Field::<bool>(Variant(RET, 1), 0)) = _7 <= _14;
_19 = _46.0 as u8;
_3 = _10;
RET = Adt48::Variant0 { fld0: _47 };
Goto(bb34)
}
bb34 = {
Call(_68 = dump_var(18_usize, 28_usize, Move(_28), 10_usize, Move(_10), 32_usize, Move(_32), 13_usize, Move(_13)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_68 = dump_var(18_usize, 48_usize, Move(_48), 23_usize, Move(_23), 8_usize, Move(_8), 21_usize, Move(_21)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_68 = dump_var(18_usize, 25_usize, Move(_25), 41_usize, Move(_41), 27_usize, Move(_27), 24_usize, Move(_24)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_68 = dump_var(18_usize, 4_usize, Move(_4), 47_usize, Move(_47), 9_usize, Move(_9), 59_usize, Move(_59)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_68 = dump_var(18_usize, 35_usize, Move(_35), 2_usize, Move(_2), 12_usize, Move(_12), 22_usize, Move(_22)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_68 = dump_var(18_usize, 45_usize, Move(_45), 15_usize, Move(_15), 33_usize, Move(_33), 69_usize, _69), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: u128,mut _16: isize) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _17: [i16; 4];
let _18: [u128; 3];
let _19: [u8; 2];
let _20: [isize; 5];
let _21: ();
let _22: ();
{
_4 = _16;
_15 = 156693930513490727026220896369167342929_u128;
_15 = 296614045019587300377514049188536626162_u128 ^ 26883057456790580212050104565066561608_u128;
Goto(bb1)
}
bb1 = {
_2 = !_12;
_14 = _4;
RET = [_11,_16,_5,_12,_12];
_4 = (-1806456721_i32) as isize;
_7 = _5 & _3;
_8 = -_14;
_13 = !_1;
_4 = _9;
_19 = [32_u8,24_u8];
RET = [_6,_1,_6,_10,_8];
_13 = _11 - _8;
Goto(bb2)
}
bb2 = {
Call(_21 = dump_var(19_usize, 11_usize, Move(_11), 4_usize, Move(_4), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_21 = dump_var(19_usize, 14_usize, Move(_14), 13_usize, Move(_13), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{61ce8}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-64_i8)), std::hint::black_box(11299_i16), std::hint::black_box((-683209818_i32)), std::hint::black_box((-6691637344267293155_i64)), std::hint::black_box((-78323079941355827888312146641571437661_i128)), std::hint::black_box(5_usize), std::hint::black_box(49_u8), std::hint::black_box(50912_u16), std::hint::black_box(2053117289_u32), std::hint::black_box(12085809207165636097_u64), std::hint::black_box(83291471292303982940904344602491413571_u128));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: *const [isize; 5],
fld1: *const usize,
fld2: isize,
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u8; 1],
fld1: [i64; 4],
fld2: *const usize,
fld3: i8,
fld4: [i16; 4],
fld5: (i8,),

},
Variant1{
fld0: i128,
fld1: *const *mut i16,
fld2: *const usize,
fld3: [u8; 2],
fld4: f32,

},
Variant2{
fld0: (*mut i16, *const [isize; 5], ([u8; 2],), *mut [i64; 3]),
fld1: usize,
fld2: [i64; 2],
fld3: Adt42,
fld4: *const usize,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: usize,

},
Variant1{
fld0: usize,
fld1: char,
fld2: f64,
fld3: *const u128,
fld4: i16,
fld5: [i64; 2],
fld6: (i16,),
fld7: [u8; 1],

},
Variant2{
fld0: [u128; 3],
fld1: u64,
fld2: *const usize,
fld3: [u64; 7],
fld4: (*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8),
fld5: i32,
fld6: Adt42,
fld7: [isize; 5],

},
Variant3{
fld0: [char; 7],
fld1: [isize; 5],
fld2: *const u128,
fld3: i8,
fld4: [u64; 7],
fld5: i32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8),
fld1: [u8; 2],
fld2: f32,

},
Variant1{
fld0: i64,
fld1: *const usize,
fld2: f32,
fld3: i128,

},
Variant2{
fld0: *const u128,
fld1: [u128; 3],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: *mut [i64; 3],
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u32,
fld1: [u8; 2],
fld2: [i64; 4],

},
Variant1{
fld0: i64,
fld1: [i64; 3],
fld2: u8,
fld3: Adt42,
fld4: f64,
fld5: *const u128,

},
Variant2{
fld0: (*mut i16, *const [isize; 5], ([u8; 2],), *mut [i64; 3]),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u64,

},
Variant1{
fld0: bool,
fld1: (*const i32, f32, *const [isize; 5], *mut [i64; 3], [isize; 5], i8),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: usize,
fld1: char,
fld2: *const usize,
fld3: [i16; 4],
fld4: Adt42,
fld5: [u128; 3],
fld6: Adt46,

},
Variant1{
fld0: (i8,),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt42,
fld1: [char; 7],

},
Variant1{
fld0: *mut [i64; 3],
fld1: (i16,),
fld2: *const usize,
fld3: *mut i16,

},
Variant2{
fld0: u16,
fld1: Adt47,
fld2: [i64; 4],
fld3: i8,
fld4: [i64; 2],
fld5: [i16; 4],
fld6: i64,
fld7: Adt44,

},
Variant3{
fld0: f32,
fld1: i64,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (i16,),
fld1: [i16; 4],

},
Variant1{
fld0: Adt45,
fld1: [u8; 2],
fld2: i64,
fld3: *mut [i64; 3],
fld4: u32,

},
Variant2{
fld0: ([u8; 2],),
fld1: [i64; 3],
fld2: u128,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: isize,

},
Variant1{
fld0: *const usize,
fld1: [i64; 2],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: ([u8; 2],),
fld1: usize,

},
Variant1{
fld0: Adt52,
fld1: [u128; 3],
fld2: Adt47,
fld3: u64,
fld4: *mut [i64; 3],
fld5: Adt45,
fld6: Adt44,

},
Variant2{
fld0: u64,
fld1: [isize; 2],
fld2: isize,
fld3: f64,
fld4: i16,
fld5: *const usize,
fld6: Adt47,
fld7: Adt46,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt52,
fld1: char,
fld2: Adt47,
fld3: *const usize,
fld4: u64,

},
Variant1{
fld0: *mut i16,
fld1: Adt53,
fld2: [isize; 5],
fld3: i8,
fld4: [u8; 1],
fld5: f32,
fld6: Adt43,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *const usize,
fld1: u32,
fld2: [i64; 3],
fld3: Adt45,

},
Variant1{
fld0: Adt47,
fld1: Adt51,
fld2: isize,
fld3: Adt54,
fld4: Adt42,
fld5: [u8; 1],

},
Variant2{
fld0: Adt50,
fld1: *const [isize; 5],
fld2: [u64; 7],
fld3: Adt44,
fld4: *const i32,

},
Variant3{
fld0: u16,
fld1: *mut [i64; 3],
fld2: f32,
fld3: [i64; 2],
fld4: [i64; 4],
fld5: Adt45,
fld6: Adt53,
fld7: Adt48,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [isize; 5],

},
Variant1{
fld0: Adt52,
fld1: usize,
fld2: Adt54,
fld3: i8,
fld4: (*mut i16, *const [isize; 5], ([u8; 2],), *mut [i64; 3]),
fld5: (i8,),

},
Variant2{
fld0: bool,
fld1: [isize; 2],
fld2: [i16; 4],
fld3: u16,
fld4: Adt43,
fld5: *mut [i64; 3],

},
Variant3{
fld0: Adt49,
fld1: f64,
fld2: Adt55,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: bool,
fld1: u8,
fld2: *const usize,
fld3: Adt43,
fld4: u64,
fld5: [isize; 5],
fld6: Adt49,

},
Variant1{
fld0: bool,
fld1: Adt46,
fld2: *mut [i64; 3],
fld3: i8,
fld4: [i64; 3],

},
Variant2{
fld0: *const [isize; 5],
fld1: (*mut i16, *const [isize; 5], ([u8; 2],), *mut [i64; 3]),
fld2: usize,

},
Variant3{
fld0: bool,
fld1: [i8; 7],
fld2: (*mut i16, *const [isize; 5], ([u8; 2],), *mut [i64; 3]),
fld3: [i64; 4],
fld4: u16,
fld5: *mut [i64; 3],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: *mut i16,
}

