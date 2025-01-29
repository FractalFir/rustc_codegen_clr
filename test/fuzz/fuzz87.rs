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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> i64 {
mir! {
type RET = i64;
let _15: f32;
let _16: Adt51;
let _17: [i64; 8];
let _18: (i32,);
let _19: Adt50;
let _20: char;
let _21: [i8; 2];
let _22: [u16; 3];
let _23: f64;
let _24: [usize; 8];
let _25: (usize, i32);
let _26: i16;
let _27: Adt53;
let _28: f32;
let _29: [u128; 6];
let _30: [u128; 6];
let _31: ();
let _32: ();
{
_5 = 26_u8 as i16;
_13 = 6015713808880296711_u64;
Call(RET = fn1(_5, _13, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 29072_u16 + 845_u16;
_15 = _5 as f32;
_9 = !10809239214424071930_usize;
_5 = (-16021_i16) ^ 25068_i16;
RET = (-49_i8) as i64;
_8 = 576740374_i32 as i128;
_3 = !80_isize;
Call(_12 = fn8(_3, _13, _3, _13, RET, _5, _5, _9, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = !184_u8;
_1 = true;
RET = -(-7658547224097202661_i64);
_10 = 124_u8 * 32_u8;
_14 = !138464249126396646110292733938551260118_u128;
_18 = (1814472208_i32,);
_17 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = (-5107969816109794960_i64) >> _14;
_10 = 114_u8 - 10_u8;
_18.0 = -(-455550322_i32);
_12 = !3043322038_u32;
_2 = '\u{a28c2}';
_10 = 248_u8;
_6 = _18.0;
_4 = 119_i8 - 48_i8;
_1 = true ^ false;
_4 = -(-82_i8);
_1 = true;
RET = 2735874926418669019_i64;
_4 = 126_i8 - 36_i8;
_1 = true | false;
Goto(bb3)
}
bb3 = {
_11 = !54453_u16;
_15 = _13 as f32;
_17 = [RET,RET,RET,RET,RET,RET,RET,RET];
_20 = _2;
_18 = (_6,);
_2 = _20;
_12 = 1567028638_u32 | 1744958884_u32;
_3 = (-9223372036854775808_isize);
RET = !8766973630139585703_i64;
_7 = RET & RET;
RET = _7;
_4 = -98_i8;
_10 = _8 as u8;
_15 = _13 as f32;
_9 = !2137274354276020598_usize;
_21 = [_4,_4];
_17 = [RET,_7,RET,_7,_7,RET,RET,RET];
_14 = 180144508047177274422798952583371357116_u128 | 255466920748180149361066036554008206186_u128;
_14 = _2 as u128;
_12 = 2919907132_u32;
_6 = _18.0 ^ _18.0;
_22 = [_11,_11,_11];
_13 = 4285087527680354099_u64 & 13444080647082061189_u64;
match _3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb4 = {
_10 = !184_u8;
_1 = true;
RET = -(-7658547224097202661_i64);
_10 = 124_u8 * 32_u8;
_14 = !138464249126396646110292733938551260118_u128;
_18 = (1814472208_i32,);
_17 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = (-5107969816109794960_i64) >> _14;
_10 = 114_u8 - 10_u8;
_18.0 = -(-455550322_i32);
_12 = !3043322038_u32;
_2 = '\u{a28c2}';
_10 = 248_u8;
_6 = _18.0;
_4 = 119_i8 - 48_i8;
_1 = true ^ false;
_4 = -(-82_i8);
_1 = true;
RET = 2735874926418669019_i64;
_4 = 126_i8 - 36_i8;
_1 = true | false;
Goto(bb3)
}
bb5 = {
_11 = 29072_u16 + 845_u16;
_15 = _5 as f32;
_9 = !10809239214424071930_usize;
_5 = (-16021_i16) ^ 25068_i16;
RET = (-49_i8) as i64;
_8 = 576740374_i32 as i128;
_3 = !80_isize;
Call(_12 = fn8(_3, _13, _3, _13, RET, _5, _5, _9, _9), ReturnTo(bb2), UnwindUnreachable())
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
RET = _7;
_12 = !171205291_u32;
_6 = _18.0 >> _5;
_17 = [RET,_7,RET,RET,RET,_7,RET,RET];
_14 = 178070739739437797198481636925582741414_u128 - 41120635790946120678741023689822285286_u128;
_20 = _2;
_18 = (_6,);
_14 = 80561734003131703460851308190859238648_u128;
RET = !_7;
_8 = -73334368807384059266524436475002973295_i128;
_2 = _20;
match _3 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb10,
340282366920938463454151235394913435648 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_10 = !184_u8;
_1 = true;
RET = -(-7658547224097202661_i64);
_10 = 124_u8 * 32_u8;
_14 = !138464249126396646110292733938551260118_u128;
_18 = (1814472208_i32,);
_17 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = (-5107969816109794960_i64) >> _14;
_10 = 114_u8 - 10_u8;
_18.0 = -(-455550322_i32);
_12 = !3043322038_u32;
_2 = '\u{a28c2}';
_10 = 248_u8;
_6 = _18.0;
_4 = 119_i8 - 48_i8;
_1 = true ^ false;
_4 = -(-82_i8);
_1 = true;
RET = 2735874926418669019_i64;
_4 = 126_i8 - 36_i8;
_1 = true | false;
Goto(bb3)
}
bb12 = {
_9 = 6183358868390496172_usize;
_8 = _12 as i128;
_4 = (-50_i8);
_2 = _20;
_22 = [_11,_11,_11];
_13 = 10451061972555033652_u64 + 1589077848060657850_u64;
_23 = _3 as f64;
_25.1 = _18.0;
_3 = _20 as isize;
_25.0 = _9;
_9 = _25.0;
_1 = _25.1 > _25.1;
_18.0 = _6 + _25.1;
_28 = -_15;
_17 = [_7,RET,_7,RET,RET,RET,RET,_7];
_17 = [_7,RET,_7,RET,RET,RET,_7,RET];
_13 = _11 as u64;
_1 = false;
Goto(bb13)
}
bb13 = {
_26 = !_5;
_24 = [_25.0,_25.0,_25.0,_9,_9,_25.0,_25.0,_9];
_9 = _3 as usize;
_3 = _13 as isize;
_1 = !false;
_18 = (_6,);
_12 = !2898853077_u32;
_8 = -(-36139650228646447917313598740438169812_i128);
_25.1 = _6 ^ _18.0;
_8 = _6 as i128;
_14 = 176887271389734297773106291093065319879_u128 << _26;
_25 = (_9, _6);
_6 = -_18.0;
_20 = _2;
_7 = RET;
Call(_12 = fn9(_6, _25), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = _7 >> _7;
RET = -_7;
_1 = true;
_6 = _18.0;
_3 = 9223372036854775807_isize >> _25.1;
_17 = [_7,RET,_7,RET,RET,_7,_7,_7];
_5 = _26;
_15 = -_28;
_10 = 22_u8 ^ 147_u8;
_25.1 = _6 << _12;
_1 = !true;
_12 = 2883349246_u32;
_15 = -_28;
_29 = [_14,_14,_14,_14,_14,_14];
_18.0 = _25.1;
_3 = (-74_isize) & 9223372036854775807_isize;
_6 = _18.0 - _18.0;
_20 = _2;
_18 = (_6,);
_18 = (_6,);
_22 = [_11,_11,_11];
_20 = _2;
_12 = 936241719_u32 * 2359582535_u32;
RET = _20 as i64;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(0_usize, 13_usize, Move(_13), 4_usize, Move(_4), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(0_usize, 22_usize, Move(_22), 17_usize, Move(_17), 3_usize, Move(_3), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(0_usize, 1_usize, Move(_1), 24_usize, Move(_24), 20_usize, Move(_20), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i16,mut _2: u64,mut _3: u64) -> i64 {
mir! {
type RET = i64;
let _4: f64;
let _5: f64;
let _6: bool;
let _7: i8;
let _8: u8;
let _9: *const usize;
let _10: (([i8; 4],), i128, [u128; 6]);
let _11: f64;
let _12: char;
let _13: bool;
let _14: [i64; 8];
let _15: [usize; 8];
let _16: *mut i16;
let _17: ([i8; 2], *mut u8);
let _18: [i64; 5];
let _19: u64;
let _20: f32;
let _21: u128;
let _22: [i64; 5];
let _23: (i32,);
let _24: i64;
let _25: Adt55;
let _26: bool;
let _27: Adt54;
let _28: [usize; 8];
let _29: *mut u128;
let _30: f64;
let _31: u8;
let _32: u128;
let _33: ();
let _34: ();
{
RET = -7927724284992420457_i64;
RET = 5987952522223423523_i64;
_2 = _3;
_1 = (-27530_i16) << RET;
_2 = '\u{d8cad}' as u64;
RET = (-4764781642585780252_i64);
_4 = 14274667273235377943_usize as f64;
_2 = _3;
_4 = RET as f64;
_1 = 5168496927252692984_usize as i16;
RET = (-906655770829368623_i64) ^ 8777076220757629570_i64;
_5 = _4;
RET = -6396707456009133415_i64;
RET = 216070815468904890726766817220813298415_u128 as i64;
RET = (-5888388693308387839_i64);
_3 = _2;
Goto(bb1)
}
bb1 = {
_2 = !_3;
_4 = _5;
_3 = !_2;
_6 = true & false;
_6 = false;
_1 = 28743_i16;
match _1 {
0 => bb2,
28743 => bb4,
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
RET = !8489616603982694473_i64;
_3 = 44_i8 as u64;
Call(RET = fn2(_6, _2, _1, _4, _5, _6, _1, _1, _1, _3, _5, _1, _5, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = 2782625985_u32 as i8;
_1 = !(-9346_i16);
_8 = 63_u8 + 43_u8;
_8 = 130_u8;
_3 = _2 >> _2;
_7 = 92_i8;
_6 = true;
_6 = !false;
RET = 4313960377088481939_i64;
_6 = !true;
_2 = _7 as u64;
_2 = _4 as u64;
_8 = 94_u8;
_3 = !_2;
_5 = _4;
_1 = -29976_i16;
RET = 579114661379116257_i64;
_10.1 = 95887266597393426101304249732993153834_i128 << _7;
RET = _6 as i64;
RET = (-7095804763782007487_i64);
_10.2 = [49166559656308740803142854928145975124_u128,260924068960137232150801266219843029512_u128,333098259175980760078547765006922581701_u128,17716315578306912190977534563409106812_u128,231763538219467587281139597045792597460_u128,321514772883197252800689346169214176887_u128];
Goto(bb6)
}
bb6 = {
_8 = !203_u8;
_1 = 39_isize as i16;
_8 = 31_u8 - 98_u8;
_1 = (-341_i16);
_11 = -_5;
_12 = '\u{f5c3d}';
RET = (-2736273149309698867_i64);
_5 = -_4;
_10.0.0 = [_7,_7,_7,_7];
_6 = false & false;
_6 = !true;
_10.1 = (-154757430235915170703080421978300488204_i128) * (-93350282840914900680299568493403291506_i128);
_10.1 = _8 as i128;
_8 = 6455870681486198400_usize as u8;
_7 = !63_i8;
_13 = !_6;
_1 = (-30876_i16);
_3 = _2 * _2;
Goto(bb7)
}
bb7 = {
_2 = _3 ^ _3;
_16 = core::ptr::addr_of_mut!(_1);
Call(_10.2 = fn4(_2, _1, _2, _2, _10.0.0, _8, _2, _12, _16, _1, _13, _2, (*_16)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = _4 * _4;
_15 = [16209601451180832132_usize,12892724456102083318_usize,6_usize,11853226072795151039_usize,11152242984183303040_usize,0_usize,15408633515205903322_usize,14520213189376624653_usize];
_7 = (-105_i8) + 111_i8;
_14 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = 2025958010_u32 as i8;
_13 = _6 & _6;
(*_16) = -32247_i16;
_11 = _5;
(*_16) = _12 as i16;
RET = 4738545552032438178_i64;
_3 = _2 >> _7;
_12 = '\u{a8a4b}';
_17.0 = [_7,_7];
_15 = [0_usize,2_usize,6727427464242793607_usize,2_usize,4_usize,11739895073747344792_usize,2_usize,4_usize];
_14 = [RET,RET,RET,RET,RET,RET,RET,RET];
_10.0.0 = [_7,_7,_7,_7];
_8 = !13_u8;
_5 = _11;
_6 = _11 != _11;
(*_16) = (-15941_i16);
Goto(bb9)
}
bb9 = {
_4 = _5 - _11;
_12 = '\u{4f89e}';
_5 = -_11;
_6 = _4 < _5;
_17.1 = core::ptr::addr_of_mut!(_8);
_10.1 = (-1393365401_i32) as i128;
_10.2 = [18061822039305250391143727212010022721_u128,88821696039718419989961219522535451972_u128,8306176967171494297408121419124776967_u128,2795891247289535677932145075156169728_u128,262075801054916419637354362910795157694_u128,147094501261704069735068772532412760917_u128];
_3 = _2;
RET = (-4304245756950959814_i64);
_15 = [2_usize,4366014061475954590_usize,4_usize,5_usize,12356668233075297073_usize,4_usize,3_usize,16771253026208608530_usize];
RET = -(-4367161819630763202_i64);
Goto(bb10)
}
bb10 = {
_13 = _6;
_3 = _2 >> _8;
(*_16) = RET as i16;
_2 = _3;
_8 = 107_u8 >> _3;
_12 = '\u{fb38c}';
(*_16) = (-836_i16) | 10842_i16;
_18 = [RET,RET,RET,RET,RET];
_4 = _11 + _5;
_12 = '\u{b31cf}';
_23 = ((-933004660_i32),);
_11 = _4;
_7 = (-109_i8) & (-68_i8);
_8 = 21_u8;
_22 = _18;
_11 = 33476054703598331879450749617138758766_u128 as f64;
_1 = _13 as i16;
RET = _13 as i64;
_21 = !101928208224344879360710701400140995095_u128;
_20 = _8 as f32;
_17.1 = core::ptr::addr_of_mut!(_8);
_19 = !_3;
_25.fld4.fld4.fld0 = [_7,_7];
_25.fld4.fld0 = [_12,_12,_12,_12];
Goto(bb11)
}
bb11 = {
_25.fld4.fld7 = _11 + _5;
_25.fld4.fld6.fld1 = [_23.0,_23.0,_23.0,_23.0,_23.0,_23.0];
_25.fld4.fld6.fld0 = _2 as i32;
_25.fld4.fld3.1 = core::ptr::addr_of_mut!(_1);
_25.fld4.fld4.fld4 = _17.1;
_25.fld4.fld6.fld4 = (0_usize, _25.fld4.fld6.fld0);
_25.fld4.fld4.fld2 = (-9223372036854775808_isize);
_25.fld4.fld6.fld0 = _25.fld4.fld6.fld4.1;
_25.fld2 = -_25.fld4.fld6.fld4.1;
_24 = _20 as i64;
(*_16) = 3085_i16;
_7 = 111_i8 | (-63_i8);
_25.fld4.fld6.fld2 = core::ptr::addr_of_mut!(_21);
_25.fld3 = _19 as i8;
_25.fld4.fld6.fld0 = _25.fld4.fld6.fld4.1 - _23.0;
_17.0 = [_25.fld3,_25.fld3];
_25.fld4.fld4.fld0 = [_25.fld3,_25.fld3];
_25.fld3 = _7;
_21 = 210511319407539816364651125925422609136_u128 << _19;
_9 = core::ptr::addr_of!(_25.fld4.fld6.fld4.0);
_19 = !_2;
_25.fld4.fld5 = [65019_u16,35234_u16,14975_u16];
_25.fld4.fld6.fld3 = Adt41::Variant0 { fld0: _25.fld4.fld6.fld4.0,fld1: _25.fld4.fld6.fld4.1,fld2: _14,fld3: _9 };
place!(Field::<usize>(Variant(_25.fld4.fld6.fld3, 0), 0)) = !(*_9);
_15 = [(*_9),(*_9),(*_9),(*_9),_25.fld4.fld6.fld4.0,Field::<usize>(Variant(_25.fld4.fld6.fld3, 0), 0),(*_9),(*_9)];
Goto(bb12)
}
bb12 = {
_25.fld4.fld1 = !_2;
(*_9) = Field::<usize>(Variant(_25.fld4.fld6.fld3, 0), 0) - Field::<usize>(Variant(_25.fld4.fld6.fld3, 0), 0);
_19 = _3 * _3;
_25.fld4.fld4.fld0 = _17.0;
_24 = RET | RET;
_10.1 = (-11338816100583821631093564220738630147_i128) | 66671104235498753766407381936760116986_i128;
_25.fld4.fld4.fld4 = core::ptr::addr_of_mut!(_8);
_12 = '\u{18f43}';
_25.fld4.fld7 = -_11;
_16 = core::ptr::addr_of_mut!((*_16));
_8 = _12 as u8;
_25.fld4.fld7 = _4;
_25.fld3 = _7;
_25.fld4.fld4.fld5 = core::ptr::addr_of_mut!(_1);
_10.2 = [_21,_21,_21,_21,_21,_21];
_9 = core::ptr::addr_of!(_25.fld4.fld6.fld4.0);
place!(Field::<usize>(Variant(_25.fld4.fld6.fld3, 0), 0)) = !(*_9);
_25.fld4.fld5 = [50502_u16,20294_u16,12850_u16];
_9 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_25.fld4.fld6.fld3, 0), 0)));
place!(Field::<*const usize>(Variant(_25.fld4.fld6.fld3, 0), 3)) = core::ptr::addr_of!((*_9));
match _25.fld4.fld4.fld2 {
0 => bb5,
1 => bb2,
2 => bb11,
3 => bb10,
4 => bb13,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb13 = {
_8 = !203_u8;
_1 = 39_isize as i16;
_8 = 31_u8 - 98_u8;
_1 = (-341_i16);
_11 = -_5;
_12 = '\u{f5c3d}';
RET = (-2736273149309698867_i64);
_5 = -_4;
_10.0.0 = [_7,_7,_7,_7];
_6 = false & false;
_6 = !true;
_10.1 = (-154757430235915170703080421978300488204_i128) * (-93350282840914900680299568493403291506_i128);
_10.1 = _8 as i128;
_8 = 6455870681486198400_usize as u8;
_7 = !63_i8;
_13 = !_6;
_1 = (-30876_i16);
_3 = _2 * _2;
Goto(bb7)
}
bb14 = {
Return()
}
bb15 = {
_25.fld0 = _18;
_20 = Field::<i32>(Variant(_25.fld4.fld6.fld3, 0), 1) as f32;
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(1_usize, 7_usize, Move(_7), 24_usize, Move(_24), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(1_usize, 10_usize, Move(_10), 23_usize, Move(_23), 18_usize, Move(_18), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: u64,mut _3: i16,mut _4: f64,mut _5: f64,mut _6: bool,mut _7: i16,mut _8: i16,mut _9: i16,mut _10: u64,mut _11: f64,mut _12: i16,mut _13: f64,mut _14: i16) -> i64 {
mir! {
type RET = i64;
let _15: char;
let _16: *mut u8;
let _17: [u128; 6];
let _18: Adt44;
let _19: bool;
let _20: u64;
let _21: isize;
let _22: *mut u8;
let _23: f32;
let _24: *const usize;
let _25: i32;
let _26: (([i8; 4],), i128, [u128; 6]);
let _27: f32;
let _28: bool;
let _29: i32;
let _30: isize;
let _31: u128;
let _32: isize;
let _33: [i64; 8];
let _34: f64;
let _35: [i8; 4];
let _36: bool;
let _37: (([i8; 4],), i128, [u128; 6]);
let _38: bool;
let _39: (*mut u32, *mut i16);
let _40: f64;
let _41: (i32,);
let _42: char;
let _43: f64;
let _44: f32;
let _45: [usize; 8];
let _46: i8;
let _47: i8;
let _48: u16;
let _49: (i32,);
let _50: ([i8; 2], *mut u8);
let _51: char;
let _52: char;
let _53: char;
let _54: (i32,);
let _55: ();
let _56: ();
{
_3 = _8;
_12 = -_14;
_13 = 51236_u16 as f64;
RET = 36_u8 as i64;
_11 = -_13;
_2 = _10 & _10;
_12 = _9;
RET = _12 as i64;
_6 = _1;
_15 = '\u{930bf}';
RET = (-99_i8) as i64;
_4 = RET as f64;
_3 = !_12;
_11 = _13 + _5;
_15 = '\u{cfdf}';
_17 = [328217412903028538904865309163822146257_u128,180948391683972750782090963674462226130_u128,14525520644677423389972722201565845752_u128,310711470533584985531845703579595383522_u128,25216189360156541514130783356578133191_u128,199794693155942799329325709041493594503_u128];
_8 = _7;
_10 = _2 & _2;
_2 = _6 as u64;
_2 = _10;
RET = 447688932_u32 as i64;
_11 = _13;
_9 = !_3;
_13 = _4 * _5;
RET = -(-2370929862573316111_i64);
match _7 {
28743 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_11 = _13;
_12 = _4 as i16;
_9 = -_3;
_5 = -_13;
_6 = RET == RET;
_8 = !_9;
_12 = _13 as i16;
_1 = _6;
_7 = -_3;
_13 = _11;
_6 = _1 ^ _1;
RET = 857026531900298009_i64;
_8 = -_3;
_1 = _6 & _6;
_2 = !_10;
_10 = _2;
_20 = _11 as u64;
_2 = _15 as u64;
_8 = _14;
_8 = _3;
RET = !5281709080923013171_i64;
_11 = 194_u8 as f64;
match _14 {
0 => bb1,
28743 => bb4,
_ => bb3
}
}
bb3 = {
Return()
}
bb4 = {
_11 = -_5;
_12 = !_9;
_23 = 3896616798_u32 as f32;
_23 = (-9223372036854775808_isize) as f32;
_13 = -_11;
_9 = _7;
_26.1 = 897404535558441524500347018898427341_i128 + (-99806818666385067110287457234687308861_i128);
_21 = -97_isize;
_26.2 = [109724458407603557424102908354485360067_u128,237850406814265149089713489064034544431_u128,213402011252556820671202836643791155928_u128,286189634069256779078042746912727146965_u128,320910727887565848985650286644399676634_u128,130477690321389105803792512662772316243_u128];
_17 = [160258652212594052949628146730945469596_u128,240535426481747584362198216673804174795_u128,18722842153898059454608616811156876373_u128,257158707589489346471983159320662821509_u128,222139319792846168935742048388918333987_u128,62492077278678111200219846799409019801_u128];
_26.1 = _11 as i128;
_4 = _5;
_14 = _3 ^ _8;
_20 = _10;
Goto(bb5)
}
bb5 = {
_27 = -_23;
_6 = _1;
_26.0.0 = [(-75_i8),5_i8,12_i8,104_i8];
_20 = _10 - _10;
_11 = -_13;
_14 = _9;
_25 = !(-2031590977_i32);
_20 = _15 as u64;
_10 = _2 >> _26.1;
_27 = _23 - _23;
_12 = _3 >> _14;
_26.1 = 78823140541418193724389676084129449430_i128;
_23 = -_27;
_19 = _6 != _1;
RET = (-564853658754614099_i64) ^ (-3801038841579921983_i64);
RET = -6781887151436520184_i64;
_17 = [50971306035898675519515769786412686559_u128,87851848229796568092009065401124593750_u128,100836409199560542445517643225128686493_u128,302162867754390771223241561477016415158_u128,151840871632960342467828252066495789159_u128,335823756315752548580180450696036149277_u128];
_26.2 = [277732401160543993079870450188337288633_u128,209515954025698400052190928041217593345_u128,286734811800561304960546621741056937423_u128,49288948265121044094013554267442521345_u128,134072535848808790470376313777304162638_u128,99249480313796957488855310219930020006_u128];
_12 = _7 ^ _3;
_3 = !_9;
_2 = !_10;
_21 = _13 as isize;
_13 = -_5;
_17 = [243339880781393664331044456385621769557_u128,30908185388227820428122144336903113169_u128,321965797686453019227039003281285438394_u128,231611474174820212538122960980145897440_u128,85063992548362663612733477465174391574_u128,123974715752300555601798734012090609878_u128];
_28 = _6;
_1 = !_19;
_14 = _7 & _8;
RET = 3811147845249126821_i64 ^ (-4852065579632766465_i64);
_8 = _5 as i16;
Call(_8 = fn3(_1, _17, _7, _1, _19, _20, _23, _26, _26), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26.0.0 = [56_i8,124_i8,50_i8,35_i8];
_29 = _25 * _25;
_3 = _14 | _7;
_14 = _3;
_13 = _4 * _5;
_31 = 187999600193550620469669418836579581444_u128;
_26.1 = 37638352815320316760842198235797519719_i128 + (-148520981686052723607482708701929718901_i128);
_21 = (-19_isize);
_20 = _31 as u64;
_8 = _26.1 as i16;
RET = 5654892071755536487_i64;
_7 = 122_u8 as i16;
_12 = -_7;
_7 = -_8;
_3 = _9 + _14;
_4 = _11 - _5;
_21 = 119_isize;
_29 = !_25;
_37.1 = _27 as i128;
_36 = _19;
match _31 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
187999600193550620469669418836579581444 => bb10,
_ => bb9
}
}
bb7 = {
Return()
}
bb8 = {
_11 = -_5;
_12 = !_9;
_23 = 3896616798_u32 as f32;
_23 = (-9223372036854775808_isize) as f32;
_13 = -_11;
_9 = _7;
_26.1 = 897404535558441524500347018898427341_i128 + (-99806818666385067110287457234687308861_i128);
_21 = -97_isize;
_26.2 = [109724458407603557424102908354485360067_u128,237850406814265149089713489064034544431_u128,213402011252556820671202836643791155928_u128,286189634069256779078042746912727146965_u128,320910727887565848985650286644399676634_u128,130477690321389105803792512662772316243_u128];
_17 = [160258652212594052949628146730945469596_u128,240535426481747584362198216673804174795_u128,18722842153898059454608616811156876373_u128,257158707589489346471983159320662821509_u128,222139319792846168935742048388918333987_u128,62492077278678111200219846799409019801_u128];
_26.1 = _11 as i128;
_4 = _5;
_14 = _3 ^ _8;
_20 = _10;
Goto(bb5)
}
bb9 = {
_11 = _13;
_12 = _4 as i16;
_9 = -_3;
_5 = -_13;
_6 = RET == RET;
_8 = !_9;
_12 = _13 as i16;
_1 = _6;
_7 = -_3;
_13 = _11;
_6 = _1 ^ _1;
RET = 857026531900298009_i64;
_8 = -_3;
_1 = _6 & _6;
_2 = !_10;
_10 = _2;
_20 = _11 as u64;
_2 = _15 as u64;
_8 = _14;
_8 = _3;
RET = !5281709080923013171_i64;
_11 = 194_u8 as f64;
match _14 {
0 => bb1,
28743 => bb4,
_ => bb3
}
}
bb10 = {
_20 = _2;
_7 = _14;
_37 = _26;
_37.1 = 3063303366_u32 as i128;
_35 = [(-128_i8),(-117_i8),124_i8,85_i8];
_9 = _8 + _7;
_37.2 = [_31,_31,_31,_31,_31,_31];
_27 = _23 - _23;
_27 = -_23;
_39.1 = core::ptr::addr_of_mut!(_9);
_30 = _21 + _21;
_34 = _5;
_37 = (_26.0, _26.1, _26.2);
_37 = (_26.0, _26.1, _26.2);
_25 = -_29;
_41.0 = _25 & _29;
_36 = !_19;
_26.1 = !_37.1;
_26.0 = _37.0;
Goto(bb11)
}
bb11 = {
RET = 3048293083787832589_i64;
_4 = _13;
Goto(bb12)
}
bb12 = {
_19 = _1;
_26.0 = (_37.0.0,);
_1 = !_19;
_13 = -_34;
_31 = !120151581178219347141517037367230558886_u128;
_10 = _20 + _20;
_43 = _4 - _13;
_28 = !_36;
_27 = _23;
_28 = _19;
_30 = _21;
_26.2 = _37.2;
_23 = _27 - _27;
_28 = _19;
_40 = -_5;
_37.2 = _26.2;
_33 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = _4;
_15 = '\u{fb800}';
_42 = _15;
match RET {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb9,
3048293083787832589 => bb13,
_ => bb5
}
}
bb13 = {
_21 = !_30;
_32 = _21;
RET = _7 as i64;
_47 = !60_i8;
_45 = [13022399226061341416_usize,2827599409526386618_usize,2730594102167074422_usize,6669699294806756296_usize,15113783541343733661_usize,11546526375855965109_usize,6_usize,2434423870468072073_usize];
_38 = _1 | _36;
_32 = 61807_u16 as isize;
_25 = _31 as i32;
_33 = [RET,RET,RET,RET,RET,RET,RET,RET];
_42 = _15;
_17 = _37.2;
_41.0 = _31 as i32;
_30 = -_21;
_41.0 = _25;
_14 = _9 - _9;
RET = !4642451579719076869_i64;
_34 = _40;
_28 = !_1;
_37.0.0 = [_47,_47,_47,_47];
_1 = !_36;
_29 = _25 << _14;
_20 = _2 >> _14;
_46 = _31 as i8;
Goto(bb14)
}
bb14 = {
_11 = _5 * _5;
_37.0 = _26.0;
_23 = _20 as f32;
_35 = [_46,_46,_47,_46];
_44 = _23 * _23;
_9 = 64314_u16 as i16;
_37.1 = _4 as i128;
_15 = '\u{fb800}';
_53 = _15;
_51 = _42;
_36 = _6 > _1;
_35 = [_46,_46,_46,_47];
_26.0.0 = _37.0.0;
_25 = _29;
_1 = _38;
_30 = !_21;
Goto(bb15)
}
bb15 = {
Call(_55 = dump_var(2_usize, 31_usize, Move(_31), 26_usize, Move(_26), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_55 = dump_var(2_usize, 37_usize, Move(_37), 53_usize, Move(_53), 47_usize, Move(_47), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_55 = dump_var(2_usize, 20_usize, Move(_20), 2_usize, Move(_2), 28_usize, Move(_28), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_55 = dump_var(2_usize, 36_usize, Move(_36), 29_usize, Move(_29), 3_usize, Move(_3), 45_usize, Move(_45)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_55 = dump_var(2_usize, 32_usize, Move(_32), 15_usize, _15, 4_usize, _4, 56_usize, _56), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: [u128; 6],mut _3: i16,mut _4: bool,mut _5: bool,mut _6: u64,mut _7: f32,mut _8: (([i8; 4],), i128, [u128; 6]),mut _9: (([i8; 4],), i128, [u128; 6])) -> i16 {
mir! {
type RET = i16;
let _10: f32;
let _11: i128;
let _12: bool;
let _13: Adt45;
let _14: [usize; 8];
let _15: Adt51;
let _16: [i64; 5];
let _17: Adt45;
let _18: i8;
let _19: (usize, i32);
let _20: (usize, i32);
let _21: [i8; 2];
let _22: i8;
let _23: [char; 4];
let _24: [i32; 6];
let _25: [u16; 3];
let _26: usize;
let _27: [i64; 8];
let _28: ([i8; 4],);
let _29: *mut u8;
let _30: isize;
let _31: u64;
let _32: [i8; 4];
let _33: u64;
let _34: [char; 4];
let _35: Adt43;
let _36: Adt47;
let _37: [usize; 8];
let _38: f64;
let _39: Adt47;
let _40: Adt56;
let _41: isize;
let _42: ();
let _43: ();
{
_6 = !17919531801699551602_u64;
_2 = _9.2;
RET = _9.1 as i16;
_10 = 1_usize as f32;
_9.0.0 = [(-99_i8),99_i8,78_i8,103_i8];
_7 = -_10;
_1 = _5 ^ _5;
_8.1 = -_9.1;
_9.0 = (_8.0.0,);
RET = _5 as i16;
_7 = _10;
_9.0.0 = [38_i8,38_i8,39_i8,(-125_i8)];
_9.0 = _8.0;
_6 = 13414745530483248770_u64 & 13224474521887534038_u64;
_12 = _1 <= _1;
RET = (-9223372036854775808_isize) as i16;
_8.2 = [296007148036685830605232699069898240353_u128,75299040715113689419134486225134130428_u128,192255312219650413258135467192134237317_u128,99494809639946131440104914574314403299_u128,146678087405859210445159547384526476693_u128,157915189946231272564991975829805728566_u128];
_14 = [4855454668525757030_usize,7108861410480833031_usize,7593446397137640322_usize,3_usize,3_usize,3811751876794311037_usize,1_usize,6258377278209196219_usize];
_8.0.0 = [1_i8,29_i8,(-19_i8),(-15_i8)];
_5 = !_12;
_14 = [1260297389239235922_usize,4_usize,1728175146042200740_usize,14591644918773973739_usize,8553700708053771197_usize,10867848969347386909_usize,7_usize,3_usize];
_8.2 = [126215928630199515005987462310044908135_u128,157106559909604977796529528083155834598_u128,111417824176682694234605266932651764953_u128,48609822637885328980883907226979512624_u128,24286401197638077303468696585144572493_u128,249999280533655626115370599223407736606_u128];
_9.2 = _8.2;
_9.2 = [84074559482595544480011880061680385933_u128,94773024974574965971491712746365082850_u128,237513886954575870141360413285261704046_u128,165637474042393285465360055514786672739_u128,48024921777354969635393751015391554750_u128,267662267140623590290706258861818873947_u128];
Goto(bb1)
}
bb1 = {
_11 = _8.1;
_4 = _12 & _1;
_8.0.0 = [83_i8,13_i8,(-103_i8),(-27_i8)];
_9.1 = _11 + _11;
_9.0 = (_8.0.0,);
_2 = [152353833234241395527691911873546591687_u128,281304529584869641362054476595488380881_u128,331382071912500689471729453991039577248_u128,18908008447207867861879317515571118572_u128,191805090838716356548850031483585527454_u128,314218808525576408970282257853775566150_u128];
_8 = (_9.0, _9.1, _9.2);
_8.2 = [210946899578730041818908357066145692663_u128,62572508717913226649485784307274035596_u128,2671299209555599154325048652212698322_u128,53821706621372266444663723158739852909_u128,294321350008712567200217368949641322803_u128,3833599079172288207726054933133244234_u128];
_9.2 = _2;
_19 = (1235712767982620065_usize, (-1145951325_i32));
_9.0 = (_8.0.0,);
_19.0 = 5_usize;
_10 = 45868_u16 as f32;
_4 = _12;
match _19.1 {
340282366920938463463374607430622260131 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_4 = _5;
_9 = (_8.0, _11, _2);
_1 = !_5;
_20.0 = _19.0 % _19.0;
_20.0 = !_19.0;
RET = '\u{6502e}' as i16;
_9 = _8;
_9.2 = _8.2;
_9.1 = (-7157881874399803784_i64) as i128;
_19 = (_20.0, (-1122070263_i32));
_11 = -_9.1;
_16 = [565045878061434299_i64,4351778394660483259_i64,740974238707637624_i64,5059161776412532203_i64,4698421581703465660_i64];
RET = _3 ^ _3;
match _19.1 {
0 => bb1,
1 => bb2,
340282366920938463463374607430646141193 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
_16 = [(-6246452220599255450_i64),(-2791271317395811851_i64),(-8336432495662073353_i64),3613447875824178377_i64,2628733022068639884_i64];
_16 = [(-4957363212943352335_i64),1361914980815751442_i64,(-3752629848212639242_i64),(-4923880644645451141_i64),(-5020083908679071103_i64)];
_19.0 = 220493808521699980116125616575030287874_u128 as usize;
_18 = !75_i8;
_22 = _18;
_3 = _8.1 as i16;
_21 = [_22,_22];
_6 = _7 as u64;
_18 = '\u{5ed8}' as i8;
_20.1 = _19.1 << _8.1;
_24 = [_20.1,_20.1,_20.1,_20.1,_20.1,_20.1];
Goto(bb6)
}
bb6 = {
_6 = _7 as u64;
_1 = _5 == _5;
_8.2 = _9.2;
_7 = _10;
_9 = (_8.0, _11, _2);
_18 = 1328713120_u32 as i8;
_10 = _7 * _7;
_8.0 = (_9.0.0,);
_28 = (_9.0.0,);
_11 = _8.1 - _9.1;
RET = -_3;
_26 = _19.0;
_18 = _22 | _22;
match _19.1 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607430646141193 => bb11,
_ => bb10
}
}
bb7 = {
_16 = [(-6246452220599255450_i64),(-2791271317395811851_i64),(-8336432495662073353_i64),3613447875824178377_i64,2628733022068639884_i64];
_16 = [(-4957363212943352335_i64),1361914980815751442_i64,(-3752629848212639242_i64),(-4923880644645451141_i64),(-5020083908679071103_i64)];
_19.0 = 220493808521699980116125616575030287874_u128 as usize;
_18 = !75_i8;
_22 = _18;
_3 = _8.1 as i16;
_21 = [_22,_22];
_6 = _7 as u64;
_18 = '\u{5ed8}' as i8;
_20.1 = _19.1 << _8.1;
_24 = [_20.1,_20.1,_20.1,_20.1,_20.1,_20.1];
Goto(bb6)
}
bb8 = {
_11 = _8.1;
_4 = _12 & _1;
_8.0.0 = [83_i8,13_i8,(-103_i8),(-27_i8)];
_9.1 = _11 + _11;
_9.0 = (_8.0.0,);
_2 = [152353833234241395527691911873546591687_u128,281304529584869641362054476595488380881_u128,331382071912500689471729453991039577248_u128,18908008447207867861879317515571118572_u128,191805090838716356548850031483585527454_u128,314218808525576408970282257853775566150_u128];
_8 = (_9.0, _9.1, _9.2);
_8.2 = [210946899578730041818908357066145692663_u128,62572508717913226649485784307274035596_u128,2671299209555599154325048652212698322_u128,53821706621372266444663723158739852909_u128,294321350008712567200217368949641322803_u128,3833599079172288207726054933133244234_u128];
_9.2 = _2;
_19 = (1235712767982620065_usize, (-1145951325_i32));
_9.0 = (_8.0.0,);
_19.0 = 5_usize;
_10 = 45868_u16 as f32;
_4 = _12;
match _19.1 {
340282366920938463463374607430622260131 => bb3,
_ => bb2
}
}
bb9 = {
_4 = _5;
_9 = (_8.0, _11, _2);
_1 = !_5;
_20.0 = _19.0 % _19.0;
_20.0 = !_19.0;
RET = '\u{6502e}' as i16;
_9 = _8;
_9.2 = _8.2;
_9.1 = (-7157881874399803784_i64) as i128;
_19 = (_20.0, (-1122070263_i32));
_11 = -_9.1;
_16 = [565045878061434299_i64,4351778394660483259_i64,740974238707637624_i64,5059161776412532203_i64,4698421581703465660_i64];
RET = _3 ^ _3;
match _19.1 {
0 => bb1,
1 => bb2,
340282366920938463463374607430646141193 => bb5,
_ => bb4
}
}
bb10 = {
Return()
}
bb11 = {
RET = _3;
_9.1 = 3711623598_u32 as i128;
_18 = _22;
_26 = _19.0;
_20 = (_26, _19.1);
_8.1 = _11;
_26 = _20.1 as usize;
_9.2 = [25756000488404920595021191106788505432_u128,137184970609084340338141000255190103618_u128,178728904358392113827695420372066160003_u128,126217822437726874069441971797326607130_u128,148648745700455762073313647512432789036_u128,108569434967840083886256014335378399111_u128];
_28.0 = _9.0.0;
_30 = !(-107_isize);
_19.0 = 88_u8 as usize;
_11 = -_8.1;
_8.0 = _28;
_31 = _6;
_32 = _8.0.0;
_8.1 = _30 as i128;
_19.1 = !_20.1;
_12 = _1;
_34 = ['\u{e1cb6}','\u{7a3a8}','\u{9522b}','\u{37770}'];
_6 = _31;
_6 = _31 * _31;
_7 = _10 * _10;
_20.1 = _19.1;
Goto(bb12)
}
bb12 = {
RET = _3;
_35.fld2 = _30;
_28.0 = [_18,_18,_18,_18];
_27 = [7247843901835986405_i64,(-799457805716450852_i64),(-4490215366019593854_i64),(-1610820527740250474_i64),1107389426653109673_i64,(-2198666078921018051_i64),156140809387621703_i64,8396950661428012444_i64];
Goto(bb13)
}
bb13 = {
_35.fld3 = Adt42::Variant2 { fld0: _14,fld1: (-5980974376937824203_i64) };
_2 = [305400443139471378726003042660333702247_u128,175641603597964772274862363377797231455_u128,66367320419971593922243580204487291246_u128,134409087035305942099488337655244819608_u128,222890561296910073035037170574925550171_u128,321032978945296373115835373387535514767_u128];
_35.fld2 = _30;
_22 = _20.1 as i8;
_33 = _6;
_4 = _5 < _12;
_3 = -RET;
_25 = [8284_u16,39386_u16,39707_u16];
_12 = _5 <= _4;
_19.0 = !_26;
_9.2 = [130037719581586102489768646037578885331_u128,166675397299694592292471800214416519319_u128,31800639549269937914991712023603539468_u128,278733446775057758724531504485055640831_u128,23635638515575225803661637265288003970_u128,177118244313049478554450886352791894699_u128];
_6 = _18 as u64;
_23 = ['\u{f0a3f}','\u{c60c}','\u{be357}','\u{be4b8}'];
_37 = [_19.0,_26,_19.0,_19.0,_26,_26,_26,_19.0];
_3 = RET << _19.0;
_32 = [_22,_22,_22,_22];
_14 = [_20.0,_19.0,_19.0,_19.0,_26,_26,_20.0,_19.0];
_20.1 = -_19.1;
_19 = (_20.0, _20.1);
_27 = [94260728406286760_i64,(-3447119787272411597_i64),(-7969059521017337627_i64),(-1111609586983365168_i64),(-8874619961405048158_i64),(-3517025791989285366_i64),(-4849705593395283670_i64),4979128482340589215_i64];
place!(Field::<[usize; 8]>(Variant(_35.fld3, 2), 0)) = [_26,_19.0,_19.0,_26,_26,_19.0,_26,_20.0];
Goto(bb14)
}
bb14 = {
place!(Field::<i64>(Variant(_35.fld3, 2), 1)) = (-7165313019169934537_i64);
_20.1 = _19.1;
_1 = _4 & _12;
_8.0.0 = [_18,_22,_22,_22];
_18 = _22;
_8.0.0 = _9.0.0;
place!(Field::<i64>(Variant(_35.fld3, 2), 1)) = !9215117321691320623_i64;
_35.fld0 = _21;
_12 = !_5;
_32 = [_18,_18,_22,_18];
_32 = _9.0.0;
_41 = _30;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(3_usize, 18_usize, Move(_18), 4_usize, Move(_4), 41_usize, Move(_41), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(3_usize, 30_usize, Move(_30), 21_usize, Move(_21), 32_usize, Move(_32), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(3_usize, 27_usize, Move(_27), 14_usize, Move(_14), 1_usize, Move(_1), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(3_usize, 22_usize, Move(_22), 3_usize, Move(_3), 11_usize, Move(_11), 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u64,mut _2: i16,mut _3: u64,mut _4: u64,mut _5: [i8; 4],mut _6: u8,mut _7: u64,mut _8: char,mut _9: *mut i16,mut _10: i16,mut _11: bool,mut _12: u64,mut _13: i16) -> [u128; 6] {
mir! {
type RET = [u128; 6];
let _14: [char; 4];
let _15: isize;
let _16: i32;
let _17: (usize, i32);
let _18: Adt46;
let _19: (([i8; 4],), i128, [u128; 6]);
let _20: f32;
let _21: [i64; 5];
let _22: u16;
let _23: isize;
let _24: Adt53;
let _25: (*mut u32, *mut i16);
let _26: char;
let _27: [bool; 5];
let _28: *mut u8;
let _29: ([i8; 4],);
let _30: Adt54;
let _31: *const usize;
let _32: i128;
let _33: i8;
let _34: *mut i16;
let _35: [i64; 5];
let _36: *mut u8;
let _37: isize;
let _38: [u128; 6];
let _39: i32;
let _40: i32;
let _41: ();
let _42: ();
{
_2 = -(*_9);
_11 = true;
Goto(bb1)
}
bb1 = {
_14 = [_8,_8,_8,_8];
_12 = !_3;
_7 = _1;
_11 = false;
_6 = (-106_isize) as u8;
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768180580 => bb7,
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
_4 = !_3;
_16 = (-680981895_i32) ^ 660410588_i32;
_2 = _13;
RET = [264413801915643102081449615434903529737_u128,176970650745032817772095511848787198488_u128,215938606914603126576789583713690286308_u128,290793749247542851042022036159254430541_u128,320336377537267266127804791346461071162_u128,245513282459064688528712227566393388562_u128];
_5 = [(-126_i8),(-87_i8),30_i8,61_i8];
_11 = false;
_11 = !false;
Call(_12 = fn5(_8, _11, _16, _2, _10, _4, _2, _6, _1, _13, _4, (*_9), _7, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11 = true;
_16 = 1961642880_i32;
_12 = !_4;
_17.0 = !3_usize;
_10 = !(*_9);
_9 = core::ptr::addr_of_mut!((*_9));
_15 = 2394019553_u32 as isize;
_17.1 = _16 & _16;
_14 = [_8,_8,_8,_8];
RET = [218864677909486082367146229484863205106_u128,31269067558649583082116312839179451600_u128,280469107058056882185472218544370808834_u128,228026694511251431293484884457527241314_u128,189077625115245370700639830340422226647_u128,313886019193995009350463122544798499889_u128];
_9 = core::ptr::addr_of_mut!((*_9));
_11 = true;
_12 = !_1;
_11 = (*_9) < (*_9);
_19.0.0 = _5;
_22 = 24043_u16 * 1578_u16;
_18.fld1 = [_17.1,_17.1,_17.1,_16,_17.1,_17.1];
_19.2 = [28243390984040053951814472449120834250_u128,117063084636933191717169643200314302029_u128,129987058825449331551849694712982962026_u128,20452945194470232636704537094867391837_u128,85664040456695370069850674371219473759_u128,237523126282693450307781805287649013908_u128];
_19.0 = (_5,);
_17 = (14720254177163774269_usize, _16);
Call(_4 = core::intrinsics::bswap(_7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_19.0 = (_5,);
_4 = _3;
_18.fld4 = (_17.0, _17.1);
_10 = -(*_9);
_23 = 1849614278_u32 as isize;
_17.1 = !_16;
_18.fld0 = _18.fld4.1 | _18.fld4.1;
_25.1 = _9;
Goto(bb10)
}
bb10 = {
_1 = !_7;
_17 = _18.fld4;
_19.2 = [324521588324543238336393168683880612222_u128,249890193321962636932036038769109681891_u128,12215854747210008922422136352522627084_u128,216961973283405986062773691941073705778_u128,300687229511265721733856242527697504579_u128,15596458190181915936083767302932000480_u128];
_20 = 265771378531743989551604665385392193629_u128 as f32;
_24 = Adt53::Variant0 { fld0: (-7894826723891579599_i64),fld1: RET };
_23 = _15;
_18.fld4 = (_17.0, _18.fld0);
_20 = 7748382318508508040_i64 as f32;
_21 = [(-2321911602952279912_i64),3335462974147445060_i64,(-1293668889851091579_i64),7936803766980406090_i64,(-2487777628855444477_i64)];
_25.1 = core::ptr::addr_of_mut!((*_9));
_28 = core::ptr::addr_of_mut!(_6);
_26 = _8;
_27 = [_11,_11,_11,_11,_11];
_8 = _26;
_17 = (_18.fld4.0, _18.fld4.1);
Goto(bb11)
}
bb11 = {
_6 = 79_u8 >> _4;
_19.0.0 = [(-110_i8),(-112_i8),(-97_i8),(-64_i8)];
_8 = _26;
_16 = _18.fld4.1 + _17.1;
Goto(bb12)
}
bb12 = {
_14 = [_8,_26,_26,_26];
(*_9) = -_2;
_17 = _18.fld4;
Goto(bb13)
}
bb13 = {
(*_9) = -_10;
_18.fld0 = _16 << (*_9);
_29 = (_5,);
_13 = 3634460869_u32 as i16;
_19.1 = (-19650880235592093070521720482206604816_i128);
_33 = 45_i8 & 55_i8;
_10 = (*_9);
place!(Field::<[u128; 6]>(Variant(_24, 0), 1)) = [8566933280990176442355262085553803273_u128,11305283407012665673304953495850416184_u128,163805958448543694312322967486793062180_u128,1236244138612466620053006612290868125_u128,252549184519845938937787792437871360250_u128,165140733391875889102996623849598589975_u128];
_27 = [_11,_11,_11,_11,_11];
_32 = _19.1 << _1;
_21 = [(-5134286850329214646_i64),(-4533455566844633388_i64),(-6555388548841231907_i64),7870988699660620241_i64,(-1041002162732138308_i64)];
_9 = core::ptr::addr_of_mut!((*_9));
place!(Field::<i64>(Variant(_24, 0), 0)) = (-1687553673232648994_i64);
place!(Field::<[u128; 6]>(Variant(_24, 0), 1)) = [84581177462357164223466959311295822334_u128,251320399508007460399457016222095190330_u128,4911188956778331084218340964624608102_u128,118112918715939294417612214199320468684_u128,58597442624159382410171394400675115195_u128,107612057510120817903433778303276220344_u128];
_35 = [Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0)];
_17.0 = _23 as usize;
_11 = !true;
_19 = (_29, _32, RET);
match _18.fld4.0 {
0 => bb9,
1 => bb12,
2 => bb4,
14720254177163774269 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_9 = core::ptr::addr_of_mut!((*_9));
_10 = !(*_9);
_21 = [Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0),Field::<i64>(Variant(_24, 0), 0)];
_6 = 37_u8 - 211_u8;
_18.fld0 = Field::<i64>(Variant(_24, 0), 0) as i32;
place!(Field::<i64>(Variant(_24, 0), 0)) = (-3553720837136173770_i64);
_18.fld0 = _17.1 * _17.1;
_19 = (_29, _32, Field::<[u128; 6]>(Variant(_24, 0), 1));
_8 = _26;
_19.0.0 = [_33,_33,_33,_33];
_19.0 = _29;
_29 = _19.0;
_18.fld0 = 176291103976253279116590752011601066008_u128 as i32;
_1 = _12;
SetDiscriminant(_24, 1);
_3 = _1;
(*_28) = 25_u8 * 49_u8;
_25.1 = core::ptr::addr_of_mut!(_13);
place!(Field::<char>(Variant(_24, 1), 1)) = _26;
Goto(bb16)
}
bb16 = {
Call(_41 = dump_var(4_usize, 33_usize, Move(_33), 26_usize, Move(_26), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(4_usize, 23_usize, Move(_23), 35_usize, Move(_35), 2_usize, Move(_2), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(4_usize, 1_usize, Move(_1), 14_usize, Move(_14), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(4_usize, 15_usize, Move(_15), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: char,mut _2: bool,mut _3: i32,mut _4: i16,mut _5: i16,mut _6: u64,mut _7: i16,mut _8: u8,mut _9: u64,mut _10: i16,mut _11: u64,mut _12: i16,mut _13: u64,mut _14: *mut i16) -> u64 {
mir! {
type RET = u64;
let _15: [i8; 4];
let _16: (*mut u32, *mut i16);
let _17: i8;
let _18: bool;
let _19: [u16; 3];
let _20: usize;
let _21: (([i8; 4],), i128, [u128; 6]);
let _22: (usize, i32);
let _23: (usize, i32);
let _24: [char; 4];
let _25: Adt47;
let _26: u64;
let _27: [i8; 2];
let _28: Adt55;
let _29: *mut u128;
let _30: [u16; 3];
let _31: usize;
let _32: f64;
let _33: [bool; 5];
let _34: (([i8; 4],), i128, [u128; 6]);
let _35: char;
let _36: usize;
let _37: isize;
let _38: *const usize;
let _39: Adt56;
let _40: ();
let _41: ();
{
RET = _11;
_3 = (-346164589_i32);
_10 = 106010818706710851099896367862547224230_i128 as i16;
_4 = _10 << RET;
_8 = 166_u8 * 12_u8;
Goto(bb1)
}
bb1 = {
_1 = '\u{d48a3}';
_8 = 29_i8 as u8;
_12 = _10 * _4;
_13 = 9223372036854775807_isize as u64;
_15 = [(-117_i8),62_i8,3_i8,(-92_i8)];
_9 = _11;
_1 = '\u{c83d9}';
match (*_14) {
0 => bb2,
1 => bb3,
340282366920938463463374607431768180580 => bb5,
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
_15 = [15_i8,42_i8,(-74_i8),(-97_i8)];
_7 = 3875194539_u32 as i16;
_11 = _9 << _4;
_16.1 = core::ptr::addr_of_mut!(_7);
_11 = _1 as u64;
_1 = '\u{af85c}';
_18 = _6 == RET;
_7 = -(*_14);
_4 = -_5;
Goto(bb6)
}
bb6 = {
(*_14) = _12 & _12;
_2 = _18;
_20 = 55455417187465965348103230076416516652_i128 as usize;
Goto(bb7)
}
bb7 = {
_11 = _6 + _13;
match _3 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb9,
340282366920938463463374607431422046867 => bb11,
_ => bb10
}
}
bb8 = {
_1 = '\u{d48a3}';
_8 = 29_i8 as u8;
_12 = _10 * _4;
_13 = 9223372036854775807_isize as u64;
_15 = [(-117_i8),62_i8,3_i8,(-92_i8)];
_9 = _11;
_1 = '\u{c83d9}';
match (*_14) {
0 => bb2,
1 => bb3,
340282366920938463463374607431768180580 => bb5,
_ => bb4
}
}
bb9 = {
_15 = [15_i8,42_i8,(-74_i8),(-97_i8)];
_7 = 3875194539_u32 as i16;
_11 = _9 << _4;
_16.1 = core::ptr::addr_of_mut!(_7);
_11 = _1 as u64;
_1 = '\u{af85c}';
_18 = _6 == RET;
_7 = -(*_14);
_4 = -_5;
Goto(bb6)
}
bb10 = {
Return()
}
bb11 = {
_1 = '\u{76a23}';
RET = !_11;
RET = 8487845856470357928_i64 as u64;
_13 = RET;
_13 = RET;
_6 = _11 | _11;
_7 = _12;
_8 = 154_u8;
_14 = core::ptr::addr_of_mut!((*_14));
_3 = 810464714_i32;
_10 = _8 as i16;
_5 = _7 * (*_14);
(*_14) = _5;
_5 = -_12;
_5 = (*_14) + (*_14);
_17 = 6_i8 << _7;
_14 = core::ptr::addr_of_mut!(_10);
_10 = _5 & _7;
_21.2 = [276241980153817056848649457161088227386_u128,81811043486265391568233592442705375376_u128,267404197699124704964201612452264733482_u128,127840477936529612574010795924375527259_u128,193337625180655974610835894123155800456_u128,73533721525235816808810869588217483570_u128];
_8 = (-26504491937098261490992960596885298274_i128) as u8;
_23.0 = _18 as usize;
RET = (-16_isize) as u64;
_16.1 = core::ptr::addr_of_mut!(_12);
_12 = _10;
_3 = -(-294419655_i32);
Goto(bb12)
}
bb12 = {
_14 = _16.1;
RET = 2733334595_u32 as u64;
_5 = _2 as i16;
_24 = [_1,_1,_1,_1];
_5 = 9223372036854775807_isize as i16;
_7 = _12 ^ _12;
_22 = (_20, _3);
_21.2 = [262968932402269741449158118340512742297_u128,52553138152132436346889954091049185083_u128,41927095159193790678310226626478390485_u128,160708465879172064740505703987139108810_u128,150491822338525374405261203989687974397_u128,202969987674007173863285334792764635061_u128];
_21.0.0 = [_17,_17,_17,_17];
(*_14) = _7;
(*_14) = !_10;
_10 = _7;
_23 = (_22.0, _22.1);
_21.1 = (-43799115555836556123938949348309978680_i128) * 139452186709629113411290016100303867562_i128;
_28.fld4.fld4.fld5 = _14;
_28.fld1.1 = core::ptr::addr_of_mut!(_5);
Call(_28.fld4.fld6.fld1 = fn6(_17, _16.1, (*_14), _7, _23, _28.fld4.fld4.fld5, _14, _10), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23 = (_20, _3);
_28.fld4.fld6.fld4.1 = _23.1;
_26 = _11 * _11;
_28.fld4.fld4.fld0 = [_17,_17];
_28.fld2 = !_3;
_17 = (-120_i8);
_28.fld4.fld4.fld4 = core::ptr::addr_of_mut!(_8);
RET = 8262966353208792614_i64 as u64;
_11 = _6 << _10;
_21.0 = (_15,);
_22.0 = 1127739918_u32 as usize;
_28.fld4.fld5 = [59320_u16,1910_u16,24907_u16];
_28.fld4.fld6.fld4 = (_23.0, _3);
_28.fld4.fld4.fld2 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_33 = [_2,_18,_2,_18,_2];
_19 = [10684_u16,39769_u16,64435_u16];
_34.1 = -_21.1;
_28.fld0 = [407542727457138035_i64,4746317705203197186_i64,(-4639355970702582450_i64),(-1993184473779423130_i64),(-7809438539473673270_i64)];
Goto(bb14)
}
bb14 = {
_5 = _10;
_22.1 = _23.1 << _5;
_5 = _7 & _7;
_28.fld4.fld4.fld2 = 9223372036854775807_isize ^ 22_isize;
RET = _26 << _5;
_26 = _11 | _11;
_34 = (_21.0, _21.1, _21.2);
_28.fld4.fld0 = [_1,_1,_1,_1];
_38 = core::ptr::addr_of!(_31);
_28.fld3 = _17 * _17;
_28.fld4.fld0 = [_1,_1,_1,_1];
(*_14) = _7;
(*_38) = 213497523_u32 as usize;
RET = _18 as u64;
_28.fld4.fld7 = _10 as f64;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(5_usize, 8_usize, Move(_8), 22_usize, Move(_22), 26_usize, Move(_26), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(5_usize, 3_usize, Move(_3), 2_usize, Move(_2), 4_usize, Move(_4), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(5_usize, 19_usize, Move(_19), 9_usize, Move(_9), 5_usize, Move(_5), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(5_usize, 34_usize, Move(_34), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i8,mut _2: *mut i16,mut _3: i16,mut _4: i16,mut _5: (usize, i32),mut _6: *mut i16,mut _7: *mut i16,mut _8: i16) -> [i32; 6] {
mir! {
type RET = [i32; 6];
let _9: u32;
let _10: [i64; 8];
let _11: isize;
let _12: ();
let _13: ();
{
_7 = core::ptr::addr_of_mut!((*_6));
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_4 = _3 << (*_7);
_8 = (*_2) * (*_7);
(*_6) = _8;
_1 = (-5_i8);
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
(*_2) = (-16673021979134230473790405222206307924_i128) as i16;
_4 = true as i16;
(*_6) = false as i16;
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
match _1 {
0 => bb1,
340282366920938463463374607431768211451 => bb3,
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
(*_7) = 2508999799_u32 as i16;
(*_7) = _3;
_5 = (807781442076075549_usize, (-1368469358_i32));
(*_7) = _8;
Call(_5.1 = fn7(_7, _6, (*_7), _8, (*_7), (*_7), _2, (*_6), _7, (*_7), _2, _1, _3, _6, _2, (*_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = 47_i8 * (-98_i8);
_10 = [(-4337962609611318324_i64),(-1813433661393557966_i64),(-7105976313308469339_i64),5704834829581431988_i64,(-8876554596514102321_i64),(-5151491168133149686_i64),(-5234023331797055637_i64),(-5735450363126687119_i64)];
_9 = 1214096213_u32 >> _8;
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
(*_7) = _8 << _8;
_9 = (*_7) as u32;
(*_7) = _8;
RET = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_10 = [4264561077140706132_i64,(-3450607880914893369_i64),2380567272246158236_i64,(-1972738238917225678_i64),(-6881451746221614392_i64),6087853279732913667_i64,1664314646496215692_i64,3660198709441828447_i64];
(*_6) = 192568893796236898486473079853278898225_u128 as i16;
_2 = core::ptr::addr_of_mut!((*_6));
Goto(bb5)
}
bb5 = {
Call(_12 = dump_var(6_usize, 8_usize, Move(_8), 3_usize, Move(_3), 10_usize, Move(_10), 13_usize, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *mut i16,mut _2: *mut i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: *mut i16,mut _8: i16,mut _9: *mut i16,mut _10: i16,mut _11: *mut i16,mut _12: i8,mut _13: i16,mut _14: *mut i16,mut _15: *mut i16,mut _16: i16) -> i32 {
mir! {
type RET = i32;
let _17: isize;
let _18: (usize, i32);
let _19: ();
let _20: ();
{
(*_11) = -_10;
(*_1) = -_8;
_11 = _7;
_14 = _9;
RET = 289279229_i32 | (-659605215_i32);
(*_11) = 9223372036854775807_isize as i16;
(*_14) = 2775201979_u32 as i16;
(*_14) = (-9223372036854775808_isize) as i16;
_3 = _12 as i16;
_17 = 106_isize;
_18 = (7_usize, RET);
(*_9) = _6;
_17 = -96_isize;
RET = _18.1 << (*_2);
_1 = _14;
(*_15) = 134169751622517431750622262289471340151_u128 as i16;
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(7_usize, 5_usize, Move(_5), 18_usize, Move(_18), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(7_usize, 16_usize, Move(_16), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: u64,mut _3: isize,mut _4: u64,mut _5: i64,mut _6: i16,mut _7: i16,mut _8: usize,mut _9: usize) -> u32 {
mir! {
type RET = u32;
let _10: f32;
let _11: [i64; 8];
let _12: (i32,);
let _13: [i8; 2];
let _14: ([i8; 2], *mut u8);
let _15: Adt46;
let _16: [u16; 3];
let _17: [i8; 4];
let _18: [usize; 8];
let _19: (([i8; 4],), i128, [u128; 6]);
let _20: isize;
let _21: [i64; 5];
let _22: isize;
let _23: *mut i16;
let _24: u8;
let _25: ();
let _26: ();
{
_7 = !_6;
RET = 244568107_u32 * 4162491720_u32;
_8 = 276446482528509930763135394287575131304_u128 as usize;
_6 = _7;
_7 = !_6;
_9 = _8 << _6;
_5 = !(-8621023909433738310_i64);
_1 = _3;
_4 = RET as u64;
RET = 220406948_u32;
_4 = _2;
_7 = _6;
_1 = _3 * _3;
_7 = _6 * _6;
_7 = !_6;
_10 = _1 as f32;
_4 = _2 ^ _2;
_9 = _8;
_7 = (-136011048514934328785276385834030225915_i128) as i16;
_7 = _6 + _6;
_6 = _7 + _7;
RET = 2368244692_u32 >> _4;
_10 = 2017527978_i32 as f32;
Goto(bb1)
}
bb1 = {
_2 = _4;
_10 = _1 as f32;
_5 = (-612058497749382491_i64) << _6;
_5 = -9161625046973681921_i64;
_11 = [_5,_5,_5,_5,_5,_5,_5,_5];
_4 = _2 << _6;
_12.0 = !1720234330_i32;
_3 = _1 >> _2;
RET = 1275974344_u32 & 3211449126_u32;
_14.0 = [75_i8,(-90_i8)];
_15.fld1 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_15.fld1 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_1 = _3;
_12.0 = (-994632245_i32);
Goto(bb2)
}
bb2 = {
_7 = RET as i16;
match _12.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607430773579211 => bb9,
_ => bb8
}
}
bb3 = {
_2 = _4;
_10 = _1 as f32;
_5 = (-612058497749382491_i64) << _6;
_5 = -9161625046973681921_i64;
_11 = [_5,_5,_5,_5,_5,_5,_5,_5];
_4 = _2 << _6;
_12.0 = !1720234330_i32;
_3 = _1 >> _2;
RET = 1275974344_u32 & 3211449126_u32;
_14.0 = [75_i8,(-90_i8)];
_15.fld1 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_15.fld1 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_1 = _3;
_12.0 = (-994632245_i32);
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
_1 = !_3;
_15.fld4.1 = _3 as i32;
_15.fld4.0 = _9 + _8;
_12.0 = _15.fld4.1;
_13 = [(-10_i8),(-51_i8)];
_5 = !706116169072655223_i64;
_11 = [_5,_5,_5,_5,_5,_5,_5,_5];
_14.0 = [(-67_i8),6_i8];
_2 = _4;
Goto(bb10)
}
bb10 = {
_19.0.0 = [(-72_i8),49_i8,48_i8,120_i8];
_8 = _9;
Goto(bb11)
}
bb11 = {
_12.0 = 122771815658306566545930371592601849519_u128 as i32;
_15.fld4.0 = _8 + _8;
RET = 145_u8 as u32;
_13 = [106_i8,(-60_i8)];
_19.1 = !142855657374059743239589517512796037637_i128;
_10 = RET as f32;
_12.0 = _15.fld4.1 * _15.fld4.1;
_21 = [_5,_5,_5,_5,_5];
Goto(bb12)
}
bb12 = {
_15.fld0 = _12.0;
Goto(bb13)
}
bb13 = {
_18 = [_15.fld4.0,_15.fld4.0,_15.fld4.0,_9,_9,_9,_9,_9];
_1 = _3;
Goto(bb14)
}
bb14 = {
_18 = [_15.fld4.0,_15.fld4.0,_9,_9,_15.fld4.0,_15.fld4.0,_9,_15.fld4.0];
_20 = _3;
RET = 2071217585_u32;
_18 = [_9,_15.fld4.0,_8,_9,_8,_15.fld4.0,_15.fld4.0,_9];
_14.1 = core::ptr::addr_of_mut!(_24);
_24 = 100_u8 + 232_u8;
_2 = '\u{e79e}' as u64;
_11 = [_5,_5,_5,_5,_5,_5,_5,_5];
_15.fld1 = [_15.fld0,_12.0,_15.fld0,_12.0,_12.0,_15.fld4.1];
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(8_usize, 12_usize, Move(_12), 8_usize, Move(_8), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(8_usize, 11_usize, Move(_11), 21_usize, Move(_21), 4_usize, Move(_4), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i32,mut _2: (usize, i32)) -> u32 {
mir! {
type RET = u32;
let _3: &'static i64;
let _4: isize;
let _5: [usize; 8];
let _6: u128;
let _7: f32;
let _8: [i64; 5];
let _9: i64;
let _10: u16;
let _11: char;
let _12: f32;
let _13: bool;
let _14: Adt46;
let _15: i8;
let _16: Adt41;
let _17: ();
let _18: ();
{
RET = 754242211_u32;
_1 = -_2.1;
_1 = _2.1 + _2.1;
_2.0 = 9223372036854775807_isize as usize;
_4 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
Call(_1 = fn10(RET, RET, _4, _4, _4, _2.1, _4, _2.0, _2.0, _2.1, _2.1, _2.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 1066916314_u32;
_4 = (-89_isize) + 9223372036854775807_isize;
_4 = 100794270614972436017967368118674933416_i128 as isize;
_4 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_2.1 = _1 >> _1;
_2 = (2562282636740721342_usize, _1);
match _2.0 {
2562282636740721342 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_4 = RET as isize;
RET = 1312374635_u32 * 310923033_u32;
_2 = (15295865175928982312_usize, _1);
RET = 4288725398_u32 - 3046662026_u32;
RET = !1054354717_u32;
_2.1 = _4 as i32;
_7 = 78217560227025029921628352996682686931_i128 as f32;
RET = 3321823321_u32;
_4 = 9223372036854775807_isize | (-73_isize);
_4 = !(-9223372036854775808_isize);
_7 = 47895_u16 as f32;
_2.0 = !4_usize;
Goto(bb4)
}
bb4 = {
_5 = [_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0];
_2 = (3_usize, _1);
_8 = [(-8581317942997662198_i64),(-8083051352958588571_i64),(-2532247414639938322_i64),355007907993208848_i64,(-2800755296148066638_i64)];
_6 = 274496158450012901421818251409202641793_u128 | 311921963679580925601790755106707366307_u128;
_6 = 123407720523827188386420086632685691466_u128;
_2 = (0_usize, _1);
_2 = (16110813974282695189_usize, _1);
_5 = [_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0];
_5 = [_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0];
_1 = _2.1 - _2.1;
RET = 1932648879_u32 * 2822229919_u32;
_5 = [_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0,_2.0];
_3 = &_9;
_6 = 208114005179030246202794697034948678118_u128 | 32428703231635550819423497062569852936_u128;
_9 = (-5779993005094775135_i64) << _2.0;
_3 = &_9;
_11 = '\u{84447}';
_2.0 = 33346792889770293_usize;
Call(_10 = core::intrinsics::bswap(30983_u16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = 1669_i16 as isize;
_13 = _2.1 >= _2.1;
_3 = &(*_3);
_14.fld4.1 = 3622239984598598269_u64 as i32;
RET = 166981098_u32 >> _2.1;
_13 = !false;
_2.1 = _1;
_14.fld4 = _2;
_15 = (-100_i8);
_2.1 = -_1;
_14.fld2 = core::ptr::addr_of_mut!(_6);
RET = 3308105953_u32 >> _1;
_12 = 10257439844370045532723351672266016476_i128 as f32;
RET = 3051164670_u32 >> _14.fld4.1;
Goto(bb6)
}
bb6 = {
Call(_17 = dump_var(9_usize, 2_usize, Move(_2), 1_usize, Move(_1), 15_usize, Move(_15), 9_usize, Move(_9)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_17 = dump_var(9_usize, 10_usize, Move(_10), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u32,mut _2: u32,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: i32,mut _7: isize,mut _8: usize,mut _9: usize,mut _10: i32,mut _11: i32,mut _12: usize) -> i32 {
mir! {
type RET = i32;
let _13: [u16; 3];
let _14: [i8; 2];
let _15: *mut i16;
let _16: f32;
let _17: [char; 4];
let _18: i16;
let _19: char;
let _20: [usize; 8];
let _21: Adt46;
let _22: [bool; 5];
let _23: bool;
let _24: u32;
let _25: Adt56;
let _26: ();
let _27: ();
{
_4 = _3 << _5;
_1 = 143_u8 as u32;
_6 = _8 as i32;
RET = _10 + _10;
_3 = -_4;
_2 = _1 + _1;
_14 = [(-46_i8),93_i8];
_2 = '\u{be89}' as u32;
RET = -_11;
_4 = _7;
_9 = _8 - _8;
_14 = [(-62_i8),58_i8];
_2 = _1;
_6 = RET * _11;
_3 = _4 >> _2;
_13 = [26019_u16,61333_u16,19694_u16];
RET = _2 as i32;
RET = '\u{5cadf}' as i32;
_16 = _4 as f32;
RET = _6 * _11;
_15 = core::ptr::addr_of_mut!(_18);
Goto(bb1)
}
bb1 = {
_13 = [22250_u16,36791_u16,37541_u16];
_5 = -_3;
(*_15) = !3793_i16;
_12 = _16 as usize;
_17 = ['\u{d2a6a}','\u{682a3}','\u{32b8c}','\u{b6245}'];
RET = _2 as i32;
(*_15) = -10669_i16;
_9 = 225585087968521476008962485616762862573_u128 as usize;
_12 = _8;
_15 = core::ptr::addr_of_mut!((*_15));
_19 = '\u{8c476}';
_4 = -_3;
_19 = '\u{95084}';
RET = _6;
_10 = -RET;
_20 = [_9,_12,_8,_9,_12,_9,_12,_12];
_8 = _9 & _12;
_4 = _3;
_17 = [_19,_19,_19,_19];
_6 = !RET;
_8 = 3805162708761010361_u64 as usize;
_5 = _7 + _4;
_7 = _3;
_12 = _8;
_11 = -_10;
_16 = 2895463802519061912_u64 as f32;
_6 = !_10;
Call(_22 = fn11(_11, _4, _5, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_21.fld4 = (_9, _6);
Goto(bb3)
}
bb3 = {
_19 = '\u{44f0a}';
_21.fld4.0 = _8;
_7 = _3 ^ _4;
_13 = [23041_u16,7029_u16,20307_u16];
_21.fld4.1 = _10 << _10;
_7 = !_3;
_8 = _21.fld4.0 >> _21.fld4.1;
_3 = !_7;
_21.fld0 = _10;
_4 = _3;
_23 = true;
_9 = _8 << _8;
RET = _10 ^ _10;
Goto(bb4)
}
bb4 = {
Call(_26 = dump_var(10_usize, 11_usize, Move(_11), 19_usize, Move(_19), 18_usize, Move(_18), 22_usize, Move(_22)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_26 = dump_var(10_usize, 17_usize, Move(_17), 7_usize, Move(_7), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_26 = dump_var(10_usize, 13_usize, Move(_13), 9_usize, Move(_9), 27_usize, _27, 27_usize, _27), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i32,mut _2: isize,mut _3: isize,mut _4: isize) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _5: f32;
let _6: *const usize;
let _7: Adt56;
let _8: isize;
let _9: isize;
let _10: Adt47;
let _11: u8;
let _12: isize;
let _13: i16;
let _14: u64;
let _15: isize;
let _16: isize;
let _17: f32;
let _18: i64;
let _19: i8;
let _20: [u128; 6];
let _21: *mut u128;
let _22: ([i8; 4],);
let _23: u128;
let _24: (i32,);
let _25: u8;
let _26: [i8; 4];
let _27: [i8; 2];
let _28: u16;
let _29: (([i8; 4],), i128, [u128; 6]);
let _30: bool;
let _31: Adt52;
let _32: u128;
let _33: f64;
let _34: (*mut u32, *mut i16);
let _35: [i32; 6];
let _36: [i8; 4];
let _37: (i32,);
let _38: ();
let _39: ();
{
RET = [true,false,true,false,false];
_5 = 39229_u16 as f32;
Call(_6 = fn12(_3, _4, _3, _4, _4, _4, _2, _4, _4, _2, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _1 as f32;
RET = [false,false,false,false,false];
_1 = 3_usize as i32;
RET = [true,true,false,false,true];
_1 = -(-652537250_i32);
_2 = !_3;
RET = [false,false,false,true,false];
RET = [false,true,false,true,true];
_2 = (-47510657065123525157953576127261991667_i128) as isize;
_2 = _4;
Goto(bb2)
}
bb2 = {
_8 = _3 << _4;
_3 = _8 + _8;
_4 = _2 - _8;
_9 = _8 + _3;
_8 = true as isize;
RET = [false,false,false,true,false];
_1 = _9 as i32;
_2 = -_9;
_8 = 146819427763854671861176328190743937696_i128 as isize;
_3 = _2 - _9;
_5 = 2009883628710444230295851600630867847_u128 as f32;
_1 = (-1332160203_i32) << _4;
RET = [false,true,false,false,false];
_11 = !140_u8;
Goto(bb3)
}
bb3 = {
RET = [true,true,false,false,false];
RET = [true,true,false,true,false];
_3 = _9;
RET = [false,false,false,false,true];
_2 = !_9;
_11 = !86_u8;
_11 = !75_u8;
_2 = 10899_u16 as isize;
_12 = _9 >> _3;
RET = [true,true,false,false,false];
RET = [false,false,true,true,false];
_8 = 312278037447740654301392451507138299560_u128 as isize;
Goto(bb4)
}
bb4 = {
_5 = 6_usize as f32;
_5 = 0_usize as f32;
Goto(bb5)
}
bb5 = {
_2 = false as isize;
_12 = 3212300271_u32 as isize;
_2 = _11 as isize;
_14 = _5 as u64;
_9 = -_4;
_13 = !(-9685_i16);
_11 = 4870457408860633606_i64 as u8;
_3 = !_4;
_16 = _3 | _9;
RET = [true,true,true,true,true];
RET = [true,true,false,true,false];
_2 = 256336789_u32 as isize;
_4 = '\u{a4b10}' as isize;
_4 = _16 * _16;
_16 = -_4;
RET = [true,false,true,true,false];
_13 = 2904886540_u32 as i16;
_17 = _5 + _5;
_15 = _9;
_18 = -1857415793914698425_i64;
RET = [false,false,true,true,false];
Goto(bb6)
}
bb6 = {
_5 = _17;
_11 = _1 as u8;
_8 = _16 ^ _4;
_3 = _17 as isize;
_8 = _4 - _4;
_3 = !_8;
_4 = _2;
_14 = _5 as u64;
_12 = !_16;
RET = [true,true,false,false,true];
_4 = _3 << _16;
_2 = -_15;
_14 = 11564846468515643375_u64;
_16 = '\u{7fa62}' as isize;
_11 = 115_u8 - 92_u8;
_17 = 2273103706_u32 as f32;
match _14 {
0 => bb1,
1 => bb5,
2 => bb4,
3 => bb7,
4 => bb8,
11564846468515643375 => bb10,
_ => bb9
}
}
bb7 = {
_2 = false as isize;
_12 = 3212300271_u32 as isize;
_2 = _11 as isize;
_14 = _5 as u64;
_9 = -_4;
_13 = !(-9685_i16);
_11 = 4870457408860633606_i64 as u8;
_3 = !_4;
_16 = _3 | _9;
RET = [true,true,true,true,true];
RET = [true,true,false,true,false];
_2 = 256336789_u32 as isize;
_4 = '\u{a4b10}' as isize;
_4 = _16 * _16;
_16 = -_4;
RET = [true,false,true,true,false];
_13 = 2904886540_u32 as i16;
_17 = _5 + _5;
_15 = _9;
_18 = -1857415793914698425_i64;
RET = [false,false,true,true,false];
Goto(bb6)
}
bb8 = {
_5 = 6_usize as f32;
_5 = 0_usize as f32;
Goto(bb5)
}
bb9 = {
_5 = _1 as f32;
RET = [false,false,false,false,false];
_1 = 3_usize as i32;
RET = [true,true,false,false,true];
_1 = -(-652537250_i32);
_2 = !_3;
RET = [false,false,false,true,false];
RET = [false,true,false,true,true];
_2 = (-47510657065123525157953576127261991667_i128) as isize;
_2 = _4;
Goto(bb2)
}
bb10 = {
_19 = 92_i8 >> _8;
_13 = (-4812_i16);
_11 = '\u{dea11}' as u8;
_14 = _18 as u64;
RET = [true,true,true,true,false];
_8 = !_4;
_18 = !4199177383689331204_i64;
_5 = 134118231091841512879030089056965760947_i128 as f32;
_14 = 4772705411829528406_u64 + 4083843001821748240_u64;
_5 = _17 - _17;
_15 = _4;
_15 = !_8;
_13 = _18 as i16;
_19 = (-59_i8);
_19 = (-27_i8) & 40_i8;
_13 = (-7461_i16);
_17 = 86895597984903436915105784490814521771_i128 as f32;
_19 = !(-115_i8);
_17 = _5 - _5;
_19 = 40_i8;
_18 = -7964784793294104836_i64;
_4 = -_3;
_3 = !_12;
_20 = [7804559517817162279701185294283809152_u128,16567913017266430697998704373269978088_u128,111804247898134370486774521715377131240_u128,146893285219225270199050906894419519229_u128,104712388266174847611809586057884196877_u128,286726203449715564093627017393767550390_u128];
Goto(bb11)
}
bb11 = {
_14 = _5 as u64;
_3 = _16;
_13 = false as i16;
_22.0 = [_19,_19,_19,_19];
_4 = 13681519236699405975_usize as isize;
_11 = 255_u8;
RET = [false,false,true,false,false];
_14 = 3773893135574303655_u64 - 14843138948409540844_u64;
_2 = _8;
_16 = _19 as isize;
_10 = Adt47::Variant0 { fld0: _20,fld1: _11,fld2: _15,fld3: _14 };
_23 = 230944456285295542203399569117004683245_u128;
_18 = -6995446322212933854_i64;
Goto(bb12)
}
bb12 = {
_12 = !_8;
_21 = core::ptr::addr_of_mut!(_23);
(*_21) = '\u{e37d}' as u128;
_22.0 = [_19,_19,_19,_19];
place!(Field::<u64>(Variant(_10, 0), 3)) = _14;
_4 = 3367830791719008063_usize as isize;
SetDiscriminant(_10, 1);
place!(Field::<i64>(Variant(_10, 1), 6)) = _11 as i64;
_24.0 = -_1;
_12 = (*_21) as isize;
_16 = _8;
place!(Field::<([i8; 4],)>(Variant(_10, 1), 3)).0 = [_19,_19,_19,_19];
_22.0 = Field::<([i8; 4],)>(Variant(_10, 1), 3).0;
_24 = (_1,);
_14 = 4100405081835165300_u64;
_8 = _15;
Goto(bb13)
}
bb13 = {
place!(Field::<[usize; 8]>(Variant(_10, 1), 1)) = [2339769089599015830_usize,6_usize,0_usize,6_usize,1_usize,11313313767305591898_usize,1523842528179650591_usize,1_usize];
place!(Field::<f32>(Variant(_10, 1), 2)) = _17 + _17;
place!(Field::<i64>(Variant(_10, 1), 6)) = _18 * _18;
_1 = (-23475428106694463856075367022282685079_i128) as i32;
_23 = _19 as u128;
_29 = (_22, 120973111116051021041639856710765913574_i128, _20);
_12 = _16;
_23 = 322661017625202933155267030183639467062_u128 & 134604903475408256869252014477315623854_u128;
_16 = -_12;
_24.0 = !_1;
_20 = _29.2;
_24.0 = _1 + _1;
_8 = _15 - _2;
_29.2 = [(*_21),(*_21),(*_21),(*_21),(*_21),(*_21)];
_17 = Field::<f32>(Variant(_10, 1), 2) * Field::<f32>(Variant(_10, 1), 2);
_23 = 115949620756442053546970183564832824192_u128;
_21 = core::ptr::addr_of_mut!((*_21));
(*_21) = 323982744845639030388875402110424271553_u128;
_30 = true;
(*_21) = !198343533635209784267412920326422507903_u128;
Goto(bb14)
}
bb14 = {
_16 = _8;
_22 = (Field::<([i8; 4],)>(Variant(_10, 1), 3).0,);
_24.0 = _17 as i32;
_18 = 65317_u16 as i64;
_28 = 14421_u16 | 44021_u16;
_31.fld4.fld2 = -_2;
_6 = core::ptr::addr_of!(_31.fld6.fld4.0);
_20 = [(*_21),(*_21),(*_21),(*_21),(*_21),(*_21)];
_31.fld4.fld5 = core::ptr::addr_of_mut!(_13);
_37.0 = _24.0;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(11_usize, 8_usize, Move(_8), 24_usize, Move(_24), 13_usize, Move(_13), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(11_usize, 18_usize, Move(_18), 11_usize, Move(_11), 12_usize, Move(_12), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(11_usize, 4_usize, Move(_4), 9_usize, Move(_9), 16_usize, Move(_16), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: i32,mut _12: i32) -> *const usize {
mir! {
type RET = *const usize;
let _13: isize;
let _14: Adt49;
let _15: f32;
let _16: (usize, i32);
let _17: i128;
let _18: bool;
let _19: char;
let _20: [usize; 8];
let _21: u128;
let _22: (([i8; 4],), i128, [u128; 6]);
let _23: [i64; 5];
let _24: [i64; 8];
let _25: (i32,);
let _26: i16;
let _27: char;
let _28: bool;
let _29: i32;
let _30: f64;
let _31: u16;
let _32: Adt42;
let _33: u64;
let _34: (*mut u32, *mut i16);
let _35: (usize, i32);
let _36: [char; 4];
let _37: (i32,);
let _38: *mut i16;
let _39: isize;
let _40: ();
let _41: ();
{
_3 = _2;
_5 = -_6;
Call(_5 = fn13(_9, _10, _3, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _6 + _1;
_10 = -_4;
_11 = -_12;
_3 = (-9155_i16) as isize;
Goto(bb2)
}
bb2 = {
_11 = 58398840696564083430151564872742767213_u128 as i32;
_3 = false as isize;
_3 = _8 >> _4;
_6 = _4;
_7 = _2 & _10;
_9 = 19711_i16 as isize;
_2 = _5;
_1 = _7 - _4;
_10 = _2 + _4;
_11 = _12 + _12;
_6 = !_10;
_11 = _12 ^ _12;
Goto(bb3)
}
bb3 = {
_3 = 3_u8 as isize;
_17 = -(-166167979850206370869944627681946777493_i128);
_8 = -_10;
_7 = (-68_i8) as isize;
_15 = 220_u8 as f32;
_16.0 = 3_usize ^ 687096834289449444_usize;
_15 = _16.0 as f32;
_11 = 4139034223_u32 as i32;
_10 = _4;
_5 = _7;
_16.1 = _12;
_3 = _1;
_5 = (-4419_i16) as isize;
_18 = !false;
_2 = -_6;
_6 = _2 << _2;
Goto(bb4)
}
bb4 = {
RET = core::ptr::addr_of!(_16.0);
_3 = _8;
_12 = _16.1;
_15 = 2119133899_u32 as f32;
_8 = _4;
_18 = false;
_16.1 = _12;
_18 = _6 == _6;
_10 = _6 - _1;
(*RET) = _17 as usize;
(*RET) = !15493076520277766954_usize;
_15 = (*RET) as f32;
_19 = '\u{10e4ea}';
_13 = _6 << _6;
_20 = [_16.0,(*RET),(*RET),_16.0,(*RET),(*RET),(*RET),(*RET)];
_11 = _19 as i32;
_2 = _8 & _1;
_19 = '\u{5e799}';
_22.0.0 = [(-79_i8),(-29_i8),67_i8,113_i8];
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of!(_16.0);
_5 = _13 | _10;
_5 = -_1;
_21 = !295876566183716417433814473592524375320_u128;
_5 = _13 ^ _9;
_9 = -_1;
_15 = 32975_u16 as f32;
_22.1 = -_17;
_7 = 12676904925827919627_u64 as isize;
_8 = _13;
_5 = -_8;
_23 = [5028105400568476644_i64,(-4924567014791484892_i64),4094004330268367527_i64,(-7038032164177357361_i64),(-2141483748346747214_i64)];
_22.2 = [_21,_21,_21,_21,_21,_21];
_24 = [(-652338935918473760_i64),8725699576255636700_i64,(-1696721991566904133_i64),(-4917380545226792940_i64),7973864269778210873_i64,(-3576069745080735856_i64),8488257195995010774_i64,2239011059217059594_i64];
_18 = !false;
_1 = _5;
_22.1 = -_17;
_4 = (-112_i8) as isize;
_23 = [(-1356377848156568794_i64),(-7783314521643372561_i64),1602157273732500348_i64,2581107330464790221_i64,(-881217911863477033_i64)];
_15 = _17 as f32;
_25 = (_12,);
(*RET) = 14347445167306272808_usize - 3_usize;
_13 = !_3;
Goto(bb6)
}
bb6 = {
_22.1 = _15 as i128;
Goto(bb7)
}
bb7 = {
(*RET) = _18 as usize;
RET = core::ptr::addr_of!(_16.0);
Goto(bb8)
}
bb8 = {
_25.0 = _22.1 as i32;
(*RET) = 2994306557733147850_i64 as usize;
_22.0.0 = [60_i8,(-104_i8),(-71_i8),(-72_i8)];
_11 = 25832_i16 as i32;
RET = core::ptr::addr_of!(_16.0);
_16 = (1_usize, _12);
_7 = _10;
_22.0.0 = [(-50_i8),127_i8,(-77_i8),(-117_i8)];
_7 = _5;
_16.1 = !_12;
_10 = _15 as isize;
_22.2 = [_21,_21,_21,_21,_21,_21];
_7 = _2 ^ _2;
(*RET) = 11763175446700224248_u64 as usize;
_13 = _19 as isize;
_25.0 = _12;
_6 = _15 as isize;
_13 = _21 as isize;
_22.0.0 = [3_i8,37_i8,12_i8,(-19_i8)];
_22.2 = [_21,_21,_21,_21,_21,_21];
_15 = 27769_u16 as f32;
_8 = _7;
_26 = !18497_i16;
_11 = _12 | _16.1;
_25.0 = !_11;
_11 = _25.0;
_12 = _21 as i32;
Call(_5 = core::intrinsics::transmute(_7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = core::ptr::addr_of!((*RET));
_16.0 = (-17_i8) as usize;
_16.0 = 3_usize - 5_usize;
_30 = _15 as f64;
_1 = _8;
_25 = (_16.1,);
Call(_31 = core::intrinsics::bswap(47736_u16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_26 = 15039_i16 & (-14858_i16);
_10 = -_5;
_16.0 = 8447199983570676478_usize;
(*RET) = !3_usize;
_19 = '\u{11a58}';
_17 = -_22.1;
_21 = !172305738055198147952562511554229958943_u128;
(*RET) = 5_usize & 1679773287122459704_usize;
_9 = _7 - _5;
_23 = [4434780695991571426_i64,(-4830180020543405318_i64),(-6329804758192041660_i64),1363235154584677651_i64,(-217341447888711294_i64)];
Goto(bb11)
}
bb11 = {
_16.1 = _25.0 - _11;
Goto(bb12)
}
bb12 = {
_8 = _10 << _9;
_3 = _8;
(*RET) = !14990210333821099606_usize;
_29 = _16.1;
_5 = _10;
_28 = _18;
_23 = [(-6930969141249623567_i64),8513522946507480808_i64,(-2142972454112484303_i64),7125861634003135834_i64,4851802473353802792_i64];
_17 = _3 as i128;
_16 = (5131419759113699821_usize, _25.0);
_17 = -_22.1;
RET = core::ptr::addr_of!((*RET));
_16.0 = !7_usize;
_3 = _8 - _8;
_31 = 6374_u16;
(*RET) = 4_usize;
_20 = [(*RET),(*RET),(*RET),(*RET),(*RET),(*RET),_16.0,_16.0];
_8 = _5;
_29 = _11 & _25.0;
_35 = (_16.0, _16.1);
_7 = _10 * _8;
(*RET) = !_35.0;
_7 = _3;
_36 = [_19,_19,_19,_19];
_31 = _11 as u16;
(*RET) = !_35.0;
_28 = _18;
_25 = (_29,);
RET = core::ptr::addr_of!((*RET));
match _35.0 {
0 => bb13,
1 => bb14,
4 => bb16,
_ => bb15
}
}
bb13 = {
_4 = _6 + _1;
_10 = -_4;
_11 = -_12;
_3 = (-9155_i16) as isize;
Goto(bb2)
}
bb14 = {
_26 = 15039_i16 & (-14858_i16);
_10 = -_5;
_16.0 = 8447199983570676478_usize;
(*RET) = !3_usize;
_19 = '\u{11a58}';
_17 = -_22.1;
_21 = !172305738055198147952562511554229958943_u128;
(*RET) = 5_usize & 1679773287122459704_usize;
_9 = _7 - _5;
_23 = [4434780695991571426_i64,(-4830180020543405318_i64),(-6329804758192041660_i64),1363235154584677651_i64,(-217341447888711294_i64)];
Goto(bb11)
}
bb15 = {
_22.1 = _15 as i128;
Goto(bb7)
}
bb16 = {
_23 = [(-369989084951564815_i64),(-7133096312904418185_i64),3430518726631661204_i64,3149929845911546603_i64,8198742660146646629_i64];
_37 = (_35.1,);
_2 = _3 + _10;
_1 = 220_u8 as isize;
_34.1 = core::ptr::addr_of_mut!(_26);
_37.0 = -_25.0;
_25 = _37;
_20 = [(*RET),(*RET),_35.0,(*RET),_35.0,_16.0,_35.0,_16.0];
_7 = _30 as isize;
RET = core::ptr::addr_of!((*RET));
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(12_usize, 9_usize, Move(_9), 4_usize, Move(_4), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(12_usize, 23_usize, Move(_23), 24_usize, Move(_24), 13_usize, Move(_13), 31_usize, Move(_31)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(12_usize, 7_usize, Move(_7), 18_usize, Move(_18), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(12_usize, 29_usize, Move(_29), 16_usize, Move(_16), 35_usize, Move(_35), 41_usize, _41), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i32) -> isize {
mir! {
type RET = isize;
let _5: isize;
let _6: f32;
let _7: i128;
let _8: isize;
let _9: Adt45;
let _10: [i32; 6];
let _11: i64;
let _12: [usize; 8];
let _13: u128;
let _14: [usize; 8];
let _15: bool;
let _16: i16;
let _17: u32;
let _18: char;
let _19: char;
let _20: [i64; 5];
let _21: isize;
let _22: *const usize;
let _23: [bool; 5];
let _24: f64;
let _25: f64;
let _26: [bool; 5];
let _27: Adt52;
let _28: (([i8; 4],), i128, [u128; 6]);
let _29: [i64; 8];
let _30: f64;
let _31: ();
let _32: ();
{
RET = _2;
_1 = _3;
_3 = _2 + RET;
_3 = '\u{d54f8}' as isize;
RET = !_2;
RET = false as isize;
RET = _2 | _2;
_5 = -_1;
_6 = _4 as f32;
_3 = !_1;
_7 = 136936110753246916019236977040410454998_i128;
_4 = -(-540712013_i32);
_1 = RET;
_3 = _2 & _2;
_8 = RET;
_3 = 2840293488_u32 as isize;
_7 = !(-119846579862188097863034338264284959484_i128);
_1 = !RET;
_6 = (-15140_i16) as f32;
_2 = !RET;
RET = (-4548921917881265084_i64) as isize;
_1 = _2;
_2 = 97107510162310489973603903827755293643_u128 as isize;
_1 = 3127449463_u32 as isize;
RET = '\u{bd1da}' as isize;
Call(_8 = fn14(_2, _5, _5, _5, _7, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = 61334215064706877575642773751034884610_i128;
_11 = -(-4764454194083037566_i64);
_1 = '\u{a06ac}' as isize;
_3 = _5;
_10 = [_4,_4,_4,_4,_4,_4];
_8 = 13911_u16 as isize;
_1 = _5;
_11 = 7114799764337988514_i64 << RET;
RET = -_3;
_5 = _7 as isize;
_11 = -(-3096573398178030941_i64);
_6 = 189624638536909814921912764012105870214_u128 as f32;
_15 = true ^ false;
_15 = true;
_15 = false;
_17 = !3775630677_u32;
match _7 {
0 => bb2,
1 => bb3,
61334215064706877575642773751034884610 => bb5,
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
_3 = _1;
_13 = 16897642219083198759127277715907849563_u128 - 159905572148011884206147778005197001426_u128;
RET = 5325_i16 as isize;
RET = _3;
_11 = !(-4764491774483965420_i64);
_16 = (-24519_i16);
_5 = _11 as isize;
_18 = '\u{1073e}';
_18 = '\u{2e178}';
_14 = [10895353604383425655_usize,1_usize,6_usize,3_usize,14249201866611376411_usize,7_usize,3_usize,14619659066738536017_usize];
RET = _5 + _3;
_16 = _15 as i16;
_16 = 2458_i16;
_17 = 84_u8 as u32;
_12 = [16663879478389539711_usize,16789521632560461361_usize,469589966635197720_usize,10167926027425201860_usize,3_usize,2_usize,4_usize,17065161402721574319_usize];
RET = _1 ^ _1;
_16 = -1842_i16;
_14 = [3_usize,1874737020570931312_usize,3_usize,2199778250100021426_usize,0_usize,6_usize,0_usize,9428190279765945972_usize];
_15 = false;
RET = -_2;
_19 = _18;
RET = _1;
_1 = !RET;
Goto(bb6)
}
bb6 = {
_13 = 234808761985127808622868846862060393154_u128 - 185593084419594461050893824668863246623_u128;
_6 = 41394_u16 as f32;
RET = _16 as isize;
_6 = _17 as f32;
_20 = [_11,_11,_11,_11,_11];
_16 = _13 as i16;
RET = _1 << _3;
_4 = (-868964081_i32) - 1524564270_i32;
_4 = 11618119877658043149_usize as i32;
_20 = [_11,_11,_11,_11,_11];
_21 = -_3;
_10 = [_4,_4,_4,_4,_4,_4];
_2 = _21 & _21;
match _7 {
61334215064706877575642773751034884610 => bb7,
_ => bb5
}
}
bb7 = {
_4 = 262214820_i32 ^ 336660822_i32;
_1 = _4 as isize;
_13 = _2 as u128;
match _7 {
61334215064706877575642773751034884610 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_19 = _18;
_4 = _6 as i32;
_20 = [_11,_11,_11,_11,_11];
_27.fld1 = 2032798544780521204_u64;
_4 = -465019589_i32;
_24 = 6_usize as f64;
_27.fld4.fld0 = [(-11_i8),(-82_i8)];
_27.fld6.fld2 = core::ptr::addr_of_mut!(_13);
match _7 {
0 => bb10,
1 => bb11,
2 => bb12,
61334215064706877575642773751034884610 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_7 = 61334215064706877575642773751034884610_i128;
_11 = -(-4764454194083037566_i64);
_1 = '\u{a06ac}' as isize;
_3 = _5;
_10 = [_4,_4,_4,_4,_4,_4];
_8 = 13911_u16 as isize;
_1 = _5;
_11 = 7114799764337988514_i64 << RET;
RET = -_3;
_5 = _7 as isize;
_11 = -(-3096573398178030941_i64);
_6 = 189624638536909814921912764012105870214_u128 as f32;
_15 = true ^ false;
_15 = true;
_15 = false;
_17 = !3775630677_u32;
match _7 {
0 => bb2,
1 => bb3,
61334215064706877575642773751034884610 => bb5,
_ => bb4
}
}
bb12 = {
_13 = 234808761985127808622868846862060393154_u128 - 185593084419594461050893824668863246623_u128;
_6 = 41394_u16 as f32;
RET = _16 as isize;
_6 = _17 as f32;
_20 = [_11,_11,_11,_11,_11];
_16 = _13 as i16;
RET = _1 << _3;
_4 = (-868964081_i32) - 1524564270_i32;
_4 = 11618119877658043149_usize as i32;
_20 = [_11,_11,_11,_11,_11];
_21 = -_3;
_10 = [_4,_4,_4,_4,_4,_4];
_2 = _21 & _21;
match _7 {
61334215064706877575642773751034884610 => bb7,
_ => bb5
}
}
bb13 = {
_3 = _1;
_13 = 16897642219083198759127277715907849563_u128 - 159905572148011884206147778005197001426_u128;
RET = 5325_i16 as isize;
RET = _3;
_11 = !(-4764491774483965420_i64);
_16 = (-24519_i16);
_5 = _11 as isize;
_18 = '\u{1073e}';
_18 = '\u{2e178}';
_14 = [10895353604383425655_usize,1_usize,6_usize,3_usize,14249201866611376411_usize,7_usize,3_usize,14619659066738536017_usize];
RET = _5 + _3;
_16 = _15 as i16;
_16 = 2458_i16;
_17 = 84_u8 as u32;
_12 = [16663879478389539711_usize,16789521632560461361_usize,469589966635197720_usize,10167926027425201860_usize,3_usize,2_usize,4_usize,17065161402721574319_usize];
RET = _1 ^ _1;
_16 = -1842_i16;
_14 = [3_usize,1874737020570931312_usize,3_usize,2199778250100021426_usize,0_usize,6_usize,0_usize,9428190279765945972_usize];
_15 = false;
RET = -_2;
_19 = _18;
RET = _1;
_1 = !RET;
Goto(bb6)
}
bb14 = {
_27.fld3.1 = core::ptr::addr_of_mut!(_16);
_16 = -26717_i16;
_27.fld6.fld4.1 = !_4;
_16 = _6 as i16;
_11 = -(-4254856940733176607_i64);
_17 = 2276738213_u32 >> _13;
_27.fld1 = _16 as u64;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(13_usize, 15_usize, Move(_15), 20_usize, Move(_20), 8_usize, Move(_8), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(13_usize, 18_usize, Move(_18), 1_usize, Move(_1), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(13_usize, 12_usize, Move(_12), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: i128,mut _6: isize) -> isize {
mir! {
type RET = isize;
let _7: Adt42;
let _8: *mut u128;
let _9: [i8; 2];
let _10: i32;
let _11: isize;
let _12: char;
let _13: u16;
let _14: [u128; 6];
let _15: i128;
let _16: i128;
let _17: i128;
let _18: isize;
let _19: *mut u8;
let _20: [u16; 3];
let _21: [i8; 4];
let _22: [char; 4];
let _23: f64;
let _24: Adt42;
let _25: u8;
let _26: [i64; 8];
let _27: i8;
let _28: ();
let _29: ();
{
_3 = _1;
_6 = -_3;
_4 = 63510_u16 as isize;
Goto(bb1)
}
bb1 = {
_3 = (-26632_i16) as isize;
_4 = _1 << _3;
RET = 4_usize as isize;
RET = !_1;
_3 = 5968_u16 as isize;
_2 = _1 | RET;
_4 = _3;
_5 = !116650712821773976059805322473174507728_i128;
Call(_8 = fn15(_1, _6, _3, _2, _1, RET, _2, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = -_2;
RET = _6;
_5 = 136930999623601569636256457261192341599_i128;
_2 = -_3;
RET = -_2;
_5 = 1923716903_i32 as i128;
_6 = !_3;
RET = _6;
_2 = 8797_i16 as isize;
_5 = (-31358500544945235972839321919376354635_i128) & 106471574369946385816253905088572268473_i128;
_3 = !_6;
_9 = [75_i8,(-97_i8)];
_5 = !(-68466166232937110278103307090817886878_i128);
_2 = RET;
_11 = 133_u8 as isize;
Goto(bb3)
}
bb3 = {
_5 = 18079144347039243074_u64 as i128;
_3 = _11;
_2 = -_4;
_13 = 6249_u16 | 49203_u16;
_5 = !60361042066889162180535461903750393471_i128;
_10 = (-299272237_i32) + 98414834_i32;
_10 = 1396728326_i32;
_10 = (-1036837670_i32);
_15 = _5;
_14 = [134382360489579899345701187108200148999_u128,96835054236946269057998045050146014947_u128,270354998973540574369853219106929562595_u128,325449412797357813382863866795792026628_u128,102103461513898885326370545243609686494_u128,174183636619231168024745601380613004970_u128];
RET = _2 << _11;
_12 = '\u{a26f1}';
_4 = 3003078584245196013_usize as isize;
_14 = [238837479302259331238843659481826802166_u128,11856708289393680148006526454176562604_u128,292014712745826229241525933376796664254_u128,112064801766511852402439926824770924304_u128,65529372374854782647953451279670418520_u128,266877194469788621931295080739321268991_u128];
_13 = RET as u16;
_1 = 2456_i16 as isize;
RET = _6;
_2 = -_4;
_14 = [275807889237330222152198805826744184432_u128,175181330182177563232202556918396804154_u128,288552245043113532596105385346208181573_u128,60859968332284543150370137631031845787_u128,70902519294968689570695606029253740184_u128,232730828882034948658184612292101914288_u128];
match _10 {
0 => bb2,
340282366920938463463374607430731373786 => bb5,
_ => bb4
}
}
bb4 = {
_3 = (-26632_i16) as isize;
_4 = _1 << _3;
RET = 4_usize as isize;
RET = !_1;
_3 = 5968_u16 as isize;
_2 = _1 | RET;
_4 = _3;
_5 = !116650712821773976059805322473174507728_i128;
Call(_8 = fn15(_1, _6, _3, _2, _1, RET, _2, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
_10 = 277568418708556210491912168037315836085_u128 as i32;
_14 = [194962850389162101270944931523033782824_u128,251713065498797898314624090720068740452_u128,61545073125794856422791783877433763192_u128,45518871499423452275286380412628172986_u128,285676850139092000034925040156937898098_u128,115309201426930944137823758210916250371_u128];
_17 = _15 * _15;
_4 = RET;
_13 = 39370_u16;
_6 = _3 << _17;
_14 = [313776257160480250587561767868261440047_u128,162201007980002344844559448577928596842_u128,322441350673785369775916149815304646632_u128,15873665091291989970690938958180963997_u128,8854193183850628891815260341307101665_u128,280598186774461867806510721176694501675_u128];
_11 = 3803390951_u32 as isize;
match _13 {
0 => bb6,
1 => bb7,
39370 => bb9,
_ => bb8
}
}
bb6 = {
_3 = (-26632_i16) as isize;
_4 = _1 << _3;
RET = 4_usize as isize;
RET = !_1;
_3 = 5968_u16 as isize;
_2 = _1 | RET;
_4 = _3;
_5 = !116650712821773976059805322473174507728_i128;
Call(_8 = fn15(_1, _6, _3, _2, _1, RET, _2, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_5 = 18079144347039243074_u64 as i128;
_3 = _11;
_2 = -_4;
_13 = 6249_u16 | 49203_u16;
_5 = !60361042066889162180535461903750393471_i128;
_10 = (-299272237_i32) + 98414834_i32;
_10 = 1396728326_i32;
_10 = (-1036837670_i32);
_15 = _5;
_14 = [134382360489579899345701187108200148999_u128,96835054236946269057998045050146014947_u128,270354998973540574369853219106929562595_u128,325449412797357813382863866795792026628_u128,102103461513898885326370545243609686494_u128,174183636619231168024745601380613004970_u128];
RET = _2 << _11;
_12 = '\u{a26f1}';
_4 = 3003078584245196013_usize as isize;
_14 = [238837479302259331238843659481826802166_u128,11856708289393680148006526454176562604_u128,292014712745826229241525933376796664254_u128,112064801766511852402439926824770924304_u128,65529372374854782647953451279670418520_u128,266877194469788621931295080739321268991_u128];
_13 = RET as u16;
_1 = 2456_i16 as isize;
RET = _6;
_2 = -_4;
_14 = [275807889237330222152198805826744184432_u128,175181330182177563232202556918396804154_u128,288552245043113532596105385346208181573_u128,60859968332284543150370137631031845787_u128,70902519294968689570695606029253740184_u128,232730828882034948658184612292101914288_u128];
match _10 {
0 => bb2,
340282366920938463463374607430731373786 => bb5,
_ => bb4
}
}
bb8 = {
_6 = -_2;
RET = _6;
_5 = 136930999623601569636256457261192341599_i128;
_2 = -_3;
RET = -_2;
_5 = 1923716903_i32 as i128;
_6 = !_3;
RET = _6;
_2 = 8797_i16 as isize;
_5 = (-31358500544945235972839321919376354635_i128) & 106471574369946385816253905088572268473_i128;
_3 = !_6;
_9 = [75_i8,(-97_i8)];
_5 = !(-68466166232937110278103307090817886878_i128);
_2 = RET;
_11 = 133_u8 as isize;
Goto(bb3)
}
bb9 = {
_14 = [218775208298983840724222449109990644076_u128,101254413841275791927600347307068911588_u128,301557495443407126308105184234976271634_u128,291601359272829388894403243798254917849_u128,179539196570015441490783360800723458430_u128,239853741123604884824752342200162182190_u128];
_5 = _17;
_6 = _2 + _4;
Goto(bb10)
}
bb10 = {
RET = 1057964586967124267_i64 as isize;
_1 = _13 as isize;
_3 = RET & RET;
_13 = (-87_i8) as u16;
_10 = 1290392932_i32 << _4;
_9 = [(-52_i8),98_i8];
_13 = 42386_u16 - 20774_u16;
_2 = _6;
_6 = _2;
_1 = 2538752099_u32 as isize;
_12 = '\u{a87d1}';
_13 = !35527_u16;
_9 = [(-70_i8),(-124_i8)];
_18 = _6;
Goto(bb11)
}
bb11 = {
_4 = _1;
_16 = !_5;
_13 = 16798_u16 << _3;
Goto(bb12)
}
bb12 = {
_3 = -_6;
RET = -_11;
_22 = [_12,_12,_12,_12];
_3 = _12 as isize;
_10 = !819251059_i32;
Call(_21 = core::intrinsics::transmute(_10), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = !_11;
_6 = 64_i8 as isize;
_2 = _18;
_22 = [_12,_12,_12,_12];
_3 = _10 as isize;
_12 = '\u{f64c9}';
RET = _4;
_10 = _13 as i32;
_20 = [_13,_13,_13];
_20 = [_13,_13,_13];
_23 = 2138696620020381825_i64 as f64;
RET = _4;
_1 = _18;
_10 = (-700271702_i32);
_14 = [113057331356038197745125430484240422149_u128,24668623855822245167854294746956033112_u128,319994806980341875274399767129723192785_u128,295519931401592739305563340202870226914_u128,233887950556683197439780145801397878517_u128,227566464838489555274944729929771390644_u128];
_10 = !(-596675135_i32);
_3 = _10 as isize;
_22 = [_12,_12,_12,_12];
_6 = _18 & _11;
_13 = 52137_u16 * 6803_u16;
_25 = _10 as u8;
_5 = !_16;
Goto(bb14)
}
bb14 = {
_12 = '\u{5d272}';
_9 = [51_i8,(-32_i8)];
_11 = _18;
_5 = -_16;
_18 = 6175127463248525300_u64 as isize;
RET = -_18;
_6 = (-10684_i16) as isize;
_25 = 137_u8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(14_usize, 11_usize, Move(_11), 2_usize, Move(_2), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(14_usize, 5_usize, Move(_5), 20_usize, Move(_20), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(14_usize, 25_usize, Move(_25), 17_usize, Move(_17), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> *mut u128 {
mir! {
type RET = *mut u128;
let _9: u16;
let _10: &'static i64;
let _11: ([i8; 2], *mut u8);
let _12: Adt55;
let _13: bool;
let _14: f64;
let _15: isize;
let _16: [i8; 2];
let _17: f32;
let _18: f32;
let _19: Adt45;
let _20: f32;
let _21: *mut u128;
let _22: ([i8; 2], *mut u8);
let _23: (*mut u32, *mut i16);
let _24: bool;
let _25: bool;
let _26: [u16; 3];
let _27: f32;
let _28: isize;
let _29: [u16; 3];
let _30: f32;
let _31: Adt51;
let _32: i64;
let _33: [u16; 3];
let _34: [i64; 5];
let _35: i64;
let _36: i128;
let _37: [bool; 5];
let _38: f64;
let _39: [i8; 4];
let _40: isize;
let _41: [i8; 4];
let _42: f64;
let _43: isize;
let _44: [u128; 6];
let _45: (i32,);
let _46: u128;
let _47: [u16; 3];
let _48: [u128; 6];
let _49: u8;
let _50: u64;
let _51: bool;
let _52: [usize; 8];
let _53: char;
let _54: isize;
let _55: f32;
let _56: isize;
let _57: ();
let _58: ();
{
_6 = _4;
_1 = _8 + _5;
_5 = _7 | _4;
_8 = _6 ^ _5;
_2 = (-27318_i16) as isize;
_7 = _5;
_7 = _8 << _1;
_6 = -_8;
_5 = _6;
_2 = _4;
_2 = -_7;
_7 = -_6;
_9 = !27277_u16;
_2 = _6 - _5;
_4 = _9 as isize;
_6 = 2717660964777715964_u64 as isize;
_5 = -_4;
_3 = -_8;
_11.0 = [(-103_i8),92_i8];
_1 = '\u{d1c3a}' as isize;
_8 = _3 ^ _3;
_8 = !_2;
_1 = _6;
_11.0 = [(-31_i8),99_i8];
_2 = _8 | _3;
_12.fld0 = [6286338076282546737_i64,(-8356428130917300611_i64),6498281039075146333_i64,6758644783744622901_i64,(-9213165220327837626_i64)];
Call(_11.1 = fn16(_3, _8, _2, _2, _11.0, _8, _11.0, _2, _3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld5 = [_9,_9,_9];
_12.fld4.fld0 = ['\u{172ec}','\u{f5286}','\u{10b7c3}','\u{10b841}'];
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld6.fld4.1 = 941622127_i32 * (-1295568883_i32);
_12.fld0 = [(-1385916769833772393_i64),6714969129317091744_i64,(-5390047101123005784_i64),(-2380662562615681276_i64),(-8266066405141616600_i64)];
_1 = _2;
_12.fld4.fld1 = '\u{ad734}' as u64;
_12.fld4.fld6.fld4.0 = '\u{1051ed}' as usize;
_2 = (-127_i8) as isize;
_12.fld4.fld6.fld0 = _12.fld4.fld6.fld4.1 | _12.fld4.fld6.fld4.1;
_12.fld3 = (-117_i8);
_3 = _1 << _1;
_13 = !false;
_12.fld4.fld6.fld1 = [_12.fld4.fld6.fld0,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1];
_12.fld4.fld7 = _9 as f64;
_12.fld4.fld7 = _12.fld4.fld6.fld4.1 as f64;
_5 = _8;
_5 = _3 ^ _3;
_7 = 5454817961913028728_i64 as isize;
_12.fld4.fld0 = ['\u{a9dbb}','\u{57a47}','\u{e4854}','\u{cb461}'];
_13 = !true;
_8 = -_3;
Goto(bb2)
}
bb2 = {
_4 = !_5;
_11.1 = _12.fld4.fld4.fld4;
_14 = _12.fld4.fld6.fld4.1 as f64;
_8 = 47808291227948186822727327645373427962_i128 as isize;
_12.fld4.fld0 = ['\u{e3d03}','\u{2e540}','\u{dbc24}','\u{77d2f}'];
_6 = _4;
_12.fld4.fld4.fld4 = _11.1;
_3 = _12.fld3 as isize;
_7 = _4;
_12.fld4.fld6.fld4.1 = 0_u8 as i32;
_13 = _4 == _6;
_12.fld2 = !_12.fld4.fld6.fld0;
_13 = _6 > _6;
_12.fld4.fld6.fld4.0 = 4_usize;
_12.fld4.fld6.fld4.1 = _12.fld2 | _12.fld2;
_12.fld2 = _12.fld4.fld6.fld4.1;
_11.0 = [_12.fld3,_12.fld3];
_6 = !_4;
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld4.fld2 = -_7;
_11.0 = [_12.fld3,_12.fld3];
_5 = 78_u8 as isize;
_15 = _12.fld4.fld4.fld2 << _7;
_12.fld4.fld6.fld4 = (12344369482611543671_usize, _12.fld2);
_12.fld4.fld4.fld0 = [_12.fld3,_12.fld3];
match _12.fld3 {
340282366920938463463374607431768211339 => bb3,
_ => bb1
}
}
bb3 = {
_6 = (-4407_i16) as isize;
_2 = 135627274787692404621940933499802792199_u128 as isize;
match _12.fld4.fld6.fld4.0 {
0 => bb1,
12344369482611543671 => bb5,
_ => bb4
}
}
bb4 = {
_4 = !_5;
_11.1 = _12.fld4.fld4.fld4;
_14 = _12.fld4.fld6.fld4.1 as f64;
_8 = 47808291227948186822727327645373427962_i128 as isize;
_12.fld4.fld0 = ['\u{e3d03}','\u{2e540}','\u{dbc24}','\u{77d2f}'];
_6 = _4;
_12.fld4.fld4.fld4 = _11.1;
_3 = _12.fld3 as isize;
_7 = _4;
_12.fld4.fld6.fld4.1 = 0_u8 as i32;
_13 = _4 == _6;
_12.fld2 = !_12.fld4.fld6.fld0;
_13 = _6 > _6;
_12.fld4.fld6.fld4.0 = 4_usize;
_12.fld4.fld6.fld4.1 = _12.fld2 | _12.fld2;
_12.fld2 = _12.fld4.fld6.fld4.1;
_11.0 = [_12.fld3,_12.fld3];
_6 = !_4;
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld4.fld2 = -_7;
_11.0 = [_12.fld3,_12.fld3];
_5 = 78_u8 as isize;
_15 = _12.fld4.fld4.fld2 << _7;
_12.fld4.fld6.fld4 = (12344369482611543671_usize, _12.fld2);
_12.fld4.fld4.fld0 = [_12.fld3,_12.fld3];
match _12.fld3 {
340282366920938463463374607431768211339 => bb3,
_ => bb1
}
}
bb5 = {
_12.fld4.fld6.fld1 = [_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_12.fld2,_12.fld2,_12.fld4.fld6.fld0];
_12.fld2 = _12.fld4.fld6.fld4.1 + _12.fld4.fld6.fld4.1;
_11 = (_12.fld4.fld4.fld0, _12.fld4.fld4.fld4);
_2 = 127_u8 as isize;
_12.fld2 = _12.fld4.fld6.fld0;
_12.fld4.fld4.fld4 = _11.1;
_13 = _7 <= _15;
_3 = _4 - _4;
_12.fld4.fld6.fld4.0 = 6083322199059286667_usize + 3_usize;
_2 = _15 | _3;
_6 = !_3;
_12.fld4.fld7 = _14;
_12.fld4.fld4.fld4 = _11.1;
_8 = !_12.fld4.fld4.fld2;
_1 = _3 << _12.fld4.fld4.fld2;
_12.fld3 = 124_i8;
_12.fld4.fld7 = _14 + _14;
_2 = _3;
_16 = [_12.fld3,_12.fld3];
_6 = _4 - _7;
_12.fld4.fld0 = ['\u{c02eb}','\u{66158}','\u{64cf}','\u{dc15d}'];
_18 = 37166300_u32 as f32;
_4 = !_8;
_5 = _15;
Goto(bb6)
}
bb6 = {
_12.fld4.fld6.fld0 = _12.fld4.fld6.fld4.1 * _12.fld2;
_22.1 = _12.fld4.fld4.fld4;
_12.fld4.fld7 = _14 - _14;
_17 = _18 - _18;
_12.fld4.fld7 = _14 + _14;
_12.fld4.fld4.fld0 = [_12.fld3,_12.fld3];
_7 = 1281348942_u32 as isize;
match _12.fld3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
124 => bb9,
_ => bb8
}
}
bb7 = {
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld5 = [_9,_9,_9];
_12.fld4.fld0 = ['\u{172ec}','\u{f5286}','\u{10b7c3}','\u{10b841}'];
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld6.fld4.1 = 941622127_i32 * (-1295568883_i32);
_12.fld0 = [(-1385916769833772393_i64),6714969129317091744_i64,(-5390047101123005784_i64),(-2380662562615681276_i64),(-8266066405141616600_i64)];
_1 = _2;
_12.fld4.fld1 = '\u{ad734}' as u64;
_12.fld4.fld6.fld4.0 = '\u{1051ed}' as usize;
_2 = (-127_i8) as isize;
_12.fld4.fld6.fld0 = _12.fld4.fld6.fld4.1 | _12.fld4.fld6.fld4.1;
_12.fld3 = (-117_i8);
_3 = _1 << _1;
_13 = !false;
_12.fld4.fld6.fld1 = [_12.fld4.fld6.fld0,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1];
_12.fld4.fld7 = _9 as f64;
_12.fld4.fld7 = _12.fld4.fld6.fld4.1 as f64;
_5 = _8;
_5 = _3 ^ _3;
_7 = 5454817961913028728_i64 as isize;
_12.fld4.fld0 = ['\u{a9dbb}','\u{57a47}','\u{e4854}','\u{cb461}'];
_13 = !true;
_8 = -_3;
Goto(bb2)
}
bb8 = {
_4 = !_5;
_11.1 = _12.fld4.fld4.fld4;
_14 = _12.fld4.fld6.fld4.1 as f64;
_8 = 47808291227948186822727327645373427962_i128 as isize;
_12.fld4.fld0 = ['\u{e3d03}','\u{2e540}','\u{dbc24}','\u{77d2f}'];
_6 = _4;
_12.fld4.fld4.fld4 = _11.1;
_3 = _12.fld3 as isize;
_7 = _4;
_12.fld4.fld6.fld4.1 = 0_u8 as i32;
_13 = _4 == _6;
_12.fld2 = !_12.fld4.fld6.fld0;
_13 = _6 > _6;
_12.fld4.fld6.fld4.0 = 4_usize;
_12.fld4.fld6.fld4.1 = _12.fld2 | _12.fld2;
_12.fld2 = _12.fld4.fld6.fld4.1;
_11.0 = [_12.fld3,_12.fld3];
_6 = !_4;
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld4.fld2 = -_7;
_11.0 = [_12.fld3,_12.fld3];
_5 = 78_u8 as isize;
_15 = _12.fld4.fld4.fld2 << _7;
_12.fld4.fld6.fld4 = (12344369482611543671_usize, _12.fld2);
_12.fld4.fld4.fld0 = [_12.fld3,_12.fld3];
match _12.fld3 {
340282366920938463463374607431768211339 => bb3,
_ => bb1
}
}
bb9 = {
_5 = _12.fld3 as isize;
_20 = _12.fld4.fld4.fld2 as f32;
_7 = _13 as isize;
_12.fld2 = _20 as i32;
_12.fld4.fld6.fld1 = [_12.fld2,_12.fld4.fld6.fld4.1,_12.fld2,_12.fld2,_12.fld2,_12.fld2];
_6 = _15 << _2;
_11.0 = _12.fld4.fld4.fld0;
_12.fld3 = _14 as i8;
_22.0 = [_12.fld3,_12.fld3];
_12.fld4.fld6.fld1 = [_12.fld2,_12.fld2,_12.fld2,_12.fld2,_12.fld2,_12.fld2];
_11 = _22;
_16 = _12.fld4.fld4.fld0;
_22.1 = _11.1;
_5 = _8 << _15;
_12.fld4.fld6.fld0 = -_12.fld2;
_15 = !_4;
_2 = _4;
_13 = true | false;
_11.0 = [_12.fld3,_12.fld3];
_17 = _20 * _18;
_12.fld4.fld5 = [_9,_9,_9];
_11.0 = _22.0;
_3 = !_2;
_24 = !_13;
_12.fld4.fld6.fld4.1 = _12.fld4.fld6.fld0 | _12.fld2;
_12.fld4.fld6.fld4.0 = !4899032685417312155_usize;
_18 = -_17;
Goto(bb10)
}
bb10 = {
_11 = (_22.0, _12.fld4.fld4.fld4);
_12.fld2 = _12.fld4.fld6.fld4.1;
_12.fld3 = 50_i8;
_12.fld4.fld6.fld4.0 = 16381674016298121318_usize + 7_usize;
_11.0 = _12.fld4.fld4.fld0;
_11.1 = _22.1;
_22.0 = [_12.fld3,_12.fld3];
_12.fld4.fld6.fld4 = (4344708850206056996_usize, _12.fld2);
_5 = _4 >> _12.fld2;
_29 = [_9,_9,_9];
_1 = _2 | _5;
_7 = 236_u8 as isize;
_12.fld4.fld7 = _14;
_20 = -_18;
_12.fld2 = _9 as i32;
_22.1 = _11.1;
_22.1 = _11.1;
_28 = _12.fld4.fld6.fld4.1 as isize;
_12.fld4.fld6.fld0 = _12.fld4.fld6.fld4.1;
_13 = _24;
_9 = (-8651470693759619623_i64) as u16;
match _12.fld4.fld6.fld4.0 {
0 => bb9,
1 => bb4,
2 => bb11,
4344708850206056996 => bb13,
_ => bb12
}
}
bb11 = {
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld5 = [_9,_9,_9];
_12.fld4.fld0 = ['\u{172ec}','\u{f5286}','\u{10b7c3}','\u{10b841}'];
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld6.fld4.1 = 941622127_i32 * (-1295568883_i32);
_12.fld0 = [(-1385916769833772393_i64),6714969129317091744_i64,(-5390047101123005784_i64),(-2380662562615681276_i64),(-8266066405141616600_i64)];
_1 = _2;
_12.fld4.fld1 = '\u{ad734}' as u64;
_12.fld4.fld6.fld4.0 = '\u{1051ed}' as usize;
_2 = (-127_i8) as isize;
_12.fld4.fld6.fld0 = _12.fld4.fld6.fld4.1 | _12.fld4.fld6.fld4.1;
_12.fld3 = (-117_i8);
_3 = _1 << _1;
_13 = !false;
_12.fld4.fld6.fld1 = [_12.fld4.fld6.fld0,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1];
_12.fld4.fld7 = _9 as f64;
_12.fld4.fld7 = _12.fld4.fld6.fld4.1 as f64;
_5 = _8;
_5 = _3 ^ _3;
_7 = 5454817961913028728_i64 as isize;
_12.fld4.fld0 = ['\u{a9dbb}','\u{57a47}','\u{e4854}','\u{cb461}'];
_13 = !true;
_8 = -_3;
Goto(bb2)
}
bb12 = {
_4 = !_5;
_11.1 = _12.fld4.fld4.fld4;
_14 = _12.fld4.fld6.fld4.1 as f64;
_8 = 47808291227948186822727327645373427962_i128 as isize;
_12.fld4.fld0 = ['\u{e3d03}','\u{2e540}','\u{dbc24}','\u{77d2f}'];
_6 = _4;
_12.fld4.fld4.fld4 = _11.1;
_3 = _12.fld3 as isize;
_7 = _4;
_12.fld4.fld6.fld4.1 = 0_u8 as i32;
_13 = _4 == _6;
_12.fld2 = !_12.fld4.fld6.fld0;
_13 = _6 > _6;
_12.fld4.fld6.fld4.0 = 4_usize;
_12.fld4.fld6.fld4.1 = _12.fld2 | _12.fld2;
_12.fld2 = _12.fld4.fld6.fld4.1;
_11.0 = [_12.fld3,_12.fld3];
_6 = !_4;
_12.fld4.fld4.fld4 = _11.1;
_12.fld4.fld4.fld2 = -_7;
_11.0 = [_12.fld3,_12.fld3];
_5 = 78_u8 as isize;
_15 = _12.fld4.fld4.fld2 << _7;
_12.fld4.fld6.fld4 = (12344369482611543671_usize, _12.fld2);
_12.fld4.fld4.fld0 = [_12.fld3,_12.fld3];
match _12.fld3 {
340282366920938463463374607431768211339 => bb3,
_ => bb1
}
}
bb13 = {
_28 = _12.fld4.fld6.fld4.0 as isize;
_32 = 8405214275332671523_i64;
_2 = !_1;
Goto(bb14)
}
bb14 = {
_12.fld4.fld6.fld4.0 = '\u{19439}' as usize;
_12.fld4.fld6.fld1 = [_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld0,_12.fld4.fld6.fld0,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld0];
_27 = -_17;
_30 = -_18;
_12.fld4.fld6.fld4.0 = !4_usize;
_11 = (_16, _12.fld4.fld4.fld4);
_15 = _28;
_9 = 46538695608461749322854150248950010664_i128 as u16;
_12.fld4.fld4.fld0 = _16;
_18 = _20 * _17;
_5 = 1667573962_u32 as isize;
_11.0 = _12.fld4.fld4.fld0;
_20 = _30 - _17;
Goto(bb15)
}
bb15 = {
_12.fld4.fld5 = [_9,_9,_9];
_25 = !_24;
_12.fld4.fld6.fld4.0 = 6_usize;
_11 = _22;
_11 = _22;
_34 = [_32,_32,_32,_32,_32];
_18 = _20 * _17;
_12.fld4.fld5 = [_9,_9,_9];
_28 = _20 as isize;
_20 = _18;
_12.fld4.fld0 = ['\u{828af}','\u{2511d}','\u{f21f}','\u{826bb}'];
_8 = -_1;
_5 = -_8;
_24 = _13 ^ _25;
_11.1 = _22.1;
_28 = -_15;
Call(_12.fld4.fld5 = core::intrinsics::transmute(_29), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_38 = _12.fld4.fld7 - _14;
_12.fld4.fld7 = -_38;
_9 = 53171_u16 + 3648_u16;
_11.0 = [_12.fld3,_12.fld3];
Call(_18 = core::intrinsics::transmute(_12.fld4.fld6.fld0), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_6 = !_15;
_12.fld2 = _12.fld4.fld6.fld4.1 >> _8;
_12.fld4.fld1 = 17060595750880779411_u64 + 15587612163619481438_u64;
_12.fld4.fld6.fld4.1 = _12.fld4.fld1 as i32;
_26 = _29;
_27 = _20;
_14 = -_38;
_29 = [_9,_9,_9];
_8 = _2 & _2;
_30 = -_18;
_43 = !_4;
_33 = [_9,_9,_9];
_15 = _8 - _2;
match _12.fld4.fld6.fld4.0 {
0 => bb14,
1 => bb7,
2 => bb3,
3 => bb15,
6 => bb18,
_ => bb13
}
}
bb18 = {
_12.fld4.fld6.fld4.1 = -_12.fld4.fld6.fld0;
_26 = _12.fld4.fld5;
_21 = core::ptr::addr_of_mut!(_46);
_9 = 35542_u16 ^ 3454_u16;
_11.0 = [_12.fld3,_12.fld3];
_11.1 = _12.fld4.fld4.fld4;
_45 = (_12.fld2,);
_12.fld4.fld4.fld2 = _15;
_45.0 = -_12.fld4.fld6.fld0;
_12.fld4.fld6.fld4 = (7429146426749383915_usize, _45.0);
_12.fld4.fld6.fld4.1 = !_12.fld2;
_12.fld4.fld6.fld1 = [_12.fld4.fld6.fld0,_12.fld2,_12.fld2,_12.fld4.fld6.fld0,_12.fld2,_12.fld2];
_40 = !_6;
_17 = _12.fld4.fld6.fld4.0 as f32;
_11.1 = _12.fld4.fld4.fld4;
_8 = 1004564578_u32 as isize;
_40 = _1;
_12.fld0 = [_32,_32,_32,_32,_32];
_46 = 100695934168230903702703206467402083141_u128;
_50 = _12.fld2 as u64;
_14 = _12.fld4.fld7;
_44 = [(*_21),_46,(*_21),(*_21),_46,(*_21)];
_5 = (-21600_i16) as isize;
Goto(bb19)
}
bb19 = {
_38 = 78260381349775827944356116904786098987_i128 as f64;
_12.fld4.fld6.fld0 = 3564182388_u32 as i32;
_49 = !79_u8;
_15 = _28 & _2;
_38 = _14;
_26 = [_9,_9,_9];
_12.fld4.fld6.fld1 = [_12.fld2,_45.0,_12.fld4.fld6.fld4.1,_12.fld4.fld6.fld4.1,_45.0,_45.0];
_40 = _32 as isize;
_12.fld4.fld4.fld4 = _11.1;
_2 = -_4;
RET = _21;
(*RET) = 51095474507754792216998607092837287829_u128 & 316140788134112655527891504037004008270_u128;
_8 = _28;
_46 = 305402621680186118579688222864267891282_u128;
_12.fld4.fld4.fld0 = [_12.fld3,_12.fld3];
_12.fld3 = (-62_i8);
_49 = 113_u8;
_2 = _8 * _1;
_49 = 133_u8 >> _43;
_47 = _29;
_12.fld4.fld6.fld2 = RET;
(*RET) = '\u{e5d9f}' as u128;
_22.0 = [_12.fld3,_12.fld3];
_20 = _32 as f32;
Goto(bb20)
}
bb20 = {
Call(_57 = dump_var(15_usize, 47_usize, Move(_47), 15_usize, Move(_15), 24_usize, Move(_24), 40_usize, Move(_40)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_57 = dump_var(15_usize, 49_usize, Move(_49), 44_usize, Move(_44), 25_usize, Move(_25), 28_usize, Move(_28)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_57 = dump_var(15_usize, 29_usize, Move(_29), 16_usize, Move(_16), 50_usize, Move(_50), 4_usize, Move(_4)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_57 = dump_var(15_usize, 6_usize, Move(_6), 26_usize, Move(_26), 58_usize, _58, 58_usize, _58), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [i8; 2],mut _6: isize,mut _7: [i8; 2],mut _8: isize,mut _9: isize,mut _10: isize) -> *mut u8 {
mir! {
type RET = *mut u8;
let _11: Adt48;
let _12: bool;
let _13: i32;
let _14: u32;
let _15: Adt43;
let _16: bool;
let _17: [bool; 5];
let _18: Adt44;
let _19: u32;
let _20: char;
let _21: usize;
let _22: bool;
let _23: isize;
let _24: char;
let _25: char;
let _26: usize;
let _27: u8;
let _28: char;
let _29: f64;
let _30: [u128; 6];
let _31: Adt45;
let _32: f64;
let _33: char;
let _34: isize;
let _35: f32;
let _36: f64;
let _37: isize;
let _38: [char; 4];
let _39: [char; 4];
let _40: u8;
let _41: f32;
let _42: Adt47;
let _43: Adt55;
let _44: (([i8; 4],), i128, [u128; 6]);
let _45: i128;
let _46: isize;
let _47: i32;
let _48: [char; 4];
let _49: ([i8; 4],);
let _50: bool;
let _51: ();
let _52: ();
{
_3 = 278747753100840737275996601217230753688_u128 as isize;
_9 = 14502998665688176499_u64 as isize;
_4 = _1 * _1;
_5 = [19_i8,(-119_i8)];
_11 = Adt48::Variant1 { fld0: 4807207987825058666_i64 };
_13 = 121_i8 as i32;
place!(Field::<i64>(Variant(_11, 1), 0)) = (-6463482617091585151_i64) | 3920082162764550502_i64;
_2 = -_10;
_8 = (-95_i8) as isize;
_12 = false;
place!(Field::<i64>(Variant(_11, 1), 0)) = 6353848782235393575_i64;
_13 = -(-1045493173_i32);
_7 = [106_i8,90_i8];
_15.fld2 = 3_usize as isize;
_9 = -_2;
_14 = 135983756_u32 | 1893480858_u32;
_10 = -_9;
_12 = _2 != _4;
SetDiscriminant(_11, 3);
_8 = _15.fld2;
place!(Field::<[usize; 8]>(Variant(_11, 3), 0)) = [1555262123472736472_usize,16592834657488186454_usize,15497115373638106305_usize,6_usize,6115422544922748251_usize,10555944929231126573_usize,3_usize,10925939564807918830_usize];
_2 = _4;
_4 = _6 - _10;
_16 = _12;
Goto(bb1)
}
bb1 = {
_6 = 21_u8 as isize;
_14 = 4028375178673219994_u64 as u32;
_16 = !_12;
_7 = [69_i8,11_i8];
_15.fld0 = [(-36_i8),(-108_i8)];
_17 = [_16,_16,_16,_16,_12];
_1 = _10 + _10;
_17 = [_16,_12,_12,_12,_12];
place!(Field::<[i8; 4]>(Variant(_11, 3), 4)) = [90_i8,(-92_i8),(-111_i8),92_i8];
place!(Field::<[usize; 8]>(Variant(_11, 3), 0)) = [1305413293999917952_usize,410702311265843281_usize,6_usize,3_usize,1_usize,7513155385627069356_usize,2_usize,0_usize];
_9 = _1 >> _4;
_4 = _9 << _15.fld2;
_16 = _12;
place!(Field::<i64>(Variant(_11, 3), 3)) = 5900908880955045909_i64 * 7319482269532099019_i64;
_3 = _1;
place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = [(-79_i8),120_i8,(-1_i8),(-5_i8)];
_7 = _5;
place!(Field::<i64>(Variant(_11, 3), 3)) = 7892372694462569987_i64;
_17 = [_16,_16,_16,_16,_16];
place!(Field::<[i8; 4]>(Variant(_11, 3), 4)) = Field::<([i8; 4],)>(Variant(_11, 3), 1).0;
_3 = (-29282_i16) as isize;
_15.fld3 = Adt42::Variant2 { fld0: Field::<[usize; 8]>(Variant(_11, 3), 0),fld1: Field::<i64>(Variant(_11, 3), 3) };
place!(Field::<i64>(Variant(_11, 3), 3)) = Field::<i64>(Variant(_15.fld3, 2), 1) >> _4;
_22 = _12;
_5 = [(-60_i8),94_i8];
Call(place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = core::intrinsics::transmute(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_22 = !_16;
place!(Field::<i64>(Variant(_15.fld3, 2), 1)) = 33786653308828427318561142568366142100_u128 as i64;
_15.fld2 = -_2;
place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = [49_i8,(-96_i8),(-23_i8),(-72_i8)];
_17 = [_22,_16,_12,_16,_22];
_22 = _16;
_20 = '\u{ddfea}';
place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = [(-7_i8),11_i8,(-123_i8),(-44_i8)];
Goto(bb3)
}
bb3 = {
_14 = !2819719017_u32;
place!(Field::<[usize; 8]>(Variant(_11, 3), 0)) = [13804968732730236708_usize,14697887257044627335_usize,6_usize,6_usize,6_usize,899381057255439036_usize,2_usize,17855929462922748777_usize];
_17 = [_12,_12,_16,_12,_12];
_15.fld2 = _1 - _1;
_24 = _20;
_19 = !_14;
SetDiscriminant(_15.fld3, 0);
_22 = _1 > _15.fld2;
place!(Field::<*const usize>(Variant(_11, 3), 5)) = core::ptr::addr_of!(_21);
RET = core::ptr::addr_of_mut!(_27);
_19 = _14 + _14;
_8 = _2;
_13 = 1698881820_i32 - 303106204_i32;
_10 = 34752_u16 as isize;
RET = core::ptr::addr_of_mut!((*RET));
Call(RET = fn17(_4, _15.fld2, _4, _1, _17, _1, _4, _17, _1, Field::<i64>(Variant(_11, 3), 3), _4, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = core::ptr::addr_of_mut!(_27);
place!(Field::<(usize, i32)>(Variant(_15.fld3, 0), 5)) = (5_usize, _13);
place!(Field::<i128>(Variant(_15.fld3, 0), 3)) = (-84907039734908571195137477679281462554_i128) + (-49419987550755932125569497946079930524_i128);
place!(Field::<[i8; 4]>(Variant(_15.fld3, 0), 4)) = [40_i8,(-66_i8),(-44_i8),11_i8];
_27 = 188_u8 - 133_u8;
place!(Field::<i128>(Variant(_15.fld3, 0), 3)) = !(-162418943091338255991591505560277035753_i128);
_25 = _24;
place!(Field::<([i8; 2], *mut u8)>(Variant(_15.fld3, 0), 1)).0 = [(-84_i8),45_i8];
_7 = [(-104_i8),(-30_i8)];
place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)) = (Field::<[i8; 4]>(Variant(_11, 3), 4),);
_4 = _1 >> _15.fld2;
_15.fld4 = core::ptr::addr_of_mut!((*RET));
_16 = !_22;
_12 = _16;
_24 = _20;
_11 = Adt48::Variant1 { fld0: 7126955725660688809_i64 };
_15.fld4 = core::ptr::addr_of_mut!((*RET));
_26 = !Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).0;
_9 = _4;
_29 = Field::<i128>(Variant(_15.fld3, 0), 3) as f64;
match Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).0 {
0 => bb5,
5 => bb7,
_ => bb6
}
}
bb5 = {
_14 = !2819719017_u32;
place!(Field::<[usize; 8]>(Variant(_11, 3), 0)) = [13804968732730236708_usize,14697887257044627335_usize,6_usize,6_usize,6_usize,899381057255439036_usize,2_usize,17855929462922748777_usize];
_17 = [_12,_12,_16,_12,_12];
_15.fld2 = _1 - _1;
_24 = _20;
_19 = !_14;
SetDiscriminant(_15.fld3, 0);
_22 = _1 > _15.fld2;
place!(Field::<*const usize>(Variant(_11, 3), 5)) = core::ptr::addr_of!(_21);
RET = core::ptr::addr_of_mut!(_27);
_19 = _14 + _14;
_8 = _2;
_13 = 1698881820_i32 - 303106204_i32;
_10 = 34752_u16 as isize;
RET = core::ptr::addr_of_mut!((*RET));
Call(RET = fn17(_4, _15.fld2, _4, _1, _17, _1, _4, _17, _1, Field::<i64>(Variant(_11, 3), 3), _4, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_6 = 21_u8 as isize;
_14 = 4028375178673219994_u64 as u32;
_16 = !_12;
_7 = [69_i8,11_i8];
_15.fld0 = [(-36_i8),(-108_i8)];
_17 = [_16,_16,_16,_16,_12];
_1 = _10 + _10;
_17 = [_16,_12,_12,_12,_12];
place!(Field::<[i8; 4]>(Variant(_11, 3), 4)) = [90_i8,(-92_i8),(-111_i8),92_i8];
place!(Field::<[usize; 8]>(Variant(_11, 3), 0)) = [1305413293999917952_usize,410702311265843281_usize,6_usize,3_usize,1_usize,7513155385627069356_usize,2_usize,0_usize];
_9 = _1 >> _4;
_4 = _9 << _15.fld2;
_16 = _12;
place!(Field::<i64>(Variant(_11, 3), 3)) = 5900908880955045909_i64 * 7319482269532099019_i64;
_3 = _1;
place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = [(-79_i8),120_i8,(-1_i8),(-5_i8)];
_7 = _5;
place!(Field::<i64>(Variant(_11, 3), 3)) = 7892372694462569987_i64;
_17 = [_16,_16,_16,_16,_16];
place!(Field::<[i8; 4]>(Variant(_11, 3), 4)) = Field::<([i8; 4],)>(Variant(_11, 3), 1).0;
_3 = (-29282_i16) as isize;
_15.fld3 = Adt42::Variant2 { fld0: Field::<[usize; 8]>(Variant(_11, 3), 0),fld1: Field::<i64>(Variant(_11, 3), 3) };
place!(Field::<i64>(Variant(_11, 3), 3)) = Field::<i64>(Variant(_15.fld3, 2), 1) >> _4;
_22 = _12;
_5 = [(-60_i8),94_i8];
Call(place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = core::intrinsics::transmute(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
place!(Field::<f64>(Variant(_15.fld3, 0), 2)) = _29 + _29;
_13 = !Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).1;
_17 = [_12,_22,_22,_16,_12];
_20 = _24;
_20 = _24;
_15.fld4 = RET;
RET = _15.fld4;
place!(Field::<([i8; 2], *mut u8)>(Variant(_15.fld3, 0), 1)) = (_15.fld0, _15.fld4);
_25 = _24;
place!(Field::<[i8; 4]>(Variant(_15.fld3, 0), 4)) = [(-86_i8),23_i8,11_i8,(-54_i8)];
(*RET) = 184_u8 * 50_u8;
place!(Field::<(usize, i32)>(Variant(_15.fld3, 0), 5)).0 = _26 >> _4;
place!(Field::<i64>(Variant(_11, 1), 0)) = -(-5286422921921704853_i64);
place!(Field::<i128>(Variant(_15.fld3, 0), 3)) = (-10600287911311552915740803046074312008_i128);
_1 = _4 * _9;
_30 = [232801471373605798485533350661446722622_u128,200039448725654260664724799442375490427_u128,23586530541537609553443863629861791168_u128,49764183391489752813390630021340996599_u128,32714963546246780222628173793453391064_u128,301842656167867920290125540452119540908_u128];
_2 = _1 >> Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).0;
_15.fld2 = _2 + _4;
_23 = 87_i8 as isize;
match Field::<i128>(Variant(_15.fld3, 0), 3) {
329682079009626910547633804385693899448 => bb9,
_ => bb8
}
}
bb8 = {
_6 = 21_u8 as isize;
_14 = 4028375178673219994_u64 as u32;
_16 = !_12;
_7 = [69_i8,11_i8];
_15.fld0 = [(-36_i8),(-108_i8)];
_17 = [_16,_16,_16,_16,_12];
_1 = _10 + _10;
_17 = [_16,_12,_12,_12,_12];
place!(Field::<[i8; 4]>(Variant(_11, 3), 4)) = [90_i8,(-92_i8),(-111_i8),92_i8];
place!(Field::<[usize; 8]>(Variant(_11, 3), 0)) = [1305413293999917952_usize,410702311265843281_usize,6_usize,3_usize,1_usize,7513155385627069356_usize,2_usize,0_usize];
_9 = _1 >> _4;
_4 = _9 << _15.fld2;
_16 = _12;
place!(Field::<i64>(Variant(_11, 3), 3)) = 5900908880955045909_i64 * 7319482269532099019_i64;
_3 = _1;
place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = [(-79_i8),120_i8,(-1_i8),(-5_i8)];
_7 = _5;
place!(Field::<i64>(Variant(_11, 3), 3)) = 7892372694462569987_i64;
_17 = [_16,_16,_16,_16,_16];
place!(Field::<[i8; 4]>(Variant(_11, 3), 4)) = Field::<([i8; 4],)>(Variant(_11, 3), 1).0;
_3 = (-29282_i16) as isize;
_15.fld3 = Adt42::Variant2 { fld0: Field::<[usize; 8]>(Variant(_11, 3), 0),fld1: Field::<i64>(Variant(_11, 3), 3) };
place!(Field::<i64>(Variant(_11, 3), 3)) = Field::<i64>(Variant(_15.fld3, 2), 1) >> _4;
_22 = _12;
_5 = [(-60_i8),94_i8];
Call(place!(Field::<([i8; 4],)>(Variant(_11, 3), 1)).0 = core::intrinsics::transmute(_13), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_5 = _15.fld0;
(*RET) = 162_u8;
_28 = _25;
_15.fld4 = Field::<([i8; 2], *mut u8)>(Variant(_15.fld3, 0), 1).1;
_32 = Field::<f64>(Variant(_15.fld3, 0), 2);
place!(Field::<i64>(Variant(_11, 1), 0)) = !2152829674328112838_i64;
_8 = _15.fld2 + _9;
_15.fld2 = !_2;
place!(Field::<(usize, i32)>(Variant(_15.fld3, 0), 5)).0 = _26;
place!(Field::<([i8; 2], *mut u8)>(Variant(_15.fld3, 0), 1)).1 = core::ptr::addr_of_mut!(_27);
_27 = 245_u8;
_6 = _9 & _2;
_32 = _26 as f64;
match _27 {
245 => bb10,
_ => bb1
}
}
bb10 = {
_7 = [97_i8,(-90_i8)];
place!(Field::<[i32; 6]>(Variant(_15.fld3, 0), 6)) = [_13,_13,Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).1,Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).1,_13,_13];
_19 = 61149_u16 as u32;
_19 = _14 * _14;
_1 = -_2;
_37 = _4 - _6;
_7 = [(-79_i8),78_i8];
_4 = _1 + _6;
_4 = _8;
_43.fld4.fld6.fld4 = Field::<(usize, i32)>(Variant(_15.fld3, 0), 5);
_1 = _37;
Call(_23 = core::intrinsics::bswap(_8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_34 = !_8;
_16 = _6 <= _4;
_43.fld1.0 = core::ptr::addr_of_mut!(_14);
_35 = 26095_i16 as f32;
_43.fld4.fld5 = [40720_u16,63241_u16,20929_u16];
Goto(bb12)
}
bb12 = {
_13 = Field::<f64>(Variant(_15.fld3, 0), 2) as i32;
_23 = 307197993521499798843735688115790145137_u128 as isize;
_32 = Field::<f64>(Variant(_15.fld3, 0), 2);
SetDiscriminant(_11, 1);
_43.fld4.fld6.fld0 = Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).1;
_1 = _34 | _34;
_43.fld1.0 = core::ptr::addr_of_mut!(_19);
_21 = _26 * _43.fld4.fld6.fld4.0;
place!(Field::<i64>(Variant(_11, 1), 0)) = (-2396103816935961235_i64);
_15.fld4 = Field::<([i8; 2], *mut u8)>(Variant(_15.fld3, 0), 1).1;
_39 = [_24,_20,_24,_25];
_43.fld4.fld1 = 9241274302866916842_u64;
(*RET) = 222_u8;
_15.fld0 = [14_i8,(-118_i8)];
_43.fld4.fld3.0 = core::ptr::addr_of_mut!(_19);
place!(Field::<([i8; 2], *mut u8)>(Variant(_15.fld3, 0), 1)) = (_5, _15.fld4);
RET = _15.fld4;
_9 = _15.fld2 ^ _34;
Call((*RET) = core::intrinsics::transmute(_22), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<[i8; 4]>(Variant(_15.fld3, 0), 4)) = [(-68_i8),85_i8,(-29_i8),(-96_i8)];
_43.fld4.fld4.fld2 = !_37;
_33 = _25;
_15.fld0 = _7;
_14 = _19 - _19;
_43.fld4.fld5 = [45086_u16,19780_u16,20797_u16];
Call((*RET) = core::intrinsics::transmute(_22), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_43.fld4.fld6.fld4.1 = _13 ^ Field::<(usize, i32)>(Variant(_15.fld3, 0), 5).1;
_44.2 = [78664696525907854978054991649514371023_u128,157251681700229433992386703103651628213_u128,202251342213851779018138847527547998996_u128,53556573308877739954586334597420957897_u128,9934873027724073171110527672931032335_u128,198568649637307863850957826294769690075_u128];
_43.fld4.fld2 = Adt51::Variant0 { fld0: _39,fld1: (-28058_i16),fld2: Move(_11),fld3: _14 };
_43.fld0 = [Field::<i64>(Variant(Field::<Adt48>(Variant(_43.fld4.fld2, 0), 2), 1), 0),Field::<i64>(Variant(Field::<Adt48>(Variant(_43.fld4.fld2, 0), 2), 1), 0),Field::<i64>(Variant(Field::<Adt48>(Variant(_43.fld4.fld2, 0), 2), 1), 0),Field::<i64>(Variant(Field::<Adt48>(Variant(_43.fld4.fld2, 0), 2), 1), 0),Field::<i64>(Variant(Field::<Adt48>(Variant(_43.fld4.fld2, 0), 2), 1), 0)];
_47 = -_43.fld4.fld6.fld0;
place!(Field::<*mut i16>(Variant(_15.fld3, 0), 0)) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_43.fld4.fld2, 0), 1)));
_11 = Adt48::Variant1 { fld0: Field::<i64>(Variant(Field::<Adt48>(Variant(_43.fld4.fld2, 0), 2), 1), 0) };
_26 = _21;
_22 = _16;
_43.fld4.fld7 = _32 * _32;
_38 = [_24,_28,_33,_28];
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(16_usize, 26_usize, Move(_26), 19_usize, Move(_19), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(16_usize, 2_usize, Move(_2), 16_usize, Move(_16), 3_usize, Move(_3), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(16_usize, 27_usize, Move(_27), 10_usize, Move(_10), 5_usize, Move(_5), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(16_usize, 37_usize, Move(_37), 20_usize, Move(_20), 9_usize, Move(_9), 39_usize, Move(_39)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [bool; 5],mut _6: isize,mut _7: isize,mut _8: [bool; 5],mut _9: isize,mut _10: i64,mut _11: isize,mut _12: isize) -> *mut u8 {
mir! {
type RET = *mut u8;
let _13: usize;
let _14: [bool; 5];
let _15: &'static i64;
let _16: i32;
let _17: f64;
let _18: Adt56;
let _19: [bool; 5];
let _20: *const usize;
let _21: bool;
let _22: (([i8; 4],), i128, [u128; 6]);
let _23: f64;
let _24: bool;
let _25: [char; 4];
let _26: *mut u128;
let _27: Adt41;
let _28: [i64; 5];
let _29: isize;
let _30: f32;
let _31: bool;
let _32: u64;
let _33: usize;
let _34: &'static i64;
let _35: [i8; 2];
let _36: bool;
let _37: f64;
let _38: isize;
let _39: Adt45;
let _40: (usize, i32);
let _41: f32;
let _42: f32;
let _43: ([i8; 4],);
let _44: *const usize;
let _45: char;
let _46: char;
let _47: Adt47;
let _48: Adt47;
let _49: f32;
let _50: f32;
let _51: [i64; 5];
let _52: char;
let _53: char;
let _54: *mut u128;
let _55: u128;
let _56: bool;
let _57: (i32,);
let _58: f64;
let _59: f64;
let _60: Adt56;
let _61: Adt42;
let _62: Adt46;
let _63: u64;
let _64: [i64; 5];
let _65: Adt46;
let _66: i16;
let _67: [bool; 5];
let _68: usize;
let _69: [i8; 2];
let _70: i64;
let _71: Adt46;
let _72: char;
let _73: f32;
let _74: f64;
let _75: u128;
let _76: ();
let _77: ();
{
_2 = _9;
_5 = _8;
Goto(bb1)
}
bb1 = {
_4 = !_11;
_4 = _3;
_12 = _1;
_5 = [false,false,false,true,false];
_7 = !_6;
_2 = _3;
_7 = !_11;
_14 = _8;
_14 = _8;
_16 = 2716493071248990539_u64 as i32;
_6 = -_11;
_4 = _10 as isize;
_10 = !8371391223948245924_i64;
_5 = _14;
_8 = _5;
Call(_10 = core::intrinsics::bswap(6245751734780781925_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _14;
_7 = 15331915727006010730_u64 as isize;
_7 = _3;
_16 = !(-1420020010_i32);
_7 = -_12;
_11 = _1;
_14 = [false,false,false,false,true];
_3 = _4;
_3 = -_4;
_7 = _6 >> _11;
_4 = _11;
_9 = 85963561501211537452126292600131199616_i128 as isize;
_13 = 1_usize;
_8[_13] = _5[_13];
_19 = _8;
_11 = 54165063633641360351806193664570723113_i128 as isize;
_2 = _3 >> _12;
_7 = _4;
_14 = [_19[_13],_8[_13],_19[_13],_5[_13],_19[_13]];
_2 = !_6;
_9 = _3;
_15 = &_10;
Goto(bb3)
}
bb3 = {
_21 = !_14[_13];
_8[_13] = !_14[_13];
_5[_13] = !_14[_13];
_22.2 = [48176598896733130681442167199031354408_u128,248958324039379741044957072993390763210_u128,286410164023517807246163702139511585899_u128,150363909452492211862651440628886159666_u128,67309448429875351693421445577706647403_u128,163344937894261234642780394398686703324_u128];
_15 = &_10;
_16 = 270980685_i32 & (-956862592_i32);
_9 = _12;
_15 = &_10;
_22.0.0[_13] = -(-92_i8);
_17 = _22.0.0[_13] as f64;
Goto(bb4)
}
bb4 = {
_20 = core::ptr::addr_of!(_13);
_14[_13] = !_5[_13];
_24 = _8[_13] ^ _14[_13];
_15 = &(*_15);
_22.1 = (-132865886229358129860759727137447419428_i128) - 50150012052752962166739485235795322184_i128;
_3 = _2;
_11 = -_9;
_5 = [_19[_13],_24,_14[_13],_8[_13],_8[_13]];
_17 = 266469363_u32 as f64;
_8 = _5;
_13 = 13536909499657338260_u64 as usize;
_5 = [_24,_24,_21,_21,_24];
_5 = [_21,_24,_21,_24,_21];
_1 = _4 - _6;
_5 = [_24,_24,_24,_24,_24];
_19 = [_24,_21,_24,_24,_21];
_5 = [_24,_24,_21,_24,_21];
_12 = _1 & _6;
_6 = _4;
_20 = core::ptr::addr_of!((*_20));
_2 = 13509473941805993564_u64 as isize;
_7 = _4 >> _12;
Call(_10 = fn18(_7, _1, _1, _1, _3, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = _16 as isize;
_15 = &_10;
_28 = [(*_15),(*_15),(*_15),_10,_10];
_22.2 = [165619002635740002569808512410740419845_u128,246831487696331423947701566302444572961_u128,179629521098248234264406191015512919864_u128,241550382440930990597905504641075593218_u128,22113487465576266042310280417590747535_u128,237596428565437655528764720007034526250_u128];
_22.2 = [46261222627475358776286444238546155778_u128,25170015589746120126999654026666792774_u128,67677261144632191786950508559120454187_u128,220213640421816166285755495918634376927_u128,123011569032734689961303128284442291173_u128,185159559349428367282991281649322813619_u128];
(*_20) = !7_usize;
_23 = _17 * _17;
_28 = [(*_15),(*_15),(*_15),_10,(*_15)];
_12 = _3 & _9;
_21 = _24;
_8 = _19;
Goto(bb6)
}
bb6 = {
_31 = _12 >= _7;
_12 = _3;
_24 = !_31;
_30 = (*_15) as f32;
_15 = &_10;
_22.0.0 = [(-83_i8),47_i8,(-104_i8),(-38_i8)];
_5 = [_31,_24,_24,_31,_31];
_33 = !(*_20);
_22.0.0 = [(-54_i8),(-8_i8),4_i8,(-84_i8)];
_2 = !_12;
_2 = _12;
_22.1 = (-12184278655860464907883488136033354827_i128);
_31 = _21;
_3 = _11;
_1 = !_7;
_22.1 = 62979677055599840472399255420761322411_i128;
_1 = _16 as isize;
_30 = 326505416606420210093652461370643335550_u128 as f32;
_22.2 = [226987280585390280764485421313461347202_u128,194132003938131589766893962207987318897_u128,292254458484855007007662037653216463990_u128,20376890771976181752120376710275844381_u128,297192436191151271683477864668237585771_u128,152496223452590296135359980796593645090_u128];
_20 = core::ptr::addr_of!(_33);
_23 = 14344_i16 as f64;
_11 = _4;
_2 = 1_u8 as isize;
(*_20) = !_13;
_8 = _14;
_19 = [_21,_31,_24,_24,_24];
Call(_33 = core::intrinsics::bswap(_13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _6;
_10 = (-933043594949688660_i64) | (-3552815452188779334_i64);
_36 = !_21;
(*_20) = _13 << _3;
_32 = !5873616673613639837_u64;
_30 = _32 as f32;
_12 = _11;
_33 = _13;
_29 = _11 ^ _3;
_10 = (-3473394985919304691_i64) * 8589145925863879797_i64;
match _22.1 {
0 => bb8,
1 => bb9,
62979677055599840472399255420761322411 => bb11,
_ => bb10
}
}
bb8 = {
_8 = _14;
_7 = 15331915727006010730_u64 as isize;
_7 = _3;
_16 = !(-1420020010_i32);
_7 = -_12;
_11 = _1;
_14 = [false,false,false,false,true];
_3 = _4;
_3 = -_4;
_7 = _6 >> _11;
_4 = _11;
_9 = 85963561501211537452126292600131199616_i128 as isize;
_13 = 1_usize;
_8[_13] = _5[_13];
_19 = _8;
_11 = 54165063633641360351806193664570723113_i128 as isize;
_2 = _3 >> _12;
_7 = _4;
_14 = [_19[_13],_8[_13],_19[_13],_5[_13],_19[_13]];
_2 = !_6;
_9 = _3;
_15 = &_10;
Goto(bb3)
}
bb9 = {
_1 = _16 as isize;
_15 = &_10;
_28 = [(*_15),(*_15),(*_15),_10,_10];
_22.2 = [165619002635740002569808512410740419845_u128,246831487696331423947701566302444572961_u128,179629521098248234264406191015512919864_u128,241550382440930990597905504641075593218_u128,22113487465576266042310280417590747535_u128,237596428565437655528764720007034526250_u128];
_22.2 = [46261222627475358776286444238546155778_u128,25170015589746120126999654026666792774_u128,67677261144632191786950508559120454187_u128,220213640421816166285755495918634376927_u128,123011569032734689961303128284442291173_u128,185159559349428367282991281649322813619_u128];
(*_20) = !7_usize;
_23 = _17 * _17;
_28 = [(*_15),(*_15),(*_15),_10,(*_15)];
_12 = _3 & _9;
_21 = _24;
_8 = _19;
Goto(bb6)
}
bb10 = {
_21 = !_14[_13];
_8[_13] = !_14[_13];
_5[_13] = !_14[_13];
_22.2 = [48176598896733130681442167199031354408_u128,248958324039379741044957072993390763210_u128,286410164023517807246163702139511585899_u128,150363909452492211862651440628886159666_u128,67309448429875351693421445577706647403_u128,163344937894261234642780394398686703324_u128];
_15 = &_10;
_16 = 270980685_i32 & (-956862592_i32);
_9 = _12;
_15 = &_10;
_22.0.0[_13] = -(-92_i8);
_17 = _22.0.0[_13] as f64;
Goto(bb4)
}
bb11 = {
_1 = _30 as isize;
_7 = !_9;
_32 = 378248297249092287_u64;
(*_20) = _13;
_36 = _31;
Goto(bb12)
}
bb12 = {
_35 = [124_i8,7_i8];
_12 = _4 & _3;
_4 = (*_20) as isize;
_24 = !_36;
_8 = [_24,_31,_36,_21,_36];
_11 = !_7;
_31 = _21;
_4 = -_11;
_31 = _21;
match _22.1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
62979677055599840472399255420761322411 => bb18,
_ => bb17
}
}
bb13 = {
_8 = _14;
_7 = 15331915727006010730_u64 as isize;
_7 = _3;
_16 = !(-1420020010_i32);
_7 = -_12;
_11 = _1;
_14 = [false,false,false,false,true];
_3 = _4;
_3 = -_4;
_7 = _6 >> _11;
_4 = _11;
_9 = 85963561501211537452126292600131199616_i128 as isize;
_13 = 1_usize;
_8[_13] = _5[_13];
_19 = _8;
_11 = 54165063633641360351806193664570723113_i128 as isize;
_2 = _3 >> _12;
_7 = _4;
_14 = [_19[_13],_8[_13],_19[_13],_5[_13],_19[_13]];
_2 = !_6;
_9 = _3;
_15 = &_10;
Goto(bb3)
}
bb14 = {
_4 = !_11;
_4 = _3;
_12 = _1;
_5 = [false,false,false,true,false];
_7 = !_6;
_2 = _3;
_7 = !_11;
_14 = _8;
_14 = _8;
_16 = 2716493071248990539_u64 as i32;
_6 = -_11;
_4 = _10 as isize;
_10 = !8371391223948245924_i64;
_5 = _14;
_8 = _5;
Call(_10 = core::intrinsics::bswap(6245751734780781925_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_1 = _16 as isize;
_15 = &_10;
_28 = [(*_15),(*_15),(*_15),_10,_10];
_22.2 = [165619002635740002569808512410740419845_u128,246831487696331423947701566302444572961_u128,179629521098248234264406191015512919864_u128,241550382440930990597905504641075593218_u128,22113487465576266042310280417590747535_u128,237596428565437655528764720007034526250_u128];
_22.2 = [46261222627475358776286444238546155778_u128,25170015589746120126999654026666792774_u128,67677261144632191786950508559120454187_u128,220213640421816166285755495918634376927_u128,123011569032734689961303128284442291173_u128,185159559349428367282991281649322813619_u128];
(*_20) = !7_usize;
_23 = _17 * _17;
_28 = [(*_15),(*_15),(*_15),_10,(*_15)];
_12 = _3 & _9;
_21 = _24;
_8 = _19;
Goto(bb6)
}
bb16 = {
_8 = _14;
_7 = 15331915727006010730_u64 as isize;
_7 = _3;
_16 = !(-1420020010_i32);
_7 = -_12;
_11 = _1;
_14 = [false,false,false,false,true];
_3 = _4;
_3 = -_4;
_7 = _6 >> _11;
_4 = _11;
_9 = 85963561501211537452126292600131199616_i128 as isize;
_13 = 1_usize;
_8[_13] = _5[_13];
_19 = _8;
_11 = 54165063633641360351806193664570723113_i128 as isize;
_2 = _3 >> _12;
_7 = _4;
_14 = [_19[_13],_8[_13],_19[_13],_5[_13],_19[_13]];
_2 = !_6;
_9 = _3;
_15 = &_10;
Goto(bb3)
}
bb17 = {
_31 = _12 >= _7;
_12 = _3;
_24 = !_31;
_30 = (*_15) as f32;
_15 = &_10;
_22.0.0 = [(-83_i8),47_i8,(-104_i8),(-38_i8)];
_5 = [_31,_24,_24,_31,_31];
_33 = !(*_20);
_22.0.0 = [(-54_i8),(-8_i8),4_i8,(-84_i8)];
_2 = !_12;
_2 = _12;
_22.1 = (-12184278655860464907883488136033354827_i128);
_31 = _21;
_3 = _11;
_1 = !_7;
_22.1 = 62979677055599840472399255420761322411_i128;
_1 = _16 as isize;
_30 = 326505416606420210093652461370643335550_u128 as f32;
_22.2 = [226987280585390280764485421313461347202_u128,194132003938131589766893962207987318897_u128,292254458484855007007662037653216463990_u128,20376890771976181752120376710275844381_u128,297192436191151271683477864668237585771_u128,152496223452590296135359980796593645090_u128];
_20 = core::ptr::addr_of!(_33);
_23 = 14344_i16 as f64;
_11 = _4;
_2 = 1_u8 as isize;
(*_20) = !_13;
_8 = _14;
_19 = [_21,_31,_24,_24,_24];
Call(_33 = core::intrinsics::bswap(_13), ReturnTo(bb7), UnwindUnreachable())
}
bb18 = {
_31 = _36 & _21;
_43.0 = [31_i8,(-25_i8),20_i8,76_i8];
_22.0 = _43;
_29 = _33 as isize;
_44 = core::ptr::addr_of!((*_20));
_8 = [_36,_24,_24,_36,_36];
_40.1 = !_16;
_15 = &_10;
_22.0.0 = [71_i8,(-3_i8),(-8_i8),(-32_i8)];
_23 = _17;
_40 = (_33, _16);
(*_44) = !_13;
_15 = &_10;
_1 = _6 | _7;
_37 = -_23;
match _32 {
0 => bb1,
1 => bb13,
2 => bb3,
3 => bb16,
4 => bb12,
5 => bb9,
6 => bb10,
378248297249092287 => bb20,
_ => bb19
}
}
bb19 = {
_4 = !_11;
_4 = _3;
_12 = _1;
_5 = [false,false,false,true,false];
_7 = !_6;
_2 = _3;
_7 = !_11;
_14 = _8;
_14 = _8;
_16 = 2716493071248990539_u64 as i32;
_6 = -_11;
_4 = _10 as isize;
_10 = !8371391223948245924_i64;
_5 = _14;
_8 = _5;
Call(_10 = core::intrinsics::bswap(6245751734780781925_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb20 = {
_10 = (-7940323471475466107_i64) ^ 5779199041323252408_i64;
(*_20) = !_40.0;
_14 = [_36,_36,_36,_31,_31];
_7 = _11;
Goto(bb21)
}
bb21 = {
_34 = &_10;
(*_20) = _40.0 - _40.0;
_40 = ((*_44), _16);
_2 = !_3;
_12 = -_1;
_41 = _30 + _30;
_1 = _9 & _4;
Goto(bb22)
}
bb22 = {
_9 = _11 * _6;
_23 = _37;
_43 = _22.0;
_9 = _11 >> _7;
_45 = '\u{4c039}';
_24 = _36;
(*_20) = _40.0 ^ _40.0;
_21 = _11 == _9;
_31 = !_24;
_4 = !_29;
_45 = '\u{68ba6}';
match _32 {
0 => bb7,
378248297249092287 => bb23,
_ => bb16
}
}
bb23 = {
_17 = _32 as f64;
_1 = _37 as isize;
_34 = &(*_34);
_49 = _30 * _30;
_22.1 = (-23435665936278222043872742919432509014_i128);
_34 = &(*_34);
_20 = _44;
Goto(bb24)
}
bb24 = {
_24 = _31;
_41 = _49;
_49 = _30 - _41;
_46 = _45;
_15 = &(*_34);
_50 = 202431413774298419250159505290005117365_u128 as f32;
_9 = (*_20) as isize;
_25 = [_45,_45,_45,_46];
_11 = _3 | _2;
_10 = 8083293140976445000_i64;
match _32 {
0 => bb15,
1 => bb7,
2 => bb20,
3 => bb4,
4 => bb5,
5 => bb25,
378248297249092287 => bb27,
_ => bb26
}
}
bb25 = {
_21 = !_14[_13];
_8[_13] = !_14[_13];
_5[_13] = !_14[_13];
_22.2 = [48176598896733130681442167199031354408_u128,248958324039379741044957072993390763210_u128,286410164023517807246163702139511585899_u128,150363909452492211862651440628886159666_u128,67309448429875351693421445577706647403_u128,163344937894261234642780394398686703324_u128];
_15 = &_10;
_16 = 270980685_i32 & (-956862592_i32);
_9 = _12;
_15 = &_10;
_22.0.0[_13] = -(-92_i8);
_17 = _22.0.0[_13] as f64;
Goto(bb4)
}
bb26 = {
_20 = core::ptr::addr_of!(_13);
_14[_13] = !_5[_13];
_24 = _8[_13] ^ _14[_13];
_15 = &(*_15);
_22.1 = (-132865886229358129860759727137447419428_i128) - 50150012052752962166739485235795322184_i128;
_3 = _2;
_11 = -_9;
_5 = [_19[_13],_24,_14[_13],_8[_13],_8[_13]];
_17 = 266469363_u32 as f64;
_8 = _5;
_13 = 13536909499657338260_u64 as usize;
_5 = [_24,_24,_21,_21,_24];
_5 = [_21,_24,_21,_24,_21];
_1 = _4 - _6;
_5 = [_24,_24,_24,_24,_24];
_19 = [_24,_21,_24,_24,_21];
_5 = [_24,_24,_21,_24,_21];
_12 = _1 & _6;
_6 = _4;
_20 = core::ptr::addr_of!((*_20));
_2 = 13509473941805993564_u64 as isize;
_7 = _4 >> _12;
Call(_10 = fn18(_7, _1, _1, _1, _3, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb27 = {
_17 = _32 as f64;
_22.0 = (_43.0,);
_29 = _11 + _2;
_4 = (-28190_i16) as isize;
_28 = [_10,_10,_10,_10,_10];
_10 = (-3680986223351174185_i64);
_23 = -_17;
_53 = _45;
_32 = !7449149091828887155_u64;
_41 = _49;
_52 = _46;
_44 = core::ptr::addr_of!((*_44));
_40 = ((*_20), _16);
_26 = core::ptr::addr_of_mut!(_55);
_31 = !_21;
_38 = _40.1 as isize;
_35 = [(-90_i8),63_i8];
_40.0 = (*_20) - (*_20);
Goto(bb28)
}
bb28 = {
_21 = _36 > _36;
_21 = _40.0 != _40.0;
_40.0 = (-81_i8) as usize;
_22.2 = [78538144656523122233163055136609452000_u128,140775413628736299250447052527844833032_u128,54620832786294475543550770405488400320_u128,327416028909484037841897356716981420000_u128,313147695618464706014476804081158662636_u128,13858983551660212920860972289030642539_u128];
_6 = -_29;
_6 = _29 | _11;
_32 = 11602380620355075921_u64 * 8808366873142848557_u64;
_40 = ((*_44), _16);
_57 = (_16,);
(*_26) = 170977122824532801576673283800875045846_u128;
_53 = _52;
_22.0 = _43;
(*_26) = 29425529342648300845083275581294193399_u128 - 278263793100411781508786882768708953181_u128;
_31 = _21;
_53 = _52;
_7 = 1710565646_u32 as isize;
_22.2 = [(*_26),(*_26),(*_26),_55,_55,(*_26)];
_31 = _21 | _24;
_4 = 17502_u16 as isize;
_40 = ((*_44), _16);
_42 = (*_26) as f32;
(*_26) = 284391853735046796094772780256436971661_u128;
_37 = -_17;
match _22.1 {
316846700984660241419501864512335702442 => bb30,
_ => bb29
}
}
bb29 = {
_20 = core::ptr::addr_of!(_13);
_14[_13] = !_5[_13];
_24 = _8[_13] ^ _14[_13];
_15 = &(*_15);
_22.1 = (-132865886229358129860759727137447419428_i128) - 50150012052752962166739485235795322184_i128;
_3 = _2;
_11 = -_9;
_5 = [_19[_13],_24,_14[_13],_8[_13],_8[_13]];
_17 = 266469363_u32 as f64;
_8 = _5;
_13 = 13536909499657338260_u64 as usize;
_5 = [_24,_24,_21,_21,_24];
_5 = [_21,_24,_21,_24,_21];
_1 = _4 - _6;
_5 = [_24,_24,_24,_24,_24];
_19 = [_24,_21,_24,_24,_21];
_5 = [_24,_24,_21,_24,_21];
_12 = _1 & _6;
_6 = _4;
_20 = core::ptr::addr_of!((*_20));
_2 = 13509473941805993564_u64 as isize;
_7 = _4 >> _12;
Call(_10 = fn18(_7, _1, _1, _1, _3, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb30 = {
_57.0 = _16 | _40.1;
_30 = -_42;
_8 = [_31,_24,_24,_24,_31];
_49 = -_30;
_16 = _17 as i32;
_38 = _41 as isize;
_1 = _41 as isize;
_22.2 = [(*_26),(*_26),(*_26),(*_26),(*_26),_55];
_40.1 = -_57.0;
_9 = !_2;
_45 = _46;
_45 = _46;
_2 = _12 * _6;
(*_44) = (-8650_i16) as usize;
_56 = !_24;
_65.fld4 = (_33, _16);
_9 = _38 - _2;
_23 = 14365_i16 as f64;
_50 = 31_u8 as f32;
_29 = _12 | _2;
_51 = [_10,_10,_10,_10,_10];
_59 = _41 as f64;
_15 = &_10;
Goto(bb31)
}
bb31 = {
_22.2 = [(*_26),(*_26),_55,_55,(*_26),(*_26)];
_12 = _36 as isize;
_62.fld4.0 = _56 as usize;
_4 = _29;
_44 = core::ptr::addr_of!(_13);
_22.2 = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_33 = _62.fld4.0 >> _12;
_49 = _41 - _30;
_62.fld4.1 = _57.0;
_14 = [_56,_56,_24,_31,_24];
_22.0 = (_43.0,);
_62.fld0 = _62.fld4.1;
(*_26) = 224773775980526544415172365958720086151_u128;
_63 = _10 as u64;
_65.fld1 = [_62.fld4.1,_40.1,_62.fld0,_57.0,_57.0,_40.1];
_19 = [_21,_31,_31,_24,_24];
match _22.1 {
0 => bb12,
1 => bb9,
2 => bb32,
3 => bb33,
4 => bb34,
5 => bb35,
316846700984660241419501864512335702442 => bb37,
_ => bb36
}
}
bb32 = {
_4 = !_11;
_4 = _3;
_12 = _1;
_5 = [false,false,false,true,false];
_7 = !_6;
_2 = _3;
_7 = !_11;
_14 = _8;
_14 = _8;
_16 = 2716493071248990539_u64 as i32;
_6 = -_11;
_4 = _10 as isize;
_10 = !8371391223948245924_i64;
_5 = _14;
_8 = _5;
Call(_10 = core::intrinsics::bswap(6245751734780781925_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb33 = {
_1 = _16 as isize;
_15 = &_10;
_28 = [(*_15),(*_15),(*_15),_10,_10];
_22.2 = [165619002635740002569808512410740419845_u128,246831487696331423947701566302444572961_u128,179629521098248234264406191015512919864_u128,241550382440930990597905504641075593218_u128,22113487465576266042310280417590747535_u128,237596428565437655528764720007034526250_u128];
_22.2 = [46261222627475358776286444238546155778_u128,25170015589746120126999654026666792774_u128,67677261144632191786950508559120454187_u128,220213640421816166285755495918634376927_u128,123011569032734689961303128284442291173_u128,185159559349428367282991281649322813619_u128];
(*_20) = !7_usize;
_23 = _17 * _17;
_28 = [(*_15),(*_15),(*_15),_10,(*_15)];
_12 = _3 & _9;
_21 = _24;
_8 = _19;
Goto(bb6)
}
bb34 = {
_3 = _6;
_10 = (-933043594949688660_i64) | (-3552815452188779334_i64);
_36 = !_21;
(*_20) = _13 << _3;
_32 = !5873616673613639837_u64;
_30 = _32 as f32;
_12 = _11;
_33 = _13;
_29 = _11 ^ _3;
_10 = (-3473394985919304691_i64) * 8589145925863879797_i64;
match _22.1 {
0 => bb8,
1 => bb9,
62979677055599840472399255420761322411 => bb11,
_ => bb10
}
}
bb35 = {
_34 = &_10;
(*_20) = _40.0 - _40.0;
_40 = ((*_44), _16);
_2 = !_3;
_12 = -_1;
_41 = _30 + _30;
_1 = _9 & _4;
Goto(bb22)
}
bb36 = {
_21 = !_14[_13];
_8[_13] = !_14[_13];
_5[_13] = !_14[_13];
_22.2 = [48176598896733130681442167199031354408_u128,248958324039379741044957072993390763210_u128,286410164023517807246163702139511585899_u128,150363909452492211862651440628886159666_u128,67309448429875351693421445577706647403_u128,163344937894261234642780394398686703324_u128];
_15 = &_10;
_16 = 270980685_i32 & (-956862592_i32);
_9 = _12;
_15 = &_10;
_22.0.0[_13] = -(-92_i8);
_17 = _22.0.0[_13] as f64;
Goto(bb4)
}
bb37 = {
_41 = 30357_u16 as f32;
(*_44) = _62.fld4.0;
_21 = _36;
_55 = _63 as u128;
_47 = Adt47::Variant0 { fld0: _22.2,fld1: 127_u8,fld2: _11,fld3: _32 };
_65.fld2 = _26;
_24 = _31 != _36;
_36 = _12 >= _3;
_24 = !_21;
_51 = _28;
_58 = _59 - _17;
_62.fld4.1 = _22.1 as i32;
_50 = 3323399999_u32 as f32;
_31 = !_56;
_32 = Field::<u64>(Variant(_47, 0), 3);
_22.0 = (_43.0,);
_65.fld0 = _57.0;
_28 = [_10,(*_15),_10,(*_15),_10];
_71.fld1 = [_40.1,_57.0,_65.fld0,_65.fld4.1,_65.fld0,_57.0];
(*_26) = 15549658702493482775077759416337202353_u128;
_43.0 = [(-111_i8),(-107_i8),21_i8,(-61_i8)];
_71.fld1 = _65.fld1;
_8 = _14;
_67 = [_36,_36,_36,_21,_36];
Goto(bb38)
}
bb38 = {
_62.fld0 = 66_u8 as i32;
_62.fld4.0 = !_13;
RET = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_47, 0), 1)));
_62.fld4.1 = _65.fld0;
_72 = _46;
_35 = [(-99_i8),(-101_i8)];
_22 = (_43, 142048248569036492852169431996194823045_i128, Field::<[u128; 6]>(Variant(_47, 0), 0));
(*RET) = 127_u8 - 199_u8;
_71.fld4.0 = _33;
_62.fld1 = _71.fld1;
Goto(bb39)
}
bb39 = {
Call(_76 = dump_var(17_usize, 1_usize, Move(_1), 40_usize, Move(_40), 56_usize, Move(_56), 12_usize, Move(_12)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_76 = dump_var(17_usize, 55_usize, Move(_55), 72_usize, Move(_72), 52_usize, Move(_52), 6_usize, Move(_6)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_76 = dump_var(17_usize, 31_usize, Move(_31), 3_usize, Move(_3), 11_usize, Move(_11), 33_usize, Move(_33)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_76 = dump_var(17_usize, 14_usize, Move(_14), 25_usize, Move(_25), 51_usize, Move(_51), 38_usize, Move(_38)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_76 = dump_var(17_usize, 29_usize, Move(_29), 8_usize, Move(_8), 9_usize, Move(_9), 46_usize, Move(_46)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> i64 {
mir! {
type RET = i64;
let _7: [u16; 3];
let _8: *mut u32;
let _9: Adt41;
let _10: char;
let _11: f64;
let _12: f32;
let _13: [i64; 8];
let _14: [u128; 6];
let _15: ();
let _16: ();
{
RET = 1288937034335328161_i64 + (-481027993444529766_i64);
_3 = _6;
_3 = _6;
_1 = _3;
RET = 130764424614446719_i64;
_5 = 199_u8 as isize;
RET = (-5499129872748319725_i64);
_3 = !_2;
_1 = -_3;
RET = _4 as i64;
_7 = [30439_u16,58014_u16,10462_u16];
_5 = _6 + _1;
_7 = [4526_u16,51314_u16,56173_u16];
_3 = (-25360_i16) as isize;
_6 = !_5;
_1 = _5;
RET = (-5781545200908682543_i64);
RET = (-3599314653851051017_i64);
_6 = _5 + _4;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463459775292777917160439 => bb5,
_ => bb4
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
_1 = 3472888708_u32 as isize;
_2 = _5 | _4;
_4 = _5 | _6;
_4 = _2;
_2 = !_4;
_2 = false as isize;
_2 = 2793216292_u32 as isize;
_10 = '\u{bad7}';
_4 = _6;
RET = 81575747737893069869615591126525607426_i128 as i64;
RET = (-1271517825621159019_i64);
_10 = '\u{420fb}';
_4 = _5;
RET = 4126225213164055694_i64 - 7280982616393542152_i64;
_4 = _5 ^ _5;
RET = -8745820680335460394_i64;
RET = true as i64;
_10 = '\u{4d1b7}';
_1 = _4;
_4 = _1 >> _5;
Goto(bb6)
}
bb6 = {
_7 = [11201_u16,51702_u16,35137_u16];
_5 = !_4;
_5 = _1 << _6;
RET = (-8250393208602814702_i64) << _6;
_3 = -_1;
_10 = '\u{dfe0f}';
_6 = _3 - _5;
_7 = [34390_u16,38379_u16,18699_u16];
RET = 8966634761225273232_i64 << _4;
_3 = _5;
_10 = '\u{f2e9e}';
_1 = _4 | _3;
_3 = -_4;
_3 = 54074_u16 as isize;
_4 = -_1;
RET = 41037743146734734626140255612953533123_u128 as i64;
_7 = [23998_u16,47111_u16,26049_u16];
RET = !(-5765658808273252639_i64);
RET = -1870309221829046316_i64;
_5 = _1 * _4;
_1 = _5 & _4;
Goto(bb7)
}
bb7 = {
_3 = !_5;
_11 = RET as f64;
RET = _10 as i64;
_4 = 91924275540174654317404400181333238677_i128 as isize;
_4 = -_3;
_11 = 4_usize as f64;
_3 = _5;
_12 = (-126914083621746952582859129532591643385_i128) as f32;
_12 = RET as f32;
_7 = [61823_u16,55445_u16,24312_u16];
_12 = (-9503647_i32) as f32;
RET = 4282090633499575680_i64;
_13 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = [41495_u16,9776_u16,54495_u16];
RET = (-7600376787986401809_i64) << _4;
_13 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = -_6;
_14 = [238416670935263910300012949298281579304_u128,334197348316052419212259606709253832506_u128,234041276492800806221524949899428955371_u128,16068620710699417431364963599169465081_u128,89198844323072380929796087680237397185_u128,4103216475762079961684480974865577575_u128];
_14 = [118485372129392988534942756217578352228_u128,193016376098422677401681820351505256462_u128,130271793433003806088295594054035521598_u128,205765742953729527007382231836259889768_u128,154896721290895068232021919399020073073_u128,187325234361697536349344084161037670536_u128];
_7 = [41443_u16,48629_u16,6933_u16];
Goto(bb8)
}
bb8 = {
Call(_15 = dump_var(18_usize, 10_usize, Move(_10), 14_usize, Move(_14), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_15 = dump_var(18_usize, 3_usize, Move(_3), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{e55bd}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-84_i8)), std::hint::black_box((-25592_i16)), std::hint::black_box((-1060868620_i32)), std::hint::black_box(9066921900888115224_i64), std::hint::black_box((-61006596456842381265487820089168138353_i128)), std::hint::black_box(7712937090077778222_usize), std::hint::black_box(184_u8), std::hint::black_box(51094_u16), std::hint::black_box(1591326160_u32), std::hint::black_box(16810899155656736594_u64), std::hint::black_box(58570266472073305919659574646370771363_u128));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: u128,
fld1: usize,
fld2: f64,
fld3: u32,

},
Variant1{
fld0: [u128; 6],

},
Variant2{
fld0: *const usize,
fld1: [i64; 5],
fld2: u32,
fld3: i8,
fld4: u16,
fld5: *mut u128,

},
Variant3{
fld0: bool,
fld1: (([i8; 4],), i128, [u128; 6]),
fld2: ([i8; 2], *mut u8),
fld3: [i8; 2],
fld4: f32,
fld5: i32,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: usize,
fld1: i32,
fld2: [i64; 8],
fld3: *const usize,

},
Variant1{
fld0: [i32; 6],
fld1: (*mut u32, *mut i16),
fld2: *mut i32,
fld3: [u128; 6],
fld4: *mut u128,

},
Variant2{
fld0: (i32,),
fld1: [i8; 4],
fld2: u16,
fld3: *mut i32,
fld4: *mut i16,
fld5: [u16; 3],

},
Variant3{
fld0: ([i8; 2], *mut u8),
fld1: i128,
fld2: [bool; 5],
fld3: i8,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *mut i16,
fld1: ([i8; 2], *mut u8),
fld2: f64,
fld3: i128,
fld4: [i8; 4],
fld5: (usize, i32),
fld6: [i32; 6],

},
Variant1{
fld0: [i64; 8],
fld1: Adt41,
fld2: isize,
fld3: *mut u32,
fld4: ([i8; 2], *mut u8),
fld5: i128,

},
Variant2{
fld0: [usize; 8],
fld1: i64,

},
Variant3{
fld0: Adt40,
fld1: char,
fld2: [i64; 5],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: [i8; 2],
fld1: Adt41,
fld2: isize,
fld3: Adt42,
fld4: *mut u8,
fld5: *mut i16,
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u64,
fld1: [usize; 8],
fld2: ([i8; 2], *mut u8),
fld3: Adt40,

},
Variant1{
fld0: bool,
fld1: (usize, i32),
fld2: ([i8; 4],),
fld3: *mut u32,
fld4: i16,
fld5: [u16; 3],
fld6: i64,

},
Variant2{
fld0: (i32,),
fld1: u128,
fld2: (usize, i32),

},
Variant3{
fld0: i128,
fld1: (([i8; 4],), i128, [u128; 6]),
fld2: isize,
fld3: Adt40,
fld4: Adt42,
fld5: *mut i32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u32,
fld1: (*mut u32, *mut i16),
fld2: *mut i16,
fld3: [i64; 8],
fld4: [char; 4],
fld5: f64,
fld6: *mut i32,

},
Variant1{
fld0: u128,
fld1: Adt44,
fld2: *mut u8,
fld3: [u128; 6],
fld4: *mut i16,
fld5: Adt41,
fld6: (([i8; 4],), i128, [u128; 6]),

},
Variant2{
fld0: [i32; 6],
fld1: ([i8; 4],),
fld2: isize,
fld3: Adt40,
fld4: (usize, i32),
fld5: (*mut u32, *mut i16),
fld6: *mut i16,
fld7: f32,

},
Variant3{
fld0: *mut i32,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: i32,
fld1: [i32; 6],
fld2: *mut u128,
fld3: Adt41,
fld4: (usize, i32),
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u128; 6],
fld1: u8,
fld2: isize,
fld3: u64,

},
Variant1{
fld0: *mut u8,
fld1: [usize; 8],
fld2: f32,
fld3: ([i8; 4],),
fld4: Adt44,
fld5: Adt42,
fld6: i64,

},
Variant2{
fld0: *mut u8,

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [u16; 3],
fld1: ([i8; 4],),
fld2: (([i8; 4],), i128, [u128; 6]),
fld3: *mut u8,
fld4: [i64; 8],
fld5: i32,
fld6: *mut u128,

},
Variant1{
fld0: i64,

},
Variant2{
fld0: [i8; 2],
fld1: Adt47,
fld2: isize,

},
Variant3{
fld0: [usize; 8],
fld1: ([i8; 4],),
fld2: Adt45,
fld3: i64,
fld4: [i8; 4],
fld5: *const usize,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [i64; 5],
fld1: [i64; 8],
fld2: (([i8; 4],), i128, [u128; 6]),
fld3: (*mut u32, *mut i16),
fld4: i16,
fld5: ([i8; 2], *mut u8),
fld6: i64,
fld7: Adt47,

},
Variant1{
fld0: [bool; 5],
fld1: (([i8; 4],), i128, [u128; 6]),
fld2: u16,
fld3: Adt48,
fld4: (*mut u32, *mut i16),
fld5: ([i8; 2], *mut u8),

},
Variant2{
fld0: (i32,),
fld1: ([i8; 2], *mut u8),

},
Variant3{
fld0: [i8; 4],
fld1: Adt44,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt42,
fld1: *const usize,
fld2: (i32,),

},
Variant1{
fld0: [bool; 5],
fld1: i32,
fld2: Adt49,

},
Variant2{
fld0: [i8; 4],
fld1: Adt44,
fld2: u64,
fld3: [char; 4],
fld4: i128,
fld5: [u128; 6],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
fld0: [char; 4],
fld1: i16,
fld2: Adt48,
fld3: u32,

},
Variant1{
fld0: [u128; 6],
fld1: [i32; 6],
fld2: isize,
fld3: [usize; 8],
fld4: (usize, i32),
fld5: Adt45,
fld6: i64,
fld7: Adt40,

},
Variant2{
fld0: Adt40,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: [char; 4],
fld1: u64,
fld2: Adt51,
fld3: (*mut u32, *mut i16),
fld4: Adt43,
fld5: [u16; 3],
fld6: Adt46,
fld7: f64,
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
fld0: i64,
fld1: [u128; 6],

},
Variant1{
fld0: u16,
fld1: char,
fld2: [char; 4],
fld3: *mut u8,
fld4: *mut u128,
fld5: ([i8; 2], *mut u8),
fld6: *mut i32,

},
Variant2{
fld0: i128,
fld1: [i8; 2],
fld2: Adt47,
fld3: u8,
fld4: i16,
fld5: [i64; 5],

},
Variant3{
fld0: bool,
fld1: (*mut u32, *mut i16),

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
fld0: Adt45,
fld1: Adt42,
fld2: i128,
fld3: u8,

},
Variant1{
fld0: *mut i16,
fld1: char,
fld2: ([i8; 2], *mut u8),
fld3: [i64; 8],
fld4: *mut u128,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [i64; 5],
fld1: (*mut u32, *mut i16),
fld2: i32,
fld3: i8,
fld4: Adt52,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt47,
fld1: ([i8; 2], *mut u8),
fld2: [u128; 6],
fld3: [i8; 2],
fld4: [char; 4],
fld5: i32,
fld6: Adt41,

},
Variant1{
fld0: u16,
fld1: Adt40,

},
Variant2{
fld0: *mut u32,
fld1: f64,
fld2: i128,
fld3: ([i8; 2], *mut u8),
fld4: Adt43,

},
Variant3{
fld0: u16,
fld1: Adt41,
fld2: Adt43,
fld3: [i8; 4],
fld4: [i64; 5],
fld5: usize,

}}

