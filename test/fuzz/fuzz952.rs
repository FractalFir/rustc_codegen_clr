#![allow(dead_code,unused_variables)]#![recursion_limit = "1024"]
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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _15: Adt53;
let _16: i128;
let _17: Adt53;
let _18: i64;
let _19: (i8,);
let _20: (u32,);
let _21: i128;
let _22: (i8, (u32,));
let _23: [u16; 1];
let _24: isize;
let _25: (u16,);
let _26: [i16; 1];
let _27: bool;
let _28: char;
let _29: [usize; 4];
let _30: f32;
let _31: Adt52;
let _32: ();
let _33: ();
{
RET = [false];
_2 = '\u{887df}';
_13 = 5577106844548969447_u64 + 4992644460720084766_u64;
_14 = 303911632680548654946601180162533739045_u128 >> _13;
RET = [true];
_5 = (-17081_i16);
_8 = 123231408455149522114591889645517066544_i128 ^ 128020026866745962901524229060722090143_i128;
_6 = (-203757026_i32) | 1142972081_i32;
_3 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_9 = !5149790387226775825_usize;
RET = [false];
_4 = 118_i8;
_2 = '\u{399b}';
_5 = (-12890_i16);
_13 = 1038685732728811843_u64;
RET = [true];
_7 = 2608913001405930910_i64;
_3 = (-52_isize) & (-116_isize);
_12 = _2 as u32;
RET = [true];
RET = [true];
RET = [false];
_15.fld0 = [_5];
_15.fld0 = [_5];
_11 = 14600_u16;
_10 = 162_u8 * 163_u8;
Call(_14 = fn1(_3, _13, _3, _7, _4, _3, _11, _3, _15, _5, _3, _11, _9, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !205_u8;
_1 = true;
RET = [_1];
_1 = true;
_3 = _14 as isize;
_9 = _1 as usize;
_6 = _4 as i32;
_5 = _12 as i16;
_16 = -_8;
_1 = true;
_7 = !6682927582374131302_i64;
_1 = !false;
_9 = !6645590568850390959_usize;
_2 = '\u{3f84a}';
_17.fld0 = [_5];
RET = [_1];
_17 = Adt53 { fld0: _15.fld0 };
_8 = _16;
_8 = _2 as i128;
_4 = -(-103_i8);
_8 = _16 * _16;
_14 = 36985817805519696604243628697109419093_u128;
_4 = 54_i8;
_6 = (-497841883_i32);
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
54 => bb10,
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
_15 = _17;
_17.fld0 = [_5];
_16 = _8;
_4 = _10 as i8;
_1 = !true;
_20.0 = _12;
_4 = (-26_i8) * 95_i8;
_9 = !2_usize;
_3 = 9223372036854775807_isize * (-9223372036854775808_isize);
_18 = _7 * _7;
_3 = 9223372036854775807_isize;
_20.0 = _13 as u32;
_7 = !_18;
_4 = 31_i8 + (-34_i8);
_8 = _16;
_3 = _20.0 as isize;
_14 = _13 as u128;
_20.0 = _12;
_14 = 70985892654110583125035430005074703160_u128;
_20.0 = _12 ^ _12;
_21 = _3 as i128;
_2 = '\u{80a87}';
RET = [_1];
_17 = _15;
_3 = _4 as isize;
_4 = 49_i8;
Goto(bb11)
}
bb11 = {
_25 = (_11,);
_23 = [_11];
_24 = _2 as isize;
_19 = (_4,);
_16 = _8 << _8;
_27 = !_1;
_24 = !_3;
_21 = -_8;
_17.fld0 = [_5];
_6 = 1359725505_i32;
_13 = !17615242787775174356_u64;
_26 = [_5];
_27 = _1;
_16 = _8;
_19 = (_4,);
_9 = _20.0 as usize;
_17 = Adt53 { fld0: _26 };
_22.0 = _19.0;
_21 = _8 | _16;
match _14 {
0 => bb2,
1 => bb12,
2 => bb13,
70985892654110583125035430005074703160 => bb15,
_ => bb14
}
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
_22.1 = _20;
_8 = !_21;
_11 = !_25.0;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(0_usize, 20_usize, Move(_20), 13_usize, Move(_13), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(0_usize, 21_usize, Move(_21), 8_usize, Move(_8), 27_usize, Move(_27), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(0_usize, 25_usize, Move(_25), 10_usize, Move(_10), 26_usize, Move(_26), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: u64,mut _3: isize,mut _4: i64,mut _5: i8,mut _6: isize,mut _7: u16,mut _8: isize,mut _9: Adt53,mut _10: i16,mut _11: isize,mut _12: u16,mut _13: usize,mut _14: u64) -> u128 {
mir! {
type RET = u128;
let _15: Adt57;
let _16: i8;
let _17: ((i8, (u32,)), u16);
let _18: *mut u32;
let _19: [i16; 1];
let _20: f64;
let _21: &'static i64;
let _22: Adt51;
let _23: bool;
let _24: bool;
let _25: f32;
let _26: bool;
let _27: [u16; 1];
let _28: f32;
let _29: *const f64;
let _30: ();
let _31: ();
{
_4 = -4277557227907435960_i64;
_3 = _6 - _8;
_5 = -(-44_i8);
RET = 35395198159971804637145408722300628852_i128 as u128;
RET = !43010130122832671608457107771680936861_u128;
_2 = _14;
_4 = -292253099607394303_i64;
_14 = _2;
Call(_11 = fn2(_3, _4, _2, _2, _10, _1, _3, _14, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.fld0 = [_10];
_11 = -_3;
RET = 276672601763770086469773977695175461668_u128;
_8 = -_3;
_5 = 9_i8 >> _8;
_13 = 5658457993135142735_usize;
_14 = (-1298671984_i32) as u64;
_7 = _1 as u16;
_5 = -(-80_i8);
_9.fld0 = [_10];
_11 = !_3;
_3 = _1 << _5;
_8 = !_1;
RET = 60984556992009651470355427213802442585_u128 & 235761193936107041295320276319677246522_u128;
RET = 268610859393713103437963076191386155327_u128;
_4 = 1374356018922367791_i64;
_14 = !_2;
_14 = !_2;
_17.1 = 972365028_i32 as u16;
_3 = -_8;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
14600 => bb7,
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
_9.fld0 = [_10];
_18 = core::ptr::addr_of_mut!(_17.0.1.0);
_7 = _17.1;
_12 = _7;
RET = 162547740514434483523930519402840046760_u128;
_17.0.0 = _4 as i8;
_17.1 = RET as u16;
_12 = _5 as u16;
_16 = _17.0.0 & _5;
_12 = _7 - _17.1;
_2 = _4 as u64;
_7 = _12 - _17.1;
_20 = _12 as f64;
_4 = 1179638997314497609_i64 | (-1987069402176307964_i64);
(*_18) = 270086537_u32 & 2484542713_u32;
_17.0.0 = -_5;
_20 = _14 as f64;
_3 = RET as isize;
_9.fld0 = [_10];
_8 = _11;
_19 = _9.fld0;
_10 = (*_18) as i16;
Call(_20 = fn3(_8, _17, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = 107179401647207463153931770188709925749_i128 as u128;
(*_18) = _14 as u32;
_11 = _8;
_8 = _11 ^ _6;
_9.fld0 = _19;
_4 = -(-2340440968648625246_i64);
_10 = _13 as i16;
_7 = false as u16;
_8 = _6 ^ _3;
_24 = _17.0.0 < _16;
Goto(bb9)
}
bb9 = {
_12 = !_17.1;
_19 = [_10];
RET = !226800509868685706583337189182677997905_u128;
_4 = _17.0.0 as i64;
_8 = _1;
_22.fld1 = '\u{8ebe0}';
_22.fld0 = [_2,_14,_14,_2,_14,_2,_14,_2];
_23 = _20 > _20;
_7 = !_17.1;
(*_18) = 2011486078_u32 & 3961342129_u32;
_21 = &_4;
_3 = !_8;
_22.fld1 = '\u{89495}';
_20 = _10 as f64;
match _13 {
0 => bb6,
1 => bb2,
2 => bb10,
3 => bb11,
5658457993135142735 => bb13,
_ => bb12
}
}
bb10 = {
_9.fld0 = [_10];
_11 = -_3;
RET = 276672601763770086469773977695175461668_u128;
_8 = -_3;
_5 = 9_i8 >> _8;
_13 = 5658457993135142735_usize;
_14 = (-1298671984_i32) as u64;
_7 = _1 as u16;
_5 = -(-80_i8);
_9.fld0 = [_10];
_11 = !_3;
_3 = _1 << _5;
_8 = !_1;
RET = 60984556992009651470355427213802442585_u128 & 235761193936107041295320276319677246522_u128;
RET = 268610859393713103437963076191386155327_u128;
_4 = 1374356018922367791_i64;
_14 = !_2;
_14 = !_2;
_17.1 = 972365028_i32 as u16;
_3 = -_8;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
14600 => bb7,
_ => bb6
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_17.0.0 = _16;
(*_18) = RET as u32;
Goto(bb14)
}
bb14 = {
_19 = _9.fld0;
_17.1 = _7;
_11 = !_8;
_28 = _20 as f32;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(1_usize, 1_usize, Move(_1), 11_usize, Move(_11), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(1_usize, 3_usize, Move(_3), 13_usize, Move(_13), 24_usize, Move(_24), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(1_usize, 14_usize, Move(_14), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: i64,mut _3: u64,mut _4: u64,mut _5: i16,mut _6: isize,mut _7: isize,mut _8: u64,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: [i128; 5];
let _11: f64;
let _12: ();
let _13: ();
{
_4 = _8 & _3;
RET = _1;
_2 = 7934751636661239961_i64 >> _7;
_6 = _1;
RET = _9 - _1;
_9 = false as isize;
_8 = !_4;
_1 = _7 | _7;
_2 = !(-102350728598405892_i64);
_8 = !_4;
RET = (-102847434507257973599532988729209671699_i128) as isize;
match _5 {
0 => bb1,
340282366920938463463374607431768198566 => bb3,
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
_4 = _3;
_6 = 2822113897_u32 as isize;
_7 = _1 | _1;
_10 = [(-32794604714538253024924017544083482029_i128),(-120121802457860322358645183971267878563_i128),41612243305874902557284777807378632134_i128,(-22191349667922429419879058814634082850_i128),(-114276247492440740510831165749292270633_i128)];
RET = '\u{3446c}' as isize;
_9 = _7;
_1 = -_9;
_11 = 37_i8 as f64;
_3 = (-2020590865_i32) as u64;
_2 = 5453290660943370533_i64 >> _7;
_3 = _4 ^ _8;
_5 = 24472_i16 ^ (-7966_i16);
_5 = _9 as i16;
_7 = _1;
_4 = _8;
_7 = 2_usize as isize;
_9 = !_1;
_1 = -_9;
_9 = _1;
RET = !_9;
_5 = !4553_i16;
_2 = (-3308014459678467267_i64) >> _1;
_3 = !_4;
RET = _9 - _9;
_9 = RET & RET;
_9 = 3041129099_u32 as isize;
Goto(bb4)
}
bb4 = {
Call(_12 = dump_var(2_usize, 4_usize, Move(_4), 3_usize, Move(_3), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_12 = dump_var(2_usize, 5_usize, Move(_5), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: ((i8, (u32,)), u16),mut _3: u16) -> f64 {
mir! {
type RET = f64;
let _4: ((i8,), (((i8, (u32,)), u16), [bool; 8], ((i8, (u32,)), i128, &'static i64), (u16,), ((i8, (u32,)), i128, &'static i64), [i8; 7]), [i8; 7], (u16,));
let _5: Adt57;
let _6: i32;
let _7: [i8; 7];
let _8: Adt51;
let _9: Adt53;
let _10: f64;
let _11: [i16; 1];
let _12: [i128; 5];
let _13: i8;
let _14: (bool, bool, *mut u32);
let _15: Adt57;
let _16: bool;
let _17: ((i8, (u32,)), u16);
let _18: u64;
let _19: u8;
let _20: Adt47;
let _21: [bool; 8];
let _22: Adt53;
let _23: bool;
let _24: [u16; 1];
let _25: Adt48;
let _26: f64;
let _27: ((i8, (u32,)), u16);
let _28: i64;
let _29: isize;
let _30: Adt49;
let _31: [bool; 1];
let _32: f32;
let _33: [i8; 7];
let _34: (u16,);
let _35: i64;
let _36: ();
let _37: ();
{
RET = 67921303758917919112497807192996402266_u128 as f64;
RET = _3 as f64;
_1 = !(-20_isize);
RET = (-62976353591039364907960629177569608980_i128) as f64;
_2.0.1 = (1532305803_u32,);
_2.0.0 = !34_i8;
_2.0.0 = 84_i8 * 82_i8;
_2.0.0 = (-114_i8) | 84_i8;
_2.0.1 = (1960244808_u32,);
RET = _1 as f64;
_3 = _2.0.0 as u16;
RET = 149040277191990530388596417700880533183_i128 as f64;
_2.0.1.0 = !124242344_u32;
_3 = _2.1;
_2.0.1 = (895142968_u32,);
_1 = -9223372036854775807_isize;
_2.0.0 = !83_i8;
_4.1.4.0.1.0 = _2.0.1.0 & _2.0.1.0;
_4.1.4.0 = _2.0;
match _4.1.4.0.1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
895142968 => bb6,
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
_4.1.2.0.1 = _4.1.4.0.1;
_6 = 20939_i16 as i32;
_7 = [_2.0.0,_2.0.0,_2.0.0,_4.1.4.0.0,_2.0.0,_4.1.4.0.0,_4.1.4.0.0];
_4.1.5 = [_2.0.0,_2.0.0,_2.0.0,_2.0.0,_2.0.0,_4.1.4.0.0,_2.0.0];
_4.1.1 = [true,true,true,true,true,false,true,false];
_2.0.0 = !_4.1.4.0.0;
_4.0 = (_4.1.4.0.0,);
_4.1.0.0.0 = _4.0.0 >> _4.1.4.0.0;
_4.1.0 = _2;
_4.1.0.0.1 = (_4.1.2.0.1.0,);
_4.1.1 = [false,false,true,false,true,true,true,false];
_4.2 = _7;
_2 = _4.1.0;
RET = 4_usize as f64;
_4.1.3 = (_2.1,);
_7 = _4.1.5;
Goto(bb7)
}
bb7 = {
_10 = RET + RET;
_4.1.3 = (_3,);
_2 = (_4.1.0.0, _4.1.0.1);
_2.0 = (_4.1.0.0.0, _4.1.2.0.1);
_4.1.4.0 = (_4.0.0, _4.1.2.0.1);
_4.1.2.0.0 = _4.0.0 << _4.1.3.0;
_4.1.2.0 = (_2.0.0, _4.1.4.0.1);
_2.1 = _4.1.0.1;
_2.0.0 = _4.1.2.0.0 ^ _4.1.0.0.0;
_4.1.4.0.0 = -_2.0.0;
_13 = !_2.0.0;
RET = _10 * _10;
_4.1.0.0.1 = _2.0.1;
_10 = RET;
Call(_4.3.0 = fn4(_10, _13, _4.1.1, _4.2, _1, _4.1.0.1, _10, _4.0.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = _10 + _10;
_4.1.0 = (_4.1.2.0, _4.3.0);
_4.2 = [_2.0.0,_13,_4.1.4.0.0,_4.1.4.0.0,_4.1.2.0.0,_4.1.2.0.0,_13];
_4.0 = (_4.1.0.0.0,);
_7 = [_4.1.0.0.0,_4.1.0.0.0,_4.1.4.0.0,_4.0.0,_4.0.0,_4.1.4.0.0,_13];
_4.1.4.0 = (_2.0.0, _4.1.0.0.1);
_9.fld0 = [(-2677_i16)];
_4.1.3 = _4.3;
_4.1.4.1 = 124810458050593018946543337561315266013_i128 * 63547369293549932815319112363887593924_i128;
_4.1.4.1 = !(-90957206927591477080450917617646628390_i128);
_14.1 = true;
_12 = [_4.1.4.1,_4.1.4.1,_4.1.4.1,_4.1.4.1,_4.1.4.1];
_14.0 = !_14.1;
_2.1 = _4.1.3.0;
_3 = _4.3.0 << _2.1;
_4.0 = (_4.1.2.0.0,);
_4.1.1 = [_14.1,_14.1,_14.0,_14.1,_14.1,_14.1,_14.1,_14.0];
_17.0.1 = (_4.1.0.0.1.0,);
_4.1.4.0 = (_4.1.0.0.0, _4.1.0.0.1);
match _4.1.4.0.1.0 {
0 => bb9,
895142968 => bb11,
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
_11 = [(-30164_i16)];
_4.1.0.0.1 = _4.1.2.0.1;
_2.1 = _4.1.0.1;
_16 = !_14.1;
_4.1.4.0 = (_2.0.0, _2.0.1);
_21 = [_16,_14.0,_14.1,_14.1,_14.1,_16,_14.1,_16];
_24 = [_3];
_17.0.0 = _2.0.0;
_8.fld0 = [8201209202032802108_u64,8787268168252998216_u64,13469775439755506007_u64,5786436923772014394_u64,8105335619857940815_u64,16234962105554637439_u64,13879184673048911506_u64,14070241072518937186_u64];
_17.1 = _4.3.0;
_18 = 7371859674164472703_u64;
_2 = (_4.1.2.0, _4.1.0.1);
_4.1.0.1 = _4.3.0 | _3;
_3 = _4.1.0.1 << _4.1.3.0;
_9 = Adt53 { fld0: _11 };
_27.0 = (_17.0.0, _17.0.1);
_10 = RET;
_2.0.0 = _13 ^ _4.1.4.0.0;
_8.fld1 = '\u{bece0}';
_13 = _8.fld1 as i8;
_4.1.2.0 = (_17.0.0, _27.0.1);
Goto(bb12)
}
bb12 = {
_27.0.1 = _4.1.0.0.1;
_7 = _4.1.5;
_4.1.0 = (_4.1.4.0, _4.1.3.0);
_4.1.2.0.1 = _27.0.1;
_22 = _9;
_18 = 3595219034118909454_u64 >> _2.0.0;
_9.fld0 = [10162_i16];
_4.0 = (_27.0.0,);
_27.1 = (-5421788457623468453_i64) as u16;
_15 = Adt57::Variant0 { fld0: 306148821802378011388658105694136918860_u128,fld1: _4.1.3.0 };
_2.0.1 = (_4.1.4.0.1.0,);
_4.1.2.2 = &_28;
_4.1.2.0.1 = _4.1.4.0.1;
_4.1.2.0.1 = (_4.1.4.0.1.0,);
_8.fld0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_4.1.0.0.1 = (_27.0.1.0,);
match _4.1.4.0.1.0 {
0 => bb1,
1 => bb7,
2 => bb6,
3 => bb8,
895142968 => bb14,
_ => bb13
}
}
bb13 = {
_4.1.2.0.1 = _4.1.4.0.1;
_6 = 20939_i16 as i32;
_7 = [_2.0.0,_2.0.0,_2.0.0,_4.1.4.0.0,_2.0.0,_4.1.4.0.0,_4.1.4.0.0];
_4.1.5 = [_2.0.0,_2.0.0,_2.0.0,_2.0.0,_2.0.0,_4.1.4.0.0,_2.0.0];
_4.1.1 = [true,true,true,true,true,false,true,false];
_2.0.0 = !_4.1.4.0.0;
_4.0 = (_4.1.4.0.0,);
_4.1.0.0.0 = _4.0.0 >> _4.1.4.0.0;
_4.1.0 = _2;
_4.1.0.0.1 = (_4.1.2.0.1.0,);
_4.1.1 = [false,false,true,false,true,true,true,false];
_4.2 = _7;
_2 = _4.1.0;
RET = 4_usize as f64;
_4.1.3 = (_2.1,);
_7 = _4.1.5;
Goto(bb7)
}
bb14 = {
_17.0.1 = (_4.1.4.0.1.0,);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(3_usize, 1_usize, Move(_1), 17_usize, Move(_17), 18_usize, Move(_18), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(3_usize, 16_usize, Move(_16), 24_usize, Move(_24), 6_usize, Move(_6), 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f64,mut _2: i8,mut _3: [bool; 8],mut _4: [i8; 7],mut _5: isize,mut _6: u16,mut _7: f64,mut _8: i8) -> u16 {
mir! {
type RET = u16;
let _9: u128;
let _10: f64;
let _11: isize;
let _12: i16;
let _13: *mut u32;
let _14: isize;
let _15: Adt51;
let _16: u8;
let _17: i8;
let _18: char;
let _19: (i8,);
let _20: i64;
let _21: bool;
let _22: isize;
let _23: i128;
let _24: f32;
let _25: isize;
let _26: Adt42;
let _27: Adt48;
let _28: (i8,);
let _29: Adt56;
let _30: (i32, [i128; 5], u64, [bool; 1]);
let _31: isize;
let _32: [bool; 8];
let _33: char;
let _34: [bool; 8];
let _35: ((i8,), (((i8, (u32,)), u16), [bool; 8], ((i8, (u32,)), i128, &'static i64), (u16,), ((i8, (u32,)), i128, &'static i64), [i8; 7]), [i8; 7], (u16,));
let _36: [u64; 8];
let _37: char;
let _38: u8;
let _39: Adt53;
let _40: (i8, (u32,));
let _41: isize;
let _42: isize;
let _43: Adt43;
let _44: i8;
let _45: Adt45;
let _46: usize;
let _47: [bool; 8];
let _48: (((i8, (u32,)), u16), [bool; 8], ((i8, (u32,)), i128, &'static i64), (u16,), ((i8, (u32,)), i128, &'static i64), [i8; 7]);
let _49: isize;
let _50: (u32,);
let _51: ();
let _52: ();
{
_4 = [_2,_2,_8,_2,_2,_2,_8];
_5 = -9223372036854775807_isize;
_1 = _7;
_10 = (-26181_i16) as f64;
_11 = -_5;
_9 = !43493786397214073431684651026612994011_u128;
_2 = !_8;
_6 = !62098_u16;
_2 = _8 * _8;
_9 = !229969285075320973035798106633535522955_u128;
RET = _8 as u16;
_14 = !_5;
_10 = _2 as f64;
_1 = _7;
_12 = (-32705_i16) + 18906_i16;
RET = _6 | _6;
RET = !_6;
_4 = [_2,_2,_8,_8,_8,_2,_2];
_15.fld0 = [13029015466184170120_u64,8488603853829933271_u64,13911880485754261019_u64,8980375641075227226_u64,6218411229523436124_u64,6842133669121026328_u64,17223235791215599558_u64,4158093894976923095_u64];
_16 = !194_u8;
_15.fld0 = [12627652572292311777_u64,10680689647372148703_u64,11953380129900347837_u64,2271180393210075196_u64,13954612952901259656_u64,7044419064833072963_u64,3640837301586899896_u64,7626524006923808927_u64];
_1 = 11348990406982239026083438095008301282_i128 as f64;
_1 = _7 - _10;
_16 = !147_u8;
Call(RET = fn5(_15.fld0, _1, _9, _1, _4, _11, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = RET;
_15.fld0 = [13863074302375826984_u64,16729725881930647441_u64,5585215476341298013_u64,2108790705110584465_u64,9609697536471537775_u64,13302352913686888545_u64,9564501116203489425_u64,803175477166754105_u64];
_6 = RET;
_5 = _8 as isize;
RET = _6;
_1 = _2 as f64;
RET = _6;
_1 = -_7;
_1 = -_7;
_18 = '\u{3a2ac}';
_16 = 250_u8;
_2 = _8 * _8;
Call(_9 = fn8(_3, _6, _12, _3, _4, _7, _14, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = [false,false,true,true,false,false,false,true];
_15.fld0 = [9657133610130275144_u64,3587338101805989584_u64,5631743793197316157_u64,7132397342319385861_u64,15831415783014363004_u64,14499048092712160537_u64,8269671072203442546_u64,17313323781222008342_u64];
_17 = _2 & _8;
_15.fld0 = [9926630676061176790_u64,6824645585167505751_u64,12393636300682499101_u64,5549477231666415309_u64,14534224660102659712_u64,2386915879457958937_u64,18031254826092613261_u64,11374038743238840076_u64];
_2 = _17;
_19 = (_2,);
_1 = _7 + _10;
_1 = _7;
_14 = _5;
_19.0 = 6012653567528577967_i64 as i8;
_10 = (-501897592_i32) as f64;
_16 = true as u8;
_15.fld1 = _18;
_2 = _19.0 - _19.0;
_5 = _14;
Goto(bb3)
}
bb3 = {
_3 = [true,true,true,false,false,true,true,true];
_11 = -_14;
_6 = RET;
_17 = !_19.0;
_20 = 5022536067028169075_i64;
_6 = RET % RET;
_16 = 29_u8 & 196_u8;
RET = !_6;
_4 = [_2,_17,_17,_2,_2,_8,_17];
_7 = -_1;
_4 = [_2,_17,_2,_8,_2,_2,_2];
_5 = _12 as isize;
_19.0 = !_17;
_4 = [_8,_17,_19.0,_2,_19.0,_17,_8];
_12 = (-14141_i16) << _19.0;
_10 = _1 - _7;
_23 = !132774715118579382610981817936969353174_i128;
_18 = _15.fld1;
_2 = _17 ^ _19.0;
_8 = _17 - _2;
_3 = [true,true,true,true,false,false,false,false];
RET = _20 as u16;
_7 = -_10;
match _20 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
5022536067028169075 => bb10,
_ => bb9
}
}
bb4 = {
_3 = [false,false,true,true,false,false,false,true];
_15.fld0 = [9657133610130275144_u64,3587338101805989584_u64,5631743793197316157_u64,7132397342319385861_u64,15831415783014363004_u64,14499048092712160537_u64,8269671072203442546_u64,17313323781222008342_u64];
_17 = _2 & _8;
_15.fld0 = [9926630676061176790_u64,6824645585167505751_u64,12393636300682499101_u64,5549477231666415309_u64,14534224660102659712_u64,2386915879457958937_u64,18031254826092613261_u64,11374038743238840076_u64];
_2 = _17;
_19 = (_2,);
_1 = _7 + _10;
_1 = _7;
_14 = _5;
_19.0 = 6012653567528577967_i64 as i8;
_10 = (-501897592_i32) as f64;
_16 = true as u8;
_15.fld1 = _18;
_2 = _19.0 - _19.0;
_5 = _14;
Goto(bb3)
}
bb5 = {
_6 = RET;
_15.fld0 = [13863074302375826984_u64,16729725881930647441_u64,5585215476341298013_u64,2108790705110584465_u64,9609697536471537775_u64,13302352913686888545_u64,9564501116203489425_u64,803175477166754105_u64];
_6 = RET;
_5 = _8 as isize;
RET = _6;
_1 = _2 as f64;
RET = _6;
_1 = -_7;
_1 = -_7;
_18 = '\u{3a2ac}';
_16 = 250_u8;
_2 = _8 * _8;
Call(_9 = fn8(_3, _6, _12, _3, _4, _7, _14, _1), ReturnTo(bb2), UnwindUnreachable())
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
_8 = _2 ^ _2;
RET = _15.fld1 as u16;
_21 = !true;
_11 = _14;
_16 = 226_u8;
_1 = _7 + _7;
_15.fld1 = _18;
_28 = _19;
_18 = _15.fld1;
RET = 5_usize as u16;
_17 = _2;
_5 = -_14;
RET = _6 >> _11;
_15.fld1 = _18;
_14 = _11;
_16 = 67_u8 << _9;
Call(_22 = core::intrinsics::bswap(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.fld1 = _18;
_25 = _5 + _5;
_16 = 228_u8 * 178_u8;
_19 = (_2,);
_19.0 = _8;
_17 = _25 as i8;
_19.0 = _11 as i8;
_3 = [_21,_21,_21,_21,_21,_21,_21,_21];
_4 = [_2,_8,_2,_8,_17,_8,_2];
_1 = _9 as f64;
_16 = !17_u8;
_15.fld1 = _18;
_24 = 963340151_i32 as f32;
_14 = _25 - _25;
_18 = _15.fld1;
_15.fld1 = _18;
_19.0 = !_17;
_2 = _8;
_6 = RET | RET;
_31 = 12440481460722593076_u64 as isize;
_30.0 = 27655270_i32 * 1751819958_i32;
Goto(bb12)
}
bb12 = {
_21 = _10 == _10;
_4 = [_19.0,_19.0,_19.0,_28.0,_2,_2,_8];
_20 = _25 as i64;
_30.1 = [_23,_23,_23,_23,_23];
_9 = !206094385589417639436951852489442258035_u128;
_15.fld0 = [17260564626370278786_u64,3703872496388175487_u64,55777644575302506_u64,831537986437539226_u64,7548613472894657023_u64,8975419870751543585_u64,10923590382099117153_u64,7169035307952160964_u64];
_30.3 = [_21];
_35.1.2.2 = &_20;
_35.1.4.1 = 13159783057086904176_usize as i128;
_10 = _7 - _7;
_9 = 227443925287272503841417176301537976939_u128 ^ 320163887054768433607840631065871654206_u128;
_17 = _12 as i8;
_34 = [_21,_21,_21,_21,_21,_21,_21,_21];
_11 = _5;
_2 = _19.0 + _8;
_35.3 = (_6,);
_35.1.4.0.1 = (2216524097_u32,);
_15.fld0 = [17543562233420652341_u64,16211671737588770660_u64,1762601127864575424_u64,14159840412870705911_u64,12416042489777479703_u64,12586395279396495042_u64,10706611283093389780_u64,9443115377201781791_u64];
_35.1.3.0 = !_35.3.0;
_40.1.0 = _35.1.4.0.1.0 >> _2;
Goto(bb13)
}
bb13 = {
_15.fld0 = [12871747964868704842_u64,15256205295020525731_u64,5373531053242215966_u64,16961638214781953756_u64,14869239667713116459_u64,2093150813688536721_u64,14239830671797996209_u64,1709093632258524577_u64];
_33 = _15.fld1;
_42 = !_14;
_11 = !_42;
_3 = [_21,_21,_21,_21,_21,_21,_21,_21];
_23 = _24 as i128;
_35.1.2.0.0 = _16 as i8;
_35.2 = _4;
_35.3 = (_6,);
_35.1.0.0 = (_2, _40.1);
_35.1.3 = _35.3;
RET = _20 as u16;
_35.3 = _35.1.3;
_28.0 = -_2;
Goto(bb14)
}
bb14 = {
_30.2 = 4855284042969333378_u64;
_15.fld1 = _33;
_35.1.0.0.1 = (_40.1.0,);
_35.1.4.0.1.0 = _30.2 as u32;
RET = !_35.3.0;
_14 = !_42;
_48.4.0.1.0 = _35.1.0.0.1.0 << _6;
_35.1.1 = [_21,_21,_21,_21,_21,_21,_21,_21];
_30.0 = !(-1466508735_i32);
_11 = _5;
_37 = _15.fld1;
_48.2.0 = (_28.0, _40.1);
_48.2.1 = _20 as i128;
_35.1.0.0.1.0 = _48.4.0.1.0 & _40.1.0;
_48.4.0.1 = _35.1.0.0.1;
_35.0 = _28;
_42 = _24 as isize;
_47 = [_21,_21,_21,_21,_21,_21,_21,_21];
_35.1.2.0.1 = (_35.1.0.0.1.0,);
_20 = 7928665721048006245_i64;
_48.2.1 = _35.1.4.1;
_36 = [_30.2,_30.2,_30.2,_30.2,_30.2,_30.2,_30.2,_30.2];
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(4_usize, 30_usize, Move(_30), 22_usize, Move(_22), 31_usize, Move(_31), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(4_usize, 28_usize, Move(_28), 4_usize, Move(_4), 17_usize, Move(_17), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(4_usize, 20_usize, Move(_20), 23_usize, Move(_23), 47_usize, Move(_47), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(4_usize, 21_usize, Move(_21), 8_usize, Move(_8), 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u64; 8],mut _2: f64,mut _3: u128,mut _4: f64,mut _5: [i8; 7],mut _6: isize,mut _7: f64,mut _8: [bool; 8]) -> u16 {
mir! {
type RET = u16;
let _9: i64;
let _10: u8;
let _11: u64;
let _12: f32;
let _13: Adt51;
let _14: Adt49;
let _15: isize;
let _16: f32;
let _17: Adt56;
let _18: (u32,);
let _19: f32;
let _20: (i32, [i128; 5], u64, [bool; 1]);
let _21: f32;
let _22: (i8,);
let _23: bool;
let _24: isize;
let _25: (u32,);
let _26: (((i8, (u32,)), u16), [bool; 8], ((i8, (u32,)), i128, &'static i64), (u16,), ((i8, (u32,)), i128, &'static i64), [i8; 7]);
let _27: [bool; 1];
let _28: u8;
let _29: ();
let _30: ();
{
_7 = _2 + _4;
_3 = 275207123690771465346793392006718933114_u128 >> _6;
RET = 22910_u16 * 37323_u16;
RET = 22397_u16 | 64159_u16;
Call(_1 = fn6(_4, _7, _2, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = (-3153474733664272738_i64) | 8362612862372261673_i64;
_6 = 5_isize * 9223372036854775807_isize;
_3 = _9 as u128;
_6 = (-9223372036854775808_isize) >> _9;
_4 = _7 - _7;
Call(RET = core::intrinsics::bswap(19063_u16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = -_4;
RET = !32118_u16;
_6 = !(-9223372036854775808_isize);
_8 = [true,true,false,false,false,false,true,false];
RET = 39887_u16 >> _3;
_9 = (-6106154184813700384_i64);
_2 = -_4;
_5 = [73_i8,(-56_i8),(-71_i8),32_i8,(-26_i8),48_i8,(-16_i8)];
RET = 24809_u16 - 10794_u16;
_10 = 1009741088_u32 as u8;
_9 = true as i64;
_10 = !226_u8;
_9 = RET as i64;
_9 = -6650647649535322102_i64;
_3 = 91871731068900209793325715126360189715_u128;
_10 = 78_u8;
RET = !52127_u16;
RET = _3 as u16;
_7 = -_4;
_2 = _7;
_10 = 36_u8 & 247_u8;
_7 = _9 as f64;
_3 = !238754097745763803941105830220736282524_u128;
Goto(bb3)
}
bb3 = {
RET = !37971_u16;
_8 = [false,false,false,true,false,true,false,true];
_5 = [(-68_i8),(-10_i8),62_i8,114_i8,66_i8,(-98_i8),72_i8];
_6 = 9223372036854775807_isize;
_3 = !170738562041028881434266336121601745666_u128;
_12 = _2 as f32;
_12 = (-56567830162025182570513704832072733475_i128) as f32;
_9 = !962722743145573460_i64;
_9 = (-5974389218882691313_i64) >> _10;
_12 = 2900591873_u32 as f32;
Goto(bb4)
}
bb4 = {
_8 = [false,true,false,false,false,true,true,true];
_6 = 9223372036854775807_isize + (-9223372036854775808_isize);
_9 = (-6053775569387486353_i64) & (-1333879349378199427_i64);
_1 = [11548307630851269541_u64,11316591977146032425_u64,5422251396481142173_u64,14072706129088791461_u64,4535259934695958437_u64,5906145197557083547_u64,8523035500014723917_u64,14470198426561900890_u64];
_10 = 138_u8;
_10 = !42_u8;
_14.fld1 = [(-70_i8),(-74_i8),46_i8,(-41_i8),84_i8,(-66_i8),55_i8];
_8 = [false,false,true,true,false,true,false,false];
_13 = Adt51 { fld0: _1,fld1: '\u{d71bf}' };
_1 = [8861830037117951801_u64,1778106819534628339_u64,9556765237322155566_u64,6917676460555420308_u64,16027719694571998805_u64,12228486927278738722_u64,15559510004009243645_u64,16348805626714948869_u64];
RET = 2651_u16 * 38115_u16;
_5 = [78_i8,101_i8,(-123_i8),(-90_i8),(-113_i8),(-45_i8),111_i8];
RET = !1922_u16;
_13 = Adt51 { fld0: _1,fld1: '\u{ccffa}' };
Goto(bb5)
}
bb5 = {
_9 = !(-61751321493989074_i64);
_12 = 9436824121568909714_u64 as f32;
_13.fld0 = [7730146277626773787_u64,6065369739742328578_u64,114714786349144685_u64,445590478801284973_u64,7172707727663870699_u64,12150762050052402279_u64,1757538579403391840_u64,16023292769779781818_u64];
RET = _10 as u16;
_14.fld4 = _4 + _2;
_10 = 113_u8;
_14.fld3 = core::ptr::addr_of_mut!(_18.0);
_19 = -_12;
_3 = 3613687759939463716_u64 as u128;
_15 = _6 - _6;
_12 = _19;
_19 = _12;
_14.fld0 = core::ptr::addr_of_mut!(_18);
RET = 3_u16;
_4 = _2;
RET = 6181_u16 << _9;
_20.1 = [(-148654859604916262447520409856444980108_i128),(-71956822991141167037985156787333522261_i128),(-23779203680741303873941646300539204755_i128),84769941697712955423214136684215661731_i128,109441016569881762190161706059310950315_i128];
_11 = 16377562137995774932_u64;
_18 = (1576592407_u32,);
Call(_2 = fn7(_4, _8, _14.fld4, _13.fld0, _14.fld4, _4, _13.fld1, _13.fld1, _1, _14.fld4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = !207_u8;
_20.0 = _6 as i32;
_5 = [(-31_i8),(-19_i8),(-13_i8),91_i8,124_i8,(-65_i8),(-1_i8)];
_4 = _14.fld4;
_19 = _12;
_19 = -_12;
_16 = _19;
_16 = _19;
_22 = (94_i8,);
_20.0 = (-174633449_i32) - (-440293206_i32);
_17 = Adt56::Variant2 { fld0: true,fld1: _16,fld2: _20.1,fld3: _14.fld4,fld4: _14.fld1 };
place!(Field::<[i128; 5]>(Variant(_17, 2), 2)) = _20.1;
_23 = _4 <= Field::<f64>(Variant(_17, 2), 3);
_21 = _20.0 as f32;
_18 = (87814132_u32,);
_20.3 = [_23];
_3 = _9 as u128;
_16 = _11 as f32;
_24 = _15;
place!(Field::<bool>(Variant(_17, 2), 0)) = _23;
_13.fld0 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = 42950_u16;
_2 = -_4;
SetDiscriminant(_17, 1);
place!(Field::<Adt53>(Variant(_17, 1), 2)).fld0 = [1767_i16];
match _18.0 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
87814132 => bb15,
_ => bb14
}
}
bb7 = {
_9 = !(-61751321493989074_i64);
_12 = 9436824121568909714_u64 as f32;
_13.fld0 = [7730146277626773787_u64,6065369739742328578_u64,114714786349144685_u64,445590478801284973_u64,7172707727663870699_u64,12150762050052402279_u64,1757538579403391840_u64,16023292769779781818_u64];
RET = _10 as u16;
_14.fld4 = _4 + _2;
_10 = 113_u8;
_14.fld3 = core::ptr::addr_of_mut!(_18.0);
_19 = -_12;
_3 = 3613687759939463716_u64 as u128;
_15 = _6 - _6;
_12 = _19;
_19 = _12;
_14.fld0 = core::ptr::addr_of_mut!(_18);
RET = 3_u16;
_4 = _2;
RET = 6181_u16 << _9;
_20.1 = [(-148654859604916262447520409856444980108_i128),(-71956822991141167037985156787333522261_i128),(-23779203680741303873941646300539204755_i128),84769941697712955423214136684215661731_i128,109441016569881762190161706059310950315_i128];
_11 = 16377562137995774932_u64;
_18 = (1576592407_u32,);
Call(_2 = fn7(_4, _8, _14.fld4, _13.fld0, _14.fld4, _4, _13.fld1, _13.fld1, _1, _14.fld4), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_8 = [false,true,false,false,false,true,true,true];
_6 = 9223372036854775807_isize + (-9223372036854775808_isize);
_9 = (-6053775569387486353_i64) & (-1333879349378199427_i64);
_1 = [11548307630851269541_u64,11316591977146032425_u64,5422251396481142173_u64,14072706129088791461_u64,4535259934695958437_u64,5906145197557083547_u64,8523035500014723917_u64,14470198426561900890_u64];
_10 = 138_u8;
_10 = !42_u8;
_14.fld1 = [(-70_i8),(-74_i8),46_i8,(-41_i8),84_i8,(-66_i8),55_i8];
_8 = [false,false,true,true,false,true,false,false];
_13 = Adt51 { fld0: _1,fld1: '\u{d71bf}' };
_1 = [8861830037117951801_u64,1778106819534628339_u64,9556765237322155566_u64,6917676460555420308_u64,16027719694571998805_u64,12228486927278738722_u64,15559510004009243645_u64,16348805626714948869_u64];
RET = 2651_u16 * 38115_u16;
_5 = [78_i8,101_i8,(-123_i8),(-90_i8),(-113_i8),(-45_i8),111_i8];
RET = !1922_u16;
_13 = Adt51 { fld0: _1,fld1: '\u{ccffa}' };
Goto(bb5)
}
bb9 = {
RET = !37971_u16;
_8 = [false,false,false,true,false,true,false,true];
_5 = [(-68_i8),(-10_i8),62_i8,114_i8,66_i8,(-98_i8),72_i8];
_6 = 9223372036854775807_isize;
_3 = !170738562041028881434266336121601745666_u128;
_12 = _2 as f32;
_12 = (-56567830162025182570513704832072733475_i128) as f32;
_9 = !962722743145573460_i64;
_9 = (-5974389218882691313_i64) >> _10;
_12 = 2900591873_u32 as f32;
Goto(bb4)
}
bb10 = {
_7 = -_4;
RET = !32118_u16;
_6 = !(-9223372036854775808_isize);
_8 = [true,true,false,false,false,false,true,false];
RET = 39887_u16 >> _3;
_9 = (-6106154184813700384_i64);
_2 = -_4;
_5 = [73_i8,(-56_i8),(-71_i8),32_i8,(-26_i8),48_i8,(-16_i8)];
RET = 24809_u16 - 10794_u16;
_10 = 1009741088_u32 as u8;
_9 = true as i64;
_10 = !226_u8;
_9 = RET as i64;
_9 = -6650647649535322102_i64;
_3 = 91871731068900209793325715126360189715_u128;
_10 = 78_u8;
RET = !52127_u16;
RET = _3 as u16;
_7 = -_4;
_2 = _7;
_10 = 36_u8 & 247_u8;
_7 = _9 as f64;
_3 = !238754097745763803941105830220736282524_u128;
Goto(bb3)
}
bb11 = {
_9 = (-3153474733664272738_i64) | 8362612862372261673_i64;
_6 = 5_isize * 9223372036854775807_isize;
_3 = _9 as u128;
_6 = (-9223372036854775808_isize) >> _9;
_4 = _7 - _7;
Call(RET = core::intrinsics::bswap(19063_u16), ReturnTo(bb2), UnwindUnreachable())
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
_18 = (1102045518_u32,);
_18 = (1688368596_u32,);
_21 = _19 - _12;
place!(Field::<(i8, (u32,))>(Variant(_17, 1), 4)).1.0 = _18.0 + _18.0;
_26.0.0.1 = (_18.0,);
_26.2.0 = (_22.0, Field::<(i8, (u32,))>(Variant(_17, 1), 4).1);
_3 = 6_usize as u128;
_25 = (_26.0.0.1.0,);
_13 = Adt51 { fld0: _1,fld1: '\u{a8b83}' };
_26.0.1 = !RET;
_14.fld0 = core::ptr::addr_of_mut!(place!(Field::<(i8, (u32,))>(Variant(_17, 1), 4)).1);
_27 = [_23];
_13.fld0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_26.4.0 = (_26.2.0.0, _26.2.0.1);
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(5_usize, 27_usize, Move(_27), 3_usize, Move(_3), 8_usize, Move(_8), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(5_usize, 15_usize, Move(_15), 5_usize, Move(_5), 10_usize, Move(_10), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: f64) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _5: usize;
let _6: Adt53;
let _7: char;
let _8: f64;
let _9: ((i8, (u32,)), u16);
let _10: i8;
let _11: [i8; 7];
let _12: i128;
let _13: char;
let _14: char;
let _15: usize;
let _16: isize;
let _17: Adt53;
let _18: isize;
let _19: (i8,);
let _20: (u16,);
let _21: [u64; 8];
let _22: u32;
let _23: *mut (u32,);
let _24: ();
let _25: ();
{
RET = [16234568812807159845_u64,4932808104810479298_u64,13054219561601345660_u64,5657821493728552274_u64,4222518937139982096_u64,13096575956638416699_u64,3005815209917793039_u64,4513916912102748444_u64];
_7 = '\u{ec39e}';
_4 = _2 + _2;
_8 = _3;
_5 = 4765071653338132522_u64 as usize;
_6.fld0 = [5544_i16];
_5 = 11285900803101038996_usize | 3_usize;
Goto(bb1)
}
bb1 = {
_3 = _4 - _2;
_3 = 97_isize as f64;
_7 = '\u{b740e}';
_2 = _8 * _4;
_7 = '\u{1f680}';
_9.0.1.0 = 763854153_u32 * 3950529322_u32;
_9.1 = !17782_u16;
_1 = 14016212213200824746_u64 as f64;
_9.0.1 = (2474959762_u32,);
_9.0.0 = (-91_i8) * (-75_i8);
_6.fld0 = [5110_i16];
_8 = -_4;
RET = [1127261417585791212_u64,14197260768428493848_u64,10327595746528703553_u64,5883874880121036370_u64,18085437748544318443_u64,11474396102302811066_u64,6125768869088922794_u64,13439230289114622988_u64];
_6.fld0 = [(-14257_i16)];
_9.1 = 19336_u16 - 9441_u16;
_11 = [_9.0.0,_9.0.0,_9.0.0,_9.0.0,_9.0.0,_9.0.0,_9.0.0];
_9.0.1.0 = 4229323380_u32 | 2009929863_u32;
_10 = _9.0.0 ^ _9.0.0;
_6.fld0 = [4614_i16];
RET = [13110829716725197997_u64,4281455146420361681_u64,10594401732585193706_u64,5235662076262772292_u64,3226642766319202362_u64,5310382995340182067_u64,1680508315306074919_u64,657236721723876268_u64];
_9.0.0 = _10 & _10;
_9.0.1 = (1652579648_u32,);
_6.fld0 = [7128_i16];
_12 = !89532641813688072108391759823291986542_i128;
_12 = 67920059248828953166310917198501636571_u128 as i128;
_6.fld0 = [(-13401_i16)];
_10 = _9.0.0 | _9.0.0;
match _9.0.1.0 {
0 => bb2,
1652579648 => bb4,
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
_2 = -_8;
_4 = _9.0.0 as f64;
RET = [10618623156358776368_u64,16782097045275497934_u64,9599056063034745109_u64,6481052715557893739_u64,2274545989277978197_u64,3253636751225587559_u64,12562372629847261970_u64,3066235293949385541_u64];
_9.0.0 = -_10;
Goto(bb5)
}
bb5 = {
_5 = _8 as usize;
_9.0.1 = (2997262421_u32,);
RET = [17541559208280919610_u64,17636484101950901592_u64,215623314302755217_u64,15564219263432809049_u64,1352552826414998264_u64,2702228827111193843_u64,12403039017934803063_u64,3622230501895656386_u64];
_14 = _7;
_11 = [_10,_9.0.0,_9.0.0,_9.0.0,_10,_10,_10];
_17 = _6;
RET = [7163177038538538347_u64,8443663559335137240_u64,11548152823019723895_u64,7729648748414761819_u64,7206409163431454524_u64,16020977143248461667_u64,9411416861161578807_u64,10446502344282914344_u64];
_7 = _14;
_13 = _7;
_12 = !27924881464327146503678582928704816515_i128;
match _9.0.1.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
2997262421 => bb12,
_ => bb11
}
}
bb6 = {
_2 = -_8;
_4 = _9.0.0 as f64;
RET = [10618623156358776368_u64,16782097045275497934_u64,9599056063034745109_u64,6481052715557893739_u64,2274545989277978197_u64,3253636751225587559_u64,12562372629847261970_u64,3066235293949385541_u64];
_9.0.0 = -_10;
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_3 = _4 - _2;
_3 = 97_isize as f64;
_7 = '\u{b740e}';
_2 = _8 * _4;
_7 = '\u{1f680}';
_9.0.1.0 = 763854153_u32 * 3950529322_u32;
_9.1 = !17782_u16;
_1 = 14016212213200824746_u64 as f64;
_9.0.1 = (2474959762_u32,);
_9.0.0 = (-91_i8) * (-75_i8);
_6.fld0 = [5110_i16];
_8 = -_4;
RET = [1127261417585791212_u64,14197260768428493848_u64,10327595746528703553_u64,5883874880121036370_u64,18085437748544318443_u64,11474396102302811066_u64,6125768869088922794_u64,13439230289114622988_u64];
_6.fld0 = [(-14257_i16)];
_9.1 = 19336_u16 - 9441_u16;
_11 = [_9.0.0,_9.0.0,_9.0.0,_9.0.0,_9.0.0,_9.0.0,_9.0.0];
_9.0.1.0 = 4229323380_u32 | 2009929863_u32;
_10 = _9.0.0 ^ _9.0.0;
_6.fld0 = [4614_i16];
RET = [13110829716725197997_u64,4281455146420361681_u64,10594401732585193706_u64,5235662076262772292_u64,3226642766319202362_u64,5310382995340182067_u64,1680508315306074919_u64,657236721723876268_u64];
_9.0.0 = _10 & _10;
_9.0.1 = (1652579648_u32,);
_6.fld0 = [7128_i16];
_12 = !89532641813688072108391759823291986542_i128;
_12 = 67920059248828953166310917198501636571_u128 as i128;
_6.fld0 = [(-13401_i16)];
_10 = _9.0.0 | _9.0.0;
match _9.0.1.0 {
0 => bb2,
1652579648 => bb4,
_ => bb3
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_9.0.1 = (3664719561_u32,);
_9.1 = 3467_u16;
_8 = 56_isize as f64;
_17.fld0 = _6.fld0;
_17 = _6;
_15 = 167483860711976103102472469491140836790_u128 as usize;
_13 = _14;
_15 = _5 * _5;
RET = [8463641610692416999_u64,1983284940598835165_u64,11621967888919604051_u64,781056437561237248_u64,11761236147038617651_u64,17832802853174525546_u64,9475072709298967555_u64,8695079897079051249_u64];
RET = [6424866342511807089_u64,14931934648543905096_u64,16187107467591510202_u64,8904286694403705922_u64,11712084688106617472_u64,12797124814627399744_u64,3326467868221170298_u64,9851454233744403954_u64];
_5 = _15;
_9.0.1 = (3613833981_u32,);
_19 = (_10,);
match _9.0.1.0 {
0 => bb13,
3613833981 => bb15,
_ => bb14
}
}
bb13 = {
_5 = _8 as usize;
_9.0.1 = (2997262421_u32,);
RET = [17541559208280919610_u64,17636484101950901592_u64,215623314302755217_u64,15564219263432809049_u64,1352552826414998264_u64,2702228827111193843_u64,12403039017934803063_u64,3622230501895656386_u64];
_14 = _7;
_11 = [_10,_9.0.0,_9.0.0,_9.0.0,_10,_10,_10];
_17 = _6;
RET = [7163177038538538347_u64,8443663559335137240_u64,11548152823019723895_u64,7729648748414761819_u64,7206409163431454524_u64,16020977143248461667_u64,9411416861161578807_u64,10446502344282914344_u64];
_7 = _14;
_13 = _7;
_12 = !27924881464327146503678582928704816515_i128;
match _9.0.1.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
2997262421 => bb12,
_ => bb11
}
}
bb14 = {
Return()
}
bb15 = {
_14 = _7;
_8 = -_2;
_16 = 9223372036854775807_isize & (-2_isize);
_9.0.1.0 = !4034286831_u32;
_18 = _4 as isize;
_5 = !_15;
_13 = _14;
_2 = 122841445001418855252435072554392018916_u128 as f64;
_5 = _15 & _15;
RET = [6220038468879607803_u64,9733312148772704487_u64,8756910124775848723_u64,6579450110133705075_u64,10735609286088179690_u64,11337379100381559511_u64,2365918530622884682_u64,4765302104762323508_u64];
_10 = _4 as i8;
_9.1 = 53230_u16;
_9.0.1.0 = 2542874385_u32 >> _5;
_6 = _17;
_13 = _14;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(6_usize, 12_usize, Move(_12), 7_usize, Move(_7), 16_usize, Move(_16), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(6_usize, 5_usize, Move(_5), 18_usize, Move(_18), 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: f64,mut _2: [bool; 8],mut _3: f64,mut _4: [u64; 8],mut _5: f64,mut _6: f64,mut _7: char,mut _8: char,mut _9: [u64; 8],mut _10: f64) -> f64 {
mir! {
type RET = f64;
let _11: ();
let _12: ();
{
_5 = 8517698748084836446_u64 as f64;
RET = -_3;
_9 = [10018465160492407582_u64,688386785896796862_u64,13884057297384618033_u64,9604796031874788745_u64,14245842727433913536_u64,12551445171688548555_u64,15539796871204723541_u64,516386683350336238_u64];
_7 = _8;
_1 = _10 * _10;
_1 = RET;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(7_usize, 7_usize, Move(_7), 8_usize, Move(_8), 12_usize, _12, 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [bool; 8],mut _2: u16,mut _3: i16,mut _4: [bool; 8],mut _5: [i8; 7],mut _6: f64,mut _7: isize,mut _8: f64) -> u128 {
mir! {
type RET = u128;
let _9: u8;
let _10: (i8,);
let _11: f64;
let _12: f32;
let _13: (bool, bool, *mut u32);
let _14: (u32,);
let _15: [bool; 1];
let _16: char;
let _17: i16;
let _18: *mut (u32,);
let _19: Adt43;
let _20: (u16,);
let _21: (u32,);
let _22: isize;
let _23: i128;
let _24: i8;
let _25: isize;
let _26: Adt55;
let _27: i32;
let _28: u32;
let _29: i16;
let _30: [i128; 5];
let _31: i128;
let _32: [u16; 1];
let _33: [bool; 8];
let _34: i128;
let _35: [bool; 8];
let _36: (u32,);
let _37: ((i8, (u32,)), u16);
let _38: f64;
let _39: [usize; 4];
let _40: (u16,);
let _41: isize;
let _42: Adt49;
let _43: ();
let _44: ();
{
_2 = 14522_u16 ^ 3264_u16;
_8 = (-7198243231328193575_i64) as f64;
RET = 11726890807012266733_u64 as u128;
_7 = _6 as isize;
_3 = !3397_i16;
RET = 1982658570_i32 as u128;
_1 = [false,true,false,true,true,true,false,true];
RET = 73620358632131929237965462357774613689_u128;
_8 = 18136687917944298329_u64 as f64;
RET = 224268200156355747993409127699472575629_u128;
RET = '\u{def18}' as u128;
RET = 7714901330897693349_u64 as u128;
_7 = _2 as isize;
RET = 69651440165028839620151480904544188896_u128;
_7 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_1 = [true,true,true,true,true,true,true,false];
_8 = _6;
_4 = _1;
_8 = 4_usize as f64;
_11 = 3751054570108956140_i64 as f64;
_4 = _1;
_2 = '\u{18dd2}' as u16;
_12 = (-4846540071043266512_i64) as f32;
_4 = [false,true,true,true,false,false,false,false];
RET = 160_u8 as u128;
_12 = 1_i8 as f32;
Goto(bb1)
}
bb1 = {
_11 = _6;
_10 = ((-22_i8),);
_4 = [false,true,true,true,true,true,false,true];
_7 = '\u{9b9c2}' as isize;
RET = !276247434536684951722837858367551164633_u128;
_8 = -_11;
_2 = 5795_u16 & 33273_u16;
_8 = _11 - _6;
_12 = 8016698810906117996_u64 as f32;
_13.0 = false;
_10 = (57_i8,);
_14 = (3200004384_u32,);
_14.0 = !753299962_u32;
_9 = !36_u8;
_18 = core::ptr::addr_of_mut!(_14);
RET = !217941821835644760842295001546572403493_u128;
RET = _10.0 as u128;
_15 = [_13.0];
_14.0 = _13.0 as u32;
_13.1 = _13.0;
_15 = [_13.0];
_13.2 = core::ptr::addr_of_mut!(_14.0);
match _10.0 {
0 => bb2,
1 => bb3,
2 => bb4,
57 => bb6,
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
(*_18) = (1772080314_u32,);
_14 = (2420006743_u32,);
_5 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_18 = core::ptr::addr_of_mut!((*_18));
_13.2 = core::ptr::addr_of_mut!(_14.0);
_16 = '\u{e7d35}';
_14.0 = _2 as u32;
(*_18).0 = 4184185115_u32;
(*_18).0 = _13.0 as u32;
_13.1 = !_13.0;
_13.0 = !_13.1;
_13.2 = core::ptr::addr_of_mut!((*_18).0);
_6 = _11 - _8;
(*_18) = (1685395429_u32,);
_13.0 = _13.1;
_12 = RET as f32;
_16 = '\u{72938}';
(*_18).0 = 3668908192_u32;
_19 = Adt43::Variant2 { fld0: _13.2,fld1: _13 };
place!(Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1)).1 = Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1).0;
_4 = _1;
_13.1 = Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1).0 | Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1).1;
_14.0 = 2041147587_u32;
_12 = (-51914699_i32) as f32;
_20.0 = 128177770061520556544724414400326189604_i128 as u16;
_7 = !9223372036854775807_isize;
place!(Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1)).1 = _13.1;
_13.2 = core::ptr::addr_of_mut!(_14.0);
Goto(bb7)
}
bb7 = {
(*_18).0 = _3 as u32;
_14.0 = 1106093741_u32;
SetDiscriminant(_19, 3);
_2 = _20.0;
place!(Field::<*mut f64>(Variant(_19, 3), 4)) = core::ptr::addr_of_mut!(_8);
RET = _3 as u128;
_21 = _14;
place!(Field::<[u16; 1]>(Variant(_19, 3), 2)) = [_20.0];
place!(Field::<(u16,)>(Variant(_19, 3), 1)) = (_2,);
place!(Field::<(u32,)>(Variant(_19, 3), 6)) = ((*_18).0,);
_13.0 = _14.0 == (*_18).0;
Call(place!(Field::<u16>(Variant(_19, 3), 0)) = fn9(_13.0, (*_18).0, Field::<*mut f64>(Variant(_19, 3), 4), _8, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20 = Field::<(u16,)>(Variant(_19, 3), 1);
place!(Field::<[bool; 8]>(Variant(_19, 3), 5)) = _1;
_9 = !89_u8;
_21 = (_14.0,);
place!(Field::<u16>(Variant(_19, 3), 0)) = Field::<(u16,)>(Variant(_19, 3), 1).0;
place!(Field::<*mut (u32,)>(Variant(_19, 3), 3)) = _18;
_27 = (-1395699989_i32) << _14.0;
_17 = _16 as i16;
_26.fld5 = RET;
RET = _26.fld5;
_26.fld7.0 = _9 as i8;
_26.fld7 = (_10.0,);
(*_18) = (_21.0,);
_4 = Field::<[bool; 8]>(Variant(_19, 3), 5);
_6 = 3095386021121752209_i64 as f64;
place!(Field::<(u32,)>(Variant(_19, 3), 6)) = (*_18);
place!(Field::<(u16,)>(Variant(_19, 3), 1)).0 = _2 >> _21.0;
_7 = _8 as isize;
_26.fld4 = -_8;
_25 = (-474294698002268428_i64) as isize;
_21.0 = !_14.0;
_26.fld1 = _16;
_30 = [(-49776704839752180781860864703668642149_i128),109767314804997575061110401285853398284_i128,(-162608038955949258791640283411054553638_i128),(-91827467802158565994990651065296929788_i128),(-131637486548861873615338005826234289340_i128)];
_13.0 = !_13.1;
place!(Field::<*mut (u32,)>(Variant(_19, 3), 3)) = core::ptr::addr_of_mut!((*_18));
_26.fld5 = RET & RET;
match (*_18).0 {
0 => bb3,
1106093741 => bb9,
_ => bb6
}
}
bb9 = {
_24 = _26.fld7.0;
(*_18) = (Field::<(u32,)>(Variant(_19, 3), 6).0,);
Goto(bb10)
}
bb10 = {
_3 = _17;
_26.fld4 = _3 as f64;
_23 = (-127764333732885807762650869801351320001_i128) >> _7;
place!(Field::<(u32,)>(Variant(_19, 3), 6)) = ((*_18).0,);
place!(Field::<*mut [bool; 1]>(Variant(_19, 3), 7)) = core::ptr::addr_of_mut!(_15);
_8 = _11 + _11;
_15 = [_13.0];
_11 = _9 as f64;
_10 = (_26.fld7.0,);
_22 = _7 - _7;
_31 = -_23;
(*_18) = (_21.0,);
_32 = Field::<[u16; 1]>(Variant(_19, 3), 2);
_29 = _3;
place!(Field::<*mut (u32,)>(Variant(_19, 3), 3)) = core::ptr::addr_of_mut!((*_18));
_21.0 = _11 as u32;
_26.fld4 = _6 + _8;
_24 = _3 as i8;
_26.fld6.fld1 = _26.fld1;
place!(Field::<[u16; 1]>(Variant(_19, 3), 2)) = [Field::<(u16,)>(Variant(_19, 3), 1).0];
_14.0 = _21.0 * Field::<(u32,)>(Variant(_19, 3), 6).0;
_12 = _26.fld7.0 as f32;
_26.fld6.fld0 = [15656097347713432616_u64,9331665286325703172_u64,15059347156399753985_u64,15989003012208284163_u64,6151323604437254595_u64,17194394953229176771_u64,1947864359951570996_u64,5518440429275222795_u64];
(*_18).0 = _21.0 * Field::<(u32,)>(Variant(_19, 3), 6).0;
_20 = (Field::<(u16,)>(Variant(_19, 3), 1).0,);
_13.0 = _13.1;
(*_18) = (_21.0,);
_33 = [_13.1,_13.1,_13.0,_13.1,_13.1,_13.1,_13.0,_13.0];
Goto(bb11)
}
bb11 = {
_26.fld2 = core::ptr::addr_of!(_11);
_16 = _26.fld6.fld1;
_10 = (_26.fld7.0,);
_21 = (*_18);
(*_18) = (_21.0,);
_20 = Field::<(u16,)>(Variant(_19, 3), 1);
RET = !_26.fld5;
_23 = _27 as i128;
_7 = _27 as isize;
_32 = [_2];
(*_18) = (_21.0,);
_33 = _1;
_11 = -_26.fld4;
_21.0 = !Field::<(u32,)>(Variant(_19, 3), 6).0;
_31 = _23 & _23;
_11 = -_26.fld4;
_10 = (_26.fld7.0,);
_26.fld6.fld0 = [3179103844318841267_u64,10724496727498998601_u64,18185247251962086963_u64,18367964677702973718_u64,12184125210648170153_u64,5856564149575948020_u64,14943053864073504693_u64,13454694492799688129_u64];
match Field::<(u32,)>(Variant(_19, 3), 6).0 {
0 => bb9,
1106093741 => bb12,
_ => bb8
}
}
bb12 = {
place!(Field::<u16>(Variant(_19, 3), 0)) = !_20.0;
_13.1 = !_13.0;
_17 = !_3;
_37.0.1.0 = !Field::<(u32,)>(Variant(_19, 3), 6).0;
SetDiscriminant(_19, 2);
place!(Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1)).1 = _13.0;
place!(Field::<*mut u32>(Variant(_19, 2), 0)) = core::ptr::addr_of_mut!(_36.0);
_17 = _31 as i16;
_37.1 = !_20.0;
_18 = core::ptr::addr_of_mut!(_37.0.1);
_35 = [_13.0,_13.1,_13.0,_13.1,Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1).1,_13.1,_13.1,_13.0];
place!(Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1)) = _13;
_37.0.0 = !_26.fld7.0;
_26.fld3 = Adt46::Variant0 { fld0: _32,fld1: _37.0,fld2: _18,fld3: _37,fld4: _26.fld6.fld0,fld5: _27 };
_28 = _17 as u32;
_20 = (_2,);
_10.0 = _22 as i8;
RET = !_26.fld5;
_37.0 = Field::<(i8, (u32,))>(Variant(_26.fld3, 0), 1);
_14 = (_28,);
_41 = _22;
place!(Field::<i32>(Variant(_26.fld3, 0), 5)) = _27;
_13.1 = Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1).1;
_13.2 = Field::<(bool, bool, *mut u32)>(Variant(_19, 2), 1).2;
_13.0 = _10.0 < _10.0;
SetDiscriminant(_19, 1);
_37.0.0 = _10.0 << _10.0;
match _26.fld7.0 {
0 => bb4,
57 => bb13,
_ => bb7
}
}
bb13 = {
_5 = [_37.0.0,_10.0,_37.0.0,_10.0,_10.0,_10.0,_37.0.0];
_24 = _10.0 >> _10.0;
_42.fld4 = 2360275404949702319_u64 as f64;
_42.fld4 = -_26.fld4;
_40 = _20;
place!(Field::<((i8, (u32,)), u16)>(Variant(_26.fld3, 0), 3)).0.1.0 = _14.0;
place!(Field::<[u64; 8]>(Variant(_19, 1), 2)) = [14374094139995842053_u64,3404387099108876219_u64,11669389299497996379_u64,3942728623651025611_u64,10533853446940022878_u64,11493887100510601433_u64,11411650719168584380_u64,16075014038987984637_u64];
_42.fld1 = _5;
Goto(bb14)
}
bb14 = {
_42.fld2 = Adt44::Variant0 { fld0: _41,fld1: _37.0.0 };
_37 = Field::<((i8, (u32,)), u16)>(Variant(_26.fld3, 0), 3);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(8_usize, 4_usize, Move(_4), 1_usize, Move(_1), 3_usize, Move(_3), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(8_usize, 25_usize, Move(_25), 23_usize, Move(_23), 41_usize, Move(_41), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(8_usize, 33_usize, Move(_33), 35_usize, Move(_35), 28_usize, Move(_28), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(8_usize, 27_usize, Move(_27), 14_usize, Move(_14), 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: u32,mut _3: *mut f64,mut _4: f64,mut _5: f64) -> u16 {
mir! {
type RET = u16;
let _6: i64;
let _7: Adt42;
let _8: f64;
let _9: Adt48;
let _10: Adt57;
let _11: Adt45;
let _12: [i128; 5];
let _13: i16;
let _14: Adt48;
let _15: u64;
let _16: isize;
let _17: [bool; 8];
let _18: char;
let _19: Adt56;
let _20: [bool; 8];
let _21: [u64; 8];
let _22: f32;
let _23: Adt54;
let _24: *mut u32;
let _25: u128;
let _26: [i128; 5];
let _27: Adt51;
let _28: char;
let _29: Adt44;
let _30: bool;
let _31: char;
let _32: i16;
let _33: char;
let _34: char;
let _35: f64;
let _36: ();
let _37: ();
{
(*_3) = 30880357624328201184608746928200052223_u128 as f64;
_1 = false | false;
RET = 21807_u16 >> _2;
_2 = 3127749362_u32 & 1026546609_u32;
_5 = 882569871_i32 as f64;
_3 = core::ptr::addr_of_mut!((*_3));
_5 = (*_3);
_2 = (-9223372036854775808_isize) as u32;
_4 = (-8627_i16) as f64;
_3 = core::ptr::addr_of_mut!((*_3));
RET = !7020_u16;
_5 = -(*_3);
_6 = 3256911768232425625_i64 << _2;
_2 = 2273485474_u32;
(*_3) = _4 - _5;
_5 = -(*_3);
RET = 60197_u16 - 6131_u16;
Call(_7 = fn10(_3, _5, _6, (*_3), RET, (*_3), _5, _1, RET, (*_3), _6, (*_3), (*_3)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = !false;
(*_3) = (-9223372036854775808_isize) as f64;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.0 = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).1 as i8;
_5 = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).0.0 as f64;
place!(Field::<usize>(Variant(_7, 1), 2)) = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).0.1.0 as usize;
_4 = _5 - (*_3);
RET = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).1;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1 = (_2,);
_4 = -_5;
(*_3) = _4;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).1 = RET;
(*_3) = _5 - _5;
place!(Field::<i64>(Variant(_7, 1), 0)) = _6;
SetDiscriminant(_7, 0);
place!(Field::<*mut (u32,)>(Variant(_7, 0), 0)) = core::ptr::addr_of_mut!(place!(Field::<(u32,)>(Variant(_7, 0), 1)));
_9 = Adt48::Variant0 { fld0: _4,fld1: 250_u8 };
(*_3) = _6 as f64;
place!(Field::<(u32,)>(Variant(_7, 0), 1)).0 = !_2;
RET = !21877_u16;
_6 = -9221734752681900417_i64;
(*_3) = _4 + _5;
place!(Field::<(u32,)>(Variant(_7, 0), 1)).0 = 154222451472083816664609853226167969887_i128 as u32;
_5 = -_4;
SetDiscriminant(_7, 2);
_8 = -_5;
place!(Field::<[i128; 5]>(Variant(_7, 2), 1)) = [71474308478843054446214829518612741746_i128,144498358450527712538543261421394082759_i128,(-18731318289331734529164272687116085645_i128),1957391951387281468046822667294449256_i128,98503381817271457519562327764582502558_i128];
RET = '\u{43c7a}' as u16;
Goto(bb2)
}
bb2 = {
_5 = (*_3);
_1 = false;
RET = 33731_u16;
_2 = 3464185061_u32 - 3967936733_u32;
place!(Field::<[i128; 5]>(Variant(_7, 2), 1)) = [(-23735507089696812033790901885816946443_i128),74973481459795414286999654003692077161_i128,(-84453040411188271505659678647893430853_i128),(-40010726083490682220477271175593942336_i128),4924102052200987399509941515706398478_i128];
place!(Field::<u8>(Variant(_9, 0), 1)) = 232_u8;
place!(Field::<[i128; 5]>(Variant(_7, 2), 1)) = [30837289894757363865474242343382965054_i128,(-26165627931423568934389375196454899095_i128),(-153151509073479852660901167833387252671_i128),(-9737561708597299006174597230827690474_i128),(-57975912152796113645374062761576038671_i128)];
_8 = -(*_3);
_8 = Field::<f64>(Variant(_9, 0), 0);
(*_3) = _8;
place!(Field::<f32>(Variant(_7, 2), 0)) = Field::<u8>(Variant(_9, 0), 1) as f32;
_8 = Field::<f32>(Variant(_7, 2), 0) as f64;
place!(Field::<f32>(Variant(_7, 2), 0)) = RET as f32;
_12 = [(-67161849175675697631706428260666801275_i128),135464158079038702962822343650421385703_i128,97454680624575698342388597091645923650_i128,(-37606720344987093915682566180314447927_i128),112936367002250223215649492353160052039_i128];
place!(Field::<f32>(Variant(_7, 2), 0)) = 108233075989365148482327539751443652244_u128 as f32;
Goto(bb3)
}
bb3 = {
_12 = [44940002793160032777336569580063333127_i128,116386646001167033004302667438762546210_i128,156200358178853761753162204152315281452_i128,118279044983652501969606149534449678506_i128,(-135366924028848167407161755223786118457_i128)];
place!(Field::<u8>(Variant(_9, 0), 1)) = !255_u8;
_13 = !(-26266_i16);
_15 = 9325009808227199473_u64;
_2 = 1547091114_u32 | 1138438590_u32;
place!(Field::<[i128; 5]>(Variant(_7, 2), 1)) = [(-116817290343762331575109523984298075186_i128),58439876802949319949257681344543291689_i128,62708855144369063684372290353752810756_i128,(-144850510604012991341511577985989390173_i128),169517032826419035821048158874766795498_i128];
_12 = [88466297415611669481453041522230913803_i128,38181303781347283177549062051079989922_i128,(-56514404846349452094577178217846781791_i128),(-162494337274169981292907324728127754878_i128),96399924548690483980183066650099936993_i128];
_15 = _5 as u64;
_16 = -9223372036854775807_isize;
_2 = 6802725256527769484162359129762273152_u128 as u32;
_8 = (-993137599_i32) as f64;
RET = !44781_u16;
SetDiscriminant(_7, 1);
_14 = Move(_9);
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1.0 = _2 * _2;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1 = (_2,);
Goto(bb4)
}
bb4 = {
place!(Field::<u8>(Variant(_14, 0), 1)) = 84_u8 - 94_u8;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).1 = !RET;
place!(Field::<[u16; 1]>(Variant(_7, 1), 1)) = [Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).1];
place!(Field::<f64>(Variant(_14, 0), 0)) = (*_3) - _4;
_3 = core::ptr::addr_of_mut!(_5);
_2 = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).0.1.0;
_16 = 7089537907969131776_usize as isize;
RET = Field::<u8>(Variant(_14, 0), 1) as u16;
_4 = (-9_i8) as f64;
(*_3) = 40_i8 as f64;
Call(_16 = fn16(Move(_14), _15, _1, _3, _8, _15, RET), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).0.1.0;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1 = (_2,);
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1 = (_2,);
_8 = -(*_3);
place!(Field::<i64>(Variant(_7, 1), 0)) = _6 & _6;
_8 = (*_3) - (*_3);
place!(Field::<usize>(Variant(_7, 1), 2)) = !12372655112829855866_usize;
_17 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = '\u{d6336}' as u16;
(*_3) = _8 + _8;
(*_3) = -_8;
_12 = [118762288892987823787720658296476421716_i128,152465366749701410325855441579542743776_i128,154161197982232007759592895162980711157_i128,90643937797141349498854305580334219268_i128,(-49141827236355866267419038690728878672_i128)];
_12 = [(-75370848226590066610225497670564385192_i128),(-60578910585721111098198322573158645506_i128),108413309948823337207125266088385109531_i128,(-123401785478160096898128849040026354587_i128),(-149255605932278677630091676180869754944_i128)];
_5 = -_8;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).1 = RET | RET;
_17 = [_1,_1,_1,_1,_1,_1,_1,_1];
_22 = _13 as f32;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.0 = (-104_i8);
_1 = _8 == (*_3);
_10 = Adt57::Variant0 { fld0: 278876299535828998152428349361519387866_u128,fld1: RET };
Call(_23.fld1 = fn18(_1, _1, _8, _1, _3, Move(_7), _3, _3, (*_3), _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = Field::<u16>(Variant(_10, 0), 1);
_20 = [_1,_1,_1,_1,_1,_1,_1,_1];
Call(_2 = core::intrinsics::transmute(_23.fld1.1.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23.fld3.fld1 = '\u{e400e}';
_9 = Adt48::Variant0 { fld0: _5,fld1: 138_u8 };
_23.fld1.1.0 = _2 ^ _2;
_23.fld0 = [_13];
_21 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 4_usize as u16;
_23.fld3 = Adt51 { fld0: _21,fld1: '\u{b97a7}' };
_17 = [_1,_1,_1,_1,_1,_1,_1,_1];
_18 = _23.fld3.fld1;
Goto(bb8)
}
bb8 = {
(*_3) = Field::<f64>(Variant(_9, 0), 0);
_12 = [(-34907829853797734242411711122046633493_i128),110656719993824405103067158998865277958_i128,20627490750877331381699412666393335517_i128,2354069111770705637341673542878164647_i128,42770758129125181555262787517383209861_i128];
place!(Field::<u128>(Variant(_10, 0), 0)) = !102618538352718782330472201554891932684_u128;
place!(Field::<u8>(Variant(_9, 0), 1)) = !90_u8;
_26 = [(-142763714110164781770190941343977750458_i128),105110699793474728304229146735739333691_i128,(-123656198713726125109202232192121735379_i128),(-108436017363004690570787850177915462653_i128),69659831424613064193202577693990144468_i128];
_6 = 2262850011908874422_i64;
_23.fld1.1.0 = !_2;
RET = Field::<u8>(Variant(_9, 0), 1) as u16;
_24 = core::ptr::addr_of_mut!(_23.fld1.1.0);
_22 = _13 as f32;
_14 = Adt48::Variant0 { fld0: _5,fld1: Field::<u8>(Variant(_9, 0), 1) };
place!(Field::<u16>(Variant(_10, 0), 1)) = RET | RET;
place!(Field::<u8>(Variant(_9, 0), 1)) = !Field::<u8>(Variant(_14, 0), 1);
_12 = _26;
_26 = _12;
_27.fld1 = _23.fld3.fld1;
(*_24) = _2 & _2;
_12 = _26;
_23.fld1.1 = (_2,);
match _23.fld1.0 {
0 => bb7,
1 => bb2,
2 => bb6,
3 => bb5,
4 => bb9,
5 => bb10,
8 => bb12,
_ => bb11
}
}
bb9 = {
_23.fld3.fld1 = '\u{e400e}';
_9 = Adt48::Variant0 { fld0: _5,fld1: 138_u8 };
_23.fld1.1.0 = _2 ^ _2;
_23.fld0 = [_13];
_21 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 4_usize as u16;
_23.fld3 = Adt51 { fld0: _21,fld1: '\u{b97a7}' };
_17 = [_1,_1,_1,_1,_1,_1,_1,_1];
_18 = _23.fld3.fld1;
Goto(bb8)
}
bb10 = {
_12 = [44940002793160032777336569580063333127_i128,116386646001167033004302667438762546210_i128,156200358178853761753162204152315281452_i128,118279044983652501969606149534449678506_i128,(-135366924028848167407161755223786118457_i128)];
place!(Field::<u8>(Variant(_9, 0), 1)) = !255_u8;
_13 = !(-26266_i16);
_15 = 9325009808227199473_u64;
_2 = 1547091114_u32 | 1138438590_u32;
place!(Field::<[i128; 5]>(Variant(_7, 2), 1)) = [(-116817290343762331575109523984298075186_i128),58439876802949319949257681344543291689_i128,62708855144369063684372290353752810756_i128,(-144850510604012991341511577985989390173_i128),169517032826419035821048158874766795498_i128];
_12 = [88466297415611669481453041522230913803_i128,38181303781347283177549062051079989922_i128,(-56514404846349452094577178217846781791_i128),(-162494337274169981292907324728127754878_i128),96399924548690483980183066650099936993_i128];
_15 = _5 as u64;
_16 = -9223372036854775807_isize;
_2 = 6802725256527769484162359129762273152_u128 as u32;
_8 = (-993137599_i32) as f64;
RET = !44781_u16;
SetDiscriminant(_7, 1);
_14 = Move(_9);
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1.0 = _2 * _2;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1 = (_2,);
Goto(bb4)
}
bb11 = {
_1 = !false;
(*_3) = (-9223372036854775808_isize) as f64;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.0 = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).1 as i8;
_5 = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).0.0 as f64;
place!(Field::<usize>(Variant(_7, 1), 2)) = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).0.1.0 as usize;
_4 = _5 - (*_3);
RET = Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3).1;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).0.1 = (_2,);
_4 = -_5;
(*_3) = _4;
place!(Field::<((i8, (u32,)), u16)>(Variant(_7, 1), 3)).1 = RET;
(*_3) = _5 - _5;
place!(Field::<i64>(Variant(_7, 1), 0)) = _6;
SetDiscriminant(_7, 0);
place!(Field::<*mut (u32,)>(Variant(_7, 0), 0)) = core::ptr::addr_of_mut!(place!(Field::<(u32,)>(Variant(_7, 0), 1)));
_9 = Adt48::Variant0 { fld0: _4,fld1: 250_u8 };
(*_3) = _6 as f64;
place!(Field::<(u32,)>(Variant(_7, 0), 1)).0 = !_2;
RET = !21877_u16;
_6 = -9221734752681900417_i64;
(*_3) = _4 + _5;
place!(Field::<(u32,)>(Variant(_7, 0), 1)).0 = 154222451472083816664609853226167969887_i128 as u32;
_5 = -_4;
SetDiscriminant(_7, 2);
_8 = -_5;
place!(Field::<[i128; 5]>(Variant(_7, 2), 1)) = [71474308478843054446214829518612741746_i128,144498358450527712538543261421394082759_i128,(-18731318289331734529164272687116085645_i128),1957391951387281468046822667294449256_i128,98503381817271457519562327764582502558_i128];
RET = '\u{43c7a}' as u16;
Goto(bb2)
}
bb12 = {
_5 = -Field::<f64>(Variant(_9, 0), 0);
(*_3) = (-77007819401101824226590510248267400891_i128) as f64;
_15 = 17020825302674359661_u64;
(*_3) = _8 * Field::<f64>(Variant(_9, 0), 0);
SetDiscriminant(_10, 2);
_26 = [(-109216023885772618116352541817515671146_i128),57046658394404999364923366977227389343_i128,(-139804672932140997900530723606783160097_i128),(-113610260659760523055617271971438671297_i128),(-49247650758829907440901936880400773868_i128)];
Goto(bb13)
}
bb13 = {
(*_24) = _27.fld1 as u32;
(*_24) = _16 as u32;
_23.fld0 = [_13];
_26 = _12;
_21 = [_15,_15,_15,_15,_15,_15,_15,_15];
_6 = 146778234759303239366726431858892397568_u128 as i64;
_9 = Move(_14);
place!(Field::<bool>(Variant(_10, 2), 0)) = !_1;
(*_3) = -_8;
_25 = 42486784493785691910453955554882786899_u128;
_23.fld0 = [_13];
_23.fld3.fld1 = _27.fld1;
_23.fld3.fld0 = [_15,_15,_15,_15,_15,_15,_15,_15];
place!(Field::<Adt51>(Variant(_10, 2), 1)) = _23.fld3;
SetDiscriminant(_9, 0);
_25 = !235199702599921303570232372285922202649_u128;
place!(Field::<[u64; 8]>(Variant(_10, 2), 4)) = [_15,_15,_15,_15,_15,_15,_15,_15];
_4 = -(*_3);
place!(Field::<Adt51>(Variant(_10, 2), 1)) = Adt51 { fld0: _23.fld3.fld0,fld1: _23.fld3.fld1 };
_23.fld2 = -_16;
Call(RET = core::intrinsics::bswap(26189_u16), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_29 = Adt44::Variant0 { fld0: _23.fld2,fld1: _23.fld1.0 };
_23.fld2 = _16;
_6 = _25 as i64;
_35 = (-1558144534_i32) as f64;
_23.fld1.1 = (_2,);
(*_3) = -_8;
_2 = (*_24) * (*_24);
place!(Field::<bool>(Variant(_10, 2), 0)) = _8 > _8;
_28 = _18;
RET = !13813_u16;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(9_usize, 17_usize, Move(_17), 20_usize, Move(_20), 6_usize, Move(_6), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(9_usize, 15_usize, Move(_15), 1_usize, Move(_1), 12_usize, Move(_12), 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *mut f64,mut _2: f64,mut _3: i64,mut _4: f64,mut _5: u16,mut _6: f64,mut _7: f64,mut _8: bool,mut _9: u16,mut _10: f64,mut _11: i64,mut _12: f64,mut _13: f64) -> Adt42 {
mir! {
type RET = Adt42;
let _14: isize;
let _15: isize;
let _16: usize;
let _17: Adt53;
let _18: (i8,);
let _19: u32;
let _20: i128;
let _21: u8;
let _22: isize;
let _23: [usize; 4];
let _24: [i16; 1];
let _25: isize;
let _26: [bool; 1];
let _27: Adt41;
let _28: [i16; 1];
let _29: bool;
let _30: (i32, [i128; 5], u64, [bool; 1]);
let _31: *mut u32;
let _32: f64;
let _33: [i16; 1];
let _34: [u16; 1];
let _35: f64;
let _36: [i8; 7];
let _37: bool;
let _38: u128;
let _39: f64;
let _40: (u16,);
let _41: u32;
let _42: isize;
let _43: ((i8, (u32,)), u16);
let _44: u8;
let _45: ((i8, (u32,)), u16);
let _46: (i8, (u32,));
let _47: f32;
let _48: [i16; 1];
let _49: [bool; 8];
let _50: f64;
let _51: char;
let _52: char;
let _53: isize;
let _54: f32;
let _55: f64;
let _56: u16;
let _57: ((i8, (u32,)), u16);
let _58: (u16,);
let _59: f32;
let _60: Adt52;
let _61: ();
let _62: ();
{
_4 = _3 as f64;
_14 = !9223372036854775807_isize;
_9 = _14 as u16;
_2 = -(*_1);
_15 = 348709854_u32 as isize;
_15 = _14 & _14;
_11 = _3;
_9 = _5;
Goto(bb1)
}
bb1 = {
_13 = _7 * _10;
_16 = 2_usize;
_17.fld0 = [(-32359_i16)];
_13 = _2;
_19 = !3364724770_u32;
_4 = _13 + _7;
_18.0 = !52_i8;
_18 = (66_i8,);
_12 = _7;
_21 = !22_u8;
Goto(bb2)
}
bb2 = {
_15 = _14 ^ _14;
_11 = _3 * _3;
_19 = !1678960002_u32;
_11 = !_3;
Call(_11 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = !true;
_4 = _10;
_9 = !_5;
Goto(bb4)
}
bb4 = {
_11 = -_3;
(*_1) = _10 + _13;
_24 = [(-28909_i16)];
_15 = _5 as isize;
_8 = true | true;
_13 = 119718132584463443706660134678032476897_i128 as f64;
_24 = [(-26940_i16)];
_23 = [_16,_16,_16,_16];
_14 = 11500723955865858896_u64 as isize;
_18 = (81_i8,);
match _23[_16] {
0 => bb3,
1 => bb5,
3 => bb7,
2 => bb9,
_ => bb8
}
}
bb5 = {
_8 = !true;
_4 = _10;
_9 = !_5;
Goto(bb4)
}
bb6 = {
_15 = _14 ^ _14;
_11 = _3 * _3;
_19 = !1678960002_u32;
_11 = !_3;
Call(_11 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_13 = _7 * _10;
_16 = 2_usize;
_17.fld0 = [(-32359_i16)];
_13 = _2;
_19 = !3364724770_u32;
_4 = _13 + _7;
_18.0 = !52_i8;
_18 = (66_i8,);
_12 = _7;
_21 = !22_u8;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_10 = _21 as f64;
_15 = _14;
_7 = (*_1) + _12;
_7 = (*_1) * (*_1);
_20 = !(-144167227443604716979031219825892480476_i128);
_10 = _15 as f64;
_20 = (-40670269227989678333135023714845811341_i128);
_17 = Adt53 { fld0: _24 };
Goto(bb10)
}
bb10 = {
_8 = true;
_4 = _18.0 as f64;
_3 = _11;
Call(_11 = fn11(_19, _4), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20 = _18.0 as i128;
_16 = 16323123133742600958_usize | 0_usize;
_8 = !false;
_17 = Adt53 { fld0: _24 };
match _18.0 {
0 => bb4,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
81 => bb19,
_ => bb18
}
}
bb12 = {
_8 = true;
_4 = _18.0 as f64;
_3 = _11;
Call(_11 = fn11(_19, _4), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_10 = _21 as f64;
_15 = _14;
_7 = (*_1) + _12;
_7 = (*_1) * (*_1);
_20 = !(-144167227443604716979031219825892480476_i128);
_10 = _15 as f64;
_20 = (-40670269227989678333135023714845811341_i128);
_17 = Adt53 { fld0: _24 };
Goto(bb10)
}
bb14 = {
Return()
}
bb15 = {
_13 = _7 * _10;
_16 = 2_usize;
_17.fld0 = [(-32359_i16)];
_13 = _2;
_19 = !3364724770_u32;
_4 = _13 + _7;
_18.0 = !52_i8;
_18 = (66_i8,);
_12 = _7;
_21 = !22_u8;
Goto(bb2)
}
bb16 = {
_15 = _14 ^ _14;
_11 = _3 * _3;
_19 = !1678960002_u32;
_11 = !_3;
Call(_11 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_13 = _7 * _10;
_16 = 2_usize;
_17.fld0 = [(-32359_i16)];
_13 = _2;
_19 = !3364724770_u32;
_4 = _13 + _7;
_18.0 = !52_i8;
_18 = (66_i8,);
_12 = _7;
_21 = !22_u8;
Goto(bb2)
}
bb18 = {
_11 = -_3;
(*_1) = _10 + _13;
_24 = [(-28909_i16)];
_15 = _5 as isize;
_8 = true | true;
_13 = 119718132584463443706660134678032476897_i128 as f64;
_24 = [(-26940_i16)];
_23 = [_16,_16,_16,_16];
_14 = 11500723955865858896_u64 as isize;
_18 = (81_i8,);
match _23[_16] {
0 => bb3,
1 => bb5,
3 => bb7,
2 => bb9,
_ => bb8
}
}
bb19 = {
(*_1) = _6;
_25 = _14 << _19;
Goto(bb20)
}
bb20 = {
_21 = 124_u8;
_2 = _20 as f64;
_28 = [(-844_i16)];
_29 = _8;
Goto(bb21)
}
bb21 = {
_21 = 47_u8;
_26 = [_29];
_9 = _5;
_16 = 3802098727173221055_usize;
_30.2 = 17732866095318483124_u64;
_32 = _19 as f64;
_10 = _7 + _7;
_4 = _32 + _32;
_30.2 = 11083450904695562211_u64;
_12 = _16 as f64;
_11 = _3 + _3;
_17 = Adt53 { fld0: _28 };
Goto(bb22)
}
bb22 = {
_21 = 185_u8;
_31 = core::ptr::addr_of_mut!(_19);
_9 = !_5;
_7 = (-14349_i16) as f64;
_25 = _15 * _14;
_18 = (5_i8,);
_1 = core::ptr::addr_of_mut!((*_1));
(*_31) = 1599733776_u32;
(*_31) = 4025061025_u32 | 3796053419_u32;
_12 = _6 * _32;
_16 = 16197586418848335136_usize & 10900811167726009339_usize;
_33 = [(-9311_i16)];
_13 = _10 * _12;
_35 = _13 + _10;
_33 = [(-7735_i16)];
_29 = _8;
_30.0 = -(-1348068026_i32);
(*_31) = !4079791690_u32;
_37 = _29;
_1 = core::ptr::addr_of_mut!(_35);
_24 = [3971_i16];
_9 = _5 - _5;
_19 = (-1237_i16) as u32;
_8 = _29;
(*_31) = !3928460697_u32;
_10 = -_13;
Call(_15 = core::intrinsics::transmute(_11), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_30.1 = [_20,_20,_20,_20,_20];
_8 = _37 & _29;
_23 = [_16,_16,_16,_16];
_30.3 = [_8];
_24 = _17.fld0;
_16 = 7703603022236874359_usize;
_36 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_35 = _10 + _10;
_33 = _24;
_23 = [_16,_16,_16,_16];
_34 = [_9];
_35 = _10 - _13;
_30.0 = (-1297700184_i32);
_30.3 = [_8];
_24 = [(-32521_i16)];
_13 = (*_1);
match _16 {
0 => bb10,
1 => bb24,
2 => bb25,
3 => bb26,
4 => bb27,
7703603022236874359 => bb29,
_ => bb28
}
}
bb24 = {
_15 = _14 ^ _14;
_11 = _3 * _3;
_19 = !1678960002_u32;
_11 = !_3;
Call(_11 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb25 = {
_15 = _14 ^ _14;
_11 = _3 * _3;
_19 = !1678960002_u32;
_11 = !_3;
Call(_11 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb26 = {
_21 = 124_u8;
_2 = _20 as f64;
_28 = [(-844_i16)];
_29 = _8;
Goto(bb21)
}
bb27 = {
_15 = _14 ^ _14;
_11 = _3 * _3;
_19 = !1678960002_u32;
_11 = !_3;
Call(_11 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb28 = {
_11 = -_3;
(*_1) = _10 + _13;
_24 = [(-28909_i16)];
_15 = _5 as isize;
_8 = true | true;
_13 = 119718132584463443706660134678032476897_i128 as f64;
_24 = [(-26940_i16)];
_23 = [_16,_16,_16,_16];
_14 = 11500723955865858896_u64 as isize;
_18 = (81_i8,);
match _23[_16] {
0 => bb3,
1 => bb5,
3 => bb7,
2 => bb9,
_ => bb8
}
}
bb29 = {
_41 = !(*_31);
(*_1) = _10 * _13;
_15 = !_25;
_19 = _41 << _18.0;
_40.0 = _9 >> _16;
_39 = 7378_i16 as f64;
_23 = [_16,_16,_16,_16];
_13 = (*_1) + (*_1);
_9 = _8 as u16;
Goto(bb30)
}
bb30 = {
_15 = _25 & _25;
_25 = _15;
Call((*_31) = core::intrinsics::transmute(_30.0), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_18 = ((-73_i8),);
_21 = !80_u8;
_20 = _30.0 as i128;
_26 = _30.3;
_18.0 = (-57_i8) & 34_i8;
_30.3 = [_29];
_9 = !_40.0;
_33 = _17.fld0;
_22 = _9 as isize;
_10 = _16 as f64;
_42 = _25;
_43.1 = _40.0 - _5;
_37 = _8;
_15 = -_22;
_23 = [_16,_16,_16,_16];
_12 = -_13;
_10 = _13;
_30.0 = 511892327_i32 - 612966512_i32;
_39 = _12 + (*_1);
_30.3 = [_29];
_45.0.0 = _18.0;
_5 = !_43.1;
_14 = !_15;
_39 = _11 as f64;
Goto(bb32)
}
bb32 = {
_43.0.1 = (_41,);
_30.0 = (-1569506691_i32);
(*_1) = _13;
_19 = !_43.0.1.0;
_35 = -_12;
_43.0.1 = (_19,);
_10 = (*_1) * _13;
(*_31) = !_41;
_2 = _10;
_40.0 = _5 >> _5;
_25 = _14 << _42;
_10 = 31757974050926422960935895382634527652_u128 as f64;
Goto(bb33)
}
bb33 = {
_45.1 = !_40.0;
_46.0 = !_18.0;
_27 = Adt41::Variant0 { fld0: _5,fld1: _30 };
_40.0 = _45.1 ^ _45.1;
_20 = 116520308338565974671387829022404216413_i128;
_42 = _22 | _25;
_46.0 = _18.0;
_46.1 = _43.0.1;
_49 = [_29,_37,_37,_8,_37,_8,_37,_37];
_12 = (*_1);
_45.0.1 = (_19,);
_31 = core::ptr::addr_of_mut!(_43.0.1.0);
match Field::<(i32, [i128; 5], u64, [bool; 1])>(Variant(_27, 0), 1).2 {
0 => bb17,
1 => bb20,
2 => bb34,
3 => bb35,
4 => bb36,
5 => bb37,
11083450904695562211 => bb39,
_ => bb38
}
}
bb34 = {
_21 = 185_u8;
_31 = core::ptr::addr_of_mut!(_19);
_9 = !_5;
_7 = (-14349_i16) as f64;
_25 = _15 * _14;
_18 = (5_i8,);
_1 = core::ptr::addr_of_mut!((*_1));
(*_31) = 1599733776_u32;
(*_31) = 4025061025_u32 | 3796053419_u32;
_12 = _6 * _32;
_16 = 16197586418848335136_usize & 10900811167726009339_usize;
_33 = [(-9311_i16)];
_13 = _10 * _12;
_35 = _13 + _10;
_33 = [(-7735_i16)];
_29 = _8;
_30.0 = -(-1348068026_i32);
(*_31) = !4079791690_u32;
_37 = _29;
_1 = core::ptr::addr_of_mut!(_35);
_24 = [3971_i16];
_9 = _5 - _5;
_19 = (-1237_i16) as u32;
_8 = _29;
(*_31) = !3928460697_u32;
_10 = -_13;
Call(_15 = core::intrinsics::transmute(_11), ReturnTo(bb23), UnwindUnreachable())
}
bb35 = {
_13 = _7 * _10;
_16 = 2_usize;
_17.fld0 = [(-32359_i16)];
_13 = _2;
_19 = !3364724770_u32;
_4 = _13 + _7;
_18.0 = !52_i8;
_18 = (66_i8,);
_12 = _7;
_21 = !22_u8;
Goto(bb2)
}
bb36 = {
_13 = _7 * _10;
_16 = 2_usize;
_17.fld0 = [(-32359_i16)];
_13 = _2;
_19 = !3364724770_u32;
_4 = _13 + _7;
_18.0 = !52_i8;
_18 = (66_i8,);
_12 = _7;
_21 = !22_u8;
Goto(bb2)
}
bb37 = {
_41 = !(*_31);
(*_1) = _10 * _13;
_15 = !_25;
_19 = _41 << _18.0;
_40.0 = _9 >> _16;
_39 = 7378_i16 as f64;
_23 = [_16,_16,_16,_16];
_13 = (*_1) + (*_1);
_9 = _8 as u16;
Goto(bb30)
}
bb38 = {
Return()
}
bb39 = {
_3 = _11 - _11;
_48 = [21421_i16];
_47 = _40.0 as f32;
_40 = (_9,);
_2 = (*_1) * _12;
_40.0 = !_45.1;
_1 = core::ptr::addr_of_mut!(_12);
_43.1 = _40.0 ^ _40.0;
match Field::<(i32, [i128; 5], u64, [bool; 1])>(Variant(_27, 0), 1).2 {
11083450904695562211 => bb40,
_ => bb20
}
}
bb40 = {
_38 = !64680259031453468485727857405967332880_u128;
match _20 {
116520308338565974671387829022404216413 => bb42,
_ => bb41
}
}
bb41 = {
_10 = _21 as f64;
_15 = _14;
_7 = (*_1) + _12;
_7 = (*_1) * (*_1);
_20 = !(-144167227443604716979031219825892480476_i128);
_10 = _15 as f64;
_20 = (-40670269227989678333135023714845811341_i128);
_17 = Adt53 { fld0: _24 };
Goto(bb10)
}
bb42 = {
_46.1.0 = _47 as u32;
place!(Field::<(i32, [i128; 5], u64, [bool; 1])>(Variant(_27, 0), 1)).3 = _26;
_43.0 = (_45.0.0, _46.1);
Goto(bb43)
}
bb43 = {
_45.1 = !_40.0;
_4 = (*_1) + _32;
_43.0.0 = !_45.0.0;
_44 = _21;
_56 = Field::<u16>(Variant(_27, 0), 0);
_22 = _43.1 as isize;
_39 = Field::<(i32, [i128; 5], u64, [bool; 1])>(Variant(_27, 0), 1).2 as f64;
_43.0.0 = _8 as i8;
_52 = '\u{65872}';
_54 = _47 * _47;
_45.0 = (_46.0, _46.1);
_53 = _38 as isize;
_19 = (*_31) & (*_31);
_46.0 = _8 as i8;
_9 = _56 & _43.1;
_54 = _3 as f32;
_47 = _54;
_55 = -_12;
_9 = _11 as u16;
_46.1.0 = _43.0.1.0 + _19;
place!(Field::<(i32, [i128; 5], u64, [bool; 1])>(Variant(_27, 0), 1)).2 = _30.2 + _30.2;
_22 = _42;
RET = Adt42::Variant1 { fld0: _3,fld1: _34,fld2: _16,fld3: _45 };
_43.0.1.0 = !_46.1.0;
Goto(bb44)
}
bb44 = {
Call(_61 = dump_var(10_usize, 3_usize, Move(_3), 41_usize, Move(_41), 48_usize, Move(_48), 20_usize, Move(_20)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_61 = dump_var(10_usize, 24_usize, Move(_24), 29_usize, Move(_29), 28_usize, Move(_28), 8_usize, Move(_8)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_61 = dump_var(10_usize, 18_usize, Move(_18), 49_usize, Move(_49), 23_usize, Move(_23), 53_usize, Move(_53)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_61 = dump_var(10_usize, 46_usize, Move(_46), 42_usize, Move(_42), 25_usize, Move(_25), 37_usize, Move(_37)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_61 = dump_var(10_usize, 34_usize, Move(_34), 5_usize, Move(_5), 62_usize, _62, 62_usize, _62), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: u32,mut _2: f64) -> i64 {
mir! {
type RET = i64;
let _3: [u64; 8];
let _4: (i8,);
let _5: u64;
let _6: char;
let _7: Adt46;
let _8: f32;
let _9: [usize; 4];
let _10: f64;
let _11: [bool; 1];
let _12: i32;
let _13: [u64; 8];
let _14: (u32,);
let _15: isize;
let _16: Adt55;
let _17: [i8; 7];
let _18: u64;
let _19: [i16; 1];
let _20: isize;
let _21: [i8; 7];
let _22: Adt45;
let _23: [bool; 8];
let _24: char;
let _25: [bool; 8];
let _26: bool;
let _27: [i16; 1];
let _28: char;
let _29: Adt41;
let _30: ();
let _31: ();
{
_2 = 14795231730738514604_u64 as f64;
_3 = [18357247276736176982_u64,8560320185289125309_u64,12763313051072592083_u64,1733323886122937195_u64,12901065326434627714_u64,3421828393382761263_u64,13339905238870157509_u64,1080002179006742937_u64];
_3 = [4053125611954046931_u64,11482166787766795576_u64,8548145217798744535_u64,14022893078881756150_u64,17364827710125233697_u64,13639756849986307712_u64,18284626168554296298_u64,8350706292965528642_u64];
RET = -(-6082396039343828606_i64);
_4.0 = 124_i8 * 119_i8;
_5 = !10586253407226567964_u64;
_5 = (-714744661_i32) as u64;
_4 = ((-104_i8),);
_1 = 345104196_u32;
_1 = 2713095765_u32 * 1051755332_u32;
_5 = !112818809705783754_u64;
RET = _1 as i64;
_2 = (-1666106371_i32) as f64;
Call(RET = fn12(_3, _2, _4, _3, _3, _4.0, _3, _3, _3, _3, _4.0, _4.0, _3, _3, _5, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = '\u{1fe55}';
Goto(bb2)
}
bb2 = {
_4 = (98_i8,);
RET = 1367542403_i32 as i64;
_4.0 = !50_i8;
_4 = ((-67_i8),);
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = _5 as f64;
_6 = '\u{5f887}';
_8 = _4.0 as f32;
_9 = [6_usize,18326799049167708142_usize,9304778605490561945_usize,15745307309435203127_usize];
RET = -(-56819247260982535_i64);
RET = (-5240413790395907027_i64);
_2 = 9223372036854775807_isize as f64;
_9 = [2_usize,13329347543006620670_usize,16522232677238309783_usize,1457692127571585547_usize];
Goto(bb3)
}
bb3 = {
_2 = 41678741319358120722947834198723034019_i128 as f64;
_10 = _2;
_1 = !3810997168_u32;
_12 = 1793303348_i32 - 1020515_i32;
RET = !(-1625161296160920838_i64);
_9 = [1685021214204135163_usize,3_usize,6_usize,14085645508016177469_usize];
_4 = ((-35_i8),);
_5 = 8597093718326548723_u64;
_6 = '\u{21c6}';
Goto(bb4)
}
bb4 = {
_10 = _2 - _2;
_13 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = _10;
_12 = -764596997_i32;
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
_1 = !433501851_u32;
_14 = (_1,);
_5 = 11234025563378928498_u64;
_16.fld6 = Adt51 { fld0: _3,fld1: _6 };
_6 = _16.fld6.fld1;
_16.fld2 = core::ptr::addr_of!(_2);
_16.fld7.0 = _4.0;
_16.fld6.fld1 = _6;
_2 = -_10;
_11 = [true];
_16.fld6.fld1 = _6;
_14 = (_1,);
_16.fld1 = _6;
_16.fld4 = 59648_u16 as f64;
match _16.fld7.0 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431768211421 => bb9,
_ => bb8
}
}
bb5 = {
_2 = 41678741319358120722947834198723034019_i128 as f64;
_10 = _2;
_1 = !3810997168_u32;
_12 = 1793303348_i32 - 1020515_i32;
RET = !(-1625161296160920838_i64);
_9 = [1685021214204135163_usize,3_usize,6_usize,14085645508016177469_usize];
_4 = ((-35_i8),);
_5 = 8597093718326548723_u64;
_6 = '\u{21c6}';
Goto(bb4)
}
bb6 = {
_4 = (98_i8,);
RET = 1367542403_i32 as i64;
_4.0 = !50_i8;
_4 = ((-67_i8),);
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = _5 as f64;
_6 = '\u{5f887}';
_8 = _4.0 as f32;
_9 = [6_usize,18326799049167708142_usize,9304778605490561945_usize,15745307309435203127_usize];
RET = -(-56819247260982535_i64);
RET = (-5240413790395907027_i64);
_2 = 9223372036854775807_isize as f64;
_9 = [2_usize,13329347543006620670_usize,16522232677238309783_usize,1457692127571585547_usize];
Goto(bb3)
}
bb7 = {
_6 = '\u{1fe55}';
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_16.fld5 = 325309788793950915408981719962411154752_u128;
_19 = [(-5732_i16)];
_15 = -9223372036854775807_isize;
_16.fld6 = Adt51 { fld0: _3,fld1: _6 };
_13 = _16.fld6.fld0;
_6 = _16.fld6.fld1;
_17 = [_4.0,_16.fld7.0,_4.0,_4.0,_4.0,_16.fld7.0,_4.0];
_20 = _15;
_21 = [_4.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_4.0,_16.fld7.0,_16.fld7.0];
_17 = [_16.fld7.0,_16.fld7.0,_4.0,_4.0,_4.0,_16.fld7.0,_16.fld7.0];
RET = (-3082062520295283630_i64);
_12 = 2023933825_i32;
_6 = _16.fld1;
_16.fld6.fld1 = _16.fld1;
_17 = [_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0];
_12 = 5_usize as i32;
_25 = [true,false,false,true,false,false,true,false];
_25 = [false,true,false,true,false,true,true,true];
Goto(bb10)
}
bb10 = {
_21 = [_4.0,_4.0,_4.0,_4.0,_4.0,_4.0,_4.0];
_24 = _6;
_18 = !_5;
_14 = (_1,);
_23 = _25;
_18 = _5;
_6 = _16.fld6.fld1;
match _5 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
11234025563378928498 => bb17,
_ => bb16
}
}
bb11 = {
_6 = '\u{1fe55}';
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_6 = '\u{1fe55}';
Goto(bb2)
}
bb14 = {
_4 = (98_i8,);
RET = 1367542403_i32 as i64;
_4.0 = !50_i8;
_4 = ((-67_i8),);
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = _5 as f64;
_6 = '\u{5f887}';
_8 = _4.0 as f32;
_9 = [6_usize,18326799049167708142_usize,9304778605490561945_usize,15745307309435203127_usize];
RET = -(-56819247260982535_i64);
RET = (-5240413790395907027_i64);
_2 = 9223372036854775807_isize as f64;
_9 = [2_usize,13329347543006620670_usize,16522232677238309783_usize,1457692127571585547_usize];
Goto(bb3)
}
bb15 = {
_2 = 41678741319358120722947834198723034019_i128 as f64;
_10 = _2;
_1 = !3810997168_u32;
_12 = 1793303348_i32 - 1020515_i32;
RET = !(-1625161296160920838_i64);
_9 = [1685021214204135163_usize,3_usize,6_usize,14085645508016177469_usize];
_4 = ((-35_i8),);
_5 = 8597093718326548723_u64;
_6 = '\u{21c6}';
Goto(bb4)
}
bb16 = {
_4 = (98_i8,);
RET = 1367542403_i32 as i64;
_4.0 = !50_i8;
_4 = ((-67_i8),);
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = _5 as f64;
_6 = '\u{5f887}';
_8 = _4.0 as f32;
_9 = [6_usize,18326799049167708142_usize,9304778605490561945_usize,15745307309435203127_usize];
RET = -(-56819247260982535_i64);
RET = (-5240413790395907027_i64);
_2 = 9223372036854775807_isize as f64;
_9 = [2_usize,13329347543006620670_usize,16522232677238309783_usize,1457692127571585547_usize];
Goto(bb3)
}
bb17 = {
_16.fld6.fld1 = _6;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(11_usize, 6_usize, Move(_6), 11_usize, Move(_11), 21_usize, Move(_21), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(11_usize, 13_usize, Move(_13), 4_usize, Move(_4), 17_usize, Move(_17), 14_usize, Move(_14)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(11_usize, 18_usize, Move(_18), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [u64; 8],mut _2: f64,mut _3: (i8,),mut _4: [u64; 8],mut _5: [u64; 8],mut _6: i8,mut _7: [u64; 8],mut _8: [u64; 8],mut _9: [u64; 8],mut _10: [u64; 8],mut _11: i8,mut _12: i8,mut _13: [u64; 8],mut _14: [u64; 8],mut _15: u64,mut _16: [u64; 8]) -> i64 {
mir! {
type RET = i64;
let _17: Adt49;
let _18: [bool; 1];
let _19: [usize; 4];
let _20: Adt48;
let _21: f64;
let _22: char;
let _23: u128;
let _24: [usize; 4];
let _25: [u16; 1];
let _26: i32;
let _27: [i8; 7];
let _28: f64;
let _29: f64;
let _30: [i16; 1];
let _31: u128;
let _32: Adt51;
let _33: bool;
let _34: ();
let _35: ();
{
_11 = _15 as i8;
_16 = [_15,_15,_15,_15,_15,_15,_15,_15];
_19 = [12940606593226021821_usize,15309937832717099574_usize,5255989147103663095_usize,3_usize];
_17.fld1 = [_12,_3.0,_12,_3.0,_11,_3.0,_12];
_19 = [3552962438933052653_usize,15754157443432545209_usize,2856619025766645521_usize,3_usize];
Goto(bb1)
}
bb1 = {
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _11 };
_6 = 4037_u16 as i8;
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_2 = 21510_u16 as f64;
_18 = [false];
_3 = (_12,);
_7 = _8;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = _14;
_12 = _11 * _6;
_15 = 7399559308994724751_u64;
_7 = _4;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17.fld2 = Adt44::Variant0 { fld0: (-35_isize),fld1: _3.0 };
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _11 };
_7 = _4;
_23 = 273378336359901957345138383037726181444_u128;
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = 65464_u16 as i8;
Goto(bb2)
}
bb2 = {
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 2086613414295922542_i64;
_17.fld1 = [_12,Field::<i8>(Variant(_17.fld2, 0), 1),_12,_3.0,_11,_12,_6];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = [_15,_15,_15,_15,_15,_15,_15,_15];
_12 = false as i8;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = (-3802236909128996528_i64) ^ (-4277769433690213979_i64);
_17.fld1 = [Field::<i8>(Variant(_17.fld2, 0), 1),_6,_12,Field::<i8>(Variant(_17.fld2, 0), 1),_11,_6,Field::<i8>(Variant(_17.fld2, 0), 1)];
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = !_11;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_21 = _2;
_24 = [0_usize,7_usize,206828709316901150_usize,6_usize];
_17.fld1 = [_12,_3.0,_3.0,_3.0,_3.0,_12,_11];
_2 = -_21;
_22 = '\u{b192e}';
RET = 16537_i16 as i64;
place!(Field::<isize>(Variant(_17.fld2, 0), 0)) = -(-9223372036854775808_isize);
_18 = [true];
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_18 = [true];
_1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_15 = 856578317977981996_u64 - 3810433109764649514_u64;
_17.fld4 = _23 as f64;
_10 = _13;
_17.fld2 = Adt44::Variant0 { fld0: (-9223372036854775808_isize),fld1: _11 };
Call(_20 = fn13(Field::<i8>(Variant(_17.fld2, 0), 1), _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = _10;
_18 = [true];
_23 = (-52248296389157490271965547000407205324_i128) as u128;
_3 = (_6,);
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _12 };
_23 = 206801058150928146909317192016407475278_u128;
_17.fld1 = [Field::<i8>(Variant(_17.fld2, 0), 1),Field::<i8>(Variant(_17.fld2, 0), 1),_6,_12,_6,_11,_11];
_19 = [1_usize,3_usize,4_usize,1_usize];
RET = 52271_u16 as i64;
_4 = _14;
_8 = _13;
_25 = [44783_u16];
_22 = '\u{74a93}';
_21 = -_17.fld4;
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = 3224696556_u32 as i8;
_11 = 2692_u16 as i8;
_22 = '\u{50581}';
_14 = _4;
_26 = (-1190105688_i32) | 1759136488_i32;
_2 = _17.fld4;
_2 = _17.fld4 - Field::<f64>(Variant(_20, 0), 0);
_1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_2 = _21;
_17.fld1 = [Field::<i8>(Variant(_17.fld2, 0), 1),Field::<i8>(Variant(_17.fld2, 0), 1),_11,Field::<i8>(Variant(_17.fld2, 0), 1),_6,_6,_12];
match Field::<u8>(Variant(_20, 0), 1) {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb10,
_ => bb9
}
}
bb4 = {
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 2086613414295922542_i64;
_17.fld1 = [_12,Field::<i8>(Variant(_17.fld2, 0), 1),_12,_3.0,_11,_12,_6];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = [_15,_15,_15,_15,_15,_15,_15,_15];
_12 = false as i8;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = (-3802236909128996528_i64) ^ (-4277769433690213979_i64);
_17.fld1 = [Field::<i8>(Variant(_17.fld2, 0), 1),_6,_12,Field::<i8>(Variant(_17.fld2, 0), 1),_11,_6,Field::<i8>(Variant(_17.fld2, 0), 1)];
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = !_11;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_21 = _2;
_24 = [0_usize,7_usize,206828709316901150_usize,6_usize];
_17.fld1 = [_12,_3.0,_3.0,_3.0,_3.0,_12,_11];
_2 = -_21;
_22 = '\u{b192e}';
RET = 16537_i16 as i64;
place!(Field::<isize>(Variant(_17.fld2, 0), 0)) = -(-9223372036854775808_isize);
_18 = [true];
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_18 = [true];
_1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_15 = 856578317977981996_u64 - 3810433109764649514_u64;
_17.fld4 = _23 as f64;
_10 = _13;
_17.fld2 = Adt44::Variant0 { fld0: (-9223372036854775808_isize),fld1: _11 };
Call(_20 = fn13(Field::<i8>(Variant(_17.fld2, 0), 1), _19), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _11 };
_6 = 4037_u16 as i8;
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_2 = 21510_u16 as f64;
_18 = [false];
_3 = (_12,);
_7 = _8;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = _14;
_12 = _11 * _6;
_15 = 7399559308994724751_u64;
_7 = _4;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17.fld2 = Adt44::Variant0 { fld0: (-35_isize),fld1: _3.0 };
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _11 };
_7 = _4;
_23 = 273378336359901957345138383037726181444_u128;
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = 65464_u16 as i8;
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
_24 = _19;
place!(Field::<isize>(Variant(_17.fld2, 0), 0)) = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3.0 = !_11;
_25 = [62888_u16];
place!(Field::<isize>(Variant(_17.fld2, 0), 0)) = -(-9223372036854775808_isize);
_21 = _15 as f64;
_3.0 = !Field::<i8>(Variant(_17.fld2, 0), 1);
_23 = 53536357098397221464820200258423166711_u128;
_3 = (Field::<i8>(Variant(_17.fld2, 0), 1),);
_5 = _8;
match _23 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
53536357098397221464820200258423166711 => bb17,
_ => bb16
}
}
bb11 = {
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 2086613414295922542_i64;
_17.fld1 = [_12,Field::<i8>(Variant(_17.fld2, 0), 1),_12,_3.0,_11,_12,_6];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = [_15,_15,_15,_15,_15,_15,_15,_15];
_12 = false as i8;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = (-3802236909128996528_i64) ^ (-4277769433690213979_i64);
_17.fld1 = [Field::<i8>(Variant(_17.fld2, 0), 1),_6,_12,Field::<i8>(Variant(_17.fld2, 0), 1),_11,_6,Field::<i8>(Variant(_17.fld2, 0), 1)];
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = !_11;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_21 = _2;
_24 = [0_usize,7_usize,206828709316901150_usize,6_usize];
_17.fld1 = [_12,_3.0,_3.0,_3.0,_3.0,_12,_11];
_2 = -_21;
_22 = '\u{b192e}';
RET = 16537_i16 as i64;
place!(Field::<isize>(Variant(_17.fld2, 0), 0)) = -(-9223372036854775808_isize);
_18 = [true];
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_18 = [true];
_1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_15 = 856578317977981996_u64 - 3810433109764649514_u64;
_17.fld4 = _23 as f64;
_10 = _13;
_17.fld2 = Adt44::Variant0 { fld0: (-9223372036854775808_isize),fld1: _11 };
Call(_20 = fn13(Field::<i8>(Variant(_17.fld2, 0), 1), _19), ReturnTo(bb3), UnwindUnreachable())
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
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _11 };
_6 = 4037_u16 as i8;
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_2 = 21510_u16 as f64;
_18 = [false];
_3 = (_12,);
_7 = _8;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = _14;
_12 = _11 * _6;
_15 = 7399559308994724751_u64;
_7 = _4;
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17.fld2 = Adt44::Variant0 { fld0: (-35_isize),fld1: _3.0 };
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17.fld2 = Adt44::Variant0 { fld0: 9223372036854775807_isize,fld1: _11 };
_7 = _4;
_23 = 273378336359901957345138383037726181444_u128;
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = 65464_u16 as i8;
Goto(bb2)
}
bb16 = {
_10 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 2086613414295922542_i64;
_17.fld1 = [_12,Field::<i8>(Variant(_17.fld2, 0), 1),_12,_3.0,_11,_12,_6];
_14 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = [_15,_15,_15,_15,_15,_15,_15,_15];
_12 = false as i8;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_8 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = (-3802236909128996528_i64) ^ (-4277769433690213979_i64);
_17.fld1 = [Field::<i8>(Variant(_17.fld2, 0), 1),_6,_12,Field::<i8>(Variant(_17.fld2, 0), 1),_11,_6,Field::<i8>(Variant(_17.fld2, 0), 1)];
place!(Field::<i8>(Variant(_17.fld2, 0), 1)) = !_11;
_4 = [_15,_15,_15,_15,_15,_15,_15,_15];
_21 = _2;
_24 = [0_usize,7_usize,206828709316901150_usize,6_usize];
_17.fld1 = [_12,_3.0,_3.0,_3.0,_3.0,_12,_11];
_2 = -_21;
_22 = '\u{b192e}';
RET = 16537_i16 as i64;
place!(Field::<isize>(Variant(_17.fld2, 0), 0)) = -(-9223372036854775808_isize);
_18 = [true];
_5 = [_15,_15,_15,_15,_15,_15,_15,_15];
_18 = [true];
_1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_15 = 856578317977981996_u64 - 3810433109764649514_u64;
_17.fld4 = _23 as f64;
_10 = _13;
_17.fld2 = Adt44::Variant0 { fld0: (-9223372036854775808_isize),fld1: _11 };
Call(_20 = fn13(Field::<i8>(Variant(_17.fld2, 0), 1), _19), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_9 = _14;
_2 = _21;
_1 = _13;
place!(Field::<f64>(Variant(_20, 0), 0)) = -_21;
RET = 8048154648160940448_i64;
_19 = _24;
_28 = _21 + _21;
_5 = _4;
_23 = (-132093368730174511127129193665594751838_i128) as u128;
_17.fld4 = -_28;
place!(Field::<f64>(Variant(_20, 0), 0)) = _21;
_29 = _17.fld4 + Field::<f64>(Variant(_20, 0), 0);
_5 = _9;
_32.fld0 = [_15,_15,_15,_15,_15,_15,_15,_15];
_18 = [true];
_24 = [5_usize,13735966738286366167_usize,7660503188991113583_usize,4_usize];
_23 = true as u128;
_24 = [17177011932161551979_usize,0_usize,2_usize,12734385993061043695_usize];
_9 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = 3670716394259255362_i64;
_17.fld1 = [_6,_6,Field::<i8>(Variant(_17.fld2, 0), 1),_3.0,_11,_11,_6];
RET = !6461779589587578205_i64;
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(12_usize, 24_usize, Move(_24), 10_usize, Move(_10), 3_usize, Move(_3), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(12_usize, 23_usize, Move(_23), 26_usize, Move(_26), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(12_usize, 1_usize, Move(_1), 12_usize, Move(_12), 25_usize, Move(_25), 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i8,mut _2: [usize; 4]) -> Adt48 {
mir! {
type RET = Adt48;
let _3: Adt50;
let _4: isize;
let _5: (bool, bool, *mut u32);
let _6: u8;
let _7: (i8,);
let _8: Adt55;
let _9: Adt44;
let _10: (u32,);
let _11: f64;
let _12: bool;
let _13: usize;
let _14: char;
let _15: Adt42;
let _16: isize;
let _17: i128;
let _18: [i8; 7];
let _19: (((i8, (u32,)), u16), [bool; 8], ((i8, (u32,)), i128, &'static i64), (u16,), ((i8, (u32,)), i128, &'static i64), [i8; 7]);
let _20: usize;
let _21: *const f64;
let _22: *const f64;
let _23: [bool; 8];
let _24: Adt44;
let _25: (u32,);
let _26: (i8,);
let _27: bool;
let _28: i16;
let _29: Adt55;
let _30: isize;
let _31: Adt55;
let _32: bool;
let _33: [u64; 8];
let _34: [i8; 7];
let _35: i32;
let _36: bool;
let _37: bool;
let _38: ();
let _39: ();
{
_1 = 12_u8 as i8;
_2 = [7_usize,3_usize,2_usize,16868237513505676185_usize];
_3.fld2.0 = 4211131613_u32;
_2 = [9241627143937337206_usize,2_usize,18296936884478363933_usize,15791328631554270775_usize];
_2 = [3_usize,8590815536418223659_usize,1_usize,1866006392885278062_usize];
_5.1 = !false;
Goto(bb1)
}
bb1 = {
_3.fld1 = _1 as i128;
_2 = [11511001366458561920_usize,5_usize,17503340389379289761_usize,2_usize];
_5.2 = core::ptr::addr_of_mut!(_3.fld2.0);
_4 = !(-9223372036854775808_isize);
_1 = 178_u8 as i8;
_5.2 = core::ptr::addr_of_mut!(_3.fld2.0);
_7.0 = _1;
_3.fld1 = _5.1 as i128;
_5.0 = _5.1;
_3.fld2.0 = (-12059_i16) as u32;
_1 = _7.0;
_1 = !_7.0;
_2 = [7_usize,0_usize,3_usize,6_usize];
_5.1 = !_5.0;
_3.fld2 = (2703860143_u32,);
_5.1 = _5.0;
_3.fld2.0 = 202968919_u32 & 796604891_u32;
_3.fld2 = (1573018399_u32,);
_3.fld2 = (2496953629_u32,);
_6 = 41_u8 << _1;
_3.fld1 = (-58232055142345791647297641800161661155_i128);
Goto(bb2)
}
bb2 = {
_6 = 190_u8 | 190_u8;
_8.fld6.fld1 = '\u{d16eb}';
_8.fld7.0 = -_1;
Goto(bb3)
}
bb3 = {
_8.fld7.0 = _6 as i8;
_10.0 = _3.fld1 as u32;
_1 = !_8.fld7.0;
_8.fld7.0 = (-6912782006409477487_i64) as i8;
_5.0 = _5.1;
_3.fld0 = core::ptr::addr_of!(_8.fld4);
_8.fld4 = 20515_i16 as f64;
_2 = [16548392645640431416_usize,4_usize,4_usize,8215635756462740981_usize];
RET = Adt48::Variant0 { fld0: _8.fld4,fld1: _6 };
_5.1 = Field::<u8>(Variant(RET, 0), 1) == _6;
_8.fld2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(RET, 0), 0)));
place!(Field::<f64>(Variant(RET, 0), 0)) = _8.fld4 * _8.fld4;
_3 = Adt50 { fld0: _8.fld2,fld1: (-150045994877496672372599058300888975345_i128),fld2: _10 };
Call(_6 = fn14(_3.fld1, Move(RET), _3.fld0, _3, _10.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = (_8.fld7.0,);
_3.fld2.0 = 1999690346_i32 as u32;
_8.fld7 = _7;
_14 = _8.fld6.fld1;
_12 = _5.1;
Goto(bb5)
}
bb5 = {
_7 = _8.fld7;
_8.fld1 = _8.fld6.fld1;
_10 = _3.fld2;
_16 = (-25023_i16) as isize;
_11 = _4 as f64;
_8.fld6.fld0 = [4087219536293907756_u64,13150441447909558538_u64,17420724313568080039_u64,7210624455978157844_u64,16707028185103954680_u64,14456600198369800646_u64,14250939464296866195_u64,7667206941907949539_u64];
_5.2 = core::ptr::addr_of_mut!(_10.0);
_5.2 = core::ptr::addr_of_mut!(_10.0);
_8.fld6.fld0 = [16363129228230582642_u64,5411796506183017758_u64,9645296997447971678_u64,6061511682831836616_u64,17117248780098896860_u64,10920094139085447378_u64,16482183132631981817_u64,16293191925125061179_u64];
_8.fld7.0 = !_7.0;
_8.fld6.fld1 = _14;
_8.fld4 = -_11;
_12 = _5.1 < _5.1;
_13 = 3_usize;
_5.1 = _3.fld2.0 > _10.0;
_8.fld6.fld0 = [3505162963545759819_u64,3068900743698096864_u64,5606795093782976507_u64,16674489350728473901_u64,14175477260398705106_u64,8734537986135499997_u64,8288369273172621507_u64,5642710780667262623_u64];
_8.fld7.0 = _13 as i8;
_7 = _8.fld7;
_3.fld2.0 = _10.0;
_17 = -_3.fld1;
match _2[_13] {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
8215635756462740981 => bb10,
_ => bb9
}
}
bb6 = {
_7 = (_8.fld7.0,);
_3.fld2.0 = 1999690346_i32 as u32;
_8.fld7 = _7;
_14 = _8.fld6.fld1;
_12 = _5.1;
Goto(bb5)
}
bb7 = {
_8.fld7.0 = _6 as i8;
_10.0 = _3.fld1 as u32;
_1 = !_8.fld7.0;
_8.fld7.0 = (-6912782006409477487_i64) as i8;
_5.0 = _5.1;
_3.fld0 = core::ptr::addr_of!(_8.fld4);
_8.fld4 = 20515_i16 as f64;
_2 = [16548392645640431416_usize,4_usize,4_usize,8215635756462740981_usize];
RET = Adt48::Variant0 { fld0: _8.fld4,fld1: _6 };
_5.1 = Field::<u8>(Variant(RET, 0), 1) == _6;
_8.fld2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(RET, 0), 0)));
place!(Field::<f64>(Variant(RET, 0), 0)) = _8.fld4 * _8.fld4;
_3 = Adt50 { fld0: _8.fld2,fld1: (-150045994877496672372599058300888975345_i128),fld2: _10 };
Call(_6 = fn14(_3.fld1, Move(RET), _3.fld0, _3, _10.0), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_6 = 190_u8 | 190_u8;
_8.fld6.fld1 = '\u{d16eb}';
_8.fld7.0 = -_1;
Goto(bb3)
}
bb9 = {
_3.fld1 = _1 as i128;
_2 = [11511001366458561920_usize,5_usize,17503340389379289761_usize,2_usize];
_5.2 = core::ptr::addr_of_mut!(_3.fld2.0);
_4 = !(-9223372036854775808_isize);
_1 = 178_u8 as i8;
_5.2 = core::ptr::addr_of_mut!(_3.fld2.0);
_7.0 = _1;
_3.fld1 = _5.1 as i128;
_5.0 = _5.1;
_3.fld2.0 = (-12059_i16) as u32;
_1 = _7.0;
_1 = !_7.0;
_2 = [7_usize,0_usize,3_usize,6_usize];
_5.1 = !_5.0;
_3.fld2 = (2703860143_u32,);
_5.1 = _5.0;
_3.fld2.0 = 202968919_u32 & 796604891_u32;
_3.fld2 = (1573018399_u32,);
_3.fld2 = (2496953629_u32,);
_6 = 41_u8 << _1;
_3.fld1 = (-58232055142345791647297641800161661155_i128);
Goto(bb2)
}
bb10 = {
_18[_13] = _7.0;
match _8.fld6.fld0[_13] {
16674489350728473901 => bb12,
_ => bb11
}
}
bb11 = {
_8.fld7.0 = _6 as i8;
_10.0 = _3.fld1 as u32;
_1 = !_8.fld7.0;
_8.fld7.0 = (-6912782006409477487_i64) as i8;
_5.0 = _5.1;
_3.fld0 = core::ptr::addr_of!(_8.fld4);
_8.fld4 = 20515_i16 as f64;
_2 = [16548392645640431416_usize,4_usize,4_usize,8215635756462740981_usize];
RET = Adt48::Variant0 { fld0: _8.fld4,fld1: _6 };
_5.1 = Field::<u8>(Variant(RET, 0), 1) == _6;
_8.fld2 = core::ptr::addr_of!(place!(Field::<f64>(Variant(RET, 0), 0)));
place!(Field::<f64>(Variant(RET, 0), 0)) = _8.fld4 * _8.fld4;
_3 = Adt50 { fld0: _8.fld2,fld1: (-150045994877496672372599058300888975345_i128),fld2: _10 };
Call(_6 = fn14(_3.fld1, Move(RET), _3.fld0, _3, _10.0), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_19.4.0.1 = (_10.0,);
_5.0 = !_5.1;
_19.0.0.0 = _18[_13];
_5.0 = _2[_13] != _2[_13];
_8.fld6.fld0[_13] = !13413594390931901023_u64;
_20 = _2[_13];
_19.4.0.0 = _8.fld7.0 >> _20;
_19.3 = (62501_u16,);
_16 = _4 << _19.4.0.0;
_19.0.0 = (_7.0, _19.4.0.1);
_7.0 = _8.fld7.0;
_3.fld2 = _19.4.0.1;
_19.5[_13] = _8.fld4 as i8;
_19.0.0.0 = _19.4.0.0;
_19.1[_13] = _12 < _12;
_1 = _8.fld6.fld0[_13] as i8;
_5.2 = core::ptr::addr_of_mut!(_10.0);
_7.0 = -_19.0.0.0;
_19.2.1 = _3.fld1;
_19.4.1 = _19.2.1 & _17;
_8.fld6.fld0[_13] = !14469042122149584411_u64;
_2[_13] = !_13;
_19.4.0 = (_7.0, _3.fld2);
_5.0 = !_19.1[_13];
Goto(bb13)
}
bb13 = {
_5.2 = core::ptr::addr_of_mut!(_19.4.0.1.0);
_22 = _8.fld2;
_19.5[_13] = !_7.0;
_7.0 = _6 as i8;
_13 = !_20;
_7 = (_19.4.0.0,);
_19.4.0.1.0 = !_10.0;
_18 = [_19.0.0.0,_7.0,_19.0.0.0,_7.0,_7.0,_19.4.0.0,_7.0];
_3.fld1 = !_19.2.1;
_10 = (_19.4.0.1.0,);
_3.fld2 = (_19.0.0.1.0,);
_19.2.0.0 = _19.0.0.0 * _19.0.0.0;
_23 = [_12,_5.0,_12,_5.0,_5.0,_5.0,_5.0,_5.0];
_11 = _8.fld4;
_3.fld0 = _22;
(*_22) = 6665620368301735060_i64 as f64;
Goto(bb14)
}
bb14 = {
(*_22) = _11 + _11;
_11 = _8.fld4 * (*_22);
_3.fld2 = (_19.4.0.1.0,);
_16 = _4;
_19.0.0.0 = _19.3.0 as i8;
_27 = !_12;
_7 = _8.fld7;
_7 = _8.fld7;
_29.fld2 = core::ptr::addr_of!(_11);
_16 = _4 >> _19.4.1;
_26 = (_8.fld7.0,);
_29.fld6.fld1 = _8.fld6.fld1;
_26.0 = _19.2.0.0 + _19.2.0.0;
_5.0 = _27 | _12;
Goto(bb15)
}
bb15 = {
_19.4.0.1.0 = _29.fld6.fld1 as u32;
_3.fld0 = _8.fld2;
_29.fld5 = !101819852221104001650769910745344975162_u128;
(*_22) = -_11;
Goto(bb16)
}
bb16 = {
_7.0 = _26.0;
_19.4.0.1.0 = _3.fld2.0;
_3.fld1 = _19.4.1;
_25 = (_3.fld2.0,);
Goto(bb17)
}
bb17 = {
_8.fld6.fld0 = [1874240639748075105_u64,5281302005514728410_u64,12168394793452045698_u64,3890457665063773977_u64,8559099879302301592_u64,4790000299723298365_u64,13774897478815362682_u64,8699835286105651002_u64];
_29.fld1 = _8.fld6.fld1;
_21 = _29.fld2;
_26 = (_7.0,);
_31.fld6.fld1 = _14;
_19.4.1 = _3.fld1;
_19.4.0 = (_26.0, _10);
_24 = Adt44::Variant0 { fld0: _16,fld1: _7.0 };
_30 = _19.4.0.1.0 as isize;
_28 = -(-19304_i16);
_35 = (*_22) as i32;
_19.5 = _18;
_31.fld5 = _29.fld5;
_19.3.0 = 10058822118885019717_u64 as u16;
_19.0.0 = (_19.4.0.0, _10);
SetDiscriminant(_24, 1);
_29.fld7.0 = !_19.0.0.0;
place!(Field::<*mut u32>(Variant(_24, 1), 5)) = core::ptr::addr_of_mut!(_3.fld2.0);
_19.2.1 = _16 as i128;
_5.1 = !_12;
_19.2.0 = (_26.0, _19.4.0.1);
RET = Adt48::Variant0 { fld0: _8.fld4,fld1: _6 };
place!(Field::<f64>(Variant(_24, 1), 2)) = (*_21) - _11;
place!(Field::<((i8, (u32,)), u16)>(Variant(_24, 1), 0)).0.1.0 = Field::<u8>(Variant(RET, 0), 1) as u32;
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(13_usize, 7_usize, Move(_7), 16_usize, Move(_16), 30_usize, Move(_30), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(13_usize, 18_usize, Move(_18), 10_usize, Move(_10), 17_usize, Move(_17), 35_usize, Move(_35)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(13_usize, 26_usize, Move(_26), 14_usize, Move(_14), 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i128,mut _2: Adt48,mut _3: *const f64,mut _4: Adt50,mut _5: u32) -> u8 {
mir! {
type RET = u8;
let _6: Adt50;
let _7: isize;
let _8: isize;
let _9: Adt54;
let _10: Adt57;
let _11: bool;
let _12: [bool; 1];
let _13: i16;
let _14: f64;
let _15: i128;
let _16: bool;
let _17: (i8,);
let _18: Adt51;
let _19: ((i8, (u32,)), u16);
let _20: i64;
let _21: bool;
let _22: isize;
let _23: [i8; 7];
let _24: char;
let _25: isize;
let _26: u32;
let _27: f64;
let _28: f64;
let _29: (u16,);
let _30: bool;
let _31: char;
let _32: Adt51;
let _33: [u16; 1];
let _34: [i128; 5];
let _35: Adt55;
let _36: [bool; 1];
let _37: isize;
let _38: (i8,);
let _39: Adt52;
let _40: i16;
let _41: [bool; 1];
let _42: i16;
let _43: u128;
let _44: (i8, (u32,));
let _45: &'static i64;
let _46: isize;
let _47: isize;
let _48: [bool; 1];
let _49: ();
let _50: ();
{
_4.fld1 = -_1;
_6.fld1 = _1;
place!(Field::<u8>(Variant(_2, 0), 1)) = 22_u8 << _5;
_4.fld1 = _1;
_6.fld0 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_2, 0), 0)));
_6 = _4;
place!(Field::<f64>(Variant(_2, 0), 0)) = 23001_i16 as f64;
_6 = Adt50 { fld0: _4.fld0,fld1: _4.fld1,fld2: _4.fld2 };
RET = '\u{c0dc}' as u8;
RET = Field::<u8>(Variant(_2, 0), 1);
_6 = _4;
Goto(bb1)
}
bb1 = {
RET = Field::<u8>(Variant(_2, 0), 1);
_8 = 67_isize ^ (-9223372036854775808_isize);
_7 = _8;
_9.fld3.fld0 = [12350070592159586380_u64,2474733268797174185_u64,4048289608531589685_u64,6055499984242889134_u64,16050195231344378585_u64,9758609685526761459_u64,13128256395489136149_u64,7950956209614773774_u64];
_4.fld2.0 = !_5;
_9.fld1.1.0 = _4.fld2.0 + _6.fld2.0;
SetDiscriminant(_2, 2);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0.1 = (_6.fld2.0,);
_6.fld2 = (_4.fld2.0,);
_9.fld1 = ((-58_i8), _6.fld2);
_9.fld1 = ((-118_i8), _6.fld2);
_4.fld2.0 = 316796942022808326566337764706808219493_u128 as u32;
place!(Field::<[u64; 8]>(Variant(_2, 2), 4)) = [8176404301845993755_u64,16201923647320692146_u64,16126881308042056868_u64,15414088066799068374_u64,12315761355232098749_u64,4701688265509432762_u64,16516628667786830092_u64,14552667819567052964_u64];
_9.fld3 = Adt51 { fld0: Field::<[u64; 8]>(Variant(_2, 2), 4),fld1: '\u{ed4b}' };
_4 = Adt50 { fld0: _3,fld1: _6.fld1,fld2: _9.fld1.1 };
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0.1.0 = _6.fld2.0;
_10 = Adt57::Variant0 { fld0: 101892164690321702902846320591204536863_u128,fld1: 17347_u16 };
place!(Field::<*mut u32>(Variant(_2, 2), 0)) = core::ptr::addr_of_mut!(_9.fld1.1.0);
_9.fld1.1 = (_5,);
_9.fld0 = [7540_i16];
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0.1 = (_5,);
_6.fld1 = _4.fld1 * _4.fld1;
match _9.fld1.0 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211338 => bb6,
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
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)) = (_9.fld1, 42959_u16);
_9.fld3.fld0 = [15205868871969477394_u64,724836413306550114_u64,10018916744873002262_u64,9802012005109984455_u64,6640474752017464103_u64,5529898132847846635_u64,15600621470113985368_u64,13831106517484050983_u64];
_9.fld1.1.0 = Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).0.1.0 - _5;
RET = _8 as u8;
_14 = 8964749907449134551_u64 as f64;
_12 = [false];
place!(Field::<u128>(Variant(_10, 0), 0)) = false as u128;
_3 = core::ptr::addr_of!(_14);
_4.fld0 = core::ptr::addr_of!(_14);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0 = _9.fld1;
_13 = 24441_i16 & 19531_i16;
_3 = core::ptr::addr_of!(_14);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0.1 = (_9.fld1.1.0,);
_9.fld3 = Adt51 { fld0: Field::<[u64; 8]>(Variant(_2, 2), 4),fld1: '\u{30507}' };
RET = 89_u8 - 172_u8;
_9.fld3.fld0 = Field::<[u64; 8]>(Variant(_2, 2), 4);
_1 = _6.fld1;
_15 = _6.fld1;
_9.fld3 = Adt51 { fld0: Field::<[u64; 8]>(Variant(_2, 2), 4),fld1: '\u{5c36a}' };
place!(Field::<*mut (u32,)>(Variant(_2, 2), 1)) = core::ptr::addr_of_mut!(_9.fld1.1);
_1 = _6.fld1;
Goto(bb7)
}
bb7 = {
RET = 249_u8 >> _9.fld1.1.0;
_9.fld1.1.0 = _4.fld2.0 & _6.fld2.0;
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0 = _9.fld1;
_19.1 = Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).1;
_10 = Adt57::Variant0 { fld0: 274228052232375408488600883609268223460_u128,fld1: Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).1 };
_4.fld2.0 = !_9.fld1.1.0;
_11 = !true;
_6.fld1 = _15;
_14 = 473115542199404104_i64 as f64;
_4.fld2 = (_6.fld2.0,);
place!(Field::<[u64; 8]>(Variant(_2, 2), 4)) = _9.fld3.fld0;
_19.0.1.0 = _4.fld2.0;
_6 = _4;
match _19.1 {
0 => bb6,
42959 => bb8,
_ => bb2
}
}
bb8 = {
_13 = (-9830_i16);
_18.fld1 = _9.fld3.fld1;
_19.0.0 = -Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).0.0;
place!(Field::<u128>(Variant(_10, 0), 0)) = 114544276901415462379030812643750987285_u128;
place!(Field::<*mut u32>(Variant(_2, 2), 0)) = core::ptr::addr_of_mut!(_6.fld2.0);
place!(Field::<Adt47>(Variant(_2, 2), 5)) = Adt47::Variant2 { fld0: Field::<u128>(Variant(_10, 0), 0),fld1: _9.fld3.fld1 };
_7 = _8 & _8;
_18.fld0 = Field::<[u64; 8]>(Variant(_2, 2), 4);
place!(Field::<u128>(Variant(place!(Field::<Adt47>(Variant(_2, 2), 5)), 2), 0)) = Field::<u128>(Variant(_10, 0), 0) + Field::<u128>(Variant(_10, 0), 0);
_20 = (-6395292668367995972_i64);
_19.1 = Field::<u16>(Variant(_10, 0), 1);
SetDiscriminant(_10, 0);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).1 = !_19.1;
_21 = _11;
_17 = (_9.fld1.0,);
_16 = !_21;
_9.fld0 = [_13];
SetDiscriminant(Field::<Adt47>(Variant(_2, 2), 5), 0);
_6.fld2 = (Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).0.1.0,);
_17.0 = _9.fld1.0;
_24 = _18.fld1;
place!(Field::<[u64; 8]>(Variant(_2, 2), 4)) = [10579732829477985971_u64,4910152923107387274_u64,6098710882697481537_u64,10073605714300614855_u64,8652728262201052529_u64,10473890759704807906_u64,2900844398689390847_u64,9781319240045563_u64];
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)) = (_19.0, _19.1);
_2 = Adt48::Variant1 { fld0: 1_usize,fld1: _24,fld2: _19.1,fld3: _19 };
_22 = -_7;
_9.fld1.0 = -_19.0.0;
Goto(bb9)
}
bb9 = {
_18 = Adt51 { fld0: _9.fld3.fld0,fld1: _24 };
_15 = _1;
_4 = _6;
_27 = -(*_3);
_19.0.1.0 = 118765983288662769407071686483936372079_u128 as u32;
_15 = !_1;
_9.fld3 = Adt51 { fld0: _18.fld0,fld1: Field::<char>(Variant(_2, 1), 1) };
_8 = _16 as isize;
_17.0 = _9.fld1.0;
_9.fld3.fld0 = [4668868396002534401_u64,2324355620318412625_u64,7059066142575090007_u64,8100163654867340000_u64,14105310344777942819_u64,9055524894953724838_u64,7592214308250745791_u64,5089077922553441790_u64];
_6.fld2.0 = _4.fld2.0 | _4.fld2.0;
_9.fld1.1 = (_6.fld2.0,);
_19.0 = (_9.fld1.0, _9.fld1.1);
_6.fld0 = _4.fld0;
_8 = Field::<char>(Variant(_2, 1), 1) as isize;
_12 = [_11];
_9.fld3.fld0 = [3954382030665011284_u64,6303939171261267129_u64,8680661902132404779_u64,11867764990739496768_u64,18446410912793893657_u64,4244546258272994533_u64,7545081941818193895_u64,1271148318498671189_u64];
_19.1 = Field::<u16>(Variant(_2, 1), 2);
_8 = _22;
Call(_18 = fn15(_4, RET, _6.fld2, _15, _3, _6.fld2.0, Field::<char>(Variant(_2, 1), 1)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3)) = _19;
_9.fld2 = _22;
place!(Field::<u16>(Variant(_10, 0), 1)) = !_19.1;
_16 = _11;
_9.fld1.1 = _4.fld2;
_32.fld0 = _9.fld3.fld0;
_18 = _9.fld3;
_6 = _4;
_17.0 = 2806730808602761318_usize as i8;
_13 = (-25671_i16) ^ (-2169_i16);
place!(Field::<u128>(Variant(_10, 0), 0)) = 261719682794277134748723843331231916737_u128;
_26 = _6.fld2.0 & _6.fld2.0;
_8 = _9.fld2;
_18.fld0 = [12241106500188898599_u64,16927541299778688482_u64,18386280917529755961_u64,4960074750719454976_u64,7382195736884597360_u64,15088608762221329798_u64,8477758327924374993_u64,17837624007847162388_u64];
_16 = _11;
_35.fld6.fld0 = [6462692087106587405_u64,9623989084308606084_u64,3838029762626627966_u64,16542512595293765323_u64,8970687398371809001_u64,9325446070281785380_u64,14919746752098332575_u64,18099159515723999531_u64];
_11 = _21;
_12 = [_11];
_35.fld5 = _20 as u128;
_18.fld1 = _9.fld3.fld1;
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3)).0.1 = (_5,);
Goto(bb11)
}
bb11 = {
SetDiscriminant(_10, 0);
place!(Field::<usize>(Variant(_2, 1), 0)) = 10045495494976795804_usize >> _7;
_5 = _4.fld2.0 | _26;
_16 = _21;
_1 = _19.0.0 as i128;
_4.fld0 = core::ptr::addr_of!(_28);
_31 = _24;
_20 = _11 as i64;
_24 = _9.fld3.fld1;
_30 = _9.fld1.1.0 != _4.fld2.0;
_38.0 = _9.fld1.0 * _17.0;
_37 = !_7;
_8 = -_37;
_40 = !_13;
_28 = -(*_3);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3)).0.1.0 = _6.fld2.0 - _26;
_4.fld0 = _6.fld0;
_35.fld1 = _24;
_35.fld7 = (Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3).0.0,);
_31 = _35.fld1;
_11 = _30 ^ _30;
_8 = _26 as isize;
RET = 123_u8;
_32 = Adt51 { fld0: _9.fld3.fld0,fld1: _31 };
_9.fld1.0 = _38.0;
match _19.1 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb6,
42959 => bb13,
_ => bb12
}
}
bb12 = {
RET = 249_u8 >> _9.fld1.1.0;
_9.fld1.1.0 = _4.fld2.0 & _6.fld2.0;
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0 = _9.fld1;
_19.1 = Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).1;
_10 = Adt57::Variant0 { fld0: 274228052232375408488600883609268223460_u128,fld1: Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).1 };
_4.fld2.0 = !_9.fld1.1.0;
_11 = !true;
_6.fld1 = _15;
_14 = 473115542199404104_i64 as f64;
_4.fld2 = (_6.fld2.0,);
place!(Field::<[u64; 8]>(Variant(_2, 2), 4)) = _9.fld3.fld0;
_19.0.1.0 = _4.fld2.0;
_6 = _4;
match _19.1 {
0 => bb6,
42959 => bb8,
_ => bb2
}
}
bb13 = {
_4 = _6;
(*_3) = _20 as f64;
_38 = (_35.fld7.0,);
_31 = _9.fld3.fld1;
_9.fld3.fld1 = _31;
_9.fld1.1.0 = !_6.fld2.0;
_18 = Adt51 { fld0: _9.fld3.fld0,fld1: _32.fld1 };
_29.0 = Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3).1;
_41 = [_30];
_17 = _38;
_35.fld7 = (_19.0.0,);
_21 = !_30;
place!(Field::<char>(Variant(_2, 1), 1)) = _31;
_35.fld6 = _18;
_35.fld7 = (Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3).0.0,);
_31 = Field::<char>(Variant(_2, 1), 1);
_17 = (_35.fld7.0,);
_32.fld0 = _35.fld6.fld0;
_9.fld1.1.0 = !_19.0.1.0;
match _19.1 {
0 => bb7,
1 => bb8,
2 => bb11,
42959 => bb15,
_ => bb14
}
}
bb14 = {
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)) = (_9.fld1, 42959_u16);
_9.fld3.fld0 = [15205868871969477394_u64,724836413306550114_u64,10018916744873002262_u64,9802012005109984455_u64,6640474752017464103_u64,5529898132847846635_u64,15600621470113985368_u64,13831106517484050983_u64];
_9.fld1.1.0 = Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2).0.1.0 - _5;
RET = _8 as u8;
_14 = 8964749907449134551_u64 as f64;
_12 = [false];
place!(Field::<u128>(Variant(_10, 0), 0)) = false as u128;
_3 = core::ptr::addr_of!(_14);
_4.fld0 = core::ptr::addr_of!(_14);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0 = _9.fld1;
_13 = 24441_i16 & 19531_i16;
_3 = core::ptr::addr_of!(_14);
place!(Field::<((i8, (u32,)), u16)>(Variant(_2, 2), 2)).0.1 = (_9.fld1.1.0,);
_9.fld3 = Adt51 { fld0: Field::<[u64; 8]>(Variant(_2, 2), 4),fld1: '\u{30507}' };
RET = 89_u8 - 172_u8;
_9.fld3.fld0 = Field::<[u64; 8]>(Variant(_2, 2), 4);
_1 = _6.fld1;
_15 = _6.fld1;
_9.fld3 = Adt51 { fld0: Field::<[u64; 8]>(Variant(_2, 2), 4),fld1: '\u{5c36a}' };
place!(Field::<*mut (u32,)>(Variant(_2, 2), 1)) = core::ptr::addr_of_mut!(_9.fld1.1);
_1 = _6.fld1;
Goto(bb7)
}
bb15 = {
_3 = _6.fld0;
_44 = Field::<((i8, (u32,)), u16)>(Variant(_2, 1), 3).0;
_31 = _35.fld1;
RET = 6_u8;
_30 = !_11;
_35.fld5 = 146578575687143087547853913041267901793_u128 * 339557115524214220130438303501548648235_u128;
SetDiscriminant(_2, 0);
_16 = _21 | _21;
_38.0 = _9.fld1.0 - _17.0;
place!(Field::<u8>(Variant(_2, 0), 1)) = _40 as u8;
_32.fld1 = _18.fld1;
_38.0 = _20 as i8;
_18.fld0 = [16200879848841354591_u64,11529286121618567856_u64,9377646693288359190_u64,6128127042541297313_u64,698983993224296879_u64,11098243358682181847_u64,9417475576291949804_u64,5310032897293840422_u64];
_44 = _19.0;
_36 = [_21];
_19.1 = _9.fld2 as u16;
_36 = [_11];
Goto(bb16)
}
bb16 = {
Call(_49 = dump_var(14_usize, 22_usize, Move(_22), 24_usize, Move(_24), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(14_usize, 31_usize, Move(_31), 15_usize, Move(_15), 44_usize, Move(_44), 37_usize, Move(_37)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(14_usize, 17_usize, Move(_17), 19_usize, Move(_19), 30_usize, Move(_30), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: Adt50,mut _2: u8,mut _3: (u32,),mut _4: i128,mut _5: *const f64,mut _6: u32,mut _7: char) -> Adt51 {
mir! {
type RET = Adt51;
let _8: [u16; 1];
let _9: usize;
let _10: Adt48;
let _11: u16;
let _12: &'static i64;
let _13: bool;
let _14: isize;
let _15: bool;
let _16: u128;
let _17: i64;
let _18: i32;
let _19: [i8; 7];
let _20: Adt57;
let _21: (i8,);
let _22: *mut u32;
let _23: Adt41;
let _24: u32;
let _25: u8;
let _26: [bool; 8];
let _27: (u32,);
let _28: ();
let _29: ();
{
RET.fld1 = _7;
RET.fld0 = [18414500389967444237_u64,13415160585544229504_u64,569174302541680513_u64,8787269147339339566_u64,14856157551560919308_u64,636347170277611276_u64,2746506980008319982_u64,18221270294877581091_u64];
_1.fld2.0 = 2_usize as u32;
RET.fld0 = [2311524739623755676_u64,14944201645884748023_u64,13408365173713162277_u64,7799745004044805770_u64,843040085891763856_u64,4950704271046307769_u64,16989653470128926361_u64,3965774277261112162_u64];
RET.fld1 = _7;
_1.fld1 = 1987858826_i32 as i128;
RET.fld1 = _7;
RET.fld1 = _7;
RET.fld1 = _7;
_3.0 = !_6;
_3.0 = 61026567557239711415895414534967021156_u128 as u32;
RET.fld0 = [4168153474556573892_u64,18429629963992250911_u64,7266560750123005332_u64,9736064356368171494_u64,919609302670641200_u64,217139096509595968_u64,10476474507804248848_u64,2889105772484042091_u64];
_2 = (-2633104075089981667_i64) as u8;
_9 = 2_usize;
_4 = 217522684698926600238751880507171901643_u128 as i128;
RET.fld0[_9] = 16257002671178388099_u64 + 18074093546552618171_u64;
_1.fld0 = core::ptr::addr_of!((*_5));
RET.fld0 = [11122773936530302397_u64,16861153970649227957_u64,10418389827870960540_u64,180635677073993465_u64,15572167771979084992_u64,9838095987898450957_u64,4196597786251487878_u64,11653741762333657182_u64];
Goto(bb1)
}
bb1 = {
_6 = _1.fld2.0;
_11 = _6 as u16;
_2 = !94_u8;
_1.fld1 = _4 & _4;
_4 = !_1.fld1;
_6 = _3.0;
_8 = [_11];
_11 = (-9223372036854775808_isize) as u16;
_2 = !253_u8;
_11 = _9 as u16;
_7 = RET.fld1;
_2 = _9 as u8;
RET.fld0[_9] = 14634638514788813064_u64 & 13364824577122451446_u64;
RET.fld0 = [16752552204964945874_u64,863179189370979022_u64,12187218603052556177_u64,15913853978007574712_u64,3784391796140745057_u64,1180802086169073253_u64,14615445439235962667_u64,10855665307997590858_u64];
_3.0 = _6 & _1.fld2.0;
_11 = 46935_u16;
_1.fld2 = (_3.0,);
_6 = _3.0 - _3.0;
_1.fld1 = !_4;
_4 = _9 as i128;
_8 = [_11];
RET.fld0[_9] = 9982150021686811517_u64;
_7 = RET.fld1;
Goto(bb2)
}
bb2 = {
_1.fld0 = core::ptr::addr_of!((*_5));
_13 = _6 != _6;
_8 = [_11];
RET.fld1 = _7;
_7 = RET.fld1;
_4 = _1.fld1;
_1.fld2 = _3;
RET.fld0 = [7707362838505889386_u64,15589599356141555680_u64,1974359533420486513_u64,2565737305302713192_u64,11309377376313446182_u64,1649446639333016639_u64,6445544677059682209_u64,13840938933963827096_u64];
_4 = _1.fld1;
_5 = _1.fld0;
RET.fld0[_9] = 15776360891945858606_u64;
Goto(bb3)
}
bb3 = {
(*_5) = 41_i8 as f64;
RET.fld1 = _7;
_8 = [_11];
_7 = RET.fld1;
RET.fld0[_9] = 9911682420331716941_u64 * 7713847672734250940_u64;
_4 = _1.fld1 | _1.fld1;
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
(*_5) = 140237631873147841739149609571406923069_u128 as f64;
(*_5) = 6_i8 as f64;
RET.fld0 = [9300581787671631510_u64,2451269056999081586_u64,15423339607618205457_u64,15562349797999782338_u64,543230384432965358_u64,12900977565678374378_u64,3736635181427031073_u64,3165008167200428326_u64];
_1.fld0 = _5;
_1.fld2 = (_6,);
_7 = RET.fld1;
RET.fld0 = [2510109613236929424_u64,8853440848485831069_u64,3727495179440405626_u64,16275801719816951025_u64,9539368692916178999_u64,4336499398622381239_u64,16900641444295105563_u64,10543244914831343467_u64];
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
_9 = 14681316379553944014_usize * 3828999485853288859_usize;
_9 = 8924144745199684411_usize;
RET.fld1 = _7;
_15 = !_13;
_5 = _1.fld0;
_14 = (-117_isize);
_7 = RET.fld1;
(*_5) = 1845482882_i32 as f64;
_1.fld0 = core::ptr::addr_of!((*_5));
Goto(bb4)
}
bb4 = {
RET.fld0 = [5339610189977680559_u64,13298294677159081443_u64,1771996339890841474_u64,6034819871948559648_u64,14647914740939482714_u64,12875423694951476250_u64,15850452267322847178_u64,5808337477698282711_u64];
RET.fld0 = [14550718572907440419_u64,9217004920030223371_u64,14945815765579578435_u64,16316301833050660753_u64,1349518939320562576_u64,4331365508228480889_u64,7938130899528289594_u64,6515091450818216086_u64];
_1.fld2.0 = _6;
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
RET.fld0 = [13650426225839416873_u64,252881676103944189_u64,11861756263400638368_u64,2260426972823757394_u64,6033143855798019074_u64,11400433784070746372_u64,17234878075427653485_u64,12229587989540260909_u64];
_1.fld2 = (_6,);
_18 = -(-260123439_i32);
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
_12 = &_17;
_19 = [(-25_i8),(-97_i8),5_i8,99_i8,(-9_i8),8_i8,(-53_i8)];
_8 = [_11];
_6 = _1.fld2.0;
_4 = _1.fld1 | _1.fld1;
_9 = 4119441396222655885_i64 as usize;
_16 = 150983832106016749767946831500730905206_u128 ^ 220068097300172645001054219327405796216_u128;
(*_5) = _2 as f64;
_22 = core::ptr::addr_of_mut!(_3.0);
_7 = RET.fld1;
_1.fld2.0 = !_6;
_21.0 = (-87_i8);
match _14 {
0 => bb5,
1 => bb6,
340282366920938463463374607431768211339 => bb8,
_ => bb7
}
}
bb5 = {
(*_5) = 41_i8 as f64;
RET.fld1 = _7;
_8 = [_11];
_7 = RET.fld1;
RET.fld0[_9] = 9911682420331716941_u64 * 7713847672734250940_u64;
_4 = _1.fld1 | _1.fld1;
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
(*_5) = 140237631873147841739149609571406923069_u128 as f64;
(*_5) = 6_i8 as f64;
RET.fld0 = [9300581787671631510_u64,2451269056999081586_u64,15423339607618205457_u64,15562349797999782338_u64,543230384432965358_u64,12900977565678374378_u64,3736635181427031073_u64,3165008167200428326_u64];
_1.fld0 = _5;
_1.fld2 = (_6,);
_7 = RET.fld1;
RET.fld0 = [2510109613236929424_u64,8853440848485831069_u64,3727495179440405626_u64,16275801719816951025_u64,9539368692916178999_u64,4336499398622381239_u64,16900641444295105563_u64,10543244914831343467_u64];
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
_9 = 14681316379553944014_usize * 3828999485853288859_usize;
_9 = 8924144745199684411_usize;
RET.fld1 = _7;
_15 = !_13;
_5 = _1.fld0;
_14 = (-117_isize);
_7 = RET.fld1;
(*_5) = 1845482882_i32 as f64;
_1.fld0 = core::ptr::addr_of!((*_5));
Goto(bb4)
}
bb6 = {
_1.fld0 = core::ptr::addr_of!((*_5));
_13 = _6 != _6;
_8 = [_11];
RET.fld1 = _7;
_7 = RET.fld1;
_4 = _1.fld1;
_1.fld2 = _3;
RET.fld0 = [7707362838505889386_u64,15589599356141555680_u64,1974359533420486513_u64,2565737305302713192_u64,11309377376313446182_u64,1649446639333016639_u64,6445544677059682209_u64,13840938933963827096_u64];
_4 = _1.fld1;
_5 = _1.fld0;
RET.fld0[_9] = 15776360891945858606_u64;
Goto(bb3)
}
bb7 = {
_6 = _1.fld2.0;
_11 = _6 as u16;
_2 = !94_u8;
_1.fld1 = _4 & _4;
_4 = !_1.fld1;
_6 = _3.0;
_8 = [_11];
_11 = (-9223372036854775808_isize) as u16;
_2 = !253_u8;
_11 = _9 as u16;
_7 = RET.fld1;
_2 = _9 as u8;
RET.fld0[_9] = 14634638514788813064_u64 & 13364824577122451446_u64;
RET.fld0 = [16752552204964945874_u64,863179189370979022_u64,12187218603052556177_u64,15913853978007574712_u64,3784391796140745057_u64,1180802086169073253_u64,14615445439235962667_u64,10855665307997590858_u64];
_3.0 = _6 & _1.fld2.0;
_11 = 46935_u16;
_1.fld2 = (_3.0,);
_6 = _3.0 - _3.0;
_1.fld1 = !_4;
_4 = _9 as i128;
_8 = [_11];
RET.fld0[_9] = 9982150021686811517_u64;
_7 = RET.fld1;
Goto(bb2)
}
bb8 = {
_7 = RET.fld1;
RET.fld0 = [2372563916765929021_u64,1635127485377529359_u64,3909497726771062530_u64,11938425515333681839_u64,3271364082655727820_u64,7685983410606205137_u64,17642693034750025690_u64,744872791270656922_u64];
_5 = _1.fld0;
_1.fld0 = core::ptr::addr_of!((*_5));
_4 = _1.fld1;
_17 = !4634277004080458691_i64;
_24 = _6;
_18 = -(-1707149514_i32);
_4 = _17 as i128;
_2 = 183_u8 ^ 207_u8;
(*_5) = _18 as f64;
_12 = &_17;
_18 = (-832807723_i32);
_18 = 2123239294_i32 - (-1087802547_i32);
_1.fld2.0 = !_24;
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
_1.fld2.0 = _24 * (*_22);
_15 = !_13;
match _21.0 {
0 => bb7,
1 => bb9,
2 => bb10,
3 => bb11,
340282366920938463463374607431768211369 => bb13,
_ => bb12
}
}
bb9 = {
_6 = _1.fld2.0;
_11 = _6 as u16;
_2 = !94_u8;
_1.fld1 = _4 & _4;
_4 = !_1.fld1;
_6 = _3.0;
_8 = [_11];
_11 = (-9223372036854775808_isize) as u16;
_2 = !253_u8;
_11 = _9 as u16;
_7 = RET.fld1;
_2 = _9 as u8;
RET.fld0[_9] = 14634638514788813064_u64 & 13364824577122451446_u64;
RET.fld0 = [16752552204964945874_u64,863179189370979022_u64,12187218603052556177_u64,15913853978007574712_u64,3784391796140745057_u64,1180802086169073253_u64,14615445439235962667_u64,10855665307997590858_u64];
_3.0 = _6 & _1.fld2.0;
_11 = 46935_u16;
_1.fld2 = (_3.0,);
_6 = _3.0 - _3.0;
_1.fld1 = !_4;
_4 = _9 as i128;
_8 = [_11];
RET.fld0[_9] = 9982150021686811517_u64;
_7 = RET.fld1;
Goto(bb2)
}
bb10 = {
_6 = _1.fld2.0;
_11 = _6 as u16;
_2 = !94_u8;
_1.fld1 = _4 & _4;
_4 = !_1.fld1;
_6 = _3.0;
_8 = [_11];
_11 = (-9223372036854775808_isize) as u16;
_2 = !253_u8;
_11 = _9 as u16;
_7 = RET.fld1;
_2 = _9 as u8;
RET.fld0[_9] = 14634638514788813064_u64 & 13364824577122451446_u64;
RET.fld0 = [16752552204964945874_u64,863179189370979022_u64,12187218603052556177_u64,15913853978007574712_u64,3784391796140745057_u64,1180802086169073253_u64,14615445439235962667_u64,10855665307997590858_u64];
_3.0 = _6 & _1.fld2.0;
_11 = 46935_u16;
_1.fld2 = (_3.0,);
_6 = _3.0 - _3.0;
_1.fld1 = !_4;
_4 = _9 as i128;
_8 = [_11];
RET.fld0[_9] = 9982150021686811517_u64;
_7 = RET.fld1;
Goto(bb2)
}
bb11 = {
_1.fld0 = core::ptr::addr_of!((*_5));
_13 = _6 != _6;
_8 = [_11];
RET.fld1 = _7;
_7 = RET.fld1;
_4 = _1.fld1;
_1.fld2 = _3;
RET.fld0 = [7707362838505889386_u64,15589599356141555680_u64,1974359533420486513_u64,2565737305302713192_u64,11309377376313446182_u64,1649446639333016639_u64,6445544677059682209_u64,13840938933963827096_u64];
_4 = _1.fld1;
_5 = _1.fld0;
RET.fld0[_9] = 15776360891945858606_u64;
Goto(bb3)
}
bb12 = {
(*_5) = 41_i8 as f64;
RET.fld1 = _7;
_8 = [_11];
_7 = RET.fld1;
RET.fld0[_9] = 9911682420331716941_u64 * 7713847672734250940_u64;
_4 = _1.fld1 | _1.fld1;
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
(*_5) = 140237631873147841739149609571406923069_u128 as f64;
(*_5) = 6_i8 as f64;
RET.fld0 = [9300581787671631510_u64,2451269056999081586_u64,15423339607618205457_u64,15562349797999782338_u64,543230384432965358_u64,12900977565678374378_u64,3736635181427031073_u64,3165008167200428326_u64];
_1.fld0 = _5;
_1.fld2 = (_6,);
_7 = RET.fld1;
RET.fld0 = [2510109613236929424_u64,8853440848485831069_u64,3727495179440405626_u64,16275801719816951025_u64,9539368692916178999_u64,4336499398622381239_u64,16900641444295105563_u64,10543244914831343467_u64];
_1 = Adt50 { fld0: _5,fld1: _4,fld2: _3 };
_9 = 14681316379553944014_usize * 3828999485853288859_usize;
_9 = 8924144745199684411_usize;
RET.fld1 = _7;
_15 = !_13;
_5 = _1.fld0;
_14 = (-117_isize);
_7 = RET.fld1;
(*_5) = 1845482882_i32 as f64;
_1.fld0 = core::ptr::addr_of!((*_5));
Goto(bb4)
}
bb13 = {
(*_22) = !_1.fld2.0;
RET.fld0 = [13526323838857989500_u64,5746238041755330997_u64,3499545815784293106_u64,10983618333850085640_u64,12392168547745568953_u64,7387699080781538521_u64,1935152526630667804_u64,5644092809552881949_u64];
Goto(bb14)
}
bb14 = {
_1.fld0 = _5;
(*_22) = !_1.fld2.0;
(*_5) = _18 as f64;
_1.fld2.0 = !(*_22);
_21 = (89_i8,);
_13 = _15;
(*_5) = 12610433449901493624_u64 as f64;
_13 = _15;
_14 = 9149_i16 as isize;
(*_22) = _7 as u32;
_3.0 = _15 as u32;
(*_22) = _2 as u32;
_24 = (*_22);
_10 = Adt48::Variant0 { fld0: (*_5),fld1: _2 };
RET.fld1 = _7;
_26 = [_15,_15,_15,_15,_13,_13,_15,_13];
RET.fld0 = [3682906970187885648_u64,17029110494768602160_u64,9992108091423920379_u64,15643878667509948852_u64,457835983630780760_u64,9956037951357852946_u64,7900458566099704938_u64,11489239757452459801_u64];
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(15_usize, 14_usize, Move(_14), 13_usize, Move(_13), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(15_usize, 21_usize, Move(_21), 6_usize, Move(_6), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(15_usize, 16_usize, Move(_16), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: Adt48,mut _2: u64,mut _3: bool,mut _4: *mut f64,mut _5: f64,mut _6: u64,mut _7: u16) -> isize {
mir! {
type RET = isize;
let _8: (u32,);
let _9: Adt48;
let _10: [bool; 8];
let _11: (i8, (u32,));
let _12: u64;
let _13: bool;
let _14: Adt53;
let _15: (i32, [i128; 5], u64, [bool; 1]);
let _16: f32;
let _17: i32;
let _18: f64;
let _19: [usize; 4];
let _20: i32;
let _21: [i16; 1];
let _22: (i8, (u32,));
let _23: Adt51;
let _24: ();
let _25: ();
{
_5 = -Field::<f64>(Variant(_1, 0), 0);
RET = 9223372036854775807_isize & (-9223372036854775808_isize);
Call(_2 = fn17(Move(_1), _5, RET, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_4) = -_5;
(*_4) = _5 - _5;
_2 = !_6;
(*_4) = _5 + _5;
_6 = RET as u64;
_11.0 = 78_i8 - (-77_i8);
_10 = [_3,_3,_3,_3,_3,_3,_3,_3];
_10 = [_3,_3,_3,_3,_3,_3,_3,_3];
_11.1 = (481599997_u32,);
match _11.1.0 {
0 => bb2,
481599997 => bb4,
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
_11.1.0 = 1_usize as u32;
_10 = [_3,_3,_3,_3,_3,_3,_3,_3];
_1 = Adt48::Variant0 { fld0: _5,fld1: 4_u8 };
_7 = 988287302611660407_i64 as u16;
_11.0 = 50058872393649931474893474953182816020_u128 as i8;
_1 = Adt48::Variant0 { fld0: (*_4),fld1: 110_u8 };
_10 = [_3,_3,_3,_3,_3,_3,_3,_3];
place!(Field::<u8>(Variant(_1, 0), 1)) = !57_u8;
_1 = Adt48::Variant0 { fld0: _5,fld1: 235_u8 };
_8 = _11.1;
_11.0 = _2 as i8;
Goto(bb5)
}
bb5 = {
_11.1 = (_8.0,);
_8.0 = _11.1.0 >> _11.0;
_6 = 28204924799440200818730667661565913090_u128 as u64;
RET = (-9223372036854775808_isize);
_11.1 = (_8.0,);
_14.fld0 = [(-1174_i16)];
Goto(bb6)
}
bb6 = {
_11.1.0 = _8.0 ^ _8.0;
_6 = _11.0 as u64;
_13 = !_3;
RET = _7 as isize;
(*_4) = -Field::<f64>(Variant(_1, 0), 0);
Goto(bb7)
}
bb7 = {
_11.1 = _8;
RET = !24_isize;
place!(Field::<f64>(Variant(_1, 0), 0)) = 62_u8 as f64;
_8 = _11.1;
_11.1.0 = _8.0 >> _6;
place!(Field::<u8>(Variant(_1, 0), 1)) = 118_u8 * 72_u8;
_13 = _3;
_11.1.0 = _8.0;
(*_4) = _5;
SetDiscriminant(_1, 0);
_12 = _2 & _2;
_4 = core::ptr::addr_of_mut!((*_4));
_7 = 45295_u16;
_4 = core::ptr::addr_of_mut!((*_4));
_4 = core::ptr::addr_of_mut!((*_4));
_1 = Adt48::Variant0 { fld0: (*_4),fld1: 39_u8 };
_13 = _3;
_5 = -Field::<f64>(Variant(_1, 0), 0);
_13 = !_3;
_10 = [_13,_13,_3,_3,_3,_3,_13,_3];
_4 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_1, 0), 0)));
place!(Field::<u8>(Variant(_1, 0), 1)) = !236_u8;
Call(_2 = core::intrinsics::bswap(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<f64>(Variant(_1, 0), 0)) = _5;
_1 = Adt48::Variant0 { fld0: _5,fld1: 188_u8 };
_15.2 = _12 >> _12;
_8 = _11.1;
_1 = Adt48::Variant0 { fld0: _5,fld1: 192_u8 };
_15.0 = (-272851651_i32) * 865837189_i32;
_19 = [4_usize,2558630652190506678_usize,9328275562444630155_usize,6_usize];
_8 = (_11.1.0,);
_15.1 = [(-56076669129308257737411393070048936734_i128),106023975115246689268749320453920162059_i128,(-118515944815658840930985838744293768430_i128),51268249915291994913698980664788168871_i128,11361186425693553628023332845403916670_i128];
place!(Field::<u8>(Variant(_1, 0), 1)) = _5 as u8;
_11.1.0 = !_8.0;
SetDiscriminant(_1, 1);
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{1010c5}';
place!(Field::<u16>(Variant(_1, 1), 2)) = _7;
_15.0 = 918747125_i32 ^ 233663736_i32;
Goto(bb9)
}
bb9 = {
_17 = -_15.0;
_18 = _5;
_15.0 = _8.0 as i32;
Goto(bb10)
}
bb10 = {
place!(Field::<((i8, (u32,)), u16)>(Variant(_1, 1), 3)).0.1 = (_8.0,);
_18 = -_5;
place!(Field::<((i8, (u32,)), u16)>(Variant(_1, 1), 3)).0 = _11;
_2 = _6;
_15.3 = [_3];
_8.0 = !_11.1.0;
_8.0 = !_11.1.0;
match _7 {
0 => bb1,
1 => bb8,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
45295 => bb16,
_ => bb15
}
}
bb11 = {
_17 = -_15.0;
_18 = _5;
_15.0 = _8.0 as i32;
Goto(bb10)
}
bb12 = {
place!(Field::<f64>(Variant(_1, 0), 0)) = _5;
_1 = Adt48::Variant0 { fld0: _5,fld1: 188_u8 };
_15.2 = _12 >> _12;
_8 = _11.1;
_1 = Adt48::Variant0 { fld0: _5,fld1: 192_u8 };
_15.0 = (-272851651_i32) * 865837189_i32;
_19 = [4_usize,2558630652190506678_usize,9328275562444630155_usize,6_usize];
_8 = (_11.1.0,);
_15.1 = [(-56076669129308257737411393070048936734_i128),106023975115246689268749320453920162059_i128,(-118515944815658840930985838744293768430_i128),51268249915291994913698980664788168871_i128,11361186425693553628023332845403916670_i128];
place!(Field::<u8>(Variant(_1, 0), 1)) = _5 as u8;
_11.1.0 = !_8.0;
SetDiscriminant(_1, 1);
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{1010c5}';
place!(Field::<u16>(Variant(_1, 1), 2)) = _7;
_15.0 = 918747125_i32 ^ 233663736_i32;
Goto(bb9)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_11.1 = (_8.0,);
_8.0 = _11.1.0 >> _11.0;
_6 = 28204924799440200818730667661565913090_u128 as u64;
RET = (-9223372036854775808_isize);
_11.1 = (_8.0,);
_14.fld0 = [(-1174_i16)];
Goto(bb6)
}
bb16 = {
place!(Field::<((i8, (u32,)), u16)>(Variant(_1, 1), 3)).1 = Field::<u16>(Variant(_1, 1), 2);
place!(Field::<u16>(Variant(_1, 1), 2)) = !_7;
_8 = (_11.1.0,);
_1 = Adt48::Variant0 { fld0: _18,fld1: 223_u8 };
_9 = Adt48::Variant0 { fld0: _5,fld1: 201_u8 };
_3 = _13;
_18 = _5;
_15.1 = [(-43030292916536224185160948540900435152_i128),78548296297357349716454611860718568689_i128,(-130750903866805229431691268888922354672_i128),(-134232163834138274886986169204957034152_i128),(-32237504318579619003897893284582875817_i128)];
_9 = Adt48::Variant0 { fld0: _18,fld1: 134_u8 };
_7 = _12 as u16;
_16 = _7 as f32;
_18 = _5;
_2 = _12 | _15.2;
_10 = [_13,_3,_3,_3,_13,_13,_13,_13];
_19 = [4_usize,5_usize,5419717395681980704_usize,3_usize];
_8.0 = _16 as u32;
place!(Field::<f64>(Variant(_9, 0), 0)) = _5;
place!(Field::<u8>(Variant(_9, 0), 1)) = _8.0 as u8;
_5 = 4_usize as f64;
place!(Field::<u8>(Variant(_9, 0), 1)) = 227_u8 >> _2;
_19 = [1_usize,4632490154292043956_usize,1_usize,0_usize];
_22 = (_11.0, _8);
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(16_usize, 3_usize, Move(_3), 8_usize, Move(_8), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(16_usize, 2_usize, Move(_2), 22_usize, Move(_22), 25_usize, _25, 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: Adt48,mut _2: f64,mut _3: isize,mut _4: u64) -> u64 {
mir! {
type RET = u64;
let _5: f64;
let _6: ((i8, (u32,)), u16);
let _7: ();
let _8: ();
{
RET = !_4;
_4 = !RET;
place!(Field::<u8>(Variant(_1, 0), 1)) = !78_u8;
place!(Field::<f64>(Variant(_1, 0), 0)) = -_2;
place!(Field::<u8>(Variant(_1, 0), 1)) = 82_u8 ^ 255_u8;
RET = !_4;
_4 = RET;
RET = !_4;
_5 = Field::<f64>(Variant(_1, 0), 0);
_2 = Field::<f64>(Variant(_1, 0), 0);
RET = !_4;
_3 = (-27_isize);
_4 = '\u{199a0}' as u64;
SetDiscriminant(_1, 1);
place!(Field::<usize>(Variant(_1, 1), 0)) = 6968690352340493038_usize;
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{2cebe}';
place!(Field::<((i8, (u32,)), u16)>(Variant(_1, 1), 3)).1 = 49453_u16;
place!(Field::<((i8, (u32,)), u16)>(Variant(_1, 1), 3)).0.1.0 = Field::<char>(Variant(_1, 1), 1) as u32;
place!(Field::<((i8, (u32,)), u16)>(Variant(_1, 1), 3)).0.0 = _3 as i8;
place!(Field::<usize>(Variant(_1, 1), 0)) = !2315014303208886799_usize;
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{16c5b}';
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(17_usize, 3_usize, Move(_3), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: bool,mut _2: bool,mut _3: f64,mut _4: bool,mut _5: *mut f64,mut _6: Adt42,mut _7: *mut f64,mut _8: *mut f64,mut _9: f64,mut _10: *mut f64) -> (i8, (u32,)) {
mir! {
type RET = (i8, (u32,));
let _11: f64;
let _12: (u32,);
let _13: f32;
let _14: f32;
let _15: i32;
let _16: Adt48;
let _17: [u64; 8];
let _18: i64;
let _19: isize;
let _20: [u64; 8];
let _21: i32;
let _22: i8;
let _23: isize;
let _24: f32;
let _25: ();
let _26: ();
{
place!(Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3)).0.1.0 = 3683833849_u32;
_2 = _1;
place!(Field::<usize>(Variant(_6, 1), 2)) = !4_usize;
_9 = (*_5);
RET.1.0 = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0;
RET.0 = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0 >> RET.1.0;
place!(Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3)).0.1.0 = RET.1.0 + RET.1.0;
RET.1 = (Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0,);
place!(Field::<[u16; 1]>(Variant(_6, 1), 1)) = [Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).1];
RET.1.0 = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0 * Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0;
(*_8) = _9 + _9;
(*_7) = 97_u8 as f64;
(*_7) = _3 - _3;
_11 = (*_5);
place!(Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3)).0.1.0 = !RET.1.0;
RET.1.0 = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0;
_5 = core::ptr::addr_of_mut!((*_7));
RET.1.0 = RET.0 as u32;
_2 = _4;
(*_5) = _11 - _11;
RET.1 = (Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0,);
_13 = 336654824224223505113068217630646662334_u128 as f32;
RET.0 = -Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0;
RET.1.0 = !Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1.0;
match Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211352 => bb7,
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
(*_5) = (-26452_i16) as f64;
_1 = (*_7) < _11;
(*_8) = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).1 as f64;
RET = (Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0, Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1);
_9 = -_11;
(*_7) = _3;
RET.0 = _4 as i8;
(*_10) = -_3;
RET = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0;
RET = (Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0, Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1);
SetDiscriminant(_6, 2);
_10 = _5;
_3 = -_11;
(*_8) = _3 * _9;
_5 = core::ptr::addr_of_mut!((*_10));
RET.1.0 = (-9223372036854775808_isize) as u32;
(*_10) = 11_isize as f64;
(*_5) = -_9;
_12.0 = RET.1.0;
_13 = 13762041405073528711_u64 as f32;
RET.1.0 = !_12.0;
Goto(bb8)
}
bb8 = {
_4 = !_2;
(*_10) = (-12_isize) as f64;
_17 = [13945103712164370612_u64,9701517715568014670_u64,11065289488872925930_u64,16819292138665366187_u64,2207727682563978239_u64,15001126901486121783_u64,4628683699026300513_u64,3843302517404162860_u64];
_12.0 = RET.1.0;
place!(Field::<[i128; 5]>(Variant(_6, 2), 1)) = [1852554711999453995315942647798613394_i128,(-67864446867557769365056705612900339427_i128),15465683336140417673054317953567780989_i128,2173153479938276197951934713895113024_i128,(-154777767849069841942267959157469164654_i128)];
(*_5) = _3 * _11;
(*_5) = _11;
Goto(bb9)
}
bb9 = {
RET.0 = (-2018523858_i32) as i8;
_4 = _2;
(*_10) = _9 + _3;
(*_7) = -_11;
_1 = _2;
(*_10) = -_11;
_17 = [7151034859312947799_u64,5010519520005669399_u64,4149829544063332657_u64,3632385773931279168_u64,7289390166546560151_u64,2720009470510335986_u64,4228028361620704104_u64,12403419993557670772_u64];
_13 = RET.0 as f32;
_12.0 = RET.1.0 * RET.1.0;
(*_8) = -_11;
RET = ((-46_i8), _12);
RET.0 = (-50_i8);
_16 = Adt48::Variant0 { fld0: (*_8),fld1: 91_u8 };
match RET.0 {
0 => bb8,
1 => bb4,
340282366920938463463374607431768211406 => bb11,
_ => bb10
}
}
bb10 = {
(*_5) = (-26452_i16) as f64;
_1 = (*_7) < _11;
(*_8) = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).1 as f64;
RET = (Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0, Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1);
_9 = -_11;
(*_7) = _3;
RET.0 = _4 as i8;
(*_10) = -_3;
RET = Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0;
RET = (Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.0, Field::<((i8, (u32,)), u16)>(Variant(_6, 1), 3).0.1);
SetDiscriminant(_6, 2);
_10 = _5;
_3 = -_11;
(*_8) = _3 * _9;
_5 = core::ptr::addr_of_mut!((*_10));
RET.1.0 = (-9223372036854775808_isize) as u32;
(*_10) = 11_isize as f64;
(*_5) = -_9;
_12.0 = RET.1.0;
_13 = 13762041405073528711_u64 as f32;
RET.1.0 = !_12.0;
Goto(bb8)
}
bb11 = {
_7 = core::ptr::addr_of_mut!(_9);
_3 = Field::<f64>(Variant(_16, 0), 0) + (*_5);
RET.0 = _12.0 as i8;
_9 = (*_5) * _3;
Goto(bb12)
}
bb12 = {
(*_7) = 5316532632629777135_u64 as f64;
_4 = !_2;
RET = ((-99_i8), _12);
RET = (127_i8, _12);
(*_5) = Field::<f64>(Variant(_16, 0), 0) - _11;
_10 = core::ptr::addr_of_mut!((*_5));
_11 = (*_8) + (*_5);
_16 = Adt48::Variant0 { fld0: (*_5),fld1: 34_u8 };
RET.1.0 = !_12.0;
RET.0 = (-52_i8) + 54_i8;
_9 = -(*_8);
_19 = _12.0 as isize;
place!(Field::<u8>(Variant(_16, 0), 1)) = !173_u8;
_17 = [14220420474944195061_u64,9457774924219881090_u64,1374063634994554900_u64,10728732107706465364_u64,2829837796662720712_u64,10412948124903366345_u64,11535828716491529859_u64,14325244555256812312_u64];
_16 = Adt48::Variant0 { fld0: (*_7),fld1: 234_u8 };
_10 = core::ptr::addr_of_mut!((*_8));
_12.0 = RET.1.0;
RET.1.0 = _12.0 >> RET.0;
(*_5) = _9 - _3;
place!(Field::<f64>(Variant(_16, 0), 0)) = (*_7) - (*_7);
(*_7) = -(*_5);
place!(Field::<u8>(Variant(_16, 0), 1)) = 66_u8 >> RET.0;
_10 = _8;
Goto(bb13)
}
bb13 = {
RET.1.0 = !_12.0;
_12 = (RET.1.0,);
(*_7) = 286034550415339736514639896182908091763_u128 as f64;
RET = (8_i8, _12);
Goto(bb14)
}
bb14 = {
(*_8) = 24816069071017494002369229221901246210_u128 as f64;
RET.1 = (_12.0,);
place!(Field::<f32>(Variant(_6, 2), 0)) = Field::<f64>(Variant(_16, 0), 0) as f32;
_8 = _10;
_11 = RET.0 as f64;
(*_8) = -_3;
(*_7) = (*_10) + _3;
_3 = 13591324171876012808_usize as f64;
_12 = RET.1;
_9 = (*_10) * (*_8);
_3 = (*_10) * Field::<f64>(Variant(_16, 0), 0);
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(18_usize, 4_usize, Move(_4), 2_usize, Move(_2), 12_usize, Move(_12), 26_usize, _26), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{b76d}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box((-93_i8)), std::hint::black_box((-12664_i16)), std::hint::black_box(1598385367_i32), std::hint::black_box((-8381533293761049263_i64)), std::hint::black_box((-29661893462023537327304514635128551180_i128)), std::hint::black_box(2_usize), std::hint::black_box(22_u8), std::hint::black_box(54833_u16), std::hint::black_box(19775530_u32), std::hint::black_box(17506694138316028807_u64), std::hint::black_box(297115925427449194816316737422426189040_u128));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: u16,
fld1: (i32, [i128; 5], u64, [bool; 1]),

},
Variant1{
fld0: [i16; 1],
fld1: i128,
fld2: (bool, bool, *mut u32),
fld3: *mut u32,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *mut (u32,),
fld1: (u32,),

},
Variant1{
fld0: i64,
fld1: [u16; 1],
fld2: usize,
fld3: ((i8, (u32,)), u16),

},
Variant2{
fld0: f32,
fld1: [i128; 5],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: char,
fld2: [bool; 8],
fld3: [i16; 1],

},
Variant1{
fld0: *const f64,
fld1: char,
fld2: [u64; 8],

},
Variant2{
fld0: *mut u32,
fld1: (bool, bool, *mut u32),

},
Variant3{
fld0: u16,
fld1: (u16,),
fld2: [u16; 1],
fld3: *mut (u32,),
fld4: *mut f64,
fld5: [bool; 8],
fld6: (u32,),
fld7: *mut [bool; 1],

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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: isize,
fld1: i8,

},
Variant1{
fld0: ((i8, (u32,)), u16),
fld1: char,
fld2: f64,
fld3: (i8,),
fld4: Adt41,
fld5: *mut u32,
fld6: usize,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: Adt43,
fld1: u128,
fld2: (u16,),
fld3: i16,

},
Variant1{
fld0: u8,
fld1: f32,
fld2: *mut (u32,),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [u16; 1],
fld1: (i8, (u32,)),
fld2: *mut (u32,),
fld3: ((i8, (u32,)), u16),
fld4: [u64; 8],
fld5: i32,

},
Variant1{
fld0: f32,
fld1: u64,
fld2: u128,
fld3: *mut [bool; 1],
fld4: i16,

},
Variant2{
fld0: *const f64,
fld1: *mut (u32,),
fld2: f32,

},
Variant3{
fld0: u32,
fld1: f32,
fld2: [i8; 7],
fld3: *mut u32,
fld4: [u16; 1],
fld5: [usize; 4],
fld6: [i128; 5],
fld7: (u32,),

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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (i8,),
fld1: Adt45,
fld2: isize,
fld3: [i8; 7],
fld4: [usize; 4],

},
Variant1{
fld0: (i32, [i128; 5], u64, [bool; 1]),
fld1: f32,
fld2: (i8, (u32,)),
fld3: Adt41,

},
Variant2{
fld0: u128,
fld1: char,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
fld0: f64,
fld1: u8,

},
Variant1{
fld0: usize,
fld1: char,
fld2: u16,
fld3: ((i8, (u32,)), u16),

},
Variant2{
fld0: *mut u32,
fld1: *mut (u32,),
fld2: ((i8, (u32,)), u16),
fld3: Adt46,
fld4: [u64; 8],
fld5: Adt47,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: *mut (u32,),
fld1: [i8; 7],
fld2: Adt44,
fld3: *mut u32,
fld4: f64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: *const f64,
fld1: i128,
fld2: (u32,),
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [u64; 8],
fld1: char,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt43,
fld1: [i8; 7],
fld2: usize,
fld3: Adt44,
fld4: i32,

},
Variant1{
fld0: *mut f64,
fld1: *mut [bool; 1],
fld2: Adt44,
fld3: (bool, bool, *mut u32),

},
Variant2{
fld0: [u64; 8],
fld1: char,
fld2: Adt43,
fld3: *mut u32,
fld4: (u32,),
fld5: [i16; 1],
fld6: i64,

},
Variant3{
fld0: i128,
fld1: *mut [bool; 1],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: [i16; 1],
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: [i16; 1],
fld1: (i8, (u32,)),
fld2: isize,
fld3: Adt51,
fld4: Adt41,
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: Adt52,
fld1: char,
fld2: *const f64,
fld3: Adt46,
fld4: f64,
fld5: u128,
fld6: Adt51,
fld7: (i8,),
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [usize; 4],
fld1: i64,
fld2: isize,
fld3: (u32,),

},
Variant1{
fld0: [i8; 7],
fld1: char,
fld2: Adt53,
fld3: Adt47,
fld4: (i8, (u32,)),
fld5: [i16; 1],

},
Variant2{
fld0: bool,
fld1: f32,
fld2: [i128; 5],
fld3: f64,
fld4: [i8; 7],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u128,
fld1: u16,

},
Variant1{
fld0: Adt55,
fld1: *mut [bool; 1],
fld2: Adt45,
fld3: u64,

},
Variant2{
fld0: bool,
fld1: Adt51,
fld2: u16,
fld3: Adt42,
fld4: [u64; 8],
fld5: Adt46,
fld6: i128,

},
Variant3{
fld0: Adt46,
fld1: char,
fld2: Adt51,
fld3: Adt49,

}}

