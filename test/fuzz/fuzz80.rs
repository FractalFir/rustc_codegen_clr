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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: usize,mut _4: i8) -> bool {
mir! {
type RET = bool;
let _5: char;
let _6: i128;
let _7: isize;
let _8: char;
let _9: [isize; 7];
let _10: f64;
let _11: (bool, f64, [isize; 1], u8);
let _12: *const f32;
let _13: *const (*const i128, u8, (isize,), *mut isize);
let _14: bool;
let _15: f64;
let _16: *const f32;
let _17: isize;
let _18: u16;
let _19: f64;
let _20: char;
let _21: ();
let _22: ();
{
RET = !false;
_5 = '\u{fb9c6}';
_3 = 5_usize;
RET = false & false;
_1 = _5 <= _5;
_4 = !58_i8;
_6 = -(-127782438909029130607125268326615525626_i128);
_4 = 11024227946882419679_u64 as i8;
_7 = 10664428466306470837_u64 as isize;
_1 = _3 >= _3;
_1 = !RET;
RET = !_1;
Call(RET = fn1(_7, _5, _5, _4, _4, _4, _1, _1, _3, _4, _1, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _5;
_4 = !(-30_i8);
_5 = _8;
_2 = _5;
RET = _1;
_4 = !(-48_i8);
_7 = 39_isize;
RET = !_1;
_8 = _2;
_5 = _8;
RET = !_1;
Goto(bb2)
}
bb2 = {
_8 = _2;
RET = _6 <= _6;
_2 = _8;
_2 = _8;
_10 = 2723110498_u32 as f64;
RET = _1;
RET = !_1;
_2 = _8;
_11.1 = -_10;
_11.2 = [_7];
_9 = [_7,_7,_7,_7,_7,_7,_7];
_3 = 7831556520722651041_usize << _6;
_8 = _5;
_8 = _2;
_9 = [_7,_7,_7,_7,_7,_7,_7];
_11.3 = 207_u8 << _3;
_6 = _5 as i128;
_11.3 = 198_u8;
_1 = _3 == _3;
_11.3 = _6 as u8;
RET = !_1;
_11.3 = 94_u8 ^ 86_u8;
Goto(bb3)
}
bb3 = {
_11.3 = 167_u8;
_9 = [_7,_7,_7,_7,_7,_7,_7];
RET = !_1;
_11.2 = [_7];
RET = _1;
_11.1 = _10 - _10;
_7 = !9223372036854775807_isize;
Goto(bb4)
}
bb4 = {
_11.1 = _10;
_8 = _5;
_1 = RET | RET;
_8 = _2;
_1 = !RET;
match _11.3 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
167 => bb11,
_ => bb10
}
}
bb5 = {
_11.3 = 167_u8;
_9 = [_7,_7,_7,_7,_7,_7,_7];
RET = !_1;
_11.2 = [_7];
RET = _1;
_11.1 = _10 - _10;
_7 = !9223372036854775807_isize;
Goto(bb4)
}
bb6 = {
_8 = _2;
RET = _6 <= _6;
_2 = _8;
_2 = _8;
_10 = 2723110498_u32 as f64;
RET = _1;
RET = !_1;
_2 = _8;
_11.1 = -_10;
_11.2 = [_7];
_9 = [_7,_7,_7,_7,_7,_7,_7];
_3 = 7831556520722651041_usize << _6;
_8 = _5;
_8 = _2;
_9 = [_7,_7,_7,_7,_7,_7,_7];
_11.3 = 207_u8 << _3;
_6 = _5 as i128;
_11.3 = 198_u8;
_1 = _3 == _3;
_11.3 = _6 as u8;
RET = !_1;
_11.3 = 94_u8 ^ 86_u8;
Goto(bb3)
}
bb7 = {
_8 = _5;
_4 = !(-30_i8);
_5 = _8;
_2 = _5;
RET = _1;
_4 = !(-48_i8);
_7 = 39_isize;
RET = !_1;
_8 = _2;
_5 = _8;
RET = !_1;
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
_4 = (-34_i8);
_2 = _8;
_4 = !(-63_i8);
_11.2 = [_7];
_11.0 = !RET;
_9 = [_7,_7,_7,_7,_7,_7,_7];
_5 = _8;
RET = _11.0;
_11.3 = 31_u8 | 189_u8;
_11.3 = 316465465109370025757918055940319134914_u128 as u8;
_11.1 = _10 + _10;
_2 = _5;
_5 = _8;
_3 = 10279561542721727292_usize;
_11.3 = 235_u8 | 1_u8;
_11.1 = _10 * _10;
_15 = _10;
_1 = _11.0;
_1 = _11.0;
_6 = _11.3 as i128;
_17 = _7;
_11.0 = RET;
_8 = _2;
_17 = -_7;
_4 = 7_i8;
_2 = _5;
_11.1 = _15 - _10;
match _4 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb10,
6 => bb12,
7 => bb14,
_ => bb13
}
}
bb12 = {
_8 = _2;
RET = _6 <= _6;
_2 = _8;
_2 = _8;
_10 = 2723110498_u32 as f64;
RET = _1;
RET = !_1;
_2 = _8;
_11.1 = -_10;
_11.2 = [_7];
_9 = [_7,_7,_7,_7,_7,_7,_7];
_3 = 7831556520722651041_usize << _6;
_8 = _5;
_8 = _2;
_9 = [_7,_7,_7,_7,_7,_7,_7];
_11.3 = 207_u8 << _3;
_6 = _5 as i128;
_11.3 = 198_u8;
_1 = _3 == _3;
_11.3 = _6 as u8;
RET = !_1;
_11.3 = 94_u8 ^ 86_u8;
Goto(bb3)
}
bb13 = {
_8 = _2;
RET = _6 <= _6;
_2 = _8;
_2 = _8;
_10 = 2723110498_u32 as f64;
RET = _1;
RET = !_1;
_2 = _8;
_11.1 = -_10;
_11.2 = [_7];
_9 = [_7,_7,_7,_7,_7,_7,_7];
_3 = 7831556520722651041_usize << _6;
_8 = _5;
_8 = _2;
_9 = [_7,_7,_7,_7,_7,_7,_7];
_11.3 = 207_u8 << _3;
_6 = _5 as i128;
_11.3 = 198_u8;
_1 = _3 == _3;
_11.3 = _6 as u8;
RET = !_1;
_11.3 = 94_u8 ^ 86_u8;
Goto(bb3)
}
bb14 = {
_11.0 = _5 >= _8;
_11.3 = !20_u8;
_15 = _11.1;
_4 = 52_i8 - 39_i8;
_18 = 184565637853282887364751038751963089602_u128 as u16;
_11.1 = _10 * _15;
_10 = -_11.1;
_14 = !RET;
_6 = (-11216207907206301732596061376860606794_i128);
_17 = (-1514431820_i32) as isize;
RET = _14 | _11.0;
_7 = _17;
_1 = _11.0;
_14 = _8 < _8;
_1 = !RET;
_17 = _7 ^ _7;
_6 = (-20004038114644229048344582596033155173_i128) >> _17;
_7 = _11.3 as isize;
_19 = _11.1;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(0_usize, 14_usize, Move(_14), 18_usize, Move(_18), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(0_usize, 8_usize, Move(_8), 9_usize, Move(_9), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: char,mut _3: char,mut _4: i8,mut _5: i8,mut _6: i8,mut _7: bool,mut _8: bool,mut _9: usize,mut _10: i8,mut _11: bool,mut _12: isize) -> bool {
mir! {
type RET = bool;
let _13: f32;
let _14: [i64; 6];
let _15: i8;
let _16: &'static u8;
let _17: i64;
let _18: f64;
let _19: (u8, u16, u64);
let _20: *mut *const i128;
let _21: Adt58;
let _22: (bool, f64, [isize; 1], u8);
let _23: (isize,);
let _24: u8;
let _25: isize;
let _26: (*const i128,);
let _27: [i64; 6];
let _28: i128;
let _29: char;
let _30: (f32,);
let _31: Adt58;
let _32: u16;
let _33: (isize,);
let _34: u8;
let _35: [i64; 3];
let _36: Adt46;
let _37: (f32,);
let _38: u8;
let _39: isize;
let _40: Adt46;
let _41: [i64; 6];
let _42: i32;
let _43: i32;
let _44: ();
let _45: ();
{
_10 = 542167260_i32 as i8;
RET = !_7;
_5 = _4;
_2 = _3;
_11 = _5 > _6;
_10 = _5;
_15 = !_4;
RET = _1 >= _1;
RET = _11 | _8;
_13 = _1 as f32;
_10 = _4 - _5;
_14[_9] = _7 as i64;
_10 = _4;
_12 = -_1;
RET = !_7;
_4 = _10;
_17 = !_14[_9];
Goto(bb1)
}
bb1 = {
_14 = [_17,_17,_17,_17,_17,_17];
_1 = _12 & _12;
_5 = _15 ^ _6;
_9 = !7_usize;
_12 = _2 as isize;
Call(_17 = fn2(_3, _4, _8, _2, _1, _1, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !_11;
_13 = (-56122372573640348185443924102561705984_i128) as f32;
_11 = !_7;
_3 = _2;
_19.1 = !41475_u16;
_8 = _11 | _11;
Call(_2 = fn16(_10, _7, _12, _1, _11, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17 = (-6790836934342757756_i64);
_19.2 = 338186171671496582865627408997646729172_u128 as u64;
_9 = 1074619101_i32 as usize;
_19.2 = 5096498121265643593_u64;
_12 = _5 as isize;
match _17 {
0 => bb1,
1 => bb2,
2 => bb4,
340282366920938463456583770497425453700 => bb6,
_ => bb5
}
}
bb4 = {
RET = !_11;
_13 = (-56122372573640348185443924102561705984_i128) as f32;
_11 = !_7;
_3 = _2;
_19.1 = !41475_u16;
_8 = _11 | _11;
Call(_2 = fn16(_10, _7, _12, _1, _11, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_14 = [_17,_17,_17,_17,_17,_17];
_1 = _12 & _12;
_5 = _15 ^ _6;
_9 = !7_usize;
_12 = _2 as isize;
Call(_17 = fn2(_3, _4, _8, _2, _1, _1, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_21.fld0 = _17 as i128;
_21.fld0 = (-133942696231555299283989554322313294188_i128) | 63222764210976035227758508608746598132_i128;
_16 = &_19.0;
_19.0 = 74_u8;
_6 = !_5;
_19.0 = 336003312431239671326497229916393969144_u128 as u8;
_22.3 = _19.0;
_12 = _1 - _1;
_22.1 = 23644_i16 as f64;
_14 = [_17,_17,_17,_17,_17,_17];
_16 = &_19.0;
_16 = &_24;
_22.0 = _4 <= _6;
_17 = 84051171882375524663300583464107685290_u128 as i64;
_7 = _22.3 > _22.3;
_18 = _19.0 as f64;
Goto(bb7)
}
bb7 = {
_5 = _19.2 as i8;
_4 = -_15;
_22.1 = _18;
_24 = _19.0 << _21.fld0;
_26.0 = core::ptr::addr_of!(_21.fld0);
_27 = [_17,_17,_17,_17,_17,_17];
_23.0 = _17 as isize;
_22.0 = RET;
_26.0 = core::ptr::addr_of!(_28);
_3 = _2;
_14 = _27;
_10 = _9 as i8;
_9 = !5_usize;
_19.2 = 9285890979717483634_u64;
_3 = _2;
_28 = 224064389259256867099035462602726580865_u128 as i128;
RET = _8;
_19.2 = !3234916325946551848_u64;
_18 = _17 as f64;
Goto(bb8)
}
bb8 = {
_1 = _12 & _12;
_25 = !_1;
_19.1 = 12885_u16 | 45700_u16;
_19.1 = 126196775_u32 as u16;
RET = _8 ^ _11;
_16 = &_22.3;
_21 = Adt58 { fld0: _28 };
_28 = -_21.fld0;
_4 = _5 - _10;
_20 = core::ptr::addr_of_mut!(_26.0);
_18 = 2111597178_i32 as f64;
_10 = _6;
_19.2 = 11636716192528979824_u64;
_3 = _2;
(*_20) = core::ptr::addr_of!(_28);
_14 = _27;
_22.2 = [_1];
_17 = (-7052539999450211999_i64);
(*_20) = core::ptr::addr_of!(_31.fld0);
_23 = (_25,);
_7 = !RET;
_19.2 = 16435552711514878409_u64 - 11319379900519331621_u64;
_19.0 = _24 | _24;
_21.fld0 = !_28;
_22.2 = [_1];
_30 = (_13,);
_23.0 = _25 - _25;
_21 = Adt58 { fld0: _28 };
match _17 {
0 => bb1,
1 => bb2,
2 => bb7,
340282366920938463456322067432317999457 => bb10,
_ => bb9
}
}
bb9 = {
_14 = [_17,_17,_17,_17,_17,_17];
_1 = _12 & _12;
_5 = _15 ^ _6;
_9 = !7_usize;
_12 = _2 as isize;
Call(_17 = fn2(_3, _4, _8, _2, _1, _1, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_8 = !_7;
_31 = Move(_21);
_27 = _14;
_30.0 = _13 - _13;
_15 = _6;
_11 = !_8;
RET = !_11;
_28 = !_31.fld0;
_19.2 = 14282319086172456062_u64 * 14607146021627707080_u64;
_9 = _31.fld0 as usize;
_30 = (_13,);
match _17 {
0 => bb9,
1 => bb2,
340282366920938463456322067432317999457 => bb11,
_ => bb3
}
}
bb11 = {
_21 = Move(_31);
_1 = _12;
_36.fld3 = core::ptr::addr_of_mut!(_36.fld4.0);
_36.fld4.1 = !(*_16);
_28 = _21.fld0 >> _25;
_22.3 = !_24;
_16 = &_34;
_36.fld4.2.0 = -_23.0;
_36.fld4.0 = core::ptr::addr_of!(_31.fld0);
_37 = (_13,);
_23 = (_36.fld4.2.0,);
_38 = _36.fld4.1 & _36.fld4.1;
(*_20) = core::ptr::addr_of!(_28);
_2 = _3;
_31 = Adt58 { fld0: _28 };
_36.fld4.3 = core::ptr::addr_of_mut!(_1);
_28 = _31.fld0;
_36.fld4.3 = core::ptr::addr_of_mut!(_36.fld4.2.0);
_36.fld5.1 = !_19.1;
_40.fld5.0 = !_24;
_30 = (_13,);
_40.fld2 = _36.fld5.1 * _19.1;
_18 = _22.1 + _22.1;
_21 = Adt58 { fld0: _31.fld0 };
_36.fld4.2.0 = _1;
_24 = _40.fld5.0;
match _17 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
340282366920938463456322067432317999457 => bb18,
_ => bb17
}
}
bb12 = {
_17 = (-6790836934342757756_i64);
_19.2 = 338186171671496582865627408997646729172_u128 as u64;
_9 = 1074619101_i32 as usize;
_19.2 = 5096498121265643593_u64;
_12 = _5 as isize;
match _17 {
0 => bb1,
1 => bb2,
2 => bb4,
340282366920938463456583770497425453700 => bb6,
_ => bb5
}
}
bb13 = {
_14 = [_17,_17,_17,_17,_17,_17];
_1 = _12 & _12;
_5 = _15 ^ _6;
_9 = !7_usize;
_12 = _2 as isize;
Call(_17 = fn2(_3, _4, _8, _2, _1, _1, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
RET = !_11;
_13 = (-56122372573640348185443924102561705984_i128) as f32;
_11 = !_7;
_3 = _2;
_19.1 = !41475_u16;
_8 = _11 | _11;
Call(_2 = fn16(_10, _7, _12, _1, _11, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_5 = _19.2 as i8;
_4 = -_15;
_22.1 = _18;
_24 = _19.0 << _21.fld0;
_26.0 = core::ptr::addr_of!(_21.fld0);
_27 = [_17,_17,_17,_17,_17,_17];
_23.0 = _17 as isize;
_22.0 = RET;
_26.0 = core::ptr::addr_of!(_28);
_3 = _2;
_14 = _27;
_10 = _9 as i8;
_9 = !5_usize;
_19.2 = 9285890979717483634_u64;
_3 = _2;
_28 = 224064389259256867099035462602726580865_u128 as i128;
RET = _8;
_19.2 = !3234916325946551848_u64;
_18 = _17 as f64;
Goto(bb8)
}
bb16 = {
_21.fld0 = _17 as i128;
_21.fld0 = (-133942696231555299283989554322313294188_i128) | 63222764210976035227758508608746598132_i128;
_16 = &_19.0;
_19.0 = 74_u8;
_6 = !_5;
_19.0 = 336003312431239671326497229916393969144_u128 as u8;
_22.3 = _19.0;
_12 = _1 - _1;
_22.1 = 23644_i16 as f64;
_14 = [_17,_17,_17,_17,_17,_17];
_16 = &_19.0;
_16 = &_24;
_22.0 = _4 <= _6;
_17 = 84051171882375524663300583464107685290_u128 as i64;
_7 = _22.3 > _22.3;
_18 = _19.0 as f64;
Goto(bb7)
}
bb17 = {
_14 = [_17,_17,_17,_17,_17,_17];
_1 = _12 & _12;
_5 = _15 ^ _6;
_9 = !7_usize;
_12 = _2 as isize;
Call(_17 = fn2(_3, _4, _8, _2, _1, _1, _2, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_22.2 = [_25];
_24 = _36.fld4.1 | _40.fld5.0;
_40.fld4 = _36.fld4;
_25 = _23.0;
_10 = _15;
Goto(bb19)
}
bb19 = {
Call(_44 = dump_var(1_usize, 6_usize, Move(_6), 11_usize, Move(_11), 27_usize, Move(_27), 24_usize, Move(_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(1_usize, 23_usize, Move(_23), 4_usize, Move(_4), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_44 = dump_var(1_usize, 3_usize, Move(_3), 7_usize, Move(_7), 9_usize, Move(_9), 45_usize, _45), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: char,mut _2: i8,mut _3: bool,mut _4: char,mut _5: isize,mut _6: isize,mut _7: char,mut _8: i8) -> i64 {
mir! {
type RET = i64;
let _9: [usize; 4];
let _10: [isize; 1];
let _11: i16;
let _12: (f32,);
let _13: *mut *const i128;
let _14: [i64; 3];
let _15: &'static u8;
let _16: Adt49;
let _17: ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize);
let _18: u32;
let _19: char;
let _20: (isize,);
let _21: Adt46;
let _22: f32;
let _23: u64;
let _24: f32;
let _25: u16;
let _26: ();
let _27: ();
{
RET = (-4285270759053340360_i64);
RET = _4 as i64;
_6 = _5 - _5;
RET = (-3160909901110165492_i64) & (-3313672500969010438_i64);
_3 = _8 > _2;
_10 = [_6];
RET = (-9074539009223685740_i64);
_3 = !false;
_5 = RET as isize;
_1 = _4;
_9 = [6_usize,2_usize,0_usize,13162866151919547337_usize];
_11 = _3 as i16;
_7 = _1;
_12.0 = 0_usize as f32;
RET = 895012214821658080_i64;
_11 = (-2661_i16);
Call(_10 = core::intrinsics::transmute(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = [_6];
_11 = 61086_u16 as i16;
_3 = false;
RET = 528883415173179360_u64 as i64;
_6 = !_5;
_11 = 9297_i16 + (-25498_i16);
RET = _11 as i64;
_8 = _2;
_6 = _5;
_5 = -_6;
_9 = [2_usize,9001412834010675951_usize,7_usize,14382817255815935776_usize];
RET = 5550338603747079948_i64;
_12.0 = _8 as f32;
_4 = _1;
_7 = _1;
_2 = -_8;
_12.0 = 3331463805_u32 as f32;
_2 = _11 as i8;
_5 = -_6;
_1 = _4;
_4 = _1;
_4 = _1;
RET = (-1697029724666927390_i64);
_4 = _1;
_11 = 216_u8 as i16;
Goto(bb2)
}
bb2 = {
_5 = -_6;
_11 = (-8728_i16);
_7 = _4;
_2 = _11 as i8;
_7 = _1;
_6 = _5;
Goto(bb3)
}
bb3 = {
_8 = _2 & _2;
Goto(bb4)
}
bb4 = {
_4 = _7;
_9 = [4621370754735630440_usize,895412288107712132_usize,3183258904181422698_usize,3_usize];
_2 = (-123174096689685526624456473210881752968_i128) as i8;
_9 = [12790187566104347256_usize,6_usize,6651933314735509180_usize,1996643623135836979_usize];
_4 = _1;
RET = -(-7518648224538495589_i64);
_12.0 = _6 as f32;
_1 = _7;
_3 = true;
_10 = [_5];
_2 = !_8;
_12.0 = _6 as f32;
_7 = _4;
_7 = _1;
_10 = [_6];
_12.0 = _5 as f32;
_10 = [_5];
RET = !6728580591349536274_i64;
_10 = [_6];
_5 = _6;
_7 = _4;
_8 = _2;
_6 = !_5;
_10 = [_6];
_14 = [RET,RET,RET];
RET = (-2519110990874442757_i64);
_14 = [RET,RET,RET];
match _11 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463463374607431768202728 => bb8,
_ => bb7
}
}
bb5 = {
_8 = _2 & _2;
Goto(bb4)
}
bb6 = {
_5 = -_6;
_11 = (-8728_i16);
_7 = _4;
_2 = _11 as i8;
_7 = _1;
_6 = _5;
Goto(bb3)
}
bb7 = {
_10 = [_6];
_11 = 61086_u16 as i16;
_3 = false;
RET = 528883415173179360_u64 as i64;
_6 = !_5;
_11 = 9297_i16 + (-25498_i16);
RET = _11 as i64;
_8 = _2;
_6 = _5;
_5 = -_6;
_9 = [2_usize,9001412834010675951_usize,7_usize,14382817255815935776_usize];
RET = 5550338603747079948_i64;
_12.0 = _8 as f32;
_4 = _1;
_7 = _1;
_2 = -_8;
_12.0 = 3331463805_u32 as f32;
_2 = _11 as i8;
_5 = -_6;
_1 = _4;
_4 = _1;
_4 = _1;
RET = (-1697029724666927390_i64);
_4 = _1;
_11 = 216_u8 as i16;
Goto(bb2)
}
bb8 = {
_3 = false;
_8 = !_2;
_10 = [_6];
_17.2 = 3_usize - 6_usize;
_17.1.0 = _3;
_1 = _4;
_17.1.0 = _3;
_14 = [RET,RET,RET];
_17.0 = [RET,RET,RET];
_17.4 = !(-1713951874_i32);
_17.6 = _11 as usize;
_8 = !_2;
_17.5 = 37_u8 as f64;
_21.fld4.2 = (_5,);
_17.1.1.1 = _17.5;
_21.fld6 = core::ptr::addr_of!(_21.fld4);
_21.fld0 = [RET,RET,RET,RET,RET,RET];
_21.fld4.1 = _17.4 as u8;
Call(_21.fld6 = fn3(_21.fld0, _21.fld4.2, _7, _8, _4, _17.1.0, _21.fld0, _17.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_19 = _4;
_21.fld3 = core::ptr::addr_of_mut!(_21.fld4.0);
_10 = [_6];
_7 = _4;
_20 = (_21.fld4.2.0,);
_1 = _19;
_21.fld2 = 4463_u16;
_10 = [_6];
_3 = _17.1.0;
_17.4 = (-1371902259_i32) + 1114044110_i32;
match _21.fld2 {
0 => bb1,
1 => bb2,
2 => bb7,
4463 => bb11,
_ => bb10
}
}
bb10 = {
_4 = _7;
_9 = [4621370754735630440_usize,895412288107712132_usize,3183258904181422698_usize,3_usize];
_2 = (-123174096689685526624456473210881752968_i128) as i8;
_9 = [12790187566104347256_usize,6_usize,6651933314735509180_usize,1996643623135836979_usize];
_4 = _1;
RET = -(-7518648224538495589_i64);
_12.0 = _6 as f32;
_1 = _7;
_3 = true;
_10 = [_5];
_2 = !_8;
_12.0 = _6 as f32;
_7 = _4;
_7 = _1;
_10 = [_6];
_12.0 = _5 as f32;
_10 = [_5];
RET = !6728580591349536274_i64;
_10 = [_6];
_5 = _6;
_7 = _4;
_8 = _2;
_6 = !_5;
_10 = [_6];
_14 = [RET,RET,RET];
RET = (-2519110990874442757_i64);
_14 = [RET,RET,RET];
match _11 {
0 => bb1,
1 => bb5,
2 => bb6,
340282366920938463463374607431768202728 => bb8,
_ => bb7
}
}
bb11 = {
_15 = &_21.fld5.0;
_17.1.1.3 = _17.4 as u8;
_21.fld5.1 = _21.fld2 | _21.fld2;
_13 = core::ptr::addr_of_mut!(_21.fld4.0);
_21.fld1 = core::ptr::addr_of_mut!(_6);
_21.fld0 = [RET,RET,RET,RET,RET,RET];
_17.0 = [RET,RET,RET];
_13 = core::ptr::addr_of_mut!(_21.fld4.0);
_21.fld2 = _11 as u16;
_17.1.1.3 = _21.fld4.1 | _21.fld4.1;
_22 = _12.0;
_21.fld2 = !_21.fld5.1;
_4 = _19;
_24 = _21.fld5.1 as f32;
_17.1.1 = (_17.1.0, _17.5, _10, _21.fld4.1);
_11 = (-17915_i16) >> _17.1.1.3;
_17.2 = _17.6 >> _2;
_4 = _7;
_18 = _17.1.1.3 as u32;
_15 = &_21.fld5.0;
_21.fld5 = (_21.fld4.1, _21.fld2, 10856399847009059184_u64);
_10 = [_20.0];
_17.1.1.1 = -_17.5;
_17.3 = core::ptr::addr_of!(_21.fld4);
_21.fld2 = _21.fld5.1 | _21.fld5.1;
_25 = _21.fld5.1 + _21.fld2;
_21.fld3 = core::ptr::addr_of_mut!((*_13));
_19 = _7;
match _21.fld5.2 {
0 => bb1,
1 => bb6,
2 => bb12,
3 => bb13,
10856399847009059184 => bb15,
_ => bb14
}
}
bb12 = {
_10 = [_6];
_11 = 61086_u16 as i16;
_3 = false;
RET = 528883415173179360_u64 as i64;
_6 = !_5;
_11 = 9297_i16 + (-25498_i16);
RET = _11 as i64;
_8 = _2;
_6 = _5;
_5 = -_6;
_9 = [2_usize,9001412834010675951_usize,7_usize,14382817255815935776_usize];
RET = 5550338603747079948_i64;
_12.0 = _8 as f32;
_4 = _1;
_7 = _1;
_2 = -_8;
_12.0 = 3331463805_u32 as f32;
_2 = _11 as i8;
_5 = -_6;
_1 = _4;
_4 = _1;
_4 = _1;
RET = (-1697029724666927390_i64);
_4 = _1;
_11 = 216_u8 as i16;
Goto(bb2)
}
bb13 = {
_5 = -_6;
_11 = (-8728_i16);
_7 = _4;
_2 = _11 as i8;
_7 = _1;
_6 = _5;
Goto(bb3)
}
bb14 = {
_8 = _2 & _2;
Goto(bb4)
}
bb15 = {
_15 = &_17.1.1.3;
_17.1.1.1 = _25 as f64;
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(2_usize, 9_usize, Move(_9), 8_usize, Move(_8), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(2_usize, 25_usize, Move(_25), 14_usize, Move(_14), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i64; 6],mut _2: (isize,),mut _3: char,mut _4: i8,mut _5: char,mut _6: bool,mut _7: [i64; 6],mut _8: [i64; 3]) -> *const (*const i128, u8, (isize,), *mut isize) {
mir! {
type RET = *const (*const i128, u8, (isize,), *mut isize);
let _9: Adt58;
let _10: isize;
let _11: char;
let _12: isize;
let _13: bool;
let _14: *mut char;
let _15: u32;
let _16: [isize; 1];
let _17: ();
let _18: ();
{
_1 = _7;
_5 = _3;
_2 = (105_isize,);
_3 = _5;
_3 = _5;
_1 = [(-2470519314269737802_i64),414858428640037994_i64,4247550285991207432_i64,8875166946547259985_i64,(-637249520448254580_i64),(-4678711047495807931_i64)];
_8 = [(-7388606628048518845_i64),2499859110453685637_i64,(-1306405345629928009_i64)];
_5 = _3;
_3 = _5;
_5 = _3;
_9 = Adt58 { fld0: (-82322071419087886142527437966915760974_i128) };
_10 = _2.0 ^ _2.0;
_4 = 37_i8;
_4 = 30_i8;
_7 = _1;
_8 = [4133886268045518187_i64,(-2984288242237883516_i64),1143151469426658733_i64];
_7 = [(-3446048602034901465_i64),386909271516853172_i64,(-2143251814931920513_i64),(-4335523102298164840_i64),2107077318483275619_i64,(-2194369509614050124_i64)];
_8 = [(-180935359533379572_i64),(-3503327304610756552_i64),(-1560745671305222973_i64)];
_2.0 = -_10;
_8 = [9052581879643543258_i64,8782381465955916811_i64,351258620805892320_i64];
_10 = -_2.0;
_3 = _5;
_10 = !_2.0;
Call(_1 = core::intrinsics::transmute(_7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = (_10,);
_2.0 = 4625055382275416913_usize as isize;
_3 = _5;
_9.fld0 = 147622031702717847287634723615624488336_i128 + (-118871620352288918997994711950864353842_i128);
_10 = !_2.0;
_9.fld0 = (-76018429464962784749910652399069136309_i128) & 143897785537113513062109844938555280929_i128;
_3 = _5;
_2.0 = !_10;
_10 = _2.0 + _2.0;
_10 = _2.0 >> _4;
_3 = _5;
_8 = [(-623866018733067623_i64),8022160731896651977_i64,9085771422660257914_i64];
_6 = false;
_2.0 = _9.fld0 as isize;
_8 = [1791509604176588547_i64,2423642015476549605_i64,2325889111588014803_i64];
_2.0 = _10;
_7 = _1;
_1 = [4889980008406353722_i64,(-6211586426166227051_i64),(-445161171098764131_i64),8640473576628605037_i64,(-4654522827421193102_i64),6843841702047966184_i64];
_4 = !87_i8;
_8 = [8700727105082117534_i64,8446751246166537129_i64,5706232206584373365_i64];
_5 = _3;
_6 = false;
_11 = _5;
_1 = [7338909190461008162_i64,2872678186205799257_i64,496023840625610366_i64,(-3380679758791947208_i64),2916102509643907659_i64,(-124443205359350253_i64)];
_12 = 62803_u16 as isize;
_8 = [5276842803695414146_i64,3904471603695909220_i64,2001098458742046971_i64];
_13 = _6;
Call(RET = fn4(_12, _2, _13, _8, _12, _1, _5, _6, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _5;
_8 = [(-7395155458274355106_i64),(-2380662910260831167_i64),(-6106291987254080672_i64)];
_2.0 = _10;
_1 = [8598871863892007470_i64,281201281994058660_i64,(-4810363044205193515_i64),6649013116494034909_i64,2541555645468349264_i64,(-7432184393005923840_i64)];
_15 = 2388615471_u32 - 182715251_u32;
_12 = _10 - _2.0;
_16 = [_12];
_4 = (-63_i8) | 104_i8;
_8 = [(-1358496159251495220_i64),(-7824422428423695631_i64),4644667678558419351_i64];
_1 = _7;
_13 = _6;
_5 = _11;
_14 = core::ptr::addr_of_mut!(_3);
_2 = (_12,);
_2 = (_10,);
Goto(bb3)
}
bb3 = {
Call(_17 = dump_var(3_usize, 16_usize, Move(_16), 8_usize, Move(_8), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_17 = dump_var(3_usize, 1_usize, Move(_1), 10_usize, Move(_10), 13_usize, Move(_13), 18_usize, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: (isize,),mut _3: bool,mut _4: [i64; 3],mut _5: isize,mut _6: [i64; 6],mut _7: char,mut _8: bool,mut _9: (isize,)) -> *const (*const i128, u8, (isize,), *mut isize) {
mir! {
type RET = *const (*const i128, u8, (isize,), *mut isize);
let _10: isize;
let _11: isize;
let _12: Adt55;
let _13: char;
let _14: [i64; 3];
let _15: Adt58;
let _16: isize;
let _17: (f32,);
let _18: [usize; 4];
let _19: Adt58;
let _20: [i64; 3];
let _21: f64;
let _22: Adt49;
let _23: [isize; 7];
let _24: bool;
let _25: f32;
let _26: (f32,);
let _27: char;
let _28: bool;
let _29: char;
let _30: f32;
let _31: [usize; 4];
let _32: isize;
let _33: u32;
let _34: f32;
let _35: char;
let _36: isize;
let _37: Adt52;
let _38: Adt46;
let _39: bool;
let _40: f32;
let _41: f32;
let _42: char;
let _43: f64;
let _44: Adt47;
let _45: (bool, (bool, f64, [isize; 1], u8));
let _46: Adt51;
let _47: u16;
let _48: (*const i128, u8, (isize,), *mut isize);
let _49: [isize; 1];
let _50: (*const i128,);
let _51: char;
let _52: Adt48;
let _53: u128;
let _54: isize;
let _55: [i64; 3];
let _56: ();
let _57: ();
{
_9 = (_1,);
_1 = _2.0;
_3 = !_8;
_2 = _9;
_7 = '\u{88e0e}';
_13 = _7;
_2 = (_1,);
_11 = -_5;
_10 = -_1;
_13 = _7;
_11 = 29035_u16 as isize;
_13 = _7;
_10 = _1 ^ _2.0;
_5 = _2.0;
_15.fld0 = 107013343908269251041736806583904932449_i128;
_2 = _9;
_15.fld0 = (-81848013784577738310766303706898483948_i128) >> _5;
_14 = [1274971197564848253_i64,(-1791536416670074294_i64),4797270592551142756_i64];
_11 = _10 ^ _10;
Goto(bb1)
}
bb1 = {
_13 = _7;
_6 = [(-7225441281342335969_i64),(-890150241853691869_i64),6223784206313782598_i64,(-7535968360161947900_i64),(-5168701442164690002_i64),5179005571648989381_i64];
Call(_9.0 = fn5(_4, _11, _2.0, _6, _14, _7, _15.fld0, _10, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _13;
_8 = _3 & _3;
_16 = _15.fld0 as isize;
_14 = [6898172722941797465_i64,3298488216814512561_i64,6859088812805107011_i64];
Call(_1 = fn6(Move(_15), _9.0, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _13;
_17.0 = (-28708_i16) as f32;
_20 = [(-4355148034516704564_i64),4346515871992971248_i64,4034235583254320930_i64];
_2 = (_5,);
_2 = _9;
_23 = [_1,_2.0,_2.0,_9.0,_1,_9.0,_1];
_16 = _9.0 & _9.0;
_4 = [(-6088338369364224934_i64),5430776697786273979_i64,(-3186853008053554199_i64)];
_7 = _13;
_19 = Adt58 { fld0: (-127238912764866285568044703813499268308_i128) };
_21 = 26700_u16 as f64;
_26.0 = _17.0 - _17.0;
_26 = _17;
_23 = [_1,_2.0,_1,_16,_1,_1,_11];
_15.fld0 = _19.fld0 * _19.fld0;
_3 = !_8;
_25 = (-471928280727613447_i64) as f32;
_9 = _2;
_21 = 4287535844108352368_usize as f64;
_19 = Move(_15);
Call(_26.0 = fn7(_17, _19.fld0, _2.0, _16, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_17 = _26;
_24 = _3;
Goto(bb5)
}
bb5 = {
_16 = -_2.0;
_26 = (_17.0,);
_30 = -_26.0;
_9.0 = _11 + _10;
_27 = _13;
_4 = [(-3315368540659838667_i64),(-7413053381703223961_i64),(-4912221478349268515_i64)];
Goto(bb6)
}
bb6 = {
_3 = _2.0 <= _1;
_15.fld0 = _19.fld0 << _2.0;
_4 = [3794062872923365360_i64,7767457788645364304_i64,(-2269511965301427827_i64)];
_33 = 30364_u16 as u32;
_7 = _27;
_29 = _13;
_18 = [4_usize,15531626558990053136_usize,1_usize,8820998058244412990_usize];
_34 = 38557733811746411240273598805837356859_u128 as f32;
_15 = Adt58 { fld0: _19.fld0 };
_6 = [(-4536428973766131687_i64),(-2799625335725296341_i64),1463215220567514262_i64,(-6193888340969852138_i64),(-1707489395953827112_i64),2148607202503599759_i64];
_30 = _26.0 * _26.0;
_2 = (_1,);
_32 = _16 * _2.0;
_37 = Adt52::Variant1 { fld0: _3,fld1: 1118093017_i32,fld2: _21,fld3: 7_usize };
_36 = _9.0 + _2.0;
_38.fld5.2 = 892848326356043762_i64 as u64;
_6 = [80507392421369565_i64,(-2604046240141171091_i64),(-1721118274380519668_i64),194390307058579385_i64,(-7055311861552217749_i64),6026395359035079936_i64];
_19.fld0 = !_15.fld0;
_27 = _29;
_38.fld2 = !55367_u16;
place!(Field::<i32>(Variant(_37, 1), 1)) = 2032055961_i32 | (-639060095_i32);
_38.fld2 = (-26498_i16) as u16;
Goto(bb7)
}
bb7 = {
_39 = _3;
Call(_15.fld0 = core::intrinsics::transmute(_19.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_38.fld4.1 = (-67_i8) as u8;
_38.fld0 = [6783390573964936567_i64,(-1550596955555105474_i64),(-2508587945097406867_i64),(-8966396304699394538_i64),8150977122049945501_i64,(-858453879923388639_i64)];
place!(Field::<usize>(Variant(_37, 1), 3)) = 4181585946318891665_usize ^ 1_usize;
_35 = _29;
_1 = Field::<i32>(Variant(_37, 1), 1) as isize;
_9 = (_32,);
_19 = Adt58 { fld0: _15.fld0 };
_27 = _13;
_31 = [Field::<usize>(Variant(_37, 1), 3),Field::<usize>(Variant(_37, 1), 3),Field::<usize>(Variant(_37, 1), 3),Field::<usize>(Variant(_37, 1), 3)];
_27 = _35;
_6 = _38.fld0;
_30 = Field::<i32>(Variant(_37, 1), 1) as f32;
_38.fld5.0 = _38.fld4.1 * _38.fld4.1;
_39 = _3 > _3;
RET = core::ptr::addr_of!(_38.fld4);
(*RET).0 = core::ptr::addr_of!(_15.fld0);
(*RET).1 = _38.fld5.0 - _38.fld5.0;
place!(Field::<usize>(Variant(_37, 1), 3)) = !1_usize;
_43 = -Field::<f64>(Variant(_37, 1), 2);
Goto(bb9)
}
bb9 = {
(*RET).2 = _9;
(*RET).2.0 = _32 << _38.fld2;
_38.fld1 = core::ptr::addr_of_mut!(_16);
_19 = Adt58 { fld0: _15.fld0 };
_26 = (_30,);
Goto(bb10)
}
bb10 = {
_38.fld3 = core::ptr::addr_of_mut!((*RET).0);
_17 = _26;
place!(Field::<bool>(Variant(_37, 1), 0)) = !_3;
_45.0 = !Field::<bool>(Variant(_37, 1), 0);
_19.fld0 = _15.fld0 - _15.fld0;
(*RET).2.0 = _36 - _36;
_29 = _35;
(*RET).3 = _38.fld1;
SetDiscriminant(_37, 1);
_14 = [2019062861163448889_i64,7639154530275934235_i64,8228646793358507673_i64];
Goto(bb11)
}
bb11 = {
_46.fld0 = [_38.fld4.2.0];
_7 = _29;
_9.0 = -_32;
_40 = _34;
_32 = (*RET).2.0;
(*RET).1 = (*RET).2.0 as u8;
_48 = ((*RET).0, _38.fld4.1, (*RET).2, (*RET).3);
_35 = _29;
_38.fld4.3 = _38.fld1;
(*RET).1 = _48.1;
_42 = _27;
_48.0 = core::ptr::addr_of!(_19.fld0);
_4 = [(-4037262877610274564_i64),(-978640440051084546_i64),(-7329526879616300348_i64)];
(*RET) = _48;
_10 = !_11;
_45.1 = (_45.0, _21, _46.fld0, (*RET).1);
(*RET) = (_48.0, _48.1, _48.2, _38.fld1);
_40 = 87_i8 as f32;
_15 = Adt58 { fld0: _19.fld0 };
_38.fld4.2 = (_48.2.0,);
place!(Field::<bool>(Variant(_37, 1), 0)) = _48.2.0 < _48.2.0;
Goto(bb12)
}
bb12 = {
(*RET).2 = (_2.0,);
_27 = _35;
_48.2 = _2;
_51 = _35;
(*RET).0 = core::ptr::addr_of!(_15.fld0);
_36 = _38.fld4.2.0;
_27 = _29;
_38.fld5 = ((*RET).1, _38.fld2, 10790098363945419686_u64);
RET = core::ptr::addr_of!((*RET));
(*RET).1 = !_38.fld5.0;
_46.fld1 = (*RET).1;
_43 = _45.1.1;
(*RET).3 = core::ptr::addr_of_mut!((*RET).2.0);
_50.0 = core::ptr::addr_of!(_19.fld0);
(*RET).0 = _50.0;
_17.0 = _34;
_43 = -_21;
_24 = _39;
_28 = _38.fld4.1 < (*RET).1;
_15.fld0 = _19.fld0 >> _38.fld5.0;
_2.0 = _38.fld2 as isize;
Goto(bb13)
}
bb13 = {
Call(_56 = dump_var(4_usize, 14_usize, Move(_14), 20_usize, Move(_20), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_56 = dump_var(4_usize, 3_usize, Move(_3), 7_usize, Move(_7), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_56 = dump_var(4_usize, 4_usize, Move(_4), 23_usize, Move(_23), 8_usize, Move(_8), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_56 = dump_var(4_usize, 10_usize, Move(_10), 31_usize, Move(_31), 57_usize, _57, 57_usize, _57), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [i64; 3],mut _2: isize,mut _3: isize,mut _4: [i64; 6],mut _5: [i64; 3],mut _6: char,mut _7: i128,mut _8: isize,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: [isize; 7];
let _11: isize;
let _12: Adt50;
let _13: Adt57;
let _14: isize;
let _15: (isize,);
let _16: Adt56;
let _17: bool;
let _18: [usize; 4];
let _19: i64;
let _20: f32;
let _21: bool;
let _22: u64;
let _23: [isize; 7];
let _24: ();
let _25: ();
{
_6 = '\u{6f20e}';
_8 = !_3;
_7 = 10790759071872399161618547425758958911_i128;
_10 = [_2,_9,_9,_2,_9,_8,_8];
RET = -_8;
_9 = -_2;
_2 = 4764_u16 as isize;
_4 = [7760101220835324636_i64,6295941399584683218_i64,(-4017696743243028197_i64),(-7568948900357304418_i64),(-3649943302883779812_i64),6597046092922200513_i64];
_6 = '\u{6342}';
RET = _9 * _8;
_10 = [_9,_9,RET,_9,RET,RET,RET];
_5 = _1;
RET = _9;
_8 = RET;
_11 = RET * _2;
_4 = [3893209620332099049_i64,(-2050899312542879157_i64),4602783067371439700_i64,5573191950137288801_i64,(-8956568743042508699_i64),(-3537825994080890637_i64)];
_9 = !_8;
_5 = [(-1758655477960851168_i64),(-5742629059022601302_i64),(-4740662445824603703_i64)];
_10 = [_9,_2,_11,_9,_11,_11,_3];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
10790759071872399161618547425758958911 => bb5,
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
_1 = _5;
_9 = _3 ^ _11;
RET = 1540275298_u32 as isize;
_10 = [_11,_9,_8,_11,_8,_9,_8];
RET = 50320_u16 as isize;
_11 = 1156701010_i32 as isize;
_9 = !_8;
_9 = _2;
RET = !_3;
_14 = _6 as isize;
_13.fld4 = 3847189640_u32;
_13.fld3 = 272321099323934483_i64 << RET;
_5 = [_13.fld3,_13.fld3,_13.fld3];
_13.fld3 = (-6387814909535635898_i64) * (-2952934866324402830_i64);
_1 = [_13.fld3,_13.fld3,_13.fld3];
_13.fld3 = -(-6330748607189414086_i64);
_3 = _9;
_13.fld0 = _13.fld3 as i128;
RET = _8 << _2;
_13.fld0 = _7;
match _13.fld0 {
0 => bb3,
1 => bb4,
10790759071872399161618547425758958911 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_2 = RET;
_13.fld5 = _1;
_2 = !RET;
_15 = (_9,);
_6 = '\u{b0874}';
_13.fld5 = [_13.fld3,_13.fld3,_13.fld3];
_13.fld4 = _6 as u32;
_9 = RET + _3;
_1 = [_13.fld3,_13.fld3,_13.fld3];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
10790759071872399161618547425758958911 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
_1 = _5;
_9 = _3 ^ _11;
RET = 1540275298_u32 as isize;
_10 = [_11,_9,_8,_11,_8,_9,_8];
RET = 50320_u16 as isize;
_11 = 1156701010_i32 as isize;
_9 = !_8;
_9 = _2;
RET = !_3;
_14 = _6 as isize;
_13.fld4 = 3847189640_u32;
_13.fld3 = 272321099323934483_i64 << RET;
_5 = [_13.fld3,_13.fld3,_13.fld3];
_13.fld3 = (-6387814909535635898_i64) * (-2952934866324402830_i64);
_1 = [_13.fld3,_13.fld3,_13.fld3];
_13.fld3 = -(-6330748607189414086_i64);
_3 = _9;
_13.fld0 = _13.fld3 as i128;
RET = _8 << _2;
_13.fld0 = _7;
match _13.fld0 {
0 => bb3,
1 => bb4,
10790759071872399161618547425758958911 => bb7,
_ => bb6
}
}
bb10 = {
_3 = 15094680844838276107_u64 as isize;
_1 = [_13.fld3,_13.fld3,_13.fld3];
_7 = _13.fld0;
_15.0 = (-566734362_i32) as isize;
_4 = [_13.fld3,_13.fld3,_13.fld3,_13.fld3,_13.fld3,_13.fld3];
_5 = [_13.fld3,_13.fld3,_13.fld3];
_17 = !true;
_5 = _1;
_13.fld3 = 6343040810750362876_i64 >> _14;
_13.fld4 = 44237_u16 as u32;
_6 = '\u{6d5d5}';
match _7 {
0 => bb5,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb11,
5 => bb12,
6 => bb13,
10790759071872399161618547425758958911 => bb15,
_ => bb14
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_2 = RET;
_13.fld5 = _1;
_2 = !RET;
_15 = (_9,);
_6 = '\u{b0874}';
_13.fld5 = [_13.fld3,_13.fld3,_13.fld3];
_13.fld4 = _6 as u32;
_9 = RET + _3;
_1 = [_13.fld3,_13.fld3,_13.fld3];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
10790759071872399161618547425758958911 => bb10,
_ => bb9
}
}
bb14 = {
Return()
}
bb15 = {
_18 = [13747848542767282783_usize,7393811390533270060_usize,1_usize,12482684395080588647_usize];
_13.fld3 = _17 as i64;
_19 = _13.fld3;
_13.fld5 = [_19,_19,_19];
_13.fld4 = 4213686274_u32;
_18 = [9046690187374784083_usize,4159338402237647019_usize,3_usize,10718291642677433840_usize];
_20 = _13.fld0 as f32;
_7 = _13.fld0 & _13.fld0;
_20 = 17820_i16 as f32;
_17 = !false;
_13.fld4 = 307777218_u32 - 1568897225_u32;
_13.fld5 = [_19,_13.fld3,_13.fld3];
_15 = (_9,);
_13.fld4 = 183_u8 as u32;
_21 = RET >= _2;
_13.fld4 = 1934474785_i32 as u32;
_14 = 40798200606205206014350969590758417858_u128 as isize;
_6 = '\u{7f243}';
_13.fld0 = _7;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(5_usize, 21_usize, Move(_21), 2_usize, Move(_2), 10_usize, Move(_10), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(5_usize, 3_usize, Move(_3), 9_usize, Move(_9), 19_usize, Move(_19), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: Adt58,mut _2: isize,mut _3: char) -> isize {
mir! {
type RET = isize;
let _4: u16;
let _5: char;
let _6: ();
let _7: ();
{
_1 = Adt58 { fld0: (-38066617800340882048708376803148049711_i128) };
RET = !_2;
RET = _2;
_1.fld0 = !(-28596945380200922031787570155900829902_i128);
_1 = Adt58 { fld0: (-24004519321503457070625564533008354485_i128) };
_1.fld0 = 46420635873679056560269591109604540846_i128;
_2 = !RET;
RET = _2 + _2;
RET = _2;
Call(RET = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = Adt58 { fld0: (-79475504303780270199370599837264603344_i128) };
_2 = RET - RET;
RET = -_2;
_1 = Adt58 { fld0: 101989191813117621156562221015065277768_i128 };
_1.fld0 = -(-19181186055815232490339556156965828186_i128);
_1.fld0 = 108981666507725943104740956811928125563_i128;
RET = _2;
_1.fld0 = !38765716534082383847984260352945583873_i128;
_3 = '\u{fbd06}';
_1.fld0 = (-5788474117314273586_i64) as i128;
_4 = 36268_u16;
_5 = _3;
_5 = _3;
_4 = 368059900_u32 as u16;
_5 = _3;
_2 = !RET;
_2 = 3_usize as isize;
Goto(bb2)
}
bb2 = {
Call(_6 = dump_var(6_usize, 5_usize, Move(_5), 4_usize, Move(_4), 7_usize, _7, 7_usize, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: (f32,),mut _2: i128,mut _3: isize,mut _4: isize,mut _5: (isize,)) -> f32 {
mir! {
type RET = f32;
let _6: bool;
let _7: (isize,);
let _8: i64;
let _9: [isize; 1];
let _10: (f32, ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64), (bool, (bool, f64, [isize; 1], u8)), usize, char, (i128, u8, *mut char));
let _11: Adt58;
let _12: isize;
let _13: Adt58;
let _14: isize;
let _15: f64;
let _16: u32;
let _17: char;
let _18: (f32,);
let _19: i128;
let _20: f32;
let _21: [i64; 3];
let _22: char;
let _23: Adt59;
let _24: i8;
let _25: Adt58;
let _26: u128;
let _27: f64;
let _28: f64;
let _29: Adt58;
let _30: (f32,);
let _31: isize;
let _32: u128;
let _33: *const *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize);
let _34: u128;
let _35: (bool, f64, [isize; 1], u8);
let _36: ();
let _37: ();
{
_5 = (_4,);
RET = _1.0;
_5 = (_4,);
_1 = (RET,);
_1 = (RET,);
_6 = true;
_1.0 = RET + RET;
_2 = !18396369121742884551457297233454278156_i128;
RET = -_1.0;
_5 = (_4,);
_7.0 = 1_usize as isize;
_5.0 = _7.0 + _3;
RET = -_1.0;
_1 = (RET,);
_6 = _3 != _4;
_7 = (_4,);
_2 = (-109862539424245312499568977351421577135_i128);
_4 = !_5.0;
_4 = _3;
RET = 659071707_i32 as f32;
_6 = _5.0 >= _7.0;
_2 = (-80745412408795311087696546147557780739_i128);
_5 = _7;
_6 = !false;
_4 = 4055006545746289458_u64 as isize;
Call(_6 = fn8(_5, _5, _5.0, _7.0, _7.0, _7, _5.0, _7.0, _5.0, _5.0, _1.0, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _5.0 >> _5.0;
_8 = 3704507264_u32 as i64;
_7 = _5;
RET = -_1.0;
match _2 {
0 => bb2,
1 => bb3,
259536954512143152375678061284210430717 => bb5,
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
_1.0 = RET;
_10.2.0 = _6 > _6;
_10.1.1.5 = 12080_u16 as f64;
_1 = (RET,);
_10.4 = '\u{e9f7b}';
_10.1.1.1.1.3 = 207_u8 - 171_u8;
_10.1.1.0 = [_8,_8,_8];
RET = -_1.0;
_10.0 = RET + _1.0;
_10.5.1 = _10.1.1.1.1.3 >> _7.0;
_3 = -_5.0;
_11.fld0 = (-25025_i16) as i128;
_10.1.1.1.1.2 = [_3];
_10.2.1.3 = _10.1.1.1.1.3;
_7 = _5;
_2 = _11.fld0 - _11.fld0;
_10.0 = -RET;
_10.1.0.0 = !_6;
_10.1.1.1.0 = _10.1.0.0;
_10.1.1.1.1.2 = [_3];
_10.1.1.0 = [_8,_8,_8];
_10.1.1.5 = _10.0 as f64;
_10.1.0 = (_6, _10.1.1.5, _10.1.1.1.1.2, _10.5.1);
Call(_7.0 = fn9(_10.5.1, _10.1.0, _10.1.1.1.0, _3, _10.1.1.1.1.2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = -_11.fld0;
_9 = [_3];
_10.2.1.3 = !_10.5.1;
_10.1.1.2 = _8 as usize;
_1 = (_10.0,);
_10.1.1.1.1.3 = !_10.5.1;
_12 = _5.0;
_13.fld0 = !_2;
RET = _10.0;
_15 = _10.1.0.1 - _10.1.1.5;
_10.3 = !_10.1.1.2;
_10.1.0.2 = [_3];
_10.1.1.1.1.3 = _10.2.1.3;
Goto(bb7)
}
bb7 = {
RET = _8 as f32;
_11.fld0 = 222245136585643066909161085363357353008_u128 as i128;
Goto(bb8)
}
bb8 = {
_10.2.0 = _10.1.1.1.0 <= _6;
_13.fld0 = -_2;
_10.5.0 = _11.fld0;
_5.0 = (-23_i8) as isize;
_13 = Adt58 { fld0: _10.5.0 };
_14 = _12;
_10.2.1.2 = [_14];
_6 = _10.2.0;
_7 = _5;
_10.1.1.0 = [_8,_8,_8];
_10.5.1 = _10.2.1.3;
_10.1.1.6 = (-52_i8) as usize;
_10.1.1.1.0 = !_10.2.0;
_10.1.0.1 = _15;
RET = _8 as f32;
_10.2.1.0 = _6 & _10.2.0;
_1.0 = RET;
_10.1.1.1.1.1 = 482343957_i32 as f64;
_10.2.1.2 = _9;
Call(_10.1.0.2 = core::intrinsics::transmute(_10.2.1.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5.0 = _13.fld0 as isize;
_1.0 = _10.0;
_10.2.1.3 = _10.1.0.3 | _10.5.1;
_10.2.1.0 = _10.2.0 & _6;
_10.1.1.2 = !_10.1.1.6;
_11.fld0 = _2;
_10.1.0.2 = _9;
_10.2 = (_10.1.1.1.0, _10.1.0);
Call(_10.3 = fn15(_10.2.1.0, _10.1.0, _10.1.0, _10.2.1, _10.2.1, _10.1.1.1.0, _10.2.1.0, _10.1.1.1.0, _10.1.1.1.0, _9, _10.1.0.0, _10.2.1, _10.1.0.0, _10.1.1.1.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10.1.1.1.1.2 = [_3];
_10.5.2 = core::ptr::addr_of_mut!(_17);
RET = -_10.0;
RET = _1.0 - _1.0;
_6 = !_10.1.1.1.0;
_10.2.1.1 = _10.1.1.1.1.1;
_17 = _10.4;
_10.5.1 = _10.1.1.1.1.3;
_10.5.1 = !_10.1.1.1.1.3;
_10.1.1.1.1.0 = _6 < _10.2.0;
_23.fld0 = core::ptr::addr_of!(_20);
_10.5.2 = core::ptr::addr_of_mut!(_17);
_23.fld3.fld5 = [_8,_8,_8];
_10.1.1.0 = [_8,_8,_8];
_6 = _10.1.1.1.1.3 != _10.1.0.3;
_10.2.1.2 = [_14];
_10.1.1.1.1.2 = [_12];
_23.fld3.fld5 = [_8,_8,_8];
_13.fld0 = !_2;
_10.1.0.3 = _10.5.1;
_10.1.1.1.1 = (_10.1.1.1.0, _15, _10.2.1.2, _10.1.0.3);
Goto(bb11)
}
bb11 = {
_4 = -_12;
_18.0 = _10.0 * _1.0;
_10.1.0.1 = _10.1.1.1.1.1 * _15;
_10.1.1.5 = _10.1.1.1.1.3 as f64;
_12 = _14;
RET = _18.0;
_10.1.1.6 = _10.1.1.2;
_1 = (RET,);
_16 = _10.2.1.0 as u32;
_23.fld0 = core::ptr::addr_of!(_20);
_22 = _17;
Goto(bb12)
}
bb12 = {
_10.2.1.3 = !_10.1.1.1.1.3;
_10.1.1.6 = _10.3 >> _10.3;
_23.fld1 = _10.5;
_10.3 = _10.1.1.6 ^ _10.1.1.6;
_10.4 = _17;
_10.3 = _10.1.1.6 | _10.1.1.6;
_11.fld0 = -_13.fld0;
_10.1.0.0 = _10.1.1.1.1.0;
_15 = -_10.1.1.5;
_11 = Adt58 { fld0: _13.fld0 };
_10.2.1.0 = !_10.1.1.1.0;
_10.1.2 = _8 >> _10.1.1.6;
_10.2 = (_10.1.1.1.1.0, _10.1.1.1.1);
_10.1.1.1 = _10.2;
_8 = !_10.1.2;
Goto(bb13)
}
bb13 = {
_13.fld0 = _23.fld1.0 << _10.3;
_13.fld0 = _10.5.0;
_29 = Adt58 { fld0: _2 };
_23.fld3.fld3 = -_8;
_23.fld1.2 = core::ptr::addr_of_mut!(_22);
_26 = 260926849115869047420222345766854206663_u128;
_23.fld1.2 = core::ptr::addr_of_mut!(_22);
_30 = (_1.0,);
_29 = Adt58 { fld0: _13.fld0 };
_8 = _23.fld3.fld3;
_10.5 = (_11.fld0, _10.1.1.1.1.3, _23.fld1.2);
_21 = [_10.1.2,_8,_10.1.2];
_28 = _15 * _15;
match _26 {
0 => bb9,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb14,
5 => bb15,
6 => bb16,
260926849115869047420222345766854206663 => bb18,
_ => bb17
}
}
bb14 = {
_10.2.1.3 = !_10.1.1.1.1.3;
_10.1.1.6 = _10.3 >> _10.3;
_23.fld1 = _10.5;
_10.3 = _10.1.1.6 ^ _10.1.1.6;
_10.4 = _17;
_10.3 = _10.1.1.6 | _10.1.1.6;
_11.fld0 = -_13.fld0;
_10.1.0.0 = _10.1.1.1.1.0;
_15 = -_10.1.1.5;
_11 = Adt58 { fld0: _13.fld0 };
_10.2.1.0 = !_10.1.1.1.0;
_10.1.2 = _8 >> _10.1.1.6;
_10.2 = (_10.1.1.1.1.0, _10.1.1.1.1);
_10.1.1.1 = _10.2;
_8 = !_10.1.2;
Goto(bb13)
}
bb15 = {
_2 = -_11.fld0;
_9 = [_3];
_10.2.1.3 = !_10.5.1;
_10.1.1.2 = _8 as usize;
_1 = (_10.0,);
_10.1.1.1.1.3 = !_10.5.1;
_12 = _5.0;
_13.fld0 = !_2;
RET = _10.0;
_15 = _10.1.0.1 - _10.1.1.5;
_10.3 = !_10.1.1.2;
_10.1.0.2 = [_3];
_10.1.1.1.1.3 = _10.2.1.3;
Goto(bb7)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_7.0 = -_14;
_35 = (_10.1.1.1.1.0, _10.1.1.5, _9, _10.1.1.1.1.3);
_23.fld1 = (_13.fld0, _10.1.1.1.1.3, _10.5.2);
_29 = Adt58 { fld0: _23.fld1.0 };
_35.1 = _26 as f64;
_5 = (_7.0,);
Goto(bb19)
}
bb19 = {
Call(_36 = dump_var(7_usize, 4_usize, Move(_4), 26_usize, Move(_26), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_36 = dump_var(7_usize, 8_usize, Move(_8), 3_usize, Move(_3), 7_usize, Move(_7), 37_usize, _37), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: (isize,),mut _2: (isize,),mut _3: isize,mut _4: isize,mut _5: isize,mut _6: (isize,),mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: f32,mut _12: isize,mut _13: (isize,)) -> bool {
mir! {
type RET = bool;
let _14: bool;
let _15: f32;
let _16: f32;
let _17: i128;
let _18: u128;
let _19: usize;
let _20: isize;
let _21: isize;
let _22: Adt57;
let _23: i128;
let _24: (bool, f64, [isize; 1], u8);
let _25: isize;
let _26: char;
let _27: usize;
let _28: i32;
let _29: ();
let _30: ();
{
_10 = !_9;
_14 = true;
_13 = (_6.0,);
RET = _3 < _8;
_8 = _10;
_7 = _9;
_10 = _8;
_12 = 255290318_u32 as isize;
_6.0 = _4 ^ _5;
_1.0 = 761902285_i32 as isize;
_12 = (-2516113176155005126_i64) as isize;
_13 = (_7,);
_7 = _2.0 >> _6.0;
RET = _14;
_6.0 = _3 & _8;
_7 = _10 * _13.0;
_17 = _14 as i128;
_1 = _6;
_6 = (_7,);
_10 = -_7;
_3 = _10 & _8;
_6.0 = _10 ^ _10;
_8 = _7 << _2.0;
Goto(bb1)
}
bb1 = {
_12 = _8;
_14 = !RET;
_13.0 = _12 | _6.0;
_11 = 15610542300198850637_u64 as f32;
_2.0 = _6.0 | _13.0;
_20 = 193039397097093486028291411313899704569_u128 as isize;
_6.0 = _2.0 + _12;
_16 = 1916681060_u32 as f32;
_4 = _7;
_22.fld0 = _17;
_4 = _6.0 + _6.0;
_11 = _16 * _16;
_22.fld3 = _17 as i64;
_10 = _7 << _12;
_22.fld0 = 550256751_i32 as i128;
_11 = 14800_i16 as f32;
_22.fld3 = 7514496040921936015_i64 ^ (-6307567126858280862_i64);
_19 = !1_usize;
_15 = -_11;
RET = !_14;
_1.0 = _10;
_23 = 5363308884616635384_u64 as i128;
Goto(bb2)
}
bb2 = {
RET = _14;
_22.fld3 = -(-4395204447799329751_i64);
_18 = RET as u128;
_4 = 17_i8 as isize;
_8 = !_2.0;
_6 = _2;
_6 = (_1.0,);
Goto(bb3)
}
bb3 = {
_21 = _16 as isize;
_9 = !_2.0;
_20 = _7;
_19 = 1869510702718794356_usize;
RET = _1.0 == _7;
_9 = _16 as isize;
_24.1 = _16 as f64;
_3 = -_6.0;
_7 = _1.0 << _8;
_21 = _22.fld3 as isize;
_8 = _13.0 >> _2.0;
_20 = _10 << _13.0;
_9 = _12;
_26 = '\u{345e7}';
_22.fld4 = 2634603058_u32;
_16 = _11;
_16 = 46891_u16 as f32;
_24.3 = _22.fld3 as u8;
_22.fld5 = [_22.fld3,_22.fld3,_22.fld3];
_25 = _22.fld4 as isize;
_12 = -_3;
_2.0 = _20 ^ _20;
Goto(bb4)
}
bb4 = {
Call(_29 = dump_var(8_usize, 13_usize, Move(_13), 10_usize, Move(_10), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_29 = dump_var(8_usize, 25_usize, Move(_25), 12_usize, Move(_12), 20_usize, Move(_20), 3_usize, Move(_3)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_29 = dump_var(8_usize, 14_usize, Move(_14), 7_usize, Move(_7), 30_usize, _30, 30_usize, _30), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u8,mut _2: (bool, f64, [isize; 1], u8),mut _3: bool,mut _4: isize,mut _5: [isize; 1]) -> isize {
mir! {
type RET = isize;
let _6: char;
let _7: u8;
let _8: u32;
let _9: u32;
let _10: Adt58;
let _11: bool;
let _12: [i64; 6];
let _13: [isize; 7];
let _14: isize;
let _15: (isize,);
let _16: isize;
let _17: ([i64; 3], (i128, u8, *mut char), *mut char);
let _18: Adt58;
let _19: u128;
let _20: f64;
let _21: Adt46;
let _22: bool;
let _23: (bool, f64, [isize; 1], u8);
let _24: (f32,);
let _25: u128;
let _26: *const i128;
let _27: f32;
let _28: i64;
let _29: ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize);
let _30: (bool, f64, [isize; 1], u8);
let _31: i64;
let _32: f32;
let _33: ();
let _34: ();
{
RET = _4 >> _1;
_2.1 = 104_i8 as f64;
_2.2 = _5;
RET = _4 + _4;
RET = _4;
_4 = _2.1 as isize;
_1 = 1389_u16 as u8;
_7 = _2.3;
RET = _4;
_8 = 122696090512901124433519050123800787406_i128 as u32;
_2.1 = _7 as f64;
_2.1 = 1260384493_i32 as f64;
_9 = (-27012_i16) as u32;
_4 = -RET;
_2.0 = _3;
_7 = _2.3;
_10.fld0 = (-137813912360376049066157848305839400415_i128);
_5 = [_4];
_10 = Adt58 { fld0: (-96839525744864967041024731646523117759_i128) };
_6 = '\u{fb68f}';
RET = _6 as isize;
_7 = _2.3;
Goto(bb1)
}
bb1 = {
_11 = _3;
_3 = _11;
_5 = [RET];
_2.2 = [_4];
_9 = !_8;
_9 = 40713_u16 as u32;
_10.fld0 = 7800_u16 as i128;
_6 = '\u{6f041}';
_8 = _9 ^ _9;
_11 = !_3;
_12 = [(-6049179306474888066_i64),(-1481378398840514211_i64),(-220947325883911317_i64),(-681575221456158710_i64),7795140588323971313_i64,(-4059047186990406265_i64)];
_13 = [RET,_4,_4,_4,_4,_4,_4];
_1 = !_7;
_5 = [_4];
Call(_2.0 = fn10(_9, _4, _3, _3, _11, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _4 + _4;
_14 = -_4;
_7 = _2.3;
_2.2 = [RET];
_6 = '\u{bdc70}';
_10.fld0 = 137018353740647916695973336750298858344_i128 - 90938932504660958222118023354483274943_i128;
_8 = _9;
_8 = _9 * _9;
_6 = '\u{6e727}';
_9 = _8;
RET = _14;
_13 = [_4,RET,RET,_4,_4,_14,_14];
RET = _4 * _14;
_15.0 = _14 ^ _14;
_7 = _1 & _1;
_2.3 = !_7;
RET = _15.0;
_2.0 = _2.3 >= _7;
_6 = '\u{5311e}';
_9 = !_8;
RET = -_14;
_15 = (RET,);
_10.fld0 = (-39506467342246360527163952166781932336_i128);
_17.1.0 = _10.fld0;
_17.2 = core::ptr::addr_of_mut!(_6);
Goto(bb3)
}
bb3 = {
_5 = [_15.0];
_17.2 = core::ptr::addr_of_mut!(_6);
_3 = _2.0 ^ _11;
_6 = '\u{83aa0}';
_17.2 = core::ptr::addr_of_mut!(_6);
_2.0 = _11;
_17.1.2 = _17.2;
_10 = Adt58 { fld0: _17.1.0 };
_4 = -_14;
_4 = _15.0;
_17.0 = [248129666592808491_i64,1371237857057729949_i64,7374214032272044401_i64];
_9 = _8;
_10.fld0 = _17.1.0 ^ _17.1.0;
_17.1 = (_10.fld0, _2.3, _17.2);
_17.1.1 = !_2.3;
Goto(bb4)
}
bb4 = {
_2.1 = _17.1.0 as f64;
_8 = RET as u32;
RET = _14;
_1 = _17.1.1;
_21.fld5 = (_2.3, 38500_u16, 3866414878716378580_u64);
_21.fld5.1 = !7172_u16;
_21.fld1 = core::ptr::addr_of_mut!(RET);
_21.fld4.2.0 = RET ^ RET;
_18.fld0 = _10.fld0;
_17.2 = _17.1.2;
_21.fld4.2 = _15;
_21.fld5.2 = 4753111736418300590_u64 ^ 5983819738799604351_u64;
Goto(bb5)
}
bb5 = {
_17.2 = _17.1.2;
Call(_16 = fn11(_2.0, _17.1, _17.1, _2.3, _2, _2.0, _21.fld5, _2.0, _2.0, _2, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_20 = _2.3 as f64;
_10 = Adt58 { fld0: _17.1.0 };
_20 = -_2.1;
_21.fld0 = [2184136942140330107_i64,(-8998086215232462288_i64),3515554535952842888_i64,(-3801940736410479395_i64),(-6842578137708449211_i64),2569220451932834428_i64];
RET = _16;
_18.fld0 = 185309910796768350510008025006905433702_u128 as i128;
_15.0 = -RET;
_21.fld4.1 = !_2.3;
_17.1 = (_18.fld0, _21.fld4.1, _17.2);
RET = 1513072931827694403_usize as isize;
_22 = _3;
_21.fld5.2 = 4025150405758792107_u64 + 11549180315966971016_u64;
_17.1.2 = core::ptr::addr_of_mut!(_6);
Call(_21.fld3 = fn13(_3, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = Adt58 { fld0: _17.1.0 };
_21.fld5.1 = !35921_u16;
_18 = Adt58 { fld0: _17.1.0 };
_21.fld1 = core::ptr::addr_of_mut!(_21.fld4.2.0);
_4 = _15.0;
_21.fld4.1 = !_1;
_21.fld3 = core::ptr::addr_of_mut!(_21.fld4.0);
_21.fld4.1 = _2.3;
_9 = _8;
_21.fld4.1 = _2.3 | _17.1.1;
_21.fld5 = (_7, 11816_u16, 5009803886538405031_u64);
_19 = 172656212715386372044875232574143668649_u128;
_9 = !_8;
_21.fld4.0 = core::ptr::addr_of!(_17.1.0);
_21.fld6 = core::ptr::addr_of!(_21.fld4);
_2 = (_3, _20, _5, _1);
match _21.fld5.2 {
0 => bb5,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
5009803886538405031 => bb13,
_ => bb12
}
}
bb8 = {
_20 = _2.3 as f64;
_10 = Adt58 { fld0: _17.1.0 };
_20 = -_2.1;
_21.fld0 = [2184136942140330107_i64,(-8998086215232462288_i64),3515554535952842888_i64,(-3801940736410479395_i64),(-6842578137708449211_i64),2569220451932834428_i64];
RET = _16;
_18.fld0 = 185309910796768350510008025006905433702_u128 as i128;
_15.0 = -RET;
_21.fld4.1 = !_2.3;
_17.1 = (_18.fld0, _21.fld4.1, _17.2);
RET = 1513072931827694403_usize as isize;
_22 = _3;
_21.fld5.2 = 4025150405758792107_u64 + 11549180315966971016_u64;
_17.1.2 = core::ptr::addr_of_mut!(_6);
Call(_21.fld3 = fn13(_3, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_11 = _3;
_3 = _11;
_5 = [RET];
_2.2 = [_4];
_9 = !_8;
_9 = 40713_u16 as u32;
_10.fld0 = 7800_u16 as i128;
_6 = '\u{6f041}';
_8 = _9 ^ _9;
_11 = !_3;
_12 = [(-6049179306474888066_i64),(-1481378398840514211_i64),(-220947325883911317_i64),(-681575221456158710_i64),7795140588323971313_i64,(-4059047186990406265_i64)];
_13 = [RET,_4,_4,_4,_4,_4,_4];
_1 = !_7;
_5 = [_4];
Call(_2.0 = fn10(_9, _4, _3, _3, _11, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_2.1 = _17.1.0 as f64;
_8 = RET as u32;
RET = _14;
_1 = _17.1.1;
_21.fld5 = (_2.3, 38500_u16, 3866414878716378580_u64);
_21.fld5.1 = !7172_u16;
_21.fld1 = core::ptr::addr_of_mut!(RET);
_21.fld4.2.0 = RET ^ RET;
_18.fld0 = _10.fld0;
_17.2 = _17.1.2;
_21.fld4.2 = _15;
_21.fld5.2 = 4753111736418300590_u64 ^ 5983819738799604351_u64;
Goto(bb5)
}
bb11 = {
_5 = [_15.0];
_17.2 = core::ptr::addr_of_mut!(_6);
_3 = _2.0 ^ _11;
_6 = '\u{83aa0}';
_17.2 = core::ptr::addr_of_mut!(_6);
_2.0 = _11;
_17.1.2 = _17.2;
_10 = Adt58 { fld0: _17.1.0 };
_4 = -_14;
_4 = _15.0;
_17.0 = [248129666592808491_i64,1371237857057729949_i64,7374214032272044401_i64];
_9 = _8;
_10.fld0 = _17.1.0 ^ _17.1.0;
_17.1 = (_10.fld0, _2.3, _17.2);
_17.1.1 = !_2.3;
Goto(bb4)
}
bb12 = {
RET = _4 + _4;
_14 = -_4;
_7 = _2.3;
_2.2 = [RET];
_6 = '\u{bdc70}';
_10.fld0 = 137018353740647916695973336750298858344_i128 - 90938932504660958222118023354483274943_i128;
_8 = _9;
_8 = _9 * _9;
_6 = '\u{6e727}';
_9 = _8;
RET = _14;
_13 = [_4,RET,RET,_4,_4,_14,_14];
RET = _4 * _14;
_15.0 = _14 ^ _14;
_7 = _1 & _1;
_2.3 = !_7;
RET = _15.0;
_2.0 = _2.3 >= _7;
_6 = '\u{5311e}';
_9 = !_8;
RET = -_14;
_15 = (RET,);
_10.fld0 = (-39506467342246360527163952166781932336_i128);
_17.1.0 = _10.fld0;
_17.2 = core::ptr::addr_of_mut!(_6);
Goto(bb3)
}
bb13 = {
_23.3 = _1 + _21.fld4.1;
_19 = 260280806854760515413129251905044662652_u128 ^ 6858285141878900611657844897261713959_u128;
_29.1 = (_3, _2);
_10 = Move(_18);
_17.1 = (_10.fld0, _21.fld5.0, _17.2);
_17.0 = [4556820213836412765_i64,(-8093744600532151330_i64),1854302495604308741_i64];
_23 = _2;
_9 = _8 ^ _8;
_18 = Adt58 { fld0: _17.1.0 };
_29.6 = 12679428176698104612_usize;
_22 = _29.1.0 == _29.1.0;
_27 = _9 as f32;
_21.fld5.2 = !4201172405709175772_u64;
_24.0 = _27;
_29.1 = (_11, _23);
_21.fld5.1 = 63996_u16;
RET = -_15.0;
_10.fld0 = !_17.1.0;
_11 = _16 >= _16;
_21.fld5.2 = 12115297195136822818_u64 * 4787930186112024604_u64;
_2.1 = _29.1.1.1 - _20;
RET = _4 * _4;
_29.1.0 = _11;
_2.0 = _29.1.1.0;
_29.1.1.0 = _22 == _11;
RET = _21.fld5.2 as isize;
match _21.fld5.1 {
63996 => bb14,
_ => bb10
}
}
bb14 = {
_11 = _23.0 ^ _22;
_29.1.1.0 = _15.0 >= _15.0;
_29.3 = core::ptr::addr_of!(_21.fld4);
_30.1 = -_2.1;
_24.0 = _27 * _27;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(9_usize, 14_usize, Move(_14), 8_usize, Move(_8), 4_usize, Move(_4), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(9_usize, 22_usize, Move(_22), 12_usize, Move(_12), 19_usize, Move(_19), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u32,mut _2: isize,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: isize) -> bool {
mir! {
type RET = bool;
let _8: Adt58;
let _9: i16;
let _10: ();
let _11: ();
{
_4 = _6 != _5;
RET = !_6;
_8 = Adt58 { fld0: (-101634964938318941022658486354678722254_i128) };
RET = _6 >= _3;
_4 = _6;
_8 = Adt58 { fld0: 65277459799599976650645947913902518918_i128 };
_5 = _6 == RET;
_5 = _3;
_1 = 4069840250_u32 + 1516840389_u32;
_9 = 2033799042_i32 as i16;
_4 = RET;
_9 = (-27655_i16);
_5 = _6;
_9 = (-9584_i16) >> _1;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(10_usize, 9_usize, Move(_9), 2_usize, Move(_2), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: bool,mut _2: (i128, u8, *mut char),mut _3: (i128, u8, *mut char),mut _4: u8,mut _5: (bool, f64, [isize; 1], u8),mut _6: bool,mut _7: (u8, u16, u64),mut _8: bool,mut _9: bool,mut _10: (bool, f64, [isize; 1], u8),mut _11: u8) -> isize {
mir! {
type RET = isize;
let _12: f64;
let _13: (isize,);
let _14: [i64; 6];
let _15: bool;
let _16: [isize; 1];
let _17: (f32,);
let _18: (*const i128, u8, (isize,), *mut isize);
let _19: Adt48;
let _20: (bool, f64, [isize; 1], u8);
let _21: [i64; 3];
let _22: (bool, (bool, f64, [isize; 1], u8));
let _23: (f32,);
let _24: (*const i128, u8, (isize,), *mut isize);
let _25: bool;
let _26: [isize; 1];
let _27: Adt55;
let _28: ();
let _29: ();
{
_5.1 = _10.1;
_10.3 = _5.3;
Goto(bb1)
}
bb1 = {
_5 = (_6, _10.1, _10.2, _4);
_12 = -_10.1;
_9 = !_10.0;
_4 = _2.1 | _7.0;
_3.2 = _2.2;
_5.1 = _12 * _12;
_3.1 = !_2.1;
_11 = '\u{29842}' as u8;
_2.0 = -_3.0;
RET = 6_isize;
_5.2 = [RET];
Goto(bb2)
}
bb2 = {
_7.0 = _10.3 ^ _3.1;
_5.2 = _10.2;
_10.2 = _5.2;
_15 = _6;
_10.2 = [RET];
_3.0 = !_2.0;
_5 = (_10.0, _10.1, _10.2, _2.1);
_3.2 = _2.2;
_5.1 = _10.1;
_10.2 = [RET];
_13.0 = RET;
_10.1 = _5.1;
_11 = _7.0 * _5.3;
_5 = (_10.0, _10.1, _10.2, _10.3);
_13.0 = RET;
_9 = _5.0;
_10.0 = _6;
_14 = [1518462081220220063_i64,(-6690264295732764125_i64),(-3567459979183826133_i64),4902700669384448785_i64,8564415196945283570_i64,6397516648935305354_i64];
_13.0 = RET;
_5.1 = _7.2 as f64;
_13 = (RET,);
_3.0 = _2.0;
_5.1 = _10.1;
_2.1 = 44_i8 as u8;
_3.1 = !_5.3;
_2.1 = _11;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
6 => bb6,
_ => bb5
}
}
bb3 = {
_5 = (_6, _10.1, _10.2, _4);
_12 = -_10.1;
_9 = !_10.0;
_4 = _2.1 | _7.0;
_3.2 = _2.2;
_5.1 = _12 * _12;
_3.1 = !_2.1;
_11 = '\u{29842}' as u8;
_2.0 = -_3.0;
RET = 6_isize;
_5.2 = [RET];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_5.1 = _12 + _10.1;
_10.2 = [RET];
_4 = _7.1 as u8;
_2.1 = 17_i8 as u8;
RET = _3.0 as isize;
_5.2 = [RET];
_5.0 = _15;
_7.1 = _5.1 as u16;
RET = -_13.0;
_9 = !_8;
_1 = _9;
_13.0 = _1 as isize;
_9 = _3.1 >= _7.0;
_13.0 = '\u{e0636}' as isize;
_13 = (RET,);
Goto(bb7)
}
bb7 = {
_10.2 = [_13.0];
_7.1 = 4269729827_u32 as u16;
_5.1 = _10.1 + _10.1;
_10.1 = _5.1;
_3.1 = _11;
_10.3 = !_3.1;
_10.3 = !_7.0;
_3.0 = -_2.0;
_15 = !_9;
_10.1 = 3_usize as f64;
_18.2 = _13;
_10 = (_6, _5.1, _5.2, _3.1);
Call(_18.2 = fn12(_15, _10.1, _10, _15, _5, _5.0, _5, _10.2, _3.1, _10, _5.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3.0 = 4292457593_u32 as i128;
_7.0 = _10.3 | _10.3;
_16 = [_18.2.0];
_17.0 = 1867472989_u32 as f32;
_7 = (_5.3, 8525_u16, 9543318779177381682_u64);
_7.1 = 33546_u16;
RET = !_18.2.0;
_7 = (_11, 61080_u16, 2089778863607479713_u64);
_20.0 = !_8;
_22 = (_5.0, _5);
_20.3 = _5.3;
_20 = (_8, _22.1.1, _16, _3.1);
_9 = _20.0 == _10.0;
_22 = (_6, _20);
_22 = (_5.0, _5);
RET = _18.2.0 | _18.2.0;
Goto(bb9)
}
bb9 = {
Call(_28 = dump_var(11_usize, 15_usize, Move(_15), 8_usize, Move(_8), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_28 = dump_var(11_usize, 14_usize, Move(_14), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: f64,mut _3: (bool, f64, [isize; 1], u8),mut _4: bool,mut _5: (bool, f64, [isize; 1], u8),mut _6: bool,mut _7: (bool, f64, [isize; 1], u8),mut _8: [isize; 1],mut _9: u8,mut _10: (bool, f64, [isize; 1], u8),mut _11: bool) -> (isize,) {
mir! {
type RET = (isize,);
let _12: Adt57;
let _13: Adt45;
let _14: isize;
let _15: f64;
let _16: Adt51;
let _17: [isize; 7];
let _18: [i64; 6];
let _19: ();
let _20: ();
{
RET = ((-9223372036854775808_isize),);
_12.fld3 = 8081026718678683997_i64;
_10.2 = _5.2;
_11 = !_10.0;
_7.2 = [RET.0];
RET = (9223372036854775807_isize,);
_12.fld5 = [_12.fld3,_12.fld3,_12.fld3];
_12.fld4 = 1886747969_u32;
_7.3 = _10.3 - _3.3;
_10.3 = _3.3 + _7.3;
_10.1 = _2 * _3.1;
_10.1 = RET.0 as f64;
_2 = -_7.1;
_7 = _5;
_10.1 = -_2;
Call(_7.3 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _7;
RET.0 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_13 = Adt45::Variant2 { fld0: (-56_i8) };
_12.fld3 = !3252292366967546576_i64;
_14 = _9 as isize;
_2 = _5.1;
RET = (_14,);
_3 = (_4, _7.1, _8, _10.3);
RET.0 = _14;
_7 = (_11, _10.1, _5.2, _10.3);
_16.fld1 = _3.3;
_16.fld4 = 2346_i16;
_5.0 = _11;
_7.3 = _5.3 >> _10.3;
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(12_usize, 14_usize, Move(_14), 4_usize, Move(_4), 11_usize, Move(_11), 20_usize, _20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: bool,mut _2: bool) -> *mut *const i128 {
mir! {
type RET = *mut *const i128;
let _3: u16;
let _4: f64;
let _5: Adt52;
let _6: usize;
let _7: f32;
let _8: char;
let _9: isize;
let _10: char;
let _11: [isize; 7];
let _12: [i64; 3];
let _13: Adt44;
let _14: i128;
let _15: (u8, u16, u64);
let _16: (u8, u16, u64);
let _17: f32;
let _18: [isize; 1];
let _19: u16;
let _20: Adt58;
let _21: f64;
let _22: Adt58;
let _23: i128;
let _24: isize;
let _25: char;
let _26: i16;
let _27: char;
let _28: *const f32;
let _29: f32;
let _30: [isize; 7];
let _31: char;
let _32: u128;
let _33: bool;
let _34: (f32, ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64), (bool, (bool, f64, [isize; 1], u8)), usize, char, (i128, u8, *mut char));
let _35: f64;
let _36: i8;
let _37: u64;
let _38: f32;
let _39: i64;
let _40: ();
let _41: ();
{
_2 = !_1;
_1 = !_2;
_2 = _1;
_1 = _2 <= _2;
_2 = _1;
_2 = _1;
_1 = _2 <= _2;
_2 = _1 & _1;
_2 = _1;
_1 = _2;
_3 = !23702_u16;
_1 = _2;
_2 = _1;
_2 = _1;
_2 = !_1;
Goto(bb1)
}
bb1 = {
_1 = _2;
_3 = 24451_u16 + 6360_u16;
_2 = _1 != _1;
_1 = !_2;
_1 = _2;
_2 = _1;
_1 = !_2;
_1 = _2 & _2;
_1 = !_2;
_4 = (-70_i8) as f64;
_3 = _2 as u16;
_6 = !0_usize;
_1 = !_2;
_3 = 63523_u16 | 19772_u16;
_7 = (-4773766233305370904_i64) as f32;
_1 = _2 ^ _2;
_3 = (-5916289810564016736_i64) as u16;
_7 = 824181264_u32 as f32;
_2 = !_1;
_5 = Adt52::Variant1 { fld0: _2,fld1: 1822656918_i32,fld2: _4,fld3: _6 };
_3 = 21656_u16 * 8241_u16;
_8 = '\u{c7976}';
_6 = _3 as usize;
place!(Field::<i32>(Variant(_5, 1), 1)) = Field::<bool>(Variant(_5, 1), 0) as i32;
_5 = Adt52::Variant1 { fld0: _2,fld1: 1628811979_i32,fld2: _4,fld3: _6 };
_5 = Adt52::Variant1 { fld0: _2,fld1: 841360824_i32,fld2: _4,fld3: _6 };
place!(Field::<i32>(Variant(_5, 1), 1)) = 1037193798_i32 + 1386732579_i32;
Call(place!(Field::<bool>(Variant(_5, 1), 0)) = fn14(_1, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<f64>(Variant(_5, 1), 2)) = _4 * _4;
Goto(bb3)
}
bb3 = {
_4 = _3 as f64;
_9 = 9223372036854775807_isize;
place!(Field::<i32>(Variant(_5, 1), 1)) = 531391289_i32;
_11 = [_9,_9,_9,_9,_9,_9,_9];
_6 = Field::<usize>(Variant(_5, 1), 3);
_12 = [8488481417703664742_i64,(-4928027011630298835_i64),(-4154260250745717576_i64)];
place!(Field::<i32>(Variant(_5, 1), 1)) = -(-1598780628_i32);
SetDiscriminant(_5, 0);
Goto(bb4)
}
bb4 = {
_11 = [_9,_9,_9,_9,_9,_9,_9];
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)).1 = !_3;
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)) = (123_u8, _3, 15111055439283895383_u64);
_10 = _8;
_9 = !(-9223372036854775808_isize);
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)).2 = (-9113411472844269803_i64) as u64;
_6 = 2993229744504362661_usize;
_1 = _2 ^ _2;
_7 = 36692283623163137192270289086115765717_u128 as f32;
place!(Field::<*mut isize>(Variant(_5, 0), 1)) = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_5, 0), 2)));
_10 = _8;
_12 = [(-8613827587414665972_i64),(-4873003398442772335_i64),6444009590495047517_i64];
_7 = 12314_i16 as f32;
_7 = (-1185352657_i32) as f32;
_15.0 = !Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0;
_15 = (Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0, _3, Field::<(u8, u16, u64)>(Variant(_5, 0), 0).2);
place!(Field::<isize>(Variant(_5, 0), 2)) = _9 | _9;
_15.1 = _3;
_16.0 = _10 as u8;
_14 = !(-32748921609743415114865481710176997579_i128);
Call(_7 = core::intrinsics::transmute(_10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10 = _8;
_7 = 970988086_i32 as f32;
_3 = 30563_i16 as u16;
_15.0 = Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0 + Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0;
_10 = _8;
place!(Field::<isize>(Variant(_5, 0), 2)) = _9 << _15.0;
_2 = !_1;
_17 = _7 * _7;
_16.0 = !_15.0;
Call(_22.fld0 = core::intrinsics::bswap(_14), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = _15.0 as f64;
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)).0 = 25363_i16 as u8;
_8 = _10;
_11 = [Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2)];
_15 = (Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0, Field::<(u8, u16, u64)>(Variant(_5, 0), 0).1, Field::<(u8, u16, u64)>(Variant(_5, 0), 0).2);
_15.0 = _14 as u8;
_21 = _4;
SetDiscriminant(_5, 2);
_16.1 = _15.1;
_10 = _8;
place!(Field::<[i64; 3]>(Variant(_5, 2), 0)) = [5334331331884204374_i64,(-4168573251091699666_i64),(-3410348781269024805_i64)];
_15.0 = _16.0 & _16.0;
_19 = _16.1;
_20.fld0 = _14;
SetDiscriminant(_5, 2);
_24 = _9 - _9;
_18 = [_24];
_16.1 = _15.1 | _19;
Goto(bb7)
}
bb7 = {
_16.2 = _15.2;
_10 = _8;
place!(Field::<[i64; 3]>(Variant(_5, 2), 0)) = [(-4399054933523018076_i64),2608700691213275107_i64,2816063467805819543_i64];
_25 = _8;
_15.0 = 2269072022_u32 as u8;
_18 = [_9];
_16.1 = _19;
_23 = _20.fld0;
_15 = (_16.0, _16.1, _16.2);
_18 = [_9];
_22 = Move(_20);
_22.fld0 = _14 >> _16.0;
_7 = _17;
_15.1 = _19;
_31 = _8;
_16.1 = _10 as u16;
_14 = !_22.fld0;
SetDiscriminant(_5, 0);
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)) = _15;
_1 = _2;
_19 = (-29707_i16) as u16;
_7 = _17 + _17;
_2 = !_1;
_15.0 = _16.0 ^ _16.0;
match _6 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb9,
6 => bb10,
2993229744504362661 => bb12,
_ => bb11
}
}
bb8 = {
_4 = _15.0 as f64;
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)).0 = 25363_i16 as u8;
_8 = _10;
_11 = [Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2)];
_15 = (Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0, Field::<(u8, u16, u64)>(Variant(_5, 0), 0).1, Field::<(u8, u16, u64)>(Variant(_5, 0), 0).2);
_15.0 = _14 as u8;
_21 = _4;
SetDiscriminant(_5, 2);
_16.1 = _15.1;
_10 = _8;
place!(Field::<[i64; 3]>(Variant(_5, 2), 0)) = [5334331331884204374_i64,(-4168573251091699666_i64),(-3410348781269024805_i64)];
_15.0 = _16.0 & _16.0;
_19 = _16.1;
_20.fld0 = _14;
SetDiscriminant(_5, 2);
_24 = _9 - _9;
_18 = [_24];
_16.1 = _15.1 | _19;
Goto(bb7)
}
bb9 = {
_10 = _8;
_7 = 970988086_i32 as f32;
_3 = 30563_i16 as u16;
_15.0 = Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0 + Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0;
_10 = _8;
place!(Field::<isize>(Variant(_5, 0), 2)) = _9 << _15.0;
_2 = !_1;
_17 = _7 * _7;
_16.0 = !_15.0;
Call(_22.fld0 = core::intrinsics::bswap(_14), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
place!(Field::<f64>(Variant(_5, 1), 2)) = _4 * _4;
Goto(bb3)
}
bb11 = {
_1 = _2;
_3 = 24451_u16 + 6360_u16;
_2 = _1 != _1;
_1 = !_2;
_1 = _2;
_2 = _1;
_1 = !_2;
_1 = _2 & _2;
_1 = !_2;
_4 = (-70_i8) as f64;
_3 = _2 as u16;
_6 = !0_usize;
_1 = !_2;
_3 = 63523_u16 | 19772_u16;
_7 = (-4773766233305370904_i64) as f32;
_1 = _2 ^ _2;
_3 = (-5916289810564016736_i64) as u16;
_7 = 824181264_u32 as f32;
_2 = !_1;
_5 = Adt52::Variant1 { fld0: _2,fld1: 1822656918_i32,fld2: _4,fld3: _6 };
_3 = 21656_u16 * 8241_u16;
_8 = '\u{c7976}';
_6 = _3 as usize;
place!(Field::<i32>(Variant(_5, 1), 1)) = Field::<bool>(Variant(_5, 1), 0) as i32;
_5 = Adt52::Variant1 { fld0: _2,fld1: 1628811979_i32,fld2: _4,fld3: _6 };
_5 = Adt52::Variant1 { fld0: _2,fld1: 841360824_i32,fld2: _4,fld3: _6 };
place!(Field::<i32>(Variant(_5, 1), 1)) = 1037193798_i32 + 1386732579_i32;
Call(place!(Field::<bool>(Variant(_5, 1), 0)) = fn14(_1, _2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_11 = [_24,_9,_24,_24,_24,_24,_24];
_27 = _25;
_22 = Adt58 { fld0: _23 };
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)).2 = _16.2;
_22.fld0 = _14;
_32 = 101101964528932234948107049696536321346_u128;
_25 = _27;
place!(Field::<isize>(Variant(_5, 0), 2)) = _24;
place!(Field::<*mut isize>(Variant(_5, 0), 1)) = core::ptr::addr_of_mut!(_24);
_34.1.1.1.1.1 = _4;
_9 = _7 as isize;
_11 = [_24,Field::<isize>(Variant(_5, 0), 2),_9,_9,_9,_9,_24];
_22 = Adt58 { fld0: _14 };
_24 = Field::<isize>(Variant(_5, 0), 2) | _9;
_15.0 = Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0;
_34.1.2 = -995491882842747905_i64;
_34.1.1.1.1.1 = _4 + _4;
_34.2.0 = !_2;
_11 = [_9,Field::<isize>(Variant(_5, 0), 2),_24,Field::<isize>(Variant(_5, 0), 2),_24,_9,Field::<isize>(Variant(_5, 0), 2)];
_34.5.2 = core::ptr::addr_of_mut!(_31);
Goto(bb13)
}
bb13 = {
_34.1.0.1 = -_4;
_29 = -_17;
_10 = _8;
_9 = _10 as isize;
_23 = _6 as i128;
Goto(bb14)
}
bb14 = {
_20.fld0 = -_22.fld0;
_14 = 202425992_i32 as i128;
_23 = !_22.fld0;
_13 = Adt44::Variant0 { fld0: _20.fld0,fld1: _15.1,fld2: _9 };
_34.5.2 = core::ptr::addr_of_mut!(_10);
SetDiscriminant(_13, 1);
place!(Field::<(bool, f64, [isize; 1], u8)>(Variant(_13, 1), 0)).1 = -_21;
place!(Field::<u64>(Variant(_13, 1), 2)) = Field::<(u8, u16, u64)>(Variant(_5, 0), 0).2;
_34.1.0.1 = _21 - _34.1.1.1.1.1;
_34.1.1.1.1.2 = _18;
_32 = _34.1.2 as u128;
_34.1.0.0 = Field::<(u8, u16, u64)>(Variant(_5, 0), 0).0 >= _16.0;
_34.4 = _10;
_34.2.1 = (_1, _34.1.0.1, _34.1.1.1.1.2, _15.0);
place!(Field::<[isize; 1]>(Variant(_13, 1), 3)) = [Field::<isize>(Variant(_5, 0), 2)];
place!(Field::<(bool, (bool, f64, [isize; 1], u8))>(Variant(_13, 1), 7)).1.3 = _15.0 ^ _15.0;
_31 = _34.4;
place!(Field::<(u8, u16, u64)>(Variant(_5, 0), 0)).0 = !_15.0;
place!(Field::<isize>(Variant(_5, 0), 2)) = Field::<(bool, f64, [isize; 1], u8)>(Variant(_13, 1), 0).1 as isize;
_34.1.1.0 = _12;
_30 = [_24,Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),_24,Field::<isize>(Variant(_5, 0), 2),_24];
_28 = core::ptr::addr_of!(_34.0);
_4 = _34.2.1.1;
_34.1.2 = !7464528529002239337_i64;
RET = core::ptr::addr_of_mut!(place!(Field::<(*const i128,)>(Variant(_13, 1), 4)).0);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(13_usize, 30_usize, Move(_30), 19_usize, Move(_19), 24_usize, Move(_24), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(13_usize, 3_usize, Move(_3), 8_usize, Move(_8), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(13_usize, 23_usize, Move(_23), 2_usize, Move(_2), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool) -> bool {
mir! {
type RET = bool;
let _5: f32;
let _6: isize;
let _7: ();
let _8: ();
{
_4 = _2;
RET = _3;
_4 = _1 <= _1;
_1 = !_3;
_6 = 9223372036854775807_isize;
_5 = 11268062221005060122_u64 as f32;
_2 = _1 & _1;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(14_usize, 2_usize, Move(_2), 3_usize, Move(_3), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: bool,mut _2: (bool, f64, [isize; 1], u8),mut _3: (bool, f64, [isize; 1], u8),mut _4: (bool, f64, [isize; 1], u8),mut _5: (bool, f64, [isize; 1], u8),mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: [isize; 1],mut _11: bool,mut _12: (bool, f64, [isize; 1], u8),mut _13: bool,mut _14: bool) -> usize {
mir! {
type RET = usize;
let _15: i16;
let _16: isize;
let _17: u32;
let _18: Adt48;
let _19: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize);
let _20: u16;
let _21: *const (*const i128, u8, (isize,), *mut isize);
let _22: char;
let _23: char;
let _24: (bool, (bool, f64, [isize; 1], u8));
let _25: isize;
let _26: i128;
let _27: (f32, ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64), (bool, (bool, f64, [isize; 1], u8)), usize, char, (i128, u8, *mut char));
let _28: Adt52;
let _29: f32;
let _30: Adt55;
let _31: [isize; 1];
let _32: (isize,);
let _33: i8;
let _34: (isize,);
let _35: isize;
let _36: (bool, (bool, f64, [isize; 1], u8));
let _37: i16;
let _38: ();
let _39: ();
{
_2 = (_1, _12.1, _5.2, _4.3);
_10 = [(-9223372036854775808_isize)];
_5.3 = _3.3;
_12.0 = _9;
_12.2 = [(-59_isize)];
_3.0 = _13;
_3.0 = _8 ^ _2.0;
_3.3 = (-9223372036854775808_isize) as u8;
_2.1 = _5.1 - _3.1;
_4.0 = !_5.0;
_13 = _12.0 >= _7;
_17 = 107_i8 as u32;
_2.0 = !_11;
_17 = !1583606663_u32;
_3.1 = -_12.1;
_3 = (_11, _2.1, _4.2, _4.3);
Goto(bb1)
}
bb1 = {
_11 = !_7;
Goto(bb2)
}
bb2 = {
_4.0 = _6 & _1;
_20 = 44180_u16 + 21209_u16;
_4 = (_9, _2.1, _5.2, _12.3);
_5.3 = 6_usize as u8;
_3 = _5;
_17 = !778169940_u32;
_5 = (_11, _4.1, _2.2, _12.3);
_3.0 = _6;
_4.1 = _12.1 * _2.1;
_12.0 = !_5.0;
_24.1.2 = [24_isize];
_24.1.2 = _10;
_24.1.1 = _12.1;
_2.2 = [(-9223372036854775808_isize)];
_5.3 = _12.3;
_2.1 = _4.1 + _12.1;
_15 = -(-6107_i16);
_3.2 = [(-11_isize)];
_24.1.2 = [(-32_isize)];
_24.1 = (_13, _2.1, _4.2, _5.3);
_16 = 109_isize;
RET = _16 as usize;
_20 = !25376_u16;
_24.1.2 = [_16];
_6 = _7 ^ _11;
_27.1.1.1.1.1 = _17 as f64;
match _16 {
0 => bb3,
1 => bb4,
109 => bb6,
_ => bb5
}
}
bb3 = {
_11 = !_7;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_18.fld0.0 = core::ptr::addr_of!(_27.5.0);
_27.1.1.6 = RET * RET;
_12.0 = !_1;
_27.1.0.1 = _2.1 + _2.1;
_5.3 = _24.1.3;
_24 = (_14, _2);
_4.3 = _2.3 & _24.1.3;
_27.1.1.1.1.1 = -_2.1;
_27.1.1.2 = !_27.1.1.6;
_26 = -16957502540503249718718486050333841067_i128;
_24.0 = _9;
_27.5.2 = core::ptr::addr_of_mut!(_22);
_27.4 = '\u{3999b}';
_27.2.0 = _24.1.0;
_4.1 = _2.1;
match _16 {
0 => bb1,
109 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_23 = _27.4;
_27.0 = _27.1.1.6 as f32;
_27.4 = _23;
_23 = _27.4;
RET = _27.1.1.6 * _27.1.1.2;
_24.1.0 = _27.2.0;
_27.1.1.1.1.2 = _2.2;
_27.1.1.4 = 1572893968_i32 << _5.3;
_27.2.1.3 = _12.3 * _4.3;
_12.2 = [_16];
match _16 {
0 => bb4,
1 => bb6,
2 => bb9,
3 => bb10,
109 => bb12,
_ => bb11
}
}
bb9 = {
Return()
}
bb10 = {
_18.fld0.0 = core::ptr::addr_of!(_27.5.0);
_27.1.1.6 = RET * RET;
_12.0 = !_1;
_27.1.0.1 = _2.1 + _2.1;
_5.3 = _24.1.3;
_24 = (_14, _2);
_4.3 = _2.3 & _24.1.3;
_27.1.1.1.1.1 = -_2.1;
_27.1.1.2 = !_27.1.1.6;
_26 = -16957502540503249718718486050333841067_i128;
_24.0 = _9;
_27.5.2 = core::ptr::addr_of_mut!(_22);
_27.4 = '\u{3999b}';
_27.2.0 = _24.1.0;
_4.1 = _2.1;
match _16 {
0 => bb1,
109 => bb8,
_ => bb7
}
}
bb11 = {
Return()
}
bb12 = {
_4.2 = [_16];
_24.1.2 = [_16];
_27.1.1.2 = !RET;
_27.1.1.1.0 = _9;
_4.0 = _8 < _8;
_16 = (-62_isize) | 123_isize;
RET = 4_i8 as usize;
RET = !_27.1.1.6;
_27.1.0.3 = !_2.3;
_27.2.1 = _4;
_24.1.3 = !_27.2.1.3;
_27.1.0 = (_12.0, _27.2.1.1, _10, _4.3);
_1 = !_27.2.1.0;
_12.0 = _2.0;
_24.1.1 = -_12.1;
_27.5.2 = core::ptr::addr_of_mut!(_22);
_27.1.0.0 = _27.1.1.1.1.1 < _27.1.0.1;
_27.3 = RET * RET;
_27.5.0 = _26 ^ _26;
_4.1 = -_2.1;
_27.2.1.1 = -_12.1;
_27.1.1.0 = [(-6978551290089692836_i64),(-108824986082730283_i64),3392387610871194792_i64];
_27.1.1.1.1.3 = !_4.3;
_32 = (_16,);
_27.1.1.1 = (_24.1.0, _2);
_27.0 = _32.0 as f32;
_31 = [_16];
_26 = _32.0 as i128;
Goto(bb13)
}
bb13 = {
_2.1 = -_4.1;
_2.2 = [_16];
_19 = core::ptr::addr_of_mut!(_27.1.1);
_24.1 = (_3.0, (*_19).1.1.1, _3.2, _27.1.1.1.1.3);
_32 = (_16,);
_28 = Adt52::Variant2 { fld0: (*_19).0 };
(*_19).1.1.2 = [_16];
_24.0 = !_27.2.1.0;
_34 = _32;
_31 = _5.2;
(*_19).0 = [(-2026686277186452420_i64),2639028508716546983_i64,(-3158365600576610720_i64)];
_27.1.1.2 = _27.1.1.4 as usize;
_27.5.1 = (*_19).1.1.1 as u8;
_36.1.3 = _2.3;
(*_19).6 = _26 as usize;
Call(_35 = core::intrinsics::bswap(_32.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_4.0 = !_2.0;
RET = _27.1.1.2 + (*_19).6;
_36.1.0 = _12.0;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(15_usize, 16_usize, Move(_16), 10_usize, Move(_10), 11_usize, Move(_11), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(15_usize, 13_usize, Move(_13), 32_usize, Move(_32), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(15_usize, 20_usize, Move(_20), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: i8,mut _2: bool,mut _3: isize,mut _4: isize,mut _5: bool,mut _6: i64) -> char {
mir! {
type RET = char;
let _7: f64;
let _8: u32;
let _9: [isize; 1];
let _10: (f32,);
let _11: [i64; 6];
let _12: f64;
let _13: [i64; 6];
let _14: *mut (*const i128,);
let _15: isize;
let _16: f64;
let _17: isize;
let _18: char;
let _19: isize;
let _20: *mut *const i128;
let _21: isize;
let _22: ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64);
let _23: i32;
let _24: isize;
let _25: i8;
let _26: (bool, (bool, f64, [isize; 1], u8));
let _27: f32;
let _28: [i64; 3];
let _29: usize;
let _30: f64;
let _31: *const *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize);
let _32: ();
let _33: ();
{
_3 = (-1422108664_i32) as isize;
_1 = !8_i8;
RET = '\u{febfe}';
_3 = _4;
_3 = _4;
_2 = _5;
RET = '\u{e8cfb}';
_7 = _6 as f64;
_3 = _4;
_5 = !_2;
_7 = 88904904096996044775440809126904163612_i128 as f64;
_6 = -4726302225540841001_i64;
Goto(bb1)
}
bb1 = {
_2 = !_5;
_2 = _5;
_2 = _6 <= _6;
_2 = _5 & _5;
_2 = !_5;
_5 = _3 != _3;
_3 = _4 >> _4;
_5 = !_2;
_6 = 2615744188053611943_i64;
RET = '\u{6cf48}';
_5 = _2 | _2;
_3 = _4 & _4;
_5 = _2;
_9 = [_4];
_6 = -785892269453337897_i64;
RET = '\u{e0fb7}';
_3 = _4;
_7 = 12049_i16 as f64;
_9 = [_3];
Goto(bb2)
}
bb2 = {
_8 = 2374419682_u32;
_9 = [_4];
RET = '\u{49b00}';
_9 = [_4];
_1 = !(-89_i8);
_10.0 = (-135116102573749982802435990033716270989_i128) as f32;
RET = '\u{6ddf6}';
_3 = 149_u8 as isize;
_9 = [_4];
_5 = _3 == _4;
_9 = [_4];
_5 = _2;
_4 = -_3;
_11 = [_6,_6,_6,_6,_6,_6];
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = [_6,_6,_6,_6,_6,_6];
_8 = 1421861190_u32;
_3 = _4 - _4;
_1 = !51_i8;
_13 = [_6,_6,_6,_6,_6,_6];
_9 = [_3];
_15 = -_4;
_3 = 487753852_i32 as isize;
_15 = _3;
_4 = _15;
_2 = !_5;
_9 = [_15];
_15 = _4;
_3 = 10428717315270147563_u64 as isize;
_10.0 = (-43530089791158560421758150276603086292_i128) as f32;
_6 = RET as i64;
_12 = _7 * _7;
_4 = !_15;
Goto(bb4)
}
bb4 = {
_12 = _4 as f64;
_9 = [_4];
_2 = !_5;
RET = '\u{8bd53}';
RET = '\u{1e454}';
_2 = !_5;
_10.0 = 152_u8 as f32;
_4 = _6 as isize;
_16 = _12 + _12;
_13 = [_6,_6,_6,_6,_6,_6];
_15 = -_3;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
1421861190 => bb6,
_ => bb5
}
}
bb5 = {
_11 = [_6,_6,_6,_6,_6,_6];
_8 = 1421861190_u32;
_3 = _4 - _4;
_1 = !51_i8;
_13 = [_6,_6,_6,_6,_6,_6];
_9 = [_3];
_15 = -_4;
_3 = 487753852_i32 as isize;
_15 = _3;
_4 = _15;
_2 = !_5;
_9 = [_15];
_15 = _4;
_3 = 10428717315270147563_u64 as isize;
_10.0 = (-43530089791158560421758150276603086292_i128) as f32;
_6 = RET as i64;
_12 = _7 * _7;
_4 = !_15;
Goto(bb4)
}
bb6 = {
_5 = _2;
_16 = -_7;
_15 = -_3;
_1 = 124_i8 << _15;
_3 = _10.0 as isize;
_18 = RET;
_15 = _3 & _3;
_13 = [_6,_6,_6,_6,_6,_6];
_17 = 84_u8 as isize;
_3 = 5903512_i32 as isize;
_10.0 = 30297_u16 as f32;
_12 = _7;
_8 = _1 as u32;
_22.2 = _6 | _6;
_22.0.3 = !236_u8;
_18 = RET;
_22.1.2 = 0_usize;
_17 = _4 * _15;
_22.0.1 = -_12;
_9 = [_15];
_22.1.1.1.2 = _9;
Goto(bb7)
}
bb7 = {
_13 = _11;
_22.0 = (_2, _16, _22.1.1.1.2, 167_u8);
_16 = _22.1.2 as f64;
_22.1.1.1 = (_2, _22.0.1, _22.0.2, _22.0.3);
_15 = _17 ^ _3;
_13 = [_22.2,_6,_22.2,_22.2,_22.2,_22.2];
_22.1.1.1.3 = _22.0.3 * _22.0.3;
_11 = [_22.2,_22.2,_6,_6,_22.2,_6];
_22.1.6 = _22.1.2 << _22.0.3;
_8 = 1904467971_u32 * 1727819441_u32;
_5 = !_2;
_1 = 42_i8;
_11 = _13;
_22.2 = _6;
_21 = _15;
_23 = _8 as i32;
_10.0 = 28457_i16 as f32;
_22.1.0 = [_22.2,_6,_6];
_15 = _3 + _21;
_19 = _23 as isize;
_22.0.2 = [_3];
_22.1.1 = (_5, _22.0);
Goto(bb8)
}
bb8 = {
RET = _18;
_4 = _21;
_5 = _22.1.1.1.0 >= _22.1.1.1.0;
_16 = _12 - _22.0.1;
_7 = -_22.1.1.1.1;
_7 = _16;
_13 = _11;
_22.1.4 = !_23;
_10.0 = _22.2 as f32;
_4 = _22.0.1 as isize;
_23 = _22.1.4;
_22.1.4 = _23;
_11 = [_22.2,_22.2,_6,_22.2,_22.2,_22.2];
_25 = 63818_u16 as i8;
_24 = 49364_u16 as isize;
_18 = RET;
_22.0 = (_22.1.1.0, _22.1.1.1.1, _9, _22.1.1.1.3);
_22.0.2 = _9;
_22.1.1.1 = (_22.0.0, _7, _9, _22.0.3);
_4 = !_15;
_22.1.1.1.1 = _7;
_22.0.2 = [_15];
_22.1.1.1.3 = 46295_u16 as u8;
_22.1.1.1.3 = !_22.0.3;
_21 = -_17;
_7 = _16;
_12 = _7;
RET = _18;
match _22.0.3 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
167 => bb17,
_ => bb16
}
}
bb9 = {
_13 = _11;
_22.0 = (_2, _16, _22.1.1.1.2, 167_u8);
_16 = _22.1.2 as f64;
_22.1.1.1 = (_2, _22.0.1, _22.0.2, _22.0.3);
_15 = _17 ^ _3;
_13 = [_22.2,_6,_22.2,_22.2,_22.2,_22.2];
_22.1.1.1.3 = _22.0.3 * _22.0.3;
_11 = [_22.2,_22.2,_6,_6,_22.2,_6];
_22.1.6 = _22.1.2 << _22.0.3;
_8 = 1904467971_u32 * 1727819441_u32;
_5 = !_2;
_1 = 42_i8;
_11 = _13;
_22.2 = _6;
_21 = _15;
_23 = _8 as i32;
_10.0 = 28457_i16 as f32;
_22.1.0 = [_22.2,_6,_6];
_15 = _3 + _21;
_19 = _23 as isize;
_22.0.2 = [_3];
_22.1.1 = (_5, _22.0);
Goto(bb8)
}
bb10 = {
_5 = _2;
_16 = -_7;
_15 = -_3;
_1 = 124_i8 << _15;
_3 = _10.0 as isize;
_18 = RET;
_15 = _3 & _3;
_13 = [_6,_6,_6,_6,_6,_6];
_17 = 84_u8 as isize;
_3 = 5903512_i32 as isize;
_10.0 = 30297_u16 as f32;
_12 = _7;
_8 = _1 as u32;
_22.2 = _6 | _6;
_22.0.3 = !236_u8;
_18 = RET;
_22.1.2 = 0_usize;
_17 = _4 * _15;
_22.0.1 = -_12;
_9 = [_15];
_22.1.1.1.2 = _9;
Goto(bb7)
}
bb11 = {
_11 = [_6,_6,_6,_6,_6,_6];
_8 = 1421861190_u32;
_3 = _4 - _4;
_1 = !51_i8;
_13 = [_6,_6,_6,_6,_6,_6];
_9 = [_3];
_15 = -_4;
_3 = 487753852_i32 as isize;
_15 = _3;
_4 = _15;
_2 = !_5;
_9 = [_15];
_15 = _4;
_3 = 10428717315270147563_u64 as isize;
_10.0 = (-43530089791158560421758150276603086292_i128) as f32;
_6 = RET as i64;
_12 = _7 * _7;
_4 = !_15;
Goto(bb4)
}
bb12 = {
_12 = _4 as f64;
_9 = [_4];
_2 = !_5;
RET = '\u{8bd53}';
RET = '\u{1e454}';
_2 = !_5;
_10.0 = 152_u8 as f32;
_4 = _6 as isize;
_16 = _12 + _12;
_13 = [_6,_6,_6,_6,_6,_6];
_15 = -_3;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
1421861190 => bb6,
_ => bb5
}
}
bb13 = {
_11 = [_6,_6,_6,_6,_6,_6];
_8 = 1421861190_u32;
_3 = _4 - _4;
_1 = !51_i8;
_13 = [_6,_6,_6,_6,_6,_6];
_9 = [_3];
_15 = -_4;
_3 = 487753852_i32 as isize;
_15 = _3;
_4 = _15;
_2 = !_5;
_9 = [_15];
_15 = _4;
_3 = 10428717315270147563_u64 as isize;
_10.0 = (-43530089791158560421758150276603086292_i128) as f32;
_6 = RET as i64;
_12 = _7 * _7;
_4 = !_15;
Goto(bb4)
}
bb14 = {
_8 = 2374419682_u32;
_9 = [_4];
RET = '\u{49b00}';
_9 = [_4];
_1 = !(-89_i8);
_10.0 = (-135116102573749982802435990033716270989_i128) as f32;
RET = '\u{6ddf6}';
_3 = 149_u8 as isize;
_9 = [_4];
_5 = _3 == _4;
_9 = [_4];
_5 = _2;
_4 = -_3;
_11 = [_6,_6,_6,_6,_6,_6];
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_2 = !_5;
_2 = _5;
_2 = _6 <= _6;
_2 = _5 & _5;
_2 = !_5;
_5 = _3 != _3;
_3 = _4 >> _4;
_5 = !_2;
_6 = 2615744188053611943_i64;
RET = '\u{6cf48}';
_5 = _2 | _2;
_3 = _4 & _4;
_5 = _2;
_9 = [_4];
_6 = -785892269453337897_i64;
RET = '\u{e0fb7}';
_3 = _4;
_7 = 12049_i16 as f64;
_9 = [_3];
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_26.0 = _22.0.0;
_9 = [_17];
_22.0.1 = -_7;
_11 = [_22.2,_22.2,_22.2,_22.2,_22.2,_6];
RET = _18;
_26.1.3 = !_22.0.3;
_12 = _22.1.1.1.1;
_22.1.1.1.0 = !_2;
_26.1.2 = [_15];
_22.0.2 = [_4];
_25 = -_1;
_10.0 = _8 as f32;
Goto(bb18)
}
bb18 = {
Call(_32 = dump_var(16_usize, 2_usize, Move(_2), 3_usize, Move(_3), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_32 = dump_var(16_usize, 13_usize, Move(_13), 21_usize, Move(_21), 11_usize, Move(_11), 25_usize, Move(_25)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(16_usize, 24_usize, Move(_24), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{7eae7}'), std::hint::black_box(5825953844124205046_usize), std::hint::black_box(76_i8));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: i128,
fld1: u16,
fld2: isize,

},
Variant1{
fld0: (bool, f64, [isize; 1], u8),
fld1: *mut char,
fld2: u64,
fld3: [isize; 1],
fld4: (*const i128,),
fld5: *mut (*const i128,),
fld6: *const (*const i128, u8, (isize,), *mut isize),
fld7: (bool, (bool, f64, [isize; 1], u8)),

},
Variant2{
fld0: u16,
fld1: *mut isize,
fld2: f64,
fld3: i128,

},
Variant3{
fld0: f32,
fld1: [i64; 3],
fld2: u64,
fld3: u8,
fld4: (f32,),
fld5: i128,
fld6: (bool, (bool, f64, [isize; 1], u8)),

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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut isize,
fld1: *mut char,
fld2: i16,
fld3: *const (*const i128, u8, (isize,), *mut isize),

},
Variant1{
fld0: ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64),
fld1: Adt44,
fld2: u128,

},
Variant2{
fld0: i8,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: [i64; 6],
fld1: *mut isize,
fld2: u16,
fld3: *mut *const i128,
fld4: (*const i128, u8, (isize,), *mut isize),
fld5: (u8, u16, u64),
fld6: *const (*const i128, u8, (isize,), *mut isize),
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u8,
fld1: *const i128,
fld2: *mut char,
fld3: *mut isize,

},
Variant1{
fld0: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld1: (f32, ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64), (bool, (bool, f64, [isize; 1], u8)), usize, char, (i128, u8, *mut char)),
fld2: f64,
fld3: *mut isize,
fld4: *mut (*const i128,),
fld5: ([i64; 3], (i128, u8, *mut char), *mut char),
fld6: i64,

},
Variant2{
fld0: usize,
fld1: [isize; 1],
fld2: isize,
fld3: [isize; 7],
fld4: (bool, (bool, f64, [isize; 1], u8)),
fld5: f32,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: (*const i128,),
}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u16,
fld1: Adt44,
fld2: *mut *const i128,
fld3: (*const i128, u8, (isize,), *mut isize),

},
Variant1{
fld0: ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64),
fld1: char,
fld2: (isize,),
fld3: i8,
fld4: i16,
fld5: (*const i128,),
fld6: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld7: Adt45,

},
Variant2{
fld0: ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld1: Adt45,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
fld0: f64,
fld1: [i64; 3],
fld2: (bool, (bool, f64, [isize; 1], u8)),
fld3: *const *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld4: u64,
fld5: *mut isize,
fld6: Adt44,

},
Variant1{
fld0: *mut char,
fld1: char,
fld2: *mut *const i128,
fld3: (bool, (bool, f64, [isize; 1], u8)),
fld4: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld5: Adt49,

},
Variant2{
fld0: *const f32,
fld1: (*const i128,),
fld2: [i64; 3],
fld3: f64,
fld4: Adt48,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [isize; 1],
fld1: u8,
fld2: u16,
fld3: *mut char,
fld4: i16,
fld5: Adt44,
}
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
fld0: (u8, u16, u64),
fld1: *mut isize,
fld2: isize,

},
Variant1{
fld0: bool,
fld1: i32,
fld2: f64,
fld3: usize,

},
Variant2{
fld0: [i64; 3],

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt52,
fld1: u32,
fld2: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld3: f32,
fld4: (*const i128, u8, (isize,), *mut isize),
fld5: [i64; 3],
fld6: *const i128,
fld7: ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),

},
Variant1{
fld0: Adt44,
fld1: Adt51,
fld2: i64,
fld3: i128,
fld4: i16,
fld5: u32,

},
Variant2{
fld0: u8,
fld1: i8,

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
fld0: Adt52,
fld1: i64,
fld2: Adt51,
fld3: u32,
fld4: u64,

},
Variant1{
fld0: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld1: (u8, u16, u64),
fld2: u128,
fld3: i8,
fld4: u8,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *const i128,
fld1: ([i64; 3], (i128, u8, *mut char), *mut char),
fld2: i128,
fld3: [i64; 6],
fld4: *mut (*const i128,),

},
Variant1{
fld0: ([i64; 3], (i128, u8, *mut char), *mut char),
fld1: Adt51,
fld2: [isize; 7],
fld3: (bool, f64, [isize; 1], u8),
fld4: i16,
fld5: *mut isize,
fld6: i64,
fld7: (bool, (bool, f64, [isize; 1], u8)),

},
Variant2{
fld0: bool,
fld1: ([i64; 3], (i128, u8, *mut char), *mut char),
fld2: (*const i128, u8, (isize,), *mut isize),
fld3: usize,
fld4: u128,
fld5: Adt44,
fld6: [i64; 6],
fld7: i128,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
fld0: Adt49,
fld1: (*const i128,),
fld2: [isize; 1],
fld3: Adt45,
fld4: *mut (*const i128,),
fld5: Adt47,
fld6: f64,
fld7: i128,

},
Variant1{
fld0: i32,
fld1: *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
fld2: Adt54,

},
Variant2{
fld0: bool,
fld1: char,
fld2: ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64),
fld3: (f32,),
fld4: i16,
fld5: (f32, ((bool, f64, [isize; 1], u8), ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize), i64), (bool, (bool, f64, [isize; 1], u8)), usize, char, (i128, u8, *mut char)),
fld6: [usize; 4],
fld7: (*const i128,),

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: i128,
fld1: *mut *const i128,
fld2: *const (*const i128, u8, (isize,), *mut isize),
fld3: i64,
fld4: u32,
fld5: [i64; 3],
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: i128,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: *const f32,
fld1: (i128, u8, *mut char),
fld2: Adt56,
fld3: Adt57,
fld4: *const *mut ([i64; 3], (bool, (bool, f64, [isize; 1], u8)), usize, *const (*const i128, u8, (isize,), *mut isize), i32, f64, usize),
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt57,

},
Variant1{
fld0: *mut (*const i128,),

}}

