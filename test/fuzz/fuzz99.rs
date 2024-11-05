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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> isize {
mir! {
type RET = isize;
let _15: f64;
let _16: Adt45;
let _17: isize;
let _18: &'static u64;
let _19: Adt48;
let _20: *mut *mut i32;
let _21: [u8; 5];
let _22: u8;
let _23: char;
let _24: i16;
let _25: u32;
let _26: f64;
let _27: Adt46;
let _28: *mut i128;
let _29: [u128; 8];
let _30: u32;
let _31: [u128; 8];
let _32: ();
let _33: ();
{
_5 = (-90975747091586712331141660900540552700_i128) as i16;
_8 = 124403314696413292296642209245761249619_i128 & (-66595150726098856290406581447044171212_i128);
_14 = 14812825223271491009_usize as u128;
_10 = _14 as u8;
RET = (-73_isize) | 9223372036854775807_isize;
_2 = '\u{7a40a}';
_7 = (-168267437879257506_i64) - 3523651524599943877_i64;
_13 = !13635738914017650514_u64;
_12 = 1626353804_u32;
Call(_11 = fn1(_14, RET, _8, _2, _10, _2, RET, _2, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (-5415315626984732879_i64) | 6751490845199414319_i64;
_3 = true as isize;
_11 = !60136_u16;
_8 = (-144797261445939027763504097988397090097_i128) ^ 53746829866637588934408196921198465605_i128;
RET = _3 + _3;
_6 = 2147389249_i32;
_2 = '\u{200f3}';
RET = _3;
_6 = _8 as i32;
_10 = 85_u8 << _6;
_10 = _2 as u8;
_9 = !9544369241090845185_usize;
_8 = (-9_i8) as i128;
_12 = 3782795278_u32;
_14 = 110745865767445595876483982473019695934_u128;
_14 = _13 as u128;
_7 = !(-6126774869625033883_i64);
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
3782795278 => bb7,
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
_8 = !(-121622388657162810217962489285160629778_i128);
_14 = !127524314098926422473060664399131236627_u128;
_3 = RET;
_10 = 96_i8 as u8;
_11 = 9358_u16;
_1 = _10 >= _10;
RET = _12 as isize;
_3 = -RET;
_4 = (-27_i8) << _9;
_14 = _4 as u128;
_5 = _14 as i16;
_9 = _14 as usize;
_15 = _7 as f64;
_7 = 7655952535479109989_i64 ^ (-8913354457373202669_i64);
_5 = (-26234_i16) ^ (-11788_i16);
_15 = _13 as f64;
Call(_7 = fn13(_5, RET, _10, _4, _6, _1, RET, _11, _5, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3 = RET;
_11 = 5545_u16 + 7311_u16;
RET = _3;
_12 = !306733613_u32;
_8 = 58850012007631158245055470022753093906_i128 & 8026496546187909963039008155049021387_i128;
RET = _3;
_13 = !14865955759228997425_u64;
_5 = !12538_i16;
_7 = 5567559769706426887_i64;
RET = !_3;
_14 = !201049128664823047970936143476472463451_u128;
_6 = (-300399726_i32);
_12 = 230076047_u32;
RET = _11 as isize;
_14 = 340265404573103451208831750129833534965_u128 & 56571180337021858493154852773860495473_u128;
Goto(bb9)
}
bb9 = {
_16.fld0 = !_9;
RET = _3;
_4 = (-69_i8);
_3 = !RET;
_11 = _12 as u16;
_17 = !_3;
match _12 {
0 => bb6,
230076047 => bb11,
_ => bb10
}
}
bb10 = {
_8 = !(-121622388657162810217962489285160629778_i128);
_14 = !127524314098926422473060664399131236627_u128;
_3 = RET;
_10 = 96_i8 as u8;
_11 = 9358_u16;
_1 = _10 >= _10;
RET = _12 as isize;
_3 = -RET;
_4 = (-27_i8) << _9;
_14 = _4 as u128;
_5 = _14 as i16;
_9 = _14 as usize;
_15 = _7 as f64;
_7 = 7655952535479109989_i64 ^ (-8913354457373202669_i64);
_5 = (-26234_i16) ^ (-11788_i16);
_15 = _13 as f64;
Call(_7 = fn13(_5, RET, _10, _4, _6, _1, RET, _11, _5, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb11 = {
_3 = _17;
RET = _17;
_13 = _4 as u64;
_6 = (-132657195_i32) + (-1943702895_i32);
RET = _3 ^ _17;
_16.fld0 = _9;
_12 = !3035601045_u32;
RET = _3;
RET = !_17;
_21 = [_10,_10,_10,_10,_10];
_16 = Adt45 { fld0: _9,fld1: _14 };
_15 = _11 as f64;
_18 = &_13;
_23 = _2;
_16 = Adt45 { fld0: _9,fld1: _14 };
_16 = Adt45 { fld0: _9,fld1: _14 };
_16 = Adt45 { fld0: _9,fld1: _14 };
_16.fld1 = !_14;
_13 = 17895269132328611117_u64;
_25 = _12 >> _14;
_18 = &_13;
_27.fld1.1 = _4 as f32;
Call(_22 = fn17(_27.fld1.1, _14, (*_18), _25, _15, _25, (*_18), _25, _17), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = !_16.fld1;
_13 = 11743171880014082572_u64 ^ 12710450176938988873_u64;
_7 = 8767065131113107956_i64 << _6;
_14 = _16.fld1;
_13 = !12661845456401455990_u64;
_3 = RET;
_13 = !10518997999320884842_u64;
RET = _17;
_6 = !726417876_i32;
_27.fld3.0 = _1 as u8;
_3 = _17;
_10 = _27.fld3.0;
RET = _17 ^ _17;
_10 = _16.fld0 as u8;
_27.fld3.0 = _11 as u8;
match _4 {
0 => bb1,
1 => bb7,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463463374607431768211387 => bb19,
_ => bb18
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_16.fld0 = !_9;
RET = _3;
_4 = (-69_i8);
_3 = !RET;
_11 = _12 as u16;
_17 = !_3;
match _12 {
0 => bb6,
230076047 => bb11,
_ => bb10
}
}
bb16 = {
Return()
}
bb17 = {
_8 = !(-121622388657162810217962489285160629778_i128);
_14 = !127524314098926422473060664399131236627_u128;
_3 = RET;
_10 = 96_i8 as u8;
_11 = 9358_u16;
_1 = _10 >= _10;
RET = _12 as isize;
_3 = -RET;
_4 = (-27_i8) << _9;
_14 = _4 as u128;
_5 = _14 as i16;
_9 = _14 as usize;
_15 = _7 as f64;
_7 = 7655952535479109989_i64 ^ (-8913354457373202669_i64);
_5 = (-26234_i16) ^ (-11788_i16);
_15 = _13 as f64;
Call(_7 = fn13(_5, RET, _10, _4, _6, _1, RET, _11, _5, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
_18 = &_13;
_13 = _23 as u64;
_26 = _15;
_27.fld3 = (_10, _12);
_26 = -_15;
_7 = !6317696523803754678_i64;
_18 = &_13;
_29 = [_16.fld1,_16.fld1,_14,_14,_16.fld1,_14,_16.fld1,_16.fld1];
_24 = _5;
_26 = _15;
_16.fld0 = !_9;
_3 = _8 as isize;
Goto(bb20)
}
bb20 = {
Call(_32 = dump_var(0_usize, 25_usize, Move(_25), 2_usize, Move(_2), 23_usize, Move(_23), 11_usize, Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(0_usize, 3_usize, Move(_3), 13_usize, Move(_13), 8_usize, Move(_8), 22_usize, Move(_22)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_32 = dump_var(0_usize, 1_usize, Move(_1), 14_usize, Move(_14), 33_usize, _33, 33_usize, _33), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u128,mut _2: isize,mut _3: i128,mut _4: char,mut _5: u8,mut _6: char,mut _7: isize,mut _8: char,mut _9: isize) -> u16 {
mir! {
type RET = u16;
let _10: Adt45;
let _11: isize;
let _12: u8;
let _13: isize;
let _14: (i128, f32);
let _15: [i32; 2];
let _16: [char; 5];
let _17: (u8, u32);
let _18: (i16,);
let _19: f32;
let _20: *mut i32;
let _21: isize;
let _22: (i16,);
let _23: Adt47;
let _24: [u16; 2];
let _25: i32;
let _26: &'static u64;
let _27: (i16, f32);
let _28: i64;
let _29: bool;
let _30: bool;
let _31: u128;
let _32: u8;
let _33: isize;
let _34: usize;
let _35: isize;
let _36: *mut *mut i32;
let _37: (i128, f32);
let _38: f32;
let _39: [i32; 2];
let _40: isize;
let _41: *mut bool;
let _42: *const u128;
let _43: ();
let _44: ();
{
_4 = _8;
_9 = _2 | _7;
RET = 30763_u16 >> _3;
_7 = _9;
_10 = Adt45 { fld0: 3431002486328242462_usize,fld1: _1 };
_4 = _8;
_9 = _7;
_5 = true as u8;
_10 = Adt45 { fld0: 8460354557022005318_usize,fld1: _1 };
_10.fld0 = 7238772659293701617_u64 as usize;
RET = 61955_u16;
_1 = _10.fld1 ^ _10.fld1;
_4 = _6;
Call(_12 = fn2(_10.fld0, _3, _2, _10, _7, _7, _8, _7, _3, _2, _9, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = _6 as isize;
_6 = _8;
_7 = -_2;
_6 = _4;
_7 = (-16_i8) as isize;
_10.fld1 = _1 << _5;
_4 = _8;
_1 = _10.fld1;
_12 = _5;
_10 = Adt45 { fld0: 8392617401717838161_usize,fld1: _1 };
RET = 39923_u16;
_2 = _9;
_10 = Adt45 { fld0: 13454181382156648510_usize,fld1: _1 };
_8 = _4;
_13 = 1431435198075941853_u64 as isize;
RET = 47920_u16 + 54788_u16;
_5 = _12 * _12;
_10.fld0 = 4622175424673547724_usize;
_4 = _6;
_3 = (-81860026665037818730717263595220344055_i128) | 21252159143073301826815154546309828511_i128;
_10.fld0 = 6_usize;
_11 = -_9;
_10.fld1 = _1;
_11 = _13;
match _10.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
6 => bb7,
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
_3 = 103970342930760621071886537898762753584_i128 - (-12472663207589510124550664183518907151_i128);
_2 = _7 - _7;
_10.fld0 = _8 as usize;
_7 = _9;
_9 = -_7;
_1 = _10.fld1 - _10.fld1;
RET = 12909_u16;
_6 = _8;
_10.fld1 = !_1;
_15 = [(-279710378_i32),706095247_i32];
_16 = [_4,_4,_4,_4,_6];
_13 = _11;
_17.1 = !464710714_u32;
_18.0 = 30615_i16 + (-30713_i16);
_15 = [(-1475883034_i32),537490275_i32];
_7 = _2 ^ _9;
_14.1 = _10.fld0 as f32;
_16 = [_4,_8,_4,_4,_6];
_12 = _5;
_2 = _7 * _9;
Call(_13 = fn3(_1, _10.fld1, _7, _7, _4, _2, _2, _7, _10.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11 = _2;
_10 = Adt45 { fld0: 3_usize,fld1: _1 };
_4 = _6;
_14.0 = _3;
_22 = _18;
_22 = (_18.0,);
_22.0 = -_18.0;
_10.fld0 = false as usize;
_17 = (_5, 1066620045_u32);
_13 = _6 as isize;
_16 = [_8,_8,_8,_6,_8];
_7 = _9 ^ _11;
_6 = _4;
_17.0 = !_5;
_27 = (_22.0, _14.1);
_20 = core::ptr::addr_of_mut!(_25);
_25 = !2137662000_i32;
_14.0 = _3 >> (*_20);
_16 = [_8,_4,_4,_8,_8];
Call(_10.fld1 = fn4(_22, _7, _7, _4, _11, _11, _20, _7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13 = _11;
_24 = [RET,RET];
_13 = _14.1 as isize;
_1 = !_10.fld1;
_23 = Adt47::Variant2 { fld0: _22.0,fld1: _25 };
_2 = _7 + _7;
_17.1 = 3910619405_u32 >> _2;
SetDiscriminant(_23, 3);
_22.0 = -_18.0;
place!(Field::<i128>(Variant(_23, 3), 1)) = -_14.0;
_27.0 = _18.0;
_3 = _17.1 as i128;
place!(Field::<[u128; 8]>(Variant(_23, 3), 0)) = [_10.fld1,_10.fld1,_10.fld1,_10.fld1,_10.fld1,_1,_1,_1];
_17.1 = !1381866987_u32;
_2 = _11 * _7;
_3 = _25 as i128;
_8 = _6;
_10.fld1 = !_1;
Goto(bb10)
}
bb10 = {
_28 = (-5329793773415974429_i64);
_27 = (_18.0, _14.1);
_16 = [_6,_4,_8,_4,_4];
_17 = (_12, 2026870684_u32);
_15 = [_25,(*_20)];
_20 = core::ptr::addr_of_mut!(_25);
_29 = !false;
_10 = Adt45 { fld0: 3_usize,fld1: _1 };
_10.fld1 = _1 & _1;
_10.fld0 = 5_usize;
_12 = _5 & _5;
place!(Field::<i128>(Variant(_23, 3), 1)) = -_14.0;
_29 = !true;
_17.0 = _12;
_27 = (_22.0, _14.1);
_31 = _10.fld1;
match RET {
12909 => bb11,
_ => bb5
}
}
bb11 = {
_2 = _7 - _7;
_27 = (_22.0, _14.1);
_11 = _27.0 as isize;
_12 = _27.1 as u8;
_22.0 = _27.0;
_27.1 = -_14.1;
_27.0 = _18.0;
_33 = (*_20) as isize;
_33 = -_2;
_18 = (_27.0,);
_10 = Adt45 { fld0: 13354540661579064724_usize,fld1: _1 };
_28 = (-4087994742887211119_i64);
_24 = [RET,RET];
(*_20) = (-1300712204_i32);
(*_20) = (-1322672869_i32);
_15 = [_25,(*_20)];
SetDiscriminant(_23, 1);
match (*_20) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb6,
6 => bb10,
340282366920938463463374607430445538587 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_21 = _2 * _33;
_36 = core::ptr::addr_of_mut!(_20);
_14.0 = _27.1 as i128;
_18 = (_27.0,);
_35 = _17.1 as isize;
_37 = (_3, _14.1);
_37 = _14;
_37.0 = _3;
_8 = _6;
_17.1 = 3661634462_u32;
_16 = [_8,_8,_6,_8,_4];
_33 = _17.0 as isize;
_11 = -_2;
(*_20) = !(-1600969962_i32);
match _10.fld0 {
0 => bb4,
1 => bb6,
13354540661579064724 => bb14,
_ => bb8
}
}
bb14 = {
_18 = _22;
_2 = _21;
_10 = Adt45 { fld0: 2486233312727330343_usize,fld1: _1 };
_19 = -_27.1;
_38 = _37.1 * _37.1;
_8 = _4;
_7 = _21 & _21;
_39 = _15;
_2 = _7 | _21;
RET = _10.fld1 as u16;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(1_usize, 28_usize, Move(_28), 21_usize, Move(_21), 35_usize, Move(_35), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(1_usize, 8_usize, Move(_8), 24_usize, Move(_24), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(1_usize, 18_usize, Move(_18), 6_usize, Move(_6), 12_usize, Move(_12), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(1_usize, 11_usize, Move(_11), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: usize,mut _2: i128,mut _3: isize,mut _4: Adt45,mut _5: isize,mut _6: isize,mut _7: char,mut _8: isize,mut _9: i128,mut _10: isize,mut _11: isize,mut _12: isize) -> u8 {
mir! {
type RET = u8;
let _13: Adt48;
let _14: (i128, f32);
let _15: (u128, &'static u64, i32);
let _16: char;
let _17: i64;
let _18: Adt46;
let _19: [i16; 4];
let _20: i8;
let _21: isize;
let _22: isize;
let _23: [u128; 8];
let _24: [char; 5];
let _25: Adt45;
let _26: &'static u64;
let _27: isize;
let _28: u8;
let _29: Adt50;
let _30: f32;
let _31: [char; 8];
let _32: [u16; 2];
let _33: (i16,);
let _34: [char; 5];
let _35: char;
let _36: *mut bool;
let _37: isize;
let _38: isize;
let _39: i32;
let _40: Adt58;
let _41: isize;
let _42: [char; 5];
let _43: (i16, *mut usize);
let _44: ();
let _45: ();
{
_8 = !_10;
_1 = _4.fld0 >> _6;
_12 = _10 ^ _10;
_3 = -_6;
_14.1 = _5 as f32;
RET = !74_u8;
_11 = -_3;
_1 = (-9069834631505207789_i64) as usize;
_12 = true as isize;
_2 = true as i128;
RET = _1 as u8;
_4.fld0 = _1;
_14.1 = _2 as f32;
Goto(bb1)
}
bb1 = {
_14.1 = (-36_i8) as f32;
_15.0 = 15705_i16 as u128;
_7 = '\u{5994f}';
_14.0 = _9;
_15.0 = _4.fld1 | _4.fld1;
_16 = _7;
_11 = _6 ^ _3;
_17 = (-2916918558123438398_i64) & 8806538208938045905_i64;
_7 = _16;
_7 = _16;
_6 = _11;
_15.2 = -(-1449361210_i32);
_14.1 = (-73_i8) as f32;
_7 = _16;
_4 = Adt45 { fld0: _1,fld1: _15.0 };
_11 = _8 << _5;
_15.2 = _14.0 as i32;
_1 = _4.fld0;
_11 = _5;
Goto(bb2)
}
bb2 = {
_18.fld1.1 = -_14.1;
_18.fld2 = [(-13097_i16),(-30691_i16),(-31126_i16),12308_i16];
RET = !222_u8;
_18.fld3 = (RET, 1586197515_u32);
_15.0 = _4.fld0 as u128;
_7 = _16;
_2 = _14.0;
_18.fld0 = (-21182_i16) as f32;
match _18.fld3.1 {
0 => bb3,
1 => bb4,
2 => bb5,
1586197515 => bb7,
_ => bb6
}
}
bb3 = {
_14.1 = (-36_i8) as f32;
_15.0 = 15705_i16 as u128;
_7 = '\u{5994f}';
_14.0 = _9;
_15.0 = _4.fld1 | _4.fld1;
_16 = _7;
_11 = _6 ^ _3;
_17 = (-2916918558123438398_i64) & 8806538208938045905_i64;
_7 = _16;
_7 = _16;
_6 = _11;
_15.2 = -(-1449361210_i32);
_14.1 = (-73_i8) as f32;
_7 = _16;
_4 = Adt45 { fld0: _1,fld1: _15.0 };
_11 = _8 << _5;
_15.2 = _14.0 as i32;
_1 = _4.fld0;
_11 = _5;
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
_9 = !_2;
_18.fld1.0 = _2;
_19 = _18.fld2;
_18.fld1 = _14;
_18.fld3.0 = _15.2 as u8;
_16 = _7;
_14.0 = -_2;
_11 = -_6;
_15.0 = _4.fld1 << _6;
_24 = [_7,_7,_16,_16,_16];
_4.fld1 = _15.0;
_4.fld1 = _15.0;
_18.fld3.1 = 469955305_u32 + 2322326477_u32;
RET = _18.fld3.0 & _18.fld3.0;
_10 = _6;
_18.fld1.1 = _10 as f32;
_18.fld0 = _18.fld1.1;
_4.fld1 = _15.0 - _15.0;
_6 = -_8;
_14.0 = _18.fld1.0;
_4.fld0 = _1 >> _3;
_18.fld3.0 = !RET;
_9 = _2 - _2;
_11 = _3;
Call(_18.fld3.1 = core::intrinsics::transmute(_7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_8 = core::intrinsics::transmute(_10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18.fld3.0 = _18.fld1.1 as u8;
_18.fld0 = _18.fld1.1;
_18.fld3 = (RET, 2365942423_u32);
_19 = [(-12506_i16),(-21599_i16),(-3051_i16),18601_i16];
_2 = !_9;
_29.fld0.1 = -_18.fld0;
_16 = _7;
_14 = (_18.fld1.0, _18.fld1.1);
_21 = _4.fld1 as isize;
_18.fld1.1 = _18.fld0 + _14.1;
_24 = [_7,_16,_7,_7,_7];
_18.fld1.1 = _29.fld0.1 - _29.fld0.1;
_18.fld3 = (RET, 2483869542_u32);
_20 = _18.fld3.0 as i8;
_7 = _16;
_1 = _4.fld0 ^ _4.fld0;
_14.0 = !_2;
_25.fld0 = _20 as usize;
Goto(bb10)
}
bb10 = {
_27 = _21;
_18.fld3.1 = !1436782662_u32;
RET = _7 as u8;
_2 = _18.fld1.0;
_12 = !_3;
_32 = [18204_u16,14583_u16];
_29.fld2 = [_18.fld3.0,RET,_18.fld3.0,_18.fld3.0,_18.fld3.0];
_18.fld1 = (_2, _14.1);
_18.fld1 = _14;
_25 = Adt45 { fld0: _1,fld1: _4.fld1 };
_29.fld0 = (_9, _14.1);
_14 = _29.fld0;
_25.fld0 = !_1;
_18.fld1.0 = _14.0;
_29.fld0.0 = _20 as i128;
Goto(bb11)
}
bb11 = {
_33 = ((-23828_i16),);
_32 = [41028_u16,50606_u16];
_22 = -_27;
_28 = RET ^ RET;
_14 = (_18.fld1.0, _29.fld0.1);
_4.fld0 = _25.fld0 + _1;
_2 = -_18.fld1.0;
_33 = (30147_i16,);
_4.fld1 = _25.fld1;
_3 = _27;
_18.fld1 = (_9, _14.1);
_35 = _16;
_10 = _22;
_17 = _33.0 as i64;
_35 = _7;
_29.fld1 = [_35,_16,_35,_35,_35];
_15.2 = _2 as i32;
_33 = (5105_i16,);
_33 = ((-18233_i16),);
RET = _7 as u8;
_20 = 35_i8;
_18.fld1 = _29.fld0;
match _20 {
0 => bb8,
1 => bb10,
2 => bb4,
3 => bb12,
35 => bb14,
_ => bb13
}
}
bb12 = {
_27 = _21;
_18.fld3.1 = !1436782662_u32;
RET = _7 as u8;
_2 = _18.fld1.0;
_12 = !_3;
_32 = [18204_u16,14583_u16];
_29.fld2 = [_18.fld3.0,RET,_18.fld3.0,_18.fld3.0,_18.fld3.0];
_18.fld1 = (_2, _14.1);
_18.fld1 = _14;
_25 = Adt45 { fld0: _1,fld1: _4.fld1 };
_29.fld0 = (_9, _14.1);
_14 = _29.fld0;
_25.fld0 = !_1;
_18.fld1.0 = _14.0;
_29.fld0.0 = _20 as i128;
Goto(bb11)
}
bb13 = {
Call(_8 = core::intrinsics::transmute(_10), ReturnTo(bb9), UnwindUnreachable())
}
bb14 = {
_4 = Adt45 { fld0: _25.fld0,fld1: _15.0 };
_18.fld3.1 = 640851845_u32;
_42 = [_7,_7,_16,_35,_7];
_15.0 = _25.fld1;
_15.2 = (-1636117777_i32) + 927299282_i32;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(2_usize, 1_usize, Move(_1), 17_usize, Move(_17), 20_usize, Move(_20), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(2_usize, 11_usize, Move(_11), 35_usize, Move(_35), 33_usize, Move(_33), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(2_usize, 22_usize, Move(_22), 3_usize, Move(_3), 19_usize, Move(_19), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: u128,mut _2: u128,mut _3: isize,mut _4: isize,mut _5: char,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: usize) -> isize {
mir! {
type RET = isize;
let _10: *mut bool;
let _11: isize;
let _12: i128;
let _13: Adt44;
let _14: u64;
let _15: isize;
let _16: (i16, *mut usize);
let _17: bool;
let _18: f64;
let _19: isize;
let _20: Adt44;
let _21: char;
let _22: isize;
let _23: f32;
let _24: usize;
let _25: (i16,);
let _26: f32;
let _27: ();
let _28: ();
{
RET = !_6;
_5 = '\u{f9fb9}';
_4 = 27_i8 as isize;
_5 = '\u{b56ac}';
_7 = _8 ^ _6;
RET = !_8;
_4 = 5944612201977462342_u64 as isize;
RET = 142_u8 as isize;
_7 = _8;
_5 = '\u{fe756}';
RET = -_8;
_9 = 1218345841829637829_usize << _6;
_2 = !_1;
_4 = !_6;
_9 = 5_usize * 8164078144070753648_usize;
_2 = 234_u8 as u128;
_5 = '\u{5d22}';
_5 = '\u{4a3cd}';
_2 = _1;
Call(_6 = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = RET as u128;
_9 = !7_usize;
Goto(bb2)
}
bb2 = {
_3 = -RET;
RET = _8 * _4;
_11 = _6;
RET = (-140293664850864356098586882467329476311_i128) as isize;
_3 = 158_u8 as isize;
_11 = _4 + _8;
_12 = !(-121604170708765693069815226684341121190_i128);
_12 = 36257465942818843744164962108090709959_i128 | (-142719532899462685844855883420378552191_i128);
_12 = 25417369140025108458529301723815803422_i128 - 153076723010377598473362293307462278564_i128;
_11 = 45_u8 as isize;
_9 = 12661420846118466144_usize * 5_usize;
_5 = '\u{101680}';
_12 = !(-70558327098620677243780001004849934022_i128);
_4 = -_7;
_3 = 477717997043770524_i64 as isize;
_14 = !17726039612686368913_u64;
_16.0 = !9348_i16;
_5 = '\u{fe666}';
_11 = -_6;
_9 = !2867964847942247239_usize;
_7 = _6;
Goto(bb3)
}
bb3 = {
_8 = _7;
_16.0 = 17483_i16 * (-8980_i16);
_16.1 = core::ptr::addr_of_mut!(_9);
_2 = _1 * _1;
_19 = 5700_u16 as isize;
_5 = '\u{cdfba}';
_4 = (-1418679268852745786_i64) as isize;
_16.0 = (-20282_i16) ^ 25618_i16;
_18 = _12 as f64;
_17 = true;
_15 = _11;
_16.0 = (-40_i8) as i16;
_14 = _12 as u64;
Goto(bb4)
}
bb4 = {
_22 = 1935305060_i32 as isize;
_16.0 = _9 as i16;
_19 = _16.0 as isize;
_2 = _14 as u128;
_18 = _12 as f64;
RET = _6 + _11;
_12 = -(-1808400031496384727705088485691616832_i128);
_22 = _3 & _11;
_21 = _5;
_2 = _1;
_24 = !_9;
_6 = _15;
RET = _1 as isize;
_23 = 73_i8 as f32;
_3 = -_11;
_15 = -_6;
_7 = (-104_i8) as isize;
_8 = !_11;
Goto(bb5)
}
bb5 = {
_14 = 7326113049408716190_u64 + 4806234091313200265_u64;
_24 = !_9;
RET = _22 * _3;
_2 = !_1;
_6 = _16.0 as isize;
_14 = 6439709989140880266_u64;
_22 = _12 as isize;
_18 = _9 as f64;
_9 = _18 as usize;
_23 = 178_u8 as f32;
_7 = (-761346091_i32) as isize;
_1 = !_2;
_21 = _5;
_16.1 = core::ptr::addr_of_mut!(_9);
_3 = _8 ^ _11;
_22 = _9 as isize;
_7 = !_15;
_22 = !_3;
_26 = _23;
_25.0 = _16.0;
_24 = _9 - _9;
_26 = _23;
Goto(bb6)
}
bb6 = {
Call(_27 = dump_var(3_usize, 15_usize, Move(_15), 12_usize, Move(_12), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_27 = dump_var(3_usize, 6_usize, Move(_6), 8_usize, Move(_8), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_27 = dump_var(3_usize, 3_usize, Move(_3), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (i16,),mut _2: isize,mut _3: isize,mut _4: char,mut _5: isize,mut _6: isize,mut _7: *mut i32,mut _8: isize) -> u128 {
mir! {
type RET = u128;
let _9: u16;
let _10: [i128; 7];
let _11: i8;
let _12: isize;
let _13: isize;
let _14: u32;
let _15: u64;
let _16: Adt58;
let _17: u16;
let _18: bool;
let _19: char;
let _20: u16;
let _21: bool;
let _22: [u128; 8];
let _23: (i16,);
let _24: [i32; 2];
let _25: Adt52;
let _26: isize;
let _27: Adt56;
let _28: f32;
let _29: [i32; 2];
let _30: i128;
let _31: (i128, f32);
let _32: u16;
let _33: [u8; 5];
let _34: char;
let _35: isize;
let _36: ();
let _37: ();
{
_5 = _3;
RET = 141771745922530800129416810183077854373_u128 + 196052624401568853458758293933108036665_u128;
_4 = '\u{3b5ee}';
(*_7) = !(-1136927562_i32);
_8 = -_6;
_8 = _3;
_2 = 39879308962527576297018231475478845488_i128 as isize;
(*_7) = !(-1451648294_i32);
_5 = -_6;
(*_7) = !1894935431_i32;
RET = 3928085629_u32 as u128;
_10 = [151687076987762579205301642765893856484_i128,(-77117079064056329072031733581029946424_i128),94361698681289273435996431472991423293_i128,150594900251534873088158874431252306791_i128,(-98078889896889205485030649870488735950_i128),(-78274471178033578141830629639096335601_i128),81044541713114935869052126326630035877_i128];
Goto(bb1)
}
bb1 = {
_1 = ((-15897_i16),);
RET = 67651212033151958992456949486818032947_u128 | 92130017803387256091730516605720162500_u128;
_9 = !24866_u16;
_10 = [(-33873689190695423842868612539482933744_i128),105832865239974904604434775904141857444_i128,123497606232551378778068984295587138162_i128,(-51698496927072384623186024069856033799_i128),89984950274505471166661107547980973759_i128,(-19523303990693157902243542173269386066_i128),34842958344211072568430911978354822784_i128];
RET = 139547251035485576114875293403581992355_u128;
_1.0 = (-31710_i16);
_10 = [(-87653421219692330333333411130409408206_i128),151111967111373678465367880345667330162_i128,37099496077858077629281797894421808636_i128,(-41944153547765921963803575802241335711_i128),(-117245803396888010842697668854979378590_i128),134182664620813738440496144352905911665_i128,(-88984700908667216396670201639540140426_i128)];
(*_7) = (-94039318662150322897033044814690663333_i128) as i32;
_2 = -_3;
_4 = '\u{23437}';
_9 = 15559_u16;
(*_7) = (-774092645_i32);
_1 = (8815_i16,);
_10 = [(-26970937280433133291334793344823222226_i128),(-21941222331457077754167222978477914089_i128),37453666970690143147742726435727167426_i128,128712195048845757156005615470320368484_i128,(-11110312143534728533289543376366196765_i128),131808394409350423405947738617509523862_i128,99251843013942722111144017457852773915_i128];
RET = 269871902815371632525026967289836712155_u128;
_9 = !11759_u16;
_12 = -_6;
_11 = -116_i8;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _5;
_13 = !_8;
_6 = _3;
_14 = _11 as u32;
_3 = _2 << _2;
_11 = 115_i8 - (-81_i8);
RET = !107301151939555849280704183366257679453_u128;
_9 = 1346_u16 << _3;
_5 = 239_u8 as isize;
_8 = _3;
_9 = 31441_u16;
_5 = _8 | _12;
_13 = 222_u8 as isize;
(*_7) = 542813035_i32 * 1943993469_i32;
_6 = _2 + _5;
RET = 208163483587917639465091739823047908094_u128;
RET = 189969984474720246622464511635664760742_u128;
_8 = _12 | _3;
_5 = _2;
_13 = _1.0 as isize;
RET = 217557923593370041334459232022821409361_u128;
_13 = !_2;
_12 = _3;
_3 = _6;
_1 = (26420_i16,);
_2 = _13 + _6;
_11 = 16_i8 * (-111_i8);
_15 = _14 as u64;
_1.0 = (-21510_i16) >> _3;
match _9 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
31441 => bb10,
_ => bb9
}
}
bb3 = {
_1 = ((-15897_i16),);
RET = 67651212033151958992456949486818032947_u128 | 92130017803387256091730516605720162500_u128;
_9 = !24866_u16;
_10 = [(-33873689190695423842868612539482933744_i128),105832865239974904604434775904141857444_i128,123497606232551378778068984295587138162_i128,(-51698496927072384623186024069856033799_i128),89984950274505471166661107547980973759_i128,(-19523303990693157902243542173269386066_i128),34842958344211072568430911978354822784_i128];
RET = 139547251035485576114875293403581992355_u128;
_1.0 = (-31710_i16);
_10 = [(-87653421219692330333333411130409408206_i128),151111967111373678465367880345667330162_i128,37099496077858077629281797894421808636_i128,(-41944153547765921963803575802241335711_i128),(-117245803396888010842697668854979378590_i128),134182664620813738440496144352905911665_i128,(-88984700908667216396670201639540140426_i128)];
(*_7) = (-94039318662150322897033044814690663333_i128) as i32;
_2 = -_3;
_4 = '\u{23437}';
_9 = 15559_u16;
(*_7) = (-774092645_i32);
_1 = (8815_i16,);
_10 = [(-26970937280433133291334793344823222226_i128),(-21941222331457077754167222978477914089_i128),37453666970690143147742726435727167426_i128,128712195048845757156005615470320368484_i128,(-11110312143534728533289543376366196765_i128),131808394409350423405947738617509523862_i128,99251843013942722111144017457852773915_i128];
RET = 269871902815371632525026967289836712155_u128;
_9 = !11759_u16;
_12 = -_6;
_11 = -116_i8;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
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
(*_7) = _6 as i32;
_2 = (*_7) as isize;
_8 = !_6;
RET = 227_u8 as u128;
_17 = _9;
_1.0 = (-23180_i16) * 20184_i16;
_1.0 = RET as i16;
Goto(bb11)
}
bb11 = {
_4 = '\u{8da16}';
_1.0 = (-18628_i16);
_19 = _4;
_17 = _9 >> (*_7);
(*_7) = 1111422444_i32 >> _9;
_18 = false;
_8 = _15 as isize;
_11 = -(-43_i8);
_5 = !_2;
_3 = _12 ^ _6;
_18 = false ^ true;
_12 = _5;
_18 = !false;
_11 = -(-114_i8);
RET = 283000281658626483355960630726802791624_u128 | 63582975460820990643103671808243020220_u128;
_3 = RET as isize;
_21 = _18;
_2 = !_12;
Call(_20 = fn5(_6, _12, _17, _5, _4, _12, _2, _2, _5, _5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_19 = _4;
_5 = !_6;
_23 = (_1.0,);
_23.0 = -_1.0;
_8 = _18 as isize;
_9 = 204_u8 as u16;
_3 = _18 as isize;
_17 = _9;
_23.0 = -_1.0;
_15 = 11786449248050148427_u64;
_4 = _19;
_18 = _21;
_5 = !_12;
_25 = Adt52::Variant2 { fld0: (-1578666764053985753_i64) };
_3 = 95038682175920068086134617935744700784_i128 as isize;
place!(Field::<i64>(Variant(_25, 2), 0)) = _14 as i64;
_13 = _2;
_23.0 = !_1.0;
Goto(bb13)
}
bb13 = {
_24 = [(*_7),(*_7)];
_26 = _13;
_18 = _5 >= _26;
_11 = _14 as i8;
_26 = _18 as isize;
_9 = _19 as u16;
RET = !79368589075290059250738980065866424643_u128;
(*_7) = -(-1857330684_i32);
_21 = _6 == _2;
(*_7) = (-82603254_i32) * (-1045052994_i32);
_5 = _19 as isize;
_28 = 4_usize as f32;
(*_7) = (-22556138521936018450430874981253166802_i128) as i32;
_13 = _5;
_4 = _19;
_23 = _1;
_12 = _6;
SetDiscriminant(_25, 1);
_32 = !_20;
_1.0 = -_23.0;
_6 = _12;
_26 = _2 | _6;
_13 = _6;
_31 = ((-59084693666990938318275028887121589253_i128), _28);
_29 = [(*_7),(*_7)];
Call(_9 = fn7(_12, _21, _26, _21, _6, _13, _13, _18, _26, _6, _6, _21, _26, _13), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = !21728752434337132659208041417133032012_u128;
_13 = !_26;
_24 = _29;
place!(Field::<(i16, *mut usize)>(Variant(_25, 1), 2)).0 = _1.0;
_30 = _31.0 ^ _31.0;
_2 = _26;
_5 = -_6;
place!(Field::<(i128, f32)>(Variant(_25, 1), 6)).0 = -_31.0;
place!(Field::<(i128, f32)>(Variant(_25, 1), 6)).1 = _31.0 as f32;
_13 = _26;
place!(Field::<i32>(Variant(_25, 1), 5)) = (*_7) << _9;
_14 = 3070144393_u32;
RET = 320089331453683606066453647217616058058_u128;
_29 = [Field::<i32>(Variant(_25, 1), 5),Field::<i32>(Variant(_25, 1), 5)];
_15 = 17704602140618747963_u64 | 10503725751553395875_u64;
_24 = _29;
place!(Field::<i32>(Variant(_25, 1), 5)) = (*_7) & (*_7);
_31.0 = -_30;
_35 = -_12;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(4_usize, 4_usize, Move(_4), 12_usize, Move(_12), 21_usize, Move(_21), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(4_usize, 13_usize, Move(_13), 11_usize, Move(_11), 29_usize, Move(_29), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(4_usize, 24_usize, Move(_24), 2_usize, Move(_2), 18_usize, Move(_18), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(4_usize, 32_usize, Move(_32), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: isize,mut _3: u16,mut _4: isize,mut _5: char,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> u16 {
mir! {
type RET = u16;
let _11: f64;
let _12: isize;
let _13: Adt49;
let _14: *mut *mut i32;
let _15: isize;
let _16: usize;
let _17: f64;
let _18: [u16; 2];
let _19: u64;
let _20: *const (u8, u32);
let _21: [u128; 8];
let _22: ();
let _23: ();
{
RET = _3;
_6 = _7;
_2 = _6;
_2 = _9 & _9;
_9 = (-3479_i16) as isize;
RET = _3;
_11 = (-8625664210823376963_i64) as f64;
_2 = _1;
_4 = RET as isize;
_6 = -_8;
_11 = 3968192806102864827_usize as f64;
_6 = _8 & _4;
_1 = 1969476773_i32 as isize;
_6 = _11 as isize;
_8 = _7;
_12 = _10;
RET = 9230278234615608537_u64 as u16;
_5 = '\u{f8a95}';
_6 = _10;
RET = 66_i8 as u16;
Goto(bb1)
}
bb1 = {
_4 = _3 as isize;
_8 = _10;
_13 = Adt49::Variant0 { fld0: 237_u8,fld1: _11 };
_4 = _8 | _12;
_5 = '\u{e0375}';
place!(Field::<u8>(Variant(_13, 0), 0)) = 69_u8 - 104_u8;
_2 = -_6;
_2 = (-9815_i16) as isize;
_10 = _4 & _12;
place!(Field::<f64>(Variant(_13, 0), 1)) = _11 - _11;
_11 = -Field::<f64>(Variant(_13, 0), 1);
_10 = _3 as isize;
_2 = _4 * _6;
_6 = _7;
Goto(bb2)
}
bb2 = {
_11 = Field::<f64>(Variant(_13, 0), 1);
_5 = '\u{e7170}';
RET = _3;
_2 = (-8508852917846484349966520655694061433_i128) as isize;
SetDiscriminant(_13, 0);
RET = 236_u8 as u16;
_7 = _4;
_8 = _7;
_13 = Adt49::Variant0 { fld0: 88_u8,fld1: _11 };
_13 = Adt49::Variant0 { fld0: 82_u8,fld1: _11 };
_15 = 3029157101508812244_u64 as isize;
Goto(bb3)
}
bb3 = {
_12 = -_7;
_1 = _10;
Call(_4 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13 = Adt49::Variant0 { fld0: 10_u8,fld1: _11 };
place!(Field::<u8>(Variant(_13, 0), 0)) = 89_u8;
_11 = 459834443_u32 as f64;
_10 = -_1;
SetDiscriminant(_13, 0);
_5 = '\u{2d57b}';
RET = 155342825054447319947372623434848208965_u128 as u16;
place!(Field::<u8>(Variant(_13, 0), 0)) = !174_u8;
place!(Field::<f64>(Variant(_13, 0), 1)) = -_11;
_13 = Adt49::Variant0 { fld0: 169_u8,fld1: _11 };
_15 = -_10;
_18 = [_3,_3];
_18 = [_3,_3];
_2 = !_7;
_8 = -_2;
_16 = 6_usize;
_12 = _6 | _2;
_5 = '\u{c9809}';
_12 = !_2;
_8 = !_12;
_8 = _12 | _4;
_13 = Adt49::Variant0 { fld0: 174_u8,fld1: _11 };
match _16 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
6 => bb9,
_ => bb8
}
}
bb5 = {
_12 = -_7;
_1 = _10;
Call(_4 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_11 = Field::<f64>(Variant(_13, 0), 1);
_5 = '\u{e7170}';
RET = _3;
_2 = (-8508852917846484349966520655694061433_i128) as isize;
SetDiscriminant(_13, 0);
RET = 236_u8 as u16;
_7 = _4;
_8 = _7;
_13 = Adt49::Variant0 { fld0: 88_u8,fld1: _11 };
_13 = Adt49::Variant0 { fld0: 82_u8,fld1: _11 };
_15 = 3029157101508812244_u64 as isize;
Goto(bb3)
}
bb7 = {
_4 = _3 as isize;
_8 = _10;
_13 = Adt49::Variant0 { fld0: 237_u8,fld1: _11 };
_4 = _8 | _12;
_5 = '\u{e0375}';
place!(Field::<u8>(Variant(_13, 0), 0)) = 69_u8 - 104_u8;
_2 = -_6;
_2 = (-9815_i16) as isize;
_10 = _4 & _12;
place!(Field::<f64>(Variant(_13, 0), 1)) = _11 - _11;
_11 = -Field::<f64>(Variant(_13, 0), 1);
_10 = _3 as isize;
_2 = _4 * _6;
_6 = _7;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
place!(Field::<f64>(Variant(_13, 0), 1)) = _3 as f64;
_8 = _1 * _1;
_16 = 1_usize << _12;
_6 = _2 >> _16;
_17 = Field::<f64>(Variant(_13, 0), 1);
Goto(bb10)
}
bb10 = {
_17 = -Field::<f64>(Variant(_13, 0), 1);
Call(_2 = fn6(_16, _12, _12, _18, Field::<f64>(Variant(_13, 0), 1), _8, _15, _7, _12, _12, _17, _4, _18, _4, _10, _8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_5 = '\u{2c02b}';
_17 = -Field::<f64>(Variant(_13, 0), 1);
_10 = -_2;
Goto(bb12)
}
bb12 = {
_12 = _6;
_15 = !_10;
_11 = (-110204253447307200523305399451948323940_i128) as f64;
_7 = _1;
place!(Field::<f64>(Variant(_13, 0), 1)) = _17;
_9 = _15;
_8 = _7 | _9;
_7 = !_4;
_7 = !_6;
_11 = -_17;
place!(Field::<u8>(Variant(_13, 0), 0)) = 230_u8 - 93_u8;
_16 = 5_usize + 1_usize;
Goto(bb13)
}
bb13 = {
_10 = _12 & _15;
_5 = '\u{add98}';
Goto(bb14)
}
bb14 = {
place!(Field::<f64>(Variant(_13, 0), 1)) = -_17;
RET = 67584857361918474615134334890673180551_i128 as u16;
_11 = (-13792_i16) as f64;
_8 = _6;
_6 = _4;
_17 = -Field::<f64>(Variant(_13, 0), 1);
_19 = 14345688754724320161_u64;
_12 = _15;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(5_usize, 10_usize, Move(_10), 19_usize, Move(_19), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(5_usize, 8_usize, Move(_8), 7_usize, Move(_7), 2_usize, Move(_2), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: usize,mut _2: isize,mut _3: isize,mut _4: [u16; 2],mut _5: f64,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: f64,mut _12: isize,mut _13: [u16; 2],mut _14: isize,mut _15: isize,mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: u8;
let _18: isize;
let _19: Adt44;
let _20: ();
let _21: ();
{
_15 = _16 ^ _8;
Goto(bb1)
}
bb1 = {
_12 = _15;
_16 = 63_i8 as isize;
_10 = _14 >> _15;
_2 = _12;
RET = -_9;
_6 = -_9;
_18 = 1115592541594922471_i64 as isize;
RET = _9 ^ _9;
_15 = !_9;
_7 = _2;
_15 = _3 & RET;
_11 = 14505472981409456365_u64 as f64;
_6 = _2 << _7;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(6_usize, 14_usize, Move(_14), 7_usize, Move(_7), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(6_usize, 18_usize, Move(_18), 12_usize, Move(_12), 10_usize, Move(_10), 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: bool,mut _3: isize,mut _4: bool,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: bool,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: bool,mut _13: isize,mut _14: isize) -> u16 {
mir! {
type RET = u16;
let _15: i32;
let _16: [i32; 2];
let _17: [i16; 4];
let _18: [u16; 2];
let _19: (i16, f32);
let _20: [i128; 7];
let _21: isize;
let _22: [i128; 7];
let _23: Adt44;
let _24: (i16, f32);
let _25: i16;
let _26: [i32; 2];
let _27: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64);
let _28: i64;
let _29: [u16; 2];
let _30: i32;
let _31: [char; 5];
let _32: i16;
let _33: *mut bool;
let _34: (u128, &'static u64, i32);
let _35: [char; 5];
let _36: Adt49;
let _37: *const (u8, u32);
let _38: i128;
let _39: (u16, u16, i8, [i16; 4], (*mut usize, u16, i128));
let _40: [char; 8];
let _41: f32;
let _42: Adt46;
let _43: *mut i128;
let _44: ();
let _45: ();
{
_9 = _3 - _5;
Goto(bb1)
}
bb1 = {
_2 = _9 == _13;
_14 = (-1772525815_i32) as isize;
_2 = !_8;
_4 = _9 > _9;
_12 = _8 | _8;
_12 = _2 & _8;
_5 = _3 | _6;
_6 = !_3;
RET = !36546_u16;
_13 = 237_u8 as isize;
_2 = !_8;
RET = 6867_u16 | 3732_u16;
_9 = _6;
_13 = 3101310544_u32 as isize;
_12 = _2;
_8 = _12;
_15 = (-599209408_i32) | (-53413454_i32);
_16 = [_15,_15];
Call(_7 = fn8(_12, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _6 != _7;
_3 = _7;
_13 = (-13_i8) as isize;
_10 = !_7;
_7 = _9 >> _9;
Goto(bb3)
}
bb3 = {
_5 = 2854594897_u32 as isize;
_17 = [4052_i16,(-21342_i16),15874_i16,16361_i16];
_4 = _2 & _2;
_13 = 6712589282515251203_i64 as isize;
_6 = !_9;
_16 = [_15,_15];
_12 = _2 > _4;
_9 = '\u{fe373}' as isize;
_15 = -1699598517_i32;
_1 = 10188616301035165604_u64 as isize;
_11 = !_10;
_1 = _11 | _3;
_17 = [26996_i16,96_i16,21321_i16,27111_i16];
_1 = _8 as isize;
_19.1 = 4161207116_u32 as f32;
Goto(bb4)
}
bb4 = {
_5 = 8685781788738458824_i64 as isize;
_15 = 1415716536_i32 << _6;
_6 = _1;
_19.0 = !(-13473_i16);
_1 = _19.1 as isize;
_12 = _4;
_8 = _4;
_18 = [RET,RET];
_14 = 6323957301993730942_i64 as isize;
_18 = [RET,RET];
_20 = [(-100311759370449286876796086198347998086_i128),(-62711425455012476598142109373196479797_i128),(-11019649477999915741670637343365689601_i128),55369456225542110256653532526381833591_i128,(-113413510138227967356178864452696152655_i128),71540678994936982100087912303239458126_i128,(-65513192742824694483343819386386584916_i128)];
_19.1 = (-58365774279505258310719528241676001750_i128) as f32;
Goto(bb5)
}
bb5 = {
_21 = _19.1 as isize;
_19.1 = 101_i8 as f32;
_20 = [(-23902395099250145546871439468437914493_i128),(-7525208806860583760868248117896680967_i128),18931343452283550898542201735000155868_i128,(-39326845872006421143927328899117206337_i128),46528110277602394019650995482976247216_i128,124231021942957199828056099858914992802_i128,(-74092659799552061850999794955153648305_i128)];
_19.1 = 81_i8 as f32;
Goto(bb6)
}
bb6 = {
_15 = 7831319845696010759_u64 as i32;
_3 = !_10;
_3 = -_7;
Call(_17 = core::intrinsics::transmute(_7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = !_8;
_11 = 15900747215726008564_usize as isize;
_5 = _6;
_19.0 = -(-5566_i16);
_13 = _10;
_14 = _10;
_26 = _16;
_25 = !_19.0;
_7 = !_13;
_22 = _20;
_24.1 = 0_usize as f32;
_10 = _13 - _6;
_11 = _6 + _7;
_19.1 = _24.1 + _24.1;
_1 = -_7;
_9 = _1;
Call(_3 = core::intrinsics::transmute(_7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = _12;
_13 = _8 as isize;
_16 = _26;
_10 = _3;
_19.0 = _2 as i16;
_18 = [RET,RET];
_5 = _13;
_27.1 = ['\u{f64b1}','\u{180e7}','\u{e0fcc}','\u{d84dd}','\u{1d299}','\u{49a}','\u{56d38}','\u{d566f}'];
_6 = !_10;
Goto(bb9)
}
bb9 = {
_24.1 = _19.1 + _19.1;
_19.1 = _24.1;
_27.2.0 = _19.0;
_10 = _13;
_6 = _3 << _5;
_10 = _11 + _1;
RET = 40487_u16 + 50520_u16;
_19.0 = _27.2.0;
_28 = _27.2.0 as i64;
_32 = _27.2.0;
_21 = _19.1 as isize;
_10 = _11;
_5 = _28 as isize;
_33 = core::ptr::addr_of_mut!(_8);
_24.1 = _19.1;
_10 = _21;
_19.0 = !_32;
_11 = (-1796880336172707798134109794608882893_i128) as isize;
_34.0 = 172_u8 as u128;
Goto(bb10)
}
bb10 = {
_17 = [_32,_32,_27.2.0,_32];
_27.0 = core::ptr::addr_of_mut!(_34.2);
Goto(bb11)
}
bb11 = {
_39.0 = RET;
_35 = ['\u{8ec1f}','\u{cd926}','\u{e26d2}','\u{4d1f1}','\u{55c0c}'];
_16 = [_15,_15];
_16 = [_15,_15];
_39.1 = _15 as u16;
_8 = _1 <= _6;
_17 = [_27.2.0,_27.2.0,_27.2.0,_32];
_27.1 = ['\u{3ec0c}','\u{aaf01}','\u{11af1}','\u{4ac71}','\u{4ab68}','\u{107924}','\u{8a490}','\u{13ff0}'];
_27.3.1 = _28 as u32;
_16 = [_15,_15];
_24 = _19;
_34.0 = _39.0 as u128;
_39.1 = _39.0 >> _19.0;
_8 = !_4;
_25 = 140796278632572779291298125393546515135_i128 as i16;
_39.3 = [_32,_27.2.0,_24.0,_27.2.0];
_37 = core::ptr::addr_of!(_27.3);
Call((*_37) = fn9(_9, _24.0, _39.3, _5, _6, _13, _24, _37, _39.3, _5, _39.1, _27.2.0, _4, _37), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_33 = core::ptr::addr_of_mut!(_2);
Call(_39.2 = fn11(_3, _24.0, _5, (*_37), _6, _6), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_39.2 = '\u{574cd}' as i8;
_8 = !(*_33);
(*_33) = _39.1 < RET;
RET = _39.1 ^ _39.1;
RET = _39.1 - _39.1;
(*_37) = (253_u8, 1914394548_u32);
_10 = _9;
_42.fld1.0 = RET as i128;
_38 = (*_37).0 as i128;
Goto(bb14)
}
bb14 = {
Call(_44 = dump_var(7_usize, 28_usize, Move(_28), 9_usize, Move(_9), 16_usize, Move(_16), 22_usize, Move(_22)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_44 = dump_var(7_usize, 10_usize, Move(_10), 21_usize, Move(_21), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(7_usize, 5_usize, Move(_5), 13_usize, Move(_13), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(7_usize, 25_usize, Move(_25), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: bool,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: ();
let _4: ();
{
RET = _2;
RET = _2 - _2;
RET = !_2;
RET = _2 * _2;
_2 = RET;
Goto(bb1)
}
bb1 = {
Call(_3 = dump_var(8_usize, 2_usize, Move(_2), 4_usize, _4, 4_usize, _4, 4_usize, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: i16,mut _3: [i16; 4],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: (i16, f32),mut _8: *const (u8, u32),mut _9: [i16; 4],mut _10: isize,mut _11: u16,mut _12: i16,mut _13: bool,mut _14: *const (u8, u32)) -> (u8, u32) {
mir! {
type RET = (u8, u32);
let _15: *const u128;
let _16: Adt46;
let _17: *mut i32;
let _18: (i128, f32);
let _19: [u8; 5];
let _20: (i16, f32);
let _21: i16;
let _22: isize;
let _23: isize;
let _24: f32;
let _25: (u8, u32);
let _26: u16;
let _27: bool;
let _28: ();
let _29: ();
{
RET.1 = 2535008414_u32 & 1026636045_u32;
_2 = _12 ^ _12;
_11 = 20272_u16;
RET = (32_u8, 3122566484_u32);
Goto(bb1)
}
bb1 = {
RET.0 = 30_i8 as u8;
RET = (248_u8, 3736370784_u32);
_9 = [_12,_2,_2,_12];
_4 = _11 as isize;
_8 = _14;
RET = (113_u8, 714706764_u32);
RET = (75_u8, 1813142316_u32);
RET.0 = !146_u8;
RET.1 = 695159968_u32 >> _7.0;
RET.1 = 3565543355463819557_u64 as u32;
_12 = _2;
_7.1 = 1533820798_i32 as f32;
_16.fld0 = -_7.1;
_16.fld1 = ((-153681702140100720265564263516572191085_i128), _16.fld0);
_3 = [_7.0,_7.0,_12,_12];
_16.fld2 = [_7.0,_12,_12,_12];
Goto(bb2)
}
bb2 = {
RET.0 = 243_u8;
match RET.0 {
243 => bb3,
_ => bb1
}
}
bb3 = {
_9 = _16.fld2;
_6 = '\u{109fc1}' as isize;
_2 = !_7.0;
RET = (156_u8, 2576080520_u32);
_4 = _10 ^ _10;
RET = (248_u8, 1576892587_u32);
_1 = _7.1 as isize;
_10 = _5 << _12;
_20.1 = _7.1;
_7 = (_12, _16.fld0);
RET = (154_u8, 3850688045_u32);
RET.1 = 1303559830_u32;
_20.1 = 13942243567566432806_usize as f32;
_6 = -_10;
_16.fld0 = _6 as f32;
_20.0 = !_7.0;
_14 = _8;
_16.fld2 = [_20.0,_7.0,_12,_2];
_16.fld3.0 = 6305797570896030907_usize as u8;
_16.fld3 = (RET.0, RET.1);
_2 = -_12;
Goto(bb4)
}
bb4 = {
_10 = _5 + _5;
_20 = (_7.0, _16.fld0);
_18 = (_16.fld1.0, _16.fld0);
RET.1 = (-1897345447_i32) as u32;
_21 = _2;
_22 = -_4;
_5 = _4 >> _12;
_20 = (_12, _18.1);
_4 = _6;
_8 = _14;
_23 = RET.0 as isize;
_16.fld1.0 = _18.0;
_16.fld1.1 = _20.1;
_16.fld3.0 = RET.0 & RET.0;
_16.fld3.0 = RET.0 - RET.0;
_8 = core::ptr::addr_of!(RET);
_16.fld0 = _18.1;
_7.0 = _11 as i16;
(*_8) = _16.fld3;
RET.1 = !_16.fld3.1;
_14 = _8;
_16.fld3.1 = 0_usize as u32;
_16.fld2 = _3;
_1 = _5 + _10;
_11 = _12 as u16;
_24 = _20.1 - _18.1;
Call(_16.fld1 = fn10(_9, _11, _2, _9, _6, _4, _18, _9, _20.1, _24, _20.1, _12, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19 = [RET.0,_16.fld3.0,(*_8).0,_16.fld3.0,(*_8).0];
(*_8).1 = _13 as u32;
_22 = RET.1 as isize;
_16 = Adt46 { fld0: _18.1,fld1: _18,fld2: _9,fld3: RET };
(*_14).1 = _16.fld3.1 - _16.fld3.1;
(*_14).1 = _16.fld3.1;
_20.0 = _18.0 as i16;
_6 = _4;
_25.0 = (*_8).0 - (*_14).0;
_5 = _11 as isize;
RET.0 = _18.0 as u8;
Goto(bb6)
}
bb6 = {
Call(_28 = dump_var(9_usize, 10_usize, Move(_10), 21_usize, Move(_21), 4_usize, Move(_4), 11_usize, Move(_11)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_28 = dump_var(9_usize, 9_usize, Move(_9), 1_usize, Move(_1), 19_usize, Move(_19), 29_usize, _29), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i16; 4],mut _2: u16,mut _3: i16,mut _4: [i16; 4],mut _5: isize,mut _6: isize,mut _7: (i128, f32),mut _8: [i16; 4],mut _9: f32,mut _10: f32,mut _11: f32,mut _12: i16,mut _13: i16) -> (i128, f32) {
mir! {
type RET = (i128, f32);
let _14: Adt50;
let _15: [u128; 8];
let _16: bool;
let _17: Adt56;
let _18: u16;
let _19: f32;
let _20: ();
let _21: ();
{
RET.0 = '\u{b9914}' as i128;
_6 = _5 ^ _5;
_2 = _11 as u16;
_14.fld0.1 = _10;
_11 = -_10;
_14.fld2 = [68_u8,37_u8,160_u8,209_u8,250_u8];
_11 = 38_u8 as f32;
_4 = [_12,_13,_12,_12];
RET = (_7.0, _10);
_14.fld1 = ['\u{1612d}','\u{ab11f}','\u{682b6}','\u{c5811}','\u{c496e}'];
_4 = [_13,_12,_13,_12];
_1 = [_12,_13,_3,_13];
_14.fld0.0 = RET.0;
RET.1 = _9;
_14.fld4 = !_13;
_16 = true | false;
_11 = _10;
_6 = 104_i8 as isize;
_15 = [25027960335656273505191598485458311185_u128,178272705382032907721205073949084392428_u128,273672052223001422819437694215733946315_u128,229388737251370121964245415792016700424_u128,152905854718134288822805180913456647309_u128,227276878059346089311670758036319794921_u128,218025787017173308559007785442730953663_u128,45715444570680996694179140524695905081_u128];
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(10_usize, 5_usize, Move(_5), 4_usize, Move(_4), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(10_usize, 1_usize, Move(_1), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: i16,mut _3: isize,mut _4: (u8, u32),mut _5: isize,mut _6: isize) -> i8 {
mir! {
type RET = i8;
let _7: isize;
let _8: *mut (u128, &'static u64, i32);
let _9: isize;
let _10: (i16, f32);
let _11: [i32; 2];
let _12: *mut (u128, &'static u64, i32);
let _13: isize;
let _14: u128;
let _15: ();
let _16: ();
{
_4 = (125_u8, 2401005838_u32);
_4 = (117_u8, 2635471975_u32);
_3 = (-424084345_i32) as isize;
_7 = _1;
_4.0 = _4.1 as u8;
RET = 2_usize as i8;
_6 = _7 * _7;
_7 = RET as isize;
RET = (-110_i8);
_1 = _4.1 as isize;
match _4.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2635471975 => bb7,
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
_1 = _5 - _5;
_4.0 = 231_u8;
_3 = RET as isize;
_1 = 7_usize as isize;
_2 = _4.1 as i16;
_9 = _6 + _6;
_7 = _9 >> _9;
_4 = (173_u8, 969754342_u32);
_5 = !_7;
_6 = _9;
_6 = -_9;
RET = (-31_i8);
RET = 92_i8;
_4.1 = 1630949850_u32 << _7;
RET = 120888233578849587473074297828525958662_i128 as i8;
Call(_6 = fn12(_9, _9, _5, _7, _7, _9, _4.1, _7, _9, _7, _9, _7, _4, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = 36_i8;
_10.1 = 7399332871792529649_i64 as f32;
_4 = (218_u8, 2761342157_u32);
_7 = _9;
_10.1 = _4.0 as f32;
_2 = 9168_i16;
_1 = _2 as isize;
RET = 931802924913696235_i64 as i8;
_4.0 = '\u{96b46}' as u8;
_9 = _6;
_6 = _7 << _7;
RET = true as i8;
_10.0 = _2 >> _5;
_11 = [(-1312684386_i32),1264760000_i32];
_10.1 = _7 as f32;
_2 = _10.0;
_10.1 = 20027_u16 as f32;
_4.1 = RET as u32;
_4 = (240_u8, 1854724559_u32);
Goto(bb9)
}
bb9 = {
_5 = -_9;
_6 = !_7;
_9 = _10.1 as isize;
_5 = _7 >> _10.0;
_7 = _6 ^ _6;
_3 = !_5;
_5 = _6;
_4.1 = 2925155720_u32 & 3464796008_u32;
RET = (-66_i8);
_13 = _5 * _6;
Call(_9 = core::intrinsics::bswap(_5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10.0 = _2 | _2;
_6 = _5 + _13;
_14 = false as u128;
_13 = 1847636308_i32 as isize;
RET = (-127_i8) >> _2;
_6 = _5 << _7;
Goto(bb11)
}
bb11 = {
Call(_15 = dump_var(11_usize, 14_usize, Move(_14), 11_usize, Move(_11), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_15 = dump_var(11_usize, 6_usize, Move(_6), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: u32,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: (u8, u32),mut _14: isize) -> isize {
mir! {
type RET = isize;
let _15: bool;
let _16: (u128, &'static u64, i32);
let _17: isize;
let _18: (i16, f32);
let _19: ();
let _20: ();
{
_12 = _6 | _2;
Goto(bb1)
}
bb1 = {
_9 = 430480508_i32 as isize;
_3 = _14 - _1;
Goto(bb2)
}
bb2 = {
_8 = _3 - _12;
_7 = !_13.1;
_12 = _3 & _5;
_6 = !_5;
_13.1 = _7 * _7;
_15 = true | false;
RET = _12 & _8;
_2 = _1;
_17 = _2 | _3;
_9 = 27993_i16 as isize;
_13 = (26_u8, _7);
_1 = -_14;
_7 = _13.1;
_16.2 = -(-714274007_i32);
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(12_usize, 9_usize, Move(_9), 15_usize, Move(_15), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_19 = dump_var(12_usize, 12_usize, Move(_12), 6_usize, Move(_6), 8_usize, Move(_8), 13_usize, Move(_13)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i16,mut _2: isize,mut _3: u8,mut _4: i8,mut _5: i32,mut _6: bool,mut _7: isize,mut _8: u16,mut _9: i16,mut _10: i16) -> i64 {
mir! {
type RET = i64;
let _11: i16;
let _12: Adt46;
let _13: bool;
let _14: isize;
let _15: *mut (u128, &'static u64, i32);
let _16: (u8, u32);
let _17: Adt53;
let _18: bool;
let _19: (u8, u32);
let _20: bool;
let _21: [i128; 7];
let _22: i16;
let _23: f64;
let _24: [u16; 2];
let _25: &'static u64;
let _26: bool;
let _27: (u128, &'static u64, i32);
let _28: u64;
let _29: Adt51;
let _30: f64;
let _31: (i128, f32);
let _32: Adt45;
let _33: f32;
let _34: u8;
let _35: [u128; 8];
let _36: char;
let _37: [char; 8];
let _38: isize;
let _39: Adt54;
let _40: u32;
let _41: u16;
let _42: Adt59;
let _43: u128;
let _44: Adt46;
let _45: [u16; 2];
let _46: Adt60;
let _47: char;
let _48: bool;
let _49: f64;
let _50: (i16, f32);
let _51: u16;
let _52: f32;
let _53: char;
let _54: bool;
let _55: u8;
let _56: Adt47;
let _57: ();
let _58: ();
{
_1 = !_9;
_10 = _1 * _9;
RET = 2622821719833014040830718419254128232_i128 as i64;
_10 = _1 >> _8;
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = _9;
_11 = _1;
_8 = 29888_u16 * 46127_u16;
_10 = 12862929334029392149_u64 as i16;
Call(_1 = core::intrinsics::transmute(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _9;
_9 = _6 as i16;
_9 = _1;
_8 = _3 as u16;
_11 = _9 ^ _9;
_8 = !41866_u16;
RET = 1753933673866650706_i64 & 98525626783810917_i64;
_4 = 29_i8;
_3 = 205_u8 - 74_u8;
_12.fld1.1 = _8 as f32;
_1 = _9 - _9;
_9 = -_10;
_12.fld1.0 = !(-73525242691327819977809485570815260180_i128);
_12.fld1.1 = _4 as f32;
_2 = !_7;
_12.fld1.0 = _6 as i128;
_6 = _11 >= _9;
_8 = 30651_u16;
Call(_11 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = !false;
_12.fld0 = -_12.fld1.1;
RET = 12850018754937374145_u64 as i64;
_12.fld1 = (9898580037623035521443584520902458900_i128, _12.fld0);
_4 = _6 as i8;
_12.fld3.0 = _3 + _3;
_4 = 77_i8;
_5 = 3410398008_u32 as i32;
RET = 2_usize as i64;
_12.fld0 = _12.fld1.1 + _12.fld1.1;
_12.fld0 = _12.fld1.0 as f32;
_16 = (_12.fld3.0, 1841515242_u32);
_3 = _16.0;
_16.1 = 657233054_u32;
_8 = !6505_u16;
_12.fld1 = (59567450236894015307895416966343322971_i128, _12.fld0);
RET = !3407894865411332846_i64;
_18 = _6;
_9 = !_11;
_12.fld2 = [_1,_9,_1,_1];
_12.fld3 = (_3, _16.1);
_16.0 = _12.fld3.0;
_12.fld3.1 = !_16.1;
RET = -8726550716437952375_i64;
Goto(bb4)
}
bb4 = {
_19 = (_3, _12.fld3.1);
_12.fld1.0 = RET as i128;
_12.fld1 = (40316346016634418178867786966122309361_i128, _12.fld0);
_11 = !_9;
_2 = _4 as isize;
RET = (-3247819436065344038_i64) * (-5231553157264407873_i64);
_13 = _19.0 != _12.fld3.0;
_12.fld3 = (_3, _19.1);
_3 = _16.0;
_4 = (-66_i8) - 64_i8;
_12.fld3 = (_3, _16.1);
_12.fld3 = (_3, _19.1);
_12.fld3 = (_3, _16.1);
_12.fld1 = ((-59519104016542353826145244700876958935_i128), _12.fld0);
_19 = (_16.0, _16.1);
_4 = (-28_i8) ^ (-37_i8);
_13 = _18;
Goto(bb5)
}
bb5 = {
_5 = (-112799598_i32);
_8 = 43537_u16;
_8 = !24296_u16;
_16.0 = _3 ^ _3;
_12.fld1.0 = !166184941222694786429613047689425688884_i128;
_13 = !_6;
_12.fld1 = (23421603373457481672661177748041477126_i128, _12.fld0);
_16.1 = _5 as u32;
_1 = _12.fld3.0 as i16;
_12.fld3.0 = _16.0 - _3;
_11 = 110092150566248148913638194268248472887_u128 as i16;
_21 = [_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0];
_12.fld1 = (64669430553038880943675959824192215877_i128, _12.fld0);
_14 = _12.fld1.1 as isize;
_13 = _18 ^ _6;
_4 = 10182462066755071981_u64 as i8;
_20 = _4 <= _4;
_9 = _16.1 as i16;
_7 = -_14;
_14 = _7 - _7;
Goto(bb6)
}
bb6 = {
_23 = _12.fld0 as f64;
_18 = !_13;
_24 = [_8,_8];
_9 = _5 as i16;
_1 = _9;
_21 = [_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0,_12.fld1.0];
_12.fld1.0 = (-120154789631843331407426684541159187572_i128) ^ 149364875747149292323215182807859448357_i128;
_11 = _23 as i16;
_12.fld1 = ((-31619918304377516228263213212818219872_i128), _12.fld0);
_22 = _10;
_19 = _12.fld3;
_19.1 = _12.fld3.1 << _3;
_27.0 = _12.fld0 as u128;
_19 = (_16.0, _16.1);
_20 = _13;
_18 = _6;
_25 = &_28;
RET = 2046585597780073716_i64 - (-4537509022863997699_i64);
Goto(bb7)
}
bb7 = {
_16.0 = _12.fld3.0 | _12.fld3.0;
_24 = [_8,_8];
_12.fld1 = ((-44450288187194630308611367764105248418_i128), _12.fld0);
Goto(bb8)
}
bb8 = {
_31.1 = _12.fld0 * _12.fld1.1;
_22 = _11 + _11;
_13 = _22 <= _22;
Goto(bb9)
}
bb9 = {
_4 = -(-75_i8);
_32 = Adt45 { fld0: 3_usize,fld1: _27.0 };
_34 = _22 as u8;
_12.fld0 = _31.1;
_26 = _13;
_14 = _2 >> _12.fld3.0;
_12.fld3.0 = !_16.0;
_22 = _11 & _11;
_19 = (_12.fld3.0, _16.1);
_14 = _2;
_15 = core::ptr::addr_of_mut!(_27);
_37 = ['\u{a6715}','\u{3e721}','\u{25691}','\u{2c204}','\u{5f0c0}','\u{294c4}','\u{666c4}','\u{aa1c0}'];
_12.fld0 = _31.1 * _12.fld1.1;
_12.fld0 = _12.fld1.1;
_38 = !_14;
_18 = _13;
_31 = (_12.fld1.0, _12.fld0);
_24 = [_8,_8];
(*_15).1 = &_28;
_33 = _12.fld1.1 - _12.fld0;
Goto(bb10)
}
bb10 = {
_13 = _19.0 == _19.0;
_32.fld0 = 2_usize | 12805234856801527882_usize;
_12.fld1 = (_31.0, _12.fld0);
_10 = _22;
_4 = !41_i8;
_31.1 = _12.fld1.1 + _33;
_12.fld3.0 = '\u{c646c}' as u8;
_20 = _13;
_18 = _20;
Goto(bb11)
}
bb11 = {
_9 = _22 | _11;
_30 = _23 - _23;
_10 = _31.0 as i16;
_12.fld3.1 = _19.1 * _16.1;
_44.fld1.1 = _31.1 + _33;
_14 = !_38;
_12.fld0 = -_33;
_41 = _8 - _8;
_18 = _13;
_27.2 = _5;
_30 = (*_15).2 as f64;
_24 = [_41,_41];
_44.fld1 = (_12.fld1.0, _33);
_20 = _18;
_44.fld3.1 = !_12.fld3.1;
Goto(bb12)
}
bb12 = {
_44.fld3 = (_16.0, _12.fld3.1);
_7 = -_14;
_44.fld2 = [_9,_9,_22,_10];
_19 = (_44.fld3.0, _44.fld3.1);
_24 = [_8,_8];
_14 = _2;
_35 = [(*_15).0,_27.0,_27.0,(*_15).0,(*_15).0,_27.0,_32.fld1,_32.fld1];
Goto(bb13)
}
bb13 = {
_14 = _2 & _7;
_31 = (_12.fld1.0, _12.fld1.1);
_12.fld1.1 = _33 * _33;
_37 = ['\u{4e2ac}','\u{5b1cb}','\u{211d7}','\u{48dc5}','\u{1cce1}','\u{10df1c}','\u{92a76}','\u{cb27a}'];
_19 = (_44.fld3.0, _12.fld3.1);
_45 = _24;
_32 = Adt45 { fld0: 2_usize,fld1: (*_15).0 };
_12.fld1.1 = _12.fld0 - _12.fld0;
_16.1 = _44.fld3.1 | _44.fld3.1;
_33 = _4 as f32;
(*_15).2 = RET as i32;
_44.fld3.1 = !_16.1;
_13 = _18;
_39 = Adt54::Variant1 { fld0: _23 };
_44.fld3 = (_19.0, _16.1);
Call(_47 = fn14(_16, _12.fld1, Move(_12), _22, _44.fld1.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_47 = '\u{383a4}';
_12.fld1.1 = -_44.fld1.1;
_50.1 = _44.fld1.1 * _44.fld1.1;
_12.fld3.1 = _32.fld0 as u32;
SetDiscriminant(_39, 0);
_12.fld1.1 = _19.0 as f32;
place!(Field::<*mut i32>(Variant(_39, 0), 1)) = core::ptr::addr_of_mut!((*_15).2);
_52 = _12.fld1.1 + _50.1;
_48 = !_18;
_12.fld3.1 = _32.fld0 as u32;
_12.fld1.0 = _23 as i128;
_27.0 = RET as u128;
_28 = !650145782448135126_u64;
_44.fld0 = -_31.1;
_12.fld0 = _27.2 as f32;
_27.1 = &_28;
_55 = _3 ^ _19.0;
place!(Field::<Adt45>(Variant(_39, 0), 5)).fld0 = !_32.fld0;
place!(Field::<i16>(Variant(_39, 0), 4)) = !_9;
place!(Field::<i128>(Variant(_39, 0), 2)) = _31.0 - _12.fld1.0;
_13 = _12.fld1.1 != _31.1;
Goto(bb15)
}
bb15 = {
Call(_57 = dump_var(13_usize, 26_usize, Move(_26), 22_usize, Move(_22), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_57 = dump_var(13_usize, 16_usize, Move(_16), 24_usize, Move(_24), 21_usize, Move(_21), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(13_usize, 11_usize, Move(_11), 37_usize, Move(_37), 5_usize, Move(_5), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_57 = dump_var(13_usize, 3_usize, Move(_3), 48_usize, Move(_48), 14_usize, Move(_14), 58_usize, _58), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (u8, u32),mut _2: (i128, f32),mut _3: Adt46,mut _4: i16,mut _5: i128) -> char {
mir! {
type RET = char;
let _6: Adt57;
let _7: f32;
let _8: [u16; 2];
let _9: char;
let _10: u8;
let _11: isize;
let _12: i32;
let _13: Adt46;
let _14: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64);
let _15: (u8, u32);
let _16: f64;
let _17: i64;
let _18: (u8, u32);
let _19: usize;
let _20: isize;
let _21: ();
let _22: ();
{
_1.1 = _3.fld3.1;
_3.fld0 = 1998322489713517848_i64 as f32;
_2 = (_3.fld1.0, _3.fld1.1);
_1.1 = _3.fld3.1 & _3.fld3.1;
_3.fld0 = -_2.1;
RET = '\u{4fe4b}';
_4 = (-13616_i16) - 17932_i16;
_3.fld3.0 = _1.0 << _3.fld1.0;
_3.fld0 = _2.1;
_2 = (_3.fld1.0, _3.fld0);
_7 = _2.1;
_3.fld3.0 = _1.0;
_1 = (_3.fld3.0, _3.fld3.1);
_4 = _3.fld0 as i16;
_3.fld1.1 = 33804522748459139039437354250662759884_u128 as f32;
_2.0 = 86_i8 as i128;
_3.fld1 = (_2.0, _7);
_1.1 = _3.fld3.1 | _3.fld3.1;
_3.fld1.0 = 121_i8 as i128;
_1 = (_3.fld3.0, _3.fld3.1);
RET = '\u{1e4}';
_1.1 = _2.1 as u32;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
295832078733743833154763239667662963038 => bb9,
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
_1.0 = _3.fld3.0 + _3.fld3.0;
Call(_3.fld2 = fn15(_7, _3.fld1, _1, _7, _2, _3.fld3, _1.0, _3.fld1.0, _1.0, _1.0, _1.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2 = (_5, _7);
_7 = -_2.1;
_3.fld1.0 = _3.fld3.0 as i128;
_8 = [56592_u16,7376_u16];
_3.fld3 = (_1.0, _1.1);
_3.fld2 = [_4,_4,_4,_4];
RET = '\u{b0208}';
_1 = (_3.fld3.0, _3.fld3.1);
_2 = _3.fld1;
_3.fld3.0 = _1.0 * _1.0;
_3.fld3 = (_1.0, _1.1);
_3.fld3 = (_1.0, _1.1);
_3.fld3.1 = RET as u32;
_3.fld1.1 = _3.fld0;
_3.fld2 = [_4,_4,_4,_4];
_1.1 = !_3.fld3.1;
_2.0 = !_3.fld1.0;
_7 = -_3.fld1.1;
_3.fld3.1 = _1.1 << _3.fld3.0;
_2.1 = _7;
_5 = RET as i128;
_2.1 = -_3.fld0;
RET = '\u{e0795}';
_9 = RET;
Goto(bb11)
}
bb11 = {
_3.fld2 = [_4,_4,_4,_4];
_2.1 = _3.fld1.1 - _3.fld0;
_3.fld2 = [_4,_4,_4,_4];
_3.fld1 = (_2.0, _2.1);
_8 = [10178_u16,61287_u16];
_1.1 = _3.fld3.1 & _3.fld3.1;
Goto(bb12)
}
bb12 = {
_3.fld1 = (_2.0, _7);
_8 = [41227_u16,9899_u16];
RET = _9;
_7 = 44112_u16 as f32;
_4 = 5041970031098118232_usize as i16;
_7 = _2.1 * _3.fld1.1;
RET = _9;
_9 = RET;
_11 = _1.0 as isize;
_2.0 = _3.fld1.0;
Call(_3.fld2 = core::intrinsics::transmute(_11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = _9;
_12 = -1954721316_i32;
_2.1 = _7;
_2.0 = -_3.fld1.0;
_3.fld1 = _2;
RET = _9;
_2.0 = _12 as i128;
_2 = (_3.fld1.0, _3.fld0);
_3.fld3 = (_1.0, _1.1);
_3.fld1.1 = _7 - _2.1;
RET = _9;
_13.fld1 = (_2.0, _2.1);
_13.fld0 = 258625211348622892885398846062919818670_u128 as f32;
_3.fld1.1 = _13.fld1.1 * _7;
_14.3.1 = !_3.fld3.1;
_13.fld1.1 = -_3.fld0;
_11 = (-56_isize);
_13 = Move(_3);
_1.1 = _14.3.1 + _13.fld3.1;
_2.1 = _7;
_3.fld1.1 = _13.fld1.1;
_14.0 = core::ptr::addr_of_mut!(_12);
_15.0 = _13.fld3.0 ^ _1.0;
_13.fld2 = [_4,_4,_4,_4];
_14.3.0 = !_1.0;
_14.4 = _4 as f64;
match _11 {
0 => bb14,
1 => bb15,
2 => bb16,
340282366920938463463374607431768211400 => bb18,
_ => bb17
}
}
bb14 = {
Return()
}
bb15 = {
_3.fld2 = [_4,_4,_4,_4];
_2.1 = _3.fld1.1 - _3.fld0;
_3.fld2 = [_4,_4,_4,_4];
_3.fld1 = (_2.0, _2.1);
_8 = [10178_u16,61287_u16];
_1.1 = _3.fld3.1 & _3.fld3.1;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET = _9;
_11 = !(-95_isize);
_3.fld1.0 = _13.fld3.0 as i128;
_9 = RET;
_13.fld1.1 = _14.4 as f32;
_14.0 = core::ptr::addr_of_mut!(_12);
_7 = _3.fld1.1;
_14.2.0 = -_4;
_3.fld1 = (_2.0, _7);
_13.fld3.1 = !_1.1;
_15.0 = !_13.fld3.0;
RET = _9;
Goto(bb19)
}
bb19 = {
Call(_21 = dump_var(14_usize, 9_usize, Move(_9), 8_usize, Move(_8), 5_usize, Move(_5), 22_usize, _22), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: f32,mut _2: (i128, f32),mut _3: (u8, u32),mut _4: f32,mut _5: (i128, f32),mut _6: (u8, u32),mut _7: u8,mut _8: i128,mut _9: u8,mut _10: u8,mut _11: u8) -> [i16; 4] {
mir! {
type RET = [i16; 4];
let _12: (u128, &'static u64, i32);
let _13: char;
let _14: Adt45;
let _15: char;
let _16: i128;
let _17: u32;
let _18: i128;
let _19: [i32; 2];
let _20: f32;
let _21: f32;
let _22: isize;
let _23: f64;
let _24: Adt46;
let _25: &'static u64;
let _26: ();
let _27: ();
{
RET = [9096_i16,5837_i16,19327_i16,20968_i16];
_11 = _6.0;
_10 = _6.0;
_6.0 = 42838_u16 as u8;
_12.2 = !(-205917067_i32);
_6 = _3;
_6.1 = _3.1;
_3.1 = _12.2 as u32;
Goto(bb1)
}
bb1 = {
_8 = _5.0 * _5.0;
_3.1 = !_6.1;
_1 = _5.1 * _5.1;
_6.0 = (-9223372036854775808_isize) as u8;
_12.0 = !126173674006232186722832117982899955228_u128;
_5 = (_8, _1);
_2 = (_8, _5.1);
_2 = _5;
_14.fld0 = !5509242178675263896_usize;
_2.0 = _3.0 as i128;
_11 = _3.0 + _7;
_14.fld0 = 7_usize;
_3 = (_7, _6.1);
_12.0 = !146950357266549445041670915026922192367_u128;
_8 = !_2.0;
_6.1 = !_3.1;
_14.fld0 = _3.0 as usize;
_2.0 = _5.0;
_4 = _5.1;
Goto(bb2)
}
bb2 = {
_9 = _1 as u8;
_14 = Adt45 { fld0: 17573013289737243925_usize,fld1: _12.0 };
_6.1 = 55991_u16 as u32;
_3 = (_7, _6.1);
_5.1 = _1;
_1 = 9232549161827759109_u64 as f32;
_5.1 = _4;
RET = [4172_i16,(-29180_i16),29797_i16,23764_i16];
_6.0 = _11 * _11;
_6.1 = _14.fld1 as u32;
_13 = '\u{43838}';
_5.0 = _6.1 as i128;
_1 = _5.1;
_3.1 = _12.2 as u32;
_8 = _5.0 >> _11;
_7 = _9 | _6.0;
_21 = _2.1 + _5.1;
_15 = _13;
match _14.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
17573013289737243925 => bb7,
_ => bb6
}
}
bb3 = {
_8 = _5.0 * _5.0;
_3.1 = !_6.1;
_1 = _5.1 * _5.1;
_6.0 = (-9223372036854775808_isize) as u8;
_12.0 = !126173674006232186722832117982899955228_u128;
_5 = (_8, _1);
_2 = (_8, _5.1);
_2 = _5;
_14.fld0 = !5509242178675263896_usize;
_2.0 = _3.0 as i128;
_11 = _3.0 + _7;
_14.fld0 = 7_usize;
_3 = (_7, _6.1);
_12.0 = !146950357266549445041670915026922192367_u128;
_8 = !_2.0;
_6.1 = !_3.1;
_14.fld0 = _3.0 as usize;
_2.0 = _5.0;
_4 = _5.1;
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
_19 = [_12.2,_12.2];
_10 = _7 >> _7;
_14.fld1 = _12.0;
_5 = (_8, _21);
_21 = -_2.1;
_1 = -_4;
_12.2 = (-623163808782381188_i64) as i32;
_20 = _21;
_7 = _14.fld1 as u8;
RET = [9400_i16,18213_i16,23297_i16,21134_i16];
_7 = _6.0 ^ _6.0;
_1 = _20 + _21;
_3.0 = _6.0 >> _7;
Call(_6.1 = fn16(_3.0, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = -_1;
_19 = [_12.2,_12.2];
_18 = !_5.0;
_3 = _6;
_16 = _18;
_16 = -_5.0;
_12.0 = _14.fld1 | _14.fld1;
_9 = _3.0;
_3 = (_7, _6.1);
_16 = _5.0;
_6 = (_7, _3.1);
_11 = _9 ^ _3.0;
_18 = _16 * _16;
Goto(bb9)
}
bb9 = {
_22 = _6.1 as isize;
RET = [(-30015_i16),(-8438_i16),1329_i16,18160_i16];
_15 = _13;
_23 = 28900_i16 as f64;
_5.0 = !_16;
_10 = _7 ^ _3.0;
_3.0 = _10 >> _7;
_24.fld1.0 = _18;
_24.fld1.1 = _1 - _4;
match _14.fld0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
17573013289737243925 => bb16,
_ => bb15
}
}
bb10 = {
_4 = -_1;
_19 = [_12.2,_12.2];
_18 = !_5.0;
_3 = _6;
_16 = _18;
_16 = -_5.0;
_12.0 = _14.fld1 | _14.fld1;
_9 = _3.0;
_3 = (_7, _6.1);
_16 = _5.0;
_6 = (_7, _3.1);
_11 = _9 ^ _3.0;
_18 = _16 * _16;
Goto(bb9)
}
bb11 = {
_19 = [_12.2,_12.2];
_10 = _7 >> _7;
_14.fld1 = _12.0;
_5 = (_8, _21);
_21 = -_2.1;
_1 = -_4;
_12.2 = (-623163808782381188_i64) as i32;
_20 = _21;
_7 = _14.fld1 as u8;
RET = [9400_i16,18213_i16,23297_i16,21134_i16];
_7 = _6.0 ^ _6.0;
_1 = _20 + _21;
_3.0 = _6.0 >> _7;
Call(_6.1 = fn16(_3.0, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
_8 = _5.0 * _5.0;
_3.1 = !_6.1;
_1 = _5.1 * _5.1;
_6.0 = (-9223372036854775808_isize) as u8;
_12.0 = !126173674006232186722832117982899955228_u128;
_5 = (_8, _1);
_2 = (_8, _5.1);
_2 = _5;
_14.fld0 = !5509242178675263896_usize;
_2.0 = _3.0 as i128;
_11 = _3.0 + _7;
_14.fld0 = 7_usize;
_3 = (_7, _6.1);
_12.0 = !146950357266549445041670915026922192367_u128;
_8 = !_2.0;
_6.1 = !_3.1;
_14.fld0 = _3.0 as usize;
_2.0 = _5.0;
_4 = _5.1;
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_8 = _5.0 * _5.0;
_3.1 = !_6.1;
_1 = _5.1 * _5.1;
_6.0 = (-9223372036854775808_isize) as u8;
_12.0 = !126173674006232186722832117982899955228_u128;
_5 = (_8, _1);
_2 = (_8, _5.1);
_2 = _5;
_14.fld0 = !5509242178675263896_usize;
_2.0 = _3.0 as i128;
_11 = _3.0 + _7;
_14.fld0 = 7_usize;
_3 = (_7, _6.1);
_12.0 = !146950357266549445041670915026922192367_u128;
_8 = !_2.0;
_6.1 = !_3.1;
_14.fld0 = _3.0 as usize;
_2.0 = _5.0;
_4 = _5.1;
Goto(bb2)
}
bb16 = {
_24.fld3.0 = _23 as u8;
_21 = -_24.fld1.1;
_5 = _24.fld1;
_5.0 = _8;
_18 = -_8;
_23 = _24.fld1.0 as f64;
_8 = false as i128;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(15_usize, 16_usize, Move(_16), 22_usize, Move(_22), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(15_usize, 19_usize, Move(_19), 9_usize, Move(_9), 27_usize, _27, 27_usize, _27), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u8,mut _2: u8) -> u32 {
mir! {
type RET = u32;
let _3: isize;
let _4: [char; 8];
let _5: i128;
let _6: bool;
let _7: f64;
let _8: [u16; 2];
let _9: (i16, f32);
let _10: u16;
let _11: f64;
let _12: u8;
let _13: char;
let _14: bool;
let _15: [u128; 8];
let _16: *mut usize;
let _17: Adt58;
let _18: (u8, u32);
let _19: Adt50;
let _20: char;
let _21: isize;
let _22: u16;
let _23: [u128; 8];
let _24: (i16, f32);
let _25: Adt54;
let _26: ();
let _27: ();
{
_1 = 8580473286612063972_u64 as u8;
_1 = _2 - _2;
_1 = _2;
_2 = (-9223372036854775808_isize) as u8;
Call(RET = core::intrinsics::bswap(2765981876_u32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _2 << _2;
_1 = !_2;
_4 = ['\u{983a5}','\u{104494}','\u{25e87}','\u{10a4f3}','\u{82475}','\u{59066}','\u{3b16d}','\u{5740d}'];
_3 = 11632096220869839881782866755325220351_u128 as isize;
_1 = _2 - _2;
_4 = ['\u{d88cb}','\u{91abc}','\u{2ba22}','\u{cfb6}','\u{f2a27}','\u{b11c9}','\u{aa8a5}','\u{8438a}'];
_6 = _1 > _2;
_1 = _2 | _2;
_5 = 32220751979929132369541569553598827661_i128;
_4 = ['\u{613d8}','\u{556dc}','\u{103aaf}','\u{c8736}','\u{82722}','\u{5a6a7}','\u{9e0b2}','\u{e8269}'];
_1 = 168177492229488332307726620787633494059_u128 as u8;
RET = 3142937614_u32;
RET = 2502847670_u32;
RET = 566131156_u32;
_3 = !(-9223372036854775808_isize);
_2 = (-126_i8) as u8;
_4 = ['\u{a1212}','\u{88d78}','\u{afb13}','\u{f64d9}','\u{87f04}','\u{afc65}','\u{37c1c}','\u{35d01}'];
_4 = ['\u{10daf7}','\u{a639f}','\u{a48b2}','\u{fa76e}','\u{aa0c6}','\u{986dc}','\u{8594a}','\u{bff9b}'];
RET = 12883277980360566588_u64 as u32;
_4 = ['\u{c3fde}','\u{e811d}','\u{45a25}','\u{1d208}','\u{e318c}','\u{53784}','\u{f466b}','\u{2be62}'];
_1 = _2;
_1 = !_2;
_1 = !_2;
_4 = ['\u{f5ff1}','\u{5a1cc}','\u{e7e3}','\u{e1619}','\u{3c66f}','\u{befdd}','\u{10bcbe}','\u{879}'];
_2 = _1;
Goto(bb2)
}
bb2 = {
_5 = 154438763324029782350686528916427895114_i128;
_1 = !_2;
_3 = 7279490487330915056_usize as isize;
_5 = _3 as i128;
_7 = 2436779012001282368_i64 as f64;
_4 = ['\u{108a9f}','\u{105966}','\u{8cc4b}','\u{43588}','\u{6b47}','\u{3b68d}','\u{48159}','\u{5065a}'];
_2 = _1;
_2 = 1900146395_i32 as u8;
_6 = _3 > _3;
_3 = '\u{69080}' as isize;
_5 = 156625548881807347384959523978509954075_i128 << _3;
_4 = ['\u{36e28}','\u{c690d}','\u{71428}','\u{7b16e}','\u{a56f3}','\u{a50a0}','\u{179b5}','\u{8f47a}'];
_1 = !_2;
_1 = (-124_i8) as u8;
Goto(bb3)
}
bb3 = {
RET = 2768074047_u32 + 120222003_u32;
_3 = 9223372036854775807_isize;
_8 = [26919_u16,21430_u16];
Call(_5 = core::intrinsics::bswap(103371305585734907678558608033650774648_i128), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = _2;
_10 = !9589_u16;
_5 = (-10468264669626030765380287565840122838_i128) | 151006701752670118581631552068628790853_i128;
_1 = _2;
match _3 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
}
bb5 = {
RET = 2768074047_u32 + 120222003_u32;
_3 = 9223372036854775807_isize;
_8 = [26919_u16,21430_u16];
Call(_5 = core::intrinsics::bswap(103371305585734907678558608033650774648_i128), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_5 = 154438763324029782350686528916427895114_i128;
_1 = !_2;
_3 = 7279490487330915056_usize as isize;
_5 = _3 as i128;
_7 = 2436779012001282368_i64 as f64;
_4 = ['\u{108a9f}','\u{105966}','\u{8cc4b}','\u{43588}','\u{6b47}','\u{3b68d}','\u{48159}','\u{5065a}'];
_2 = _1;
_2 = 1900146395_i32 as u8;
_6 = _3 > _3;
_3 = '\u{69080}' as isize;
_5 = 156625548881807347384959523978509954075_i128 << _3;
_4 = ['\u{36e28}','\u{c690d}','\u{71428}','\u{7b16e}','\u{a56f3}','\u{a50a0}','\u{179b5}','\u{8f47a}'];
_1 = !_2;
_1 = (-124_i8) as u8;
Goto(bb3)
}
bb7 = {
_1 = _2 << _2;
_1 = !_2;
_4 = ['\u{983a5}','\u{104494}','\u{25e87}','\u{10a4f3}','\u{82475}','\u{59066}','\u{3b16d}','\u{5740d}'];
_3 = 11632096220869839881782866755325220351_u128 as isize;
_1 = _2 - _2;
_4 = ['\u{d88cb}','\u{91abc}','\u{2ba22}','\u{cfb6}','\u{f2a27}','\u{b11c9}','\u{aa8a5}','\u{8438a}'];
_6 = _1 > _2;
_1 = _2 | _2;
_5 = 32220751979929132369541569553598827661_i128;
_4 = ['\u{613d8}','\u{556dc}','\u{103aaf}','\u{c8736}','\u{82722}','\u{5a6a7}','\u{9e0b2}','\u{e8269}'];
_1 = 168177492229488332307726620787633494059_u128 as u8;
RET = 3142937614_u32;
RET = 2502847670_u32;
RET = 566131156_u32;
_3 = !(-9223372036854775808_isize);
_2 = (-126_i8) as u8;
_4 = ['\u{a1212}','\u{88d78}','\u{afb13}','\u{f64d9}','\u{87f04}','\u{afc65}','\u{37c1c}','\u{35d01}'];
_4 = ['\u{10daf7}','\u{a639f}','\u{a48b2}','\u{fa76e}','\u{aa0c6}','\u{986dc}','\u{8594a}','\u{bff9b}'];
RET = 12883277980360566588_u64 as u32;
_4 = ['\u{c3fde}','\u{e811d}','\u{45a25}','\u{1d208}','\u{e318c}','\u{53784}','\u{f466b}','\u{2be62}'];
_1 = _2;
_1 = !_2;
_1 = !_2;
_4 = ['\u{f5ff1}','\u{5a1cc}','\u{e7e3}','\u{e1619}','\u{3c66f}','\u{befdd}','\u{10bcbe}','\u{879}'];
_2 = _1;
Goto(bb2)
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
_11 = 334494890162127161790572372953575909310_u128 as f64;
_5 = 100299906808445616459966860362168394043_i128;
_9.1 = 376900459_i32 as f32;
_8 = [_10,_10];
_1 = _2;
_10 = 10171748538721912113_usize as u16;
_9.0 = 12572_i16 * 20165_i16;
RET = 2214947563_u32 + 3541776681_u32;
_13 = '\u{525e8}';
_8 = [_10,_10];
_8 = [_10,_10];
_11 = _7;
_15 = [116936350866448368262461961384992552790_u128,173337157040213523689776547580231411942_u128,48768739965494646771629605739686661011_u128,166366049793108012777244884777173546846_u128,324509221317241318356716460448667044267_u128,220265277055527132443479200818739539908_u128,222398269902073895173183338967974170665_u128,299845995942850064637537164046315417662_u128];
_4 = [_13,_13,_13,_13,_13,_13,_13,_13];
_7 = _11;
Goto(bb13)
}
bb13 = {
_12 = _13 as u8;
_14 = _6;
_14 = !_6;
_14 = _6;
_19.fld2 = [_1,_12,_2,_1,_12];
_19.fld5 = Adt47::Variant3 { fld0: _15,fld1: _5 };
_19.fld0.0 = 2205782047519797401_usize as i128;
_18 = (_2, RET);
_10 = 14360_u16;
_8 = [_10,_10];
place!(Field::<[u128; 8]>(Variant(_19.fld5, 3), 0)) = [53961145707773708994113686056363167284_u128,21298063922907006918111082022927779940_u128,270090795618345812408393697536666547463_u128,6726552077124091987225888830273066371_u128,203998071977300743678811350986278862337_u128,37723211269490120477067378337275119872_u128,169680735142289530700075484259294720641_u128,176762671222723529874012513837895908872_u128];
_1 = _3 as u8;
_19.fld2 = [_12,_1,_18.0,_18.0,_12];
_11 = -_7;
_19.fld3 = core::ptr::addr_of_mut!(_14);
_1 = _2 + _18.0;
_9.0 = -(-14493_i16);
_15 = [111192904064804117107325372086377304899_u128,302182440880268737283238736787478135449_u128,66158866594171932127269380355346505762_u128,109106552475221602946384805840048208976_u128,279832118923846993388558026317134857047_u128,166411992079898462549225341802266593076_u128,91027211553692376121136837679976739676_u128,277046613064736681382899595752417919249_u128];
_19.fld0.1 = _9.1;
_19.fld2 = [_1,_18.0,_1,_18.0,_1];
_4 = [_13,_13,_13,_13,_13,_13,_13,_13];
place!(Field::<i128>(Variant(_19.fld5, 3), 1)) = !_19.fld0.0;
_1 = Field::<i128>(Variant(_19.fld5, 3), 1) as u8;
_9.1 = _19.fld0.1 - _19.fld0.1;
_19.fld3 = core::ptr::addr_of_mut!(_14);
match _3 {
0 => bb1,
1 => bb11,
2 => bb3,
3 => bb4,
4 => bb5,
9223372036854775807 => bb14,
_ => bb12
}
}
bb14 = {
_24.0 = _13 as i16;
place!(Field::<[u128; 8]>(Variant(_19.fld5, 3), 0)) = [284462018809204969098795225630015426117_u128,317019460587756113748445727036140566279_u128,59615404588945072825726868869797722438_u128,189971991930339323817558632646110906658_u128,231179913455485898740095181430610368175_u128,242598300232374591947114306961825985147_u128,51225480494660836957311289411275849878_u128,288798311858009180004417609844014919469_u128];
_24.0 = _9.0 & _9.0;
_18 = (_12, RET);
_24 = _9;
_20 = _13;
_21 = !_3;
_18.0 = _2;
_23 = _15;
_24.1 = _9.1;
_19.fld4 = 353431857_i32 as i16;
_14 = RET >= RET;
_9 = _24;
_10 = _12 as u16;
_18.1 = RET;
_5 = Field::<i128>(Variant(_19.fld5, 3), 1);
_3 = (-8604476835656391043_i64) as isize;
_18.1 = RET;
_10 = 33791_u16;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(16_usize, 5_usize, Move(_5), 3_usize, Move(_3), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(16_usize, 15_usize, Move(_15), 23_usize, Move(_23), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: f32,mut _2: u128,mut _3: u64,mut _4: u32,mut _5: f64,mut _6: u32,mut _7: u64,mut _8: u32,mut _9: isize) -> u8 {
mir! {
type RET = u8;
let _10: isize;
let _11: u64;
let _12: Adt45;
let _13: isize;
let _14: Adt59;
let _15: isize;
let _16: Adt45;
let _17: usize;
let _18: u64;
let _19: f32;
let _20: Adt46;
let _21: Adt44;
let _22: usize;
let _23: [i128; 7];
let _24: f32;
let _25: (u16, u16, i8, [i16; 4], (*mut usize, u16, i128));
let _26: [u8; 5];
let _27: Adt46;
let _28: i8;
let _29: i16;
let _30: Adt55;
let _31: (i16, f32);
let _32: [u128; 8];
let _33: [char; 8];
let _34: ();
let _35: ();
{
_5 = 98022807837340410587685627812085950246_i128 as f64;
RET = 236_u8;
_5 = 2_i8 as f64;
_1 = _2 as f32;
_1 = (-1267645539_i32) as f32;
_4 = _8 ^ _6;
_4 = _6 & _6;
_3 = 1263166483_i32 as u64;
_8 = _6 - _4;
RET = !65_u8;
_4 = _6 & _8;
_2 = !219377579987800643642951587244898562866_u128;
_4 = _8;
_10 = _9;
_1 = _10 as f32;
match _7 {
0 => bb1,
1 => bb2,
17895269132328611117 => bb4,
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
_8 = _10 as u32;
_11 = !_7;
_10 = -_9;
_2 = 128712291414723220943533666815501398762_u128;
_1 = _2 as f32;
_3 = RET as u64;
_10 = _5 as isize;
_3 = !_11;
_12 = Adt45 { fld0: 434777815041570901_usize,fld1: _2 };
_12.fld0 = 9107253335548908870_usize * 12981899983271512831_usize;
_12.fld1 = _2;
_10 = _9;
_2 = _1 as u128;
_2 = 30459_i16 as u128;
RET = _5 as u8;
match _12.fld1 {
0 => bb5,
1 => bb6,
2 => bb7,
128712291414723220943533666815501398762 => bb9,
_ => bb8
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
RET = !76_u8;
_8 = _4;
_13 = _10 + _10;
_16.fld0 = _12.fld0 & _12.fld0;
_3 = _16.fld0 as u64;
_16 = Adt45 { fld0: _12.fld0,fld1: _12.fld1 };
RET = 92_u8;
_12.fld0 = 91_i8 as usize;
_7 = _11;
_9 = _10;
_11 = true as u64;
_7 = 26_i8 as u64;
_8 = 20775_i16 as u32;
_6 = _4;
_8 = !_6;
_7 = !_11;
_12 = Adt45 { fld0: _16.fld0,fld1: _2 };
_18 = 115497586011282996649382799330389117829_i128 as u64;
_20.fld3.0 = RET >> _3;
_17 = (-91_i8) as usize;
match RET {
0 => bb1,
1 => bb8,
2 => bb7,
3 => bb4,
4 => bb5,
92 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_20.fld3.1 = (-1672100340_i32) as u32;
_9 = _1 as isize;
_11 = !_7;
_20.fld1.0 = !133767432579207577461581000949973162847_i128;
Goto(bb12)
}
bb12 = {
_16.fld1 = (-2368390121516193625_i64) as u128;
_20.fld0 = _1 * _1;
_11 = _3;
_16 = Adt45 { fld0: _12.fld0,fld1: _12.fld1 };
_20.fld3 = (RET, _8);
_24 = _12.fld1 as f32;
_12.fld0 = _8 as usize;
_20.fld3 = (RET, _6);
_20.fld1 = (41241709418678346381159285546798942592_i128, _1);
_6 = _4 * _20.fld3.1;
_3 = _24 as u64;
_27.fld2 = [(-31396_i16),(-31979_i16),(-23467_i16),19147_i16];
_27.fld1.1 = -_20.fld1.1;
_6 = _8 - _8;
_24 = (-271016499_i32) as f32;
_26 = [_20.fld3.0,_20.fld3.0,RET,_20.fld3.0,RET];
_15 = _20.fld3.1 as isize;
_20.fld3.0 = RET;
_20.fld1 = (134502079897739404880443392249243924803_i128, _20.fld0);
_27.fld1.0 = _20.fld1.0 - _20.fld1.0;
_12.fld1 = _2 - _16.fld1;
_27.fld0 = _20.fld1.1 * _20.fld1.1;
_28 = _4 as i8;
match _20.fld1.0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
134502079897739404880443392249243924803 => bb21,
_ => bb20
}
}
bb13 = {
_20.fld3.1 = (-1672100340_i32) as u32;
_9 = _1 as isize;
_11 = !_7;
_20.fld1.0 = !133767432579207577461581000949973162847_i128;
Goto(bb12)
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
Return()
}
bb19 = {
Return()
}
bb20 = {
_8 = _10 as u32;
_11 = !_7;
_10 = -_9;
_2 = 128712291414723220943533666815501398762_u128;
_1 = _2 as f32;
_3 = RET as u64;
_10 = _5 as isize;
_3 = !_11;
_12 = Adt45 { fld0: 434777815041570901_usize,fld1: _2 };
_12.fld0 = 9107253335548908870_usize * 12981899983271512831_usize;
_12.fld1 = _2;
_10 = _9;
_2 = _1 as u128;
_2 = 30459_i16 as u128;
RET = _5 as u8;
match _12.fld1 {
0 => bb5,
1 => bb6,
2 => bb7,
128712291414723220943533666815501398762 => bb9,
_ => bb8
}
}
bb21 = {
_22 = _5 as usize;
_20.fld3.0 = RET / RET;
_26 = [_20.fld3.0,_20.fld3.0,_20.fld3.0,_20.fld3.0,_20.fld3.0];
_19 = _27.fld0 + _27.fld1.1;
_28 = (-83_i8) & 67_i8;
_25.4.0 = core::ptr::addr_of_mut!(_16.fld0);
_27.fld3.0 = RET & _20.fld3.0;
_22 = !_12.fld0;
_20.fld2 = [2061_i16,30886_i16,(-31270_i16),(-17311_i16)];
_9 = _15;
_20.fld3.0 = !RET;
_19 = _27.fld0 - _27.fld0;
_27.fld1.1 = -_19;
_25.3 = _20.fld2;
Goto(bb22)
}
bb22 = {
Call(_34 = dump_var(17_usize, 4_usize, Move(_4), 6_usize, Move(_6), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_34 = dump_var(17_usize, 26_usize, Move(_26), 17_usize, Move(_17), 18_usize, Move(_18), 13_usize, Move(_13)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{1ce2d}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(58_i8), std::hint::black_box(730_i16), std::hint::black_box((-731547125_i32)), std::hint::black_box((-3580227729987152336_i64)), std::hint::black_box((-97416968939820754252122979336935919611_i128)), std::hint::black_box(6_usize), std::hint::black_box(216_u8), std::hint::black_box(60458_u16), std::hint::black_box(1633664623_u32), std::hint::black_box(469351311195976496_u64), std::hint::black_box(325275743890792772171035496042433242008_u128));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut bool,

},
Variant1{
fld0: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64),

},
Variant2{
fld0: (u8, u32),
fld1: (i16, f32),
fld2: (*mut usize, u16, i128),
fld3: (u16, u16, i8, [i16; 4], (*mut usize, u16, i128)),
fld4: i16,
fld5: f32,
fld6: i64,

},
Variant3{
fld0: *mut bool,
fld1: f64,
fld2: u32,
fld3: i8,
fld4: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64),
fld5: (i16, f32),
fld6: [u128; 8],
fld7: [i128; 7],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: usize,
fld1: u128,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: f32,
fld1: (i128, f32),
fld2: [i16; 4],
fld3: (u8, u32),
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: u64,
fld2: *const u128,
fld3: [u128; 8],
fld4: i16,
fld5: i32,

},
Variant1{
fld0: *const (u8, u32),

},
Variant2{
fld0: i16,
fld1: i32,

},
Variant3{
fld0: [u128; 8],
fld1: i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut usize,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: [i128; 7],
fld3: (u16, u16, i8, [i16; 4], (*mut usize, u16, i128)),
fld4: [u16; 2],
fld5: (i128, f32),
fld6: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64),
fld7: (u8, u32),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u8,
fld1: f64,

},
Variant1{
fld0: bool,
fld1: char,
fld2: Adt48,
fld3: *const (u8, u32),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: (i128, f32),
fld1: [char; 5],
fld2: [u8; 5],
fld3: *mut bool,
fld4: i16,
fld5: Adt47,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u32,
fld1: [i128; 7],
fld2: [char; 5],
fld3: Adt47,
fld4: [u16; 2],
fld5: i32,

},
Variant1{
fld0: f32,
fld1: char,
fld2: Adt49,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64),

},
Variant1{
fld0: bool,
fld1: *mut usize,
fld2: (i16, *mut usize),
fld3: Adt47,
fld4: u128,
fld5: i32,
fld6: (i128, f32),

},
Variant2{
fld0: i64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u8,

},
Variant1{
fld0: Adt51,
fld1: Adt52,
fld2: (i16,),
fld3: [u8; 5],
fld4: Adt45,
fld5: *mut i32,
fld6: *mut bool,
fld7: u16,

},
Variant2{
fld0: [u8; 5],
fld1: (u16, u16, i8, [i16; 4], (*mut usize, u16, i128)),
fld2: Adt47,
fld3: Adt44,
fld4: *mut usize,
fld5: *mut i32,

},
Variant3{
fld0: *mut i32,
fld1: char,
fld2: Adt49,
fld3: Adt46,
fld4: u16,
fld5: [i128; 7],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: (i16, f32),
fld1: *mut i32,
fld2: i128,
fld3: *mut bool,
fld4: i16,
fld5: Adt45,
fld6: (i16,),

},
Variant1{
fld0: f64,

},
Variant2{
fld0: i128,
fld1: (i16, f32),
fld2: Adt49,
fld3: u64,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: u32,
fld1: *const u128,
fld2: [i32; 2],
fld3: Adt49,
fld4: i16,
fld5: (i16, f32),
fld6: [i128; 7],

},
Variant1{
fld0: *mut *mut i32,

},
Variant2{
fld0: u32,
fld1: Adt45,
fld2: Adt54,
fld3: usize,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *mut *mut i32,
fld1: char,
fld2: Adt50,
fld3: i8,
fld4: (i16, f32),
fld5: i32,

},
Variant1{
fld0: *const (u8, u32),
fld1: Adt55,
fld2: Adt53,

},
Variant2{
fld0: (i16,),
fld1: char,
fld2: Adt49,
fld3: f32,
fld4: Adt52,
fld5: f64,
fld6: Adt45,
fld7: *const (u8, u32),

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u8,
fld1: i32,
fld2: *mut bool,

},
Variant1{
fld0: [u16; 2],
fld1: Adt52,
fld2: f64,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: *mut *mut i32,
fld1: usize,
fld2: [i128; 7],
fld3: u64,
fld4: i16,
fld5: *const u128,
fld6: u8,

},
Variant1{
fld0: Adt49,
fld1: char,
fld2: isize,
fld3: *mut usize,
fld4: u128,
fld5: (*mut usize, u16, i128),
fld6: f64,

},
Variant2{
fld0: (i128, f32),
fld1: (i16,),
fld2: Adt56,
fld3: [i128; 7],

},
Variant3{
fld0: *mut usize,
fld1: char,
fld2: *const (u8, u32),
fld3: Adt56,
fld4: [u128; 8],
fld5: i32,
fld6: Adt44,
fld7: (i128, f32),

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *mut *mut i32,

},
Variant1{
fld0: *const u128,
fld1: *mut bool,
fld2: [u8; 5],
fld3: i128,

},
Variant2{
fld0: u8,
fld1: *mut i32,
fld2: u32,
fld3: i8,
fld4: f32,
fld5: Adt58,
fld6: i64,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: (*mut i32, [char; 8], (i16, *mut usize), (u8, u32), f64),
fld1: [char; 5],
fld2: *mut bool,
fld3: f32,
fld4: (u8, u32),
fld5: *mut usize,
fld6: (u16, u16, i8, [i16; 4], (*mut usize, u16, i128)),
fld7: Adt44,

},
Variant1{
fld0: i32,
fld1: char,
fld2: (*mut usize, u16, i128),
fld3: *const u128,

},
Variant2{
fld0: Adt59,
fld1: *mut i32,
fld2: Adt58,
fld3: Adt49,

}}

