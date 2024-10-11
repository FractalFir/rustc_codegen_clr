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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: u64,mut _5: usize,mut _6: i32,mut _7: u32,mut _8: i128) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _9: Adt55;
let _10: [i32; 5];
let _11: *const f64;
let _12: i64;
let _13: u8;
let _14: Adt51;
let _15: i16;
let _16: i16;
let _17: bool;
let _18: Adt51;
let _19: [isize; 8];
let _20: [u8; 1];
let _21: (f64, u128);
let _22: [u32; 7];
let _23: i32;
let _24: Adt44;
let _25: Adt45;
let _26: Adt46;
let _27: usize;
let _28: [isize; 5];
let _29: ();
let _30: ();
{
_4 = !2299616192055832794_u64;
_2 = '\u{2a2d}';
_7 = !72987770_u32;
_3 = 6421539829477292209670024276432723126_u128;
_3 = 260517506317415562404111981299001731718_u128;
RET = [_4,_4,_4,_4,_4];
_5 = 13390974221711654075_usize | 5295671773499450182_usize;
_10 = [(-2096341543_i32),141427549_i32,(-414228135_i32),(-902762504_i32),(-1383586501_i32)];
_7 = !4023965455_u32;
_1 = !true;
_8 = _1 as i128;
_1 = !true;
_8 = (-166627964100537223578592723069233115556_i128) << _7;
_3 = 61951476435392137599601342310342774197_u128;
RET = [_4,_4,_4,_4,_4];
_6 = 943570202_i32 + (-393934718_i32);
match _3 {
61951476435392137599601342310342774197 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_10 = [_6,_6,_6,_6,_6];
_4 = 12417646173493611010_u64;
_5 = 18139499888502770015_usize;
_2 = '\u{3959f}';
_3 = 67202761569903324636924199300317591607_u128;
_7 = !2613140550_u32;
RET = [_4,_4,_4,_4,_4];
Call(_11 = fn1(_7, _4, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = 303331285700351973943975371812924758082_u128 * 275715377976979322812937855021971470373_u128;
_12 = !2706859243000071569_i64;
_1 = !true;
_3 = 317145069184070784773342891743156704888_u128 & 317661262678114575617315168109092027210_u128;
_3 = _6 as u128;
RET = [_4,_4,_4,_4,_4];
_1 = _8 <= _8;
_6 = _7 as i32;
_6 = _2 as i32;
_1 = false | true;
_13 = 9223372036854775807_isize as u8;
_10 = [_6,_6,_6,_6,_6];
_6 = (-25178_i16) as i32;
_3 = _12 as u128;
_17 = !_1;
_10 = [_6,_6,_6,_6,_6];
_13 = 80_u8 & 127_u8;
_8 = (-27676512946284551287143488845274900214_i128);
_15 = (-2187_i16);
_10 = [_6,_6,_6,_6,_6];
_16 = -_15;
_10 = [_6,_6,_6,_6,_6];
_6 = -(-1726377779_i32);
_5 = _12 as usize;
_8 = -(-162607888341591856690668953430838562425_i128);
match _15 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431768209269 => bb10,
_ => bb9
}
}
bb4 = {
_10 = [_6,_6,_6,_6,_6];
_4 = 12417646173493611010_u64;
_5 = 18139499888502770015_usize;
_2 = '\u{3959f}';
_3 = 67202761569903324636924199300317591607_u128;
_7 = !2613140550_u32;
RET = [_4,_4,_4,_4,_4];
Call(_11 = fn1(_7, _4, RET), ReturnTo(bb3), UnwindUnreachable())
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
RET = [_4,_4,_4,_4,_4];
_16 = _15 << _7;
_16 = _1 as i16;
_17 = _1;
_4 = 14779814649452056689_u64;
_6 = !(-1543930501_i32);
RET = [_4,_4,_4,_4,_4];
_1 = _5 == _5;
_10 = [_6,_6,_6,_6,_6];
_15 = _16;
_5 = 6_usize;
_13 = _3 as u8;
_20 = [_13];
_1 = _6 <= _6;
RET = [_4,_4,_4,_4,_4];
_23 = _6 | _6;
_20 = [_13];
_8 = (-165421308084596689382919582611100295741_i128);
_19[_5] = 40_i8 as isize;
RET = [_4,_4,_4,_4,_4];
Goto(bb11)
}
bb11 = {
_21.0 = _13 as f64;
_16 = _15 & _15;
_10 = [_6,_23,_23,_23,_6];
_2 = '\u{970eb}';
_13 = 120_u8 ^ 207_u8;
_19[_5] = (-9223372036854775808_isize);
_19 = [9223372036854775807_isize,(-9223372036854775808_isize),(-97_isize),(-9223372036854775808_isize),105_isize,9223372036854775807_isize,(-9223372036854775808_isize),31_isize];
_10 = [_6,_23,_6,_23,_23];
_17 = !_1;
_10 = [_6,_23,_23,_23,_23];
_4 = !15321033696372136748_u64;
_21.1 = _3;
match _19[_5] {
0 => bb12,
340282366920938463454151235394913435648 => bb14,
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
_1 = _17;
_15 = _16 ^ _16;
_4 = 2361212113328325399_u64;
_25.fld1 = core::ptr::addr_of!(_3);
_11 = core::ptr::addr_of!(_21.0);
_22[_5] = _7;
_11 = core::ptr::addr_of!((*_11));
_22 = [_7,_7,_7,_7,_7,_7,_7];
_8 = -141382591998855411758327586256865524357_i128;
_22 = [_7,_7,_7,_7,_7,_7,_7];
_25.fld2 = _15 * _16;
_5 = _21.1 as usize;
_8 = 151589299914068205986209512442229320652_i128 | (-52213643047985259437235189200563718843_i128);
_17 = _25.fld2 != _25.fld2;
_4 = 11343041024264211066_u64 * 17339148863113166208_u64;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(0_usize, 5_usize, Move(_5), 22_usize, Move(_22), 12_usize, Move(_12), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(0_usize, 15_usize, Move(_15), 3_usize, Move(_3), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(0_usize, 1_usize, Move(_1), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u32,mut _2: u64,mut _3: [u64; 5]) -> *const f64 {
mir! {
type RET = *const f64;
let _4: [isize; 5];
let _5: [isize; 5];
let _6: f64;
let _7: char;
let _8: (i128, u128, &'static f32, i32);
let _9: isize;
let _10: (u64, i32, u8);
let _11: Adt57;
let _12: Adt46;
let _13: (f32, u16, isize, u128);
let _14: char;
let _15: u16;
let _16: i16;
let _17: Adt58;
let _18: *const f64;
let _19: (i128, u128, &'static f32, i32);
let _20: isize;
let _21: [u8; 1];
let _22: *mut *const u128;
let _23: [bool; 2];
let _24: i16;
let _25: (f64, u128);
let _26: [i32; 5];
let _27: [u16; 7];
let _28: ();
let _29: ();
{
_1 = 2908679535_u32;
_2 = !2190720555670221002_u64;
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-58_isize),26_isize];
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-75_isize),9223372036854775807_isize];
_4 = _5;
_3 = [_2,_2,_2,_2,_2];
_1 = 797502800_u32 - 3625951235_u32;
_2 = 11704308629994969407_u64 * 3966423902244212415_u64;
_5 = _4;
_1 = !3568858863_u32;
Call(_6 = fn2(_4, _5, _5, _5, _2, _5, _5, _4, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = core::ptr::addr_of!(_6);
_5 = [9223372036854775807_isize,9223372036854775807_isize,76_isize,(-91_isize),(-9223372036854775808_isize)];
_8.3 = -(-999138033_i32);
_8.0 = 110928510087011712141102572522703730617_i128;
_7 = '\u{b3d5d}';
_8.3 = 182744187_i32;
_8.0 = 331035750895939797059367578642927906919_u128 as i128;
(*RET) = (-13_i8) as f64;
_8.1 = 171979338706614457656410941169640442038_u128 * 95707318694926820576760089470524911427_u128;
_4 = [22_isize,52_isize,113_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_8.1 = _1 as u128;
_10.2 = 173_u8 << _8.3;
_10.0 = !_2;
_1 = !2314655675_u32;
_8.0 = _8.1 as i128;
_10.0 = _8.1 as u64;
RET = core::ptr::addr_of!(_6);
(*RET) = _1 as f64;
_3 = [_2,_2,_2,_2,_2];
_10 = (_2, _8.3, 93_u8);
_6 = 5_usize as f64;
RET = core::ptr::addr_of!(_6);
_7 = '\u{b461d}';
RET = core::ptr::addr_of!(_6);
_8.1 = 32546690817263653664258057282887271609_u128 | 266243522063494125983260242672934574379_u128;
_9 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
(*RET) = _10.2 as f64;
match _10.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
93 => bb8,
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
_2 = _9 as u64;
_10 = (_2, _8.3, 118_u8);
(*RET) = 90_i8 as f64;
_1 = 2532430265_u32 + 1098183792_u32;
_9 = -(-9223372036854775808_isize);
RET = core::ptr::addr_of!((*RET));
(*RET) = _9 as f64;
_10 = (_2, _8.3, 173_u8);
_8.2 = &_13.0;
_6 = _8.0 as f64;
_10.2 = 226_u8;
Goto(bb9)
}
bb9 = {
_3 = [_10.0,_10.0,_2,_2,_10.0];
Goto(bb10)
}
bb10 = {
_10.1 = _8.3 - _8.3;
_8.2 = &_13.0;
_8.2 = &_13.0;
_8.3 = (*RET) as i32;
_15 = 37413_u16;
_13.2 = _9;
_13.3 = _8.1;
(*RET) = 4603895616444703992_i64 as f64;
_4 = _5;
_8.1 = _13.3;
Goto(bb11)
}
bb11 = {
_2 = _10.0;
_15 = !48187_u16;
_13.0 = (-5656_i16) as f32;
_7 = '\u{b4aa}';
_13.0 = _15 as f32;
_11 = Adt57::Variant2 { fld0: _13.0,fld1: _7,fld2: 67_i8 };
place!(Field::<i8>(Variant(_11, 2), 2)) = (-105_i8);
RET = core::ptr::addr_of!(_6);
_13.2 = (-1254996728829413689_i64) as isize;
_16 = (-3964_i16) ^ (-16915_i16);
_5 = [_9,_13.2,_9,_9,_9];
place!(Field::<char>(Variant(_11, 2), 1)) = _7;
SetDiscriminant(_11, 3);
_14 = _7;
place!(Field::<u64>(Variant(_11, 3), 5)) = _10.0 + _10.0;
place!(Field::<i8>(Variant(_11, 3), 3)) = -33_i8;
_9 = !_13.2;
_3 = [_2,Field::<u64>(Variant(_11, 3), 5),Field::<u64>(Variant(_11, 3), 5),Field::<u64>(Variant(_11, 3), 5),Field::<u64>(Variant(_11, 3), 5)];
(*RET) = _13.0 as f64;
_8.1 = _13.3;
_4 = [_9,_13.2,_9,_13.2,_9];
place!(Field::<*mut u32>(Variant(_11, 3), 1)) = core::ptr::addr_of_mut!(_1);
_13.1 = !_15;
(*RET) = _9 as f64;
Goto(bb12)
}
bb12 = {
_8.2 = &_13.0;
place!(Field::<Adt54>(Variant(_11, 3), 2)) = Adt54::Variant0 { fld0: _3,fld1: _8.0 };
_10 = (Field::<u64>(Variant(_11, 3), 5), _8.3, 171_u8);
_8.1 = _13.3 ^ _13.3;
_19.0 = _8.0 + Field::<i128>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 0), 1);
(*RET) = 7205972866491429378_usize as f64;
_8.2 = &_13.0;
Call(_10.2 = core::intrinsics::bswap(171_u8), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = Field::<i8>(Variant(_11, 3), 3) as f64;
_19.1 = _8.1;
_10 = (Field::<u64>(Variant(_11, 3), 5), _8.3, 50_u8);
place!(Field::<[u64; 5]>(Variant(place!(Field::<Adt54>(Variant(_11, 3), 2)), 0), 0)) = [_10.0,_10.0,_2,_10.0,Field::<u64>(Variant(_11, 3), 5)];
_19.3 = _10.1 & _8.3;
place!(Field::<*mut u32>(Variant(_11, 3), 1)) = core::ptr::addr_of_mut!(_1);
RET = core::ptr::addr_of!(_6);
SetDiscriminant(Field::<Adt54>(Variant(_11, 3), 2), 3);
place!(Field::<(u64, i32, u8)>(Variant(place!(Field::<Adt54>(Variant(_11, 3), 2)), 3), 0)).1 = _19.3;
_10 = (Field::<u64>(Variant(_11, 3), 5), _8.3, 114_u8);
place!(Field::<i8>(Variant(place!(Field::<Adt54>(Variant(_11, 3), 2)), 3), 2)) = Field::<i8>(Variant(_11, 3), 3);
place!(Field::<(u64, i32, u8)>(Variant(place!(Field::<Adt54>(Variant(_11, 3), 2)), 3), 0)) = (_10.0, _10.1, _10.2);
_8.0 = Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).2 as i128;
(*RET) = _15 as f64;
_13.2 = _8.0 as isize;
_10 = (Field::<u64>(Variant(_11, 3), 5), _19.3, Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).2);
_14 = _7;
_2 = Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).0;
_10.2 = !Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).2;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
place!(Field::<*mut u32>(Variant(_11, 3), 1)) = core::ptr::addr_of_mut!(_1);
_21 = [_10.2];
_19.2 = &_13.0;
_3 = [Field::<u64>(Variant(_11, 3), 5),Field::<u64>(Variant(_11, 3), 5),Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).0,Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).0,_10.0];
Goto(bb14)
}
bb14 = {
_21 = [_10.2];
_8.2 = Move(_19.2);
(*RET) = _19.1 as f64;
_18 = RET;
_13.3 = _19.1;
(*RET) = Field::<u64>(Variant(_11, 3), 5) as f64;
place!(Field::<*mut u32>(Variant(_11, 3), 1)) = core::ptr::addr_of_mut!(_1);
(*_18) = Field::<i8>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 2) as f64;
_10.2 = Field::<(u64, i32, u8)>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 3), 0).2;
place!(Field::<(u64, i32, u8)>(Variant(place!(Field::<Adt54>(Variant(_11, 3), 2)), 3), 0)) = _10;
place!(Field::<Adt54>(Variant(_11, 3), 2)) = Adt54::Variant0 { fld0: _3,fld1: _8.0 };
_10.1 = -_19.3;
(*RET) = Field::<i8>(Variant(_11, 3), 3) as f64;
_20 = _13.2 + _13.2;
_19.3 = 7208734818365854829_i64 as i32;
_8.1 = false as u128;
_23 = [false,false];
_19.2 = &_13.0;
_8.0 = Field::<i128>(Variant(Field::<Adt54>(Variant(_11, 3), 2), 0), 1);
_19.0 = -_8.0;
_7 = _14;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(1_usize, 9_usize, Move(_9), 7_usize, Move(_7), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(1_usize, 16_usize, Move(_16), 5_usize, Move(_5), 1_usize, Move(_1), 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [isize; 5],mut _2: [isize; 5],mut _3: [isize; 5],mut _4: [isize; 5],mut _5: u64,mut _6: [isize; 5],mut _7: [isize; 5],mut _8: [isize; 5],mut _9: [isize; 5]) -> f64 {
mir! {
type RET = f64;
let _10: char;
let _11: u64;
let _12: (f64, u128);
let _13: [usize; 7];
let _14: isize;
let _15: Adt50;
let _16: usize;
let _17: f64;
let _18: *const i16;
let _19: isize;
let _20: *mut *const u128;
let _21: [i32; 5];
let _22: f32;
let _23: Adt54;
let _24: char;
let _25: i16;
let _26: i32;
let _27: [bool; 2];
let _28: ();
let _29: ();
{
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-106_isize),(-56_isize),(-123_isize)];
_8 = _2;
_8 = [9223372036854775807_isize,(-126_isize),(-22_isize),9223372036854775807_isize,104_isize];
_3 = _4;
_8 = [9223372036854775807_isize,66_isize,49_isize,9223372036854775807_isize,9223372036854775807_isize];
_6 = _4;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_9 = _7;
_1 = _7;
_9 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET = 968907201_i32 as f64;
_4 = _1;
Goto(bb1)
}
bb1 = {
_4 = _9;
_1 = [(-119_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-123_isize),(-9223372036854775808_isize)];
_4 = _3;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),40_isize,9223372036854775807_isize,9223372036854775807_isize];
_10 = '\u{daf91}';
_5 = !12651856506396754002_u64;
_2 = [(-9223372036854775808_isize),24_isize,(-9223372036854775808_isize),15_isize,(-9223372036854775808_isize)];
_2 = _6;
Call(_2 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = '\u{754a0}';
_3 = [(-91_isize),(-37_isize),121_isize,9223372036854775807_isize,125_isize];
_2 = [9223372036854775807_isize,85_isize,(-31_isize),9223372036854775807_isize,9223372036854775807_isize];
_4 = [(-9223372036854775808_isize),(-66_isize),93_isize,(-14_isize),9223372036854775807_isize];
RET = (-12_i8) as f64;
_12 = (RET, 108927721000648268723275004287174389903_u128);
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_6 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_11 = _5 ^ _5;
_9 = [(-9223372036854775808_isize),(-125_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_12 = (RET, 285918685274737730480072707741477078724_u128);
_6 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-19_isize),(-9223372036854775808_isize)];
_7 = _9;
_7 = [(-91_isize),9223372036854775807_isize,121_isize,9223372036854775807_isize,(-84_isize)];
_3 = _6;
_4 = [(-9223372036854775808_isize),(-64_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_3 = _8;
match _12.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
285918685274737730480072707741477078724 => bb9,
_ => bb8
}
}
bb3 = {
_4 = _9;
_1 = [(-119_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-123_isize),(-9223372036854775808_isize)];
_4 = _3;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),40_isize,9223372036854775807_isize,9223372036854775807_isize];
_10 = '\u{daf91}';
_5 = !12651856506396754002_u64;
_2 = [(-9223372036854775808_isize),24_isize,(-9223372036854775808_isize),15_isize,(-9223372036854775808_isize)];
_2 = _6;
Call(_2 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
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
_1 = _4;
Call(_6 = fn3(_3, _2, _1, _4, _8, _12.1, _2, _12.1, _7, _3, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_5 = 14478357583628342915_usize as u64;
_6 = _2;
_15.fld3.fld1 = core::ptr::addr_of!(_15.fld2.1);
_15.fld0 = [3_usize,4_usize,15207591101143383828_usize,11649587829144069834_usize,4778168484750241275_usize,11313371922187700684_usize,17884765884811239258_usize];
_13 = [10640190027544698152_usize,4_usize,6_usize,6_usize,7009034431378428355_usize,3_usize,2_usize];
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-81_isize),9223372036854775807_isize];
_11 = _10 as u64;
_15.fld1 = _5;
_15.fld2.3 = RET * RET;
_18 = core::ptr::addr_of!(_15.fld3.fld2);
(*_18) = 14729_i16 - 18138_i16;
_15.fld2.3 = -_12.0;
_15.fld3.fld1 = core::ptr::addr_of!(_15.fld2.1);
_17 = _15.fld2.3;
_21 = [(-1972134831_i32),974624746_i32,330330388_i32,1072676118_i32,582114864_i32];
match _12.1 {
0 => bb1,
1 => bb6,
2 => bb8,
3 => bb9,
4 => bb5,
5 => bb11,
285918685274737730480072707741477078724 => bb13,
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
_2 = _7;
_10 = '\u{56c0b}';
_15.fld2.1 = _12.1 << _15.fld3.fld2;
_12.1 = _17 as u128;
_7 = _6;
RET = _15.fld2.3;
_14 = 11893_u16 as isize;
_19 = !_14;
_15.fld3.fld0 = _1;
(*_18) = (-27790_i16) >> _11;
_15.fld3.fld1 = core::ptr::addr_of!(_15.fld2.1);
_20 = core::ptr::addr_of_mut!(_15.fld3.fld1);
_12.0 = RET;
_15.fld2 = (2343187410389156424_i64, _12.1, 23_i8, RET);
(*_18) = 30096_i16;
_10 = '\u{6bdd4}';
_4 = [_19,_19,_19,_14,_14];
_4 = [_14,_19,_14,_19,_19];
_15.fld2.0 = -3378962839987100145_i64;
_16 = (-6087500013718616478751866001863099540_i128) as usize;
RET = _17 * _17;
_15.fld3.fld2 = (-30119_i16);
match _15.fld2.2 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
23 => bb20,
_ => bb19
}
}
bb14 = {
Return()
}
bb15 = {
_4 = _9;
_1 = [(-119_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-123_isize),(-9223372036854775808_isize)];
_4 = _3;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),40_isize,9223372036854775807_isize,9223372036854775807_isize];
_10 = '\u{daf91}';
_5 = !12651856506396754002_u64;
_2 = [(-9223372036854775808_isize),24_isize,(-9223372036854775808_isize),15_isize,(-9223372036854775808_isize)];
_2 = _6;
Call(_2 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_5 = 14478357583628342915_usize as u64;
_6 = _2;
_15.fld3.fld1 = core::ptr::addr_of!(_15.fld2.1);
_15.fld0 = [3_usize,4_usize,15207591101143383828_usize,11649587829144069834_usize,4778168484750241275_usize,11313371922187700684_usize,17884765884811239258_usize];
_13 = [10640190027544698152_usize,4_usize,6_usize,6_usize,7009034431378428355_usize,3_usize,2_usize];
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-81_isize),9223372036854775807_isize];
_11 = _10 as u64;
_15.fld1 = _5;
_15.fld2.3 = RET * RET;
_18 = core::ptr::addr_of!(_15.fld3.fld2);
(*_18) = 14729_i16 - 18138_i16;
_15.fld2.3 = -_12.0;
_15.fld3.fld1 = core::ptr::addr_of!(_15.fld2.1);
_17 = _15.fld2.3;
_21 = [(-1972134831_i32),974624746_i32,330330388_i32,1072676118_i32,582114864_i32];
match _12.1 {
0 => bb1,
1 => bb6,
2 => bb8,
3 => bb9,
4 => bb5,
5 => bb11,
285918685274737730480072707741477078724 => bb13,
_ => bb12
}
}
bb17 = {
_1 = _4;
Call(_6 = fn3(_3, _2, _1, _4, _8, _12.1, _2, _12.1, _7, _3, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_14 = _19;
_27 = [false,false];
Goto(bb21)
}
bb21 = {
Call(_28 = dump_var(2_usize, 9_usize, Move(_9), 11_usize, Move(_11), 4_usize, Move(_4), 14_usize, Move(_14)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_28 = dump_var(2_usize, 19_usize, Move(_19), 1_usize, Move(_1), 21_usize, Move(_21), 16_usize, Move(_16)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [isize; 5],mut _2: [isize; 5],mut _3: [isize; 5],mut _4: [isize; 5],mut _5: [isize; 5],mut _6: u128,mut _7: [isize; 5],mut _8: u128,mut _9: [isize; 5],mut _10: [isize; 5],mut _11: [isize; 5]) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _12: [u16; 7];
let _13: f32;
let _14: [bool; 2];
let _15: Adt58;
let _16: f64;
let _17: char;
let _18: [usize; 5];
let _19: char;
let _20: (f64, u128);
let _21: (*mut &'static f32, &'static f32, bool);
let _22: (i64, u128, i8, f64);
let _23: Adt59;
let _24: isize;
let _25: i16;
let _26: *mut *const u128;
let _27: [isize; 5];
let _28: f64;
let _29: f64;
let _30: Adt47;
let _31: [isize; 8];
let _32: [bool; 2];
let _33: [i32; 5];
let _34: (u64, i32, u8);
let _35: isize;
let _36: ();
let _37: ();
{
_5 = _1;
RET = _2;
_4 = _9;
_7 = _3;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-15_isize),(-41_isize),(-92_isize)];
_8 = !_6;
_2 = [49_isize,9223372036854775807_isize,126_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [(-110_isize),(-9223372036854775808_isize),92_isize,(-9223372036854775808_isize),(-42_isize)];
_1 = _2;
_8 = _6 + _6;
RET = [101_isize,9223372036854775807_isize,61_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [(-9223372036854775808_isize),(-67_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-79_isize)];
_9 = [58_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_13 = 60890671639072617448585724641678610190_i128 as f32;
RET = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,66_isize];
RET = _10;
_14 = [false,false];
Call(_4 = core::intrinsics::transmute(_11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [9223372036854775807_isize,(-9223372036854775808_isize),8_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_16 = 14_u8 as f64;
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [104_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-55_isize)];
_17 = '\u{22c03}';
_14 = [false,true];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_11 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-18_isize)];
_19 = _17;
_6 = 1377534062_i32 as u128;
_10 = [76_isize,(-9223372036854775808_isize),10_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = _7;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-116_isize)];
_20.1 = !_8;
_21.1 = &_13;
_21.0 = core::ptr::addr_of_mut!(_21.1);
Call(_3 = fn4(_7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_22 = ((-6294987973541003710_i64), _8, (-126_i8), _16);
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,54_isize,(-113_isize)];
_20 = (_22.3, _22.1);
_12 = [971_u16,54809_u16,56880_u16,42546_u16,64946_u16,16018_u16,65281_u16];
_18 = [11560716282198568045_usize,712739926402151904_usize,3279931223621656210_usize,17915513967240951211_usize,16607916285805619174_usize];
_22.3 = -_20.0;
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6 = !_8;
_22.3 = 17673266568190341426_u64 as f64;
_3 = [9223372036854775807_isize,85_isize,(-9223372036854775808_isize),(-108_isize),115_isize];
_4 = _9;
_2 = [9223372036854775807_isize,(-25_isize),9223372036854775807_isize,9223372036854775807_isize,68_isize];
_13 = 115_u8 as f32;
_23.fld4 = 21_u8 as f32;
_11 = [(-9223372036854775808_isize),9223372036854775807_isize,(-106_isize),(-66_isize),(-11_isize)];
_2 = [(-59_isize),9223372036854775807_isize,(-78_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_21.1 = &_13;
_23.fld2.0 = _22.2 as i64;
_23.fld0.3 = !_22.1;
Goto(bb3)
}
bb3 = {
_20.1 = !_8;
_23.fld5 = [5_usize,6_usize,6635090462247057371_usize,0_usize,7_usize,8792663993285557564_usize,3_usize];
_24 = 67_isize;
_1 = [_24,_24,_24,_24,_24];
_29 = -_22.3;
RET = _11;
_23.fld2.1 = _22.1 * _22.1;
_23.fld0.1 = _22.2 as u16;
RET = _3;
_23.fld0 = (_23.fld4, 24954_u16, _24, _20.1);
RET = [_24,_24,_24,_23.fld0.2,_24];
_23.fld5 = [6096405979014931583_usize,3_usize,5_usize,7_usize,0_usize,0_usize,3_usize];
_2 = [_23.fld0.2,_23.fld0.2,_24,_24,_24];
_23.fld0.3 = (-22726_i16) as u128;
_29 = (-2532_i16) as f64;
match _22.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
340282366920938463457079619458227207746 => bb10,
_ => bb9
}
}
bb4 = {
_22 = ((-6294987973541003710_i64), _8, (-126_i8), _16);
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,54_isize,(-113_isize)];
_20 = (_22.3, _22.1);
_12 = [971_u16,54809_u16,56880_u16,42546_u16,64946_u16,16018_u16,65281_u16];
_18 = [11560716282198568045_usize,712739926402151904_usize,3279931223621656210_usize,17915513967240951211_usize,16607916285805619174_usize];
_22.3 = -_20.0;
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6 = !_8;
_22.3 = 17673266568190341426_u64 as f64;
_3 = [9223372036854775807_isize,85_isize,(-9223372036854775808_isize),(-108_isize),115_isize];
_4 = _9;
_2 = [9223372036854775807_isize,(-25_isize),9223372036854775807_isize,9223372036854775807_isize,68_isize];
_13 = 115_u8 as f32;
_23.fld4 = 21_u8 as f32;
_11 = [(-9223372036854775808_isize),9223372036854775807_isize,(-106_isize),(-66_isize),(-11_isize)];
_2 = [(-59_isize),9223372036854775807_isize,(-78_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_21.1 = &_13;
_23.fld2.0 = _22.2 as i64;
_23.fld0.3 = !_22.1;
Goto(bb3)
}
bb5 = {
RET = [9223372036854775807_isize,(-9223372036854775808_isize),8_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_16 = 14_u8 as f64;
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [104_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-55_isize)];
_17 = '\u{22c03}';
_14 = [false,true];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_11 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-18_isize)];
_19 = _17;
_6 = 1377534062_i32 as u128;
_10 = [76_isize,(-9223372036854775808_isize),10_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = _7;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-116_isize)];
_20.1 = !_8;
_21.1 = &_13;
_21.0 = core::ptr::addr_of_mut!(_21.1);
Call(_3 = fn4(_7, _7), ReturnTo(bb2), UnwindUnreachable())
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
_22.2 = 60_i8 ^ (-64_i8);
match _23.fld0.2 {
0 => bb3,
1 => bb5,
2 => bb11,
3 => bb12,
67 => bb14,
_ => bb13
}
}
bb11 = {
_20.1 = !_8;
_23.fld5 = [5_usize,6_usize,6635090462247057371_usize,0_usize,7_usize,8792663993285557564_usize,3_usize];
_24 = 67_isize;
_1 = [_24,_24,_24,_24,_24];
_29 = -_22.3;
RET = _11;
_23.fld2.1 = _22.1 * _22.1;
_23.fld0.1 = _22.2 as u16;
RET = _3;
_23.fld0 = (_23.fld4, 24954_u16, _24, _20.1);
RET = [_24,_24,_24,_23.fld0.2,_24];
_23.fld5 = [6096405979014931583_usize,3_usize,5_usize,7_usize,0_usize,0_usize,3_usize];
_2 = [_23.fld0.2,_23.fld0.2,_24,_24,_24];
_23.fld0.3 = (-22726_i16) as u128;
_29 = (-2532_i16) as f64;
match _22.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
340282366920938463457079619458227207746 => bb10,
_ => bb9
}
}
bb12 = {
Return()
}
bb13 = {
_22 = ((-6294987973541003710_i64), _8, (-126_i8), _16);
_10 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,54_isize,(-113_isize)];
_20 = (_22.3, _22.1);
_12 = [971_u16,54809_u16,56880_u16,42546_u16,64946_u16,16018_u16,65281_u16];
_18 = [11560716282198568045_usize,712739926402151904_usize,3279931223621656210_usize,17915513967240951211_usize,16607916285805619174_usize];
_22.3 = -_20.0;
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6 = !_8;
_22.3 = 17673266568190341426_u64 as f64;
_3 = [9223372036854775807_isize,85_isize,(-9223372036854775808_isize),(-108_isize),115_isize];
_4 = _9;
_2 = [9223372036854775807_isize,(-25_isize),9223372036854775807_isize,9223372036854775807_isize,68_isize];
_13 = 115_u8 as f32;
_23.fld4 = 21_u8 as f32;
_11 = [(-9223372036854775808_isize),9223372036854775807_isize,(-106_isize),(-66_isize),(-11_isize)];
_2 = [(-59_isize),9223372036854775807_isize,(-78_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_21.1 = &_13;
_23.fld2.0 = _22.2 as i64;
_23.fld0.3 = !_22.1;
Goto(bb3)
}
bb14 = {
_11 = [_23.fld0.2,_24,_23.fld0.2,_23.fld0.2,_23.fld0.2];
_7 = [_24,_23.fld0.2,_23.fld0.2,_24,_23.fld0.2];
_5 = _3;
_25 = -31297_i16;
_4 = [_24,_24,_24,_23.fld0.2,_23.fld0.2];
_21.2 = false;
_22.2 = 55_i8;
_19 = _17;
_21.0 = core::ptr::addr_of_mut!(_21.1);
_17 = _19;
_23.fld0 = (_23.fld4, 13305_u16, _24, _6);
_1 = [_23.fld0.2,_24,_23.fld0.2,_24,_23.fld0.2];
_22.2 = 7806110339913871946556901267233345669_i128 as i8;
_11 = [_24,_23.fld0.2,_23.fld0.2,_24,_24];
_23.fld2.3 = _16 * _16;
_29 = _20.0;
_22.0 = _23.fld2.0;
_23.fld0.2 = !_24;
_11 = RET;
_14 = [_21.2,_21.2];
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(3_usize, 14_usize, Move(_14), 17_usize, Move(_17), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(3_usize, 11_usize, Move(_11), 19_usize, Move(_19), 25_usize, Move(_25), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(3_usize, 8_usize, Move(_8), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [isize; 5],mut _2: [isize; 5]) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _3: Adt56;
let _4: bool;
let _5: Adt56;
let _6: (u16, i16);
let _7: i32;
let _8: (f32, u16, isize, u128);
let _9: Adt46;
let _10: Adt47;
let _11: (f32, bool, u64, i32);
let _12: Adt46;
let _13: isize;
let _14: Adt54;
let _15: bool;
let _16: f64;
let _17: u16;
let _18: isize;
let _19: Adt54;
let _20: usize;
let _21: isize;
let _22: [u16; 7];
let _23: bool;
let _24: ();
let _25: ();
{
_1 = _2;
RET = [42_isize,40_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),97_isize,9223372036854775807_isize];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),82_isize,(-24_isize)];
RET = [39_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-12_isize),9223372036854775807_isize,59_isize];
RET = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,16_isize,9223372036854775807_isize];
_4 = !true;
_2 = RET;
_2 = [113_isize,(-9223372036854775808_isize),97_isize,(-119_isize),(-12_isize)];
_4 = false;
_4 = !false;
Goto(bb1)
}
bb1 = {
_6.1 = 225661164_u32 as i16;
_1 = RET;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,89_isize,9223372036854775807_isize,9223372036854775807_isize];
_6.0 = 26914_u16 | 26350_u16;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = -(-1493429745_i32);
_6 = (52907_u16, (-24170_i16));
_8.0 = 3_u8 as f32;
RET = _1;
RET = _2;
_8.1 = !_6.0;
_8.1 = !_6.0;
_11.1 = _6.0 >= _6.0;
_8.0 = 4449354938778575716_i64 as f32;
_11.2 = !18386170047226940881_u64;
_6.1 = -(-20137_i16);
RET = _1;
_11.0 = _6.1 as f32;
_1 = [(-58_isize),94_isize,9223372036854775807_isize,44_isize,9223372036854775807_isize];
_6 = (_8.1, 334_i16);
_6 = (_8.1, (-4180_i16));
_11.0 = _8.0 - _8.0;
_13 = !(-9223372036854775808_isize);
_6 = (_8.1, 24292_i16);
_6.0 = _13 as u16;
_11 = (_8.0, _4, 1153257069871216085_u64, _7);
match _11.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1153257069871216085 => bb10,
_ => bb9
}
}
bb3 = {
_6.1 = 225661164_u32 as i16;
_1 = RET;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,89_isize,9223372036854775807_isize,9223372036854775807_isize];
_6.0 = 26914_u16 | 26350_u16;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
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
_8 = (_11.0, _6.0, _13, 181014277025243967953618550387864416273_u128);
_17 = _6.0;
_8.0 = _11.0;
_11.0 = _8.0 * _8.0;
_11.1 = !_4;
_17 = !_6.0;
_15 = _13 == _8.2;
_16 = _8.3 as f64;
_2 = [_13,_13,_8.2,_13,_13];
_11.1 = !_15;
_6.0 = _17 & _17;
_8.0 = 182_u8 as f32;
_11 = (_8.0, _4, 6460178298280465685_u64, _7);
_20 = _4 as usize;
Goto(bb11)
}
bb11 = {
_18 = _8.2 * _8.2;
_8.3 = _11.0 as u128;
_6.0 = _11.0 as u16;
_8.2 = _11.2 as isize;
RET = [_8.2,_18,_8.2,_8.2,_13];
_11 = (_8.0, _4, 7638696498436804727_u64, _7);
_8 = (_11.0, _6.0, _18, 86252114571365381717441113245377324462_u128);
_8.3 = !279937184846531466092247044523748937975_u128;
_11.3 = _7 | _7;
Goto(bb12)
}
bb12 = {
RET = _1;
_11.0 = _8.0 + _8.0;
_17 = _6.0;
_7 = _8.1 as i32;
_8.1 = _17;
_11.1 = _4;
_21 = _18 & _8.2;
Call(_11.2 = fn5(_8, _6.1, _1, _21, _21, _18, _11.3, RET, _6.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4 = _11.1 | _11.1;
_8.2 = 148_u8 as isize;
match _6.1 {
0 => bb14,
1 => bb15,
2 => bb16,
24292 => bb18,
_ => bb17
}
}
bb14 = {
RET = _1;
_11.0 = _8.0 + _8.0;
_17 = _6.0;
_7 = _8.1 as i32;
_8.1 = _17;
_11.1 = _4;
_21 = _18 & _8.2;
Call(_11.2 = fn5(_8, _6.1, _1, _21, _21, _18, _11.3, RET, _6.1), ReturnTo(bb13), UnwindUnreachable())
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
_17 = !_8.1;
_11.1 = _8.2 == _21;
_1 = [_18,_8.2,_21,_18,_21];
_11.1 = _18 < _21;
_1 = RET;
_6.1 = 27440_i16;
_8.3 = 204280662387412272594375379706818850862_u128 << _11.3;
_11.1 = _15;
_4 = !_11.1;
_21 = _8.2;
_22 = [_17,_17,_6.0,_17,_17,_17,_17];
_1 = [_18,_18,_21,_13,_21];
_11.0 = _8.0;
_6.0 = _8.1 >> _11.3;
_8.1 = 8242034656977720675_i64 as u16;
_15 = _4;
_5 = Adt56::Variant3 { fld0: _8,fld1: 39331477078823407004160283676073410299_i128,fld2: _11.2 };
_6.0 = !_8.1;
place!(Field::<(f32, u16, isize, u128)>(Variant(_5, 3), 0)).2 = _20 as isize;
place!(Field::<(f32, u16, isize, u128)>(Variant(_5, 3), 0)).1 = _17 * _6.0;
_8.2 = 16_i8 as isize;
Goto(bb19)
}
bb19 = {
Call(_24 = dump_var(4_usize, 18_usize, Move(_18), 1_usize, Move(_1), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_24 = dump_var(4_usize, 2_usize, Move(_2), 20_usize, Move(_20), 25_usize, _25, 25_usize, _25), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (f32, u16, isize, u128),mut _2: i16,mut _3: [isize; 5],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: i32,mut _8: [isize; 5],mut _9: i16) -> u64 {
mir! {
type RET = u64;
let _10: (f32, u16, isize, u128);
let _11: char;
let _12: Adt51;
let _13: [usize; 7];
let _14: f32;
let _15: Adt46;
let _16: usize;
let _17: (i128, u128, &'static f32, i32);
let _18: *mut *const u128;
let _19: isize;
let _20: u16;
let _21: (*mut &'static f32, &'static f32, bool);
let _22: Adt48;
let _23: i64;
let _24: bool;
let _25: Adt57;
let _26: isize;
let _27: isize;
let _28: (f32, bool, u64, i32);
let _29: f64;
let _30: (*mut &'static f32, &'static f32, bool);
let _31: char;
let _32: Adt53;
let _33: ();
let _34: ();
{
RET = 9136446156462179414_u64;
_7 = '\u{abc9d}' as i32;
_3 = [_4,_1.2,_5,_5,_5];
_1.3 = 3843907630272167352315938069411952160_u128;
_1.1 = 33453_u16;
_9 = -_2;
_7 = _1.3 as i32;
_11 = '\u{2683e}';
_10 = (_1.0, _1.1, _5, _1.3);
_9 = _2 >> _4;
_3 = _8;
_5 = !_4;
_11 = '\u{ee2c3}';
_6 = -_4;
RET = _10.0 as u64;
_10.1 = !_1.1;
_11 = '\u{41085}';
_9 = _2 + _2;
Goto(bb1)
}
bb1 = {
_4 = _10.1 as isize;
_14 = _10.0 + _10.0;
_10.0 = 17713724423547344412_usize as f32;
_13 = [4_usize,6_usize,6_usize,6_usize,0_usize,6_usize,6_usize];
_10.3 = _1.3 ^ _1.3;
_13 = [7859608176841987177_usize,2_usize,2_usize,10343762990167188467_usize,12697647762064782400_usize,4_usize,7741304411741761478_usize];
_2 = !_9;
_10.3 = _1.3;
_17.0 = 99_i8 as i128;
_14 = _1.2 as f32;
_17.1 = _10.3;
_17.2 = &_14;
_17.0 = -(-58111502299909362924480524068101897625_i128);
_10.2 = _11 as isize;
_4 = _1.2;
_1.1 = !_10.1;
_7 = 285536251_i32 & (-1325749124_i32);
_10 = (_14, _1.1, _6, _17.1);
_10.1 = !_1.1;
_1.2 = _6;
_6 = _17.0 as isize;
_19 = 0_usize as isize;
Goto(bb2)
}
bb2 = {
_4 = -_1.2;
_17.3 = !_7;
_1.0 = _17.0 as f32;
_1.1 = _10.1;
_6 = _1.2;
_10 = (_1.0, _1.1, _4, _1.3);
_17.2 = &_10.0;
_5 = _2 as isize;
_16 = 7_usize;
_1.2 = _10.2 + _5;
_6 = -_5;
_5 = _1.2;
Call(_14 = fn6(_13, _10, _8, _5, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10.1 = _1.1 << _5;
_10 = (_1.0, _1.1, _5, _1.3);
_21.0 = core::ptr::addr_of_mut!(_21.1);
_21.1 = &_1.0;
_1.0 = -_10.0;
_2 = -_9;
Call(_10.0 = fn7(_13, _1.2, _5, _9, _6, _4, _1, _1, _3, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10.1 = 95_i8 as u16;
_6 = _4 ^ _5;
_21.1 = &_1.0;
_13 = [_16,_16,_16,_16,_16,_16,_16];
_19 = _11 as isize;
_21.2 = _10.2 != _19;
_1.1 = _10.1;
_10 = (_14, _1.1, _6, _17.1);
_21.1 = &_10.0;
_17.3 = _7 & _7;
_16 = !6_usize;
_9 = _10.1 as i16;
_21.1 = &_1.0;
_24 = _21.2 ^ _21.2;
match _1.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
3843907630272167352315938069411952160 => bb10,
_ => bb9
}
}
bb5 = {
_10.1 = _1.1 << _5;
_10 = (_1.0, _1.1, _5, _1.3);
_21.0 = core::ptr::addr_of_mut!(_21.1);
_21.1 = &_1.0;
_1.0 = -_10.0;
_2 = -_9;
Call(_10.0 = fn7(_13, _1.2, _5, _9, _6, _4, _1, _1, _3, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_4 = -_1.2;
_17.3 = !_7;
_1.0 = _17.0 as f32;
_1.1 = _10.1;
_6 = _1.2;
_10 = (_1.0, _1.1, _4, _1.3);
_17.2 = &_10.0;
_5 = _2 as isize;
_16 = 7_usize;
_1.2 = _10.2 + _5;
_6 = -_5;
_5 = _1.2;
Call(_14 = fn6(_13, _10, _8, _5, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4 = _10.1 as isize;
_14 = _10.0 + _10.0;
_10.0 = 17713724423547344412_usize as f32;
_13 = [4_usize,6_usize,6_usize,6_usize,0_usize,6_usize,6_usize];
_10.3 = _1.3 ^ _1.3;
_13 = [7859608176841987177_usize,2_usize,2_usize,10343762990167188467_usize,12697647762064782400_usize,4_usize,7741304411741761478_usize];
_2 = !_9;
_10.3 = _1.3;
_17.0 = 99_i8 as i128;
_14 = _1.2 as f32;
_17.1 = _10.3;
_17.2 = &_14;
_17.0 = -(-58111502299909362924480524068101897625_i128);
_10.2 = _11 as isize;
_4 = _1.2;
_1.1 = !_10.1;
_7 = 285536251_i32 & (-1325749124_i32);
_10 = (_14, _1.1, _6, _17.1);
_10.1 = !_1.1;
_1.2 = _6;
_6 = _17.0 as isize;
_19 = 0_usize as isize;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_10.0 = _14 - _1.0;
Goto(bb11)
}
bb11 = {
RET = 10891657689821711333_u64 & 11040336286761802899_u64;
_21.0 = core::ptr::addr_of_mut!(_17.2);
_1.2 = _10.2 ^ _10.2;
_17.3 = -_7;
_10.1 = _1.1 + _1.1;
_1.0 = _16 as f32;
_23 = (-4467562400725298701_i64);
_10.0 = _14;
_10.1 = !_1.1;
_1.2 = _10.2 + _5;
RET = 5427133364667418450_u64 - 17449539349194866142_u64;
_19 = !_6;
_8 = [_1.2,_6,_5,_1.2,_6];
_17.2 = &_10.0;
_10.1 = _21.2 as u16;
_5 = _7 as isize;
_1.3 = _17.1;
_24 = _1.2 >= _1.2;
_11 = '\u{ae9ce}';
match _23 {
0 => bb3,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
340282366920938463458907045031042912755 => bb18,
_ => bb17
}
}
bb12 = {
_10.1 = 95_i8 as u16;
_6 = _4 ^ _5;
_21.1 = &_1.0;
_13 = [_16,_16,_16,_16,_16,_16,_16];
_19 = _11 as isize;
_21.2 = _10.2 != _19;
_1.1 = _10.1;
_10 = (_14, _1.1, _6, _17.1);
_21.1 = &_10.0;
_17.3 = _7 & _7;
_16 = !6_usize;
_9 = _10.1 as i16;
_21.1 = &_1.0;
_24 = _21.2 ^ _21.2;
match _1.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
3843907630272167352315938069411952160 => bb10,
_ => bb9
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_4 = _10.1 as isize;
_14 = _10.0 + _10.0;
_10.0 = 17713724423547344412_usize as f32;
_13 = [4_usize,6_usize,6_usize,6_usize,0_usize,6_usize,6_usize];
_10.3 = _1.3 ^ _1.3;
_13 = [7859608176841987177_usize,2_usize,2_usize,10343762990167188467_usize,12697647762064782400_usize,4_usize,7741304411741761478_usize];
_2 = !_9;
_10.3 = _1.3;
_17.0 = 99_i8 as i128;
_14 = _1.2 as f32;
_17.1 = _10.3;
_17.2 = &_14;
_17.0 = -(-58111502299909362924480524068101897625_i128);
_10.2 = _11 as isize;
_4 = _1.2;
_1.1 = !_10.1;
_7 = 285536251_i32 & (-1325749124_i32);
_10 = (_14, _1.1, _6, _17.1);
_10.1 = !_1.1;
_1.2 = _6;
_6 = _17.0 as isize;
_19 = 0_usize as isize;
Goto(bb2)
}
bb16 = {
_4 = -_1.2;
_17.3 = !_7;
_1.0 = _17.0 as f32;
_1.1 = _10.1;
_6 = _1.2;
_10 = (_1.0, _1.1, _4, _1.3);
_17.2 = &_10.0;
_5 = _2 as isize;
_16 = 7_usize;
_1.2 = _10.2 + _5;
_6 = -_5;
_5 = _1.2;
Call(_14 = fn6(_13, _10, _8, _5, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_10.1 = _1.1 << _5;
_10 = (_1.0, _1.1, _5, _1.3);
_21.0 = core::ptr::addr_of_mut!(_21.1);
_21.1 = &_1.0;
_1.0 = -_10.0;
_2 = -_9;
Call(_10.0 = fn7(_13, _1.2, _5, _9, _6, _4, _1, _1, _3, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb18 = {
_23 = 4837093833217564492_i64 ^ (-2115619503250854383_i64);
_17.1 = _1.3 & _1.3;
_17.0 = _11 as i128;
_11 = '\u{180dc}';
_28.3 = !_17.3;
_28 = (_1.0, _24, RET, _17.3);
_28.0 = _10.0 + _10.0;
_4 = _23 as isize;
_19 = _1.2;
_16 = 4597715207319519107_usize << _10.2;
_1.1 = _10.1 ^ _10.1;
_25 = Adt57::Variant2 { fld0: _1.0,fld1: _11,fld2: (-6_i8) };
_21.1 = &_1.0;
_10.0 = -_28.0;
place!(Field::<i8>(Variant(_25, 2), 2)) = _6 as i8;
_30 = (Move(_21.0), Move(_21.1), _28.1);
_28 = (_10.0, _24, RET, _7);
_30.2 = !_24;
_21.1 = &_10.0;
_28.2 = _19 as u64;
_6 = _1.1 as isize;
_27 = !_1.2;
_17.2 = Move(_21.1);
_7 = _28.0 as i32;
_31 = _11;
Goto(bb19)
}
bb19 = {
Call(_33 = dump_var(5_usize, 13_usize, Move(_13), 31_usize, Move(_31), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(5_usize, 2_usize, Move(_2), 19_usize, Move(_19), 7_usize, Move(_7), 23_usize, Move(_23)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [usize; 7],mut _2: (f32, u16, isize, u128),mut _3: [isize; 5],mut _4: isize,mut _5: [isize; 5],mut _6: isize) -> f32 {
mir! {
type RET = f32;
let _7: char;
let _8: [isize; 8];
let _9: u32;
let _10: f32;
let _11: char;
let _12: u128;
let _13: Adt56;
let _14: [u16; 7];
let _15: Adt43;
let _16: isize;
let _17: i64;
let _18: usize;
let _19: [bool; 2];
let _20: u128;
let _21: u64;
let _22: Adt53;
let _23: bool;
let _24: bool;
let _25: ();
let _26: ();
{
_1 = [0_usize,1378713968235458567_usize,0_usize,17549335494530549413_usize,6_usize,15788351158054248209_usize,15906270854894834189_usize];
_5 = [_6,_4,_6,_4,_6];
_4 = _6;
RET = _2.0;
RET = _2.0;
_3 = [_6,_4,_6,_6,_4];
match _2.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
3843907630272167352315938069411952160 => bb6,
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
_2 = (RET, 48339_u16, _4, 164746134250648500337425699065338266578_u128);
_2 = (RET, 27972_u16, _4, 164723153361711872553795400763796516676_u128);
_1 = [465044857970630516_usize,11700661733891231478_usize,1_usize,1_usize,4_usize,852362413833350057_usize,12047231951290861364_usize];
_2.0 = RET - RET;
_2.0 = RET;
_2.3 = 336940554736069408496074395464895101744_u128 ^ 319178997739619928972236394948138072733_u128;
Goto(bb7)
}
bb7 = {
_5 = _3;
_2.0 = _2.3 as f32;
_2 = (RET, 38274_u16, _4, 202389117692561208987482496294570334874_u128);
_1 = [5096422149982688509_usize,6_usize,3851912308621118471_usize,6_usize,3_usize,2465688311261977654_usize,8239736638605790897_usize];
_6 = _2.2 >> _2.3;
_2.2 = _4 + _6;
_2.3 = 181742776883974730049980042906435646189_u128 | 115681959126575164555151084910528621427_u128;
_8 = [_6,_6,_6,_6,_6,_2.2,_2.2,_2.2];
_7 = '\u{b8e7d}';
_2.3 = 331510545901604516627777408933233337733_u128;
_2.1 = !15457_u16;
_3 = [_4,_2.2,_2.2,_6,_4];
_2.1 = _6 as u16;
_5 = [_2.2,_2.2,_2.2,_4,_6];
_2 = (RET, 26092_u16, _4, 237715552171014583349024456087598900976_u128);
_1 = [3188372511774870958_usize,1_usize,14617837319534200011_usize,3_usize,13814341165572499375_usize,1_usize,5_usize];
RET = -_2.0;
_9 = !127294095_u32;
_9 = !3607759326_u32;
_12 = _2.3;
_5 = [_2.2,_2.2,_6,_4,_4];
match _12 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb8,
4 => bb9,
5 => bb10,
237715552171014583349024456087598900976 => bb12,
_ => bb11
}
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
_1 = [5_usize,3189203341843246321_usize,18185595959893177086_usize,3156181002035752804_usize,3_usize,9294562962863824706_usize,12796764643562644585_usize];
_10 = -RET;
_9 = (-73_i8) as u32;
_2.0 = _10 - _10;
_14 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
_13 = Adt56::Variant3 { fld0: _2,fld1: (-104566526440791512183551560608844620061_i128),fld2: 11841375752662979526_u64 };
place!(Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0)).3 = !_2.3;
_6 = !_4;
place!(Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0)).3 = !_12;
_8 = [_4,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2,_4,_4,_4,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2];
_4 = _2.2 & _6;
RET = -_2.0;
_2.1 = Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).1;
place!(Field::<i128>(Variant(_13, 3), 1)) = (-760459813196323071_i64) as i128;
_10 = RET;
place!(Field::<u64>(Variant(_13, 3), 2)) = 8674756115548249067_u64 & 3277556187139131600_u64;
place!(Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0)).0 = -RET;
_12 = !Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).3;
_19 = [false,true];
_18 = 15212169793652704029_usize;
_17 = (-9141884447429575029_i64) - 1018535301412201750_i64;
_7 = '\u{2bfe2}';
_6 = _18 as isize;
_17 = (-3677543143210898156_i64) << Field::<u64>(Variant(_13, 3), 2);
_8 = [_4,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2,_2.2,_4,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2,_2.2,Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).2,_4];
_17 = _2.3 as i64;
match _2.1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
26092 => bb19,
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
_5 = _3;
_2.0 = _2.3 as f32;
_2 = (RET, 38274_u16, _4, 202389117692561208987482496294570334874_u128);
_1 = [5096422149982688509_usize,6_usize,3851912308621118471_usize,6_usize,3_usize,2465688311261977654_usize,8239736638605790897_usize];
_6 = _2.2 >> _2.3;
_2.2 = _4 + _6;
_2.3 = 181742776883974730049980042906435646189_u128 | 115681959126575164555151084910528621427_u128;
_8 = [_6,_6,_6,_6,_6,_2.2,_2.2,_2.2];
_7 = '\u{b8e7d}';
_2.3 = 331510545901604516627777408933233337733_u128;
_2.1 = !15457_u16;
_3 = [_4,_2.2,_2.2,_6,_4];
_2.1 = _6 as u16;
_5 = [_2.2,_2.2,_2.2,_4,_6];
_2 = (RET, 26092_u16, _4, 237715552171014583349024456087598900976_u128);
_1 = [3188372511774870958_usize,1_usize,14617837319534200011_usize,3_usize,13814341165572499375_usize,1_usize,5_usize];
RET = -_2.0;
_9 = !127294095_u32;
_9 = !3607759326_u32;
_12 = _2.3;
_5 = [_2.2,_2.2,_6,_4,_4];
match _12 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb8,
4 => bb9,
5 => bb10,
237715552171014583349024456087598900976 => bb12,
_ => bb11
}
}
bb18 = {
_2 = (RET, 48339_u16, _4, 164746134250648500337425699065338266578_u128);
_2 = (RET, 27972_u16, _4, 164723153361711872553795400763796516676_u128);
_1 = [465044857970630516_usize,11700661733891231478_usize,1_usize,1_usize,4_usize,852362413833350057_usize,12047231951290861364_usize];
_2.0 = RET - RET;
_2.0 = RET;
_2.3 = 336940554736069408496074395464895101744_u128 ^ 319178997739619928972236394948138072733_u128;
Goto(bb7)
}
bb19 = {
_12 = !Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).3;
_17 = 8544347647358146180_i64;
_22.fld2.0 = RET;
_7 = '\u{98609}';
_20 = _2.3 & _2.3;
place!(Field::<u64>(Variant(_13, 3), 2)) = 2209454019135220694_u64;
_22.fld2.1 = Field::<u64>(Variant(_13, 3), 2) as u16;
_2.1 = Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).1;
_22.fld1 = [420222899_i32,(-1606437272_i32),(-1021967215_i32),(-914228132_i32),(-453710404_i32)];
_2.2 = _4 << _2.1;
_9 = 3242908552_u32;
RET = _17 as f32;
place!(Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0)).2 = _2.2 | _4;
_19 = [false,true];
_11 = _7;
_2.3 = (-683939842_i32) as u128;
_18 = 750066370561670797_usize ^ 4_usize;
_22.fld2.0 = Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0).0 - RET;
_18 = _17 as usize;
_21 = !Field::<u64>(Variant(_13, 3), 2);
place!(Field::<(f32, u16, isize, u128)>(Variant(_13, 3), 0)).2 = !_2.2;
_23 = true | false;
Goto(bb20)
}
bb20 = {
Call(_25 = dump_var(6_usize, 8_usize, Move(_8), 20_usize, Move(_20), 3_usize, Move(_3), 23_usize, Move(_23)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_25 = dump_var(6_usize, 9_usize, Move(_9), 11_usize, Move(_11), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [usize; 7],mut _2: isize,mut _3: isize,mut _4: i16,mut _5: isize,mut _6: isize,mut _7: (f32, u16, isize, u128),mut _8: (f32, u16, isize, u128),mut _9: [isize; 5],mut _10: f32) -> f32 {
mir! {
type RET = f32;
let _11: Adt58;
let _12: f64;
let _13: i8;
let _14: i128;
let _15: Adt45;
let _16: *mut &'static f32;
let _17: (u64, i32, u8);
let _18: [isize; 8];
let _19: isize;
let _20: isize;
let _21: i32;
let _22: f32;
let _23: f64;
let _24: Adt51;
let _25: bool;
let _26: ();
let _27: ();
{
_7.1 = !_8.1;
_8.0 = _10 - _10;
_4 = -(-7579_i16);
_8 = (_10, _7.1, _7.2, _7.3);
_7.3 = 15228280280890391026_u64 as u128;
_7.0 = -_10;
RET = _10 * _8.0;
_9 = [_7.2,_5,_8.2,_7.2,_8.2];
_7.1 = _8.1 >> _8.2;
_12 = 515700233975786198_i64 as f64;
_10 = _8.0;
_2 = -_5;
_5 = 4329019740074197958_i64 as isize;
_8 = (_10, _7.1, _7.2, _7.3);
_8.2 = _6 + _7.2;
_2 = -_7.2;
_12 = 16_u8 as f64;
_4 = (-32577_i16) * 12798_i16;
_8.1 = _7.1;
_1 = [3702539977664776012_usize,0_usize,0_usize,3279030026533972853_usize,12117971613123185889_usize,11232074379895505883_usize,6_usize];
_3 = 1606999580_i32 as isize;
_8.2 = !_7.2;
_8.0 = -_7.0;
RET = -_7.0;
_7 = _8;
_5 = _7.2 + _6;
_12 = (-20700950879654677647091097588582844317_i128) as f64;
_3 = _7.2 | _7.2;
_7 = (_8.0, _8.1, _6, _8.3);
_7.2 = _3 << _8.2;
Goto(bb1)
}
bb1 = {
_8 = _7;
_8.2 = _7.2;
_2 = _5 ^ _8.2;
_8.3 = _7.3 >> _5;
_12 = _8.3 as f64;
_12 = (-1480753460_i32) as f64;
RET = -_8.0;
_2 = _7.2 + _7.2;
RET = _10 - _8.0;
RET = _7.0 + _8.0;
Call(_9 = fn8(_8.2, _7, _8.2, _2, _2, _7.1, _8.2, _3, _7.2, _8.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _8.0 - _7.0;
_7.3 = _8.3;
_8.2 = !_7.2;
_1 = [1_usize,2_usize,18295174563694447913_usize,5_usize,6_usize,2_usize,5_usize];
_7.1 = _8.1;
_2 = _7.2 - _3;
_10 = _7.3 as f32;
_7.2 = 218_u8 as isize;
_7.2 = _2;
_8.3 = '\u{58689}' as u128;
_17 = (3913995751533044209_u64, (-1999019011_i32), 233_u8);
_17.0 = 13134083988106647875_u64;
_13 = !122_i8;
_7 = (_10, _8.1, _3, _8.3);
_7.2 = !_5;
_6 = _7.2 >> _2;
_8.2 = !_2;
_15.fld1 = core::ptr::addr_of!(_8.3);
_17 = (2804855576419937839_u64, (-1250240572_i32), 252_u8);
_13 = !(-103_i8);
_10 = -_7.0;
_14 = (-67280398626047410241643779099408256502_i128);
_15.fld1 = core::ptr::addr_of!(_8.3);
_14 = (-150669517163992135280591018566636315148_i128) - 51292481743403914142321984612569242646_i128;
_17 = (10533354898257081416_u64, 1285467204_i32, 72_u8);
_8.2 = !_7.2;
_3 = _6;
Goto(bb3)
}
bb3 = {
_7.1 = _8.1 + _8.1;
_15.fld1 = core::ptr::addr_of!(_8.3);
_17.2 = _17.1 as u8;
_19 = _3 >> _2;
_9 = [_19,_19,_19,_6,_19];
_1 = [2_usize,17400395489927958581_usize,14069549226458466011_usize,1_usize,1_usize,0_usize,3_usize];
_6 = -_3;
_21 = _17.1;
_17.1 = _21 | _21;
_20 = _3 ^ _7.2;
_9 = [_7.2,_6,_19,_6,_7.2];
_23 = _12 + _12;
match _21 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
1285467204 => bb10,
_ => bb9
}
}
bb4 = {
RET = _8.0 - _7.0;
_7.3 = _8.3;
_8.2 = !_7.2;
_1 = [1_usize,2_usize,18295174563694447913_usize,5_usize,6_usize,2_usize,5_usize];
_7.1 = _8.1;
_2 = _7.2 - _3;
_10 = _7.3 as f32;
_7.2 = 218_u8 as isize;
_7.2 = _2;
_8.3 = '\u{58689}' as u128;
_17 = (3913995751533044209_u64, (-1999019011_i32), 233_u8);
_17.0 = 13134083988106647875_u64;
_13 = !122_i8;
_7 = (_10, _8.1, _3, _8.3);
_7.2 = !_5;
_6 = _7.2 >> _2;
_8.2 = !_2;
_15.fld1 = core::ptr::addr_of!(_8.3);
_17 = (2804855576419937839_u64, (-1250240572_i32), 252_u8);
_13 = !(-103_i8);
_10 = -_7.0;
_14 = (-67280398626047410241643779099408256502_i128);
_15.fld1 = core::ptr::addr_of!(_8.3);
_14 = (-150669517163992135280591018566636315148_i128) - 51292481743403914142321984612569242646_i128;
_17 = (10533354898257081416_u64, 1285467204_i32, 72_u8);
_8.2 = !_7.2;
_3 = _6;
Goto(bb3)
}
bb5 = {
_8 = _7;
_8.2 = _7.2;
_2 = _5 ^ _8.2;
_8.3 = _7.3 >> _5;
_12 = _8.3 as f64;
_12 = (-1480753460_i32) as f64;
RET = -_8.0;
_2 = _7.2 + _7.2;
RET = _10 - _8.0;
RET = _7.0 + _8.0;
Call(_9 = fn8(_8.2, _7, _8.2, _2, _2, _7.1, _8.2, _3, _7.2, _8.2), ReturnTo(bb2), UnwindUnreachable())
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
_3 = !_19;
_13 = _17.0 as i8;
_20 = _3 - _3;
_7 = (_10, _8.1, _6, _8.3);
RET = _7.0 * _10;
_8 = _7;
_18 = [_3,_6,_2,_8.2,_20,_20,_19,_8.2];
_1 = [17302445494017689344_usize,0_usize,907210705395088781_usize,4_usize,6_usize,1_usize,6_usize];
Goto(bb11)
}
bb11 = {
Call(_26 = dump_var(7_usize, 18_usize, Move(_18), 9_usize, Move(_9), 2_usize, Move(_2), 19_usize, Move(_19)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_26 = dump_var(7_usize, 3_usize, Move(_3), 5_usize, Move(_5), 6_usize, Move(_6), 27_usize, _27), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: (f32, u16, isize, u128),mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u16,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _11: u128;
let _12: ();
let _13: ();
{
RET = [_5,_5,_2.2,_5,_9];
_9 = !_7;
_8 = !_5;
_11 = !_2.3;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(8_usize, 10_usize, Move(_10), 7_usize, Move(_7), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_12 = dump_var(8_usize, 8_usize, Move(_8), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{aed66}'), std::hint::black_box(61561164004446610947653239989719063398_u128), std::hint::black_box(88776426979200163_u64), std::hint::black_box(3_usize), std::hint::black_box(391474769_i32), std::hint::black_box(777536505_u32), std::hint::black_box(81888711505236820614919439427569669810_i128));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: usize,
fld1: *const f64,
fld2: isize,
fld3: f32,
fld4: i16,
fld5: u16,
fld6: u8,
fld7: [u16; 7],

},
Variant1{
fld0: u128,
fld1: [u64; 5],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
fld0: [usize; 5],
fld1: Adt43,
fld2: u128,
fld3: [u16; 7],
fld4: *mut u32,
fld5: u8,

},
Variant1{
fld0: (u64, i32, u8),
fld1: [usize; 5],
fld2: [isize; 8],
fld3: i8,
fld4: usize,
fld5: [u64; 5],
fld6: Adt43,
fld7: [u16; 7],

},
Variant2{
fld0: (f32, bool, u64, i32),
fld1: *mut i16,
fld2: u8,

},
Variant3{
fld0: [usize; 7],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: [isize; 5],
fld1: *const u128,
fld2: i16,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: i16,
fld1: [u32; 7],
fld2: [usize; 7],
fld3: (u64, i32, u8),

},
Variant1{
fld0: (f32, bool, u64, i32),
fld1: *const u128,
fld2: [u32; 7],
fld3: Adt45,
fld4: [usize; 5],

},
Variant2{
fld0: (f32, u16, isize, u128),
fld1: [u16; 7],
fld2: (i64, u128, i8, f64),
fld3: i8,

},
Variant3{
fld0: f64,
fld1: (u64, i32, u8),
fld2: u16,
fld3: (f64, u128),
fld4: i128,
fld5: (f32, bool, u64, i32),

}}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [i32; 5],
fld1: u64,
fld2: Adt44,
fld3: Adt46,
fld4: *mut u32,
fld5: i128,

},
Variant1{
fld0: (f32, bool, u64, i32),
fld1: u64,
fld2: u8,
fld3: (f64, u128),
fld4: u32,

},
Variant2{
fld0: bool,
fld1: char,
fld2: isize,
fld3: i8,
fld4: u64,
fld5: [bool; 2],
fld6: *mut *const u128,
fld7: Adt45,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: bool,
fld1: i128,
fld2: u64,
fld3: (f32, u16, isize, u128),
fld4: u16,

},
Variant1{
fld0: u64,
fld1: isize,

},
Variant2{
fld0: *const u128,
fld1: f32,
fld2: [i32; 5],
fld3: i8,
fld4: [u64; 5],
fld5: Adt47,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [i32; 5],

},
Variant1{
fld0: Adt44,
fld1: Adt43,
fld2: Adt47,
fld3: i8,
fld4: i64,

},
Variant2{
fld0: f32,
fld1: *mut i16,

},
Variant3{
fld0: usize,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [usize; 7],
fld1: u64,
fld2: (i64, u128, i8, f64),
fld3: Adt45,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: f32,
fld1: [isize; 5],
fld2: *const f64,

},
Variant1{
fld0: Adt45,
fld1: Adt46,
fld2: isize,
fld3: (f32, u16, isize, u128),
fld4: f32,
fld5: Adt47,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (f64, u128),
fld1: [u8; 1],
fld2: [u64; 5],

},
Variant1{
fld0: u16,
fld1: u128,
fld2: Adt44,

},
Variant2{
fld0: [i32; 5],
fld1: (f64, u128),
fld2: *mut i16,
fld3: u32,
fld4: (f32, u16, isize, u128),
fld5: i32,
fld6: [isize; 8],
fld7: *const i16,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: *mut *const u128,
fld1: [i32; 5],
fld2: (f32, u16, isize, u128),
fld3: Adt51,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u64; 5],
fld1: i128,

},
Variant1{
fld0: Adt43,
fld1: Adt44,
fld2: *mut *const u128,
fld3: Adt52,
fld4: i16,
fld5: *const i16,

},
Variant2{
fld0: (f64, u128),
fld1: f64,
fld2: Adt48,

},
Variant3{
fld0: (u64, i32, u8),
fld1: [usize; 5],
fld2: i8,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [isize; 5],

},
Variant1{
fld0: [usize; 5],
fld1: [u8; 1],
fld2: Adt50,
fld3: *const u128,
fld4: u128,
fld5: Adt46,
fld6: Adt48,

},
Variant2{
fld0: Adt49,
fld1: char,
fld2: i128,
fld3: i8,
fld4: i32,

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
fld0: [u8; 1],

},
Variant1{
fld0: bool,
fld1: Adt47,
fld2: u32,
fld3: [usize; 7],
fld4: i16,
fld5: *const u128,

},
Variant2{
fld0: u128,
fld1: *mut i16,
fld2: [isize; 8],
fld3: Adt54,
fld4: u32,
fld5: [bool; 2],
fld6: i64,
fld7: i128,

},
Variant3{
fld0: (f32, u16, isize, u128),
fld1: i128,
fld2: u64,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
fld0: Adt53,

},
Variant1{
fld0: [usize; 7],
fld1: usize,

},
Variant2{
fld0: f32,
fld1: char,
fld2: i8,

},
Variant3{
fld0: Adt52,
fld1: *mut u32,
fld2: Adt54,
fld3: i8,
fld4: Adt44,
fld5: u64,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [usize; 7],
fld1: Adt54,
fld2: Adt48,
fld3: Adt47,

},
Variant1{
fld0: (u64, i32, u8),
fld1: [isize; 8],
fld2: u32,
fld3: *const u128,
fld4: u64,
fld5: u16,

},
Variant2{
fld0: (f32, u16, isize, u128),
fld1: char,
fld2: *const i16,
fld3: Adt49,
fld4: Adt50,
fld5: Adt44,
fld6: Adt57,
fld7: *const u128,

},
Variant3{
fld0: Adt45,
fld1: u32,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: (f32, u16, isize, u128),
fld1: [usize; 5],
fld2: (i64, u128, i8, f64),
fld3: *mut i16,
fld4: f32,
fld5: [usize; 7],
fld6: Adt47,
}

