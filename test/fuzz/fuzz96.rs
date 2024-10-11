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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u16,mut _7: u32,mut _8: i128,mut _9: usize,mut _10: u8) -> (i128, u32, i32) {
mir! {
type RET = (i128, u32, i32);
let _11: i128;
let _12: [char; 2];
let _13: char;
let _14: i16;
let _15: [i16; 1];
let _16: i128;
let _17: (i128, (u8,));
let _18: i16;
let _19: i64;
let _20: i128;
let _21: [i16; 1];
let _22: Adt53;
let _23: (u8,);
let _24: [i16; 1];
let _25: [i128; 6];
let _26: bool;
let _27: f64;
let _28: *const [i8; 2];
let _29: [char; 2];
let _30: *mut (usize, [i128; 6]);
let _31: i128;
let _32: [i16; 1];
let _33: bool;
let _34: isize;
let _35: [i128; 6];
let _36: bool;
let _37: (u8,);
let _38: ();
let _39: ();
{
_1 = !false;
RET.2 = -866057805_i32;
_8 = _1 as i128;
_6 = 22460_u16 ^ 51335_u16;
_3 = 5170717282942446417_u64 as isize;
_9 = 8614146932614349117_usize * 3_usize;
_3 = !118_isize;
RET = (_8, 916291676_u32, 898003787_i32);
RET.1 = 913692770_u32;
_10 = _9 as u8;
_6 = 43656_u16 & 18439_u16;
RET.2 = (-39_i8) as i32;
_2 = '\u{b89ab}';
_15 = [(-25401_i16)];
_4 = !(-120_i8);
_13 = _2;
_14 = 9176_i16;
_16 = _8 << _6;
_1 = true;
_7 = 243852313841095402368193658183799326052_u128 as u32;
RET.0 = _16;
Call(RET.2 = fn1(RET.0, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = _2;
_4 = (-8_i8);
_17.1.0 = _10 << _6;
RET.1 = _13 as u32;
_17.0 = 5601506619946373625_u64 as i128;
RET.1 = !_7;
Goto(bb2)
}
bb2 = {
_13 = _2;
_17.0 = _16 & RET.0;
_4 = 14931040026424236760_u64 as i8;
_11 = -RET.0;
RET.2 = !2069135085_i32;
_11 = _6 as i128;
_19 = !(-5127505517631535242_i64);
_12 = [_2,_2];
match _14 {
0 => bb3,
9176 => bb5,
_ => bb4
}
}
bb3 = {
_13 = _2;
_4 = (-8_i8);
_17.1.0 = _10 << _6;
RET.1 = _13 as u32;
_17.0 = 5601506619946373625_u64 as i128;
RET.1 = !_7;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
RET.1 = _7 | _7;
_17.0 = !_16;
_8 = _11;
_23.0 = !_10;
_23.0 = _19 as u8;
_6 = RET.2 as u16;
_2 = _13;
Goto(bb6)
}
bb6 = {
_22.fld0 = _1;
_7 = RET.2 as u32;
_14 = (-1912_i16) ^ 12145_i16;
_6 = !39664_u16;
RET.2 = (-1523191997_i32);
_1 = _16 != _17.0;
_4 = (-49_i8) | 120_i8;
_21 = [_14];
_15 = [_14];
_1 = _22.fld0;
_5 = _14 ^ _14;
_3 = _6 as isize;
_3 = -(-9223372036854775808_isize);
RET = (_8, _7, (-577081977_i32));
_11 = _17.0;
RET.0 = -_16;
_22 = Adt53 { fld0: _1 };
Goto(bb7)
}
bb7 = {
RET.0 = _17.0 * _16;
_23.0 = _17.1.0;
_4 = _19 as i8;
_20 = !RET.0;
_6 = RET.1 as u16;
RET.1 = _4 as u32;
_2 = _13;
_17.1.0 = !_23.0;
_7 = RET.1 & RET.1;
RET = (_8, _7, 2134436043_i32);
_3 = (-93_isize);
_27 = 9885600762157564654_u64 as f64;
_12 = [_13,_13];
RET.2 = (-117322138_i32) * (-1966091161_i32);
_27 = 10797437149787525980_u64 as f64;
_24 = [_5];
_17 = (_8, _23);
_16 = 123988528129536820614257672826794207962_u128 as i128;
RET.0 = _8;
_6 = 64472_u16 << _8;
_17 = (RET.0, _23);
_17.1.0 = _10 ^ _23.0;
_7 = !RET.1;
_2 = _13;
match _3 {
0 => bb4,
1 => bb2,
340282366920938463463374607431768211363 => bb9,
_ => bb8
}
}
bb8 = {
_13 = _2;
_17.0 = _16 & RET.0;
_4 = 14931040026424236760_u64 as i8;
_11 = -RET.0;
RET.2 = !2069135085_i32;
_11 = _6 as i128;
_19 = !(-5127505517631535242_i64);
_12 = [_2,_2];
match _14 {
0 => bb3,
9176 => bb5,
_ => bb4
}
}
bb9 = {
_25 = [_11,_11,RET.0,_20,_17.0,_20];
_17 = (RET.0, _23);
Goto(bb10)
}
bb10 = {
_26 = _1 < _22.fld0;
_27 = 12105099251695611500_u64 as f64;
RET.2 = 396678288_i32 | (-619680554_i32);
_5 = _14 & _14;
_20 = -RET.0;
_29 = [_13,_2];
_15 = [_14];
_17.1 = (_23.0,);
_25 = [_20,_11,_17.0,_17.0,_17.0,_11];
_19 = _7 as i64;
_10 = _23.0;
_36 = _26 | _1;
_18 = _5;
_2 = _13;
match _3 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607431768211363 => bb15,
_ => bb14
}
}
bb11 = {
_13 = _2;
_4 = (-8_i8);
_17.1.0 = _10 << _6;
RET.1 = _13 as u32;
_17.0 = 5601506619946373625_u64 as i128;
RET.1 = !_7;
Goto(bb2)
}
bb12 = {
_13 = _2;
_17.0 = _16 & RET.0;
_4 = 14931040026424236760_u64 as i8;
_11 = -RET.0;
RET.2 = !2069135085_i32;
_11 = _6 as i128;
_19 = !(-5127505517631535242_i64);
_12 = [_2,_2];
match _14 {
0 => bb3,
9176 => bb5,
_ => bb4
}
}
bb13 = {
_13 = _2;
_4 = (-8_i8);
_17.1.0 = _10 << _6;
RET.1 = _13 as u32;
_17.0 = 5601506619946373625_u64 as i128;
RET.1 = !_7;
Goto(bb2)
}
bb14 = {
_22.fld0 = _1;
_7 = RET.2 as u32;
_14 = (-1912_i16) ^ 12145_i16;
_6 = !39664_u16;
RET.2 = (-1523191997_i32);
_1 = _16 != _17.0;
_4 = (-49_i8) | 120_i8;
_21 = [_14];
_15 = [_14];
_1 = _22.fld0;
_5 = _14 ^ _14;
_3 = _6 as isize;
_3 = -(-9223372036854775808_isize);
RET = (_8, _7, (-577081977_i32));
_11 = _17.0;
RET.0 = -_16;
_22 = Adt53 { fld0: _1 };
Goto(bb7)
}
bb15 = {
_14 = _18;
_21 = [_5];
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(0_usize, 12_usize, Move(_12), 16_usize, Move(_16), 15_usize, Move(_15), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(0_usize, 4_usize, Move(_4), 21_usize, Move(_21), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(0_usize, 24_usize, Move(_24), 26_usize, Move(_26), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(0_usize, 18_usize, Move(_18), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i128,mut _2: isize) -> i32 {
mir! {
type RET = i32;
let _3: [u128; 2];
let _4: Adt53;
let _5: [i128; 6];
let _6: *mut [i64; 3];
let _7: u32;
let _8: *mut [i64; 3];
let _9: *mut [i64; 3];
let _10: i128;
let _11: i16;
let _12: i64;
let _13: *mut (usize, [i128; 6]);
let _14: bool;
let _15: usize;
let _16: (u8, i8, [i64; 3], usize);
let _17: (usize, [i128; 6]);
let _18: u64;
let _19: char;
let _20: ();
let _21: ();
{
_1 = !(-58110390718658964405073458947431465364_i128);
_2 = (-115_isize) | 9223372036854775807_isize;
RET = '\u{e5211}' as i32;
RET = -(-771078072_i32);
_2 = -(-126_isize);
_2 = (-28_isize);
RET = (-2097456619_i32);
RET = 230994925_i32;
_3 = [136887790863481600206534935702150907198_u128,255944656348449150871186666062244478055_u128];
_3 = [156353172026014559194257434760698333355_u128,24806509942393652180973434758542173145_u128];
RET = 28685027_i32 << _2;
_1 = !116515632729160028569038990937033278673_i128;
_2 = -(-9223372036854775808_isize);
RET = (-2112845547_i32) ^ 1819960143_i32;
_1 = 30503_i16 as i128;
RET = 251436642_i32 - 652982582_i32;
_3 = [222219766972600379851308138601740516038_u128,20206438967825855515532982953880145566_u128];
Goto(bb1)
}
bb1 = {
_2 = (-9223372036854775808_isize) >> _1;
_2 = !(-9223372036854775808_isize);
_2 = 9223372036854775807_isize;
_3 = [99363942376216076760598687283957212854_u128,282092068826280419262084056071454677071_u128];
_2 = (-97_isize);
_4.fld0 = true;
_3 = [25092177550631157206844346441078177138_u128,288990442561458007885061289246286101550_u128];
_5 = [_1,_1,_1,_1,_1,_1];
_2 = -36_isize;
_5 = [_1,_1,_1,_1,_1,_1];
RET = 15992145255600738097_usize as i32;
RET = (-707489077_i32);
_5 = [_1,_1,_1,_1,_1,_1];
_5 = [_1,_1,_1,_1,_1,_1];
RET = (-886787750_i32);
_4 = Adt53 { fld0: false };
_1 = _2 as i128;
_7 = 3878349376_u32 | 1087517535_u32;
RET = 1821952392_i32;
RET = 22836_u16 as i32;
_5 = [_1,_1,_1,_1,_1,_1];
_7 = 2443473696_u32;
_7 = 7_usize as u32;
RET = !(-1124362993_i32);
Goto(bb2)
}
bb2 = {
_1 = 103135514488840060996260642385882283361_i128 >> _2;
_2 = (-28_isize);
_1 = 30362498833186339096579419871776305426_i128 - (-2167531152831310451594755483826443951_i128);
RET = (-1676229835_i32);
RET = 288743037_i32;
RET = (-1519205197_i32) >> _1;
Call(_6 = fn2(_1, _2, _3, RET, _4.fld0, _4.fld0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = [_1,_1,_1,_1,_1,_1];
RET = 47182_u16 as i32;
_8 = _6;
_5 = [_1,_1,_1,_1,_1,_1];
_1 = (-64884568445203605539627768488361073935_i128);
_10 = !_1;
_7 = 1744353731094467042_u64 as u32;
_8 = _6;
_5 = [_1,_1,_10,_1,_10,_10];
RET = -(-1830363670_i32);
_10 = 37_u8 as i128;
_6 = _8;
RET = 385153765_i32 + 920597641_i32;
_3 = [151009330814759353047715388457094681492_u128,320825095599693910408208287168372348331_u128];
_9 = _6;
_9 = _8;
RET = !1582573554_i32;
_9 = _6;
_2 = (-9223372036854775808_isize);
_4.fld0 = !false;
_4 = Adt53 { fld0: true };
_2 = _7 as isize;
RET = 1054184509_i32;
RET = _2 as i32;
_5 = [_1,_1,_1,_1,_10,_1];
_5 = [_1,_1,_1,_1,_1,_1];
_7 = 769199414_u32 | 1391817206_u32;
_11 = !(-1018_i16);
match _1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
275397798475734857923746838943407137521 => bb8,
_ => bb7
}
}
bb4 = {
_1 = 103135514488840060996260642385882283361_i128 >> _2;
_2 = (-28_isize);
_1 = 30362498833186339096579419871776305426_i128 - (-2167531152831310451594755483826443951_i128);
RET = (-1676229835_i32);
RET = 288743037_i32;
RET = (-1519205197_i32) >> _1;
Call(_6 = fn2(_1, _2, _3, RET, _4.fld0, _4.fld0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2 = (-9223372036854775808_isize) >> _1;
_2 = !(-9223372036854775808_isize);
_2 = 9223372036854775807_isize;
_3 = [99363942376216076760598687283957212854_u128,282092068826280419262084056071454677071_u128];
_2 = (-97_isize);
_4.fld0 = true;
_3 = [25092177550631157206844346441078177138_u128,288990442561458007885061289246286101550_u128];
_5 = [_1,_1,_1,_1,_1,_1];
_2 = -36_isize;
_5 = [_1,_1,_1,_1,_1,_1];
RET = 15992145255600738097_usize as i32;
RET = (-707489077_i32);
_5 = [_1,_1,_1,_1,_1,_1];
_5 = [_1,_1,_1,_1,_1,_1];
RET = (-886787750_i32);
_4 = Adt53 { fld0: false };
_1 = _2 as i128;
_7 = 3878349376_u32 | 1087517535_u32;
RET = 1821952392_i32;
RET = 22836_u16 as i32;
_5 = [_1,_1,_1,_1,_1,_1];
_7 = 2443473696_u32;
_7 = 7_usize as u32;
RET = !(-1124362993_i32);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Goto(bb9)
}
bb9 = {
_11 = (-26410_i16);
RET = !730633515_i32;
_4.fld0 = _2 > _2;
_9 = _8;
_3 = [267278055551679837490321353858500538572_u128,287668566456961231338701158893747319761_u128];
match _11 {
0 => bb4,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607431768185046 => bb15,
_ => bb14
}
}
bb10 = {
_5 = [_1,_1,_1,_1,_1,_1];
RET = 47182_u16 as i32;
_8 = _6;
_5 = [_1,_1,_1,_1,_1,_1];
_1 = (-64884568445203605539627768488361073935_i128);
_10 = !_1;
_7 = 1744353731094467042_u64 as u32;
_8 = _6;
_5 = [_1,_1,_10,_1,_10,_10];
RET = -(-1830363670_i32);
_10 = 37_u8 as i128;
_6 = _8;
RET = 385153765_i32 + 920597641_i32;
_3 = [151009330814759353047715388457094681492_u128,320825095599693910408208287168372348331_u128];
_9 = _6;
_9 = _8;
RET = !1582573554_i32;
_9 = _6;
_2 = (-9223372036854775808_isize);
_4.fld0 = !false;
_4 = Adt53 { fld0: true };
_2 = _7 as isize;
RET = 1054184509_i32;
RET = _2 as i32;
_5 = [_1,_1,_1,_1,_10,_1];
_5 = [_1,_1,_1,_1,_1,_1];
_7 = 769199414_u32 | 1391817206_u32;
_11 = !(-1018_i16);
match _1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
275397798475734857923746838943407137521 => bb8,
_ => bb7
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_2 = (-9223372036854775808_isize) >> _1;
_2 = !(-9223372036854775808_isize);
_2 = 9223372036854775807_isize;
_3 = [99363942376216076760598687283957212854_u128,282092068826280419262084056071454677071_u128];
_2 = (-97_isize);
_4.fld0 = true;
_3 = [25092177550631157206844346441078177138_u128,288990442561458007885061289246286101550_u128];
_5 = [_1,_1,_1,_1,_1,_1];
_2 = -36_isize;
_5 = [_1,_1,_1,_1,_1,_1];
RET = 15992145255600738097_usize as i32;
RET = (-707489077_i32);
_5 = [_1,_1,_1,_1,_1,_1];
_5 = [_1,_1,_1,_1,_1,_1];
RET = (-886787750_i32);
_4 = Adt53 { fld0: false };
_1 = _2 as i128;
_7 = 3878349376_u32 | 1087517535_u32;
RET = 1821952392_i32;
RET = 22836_u16 as i32;
_5 = [_1,_1,_1,_1,_1,_1];
_7 = 2443473696_u32;
_7 = 7_usize as u32;
RET = !(-1124362993_i32);
Goto(bb2)
}
bb14 = {
_1 = 103135514488840060996260642385882283361_i128 >> _2;
_2 = (-28_isize);
_1 = 30362498833186339096579419871776305426_i128 - (-2167531152831310451594755483826443951_i128);
RET = (-1676229835_i32);
RET = 288743037_i32;
RET = (-1519205197_i32) >> _1;
Call(_6 = fn2(_1, _2, _3, RET, _4.fld0, _4.fld0), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_1 = (-848580482087555609_i64) as i128;
_2 = _4.fld0 as isize;
_16.0 = 170_u8 ^ 221_u8;
_16.3 = 68_i8 as usize;
_1 = _10 << _10;
_19 = '\u{104a38}';
_17.1 = [_1,_1,_1,_10,_10,_1];
_8 = core::ptr::addr_of_mut!(_16.2);
_18 = !11578651580052500396_u64;
_6 = _9;
_16.1 = (-28_i8);
RET = -(-71829272_i32);
_7 = 3916379317_u32;
_13 = core::ptr::addr_of_mut!(_17);
(*_13) = (_16.3, _5);
_11 = (*_13).0 as i16;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(1_usize, 17_usize, Move(_17), 2_usize, Move(_2), 18_usize, Move(_18), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(1_usize, 1_usize, Move(_1), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: isize,mut _3: [u128; 2],mut _4: i32,mut _5: bool,mut _6: bool) -> *mut [i64; 3] {
mir! {
type RET = *mut [i64; 3];
let _7: (usize, [i128; 6]);
let _8: (usize, [i128; 6]);
let _9: f64;
let _10: [i8; 2];
let _11: f32;
let _12: u128;
let _13: isize;
let _14: isize;
let _15: (u16, (i64, *const i16), [u128; 2], &'static usize, isize);
let _16: isize;
let _17: char;
let _18: char;
let _19: u64;
let _20: f64;
let _21: char;
let _22: [char; 2];
let _23: f32;
let _24: *const *mut [i64; 3];
let _25: [i8; 2];
let _26: f32;
let _27: (u128, [i8; 2]);
let _28: (u128, [i8; 2]);
let _29: f32;
let _30: *mut u64;
let _31: (u128, [i8; 2]);
let _32: [i16; 1];
let _33: f32;
let _34: Adt44;
let _35: Adt44;
let _36: [char; 2];
let _37: [u32; 6];
let _38: u8;
let _39: Adt54;
let _40: u128;
let _41: (u16, (i64, *const i16), [u128; 2], &'static usize, isize);
let _42: isize;
let _43: isize;
let _44: (u16, (i64, *const i16), [u128; 2], &'static usize, isize);
let _45: f64;
let _46: f64;
let _47: isize;
let _48: [i32; 7];
let _49: Adt50;
let _50: Adt47;
let _51: Adt59;
let _52: [char; 2];
let _53: f64;
let _54: [i8; 2];
let _55: char;
let _56: u32;
let _57: *const *const [i8; 2];
let _58: bool;
let _59: u64;
let _60: f64;
let _61: isize;
let _62: bool;
let _63: u128;
let _64: (i128, u32, i32);
let _65: char;
let _66: bool;
let _67: i32;
let _68: bool;
let _69: [char; 2];
let _70: *const i16;
let _71: *mut u64;
let _72: bool;
let _73: [i8; 2];
let _74: (u32, &'static usize, (i128, u32, i32));
let _75: i128;
let _76: [i16; 1];
let _77: [i32; 7];
let _78: (i128, (u8,));
let _79: Adt44;
let _80: [char; 2];
let _81: [i32; 7];
let _82: Adt60;
let _83: [i32; 7];
let _84: i128;
let _85: usize;
let _86: Adt48;
let _87: (i128, (u8,));
let _88: [u32; 6];
let _89: (i128, (u8,));
let _90: i64;
let _91: [u32; 6];
let _92: *mut [i64; 3];
let _93: isize;
let _94: Adt54;
let _95: (i128, (u8,));
let _96: isize;
let _97: (usize, [i128; 6]);
let _98: Adt59;
let _99: bool;
let _100: char;
let _101: isize;
let _102: f32;
let _103: char;
let _104: Adt52;
let _105: f64;
let _106: f32;
let _107: (usize, [i128; 6]);
let _108: [u128; 2];
let _109: *mut [i64; 3];
let _110: [i16; 1];
let _111: Adt52;
let _112: f32;
let _113: u128;
let _114: [u128; 2];
let _115: (u8,);
let _116: isize;
let _117: [i128; 6];
let _118: isize;
let _119: ();
let _120: ();
{
_6 = _5 <= _5;
_8.0 = 1_usize;
_2 = (-9223372036854775808_isize) << _1;
_1 = 41847075894681608541110825010687237515_i128 + 75533527167546754708172379976996155245_i128;
_7.1 = [_1,_1,_1,_1,_1,_1];
_2 = 9223372036854775807_isize | 9223372036854775807_isize;
_6 = !_5;
_7.0 = _6 as usize;
_3 = [21822129572413210268790371400255165299_u128,302847513480236135543042006558354719515_u128];
_8 = (_7.0, _7.1);
_6 = _7.0 <= _7.0;
_1 = 110708976590535644293037039209441054222_i128 + 106387679502388651937488418721660685235_i128;
_1 = 126_u8 as i128;
_7.1 = [_1,_1,_1,_1,_1,_1];
_2 = 58_isize;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
58 => bb8,
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
_11 = 5881_u16 as f32;
_9 = 9163291896278310967_u64 as f64;
_7 = _8;
_7.0 = _4 as usize;
_9 = 1677765449_u32 as f64;
_1 = 37815523949725528807986465424776780432_i128 << _2;
_7.0 = _8.0;
_6 = _2 != _2;
_9 = 36472_u16 as f64;
_13 = _2;
_13 = -_2;
_11 = 2960709049776815869_i64 as f32;
_7 = _8;
_7.1 = [_1,_1,_1,_1,_1,_1];
_3 = [289250450321885730768521385047571695842_u128,38352958149841603327706816375308920220_u128];
_15.3 = &_7.0;
_5 = _9 > _9;
_7.1 = [_1,_1,_1,_1,_1,_1];
Goto(bb9)
}
bb9 = {
_5 = _6 ^ _6;
_8.1 = _7.1;
_8 = (_7.0, _7.1);
_17 = '\u{b6584}';
_8.1 = [_1,_1,_1,_1,_1,_1];
_15.0 = _4 as u16;
_8 = (_7.0, _7.1);
_8.0 = !_7.0;
_7 = (_8.0, _8.1);
_4 = (-1352617518_i32);
_7.0 = _8.0;
_15.4 = _13 << _8.0;
_8 = (_7.0, _7.1);
_10 = [35_i8,(-62_i8)];
_8.0 = _7.0;
_16 = _2 << _1;
_15.1.0 = (-2432413491435656454_i64) >> _8.0;
_20 = -_9;
Call(_8.1 = fn3(_5, _7, _11, _5, _20, _10, _17, _15.4, _1, _7.1, _15.4, _15.0, _16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7.0 = _1 as usize;
_11 = _9 as f32;
_5 = !_6;
_15.4 = _13 << _7.0;
_19 = 17575233100238894702_u64 ^ 13435747368382731187_u64;
_10 = [(-121_i8),(-60_i8)];
_18 = _17;
_12 = 48_i8 as u128;
_9 = _12 as f64;
_14 = _15.4;
_21 = _18;
Goto(bb11)
}
bb11 = {
_12 = 147059880656891421429602334275746887621_u128;
_1 = !117986347897559731211319960237097967955_i128;
_2 = !_13;
_3 = [_12,_12];
_15.3 = &_7.0;
_24 = core::ptr::addr_of!(RET);
_3 = [_12,_12];
_15.2 = [_12,_12];
_22 = [_18,_21];
_7.0 = _8.0 | _8.0;
_15.3 = &_8.0;
_23 = -_11;
_5 = !_6;
_4 = (-1845179979_i32) | (-1663841821_i32);
_15.2 = _3;
_3 = _15.2;
_29 = _23;
_10 = [49_i8,(-102_i8)];
_10 = [(-101_i8),67_i8];
_30 = core::ptr::addr_of_mut!(_19);
_1 = !158113259346634388872336519044619466017_i128;
_27.0 = _12;
match _12 {
0 => bb3,
147059880656891421429602334275746887621 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_11 = _15.1.0 as f32;
_27.1 = [18_i8,(-76_i8)];
(*_30) = 4952529214130379036_u64;
_16 = _13;
_28 = (_12, _27.1);
_14 = -_16;
_27.1 = [(-62_i8),0_i8];
_6 = _5;
_20 = _9 - _9;
_12 = _28.0;
_25 = [63_i8,(-21_i8)];
(*_30) = !5238574645561815325_u64;
_26 = _12 as f32;
_1 = !47815503851103709188809389197544312251_i128;
_28.1 = [33_i8,(-27_i8)];
_12 = !_27.0;
_5 = _6;
_8 = (_7.0, _7.1);
_31.1 = _10;
_34 = Adt44::Variant1 { fld0: _17 };
_27.1 = [(-59_i8),5_i8];
_16 = _13 * _14;
_8.0 = 215_u8 as usize;
_30 = core::ptr::addr_of_mut!((*_30));
_11 = -_29;
match _28.0 {
0 => bb14,
147059880656891421429602334275746887621 => bb16,
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
_21 = _17;
_12 = Field::<char>(Variant(_34, 1), 0) as u128;
_13 = _2 * _16;
_31.0 = _28.0;
_25 = [41_i8,(-72_i8)];
_31.1 = _25;
_10 = [4_i8,119_i8];
_12 = _27.0 - _31.0;
_33 = -_29;
_15.3 = &_7.0;
_4 = 1461248245_i32;
Goto(bb17)
}
bb17 = {
_22 = [Field::<char>(Variant(_34, 1), 0),Field::<char>(Variant(_34, 1), 0)];
_6 = !_5;
SetDiscriminant(_34, 1);
_27 = _31;
_19 = 17565835140384425575_u64;
_36 = [_21,_17];
_32 = [(-9575_i16)];
_41.1.0 = _4 as i64;
Goto(bb18)
}
bb18 = {
_42 = _15.4 & _15.4;
_41.2 = [_12,_27.0];
_8 = (_7.0, _7.1);
_38 = !200_u8;
_15.1.0 = _41.1.0 + _41.1.0;
_41.3 = &_8.0;
_28.1 = _31.1;
place!(Field::<char>(Variant(_34, 1), 0)) = _17;
_28 = (_27.0, _27.1);
_19 = 15534335797436187451_u64 * 2675771092998432853_u64;
_41.1.0 = _15.1.0;
_8.0 = _38 as usize;
_27.0 = _28.0 + _28.0;
_35 = Move(_34);
_44.3 = &_7.0;
_40 = _28.0 << _42;
_23 = _1 as f32;
_28 = (_40, _25);
_44.0 = _4 as u16;
_17 = _21;
(*_30) = 8393191589052677342_u64 - 3068227231772904441_u64;
_41.1.0 = _15.1.0;
Goto(bb19)
}
bb19 = {
Goto(bb20)
}
bb20 = {
_30 = core::ptr::addr_of_mut!(_19);
_24 = core::ptr::addr_of!((*_24));
_2 = _42;
_32 = [5546_i16];
_16 = _42 - _13;
_41.0 = !_15.0;
Goto(bb21)
}
bb21 = {
_27.1 = [56_i8,46_i8];
_7 = (_8.0, _8.1);
_28.1 = [15_i8,51_i8];
_8.0 = _7.0;
_44.2 = [_40,_28.0];
_27.1 = [(-26_i8),(-125_i8)];
_1 = !78196850510396432934258093122371510841_i128;
_43 = -_2;
_3 = [_28.0,_40];
_41.4 = _15.0 as isize;
_15.3 = &_7.0;
_51.fld0 = _7.0 >> _43;
_38 = !21_u8;
_51.fld2 = _20 * _20;
_8.0 = !_51.fld0;
_7 = (_8.0, _8.1);
_51.fld7 = Adt53 { fld0: _6 };
_13 = _16;
_46 = _4 as f64;
_34 = Move(_35);
_48 = [_4,_4,_4,_4,_4,_4,_4];
_44.1.0 = -_15.1.0;
_51.fld1 = (_51.fld0, _7.1);
Goto(bb22)
}
bb22 = {
_33 = _23;
_15.0 = !_44.0;
_2 = _16 >> _14;
_24 = core::ptr::addr_of!((*_24));
_15.4 = _13;
_44.3 = &_7.0;
_9 = 3335884278_u32 as f64;
_51.fld5 = _51.fld1.1;
_41.3 = &_51.fld0;
_35 = Move(_34);
_44.4 = _42;
_41.4 = -_13;
_25 = [107_i8,30_i8];
_31.1 = [(-62_i8),(-53_i8)];
_27.0 = _40;
_15.2 = [_40,_28.0];
_31 = _28;
_51.fld5 = [_1,_1,_1,_1,_1,_1];
_7.0 = _44.1.0 as usize;
_52 = [_18,_17];
_18 = _21;
_12 = _38 as u128;
_24 = core::ptr::addr_of!((*_24));
_17 = Field::<char>(Variant(_35, 1), 0);
Call(_59 = core::intrinsics::transmute(_16), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_21 = Field::<char>(Variant(_35, 1), 0);
_8 = (_7.0, _51.fld1.1);
SetDiscriminant(_35, 0);
_3 = _15.2;
_51.fld1.1 = [_1,_1,_1,_1,_1,_1];
_32 = [13045_i16];
_7.1 = [_1,_1,_1,_1,_1,_1];
(*_30) = _59 << _16;
_46 = _51.fld2 + _9;
_40 = _28.0;
_40 = _4 as u128;
_55 = _21;
_31 = (_28.0, _28.1);
Goto(bb24)
}
bb24 = {
_7.0 = _18 as usize;
_28.0 = _38 as u128;
_51.fld7 = Adt53 { fld0: _5 };
_22 = [_18,_18];
_29 = -_26;
_51.fld7.fld0 = _46 < _46;
_41.1.0 = _44.1.0 << (*_30);
_44.3 = &_51.fld1.0;
_51.fld1.0 = _51.fld0 << _41.1.0;
_5 = _51.fld7.fld0;
_18 = _55;
_21 = _17;
_62 = !_5;
_17 = _21;
_7 = _51.fld1;
_51.fld6 = _41.1.0;
_4 = 121_i8 as i32;
_58 = _51.fld6 <= _41.1.0;
_41.3 = &_7.0;
_54 = [(-2_i8),(-77_i8)];
Call(_22 = fn13(_58, _42, _41.4, _41.2, _7.0, _27, _13, _41.4, _51.fld1.0, _51.fld1, _41.4, _51.fld6, _19), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_60 = _51.fld2;
_64.0 = _1 + _1;
_44.2 = [_27.0,_31.0];
place!(Field::<i128>(Variant(_35, 0), 1)) = _64.0 & _64.0;
_35 = Adt44::Variant1 { fld0: _17 };
_41.3 = &_8.0;
_32 = [(-11538_i16)];
_19 = !_59;
_51.fld3 = [Field::<char>(Variant(_35, 1), 0),_17,Field::<char>(Variant(_35, 1), 0),_21,_17,Field::<char>(Variant(_35, 1), 0),_55];
_65 = _21;
_61 = _13;
_63 = _28.0 << _51.fld6;
_26 = -_29;
_7 = (_51.fld0, _51.fld1.1);
_44.4 = !_43;
_34 = Move(_35);
_25 = [(-56_i8),110_i8];
_28.1 = [9_i8,17_i8];
_14 = -_44.4;
_51.fld7 = Adt53 { fld0: _58 };
_31.1 = [66_i8,(-24_i8)];
_24 = core::ptr::addr_of!(RET);
Goto(bb26)
}
bb26 = {
SetDiscriminant(_34, 1);
_7 = (_8.0, _51.fld1.1);
_3 = [_63,_31.0];
_34 = Adt44::Variant1 { fld0: _18 };
_29 = _26;
_9 = _51.fld2;
_3 = _41.2;
_51.fld0 = (-16909_i16) as usize;
_59 = (*_30);
_64.1 = _61 as u32;
_8.1 = [_64.0,_64.0,_1,_64.0,_1,_1];
_41.0 = _51.fld7.fld0 as u16;
(*_30) = _59 ^ _59;
place!(Field::<char>(Variant(_34, 1), 0)) = _18;
_46 = -_60;
_64.0 = -_1;
_7.0 = _4 as usize;
_41.0 = !_15.0;
_74.0 = _58 as u32;
_71 = _30;
_44.1.0 = !_51.fld6;
_47 = !_44.4;
_74.1 = &_51.fld1.0;
_35 = Move(_34);
_53 = -_51.fld2;
Goto(bb27)
}
bb27 = {
_7.1 = _8.1;
_72 = _5 != _58;
_56 = _74.0;
_14 = _13;
_7.1 = _8.1;
_15.3 = Move(_74.1);
_67 = !_4;
_74.1 = &_51.fld1.0;
_51.fld0 = !_51.fld1.0;
_51.fld2 = _46;
_71 = core::ptr::addr_of_mut!((*_71));
_45 = _38 as f64;
_12 = _4 as u128;
_56 = _74.0;
SetDiscriminant(_35, 0);
_77 = [_67,_67,_4,_4,_4,_67,_67];
_41.1.0 = _17 as i64;
(*_30) = _58 as u64;
Goto(bb28)
}
bb28 = {
_19 = _59;
_68 = _56 >= _56;
(*_30) = !_59;
_75 = _64.0;
_27 = (_31.0, _25);
_60 = _9;
_78.1 = (_38,);
_33 = -_23;
_15.2 = _41.2;
_78.1.0 = _29 as u8;
_28.0 = 3904_i16 as u128;
_27.1 = [(-104_i8),(-72_i8)];
_78.1 = (_38,);
_2 = (*_30) as isize;
_30 = _71;
_44.2 = [_27.0,_31.0];
Call(_15.1.0 = core::intrinsics::bswap(_51.fld6), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
_66 = _58 <= _68;
_78.0 = _75;
_44.3 = &_7.0;
_25 = [(-79_i8),(-100_i8)];
_14 = !_16;
_55 = _65;
_51.fld5 = [_75,_1,_78.0,_1,_75,_78.0];
_54 = [73_i8,99_i8];
_35 = Adt44::Variant1 { fld0: _65 };
_80 = [_18,_18];
SetDiscriminant(_35, 1);
_28.1 = [(-67_i8),100_i8];
_67 = _4;
_52 = _22;
_33 = _23 - _26;
_45 = _26 as f64;
_74.2.1 = _59 as u32;
_41.1.0 = _63 as i64;
_64.0 = _1 << _74.0;
_78.1 = (_38,);
_51.fld1.0 = !_51.fld0;
_44.0 = !_15.0;
_27 = (_12, _28.1);
Call(_44.2 = core::intrinsics::transmute(_15.2), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_43 = _41.4 + _41.4;
_72 = _68;
_35 = Adt44::Variant1 { fld0: _65 };
_27.0 = _72 as u128;
_74.1 = &_85;
_81 = _48;
_51.fld0 = !_51.fld1.0;
_32 = [(-10216_i16)];
_13 = _17 as isize;
_38 = _78.1.0 >> _43;
_3 = [_63,_27.0];
_31 = _27;
_51.fld6 = _44.1.0 | _41.1.0;
_28.0 = _27.0;
_64.1 = _43 as u32;
_74.2.0 = _64.0;
Goto(bb31)
}
bb31 = {
_74.2.1 = _56 * _74.0;
_73 = [44_i8,(-14_i8)];
_20 = _9;
_41.4 = _43;
_41.3 = &_51.fld0;
_43 = Field::<char>(Variant(_35, 1), 0) as isize;
Goto(bb32)
}
bb32 = {
_27.0 = _31.0;
_37 = [_74.2.1,_56,_56,_74.2.1,_74.2.1,_74.2.1];
_8.0 = !_51.fld0;
_52 = _36;
_63 = _27.0 + _27.0;
_33 = _29;
_51.fld5 = [_64.0,_64.0,_64.0,_74.2.0,_74.2.0,_74.2.0];
_60 = _53 - _9;
_44.4 = _53 as isize;
_15.3 = &_8.0;
_54 = [(-86_i8),78_i8];
_44.1.0 = _41.1.0;
_52 = [Field::<char>(Variant(_35, 1), 0),_17];
_83 = [_67,_4,_4,_4,_4,_67,_67];
_9 = _53;
_72 = !_58;
Call(_15.1 = fn15(_8.0, _37, _68, _2, _63, _27.0, _31.0), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
_33 = _38 as f32;
_28.1 = _54;
_8.1 = _51.fld5;
SetDiscriminant(_35, 1);
_1 = _15.1.0 as i128;
_24 = core::ptr::addr_of!(RET);
_87.0 = _74.2.0 * _64.0;
_51.fld7.fld0 = !_72;
_45 = _60 + _60;
_51.fld0 = !_8.0;
_89.1 = (_38,);
_15.3 = Move(_44.3);
_70 = _15.1.1;
_27 = _28;
_31 = (_28.0, _54);
_74.2.2 = _67;
_29 = _33;
_15.1 = (_51.fld6, _70);
_8 = (_51.fld1.0, _51.fld5);
_87 = (_64.0, _89.1);
_51.fld3 = [_18,_18,_65,_55,_17,_65,_18];
_91 = _37;
Goto(bb34)
}
bb34 = {
_87.0 = -_1;
_88 = _37;
_41.3 = &_7.0;
_89.0 = _20 as i128;
_41.2 = [_27.0,_28.0];
_8.0 = _51.fld1.0 ^ _51.fld0;
_41.1.1 = _15.1.1;
_8 = (_51.fld1.0, _51.fld5);
_69 = [_55,_17];
_59 = !(*_30);
_15.2 = _41.2;
_73 = _25;
_31.1 = [28_i8,19_i8];
_15.0 = 3018_i16 as u16;
Goto(bb35)
}
bb35 = {
_73 = _28.1;
_34 = Adt44::Variant1 { fld0: _17 };
_89.1.0 = !_78.1.0;
_15 = Move(_41);
_15.4 = !_42;
_31 = (_28.0, _73);
_74.0 = !_74.2.1;
SetDiscriminant(_34, 1);
_15.1.1 = _70;
_58 = _66;
_84 = -_1;
_73 = _54;
_41.1 = _15.1;
_58 = _66;
_15.1.0 = _51.fld6;
_95 = _87;
_91 = _37;
_51.fld1.0 = _53 as usize;
_95.1 = _87.1;
_31.1 = [124_i8,88_i8];
Goto(bb36)
}
bb36 = {
_74.0 = _56;
(*_30) = _59;
_98.fld7.fld0 = _66;
_59 = (*_30) ^ (*_30);
_98.fld3 = [_21,_17,_65,_21,_18,_18,_55];
Call(_74.2.2 = core::intrinsics::transmute(_74.2.1), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
_75 = _87.1.0 as i128;
_98.fld1 = (_8.0, _8.1);
_8 = (_51.fld0, _51.fld5);
_65 = _17;
place!(Field::<char>(Variant(_35, 1), 0)) = _55;
_15.2 = _3;
_28.1 = _73;
_51.fld7.fld0 = _74.2.2 != _74.2.2;
_8 = (_51.fld1.0, _51.fld5);
_51.fld6 = _41.1.0;
_63 = _27.0;
Goto(bb38)
}
bb38 = {
_9 = _44.0 as f64;
_44.1 = (_15.1.0, _70);
Goto(bb39)
}
bb39 = {
_41.3 = &_98.fld1.0;
_15.1 = _41.1;
_51.fld0 = !_98.fld1.0;
_95.1 = (_87.1.0,);
_98.fld7.fld0 = _72;
_85 = !_51.fld0;
_41.1.1 = _15.1.1;
_43 = _61;
_22 = [Field::<char>(Variant(_35, 1), 0),_21];
SetDiscriminant(_35, 0);
_7.1 = [_1,_75,_87.0,_74.2.0,_95.0,_84];
Goto(bb40)
}
bb40 = {
_21 = _65;
_97 = (_85, _7.1);
_74.1 = &_107.0;
place!(Field::<[char; 7]>(Variant(_35, 0), 3)) = [_17,_18,_17,_65,_17,_21,_21];
_7.0 = !_98.fld1.0;
_2 = _16;
(*_71) = _59;
_15.1.1 = _70;
_107 = _97;
_44.4 = _42 & _15.4;
_2 = _61;
_9 = _60 + _51.fld2;
Goto(bb41)
}
bb41 = {
_70 = _41.1.1;
_35 = Adt44::Variant0 { fld0: _15.0,fld1: _87.0,fld2: _15.4,fld3: _51.fld3 };
(*_71) = !_59;
_80 = _22;
_44.0 = _18 as u16;
Goto(bb42)
}
bb42 = {
_41.0 = _15.0;
_14 = _44.4;
_15.0 = !Field::<u16>(Variant(_35, 0), 0);
_103 = _21;
_58 = _72;
_107.0 = _51.fld0;
_98.fld2 = -_60;
_41.3 = &_107.0;
_98.fld1.1 = [_1,_87.0,_1,Field::<i128>(Variant(_35, 0), 1),_64.0,Field::<i128>(Variant(_35, 0), 1)];
_7 = (_97.0, _97.1);
_73 = [10_i8,21_i8];
_81 = _83;
_31.1 = [(-123_i8),(-110_i8)];
_1 = _87.0 | _87.0;
_67 = _74.2.2 & _74.2.2;
_41.2 = [_28.0,_27.0];
_9 = -_46;
place!(Field::<[char; 7]>(Variant(_35, 0), 3)) = [_17,_55,_18,_21,_18,_18,_17];
_104 = Adt52::Variant1 { fld0: _74.0,fld1: _95 };
_98.fld0 = !_98.fld1.0;
_115 = (Field::<(i128, (u8,))>(Variant(_104, 1), 1).1.0,);
SetDiscriminant(_104, 0);
place!(Field::<(i128, (u8,))>(Variant(_104, 0), 0)) = (_95.0, _115);
Goto(bb43)
}
bb43 = {
_50 = Adt47::Variant1 { fld0: _71 };
_97 = _7;
RET = core::ptr::addr_of_mut!(place!(Field::<[i64; 3]>(Variant(_104, 0), 5)));
_99 = _51.fld7.fld0;
_40 = _41.0 as u128;
_79 = Adt44::Variant0 { fld0: _15.0,fld1: _1,fld2: _16,fld3: _51.fld3 };
_56 = !_64.1;
_100 = _55;
_64.2 = (*_30) as i32;
Goto(bb44)
}
bb44 = {
Call(_119 = dump_var(2_usize, 3_usize, Move(_3), 77_usize, Move(_77), 52_usize, Move(_52), 7_usize, Move(_7)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_119 = dump_var(2_usize, 115_usize, Move(_115), 25_usize, Move(_25), 97_usize, Move(_97), 28_usize, Move(_28)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_119 = dump_var(2_usize, 69_usize, Move(_69), 19_usize, Move(_19), 68_usize, Move(_68), 14_usize, Move(_14)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_119 = dump_var(2_usize, 2_usize, Move(_2), 16_usize, Move(_16), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_119 = dump_var(2_usize, 88_usize, Move(_88), 31_usize, Move(_31), 27_usize, Move(_27), 62_usize, Move(_62)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_119 = dump_var(2_usize, 40_usize, Move(_40), 61_usize, Move(_61), 17_usize, Move(_17), 65_usize, Move(_65)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_119 = dump_var(2_usize, 64_usize, Move(_64), 87_usize, Move(_87), 56_usize, Move(_56), 54_usize, Move(_54)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_119 = dump_var(2_usize, 48_usize, Move(_48), 59_usize, Move(_59), 8_usize, Move(_8), 80_usize, Move(_80)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_119 = dump_var(2_usize, 38_usize, Move(_38), 120_usize, _120, 120_usize, _120, 120_usize, _120), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: (usize, [i128; 6]),mut _3: f32,mut _4: bool,mut _5: f64,mut _6: [i8; 2],mut _7: char,mut _8: isize,mut _9: i128,mut _10: [i128; 6],mut _11: isize,mut _12: u16,mut _13: isize) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _14: i16;
let _15: *const [i8; 2];
let _16: [i16; 1];
let _17: i8;
let _18: usize;
let _19: [char; 2];
let _20: [char; 2];
let _21: *const [i8; 2];
let _22: i64;
let _23: u32;
let _24: f32;
let _25: i64;
let _26: Adt48;
let _27: f64;
let _28: *const *const [i8; 2];
let _29: u32;
let _30: (i64, *const i16);
let _31: ();
let _32: ();
{
_3 = 2938521229_u32 as f32;
RET = [_9,_9,_9,_9,_9,_9];
_10 = _2.1;
_3 = _13 as f32;
_15 = core::ptr::addr_of!(_6);
_6 = [(-127_i8),73_i8];
_9 = -(-127455752364401162298040697178133683628_i128);
(*_15) = [(-51_i8),91_i8];
_11 = _13 & _13;
_10 = [_9,_9,_9,_9,_9,_9];
RET = _2.1;
_7 = '\u{c2804}';
_2.0 = !13033368028602172804_usize;
_5 = 317753725155248336686157362575799799287_u128 as f64;
Call(_12 = fn4(_11, _13, _1, (*_15), _11, (*_15), (*_15), _4, _1, (*_15), _4, (*_15)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_9,_9,_9,_9,_9,_9];
_2 = (7047367608369118505_usize, _10);
_16 = [10376_i16];
_14 = _2.0 as i16;
_10 = [_9,_9,_9,_9,_9,_9];
_2 = (4_usize, _10);
_18 = _2.0 / _2.0;
_4 = _1;
_2.1 = _10;
_13 = _11 | _8;
_11 = _13 * _13;
Goto(bb2)
}
bb2 = {
_2.1 = [_9,_9,_9,_9,_9,_9];
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
5 => bb7,
4 => bb9,
_ => bb8
}
}
bb3 = {
RET = [_9,_9,_9,_9,_9,_9];
_2 = (7047367608369118505_usize, _10);
_16 = [10376_i16];
_14 = _2.0 as i16;
_10 = [_9,_9,_9,_9,_9,_9];
_2 = (4_usize, _10);
_18 = _2.0 / _2.0;
_4 = _1;
_2.1 = _10;
_13 = _11 | _8;
_11 = _13 * _13;
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
_9 = 22953492660331357980870394029856266513_i128;
_19 = [_7,_7];
_2.1 = RET;
_5 = 82692187463941624546221106858821267867_u128 as f64;
_17 = 83166397_u32 as i8;
_17 = -(-41_i8);
_13 = _11;
_10 = RET;
_5 = 1636569088_i32 as f64;
_19 = [_7,_7];
(*_15) = [_17,_17];
Goto(bb10)
}
bb10 = {
_19 = [_7,_7];
_2.1 = RET;
_15 = core::ptr::addr_of!((*_15));
_13 = _8 << _12;
(*_15) = [_17,_17];
_16 = [_14];
_17 = (-82_i8) + (-55_i8);
_5 = 76_u8 as f64;
_12 = !13879_u16;
Call(RET = fn5(_11, _12, _13, _13, _13, _13), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = [_17,_17];
_8 = _13;
_10 = [_9,_9,_9,_9,_9,_9];
_22 = 3287149568727839716_i64;
match _2.0 {
0 => bb1,
1 => bb8,
2 => bb10,
3 => bb4,
4 => bb14,
_ => bb13
}
}
bb12 = {
_2.1 = [_9,_9,_9,_9,_9,_9];
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
5 => bb7,
4 => bb9,
_ => bb8
}
}
bb13 = {
Return()
}
bb14 = {
_20 = [_7,_7];
_2.1 = RET;
_13 = _17 as isize;
RET = _2.1;
_3 = _22 as f32;
_14 = !2340_i16;
_25 = 10385002919657245187_u64 as i64;
(*_15) = [_17,_17];
_20 = [_7,_7];
_6 = [_17,_17];
_24 = _3 + _3;
_21 = _15;
_5 = _17 as f64;
_23 = _2.0 as u32;
_5 = _25 as f64;
_17 = _12 as i8;
_25 = 76_u8 as i64;
_28 = core::ptr::addr_of!(_21);
_17 = -(-77_i8);
(*_15) = [_17,_17];
_28 = core::ptr::addr_of!((*_28));
(*_28) = core::ptr::addr_of!((*_21));
_5 = _24 as f64;
RET = [_9,_9,_9,_9,_9,_9];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(3_usize, 22_usize, Move(_22), 14_usize, Move(_14), 6_usize, Move(_6), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(3_usize, 18_usize, Move(_18), 11_usize, Move(_11), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(3_usize, 23_usize, Move(_23), 16_usize, Move(_16), 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: [i8; 2],mut _5: isize,mut _6: [i8; 2],mut _7: [i8; 2],mut _8: bool,mut _9: bool,mut _10: [i8; 2],mut _11: bool,mut _12: [i8; 2]) -> u16 {
mir! {
type RET = u16;
let _13: [u32; 6];
let _14: Adt60;
let _15: ();
let _16: ();
{
_4 = [(-121_i8),36_i8];
_9 = !_8;
_2 = _1;
_3 = _5 < _5;
_3 = !_11;
RET = !64499_u16;
RET = 10696008173772845700_u64 as u16;
_2 = _1 * _5;
RET = 64319_u16 | 26839_u16;
_7 = [(-125_i8),(-98_i8)];
_5 = 128872868803572904140965863284148281909_u128 as isize;
_5 = _2 ^ _2;
_1 = _5 - _5;
_5 = _1 - _1;
Goto(bb1)
}
bb1 = {
_13 = [3654835856_u32,3115415317_u32,4164935848_u32,1339580315_u32,3401591481_u32,2425904978_u32];
_12 = [(-47_i8),(-73_i8)];
RET = 54298_u16 | 42123_u16;
RET = 37220_u16 | 13336_u16;
RET = 13864_u16 ^ 60786_u16;
_12 = [(-72_i8),(-78_i8)];
RET = 1917454444_i32 as u16;
Call(_2 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _6;
_5 = _2;
RET = 27193_u16;
RET = 46867_u16 << _5;
Goto(bb3)
}
bb3 = {
Call(_15 = dump_var(4_usize, 10_usize, Move(_10), 3_usize, Move(_3), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_15 = dump_var(4_usize, 1_usize, Move(_1), 8_usize, Move(_8), 16_usize, _16, 16_usize, _16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: u16,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _7: [i32; 7];
let _8: i32;
let _9: bool;
let _10: f32;
let _11: *const [i8; 2];
let _12: [char; 7];
let _13: [char; 2];
let _14: *const *mut [i64; 3];
let _15: [char; 7];
let _16: (i128, (u8,));
let _17: bool;
let _18: bool;
let _19: (usize, [i128; 6]);
let _20: isize;
let _21: isize;
let _22: Adt50;
let _23: [i8; 2];
let _24: ();
let _25: ();
{
RET = [50567440958304545500228000422582148337_i128,166268466094352522131460081244728538678_i128,26648529186648739701880306131027202611_i128,(-129685795266213918798753813759885746601_i128),(-1220543678094958786549103743274621872_i128),62051804710176431883536518131155515544_i128];
Goto(bb1)
}
bb1 = {
RET = [153800105378422885522783630171295485743_i128,55808911981116255003756862679048296137_i128,117746149848615108926469977417604386967_i128,(-8404629550925001269205465687194084356_i128),111703231529627769090276823796782090556_i128,(-98272679613209899448117671613598548341_i128)];
Goto(bb2)
}
bb2 = {
_1 = _3;
_4 = -_1;
_4 = '\u{3a395}' as isize;
RET = [75517963124054935305146000105668778344_i128,(-163061350170138939650699728357679593486_i128),(-50190795499574791573377259927309925026_i128),66598876333130844264299181229771509723_i128,112153060574229099038576667764396594140_i128,86310120026900444848451055449300424047_i128];
RET = [43879167298317379858068803543789497106_i128,(-3541414827770677459920505130898688738_i128),38615443148491506841311775981759805401_i128,(-73409893463406712440065745737554313607_i128),132286024612302076326133813321982920155_i128,68545404190517170677266080627358965744_i128];
_7 = [866768070_i32,(-294128694_i32),(-404754331_i32),(-961237552_i32),1063910498_i32,130734681_i32,301574039_i32];
_6 = -_3;
_6 = _5;
_4 = -_3;
RET = [(-83540279816719160097841707290355343053_i128),(-57185374463637144464414880242013202229_i128),(-8527830542218354816513899190691920809_i128),(-40993800363630494667658696334899524042_i128),(-151477944083819140727813688285859652144_i128),(-51103998063187275730698252593928596410_i128)];
RET = [(-82023556886184393783393698519510152932_i128),83054231956685339251189262193101828117_i128,23909802936614082443522790178936219139_i128,(-63592042323139343439788548674480326673_i128),(-92148564628812001679356106364469698566_i128),(-149883932058541423216172947588042383017_i128)];
_9 = true;
RET = [57944163065186622609882549444817353789_i128,(-48396386275055730272946265787562003358_i128),26880999922997612793057477231993971043_i128,58936409539742293045826782536325669494_i128,(-92744056096921050853353388548294937864_i128),(-104657073588811936131660878033877009743_i128)];
_8 = (-1859761217_i32) | 969375084_i32;
_2 = !37665_u16;
_4 = 123_i8 as isize;
RET = [20969777599873702155029304789142018703_i128,92553147947572265324534698821913587752_i128,(-78401626990391657101694827237842848095_i128),(-127266140193026888579259801458367987046_i128),22163353123046555648629156554428792280_i128,109695499692507645954833467250217353269_i128];
_2 = 52392_u16;
_8 = -103165499_i32;
RET = [46377270291659081915960523253318447005_i128,(-94521226197714353345310514824573015259_i128),(-113423447167516880385545646846180195466_i128),59875042776369764785631105759489632608_i128,(-8646094551485617615729859789341774061_i128),(-161727213276802357411181997818430899781_i128)];
_4 = _9 as isize;
RET = [(-90004311757951484508630688521003468530_i128),122606190095858481886197511643586776051_i128,(-9638370867876596294990343536647580916_i128),90520089515421475459628234032162606743_i128,80839429589534840683621947733205794035_i128,60759394491259728380809507364858969081_i128];
_4 = _6 << _6;
_1 = _3 * _3;
match _2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
52392 => bb9,
_ => bb8
}
}
bb3 = {
RET = [153800105378422885522783630171295485743_i128,55808911981116255003756862679048296137_i128,117746149848615108926469977417604386967_i128,(-8404629550925001269205465687194084356_i128),111703231529627769090276823796782090556_i128,(-98272679613209899448117671613598548341_i128)];
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
_6 = _4;
RET = [7690230960892485191624471466127541869_i128,(-14709063586124395041966855709259105852_i128),(-167296103279162756308391020374362320162_i128),145246673782040629252850587407801664484_i128,(-124237147229818192569397363365411981809_i128),(-91423969410119044132124359827790041392_i128)];
RET = [153245078507673739950169438885885192194_i128,(-135295603949497372927572649277731949226_i128),93978472656711863965618038612069192773_i128,147690144358281011511589189652479996652_i128,50285490822640360015329926096348178065_i128,(-16661786714447109110300799038182333719_i128)];
_9 = true;
_9 = !false;
_10 = _8 as f32;
_1 = _2 as isize;
match _2 {
0 => bb5,
1 => bb3,
52392 => bb11,
_ => bb10
}
}
bb10 = {
RET = [153800105378422885522783630171295485743_i128,55808911981116255003756862679048296137_i128,117746149848615108926469977417604386967_i128,(-8404629550925001269205465687194084356_i128),111703231529627769090276823796782090556_i128,(-98272679613209899448117671613598548341_i128)];
Goto(bb2)
}
bb11 = {
_15 = ['\u{f73c0}','\u{49f41}','\u{cc993}','\u{ada2e}','\u{66d61}','\u{748bf}','\u{179f9}'];
_2 = 48390_u16;
_16.1.0 = (-16194_i16) as u8;
_5 = _4 | _3;
_1 = !_6;
RET = [19896138997357670176815553474730082687_i128,(-80439945161966868936888024688798386408_i128),(-89405023208713428517981648257759313016_i128),76018132208103682849632722387127636162_i128,38988550070110401816476909509162760767_i128,100005429423790772483160286380752950351_i128];
_8 = 1511489489_i32;
Goto(bb12)
}
bb12 = {
_13 = ['\u{5fa45}','\u{6fdbd}'];
_1 = _4;
_15 = ['\u{236c6}','\u{accf8}','\u{e5166}','\u{6e082}','\u{93470}','\u{d5e57}','\u{34783}'];
_16.1 = (4_u8,);
_16.1 = (248_u8,);
_12 = ['\u{42340}','\u{6df78}','\u{adea5}','\u{24f62}','\u{aa4a9}','\u{a155}','\u{d8829}'];
_6 = _4;
_4 = _5 + _6;
_16.0 = 104351463464413796939003205267584909490_i128;
_16.1 = (42_u8,);
_16.0 = !67904735681084352136840266320483238019_i128;
_6 = (-107_i8) as isize;
_10 = 20745_i16 as f32;
_9 = true & true;
_1 = _5;
_17 = !_9;
_5 = _3;
_3 = !_4;
_1 = _3;
RET = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
Call(_14 = fn6(_3, _1, _3, _4, _3, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4 = (-30267_i16) as isize;
_7 = [_8,_8,_8,_8,_8,_8,_8];
_10 = 704820582_u32 as f32;
Call(RET = fn12(_5, _1, _1, _3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7 = [_8,_8,_8,_8,_8,_8,_8];
_19.0 = !3_usize;
_16.1.0 = 106_u8 & 117_u8;
_16.1 = (65_u8,);
RET = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_8 = -825587677_i32;
_19.0 = 6423286178781659691_usize << _5;
RET = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_19.1 = RET;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(5_usize, 2_usize, Move(_2), 16_usize, Move(_16), 17_usize, Move(_17), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(5_usize, 6_usize, Move(_6), 4_usize, Move(_4), 9_usize, Move(_9), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> *const *mut [i64; 3] {
mir! {
type RET = *const *mut [i64; 3];
let _7: f32;
let _8: [char; 7];
let _9: f32;
let _10: isize;
let _11: bool;
let _12: f32;
let _13: *const *const [i8; 2];
let _14: [char; 2];
let _15: isize;
let _16: char;
let _17: [i16; 1];
let _18: i16;
let _19: [i16; 1];
let _20: i16;
let _21: i16;
let _22: [i64; 3];
let _23: isize;
let _24: i16;
let _25: isize;
let _26: (u32, &'static usize, (i128, u32, i32));
let _27: char;
let _28: Adt47;
let _29: *mut [i64; 3];
let _30: (usize, [i128; 6]);
let _31: isize;
let _32: *const i16;
let _33: [char; 2];
let _34: u128;
let _35: Adt59;
let _36: isize;
let _37: char;
let _38: &'static usize;
let _39: i128;
let _40: ();
let _41: ();
{
_1 = _3;
_5 = _4 + _1;
_6 = _1 ^ _4;
_6 = !_3;
_3 = 3139015289988239419116800961114247439_u128 as isize;
_7 = 123_u8 as f32;
_4 = true as isize;
_8 = ['\u{6e56c}','\u{a543b}','\u{14c96}','\u{2151e}','\u{bdaad}','\u{65531}','\u{abeb4}'];
_3 = _1;
_2 = -_5;
Goto(bb1)
}
bb1 = {
_4 = (-154520481606082950251651067847020589174_i128) as isize;
_1 = _6 & _3;
_6 = !_2;
_5 = !_3;
_9 = _7;
_10 = !_3;
_2 = _10 ^ _5;
_3 = _2;
_10 = _7 as isize;
Call(_3 = fn7(_2, _1, _5, _6, _2, _6, _5, _6, _2, _2, _6, _6, _1, _5, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _5;
_7 = _9 + _9;
_10 = _6;
_4 = !_5;
_10 = -_5;
_9 = -_7;
_4 = !_1;
_7 = _9 + _9;
_1 = !_10;
_6 = _10;
_10 = _5 | _5;
_5 = _4;
Call(_9 = fn8(_5, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _10;
_5 = _4 - _2;
_2 = _4 >> _1;
_1 = _5 >> _10;
_12 = -_9;
_16 = '\u{b1594}';
_5 = _2 & _2;
_6 = _3 ^ _5;
_1 = _5;
_10 = -_6;
_12 = 362849431_i32 as f32;
_10 = _4 - _3;
_15 = 3391889111_u32 as isize;
_14 = [_16,_16];
_6 = !_10;
_15 = 33_u8 as isize;
_10 = _5;
_8 = [_16,_16,_16,_16,_16,_16,_16];
_14 = [_16,_16];
_10 = (-120692322167073136898277928539689813582_i128) as isize;
Goto(bb4)
}
bb4 = {
_17 = [(-29135_i16)];
_1 = _2;
_5 = _4 + _3;
_8 = [_16,_16,_16,_16,_16,_16,_16];
_12 = 192_u8 as f32;
_5 = _3 << _6;
_17 = [11464_i16];
_17 = [5318_i16];
Goto(bb5)
}
bb5 = {
_9 = 4294245310_u32 as f32;
_14 = [_16,_16];
_10 = -_2;
_2 = 2093365563_u32 as isize;
_18 = 4182574575_u32 as i16;
Call(_2 = core::intrinsics::bswap(_6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = -_3;
_19 = _17;
_8 = [_16,_16,_16,_16,_16,_16,_16];
_20 = _18 | _18;
_9 = _20 as f32;
_3 = _5;
_3 = !_10;
_6 = _10 ^ _1;
_12 = 52769_u16 as f32;
_18 = _20 << _4;
_6 = _4 & _3;
_19 = [_18];
_19 = [_18];
_7 = _12;
_11 = true;
_10 = !_1;
_8 = [_16,_16,_16,_16,_16,_16,_16];
_20 = 38_i8 as i16;
_4 = _3 ^ _3;
_11 = !true;
_20 = _18 + _18;
_7 = 2624289467_u32 as f32;
_7 = _18 as f32;
_23 = (-464782092_i32) as isize;
_20 = _18;
Goto(bb7)
}
bb7 = {
_15 = -_4;
_22 = [6917464247769422603_i64,(-1046177358666868018_i64),6021871605826042481_i64];
_21 = !_18;
_24 = _20 ^ _20;
_17 = _19;
_15 = -_10;
_18 = (-1968503388_i32) as i16;
_10 = _2;
_25 = -_5;
_9 = _7 - _7;
_16 = '\u{1077e5}';
_6 = (-324347915_i32) as isize;
_21 = !_20;
_7 = _9;
_26.2.0 = !88955625598454194459639161942141577852_i128;
_22 = [7007740996132411968_i64,6454191867689653035_i64,(-554047275410837426_i64)];
_23 = -_10;
_14 = [_16,_16];
_14 = [_16,_16];
_10 = -_15;
Call(_2 = fn9(_24, _25, _24, _15, _3, _10, _25, _21), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = [_21];
_12 = -_9;
_5 = 26791437851639402100555679383814583708_u128 as isize;
_6 = 53660_u16 as isize;
_23 = _3 ^ _2;
_14 = [_16,_16];
_16 = '\u{98cef}';
_29 = core::ptr::addr_of_mut!(_22);
Goto(bb9)
}
bb9 = {
_26.2.2 = 2857968212757939128_i64 as i32;
Goto(bb10)
}
bb10 = {
RET = core::ptr::addr_of!(_29);
(*RET) = core::ptr::addr_of_mut!((*_29));
_11 = !true;
_25 = (-5663788336339491503_i64) as isize;
_27 = _16;
_8 = [_27,_27,_16,_27,_16,_27,_16];
_9 = -_7;
_31 = _3;
_26.2.1 = !2950443007_u32;
Goto(bb11)
}
bb11 = {
(*RET) = core::ptr::addr_of_mut!((*_29));
_9 = _12 + _7;
_10 = _2 + _4;
_19 = [_24];
Goto(bb12)
}
bb12 = {
_19 = [_21];
_2 = _1;
_23 = 1865098420822373389_i64 as isize;
_20 = _21 * _21;
_30.1 = [_26.2.0,_26.2.0,_26.2.0,_26.2.0,_26.2.0,_26.2.0];
_33 = [_16,_16];
_26.1 = &_30.0;
_25 = _2;
_8 = [_27,_27,_16,_16,_16,_16,_16];
_12 = _7 + _7;
RET = core::ptr::addr_of!((*RET));
_15 = !_1;
_31 = !_10;
_7 = _9 * _12;
Call(_19 = fn10(_1, _7, _9, _3, _4, _10, _10, _3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23 = -_31;
_8 = [_16,_16,_16,_27,_16,_27,_27];
_26.0 = _26.2.1 >> _21;
_5 = _25 ^ _15;
_22 = [5471917524518310393_i64,(-4943790354868067238_i64),(-7438469472345587665_i64)];
_2 = -_4;
_30.0 = 7_usize >> _20;
_34 = 186323616695196840183978220444639055870_u128;
_35.fld1.0 = !_30.0;
_23 = _1 * _5;
_26.2.1 = _26.0 - _26.0;
_36 = _5 >> _21;
_18 = _36 as i16;
_8 = [_27,_27,_27,_27,_16,_27,_27];
_35.fld1.0 = _30.0;
RET = core::ptr::addr_of!((*RET));
match _34 {
0 => bb5,
1 => bb12,
186323616695196840183978220444639055870 => bb14,
_ => bb3
}
}
bb14 = {
_8 = [_16,_16,_27,_27,_16,_16,_27];
_26.2 = ((-168540575378815552480706238653240418484_i128), _26.0, (-688771416_i32));
(*RET) = core::ptr::addr_of_mut!((*_29));
_1 = _34 as isize;
_34 = 19506990487141429920470389391194644677_u128;
_35.fld2 = _2 as f64;
_36 = -_5;
_34 = _11 as u128;
_37 = _27;
_20 = _24 * _18;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(6_usize, 15_usize, Move(_15), 31_usize, Move(_31), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(6_usize, 10_usize, Move(_10), 24_usize, Move(_24), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(6_usize, 30_usize, Move(_30), 8_usize, Move(_8), 14_usize, Move(_14), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(6_usize, 18_usize, Move(_18), 17_usize, Move(_17), 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: [i16; 1];
let _18: f32;
let _19: isize;
let _20: ();
let _21: ();
{
_1 = _6;
_10 = -_11;
_10 = 115647040141480367937739764543278317744_i128 as isize;
_11 = -_15;
_11 = -_7;
RET = -_1;
_14 = !_9;
_6 = -_7;
RET = '\u{974c4}' as isize;
_16 = _15 >> _13;
_10 = _6;
_5 = _12 << _10;
_8 = -_13;
_4 = 133_u8 as isize;
_10 = _13 << _5;
RET = _11;
_13 = -_14;
_16 = !_11;
_11 = _5;
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(7_usize, 1_usize, Move(_1), 16_usize, Move(_16), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(7_usize, 3_usize, Move(_3), 13_usize, Move(_13), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize) -> f32 {
mir! {
type RET = f32;
let _3: (i128, (u8,));
let _4: *const [i8; 2];
let _5: [i128; 6];
let _6: u16;
let _7: [i64; 3];
let _8: Adt49;
let _9: (u32, &'static usize, (i128, u32, i32));
let _10: Adt52;
let _11: (u16, (i64, *const i16), [u128; 2], &'static usize, isize);
let _12: i64;
let _13: [u128; 2];
let _14: Adt59;
let _15: f64;
let _16: char;
let _17: i8;
let _18: i16;
let _19: u64;
let _20: ();
let _21: ();
{
RET = (-1844825642_i32) as f32;
_2 = _1;
RET = 208_u8 as f32;
_1 = _2;
_3.1.0 = !183_u8;
_3.0 = !(-152629594562859230470403407338698285536_i128);
_3.1 = (254_u8,);
_3.0 = 7745877955514171264346929532406009789_i128;
RET = 22583_i16 as f32;
_1 = RET as isize;
_3.0 = (-109650911061234435626830577114128149033_i128);
_1 = false as isize;
_1 = _2;
_7 = [7321553164211480972_i64,(-5209538618004227903_i64),7231177457713211579_i64];
_6 = 31395_u16 | 13396_u16;
_5 = [_3.0,_3.0,_3.0,_3.0,_3.0,_3.0];
RET = 3202771060_u32 as f32;
_3.1 = (161_u8,);
_1 = _2 << _2;
_3.1.0 = 72_u8;
_1 = _2 | _2;
_3.1.0 = 193_u8;
_2 = _1 + _1;
_7 = [(-1858662331787016636_i64),2347037089624012806_i64,(-3805197085782236274_i64)];
Goto(bb1)
}
bb1 = {
_9.2.0 = !_3.0;
match _3.1.0 {
193 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_2 = !_1;
RET = _6 as f32;
_11.4 = _1 << _2;
_2 = _11.4;
_9.2.0 = !_3.0;
match _3.1.0 {
0 => bb4,
1 => bb5,
193 => bb7,
_ => bb6
}
}
bb4 = {
Return()
}
bb5 = {
_9.2.0 = !_3.0;
match _3.1.0 {
193 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
_9.2.1 = 9147_i16 as u32;
_7 = [307674580410153151_i64,4701729902725307426_i64,(-1530437468994288541_i64)];
_12 = _6 as i64;
_9.2.0 = _3.0;
_13 = [166049657040268581548833173878985931064_u128,302017243904165921029482569875461271225_u128];
_7 = [_12,_12,_12];
RET = _3.0 as f32;
_14.fld5 = [_3.0,_3.0,_9.2.0,_9.2.0,_3.0,_3.0];
_14.fld7.fld0 = false;
_15 = _11.4 as f64;
_9.2.1 = RET as u32;
Goto(bb8)
}
bb8 = {
_11.4 = !_2;
_14.fld3 = ['\u{9c1cf}','\u{b8b3c}','\u{dd580}','\u{59337}','\u{1bd63}','\u{13efe}','\u{de3c9}'];
_3.1.0 = !227_u8;
_2 = 163452542513920663152881921568150442796_u128 as isize;
_6 = (-592021434_i32) as u16;
_14.fld1 = (733506464238916007_usize, _14.fld5);
_3.1 = (89_u8,);
_11.4 = _1 | _1;
_7 = [_12,_12,_12];
_7 = [_12,_12,_12];
_9.2.1 = (-297023385_i32) as u32;
match _14.fld1.0 {
0 => bb6,
1 => bb3,
2 => bb9,
733506464238916007 => bb11,
_ => bb10
}
}
bb9 = {
_9.2.0 = !_3.0;
match _3.1.0 {
193 => bb3,
_ => bb2
}
}
bb10 = {
Return()
}
bb11 = {
_11.1.0 = RET as i64;
_17 = -(-107_i8);
_14.fld0 = _14.fld1.0;
_14.fld6 = !_12;
_11.3 = &_14.fld1.0;
_2 = _12 as isize;
RET = (-657317189_i32) as f32;
_7 = [_14.fld6,_14.fld6,_11.1.0];
_14.fld2 = -_15;
_9.1 = Move(_11.3);
_9.2 = (_3.0, 3660219471_u32, (-905999991_i32));
_9.0 = _14.fld0 as u32;
_11.4 = _1 ^ _1;
RET = _14.fld0 as f32;
_18 = 31529_i16 + (-8545_i16);
_7 = [_12,_12,_12];
Goto(bb12)
}
bb12 = {
_9.2.2 = !1730556918_i32;
_11.2 = [182279808431557188365138200594424479787_u128,108786371169778936350317033827157157874_u128];
_7 = [_14.fld6,_14.fld6,_12];
RET = _14.fld2 as f32;
Goto(bb13)
}
bb13 = {
Call(_20 = dump_var(8_usize, 18_usize, Move(_18), 2_usize, Move(_2), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_20 = dump_var(8_usize, 13_usize, Move(_13), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i16,mut _2: isize,mut _3: i16,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: i16) -> isize {
mir! {
type RET = isize;
let _9: u64;
let _10: ();
let _11: ();
{
RET = _7 ^ _2;
_1 = _3;
_5 = !_2;
_3 = false as i16;
_7 = !_4;
_6 = _2 | RET;
_1 = -_8;
_4 = _6;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(9_usize, 4_usize, Move(_4), 1_usize, Move(_1), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: f32,mut _3: f32,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _9: [i32; 7];
let _10: (usize, [i128; 6]);
let _11: Adt56;
let _12: bool;
let _13: u32;
let _14: isize;
let _15: i32;
let _16: [u128; 2];
let _17: isize;
let _18: f64;
let _19: Adt59;
let _20: Adt58;
let _21: ();
let _22: ();
{
_2 = _3;
_9 = [1654639087_i32,384096369_i32,(-1278050131_i32),(-629222733_i32),(-1671247991_i32),236376182_i32,1064109591_i32];
_8 = (-21972_i16) as isize;
_10.0 = 16711654443471911613_usize;
_9 = [190079700_i32,353845717_i32,(-304624275_i32),830168810_i32,(-1763579678_i32),(-2096563170_i32),1385428904_i32];
RET = [24874_i16];
_1 = _4 << _5;
_3 = _2;
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
16711654443471911613 => bb9,
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
_6 = _4 ^ _5;
_3 = _2;
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb5,
5 => bb10,
6 => bb11,
16711654443471911613 => bb13,
_ => bb12
}
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
_7 = -_4;
_14 = _7 | _7;
_2 = _3 + _3;
_12 = _6 == _4;
_3 = _2 + _2;
RET = [28085_i16];
_10.1 = [102261328141502324223544446803661977849_i128,33072681177880784257493505008992320432_i128,153075150175238646864230259057259944810_i128,54058613651709961529725280723873880157_i128,(-80677757342991996200374654595191003030_i128),(-116411318155767893784874200924308381179_i128)];
_8 = _14 & _6;
_3 = (-282367069_i32) as f32;
_10.0 = 14557112459570865013_usize - 12457306515962121085_usize;
_17 = (-1217981858_i32) as isize;
_15 = -620419522_i32;
_2 = 1018416651_u32 as f32;
Call(_17 = fn11(_4, _8, _7, _5, _4, _6, _14, _8, _12, _6, _4, _7, _14), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_16 = [97622419667615213329158146806381298599_u128,40940112053270588384502265896195130163_u128];
_13 = !4083808777_u32;
_1 = _8;
_14 = _1 - _1;
_3 = _2;
_5 = _6;
RET = [26458_i16];
_16 = [13535888420917361046542940601984620341_u128,46761535848755234236857163392661323138_u128];
_10.1 = [120154720157711223520686715116249248895_i128,157333380141946980264587274510931306541_i128,(-48255538102337024447975000216372523876_i128),5969366728006419024184275274067223377_i128,(-126997702445226513109811235719808334129_i128),(-88412857850577710644139392392216992459_i128)];
_4 = _8 - _14;
_8 = (-16327218412454971819677858041441121223_i128) as isize;
_16 = [92846561811095848731149808005281706473_u128,47051437352249514219846638451139304082_u128];
RET = [18805_i16];
_6 = _1 ^ _17;
_12 = _7 <= _14;
_10.0 = !4_usize;
_7 = _5 + _14;
_18 = 10047102212824201901643098339301535180_i128 as f64;
_19.fld7.fld0 = _12;
_4 = !_5;
_19.fld7 = Adt53 { fld0: _12 };
_19.fld1 = _10;
_19.fld2 = _15 as f64;
_16 = [154320483514150940958171126755166460056_u128,113275442380173989774902194776290458960_u128];
_18 = _19.fld2;
RET = [3622_i16];
_19.fld0 = 96_u8 as usize;
_19.fld5 = [140002827450738341073922668560826589852_i128,24243167411437254067764973409001369017_i128,(-120730744578248148388939237888996983587_i128),152764458165764958643443632360174090727_i128,36636423355805915482371256356291112858_i128,(-63989159920071433269143681309649560791_i128)];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(10_usize, 1_usize, Move(_1), 6_usize, Move(_6), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(10_usize, 10_usize, Move(_10), 16_usize, Move(_16), 17_usize, Move(_17), 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: bool,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: [u128; 2];
let _15: Adt53;
let _16: isize;
let _17: Adt53;
let _18: ();
let _19: ();
{
_1 = _9 as isize;
Goto(bb1)
}
bb1 = {
_15 = Adt53 { fld0: _9 };
_11 = _5 ^ _7;
_7 = (-45_i8) as isize;
_11 = _12 & _10;
RET = 10492401372377634166_u64 as isize;
RET = _5;
_2 = 11_u8 as isize;
_14 = [92165771235567730275163004718380220761_u128,310728344544631220153931218119794921502_u128];
_2 = RET;
_1 = '\u{fd33c}' as isize;
RET = _10;
_16 = _11 * _2;
_1 = 24_i8 as isize;
RET = -_13;
_6 = 3527_i16 as isize;
_13 = _10 + RET;
_9 = _15.fld0;
_7 = !_10;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(11_usize, 14_usize, Move(_14), 12_usize, Move(_12), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(11_usize, 6_usize, Move(_6), 10_usize, Move(_10), 8_usize, Move(_8), 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _5: f32;
let _6: u128;
let _7: [char; 7];
let _8: [u32; 6];
let _9: (i128, (u8,));
let _10: *mut [i64; 3];
let _11: bool;
let _12: u32;
let _13: [i128; 6];
let _14: i64;
let _15: isize;
let _16: bool;
let _17: *mut u64;
let _18: [u128; 2];
let _19: [i16; 1];
let _20: [u128; 2];
let _21: (i128, u32, i32);
let _22: f64;
let _23: f32;
let _24: Adt49;
let _25: f64;
let _26: ();
let _27: ();
{
RET = [149782565239779776774738750760816016358_i128,(-113664533572780690919545802729862028947_i128),16635846044993623333947163612068635421_i128,(-168837849134292712323800800430967726820_i128),(-9351872620855016067896826810134908853_i128),(-137865073343680476687308875468165363631_i128)];
RET = [86689515816830511353322962777508300746_i128,(-47425680733297238944576212714740558510_i128),104158773986282581203060974019736333374_i128,96378034148953637797607310650536763536_i128,51715651111155022125973543152883026101_i128,9122346413341111960344209077658112076_i128];
_1 = _4;
_3 = _1;
RET = [(-94837029348598098788666794010561449645_i128),92495198235703832733709026619737824084_i128,(-46742107689029970635677597371660007821_i128),(-6528282671961826564539559529440399399_i128),(-139840203611414139995192670995352681269_i128),(-99813534585486083182338769304240889093_i128)];
RET = [(-24914704748093165672132374542357421975_i128),(-2993543168895372734447093605664198660_i128),34644801697594287267771633412950804418_i128,(-23341499608324430478919814916413472140_i128),102021753965479861344030451338561798939_i128,(-24101517113338015977187220479830065165_i128)];
_2 = !_4;
_2 = !_4;
_1 = -_2;
_5 = 37_i8 as f32;
_2 = true as isize;
RET = [(-155440890182665804708223200670923355269_i128),(-63842795987893581592514379585237014374_i128),81743050801835891713601999323241789917_i128,39469071287234120304028744738967196941_i128,(-67512600528362060987358659601263959135_i128),136164535340676468194257695952545540943_i128];
_1 = _3 << _4;
_1 = -_3;
RET = [144314259525679485995403237154693357937_i128,(-14993256265608531920656976059530774745_i128),(-9312538776924582761227487236330136927_i128),148046274948914917369969780702227128656_i128,26243884942199293370150053380843328391_i128,90747720649068546911990448337140539545_i128];
RET = [(-78225978810368697445227730174928228956_i128),21172037110471497932807641984500857531_i128,(-94992922686902100285517043485813691488_i128),120001832297220526111350839474254381197_i128,(-142339730987864369077562990102729052063_i128),49446992147074798150776356770604250615_i128];
Goto(bb1)
}
bb1 = {
_7 = ['\u{aa5fb}','\u{affa1}','\u{d686b}','\u{81347}','\u{9b26e}','\u{e3bab}','\u{8828a}'];
_1 = 8119309415939251862_u64 as isize;
_5 = (-4557446282364707289328135597364937346_i128) as f32;
RET = [139815803672226820673969141605845177562_i128,56659603839441048996939925833199185267_i128,53686669818269613406412993349502498615_i128,134236337320525584611869444681226185098_i128,96394986698294537991790122095084463669_i128,107478282545851013480657608327002194297_i128];
_9.0 = 129057292260594893417431049851971832901_i128;
_9.0 = false as i128;
_3 = 4032833085_u32 as isize;
_6 = !93632456910583614861209662906360466222_u128;
_5 = _9.0 as f32;
_3 = (-27483_i16) as isize;
_1 = _4 ^ _4;
_4 = _1;
_9.0 = !(-36735504481274795837820904757339096578_i128);
_8 = [2359842701_u32,1362100952_u32,1301797258_u32,1236296039_u32,1397620856_u32,916797379_u32];
_9.0 = (-6962492043100342177559042435586619091_i128);
_4 = true as isize;
_4 = (-1419772550_i32) as isize;
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
Goto(bb2)
}
bb2 = {
_11 = !true;
_3 = -_1;
_12 = 176576301_u32 ^ 2921152438_u32;
_7 = ['\u{e88a0}','\u{b154c}','\u{e48a2}','\u{9b983}','\u{8a34a}','\u{dba51}','\u{f6287}'];
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_5 = 13223069080742827761_u64 as f32;
_12 = 859469944_u32 >> _3;
_6 = 52_i8 as u128;
_3 = -_1;
_9.1 = (115_u8,);
match _9.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
333319874877838121285815564996181592365 => bb7,
_ => bb6
}
}
bb3 = {
_7 = ['\u{aa5fb}','\u{affa1}','\u{d686b}','\u{81347}','\u{9b26e}','\u{e3bab}','\u{8828a}'];
_1 = 8119309415939251862_u64 as isize;
_5 = (-4557446282364707289328135597364937346_i128) as f32;
RET = [139815803672226820673969141605845177562_i128,56659603839441048996939925833199185267_i128,53686669818269613406412993349502498615_i128,134236337320525584611869444681226185098_i128,96394986698294537991790122095084463669_i128,107478282545851013480657608327002194297_i128];
_9.0 = 129057292260594893417431049851971832901_i128;
_9.0 = false as i128;
_3 = 4032833085_u32 as isize;
_6 = !93632456910583614861209662906360466222_u128;
_5 = _9.0 as f32;
_3 = (-27483_i16) as isize;
_1 = _4 ^ _4;
_4 = _1;
_9.0 = !(-36735504481274795837820904757339096578_i128);
_8 = [2359842701_u32,1362100952_u32,1301797258_u32,1236296039_u32,1397620856_u32,916797379_u32];
_9.0 = (-6962492043100342177559042435586619091_i128);
_4 = true as isize;
_4 = (-1419772550_i32) as isize;
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
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
_9.0 = 1740506705790580310733919492632705210_i128;
_9.1.0 = !81_u8;
_3 = -_1;
_13 = RET;
_9.0 = -62128218825317567876792249513480623828_i128;
_4 = _3;
_3 = -_1;
_8 = [_12,_12,_12,_12,_12,_12];
_8 = [_12,_12,_12,_12,_12,_12];
_16 = _11;
_7 = ['\u{82f80}','\u{8fefd}','\u{f582}','\u{76fa}','\u{e8c79}','\u{e34bd}','\u{2e9bc}'];
_11 = _1 >= _3;
_16 = _11 ^ _11;
_12 = !1246905061_u32;
_9.1.0 = 23_u8;
Goto(bb8)
}
bb8 = {
_8 = [_12,_12,_12,_12,_12,_12];
_15 = _1 * _1;
match _9.1.0 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
23 => bb10,
_ => bb9
}
}
bb9 = {
_7 = ['\u{aa5fb}','\u{affa1}','\u{d686b}','\u{81347}','\u{9b26e}','\u{e3bab}','\u{8828a}'];
_1 = 8119309415939251862_u64 as isize;
_5 = (-4557446282364707289328135597364937346_i128) as f32;
RET = [139815803672226820673969141605845177562_i128,56659603839441048996939925833199185267_i128,53686669818269613406412993349502498615_i128,134236337320525584611869444681226185098_i128,96394986698294537991790122095084463669_i128,107478282545851013480657608327002194297_i128];
_9.0 = 129057292260594893417431049851971832901_i128;
_9.0 = false as i128;
_3 = 4032833085_u32 as isize;
_6 = !93632456910583614861209662906360466222_u128;
_5 = _9.0 as f32;
_3 = (-27483_i16) as isize;
_1 = _4 ^ _4;
_4 = _1;
_9.0 = !(-36735504481274795837820904757339096578_i128);
_8 = [2359842701_u32,1362100952_u32,1301797258_u32,1236296039_u32,1397620856_u32,916797379_u32];
_9.0 = (-6962492043100342177559042435586619091_i128);
_4 = true as isize;
_4 = (-1419772550_i32) as isize;
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
Goto(bb2)
}
bb10 = {
_18 = [_6,_6];
_4 = !_3;
_7 = ['\u{e53e}','\u{f64d3}','\u{e92cb}','\u{762ea}','\u{4e374}','\u{c355b}','\u{df4b3}'];
_4 = !_1;
_3 = _4;
_9.0 = (-40_i8) as i128;
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_13 = RET;
_1 = -_3;
_2 = !_4;
_12 = !211501452_u32;
_15 = !_2;
_14 = !644963198699177143_i64;
_8 = [_12,_12,_12,_12,_12,_12];
_19 = [22844_i16];
_19 = [12401_i16];
_6 = 234949386889191013999076516777191780887_u128 << _1;
_11 = !_16;
_14 = !398495074084236382_i64;
match _9.1.0 {
0 => bb1,
1 => bb11,
2 => bb12,
3 => bb13,
23 => bb15,
_ => bb14
}
}
bb11 = {
_11 = !true;
_3 = -_1;
_12 = 176576301_u32 ^ 2921152438_u32;
_7 = ['\u{e88a0}','\u{b154c}','\u{e48a2}','\u{9b983}','\u{8a34a}','\u{dba51}','\u{f6287}'];
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_5 = 13223069080742827761_u64 as f32;
_12 = 859469944_u32 >> _3;
_6 = 52_i8 as u128;
_3 = -_1;
_9.1 = (115_u8,);
match _9.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
333319874877838121285815564996181592365 => bb7,
_ => bb6
}
}
bb12 = {
_8 = [_12,_12,_12,_12,_12,_12];
_15 = _1 * _1;
match _9.1.0 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
23 => bb10,
_ => bb9
}
}
bb13 = {
_9.0 = 1740506705790580310733919492632705210_i128;
_9.1.0 = !81_u8;
_3 = -_1;
_13 = RET;
_9.0 = -62128218825317567876792249513480623828_i128;
_4 = _3;
_3 = -_1;
_8 = [_12,_12,_12,_12,_12,_12];
_8 = [_12,_12,_12,_12,_12,_12];
_16 = _11;
_7 = ['\u{82f80}','\u{8fefd}','\u{f582}','\u{76fa}','\u{e8c79}','\u{e34bd}','\u{2e9bc}'];
_11 = _1 >= _3;
_16 = _11 ^ _11;
_12 = !1246905061_u32;
_9.1.0 = 23_u8;
Goto(bb8)
}
bb14 = {
_7 = ['\u{aa5fb}','\u{affa1}','\u{d686b}','\u{81347}','\u{9b26e}','\u{e3bab}','\u{8828a}'];
_1 = 8119309415939251862_u64 as isize;
_5 = (-4557446282364707289328135597364937346_i128) as f32;
RET = [139815803672226820673969141605845177562_i128,56659603839441048996939925833199185267_i128,53686669818269613406412993349502498615_i128,134236337320525584611869444681226185098_i128,96394986698294537991790122095084463669_i128,107478282545851013480657608327002194297_i128];
_9.0 = 129057292260594893417431049851971832901_i128;
_9.0 = false as i128;
_3 = 4032833085_u32 as isize;
_6 = !93632456910583614861209662906360466222_u128;
_5 = _9.0 as f32;
_3 = (-27483_i16) as isize;
_1 = _4 ^ _4;
_4 = _1;
_9.0 = !(-36735504481274795837820904757339096578_i128);
_8 = [2359842701_u32,1362100952_u32,1301797258_u32,1236296039_u32,1397620856_u32,916797379_u32];
_9.0 = (-6962492043100342177559042435586619091_i128);
_4 = true as isize;
_4 = (-1419772550_i32) as isize;
RET = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
Goto(bb2)
}
bb15 = {
_22 = _9.0 as f64;
_15 = _14 as isize;
_3 = !_1;
_21.1 = _12;
_7 = ['\u{84b}','\u{1c8a5}','\u{a551}','\u{109ff4}','\u{10ae47}','\u{342fe}','\u{c2fa0}'];
_15 = (-1859856783_i32) as isize;
_3 = _1 - _4;
_15 = _22 as isize;
_5 = 2037188397_i32 as f32;
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(12_usize, 14_usize, Move(_14), 8_usize, Move(_8), 4_usize, Move(_4), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(12_usize, 13_usize, Move(_13), 16_usize, Move(_16), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: [u128; 2],mut _5: usize,mut _6: (u128, [i8; 2]),mut _7: isize,mut _8: isize,mut _9: usize,mut _10: (usize, [i128; 6]),mut _11: isize,mut _12: i64,mut _13: u64) -> [char; 2] {
mir! {
type RET = [char; 2];
let _14: [i128; 6];
let _15: bool;
let _16: Adt45;
let _17: (i128, u32, i32);
let _18: [i32; 7];
let _19: usize;
let _20: u128;
let _21: (u8,);
let _22: isize;
let _23: f32;
let _24: (u128, [i8; 2]);
let _25: [u128; 2];
let _26: [char; 2];
let _27: Adt47;
let _28: (u8,);
let _29: char;
let _30: u8;
let _31: isize;
let _32: isize;
let _33: isize;
let _34: ();
let _35: ();
{
_11 = _2;
_10.0 = _5 >> _9;
_7 = _11 + _8;
_15 = !_1;
_2 = _7;
_11 = _3;
_11 = _7 << _8;
_10.0 = !_5;
_12 = !(-1240378699932358893_i64);
_7 = _2 - _3;
_2 = -_11;
_3 = !_7;
_10.0 = '\u{15b5c}' as usize;
_16.fld2 = ['\u{2c443}','\u{2e58}'];
_9 = !_5;
_9 = _5 - _5;
_16.fld3 = (-940890786_i32) as i8;
_16.fld4 = 29_u8 as f64;
_1 = !_15;
_17.0 = _13 as i128;
_9 = _5 * _5;
RET = _16.fld2;
_11 = -_7;
Call(_15 = fn14(_11, _1, _6, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_19 = !_9;
_15 = _1;
_17 = ((-157000748861167334661418955459015370084_i128), 1916849061_u32, 931058863_i32);
_7 = _2 * _11;
_8 = _5 as isize;
_19 = _12 as usize;
_1 = _15;
_15 = !_1;
Goto(bb2)
}
bb2 = {
_18 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_20 = !_6.0;
_16.fld1 = _17.2 as u128;
_4 = [_16.fld1,_6.0];
_14 = [_17.0,_17.0,_17.0,_17.0,_17.0,_17.0];
_21 = (215_u8,);
_4 = [_6.0,_20];
_6.0 = _16.fld1 & _20;
_3 = _7 & _11;
_10.1 = _14;
_21 = (113_u8,);
_23 = 12031_i16 as f32;
_2 = !_11;
_9 = _5 & _5;
_16.fld1 = _6.0 << _5;
_22 = _12 as isize;
RET = ['\u{102ce7}','\u{bd9}'];
_10.0 = _9 >> _20;
_26 = ['\u{1d5c9}','\u{2864f}'];
_14 = _10.1;
RET = ['\u{7470f}','\u{3e3e4}'];
_11 = _8 + _2;
match _17.2 {
0 => bb3,
1 => bb4,
2 => bb5,
931058863 => bb7,
_ => bb6
}
}
bb3 = {
_19 = !_9;
_15 = _1;
_17 = ((-157000748861167334661418955459015370084_i128), 1916849061_u32, 931058863_i32);
_7 = _2 * _11;
_8 = _5 as isize;
_19 = _12 as usize;
_1 = _15;
_15 = !_1;
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
_8 = _7;
_26 = ['\u{e55dd}','\u{8333b}'];
_16.fld3 = -78_i8;
_18 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_6.0 = !_16.fld1;
_10.0 = _15 as usize;
_1 = !_15;
_24.0 = _6.0;
_11 = _16.fld4 as isize;
_29 = '\u{b4706}';
_9 = _10.0;
Goto(bb8)
}
bb8 = {
_21 = (80_u8,);
_6.0 = _9 as u128;
_18 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_6.1 = [_16.fld3,_16.fld3];
match _17.2 {
0 => bb5,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
931058863 => bb16,
_ => bb15
}
}
bb9 = {
_8 = _7;
_26 = ['\u{e55dd}','\u{8333b}'];
_16.fld3 = -78_i8;
_18 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_6.0 = !_16.fld1;
_10.0 = _15 as usize;
_1 = !_15;
_24.0 = _6.0;
_11 = _16.fld4 as isize;
_29 = '\u{b4706}';
_9 = _10.0;
Goto(bb8)
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
_19 = !_9;
_15 = _1;
_17 = ((-157000748861167334661418955459015370084_i128), 1916849061_u32, 931058863_i32);
_7 = _2 * _11;
_8 = _5 as isize;
_19 = _12 as usize;
_1 = _15;
_15 = !_1;
Goto(bb2)
}
bb14 = {
_18 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_20 = !_6.0;
_16.fld1 = _17.2 as u128;
_4 = [_16.fld1,_6.0];
_14 = [_17.0,_17.0,_17.0,_17.0,_17.0,_17.0];
_21 = (215_u8,);
_4 = [_6.0,_20];
_6.0 = _16.fld1 & _20;
_3 = _7 & _11;
_10.1 = _14;
_21 = (113_u8,);
_23 = 12031_i16 as f32;
_2 = !_11;
_9 = _5 & _5;
_16.fld1 = _6.0 << _5;
_22 = _12 as isize;
RET = ['\u{102ce7}','\u{bd9}'];
_10.0 = _9 >> _20;
_26 = ['\u{1d5c9}','\u{2864f}'];
_14 = _10.1;
RET = ['\u{7470f}','\u{3e3e4}'];
_11 = _8 + _2;
match _17.2 {
0 => bb3,
1 => bb4,
2 => bb5,
931058863 => bb7,
_ => bb6
}
}
bb15 = {
_19 = !_9;
_15 = _1;
_17 = ((-157000748861167334661418955459015370084_i128), 1916849061_u32, 931058863_i32);
_7 = _2 * _11;
_8 = _5 as isize;
_19 = _12 as usize;
_1 = _15;
_15 = !_1;
Goto(bb2)
}
bb16 = {
_4 = [_24.0,_6.0];
_24.1 = [_16.fld3,_16.fld3];
_15 = !_1;
_10.1 = _14;
_23 = 65335_u16 as f32;
_32 = _3 * _2;
_17 = ((-134057491914581115661709269349750825662_i128), 3496530342_u32, 1456219729_i32);
_24.0 = _16.fld1 | _6.0;
_17.1 = 250581824_u32 << _3;
_22 = _16.fld4 as isize;
_16.fld3 = 53_i8 + 21_i8;
RET = [_29,_29];
_24.1 = _6.1;
_17.0 = (-82529732082521877773009247883702754622_i128);
_6.1 = [_16.fld3,_16.fld3];
_25 = [_6.0,_24.0];
_19 = !_5;
_29 = '\u{602d3}';
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(13_usize, 4_usize, Move(_4), 24_usize, Move(_24), 15_usize, Move(_15), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(13_usize, 14_usize, Move(_14), 20_usize, Move(_20), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(13_usize, 19_usize, Move(_19), 32_usize, Move(_32), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(13_usize, 25_usize, Move(_25), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: bool,mut _3: (u128, [i8; 2]),mut _4: u64) -> bool {
mir! {
type RET = bool;
let _5: Adt49;
let _6: i32;
let _7: char;
let _8: [u128; 2];
let _9: ();
let _10: ();
{
_1 = (-31212_i16) as isize;
Goto(bb1)
}
bb1 = {
RET = !_2;
_3.1 = [85_i8,16_i8];
RET = _3.0 <= _3.0;
_3.1 = [40_i8,(-28_i8)];
_2 = RET < RET;
RET = _2;
_6 = !1660990291_i32;
_3.0 = 52468691134811938613816430774577360500_u128;
_7 = '\u{d3c10}';
_8 = [_3.0,_3.0];
Goto(bb2)
}
bb2 = {
Call(_9 = dump_var(14_usize, 6_usize, Move(_6), 8_usize, Move(_8), 1_usize, Move(_1), 10_usize, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: usize,mut _2: [u32; 6],mut _3: bool,mut _4: isize,mut _5: u128,mut _6: u128,mut _7: u128) -> (i64, *const i16) {
mir! {
type RET = (i64, *const i16);
let _8: (u8,);
let _9: char;
let _10: [u128; 2];
let _11: Adt52;
let _12: Adt57;
let _13: (u8, i8, [i64; 3], usize);
let _14: (i64, *const i16);
let _15: Adt44;
let _16: u64;
let _17: f64;
let _18: [i32; 7];
let _19: [i8; 2];
let _20: f32;
let _21: bool;
let _22: bool;
let _23: f64;
let _24: i32;
let _25: Adt44;
let _26: [char; 7];
let _27: Adt51;
let _28: u64;
let _29: [u32; 6];
let _30: [char; 7];
let _31: [i8; 2];
let _32: u8;
let _33: u128;
let _34: [i128; 6];
let _35: f32;
let _36: u16;
let _37: u128;
let _38: isize;
let _39: (u128, [i8; 2]);
let _40: Adt53;
let _41: [u32; 6];
let _42: char;
let _43: isize;
let _44: usize;
let _45: isize;
let _46: char;
let _47: [i16; 1];
let _48: i32;
let _49: isize;
let _50: isize;
let _51: Adt53;
let _52: Adt47;
let _53: i32;
let _54: bool;
let _55: f32;
let _56: Adt44;
let _57: Adt55;
let _58: *const *mut [i64; 3];
let _59: bool;
let _60: f32;
let _61: u32;
let _62: bool;
let _63: f32;
let _64: ();
let _65: ();
{
_2 = [4070780413_u32,4084672998_u32,64908558_u32,1554374205_u32,4161308637_u32,445987045_u32];
_1 = 120_i8 as usize;
_6 = _5 * _5;
RET.0 = (-605468333604361889_i64) << _5;
_4 = !(-20_isize);
_5 = _6;
_9 = '\u{5d1a}';
_8 = (154_u8,);
RET.0 = !380253844158355817_i64;
_4 = (-123_isize) * (-9223372036854775808_isize);
_10 = [_7,_7];
_5 = _6;
_1 = 6582356552945026770_usize - 0_usize;
RET.0 = (-6171935165685226725_i64) >> _6;
_10 = [_6,_5];
_3 = false;
_5 = !_6;
RET.0 = (-2310746790715710764_i64) << _6;
_1 = 404502513096133979_usize;
_3 = _6 >= _6;
_6 = !_5;
_6 = _7;
Goto(bb1)
}
bb1 = {
RET.0 = (-5621946425284504993_i64) + (-6688463439496559233_i64);
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _3 as isize;
_2 = [796682007_u32,1061111070_u32,3178946399_u32,1457300057_u32,497744363_u32,2960483871_u32];
_2 = [2437976948_u32,1538569860_u32,1189704369_u32,1437771057_u32,2374264537_u32,1699424733_u32];
_5 = _6;
_9 = '\u{195ed}';
_13.1 = 112_i8;
RET.0 = (-8278396065065697399_i64) ^ (-3792862411307624832_i64);
_12.fld3 = [_5,_5];
_13.3 = 1726137101_i32 as usize;
_9 = '\u{15491}';
_13.2 = [RET.0,RET.0,RET.0];
_13.0 = _8.0;
_9 = '\u{fb760}';
_7 = _6 & _6;
_12.fld2 = 91894297123272157807951766934365034702_i128 - (-165187048557035368893065022704679625146_i128);
_8 = (_13.0,);
_13.0 = _8.0 % _8.0;
_13.2 = [RET.0,RET.0,RET.0];
_14.0 = 979969351_i32 as i64;
_7 = _5 - _5;
_14.0 = _8.0 as i64;
_14.0 = RET.0;
_6 = _7 | _7;
_12.fld1 = !1229119735_u32;
match _8.0 {
0 => bb3,
1 => bb4,
2 => bb5,
154 => bb7,
_ => bb6
}
}
bb3 = {
RET.0 = (-5621946425284504993_i64) + (-6688463439496559233_i64);
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
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
_13.0 = !_8.0;
_10 = [_6,_6];
Goto(bb8)
}
bb8 = {
_4 = 2053995641_i32 as isize;
_9 = '\u{b9bc4}';
_18 = [(-689032082_i32),(-1588877963_i32),1650518850_i32,2147202885_i32,1015411471_i32,494016265_i32,1439147641_i32];
_7 = _6 + _5;
RET.0 = _14.0 - _14.0;
_12.fld2 = 38348369504779303691626532949715342497_i128;
RET.0 = _14.0 ^ _14.0;
_8 = (_13.0,);
_19 = [_13.1,_13.1];
RET.0 = _14.0;
_19 = [_13.1,_13.1];
RET.0 = _14.0 * _14.0;
_22 = _3 ^ _3;
_10 = _12.fld3;
match _13.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb9,
112 => bb11,
_ => bb10
}
}
bb9 = {
_13.0 = !_8.0;
_10 = [_6,_6];
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_8.0 = !_13.0;
_8.0 = !_13.0;
_9 = '\u{76919}';
_3 = !_22;
_13.2 = [RET.0,RET.0,_14.0];
_2 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_20 = _13.3 as f32;
_9 = '\u{1f382}';
_3 = !_22;
_17 = _1 as f64;
_13.2 = [RET.0,RET.0,RET.0];
_16 = 3437604151981363747_u64 & 13060604948316288667_u64;
_13.0 = _8.0;
_4 = -(-9223372036854775808_isize);
_18 = [290373202_i32,(-1918745774_i32),(-1075727638_i32),(-836513116_i32),605759685_i32,(-249784048_i32),(-337859893_i32)];
RET.0 = _14.0 << _6;
_12.fld2 = -96329072534457389816672214719587424056_i128;
_6 = _5;
_8.0 = _13.3 as u8;
_14.0 = RET.0;
_12.fld3 = _10;
Goto(bb12)
}
bb12 = {
_8.0 = _13.0;
_22 = !_3;
_23 = _17 - _17;
_13.2 = [_14.0,_14.0,_14.0];
_7 = _9 as u128;
RET.0 = !_14.0;
_14.0 = RET.0 + RET.0;
_10 = [_6,_6];
RET.0 = !_14.0;
_15 = Adt44::Variant1 { fld0: _9 };
_9 = Field::<char>(Variant(_15, 1), 0);
RET.0 = !_14.0;
place!(Field::<char>(Variant(_15, 1), 0)) = _9;
SetDiscriminant(_15, 1);
_1 = _13.3;
_16 = (-21152_i16) as u64;
_14.0 = RET.0 * RET.0;
RET.0 = _13.3 as i64;
RET.0 = _12.fld1 as i64;
_13.3 = _1;
_13.0 = _8.0 + _8.0;
_25 = Adt44::Variant1 { fld0: _9 };
RET.0 = _14.0 ^ _14.0;
_12.fld3 = [_5,_5];
_25 = Adt44::Variant1 { fld0: _9 };
_7 = _6 | _5;
place!(Field::<char>(Variant(_15, 1), 0)) = Field::<char>(Variant(_25, 1), 0);
Goto(bb13)
}
bb13 = {
_13.2 = [RET.0,_14.0,RET.0];
_17 = _23 + _23;
_24 = _9 as i32;
_14.0 = RET.0 >> RET.0;
_24 = !(-1763433145_i32);
_2 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_13.3 = _12.fld1 as usize;
_28 = _16 ^ _16;
place!(Field::<char>(Variant(_25, 1), 0)) = Field::<char>(Variant(_15, 1), 0);
_18 = [_24,_24,_24,_24,_24,_24,_24];
_14.0 = RET.0 * RET.0;
_4 = _28 as isize;
_22 = !_3;
_15 = Move(_25);
_31 = [_13.1,_13.1];
Call(_18 = fn16(_22, _12.fld3, _7, _7, _13, _13.2, _13.2, _6, _7, _6), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_6 = _5 * _5;
_3 = _6 >= _7;
_8.0 = _13.0 >> _14.0;
_22 = !_3;
_33 = _5 | _5;
_8 = (_13.0,);
_25 = Move(_15);
_12.fld2 = _4 as i128;
_17 = _23 + _23;
_15 = Adt44::Variant1 { fld0: _9 };
_13.2 = [RET.0,_14.0,RET.0];
_5 = _6 - _33;
_7 = !_5;
SetDiscriminant(_25, 1);
_32 = _13.0;
match _13.1 {
0 => bb6,
1 => bb2,
2 => bb3,
112 => bb15,
_ => bb8
}
}
bb15 = {
_17 = -_23;
_24 = (-708046018_i32) ^ (-1545215407_i32);
_24 = 818254882_i32 * 504383356_i32;
_8.0 = _17 as u8;
SetDiscriminant(_15, 1);
_33 = _7 | _7;
_22 = _3;
_19 = [_13.1,_13.1];
_18 = [_24,_24,_24,_24,_24,_24,_24];
_13.2 = [RET.0,_14.0,_14.0];
_13.0 = !_8.0;
_21 = !_22;
_13.1 = 12236_u16 as i8;
_19 = [_13.1,_13.1];
_30 = [_9,_9,_9,_9,_9,_9,_9];
_35 = _20 + _20;
Goto(bb16)
}
bb16 = {
_3 = _22;
_9 = '\u{a0608}';
_12.fld1 = _9 as u32;
_30 = [_9,_9,_9,_9,_9,_9,_9];
Call(_33 = fn19(_14.0, _5, _5, _12.fld3, _13.2, _21, _5, _12.fld2, _7, _3, _14.0, _22, _22, _7), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_15 = Adt44::Variant1 { fld0: _9 };
_21 = _3;
_17 = _23;
RET.0 = _14.0 >> _6;
_31 = [_13.1,_13.1];
_31 = [_13.1,_13.1];
_14.0 = RET.0;
_5 = _6 << RET.0;
_28 = !_16;
_12.fld3 = [_5,_6];
_12.fld1 = 1042832600_u32;
_24 = _20 as i32;
_14.0 = -RET.0;
_34 = [_12.fld2,_12.fld2,_12.fld2,_12.fld2,_12.fld2,_12.fld2];
_22 = !_3;
_19 = [_13.1,_13.1];
_10 = [_7,_5];
_28 = !_16;
_30 = [_9,_9,Field::<char>(Variant(_15, 1), 0),_9,_9,_9,Field::<char>(Variant(_15, 1), 0)];
SetDiscriminant(_15, 0);
_36 = _9 as u16;
_15 = Adt44::Variant1 { fld0: _9 };
match _12.fld1 {
0 => bb7,
1 => bb18,
1042832600 => bb20,
_ => bb19
}
}
bb18 = {
Return()
}
bb19 = {
RET.0 = (-5621946425284504993_i64) + (-6688463439496559233_i64);
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb20 = {
_18 = [_24,_24,_24,_24,_24,_24,_24];
_40.fld0 = !_21;
_38 = _4;
_3 = _21;
_37 = !_7;
_36 = !9688_u16;
_25 = Adt44::Variant1 { fld0: Field::<char>(Variant(_15, 1), 0) };
_13.2 = [_14.0,_14.0,_14.0];
_1 = _13.3 * _13.3;
_25 = Adt44::Variant1 { fld0: Field::<char>(Variant(_15, 1), 0) };
_10 = [_5,_37];
_4 = _38 + _38;
_1 = _13.3 << _7;
_39.1 = [_13.1,_13.1];
_37 = !_5;
_42 = Field::<char>(Variant(_15, 1), 0);
Goto(bb21)
}
bb21 = {
_38 = _4 + _4;
_44 = _28 as usize;
Goto(bb22)
}
bb22 = {
_36 = !42510_u16;
_22 = !_3;
_8 = (_13.0,);
_40 = Adt53 { fld0: _22 };
_21 = _22 ^ _22;
place!(Field::<char>(Variant(_25, 1), 0)) = Field::<char>(Variant(_15, 1), 0);
place!(Field::<char>(Variant(_15, 1), 0)) = _42;
match _12.fld1 {
0 => bb1,
1042832600 => bb23,
_ => bb17
}
}
bb23 = {
SetDiscriminant(_15, 0);
_45 = !_38;
_13.0 = _1 as u8;
_8 = (_13.0,);
SetDiscriminant(_25, 1);
RET.0 = _14.0 * _14.0;
_29 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_39.0 = _45 as u128;
place!(Field::<isize>(Variant(_15, 0), 2)) = _38 - _4;
_50 = _24 as isize;
_3 = _22;
_42 = _9;
_13.0 = _12.fld2 as u8;
Goto(bb24)
}
bb24 = {
place!(Field::<char>(Variant(_25, 1), 0)) = _9;
_3 = _21;
place!(Field::<[char; 7]>(Variant(_15, 0), 3)) = _30;
_4 = Field::<isize>(Variant(_15, 0), 2);
_23 = _35 as f64;
_42 = Field::<char>(Variant(_25, 1), 0);
_33 = _7 - _7;
_51 = Adt53 { fld0: _40.fld0 };
place!(Field::<i128>(Variant(_15, 0), 1)) = _12.fld2 * _12.fld2;
_50 = Field::<isize>(Variant(_15, 0), 2) & Field::<isize>(Variant(_15, 0), 2);
_40.fld0 = _51.fld0;
_47 = [(-20879_i16)];
_39.1 = _19;
_39.1 = _31;
SetDiscriminant(_25, 0);
_21 = _51.fld0 ^ _22;
_4 = !_45;
_49 = _7 as isize;
_48 = _49 as i32;
_51.fld0 = _22 & _40.fld0;
_13.0 = _8.0 ^ _8.0;
Goto(bb25)
}
bb25 = {
_39 = (_6, _31);
_27 = Adt51::Variant0 { fld0: _47,fld1: _8,fld2: Field::<i128>(Variant(_15, 0), 1),fld3: Field::<[char; 7]>(Variant(_15, 0), 3),fld4: _36 };
Goto(bb26)
}
bb26 = {
_15 = Adt44::Variant1 { fld0: _9 };
Call(_28 = core::intrinsics::bswap(_16), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_41 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_13.3 = _1 ^ _1;
place!(Field::<i128>(Variant(_25, 0), 1)) = _12.fld2;
place!(Field::<isize>(Variant(_25, 0), 2)) = _50;
place!(Field::<u16>(Variant(_25, 0), 0)) = _45 as u16;
_43 = -Field::<isize>(Variant(_25, 0), 2);
_13.0 = _8.0 << _5;
place!(Field::<i128>(Variant(_27, 0), 2)) = -_12.fld2;
_19 = [_13.1,_13.1];
_7 = !_5;
_13.3 = _1;
_40.fld0 = !_21;
_1 = _13.3;
_56 = Move(_15);
_53 = -_48;
SetDiscriminant(_27, 2);
_44 = !_13.3;
_54 = _51.fld0;
place!(Field::<(i128, u32, i32)>(Variant(_27, 2), 3)).1 = !_12.fld1;
_46 = _9;
_33 = !_39.0;
_40.fld0 = _3;
RET.0 = _14.0 * _14.0;
place!(Field::<(i128, u32, i32)>(Variant(_27, 2), 3)) = (Field::<i128>(Variant(_25, 0), 1), _12.fld1, _53);
place!(Field::<f32>(Variant(_27, 2), 6)) = _35 - _20;
_29 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1,_12.fld1];
place!(Field::<f32>(Variant(_27, 2), 6)) = -_35;
match _12.fld1 {
0 => bb20,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb24,
5 => bb28,
1042832600 => bb30,
_ => bb29
}
}
bb28 = {
_3 = _22;
_9 = '\u{a0608}';
_12.fld1 = _9 as u32;
_30 = [_9,_9,_9,_9,_9,_9,_9];
Call(_33 = fn19(_14.0, _5, _5, _12.fld3, _13.2, _21, _5, _12.fld2, _7, _3, _14.0, _22, _22, _7), ReturnTo(bb17), UnwindUnreachable())
}
bb29 = {
_18 = [_24,_24,_24,_24,_24,_24,_24];
_40.fld0 = !_21;
_38 = _4;
_3 = _21;
_37 = !_7;
_36 = !9688_u16;
_25 = Adt44::Variant1 { fld0: Field::<char>(Variant(_15, 1), 0) };
_13.2 = [_14.0,_14.0,_14.0];
_1 = _13.3 * _13.3;
_25 = Adt44::Variant1 { fld0: Field::<char>(Variant(_15, 1), 0) };
_10 = [_5,_37];
_4 = _38 + _38;
_1 = _13.3 << _7;
_39.1 = [_13.1,_13.1];
_37 = !_5;
_42 = Field::<char>(Variant(_15, 1), 0);
Goto(bb21)
}
bb30 = {
_8.0 = _43 as u8;
_21 = _40.fld0;
place!(Field::<(i128, u32, i32)>(Variant(_27, 2), 3)) = (Field::<i128>(Variant(_25, 0), 1), _12.fld1, _48);
_40 = Adt53 { fld0: _3 };
Goto(bb31)
}
bb31 = {
place!(Field::<(i128, u32, i32)>(Variant(_27, 2), 3)) = (Field::<i128>(Variant(_25, 0), 1), _12.fld1, _53);
_39 = (_37, _19);
_38 = _49 | _50;
place!(Field::<i128>(Variant(_25, 0), 1)) = Field::<char>(Variant(_56, 1), 0) as i128;
_9 = _42;
_23 = -_17;
place!(Field::<i128>(Variant(_25, 0), 1)) = -_12.fld2;
_14.1 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_27, 2), 4)));
_1 = _13.0 as usize;
_8.0 = (-4110_i16) as u8;
_61 = !_12.fld1;
_10 = [_33,_33];
_45 = Field::<isize>(Variant(_25, 0), 2) - Field::<isize>(Variant(_25, 0), 2);
_15 = Move(_56);
_37 = Field::<f32>(Variant(_27, 2), 6) as u128;
RET = (_14.0, _14.1);
_13.2 = [_14.0,_14.0,RET.0];
_13.3 = !_1;
_44 = _1;
_26 = [Field::<char>(Variant(_15, 1), 0),_42,_46,_9,_42,_42,Field::<char>(Variant(_15, 1), 0)];
Goto(bb32)
}
bb32 = {
Call(_64 = dump_var(15_usize, 7_usize, Move(_7), 21_usize, Move(_21), 36_usize, Move(_36), 2_usize, Move(_2)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_64 = dump_var(15_usize, 47_usize, Move(_47), 39_usize, Move(_39), 1_usize, Move(_1), 43_usize, Move(_43)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_64 = dump_var(15_usize, 53_usize, Move(_53), 48_usize, Move(_48), 49_usize, Move(_49), 28_usize, Move(_28)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_64 = dump_var(15_usize, 10_usize, Move(_10), 24_usize, Move(_24), 19_usize, Move(_19), 4_usize, Move(_4)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_64 = dump_var(15_usize, 18_usize, Move(_18), 38_usize, Move(_38), 34_usize, Move(_34), 3_usize, Move(_3)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_64 = dump_var(15_usize, 16_usize, Move(_16), 65_usize, _65, 65_usize, _65, 65_usize, _65), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: [u128; 2],mut _3: u128,mut _4: u128,mut _5: (u8, i8, [i64; 3], usize),mut _6: [i64; 3],mut _7: [i64; 3],mut _8: u128,mut _9: u128,mut _10: u128) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _11: char;
let _12: Adt54;
let _13: [char; 7];
let _14: char;
let _15: [char; 2];
let _16: f64;
let _17: Adt59;
let _18: (u16, (i64, *const i16), [u128; 2], &'static usize, isize);
let _19: i8;
let _20: i16;
let _21: u8;
let _22: Adt44;
let _23: i64;
let _24: char;
let _25: u16;
let _26: (i128, (u8,));
let _27: i32;
let _28: *mut (usize, [i128; 6]);
let _29: isize;
let _30: Adt49;
let _31: *mut u64;
let _32: u128;
let _33: (usize, [i128; 6]);
let _34: Adt44;
let _35: u128;
let _36: &'static usize;
let _37: bool;
let _38: f64;
let _39: (u32, &'static usize, (i128, u32, i32));
let _40: Adt59;
let _41: Adt58;
let _42: *mut [i64; 3];
let _43: (u8, i8, [i64; 3], usize);
let _44: bool;
let _45: i32;
let _46: Adt48;
let _47: isize;
let _48: ();
let _49: ();
{
_5 = (178_u8, 118_i8, _6, 4_usize);
_4 = !_8;
_5 = (160_u8, 96_i8, _7, 0_usize);
_2 = [_8,_3];
_11 = '\u{fd9ff}';
Goto(bb1)
}
bb1 = {
RET = [1701925487_i32,680380985_i32,(-149097788_i32),896402962_i32,1535970694_i32,(-1903061251_i32),(-1499878423_i32)];
RET = [(-691937616_i32),(-1227921181_i32),695356119_i32,(-1342765587_i32),(-475889498_i32),(-1029713342_i32),504955092_i32];
_4 = !_9;
RET = [1254680062_i32,1486090806_i32,(-1923134875_i32),778990370_i32,788853238_i32,1669971520_i32,(-42303042_i32)];
_5 = (102_u8, (-96_i8), _7, 18196123170729490035_usize);
_3 = _10 | _9;
_5 = (204_u8, 70_i8, _7, 1_usize);
_5.1 = 18355618481347343089_u64 as i8;
_1 = true;
_11 = '\u{ccb26}';
_11 = '\u{107dfe}';
_2 = [_10,_10];
_7 = [(-1389696913326780155_i64),3970843290969807093_i64,(-6403270862939579502_i64)];
_7 = [8777577458623339657_i64,7828998717805285537_i64,1342904838494682138_i64];
_1 = _9 <= _8;
_6 = _5.2;
_6 = _5.2;
Call(_4 = core::intrinsics::bswap(_8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.2 = [(-8702338135525747491_i64),(-6576904284871723175_i64),7164189717555286544_i64];
_5.2 = _7;
_6 = _7;
_3 = (-9223372036854775808_isize) as u128;
_1 = false;
_14 = _11;
_5 = (251_u8, (-111_i8), _6, 6823890397010830208_usize);
_5.2 = _7;
_11 = _14;
_4 = (-995124531_i32) as u128;
_5.2 = _6;
_5 = (21_u8, 6_i8, _7, 1_usize);
RET = [1455108972_i32,(-123210599_i32),1728102901_i32,1596141868_i32,(-543891759_i32),(-789207728_i32),(-162066624_i32)];
_11 = _14;
_9 = 1919356557_u32 as u128;
_3 = !_10;
_3 = _8 >> _8;
_1 = true;
_5.2 = [(-891908232477931796_i64),(-1714934928274314826_i64),6316375883086570785_i64];
_5.3 = !17132082563548606550_usize;
_8 = _10 | _3;
_5.0 = !52_u8;
_13 = [_11,_14,_14,_14,_14,_14,_14];
_3 = _8 ^ _10;
_17.fld0 = _5.3 * _5.3;
_4 = _10 - _3;
_17.fld2 = 12801_i16 as f64;
Goto(bb3)
}
bb3 = {
_17.fld7 = Adt53 { fld0: _1 };
_17.fld3 = _13;
_7 = [2404802190726406365_i64,7375069533652127358_i64,9082840509774341421_i64];
_15 = [_14,_14];
_5.0 = !75_u8;
_17.fld7 = Adt53 { fld0: _1 };
_11 = _14;
RET = [(-2134923604_i32),1181962613_i32,(-926199643_i32),(-547999682_i32),692343625_i32,1367252762_i32,1031746034_i32];
_15 = [_14,_11];
_17.fld1.0 = _17.fld0;
_18.4 = 9223372036854775807_isize;
_17.fld6 = !(-5695365308933828085_i64);
_17.fld1.1 = [168605262118244666642808238951449835354_i128,(-139288283016449397440201555639191825349_i128),(-26775008809416176841197270136303739777_i128),1841376626822554796836089321953423693_i128,136430358819091907873802597039785953598_i128,114556230278993002116525484609409455171_i128];
_14 = _11;
_17.fld2 = _17.fld6 as f64;
_4 = _5.3 as u128;
_18.3 = &_17.fld1.0;
_2 = [_3,_3];
_1 = _17.fld7.fld0;
_17.fld2 = _5.1 as f64;
_18.0 = 30159_u16 | 45979_u16;
_9 = _3 >> _8;
_1 = !_17.fld7.fld0;
_5.0 = 71_u8;
RET = [1526077062_i32,(-1020459870_i32),713210119_i32,726873393_i32,(-359574258_i32),(-1119764105_i32),(-1580316772_i32)];
_3 = _8;
RET = [58080176_i32,(-1173728849_i32),(-1030174028_i32),1935045317_i32,(-923214907_i32),461817846_i32,1261458579_i32];
_16 = _17.fld2;
Call(_17.fld2 = core::intrinsics::transmute(_17.fld1.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = _17.fld2 + _17.fld2;
_17.fld6 = _5.1 as i64;
_4 = _8;
_17.fld3 = [_11,_14,_14,_14,_14,_11,_11];
_18.3 = &_17.fld0;
Goto(bb5)
}
bb5 = {
_5.2 = [_17.fld6,_17.fld6,_17.fld6];
_20 = 31924_i16 * 5704_i16;
_23 = !_17.fld6;
_18.1.1 = core::ptr::addr_of!(_20);
_10 = !_3;
_17.fld3 = _13;
_18.3 = &_17.fld1.0;
_19 = _14 as i8;
_16 = -_17.fld2;
_17.fld7.fld0 = _1 & _1;
_11 = _14;
_18.2 = [_8,_8];
_14 = _11;
_18.4 = _16 as isize;
_8 = !_9;
_6 = [_23,_23,_17.fld6];
_17.fld1.1 = [(-32647263002027618987043125220310404788_i128),(-127681792893595128316047023510917431215_i128),(-76034194574290025173753039457909980906_i128),(-122394214339071602504032477731657888571_i128),74467634726118598989332302827835451707_i128,109022928685846379274207940290719410897_i128];
_26.1 = (_5.0,);
_25 = _18.4 as u16;
RET = [1102100546_i32,(-1796437852_i32),(-1235112875_i32),(-1361569716_i32),(-762130038_i32),(-786073743_i32),2146540293_i32];
_19 = _5.1;
_27 = (-2085654465_i32) << _3;
_26.0 = -67682291290824050927818368891132049223_i128;
_17.fld0 = !_5.3;
Goto(bb6)
}
bb6 = {
_26.1 = (_5.0,);
_17.fld3 = [_11,_14,_14,_11,_11,_14,_14];
_6 = _7;
_5.2 = _7;
_25 = _1 as u16;
_2 = [_10,_8];
_3 = !_9;
_26.1 = (_5.0,);
_14 = _11;
_22 = Adt44::Variant0 { fld0: _25,fld1: _26.0,fld2: _18.4,fld3: _17.fld3 };
SetDiscriminant(_22, 1);
_28 = core::ptr::addr_of_mut!(_17.fld1);
Call(_21 = core::intrinsics::bswap(_5.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = _17.fld7.fld0 as u128;
_25 = _18.0 ^ _18.0;
Call(_6 = fn17(_27, _2, _4, _10, _18.2, _4, _9, _18.2, RET, _5.2, _10, _9, _9, _9, _27), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17.fld0 = !_5.3;
_18.1.0 = _17.fld6;
match _19 {
0 => bb7,
1 => bb2,
2 => bb3,
6 => bb9,
_ => bb5
}
}
bb9 = {
_26.1 = (_5.0,);
_27 = -(-2023144850_i32);
_18.3 = &_17.fld1.0;
_5.0 = _18.4 as u8;
(*_28).1 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
place!(Field::<char>(Variant(_22, 1), 0)) = _11;
_33.1 = (*_28).1;
(*_28).1 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_4 = _1 as u128;
(*_28).1 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
(*_28).0 = _18.4 as usize;
_19 = _5.1;
_23 = _18.4 as i64;
_15 = [_11,Field::<char>(Variant(_22, 1), 0)];
_17.fld5 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_17.fld5 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_18.4 = -9223372036854775807_isize;
_33 = ((*_28).0, (*_28).1);
match _5.1 {
0 => bb1,
1 => bb2,
6 => bb10,
_ => bb5
}
}
bb10 = {
_17.fld7 = Adt53 { fld0: _1 };
_27 = _9 as i32;
_32 = !_3;
_18.0 = _26.0 as u16;
_17.fld1.0 = _33.0 << _9;
_24 = _11;
_18.1.1 = core::ptr::addr_of!(_20);
_18.0 = _25 - _25;
_9 = _17.fld2 as u128;
_26.0 = (*_28).0 as i128;
SetDiscriminant(_22, 1);
_11 = _14;
_9 = _3;
(*_28).0 = _17.fld0;
_4 = _3 - _10;
_19 = !_5.1;
_17.fld7.fld0 = _1 ^ _1;
_27 = _17.fld2 as i32;
RET = [_27,_27,_27,_27,_27,_27,_27];
(*_28).0 = !_17.fld0;
(*_28).0 = _33.0 - _5.3;
(*_28).1 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_5 = (_26.1.0, _19, _6, (*_28).0);
Goto(bb11)
}
bb11 = {
_29 = !_18.4;
RET = [_27,_27,_27,_27,_27,_27,_27];
_5.1 = _19 << _10;
_5 = (_26.1.0, _19, _6, (*_28).0);
RET = [_27,_27,_27,_27,_27,_27,_27];
_33.1 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_11 = _14;
place!(Field::<char>(Variant(_22, 1), 0)) = _11;
_29 = _5.0 as isize;
Goto(bb12)
}
bb12 = {
_6 = [_17.fld6,_17.fld6,_18.1.0];
SetDiscriminant(_22, 0);
_29 = _18.4 & _18.4;
_17.fld6 = _18.1.0 * _18.1.0;
match _26.1.0 {
0 => bb4,
1 => bb5,
71 => bb13,
_ => bb10
}
}
bb13 = {
_40.fld1.0 = !_5.3;
_26.1.0 = _5.0 | _5.0;
_34 = Adt44::Variant1 { fld0: _11 };
Call(_39.2.0 = fn18(_33, _19, _26, _33, _5.3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_18.3 = &_40.fld0;
_35 = _10 ^ _3;
_42 = core::ptr::addr_of_mut!(_6);
(*_28) = (_33.0, _33.1);
_43.0 = _26.1.0 ^ _26.1.0;
_39.2.1 = _18.0 as u32;
_16 = _17.fld2;
_5.0 = 5098768739621826085_u64 as u8;
_33 = (*_28);
(*_28).1 = _33.1;
_18.2 = [_35,_35];
place!(Field::<[char; 7]>(Variant(_22, 0), 3)) = _17.fld3;
_5.1 = _19;
(*_42) = _5.2;
_26.0 = _39.2.0;
_33 = (_17.fld1.0, _17.fld1.1);
place!(Field::<[char; 7]>(Variant(_22, 0), 3)) = _17.fld3;
_40.fld1.1 = (*_28).1;
_43.2 = [_17.fld6,_17.fld6,_17.fld6];
_40.fld2 = -_17.fld2;
_22 = Move(_34);
_36 = &_40.fld1.0;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(16_usize, 33_usize, Move(_33), 10_usize, Move(_10), 24_usize, Move(_24), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(16_usize, 35_usize, Move(_35), 26_usize, Move(_26), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(16_usize, 8_usize, Move(_8), 13_usize, Move(_13), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(16_usize, 11_usize, Move(_11), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i32,mut _2: [u128; 2],mut _3: u128,mut _4: u128,mut _5: [u128; 2],mut _6: u128,mut _7: u128,mut _8: [u128; 2],mut _9: [i32; 7],mut _10: [i64; 3],mut _11: u128,mut _12: u128,mut _13: u128,mut _14: u128,mut _15: i32) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _16: Adt55;
let _17: u32;
let _18: [u32; 6];
let _19: usize;
let _20: isize;
let _21: u128;
let _22: Adt44;
let _23: Adt46;
let _24: f64;
let _25: f64;
let _26: Adt53;
let _27: i8;
let _28: (i128, u32, i32);
let _29: f32;
let _30: f64;
let _31: i8;
let _32: [i32; 7];
let _33: u32;
let _34: Adt46;
let _35: [i8; 2];
let _36: [i8; 2];
let _37: Adt54;
let _38: bool;
let _39: [i64; 3];
let _40: (usize, [i128; 6]);
let _41: [i32; 7];
let _42: isize;
let _43: [char; 7];
let _44: f32;
let _45: [u128; 2];
let _46: [i8; 2];
let _47: f64;
let _48: f32;
let _49: [i8; 2];
let _50: u128;
let _51: ();
let _52: ();
{
_13 = 208_u8 as u128;
Call(_6 = core::intrinsics::transmute(_7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = !_3;
_2 = _5;
_15 = !_1;
_1 = -_15;
_3 = _14 << _1;
_17 = 3372084571_u32 * 1100384553_u32;
_5 = _8;
Goto(bb2)
}
bb2 = {
_1 = _15 << _3;
RET = [(-446708036109555991_i64),(-4529573724050971202_i64),(-4185294434343121602_i64)];
_1 = _15;
_18 = [_17,_17,_17,_17,_17,_17];
_5 = [_11,_11];
_7 = _4;
_7 = !_12;
_5 = _8;
_14 = 14235_u16 as u128;
_17 = 2114823443_u32 * 1346694967_u32;
_21 = _3;
_11 = !_4;
_21 = _12 >> _4;
_14 = _21;
_21 = !_6;
_3 = !_14;
_13 = _3;
Goto(bb3)
}
bb3 = {
RET = [(-9058992962276021661_i64),(-3215748281321442700_i64),6621888965160704568_i64];
Goto(bb4)
}
bb4 = {
_20 = 9223372036854775807_isize;
_11 = _17 as u128;
_5 = [_14,_14];
_4 = _3 & _13;
_3 = !_12;
_2 = [_4,_14];
_12 = _21;
_25 = (-14173_i16) as f64;
_26.fld0 = _15 >= _1;
_26.fld0 = true;
_5 = _8;
_27 = 45064_u16 as i8;
_5 = [_21,_6];
RET = _10;
_26 = Adt53 { fld0: false };
_25 = 62_u8 as f64;
_24 = _20 as f64;
Goto(bb5)
}
bb5 = {
_26.fld0 = false;
_2 = _8;
_6 = _4 + _14;
_13 = !_3;
_24 = _25;
_25 = 1_usize as f64;
_14 = 1119365038400367839_i64 as u128;
_26 = Adt53 { fld0: false };
_24 = _25;
_24 = _25;
_25 = _24 + _24;
_24 = _25;
_13 = _6 >> _3;
_26 = Adt53 { fld0: true };
_19 = _20 as usize;
_1 = _6 as i32;
_21 = _12 >> _3;
RET = _10;
_1 = _15;
_30 = _24 - _24;
Call(_30 = core::intrinsics::fmaf64(_25, _24, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = _6 * _3;
_15 = _1;
_18 = [_17,_17,_17,_17,_17,_17];
_24 = _19 as f64;
_28.2 = !_1;
_28.0 = (-14674413270831954380097398086829660956_i128) - (-78934218364515296633683584785508986145_i128);
_15 = _17 as i32;
_7 = !_6;
_13 = _4 + _6;
_28 = ((-7565985250568856634704689191365356928_i128), _17, _1);
_26.fld0 = true;
_18 = [_28.1,_28.1,_17,_28.1,_17,_28.1];
_25 = _30;
_28.2 = _1 & _1;
_30 = _25;
RET = _10;
_5 = _8;
_31 = _27 ^ _27;
_4 = _6;
_11 = 47502_u16 as u128;
_10 = RET;
_12 = _4;
_24 = _19 as f64;
RET = [9160251422640926999_i64,5880191644070705963_i64,120039982953388977_i64];
_13 = !_6;
_25 = _30 * _30;
_29 = _21 as f32;
_27 = !_31;
_28.1 = _17 * _17;
Call(_20 = core::intrinsics::bswap(51_isize), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = !_28.2;
_1 = _28.2 >> _12;
_33 = _28.1 | _28.1;
_35 = [_27,_27];
_26 = Adt53 { fld0: false };
_11 = _12;
_3 = !_21;
_36 = [_31,_31];
_27 = !_31;
match _28.0 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
332716381670369606828669918240402854528 => bb12,
_ => bb11
}
}
bb8 = {
_1 = _15 << _3;
RET = [(-446708036109555991_i64),(-4529573724050971202_i64),(-4185294434343121602_i64)];
_1 = _15;
_18 = [_17,_17,_17,_17,_17,_17];
_5 = [_11,_11];
_7 = _4;
_7 = !_12;
_5 = _8;
_14 = 14235_u16 as u128;
_17 = 2114823443_u32 * 1346694967_u32;
_21 = _3;
_11 = !_4;
_21 = _12 >> _4;
_14 = _21;
_21 = !_6;
_3 = !_14;
_13 = _3;
Goto(bb3)
}
bb9 = {
_26.fld0 = false;
_2 = _8;
_6 = _4 + _14;
_13 = !_3;
_24 = _25;
_25 = 1_usize as f64;
_14 = 1119365038400367839_i64 as u128;
_26 = Adt53 { fld0: false };
_24 = _25;
_24 = _25;
_25 = _24 + _24;
_24 = _25;
_13 = _6 >> _3;
_26 = Adt53 { fld0: true };
_19 = _20 as usize;
_1 = _6 as i32;
_21 = _12 >> _3;
RET = _10;
_1 = _15;
_30 = _24 - _24;
Call(_30 = core::intrinsics::fmaf64(_25, _24, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_20 = 9223372036854775807_isize;
_11 = _17 as u128;
_5 = [_14,_14];
_4 = _3 & _13;
_3 = !_12;
_2 = [_4,_14];
_12 = _21;
_25 = (-14173_i16) as f64;
_26.fld0 = _15 >= _1;
_26.fld0 = true;
_5 = _8;
_27 = 45064_u16 as i8;
_5 = [_21,_6];
RET = _10;
_26 = Adt53 { fld0: false };
_25 = 62_u8 as f64;
_24 = _20 as f64;
Goto(bb5)
}
bb11 = {
RET = [(-9058992962276021661_i64),(-3215748281321442700_i64),6621888965160704568_i64];
Goto(bb4)
}
bb12 = {
_6 = !_12;
_27 = _31;
_2 = _8;
_26.fld0 = !true;
_9 = [_28.2,_1,_28.2,_1,_1,_1,_28.2];
RET = [3486835535220226485_i64,(-5511741036506342752_i64),(-8271568021802621767_i64)];
_32 = [_28.2,_28.2,_1,_28.2,_1,_1,_28.2];
_5 = _8;
_9 = [_1,_28.2,_28.2,_1,_28.2,_1,_1];
_6 = _12 & _13;
_7 = _6 | _6;
_26 = Adt53 { fld0: false };
_27 = _31;
_17 = '\u{361e}' as u32;
_38 = !_26.fld0;
_18 = [_33,_33,_33,_33,_33,_33];
_32 = _9;
_11 = 11_u8 as u128;
_36 = _35;
_28 = ((-29620032316245691421536563743500632647_i128), _33, _1);
RET = _10;
_11 = _6 & _12;
_14 = _3;
RET = _10;
_11 = !_13;
_39 = [(-3497792759974286351_i64),(-5149004153270459987_i64),5864036121899918146_i64];
_26 = Adt53 { fld0: _38 };
Call(_13 = core::intrinsics::bswap(_14), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_27 = (-22426_i16) as i8;
RET = [1099880511028028023_i64,8469365459030331660_i64,(-6911518550866712721_i64)];
_39 = [(-7296332133587267989_i64),(-1132940293988742267_i64),8612096927681487339_i64];
RET = _10;
_6 = _28.1 as u128;
_28.1 = _27 as u32;
_4 = _12 & _11;
_28.1 = _17 & _33;
_15 = _1;
_40.1 = [_28.0,_28.0,_28.0,_28.0,_28.0,_28.0];
_26 = Adt53 { fld0: _38 };
_14 = _21;
_25 = -_30;
_43 = ['\u{e9e2d}','\u{eac78}','\u{e0ddf}','\u{d3973}','\u{9f84d}','\u{20f6b}','\u{101b9c}'];
_26.fld0 = _38 ^ _38;
_30 = _24 - _25;
_33 = _28.1;
_44 = _29;
_43 = ['\u{b3d91}','\u{5054a}','\u{e2fe5}','\u{10b9c2}','\u{45523}','\u{ab9ea}','\u{2341f}'];
_36 = [_31,_31];
Goto(bb14)
}
bb14 = {
_15 = !_28.2;
_47 = _25 - _25;
_42 = _19 as isize;
_18 = [_28.1,_33,_33,_33,_33,_28.1];
_6 = !_12;
_15 = _28.2 << _28.0;
_42 = '\u{7f450}' as isize;
_36 = _35;
_24 = _30;
_28.0 = 84186516879102665028025913321320232641_i128;
_25 = 103_u8 as f64;
_48 = _28.0 as f32;
_40.0 = _19;
_24 = -_47;
_46 = [_27,_27];
_9 = [_28.2,_15,_15,_1,_28.2,_15,_28.2];
_49 = [_31,_27];
_36 = [_31,_31];
_31 = _28.0 as i8;
_44 = _29 * _29;
_24 = _31 as f64;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(17_usize, 38_usize, Move(_38), 33_usize, Move(_33), 31_usize, Move(_31), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(17_usize, 40_usize, Move(_40), 43_usize, Move(_43), 12_usize, Move(_12), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(17_usize, 11_usize, Move(_11), 39_usize, Move(_39), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(17_usize, 28_usize, Move(_28), 27_usize, Move(_27), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_51 = dump_var(17_usize, 1_usize, Move(_1), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: (usize, [i128; 6]),mut _2: i8,mut _3: (i128, (u8,)),mut _4: (usize, [i128; 6]),mut _5: usize) -> i128 {
mir! {
type RET = i128;
let _6: (u8,);
let _7: isize;
let _8: ();
let _9: ();
{
_1 = _4;
RET = _3.0;
_5 = false as usize;
_3.1.0 = 253_u8;
_4 = (_1.0, _1.1);
_1 = (_5, _4.1);
_4 = (_5, _1.1);
_3.1 = (17_u8,);
_3.1 = (32_u8,);
_1.1 = [_3.0,_3.0,_3.0,RET,_3.0,_3.0];
_3.1 = (168_u8,);
RET = _3.0 ^ _3.0;
_2 = -0_i8;
_3.0 = 43562075475662024106115647134900834346_u128 as i128;
_6.0 = 5440860959077126954_u64 as u8;
_6.0 = _3.1.0 + _3.1.0;
_3.0 = RET;
_7 = -(-45_isize);
_5 = !_4.0;
_3.1 = (_6.0,);
_6.0 = !_3.1.0;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(18_usize, 5_usize, Move(_5), 4_usize, Move(_4), 2_usize, Move(_2), 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i64,mut _2: u128,mut _3: u128,mut _4: [u128; 2],mut _5: [i64; 3],mut _6: bool,mut _7: u128,mut _8: i128,mut _9: u128,mut _10: bool,mut _11: i64,mut _12: bool,mut _13: bool,mut _14: u128) -> u128 {
mir! {
type RET = u128;
let _15: ();
let _16: ();
{
_7 = !_3;
_3 = _7;
_2 = _3;
_14 = _7 << _2;
RET = _14 << _2;
_2 = _9 >> _14;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(19_usize, 6_usize, Move(_6), 5_usize, Move(_5), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(19_usize, 12_usize, Move(_12), 10_usize, Move(_10), 1_usize, Move(_1), 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{d7c35}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-28_i8)), std::hint::black_box(1294_i16), std::hint::black_box(30766_u16), std::hint::black_box(3595072237_u32), std::hint::black_box((-136021917685787477658901978051018366321_i128)), std::hint::black_box(5_usize), std::hint::black_box(147_u8));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u16,
fld1: i128,
fld2: isize,
fld3: [char; 7],

},
Variant1{
fld0: char,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *const *mut (usize, [i128; 6]),
fld1: u128,
fld2: [char; 2],
fld3: i8,
fld4: f64,
fld5: *const *mut [i64; 3],
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: (i64, *const i16),
fld1: [char; 2],
fld2: i8,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: usize,
fld3: Adt44,
fld4: [char; 7],

},
Variant2{
fld0: *mut u64,
fld1: u8,
fld2: *const *mut [i64; 3],

},
Variant3{
fld0: [u32; 6],
fld1: (usize, [i128; 6]),
fld2: *const *const [i8; 2],
fld3: i128,
fld4: [u128; 2],
fld5: (i128, (u8,)),
fld6: (u128, [i8; 2]),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const *mut (usize, [i128; 6]),
fld1: Adt45,
fld2: i32,
fld3: f32,
fld4: *const *mut [i64; 3],

},
Variant1{
fld0: *mut u64,

},
Variant2{
fld0: *const i16,
fld1: char,
fld2: i64,
fld3: *const *mut [i64; 3],
fld4: usize,

},
Variant3{
fld0: u8,
fld1: char,
fld2: Adt45,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: (u8, i8, [i64; 3], usize),
fld1: char,
fld2: [i128; 6],
fld3: *const [i8; 2],
fld4: i16,
fld5: (i128, (u8,)),
fld6: f32,
fld7: i128,

},
Variant1{
fld0: *mut (usize, [i128; 6]),

},
Variant2{
fld0: bool,
fld1: [i16; 1],
fld2: isize,
fld3: f64,
fld4: i16,
fld5: (u128, [i8; 2]),
fld6: *const i16,
fld7: (usize, [i128; 6]),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u16,
fld1: (u128, [i8; 2]),
fld2: i32,

},
Variant1{
fld0: bool,
fld1: [i128; 6],
fld2: isize,
fld3: (i64, *const i16),
fld4: [i32; 7],
fld5: *mut [i64; 3],
fld6: (u128, [i8; 2]),

},
Variant2{
fld0: Adt46,
fld1: (u128, [i8; 2]),
fld2: [i64; 3],
fld3: f64,
fld4: Adt48,
fld5: Adt47,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt48,

},
Variant1{
fld0: Adt49,
fld1: Adt45,
fld2: i64,
fld3: (u128, [i8; 2]),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [i16; 1],
fld1: (u8,),
fld2: i128,
fld3: [char; 7],
fld4: u16,

},
Variant1{
fld0: f32,
fld1: Adt49,
fld2: u64,
fld3: u32,

},
Variant2{
fld0: *const *mut (usize, [i128; 6]),
fld1: [char; 7],
fld2: [i128; 6],
fld3: (i128, u32, i32),
fld4: i16,
fld5: i32,
fld6: f32,
fld7: *const *const [i8; 2],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
fld0: (i128, (u8,)),
fld1: *const *mut [i64; 3],
fld2: isize,
fld3: Adt48,
fld4: Adt46,
fld5: [i64; 3],
fld6: Adt44,
fld7: Adt50,

},
Variant1{
fld0: u32,
fld1: (i128, (u8,)),

},
Variant2{
fld0: [char; 2],
fld1: [i8; 2],
fld2: isize,
fld3: (usize, [i128; 6]),
fld4: i16,
fld5: u32,
fld6: i64,
fld7: Adt45,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: bool,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt51,
fld1: u32,
fld2: [i32; 7],
fld3: i8,
fld4: [i8; 2],
fld5: Adt46,
fld6: [char; 2],
fld7: u16,

},
Variant1{
fld0: *mut u64,
fld1: *mut [i64; 3],
fld2: (i128, u32, i32),
fld3: Adt50,
fld4: [i32; 7],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *mut (usize, [i128; 6]),
fld1: Adt49,
fld2: Adt54,
fld3: *const i16,
fld4: [char; 2],
fld5: [i128; 6],
fld6: u32,
fld7: *const *mut (usize, [i128; 6]),

},
Variant1{
fld0: [char; 7],
fld1: *mut u64,
fld2: (i128, u32, i32),

},
Variant2{
fld0: bool,
fld1: *mut (usize, [i128; 6]),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (u8, i8, [i64; 3], usize),
fld1: *const *const [i8; 2],

},
Variant1{
fld0: (i128, u32, i32),
fld1: Adt49,
fld2: *const i16,
fld3: Adt50,
fld4: *const *mut [i64; 3],
fld5: (u8, i8, [i64; 3], usize),
fld6: usize,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: Adt56,
fld1: u32,
fld2: i128,
fld3: [u128; 2],
fld4: *const *mut (usize, [i128; 6]),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: Adt47,
fld1: u32,

},
Variant1{
fld0: [char; 2],
fld1: *const *const [i8; 2],
fld2: *mut [i64; 3],
fld3: i128,
fld4: (u128, [i8; 2]),
fld5: i32,
fld6: [i8; 2],

},
Variant2{
fld0: *const *mut (usize, [i128; 6]),

},
Variant3{
fld0: Adt51,
fld1: [i128; 6],
fld2: u64,
fld3: *const *mut [i64; 3],
fld4: f64,
fld5: Adt48,
fld6: [i32; 7],
fld7: usize,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: usize,
fld1: (usize, [i128; 6]),
fld2: f64,
fld3: [char; 7],
fld4: Adt58,
fld5: [i128; 6],
fld6: i64,
fld7: Adt53,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt50,
fld1: Adt59,
fld2: [i16; 1],
fld3: Adt52,
fld4: f32,
fld5: Adt45,

},
Variant1{
fld0: (usize, [i128; 6]),
fld1: (i128, u32, i32),
fld2: Adt59,
fld3: (u128, [i8; 2]),
fld4: *const *const [i8; 2],
fld5: Adt57,

}}

