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
pub fn fn0() -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _1: *mut &'static [u16; 8];
let _2: u32;
let _3: [bool; 7];
let _4: char;
let _5: [usize; 5];
let _6: usize;
let _7: &'static *const u8;
let _8: &'static f64;
let _9: Adt73;
let _10: [usize; 7];
let _11: char;
let _12: &'static i32;
let _13: *mut &'static u32;
let _14: i16;
let _15: [usize; 3];
let _16: Adt73;
let _17: usize;
let _18: f32;
let _19: ((bool, u64, usize, isize),);
let _20: i64;
let _21: usize;
let _22: isize;
let _23: ();
let _24: ();
{
RET = [4922705149366732038_usize,7193958119298464874_usize,5293048310482529944_usize,6839598073080295118_usize,4_usize,15546354736307588816_usize];
RET = [0_usize,3375653596982013107_usize,7797136737605050776_usize,5662239702355665163_usize,4_usize,5_usize];
RET = [688119065997761584_usize,0_usize,2_usize,3137771487776717847_usize,1_usize,3_usize];
RET = [14085841216606851013_usize,15450111689274535167_usize,7_usize,9832253312026936196_usize,0_usize,1550023693763699594_usize];
RET = [17333156446359905443_usize,2375073305740848995_usize,13100603169607709726_usize,15578917781140422734_usize,1273896478787136391_usize,8145586781499073927_usize];
RET = [2_usize,13940751467906699471_usize,1675652331033262012_usize,1_usize,6_usize,4_usize];
_2 = !1176354541_u32;
RET = [3_usize,7083744563780862517_usize,11031996644347492191_usize,1_usize,6_usize,14979991753023044182_usize];
RET = [6_usize,4_usize,942830009315635781_usize,8783847161428113315_usize,15749592499236455085_usize,6_usize];
_3 = [true,false,true,true,false,true,false];
_3 = [true,true,false,false,true,false,true];
RET = [5850869554425894255_usize,8543084474674405984_usize,16637815706008297129_usize,6_usize,5_usize,3_usize];
_3 = [true,false,true,true,true,false,true];
_3 = [false,false,false,false,true,true,true];
Goto(bb1)
}
bb1 = {
_4 = '\u{10b8df}';
_2 = !3088905605_u32;
_2 = 1983108446_u32;
_2 = 284399227902086749494888683415544105902_u128 as u32;
RET = [13125007902330448795_usize,5_usize,14684949095844546416_usize,17733620489653878597_usize,4_usize,7_usize];
_2 = 3411457091_u32;
RET = [1_usize,3_usize,6_usize,2_usize,10678604270237164795_usize,12532314808314113724_usize];
_5 = [0_usize,1_usize,5402457976794908351_usize,5084906076906635667_usize,2_usize];
RET = [0_usize,0_usize,10020699877068777042_usize,2316309405823897541_usize,3_usize,10445345035372685977_usize];
_2 = !2831271295_u32;
_4 = '\u{8bfbd}';
Goto(bb2)
}
bb2 = {
_2 = !3787102985_u32;
Goto(bb3)
}
bb3 = {
RET = [6_usize,0_usize,4_usize,3073535389802199471_usize,14525199694241980410_usize,4_usize];
_5 = [16077007873234218563_usize,1103162745870638106_usize,14317417734128460583_usize,5_usize,8904377649053575110_usize];
_2 = 3070746785_u32;
_5 = [6568878454277464243_usize,4_usize,8266176928614216327_usize,10636341372525642647_usize,4_usize];
_3 = [false,true,false,true,false,false,true];
_2 = 1158418567_u32;
_3 = [false,true,false,false,true,false,false];
_4 = '\u{5aa12}';
_3 = [false,true,false,false,true,true,false];
_2 = 1622608445_u32;
_2 = 602820325_i32 as u32;
_5 = [6_usize,7575750686891113665_usize,17088495628680687264_usize,18206998303348745487_usize,1_usize];
_3 = [false,true,false,true,true,true,false];
_3 = [false,true,false,false,false,true,true];
_4 = '\u{2daa0}';
_5 = [0_usize,980966709539671552_usize,11347230400582804703_usize,2_usize,1_usize];
Goto(bb4)
}
bb4 = {
_4 = '\u{37ad6}';
RET = [12271254577674843989_usize,1_usize,5_usize,4_usize,3_usize,3_usize];
_4 = '\u{b0850}';
_6 = 7_usize & 7735671977457628307_usize;
Call(_5 = fn1(RET, _3, _6, _3, _4, RET, _3, RET, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = [false,false,true,true,false,false,false];
_3 = [true,false,false,true,true,false,true];
_4 = '\u{e989e}';
_4 = '\u{104ba4}';
_10 = [_6,_6,_6,_6,_6,_6,_6];
_2 = 1675971913_u32 * 3321258956_u32;
_6 = 4_usize + 7_usize;
_11 = _4;
RET = [_6,_6,_6,_6,_6,_6];
_4 = _11;
_11 = _4;
RET = [_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6];
Goto(bb6)
}
bb6 = {
_6 = !0_usize;
_3 = [true,true,false,true,false,false,false];
_11 = _4;
_5 = [_6,_6,_6,_6,_6];
_5 = [_6,_6,_6,_6,_6];
_10 = [_6,_6,_6,_6,_6,_6,_6];
_6 = !4556455229355498901_usize;
_5 = [_6,_6,_6,_6,_6];
_3 = [true,true,true,false,false,false,true];
_3 = [false,false,true,false,false,true,true];
_3 = [true,false,true,false,true,false,false];
RET = [_6,_6,_6,_6,_6,_6];
_3 = [false,false,false,false,true,false,true];
_6 = 3_usize;
_4 = _11;
_14 = -(-9124_i16);
_14 = (-16128_i16) << _2;
RET[_6] = _2 as usize;
Goto(bb7)
}
bb7 = {
Goto(bb8)
}
bb8 = {
RET[_6] = 5666193812281206018_i64 as usize;
_10[_6] = _6 | _5[_6];
_3 = [false,true,false,false,false,false,true];
_4 = _11;
_11 = _4;
_3 = [false,false,false,true,true,false,true];
RET = [_6,_10[_6],_6,_10[_6],_5[_6],_10[_6]];
_17 = !_10[_6];
_14 = _2 as i16;
_11 = _4;
_5 = [_17,RET[_6],_10[_6],_10[_6],RET[_6]];
RET = [_10[_6],_17,_5[_6],_5[_6],_17,_10[_6]];
RET[_6] = _2 as usize;
RET[_6] = _17;
RET[_6] = _5[_6];
_17 = RET[_6];
_18 = (-88_i8) as f32;
_3[_6] = _14 != _14;
_15 = [_6,_10[_6],_5[_6]];
RET[_6] = _17 | _6;
_5 = [_17,_10[_6],_10[_6],_10[_6],_10[_6]];
_14 = 30787_u16 as i16;
_19.0.2 = 246817199582031936172782417008147922147_u128 as usize;
_19.0 = (_3[_6], 6254782981105329023_u64, _17, (-116_isize));
match _19.0.3 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463463374607431768211340 => bb14,
_ => bb13
}
}
bb9 = {
Goto(bb8)
}
bb10 = {
_6 = !0_usize;
_3 = [true,true,false,true,false,false,false];
_11 = _4;
_5 = [_6,_6,_6,_6,_6];
_5 = [_6,_6,_6,_6,_6];
_10 = [_6,_6,_6,_6,_6,_6,_6];
_6 = !4556455229355498901_usize;
_5 = [_6,_6,_6,_6,_6];
_3 = [true,true,true,false,false,false,true];
_3 = [false,false,true,false,false,true,true];
_3 = [true,false,true,false,true,false,false];
RET = [_6,_6,_6,_6,_6,_6];
_3 = [false,false,false,false,true,false,true];
_6 = 3_usize;
_4 = _11;
_14 = -(-9124_i16);
_14 = (-16128_i16) << _2;
RET[_6] = _2 as usize;
Goto(bb7)
}
bb11 = {
_2 = !3787102985_u32;
Goto(bb3)
}
bb12 = {
_4 = '\u{37ad6}';
RET = [12271254577674843989_usize,1_usize,5_usize,4_usize,3_usize,3_usize];
_4 = '\u{b0850}';
_6 = 7_usize & 7735671977457628307_usize;
Call(_5 = fn1(RET, _3, _6, _3, _4, RET, _3, RET, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
RET = [6_usize,0_usize,4_usize,3073535389802199471_usize,14525199694241980410_usize,4_usize];
_5 = [16077007873234218563_usize,1103162745870638106_usize,14317417734128460583_usize,5_usize,8904377649053575110_usize];
_2 = 3070746785_u32;
_5 = [6568878454277464243_usize,4_usize,8266176928614216327_usize,10636341372525642647_usize,4_usize];
_3 = [false,true,false,true,false,false,true];
_2 = 1158418567_u32;
_3 = [false,true,false,false,true,false,false];
_4 = '\u{5aa12}';
_3 = [false,true,false,false,true,true,false];
_2 = 1622608445_u32;
_2 = 602820325_i32 as u32;
_5 = [6_usize,7575750686891113665_usize,17088495628680687264_usize,18206998303348745487_usize,1_usize];
_3 = [false,true,false,true,true,true,false];
_3 = [false,true,false,false,false,true,true];
_4 = '\u{2daa0}';
_5 = [0_usize,980966709539671552_usize,11347230400582804703_usize,2_usize,1_usize];
Goto(bb4)
}
bb14 = {
RET = [_19.0.2,_17,_6,_5[_6],_17,_6];
_3 = [_19.0.0,_19.0.0,_19.0.0,_19.0.0,_19.0.0,_19.0.0,_19.0.0];
_17 = _10[_6];
_10[_6] = !_19.0.2;
_11 = _4;
_19.0 = (_3[_6], 5187316313601732580_u64, _5[_6], 54_isize);
_18 = _19.0.1 as f32;
_19.0.3 = _14 as isize;
_19.0 = (_3[_6], 2664040353521611615_u64, _17, 68_isize);
_4 = _11;
_5[_6] = 171947766915124016686603291448209380681_u128 as usize;
_19.0.3 = !(-66_isize);
_17 = _6;
RET = [_5[_6],_5[_6],_5[_6],_10[_6],_10[_6],_10[_6]];
RET[_6] = _18 as usize;
_19.0.1 = !1941338349014523662_u64;
_10 = [RET[_6],_19.0.2,RET[_6],RET[_6],RET[_6],_19.0.2,RET[_6]];
RET = [_19.0.2,_10[_6],_10[_6],_10[_6],_5[_6],_19.0.2];
_5 = [RET[_6],RET[_6],RET[_6],RET[_6],_10[_6]];
_19.0.2 = RET[_6];
RET[_6] = 181_u8 as usize;
_19.0.0 = _3[_6] & _3[_6];
_4 = _11;
_10 = [_6,_19.0.2,_6,_5[_6],_17,_6,_17];
_14 = 267473597176792587247829204934150458173_u128 as i16;
_19.0.3 = 274489754884608381586622976893843578483_u128 as isize;
_10 = [_19.0.2,_19.0.2,RET[_6],_5[_6],_5[_6],_5[_6],_19.0.2];
RET[_6] = 70_u8 as usize;
_19.0.1 = 802158091448926191_u64 & 16484097698666945795_u64;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(0_usize, 11_usize, Move(_11), 4_usize, Move(_4), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(0_usize, 3_usize, Move(_3), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [usize; 6],mut _2: [bool; 7],mut _3: usize,mut _4: [bool; 7],mut _5: char,mut _6: [usize; 6],mut _7: [bool; 7],mut _8: [usize; 6],mut _9: [bool; 7]) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _10: *mut &'static u32;
let _11: Adt73;
let _12: Adt71;
let _13: u32;
let _14: isize;
let _15: isize;
let _16: [u32; 1];
let _17: (*const i64, usize, [u32; 5], ([u32; 5], i8));
let _18: char;
let _19: isize;
let _20: &'static [u32; 5];
let _21: [usize; 5];
let _22: (*const i64, usize, [u32; 5], ([u32; 5], i8));
let _23: f64;
let _24: [usize; 6];
let _25: &'static Adt22;
let _26: u128;
let _27: f64;
let _28: char;
let _29: (bool, u64, usize, isize);
let _30: usize;
let _31: i32;
let _32: ();
let _33: ();
{
_3 = 4_usize - 3_usize;
_6 = [_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3];
_5 = '\u{a3030}';
_7 = [false,true,false,false,false,true,false];
_4 = [false,true,true,true,false,true,true];
_4 = _9;
_6 = [_3,_3,_3,_3,_3,_3];
_5 = '\u{55855}';
_9 = _4;
_2 = [false,true,false,true,false,true,true];
_9 = [false,false,false,true,true,false,false];
_8 = [_3,_3,_3,_3,_3,_3];
_5 = '\u{79e0e}';
_7 = _9;
_3 = !3953741689007326464_usize;
Goto(bb1)
}
bb1 = {
_3 = 17706868330549270760_usize;
_2 = _4;
_2 = _7;
_8 = _1;
_13 = !2905704957_u32;
Goto(bb2)
}
bb2 = {
_12.fld3 = [_13,_13,_13,_13,_13];
_12.fld3 = [_13,_13,_13,_13,_13];
_12.fld0.3 = true;
_5 = '\u{3fd69}';
match _3 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
17706868330549270760 => bb10,
_ => bb9
}
}
bb3 = {
_3 = 17706868330549270760_usize;
_2 = _4;
_2 = _7;
_8 = _1;
_13 = !2905704957_u32;
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
_4 = [_12.fld0.3,_12.fld0.3,_12.fld0.3,_12.fld0.3,_12.fld0.3,_12.fld0.3,_12.fld0.3];
_3 = 5_usize >> _13;
_12.fld0.1 = Adt44::Variant1 { fld0: 4619170767559230733_u64 };
_15 = 9223372036854775807_isize;
_15 = _3 as isize;
_16 = [_13];
Goto(bb11)
}
bb11 = {
_5 = '\u{10b40f}';
_6 = [_3,_3,_3,_3,_3,_3];
RET = [_3,_3,_3,_3,_3];
_12.fld0.2 = _7;
_12.fld1.0 = (_12.fld0.3, 17288539963507516916_u64, _3, _15);
Goto(bb12)
}
bb12 = {
_12.fld0.0.0 = core::ptr::addr_of!(_12.fld2);
_17.3 = (_12.fld3, (-68_i8));
_15 = (-2121784616_i32) as isize;
_12.fld1.0.0 = _17.3.1 != _17.3.1;
_12.fld3 = [_13,_13,_13,_13,_13];
_18 = _5;
_1 = [_3,_12.fld1.0.2,_3,_12.fld1.0.2,_12.fld1.0.2,_3];
_7 = [_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0];
_17.1 = !_3;
_12.fld0.0.2 = 646344882_i32;
_2 = [_12.fld0.3,_12.fld1.0.0,_12.fld0.3,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0];
RET = [_3,_3,_3,_17.1,_17.1];
_7 = [_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld0.3,_12.fld1.0.0,_12.fld1.0.0];
_9 = [_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld0.3,_12.fld1.0.0,_12.fld1.0.0];
_12.fld0.0.2 = 234_u8 as i32;
_1 = _8;
_19 = _12.fld1.0.3 & _12.fld1.0.3;
_17.3.0 = [_13,_13,_13,_13,_13];
_14 = _19;
_17.3.1 = _12.fld1.0.0 as i8;
_22.2 = [_13,_13,_13,_13,_13];
_7 = _9;
_13 = _12.fld1.0.1 as u32;
_22.3 = (_12.fld3, _17.3.1);
Call(_15 = fn2(_12.fld1.0.1, _3, _5, _12.fld1, _12.fld1, _19, _13, _4, _16, _1, _12.fld1.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_12.fld0.2 = [_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0];
_12.fld1.0 = (_12.fld0.3, 18133900278131112547_u64, _3, _15);
_22.2 = [_13,_13,_13,_13,_13];
_17.3 = (_22.2, _22.3.1);
_20 = &_17.2;
_24 = _6;
_12.fld0.0.0 = core::ptr::addr_of!(_12.fld2);
_17.3 = _22.3;
RET = [_17.1,_17.1,_17.1,_12.fld1.0.2,_3];
_2 = [_12.fld0.3,_12.fld1.0.0,_12.fld0.3,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0,_12.fld1.0.0];
_1 = [_17.1,_12.fld1.0.2,_12.fld1.0.2,_12.fld1.0.2,_17.1,_3];
_1 = [_12.fld1.0.2,_17.1,_3,_17.1,_3,_17.1];
_17.2 = _22.2;
_12.fld0.3 = _12.fld1.0.0;
place!(Field::<u64>(Variant(_12.fld0.1, 1), 0)) = !_12.fld1.0.1;
_12.fld1.0.3 = _15;
_22.3 = (_22.2, _17.3.1);
_4 = [_12.fld0.3,_12.fld0.3,_12.fld1.0.0,_12.fld0.3,_12.fld1.0.0,_12.fld0.3,_12.fld0.3];
_12.fld0.0.2 = (-1063095010_i32) * 1663722734_i32;
_12.fld1.0.1 = !Field::<u64>(Variant(_12.fld0.1, 1), 0);
_15 = _12.fld1.0.3 * _19;
_12.fld1.0 = (_12.fld0.3, Field::<u64>(Variant(_12.fld0.1, 1), 0), _3, _15);
_18 = _5;
Goto(bb14)
}
bb14 = {
_13 = !4218225159_u32;
_22.3.1 = _18 as i8;
_20 = &_22.3.0;
SetDiscriminant(_12.fld0.1, 1);
_12.fld1.0.0 = _12.fld1.0.3 > _15;
_17.3.1 = -_22.3.1;
_15 = _12.fld1.0.3;
_12.fld0.0.2 = !120243912_i32;
_23 = (-6586293526585564903250826253087428431_i128) as f64;
_31 = _12.fld0.0.2;
_13 = _12.fld1.0.0 as u32;
_4 = _12.fld0.2;
_12.fld1.0.1 = 8394355543627751993_u64;
place!(Field::<u64>(Variant(_12.fld0.1, 1), 0)) = _12.fld1.0.0 as u64;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(1_usize, 18_usize, Move(_18), 9_usize, Move(_9), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(1_usize, 16_usize, Move(_16), 19_usize, Move(_19), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u64,mut _2: usize,mut _3: char,mut _4: ((bool, u64, usize, isize),),mut _5: ((bool, u64, usize, isize),),mut _6: isize,mut _7: u32,mut _8: [bool; 7],mut _9: [u32; 1],mut _10: [usize; 6],mut _11: (bool, u64, usize, isize)) -> isize {
mir! {
type RET = isize;
let _12: f64;
let _13: [i8; 8];
let _14: Adt58;
let _15: [u32; 3];
let _16: [usize; 7];
let _17: ();
let _18: ();
{
RET = _5.0.3;
_2 = 6429784604415009181_i64 as usize;
_11.1 = _3 as u64;
Call(_2 = core::intrinsics::bswap(_11.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.1 = _1;
_4 = _5;
_5.0.0 = RET <= _6;
_5.0.2 = _11.2 - _2;
_4.0.2 = _5.0.2;
_6 = 114_u8 as isize;
_1 = _5.0.1;
_11.2 = _4.0.2 << _7;
_9 = [_7];
_11.0 = !_5.0.0;
_4.0.3 = _5.0.3 >> _5.0.1;
_1 = 1219796771_i32 as u64;
_11.3 = _4.0.3;
_9 = [_7];
_11.1 = _4.0.3 as u64;
_6 = RET;
_7 = (-94153208574767802812999939863541566014_i128) as u32;
_2 = !_11.2;
_5.0.1 = _4.0.1 & _1;
_1 = !_4.0.1;
Call(_14 = fn3(_4.0, _11.3, _4.0.3, _5.0.0, _4.0, _11, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = -Field::<(*mut u8, f64, char, i64)>(Variant(_14, 1), 4).1;
_7 = Field::<u8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 1) as u32;
_11.1 = _1 ^ _5.0.1;
_1 = _11.1;
_4.0.0 = Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 6).0 ^ Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 6).0;
place!(Field::<isize>(Variant(_14, 1), 2)) = Field::<(*mut u8, f64, char, i64)>(Variant(_14, 1), 4).2 as isize;
_13 = [Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3),Field::<i8>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 3)];
place!(Field::<u64>(Variant(place!(Field::<Adt22>(Variant(_14, 1), 5)), 2), 0)) = _11.1 >> Field::<isize>(Variant(_14, 1), 2);
place!(Field::<(bool, u64, usize, isize)>(Variant(place!(Field::<Adt22>(Variant(_14, 1), 5)), 2), 6)).2 = _5.0.2;
RET = Field::<isize>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 2) >> Field::<u64>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 0);
place!(Field::<(*mut u8, f64, char, i64)>(Variant(_14, 1), 4)).2 = _3;
_5.0 = (Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 6).0, _4.0.1, _11.2, Field::<isize>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 2));
_6 = -RET;
place!(Field::<f64>(Variant(place!(Field::<Adt22>(Variant(_14, 1), 5)), 2), 4)) = -Field::<(*mut u8, f64, char, i64)>(Variant(_14, 1), 4).1;
_5.0.0 = !Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(_14, 1), 5), 2), 6).0;
_5 = (_4.0,);
_4.0.0 = !_11.0;
place!(Field::<u64>(Variant(place!(Field::<Adt22>(Variant(_14, 1), 5)), 2), 0)) = _1;
Goto(bb3)
}
bb3 = {
Call(_17 = dump_var(2_usize, 8_usize, Move(_8), 6_usize, Move(_6), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_17 = dump_var(2_usize, 9_usize, Move(_9), 2_usize, Move(_2), 18_usize, _18, 18_usize, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: (bool, u64, usize, isize),mut _2: isize,mut _3: isize,mut _4: bool,mut _5: (bool, u64, usize, isize),mut _6: (bool, u64, usize, isize),mut _7: [u32; 1]) -> Adt58 {
mir! {
type RET = Adt58;
let _8: (f64, *mut &'static u32, *const i8, *mut u8);
let _9: isize;
let _10: u128;
let _11: f64;
let _12: [u128; 5];
let _13: [i8; 2];
let _14: i8;
let _15: isize;
let _16: *mut u128;
let _17: f32;
let _18: *mut *const u32;
let _19: [u32; 1];
let _20: &'static u32;
let _21: f64;
let _22: ();
let _23: ();
{
_5.0 = !_6.0;
_1.1 = _5.1;
_5.1 = _6.1;
_8.0 = 218172455987854566345842221758480331941_u128 as f64;
_6 = (_5.0, _1.1, _5.2, _3);
_8.0 = _6.3 as f64;
_9 = -_5.3;
_1 = _5;
_9 = _6.3 - _1.3;
Call(_1.3 = core::intrinsics::transmute(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5.0 = _6.0 < _6.0;
_1 = (_6.0, _5.1, _5.2, _6.3);
_1.1 = 138670697942241440086624827284029853699_i128 as u64;
_9 = !_1.3;
Goto(bb2)
}
bb2 = {
_9 = _2;
_6.2 = _5.2;
_6.2 = _5.2;
_5.2 = _6.2 << _2;
_1.0 = _5.0 ^ _4;
_6.0 = _1.0;
_10 = !160872185296054266998857344639415287312_u128;
_6.0 = _1.0;
_8.0 = _3 as f64;
_4 = _6.0;
_5.0 = _9 <= _6.3;
_1.3 = _3 & _6.3;
_6.0 = _5.0 != _1.0;
_5.0 = _5.2 != _5.2;
_5 = _1;
_1.3 = _6.1 as isize;
_12 = [_10,_10,_10,_10,_10];
_6 = (_5.0, _1.1, _5.2, _5.3);
_1.2 = _5.2;
_6.3 = !_2;
_1.0 = _5.0 != _6.0;
_1.0 = _5.0;
_1.3 = _5.3;
_9 = _5.3;
_11 = _8.0;
_1 = (_6.0, _5.1, _6.2, _5.3);
_1.1 = _6.1;
Call(_11 = core::intrinsics::fmaf64(_8.0, _8.0, _8.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1.1 = _6.1;
_1.1 = _5.1;
RET = Adt58::Variant2 { fld0: _12 };
_11 = 3830374645644141529_i64 as f64;
_8.2 = core::ptr::addr_of!(_14);
_5 = (_1.0, _1.1, _6.2, _1.3);
_12 = Field::<[u128; 5]>(Variant(RET, 2), 0);
_5.3 = _1.3 ^ _9;
_9 = _5.2 as isize;
_8.2 = core::ptr::addr_of!(_14);
_11 = -_8.0;
_2 = (-14528_i16) as isize;
_13 = [90_i8,45_i8];
Call(_3 = fn4(_6.0, _5.3, _5.0, _5.3, _1.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1.3 = -_3;
_14 = (-126_i8) & 56_i8;
_6 = (_4, _5.1, _5.2, _1.3);
_7 = [2417014631_u32];
_4 = _6.0;
_2 = _6.3;
SetDiscriminant(RET, 1);
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)).2 = '\u{102d00}';
_12 = [_10,_10,_10,_10,_10];
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)).1 = _8.0 - _11;
_5.2 = _1.2 & _1.2;
_4 = !_5.0;
_6.2 = _5.2 - _5.2;
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)).3 = 7133663637663343446_i64 - 1587533860613201719_i64;
_5 = (_6.0, _6.1, _6.2, _2);
_1 = (_5.0, _6.1, _5.2, _6.3);
_6.3 = 7549_i16 as isize;
_16 = core::ptr::addr_of_mut!(_10);
_16 = core::ptr::addr_of_mut!(_10);
_3 = _5.3;
(*_16) = 278626409960065934305608215402181245679_u128 << _2;
_11 = _8.0;
Call(place!(Field::<*const u32>(Variant(RET, 1), 3)) = fn5(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4).2 as isize;
_15 = _3;
place!(Field::<isize>(Variant(RET, 1), 2)) = !_3;
place!(Field::<Adt22>(Variant(RET, 1), 5)) = Adt22::Variant2 { fld0: _6.1,fld1: 44_u8,fld2: _3,fld3: _14,fld4: Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4).1,fld5: 138482484575522071653779538611504088168_i128,fld6: _5 };
_13 = [Field::<i8>(Variant(Field::<Adt22>(Variant(RET, 1), 5), 2), 3),_14];
place!(Field::<f64>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 4)) = Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4).1 + _11;
_14 = Field::<i8>(Variant(Field::<Adt22>(Variant(RET, 1), 5), 2), 3);
_18 = core::ptr::addr_of_mut!(place!(Field::<*const u32>(Variant(RET, 1), 3)));
_16 = core::ptr::addr_of_mut!((*_16));
place!(Field::<u16>(Variant(RET, 1), 0)) = 22438_u16 >> _3;
place!(Field::<u8>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 1)) = !41_u8;
place!(Field::<i128>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 5)) = 9951756142403331202222753884337027977_i128;
place!(Field::<i128>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 5)) = -56925191110688159973735139194298491495_i128;
_8.3 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 1)));
_16 = core::ptr::addr_of_mut!((*_16));
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)).2 = '\u{47ce0}';
Goto(bb6)
}
bb6 = {
place!(Field::<(bool, u64, usize, isize)>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 6)).1 = !_5.1;
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)).2 = '\u{e687f}';
place!(Field::<f64>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 4)) = _8.0 * _11;
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)).3 = (-7624692121935588437_i64);
Call(_21 = fn19((*_16), _4, (*_16), Move(_16), Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(RET, 1), 5), 2), 6), Field::<isize>(Variant(RET, 1), 2), _15, _1.3, Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(RET, 1), 5), 2), 6)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
place!(Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4)) = (Move(_8.3), _11, '\u{ff14e}', 8697902537289712881_i64);
place!(Field::<u16>(Variant(RET, 1), 0)) = 40673_u16 ^ 23048_u16;
place!(Field::<*mut isize>(Variant(RET, 1), 1)) = core::ptr::addr_of_mut!(_1.3);
place!(Field::<(bool, u64, usize, isize)>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 6)).3 = Field::<u8>(Variant(Field::<Adt22>(Variant(RET, 1), 5), 2), 1) as isize;
place!(Field::<f64>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 4)) = Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4).1;
_1 = _6;
_16 = core::ptr::addr_of_mut!(_10);
_5 = (Field::<(bool, u64, usize, isize)>(Variant(Field::<Adt22>(Variant(RET, 1), 5), 2), 6).0, _6.1, _6.2, _15);
_9 = _8.0 as isize;
_5.2 = Field::<(*mut u8, f64, char, i64)>(Variant(RET, 1), 4).2 as usize;
place!(Field::<*mut isize>(Variant(RET, 1), 1)) = core::ptr::addr_of_mut!(place!(Field::<(bool, u64, usize, isize)>(Variant(place!(Field::<Adt22>(Variant(RET, 1), 5)), 2), 6)).3);
place!(Field::<isize>(Variant(RET, 1), 2)) = _15;
Goto(bb8)
}
bb8 = {
Call(_22 = dump_var(3_usize, 7_usize, Move(_7), 3_usize, Move(_3), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_22 = dump_var(3_usize, 5_usize, Move(_5), 4_usize, Move(_4), 23_usize, _23, 23_usize, _23), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: isize,mut _3: bool,mut _4: isize,mut _5: bool) -> isize {
mir! {
type RET = isize;
let _6: *mut *const u32;
let _7: f64;
let _8: u32;
let _9: *mut *mut &'static [u16; 8];
let _10: [u16; 8];
let _11: f64;
let _12: i16;
let _13: u128;
let _14: bool;
let _15: i128;
let _16: &'static f64;
let _17: (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _18: bool;
let _19: ((*const u32, Adt22, i32), Adt44, [bool; 7], bool);
let _20: f32;
let _21: Adt73;
let _22: char;
let _23: u8;
let _24: isize;
let _25: ();
let _26: ();
{
_4 = !_2;
RET = (-654970946_i32) as isize;
_4 = _2 | _2;
Goto(bb1)
}
bb1 = {
_2 = _4;
_2 = _4 & _4;
_3 = !_5;
_4 = (-135079660323695789387370594972645332596_i128) as isize;
_7 = _2 as f64;
_2 = RET;
_4 = RET;
_5 = _3;
_5 = _1 & _3;
_1 = _7 != _7;
_1 = _5;
_8 = 205_u8 as u32;
_3 = !_5;
RET = _4;
_8 = 3169119303_u32 >> RET;
_4 = -RET;
_7 = 97_u8 as f64;
_4 = (-43912842503519181790102802373024169491_i128) as isize;
_5 = _3 > _1;
RET = _4 >> _2;
RET = '\u{10d7f8}' as isize;
_7 = _2 as f64;
RET = _4;
_7 = RET as f64;
Goto(bb2)
}
bb2 = {
_8 = 19761356_u32;
_3 = _5;
_10 = [1242_u16,62119_u16,64069_u16,26055_u16,37145_u16,14491_u16,35161_u16,17899_u16];
RET = _4;
RET = _4;
_2 = (-44_i8) as isize;
_8 = 109907613030949254030323620023930997825_i128 as u32;
_5 = !_1;
RET = _4;
_8 = 870082307_u32 | 569644453_u32;
_7 = (-571745559_i32) as f64;
Goto(bb3)
}
bb3 = {
_4 = 7_usize as isize;
_5 = _1 >= _3;
_2 = _4 + RET;
_12 = !27002_i16;
RET = 1_usize as isize;
_13 = 16446055467515502375_u64 as u128;
RET = 133363808494165961383699700551116141984_i128 as isize;
_4 = !RET;
Goto(bb4)
}
bb4 = {
_4 = !_2;
_7 = 6363970644237604425_u64 as f64;
_14 = _5 != _5;
_3 = _14 <= _14;
_11 = -_7;
_4 = !RET;
_2 = _12 as isize;
_5 = _1 == _14;
_10 = [22381_u16,31265_u16,41615_u16,5740_u16,63728_u16,40859_u16,2999_u16,51916_u16];
_14 = !_3;
_8 = 50979_u16 as u32;
RET = _2;
_14 = !_1;
_8 = 562233765_u32;
_16 = &_11;
_10 = [27973_u16,56294_u16,35312_u16,61528_u16,58777_u16,56903_u16,60945_u16,43025_u16];
_11 = _7;
RET = _4 ^ _4;
_7 = -_11;
_11 = -_7;
_2 = RET << RET;
_7 = -_11;
RET = 4_usize as isize;
_5 = !_3;
_4 = _8 as isize;
_15 = -83585360172288903603331690281380552310_i128;
_7 = _11 * _11;
Goto(bb5)
}
bb5 = {
_10 = [15192_u16,57685_u16,12141_u16,13735_u16,41780_u16,55639_u16,42412_u16,46830_u16];
_19.0.0 = core::ptr::addr_of!(_8);
_16 = &_17.2.0.1;
_17.2.2 = !7_u8;
_5 = _14 <= _3;
_19.1 = Adt44::Variant1 { fld0: 12222222545729778172_u64 };
_1 = _5 & _3;
_19.2 = [_1,_14,_1,_3,_5,_14,_1];
_17.2.0.2 = '\u{61ea8}';
_17.2.0.3 = -8236670949151709609_i64;
_17.0 = core::ptr::addr_of!(_19.0.1);
place!(Field::<u64>(Variant(_19.1, 1), 0)) = 3748035326580523153_u64;
SetDiscriminant(_19.1, 1);
_17.2.0.2 = '\u{26bfa}';
_19.2 = [_1,_3,_1,_5,_1,_14,_14];
_17.0 = core::ptr::addr_of!(_19.0.1);
_4 = (-114_i8) as isize;
_17.2.0.1 = _7 * _11;
_12 = _5 as i16;
_17.2.3 = _17.2.2;
_17.2.0.0 = core::ptr::addr_of_mut!(_17.2.3);
_19.3 = !_5;
_14 = !_5;
_19.0.1 = Adt22::Variant0 { fld0: _7,fld1: _15,fld2: _12,fld3: 116_i8 };
_19.0.2 = (-498710187_i32);
match _19.0.2 {
0 => bb1,
340282366920938463463374607431269501269 => bb7,
_ => bb6
}
}
bb6 = {
_4 = 7_usize as isize;
_5 = _1 >= _3;
_2 = _4 + RET;
_12 = !27002_i16;
RET = 1_usize as isize;
_13 = 16446055467515502375_u64 as u128;
RET = 133363808494165961383699700551116141984_i128 as isize;
_4 = !RET;
Goto(bb4)
}
bb7 = {
_19.2 = [_19.3,_3,_19.3,_5,_19.3,_1,_19.3];
_5 = !_1;
_19.0.0 = core::ptr::addr_of!(_8);
_4 = _5 as isize;
_19.3 = _1;
_3 = _5;
_16 = &_11;
_13 = 73689898967443236130133167402981631841_u128;
_19.1 = Adt44::Variant1 { fld0: 8171725529914024736_u64 };
_20 = _15 as f32;
_17.2.0.0 = core::ptr::addr_of_mut!(_17.2.3);
_20 = _8 as f32;
_19.0.0 = core::ptr::addr_of!(_8);
_5 = _1 | _19.3;
RET = _4 >> _12;
_1 = !_5;
_1 = !_19.3;
Goto(bb8)
}
bb8 = {
Call(_25 = dump_var(4_usize, 4_usize, Move(_4), 1_usize, Move(_1), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_25 = dump_var(4_usize, 14_usize, Move(_14), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5() -> *const u32 {
mir! {
type RET = *const u32;
let _1: isize;
let _2: &'static *mut u128;
let _3: [i8; 8];
let _4: isize;
let _5: (bool, u64, usize, isize);
let _6: (((*mut u8, f64, char, i64), &'static usize, u8, u8), i64, [usize; 3], (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8)));
let _7: (*const i64, usize, [u32; 5], ([u32; 5], i8));
let _8: Adt81;
let _9: i128;
let _10: i16;
let _11: (*const u32, Adt22, i32);
let _12: f64;
let _13: (u8, u64, i8, u16);
let _14: *mut &'static u32;
let _15: isize;
let _16: [usize; 3];
let _17: f32;
let _18: &'static u32;
let _19: i128;
let _20: u64;
let _21: Adt73;
let _22: &'static [u16; 8];
let _23: (bool, u64, usize, isize);
let _24: *const i64;
let _25: *mut *const u32;
let _26: *mut isize;
let _27: [usize; 3];
let _28: i64;
let _29: [u16; 8];
let _30: u64;
let _31: f64;
let _32: (*mut u8, f64, char, i64);
let _33: usize;
let _34: *mut Adt49;
let _35: *mut Adt49;
let _36: *mut Adt49;
let _37: [usize; 3];
let _38: [bool; 7];
let _39: [u32; 1];
let _40: u32;
let _41: [u32; 3];
let _42: ();
let _43: ();
{
_1 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_1 = !9223372036854775807_isize;
_1 = 9223372036854775807_isize & (-9223372036854775808_isize);
_1 = 9223372036854775807_isize;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
9223372036854775807 => bb8,
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
_1 = (-62_isize);
_1 = -5_isize;
_3 = [48_i8,(-91_i8),(-10_i8),(-72_i8),(-11_i8),44_i8,(-93_i8),92_i8];
_1 = 9223372036854775807_isize;
_3 = [83_i8,119_i8,(-62_i8),(-29_i8),8_i8,120_i8,(-39_i8),(-6_i8)];
_1 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_3 = [(-54_i8),(-62_i8),69_i8,62_i8,95_i8,(-42_i8),(-7_i8),126_i8];
_1 = '\u{c0fea}' as isize;
_1 = -9223372036854775807_isize;
_3 = [77_i8,(-119_i8),75_i8,11_i8,28_i8,44_i8,(-127_i8),4_i8];
_3 = [5_i8,108_i8,(-30_i8),(-34_i8),45_i8,76_i8,25_i8,(-3_i8)];
_3 = [82_i8,(-47_i8),(-13_i8),(-85_i8),29_i8,(-101_i8),92_i8,101_i8];
_3 = [(-77_i8),(-48_i8),3_i8,71_i8,86_i8,60_i8,9_i8,65_i8];
_1 = 9223372036854775807_isize;
_3 = [14_i8,(-27_i8),43_i8,13_i8,108_i8,(-91_i8),8_i8,106_i8];
_3 = [(-53_i8),(-51_i8),(-36_i8),(-121_i8),63_i8,(-101_i8),(-28_i8),(-74_i8)];
_5.3 = _1 >> _1;
_5.1 = 18090521373870146783_u64 << _5.3;
_5.0 = false;
_5.0 = false;
_3 = [2_i8,63_i8,(-70_i8),(-50_i8),(-68_i8),36_i8,68_i8,(-90_i8)];
_6.3.2.1 = &_5.2;
match _1 {
0 => bb3,
9223372036854775807 => bb9,
_ => bb2
}
}
bb9 = {
_8.fld0 = [2155973121_u32];
_6.0.1 = &_7.1;
_5.3 = _1;
_7.3.1 = (-44_i8) * (-4_i8);
_6.0.0.2 = '\u{6454e}';
_6.3.2.0.3 = (-7925072341223947034_i64);
_5 = (true, 8005098479869773983_u64, 13103485287553525857_usize, _1);
_9 = (-57480224884474547752689131097726247670_i128) & 140082758877692413505462962018897287178_i128;
_6.3.2.3 = 96_u8;
_6.0.0.0 = core::ptr::addr_of_mut!(_6.0.3);
Goto(bb10)
}
bb10 = {
_6.0.2 = !_6.3.2.3;
_6.0.0.1 = _5.2 as f64;
_7.0 = core::ptr::addr_of!(_6.3.2.0.3);
_6.3.2.0.0 = core::ptr::addr_of_mut!(_6.0.3);
_6.3.2.1 = &_7.1;
_6.3.2.2 = !_6.3.2.3;
_5.3 = _1;
_6.3.2.0.3 = 2135645336896597308_i64;
_7.1 = 15047_u16 as usize;
_6.3.1 = [327288175783318159936863825614934894838_u128,88855210521795650160463717703964451640_u128,7632024285086857276159869560121071913_u128,239767842760633191207600104297535363981_u128,85674934770519384780672045293482258118_u128];
_5.3 = _1;
_7.3.0 = [968305871_u32,3753446947_u32,3052562381_u32,2607525926_u32,204132013_u32];
_7.3.1 = 60_i8;
_7.3.1 = 111_i8 | (-26_i8);
_4 = -_1;
_6.3.2.0.0 = core::ptr::addr_of_mut!(_6.3.2.2);
_7.2 = [3744729906_u32,411788423_u32,2153052856_u32,1622699697_u32,902482296_u32];
_5.0 = true | false;
_3 = [_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1];
_3 = [_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1,_7.3.1];
_6.3.0 = core::ptr::addr_of!(_11.1);
_6.0.2 = _6.3.2.3 + _6.3.2.3;
_6.0.1 = &_7.1;
_6.3.2.2 = !_6.3.2.3;
_1 = -_5.3;
_5.0 = true;
match _5.1 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb5,
8005098479869773983 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_6.0.0.0 = core::ptr::addr_of_mut!(_6.3.2.2);
_13.1 = _5.1;
_7.1 = 2194400853592163177083607738367281541_u128 as usize;
_13.2 = _9 as i8;
_13 = (_6.3.2.2, _5.1, _7.3.1, 53662_u16);
_6.0.3 = !_6.3.2.3;
_5.3 = 92257575404246057356130076116065494978_u128 as isize;
_6.3.2.0.3 = (-2485033787704647333_i64) << _13.3;
_15 = 88446721789526776723407379162218642137_u128 as isize;
_6.3.2.0.1 = -_6.0.0.1;
_6.3.2.0.1 = _6.0.0.1;
match _13.1 {
0 => bb5,
1 => bb9,
2 => bb3,
3 => bb10,
8005098479869773983 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_7.3.0 = [1868964500_u32,1321890725_u32,2027403813_u32,1397709602_u32,2703392954_u32];
_6.2 = [_5.2,_5.2,_5.2];
_5.1 = _6.0.0.2 as u64;
_6.0.0.3 = !_6.3.2.0.3;
_6.3.2.3 = _6.0.2;
_6.3.2.0.0 = core::ptr::addr_of_mut!(_13.0);
_13 = (_6.3.2.2, _5.1, _7.3.1, 55599_u16);
_6.0.0.2 = '\u{71457}';
_7.1 = _5.2 << _13.3;
_11.1 = Adt22::Variant0 { fld0: _6.0.0.1,fld1: _9,fld2: 25046_i16,fld3: _13.2 };
_16 = _6.2;
_11.2 = -(-1434132954_i32);
Call(_6.3.2.0.3 = fn6(_13.3, Move(_6.0.0), _16, _6.2, Move(_6.3.0), _7.1, _4, _4, _7.1, _13.3, _6.0.2, _13.3), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_5.1 = _13.1 | _13.1;
_10 = 22013_i16 + 5470_i16;
place!(Field::<i16>(Variant(_11.1, 0), 2)) = _10;
_9 = Field::<i128>(Variant(_11.1, 0), 1) & Field::<i128>(Variant(_11.1, 0), 1);
_7.1 = _5.2 * _5.2;
_6.3.2.0.0 = core::ptr::addr_of_mut!(_6.0.3);
_7.3.0 = [462120180_u32,2373487229_u32,1830578482_u32,1354593543_u32,4058510263_u32];
_13.2 = -_7.3.1;
_6.3.2.1 = &_7.1;
match _13.3 {
0 => bb5,
1 => bb16,
55599 => bb18,
_ => bb17
}
}
bb16 = {
_1 = (-62_isize);
_1 = -5_isize;
_3 = [48_i8,(-91_i8),(-10_i8),(-72_i8),(-11_i8),44_i8,(-93_i8),92_i8];
_1 = 9223372036854775807_isize;
_3 = [83_i8,119_i8,(-62_i8),(-29_i8),8_i8,120_i8,(-39_i8),(-6_i8)];
_1 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_3 = [(-54_i8),(-62_i8),69_i8,62_i8,95_i8,(-42_i8),(-7_i8),126_i8];
_1 = '\u{c0fea}' as isize;
_1 = -9223372036854775807_isize;
_3 = [77_i8,(-119_i8),75_i8,11_i8,28_i8,44_i8,(-127_i8),4_i8];
_3 = [5_i8,108_i8,(-30_i8),(-34_i8),45_i8,76_i8,25_i8,(-3_i8)];
_3 = [82_i8,(-47_i8),(-13_i8),(-85_i8),29_i8,(-101_i8),92_i8,101_i8];
_3 = [(-77_i8),(-48_i8),3_i8,71_i8,86_i8,60_i8,9_i8,65_i8];
_1 = 9223372036854775807_isize;
_3 = [14_i8,(-27_i8),43_i8,13_i8,108_i8,(-91_i8),8_i8,106_i8];
_3 = [(-53_i8),(-51_i8),(-36_i8),(-121_i8),63_i8,(-101_i8),(-28_i8),(-74_i8)];
_5.3 = _1 >> _1;
_5.1 = 18090521373870146783_u64 << _5.3;
_5.0 = false;
_5.0 = false;
_3 = [2_i8,63_i8,(-70_i8),(-50_i8),(-68_i8),36_i8,68_i8,(-90_i8)];
_6.3.2.1 = &_5.2;
match _1 {
0 => bb3,
9223372036854775807 => bb9,
_ => bb2
}
}
bb17 = {
Return()
}
bb18 = {
_17 = _6.0.3 as f32;
_6.0.3 = _6.3.2.3;
_5 = (false, _13.1, _7.1, _4);
_11.1 = Adt22::Variant2 { fld0: _5.1,fld1: _6.0.3,fld2: _4,fld3: _7.3.1,fld4: _6.3.2.0.1,fld5: _9,fld6: _5 };
_7.0 = core::ptr::addr_of!(_6.3.2.0.3);
_23.3 = !_5.3;
_6.0.0.1 = _6.3.2.0.1 * Field::<f64>(Variant(_11.1, 2), 4);
place!(Field::<f64>(Variant(_11.1, 2), 4)) = -_6.0.0.1;
_19 = _9;
_6.0.0 = (Move(_6.3.2.0.0), Field::<f64>(Variant(_11.1, 2), 4), '\u{f7a39}', _6.3.2.0.3);
_6.0.1 = Move(_6.3.2.1);
_8.fld0 = [634158756_u32];
_7.1 = Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6).2 + Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6).2;
place!(Field::<u64>(Variant(_11.1, 2), 0)) = Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6).1 & _5.1;
_13.2 = Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6).0 as i8;
place!(Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6)).1 = !Field::<u64>(Variant(_11.1, 2), 0);
_1 = -_4;
_8.fld0 = [2949347171_u32];
place!(Field::<isize>(Variant(_11.1, 2), 2)) = !_4;
place!(Field::<u8>(Variant(_11.1, 2), 1)) = _6.0.3 << _6.3.2.0.3;
_11.1 = Adt22::Variant2 { fld0: _13.1,fld1: _6.3.2.2,fld2: _4,fld3: _13.2,fld4: _6.0.0.1,fld5: _9,fld6: _5 };
_24 = core::ptr::addr_of!(_6.0.0.3);
_12 = _17 as f64;
Goto(bb19)
}
bb19 = {
_6.0.0.0 = core::ptr::addr_of_mut!(_6.0.2);
_6.3.2.0.3 = (*_24);
_6.3.0 = core::ptr::addr_of!(_11.1);
_6.3.2.2 = !Field::<u8>(Variant(_11.1, 2), 1);
place!(Field::<i128>(Variant(_11.1, 2), 5)) = _19;
(*_24) = _6.3.2.0.3;
_1 = _5.3;
_23 = Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6);
_25 = core::ptr::addr_of_mut!(_11.0);
Goto(bb20)
}
bb20 = {
_6.3.2.2 = !_6.3.2.3;
_9 = Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6).2 as i128;
_6.0.3 = !_6.3.2.2;
_23 = _5;
place!(Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6)) = (_5.0, _23.1, _5.2, _23.3);
_6.3.1 = [61268858873357482414967814133113122182_u128,72804375721071938782752091515630802123_u128,148714196569552159743911036138648526458_u128,107676687444263755120925496511256500772_u128,188756818087478578905937637489272310796_u128];
place!(Field::<i8>(Variant(_11.1, 2), 3)) = !_13.2;
_13 = (_6.0.2, Field::<u64>(Variant(_11.1, 2), 0), Field::<i8>(Variant(_11.1, 2), 3), 20819_u16);
_7.0 = Move(_24);
_7.0 = core::ptr::addr_of!(_6.0.0.3);
_32.3 = -_6.0.0.3;
Goto(bb21)
}
bb21 = {
place!(Field::<f64>(Variant(_11.1, 2), 4)) = _6.0.0.1;
_30 = _6.3.2.2 as u64;
_23 = Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6);
_6.0.0.2 = '\u{5ef44}';
place!(Field::<u64>(Variant(_11.1, 2), 0)) = !_23.1;
_7.3.1 = !Field::<i8>(Variant(_11.1, 2), 3);
_20 = _13.3 as u64;
_5.2 = !_7.1;
_32.1 = _6.3.2.0.1 * _12;
_31 = -_6.3.2.0.1;
_14 = core::ptr::addr_of_mut!(_18);
_32.2 = _6.0.0.2;
_6.0.1 = &place!(Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6)).2;
_6.3.2 = (Move(_6.0.0), Move(_6.0.1), _13.0, _6.0.3);
SetDiscriminant(_11.1, 1);
_11.1 = Adt22::Variant0 { fld0: _6.3.2.0.1,fld1: _9,fld2: _10,fld3: _13.2 };
_6.3.1 = [95771576761826336171083182443146919996_u128,77192946312301008516385575643908801185_u128,1815481758644889251407177530607182840_u128,284730021761386009415824726674760848478_u128,700137982396550722859122496806041487_u128];
_22 = &_29;
_32.1 = -Field::<f64>(Variant(_11.1, 0), 0);
_23.2 = !_5.2;
_13.3 = 1185904133_u32 as u16;
SetDiscriminant(_11.1, 2);
_25 = core::ptr::addr_of_mut!(_11.0);
_30 = _20;
_7.2 = _7.3.0;
_6.3.2.0.3 = _32.3;
_23 = _5;
Call(_28 = fn7(_13.2, Move(_6.3.2.0), Move(_7.0), _23.2, Move(_6.3.0), _23.0, _23.2), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_32.1 = _31 * _31;
_6.0.0.1 = _32.1 + _31;
_23 = _5;
_6.0.0.1 = _31;
place!(Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6)).3 = _4 * _4;
_13.1 = _20 * _30;
_26 = core::ptr::addr_of_mut!(_23.3);
Goto(bb23)
}
bb23 = {
_6.3.2.2 = _6.3.2.3 & _13.0;
_6.0.2 = !_6.3.2.3;
_16 = [_5.2,_7.1,_7.1];
_6.0.0.2 = _32.2;
_23.2 = _5.2;
_37 = [_23.2,_7.1,_5.2];
place!(Field::<u8>(Variant(_11.1, 2), 1)) = !_6.0.3;
_6.3.2.1 = &_23.2;
_6.0.1 = Move(_6.3.2.1);
_4 = _28 as isize;
_6.0.2 = _6.0.3;
_7.2 = [952992258_u32,3307479575_u32,2226144473_u32,2243824963_u32,274516459_u32];
_32.1 = _12 * _6.0.0.1;
place!(Field::<f64>(Variant(_11.1, 2), 4)) = _32.1 * _6.0.0.1;
_6.3.2.0.0 = core::ptr::addr_of_mut!(_13.0);
Call(_6.3.0 = fn8(_28, _20, _17, _7.3.0, _7.3.1, _5.0, _6.0.0.2, _37, _23.0, _7.3.0, _23), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_38 = [_5.0,_5.0,_5.0,_23.0,_5.0,_23.0,_5.0];
_6.3.2.0.3 = -_28;
place!(Field::<u64>(Variant(_11.1, 2), 0)) = _32.3 as u64;
_6.0.0 = (Move(_6.3.2.0.0), Field::<f64>(Variant(_11.1, 2), 4), _32.2, _28);
_6.3.2.0.2 = _32.2;
_32.0 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_11.1, 2), 1)));
place!(Field::<i128>(Variant(_11.1, 2), 5)) = _9 & _9;
_6.0.0 = (Move(_32.0), _31, _6.3.2.0.2, _32.3);
RET = core::ptr::addr_of!(_40);
_13.3 = 61762_u16 - 18492_u16;
place!(Field::<i8>(Variant(_11.1, 2), 3)) = _17 as i8;
(*_25) = core::ptr::addr_of!((*RET));
place!(Field::<(bool, u64, usize, isize)>(Variant(_11.1, 2), 6)).1 = _10 as u64;
place!(Field::<u64>(Variant(_11.1, 2), 0)) = _17 as u64;
_6.0.0.3 = _32.3;
(*RET) = 177329334_u32;
_23.1 = _9 as u64;
_6.0.1 = &_23.2;
_11.0 = core::ptr::addr_of!((*RET));
_6.3.1 = [328546130834314121869325158461175225631_u128,181633737561283202037558477738106804275_u128,193651120210584680122745981667318143868_u128,289520710432081472055022509796841737317_u128,81670737592192949443698189416084813939_u128];
Goto(bb25)
}
bb25 = {
Call(_42 = dump_var(5_usize, 20_usize, Move(_20), 23_usize, Move(_23), 10_usize, Move(_10), 30_usize, Move(_30)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_42 = dump_var(5_usize, 3_usize, Move(_3), 16_usize, Move(_16), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u16,mut _2: (*mut u8, f64, char, i64),mut _3: [usize; 3],mut _4: [usize; 3],mut _5: *const Adt22,mut _6: usize,mut _7: isize,mut _8: isize,mut _9: usize,mut _10: u16,mut _11: u8,mut _12: u16) -> i64 {
mir! {
type RET = i64;
let _13: i32;
let _14: bool;
let _15: u16;
let _16: &'static u32;
let _17: (*mut isize, i16);
let _18: &'static i32;
let _19: bool;
let _20: ();
let _21: ();
{
RET = _2.3 ^ _2.3;
_2.3 = RET;
_10 = 4231678464_u32 as u16;
_11 = 925195493_i32 as u8;
_9 = 884837736_i32 as usize;
_2.0 = core::ptr::addr_of_mut!(_11);
_2.0 = core::ptr::addr_of_mut!(_11);
_15 = !_1;
_2.1 = 1_i8 as f64;
_15 = _12 << _2.3;
_18 = &_13;
_18 = &_13;
_17.1 = !19950_i16;
_13 = (-1376873132_i32) * (-108363920_i32);
_18 = &_13;
_4 = [_6,_6,_9];
_8 = !_7;
_2.0 = core::ptr::addr_of_mut!(_11);
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(6_usize, 15_usize, Move(_15), 3_usize, Move(_3), 11_usize, Move(_11), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(6_usize, 4_usize, Move(_4), 12_usize, Move(_12), 21_usize, _21, 21_usize, _21), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i8,mut _2: (*mut u8, f64, char, i64),mut _3: *const i64,mut _4: usize,mut _5: *const Adt22,mut _6: bool,mut _7: usize) -> i64 {
mir! {
type RET = i64;
let _8: (*const u32, Adt22, i32);
let _9: isize;
let _10: (*mut u8, f64, char, i64);
let _11: i16;
let _12: isize;
let _13: ();
let _14: ();
{
_6 = true;
_8.1 = Adt22::Variant0 { fld0: _2.1,fld1: 98406705695042996086715869852209810287_i128,fld2: (-32370_i16),fld3: _1 };
_9 = -0_isize;
place!(Field::<i128>(Variant(_8.1, 0), 1)) = 16416248418342027372787031112634178557_i128 * (-97867629408026469080137056854004924278_i128);
Goto(bb1)
}
bb1 = {
_8.2 = -(-1354652286_i32);
_2.1 = Field::<i8>(Variant(_8.1, 0), 3) as f64;
place!(Field::<i128>(Variant(_8.1, 0), 1)) = 128627807366906107958591736511081073656_i128 - (-9485596338829600957363055893835087795_i128);
_10 = (Move(_2.0), _2.1, _2.2, _2.3);
_10.1 = _7 as f64;
_2.0 = Move(_10.0);
RET = !_2.3;
place!(Field::<f64>(Variant(_8.1, 0), 0)) = _2.1 * _2.1;
place!(Field::<i128>(Variant(_8.1, 0), 1)) = 81939034840579615674012566056850334297_i128 << _9;
_3 = core::ptr::addr_of!(_2.3);
_11 = (-9506_i16);
place!(Field::<i8>(Variant(_8.1, 0), 3)) = _1;
_7 = _4;
_2.1 = -Field::<f64>(Variant(_8.1, 0), 0);
_9 = !(-9223372036854775808_isize);
_7 = _8.2 as usize;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(7_usize, 4_usize, Move(_4), 6_usize, Move(_6), 1_usize, Move(_1), 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i64,mut _2: u64,mut _3: f32,mut _4: [u32; 5],mut _5: i8,mut _6: bool,mut _7: char,mut _8: [usize; 3],mut _9: bool,mut _10: [u32; 5],mut _11: (bool, u64, usize, isize)) -> *const Adt22 {
mir! {
type RET = *const Adt22;
let _12: isize;
let _13: isize;
let _14: &'static [u16; 8];
let _15: i64;
let _16: (f64, *mut &'static u32, *const i8, *mut u8);
let _17: u128;
let _18: u8;
let _19: f32;
let _20: isize;
let _21: isize;
let _22: isize;
let _23: f64;
let _24: f32;
let _25: [bool; 7];
let _26: (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _27: *mut &'static [u16; 8];
let _28: &'static i8;
let _29: isize;
let _30: [u128; 5];
let _31: char;
let _32: u128;
let _33: i8;
let _34: isize;
let _35: Adt22;
let _36: bool;
let _37: i32;
let _38: &'static i32;
let _39: [usize; 5];
let _40: ();
let _41: ();
{
_10 = [510034773_u32,267137659_u32,2611749561_u32,1256877532_u32,416305733_u32];
_6 = _1 <= _1;
_11.2 = 29_u8 as usize;
_8 = [_11.2,_11.2,_11.2];
_11.1 = _2;
_3 = 41290_u16 as f32;
_11.2 = 17052127798644583957_usize + 10887539171195944054_usize;
_9 = !_11.0;
_11.0 = _6;
_7 = '\u{48a02}';
_6 = !_11.0;
_5 = _11.3 as i8;
_5 = 86_i8 + 32_i8;
_8 = [_11.2,_11.2,_11.2];
_11 = (_6, _2, 8097752207096694876_usize, 83_isize);
_8 = [_11.2,_11.2,_11.2];
_12 = !_11.3;
_13 = _11.3;
_9 = _11.0;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
83 => bb9,
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
_11.0 = !_6;
_6 = _13 == _11.3;
match _11.2 {
8097752207096694876 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_5 = (-82_i8);
_13 = 47647_u16 as isize;
_11.2 = !11540859946797550552_usize;
_16.2 = core::ptr::addr_of!(_5);
_7 = '\u{94662}';
match _11.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb7,
83 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_3 = _1 as f32;
_6 = _12 != _11.3;
_2 = _12 as u64;
_4 = _10;
_10 = [790803634_u32,1929880915_u32,1079158435_u32,2270182843_u32,4128211336_u32];
_8 = [_11.2,_11.2,_11.2];
_17 = 133590103961803277490710749235472288980_u128 - 298515673227320282437439611441364372828_u128;
_18 = !204_u8;
_1 = -(-529647863099957191_i64);
_4 = _10;
_9 = !_6;
_1 = (-43271240095551897030286894501308613700_i128) as i64;
_19 = _3 - _3;
_16.2 = core::ptr::addr_of!(_5);
_2 = _11.1 - _11.1;
_19 = _3;
_16.3 = core::ptr::addr_of_mut!(_18);
_16.3 = core::ptr::addr_of_mut!(_18);
_11.2 = !9665277049722759048_usize;
_18 = !226_u8;
_10 = [2678278787_u32,3842612621_u32,704050348_u32,1115892898_u32,783239059_u32];
_20 = _2 as isize;
_2 = !_11.1;
Call(_15 = fn9(_11.3, _6, _12), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11.2 = !4_usize;
_4 = [3498647994_u32,2204885961_u32,3537669863_u32,1731419527_u32,2662685201_u32];
_16.0 = (-116997730529093661674640655592889213653_i128) as f64;
_11 = (_9, _2, 5_usize, _20);
_10 = [307810773_u32,1030789476_u32,1625320747_u32,3796303212_u32,2902136929_u32];
_16.3 = core::ptr::addr_of_mut!(_18);
_2 = _11.1;
_12 = _11.3;
_22 = _11.3;
_3 = _19;
_25 = [_11.0,_11.0,_11.0,_11.0,_9,_6,_6];
Call(_26.2.0.1 = fn10(_11.2, _11, _20, _9, _9, _25, _6, _11, _2), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_3 = _19 + _19;
_26.2.0.3 = (-15850_i16) as i64;
_26.2.0.3 = _2 as i64;
_20 = _12;
_26.2.1 = &_11.2;
_26.2.0 = (Move(_16.3), _16.0, _7, _15);
_15 = !_26.2.0.3;
_26.2.0.2 = _7;
_11.2 = !1435784390611223884_usize;
_27 = core::ptr::addr_of_mut!(_14);
_23 = _18 as f64;
_26.1 = [_17,_17,_17,_17,_17];
_8 = [_11.2,_11.2,_11.2];
_24 = _20 as f32;
_3 = -_19;
_28 = &_5;
_10 = [1081792413_u32,3794452725_u32,3722137130_u32,4217133076_u32,946900560_u32];
_27 = core::ptr::addr_of_mut!((*_27));
_4 = _10;
_21 = !_11.3;
_16.3 = core::ptr::addr_of_mut!(_18);
_26.2.1 = &_11.2;
_22 = _26.2.0.2 as isize;
_25 = [_6,_11.0,_11.0,_9,_11.0,_9,_9];
_28 = &(*_28);
_4 = _10;
_11.0 = !_6;
_28 = &(*_28);
_26.1 = [_17,_17,_17,_17,_17];
_26.2.0.1 = _16.0 * _16.0;
match _5 {
0 => bb11,
1 => bb16,
2 => bb17,
3 => bb18,
4 => bb19,
340282366920938463463374607431768211374 => bb21,
_ => bb20
}
}
bb16 = {
Return()
}
bb17 = {
_3 = _1 as f32;
_6 = _12 != _11.3;
_2 = _12 as u64;
_4 = _10;
_10 = [790803634_u32,1929880915_u32,1079158435_u32,2270182843_u32,4128211336_u32];
_8 = [_11.2,_11.2,_11.2];
_17 = 133590103961803277490710749235472288980_u128 - 298515673227320282437439611441364372828_u128;
_18 = !204_u8;
_1 = -(-529647863099957191_i64);
_4 = _10;
_9 = !_6;
_1 = (-43271240095551897030286894501308613700_i128) as i64;
_19 = _3 - _3;
_16.2 = core::ptr::addr_of!(_5);
_2 = _11.1 - _11.1;
_19 = _3;
_16.3 = core::ptr::addr_of_mut!(_18);
_16.3 = core::ptr::addr_of_mut!(_18);
_11.2 = !9665277049722759048_usize;
_18 = !226_u8;
_10 = [2678278787_u32,3842612621_u32,704050348_u32,1115892898_u32,783239059_u32];
_20 = _2 as isize;
_2 = !_11.1;
Call(_15 = fn9(_11.3, _6, _12), ReturnTo(bb14), UnwindUnreachable())
}
bb18 = {
_11.0 = !_6;
_6 = _13 == _11.3;
match _11.2 {
8097752207096694876 => bb11,
_ => bb10
}
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_26.2.1 = &_11.2;
Goto(bb22)
}
bb22 = {
_26.2.1 = &_11.2;
_27 = core::ptr::addr_of_mut!((*_27));
_31 = _26.2.0.2;
_23 = _26.2.0.1 - _16.0;
_26.2.0 = (Move(_16.3), _23, _31, _15);
_1 = !_15;
_26.2.1 = &_11.2;
_26.2.2 = !_18;
_16.0 = _23;
_9 = _6 ^ _6;
_26.2.0.2 = _7;
_18 = !_26.2.2;
_4 = [1840676953_u32,1576529997_u32,2647194693_u32,391389603_u32,976214682_u32];
_11.3 = !_20;
_11.2 = !4_usize;
_16.0 = _23;
_7 = _31;
_8 = [_11.2,_11.2,_11.2];
_23 = -_26.2.0.1;
_13 = _11.3 - _21;
_11.3 = _13;
_13 = !_11.3;
_6 = _21 != _20;
_2 = _12 as u64;
_29 = -_11.3;
_11.1 = _2 | _2;
_6 = _11.3 >= _13;
_34 = _21;
Goto(bb23)
}
bb23 = {
_16.3 = core::ptr::addr_of_mut!(_26.2.3);
_26.2.0.2 = _31;
_18 = _26.2.2 + _26.2.2;
_1 = _26.2.0.2 as i64;
_11.2 = 0_usize;
_3 = _17 as f32;
_11.0 = _11.3 < _34;
RET = core::ptr::addr_of!(_35);
_26.2.0.0 = core::ptr::addr_of_mut!(_26.2.3);
_35 = Adt22::Variant0 { fld0: _16.0,fld1: (-149513937752697288630834646513108311599_i128),fld2: 31856_i16,fld3: (*_28) };
place!(Field::<f64>(Variant((*RET), 0), 0)) = -_16.0;
_37 = _11.2 as i32;
_21 = _29;
place!(Field::<i128>(Variant((*RET), 0), 1)) = (-166464647728258948173949869949371565734_i128);
_28 = &_5;
place!(Field::<i128>(Variant((*RET), 0), 1)) = (-13094986840074086099068979976851680245_i128);
_15 = _26.2.0.3;
_26.2.3 = !_18;
_26.2.0.2 = _7;
_30 = _26.1;
_26.2.3 = Field::<i128>(Variant((*RET), 0), 1) as u8;
RET = core::ptr::addr_of!(_35);
place!(Field::<i16>(Variant((*RET), 0), 2)) = !24651_i16;
place!(Field::<i128>(Variant((*RET), 0), 1)) = 49620379565720143692337614755655217137_i128;
_26.2.0.0 = Move(_16.3);
_16.0 = _23 + Field::<f64>(Variant(_35, 0), 0);
Goto(bb24)
}
bb24 = {
Call(_40 = dump_var(8_usize, 21_usize, Move(_21), 30_usize, Move(_30), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_40 = dump_var(8_usize, 15_usize, Move(_15), 4_usize, Move(_4), 8_usize, Move(_8), 22_usize, Move(_22)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_40 = dump_var(8_usize, 20_usize, Move(_20), 7_usize, Move(_7), 6_usize, Move(_6), 37_usize, Move(_37)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: bool,mut _3: isize) -> i64 {
mir! {
type RET = i64;
let _4: u32;
let _5: f32;
let _6: i8;
let _7: Adt81;
let _8: *mut Adt49;
let _9: &'static [u32; 5];
let _10: f64;
let _11: &'static u32;
let _12: [usize; 3];
let _13: *mut u8;
let _14: u8;
let _15: isize;
let _16: &'static usize;
let _17: (*const i64, usize, [u32; 5], ([u32; 5], i8));
let _18: i128;
let _19: (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _20: i8;
let _21: *mut (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _22: Adt44;
let _23: ([u32; 5], i8);
let _24: [usize; 5];
let _25: f64;
let _26: [usize; 7];
let _27: (*const i64, usize, [u32; 5], ([u32; 5], i8));
let _28: isize;
let _29: char;
let _30: isize;
let _31: [bool; 7];
let _32: f32;
let _33: ();
let _34: ();
{
_1 = -_3;
RET = 1822757493603393860_i64;
RET = !1323443856633657296_i64;
_3 = -_1;
RET = 2952836129331965336_i64;
RET = -(-7370664958794521559_i64);
RET = (-1570059694774501296_i64) - 8081546214914175638_i64;
RET = '\u{1ff84}' as i64;
_1 = _3 + _3;
_2 = _1 < _1;
RET = (-2138788148780593424_i64);
_3 = _1;
_4 = 1943879977_u32;
_3 = 339455139216363218706043975192552222471_u128 as isize;
_2 = false | false;
_1 = 138_u8 as isize;
_1 = !_3;
_4 = 2902429286_u32;
_3 = _1;
_4 = !479488060_u32;
_4 = 2641421767_u32 >> _3;
_3 = 27688_i16 as isize;
_5 = (-9_i8) as f32;
match RET {
0 => bb1,
1 => bb2,
340282366920938463461235819282987618032 => bb4,
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
RET = 8796132368858802472_i64 << _3;
_6 = 38_i8 >> _4;
_7.fld0 = [_4];
_7.fld0 = [_4];
_7.fld0 = [_4];
RET = 1013645314954304116_i64 ^ 8255552893139593232_i64;
_10 = 14868815170297559884_u64 as f64;
_3 = _1;
_10 = 13716820784300240334_usize as f64;
_4 = _10 as u32;
_6 = 8_i8;
RET = _6 as i64;
_6 = (-121_i8);
_13 = core::ptr::addr_of_mut!(_14);
_3 = _1;
_13 = core::ptr::addr_of_mut!((*_13));
_12 = [6318325418409852350_usize,5766336156573193148_usize,1_usize];
_11 = &_4;
_13 = core::ptr::addr_of_mut!((*_13));
(*_13) = 208_u8;
_13 = core::ptr::addr_of_mut!(_14);
_12 = [6096781888337465932_usize,4218488468830028832_usize,4179593031449555513_usize];
match (*_13) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
208 => bb9,
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
_12 = [13137796786834539819_usize,15377787551388842980_usize,7_usize];
RET = (-3635345617645276716_i64);
_10 = 1707610104150346193_u64 as f64;
RET = 61879_u16 as i64;
match _6 {
0 => bb8,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211335 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_11 = &(*_11);
_17.0 = core::ptr::addr_of!(RET);
_17.3.0 = [(*_11),(*_11),(*_11),(*_11),_4];
_17.1 = 12390566685687847061_usize * 3_usize;
_17.3.0 = [(*_11),(*_11),_4,_4,_4];
_18 = 113716326539853646747905292097242159042_i128;
_16 = &_17.1;
RET = 20008_u16 as i64;
(*_13) = !54_u8;
_11 = &(*_11);
_3 = -_1;
_2 = !false;
_19.2.0.3 = _2 as i64;
_19.2.0 = (Move(_13), _10, '\u{7a55f}', RET);
_12 = [(*_16),(*_16),(*_16)];
_3 = _6 as isize;
_18 = !78919899494434086375532919463774495849_i128;
_20 = _6 + _6;
_13 = core::ptr::addr_of_mut!(_19.2.2);
_4 = 949451606_u32;
RET = !_19.2.0.3;
_19.2.0.3 = 2376620040283397940347482314540850231_u128 as i64;
_19.1 = [5851339338097114099777554217855613199_u128,137072565542342722067762283408417996225_u128,92950295457751978401426209403774283677_u128,167569526539401777393632159424475060147_u128,233389244717871790926448374418202684174_u128];
Goto(bb12)
}
bb12 = {
_13 = Move(_19.2.0.0);
_17.0 = core::ptr::addr_of!(_19.2.0.3);
_17.3.1 = !_6;
_18 = -(-89875006819454799440287601123830535409_i128);
_4 = 2101689353_u32 >> _6;
_17.3.0 = [_4,_4,_4,_4,_4];
_17.0 = core::ptr::addr_of!(RET);
_12 = [(*_16),_17.1,_17.1];
_6 = _20;
_10 = _19.2.0.1 * _19.2.0.1;
_17.2 = [_4,_4,_4,_4,_4];
_19.2.3 = !_14;
_19.2.2 = _14;
_24 = [(*_16),(*_16),(*_16),_17.1,(*_16)];
_19.2.1 = Move(_16);
_17.2 = [_4,_4,_4,_4,_4];
_19.2.1 = &_17.1;
_16 = Move(_19.2.1);
_23 = (_17.3.0, _6);
_17.2 = [_4,_4,_4,_4,_4];
Goto(bb13)
}
bb13 = {
_19.2.1 = &_17.1;
_23 = _17.3;
_19.2.3 = _19.2.2;
_19.2.0.2 = '\u{7d623}';
_19.2.0.0 = Move(_13);
_11 = &_4;
Goto(bb14)
}
bb14 = {
_12 = [_17.1,_17.1,_17.1];
_27 = Move(_17);
_11 = &(*_11);
_17.1 = _27.1 & _27.1;
RET = _19.2.0.3 >> _4;
_27.0 = core::ptr::addr_of!(_19.2.0.3);
_17.3.1 = -_20;
_27.3 = (_23.0, _23.1);
_17.0 = Move(_27.0);
_24 = [_27.1,_17.1,_17.1,_27.1,_27.1];
_14 = _19.2.2;
_27.3 = (_27.2, _20);
_25 = _10;
_26 = [_27.1,_17.1,_17.1,_27.1,_17.1,_17.1,_17.1];
_11 = &(*_11);
_16 = &_27.1;
_19.2.3 = 25582025825221290785754694292025678895_u128 as u8;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(9_usize, 2_usize, Move(_2), 3_usize, Move(_3), 12_usize, Move(_12), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(9_usize, 14_usize, Move(_14), 18_usize, Move(_18), 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: usize,mut _2: (bool, u64, usize, isize),mut _3: isize,mut _4: bool,mut _5: bool,mut _6: [bool; 7],mut _7: bool,mut _8: (bool, u64, usize, isize),mut _9: u64) -> f64 {
mir! {
type RET = f64;
let _10: [i8; 2];
let _11: &'static *mut u128;
let _12: i64;
let _13: usize;
let _14: [i8; 2];
let _15: i128;
let _16: (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _17: i32;
let _18: ([u32; 5], i8);
let _19: u64;
let _20: isize;
let _21: Adt22;
let _22: &'static bool;
let _23: u8;
let _24: [usize; 6];
let _25: [i8; 2];
let _26: f32;
let _27: char;
let _28: (u8, u64, i8, u16);
let _29: u32;
let _30: u128;
let _31: i8;
let _32: (u8, u64, i8, u16);
let _33: bool;
let _34: *mut isize;
let _35: (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _36: &'static &'static bool;
let _37: [u32; 3];
let _38: *const u8;
let _39: &'static Adt22;
let _40: i128;
let _41: isize;
let _42: f32;
let _43: (u8, u64, i8, u16);
let _44: ();
let _45: ();
{
_7 = !_6[_1];
RET = 166648142144067870463148814308321130101_u128 as f64;
_2.3 = _3;
_8 = (_4, _9, _2.2, _3);
_6 = [_7,_7,_8.0,_8.0,_7,_8.0,_4];
_2 = (_4, _8.1, _8.2, _3);
_8 = _2;
_8.3 = _2.3;
_8 = _2;
_2 = _8;
_3 = 53674_u16 as isize;
_8.3 = _1 as isize;
_2.0 = _7;
_3 = _8.3;
_1 = 303270876736350796817902270989158334220_u128 as usize;
_3 = 121_i8 as isize;
_10 = [51_i8,(-118_i8)];
_6 = [_4,_2.0,_5,_2.0,_5,_5,_4];
Goto(bb1)
}
bb1 = {
_2.2 = _8.2 - _8.2;
_8.3 = _2.3;
_5 = !_7;
_10 = [41_i8,76_i8];
_8.0 = !_2.0;
_2.3 = _8.3;
_6 = [_4,_8.0,_5,_7,_7,_5,_8.0];
_9 = 81_i8 as u64;
_12 = !4780380622961636802_i64;
_14 = [21_i8,(-106_i8)];
_1 = _5 as usize;
_10 = _14;
_8.2 = _1 ^ _1;
_2.3 = _8.3;
_5 = _2.0;
_8.2 = _2.2 ^ _2.2;
_7 = !_8.0;
_14 = [(-78_i8),57_i8];
_7 = !_2.0;
_8.0 = _5;
_8 = (_5, _9, _2.2, _2.3);
_2.0 = _8.0;
_2.3 = _8.3 * _8.3;
_2.0 = _5;
_5 = !_2.0;
Goto(bb2)
}
bb2 = {
_4 = _8.0;
_10 = [(-54_i8),(-74_i8)];
_4 = _7 > _2.0;
_8.3 = !_2.3;
_2 = (_8.0, _8.1, _1, _8.3);
_2.0 = _5 | _5;
_16.1.0.1 = RET + RET;
_16.1.0.0 = core::ptr::addr_of_mut!(_16.1.2);
_5 = _2.0;
_16.1.0.2 = '\u{1a9c6}';
_3 = -_8.3;
_16.0 = _2.2 + _8.2;
_8.3 = _3 & _2.3;
_4 = _2.2 < _8.2;
_5 = _4;
_2.1 = !_9;
_2.1 = (-16783883867707523936407041778286980458_i128) as u64;
_10 = [76_i8,63_i8];
_16.1.0.1 = RET + RET;
_4 = !_2.0;
_9 = _2.1 >> _8.3;
Call(_7 = fn11(_1, _8.0, _2.2, _2, _3, _8.0, _8, _8, _1, _1, _2.3, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _2.1;
_3 = _8.3 - _8.3;
_1 = !_2.2;
_16.1.1 = &_8.2;
_14 = [(-19_i8),(-71_i8)];
_8.2 = _16.0 << _2.3;
_16.1.2 = _12 as u8;
_4 = !_5;
_8.1 = _8.3 as u64;
_16.1.3 = _16.1.2 << _16.0;
_18.0 = [3575871626_u32,2026406197_u32,127562518_u32,4113477784_u32,1265022947_u32];
_2.2 = !_8.2;
Goto(bb4)
}
bb4 = {
_8.1 = !_2.1;
RET = 1513031745_u32 as f64;
_16.1.1 = &_16.0;
_5 = !_7;
_16.1.1 = &_13;
Goto(bb5)
}
bb5 = {
_19 = _8.1 >> _3;
_2.1 = _19 << _8.3;
_16.1.0.3 = (-77432550960780773438038624569131429046_i128) as i64;
_16.1.1 = &_1;
_13 = !_8.2;
_20 = 463840369_i32 as isize;
_8.0 = _4;
_18.1 = 53_i8 + (-51_i8);
_2.2 = _8.2 - _13;
_16.1.3 = !_16.1.2;
_17 = 330915549_u32 as i32;
_12 = _16.1.0.3 << _3;
_21 = Adt22::Variant2 { fld0: _19,fld1: _16.1.3,fld2: _3,fld3: _18.1,fld4: _16.1.0.1,fld5: 4388601329272719989443524343305381044_i128,fld6: _2 };
Goto(bb6)
}
bb6 = {
_16.1.3 = !Field::<u8>(Variant(_21, 2), 1);
_3 = Field::<isize>(Variant(_21, 2), 2);
place!(Field::<i128>(Variant(_21, 2), 5)) = (-29626628117480604385703269183342958718_i128);
_10 = [Field::<i8>(Variant(_21, 2), 3),Field::<i8>(Variant(_21, 2), 3)];
_2.0 = _8.0;
_15 = _8.0 as i128;
place!(Field::<u64>(Variant(_21, 2), 0)) = !_2.1;
_16.1.0.1 = -Field::<f64>(Variant(_21, 2), 4);
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).2 = !_2.2;
_16.1.0.2 = '\u{71b97}';
_20 = Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).3 & _8.3;
RET = _16.1.3 as f64;
_8.3 = !Field::<isize>(Variant(_21, 2), 2);
_24 = [_13,_13,Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).2,_2.2,_1,Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).2];
_7 = !_8.0;
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).1 = _2.1;
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).1 = _12 as u64;
_23 = _16.1.0.2 as u8;
_7 = _8.0 != _8.0;
_26 = _12 as f32;
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).2 = _13;
_1 = !_2.2;
_9 = Field::<u64>(Variant(_21, 2), 0) - _19;
Call(_16.0 = fn12(_24, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_28.3 = 3379803306_u32 as u16;
_16.1.0.0 = core::ptr::addr_of_mut!(_16.1.3);
_16.1.1 = &_1;
_8 = (_4, _19, _1, Field::<isize>(Variant(_21, 2), 2));
_18.1 = Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).3 as i8;
_4 = _8.1 == _2.1;
SetDiscriminant(_21, 1);
_2.3 = _3;
_26 = _18.1 as f32;
_8 = (_5, _19, _1, _20);
_16.1.1 = &_13;
_22 = &_7;
_28.1 = !_2.1;
_20 = _8.3 >> _8.1;
place!(Field::<[u32; 5]>(Variant(_21, 1), 4)) = _18.0;
_32.0 = _16.1.3 >> _2.1;
_4 = !_2.0;
_2.3 = _17 as isize;
_19 = _28.1 + _8.1;
_23 = _16.1.0.1 as u8;
_19 = _8.1 | _2.1;
Goto(bb8)
}
bb8 = {
_20 = _8.3 >> _8.3;
_16.1.1 = &_2.2;
_28 = (_32.0, _9, _18.1, 25616_u16);
_10 = [_18.1,_28.2];
_8 = _2;
_28.0 = _32.0;
_8.2 = _13 << _9;
_2 = (_5, _9, _8.2, _20);
_32.3 = _28.3 >> _12;
Goto(bb9)
}
bb9 = {
_34 = core::ptr::addr_of_mut!(_3);
_31 = _19 as i8;
_35.1.0.1 = _16.1.0.1;
_28.1 = _2.1 ^ _19;
_17 = (-1695966455_i32) * 1874603977_i32;
_2.2 = _7 as usize;
_10 = _14;
_33 = !_5;
place!(Field::<u128>(Variant(_21, 1), 2)) = _19 as u128;
_35.1.0.3 = -_12;
_21 = Adt22::Variant0 { fld0: _35.1.0.1,fld1: _15,fld2: 17680_i16,fld3: _28.2 };
_32.0 = _28.0;
_32.0 = _15 as u8;
_35.1.0.2 = _16.1.0.2;
_2.0 = _9 != _2.1;
_8.1 = _9 * _9;
_25 = _14;
_29 = _15 as u32;
_22 = &(*_22);
place!(Field::<i16>(Variant(_21, 0), 2)) = (-22176_i16);
_35.1.2 = Field::<i8>(Variant(_21, 0), 3) as u8;
_20 = _17 as isize;
_21 = Adt22::Variant0 { fld0: RET,fld1: _15,fld2: (-30934_i16),fld3: _31 };
_22 = &_33;
_22 = &_5;
place!(Field::<i8>(Variant(_21, 0), 3)) = _28.2 >> _35.1.0.3;
_32.2 = _29 as i8;
Goto(bb10)
}
bb10 = {
_38 = core::ptr::addr_of!(_35.1.3);
_16.1.3 = !_28.0;
_22 = &_33;
_23 = !_28.0;
_16.1.0.3 = _3 as i64;
_26 = _12 as f32;
_36 = &_22;
_16.1.2 = _28.0 << Field::<i128>(Variant(_21, 0), 1);
(*_38) = !_16.1.2;
place!(Field::<i8>(Variant(_21, 0), 3)) = _32.3 as i8;
_35.1.0.2 = _16.1.0.2;
_1 = _13;
Call(_35.1.2 = core::intrinsics::transmute(_23), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_29 = _2.3 as u32;
_35.0 = !_1;
RET = Field::<f64>(Variant(_21, 0), 0);
_29 = 4201439836_u32;
(*_34) = -_8.3;
_1 = _35.0;
_41 = _2.3;
match _28.3 {
0 => bb7,
1 => bb2,
2 => bb4,
3 => bb12,
25616 => bb14,
_ => bb13
}
}
bb12 = {
_20 = _8.3 >> _8.3;
_16.1.1 = &_2.2;
_28 = (_32.0, _9, _18.1, 25616_u16);
_10 = [_18.1,_28.2];
_8 = _2;
_28.0 = _32.0;
_8.2 = _13 << _9;
_2 = (_5, _9, _8.2, _20);
_32.3 = _28.3 >> _12;
Goto(bb9)
}
bb13 = {
_16.1.3 = !Field::<u8>(Variant(_21, 2), 1);
_3 = Field::<isize>(Variant(_21, 2), 2);
place!(Field::<i128>(Variant(_21, 2), 5)) = (-29626628117480604385703269183342958718_i128);
_10 = [Field::<i8>(Variant(_21, 2), 3),Field::<i8>(Variant(_21, 2), 3)];
_2.0 = _8.0;
_15 = _8.0 as i128;
place!(Field::<u64>(Variant(_21, 2), 0)) = !_2.1;
_16.1.0.1 = -Field::<f64>(Variant(_21, 2), 4);
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).2 = !_2.2;
_16.1.0.2 = '\u{71b97}';
_20 = Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).3 & _8.3;
RET = _16.1.3 as f64;
_8.3 = !Field::<isize>(Variant(_21, 2), 2);
_24 = [_13,_13,Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).2,_2.2,_1,Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6).2];
_7 = !_8.0;
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).1 = _2.1;
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).1 = _12 as u64;
_23 = _16.1.0.2 as u8;
_7 = _8.0 != _8.0;
_26 = _12 as f32;
place!(Field::<(bool, u64, usize, isize)>(Variant(_21, 2), 6)).2 = _13;
_1 = !_2.2;
_9 = Field::<u64>(Variant(_21, 2), 0) - _19;
Call(_16.0 = fn12(_24, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb14 = {
_32 = (_35.1.3, _28.1, _31, _28.3);
_42 = -_26;
_16.1.1 = &_8.2;
_15 = _16.1.0.1 as i128;
_35.1.0 = (Move(_16.1.0.0), _16.1.0.1, _16.1.0.2, _12);
_10 = [_32.2,_32.2];
(*_34) = _2.3;
_16.1.0.0 = Move(_35.1.0.0);
_43.0 = _23 - _35.1.3;
_35.0 = !_2.2;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(10_usize, 17_usize, Move(_17), 24_usize, Move(_24), 31_usize, Move(_31), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(10_usize, 4_usize, Move(_4), 10_usize, Move(_10), 12_usize, Move(_12), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(10_usize, 25_usize, Move(_25), 28_usize, Move(_28), 29_usize, Move(_29), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(10_usize, 32_usize, Move(_32), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: usize,mut _2: bool,mut _3: usize,mut _4: (bool, u64, usize, isize),mut _5: isize,mut _6: bool,mut _7: (bool, u64, usize, isize),mut _8: (bool, u64, usize, isize),mut _9: usize,mut _10: usize,mut _11: isize,mut _12: (bool, u64, usize, isize)) -> bool {
mir! {
type RET = bool;
let _13: u32;
let _14: ();
let _15: ();
{
_4.3 = _12.3 & _7.3;
_12.1 = _4.1 * _7.1;
_7.1 = !_12.1;
_4.2 = _12.2;
_13 = (-91_i8) as u32;
_4 = _12;
_7.3 = _10 as isize;
RET = _2;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(11_usize, 2_usize, Move(_2), 4_usize, Move(_4), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(11_usize, 9_usize, Move(_9), 13_usize, Move(_13), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [usize; 6],mut _2: isize) -> usize {
mir! {
type RET = usize;
let _3: *const u8;
let _4: *mut *mut &'static [u16; 8];
let _5: &'static i8;
let _6: u16;
let _7: &'static *mut u128;
let _8: f64;
let _9: [u8; 8];
let _10: i128;
let _11: f64;
let _12: &'static u32;
let _13: ();
let _14: ();
{
RET = 7054651569883533749_i64 as usize;
_2 = 9223372036854775807_isize >> RET;
RET = !7_usize;
RET = !5373897178669554912_usize;
Goto(bb1)
}
bb1 = {
RET = 3_usize;
RET = _1[RET] * _1[RET];
_2 = !9223372036854775807_isize;
_2 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = !10786804639551661372_usize;
_2 = 9223372036854775807_isize;
_2 = (-41_isize);
_2 = false as isize;
RET = 1657408240233599873_usize | 2_usize;
RET = (-123277449862844679489029923219456873468_i128) as usize;
_1 = [RET,RET,RET,RET,RET,RET];
Call(_3 = fn13(RET, RET, _1, _2, _1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 229_i16 as usize;
Goto(bb3)
}
bb3 = {
RET = 11701343974850211508_usize;
_1 = [RET,RET,RET,RET,RET,RET];
_2 = 35_isize;
_2 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
RET = '\u{9a1de}' as usize;
RET = 5499475669269270676_usize;
_1 = [RET,RET,RET,RET,RET,RET];
RET = 3_usize >> _2;
_1 = [RET,RET,RET,RET,RET,RET];
_1 = [RET,RET,RET,RET,RET,RET];
_2 = !(-9223372036854775808_isize);
RET = 4_usize ^ 7377193374380780257_usize;
_2 = (-9223372036854775808_isize);
RET = 3_usize - 7_usize;
_2 = (-9223372036854775808_isize);
_1 = [RET,RET,RET,RET,RET,RET];
RET = 11462037364251686093_usize | 7876669612865985861_usize;
_2 = (-59_isize) | 9223372036854775807_isize;
_1 = [RET,RET,RET,RET,RET,RET];
RET = !4_usize;
Goto(bb4)
}
bb4 = {
_2 = (-35_isize) - (-9223372036854775808_isize);
_6 = 65498_u16 + 48290_u16;
RET = 1_usize;
_1[RET] = !RET;
_2 = 8986476296112094665_i64 as isize;
_6 = !29417_u16;
_6 = 50973_u16;
_2 = 4_isize & (-9223372036854775808_isize);
_6 = 4954_u16 - 1156_u16;
_1[RET] = _6 as usize;
_2 = (-9223372036854775808_isize) + (-2_isize);
RET = _1[RET];
RET = 7738226752648764658_usize;
match RET {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
7738226752648764658 => bb13,
_ => bb12
}
}
bb5 = {
RET = 11701343974850211508_usize;
_1 = [RET,RET,RET,RET,RET,RET];
_2 = 35_isize;
_2 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
RET = '\u{9a1de}' as usize;
RET = 5499475669269270676_usize;
_1 = [RET,RET,RET,RET,RET,RET];
RET = 3_usize >> _2;
_1 = [RET,RET,RET,RET,RET,RET];
_1 = [RET,RET,RET,RET,RET,RET];
_2 = !(-9223372036854775808_isize);
RET = 4_usize ^ 7377193374380780257_usize;
_2 = (-9223372036854775808_isize);
RET = 3_usize - 7_usize;
_2 = (-9223372036854775808_isize);
_1 = [RET,RET,RET,RET,RET,RET];
RET = 11462037364251686093_usize | 7876669612865985861_usize;
_2 = (-59_isize) | 9223372036854775807_isize;
_1 = [RET,RET,RET,RET,RET,RET];
RET = !4_usize;
Goto(bb4)
}
bb6 = {
RET = 229_i16 as usize;
Goto(bb3)
}
bb7 = {
RET = 3_usize;
RET = _1[RET] * _1[RET];
_2 = !9223372036854775807_isize;
_2 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = !10786804639551661372_usize;
_2 = 9223372036854775807_isize;
_2 = (-41_isize);
_2 = false as isize;
RET = 1657408240233599873_usize | 2_usize;
RET = (-123277449862844679489029923219456873468_i128) as usize;
_1 = [RET,RET,RET,RET,RET,RET];
Call(_3 = fn13(RET, RET, _1, _2, _1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
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
_6 = 53904_u16;
_1 = [RET,RET,RET,RET,RET,RET];
_2 = (-3337797545976084804_i64) as isize;
_2 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_6 = 50897_u16;
RET = 6_usize >> _2;
RET = !1_usize;
RET = !6_usize;
_6 = 81_i8 as u16;
_8 = 1929660765_i32 as f64;
_6 = 27006_u16 ^ 53172_u16;
_6 = 33638_u16;
_2 = (-3723708437339535381_i64) as isize;
RET = 6305537124768943444_usize;
_1 = [RET,RET,RET,RET,RET,RET];
RET = (-65_i8) as usize;
RET = !11482339210806361884_usize;
RET = 5555341172878261698_usize >> _6;
_1 = [RET,RET,RET,RET,RET,RET];
_8 = 1109099380_u32 as f64;
RET = !739042347008427934_usize;
_10 = 89157563826477368977752267144926807256_i128 - 158391122345181734707860438471214784666_i128;
_6 = 32071_i16 as u16;
_6 = 48674_u16 + 56893_u16;
_9 = [124_u8,65_u8,144_u8,186_u8,192_u8,137_u8,204_u8,76_u8];
Goto(bb14)
}
bb14 = {
_11 = -_8;
Goto(bb15)
}
bb15 = {
Call(_13 = dump_var(12_usize, 2_usize, Move(_2), 9_usize, Move(_9), 14_usize, _14, 14_usize, _14), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: usize,mut _2: usize,mut _3: [usize; 6],mut _4: isize,mut _5: [usize; 6],mut _6: [usize; 6],mut _7: isize) -> *const u8 {
mir! {
type RET = *const u8;
let _8: *mut *mut &'static [u16; 8];
let _9: u32;
let _10: f32;
let _11: u64;
let _12: *mut &'static u32;
let _13: Adt81;
let _14: i128;
let _15: &'static *mut &'static [u16; 8];
let _16: [u32; 1];
let _17: &'static [u32; 5];
let _18: f32;
let _19: *mut u128;
let _20: [i8; 2];
let _21: isize;
let _22: ((*mut u8, f64, char, i64), &'static usize, u8, u8);
let _23: f64;
let _24: ([u32; 5], i8);
let _25: &'static *mut u128;
let _26: Adt75;
let _27: *const u32;
let _28: [u8; 8];
let _29: f64;
let _30: [usize; 3];
let _31: usize;
let _32: char;
let _33: (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _34: *mut *mut &'static [u16; 8];
let _35: i128;
let _36: f32;
let _37: Adt75;
let _38: i32;
let _39: ((bool, u64, usize, isize),);
let _40: u8;
let _41: u16;
let _42: &'static *mut u128;
let _43: ((bool, u64, usize, isize),);
let _44: char;
let _45: i32;
let _46: [i8; 8];
let _47: [bool; 7];
let _48: [u16; 8];
let _49: &'static *mut &'static [u16; 8];
let _50: Adt44;
let _51: ();
let _52: ();
{
_4 = !_7;
_7 = !_4;
_7 = !_4;
_5 = [_1,_2,_2,_2,_1,_1];
_1 = 191_u8 as usize;
_6 = [_1,_2,_1,_1,_1,_1];
_9 = 276303019_u32 << _7;
_3 = [_2,_1,_1,_1,_2,_2];
Goto(bb1)
}
bb1 = {
_9 = !1979960670_u32;
_7 = !_4;
_1 = !_2;
_3 = [_1,_1,_1,_1,_1,_2];
_6 = [_1,_2,_2,_2,_1,_1];
_7 = _4;
_1 = _2;
Goto(bb2)
}
bb2 = {
_11 = 4545328543571547468_u64 << _9;
_9 = 972541952_u32 << _7;
_10 = 90_i8 as f32;
_10 = (-2183949651820817981_i64) as f32;
_3 = [_1,_2,_2,_2,_1,_2];
_2 = _10 as usize;
_10 = (-2648_i16) as f32;
_13.fld0 = [_9];
Call(_14 = core::intrinsics::bswap((-14491444901784437447162252123342085037_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = !_7;
_14 = _10 as i128;
_7 = _4;
_11 = 18131059073912531613_u64;
_3 = [_1,_1,_1,_1,_2,_1];
_10 = 144_u8 as f32;
_5 = [_2,_2,_1,_1,_2,_1];
_3 = [_2,_1,_2,_2,_2,_1];
_9 = 1053404223_u32;
_3 = [_1,_2,_2,_1,_1,_1];
_7 = _4;
_2 = _1;
_4 = _7;
_6 = [_2,_2,_1,_1,_1,_1];
_14 = '\u{49469}' as i128;
_2 = (-4667732803449033720_i64) as usize;
_3 = [_2,_2,_2,_2,_1,_1];
_16 = [_9];
_13 = Adt81 { fld0: _16 };
_6 = _5;
_11 = 9067304285912581940_u64;
_5 = [_1,_1,_2,_2,_2,_2];
_16 = [_9];
_16 = [_9];
_5 = [_2,_1,_2,_2,_1,_1];
_16 = _13.fld0;
_3 = [_1,_1,_2,_1,_1,_2];
_9 = !4016756274_u32;
match _11 {
0 => bb1,
9067304285912581940 => bb4,
_ => bb2
}
}
bb4 = {
_7 = !_4;
_11 = !14257167710990883703_u64;
_11 = 5777979327800466917_u64 & 14690579351860287711_u64;
_2 = !_1;
_6 = [_2,_1,_2,_2,_2,_1];
_13 = Adt81 { fld0: _16 };
_21 = _4 ^ _4;
_21 = 967496534_i32 as isize;
_9 = (-433622114_i32) as u32;
_21 = _4;
_6 = [_1,_1,_2,_1,_1,_1];
_9 = _11 as u32;
_4 = -_21;
_13 = Adt81 { fld0: _16 };
_6 = [_2,_2,_2,_2,_1,_2];
_18 = _10 + _10;
_14 = _1 as i128;
_20 = [(-16_i8),100_i8];
_3 = _5;
_20 = [(-125_i8),(-76_i8)];
_14 = 30337501091637260034821872610769055404_i128;
_1 = _2 >> _2;
_2 = false as usize;
Call(_6 = fn14(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_21 = _4;
_7 = false as isize;
_7 = -_21;
_1 = !_2;
_22.0.1 = 17551449147980808343568265840391526392_u128 as f64;
_22.0.2 = '\u{359f4}';
_13 = Adt81 { fld0: _16 };
_4 = _21 + _7;
_22.2 = _22.0.2 as u8;
_22.3 = _22.2;
_17 = &_24.0;
_22.3 = _22.2 << _22.2;
_13.fld0 = _16;
_22.1 = &_2;
_2 = _1;
RET = core::ptr::addr_of!(_22.3);
match _14 {
30337501091637260034821872610769055404 => bb7,
_ => bb6
}
}
bb6 = {
_9 = !1979960670_u32;
_7 = !_4;
_1 = !_2;
_3 = [_1,_1,_1,_1,_1,_2];
_6 = [_1,_2,_2,_2,_1,_1];
_7 = _4;
_1 = _2;
Goto(bb2)
}
bb7 = {
_1 = 62757_u16 as usize;
(*RET) = _22.2 - _22.2;
_22.3 = !_22.2;
_24.0 = [_9,_9,_9,_9,_9];
_22.0.2 = '\u{c03a9}';
_21 = _4 + _4;
_21 = _7;
_26.fld6 = _11;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld5);
_26.fld4.3 = (-134410760211838448_i64) | 2453348601308750964_i64;
_16 = _13.fld0;
RET = core::ptr::addr_of!(_22.2);
_25 = &_19;
_1 = !_2;
_24.0 = [_9,_9,_9,_9,_9];
_22.0.3 = _26.fld4.3 & _26.fld4.3;
_14 = 129580214649519059039092498430654688958_i128;
_26.fld4.1 = _22.0.1;
_26.fld1 = _14 >> _9;
_25 = &_19;
_1 = _2 & _2;
_17 = &_24.0;
_26.fld4.2 = _22.0.2;
Goto(bb8)
}
bb8 = {
_26.fld4.3 = _22.0.3 * _22.0.3;
_26.fld2.2 = -(-133891570_i32);
_18 = _10 * _10;
_30 = [_1,_2,_1];
_20 = [30_i8,40_i8];
_20 = [63_i8,(-30_i8)];
_26.fld0 = Adt44::Variant0 { fld0: _9,fld1: Move(RET),fld2: (*RET),fld3: _30 };
_6 = _5;
_26.fld4.2 = _22.0.2;
_24.0 = [_9,Field::<u32>(Variant(_26.fld0, 0), 0),_9,_9,Field::<u32>(Variant(_26.fld0, 0), 0)];
_33.2.3 = _22.2;
Goto(bb9)
}
bb9 = {
_26.fld6 = !_11;
RET = Move(Field::<*const u8>(Variant(_26.fld0, 0), 1));
_26.fld3 = core::ptr::addr_of_mut!(_26.fld5);
_26.fld2.0 = core::ptr::addr_of!(_9);
_22.0.2 = _26.fld4.2;
_22.0.0 = core::ptr::addr_of_mut!(_33.2.3);
place!(Field::<u8>(Variant(_26.fld0, 0), 2)) = !_22.2;
_18 = 38505_u16 as f32;
_29 = _33.2.3 as f64;
_26.fld6 = _11 + _11;
_35 = _14 + _26.fld1;
place!(Field::<*const u8>(Variant(_26.fld0, 0), 1)) = Move(RET);
_14 = !_35;
Goto(bb10)
}
bb10 = {
_26.fld4.3 = _22.0.3 - _22.0.3;
_22.2 = Field::<u8>(Variant(_26.fld0, 0), 2) - Field::<u8>(Variant(_26.fld0, 0), 2);
_18 = _10 - _10;
_26.fld4.0 = Move(_22.0.0);
_10 = _18;
_27 = Move(_26.fld2.0);
_26.fld4.1 = _22.0.1;
_3 = [_1,_1,_1,_2,_2,_2];
place!(Field::<u32>(Variant(_26.fld0, 0), 0)) = _9;
_37.fld2.2 = _26.fld2.2;
_24.0 = [Field::<u32>(Variant(_26.fld0, 0), 0),_9,_9,_9,_9];
Goto(bb11)
}
bb11 = {
_23 = _4 as f64;
_30 = [_1,_2,_2];
_37.fld4 = (Move(_26.fld4.0), _26.fld4.1, _22.0.2, _26.fld4.3);
_39.0.0 = _37.fld4.1 >= _26.fld4.1;
_26.fld4 = Move(_37.fld4);
_39.0.3 = 186803949358049432116719076998170409185_u128 as isize;
_37.fld0 = Adt44::Variant1 { fld0: _26.fld6 };
_26.fld2.2 = _37.fld2.2 << _22.0.3;
_32 = _22.0.2;
_32 = _26.fld4.2;
_13 = Adt81 { fld0: _16 };
_33.2.2 = _33.2.3 >> _2;
_33.2.1 = &_39.0.2;
Goto(bb12)
}
bb12 = {
_33.2.0.3 = _22.0.3 >> _11;
_37.fld4.2 = _26.fld4.2;
SetDiscriminant(_37.fld0, 0);
_33.0 = core::ptr::addr_of!(_37.fld2.1);
_9 = !Field::<u32>(Variant(_26.fld0, 0), 0);
_10 = 94_i8 as f32;
_38 = _26.fld2.2 << _33.2.0.3;
_37.fld3 = core::ptr::addr_of_mut!(_26.fld5);
_37.fld4 = (Move(_26.fld4.0), _22.0.1, _26.fld4.2, _33.2.0.3);
_37.fld1 = !_14;
_22.0.0 = Move(_37.fld4.0);
_40 = !_33.2.3;
_9 = !Field::<u32>(Variant(_26.fld0, 0), 0);
SetDiscriminant(_26.fld0, 1);
_22.0.0 = core::ptr::addr_of_mut!(_33.2.3);
_39.0.0 = !true;
_26.fld2.0 = Move(_27);
_5 = _6;
_10 = _18;
_35 = -_14;
_33.2.0.0 = core::ptr::addr_of_mut!(_40);
_39.0.1 = 17661_u16 as u64;
_26.fld4 = Move(_22.0);
_10 = _18;
_26.fld4.2 = _32;
_17 = &_24.0;
Goto(bb13)
}
bb13 = {
_11 = !_39.0.1;
_22.0 = (Move(_33.2.0.0), _23, _32, _33.2.0.3);
_37.fld0 = Adt44::Variant1 { fld0: _26.fld6 };
_1 = _2 + _2;
_33.2.0.1 = -_23;
_26.fld3 = core::ptr::addr_of_mut!(_26.fld5);
_24.1 = 42_i8;
_22.1 = &_2;
Goto(bb14)
}
bb14 = {
_13 = Adt81 { fld0: _16 };
_41 = 18442_u16;
_38 = !_26.fld2.2;
_28 = [_40,_40,_40,_22.2,_22.3,_33.2.2,_33.2.2,_22.2];
_33.2.0.1 = (-9777_i16) as f64;
_33.1 = [78980593989728150488494349561492588573_u128,99242831683388438306085682889021802422_u128,221859857431613847516241904582646367344_u128,305004023206469246110583268350297300354_u128,103105230954719984253919805153383167003_u128];
_37.fld6 = _26.fld6;
_17 = &(*_17);
SetDiscriminant(_37.fld0, 1);
_22.0 = (Move(_26.fld4.0), _23, _32, _33.2.0.3);
_33.2.0.3 = -_26.fld4.3;
RET = core::ptr::addr_of!(_33.2.2);
_40 = (*RET) & _22.2;
_22.2 = _33.2.2;
_46 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_37.fld1 = _35;
_43.0.3 = _4 + _7;
_27 = core::ptr::addr_of!(_9);
_26.fld2.2 = !_38;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(13_usize, 2_usize, Move(_2), 32_usize, Move(_32), 5_usize, Move(_5), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(13_usize, 11_usize, Move(_11), 30_usize, Move(_30), 6_usize, Move(_6), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(13_usize, 21_usize, Move(_21), 38_usize, Move(_38), 1_usize, Move(_1), 52_usize, _52), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14() -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _1: *const Adt22;
let _2: [i8; 2];
let _3: [u128; 5];
let _4: i128;
let _5: ([u32; 5], i8);
let _6: f64;
let _7: char;
let _8: &'static bool;
let _9: (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _10: *const i8;
let _11: char;
let _12: [usize; 6];
let _13: [usize; 7];
let _14: i16;
let _15: f32;
let _16: *const u32;
let _17: [u32; 3];
let _18: i16;
let _19: i128;
let _20: Adt22;
let _21: [u32; 1];
let _22: *const u32;
let _23: ();
let _24: ();
{
RET = [0_usize,0_usize,7_usize,0_usize,16903000556046569540_usize,1_usize];
RET = [12916489755780807440_usize,1_usize,11239562270405142362_usize,2936454933455632680_usize,5_usize,10300563742454397564_usize];
RET = [7036623617389842420_usize,2243453360645139107_usize,4_usize,7_usize,15602964772936379463_usize,17380014386635970553_usize];
RET = [3_usize,2919303447389645231_usize,4_usize,6_usize,1876143724399591936_usize,5_usize];
RET = [8670011941899864296_usize,3810290792980976407_usize,4_usize,4_usize,10335832237207301044_usize,3_usize];
RET = [5_usize,7_usize,2_usize,4018216505942859789_usize,4_usize,5_usize];
RET = [11487884771781354167_usize,13020279386129294217_usize,622604610243375459_usize,18318692930452113117_usize,2_usize,2324148656433222588_usize];
RET = [9275436004694706786_usize,15376408589465997148_usize,9240297174355096251_usize,5_usize,1_usize,0_usize];
RET = [6368942469453022858_usize,7_usize,7_usize,3360563076281492254_usize,14465441945384184880_usize,7_usize];
RET = [4_usize,17218623530004346987_usize,4_usize,18280343680763349161_usize,3_usize,6973771922677775502_usize];
RET = [0_usize,2_usize,893751292673615594_usize,17291399380907436991_usize,17003441563858192961_usize,7_usize];
Goto(bb1)
}
bb1 = {
RET = [2876702982002343261_usize,3_usize,6_usize,1696818108168325271_usize,6_usize,15253165553801284901_usize];
RET = [0_usize,12249152849329226021_usize,12998985418310965573_usize,8341245738073556644_usize,5_usize,0_usize];
RET = [5281973095153227051_usize,12874453580870820685_usize,5_usize,5_usize,3_usize,2577458546049173371_usize];
RET = [6707962213626013045_usize,16429320296807141177_usize,18227388775798229834_usize,7884997297198056384_usize,742613731063740459_usize,11629797472439582904_usize];
RET = [13786928701681162011_usize,3_usize,6_usize,6_usize,5_usize,5732499713794835262_usize];
RET = [6_usize,17369181605494398170_usize,3_usize,9234349751317119674_usize,7_usize,9546792052336068284_usize];
RET = [2_usize,2_usize,13102338355115108887_usize,0_usize,3_usize,18108132056638643799_usize];
RET = [16516955088485047656_usize,10583049048814049350_usize,3_usize,3_usize,5527814719275693839_usize,7_usize];
RET = [1_usize,4_usize,0_usize,6_usize,3_usize,14460082044640425265_usize];
RET = [14893301510671440443_usize,2852115981913386884_usize,10082326063854173745_usize,14297867369012809989_usize,0_usize,13232544190417186918_usize];
RET = [1914574589232364950_usize,9596675202936596850_usize,7_usize,6148646105830855800_usize,10394828960087575759_usize,12202533467663641099_usize];
RET = [5_usize,3_usize,14232715101183552745_usize,4838620598002577547_usize,3_usize,2858418107282586460_usize];
_3 = [83319829974681114222802144750858029395_u128,311344710078732670976272206847742181219_u128,82438365180866176648855792161931206397_u128,32488507212598365949437464990550056835_u128,72742837461319148664546981286385626362_u128];
Goto(bb2)
}
bb2 = {
RET = [16757519392765316513_usize,1_usize,4785352995584823810_usize,1_usize,15470714384933643053_usize,689428125356403708_usize];
_2 = [(-50_i8),117_i8];
_2 = [79_i8,(-81_i8)];
RET = [1_usize,10786402617259070972_usize,3_usize,724450206900962467_usize,2_usize,6_usize];
_3 = [73307486930963573749867807876450281908_u128,282564662996670430503834248469444664961_u128,10771918607815533365829667978066709252_u128,158117883111815078144120846880783113965_u128,38637420677565172025378085415246495278_u128];
RET = [5_usize,11174487095989345362_usize,6_usize,3_usize,3549788750814816301_usize,11720704544540309333_usize];
_2 = [(-27_i8),(-22_i8)];
_3 = [20724278406309367848808483251788644466_u128,126180771645514598016796758489369854932_u128,43597270694226783128807434843771864035_u128,88585170971192383349289361296208229507_u128,257336104588822683185532173738061020936_u128];
RET = [7_usize,1_usize,3_usize,1_usize,3_usize,4_usize];
RET = [2_usize,0_usize,1_usize,7400357444859750297_usize,0_usize,1439128493605758716_usize];
RET = [16553883848926010903_usize,7731074356045762123_usize,12654857499631550456_usize,14183231488409315224_usize,5_usize,4_usize];
RET = [0_usize,7_usize,8927741669936805114_usize,7211791368632032880_usize,0_usize,1225585769047941116_usize];
RET = [6_usize,0_usize,7_usize,0_usize,14378884835205744633_usize,8825819995536177447_usize];
_5.1 = 89_i8 * 112_i8;
_5.1 = -(-4_i8);
Call(_6 = fn15(RET, _3, _2, _3, _3, RET, RET, RET, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = 8864438296590004986_i64 as f64;
RET = [11055702507756986654_usize,0_usize,3_usize,12219205181996453955_usize,3_usize,9502520707484683111_usize];
RET = [6_usize,10433929732906660121_usize,7_usize,6374728153385011042_usize,1_usize,6990855987635757858_usize];
_5.0 = [3321697918_u32,453568400_u32,3931658060_u32,607859609_u32,3367567790_u32];
_9.2.2 = 187_u8 >> _5.1;
_7 = '\u{d5dc6}';
RET = [9656011415163142925_usize,6491472379206009451_usize,2836963791406690896_usize,14708648635421810672_usize,2_usize,6647185953097949577_usize];
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_9.1 = _3;
_10 = core::ptr::addr_of!(_5.1);
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_4 = (-20128342946664947795349120614311706387_i128);
(*_10) = 13814677154122036315_u64 as i8;
_9.2.0.2 = _7;
RET = [7_usize,1_usize,10139477799560140173_usize,1_usize,1_usize,17965993296245697906_usize];
_6 = (-114_isize) as f64;
(*_10) = (-24662_i16) as i8;
_6 = (-361977362_i32) as f64;
_9.2.0.1 = -_6;
_5.1 = 88_i8 * 84_i8;
_9.1 = _3;
_11 = _9.2.0.2;
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_10 = core::ptr::addr_of!((*_10));
_3 = [31318312716855836095395446349090962396_u128,250726807348882000780098489019477502892_u128,121657290438808067917554079610376055095_u128,318476225534349373795721129281485040774_u128,207790807812638560366134473865146094735_u128];
RET = [1_usize,132041031160113776_usize,0_usize,3_usize,903730690302674818_usize,4_usize];
Call(_9.2.0.2 = fn16(_11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9.2.0.2 = _11;
_5.1 = -33_i8;
_5.0 = [1035797010_u32,2695075060_u32,449255918_u32,3392385436_u32,4201164852_u32];
_6 = 6124678401675977643_i64 as f64;
_9.2.2 = 178_u8 << (*_10);
match _4 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
320154023974273515668025486817456505069 => bb8,
_ => bb7
}
}
bb5 = {
_6 = 8864438296590004986_i64 as f64;
RET = [11055702507756986654_usize,0_usize,3_usize,12219205181996453955_usize,3_usize,9502520707484683111_usize];
RET = [6_usize,10433929732906660121_usize,7_usize,6374728153385011042_usize,1_usize,6990855987635757858_usize];
_5.0 = [3321697918_u32,453568400_u32,3931658060_u32,607859609_u32,3367567790_u32];
_9.2.2 = 187_u8 >> _5.1;
_7 = '\u{d5dc6}';
RET = [9656011415163142925_usize,6491472379206009451_usize,2836963791406690896_usize,14708648635421810672_usize,2_usize,6647185953097949577_usize];
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_9.1 = _3;
_10 = core::ptr::addr_of!(_5.1);
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_4 = (-20128342946664947795349120614311706387_i128);
(*_10) = 13814677154122036315_u64 as i8;
_9.2.0.2 = _7;
RET = [7_usize,1_usize,10139477799560140173_usize,1_usize,1_usize,17965993296245697906_usize];
_6 = (-114_isize) as f64;
(*_10) = (-24662_i16) as i8;
_6 = (-361977362_i32) as f64;
_9.2.0.1 = -_6;
_5.1 = 88_i8 * 84_i8;
_9.1 = _3;
_11 = _9.2.0.2;
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_10 = core::ptr::addr_of!((*_10));
_3 = [31318312716855836095395446349090962396_u128,250726807348882000780098489019477502892_u128,121657290438808067917554079610376055095_u128,318476225534349373795721129281485040774_u128,207790807812638560366134473865146094735_u128];
RET = [1_usize,132041031160113776_usize,0_usize,3_usize,903730690302674818_usize,4_usize];
Call(_9.2.0.2 = fn16(_11), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
RET = [16757519392765316513_usize,1_usize,4785352995584823810_usize,1_usize,15470714384933643053_usize,689428125356403708_usize];
_2 = [(-50_i8),117_i8];
_2 = [79_i8,(-81_i8)];
RET = [1_usize,10786402617259070972_usize,3_usize,724450206900962467_usize,2_usize,6_usize];
_3 = [73307486930963573749867807876450281908_u128,282564662996670430503834248469444664961_u128,10771918607815533365829667978066709252_u128,158117883111815078144120846880783113965_u128,38637420677565172025378085415246495278_u128];
RET = [5_usize,11174487095989345362_usize,6_usize,3_usize,3549788750814816301_usize,11720704544540309333_usize];
_2 = [(-27_i8),(-22_i8)];
_3 = [20724278406309367848808483251788644466_u128,126180771645514598016796758489369854932_u128,43597270694226783128807434843771864035_u128,88585170971192383349289361296208229507_u128,257336104588822683185532173738061020936_u128];
RET = [7_usize,1_usize,3_usize,1_usize,3_usize,4_usize];
RET = [2_usize,0_usize,1_usize,7400357444859750297_usize,0_usize,1439128493605758716_usize];
RET = [16553883848926010903_usize,7731074356045762123_usize,12654857499631550456_usize,14183231488409315224_usize,5_usize,4_usize];
RET = [0_usize,7_usize,8927741669936805114_usize,7211791368632032880_usize,0_usize,1225585769047941116_usize];
RET = [6_usize,0_usize,7_usize,0_usize,14378884835205744633_usize,8825819995536177447_usize];
_5.1 = 89_i8 * 112_i8;
_5.1 = -(-4_i8);
Call(_6 = fn15(RET, _3, _2, _3, _3, RET, RET, RET, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = [2876702982002343261_usize,3_usize,6_usize,1696818108168325271_usize,6_usize,15253165553801284901_usize];
RET = [0_usize,12249152849329226021_usize,12998985418310965573_usize,8341245738073556644_usize,5_usize,0_usize];
RET = [5281973095153227051_usize,12874453580870820685_usize,5_usize,5_usize,3_usize,2577458546049173371_usize];
RET = [6707962213626013045_usize,16429320296807141177_usize,18227388775798229834_usize,7884997297198056384_usize,742613731063740459_usize,11629797472439582904_usize];
RET = [13786928701681162011_usize,3_usize,6_usize,6_usize,5_usize,5732499713794835262_usize];
RET = [6_usize,17369181605494398170_usize,3_usize,9234349751317119674_usize,7_usize,9546792052336068284_usize];
RET = [2_usize,2_usize,13102338355115108887_usize,0_usize,3_usize,18108132056638643799_usize];
RET = [16516955088485047656_usize,10583049048814049350_usize,3_usize,3_usize,5527814719275693839_usize,7_usize];
RET = [1_usize,4_usize,0_usize,6_usize,3_usize,14460082044640425265_usize];
RET = [14893301510671440443_usize,2852115981913386884_usize,10082326063854173745_usize,14297867369012809989_usize,0_usize,13232544190417186918_usize];
RET = [1914574589232364950_usize,9596675202936596850_usize,7_usize,6148646105830855800_usize,10394828960087575759_usize,12202533467663641099_usize];
RET = [5_usize,3_usize,14232715101183552745_usize,4838620598002577547_usize,3_usize,2858418107282586460_usize];
_3 = [83319829974681114222802144750858029395_u128,311344710078732670976272206847742181219_u128,82438365180866176648855792161931206397_u128,32488507212598365949437464990550056835_u128,72742837461319148664546981286385626362_u128];
Goto(bb2)
}
bb8 = {
_3 = _9.1;
Goto(bb9)
}
bb9 = {
_13 = [1889831114278483253_usize,949701128319482573_usize,13085466281356911723_usize,3137699455979988552_usize,5_usize,15583569556585227910_usize,8645767591285814327_usize];
_13 = [559194922526669522_usize,4_usize,16421128404600283844_usize,1_usize,4804583726975806582_usize,4_usize,3477626555042037338_usize];
(*_10) = 114_i8;
_12 = RET;
_11 = _9.2.0.2;
_9.2.0.2 = _7;
_2 = [(*_10),(*_10)];
_9.2.0.3 = -(-4475132886785415084_i64);
RET = _12;
_13 = [11601758034855119845_usize,1_usize,15679631135092130250_usize,3_usize,8143715806508362868_usize,13514126073445919617_usize,3_usize];
_9.2.0.3 = 160620348_u32 as i64;
_12 = RET;
(*_10) = !(-82_i8);
_10 = core::ptr::addr_of!(_5.1);
_14 = 42473594094289132394494851307300809484_u128 as i16;
_7 = _9.2.0.2;
RET = [17871476025237925801_usize,2998066274636538990_usize,5107410968593797877_usize,17634724461222791668_usize,4_usize,365937634206355320_usize];
Goto(bb10)
}
bb10 = {
_13 = [9440611024070667070_usize,13241369977493469862_usize,14541935413936135001_usize,374090147288351990_usize,0_usize,4_usize,16560758126338900058_usize];
_9.2.2 = !188_u8;
RET = _12;
_14 = 40636_u16 as i16;
_9.2.3 = _9.2.2 + _9.2.2;
_5.1 = 101_i8 - (-34_i8);
_15 = _9.2.0.3 as f32;
_13 = [4_usize,16907964564376095286_usize,6_usize,2_usize,14850846258428186775_usize,6_usize,7_usize];
_12 = [15433376840030127885_usize,7_usize,2878961985499733798_usize,10914779376590781710_usize,10359703804780315821_usize,5_usize];
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
_12 = [7_usize,9852138956066781634_usize,22291131647192429_usize,0_usize,6_usize,10678777480181158774_usize];
_9.1 = [165248886871559019441032018643815748136_u128,98930424405402509278499497196469592819_u128,326202346109431943044194050562837880975_u128,120309843881889514273384899538616667128_u128,120775668558554131995025248064345870055_u128];
_5.1 = (-58_i8);
_9.2.0.0 = core::ptr::addr_of_mut!(_9.2.3);
RET = _12;
_4 = (-90046857641802449915919658385578931025_i128);
Goto(bb11)
}
bb11 = {
_4 = _14 as i128;
_11 = _7;
_12 = [6979479980142383151_usize,8347850697392338599_usize,18370165845554988003_usize,1_usize,5_usize,7862683664909213986_usize];
RET = [6_usize,7221072441462185499_usize,3_usize,778658836814472727_usize,18045507223473636065_usize,6_usize];
_9.2.2 = _9.2.3 << _9.2.3;
_14 = !(-7585_i16);
_4 = _9.2.3 as i128;
_5.0 = [3460095407_u32,857357068_u32,60334519_u32,408566986_u32,2980813695_u32];
(*_10) = 126_i8;
_4 = (-159840363179315913635077040150441209483_i128);
_9.2.0.2 = _7;
_9.2.3 = _9.2.2;
_14 = 27609_i16 ^ (-22419_i16);
_9.2.3 = _9.2.2 << _4;
_10 = core::ptr::addr_of!(_5.1);
_6 = _14 as f64;
_9.2.0.1 = _6;
_2 = [_5.1,(*_10)];
_4 = 67881201852397298925566643486032310377_i128 ^ (-139876431218244812374870442788575509557_i128);
_9.2.3 = 276593832212612457426424942245774806417_u128 as u8;
Goto(bb12)
}
bb12 = {
_9.2.2 = _9.2.3;
_18 = -_14;
_9.2.0.3 = !3970120605827170101_i64;
_6 = _9.2.0.1 - _9.2.0.1;
RET = _12;
_9.2.0.2 = _7;
RET = [1761881528580358842_usize,4_usize,3320982987475900471_usize,4424592797588800779_usize,13534652882401169480_usize,4_usize];
_10 = core::ptr::addr_of!((*_10));
_9.2.3 = _9.2.0.3 as u8;
(*_10) = _7 as i8;
_4 = 23494720115961297416110044832109986014_i128 + 48307452270067341737474740625125868853_i128;
_9.2.0.3 = _9.2.2 as i64;
_18 = _15 as i16;
RET = [13583920243486744494_usize,7_usize,7_usize,9533952646583847853_usize,7_usize,6_usize];
_4 = (-160803846551996339833132339458697584583_i128);
_13 = [4552321181960683804_usize,1_usize,9585777869453789378_usize,3938378690651320721_usize,4591106525497495899_usize,4_usize,6751918626690607936_usize];
_3 = [249687906912204587423857775131463096730_u128,5769764779715497942756790280797432193_u128,247095172645368798697511400135137094409_u128,44629400559540179301465997359222904286_u128,12877403619434288488405430849031414771_u128];
_9.0 = core::ptr::addr_of!(_20);
_9.2.0.2 = _7;
_9.2.3 = _9.2.2 * _9.2.2;
_2 = [_5.1,_5.1];
_9.0 = core::ptr::addr_of!(_20);
_9.2.3 = _6 as u8;
_19 = !_4;
match _4 {
0 => bb1,
1 => bb11,
179478520368942123630242267973070626873 => bb14,
_ => bb13
}
}
bb13 = {
_13 = [1889831114278483253_usize,949701128319482573_usize,13085466281356911723_usize,3137699455979988552_usize,5_usize,15583569556585227910_usize,8645767591285814327_usize];
_13 = [559194922526669522_usize,4_usize,16421128404600283844_usize,1_usize,4804583726975806582_usize,4_usize,3477626555042037338_usize];
(*_10) = 114_i8;
_12 = RET;
_11 = _9.2.0.2;
_9.2.0.2 = _7;
_2 = [(*_10),(*_10)];
_9.2.0.3 = -(-4475132886785415084_i64);
RET = _12;
_13 = [11601758034855119845_usize,1_usize,15679631135092130250_usize,3_usize,8143715806508362868_usize,13514126073445919617_usize,3_usize];
_9.2.0.3 = 160620348_u32 as i64;
_12 = RET;
(*_10) = !(-82_i8);
_10 = core::ptr::addr_of!(_5.1);
_14 = 42473594094289132394494851307300809484_u128 as i16;
_7 = _9.2.0.2;
RET = [17871476025237925801_usize,2998066274636538990_usize,5107410968593797877_usize,17634724461222791668_usize,4_usize,365937634206355320_usize];
Goto(bb10)
}
bb14 = {
_17 = [1146526982_u32,2845416474_u32,1207879044_u32];
_4 = -_19;
_13 = [5_usize,7_usize,10334311679762647458_usize,16938279136136137446_usize,2925947200758044387_usize,1_usize,7_usize];
_9.2.0.3 = !5142319721089777854_i64;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(14_usize, 19_usize, Move(_19), 2_usize, Move(_2), 17_usize, Move(_17), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(14_usize, 13_usize, Move(_13), 12_usize, Move(_12), 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [usize; 6],mut _2: [u128; 5],mut _3: [i8; 2],mut _4: [u128; 5],mut _5: [u128; 5],mut _6: [usize; 6],mut _7: [usize; 6],mut _8: [usize; 6],mut _9: [u128; 5]) -> f64 {
mir! {
type RET = f64;
let _10: isize;
let _11: u128;
let _12: (bool, u64, usize, isize);
let _13: *const i64;
let _14: i16;
let _15: &'static u32;
let _16: (*mut isize, i16);
let _17: u64;
let _18: &'static [u32; 5];
let _19: (*const u32, Adt22, i32);
let _20: f32;
let _21: *mut isize;
let _22: f32;
let _23: &'static Adt22;
let _24: f64;
let _25: *mut *const u32;
let _26: &'static *mut u128;
let _27: usize;
let _28: u8;
let _29: (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _30: [usize; 5];
let _31: [u16; 8];
let _32: f32;
let _33: i16;
let _34: isize;
let _35: (*mut isize, i16);
let _36: bool;
let _37: bool;
let _38: ();
let _39: ();
{
RET = 5060796104806634237_i64 as f64;
_4 = _5;
_6 = [8291204712916504010_usize,2_usize,6_usize,1238418291654872929_usize,0_usize,7526151026135936910_usize];
_6 = [0_usize,4_usize,9290041351300242796_usize,2_usize,3632889344727122868_usize,6_usize];
RET = (-9223372036854775808_isize) as f64;
_12.0 = false;
_6 = _1;
Goto(bb1)
}
bb1 = {
_10 = -9223372036854775807_isize;
_6 = [8983924076502740948_usize,7_usize,12169887405662623461_usize,0_usize,17972274710688319057_usize,3_usize];
_8 = [585922157144823334_usize,8138641344889923274_usize,2_usize,3_usize,0_usize,3592344933090132369_usize];
_1 = [6_usize,3_usize,6_usize,6_usize,6_usize,5_usize];
_12.2 = 6_usize + 1_usize;
_12.3 = 121079554053566511484572357186354161146_u128 as isize;
_12.0 = RET >= RET;
_1 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
Goto(bb2)
}
bb2 = {
_3 = [41_i8,(-93_i8)];
_5 = [281147037887469068746023776569285895578_u128,281759520243974368188634338448058927701_u128,159204459619589747400255638134869750373_u128,229040321135474640451701010789745634093_u128,46914725318911037552868454209004405614_u128];
_12.1 = 7060736106557517886_u64 + 15404593949788924045_u64;
_7 = _6;
_11 = 131446099830293758512749301696021888797_u128;
_7 = _6;
_3 = [(-23_i8),110_i8];
_12.0 = false;
_10 = _12.3;
_12.3 = _10 ^ _10;
_11 = 5380379727206098568_i64 as u128;
_6 = _8;
_9 = [_11,_11,_11,_11,_11];
_14 = (-8215_i16) << _12.2;
_12.0 = true;
RET = _12.1 as f64;
RET = 77230673185002683142297194310701220301_i128 as f64;
_19.2 = (-1172521227_i32);
_12.3 = _14 as isize;
_14 = (-18178_i16);
_16.1 = _12.1 as i16;
_17 = _11 as u64;
_10 = _12.3;
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
RET = _10 as f64;
_12.0 = _12.2 >= _12.2;
_17 = _12.1;
Goto(bb3)
}
bb3 = {
_10 = _12.3 | _12.3;
_12 = (true, _17, 6_usize, _10);
_12.2 = 16628445114961294843_usize;
_10 = _12.3;
_16.0 = core::ptr::addr_of_mut!(_12.3);
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
Goto(bb4)
}
bb4 = {
RET = _10 as f64;
RET = _16.1 as f64;
_17 = _12.1 | _12.1;
_11 = 132252537683774627329951682448481403265_u128;
_2 = [_11,_11,_11,_11,_11];
_14 = _16.1;
_12.1 = _17;
_20 = _10 as f32;
_1 = _7;
RET = (-16_i8) as f64;
_12.2 = 56239_u16 as usize;
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
_21 = Move(_16.0);
_14 = _16.1 >> _11;
_10 = _12.3 << _14;
_19.2 = 5634992928042983387_i64 as i32;
RET = _10 as f64;
_7 = _6;
_10 = _12.3 & _12.3;
match _11 {
132252537683774627329951682448481403265 => bb6,
_ => bb5
}
}
bb5 = {
_10 = _12.3 | _12.3;
_12 = (true, _17, 6_usize, _10);
_12.2 = 16628445114961294843_usize;
_10 = _12.3;
_16.0 = core::ptr::addr_of_mut!(_12.3);
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
Goto(bb4)
}
bb6 = {
_16.1 = RET as i16;
_7 = _1;
RET = _20 as f64;
_6 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
_2 = _5;
_16.0 = core::ptr::addr_of_mut!(_10);
_22 = _20 * _20;
_12.3 = -_10;
Goto(bb7)
}
bb7 = {
_10 = -_12.3;
_12.1 = !_17;
_5 = _4;
_12.1 = _17;
_12.2 = 5_usize;
_24 = -RET;
_8 = _7;
_12.2 = (-7003067291661921183_i64) as usize;
_12.1 = _14 as u64;
_5 = [_11,_11,_11,_11,_11];
_29.1.0.0 = core::ptr::addr_of_mut!(_29.1.3);
_10 = '\u{65844}' as isize;
_12.2 = 2_usize >> _12.3;
_23 = &_19.1;
_29.1.3 = 110_u8 - 66_u8;
_13 = core::ptr::addr_of!(_29.1.0.3);
_29.0 = _12.2 | _12.2;
_29.1.2 = _12.0 as u8;
_21 = core::ptr::addr_of_mut!(_10);
_16.1 = _14 * _14;
_29.1.2 = _16.1 as u8;
(*_13) = !(-2325196134214453696_i64);
_12 = (false, _17, _29.0, (*_21));
_19.1 = Adt22::Variant0 { fld0: _24,fld1: 148883870487019602783819450215652923580_i128,fld2: _16.1,fld3: 79_i8 };
_16.0 = core::ptr::addr_of_mut!(_12.3);
match _11 {
0 => bb6,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
132252537683774627329951682448481403265 => bb14,
_ => bb13
}
}
bb8 = {
_16.1 = RET as i16;
_7 = _1;
RET = _20 as f64;
_6 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
_2 = _5;
_16.0 = core::ptr::addr_of_mut!(_10);
_22 = _20 * _20;
_12.3 = -_10;
Goto(bb7)
}
bb9 = {
_10 = _12.3 | _12.3;
_12 = (true, _17, 6_usize, _10);
_12.2 = 16628445114961294843_usize;
_10 = _12.3;
_16.0 = core::ptr::addr_of_mut!(_12.3);
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
Goto(bb4)
}
bb10 = {
RET = _10 as f64;
RET = _16.1 as f64;
_17 = _12.1 | _12.1;
_11 = 132252537683774627329951682448481403265_u128;
_2 = [_11,_11,_11,_11,_11];
_14 = _16.1;
_12.1 = _17;
_20 = _10 as f32;
_1 = _7;
RET = (-16_i8) as f64;
_12.2 = 56239_u16 as usize;
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
_21 = Move(_16.0);
_14 = _16.1 >> _11;
_10 = _12.3 << _14;
_19.2 = 5634992928042983387_i64 as i32;
RET = _10 as f64;
_7 = _6;
_10 = _12.3 & _12.3;
match _11 {
132252537683774627329951682448481403265 => bb6,
_ => bb5
}
}
bb11 = {
_10 = _12.3 | _12.3;
_12 = (true, _17, 6_usize, _10);
_12.2 = 16628445114961294843_usize;
_10 = _12.3;
_16.0 = core::ptr::addr_of_mut!(_12.3);
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
Goto(bb4)
}
bb12 = {
_3 = [41_i8,(-93_i8)];
_5 = [281147037887469068746023776569285895578_u128,281759520243974368188634338448058927701_u128,159204459619589747400255638134869750373_u128,229040321135474640451701010789745634093_u128,46914725318911037552868454209004405614_u128];
_12.1 = 7060736106557517886_u64 + 15404593949788924045_u64;
_7 = _6;
_11 = 131446099830293758512749301696021888797_u128;
_7 = _6;
_3 = [(-23_i8),110_i8];
_12.0 = false;
_10 = _12.3;
_12.3 = _10 ^ _10;
_11 = 5380379727206098568_i64 as u128;
_6 = _8;
_9 = [_11,_11,_11,_11,_11];
_14 = (-8215_i16) << _12.2;
_12.0 = true;
RET = _12.1 as f64;
RET = 77230673185002683142297194310701220301_i128 as f64;
_19.2 = (-1172521227_i32);
_12.3 = _14 as isize;
_14 = (-18178_i16);
_16.1 = _12.1 as i16;
_17 = _11 as u64;
_10 = _12.3;
_8 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
RET = _10 as f64;
_12.0 = _12.2 >= _12.2;
_17 = _12.1;
Goto(bb3)
}
bb13 = {
_10 = -9223372036854775807_isize;
_6 = [8983924076502740948_usize,7_usize,12169887405662623461_usize,0_usize,17972274710688319057_usize,3_usize];
_8 = [585922157144823334_usize,8138641344889923274_usize,2_usize,3_usize,0_usize,3592344933090132369_usize];
_1 = [6_usize,3_usize,6_usize,6_usize,6_usize,5_usize];
_12.2 = 6_usize + 1_usize;
_12.3 = 121079554053566511484572357186354161146_u128 as isize;
_12.0 = RET >= RET;
_1 = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
Goto(bb2)
}
bb14 = {
_29.1.0.0 = core::ptr::addr_of_mut!(_29.1.2);
_30 = [_29.0,_29.0,_12.2,_12.2,_29.0];
_31 = [59173_u16,38893_u16,45355_u16,28258_u16,21955_u16,30260_u16,4186_u16,63392_u16];
_29.1.0.2 = '\u{f1f7d}';
_2 = [_11,_11,_11,_11,_11];
_2 = [_11,_11,_11,_11,_11];
_12.0 = false;
_10 = _12.3;
_27 = _12.2;
_35.1 = -Field::<i16>(Variant(_19.1, 0), 2);
_19.2 = -(-896327748_i32);
_29.1.0.0 = core::ptr::addr_of_mut!(_29.1.2);
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(15_usize, 30_usize, Move(_30), 3_usize, Move(_3), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(15_usize, 17_usize, Move(_17), 14_usize, Move(_14), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: char) -> char {
mir! {
type RET = char;
let _2: i32;
let _3: u8;
let _4: i16;
let _5: [bool; 7];
let _6: bool;
let _7: isize;
let _8: Adt73;
let _9: (bool, u64, usize, isize);
let _10: *mut (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _11: [usize; 3];
let _12: Adt71;
let _13: isize;
let _14: u64;
let _15: [usize; 6];
let _16: (u8, u64, i8, u16);
let _17: *const u8;
let _18: f64;
let _19: [u8; 8];
let _20: i16;
let _21: *mut u8;
let _22: ();
let _23: ();
{
_1 = '\u{ae56a}';
RET = _1;
RET = _1;
RET = _1;
_1 = RET;
RET = _1;
RET = _1;
_4 = (-7208_i16);
_3 = 30_u8 << _4;
Goto(bb1)
}
bb1 = {
_3 = !244_u8;
_4 = -21919_i16;
_1 = RET;
_4 = 1095_i16 ^ (-27659_i16);
RET = _1;
_7 = -(-87_isize);
_4 = (-4023_i16) ^ (-24226_i16);
_6 = !false;
_7 = 9223372036854775807_isize;
_3 = 28_u8;
_7 = 17182_u16 as isize;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
28 => bb7,
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
_3 = 116_u8;
_4 = 1085037998_u32 as i16;
Goto(bb8)
}
bb8 = {
_2 = !2047322258_i32;
_6 = !false;
_9.0 = _7 != _7;
_2 = -(-1761727037_i32);
_1 = RET;
_1 = RET;
_9 = (_6, 14332267926127558963_u64, 388972137522048056_usize, _7);
_1 = RET;
_9.0 = !_6;
_9.0 = _6 & _6;
RET = _1;
_9.1 = !14369536030075760312_u64;
_9.0 = !_6;
_3 = !195_u8;
RET = _1;
_3 = _4 as u8;
_2 = 534116678_i32 ^ (-755906435_i32);
_2 = 27122_u16 as i32;
_5 = [_6,_6,_9.0,_9.0,_9.0,_6,_6];
RET = _1;
_7 = -_9.3;
_9.0 = _6;
_9.2 = _4 as usize;
Call(_12.fld1.0.2 = core::intrinsics::bswap(_9.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12.fld1.0.1 = !_9.1;
_12.fld1.0.2 = !_9.2;
_12.fld2 = !2709112635_u32;
_12.fld0.0.2 = _2 - _2;
_6 = _9.0 & _9.0;
_12.fld1.0 = (_9.0, _9.1, _9.2, _7);
_13 = _9.2 as isize;
_12.fld1.0 = (_9.0, _9.1, _9.2, _9.3);
_12.fld1.0.3 = !_13;
_12.fld0.0.2 = _2 + _2;
_4 = !30312_i16;
_12.fld2 = 176416910_u32;
_12.fld2 = 2607374180_u32 >> _2;
_12.fld0.3 = _2 >= _12.fld0.0.2;
_9.3 = -_12.fld1.0.3;
_9.3 = _12.fld1.0.3 ^ _7;
_12.fld0.0.2 = -_2;
_5 = [_6,_12.fld0.3,_9.0,_12.fld0.3,_9.0,_12.fld0.3,_12.fld0.3];
_9.0 = _6;
_12.fld0.1 = Adt44::Variant1 { fld0: _12.fld1.0.1 };
Call(_9.3 = core::intrinsics::bswap(_7), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = RET;
_12.fld0.0.2 = _2;
_9.3 = _4 as isize;
_11 = [_9.2,_12.fld1.0.2,_12.fld1.0.2];
_12.fld1.0.1 = _9.1;
_9 = (_6, Field::<u64>(Variant(_12.fld0.1, 1), 0), _12.fld1.0.2, _12.fld1.0.3);
_11 = [_9.2,_12.fld1.0.2,_9.2];
SetDiscriminant(_12.fld0.1, 1);
_9.3 = _13;
_9.3 = !_7;
_14 = !_9.1;
_12.fld0.0.0 = core::ptr::addr_of!(_12.fld2);
_12.fld0.3 = _9.0;
place!(Field::<u64>(Variant(_12.fld0.1, 1), 0)) = _12.fld1.0.1 >> _9.1;
RET = _1;
_1 = RET;
_9.3 = _7 - _12.fld1.0.3;
RET = _1;
_12.fld0.2 = _5;
_11 = [_9.2,_12.fld1.0.2,_12.fld1.0.2];
_9.1 = _12.fld1.0.1 << _9.2;
_16 = (_3, _9.1, (-75_i8), 9639_u16);
_4 = !1884_i16;
_1 = RET;
_16.0 = _3;
_6 = !_12.fld1.0.0;
_12.fld0.2 = [_12.fld1.0.0,_12.fld0.3,_6,_9.0,_12.fld1.0.0,_9.0,_12.fld0.3];
_17 = core::ptr::addr_of!(_16.0);
Call(_10 = fn17(_16.3, _16.3, _16, _12.fld1.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12.fld1.0 = _9;
_12.fld1.0.3 = _16.2 as isize;
(*_17) = _3 - _3;
_12.fld0.1 = Adt44::Variant0 { fld0: _12.fld2,fld1: Move(_17),fld2: (*_17),fld3: _11 };
_12.fld0.1 = Adt44::Variant1 { fld0: _16.1 };
_14 = _12.fld1.0.3 as u64;
_13 = _9.3 + _9.3;
_20 = 6106990101548819800_i64 as i16;
_12.fld2 = !1457358170_u32;
_12.fld1.0.0 = Field::<u64>(Variant(_12.fld0.1, 1), 0) != Field::<u64>(Variant(_12.fld0.1, 1), 0);
SetDiscriminant(_12.fld0.1, 1);
Goto(bb12)
}
bb12 = {
_16 = (_3, _14, (-25_i8), 20227_u16);
_12.fld1.0.1 = _14;
place!(Field::<u64>(Variant(_12.fld0.1, 1), 0)) = _12.fld1.0.1 >> _16.3;
_17 = core::ptr::addr_of!(_3);
Call(_16.2 = core::intrinsics::transmute(_3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_12.fld1 = (_9,);
place!(Field::<u64>(Variant(_12.fld0.1, 1), 0)) = (-7231928480008471259_i64) as u64;
_16.3 = 38543_u16 + 61771_u16;
_12.fld1.0.2 = !_9.2;
_12.fld3 = [_12.fld2,_12.fld2,_12.fld2,_12.fld2,_12.fld2];
(*_17) = _7 as u8;
place!(Field::<u64>(Variant(_12.fld0.1, 1), 0)) = !_16.1;
Goto(bb14)
}
bb14 = {
_12.fld0.0.2 = _2;
_12.fld3 = [_12.fld2,_12.fld2,_12.fld2,_12.fld2,_12.fld2];
_12.fld1.0.0 = _12.fld0.3;
_18 = _16.3 as f64;
_16.2 = _18 as i8;
_16.0 = _3;
_12.fld0.0.1 = Adt22::Variant0 { fld0: _18,fld1: (-16035621823033934345283708368802124256_i128),fld2: _20,fld3: _16.2 };
_12.fld1 = (_9,);
_9.3 = _13;
(*_17) = _16.0 & _16.0;
_12.fld1.0.2 = _9.2;
RET = _1;
_1 = RET;
_12.fld1.0.0 = _6;
_21 = core::ptr::addr_of_mut!((*_17));
(*_21) = _16.0;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(16_usize, 5_usize, Move(_5), 7_usize, Move(_7), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(16_usize, 20_usize, Move(_20), 14_usize, Move(_14), 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: u16,mut _2: u16,mut _3: (u8, u64, i8, u16),mut _4: (bool, u64, usize, isize)) -> *mut (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8)) {
mir! {
type RET = *mut (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _5: *mut Adt49;
let _6: f64;
let _7: [u8; 8];
let _8: *mut u8;
let _9: char;
let _10: isize;
let _11: [usize; 5];
let _12: *mut *mut &'static [u16; 8];
let _13: isize;
let _14: i128;
let _15: f32;
let _16: i64;
let _17: isize;
let _18: char;
let _19: f64;
let _20: f32;
let _21: &'static *mut u128;
let _22: bool;
let _23: f64;
let _24: f32;
let _25: [bool; 7];
let _26: *mut &'static [u16; 8];
let _27: i32;
let _28: [u32; 3];
let _29: *mut (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _30: char;
let _31: isize;
let _32: bool;
let _33: ([u32; 5], i8);
let _34: isize;
let _35: &'static Adt22;
let _36: [u128; 5];
let _37: bool;
let _38: (bool, u64, usize, isize);
let _39: Adt44;
let _40: Adt81;
let _41: f64;
let _42: *const Adt22;
let _43: f64;
let _44: *const u8;
let _45: u8;
let _46: [u8; 8];
let _47: (((*mut u8, f64, char, i64), &'static usize, u8, u8), i64, [usize; 3], (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8)));
let _48: i8;
let _49: ((bool, u64, usize, isize),);
let _50: isize;
let _51: [bool; 7];
let _52: [usize; 3];
let _53: isize;
let _54: i128;
let _55: f64;
let _56: [usize; 5];
let _57: [u32; 1];
let _58: bool;
let _59: bool;
let _60: isize;
let _61: [usize; 6];
let _62: *mut u8;
let _63: char;
let _64: f32;
let _65: char;
let _66: (u8, u64, i8, u16);
let _67: [bool; 7];
let _68: isize;
let _69: (bool, u64, usize, isize);
let _70: ((bool, u64, usize, isize),);
let _71: *const *mut u8;
let _72: *const u8;
let _73: (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _74: (usize, ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _75: f32;
let _76: isize;
let _77: isize;
let _78: u8;
let _79: (*const Adt22, [u128; 5], ((*mut u8, f64, char, i64), &'static usize, u8, u8));
let _80: [u128; 5];
let _81: ();
let _82: ();
{
_4.3 = -(-9223372036854775808_isize);
_3.1 = _4.1 << _3.3;
_4.2 = 29265842698735371127297879770179842317_u128 as usize;
_4.2 = _4.3 as usize;
_4 = (true, _3.1, 4_usize, 9223372036854775807_isize);
_4.2 = 6145141883206021204_usize & 0_usize;
_2 = !_1;
_4.1 = _3.1;
_3.2 = _1 as i8;
_4 = (true, _3.1, 13007389722808418928_usize, 9223372036854775807_isize);
_3.1 = _1 as u64;
_2 = _4.3 as u16;
_4.3 = _3.0 as isize;
_4.3 = (-103_isize) ^ 9223372036854775807_isize;
_4.1 = 1345706587_u32 as u64;
_4.0 = false | false;
_3.2 = (-19399030948581852830630976989233904707_i128) as i8;
match _3.3 {
0 => bb1,
9639 => bb3,
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
_4.1 = _1 as u64;
_8 = core::ptr::addr_of_mut!(_3.0);
_4.3 = 119_isize;
Goto(bb4)
}
bb4 = {
_6 = 1150069872_u32 as f64;
_1 = !_2;
_1 = _2 % _3.3;
_9 = '\u{699b9}';
_3.3 = 1542020366_i32 as u16;
_3.2 = 105_i8 * (-19_i8);
_4.3 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_8) = 89_u8;
_3.0 = _1 as u8;
_10 = (*_8) as isize;
_2 = _1;
_13 = _10;
_10 = _13 >> _1;
_4.1 = _3.1;
_1 = _2;
_16 = 6557517026179332957_i64;
Goto(bb5)
}
bb5 = {
_3 = (139_u8, _4.1, (-32_i8), _1);
_7 = [(*_8),(*_8),(*_8),(*_8),(*_8),_3.0,(*_8),_3.0];
_4.0 = _3.2 < _3.2;
_4.0 = !false;
_9 = '\u{c30f5}';
_17 = !_13;
_13 = _10 & _10;
_1 = _3.3;
_15 = _3.3 as f32;
_11 = [_4.2,_4.2,_4.2,_4.2,_4.2];
_4.0 = false;
_17 = _13;
(*_8) = _4.0 as u8;
_14 = (-11753894488056976188134978322314196473_i128);
_10 = !_13;
_7 = [_3.0,_3.0,(*_8),_3.0,_3.0,(*_8),(*_8),_3.0];
_19 = _6 * _6;
_15 = _3.0 as f32;
_11 = [_4.2,_4.2,_4.2,_4.2,_4.2];
_4.1 = !_3.1;
_18 = _9;
_3.3 = !_1;
_19 = (*_8) as f64;
_4.1 = _3.1;
match _3.2 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463463374607431768211424 => bb12,
_ => bb11
}
}
bb6 = {
_6 = 1150069872_u32 as f64;
_1 = !_2;
_1 = _2 % _3.3;
_9 = '\u{699b9}';
_3.3 = 1542020366_i32 as u16;
_3.2 = 105_i8 * (-19_i8);
_4.3 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_8) = 89_u8;
_3.0 = _1 as u8;
_10 = (*_8) as isize;
_2 = _1;
_13 = _10;
_10 = _13 >> _1;
_4.1 = _3.1;
_1 = _2;
_16 = 6557517026179332957_i64;
Goto(bb5)
}
bb7 = {
_4.1 = _1 as u64;
_8 = core::ptr::addr_of_mut!(_3.0);
_4.3 = 119_isize;
Goto(bb4)
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
_19 = _6;
_9 = _18;
(*_8) = !156_u8;
match _3.2 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
340282366920938463463374607431768211424 => bb20,
_ => bb19
}
}
bb13 = {
_4.1 = _1 as u64;
_8 = core::ptr::addr_of_mut!(_3.0);
_4.3 = 119_isize;
Goto(bb4)
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
_4.1 = _1 as u64;
_8 = core::ptr::addr_of_mut!(_3.0);
_4.3 = 119_isize;
Goto(bb4)
}
bb18 = {
_6 = 1150069872_u32 as f64;
_1 = !_2;
_1 = _2 % _3.3;
_9 = '\u{699b9}';
_3.3 = 1542020366_i32 as u16;
_3.2 = 105_i8 * (-19_i8);
_4.3 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_8) = 89_u8;
_3.0 = _1 as u8;
_10 = (*_8) as isize;
_2 = _1;
_13 = _10;
_10 = _13 >> _1;
_4.1 = _3.1;
_1 = _2;
_16 = 6557517026179332957_i64;
Goto(bb5)
}
bb19 = {
Return()
}
bb20 = {
_3.3 = _1;
_16 = !(-574014310932362707_i64);
_22 = _4.0 | _4.0;
_14 = -(-153286939333372584960860122399741903439_i128);
_4.1 = _3.1 * _3.1;
_13 = _17 & _10;
_20 = _6 as f32;
_7 = [_3.0,_3.0,_3.0,(*_8),(*_8),_3.0,(*_8),_3.0];
_3.2 = !(-32_i8);
_23 = _19;
_4.3 = !_17;
_16 = (-3678391250005097382_i64) ^ (-5856394292590926507_i64);
match _4.2 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb21,
13007389722808418928 => bb23,
_ => bb22
}
}
bb21 = {
Return()
}
bb22 = {
_19 = _6;
_9 = _18;
(*_8) = !156_u8;
match _3.2 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
340282366920938463463374607431768211424 => bb20,
_ => bb19
}
}
bb23 = {
_11 = [_4.2,_4.2,_4.2,_4.2,_4.2];
_10 = -_4.3;
_22 = _4.0;
_4.0 = _22;
_9 = _18;
_16 = (-1378401365979677247_i64) ^ (-7263091582517100492_i64);
_12 = core::ptr::addr_of_mut!(_26);
(*_8) = !102_u8;
_15 = -_20;
_4.2 = _13 as usize;
_23 = _6 * _19;
_20 = _15;
_18 = _9;
_22 = !_4.0;
_9 = _18;
_30 = _18;
_20 = _3.0 as f32;
(*_8) = 81_u8 & 231_u8;
_4.1 = _3.1 << _4.2;
_28 = [3053682029_u32,823037552_u32,2952208391_u32];
_30 = _9;
_3.3 = _2;
Goto(bb24)
}
bb24 = {
_30 = _9;
_9 = _18;
Goto(bb25)
}
bb25 = {
_31 = _10;
_22 = !_4.0;
_32 = _22;
_20 = -_15;
_4.3 = -_10;
_3.2 = (-49_i8);
_4.2 = 13105582207066077918_usize | 17614116692420108181_usize;
_7 = [(*_8),(*_8),_3.0,(*_8),(*_8),(*_8),(*_8),(*_8)];
_13 = -_4.3;
(*_8) = _19 as u8;
match _3.2 {
0 => bb1,
1 => bb23,
2 => bb3,
3 => bb18,
4 => bb5,
5 => bb24,
340282366920938463463374607431768211407 => bb26,
_ => bb7
}
}
bb26 = {
_24 = 1238825590_i32 as f32;
_3.0 = (-26104_i16) as u8;
_34 = _13 << _10;
_3.3 = _2 ^ _2;
_3 = (184_u8, _4.1, (-43_i8), _2);
_1 = (-24156_i16) as u16;
_3.0 = !37_u8;
_8 = core::ptr::addr_of_mut!((*_8));
_25 = [_32,_4.0,_4.0,_32,_22,_32,_4.0];
_13 = _10 - _31;
_33.1 = _3.2;
_3 = (30_u8, _4.1, _33.1, _2);
_3.2 = _33.1;
Goto(bb27)
}
bb27 = {
_19 = -_6;
_20 = _24 * _15;
_32 = _22;
_22 = !_4.0;
_24 = _20 * _15;
_4.3 = _31;
_6 = 3844939273_u32 as f64;
_27 = -2022440376_i32;
_33.1 = -_3.2;
_24 = _20;
_18 = _30;
_18 = _30;
_38 = (_4.0, _3.1, _4.2, _10);
_6 = -_23;
_37 = _4.0 ^ _32;
_9 = _30;
_18 = _30;
_6 = _23 * _23;
match _3.0 {
0 => bb11,
1 => bb21,
30 => bb29,
_ => bb28
}
}
bb28 = {
_4.1 = _1 as u64;
_8 = core::ptr::addr_of_mut!(_3.0);
_4.3 = 119_isize;
Goto(bb4)
}
bb29 = {
_4.3 = !_34;
_36 = [63476847886955904000439254350192431476_u128,324102382038079974546013893801490789124_u128,41760725529566106368404966637798504969_u128,301835626946471513214526731504312546597_u128,336722754692050104204318871868664928958_u128];
_3.3 = _2 ^ _2;
_6 = _23 - _19;
_4 = _38;
_4.3 = !_31;
_28 = [4036830990_u32,2095596285_u32,1336853423_u32];
Goto(bb30)
}
bb30 = {
_38.0 = !_37;
_37 = _38.0;
_6 = _23 + _23;
_6 = _24 as f64;
_38 = _4;
_13 = !_34;
_3 = (71_u8, _4.1, _33.1, _2);
_34 = _31;
_2 = !_3.3;
_4.2 = !_38.2;
_14 = (-19887792762479240172512470513959637443_i128);
_10 = _31 & _38.3;
_38 = (_4.0, _3.1, _4.2, _13);
_30 = _9;
Call(_4.3 = fn18(_13, _38, _3.0, _3.2, (*_8), _3.1, _3.0, _3.2, _38.1, _38.3, _3, _3.1), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_3.3 = _2 - _2;
_34 = _38.3;
_10 = _16 as isize;
_44 = core::ptr::addr_of!(_3.0);
_33.1 = -_3.2;
_2 = 1121340654_u32 as u16;
(*_44) = 142_u8;
_20 = _15;
(*_8) = 16_u8 | 92_u8;
_47.2 = [_4.2,_38.2,_4.2];
Goto(bb32)
}
bb32 = {
_4.3 = _31 | _38.3;
_47.3.1 = _36;
_47.0.0.3 = !_16;
_47.0.0.0 = Move(_8);
_49.0 = _4;
_9 = _18;
_40.fld0 = [1943017029_u32];
_45 = (*_44) << _17;
match _14 {
0 => bb33,
1 => bb34,
2 => bb35,
3 => bb36,
4 => bb37,
5 => bb38,
6 => bb39,
320394574158459223290862136917808574013 => bb41,
_ => bb40
}
}
bb33 = {
_4.1 = _1 as u64;
_8 = core::ptr::addr_of_mut!(_3.0);
_4.3 = 119_isize;
Goto(bb4)
}
bb34 = {
_6 = 1150069872_u32 as f64;
_1 = !_2;
_1 = _2 % _3.3;
_9 = '\u{699b9}';
_3.3 = 1542020366_i32 as u16;
_3.2 = 105_i8 * (-19_i8);
_4.3 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
(*_8) = 89_u8;
_3.0 = _1 as u8;
_10 = (*_8) as isize;
_2 = _1;
_13 = _10;
_10 = _13 >> _1;
_4.1 = _3.1;
_1 = _2;
_16 = 6557517026179332957_i64;
Goto(bb5)
}
bb35 = {
_4.3 = !_34;
_36 = [63476847886955904000439254350192431476_u128,324102382038079974546013893801490789124_u128,41760725529566106368404966637798504969_u128,301835626946471513214526731504312546597_u128,336722754692050104204318871868664928958_u128];
_3.3 = _2 ^ _2;
_6 = _23 - _19;
_4 = _38;
_4.3 = !_31;
_28 = [4036830990_u32,2095596285_u32,1336853423_u32];
Goto(bb30)
}
bb36 = {
_3 = (139_u8, _4.1, (-32_i8), _1);
_7 = [(*_8),(*_8),(*_8),(*_8),(*_8),_3.0,(*_8),_3.0];
_4.0 = _3.2 < _3.2;
_4.0 = !false;
_9 = '\u{c30f5}';
_17 = !_13;
_13 = _10 & _10;
_1 = _3.3;
_15 = _3.3 as f32;
_11 = [_4.2,_4.2,_4.2,_4.2,_4.2];
_4.0 = false;
_17 = _13;
(*_8) = _4.0 as u8;
_14 = (-11753894488056976188134978322314196473_i128);
_10 = !_13;
_7 = [_3.0,_3.0,(*_8),_3.0,_3.0,(*_8),(*_8),_3.0];
_19 = _6 * _6;
_15 = _3.0 as f32;
_11 = [_4.2,_4.2,_4.2,_4.2,_4.2];
_4.1 = !_3.1;
_18 = _9;
_3.3 = !_1;
_19 = (*_8) as f64;
_4.1 = _3.1;
match _3.2 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463463374607431768211424 => bb12,
_ => bb11
}
}
bb37 = {
_19 = -_6;
_20 = _24 * _15;
_32 = _22;
_22 = !_4.0;
_24 = _20 * _15;
_4.3 = _31;
_6 = 3844939273_u32 as f64;
_27 = -2022440376_i32;
_33.1 = -_3.2;
_24 = _20;
_18 = _30;
_18 = _30;
_38 = (_4.0, _3.1, _4.2, _10);
_6 = -_23;
_37 = _4.0 ^ _32;
_9 = _30;
_18 = _30;
_6 = _23 * _23;
match _3.0 {
0 => bb11,
1 => bb21,
30 => bb29,
_ => bb28
}
}
bb38 = {
Return()
}
bb39 = {
Return()
}
bb40 = {
_30 = _9;
_9 = _18;
Goto(bb25)
}
bb41 = {
_3.1 = _17 as u64;
Goto(bb42)
}
bb42 = {
_41 = _45 as f64;
_34 = !_4.3;
Goto(bb43)
}
bb43 = {
_28 = [459700499_u32,3893473063_u32,4069032653_u32];
_47.0.0.2 = _9;
_49.0.3 = _13;
_47.0.0.1 = _3.3 as f64;
_3.3 = _9 as u16;
_33.0 = [2862134552_u32,970999082_u32,2990728785_u32,4083685017_u32,3928821719_u32];
_28 = [4276925814_u32,993661662_u32,204556936_u32];
_50 = _37 as isize;
_10 = _34 - _13;
_47.3.2.0.2 = _47.0.0.2;
_30 = _47.3.2.0.2;
_4.3 = _47.0.0.3 as isize;
_14 = _27 as i128;
_38 = (_37, _49.0.1, _49.0.2, _17);
_47.3.2.0.1 = _47.0.0.1 - _41;
_54 = _14 - _14;
_38.3 = _10 + _10;
_20 = -_15;
_47.0.1 = &_49.0.2;
_4.2 = _49.0.2 * _38.2;
_47.2 = [_4.2,_38.2,_38.2];
_47.0.0.2 = _30;
_4.2 = _38.2;
_33.1 = _3.2;
Call(_47.1 = core::intrinsics::bswap(_16), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
_52 = _47.2;
_53 = -_31;
_7 = [_45,_45,_45,_45,_45,_45,_45,_45];
_47.0.1 = &_4.2;
_49.0.1 = _4.1;
_47.3.2.1 = &_49.0.2;
_23 = _41 * _47.3.2.0.1;
_22 = _49.0.0;
_27 = (-724629887_i32) ^ 1357891617_i32;
_55 = _23 * _23;
_4.1 = _37 as u64;
_13 = _49.0.3;
_15 = -_24;
_38.2 = _4.2;
_32 = _17 <= _38.3;
_51 = [_32,_32,_32,_32,_32,_32,_32];
_39 = Adt44::Variant1 { fld0: _38.1 };
_22 = _32;
_30 = _9;
_31 = _24 as isize;
_39 = Adt44::Variant0 { fld0: 3281753728_u32,fld1: Move(_44),fld2: _45,fld3: _52 };
_48 = _33.1;
_47.3.2.0 = (Move(_47.0.0.0), _23, _47.0.0.2, _47.0.0.3);
Goto(bb45)
}
bb45 = {
_59 = _3.2 != _3.2;
_57 = [2256831631_u32];
_47.0.0.0 = Move(_47.3.2.0.0);
_49.0 = (_32, _38.1, _4.2, _17);
_46 = [Field::<u8>(Variant(_39, 0), 2),_45,Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),_45];
_58 = _59;
_7 = [Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),_45,Field::<u8>(Variant(_39, 0), 2),Field::<u8>(Variant(_39, 0), 2),_45,_3.0];
place!(Field::<u32>(Variant(_39, 0), 0)) = _27 as u32;
_40 = Adt81 { fld0: _57 };
_47.3.2.0.0 = core::ptr::addr_of_mut!(_45);
_30 = _18;
_11 = [_49.0.2,_49.0.2,_49.0.2,_38.2,_38.2];
_54 = _14;
_19 = _23 * _23;
_10 = _38.3 ^ _38.3;
_47.3.2.1 = &_38.2;
_32 = _47.3.2.0.3 <= _16;
_47.3.2.2 = !_45;
_47.0.0.2 = _30;
_44 = Move(Field::<*const u8>(Variant(_39, 0), 1));
_47.0.0.3 = !_16;
_38.0 = !_22;
Goto(bb46)
}
bb46 = {
_38.0 = !_49.0.0;
_4.2 = _38.2;
_63 = _9;
Goto(bb47)
}
bb47 = {
_15 = -_24;
_54 = _14;
place!(Field::<u32>(Variant(_39, 0), 0)) = 1223736685_u32 & 809291041_u32;
_38 = _49.0;
_49 = (_38,);
_23 = -_19;
_61 = [_49.0.2,_49.0.2,_4.2,_49.0.2,_38.2,_38.2];
_56 = [_38.2,_4.2,_4.2,_38.2,_4.2];
_64 = _45 as f32;
_4.3 = _38.3;
_8 = core::ptr::addr_of_mut!(_47.3.2.3);
_3.2 = _48 << _48;
_19 = _23;
_47.3.2.0 = (Move(_47.0.0.0), _55, _9, _47.0.0.3);
_20 = _38.1 as f32;
_66 = (_45, _38.1, _33.1, _1);
_11 = [_38.2,_38.2,_38.2,_49.0.2,_4.2];
_17 = _34;
Goto(bb48)
}
bb48 = {
_40.fld0 = [Field::<u32>(Variant(_39, 0), 0)];
_28 = [Field::<u32>(Variant(_39, 0), 0),Field::<u32>(Variant(_39, 0), 0),Field::<u32>(Variant(_39, 0), 0)];
Goto(bb49)
}
bb49 = {
_12 = core::ptr::addr_of_mut!((*_12));
_59 = _38.0;
_4.2 = _49.0.2;
_8 = Move(_47.3.2.0.0);
_49.0.2 = _4.2 ^ _4.2;
_64 = _20;
_3.2 = !_66.2;
Call(_47.3.2.0.1 = core::intrinsics::fmaf64(_23, _23, _23), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
_71 = core::ptr::addr_of!(_47.0.0.0);
_73.1.0.0 = Move(_8);
_64 = -_20;
_68 = _10;
_15 = _64;
_4.1 = _38.1;
_47.0.0 = (Move(_73.1.0.0), _19, _30, _16);
_47.3.2.0.0 = Move((*_71));
_63 = _47.3.2.0.2;
_55 = -_47.0.0.1;
_24 = _64 - _64;
_73.0 = _4.3 as usize;
RET = core::ptr::addr_of_mut!(_74);
(*RET).1.0.1 = -_55;
Goto(bb51)
}
bb51 = {
Call(_81 = dump_var(17_usize, 11_usize, Move(_11), 14_usize, Move(_14), 56_usize, Move(_56), 63_usize, Move(_63)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_81 = dump_var(17_usize, 17_usize, Move(_17), 30_usize, Move(_30), 18_usize, Move(_18), 10_usize, Move(_10)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_81 = dump_var(17_usize, 48_usize, Move(_48), 50_usize, Move(_50), 28_usize, Move(_28), 51_usize, Move(_51)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_81 = dump_var(17_usize, 3_usize, Move(_3), 45_usize, Move(_45), 34_usize, Move(_34), 57_usize, Move(_57)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_81 = dump_var(17_usize, 61_usize, Move(_61), 36_usize, Move(_36), 49_usize, Move(_49), 59_usize, Move(_59)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_81 = dump_var(17_usize, 54_usize, Move(_54), 82_usize, _82, 82_usize, _82, 82_usize, _82), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: (bool, u64, usize, isize),mut _3: u8,mut _4: i8,mut _5: u8,mut _6: u64,mut _7: u8,mut _8: i8,mut _9: u64,mut _10: isize,mut _11: (u8, u64, i8, u16),mut _12: u64) -> isize {
mir! {
type RET = isize;
let _13: *const *mut u8;
let _14: *mut *const u32;
let _15: [bool; 7];
let _16: [usize; 7];
let _17: [u32; 5];
let _18: char;
let _19: isize;
let _20: *mut &'static u32;
let _21: *const u32;
let _22: Adt44;
let _23: i16;
let _24: i16;
let _25: *mut *mut &'static [u16; 8];
let _26: i8;
let _27: ();
let _28: ();
{
RET = _10 * _1;
_12 = _9;
_6 = !_11.1;
_5 = !_7;
_11.2 = _4 >> _1;
_8 = _4 + _4;
_3 = !_5;
RET = _2.3;
RET = 37287928119310083010542050471065106615_u128 as isize;
_11.0 = _7 % _7;
_2 = (true, _9, 11374684837859943228_usize, _10);
_2.1 = (-2189073434848071059_i64) as u64;
Goto(bb1)
}
bb1 = {
RET = _1 & _2.3;
_11.0 = 3623772391_u32 as u8;
RET = _10 + _10;
_11 = (_7, _6, _8, 32879_u16);
_18 = '\u{a23b}';
_12 = _6 - _9;
_11.1 = 66630544636271578076188491898089490073_i128 as u64;
_10 = _12 as isize;
_11.1 = 180513111154471165652774658652164086427_u128 as u64;
_17 = [1890022293_u32,2593001035_u32,3383549987_u32,4135129925_u32,4182971908_u32];
_7 = _11.0 | _3;
_11.0 = !_7;
_12 = _6 - _9;
_1 = _5 as isize;
_15 = [_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0];
_3 = _7;
RET = -_1;
_11.1 = _6;
_11.0 = _7 >> _6;
RET = _1 >> _11.2;
_2.1 = !_12;
_12 = _6;
Goto(bb2)
}
bb2 = {
Call(_27 = dump_var(18_usize, 2_usize, Move(_2), 9_usize, Move(_9), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_27 = dump_var(18_usize, 18_usize, Move(_18), 11_usize, Move(_11), 12_usize, Move(_12), 28_usize, _28), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: u128,mut _2: bool,mut _3: u128,mut _4: *mut u128,mut _5: (bool, u64, usize, isize),mut _6: isize,mut _7: isize,mut _8: isize,mut _9: (bool, u64, usize, isize)) -> f64 {
mir! {
type RET = f64;
let _10: (*mut isize, i16);
let _11: *mut Adt49;
let _12: i8;
let _13: Adt81;
let _14: *mut *const u32;
let _15: [bool; 7];
let _16: bool;
let _17: bool;
let _18: u8;
let _19: [i8; 8];
let _20: [usize; 5];
let _21: Adt44;
let _22: Adt44;
let _23: bool;
let _24: (*mut u8, f64, char, i64);
let _25: isize;
let _26: [u32; 1];
let _27: f32;
let _28: u64;
let _29: *const i64;
let _30: f64;
let _31: i16;
let _32: [usize; 5];
let _33: char;
let _34: ();
let _35: ();
{
_1 = _3;
_10.0 = core::ptr::addr_of_mut!(_9.3);
Goto(bb1)
}
bb1 = {
_5.2 = _9.2;
_9.2 = !_5.2;
RET = 111_u8 as f64;
_8 = _7 << _1;
_9.0 = _2;
_9.3 = _9.1 as isize;
_9.2 = _2 as usize;
_1 = _3;
_3 = _8 as u128;
_10.1 = RET as i16;
_5.0 = _9.0;
_8 = -_7;
_5 = _9;
_9.3 = _9.2 as isize;
_10.0 = core::ptr::addr_of_mut!(_7);
_5.3 = 56814_u16 as isize;
_2 = _9.0;
_9.1 = _5.1;
_6 = -_7;
RET = _9.2 as f64;
_9.1 = _6 as u64;
RET = 24364_u16 as f64;
_9.0 = _5.0;
_5.1 = _6 as u64;
Goto(bb2)
}
bb2 = {
_9.3 = -_6;
_5.1 = _9.0 as u64;
RET = 54644444_u32 as f64;
_5.2 = _9.2;
_2 = _6 < _7;
_10.1 = 9838_i16 + (-1618_i16);
_8 = -_9.3;
_9.0 = _2;
_5.0 = _9.0;
_5.0 = _9.0;
_13.fld0 = [520274227_u32];
_2 = _9.0 < _9.0;
_7 = _9.2 as isize;
_5.3 = '\u{a91b2}' as isize;
_6 = _9.3;
_1 = 166_u8 as u128;
_10.1 = (-16570134775244138262880587287595330761_i128) as i16;
_10.0 = core::ptr::addr_of_mut!(_6);
_7 = -_6;
_12 = 66_i8 >> _7;
_15 = [_5.0,_5.0,_2,_9.0,_9.0,_9.0,_5.0];
_6 = _9.3;
_2 = _9.0;
_3 = !_1;
_12 = 39_i8 ^ (-15_i8);
Goto(bb3)
}
bb3 = {
_5 = (_9.0, _9.1, _9.2, _7);
_5.1 = (-116615986609345026360186370298222022517_i128) as u64;
_5.3 = RET as isize;
RET = _9.2 as f64;
_9 = _5;
_8 = _2 as isize;
_10.1 = -(-1968_i16);
_18 = 146_u8 * 249_u8;
RET = _1 as f64;
_15 = [_5.0,_9.0,_5.0,_2,_9.0,_9.0,_5.0];
_5.3 = -_6;
_7 = _8;
_4 = core::ptr::addr_of_mut!(_1);
_2 = _5.0;
_17 = _5.0 <= _2;
_13.fld0 = [3897072304_u32];
_10.1 = (-6298_i16) << _5.3;
_19 = [_12,_12,_12,_12,_12,_12,_12,_12];
Goto(bb4)
}
bb4 = {
_5.1 = _9.1;
_5.0 = !_17;
_13.fld0 = [2237384753_u32];
_5 = _9;
_21 = Adt44::Variant1 { fld0: _9.1 };
_24.2 = '\u{f5f39}';
_9.3 = -_8;
_15 = [_2,_17,_5.0,_2,_9.0,_17,_9.0];
_5.2 = _9.1 as usize;
(*_4) = _3;
_27 = _5.1 as f32;
_16 = !_5.0;
_9.0 = _16 & _17;
_23 = _16;
_5.0 = _9.0;
(*_4) = _3 * _3;
_25 = _24.2 as isize;
_27 = _12 as f32;
_27 = 2883539849_u32 as f32;
(*_4) = (-6288229977826029117_i64) as u128;
_24.0 = core::ptr::addr_of_mut!(_18);
_18 = 41_u8;
_2 = _16 | _16;
_12 = (-76_i8) - 124_i8;
_20 = [_9.2,_9.2,_9.2,_9.2,_9.2];
_24.2 = '\u{7c2cc}';
match _18 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
41 => bb8,
_ => bb7
}
}
bb5 = {
_5 = (_9.0, _9.1, _9.2, _7);
_5.1 = (-116615986609345026360186370298222022517_i128) as u64;
_5.3 = RET as isize;
RET = _9.2 as f64;
_9 = _5;
_8 = _2 as isize;
_10.1 = -(-1968_i16);
_18 = 146_u8 * 249_u8;
RET = _1 as f64;
_15 = [_5.0,_9.0,_5.0,_2,_9.0,_9.0,_5.0];
_5.3 = -_6;
_7 = _8;
_4 = core::ptr::addr_of_mut!(_1);
_2 = _5.0;
_17 = _5.0 <= _2;
_13.fld0 = [3897072304_u32];
_10.1 = (-6298_i16) << _5.3;
_19 = [_12,_12,_12,_12,_12,_12,_12,_12];
Goto(bb4)
}
bb6 = {
_9.3 = -_6;
_5.1 = _9.0 as u64;
RET = 54644444_u32 as f64;
_5.2 = _9.2;
_2 = _6 < _7;
_10.1 = 9838_i16 + (-1618_i16);
_8 = -_9.3;
_9.0 = _2;
_5.0 = _9.0;
_5.0 = _9.0;
_13.fld0 = [520274227_u32];
_2 = _9.0 < _9.0;
_7 = _9.2 as isize;
_5.3 = '\u{a91b2}' as isize;
_6 = _9.3;
_1 = 166_u8 as u128;
_10.1 = (-16570134775244138262880587287595330761_i128) as i16;
_10.0 = core::ptr::addr_of_mut!(_6);
_7 = -_6;
_12 = 66_i8 >> _7;
_15 = [_5.0,_5.0,_2,_9.0,_9.0,_9.0,_5.0];
_6 = _9.3;
_2 = _9.0;
_3 = !_1;
_12 = 39_i8 ^ (-15_i8);
Goto(bb3)
}
bb7 = {
_5.2 = _9.2;
_9.2 = !_5.2;
RET = 111_u8 as f64;
_8 = _7 << _1;
_9.0 = _2;
_9.3 = _9.1 as isize;
_9.2 = _2 as usize;
_1 = _3;
_3 = _8 as u128;
_10.1 = RET as i16;
_5.0 = _9.0;
_8 = -_7;
_5 = _9;
_9.3 = _9.2 as isize;
_10.0 = core::ptr::addr_of_mut!(_7);
_5.3 = 56814_u16 as isize;
_2 = _9.0;
_9.1 = _5.1;
_6 = -_7;
RET = _9.2 as f64;
_9.1 = _6 as u64;
RET = 24364_u16 as f64;
_9.0 = _5.0;
_5.1 = _6 as u64;
Goto(bb2)
}
bb8 = {
_8 = _6;
Goto(bb9)
}
bb9 = {
(*_4) = 144059563475608194045279220727155135938_i128 as u128;
_15 = [_23,_5.0,_23,_5.0,_16,_2,_16];
Goto(bb10)
}
bb10 = {
_24.3 = (-3570321331026720152_i64);
_10.0 = core::ptr::addr_of_mut!(_7);
RET = _18 as f64;
_24.2 = '\u{3f168}';
_9.3 = _7;
_2 = _23;
_17 = !_5.0;
_26 = [3699417514_u32];
_22 = Move(_21);
_3 = _9.2 as u128;
_8 = _10.1 as isize;
_4 = core::ptr::addr_of_mut!((*_4));
_9.3 = -_8;
_13 = Adt81 { fld0: _26 };
(*_4) = _3;
_21 = Move(_22);
_22 = Move(_21);
_18 = !194_u8;
SetDiscriminant(_22, 0);
_9 = _5;
_2 = !_17;
_9.2 = !_5.2;
_5.2 = _18 as usize;
RET = _9.1 as f64;
_9.0 = _23 != _17;
place!(Field::<u32>(Variant(_22, 0), 0)) = 457044522_u32;
(*_4) = _3;
_3 = (*_4);
_5.2 = _9.2 << (*_4);
place!(Field::<u32>(Variant(_22, 0), 0)) = !4179621228_u32;
match _24.3 {
0 => bb7,
1 => bb11,
340282366920938463459804286100741491304 => bb13,
_ => bb12
}
}
bb11 = {
_5 = (_9.0, _9.1, _9.2, _7);
_5.1 = (-116615986609345026360186370298222022517_i128) as u64;
_5.3 = RET as isize;
RET = _9.2 as f64;
_9 = _5;
_8 = _2 as isize;
_10.1 = -(-1968_i16);
_18 = 146_u8 * 249_u8;
RET = _1 as f64;
_15 = [_5.0,_9.0,_5.0,_2,_9.0,_9.0,_5.0];
_5.3 = -_6;
_7 = _8;
_4 = core::ptr::addr_of_mut!(_1);
_2 = _5.0;
_17 = _5.0 <= _2;
_13.fld0 = [3897072304_u32];
_10.1 = (-6298_i16) << _5.3;
_19 = [_12,_12,_12,_12,_12,_12,_12,_12];
Goto(bb4)
}
bb12 = {
_8 = _6;
Goto(bb9)
}
bb13 = {
_20 = [_5.2,_9.2,_5.2,_5.2,_5.2];
_17 = _2;
_4 = core::ptr::addr_of_mut!(_1);
_5.0 = _17;
_10.0 = core::ptr::addr_of_mut!(_9.3);
_17 = _8 <= _8;
_16 = _23 >= _9.0;
_10.1 = !21674_i16;
_9.2 = (-1954931065_i32) as usize;
_13.fld0 = [Field::<u32>(Variant(_22, 0), 0)];
place!(Field::<*const u8>(Variant(_22, 0), 1)) = core::ptr::addr_of!(_18);
_26 = [Field::<u32>(Variant(_22, 0), 0)];
place!(Field::<u32>(Variant(_22, 0), 0)) = 2119112705_u32;
place!(Field::<[usize; 3]>(Variant(_22, 0), 3)) = [_9.2,_5.2,_5.2];
RET = _24.3 as f64;
_24.1 = _18 as f64;
place!(Field::<u8>(Variant(_22, 0), 2)) = !_18;
_13 = Adt81 { fld0: _26 };
_9.2 = _5.2;
_9.2 = _3 as usize;
_2 = _9.0 | _9.0;
_22 = Adt44::Variant1 { fld0: _5.1 };
_30 = _3 as f64;
_24.0 = core::ptr::addr_of_mut!(_18);
_5.2 = _12 as usize;
_21 = Adt44::Variant1 { fld0: _5.1 };
SetDiscriminant(_22, 0);
_12 = _8 as i8;
_9 = (_23, _5.1, _5.2, _8);
Goto(bb14)
}
bb14 = {
_30 = RET * RET;
_15 = [_17,_16,_2,_9.0,_23,_2,_23];
place!(Field::<*const u8>(Variant(_22, 0), 1)) = core::ptr::addr_of!(place!(Field::<u8>(Variant(_22, 0), 2)));
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(19_usize, 7_usize, Move(_7), 15_usize, Move(_15), 2_usize, Move(_2), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(19_usize, 1_usize, Move(_1), 18_usize, Move(_18), 12_usize, Move(_12), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(19_usize, 26_usize, Move(_26), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0();
                
            }
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: f64,
fld1: i128,
fld2: i16,
fld3: i8,

},
Variant1{
fld0: u32,
fld1: u64,
fld2: u128,
fld3: *const u32,
fld4: [u32; 5],
fld5: i32,

},
Variant2{
fld0: u64,
fld1: u8,
fld2: isize,
fld3: i8,
fld4: f64,
fld5: i128,
fld6: (bool, u64, usize, isize),

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u32,
fld1: *const u8,
fld2: u8,
fld3: [usize; 3],

},
Variant1{
fld0: u64,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt44,
fld1: (*const u32, Adt22, i32),
fld2: [u128; 5],
fld3: [u16; 8],
fld4: i16,
fld5: *mut u8,

},
Variant1{
fld0: i64,
fld1: *mut *const u32,
fld2: i8,

},
Variant2{
fld0: bool,
fld1: *mut u128,
fld2: u64,
fld3: usize,
fld4: [bool; 7],
fld5: ([u32; 5], i8),
fld6: *const u32,

},
Variant3{
fld0: [i8; 8],
fld1: [bool; 7],
fld2: f32,
fld3: Adt44,
fld4: i16,
fld5: Adt22,
fld6: (u8, u64, i8, u16),
fld7: u16,

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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt49,
fld1: (*const u32, Adt22, i32),
fld2: f64,
fld3: *const i8,
fld4: *mut u128,
fld5: u64,
fld6: *const *mut u8,

},
Variant1{
fld0: u16,
fld1: *mut isize,
fld2: isize,
fld3: *const u32,
fld4: (*mut u8, f64, char, i64),
fld5: Adt22,

},
Variant2{
fld0: [u128; 5],

},
Variant3{
fld0: *const u8,
fld1: f64,

}}
impl PrintFDebug for Adt71{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt71{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt71 {
fld0: ((*const u32, Adt22, i32), Adt44, [bool; 7], bool),
fld1: ((bool, u64, usize, isize),),
fld2: u32,
fld3: [u32; 5],
}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: u64,
fld1: char,
fld2: *const *mut u8,
fld3: i8,
fld4: Adt71,
fld5: i32,

},
Variant1{
fld0: (*const u32, Adt22, i32),
fld1: u16,
fld2: ((*const u32, Adt22, i32), Adt44, [bool; 7], bool),
fld3: *const i64,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt75{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt75 {
fld0: Adt44,
fld1: i128,
fld2: (*const u32, Adt22, i32),
fld3: *mut Adt49,
fld4: (*mut u8, f64, char, i64),
fld5: Adt49,
fld6: u64,
}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt81{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt81 {
fld0: [u32; 1],
}

