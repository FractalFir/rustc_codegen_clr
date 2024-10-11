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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u64,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32) -> bool {
mir! {
type RET = bool;
let _13: isize;
let _14: *mut u16;
let _15: f64;
let _16: f64;
let _17: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16));
let _18: [i128; 4];
let _19: Adt54;
let _20: [i128; 3];
let _21: i64;
let _22: char;
let _23: f64;
let _24: isize;
let _25: ();
let _26: ();
{
_6 = !2352219500213205156_u64;
RET = !false;
_2 = '\u{393a0}';
_1 = RET != RET;
_9 = !5448848979714852013_usize;
_8 = (-36434609279792475479757884400190728076_i128) | 136161623310708503692977958509548587567_i128;
_4 = 9223372036854775807_isize as i8;
_14 = core::ptr::addr_of_mut!(_11);
RET = _6 >= _6;
_7 = (-4376888395545644751_i64);
_12 = 2479721209_u32;
_6 = 39_u8 as u64;
_7 = (-668016939_i32) as i64;
Call(_2 = fn1(_14, _14, _1, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_14) = _12 as u16;
_9 = !3_usize;
_9 = 6893583730113241253_usize + 5_usize;
_7 = (-8173578933940372267_i64) + 7268254949568782913_i64;
_13 = 100_isize << _9;
_10 = 172_u8;
Goto(bb2)
}
bb2 = {
RET = _1;
_1 = RET | RET;
_6 = _13 as u64;
_9 = !3_usize;
_5 = 17750_i16;
_9 = 4_usize * 0_usize;
RET = (*_14) == (*_14);
_6 = !7818695252062175121_u64;
(*_14) = !42308_u16;
_4 = 97_i8 + (-112_i8);
_14 = core::ptr::addr_of_mut!((*_14));
_3 = _13;
_9 = _13 as usize;
_4 = (-74_i8) + (-102_i8);
Goto(bb3)
}
bb3 = {
_16 = (*_14) as f64;
_6 = 962871753_i32 as u64;
_12 = !3189408835_u32;
_16 = _7 as f64;
_15 = _16;
(*_14) = 1333_u16 & 33937_u16;
_11 = 3108_u16 ^ 37819_u16;
_14 = core::ptr::addr_of_mut!((*_14));
RET = !_1;
_10 = !123_u8;
_9 = _2 as usize;
_3 = _13;
_7 = (*_14) as i64;
RET = _1;
_13 = _3;
_10 = _6 as u8;
(*_14) = 31089_u16;
_13 = _3;
Goto(bb4)
}
bb4 = {
RET = !_1;
RET = !_1;
_11 = 34753_u16;
Goto(bb5)
}
bb5 = {
_17.1.0 = _15 - _15;
Goto(bb6)
}
bb6 = {
RET = !_1;
_17.2 = [_5,_5,_5,_5,_5];
_6 = 7737165786716156089_u64;
_17.4.1 = core::ptr::addr_of!(_17.1.1.1);
_6 = 16614730858701516582_u64 | 10606616651873265241_u64;
_17.3 = 2060566239_i32 - (-1936374056_i32);
_1 = !RET;
_8 = !39795853964014037675044763871043092007_i128;
_17.0 = _1 | _1;
_14 = core::ptr::addr_of_mut!((*_14));
_17.3 = _10 as i32;
Goto(bb7)
}
bb7 = {
_14 = core::ptr::addr_of_mut!((*_14));
_18 = [_8,_8,_8,_8];
_3 = -_13;
_17.1.1.0 = -_8;
RET = _17.0 ^ _17.0;
_9 = 2_usize;
_17.4.0 = _12;
_13 = _3 >> _18[_9];
_16 = -_17.1.0;
_17.1.1 = (_18[_9], _17.3);
Goto(bb8)
}
bb8 = {
Call(_25 = dump_var(0_usize, 5_usize, Move(_5), 6_usize, Move(_6), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_25 = dump_var(0_usize, 8_usize, Move(_8), 12_usize, Move(_12), 18_usize, Move(_18), 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: *mut u16,mut _2: *mut u16,mut _3: bool,mut _4: u32) -> char {
mir! {
type RET = char;
let _5: Adt59;
let _6: u8;
let _7: ((i128, i32), [i128; 3], i8);
let _8: i128;
let _9: usize;
let _10: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _11: bool;
let _12: (i128, i32);
let _13: isize;
let _14: Adt51;
let _15: Adt44;
let _16: [i128; 3];
let _17: f32;
let _18: (i128, i32);
let _19: (*mut &'static f64, u64, *mut *mut &'static f64);
let _20: (i128, i32);
let _21: isize;
let _22: isize;
let _23: u8;
let _24: *const u8;
let _25: f64;
let _26: ();
let _27: ();
{
_2 = _1;
(*_2) = !62255_u16;
_3 = !false;
(*_1) = 2129_u16 & 16072_u16;
_5.fld2.1.2 = 242169868129037976_u64 as i128;
_5.fld2.1.1.0 = !_5.fld2.1.2;
_5.fld1 = !142_u8;
_6 = _3 as u8;
(*_1) = !16409_u16;
_5.fld2.3 = (-945193461_i32) & 1910035133_i32;
(*_2) = 50068_u16;
(*_1) = !21986_u16;
_5.fld2.4.2 = core::ptr::addr_of!((*_1));
_7.1 = [_5.fld2.1.1.0,_5.fld2.1.1.0,_5.fld2.1.2];
_7.0.0 = _5.fld2.1.1.0;
_7.0.1 = _5.fld2.3;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2479721209 => bb7,
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
_5.fld2.2 = [26128_i16,18818_i16,15783_i16,6386_i16,21280_i16];
_7.0.0 = !_5.fld2.1.1.0;
(*_2) = (-9223372036854775808_isize) as u16;
_7.2 = 9004_i16 as i8;
_10.1 = (_7.0, _7.1, _7.2);
_5.fld0 = _3 ^ _3;
_10.4 = _4 as f64;
RET = '\u{101598}';
_5.fld2.1.1.1 = _10.1.0.1 - _10.1.0.1;
_10.1.0.0 = _5.fld2.1.1.0;
_7.0.1 = _5.fld2.3;
_10.1.0.1 = (*_1) as i32;
_7.0.0 = _5.fld2.1.1.0;
_9 = 0_usize >> (*_1);
_9 = 14713279789892659265_usize;
_10.1.0.0 = _5.fld2.1.1.0 ^ _7.0.0;
_10.2 = _5.fld2.3 as u128;
_5.fld2.1.1.1 = _10.1.0.0 as i32;
_10.1.2 = _7.2 + _7.2;
_5.fld2.1.0 = _10.4;
RET = '\u{27a9}';
RET = '\u{6fa42}';
RET = '\u{ec6a1}';
_10.1.0.1 = _5.fld2.3;
_5.fld2.1.1.0 = !_10.1.0.0;
_12.0 = _5.fld2.1.1.0 | _10.1.0.0;
_13 = (-9223372036854775808_isize);
Call(_2 = fn2(_10.1.0, _1, _5.fld2.1.0, _5.fld2.2, _10.1, _5.fld2.1.1.1, _7.0.1, _7.2, RET, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_1) = !43276_u16;
_5.fld2.1.1.1 = 25510_i16 as i32;
(*_2) = 1003_u16;
_10.1 = (_7.0, _7.1, _7.2);
Goto(bb9)
}
bb9 = {
(*_1) = 40770_u16 + 26170_u16;
_5.fld2.1.1 = (_12.0, _5.fld2.3);
_8 = 12591187425663469475_u64 as i128;
_7.2 = !_10.1.2;
_15.fld0.0 = core::ptr::addr_of_mut!(_16);
_5.fld1 = _6;
_10 = (_9, _7, 326232445396820111251502284329436476648_u128, 284499818191891762013347145132846715262_u128, _5.fld2.1.0);
_5.fld2.4.0 = _6 as u32;
_15.fld1 = core::ptr::addr_of_mut!(_7.1);
_5.fld0 = _10.3 >= _10.3;
_5.fld2.1.1.0 = _12.0;
_7.0.0 = _5.fld2.1.1.0;
_7.1 = [_8,_5.fld2.1.2,_7.0.0];
(*_1) = !54332_u16;
_5.fld2.1.2 = _10.1.0.0 + _7.0.0;
_10.3 = _5.fld2.4.0 as u128;
_15.fld1 = core::ptr::addr_of_mut!(_7.1);
Goto(bb10)
}
bb10 = {
_11 = !_3;
_13 = 29_isize;
_5.fld2.1.1 = _7.0;
_2 = _1;
_10.1.0.1 = _7.0.1 | _5.fld2.3;
_18.1 = _10.1.0.1;
_15.fld7 = core::ptr::addr_of!(_10.1.0.1);
_5.fld2.1.1.0 = -_10.1.0.0;
_10.4 = _5.fld2.1.0 - _5.fld2.1.0;
RET = '\u{3a7c}';
_7.0.1 = !_10.1.0.1;
_15.fld6 = _18.1 as i64;
_10.3 = _10.2 * _10.2;
_10.2 = _10.3;
_5.fld1 = _6 - _6;
_5.fld2.1.1.0 = _7.0.0 << _6;
(*_2) = _5.fld0 as u16;
_5.fld0 = !_3;
_10.1.0.0 = -_5.fld2.1.2;
_7.1 = [_5.fld2.1.1.0,_10.1.0.0,_5.fld2.1.1.0];
_5.fld2.1.1.0 = -_12.0;
Call(_7.1 = core::intrinsics::transmute(_10.1.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = Adt51::Variant2 { fld0: _5.fld2.1 };
_10.1.0 = (Field::<(f64, (i128, i32), i128)>(Variant(_14, 2), 0).1.0, _18.1);
_10.1.0.0 = _5.fld2.1.1.0 | _5.fld2.1.1.0;
_20.1 = !_7.0.1;
_9 = _5.fld2.4.0 as usize;
_16 = _7.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_14, 2), 0)).1.0 = _10.1.0.0;
match _13 {
0 => bb12,
29 => bb14,
_ => bb13
}
}
bb12 = {
_11 = !_3;
_13 = 29_isize;
_5.fld2.1.1 = _7.0;
_2 = _1;
_10.1.0.1 = _7.0.1 | _5.fld2.3;
_18.1 = _10.1.0.1;
_15.fld7 = core::ptr::addr_of!(_10.1.0.1);
_5.fld2.1.1.0 = -_10.1.0.0;
_10.4 = _5.fld2.1.0 - _5.fld2.1.0;
RET = '\u{3a7c}';
_7.0.1 = !_10.1.0.1;
_15.fld6 = _18.1 as i64;
_10.3 = _10.2 * _10.2;
_10.2 = _10.3;
_5.fld1 = _6 - _6;
_5.fld2.1.1.0 = _7.0.0 << _6;
(*_2) = _5.fld0 as u16;
_5.fld0 = !_3;
_10.1.0.0 = -_5.fld2.1.2;
_7.1 = [_5.fld2.1.1.0,_10.1.0.0,_5.fld2.1.1.0];
_5.fld2.1.1.0 = -_12.0;
Call(_7.1 = core::intrinsics::transmute(_10.1.1), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
_5.fld2.4.2 = core::ptr::addr_of!((*_2));
_5.fld2.4.1 = core::ptr::addr_of!(_10.1.0.1);
_7.0.0 = -Field::<(f64, (i128, i32), i128)>(Variant(_14, 2), 0).1.0;
_15.fld1 = core::ptr::addr_of_mut!(_10.1.1);
_5.fld1 = _6;
_2 = core::ptr::addr_of_mut!((*_2));
_9 = _10.0;
_5.fld2.1.2 = RET as i128;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_14, 2), 0)).2 = Field::<(f64, (i128, i32), i128)>(Variant(_14, 2), 0).1.0 >> _7.0.1;
_1 = _2;
_15.fld1 = core::ptr::addr_of_mut!(_10.1.1);
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(1_usize, 13_usize, Move(_13), 11_usize, Move(_11), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: (i128, i32),mut _2: *mut u16,mut _3: f64,mut _4: [i16; 5],mut _5: ((i128, i32), [i128; 3], i8),mut _6: i32,mut _7: i32,mut _8: i8,mut _9: char,mut _10: *mut u16) -> *mut u16 {
mir! {
type RET = *mut u16;
let _11: Adt46;
let _12: i64;
let _13: u128;
let _14: usize;
let _15: (isize, bool, u32);
let _16: f32;
let _17: bool;
let _18: Adt52;
let _19: (isize, bool, u32);
let _20: usize;
let _21: [bool; 7];
let _22: [i32; 4];
let _23: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _24: Adt49;
let _25: bool;
let _26: (isize, bool, u32);
let _27: (u32, *const i32, *const u16);
let _28: (isize, bool, u32);
let _29: i8;
let _30: isize;
let _31: &'static f64;
let _32: isize;
let _33: (*mut &'static f64, u64, *mut *mut &'static f64);
let _34: (i128, i32);
let _35: (isize, bool, u32);
let _36: Adt53;
let _37: bool;
let _38: [i16; 5];
let _39: f32;
let _40: ();
let _41: ();
{
_10 = core::ptr::addr_of_mut!((*_2));
RET = core::ptr::addr_of_mut!((*_2));
_5.0.1 = _6 << (*_10);
(*RET) = _8 as u16;
_5.0 = (_1.0, _7);
_1.0 = _5.0.0;
_4 = [(-18334_i16),(-9619_i16),14970_i16,(-19778_i16),(-17088_i16)];
_5.0.1 = -_7;
(*RET) = 129758267085698436309487669631998717082_u128 as u16;
_1.1 = !_6;
_3 = 209_u8 as f64;
_3 = (-113_isize) as f64;
_8 = _5.2 + _5.2;
_11.fld1 = 2195054165_u32;
_1.1 = _8 as i32;
(*RET) = 48544_u16 >> _1.1;
_5.0.1 = _7 | _1.1;
_15.2 = _11.fld1;
_15.1 = (*_10) > (*RET);
_13 = 77087133314492576045189009279826801966_u128 & 105756879054587429410225683649086499636_u128;
(*RET) = 41410_u16;
_3 = _8 as f64;
_16 = _8 as f32;
_15.0 = 9241_i16 as isize;
Goto(bb1)
}
bb1 = {
_13 = 28937_i16 as u128;
_15 = ((-9223372036854775808_isize), false, _11.fld1);
_1 = (_5.0.0, _5.0.1);
_1.0 = _16 as i128;
_9 = '\u{d212f}';
_14 = _3 as usize;
_7 = _1.1;
_11.fld2 = [106_u8,51_u8,99_u8,113_u8];
(*RET) = !54033_u16;
(*RET) = 198_u8 as u16;
_2 = core::ptr::addr_of_mut!((*RET));
_12 = (-4185413767593685251_i64) << _7;
_4 = [23337_i16,(-28455_i16),(-8305_i16),20256_i16,342_i16];
(*_2) = 49103_u16;
_7 = !_5.0.1;
match (*RET) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
49103 => bb8,
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
_17 = _1.1 < _5.0.1;
_15.0 = (-9223372036854775808_isize);
(*_10) = 49867_u16;
_12 = 249_u8 as i64;
_5.0.1 = _7;
_1.1 = _3 as i32;
_1.0 = _5.0.0 * _5.0.0;
_11.fld0 = (*_2) as i32;
_14 = 15543504336414414332_usize;
(*RET) = _13 as u16;
_5.0.0 = _13 as i128;
(*RET) = (-4454_i16) as u16;
_1 = (_5.0.0, _5.0.1);
_5.1 = [_5.0.0,_1.0,_5.0.0];
Goto(bb9)
}
bb9 = {
_16 = _13 as f32;
_17 = _15.1;
_11.fld2 = [31_u8,61_u8,222_u8,172_u8];
_1.1 = _5.0.1 << _1.0;
_5.1 = [_1.0,_1.0,_5.0.0];
_15.2 = _11.fld1 & _11.fld1;
_19.2 = _7 as u32;
_15.1 = _17;
_2 = core::ptr::addr_of_mut!((*_2));
_14 = 6343000012546097475_u64 as usize;
_11.fld1 = _19.2 & _19.2;
_19.1 = (*RET) == (*RET);
(*RET) = !24903_u16;
_3 = 150_u8 as f64;
_15.2 = _11.fld1;
_5.0.1 = _7;
(*_10) = !42140_u16;
_11.fld0 = _5.0.1;
_19 = (_15.0, _15.1, _15.2);
_15.2 = _19.2 << _5.2;
_22 = [_11.fld0,_11.fld0,_1.1,_11.fld0];
_23.4 = -_3;
_23.0 = _23.4 as usize;
_23.2 = _13 - _13;
_23.1.0.0 = _5.0.0;
_5.0.0 = _23.1.0.0 * _1.0;
Call((*RET) = fn3(_22, _19, _15), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_21 = [_19.1,_15.1,_19.1,_15.1,_17,_19.1,_17];
_23.1.0 = (_5.0.0, _11.fld0);
_5.0 = _23.1.0;
_19.0 = _15.0 >> _1.1;
Goto(bb11)
}
bb11 = {
_23.4 = -_3;
_5.1 = [_23.1.0.0,_5.0.0,_1.0];
_23 = (_14, _5, _13, _13, _3);
(*_2) = 3481_u16;
(*_2) = !37133_u16;
_17 = _19.1 | _19.1;
_23.1.0.0 = -_1.0;
_20 = _15.1 as usize;
_26.2 = !_19.2;
_29 = 13815933469690240719_u64 as i8;
_23.4 = _3 + _3;
_27.0 = !_15.2;
_26 = _15;
Goto(bb12)
}
bb12 = {
_27.2 = core::ptr::addr_of!((*_2));
_31 = &_23.4;
_32 = -_19.0;
_23.1.0 = (_5.0.0, _11.fld0);
(*_10) = _9 as u16;
(*_10) = 95_u8 as u16;
_28.0 = -_19.0;
_23.2 = _1.1 as u128;
_23.2 = !_23.3;
_23.1.1 = [_23.1.0.0,_1.0,_5.0.0];
_19 = _15;
_33.2 = core::ptr::addr_of_mut!(_33.0);
_17 = _5.2 >= _8;
_8 = 84_u8 as i8;
_20 = 196_u8 as usize;
_11.fld1 = _15.2 * _27.0;
_23.4 = -_3;
_19.1 = _17;
_26.1 = _17;
_23.0 = _20 + _14;
_13 = !_23.2;
_26.1 = !_15.1;
_13 = _23.3 + _23.3;
RET = core::ptr::addr_of_mut!((*RET));
_11.fld0 = _23.0 as i32;
Goto(bb13)
}
bb13 = {
_11.fld1 = _27.0 + _19.2;
_21 = [_19.1,_17,_26.1,_15.1,_26.1,_26.1,_17];
_35 = (_28.0, _17, _26.2);
_11.fld0 = _1.0 as i32;
RET = _2;
_23.1.0 = _5.0;
RET = core::ptr::addr_of_mut!((*RET));
_23.1.1 = [_5.0.0,_5.0.0,_5.0.0];
(*_2) = _1.1 as u16;
_28.2 = !_27.0;
_1.0 = _23.1.0.0;
_15 = (_32, _17, _35.2);
_31 = &_3;
_35.0 = _32 | _15.0;
_19.1 = _26.2 < _26.2;
_14 = !_20;
_5.0 = (_23.1.0.0, _6);
_2 = RET;
_34 = (_5.0.0, _7);
_29 = -_5.2;
_5.0.0 = _1.0 + _1.0;
match _19.0 {
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb14 = {
_13 = 28937_i16 as u128;
_15 = ((-9223372036854775808_isize), false, _11.fld1);
_1 = (_5.0.0, _5.0.1);
_1.0 = _16 as i128;
_9 = '\u{d212f}';
_14 = _3 as usize;
_7 = _1.1;
_11.fld2 = [106_u8,51_u8,99_u8,113_u8];
(*RET) = !54033_u16;
(*RET) = 198_u8 as u16;
_2 = core::ptr::addr_of_mut!((*RET));
_12 = (-4185413767593685251_i64) << _7;
_4 = [23337_i16,(-28455_i16),(-8305_i16),20256_i16,342_i16];
(*_2) = 49103_u16;
_7 = !_5.0.1;
match (*RET) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
49103 => bb8,
_ => bb7
}
}
bb15 = {
_20 = _14 >> (*_10);
_15 = (_35.0, _35.1, _11.fld1);
_26 = (_15.0, _19.1, _27.0);
_28.2 = _11.fld1 & _26.2;
_33.1 = 14558707156196586103_u64;
_23.1.1 = [_34.0,_23.1.0.0,_1.0];
_9 = '\u{6f4ac}';
_10 = RET;
_12 = 8611951797777139193_i64;
_6 = _7;
_6 = _1.1 | _34.1;
_5.0.0 = _23.1.0.0 << _26.0;
Goto(bb16)
}
bb16 = {
Call(_40 = dump_var(2_usize, 1_usize, Move(_1), 29_usize, Move(_29), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(2_usize, 9_usize, Move(_9), 8_usize, Move(_8), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(2_usize, 13_usize, Move(_13), 26_usize, Move(_26), 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i32; 4],mut _2: (isize, bool, u32),mut _3: (isize, bool, u32)) -> u16 {
mir! {
type RET = u16;
let _4: f64;
let _5: Adt53;
let _6: Adt53;
let _7: *mut ((i128, i32), [i128; 3], i8);
let _8: u128;
let _9: *const i32;
let _10: [i16; 5];
let _11: i128;
let _12: bool;
let _13: isize;
let _14: bool;
let _15: u64;
let _16: isize;
let _17: f32;
let _18: [bool; 7];
let _19: isize;
let _20: f64;
let _21: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _22: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _23: Adt50;
let _24: [i128; 3];
let _25: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _26: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _27: &'static f64;
let _28: usize;
let _29: *const u16;
let _30: ();
let _31: ();
{
_3 = (_2.0, _2.1, _2.2);
_2.1 = _3.1;
_3 = (_2.0, _2.1, _2.2);
_2.1 = !_3.1;
_3.2 = _2.2 + _2.2;
RET = !31515_u16;
RET = 62893_u16;
RET = !7734_u16;
_2.0 = _3.0;
_3.2 = _2.2;
_2.1 = _2.2 < _2.2;
_3.1 = !_2.1;
_3.0 = -_2.0;
_2.1 = !_3.1;
_4 = (-65386926105001878587461747389529339667_i128) as f64;
_1 = [1513900565_i32,1777178527_i32,(-1941523533_i32),1880656101_i32];
RET = 3603861451538187610_u64 as u16;
RET = _2.1 as u16;
RET = 50454_u16;
_3.2 = _2.2 + _2.2;
_3 = (_2.0, _2.1, _2.2);
_3 = (_2.0, _2.1, _2.2);
_1 = [(-1652026250_i32),283098938_i32,(-1909335136_i32),895051127_i32];
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
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
_4 = _3.0 as f64;
RET = 58318_u16 & 41008_u16;
_2.0 = (-1815595411_i32) as isize;
_3.2 = !_2.2;
_3.0 = _2.0;
_3.1 = !_2.1;
_3.1 = _2.1;
_2 = (_3.0, _3.1, _3.2);
_8 = !197045838196643340475400897186692504119_u128;
_2.2 = _3.2 | _3.2;
_2.2 = _3.2 * _3.2;
_10 = [32159_i16,(-5207_i16),(-26302_i16),(-15501_i16),(-25089_i16)];
_3.0 = 18215072227447721853_usize as isize;
_3.1 = _2.1;
_3 = (_2.0, _2.1, _2.2);
_3.1 = _2.1;
_4 = (-99271805699290129364066254620204510961_i128) as f64;
_2.1 = _3.1;
_3 = (_2.0, _2.1, _2.2);
_3.2 = !_2.2;
_8 = 234_u8 as u128;
_2.0 = 20348_i16 as isize;
Goto(bb7)
}
bb7 = {
_10 = [30019_i16,14137_i16,(-27715_i16),(-3652_i16),30690_i16];
_3.1 = _2.1;
_3.0 = _2.0;
RET = '\u{65143}' as u16;
_8 = 133545271252355595564060209065366605089_u128 & 105348429767011519111537065074588368064_u128;
RET = !50862_u16;
_2.1 = _3.1 > _3.1;
_12 = _3.1;
_13 = _3.0 & _3.0;
_3.0 = _13 - _2.0;
_3.2 = _2.2;
RET = 37452_u16;
_3.2 = !_2.2;
RET = '\u{13260}' as u16;
_3.1 = _2.1;
_8 = !94283781244111355527675406106655851184_u128;
_3.2 = _2.2 << _3.0;
_3 = (_13, _12, _2.2);
Goto(bb8)
}
bb8 = {
_4 = 62217398_i32 as f64;
_12 = _2.1;
_3.2 = _2.2;
_10 = [30010_i16,(-27615_i16),(-25365_i16),19846_i16,2906_i16];
_16 = !_2.0;
_4 = 23038_i16 as f64;
RET = 7848911759090805138_u64 as u16;
_15 = 0_usize as u64;
_9 = core::ptr::addr_of!(_21.1.0.1);
_18 = [_2.1,_3.1,_12,_12,_3.1,_2.1,_3.1];
RET = !19572_u16;
_15 = 14474264906517316528_u64 + 4866665529460788160_u64;
_11 = (-152978465143099460120753143867247362074_i128) + 160129237686264628882888986870298904388_i128;
Call(_5 = fn4(_3.1, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_22.1.0 = (_11, (-961503896_i32));
_2.0 = !_3.0;
_10 = [17823_i16,(-23972_i16),(-31745_i16),(-10946_i16),(-21291_i16)];
_23.fld0.fld2 = [228_u8,97_u8,29_u8,116_u8];
_21.1.0.0 = !_22.1.0.0;
_2.1 = !_12;
_20 = _4 - _4;
_23.fld0.fld0 = _22.1.0.1 - _22.1.0.1;
_22.4 = _20;
_22.1.1 = [_22.1.0.0,_22.1.0.0,_21.1.0.0];
Goto(bb10)
}
bb10 = {
_6 = Adt53::Variant0 { fld0: Field::<[bool; 7]>(Variant(_5, 0), 0) };
_22.2 = _8 & _8;
_22.4 = _4 + _20;
match _22.1.0.1 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb8,
340282366920938463463374607430806707560 => bb11,
_ => bb6
}
}
bb11 = {
_9 = core::ptr::addr_of!(_25.1.0.1);
_8 = _22.2 & _22.2;
_24 = _22.1.1;
_26.1.0.1 = _22.1.0.1 & _23.fld0.fld0;
_2.1 = !_12;
_19 = _2.0;
_3.1 = _2.1;
_9 = core::ptr::addr_of!(_22.1.0.1);
_25.3 = !_8;
_21.1.0.1 = !_23.fld0.fld0;
_25.1.2 = -125_i8;
_10 = [11522_i16,(-6680_i16),24164_i16,11259_i16,23758_i16];
_26.1 = (_21.1.0, _24, _25.1.2);
_21.1.2 = _25.1.2 >> _26.1.2;
_25.2 = !_8;
_21.2 = _8;
_21.3 = _21.2;
_6 = Move(_5);
_26.4 = -_4;
_3 = (_2.0, _2.1, _2.2);
Goto(bb12)
}
bb12 = {
_3.0 = _2.0 | _19;
_26.3 = !_25.2;
_26.1.0.1 = _2.0 as i32;
match _22.1.0.1 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb7,
340282366920938463463374607430806707560 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_17 = _22.1.0.0 as f32;
_26.4 = 4765226549578823662_i64 as f64;
_13 = 92_u8 as isize;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(3_usize, 24_usize, Move(_24), 15_usize, Move(_15), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(3_usize, 19_usize, Move(_19), 16_usize, Move(_16), 31_usize, _31, 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: (isize, bool, u32)) -> Adt53 {
mir! {
type RET = Adt53;
let _3: char;
let _4: Adt60;
let _5: [u8; 4];
let _6: i32;
let _7: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _8: Adt52;
let _9: isize;
let _10: char;
let _11: [u8; 4];
let _12: ((i128, i32), [i128; 3], i8);
let _13: Adt55;
let _14: &'static f64;
let _15: u32;
let _16: u128;
let _17: [i16; 5];
let _18: (f64, (i128, i32), i128);
let _19: bool;
let _20: Adt48;
let _21: [i32; 4];
let _22: u32;
let _23: u8;
let _24: isize;
let _25: i64;
let _26: (f64, (i128, i32), i128);
let _27: (i128, i32);
let _28: Adt56;
let _29: [i128; 4];
let _30: (f64, (i128, i32), i128);
let _31: [u8; 4];
let _32: char;
let _33: f32;
let _34: Adt49;
let _35: bool;
let _36: (isize, bool, u32);
let _37: [bool; 7];
let _38: Adt57;
let _39: bool;
let _40: isize;
let _41: char;
let _42: u32;
let _43: Adt45;
let _44: i32;
let _45: u64;
let _46: (i128, i32);
let _47: Adt51;
let _48: [i128; 3];
let _49: i32;
let _50: f64;
let _51: [i32; 4];
let _52: bool;
let _53: bool;
let _54: u32;
let _55: isize;
let _56: (f64, (i128, i32), i128);
let _57: char;
let _58: u128;
let _59: usize;
let _60: (i128, i32);
let _61: [i32; 4];
let _62: u32;
let _63: (*mut &'static f64, u64, *mut *mut &'static f64);
let _64: u16;
let _65: u64;
let _66: u8;
let _67: bool;
let _68: isize;
let _69: bool;
let _70: (isize, bool, u32);
let _71: Adt45;
let _72: i128;
let _73: [i16; 5];
let _74: *mut u16;
let _75: char;
let _76: ();
let _77: ();
{
_3 = '\u{10d7ed}';
_2.1 = _1 ^ _1;
_2 = ((-9223372036854775808_isize), _1, 1343934489_u32);
_4.fld1 = 214750006100702662111865394244965155444_u128 as f32;
_4.fld3.1.1 = !(-528673105_i32);
_4.fld0.5.2 = -143335067289663450655271932047729642394_i128;
_4.fld0.4.1 = !_4.fld3.1.1;
_4.fld3.1.0 = !_4.fld0.5.2;
_4.fld0.5.2 = _4.fld3.1.0;
_4.fld0.1 = 5718338654540412815_u64 as usize;
_4.fld5 = core::ptr::addr_of!(_4.fld0.3);
_4.fld4 = 320934418077870264550996977015232808035_u128 as f64;
_4.fld3.1.0 = !_4.fld0.5.2;
_4.fld0.3 = 47_u8 ^ 202_u8;
_4.fld0.5.1 = (_4.fld3.1.0, _4.fld3.1.1);
_7.1.2 = !(-37_i8);
_4.fld0.6 = !_4.fld0.4.1;
_5 = [_4.fld0.3,_4.fld0.3,_4.fld0.3,_4.fld0.3];
_7.1.0.0 = _4.fld1 as i128;
_7.3 = 4341989337537490884014635000763642794_u128;
_4.fld0.3 = 242_u8 ^ 14_u8;
Goto(bb1)
}
bb1 = {
_7.1.1 = [_4.fld0.5.1.0,_4.fld3.1.0,_4.fld3.1.0];
_2.1 = _1;
_4.fld0.4 = _4.fld3.1;
match _2.2 {
0 => bb2,
1 => bb3,
2 => bb4,
1343934489 => bb6,
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
_2.2 = 3696545493_u32;
_4.fld0.2 = core::ptr::addr_of!(_4.fld0.5.1.1);
_4.fld0.5.0 = _2.0 as f64;
_4.fld0.2 = core::ptr::addr_of!(_4.fld0.6);
_7.1.2 = -70_i8;
Goto(bb7)
}
bb7 = {
_4.fld3.2 = !_7.1.0.0;
_4.fld3.0 = _4.fld0.5.0;
_7.1.0.1 = _4.fld0.6 >> _2.0;
_4.fld0.4.0 = _4.fld3.2 >> _7.1.2;
_4.fld3.1.1 = 1388_u16 as i32;
_6 = _7.1.0.1;
_4.fld0.0 = -_2.0;
_4.fld0.5 = (_4.fld3.0, _7.1.0, _7.1.0.0);
_9 = _2.0 | _2.0;
_4.fld0.5.1 = _7.1.0;
_4.fld5 = core::ptr::addr_of!(_4.fld0.3);
_9 = -_4.fld0.0;
_7.1.1 = [_4.fld0.4.0,_4.fld3.2,_4.fld0.4.0];
_11 = [_4.fld0.3,_4.fld0.3,_4.fld0.3,_4.fld0.3];
_4.fld0.5 = (_4.fld3.0, _7.1.0, _7.1.0.0);
_4.fld0.5 = (_4.fld3.0, _7.1.0, _4.fld0.4.0);
_2.0 = _9 >> _4.fld0.5.1.1;
_7.1.0 = (_4.fld0.5.2, _4.fld0.5.1.1);
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
3696545493 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_4.fld0.4 = (_4.fld3.2, _7.1.0.1);
_4.fld0.5 = (_4.fld3.0, _7.1.0, _4.fld3.2);
_4.fld0.5.1 = (_4.fld3.2, _6);
_4.fld3.1.1 = _4.fld0.5.0 as i32;
_7.1.2 = (-77_i8) & (-13_i8);
_12.2 = _9 as i8;
_4.fld0.4 = _4.fld0.5.1;
_4.fld0.4.0 = -_4.fld3.2;
_1 = _2.1;
_2.1 = _1 | _1;
_3 = '\u{7940}';
Goto(bb10)
}
bb10 = {
_7.1.0.0 = _2.0 as i128;
_14 = &_7.4;
_2 = (_9, _1, 1001395182_u32);
_4.fld0.5 = (_4.fld3.0, _4.fld3.1, _7.1.0.0);
_13.fld2 = 3512636969797829050_i64 as isize;
_2 = (_4.fld0.0, _1, 1100682766_u32);
_2.2 = 1768676157_u32;
_13.fld2 = -_4.fld0.0;
_7.2 = _7.3 << _13.fld2;
_4.fld4 = _4.fld0.5.0;
_2.0 = (-3873922089652311337_i64) as isize;
_7.1.0 = (_4.fld0.5.2, _6);
_6 = _7.2 as i32;
_4.fld0.6 = !_4.fld3.1.1;
_18.2 = _7.1.0.0;
_7.1.0.1 = _4.fld0.4.1;
_12.1 = _7.1.1;
_6 = !_4.fld0.5.1.1;
_7.1.0.0 = _4.fld0.5.2;
_4.fld0.5.0 = _4.fld0.5.1.1 as f64;
match _2.2 {
1768676157 => bb12,
_ => bb11
}
}
bb11 = {
_7.1.1 = [_4.fld0.5.1.0,_4.fld3.1.0,_4.fld3.1.0];
_2.1 = _1;
_4.fld0.4 = _4.fld3.1;
match _2.2 {
0 => bb2,
1 => bb3,
2 => bb4,
1343934489 => bb6,
_ => bb5
}
}
bb12 = {
_7.0 = _4.fld0.1 * _4.fld0.1;
_14 = &_7.4;
_18.0 = 8953211893401269884_u64 as f64;
_4.fld5 = core::ptr::addr_of!(_4.fld0.3);
_10 = _3;
_4.fld3.1.1 = !_4.fld0.4.1;
_4.fld0.4 = (_18.2, _6);
_7.4 = -_4.fld0.5.0;
_12.1 = _7.1.1;
_4.fld0.5.1.0 = _7.2 as i128;
_18.1 = _4.fld0.5.1;
_16 = _7.2;
_7.1.0.1 = _2.2 as i32;
_16 = _7.2 % _7.3;
_2.0 = _4.fld0.0;
_13.fld1 = 10485465978300524521_u64 as f64;
_18.1.0 = !_4.fld0.4.0;
_4.fld0.0 = _2.2 as isize;
_4.fld3.1.1 = _2.2 as i32;
_7.4 = _4.fld3.0 - _4.fld3.0;
_4.fld0.4.0 = _4.fld0.5.2 | _4.fld0.5.1.0;
_2.1 = _1 | _1;
_2.0 = _9;
_12.2 = -_7.1.2;
_23 = _12.2 as u8;
match _2.2 {
0 => bb13,
1 => bb14,
1768676157 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_4.fld0.4 = (_4.fld3.2, _7.1.0.1);
_4.fld0.5 = (_4.fld3.0, _7.1.0, _4.fld3.2);
_4.fld0.5.1 = (_4.fld3.2, _6);
_4.fld3.1.1 = _4.fld0.5.0 as i32;
_7.1.2 = (-77_i8) & (-13_i8);
_12.2 = _9 as i8;
_4.fld0.4 = _4.fld0.5.1;
_4.fld0.4.0 = -_4.fld3.2;
_1 = _2.1;
_2.1 = _1 | _1;
_3 = '\u{7940}';
Goto(bb10)
}
bb16 = {
_4.fld5 = core::ptr::addr_of!(_4.fld0.3);
_7.1.0.1 = !_4.fld0.4.1;
_16 = _7.2;
_12.0.1 = _18.1.1;
_2 = (_9, _1, 1875670305_u32);
_7.1.0 = _18.1;
_18.1.0 = _4.fld0.4.0 ^ _18.2;
_4.fld0.4.1 = -_6;
_4.fld0.3 = _23 + _23;
_4.fld3 = _18;
Call(_2.2 = fn5(_2.0, _18.1.1, _7.2, _4.fld3.1), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_16 = _4.fld0.3 as u128;
Goto(bb18)
}
bb18 = {
_12.0.1 = _2.2 as i32;
_21 = [_4.fld0.4.1,_18.1.1,_6,_18.1.1];
_4.fld0.5.2 = -_4.fld0.4.0;
_7.0 = !_4.fld0.1;
_2 = (_9, _1, 2556675716_u32);
_4.fld0.2 = core::ptr::addr_of!(_4.fld0.4.1);
_12 = (_7.1.0, _7.1.1, _7.1.2);
_17 = [21639_i16,(-751_i16),25382_i16,(-4934_i16),(-23593_i16)];
_2.1 = !_1;
_7.3 = _7.2 >> _18.1.0;
Goto(bb19)
}
bb19 = {
_4.fld4 = -_4.fld0.5.0;
_4.fld0.4.0 = !_12.0.0;
_4.fld4 = _7.1.2 as f64;
_1 = _2.1 & _2.1;
RET = Adt53::Variant1 { fld0: _17,fld1: (-3417488226440610901_i64),fld2: _4.fld0,fld3: _4.fld0.5,fld4: 15037074257655866497_u64,fld5: _18.1.0 };
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5 = (_4.fld0.5.0, _4.fld3.1, Field::<i128>(Variant(RET, 1), 5));
match _2.2 {
0 => bb5,
1 => bb9,
2556675716 => bb21,
_ => bb20
}
}
bb20 = {
Return()
}
bb21 = {
_3 = _10;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0 + _13.fld1;
_3 = _10;
Goto(bb22)
}
bb22 = {
_4.fld3.1 = (_4.fld0.5.2, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.1);
_4.fld5 = core::ptr::addr_of!(place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).3);
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1.1 = -_18.1.1;
_19 = !_1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = _4.fld0.5.0 * Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2, _4.fld0.6);
_6 = !_4.fld3.1.1;
_7 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1, _12, _16, _16, Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0);
_4.fld0.0 = _4.fld1 as isize;
_26.2 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0;
_14 = &_13.fld1;
_26 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1, _4.fld0.4.0);
_27 = _4.fld3.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5 = (_7.4, _27, _26.2);
_26.2 = 10744_u16 as i128;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).2 = -Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0;
_26.1.1 = _7.1.0.1 >> _4.fld3.1.1;
_22 = _4.fld1 as u32;
place!(Field::<u64>(Variant(RET, 1), 4)) = 7490336684663599697_u64;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (_26.1.0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.1);
_18.1 = _26.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (_26.1.0, _4.fld0.4.1);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = _4.fld1 as i32;
_13.fld1 = _26.0;
_29 = [Field::<i128>(Variant(RET, 1), 5),Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0,Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0,_12.0.0];
_4.fld0.5.1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0, _4.fld3.1.1);
RET = Adt53::Variant1 { fld0: _17,fld1: (-6229743357606288669_i64),fld2: _4.fld0,fld3: _26,fld4: 9459952968116852454_u64,fld5: _12.0.0 };
Goto(bb23)
}
bb23 = {
_4.fld3.0 = _23 as f64;
place!(Field::<i64>(Variant(RET, 1), 1)) = !8624706762779087689_i64;
_7.1.0.0 = Field::<i128>(Variant(RET, 1), 5);
_9 = _2.0 - _13.fld2;
_11 = _5;
_9 = _7.4 as isize;
_7.1.2 = _12.2 * _12.2;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.1.1 = -_4.fld0.5.1.1;
_4.fld0.1 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1 ^ Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1;
_12.0 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1;
_4.fld3 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0, _27, Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0);
_15 = _4.fld1 as u32;
_12.0.1 = _4.fld0.5.1.1 + Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.1;
_7.1.1 = _12.1;
place!(Field::<i64>(Variant(RET, 1), 1)) = 379197333088027307_i64 & (-4346505341785700790_i64);
_4.fld0.4.1 = !Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).4.1 = _7.3 as i32;
match _2.2 {
0 => bb15,
1 => bb3,
2 => bb24,
3 => bb25,
4 => bb26,
5 => bb27,
2556675716 => bb29,
_ => bb28
}
}
bb24 = {
_4.fld3.1 = (_4.fld0.5.2, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.1);
_4.fld5 = core::ptr::addr_of!(place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).3);
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1.1 = -_18.1.1;
_19 = !_1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = _4.fld0.5.0 * Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2, _4.fld0.6);
_6 = !_4.fld3.1.1;
_7 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1, _12, _16, _16, Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0);
_4.fld0.0 = _4.fld1 as isize;
_26.2 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0;
_14 = &_13.fld1;
_26 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1, _4.fld0.4.0);
_27 = _4.fld3.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5 = (_7.4, _27, _26.2);
_26.2 = 10744_u16 as i128;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).2 = -Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0;
_26.1.1 = _7.1.0.1 >> _4.fld3.1.1;
_22 = _4.fld1 as u32;
place!(Field::<u64>(Variant(RET, 1), 4)) = 7490336684663599697_u64;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (_26.1.0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.1);
_18.1 = _26.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (_26.1.0, _4.fld0.4.1);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = _4.fld1 as i32;
_13.fld1 = _26.0;
_29 = [Field::<i128>(Variant(RET, 1), 5),Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0,Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0,_12.0.0];
_4.fld0.5.1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0, _4.fld3.1.1);
RET = Adt53::Variant1 { fld0: _17,fld1: (-6229743357606288669_i64),fld2: _4.fld0,fld3: _26,fld4: 9459952968116852454_u64,fld5: _12.0.0 };
Goto(bb23)
}
bb25 = {
_4.fld3.2 = !_7.1.0.0;
_4.fld3.0 = _4.fld0.5.0;
_7.1.0.1 = _4.fld0.6 >> _2.0;
_4.fld0.4.0 = _4.fld3.2 >> _7.1.2;
_4.fld3.1.1 = 1388_u16 as i32;
_6 = _7.1.0.1;
_4.fld0.0 = -_2.0;
_4.fld0.5 = (_4.fld3.0, _7.1.0, _7.1.0.0);
_9 = _2.0 | _2.0;
_4.fld0.5.1 = _7.1.0;
_4.fld5 = core::ptr::addr_of!(_4.fld0.3);
_9 = -_4.fld0.0;
_7.1.1 = [_4.fld0.4.0,_4.fld3.2,_4.fld0.4.0];
_11 = [_4.fld0.3,_4.fld0.3,_4.fld0.3,_4.fld0.3];
_4.fld0.5 = (_4.fld3.0, _7.1.0, _7.1.0.0);
_4.fld0.5 = (_4.fld3.0, _7.1.0, _4.fld0.4.0);
_2.0 = _9 >> _4.fld0.5.1.1;
_7.1.0 = (_4.fld0.5.2, _4.fld0.5.1.1);
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
3696545493 => bb9,
_ => bb8
}
}
bb26 = {
Return()
}
bb27 = {
Return()
}
bb28 = {
_4.fld0.4 = (_4.fld3.2, _7.1.0.1);
_4.fld0.5 = (_4.fld3.0, _7.1.0, _4.fld3.2);
_4.fld0.5.1 = (_4.fld3.2, _6);
_4.fld3.1.1 = _4.fld0.5.0 as i32;
_7.1.2 = (-77_i8) & (-13_i8);
_12.2 = _9 as i8;
_4.fld0.4 = _4.fld0.5.1;
_4.fld0.4.0 = -_4.fld3.2;
_1 = _2.1;
_2.1 = _1 | _1;
_3 = '\u{7940}';
Goto(bb10)
}
bb29 = {
_14 = &place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).0;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).4.0 = _12.0.1 as i128;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1.1 = _27.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = _26.1.1 - _4.fld3.1.1;
RET = Adt53::Variant1 { fld0: _17,fld1: 408345389226475959_i64,fld2: _4.fld0,fld3: _4.fld0.5,fld4: 847479671910763281_u64,fld5: _4.fld0.4.0 };
_18.1.1 = _4.fld3.1.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = _18.1;
_32 = _10;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.1.0 = -_12.0.0;
_7.4 = -_26.0;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).2 = core::ptr::addr_of!(_12.0.1);
_26 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4, _4.fld3.1.0);
_7.0 = !Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1;
_32 = _3;
_23 = !Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).3;
_12.1 = _7.1.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).4 = (_4.fld0.5.1.0, Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.1);
_13.fld1 = _12.2 as f64;
place!(Field::<i128>(Variant(RET, 1), 5)) = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2;
_4.fld3.1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0, _27.1);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.1.1 = !Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).6;
_13.fld1 = _7.4 + _26.0;
match _2.2 {
0 => bb20,
1 => bb26,
2 => bb3,
2556675716 => bb31,
_ => bb30
}
}
bb30 = {
_16 = _4.fld0.3 as u128;
Goto(bb18)
}
bb31 = {
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = (-3911896407271521068_i64) as i32;
_41 = _10;
_18.1 = (_12.0.0, _4.fld0.5.1.1);
_36.2 = _2.2 & _2.2;
_36 = (_9, _19, _2.2);
_26.1 = (Field::<i128>(Variant(RET, 1), 5), _12.0.1);
_18.1.1 = !_4.fld0.4.1;
_7.1.2 = _12.2;
_18 = (_7.4, _26.1, _4.fld0.5.1.0);
_23 = _4.fld0.3 * Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).3;
_4.fld3.1.1 = _18.1.1 & _12.0.1;
_45 = 13423808131465344759_u64 * 8657495821026790039_u64;
_13.fld0 = [_4.fld3.2,_7.1.0.0,Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.2];
_7 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1, _12, _16, _16, _13.fld1);
Goto(bb32)
}
bb32 = {
_7.2 = !_7.3;
_3 = _41;
Goto(bb33)
}
bb33 = {
_23 = _4.fld0.3;
_32 = _10;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).1 = _7.0;
_7.4 = -_18.0;
_36.2 = _2.2;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).0 = -_26.0;
_13.fld2 = _9;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = (-22781_i16) as f64;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = _13.fld1 - _13.fld1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = !_7.1.0.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0 - _26.0;
_30.1 = (_26.2, _4.fld3.1.1);
_7.1.0 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0, _4.fld0.6);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).3 = _7.1.0.1 as u8;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.1.0 = -_4.fld0.5.1.0;
_10 = _32;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1.1 = _30.1.1 | _12.0.1;
_39 = _1;
_2.1 = _19 | _36.1;
Goto(bb34)
}
bb34 = {
_4.fld0.5.2 = -Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2;
_4.fld0.5.1 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1;
_7.1.0.0 = _3 as i128;
_14 = &_7.4;
_39 = _36.1 ^ _2.1;
_25 = 1778116193821347233_i64 ^ (-6579969014570766458_i64);
place!(Field::<u64>(Variant(RET, 1), 4)) = 3838_i16 as u64;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).3 = _4.fld0.3;
_7.4 = -Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0;
_55 = _4.fld0.1 as isize;
Goto(bb35)
}
bb35 = {
_46 = (_18.2, _26.1.1);
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).0 = -_18.0;
_27.1 = _32 as i32;
_56.1.1 = _12.0.1;
_2 = _36;
_49 = !Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.1;
_18.2 = _7.0 as i128;
_2.0 = -_36.0;
_30.1.0 = 5282_u16 as i128;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).4.0 = _46.0 - Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.2;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5 = (_7.4, _27, _46.0);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = -_18.1.1;
_30.0 = _36.2 as f64;
_2.0 = _36.0;
_19 = _12.0.1 <= Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).6;
_7.1.1 = _13.fld0;
_31 = _11;
_18.0 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0 as f64;
_40 = _36.0 | _13.fld2;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)) = _4.fld0.5;
_59 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1;
_12.2 = _7.1.2 + _7.1.2;
match _2.2 {
0 => bb14,
1 => bb17,
2 => bb30,
3 => bb36,
2556675716 => bb38,
_ => bb37
}
}
bb36 = {
_4.fld0.5.2 = -Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2;
_4.fld0.5.1 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1;
_7.1.0.0 = _3 as i128;
_14 = &_7.4;
_39 = _36.1 ^ _2.1;
_25 = 1778116193821347233_i64 ^ (-6579969014570766458_i64);
place!(Field::<u64>(Variant(RET, 1), 4)) = 3838_i16 as u64;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).3 = _4.fld0.3;
_7.4 = -Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0;
_55 = _4.fld0.1 as isize;
Goto(bb35)
}
bb37 = {
_4.fld0.4 = (_4.fld3.2, _7.1.0.1);
_4.fld0.5 = (_4.fld3.0, _7.1.0, _4.fld3.2);
_4.fld0.5.1 = (_4.fld3.2, _6);
_4.fld3.1.1 = _4.fld0.5.0 as i32;
_7.1.2 = (-77_i8) & (-13_i8);
_12.2 = _9 as i8;
_4.fld0.4 = _4.fld0.5.1;
_4.fld0.4.0 = -_4.fld3.2;
_1 = _2.1;
_2.1 = _1 | _1;
_3 = '\u{7940}';
Goto(bb10)
}
bb38 = {
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)) = _4.fld0.5;
_4.fld4 = _26.0 + _13.fld1;
Goto(bb39)
}
bb39 = {
_4.fld0.1 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1;
_61 = [_49,_30.1.1,Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).6,Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.1];
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).4.0 = _4.fld3.1.0 + Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5 = (_13.fld1, Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1, _4.fld3.2);
_13.fld1 = _30.0 + _4.fld4;
_12.0.1 = _4.fld0.4.1 * _49;
_18.0 = -Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0;
_2.2 = !_36.2;
_60.0 = _12.2 as i128;
_42 = _36.2 - _36.2;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).0 = !_36.0;
_42 = _2.2;
_59 = !_7.0;
_12.0.1 = _30.1.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).0 = _30.0 + _26.0;
_14 = &_18.0;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1.0 = _26.1.0;
match _36.2 {
0 => bb28,
1 => bb6,
2 => bb40,
2556675716 => bb42,
_ => bb41
}
}
bb40 = {
_4.fld3.1 = (_4.fld0.5.2, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.1);
_4.fld5 = core::ptr::addr_of!(place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).3);
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1.1 = -_18.1.1;
_19 = !_1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = _4.fld0.5.0 * Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2, _4.fld0.6);
_6 = !_4.fld3.1.1;
_7 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).1, _12, _16, _16, Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0);
_4.fld0.0 = _4.fld1 as isize;
_26.2 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0;
_14 = &_13.fld1;
_26 = (Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1, _4.fld0.4.0);
_27 = _4.fld3.1;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5 = (_7.4, _27, _26.2);
_26.2 = 10744_u16 as i128;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).2 = -Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0;
_26.1.1 = _7.1.0.1 >> _4.fld3.1.1;
_22 = _4.fld1 as u32;
place!(Field::<u64>(Variant(RET, 1), 4)) = 7490336684663599697_u64;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (_26.1.0, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.1);
_18.1 = _26.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3)).1 = (_26.1.0, _4.fld0.4.1);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).6 = _4.fld1 as i32;
_13.fld1 = _26.0;
_29 = [Field::<i128>(Variant(RET, 1), 5),Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).4.0,Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0,_12.0.0];
_4.fld0.5.1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0, _4.fld3.1.1);
RET = Adt53::Variant1 { fld0: _17,fld1: (-6229743357606288669_i64),fld2: _4.fld0,fld3: _26,fld4: 9459952968116852454_u64,fld5: _12.0.0 };
Goto(bb23)
}
bb41 = {
_3 = _10;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.0 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).0 + _13.fld1;
_3 = _10;
Goto(bb22)
}
bb42 = {
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).2 = core::ptr::addr_of!(_4.fld0.4.1);
match _36.2 {
0 => bb9,
1 => bb39,
2 => bb26,
2556675716 => bb43,
_ => bb6
}
}
bb43 = {
_4.fld3 = ((*_14), Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1, _26.2);
place!(Field::<[i16; 5]>(Variant(RET, 1), 0)) = [24740_i16,14496_i16,17326_i16,(-19536_i16),18009_i16];
_36.1 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2 != Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.2;
_4.fld3.1 = (_26.2, Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).6);
_48 = [Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).1.0,_4.fld0.5.2,_26.2];
_46.0 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.1.0;
_56.0 = -_30.0;
_26.1.0 = _30.1.1 as i128;
_4.fld0.4 = (_46.0, _4.fld0.5.1.1);
_4.fld3.1.1 = -_7.1.0.1;
_30.1.0 = Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).5.2 & Field::<i128>(Variant(RET, 1), 5);
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.1.1 = _36.2 as i32;
_4.fld0.0 = _9 * _36.0;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).2 = _4.fld0.2;
_3 = _32;
_4.fld0.0 = _2.0 << _4.fld0.5.1.1;
_37 = [_36.1,_19,_19,_2.1,_39,_19,_39];
_23 = !Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2).3;
place!(Field::<(isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32)>(Variant(RET, 1), 2)).5.1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 3).2, _4.fld0.4.1);
_57 = _32;
Call(_65 = core::intrinsics::transmute(_9), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
place!(Field::<i64>(Variant(RET, 1), 1)) = _4.fld1 as i64;
SetDiscriminant(RET, 0);
_46 = _27;
_63.1 = _65;
_52 = _36.1 | _39;
_42 = _2.2;
_36.1 = _19;
_18 = (_4.fld3.0, _4.fld0.4, _4.fld3.1.0);
_4.fld3.1.1 = !_56.1.1;
_51 = [_30.1.1,_56.1.1,_7.1.0.1,_4.fld0.5.1.1];
_60 = (_26.1.0, _49);
_62 = !_36.2;
Call(_18.1.0 = core::intrinsics::bswap(_60.0), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
_56 = (_7.4, _60, _4.fld0.4.0);
_33 = _12.2 as f32;
_13.fld0 = [_4.fld3.2,_26.1.0,_4.fld3.2];
_18.0 = -_4.fld4;
_46.0 = _2.2 as i128;
_10 = _57;
_49 = _26.1.1;
_12 = (_4.fld0.5.1, _13.fld0, _7.1.2);
_13.fld3 = core::ptr::addr_of_mut!(_64);
RET = Adt53::Variant0 { fld0: _37 };
_18.0 = _56.0 * _4.fld3.0;
_4.fld0.5.1.0 = !_60.0;
_4.fld5 = core::ptr::addr_of!(_4.fld0.3);
_68 = !_40;
_18.1.0 = -_4.fld0.4.0;
_13.fld4 = Adt54::Variant3 { fld0: _25,fld1: _12.1 };
Goto(bb46)
}
bb46 = {
Call(_76 = dump_var(4_usize, 22_usize, Move(_22), 52_usize, Move(_52), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_76 = dump_var(4_usize, 55_usize, Move(_55), 46_usize, Move(_46), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_76 = dump_var(4_usize, 32_usize, Move(_32), 45_usize, Move(_45), 17_usize, Move(_17), 68_usize, Move(_68)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_76 = dump_var(4_usize, 2_usize, Move(_2), 3_usize, Move(_3), 21_usize, Move(_21), 49_usize, Move(_49)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_76 = dump_var(4_usize, 36_usize, Move(_36), 41_usize, Move(_41), 48_usize, Move(_48), 59_usize, Move(_59)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: i32,mut _3: u128,mut _4: (i128, i32)) -> u32 {
mir! {
type RET = u32;
let _5: u8;
let _6: [i16; 5];
let _7: [i128; 3];
let _8: (isize, bool, u32);
let _9: ((i128, i32), [i128; 3], i8);
let _10: ((i128, i32), [i128; 3], i8);
let _11: f64;
let _12: isize;
let _13: Adt46;
let _14: Adt46;
let _15: ();
let _16: ();
{
RET = 957420937_u32;
_3 = !150382757247859953169697533733511755008_u128;
_4 = ((-1681241597238429375871494614664633566_i128), _2);
_3 = 62491419868550277120480661239255299792_u128;
_4 = (140310787184579814692607771265475097452_i128, _2);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
62491419868550277120480661239255299792 => bb8,
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
_1 = -(-9223372036854775808_isize);
_6 = [25227_i16,(-14672_i16),(-21946_i16),1740_i16,26514_i16];
_5 = 123_i8 as u8;
_6 = [20332_i16,13456_i16,1787_i16,19400_i16,(-9721_i16)];
Goto(bb9)
}
bb9 = {
_4.1 = _3 as i32;
_2 = -_4.1;
RET = true as u32;
_4.0 = _1 as i128;
_6 = [9335_i16,(-28310_i16),32496_i16,(-13001_i16),(-14280_i16)];
RET = 3640964802_u32;
_1 = 0_usize as isize;
_4.0 = _1 as i128;
_5 = (-49_i8) as u8;
Call(_3 = fn6(_6, _6, _6, _1, _6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4.1 = _2 ^ _2;
_4.0 = !152817845325743290940625504902482690492_i128;
_2 = -_4.1;
_8 = (_1, true, RET);
_4 = ((-122075005182806826093231404082149936175_i128), _2);
_9.0.0 = _4.0;
_10.0.1 = _2;
_9.0 = (_4.0, _10.0.1);
_2 = -_9.0.1;
_4.0 = _9.0.0;
_10.2 = 35_i8 & 109_i8;
_7 = [_9.0.0,_4.0,_4.0];
_1 = _8.0;
_9.1 = [_9.0.0,_9.0.0,_9.0.0];
_7 = [_9.0.0,_4.0,_4.0];
_6 = [(-5349_i16),(-10172_i16),(-11631_i16),(-318_i16),19046_i16];
_7 = [_9.0.0,_9.0.0,_9.0.0];
_8.0 = _1 >> _10.0.1;
_10.2 = (-115_i8);
_3 = 35023751088633801868147089084204259004_u128;
match _3 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb11,
5 => bb12,
35023751088633801868147089084204259004 => bb14,
_ => bb13
}
}
bb11 = {
_4.1 = _3 as i32;
_2 = -_4.1;
RET = true as u32;
_4.0 = _1 as i128;
_6 = [9335_i16,(-28310_i16),32496_i16,(-13001_i16),(-14280_i16)];
RET = 3640964802_u32;
_1 = 0_usize as isize;
_4.0 = _1 as i128;
_5 = (-49_i8) as u8;
Call(_3 = fn6(_6, _6, _6, _1, _6), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_10.1 = [_4.0,_9.0.0,_9.0.0];
_12 = -_8.0;
_10.1 = [_4.0,_4.0,_9.0.0];
_13.fld2 = [_5,_5,_5,_5];
_10 = (_9.0, _7, (-85_i8));
_8.1 = false;
_5 = 170_u8;
_13.fld0 = 1366_u16 as i32;
_9 = (_4, _10.1, _10.2);
_14.fld1 = _8.2;
_4 = (_10.0.0, _10.0.1);
_8 = (_12, true, _14.fld1);
_6 = [(-21343_i16),(-31113_i16),(-30935_i16),18239_i16,(-5322_i16)];
RET = !_8.2;
_4.0 = 0_usize as i128;
_10.0 = (_9.0.0, _9.0.1);
_9.0 = _4;
_8 = (_12, true, RET);
_14 = Adt46 { fld0: _4.1,fld1: RET,fld2: _13.fld2 };
_10.0.0 = _9.0.0;
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(5_usize, 12_usize, Move(_12), 10_usize, Move(_10), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_15 = dump_var(5_usize, 1_usize, Move(_1), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i16; 5],mut _2: [i16; 5],mut _3: [i16; 5],mut _4: isize,mut _5: [i16; 5]) -> u128 {
mir! {
type RET = u128;
let _6: Adt50;
let _7: *mut u16;
let _8: *mut u16;
let _9: u8;
let _10: bool;
let _11: isize;
let _12: f64;
let _13: f32;
let _14: f32;
let _15: u32;
let _16: isize;
let _17: char;
let _18: i64;
let _19: f64;
let _20: isize;
let _21: [bool; 7];
let _22: Adt46;
let _23: [u8; 4];
let _24: Adt49;
let _25: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _26: *mut &'static f64;
let _27: f64;
let _28: ();
let _29: ();
{
RET = 309317195789087051365249902206297477777_u128 << _4;
_5 = _2;
_4 = 9223372036854775807_isize;
_6.fld0.fld1 = !1860262953_u32;
_2 = [(-30882_i16),25176_i16,19645_i16,31634_i16,10355_i16];
_6.fld0.fld0 = (-1490688017_i32) * (-186318497_i32);
RET = !293646971827054919987639243363905751958_u128;
_6.fld0.fld2 = [189_u8,37_u8,247_u8,117_u8];
_3 = _5;
_6.fld0.fld1 = _6.fld0.fld0 as u32;
_6.fld0.fld1 = !2863748498_u32;
_6.fld0.fld0 = 1597734704_i32 << _4;
_5 = [5808_i16,(-15962_i16),(-3078_i16),8008_i16,(-1611_i16)];
_2 = [(-21179_i16),29780_i16,30762_i16,1586_i16,(-28571_i16)];
_1 = _5;
_6.fld0.fld2 = [175_u8,147_u8,252_u8,175_u8];
_6.fld0.fld2 = [111_u8,218_u8,14_u8,109_u8];
_6.fld0.fld0 = _6.fld0.fld1 as i32;
_6.fld0.fld2 = [2_u8,191_u8,60_u8,98_u8];
Goto(bb1)
}
bb1 = {
_9 = 7582540104589143604_usize as u8;
Call(_6 = fn7(_5, _3, _2, _4, _4, _2, _4, _4, _1, _1, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).2;
_6.fld0.fld1 = Field::<u16>(Variant(_6.fld1, 1), 5) as u32;
_10 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).0;
_6.fld0.fld2 = [_9,_9,_9,_9];
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).3 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).0 as i32;
_10 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.1 < _6.fld0.fld0;
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld0 = !Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).1 = (Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).2, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).3);
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)) = (_4, _10, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).4.0);
match Field::<i16>(Variant(_6.fld1, 1), 4) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768196672 => bb9,
_ => bb8
}
}
bb3 = {
_9 = 7582540104589143604_usize as u8;
Call(_6 = fn7(_5, _3, _2, _4, _4, _2, _4, _4, _1, _1, RET), ReturnTo(bb2), UnwindUnreachable())
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
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).1.1 = _9 as i32;
Goto(bb10)
}
bb10 = {
SetDiscriminant(_6.fld1, 1);
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).1.0 = 144846692813863245788837375033515352431_i128;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.0 = _6.fld0.fld0 as f64;
_6.fld0.fld0 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.0 as i32;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.2 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1.0;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).4.2 = core::ptr::addr_of!(place!(Field::<u16>(Variant(_6.fld1, 1), 5)));
_8 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_6.fld1, 1), 5)));
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).4.0 = _6.fld0.fld1 + _6.fld0.fld1;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).4.2 = core::ptr::addr_of!(place!(Field::<u16>(Variant(_6.fld1, 1), 5)));
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).2 = !_6.fld0.fld1;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).4.2 = core::ptr::addr_of!((*_8));
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).4.1 = core::ptr::addr_of!(place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld0);
place!(Field::<u16>(Variant(_6.fld1, 1), 5)) = 44701_u16;
_11 = _4 * _4;
_10 = !true;
_16 = -_4;
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld1 = RET as u32;
Goto(bb11)
}
bb11 = {
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).2 = [(-15222_i16),(-8431_i16),(-31052_i16),3190_i16,(-27881_i16)];
_12 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.0 - Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.0;
_10 = true ^ false;
_19 = 89_i8 as f64;
_17 = '\u{afd51}';
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.1 = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2, _6.fld0.fld0);
_11 = -_16;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).2 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.0 | Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2;
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld0 = -_6.fld0.fld0;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.0 = -_12;
_14 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).2 as f32;
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).0 = _4 * _4;
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).2 = _6.fld0.fld1;
_15 = !Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).4.0;
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld2 = [_9,_9,_9,_9];
_13 = _14;
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).2 = Field::<Adt46>(Variant(_6.fld1, 1), 1).fld1;
_16 = _4 << _6.fld0.fld0;
Goto(bb12)
}
bb12 = {
_16 = Field::<u16>(Variant(_6.fld1, 1), 5) as isize;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).0 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.0;
_10 = true;
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).0 = _10 as isize;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).1.1 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.1;
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).1 = _10 | _10;
place!(Field::<u16>(Variant(_6.fld1, 1), 5)) = 37147_u16;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)) = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.0, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.0);
_6.fld0.fld2 = Field::<Adt46>(Variant(_6.fld1, 1), 1).fld2;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.1 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1;
_22 = Adt46 { fld0: _6.fld0.fld0,fld1: Field::<Adt46>(Variant(_6.fld1, 1), 1).fld1,fld2: Field::<Adt46>(Variant(_6.fld1, 1), 1).fld2 };
_6.fld0 = Move(Field::<Adt46>(Variant(_6.fld1, 1), 1));
_20 = !_11;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.1 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1;
RET = !192565312150468249693191133395198967647_u128;
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)) = Move(_22);
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)) = (_4, _10, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).4.0);
_19 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).0;
_6.fld0 = Adt46 { fld0: Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.1,fld1: _15,fld2: Field::<Adt46>(Variant(_6.fld1, 1), 1).fld2 };
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.2 = !Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.0;
Goto(bb13)
}
bb13 = {
_6.fld0.fld0 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1.1;
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld1 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).4.0 ^ Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).4.0;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).2 = [(-9517_i16),25941_i16,(-27704_i16),(-1440_i16),(-15381_i16)];
place!(Field::<Adt46>(Variant(_6.fld1, 1), 1)).fld2 = _6.fld0.fld2;
_25.4 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).2 as f64;
_11 = _20 & Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0).0;
_22 = Adt46 { fld0: Field::<Adt46>(Variant(_6.fld1, 1), 1).fld0,fld1: _6.fld0.fld1,fld2: Field::<Adt46>(Variant(_6.fld1, 1), 1).fld2 };
_16 = !_20;
_25.1.1 = [Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2,Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1.0];
_25.1.0.0 = -Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2;
_25.4 = _22.fld0 as f64;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.1 = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2, _6.fld0.fld0);
_23 = _22.fld2;
_6.fld0.fld2 = [_9,_9,_9,_9];
_9 = !143_u8;
_25.1.0.0 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.2;
_13 = _14;
_25.2 = !RET;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).1.2 = Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).2 << _22.fld0;
Goto(bb14)
}
bb14 = {
place!(Field::<(isize, bool, u32)>(Variant(_6.fld1, 1), 0)).2 = _15 + _6.fld0.fld1;
_22.fld0 = !Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1.1;
_25.1.0 = (Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3).1.0, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2).1.1.1);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(_6.fld1, 1), 2)).3 = 14192130278391216788_usize as i32;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).1.1 = _25.1.0.1;
_20 = -_4;
place!(Field::<(f64, (i128, i32), i128)>(Variant(_6.fld1, 1), 3)).0 = 13731230009646815803_u64 as f64;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(6_usize, 16_usize, Move(_16), 15_usize, Move(_15), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(6_usize, 2_usize, Move(_2), 5_usize, Move(_5), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i16; 5],mut _2: [i16; 5],mut _3: [i16; 5],mut _4: isize,mut _5: isize,mut _6: [i16; 5],mut _7: isize,mut _8: isize,mut _9: [i16; 5],mut _10: [i16; 5],mut _11: u128) -> Adt50 {
mir! {
type RET = Adt50;
let _12: Adt50;
let _13: (f64, (i128, i32), i128);
let _14: Adt46;
let _15: bool;
let _16: (i128, i32);
let _17: isize;
let _18: f64;
let _19: isize;
let _20: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _21: Adt44;
let _22: (isize, bool, u32);
let _23: isize;
let _24: *const u16;
let _25: u8;
let _26: (*mut [i128; 3],);
let _27: bool;
let _28: [bool; 7];
let _29: u128;
let _30: [i128; 4];
let _31: (i128, i32);
let _32: Adt55;
let _33: i8;
let _34: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _35: f32;
let _36: bool;
let _37: Adt59;
let _38: isize;
let _39: isize;
let _40: *mut u16;
let _41: f64;
let _42: Adt51;
let _43: [i16; 5];
let _44: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32);
let _45: (f64, (i128, i32), i128);
let _46: *mut [i128; 3];
let _47: usize;
let _48: Adt53;
let _49: f32;
let _50: f32;
let _51: ((i128, i32), [i128; 3], i8);
let _52: (isize, bool, u32);
let _53: [u8; 4];
let _54: (isize, bool, u32);
let _55: Adt44;
let _56: i32;
let _57: [i16; 5];
let _58: [bool; 7];
let _59: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32);
let _60: f32;
let _61: (f64, (i128, i32), i128);
let _62: (f64, (i128, i32), i128);
let _63: f64;
let _64: Adt48;
let _65: *mut &'static f64;
let _66: isize;
let _67: Adt49;
let _68: Adt46;
let _69: Adt46;
let _70: char;
let _71: Adt46;
let _72: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _73: i64;
let _74: Adt47;
let _75: bool;
let _76: u16;
let _77: [i128; 3];
let _78: ((i128, i32), [i128; 3], i8);
let _79: Adt45;
let _80: *const i32;
let _81: Adt47;
let _82: i16;
let _83: [bool; 7];
let _84: f32;
let _85: Adt55;
let _86: [i128; 4];
let _87: bool;
let _88: i32;
let _89: [u8; 4];
let _90: u16;
let _91: i16;
let _92: (*mut [i128; 3],);
let _93: i64;
let _94: ((i128, i32), [i128; 3], i8);
let _95: isize;
let _96: ();
let _97: ();
{
_12.fld0.fld2 = [115_u8,67_u8,196_u8,107_u8];
RET.fld0.fld1 = 320392972_u32;
_10 = [(-1738_i16),5456_i16,(-26803_i16),16652_i16,30168_i16];
_12.fld0.fld0 = -(-1704028396_i32);
RET.fld0 = Adt46 { fld0: _12.fld0.fld0,fld1: 3195867071_u32,fld2: _12.fld0.fld2 };
RET.fld0.fld0 = !_12.fld0.fld0;
_12.fld0.fld1 = RET.fld0.fld1;
_14.fld1 = _7 as u32;
_7 = (-9074258513324198756_i64) as isize;
_12.fld0.fld2 = [192_u8,102_u8,229_u8,122_u8];
RET.fld0.fld2 = [12_u8,16_u8,249_u8,91_u8];
RET.fld0.fld1 = 16559389383533380788_u64 as u32;
_14.fld2 = [219_u8,32_u8,174_u8,16_u8];
Goto(bb1)
}
bb1 = {
_6 = [(-19580_i16),(-16886_i16),16832_i16,(-5400_i16),(-17057_i16)];
_13.0 = 3185088329943763434_u64 as f64;
_4 = _5;
RET.fld0 = Adt46 { fld0: _12.fld0.fld0,fld1: _12.fld0.fld1,fld2: _14.fld2 };
_15 = !false;
_16.0 = -(-7275712998396151529683508230653281942_i128);
_3 = [3373_i16,16106_i16,16252_i16,12573_i16,(-23185_i16)];
_7 = _5 << _14.fld1;
RET.fld0.fld2 = [84_u8,164_u8,55_u8,217_u8];
_16 = ((-36166153917075723138374747154828761746_i128), _12.fld0.fld0);
_13.1.1 = (-11864_i16) as i32;
_13.2 = _16.0 & _16.0;
RET.fld0.fld0 = -_16.1;
_17 = !_7;
_10 = _9;
RET.fld0.fld0 = -_12.fld0.fld0;
_14 = Move(RET.fld0);
_4 = _13.0 as isize;
_14 = Adt46 { fld0: _16.1,fld1: _12.fld0.fld1,fld2: _12.fld0.fld2 };
_12.fld0.fld2 = _14.fld2;
Goto(bb2)
}
bb2 = {
_13.2 = 121_i8 as i128;
_17 = !_7;
RET.fld0.fld2 = [102_u8,68_u8,184_u8,123_u8];
RET.fld0 = Move(_12.fld0);
_10 = [(-6400_i16),25598_i16,(-7576_i16),17315_i16,2967_i16];
_12.fld0.fld0 = _13.1.1 - _16.1;
RET.fld0.fld1 = _14.fld1 >> _14.fld1;
_14.fld1 = RET.fld0.fld1;
_11 = 306431233617429141364683698251328835644_u128 >> _14.fld1;
_14 = Adt46 { fld0: _13.1.1,fld1: RET.fld0.fld1,fld2: RET.fld0.fld2 };
_20.4 = _13.0 + _13.0;
_13.0 = _20.4 - _20.4;
RET.fld0.fld1 = _14.fld1;
_21.fld7 = core::ptr::addr_of!(_16.1);
_20.4 = _13.0 * _13.0;
_13.1.1 = _14.fld0 ^ _16.1;
_13.1.0 = !_16.0;
_9 = _10;
RET.fld0.fld0 = _12.fld0.fld0 + _16.1;
_20.0 = _16.0 as usize;
_21.fld7 = core::ptr::addr_of!(_16.1);
_21.fld1 = core::ptr::addr_of_mut!(_20.1.1);
_13.0 = 38_i8 as f64;
_12.fld0.fld1 = _16.0 as u32;
_12.fld0 = Move(_14);
_20.1.0.1 = _16.1;
match _5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb3 = {
_6 = [(-19580_i16),(-16886_i16),16832_i16,(-5400_i16),(-17057_i16)];
_13.0 = 3185088329943763434_u64 as f64;
_4 = _5;
RET.fld0 = Adt46 { fld0: _12.fld0.fld0,fld1: _12.fld0.fld1,fld2: _14.fld2 };
_15 = !false;
_16.0 = -(-7275712998396151529683508230653281942_i128);
_3 = [3373_i16,16106_i16,16252_i16,12573_i16,(-23185_i16)];
_7 = _5 << _14.fld1;
RET.fld0.fld2 = [84_u8,164_u8,55_u8,217_u8];
_16 = ((-36166153917075723138374747154828761746_i128), _12.fld0.fld0);
_13.1.1 = (-11864_i16) as i32;
_13.2 = _16.0 & _16.0;
RET.fld0.fld0 = -_16.1;
_17 = !_7;
_10 = _9;
RET.fld0.fld0 = -_12.fld0.fld0;
_14 = Move(RET.fld0);
_4 = _13.0 as isize;
_14 = Adt46 { fld0: _16.1,fld1: _12.fld0.fld1,fld2: _12.fld0.fld2 };
_12.fld0.fld2 = _14.fld2;
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
_19 = _8;
RET.fld0.fld2 = _12.fld0.fld2;
_22 = (_4, _15, _12.fld0.fld1);
_14.fld2 = RET.fld0.fld2;
_5 = _12.fld0.fld1 as isize;
_21.fld4 = 14681257767480456843_u64;
_4 = _5 & _19;
Goto(bb9)
}
bb9 = {
RET.fld0.fld1 = _22.2;
_12.fld0.fld2 = RET.fld0.fld2;
_23 = '\u{d776d}' as isize;
_19 = !_5;
RET.fld0.fld2 = [41_u8,60_u8,237_u8,126_u8];
RET.fld0.fld1 = !_22.2;
RET.fld0 = Adt46 { fld0: _20.1.0.1,fld1: _22.2,fld2: _12.fld0.fld2 };
_18 = 113_i8 as f64;
_16.1 = _13.1.1 + _20.1.0.1;
_10 = [(-106_i16),(-31395_i16),8459_i16,(-14474_i16),15868_i16];
Goto(bb10)
}
bb10 = {
_21.fld5 = _11;
_20.1.2 = 66_i8 << _4;
_13.0 = _20.4;
_21.fld7 = core::ptr::addr_of!(_12.fld0.fld0);
_20.1.0.0 = !_16.0;
_21.fld2 = [_20.1.0.0,_20.1.0.0,_20.1.0.0,_16.0];
RET.fld0 = Move(_12.fld0);
_14.fld1 = _16.0 as u32;
_20.1.0.1 = _16.1 - RET.fld0.fld0;
_21.fld3 = _21.fld4 as f32;
RET.fld0 = Adt46 { fld0: _13.1.1,fld1: _22.2,fld2: _14.fld2 };
_13.1.1 = _20.1.0.1 + _20.1.0.1;
match _16.0 {
304116213003862740324999860276939449710 => bb12,
_ => bb11
}
}
bb11 = {
_6 = [(-19580_i16),(-16886_i16),16832_i16,(-5400_i16),(-17057_i16)];
_13.0 = 3185088329943763434_u64 as f64;
_4 = _5;
RET.fld0 = Adt46 { fld0: _12.fld0.fld0,fld1: _12.fld0.fld1,fld2: _14.fld2 };
_15 = !false;
_16.0 = -(-7275712998396151529683508230653281942_i128);
_3 = [3373_i16,16106_i16,16252_i16,12573_i16,(-23185_i16)];
_7 = _5 << _14.fld1;
RET.fld0.fld2 = [84_u8,164_u8,55_u8,217_u8];
_16 = ((-36166153917075723138374747154828761746_i128), _12.fld0.fld0);
_13.1.1 = (-11864_i16) as i32;
_13.2 = _16.0 & _16.0;
RET.fld0.fld0 = -_16.1;
_17 = !_7;
_10 = _9;
RET.fld0.fld0 = -_12.fld0.fld0;
_14 = Move(RET.fld0);
_4 = _13.0 as isize;
_14 = Adt46 { fld0: _16.1,fld1: _12.fld0.fld1,fld2: _12.fld0.fld2 };
_12.fld0.fld2 = _14.fld2;
Goto(bb2)
}
bb12 = {
RET.fld0.fld1 = _22.2;
_26.0 = core::ptr::addr_of_mut!(_20.1.1);
_7 = -_19;
_12.fld0.fld0 = 46983_u16 as i32;
_21.fld1 = core::ptr::addr_of_mut!(_20.1.1);
RET.fld0.fld0 = -_16.1;
_14.fld2 = [247_u8,142_u8,254_u8,102_u8];
_12.fld0.fld1 = _22.2 - RET.fld0.fld1;
_20.2 = _11;
_20.2 = !_21.fld5;
_20.0 = !7_usize;
_21.fld3 = _21.fld4 as f32;
_27 = _22.1;
_25 = 197_u8;
_14 = Move(RET.fld0);
_21.fld3 = _13.0 as f32;
_20.3 = !_21.fld5;
_1 = _9;
_20.3 = !_11;
_20.2 = _11 << _7;
_21.fld6 = 6412472857375002808_i64 ^ 5944023759430010863_i64;
_20.4 = _13.0 * _13.0;
_20.1.1 = [_16.0,_16.0,_16.0];
RET.fld0.fld1 = !_22.2;
_13.1 = (_16.0, _20.1.0.1);
RET.fld0.fld1 = _12.fld0.fld1;
_12.fld0 = Move(_14);
_28 = [_22.1,_27,_27,_15,_22.1,_22.1,_15];
_14.fld2 = _12.fld0.fld2;
_13.1.0 = _16.0 | _16.0;
_20.4 = _13.0;
Goto(bb13)
}
bb13 = {
_14 = Adt46 { fld0: _13.1.1,fld1: _22.2,fld2: _12.fld0.fld2 };
_20.1.0.0 = _13.2 | _13.1.0;
RET.fld0 = Move(_14);
_21.fld1 = core::ptr::addr_of_mut!(_20.1.1);
_16.0 = -_13.2;
_20.1.0 = _16;
_12.fld0.fld0 = 15507_u16 as i32;
_15 = _25 == _25;
_7 = -_4;
_21.fld6 = _13.1.0 as i64;
_26 = (_21.fld1,);
_31.1 = -RET.fld0.fld0;
_21.fld0 = _26;
_14.fld1 = _20.4 as u32;
_2 = _3;
_12.fld0 = Adt46 { fld0: _13.1.1,fld1: _14.fld1,fld2: RET.fld0.fld2 };
_21.fld4 = 17254078584773597094_u64 >> _4;
Call(_32.fld2 = core::intrinsics::bswap(_19), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_18 = _13.0 + _20.4;
_30 = [_16.0,_16.0,_13.1.0,_13.1.0];
_14.fld2 = RET.fld0.fld2;
_34.1.2 = _7 as i8;
_34.0 = _20.0 - _20.0;
_25 = 142_u8 & 243_u8;
_16.0 = _13.1.0;
_11 = _21.fld5;
_5 = _13.0 as isize;
_21.fld7 = core::ptr::addr_of!(RET.fld0.fld0);
RET.fld0.fld1 = !_22.2;
_17 = _15 as isize;
_12.fld0 = Adt46 { fld0: RET.fld0.fld0,fld1: RET.fld0.fld1,fld2: _14.fld2 };
Call(RET.fld0.fld2 = fn8(_19, _20.1.2, _7, _21.fld4, Move(_21), Move(_12.fld0), _20, _13.0, _14.fld1, _22.2, _4, _4, _20.1.2, _19, _20.1.0.1), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_20.1.1 = [_20.1.0.0,_20.1.0.0,_13.1.0];
_32.fld1 = -_13.0;
_21.fld2 = [_16.0,_13.2,_16.0,_16.0];
Call(_21.fld0 = fn11(_18, _9, _20.3, _20.1.0, _11, _7, _13.0, _7, Move(RET.fld0), _6, _4, _20.4, _6, _4), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_27 = _18 != _13.0;
_20.1.1 = [_16.0,_13.1.0,_16.0];
_22.1 = !_27;
_26.0 = core::ptr::addr_of_mut!(_34.1.1);
_34.1.1 = [_16.0,_20.1.0.0,_16.0];
RET.fld0 = Adt46 { fld0: _31.1,fld1: _22.2,fld2: _14.fld2 };
_12.fld0.fld0 = 64226_u16 as i32;
_12.fld0.fld1 = 46738_u16 as u32;
_21.fld6 = (-3265_i16) as i64;
_21.fld7 = core::ptr::addr_of!(_13.1.1);
RET.fld0 = Adt46 { fld0: _13.1.1,fld1: _22.2,fld2: _14.fld2 };
_22.2 = RET.fld0.fld1;
_12.fld0.fld0 = _20.1.2 as i32;
_37.fld2.1 = (_18, _16, _13.1.0);
_7 = _4 + _19;
_13.1 = (_37.fld2.1.1.0, _31.1);
_37.fld4 = Adt45::Variant1 { fld0: _2,fld1: _20.1,fld2: _30,fld3: _34.1.2,fld4: _20.1.1 };
_20.1.0.1 = Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.1 << Field::<i8>(Variant(_37.fld4, 1), 3);
_32.fld0 = [_16.0,Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.0,Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.0];
_21.fld4 = _13.1.0 as u64;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).0.1 = _21.fld4 as i32;
_16 = _20.1.0;
_15 = _27;
Call(_34.4 = core::intrinsics::fmaf64(_37.fld2.1.0, _13.0, _37.fld2.1.0), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
RET.fld0.fld1 = !_22.2;
RET.fld0.fld1 = !_22.2;
SetDiscriminant(_37.fld4, 0);
_37.fld2.3 = !_12.fld0.fld0;
_21.fld0 = (_26.0,);
_9 = [28253_i16,(-23654_i16),(-31050_i16),(-27910_i16),24143_i16];
_29 = !_20.2;
_11 = _22.2 as u128;
_21.fld0.0 = core::ptr::addr_of_mut!(_34.1.1);
_33 = -_20.1.2;
_37.fld2.1.1.1 = _12.fld0.fld0 >> _20.1.0.1;
_33 = -_20.1.2;
_37.fld2.4.1 = core::ptr::addr_of!(_14.fld0);
_34.4 = 27956_i16 as f64;
_13.1 = (_13.2, _37.fld2.1.1.1);
_14.fld2 = RET.fld0.fld2;
_37.fld1 = _25 | _25;
_21.fld0.0 = _26.0;
_15 = !_27;
_37.fld2.4.0 = !_14.fld1;
_32.fld2 = _5 * _7;
_13.0 = -_32.fld1;
_16 = (_20.1.0.0, _20.1.0.1);
Goto(bb18)
}
bb18 = {
_36 = _13.1.1 <= _20.1.0.1;
_22.2 = !_12.fld0.fld1;
Call(_20.1.2 = core::intrinsics::transmute(_36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_44.1 = _20.0 | _20.0;
_31.0 = _16.0 | _37.fld2.1.1.0;
_28 = [_36,_22.1,_22.1,_15,_15,_27,_36];
_20.1.1 = _34.1.1;
Goto(bb20)
}
bb20 = {
_20.2 = _7 as u128;
_20.1.0.1 = _31.1;
_13.2 = -_37.fld2.1.1.0;
_21.fld2 = [_31.0,_37.fld2.1.2,_20.1.0.0,_31.0];
_45.0 = _32.fld1 + _18;
_45.2 = !_13.2;
_44.4 = _37.fld2.1.1;
RET.fld0 = Adt46 { fld0: _13.1.1,fld1: _37.fld2.4.0,fld2: _14.fld2 };
_34.0 = _44.1;
Call(_45.1.0 = core::intrinsics::transmute(_29), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_20.1.0.1 = _13.1.1;
_44.0 = _7 & _7;
place!(Field::<[i128; 4]>(Variant(_37.fld4, 0), 1)) = [_44.4.0,_37.fld2.1.1.0,_45.1.0,_37.fld2.1.1.0];
_37.fld2.1.2 = _20.3 as i128;
_34.3 = _37.fld2.1.0 as u128;
_49 = _14.fld1 as f32;
_44.5 = (_32.fld1, _16, _44.4.0);
_37.fld4 = Adt45::Variant1 { fld0: _6,fld1: _20.1,fld2: _21.fld2,fld3: _20.1.2,fld4: _20.1.1 };
_12.fld0 = Adt46 { fld0: Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.1,fld1: _37.fld2.4.0,fld2: _14.fld2 };
_34.0 = !_20.0;
_34.0 = _44.1 | _44.1;
_51.0.0 = !_45.1.0;
_37.fld3 = core::ptr::addr_of!(_25);
_21.fld6 = (-4815025610480190487_i64);
RET.fld0.fld1 = _14.fld1 ^ _12.fld0.fld1;
_30 = _21.fld2;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).1 = [_37.fld2.1.2,_45.1.0,_51.0.0];
_13.1 = (_45.1.0, _37.fld2.3);
RET.fld0.fld0 = _37.fld2.1.1.1 | _44.4.1;
_34 = (_44.1, Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1), _29, _29, _45.0);
_44.5 = (_18, _44.4, _37.fld2.1.2);
_14.fld0 = _34.1.0.1 | _20.1.0.1;
match _8 {
0 => bb7,
9223372036854775807 => bb22,
_ => bb15
}
}
bb22 = {
RET.fld0 = Adt46 { fld0: _20.1.0.1,fld1: _22.2,fld2: _14.fld2 };
_44.4.1 = _34.1.0.1;
_51.0.1 = !_44.5.1.1;
_45.1.1 = -_14.fld0;
_41 = _4 as f64;
_19 = -_32.fld2;
_34.4 = _44.5.0 + _45.0;
_51 = (_13.1, _34.1.1, _20.1.2);
_2 = [(-5591_i16),(-5969_i16),(-14817_i16),(-14593_i16),(-10141_i16)];
_22.1 = !_36;
_44.0 = !_7;
Goto(bb23)
}
bb23 = {
_10 = [(-1981_i16),25289_i16,12365_i16,5579_i16,3596_i16];
_37.fld2.0 = _22.1;
_12.fld0.fld1 = _44.4.1 as u32;
_38 = _32.fld2;
_18 = -_37.fld2.1.0;
_14.fld2 = [_37.fld1,_37.fld1,_25,_37.fld1];
_26 = (_21.fld0.0,);
_21.fld5 = _20.2 | _34.2;
_22 = (_32.fld2, _36, _12.fld0.fld1);
place!(Field::<[i16; 5]>(Variant(_37.fld4, 1), 0)) = [6491_i16,(-31819_i16),(-5560_i16),(-14962_i16),8140_i16];
Goto(bb24)
}
bb24 = {
_20 = _34;
_54.0 = _37.fld2.4.0 as isize;
_54.2 = _37.fld2.4.0 << _13.1.1;
_34.0 = _49 as usize;
_32.fld2 = -_22.0;
_7 = _5;
_44.5.1.1 = _21.fld6 as i32;
_46 = _21.fld0.0;
_21.fld5 = _25 as u128;
_34.4 = _18 - _45.0;
_27 = _22.1;
_43 = _2;
_44.5.1 = (_37.fld2.1.2, _12.fld0.fld0);
_37.fld2.1.2 = -_44.4.0;
_44.5.0 = _41 * _45.0;
place!(Field::<[i16; 5]>(Variant(_37.fld4, 1), 0)) = [6856_i16,(-20601_i16),(-20222_i16),(-1090_i16),30002_i16];
(*_46) = [_13.1.0,_51.0.0,_51.0.0];
_21.fld3 = -_49;
_9 = [(-32605_i16),3598_i16,(-21986_i16),4041_i16,22910_i16];
_44.4 = _45.1;
_58 = _28;
_34.1.2 = _37.fld1 as i8;
_21.fld6 = 3057902356051822730_i64 * 2807702861515985400_i64;
_34.0 = _44.1;
Goto(bb25)
}
bb25 = {
_34.1.0.1 = _20.1.0.1 >> _20.1.2;
SetDiscriminant(_37.fld4, 1);
_34.1.0 = _45.1;
_20 = _34;
_36 = _20.4 != _34.4;
RET.fld0.fld2 = [_37.fld1,_37.fld1,_37.fld1,_37.fld1];
_44.4 = (_45.1.0, _14.fld0);
_59.5.1.0 = _51.2 as i128;
_52.0 = !_4;
_37.fld2.1.2 = -_13.1.0;
_22.2 = !_12.fld0.fld1;
match _8 {
0 => bb26,
9223372036854775807 => bb28,
_ => bb27
}
}
bb26 = {
RET.fld0.fld1 = !_22.2;
RET.fld0.fld1 = !_22.2;
SetDiscriminant(_37.fld4, 0);
_37.fld2.3 = !_12.fld0.fld0;
_21.fld0 = (_26.0,);
_9 = [28253_i16,(-23654_i16),(-31050_i16),(-27910_i16),24143_i16];
_29 = !_20.2;
_11 = _22.2 as u128;
_21.fld0.0 = core::ptr::addr_of_mut!(_34.1.1);
_33 = -_20.1.2;
_37.fld2.1.1.1 = _12.fld0.fld0 >> _20.1.0.1;
_33 = -_20.1.2;
_37.fld2.4.1 = core::ptr::addr_of!(_14.fld0);
_34.4 = 27956_i16 as f64;
_13.1 = (_13.2, _37.fld2.1.1.1);
_14.fld2 = RET.fld0.fld2;
_37.fld1 = _25 | _25;
_21.fld0.0 = _26.0;
_15 = !_27;
_37.fld2.4.0 = !_14.fld1;
_32.fld2 = _5 * _7;
_13.0 = -_32.fld1;
_16 = (_20.1.0.0, _20.1.0.1);
Goto(bb18)
}
bb27 = {
_13.2 = 121_i8 as i128;
_17 = !_7;
RET.fld0.fld2 = [102_u8,68_u8,184_u8,123_u8];
RET.fld0 = Move(_12.fld0);
_10 = [(-6400_i16),25598_i16,(-7576_i16),17315_i16,2967_i16];
_12.fld0.fld0 = _13.1.1 - _16.1;
RET.fld0.fld1 = _14.fld1 >> _14.fld1;
_14.fld1 = RET.fld0.fld1;
_11 = 306431233617429141364683698251328835644_u128 >> _14.fld1;
_14 = Adt46 { fld0: _13.1.1,fld1: RET.fld0.fld1,fld2: RET.fld0.fld2 };
_20.4 = _13.0 + _13.0;
_13.0 = _20.4 - _20.4;
RET.fld0.fld1 = _14.fld1;
_21.fld7 = core::ptr::addr_of!(_16.1);
_20.4 = _13.0 * _13.0;
_13.1.1 = _14.fld0 ^ _16.1;
_13.1.0 = !_16.0;
_9 = _10;
RET.fld0.fld0 = _12.fld0.fld0 + _16.1;
_20.0 = _16.0 as usize;
_21.fld7 = core::ptr::addr_of!(_16.1);
_21.fld1 = core::ptr::addr_of_mut!(_20.1.1);
_13.0 = 38_i8 as f64;
_12.fld0.fld1 = _16.0 as u32;
_12.fld0 = Move(_14);
_20.1.0.1 = _16.1;
match _5 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb28 = {
_37.fld0 = _36;
_13.0 = _29 as f64;
_20.0 = '\u{6d944}' as usize;
_59.4.1 = _44.5.1.1;
_52.2 = _22.2;
_12.fld0.fld1 = _37.fld2.4.0;
_37.fld2.1.1 = _45.1;
match _8 {
9223372036854775807 => bb30,
_ => bb29
}
}
bb29 = {
_20.1.0.1 = _13.1.1;
_44.0 = _7 & _7;
place!(Field::<[i128; 4]>(Variant(_37.fld4, 0), 1)) = [_44.4.0,_37.fld2.1.1.0,_45.1.0,_37.fld2.1.1.0];
_37.fld2.1.2 = _20.3 as i128;
_34.3 = _37.fld2.1.0 as u128;
_49 = _14.fld1 as f32;
_44.5 = (_32.fld1, _16, _44.4.0);
_37.fld4 = Adt45::Variant1 { fld0: _6,fld1: _20.1,fld2: _21.fld2,fld3: _20.1.2,fld4: _20.1.1 };
_12.fld0 = Adt46 { fld0: Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.1,fld1: _37.fld2.4.0,fld2: _14.fld2 };
_34.0 = !_20.0;
_34.0 = _44.1 | _44.1;
_51.0.0 = !_45.1.0;
_37.fld3 = core::ptr::addr_of!(_25);
_21.fld6 = (-4815025610480190487_i64);
RET.fld0.fld1 = _14.fld1 ^ _12.fld0.fld1;
_30 = _21.fld2;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).1 = [_37.fld2.1.2,_45.1.0,_51.0.0];
_13.1 = (_45.1.0, _37.fld2.3);
RET.fld0.fld0 = _37.fld2.1.1.1 | _44.4.1;
_34 = (_44.1, Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1), _29, _29, _45.0);
_44.5 = (_18, _44.4, _37.fld2.1.2);
_14.fld0 = _34.1.0.1 | _20.1.0.1;
match _8 {
0 => bb7,
9223372036854775807 => bb22,
_ => bb15
}
}
bb30 = {
_37.fld2.1 = (_20.4, _44.5.1, _59.5.1.0);
_59.4.1 = _37.fld2.0 as i32;
_32.fld4 = Adt54::Variant3 { fld0: _21.fld6,fld1: (*_46) };
_1 = [32458_i16,(-31439_i16),4687_i16,628_i16,19703_i16];
match _8 {
0 => bb19,
9223372036854775807 => bb31,
_ => bb29
}
}
bb31 = {
_37.fld2.1 = (_34.4, _20.1.0, _20.1.0.0);
_59.2 = core::ptr::addr_of!(_13.1.1);
Goto(bb32)
}
bb32 = {
_17 = _22.0;
_22.2 = !_54.2;
_3 = [155_i16,13905_i16,(-10514_i16),(-3162_i16),(-18981_i16)];
_37.fld2.3 = _44.5.1.1;
_14 = Adt46 { fld0: _37.fld2.3,fld1: _52.2,fld2: RET.fld0.fld2 };
_34.1.0.0 = -_44.4.0;
_59.5 = (_37.fld2.1.0, _20.1.0, _37.fld2.1.2);
place!(Field::<i8>(Variant(_37.fld4, 1), 3)) = _33 * _33;
_16.0 = _51.0.0;
_54.1 = !_37.fld0;
_44.3 = _37.fld1 | _37.fld1;
_37.fld2.4.0 = !_14.fld1;
_55.fld1 = _26.0;
Goto(bb33)
}
bb33 = {
_61 = (_37.fld2.1.0, _51.0, _44.4.0);
_61.1 = _34.1.0;
_15 = !_37.fld2.0;
_13.1.0 = _45.2;
_59.4.0 = !_20.1.0.0;
_59.0 = _44.0 >> _17;
Goto(bb34)
}
bb34 = {
_20.1.0 = (_61.2, _13.1.1);
_37.fld2.1.0 = _18 * _44.5.0;
_59.5 = _37.fld2.1;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).0 = (_37.fld2.1.2, _44.4.1);
_55.fld4 = !_21.fld4;
_37.fld2.1.0 = -_41;
place!(Field::<i64>(Variant(_32.fld4, 3), 0)) = -_21.fld6;
_4 = -_52.0;
_21.fld4 = _55.fld4;
_55.fld6 = _51.0.0 as i64;
_12.fld0.fld2 = [_44.3,_44.3,_37.fld1,_44.3];
_55.fld5 = _54.2 as u128;
_55.fld0 = (_21.fld0.0,);
_53 = [_37.fld1,_37.fld1,_44.3,_37.fld1];
_47 = _34.0 & _44.1;
SetDiscriminant(_32.fld4, 1);
_20.1.1 = (*_46);
_20.3 = _44.4.1 as u128;
_39 = _4 - _59.0;
place!(Field::<Adt50>(Variant(_32.fld4, 1), 6)).fld0 = Adt46 { fld0: _45.1.1,fld1: _37.fld2.4.0,fld2: _53 };
_62.1.1 = _45.1.1 << _37.fld2.1.1.1;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)) = (_51.0, _51.1, _33);
_61.2 = -_59.5.2;
_61 = _59.5;
_31.1 = _55.fld4 as i32;
match _8 {
9223372036854775807 => bb35,
_ => bb28
}
}
bb35 = {
_37.fld2.3 = _14.fld0;
_27 = _36;
_30 = _21.fld2;
match _8 {
0 => bb13,
9223372036854775807 => bb37,
_ => bb36
}
}
bb36 = {
_10 = [(-1981_i16),25289_i16,12365_i16,5579_i16,3596_i16];
_37.fld2.0 = _22.1;
_12.fld0.fld1 = _44.4.1 as u32;
_38 = _32.fld2;
_18 = -_37.fld2.1.0;
_14.fld2 = [_37.fld1,_37.fld1,_25,_37.fld1];
_26 = (_21.fld0.0,);
_21.fld5 = _20.2 | _34.2;
_22 = (_32.fld2, _36, _12.fld0.fld1);
place!(Field::<[i16; 5]>(Variant(_37.fld4, 1), 0)) = [6491_i16,(-31819_i16),(-5560_i16),(-14962_i16),8140_i16];
Goto(bb24)
}
bb37 = {
_59.5.1.0 = _45.1.0 - _34.1.0.0;
match _8 {
0 => bb9,
1 => bb10,
2 => bb38,
3 => bb39,
4 => bb40,
5 => bb41,
9223372036854775807 => bb43,
_ => bb42
}
}
bb38 = {
Return()
}
bb39 = {
RET.fld0.fld1 = !_22.2;
RET.fld0.fld1 = !_22.2;
SetDiscriminant(_37.fld4, 0);
_37.fld2.3 = !_12.fld0.fld0;
_21.fld0 = (_26.0,);
_9 = [28253_i16,(-23654_i16),(-31050_i16),(-27910_i16),24143_i16];
_29 = !_20.2;
_11 = _22.2 as u128;
_21.fld0.0 = core::ptr::addr_of_mut!(_34.1.1);
_33 = -_20.1.2;
_37.fld2.1.1.1 = _12.fld0.fld0 >> _20.1.0.1;
_33 = -_20.1.2;
_37.fld2.4.1 = core::ptr::addr_of!(_14.fld0);
_34.4 = 27956_i16 as f64;
_13.1 = (_13.2, _37.fld2.1.1.1);
_14.fld2 = RET.fld0.fld2;
_37.fld1 = _25 | _25;
_21.fld0.0 = _26.0;
_15 = !_27;
_37.fld2.4.0 = !_14.fld1;
_32.fld2 = _5 * _7;
_13.0 = -_32.fld1;
_16 = (_20.1.0.0, _20.1.0.1);
Goto(bb18)
}
bb40 = {
_27 = _18 != _13.0;
_20.1.1 = [_16.0,_13.1.0,_16.0];
_22.1 = !_27;
_26.0 = core::ptr::addr_of_mut!(_34.1.1);
_34.1.1 = [_16.0,_20.1.0.0,_16.0];
RET.fld0 = Adt46 { fld0: _31.1,fld1: _22.2,fld2: _14.fld2 };
_12.fld0.fld0 = 64226_u16 as i32;
_12.fld0.fld1 = 46738_u16 as u32;
_21.fld6 = (-3265_i16) as i64;
_21.fld7 = core::ptr::addr_of!(_13.1.1);
RET.fld0 = Adt46 { fld0: _13.1.1,fld1: _22.2,fld2: _14.fld2 };
_22.2 = RET.fld0.fld1;
_12.fld0.fld0 = _20.1.2 as i32;
_37.fld2.1 = (_18, _16, _13.1.0);
_7 = _4 + _19;
_13.1 = (_37.fld2.1.1.0, _31.1);
_37.fld4 = Adt45::Variant1 { fld0: _2,fld1: _20.1,fld2: _30,fld3: _34.1.2,fld4: _20.1.1 };
_20.1.0.1 = Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.1 << Field::<i8>(Variant(_37.fld4, 1), 3);
_32.fld0 = [_16.0,Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.0,Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.0];
_21.fld4 = _13.1.0 as u64;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).0.1 = _21.fld4 as i32;
_16 = _20.1.0;
_15 = _27;
Call(_34.4 = core::intrinsics::fmaf64(_37.fld2.1.0, _13.0, _37.fld2.1.0), ReturnTo(bb17), UnwindUnreachable())
}
bb41 = {
Return()
}
bb42 = {
_20.2 = _7 as u128;
_20.1.0.1 = _31.1;
_13.2 = -_37.fld2.1.1.0;
_21.fld2 = [_31.0,_37.fld2.1.2,_20.1.0.0,_31.0];
_45.0 = _32.fld1 + _18;
_45.2 = !_13.2;
_44.4 = _37.fld2.1.1;
RET.fld0 = Adt46 { fld0: _13.1.1,fld1: _37.fld2.4.0,fld2: _14.fld2 };
_34.0 = _44.1;
Call(_45.1.0 = core::intrinsics::transmute(_29), ReturnTo(bb21), UnwindUnreachable())
}
bb43 = {
_44 = (_39, _47, _37.fld2.4.1, _37.fld1, _59.5.1, _45, _59.5.1.1);
_69.fld2 = [_25,_44.3,_44.3,_37.fld1];
_20.1 = _34.1;
_44.5.1 = (_59.5.2, _20.1.0.1);
_62.1 = (_51.0.0, _14.fld0);
_69 = Adt46 { fld0: _62.1.1,fld1: _14.fld1,fld2: _12.fld0.fld2 };
_59.6 = _21.fld3 as i32;
_34.1.0 = _44.5.1;
_21.fld5 = _52.2 as u128;
place!(Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1)).2 = _20.3;
_20.1 = _34.1;
_20.1.0.0 = _61.1.0 & _62.1.0;
RET.fld0 = Adt46 { fld0: _69.fld0,fld1: _37.fld2.4.0,fld2: _53 };
_21.fld5 = _34.2;
_33 = Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).2 << _44.4.1;
place!(Field::<i8>(Variant(_37.fld4, 1), 3)) = _34.1.2;
_68.fld0 = _62.1.1;
_13.1 = _20.1.0;
_72.1.1 = [_20.1.0.0,_16.0,_51.0.0];
_71.fld2 = [_25,_44.3,_44.3,_37.fld1];
match _8 {
0 => bb22,
1 => bb36,
2 => bb24,
3 => bb21,
4 => bb44,
9223372036854775807 => bb46,
_ => bb45
}
}
bb44 = {
_37.fld2.3 = _14.fld0;
_27 = _36;
_30 = _21.fld2;
match _8 {
0 => bb13,
9223372036854775807 => bb37,
_ => bb36
}
}
bb45 = {
_27 = _18 != _13.0;
_20.1.1 = [_16.0,_13.1.0,_16.0];
_22.1 = !_27;
_26.0 = core::ptr::addr_of_mut!(_34.1.1);
_34.1.1 = [_16.0,_20.1.0.0,_16.0];
RET.fld0 = Adt46 { fld0: _31.1,fld1: _22.2,fld2: _14.fld2 };
_12.fld0.fld0 = 64226_u16 as i32;
_12.fld0.fld1 = 46738_u16 as u32;
_21.fld6 = (-3265_i16) as i64;
_21.fld7 = core::ptr::addr_of!(_13.1.1);
RET.fld0 = Adt46 { fld0: _13.1.1,fld1: _22.2,fld2: _14.fld2 };
_22.2 = RET.fld0.fld1;
_12.fld0.fld0 = _20.1.2 as i32;
_37.fld2.1 = (_18, _16, _13.1.0);
_7 = _4 + _19;
_13.1 = (_37.fld2.1.1.0, _31.1);
_37.fld4 = Adt45::Variant1 { fld0: _2,fld1: _20.1,fld2: _30,fld3: _34.1.2,fld4: _20.1.1 };
_20.1.0.1 = Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.1 << Field::<i8>(Variant(_37.fld4, 1), 3);
_32.fld0 = [_16.0,Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.0,Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1).0.0];
_21.fld4 = _13.1.0 as u64;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).0.1 = _21.fld4 as i32;
_16 = _20.1.0;
_15 = _27;
Call(_34.4 = core::intrinsics::fmaf64(_37.fld2.1.0, _13.0, _37.fld2.1.0), ReturnTo(bb17), UnwindUnreachable())
}
bb46 = {
_71.fld1 = Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0.fld1 >> _33;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).2 = _20.1.2;
_55.fld3 = -_49;
_68.fld1 = !_69.fld1;
_37.fld4 = Adt45::Variant1 { fld0: _1,fld1: _34.1,fld2: _30,fld3: _33,fld4: _34.1.1 };
_34.1.2 = _33 >> _22.2;
_55.fld0 = _26;
_51.1 = [_59.5.2,_45.1.0,_51.0.0];
place!(Field::<*mut ((i128, i32), [i128; 3], i8)>(Variant(_32.fld4, 1), 5)) = core::ptr::addr_of_mut!(place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)));
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_37.fld4, 1), 1)).1 = (*_46);
RET.fld0.fld1 = !Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0.fld1;
_72 = (_47, _20.1, _11, _34.2, _20.4);
_33 = Field::<i8>(Variant(_37.fld4, 1), 3);
_56 = -_20.1.0.1;
_15 = _36;
_51.0.1 = _44.6 << _34.1.2;
_59.5.0 = -_34.4;
RET.fld0 = Adt46 { fld0: _59.4.1,fld1: _22.2,fld2: Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0.fld2 };
match _8 {
0 => bb43,
1 => bb21,
2 => bb28,
3 => bb5,
4 => bb47,
5 => bb48,
6 => bb49,
9223372036854775807 => bb51,
_ => bb50
}
}
bb47 = {
Return()
}
bb48 = {
_14 = Adt46 { fld0: _13.1.1,fld1: _22.2,fld2: _12.fld0.fld2 };
_20.1.0.0 = _13.2 | _13.1.0;
RET.fld0 = Move(_14);
_21.fld1 = core::ptr::addr_of_mut!(_20.1.1);
_16.0 = -_13.2;
_20.1.0 = _16;
_12.fld0.fld0 = 15507_u16 as i32;
_15 = _25 == _25;
_7 = -_4;
_21.fld6 = _13.1.0 as i64;
_26 = (_21.fld1,);
_31.1 = -RET.fld0.fld0;
_21.fld0 = _26;
_14.fld1 = _20.4 as u32;
_2 = _3;
_12.fld0 = Adt46 { fld0: _13.1.1,fld1: _14.fld1,fld2: RET.fld0.fld2 };
_21.fld4 = 17254078584773597094_u64 >> _4;
Call(_32.fld2 = core::intrinsics::bswap(_19), ReturnTo(bb14), UnwindUnreachable())
}
bb49 = {
_34.1.0.1 = _20.1.0.1 >> _20.1.2;
SetDiscriminant(_37.fld4, 1);
_34.1.0 = _45.1;
_20 = _34;
_36 = _20.4 != _34.4;
RET.fld0.fld2 = [_37.fld1,_37.fld1,_37.fld1,_37.fld1];
_44.4 = (_45.1.0, _14.fld0);
_59.5.1.0 = _51.2 as i128;
_52.0 = !_4;
_37.fld2.1.2 = -_13.1.0;
_22.2 = !_12.fld0.fld1;
match _8 {
0 => bb26,
9223372036854775807 => bb28,
_ => bb27
}
}
bb50 = {
_6 = [(-19580_i16),(-16886_i16),16832_i16,(-5400_i16),(-17057_i16)];
_13.0 = 3185088329943763434_u64 as f64;
_4 = _5;
RET.fld0 = Adt46 { fld0: _12.fld0.fld0,fld1: _12.fld0.fld1,fld2: _14.fld2 };
_15 = !false;
_16.0 = -(-7275712998396151529683508230653281942_i128);
_3 = [3373_i16,16106_i16,16252_i16,12573_i16,(-23185_i16)];
_7 = _5 << _14.fld1;
RET.fld0.fld2 = [84_u8,164_u8,55_u8,217_u8];
_16 = ((-36166153917075723138374747154828761746_i128), _12.fld0.fld0);
_13.1.1 = (-11864_i16) as i32;
_13.2 = _16.0 & _16.0;
RET.fld0.fld0 = -_16.1;
_17 = !_7;
_10 = _9;
RET.fld0.fld0 = -_12.fld0.fld0;
_14 = Move(RET.fld0);
_4 = _13.0 as isize;
_14 = Adt46 { fld0: _16.1,fld1: _12.fld0.fld1,fld2: _12.fld0.fld2 };
_12.fld0.fld2 = _14.fld2;
Goto(bb2)
}
bb51 = {
_9 = _6;
SetDiscriminant(_37.fld4, 0);
match _8 {
0 => bb19,
1 => bb40,
2 => bb45,
3 => bb4,
4 => bb49,
5 => bb6,
6 => bb43,
9223372036854775807 => bb52,
_ => bb48
}
}
bb52 = {
_69.fld1 = _71.fld1 + _54.2;
_55.fld7 = _44.2;
RET.fld0.fld1 = Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0.fld1;
_34.4 = _20.4 - _72.4;
place!(Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1)).1.2 = -_51.2;
_21.fld0 = (_55.fld0.0,);
place!(Field::<[i128; 4]>(Variant(_37.fld4, 0), 1)) = _21.fld2;
_55.fld5 = 6367_u16 as u128;
place!(Field::<((i128, i32), [i128; 3], i8)>(Variant(_32.fld4, 1), 2)) = (_44.5.1, _20.1.1, _20.1.2);
_14.fld1 = _69.fld1 ^ _22.2;
RET.fld0 = Adt46 { fld0: Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0.fld0,fld1: _69.fld1,fld2: _12.fld0.fld2 };
place!(Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1)).4 = _61.0;
place!(Field::<Adt50>(Variant(_32.fld4, 1), 6)).fld0 = Adt46 { fld0: _56,fld1: _52.2,fld2: _14.fld2 };
_37.fld2.4.2 = core::ptr::addr_of!(_76);
_37.fld2.4.2 = core::ptr::addr_of!(_76);
place!(Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1)) = (_72.0, _72.1, _29, _34.2, _59.5.0);
_26.0 = core::ptr::addr_of_mut!((*_46));
_69 = Adt46 { fld0: _13.1.1,fld1: _68.fld1,fld2: Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0.fld2 };
Goto(bb53)
}
bb53 = {
_11 = _20.3;
_68.fld2 = [_37.fld1,_25,_25,_25];
_21.fld1 = _46;
_59 = _44;
RET.fld0.fld0 = _51.0.1;
_12.fld0.fld0 = _44.5.1.1 | _44.6;
_45.1.1 = Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1).1.0.1 - _69.fld0;
_44 = (_54.0, _47, _21.fld7, _25, _37.fld2.1.1, _37.fld2.1, _34.1.0.1);
_52 = (_39, _15, _14.fld1);
match _8 {
0 => bb32,
1 => bb52,
2 => bb15,
3 => bb16,
4 => bb48,
5 => bb54,
9223372036854775807 => bb56,
_ => bb55
}
}
bb54 = {
Return()
}
bb55 = {
_37.fld2.1 = (_20.4, _44.5.1, _59.5.1.0);
_59.4.1 = _37.fld2.0 as i32;
_32.fld4 = Adt54::Variant3 { fld0: _21.fld6,fld1: (*_46) };
_1 = [32458_i16,(-31439_i16),4687_i16,628_i16,19703_i16];
match _8 {
0 => bb19,
9223372036854775807 => bb31,
_ => bb29
}
}
bb56 = {
_21.fld4 = _55.fld4;
_80 = core::ptr::addr_of!(RET.fld0.fld0);
place!(Field::<f64>(Variant(_32.fld4, 1), 0)) = _25 as f64;
_8 = _59.0;
_76 = 25516_u16 ^ 8646_u16;
_59.4.1 = _45.1.1;
_49 = _37.fld1 as f32;
_89 = [_37.fld1,_59.3,_25,_59.3];
_55.fld6 = _54.2 as i64;
_31 = _34.1.0;
_37.fld2.2 = [(-23298_i16),11612_i16,(-25935_i16),12757_i16,2403_i16];
_92.0 = core::ptr::addr_of_mut!(_34.1.1);
_37.fld4 = Adt45::Variant1 { fld0: _9,fld1: _72.1,fld2: _21.fld2,fld3: _51.2,fld4: _72.1.1 };
_92 = _55.fld0;
Goto(bb57)
}
bb57 = {
_22.1 = _36 & _36;
_80 = _59.2;
_78.0.1 = -_20.1.0.1;
_59.5 = (_72.4, Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1).1.0, _61.1.0);
_50 = _21.fld3;
_19 = _59.0 << _34.1.0.0;
_88 = !_51.0.1;
_72.3 = _44.5.1.0 as u128;
_92 = (_26.0,);
_63 = _20.4;
_75 = !_54.1;
_54.0 = _8 - _38;
_94 = Field::<(usize, ((i128, i32), [i128; 3], i8), u128, u128, f64)>(Variant(_32.fld4, 1), 1).1;
RET.fld1 = Adt49::Variant1 { fld0: _22,fld1: Move(Field::<Adt50>(Variant(_32.fld4, 1), 6).fld0),fld2: _37.fld2,fld3: _45,fld4: 17234_i16,fld5: _76 };
SetDiscriminant(_37.fld4, 1);
place!(Field::<i16>(Variant(RET.fld1, 1), 4)) = (-14784_i16);
Goto(bb58)
}
bb58 = {
Call(_96 = dump_var(7_usize, 43_usize, Move(_43), 8_usize, Move(_8), 15_usize, Move(_15), 29_usize, Move(_29)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_96 = dump_var(7_usize, 5_usize, Move(_5), 89_usize, Move(_89), 51_usize, Move(_51), 94_usize, Move(_94)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_96 = dump_var(7_usize, 58_usize, Move(_58), 17_usize, Move(_17), 9_usize, Move(_9), 54_usize, Move(_54)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_96 = dump_var(7_usize, 38_usize, Move(_38), 2_usize, Move(_2), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_96 = dump_var(7_usize, 4_usize, Move(_4), 75_usize, Move(_75), 28_usize, Move(_28), 27_usize, Move(_27)), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: i8,mut _3: isize,mut _4: u64,mut _5: Adt44,mut _6: Adt46,mut _7: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64),mut _8: f64,mut _9: u32,mut _10: u32,mut _11: isize,mut _12: isize,mut _13: i8,mut _14: isize,mut _15: i32) -> [u8; 4] {
mir! {
type RET = [u8; 4];
let _16: u16;
let _17: u64;
let _18: Adt52;
let _19: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16));
let _20: isize;
let _21: i64;
let _22: u32;
let _23: [bool; 7];
let _24: f32;
let _25: char;
let _26: isize;
let _27: u128;
let _28: Adt49;
let _29: [u8; 4];
let _30: [bool; 7];
let _31: *mut &'static f64;
let _32: *const u8;
let _33: Adt48;
let _34: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _35: Adt44;
let _36: *const (f64, (i128, i32), i128);
let _37: [i32; 4];
let _38: u64;
let _39: [i16; 5];
let _40: i16;
let _41: Adt55;
let _42: ((i128, i32), [i128; 3], i8);
let _43: (i128, i32);
let _44: ((i128, i32), [i128; 3], i8);
let _45: (f64, (i128, i32), i128);
let _46: [i32; 4];
let _47: i128;
let _48: isize;
let _49: u64;
let _50: isize;
let _51: isize;
let _52: [i32; 4];
let _53: isize;
let _54: f64;
let _55: Adt46;
let _56: u16;
let _57: ();
let _58: ();
{
_7.1.0.1 = 32165_i16 as i32;
_11 = 28_u8 as isize;
_7.1.0.1 = _6.fld0 + _15;
_7.1.0.0 = _8 as i128;
_7.2 = '\u{f8fa0}' as u128;
_7.3 = (-2685_i16) as u128;
RET = [128_u8,121_u8,124_u8,226_u8];
_4 = _5.fld4;
RET = [62_u8,251_u8,192_u8,47_u8];
_7.1.0 = ((-43890287280805204464870628481961176538_i128), _15);
_7.2 = _7.1.2 as u128;
_7.2 = !_5.fld5;
_1 = _3;
_13 = !_2;
_6.fld0 = _7.1.0.1 * _15;
_7.2 = _5.fld5;
_7.1.0.0 = !108867343936799564799758935091402124117_i128;
_5.fld3 = (-26887_i16) as f32;
_13 = _7.1.2;
_12 = _3 << _6.fld1;
_5.fld0.0 = core::ptr::addr_of_mut!(_7.1.1);
_17 = _4;
_5.fld0 = (_5.fld1,);
_14 = !_1;
RET = [107_u8,22_u8,13_u8,197_u8];
Goto(bb1)
}
bb1 = {
_7.1.1 = [_7.1.0.0,_7.1.0.0,_7.1.0.0];
_6.fld1 = !_10;
_4 = false as u64;
_6.fld2 = [99_u8,70_u8,107_u8,94_u8];
_5.fld5 = _7.2 & _7.2;
_7.1.0.0 = _5.fld3 as i128;
Goto(bb2)
}
bb2 = {
_7.1.1 = [_7.1.0.0,_7.1.0.0,_7.1.0.0];
_7.1.0.0 = 98018801322896897954440110952162089256_i128 - 1774583836456414907467599347265452680_i128;
_7.0 = 7_usize;
_5.fld2 = [_7.1.0.0,_7.1.0.0,_7.1.0.0,_7.1.0.0];
_5.fld0.0 = core::ptr::addr_of_mut!(_7.1.1);
_5.fld7 = core::ptr::addr_of!(_15);
_7.4 = _8;
_5.fld0 = (_5.fld1,);
_5.fld0.0 = _5.fld1;
_2 = _7.1.2 ^ _13;
_6.fld2 = RET;
_5.fld5 = !_7.2;
_13 = _3 as i8;
_6 = Adt46 { fld0: _15,fld1: _9,fld2: RET };
_12 = _7.4 as isize;
_10 = !_9;
RET = [76_u8,60_u8,181_u8,219_u8];
_14 = _3;
_5.fld1 = _5.fld0.0;
_14 = _8 as isize;
_5.fld1 = _5.fld0.0;
_5.fld4 = _17;
_3 = 64907_u16 as isize;
_16 = 44282_u16;
Goto(bb3)
}
bb3 = {
_11 = _1;
_5.fld6 = 717428551356728282_i64;
_7.1.0.0 = _5.fld5 as i128;
_14 = _11 + _11;
_6.fld2 = [110_u8,159_u8,120_u8,3_u8];
_3 = _1 >> _7.1.0.0;
_7.0 = 1762808977319556147_usize + 7_usize;
_3 = _1 * _14;
Goto(bb4)
}
bb4 = {
_7.1.0.1 = -_15;
_7.1.2 = !_2;
_7.1.1 = [_7.1.0.0,_7.1.0.0,_7.1.0.0];
_7.2 = _5.fld5 >> _1;
_2 = _7.1.2;
_19.3 = _7.1.0.1 >> _7.2;
_5.fld0 = (_5.fld1,);
_6.fld0 = _1 as i32;
_19.1.2 = _7.1.2 as i128;
_5.fld3 = 173_u8 as f32;
_19.3 = -_15;
_19.4.0 = _10 - _9;
_20 = _1 << _7.2;
_19.1 = (_8, _7.1.0, _7.1.0.0);
_7.4 = _8;
_6.fld1 = _9 ^ _19.4.0;
_19.4.1 = _5.fld7;
_5.fld0 = (_5.fld1,);
_22 = !_10;
_9 = _6.fld1 * _19.4.0;
RET = [73_u8,115_u8,185_u8,157_u8];
_14 = _20 * _3;
_6.fld2 = [164_u8,29_u8,146_u8,65_u8];
_19.1 = (_8, _7.1.0, _7.1.0.0);
_6 = Adt46 { fld0: _7.1.0.1,fld1: _9,fld2: RET };
Call(_8 = core::intrinsics::transmute(_14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6.fld1 = !_9;
_6.fld1 = !_19.4.0;
_6.fld0 = _3 as i32;
Goto(bb6)
}
bb6 = {
_19.1.1 = (_7.1.0.0, _6.fld0);
_19.2 = [(-6500_i16),1941_i16,18191_i16,15048_i16,(-13781_i16)];
_6.fld0 = _5.fld3 as i32;
_19.4.2 = core::ptr::addr_of!(_16);
_6.fld0 = !_19.1.1.1;
_4 = _17 * _17;
_23 = [true,true,true,false,true,true,true];
_19.1.0 = _8;
_5.fld0 = (_5.fld1,);
_7.1.0 = (_19.1.2, _19.3);
_5.fld1 = core::ptr::addr_of_mut!(_7.1.1);
_5.fld0 = (_5.fld1,);
match _16 {
0 => bb2,
1 => bb7,
44282 => bb9,
_ => bb8
}
}
bb7 = {
_11 = _1;
_5.fld6 = 717428551356728282_i64;
_7.1.0.0 = _5.fld5 as i128;
_14 = _11 + _11;
_6.fld2 = [110_u8,159_u8,120_u8,3_u8];
_3 = _1 >> _7.1.0.0;
_7.0 = 1762808977319556147_usize + 7_usize;
_3 = _1 * _14;
Goto(bb4)
}
bb8 = {
_7.1.0.1 = -_15;
_7.1.2 = !_2;
_7.1.1 = [_7.1.0.0,_7.1.0.0,_7.1.0.0];
_7.2 = _5.fld5 >> _1;
_2 = _7.1.2;
_19.3 = _7.1.0.1 >> _7.2;
_5.fld0 = (_5.fld1,);
_6.fld0 = _1 as i32;
_19.1.2 = _7.1.2 as i128;
_5.fld3 = 173_u8 as f32;
_19.3 = -_15;
_19.4.0 = _10 - _9;
_20 = _1 << _7.2;
_19.1 = (_8, _7.1.0, _7.1.0.0);
_7.4 = _8;
_6.fld1 = _9 ^ _19.4.0;
_19.4.1 = _5.fld7;
_5.fld0 = (_5.fld1,);
_22 = !_10;
_9 = _6.fld1 * _19.4.0;
RET = [73_u8,115_u8,185_u8,157_u8];
_14 = _20 * _3;
_6.fld2 = [164_u8,29_u8,146_u8,65_u8];
_19.1 = (_8, _7.1.0, _7.1.0.0);
_6 = Adt46 { fld0: _7.1.0.1,fld1: _9,fld2: RET };
Call(_8 = core::intrinsics::transmute(_14), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_27 = _7.2 - _7.2;
_19.1.0 = _8 * _8;
_25 = '\u{6c429}';
_19.3 = _6.fld0;
_6.fld0 = _19.1.1.1;
_24 = _5.fld3 * _5.fld3;
Call(_13 = core::intrinsics::transmute(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7.1.0.0 = -_19.1.2;
_34.3 = _7.2 << _27;
_19.1 = (_8, _7.1.0, _7.1.0.0);
_6 = Adt46 { fld0: _19.3,fld1: _9,fld2: RET };
_35.fld3 = _24;
_3 = _14;
_22 = _9 | _9;
_7.1.0.1 = _6.fld0;
_19.4.0 = _19.1.0 as u32;
_5.fld7 = core::ptr::addr_of!(_7.1.0.1);
_7.0 = !2_usize;
_3 = _11 - _20;
_35.fld7 = _5.fld7;
_9 = _22;
_19.4.0 = _9;
_26 = _24 as isize;
_35.fld5 = _34.3;
_20 = !_3;
_38 = _17;
_34.3 = _27;
Goto(bb11)
}
bb11 = {
_21 = _5.fld6 + _5.fld6;
_34.4 = -_8;
_7.1.2 = _2;
_37 = [_19.3,_6.fld0,_7.1.0.1,_19.3];
RET = [36_u8,236_u8,90_u8,67_u8];
_38 = _4;
_34.1.0 = _7.1.0;
_35.fld2 = [_19.1.2,_7.1.0.0,_19.1.1.0,_19.1.1.0];
_16 = 10220_u16;
_19.3 = _7.0 as i32;
_19.1.1.1 = _2 as i32;
_38 = _6.fld1 as u64;
_17 = _38;
_8 = _34.4 - _7.4;
_23 = [true,false,true,true,true,true,false];
_10 = _14 as u32;
_35.fld6 = _21 ^ _5.fld6;
_5.fld7 = core::ptr::addr_of!(_7.1.0.1);
_19.0 = _6.fld0 <= _19.1.1.1;
_13 = _7.1.2;
_8 = _2 as f64;
_29 = [167_u8,235_u8,196_u8,135_u8];
_27 = _35.fld6 as u128;
_7.1.1 = [_19.1.1.0,_19.1.1.0,_34.1.0.0];
_34 = (_7.0, _7.1, _5.fld5, _35.fld5, _8);
Goto(bb12)
}
bb12 = {
_6.fld0 = !_7.1.0.1;
_44.1 = _7.1.1;
_42.1 = _7.1.1;
_19.1.1.0 = _2 as i128;
_36 = core::ptr::addr_of!(_45);
_7.1 = (_19.1.1, _42.1, _34.1.2);
(*_36).2 = !_19.1.2;
_13 = -_7.1.2;
_27 = _35.fld5;
_36 = core::ptr::addr_of!(_45);
(*_36) = (_19.1.0, _7.1.0, _7.1.0.0);
_35 = Adt44 { fld0: _5.fld0,fld1: _5.fld0.0,fld2: _5.fld2,fld3: _24,fld4: _38,fld5: _27,fld6: _5.fld6,fld7: _19.4.1 };
(*_36).1.1 = -_19.1.1.1;
_19.4.1 = _5.fld7;
_35 = Adt44 { fld0: _5.fld0,fld1: _5.fld0.0,fld2: _5.fld2,fld3: _24,fld4: _5.fld4,fld5: _27,fld6: _5.fld6,fld7: _19.4.1 };
(*_36) = _19.1;
_45.1.0 = !_7.1.0.0;
Call(_33 = fn9(_10, _36, _19.4, _14, _19.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_35.fld4 = _38;
_34.2 = _5.fld5 << _19.1.1.0;
_43.1 = _9 as i32;
place!(Field::<(i128, i32)>(Variant(place!(Field::<Adt47>(Variant(_33, 1), 1)), 0), 1)) = _45.1;
SetDiscriminant(Field::<Adt47>(Variant(_33, 1), 1), 1);
_7.2 = !_34.2;
_5.fld3 = _7.4 as f32;
RET = _6.fld2;
(*_36).2 = -_7.1.0.0;
_52 = [_45.1.1,_45.1.1,_43.1,Field::<(f64, (i128, i32), i128)>(Variant(_33, 1), 0).1.1];
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(_33, 1), 1)), 1), 0)).1.2 = _19.1.1.0;
_48 = _20;
_29 = [206_u8,53_u8,139_u8,139_u8];
Call(place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(_33, 1), 1)), 1), 0)).1.1.0 = fn10(Field::<*const i32>(Variant(_33, 1), 2), _19.0, _36, _19, _34, _19.1.1, _19.4, _34.1, Field::<(f64, (i128, i32), i128)>(Variant(_33, 1), 0), _19.1, Field::<(f64, (i128, i32), i128)>(Variant(_33, 1), 0), _36, _19.1.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_42.0.0 = _13 as i128;
_47 = _7.1.2 as i128;
_7.1.2 = _34.1.2 ^ _34.1.2;
_44 = (Field::<(f64, (i128, i32), i128)>(Variant(_33, 1), 0).1, _42.1, _7.1.2);
_6 = Adt46 { fld0: _19.1.1.1,fld1: _10,fld2: _29 };
_19.4.2 = core::ptr::addr_of!(_16);
(*_36).0 = _19.1.0;
_41.fld2 = _7.2 as isize;
_34 = (_7.0, _44, _35.fld5, _7.2, _45.0);
_49 = _16 as u64;
_43.0 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(_33, 1), 1), 1), 0).1.1.0;
_8 = -_34.4;
_42.0.1 = Field::<(f64, (i128, i32), i128)>(Variant(_33, 1), 0).1.1 ^ (*_36).1.1;
_7.1.0.1 = _44.0.1 + _42.0.1;
_19.1 = (_45.0, _43, _45.1.0);
_42 = (_34.1.0, _34.1.1, _2);
_22 = !_10;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(_33, 1), 1)), 1), 0)).4.1 = _19.4.1;
_52 = [Field::<(f64, (i128, i32), i128)>(Variant(_33, 1), 0).1.1,(*_36).1.1,(*_36).1.1,(*_36).1.1];
_35.fld5 = _27 >> _42.0.1;
_35.fld0 = (_35.fld1,);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(_33, 1), 1)), 1), 0)).2 = [(-10807_i16),5933_i16,6539_i16,(-30484_i16),(-4980_i16)];
Goto(bb15)
}
bb15 = {
Call(_57 = dump_var(8_usize, 23_usize, Move(_23), 47_usize, Move(_47), 42_usize, Move(_42), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_57 = dump_var(8_usize, 12_usize, Move(_12), 4_usize, Move(_4), 14_usize, Move(_14), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(8_usize, 3_usize, Move(_3), 52_usize, Move(_52), 25_usize, Move(_25), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_57 = dump_var(8_usize, 49_usize, Move(_49), 1_usize, Move(_1), 22_usize, Move(_22), 58_usize, _58), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u32,mut _2: *const (f64, (i128, i32), i128),mut _3: (u32, *const i32, *const u16),mut _4: isize,mut _5: bool) -> Adt48 {
mir! {
type RET = Adt48;
let _6: Adt44;
let _7: char;
let _8: u8;
let _9: f32;
let _10: [bool; 7];
let _11: isize;
let _12: i32;
let _13: f32;
let _14: [i32; 4];
let _15: isize;
let _16: *mut ((i128, i32), [i128; 3], i8);
let _17: f64;
let _18: *const u16;
let _19: (i128, i32);
let _20: char;
let _21: (f64, (i128, i32), i128);
let _22: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32);
let _23: char;
let _24: ();
let _25: ();
{
(*_2).1.1 = _1 as i32;
(*_2).1.1 = 19_u8 as i32;
_6.fld3 = (-22405_i16) as f32;
_6.fld2 = [(*_2).2,(*_2).1.0,(*_2).1.0,(*_2).1.0];
_6.fld2 = [(*_2).2,(*_2).1.0,(*_2).2,(*_2).2];
(*_2).1.1 = 523763306_i32;
_6.fld5 = 11650495393280694581_u64 as u128;
_6.fld7 = _3.1;
_6.fld6 = 1473542405398493432_i64 << _1;
(*_2).1.1 = (-410488841_i32);
(*_2).1.0 = (*_2).2 - (*_2).2;
(*_2).2 = (*_2).1.1 as i128;
(*_2).0 = 1214_i16 as f64;
match (*_2).1.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431357722615 => bb6,
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
_6.fld6 = (-6890933451451047306_i64);
(*_2).1.0 = (*_2).2 << _3.0;
(*_2).1 = ((*_2).2, 1160065022_i32);
_1 = _3.0;
_3.1 = core::ptr::addr_of!((*_2).1.1);
_2 = core::ptr::addr_of!((*_2));
_6.fld6 = !851194142228757854_i64;
_6.fld3 = (*_2).1.1 as f32;
_7 = '\u{4e0b2}';
(*_2).0 = 126_u8 as f64;
(*_2).1.0 = (*_2).2 ^ (*_2).2;
(*_2).0 = 14863_i16 as f64;
RET = Adt48::Variant0 { fld0: _5,fld1: (*_2).1.1,fld2: _4,fld3: (*_2) };
(*_2).1.1 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 0), 3).1.1 << Field::<isize>(Variant(RET, 0), 2);
_6.fld4 = 3801844310317394644_u64;
_6.fld5 = 1477646677523249575231929880292793836_u128;
SetDiscriminant(RET, 1);
place!(Field::<Adt47>(Variant(RET, 1), 1)) = Adt47::Variant0 { fld0: _3.1,fld1: (*_2).1 };
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1.1 = Field::<(i128, i32)>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 0), 1).1;
(*_2).1 = (Field::<(i128, i32)>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 0), 1).0, Field::<(i128, i32)>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 0), 1).1);
place!(Field::<(i128, i32)>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 0), 1)).1 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).1.1;
place!(Field::<*const i32>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 0), 0)) = core::ptr::addr_of!(place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1.1);
Goto(bb7)
}
bb7 = {
(*_2).1.0 = Field::<(i128, i32)>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 0), 1).0;
SetDiscriminant(Field::<Adt47>(Variant(RET, 1), 1), 1);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).1.1 = (*_2).1;
_9 = _6.fld3 - _6.fld3;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).4 = _3;
_12 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1.0 = (*_2).2 - Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1.0;
(*_2).1.1 = _12;
place!(Field::<i16>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 4)) = (-19093_i16);
place!(Field::<*const i32>(Variant(RET, 1), 2)) = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).4.1;
_5 = !true;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1 = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1.0, _12);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).1.2 = (*_2).1.0 + (*_2).1.0;
place!(Field::<i16>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 4)) = _6.fld5 as i16;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).3 = _6.fld6 as i32;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).0 = !_5;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).3 = (*_2).1.1 ^ (*_2).1.1;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)) = ((*_2).0, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1, (*_2).1.0);
(*_2).0 = -Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).0;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1.1 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).0 as i32;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).1.0 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).0;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).2 = [Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4)];
(*_2).1 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1;
_8 = _7 as u8;
place!(Field::<[bool; 7]>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 2)) = [Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,_5,_5,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,_5];
_10 = [Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,_5,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0,_5,Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).0];
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1.1 = _6.fld4 as i32;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)) = ((*_2).0, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1, (*_2).2);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).4.0 = _3.0;
Call(place!(Field::<*const i32>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 3)) = core::intrinsics::arith_offset(_3.1, 9223372036854775807_isize), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<[bool; 7]>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 2)) = _10;
_8 = _6.fld6 as u8;
_6.fld5 = 331584830298784349304052727577316660965_u128 + 51114753522492001405566006185264542361_u128;
(*_2).1 = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1.0, _12);
_1 = Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).4.0 << Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).3;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)) = ((*_2).0, (*_2).1, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.2);
(*_2).1.0 = !Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).1.0;
(*_2) = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.0, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1.0);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).0 = _5;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1 = ((*_2).2, _12);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).1.1 = (Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).2, _12);
_15 = _4;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).2 = [Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4),Field::<i16>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 4)];
Goto(bb9)
}
bb9 = {
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).1.1 = (*_2).1.1 | (*_2).1.1;
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).1.0 = Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).0;
_2 = core::ptr::addr_of!(place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)));
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).4.1 = Field::<*const i32>(Variant(RET, 1), 2);
place!(Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(place!(Field::<Adt47>(Variant(RET, 1), 1)), 1), 0)).4 = (_3.0, Field::<*const i32>(Variant(RET, 1), 2), _3.2);
_13 = -_6.fld3;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)) = (Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.0, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.1, Field::<(bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16))>(Variant(Field::<Adt47>(Variant(RET, 1), 1), 1), 0).1.2);
place!(Field::<Adt47>(Variant(RET, 1), 1)) = Adt47::Variant0 { fld0: Field::<*const i32>(Variant(RET, 1), 2),fld1: Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0).1 };
_1 = _3.0;
_6.fld3 = _12 as f32;
place!(Field::<(f64, (i128, i32), i128)>(Variant(RET, 1), 0)).0 = (-120_i8) as f64;
_22.5.1.1 = !(*_2).1.1;
Goto(bb10)
}
bb10 = {
Call(_24 = dump_var(9_usize, 5_usize, Move(_5), 12_usize, Move(_12), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const i32,mut _2: bool,mut _3: *const (f64, (i128, i32), i128),mut _4: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),mut _5: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64),mut _6: (i128, i32),mut _7: (u32, *const i32, *const u16),mut _8: ((i128, i32), [i128; 3], i8),mut _9: (f64, (i128, i32), i128),mut _10: (f64, (i128, i32), i128),mut _11: (f64, (i128, i32), i128),mut _12: *const (f64, (i128, i32), i128),mut _13: f64) -> i128 {
mir! {
type RET = i128;
let _14: *mut u16;
let _15: usize;
let _16: [i16; 5];
let _17: isize;
let _18: [u8; 4];
let _19: u16;
let _20: ();
let _21: ();
{
_6.0 = (*_12).2;
_9.2 = _10.1.0 - (*_3).2;
(*_12).1 = (_9.2, _11.1.1);
_11 = _4.1;
RET = !(*_12).1.0;
_10.1.0 = (*_12).1.0 - (*_3).2;
_5.1 = ((*_12).1, _8.1, _8.2);
_5.1 = (_10.1, _8.1, _8.2);
_7.1 = _1;
_9.1.1 = -_6.1;
(*_12).0 = _10.0 + _10.0;
_11.1.1 = -(*_12).1.1;
_5.1.1 = _8.1;
_5.1.0 = (RET, (*_1));
_4.4.1 = _1;
(*_3).1.1 = -_11.1.1;
(*_12).1.0 = _8.0.0;
_5.1 = ((*_12).1, _8.1, _8.2);
(*_12).1.0 = _5.1.0.0 + _10.1.0;
_8.0.1 = _2 as i32;
_10.1.1 = _11.1.1;
_5.1.0.1 = !_6.1;
_16 = [2325_i16,(-2195_i16),30760_i16,19696_i16,(-15381_i16)];
_17 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_4.1.1 = ((*_3).2, (*_12).1.1);
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(10_usize, 16_usize, Move(_16), 17_usize, Move(_17), 21_usize, _21, 21_usize, _21), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: f64,mut _2: [i16; 5],mut _3: u128,mut _4: (i128, i32),mut _5: u128,mut _6: isize,mut _7: f64,mut _8: isize,mut _9: Adt46,mut _10: [i16; 5],mut _11: isize,mut _12: f64,mut _13: [i16; 5],mut _14: isize) -> (*mut [i128; 3],) {
mir! {
type RET = (*mut [i128; 3],);
let _15: char;
let _16: (isize, bool, u32);
let _17: (f64, (i128, i32), i128);
let _18: [u8; 4];
let _19: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _20: isize;
let _21: char;
let _22: isize;
let _23: char;
let _24: [i16; 5];
let _25: (f64, (i128, i32), i128);
let _26: [bool; 7];
let _27: (i128, i32);
let _28: [i16; 5];
let _29: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32);
let _30: f64;
let _31: [i32; 4];
let _32: isize;
let _33: u64;
let _34: (*mut [i128; 3],);
let _35: ();
let _36: ();
{
_2 = [18837_i16,(-12865_i16),17824_i16,30619_i16,(-8060_i16)];
_4.1 = 102_i8 as i32;
_13 = [8556_i16,4044_i16,(-7028_i16),26387_i16,11953_i16];
_11 = -_6;
_9.fld0 = _4.1 | _4.1;
Goto(bb1)
}
bb1 = {
_9.fld2 = [14_u8,242_u8,213_u8,248_u8];
_8 = _11;
_1 = _7 - _7;
_3 = _5 >> _6;
_3 = 1837454391902425946_u64 as u128;
_4.1 = !_9.fld0;
_16 = (_6, true, _9.fld1);
_12 = _1 + _7;
_17.2 = _4.0 + _4.0;
_9.fld0 = 15546_i16 as i32;
_10 = [(-20997_i16),(-7097_i16),(-20144_i16),22178_i16,5582_i16];
_4.0 = _17.2 | _17.2;
_17.0 = _7 - _7;
_17.2 = _4.0;
_17.1 = (_4.0, _4.1);
_2 = [(-4808_i16),26175_i16,17369_i16,19351_i16,(-14259_i16)];
_3 = _5;
_19.2 = _3 ^ _5;
_5 = _9.fld1 as u128;
_7 = _9.fld1 as f64;
Goto(bb2)
}
bb2 = {
_19.3 = _5;
_19.3 = 28_u8 as u128;
_19.0 = 13710994043404969672_usize;
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
_9.fld2 = [39_u8,81_u8,74_u8,25_u8];
_4.0 = !_17.1.0;
_15 = '\u{430a}';
_19.1.0.0 = _4.0;
_23 = _15;
_16 = (_8, true, _9.fld1);
_9.fld0 = !_4.1;
_11 = _16.0 >> _17.1.0;
_10 = _2;
_4.0 = -_17.2;
_19.4 = _16.2 as f64;
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
_19.1.2 = -(-22_i8);
_17 = (_7, _4, _4.0);
_20 = _16.0;
_19.4 = _1 * _17.0;
_19.0 = 5_usize - 4_usize;
_22 = (-14601_i16) as isize;
_21 = _23;
_2 = [21315_i16,(-26047_i16),(-5007_i16),6470_i16,31482_i16];
_20 = -_11;
_9.fld0 = _17.0 as i32;
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
_16 = (_6, true, _9.fld1);
Call(_6 = core::intrinsics::bswap(_20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_23 = _15;
_19.1.0.1 = -_9.fld0;
_25 = (_12, _19.1.0, _19.1.0.0);
_25 = _17;
_25.2 = _25.1.0 - _4.0;
_19.1.1 = [_25.1.0,_25.2,_4.0];
_22 = -_6;
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
_19.1.2 = 98_i8;
_25.1.1 = _19.1.0.1 << _5;
_9.fld2 = [78_u8,36_u8,224_u8,64_u8];
_17.2 = _19.0 as i128;
_13 = _10;
_22 = _20 ^ _14;
_10 = [13387_i16,20496_i16,24954_i16,(-21801_i16),(-21270_i16)];
_16 = (_14, true, _9.fld1);
_17.0 = _1;
_16.1 = _6 >= _22;
_19.1.1 = [_25.2,_25.2,_4.0];
_8 = 468592805553910010_u64 as isize;
_19.1.2 = (-38_i8);
_19.1.1 = [_17.1.0,_25.2,_17.1.0];
_17.1.0 = _15 as i128;
_12 = _19.1.2 as f64;
_5 = _3;
_19.1.0.0 = _4.0 - _25.2;
_5 = !_3;
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
Call(_19.2 = core::intrinsics::bswap(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_25 = (_17.0, _17.1, _19.1.0.0);
_15 = _23;
_29.4 = (_25.2, _9.fld0);
_29.1 = !_19.0;
_17.0 = _7 * _19.4;
_9.fld0 = 6982069189495403350_u64 as i32;
Goto(bb5)
}
bb5 = {
_1 = _7;
_27.1 = !_29.4.1;
_19.1.2 = _19.1.0.1 as i8;
_4 = (_19.1.0.0, _19.1.0.1);
_14 = _11 - _22;
_29.5.1 = (_4.0, _27.1);
_25.0 = _19.4 - _19.4;
_28 = _2;
_29.6 = _29.4.1 + _19.1.0.1;
_4.0 = _25.2 << _14;
_12 = _25.0;
_3 = 18334411486700617799_u64 as u128;
_29.0 = -_22;
_29.5.2 = _4.0 & _4.0;
_29.2 = core::ptr::addr_of!(_19.1.0.1);
_17.0 = -_25.0;
_18 = _9.fld2;
_16.1 = true & false;
_24 = _13;
_31 = [_29.5.1.1,_29.6,_27.1,_29.4.1];
_29.5.1.1 = _29.6 & _4.1;
_19.1.1 = [_29.4.0,_25.2,_4.0];
_9.fld1 = _16.2;
_27.0 = _29.5.2;
_19.1.2 = 113_i8 & 110_i8;
Call(_30 = fn12(_4, _19.2, _19.2, _27.0, _4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19.1.1 = [_27.0,_29.5.2,_29.5.2];
_33 = !7043975720429952781_u64;
_13 = _10;
_29.0 = _11 | _22;
_31 = [_29.4.1,_29.5.1.1,_29.5.1.1,_29.6];
_17.1.1 = _9.fld1 as i32;
_19.1.0.1 = -_29.6;
_17.2 = -_29.5.2;
_9 = Adt46 { fld0: _29.6,fld1: _16.2,fld2: _18 };
_29.4.0 = _19.1.0.0 << _19.2;
RET.0 = core::ptr::addr_of_mut!(_19.1.1);
Goto(bb7)
}
bb7 = {
Call(_35 = dump_var(11_usize, 2_usize, Move(_2), 22_usize, Move(_22), 15_usize, Move(_15), 16_usize, Move(_16)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_35 = dump_var(11_usize, 23_usize, Move(_23), 27_usize, Move(_27), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_35 = dump_var(11_usize, 10_usize, Move(_10), 14_usize, Move(_14), 21_usize, Move(_21), 36_usize, _36), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (i128, i32),mut _2: u128,mut _3: u128,mut _4: i128,mut _5: (i128, i32)) -> f64 {
mir! {
type RET = f64;
let _6: *mut [i128; 3];
let _7: u16;
let _8: Adt47;
let _9: Adt46;
let _10: u8;
let _11: *mut [i128; 3];
let _12: (i128, i32);
let _13: (f64, (i128, i32), i128);
let _14: ((i128, i32), [i128; 3], i8);
let _15: u128;
let _16: Adt46;
let _17: i16;
let _18: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _19: u128;
let _20: bool;
let _21: i32;
let _22: i16;
let _23: f64;
let _24: u16;
let _25: (i128, i32);
let _26: isize;
let _27: Adt45;
let _28: [i128; 4];
let _29: [i128; 3];
let _30: Adt44;
let _31: isize;
let _32: [i128; 4];
let _33: Adt57;
let _34: ();
let _35: ();
{
RET = 5497493495499723021_usize as f64;
_1 = _5;
_4 = !_1.0;
_1.1 = _5.1 * _5.1;
RET = _1.0 as f64;
_3 = _2;
_2 = _3 << _5.0;
_3 = !_2;
_5 = (_4, _1.1);
_1 = (_5.0, _5.1);
_7 = 47991_u16;
RET = 1454382298_u32 as f64;
_5.0 = _1.0 * _4;
_1.1 = -_5.1;
RET = (-6_isize) as f64;
Goto(bb1)
}
bb1 = {
RET = 87_i8 as f64;
_9.fld1 = 147_u8 as u32;
_1.0 = !_5.0;
_3 = (-8305_i16) as u128;
_9.fld0 = !_1.1;
_2 = _3 << _4;
_9.fld2 = [64_u8,235_u8,173_u8,130_u8];
_9.fld1 = 4179055602_u32 | 3491780251_u32;
_9.fld0 = _5.1;
Call(_9.fld1 = fn13(_5, _5, _5.0, _5, _1, _5.0, _2, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = 13550_u16 << _4;
_4 = _1.0;
_7 = 23056_u16 ^ 17892_u16;
_9.fld0 = _1.1 | _1.1;
_9.fld0 = -_1.1;
_5.0 = _2 as i128;
_5.1 = !_9.fld0;
_9.fld2 = [212_u8,186_u8,132_u8,59_u8];
_1.0 = 80_u8 as i128;
RET = 4757746560293040317_i64 as f64;
_12.1 = _5.1 ^ _1.1;
_2 = _3;
_12.1 = _5.1;
_12.0 = _5.0 + _4;
_1.0 = _4 - _12.0;
_1.1 = _12.1 & _12.1;
_10 = !88_u8;
_7 = 28504_i16 as u16;
_1 = _12;
_12.0 = -_4;
_9.fld1 = (-3339345041676761609_i64) as u32;
_12.0 = _1.1 as i128;
_9.fld2 = [_10,_10,_10,_10];
_3 = _2;
RET = _10 as f64;
Call(_9.fld1 = core::intrinsics::transmute(_12.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5.0 = !_12.0;
RET = _10 as f64;
_12.0 = _4;
RET = _3 as f64;
_9.fld0 = _1.1 + _1.1;
RET = 2967766589949212720_i64 as f64;
_3 = _2 ^ _2;
_2 = _3 ^ _3;
_9.fld2 = [_10,_10,_10,_10];
Goto(bb4)
}
bb4 = {
_12.1 = !_5.1;
_9.fld0 = -_12.1;
_12 = (_1.0, _9.fld0);
_5.0 = _1.0;
RET = _10 as f64;
_4 = !_1.0;
_13.1.0 = -_5.0;
_13 = (RET, _12, _1.0);
_5.1 = _5.0 as i32;
Call(_3 = fn17(_4, _1.0, _1.0, _1.0, _5, _1, _12.0, _12.1, _9.fld1, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = -_13.0;
_5 = (_4, _13.1.1);
_13 = (RET, _5, _12.0);
RET = _13.0 * _13.0;
_13.0 = 11988528567005190578_u64 as f64;
_14.0.1 = _12.1;
_3 = _2;
_7 = 50541_u16 ^ 22877_u16;
RET = -_13.0;
RET = _13.0 * _13.0;
_16.fld2 = _9.fld2;
_13.0 = 507305424052104732_i64 as f64;
_14.0 = (_5.0, _9.fld0);
_11 = core::ptr::addr_of_mut!(_14.1);
_5.0 = _12.0 >> _14.0.0;
_13 = (RET, _12, _14.0.0);
_18.2 = _2 * _2;
Call(_18.1.1 = fn19(_14.0.0, _5.0, _13.1, _5, _12, _1.0, _1.0, _13.1, _13.1, _4, _14.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_18.4 = _13.0 - _13.0;
_13 = (_18.4, _14.0, _14.0.0);
_17 = !26306_i16;
_16.fld1 = !_9.fld1;
_20 = false & true;
_18.1.2 = !(-5_i8);
_18.0 = 3_usize;
_12.1 = _18.0 as i32;
_18.1.0.1 = _20 as i32;
_1.0 = _12.0;
_5.0 = 13880292636755688430_u64 as i128;
_16.fld0 = _4 as i32;
_18.1.0 = (_1.0, _9.fld0);
(*_11) = _18.1.1;
_9 = Adt46 { fld0: _13.1.1,fld1: _16.fld1,fld2: _16.fld2 };
_14 = (_1, _18.1.1, _18.1.2);
_18.1 = (_12, (*_11), _14.2);
_22 = _9.fld1 as i16;
_3 = _18.2 * _18.2;
_16.fld0 = -_5.1;
_5.1 = _16.fld0 - _14.0.1;
_19 = _3;
_4 = _1.0;
Goto(bb7)
}
bb7 = {
_21 = 5497467184929738309_u64 as i32;
_13.1 = (_12.0, _9.fld0);
_13.1 = (_13.2, _16.fld0);
Goto(bb8)
}
bb8 = {
_14.2 = _16.fld1 as i8;
_25.0 = !_1.0;
_26 = !(-47_isize);
_16.fld0 = -_13.1.1;
_9.fld1 = _16.fld1;
_13.1 = _1;
_12.0 = _18.1.0.0;
_25 = _18.1.0;
_6 = _11;
RET = _18.4 + _13.0;
_21 = _1.1 & _5.1;
_17 = _22 & _22;
_23 = _18.0 as f64;
_2 = _18.2 | _19;
(*_6) = _18.1.1;
_11 = core::ptr::addr_of_mut!((*_11));
_25.0 = _4;
_13.0 = -RET;
_14.1 = [_13.1.0,_13.1.0,_1.0];
(*_6) = [_14.0.0,_5.0,_4];
_22 = !_17;
_16 = Adt46 { fld0: _21,fld1: _9.fld1,fld2: _9.fld2 };
_5.0 = _4;
_29 = [_1.0,_12.0,_13.1.0];
_13.2 = _12.0;
Goto(bb9)
}
bb9 = {
_18.1 = (_14.0, _29, _14.2);
_25.1 = -_5.1;
_23 = RET;
_13.2 = -_4;
_22 = _17;
_16.fld0 = !_5.1;
_21 = !_5.1;
_18.1 = _14;
_16 = Move(_9);
(*_6) = [_25.0,_4,_13.1.0];
_18.3 = !_2;
_23 = _18.3 as f64;
_30.fld6 = -8230496286243968797_i64;
RET = _16.fld1 as f64;
_5 = (_25.0, _16.fld0);
_11 = core::ptr::addr_of_mut!((*_6));
_30.fld0.0 = _11;
_3 = _18.3 - _18.3;
_30.fld2 = [_18.1.0.0,_5.0,_1.0,_1.0];
_13.2 = _26 as i128;
_30.fld1 = core::ptr::addr_of_mut!((*_6));
_19 = _18.3;
_31 = _26;
_18.1 = (_12, (*_11), _14.2);
_13.1.0 = _16.fld1 as i128;
_30.fld1 = core::ptr::addr_of_mut!((*_6));
_18.1.0.0 = _16.fld1 as i128;
Goto(bb10)
}
bb10 = {
Call(_34 = dump_var(12_usize, 1_usize, Move(_1), 21_usize, Move(_21), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_34 = dump_var(12_usize, 29_usize, Move(_29), 25_usize, Move(_25), 31_usize, Move(_31), 22_usize, Move(_22)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_34 = dump_var(12_usize, 2_usize, Move(_2), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: (i128, i32),mut _2: (i128, i32),mut _3: i128,mut _4: (i128, i32),mut _5: (i128, i32),mut _6: i128,mut _7: u128,mut _8: i128) -> u32 {
mir! {
type RET = u32;
let _9: bool;
let _10: *mut u16;
let _11: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16));
let _12: Adt46;
let _13: isize;
let _14: char;
let _15: isize;
let _16: [u8; 4];
let _17: *const &'static f64;
let _18: *mut [i128; 3];
let _19: Adt50;
let _20: bool;
let _21: bool;
let _22: f64;
let _23: Adt44;
let _24: Adt56;
let _25: &'static f64;
let _26: [i128; 4];
let _27: Adt60;
let _28: [i128; 4];
let _29: f64;
let _30: u128;
let _31: [bool; 7];
let _32: bool;
let _33: (isize, bool, u32);
let _34: Adt46;
let _35: [u8; 4];
let _36: char;
let _37: ((i128, i32), [i128; 3], i8);
let _38: u16;
let _39: f64;
let _40: isize;
let _41: i8;
let _42: Adt45;
let _43: f32;
let _44: ();
let _45: ();
{
_4.1 = _1.1 & _5.1;
_3 = _4.0;
_5.1 = -_1.1;
_6 = 3_usize as i128;
RET = !2922827008_u32;
Goto(bb1)
}
bb1 = {
_4.1 = RET as i32;
_5.0 = !_2.0;
_2 = (_8, _5.1);
_5 = (_4.0, _1.1);
_5.1 = _2.1 | _1.1;
RET = false as u32;
_2.1 = !_5.1;
RET = !2691173911_u32;
_1.0 = _4.0 ^ _8;
_8 = _3;
_5.0 = 23_u8 as i128;
_7 = !80115273065155211648715825829605864970_u128;
_5 = (_3, _2.1);
_5.0 = !_8;
_2.1 = _5.1;
Call(_5 = fn14(_8, _4, _2, _4.0, _4, _2, _3, _2, _2.0, _4, _4.0, _4, _1.1, _1.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _1;
_9 = false;
_3 = _2.0;
_1.1 = _2.0 as i32;
_2.1 = _5.1;
_6 = _4.0 | _3;
_8 = !_3;
_2.1 = _1.1 & _5.1;
_11.1.1.1 = !_1.1;
_11.0 = !_9;
_11.1.1 = (_8, _2.1);
Goto(bb3)
}
bb3 = {
_11.1.1.1 = _2.1;
_2 = _11.1.1;
_11.3 = 1_usize as i32;
Goto(bb4)
}
bb4 = {
_11.1.2 = _1.0;
_11.4.1 = core::ptr::addr_of!(_4.1);
_8 = (-9223372036854775808_isize) as i128;
RET = 3831341835_u32;
_11.4.0 = RET - RET;
_11.4.1 = core::ptr::addr_of!(_11.1.1.1);
_11.1.0 = _6 as f64;
_11.1.0 = _2.0 as f64;
_2.1 = !_1.1;
_13 = 6523250798908620037_u64 as isize;
_15 = _13 << _3;
_2 = _5;
_2 = _11.1.1;
_11.1.1.1 = 2_usize as i32;
_11.1.1 = (_1.0, _5.1);
RET = _11.0 as u32;
_21 = _9;
_11.0 = !_9;
RET = 25379_u16 as u32;
_2.1 = !_4.1;
_1 = (_3, _11.1.1.1);
_12.fld1 = 1558732449351123429_u64 as u32;
Goto(bb5)
}
bb5 = {
_1.0 = 7_usize as i128;
_12.fld0 = _5.1;
_9 = _15 != _15;
_5.0 = _7 as i128;
_11.2 = [(-31582_i16),(-22714_i16),(-1589_i16),(-32078_i16),(-12001_i16)];
_11.1.2 = _4.0;
_8 = '\u{96098}' as i128;
_12.fld2 = [53_u8,28_u8,139_u8,0_u8];
_2 = _4;
_16 = _12.fld2;
_11.1.1.1 = (-5546327765753734101_i64) as i32;
_9 = _11.0 ^ _21;
_11.1.1.1 = _11.3 | _12.fld0;
_2.1 = _12.fld1 as i32;
_2 = _5;
_2.1 = 17373_i16 as i32;
Goto(bb6)
}
bb6 = {
_21 = _9 ^ _9;
_12.fld0 = _11.1.1.1 & _11.1.1.1;
_19.fld0 = Adt46 { fld0: _12.fld0,fld1: _12.fld1,fld2: _16 };
_2.1 = 42409912596585944_u64 as i32;
_21 = !_11.0;
_2.0 = _11.1.0 as i128;
Call(_11.1.1 = fn15(_1.1, _2.0, _15, _12.fld0, _2.0, _6, _11.4.1, Move(_12), _6, _4, _4.1, _11.4.1, _19.fld0.fld0, _5), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1.0 = 25826_u16 as i128;
_1.0 = 219_u8 as i128;
_20 = !_21;
_11.1.0 = 40371_u16 as f64;
_14 = '\u{28787}';
_4.0 = _6 + _11.1.2;
RET = _11.4.0;
_22 = -_11.1.0;
_19.fld0.fld0 = _11.1.1.1;
_22 = _11.1.0 * _11.1.0;
_11.1 = (_22, _4, _4.0);
_2 = _11.1.1;
_12.fld2 = [194_u8,168_u8,125_u8,144_u8];
_20 = _11.0;
_15 = !_13;
_11.1.1 = (_3, _19.fld0.fld0);
_11.1.1.1 = !_19.fld0.fld0;
_8 = _4.0 - _4.0;
_19.fld0.fld2 = [34_u8,199_u8,145_u8,103_u8];
RET = _4.1 as u32;
_21 = _6 >= _4.0;
_19.fld0.fld0 = _2.1 << _2.1;
_23.fld4 = _14 as u64;
_17 = core::ptr::addr_of!(_25);
_23.fld3 = 7713_i16 as f32;
RET = _19.fld0.fld1 & _19.fld0.fld1;
_27.fld3.1 = (_11.1.1.0, _1.1);
Goto(bb8)
}
bb8 = {
_19.fld0.fld2 = [141_u8,79_u8,122_u8,179_u8];
_13 = !_15;
_26 = [_27.fld3.1.0,_2.0,_11.1.1.0,_11.1.1.0];
_16 = [136_u8,210_u8,94_u8,210_u8];
_5.0 = -_4.0;
_27.fld3.0 = _11.1.0;
_19.fld0.fld1 = (-8666_i16) as u32;
_27.fld0.5.1 = (_5.0, _19.fld0.fld0);
_23.fld5 = RET as u128;
_11.1.1.0 = _27.fld3.1.0 + _8;
_27.fld0.5 = _11.1;
_23.fld6 = -6957894404283555348_i64;
_25 = &_22;
_27.fld3.0 = _23.fld6 as f64;
_12 = Adt46 { fld0: _27.fld0.5.1.1,fld1: RET,fld2: _16 };
_17 = core::ptr::addr_of!((*_17));
_27.fld0.3 = !75_u8;
_27.fld0.6 = _27.fld3.1.1;
_27.fld3 = (_22, _5, _27.fld0.5.1.0);
Goto(bb9)
}
bb9 = {
_12.fld1 = !_11.4.0;
_23.fld3 = _12.fld0 as f32;
_27.fld0.4.0 = _8 * _2.0;
_19.fld0 = Adt46 { fld0: _27.fld0.5.1.1,fld1: _11.4.0,fld2: _16 };
(*_17) = &_22;
_26 = [_3,_4.0,_8,_27.fld0.4.0];
_2 = (_4.0, _5.1);
_27.fld0.0 = _14 as isize;
_27.fld0.1 = !2077850651395927409_usize;
_12.fld1 = RET;
_29 = _6 as f64;
_23.fld7 = core::ptr::addr_of!(_1.1);
_27.fld0.4.1 = _27.fld0.5.1.1;
_27.fld4 = _29 + (*_25);
_27.fld3.1.0 = _27.fld0.4.0;
_4.1 = _27.fld0.3 as i32;
_27.fld0 = (_13, 1509634526603471320_usize, _11.4.1, 200_u8, _2, _27.fld3, _5.1);
_11.2 = [27361_i16,(-21839_i16),26882_i16,4046_i16,(-20792_i16)];
Call(_27.fld0 = fn16(_5.0, _12.fld0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_26 = [_27.fld3.2,_27.fld0.4.0,_27.fld0.5.2,_27.fld3.2];
_27.fld0 = (_13, 7178232402129726899_usize, _11.4.1, 212_u8, _11.1.1, _27.fld3, _2.1);
_8 = _23.fld4 as i128;
_27.fld3 = (_27.fld4, _4, _27.fld0.5.2);
_15 = _13;
Goto(bb11)
}
bb11 = {
_23.fld2 = _26;
_22 = _27.fld4 + _27.fld3.0;
_4.0 = -_27.fld0.5.1.0;
_1 = _5;
_11.1 = _27.fld3;
_13 = !_27.fld0.0;
_22 = -_27.fld3.0;
_16 = [_27.fld0.3,_27.fld0.3,_27.fld0.3,_27.fld0.3];
(*_17) = &_27.fld3.0;
_1.0 = (-16_i8) as i128;
Goto(bb12)
}
bb12 = {
_23.fld2 = [_11.1.1.0,_3,_2.0,_2.0];
_11.3 = _27.fld0.4.1;
_5 = _27.fld0.5.1;
_3 = _14 as i128;
_2.0 = _11.1.2;
_30 = _27.fld0.3 as u128;
_21 = _9;
_27.fld3.0 = _19.fld0.fld1 as f64;
_11.1 = (_27.fld4, _1, _2.0);
_34.fld0 = _27.fld0.1 as i32;
_5 = (_27.fld3.2, _27.fld0.6);
_13 = _27.fld0.0;
_36 = _14;
_31 = [_21,_9,_21,_9,_21,_21,_21];
_33.0 = _13;
_27.fld0 = (_33.0, 3_usize, _23.fld7, 184_u8, _4, _11.1, _12.fld0);
_33.2 = _23.fld6 as u32;
_12.fld0 = 17334_i16 as i32;
_33.2 = !RET;
_11.1 = (_29, _27.fld3.1, _27.fld3.2);
_11.1 = (_22, _27.fld3.1, _27.fld3.2);
Goto(bb13)
}
bb13 = {
_33 = (_15, _21, RET);
_1.0 = _6 * _27.fld3.1.0;
(*_17) = &_11.1.0;
_34.fld2 = [_27.fld0.3,_27.fld0.3,_27.fld0.3,_27.fld0.3];
match _27.fld0.3 {
0 => bb7,
1 => bb14,
2 => bb15,
3 => bb16,
184 => bb18,
_ => bb17
}
}
bb14 = {
_1.0 = 7_usize as i128;
_12.fld0 = _5.1;
_9 = _15 != _15;
_5.0 = _7 as i128;
_11.2 = [(-31582_i16),(-22714_i16),(-1589_i16),(-32078_i16),(-12001_i16)];
_11.1.2 = _4.0;
_8 = '\u{96098}' as i128;
_12.fld2 = [53_u8,28_u8,139_u8,0_u8];
_2 = _4;
_16 = _12.fld2;
_11.1.1.1 = (-5546327765753734101_i64) as i32;
_9 = _11.0 ^ _21;
_11.1.1.1 = _11.3 | _12.fld0;
_2.1 = _12.fld1 as i32;
_2 = _5;
_2.1 = 17373_i16 as i32;
Goto(bb6)
}
bb15 = {
_23.fld2 = _26;
_22 = _27.fld4 + _27.fld3.0;
_4.0 = -_27.fld0.5.1.0;
_1 = _5;
_11.1 = _27.fld3;
_13 = !_27.fld0.0;
_22 = -_27.fld3.0;
_16 = [_27.fld0.3,_27.fld0.3,_27.fld0.3,_27.fld0.3];
(*_17) = &_27.fld3.0;
_1.0 = (-16_i8) as i128;
Goto(bb12)
}
bb16 = {
_4 = _1;
_9 = false;
_3 = _2.0;
_1.1 = _2.0 as i32;
_2.1 = _5.1;
_6 = _4.0 | _3;
_8 = !_3;
_2.1 = _1.1 & _5.1;
_11.1.1.1 = !_1.1;
_11.0 = !_9;
_11.1.1 = (_8, _2.1);
Goto(bb3)
}
bb17 = {
_1.0 = 25826_u16 as i128;
_1.0 = 219_u8 as i128;
_20 = !_21;
_11.1.0 = 40371_u16 as f64;
_14 = '\u{28787}';
_4.0 = _6 + _11.1.2;
RET = _11.4.0;
_22 = -_11.1.0;
_19.fld0.fld0 = _11.1.1.1;
_22 = _11.1.0 * _11.1.0;
_11.1 = (_22, _4, _4.0);
_2 = _11.1.1;
_12.fld2 = [194_u8,168_u8,125_u8,144_u8];
_20 = _11.0;
_15 = !_13;
_11.1.1 = (_3, _19.fld0.fld0);
_11.1.1.1 = !_19.fld0.fld0;
_8 = _4.0 - _4.0;
_19.fld0.fld2 = [34_u8,199_u8,145_u8,103_u8];
RET = _4.1 as u32;
_21 = _6 >= _4.0;
_19.fld0.fld0 = _2.1 << _2.1;
_23.fld4 = _14 as u64;
_17 = core::ptr::addr_of!(_25);
_23.fld3 = 7713_i16 as f32;
RET = _19.fld0.fld1 & _19.fld0.fld1;
_27.fld3.1 = (_11.1.1.0, _1.1);
Goto(bb8)
}
bb18 = {
_11.1.1.0 = _27.fld3.1.0;
_27.fld0.4.1 = _27.fld0.6;
_27.fld0.4 = _5;
_21 = !_33.1;
_27.fld0.4.0 = _6 << _1.0;
_12.fld1 = _19.fld0.fld1 ^ _19.fld0.fld1;
_34 = Adt46 { fld0: _19.fld0.fld0,fld1: _19.fld0.fld1,fld2: _16 };
_11.1.0 = _27.fld0.3 as f64;
_4 = _1;
_30 = !_23.fld5;
_18 = core::ptr::addr_of_mut!(_37.1);
_11.1.0 = _27.fld0.5.0 - _27.fld4;
_37.0.1 = _23.fld4 as i32;
_16 = [_27.fld0.3,_27.fld0.3,_27.fld0.3,_27.fld0.3];
(*_17) = &_27.fld4;
_27.fld3 = (_27.fld4, _5, _11.1.1.0);
_27.fld0.5.0 = _27.fld0.1 as f64;
_27.fld1 = _23.fld3 - _23.fld3;
_39 = (-29374_i16) as f64;
_27.fld1 = _23.fld3;
Goto(bb19)
}
bb19 = {
Call(_44 = dump_var(13_usize, 36_usize, Move(_36), 30_usize, Move(_30), 26_usize, Move(_26), 31_usize, Move(_31)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(13_usize, 16_usize, Move(_16), 4_usize, Move(_4), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_44 = dump_var(13_usize, 2_usize, Move(_2), 5_usize, Move(_5), 45_usize, _45, 45_usize, _45), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i128,mut _2: (i128, i32),mut _3: (i128, i32),mut _4: i128,mut _5: (i128, i32),mut _6: (i128, i32),mut _7: i128,mut _8: (i128, i32),mut _9: i128,mut _10: (i128, i32),mut _11: i128,mut _12: (i128, i32),mut _13: i32,mut _14: i128) -> (i128, i32) {
mir! {
type RET = (i128, i32);
let _15: Adt51;
let _16: ((i128, i32), [i128; 3], i8);
let _17: Adt52;
let _18: ();
let _19: ();
{
_5 = (_3.0, _8.1);
_3 = (_14, _5.1);
RET.1 = 3722846569727298883_u64 as i32;
_2 = (_3.0, _6.1);
_10.1 = _4 as i32;
_4 = 21909_u16 as i128;
Goto(bb1)
}
bb1 = {
RET = (_3.0, _6.1);
_5.1 = 9_u8 as i32;
_3.0 = RET.0;
_6.0 = !_8.0;
_12 = _3;
_7 = !_11;
RET.0 = _2.0 & _6.0;
_8 = _3;
_4 = _8.0 + _7;
_14 = -_10.0;
_8 = (_11, _10.1);
_5.0 = -_10.0;
_6 = _10;
_16.0 = (_4, _13);
_6.1 = -_8.1;
_7 = _4 >> _6.0;
_5.1 = 238_u8 as i32;
_10.1 = true as i32;
_16.1 = [_16.0.0,_9,_11];
_2.1 = !_3.1;
_12.0 = _5.0;
RET.0 = 241_u8 as i128;
_6.0 = (-4543_i16) as i128;
_13 = RET.1;
_1 = 136_u8 as i128;
_16.1 = [_8.0,_16.0.0,_10.0];
_16.0.1 = 133_u8 as i32;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(14_usize, 11_usize, Move(_11), 12_usize, Move(_12), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(14_usize, 8_usize, Move(_8), 14_usize, Move(_14), 2_usize, Move(_2), 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i32,mut _2: i128,mut _3: isize,mut _4: i32,mut _5: i128,mut _6: i128,mut _7: *const i32,mut _8: Adt46,mut _9: i128,mut _10: (i128, i32),mut _11: i32,mut _12: *const i32,mut _13: i32,mut _14: (i128, i32)) -> (i128, i32) {
mir! {
type RET = (i128, i32);
let _15: ();
let _16: ();
{
RET = (_6, _4);
_14 = RET;
_13 = _8.fld0;
_7 = core::ptr::addr_of!(_4);
_9 = 196_u8 as i128;
(*_7) = (-50_i8) as i32;
_8.fld1 = 3607012229_u32 - 1092438560_u32;
(*_7) = -_13;
(*_7) = !_11;
_14.0 = false as i128;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(15_usize, 10_usize, Move(_10), 6_usize, Move(_6), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(15_usize, 1_usize, Move(_1), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: i128,mut _2: i32) -> (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32) {
mir! {
type RET = (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32);
let _3: *mut &'static f64;
let _4: Adt51;
let _5: Adt46;
let _6: isize;
let _7: [i128; 3];
let _8: ();
let _9: ();
{
RET.2 = core::ptr::addr_of!(_2);
RET.4.0 = 339337661866398894179774010281931636830_u128 as i128;
RET.4.1 = _2;
RET.5.2 = _1;
RET.1 = RET.4.1 as usize;
Goto(bb1)
}
bb1 = {
RET.0 = '\u{5da2d}' as isize;
RET.6 = _2 & _2;
RET.4.0 = RET.5.2 | RET.5.2;
RET.5.1 = RET.4;
RET.4.1 = 698497164936258801_u64 as i32;
RET.3 = 18_u8 & 28_u8;
RET.5.2 = RET.4.0;
RET.1 = 14544881052306690956_usize & 15218338683292975822_usize;
Goto(bb2)
}
bb2 = {
RET.5.1 = (RET.5.2, _2);
RET.0 = (-33_isize);
RET.5.1 = (RET.5.2, RET.6);
RET.0 = RET.1 as isize;
RET.4.0 = RET.5.2;
RET.3 = !129_u8;
RET.4.0 = !_1;
RET.1 = 11732302549134465504_usize;
RET.4.1 = _2 | _2;
_2 = RET.4.1;
RET.5.1 = (RET.4.0, RET.4.1);
RET.5.0 = RET.0 as f64;
RET.2 = core::ptr::addr_of!(_2);
_5.fld2 = [RET.3,RET.3,RET.3,RET.3];
RET.4.1 = RET.5.0 as i32;
RET.2 = core::ptr::addr_of!(_2);
RET.5.1.1 = _2;
RET.5.2 = -RET.4.0;
RET.3 = RET.0 as u8;
RET.5.0 = _2 as f64;
RET.5.1.1 = RET.6 + _2;
_7 = [RET.5.2,RET.5.1.0,RET.5.2];
Goto(bb3)
}
bb3 = {
Call(_8 = dump_var(16_usize, 7_usize, Move(_7), 9_usize, _9, 9_usize, _9, 9_usize, _9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i128,mut _2: i128,mut _3: i128,mut _4: i128,mut _5: (i128, i32),mut _6: (i128, i32),mut _7: i128,mut _8: i32,mut _9: u32,mut _10: (i128, i32)) -> u128 {
mir! {
type RET = u128;
let _11: (f64, (i128, i32), i128);
let _12: Adt52;
let _13: Adt46;
let _14: u64;
let _15: [i128; 3];
let _16: *const i32;
let _17: char;
let _18: [i32; 4];
let _19: [i32; 4];
let _20: isize;
let _21: [i16; 5];
let _22: bool;
let _23: char;
let _24: u64;
let _25: [i128; 3];
let _26: i64;
let _27: isize;
let _28: bool;
let _29: ();
let _30: ();
{
_8 = 162_u8 as i32;
RET = 75102518876919745398053068688878095385_u128;
_9 = 241293424435821401_u64 as u32;
_4 = true as i128;
_11.0 = 51322_u16 as f64;
RET = !333138313303405909221690437779072997508_u128;
_11.1.0 = _2;
_7 = _6.0;
_6.1 = !_5.1;
_10.1 = -_6.1;
_13.fld2 = [17_u8,144_u8,126_u8,235_u8];
_11.1.0 = _5.0 >> _6.1;
Goto(bb1)
}
bb1 = {
_13.fld1 = 92_i8 as u32;
_6.1 = _10.1 - _5.1;
_10.1 = _6.1;
_11.1 = (_6.0, _6.1);
_5 = (_2, _10.1);
_11.1.1 = !_6.1;
_10 = (_2, _6.1);
_6.0 = _3 ^ _7;
_2 = _10.0 >> _10.1;
_14 = 17821120738914462227_u64;
RET = !96935995669547226940089144189382771539_u128;
_11.1.1 = _10.1;
_8 = _5.1 ^ _5.1;
_15 = [_2,_5.0,_10.0];
_4 = _1;
RET = 35440602080959973619185324666451996101_u128 | 158294940280963175666963363025084901575_u128;
_11.1 = (_3, _10.1);
Goto(bb2)
}
bb2 = {
_4 = 40097_u16 as i128;
_4 = _11.1.0;
_7 = -_11.1.0;
_11.0 = (-21506_i16) as f64;
RET = 268965428270056285080733694908796006989_u128;
_13.fld0 = _10.1;
_5.1 = -_13.fld0;
_17 = '\u{e51e4}';
_11.0 = _3 as f64;
_2 = !_5.0;
_18 = [_6.1,_8,_5.1,_10.1];
_11.0 = _14 as f64;
RET = !141049433323481294204971088748486643681_u128;
match _14 {
17821120738914462227 => bb4,
_ => bb3
}
}
bb3 = {
_13.fld1 = 92_i8 as u32;
_6.1 = _10.1 - _5.1;
_10.1 = _6.1;
_11.1 = (_6.0, _6.1);
_5 = (_2, _10.1);
_11.1.1 = !_6.1;
_10 = (_2, _6.1);
_6.0 = _3 ^ _7;
_2 = _10.0 >> _10.1;
_14 = 17821120738914462227_u64;
RET = !96935995669547226940089144189382771539_u128;
_11.1.1 = _10.1;
_8 = _5.1 ^ _5.1;
_15 = [_2,_5.0,_10.0];
_4 = _1;
RET = 35440602080959973619185324666451996101_u128 | 158294940280963175666963363025084901575_u128;
_11.1 = (_3, _10.1);
Goto(bb2)
}
bb4 = {
_13.fld2 = [124_u8,11_u8,79_u8,113_u8];
_6 = (_4, _5.1);
_6.0 = 55823_u16 as i128;
_11.0 = 10963_u16 as f64;
Call(_1 = fn18(_7, _8, _13.fld0, _10, _5.1, _2, _5.1, _15, _13.fld0, _5.0, _15, _13.fld0, _10.1, _4, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_15 = [_1,_11.1.0,_3];
_18 = [_10.1,_5.1,_6.1,_11.1.1];
_6.1 = _11.1.1;
_16 = core::ptr::addr_of!(_11.1.1);
_11.0 = 9223372036854775807_isize as f64;
_8 = (*_16) - _10.1;
(*_16) = _10.1 << _5.0;
RET = 80691239198710285330853232185626594052_u128;
_5.0 = !_1;
_1 = -_11.1.0;
_10.0 = _5.0;
_3 = _11.1.0 << _10.0;
_5 = (_1, (*_16));
_10.0 = _1;
_11.0 = _13.fld0 as f64;
_4 = 16777_i16 as i128;
_16 = core::ptr::addr_of!(_5.1);
_5 = (_7, _6.1);
_6.1 = (-108_i8) as i32;
(*_16) = !_10.1;
_6.1 = _13.fld0;
_6 = (_5.0, _13.fld0);
_10.0 = RET as i128;
_13.fld1 = 88_i8 as u32;
_11.2 = _11.1.0 << _2;
RET = 266516403345928766591150971266555814289_u128 ^ 283579990555770084161566142477118637388_u128;
Goto(bb6)
}
bb6 = {
_19 = [_6.1,_11.1.1,_13.fld0,_6.1];
_21 = [6589_i16,27674_i16,(-8781_i16),19332_i16,4020_i16];
_13.fld1 = _9 + _9;
(*_16) = _6.1;
Goto(bb7)
}
bb7 = {
_10.0 = _11.2 << _6.1;
_5.0 = false as i128;
_11.1.1 = !_5.1;
_6.1 = (-24025_i16) as i32;
Goto(bb8)
}
bb8 = {
_11.1 = (_2, (*_16));
_3 = (-16_i8) as i128;
_18 = [_8,_10.1,(*_16),(*_16)];
_11.2 = -_11.1.0;
_19 = [_8,_10.1,_13.fld0,_13.fld0];
_5.0 = 13935_u16 as i128;
_8 = !_10.1;
_7 = _10.0 * _11.2;
_21 = [13426_i16,21647_i16,(-2160_i16),(-2466_i16),3517_i16];
_13.fld0 = (-12_i8) as i32;
_4 = -_2;
_6 = _5;
_10 = (_11.2, _5.1);
_18 = _19;
_17 = '\u{74a1b}';
_10.0 = 4_isize as i128;
_10.0 = 57_u8 as i128;
_14 = 3804373275956434618_u64;
_1 = _2 >> _8;
_2 = 21_isize as i128;
_19 = _18;
_8 = !_6.1;
match _14 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
4 => bb9,
5 => bb10,
3804373275956434618 => bb12,
_ => bb11
}
}
bb9 = {
_13.fld1 = 92_i8 as u32;
_6.1 = _10.1 - _5.1;
_10.1 = _6.1;
_11.1 = (_6.0, _6.1);
_5 = (_2, _10.1);
_11.1.1 = !_6.1;
_10 = (_2, _6.1);
_6.0 = _3 ^ _7;
_2 = _10.0 >> _10.1;
_14 = 17821120738914462227_u64;
RET = !96935995669547226940089144189382771539_u128;
_11.1.1 = _10.1;
_8 = _5.1 ^ _5.1;
_15 = [_2,_5.0,_10.0];
_4 = _1;
RET = 35440602080959973619185324666451996101_u128 | 158294940280963175666963363025084901575_u128;
_11.1 = (_3, _10.1);
Goto(bb2)
}
bb10 = {
_13.fld1 = 92_i8 as u32;
_6.1 = _10.1 - _5.1;
_10.1 = _6.1;
_11.1 = (_6.0, _6.1);
_5 = (_2, _10.1);
_11.1.1 = !_6.1;
_10 = (_2, _6.1);
_6.0 = _3 ^ _7;
_2 = _10.0 >> _10.1;
_14 = 17821120738914462227_u64;
RET = !96935995669547226940089144189382771539_u128;
_11.1.1 = _10.1;
_8 = _5.1 ^ _5.1;
_15 = [_2,_5.0,_10.0];
_4 = _1;
RET = 35440602080959973619185324666451996101_u128 | 158294940280963175666963363025084901575_u128;
_11.1 = (_3, _10.1);
Goto(bb2)
}
bb11 = {
_13.fld2 = [124_u8,11_u8,79_u8,113_u8];
_6 = (_4, _5.1);
_6.0 = 55823_u16 as i128;
_11.0 = 10963_u16 as f64;
Call(_1 = fn18(_7, _8, _13.fld0, _10, _5.1, _2, _5.1, _15, _13.fld0, _5.0, _15, _13.fld0, _10.1, _4, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb12 = {
_7 = _11.2;
_7 = _1 >> _5.1;
_24 = _14 / _14;
_11.2 = _11.1.0;
_11.0 = _13.fld1 as f64;
_8 = !_5.1;
_25 = _15;
_13.fld2 = [47_u8,97_u8,242_u8,75_u8];
_25 = [_11.2,_11.2,_7];
_22 = true;
_5 = _10;
_19 = _18;
_6.0 = RET as i128;
_2 = !_11.2;
_9 = !_13.fld1;
_6 = (_11.2, _11.1.1);
_16 = core::ptr::addr_of!((*_16));
_26 = (-9206010383081900310_i64);
_19 = [_6.1,_5.1,(*_16),_8];
_10.1 = (*_16) ^ _11.1.1;
_7 = _4;
_8 = _22 as i32;
_4 = _11.2 - _7;
match _26 {
0 => bb6,
1 => bb9,
2 => bb3,
3 => bb13,
340282366920938463454168597048686311146 => bb15,
_ => bb14
}
}
bb13 = {
_13.fld1 = 92_i8 as u32;
_6.1 = _10.1 - _5.1;
_10.1 = _6.1;
_11.1 = (_6.0, _6.1);
_5 = (_2, _10.1);
_11.1.1 = !_6.1;
_10 = (_2, _6.1);
_6.0 = _3 ^ _7;
_2 = _10.0 >> _10.1;
_14 = 17821120738914462227_u64;
RET = !96935995669547226940089144189382771539_u128;
_11.1.1 = _10.1;
_8 = _5.1 ^ _5.1;
_15 = [_2,_5.0,_10.0];
_4 = _1;
RET = 35440602080959973619185324666451996101_u128 | 158294940280963175666963363025084901575_u128;
_11.1 = (_3, _10.1);
Goto(bb2)
}
bb14 = {
_10.0 = _11.2 << _6.1;
_5.0 = false as i128;
_11.1.1 = !_5.1;
_6.1 = (-24025_i16) as i32;
Goto(bb8)
}
bb15 = {
_14 = _24 ^ _24;
_26 = 7603728990466135339_i64 | 2423173534846037585_i64;
_20 = -(-9223372036854775808_isize);
_11.1 = _10;
_10 = (_1, _6.1);
_11.1.0 = !_7;
_11.1 = (_4, (*_16));
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(17_usize, 26_usize, Move(_26), 20_usize, Move(_20), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(17_usize, 4_usize, Move(_4), 9_usize, Move(_9), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(17_usize, 1_usize, Move(_1), 3_usize, Move(_3), 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i128,mut _2: i32,mut _3: i32,mut _4: (i128, i32),mut _5: i32,mut _6: i128,mut _7: i32,mut _8: [i128; 3],mut _9: i32,mut _10: i128,mut _11: [i128; 3],mut _12: i32,mut _13: i32,mut _14: i128,mut _15: i128) -> i128 {
mir! {
type RET = i128;
let _16: isize;
let _17: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64);
let _18: f32;
let _19: bool;
let _20: Adt47;
let _21: ();
let _22: ();
{
_5 = _9 >> _3;
Goto(bb1)
}
bb1 = {
RET = !_1;
_3 = 9223372036854775807_isize as i32;
_4.1 = _9;
_10 = (-28196_i16) as i128;
_2 = _4.1 ^ _13;
RET = !_6;
_17.1 = (_4, _8, (-73_i8));
_13 = _4.1;
_3 = 2425188070_u32 as i32;
_17.3 = 242749146088956237754789981831401522355_u128 | 140855376593769723654706113444332651412_u128;
_17.1.0.1 = _7;
_9 = !_5;
_11 = _8;
_9 = _12 ^ _4.1;
_17.2 = _17.3;
_17.3 = !_17.2;
_19 = !true;
_3 = 1_usize as i32;
_17.0 = 4321877542598435430_usize + 15765546205844469965_usize;
_8 = [_14,_17.1.0.0,_6];
_11 = [_1,_4.0,_17.1.0.0];
_1 = _17.3 as i128;
_16 = 120_u8 as isize;
_3 = (-5733227037181780879_i64) as i32;
_18 = 46014220_u32 as f32;
_17.4 = 391493818_u32 as f64;
_13 = -_5;
Goto(bb2)
}
bb2 = {
Call(_21 = dump_var(18_usize, 6_usize, Move(_6), 8_usize, Move(_8), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_21 = dump_var(18_usize, 2_usize, Move(_2), 3_usize, Move(_3), 19_usize, Move(_19), 12_usize, Move(_12)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i128,mut _2: i128,mut _3: (i128, i32),mut _4: (i128, i32),mut _5: (i128, i32),mut _6: i128,mut _7: i128,mut _8: (i128, i32),mut _9: (i128, i32),mut _10: i128,mut _11: (i128, i32)) -> [i128; 3] {
mir! {
type RET = [i128; 3];
let _12: u64;
let _13: ();
let _14: ();
{
RET = [_8.0,_2,_3.0];
_8 = (_3.0, _9.1);
_3.0 = _4.1 as i128;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(19_usize, 2_usize, Move(_2), 4_usize, Move(_4), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(19_usize, 8_usize, Move(_8), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{5d4c4}'), std::hint::black_box((-55_isize)), std::hint::black_box((-105_i8)), std::hint::black_box((-28519_i16)), std::hint::black_box(11003452709417864751_u64), std::hint::black_box(3094764027933215660_i64), std::hint::black_box((-119147334576538807220646737500278899478_i128)), std::hint::black_box(0_usize), std::hint::black_box(250_u8), std::hint::black_box(32029_u16), std::hint::black_box(2274496219_u32));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: (*mut [i128; 3],),
fld1: *mut [i128; 3],
fld2: [i128; 4],
fld3: f32,
fld4: u64,
fld5: u128,
fld6: i64,
fld7: *const i32,
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut [i128; 3],
fld1: [i128; 4],

},
Variant1{
fld0: [i16; 5],
fld1: ((i128, i32), [i128; 3], i8),
fld2: [i128; 4],
fld3: i8,
fld4: [i128; 3],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: i32,
fld1: u32,
fld2: [u8; 4],
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const i32,
fld1: (i128, i32),

},
Variant1{
fld0: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),
fld1: (*mut [i128; 3],),
fld2: [bool; 7],
fld3: *const i32,
fld4: i16,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: bool,
fld1: i32,
fld2: isize,
fld3: (f64, (i128, i32), i128),

},
Variant1{
fld0: (f64, (i128, i32), i128),
fld1: Adt47,
fld2: *const i32,

},
Variant2{
fld0: *const u8,
fld1: *const u16,
fld2: Adt44,

},
Variant3{
fld0: usize,
fld1: char,
fld2: *mut ((i128, i32), [i128; 3], i8),
fld3: Adt46,
fld4: i16,
fld5: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),
fld6: [bool; 7],
fld7: [i16; 5],

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt47,
fld1: (*mut [i128; 3],),
fld2: u128,
fld3: i8,
fld4: [i128; 4],
fld5: [i128; 3],
fld6: (u32, *const i32, *const u16),
fld7: u8,

},
Variant1{
fld0: (isize, bool, u32),
fld1: Adt46,
fld2: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),
fld3: (f64, (i128, i32), i128),
fld4: i16,
fld5: u16,

},
Variant2{
fld0: Adt48,
fld1: u8,
fld2: (isize, bool, u32),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: Adt46,
fld1: Adt49,
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (*mut [i128; 3],),
fld1: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),
fld2: Adt50,

},
Variant1{
fld0: *mut u16,
fld1: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32),
fld2: (u32, *const i32, *const u16),
fld3: u128,

},
Variant2{
fld0: (f64, (i128, i32), i128),

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: ((i128, i32), [i128; 3], i8),
fld1: u64,
fld2: *mut ((i128, i32), [i128; 3], i8),
fld3: i8,
fld4: [i128; 3],
fld5: [i128; 4],
fld6: i64,
fld7: u128,

},
Variant1{
fld0: u64,
fld1: Adt44,
fld2: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),
fld3: Adt51,
fld4: f64,
fld5: [i32; 4],
fld6: Adt50,
fld7: [i16; 5],

},
Variant2{
fld0: Adt45,
fld1: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64),
fld2: *mut u16,
fld3: f64,
fld4: Adt44,
fld5: *mut ((i128, i32), [i128; 3], i8),
fld6: *const u16,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [bool; 7],

},
Variant1{
fld0: [i16; 5],
fld1: i64,
fld2: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32),
fld3: (f64, (i128, i32), i128),
fld4: u64,
fld5: i128,

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: char,
fld2: (f64, (i128, i32), i128),
fld3: [i16; 5],
fld4: i16,
fld5: [bool; 7],
fld6: Adt46,

},
Variant1{
fld0: f64,
fld1: (usize, ((i128, i32), [i128; 3], i8), u128, u128, f64),
fld2: ((i128, i32), [i128; 3], i8),
fld3: *mut [i128; 3],
fld4: u32,
fld5: *mut ((i128, i32), [i128; 3], i8),
fld6: Adt50,

},
Variant2{
fld0: *const i32,
fld1: usize,

},
Variant3{
fld0: i64,
fld1: [i128; 3],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [i128; 3],
fld1: f64,
fld2: isize,
fld3: *mut u16,
fld4: Adt54,
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [bool; 7],
fld1: char,
fld2: Adt54,
fld3: f32,
fld4: u8,

},
Variant1{
fld0: *const i32,
fld1: Adt45,
fld2: u16,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *const u16,
fld1: [bool; 7],
fld2: isize,
fld3: i32,
fld4: (u32, *const i32, *const u16),

},
Variant1{
fld0: (i128, i32),

},
Variant2{
fld0: Adt47,
fld1: *mut u16,
fld2: isize,
fld3: [i128; 3],
fld4: [i128; 4],
fld5: *const u8,
fld6: u8,

},
Variant3{
fld0: ((i128, i32), [i128; 3], i8),
fld1: char,
fld2: *mut [i128; 3],
fld3: u16,
fld4: f64,
fld5: *const u16,
fld6: Adt44,

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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: i64,
fld1: (*mut [i128; 3],),
fld2: i128,
fld3: Adt48,

},
Variant1{
fld0: [u8; 4],
fld1: char,
fld2: (f64, (i128, i32), i128),
fld3: [bool; 7],
fld4: (isize, bool, u32),
fld5: Adt55,
fld6: (*mut [i128; 3],),
fld7: i128,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: bool,
fld1: u8,
fld2: (bool, (f64, (i128, i32), i128), [i16; 5], i32, (u32, *const i32, *const u16)),
fld3: *const u8,
fld4: Adt45,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: (isize, usize, *const i32, u8, (i128, i32), (f64, (i128, i32), i128), i32),
fld1: f32,
fld2: Adt49,
fld3: (f64, (i128, i32), i128),
fld4: f64,
fld5: *const u8,
fld6: Adt45,
}

