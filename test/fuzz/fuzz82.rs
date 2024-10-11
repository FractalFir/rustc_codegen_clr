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
pub fn fn0(mut _1: u32,mut _2: u128,mut _3: u16,mut _4: i8,mut _5: usize,mut _6: i32) -> f32 {
mir! {
type RET = f32;
let _7: f32;
let _8: Adt52;
let _9: f64;
let _10: Adt63;
let _11: u16;
let _12: [isize; 7];
let _13: [isize; 7];
let _14: [i32; 1];
let _15: char;
let _16: i64;
let _17: f32;
let _18: isize;
let _19: isize;
let _20: [i32; 1];
let _21: isize;
let _22: Adt52;
let _23: [isize; 7];
let _24: [usize; 7];
let _25: (f32, bool);
let _26: *const usize;
let _27: i8;
let _28: ();
let _29: ();
{
_4 = (-85_i8) & (-2_i8);
_2 = !304650566913134584204302796521206408859_u128;
RET = 13060140848223645684_u64 as f32;
_3 = 90_u8 as u16;
_8.fld1 = !0_usize;
_5 = true as usize;
_8.fld2 = [3321490259377691685_i64,3433575060151908949_i64,3448407185698507878_i64,(-7424577266542156414_i64),(-3398189955137466776_i64)];
_6 = 506974158_i32;
_8.fld1 = _5 + _5;
_8.fld2 = [(-7471676050624678170_i64),34578145083850209_i64,6523521826062087299_i64,3483347816643900667_i64,(-1010520374440946586_i64)];
_4 = 59_i8 - 77_i8;
_6 = -(-2017587355_i32);
_8.fld2 = [6584106314674870916_i64,1526241073496992508_i64,(-1884809326924787436_i64),1981475903581894306_i64,(-2108109283883648668_i64)];
RET = 3823493589_u32 as f32;
_7 = 9223372036854775807_isize as f32;
_8.fld0 = core::ptr::addr_of_mut!(_4);
_1 = 267132462_u32;
Call(_8.fld3 = fn1(_8.fld2, _8.fld2, _5, _8.fld2, _5, _7, _4, _1, _8.fld2, _8.fld2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 3005239705_u32 ^ 2508409053_u32;
_3 = 10103_u16;
_9 = _6 as f64;
match _3 {
10103 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_7 = (-32356346419879485644436431183255333799_i128) as f32;
_4 = _1 as i8;
RET = (-1421_i16) as f32;
_8.fld3 = [true];
_9 = (-11144_i16) as f64;
_4 = (-6_i8) & (-116_i8);
_8.fld2 = [(-9202643431211755976_i64),(-3443203742383309901_i64),(-5328191904440905402_i64),(-5775036294605676628_i64),519343914574595760_i64];
_9 = 150990351033286575249017263054821911483_i128 as f64;
_8.fld1 = _9 as usize;
_12 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_11 = !_3;
_8.fld1 = _5 + _5;
RET = _7 * _7;
_2 = 125452337336149363332355546324696229958_u128;
_13 = _12;
match _3 {
0 => bb2,
10103 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
RET = _7;
_5 = _8.fld1 * _8.fld1;
Goto(bb6)
}
bb6 = {
_9 = _11 as f64;
_7 = -RET;
_6 = !(-496956160_i32);
RET = -_7;
_11 = _3 - _3;
_2 = _1 as u128;
RET = -_7;
_13 = _12;
_8.fld2 = [(-3111899078263933418_i64),4623851534234686780_i64,(-4831431192391971846_i64),6303681331959904240_i64,(-8679500732206987597_i64)];
_8.fld1 = _6 as usize;
_15 = '\u{e50ec}';
_7 = RET;
RET = _1 as f32;
_4 = _2 as i8;
_8.fld3 = [false];
_8.fld0 = core::ptr::addr_of_mut!(_4);
_6 = (-622485929_i32) ^ 1894384290_i32;
Goto(bb7)
}
bb7 = {
_14 = [_6];
RET = _7;
_4 = (-83_i8) >> _5;
_8.fld3 = [true];
_3 = _11;
_15 = '\u{e1645}';
_14 = [_6];
_13 = _12;
_5 = _8.fld1;
_11 = !_3;
RET = _7;
_12 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_8.fld0 = core::ptr::addr_of_mut!(_4);
_8.fld0 = core::ptr::addr_of_mut!(_4);
_11 = 9223372036854775807_isize as u16;
_13 = [(-98_isize),9223372036854775807_isize,(-12_isize),(-14_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),111_isize];
_8.fld0 = core::ptr::addr_of_mut!(_4);
_18 = -(-9223372036854775808_isize);
RET = _1 as f32;
_2 = _15 as u128;
_5 = !_8.fld1;
_20 = [_6];
_1 = _18 as u32;
_8.fld3 = [false];
_15 = '\u{8860}';
Call(_5 = core::intrinsics::transmute(_8.fld1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = _7;
_22.fld1 = _5 ^ _8.fld1;
_20 = _14;
_3 = _11;
_21 = _18;
_16 = (-6898520286463026977_i64);
_20 = [_6];
_20 = [_6];
_22.fld2 = [_16,_16,_16,_16,_16];
_14 = [_6];
_3 = _11 | _11;
_8.fld0 = core::ptr::addr_of_mut!(_4);
_3 = _6 as u16;
_2 = 16627261081870510910134573811852064227_u128;
_22.fld3 = [false];
_19 = _21;
_14 = [_6];
_17 = -_7;
_21 = !_18;
_22.fld3 = [false];
_22.fld1 = 112293462070130213258992083925944596334_i128 as usize;
_19 = _18;
_22 = _8;
_18 = !_21;
_8 = Adt52 { fld0: _22.fld0,fld1: _22.fld1,fld2: _22.fld2,fld3: _22.fld3 };
_25.1 = _2 <= _2;
match _16 {
0 => bb9,
340282366920938463456476087145305184479 => bb11,
_ => bb10
}
}
bb9 = {
RET = _7;
_5 = _8.fld1 * _8.fld1;
Goto(bb6)
}
bb10 = {
Return()
}
bb11 = {
_13 = [_18,_18,_19,_21,_19,_18,_21];
match _16 {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
340282366920938463456476087145305184479 => bb16,
_ => bb15
}
}
bb12 = {
Return()
}
bb13 = {
_9 = _11 as f64;
_7 = -RET;
_6 = !(-496956160_i32);
RET = -_7;
_11 = _3 - _3;
_2 = _1 as u128;
RET = -_7;
_13 = _12;
_8.fld2 = [(-3111899078263933418_i64),4623851534234686780_i64,(-4831431192391971846_i64),6303681331959904240_i64,(-8679500732206987597_i64)];
_8.fld1 = _6 as usize;
_15 = '\u{e50ec}';
_7 = RET;
RET = _1 as f32;
_4 = _2 as i8;
_8.fld3 = [false];
_8.fld0 = core::ptr::addr_of_mut!(_4);
_6 = (-622485929_i32) ^ 1894384290_i32;
Goto(bb7)
}
bb14 = {
Return()
}
bb15 = {
_14 = [_6];
RET = _7;
_4 = (-83_i8) >> _5;
_8.fld3 = [true];
_3 = _11;
_15 = '\u{e1645}';
_14 = [_6];
_13 = _12;
_5 = _8.fld1;
_11 = !_3;
RET = _7;
_12 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_8.fld0 = core::ptr::addr_of_mut!(_4);
_8.fld0 = core::ptr::addr_of_mut!(_4);
_11 = 9223372036854775807_isize as u16;
_13 = [(-98_isize),9223372036854775807_isize,(-12_isize),(-14_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),111_isize];
_8.fld0 = core::ptr::addr_of_mut!(_4);
_18 = -(-9223372036854775808_isize);
RET = _1 as f32;
_2 = _15 as u128;
_5 = !_8.fld1;
_20 = [_6];
_1 = _18 as u32;
_8.fld3 = [false];
_15 = '\u{8860}';
Call(_5 = core::intrinsics::transmute(_8.fld1), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
_15 = '\u{66f41}';
_23 = [_18,_18,_19,_18,_18,_19,_18];
_25 = (RET, true);
_2 = 324665398148701834225933425099931364139_u128;
_14 = _20;
_8.fld1 = (-160986325288496775301299620833073605989_i128) as usize;
_14 = [_6];
_26 = core::ptr::addr_of!(_5);
_26 = core::ptr::addr_of!((*_26));
_25.0 = -_17;
_22 = _8;
_27 = -_4;
_19 = _11 as isize;
_3 = _11;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(0_usize, 1_usize, Move(_1), 5_usize, Move(_5), 19_usize, Move(_19), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(0_usize, 18_usize, Move(_18), 6_usize, Move(_6), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(0_usize, 14_usize, Move(_14), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i64; 5],mut _2: [i64; 5],mut _3: usize,mut _4: [i64; 5],mut _5: usize,mut _6: f32,mut _7: i8,mut _8: u32,mut _9: [i64; 5],mut _10: [i64; 5]) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _11: *mut f64;
let _12: i8;
let _13: (*const u8, isize, *const &'static i8, *mut bool);
let _14: i8;
let _15: Adt51;
let _16: [u32; 5];
let _17: Adt55;
let _18: [usize; 7];
let _19: (u64,);
let _20: i128;
let _21: char;
let _22: [i128; 6];
let _23: (char, i32, [i32; 1]);
let _24: char;
let _25: [i8; 6];
let _26: ([i32; 1],);
let _27: u16;
let _28: usize;
let _29: &'static i8;
let _30: f64;
let _31: Adt48;
let _32: [i64; 5];
let _33: isize;
let _34: ();
let _35: ();
{
_7 = (-76_i8);
_8 = 688640449_u32 | 106753750_u32;
_3 = true as usize;
_5 = _6 as usize;
RET = [false];
Goto(bb1)
}
bb1 = {
_13.1 = 9223372036854775807_isize >> _7;
_6 = 152879628939967876631095124582991129864_i128 as f32;
_10 = _9;
_1 = [(-8408822688654142885_i64),(-7967540726209038156_i64),(-8075657102708373365_i64),6889838676721529298_i64,5878642773638552216_i64];
_6 = (-809067499_i32) as f32;
_9 = _2;
match _7 {
340282366920938463463374607431768211380 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_12 = !_7;
_6 = (-10369_i16) as f32;
_1 = _10;
_2 = [(-171046702944339910_i64),7356591236219385991_i64,(-2713908644220921513_i64),(-2762077303198898426_i64),(-2872287484724680598_i64)];
_6 = 384035850_i32 as f32;
_3 = 23928_i16 as usize;
RET = [true];
_2 = _9;
_14 = '\u{575d8}' as i8;
_6 = 1747791158_i32 as f32;
_2 = _10;
Goto(bb4)
}
bb4 = {
_5 = _3 | _3;
_10 = _9;
_13.1 = !(-9223372036854775808_isize);
_5 = !_3;
_3 = _5 + _5;
_12 = !_14;
_16 = [_8,_8,_8,_8,_8];
_9 = [(-1401675409453419613_i64),(-4940725004798261254_i64),1878879674692403221_i64,(-8830785904863014964_i64),(-7377938831475269378_i64)];
_6 = _14 as f32;
_14 = _12;
Goto(bb5)
}
bb5 = {
_17.fld2 = [_8,_8,_8,_8,_8];
_13.1 = _6 as isize;
_18 = [_5,_5,_3,_3,_3,_3,_5];
_2 = [(-1704349964895043048_i64),(-8661402997086455036_i64),8006391167727716180_i64,(-8556671868379505034_i64),(-2667673712813537590_i64)];
_15 = Adt51::Variant0 { fld0: _7 };
RET = [false];
_17.fld1 = [Field::<i8>(Variant(_15, 0), 0),_14,_12,_12,_12,Field::<i8>(Variant(_15, 0), 0),_7,_12];
_18 = [_3,_3,_5,_5,_5,_3,_5];
_19.0 = (-7110_i16) as u64;
_22 = [(-50780175426972256302081384513857135616_i128),94355196730413463325535513764523588668_i128,49996550082949609174283799727483098236_i128,(-78748133476672060392888216917335144693_i128),(-93170782485979779300880381135215814148_i128),(-76958109871967232699609049601008318519_i128)];
_22 = [(-12015483946560605828025632161295226402_i128),(-91712100183038702925769082017412019369_i128),(-3192499316786113095502276034787563321_i128),(-145534537005043632361732052116773273778_i128),155272945257717444233122323530441925158_i128,127762016893764181705912164806109230268_i128];
_8 = _13.1 as u32;
_18 = [_3,_5,_5,_3,_3,_5,_5];
_9 = _10;
_19.0 = 41508_u16 as u64;
_10 = [6706239777818866538_i64,(-1832770750562136262_i64),(-580516955706062448_i64),8813815859416454155_i64,5419045762476164050_i64];
_23.0 = '\u{a16fe}';
_23.2 = [(-352192362_i32)];
SetDiscriminant(_15, 0);
_5 = _3 & _3;
_23.1 = 1158058396_i32 + (-400578609_i32);
_8 = 3607460766_u32;
_17.fld3 = [_23.1];
Goto(bb6)
}
bb6 = {
_21 = _23.0;
_17.fld1 = [_12,_7,_7,_14,_7,_7,_14,_12];
_1 = _9;
_12 = _14;
_15 = Adt51::Variant0 { fld0: _14 };
RET = [false];
_9 = _10;
_23.2 = _17.fld3;
Goto(bb7)
}
bb7 = {
_15 = Adt51::Variant0 { fld0: _14 };
_26 = (_23.2,);
_3 = _5;
_7 = Field::<i8>(Variant(_15, 0), 0) * Field::<i8>(Variant(_15, 0), 0);
_12 = -_7;
_20 = 57_u8 as i128;
_19 = (13244140173158518766_u64,);
_18 = [_5,_3,_5,_5,_3,_5,_3];
_7 = _13.1 as i8;
_2 = _10;
RET = [true];
_23.2 = _17.fld3;
_24 = _23.0;
_5 = _3 ^ _3;
RET = [false];
Call(_17.fld3 = fn2(_2, _17.fld2, _23.2, _23.1, _20, _9, _10, _3, _26.0, _9, _12, _1, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = _3;
_19 = (13868548394536785591_u64,);
_10 = [(-3104917444527898805_i64),1085399583858797134_i64,(-6199381280718676057_i64),185763614474621180_i64,7930341491482184558_i64];
_23.1 = 997311371_i32 >> _12;
_29 = &_7;
_19.0 = 4476779362763411067_u64 - 3107469188477574767_u64;
_9 = _10;
_4 = [(-4283849549375074173_i64),6459172963886327244_i64,1674884179042274910_i64,857226261302916031_i64,5021236814733204331_i64];
_13.2 = core::ptr::addr_of!(_29);
_25 = [(*_29),_12,(*_29),_12,_14,Field::<i8>(Variant(_15, 0), 0)];
_15 = Adt51::Variant0 { fld0: _14 };
_12 = _13.1 as i8;
_24 = _23.0;
SetDiscriminant(_15, 1);
_11 = core::ptr::addr_of_mut!(_30);
place!(Field::<(char, i32, [i32; 1])>(Variant(_15, 1), 3)) = (_21, _23.1, _17.fld3);
match _8 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
3607460766 => bb15,
_ => bb14
}
}
bb9 = {
_15 = Adt51::Variant0 { fld0: _14 };
_26 = (_23.2,);
_3 = _5;
_7 = Field::<i8>(Variant(_15, 0), 0) * Field::<i8>(Variant(_15, 0), 0);
_12 = -_7;
_20 = 57_u8 as i128;
_19 = (13244140173158518766_u64,);
_18 = [_5,_3,_5,_5,_3,_5,_3];
_7 = _13.1 as i8;
_2 = _10;
RET = [true];
_23.2 = _17.fld3;
_24 = _23.0;
_5 = _3 ^ _3;
RET = [false];
Call(_17.fld3 = fn2(_2, _17.fld2, _23.2, _23.1, _20, _9, _10, _3, _26.0, _9, _12, _1, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_21 = _23.0;
_17.fld1 = [_12,_7,_7,_14,_7,_7,_14,_12];
_1 = _9;
_12 = _14;
_15 = Adt51::Variant0 { fld0: _14 };
RET = [false];
_9 = _10;
_23.2 = _17.fld3;
Goto(bb7)
}
bb11 = {
_17.fld2 = [_8,_8,_8,_8,_8];
_13.1 = _6 as isize;
_18 = [_5,_5,_3,_3,_3,_3,_5];
_2 = [(-1704349964895043048_i64),(-8661402997086455036_i64),8006391167727716180_i64,(-8556671868379505034_i64),(-2667673712813537590_i64)];
_15 = Adt51::Variant0 { fld0: _7 };
RET = [false];
_17.fld1 = [Field::<i8>(Variant(_15, 0), 0),_14,_12,_12,_12,Field::<i8>(Variant(_15, 0), 0),_7,_12];
_18 = [_3,_3,_5,_5,_5,_3,_5];
_19.0 = (-7110_i16) as u64;
_22 = [(-50780175426972256302081384513857135616_i128),94355196730413463325535513764523588668_i128,49996550082949609174283799727483098236_i128,(-78748133476672060392888216917335144693_i128),(-93170782485979779300880381135215814148_i128),(-76958109871967232699609049601008318519_i128)];
_22 = [(-12015483946560605828025632161295226402_i128),(-91712100183038702925769082017412019369_i128),(-3192499316786113095502276034787563321_i128),(-145534537005043632361732052116773273778_i128),155272945257717444233122323530441925158_i128,127762016893764181705912164806109230268_i128];
_8 = _13.1 as u32;
_18 = [_3,_5,_5,_3,_3,_5,_5];
_9 = _10;
_19.0 = 41508_u16 as u64;
_10 = [6706239777818866538_i64,(-1832770750562136262_i64),(-580516955706062448_i64),8813815859416454155_i64,5419045762476164050_i64];
_23.0 = '\u{a16fe}';
_23.2 = [(-352192362_i32)];
SetDiscriminant(_15, 0);
_5 = _3 & _3;
_23.1 = 1158058396_i32 + (-400578609_i32);
_8 = 3607460766_u32;
_17.fld3 = [_23.1];
Goto(bb6)
}
bb12 = {
_5 = _3 | _3;
_10 = _9;
_13.1 = !(-9223372036854775808_isize);
_5 = !_3;
_3 = _5 + _5;
_12 = !_14;
_16 = [_8,_8,_8,_8,_8];
_9 = [(-1401675409453419613_i64),(-4940725004798261254_i64),1878879674692403221_i64,(-8830785904863014964_i64),(-7377938831475269378_i64)];
_6 = _14 as f32;
_14 = _12;
Goto(bb5)
}
bb13 = {
_12 = !_7;
_6 = (-10369_i16) as f32;
_1 = _10;
_2 = [(-171046702944339910_i64),7356591236219385991_i64,(-2713908644220921513_i64),(-2762077303198898426_i64),(-2872287484724680598_i64)];
_6 = 384035850_i32 as f32;
_3 = 23928_i16 as usize;
RET = [true];
_2 = _9;
_14 = '\u{575d8}' as i8;
_6 = 1747791158_i32 as f32;
_2 = _10;
Goto(bb4)
}
bb14 = {
Return()
}
bb15 = {
(*_11) = 42923_u16 as f64;
_28 = _5 & _3;
_13.1 = (-9223372036854775808_isize);
_7 = _12;
place!(Field::<(char, i32, [i32; 1])>(Variant(_15, 1), 3)).1 = _23.1;
place!(Field::<(f32, bool)>(Variant(_15, 1), 4)).0 = _6;
(*_11) = _20 as f64;
_17.fld1 = [_14,_12,_7,_14,_7,_7,_12,_7];
_24 = _21;
_23.1 = Field::<(char, i32, [i32; 1])>(Variant(_15, 1), 3).1 + Field::<(char, i32, [i32; 1])>(Variant(_15, 1), 3).1;
_14 = !_7;
_5 = _23.1 as usize;
place!(Field::<(f32, bool)>(Variant(_15, 1), 4)).0 = -_6;
_17.fld2 = _16;
_27 = _21 as u16;
_13.2 = core::ptr::addr_of!(_29);
_15 = Adt51::Variant0 { fld0: _12 };
_21 = _24;
_3 = !_5;
_2 = [2182613007113099950_i64,2976217681877551435_i64,1662454631178818434_i64,(-499774093841832652_i64),1966865874294945063_i64];
_23 = (_21, 33790321_i32, _26.0);
_23 = (_24, 1803046094_i32, _26.0);
RET = [false];
_23.0 = _24;
_7 = _12 & Field::<i8>(Variant(_15, 0), 0);
SetDiscriminant(_15, 0);
(*_11) = _5 as f64;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(1_usize, 23_usize, Move(_23), 14_usize, Move(_14), 24_usize, Move(_24), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(1_usize, 19_usize, Move(_19), 28_usize, Move(_28), 26_usize, Move(_26), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(1_usize, 22_usize, Move(_22), 21_usize, Move(_21), 4_usize, Move(_4), 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [i64; 5],mut _2: [u32; 5],mut _3: [i32; 1],mut _4: i32,mut _5: i128,mut _6: [i64; 5],mut _7: [i64; 5],mut _8: usize,mut _9: [i32; 1],mut _10: [i64; 5],mut _11: i8,mut _12: [i64; 5],mut _13: usize) -> [i32; 1] {
mir! {
type RET = [i32; 1];
let _14: usize;
let _15: f64;
let _16: bool;
let _17: (f32, bool);
let _18: u128;
let _19: [usize; 5];
let _20: [isize; 7];
let _21: char;
let _22: u16;
let _23: char;
let _24: isize;
let _25: f64;
let _26: ();
let _27: ();
{
_2 = [3782341124_u32,1715304542_u32,1158320340_u32,3323063250_u32,1831039497_u32];
_1 = [(-1329314349823896963_i64),7819854948702321615_i64,8796898730612197027_i64,(-1470131202558024070_i64),6600401005992341423_i64];
_2 = [1954964358_u32,3892311055_u32,881624287_u32,2332442113_u32,1063192877_u32];
_11 = 228_u8 as i8;
_6 = [6042575099867107840_i64,4893094050415886260_i64,7205944015064436017_i64,(-2623791593670480228_i64),(-9081864645713415080_i64)];
_9 = _3;
RET = _9;
_1 = _10;
RET = [_4];
_12 = [7284606031061138343_i64,(-23382608114432438_i64),7515839828527307904_i64,(-7466777735696977899_i64),(-2555779942502159372_i64)];
_7 = [(-3011147646978419586_i64),1544968635317434022_i64,703670303029769577_i64,(-4922815233259656434_i64),(-2996270062400992058_i64)];
_12 = [(-2915888551572519551_i64),2481750389752608153_i64,4845365602858111399_i64,(-8144418758954408598_i64),8489354711915856481_i64];
_3 = [_4];
_4 = (-9223372036854775808_isize) as i32;
_6 = _10;
_8 = _13 - _13;
_8 = !_13;
_1 = _10;
_7 = [1317553760564740375_i64,6582030547122354265_i64,(-7954873613654118440_i64),313544460984609550_i64,3274065197607783202_i64];
_3 = _9;
_14 = _5 as usize;
_4 = -616838948_i32;
_13 = _8 - _8;
_16 = false | false;
_6 = [(-4106867625406992467_i64),7526977105457987290_i64,3283087497175511619_i64,5977063419939747449_i64,6287591461259431670_i64];
_5 = (-39284551747641115948984407291568749240_i128);
_7 = [1735203388862067125_i64,(-5521347564297646_i64),(-3014945855597125013_i64),(-4825841227813033794_i64),6240551698913363372_i64];
_7 = [5151802934108709244_i64,(-3070533311444945680_i64),3449906880020723344_i64,25361681194827653_i64,5476750252971838752_i64];
Call(_12 = fn3(_13, _9, _8, _1, _6, _13, _13, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_4];
_1 = _12;
_13 = _8;
_12 = [4928370598514613145_i64,(-2445258235325185453_i64),(-6827234467255505221_i64),4785615282173193076_i64,6838094744462954502_i64];
_2 = [352178213_u32,1092570817_u32,1634320365_u32,3195942919_u32,1493698872_u32];
_15 = _13 as f64;
_16 = false;
_15 = 15579233088722468563_u64 as f64;
_6 = [6711834762372988529_i64,(-1875198651863092880_i64),(-1731332591266697787_i64),5318138057793987146_i64,6757041510234402798_i64];
_4 = (-825990809_i32);
_1 = [7688686517056377430_i64,5915492068401061251_i64,2027044695047119716_i64,2778627835899475656_i64,(-2693239551493153333_i64)];
_2 = [897303124_u32,1463210533_u32,55264348_u32,1691177334_u32,3123401781_u32];
_16 = true ^ false;
_5 = _4 as i128;
_3 = [_4];
_11 = (-47_i8) * (-45_i8);
_6 = [4171778399923068497_i64,(-1407916839112439601_i64),5015796611342946100_i64,5951530988368810812_i64,(-6544655368836160683_i64)];
_8 = _13;
_6 = [3116899964724132117_i64,8590962943501040060_i64,4978543925475473983_i64,(-6928215050455875540_i64),6183233200892348403_i64];
_1 = _7;
match _4 {
0 => bb2,
1 => bb3,
340282366920938463463374607430942220647 => bb5,
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
RET = _9;
_10 = [9088392222953313091_i64,(-4398212433324174446_i64),276967569796694420_i64,(-986838537928615791_i64),(-708423845871408465_i64)];
_9 = [_4];
_6 = [3204719845946165158_i64,6617689716574920023_i64,(-5779199018090821633_i64),(-2074144993104539420_i64),8229753007600953290_i64];
Call(_4 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5 = _15 as i128;
_1 = [657630158586501011_i64,317145867186269732_i64,1909836894519323575_i64,(-7748275254393771602_i64),(-2041850529155990954_i64)];
_3 = [_4];
_15 = 9048316205874868036_i64 as f64;
_17.0 = 1291_u16 as f32;
_17.1 = _4 < _4;
_4 = 140425970172060887701917768738037141032_u128 as i32;
_8 = 87_u8 as usize;
_2 = [250131924_u32,1685190402_u32,1752328111_u32,283942861_u32,111597364_u32];
_16 = _17.1 | _17.1;
_1 = [(-7363063348309500576_i64),(-7363399736930944426_i64),2371917295963083377_i64,1471934359643614914_i64,(-8899142469273745683_i64)];
_14 = _13;
_16 = !_17.1;
_17.0 = 2414719582053240702_i64 as f32;
Goto(bb7)
}
bb7 = {
RET = [_4];
_3 = [_4];
_18 = 169429914541309390166680538658931483449_u128 | 210804272245366757940088326751715566086_u128;
_7 = [(-1511078064619362206_i64),(-8027462748447780985_i64),(-8249955684960275587_i64),(-3090963746778162325_i64),(-9149180735025881913_i64)];
_3 = [_4];
_1 = [(-5978046320158467209_i64),5752984259368441970_i64,434667710206682308_i64,2449332869340137242_i64,1907818290283415980_i64];
RET = _9;
_9 = [_4];
_5 = (-115033613423780262425336178550664078151_i128);
_1 = [6161366826259807996_i64,(-5344227845656962727_i64),(-4391493624563481866_i64),(-7372071689503076296_i64),(-3307342792484243075_i64)];
Goto(bb8)
}
bb8 = {
_17.1 = _16 & _16;
_19 = [_14,_13,_14,_14,_13];
_6 = [(-2637369192463901801_i64),6078946383182753185_i64,(-2887248336492738143_i64),4448502701065982981_i64,(-3204938938660183057_i64)];
_11 = 109_i8;
_2 = [484894711_u32,2013098686_u32,3301111047_u32,2777579450_u32,4133572760_u32];
match _11 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb10,
4 => bb11,
109 => bb13,
_ => bb12
}
}
bb9 = {
RET = [_4];
_3 = [_4];
_18 = 169429914541309390166680538658931483449_u128 | 210804272245366757940088326751715566086_u128;
_7 = [(-1511078064619362206_i64),(-8027462748447780985_i64),(-8249955684960275587_i64),(-3090963746778162325_i64),(-9149180735025881913_i64)];
_3 = [_4];
_1 = [(-5978046320158467209_i64),5752984259368441970_i64,434667710206682308_i64,2449332869340137242_i64,1907818290283415980_i64];
RET = _9;
_9 = [_4];
_5 = (-115033613423780262425336178550664078151_i128);
_1 = [6161366826259807996_i64,(-5344227845656962727_i64),(-4391493624563481866_i64),(-7372071689503076296_i64),(-3307342792484243075_i64)];
Goto(bb8)
}
bb10 = {
_5 = _15 as i128;
_1 = [657630158586501011_i64,317145867186269732_i64,1909836894519323575_i64,(-7748275254393771602_i64),(-2041850529155990954_i64)];
_3 = [_4];
_15 = 9048316205874868036_i64 as f64;
_17.0 = 1291_u16 as f32;
_17.1 = _4 < _4;
_4 = 140425970172060887701917768738037141032_u128 as i32;
_8 = 87_u8 as usize;
_2 = [250131924_u32,1685190402_u32,1752328111_u32,283942861_u32,111597364_u32];
_16 = _17.1 | _17.1;
_1 = [(-7363063348309500576_i64),(-7363399736930944426_i64),2371917295963083377_i64,1471934359643614914_i64,(-8899142469273745683_i64)];
_14 = _13;
_16 = !_17.1;
_17.0 = 2414719582053240702_i64 as f32;
Goto(bb7)
}
bb11 = {
RET = _9;
_10 = [9088392222953313091_i64,(-4398212433324174446_i64),276967569796694420_i64,(-986838537928615791_i64),(-708423845871408465_i64)];
_9 = [_4];
_6 = [3204719845946165158_i64,6617689716574920023_i64,(-5779199018090821633_i64),(-2074144993104539420_i64),8229753007600953290_i64];
Call(_4 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_22 = 36768_u16;
_5 = 85342730515215506600696183914911136326_i128 + 38521191911675734652907708685071983465_i128;
RET = _9;
_18 = !112580695991451007606450087043320185448_u128;
_17.0 = 11_u8 as f32;
RET = _9;
_8 = _15 as usize;
_10 = [882922524392100770_i64,5623716123935284236_i64,(-310227516314678095_i64),1880846338671392055_i64,(-790021124841900915_i64)];
_5 = _18 as i128;
Call(RET = core::intrinsics::transmute(_9), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17.0 = _11 as f32;
_3 = [_4];
_22 = 33107_u16 << _5;
_19 = [_14,_13,_13,_14,_14];
_9 = _3;
_14 = _13;
_9 = [_4];
_21 = '\u{b4ec2}';
_23 = _21;
RET = [_4];
_13 = _14;
_20 = [18_isize,(-30_isize),(-9223372036854775808_isize),68_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_25 = -_15;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(2_usize, 12_usize, Move(_12), 13_usize, Move(_13), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(2_usize, 23_usize, Move(_23), 7_usize, Move(_7), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(2_usize, 5_usize, Move(_5), 2_usize, Move(_2), 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: usize,mut _2: [i32; 1],mut _3: usize,mut _4: [i64; 5],mut _5: [i64; 5],mut _6: usize,mut _7: usize,mut _8: i128) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _9: f64;
let _10: bool;
let _11: [bool; 1];
let _12: *const usize;
let _13: Adt55;
let _14: [i32; 1];
let _15: (u64,);
let _16: [usize; 5];
let _17: [usize; 7];
let _18: [usize; 7];
let _19: i16;
let _20: f64;
let _21: u128;
let _22: i32;
let _23: isize;
let _24: [bool; 1];
let _25: i16;
let _26: f32;
let _27: *mut *mut f64;
let _28: [i128; 6];
let _29: usize;
let _30: char;
let _31: ();
let _32: ();
{
RET = [1473511084849972855_i64,3990391708812010709_i64,(-1157783593553124342_i64),4500186253251430965_i64,(-3225626893291359341_i64)];
_4 = [(-5254246862421105532_i64),7516834422758585700_i64,7165235638672354095_i64,743254907482717127_i64,5326297019062704409_i64];
_1 = !_7;
_5 = [3543683626065044201_i64,5033973713546689042_i64,1746313327863797907_i64,(-6268314508338026046_i64),1967911171188538824_i64];
_4 = [(-8240898766464955930_i64),(-3464774711927832604_i64),7944033604845399182_i64,8362234815908333168_i64,6158578801061224248_i64];
_5 = [3053444073770951413_i64,5879309058400437798_i64,3860528933779301192_i64,(-8313275522719503170_i64),(-8494130666019849743_i64)];
_2 = [(-323753676_i32)];
_8 = -68360287702396565785441617695373372796_i128;
_2 = [1270091606_i32];
_9 = 20_isize as f64;
_1 = 307105586263308490004143381640295075951_u128 as usize;
_5 = [4096014592514545097_i64,(-3179624579858111320_i64),(-1318517481953589347_i64),4150409327400268895_i64,1111890536624027058_i64];
_4 = [2055304880291274629_i64,(-3094995545699322876_i64),7866484154445430425_i64,2173915847285032497_i64,(-7935407821641466321_i64)];
_9 = 42_i8 as f64;
_10 = false;
_2 = [236820186_i32];
RET = [(-5462833102405609599_i64),(-168411186502221673_i64),7444500625595307382_i64,(-4460030230706365492_i64),5547530749706616981_i64];
_3 = _7;
_1 = '\u{50afa}' as usize;
_7 = (-1134778285_i32) as usize;
_1 = !_3;
RET = _4;
_2 = [1332626428_i32];
_11 = [_10];
_10 = false;
_3 = _1;
RET = [(-4429349151629690388_i64),7862570816092529821_i64,7755700292000526616_i64,(-2682221899671125313_i64),8793454185078133375_i64];
RET = [3003826022477936605_i64,(-8713176292557284111_i64),6633266055023090095_i64,1467294386206895001_i64,1678008881154830692_i64];
Call(_2 = fn4(_3, _6, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_1;
_13.fld2 = [2378227029_u32,2495159045_u32,92911611_u32,3254661850_u32,2405641593_u32];
_8 = -(-80713685262338311849102757948497969478_i128);
_3 = !_6;
Goto(bb2)
}
bb2 = {
_14 = _2;
_13.fld3 = _14;
_6 = _3;
_13.fld2 = [2914809172_u32,3758190038_u32,2352161950_u32,3163387294_u32,3855070212_u32];
_13.fld2 = [2237514674_u32,3381410938_u32,739569785_u32,494447961_u32,939930083_u32];
RET = [(-1646164010003686024_i64),(-3723084014043408072_i64),2542636299673956700_i64,7117567018372737867_i64,6070447688233214401_i64];
_15.0 = !6098594492278133819_u64;
_16 = [_1,_1,_3,_6,_3];
_13.fld0 = core::ptr::addr_of_mut!(_10);
_6 = _3;
_4 = [1524706629278255733_i64,(-326055236151551758_i64),(-8163266623438330986_i64),(-7425315780740811344_i64),4485836228105332466_i64];
_4 = [4669431768175950475_i64,(-3732467582898108138_i64),(-3029682438437986968_i64),(-2354563275725850785_i64),(-4063459534697767095_i64)];
_12 = core::ptr::addr_of!(_7);
_13.fld3 = [(-862994071_i32)];
_1 = (-5981490077776472463_i64) as usize;
_11 = [_10];
_4 = [(-8547273099634146310_i64),884796085542433359_i64,1046541414242491865_i64,(-9215149433457255837_i64),(-3218481503105236980_i64)];
_6 = _3 | _3;
_17 = [_6,_3,_6,_3,_6,_1,_6];
_5 = [(-537999972713612357_i64),(-5545610766913053759_i64),955216904688241661_i64,8313123320986306710_i64,2804990346506600140_i64];
_7 = 1078260060_u32 as usize;
Goto(bb3)
}
bb3 = {
_15 = (8750022973233005372_u64,);
_21 = 226596817117236340721847615073268229750_u128;
_6 = (*_12) * _3;
_20 = -_9;
_13.fld0 = core::ptr::addr_of_mut!(_10);
RET = _4;
RET = [(-4487337402494649972_i64),(-7544785288498053871_i64),3146838236409860317_i64,3064615217491753849_i64,656145103707793828_i64];
_22 = 173317637_i32 - (-401428175_i32);
_19 = 3542_i16 + 27048_i16;
_22 = (-1223475714_i32);
_18 = _17;
RET = [8217777111650053918_i64,(-3178977642574931867_i64),4312468461252736801_i64,(-6472308559353113766_i64),2368105817621158377_i64];
_22 = 1553795215_i32;
_18 = [_6,_3,_3,(*_12),_3,_6,_6];
_19 = (-5736_i16);
RET = _4;
_13.fld3 = [_22];
_16 = [_1,_6,_3,_6,_3];
match _19 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768205720 => bb9,
_ => bb8
}
}
bb4 = {
_14 = _2;
_13.fld3 = _14;
_6 = _3;
_13.fld2 = [2914809172_u32,3758190038_u32,2352161950_u32,3163387294_u32,3855070212_u32];
_13.fld2 = [2237514674_u32,3381410938_u32,739569785_u32,494447961_u32,939930083_u32];
RET = [(-1646164010003686024_i64),(-3723084014043408072_i64),2542636299673956700_i64,7117567018372737867_i64,6070447688233214401_i64];
_15.0 = !6098594492278133819_u64;
_16 = [_1,_1,_3,_6,_3];
_13.fld0 = core::ptr::addr_of_mut!(_10);
_6 = _3;
_4 = [1524706629278255733_i64,(-326055236151551758_i64),(-8163266623438330986_i64),(-7425315780740811344_i64),4485836228105332466_i64];
_4 = [4669431768175950475_i64,(-3732467582898108138_i64),(-3029682438437986968_i64),(-2354563275725850785_i64),(-4063459534697767095_i64)];
_12 = core::ptr::addr_of!(_7);
_13.fld3 = [(-862994071_i32)];
_1 = (-5981490077776472463_i64) as usize;
_11 = [_10];
_4 = [(-8547273099634146310_i64),884796085542433359_i64,1046541414242491865_i64,(-9215149433457255837_i64),(-3218481503105236980_i64)];
_6 = _3 | _3;
_17 = [_6,_3,_6,_3,_6,_1,_6];
_5 = [(-537999972713612357_i64),(-5545610766913053759_i64),955216904688241661_i64,8313123320986306710_i64,2804990346506600140_i64];
_7 = 1078260060_u32 as usize;
Goto(bb3)
}
bb5 = {
_3 = !_1;
_13.fld2 = [2378227029_u32,2495159045_u32,92911611_u32,3254661850_u32,2405641593_u32];
_8 = -(-80713685262338311849102757948497969478_i128);
_3 = !_6;
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
RET = [7418835904409194935_i64,5501419223979811841_i64,6728038301504882312_i64,(-4727889908305990799_i64),1637344663331568700_i64];
_3 = !_6;
_10 = true & true;
_13.fld1 = [87_i8,73_i8,(-5_i8),103_i8,(-106_i8),109_i8,(-66_i8),(-1_i8)];
_3 = (*_12);
_15 = (2210321889554405522_u64,);
_13.fld2 = [3553832337_u32,4068315717_u32,1382454071_u32,1694115842_u32,820092179_u32];
_23 = (-9223372036854775808_isize);
(*_12) = _6 & _6;
_19 = -26150_i16;
_1 = !(*_12);
_13.fld3 = [_22];
_23 = 9223372036854775807_isize >> _1;
_23 = 9223372036854775807_isize;
_11 = [_10];
_17 = [_1,_7,_7,(*_12),(*_12),_1,(*_12)];
Goto(bb10)
}
bb10 = {
_13.fld3 = [_22];
_13.fld1 = [(-62_i8),(-97_i8),11_i8,105_i8,(-38_i8),(-36_i8),43_i8,3_i8];
_19 = -(-103_i16);
_19 = _20 as i16;
RET = _4;
_2 = [_22];
_21 = 313556998147523030799595743458939359332_u128 + 218221030062277159984058230032185083497_u128;
_1 = 12521_u16 as usize;
_7 = _3;
_24 = [_10];
_4 = [(-5345239658215046588_i64),(-2276576894037519517_i64),8154474023646292915_i64,(-8783442591794037136_i64),7989989630413069954_i64];
_8 = (-46058749611147887323024043524731142267_i128) << _6;
_15 = (10483129971651147476_u64,);
_19 = -25134_i16;
_22 = _23 as i32;
_6 = _1 + (*_12);
_15.0 = _21 as u64;
_19 = 4147714914_u32 as i16;
_8 = 116969828418095695596405125724872694467_i128 + 93471295552549477411620035819189565604_i128;
_4 = _5;
_1 = _3 >> _6;
_24 = _11;
_23 = !16_isize;
Goto(bb11)
}
bb11 = {
_13.fld2 = [4069513266_u32,1936172402_u32,1675897931_u32,344008723_u32,2063482896_u32];
_25 = _19 << _7;
_15.0 = 10566134693012104144_u64;
_20 = _9 * _9;
_12 = core::ptr::addr_of!(_7);
_13.fld2 = [1846698455_u32,2511244364_u32,164343530_u32,1405693927_u32,3364908644_u32];
_6 = !(*_12);
_14 = [_22];
_19 = !_25;
_13.fld3 = [_22];
RET = _4;
_7 = _1;
_15.0 = 9400323827390969338_u64 >> (*_12);
_12 = core::ptr::addr_of!(_1);
_13.fld0 = core::ptr::addr_of_mut!(_10);
_23 = (-64_isize) & 9223372036854775807_isize;
_2 = _14;
_30 = '\u{e0b6d}';
_13.fld1 = [(-117_i8),75_i8,19_i8,7_i8,36_i8,124_i8,40_i8,75_i8];
_24 = _11;
_20 = _9 + _9;
_22 = 943095032_i32;
match _22 {
0 => bb1,
1 => bb9,
2 => bb5,
3 => bb12,
943095032 => bb14,
_ => bb13
}
}
bb12 = {
_15 = (8750022973233005372_u64,);
_21 = 226596817117236340721847615073268229750_u128;
_6 = (*_12) * _3;
_20 = -_9;
_13.fld0 = core::ptr::addr_of_mut!(_10);
RET = _4;
RET = [(-4487337402494649972_i64),(-7544785288498053871_i64),3146838236409860317_i64,3064615217491753849_i64,656145103707793828_i64];
_22 = 173317637_i32 - (-401428175_i32);
_19 = 3542_i16 + 27048_i16;
_22 = (-1223475714_i32);
_18 = _17;
RET = [8217777111650053918_i64,(-3178977642574931867_i64),4312468461252736801_i64,(-6472308559353113766_i64),2368105817621158377_i64];
_22 = 1553795215_i32;
_18 = [_6,_3,_3,(*_12),_3,_6,_6];
_19 = (-5736_i16);
RET = _4;
_13.fld3 = [_22];
_16 = [_1,_6,_3,_6,_3];
match _19 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768205720 => bb9,
_ => bb8
}
}
bb13 = {
Return()
}
bb14 = {
_6 = !(*_12);
_29 = (*_12);
_5 = [(-4057445824554067629_i64),(-2212551669132509775_i64),(-3993274646222714574_i64),(-1653167572917691886_i64),8606873236591311841_i64];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(3_usize, 24_usize, Move(_24), 1_usize, Move(_1), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(3_usize, 10_usize, Move(_10), 14_usize, Move(_14), 22_usize, Move(_22), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(3_usize, 2_usize, Move(_2), 30_usize, Move(_30), 3_usize, Move(_3), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize) -> [i32; 1] {
mir! {
type RET = [i32; 1];
let _5: isize;
let _6: (i8, [bool; 1], i128);
let _7: isize;
let _8: f32;
let _9: u128;
let _10: ([i8; 6],);
let _11: [i32; 1];
let _12: f64;
let _13: u16;
let _14: [i32; 1];
let _15: f32;
let _16: isize;
let _17: [usize; 7];
let _18: i32;
let _19: Adt51;
let _20: i32;
let _21: Adt59;
let _22: (f32, [bool; 1], [isize; 7]);
let _23: [usize; 7];
let _24: f64;
let _25: [isize; 7];
let _26: f32;
let _27: usize;
let _28: isize;
let _29: isize;
let _30: u16;
let _31: i128;
let _32: *mut i8;
let _33: isize;
let _34: i64;
let _35: ();
let _36: ();
{
_4 = _2 * _1;
_1 = _3 + _4;
_3 = _1 + _1;
_4 = _3;
_4 = _1 ^ _3;
_1 = !_3;
_1 = _2 & _3;
_1 = _4 | _3;
RET = [73674271_i32];
_5 = !9223372036854775807_isize;
_6.2 = 49245484137291979195782947954258022132_i128;
_2 = !_3;
_8 = 6547998901143959621_i64 as f32;
_6.2 = 225063304181331765711418252965856208007_u128 as i128;
_3 = _6.2 as usize;
_6.0 = -(-113_i8);
_2 = _4;
_6.1 = [true];
_7 = _5 ^ _5;
RET = [(-967069452_i32)];
_6.2 = _1 as i128;
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_7 = -_5;
Goto(bb1)
}
bb1 = {
_6.2 = !(-53183733525069311911200997029319836999_i128);
_4 = true as usize;
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
RET = [1624829883_i32];
_8 = 110224240896444543586710228042833861722_u128 as f32;
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_9 = 240103499359307893709342893519892698579_u128;
_6.2 = 3485296193_u32 as i128;
_5 = _7 << _1;
_7 = _5;
_14 = [1679785973_i32];
_6.1 = [false];
_8 = 34229_u16 as f32;
_8 = _6.2 as f32;
Call(_6.1 = fn5(_1, _2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _2 << _7;
_15 = _7 as f32;
_7 = _5 + _5;
_6.1 = [true];
_5 = 7034_i16 as isize;
_13 = 44899_u16;
_7 = _9 as isize;
_15 = _8;
_11 = [1852404530_i32];
_2 = !_1;
_8 = _13 as f32;
_14 = [461558055_i32];
_1 = !_2;
_3 = _1;
_1 = !_3;
_12 = _8 as f64;
_7 = _9 as isize;
_9 = 12619006523694690201_u64 as u128;
_17 = [_2,_2,_1,_2,_2,_1,_3];
_6.2 = (-12812316485856592170569760804630443607_i128) + (-13684482351232987794292603334068801316_i128);
_11 = [644046723_i32];
RET = [(-2025390552_i32)];
_8 = -_15;
_13 = 28246_u16 | 19190_u16;
_7 = _5 | _5;
_14 = [84604384_i32];
Call(_13 = fn6(_1, _2, _2, _2, _2, _1, _2, _1, _1, _3, _2, _1, _3, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = 36616_u16;
_16 = _7 & _5;
_5 = _16;
_22.0 = _8;
_20 = 455252977_u32 as i32;
_13 = '\u{cbbd9}' as u16;
_6.1 = [true];
_11 = [_20];
_5 = _7;
_22.1 = _6.1;
_4 = _3;
_5 = 1438463694_u32 as isize;
_6.2 = -166285192481849248774263078634289595208_i128;
_4 = !_3;
_3 = _4;
_8 = _12 as f32;
_16 = -_7;
_7 = !_5;
_3 = '\u{5cad3}' as usize;
_23 = _17;
_25 = [_16,_16,_7,_16,_5,_16,_16];
Goto(bb4)
}
bb4 = {
_16 = 115_u8 as isize;
RET = [_20];
_23 = _17;
_22.0 = _20 as f32;
_8 = -_22.0;
_22.2 = _25;
_13 = _15 as u16;
_5 = 12196453944854579810_u64 as isize;
_27 = (-10909_i16) as usize;
_6.2 = (-112895194510115072468314459461228055833_i128) | (-163161464307589600030574062733214239489_i128);
_12 = _7 as f64;
_14 = [_20];
Call(_6 = fn8(_17, _2, _4, _2, _4, _23, _4, _1, _1, _2, _23, _4, _22, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = _20;
_22.0 = _15;
_9 = 147149537692219157216219351420741426698_u128 >> _4;
_9 = !309720794356774746438621080929333946375_u128;
_19 = Adt51::Variant0 { fld0: _6.0 };
place!(Field::<i8>(Variant(_19, 0), 0)) = 14005029400378804457_u64 as i8;
_6.2 = 52139142724714057715924419120964763369_i128 * 122867976847774372785824965692983640670_i128;
_8 = _15;
_29 = !_16;
_25 = _22.2;
_5 = _16 >> _1;
_6.2 = (-106745153361521978149645485121516832789_i128);
_13 = !4590_u16;
_8 = _15;
_6 = (Field::<i8>(Variant(_19, 0), 0), _22.1, (-9799457080739187886856303109622807029_i128));
match _6.2 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
330482909840199275576518304322145404427 => bb11,
_ => bb10
}
}
bb6 = {
_16 = 115_u8 as isize;
RET = [_20];
_23 = _17;
_22.0 = _20 as f32;
_8 = -_22.0;
_22.2 = _25;
_13 = _15 as u16;
_5 = 12196453944854579810_u64 as isize;
_27 = (-10909_i16) as usize;
_6.2 = (-112895194510115072468314459461228055833_i128) | (-163161464307589600030574062733214239489_i128);
_12 = _7 as f64;
_14 = [_20];
Call(_6 = fn8(_17, _2, _4, _2, _4, _23, _4, _1, _1, _2, _23, _4, _22, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_13 = 36616_u16;
_16 = _7 & _5;
_5 = _16;
_22.0 = _8;
_20 = 455252977_u32 as i32;
_13 = '\u{cbbd9}' as u16;
_6.1 = [true];
_11 = [_20];
_5 = _7;
_22.1 = _6.1;
_4 = _3;
_5 = 1438463694_u32 as isize;
_6.2 = -166285192481849248774263078634289595208_i128;
_4 = !_3;
_3 = _4;
_8 = _12 as f32;
_16 = -_7;
_7 = !_5;
_3 = '\u{5cad3}' as usize;
_23 = _17;
_25 = [_16,_16,_7,_16,_5,_16,_16];
Goto(bb4)
}
bb8 = {
_1 = _2 << _7;
_15 = _7 as f32;
_7 = _5 + _5;
_6.1 = [true];
_5 = 7034_i16 as isize;
_13 = 44899_u16;
_7 = _9 as isize;
_15 = _8;
_11 = [1852404530_i32];
_2 = !_1;
_8 = _13 as f32;
_14 = [461558055_i32];
_1 = !_2;
_3 = _1;
_1 = !_3;
_12 = _8 as f64;
_7 = _9 as isize;
_9 = 12619006523694690201_u64 as u128;
_17 = [_2,_2,_1,_2,_2,_1,_3];
_6.2 = (-12812316485856592170569760804630443607_i128) + (-13684482351232987794292603334068801316_i128);
_11 = [644046723_i32];
RET = [(-2025390552_i32)];
_8 = -_15;
_13 = 28246_u16 | 19190_u16;
_7 = _5 | _5;
_14 = [84604384_i32];
Call(_13 = fn6(_1, _2, _2, _2, _2, _1, _2, _1, _1, _3, _2, _1, _3, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_6.2 = !(-53183733525069311911200997029319836999_i128);
_4 = true as usize;
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
RET = [1624829883_i32];
_8 = 110224240896444543586710228042833861722_u128 as f32;
_10.0 = [_6.0,_6.0,_6.0,_6.0,_6.0,_6.0];
_9 = 240103499359307893709342893519892698579_u128;
_6.2 = 3485296193_u32 as i128;
_5 = _7 << _1;
_7 = _5;
_14 = [1679785973_i32];
_6.1 = [false];
_8 = 34229_u16 as f32;
_8 = _6.2 as f32;
Call(_6.1 = fn5(_1, _2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_5 = _9 as isize;
_20 = -_18;
_28 = _6.0 as isize;
_22.0 = -_15;
_7 = _28;
RET = [_18];
RET = _11;
_28 = !_7;
_7 = _12 as isize;
_26 = -_15;
Goto(bb12)
}
bb12 = {
_12 = 12045307766437080117_u64 as f64;
_6.2 = (-34934722404991367275745561499157584055_i128);
_16 = !_28;
_28 = _5;
RET = [_20];
_5 = !_7;
_22.1 = _6.1;
_5 = _16 << _4;
_5 = _7 >> _4;
_6.0 = 15185188029134256579_u64 as i8;
SetDiscriminant(_19, 0);
_31 = _6.2;
_24 = -_12;
_2 = !_4;
match _6.2 {
305347644515947096187629045932610627401 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_2 = _18 as usize;
_5 = false as isize;
_2 = _1;
_6.0 = 42_i8 >> _1;
_12 = _24;
_25 = [_28,_7,_28,_28,_16,_5,_16];
_23 = _17;
place!(Field::<i8>(Variant(_19, 0), 0)) = _6.0;
_32 = core::ptr::addr_of_mut!(_6.0);
(*_32) = !Field::<i8>(Variant(_19, 0), 0);
_11 = [_18];
SetDiscriminant(_19, 1);
_29 = (-8855242398575845449_i64) as isize;
RET = _11;
_8 = -_15;
place!(Field::<(f32, bool)>(Variant(_19, 1), 4)) = (_22.0, false);
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(4_usize, 10_usize, Move(_10), 28_usize, Move(_28), 9_usize, Move(_9), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(4_usize, 18_usize, Move(_18), 20_usize, Move(_20), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(4_usize, 17_usize, Move(_17), 27_usize, Move(_27), 3_usize, Move(_3), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: usize,mut _2: usize,mut _3: usize) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _4: [u32; 5];
let _5: isize;
let _6: u32;
let _7: [i128; 6];
let _8: [i8; 8];
let _9: f64;
let _10: (i8, [bool; 1], i128);
let _11: u64;
let _12: [u128; 8];
let _13: u64;
let _14: i64;
let _15: [i8; 6];
let _16: Adt55;
let _17: u128;
let _18: bool;
let _19: [bool; 1];
let _20: f32;
let _21: [i64; 5];
let _22: i16;
let _23: i128;
let _24: i64;
let _25: [u32; 5];
let _26: ();
let _27: ();
{
_2 = !_1;
_3 = 79_i8 as usize;
RET = [true];
_6 = 3772713510_u32 << _1;
_6 = 1907833545_u32;
_2 = _1;
_6 = (-6611625109478641378_i64) as u32;
_5 = (-70_isize) | 20_isize;
_3 = '\u{e81be}' as usize;
_5 = -(-108_isize);
RET = [false];
RET = [false];
_1 = _2 << _5;
_5 = 9223372036854775807_isize;
_7 = [123443027254911806685277473465991726595_i128,(-165261798293501105972755325696362381247_i128),161440179308190741533845999570784412935_i128,(-43925009314011917075881672222325834271_i128),60895396688439061718551205443343614281_i128,109440304524223035403741219447049258205_i128];
_1 = (-72367872241551145336837198528542323432_i128) as usize;
_1 = _2;
_1 = !_2;
_4 = [_6,_6,_6,_6,_6];
Goto(bb1)
}
bb1 = {
_8 = [96_i8,90_i8,(-12_i8),60_i8,60_i8,94_i8,(-13_i8),40_i8];
_6 = 56431_u16 as u32;
_8 = [(-65_i8),(-30_i8),(-49_i8),(-105_i8),(-48_i8),118_i8,102_i8,76_i8];
_9 = _2 as f64;
RET = [true];
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
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
_2 = !_1;
_4 = [_6,_6,_6,_6,_6];
_9 = 32147_i16 as f64;
_5 = !(-9223372036854775808_isize);
RET = [true];
_1 = _2 & _2;
_10 = (49_i8, RET, 19462003728140672135273394332406502655_i128);
_4 = [_6,_6,_6,_6,_6];
_2 = 14064_i16 as usize;
_3 = _10.2 as usize;
_5 = (-56_isize) - 9223372036854775807_isize;
_1 = !_2;
_4 = [_6,_6,_6,_6,_6];
_10.2 = 163911466291278795830005263697461647565_i128;
_3 = _1 | _1;
_4 = [_6,_6,_6,_6,_6];
_10 = (77_i8, RET, (-152600127236188962228934188098171487006_i128));
_7 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2];
_5 = 4443_i16 as isize;
_1 = !_3;
_10.0 = 20_i8;
RET = _10.1;
_8 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_2 = !_3;
_6 = 2823546321_u32;
_1 = _2 | _2;
_5 = 9223372036854775807_isize << _2;
_5 = !(-115_isize);
_9 = _6 as f64;
match _10.0 {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
20 => bb11,
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
_4 = [_6,_6,_6,_6,_6];
_6 = !2286665162_u32;
RET = _10.1;
_3 = _1 * _1;
_5 = 9223372036854775807_isize;
_10.1 = [false];
_12 = [282890414332071918716515145862311514434_u128,139737541807522750527320237446224062929_u128,70799014777238305337329670627027897878_u128,13234040843299691677404480408662367002_u128,282654161022134269400155658702321827271_u128,332231977033696163321379518476069315034_u128,226759498092015824387549436051134010429_u128,141349719765463892820282420274714785157_u128];
match _10.2 {
0 => bb7,
1 => bb2,
2 => bb8,
3 => bb4,
187682239684749501234440419333596724450 => bb12,
_ => bb5
}
}
bb12 = {
_2 = false as usize;
_11 = 13456108097087494477_u64 ^ 5343605786896228081_u64;
_12 = [161894923722004832392807077208825317773_u128,173958895744307736987793830950834407097_u128,176014018448708404911286924468382046487_u128,244286397627219388288952031949547054282_u128,231285038795011278469504508006884324932_u128,118146037186997203024354206198640915230_u128,313088697119780337002928397046563514958_u128,99283136053214859731583162486656723369_u128];
_14 = '\u{cbedf}' as i64;
_10 = ((-54_i8), RET, (-63754145726834589975035294271673543681_i128));
_10.2 = (-3610_i16) as i128;
RET = _10.1;
_14 = -(-898235102808056930_i64);
_11 = 3282072414552845706_u64 - 14933644802867382089_u64;
_3 = 63_u8 as usize;
_8 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb6,
340282366920938463463374607431768211402 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_10.2 = '\u{ccae4}' as i128;
_2 = _3 - _1;
_5 = 9223372036854775807_isize + 9223372036854775807_isize;
_10.1 = RET;
_10.2 = (-145795932052417723946091736713212393282_i128);
_15 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_3 = _2;
_22 = _1 as i16;
_20 = _11 as f32;
_12 = [184235524135454972880261379052970421509_u128,327497996813192460933736267161374642317_u128,174652594971808068820340400004119496733_u128,172082038678566904919907647437415909056_u128,142226494976918654212017104314096906606_u128,298749822309956517733543696092912000562_u128,242274302073604151188865142717668084923_u128,16534963402383426837293892052631111806_u128];
_3 = _1 >> _1;
_13 = _11 & _11;
_10.2 = 101556066098323956460028410124231297708_i128 & 100280459863589798881532511707524067539_i128;
_16.fld3 = [(-1684560025_i32)];
RET = [false];
_19 = [true];
_14 = (-829511705163567175_i64) << _22;
_22 = 11167_i16 | 32340_i16;
_16.fld2 = [_6,_6,_6,_6,_6];
_25 = [_6,_6,_6,_6,_6];
_10 = (75_i8, RET, 26171477923858565315124330644521773064_i128);
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(5_usize, 4_usize, Move(_4), 5_usize, Move(_5), 8_usize, Move(_8), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(5_usize, 25_usize, Move(_25), 14_usize, Move(_14), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize,mut _12: usize,mut _13: usize,mut _14: usize,mut _15: usize) -> u16 {
mir! {
type RET = u16;
let _16: (i8, [bool; 1], i128);
let _17: (f32, [bool; 1], [isize; 7]);
let _18: [i64; 5];
let _19: Adt48;
let _20: *mut *mut f64;
let _21: u32;
let _22: isize;
let _23: *const &'static i8;
let _24: ();
let _25: ();
{
_3 = !_10;
_12 = _14;
_8 = !_5;
_8 = _10 >> _2;
_16.0 = 84_i8 | 97_i8;
RET = 59221_u16;
_9 = _4;
_16.1 = [true];
_16.2 = !(-71217476387308799014911535665976735233_i128);
_13 = _12;
_8 = _4 ^ _10;
_17.2 = [76_isize,5_isize,9223372036854775807_isize,(-41_isize),76_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_18 = [(-6951415673220898336_i64),4856996259510768530_i64,312387366604189537_i64,(-3785832917977711538_i64),(-567411927120306716_i64)];
_7 = _4 << _10;
_4 = !_8;
_2 = 19428_i16 as usize;
_5 = _1;
_6 = _11;
_17.1 = _16.1;
_19 = Adt48::Variant2 { fld0: 249089373906119928858999056290011817979_u128,fld1: _16.0 };
_18 = [3594736017916621994_i64,(-2099077545781866817_i64),5707456132838653692_i64,(-7900488718627683232_i64),4330097239716735659_i64];
match RET {
59221 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_16.0 = Field::<i8>(Variant(_19, 2), 1) - Field::<i8>(Variant(_19, 2), 1);
_16 = (Field::<i8>(Variant(_19, 2), 1), _17.1, 39736045953311570772665749349564590197_i128);
_11 = _12;
_16.1 = [false];
_1 = _9 + _6;
_1 = !_5;
Goto(bb3)
}
bb3 = {
_17.0 = RET as f32;
_15 = _12;
_16.0 = -Field::<i8>(Variant(_19, 2), 1);
_16.0 = -Field::<i8>(Variant(_19, 2), 1);
place!(Field::<u128>(Variant(_19, 2), 0)) = !333820557656586545162470995452167027570_u128;
_14 = _11;
_11 = !_13;
_17.2 = [110_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-74_isize)];
_18 = [(-256890370862582649_i64),(-1149788118564867915_i64),(-6657607441344635325_i64),(-6719221736204031187_i64),5887276738293827443_i64];
_2 = _12;
_4 = _12 ^ _3;
_4 = _11 & _5;
_22 = 9223372036854775807_isize >> _12;
place!(Field::<u128>(Variant(_19, 2), 0)) = 198778128460134047939349611055576826900_u128;
_4 = !_9;
Call(_16 = fn7(_13, _22, _14, _10, _1, _6, _15, _3, _14, _10, _10, _2, _8, _8, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = !_5;
RET = 14384_u16;
_11 = _8;
_2 = 7450668148928221254_u64 as usize;
_4 = _6 | _8;
place!(Field::<i8>(Variant(_19, 2), 1)) = _22 as i8;
_9 = !_6;
_9 = _17.0 as usize;
_22 = (-11_isize) & (-38_isize);
_2 = 29930_i16 as usize;
_12 = _15 >> _15;
_2 = '\u{109608}' as usize;
_21 = 10208314657641693432_u64 as u32;
SetDiscriminant(_19, 0);
RET = 21138_u16 >> _6;
_21 = 4220798723_u32;
_11 = _17.0 as usize;
_18 = [7688361259237757312_i64,(-7717759624678195941_i64),3100556616781875071_i64,399863854865301622_i64,4445178111294475873_i64];
_1 = _14 & _3;
Goto(bb5)
}
bb5 = {
Call(_24 = dump_var(6_usize, 4_usize, Move(_4), 8_usize, Move(_8), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_24 = dump_var(6_usize, 7_usize, Move(_7), 11_usize, Move(_11), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_24 = dump_var(6_usize, 3_usize, Move(_3), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: usize,mut _2: isize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize,mut _12: usize,mut _13: usize,mut _14: usize,mut _15: usize) -> (i8, [bool; 1], i128) {
mir! {
type RET = (i8, [bool; 1], i128);
let _16: (f32, bool);
let _17: char;
let _18: i8;
let _19: *const &'static i8;
let _20: char;
let _21: isize;
let _22: [i32; 1];
let _23: isize;
let _24: bool;
let _25: [i128; 6];
let _26: Adt54;
let _27: Adt55;
let _28: char;
let _29: bool;
let _30: Adt53;
let _31: u8;
let _32: *mut [i8; 6];
let _33: f32;
let _34: ();
let _35: ();
{
RET.2 = 3485046582_u32 as i128;
_14 = _13;
_13 = 212_u8 as usize;
RET.1 = [true];
_3 = !_4;
RET.0 = (-106_i8);
_11 = _5 ^ _1;
_10 = _12 >> _3;
_17 = '\u{6ca7e}';
_10 = _1 ^ _5;
_18 = -RET.0;
_18 = RET.0;
_18 = RET.0;
_16.1 = !true;
_17 = '\u{a04fe}';
RET.2 = 62331140342919720719091588808830449337_i128 - (-169460846848156747464774177322738328370_i128);
Goto(bb1)
}
bb1 = {
_16.0 = 1420299191_i32 as f32;
_17 = '\u{c7799}';
_4 = !_5;
_3 = _9 + _9;
_4 = _3;
RET.0 = _18;
_3 = !_7;
_7 = !_11;
_15 = !_7;
_17 = '\u{de479}';
RET.0 = _18 | _18;
_9 = _4;
match _18 {
340282366920938463463374607431768211350 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET.1 = [_16.1];
_1 = _4;
_3 = 174_u8 as usize;
_16.0 = _2 as f32;
_11 = _7 + _8;
_4 = _11 - _10;
_11 = _8 >> _9;
_18 = !RET.0;
RET.1 = [_16.1];
_6 = 1770473844964574682_u64 as usize;
_7 = _1;
_20 = _17;
_13 = _1;
_2 = 19032_i16 as isize;
_24 = _16.1;
_3 = _5 | _12;
_16.0 = RET.0 as f32;
_22 = [(-1170995164_i32)];
RET.2 = -(-49833717552381612714029871883686923734_i128);
Goto(bb4)
}
bb4 = {
_4 = _2 as usize;
RET.1 = [_16.1];
_18 = !RET.0;
_27.fld1 = [_18,RET.0,_18,_18,RET.0,_18,_18,_18];
_27.fld0 = core::ptr::addr_of_mut!(_24);
_3 = !_9;
Call(_15 = core::intrinsics::bswap(_12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = _12;
RET.2 = (-123356713029875043037979277268465593681_i128) << _1;
_11 = !_14;
_10 = _7 << _4;
_4 = _10;
_1 = !_5;
_28 = _20;
_29 = _16.1;
_31 = 117_u8;
_12 = _8;
_6 = _31 as usize;
_12 = _29 as usize;
_31 = !172_u8;
_7 = _8;
_2 = (-9223372036854775808_isize) >> _4;
Goto(bb6)
}
bb6 = {
Call(_34 = dump_var(7_usize, 13_usize, Move(_13), 5_usize, Move(_5), 29_usize, Move(_29), 7_usize, Move(_7)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_34 = dump_var(7_usize, 10_usize, Move(_10), 9_usize, Move(_9), 1_usize, Move(_1), 22_usize, Move(_22)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_34 = dump_var(7_usize, 6_usize, Move(_6), 12_usize, Move(_12), 2_usize, Move(_2), 35_usize, _35), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [usize; 7],mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: [usize; 7],mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: [usize; 7],mut _12: usize,mut _13: (f32, [bool; 1], [isize; 7]),mut _14: usize) -> (i8, [bool; 1], i128) {
mir! {
type RET = (i8, [bool; 1], i128);
let _15: isize;
let _16: [i16; 4];
let _17: i32;
let _18: u8;
let _19: u64;
let _20: (f32, bool);
let _21: u8;
let _22: isize;
let _23: Adt62;
let _24: bool;
let _25: *mut *mut f64;
let _26: (f32, [bool; 1], [isize; 7]);
let _27: [i8; 8];
let _28: (*const u8, isize, *const &'static i8, *mut bool);
let _29: (char, i32, [i32; 1]);
let _30: Adt58;
let _31: Adt59;
let _32: Adt60;
let _33: f32;
let _34: (*const u8, isize, *const &'static i8, *mut bool);
let _35: isize;
let _36: f32;
let _37: u32;
let _38: ();
let _39: ();
{
RET.1 = [false];
RET.2 = (-130943401884504147258486370666710190015_i128);
RET.0 = 22_i8;
_9 = !_10;
_4 = _10 << _9;
_7 = _14;
_12 = _13.0 as usize;
_7 = _3;
_13.1 = [true];
_13.0 = 10738_u16 as f32;
RET.2 = (-46254726040033601950232427018462900071_i128) - (-108310905857392615467746684962250104971_i128);
_5 = RET.2 as usize;
_1 = [_8,_8,_7,_2,_4,_2,_9];
_7 = _4;
_9 = (-3375484309095951030_i64) as usize;
_6 = _11;
_11 = [_7,_8,_7,_7,_3,_2,_14];
_2 = !_14;
_10 = 81_isize as usize;
RET.0 = _3 as i8;
_15 = 10_isize << _4;
RET = (44_i8, _13.1, 95282189321113567969535171989927116592_i128);
Goto(bb1)
}
bb1 = {
_14 = (-1839569912342148604_i64) as usize;
_12 = !_7;
_13.2 = [_15,_15,_15,_15,_15,_15,_15];
RET.2 = (-22422005857038501308509620234499575807_i128);
_16 = [15323_i16,5311_i16,29808_i16,17305_i16];
RET.2 = 65390_u16 as i128;
_11 = [_3,_5,_7,_8,_8,_7,_5];
RET.0 = RET.2 as i8;
_13.2 = [_15,_15,_15,_15,_15,_15,_15];
_4 = !_8;
_1 = _6;
_6 = _11;
_13.2 = [_15,_15,_15,_15,_15,_15,_15];
Goto(bb2)
}
bb2 = {
_20 = (_13.0, true);
_18 = _13.0 as u8;
_23.fld1.0 = _20.0;
RET.2 = (-101582335492262095875063878227981695028_i128) + 10388283881483854301551019607944831798_i128;
_19 = RET.0 as u64;
_22 = -_15;
_20.0 = _22 as f32;
_8 = '\u{c3236}' as usize;
RET.2 = 19900349242286850593658944100429759430_i128;
_7 = !_2;
_23.fld1.1 = _20.1;
_8 = _12;
_13.0 = -_20.0;
_24 = !_20.1;
_23.fld1 = (_13.0, _24);
Call(RET.0 = fn9(_11, _13, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = _8 << _8;
_4 = _7;
_13.0 = RET.2 as f32;
_17 = -70760397_i32;
_5 = RET.2 as usize;
_26.1 = [_24];
_22 = _2 as isize;
_26.1 = [_24];
RET = (123_i8, _26.1, (-127574048224973534074500221257221386245_i128));
_27 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_13.1 = [_24];
_23.fld1.1 = _20.1;
_26 = (_20.0, RET.1, _13.2);
match RET.0 {
123 => bb5,
_ => bb4
}
}
bb4 = {
_20 = (_13.0, true);
_18 = _13.0 as u8;
_23.fld1.0 = _20.0;
RET.2 = (-101582335492262095875063878227981695028_i128) + 10388283881483854301551019607944831798_i128;
_19 = RET.0 as u64;
_22 = -_15;
_20.0 = _22 as f32;
_8 = '\u{c3236}' as usize;
RET.2 = 19900349242286850593658944100429759430_i128;
_7 = !_2;
_23.fld1.1 = _20.1;
_8 = _12;
_13.0 = -_20.0;
_24 = !_20.1;
_23.fld1 = (_13.0, _24);
Call(RET.0 = fn9(_11, _13, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_9 = _4 | _2;
_26.0 = -_20.0;
_8 = !_2;
_2 = _4;
_20.0 = -_26.0;
_3 = _9 * _7;
_26 = _13;
_4 = 3622086807_u32 as usize;
Call(RET = fn10(_20, _13.2, _1, _20, _23.fld1.0, _26, _23.fld1, _20.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_23.fld0 = [_17];
_28.1 = !_22;
_15 = _28.1;
_28.0 = core::ptr::addr_of!(_18);
_28.0 = core::ptr::addr_of!(_18);
_4 = RET.0 as usize;
Call(RET.2 = core::intrinsics::bswap((-11332010856104971709983733679381869752_i128)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_18 = 64_u8 - 73_u8;
_17 = (-1100194111_i32) + 770999088_i32;
_29.2 = [_17];
_15 = _18 as isize;
_26 = _13;
_8 = _22 as usize;
_29.2 = [_17];
_26.0 = _20.0;
RET.1 = [_20.1];
Call(_28.2 = fn11(_29.2, _20.0, _26.0, _2, _22, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_23.fld0 = [_17];
_28.3 = core::ptr::addr_of_mut!(_24);
_29.1 = _17 | _17;
_20.0 = _23.fld1.0 + _23.fld1.0;
RET.2 = 46787978393430731245813189048597367086_i128 >> _3;
RET.2 = _9 as i128;
_16 = [26433_i16,(-14823_i16),8747_i16,(-7536_i16)];
_35 = RET.2 as isize;
_3 = _9 * _2;
_34.3 = _28.3;
_20 = (_26.0, _24);
_36 = 4208718189454762513_i64 as f32;
_33 = _23.fld1.0 + _23.fld1.0;
_2 = _3 << _28.1;
_34.3 = _28.3;
_18 = !219_u8;
Goto(bb9)
}
bb9 = {
Call(_38 = dump_var(8_usize, 10_usize, Move(_10), 11_usize, Move(_11), 35_usize, Move(_35), 2_usize, Move(_2)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_38 = dump_var(8_usize, 22_usize, Move(_22), 17_usize, Move(_17), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_38 = dump_var(8_usize, 1_usize, Move(_1), 4_usize, Move(_4), 24_usize, Move(_24), 39_usize, _39), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [usize; 7],mut _2: (f32, [bool; 1], [isize; 7]),mut _3: (f32, [bool; 1], [isize; 7])) -> i8 {
mir! {
type RET = i8;
let _4: [usize; 7];
let _5: i64;
let _6: i32;
let _7: Adt48;
let _8: isize;
let _9: u64;
let _10: isize;
let _11: isize;
let _12: (f32, bool);
let _13: usize;
let _14: bool;
let _15: u128;
let _16: bool;
let _17: u64;
let _18: u16;
let _19: (char, i32, [i32; 1]);
let _20: f64;
let _21: [i8; 6];
let _22: ();
let _23: ();
{
_4 = [10440105565850876562_usize,3763191625384714502_usize,2_usize,0_usize,994036475536656720_usize,11953152304684296145_usize,10739662968589240587_usize];
_1 = [2816685305255498205_usize,5_usize,3497182214830890941_usize,8071029374072724358_usize,7_usize,10503327696344745470_usize,14146082305137378944_usize];
RET = !20_i8;
_2.2 = [9223372036854775807_isize,(-20_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3.2 = [(-9223372036854775808_isize),53_isize,(-9223372036854775808_isize),103_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = !(-17_i8);
_2.1 = _3.1;
_3.2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-75_isize),(-9223372036854775808_isize),74_isize,61_isize,125_isize];
_3.2 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_3.1 = _2.1;
_5 = _2.0 as i64;
RET = true as i8;
_2.0 = -_3.0;
_2 = _3;
_1 = [16964297451026448351_usize,5_usize,7347307436359117624_usize,7406459939436019157_usize,13619488233982002044_usize,6_usize,5798038833894459284_usize];
_6 = 284723270925266692040531705432317374671_u128 as i32;
_2.2 = _3.2;
_2.2 = _3.2;
_3.0 = -_2.0;
RET = (-83_i8);
_8 = -(-113_isize);
_4 = [10359418889927446606_usize,8500773250229676823_usize,3_usize,3_usize,15352229554382265907_usize,1903677701921498121_usize,0_usize];
_2.2 = [_8,_8,_8,_8,_8,_8,_8];
_3.0 = _2.0;
_1 = [15662517377894849640_usize,2_usize,17257367141137624683_usize,6_usize,3_usize,4_usize,4_usize];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211373 => bb8,
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
_8 = (-93_isize);
_4 = [5726826937518961919_usize,11022570029475301099_usize,2301583820699372162_usize,4_usize,12677626757320844901_usize,4_usize,17555950434602401672_usize];
_3.0 = _2.0;
_2 = (_3.0, _3.1, _3.2);
_2.0 = _3.0 - _3.0;
_3.1 = [true];
_2.1 = [false];
_9 = 10264853297500609713_u64;
_2 = _3;
_6 = -(-694071197_i32);
_1 = [6_usize,0_usize,3311183349592661262_usize,4_usize,5239667531071908596_usize,12982502985270726041_usize,1680953524584727334_usize];
_8 = 45061_u16 as isize;
_5 = _3.0 as i64;
_10 = _8 + _8;
_4 = [10919255760236628067_usize,0_usize,6603290031544738203_usize,6_usize,1923040723506803669_usize,3691314412304950415_usize,1_usize];
_10 = _8;
_6 = (-276479725_i32);
_5 = (-556334648822655638_i64) | (-5176145096706277382_i64);
_3.2 = [_10,_10,_10,_10,_8,_10,_8];
RET = 16_i8 << _9;
_3.1 = [true];
_1 = _4;
RET = !(-123_i8);
_3.1 = _2.1;
_7 = Adt48::Variant2 { fld0: 228945650214086347628689917040043550814_u128,fld1: RET };
_1 = _4;
Goto(bb9)
}
bb9 = {
_4 = _1;
_10 = (-26305_i16) as isize;
place!(Field::<i8>(Variant(_7, 2), 1)) = _10 as i8;
Goto(bb10)
}
bb10 = {
RET = Field::<i8>(Variant(_7, 2), 1) * Field::<i8>(Variant(_7, 2), 1);
_2.2 = [_8,_8,_10,_10,_8,_8,_8];
RET = _5 as i8;
_3.1 = _2.1;
_3.1 = [true];
place!(Field::<i8>(Variant(_7, 2), 1)) = RET;
_3 = (_2.0, _2.1, _2.2);
place!(Field::<u128>(Variant(_7, 2), 0)) = 312037290986526884469800185255525518664_u128;
_2 = (_3.0, _3.1, _3.2);
_2.1 = [true];
_11 = 101_u8 as isize;
_13 = _8 as usize;
_1 = _4;
_2 = _3;
_10 = !_8;
_7 = Adt48::Variant2 { fld0: 15889576585527892685513083926940225839_u128,fld1: RET };
place!(Field::<u128>(Variant(_7, 2), 0)) = 311388261601903871777446598455037618603_u128 << RET;
_3.2 = _2.2;
_12.0 = Field::<u128>(Variant(_7, 2), 0) as f32;
RET = Field::<i8>(Variant(_7, 2), 1) * Field::<i8>(Variant(_7, 2), 1);
Goto(bb11)
}
bb11 = {
_14 = false;
_12 = (_3.0, _14);
_2 = (_12.0, _3.1, _3.2);
_3 = _2;
_13 = 7_usize | 1_usize;
_5 = !(-9029843612137609118_i64);
_2 = (_3.0, _3.1, _3.2);
SetDiscriminant(_7, 1);
place!(Field::<[bool; 1]>(Variant(_7, 1), 4)) = [_12.1];
RET = 116_i8 >> _8;
match _9 {
0 => bb2,
10264853297500609713 => bb13,
_ => bb12
}
}
bb12 = {
RET = Field::<i8>(Variant(_7, 2), 1) * Field::<i8>(Variant(_7, 2), 1);
_2.2 = [_8,_8,_10,_10,_8,_8,_8];
RET = _5 as i8;
_3.1 = _2.1;
_3.1 = [true];
place!(Field::<i8>(Variant(_7, 2), 1)) = RET;
_3 = (_2.0, _2.1, _2.2);
place!(Field::<u128>(Variant(_7, 2), 0)) = 312037290986526884469800185255525518664_u128;
_2 = (_3.0, _3.1, _3.2);
_2.1 = [true];
_11 = 101_u8 as isize;
_13 = _8 as usize;
_1 = _4;
_2 = _3;
_10 = !_8;
_7 = Adt48::Variant2 { fld0: 15889576585527892685513083926940225839_u128,fld1: RET };
place!(Field::<u128>(Variant(_7, 2), 0)) = 311388261601903871777446598455037618603_u128 << RET;
_3.2 = _2.2;
_12.0 = Field::<u128>(Variant(_7, 2), 0) as f32;
RET = Field::<i8>(Variant(_7, 2), 1) * Field::<i8>(Variant(_7, 2), 1);
Goto(bb11)
}
bb13 = {
_9 = !1825585339055340759_u64;
_2.0 = _12.0;
_8 = _11;
_1 = _4;
place!(Field::<[u128; 8]>(Variant(_7, 1), 1)) = [126288543795999368082375413858821044673_u128,306180710105776215719306518888117415327_u128,30890917469644652033225385677918599395_u128,272427426416991575252247790494304746932_u128,162982867900901086209128655438128835320_u128,324769112811324250848164870804966973053_u128,194920369068016331928310315536260310163_u128,340129005881602949208176625175253949620_u128];
_12.1 = _14;
_2.0 = 742222260_u32 as f32;
_3 = (_12.0, _2.1, _2.2);
_12.1 = _14;
place!(Field::<[i8; 6]>(Variant(_7, 1), 2)) = [RET,RET,RET,RET,RET,RET];
_2.0 = _12.0;
place!(Field::<[bool; 1]>(Variant(_7, 1), 4)) = _2.1;
place!(Field::<[u128; 8]>(Variant(_7, 1), 1)) = [317773632680336891751533743913596184149_u128,32980356399073313168899297785194787790_u128,265306166903209880993521644864178381409_u128,243731946863408732660766492411361796507_u128,178662675799297362159450514275311531393_u128,177482604383308999604936903116577228086_u128,181364663728154382498733113721338177104_u128,229031215854191714487084736391938862222_u128];
_3 = (_2.0, Field::<[bool; 1]>(Variant(_7, 1), 4), _2.2);
_3 = _2;
_4 = _1;
_3 = (_12.0, Field::<[bool; 1]>(Variant(_7, 1), 4), _2.2);
_15 = !27513657208763954635089624150087796531_u128;
_9 = 5653234081871830768_u64;
_10 = _11 & _8;
_3.2 = [_10,_11,_10,_10,_11,_10,_10];
_1 = [_13,_13,_13,_13,_13,_13,_13];
_11 = _10 * _8;
place!(Field::<[usize; 7]>(Variant(_7, 1), 0)) = _4;
match _9 {
5653234081871830768 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_17 = _9;
_12 = (_3.0, _14);
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(9_usize, 10_usize, Move(_10), 9_usize, Move(_9), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(9_usize, 17_usize, Move(_17), 5_usize, Move(_5), 23_usize, _23, 23_usize, _23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (f32, bool),mut _2: [isize; 7],mut _3: [usize; 7],mut _4: (f32, bool),mut _5: f32,mut _6: (f32, [bool; 1], [isize; 7]),mut _7: (f32, bool),mut _8: f32) -> (i8, [bool; 1], i128) {
mir! {
type RET = (i8, [bool; 1], i128);
let _9: [i8; 6];
let _10: f64;
let _11: char;
let _12: usize;
let _13: *const *mut bool;
let _14: char;
let _15: [i8; 6];
let _16: f64;
let _17: isize;
let _18: [isize; 7];
let _19: usize;
let _20: Adt59;
let _21: (f32, bool);
let _22: &'static i8;
let _23: bool;
let _24: (u64,);
let _25: Adt51;
let _26: &'static i8;
let _27: [u128; 8];
let _28: char;
let _29: u16;
let _30: [i8; 8];
let _31: isize;
let _32: Adt61;
let _33: u64;
let _34: ([i32; 1],);
let _35: ();
let _36: ();
{
_4.0 = _8 + _6.0;
RET.0 = 70120459894562892157135081215528333085_u128 as i8;
_6.1 = [_1.1];
_2 = [9223372036854775807_isize,9223372036854775807_isize,4_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-84_isize)];
RET.1 = [_1.1];
_4.0 = -_7.0;
RET.1 = _6.1;
RET.0 = (-26_i8) * (-46_i8);
_1.1 = _7.1;
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),109_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = (_1.0, _7.1);
_6.1 = [_1.1];
_4 = (_8, _1.1);
_7.1 = _4.1;
_10 = 40095_u16 as f64;
_12 = 8898722075703908103_usize ^ 3_usize;
_6.1 = [_4.1];
_7 = (_5, _4.1);
RET = ((-93_i8), _6.1, 81667854661761734529178701034295533722_i128);
_11 = '\u{8a7a2}';
_14 = _11;
_9 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
match RET.2 {
0 => bb1,
81667854661761734529178701034295533722 => bb3,
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
RET.0 = 21_i8;
RET.2 = (-56630987339362095338082478794182796503_i128);
_6 = (_1.0, RET.1, _2);
RET.2 = RET.0 as i128;
_3 = [_12,_12,_12,_12,_12,_12,_12];
_15 = _9;
_6.1 = RET.1;
_16 = _10 - _10;
_3 = [_12,_12,_12,_12,_12,_12,_12];
match RET.0 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
21 => bb8,
_ => bb7
}
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
_1 = _4;
_4.1 = !_7.1;
_17 = 9223372036854775807_isize - 61_isize;
RET.2 = !(-42116406264946382267852634013794386529_i128);
_8 = _6.0;
_9 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_16 = -_10;
_5 = RET.0 as f32;
RET.0 = !(-11_i8);
_1 = _4;
_17 = 9223372036854775807_isize;
_19 = !_12;
Goto(bb9)
}
bb9 = {
_8 = -_7.0;
RET.2 = !(-46742131992072016646531526930568849467_i128);
_1.1 = !_4.1;
_4.0 = (-1424527336_i32) as f32;
_16 = -_10;
RET = (41_i8, _6.1, (-145315644486166893322734438143660264418_i128));
_10 = -_16;
_3 = [_19,_19,_12,_12,_12,_19,_12];
_11 = _14;
RET = ((-112_i8), _6.1, 72474090890575566761820958752266685392_i128);
_11 = _14;
_7 = (_6.0, _1.1);
_1 = (_8, _7.1);
_21 = (_6.0, _4.1);
_15 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_18 = [_17,_17,_17,_17,_17,_17,_17];
_1 = (_8, _4.1);
_4.0 = -_21.0;
RET.0 = (-34_i8);
RET.2 = RET.0 as i128;
RET.0 = !(-33_i8);
_16 = 65_u8 as f64;
_19 = _12;
Goto(bb10)
}
bb10 = {
_6.0 = _21.0;
_18 = _2;
RET.0 = (-112_i8);
_17 = 109_isize;
_1.1 = !_21.1;
RET = ((-83_i8), _6.1, (-139826610483884226267120334584949252293_i128));
_7.1 = _21.1 ^ _21.1;
_5 = 19878_u16 as f32;
_9 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_1.1 = !_4.1;
RET = ((-62_i8), _6.1, (-151571891937889216281688312990080729055_i128));
RET.1 = [_7.1];
RET = (30_i8, _6.1, (-92011095085263179908350990832786392767_i128));
RET.2 = !(-75612388583713211589534090011669056231_i128);
_14 = _11;
_1.0 = 232056632646612206814354522137176122768_u128 as f32;
_6.1 = RET.1;
_6.0 = _4.0;
_4.0 = _8;
_12 = 154_u8 as usize;
_4.1 = !_7.1;
_7 = _4;
_1.1 = _7.1;
_12 = !_19;
_8 = _7.0 + _7.0;
RET.2 = 14991_i16 as i128;
_21 = (_8, _7.1);
_9 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_1 = _4;
Goto(bb11)
}
bb11 = {
_6.1 = RET.1;
RET = (78_i8, _6.1, 67360891806430025865336676797490782661_i128);
_17 = 105_isize ^ (-9223372036854775808_isize);
_4 = (_6.0, _1.1);
RET.0 = -(-18_i8);
_2 = [_17,_17,_17,_17,_17,_17,_17];
_7.0 = _21.0;
_24.0 = 9339608030663170455_u64;
_22 = &RET.0;
_5 = _1.0;
_1.1 = _6.0 == _7.0;
_6.0 = RET.2 as f32;
_14 = _11;
_1.1 = _7.1 | _7.1;
_23 = _21.1;
_7.1 = _1.1;
_5 = 3836706934_u32 as f32;
_1 = (_8, _7.1);
_6 = (_21.0, RET.1, _18);
_2 = _18;
_25 = Adt51::Variant0 { fld0: (*_22) };
_1.1 = !_7.1;
_15 = [Field::<i8>(Variant(_25, 0), 0),RET.0,Field::<i8>(Variant(_25, 0), 0),Field::<i8>(Variant(_25, 0), 0),RET.0,(*_22)];
_7.0 = -_21.0;
_26 = Move(_22);
Goto(bb12)
}
bb12 = {
RET.2 = 168575356144761241646897620084230548862_i128 * (-89037945317959592759332848469641556954_i128);
_1 = (_8, _23);
_11 = _14;
_21.1 = !_7.1;
RET.2 = (-107666867667020751560946210379732051228_i128) & (-5833952040444693323395894287962578300_i128);
SetDiscriminant(_25, 1);
RET.2 = (-56838894782954906649923046035838842408_i128);
_12 = !_19;
_19 = !_12;
place!(Field::<(f32, bool)>(Variant(_25, 1), 4)).0 = _8;
_12 = !_19;
place!(Field::<[i128; 6]>(Variant(_25, 1), 2)) = [RET.2,RET.2,RET.2,RET.2,RET.2,RET.2];
_6 = (_4.0, RET.1, _2);
_22 = Move(_26);
Goto(bb13)
}
bb13 = {
_17 = -9223372036854775807_isize;
_1 = (_4.0, _23);
RET.2 = (-1010208185_i32) as i128;
place!(Field::<i128>(Variant(_25, 1), 5)) = !RET.2;
_24 = (12874845451273022051_u64,);
_4 = (Field::<(f32, bool)>(Variant(_25, 1), 4).0, _7.1);
_26 = Move(_22);
_1.1 = _4.1;
place!(Field::<(f32, bool)>(Variant(_25, 1), 4)).0 = _4.0 - _6.0;
Goto(bb14)
}
bb14 = {
place!(Field::<(char, i32, [i32; 1])>(Variant(_25, 1), 3)).2 = [(-458857364_i32)];
_29 = 2848070987_u32 as u16;
_12 = _19;
place!(Field::<(char, i32, [i32; 1])>(Variant(_25, 1), 3)).2 = [(-1456343128_i32)];
place!(Field::<(f32, bool)>(Variant(_25, 1), 4)).1 = !_21.1;
_27 = [45766068710436977619874058171716659095_u128,278432127605198856915956685262696779349_u128,290203714273928302146138178753130652999_u128,7353080708894609065382110909329904874_u128,274821811320406015470034524876497500075_u128,141528359784836866143789529513103519314_u128,291645877773904838355861458325044897849_u128,170732866802613158082088108371080748169_u128];
_24 = (7778992072119827020_u64,);
place!(Field::<(char, i32, [i32; 1])>(Variant(_25, 1), 3)).0 = _11;
_25 = Adt51::Variant0 { fld0: (*_26) };
_4.1 = _21.0 > _8;
_5 = _6.0;
_22 = Move(_26);
_28 = _11;
_30 = [RET.0,RET.0,(*_22),(*_22),(*_22),Field::<i8>(Variant(_25, 0), 0),RET.0,(*_22)];
_7.1 = !_4.1;
RET.0 = (-1659882666398559874_i64) as i8;
_10 = _16;
_24.0 = 10789872961140541250_u64;
_21.0 = _4.0 - _8;
_3 = [_19,_19,_19,_12,_19,_19,_19];
_12 = RET.0 as usize;
_25 = Adt51::Variant0 { fld0: RET.0 };
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(10_usize, 23_usize, Move(_23), 19_usize, Move(_19), 11_usize, Move(_11), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(10_usize, 17_usize, Move(_17), 30_usize, Move(_30), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [i32; 1],mut _2: f32,mut _3: f32,mut _4: usize,mut _5: isize,mut _6: usize) -> *const &'static i8 {
mir! {
type RET = *const &'static i8;
let _7: (i8, [bool; 1], i128);
let _8: [i16; 4];
let _9: f32;
let _10: i8;
let _11: f32;
let _12: [isize; 7];
let _13: *mut i8;
let _14: bool;
let _15: Adt50;
let _16: [i128; 6];
let _17: u32;
let _18: [i8; 6];
let _19: f64;
let _20: char;
let _21: [usize; 7];
let _22: *mut *mut f64;
let _23: Adt48;
let _24: char;
let _25: *mut f64;
let _26: ([i8; 6],);
let _27: char;
let _28: [i32; 1];
let _29: [usize; 5];
let _30: f32;
let _31: f32;
let _32: [usize; 7];
let _33: f64;
let _34: Adt53;
let _35: [i16; 4];
let _36: [i64; 5];
let _37: isize;
let _38: (i8, [bool; 1], i128);
let _39: [usize; 5];
let _40: char;
let _41: [i32; 1];
let _42: [isize; 7];
let _43: i16;
let _44: isize;
let _45: (u64,);
let _46: (f32, bool);
let _47: isize;
let _48: i16;
let _49: char;
let _50: usize;
let _51: ([i32; 1],);
let _52: Adt48;
let _53: usize;
let _54: [usize; 7];
let _55: i32;
let _56: i32;
let _57: isize;
let _58: i64;
let _59: [isize; 7];
let _60: [i8; 6];
let _61: isize;
let _62: f32;
let _63: isize;
let _64: *mut bool;
let _65: [isize; 7];
let _66: (f32, bool);
let _67: u64;
let _68: [i32; 1];
let _69: [i32; 1];
let _70: [usize; 5];
let _71: [i8; 6];
let _72: [usize; 5];
let _73: [usize; 7];
let _74: ([i32; 1],);
let _75: [i16; 4];
let _76: (*const u8, isize, *const &'static i8, *mut bool);
let _77: u32;
let _78: char;
let _79: f32;
let _80: Adt57;
let _81: [usize; 7];
let _82: Adt59;
let _83: i64;
let _84: [i64; 5];
let _85: [bool; 1];
let _86: u16;
let _87: bool;
let _88: i32;
let _89: [usize; 5];
let _90: (char, i32, [i32; 1]);
let _91: u64;
let _92: bool;
let _93: Adt51;
let _94: *const *mut bool;
let _95: i64;
let _96: Adt54;
let _97: bool;
let _98: isize;
let _99: f32;
let _100: bool;
let _101: [bool; 1];
let _102: f32;
let _103: Adt63;
let _104: u128;
let _105: &'static i8;
let _106: f32;
let _107: ();
let _108: ();
{
_3 = _2 + _2;
_7.2 = -(-37386211619305030224052245881841138525_i128);
_7.1 = [false];
_7.1 = [false];
_1 = [(-1841478645_i32)];
_7.2 = (-22724_i16) as i128;
_7.0 = _6 as i8;
_7.2 = 6962633534887064834_u64 as i128;
_4 = _6 << _7.0;
Call(_7.1 = fn12(_5, _5, _5, _2, _5, _5, _5, _5, _2, _5, _3, _5, _3, _2, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 9223372036854775807_isize + (-127_isize);
_7.2 = 54110869121924673751945785240117391724_i128;
_7.0 = 25_i8;
_7.2 = 946184424_u32 as i128;
_6 = _4 + _4;
_4 = !_6;
_1 = [(-822070271_i32)];
_1 = [1384749520_i32];
_7.0 = (-44_i8);
_3 = _2 - _2;
_9 = 113_u8 as f32;
_6 = _4;
_7.1 = [true];
_1 = [(-1361711918_i32)];
_7.0 = -(-26_i8);
Goto(bb2)
}
bb2 = {
_7.2 = _5 as i128;
_7.0 = -47_i8;
_8 = [30734_i16,(-8381_i16),(-14891_i16),(-2725_i16)];
_10 = _7.0;
_4 = 40487_u16 as usize;
_11 = -_2;
_4 = _6;
_12 = [_5,_5,_5,_5,_5,_5,_5];
_7.1 = [false];
_7.1 = [true];
_9 = _2;
_6 = !_4;
_1 = [1798234477_i32];
_7.0 = _10 | _10;
Goto(bb3)
}
bb3 = {
_7.2 = !71544767711941548564786787748444067693_i128;
_12 = [_5,_5,_5,_5,_5,_5,_5];
_9 = 7874137207346427094_u64 as f32;
_10 = 3987289960_u32 as i8;
_7.1 = [true];
_10 = !_7.0;
_4 = !_6;
_7.2 = -(-63892538919032847985324700018921118949_i128);
_11 = -_2;
_4 = _6;
_3 = _2;
_5 = 9223372036854775807_isize;
_11 = -_3;
_5 = true as isize;
_4 = _7.0 as usize;
_5 = !9223372036854775807_isize;
_11 = -_3;
_4 = _6;
_1 = [1150873374_i32];
_7.2 = (-19099_i16) as i128;
_7.2 = 106462927867176187488087895085354395978_i128;
_7.1 = [true];
_2 = 7926874071269564140_u64 as f32;
_7.0 = -_10;
_3 = 53_u8 as f32;
_7.1 = [false];
Goto(bb4)
}
bb4 = {
_13 = core::ptr::addr_of_mut!(_10);
_10 = '\u{9d0b1}' as i8;
(*_13) = _7.0 ^ _7.0;
_14 = _11 > _11;
_1 = [(-1160818299_i32)];
_6 = _4 << _4;
_6 = _4 * _4;
_11 = _5 as f32;
_4 = _7.0 as usize;
_5 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_11 = _3;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
Goto(bb5)
}
bb5 = {
_12 = [_5,_5,_5,_5,_5,_5,_5];
_4 = _6 << _6;
_4 = 97167636025724615914590671448048558913_u128 as usize;
_17 = 1719765757_u32;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_11 = _2;
_3 = -_9;
Goto(bb6)
}
bb6 = {
_8 = [21594_i16,(-25156_i16),(-19713_i16),(-25794_i16)];
_6 = _4 + _4;
_9 = _2;
_8 = [(-3745_i16),(-10798_i16),6120_i16,9042_i16];
_17 = !237858253_u32;
_14 = !false;
_19 = _2 as f64;
_2 = _3;
(*_13) = _7.0;
(*_13) = -_7.0;
(*_13) = _7.0;
_18 = [_7.0,(*_13),(*_13),(*_13),(*_13),_7.0];
_10 = _7.0;
_9 = _2 - _2;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_19 = 17294223334214692622_u64 as f64;
_1 = [1707250402_i32];
match _7.2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb7,
4 => bb8,
5 => bb9,
106462927867176187488087895085354395978 => bb11,
_ => bb10
}
}
bb7 = {
_12 = [_5,_5,_5,_5,_5,_5,_5];
_4 = _6 << _6;
_4 = 97167636025724615914590671448048558913_u128 as usize;
_17 = 1719765757_u32;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_11 = _2;
_3 = -_9;
Goto(bb6)
}
bb8 = {
_13 = core::ptr::addr_of_mut!(_10);
_10 = '\u{9d0b1}' as i8;
(*_13) = _7.0 ^ _7.0;
_14 = _11 > _11;
_1 = [(-1160818299_i32)];
_6 = _4 << _4;
_6 = _4 * _4;
_11 = _5 as f32;
_4 = _7.0 as usize;
_5 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_11 = _3;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
Goto(bb5)
}
bb9 = {
_7.2 = !71544767711941548564786787748444067693_i128;
_12 = [_5,_5,_5,_5,_5,_5,_5];
_9 = 7874137207346427094_u64 as f32;
_10 = 3987289960_u32 as i8;
_7.1 = [true];
_10 = !_7.0;
_4 = !_6;
_7.2 = -(-63892538919032847985324700018921118949_i128);
_11 = -_2;
_4 = _6;
_3 = _2;
_5 = 9223372036854775807_isize;
_11 = -_3;
_5 = true as isize;
_4 = _7.0 as usize;
_5 = !9223372036854775807_isize;
_11 = -_3;
_4 = _6;
_1 = [1150873374_i32];
_7.2 = (-19099_i16) as i128;
_7.2 = 106462927867176187488087895085354395978_i128;
_7.1 = [true];
_2 = 7926874071269564140_u64 as f32;
_7.0 = -_10;
_3 = 53_u8 as f32;
_7.1 = [false];
Goto(bb4)
}
bb10 = {
_5 = 9223372036854775807_isize + (-127_isize);
_7.2 = 54110869121924673751945785240117391724_i128;
_7.0 = 25_i8;
_7.2 = 946184424_u32 as i128;
_6 = _4 + _4;
_4 = !_6;
_1 = [(-822070271_i32)];
_1 = [1384749520_i32];
_7.0 = (-44_i8);
_3 = _2 - _2;
_9 = 113_u8 as f32;
_6 = _4;
_7.1 = [true];
_1 = [(-1361711918_i32)];
_7.0 = -(-26_i8);
Goto(bb2)
}
bb11 = {
_9 = _3;
_19 = _11 as f64;
_5 = 34664272431059365918715111303945341084_u128 as isize;
_7.1 = [_14];
_12 = [_5,_5,_5,_5,_5,_5,_5];
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_7.2 = _19 as i128;
_20 = '\u{9e949}';
_9 = _11;
_3 = (-6671513983634153228_i64) as f32;
_13 = core::ptr::addr_of_mut!((*_13));
_13 = core::ptr::addr_of_mut!((*_13));
_17 = 4175419861_u32 ^ 1269002963_u32;
Goto(bb12)
}
bb12 = {
_17 = !2476766830_u32;
_10 = _14 as i8;
_20 = '\u{7c40f}';
_14 = true ^ true;
_5 = 93_isize;
_14 = !true;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
match _5 {
0 => bb1,
93 => bb13,
_ => bb4
}
}
bb13 = {
_24 = _20;
_7.0 = (*_13);
_13 = core::ptr::addr_of_mut!((*_13));
_3 = _2 + _11;
_14 = false;
_13 = core::ptr::addr_of_mut!((*_13));
_17 = 2001507236_u32 * 1083622587_u32;
_17 = !1235828362_u32;
_11 = 159015346949244740200831180861061165033_u128 as f32;
_23 = Adt48::Variant2 { fld0: 164882351979944988758422415453081004524_u128,fld1: (*_13) };
_21 = [_6,_6,_6,_4,_4,_6,_6];
_19 = _3 as f64;
_14 = false & false;
_22 = core::ptr::addr_of_mut!(_25);
(*_22) = core::ptr::addr_of_mut!(_19);
(*_22) = core::ptr::addr_of_mut!((*_25));
_7.1 = [_14];
_19 = (-6522_i16) as f64;
_1 = [208801762_i32];
_2 = _3;
_2 = _9 - _9;
_24 = _20;
(*_22) = core::ptr::addr_of_mut!((*_25));
_24 = _20;
_6 = _17 as usize;
_22 = core::ptr::addr_of_mut!(_25);
(*_13) = -Field::<i8>(Variant(_23, 2), 1);
_21 = [_6,_6,_4,_4,_6,_6,_6];
_12 = [_5,_5,_5,_5,_5,_5,_5];
_20 = _24;
_14 = true;
Goto(bb14)
}
bb14 = {
_17 = 1690025854_u32 & 1461214099_u32;
place!(Field::<i8>(Variant(_23, 2), 1)) = _10;
_14 = _20 == _24;
_19 = 144_u8 as f64;
_17 = (-10915_i16) as u32;
_8 = [7212_i16,6129_i16,(-25824_i16),10389_i16];
_2 = _3 * _11;
Goto(bb15)
}
bb15 = {
_4 = _6 | _6;
_8 = [(-29043_i16),31185_i16,(-29047_i16),20643_i16];
_7.2 = (-41960248552322635900711986865295384849_i128) >> _17;
(*_13) = _7.0;
(*_13) = Field::<i8>(Variant(_23, 2), 1) >> _4;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = 14443_u16 as f64;
(*_22) = core::ptr::addr_of_mut!(_19);
_7.0 = !_10;
_19 = 39066606934215755351381605836072405516_u128 as f64;
_26.0 = _18;
(*_22) = core::ptr::addr_of_mut!((*_25));
(*_25) = 8426681873417468078_u64 as f64;
_17 = !2031518427_u32;
(*_13) = _7.2 as i8;
_29 = [_4,_4,_6,_4,_6];
_13 = core::ptr::addr_of_mut!(_7.0);
place!(Field::<u128>(Variant(_23, 2), 0)) = 321045392327256687573112918344628840579_u128;
Goto(bb16)
}
bb16 = {
_8 = [14002_i16,(-28165_i16),(-19462_i16),9487_i16];
_9 = -_3;
_27 = _24;
_25 = core::ptr::addr_of_mut!((*_25));
place!(Field::<i8>(Variant(_23, 2), 1)) = (*_13);
_25 = core::ptr::addr_of_mut!(_19);
(*_22) = core::ptr::addr_of_mut!(_19);
_17 = 1272743435_u32;
(*_13) = 624587185_i32 as i8;
_31 = _3;
place!(Field::<u128>(Variant(_23, 2), 0)) = 155529949582341555704654460573349662964_u128;
(*_13) = Field::<i8>(Variant(_23, 2), 1);
_28 = [(-1607342604_i32)];
_32 = [_4,_4,_4,_4,_6,_6,_4];
_31 = -_9;
SetDiscriminant(_23, 2);
_25 = core::ptr::addr_of_mut!((*_25));
Goto(bb17)
}
bb17 = {
_3 = _11 * _9;
_11 = -_2;
(*_13) = _10;
match _5 {
0 => bb9,
1 => bb2,
2 => bb11,
3 => bb15,
4 => bb5,
5 => bb6,
93 => bb19,
_ => bb18
}
}
bb18 = {
_8 = [14002_i16,(-28165_i16),(-19462_i16),9487_i16];
_9 = -_3;
_27 = _24;
_25 = core::ptr::addr_of_mut!((*_25));
place!(Field::<i8>(Variant(_23, 2), 1)) = (*_13);
_25 = core::ptr::addr_of_mut!(_19);
(*_22) = core::ptr::addr_of_mut!(_19);
_17 = 1272743435_u32;
(*_13) = 624587185_i32 as i8;
_31 = _3;
place!(Field::<u128>(Variant(_23, 2), 0)) = 155529949582341555704654460573349662964_u128;
(*_13) = Field::<i8>(Variant(_23, 2), 1);
_28 = [(-1607342604_i32)];
_32 = [_4,_4,_4,_4,_6,_6,_4];
_31 = -_9;
SetDiscriminant(_23, 2);
_25 = core::ptr::addr_of_mut!((*_25));
Goto(bb17)
}
bb19 = {
(*_22) = core::ptr::addr_of_mut!((*_25));
_30 = _9;
(*_22) = core::ptr::addr_of_mut!((*_25));
_19 = 124_u8 as f64;
_18 = [(*_13),(*_13),_10,_7.0,_7.0,_7.0];
match _5 {
0 => bb13,
1 => bb2,
2 => bb17,
3 => bb9,
4 => bb5,
5 => bb14,
6 => bb15,
93 => bb21,
_ => bb20
}
}
bb20 = {
_3 = _11 * _9;
_11 = -_2;
(*_13) = _10;
match _5 {
0 => bb9,
1 => bb2,
2 => bb11,
3 => bb15,
4 => bb5,
5 => bb6,
93 => bb19,
_ => bb18
}
}
bb21 = {
_28 = [(-803083688_i32)];
_20 = _27;
(*_25) = _6 as f64;
_10 = (*_13);
_34 = Adt53::Variant2 { fld0: 1537024325_i32,fld1: (*_22),fld2: 64743738127035710491559164870153510891_u128 };
_7.0 = _14 as i8;
(*_25) = _17 as f64;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_4 = _6;
_30 = _2;
match _17 {
1272743435 => bb23,
_ => bb22
}
}
bb22 = {
_5 = 9223372036854775807_isize + (-127_isize);
_7.2 = 54110869121924673751945785240117391724_i128;
_7.0 = 25_i8;
_7.2 = 946184424_u32 as i128;
_6 = _4 + _4;
_4 = !_6;
_1 = [(-822070271_i32)];
_1 = [1384749520_i32];
_7.0 = (-44_i8);
_3 = _2 - _2;
_9 = 113_u8 as f32;
_6 = _4;
_7.1 = [true];
_1 = [(-1361711918_i32)];
_7.0 = -(-26_i8);
Goto(bb2)
}
bb23 = {
_28 = [(-1226823825_i32)];
_37 = !_5;
(*_13) = _10 >> _10;
_11 = -_3;
_38.2 = !_7.2;
_38 = (_7.0, _7.1, _7.2);
_24 = _27;
_24 = _27;
_3 = (*_13) as f32;
_35 = [(-11989_i16),13886_i16,16250_i16,(-458_i16)];
_1 = _28;
_18 = [_38.0,_7.0,(*_13),_10,_10,(*_13)];
_32 = [_4,_6,_6,_4,_6,_6,_6];
_34 = Adt53::Variant2 { fld0: (-858894851_i32),fld1: _25,fld2: 180665074891008834581779841272163228662_u128 };
_24 = _20;
match _17 {
0 => bb24,
1 => bb25,
2 => bb26,
1272743435 => bb28,
_ => bb27
}
}
bb24 = {
_7.2 = !71544767711941548564786787748444067693_i128;
_12 = [_5,_5,_5,_5,_5,_5,_5];
_9 = 7874137207346427094_u64 as f32;
_10 = 3987289960_u32 as i8;
_7.1 = [true];
_10 = !_7.0;
_4 = !_6;
_7.2 = -(-63892538919032847985324700018921118949_i128);
_11 = -_2;
_4 = _6;
_3 = _2;
_5 = 9223372036854775807_isize;
_11 = -_3;
_5 = true as isize;
_4 = _7.0 as usize;
_5 = !9223372036854775807_isize;
_11 = -_3;
_4 = _6;
_1 = [1150873374_i32];
_7.2 = (-19099_i16) as i128;
_7.2 = 106462927867176187488087895085354395978_i128;
_7.1 = [true];
_2 = 7926874071269564140_u64 as f32;
_7.0 = -_10;
_3 = 53_u8 as f32;
_7.1 = [false];
Goto(bb4)
}
bb25 = {
_7.2 = _5 as i128;
_7.0 = -47_i8;
_8 = [30734_i16,(-8381_i16),(-14891_i16),(-2725_i16)];
_10 = _7.0;
_4 = 40487_u16 as usize;
_11 = -_2;
_4 = _6;
_12 = [_5,_5,_5,_5,_5,_5,_5];
_7.1 = [false];
_7.1 = [true];
_9 = _2;
_6 = !_4;
_1 = [1798234477_i32];
_7.0 = _10 | _10;
Goto(bb3)
}
bb26 = {
_13 = core::ptr::addr_of_mut!(_10);
_10 = '\u{9d0b1}' as i8;
(*_13) = _7.0 ^ _7.0;
_14 = _11 > _11;
_1 = [(-1160818299_i32)];
_6 = _4 << _4;
_6 = _4 * _4;
_11 = _5 as f32;
_4 = _7.0 as usize;
_5 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_11 = _3;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
Goto(bb5)
}
bb27 = {
_4 = _6 | _6;
_8 = [(-29043_i16),31185_i16,(-29047_i16),20643_i16];
_7.2 = (-41960248552322635900711986865295384849_i128) >> _17;
(*_13) = _7.0;
(*_13) = Field::<i8>(Variant(_23, 2), 1) >> _4;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = 14443_u16 as f64;
(*_22) = core::ptr::addr_of_mut!(_19);
_7.0 = !_10;
_19 = 39066606934215755351381605836072405516_u128 as f64;
_26.0 = _18;
(*_22) = core::ptr::addr_of_mut!((*_25));
(*_25) = 8426681873417468078_u64 as f64;
_17 = !2031518427_u32;
(*_13) = _7.2 as i8;
_29 = [_4,_4,_6,_4,_6];
_13 = core::ptr::addr_of_mut!(_7.0);
place!(Field::<u128>(Variant(_23, 2), 0)) = 321045392327256687573112918344628840579_u128;
Goto(bb16)
}
bb28 = {
(*_25) = 16736_i16 as f64;
_35 = _8;
_20 = _24;
_19 = 79_u8 as f64;
_42 = [_37,_37,_37,_5,_5,_5,_37];
_36 = [3026702156005556144_i64,5475566703291470228_i64,3747167345749573357_i64,(-8459230207426540621_i64),(-1424993417139868457_i64)];
_33 = -(*_25);
_31 = 23828_u16 as f32;
_1 = [389509325_i32];
place!(Field::<*mut f64>(Variant(_34, 2), 1)) = (*_22);
_43 = (-30356_i16) >> _10;
_39 = _29;
_34 = Adt53::Variant2 { fld0: 1435863884_i32,fld1: _25,fld2: 250975598525749882964762335237485352868_u128 };
_38.0 = (*_13);
_31 = -_3;
Call((*_22) = fn15((*_13), _19, _38, _7.2, _5, _29, (*_13), _1, _31, _10), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
_9 = _3 * _2;
_40 = _20;
_5 = 210925002620713296168518952618587047528_u128 as isize;
_40 = _27;
_47 = -_5;
_45.0 = _17 as u64;
_28 = [449844209_i32];
_30 = -_9;
_48 = 1499352203014682894_i64 as i16;
_6 = !_4;
_44 = _5;
_36 = [(-4265489940227327840_i64),4607298426599020476_i64,6345487853039461850_i64,(-1020058035916620739_i64),248094596644314898_i64];
place!(Field::<u128>(Variant(_34, 2), 2)) = !313206002359639091888323242769570080133_u128;
_48 = !_43;
Goto(bb30)
}
bb30 = {
_24 = _20;
_7.2 = _24 as i128;
_32 = [_4,_4,_6,_6,_6,_6,_6];
_46.1 = _17 >= _17;
_26.0 = [(*_13),(*_13),_38.0,(*_13),_10,(*_13)];
_29 = [_4,_6,_6,_4,_4];
_17 = 4171639929_u32 + 3818823787_u32;
_51 = (_1,);
_26.0 = [(*_13),(*_13),(*_13),(*_13),_7.0,_10];
_25 = core::ptr::addr_of_mut!(_33);
_24 = _40;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = _33;
_51.0 = [1152632481_i32];
_53 = _6 << _38.0;
_2 = _11 - _31;
_8 = [_43,_48,_48,_48];
_49 = _24;
_30 = -_9;
_13 = core::ptr::addr_of_mut!(_7.0);
_8 = _35;
Goto(bb31)
}
bb31 = {
_54 = [_53,_53,_53,_6,_53,_53,_53];
_54 = [_53,_53,_53,_53,_53,_53,_53];
_43 = _48;
_45 = (10534733345686375878_u64,);
_19 = _45.0 as f64;
_46.1 = _14;
_24 = _20;
_55 = -(-1588717080_i32);
_47 = !_5;
_33 = _37 as f64;
_41 = _51.0;
_22 = core::ptr::addr_of_mut!(place!(Field::<*mut f64>(Variant(_34, 2), 1)));
match _45.0 {
0 => bb1,
1 => bb32,
2 => bb33,
10534733345686375878 => bb35,
_ => bb34
}
}
bb32 = {
_4 = _6 | _6;
_8 = [(-29043_i16),31185_i16,(-29047_i16),20643_i16];
_7.2 = (-41960248552322635900711986865295384849_i128) >> _17;
(*_13) = _7.0;
(*_13) = Field::<i8>(Variant(_23, 2), 1) >> _4;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = 14443_u16 as f64;
(*_22) = core::ptr::addr_of_mut!(_19);
_7.0 = !_10;
_19 = 39066606934215755351381605836072405516_u128 as f64;
_26.0 = _18;
(*_22) = core::ptr::addr_of_mut!((*_25));
(*_25) = 8426681873417468078_u64 as f64;
_17 = !2031518427_u32;
(*_13) = _7.2 as i8;
_29 = [_4,_4,_6,_4,_6];
_13 = core::ptr::addr_of_mut!(_7.0);
place!(Field::<u128>(Variant(_23, 2), 0)) = 321045392327256687573112918344628840579_u128;
Goto(bb16)
}
bb33 = {
_5 = 9223372036854775807_isize + (-127_isize);
_7.2 = 54110869121924673751945785240117391724_i128;
_7.0 = 25_i8;
_7.2 = 946184424_u32 as i128;
_6 = _4 + _4;
_4 = !_6;
_1 = [(-822070271_i32)];
_1 = [1384749520_i32];
_7.0 = (-44_i8);
_3 = _2 - _2;
_9 = 113_u8 as f32;
_6 = _4;
_7.1 = [true];
_1 = [(-1361711918_i32)];
_7.0 = -(-26_i8);
Goto(bb2)
}
bb34 = {
_7.2 = _5 as i128;
_7.0 = -47_i8;
_8 = [30734_i16,(-8381_i16),(-14891_i16),(-2725_i16)];
_10 = _7.0;
_4 = 40487_u16 as usize;
_11 = -_2;
_4 = _6;
_12 = [_5,_5,_5,_5,_5,_5,_5];
_7.1 = [false];
_7.1 = [true];
_9 = _2;
_6 = !_4;
_1 = [1798234477_i32];
_7.0 = _10 | _10;
Goto(bb3)
}
bb35 = {
_16 = [_38.2,_38.2,_7.2,_38.2,_7.2,_7.2];
_57 = !_5;
_55 = _38.2 as i32;
_17 = 510691165_u32;
_45.0 = 11469847975901335299_u64 >> _53;
_9 = _30 - _2;
_27 = _20;
_46 = (_31, _14);
_18 = [_7.0,_7.0,(*_13),_7.0,_10,(*_13)];
_67 = _45.0 << _43;
place!(Field::<i8>(Variant(_23, 2), 1)) = (*_13) + (*_13);
_68 = [_55];
_48 = !_43;
_69 = [_55];
(*_25) = _55 as f64;
_56 = _55;
(*_22) = _25;
_25 = core::ptr::addr_of_mut!((*_25));
_46.1 = _14;
_56 = _55;
_33 = (*_25) * (*_25);
_62 = _3 - _31;
_38.1 = [_14];
_30 = -_3;
Call(place!(Field::<u128>(Variant(_34, 2), 2)) = fn17(_45.0, _2, _41), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
_45 = (_67,);
_54 = _32;
_66.1 = _46.1;
_16 = [_7.2,_38.2,_38.2,_7.2,_38.2,_38.2];
_40 = _27;
_14 = _46.1;
place!(Field::<u128>(Variant(_23, 2), 0)) = !Field::<u128>(Variant(_34, 2), 2);
(*_22) = _25;
match _17 {
510691165 => bb37,
_ => bb9
}
}
bb37 = {
_22 = core::ptr::addr_of_mut!(_25);
SetDiscriminant(_23, 0);
_7.1 = _38.1;
_2 = _47 as f32;
(*_25) = Field::<u128>(Variant(_34, 2), 2) as f64;
_23 = Adt48::Variant2 { fld0: Field::<u128>(Variant(_34, 2), 2),fld1: _38.0 };
Goto(bb38)
}
bb38 = {
_71 = _18;
_34 = Adt53::Variant2 { fld0: _56,fld1: (*_22),fld2: Field::<u128>(Variant(_23, 2), 0) };
_7 = (_10, _38.1, _38.2);
_33 = (*_25) * _19;
_50 = _53;
Goto(bb39)
}
bb39 = {
_5 = _37;
(*_25) = _33 + _33;
_47 = _5;
_2 = _9 - _62;
_66 = _46;
_65 = _12;
_68 = [_55];
_29 = [_50,_53,_50,_53,_53];
_73 = _21;
_49 = _40;
place!(Field::<u128>(Variant(_34, 2), 2)) = Field::<u128>(Variant(_23, 2), 0);
_34 = Adt53::Variant0 { fld0: Field::<u128>(Variant(_23, 2), 0),fld1: _45 };
match _17 {
0 => bb16,
1 => bb12,
2 => bb40,
3 => bb41,
4 => bb42,
510691165 => bb44,
_ => bb43
}
}
bb40 = {
_24 = _20;
_7.2 = _24 as i128;
_32 = [_4,_4,_6,_6,_6,_6,_6];
_46.1 = _17 >= _17;
_26.0 = [(*_13),(*_13),_38.0,(*_13),_10,(*_13)];
_29 = [_4,_6,_6,_4,_4];
_17 = 4171639929_u32 + 3818823787_u32;
_51 = (_1,);
_26.0 = [(*_13),(*_13),(*_13),(*_13),_7.0,_10];
_25 = core::ptr::addr_of_mut!(_33);
_24 = _40;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = _33;
_51.0 = [1152632481_i32];
_53 = _6 << _38.0;
_2 = _11 - _31;
_8 = [_43,_48,_48,_48];
_49 = _24;
_30 = -_9;
_13 = core::ptr::addr_of_mut!(_7.0);
_8 = _35;
Goto(bb31)
}
bb41 = {
_5 = 9223372036854775807_isize + (-127_isize);
_7.2 = 54110869121924673751945785240117391724_i128;
_7.0 = 25_i8;
_7.2 = 946184424_u32 as i128;
_6 = _4 + _4;
_4 = !_6;
_1 = [(-822070271_i32)];
_1 = [1384749520_i32];
_7.0 = (-44_i8);
_3 = _2 - _2;
_9 = 113_u8 as f32;
_6 = _4;
_7.1 = [true];
_1 = [(-1361711918_i32)];
_7.0 = -(-26_i8);
Goto(bb2)
}
bb42 = {
_4 = _6 | _6;
_8 = [(-29043_i16),31185_i16,(-29047_i16),20643_i16];
_7.2 = (-41960248552322635900711986865295384849_i128) >> _17;
(*_13) = _7.0;
(*_13) = Field::<i8>(Variant(_23, 2), 1) >> _4;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = 14443_u16 as f64;
(*_22) = core::ptr::addr_of_mut!(_19);
_7.0 = !_10;
_19 = 39066606934215755351381605836072405516_u128 as f64;
_26.0 = _18;
(*_22) = core::ptr::addr_of_mut!((*_25));
(*_25) = 8426681873417468078_u64 as f64;
_17 = !2031518427_u32;
(*_13) = _7.2 as i8;
_29 = [_4,_4,_6,_4,_6];
_13 = core::ptr::addr_of_mut!(_7.0);
place!(Field::<u128>(Variant(_23, 2), 0)) = 321045392327256687573112918344628840579_u128;
Goto(bb16)
}
bb43 = {
_17 = !2476766830_u32;
_10 = _14 as i8;
_20 = '\u{7c40f}';
_14 = true ^ true;
_5 = 93_isize;
_14 = !true;
_16 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
match _5 {
0 => bb1,
93 => bb13,
_ => bb4
}
}
bb44 = {
_64 = core::ptr::addr_of_mut!(_66.1);
_52 = Adt48::Variant2 { fld0: Field::<u128>(Variant(_34, 0), 0),fld1: _10 };
_70 = [_53,_53,_53,_50,_50];
_76.3 = core::ptr::addr_of_mut!(_14);
_7.2 = _38.2 << Field::<(u64,)>(Variant(_34, 0), 1).0;
_51 = (_69,);
(*_22) = core::ptr::addr_of_mut!((*_25));
_36 = [(-8630120857144248430_i64),3945211414490733302_i64,1195282080132988769_i64,(-1400965416663735484_i64),(-4867550430561335563_i64)];
_54 = [_50,_50,_53,_53,_50,_53,_50];
_47 = _57 + _5;
(*_25) = _33;
_75 = _35;
_51.0 = _69;
_46.0 = _11;
_51.0 = [_56];
_38.1 = [_66.1];
_24 = _49;
_59 = [_47,_37,_47,_37,_57,_47,_57];
_50 = _46.1 as usize;
_21 = [_53,_53,_50,_50,_53,_50,_53];
place!(Field::<(u64,)>(Variant(_34, 0), 1)).0 = _45.0;
_58 = !31796585633153347_i64;
match _17 {
0 => bb34,
1 => bb30,
2 => bb12,
3 => bb27,
510691165 => bb45,
_ => bb17
}
}
bb45 = {
_45 = Field::<(u64,)>(Variant(_34, 0), 1);
_51 = (_68,);
place!(Field::<(u64,)>(Variant(_34, 0), 1)).0 = _7.2 as u64;
_13 = core::ptr::addr_of_mut!((*_13));
_70 = [_53,_50,_53,_53,_50];
_46 = _66;
_54 = _21;
_81 = [_50,_53,_53,_53,_53,_53,_53];
_16 = [_7.2,_7.2,_38.2,_7.2,_38.2,_7.2];
_72 = [_53,_53,_53,_53,_4];
_81 = [_50,_53,_53,_50,_53,_53,_53];
_73 = [_50,_6,_4,_50,_6,_53,_53];
_74 = _51;
_22 = core::ptr::addr_of_mut!((*_22));
_59 = [_47,_47,_37,_5,_57,_47,_5];
_26.0 = [_38.0,Field::<i8>(Variant(_23, 2), 1),Field::<i8>(Variant(_23, 2), 1),Field::<i8>(Variant(_23, 2), 1),_10,(*_13)];
Goto(bb46)
}
bb46 = {
_55 = -_56;
place!(Field::<i8>(Variant(_23, 2), 1)) = -(*_13);
_18 = _71;
_72 = [_53,_53,_53,_53,_53];
(*_25) = -_33;
_76.3 = core::ptr::addr_of_mut!(_46.1);
_1 = [_55];
_66 = _46;
_32 = _73;
_61 = -_47;
_65 = [_44,_47,_47,_44,_47,_44,_5];
_29 = [_53,_53,_4,_53,_53];
_47 = _61 << Field::<(u64,)>(Variant(_34, 0), 1).0;
_3 = -_9;
match _17 {
510691165 => bb47,
_ => bb2
}
}
bb47 = {
_49 = _24;
_47 = _5 * _37;
_24 = _20;
_76.1 = _47 >> Field::<(u64,)>(Variant(_34, 0), 1).0;
_67 = _45.0 + Field::<(u64,)>(Variant(_34, 0), 1).0;
_73 = _54;
_51 = _74;
_60 = [(*_13),_10,(*_13),_38.0,_7.0,_7.0];
_51 = (_69,);
_25 = core::ptr::addr_of_mut!((*_25));
_76.3 = core::ptr::addr_of_mut!(_87);
_70 = [_4,_53,_53,_53,_4];
_76.3 = core::ptr::addr_of_mut!(_46.1);
_36 = [_58,_58,_58,_58,_58];
SetDiscriminant(_23, 2);
(*_22) = core::ptr::addr_of_mut!(_19);
_21 = [_4,_53,_53,_50,_53,_53,_53];
place!(Field::<u128>(Variant(_23, 2), 0)) = _76.1 as u128;
SetDiscriminant(_34, 1);
_33 = _47 as f64;
_90 = (_27, _55, _41);
_69 = [_90.1];
_32 = [_50,_50,_53,_53,_53,_50,_4];
_84 = [_58,_58,_58,_58,_58];
Goto(bb48)
}
bb48 = {
(*_64) = _9 >= _2;
_66.1 = _46.1 & _46.1;
_90.0 = _40;
_11 = -_9;
_57 = 46368_u16 as isize;
_7.1 = [(*_64)];
_35 = [_48,_48,_48,_43];
_92 = (*_64);
place!(Field::<i8>(Variant(_52, 2), 1)) = _38.0 << _48;
match _17 {
0 => bb40,
1 => bb49,
2 => bb50,
3 => bb51,
4 => bb52,
5 => bb53,
510691165 => bb55,
_ => bb54
}
}
bb49 = {
_71 = _18;
_34 = Adt53::Variant2 { fld0: _56,fld1: (*_22),fld2: Field::<u128>(Variant(_23, 2), 0) };
_7 = (_10, _38.1, _38.2);
_33 = (*_25) * _19;
_50 = _53;
Goto(bb39)
}
bb50 = {
_28 = [(-1226823825_i32)];
_37 = !_5;
(*_13) = _10 >> _10;
_11 = -_3;
_38.2 = !_7.2;
_38 = (_7.0, _7.1, _7.2);
_24 = _27;
_24 = _27;
_3 = (*_13) as f32;
_35 = [(-11989_i16),13886_i16,16250_i16,(-458_i16)];
_1 = _28;
_18 = [_38.0,_7.0,(*_13),_10,_10,(*_13)];
_32 = [_4,_6,_6,_4,_6,_6,_6];
_34 = Adt53::Variant2 { fld0: (-858894851_i32),fld1: _25,fld2: 180665074891008834581779841272163228662_u128 };
_24 = _20;
match _17 {
0 => bb24,
1 => bb25,
2 => bb26,
1272743435 => bb28,
_ => bb27
}
}
bb51 = {
_45 = (_67,);
_54 = _32;
_66.1 = _46.1;
_16 = [_7.2,_38.2,_38.2,_7.2,_38.2,_38.2];
_40 = _27;
_14 = _46.1;
place!(Field::<u128>(Variant(_23, 2), 0)) = !Field::<u128>(Variant(_34, 2), 2);
(*_22) = _25;
match _17 {
510691165 => bb37,
_ => bb9
}
}
bb52 = {
_24 = _20;
_7.2 = _24 as i128;
_32 = [_4,_4,_6,_6,_6,_6,_6];
_46.1 = _17 >= _17;
_26.0 = [(*_13),(*_13),_38.0,(*_13),_10,(*_13)];
_29 = [_4,_6,_6,_4,_4];
_17 = 4171639929_u32 + 3818823787_u32;
_51 = (_1,);
_26.0 = [(*_13),(*_13),(*_13),(*_13),_7.0,_10];
_25 = core::ptr::addr_of_mut!(_33);
_24 = _40;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = _33;
_51.0 = [1152632481_i32];
_53 = _6 << _38.0;
_2 = _11 - _31;
_8 = [_43,_48,_48,_48];
_49 = _24;
_30 = -_9;
_13 = core::ptr::addr_of_mut!(_7.0);
_8 = _35;
Goto(bb31)
}
bb53 = {
_24 = _20;
_7.2 = _24 as i128;
_32 = [_4,_4,_6,_6,_6,_6,_6];
_46.1 = _17 >= _17;
_26.0 = [(*_13),(*_13),_38.0,(*_13),_10,(*_13)];
_29 = [_4,_6,_6,_4,_4];
_17 = 4171639929_u32 + 3818823787_u32;
_51 = (_1,);
_26.0 = [(*_13),(*_13),(*_13),(*_13),_7.0,_10];
_25 = core::ptr::addr_of_mut!(_33);
_24 = _40;
_25 = core::ptr::addr_of_mut!(_19);
(*_25) = _33;
_51.0 = [1152632481_i32];
_53 = _6 << _38.0;
_2 = _11 - _31;
_8 = [_43,_48,_48,_48];
_49 = _24;
_30 = -_9;
_13 = core::ptr::addr_of_mut!(_7.0);
_8 = _35;
Goto(bb31)
}
bb54 = {
_17 = 1690025854_u32 & 1461214099_u32;
place!(Field::<i8>(Variant(_23, 2), 1)) = _10;
_14 = _20 == _24;
_19 = 144_u8 as f64;
_17 = (-10915_i16) as u32;
_8 = [7212_i16,6129_i16,(-25824_i16),10389_i16];
_2 = _3 * _11;
Goto(bb15)
}
bb55 = {
_88 = _90.1 & _55;
_47 = _76.1 + _76.1;
place!(Field::<u128>(Variant(_52, 2), 0)) = Field::<u128>(Variant(_23, 2), 0) ^ Field::<u128>(Variant(_23, 2), 0);
_10 = _66.1 as i8;
Goto(bb56)
}
bb56 = {
(*_64) = _45.0 <= _67;
_17 = Field::<u128>(Variant(_52, 2), 0) as u32;
_46 = (_62, (*_64));
_95 = _2 as i64;
_79 = _7.2 as f32;
_80 = Adt57::Variant1 { fld0: _28,fld1: (*_25),fld2: Field::<u128>(Variant(_52, 2), 0),fld3: (*_13),fld4: _16,fld5: _46.0,fld6: _64 };
_41 = [_88];
_94 = core::ptr::addr_of!(place!(Field::<*mut bool>(Variant(_80, 1), 6)));
_21 = [_6,_53,_53,_53,_50,_4,_6];
_51.0 = _41;
_48 = _43;
_61 = _47;
place!(Field::<i8>(Variant(_23, 2), 1)) = _17 as i8;
_1 = [_88];
SetDiscriminant(_80, 2);
place!(Field::<u128>(Variant(_52, 2), 0)) = !Field::<u128>(Variant(_23, 2), 0);
place!(Field::<i8>(Variant(_23, 2), 1)) = _7.0 ^ (*_13);
place!(Field::<(char, i32, [i32; 1])>(Variant(_80, 2), 3)) = _90;
_66 = (_2, _46.1);
_5 = _47 | _47;
_79 = _66.0 + _3;
_18 = [Field::<i8>(Variant(_52, 2), 1),_10,Field::<i8>(Variant(_52, 2), 1),Field::<i8>(Variant(_52, 2), 1),_38.0,_38.0];
_63 = -_47;
_26 = (_60,);
Goto(bb57)
}
bb57 = {
_94 = core::ptr::addr_of!(_64);
_68 = [_56];
_25 = core::ptr::addr_of_mut!(_33);
_57 = _47;
_94 = core::ptr::addr_of!(_64);
_37 = -_5;
(*_25) = 71_u8 as f64;
_98 = -_37;
_60 = [Field::<i8>(Variant(_52, 2), 1),_38.0,Field::<i8>(Variant(_52, 2), 1),Field::<i8>(Variant(_52, 2), 1),(*_13),Field::<i8>(Variant(_23, 2), 1)];
SetDiscriminant(_23, 2);
Goto(bb58)
}
bb58 = {
(*_13) = _38.0 << _5;
place!(Field::<u32>(Variant(_80, 2), 0)) = _17 ^ _17;
_29 = [_50,_4,_53,_53,_50];
place!(Field::<u32>(Variant(_80, 2), 0)) = (*_25) as u32;
_82 = Adt59::Variant1 { fld0: _7.0,fld1: _36,fld2: _26 };
_63 = !_37;
RET = core::ptr::addr_of!(_105);
place!(Field::<u128>(Variant(_23, 2), 0)) = _67 as u128;
_62 = _3 + _66.0;
_89 = [_50,_53,_4,_53,_53];
_66.1 = _46.1;
_25 = core::ptr::addr_of_mut!((*_25));
_97 = _66.1 ^ _46.1;
Goto(bb59)
}
bb59 = {
Call(_107 = dump_var(11_usize, 27_usize, Move(_27), 60_usize, Move(_60), 58_usize, Move(_58), 18_usize, Move(_18)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_107 = dump_var(11_usize, 20_usize, Move(_20), 6_usize, Move(_6), 17_usize, Move(_17), 40_usize, Move(_40)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_107 = dump_var(11_usize, 56_usize, Move(_56), 57_usize, Move(_57), 61_usize, Move(_61), 67_usize, Move(_67)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_107 = dump_var(11_usize, 74_usize, Move(_74), 7_usize, Move(_7), 35_usize, Move(_35), 63_usize, Move(_63)), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Call(_107 = dump_var(11_usize, 72_usize, Move(_72), 32_usize, Move(_32), 4_usize, Move(_4), 70_usize, Move(_70)), ReturnTo(bb64), UnwindUnreachable())
}
bb64 = {
Call(_107 = dump_var(11_usize, 55_usize, Move(_55), 98_usize, Move(_98), 45_usize, Move(_45), 28_usize, Move(_28)), ReturnTo(bb65), UnwindUnreachable())
}
bb65 = {
Call(_107 = dump_var(11_usize, 59_usize, Move(_59), 37_usize, Move(_37), 69_usize, Move(_69), 24_usize, Move(_24)), ReturnTo(bb66), UnwindUnreachable())
}
bb66 = {
Call(_107 = dump_var(11_usize, 26_usize, Move(_26), 53_usize, Move(_53), 5_usize, Move(_5), 95_usize, Move(_95)), ReturnTo(bb67), UnwindUnreachable())
}
bb67 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: f32,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: f32,mut _10: isize,mut _11: f32,mut _12: isize,mut _13: f32,mut _14: f32,mut _15: usize) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _16: f64;
let _17: Adt52;
let _18: (char, i32, [i32; 1]);
let _19: f64;
let _20: isize;
let _21: f32;
let _22: isize;
let _23: usize;
let _24: [i128; 6];
let _25: [i8; 6];
let _26: usize;
let _27: u8;
let _28: isize;
let _29: isize;
let _30: (char, i32, [i32; 1]);
let _31: ([i32; 1],);
let _32: [i64; 5];
let _33: (f32, bool);
let _34: Adt60;
let _35: ();
let _36: ();
{
_8 = 13158998410372436976_u64 as isize;
_5 = _6;
_11 = _9;
_9 = _4 + _13;
Goto(bb1)
}
bb1 = {
RET = [false];
_16 = _15 as f64;
_17.fld2 = [(-2001497964347421759_i64),8793423266415246749_i64,(-4797251933629676870_i64),(-8282322002200726663_i64),6814268796818326570_i64];
_18.1 = !1700037616_i32;
_2 = -_3;
_17.fld1 = _15 * _15;
_17.fld3 = RET;
_18.1 = (-1537962217_i32);
Goto(bb2)
}
bb2 = {
_18.1 = -(-1372221738_i32);
_17.fld3 = RET;
_11 = _4;
_17.fld2 = [20335395485272336_i64,(-8338249315949404334_i64),3867718420629628867_i64,7178677212567257955_i64,5270704565886817152_i64];
Call(_18.1 = fn13(_9, _11, _1, _7, _17.fld1, _14, _17.fld1, _16, _10, _14, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = _10 | _7;
_24 = [(-85383667028719596439780910475976207620_i128),21482644554204404432399384709028814862_i128,39963990409423433827939542757766933783_i128,159823794082745124541090761018448495252_i128,(-15421220321321251634003003772057939543_i128),(-1918728306658401952857586593448649288_i128)];
_18.0 = '\u{3e7de}';
_22 = 218_u8 as isize;
_9 = _18.1 as f32;
_23 = _17.fld1;
_17.fld1 = !_23;
_5 = 163783957297920143807703576250305801425_i128 as isize;
_8 = -_10;
_18.2 = [_18.1];
_5 = _8;
_16 = _6 as f64;
_1 = _10 << _8;
_9 = _13;
_2 = 7172686235497032749_i64 as isize;
_9 = _11;
_23 = 31292_u16 as usize;
_20 = _5 + _3;
_3 = -_20;
_26 = _17.fld1;
Call(_27 = fn14(_17.fld1, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = [true];
_6 = 112590693931606055943972784896397012446_i128 as isize;
_16 = (-48_i8) as f64;
_7 = -_3;
_1 = !_3;
_29 = _1 + _3;
_22 = _3 & _8;
_25 = [(-78_i8),46_i8,78_i8,(-12_i8),(-124_i8),(-89_i8)];
_4 = _9 - _13;
_10 = (-17303_i16) as isize;
_18.1 = (-554765445_i32);
RET = [false];
_15 = _17.fld1 >> _8;
_2 = (-7769603924256292190_i64) as isize;
_20 = _7 << _17.fld1;
_31 = (_18.2,);
_30 = (_18.0, _18.1, _31.0);
_17.fld1 = 112804349469690021260222746088425341000_u128 as usize;
Call(_9 = core::intrinsics::transmute(_31.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_26 = _15 - _15;
_20 = 1251003814_u32 as isize;
match _18.1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463463374607431213446011 => bb14,
_ => bb13
}
}
bb6 = {
RET = [true];
_6 = 112590693931606055943972784896397012446_i128 as isize;
_16 = (-48_i8) as f64;
_7 = -_3;
_1 = !_3;
_29 = _1 + _3;
_22 = _3 & _8;
_25 = [(-78_i8),46_i8,78_i8,(-12_i8),(-124_i8),(-89_i8)];
_4 = _9 - _13;
_10 = (-17303_i16) as isize;
_18.1 = (-554765445_i32);
RET = [false];
_15 = _17.fld1 >> _8;
_2 = (-7769603924256292190_i64) as isize;
_20 = _7 << _17.fld1;
_31 = (_18.2,);
_30 = (_18.0, _18.1, _31.0);
_17.fld1 = 112804349469690021260222746088425341000_u128 as usize;
Call(_9 = core::intrinsics::transmute(_31.0), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_5 = _10 | _7;
_24 = [(-85383667028719596439780910475976207620_i128),21482644554204404432399384709028814862_i128,39963990409423433827939542757766933783_i128,159823794082745124541090761018448495252_i128,(-15421220321321251634003003772057939543_i128),(-1918728306658401952857586593448649288_i128)];
_18.0 = '\u{3e7de}';
_22 = 218_u8 as isize;
_9 = _18.1 as f32;
_23 = _17.fld1;
_17.fld1 = !_23;
_5 = 163783957297920143807703576250305801425_i128 as isize;
_8 = -_10;
_18.2 = [_18.1];
_5 = _8;
_16 = _6 as f64;
_1 = _10 << _8;
_9 = _13;
_2 = 7172686235497032749_i64 as isize;
_9 = _11;
_23 = 31292_u16 as usize;
_20 = _5 + _3;
_3 = -_20;
_26 = _17.fld1;
Call(_27 = fn14(_17.fld1, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_18.1 = -(-1372221738_i32);
_17.fld3 = RET;
_11 = _4;
_17.fld2 = [20335395485272336_i64,(-8338249315949404334_i64),3867718420629628867_i64,7178677212567257955_i64,5270704565886817152_i64];
Call(_18.1 = fn13(_9, _11, _1, _7, _17.fld1, _14, _17.fld1, _16, _10, _14, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
RET = [false];
_16 = _15 as f64;
_17.fld2 = [(-2001497964347421759_i64),8793423266415246749_i64,(-4797251933629676870_i64),(-8282322002200726663_i64),6814268796818326570_i64];
_18.1 = !1700037616_i32;
_2 = -_3;
_17.fld1 = _15 * _15;
_17.fld3 = RET;
_18.1 = (-1537962217_i32);
Goto(bb2)
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
_18 = _30;
_18.1 = _30.1;
_17.fld3 = RET;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(12_usize, 26_usize, Move(_26), 6_usize, Move(_6), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(12_usize, 7_usize, Move(_7), 31_usize, Move(_31), 15_usize, Move(_15), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(12_usize, 18_usize, Move(_18), 24_usize, Move(_24), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: f32,mut _2: f32,mut _3: isize,mut _4: isize,mut _5: usize,mut _6: f32,mut _7: usize,mut _8: f64,mut _9: isize,mut _10: f32,mut _11: usize) -> i32 {
mir! {
type RET = i32;
let _12: i8;
let _13: [isize; 7];
let _14: Adt61;
let _15: isize;
let _16: *const usize;
let _17: ([i8; 6],);
let _18: Adt53;
let _19: [i128; 6];
let _20: f32;
let _21: Adt55;
let _22: char;
let _23: ();
let _24: ();
{
_7 = _11;
_12 = 52959_u16 as i8;
_7 = _11;
RET = -(-1496143274_i32);
_12 = (-38_i8);
_2 = _9 as f32;
_13 = [_4,_3,_3,_4,_4,_9,_4];
_10 = _6;
RET = !949334859_i32;
_7 = _11;
_8 = 78708058362141089202008129519774226534_i128 as f64;
_5 = !_11;
_8 = 174_u8 as f64;
_8 = 649737884_u32 as f64;
_3 = _4;
_6 = (-14213_i16) as f32;
_4 = _3;
_2 = _1;
_10 = (-5387155310601965179_i64) as f32;
RET = 132992276189177897456699182530962852403_u128 as i32;
_13 = [_3,_3,_4,_4,_3,_3,_4];
_9 = _3;
_11 = _7;
RET = !1345328410_i32;
RET = -1696328800_i32;
match _12 {
0 => bb1,
1 => bb2,
340282366920938463463374607431768211418 => bb4,
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
_5 = _11 << _11;
_10 = _8 as f32;
_9 = _11 as isize;
RET = 332227053001633208569718928948973916824_u128 as i32;
RET = (-833032860_i32) + 203471398_i32;
match _12 {
0 => bb1,
1 => bb2,
340282366920938463463374607431768211418 => bb5,
_ => bb3
}
}
bb5 = {
_12 = 197_u8 as i8;
RET = (-202888483_i32);
_5 = 6460322314071656927_u64 as usize;
_15 = _4 * _3;
RET = -1715823569_i32;
RET = 1688192536_i32;
_12 = 111_i8;
_5 = _11 << _15;
_10 = -_1;
_5 = !_7;
RET = 95_u8 as i32;
_8 = _1 as f64;
_10 = _2 + _2;
_7 = _5;
_10 = 5293699966984114405_u64 as f32;
_6 = _2 + _1;
_12 = -28_i8;
_9 = _15;
_10 = _8 as f32;
Goto(bb6)
}
bb6 = {
RET = !(-873692416_i32);
_5 = !_7;
_20 = -_2;
_11 = _5 - _5;
_16 = core::ptr::addr_of!(_11);
_4 = _9 & _3;
_15 = _4 & _3;
_13 = [_15,_3,_15,_9,_9,_15,_15];
_2 = _20 * _10;
_17.0 = [_12,_12,_12,_12,_12,_12];
_13 = [_9,_3,_4,_4,_3,_4,_15];
(*_16) = 134766353617268277799295406066642360662_i128 as usize;
_16 = core::ptr::addr_of!(_11);
_20 = -_2;
_21.fld2 = [1836532978_u32,2764381783_u32,2648968085_u32,3486435521_u32,2290141816_u32];
_13 = [_4,_15,_9,_15,_9,_15,_15];
_6 = _1 * _10;
(*_16) = _5 ^ _7;
_21.fld3 = [RET];
Goto(bb7)
}
bb7 = {
_19 = [(-10340992879930914288274110237653516991_i128),(-129409045372961560587946584757806452027_i128),17602128678436800178929704599570090249_i128,(-33129277687252743352290589277306457085_i128),(-22654140161491802166578569048727772060_i128),51853814057447464467403602159218968538_i128];
_21.fld1 = [_12,_12,_12,_12,_12,_12,_12,_12];
_20 = -_1;
_17.0 = [_12,_12,_12,_12,_12,_12];
_13 = [_15,_3,_9,_15,_4,_4,_9];
_2 = _1;
(*_16) = !_5;
_21.fld3 = [RET];
_1 = _20;
_21.fld1 = [_12,_12,_12,_12,_12,_12,_12,_12];
RET = _6 as i32;
RET = 1173860089_i32 >> (*_16);
Goto(bb8)
}
bb8 = {
Call(_23 = dump_var(13_usize, 9_usize, Move(_9), 15_usize, Move(_15), 11_usize, Move(_11), 19_usize, Move(_19)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_23 = dump_var(13_usize, 5_usize, Move(_5), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: usize,mut _2: f32) -> u8 {
mir! {
type RET = u8;
let _3: (f32, bool);
let _4: bool;
let _5: [bool; 1];
let _6: f32;
let _7: i64;
let _8: [i8; 6];
let _9: (i8, [bool; 1], i128);
let _10: isize;
let _11: (char, i32, [i32; 1]);
let _12: isize;
let _13: u32;
let _14: [i64; 5];
let _15: (u64,);
let _16: i64;
let _17: Adt58;
let _18: i32;
let _19: u16;
let _20: isize;
let _21: isize;
let _22: [usize; 5];
let _23: isize;
let _24: ([i32; 1],);
let _25: *mut bool;
let _26: *mut i8;
let _27: ();
let _28: ();
{
RET = 22_u8 >> _1;
_3.0 = _2 * _2;
_3.1 = !false;
_1 = 217265816983491428_usize | 15307397914148072945_usize;
RET = 147_u8 | 133_u8;
RET = 80_u8 - 147_u8;
Goto(bb1)
}
bb1 = {
_1 = RET as usize;
_3.0 = _2;
_4 = _3.1;
_5 = [_4];
_2 = _3.0;
_3.1 = !_4;
_3.0 = -_2;
RET = !57_u8;
_2 = (-354552009_i32) as f32;
_6 = 58647_u16 as f32;
_2 = _3.0;
_6 = _3.0;
_1 = 7_usize ^ 15334781715727364916_usize;
_4 = _3.0 >= _3.0;
RET = 251_u8;
_1 = !7_usize;
_2 = 16724352810462901948_u64 as f32;
_3 = (_6, _4);
_8 = [(-93_i8),(-98_i8),66_i8,72_i8,(-62_i8),122_i8];
RET = 143_u8;
Call(_1 = core::intrinsics::bswap(4_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = (_6, _4);
_3.1 = _4 & _4;
_9.0 = 1860286689_u32 as i8;
_7 = -6509858661453724142_i64;
_5 = [_3.1];
_7 = (-402824207446981976_i64) + 4030370961260087939_i64;
_8 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_10 = (-9223372036854775808_isize);
RET = 58_u8 ^ 99_u8;
_1 = 3_usize * 14691279983393334308_usize;
_1 = 7125363988400054180_u64 as usize;
_3.0 = -_6;
_9.1 = [_3.1];
_11.2 = [892488443_i32];
_11.1 = (-1661055126_i32);
_11.2 = [_11.1];
_9.1 = [_4];
_3 = (_6, _4);
_2 = _10 as f32;
_11.2 = [_11.1];
_9.2 = -87368548278423692553025614703594344867_i128;
_6 = _3.0;
match _11.1 {
0 => bb3,
1 => bb4,
2 => bb5,
340282366920938463463374607430107156330 => bb7,
_ => bb6
}
}
bb3 = {
_1 = RET as usize;
_3.0 = _2;
_4 = _3.1;
_5 = [_4];
_2 = _3.0;
_3.1 = !_4;
_3.0 = -_2;
RET = !57_u8;
_2 = (-354552009_i32) as f32;
_6 = 58647_u16 as f32;
_2 = _3.0;
_6 = _3.0;
_1 = 7_usize ^ 15334781715727364916_usize;
_4 = _3.0 >= _3.0;
RET = 251_u8;
_1 = !7_usize;
_2 = 16724352810462901948_u64 as f32;
_3 = (_6, _4);
_8 = [(-93_i8),(-98_i8),66_i8,72_i8,(-62_i8),122_i8];
RET = 143_u8;
Call(_1 = core::intrinsics::bswap(4_usize), ReturnTo(bb2), UnwindUnreachable())
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
_7 = (-2235419991552053026_i64) >> _1;
_9.1 = _5;
_9.0 = -(-17_i8);
_3 = (_6, _4);
_7 = 2858515327919139502_i64 - (-1584688012141059901_i64);
_11.0 = '\u{cefa8}';
_9.2 = (-37632819572981997811050093728300461299_i128);
RET = 127_u8 + 103_u8;
_13 = (-12163_i16) as u32;
RET = 2069315867679513933_u64 as u8;
RET = 161_u8;
_3.0 = _1 as f32;
RET = !60_u8;
_14 = [_7,_7,_7,_7,_7];
_11.0 = '\u{61c18}';
_15 = (4658518688787914365_u64,);
_9 = ((-36_i8), _5, (-15824845088938887713387294765830547177_i128));
RET = _1 as u8;
_11.1 = -1034581731_i32;
_1 = 3_usize * 5_usize;
_12 = _10;
_3 = (_6, _4);
Goto(bb8)
}
bb8 = {
_9.1 = _5;
_5 = _9.1;
_9.2 = RET as i128;
_8 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_18 = _11.1 | _11.1;
_4 = _9.0 < _9.0;
_15 = (13168414179364489252_u64,);
_10 = _3.0 as isize;
_18 = -_11.1;
_6 = -_3.0;
_9.1 = [_4];
_15 = (8633018171895526869_u64,);
_9.0 = (-75_i8) << _10;
_3.1 = !_4;
_9.0 = (-42_i8) << _10;
_7 = (-4917954085567896363_i64);
_9.0 = 48_i8 ^ (-52_i8);
_1 = !2_usize;
_4 = _3.1 | _3.1;
_5 = [_4];
_11.0 = '\u{7dd69}';
_3 = (_6, _4);
_8 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_15 = (16042055674336199543_u64,);
_13 = 308816525249496414120036106582565502995_u128 as u32;
_6 = -_3.0;
_4 = _3.1 == _3.1;
RET = 77_u8;
match _7 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463458456653346200315093 => bb13,
_ => bb12
}
}
bb9 = {
_7 = (-2235419991552053026_i64) >> _1;
_9.1 = _5;
_9.0 = -(-17_i8);
_3 = (_6, _4);
_7 = 2858515327919139502_i64 - (-1584688012141059901_i64);
_11.0 = '\u{cefa8}';
_9.2 = (-37632819572981997811050093728300461299_i128);
RET = 127_u8 + 103_u8;
_13 = (-12163_i16) as u32;
RET = 2069315867679513933_u64 as u8;
RET = 161_u8;
_3.0 = _1 as f32;
RET = !60_u8;
_14 = [_7,_7,_7,_7,_7];
_11.0 = '\u{61c18}';
_15 = (4658518688787914365_u64,);
_9 = ((-36_i8), _5, (-15824845088938887713387294765830547177_i128));
RET = _1 as u8;
_11.1 = -1034581731_i32;
_1 = 3_usize * 5_usize;
_12 = _10;
_3 = (_6, _4);
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_1 = RET as usize;
_3.0 = _2;
_4 = _3.1;
_5 = [_4];
_2 = _3.0;
_3.1 = !_4;
_3.0 = -_2;
RET = !57_u8;
_2 = (-354552009_i32) as f32;
_6 = 58647_u16 as f32;
_2 = _3.0;
_6 = _3.0;
_1 = 7_usize ^ 15334781715727364916_usize;
_4 = _3.0 >= _3.0;
RET = 251_u8;
_1 = !7_usize;
_2 = 16724352810462901948_u64 as f32;
_3 = (_6, _4);
_8 = [(-93_i8),(-98_i8),66_i8,72_i8,(-62_i8),122_i8];
RET = 143_u8;
Call(_1 = core::intrinsics::bswap(4_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_9.1 = [_4];
_20 = !_10;
_22 = [_1,_1,_1,_1,_1];
_22 = [_1,_1,_1,_1,_1];
_9 = ((-73_i8), _5, 166346803055688985714067411103138318598_i128);
_19 = 45650_u16 >> _20;
_6 = -_3.0;
_1 = !1_usize;
_11.1 = _18 ^ _18;
_9.2 = -(-41624407136599768534889048075294948067_i128);
_3 = (_6, _4);
_15.0 = 14306405996094281100_u64 - 14931056864258623416_u64;
_8 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_16 = _7;
_9.2 = RET as i128;
_20 = -_10;
_5 = _9.1;
_21 = _1 as isize;
_6 = _3.0 - _3.0;
_15 = (16221601011501264617_u64,);
_9 = ((-104_i8), _5, (-150874418001582679659987899826869258707_i128));
Goto(bb14)
}
bb14 = {
_12 = _11.1 as isize;
_22 = [_1,_1,_1,_1,_1];
_11.2 = [_11.1];
_9.2 = 137256866705398964215032434742521039688_i128;
_15.0 = !144496368053803603_u64;
_13 = !3944022242_u32;
_11.0 = '\u{e2638}';
_3 = (_6, _4);
_13 = 2808995042_u32;
_13 = 3433831330_u32 | 560056781_u32;
_9 = (89_i8, _5, (-38520491882412674240907086388221707775_i128));
_18 = _20 as i32;
_26 = core::ptr::addr_of_mut!(_9.0);
_23 = _16 as isize;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(14_usize, 23_usize, Move(_23), 8_usize, Move(_8), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(14_usize, 5_usize, Move(_5), 12_usize, Move(_12), 9_usize, Move(_9), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(14_usize, 16_usize, Move(_16), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i8,mut _2: f64,mut _3: (i8, [bool; 1], i128),mut _4: i128,mut _5: isize,mut _6: [usize; 5],mut _7: i8,mut _8: [i32; 1],mut _9: f32,mut _10: i8) -> *mut f64 {
mir! {
type RET = *mut f64;
let _11: Adt47;
let _12: bool;
let _13: u16;
let _14: [i8; 8];
let _15: isize;
let _16: i16;
let _17: Adt60;
let _18: isize;
let _19: *const u8;
let _20: Adt51;
let _21: [i8; 6];
let _22: isize;
let _23: isize;
let _24: i8;
let _25: [u32; 5];
let _26: isize;
let _27: char;
let _28: char;
let _29: Adt49;
let _30: [i128; 6];
let _31: f32;
let _32: Adt54;
let _33: ([i8; 6],);
let _34: char;
let _35: ([i8; 6],);
let _36: [u32; 5];
let _37: [i64; 5];
let _38: u64;
let _39: isize;
let _40: ([i32; 1],);
let _41: ();
let _42: ();
{
_3.1 = [false];
RET = core::ptr::addr_of_mut!(_2);
RET = core::ptr::addr_of_mut!(_2);
_5 = (-9223372036854775808_isize);
_5 = !(-9223372036854775808_isize);
_10 = _1 << _3.0;
_7 = _10;
_8 = [655315056_i32];
(*RET) = _5 as f64;
_7 = _3.0 ^ _10;
_7 = -_3.0;
RET = core::ptr::addr_of_mut!(_2);
_4 = _3.2 - _3.2;
_6 = [6_usize,2_usize,1_usize,10754013629109144139_usize,6291744756859721525_usize];
_1 = _7;
_3.1 = [false];
_11 = Adt47::Variant0 { fld0: 83_u8,fld1: 7_usize,fld2: 56484_u16,fld3: _3.2 };
(*RET) = 3769178611418242672_u64 as f64;
RET = core::ptr::addr_of_mut!((*RET));
place!(Field::<u8>(Variant(_11, 0), 0)) = !106_u8;
place!(Field::<u8>(Variant(_11, 0), 0)) = _3.0 as u8;
_12 = true;
place!(Field::<i128>(Variant(_11, 0), 3)) = _4;
place!(Field::<i128>(Variant(_11, 0), 3)) = _4;
_11 = Adt47::Variant0 { fld0: 159_u8,fld1: 4_usize,fld2: 5645_u16,fld3: _4 };
(*RET) = 97_u8 as f64;
Goto(bb1)
}
bb1 = {
_3.2 = Field::<i128>(Variant(_11, 0), 3);
place!(Field::<u8>(Variant(_11, 0), 0)) = 1535230659_u32 as u8;
_8 = [2085393871_i32];
_8 = [709932015_i32];
_1 = (*RET) as i8;
_6 = [16968041838999542711_usize,3_usize,7_usize,2_usize,11553632996846608203_usize];
_5 = !(-9223372036854775808_isize);
place!(Field::<usize>(Variant(_11, 0), 1)) = !6_usize;
_1 = _10 + _3.0;
_1 = _3.2 as i8;
_11 = Adt47::Variant0 { fld0: 184_u8,fld1: 3_usize,fld2: 14415_u16,fld3: _4 };
_15 = _5 | _5;
(*RET) = _15 as f64;
(*RET) = 180755974561857758493819140573778512761_u128 as f64;
place!(Field::<u8>(Variant(_11, 0), 0)) = 128_u8;
_2 = 70765750_i32 as f64;
place!(Field::<i128>(Variant(_11, 0), 3)) = _5 as i128;
place!(Field::<usize>(Variant(_11, 0), 1)) = _2 as usize;
_4 = _3.2;
_13 = 1572191111_i32 as u16;
place!(Field::<u8>(Variant(_11, 0), 0)) = !133_u8;
(*RET) = _15 as f64;
_8 = [(-436013395_i32)];
Call(RET = fn16(_10, _3.2, _3.2, _15, _10, _6, _3.0, _12, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18 = _15;
_18 = -_15;
_6 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_3.2 = 336238499082767023190921483701899420949_u128 as i128;
RET = core::ptr::addr_of_mut!(_2);
_22 = _9 as isize;
_14 = [_10,_1,_10,_3.0,_10,_7,_10,_3.0];
_22 = -_15;
_21 = [_1,_7,_3.0,_3.0,_3.0,_10];
place!(Field::<usize>(Variant(_11, 0), 1)) = !1_usize;
_2 = 336772951975629856950472414246209133851_u128 as f64;
place!(Field::<i128>(Variant(_11, 0), 3)) = _3.2 ^ _4;
_8 = [(-1643355885_i32)];
Goto(bb3)
}
bb3 = {
_15 = _22 ^ _18;
_5 = _18;
_16 = -(-8569_i16);
_11 = Adt47::Variant0 { fld0: 37_u8,fld1: 7347247643164920513_usize,fld2: _13,fld3: _3.2 };
place!(Field::<usize>(Variant(_11, 0), 1)) = 7_usize - 5518071031436996399_usize;
place!(Field::<usize>(Variant(_11, 0), 1)) = 14600168783779389208_usize;
place!(Field::<u8>(Variant(_11, 0), 0)) = Field::<u16>(Variant(_11, 0), 2) as u8;
_21 = [_10,_1,_7,_10,_1,_10];
place!(Field::<i128>(Variant(_11, 0), 3)) = !_3.2;
Goto(bb4)
}
bb4 = {
_25 = [1863833992_u32,14190666_u32,228561316_u32,3047348493_u32,1928841772_u32];
_13 = Field::<u16>(Variant(_11, 0), 2);
_9 = 14298275523665740075_u64 as f32;
_12 = !false;
_24 = _10;
Call((*RET) = core::intrinsics::transmute(_5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<u16>(Variant(_11, 0), 2)) = _13;
_2 = 2876642229_u32 as f64;
RET = core::ptr::addr_of_mut!((*RET));
match Field::<usize>(Variant(_11, 0), 1) {
0 => bb2,
1 => bb6,
14600168783779389208 => bb8,
_ => bb7
}
}
bb6 = {
_25 = [1863833992_u32,14190666_u32,228561316_u32,3047348493_u32,1928841772_u32];
_13 = Field::<u16>(Variant(_11, 0), 2);
_9 = 14298275523665740075_u64 as f32;
_12 = !false;
_24 = _10;
Call((*RET) = core::intrinsics::transmute(_5), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_18 = _15;
_18 = -_15;
_6 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_3.2 = 336238499082767023190921483701899420949_u128 as i128;
RET = core::ptr::addr_of_mut!(_2);
_22 = _9 as isize;
_14 = [_10,_1,_10,_3.0,_10,_7,_10,_3.0];
_22 = -_15;
_21 = [_1,_7,_3.0,_3.0,_3.0,_10];
place!(Field::<usize>(Variant(_11, 0), 1)) = !1_usize;
_2 = 336772951975629856950472414246209133851_u128 as f64;
place!(Field::<i128>(Variant(_11, 0), 3)) = _3.2 ^ _4;
_8 = [(-1643355885_i32)];
Goto(bb3)
}
bb8 = {
_25 = [4252931143_u32,1414942101_u32,3880964401_u32,1211704993_u32,2959769462_u32];
_21 = [_7,_7,_7,_1,_3.0,_1];
_10 = _24;
_3.0 = _7;
_15 = _9 as isize;
_10 = _1 - _3.0;
match Field::<usize>(Variant(_11, 0), 1) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
14600168783779389208 => bb9,
_ => bb5
}
}
bb9 = {
(*RET) = 6511549209356802102_i64 as f64;
_19 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_11, 0), 0)));
_26 = _18 << _10;
_3.2 = _9 as i128;
_24 = _7;
place!(Field::<i128>(Variant(_11, 0), 3)) = -_4;
RET = core::ptr::addr_of_mut!(_2);
RET = core::ptr::addr_of_mut!((*RET));
_29 = Adt49::Variant3 { fld0: (*_19) };
(*RET) = 5361115029050425036_u64 as f64;
_20 = Adt51::Variant0 { fld0: _10 };
(*_19) = !Field::<u8>(Variant(_29, 3), 0);
_19 = core::ptr::addr_of!((*_19));
_23 = Field::<u8>(Variant(_29, 3), 0) as isize;
_27 = '\u{4c6af}';
(*_19) = Field::<u8>(Variant(_29, 3), 0);
_9 = (-87202166_i32) as f32;
place!(Field::<usize>(Variant(_11, 0), 1)) = !17940660714767551996_usize;
SetDiscriminant(_11, 1);
place!(Field::<(f32, bool)>(Variant(_11, 1), 3)).1 = !_12;
_26 = _22 | _18;
Goto(bb10)
}
bb10 = {
_28 = _27;
_9 = (*RET) as f32;
_31 = _7 as f32;
_3.1 = [Field::<(f32, bool)>(Variant(_11, 1), 3).1];
_23 = _26 ^ _26;
_22 = 4041210522_u32 as isize;
_27 = _28;
_10 = 9242518431326533619_u64 as i8;
(*RET) = 7996925111408406926_u64 as f64;
_8 = [2095018275_i32];
_33.0 = [_24,_7,_24,_3.0,_3.0,_24];
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)).0 = [302359558_i32];
place!(Field::<(f32, bool)>(Variant(_11, 1), 3)) = (_31, _12);
RET = core::ptr::addr_of_mut!(_2);
_1 = !_7;
_30 = [_4,_4,_4,_4,_4,_4];
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)).0 = _8;
_22 = _15 << _23;
_3.0 = _31 as i8;
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)) = (_8,);
_13 = (*RET) as u16;
_19 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_29, 3), 0)));
_30 = [_4,_4,_4,_4,_4,_4];
(*RET) = 17345670589721241407_usize as f64;
Goto(bb11)
}
bb11 = {
_24 = Field::<i8>(Variant(_20, 0), 0) >> _23;
_35 = _33;
_14 = [Field::<i8>(Variant(_20, 0), 0),Field::<i8>(Variant(_20, 0), 0),Field::<i8>(Variant(_20, 0), 0),_24,_24,Field::<i8>(Variant(_20, 0), 0),Field::<i8>(Variant(_20, 0), 0),_24];
_9 = Field::<(f32, bool)>(Variant(_11, 1), 3).0 - _31;
place!(Field::<(f32, bool)>(Variant(_11, 1), 3)).1 = !_12;
_13 = 18951_u16;
_35 = _33;
_12 = !Field::<(f32, bool)>(Variant(_11, 1), 3).1;
_3.0 = _26 as i8;
_27 = _28;
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)) = (_8,);
_8 = [(-305276747_i32)];
Call(_23 = core::intrinsics::bswap(_22), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
SetDiscriminant(_20, 0);
(*RET) = _16 as f64;
_23 = _22 << _18;
place!(Field::<(f32, bool)>(Variant(_11, 1), 3)) = (_31, _12);
_36 = [3083944054_u32,593147646_u32,2235873163_u32,374481468_u32,3796647969_u32];
RET = core::ptr::addr_of_mut!((*RET));
_16 = 4289_i16 >> _24;
_26 = _23;
place!(Field::<u32>(Variant(_11, 1), 2)) = !1304350780_u32;
RET = core::ptr::addr_of_mut!((*RET));
match _13 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb13,
5 => bb14,
6 => bb15,
18951 => bb17,
_ => bb16
}
}
bb13 = {
_24 = Field::<i8>(Variant(_20, 0), 0) >> _23;
_35 = _33;
_14 = [Field::<i8>(Variant(_20, 0), 0),Field::<i8>(Variant(_20, 0), 0),Field::<i8>(Variant(_20, 0), 0),_24,_24,Field::<i8>(Variant(_20, 0), 0),Field::<i8>(Variant(_20, 0), 0),_24];
_9 = Field::<(f32, bool)>(Variant(_11, 1), 3).0 - _31;
place!(Field::<(f32, bool)>(Variant(_11, 1), 3)).1 = !_12;
_13 = 18951_u16;
_35 = _33;
_12 = !Field::<(f32, bool)>(Variant(_11, 1), 3).1;
_3.0 = _26 as i8;
_27 = _28;
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)) = (_8,);
_8 = [(-305276747_i32)];
Call(_23 = core::intrinsics::bswap(_22), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_28 = _27;
_9 = (*RET) as f32;
_31 = _7 as f32;
_3.1 = [Field::<(f32, bool)>(Variant(_11, 1), 3).1];
_23 = _26 ^ _26;
_22 = 4041210522_u32 as isize;
_27 = _28;
_10 = 9242518431326533619_u64 as i8;
(*RET) = 7996925111408406926_u64 as f64;
_8 = [2095018275_i32];
_33.0 = [_24,_7,_24,_3.0,_3.0,_24];
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)).0 = [302359558_i32];
place!(Field::<(f32, bool)>(Variant(_11, 1), 3)) = (_31, _12);
RET = core::ptr::addr_of_mut!(_2);
_1 = !_7;
_30 = [_4,_4,_4,_4,_4,_4];
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)).0 = _8;
_22 = _15 << _23;
_3.0 = _31 as i8;
place!(Field::<([i32; 1],)>(Variant(_11, 1), 1)) = (_8,);
_13 = (*RET) as u16;
_19 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_29, 3), 0)));
_30 = [_4,_4,_4,_4,_4,_4];
(*RET) = 17345670589721241407_usize as f64;
Goto(bb11)
}
bb15 = {
_25 = [1863833992_u32,14190666_u32,228561316_u32,3047348493_u32,1928841772_u32];
_13 = Field::<u16>(Variant(_11, 0), 2);
_9 = 14298275523665740075_u64 as f32;
_12 = !false;
_24 = _10;
Call((*RET) = core::intrinsics::transmute(_5), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_3.2 = Field::<i128>(Variant(_11, 0), 3);
place!(Field::<u8>(Variant(_11, 0), 0)) = 1535230659_u32 as u8;
_8 = [2085393871_i32];
_8 = [709932015_i32];
_1 = (*RET) as i8;
_6 = [16968041838999542711_usize,3_usize,7_usize,2_usize,11553632996846608203_usize];
_5 = !(-9223372036854775808_isize);
place!(Field::<usize>(Variant(_11, 0), 1)) = !6_usize;
_1 = _10 + _3.0;
_1 = _3.2 as i8;
_11 = Adt47::Variant0 { fld0: 184_u8,fld1: 3_usize,fld2: 14415_u16,fld3: _4 };
_15 = _5 | _5;
(*RET) = _15 as f64;
(*RET) = 180755974561857758493819140573778512761_u128 as f64;
place!(Field::<u8>(Variant(_11, 0), 0)) = 128_u8;
_2 = 70765750_i32 as f64;
place!(Field::<i128>(Variant(_11, 0), 3)) = _5 as i128;
place!(Field::<usize>(Variant(_11, 0), 1)) = _2 as usize;
_4 = _3.2;
_13 = 1572191111_i32 as u16;
place!(Field::<u8>(Variant(_11, 0), 0)) = !133_u8;
(*RET) = _15 as f64;
_8 = [(-436013395_i32)];
Call(RET = fn16(_10, _3.2, _3.2, _15, _10, _6, _3.0, _12, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_24 = 341054371_i32 as i8;
_39 = -_26;
_4 = _3.2;
_14 = [_3.0,_7,_7,_1,_1,_7,_1,_1];
_35.0 = [_1,_3.0,_7,_3.0,_7,_3.0];
_12 = !Field::<(f32, bool)>(Variant(_11, 1), 3).1;
_12 = Field::<(f32, bool)>(Variant(_11, 1), 3).1;
_34 = _28;
_13 = !20750_u16;
place!(Field::<i8>(Variant(_20, 0), 0)) = (-1452866405_i32) as i8;
(*_19) = !226_u8;
place!(Field::<u8>(Variant(_29, 3), 0)) = !217_u8;
_3.0 = _1;
_25 = _36;
_6 = [7128210904541967293_usize,14391876867525425839_usize,9371867950478198640_usize,14353803850886742417_usize,4412516811312443048_usize];
(*RET) = Field::<u32>(Variant(_11, 1), 2) as f64;
_16 = 11077839638417530640102251384266175214_u128 as i16;
_19 = core::ptr::addr_of!((*_19));
RET = core::ptr::addr_of_mut!(_2);
_8 = [1479137617_i32];
_14 = [_3.0,_3.0,Field::<i8>(Variant(_20, 0), 0),_3.0,_1,_24,_24,_24];
_35.0 = [_1,_1,_3.0,Field::<i8>(Variant(_20, 0), 0),_7,_7];
_21 = [_3.0,_7,_7,_1,_10,_1];
_39 = Field::<(f32, bool)>(Variant(_11, 1), 3).1 as isize;
Goto(bb18)
}
bb18 = {
Call(_41 = dump_var(15_usize, 3_usize, Move(_3), 30_usize, Move(_30), 10_usize, Move(_10), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(15_usize, 12_usize, Move(_12), 7_usize, Move(_7), 24_usize, Move(_24), 4_usize, Move(_4)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_41 = dump_var(15_usize, 8_usize, Move(_8), 35_usize, Move(_35), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_41 = dump_var(15_usize, 23_usize, Move(_23), 14_usize, Move(_14), 42_usize, _42, 42_usize, _42), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i8,mut _2: i128,mut _3: i128,mut _4: isize,mut _5: i8,mut _6: [usize; 5],mut _7: i8,mut _8: bool,mut _9: u16) -> *mut f64 {
mir! {
type RET = *mut f64;
let _10: u64;
let _11: u32;
let _12: i8;
let _13: *mut f64;
let _14: char;
let _15: u128;
let _16: Adt55;
let _17: u128;
let _18: (f32, [bool; 1], [isize; 7]);
let _19: [i32; 1];
let _20: char;
let _21: isize;
let _22: Adt50;
let _23: [bool; 1];
let _24: usize;
let _25: *mut bool;
let _26: Adt62;
let _27: isize;
let _28: u64;
let _29: isize;
let _30: [i32; 1];
let _31: char;
let _32: char;
let _33: (f32, bool);
let _34: isize;
let _35: [isize; 7];
let _36: usize;
let _37: i128;
let _38: *const usize;
let _39: (f32, bool);
let _40: f64;
let _41: *const usize;
let _42: ();
let _43: ();
{
_3 = _2;
_3 = 0_usize as i128;
_9 = 18249_u16;
_3 = _2;
_9 = 37744_u16 - 34809_u16;
_3 = _2 * _2;
_7 = (-1061907774_i32) as i8;
_5 = -_1;
_8 = _5 >= _1;
_5 = _1;
_7 = _1;
_7 = _1 - _5;
_2 = _3;
_1 = _5 + _5;
_1 = -_7;
_1 = _7;
_1 = _5;
_2 = _3;
_3 = _2;
_4 = -(-9223372036854775808_isize);
_8 = !true;
_11 = 3986459036_u32;
_2 = _3;
_4 = (-94_isize) << _7;
Goto(bb1)
}
bb1 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb2 = {
_11 = 4_usize as u32;
_4 = 58_isize ^ 9223372036854775807_isize;
_4 = (-57_isize);
_3 = -_2;
_9 = 40860_u16 << _1;
_9 = (-1856271556_i32) as u16;
_3 = _2 + _2;
_5 = _1;
_12 = _5;
_9 = !31207_u16;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211399 => bb7,
_ => bb6
}
}
bb3 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
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
_11 = 87040552208712282819063690614421025055_u128 as u32;
_8 = true ^ true;
_7 = _12 ^ _1;
_9 = 17650_u16;
_7 = _12 + _5;
_5 = _7;
_16.fld2 = [_11,_11,_11,_11,_11];
_12 = -_1;
_8 = !false;
_11 = _4 as u32;
_6 = [665678020110291711_usize,0_usize,14289743282338969632_usize,1_usize,15822469412926657349_usize];
_14 = '\u{10d1e8}';
_16.fld0 = core::ptr::addr_of_mut!(_8);
_16.fld3 = [(-1032620351_i32)];
_6 = [4_usize,3_usize,4_usize,3_usize,2_usize];
_5 = -_7;
match _9 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
17650 => bb12,
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
_11 = 4_usize as u32;
_4 = 58_isize ^ 9223372036854775807_isize;
_4 = (-57_isize);
_3 = -_2;
_9 = 40860_u16 << _1;
_9 = (-1856271556_i32) as u16;
_3 = _2 + _2;
_5 = _1;
_12 = _5;
_9 = !31207_u16;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211399 => bb7,
_ => bb6
}
}
bb11 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb12 = {
_5 = _7 ^ _7;
_11 = 1221520215_u32;
_18.1 = [_8];
_7 = _5 * _5;
_9 = 54464_u16 * 53596_u16;
_18.2 = [_4,_4,_4,_4,_4,_4,_4];
_18.1 = [_8];
_19 = [370101196_i32];
_16.fld2 = [_11,_11,_11,_11,_11];
_6 = [12973800619498056009_usize,6_usize,0_usize,6_usize,7757239943434597351_usize];
_8 = !false;
_16.fld1 = [_7,_7,_7,_7,_7,_7,_5,_7];
_12 = _5 * _7;
_10 = 2949556846385504147_u64 >> _1;
_9 = _11 as u16;
_21 = -_4;
_18.1 = [_8];
_16.fld0 = core::ptr::addr_of_mut!(_8);
_5 = _7 | _7;
_5 = 48928060407031365619329303789860122111_u128 as i8;
_10 = _14 as u64;
_6 = [4_usize,7_usize,9552824230650374254_usize,7093654013751766363_usize,4_usize];
_20 = _14;
_18.0 = 5_u8 as f32;
_9 = 28230_u16 & 52288_u16;
_17 = _8 as u128;
_11 = 1412220311_u32 | 2291696193_u32;
_16.fld1 = [_7,_7,_7,_12,_7,_5,_12,_12];
match _4 {
0 => bb11,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
340282366920938463463374607431768211399 => bb18,
_ => bb17
}
}
bb13 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb14 = {
_11 = 4_usize as u32;
_4 = 58_isize ^ 9223372036854775807_isize;
_4 = (-57_isize);
_3 = -_2;
_9 = 40860_u16 << _1;
_9 = (-1856271556_i32) as u16;
_3 = _2 + _2;
_5 = _1;
_12 = _5;
_9 = !31207_u16;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211399 => bb7,
_ => bb6
}
}
bb15 = {
Return()
}
bb16 = {
_11 = 4_usize as u32;
_4 = 58_isize ^ 9223372036854775807_isize;
_4 = (-57_isize);
_3 = -_2;
_9 = 40860_u16 << _1;
_9 = (-1856271556_i32) as u16;
_3 = _2 + _2;
_5 = _1;
_12 = _5;
_9 = !31207_u16;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211399 => bb7,
_ => bb6
}
}
bb17 = {
_11 = 87040552208712282819063690614421025055_u128 as u32;
_8 = true ^ true;
_7 = _12 ^ _1;
_9 = 17650_u16;
_7 = _12 + _5;
_5 = _7;
_16.fld2 = [_11,_11,_11,_11,_11];
_12 = -_1;
_8 = !false;
_11 = _4 as u32;
_6 = [665678020110291711_usize,0_usize,14289743282338969632_usize,1_usize,15822469412926657349_usize];
_14 = '\u{10d1e8}';
_16.fld0 = core::ptr::addr_of_mut!(_8);
_16.fld3 = [(-1032620351_i32)];
_6 = [4_usize,3_usize,4_usize,3_usize,2_usize];
_5 = -_7;
match _9 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
17650 => bb12,
_ => bb11
}
}
bb18 = {
_17 = _9 as u128;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_1 = _14 as i8;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_6 = [4061132094033672830_usize,18421506339031499761_usize,6_usize,6_usize,5_usize];
_11 = !76878205_u32;
_11 = 4_usize as u32;
_6 = [17851620969595051945_usize,5_usize,2_usize,4_usize,8224573397509330810_usize];
_2 = _18.0 as i128;
_21 = !_4;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_20 = _14;
_21 = _17 as isize;
match _4 {
0 => bb5,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
340282366920938463463374607431768211399 => bb26,
_ => bb25
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
_11 = 4_usize as u32;
_4 = 58_isize ^ 9223372036854775807_isize;
_4 = (-57_isize);
_3 = -_2;
_9 = 40860_u16 << _1;
_9 = (-1856271556_i32) as u16;
_3 = _2 + _2;
_5 = _1;
_12 = _5;
_9 = !31207_u16;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211399 => bb7,
_ => bb6
}
}
bb23 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb24 = {
_5 = _7 ^ _7;
_11 = 1221520215_u32;
_18.1 = [_8];
_7 = _5 * _5;
_9 = 54464_u16 * 53596_u16;
_18.2 = [_4,_4,_4,_4,_4,_4,_4];
_18.1 = [_8];
_19 = [370101196_i32];
_16.fld2 = [_11,_11,_11,_11,_11];
_6 = [12973800619498056009_usize,6_usize,0_usize,6_usize,7757239943434597351_usize];
_8 = !false;
_16.fld1 = [_7,_7,_7,_7,_7,_7,_5,_7];
_12 = _5 * _7;
_10 = 2949556846385504147_u64 >> _1;
_9 = _11 as u16;
_21 = -_4;
_18.1 = [_8];
_16.fld0 = core::ptr::addr_of_mut!(_8);
_5 = _7 | _7;
_5 = 48928060407031365619329303789860122111_u128 as i8;
_10 = _14 as u64;
_6 = [4_usize,7_usize,9552824230650374254_usize,7093654013751766363_usize,4_usize];
_20 = _14;
_18.0 = 5_u8 as f32;
_9 = 28230_u16 & 52288_u16;
_17 = _8 as u128;
_11 = 1412220311_u32 | 2291696193_u32;
_16.fld1 = [_7,_7,_7,_12,_7,_5,_12,_12];
match _4 {
0 => bb11,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
340282366920938463463374607431768211399 => bb18,
_ => bb17
}
}
bb25 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb26 = {
_23 = [_8];
_15 = !_17;
_10 = 17234217617628374462_u64;
_1 = _7 >> _7;
_23 = [_8];
_1 = _12;
_19 = [(-19065718_i32)];
_26.fld1.0 = _18.0 - _18.0;
_26.fld1 = (_18.0, _8);
_16.fld0 = core::ptr::addr_of_mut!(_8);
_12 = _11 as i8;
_21 = _4;
_11 = _26.fld1.1 as u32;
_2 = _11 as i128;
_27 = _4 | _4;
_21 = _27 >> _7;
_25 = core::ptr::addr_of_mut!(_26.fld1.1);
_12 = _7 << _7;
_16.fld1 = [_1,_1,_1,_12,_7,_12,_12,_1];
_29 = _21 & _21;
_26.fld1.0 = _18.0 - _18.0;
match _4 {
0 => bb11,
1 => bb27,
2 => bb28,
340282366920938463463374607431768211399 => bb30,
_ => bb29
}
}
bb27 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb28 = {
_2 = _3 - _3;
_6 = [4_usize,4_usize,13979556183582303752_usize,3_usize,2365666986994878462_usize];
_2 = 234669886423206114128630288101362887577_u128 as i128;
Goto(bb2)
}
bb29 = {
_17 = _9 as u128;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_1 = _14 as i8;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_6 = [4061132094033672830_usize,18421506339031499761_usize,6_usize,6_usize,5_usize];
_11 = !76878205_u32;
_11 = 4_usize as u32;
_6 = [17851620969595051945_usize,5_usize,2_usize,4_usize,8224573397509330810_usize];
_2 = _18.0 as i128;
_21 = !_4;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_20 = _14;
_21 = _17 as isize;
match _4 {
0 => bb5,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
340282366920938463463374607431768211399 => bb26,
_ => bb25
}
}
bb30 = {
_8 = _1 != _12;
_20 = _14;
_9 = 61064_u16;
_24 = _20 as usize;
_9 = _15 as u16;
(*_25) = _8 == _8;
_18.0 = -_26.fld1.0;
_5 = _8 as i8;
_11 = _9 as u32;
_28 = _10;
_5 = _12 - _1;
_18.1 = [(*_25)];
_34 = _29;
_21 = (-19859_i16) as isize;
_16.fld2 = [_11,_11,_11,_11,_11];
_18.0 = _26.fld1.0 + _26.fld1.0;
_18.0 = _3 as f32;
_16.fld1 = [_7,_5,_5,_1,_12,_1,_12,_12];
_1 = _5 - _5;
Call(_34 = core::intrinsics::bswap(_29), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_18.1 = _23;
_29 = _34 | _34;
_35 = _18.2;
_33.0 = -_18.0;
_2 = _3 ^ _3;
_23 = [_26.fld1.1];
_18.2 = [_29,_29,_29,_29,_29,_34,_34];
_33 = (_26.fld1.0, (*_25));
_30 = [621765095_i32];
_32 = _20;
_31 = _14;
_12 = _33.0 as i8;
_36 = _24 << _5;
_26.fld1 = _33;
_21 = _29;
_33.1 = _21 > _21;
_2 = _3;
_8 = _26.fld1.1;
_38 = core::ptr::addr_of!(_36);
_36 = _24;
_21 = _34;
(*_38) = !_24;
_16.fld0 = core::ptr::addr_of_mut!(_8);
_26.fld1 = (_18.0, _33.1);
_18.2 = [_29,_34,_34,_21,_21,_21,_21];
_6 = [(*_38),(*_38),_36,_24,(*_38)];
_4 = -_34;
_7 = _1;
Goto(bb32)
}
bb32 = {
_34 = !_4;
_37 = 14510_i16 as i128;
_14 = _31;
_36 = !_24;
_25 = _16.fld0;
_18.0 = -_33.0;
_12 = !_7;
_27 = !_29;
_40 = _28 as f64;
_15 = _17;
_26.fld1.1 = !_8;
_39.0 = _33.0;
_12 = _5;
RET = core::ptr::addr_of_mut!(_40);
_25 = _16.fld0;
_37 = _3 * _3;
_39 = _26.fld1;
_24 = (*RET) as usize;
RET = core::ptr::addr_of_mut!(_40);
_25 = _16.fld0;
_8 = _33.1;
_33.0 = _39.0 - _26.fld1.0;
(*_25) = !_39.1;
_40 = (-976141059212088486_i64) as f64;
_26.fld0 = _19;
_15 = _37 as u128;
_30 = [1762582418_i32];
_17 = !_15;
_4 = -_21;
_2 = 205_u8 as i128;
Goto(bb33)
}
bb33 = {
Call(_42 = dump_var(16_usize, 14_usize, Move(_14), 29_usize, Move(_29), 31_usize, Move(_31), 28_usize, Move(_28)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_42 = dump_var(16_usize, 12_usize, Move(_12), 7_usize, Move(_7), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_42 = dump_var(16_usize, 34_usize, Move(_34), 37_usize, Move(_37), 36_usize, Move(_36), 8_usize, Move(_8)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_42 = dump_var(16_usize, 1_usize, Move(_1), 30_usize, Move(_30), 4_usize, Move(_4), 43_usize, _43), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u64,mut _2: f32,mut _3: [i32; 1]) -> u128 {
mir! {
type RET = u128;
let _4: f64;
let _5: usize;
let _6: *const *mut bool;
let _7: char;
let _8: i32;
let _9: i32;
let _10: Adt62;
let _11: [i128; 6];
let _12: [i8; 8];
let _13: u64;
let _14: isize;
let _15: i128;
let _16: f32;
let _17: [bool; 1];
let _18: Adt63;
let _19: bool;
let _20: bool;
let _21: *mut [i8; 6];
let _22: i16;
let _23: (char, i32, [i32; 1]);
let _24: &'static i8;
let _25: *mut *mut f64;
let _26: Adt49;
let _27: [i32; 1];
let _28: char;
let _29: ();
let _30: ();
{
RET = 5_u8 as u128;
_2 = _1 as f32;
_1 = 3446333478577435491_i64 as u64;
RET = !58597299786223484326920610727850855952_u128;
RET = !36242103649912520975301549148609835400_u128;
RET = !323920749567623604606493404817374628289_u128;
_4 = (-5537854456328087560_i64) as f64;
_4 = 7_usize as f64;
_3 = [(-1911465503_i32)];
RET = !46833063214696890656393555009357403852_u128;
_1 = !12062673629655775279_u64;
_3 = [(-858823007_i32)];
Call(RET = core::intrinsics::bswap(302312630763902936512791172663857090930_u128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-6606_i16) as f64;
_4 = 5011521859099913387_i64 as f64;
Goto(bb2)
}
bb2 = {
_2 = 6017953259984303261_usize as f32;
Goto(bb3)
}
bb3 = {
_1 = 15537720348147710320_u64 - 6452620112840953108_u64;
RET = 266982733366863324858633478069562743667_u128 >> _1;
_2 = 1796299671454353312_i64 as f32;
_7 = '\u{70061}';
_4 = (-54_i8) as f64;
RET = 173977541277084880350034244786055929047_u128 | 181673753205309348532066596726206159436_u128;
_8 = 1083546267_i32;
_5 = !7_usize;
_5 = 52113_u16 as usize;
_2 = 11_i8 as f32;
_4 = 6444_i16 as f64;
_4 = _5 as f64;
RET = false as u128;
_3 = [_8];
_7 = '\u{13b60}';
_5 = 6168766552335113097_usize >> _1;
RET = (-101_i8) as u128;
_2 = _4 as f32;
RET = 3944747729220518593711974433442452354_u128;
_7 = '\u{26e4c}';
_9 = _8 + _8;
_5 = 7637779191174062764_usize;
RET = _5 as u128;
RET = !7721546084799226879244038067115054268_u128;
_1 = !4870925165713012911_u64;
_2 = 222_u8 as f32;
_2 = 21_isize as f32;
RET = 101719708744639088059622311724856368960_u128 >> _9;
Goto(bb4)
}
bb4 = {
_1 = 2541140645280914278_u64 & 14976568645589059219_u64;
RET = !93626603428760861107202273366432624046_u128;
RET = 55_i8 as u128;
RET = !233912124855248626627735833957578608898_u128;
_10.fld1.0 = (-46_i8) as f32;
_10.fld1.0 = _9 as f32;
_10.fld1 = (_2, true);
_5 = 3_usize + 0_usize;
_10.fld1.1 = true;
_4 = _9 as f64;
_10.fld0 = [_8];
RET = 297015480900982033480566180345435735482_u128 << _1;
_9 = _1 as i32;
_11 = [(-56278163574141673789318989045698391199_i128),(-7672563687507168453533592213007905350_i128),(-32314306988883219959150661811794197344_i128),(-152451963830802595127578133255816159459_i128),(-122154676514399488129262718787864286529_i128),160126142625122169535097904035700873314_i128];
_8 = _9;
Goto(bb5)
}
bb5 = {
_3 = _10.fld0;
_10.fld0 = [_9];
_8 = _10.fld1.0 as i32;
_1 = !16118841949589186876_u64;
_10.fld0 = [_9];
_8 = _9;
_13 = _1 + _1;
RET = 3016611307731344459877629747055117162_u128;
_11 = [(-33228551587450533738274794953703008221_i128),(-67407676762185468849861998524568913413_i128),(-86779541714118889549373128290641457084_i128),(-161508795269307359179438619299983301455_i128),(-77389412849716898115021560160514235134_i128),(-35230344433476751798129619172818323360_i128)];
_10.fld1 = (_2, true);
_8 = (-11260_i16) as i32;
_13 = !_1;
RET = !284007098814373306281671027077070223386_u128;
_5 = !4_usize;
_10.fld1 = (_2, true);
_12 = [55_i8,(-43_i8),25_i8,73_i8,115_i8,(-18_i8),56_i8,(-108_i8)];
_9 = -_8;
_5 = _7 as usize;
_4 = (-89_i8) as f64;
_14 = (-9223372036854775808_isize);
_9 = _8;
_10.fld0 = [_8];
Call(_10.fld1 = fn18(_11, _12, _11, _11, _12, _11, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_9 = _8 << _1;
_15 = 270692558085621877915100203283452396_i128 | (-106064669606482198675606266789026950479_i128);
_10.fld0 = [_9];
_10.fld0 = [_9];
_10.fld1.1 = !true;
_5 = 11910563186396622589_usize;
_1 = _4 as u64;
_11 = [_15,_15,_15,_15,_15,_15];
_13 = _1;
_11 = [_15,_15,_15,_15,_15,_15];
_17 = [_10.fld1.1];
_12 = [(-65_i8),(-94_i8),(-126_i8),70_i8,(-77_i8),118_i8,(-67_i8),50_i8];
_10.fld0 = [_9];
_16 = -_2;
_10.fld0 = _3;
_10.fld1.0 = (-2696_i16) as f32;
Goto(bb7)
}
bb7 = {
_10.fld0 = [_8];
_12 = [41_i8,(-8_i8),(-19_i8),49_i8,66_i8,(-88_i8),(-123_i8),56_i8];
_7 = '\u{d3de1}';
_19 = !_10.fld1.1;
_15 = -153812047597568193517945621092842952555_i128;
_11 = [_15,_15,_15,_15,_15,_15];
_11 = [_15,_15,_15,_15,_15,_15];
_10.fld1.1 = _14 < _14;
_13 = _1 - _1;
_10.fld0 = [_9];
_17 = [_10.fld1.1];
_4 = (-6428598484537566113_i64) as f64;
_2 = -_10.fld1.0;
RET = 146133817574600340415709877094077368537_u128;
_13 = _1;
_5 = !10130218161938704673_usize;
_22 = (-25506_i16) - 3603_i16;
_20 = _10.fld1.1;
RET = 96325663989021196433375757701211473080_u128 << _1;
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb8 = {
_1 = 15537720348147710320_u64 - 6452620112840953108_u64;
RET = 266982733366863324858633478069562743667_u128 >> _1;
_2 = 1796299671454353312_i64 as f32;
_7 = '\u{70061}';
_4 = (-54_i8) as f64;
RET = 173977541277084880350034244786055929047_u128 | 181673753205309348532066596726206159436_u128;
_8 = 1083546267_i32;
_5 = !7_usize;
_5 = 52113_u16 as usize;
_2 = 11_i8 as f32;
_4 = 6444_i16 as f64;
_4 = _5 as f64;
RET = false as u128;
_3 = [_8];
_7 = '\u{13b60}';
_5 = 6168766552335113097_usize >> _1;
RET = (-101_i8) as u128;
_2 = _4 as f32;
RET = 3944747729220518593711974433442452354_u128;
_7 = '\u{26e4c}';
_9 = _8 + _8;
_5 = 7637779191174062764_usize;
RET = _5 as u128;
RET = !7721546084799226879244038067115054268_u128;
_1 = !4870925165713012911_u64;
_2 = 222_u8 as f32;
_2 = 21_isize as f32;
RET = 101719708744639088059622311724856368960_u128 >> _9;
Goto(bb4)
}
bb9 = {
_3 = _10.fld0;
_10.fld0 = [_9];
_8 = _10.fld1.0 as i32;
_1 = !16118841949589186876_u64;
_10.fld0 = [_9];
_8 = _9;
_13 = _1 + _1;
RET = 3016611307731344459877629747055117162_u128;
_11 = [(-33228551587450533738274794953703008221_i128),(-67407676762185468849861998524568913413_i128),(-86779541714118889549373128290641457084_i128),(-161508795269307359179438619299983301455_i128),(-77389412849716898115021560160514235134_i128),(-35230344433476751798129619172818323360_i128)];
_10.fld1 = (_2, true);
_8 = (-11260_i16) as i32;
_13 = !_1;
RET = !284007098814373306281671027077070223386_u128;
_5 = !4_usize;
_10.fld1 = (_2, true);
_12 = [55_i8,(-43_i8),25_i8,73_i8,115_i8,(-18_i8),56_i8,(-108_i8)];
_9 = -_8;
_5 = _7 as usize;
_4 = (-89_i8) as f64;
_14 = (-9223372036854775808_isize);
_9 = _8;
_10.fld0 = [_8];
Call(_10.fld1 = fn18(_11, _12, _11, _11, _12, _11, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
Goto(bb11)
}
bb11 = {
_17 = [_10.fld1.1];
RET = 25099_u16 as u128;
_9 = !_8;
_19 = _10.fld1.1 | _20;
_8 = _13 as i32;
_27 = [_8];
_22 = _7 as i16;
_8 = !_9;
_10.fld1 = (_16, _20);
_20 = _10.fld1.1;
_23.2 = _3;
_2 = _16;
_23 = (_7, _9, _10.fld0);
_3 = [_23.1];
match _14 {
0 => bb9,
1 => bb2,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb12 = {
Goto(bb11)
}
bb13 = {
_3 = _10.fld0;
_10.fld0 = [_9];
_8 = _10.fld1.0 as i32;
_1 = !16118841949589186876_u64;
_10.fld0 = [_9];
_8 = _9;
_13 = _1 + _1;
RET = 3016611307731344459877629747055117162_u128;
_11 = [(-33228551587450533738274794953703008221_i128),(-67407676762185468849861998524568913413_i128),(-86779541714118889549373128290641457084_i128),(-161508795269307359179438619299983301455_i128),(-77389412849716898115021560160514235134_i128),(-35230344433476751798129619172818323360_i128)];
_10.fld1 = (_2, true);
_8 = (-11260_i16) as i32;
_13 = !_1;
RET = !284007098814373306281671027077070223386_u128;
_5 = !4_usize;
_10.fld1 = (_2, true);
_12 = [55_i8,(-43_i8),25_i8,73_i8,115_i8,(-18_i8),56_i8,(-108_i8)];
_9 = -_8;
_5 = _7 as usize;
_4 = (-89_i8) as f64;
_14 = (-9223372036854775808_isize);
_9 = _8;
_10.fld0 = [_8];
Call(_10.fld1 = fn18(_11, _12, _11, _11, _12, _11, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_2 = 6017953259984303261_usize as f32;
Goto(bb3)
}
bb15 = {
_1 = 15537720348147710320_u64 - 6452620112840953108_u64;
RET = 266982733366863324858633478069562743667_u128 >> _1;
_2 = 1796299671454353312_i64 as f32;
_7 = '\u{70061}';
_4 = (-54_i8) as f64;
RET = 173977541277084880350034244786055929047_u128 | 181673753205309348532066596726206159436_u128;
_8 = 1083546267_i32;
_5 = !7_usize;
_5 = 52113_u16 as usize;
_2 = 11_i8 as f32;
_4 = 6444_i16 as f64;
_4 = _5 as f64;
RET = false as u128;
_3 = [_8];
_7 = '\u{13b60}';
_5 = 6168766552335113097_usize >> _1;
RET = (-101_i8) as u128;
_2 = _4 as f32;
RET = 3944747729220518593711974433442452354_u128;
_7 = '\u{26e4c}';
_9 = _8 + _8;
_5 = 7637779191174062764_usize;
RET = _5 as u128;
RET = !7721546084799226879244038067115054268_u128;
_1 = !4870925165713012911_u64;
_2 = 222_u8 as f32;
_2 = 21_isize as f32;
RET = 101719708744639088059622311724856368960_u128 >> _9;
Goto(bb4)
}
bb16 = {
_9 = _8 << _1;
_15 = 270692558085621877915100203283452396_i128 | (-106064669606482198675606266789026950479_i128);
_10.fld0 = [_9];
_10.fld0 = [_9];
_10.fld1.1 = !true;
_5 = 11910563186396622589_usize;
_1 = _4 as u64;
_11 = [_15,_15,_15,_15,_15,_15];
_13 = _1;
_11 = [_15,_15,_15,_15,_15,_15];
_17 = [_10.fld1.1];
_12 = [(-65_i8),(-94_i8),(-126_i8),70_i8,(-77_i8),118_i8,(-67_i8),50_i8];
_10.fld0 = [_9];
_16 = -_2;
_10.fld0 = _3;
_10.fld1.0 = (-2696_i16) as f32;
Goto(bb7)
}
bb17 = {
_23.1 = (-77_i8) as i32;
_23.2 = [_23.1];
_10.fld1.1 = _19;
_14 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_27 = _23.2;
_13 = _1 & _1;
_20 = !_10.fld1.1;
_5 = 6779980953546182557_usize;
_23.2 = [_8];
_26 = Adt49::Variant3 { fld0: 218_u8 };
Goto(bb18)
}
bb18 = {
Call(_29 = dump_var(17_usize, 23_usize, Move(_23), 13_usize, Move(_13), 17_usize, Move(_17), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(17_usize, 8_usize, Move(_8), 11_usize, Move(_11), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [i128; 6],mut _2: [i8; 8],mut _3: [i128; 6],mut _4: [i128; 6],mut _5: [i8; 8],mut _6: [i128; 6],mut _7: char) -> (f32, bool) {
mir! {
type RET = (f32, bool);
let _8: f32;
let _9: *mut bool;
let _10: (u64,);
let _11: isize;
let _12: (char, i32, [i32; 1]);
let _13: [i8; 6];
let _14: Adt49;
let _15: i64;
let _16: isize;
let _17: (i8, [bool; 1], i128);
let _18: i16;
let _19: (f32, bool);
let _20: [i8; 8];
let _21: *mut i8;
let _22: [u32; 5];
let _23: isize;
let _24: bool;
let _25: u32;
let _26: isize;
let _27: Adt52;
let _28: f32;
let _29: char;
let _30: Adt53;
let _31: [u32; 5];
let _32: char;
let _33: *const usize;
let _34: isize;
let _35: Adt62;
let _36: *mut f64;
let _37: (f32, bool);
let _38: bool;
let _39: *mut *mut f64;
let _40: [i16; 4];
let _41: Adt54;
let _42: i128;
let _43: ();
let _44: ();
{
RET.1 = !false;
_5 = [32_i8,(-118_i8),116_i8,81_i8,65_i8,75_i8,(-4_i8),(-30_i8)];
_3 = [94223257456293553087352815736699958814_i128,(-83658561353541654465545662964240204655_i128),(-94181765754963731370241477497958161474_i128),(-118173749344373614464331361905491197518_i128),124098018064322565486179079553871347821_i128,46547179368814073146053327656546639402_i128];
_2 = [(-5_i8),(-97_i8),(-75_i8),79_i8,(-80_i8),(-56_i8),(-90_i8),(-64_i8)];
_8 = (-9168933226293776992_i64) as f32;
RET.0 = -_8;
RET.1 = RET.0 != RET.0;
RET.1 = RET.0 <= RET.0;
_2 = [(-95_i8),(-20_i8),(-31_i8),(-85_i8),18_i8,55_i8,(-22_i8),35_i8];
RET.1 = true ^ false;
_2 = _5;
_1 = [139339272468079911517804124344608632994_i128,168488731351782597693187292195626025523_i128,44583824389280487713806856209891881022_i128,(-77436676651500886932317011848861622240_i128),(-32550033172668623093434092139659798186_i128),(-18589255731410275158402945573346288338_i128)];
_11 = !9223372036854775807_isize;
RET = (_8, false);
RET = (_8, true);
_10 = (10389074094878529442_u64,);
_10.0 = 6122518771343477686_usize as u64;
_7 = '\u{ea292}';
Goto(bb1)
}
bb1 = {
_12.0 = _7;
RET.1 = false;
_2 = [71_i8,87_i8,34_i8,25_i8,(-29_i8),52_i8,(-98_i8),(-100_i8)];
_1 = _4;
_12.1 = 1196724211_u32 as i32;
_5 = [(-93_i8),(-73_i8),(-69_i8),(-111_i8),54_i8,(-23_i8),(-19_i8),123_i8];
_12.0 = _7;
_5 = [(-91_i8),(-23_i8),(-119_i8),(-57_i8),(-63_i8),86_i8,50_i8,(-116_i8)];
_10.0 = 15598800240967569889_u64;
_12.2 = [_12.1];
RET = (_8, false);
_7 = _12.0;
_8 = 0_usize as f32;
Goto(bb2)
}
bb2 = {
_12.1 = 1672753087_i32 << _11;
_17.2 = -(-93661013772403983432992658861399131141_i128);
_16 = RET.1 as isize;
_12.0 = _7;
_19.0 = 1782945168827332093_i64 as f32;
_4 = _6;
_9 = core::ptr::addr_of_mut!(RET.1);
_13 = [76_i8,46_i8,91_i8,77_i8,93_i8,99_i8];
_12.0 = _7;
match _10.0 {
0 => bb3,
15598800240967569889 => bb5,
_ => bb4
}
}
bb3 = {
_12.0 = _7;
RET.1 = false;
_2 = [71_i8,87_i8,34_i8,25_i8,(-29_i8),52_i8,(-98_i8),(-100_i8)];
_1 = _4;
_12.1 = 1196724211_u32 as i32;
_5 = [(-93_i8),(-73_i8),(-69_i8),(-111_i8),54_i8,(-23_i8),(-19_i8),123_i8];
_12.0 = _7;
_5 = [(-91_i8),(-23_i8),(-119_i8),(-57_i8),(-63_i8),86_i8,50_i8,(-116_i8)];
_10.0 = 15598800240967569889_u64;
_12.2 = [_12.1];
RET = (_8, false);
_7 = _12.0;
_8 = 0_usize as f32;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
RET.0 = -_19.0;
_23 = _12.0 as isize;
_23 = 1391497260_u32 as isize;
RET.0 = -_8;
_11 = _16 ^ _16;
(*_9) = !true;
RET = (_19.0, false);
_22 = [3096499943_u32,4127043939_u32,4131632045_u32,392631109_u32,3071182614_u32];
_2 = [123_i8,(-114_i8),84_i8,40_i8,(-115_i8),(-48_i8),(-84_i8),29_i8];
_18 = (*_9) as i16;
_15 = 2535159208240349631_i64 - 1949934727731281821_i64;
_13 = [(-62_i8),(-2_i8),119_i8,71_i8,(-30_i8),(-101_i8)];
_24 = (*_9);
_19.0 = -_8;
_21 = core::ptr::addr_of_mut!(_17.0);
(*_21) = 74_i8 | 97_i8;
_12.2 = [_12.1];
_7 = _12.0;
_24 = (*_9);
_21 = core::ptr::addr_of_mut!(_17.0);
_19 = RET;
_7 = _12.0;
match _10.0 {
0 => bb1,
1 => bb4,
2 => bb3,
15598800240967569889 => bb7,
_ => bb6
}
}
bb6 = {
_12.0 = _7;
RET.1 = false;
_2 = [71_i8,87_i8,34_i8,25_i8,(-29_i8),52_i8,(-98_i8),(-100_i8)];
_1 = _4;
_12.1 = 1196724211_u32 as i32;
_5 = [(-93_i8),(-73_i8),(-69_i8),(-111_i8),54_i8,(-23_i8),(-19_i8),123_i8];
_12.0 = _7;
_5 = [(-91_i8),(-23_i8),(-119_i8),(-57_i8),(-63_i8),86_i8,50_i8,(-116_i8)];
_10.0 = 15598800240967569889_u64;
_12.2 = [_12.1];
RET = (_8, false);
_7 = _12.0;
_8 = 0_usize as f32;
Goto(bb2)
}
bb7 = {
_22 = [1265444345_u32,3535411896_u32,3281607468_u32,3020654325_u32,3279766406_u32];
_18 = !(-3191_i16);
Call(_4 = fn19(_19.1, _1, _19.1, _8, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10.0 = !11903538910193804946_u64;
(*_21) = 100_i8;
RET = (_8, _19.1);
_13 = [(*_21),(*_21),(*_21),(*_21),(*_21),(*_21)];
_27.fld2 = [_15,_15,_15,_15,_15];
_20 = [(*_21),(*_21),_17.0,_17.0,(*_21),_17.0,(*_21),_17.0];
_15 = -8265186644123439463_i64;
_27.fld0 = _21;
_18 = (-2203_i16) & (-18512_i16);
_11 = _16 + _23;
_28 = _8;
_26 = 221_u8 as isize;
_2 = [(*_21),(*_21),_17.0,(*_21),_17.0,_17.0,_17.0,(*_21)];
RET.0 = _19.0;
_4 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
Goto(bb9)
}
bb9 = {
_21 = core::ptr::addr_of_mut!((*_21));
_12.0 = _7;
RET.0 = _19.0;
_26 = !_11;
_14 = Adt49::Variant3 { fld0: 208_u8 };
_7 = _12.0;
_27.fld2 = [_15,_15,_15,_15,_15];
_23 = _12.1 as isize;
_10.0 = 54912_u16 as u64;
_13 = [(*_21),(*_21),(*_21),(*_21),(*_21),_17.0];
_8 = 3720742954550882596_usize as f32;
_27.fld3 = [_24];
RET.0 = -_8;
_17.0 = (-4_i8);
_17.1 = _27.fld3;
_4 = _3;
_27.fld2 = [_15,_15,_15,_15,_15];
RET = (_8, _19.1);
_27.fld3 = [_19.1];
_32 = _7;
_31 = [726837095_u32,2615800494_u32,2528677839_u32,1894245135_u32,3050099_u32];
_2 = [(*_21),_17.0,(*_21),(*_21),(*_21),_17.0,_17.0,_17.0];
_19 = RET;
RET.1 = _24;
_31 = [3932466043_u32,2448821523_u32,3853098866_u32,1434744661_u32,1604517588_u32];
Goto(bb10)
}
bb10 = {
_17 = (52_i8, _27.fld3, (-105232899935503618887234824943485936466_i128));
_22 = [3879743345_u32,3804789008_u32,3203702015_u32,424092417_u32,1598901640_u32];
_27.fld1 = !6_usize;
_13 = [(*_21),(*_21),(*_21),_17.0,(*_21),_17.0];
place!(Field::<u8>(Variant(_14, 3), 0)) = 217_u8 - 109_u8;
(*_9) = _19.1 ^ _19.1;
_27.fld1 = _11 as usize;
_27.fld3 = [RET.1];
_4 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_4 = _1;
_27.fld3 = [(*_9)];
_19 = RET;
_31 = [3994024150_u32,3133377997_u32,2973105797_u32,3315486894_u32,343188497_u32];
_17.1 = [RET.1];
_30 = Adt53::Variant1 { fld0: 280394139228431131651304982046428006682_u128,fld1: _15 };
_5 = _2;
RET = (_8, _19.1);
_34 = _10.0 as isize;
match _17.2 {
0 => bb9,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
235049466985434844576139782488282274990 => bb12,
_ => bb11
}
}
bb11 = {
_22 = [1265444345_u32,3535411896_u32,3281607468_u32,3020654325_u32,3279766406_u32];
_18 = !(-3191_i16);
Call(_4 = fn19(_19.1, _1, _19.1, _8, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
_15 = !Field::<i64>(Variant(_30, 1), 1);
_13 = [(*_21),(*_21),(*_21),_17.0,(*_21),_17.0];
_24 = (*_9) & (*_9);
_25 = Field::<u8>(Variant(_14, 3), 0) as u32;
place!(Field::<u128>(Variant(_30, 1), 0)) = !189004907182608208063906752306625161191_u128;
_24 = !(*_9);
_23 = _34;
_35.fld1.1 = !_19.1;
_9 = core::ptr::addr_of_mut!(_37.1);
_17 = (112_i8, _27.fld3, 3837771890051813256401515939635677600_i128);
_21 = core::ptr::addr_of_mut!((*_21));
_22 = [_25,_25,_25,_25,_25];
_35.fld2 = core::ptr::addr_of_mut!(_13);
_37 = (RET.0, _35.fld1.1);
_10 = (4334154665458327962_u64,);
(*_21) = 70_i8 << Field::<u8>(Variant(_14, 3), 0);
_12.0 = _7;
match _17.2 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb11,
4 => bb6,
3837771890051813256401515939635677600 => bb14,
_ => bb13
}
}
bb13 = {
RET.0 = -_19.0;
_23 = _12.0 as isize;
_23 = 1391497260_u32 as isize;
RET.0 = -_8;
_11 = _16 ^ _16;
(*_9) = !true;
RET = (_19.0, false);
_22 = [3096499943_u32,4127043939_u32,4131632045_u32,392631109_u32,3071182614_u32];
_2 = [123_i8,(-114_i8),84_i8,40_i8,(-115_i8),(-48_i8),(-84_i8),29_i8];
_18 = (*_9) as i16;
_15 = 2535159208240349631_i64 - 1949934727731281821_i64;
_13 = [(-62_i8),(-2_i8),119_i8,71_i8,(-30_i8),(-101_i8)];
_24 = (*_9);
_19.0 = -_8;
_21 = core::ptr::addr_of_mut!(_17.0);
(*_21) = 74_i8 | 97_i8;
_12.2 = [_12.1];
_7 = _12.0;
_24 = (*_9);
_21 = core::ptr::addr_of_mut!(_17.0);
_19 = RET;
_7 = _12.0;
match _10.0 {
0 => bb1,
1 => bb4,
2 => bb3,
15598800240967569889 => bb7,
_ => bb6
}
}
bb14 = {
_12.2 = [_12.1];
_6 = _1;
(*_9) = _35.fld1.1;
_12.0 = _32;
_28 = -_19.0;
_37 = (_28, _35.fld1.1);
_35.fld1.0 = _27.fld1 as f32;
place!(Field::<u128>(Variant(_30, 1), 0)) = !49312804070546068577748602692185120331_u128;
place!(Field::<u128>(Variant(_30, 1), 0)) = 322545351177597993716083863733388046606_u128;
_17.0 = (-122_i8);
_19.0 = _12.1 as f32;
place!(Field::<i64>(Variant(_30, 1), 1)) = Field::<u8>(Variant(_14, 3), 0) as i64;
_27.fld2 = [_15,_15,Field::<i64>(Variant(_30, 1), 1),Field::<i64>(Variant(_30, 1), 1),Field::<i64>(Variant(_30, 1), 1)];
_29 = _32;
_20 = _5;
_37.0 = _28 * _19.0;
SetDiscriminant(_30, 1);
_33 = core::ptr::addr_of!(_27.fld1);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(18_usize, 34_usize, Move(_34), 23_usize, Move(_23), 6_usize, Move(_6), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(18_usize, 24_usize, Move(_24), 7_usize, Move(_7), 29_usize, Move(_29), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(18_usize, 15_usize, Move(_15), 10_usize, Move(_10), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: bool,mut _2: [i128; 6],mut _3: bool,mut _4: f32,mut _5: *mut bool) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _6: u128;
let _7: f32;
let _8: bool;
let _9: isize;
let _10: u8;
let _11: char;
let _12: Adt47;
let _13: *mut bool;
let _14: Adt49;
let _15: i32;
let _16: u8;
let _17: f32;
let _18: Adt63;
let _19: bool;
let _20: u64;
let _21: f64;
let _22: char;
let _23: (i8, [bool; 1], i128);
let _24: char;
let _25: Adt48;
let _26: u16;
let _27: [i32; 1];
let _28: i128;
let _29: Adt62;
let _30: Adt50;
let _31: [i16; 4];
let _32: u32;
let _33: ();
let _34: ();
{
_3 = _1;
RET = _2;
_5 = core::ptr::addr_of_mut!((*_5));
_1 = _3;
_6 = !326134807249042598314409940706197290737_u128;
(*_5) = _3;
(*_5) = !_3;
_3 = !(*_5);
(*_5) = !_1;
_1 = (*_5);
_2 = RET;
(*_5) = _3;
_7 = _4 * _4;
_3 = _1 ^ _1;
_2 = [118092142688162727787597589536011056252_i128,(-132158471323620755317592324091147500979_i128),(-27742850027416078095289548456852502473_i128),12290842705248645668463362077076978392_i128,149561748597378010852775148191599750369_i128,(-84725129351283696624441431852541516166_i128)];
_1 = !(*_5);
Goto(bb1)
}
bb1 = {
_2 = [115977038001672430235184444623947044652_i128,(-8422236153007441057087693130128185182_i128),133239924628435193136136104388704779812_i128,61311239713623345960686247284117571586_i128,(-504041486611118875653601957664899114_i128),23248636605621983344245271258629770425_i128];
_6 = (-5180876952167478632962824121502859226_i128) as u128;
_2 = [65872699680700059925812881808604284313_i128,77554811843211356025542737299465268276_i128,29164091715581360686592071702629995783_i128,3089136170598179342114369544215976505_i128,(-116726488054373868021174034274651446243_i128),(-21602983994955628023225342790660349555_i128)];
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = !_3;
_2 = RET;
_6 = 24526600005431604570543030653696174812_u128;
_5 = core::ptr::addr_of_mut!(_1);
_1 = _3;
_5 = core::ptr::addr_of_mut!(_3);
_8 = _3 != (*_5);
_7 = -_4;
(*_5) = !_8;
match _6 {
24526600005431604570543030653696174812 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_2 = RET;
_6 = !253119107866281166765422167039012218354_u128;
_5 = core::ptr::addr_of_mut!((*_5));
_10 = 230_u8 >> _6;
_7 = _4 - _4;
_2 = [74725128809017850918686268248356932728_i128,(-142270720346339791577300368060593752174_i128),(-60454132696455871384236735975851545499_i128),(-30725205653063881380309725617399194795_i128),96300198379658654661502839795155072713_i128,(-32779033268722763976619196586076568287_i128)];
_3 = _10 >= _10;
_6 = 1854746055_i32 as u128;
_8 = !_3;
RET = [(-32843870815858440368218591555827645713_i128),95943111851048240308453584232367083674_i128,(-109018378711488395267610355913880564472_i128),46118202775617977520276386278293132147_i128,61488737840129903856828980140479282106_i128,151804561856663176354129652045239561082_i128];
(*_5) = _8 > _1;
_8 = _3;
Goto(bb4)
}
bb4 = {
_9 = (-956999677265720832_i64) as isize;
Goto(bb5)
}
bb5 = {
_9 = 9223372036854775807_isize;
_3 = _8;
_11 = '\u{9a748}';
(*_5) = _8 <= _8;
_9 = (-9223372036854775808_isize);
_1 = _3 >= _8;
_4 = _9 as f32;
(*_5) = !_1;
_8 = (*_5);
_8 = _1 | (*_5);
(*_5) = _8 != _8;
_15 = 1491137059_i32;
_15 = (-365797647_i32) ^ 1610475611_i32;
_8 = (*_5);
_10 = 105_u8 & 7_u8;
_6 = 156097221043132741538654031313207510537_u128;
_2 = [(-131280788740893926460989442573996744581_i128),7561808917860905560722795405380308326_i128,109743737697486581550275508860212581481_i128,(-123323278650115879135530283141469118359_i128),(-50358664961502049116438263452874394971_i128),(-114648992488525040980684641980106423738_i128)];
_11 = '\u{89778}';
_7 = _4 * _4;
_12 = Adt47::Variant0 { fld0: _10,fld1: 12764577973841118456_usize,fld2: 6458_u16,fld3: (-20733221035584981475599079140000749440_i128) };
_4 = _7 * _7;
_16 = Field::<u8>(Variant(_12, 0), 0);
_15 = -(-1714258581_i32);
place!(Field::<i128>(Variant(_12, 0), 3)) = 20449420783664909070503028264107912494_i128;
_14 = Adt49::Variant3 { fld0: _16 };
_9 = -(-9223372036854775808_isize);
match _6 {
0 => bb2,
156097221043132741538654031313207510537 => bb7,
_ => bb6
}
}
bb6 = {
_2 = RET;
_6 = !253119107866281166765422167039012218354_u128;
_5 = core::ptr::addr_of_mut!((*_5));
_10 = 230_u8 >> _6;
_7 = _4 - _4;
_2 = [74725128809017850918686268248356932728_i128,(-142270720346339791577300368060593752174_i128),(-60454132696455871384236735975851545499_i128),(-30725205653063881380309725617399194795_i128),96300198379658654661502839795155072713_i128,(-32779033268722763976619196586076568287_i128)];
_3 = _10 >= _10;
_6 = 1854746055_i32 as u128;
_8 = !_3;
RET = [(-32843870815858440368218591555827645713_i128),95943111851048240308453584232367083674_i128,(-109018378711488395267610355913880564472_i128),46118202775617977520276386278293132147_i128,61488737840129903856828980140479282106_i128,151804561856663176354129652045239561082_i128];
(*_5) = _8 > _1;
_8 = _3;
Goto(bb4)
}
bb7 = {
_17 = -_4;
place!(Field::<u8>(Variant(_14, 3), 0)) = _10 + _10;
_11 = '\u{1a596}';
place!(Field::<u16>(Variant(_12, 0), 2)) = 1194930797942620567_i64 as u16;
_4 = _7 + _7;
_11 = '\u{71b5f}';
_13 = _5;
_10 = 4134994859_u32 as u8;
_7 = 1814599419_u32 as f32;
_3 = !_8;
_17 = _4;
RET = [Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3)];
_4 = (-420201520825338706_i64) as f32;
_2 = [Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3)];
_13 = _5;
_10 = !_16;
_17 = _7;
_10 = Field::<u8>(Variant(_12, 0), 0);
Goto(bb8)
}
bb8 = {
place!(Field::<u16>(Variant(_12, 0), 2)) = (-48_i8) as u16;
(*_13) = _8;
place!(Field::<u8>(Variant(_12, 0), 0)) = _10 * Field::<u8>(Variant(_14, 3), 0);
place!(Field::<i128>(Variant(_12, 0), 3)) = 113419278148136225013888195537375357132_i128;
(*_5) = _8;
_16 = !Field::<u8>(Variant(_12, 0), 0);
_20 = !10175246231399824973_u64;
_11 = '\u{cb47c}';
_19 = _8;
_9 = (-9223372036854775808_isize);
_2 = RET;
place!(Field::<u8>(Variant(_14, 3), 0)) = _16;
place!(Field::<i128>(Variant(_12, 0), 3)) = 130636455723647906647877824649052259683_i128;
_16 = !Field::<u8>(Variant(_12, 0), 0);
place!(Field::<usize>(Variant(_12, 0), 1)) = 1597903164848877498_usize;
(*_13) = _8;
match Field::<i128>(Variant(_12, 0), 3) {
130636455723647906647877824649052259683 => bb9,
_ => bb1
}
}
bb9 = {
_19 = _3 & _8;
place!(Field::<u8>(Variant(_14, 3), 0)) = !Field::<u8>(Variant(_12, 0), 0);
place!(Field::<u16>(Variant(_12, 0), 2)) = 4920_u16 & 42296_u16;
place!(Field::<u8>(Variant(_12, 0), 0)) = Field::<i128>(Variant(_12, 0), 3) as u8;
place!(Field::<usize>(Variant(_12, 0), 1)) = 1_usize ^ 12446028762560457911_usize;
_20 = 12721287081631558415_u64 | 7194015629194078320_u64;
_1 = (*_5);
(*_5) = _1;
_6 = Field::<i128>(Variant(_12, 0), 3) as u128;
_21 = (-1891445014150116147_i64) as f64;
_5 = core::ptr::addr_of_mut!(_3);
_20 = _4 as u64;
_20 = _8 as u64;
_21 = _9 as f64;
_14 = Adt49::Variant3 { fld0: _16 };
_11 = '\u{b00c4}';
_2 = RET;
_13 = core::ptr::addr_of_mut!(_8);
place!(Field::<u16>(Variant(_12, 0), 2)) = _11 as u16;
place!(Field::<i128>(Variant(_12, 0), 3)) = _20 as i128;
SetDiscriminant(_12, 2);
SetDiscriminant(_14, 0);
_6 = 143803054244736365576847711533646609783_u128 * 44909304824893519546799373196313679872_u128;
place!(Field::<[i8; 6]>(Variant(_14, 0), 1)) = [69_i8,(-53_i8),(-69_i8),(-11_i8),(-114_i8),(-5_i8)];
place!(Field::<*mut f64>(Variant(_12, 2), 5)) = core::ptr::addr_of_mut!(_21);
Goto(bb10)
}
bb10 = {
RET = _2;
place!(Field::<isize>(Variant(_14, 0), 2)) = _9 >> _20;
_8 = _20 >= _20;
_5 = core::ptr::addr_of_mut!(_19);
place!(Field::<[i8; 6]>(Variant(_14, 0), 1)) = [83_i8,64_i8,79_i8,46_i8,(-57_i8),43_i8];
place!(Field::<i64>(Variant(_12, 2), 6)) = (*_13) as i64;
_8 = _1;
_2 = [130322173059066174824423103772708292991_i128,106775801833369926708051575756248303344_i128,(-77879273485201293860790036642443653381_i128),(-39175093609020810457026194405797292594_i128),(-68367973033542821416152120886683177175_i128),122828788076158322658753921321708956689_i128];
place!(Field::<isize>(Variant(_14, 0), 2)) = _6 as isize;
_3 = !(*_5);
_11 = '\u{52f1d}';
place!(Field::<bool>(Variant(_12, 2), 0)) = !_8;
_23.1 = [_1];
_24 = _11;
place!(Field::<char>(Variant(_12, 2), 1)) = _11;
_2 = [(-132833588957497260613536432704404037589_i128),10403267256796274446217395464256179467_i128,27261929600042343266249778677375968791_i128,127240672927266939026970753420721254812_i128,124012413516290180411069129925786769408_i128,119354819774614426037118598939811411134_i128];
_23.0 = _20 as i8;
Goto(bb11)
}
bb11 = {
place!(Field::<u32>(Variant(_12, 2), 4)) = !1379666222_u32;
_23.2 = -(-7186351239134518283911326579528624292_i128);
(*_13) = !_19;
_12 = Adt47::Variant0 { fld0: _16,fld1: 5_usize,fld2: 32438_u16,fld3: _23.2 };
_19 = !(*_13);
_20 = !2812726456950261592_u64;
_16 = Field::<u8>(Variant(_12, 0), 0);
match _9 {
0 => bb5,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb12 = {
_2 = [115977038001672430235184444623947044652_i128,(-8422236153007441057087693130128185182_i128),133239924628435193136136104388704779812_i128,61311239713623345960686247284117571586_i128,(-504041486611118875653601957664899114_i128),23248636605621983344245271258629770425_i128];
_6 = (-5180876952167478632962824121502859226_i128) as u128;
_2 = [65872699680700059925812881808604284313_i128,77554811843211356025542737299465268276_i128,29164091715581360686592071702629995783_i128,3089136170598179342114369544215976505_i128,(-116726488054373868021174034274651446243_i128),(-21602983994955628023225342790660349555_i128)];
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = !_3;
_2 = RET;
_6 = 24526600005431604570543030653696174812_u128;
_5 = core::ptr::addr_of_mut!(_1);
_1 = _3;
_5 = core::ptr::addr_of_mut!(_3);
_8 = _3 != (*_5);
_7 = -_4;
(*_5) = !_8;
match _6 {
24526600005431604570543030653696174812 => bb3,
_ => bb2
}
}
bb13 = {
_24 = _11;
(*_5) = !_1;
place!(Field::<[i8; 6]>(Variant(_14, 0), 1)) = [_23.0,_23.0,_23.0,_23.0,_23.0,_23.0];
_26 = _6 as u16;
place!(Field::<usize>(Variant(_12, 0), 1)) = 1_usize << _23.0;
place!(Field::<usize>(Variant(_12, 0), 1)) = 4_usize;
_27 = [_15];
_17 = _4;
RET = [Field::<i128>(Variant(_12, 0), 3),_23.2,Field::<i128>(Variant(_12, 0), 3),_23.2,Field::<i128>(Variant(_12, 0), 3),Field::<i128>(Variant(_12, 0), 3)];
_29.fld1.0 = _4 * _7;
(*_13) = _23.0 > _23.0;
_7 = _9 as f32;
place!(Field::<isize>(Variant(_14, 0), 2)) = _26 as isize;
RET = _2;
Goto(bb14)
}
bb14 = {
_29.fld1.1 = _3 >= _3;
_9 = Field::<isize>(Variant(_14, 0), 2) - Field::<isize>(Variant(_14, 0), 2);
place!(Field::<(i8, [bool; 1], i128)>(Variant(_14, 0), 3)) = (_23.0, _23.1, _23.2);
_19 = _1;
_9 = Field::<isize>(Variant(_14, 0), 2);
place!(Field::<u16>(Variant(_12, 0), 2)) = _17 as u16;
place!(Field::<u16>(Variant(_12, 0), 2)) = _26 << Field::<(i8, [bool; 1], i128)>(Variant(_14, 0), 3).0;
place!(Field::<(i8, [bool; 1], i128)>(Variant(_14, 0), 3)).2 = (-15089_i16) as i128;
_4 = -_17;
_27 = [_15];
SetDiscriminant(_12, 0);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(19_usize, 27_usize, Move(_27), 8_usize, Move(_8), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(19_usize, 3_usize, Move(_3), 20_usize, Move(_20), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(3809961865_u32), std::hint::black_box(248726640599027110584342007692591391656_u128), std::hint::black_box(19138_u16), std::hint::black_box(112_i8), std::hint::black_box(3_usize), std::hint::black_box((-2089136002_i32)));
                
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u8,
fld1: usize,
fld2: u16,
fld3: i128,

},
Variant1{
fld0: usize,
fld1: ([i32; 1],),
fld2: u32,
fld3: (f32, bool),

},
Variant2{
fld0: bool,
fld1: char,
fld2: u16,
fld3: u128,
fld4: u32,
fld5: *mut f64,
fld6: i64,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const *mut bool,

},
Variant1{
fld0: [usize; 7],
fld1: [u128; 8],
fld2: [i8; 6],
fld3: [i8; 8],
fld4: [bool; 1],

},
Variant2{
fld0: u128,
fld1: i8,

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt47,
fld1: [i8; 6],
fld2: isize,
fld3: (i8, [bool; 1], i128),

},
Variant1{
fld0: u8,
fld1: [u128; 8],
fld2: usize,

},
Variant2{
fld0: Adt47,
fld1: [usize; 7],
fld2: *mut *mut f64,
fld3: u8,
fld4: i16,

},
Variant3{
fld0: u8,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f64,
fld1: *const usize,
fld2: [i8; 8],
fld3: Adt49,
fld4: ([i32; 1],),
fld5: i64,

},
Variant1{
fld0: i128,
fld1: [u32; 5],
fld2: [i16; 4],
fld3: [i128; 6],
fld4: i16,

},
Variant2{
fld0: i8,
fld1: *mut [i8; 6],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: i8,

},
Variant1{
fld0: *const usize,
fld1: *const u8,
fld2: [i128; 6],
fld3: (char, i32, [i32; 1]),
fld4: (f32, bool),
fld5: i128,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: *mut i8,
fld1: usize,
fld2: [i64; 5],
fld3: [bool; 1],
}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u128,
fld1: (u64,),

},
Variant1{
fld0: u128,
fld1: i64,

},
Variant2{
fld0: i32,
fld1: *mut f64,
fld2: u128,

}}
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: ([i8; 6],),
fld1: *const u8,
fld2: *mut [i8; 6],
fld3: Adt52,
fld4: [i8; 8],
fld5: [i16; 4],
fld6: Adt51,
fld7: i128,

},
Variant1{
fld0: f32,
fld1: [bool; 1],
fld2: (f32, bool),
fld3: (f32, [bool; 1], [isize; 7]),
fld4: (i8, [bool; 1], i128),
fld5: [i64; 5],
fld6: i64,
fld7: u8,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: *mut bool,
fld1: [i8; 8],
fld2: [u32; 5],
fld3: [i32; 1],
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *const *mut bool,
fld1: Adt47,
fld2: [i8; 6],
fld3: Adt54,
fld4: Adt52,

},
Variant1{
fld0: i16,
fld1: Adt51,

},
Variant2{
fld0: u64,

},
Variant3{
fld0: bool,
fld1: u64,
fld2: *const *mut bool,
fld3: i64,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (u64,),
fld1: [i64; 5],
fld2: ([i8; 6],),
fld3: Adt47,
fld4: [usize; 5],
fld5: u32,
fld6: Adt53,
fld7: *const usize,

},
Variant1{
fld0: [i32; 1],
fld1: f64,
fld2: u128,
fld3: i8,
fld4: [i128; 6],
fld5: f32,
fld6: *mut bool,

},
Variant2{
fld0: u32,
fld1: *mut i8,
fld2: [i128; 6],
fld3: (char, i32, [i32; 1]),
fld4: i32,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
fld0: (u64,),
fld1: Adt50,
fld2: isize,
fld3: i8,
fld4: [u128; 8],

},
Variant1{
fld0: *const u8,
fld1: u16,
fld2: (char, i32, [i32; 1]),
fld3: f32,
fld4: [i128; 6],
fld5: [usize; 7],
fld6: i64,

},
Variant2{
fld0: ([i8; 6],),
fld1: *const u8,
fld2: *mut [i8; 6],
fld3: [i128; 6],
fld4: (f32, bool),
fld5: *const *mut bool,
fld6: Adt50,
fld7: [i32; 1],

},
Variant3{
fld0: *mut bool,
fld1: [i32; 1],
fld2: *mut f64,
fld3: Adt54,
fld4: i16,
fld5: [usize; 7],
fld6: f64,
fld7: (f32, [bool; 1], [isize; 7]),

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *mut [i8; 6],
fld1: char,
fld2: [u32; 5],
fld3: i16,

},
Variant1{
fld0: i8,
fld1: [i64; 5],
fld2: ([i8; 6],),

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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: [isize; 7],
fld1: [i8; 6],
fld2: (i8, [bool; 1], i128),
fld3: Adt57,
fld4: u32,
fld5: (char, i32, [i32; 1]),
fld6: i64,
fld7: i128,

},
Variant1{
fld0: *mut i8,
fld1: (char, i32, [i32; 1]),

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt61{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt61 {
fld0: Adt59,
}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt62{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt62 {
fld0: [i32; 1],
fld1: (f32, bool),
fld2: *mut [i8; 6],
}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt63::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: Adt52,
fld1: ([i32; 1],),
fld2: u32,
fld3: i8,
fld4: f32,
fld5: *mut bool,
fld6: i64,

},
Variant1{
fld0: f32,
fld1: i128,
fld2: *mut [i8; 6],
fld3: Adt49,
fld4: i16,
fld5: (f32, [bool; 1], [isize; 7]),

},
Variant2{
fld0: (f32, [bool; 1], [isize; 7]),

},
Variant3{
fld0: [i64; 5],
fld1: *const u8,
fld2: Adt58,
fld3: (i8, [bool; 1], i128),
fld4: [i128; 6],
fld5: *mut f64,
fld6: *mut i8,
fld7: i128,

}}

