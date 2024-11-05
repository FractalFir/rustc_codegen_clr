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
pub fn fn0(mut _1: u16,mut _2: i128,mut _3: u64,mut _4: u32,mut _5: i16,mut _6: i32) -> i32 {
mir! {
type RET = i32;
let _7: usize;
let _8: usize;
let _9: ([i64; 4], [bool; 7]);
let _10: usize;
let _11: i64;
let _12: *mut i8;
let _13: u64;
let _14: i128;
let _15: (isize,);
let _16: [u32; 8];
let _17: [u32; 8];
let _18: i16;
let _19: (*const u128, u128, usize);
let _20: u8;
let _21: [i64; 4];
let _22: char;
let _23: [u128; 1];
let _24: [i64; 4];
let _25: f64;
let _26: ();
let _27: ();
{
_4 = 230490761_u32 * 370061777_u32;
_1 = !16382_u16;
_5 = (-19509_i16);
_3 = 15562962014568474211_u64;
_3 = '\u{f7d69}' as u64;
_2 = !(-69258254812880019615145480754389960279_i128);
RET = !(-1629614885_i32);
_1 = 49679_u16;
_6 = RET & RET;
_3 = 17460173959099119766_u64;
_6 = RET;
_8 = !5_usize;
RET = -_6;
_1 = 41468_u16 & 17495_u16;
_2 = 132454112569114852288038572277591312196_i128;
_1 = false as u16;
_3 = !6999048032819113936_u64;
_5 = 4069_i16;
_5 = 32746_i16 - (-2679_i16);
RET = '\u{4f376}' as i32;
RET = (-82_i8) as i32;
RET = _6;
_5 = 27652_i16 | (-4141_i16);
RET = !_6;
_1 = _8 as u16;
Call(_5 = fn1(_4, _2, _2, _2, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = (-114_i8) as usize;
_6 = _3 as i32;
_1 = 43960_u16 | 17227_u16;
RET = !_6;
_1 = (-4433774098088644698_i64) as u16;
_8 = _7;
_2 = 72649524172594081554453837811828510925_i128 & (-96386642466882701174730871515510899583_i128);
_4 = !557080956_u32;
_8 = 74_i8 as usize;
_9.1 = [true,true,true,false,false,true,true];
_1 = 6994_u16;
_4 = !1769958520_u32;
_9.0 = [4748030520027557760_i64,773448799604606323_i64,8696891869297400875_i64,5949213992379585522_i64];
_9.1 = [true,true,true,true,false,false,true];
_8 = !_7;
_6 = RET + RET;
_7 = '\u{ef6c}' as usize;
_8 = !_7;
match _1 {
0 => bb2,
6994 => bb4,
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
_8 = RET as usize;
_2 = 80529258733869472847056861660346884303_i128;
_1 = '\u{40bd8}' as u16;
RET = -_6;
RET = _6 ^ _6;
_4 = !4013648263_u32;
RET = !_6;
_7 = _8;
_3 = !2494063447154231810_u64;
_5 = 13090_i16;
_8 = '\u{648d9}' as usize;
_2 = !164712848015655945853051004654333779774_i128;
_7 = false as usize;
_6 = RET;
Goto(bb5)
}
bb5 = {
_10 = !_7;
_6 = !RET;
RET = _6 + _6;
RET = '\u{53b4}' as i32;
_8 = _4 as usize;
_8 = (-18_i8) as usize;
_8 = !_10;
_9.0 = [5987228749821332695_i64,7077618605372692110_i64,7746918391580266435_i64,8889486037366852067_i64];
_8 = _7;
_9.1 = [false,true,false,false,false,true,true];
_6 = 9223372036854775807_isize as i32;
RET = _6;
_6 = -RET;
RET = _6 + _6;
_11 = _3 as i64;
_2 = -(-87061623631831560137908853890899508673_i128);
_5 = 8332_i16 - 18203_i16;
_1 = 48368_u16 << _11;
RET = _6;
_5 = _11 as i16;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = _7;
RET = _6 ^ _6;
_7 = !_8;
_1 = 61799_u16 * 12757_u16;
_5 = _4 as i16;
_9.0 = [_11,_11,_11,_11];
_6 = RET | RET;
_6 = RET;
_7 = _10;
_2 = _8 as i128;
_3 = 14399264918393179489_u64;
match _3 {
0 => bb4,
1 => bb7,
2 => bb8,
14399264918393179489 => bb10,
_ => bb9
}
}
bb7 = {
_10 = !_7;
_6 = !RET;
RET = _6 + _6;
RET = '\u{53b4}' as i32;
_8 = _4 as usize;
_8 = (-18_i8) as usize;
_8 = !_10;
_9.0 = [5987228749821332695_i64,7077618605372692110_i64,7746918391580266435_i64,8889486037366852067_i64];
_8 = _7;
_9.1 = [false,true,false,false,false,true,true];
_6 = 9223372036854775807_isize as i32;
RET = _6;
_6 = -RET;
RET = _6 + _6;
_11 = _3 as i64;
_2 = -(-87061623631831560137908853890899508673_i128);
_5 = 8332_i16 - 18203_i16;
_1 = 48368_u16 << _11;
RET = _6;
_5 = _11 as i16;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_8 = RET as usize;
_2 = 80529258733869472847056861660346884303_i128;
_1 = '\u{40bd8}' as u16;
RET = -_6;
RET = _6 ^ _6;
_4 = !4013648263_u32;
RET = !_6;
_7 = _8;
_3 = !2494063447154231810_u64;
_5 = 13090_i16;
_8 = '\u{648d9}' as usize;
_2 = !164712848015655945853051004654333779774_i128;
_7 = false as usize;
_6 = RET;
Goto(bb5)
}
bb9 = {
Return()
}
bb10 = {
_4 = 2799654084_u32;
_5 = 125979705946107706336069742628232283692_u128 as i16;
_4 = !2780181920_u32;
_4 = (-70_i8) as u32;
_13 = !_3;
_13 = _3 & _3;
_15 = ((-9223372036854775808_isize),);
_4 = '\u{faa5a}' as u32;
_9.0 = [_11,_11,_11,_11];
_9.0 = [_11,_11,_11,_11];
_15 = ((-9223372036854775808_isize),);
RET = _2 as i32;
_9.0 = [_11,_11,_11,_11];
_8 = !_10;
_10 = _8 * _8;
_7 = 217_u8 as usize;
_4 = 522826083_u32 >> _10;
RET = 173_u8 as i32;
_10 = _8;
RET = _6 * _6;
_10 = _8;
_8 = _2 as usize;
_7 = _10 >> _3;
_17 = [_4,_4,_4,_4,_4,_4,_4,_4];
Goto(bb11)
}
bb11 = {
_16 = _17;
_16 = [_4,_4,_4,_4,_4,_4,_4,_4];
_17 = _16;
_19.1 = 95355662481537516296027873413949858526_u128 | 59234597890280541765061582231193885836_u128;
_17 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = !_6;
_19.1 = 278194465982415027344337129136683144252_u128 << _8;
_19.2 = _8;
_2 = !132375665202907213839758519530800315877_i128;
_15.0 = false as isize;
_14 = _13 as i128;
_8 = !_7;
_9.1 = [false,false,true,false,true,true,true];
_17 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = _13 as i32;
_2 = _3 as i128;
_19.0 = core::ptr::addr_of!(_19.1);
_15.0 = (-9223372036854775808_isize);
_17 = [_4,_4,_4,_4,_4,_4,_4,_4];
_10 = _8 + _8;
match _15.0 {
0 => bb9,
1 => bb8,
2 => bb3,
3 => bb10,
4 => bb12,
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
}
bb12 = {
_10 = !_7;
_6 = !RET;
RET = _6 + _6;
RET = '\u{53b4}' as i32;
_8 = _4 as usize;
_8 = (-18_i8) as usize;
_8 = !_10;
_9.0 = [5987228749821332695_i64,7077618605372692110_i64,7746918391580266435_i64,8889486037366852067_i64];
_8 = _7;
_9.1 = [false,true,false,false,false,true,true];
_6 = 9223372036854775807_isize as i32;
RET = _6;
_6 = -RET;
RET = _6 + _6;
_11 = _3 as i64;
_2 = -(-87061623631831560137908853890899508673_i128);
_5 = 8332_i16 - 18203_i16;
_1 = 48368_u16 << _11;
RET = _6;
_5 = _11 as i16;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb13 = {
_7 = (-114_i8) as usize;
_6 = _3 as i32;
_1 = 43960_u16 | 17227_u16;
RET = !_6;
_1 = (-4433774098088644698_i64) as u16;
_8 = _7;
_2 = 72649524172594081554453837811828510925_i128 & (-96386642466882701174730871515510899583_i128);
_4 = !557080956_u32;
_8 = 74_i8 as usize;
_9.1 = [true,true,true,false,false,true,true];
_1 = 6994_u16;
_4 = !1769958520_u32;
_9.0 = [4748030520027557760_i64,773448799604606323_i64,8696891869297400875_i64,5949213992379585522_i64];
_9.1 = [true,true,true,true,false,false,true];
_8 = !_7;
_6 = RET + RET;
_7 = '\u{ef6c}' as usize;
_8 = !_7;
match _1 {
0 => bb2,
6994 => bb4,
_ => bb3
}
}
bb14 = {
_15 = ((-44_isize),);
_8 = _11 as usize;
_4 = !3235475024_u32;
_7 = _10;
_8 = _11 as usize;
_18 = _5;
_3 = _13 ^ _13;
_15 = (9223372036854775807_isize,);
RET = !_6;
_17 = [_4,_4,_4,_4,_4,_4,_4,_4];
_18 = _5;
_22 = '\u{8daf3}';
_21 = [_11,_11,_11,_11];
_7 = _10;
_14 = _3 as i128;
_15 = ((-9223372036854775808_isize),);
_9.1 = [true,true,false,false,false,false,false];
_17 = [_4,_4,_4,_4,_4,_4,_4,_4];
_9.1 = [true,false,false,true,true,false,false];
_5 = 195_u8 as i16;
_9.0 = [_11,_11,_11,_11];
_4 = 1644712995_u32;
_10 = _7;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(0_usize, 5_usize, Move(_5), 11_usize, Move(_11), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(0_usize, 22_usize, Move(_22), 10_usize, Move(_10), 16_usize, Move(_16), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(0_usize, 18_usize, Move(_18), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u32,mut _2: i128,mut _3: i128,mut _4: i128,mut _5: u16) -> i16 {
mir! {
type RET = i16;
let _6: f32;
let _7: [i64; 4];
let _8: isize;
let _9: isize;
let _10: (i32, (isize, isize));
let _11: isize;
let _12: Adt44;
let _13: [u128; 1];
let _14: Adt50;
let _15: Adt53;
let _16: f32;
let _17: [u128; 1];
let _18: char;
let _19: isize;
let _20: isize;
let _21: f32;
let _22: bool;
let _23: i16;
let _24: i32;
let _25: ();
let _26: ();
{
RET = (-7810_i16) >> _3;
Goto(bb1)
}
bb1 = {
_2 = 8032073843617860946_u64 as i128;
_4 = 2016423626_i32 as i128;
_6 = 86_i8 as f32;
_7 = [(-2523373752760621085_i64),3809080285941212246_i64,7839469718495341263_i64,4914325032572724580_i64];
_6 = 16454500264558633585_usize as f32;
RET = 2982_i16;
_5 = 5214_u16;
Goto(bb2)
}
bb2 = {
_4 = _3 & _3;
_7 = [(-2042622899796510607_i64),(-3677175330757317264_i64),6053540114040029834_i64,(-8872068800955815260_i64)];
_2 = '\u{205c}' as i128;
_5 = 27623_u16;
_10.1 = (59_isize, (-9223372036854775808_isize));
_5 = !60910_u16;
_10.1.0 = -_10.1.1;
_10.1.1 = RET as isize;
_10.0 = (-278949317_i32);
_10.0 = (-1598710512_i32);
_11 = -_10.1.1;
match _3 {
0 => bb3,
1 => bb4,
132454112569114852288038572277591312196 => bb6,
_ => bb5
}
}
bb3 = {
_2 = 8032073843617860946_u64 as i128;
_4 = 2016423626_i32 as i128;
_6 = 86_i8 as f32;
_7 = [(-2523373752760621085_i64),3809080285941212246_i64,7839469718495341263_i64,4914325032572724580_i64];
_6 = 16454500264558633585_usize as f32;
RET = 2982_i16;
_5 = 5214_u16;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_2 = _3;
_9 = 151_u8 as isize;
match _3 {
0 => bb4,
132454112569114852288038572277591312196 => bb7,
_ => bb5
}
}
bb7 = {
_13 = [250163567559261895773287133299332119037_u128];
_5 = 1534811868232458333_u64 as u16;
_8 = _9 << _10.1.0;
_8 = _11;
_14 = Adt50::Variant0 { fld0: _10.1 };
Call(_11 = fn2(_1, _7, Move(_14), _10.1.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_16 = _6 + _6;
_7 = [7302752189162513089_i64,(-4812429702444504472_i64),2031011930683998876_i64,(-6646251426444959699_i64)];
_10.0 = 2035964515_i32;
_10.1.1 = _11 + _10.1.0;
RET = 24239_i16;
_10.1 = (_9, _11);
_1 = 3470761948_u32 + 2245509754_u32;
_14 = Adt50::Variant0 { fld0: _10.1 };
_10.1.0 = !_9;
_10.1.0 = _10.1.1;
_7 = [5220951518610160150_i64,(-2837067848547300095_i64),632581220352616226_i64,151765199218101528_i64];
place!(Field::<(isize, isize)>(Variant(_14, 0), 0)).1 = (-6809628616099066427_i64) as isize;
_13 = [43884199327654464873440205564588654531_u128];
_10 = ((-440285019_i32), Field::<(isize, isize)>(Variant(_14, 0), 0));
_2 = true as i128;
_1 = _5 as u32;
_14 = Adt50::Variant0 { fld0: _10.1 };
_1 = !1250386639_u32;
RET = !(-28097_i16);
_12 = Adt44::Variant3 { fld0: (-64_i8) };
Goto(bb9)
}
bb9 = {
_10.0 = (-816346953_i32) & 101688282_i32;
_10.1 = (_11, _9);
place!(Field::<(isize, isize)>(Variant(_14, 0), 0)) = _10.1;
_10.0 = -(-1333554858_i32);
place!(Field::<(isize, isize)>(Variant(_14, 0), 0)) = (_8, _9);
_10.1.0 = _10.0 as isize;
_6 = _16;
_3 = _4;
_8 = !_9;
_17 = _13;
RET = (-14572_i16);
_3 = -_4;
RET = (-26204_i16);
SetDiscriminant(_14, 2);
Call(_5 = core::intrinsics::transmute(RET), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<i8>(Variant(_12, 3), 0)) = 17_i8 | (-37_i8);
_8 = -_10.1.0;
_10.1.1 = 2794268766131074053_i64 as isize;
_10.0 = -928655374_i32;
place!(Field::<i8>(Variant(_12, 3), 0)) = (-127_i8);
_13 = _17;
_8 = -_10.1.0;
SetDiscriminant(_12, 2);
place!(Field::<[bool; 7]>(Variant(_12, 2), 2)) = [true,true,false,false,false,false,false];
place!(Field::<*const u16>(Variant(_14, 2), 4)) = core::ptr::addr_of!(_5);
match RET {
0 => bb1,
1 => bb5,
2 => bb8,
3 => bb4,
4 => bb11,
5 => bb12,
340282366920938463463374607431768185252 => bb14,
_ => bb13
}
}
bb11 = {
_10.0 = (-816346953_i32) & 101688282_i32;
_10.1 = (_11, _9);
place!(Field::<(isize, isize)>(Variant(_14, 0), 0)) = _10.1;
_10.0 = -(-1333554858_i32);
place!(Field::<(isize, isize)>(Variant(_14, 0), 0)) = (_8, _9);
_10.1.0 = _10.0 as isize;
_6 = _16;
_3 = _4;
_8 = !_9;
_17 = _13;
RET = (-14572_i16);
_3 = -_4;
RET = (-26204_i16);
SetDiscriminant(_14, 2);
Call(_5 = core::intrinsics::transmute(RET), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_2 = 8032073843617860946_u64 as i128;
_4 = 2016423626_i32 as i128;
_6 = 86_i8 as f32;
_7 = [(-2523373752760621085_i64),3809080285941212246_i64,7839469718495341263_i64,4914325032572724580_i64];
_6 = 16454500264558633585_usize as f32;
RET = 2982_i16;
_5 = 5214_u16;
Goto(bb2)
}
bb14 = {
RET = (-23350_i16) | (-14438_i16);
place!(Field::<(bool,)>(Variant(_14, 2), 1)) = (true,);
place!(Field::<[bool; 7]>(Variant(_14, 2), 3)) = Field::<[bool; 7]>(Variant(_12, 2), 2);
place!(Field::<char>(Variant(_12, 2), 1)) = '\u{1b388}';
place!(Field::<[bool; 7]>(Variant(_12, 2), 2)) = Field::<[bool; 7]>(Variant(_14, 2), 3);
_18 = Field::<char>(Variant(_12, 2), 1);
_2 = (-6486232693689744570_i64) as i128;
_10.1.1 = _11;
_6 = 172_u8 as f32;
_20 = _8;
place!(Field::<[i128; 4]>(Variant(_12, 2), 4)) = [_4,_4,_4,_4];
_21 = _16 + _6;
_5 = 12662_u16 - 65307_u16;
_20 = 127_i8 as isize;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(1_usize, 11_usize, Move(_11), 5_usize, Move(_5), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(1_usize, 8_usize, Move(_8), 13_usize, Move(_13), 3_usize, Move(_3), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u32,mut _2: [i64; 4],mut _3: Adt50,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: (isize, isize);
let _6: char;
let _7: char;
let _8: i16;
let _9: f32;
let _10: (bool,);
let _11: [u128; 1];
let _12: i64;
let _13: ([i64; 4], [bool; 7]);
let _14: i32;
let _15: u32;
let _16: *mut usize;
let _17: (isize, isize);
let _18: (f32,);
let _19: (bool,);
let _20: u32;
let _21: bool;
let _22: [u32; 8];
let _23: i16;
let _24: usize;
let _25: [i128; 4];
let _26: f64;
let _27: u16;
let _28: ();
let _29: ();
{
RET = Field::<(isize, isize)>(Variant(_3, 0), 0).0 >> _4;
RET = '\u{a0e12}' as isize;
RET = _4;
_5 = (_4, _4);
SetDiscriminant(_3, 1);
Goto(bb1)
}
bb1 = {
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [68835277564327619020743996010634774410_u128];
_2 = [8881386669542176685_i64,2642120832626694767_i64,3453012151623613753_i64,(-2185942119096127716_i64)];
place!(Field::<(bool,)>(Variant(_3, 1), 0)).0 = !false;
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = [5794283730402483519_i64,3492915562131334938_i64,8728640578034236740_i64,(-8807427613496901097_i64)];
RET = 28_u8 as isize;
_8 = (-13011_i16) ^ (-12301_i16);
_6 = '\u{16a2b}';
_8 = (-62_i8) as i16;
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = _2;
Call(_7 = fn3(_5.0, Field::<(bool,)>(Variant(_3, 1), 0), RET, _4, Field::<(bool,)>(Variant(_3, 1), 0).0, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _5.0;
Goto(bb3)
}
bb3 = {
_2 = Field::<[i64; 4]>(Variant(_3, 1), 4);
_5 = (_4, _4);
_3 = Adt50::Variant0 { fld0: _5 };
_5 = (Field::<(isize, isize)>(Variant(_3, 0), 0).0, Field::<(isize, isize)>(Variant(_3, 0), 0).1);
SetDiscriminant(_3, 1);
_4 = !RET;
_7 = _6;
_12 = 38423995134045706_i64 + 2861373786766873162_i64;
_7 = _6;
_1 = _8 as u32;
_14 = !68654291_i32;
_6 = _7;
_10.0 = !true;
_8 = 11976_i16 >> _5.1;
_11 = [285686342337995995099931894800273360478_u128];
RET = _5.0;
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = _11;
place!(Field::<(bool,)>(Variant(_3, 1), 0)).0 = !_10.0;
_9 = 17268920301795163153_u64 as f32;
RET = _5.0 << _4;
_7 = _6;
_10 = (Field::<(bool,)>(Variant(_3, 1), 0).0,);
_2 = [_12,_12,_12,_12];
Goto(bb4)
}
bb4 = {
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = _11;
_5.1 = _5.0;
Goto(bb5)
}
bb5 = {
_10 = Field::<(bool,)>(Variant(_3, 1), 0);
_11 = [138582047528825652890517889927484237430_u128];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = _2;
place!(Field::<Adt45>(Variant(_3, 1), 2)) = Adt45::Variant2 { fld0: 169_u8,fld1: Field::<[i64; 4]>(Variant(_3, 1), 4) };
_9 = 73519273558239426121312941863027293621_u128 as f32;
_10.0 = Field::<(bool,)>(Variant(_3, 1), 0).0;
_10.0 = !Field::<(bool,)>(Variant(_3, 1), 0).0;
_15 = _1 * _1;
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [122570930157673400171787191266015138059_u128];
_5 = (RET, RET);
_6 = _7;
RET = _9 as isize;
_13.0 = [_12,_12,_12,_12];
Goto(bb6)
}
bb6 = {
_18 = (_9,);
_7 = _6;
place!(Field::<u8>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 2), 0)) = _5.0 as u8;
_5 = (RET, RET);
SetDiscriminant(Field::<Adt45>(Variant(_3, 1), 2), 0);
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [57719020625080853668713094269471368919_u128];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = [_12,_12,_12,_12];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = [_12,_12,_12,_12];
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
_18 = (_9,);
_17.1 = _5.1;
RET = _17.1 >> _5.1;
place!(Field::<bool>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 0)) = !Field::<(bool,)>(Variant(_3, 1), 0).0;
Goto(bb7)
}
bb7 = {
place!(Field::<bool>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 0)) = !_10.0;
_17.0 = _4;
_10.0 = _8 != _8;
_13.1 = [Field::<(bool,)>(Variant(_3, 1), 0).0,_10.0,_10.0,Field::<(bool,)>(Variant(_3, 1), 0).0,_10.0,_10.0,_10.0];
_7 = _6;
_12 = (-2690770198635261999_i64) & (-7593760249601856331_i64);
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
place!(Field::<(bool,)>(Variant(_3, 1), 0)).0 = !_10.0;
_18 = (_9,);
_23 = -_8;
_19 = (_10.0,);
_5 = (RET, _17.0);
_9 = _18.0;
_9 = -_18.0;
_5 = (RET, _17.0);
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [179429084614600006538424969891367231272_u128];
_5 = (RET, RET);
_19.0 = _10.0;
Call(_5.0 = fn19(_10.0, _18, _19.0, _17.0, Field::<(bool,)>(Variant(_3, 1), 0), _11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_19.0 = !_10.0;
_5.0 = 27586_u16 as isize;
_6 = _7;
_13.0 = [_12,_12,_12,_12];
_13.0 = [_12,_12,_12,_12];
Goto(bb9)
}
bb9 = {
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
Goto(bb10)
}
bb10 = {
_24 = 1_usize;
_19.0 = Field::<(bool,)>(Variant(_3, 1), 0).0;
place!(Field::<[i128; 4]>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 4))[_24] = 22432_u16 as i128;
_2 = Field::<[i64; 4]>(Variant(_3, 1), 4);
RET = _4;
_9 = _8 as f32;
place!(Field::<[i128; 4]>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 4))[_24] = _18.0 as i128;
_11 = [81616144843006472884108769667582216139_u128];
_17.0 = -_5.1;
place!(Field::<[i64; 4]>(Variant(_3, 1), 4))[_24] = -_13.0[_24];
place!(Field::<i8>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 3)) = -(-112_i8);
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_19.0,);
_9 = -_18.0;
_16 = core::ptr::addr_of_mut!(_24);
RET = !_5.1;
match _24 {
0 => bb7,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
1 => bb18,
_ => bb17
}
}
bb11 = {
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
Goto(bb10)
}
bb12 = {
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [68835277564327619020743996010634774410_u128];
_2 = [8881386669542176685_i64,2642120832626694767_i64,3453012151623613753_i64,(-2185942119096127716_i64)];
place!(Field::<(bool,)>(Variant(_3, 1), 0)).0 = !false;
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = [5794283730402483519_i64,3492915562131334938_i64,8728640578034236740_i64,(-8807427613496901097_i64)];
RET = 28_u8 as isize;
_8 = (-13011_i16) ^ (-12301_i16);
_6 = '\u{16a2b}';
_8 = (-62_i8) as i16;
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = _2;
Call(_7 = fn3(_5.0, Field::<(bool,)>(Variant(_3, 1), 0), RET, _4, Field::<(bool,)>(Variant(_3, 1), 0).0, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
place!(Field::<bool>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 0)) = !_10.0;
_17.0 = _4;
_10.0 = _8 != _8;
_13.1 = [Field::<(bool,)>(Variant(_3, 1), 0).0,_10.0,_10.0,Field::<(bool,)>(Variant(_3, 1), 0).0,_10.0,_10.0,_10.0];
_7 = _6;
_12 = (-2690770198635261999_i64) & (-7593760249601856331_i64);
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
place!(Field::<(bool,)>(Variant(_3, 1), 0)).0 = !_10.0;
_18 = (_9,);
_23 = -_8;
_19 = (_10.0,);
_5 = (RET, _17.0);
_9 = _18.0;
_9 = -_18.0;
_5 = (RET, _17.0);
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [179429084614600006538424969891367231272_u128];
_5 = (RET, RET);
_19.0 = _10.0;
Call(_5.0 = fn19(_10.0, _18, _19.0, _17.0, Field::<(bool,)>(Variant(_3, 1), 0), _11), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_18 = (_9,);
_7 = _6;
place!(Field::<u8>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 2), 0)) = _5.0 as u8;
_5 = (RET, RET);
SetDiscriminant(Field::<Adt45>(Variant(_3, 1), 2), 0);
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [57719020625080853668713094269471368919_u128];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = [_12,_12,_12,_12];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = [_12,_12,_12,_12];
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
_18 = (_9,);
_17.1 = _5.1;
RET = _17.1 >> _5.1;
place!(Field::<bool>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 0)) = !Field::<(bool,)>(Variant(_3, 1), 0).0;
Goto(bb7)
}
bb15 = {
_10 = Field::<(bool,)>(Variant(_3, 1), 0);
_11 = [138582047528825652890517889927484237430_u128];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4)) = _2;
place!(Field::<Adt45>(Variant(_3, 1), 2)) = Adt45::Variant2 { fld0: 169_u8,fld1: Field::<[i64; 4]>(Variant(_3, 1), 4) };
_9 = 73519273558239426121312941863027293621_u128 as f32;
_10.0 = Field::<(bool,)>(Variant(_3, 1), 0).0;
_10.0 = !Field::<(bool,)>(Variant(_3, 1), 0).0;
_15 = _1 * _1;
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = [122570930157673400171787191266015138059_u128];
_5 = (RET, RET);
_6 = _7;
RET = _9 as isize;
_13.0 = [_12,_12,_12,_12];
Goto(bb6)
}
bb16 = {
place!(Field::<[u128; 1]>(Variant(_3, 1), 3)) = _11;
_5.1 = _5.0;
Goto(bb5)
}
bb17 = {
RET = _5.0;
Goto(bb3)
}
bb18 = {
_17 = (RET, _5.0);
place!(Field::<(bool,)>(Variant(_3, 1), 0)) = (_10.0,);
_18.0 = _9;
place!(Field::<[i128; 4]>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 4)) = [(-65514787937367741951110174678902982669_i128),(-89148294700023954038695025529921810970_i128),52221619014515929816881684239441624140_i128,(-28266980288229279155705914945537777369_i128)];
place!(Field::<[i64; 4]>(Variant(_3, 1), 4))[_24] = _13.0[_24];
_21 = _19.0 | _19.0;
_22[_24] = _13.0[_24] as u32;
_26 = 9892244390734498615_u64 as f64;
_22 = [_1,_15,_15,_15,_15,_15,_15,_15];
_2[_24] = Field::<[i64; 4]>(Variant(_3, 1), 4)[_24];
_16 = core::ptr::addr_of_mut!((*_16));
_1 = !_22[_24];
place!(Field::<[i128; 4]>(Variant(place!(Field::<Adt45>(Variant(_3, 1), 2)), 0), 4)) = [163884281840286003225046532062741384803_i128,(-157336532802115504937461805367310401432_i128),(-169599260911989429432227674882722505075_i128),(-15823803528884117590412650815241674979_i128)];
place!(Field::<Adt45>(Variant(_3, 1), 2)) = Adt45::Variant2 { fld0: 51_u8,fld1: _13.0 };
_11 = Field::<[u128; 1]>(Variant(_3, 1), 3);
_20 = _26 as u32;
_3 = Adt50::Variant0 { fld0: _17 };
_27 = 2700_u16 - 51357_u16;
Goto(bb19)
}
bb19 = {
Call(_28 = dump_var(2_usize, 6_usize, Move(_6), 17_usize, Move(_17), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(2_usize, 7_usize, Move(_7), 14_usize, Move(_14), 21_usize, Move(_21), 23_usize, Move(_23)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_28 = dump_var(2_usize, 10_usize, Move(_10), 4_usize, Move(_4), 29_usize, _29, 29_usize, _29), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: (bool,),mut _3: isize,mut _4: isize,mut _5: bool,mut _6: u32) -> char {
mir! {
type RET = char;
let _7: (bool,);
let _8: *mut u128;
let _9: [u32; 8];
let _10: [i64; 4];
let _11: Adt43;
let _12: isize;
let _13: Adt54;
let _14: (isize,);
let _15: *mut u128;
let _16: [i128; 4];
let _17: (f32,);
let _18: Adt53;
let _19: i16;
let _20: i32;
let _21: char;
let _22: f64;
let _23: isize;
let _24: usize;
let _25: (i32, (isize, isize));
let _26: [u32; 8];
let _27: (isize,);
let _28: Adt42;
let _29: (bool,);
let _30: ([i64; 4], [bool; 7]);
let _31: i128;
let _32: [bool; 7];
let _33: ();
let _34: ();
{
RET = '\u{67035}';
_2 = (_5,);
_5 = !_2.0;
_3 = _1 + _1;
_2.0 = _5 > _5;
_2 = (_5,);
_4 = 17768035127143885563_u64 as isize;
RET = '\u{4b827}';
_3 = (-673461123_i32) as isize;
_3 = -_4;
_7 = _2;
RET = '\u{5dbb4}';
_2 = _7;
Goto(bb1)
}
bb1 = {
RET = '\u{48cff}';
RET = '\u{be868}';
_5 = _2.0;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb2 = {
_7 = (_5,);
_6 = !3928973485_u32;
_10 = [(-1750376149675614125_i64),(-8214681246033414203_i64),8648504855984048018_i64,38510742032278207_i64];
_10 = [5540207466996524992_i64,(-4101586342562439888_i64),(-8014946530809171824_i64),(-4348133523369094704_i64)];
Goto(bb3)
}
bb3 = {
_7.0 = !_5;
_1 = _4;
_10 = [(-8514165944186024307_i64),(-7526248238189933371_i64),1819437635406052757_i64,3719139533371483141_i64];
_10 = [3948485176130197358_i64,848607358727744590_i64,(-3132069300908547435_i64),(-762552191477589738_i64)];
RET = '\u{69b8f}';
RET = '\u{1607c}';
_10 = [2752170624404565912_i64,(-6890726734785450740_i64),(-2730109279106573818_i64),12935121382041246_i64];
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_6 = _1 as u32;
_7 = (_2.0,);
_5 = _7.0;
Call(_9 = fn4(_2.0, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = '\u{436b7}';
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_12 = !_3;
_7.0 = _5;
RET = '\u{bd508}';
_5 = RET < RET;
_4 = 109_i8 as isize;
_6 = 3209503075_u32 | 3676978775_u32;
_6 = (-92_i8) as u32;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_3 = _4 - _1;
_1 = -_12;
_14.0 = 41975_u16 as isize;
_5 = _3 != _3;
_7 = _2;
_14.0 = -_1;
_12 = (-4172_i16) as isize;
_14 = (_3,);
_1 = _12 ^ _14.0;
_2 = (_5,);
_19 = (-30555_i16) * 18877_i16;
_6 = 147603617177731197925692004169364572530_i128 as u32;
Goto(bb5)
}
bb5 = {
_14.0 = 25356_u16 as isize;
_5 = _2.0;
_14 = (_1,);
_20 = (-186694417_i32);
_17.0 = 226603779425898936620095747958825116458_u128 as f32;
_16 = [45665275894111829032463391004781388923_i128,(-123474674678712139385009415953329201270_i128),(-57678716876989680944272691245401103843_i128),60516055144500828201743223352430784188_i128];
_7 = (_2.0,);
_5 = !_2.0;
_21 = RET;
match _20 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
340282366920938463463374607431581517039 => bb8,
_ => bb7
}
}
bb6 = {
RET = '\u{48cff}';
RET = '\u{be868}';
_5 = _2.0;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb7 = {
_7 = (_5,);
_6 = !3928973485_u32;
_10 = [(-1750376149675614125_i64),(-8214681246033414203_i64),8648504855984048018_i64,38510742032278207_i64];
_10 = [5540207466996524992_i64,(-4101586342562439888_i64),(-8014946530809171824_i64),(-4348133523369094704_i64)];
Goto(bb3)
}
bb8 = {
_1 = !_12;
_12 = _1 << _6;
_22 = (-111614794233608404906568043505952051034_i128) as f64;
_17.0 = (-104823924249501412110500659953733731454_i128) as f32;
_22 = 1416434592068499387_u64 as f64;
_12 = _5 as isize;
_22 = 132_u8 as f64;
Goto(bb9)
}
bb9 = {
_19 = 2333_i16;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_22 = (-149953403882979681585636500771030619135_i128) as f64;
_23 = !_12;
Goto(bb10)
}
bb10 = {
_2 = _7;
_17.0 = _6 as f32;
_25.1.1 = !_1;
RET = _21;
_24 = 2_usize ^ 4344165062514037645_usize;
_22 = _24 as f64;
_21 = RET;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_2.0 = _5 | _7.0;
_6 = _24 as u32;
_21 = RET;
_24 = 107342784288911280563802571101540757468_i128 as usize;
_7 = (_2.0,);
_25.1.1 = !_4;
_25.1.0 = 46623_u16 as isize;
match _19 {
0 => bb8,
2333 => bb11,
_ => bb5
}
}
bb11 = {
_2.0 = _7.0;
_19 = (-20620_i16) << _23;
_7 = _2;
_7.0 = _19 != _19;
_25.0 = _20;
_12 = _21 as isize;
RET = _21;
RET = _21;
_6 = 105387025322799414508778114885157363072_u128 as u32;
_2 = (_7.0,);
_21 = RET;
_4 = _23;
_21 = RET;
_7.0 = _4 >= _23;
RET = _21;
_25.0 = _20 & _20;
_2.0 = _7.0;
_14.0 = _3 & _4;
_2.0 = _5;
_3 = !_4;
_26 = _9;
match _20 {
0 => bb1,
1 => bb4,
340282366920938463463374607431581517039 => bb12,
_ => bb7
}
}
bb12 = {
_10 = [1523240003184805651_i64,1328306843882841131_i64,(-2063856691013626267_i64),(-4819562954412592083_i64)];
_24 = _22 as usize;
_22 = 35_i8 as f64;
_14.0 = _25.1.1 * _4;
_28 = Adt42::Variant1 { fld0: _16 };
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_29 = (_2.0,);
_25.1.0 = _14.0;
SetDiscriminant(_28, 1);
_30.1 = [_7.0,_5,_29.0,_5,_7.0,_29.0,_2.0];
_30.0 = _10;
_27 = (_25.1.0,);
_12 = _3 & _25.1.0;
match _20 {
0 => bb13,
1 => bb14,
2 => bb15,
340282366920938463463374607431581517039 => bb17,
_ => bb16
}
}
bb13 = {
RET = '\u{48cff}';
RET = '\u{be868}';
_5 = _2.0;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb14 = {
_2 = _7;
_17.0 = _6 as f32;
_25.1.1 = !_1;
RET = _21;
_24 = 2_usize ^ 4344165062514037645_usize;
_22 = _24 as f64;
_21 = RET;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_2.0 = _5 | _7.0;
_6 = _24 as u32;
_21 = RET;
_24 = 107342784288911280563802571101540757468_i128 as usize;
_7 = (_2.0,);
_25.1.1 = !_4;
_25.1.0 = 46623_u16 as isize;
match _19 {
0 => bb8,
2333 => bb11,
_ => bb5
}
}
bb15 = {
RET = '\u{436b7}';
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_12 = !_3;
_7.0 = _5;
RET = '\u{bd508}';
_5 = RET < RET;
_4 = 109_i8 as isize;
_6 = 3209503075_u32 | 3676978775_u32;
_6 = (-92_i8) as u32;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_3 = _4 - _1;
_1 = -_12;
_14.0 = 41975_u16 as isize;
_5 = _3 != _3;
_7 = _2;
_14.0 = -_1;
_12 = (-4172_i16) as isize;
_14 = (_3,);
_1 = _12 ^ _14.0;
_2 = (_5,);
_19 = (-30555_i16) * 18877_i16;
_6 = 147603617177731197925692004169364572530_i128 as u32;
Goto(bb5)
}
bb16 = {
_7.0 = !_5;
_1 = _4;
_10 = [(-8514165944186024307_i64),(-7526248238189933371_i64),1819437635406052757_i64,3719139533371483141_i64];
_10 = [3948485176130197358_i64,848607358727744590_i64,(-3132069300908547435_i64),(-762552191477589738_i64)];
RET = '\u{69b8f}';
RET = '\u{1607c}';
_10 = [2752170624404565912_i64,(-6890726734785450740_i64),(-2730109279106573818_i64),12935121382041246_i64];
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_6 = _1 as u32;
_7 = (_2.0,);
_5 = _7.0;
Call(_9 = fn4(_2.0, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb17 = {
_10 = _30.0;
_22 = (-7_i8) as f64;
Goto(bb18)
}
bb18 = {
Call(_33 = dump_var(3_usize, 1_usize, Move(_1), 21_usize, Move(_21), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(3_usize, 6_usize, Move(_6), 2_usize, Move(_2), 5_usize, Move(_5), 27_usize, Move(_27)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(3_usize, 12_usize, Move(_12), 7_usize, Move(_7), 24_usize, Move(_24), 34_usize, _34), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: [i64; 4]) -> [u32; 8] {
mir! {
type RET = [u32; 8];
let _3: u32;
let _4: (isize,);
let _5: isize;
let _6: isize;
let _7: Adt55;
let _8: [bool; 7];
let _9: [bool; 7];
let _10: isize;
let _11: ((i32, (isize, isize)), (isize, isize), u16);
let _12: (f32,);
let _13: [u128; 1];
let _14: Adt48;
let _15: (i32, (isize, isize));
let _16: (f32,);
let _17: ();
let _18: ();
{
RET = [3615365925_u32,966447639_u32,67240471_u32,2614902372_u32,2167658704_u32,1187220260_u32,1337028762_u32,3811928898_u32];
RET = [458719732_u32,1734830741_u32,3145096188_u32,896341445_u32,3870824651_u32,1422922438_u32,1667108593_u32,2967582608_u32];
_2 = [(-6409557301580765713_i64),(-3505502146327589241_i64),(-6487772148918276511_i64),(-7421047815903833766_i64)];
RET = [3014256127_u32,3958730978_u32,2279266142_u32,3821398029_u32,2902561072_u32,941484092_u32,1941134079_u32,171121110_u32];
_4.0 = (-9223372036854775808_isize);
_4 = (9223372036854775807_isize,);
RET = [4234280076_u32,565839523_u32,3013547459_u32,3584146076_u32,2694298032_u32,3734609742_u32,1879508166_u32,3405409584_u32];
RET = [3712755596_u32,1078818194_u32,2567082320_u32,4199016824_u32,535190113_u32,1392680808_u32,668675669_u32,3846441747_u32];
_7.fld0 = [1700868191_u32,520194402_u32,3954366234_u32,356289_u32,1180219147_u32,811810040_u32,3086805012_u32,2159991661_u32];
Call(_7.fld5.3 = core::intrinsics::transmute(_4.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 9733669401730963908_usize as u32;
_7.fld5.2 = 201849994279787665401961467588594310999_u128 + 40158822354070471133402916741097505023_u128;
_7.fld7 = [8435148875535913224_i64,8186692152951835785_i64,2658523206098722446_i64,3115294147921351227_i64];
_7.fld6 = (_4.0, _7.fld5.3);
_8 = [_1,_1,_1,_1,_1,_1,_1];
_7.fld1.1 = !_7.fld5.2;
_3 = 613065426_u32 * 709727089_u32;
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
_7.fld5.1 = 3_usize as f64;
_7.fld6.1 = _4.0 | _7.fld5.3;
_7.fld3 = Adt50::Variant0 { fld0: _7.fld6 };
_7.fld5.3 = _1 as isize;
_2 = [(-1051433462246831681_i64),3512545212614087850_i64,4275228377724757998_i64,(-958280073019570881_i64)];
_7.fld7 = [(-7752779467537935223_i64),1446414736350087113_i64,(-3468898485026787881_i64),4804888181623557307_i64];
_5 = Field::<(isize, isize)>(Variant(_7.fld3, 0), 0).1;
place!(Field::<(isize, isize)>(Variant(_7.fld3, 0), 0)) = _7.fld6;
_6 = 32_i8 as isize;
_2 = [(-6530523226439605935_i64),6419253584180412862_i64,(-5618218119869610038_i64),(-2904363727366034283_i64)];
_5 = 60468_u16 as isize;
_7.fld5.1 = 3873691487665729255_i64 as f64;
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
place!(Field::<(isize, isize)>(Variant(_7.fld3, 0), 0)).0 = _6 * _4.0;
_7.fld1.1 = _7.fld5.2 * _7.fld5.2;
Call(_7.fld7 = fn5(Field::<(isize, isize)>(Variant(_7.fld3, 0), 0).0, _7.fld6.1, _4, Field::<(isize, isize)>(Variant(_7.fld3, 0), 0), _7.fld0, _7.fld6.0, Move(_7.fld3), _7.fld0, _7.fld6.0, _7.fld6.1, _7.fld0, _2, _7.fld6.1, _7.fld0, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.fld6 = (_5, _5);
_7.fld5.2 = _7.fld1.1 * _7.fld1.1;
_4 = (_7.fld5.3,);
_6 = 7485857908528540782_u64 as isize;
_7.fld1.0 = core::ptr::addr_of!(_7.fld1.1);
_7.fld1.1 = _3 as u128;
_7.fld6 = (_6, _7.fld5.3);
_7.fld0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_7.fld2 = 196_u8;
_3 = 3686062308_u32;
_7.fld2 = !186_u8;
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
3686062308 => bb10,
_ => bb9
}
}
bb3 = {
_3 = 9733669401730963908_usize as u32;
_7.fld5.2 = 201849994279787665401961467588594310999_u128 + 40158822354070471133402916741097505023_u128;
_7.fld7 = [8435148875535913224_i64,8186692152951835785_i64,2658523206098722446_i64,3115294147921351227_i64];
_7.fld6 = (_4.0, _7.fld5.3);
_8 = [_1,_1,_1,_1,_1,_1,_1];
_7.fld1.1 = !_7.fld5.2;
_3 = 613065426_u32 * 709727089_u32;
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
_7.fld5.1 = 3_usize as f64;
_7.fld6.1 = _4.0 | _7.fld5.3;
_7.fld3 = Adt50::Variant0 { fld0: _7.fld6 };
_7.fld5.3 = _1 as isize;
_2 = [(-1051433462246831681_i64),3512545212614087850_i64,4275228377724757998_i64,(-958280073019570881_i64)];
_7.fld7 = [(-7752779467537935223_i64),1446414736350087113_i64,(-3468898485026787881_i64),4804888181623557307_i64];
_5 = Field::<(isize, isize)>(Variant(_7.fld3, 0), 0).1;
place!(Field::<(isize, isize)>(Variant(_7.fld3, 0), 0)) = _7.fld6;
_6 = 32_i8 as isize;
_2 = [(-6530523226439605935_i64),6419253584180412862_i64,(-5618218119869610038_i64),(-2904363727366034283_i64)];
_5 = 60468_u16 as isize;
_7.fld5.1 = 3873691487665729255_i64 as f64;
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
place!(Field::<(isize, isize)>(Variant(_7.fld3, 0), 0)).0 = _6 * _4.0;
_7.fld1.1 = _7.fld5.2 * _7.fld5.2;
Call(_7.fld7 = fn5(Field::<(isize, isize)>(Variant(_7.fld3, 0), 0).0, _7.fld6.1, _4, Field::<(isize, isize)>(Variant(_7.fld3, 0), 0), _7.fld0, _7.fld6.0, Move(_7.fld3), _7.fld0, _7.fld6.0, _7.fld6.1, _7.fld0, _2, _7.fld6.1, _7.fld0, _2), ReturnTo(bb2), UnwindUnreachable())
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
_1 = false;
_7.fld6 = (_4.0, _5);
_7.fld6.1 = _4.0;
_7.fld1.1 = !_7.fld5.2;
_7.fld2 = (-846620937_i32) as u8;
_7.fld6 = (_5, _5);
_4.0 = 25_i8 as isize;
_7.fld1.1 = _7.fld5.2 ^ _7.fld5.2;
_11.1 = (_5, _4.0);
_10 = 4776_u16 as isize;
Call(_9 = fn6(_10, _7.fld1.1, _7.fld5.3, _2, _10, _7.fld5.2, _7.fld1.1, _4.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11.1.0 = (-4686611337957714208_i64) as isize;
_7.fld0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_11.2 = 46347_u16;
_7.fld1.2 = 1_usize;
RET = [_3,_3,_3,_3,_3,_3,_3,_3];
_11.1.1 = _1 as isize;
_3 = 3677588869_u32;
_11.0 = ((-1742294196_i32), _11.1);
_10 = _6;
_7.fld6.0 = !_4.0;
_11.1.1 = _7.fld6.1;
_7.fld5.1 = _11.0.0 as f64;
_7.fld2 = 191_u8 + 175_u8;
_11.1.0 = _1 as isize;
_11.1.0 = _7.fld6.1;
_7.fld6 = _11.1;
_4.0 = _11.1.1 >> _7.fld5.2;
_11.0.1.1 = _4.0 & _4.0;
_5 = -_11.0.1.1;
_7.fld1.0 = core::ptr::addr_of!(_7.fld5.2);
Goto(bb12)
}
bb12 = {
_7.fld0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_15.1 = _11.0.1;
_11.0.1.1 = _11.0.0 as isize;
_11.1.1 = _15.1.1 ^ _15.1.1;
_5 = -_4.0;
_11.2 = 35480_u16;
_1 = _15.1.1 > _15.1.1;
match _11.0.0 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb10,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607430025917260 => bb14,
_ => bb13
}
}
bb13 = {
_1 = false;
_7.fld6 = (_4.0, _5);
_7.fld6.1 = _4.0;
_7.fld1.1 = !_7.fld5.2;
_7.fld2 = (-846620937_i32) as u8;
_7.fld6 = (_5, _5);
_4.0 = 25_i8 as isize;
_7.fld1.1 = _7.fld5.2 ^ _7.fld5.2;
_11.1 = (_5, _4.0);
_10 = 4776_u16 as isize;
Call(_9 = fn6(_10, _7.fld1.1, _7.fld5.3, _2, _10, _7.fld5.2, _7.fld1.1, _4.0), ReturnTo(bb11), UnwindUnreachable())
}
bb14 = {
_16.0 = _7.fld1.1 as f32;
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(4_usize, 11_usize, Move(_11), 1_usize, Move(_1), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_17 = dump_var(4_usize, 9_usize, Move(_9), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: (isize,),mut _4: (isize, isize),mut _5: [u32; 8],mut _6: isize,mut _7: Adt50,mut _8: [u32; 8],mut _9: isize,mut _10: isize,mut _11: [u32; 8],mut _12: [i64; 4],mut _13: isize,mut _14: [u32; 8],mut _15: [i64; 4]) -> [i64; 4] {
mir! {
type RET = [i64; 4];
let _16: isize;
let _17: Adt44;
let _18: Adt39;
let _19: ([i64; 4], [bool; 7]);
let _20: f32;
let _21: [i128; 4];
let _22: [u32; 8];
let _23: [bool; 7];
let _24: isize;
let _25: bool;
let _26: (f32,);
let _27: (*mut i8, f64, u128, isize);
let _28: [i128; 4];
let _29: (isize,);
let _30: *const u16;
let _31: Adt55;
let _32: [i64; 4];
let _33: Adt52;
let _34: f64;
let _35: usize;
let _36: (bool,);
let _37: u32;
let _38: usize;
let _39: *mut u128;
let _40: bool;
let _41: f32;
let _42: [i64; 4];
let _43: isize;
let _44: [i64; 4];
let _45: (bool,);
let _46: *const u16;
let _47: *const i64;
let _48: ();
let _49: ();
{
_2 = 5368291219237615918_u64 as isize;
SetDiscriminant(_7, 0);
_15 = [(-3051226010653206593_i64),(-6311933989536150610_i64),5851900208398031273_i64,2844183399952315104_i64];
_2 = -_4.0;
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
9223372036854775807 => bb7,
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
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)) = _4;
RET = _15;
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)).1 = -_4.1;
RET = [6108538003949782608_i64,(-4803180003368586143_i64),2868134419012281914_i64,(-1654462503400235723_i64)];
_10 = 7052018108997949670_i64 as isize;
_8 = [1940502774_u32,3297417519_u32,2900459879_u32,706452546_u32,958270362_u32,2353740451_u32,2942136418_u32,3573800281_u32];
_2 = 57595251524663632982478910420949021734_u128 as isize;
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)).0 = _10 & _4.0;
_9 = -Field::<(isize, isize)>(Variant(_7, 0), 0).0;
_4.1 = _10;
_6 = _9 + _13;
_19.0 = [2820615750111025353_i64,(-8807487378198987437_i64),(-8580175577119358002_i64),3118461926757403738_i64];
_3 = (_13,);
_20 = 7_usize as f32;
_17 = Adt44::Variant3 { fld0: (-88_i8) };
_16 = '\u{1025b7}' as isize;
_1 = -_13;
_4.1 = 1391617056_u32 as isize;
_16 = _9 | _10;
_8 = _5;
_8 = [1664219413_u32,2958116036_u32,2923383360_u32,1100509422_u32,2997740578_u32,3976986046_u32,1734670600_u32,1641923114_u32];
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)).0 = 2211614816_u32 as isize;
Goto(bb8)
}
bb8 = {
_4.0 = !_1;
_20 = (-78863522210455282832819270662604996127_i128) as f32;
_21 = [(-120669394296847643577700666531156832009_i128),25353577248792550895342653997096785862_i128,66055798875284070015287017650817979662_i128,160549223038349393001261444894124317352_i128];
_27.3 = _9 - _1;
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)).1 = 130_u8 as isize;
_22 = _11;
_22 = [2763378948_u32,2316306080_u32,37900383_u32,3463461643_u32,3373834245_u32,2361841499_u32,3875877958_u32,2032649550_u32];
_27.3 = 64398_u16 as isize;
_15 = [(-8116396942686372794_i64),6202109468576597760_i64,2904298977447068092_i64,2733762527594775594_i64];
_12 = [3743170420969959695_i64,(-5648051898248954709_i64),(-9041751886023191219_i64),(-6453848010196470032_i64)];
_12 = RET;
_25 = _9 > _16;
SetDiscriminant(_7, 2);
_24 = -_13;
_21 = [67221760318050440367616372282111882946_i128,114759453568943476872551503875450715454_i128,(-111666333362589389421413768638141344662_i128),136471502668495946395643350680052326242_i128];
_20 = 35778691548845669434877477292331177279_i128 as f32;
_15 = [(-2957825492454816899_i64),(-2240394016146369813_i64),(-4771300610532320882_i64),3133649773717142035_i64];
Goto(bb9)
}
bb9 = {
_31.fld5.3 = 120_i8 as isize;
_26.0 = -_20;
_6 = -_24;
RET = [982706317391871705_i64,7397123233068209924_i64,7488002237834672406_i64,(-3272187621737139015_i64)];
place!(Field::<i8>(Variant(_17, 3), 0)) = !(-11_i8);
_28 = _21;
_25 = !true;
_9 = _1 + _27.3;
SetDiscriminant(_17, 3);
_3.0 = _13 >> _9;
_4 = (_16, _1);
_34 = 161964544133685073001740731837018130396_i128 as f64;
Goto(bb10)
}
bb10 = {
place!(Field::<(bool,)>(Variant(_7, 2), 1)) = (_25,);
_7 = Adt50::Variant0 { fld0: _4 };
_31.fld5.2 = 151741133435187083117642531790676292662_u128 + 195082153642812316948300118425641882987_u128;
_3.0 = 49243_u16 as isize;
_31.fld2 = !64_u8;
_24 = _16;
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)).1 = !_1;
_4 = (_24, _24);
_27.0 = core::ptr::addr_of_mut!(place!(Field::<i8>(Variant(_17, 3), 0)));
_10 = _16 ^ _24;
_19.0 = [8233027626769305132_i64,(-5641541019695389461_i64),(-1184568755725715974_i64),(-8199994759542173874_i64)];
_27.1 = 9774082299602935027_u64 as f64;
RET = [(-2876630510439373558_i64),(-2059436499510270679_i64),2851236612351769459_i64,(-6046495293016507499_i64)];
_34 = _27.1;
_29.0 = 663104304_i32 as isize;
_7 = Adt50::Variant0 { fld0: _4 };
_35 = 8480530456574265532_usize;
_31.fld6.1 = -_4.0;
_31.fld5.1 = -_27.1;
_31.fld0 = [2779929216_u32,528364417_u32,1745655903_u32,4017065904_u32,4078303721_u32,3393910986_u32,2540663772_u32,1254178059_u32];
_31.fld5 = (_27.0, _34, 276536253656459951526024977699363035031_u128, Field::<(isize, isize)>(Variant(_7, 0), 0).0);
_3 = _29;
Goto(bb11)
}
bb11 = {
_36.0 = !_25;
_15 = _19.0;
_36 = (_25,);
_27 = (_31.fld5.0, _31.fld5.1, _31.fld5.2, _24);
_15 = [5858158349512291415_i64,(-6755260419175344030_i64),6639458884914093976_i64,(-1996561481807587775_i64)];
place!(Field::<(isize, isize)>(Variant(_7, 0), 0)).0 = 15794282165900427402_u64 as isize;
_31.fld3 = Move(_7);
_31.fld6.0 = !_10;
_29.0 = _10 & Field::<(isize, isize)>(Variant(_31.fld3, 0), 0).0;
_9 = -_10;
_21 = _28;
_41 = _26.0 * _26.0;
_31.fld6 = _4;
_3.0 = 13169_i16 as isize;
_23 = [_25,_36.0,_36.0,_36.0,_36.0,_36.0,_25];
_3.0 = _9;
_9 = _24;
_32 = [(-1709668684477705771_i64),(-8204742797464097019_i64),5051948995076280299_i64,1318162075646369141_i64];
SetDiscriminant(_31.fld3, 0);
_4.0 = _31.fld6.1 >> _27.3;
_13 = _31.fld6.1;
match _31.fld5.2 {
0 => bb12,
276536253656459951526024977699363035031 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_37 = !2330019098_u32;
place!(Field::<(isize, isize)>(Variant(_31.fld3, 0), 0)) = _4;
_32 = [(-3578992604598482470_i64),(-3897415044485226160_i64),(-2601744590730786066_i64),4173579113794693798_i64];
_4 = (_13, Field::<(isize, isize)>(Variant(_31.fld3, 0), 0).0);
_31.fld3 = Adt50::Variant0 { fld0: _4 };
_31.fld1.2 = !_35;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(5_usize, 15_usize, Move(_15), 22_usize, Move(_22), 2_usize, Move(_2), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(5_usize, 35_usize, Move(_35), 12_usize, Move(_12), 28_usize, Move(_28), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(5_usize, 9_usize, Move(_9), 24_usize, Move(_24), 23_usize, Move(_23), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(5_usize, 4_usize, Move(_4), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: u128,mut _3: isize,mut _4: [i64; 4],mut _5: isize,mut _6: u128,mut _7: u128,mut _8: isize) -> [bool; 7] {
mir! {
type RET = [bool; 7];
let _9: char;
let _10: (isize,);
let _11: f64;
let _12: ([i64; 4], [bool; 7]);
let _13: f32;
let _14: [u32; 8];
let _15: i8;
let _16: *const u16;
let _17: i32;
let _18: bool;
let _19: char;
let _20: ([i64; 4], [bool; 7]);
let _21: i16;
let _22: bool;
let _23: Adt40;
let _24: f64;
let _25: (isize, isize);
let _26: u32;
let _27: f32;
let _28: (isize, isize);
let _29: Adt48;
let _30: u8;
let _31: ();
let _32: ();
{
RET = [false,true,true,true,true,true,true];
_1 = 8267884894617996548_i64 as isize;
_3 = !_5;
RET = [true,false,true,false,false,true,false];
_2 = (-99023360201084650634887735139669623892_i128) as u128;
_9 = '\u{5123e}';
RET = [true,true,true,true,true,true,false];
_3 = _8;
_9 = '\u{8653c}';
_2 = _6;
_2 = _7;
_6 = !_7;
_5 = _3;
_7 = 4_usize as u128;
_1 = _2 as isize;
_5 = _1;
RET = [true,true,true,true,true,false,false];
Call(_9 = fn7(_5, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _6 - _6;
RET = [true,false,false,true,false,false,true];
_2 = 147_u8 as u128;
_2 = !_6;
_7 = !_6;
_3 = _5;
_7 = _2;
_2 = !_7;
_10.0 = !_5;
_3 = _5 << _5;
_12.0 = [8261070362393031512_i64,(-3650822491199388723_i64),1776960543225233115_i64,4321187983564245042_i64];
RET = [true,false,false,true,true,false,true];
RET = [false,true,true,false,true,true,true];
_10.0 = _3;
_6 = !_7;
_3 = (-101_i8) as isize;
_3 = 60_u8 as isize;
_12.1 = RET;
_4 = _12.0;
_11 = 1407623622153401830_i64 as f64;
_8 = _10.0;
_7 = 821957867_u32 as u128;
_6 = _2;
_13 = 34786_u16 as f32;
_3 = 84_i8 as isize;
Goto(bb2)
}
bb2 = {
_10 = (_5,);
_9 = '\u{ec8ec}';
_6 = !_2;
_11 = 38_i8 as f64;
_12.0 = [3158866497736654662_i64,(-7028381806828240272_i64),(-3553638632730599615_i64),7318683314577500613_i64];
_13 = (-7465185645312439409_i64) as f32;
_7 = _2 * _2;
_1 = _8;
_6 = _2;
RET = [false,true,false,false,true,false,false];
_10 = (_8,);
_5 = _1;
_6 = _7 + _7;
_12.0 = [2512979500287400897_i64,(-3172630493190761371_i64),(-419445397818698008_i64),6195466155302843684_i64];
_14 = [4066618404_u32,167398471_u32,3327632893_u32,1397785049_u32,1021232721_u32,609128437_u32,1911956337_u32,3182261117_u32];
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = 0_usize as f64;
_17 = (-34132941_i32) << _6;
_12 = (_4, RET);
_9 = '\u{93119}';
Goto(bb4)
}
bb4 = {
RET = [false,false,true,true,false,true,false];
_1 = _8;
RET = [true,false,false,true,true,false,true];
_10 = (_8,);
_21 = (-24414_i16);
_12.1 = RET;
_12.1 = [false,true,true,false,true,false,false];
_20.0 = [(-2604060959825133074_i64),(-8372260694461256028_i64),(-3919215309157060076_i64),(-1126294169417620265_i64)];
match _21 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431768187042 => bb11,
_ => bb10
}
}
bb5 = {
_11 = 0_usize as f64;
_17 = (-34132941_i32) << _6;
_12 = (_4, RET);
_9 = '\u{93119}';
Goto(bb4)
}
bb6 = {
_10 = (_5,);
_9 = '\u{ec8ec}';
_6 = !_2;
_11 = 38_i8 as f64;
_12.0 = [3158866497736654662_i64,(-7028381806828240272_i64),(-3553638632730599615_i64),7318683314577500613_i64];
_13 = (-7465185645312439409_i64) as f32;
_7 = _2 * _2;
_1 = _8;
_6 = _2;
RET = [false,true,false,false,true,false,false];
_10 = (_8,);
_5 = _1;
_6 = _7 + _7;
_12.0 = [2512979500287400897_i64,(-3172630493190761371_i64),(-419445397818698008_i64),6195466155302843684_i64];
_14 = [4066618404_u32,167398471_u32,3327632893_u32,1397785049_u32,1021232721_u32,609128437_u32,1911956337_u32,3182261117_u32];
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_2 = _6 - _6;
RET = [true,false,false,true,false,false,true];
_2 = 147_u8 as u128;
_2 = !_6;
_7 = !_6;
_3 = _5;
_7 = _2;
_2 = !_7;
_10.0 = !_5;
_3 = _5 << _5;
_12.0 = [8261070362393031512_i64,(-3650822491199388723_i64),1776960543225233115_i64,4321187983564245042_i64];
RET = [true,false,false,true,true,false,true];
RET = [false,true,true,false,true,true,true];
_10.0 = _3;
_6 = !_7;
_3 = (-101_i8) as isize;
_3 = 60_u8 as isize;
_12.1 = RET;
_4 = _12.0;
_11 = 1407623622153401830_i64 as f64;
_8 = _10.0;
_7 = 821957867_u32 as u128;
_6 = _2;
_13 = 34786_u16 as f32;
_3 = 84_i8 as isize;
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
_15 = _11 as i8;
RET = _12.1;
_7 = _6 >> _8;
_3 = -_1;
_22 = _7 <= _7;
_9 = '\u{56f37}';
_20 = _12;
_7 = _2 ^ _6;
_20 = (_12.0, RET);
_14 = [331751435_u32,1267181908_u32,2168606976_u32,3912531486_u32,306793117_u32,3982633427_u32,2837578634_u32,976207089_u32];
_9 = '\u{ca5cc}';
_2 = !_7;
_7 = _2 & _6;
_11 = _21 as f64;
_18 = _5 == _10.0;
match _21 {
0 => bb3,
1 => bb12,
340282366920938463463374607431768187042 => bb14,
_ => bb13
}
}
bb12 = {
_2 = _6 - _6;
RET = [true,false,false,true,false,false,true];
_2 = 147_u8 as u128;
_2 = !_6;
_7 = !_6;
_3 = _5;
_7 = _2;
_2 = !_7;
_10.0 = !_5;
_3 = _5 << _5;
_12.0 = [8261070362393031512_i64,(-3650822491199388723_i64),1776960543225233115_i64,4321187983564245042_i64];
RET = [true,false,false,true,true,false,true];
RET = [false,true,true,false,true,true,true];
_10.0 = _3;
_6 = !_7;
_3 = (-101_i8) as isize;
_3 = 60_u8 as isize;
_12.1 = RET;
_4 = _12.0;
_11 = 1407623622153401830_i64 as f64;
_8 = _10.0;
_7 = 821957867_u32 as u128;
_6 = _2;
_13 = 34786_u16 as f32;
_3 = 84_i8 as isize;
Goto(bb2)
}
bb13 = {
_2 = _6 - _6;
RET = [true,false,false,true,false,false,true];
_2 = 147_u8 as u128;
_2 = !_6;
_7 = !_6;
_3 = _5;
_7 = _2;
_2 = !_7;
_10.0 = !_5;
_3 = _5 << _5;
_12.0 = [8261070362393031512_i64,(-3650822491199388723_i64),1776960543225233115_i64,4321187983564245042_i64];
RET = [true,false,false,true,true,false,true];
RET = [false,true,true,false,true,true,true];
_10.0 = _3;
_6 = !_7;
_3 = (-101_i8) as isize;
_3 = 60_u8 as isize;
_12.1 = RET;
_4 = _12.0;
_11 = 1407623622153401830_i64 as f64;
_8 = _10.0;
_7 = 821957867_u32 as u128;
_6 = _2;
_13 = 34786_u16 as f32;
_3 = 84_i8 as isize;
Goto(bb2)
}
bb14 = {
_26 = 16822253572117568678_u64 as u32;
_12 = (_20.0, RET);
_11 = _15 as f64;
_7 = !_2;
_7 = _2;
_20 = (_12.0, _12.1);
_3 = !_5;
_25.0 = 264396909250491889_usize as isize;
_1 = 235_u8 as isize;
_28.0 = _5 * _3;
_15 = -52_i8;
_28 = (_3, _3);
_6 = _7 - _7;
_20 = (_12.0, RET);
_27 = _13;
_26 = 542154651_u32 << _6;
_12.0 = _20.0;
_20.0 = [1623300375019472534_i64,(-8307310836240510246_i64),7113384290457421309_i64,3462922838886235287_i64];
_5 = _3;
_3 = _27 as isize;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(6_usize, 10_usize, Move(_10), 1_usize, Move(_1), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(6_usize, 3_usize, Move(_3), 8_usize, Move(_8), 7_usize, Move(_7), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(6_usize, 21_usize, Move(_21), 28_usize, Move(_28), 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: u128,mut _3: u128) -> char {
mir! {
type RET = char;
let _4: char;
let _5: (f32,);
let _6: f32;
let _7: i32;
let _8: [u128; 1];
let _9: Adt46;
let _10: [u128; 1];
let _11: ([i64; 4], [bool; 7]);
let _12: Adt55;
let _13: *mut i8;
let _14: isize;
let _15: bool;
let _16: ((i32, (isize, isize)), (isize, isize), u16);
let _17: &'static i64;
let _18: (isize,);
let _19: [u32; 8];
let _20: (*const u128, u128, usize);
let _21: f64;
let _22: i32;
let _23: char;
let _24: Adt42;
let _25: (*mut i8, f64, u128, isize);
let _26: u128;
let _27: u64;
let _28: ((i32, (isize, isize)), (isize, isize), u16);
let _29: [u32; 8];
let _30: char;
let _31: isize;
let _32: Adt39;
let _33: isize;
let _34: f32;
let _35: f32;
let _36: Adt45;
let _37: i16;
let _38: i8;
let _39: Adt49;
let _40: (f32,);
let _41: isize;
let _42: u8;
let _43: Adt50;
let _44: Adt41;
let _45: [u128; 1];
let _46: *mut i8;
let _47: Adt47;
let _48: f64;
let _49: &'static i64;
let _50: usize;
let _51: (f32,);
let _52: (isize,);
let _53: ((i32, (isize, isize)), (isize, isize), u16);
let _54: f32;
let _55: (f32,);
let _56: u64;
let _57: ();
let _58: ();
{
_3 = _2;
_3 = _2 & _2;
_2 = _3;
RET = '\u{991a0}';
_2 = _3 >> _3;
_1 = (-9223372036854775808_isize) << _2;
RET = '\u{f50cb}';
_1 = 9223372036854775807_isize;
_2 = _3;
_2 = _3;
_1 = (-9223372036854775808_isize);
_2 = _3;
_3 = _2 | _2;
_1 = 74_isize + 9223372036854775807_isize;
_3 = _2;
_1 = -9223372036854775807_isize;
RET = '\u{13543}';
_2 = _3 ^ _3;
_6 = 1627057635_u32 as f32;
_3 = !_2;
_5 = (_6,);
_2 = _3 * _3;
_6 = -_5.0;
RET = '\u{e93fe}';
_5.0 = -_6;
RET = '\u{b3ccb}';
Goto(bb1)
}
bb1 = {
RET = '\u{7daa7}';
RET = '\u{e66f2}';
_5 = (_6,);
_1 = 4_u8 as isize;
RET = '\u{f286}';
RET = '\u{59426}';
_4 = RET;
RET = _4;
_5 = (_6,);
_5 = (_6,);
_9.fld0.1 = [true,false,false,true,false,true,true];
_2 = 3786820913_u32 as u128;
_9.fld0.0 = [6397720206549533077_i64,2401978199245611966_i64,4037044385861714624_i64,4990276448768623289_i64];
_1 = !71_isize;
_5 = (_6,);
_9.fld0.1 = [false,true,false,true,false,false,true];
_8 = [_3];
_3 = !_2;
_11.1 = _9.fld0.1;
_10 = _8;
_11.1 = _9.fld0.1;
_12.fld0 = [3353707027_u32,4056300403_u32,2260271544_u32,407149209_u32,4188064140_u32,3428749054_u32,1469506963_u32,1821442069_u32];
_2 = _3;
_6 = _5.0;
_12.fld6.0 = _1 | _1;
Goto(bb2)
}
bb2 = {
_11 = _9.fld0;
_12.fld5.3 = _12.fld6.0;
_9.fld0.0 = [1136546329123452089_i64,(-5208290327067401466_i64),5661803023323416122_i64,6137822569694913277_i64];
_14 = _12.fld5.3;
_5.0 = _6 - _6;
_10 = _8;
_12.fld1.1 = _2 | _2;
_12.fld6.0 = _12.fld5.3;
_12.fld6 = (_12.fld5.3, _12.fld5.3);
_12.fld5.2 = _5.0 as u128;
_12.fld7 = [(-2024361203679685815_i64),6444763529030299121_i64,3894307582311610729_i64,5934933389568369835_i64];
_12.fld1.1 = _12.fld5.2 >> _12.fld5.2;
RET = _4;
_12.fld3 = Adt50::Variant0 { fld0: _12.fld6 };
_18 = (_12.fld6.0,);
_12.fld2 = 179_u8 & 246_u8;
_9.fld0 = (_11.0, _11.1);
_10 = [_12.fld5.2];
_3 = _12.fld1.1;
_18 = (_14,);
_12.fld5.2 = _2;
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_2 = !_3;
_21 = _3 as f64;
_1 = Field::<(isize, isize)>(Variant(_12.fld3, 0), 0).1 ^ _12.fld5.3;
_16.0.1.1 = _18.0 & _1;
_16.1.1 = _1 * _16.0.1.1;
_9.fld1 = Adt39::Variant1 { fld0: 10230752195951579742_u64 };
_28.0 = (1360738130_i32, _12.fld6);
SetDiscriminant(_12.fld3, 1);
_12.fld1.2 = 1_usize;
_16.0 = (_28.0.0, _12.fld6);
_16 = (_28.0, _28.0.1, 3238_u16);
_20.0 = core::ptr::addr_of!(_20.1);
place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)) = Adt45::Variant2 { fld0: _12.fld2,fld1: _12.fld7 };
_23 = _4;
_12.fld1.0 = core::ptr::addr_of!(_3);
place!(Field::<[u128; 1]>(Variant(_12.fld3, 1), 3)) = _8;
_23 = _4;
_9.fld0 = _11;
SetDiscriminant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 2);
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 2), 1)) = _12.fld7;
_12.fld1.2 = 13616274792866860635_usize & 7236921944148446081_usize;
_28.0.1 = (_1, _16.1.0);
_12.fld2 = 221_u8;
_11 = _9.fld0;
_12.fld7 = [650652483254977268_i64,(-8078884541673520854_i64),(-5935452164405561580_i64),6709373175651354466_i64];
Goto(bb5)
}
bb5 = {
_12.fld6 = (_28.0.1.1, _28.0.1.0);
match _16.2 {
0 => bb3,
3238 => bb6,
_ => bb2
}
}
bb6 = {
_28.0.1.0 = !_28.0.1.1;
_8 = [_12.fld1.1];
_20.0 = _12.fld1.0;
_12.fld5.3 = (-4098533211104780455478175285206632130_i128) as isize;
_16.1 = (_28.0.1.1, _28.0.1.1);
_28.0.1 = (_1, _12.fld6.0);
_28.1.0 = _16.1.1 >> _2;
place!(Field::<(bool,)>(Variant(_12.fld3, 1), 0)).0 = _16.0.1.1 != _14;
_4 = _23;
_32 = Adt39::Variant1 { fld0: 8565074204434605903_u64 };
_15 = Field::<(bool,)>(Variant(_12.fld3, 1), 0).0 | Field::<(bool,)>(Variant(_12.fld3, 1), 0).0;
_28.2 = _16.2;
_9.fld0.1 = [_15,_15,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,_15,_15,_15];
_14 = 3895156529_u32 as isize;
_10 = [_2];
_12.fld1.0 = _20.0;
_20.2 = _12.fld1.2;
place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)) = Adt45::Variant2 { fld0: _12.fld2,fld1: _11.0 };
SetDiscriminant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1);
_18 = (_1,);
_12.fld7 = [(-3283619218491810826_i64),2623821993329693365_i64,(-3966770464118808846_i64),6399530668234481765_i64];
_15 = !Field::<(bool,)>(Variant(_12.fld3, 1), 0).0;
_11.1 = _9.fld0.1;
_12.fld5.1 = _21 + _21;
match _28.2 {
3238 => bb8,
_ => bb7
}
}
bb7 = {
_2 = !_3;
_21 = _3 as f64;
_1 = Field::<(isize, isize)>(Variant(_12.fld3, 0), 0).1 ^ _12.fld5.3;
_16.0.1.1 = _18.0 & _1;
_16.1.1 = _1 * _16.0.1.1;
_9.fld1 = Adt39::Variant1 { fld0: 10230752195951579742_u64 };
_28.0 = (1360738130_i32, _12.fld6);
SetDiscriminant(_12.fld3, 1);
_12.fld1.2 = 1_usize;
_16.0 = (_28.0.0, _12.fld6);
_16 = (_28.0, _28.0.1, 3238_u16);
_20.0 = core::ptr::addr_of!(_20.1);
place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)) = Adt45::Variant2 { fld0: _12.fld2,fld1: _12.fld7 };
_23 = _4;
_12.fld1.0 = core::ptr::addr_of!(_3);
place!(Field::<[u128; 1]>(Variant(_12.fld3, 1), 3)) = _8;
_23 = _4;
_9.fld0 = _11;
SetDiscriminant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 2);
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 2), 1)) = _12.fld7;
_12.fld1.2 = 13616274792866860635_usize & 7236921944148446081_usize;
_28.0.1 = (_1, _16.1.0);
_12.fld2 = 221_u8;
_11 = _9.fld0;
_12.fld7 = [650652483254977268_i64,(-8078884541673520854_i64),(-5935452164405561580_i64),6709373175651354466_i64];
Goto(bb5)
}
bb8 = {
_5.0 = _6;
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).0.1 = (_12.fld6.1, _18.0);
_31 = _28.1.0 >> Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.1;
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 3)) = [7407141174987953309_i64,(-3462048382327035041_i64),(-1753644271100890922_i64),(-2167910466797079463_i64)];
_12.fld6 = (Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.0, _31);
_23 = _4;
_34 = _5.0;
place!(Field::<[bool; 7]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 0)) = _9.fld0.1;
place!(Field::<isize>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 2)) = !_28.1.0;
_22 = _28.0.0;
_16.0.1.0 = !_31;
_12.fld1.0 = core::ptr::addr_of!(_26);
_11.0 = _12.fld7;
_23 = RET;
_25.3 = !_28.0.1.0;
place!(Field::<isize>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 2)) = -_1;
_16.0.1.1 = _12.fld2 as isize;
_12.fld2 = 43_u8;
place!(Field::<u128>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 4)) = _2;
_16.0.1.1 = -_16.0.1.0;
_8 = Field::<[u128; 1]>(Variant(_12.fld3, 1), 3);
_4 = RET;
place!(Field::<char>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 1)) = _23;
_26 = !_3;
_12.fld6.1 = _31 ^ _16.0.1.1;
Goto(bb9)
}
bb9 = {
_2 = !_3;
_38 = 24812_i16 as i8;
place!(Field::<Adt41>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 5)) = Adt41::Variant2 { fld0: 182377427_u32,fld1: _9.fld0 };
_12.fld4 = core::ptr::addr_of_mut!(_38);
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).1.0 = Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.0;
_25.3 = _12.fld2 as isize;
_19 = _12.fld0;
_12.fld5 = (_12.fld4, _21, Field::<u128>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 4), Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.0);
_16.0 = _28.0;
_9.fld0.1 = [Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,_15,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,_15,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0];
place!(Field::<[i64; 4]>(Variant(_12.fld3, 1), 4)) = [583851231749976564_i64,9062446384155012044_i64,7658388228805119127_i64,2220248116104304138_i64];
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).2 = _16.2 | _16.2;
_16.0.1.0 = _12.fld6.1;
place!(Field::<isize>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 2)) = (-9007698105570615534_i64) as isize;
_12.fld1.1 = _26 * _26;
_16.1.0 = _28.0.0 as isize;
_25 = _12.fld5;
_28.1.1 = (-18666871384782937999217908814044488886_i128) as isize;
_2 = _34 as u128;
_33 = _28.0.1.0 + _12.fld6.1;
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).0.1 = _16.0.1;
Call(_38 = fn8(_16.2, _28.0, _5, _16.1.1, _33, _8, Field::<[u128; 1]>(Variant(_12.fld3, 1), 3), _33, _25, Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1, Field::<[u128; 1]>(Variant(_12.fld3, 1), 3), _16.0, _11, Field::<[u128; 1]>(Variant(_12.fld3, 1), 3), Field::<[u128; 1]>(Variant(_12.fld3, 1), 3), _16.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7 = (-89918932861501985114124183432766405962_i128) as i32;
_28.0 = (_16.0.0, _12.fld6);
_29 = [2792630874_u32,2140041968_u32,1180407175_u32,1467937593_u32,3574144996_u32,3960738507_u32,2097970384_u32,3582173060_u32];
_11 = _9.fld0;
_12.fld1 = (_20.0, _26, _20.2);
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 3)) = [7471284701049140325_i64,4562097927380105275_i64,6954045894653775395_i64,(-7578788598940206111_i64)];
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 3)) = Field::<[i64; 4]>(Variant(_12.fld3, 1), 4);
_40.0 = _34;
_16.0.1 = _12.fld6;
_12.fld6.1 = _33 << Field::<isize>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 2);
_47 = Adt47::Variant1 { fld0: _40,fld1: Field::<(bool,)>(Variant(_12.fld3, 1), 0),fld2: _18,fld3: _9.fld0.0,fld4: 391889227_u32 };
_20 = _12.fld1;
_41 = !_16.1.0;
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)) = _28;
_12.fld5.2 = _15 as u128;
_34 = _5.0;
_28.0.1.1 = !_33;
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).0.1.0 = _41 & _25.3;
_9.fld1 = Adt39::Variant0 { fld0: _15,fld1: _25,fld2: 79379834235664291521838146178687121254_i128,fld3: _12.fld1.2,fld4: Field::<[i64; 4]>(Variant(_47, 1), 3),fld5: Field::<([i64; 4], [bool; 7])>(Variant(Field::<Adt41>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 5), 2), 1).1,fld6: _8 };
place!(Field::<([i64; 4], [bool; 7])>(Variant(place!(Field::<Adt41>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 5)), 2), 1)).1 = [Field::<(bool,)>(Variant(_47, 1), 1).0,Field::<(bool,)>(Variant(_47, 1), 1).0,_15,Field::<(bool,)>(Variant(_12.fld3, 1), 0).0,Field::<bool>(Variant(_9.fld1, 0), 0),Field::<bool>(Variant(_9.fld1, 0), 0),Field::<(bool,)>(Variant(_47, 1), 1).0];
Goto(bb11)
}
bb11 = {
place!(Field::<[u128; 1]>(Variant(_12.fld3, 1), 3)) = [_12.fld1.1];
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)) = _28;
_28.1.0 = _40.0 as isize;
_25.0 = core::ptr::addr_of_mut!(_38);
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).1.0 = _38 as isize;
_16.0.1.0 = 3363225636574859289_u64 as isize;
_25.3 = _28.0.1.1 + _16.0.1.1;
_32 = Adt39::Variant0 { fld0: Field::<(bool,)>(Variant(_47, 1), 1).0,fld1: _25,fld2: 141602341138069995146407741508370099804_i128,fld3: _12.fld1.2,fld4: Field::<[i64; 4]>(Variant(_12.fld3, 1), 4),fld5: Field::<[bool; 7]>(Variant(_9.fld1, 0), 5),fld6: Field::<[u128; 1]>(Variant(_9.fld1, 0), 6) };
_35 = (-23299_i16) as f32;
_9.fld1 = Adt39::Variant0 { fld0: Field::<bool>(Variant(_32, 0), 0),fld1: Field::<(*mut i8, f64, u128, isize)>(Variant(_32, 0), 1),fld2: (-67734886605621368826233469505948719752_i128),fld3: Field::<usize>(Variant(_32, 0), 3),fld4: _9.fld0.0,fld5: Field::<[bool; 7]>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 0),fld6: Field::<[u128; 1]>(Variant(_32, 0), 6) };
place!(Field::<[u128; 1]>(Variant(_9.fld1, 0), 6)) = [Field::<u128>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 4)];
place!(Field::<(*mut i8, f64, u128, isize)>(Variant(_32, 0), 1)).0 = core::ptr::addr_of_mut!(_38);
_37 = 25555_i16;
_40.0 = -_34;
_28.0.1 = (_31, Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1).3);
place!(Field::<[u128; 1]>(Variant(_9.fld1, 0), 6)) = Field::<[u128; 1]>(Variant(_32, 0), 6);
Goto(bb12)
}
bb12 = {
place!(Field::<u32>(Variant(_47, 1), 4)) = _38 as u32;
SetDiscriminant(_47, 1);
_12.fld5 = Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1);
match _16.0.0 {
1360738130 => bb13,
_ => bb9
}
}
bb13 = {
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).0.1.1 = _25.3;
_12.fld0 = [2984566483_u32,1041838268_u32,2792293299_u32,3521878748_u32,527542209_u32,2064756536_u32,3406780591_u32,1547029560_u32];
place!(Field::<i128>(Variant(_9.fld1, 0), 2)) = (-66439728282713607805862898032491759722_i128);
place!(Field::<(bool,)>(Variant(_12.fld3, 1), 0)) = (Field::<bool>(Variant(_32, 0), 0),);
_45 = [_12.fld5.2];
_5 = (_6,);
place!(Field::<(f32,)>(Variant(_47, 1), 0)).0 = -_6;
_4 = Field::<char>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 1);
place!(Field::<[bool; 7]>(Variant(_9.fld1, 0), 5)) = Field::<[bool; 7]>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 0);
place!(Field::<(bool,)>(Variant(_12.fld3, 1), 0)).0 = _25.3 != Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).1.1;
_20.2 = Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1).1 as usize;
place!(Field::<(isize,)>(Variant(_47, 1), 2)).0 = _16.2 as isize;
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).1.0 = -_25.3;
place!(Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1)) = Field::<(*mut i8, f64, u128, isize)>(Variant(_32, 0), 1);
_41 = Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1).1 as isize;
place!(Field::<(isize,)>(Variant(_47, 1), 2)).0 = _25.1 as isize;
_12.fld0 = [2235324473_u32,1722063702_u32,2026777273_u32,2872808058_u32,626561416_u32,3050214134_u32,2493116223_u32,608294081_u32];
_11.0 = Field::<[i64; 4]>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 3);
_16.0.0 = Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.0;
_53.0 = (_28.0.0, _28.0.1);
match _28.2 {
0 => bb9,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
3238 => bb20,
_ => bb19
}
}
bb14 = {
_11 = _9.fld0;
_12.fld5.3 = _12.fld6.0;
_9.fld0.0 = [1136546329123452089_i64,(-5208290327067401466_i64),5661803023323416122_i64,6137822569694913277_i64];
_14 = _12.fld5.3;
_5.0 = _6 - _6;
_10 = _8;
_12.fld1.1 = _2 | _2;
_12.fld6.0 = _12.fld5.3;
_12.fld6 = (_12.fld5.3, _12.fld5.3);
_12.fld5.2 = _5.0 as u128;
_12.fld7 = [(-2024361203679685815_i64),6444763529030299121_i64,3894307582311610729_i64,5934933389568369835_i64];
_12.fld1.1 = _12.fld5.2 >> _12.fld5.2;
RET = _4;
_12.fld3 = Adt50::Variant0 { fld0: _12.fld6 };
_18 = (_12.fld6.0,);
_12.fld2 = 179_u8 & 246_u8;
_9.fld0 = (_11.0, _11.1);
_10 = [_12.fld5.2];
_3 = _12.fld1.1;
_18 = (_14,);
_12.fld5.2 = _2;
Goto(bb3)
}
bb15 = {
RET = '\u{7daa7}';
RET = '\u{e66f2}';
_5 = (_6,);
_1 = 4_u8 as isize;
RET = '\u{f286}';
RET = '\u{59426}';
_4 = RET;
RET = _4;
_5 = (_6,);
_5 = (_6,);
_9.fld0.1 = [true,false,false,true,false,true,true];
_2 = 3786820913_u32 as u128;
_9.fld0.0 = [6397720206549533077_i64,2401978199245611966_i64,4037044385861714624_i64,4990276448768623289_i64];
_1 = !71_isize;
_5 = (_6,);
_9.fld0.1 = [false,true,false,true,false,false,true];
_8 = [_3];
_3 = !_2;
_11.1 = _9.fld0.1;
_10 = _8;
_11.1 = _9.fld0.1;
_12.fld0 = [3353707027_u32,4056300403_u32,2260271544_u32,407149209_u32,4188064140_u32,3428749054_u32,1469506963_u32,1821442069_u32];
_2 = _3;
_6 = _5.0;
_12.fld6.0 = _1 | _1;
Goto(bb2)
}
bb16 = {
Goto(bb4)
}
bb17 = {
_12.fld6 = (_28.0.1.1, _28.0.1.0);
match _16.2 {
0 => bb3,
3238 => bb6,
_ => bb2
}
}
bb18 = {
_5.0 = _6;
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).0.1 = (_12.fld6.1, _18.0);
_31 = _28.1.0 >> Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.1;
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 3)) = [7407141174987953309_i64,(-3462048382327035041_i64),(-1753644271100890922_i64),(-2167910466797079463_i64)];
_12.fld6 = (Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.0, _31);
_23 = _4;
_34 = _5.0;
place!(Field::<[bool; 7]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 0)) = _9.fld0.1;
place!(Field::<isize>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 2)) = !_28.1.0;
_22 = _28.0.0;
_16.0.1.0 = !_31;
_12.fld1.0 = core::ptr::addr_of!(_26);
_11.0 = _12.fld7;
_23 = RET;
_25.3 = !_28.0.1.0;
place!(Field::<isize>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 2)) = -_1;
_16.0.1.1 = _12.fld2 as isize;
_12.fld2 = 43_u8;
place!(Field::<u128>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 4)) = _2;
_16.0.1.1 = -_16.0.1.0;
_8 = Field::<[u128; 1]>(Variant(_12.fld3, 1), 3);
_4 = RET;
place!(Field::<char>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 1)) = _23;
_26 = !_3;
_12.fld6.1 = _31 ^ _16.0.1.1;
Goto(bb9)
}
bb19 = {
_2 = !_3;
_21 = _3 as f64;
_1 = Field::<(isize, isize)>(Variant(_12.fld3, 0), 0).1 ^ _12.fld5.3;
_16.0.1.1 = _18.0 & _1;
_16.1.1 = _1 * _16.0.1.1;
_9.fld1 = Adt39::Variant1 { fld0: 10230752195951579742_u64 };
_28.0 = (1360738130_i32, _12.fld6);
SetDiscriminant(_12.fld3, 1);
_12.fld1.2 = 1_usize;
_16.0 = (_28.0.0, _12.fld6);
_16 = (_28.0, _28.0.1, 3238_u16);
_20.0 = core::ptr::addr_of!(_20.1);
place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)) = Adt45::Variant2 { fld0: _12.fld2,fld1: _12.fld7 };
_23 = _4;
_12.fld1.0 = core::ptr::addr_of!(_3);
place!(Field::<[u128; 1]>(Variant(_12.fld3, 1), 3)) = _8;
_23 = _4;
_9.fld0 = _11;
SetDiscriminant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 2);
place!(Field::<[i64; 4]>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 2), 1)) = _12.fld7;
_12.fld1.2 = 13616274792866860635_usize & 7236921944148446081_usize;
_28.0.1 = (_1, _16.1.0);
_12.fld2 = 221_u8;
_11 = _9.fld0;
_12.fld7 = [650652483254977268_i64,(-8078884541673520854_i64),(-5935452164405561580_i64),6709373175651354466_i64];
Goto(bb5)
}
bb20 = {
place!(Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1)).3 = Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).1.0 * _28.0.1.1;
_22 = _53.0.0 ^ _16.0.0;
_25.1 = Field::<(*mut i8, f64, u128, isize)>(Variant(_32, 0), 1).1 + _21;
_13 = core::ptr::addr_of_mut!(_38);
_53.1 = (_53.0.1.1, _25.3);
_8 = [_12.fld1.1];
_12.fld1.0 = _20.0;
_16.2 = !_28.2;
_52 = (Field::<(*mut i8, f64, u128, isize)>(Variant(_32, 0), 1).3,);
_25.0 = core::ptr::addr_of_mut!((*_13));
_16.1 = (Field::<(isize,)>(Variant(_47, 1), 2).0, Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1.1);
place!(Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1)).1 = _25.1 - Field::<(*mut i8, f64, u128, isize)>(Variant(_32, 0), 1).1;
_16.0.0 = _22;
_32 = Adt39::Variant1 { fld0: 4078481672109448780_u64 };
place!(Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 7)).1 = _28.1;
_28.2 = _16.2;
place!(Field::<u32>(Variant(_47, 1), 4)) = 4200227303_u32 & 1767595958_u32;
_28.1 = Field::<((i32, (isize, isize)), (isize, isize), u16)>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 7).0.1;
_16.0.1.0 = Field::<u32>(Variant(_47, 1), 4) as isize;
place!(Field::<u32>(Variant(place!(Field::<Adt41>(Variant(place!(Field::<Adt45>(Variant(_12.fld3, 1), 2)), 1), 5)), 2), 0)) = Field::<u32>(Variant(_47, 1), 4) & Field::<u32>(Variant(_47, 1), 4);
_55 = (Field::<(f32,)>(Variant(_47, 1), 0).0,);
_51.0 = _40.0;
_53.0.1.0 = _28.0.1.1 * _53.1.0;
_16.1.1 = _37 as isize;
_52 = (_12.fld6.1,);
_47 = Adt47::Variant1 { fld0: _40,fld1: Field::<(bool,)>(Variant(_12.fld3, 1), 0),fld2: _52,fld3: Field::<[i64; 4]>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 3),fld4: Field::<u32>(Variant(Field::<Adt41>(Variant(Field::<Adt45>(Variant(_12.fld3, 1), 2), 1), 5), 2), 0) };
_16.1.0 = !_31;
_53.0 = (_22, _28.0.1);
_34 = _5.0;
_21 = Field::<(*mut i8, f64, u128, isize)>(Variant(_9.fld1, 0), 1).1;
Goto(bb21)
}
bb21 = {
Call(_57 = dump_var(7_usize, 52_usize, Move(_52), 28_usize, Move(_28), 19_usize, Move(_19), 45_usize, Move(_45)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_57 = dump_var(7_usize, 11_usize, Move(_11), 38_usize, Move(_38), 23_usize, Move(_23), 26_usize, Move(_26)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_57 = dump_var(7_usize, 1_usize, Move(_1), 2_usize, Move(_2), 37_usize, Move(_37), 15_usize, Move(_15)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u16,mut _2: (i32, (isize, isize)),mut _3: (f32,),mut _4: isize,mut _5: isize,mut _6: [u128; 1],mut _7: [u128; 1],mut _8: isize,mut _9: (*mut i8, f64, u128, isize),mut _10: (isize, isize),mut _11: [u128; 1],mut _12: (i32, (isize, isize)),mut _13: ([i64; 4], [bool; 7]),mut _14: [u128; 1],mut _15: [u128; 1],mut _16: (i32, (isize, isize))) -> i8 {
mir! {
type RET = i8;
let _17: f64;
let _18: [i64; 4];
let _19: Adt42;
let _20: f64;
let _21: f32;
let _22: f32;
let _23: ();
let _24: ();
{
_15 = _6;
_12.1.1 = _8;
_5 = !_10.0;
match _1 {
3238 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_9.0 = core::ptr::addr_of_mut!(RET);
_6 = _11;
_1 = 22118_u16;
_15 = _6;
_6 = [_9.2];
_3.0 = (-37_i8) as f32;
_16.1.0 = _9.2 as isize;
_17 = _9.1 + _9.1;
_10 = (_8, _12.1.1);
match _12.0 {
0 => bb3,
1 => bb4,
1360738130 => bb6,
_ => bb5
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
RET = !(-45_i8);
_9.0 = core::ptr::addr_of_mut!(RET);
_10.1 = _5;
_16.1.1 = -_12.1.0;
_6 = [_9.2];
_9.1 = _2.0 as f64;
RET = 84_i8;
_16.0 = _12.0;
_9.2 = !109672255486715718174508900074798011226_u128;
_20 = _17 + _9.1;
_10 = (_8, _16.1.1);
_2.1.0 = false as isize;
_16 = (_12.0, _10);
_12.0 = !_16.0;
_10.1 = _8;
Call(RET = fn9(_8, _16.1.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2.0 = _3.0 as i32;
_4 = '\u{e110a}' as isize;
_20 = _17 - _17;
_18 = [5702168206159639161_i64,(-5266669550061446696_i64),(-3396932935591225520_i64),2273434586800953950_i64];
_16.1 = (_12.1.0, _8);
_12 = (_16.0, _2.1);
_17 = _8 as f64;
_16 = (_2.0, _10);
_16.0 = _12.0;
_20 = -_17;
_3.0 = 3_usize as f32;
_20 = 100266226468004311917690596917038178975_i128 as f64;
_16 = (_12.0, _10);
_15 = [_9.2];
_10.1 = (-17322_i16) as isize;
_4 = _16.1.0;
_8 = -_16.1.1;
_9.1 = -_17;
_1 = _3.0 as u16;
RET = (-104_i8) << _5;
_7 = [_9.2];
_10 = (_5, _4);
_15 = _14;
_2 = _12;
_9.3 = (-24606_i16) as isize;
Goto(bb8)
}
bb8 = {
Call(_23 = dump_var(8_usize, 11_usize, Move(_11), 5_usize, Move(_5), 14_usize, Move(_14), 16_usize, Move(_16)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_23 = dump_var(8_usize, 8_usize, Move(_8), 6_usize, Move(_6), 13_usize, Move(_13), 24_usize, _24), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize) -> i8 {
mir! {
type RET = i8;
let _3: [u128; 1];
let _4: Adt51;
let _5: isize;
let _6: (bool,);
let _7: Adt49;
let _8: usize;
let _9: i32;
let _10: Adt52;
let _11: (isize,);
let _12: (*mut i8, f64, u128, isize);
let _13: isize;
let _14: isize;
let _15: [i64; 4];
let _16: ();
let _17: ();
{
_1 = !_2;
Goto(bb1)
}
bb1 = {
RET = 34_i8;
_2 = -_1;
RET = !73_i8;
_1 = 14982368022836181808_u64 as isize;
_2 = _1;
_4.fld3.1 = !167662828165663308419766539418383417645_u128;
_4.fld2 = _1 & _1;
_3 = [_4.fld3.1];
RET = 109_i8;
_4.fld3.2 = 2125436082_u32 as usize;
_4.fld4 = (-23571_i16) as u128;
_4.fld2 = _1;
_4.fld3.1 = _4.fld4 - _4.fld4;
_4.fld4 = RET as u128;
_6 = (false,);
_3 = [_4.fld3.1];
_3 = [_4.fld3.1];
_5 = 67337488_u32 as isize;
_6.0 = true;
_4.fld3.2 = 3554073902937114922_usize;
RET = 87_i8;
_4.fld2 = !_1;
_1 = _5;
_4.fld1 = (-1353245528_i32) as f64;
_4.fld3.0 = core::ptr::addr_of!(_4.fld4);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
87 => bb9,
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
_1 = _2;
_5 = 31161_u16 as isize;
_2 = _5 - _4.fld2;
_4.fld3.2 = 13444813591385593799_usize ^ 16477740995749993194_usize;
Call(_1 = fn10(_2, _4.fld3.1, _4.fld4, _4.fld3.0, _4.fld3, _4.fld3.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4.fld1 = 2051365796_i32 as f64;
_5 = (-152906664619098950882039151805272765882_i128) as isize;
_4.fld3.2 = 3_usize >> _1;
_5 = _1 >> _4.fld3.2;
_9 = 882147691_u32 as i32;
_9 = 1935368607_i32;
_5 = !_1;
_11.0 = _1 & _2;
_3 = [_4.fld3.1];
_5 = _4.fld2 << _1;
_12.2 = _4.fld4;
_12.0 = core::ptr::addr_of_mut!(RET);
Call(_4.fld3.1 = fn17(_5, _2, _11.0, _2, _11, _4.fld3.2, _5, _11, _11, _2, _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_4.fld3.0 = core::ptr::addr_of!(_4.fld3.1);
_5 = _1;
Goto(bb12)
}
bb12 = {
_6.0 = !true;
RET = 93_i8 + (-88_i8);
_11 = (_5,);
RET = 87_i8 ^ (-15_i8);
match _9 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
1935368607 => bb19,
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
Return()
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET = 34_i8;
_2 = -_1;
RET = !73_i8;
_1 = 14982368022836181808_u64 as isize;
_2 = _1;
_4.fld3.1 = !167662828165663308419766539418383417645_u128;
_4.fld2 = _1 & _1;
_3 = [_4.fld3.1];
RET = 109_i8;
_4.fld3.2 = 2125436082_u32 as usize;
_4.fld4 = (-23571_i16) as u128;
_4.fld2 = _1;
_4.fld3.1 = _4.fld4 - _4.fld4;
_4.fld4 = RET as u128;
_6 = (false,);
_3 = [_4.fld3.1];
_3 = [_4.fld3.1];
_5 = 67337488_u32 as isize;
_6.0 = true;
_4.fld3.2 = 3554073902937114922_usize;
RET = 87_i8;
_4.fld2 = !_1;
_1 = _5;
_4.fld1 = (-1353245528_i32) as f64;
_4.fld3.0 = core::ptr::addr_of!(_4.fld4);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
87 => bb9,
_ => bb8
}
}
bb19 = {
_5 = _11.0;
_4.fld1 = 11721761731318449039_u64 as f64;
_4.fld3.0 = core::ptr::addr_of!(_4.fld3.1);
_13 = _4.fld1 as isize;
_5 = _11.0;
_4.fld3.2 = 4_usize;
_1 = _11.0;
_8 = _4.fld3.2;
_12.1 = _1 as f64;
_1 = _11.0 - _2;
_12.3 = -_2;
_4.fld3.1 = _12.2;
_12.0 = core::ptr::addr_of_mut!(RET);
_4.fld3.1 = _4.fld4 | _4.fld4;
Goto(bb20)
}
bb20 = {
Call(_16 = dump_var(9_usize, 8_usize, Move(_8), 2_usize, Move(_2), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: u128,mut _3: u128,mut _4: *const u128,mut _5: (*const u128, u128, usize),mut _6: u128) -> isize {
mir! {
type RET = isize;
let _7: Adt49;
let _8: Adt44;
let _9: [u128; 1];
let _10: [i64; 4];
let _11: f64;
let _12: ([i64; 4], [bool; 7]);
let _13: [u32; 8];
let _14: Adt47;
let _15: f32;
let _16: f64;
let _17: usize;
let _18: [u128; 1];
let _19: (f32,);
let _20: Adt54;
let _21: usize;
let _22: f64;
let _23: bool;
let _24: [i128; 4];
let _25: Adt46;
let _26: (i32, (isize, isize));
let _27: f64;
let _28: char;
let _29: (isize,);
let _30: [i128; 4];
let _31: Adt46;
let _32: isize;
let _33: f32;
let _34: i128;
let _35: u32;
let _36: isize;
let _37: f32;
let _38: i16;
let _39: f32;
let _40: [bool; 7];
let _41: *const u16;
let _42: f32;
let _43: ();
let _44: ();
{
_5.0 = core::ptr::addr_of!((*_4));
RET = _1 * _1;
_4 = core::ptr::addr_of!(_3);
_5.0 = core::ptr::addr_of!(_6);
Goto(bb1)
}
bb1 = {
_1 = (-32308_i16) as isize;
_5.2 = !0_usize;
(*_4) = _6 ^ _5.1;
_5.0 = core::ptr::addr_of!(_3);
(*_4) = RET as u128;
RET = _1;
_6 = (*_4) | _3;
RET = !_1;
_2 = _3 & _3;
(*_4) = _6 * _2;
(*_4) = _2;
(*_4) = _6 ^ _2;
RET = !_1;
_5.0 = _4;
_2 = _6 ^ (*_4);
_5.2 = 4955641738906827236_usize;
_5.0 = _4;
(*_4) = 242_u8 as u128;
_2 = (*_4);
Goto(bb2)
}
bb2 = {
_5.1 = (-122114710146757599600348279554955156596_i128) as u128;
_9 = [_6];
_3 = !_6;
Goto(bb3)
}
bb3 = {
_12.1 = [false,false,false,true,false,true,false];
_4 = _5.0;
_9 = [(*_4)];
_9 = [_3];
_5.2 = 16564227001599697203_usize * 7156209044571520783_usize;
_10 = [(-395846783083896779_i64),(-3104946538328830282_i64),1193403299620169429_i64,7449287890676075820_i64];
_5.2 = 1082534518063901666_usize;
_11 = (-425496175_i32) as f64;
_13 = [1567525283_u32,1992589921_u32,1517274834_u32,1579020100_u32,2562440317_u32,740406683_u32,2824002215_u32,3890691356_u32];
_12.1 = [false,false,true,true,true,false,false];
RET = _1 - _1;
Call(_11 = fn11(_5, _4, _1, _5, _3, _6, _5, _5, _3, _4, _4, _4, _5.0, _5, _10, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _1 * _1;
_13 = [1873395624_u32,3208180121_u32,2554603640_u32,2785842595_u32,2859655110_u32,2003744782_u32,4096195989_u32,398642600_u32];
_11 = (-1575286899067741474_i64) as f64;
_13 = [2337610402_u32,525730955_u32,336883315_u32,4285734782_u32,413801492_u32,124422197_u32,4035289062_u32,79509552_u32];
_1 = 8928763530863287754_u64 as isize;
_12.1 = [false,true,false,false,false,false,true];
_5.2 = 40_i8 as usize;
_5.2 = 2_usize;
_12.0 = [(-651617209208193923_i64),2641741545885161097_i64,(-1045061460085790543_i64),(-504259981091069256_i64)];
_4 = core::ptr::addr_of!(_3);
(*_4) = !_5.1;
_10 = [(-4778980471731770813_i64),(-8656784482332296804_i64),4065211966064696238_i64,(-1684964154267254980_i64)];
_11 = 8668091773647478595_u64 as f64;
_12.0 = [4815790218807541147_i64,(-6603617623666315156_i64),1532624132964885159_i64,(-4046376335337783484_i64)];
_18 = [_6];
_9 = [_5.1];
_5.2 = 5554459363473359982_usize;
_5.2 = 47621607440213174664667147990622259246_i128 as usize;
_15 = 524242445_i32 as f32;
_8 = Adt44::Variant3 { fld0: (-4_i8) };
(*_4) = !_6;
Call(_21 = fn12(_5, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = !(*_4);
_16 = _11 - _11;
_19.0 = _16 as f32;
_4 = _5.0;
_6 = (*_4) & _2;
(*_4) = _2;
(*_4) = _2;
_18 = [(*_4)];
_11 = _16 - _16;
_1 = !RET;
Goto(bb6)
}
bb6 = {
_5.1 = !_3;
_17 = _21;
_22 = -_11;
_5 = (_4, _6, _21);
_15 = 153639557286757839429230691802312204255_i128 as f32;
_24 = [131165231285982413275619628091948855414_i128,(-54168086442142717169561461931576598857_i128),(-41137005947294016515106053542092267962_i128),(-99177451788011657554942483117744733718_i128)];
_8 = Adt44::Variant3 { fld0: (-59_i8) };
_5.2 = _17;
_6 = !(*_4);
_10 = [(-2144092822189338062_i64),4462485913567267417_i64,1810087655770359035_i64,5821888779137369329_i64];
_25.fld0.0 = [5318321106214651965_i64,1780266376166776310_i64,(-2613841270150764332_i64),1008471858085179316_i64];
_2 = !_6;
_5.2 = !_17;
Goto(bb7)
}
bb7 = {
_19.0 = _15;
_17 = '\u{c089b}' as usize;
_5 = (_4, _2, _21);
_21 = _5.2;
_5.0 = _4;
_26.1.1 = _22 as isize;
_5.1 = !(*_4);
_5.1 = _3;
_3 = _6;
_21 = _5.2;
_25.fld0.0 = [(-4364216288644171106_i64),8916873309972461164_i64,7243511849508781422_i64,(-3542502466447767967_i64)];
_6 = _5.1 << _5.2;
_23 = !false;
_26.1.1 = RET - _1;
(*_4) = _6;
_19 = (_15,);
place!(Field::<i8>(Variant(_8, 3), 0)) = (-105_i8);
_28 = '\u{7fe0a}';
_6 = _3;
_26.1.0 = !RET;
_17 = _21 ^ _5.2;
_10 = [(-7607187147130131971_i64),2322365367973974846_i64,(-7727586661259211851_i64),(-3363449551136362420_i64)];
_5 = (_4, (*_4), _17);
_8 = Adt44::Variant3 { fld0: 79_i8 };
_9 = [(*_4)];
_25.fld0 = (_10, _12.1);
_12.1 = [_23,_23,_23,_23,_23,_23,_23];
Goto(bb8)
}
bb8 = {
_9 = [_6];
_9 = [(*_4)];
_25.fld0.1 = [_23,_23,_23,_23,_23,_23,_23];
_29 = (_26.1.1,);
RET = _26.1.0;
(*_4) = _6 + _6;
_22 = -_11;
_5.2 = !_17;
_17 = !_21;
(*_4) = !_5.1;
Call(_21 = core::intrinsics::bswap(_5.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = !(*_4);
_18 = [(*_4)];
_5.0 = core::ptr::addr_of!(_3);
_4 = core::ptr::addr_of!((*_4));
_19.0 = -_15;
_19.0 = _15 - _15;
_31.fld0 = (_12.0, _25.fld0.1);
_2 = (*_4);
_12.0 = [5329831206528022525_i64,(-945371266261868716_i64),423253132764207179_i64,5750913870231242963_i64];
_4 = core::ptr::addr_of!(_6);
_12.1 = [_23,_23,_23,_23,_23,_23,_23];
_22 = -_11;
Goto(bb10)
}
bb10 = {
_31.fld1 = Adt39::Variant1 { fld0: 5652262401056402166_u64 };
_5.2 = _17 << _21;
_12 = (_31.fld0.0, _31.fld0.1);
_12.0 = [(-5129350164732388662_i64),(-6844486755132773137_i64),1594401362969089088_i64,(-7768641992482768485_i64)];
Call(_3 = fn14(_4, _6, _5.0, _10, _21, _17, _6, _5, _5.1, _4, _5.2, _5.2, _4, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = (-7559897298771920636_i64) as isize;
RET = _26.1.1;
_5.0 = _4;
_31.fld0 = _12;
_31.fld0.1 = [_23,_23,_23,_23,_23,_23,_23];
_25.fld1 = Adt39::Variant1 { fld0: 1598221402302195671_u64 };
_18 = [(*_4)];
_27 = _16;
place!(Field::<i8>(Variant(_8, 3), 0)) = 121_i8 | (-116_i8);
_22 = _27;
SetDiscriminant(_8, 3);
_33 = _19.0;
_33 = 1426846518398769718_i64 as f32;
_12.1 = [_23,_23,_23,_23,_23,_23,_23];
_33 = _19.0 - _15;
place!(Field::<i8>(Variant(_8, 3), 0)) = -118_i8;
(*_4) = _2 & _3;
_31.fld0 = (_25.fld0.0, _25.fld0.1);
_23 = true ^ true;
_34 = _33 as i128;
Goto(bb12)
}
bb12 = {
_33 = _19.0;
Call(RET = core::intrinsics::transmute(_5.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_4) = _2;
SetDiscriminant(_8, 1);
_5 = (_4, (*_4), _17);
_5.2 = !_21;
place!(Field::<u64>(Variant(_31.fld1, 1), 0)) = 12828190138159891444_u64 - 2837264189098708077_u64;
RET = _26.1.1;
RET = _26.1.1;
(*_4) = _23 as u128;
_12 = (_31.fld0.0, _31.fld0.1);
SetDiscriminant(_31.fld1, 0);
Call(_31.fld0.1 = fn16(_5, _3, _18, _2, _5.1, _18, _3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_26.1.0 = _29.0 & _29.0;
RET = -_26.1.0;
place!(Field::<[i64; 4]>(Variant(_31.fld1, 0), 4)) = [(-3733740349557503632_i64),(-2988496619213174568_i64),5740226024953134481_i64,(-6068739990720951715_i64)];
_13 = [1710921682_u32,981578610_u32,700565862_u32,1053420917_u32,2962714638_u32,4130512303_u32,944821477_u32,1528408597_u32];
place!(Field::<u32>(Variant(_8, 1), 3)) = 17073_u16 as u32;
_31.fld0.1 = [_23,_23,_23,_23,_23,_23,_23];
_34 = 7510039475944281377614541739627805217_i128;
place!(Field::<(*mut i8, f64, u128, isize)>(Variant(_31.fld1, 0), 1)).2 = !_2;
_25.fld0.1 = [_23,_23,_23,_23,_23,_23,_23];
_26.1.0 = _29.0 | _29.0;
(*_4) = _3;
place!(Field::<(*mut i8, f64, u128, isize)>(Variant(_31.fld1, 0), 1)).1 = _11;
place!(Field::<usize>(Variant(_8, 1), 2)) = !_5.2;
place!(Field::<u8>(Variant(_8, 1), 4)) = !16_u8;
_29 = (_26.1.1,);
_1 = _26.1.0;
place!(Field::<u64>(Variant(_25.fld1, 1), 0)) = (-108_i8) as u64;
_40 = [_23,_23,_23,_23,_23,_23,_23];
place!(Field::<u32>(Variant(_8, 1), 3)) = !418381886_u32;
place!(Field::<u32>(Variant(_8, 1), 3)) = 4082760789_u32 * 1689116072_u32;
_37 = _19.0;
place!(Field::<[bool; 7]>(Variant(_31.fld1, 0), 5)) = [_23,_23,_23,_23,_23,_23,_23];
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(10_usize, 2_usize, Move(_2), 24_usize, Move(_24), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(10_usize, 17_usize, Move(_17), 34_usize, Move(_34), 1_usize, Move(_1), 40_usize, Move(_40)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: (*const u128, u128, usize),mut _2: *const u128,mut _3: isize,mut _4: (*const u128, u128, usize),mut _5: u128,mut _6: u128,mut _7: (*const u128, u128, usize),mut _8: (*const u128, u128, usize),mut _9: u128,mut _10: *const u128,mut _11: *const u128,mut _12: *const u128,mut _13: *const u128,mut _14: (*const u128, u128, usize),mut _15: [i64; 4],mut _16: u128) -> f64 {
mir! {
type RET = f64;
let _17: bool;
let _18: bool;
let _19: Adt40;
let _20: i32;
let _21: (bool,);
let _22: char;
let _23: *mut usize;
let _24: bool;
let _25: Adt46;
let _26: Adt44;
let _27: ();
let _28: ();
{
(*_11) = _6;
_4.0 = _13;
_8.2 = 13210_u16 as usize;
_8.0 = _14.0;
_7 = _4;
_4 = _14;
_5 = !(*_2);
RET = 132_u8 as f64;
_8.1 = _6;
_14 = (_11, (*_10), _4.2);
_1.0 = _2;
_11 = core::ptr::addr_of!(_16);
_16 = _3 as u128;
_4.2 = _14.2;
(*_2) = !_5;
_4.2 = _7.2;
_7.2 = _14.2;
_17 = _7.2 > _8.2;
_7.0 = _13;
_18 = _17 | _17;
match _7.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1082534518063901666 => bb9,
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
_16 = _7.1;
(*_10) = !_5;
_8 = (_12, (*_12), _4.2);
_5 = !_6;
Goto(bb10)
}
bb10 = {
_4.0 = core::ptr::addr_of!((*_10));
_4 = _8;
_12 = _14.0;
_7.0 = _1.0;
_8.2 = _4.2 ^ _4.2;
(*_2) = _8.1 + _7.1;
(*_10) = _4.1 - (*_11);
_12 = core::ptr::addr_of!((*_13));
_12 = core::ptr::addr_of!((*_12));
_16 = 971387979149330742_i64 as u128;
(*_11) = (*_12) >> _5;
_1 = (_2, (*_12), _4.2);
_18 = _17 & _17;
_4.2 = !_7.2;
_5 = (*_13);
(*_13) = RET as u128;
_8.2 = _14.2 % _7.2;
_8.1 = !_9;
_16 = _6;
RET = 32859215966571118078086029910167794360_i128 as f64;
(*_13) = _1.1;
(*_13) = !_9;
match _14.2 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1082534518063901666 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_4.0 = _1.0;
_14 = (_1.0, _5, _8.2);
_14 = (_2, _8.1, _7.2);
_14.2 = !_8.2;
(*_11) = (*_13) & _1.1;
_13 = core::ptr::addr_of!((*_11));
_18 = _17;
_13 = _1.0;
_5 = (*_11) & _8.1;
(*_12) = (*_11) & _7.1;
_1.0 = _11;
RET = 16323901395976551488_u64 as f64;
_1.1 = 905867813_u32 as u128;
(*_13) = _5;
(*_13) = (-6774_i16) as u128;
match _1.2 {
1082534518063901666 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
(*_13) = !_5;
RET = _3 as f64;
_4.0 = core::ptr::addr_of!(_1.1);
RET = 34069_u16 as f64;
_20 = -(-1269752636_i32);
(*_12) = _8.1;
_15 = [4602812405891549454_i64,(-5709677415281664542_i64),(-2427511184529733120_i64),(-3989836116157318835_i64)];
_7.0 = core::ptr::addr_of!((*_2));
_5 = !(*_12);
_25.fld0.1 = [_18,_17,_17,_17,_17,_18,_18];
_8.1 = !_6;
_15 = [3016063190393718655_i64,8059293408507410902_i64,(-5340763277860735666_i64),(-4027473290132708909_i64)];
(*_12) = !(*_11);
(*_13) = (*_11);
_25.fld0.0 = [5794196728703322653_i64,2878386779688756146_i64,2672082696312060380_i64,3602328906530354642_i64];
_22 = '\u{eaa36}';
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(11_usize, 17_usize, Move(_17), 20_usize, Move(_20), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(11_usize, 3_usize, Move(_3), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (*const u128, u128, usize),mut _2: u128) -> usize {
mir! {
type RET = usize;
let _3: Adt47;
let _4: char;
let _5: i32;
let _6: [u128; 1];
let _7: bool;
let _8: ([i64; 4], [bool; 7]);
let _9: isize;
let _10: Adt42;
let _11: [u128; 1];
let _12: ();
let _13: ();
{
_2 = 4553084175918350628_i64 as u128;
_2 = 15476933701064889742_u64 as u128;
RET = !_1.2;
_1.1 = _2 - _2;
RET = _1.1 as usize;
_1.2 = RET;
_1.0 = core::ptr::addr_of!(_1.1);
_1.1 = (-9223372036854775808_isize) as u128;
RET = '\u{c1e9d}' as usize;
RET = _1.2 + _1.2;
_1.0 = core::ptr::addr_of!(_2);
_1.2 = !RET;
_1.1 = _2;
_2 = (-42_isize) as u128;
_2 = (-88_isize) as u128;
_1.1 = _2 << _1.2;
_1.0 = core::ptr::addr_of!(_1.1);
RET = _1.2;
_4 = '\u{1ceea}';
RET = _1.2;
_1.1 = !_2;
Call(_1.2 = fn13(RET, _2, RET, _2, RET, _1.0, _1.0, _2, _4, _2, RET, RET, RET, _1.0, _1.1, _1.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.1 = !_2;
RET = !_1.2;
_6 = [_1.1];
_1.0 = core::ptr::addr_of!(_1.1);
_7 = !false;
_7 = true;
_4 = '\u{23669}';
_4 = '\u{b7b28}';
RET = (-48108435532617056660127603433497949383_i128) as usize;
_5 = 646342632_u32 as i32;
_8.0 = [(-8751992718293223543_i64),7548291479749112136_i64,5986212931194030050_i64,6694216828520038229_i64];
_5 = 7105932741674897294229447840045687216_i128 as i32;
_2 = _1.1 << _1.2;
_6 = [_2];
RET = _1.2 ^ _1.2;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(12_usize, 6_usize, Move(_6), 4_usize, Move(_4), 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: usize,mut _2: u128,mut _3: usize,mut _4: u128,mut _5: usize,mut _6: *const u128,mut _7: *const u128,mut _8: u128,mut _9: char,mut _10: u128,mut _11: usize,mut _12: usize,mut _13: usize,mut _14: *const u128,mut _15: u128,mut _16: u128) -> usize {
mir! {
type RET = usize;
let _17: isize;
let _18: bool;
let _19: (f32,);
let _20: isize;
let _21: (i32, (isize, isize));
let _22: (isize, isize);
let _23: (isize,);
let _24: *const u16;
let _25: Adt44;
let _26: char;
let _27: Adt46;
let _28: (i32, (isize, isize));
let _29: u32;
let _30: ([i64; 4], [bool; 7]);
let _31: char;
let _32: ();
let _33: ();
{
(*_7) = !_2;
_16 = _8 << _13;
(*_6) = !_16;
(*_7) = _16 ^ _16;
_12 = !_13;
(*_7) = !_16;
_2 = (-25814_i16) as u128;
RET = 3574344358_u32 as usize;
_4 = (-60_isize) as u128;
_16 = !_10;
_5 = 53597_u16 as usize;
_3 = 80259447955812628751664094651496918593_i128 as usize;
_12 = _13;
_13 = 30499_i16 as usize;
(*_14) = _2;
_9 = '\u{92396}';
_17 = 11652247945977142830_u64 as isize;
_3 = !_1;
(*_6) = !_10;
_9 = '\u{10579a}';
(*_6) = _4;
_19.0 = 178_u8 as f32;
_18 = !false;
_20 = _17;
Goto(bb1)
}
bb1 = {
_9 = '\u{a52ca}';
(*_14) = 7360420430731390557_u64 as u128;
_21.1.1 = _17 ^ _17;
_14 = _6;
(*_6) = (-21425_i16) as u128;
_2 = _15 | (*_6);
_21.1 = (_17, _20);
_17 = _20 & _20;
_21.1.1 = 1874874754968123309_i64 as isize;
Goto(bb2)
}
bb2 = {
_14 = _7;
_19.0 = _11 as f32;
_19.0 = 7272736399150040167_i64 as f32;
_10 = _2;
_21.1.0 = !_21.1.1;
_18 = !false;
_15 = !_4;
RET = _9 as usize;
_22 = (_20, _17);
_4 = !_8;
_8 = 732359390_u32 as u128;
_21.1.1 = _22.0 << _3;
_21.0 = (-1727044580_i32);
_17 = _18 as isize;
_10 = 2748911924107902964_i64 as u128;
(*_14) = _2 & _16;
_22 = (_21.1.1, _20);
_21 = (550768173_i32, _22);
(*_14) = !_2;
_15 = _21.0 as u128;
_16 = !(*_7);
(*_14) = !_16;
_21.1.1 = _2 as isize;
_19.0 = _5 as f32;
_21 = ((-1043631427_i32), _22);
RET = !_11;
_15 = 102575301655559093970889893686737204721_i128 as u128;
_13 = _1;
_11 = _12 - _12;
_21.1 = (_22.0, _20);
(*_7) = _4;
Goto(bb3)
}
bb3 = {
_4 = !(*_6);
_9 = '\u{58667}';
RET = _18 as usize;
_16 = 21367_u16 as u128;
_20 = 12438743978040621686_u64 as isize;
_15 = (*_6);
_23 = (_21.1.0,);
_12 = !_13;
_1 = !_13;
_13 = _3;
_7 = core::ptr::addr_of!((*_7));
_15 = _2 | (*_7);
_21 = ((-225222617_i32), _22);
_12 = _11 & _11;
(*_7) = 88395060180893951417671066446631209734_i128 as u128;
_3 = !_13;
(*_14) = (-8172169937152881436_i64) as u128;
_17 = _22.0 + _22.1;
_1 = _13 >> _15;
_20 = -_23.0;
(*_7) = _10;
(*_7) = (-100246089943849599035629251640243965825_i128) as u128;
_2 = _16;
(*_14) = !_16;
_4 = (*_6) ^ _16;
match _21.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431542988839 => bb9,
_ => bb8
}
}
bb4 = {
_14 = _7;
_19.0 = _11 as f32;
_19.0 = 7272736399150040167_i64 as f32;
_10 = _2;
_21.1.0 = !_21.1.1;
_18 = !false;
_15 = !_4;
RET = _9 as usize;
_22 = (_20, _17);
_4 = !_8;
_8 = 732359390_u32 as u128;
_21.1.1 = _22.0 << _3;
_21.0 = (-1727044580_i32);
_17 = _18 as isize;
_10 = 2748911924107902964_i64 as u128;
(*_14) = _2 & _16;
_22 = (_21.1.1, _20);
_21 = (550768173_i32, _22);
(*_14) = !_2;
_15 = _21.0 as u128;
_16 = !(*_7);
(*_14) = !_16;
_21.1.1 = _2 as isize;
_19.0 = _5 as f32;
_21 = ((-1043631427_i32), _22);
RET = !_11;
_15 = 102575301655559093970889893686737204721_i128 as u128;
_13 = _1;
_11 = _12 - _12;
_21.1 = (_22.0, _20);
(*_7) = _4;
Goto(bb3)
}
bb5 = {
_9 = '\u{a52ca}';
(*_14) = 7360420430731390557_u64 as u128;
_21.1.1 = _17 ^ _17;
_14 = _6;
(*_6) = (-21425_i16) as u128;
_2 = _15 | (*_6);
_21.1 = (_17, _20);
_17 = _20 & _20;
_21.1.1 = 1874874754968123309_i64 as isize;
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
_13 = _12 + _12;
_25 = Adt44::Variant3 { fld0: 28_i8 };
_22 = _21.1;
place!(Field::<i8>(Variant(_25, 3), 0)) = _4 as i8;
_26 = _9;
_5 = 13225234621423721644_u64 as usize;
SetDiscriminant(_25, 1);
(*_14) = _2;
Goto(bb10)
}
bb10 = {
_21.1 = (_20, _23.0);
_22 = (_17, _23.0);
_21.1.0 = _18 as isize;
_21.1 = (_20, _23.0);
_23 = (_17,);
place!(Field::<u8>(Variant(_25, 1), 4)) = !81_u8;
place!(Field::<usize>(Variant(_25, 1), 2)) = !_13;
RET = Field::<usize>(Variant(_25, 1), 2);
_5 = _13 + Field::<usize>(Variant(_25, 1), 2);
place!(Field::<*mut usize>(Variant(_25, 1), 1)) = core::ptr::addr_of_mut!(_3);
_14 = _6;
RET = 14339794707676178404_u64 as usize;
_3 = _5 & _5;
_21.1.1 = _22.1 - _21.1.0;
_13 = !_3;
_8 = _18 as u128;
_21.1.1 = _23.0 & _22.0;
place!(Field::<u8>(Variant(_25, 1), 4)) = !220_u8;
_8 = !(*_6);
_27.fld1 = Adt39::Variant1 { fld0: 16473158181406106503_u64 };
_22.0 = _21.1.1;
_22.0 = _17;
place!(Field::<u32>(Variant(_25, 1), 3)) = !1589914499_u32;
_6 = _14;
RET = 3_i8 as usize;
_28.1.0 = _22.0 * _21.1.1;
_22 = _21.1;
match _21.0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
340282366920938463463374607431542988839 => bb12,
_ => bb11
}
}
bb11 = {
_9 = '\u{a52ca}';
(*_14) = 7360420430731390557_u64 as u128;
_21.1.1 = _17 ^ _17;
_14 = _6;
(*_6) = (-21425_i16) as u128;
_2 = _15 | (*_6);
_21.1 = (_17, _20);
_17 = _20 & _20;
_21.1.1 = 1874874754968123309_i64 as isize;
Goto(bb2)
}
bb12 = {
_28.1 = (_21.1.0, _22.1);
_28 = (_21.0, _21.1);
RET = Field::<usize>(Variant(_25, 1), 2);
_26 = _9;
_21.1.1 = _23.0;
_19.0 = (*_7) as f32;
_8 = Field::<u32>(Variant(_25, 1), 3) as u128;
_5 = !RET;
_18 = !true;
(*_7) = _10;
_11 = _3 & _1;
_26 = _9;
_12 = _3;
_30.0 = [6778485581767552512_i64,1723037253940251126_i64,(-5934198556193439233_i64),(-6834763617400964897_i64)];
_9 = _26;
_26 = _9;
_9 = _26;
_21.1 = _28.1;
_28.1.0 = _20 - _21.1.1;
_23.0 = _17 | _22.1;
_9 = _26;
_29 = _20 as u32;
_21 = (_28.0, _28.1);
_21.1.0 = !_22.1;
Goto(bb13)
}
bb13 = {
Call(_32 = dump_var(13_usize, 5_usize, Move(_5), 17_usize, Move(_17), 20_usize, Move(_20), 28_usize, Move(_28)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_32 = dump_var(13_usize, 3_usize, Move(_3), 8_usize, Move(_8), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_32 = dump_var(13_usize, 22_usize, Move(_22), 2_usize, Move(_2), 1_usize, Move(_1), 33_usize, _33), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *const u128,mut _2: u128,mut _3: *const u128,mut _4: [i64; 4],mut _5: usize,mut _6: usize,mut _7: u128,mut _8: (*const u128, u128, usize),mut _9: u128,mut _10: *const u128,mut _11: usize,mut _12: usize,mut _13: *const u128,mut _14: u128) -> u128 {
mir! {
type RET = u128;
let _15: Adt54;
let _16: i8;
let _17: isize;
let _18: ((i32, (isize, isize)), (isize, isize), u16);
let _19: Adt48;
let _20: (bool,);
let _21: bool;
let _22: ((i32, (isize, isize)), (isize, isize), u16);
let _23: ();
let _24: ();
{
_4 = [7892944406301761809_i64,(-1661550793847034064_i64),(-7343911675932920596_i64),8060245897386562379_i64];
_6 = _11;
_10 = _13;
_8 = (_10, _2, _12);
_9 = (-13_isize) as u128;
RET = 37914_u16 as u128;
Call((*_10) = fn15(_1, _5, _7, _8.0, _10, _5, _8, _10, _8, _10, _5, _6, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_13) = 14637341042800002188_u64 as u128;
(*_10) = 2869094227_u32 as u128;
(*_10) = 9_i8 as u128;
_5 = _6;
_1 = _3;
_4 = [6467053223841566694_i64,(-8824543213686576516_i64),(-9023783807982592360_i64),(-7141279132510718393_i64)];
_14 = 124_u8 as u128;
_8.2 = _6;
RET = !_8.1;
_7 = _8.1;
RET = true as u128;
(*_13) = _8.1 - _8.1;
_12 = !_5;
Goto(bb2)
}
bb2 = {
_14 = 14_u8 as u128;
_16 = 14484654369255072565_u64 as i8;
Goto(bb3)
}
bb3 = {
_6 = _11 + _11;
_10 = _13;
RET = !(*_13);
_18.0.1.0 = (-9223372036854775808_isize);
_4 = [(-8579945901365914585_i64),(-5914058252555346316_i64),7241829397159918755_i64,(-2665940788262462803_i64)];
_17 = -_18.0.1.0;
RET = false as u128;
_13 = core::ptr::addr_of!(_7);
_21 = false;
_18.0.1 = (_17, _17);
_11 = _5 ^ _12;
_18.2 = !23238_u16;
_3 = _10;
RET = _18.0.1.0 as u128;
_10 = _3;
Goto(bb4)
}
bb4 = {
_18.1.0 = _18.0.1.0 * _18.0.1.0;
_4 = [7551556680468140546_i64,(-2566927773194944931_i64),4696230080271234045_i64,(-331348456768399828_i64)];
_14 = _2;
_18.0.0 = 1218478314_i32 - (-1472739957_i32);
RET = (*_13);
Goto(bb5)
}
bb5 = {
Call(_23 = dump_var(14_usize, 12_usize, Move(_12), 16_usize, Move(_16), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_23 = dump_var(14_usize, 6_usize, Move(_6), 2_usize, Move(_2), 24_usize, _24, 24_usize, _24), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: *const u128,mut _2: usize,mut _3: u128,mut _4: *const u128,mut _5: *const u128,mut _6: usize,mut _7: (*const u128, u128, usize),mut _8: *const u128,mut _9: (*const u128, u128, usize),mut _10: *const u128,mut _11: usize,mut _12: usize,mut _13: (*const u128, u128, usize)) -> u128 {
mir! {
type RET = u128;
let _14: [i128; 4];
let _15: [u128; 1];
let _16: [bool; 7];
let _17: ();
let _18: ();
{
RET = _13.1 >> _7.2;
_13.1 = _9.1;
RET = (-291459415_i32) as u128;
_13.0 = core::ptr::addr_of!(_13.1);
_11 = '\u{72361}' as usize;
_11 = 74_i8 as usize;
RET = _9.1 + _3;
_8 = core::ptr::addr_of!(_9.1);
_15 = [_3];
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(15_usize, 15_usize, Move(_15), 6_usize, Move(_6), 11_usize, Move(_11), 18_usize, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: (*const u128, u128, usize),mut _2: u128,mut _3: [u128; 1],mut _4: u128,mut _5: u128,mut _6: [u128; 1],mut _7: u128) -> [bool; 7] {
mir! {
type RET = [bool; 7];
let _8: i16;
let _9: Adt50;
let _10: [u128; 1];
let _11: [u128; 1];
let _12: isize;
let _13: (i32, (isize, isize));
let _14: f64;
let _15: u16;
let _16: i128;
let _17: u64;
let _18: i16;
let _19: u8;
let _20: f64;
let _21: isize;
let _22: (i32, (isize, isize));
let _23: char;
let _24: bool;
let _25: isize;
let _26: *mut u32;
let _27: [bool; 7];
let _28: (bool,);
let _29: *const u16;
let _30: [bool; 7];
let _31: isize;
let _32: isize;
let _33: i16;
let _34: u128;
let _35: [i128; 4];
let _36: (isize, isize);
let _37: bool;
let _38: bool;
let _39: char;
let _40: isize;
let _41: ();
let _42: ();
{
_3 = [_1.1];
RET = [false,true,true,true,false,true,false];
_3 = [_2];
_2 = 88_i8 as u128;
_2 = !_1.1;
_5 = !_1.1;
_8 = 4127_i16 + 9119_i16;
_7 = _2 << _1.2;
RET = [false,false,false,false,true,true,false];
_10 = [_5];
RET = [true,false,false,true,false,true,true];
_8 = -9692_i16;
_10 = _6;
_1.0 = core::ptr::addr_of!(_7);
_7 = _1.2 as u128;
_4 = _7 * _5;
_6 = [_1.1];
RET = [false,true,false,true,false,false,true];
_11 = _3;
RET = [false,false,false,true,true,false,false];
_7 = !_2;
_8 = !24618_i16;
Goto(bb1)
}
bb1 = {
_10 = _11;
_13.0 = 1773332232_i32;
_14 = _8 as f64;
_13.1 = (9223372036854775807_isize, 9223372036854775807_isize);
RET = [false,false,false,true,true,true,false];
_13.0 = (-712162331_i32) >> _4;
_9 = Adt50::Variant0 { fld0: _13.1 };
_12 = -Field::<(isize, isize)>(Variant(_9, 0), 0).0;
_13 = (961483543_i32, Field::<(isize, isize)>(Variant(_9, 0), 0));
_5 = _7 << _1.1;
_15 = !31029_u16;
_1.2 = 6824666854404334536_usize;
_15 = 10512_u16 | 5699_u16;
_13.1.0 = Field::<(isize, isize)>(Variant(_9, 0), 0).0 | Field::<(isize, isize)>(Variant(_9, 0), 0).0;
_13 = (283494853_i32, Field::<(isize, isize)>(Variant(_9, 0), 0));
_6 = _11;
_1.1 = 235588235_u32 as u128;
_1.2 = 16653025312802752996_usize | 7142302188803607838_usize;
_7 = _8 as u128;
_4 = _8 as u128;
_1.2 = 5_usize + 10087942128871149605_usize;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).0 = Field::<(isize, isize)>(Variant(_9, 0), 0).1 * _13.1.0;
_16 = -(-78391879714359021209120506227529735115_i128);
_14 = _15 as f64;
Goto(bb2)
}
bb2 = {
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).0 = 4065389448_u32 as isize;
_10 = [_5];
_13.1.0 = !Field::<(isize, isize)>(Variant(_9, 0), 0).1;
_2 = _5;
SetDiscriminant(_9, 0);
_3 = _11;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).0 = _14 as isize;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)) = (_12, _12);
_11 = _3;
match _13.1.1 {
0 => bb1,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb3 = {
_10 = _11;
_13.0 = 1773332232_i32;
_14 = _8 as f64;
_13.1 = (9223372036854775807_isize, 9223372036854775807_isize);
RET = [false,false,false,true,true,true,false];
_13.0 = (-712162331_i32) >> _4;
_9 = Adt50::Variant0 { fld0: _13.1 };
_12 = -Field::<(isize, isize)>(Variant(_9, 0), 0).0;
_13 = (961483543_i32, Field::<(isize, isize)>(Variant(_9, 0), 0));
_5 = _7 << _1.1;
_15 = !31029_u16;
_1.2 = 6824666854404334536_usize;
_15 = 10512_u16 | 5699_u16;
_13.1.0 = Field::<(isize, isize)>(Variant(_9, 0), 0).0 | Field::<(isize, isize)>(Variant(_9, 0), 0).0;
_13 = (283494853_i32, Field::<(isize, isize)>(Variant(_9, 0), 0));
_6 = _11;
_1.1 = 235588235_u32 as u128;
_1.2 = 16653025312802752996_usize | 7142302188803607838_usize;
_7 = _8 as u128;
_4 = _8 as u128;
_1.2 = 5_usize + 10087942128871149605_usize;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).0 = Field::<(isize, isize)>(Variant(_9, 0), 0).1 * _13.1.0;
_16 = -(-78391879714359021209120506227529735115_i128);
_14 = _15 as f64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_21 = _8 as isize;
match _13.1.1 {
9223372036854775807 => bb7,
_ => bb4
}
}
bb7 = {
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)) = _13.1;
_1.1 = _8 as u128;
_13.1.0 = _13.1.1;
_21 = Field::<(isize, isize)>(Variant(_9, 0), 0).1;
_8 = !(-32172_i16);
_1.1 = _5;
_1.0 = core::ptr::addr_of!(_4);
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).1 = _13.1.1 | Field::<(isize, isize)>(Variant(_9, 0), 0).0;
_10 = [_1.1];
_20 = _14 * _14;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).0 = Field::<(isize, isize)>(Variant(_9, 0), 0).1 - _13.1.1;
_17 = _1.2 as u64;
_22.1.1 = Field::<(isize, isize)>(Variant(_9, 0), 0).0 * Field::<(isize, isize)>(Variant(_9, 0), 0).0;
Goto(bb8)
}
bb8 = {
_12 = Field::<(isize, isize)>(Variant(_9, 0), 0).0 * _13.1.0;
_22.1 = (_12, Field::<(isize, isize)>(Variant(_9, 0), 0).0);
_16 = (-76616097250036823708208443149046753443_i128) >> _12;
_13.0 = (-1116135028_i32);
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)) = _22.1;
SetDiscriminant(_9, 0);
_18 = _8 & _8;
_16 = (-6992934420230161767957637969798131363_i128) | 145014520045562617820429077456680928358_i128;
_5 = !_2;
_12 = _22.1.1;
_22.0 = _13.0 | _13.0;
_22.1.1 = _12;
_22.1.0 = _22.1.1;
_16 = 28549635301304661084610845807494170353_i128;
_23 = '\u{80ce9}';
_16 = -169679771186591948710698856171109752034_i128;
_1.0 = core::ptr::addr_of!(_5);
_14 = _20;
_13.1.0 = -_22.1.0;
_27 = [false,false,false,false,true,true,true];
_15 = 47217_u16 | 34968_u16;
_19 = !79_u8;
_13.1 = (_12, _22.1.1);
Call(_4 = core::intrinsics::bswap(_5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = _5 >> _5;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)) = (_12, _13.1.0);
_1.2 = 12764942259848220811_usize;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).1 = _22.1.0 + _12;
_13 = (_22.0, Field::<(isize, isize)>(Variant(_9, 0), 0));
_1.2 = 5210808269405165679_usize * 7_usize;
_13.1.1 = _5 as isize;
_13.1.1 = !_12;
_18 = _8 & _8;
SetDiscriminant(_9, 1);
_28 = (true,);
place!(Field::<[i64; 4]>(Variant(_9, 1), 4)) = [1741639326694188516_i64,1243778444759786000_i64,3435774648451654743_i64,172926759239694440_i64];
Call(_17 = core::intrinsics::bswap(14565647252755758242_u64), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1.0 = core::ptr::addr_of!(_7);
_20 = _14;
_11 = [_1.1];
_13.1 = _22.1;
_4 = _5;
_28 = (true,);
_13.0 = _22.0 + _22.0;
_20 = _14;
_23 = '\u{eb849}';
_13.1 = (_12, _22.1.0);
_18 = _8 * _8;
_3 = [_5];
_13.1 = (_12, _12);
place!(Field::<[u128; 1]>(Variant(_9, 1), 3)) = _3;
_1.2 = 5563410401586105285_usize;
_1.2 = 1_usize;
_10 = [_4];
_27 = [_28.0,_28.0,_28.0,_28.0,_28.0,_28.0,_28.0];
place!(Field::<(bool,)>(Variant(_9, 1), 0)) = (_28.0,);
RET = [_28.0,Field::<(bool,)>(Variant(_9, 1), 0).0,Field::<(bool,)>(Variant(_9, 1), 0).0,_28.0,Field::<(bool,)>(Variant(_9, 1), 0).0,_28.0,Field::<(bool,)>(Variant(_9, 1), 0).0];
_7 = _4;
_1.1 = _7;
_16 = (-7203614377263582088291671744051804893_i128);
match _16 {
333078752543674881375082935687716406563 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_22 = (_13.0, _13.1);
_6 = [_1.1];
place!(Field::<[u128; 1]>(Variant(_9, 1), 3)) = [_1.1];
place!(Field::<[i64; 4]>(Variant(_9, 1), 4)) = [(-5902694334665320552_i64),(-4135688000671701595_i64),6544712993718079088_i64,(-6038176173670963963_i64)];
_23 = '\u{d7d33}';
_3 = Field::<[u128; 1]>(Variant(_9, 1), 3);
Goto(bb13)
}
bb13 = {
_19 = !113_u8;
_19 = 23_u8;
_3 = [_7];
place!(Field::<(bool,)>(Variant(_9, 1), 0)) = _28;
_25 = _13.1.1;
Goto(bb14)
}
bb14 = {
_1.1 = _4 >> _4;
_6 = [_1.1];
_22.1 = (_21, _25);
_1.1 = _25 as u128;
_25 = _13.1.1;
_11 = [_2];
place!(Field::<[i64; 4]>(Variant(_9, 1), 4)) = [(-8895322773958380622_i64),(-3180201693642738819_i64),5518828928753790203_i64,5234667276870324315_i64];
_11 = [_2];
_34 = _17 as u128;
_20 = _14 * _14;
_11 = [_2];
_23 = '\u{1b760}';
_33 = _18;
_6 = [_2];
_6 = _11;
_9 = Adt50::Variant0 { fld0: _13.1 };
_1.2 = 3_usize;
_9 = Adt50::Variant0 { fld0: _13.1 };
_36.0 = Field::<(isize, isize)>(Variant(_9, 0), 0).1 << _2;
place!(Field::<(isize, isize)>(Variant(_9, 0), 0)).1 = -_36.0;
_13 = (_22.0, Field::<(isize, isize)>(Variant(_9, 0), 0));
_23 = '\u{66311}';
_30 = [_28.0,_28.0,_28.0,_28.0,_28.0,_28.0,_28.0];
_22.1.1 = _23 as isize;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(16_usize, 34_usize, Move(_34), 10_usize, Move(_10), 4_usize, Move(_4), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(16_usize, 12_usize, Move(_12), 7_usize, Move(_7), 27_usize, Move(_27), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(16_usize, 11_usize, Move(_11), 5_usize, Move(_5), 28_usize, Move(_28), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: (isize,),mut _6: usize,mut _7: isize,mut _8: (isize,),mut _9: (isize,),mut _10: isize,mut _11: isize) -> u128 {
mir! {
type RET = u128;
let _12: (*mut i8, f64, u128, isize);
let _13: *mut u128;
let _14: [i64; 4];
let _15: isize;
let _16: f64;
let _17: (isize,);
let _18: isize;
let _19: (i32, (isize, isize));
let _20: isize;
let _21: i128;
let _22: ([i64; 4], [bool; 7]);
let _23: f32;
let _24: (isize,);
let _25: i64;
let _26: u8;
let _27: Adt50;
let _28: [u128; 1];
let _29: [u32; 8];
let _30: isize;
let _31: (f32,);
let _32: bool;
let _33: [u32; 8];
let _34: Adt53;
let _35: (isize,);
let _36: (i32, (isize, isize));
let _37: bool;
let _38: i128;
let _39: char;
let _40: (bool,);
let _41: *mut u32;
let _42: char;
let _43: [u128; 1];
let _44: f64;
let _45: u16;
let _46: [u128; 1];
let _47: [i64; 4];
let _48: ();
let _49: ();
{
_11 = _8.0 << _9.0;
RET = true as u128;
_12.1 = _7 as f64;
_2 = !_3;
_13 = core::ptr::addr_of_mut!(RET);
_9 = (_8.0,);
_14 = [(-417841554954647910_i64),4538859214501758211_i64,(-1429645961177203656_i64),(-798294179898566692_i64)];
_9.0 = (-1769167402_i32) as isize;
_11 = RET as isize;
(*_13) = 246671029486212926976602048871216189343_u128;
_15 = _8.0;
_8.0 = _1 + _3;
_6 = 3_usize;
_12.3 = -_3;
_9.0 = _15;
RET = 5913690155819450912238098006520748557_u128;
_14[_6] = 3130077055024097142_i64 << _1;
RET = !121876920368106008554735216873703777895_u128;
RET = 55867555803767084033431286886528444835_i128 as u128;
_16 = _12.1;
_12.3 = 32691_i16 as isize;
_2 = _3;
_6 = !6548482431021628911_usize;
_13 = core::ptr::addr_of_mut!(_12.2);
_16 = -_12.1;
_8.0 = _1;
Call(_10 = core::intrinsics::bswap(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _8.0;
_8.0 = false as isize;
_5.0 = _9.0 << _1;
_10 = _2 & _15;
_11 = _5.0 >> _3;
_17 = _5;
(*_13) = !RET;
_1 = 167550217198634559598833410548638650531_i128 as isize;
_3 = !_4;
_12.2 = (-2447095470069951658_i64) as u128;
_4 = -_17.0;
_18 = -_4;
_9 = _5;
_19.1.0 = 23972_u16 as isize;
_6 = 706576330_u32 as usize;
RET = (*_13) - _12.2;
_6 = 3_usize ^ 9702030482075419876_usize;
(*_13) = RET;
_19.1.1 = _5.0;
_14 = [5127717826868731218_i64,3908256523362466217_i64,3368995027161857620_i64,5807204819478942896_i64];
_19.0 = 388493595_i32 - 698389251_i32;
Goto(bb2)
}
bb2 = {
_8 = _9;
_19.1 = (_9.0, _17.0);
_6 = 4406983483702119851_usize - 1_usize;
_8.0 = !_19.1.0;
(*_13) = RET + RET;
RET = (*_13) * (*_13);
_17.0 = _6 as isize;
_23 = 8181_i16 as f32;
(*_13) = 3528771606_u32 as u128;
_2 = _16 as isize;
_17 = _5;
_25 = (-5477686992095598177_i64) | 4769257449393003744_i64;
_22.1 = [true,true,false,true,true,true,false];
_21 = true as i128;
_11 = _9.0 << _4;
Goto(bb3)
}
bb3 = {
_20 = '\u{bc751}' as isize;
_25 = !2087764857632859119_i64;
_4 = !_10;
RET = (*_13);
_18 = _16 as isize;
_10 = _19.1.0;
_10 = _11;
_12.1 = _23 as f64;
_20 = _10;
RET = _21 as u128;
_11 = !_10;
_19.1.0 = (-49_i8) as isize;
_4 = _20;
_4 = _5.0;
(*_13) = !RET;
_13 = core::ptr::addr_of_mut!(_12.2);
_4 = '\u{d5bd9}' as isize;
RET = !(*_13);
Call(_12 = fn18(_11, _17, _17.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24 = _8;
_26 = !9_u8;
RET = (*_13) | _12.2;
_6 = 4_usize;
_17.0 = _18;
_29 = [3466051007_u32,1665150439_u32,3931719413_u32,1201533152_u32,2760604361_u32,2458974121_u32,4247308182_u32,1823771678_u32];
_22.1 = [false,true,false,true,false,true,false];
_24.0 = _6 as isize;
_10 = !_7;
_5 = _8;
_22.1 = [false,false,false,false,true,false,true];
_7 = 14030181593880554005_u64 as isize;
_24.0 = _19.0 as isize;
_19.1.0 = _26 as isize;
(*_13) = RET;
Goto(bb5)
}
bb5 = {
_22.0 = _14;
_4 = _12.3 >> _12.3;
_21 = 8176675641959687557_u64 as i128;
_5.0 = (-120_i8) as isize;
_19.0 = 1552360910_i32;
match _29[_6] {
0 => bb4,
1 => bb2,
2 => bb6,
2760604361 => bb8,
_ => bb7
}
}
bb6 = {
_8 = _9;
_19.1 = (_9.0, _17.0);
_6 = 4406983483702119851_usize - 1_usize;
_8.0 = !_19.1.0;
(*_13) = RET + RET;
RET = (*_13) * (*_13);
_17.0 = _6 as isize;
_23 = 8181_i16 as f32;
(*_13) = 3528771606_u32 as u128;
_2 = _16 as isize;
_17 = _5;
_25 = (-5477686992095598177_i64) | 4769257449393003744_i64;
_22.1 = [true,true,false,true,true,true,false];
_21 = true as i128;
_11 = _9.0 << _4;
Goto(bb3)
}
bb7 = {
_20 = '\u{bc751}' as isize;
_25 = !2087764857632859119_i64;
_4 = !_10;
RET = (*_13);
_18 = _16 as isize;
_10 = _19.1.0;
_10 = _11;
_12.1 = _23 as f64;
_20 = _10;
RET = _21 as u128;
_11 = !_10;
_19.1.0 = (-49_i8) as isize;
_4 = _20;
_4 = _5.0;
(*_13) = !RET;
_13 = core::ptr::addr_of_mut!(_12.2);
_4 = '\u{d5bd9}' as isize;
RET = !(*_13);
Call(_12 = fn18(_11, _17, _17.0), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_33 = [_29[_6],_29[_6],_29[_6],_29[_6],_29[_6],_29[_6],_29[_6],_29[_6]];
_9 = (_12.3,);
_32 = _22.1[_6];
_31 = (_23,);
Goto(bb9)
}
bb9 = {
(*_13) = '\u{ba215}' as u128;
_28 = [_12.2];
_11 = _19.1.1;
_12.2 = RET * RET;
_37 = _32;
_12.2 = !RET;
_25 = 3721244966388080888_i64;
_33[_6] = !_29[_6];
(*_13) = RET;
_2 = -_20;
_33 = [_29[_6],_29[_6],_29[_6],_29[_6],_29[_6],_29[_6],_29[_6],_29[_6]];
_11 = _19.0 as isize;
_36.0 = _29[_6] as i32;
_38 = _21 & _21;
RET = _36.0 as u128;
_15 = !_12.3;
Goto(bb10)
}
bb10 = {
_13 = core::ptr::addr_of_mut!((*_13));
_19.1 = (_4, _20);
_27 = Adt50::Variant0 { fld0: _19.1 };
_36.1.1 = _38 as isize;
_18 = _6 as isize;
Goto(bb11)
}
bb11 = {
place!(Field::<(isize, isize)>(Variant(_27, 0), 0)) = (_15, _15);
_36.1 = (_20, Field::<(isize, isize)>(Variant(_27, 0), 0).0);
_21 = _38;
_19.0 = -_36.0;
_35 = (_4,);
_33[_6] = _29[_6] & _29[_6];
_10 = _16 as isize;
_8 = (_9.0,);
_36.1 = Field::<(isize, isize)>(Variant(_27, 0), 0);
_22.1 = [_32,_37,_32,_37,_32,_32,_37];
_36.1 = (_15, _35.0);
_32 = !_22.1[_6];
_36.0 = '\u{e30f7}' as i32;
_32 = _37 & _37;
_30 = _26 as isize;
match _29[_6] {
0 => bb4,
1 => bb2,
2 => bb7,
3 => bb12,
4 => bb13,
5 => bb14,
2760604361 => bb16,
_ => bb15
}
}
bb12 = {
_13 = core::ptr::addr_of_mut!((*_13));
_19.1 = (_4, _20);
_27 = Adt50::Variant0 { fld0: _19.1 };
_36.1.1 = _38 as isize;
_18 = _6 as isize;
Goto(bb11)
}
bb13 = {
_8 = _9;
_19.1 = (_9.0, _17.0);
_6 = 4406983483702119851_usize - 1_usize;
_8.0 = !_19.1.0;
(*_13) = RET + RET;
RET = (*_13) * (*_13);
_17.0 = _6 as isize;
_23 = 8181_i16 as f32;
(*_13) = 3528771606_u32 as u128;
_2 = _16 as isize;
_17 = _5;
_25 = (-5477686992095598177_i64) | 4769257449393003744_i64;
_22.1 = [true,true,false,true,true,true,false];
_21 = true as i128;
_11 = _9.0 << _4;
Goto(bb3)
}
bb14 = {
_8 = _9;
_19.1 = (_9.0, _17.0);
_6 = 4406983483702119851_usize - 1_usize;
_8.0 = !_19.1.0;
(*_13) = RET + RET;
RET = (*_13) * (*_13);
_17.0 = _6 as isize;
_23 = 8181_i16 as f32;
(*_13) = 3528771606_u32 as u128;
_2 = _16 as isize;
_17 = _5;
_25 = (-5477686992095598177_i64) | 4769257449393003744_i64;
_22.1 = [true,true,false,true,true,true,false];
_21 = true as i128;
_11 = _9.0 << _4;
Goto(bb3)
}
bb15 = {
_22.0 = _14;
_4 = _12.3 >> _12.3;
_21 = 8176675641959687557_u64 as i128;
_5.0 = (-120_i8) as isize;
_19.0 = 1552360910_i32;
match _29[_6] {
0 => bb4,
1 => bb2,
2 => bb6,
2760604361 => bb8,
_ => bb7
}
}
bb16 = {
RET = _12.2 + (*_13);
_28 = [(*_13)];
_29 = _33;
Goto(bb17)
}
bb17 = {
Call(_48 = dump_var(17_usize, 7_usize, Move(_7), 9_usize, Move(_9), 1_usize, Move(_1), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(17_usize, 36_usize, Move(_36), 24_usize, Move(_24), 5_usize, Move(_5), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(17_usize, 32_usize, Move(_32), 22_usize, Move(_22), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(17_usize, 33_usize, Move(_33), 19_usize, Move(_19), 20_usize, Move(_20), 49_usize, _49), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: (isize,),mut _3: isize) -> (*mut i8, f64, u128, isize) {
mir! {
type RET = (*mut i8, f64, u128, isize);
let _4: char;
let _5: [i64; 4];
let _6: (i32, (isize, isize));
let _7: [i64; 4];
let _8: i8;
let _9: Adt52;
let _10: char;
let _11: Adt39;
let _12: bool;
let _13: f64;
let _14: i8;
let _15: bool;
let _16: [i128; 4];
let _17: Adt55;
let _18: ();
let _19: ();
{
RET.2 = 5405613215703251944_usize as u128;
RET.3 = !_1;
Goto(bb1)
}
bb1 = {
_2.0 = true as isize;
RET.1 = 13968_i16 as f64;
_2.0 = -RET.3;
RET.1 = 3_usize as f64;
_6.1.1 = RET.3;
_5 = [(-3921794423193498052_i64),(-6353628571154541155_i64),(-2465773294897773516_i64),(-8420184854367130910_i64)];
RET.1 = RET.2 as f64;
_1 = _6.1.1;
_6.0 = 1011376340_i32;
_6.1 = (_1, RET.3);
RET.1 = (-124_i8) as f64;
RET.3 = 91_i8 as isize;
RET.2 = 141299144157237308185452432135234920590_u128;
Call(RET.3 = core::intrinsics::bswap(_2.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _5;
_6.1.0 = _3 << RET.3;
_1 = _6.1.1 >> _3;
_2.0 = _1;
_7 = [474783492628295680_i64,8910394493722665972_i64,(-8375431650517606700_i64),(-6142244775936295214_i64)];
RET.2 = 159091425273584677785058402747673218223_u128;
RET.2 = 147879457848525122929806086190242842984_u128 & 108817375639707622352548139572747005013_u128;
_5 = _7;
_8 = !100_i8;
_2 = (_1,);
Goto(bb3)
}
bb3 = {
RET.1 = _8 as f64;
RET.2 = 146774595633869314318410621104661141822_i128 as u128;
RET.2 = 223737185708305761848635599301360552096_u128;
_4 = '\u{82f87}';
_8 = 201_u8 as i8;
match _6.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1011376340 => bb9,
_ => bb8
}
}
bb4 = {
_7 = _5;
_6.1.0 = _3 << RET.3;
_1 = _6.1.1 >> _3;
_2.0 = _1;
_7 = [474783492628295680_i64,8910394493722665972_i64,(-8375431650517606700_i64),(-6142244775936295214_i64)];
RET.2 = 159091425273584677785058402747673218223_u128;
RET.2 = 147879457848525122929806086190242842984_u128 & 108817375639707622352548139572747005013_u128;
_5 = _7;
_8 = !100_i8;
_2 = (_1,);
Goto(bb3)
}
bb5 = {
_2.0 = true as isize;
RET.1 = 13968_i16 as f64;
_2.0 = -RET.3;
RET.1 = 3_usize as f64;
_6.1.1 = RET.3;
_5 = [(-3921794423193498052_i64),(-6353628571154541155_i64),(-2465773294897773516_i64),(-8420184854367130910_i64)];
RET.1 = RET.2 as f64;
_1 = _6.1.1;
_6.0 = 1011376340_i32;
_6.1 = (_1, RET.3);
RET.1 = (-124_i8) as f64;
RET.3 = 91_i8 as isize;
RET.2 = 141299144157237308185452432135234920590_u128;
Call(RET.3 = core::intrinsics::bswap(_2.0), ReturnTo(bb2), UnwindUnreachable())
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
_6.1.1 = _6.1.0 ^ _2.0;
_3 = _6.1.1 + _1;
RET.2 = 997982462928286135487356805300130498_u128;
RET.0 = core::ptr::addr_of_mut!(_8);
RET.3 = _2.0 & _6.1.1;
_7 = [3264303047700217726_i64,3056701412246399514_i64,(-521899340780160349_i64),3436123695616042222_i64];
RET.3 = _2.0;
_14 = _8;
RET.3 = _6.1.1 * _3;
_17.fld6 = (_3, _3);
_17.fld1.0 = core::ptr::addr_of!(RET.2);
_15 = true;
_6.1.1 = RET.3;
_17.fld3 = Adt50::Variant0 { fld0: _17.fld6 };
_12 = _15 | _15;
RET.2 = 149009995040123766089598737778558492029_u128;
_12 = _6.1.1 < Field::<(isize, isize)>(Variant(_17.fld3, 0), 0).0;
_17.fld1.0 = core::ptr::addr_of!(_17.fld1.1);
_17.fld5.1 = -RET.1;
_9 = Adt52::Variant0 { fld0: _12,fld1: 81014038573931955128626715013220191909_i128 };
RET.3 = Field::<(isize, isize)>(Variant(_17.fld3, 0), 0).0;
SetDiscriminant(_17.fld3, 1);
Goto(bb10)
}
bb10 = {
Call(_18 = dump_var(18_usize, 8_usize, Move(_8), 3_usize, Move(_3), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_18 = dump_var(18_usize, 2_usize, Move(_2), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: bool,mut _2: (f32,),mut _3: bool,mut _4: isize,mut _5: (bool,),mut _6: [u128; 1]) -> isize {
mir! {
type RET = isize;
let _7: (*const u128, u128, usize);
let _8: (f32,);
let _9: (f32,);
let _10: Adt52;
let _11: ([i64; 4], [bool; 7]);
let _12: *mut u128;
let _13: *const u16;
let _14: Adt42;
let _15: u128;
let _16: char;
let _17: isize;
let _18: ();
let _19: ();
{
_6 = [187288243882840832395290032301544072963_u128];
_5.0 = _3;
_7.2 = 9071002122539770754_usize;
Goto(bb1)
}
bb1 = {
_7.0 = core::ptr::addr_of!(_7.1);
RET = _4 >> _7.2;
_1 = !_3;
_7.1 = 175791721598655471890663077544192422819_u128;
_3 = RET != RET;
_1 = !_5.0;
_5 = (_1,);
_8 = (_2.0,);
_9 = (_2.0,);
_8.0 = _2.0 + _2.0;
_7.2 = 7597252498965280016_usize;
RET = '\u{66273}' as isize;
_6 = [_7.1];
_6 = [_7.1];
match _7.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
7597252498965280016 => bb10,
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
_2.0 = -_8.0;
_4 = RET;
_10 = Adt52::Variant0 { fld0: _1,fld1: (-39396748306388682316359751434070706437_i128) };
place!(Field::<i128>(Variant(_10, 0), 1)) = (-97192339033699315485216935746131010228_i128);
_2.0 = _8.0 + _9.0;
_7.1 = 105472835009148703814097318558043302287_u128 << Field::<i128>(Variant(_10, 0), 1);
SetDiscriminant(_10, 3);
_11.1 = [_5.0,_3,_1,_1,_5.0,_3,_1];
RET = _4;
place!(Field::<(f32,)>(Variant(_10, 3), 2)) = _8;
_4 = -RET;
place!(Field::<char>(Variant(_10, 3), 1)) = '\u{7b7b6}';
_10 = Adt52::Variant0 { fld0: _5.0,fld1: 112374798520411854151656262700043893333_i128 };
_7.0 = core::ptr::addr_of!(_7.1);
_12 = core::ptr::addr_of_mut!(_7.1);
(*_12) = !258509587195209978996817784204050326356_u128;
match _7.2 {
7597252498965280016 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_2.0 = (-114_i8) as f32;
_6 = [(*_12)];
place!(Field::<bool>(Variant(_10, 0), 0)) = _3;
_6 = [(*_12)];
_7.2 = 7_usize;
(*_12) = !202973449350181978343526490019641733819_u128;
(*_12) = !79802049669112867693358598484085426530_u128;
match _7.2 {
0 => bb1,
1 => bb7,
2 => bb10,
7 => bb13,
_ => bb4
}
}
bb13 = {
_7.0 = core::ptr::addr_of!(_15);
match _7.2 {
0 => bb9,
1 => bb7,
2 => bb14,
7 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_5.0 = !Field::<bool>(Variant(_10, 0), 0);
place!(Field::<bool>(Variant(_10, 0), 0)) = _3;
_8.0 = 1753803444895436425_i64 as f32;
_15 = !_7.1;
_2.0 = _8.0 * _9.0;
_11.0 = [7420273665907410536_i64,137839496254818147_i64,(-3082981083108691223_i64),1668809304580469483_i64];
_11.1 = [_5.0,_5.0,_5.0,_3,_1,_3,_1];
(*_12) = RET as u128;
_11.0 = [2122667821221211512_i64,1530964347705369836_i64,5798800236562235449_i64,1939758609561356611_i64];
_11.1 = [Field::<bool>(Variant(_10, 0), 0),_3,_3,_3,Field::<bool>(Variant(_10, 0), 0),_1,Field::<bool>(Variant(_10, 0), 0)];
Goto(bb17)
}
bb17 = {
Call(_18 = dump_var(19_usize, 5_usize, Move(_5), 6_usize, Move(_6), 4_usize, Move(_4), 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(34758_u16), std::hint::black_box((-152653891977054240201169898655519071338_i128)), std::hint::black_box(555149901234343845_u64), std::hint::black_box(1752209539_u32), std::hint::black_box(18245_i16), std::hint::black_box((-1429534012_i32)));
                
            }
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: bool,
fld1: (*mut i8, f64, u128, isize),
fld2: i128,
fld3: usize,
fld4: [i64; 4],
fld5: [bool; 7],
fld6: [u128; 1],

},
Variant1{
fld0: u64,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: [i64; 4],
fld1: *mut usize,

},
Variant1{
fld0: *const u16,
fld1: i64,
fld2: *mut i8,
fld3: *const u128,
fld4: f64,
fld5: (i32, (isize, isize)),

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: (*const u128, u128, usize),

},
Variant1{
fld0: *const u16,

},
Variant2{
fld0: u32,
fld1: ([i64; 4], [bool; 7]),

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *mut u128,
fld1: (isize,),
fld2: isize,
fld3: [u32; 8],
fld4: usize,

},
Variant1{
fld0: [i128; 4],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [i128; 4],
fld1: [i64; 4],
fld2: u16,
fld3: (i32, (isize, isize)),
fld4: *mut i8,
fld5: u8,
fld6: ([i64; 4], [bool; 7]),

},
Variant1{
fld0: Adt41,
fld1: (isize, isize),
fld2: usize,
fld3: i8,
fld4: u16,
fld5: (isize,),

},
Variant2{
fld0: ([i64; 4], [bool; 7]),
fld1: i8,
fld2: isize,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut u128,
fld1: u16,

},
Variant1{
fld0: i128,
fld1: *mut usize,
fld2: usize,
fld3: u32,
fld4: u8,

},
Variant2{
fld0: [u128; 1],
fld1: char,
fld2: [bool; 7],
fld3: *mut i8,
fld4: [i128; 4],

},
Variant3{
fld0: i8,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt40,
fld3: i8,
fld4: [i128; 4],

},
Variant1{
fld0: [bool; 7],
fld1: char,
fld2: isize,
fld3: [i64; 4],
fld4: u128,
fld5: Adt41,
fld6: *const i64,
fld7: ((i32, (isize, isize)), (isize, isize), u16),

},
Variant2{
fld0: u8,
fld1: [i64; 4],

},
Variant3{
fld0: ([i64; 4], [bool; 7]),
fld1: *const u16,
fld2: isize,
fld3: *const u128,
fld4: Adt43,
fld5: (*mut i8, f64, u128, isize),
fld6: (isize,),
fld7: *mut usize,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: ([i64; 4], [bool; 7]),
fld1: Adt39,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: f64,
fld1: (isize,),
fld2: Adt41,
fld3: *mut u32,

},
Variant1{
fld0: (f32,),
fld1: (bool,),
fld2: (isize,),
fld3: [i64; 4],
fld4: u32,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i64; 4],
fld1: f32,
fld2: *const u16,
fld3: u32,
fld4: f64,
fld5: i32,

},
Variant1{
fld0: *const u128,
fld1: Adt41,
fld2: *mut u32,

},
Variant2{
fld0: [i128; 4],
fld1: (*mut i8, f64, u128, isize),
fld2: *mut u32,
fld3: i8,
fld4: f64,
fld5: i32,
fld6: *mut i8,
fld7: Adt45,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
fld0: *mut u32,
fld1: *mut i8,
fld2: [u128; 1],
fld3: usize,

},
Variant1{
fld0: *const u128,
fld1: ((i32, (isize, isize)), (isize, isize), u16),
fld2: (f32,),
fld3: i8,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (isize, isize),

},
Variant1{
fld0: (bool,),
fld1: Adt48,
fld2: Adt45,
fld3: [u128; 1],
fld4: [i64; 4],

},
Variant2{
fld0: *mut usize,
fld1: (bool,),
fld2: Adt49,
fld3: [bool; 7],
fld4: *const u16,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: Adt47,
fld1: f64,
fld2: isize,
fld3: (*const u128, u128, usize),
fld4: u128,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: bool,
fld1: i128,

},
Variant1{
fld0: (i32, (isize, isize)),

},
Variant2{
fld0: [i128; 4],
fld1: Adt44,
fld2: isize,
fld3: Adt42,
fld4: i128,

},
Variant3{
fld0: *mut u32,
fld1: char,
fld2: (f32,),
fld3: (isize, isize),
fld4: Adt41,
fld5: f64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u32; 8],
fld1: char,
fld2: usize,
fld3: *const u128,

},
Variant1{
fld0: Adt43,
fld1: Adt40,
fld2: isize,
fld3: u16,
fld4: Adt46,
fld5: [i64; 4],
fld6: i64,

},
Variant2{
fld0: Adt49,
fld1: (*mut i8, f64, u128, isize),
fld2: [u32; 8],
fld3: Adt40,
fld4: (isize,),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt42,
fld1: *mut u32,
fld2: isize,
fld3: Adt43,

},
Variant1{
fld0: *mut u32,
fld1: *mut i8,
fld2: (*mut i8, f64, u128, isize),
fld3: [u128; 1],
fld4: Adt43,
fld5: i32,
fld6: [i64; 4],

},
Variant2{
fld0: *const u16,
fld1: Adt39,
fld2: isize,
fld3: i64,

},
Variant3{
fld0: [u32; 8],
fld1: *mut u128,
fld2: u64,
fld3: (f32,),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [u32; 8],
fld1: (*const u128, u128, usize),
fld2: u8,
fld3: Adt50,
fld4: *mut i8,
fld5: (*mut i8, f64, u128, isize),
fld6: (isize, isize),
fld7: [i64; 4],
}

