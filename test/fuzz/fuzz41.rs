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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u128,mut _11: u16,mut _12: u32,mut _13: u64) -> f32 {
mir! {
type RET = f32;
let _14: i16;
let _15: u64;
let _16: i32;
let _17: u64;
let _18: u8;
let _19: f32;
let _20: f32;
let _21: Adt45;
let _22: u16;
let _23: (bool,);
let _24: Adt47;
let _25: Adt56;
let _26: isize;
let _27: Adt56;
let _28: usize;
let _29: [u64; 6];
let _30: i8;
let _31: [u64; 6];
let _32: Adt45;
let _33: char;
let _34: (u128, i128);
let _35: char;
let _36: u16;
let _37: Adt47;
let _38: u16;
let _39: i32;
let _40: isize;
let _41: usize;
let _42: (bool,);
let _43: Adt54;
let _44: u16;
let _45: u64;
let _46: ([u128; 6], i8, f32, u8);
let _47: i32;
let _48: f64;
let _49: i64;
let _50: f32;
let _51: *mut ([u128; 6], i8, f32, u8);
let _52: bool;
let _53: u16;
let _54: ();
let _55: ();
{
_6 = -(-1949510183_i32);
_8 = 17255028067473181918_usize as i128;
RET = (-71_i8) as f32;
RET = 59270_u16 as f32;
_3 = 9223372036854775807_isize;
_11 = 60216_u16;
_1 = true;
_9 = 5_usize + 3771311123576702546_usize;
_2 = '\u{31a3c}';
_14 = 27009_i16;
_7 = _11 as i64;
Goto(bb1)
}
bb1 = {
_10 = 3582752617247746687327215899461472269_u128 & 324121664491733268922892692439968262440_u128;
_5 = _14 >> _11;
_9 = 5_usize ^ 8445651087787020377_usize;
_3 = !(-9223372036854775808_isize);
_13 = _10 as u64;
_16 = _6 >> _6;
_14 = _5 + _5;
_4 = -84_i8;
_11 = !51194_u16;
_12 = _16 as u32;
_8 = _7 as i128;
_18 = _11 as u8;
_4 = _12 as i8;
_1 = !true;
_17 = _13;
RET = _12 as f32;
RET = _4 as f32;
Goto(bb2)
}
bb2 = {
_8 = -115294362591631144380184977375229236330_i128;
_11 = !62806_u16;
_5 = -_14;
_19 = RET - RET;
_17 = _13 ^ _13;
_3 = 9223372036854775807_isize;
_19 = -RET;
_2 = '\u{fe5bb}';
_2 = '\u{18b67}';
_9 = !4_usize;
_15 = !_17;
_6 = _16 << _3;
_17 = _15;
Goto(bb3)
}
bb3 = {
_17 = _15;
_17 = _1 as u64;
_10 = !48611281419413465327127034025709817215_u128;
_15 = _13 - _17;
_1 = !true;
_16 = -_6;
_17 = _15 ^ _15;
_14 = _3 as i16;
_22 = _11 + _11;
_12 = _9 as u32;
_19 = _4 as f32;
_23 = (_1,);
_16 = _6 - _6;
_4 = _16 as i8;
_22 = _11 & _11;
Goto(bb4)
}
bb4 = {
_2 = '\u{cb01a}';
_15 = _17;
_12 = _4 as u32;
_4 = -65_i8;
_12 = _5 as u32;
_2 = '\u{c4b00}';
_10 = _3 as u128;
_3 = _12 as isize;
_20 = -_19;
_13 = _10 as u64;
_3 = _2 as isize;
_11 = !_22;
_12 = !134143355_u32;
_22 = !_11;
_26 = _3 ^ _3;
_7 = _15 as i64;
_16 = _8 as i32;
_2 = '\u{ac9b2}';
_7 = (-6674148467226936634_i64);
RET = _20 - _19;
_13 = _5 as u64;
_19 = RET;
_23.0 = _1;
_8 = 116601065785928163845298915433663022703_i128;
_23.0 = !_1;
_26 = _3 * _3;
Call(_6 = fn1(_9, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6 = _18 as i32;
_8 = _1 as i128;
_2 = '\u{d6c9d}';
_8 = (-114865151895105567007245708722256700326_i128) & (-137694913305327247501549053595409400657_i128);
Call(_22 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = _26;
_15 = _13 << _17;
_26 = _3;
_7 = -8151353603596364358_i64;
_8 = (-69939353051656899317872866986477062438_i128);
_17 = _15;
_22 = _13 as u16;
_9 = !0_usize;
_29 = [_13,_17,_15,_15,_15,_17];
_11 = _6 as u16;
_10 = 129143709086771819124870856312202730140_u128;
Goto(bb7)
}
bb7 = {
_9 = _15 as usize;
_1 = !_23.0;
_17 = _15;
_11 = !_22;
_33 = _2;
_31 = _29;
_15 = _18 as u64;
_34.1 = _8;
_19 = _17 as f32;
_5 = _33 as i16;
_34.1 = _8;
_19 = _10 as f32;
_15 = _13 | _17;
_9 = 7_usize;
_2 = _33;
_34.0 = _10;
match _10 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
129143709086771819124870856312202730140 => bb9,
_ => bb8
}
}
bb8 = {
_17 = _15;
_17 = _1 as u64;
_10 = !48611281419413465327127034025709817215_u128;
_15 = _13 - _17;
_1 = !true;
_16 = -_6;
_17 = _15 ^ _15;
_14 = _3 as i16;
_22 = _11 + _11;
_12 = _9 as u32;
_19 = _4 as f32;
_23 = (_1,);
_16 = _6 - _6;
_4 = _16 as i8;
_22 = _11 & _11;
Goto(bb4)
}
bb9 = {
_17 = _15;
_26 = -_3;
_1 = _15 != _17;
RET = _20 - _20;
_10 = _34.0 % _34.0;
_23.0 = _1;
_39 = _6;
_7 = !(-322449835925204656_i64);
_31 = [_15,_17,_17,_17,_17,_17];
_40 = _3 << _10;
_36 = _22;
_11 = _36;
_40 = _9 as isize;
_31 = [_13,_17,_17,_13,_15,_17];
RET = _18 as f32;
_13 = _15 << _11;
_42 = _23;
_23.0 = _1 >= _1;
_35 = _2;
_38 = _26 as u16;
_42.0 = !_23.0;
_26 = -_3;
match _34.1 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb10,
270343013869281564145501740445291149018 => bb12,
_ => bb11
}
}
bb10 = {
_6 = _18 as i32;
_8 = _1 as i128;
_2 = '\u{d6c9d}';
_8 = (-114865151895105567007245708722256700326_i128) & (-137694913305327247501549053595409400657_i128);
Call(_22 = core::intrinsics::transmute(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb11 = {
_17 = _15;
_17 = _1 as u64;
_10 = !48611281419413465327127034025709817215_u128;
_15 = _13 - _17;
_1 = !true;
_16 = -_6;
_17 = _15 ^ _15;
_14 = _3 as i16;
_22 = _11 + _11;
_12 = _9 as u32;
_19 = _4 as f32;
_23 = (_1,);
_16 = _6 - _6;
_4 = _16 as i8;
_22 = _11 & _11;
Goto(bb4)
}
bb12 = {
_10 = _9 as u128;
_26 = _3;
_20 = _6 as f32;
_6 = _18 as i32;
_14 = _9 as i16;
_40 = _3;
_7 = !6467169628064025886_i64;
_23.0 = _15 >= _17;
_34 = (_10, _8);
_23 = (_42.0,);
Call(_44 = core::intrinsics::bswap(_36), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_44 = _38 << _15;
_45 = _15 << _16;
_2 = _35;
_47 = _16;
_13 = _15 << _45;
_44 = _10 as u16;
_42.0 = _23.0 == _23.0;
_45 = _17;
_7 = 5935665781090879926_i64 ^ (-3233950508913198377_i64);
_4 = -(-124_i8);
_40 = !_26;
_46.0 = [_34.0,_34.0,_34.0,_34.0,_10,_10];
_34.1 = -_8;
_13 = !_17;
_39 = !_6;
_15 = _45;
_9 = 5103683429011483545_usize;
_19 = _20;
_14 = _3 as i16;
_46.2 = _36 as f32;
_2 = _35;
Goto(bb14)
}
bb14 = {
_8 = -_34.1;
_16 = _6 + _6;
_34.0 = !_10;
_20 = RET * _46.2;
_49 = !_7;
_20 = RET * _19;
_5 = _14;
_36 = _34.0 as u16;
_3 = !_40;
_19 = -RET;
_52 = _1;
_10 = _34.0;
_23.0 = _42.0 < _52;
_16 = _33 as i32;
_3 = _49 as isize;
_40 = _42.0 as isize;
_2 = _33;
_19 = _46.2 + _46.2;
_14 = _5 * _5;
_46.3 = _18 - _18;
_26 = _14 as isize;
_10 = _34.0;
_46.0 = [_34.0,_10,_10,_10,_10,_10];
_5 = _14;
_51 = core::ptr::addr_of_mut!(_46);
_17 = (*_51).3 as u64;
_31 = _29;
Goto(bb15)
}
bb15 = {
Call(_54 = dump_var(0_usize, 45_usize, Move(_45), 49_usize, Move(_49), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_54 = dump_var(0_usize, 8_usize, Move(_8), 38_usize, Move(_38), 14_usize, Move(_14), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(0_usize, 22_usize, Move(_22), 7_usize, Move(_7), 6_usize, Move(_6), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_54 = dump_var(0_usize, 2_usize, Move(_2), 40_usize, Move(_40), 15_usize, Move(_15), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_54 = dump_var(0_usize, 9_usize, Move(_9), 12_usize, Move(_12), 55_usize, _55, 55_usize, _55), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: usize,mut _2: i16) -> i32 {
mir! {
type RET = i32;
let _3: *mut (u64, [u128; 6], *mut f32, u64, u16);
let _4: [u64; 1];
let _5: *mut ([u128; 6], i8, f32, u8);
let _6: [u64; 6];
let _7: bool;
let _8: u128;
let _9: (isize, i64, i32);
let _10: char;
let _11: Adt49;
let _12: Adt48;
let _13: Adt51;
let _14: [i32; 7];
let _15: u64;
let _16: *mut f32;
let _17: [u64; 1];
let _18: ();
let _19: ();
{
RET = !98951769_i32;
_1 = 17160164191950752987_usize;
RET = (-759027815_i32) ^ (-1436245129_i32);
Call(RET = fn2(_1, _1, _2, _1, _1, _1, _2, _1, _1, _1, _2, _2, _2, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-612553919_i32) | (-1557949603_i32);
RET = 861056920_i32 * 1353806304_i32;
_2 = -16466_i16;
RET = 18351_u16 as i32;
Goto(bb2)
}
bb2 = {
RET = true as i32;
_4 = [349279697105508741_u64];
RET = (-2116667318_i32);
_2 = true as i16;
RET = 4051581384_u32 as i32;
_6 = [15961523961008441983_u64,12521815191955074135_u64,2230987016974400833_u64,6481161156572116535_u64,6858758115920235250_u64,391230743931617736_u64];
_1 = 16156085391861682065_usize << RET;
RET = -1159097668_i32;
_6 = [17131311939636287207_u64,15969801433730228330_u64,1670010949292850560_u64,6893545766945672537_u64,1607853673563135352_u64,13760003778610155917_u64];
_4 = [5581736775776769982_u64];
_6 = [135010796149248031_u64,5101327724846966590_u64,1478663382414483103_u64,15167874760063589621_u64,631889948776440161_u64,11330526164394615825_u64];
_1 = 17888013777565389165_usize;
_6 = [12830816351235933096_u64,17393950778226395615_u64,16797097219419381063_u64,6895344222618980079_u64,8056661667499479508_u64,18423536104363705674_u64];
match _1 {
0 => bb3,
1 => bb4,
17888013777565389165 => bb6,
_ => bb5
}
}
bb3 = {
RET = (-612553919_i32) | (-1557949603_i32);
RET = 861056920_i32 * 1353806304_i32;
_2 = -16466_i16;
RET = 18351_u16 as i32;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_1 = 1_usize;
RET = 87351616_i32 << _6[_1];
_2 = (-12238_i16) & 10144_i16;
_6[_1] = (-35016277535097740495458654608005622129_i128) as u64;
RET = !(-2052463342_i32);
RET = (-712241076_i32);
_2 = (-3522_i16) + (-2465_i16);
_6 = [10712410467803795813_u64,3016766674180053426_u64,5971306558925965667_u64,2745518227456536613_u64,12172705234961871948_u64,17997658848302558478_u64];
_6[_1] = 115973133790360296529401639685139786401_i128 as u64;
_6 = [4895275915382728656_u64,9347554952423587847_u64,6978365770921216070_u64,677441416413516454_u64,17228610141243727246_u64,390502964108232420_u64];
Goto(bb7)
}
bb7 = {
_1 = 2_usize;
_9 = ((-9223372036854775808_isize), 7262365768818365441_i64, RET);
_9.0 = 3966928568_u32 as isize;
_1 = !7_usize;
match _9.1 {
0 => bb3,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
7262365768818365441 => bb13,
_ => bb12
}
}
bb8 = {
_1 = 1_usize;
RET = 87351616_i32 << _6[_1];
_2 = (-12238_i16) & 10144_i16;
_6[_1] = (-35016277535097740495458654608005622129_i128) as u64;
RET = !(-2052463342_i32);
RET = (-712241076_i32);
_2 = (-3522_i16) + (-2465_i16);
_6 = [10712410467803795813_u64,3016766674180053426_u64,5971306558925965667_u64,2745518227456536613_u64,12172705234961871948_u64,17997658848302558478_u64];
_6[_1] = 115973133790360296529401639685139786401_i128 as u64;
_6 = [4895275915382728656_u64,9347554952423587847_u64,6978365770921216070_u64,677441416413516454_u64,17228610141243727246_u64,390502964108232420_u64];
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
RET = (-612553919_i32) | (-1557949603_i32);
RET = 861056920_i32 * 1353806304_i32;
_2 = -16466_i16;
RET = 18351_u16 as i32;
Goto(bb2)
}
bb11 = {
RET = (-612553919_i32) | (-1557949603_i32);
RET = 861056920_i32 * 1353806304_i32;
_2 = -16466_i16;
RET = 18351_u16 as i32;
Goto(bb2)
}
bb12 = {
RET = true as i32;
_4 = [349279697105508741_u64];
RET = (-2116667318_i32);
_2 = true as i16;
RET = 4051581384_u32 as i32;
_6 = [15961523961008441983_u64,12521815191955074135_u64,2230987016974400833_u64,6481161156572116535_u64,6858758115920235250_u64,391230743931617736_u64];
_1 = 16156085391861682065_usize << RET;
RET = -1159097668_i32;
_6 = [17131311939636287207_u64,15969801433730228330_u64,1670010949292850560_u64,6893545766945672537_u64,1607853673563135352_u64,13760003778610155917_u64];
_4 = [5581736775776769982_u64];
_6 = [135010796149248031_u64,5101327724846966590_u64,1478663382414483103_u64,15167874760063589621_u64,631889948776440161_u64,11330526164394615825_u64];
_1 = 17888013777565389165_usize;
_6 = [12830816351235933096_u64,17393950778226395615_u64,16797097219419381063_u64,6895344222618980079_u64,8056661667499479508_u64,18423536104363705674_u64];
match _1 {
0 => bb3,
1 => bb4,
17888013777565389165 => bb6,
_ => bb5
}
}
bb13 = {
_4 = [4228290849730375759_u64];
RET = -_9.2;
_10 = '\u{10f839}';
_9.1 = (-1065511334207257766_i64) * (-6855108034840831918_i64);
_8 = _2 as u128;
_14 = [_9.2,_9.2,RET,_9.2,_9.2,RET,RET];
_14 = [_9.2,_9.2,_9.2,RET,_9.2,RET,RET];
RET = _9.2 - _9.2;
_4 = [7758904471686537293_u64];
_8 = 53964441516574513457054688703821191830_u128 >> _9.0;
_8 = 284563336864494260403116351097617410588_u128;
_10 = '\u{598ce}';
_15 = 6268503061079296600_u64 & 4189895552467620427_u64;
_14 = [RET,_9.2,RET,RET,RET,RET,RET];
_2 = -(-12413_i16);
_9 = ((-9223372036854775808_isize), (-6528977409730754204_i64), RET);
RET = _1 as i32;
RET = _9.2 * _9.2;
_17 = [_15];
_7 = _1 > _1;
_15 = !16070158415341098838_u64;
match _9.1 {
340282366920938463456845630022037457252 => bb14,
_ => bb4
}
}
bb14 = {
_8 = 246845615094681926256598422754143490233_u128 * 303696265525287053881973619327019519411_u128;
_2 = 15124_i16;
_8 = !11118948042979322763811231926230242822_u128;
_4 = [_15];
_10 = '\u{c5581}';
_6 = [_15,_15,_15,_15,_15,_15];
_9.2 = RET;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(1_usize, 9_usize, Move(_9), 1_usize, Move(_1), 6_usize, Move(_6), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(1_usize, 4_usize, Move(_4), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: usize,mut _2: usize,mut _3: i16,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: i16,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: i16,mut _12: i16,mut _13: i16,mut _14: usize,mut _15: usize) -> i32 {
mir! {
type RET = i32;
let _16: ([u128; 6], i8, f32, u8);
let _17: i8;
let _18: char;
let _19: (isize, i64, i32);
let _20: char;
let _21: ([u128; 6], i8, f32, u8);
let _22: char;
let _23: *mut *const bool;
let _24: bool;
let _25: [usize; 5];
let _26: bool;
let _27: [i32; 7];
let _28: f32;
let _29: [usize; 5];
let _30: u16;
let _31: *mut (u64, [u128; 6], *mut f32, u64, u16);
let _32: (u64, [u128; 6], *mut f32, u64, u16);
let _33: [u128; 6];
let _34: u8;
let _35: ([u128; 6], i8, f32, u8);
let _36: char;
let _37: bool;
let _38: usize;
let _39: ();
let _40: ();
{
RET = !(-1077676843_i32);
_16.2 = 9223372036854775807_isize as f32;
_8 = _9;
_11 = false as i16;
_16.0 = [153542100746401342134385180599788745578_u128,300839802118930750791707021904540972652_u128,320938622891902658735237935662491856362_u128,195504616110335795731563966091410737863_u128,180421137754770735983858841979755835726_u128,20136210541761977080965726348523035868_u128];
_16.0 = [51605885186385071682330549609363014539_u128,149728244457848710648428429543049625042_u128,296927052966184779434824467625501551694_u128,109739099287980963391212714085078977518_u128,215382453299912793785843026556260157299_u128,134707899798191834258918005042158497391_u128];
_19.0 = 9223372036854775807_isize;
_2 = _8 % _5;
_19.1 = (-806681197527924099_i64) - 1653294831118574667_i64;
_16.0 = [232748053586002073535957479370443121030_u128,155562737180682173991173088804890228254_u128,264178701652419139054410940424210747715_u128,180153772709318733590494687425290518257_u128,152992061012354579860032233307232554282_u128,323740351036428979964495349571569450593_u128];
_4 = _10;
RET = (-751179518_i32) - (-1046482563_i32);
Goto(bb1)
}
bb1 = {
_6 = !_15;
_4 = 25464613123852304766788262331846599994_u128 as usize;
_19.2 = RET >> _9;
_16.2 = (-33_i8) as f32;
_15 = 21690_u16 as usize;
_17 = 73_i8;
_20 = '\u{b9ab5}';
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
17160164191950752987 => bb8,
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
_16.1 = -_17;
_18 = _20;
_19.0 = _17 as isize;
_9 = !_2;
_22 = _20;
_16.2 = _12 as f32;
_18 = _22;
_22 = _18;
_17 = _16.1;
_21.0 = [301430020427068705431241640137700248237_u128,33861072994306743930042823721163575734_u128,245442612219588375598598808856181994811_u128,164915916868626585755670134411844169772_u128,242747598543509922999391768803815334778_u128,118107545353435756466310723011404367621_u128];
_9 = !_6;
RET = !_19.2;
_5 = 1720707233_u32 as usize;
_19.2 = RET;
RET = _19.2;
_12 = -_3;
_4 = _15 % _10;
_21.0 = [88751632177765123586218278497790076198_u128,14760351996247075538343684628819554636_u128,324358754995526034820289056698310255873_u128,107350820337218883183708696830210679603_u128,190624070987541521208748810429602073988_u128,172803202425245247855670100960514136413_u128];
_26 = !true;
Call(_21 = fn3(_8, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_21.0 = [78783505831377576660705290134559070238_u128,84708217099275077016324158450170962367_u128,131714473121998833651120956710344406966_u128,326344134755643647630323879927885554368_u128,68564557679841038911023463196316155691_u128,38870888908522052174220714215841991137_u128];
_10 = _8 >> _14;
_16.0 = [235582139906258343933728927405212413486_u128,159791572616311662014375856292397413761_u128,13236132488973889698983576163700211259_u128,249346987158942157001444390779655323914_u128,306236784406068013499530293817573109693_u128,88748737858104477364842269336699096931_u128];
_16.2 = _21.2;
_22 = _18;
_15 = _19.0 as usize;
_19.1 = (-4261296079454710351_i64) + 6219325349588659433_i64;
_21.0 = [112042337821153316672273481680806940740_u128,203742244705950622642484567364348794331_u128,277243425449735065481700423166908315535_u128,141854965999472964259946400138478259886_u128,86250138731010364288970460836022789376_u128,184897944004136964234249108519562439560_u128];
_16 = (_21.0, _17, _21.2, _21.3);
_14 = _4 * _4;
_12 = -_7;
_16 = (_21.0, _21.1, _21.2, _21.3);
_19.0 = !(-19_isize);
_5 = _21.1 as usize;
_26 = true;
_9 = _1 * _4;
_28 = _17 as f32;
_21.1 = _16.1;
_2 = _5;
_27 = [RET,_19.2,RET,_19.2,RET,RET,_19.2];
_4 = _8 & _8;
_26 = false;
_21.0 = [192206762400453004817151177703752997883_u128,227363524800925084513599099136832080939_u128,225385693719840976231497176578786567791_u128,83306610083639607404613805042592037862_u128,27870323112347842759080605293392300559_u128,161723517876946602324940319026344839522_u128];
_16 = _21;
_16.2 = _28 + _28;
_28 = _19.1 as f32;
Call(_16.0 = fn7(_21.1, _19.2, _21.0, _4, _8, _21, _21, _19, _1, _4, _19.0, _27), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7 = _12 ^ _13;
_24 = _6 == _2;
_16.3 = !_21.3;
_25 = [_10,_4,_14,_10,_14];
_29 = [_6,_10,_9,_10,_5];
_24 = _26;
_12 = _22 as i16;
_12 = 49353529990509005025293210698115918295_i128 as i16;
_6 = !_9;
_4 = _9 * _14;
_16 = (_21.0, _21.1, _28, _21.3);
_2 = _10 - _5;
_11 = -_12;
_28 = _16.2 * _16.2;
_16.1 = _16.2 as i8;
_28 = _21.2;
_1 = (-76704547115521973468930601099101701093_i128) as usize;
_30 = 52627_u16;
_10 = _14;
_18 = _22;
RET = _19.2 & _19.2;
_13 = 279307414983822519141383676060368325460_u128 as i16;
_5 = !_4;
_16.0 = _21.0;
_16.2 = _17 as f32;
_6 = _10;
_8 = _6;
_21.3 = _13 as u8;
_21.1 = _16.1;
Call(_21.2 = core::intrinsics::transmute(_20), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_17 = _16.1 * _21.1;
_2 = !_5;
_21.0 = [37837225568344019925349244417086253792_u128,230456028287427695677683786588676587523_u128,306940366194526596989162658983610484108_u128,17706130402072501800858410774486789750_u128,212168507618642837056836134032492648729_u128,277150446429316830213266138870300330813_u128];
_5 = 1680347015041179266_u64 as usize;
_19.2 = RET * RET;
_29 = [_4,_1,_15,_10,_10];
_19.2 = _26 as i32;
_6 = _2;
_30 = !58384_u16;
Goto(bb12)
}
bb12 = {
_5 = !_2;
_9 = _2 << _6;
_18 = _22;
_30 = _7 as u16;
Call(_21.3 = core::intrinsics::transmute(_17), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_11 = _17 as i16;
_29 = [_4,_2,_5,_10,_2];
_32.3 = 13130040534114310416_u64 & 7787238527635710552_u64;
RET = _19.2 * _19.2;
_33 = [116634541669125857463430863832218387491_u128,182327389531944566923219076080371919938_u128,189441894985261326657555413552034129513_u128,173767167946468210811119054270867399039_u128,132052536692410920065586863648360535307_u128,215115159700816639461994491340537815017_u128];
_32.2 = core::ptr::addr_of_mut!(_16.2);
_17 = _21.3 as i8;
Goto(bb14)
}
bb14 = {
_29 = [_9,_9,_2,_4,_9];
RET = _19.2 >> _14;
_32.4 = !_30;
_32.1 = [283849573565667120926722102357286305822_u128,335485304384713995696957745550344783937_u128,310688381046921993492457822974521368318_u128,324798074314186219216486658762952982197_u128,194052669357338103299746197546127223019_u128,233510562326822787551088241310425043451_u128];
_13 = _21.1 as i16;
_29 = [_6,_2,_14,_6,_8];
_30 = _32.4;
_30 = _20 as u16;
_32.1 = [8689015089355993553045183642633561818_u128,208981271617919371857765386767773363981_u128,10792127961955247614597310024981373091_u128,309048256029512062121142073364377745389_u128,157936909614457478227103306164838577615_u128,230561644443320817246461061737663064163_u128];
_35.1 = _32.4 as i8;
_35.3 = !_16.3;
_19.2 = RET ^ RET;
_36 = _20;
_29 = _25;
_6 = !_5;
_35.3 = _16.3 - _21.3;
_22 = _20;
_32.4 = _30;
_20 = _22;
_22 = _36;
_20 = _22;
_19.1 = (-4122211308108621667_i64);
_19.2 = RET & RET;
_27 = [RET,_19.2,_19.2,_19.2,_19.2,RET,_19.2];
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(2_usize, 17_usize, Move(_17), 10_usize, Move(_10), 29_usize, Move(_29), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(2_usize, 25_usize, Move(_25), 30_usize, Move(_30), 22_usize, Move(_22), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(2_usize, 12_usize, Move(_12), 19_usize, Move(_19), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(2_usize, 2_usize, Move(_2), 33_usize, Move(_33), 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: usize,mut _2: usize) -> ([u128; 6], i8, f32, u8) {
mir! {
type RET = ([u128; 6], i8, f32, u8);
let _3: [u8; 4];
let _4: [u8; 4];
let _5: (bool,);
let _6: *const bool;
let _7: isize;
let _8: f64;
let _9: f64;
let _10: isize;
let _11: *mut *const bool;
let _12: (isize, i64, i32);
let _13: f32;
let _14: isize;
let _15: [u64; 7];
let _16: [u64; 7];
let _17: (isize, i64, i32);
let _18: i128;
let _19: [u32; 6];
let _20: [usize; 5];
let _21: [u64; 6];
let _22: bool;
let _23: i32;
let _24: [u64; 1];
let _25: *mut i32;
let _26: u8;
let _27: ();
let _28: ();
{
RET.1 = 43_i8 * 97_i8;
RET.2 = RET.1 as f32;
RET.2 = (-66626886110552344715707963733290986896_i128) as f32;
_2 = _1 & _1;
_1 = _2 & _2;
RET.3 = 254_u8;
RET.1 = -118_i8;
RET.0 = [171704041778162161902361778083890476652_u128,280024888217196087069188276064522081020_u128,12037468158438294391806665670010794491_u128,271598255293251888243537948521116344176_u128,308858889805450848515126938549872082878_u128,155575765207573998071821730650204207554_u128];
RET.2 = RET.1 as f32;
RET.1 = false as i8;
_3 = [RET.3,RET.3,RET.3,RET.3];
RET.3 = !8_u8;
_3 = [RET.3,RET.3,RET.3,RET.3];
_1 = _2;
_3 = [RET.3,RET.3,RET.3,RET.3];
_5.0 = true;
RET.2 = 7985463114692060889_u64 as f32;
Goto(bb1)
}
bb1 = {
_5 = (false,);
RET.1 = !11_i8;
_8 = RET.1 as f64;
_1 = !_2;
RET.0 = [322303372265141104942104060590924141567_u128,294676761361500156830695883095325406389_u128,212151690619153257378857087823693450838_u128,320837715115661380232042072112227640618_u128,314671960565608152839399749714737107265_u128,218584345527804141075724595918214884403_u128];
RET.2 = RET.3 as f32;
_7 = (-27_isize) << _1;
_6 = core::ptr::addr_of!(_5.0);
_2 = _1 + _1;
_2 = _1 & _1;
_8 = 58522_u16 as f64;
(*_6) = false;
_4 = _3;
_9 = (-5365716821898301308_i64) as f64;
Goto(bb2)
}
bb2 = {
_3 = [RET.3,RET.3,RET.3,RET.3];
_8 = -_9;
_1 = !_2;
_8 = _9 * _9;
RET.3 = 60706272191759531825416335720365913779_i128 as u8;
_8 = 482217132_i32 as f64;
(*_6) = !false;
_7 = (-9223372036854775808_isize);
Call(_12 = fn4(RET, _7, RET.0, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.2 = _7 as i32;
RET.0 = [200497120031093286344731172056853088701_u128,130719167407498573272290642428050085301_u128,114047171857129986168523101088277662756_u128,153046955414772371708014588821261694404_u128,50909248177290711237717320560443503777_u128,104302992130956715187416003283863617736_u128];
_11 = core::ptr::addr_of_mut!(_6);
_5.0 = !false;
_13 = RET.2;
RET.1 = -1_i8;
RET.1 = 118_i8;
_7 = _12.0;
Goto(bb4)
}
bb4 = {
_9 = 109697191143348555248708591802777168167_u128 as f64;
_12.2 = 15856249253178269765_u64 as i32;
(*_11) = core::ptr::addr_of!((*_6));
_10 = _7 ^ _12.0;
RET.2 = _13;
Goto(bb5)
}
bb5 = {
_1 = _2 & _2;
RET.2 = _13 * _13;
(*_6) = false;
_11 = core::ptr::addr_of_mut!((*_11));
RET.3 = _2 as u8;
RET.3 = !212_u8;
_4 = _3;
RET.1 = 57_i8 >> _10;
_12 = (_10, (-7535651259022129546_i64), 1874959715_i32);
_15 = [1459633143474207677_u64,13047272290304567573_u64,9443659847716109265_u64,15827806470553106590_u64,9599161311223089375_u64,12205992435640469586_u64,17872143118065924222_u64];
RET.2 = _13 - _13;
_2 = _1;
_12.1 = !(-3573314276221058941_i64);
_12.1 = RET.1 as i64;
_12 = (_7, (-664792779741047649_i64), (-1889051411_i32));
RET.1 = 9_i8;
RET.2 = _10 as f32;
_6 = core::ptr::addr_of!((*_6));
match RET.1 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
9 => bb12,
_ => bb11
}
}
bb6 = {
_9 = 109697191143348555248708591802777168167_u128 as f64;
_12.2 = 15856249253178269765_u64 as i32;
(*_11) = core::ptr::addr_of!((*_6));
_10 = _7 ^ _12.0;
RET.2 = _13;
Goto(bb5)
}
bb7 = {
_12.2 = _7 as i32;
RET.0 = [200497120031093286344731172056853088701_u128,130719167407498573272290642428050085301_u128,114047171857129986168523101088277662756_u128,153046955414772371708014588821261694404_u128,50909248177290711237717320560443503777_u128,104302992130956715187416003283863617736_u128];
_11 = core::ptr::addr_of_mut!(_6);
_5.0 = !false;
_13 = RET.2;
RET.1 = -1_i8;
RET.1 = 118_i8;
_7 = _12.0;
Goto(bb4)
}
bb8 = {
_3 = [RET.3,RET.3,RET.3,RET.3];
_8 = -_9;
_1 = !_2;
_8 = _9 * _9;
RET.3 = 60706272191759531825416335720365913779_i128 as u8;
_8 = 482217132_i32 as f64;
(*_6) = !false;
_7 = (-9223372036854775808_isize);
Call(_12 = fn4(RET, _7, RET.0, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_5 = (false,);
RET.1 = !11_i8;
_8 = RET.1 as f64;
_1 = !_2;
RET.0 = [322303372265141104942104060590924141567_u128,294676761361500156830695883095325406389_u128,212151690619153257378857087823693450838_u128,320837715115661380232042072112227640618_u128,314671960565608152839399749714737107265_u128,218584345527804141075724595918214884403_u128];
RET.2 = RET.3 as f32;
_7 = (-27_isize) << _1;
_6 = core::ptr::addr_of!(_5.0);
_2 = _1 + _1;
_2 = _1 & _1;
_8 = 58522_u16 as f64;
(*_6) = false;
_4 = _3;
_9 = (-5365716821898301308_i64) as f64;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
RET.2 = _13;
_14 = (*_6) as isize;
_16 = _15;
RET.1 = (-77_i8);
_12.2 = 120134237576863414929499873050227259098_u128 as i32;
_15 = _16;
_1 = !_2;
_11 = core::ptr::addr_of_mut!((*_11));
(*_11) = core::ptr::addr_of!((*_6));
_12.0 = (-18654_i16) as isize;
RET.2 = _13 + _13;
_18 = 103466093911453782946633345217619294922_i128 * 2772450629714903774262542034007397940_i128;
_2 = _1 - _1;
(*_11) = core::ptr::addr_of!(_5.0);
_6 = core::ptr::addr_of!(_5.0);
_18 = 66561626972935742020414234616465164133_i128 << _1;
_10 = 48_i16 as isize;
_16 = [681861107982852600_u64,11635409394828280169_u64,17571433884771117904_u64,2285613134167082803_u64,1708607199954236233_u64,5106354165225072126_u64,11886855393415014785_u64];
_21 = [3786858647110448680_u64,11460119223554365317_u64,4904686398847581201_u64,2945849168411306951_u64,11786560259162061980_u64,15577766123604007019_u64];
_19 = [4011759767_u32,3008239539_u32,17314257_u32,820237113_u32,2284721617_u32,1043321194_u32];
Goto(bb13)
}
bb13 = {
_6 = core::ptr::addr_of!(_22);
(*_6) = _5.0 ^ _5.0;
_23 = 3669880579_u32 as i32;
_17 = (_14, _12.1, _23);
_11 = core::ptr::addr_of_mut!((*_11));
_24 = [8222865852812337813_u64];
_5 = ((*_6),);
_10 = -_17.0;
RET.3 = 106768079942062556452375564595900174543_u128 as u8;
_17.2 = (*_6) as i32;
_8 = _9 + _9;
Goto(bb14)
}
bb14 = {
_5.0 = !(*_6);
(*_11) = core::ptr::addr_of!((*_6));
(*_6) = _12.1 > _17.1;
_21 = [6234817480324779406_u64,99378044135574939_u64,16672202856906115015_u64,2900171590721918702_u64,18143845491451380960_u64,8799373943650087145_u64];
_20 = [_1,_2,_1,_1,_2];
RET.2 = -_13;
_15 = [9022754572374612455_u64,330597220300350336_u64,10733387074613407369_u64,2966113162924020252_u64,13933185364281538817_u64,1767889067711978472_u64,5289977621879642831_u64];
_8 = _9 * _9;
_20 = [_2,_1,_2,_2,_1];
_17.2 = _23 >> _1;
_1 = _2 ^ _2;
RET.2 = -_13;
_4 = _3;
_15 = [16527115245538356066_u64,1093745333425675888_u64,12454683971176138915_u64,4975469937687809055_u64,7136549835200872341_u64,10461003774529010133_u64,381510983324212322_u64];
RET.1 = 76_i8;
_5.0 = !(*_6);
RET.2 = -_13;
_17.2 = _12.2 & _12.2;
RET.0 = [107505422816659279135846082739200559902_u128,98960678398205170370742112963129800897_u128,253836393835884321649117041321394517764_u128,32287695807099908552069392062936553942_u128,130637557623780865728038538501970219210_u128,261066877755033992307498309541513765160_u128];
_6 = core::ptr::addr_of!(_5.0);
_8 = -_9;
RET.2 = _13;
_9 = _8 * _8;
_4 = _3;
(*_6) = !_22;
_17.1 = !_12.1;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(3_usize, 1_usize, Move(_1), 18_usize, Move(_18), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(3_usize, 17_usize, Move(_17), 20_usize, Move(_20), 22_usize, Move(_22), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(3_usize, 4_usize, Move(_4), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: ([u128; 6], i8, f32, u8),mut _2: isize,mut _3: [u128; 6],mut _4: usize) -> (isize, i64, i32) {
mir! {
type RET = (isize, i64, i32);
let _5: isize;
let _6: isize;
let _7: [u128; 6];
let _8: [u8; 4];
let _9: [u64; 6];
let _10: (isize, i64, i32);
let _11: [u64; 1];
let _12: [u64; 7];
let _13: [bool; 7];
let _14: (u128, i128);
let _15: [bool; 7];
let _16: isize;
let _17: bool;
let _18: *mut ([u128; 6], i8, f32, u8);
let _19: f64;
let _20: f32;
let _21: isize;
let _22: f64;
let _23: bool;
let _24: [bool; 7];
let _25: f32;
let _26: ();
let _27: ();
{
RET = (_2, 3454640325062389808_i64, (-187361401_i32));
_1.1 = !97_i8;
_1.2 = 1419002360_u32 as f32;
_2 = 48787_u16 as isize;
RET.0 = !_2;
_4 = _1.1 as usize;
RET = (_2, (-8712116397419593793_i64), 1191452634_i32);
RET.2 = -(-1983322521_i32);
RET.2 = _4 as i32;
_1.0 = _3;
RET.0 = _2 * _2;
_3 = _1.0;
_1.3 = !201_u8;
_5 = RET.0;
_3 = _1.0;
RET = (_5, 3472587914743879128_i64, 1178661631_i32);
_1.3 = !158_u8;
RET.2 = (-14276_i16) as i32;
_8 = [_1.3,_1.3,_1.3,_1.3];
match RET.1 {
0 => bb1,
3472587914743879128 => bb3,
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
_3 = [277688344436919570418634396656944194429_u128,97568780357937892721016882531373231405_u128,250847856065317558281139206827663025197_u128,171097580074088743176034105795320257854_u128,74937229527903161650808703585493891223_u128,19927938062385391067581919298559716975_u128];
_1.3 = !61_u8;
Goto(bb4)
}
bb4 = {
_8 = [_1.3,_1.3,_1.3,_1.3];
_6 = !_5;
_7 = [315750964108379795813860151907361773446_u128,133193863078235996026419009755136953887_u128,195022370027752079322892124292433957481_u128,201050993318447205683041920609176584637_u128,318778217915010775623704824993277859858_u128,169583715186544769626565442677167039838_u128];
_3 = [172809780777031102819764593235862619172_u128,182621451158432121982708270334906413424_u128,338859308140948698104147463927950108421_u128,184910218021233809605816680735466029619_u128,167751331892096623340770167337095085240_u128,241444021423470704439402596763418078095_u128];
_3 = _7;
_1.1 = 52_i8;
_2 = _6;
_10.1 = _4 as i64;
match RET.1 {
3472587914743879128 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_2 = '\u{f5c41}' as isize;
_10.2 = !RET.2;
RET = (_2, _10.1, _10.2);
_10 = RET;
_1.1 = (-61_i8);
RET = (_5, _10.1, _10.2);
_10.1 = !RET.1;
_8 = [_1.3,_1.3,_1.3,_1.3];
RET.0 = !_5;
RET.2 = _10.2;
_13 = [true,false,true,true,false,false,true];
_2 = _10.0 - _6;
RET.0 = !_6;
_11 = [2891141445514908603_u64];
_12 = [18337804305938899793_u64,11131030926151729884_u64,2415035532411784034_u64,4751378940405818644_u64,2981941225332654233_u64,3783495464986052166_u64,314052043127364874_u64];
_14.0 = RET.2 as u128;
RET = (_5, _10.1, _10.2);
RET.1 = _10.1;
_14.1 = (-53645442583119591818980301610461885596_i128) | (-7424197442107974517130066014320014937_i128);
_7 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_8 = [_1.3,_1.3,_1.3,_1.3];
RET.1 = _10.1;
match _1.1 {
340282366920938463463374607431768211395 => bb8,
_ => bb7
}
}
bb7 = {
_3 = [277688344436919570418634396656944194429_u128,97568780357937892721016882531373231405_u128,250847856065317558281139206827663025197_u128,171097580074088743176034105795320257854_u128,74937229527903161650808703585493891223_u128,19927938062385391067581919298559716975_u128];
_1.3 = !61_u8;
Goto(bb4)
}
bb8 = {
RET.0 = _4 as isize;
_1.0 = _7;
_10 = (_5, RET.1, RET.2);
_12 = [17980014345912826656_u64,18093766484613142362_u64,18321387209058033959_u64,10488566238972561479_u64,8695230098775393936_u64,10551633665205894708_u64,4310294402558867859_u64];
_4 = 10292671161179977197_usize >> RET.0;
_1.0 = _7;
RET.2 = _10.2;
_11 = [12275675849162363616_u64];
_1.2 = _10.2 as f32;
_16 = !_10.0;
RET = (_6, _10.1, _10.2);
_13 = [false,false,false,true,true,false,true];
RET.1 = _10.1 ^ _10.1;
_15 = [true,true,false,true,true,false,true];
RET.1 = _10.1;
_10.0 = _1.3 as isize;
RET.1 = _10.1;
_15 = _13;
_12 = [17182608447912976500_u64,17013827867632234770_u64,13182429294758873958_u64,17814420437012289660_u64,11176014023922608182_u64,12911006988877749141_u64,1129112967354027513_u64];
_9 = [5770266991068362265_u64,4401823920464994688_u64,13033598793892507521_u64,11561602727962547644_u64,8442384307947035734_u64,12339415950704983410_u64];
_17 = _2 != _16;
_14.0 = 148557409078693108438133852748636283704_u128 * 259655125904592231750272780873674602405_u128;
RET.2 = _10.2;
_5 = _16;
match _1.1 {
0 => bb9,
1 => bb10,
340282366920938463463374607431768211395 => bb12,
_ => bb11
}
}
bb9 = {
Return()
}
bb10 = {
_8 = [_1.3,_1.3,_1.3,_1.3];
_6 = !_5;
_7 = [315750964108379795813860151907361773446_u128,133193863078235996026419009755136953887_u128,195022370027752079322892124292433957481_u128,201050993318447205683041920609176584637_u128,318778217915010775623704824993277859858_u128,169583715186544769626565442677167039838_u128];
_3 = [172809780777031102819764593235862619172_u128,182621451158432121982708270334906413424_u128,338859308140948698104147463927950108421_u128,184910218021233809605816680735466029619_u128,167751331892096623340770167337095085240_u128,241444021423470704439402596763418078095_u128];
_3 = _7;
_1.1 = 52_i8;
_2 = _6;
_10.1 = _4 as i64;
match RET.1 {
3472587914743879128 => bb6,
_ => bb5
}
}
bb11 = {
Return()
}
bb12 = {
_10 = (RET.0, RET.1, RET.2);
_10 = (_16, RET.1, RET.2);
Call(_14 = fn5(_8, _3, RET, _3, _12, RET.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_1.3 = 145_u8;
_10.0 = RET.0;
_13 = [_17,_17,_17,_17,_17,_17,_17];
_10 = (_5, RET.1, RET.2);
RET = _10;
_18 = core::ptr::addr_of_mut!(_1);
(*_18).3 = _17 as u8;
_19 = 32340_u16 as f64;
_20 = -(*_18).2;
match _1.1 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
340282366920938463463374607431768211395 => bb20,
_ => bb19
}
}
bb14 = {
Return()
}
bb15 = {
_2 = '\u{f5c41}' as isize;
_10.2 = !RET.2;
RET = (_2, _10.1, _10.2);
_10 = RET;
_1.1 = (-61_i8);
RET = (_5, _10.1, _10.2);
_10.1 = !RET.1;
_8 = [_1.3,_1.3,_1.3,_1.3];
RET.0 = !_5;
RET.2 = _10.2;
_13 = [true,false,true,true,false,false,true];
_2 = _10.0 - _6;
RET.0 = !_6;
_11 = [2891141445514908603_u64];
_12 = [18337804305938899793_u64,11131030926151729884_u64,2415035532411784034_u64,4751378940405818644_u64,2981941225332654233_u64,3783495464986052166_u64,314052043127364874_u64];
_14.0 = RET.2 as u128;
RET = (_5, _10.1, _10.2);
RET.1 = _10.1;
_14.1 = (-53645442583119591818980301610461885596_i128) | (-7424197442107974517130066014320014937_i128);
_7 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_8 = [_1.3,_1.3,_1.3,_1.3];
RET.1 = _10.1;
match _1.1 {
340282366920938463463374607431768211395 => bb8,
_ => bb7
}
}
bb16 = {
_8 = [_1.3,_1.3,_1.3,_1.3];
_6 = !_5;
_7 = [315750964108379795813860151907361773446_u128,133193863078235996026419009755136953887_u128,195022370027752079322892124292433957481_u128,201050993318447205683041920609176584637_u128,318778217915010775623704824993277859858_u128,169583715186544769626565442677167039838_u128];
_3 = [172809780777031102819764593235862619172_u128,182621451158432121982708270334906413424_u128,338859308140948698104147463927950108421_u128,184910218021233809605816680735466029619_u128,167751331892096623340770167337095085240_u128,241444021423470704439402596763418078095_u128];
_3 = _7;
_1.1 = 52_i8;
_2 = _6;
_10.1 = _4 as i64;
match RET.1 {
3472587914743879128 => bb6,
_ => bb5
}
}
bb17 = {
Return()
}
bb18 = {
RET.0 = _4 as isize;
_1.0 = _7;
_10 = (_5, RET.1, RET.2);
_12 = [17980014345912826656_u64,18093766484613142362_u64,18321387209058033959_u64,10488566238972561479_u64,8695230098775393936_u64,10551633665205894708_u64,4310294402558867859_u64];
_4 = 10292671161179977197_usize >> RET.0;
_1.0 = _7;
RET.2 = _10.2;
_11 = [12275675849162363616_u64];
_1.2 = _10.2 as f32;
_16 = !_10.0;
RET = (_6, _10.1, _10.2);
_13 = [false,false,false,true,true,false,true];
RET.1 = _10.1 ^ _10.1;
_15 = [true,true,false,true,true,false,true];
RET.1 = _10.1;
_10.0 = _1.3 as isize;
RET.1 = _10.1;
_15 = _13;
_12 = [17182608447912976500_u64,17013827867632234770_u64,13182429294758873958_u64,17814420437012289660_u64,11176014023922608182_u64,12911006988877749141_u64,1129112967354027513_u64];
_9 = [5770266991068362265_u64,4401823920464994688_u64,13033598793892507521_u64,11561602727962547644_u64,8442384307947035734_u64,12339415950704983410_u64];
_17 = _2 != _16;
_14.0 = 148557409078693108438133852748636283704_u128 * 259655125904592231750272780873674602405_u128;
RET.2 = _10.2;
_5 = _16;
match _1.1 {
0 => bb9,
1 => bb10,
340282366920938463463374607431768211395 => bb12,
_ => bb11
}
}
bb19 = {
_3 = [277688344436919570418634396656944194429_u128,97568780357937892721016882531373231405_u128,250847856065317558281139206827663025197_u128,171097580074088743176034105795320257854_u128,74937229527903161650808703585493891223_u128,19927938062385391067581919298559716975_u128];
_1.3 = !61_u8;
Goto(bb4)
}
bb20 = {
_15 = [_17,_17,_17,_17,_17,_17,_17];
_10 = RET;
_20 = (*_18).2 * _1.2;
_17 = !true;
_10.0 = _14.1 as isize;
(*_18) = (_7, (-106_i8), _20, 32_u8);
(*_18).1 = _14.0 as i8;
Goto(bb21)
}
bb21 = {
Call(_26 = dump_var(4_usize, 5_usize, Move(_5), 15_usize, Move(_15), 16_usize, Move(_16), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_26 = dump_var(4_usize, 4_usize, Move(_4), 11_usize, Move(_11), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u8; 4],mut _2: [u128; 6],mut _3: (isize, i64, i32),mut _4: [u128; 6],mut _5: [u64; 7],mut _6: isize) -> (u128, i128) {
mir! {
type RET = (u128, i128);
let _7: i8;
let _8: f64;
let _9: [i32; 7];
let _10: isize;
let _11: i16;
let _12: char;
let _13: [u64; 7];
let _14: (u128, i128);
let _15: Adt49;
let _16: isize;
let _17: f32;
let _18: [u8; 4];
let _19: Adt49;
let _20: Adt51;
let _21: [u8; 4];
let _22: (u128, i128);
let _23: [u128; 6];
let _24: isize;
let _25: bool;
let _26: ();
let _27: ();
{
RET.1 = 2055119653_u32 as i128;
_3.0 = _6 * _6;
RET.0 = !259076255697936551431523124766528831618_u128;
_4 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Goto(bb1)
}
bb1 = {
_3.2 = 28_i8 as i32;
_7 = 11_i8 ^ (-21_i8);
RET = (208270358459281472483795021384469256451_u128, 26279143743522496413195572947920126719_i128);
RET.0 = 29965634655244678645301934372673138699_u128 | 113990060513092317887186361671771978477_u128;
_1 = [175_u8,16_u8,136_u8,89_u8];
_3 = (_6, 7231886142236691865_i64, 478776249_i32);
_6 = _3.0 | _3.0;
_3.0 = _3.1 as isize;
RET.0 = 20144_u16 as u128;
_7 = 1570416971_u32 as i8;
RET.0 = 98476784687450618260298370706789594092_u128 >> _3.0;
RET.1 = (-9627_i16) as i128;
_10 = !_3.0;
RET = (144322502275687846100377877338229221185_u128, 11064696450755904595373858786004691022_i128);
_2 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET = (100917639549565709208453727083291841276_u128, 67967888041071979562210918152998478129_i128);
_3.0 = RET.1 as isize;
RET.1 = 73283232025169579944588761503241889353_i128 >> _6;
_1 = [254_u8,57_u8,176_u8,82_u8];
_5 = [12392737101807097159_u64,17858032609095692106_u64,2927013688359827201_u64,15069151384961717225_u64,3593893295622270354_u64,17602547645366581073_u64,7868604720701824263_u64];
Goto(bb2)
}
bb2 = {
RET.1 = 10619962897405406927_usize as i128;
RET.1 = 7546689668257931952691869449837180439_i128;
_3.2 = 626563887_i32 - 1990395620_i32;
RET.0 = 37850726361637927418648027107866696946_u128 >> _10;
_3 = (_6, (-1361673762071307924_i64), 682671051_i32);
_11 = 29512_i16;
_13 = [10422045679158465754_u64,10434730937719845514_u64,2585824133429721579_u64,17858367982477625604_u64,18407188202113406637_u64,8396950270338718911_u64,13992706116434272804_u64];
_8 = 255_u8 as f64;
_13 = _5;
_10 = _3.0;
_4 = _2;
match _3.2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
682671051 => bb9,
_ => bb8
}
}
bb3 = {
_3.2 = 28_i8 as i32;
_7 = 11_i8 ^ (-21_i8);
RET = (208270358459281472483795021384469256451_u128, 26279143743522496413195572947920126719_i128);
RET.0 = 29965634655244678645301934372673138699_u128 | 113990060513092317887186361671771978477_u128;
_1 = [175_u8,16_u8,136_u8,89_u8];
_3 = (_6, 7231886142236691865_i64, 478776249_i32);
_6 = _3.0 | _3.0;
_3.0 = _3.1 as isize;
RET.0 = 20144_u16 as u128;
_7 = 1570416971_u32 as i8;
RET.0 = 98476784687450618260298370706789594092_u128 >> _3.0;
RET.1 = (-9627_i16) as i128;
_10 = !_3.0;
RET = (144322502275687846100377877338229221185_u128, 11064696450755904595373858786004691022_i128);
_2 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET = (100917639549565709208453727083291841276_u128, 67967888041071979562210918152998478129_i128);
_3.0 = RET.1 as isize;
RET.1 = 73283232025169579944588761503241889353_i128 >> _6;
_1 = [254_u8,57_u8,176_u8,82_u8];
_5 = [12392737101807097159_u64,17858032609095692106_u64,2927013688359827201_u64,15069151384961717225_u64,3593893295622270354_u64,17602547645366581073_u64,7868604720701824263_u64];
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
RET.0 = !83524044287155677595047469287759289487_u128;
_17 = RET.1 as f32;
_9 = [_3.2,_3.2,_3.2,_3.2,_3.2,_3.2,_3.2];
_3.0 = _6;
_3 = (_10, 8907018971953833082_i64, 2132031013_i32);
_8 = _17 as f64;
_14.1 = _3.2 as i128;
_1 = [15_u8,178_u8,129_u8,91_u8];
_11 = 24829_i16;
_3.2 = _6 as i32;
_5 = _13;
_3.0 = 1140000006_u32 as isize;
Goto(bb10)
}
bb10 = {
RET = (188716074144626093095735162722547098275_u128, _14.1);
RET.1 = -_14.1;
_9 = [_3.2,_3.2,_3.2,_3.2,_3.2,_3.2,_3.2];
_3 = (_6, 4795357783600230983_i64, 1341274763_i32);
_5 = _13;
_11 = _3.1 as i16;
_4 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_10 = _3.0 & _6;
_3 = (_10, (-7914490703759720467_i64), 1494808303_i32);
_3 = (_10, (-3568714475381187751_i64), 1755673445_i32);
RET.0 = 327355217770707499375892946421959504169_u128 - 165706129701548245156737077401278436317_u128;
_4 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17 = RET.0 as f32;
_4 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_1 = [25_u8,181_u8,55_u8,76_u8];
_12 = '\u{84899}';
RET.1 = _14.1;
_14.0 = RET.0 - RET.0;
_14.0 = RET.0;
_14.0 = _3.1 as u128;
RET = (_14.0, _14.1);
_16 = -_3.0;
RET = (_14.0, _14.1);
_3.0 = _7 as isize;
_3.2 = (-705918692_i32) ^ 2024708662_i32;
_14.0 = !RET.0;
match _3.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb7,
340282366920938463459805892956387023705 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_14 = (RET.0, RET.1);
_16 = !_10;
RET.0 = !_14.0;
_3 = (_10, (-5166008106926019257_i64), 1778979171_i32);
_4 = [RET.0,_14.0,RET.0,RET.0,RET.0,RET.0];
_3.0 = -_16;
_17 = _14.1 as f32;
_18 = [1_u8,218_u8,168_u8,220_u8];
RET.0 = _3.1 as u128;
RET = (_14.0, _14.1);
_22.0 = !RET.0;
_10 = _16 >> _14.0;
_14 = (RET.0, RET.1);
_4 = _2;
Call(_19 = fn6(RET.1, RET, _3, RET, _16, _3.1, _16, _3, RET), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_22.1 = _8 as i128;
_14.0 = _14.1 as u128;
_21 = [65_u8,91_u8,90_u8,192_u8];
_12 = '\u{7e26}';
_24 = _10 * _10;
_22.1 = _14.1;
_15 = _19;
_3.0 = _24;
_22.1 = 1692124012953349148_u64 as i128;
_21 = [91_u8,26_u8,141_u8,22_u8];
_3.0 = _3.2 as isize;
_17 = 11451103407113812169_usize as f32;
_13 = [13572441123149999796_u64,16059424233344455863_u64,15592466243124997025_u64,17067533224522350524_u64,10286682166346205094_u64,13588934234380869269_u64,2806669555399153516_u64];
SetDiscriminant(_15, 0);
_23 = [RET.0,RET.0,RET.0,_14.0,RET.0,RET.0];
place!(Field::<i32>(Variant(_15, 0), 1)) = _14.1 as i32;
_6 = 13331996955706381086_u64 as isize;
_25 = false;
place!(Field::<[u32; 6]>(Variant(_19, 0), 0)) = [1280114778_u32,2344181728_u32,1091780159_u32,2572027608_u32,3061197490_u32,10618717_u32];
RET.1 = _16 as i128;
_16 = _24;
_22.0 = 17661566919667830918_usize as u128;
_22.1 = RET.1 << _10;
Goto(bb14)
}
bb14 = {
_14.1 = _22.1;
_3 = (_16, (-6421839302078155595_i64), Field::<i32>(Variant(_15, 0), 1));
_15 = _19;
_22.1 = _14.1 | _14.1;
_9 = [_3.2,_3.2,Field::<i32>(Variant(_15, 0), 1),_3.2,Field::<i32>(Variant(_15, 0), 1),Field::<i32>(Variant(_19, 0), 1),Field::<i32>(Variant(_19, 0), 1)];
place!(Field::<i32>(Variant(_19, 0), 1)) = !_3.2;
place!(Field::<[u32; 6]>(Variant(_19, 0), 0)) = Field::<[u32; 6]>(Variant(_15, 0), 0);
_14 = (RET.0, _22.1);
_9 = [Field::<i32>(Variant(_15, 0), 1),Field::<i32>(Variant(_19, 0), 1),Field::<i32>(Variant(_15, 0), 1),_3.2,Field::<i32>(Variant(_19, 0), 1),_3.2,Field::<i32>(Variant(_15, 0), 1)];
_9 = [Field::<i32>(Variant(_15, 0), 1),Field::<i32>(Variant(_19, 0), 1),Field::<i32>(Variant(_15, 0), 1),Field::<i32>(Variant(_15, 0), 1),Field::<i32>(Variant(_15, 0), 1),Field::<i32>(Variant(_19, 0), 1),_3.2];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(5_usize, 6_usize, Move(_6), 11_usize, Move(_11), 25_usize, Move(_25), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(5_usize, 14_usize, Move(_14), 24_usize, Move(_24), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(5_usize, 22_usize, Move(_22), 9_usize, Move(_9), 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i128,mut _2: (u128, i128),mut _3: (isize, i64, i32),mut _4: (u128, i128),mut _5: isize,mut _6: i64,mut _7: isize,mut _8: (isize, i64, i32),mut _9: (u128, i128)) -> Adt49 {
mir! {
type RET = Adt49;
let _10: [u32; 6];
let _11: isize;
let _12: [u32; 6];
let _13: ();
let _14: ();
{
_8.0 = _5 << _1;
_3.1 = 2955076920971584730_u64 as i64;
_8.1 = _6;
_9.0 = _4.1 as u128;
_1 = _2.1;
_6 = !_8.1;
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1778979171 => bb7,
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
_5 = -_8.0;
_4 = (_2.0, _1);
_9 = (_2.0, _2.1);
_4.1 = _9.1 * _9.1;
_10 = [2998085601_u32,2308710260_u32,4164167547_u32,2171756986_u32,324383088_u32,693785237_u32];
_10 = [2171422849_u32,3049060140_u32,3982809355_u32,536558900_u32,533068973_u32,1183871178_u32];
_8.1 = -_6;
RET = Adt49::Variant0 { fld0: _10,fld1: _8.2 };
_1 = _9.1;
Goto(bb8)
}
bb8 = {
Call(_13 = dump_var(6_usize, 4_usize, Move(_4), 9_usize, Move(_9), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_13 = dump_var(6_usize, 10_usize, Move(_10), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i8,mut _2: i32,mut _3: [u128; 6],mut _4: usize,mut _5: usize,mut _6: ([u128; 6], i8, f32, u8),mut _7: ([u128; 6], i8, f32, u8),mut _8: (isize, i64, i32),mut _9: usize,mut _10: usize,mut _11: isize,mut _12: [i32; 7]) -> [u128; 6] {
mir! {
type RET = [u128; 6];
let _13: f32;
let _14: (isize, i64, i32);
let _15: f64;
let _16: [u8; 4];
let _17: Adt44;
let _18: [u128; 6];
let _19: Adt48;
let _20: Adt42;
let _21: [bool; 7];
let _22: u8;
let _23: isize;
let _24: Adt40;
let _25: (isize, i64, i32);
let _26: isize;
let _27: f32;
let _28: (isize, i64, i32);
let _29: [i32; 7];
let _30: (isize, i64, i32);
let _31: Adt52;
let _32: i16;
let _33: Adt44;
let _34: char;
let _35: usize;
let _36: isize;
let _37: ();
let _38: ();
{
_6 = (_7.0, _7.1, _7.2, _7.3);
_6.0 = [127339589401136080564116625678447756905_u128,99561456052610110085785427337076367170_u128,271772758383815441945584218230199390888_u128,154471895158504190529915410821190655331_u128,319655468195200242805286332666882621919_u128,284385662729906446118309515946589880038_u128];
_2 = !_8.2;
_7.1 = -_1;
RET = _6.0;
_6.1 = _1;
_11 = !_8.0;
_12 = [_2,_8.2,_8.2,_8.2,_2,_2,_8.2];
_7.0 = [334103742818233638923244077237151690348_u128,39682671631431274207425238915737644352_u128,158702024549871830605071738431657142553_u128,273974692107523665138888984185331588921_u128,212035889443995367069916214144813403768_u128,93617355097363107271646214941582026961_u128];
_8.0 = _4 as isize;
_7.3 = 5836721356033053992_u64 as u8;
_6.1 = _7.1;
RET = _7.0;
_8 = (_11, 6580435319781720355_i64, _2);
_10 = !_9;
_7.0 = [207725169416658552095879027410025841886_u128,317947267783951310174547500770691984870_u128,205478889316988488325762682475359117727_u128,56650437515638281838448512695786660742_u128,199756910421684747515887921455967588745_u128,252258400082855017202422675919013683818_u128];
_1 = false as i8;
_9 = _10 << _1;
_8.1 = !(-7363732410072021335_i64);
_14.0 = _11;
RET = [191787811113948167070171582053301442240_u128,310455383800372654986064770057159078346_u128,190141831045555721924615955299383643765_u128,335458681839337673719329336381873599847_u128,313786038972488518623125253478798026117_u128,325526484815255466326265274203653560659_u128];
_6.1 = true as i8;
_13 = _6.3 as f32;
_10 = !_9;
_11 = _14.0 + _8.0;
_8.1 = 694266728431876681_i64;
Call(_10 = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.1 = !_1;
_1 = _6.1 * _7.1;
_6 = (_3, _1, _7.2, _7.3);
_15 = _1 as f64;
_9 = _10 | _10;
_6.3 = !_7.3;
_16 = [_7.3,_7.3,_6.3,_6.3];
_6.3 = !_7.3;
_14.2 = _7.3 as i32;
_6.3 = _7.3;
_9 = _4 * _10;
_12 = [_2,_2,_2,_2,_2,_8.2,_8.2];
_7.2 = _6.2;
_7.1 = _1 >> _2;
_5 = _9;
_6 = (_7.0, _7.1, _7.2, _7.3);
_8.0 = _11;
_13 = -_6.2;
_8.0 = _14.0 * _11;
_17 = Adt44 { fld0: 10288538594229709018054642870299803677_u128 };
_11 = _8.0 ^ _8.0;
_17.fld0 = 160372650060837901869915595890395782090_u128 >> _11;
_7.3 = _6.3;
_2 = _11 as i32;
_12 = [_8.2,_8.2,_2,_2,_8.2,_14.2,_8.2];
_16 = [_7.3,_7.3,_6.3,_6.3];
_6.0 = _3;
_6 = (_7.0, _7.1, _13, _7.3);
match _8.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
694266728431876681 => bb7,
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
_7.1 = _6.1;
_6.1 = !_1;
_8.1 = -(-6659401702784376136_i64);
_6.1 = _7.1 | _1;
_14 = _8;
_10 = _4;
Goto(bb8)
}
bb8 = {
_7.1 = _6.1 & _6.1;
_18 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_8.1 = -_14.1;
_6 = (RET, _7.1, _13, _7.3);
_18 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_8 = _14;
_12 = [_8.2,_8.2,_8.2,_2,_8.2,_8.2,_8.2];
_1 = _6.1 | _7.1;
_16 = [_7.3,_7.3,_6.3,_7.3];
_6.3 = _7.3;
_10 = _17.fld0 as usize;
_6 = _7;
_12 = [_8.2,_2,_14.2,_2,_2,_2,_8.2];
_5 = _10 - _9;
_2 = -_8.2;
Goto(bb9)
}
bb9 = {
_16 = [_6.3,_7.3,_6.3,_6.3];
_7.1 = !_1;
_9 = _10;
_14.2 = _2;
_6.1 = -_1;
_12 = [_14.2,_2,_8.2,_14.2,_8.2,_8.2,_2];
_6.3 = _7.3;
_16 = [_7.3,_7.3,_6.3,_7.3];
_17 = Adt44 { fld0: 309718882067240031235497806830150548012_u128 };
_9 = _10 + _5;
_14.0 = _11;
_12 = [_14.2,_14.2,_2,_8.2,_2,_8.2,_8.2];
_12 = [_2,_8.2,_2,_14.2,_8.2,_2,_2];
_5 = _8.1 as usize;
_4 = !_9;
_1 = _6.1;
RET = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_8.0 = !_11;
RET = _18;
_11 = 1683247038_u32 as isize;
_12 = [_8.2,_14.2,_8.2,_14.2,_2,_8.2,_8.2];
match _17.fld0 {
0 => bb7,
1 => bb6,
2 => bb5,
309718882067240031235497806830150548012 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_17 = Adt44 { fld0: 310158317455488937129443399843505925587_u128 };
_8.1 = -_14.1;
_6 = _7;
_21 = [true,true,false,true,false,false,true];
_14.2 = _17.fld0 as i32;
_15 = _2 as f64;
_8 = (_11, _14.1, _14.2);
RET = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_11 = _8.0;
_18 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_8.1 = _14.1 + _14.1;
_9 = _5 & _4;
_1 = _7.1 * _6.1;
_3 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_17.fld0 = !175444281366124372278879570364326761141_u128;
_18 = _7.0;
_22 = _7.3;
_10 = _9 ^ _9;
_15 = 4238779487_u32 as f64;
_16 = [_22,_6.3,_6.3,_6.3];
_7.0 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_16 = [_22,_22,_7.3,_7.3];
Call(_14.2 = fn8(_6.1, _8.0, _9, _9), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6.1 = -_1;
_17.fld0 = 308250410799925771141938879285682806646_u128;
_25.0 = _14.0 >> _14.0;
_6.0 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_18 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_3 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_15 = _14.2 as f64;
_7.3 = _6.3 - _22;
_23 = _25.0 - _11;
_17.fld0 = 229835819838540844917411969622980128831_u128 & 184727775679492037721075883277848477181_u128;
_7.0 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_10 = _4 >> _1;
_18 = _3;
_6.3 = _22 + _22;
_14.2 = -_2;
_14.2 = _2;
_22 = _6.3 - _6.3;
_21 = [true,false,false,true,false,true,false];
_11 = -_25.0;
_17.fld0 = 22502_u16 as u128;
_9 = _4;
_25.0 = !_23;
_26 = _25.0;
Goto(bb13)
}
bb13 = {
_11 = _14.0;
_6.1 = _1;
_14.0 = _25.0;
_6.2 = -_7.2;
_12 = [_2,_14.2,_2,_14.2,_2,_2,_14.2];
_7.3 = !_22;
_30.0 = _15 as isize;
_9 = !_10;
_25.1 = (-85440610467608874112783852802116684972_i128) as i64;
_28.0 = _14.0 << _9;
_6 = (_3, _7.1, _7.2, _7.3);
_2 = -_14.2;
_30 = _14;
_3 = _18;
_1 = _7.1 | _7.1;
_25.2 = -_2;
_23 = _13 as isize;
_28 = _25;
_16 = [_7.3,_7.3,_7.3,_7.3];
_28.1 = -_14.1;
_16 = [_7.3,_7.3,_6.3,_22];
_15 = 20141997168160386238381751647676910522_i128 as f64;
_11 = !_25.0;
_11 = _1 as isize;
Goto(bb14)
}
bb14 = {
_7.1 = _14.2 as i8;
_25.0 = !_11;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 28_usize, Move(_28), 9_usize, Move(_9), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 21_usize, Move(_21), 25_usize, Move(_25), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(7_usize, 18_usize, Move(_18), 1_usize, Move(_1), 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i8,mut _2: isize,mut _3: usize,mut _4: usize) -> i32 {
mir! {
type RET = i32;
let _5: f32;
let _6: char;
let _7: (u128, i128);
let _8: f64;
let _9: Adt44;
let _10: u128;
let _11: isize;
let _12: [u8; 4];
let _13: f64;
let _14: i32;
let _15: *mut ([u128; 6], i8, f32, u8);
let _16: (isize, i64, i32);
let _17: [u64; 1];
let _18: [bool; 7];
let _19: [u8; 4];
let _20: bool;
let _21: f64;
let _22: isize;
let _23: f64;
let _24: isize;
let _25: u16;
let _26: i32;
let _27: [usize; 5];
let _28: f64;
let _29: f64;
let _30: f32;
let _31: bool;
let _32: [u64; 6];
let _33: [u64; 7];
let _34: bool;
let _35: isize;
let _36: [u64; 6];
let _37: Adt49;
let _38: f64;
let _39: u128;
let _40: isize;
let _41: ();
let _42: ();
{
_1 = 296560221928922399367354953226557702184_u128 as i8;
RET = (-7478_i16) as i32;
Call(_3 = fn9(_1, _4, RET, _4, _4, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = !_3;
_2 = (-9223372036854775808_isize);
_3 = _4 | _4;
_4 = _3 - _3;
RET = 2058_i16 as i32;
_6 = '\u{43494}';
_5 = 45982_u16 as f32;
RET = 43587_u16 as i32;
RET = (-2083552807_i32) & (-157744462_i32);
_1 = _2 as i8;
_7 = (258075282918429480021801374098742631464_u128, (-162878591702739275648779719835169821298_i128));
_3 = _4;
RET = -(-165709374_i32);
_7.1 = 99457881643051570272631880777224393814_i128 * 121621844168064146633296452521990370232_i128;
_7.0 = 19989241776006109130816751434705065916_u128 & 219345450589128970738317234420632200245_u128;
_7.0 = !258645100092127120013430281742193801843_u128;
_5 = 1526547112_u32 as f32;
_5 = _4 as f32;
_7 = (23260065241612852708604612541478406143_u128, (-123961855794484446768008693564285764335_i128));
_7.1 = RET as i128;
_7.1 = 143854076787378698364933194818399155524_i128;
_3 = _4 ^ _4;
_1 = 15_i8 - 74_i8;
RET = 923435509_i32 * (-1736727069_i32);
_7 = (85174453386611709660717140401476615329_u128, (-141673437852218707709098804044848760719_i128));
_8 = _2 as f64;
_3 = 1597888118_u32 as usize;
Goto(bb2)
}
bb2 = {
_8 = _1 as f64;
_4 = true as usize;
RET = (-1375382365_i32) ^ 1821282798_i32;
_5 = _1 as f32;
_3 = _4 + _4;
RET = 1970185246_i32 << _7.1;
_2 = _7.0 as isize;
_6 = '\u{2416f}';
_9.fld0 = _7.0 << _2;
_4 = _5 as usize;
_8 = RET as f64;
_8 = _7.0 as f64;
RET = _3 as i32;
_9.fld0 = _7.0;
_9.fld0 = 189_u8 as u128;
Goto(bb3)
}
bb3 = {
RET = (-1653664644_i32);
_9.fld0 = !_7.0;
_7.1 = (-82427078439328303465190214944204078943_i128) | 164213602725528794263343972398930718003_i128;
_7.1 = (-30198593522512704482759712284192300401_i128);
_10 = !_7.0;
_4 = !_3;
_5 = 28520_i16 as f32;
RET = (-212588007_i32) >> _2;
_9.fld0 = _7.0 - _10;
_2 = -102_isize;
_2 = 3027369665_u32 as isize;
_10 = _9.fld0 << _4;
_3 = _4;
_12 = [129_u8,31_u8,87_u8,129_u8];
_5 = 6304_i16 as f32;
_7 = (_9.fld0, 19184251930610633236067788402866416538_i128);
_10 = _9.fld0;
_9 = Adt44 { fld0: _10 };
_11 = _2 - _2;
RET = !1771736284_i32;
match _7.1 {
0 => bb2,
19184251930610633236067788402866416538 => bb5,
_ => bb4
}
}
bb4 = {
_4 = !_3;
_2 = (-9223372036854775808_isize);
_3 = _4 | _4;
_4 = _3 - _3;
RET = 2058_i16 as i32;
_6 = '\u{43494}';
_5 = 45982_u16 as f32;
RET = 43587_u16 as i32;
RET = (-2083552807_i32) & (-157744462_i32);
_1 = _2 as i8;
_7 = (258075282918429480021801374098742631464_u128, (-162878591702739275648779719835169821298_i128));
_3 = _4;
RET = -(-165709374_i32);
_7.1 = 99457881643051570272631880777224393814_i128 * 121621844168064146633296452521990370232_i128;
_7.0 = 19989241776006109130816751434705065916_u128 & 219345450589128970738317234420632200245_u128;
_7.0 = !258645100092127120013430281742193801843_u128;
_5 = 1526547112_u32 as f32;
_5 = _4 as f32;
_7 = (23260065241612852708604612541478406143_u128, (-123961855794484446768008693564285764335_i128));
_7.1 = RET as i128;
_7.1 = 143854076787378698364933194818399155524_i128;
_3 = _4 ^ _4;
_1 = 15_i8 - 74_i8;
RET = 923435509_i32 * (-1736727069_i32);
_7 = (85174453386611709660717140401476615329_u128, (-141673437852218707709098804044848760719_i128));
_8 = _2 as f64;
_3 = 1597888118_u32 as usize;
Goto(bb2)
}
bb5 = {
RET = 397623919_i32 * (-1862214543_i32);
_1 = -40_i8;
_13 = _8;
_4 = _3 >> _7.1;
_10 = _9.fld0;
_7.1 = !(-3370457137645405799683607749765239861_i128);
_13 = -_8;
_13 = 14436395975749920990_u64 as f64;
_10 = 30894_u16 as u128;
RET = (-770133954_i32) ^ 999339315_i32;
Call(_15 = fn10(_7.0, _7.0, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_9 = Adt44 { fld0: _7.0 };
_16.2 = 9060266386417625542_u64 as i32;
_1 = (-10_i8);
_11 = _2 + _2;
_16.2 = -RET;
_16.0 = _9.fld0 as isize;
_9.fld0 = !_7.0;
_6 = '\u{4f477}';
RET = !_16.2;
_5 = _4 as f32;
_18 = [false,true,true,false,true,false,false];
_5 = _4 as f32;
_1 = (-104_i8);
_12 = [91_u8,173_u8,71_u8,61_u8];
_10 = _7.0;
_10 = _9.fld0;
_19 = [4_u8,144_u8,32_u8,243_u8];
_17 = [14666583531100256893_u64];
_2 = false as isize;
_11 = -_16.0;
_14 = 31_u8 as i32;
Goto(bb7)
}
bb7 = {
_7.0 = !_9.fld0;
_3 = _4;
_8 = _4 as f64;
_5 = 24634_u16 as f32;
_12 = [11_u8,107_u8,241_u8,253_u8];
_22 = 56534_u16 as isize;
RET = _5 as i32;
_5 = 56232_u16 as f32;
_12 = _19;
_6 = '\u{e8114}';
_3 = _4 >> _16.0;
_8 = _13 + _13;
Call(_12 = fn18(_15, _15, _15), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_21 = _13 + _8;
_1 = 39_i8 - 58_i8;
RET = _16.2;
_9.fld0 = !_10;
_10 = _5 as u128;
_12 = [121_u8,119_u8,11_u8,141_u8];
_19 = _12;
_9 = Adt44 { fld0: _7.0 };
_22 = _11 | _2;
_21 = -_13;
_24 = _11 | _22;
_10 = _7.0 >> _16.2;
_23 = _3 as f64;
_7.0 = _9.fld0 & _9.fld0;
_13 = _7.1 as f64;
_9.fld0 = _10;
Call(_26 = core::intrinsics::transmute(_19), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = [130_u8,242_u8,108_u8,31_u8];
_29 = _23 + _8;
_30 = _7.1 as f32;
_7.0 = !_10;
_10 = 9646965761048273516_u64 as u128;
_16.1 = !6770549167201357600_i64;
_18 = [true,true,true,false,true,false,true];
_27 = [_4,_3,_4,_3,_4];
_31 = !false;
_10 = _7.0 + _9.fld0;
_7.0 = 2688216407_u32 as u128;
_12 = _19;
_5 = _30 - _30;
_1 = _31 as i8;
_16.1 = !2022945965689323200_i64;
RET = _16.2 + _14;
_19 = [233_u8,162_u8,112_u8,63_u8];
_4 = 13203046216760006858_u64 as usize;
_23 = -_8;
_33 = [12402335926340721478_u64,9147173465147925796_u64,5963838203920919108_u64,3915069422934200676_u64,15270513904056644337_u64,6589719169679835324_u64,5073094813395954080_u64];
Goto(bb10)
}
bb10 = {
_16.0 = _11;
RET = _26 + _14;
_10 = !_9.fld0;
RET = _16.2 | _26;
_1 = !90_i8;
_25 = 60692_u16;
_7 = (_10, (-17836343928962573390578354839086276282_i128));
_26 = _5 as i32;
_12 = [146_u8,145_u8,153_u8,254_u8];
_10 = _16.2 as u128;
_34 = _16.0 < _22;
_12 = _19;
_18 = [_34,_34,_34,_34,_34,_34,_34];
_2 = !_11;
_16.0 = _24 - _24;
_9.fld0 = !_7.0;
_9.fld0 = _7.0;
_9 = Adt44 { fld0: _7.0 };
_32 = [5726582898223741769_u64,15480767103622055436_u64,2003104630207689392_u64,2389187576385468979_u64,5781359025239360781_u64,2331375557348090491_u64];
_7.1 = !93401712145085990639865846410927559570_i128;
_13 = _23;
_31 = _3 <= _3;
_14 = 18342664882542531550_u64 as i32;
_20 = _31;
match _25 {
60692 => bb11,
_ => bb9
}
}
bb11 = {
_36 = [508009641569607628_u64,5301175043041625437_u64,11652975531471622639_u64,18179442347426383333_u64,1576355383060673915_u64,18084842141860964024_u64];
_35 = _22 | _16.0;
_1 = 45_i8;
_5 = -_30;
_20 = _31;
_18 = [_31,_31,_34,_34,_31,_34,_20];
RET = _26;
match _25 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb7,
4 => bb9,
5 => bb6,
60692 => bb13,
_ => bb12
}
}
bb12 = {
RET = 397623919_i32 * (-1862214543_i32);
_1 = -40_i8;
_13 = _8;
_4 = _3 >> _7.1;
_10 = _9.fld0;
_7.1 = !(-3370457137645405799683607749765239861_i128);
_13 = -_8;
_13 = 14436395975749920990_u64 as f64;
_10 = 30894_u16 as u128;
RET = (-770133954_i32) ^ 999339315_i32;
Call(_15 = fn10(_7.0, _7.0, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb13 = {
_13 = _29 + _29;
_30 = _5;
_30 = _1 as f32;
_35 = !_24;
_16 = (_35, (-2281670942045944798_i64), RET);
_20 = _31;
_36 = _32;
_25 = !64922_u16;
_16.2 = !RET;
RET = _7.0 as i32;
_9 = Adt44 { fld0: _7.0 };
_16.0 = _11 | _2;
_16.1 = (-1632308140431639922_i64) | 7365307778540506759_i64;
_9.fld0 = (-2454_i16) as u128;
_21 = _23;
_5 = _30;
_5 = _30 - _30;
Goto(bb14)
}
bb14 = {
RET = _25 as i32;
_27 = [_3,_3,_3,_3,_3];
_7.1 = (-105901989493640142044161273220200828786_i128);
RET = -_14;
_14 = _26 + _16.2;
_4 = !_3;
_34 = !_31;
RET = _16.2 + _26;
_14 = _31 as i32;
RET = 245_u8 as i32;
_16.2 = -_14;
_10 = _31 as u128;
_7.0 = _31 as u128;
_24 = _11 >> _10;
_35 = _23 as isize;
RET = !_26;
_29 = _23 * _21;
_21 = -_13;
RET = _14;
_27 = [_4,_3,_3,_4,_4];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(8_usize, 18_usize, Move(_18), 36_usize, Move(_36), 19_usize, Move(_19), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(8_usize, 11_usize, Move(_11), 12_usize, Move(_12), 22_usize, Move(_22), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(8_usize, 34_usize, Move(_34), 4_usize, Move(_4), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(8_usize, 20_usize, Move(_20), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i8,mut _2: usize,mut _3: i32,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize) -> usize {
mir! {
type RET = usize;
let _8: i128;
let _9: Adt45;
let _10: [u32; 6];
let _11: isize;
let _12: Adt41;
let _13: u32;
let _14: ();
let _15: ();
{
_8 = 129163822613280479716943478966392973220_i128;
_4 = _5 + _2;
RET = !_7;
_4 = 104_u8 as usize;
_3 = !(-1314689228_i32);
_5 = _6;
_2 = !_6;
_2 = _7 << RET;
RET = _6;
_13 = _5 as u32;
_10 = [_13,_13,_13,_13,_13,_13];
_13 = 2562332016_u32 << _5;
_3 = (-2010153085_i32) ^ (-698877780_i32);
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(9_usize, 2_usize, Move(_2), 10_usize, Move(_10), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(9_usize, 4_usize, Move(_4), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u128,mut _2: u128,mut _3: usize) -> *mut ([u128; 6], i8, f32, u8) {
mir! {
type RET = *mut ([u128; 6], i8, f32, u8);
let _4: u64;
let _5: [u64; 6];
let _6: *mut ([u128; 6], i8, f32, u8);
let _7: [usize; 5];
let _8: bool;
let _9: [u128; 6];
let _10: isize;
let _11: bool;
let _12: i16;
let _13: i32;
let _14: Adt55;
let _15: isize;
let _16: isize;
let _17: [u128; 6];
let _18: [u64; 1];
let _19: i32;
let _20: f64;
let _21: [u128; 6];
let _22: bool;
let _23: [usize; 5];
let _24: isize;
let _25: [u8; 4];
let _26: [u64; 6];
let _27: f32;
let _28: isize;
let _29: (bool,);
let _30: char;
let _31: *const bool;
let _32: [i32; 7];
let _33: [usize; 5];
let _34: [usize; 5];
let _35: isize;
let _36: Adt49;
let _37: bool;
let _38: Adt52;
let _39: bool;
let _40: usize;
let _41: isize;
let _42: i64;
let _43: Adt43;
let _44: isize;
let _45: [u128; 6];
let _46: Adt41;
let _47: ([u128; 6], i8, f32, u8);
let _48: Adt56;
let _49: Adt55;
let _50: [u8; 4];
let _51: bool;
let _52: isize;
let _53: ();
let _54: ();
{
_1 = _2 | _2;
_4 = 4118230040_u32 as u64;
_3 = 16702951592551211061_usize;
_1 = 3769947559_u32 as u128;
_5 = [_4,_4,_4,_4,_4,_4];
_3 = 4_usize * 14677681555838918956_usize;
_5 = [_4,_4,_4,_4,_4,_4];
_1 = _2 << _3;
_1 = _2 ^ _2;
_3 = '\u{1bafe}' as usize;
_4 = 15268583617650845444_u64;
_2 = !_1;
_1 = _2;
_5 = [_4,_4,_4,_4,_4,_4];
_8 = true;
_4 = (-11509_i16) as u64;
_1 = _2;
_7 = [_3,_3,_3,_3,_3];
_3 = 14772313095687248652_usize;
_2 = _1;
_5 = [_4,_4,_4,_4,_4,_4];
_5 = [_4,_4,_4,_4,_4,_4];
Call(_3 = fn11(_7, _2, _2, _2, _1, _8, _1, _8, _2, _1, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 13489784831274625758_usize ^ 8015911493212886384_usize;
_3 = 2774013122948320278_usize;
_11 = !_8;
_9 = [_2,_2,_2,_1,_1,_2];
_3 = 1071369599282650804_usize;
_9 = [_1,_2,_1,_1,_1,_1];
_2 = _1 + _1;
_7 = [_3,_3,_3,_3,_3];
_9 = [_1,_1,_1,_2,_2,_2];
_4 = _3 as u64;
_10 = 9223372036854775807_isize >> _2;
_11 = !_8;
_11 = !_8;
_7 = [_3,_3,_3,_3,_3];
_13 = (-163972409535537195328940416829985015642_i128) as i32;
Goto(bb2)
}
bb2 = {
_4 = !3842845265106765509_u64;
_1 = _2;
_12 = !29456_i16;
Goto(bb3)
}
bb3 = {
_1 = _4 as u128;
_15 = _10;
_17 = [_2,_2,_2,_2,_2,_2];
_12 = (-32444_i16);
_15 = _10;
_4 = 15472450580910065071_u64;
_9 = _17;
_13 = _3 as i32;
_13 = 539920888_i32;
_10 = _15 + _15;
_14.fld0 = _12 as f32;
_15 = _14.fld0 as isize;
_15 = _10;
match _13 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
539920888 => bb8,
_ => bb7
}
}
bb4 = {
_4 = !3842845265106765509_u64;
_1 = _2;
_12 = !29456_i16;
Goto(bb3)
}
bb5 = {
_3 = 13489784831274625758_usize ^ 8015911493212886384_usize;
_3 = 2774013122948320278_usize;
_11 = !_8;
_9 = [_2,_2,_2,_1,_1,_2];
_3 = 1071369599282650804_usize;
_9 = [_1,_2,_1,_1,_1,_1];
_2 = _1 + _1;
_7 = [_3,_3,_3,_3,_3];
_9 = [_1,_1,_1,_2,_2,_2];
_4 = _3 as u64;
_10 = 9223372036854775807_isize >> _2;
_11 = !_8;
_11 = !_8;
_7 = [_3,_3,_3,_3,_3];
_13 = (-163972409535537195328940416829985015642_i128) as i32;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb9 = {
_10 = _16 ^ _15;
Call(_1 = core::intrinsics::bswap(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = !2_usize;
_18 = [_4];
_5 = [_4,_4,_4,_4,_4,_4];
_3 = 11897995495086172107_usize << _10;
_10 = _15;
_4 = (-113_i8) as u64;
_13 = 1284111448_i32;
_15 = _16;
_3 = 2_u8 as usize;
_7 = [_3,_3,_3,_3,_3];
_17 = [_2,_2,_2,_1,_2,_1];
_18 = [_4];
_21 = [_1,_1,_1,_1,_2,_2];
_4 = 80_u8 as u64;
_1 = !_2;
_22 = _16 > _10;
_19 = _13;
_23 = _7;
_15 = -_10;
_14.fld0 = 2757993212_u32 as f32;
_19 = -_13;
_27 = 1377111586_u32 as f32;
_20 = _27 as f64;
match _12 {
0 => bb7,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb11,
340282366920938463463374607431768179012 => bb13,
_ => bb12
}
}
bb11 = {
_3 = 13489784831274625758_usize ^ 8015911493212886384_usize;
_3 = 2774013122948320278_usize;
_11 = !_8;
_9 = [_2,_2,_2,_1,_1,_2];
_3 = 1071369599282650804_usize;
_9 = [_1,_2,_1,_1,_1,_1];
_2 = _1 + _1;
_7 = [_3,_3,_3,_3,_3];
_9 = [_1,_1,_1,_2,_2,_2];
_4 = _3 as u64;
_10 = 9223372036854775807_isize >> _2;
_11 = !_8;
_11 = !_8;
_7 = [_3,_3,_3,_3,_3];
_13 = (-163972409535537195328940416829985015642_i128) as i32;
Goto(bb2)
}
bb12 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb13 = {
_14 = Adt55 { fld0: _27 };
_2 = _1;
_18 = [_4];
_19 = _12 as i32;
_23 = [_3,_3,_3,_3,_3];
_14 = Adt55 { fld0: _27 };
_4 = 965553551933210199_u64;
_9 = [_1,_1,_1,_1,_2,_2];
_27 = 2_u8 as f32;
_23 = [_3,_3,_3,_3,_3];
_7 = [_3,_3,_3,_3,_3];
_10 = 8310_u16 as isize;
_12 = 7471241492036805987_i64 as i16;
_26 = _5;
_3 = 12080544090992848346_usize;
_25 = [76_u8,95_u8,10_u8,67_u8];
_14 = Adt55 { fld0: _27 };
_5 = _26;
Goto(bb14)
}
bb14 = {
_24 = !_15;
_26 = [_4,_4,_4,_4,_4,_4];
_8 = !_22;
_3 = 4840597175200446065_usize ^ 4316308501259974084_usize;
_17 = _21;
_5 = [_4,_4,_4,_4,_4,_4];
_27 = _14.fld0;
_26 = _5;
_10 = _15 >> _16;
_20 = 211_u8 as f64;
_28 = -_24;
_29 = (_22,);
_7 = [_3,_3,_3,_3,_3];
_2 = 21296_u16 as u128;
_31 = core::ptr::addr_of!(_8);
_5 = _26;
_32 = [_19,_13,_13,_13,_19,_13,_19];
_23 = [_3,_3,_3,_3,_3];
Goto(bb15)
}
bb15 = {
_28 = _16;
_34 = _23;
_2 = _1;
_9 = _21;
match _4 {
0 => bb16,
965553551933210199 => bb18,
_ => bb17
}
}
bb16 = {
Return()
}
bb17 = {
_4 = !3842845265106765509_u64;
_1 = _2;
_12 = !29456_i16;
Goto(bb3)
}
bb18 = {
_12 = 6955_i16;
_14.fld0 = 75_i8 as f32;
_25 = [62_u8,79_u8,144_u8,196_u8];
_22 = (*_31);
_4 = !15596654526708134441_u64;
_26 = _5;
_14.fld0 = (-138061772286700939089878799194609217006_i128) as f32;
Goto(bb19)
}
bb19 = {
_18 = [_4];
_22 = !_8;
_10 = _20 as isize;
_27 = _14.fld0;
_32 = [_13,_13,_19,_13,_19,_13,_19];
_7 = [_3,_3,_3,_3,_3];
_5 = [_4,_4,_4,_4,_4,_4];
_33 = [_3,_3,_3,_3,_3];
_10 = _3 as isize;
_1 = _2 >> _16;
_25 = [253_u8,153_u8,71_u8,191_u8];
_11 = _8;
_4 = 2561103562066271840_u64;
_14 = Adt55 { fld0: _27 };
_11 = !(*_31);
_10 = _2 as isize;
_13 = !_19;
_3 = 214_u8 as usize;
_39 = _11 == _11;
match _12 {
0 => bb4,
6955 => bb21,
_ => bb20
}
}
bb20 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb21 = {
_41 = _8 as isize;
_12 = (-13611_i16) * 908_i16;
_19 = 246_u8 as i32;
_34 = [_3,_3,_3,_3,_3];
_28 = _41 - _41;
Goto(bb22)
}
bb22 = {
_44 = 1370949783_u32 as isize;
_37 = !_29.0;
_6 = core::ptr::addr_of_mut!(_47);
(*_6).2 = 11_u8 as f32;
_9 = [_1,_1,_2,_1,_1,_1];
_45 = _9;
_19 = 1982680948984284408_i64 as i32;
(*_6).0 = [_2,_1,_1,_2,_1,_1];
_21 = _47.0;
_44 = _16 & _16;
(*_6).3 = 191_u8;
(*_6).1 = (-117_i8) ^ (-99_i8);
_18 = [_4];
_49.fld0 = (*_6).2 - _14.fld0;
_45 = [_2,_2,_2,_1,_2,_1];
_4 = !17545818137776403575_u64;
(*_6).2 = -_27;
_30 = '\u{cd1df}';
match (*_6).3 {
0 => bb6,
1 => bb16,
2 => bb3,
3 => bb23,
4 => bb24,
5 => bb25,
6 => bb26,
191 => bb28,
_ => bb27
}
}
bb23 = {
_41 = _8 as isize;
_12 = (-13611_i16) * 908_i16;
_19 = 246_u8 as i32;
_34 = [_3,_3,_3,_3,_3];
_28 = _41 - _41;
Goto(bb22)
}
bb24 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb25 = {
_28 = _16;
_34 = _23;
_2 = _1;
_9 = _21;
match _4 {
0 => bb16,
965553551933210199 => bb18,
_ => bb17
}
}
bb26 = {
Return()
}
bb27 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb28 = {
_11 = !(*_31);
(*_31) = _41 > _24;
match (*_6).3 {
0 => bb22,
1 => bb27,
2 => bb3,
3 => bb12,
4 => bb5,
5 => bb26,
6 => bb29,
191 => bb31,
_ => bb30
}
}
bb29 = {
Return()
}
bb30 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb31 = {
_2 = _3 as u128;
_47.2 = _27;
(*_31) = _22;
_24 = _12 as isize;
_31 = core::ptr::addr_of!((*_31));
(*_6).3 = !136_u8;
Goto(bb32)
}
bb32 = {
_4 = 7170987376544429776_u64 | 9335068393262222135_u64;
(*_6).1 = 38_i8;
_34 = [_3,_3,_3,_3,_3];
_15 = -_28;
_39 = _28 <= _41;
_47.1 = 101_i8 * (-107_i8);
_21 = [_1,_1,_1,_1,_2,_1];
_5 = [_4,_4,_4,_4,_4,_4];
_45 = [_1,_1,_1,_1,_1,_1];
_29 = (_22,);
_3 = 11842844434928151134_usize;
(*_6).0 = _9;
_45 = (*_6).0;
(*_6).0 = [_2,_1,_1,_1,_1,_1];
_40 = _3 & _3;
_47 = (_21, (-23_i8), _49.fld0, 201_u8);
_1 = !_2;
Goto(bb33)
}
bb33 = {
_44 = _28 ^ _28;
_47.2 = _27;
_42 = _4 as i64;
_39 = (*_31) | (*_31);
_24 = _40 as isize;
_46 = Adt41::Variant0 { fld0: _42,fld1: _47,fld2: _9 };
(*_6).1 = Field::<([u128; 6], i8, f32, u8)>(Variant(_46, 0), 1).1 & Field::<([u128; 6], i8, f32, u8)>(Variant(_46, 0), 1).1;
_47.1 = Field::<([u128; 6], i8, f32, u8)>(Variant(_46, 0), 1).1 + Field::<([u128; 6], i8, f32, u8)>(Variant(_46, 0), 1).1;
Goto(bb34)
}
bb34 = {
_32 = [_19,_13,_13,_19,_19,_19,_13];
_4 = 12951930101988476225_u64 << (*_6).3;
_10 = !_15;
_7 = [_40,_40,_40,_40,_40];
_6 = core::ptr::addr_of_mut!(_47);
place!(Field::<([u128; 6], i8, f32, u8)>(Variant(_46, 0), 1)).0 = [_1,_2,_2,_1,_2,_2];
Goto(bb35)
}
bb35 = {
_2 = !_1;
SetDiscriminant(_46, 1);
_14.fld0 = _20 as f32;
_39 = _22 | _8;
(*_6).2 = -_49.fld0;
_29.0 = !_11;
_8 = !_22;
_41 = !_15;
_51 = _8 & _8;
_24 = -_44;
_39 = (*_6).1 == (*_6).1;
place!(Field::<*mut f32>(Variant(_46, 1), 1)) = core::ptr::addr_of_mut!(_27);
_50 = [(*_6).3,(*_6).3,(*_6).3,(*_6).3];
match _47.3 {
0 => bb36,
1 => bb37,
201 => bb39,
_ => bb38
}
}
bb36 = {
_16 = !_10;
_10 = _14.fld0 as isize;
_9 = [_2,_2,_2,_2,_2,_2];
_14.fld0 = _4 as f32;
_17 = [_2,_2,_2,_2,_2,_2];
_1 = _2;
Goto(bb9)
}
bb37 = {
_18 = [_4];
_22 = !_8;
_10 = _20 as isize;
_27 = _14.fld0;
_32 = [_13,_13,_19,_13,_19,_13,_19];
_7 = [_3,_3,_3,_3,_3];
_5 = [_4,_4,_4,_4,_4,_4];
_33 = [_3,_3,_3,_3,_3];
_10 = _3 as isize;
_1 = _2 >> _16;
_25 = [253_u8,153_u8,71_u8,191_u8];
_11 = _8;
_4 = 2561103562066271840_u64;
_14 = Adt55 { fld0: _27 };
_11 = !(*_31);
_10 = _2 as isize;
_13 = !_19;
_3 = 214_u8 as usize;
_39 = _11 == _11;
match _12 {
0 => bb4,
6955 => bb21,
_ => bb20
}
}
bb38 = {
_3 = 13489784831274625758_usize ^ 8015911493212886384_usize;
_3 = 2774013122948320278_usize;
_11 = !_8;
_9 = [_2,_2,_2,_1,_1,_2];
_3 = 1071369599282650804_usize;
_9 = [_1,_2,_1,_1,_1,_1];
_2 = _1 + _1;
_7 = [_3,_3,_3,_3,_3];
_9 = [_1,_1,_1,_2,_2,_2];
_4 = _3 as u64;
_10 = 9223372036854775807_isize >> _2;
_11 = !_8;
_11 = !_8;
_7 = [_3,_3,_3,_3,_3];
_13 = (-163972409535537195328940416829985015642_i128) as i32;
Goto(bb2)
}
bb39 = {
_27 = _49.fld0;
_30 = '\u{eecc4}';
_24 = !_15;
_39 = _51 < _37;
_28 = -_10;
RET = core::ptr::addr_of_mut!((*_6));
(*_6).2 = _49.fld0;
(*RET).3 = 51_u8 & 168_u8;
_23 = _34;
_33 = [_40,_40,_40,_3,_40];
_33 = [_40,_40,_40,_40,_40];
Goto(bb40)
}
bb40 = {
Call(_53 = dump_var(10_usize, 9_usize, Move(_9), 13_usize, Move(_13), 24_usize, Move(_24), 41_usize, Move(_41)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_53 = dump_var(10_usize, 15_usize, Move(_15), 50_usize, Move(_50), 11_usize, Move(_11), 42_usize, Move(_42)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_53 = dump_var(10_usize, 3_usize, Move(_3), 19_usize, Move(_19), 40_usize, Move(_40), 51_usize, Move(_51)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_53 = dump_var(10_usize, 29_usize, Move(_29), 21_usize, Move(_21), 34_usize, Move(_34), 7_usize, Move(_7)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_53 = dump_var(10_usize, 25_usize, Move(_25), 2_usize, Move(_2), 16_usize, Move(_16), 54_usize, _54), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [usize; 5],mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: bool,mut _7: u128,mut _8: bool,mut _9: u128,mut _10: u128,mut _11: u128,mut _12: [u64; 6]) -> usize {
mir! {
type RET = usize;
let _13: (bool,);
let _14: (u64, [u128; 6], *mut f32, u64, u16);
let _15: isize;
let _16: [u8; 4];
let _17: u8;
let _18: [u128; 6];
let _19: u64;
let _20: [u8; 4];
let _21: i128;
let _22: u8;
let _23: u128;
let _24: isize;
let _25: bool;
let _26: isize;
let _27: f32;
let _28: [u64; 6];
let _29: (u128, i128);
let _30: char;
let _31: f32;
let _32: u32;
let _33: f32;
let _34: u16;
let _35: char;
let _36: ();
let _37: ();
{
_8 = _11 >= _11;
_7 = !_11;
_12 = [4849322952914079108_u64,10814100862729051720_u64,18322320120325542290_u64,13340603763276817875_u64,8096726707670194244_u64,13888276839113516767_u64];
_9 = !_3;
_7 = _11;
RET = (-15462_i16) as usize;
RET = 4_usize * 5758344503270325440_usize;
Call(RET = fn12(_10, _12, _3, _10, _11, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _11 ^ _9;
_8 = _6 ^ _6;
_11 = _4 * _5;
_1 = [RET,RET,RET,RET,RET];
RET = 10849091009575475647_usize;
_8 = _6;
_5 = _7 | _10;
_11 = !_5;
_13.0 = _2 < _11;
Call(RET = core::intrinsics::bswap(1_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = [RET,RET,RET,RET,RET];
RET = (-29073_i16) as usize;
_6 = !_13.0;
_13.0 = _6;
_6 = _13.0;
_3 = _10 - _4;
_8 = _6;
_6 = _8;
_11 = _3 >> _2;
_7 = '\u{483d6}' as u128;
_7 = _11;
_2 = 3394685081335808298400569250208213607_i128 as u128;
_6 = _11 != _7;
_13.0 = _8;
_1 = [RET,RET,RET,RET,RET];
_14.3 = 5067890067167717445_u64;
_14.0 = _14.3;
_14.4 = 21711_u16 >> _5;
Goto(bb3)
}
bb3 = {
_8 = _13.0 == _6;
_10 = _3 - _9;
_16 = [206_u8,86_u8,123_u8,171_u8];
_8 = !_6;
_14.3 = !_14.0;
_6 = !_8;
RET = !7_usize;
_15 = 958208596_u32 as isize;
_14.1 = [_7,_10,_10,_4,_5,_11];
_12 = [_14.3,_14.3,_14.0,_14.0,_14.3,_14.0];
_16 = [46_u8,244_u8,250_u8,194_u8];
_14.0 = _14.3 & _14.3;
_14.3 = !_14.0;
Goto(bb4)
}
bb4 = {
_2 = _4 * _7;
_14.4 = 99_u8 as u16;
_5 = 239173067_u32 as u128;
_14.0 = !_14.3;
_9 = _15 as u128;
RET = 16750732977213967779_usize;
_17 = 2590485421_u32 as u8;
_1 = [RET,RET,RET,RET,RET];
_14.3 = _14.0;
_14.0 = 79_i8 as u64;
_12 = [_14.0,_14.0,_14.3,_14.3,_14.0,_14.3];
_14.1 = [_7,_7,_10,_11,_2,_10];
_9 = _7 & _7;
_5 = _13.0 as u128;
_14.1 = [_11,_10,_10,_5,_10,_7];
_5 = _9;
_18 = [_2,_5,_10,_2,_10,_7];
_17 = 123_u8;
_6 = _5 > _7;
_9 = _17 as u128;
_9 = !_5;
_19 = RET as u64;
_1 = [RET,RET,RET,RET,RET];
_9 = !_2;
Goto(bb5)
}
bb5 = {
_15 = (-9223372036854775808_isize);
match RET {
0 => bb6,
16750732977213967779 => bb8,
_ => bb7
}
}
bb6 = {
_1 = [RET,RET,RET,RET,RET];
RET = (-29073_i16) as usize;
_6 = !_13.0;
_13.0 = _6;
_6 = _13.0;
_3 = _10 - _4;
_8 = _6;
_6 = _8;
_11 = _3 >> _2;
_7 = '\u{483d6}' as u128;
_7 = _11;
_2 = 3394685081335808298400569250208213607_i128 as u128;
_6 = _11 != _7;
_13.0 = _8;
_1 = [RET,RET,RET,RET,RET];
_14.3 = 5067890067167717445_u64;
_14.0 = _14.3;
_14.4 = 21711_u16 >> _5;
Goto(bb3)
}
bb7 = {
_10 = _11 ^ _9;
_8 = _6 ^ _6;
_11 = _4 * _5;
_1 = [RET,RET,RET,RET,RET];
RET = 10849091009575475647_usize;
_8 = _6;
_5 = _7 | _10;
_11 = !_5;
_13.0 = _2 < _11;
Call(RET = core::intrinsics::bswap(1_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_9 = _7;
_10 = _5 * _5;
_10 = _3;
_16 = [_17,_17,_17,_17];
match RET {
16750732977213967779 => bb10,
_ => bb9
}
}
bb9 = {
_10 = _11 ^ _9;
_8 = _6 ^ _6;
_11 = _4 * _5;
_1 = [RET,RET,RET,RET,RET];
RET = 10849091009575475647_usize;
_8 = _6;
_5 = _7 | _10;
_11 = !_5;
_13.0 = _2 < _11;
Call(RET = core::intrinsics::bswap(1_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_13.0 = _6;
_13 = (_8,);
_18 = [_7,_5,_9,_5,_11,_2];
_20 = [_17,_17,_17,_17];
_1 = [RET,RET,RET,RET,RET];
_14.1 = [_5,_10,_9,_11,_7,_5];
_18 = [_9,_10,_3,_5,_9,_4];
_18 = [_10,_7,_7,_5,_2,_9];
_11 = !_3;
_20 = [_17,_17,_17,_17];
_21 = !82492645999179758422544614923928550514_i128;
_17 = !119_u8;
_16 = _20;
_10 = !_5;
_7 = _9 >> _10;
_14.0 = _19;
RET = 16687930299626529460_usize ^ 7_usize;
_19 = _14.3 >> _2;
_2 = _19 as u128;
_18 = [_10,_7,_4,_3,_10,_5];
_11 = _5 << _9;
_19 = _14.0 * _14.0;
_6 = _13.0;
_18 = _14.1;
_8 = _13.0;
Goto(bb11)
}
bb11 = {
_21 = 1119155985_i32 as i128;
_18 = [_2,_5,_9,_9,_5,_10];
_4 = !_5;
_11 = _10;
_13.0 = _8 | _6;
_14.0 = _14.3;
_13.0 = _6 ^ _8;
_26 = -_15;
_10 = 1771564325_i32 as u128;
_5 = _9;
_14.1 = _18;
_28 = [_14.0,_19,_14.0,_19,_19,_14.3];
_29 = (_4, _21);
_18 = [_3,_4,_7,_7,_11,_29.0];
_9 = _7;
_27 = _14.3 as f32;
_26 = (-926442320_i32) as isize;
_24 = _26 & _15;
_1 = [RET,RET,RET,RET,RET];
match _15 {
0 => bb4,
340282366920938463454151235394913435648 => bb12,
_ => bb5
}
}
bb12 = {
_16 = [_17,_17,_17,_17];
_14.2 = core::ptr::addr_of_mut!(_27);
_25 = _6;
_1 = [RET,RET,RET,RET,RET];
_11 = _29.0 * _7;
_15 = -_24;
_14.0 = _14.3;
_18 = _14.1;
_9 = _29.0 << _2;
_12 = [_19,_19,_14.3,_19,_19,_19];
_14.4 = !815_u16;
_14.4 = 29988_u16;
Goto(bb13)
}
bb13 = {
_14.3 = !_19;
_20 = [_17,_17,_17,_17];
_31 = _27 * _27;
_33 = (-11947_i16) as f32;
_14.3 = _19;
_7 = _14.4 as u128;
_22 = _17 << _29.0;
_17 = _22 & _22;
_32 = 23141_i16 as u32;
_13.0 = _22 >= _17;
_27 = _31 - _33;
_8 = _4 >= _11;
_4 = _32 as u128;
_19 = _22 as u64;
_2 = _5;
_32 = 2079678162_u32 | 3183452981_u32;
_14.2 = core::ptr::addr_of_mut!(_31);
_27 = -_31;
_16 = _20;
_29 = (_11, _21);
_28 = [_19,_19,_19,_19,_19,_19];
_13.0 = _19 == _19;
RET = !6_usize;
_9 = !_29.0;
_29.1 = -_21;
_14.0 = _22 as u64;
_13 = (_25,);
_14.1 = _18;
match _14.4 {
0 => bb11,
29988 => bb15,
_ => bb14
}
}
bb14 = {
_13.0 = _6;
_13 = (_8,);
_18 = [_7,_5,_9,_5,_11,_2];
_20 = [_17,_17,_17,_17];
_1 = [RET,RET,RET,RET,RET];
_14.1 = [_5,_10,_9,_11,_7,_5];
_18 = [_9,_10,_3,_5,_9,_4];
_18 = [_10,_7,_7,_5,_2,_9];
_11 = !_3;
_20 = [_17,_17,_17,_17];
_21 = !82492645999179758422544614923928550514_i128;
_17 = !119_u8;
_16 = _20;
_10 = !_5;
_7 = _9 >> _10;
_14.0 = _19;
RET = 16687930299626529460_usize ^ 7_usize;
_19 = _14.3 >> _2;
_2 = _19 as u128;
_18 = [_10,_7,_4,_3,_10,_5];
_11 = _5 << _9;
_19 = _14.0 * _14.0;
_6 = _13.0;
_18 = _14.1;
_8 = _13.0;
Goto(bb11)
}
bb15 = {
_10 = _11 - _11;
_13.0 = _29.0 >= _10;
_28 = _12;
_23 = !_11;
_27 = _31 - _31;
_7 = _9 & _9;
_31 = _27 + _27;
_7 = !_9;
_5 = '\u{1e3be}' as u128;
RET = 1_usize ^ 15534001005427611616_usize;
_17 = 62_i8 as u8;
_9 = _3;
_16 = [_22,_22,_22,_22];
_14.0 = _10 as u64;
_14.4 = !15347_u16;
_31 = _14.4 as f32;
_27 = _33;
RET = !12077724379006050199_usize;
_13.0 = _29.0 <= _23;
_9 = !_29.0;
_19 = _14.0;
_34 = _14.4 + _14.4;
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(11_usize, 21_usize, Move(_21), 3_usize, Move(_3), 6_usize, Move(_6), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(11_usize, 10_usize, Move(_10), 28_usize, Move(_28), 23_usize, Move(_23), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(11_usize, 13_usize, Move(_13), 8_usize, Move(_8), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(11_usize, 20_usize, Move(_20), 1_usize, Move(_1), 37_usize, _37, 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u128,mut _2: [u64; 6],mut _3: u128,mut _4: u128,mut _5: u128,mut _6: bool) -> usize {
mir! {
type RET = usize;
let _7: [u8; 4];
let _8: &'static f64;
let _9: [u64; 6];
let _10: u128;
let _11: Adt55;
let _12: [u64; 6];
let _13: u8;
let _14: char;
let _15: (isize, i64, i32);
let _16: isize;
let _17: isize;
let _18: f64;
let _19: isize;
let _20: u64;
let _21: Adt46;
let _22: [usize; 5];
let _23: Adt53;
let _24: [bool; 7];
let _25: bool;
let _26: [i32; 7];
let _27: u64;
let _28: (bool,);
let _29: ();
let _30: ();
{
_5 = _3 - _3;
Call(RET = fn13(_3, _5, _5, _1, _2, _1, _1, _2, _5, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 7_usize;
_1 = !_4;
Goto(bb2)
}
bb2 = {
RET = 5_usize;
_2[RET] = 4343728263288013826_u64;
_2[RET] = 489108282467060019_u64 << _3;
_3 = !_5;
_7 = [205_u8,214_u8,196_u8,92_u8];
_6 = false;
RET = 6_usize;
Goto(bb3)
}
bb3 = {
RET = !11422150870101830780_usize;
_1 = _3 + _4;
_4 = _6 as u128;
_4 = _1;
_5 = !_1;
RET = 2_usize;
_9 = _2;
_7 = [229_u8,25_u8,124_u8,198_u8];
RET = 7260402436783176072_usize;
_11.fld0 = 122808017980301408781520341545679612406_i128 as f32;
_7 = [6_u8,133_u8,48_u8,27_u8];
_5 = _1;
RET = '\u{652b2}' as usize;
_5 = (-78204962_i32) as u128;
Goto(bb4)
}
bb4 = {
_10 = 18974_i16 as u128;
_12 = _2;
_6 = false & false;
_11.fld0 = 144_u8 as f32;
_7 = [8_u8,125_u8,227_u8,206_u8];
_10 = _4;
_11.fld0 = (-6913688079854872585_i64) as f32;
_4 = !_10;
_4 = (-109_i8) as u128;
_12 = [18089215606516918831_u64,16765439230090006047_u64,198245837576155973_u64,7214400244412352711_u64,9581894370197992890_u64,8207872705429099446_u64];
_9 = [15085150067769481221_u64,6546744235235115970_u64,10966400758475254762_u64,2338339323328702398_u64,17767224494377628226_u64,7121059663177599923_u64];
_1 = 7912627241512174742_u64 as u128;
_3 = _10 >> _10;
RET = 6_usize ^ 3_usize;
_10 = !_3;
_10 = _11.fld0 as u128;
_4 = _3 & _3;
_9 = [8655798285708609735_u64,15164455700357663422_u64,14653935961604431357_u64,1459692833599736790_u64,1031635210167312220_u64,10662554067481513768_u64];
_15.1 = 2311810992_u32 as i64;
_1 = !_3;
Goto(bb5)
}
bb5 = {
_11.fld0 = 3390927310_u32 as f32;
_13 = 99_u8;
_2 = _9;
RET = _6 as usize;
_16 = 9223372036854775807_isize;
_4 = !_3;
RET = 1_usize;
_15.0 = _16;
_15.0 = 11_i8 as isize;
_7 = [_13,_13,_13,_13];
_7[RET] = !_13;
_14 = '\u{46786}';
_17 = _1 as isize;
_7[RET] = (-10_i8) as u8;
_15 = (_17, 5109273334260131434_i64, 337056517_i32);
_14 = '\u{dbbb2}';
_15 = (_17, (-3155809375053986671_i64), 580170419_i32);
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb6 = {
_10 = 18974_i16 as u128;
_12 = _2;
_6 = false & false;
_11.fld0 = 144_u8 as f32;
_7 = [8_u8,125_u8,227_u8,206_u8];
_10 = _4;
_11.fld0 = (-6913688079854872585_i64) as f32;
_4 = !_10;
_4 = (-109_i8) as u128;
_12 = [18089215606516918831_u64,16765439230090006047_u64,198245837576155973_u64,7214400244412352711_u64,9581894370197992890_u64,8207872705429099446_u64];
_9 = [15085150067769481221_u64,6546744235235115970_u64,10966400758475254762_u64,2338339323328702398_u64,17767224494377628226_u64,7121059663177599923_u64];
_1 = 7912627241512174742_u64 as u128;
_3 = _10 >> _10;
RET = 6_usize ^ 3_usize;
_10 = !_3;
_10 = _11.fld0 as u128;
_4 = _3 & _3;
_9 = [8655798285708609735_u64,15164455700357663422_u64,14653935961604431357_u64,1459692833599736790_u64,1031635210167312220_u64,10662554067481513768_u64];
_15.1 = 2311810992_u32 as i64;
_1 = !_3;
Goto(bb5)
}
bb7 = {
RET = !11422150870101830780_usize;
_1 = _3 + _4;
_4 = _6 as u128;
_4 = _1;
_5 = !_1;
RET = 2_usize;
_9 = _2;
_7 = [229_u8,25_u8,124_u8,198_u8];
RET = 7260402436783176072_usize;
_11.fld0 = 122808017980301408781520341545679612406_i128 as f32;
_7 = [6_u8,133_u8,48_u8,27_u8];
_5 = _1;
RET = '\u{652b2}' as usize;
_5 = (-78204962_i32) as u128;
Goto(bb4)
}
bb8 = {
RET = 5_usize;
_2[RET] = 4343728263288013826_u64;
_2[RET] = 489108282467060019_u64 << _3;
_3 = !_5;
_7 = [205_u8,214_u8,196_u8,92_u8];
_6 = false;
RET = 6_usize;
Goto(bb3)
}
bb9 = {
RET = 7_usize;
_1 = !_4;
Goto(bb2)
}
bb10 = {
_16 = 93_i8 as isize;
_1 = (-103_i8) as u128;
_15.2 = 538167543_i32;
RET = (-86_i8) as usize;
_7 = [_13,_13,_13,_13];
_8 = &_18;
_14 = '\u{54ee5}';
_15.0 = _15.2 as isize;
_12 = [14552757479495946008_u64,7818279590439793387_u64,4297110057168636394_u64,15841634768515815245_u64,3229785623897453710_u64,8736739050327186633_u64];
_6 = true ^ false;
_4 = 12350731922558398632_u64 as u128;
_19 = _17 << _3;
_19 = _17;
_13 = 16_u8;
_17 = _19 * _19;
RET = !3_usize;
_3 = _1;
Goto(bb11)
}
bb11 = {
_8 = &(*_8);
RET = 2034306488688093916_usize;
_8 = &(*_8);
_4 = _3 | _5;
_18 = _1 as f64;
_15.0 = _15.1 as isize;
_11.fld0 = _15.2 as f32;
Goto(bb12)
}
bb12 = {
_24 = [_6,_6,_6,_6,_6,_6,_6];
_15.2 = _18 as i32;
_16 = -_17;
_15.2 = _15.1 as i32;
_17 = !_16;
_10 = _6 as u128;
_22 = [RET,RET,RET,RET,RET];
_17 = !_16;
match _15.1 {
0 => bb6,
1 => bb13,
340282366920938463460218798056714224785 => bb15,
_ => bb14
}
}
bb13 = {
_11.fld0 = 3390927310_u32 as f32;
_13 = 99_u8;
_2 = _9;
RET = _6 as usize;
_16 = 9223372036854775807_isize;
_4 = !_3;
RET = 1_usize;
_15.0 = _16;
_15.0 = 11_i8 as isize;
_7 = [_13,_13,_13,_13];
_7[RET] = !_13;
_14 = '\u{46786}';
_17 = _1 as isize;
_7[RET] = (-10_i8) as u8;
_15 = (_17, 5109273334260131434_i64, 337056517_i32);
_14 = '\u{dbbb2}';
_15 = (_17, (-3155809375053986671_i64), 580170419_i32);
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb14 = {
_16 = 93_i8 as isize;
_1 = (-103_i8) as u128;
_15.2 = 538167543_i32;
RET = (-86_i8) as usize;
_7 = [_13,_13,_13,_13];
_8 = &_18;
_14 = '\u{54ee5}';
_15.0 = _15.2 as isize;
_12 = [14552757479495946008_u64,7818279590439793387_u64,4297110057168636394_u64,15841634768515815245_u64,3229785623897453710_u64,8736739050327186633_u64];
_6 = true ^ false;
_4 = 12350731922558398632_u64 as u128;
_19 = _17 << _3;
_19 = _17;
_13 = 16_u8;
_17 = _19 * _19;
RET = !3_usize;
_3 = _1;
Goto(bb11)
}
bb15 = {
_20 = !11790374880689142318_u64;
_7 = [_13,_13,_13,_13];
_26 = [_15.2,_15.2,_15.2,_15.2,_15.2,_15.2,_15.2];
_20 = !2033845376761728549_u64;
_1 = _10 >> _15.2;
_15.2 = 1287795078_i32;
_8 = &_18;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(12_usize, 2_usize, Move(_2), 24_usize, Move(_24), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(12_usize, 14_usize, Move(_14), 4_usize, Move(_4), 6_usize, Move(_6), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(12_usize, 26_usize, Move(_26), 20_usize, Move(_20), 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: [u64; 6],mut _6: u128,mut _7: u128,mut _8: [u64; 6],mut _9: u128,mut _10: bool) -> usize {
mir! {
type RET = usize;
let _11: [u64; 6];
let _12: [u32; 6];
let _13: (u128, i128);
let _14: Adt44;
let _15: f32;
let _16: char;
let _17: f32;
let _18: (isize, i64, i32);
let _19: (isize, i64, i32);
let _20: i32;
let _21: [u8; 4];
let _22: *const bool;
let _23: [u64; 1];
let _24: isize;
let _25: Adt56;
let _26: isize;
let _27: f64;
let _28: char;
let _29: i16;
let _30: [u64; 6];
let _31: f64;
let _32: bool;
let _33: f64;
let _34: i128;
let _35: Adt50;
let _36: f32;
let _37: Adt44;
let _38: [bool; 7];
let _39: [usize; 5];
let _40: [u32; 6];
let _41: isize;
let _42: u16;
let _43: isize;
let _44: i32;
let _45: u8;
let _46: [u64; 6];
let _47: isize;
let _48: [u8; 4];
let _49: [u64; 1];
let _50: isize;
let _51: (u64, [u128; 6], *mut f32, u64, u16);
let _52: (u128, i128);
let _53: ();
let _54: ();
{
_7 = (-9223372036854775808_isize) as u128;
_5 = [16096328093004920052_u64,2689305541702423282_u64,15518252326037228786_u64,12437655566353696876_u64,6365896611289844829_u64,17087471244742526354_u64];
Goto(bb1)
}
bb1 = {
RET = 79_i8 as usize;
_2 = _9;
_7 = !_2;
_2 = !_9;
_11 = _5;
_1 = _6 & _7;
_10 = _7 >= _2;
_6 = !_9;
_4 = _6;
_13.0 = _6;
_13.0 = _2;
_13.1 = 95923376470267016079943923103035380555_i128;
_9 = _13.0 - _2;
_9 = _4 - _7;
_13 = (_2, 67901652596870133415330612219766728498_i128);
_10 = false;
_4 = !_3;
_4 = _13.1 as u128;
_12 = [674977142_u32,740234144_u32,4083911481_u32,1155855102_u32,89943029_u32,48565623_u32];
Goto(bb2)
}
bb2 = {
_12 = [4234093578_u32,1728449531_u32,3900519600_u32,2622564676_u32,1042794936_u32,1466222695_u32];
_5 = [5061408497760294345_u64,5742632828796080153_u64,9875564609176063505_u64,4541678239786122238_u64,17624142236462961577_u64,8035117007597235295_u64];
_14.fld0 = _1 + _13.0;
_1 = _2;
_7 = _14.fld0 + _14.fld0;
_13 = (_7, (-97853181212873005747962571824332489375_i128));
_11 = _5;
_6 = !_9;
_10 = _7 < _14.fld0;
_5 = [5285076626839739102_u64,9828229655178136837_u64,13043133320478249529_u64,16298440661618275422_u64,15358289424212098541_u64,7730714646571376654_u64];
_11 = [3870706647019653003_u64,13108852874833155175_u64,18067402948103786231_u64,3493789063917357502_u64,9758251834302008594_u64,5994907765659676143_u64];
_17 = _13.1 as f32;
_13.1 = (-79638504391488362984632602792084205392_i128);
_18.0 = 38_isize;
_13.1 = (-23922307588743100767943129744807549466_i128);
_6 = _13.1 as u128;
_4 = _7 & _7;
_18.1 = 6923391771165537033_u64 as i64;
_10 = _4 > _1;
_18.2 = !(-2080771853_i32);
_13.0 = !_7;
_14.fld0 = _3;
_13.0 = _4 & _4;
_15 = -_17;
_18.0 = -9223372036854775807_isize;
_15 = -_17;
Call(_14 = fn14(_7, _15, _4, _13.0, _10, _15, _7, _4, _4, _13, _17, _4, _9, _15, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = (_18.0, _18.1, _18.2);
_1 = _4 & _13.0;
Goto(bb4)
}
bb4 = {
_19.2 = _18.2 >> _13.0;
_4 = 2288119408_u32 as u128;
_12 = [2183935457_u32,1166308231_u32,859953531_u32,3219462429_u32,2479864817_u32,302951234_u32];
_7 = RET as u128;
RET = 0_usize;
RET = 2521010000299953570_usize;
_13 = (_14.fld0, 2788062367621427087498955306398980189_i128);
_14.fld0 = _1;
Call(_5 = fn15(_19, _19, _1, _14.fld0, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = '\u{7e923}';
_4 = !_13.0;
_19 = (_18.0, _18.1, _18.2);
_2 = _4;
_2 = _4;
_2 = _4;
_1 = _2 >> _13.0;
_20 = 7567359330089523547_u64 as i32;
_14.fld0 = _1;
_19.1 = _18.2 as i64;
_21 = [191_u8,179_u8,54_u8,39_u8];
_9 = !_1;
_2 = 66_u8 as u128;
_4 = !_14.fld0;
RET = 11772802845167183090_usize;
_13.1 = 138946507209426139532066054324942589894_i128;
_12 = [2477776311_u32,2865581818_u32,1454935236_u32,2717676299_u32,400733851_u32,3652140303_u32];
_13 = (_4, 161376790097927193510619821651504552844_i128);
_6 = _4 ^ _13.0;
RET = (-17384_i16) as usize;
_1 = _14.fld0 + _4;
_22 = core::ptr::addr_of!(_10);
Goto(bb6)
}
bb6 = {
_12 = [990252906_u32,1045802275_u32,1054741091_u32,1862466460_u32,2272807148_u32,3238017646_u32];
_13.1 = _18.2 as i128;
_21 = [80_u8,58_u8,183_u8,37_u8];
_13.1 = (-122751424685973764465046115180622301382_i128) << _1;
_18.0 = _19.0 * _19.0;
_17 = _15 * _15;
_14 = Adt44 { fld0: _6 };
_23 = [16366642035954072237_u64];
_10 = false;
_19.1 = _18.1;
Goto(bb7)
}
bb7 = {
_19.0 = !_18.0;
_21 = [15_u8,183_u8,9_u8,83_u8];
_11 = _5;
_9 = _19.1 as u128;
_19.2 = RET as i32;
RET = !6_usize;
_19.1 = _18.1 | _18.1;
_14.fld0 = _1;
(*_22) = true;
_22 = core::ptr::addr_of!((*_22));
_2 = !_14.fld0;
_18.1 = -_19.1;
_16 = '\u{513ce}';
_19.1 = -_18.1;
_27 = RET as f64;
_19 = _18;
Goto(bb8)
}
bb8 = {
_13.1 = (-133892034957557907751850684852588606538_i128) & (-153879784916977524063621587539976332073_i128);
_23 = [16595041857896617604_u64];
_2 = _14.fld0;
_23 = [12497412458465830537_u64];
_27 = RET as f64;
_2 = 178_u8 as u128;
_19.1 = _18.1 << _13.0;
_29 = -(-24132_i16);
_9 = _14.fld0 - _14.fld0;
_19.1 = _18.1 >> _4;
_15 = RET as f32;
_10 = !false;
_27 = 4048900223_u32 as f64;
_19 = (_18.0, _18.1, _20);
Call(_18.0 = core::intrinsics::transmute(RET), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_23 = [11736361234503877345_u64];
_3 = 17088069282420680675_u64 as u128;
RET = 1_usize;
_31 = -_27;
_18.0 = !_19.0;
_10 = true;
_30[RET] = _19.0 as u64;
_30[RET] = _13.1 as u64;
_7 = !_13.0;
_30 = [_11[RET],_11[RET],_11[RET],_5[RET],_11[RET],_11[RET]];
Goto(bb10)
}
bb10 = {
_21[RET] = !188_u8;
_8[RET] = _30[RET];
_14 = Adt44 { fld0: _13.0 };
_21[RET] = 31759_u16 as u8;
_13.0 = !_7;
(*_22) = !true;
_24 = _19.0 + _18.0;
_13.1 = (-11293472342456263688483293854300525106_i128) ^ 81622206060196793322805490178760694505_i128;
_31 = _13.0 as f64;
_26 = _24;
_5 = [_8[RET],_11[RET],_8[RET],_11[RET],_8[RET],_30[RET]];
_24 = !_19.0;
_38 = [(*_22),(*_22),(*_22),(*_22),_10,(*_22),(*_22)];
_18.2 = _19.2;
_3 = !_1;
_5[RET] = _19.0 as u64;
_30 = [_8[RET],_11[RET],_11[RET],_8[RET],_8[RET],_8[RET]];
(*_22) = _38[RET];
(*_22) = _38[RET] & _38[RET];
_26 = _16 as isize;
_40 = [_12[RET],_12[RET],_12[RET],_12[RET],_12[RET],_12[RET]];
(*_22) = !_38[RET];
RET = !3_usize;
_24 = _29 as isize;
_19.1 = _18.1;
_33 = -_31;
Goto(bb11)
}
bb11 = {
_32 = _10;
_33 = -_31;
_37.fld0 = _3 * _13.0;
_28 = _16;
_20 = _18.2 >> _4;
_29 = 4916_i16;
_27 = 5048977059460732639_u64 as f64;
_6 = _14.fld0;
_2 = !_14.fld0;
_32 = !(*_22);
_33 = _29 as f64;
_5 = _11;
_39 = [RET,RET,RET,RET,RET];
_39 = [RET,RET,RET,RET,RET];
_16 = _28;
_30 = [5114072623694258726_u64,12715085858343602892_u64,13151241611277616363_u64,14849937046901318471_u64,17547576252668717126_u64,13394749218546668460_u64];
_30 = [13172961886138926020_u64,12771793998602292875_u64,13723233017188758265_u64,3810404389202875235_u64,1050752237556919056_u64,10907633063597170_u64];
_30 = _11;
_19.1 = !_18.1;
_10 = _32 & _32;
_37 = Adt44 { fld0: _7 };
_34 = _13.1;
(*_22) = _32;
_8 = _11;
_19.0 = _18.0;
_41 = _18.0;
_19 = (_41, _18.1, _20);
Goto(bb12)
}
bb12 = {
_14 = _37;
_23 = [618513693154244858_u64];
_6 = _2 & _9;
_5 = [14401152927178629851_u64,6559356082856344073_u64,7637133700295348101_u64,14114586870453575557_u64,16680114381520721686_u64,10273489786003857815_u64];
_10 = !_32;
_6 = _2 & _2;
_14.fld0 = !_7;
_30 = _8;
_37 = Adt44 { fld0: _7 };
(*_22) = !_32;
_23 = [8217146329032936842_u64];
RET = 8224480706873476402_usize & 0_usize;
_36 = -_17;
_17 = -_36;
_14.fld0 = _9;
_22 = core::ptr::addr_of!(_10);
_31 = _33 - _27;
_38 = [(*_22),(*_22),(*_22),_10,_32,_32,(*_22)];
_43 = _26 >> _7;
_34 = _13.1 ^ _13.1;
_37.fld0 = _7;
Goto(bb13)
}
bb13 = {
_19.2 = _20 >> _3;
_47 = _43;
_6 = _1;
_38 = [_32,_32,(*_22),_10,(*_22),(*_22),_32];
_47 = _43 << _3;
_14 = _37;
_48 = [129_u8,38_u8,188_u8,48_u8];
_42 = 59428_u16 | 21754_u16;
(*_22) = _3 > _6;
_37 = Adt44 { fld0: _14.fld0 };
_44 = _19.2;
_9 = _6 & _6;
_6 = _37.fld0 + _13.0;
_48 = [133_u8,66_u8,231_u8,61_u8];
_46 = _8;
_8 = _30;
Call(_8 = fn17((*_22), _44, _2, _22, _36, _22, _19, _22, _19, _46), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_18.1 = _19.1;
_17 = -_36;
_19.2 = _44;
_8 = _46;
_51.3 = 9872312121266063631_u64;
_36 = _43 as f32;
_47 = _43 & _43;
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(13_usize, 28_usize, Move(_28), 20_usize, Move(_20), 40_usize, Move(_40), 34_usize, Move(_34)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(13_usize, 9_usize, Move(_9), 30_usize, Move(_30), 10_usize, Move(_10), 47_usize, Move(_47)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(13_usize, 29_usize, Move(_29), 12_usize, Move(_12), 41_usize, Move(_41), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(13_usize, 4_usize, Move(_4), 38_usize, Move(_38), 2_usize, Move(_2), 43_usize, Move(_43)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(13_usize, 7_usize, Move(_7), 24_usize, Move(_24), 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: u128,mut _2: f32,mut _3: u128,mut _4: u128,mut _5: bool,mut _6: f32,mut _7: u128,mut _8: u128,mut _9: u128,mut _10: (u128, i128),mut _11: f32,mut _12: u128,mut _13: u128,mut _14: f32,mut _15: (u128, i128)) -> Adt44 {
mir! {
type RET = Adt44;
let _16: f64;
let _17: i128;
let _18: isize;
let _19: ();
let _20: ();
{
_4 = 0_usize as u128;
_9 = 4955499641452376128_usize as u128;
match _15.1 {
0 => bb1,
1 => bb2,
316360059332195362695431477686960661990 => bb4,
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
_7 = 32744_u16 as u128;
_8 = _3;
RET.fld0 = _2 as u128;
_14 = _11 + _2;
_15.0 = !_3;
_16 = _6 as f64;
RET.fld0 = _15.0;
_16 = (-114_i8) as f64;
RET.fld0 = (-3313718477098402645_i64) as u128;
_6 = _14;
_17 = _10.1;
_11 = _6;
_6 = _11 * _2;
_12 = 1212214319022360527_i64 as u128;
RET = Adt44 { fld0: _1 };
RET.fld0 = _10.0 >> _13;
Goto(bb5)
}
bb5 = {
Call(_19 = dump_var(14_usize, 15_usize, Move(_15), 4_usize, Move(_4), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_19 = dump_var(14_usize, 5_usize, Move(_5), 7_usize, Move(_7), 20_usize, _20, 20_usize, _20), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: (isize, i64, i32),mut _2: (isize, i64, i32),mut _3: u128,mut _4: u128,mut _5: bool) -> [u64; 6] {
mir! {
type RET = [u64; 6];
let _6: u32;
let _7: i16;
let _8: i128;
let _9: char;
let _10: u128;
let _11: f64;
let _12: [u64; 7];
let _13: ();
let _14: ();
{
_5 = true;
_1.1 = _2.1 >> _1.2;
_6 = 2467184676_u32 - 1778278330_u32;
_2.1 = _1.1;
_2.0 = !_1.0;
RET = [7387504309499418784_u64,10089218947665949222_u64,3627595647243574982_u64,6327367531907148367_u64,18066129530679514919_u64,8357799248072902274_u64];
RET = [15928346203603681999_u64,13427951087215217126_u64,1055220004745739420_u64,7319618571741979191_u64,14930581022976987639_u64,1594582919387367274_u64];
_1 = (_2.0, _2.1, _2.2);
_7 = (-335_i16) | 8470_i16;
_4 = !_3;
RET = [16084319534255756351_u64,7166867990023811847_u64,11360224259229785912_u64,11443871490581684005_u64,12905273129610428182_u64,113147618982860086_u64];
_8 = -(-120113328641565710247698409410521816175_i128);
_6 = !4242518934_u32;
Call(RET = fn16(_2.2, _2.1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 225_u8 as u32;
_6 = !3268793366_u32;
_7 = 6558_i16;
_1 = (_2.0, _2.1, _2.2);
_9 = '\u{5f0ae}';
_2.0 = _1.0 ^ _1.0;
_9 = '\u{53c7b}';
_1 = (_2.0, _2.1, _2.2);
_1.1 = !_2.1;
_1.0 = _2.0 - _2.0;
_10 = _3;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(15_usize, 9_usize, Move(_9), 3_usize, Move(_3), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(15_usize, 2_usize, Move(_2), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: i32,mut _2: i64,mut _3: (isize, i64, i32)) -> [u64; 6] {
mir! {
type RET = [u64; 6];
let _4: u32;
let _5: [u64; 7];
let _6: ([u128; 6], i8, f32, u8);
let _7: [i32; 7];
let _8: i32;
let _9: Adt44;
let _10: Adt53;
let _11: u64;
let _12: bool;
let _13: ([u128; 6], i8, f32, u8);
let _14: char;
let _15: [u64; 6];
let _16: [u64; 1];
let _17: i16;
let _18: isize;
let _19: bool;
let _20: Adt53;
let _21: i8;
let _22: (u64, [u128; 6], *mut f32, u64, u16);
let _23: Adt42;
let _24: [u128; 6];
let _25: ();
let _26: ();
{
_3.0 = -(-9223372036854775808_isize);
_1 = _3.2;
_4 = !1754424618_u32;
_4 = _1 as u32;
RET = [14963894312311172995_u64,3444415558459592794_u64,10844989211874776321_u64,8434894928864846507_u64,4939140485808623956_u64,1186680855064373252_u64];
_3 = (9223372036854775807_isize, _2, _1);
_2 = -_3.1;
_2 = _3.1 | _3.1;
_5 = [3426145661370147800_u64,11848739946221062667_u64,12063580370340537624_u64,4896387377733351284_u64,238557213667961216_u64,10782798820096986772_u64,12469018878758606396_u64];
Goto(bb1)
}
bb1 = {
_3 = ((-9223372036854775808_isize), _2, _1);
_3.2 = _1 | _1;
_2 = _3.1;
_3 = (9223372036854775807_isize, _2, _1);
_3.0 = 9223372036854775807_isize;
_3.0 = -9223372036854775807_isize;
_6.1 = !(-44_i8);
_1 = 18160109930473186192_u64 as i32;
_6.3 = 88_u8;
_6.0 = [2513973584543361421793543364285588905_u128,135101991268226963136862042850141529348_u128,248786337311924690689639072080555566451_u128,327800733774102973486190862442843579099_u128,119285956610491373913235180034951290607_u128,231064751812814370762099461229754102448_u128];
_8 = _3.2;
_4 = 2341752622_u32 << _2;
_6.2 = _6.3 as f32;
_9 = Adt44 { fld0: 199923761758135880675782229312734381718_u128 };
RET = [16147588708419825309_u64,9271631563405664797_u64,9040950691367176152_u64,877225261875352296_u64,4288229589159816508_u64,14858764716143633440_u64];
Call(_6.3 = core::intrinsics::transmute(_6.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = ((-50_isize), _2, _8);
_7 = [_3.2,_8,_3.2,_3.2,_8,_8,_8];
_9.fld0 = 96654167809943806312879632675747532561_u128;
_3.0 = _3.2 as isize;
_6.2 = 27981_i16 as f32;
_3.0 = true as isize;
_3 = ((-9223372036854775808_isize), _2, _8);
_6.0 = [_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0];
_6.0 = [_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0];
_6.2 = _9.fld0 as f32;
_3.1 = _2;
match _3.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb3 = {
_3 = ((-9223372036854775808_isize), _2, _1);
_3.2 = _1 | _1;
_2 = _3.1;
_3 = (9223372036854775807_isize, _2, _1);
_3.0 = 9223372036854775807_isize;
_3.0 = -9223372036854775807_isize;
_6.1 = !(-44_i8);
_1 = 18160109930473186192_u64 as i32;
_6.3 = 88_u8;
_6.0 = [2513973584543361421793543364285588905_u128,135101991268226963136862042850141529348_u128,248786337311924690689639072080555566451_u128,327800733774102973486190862442843579099_u128,119285956610491373913235180034951290607_u128,231064751812814370762099461229754102448_u128];
_8 = _3.2;
_4 = 2341752622_u32 << _2;
_6.2 = _6.3 as f32;
_9 = Adt44 { fld0: 199923761758135880675782229312734381718_u128 };
RET = [16147588708419825309_u64,9271631563405664797_u64,9040950691367176152_u64,877225261875352296_u64,4288229589159816508_u64,14858764716143633440_u64];
Call(_6.3 = core::intrinsics::transmute(_6.1), ReturnTo(bb2), UnwindUnreachable())
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
_10 = Adt53::Variant2 { fld0: _6.1,fld1: _9 };
Call(_11 = core::intrinsics::transmute(_3.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = 1994315355_u32 & 3776417378_u32;
_4 = 1940196098_u32 - 2310437018_u32;
_6.3 = 50_u8;
_6.2 = _8 as f32;
_3.1 = _2;
Goto(bb11)
}
bb11 = {
_3 = (9223372036854775807_isize, _2, _8);
_6.3 = 54_u8 - 165_u8;
_8 = -_3.2;
_12 = _6.2 <= _6.2;
place!(Field::<Adt44>(Variant(_10, 2), 1)) = Adt44 { fld0: _9.fld0 };
_8 = -_3.2;
SetDiscriminant(_10, 3);
_13.3 = !_6.3;
_2 = _6.3 as i64;
_3.2 = -_8;
_6.1 = '\u{fd86d}' as i8;
_6.3 = !_13.3;
_6.2 = _11 as f32;
_6.0 = [_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0];
_7 = [_8,_8,_3.2,_8,_3.2,_3.2,_3.2];
_13.1 = _6.1;
_13.3 = _6.3;
_12 = true;
_15 = [_11,_11,_11,_11,_11,_11];
_13.3 = _3.1 as u8;
_13.0 = _6.0;
_13.0 = _6.0;
_6.3 = _13.3 >> _8;
_11 = _9.fld0 as u64;
_14 = '\u{d5483}';
place!(Field::<*const bool>(Variant(_10, 3), 1)) = core::ptr::addr_of!(_12);
_15 = [_11,_11,_11,_11,_11,_11];
Goto(bb12)
}
bb12 = {
_16 = [_11];
_10 = Adt53::Variant2 { fld0: _13.1,fld1: _9 };
RET = _15;
SetDiscriminant(_10, 0);
place!(Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2)).0 = _3.0 as u64;
_14 = '\u{e0f4}';
_17 = 50530_u16 as i16;
_3.2 = _13.1 as i32;
place!(Field::<f32>(Variant(_10, 0), 4)) = _6.2;
_11 = Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0;
place!(Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2)).2 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_10, 0), 4)));
place!(Field::<u16>(Variant(_10, 0), 3)) = 25967_u16 >> _11;
Goto(bb13)
}
bb13 = {
_3 = (69_isize, _2, _8);
_18 = _3.0;
_11 = Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0 | Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0;
_15 = [Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,_11];
RET = [Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0];
_13 = (_6.0, _6.1, Field::<f32>(Variant(_10, 0), 4), _6.3);
place!(Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2)).1 = _13.0;
_5 = [_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,_11,_11,_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0];
_6.2 = _13.2 - Field::<f32>(Variant(_10, 0), 4);
_13.1 = _6.1 | _6.1;
_19 = _12;
_22.2 = Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).2;
_13.0 = Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).1;
_4 = !4142136633_u32;
_15 = [_11,_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0,_11,Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0];
_11 = Field::<(u64, [u128; 6], *mut f32, u64, u16)>(Variant(_10, 0), 2).0;
place!(Field::<u16>(Variant(_10, 0), 3)) = _13.2 as u16;
_13.0 = [_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0,_9.fld0];
_13 = _6;
Goto(bb14)
}
bb14 = {
Call(_25 = dump_var(16_usize, 12_usize, Move(_12), 5_usize, Move(_5), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_25 = dump_var(16_usize, 19_usize, Move(_19), 18_usize, Move(_18), 3_usize, Move(_3), 26_usize, _26), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: bool,mut _2: i32,mut _3: u128,mut _4: *const bool,mut _5: f32,mut _6: *const bool,mut _7: (isize, i64, i32),mut _8: *const bool,mut _9: (isize, i64, i32),mut _10: [u64; 6]) -> [u64; 6] {
mir! {
type RET = [u64; 6];
let _11: [u64; 6];
let _12: [usize; 5];
let _13: [u128; 6];
let _14: (u128, i128);
let _15: ();
let _16: ();
{
_4 = _8;
_9.0 = 3749661727_u32 as isize;
_8 = _4;
_6 = _8;
_9 = (_7.0, _7.1, _7.2);
_7.2 = _2 ^ _9.2;
RET = [3298599890534060181_u64,12098554762397229877_u64,18446674237305560860_u64,4030178241509975780_u64,3931981669870237570_u64,10074133526020179633_u64];
_9.1 = _7.1;
(*_8) = _1;
_9.0 = 68886231284486183817134640467834961600_i128 as isize;
_11 = [9981885900447571680_u64,2271238980118993861_u64,2449535498473982723_u64,8918670501645508878_u64,14532970465475364245_u64,14587014019730670397_u64];
_1 = !(*_4);
RET = [15206506342081073674_u64,13161585624267254207_u64,5083743774577032835_u64,3574253062244229372_u64,17789695040552871365_u64,3260805748902015453_u64];
(*_4) = _5 <= _5;
_12 = [17609271370018946716_usize,0_usize,3_usize,17899610566480648434_usize,6766728644508997559_usize];
_5 = (-59_i8) as f32;
_9.2 = !_2;
_12 = [507786649960108746_usize,4309109069048285893_usize,1136424764871000137_usize,4_usize,1_usize];
(*_8) = !_1;
_7.0 = _5 as isize;
(*_6) = !_1;
_4 = _8;
_12 = [0_usize,10031053622347586661_usize,4531213105715354840_usize,5348203792720097967_usize,2_usize];
_8 = _4;
_3 = !181134938032361431412660429014965388035_u128;
RET = [18373275024449514554_u64,4622857586300388765_u64,11609948453436051489_u64,15353900762109836436_u64,9670634937168902825_u64,16883670859502738696_u64];
Call(_7.2 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14.0 = !_3;
_14.0 = _3;
_5 = 28138068357031458449674235614437723290_i128 as f32;
RET = _10;
(*_4) = _1;
_4 = _6;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(17_usize, 9_usize, Move(_9), 3_usize, Move(_3), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *mut ([u128; 6], i8, f32, u8),mut _2: *mut ([u128; 6], i8, f32, u8),mut _3: *mut ([u128; 6], i8, f32, u8)) -> [u8; 4] {
mir! {
type RET = [u8; 4];
let _4: isize;
let _5: Adt55;
let _6: bool;
let _7: [u64; 1];
let _8: f32;
let _9: [u64; 6];
let _10: [u64; 1];
let _11: char;
let _12: ([u128; 6], i8, f32, u8);
let _13: ();
let _14: ();
{
RET = [127_u8,113_u8,14_u8,155_u8];
RET = [60_u8,237_u8,22_u8,60_u8];
_2 = _3;
_2 = _1;
_3 = _1;
Goto(bb1)
}
bb1 = {
_3 = _1;
_3 = _1;
_3 = _2;
RET = [250_u8,156_u8,96_u8,204_u8];
_4 = (-9223372036854775808_isize) >> 34561_u16;
_1 = _2;
RET = [222_u8,31_u8,152_u8,185_u8];
RET = [234_u8,100_u8,39_u8,176_u8];
_6 = !false;
_1 = _3;
_2 = _3;
RET = [61_u8,252_u8,112_u8,232_u8];
_2 = _3;
RET = [2_u8,97_u8,120_u8,165_u8];
RET = [216_u8,179_u8,232_u8,103_u8];
_4 = 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_5.fld0 = 1189903563_i32 as f32;
_6 = true;
_6 = true ^ true;
_3 = _1;
_1 = _2;
RET = [173_u8,124_u8,88_u8,107_u8];
RET = [180_u8,137_u8,118_u8,35_u8];
_3 = _1;
_6 = !true;
_2 = _3;
_7 = [14660562259181562330_u64];
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
9223372036854775807 => bb11,
_ => bb10
}
}
bb3 = {
_3 = _1;
_3 = _1;
_3 = _2;
RET = [250_u8,156_u8,96_u8,204_u8];
_4 = (-9223372036854775808_isize) >> 34561_u16;
_1 = _2;
RET = [222_u8,31_u8,152_u8,185_u8];
RET = [234_u8,100_u8,39_u8,176_u8];
_6 = !false;
_1 = _3;
_2 = _3;
RET = [61_u8,252_u8,112_u8,232_u8];
_2 = _3;
RET = [2_u8,97_u8,120_u8,165_u8];
RET = [216_u8,179_u8,232_u8,103_u8];
_4 = 9223372036854775807_isize;
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
Return()
}
bb11 = {
_7 = [5823998369455126974_u64];
_1 = _3;
_1 = _2;
_1 = _3;
_3 = _1;
_4 = (-80_isize);
RET = [173_u8,198_u8,38_u8,109_u8];
_7 = [14439965364324216741_u64];
_1 = _3;
_8 = _5.fld0 * _5.fld0;
RET = [8_u8,94_u8,132_u8,169_u8];
_5.fld0 = -_8;
_6 = !true;
_3 = _2;
_1 = _3;
_7 = [17072297052351634640_u64];
_3 = _2;
_1 = _3;
_3 = _2;
Goto(bb12)
}
bb12 = {
_2 = _3;
match _4 {
0 => bb7,
1 => bb11,
2 => bb9,
3 => bb5,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211376 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_3 = _1;
_3 = _1;
_3 = _2;
RET = [250_u8,156_u8,96_u8,204_u8];
_4 = (-9223372036854775808_isize) >> 34561_u16;
_1 = _2;
RET = [222_u8,31_u8,152_u8,185_u8];
RET = [234_u8,100_u8,39_u8,176_u8];
_6 = !false;
_1 = _3;
_2 = _3;
RET = [61_u8,252_u8,112_u8,232_u8];
_2 = _3;
RET = [2_u8,97_u8,120_u8,165_u8];
RET = [216_u8,179_u8,232_u8,103_u8];
_4 = 9223372036854775807_isize;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_9 = [16490697332920120270_u64,4157763236678878094_u64,12711334902491345477_u64,5508418258395458480_u64,14567285602251399507_u64,13654201716372581018_u64];
_5 = Adt55 { fld0: _8 };
_6 = _8 >= _8;
RET = [230_u8,174_u8,212_u8,29_u8];
_12.1 = !(-2_i8);
_9 = [16388153061468486459_u64,14305707957815797445_u64,14144559506302010765_u64,16227575517457826543_u64,826270052591138617_u64,9534066067338013700_u64];
_6 = false;
RET = [89_u8,236_u8,77_u8,54_u8];
_9 = [8071532477090229328_u64,17355711423128171324_u64,16101300911237301509_u64,11281402716373664210_u64,586670278336131216_u64,11094607050018853143_u64];
RET = [246_u8,13_u8,247_u8,107_u8];
_1 = core::ptr::addr_of_mut!(_12);
(*_1).0 = [138740972403614625884352030509420207647_u128,220269652754705632532416761183939867892_u128,237424064714016856407810036068717092630_u128,152282427739645105896834809257317155130_u128,253755083774418821599711113189499667464_u128,152411172186851271076832452159050076317_u128];
(*_1).3 = 32_u8 << _12.1;
_4 = -(-17_isize);
Goto(bb17)
}
bb17 = {
Call(_13 = dump_var(18_usize, 6_usize, Move(_6), 7_usize, Move(_7), 14_usize, _14, 14_usize, _14), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{bbb87}'), std::hint::black_box((-120_isize)), std::hint::black_box((-61_i8)), std::hint::black_box((-8064_i16)), std::hint::black_box(589412556_i32), std::hint::black_box(5692837267721159989_i64), std::hint::black_box(128107363744517096967212887417686031557_i128), std::hint::black_box(8206132920254166105_usize), std::hint::black_box(70885750050481753644977298845004252866_u128), std::hint::black_box(53131_u16), std::hint::black_box(190787679_u32), std::hint::black_box(9209070436375295521_u64));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: [bool; 7],
fld1: *const bool,
fld2: isize,
fld3: u64,
fld4: *mut (u64, [u128; 6], *mut f32, u64, u16),
fld5: [u32; 6],
fld6: usize,
fld7: *mut ([u128; 6], i8, f32, u8),

},
Variant1{
fld0: u128,
fld1: *mut (u64, [u128; 6], *mut f32, u64, u16),
fld2: isize,
fld3: [u128; 6],

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: i64,
fld1: ([u128; 6], i8, f32, u8),
fld2: [u128; 6],

},
Variant1{
fld0: bool,
fld1: *mut f32,
fld2: f64,
fld3: i8,
fld4: i16,
fld5: u32,

},
Variant2{
fld0: bool,
fld1: usize,
fld2: [u64; 7],
fld3: [u32; 6],
fld4: i64,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (u128, i128),
fld1: Adt40,

},
Variant1{
fld0: *mut i32,
fld1: Adt40,
fld2: *const bool,
fld3: u32,

}}
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: usize,
fld1: char,
fld2: (bool, *const bool),
fld3: *mut *const bool,
fld4: u64,
fld5: [u32; 6],
fld6: f64,
fld7: i128,

},
Variant1{
fld0: *mut *const bool,

},
Variant2{
fld0: [i32; 7],
fld1: i8,
fld2: *mut i32,

},
Variant3{
fld0: bool,
fld1: u64,
fld2: isize,
fld3: [u8; 4],
fld4: (bool, *const bool),
fld5: [u64; 7],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: u128,
}
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut *const bool,
fld1: [i32; 7],
fld2: (bool,),
fld3: i8,
fld4: u32,

},
Variant1{
fld0: *mut *const bool,

}}
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [u64; 7],
fld1: Adt42,
fld2: u32,
fld3: Adt40,

},
Variant1{
fld0: [u64; 1],
fld1: (bool, *const bool),
fld2: Adt42,

},
Variant2{
fld0: bool,
fld1: char,
fld2: (bool,),
fld3: [u8; 4],
fld4: i16,
fld5: u128,
fld6: [u64; 7],
fld7: usize,

},
Variant3{
fld0: [i32; 7],
fld1: i128,
fld2: i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt40,
fld1: Adt46,
fld2: ([u128; 6], i8, f32, u8),
fld3: i8,
fld4: i16,
fld5: (isize, i64, i32),
fld6: (bool, *const bool),

},
Variant1{
fld0: u16,
fld1: (u128, i128),
fld2: *mut i32,
fld3: (u64, [u128; 6], *mut f32, u64, u16),
fld4: i16,
fld5: Adt45,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut i32,
fld1: f32,
fld2: Adt43,
fld3: i8,
fld4: [u32; 6],
fld5: Adt46,
fld6: i64,

},
Variant1{
fld0: *mut f32,
fld1: [usize; 5],
fld2: ([u128; 6], i8, f32, u8),
fld3: [i32; 7],

},
Variant2{
fld0: Adt40,
fld1: char,
fld2: f64,
fld3: Adt44,
fld4: usize,
fld5: *mut f32,
fld6: *mut ([u128; 6], i8, f32, u8),

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [u32; 6],
fld1: i32,

},
Variant1{
fld0: *const bool,
fld1: [u64; 6],
fld2: *mut *const bool,
fld3: u64,
fld4: [bool; 7],

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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
fld0: *mut i32,
fld1: [u64; 6],

},
Variant1{
fld0: (bool, *const bool),
fld1: f64,

},
Variant2{
fld0: Adt44,
fld1: Adt45,
fld2: u128,
fld3: [i32; 7],

},
Variant3{
fld0: *mut *const bool,
fld1: Adt40,

}}
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [i32; 7],
fld1: Adt45,
fld2: [u64; 1],

},
Variant1{
fld0: [u64; 1],
fld1: char,
fld2: [i32; 7],
fld3: (u64, [u128; 6], *mut f32, u64, u16),
fld4: Adt48,
fld5: u64,

},
Variant2{
fld0: i64,

}}
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [u8; 4],
fld1: (u128, i128),
fld2: *mut ([u128; 6], i8, f32, u8),
fld3: *mut *const bool,
fld4: i16,

},
Variant1{
fld0: bool,
fld1: Adt43,
fld2: (bool,),
fld3: i8,
fld4: [u8; 4],
fld5: Adt41,
fld6: f64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt49,
fld1: Adt40,
fld2: (u64, [u128; 6], *mut f32, u64, u16),
fld3: u16,
fld4: f32,
fld5: i32,
fld6: Adt45,
fld7: Adt41,

},
Variant1{
fld0: (u128, i128),
fld1: (isize, i64, i32),
fld2: [u128; 6],
fld3: *const bool,
fld4: [i32; 7],
fld5: i32,
fld6: f32,
fld7: Adt42,

},
Variant2{
fld0: i8,
fld1: Adt44,

},
Variant3{
fld0: Adt42,
fld1: *const bool,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt40,
fld1: f64,
fld2: [u64; 1],

},
Variant1{
fld0: u32,
fld1: u16,
fld2: ([u128; 6], i8, f32, u8),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: f32,
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: u8,
fld1: (bool, *const bool),
fld2: [u64; 1],
fld3: Adt45,

},
Variant1{
fld0: Adt41,
fld1: (bool, *const bool),
fld2: (isize, i64, i32),
fld3: Adt46,
fld4: i64,

}}

