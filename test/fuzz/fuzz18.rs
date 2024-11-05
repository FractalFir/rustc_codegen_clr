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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> i8 {
mir! {
type RET = i8;
let _15: (usize,);
let _16: isize;
let _17: isize;
let _18: ([isize; 2],);
let _19: Adt56;
let _20: u64;
let _21: u32;
let _22: [char; 2];
let _23: char;
let _24: isize;
let _25: *const char;
let _26: bool;
let _27: (bool, i32, i32);
let _28: Adt47;
let _29: [char; 2];
let _30: [i64; 8];
let _31: (char,);
let _32: [u8; 5];
let _33: ([usize; 3], *mut i128, f64);
let _34: u128;
let _35: [u8; 5];
let _36: [i8; 1];
let _37: f32;
let _38: bool;
let _39: isize;
let _40: (char,);
let _41: ();
let _42: ();
{
_6 = 126507400353240027161832165942605876938_u128 as i32;
_13 = 11770152992396905300_u64 >> _6;
_3 = 9223372036854775807_isize;
RET = 115_i8;
Call(_12 = core::intrinsics::bswap(1367021752_u32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = RET * RET;
_15.0 = !3_usize;
_13 = !16526173206250178085_u64;
_3 = 9223372036854775807_isize ^ (-85_isize);
RET = _4;
RET = -_4;
_15 = (14414654326199875413_usize,);
_11 = _13 as u16;
RET = _4 << _13;
_12 = _13 as u32;
_11 = (-293_i16) as u16;
_1 = true;
_3 = RET as isize;
_11 = _3 as u16;
_13 = 15276049968370712968_u64;
_12 = 2356307099_u32 | 432000675_u32;
_9 = 167468772612822260936673284325583041025_u128 as usize;
_12 = 3230163911_u32;
_14 = _13 as u128;
_10 = !238_u8;
_4 = RET;
_14 = (-125746055514030101061959852268778408635_i128) as u128;
_7 = _14 as i64;
_2 = '\u{2d75}';
Goto(bb2)
}
bb2 = {
_19.fld5.fld2 = [19189_i16];
_19.fld2 = [_12,_12,_12,_12,_12];
_20 = _13;
_18.0 = [_3,_3];
_15.0 = _9;
_18.0 = [_3,_3];
_7 = (-3125548865260586994_i64) ^ 3911347470750863616_i64;
Call(_19.fld1.1 = fn1(_18, _18.0, _18, _12, _3, _19.fld5.fld2, _13, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = !_3;
_19.fld0 = (-37690146257007159808936793820675652180_i128);
_10 = 79_u8;
_2 = '\u{fe58c}';
_19.fld1.0 = -2969_i16;
_5 = !_19.fld1.0;
_19.fld5.fld1 = core::ptr::addr_of_mut!(_13);
_19.fld0 = _10 as i128;
_9 = _15.0 + _15.0;
_12 = 1956050349_u32 | 3719733493_u32;
_8 = _1 as i128;
_19.fld1.2 = core::ptr::addr_of_mut!(_22);
_19.fld5.fld4 = _20 as i16;
_19.fld2 = [_12,_12,_12,_12,_12];
_20 = _9 as u64;
_19.fld4 = [_9,_9,_15.0];
_23 = _2;
_19.fld5.fld1 = core::ptr::addr_of_mut!(_13);
_6 = -1632125296_i32;
RET = _19.fld1.1;
_10 = 67_u8;
_9 = _15.0 & _15.0;
_23 = _2;
_1 = true;
_19.fld5.fld2 = [_5];
_14 = RET as u128;
Goto(bb4)
}
bb4 = {
_10 = 200_u8;
_27 = (_1, _6, _6);
_19.fld0 = _8;
_7 = (-17515570160991587_i64);
_12 = 4272575325_u32 * 504572784_u32;
_12 = 3830745089_u32;
_1 = _27.0;
_18.0 = [_3,_3];
match _13 {
0 => bb3,
1 => bb5,
2 => bb6,
15276049968370712968 => bb8,
_ => bb7
}
}
bb5 = {
_16 = !_3;
_19.fld0 = (-37690146257007159808936793820675652180_i128);
_10 = 79_u8;
_2 = '\u{fe58c}';
_19.fld1.0 = -2969_i16;
_5 = !_19.fld1.0;
_19.fld5.fld1 = core::ptr::addr_of_mut!(_13);
_19.fld0 = _10 as i128;
_9 = _15.0 + _15.0;
_12 = 1956050349_u32 | 3719733493_u32;
_8 = _1 as i128;
_19.fld1.2 = core::ptr::addr_of_mut!(_22);
_19.fld5.fld4 = _20 as i16;
_19.fld2 = [_12,_12,_12,_12,_12];
_20 = _9 as u64;
_19.fld4 = [_9,_9,_15.0];
_23 = _2;
_19.fld5.fld1 = core::ptr::addr_of_mut!(_13);
_6 = -1632125296_i32;
RET = _19.fld1.1;
_10 = 67_u8;
_9 = _15.0 & _15.0;
_23 = _2;
_1 = true;
_19.fld5.fld2 = [_5];
_14 = RET as u128;
Goto(bb4)
}
bb6 = {
_19.fld5.fld2 = [19189_i16];
_19.fld2 = [_12,_12,_12,_12,_12];
_20 = _13;
_18.0 = [_3,_3];
_15.0 = _9;
_18.0 = [_3,_3];
_7 = (-3125548865260586994_i64) ^ 3911347470750863616_i64;
Call(_19.fld1.1 = fn1(_18, _18.0, _18, _12, _3, _19.fld5.fld2, _13, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4 = RET * RET;
_15.0 = !3_usize;
_13 = !16526173206250178085_u64;
_3 = 9223372036854775807_isize ^ (-85_isize);
RET = _4;
RET = -_4;
_15 = (14414654326199875413_usize,);
_11 = _13 as u16;
RET = _4 << _13;
_12 = _13 as u32;
_11 = (-293_i16) as u16;
_1 = true;
_3 = RET as isize;
_11 = _3 as u16;
_13 = 15276049968370712968_u64;
_12 = 2356307099_u32 | 432000675_u32;
_9 = 167468772612822260936673284325583041025_u128 as usize;
_12 = 3230163911_u32;
_14 = _13 as u128;
_10 = !238_u8;
_4 = RET;
_14 = (-125746055514030101061959852268778408635_i128) as u128;
_7 = _14 as i64;
_2 = '\u{2d75}';
Goto(bb2)
}
bb8 = {
_21 = _12;
_19.fld5.fld2 = [_19.fld5.fld4];
_5 = _19.fld5.fld4 & _19.fld1.0;
_22 = [_23,_23];
_9 = _15.0 * _15.0;
RET = _19.fld1.1;
_16 = _1 as isize;
_15 = (_9,);
_13 = _6 as u64;
_18.0 = [_16,_3];
_19.fld4 = [_9,_15.0,_9];
_12 = !_21;
match _21 {
0 => bb3,
1 => bb6,
3830745089 => bb10,
_ => bb9
}
}
bb9 = {
_10 = 200_u8;
_27 = (_1, _6, _6);
_19.fld0 = _8;
_7 = (-17515570160991587_i64);
_12 = 4272575325_u32 * 504572784_u32;
_12 = 3830745089_u32;
_1 = _27.0;
_18.0 = [_3,_3];
match _13 {
0 => bb3,
1 => bb5,
2 => bb6,
15276049968370712968 => bb8,
_ => bb7
}
}
bb10 = {
_27.2 = _27.1;
_17 = -_3;
_33.1 = core::ptr::addr_of_mut!(_8);
_27.0 = _3 >= _17;
_3 = _1 as isize;
_22 = [_23,_23];
_19.fld5.fld4 = _19.fld1.0;
_33.1 = core::ptr::addr_of_mut!(_19.fld0);
_7 = _11 as i64;
_19.fld4 = [_15.0,_9,_9];
RET = _19.fld1.1;
_25 = core::ptr::addr_of!(_2);
_19.fld1.1 = _19.fld0 as i8;
_20 = _13;
_31.0 = (*_25);
_33.2 = _13 as f64;
_27.2 = _7 as i32;
match _21 {
0 => bb11,
3830745089 => bb13,
_ => bb12
}
}
bb11 = {
_16 = !_3;
_19.fld0 = (-37690146257007159808936793820675652180_i128);
_10 = 79_u8;
_2 = '\u{fe58c}';
_19.fld1.0 = -2969_i16;
_5 = !_19.fld1.0;
_19.fld5.fld1 = core::ptr::addr_of_mut!(_13);
_19.fld0 = _10 as i128;
_9 = _15.0 + _15.0;
_12 = 1956050349_u32 | 3719733493_u32;
_8 = _1 as i128;
_19.fld1.2 = core::ptr::addr_of_mut!(_22);
_19.fld5.fld4 = _20 as i16;
_19.fld2 = [_12,_12,_12,_12,_12];
_20 = _9 as u64;
_19.fld4 = [_9,_9,_15.0];
_23 = _2;
_19.fld5.fld1 = core::ptr::addr_of_mut!(_13);
_6 = -1632125296_i32;
RET = _19.fld1.1;
_10 = 67_u8;
_9 = _15.0 & _15.0;
_23 = _2;
_1 = true;
_19.fld5.fld2 = [_5];
_14 = RET as u128;
Goto(bb4)
}
bb12 = {
_19.fld5.fld2 = [19189_i16];
_19.fld2 = [_12,_12,_12,_12,_12];
_20 = _13;
_18.0 = [_3,_3];
_15.0 = _9;
_18.0 = [_3,_3];
_7 = (-3125548865260586994_i64) ^ 3911347470750863616_i64;
Call(_19.fld1.1 = fn1(_18, _18.0, _18, _12, _3, _19.fld5.fld2, _13, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_32 = [_10,_10,_10,_10,_10];
Goto(bb14)
}
bb14 = {
_33.0 = [_15.0,_15.0,_15.0];
_33.2 = _14 as f64;
RET = _10 as i8;
_11 = 27027_u16;
_29 = [(*_25),(*_25)];
_28.fld0 = _33.1;
_29 = _22;
_28 = Adt47 { fld0: _33.1 };
_18.0 = [_17,_3];
_2 = _31.0;
RET = _4 | _4;
_13 = _20;
_1 = !_27.0;
_33.0 = [_15.0,_9,_9];
RET = _19.fld1.1 - _4;
_34 = _33.2 as u128;
_16 = _17 << _20;
_28.fld0 = _33.1;
_40.0 = _31.0;
_15.0 = _27.2 as usize;
_26 = _1 > _1;
_17 = _16 & _16;
_30 = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = 53_u8 >> _4;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(0_usize, 22_usize, Move(_22), 30_usize, Move(_30), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(0_usize, 21_usize, Move(_21), 9_usize, Move(_9), 13_usize, Move(_13), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(0_usize, 26_usize, Move(_26), 12_usize, Move(_12), 4_usize, Move(_4), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(0_usize, 1_usize, Move(_1), 14_usize, Move(_14), 23_usize, Move(_23), 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: ([isize; 2],),mut _2: [isize; 2],mut _3: ([isize; 2],),mut _4: u32,mut _5: isize,mut _6: [i16; 1],mut _7: u64,mut _8: u8) -> i8 {
mir! {
type RET = i8;
let _9: usize;
let _10: f64;
let _11: [u32; 5];
let _12: usize;
let _13: f32;
let _14: i64;
let _15: f64;
let _16: *const char;
let _17: Adt43;
let _18: f64;
let _19: (*mut i128,);
let _20: [i16; 1];
let _21: (i16, i8, *mut [char; 2]);
let _22: [i64; 8];
let _23: isize;
let _24: isize;
let _25: u128;
let _26: *const char;
let _27: Adt44;
let _28: ([isize; 2],);
let _29: (usize,);
let _30: usize;
let _31: bool;
let _32: i32;
let _33: i8;
let _34: ([isize; 2],);
let _35: isize;
let _36: [i8; 1];
let _37: [i8; 1];
let _38: f32;
let _39: usize;
let _40: ([isize; 2],);
let _41: isize;
let _42: i16;
let _43: u128;
let _44: u8;
let _45: isize;
let _46: *const char;
let _47: u16;
let _48: Adt52;
let _49: [u8; 5];
let _50: [u8; 5];
let _51: (*mut i128,);
let _52: u16;
let _53: (usize,);
let _54: ();
let _55: ();
{
_8 = 172_u8;
RET = 119_i8;
_4 = _7 as u32;
_4 = _7 as u32;
RET = false as i8;
_3 = (_1.0,);
RET = 1_usize as i8;
_8 = (-3167148878717384092_i64) as u8;
_3.0 = [_5,_5];
RET = _4 as i8;
RET = -76_i8;
_4 = !52488614_u32;
_5 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_4 = 2943101109_u32 << _7;
Goto(bb1)
}
bb1 = {
_7 = 5941394757709058938_u64;
_2 = [_5,_5];
_3 = _1;
_3.0 = [_5,_5];
_3.0 = _1.0;
_2 = _3.0;
_7 = _4 as u64;
Call(_8 = fn2(_3, _1, _5, _5, _2, _1, _5, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-25164_i16) as i8;
RET = (-8_i8);
_9 = 7_usize - 0_usize;
_8 = 299078601037727276201151865090925184385_u128 as u8;
_1 = (_2,);
_8 = !60_u8;
_4 = (-897139905_i32) as u32;
_4 = !2656550006_u32;
RET = 65_i8;
_2 = _1.0;
_4 = !3663166567_u32;
_11 = [_4,_4,_4,_4,_4];
_5 = 88_isize - (-9223372036854775808_isize);
_10 = _4 as f64;
_11 = [_4,_4,_4,_4,_4];
_2 = [_5,_5];
match RET {
65 => bb3,
_ => bb1
}
}
bb3 = {
_13 = RET as f32;
_8 = 34_u8;
_9 = !12453032809032766627_usize;
_1.0 = _3.0;
_3 = (_1.0,);
_1 = (_2,);
RET = _10 as i8;
_3.0 = [_5,_5];
_12 = _9 * _9;
RET = -78_i8;
_10 = _5 as f64;
_4 = 198528640_u32 >> _7;
RET = 94_i8;
_4 = !1814526160_u32;
_7 = 10957407549463584123_u64;
_3.0 = [_5,_5];
_3.0 = [_5,_5];
_15 = _10 + _10;
_1.0 = [_5,_5];
_13 = _15 as f32;
_8 = 138_u8 * 119_u8;
_9 = _12 >> _8;
RET = 102_i8 + 36_i8;
_1.0 = [_5,_5];
_7 = 2872661476745460726_u64 & 14615312678365639674_u64;
_1 = (_2,);
_4 = !2963796676_u32;
_10 = _15 * _15;
Goto(bb4)
}
bb4 = {
_2 = [_5,_5];
_1.0 = _2;
_6 = [25390_i16];
RET = 76_i8;
_3 = (_2,);
_15 = -_10;
_10 = _15 * _15;
_6 = [10850_i16];
_3 = (_2,);
_1.0 = [_5,_5];
_5 = (-4_isize) * 9223372036854775807_isize;
_13 = (-31696_i16) as f32;
_15 = _10 + _10;
_15 = _10 * _10;
_8 = 20_u8;
Goto(bb5)
}
bb5 = {
_18 = -_10;
_13 = 18079_i16 as f32;
_1.0 = _3.0;
_13 = _4 as f32;
_10 = -_18;
_18 = -_10;
_21.1 = (-9545_i16) as i8;
_11 = [_4,_4,_4,_4,_4];
_23 = _7 as isize;
_5 = _7 as isize;
_25 = 221972932491537682796195376853770201753_u128;
_2 = [_5,_23];
_9 = !_12;
_22 = [(-6889266031039476219_i64),(-1674559553626685898_i64),6548071770937034020_i64,(-4528600655171881361_i64),(-6388190061712064259_i64),3591581679036521990_i64,(-532551668672638464_i64),(-632043303614034921_i64)];
_7 = 7853647451957183512_u64 * 12873639253091150679_u64;
_23 = _5 ^ _5;
_10 = _15;
_21.1 = RET ^ RET;
Goto(bb6)
}
bb6 = {
_18 = -_10;
_4 = _25 as u32;
_22 = [93368025933528439_i64,(-2384614829695012366_i64),7820294077587083727_i64,(-1406415426718646422_i64),(-4592617369381187861_i64),(-8643562242047571256_i64),(-8882664971010901203_i64),(-6204235014479204652_i64)];
_20 = [(-12650_i16)];
_14 = !(-2252181124877936036_i64);
_23 = -_5;
_15 = -_18;
_8 = !0_u8;
_21.1 = RET << _5;
_22 = [_14,_14,_14,_14,_14,_14,_14,_14];
_20 = [27556_i16];
_6 = _20;
_18 = _10 - _15;
_21.0 = -30315_i16;
_8 = !6_u8;
_4 = 60163875011816160938141271099413423696_i128 as u32;
_25 = 320223942003283545294513368284710239615_u128 * 225152344792273303695595797253710553305_u128;
RET = _21.1;
Call(_13 = fn18(_18, _3, _2, _18, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = !3775446843_u32;
_7 = 6473446792809595945_u64;
_29 = (_12,);
_3.0 = [_23,_5];
_13 = _29.0 as f32;
_2 = [_5,_23];
_4 = _21.0 as u32;
_8 = _5 as u8;
_31 = true;
_30 = _29.0 + _12;
_10 = _15;
_14 = (-8844624441517435134_i64);
_34 = (_1.0,);
_29.0 = !_9;
_28 = _34;
_8 = !178_u8;
_24 = _5;
_32 = -(-2049952172_i32);
_5 = -_23;
Call(_33 = core::intrinsics::transmute(_31), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14 = !6148065054156213048_i64;
_25 = 142620469367501785854427089467891223437_u128;
_4 = 1968378129_u32;
_29 = (_9,);
_29 = (_30,);
_10 = -_18;
_29.0 = _30 | _30;
_1 = (_2,);
_6 = [_21.0];
_37 = [RET];
_31 = true;
_31 = false;
_29 = (_30,);
_3.0 = [_5,_23];
match _25 {
0 => bb4,
1 => bb2,
2 => bb7,
142620469367501785854427089467891223437 => bb10,
_ => bb9
}
}
bb9 = {
_18 = -_10;
_4 = _25 as u32;
_22 = [93368025933528439_i64,(-2384614829695012366_i64),7820294077587083727_i64,(-1406415426718646422_i64),(-4592617369381187861_i64),(-8643562242047571256_i64),(-8882664971010901203_i64),(-6204235014479204652_i64)];
_20 = [(-12650_i16)];
_14 = !(-2252181124877936036_i64);
_23 = -_5;
_15 = -_18;
_8 = !0_u8;
_21.1 = RET << _5;
_22 = [_14,_14,_14,_14,_14,_14,_14,_14];
_20 = [27556_i16];
_6 = _20;
_18 = _10 - _15;
_21.0 = -30315_i16;
_8 = !6_u8;
_4 = 60163875011816160938141271099413423696_i128 as u32;
_25 = 320223942003283545294513368284710239615_u128 * 225152344792273303695595797253710553305_u128;
RET = _21.1;
Call(_13 = fn18(_18, _3, _2, _18, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb10 = {
_9 = !_29.0;
_6 = [_21.0];
_35 = _24;
_21.1 = !_33;
_41 = _24;
_7 = _32 as u64;
_36 = [_21.1];
_34.0 = [_35,_23];
_34 = (_3.0,);
_42 = -_21.0;
_3.0 = [_35,_35];
_14 = (-6802569336307883767_i64) << _35;
_28 = (_34.0,);
_38 = -_13;
_3 = (_2,);
_40.0 = [_41,_41];
_35 = -_23;
_29.0 = _21.1 as usize;
_34.0 = _3.0;
_3 = (_28.0,);
_23 = _41 - _41;
_5 = RET as isize;
_36 = [_33];
Goto(bb11)
}
bb11 = {
_42 = !_21.0;
_9 = _30 | _30;
_42 = _14 as i16;
_44 = !_8;
_25 = !304655931130280006127132347032627985721_u128;
_39 = _18 as usize;
_40.0 = [_23,_35];
_1 = _28;
_25 = _38 as u128;
_14 = (-7123509030397326756_i64);
Goto(bb12)
}
bb12 = {
_37 = _36;
_29 = (_39,);
_28 = (_34.0,);
_24 = _5;
_43 = _25 >> _39;
_35 = _23 & _5;
_34 = (_2,);
_1 = (_28.0,);
match _4 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
1968378129 => bb21,
_ => bb20
}
}
bb13 = {
_42 = !_21.0;
_9 = _30 | _30;
_42 = _14 as i16;
_44 = !_8;
_25 = !304655931130280006127132347032627985721_u128;
_39 = _18 as usize;
_40.0 = [_23,_35];
_1 = _28;
_25 = _38 as u128;
_14 = (-7123509030397326756_i64);
Goto(bb12)
}
bb14 = {
_9 = !_29.0;
_6 = [_21.0];
_35 = _24;
_21.1 = !_33;
_41 = _24;
_7 = _32 as u64;
_36 = [_21.1];
_34.0 = [_35,_23];
_34 = (_3.0,);
_42 = -_21.0;
_3.0 = [_35,_35];
_14 = (-6802569336307883767_i64) << _35;
_28 = (_34.0,);
_38 = -_13;
_3 = (_2,);
_40.0 = [_41,_41];
_35 = -_23;
_29.0 = _21.1 as usize;
_34.0 = _3.0;
_3 = (_28.0,);
_23 = _41 - _41;
_5 = RET as isize;
_36 = [_33];
Goto(bb11)
}
bb15 = {
_18 = -_10;
_4 = _25 as u32;
_22 = [93368025933528439_i64,(-2384614829695012366_i64),7820294077587083727_i64,(-1406415426718646422_i64),(-4592617369381187861_i64),(-8643562242047571256_i64),(-8882664971010901203_i64),(-6204235014479204652_i64)];
_20 = [(-12650_i16)];
_14 = !(-2252181124877936036_i64);
_23 = -_5;
_15 = -_18;
_8 = !0_u8;
_21.1 = RET << _5;
_22 = [_14,_14,_14,_14,_14,_14,_14,_14];
_20 = [27556_i16];
_6 = _20;
_18 = _10 - _15;
_21.0 = -30315_i16;
_8 = !6_u8;
_4 = 60163875011816160938141271099413423696_i128 as u32;
_25 = 320223942003283545294513368284710239615_u128 * 225152344792273303695595797253710553305_u128;
RET = _21.1;
Call(_13 = fn18(_18, _3, _2, _18, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb16 = {
_14 = !6148065054156213048_i64;
_25 = 142620469367501785854427089467891223437_u128;
_4 = 1968378129_u32;
_29 = (_9,);
_29 = (_30,);
_10 = -_18;
_29.0 = _30 | _30;
_1 = (_2,);
_6 = [_21.0];
_37 = [RET];
_31 = true;
_31 = false;
_29 = (_30,);
_3.0 = [_5,_23];
match _25 {
0 => bb4,
1 => bb2,
2 => bb7,
142620469367501785854427089467891223437 => bb10,
_ => bb9
}
}
bb17 = {
_7 = 5941394757709058938_u64;
_2 = [_5,_5];
_3 = _1;
_3.0 = [_5,_5];
_3.0 = _1.0;
_2 = _3.0;
_7 = _4 as u64;
Call(_8 = fn2(_3, _1, _5, _5, _2, _1, _5, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_13 = RET as f32;
_8 = 34_u8;
_9 = !12453032809032766627_usize;
_1.0 = _3.0;
_3 = (_1.0,);
_1 = (_2,);
RET = _10 as i8;
_3.0 = [_5,_5];
_12 = _9 * _9;
RET = -78_i8;
_10 = _5 as f64;
_4 = 198528640_u32 >> _7;
RET = 94_i8;
_4 = !1814526160_u32;
_7 = 10957407549463584123_u64;
_3.0 = [_5,_5];
_3.0 = [_5,_5];
_15 = _10 + _10;
_1.0 = [_5,_5];
_13 = _15 as f32;
_8 = 138_u8 * 119_u8;
_9 = _12 >> _8;
RET = 102_i8 + 36_i8;
_1.0 = [_5,_5];
_7 = 2872661476745460726_u64 & 14615312678365639674_u64;
_1 = (_2,);
_4 = !2963796676_u32;
_10 = _15 * _15;
Goto(bb4)
}
bb19 = {
_18 = -_10;
_13 = 18079_i16 as f32;
_1.0 = _3.0;
_13 = _4 as f32;
_10 = -_18;
_18 = -_10;
_21.1 = (-9545_i16) as i8;
_11 = [_4,_4,_4,_4,_4];
_23 = _7 as isize;
_5 = _7 as isize;
_25 = 221972932491537682796195376853770201753_u128;
_2 = [_5,_23];
_9 = !_12;
_22 = [(-6889266031039476219_i64),(-1674559553626685898_i64),6548071770937034020_i64,(-4528600655171881361_i64),(-6388190061712064259_i64),3591581679036521990_i64,(-532551668672638464_i64),(-632043303614034921_i64)];
_7 = 7853647451957183512_u64 * 12873639253091150679_u64;
_23 = _5 ^ _5;
_10 = _15;
_21.1 = RET ^ RET;
Goto(bb6)
}
bb20 = {
_2 = [_5,_5];
_1.0 = _2;
_6 = [25390_i16];
RET = 76_i8;
_3 = (_2,);
_15 = -_10;
_10 = _15 * _15;
_6 = [10850_i16];
_3 = (_2,);
_1.0 = [_5,_5];
_5 = (-4_isize) * 9223372036854775807_isize;
_13 = (-31696_i16) as f32;
_15 = _10 + _10;
_15 = _10 * _10;
_8 = 20_u8;
Goto(bb5)
}
bb21 = {
_1.0 = _2;
_10 = _18 + _18;
_6 = _20;
_23 = -_35;
_34.0 = [_35,_35];
_39 = _29.0 * _29.0;
_8 = _44;
_23 = _35 << _39;
_45 = _23 << _39;
_49 = [_8,_44,_8,_44,_44];
_31 = !true;
_10 = -_18;
_50 = [_8,_44,_8,_44,_8];
_28 = (_2,);
_7 = !6520096239914659896_u64;
_1 = _40;
_32 = 1939980901_i32;
_47 = 39859_u16;
_53 = (_39,);
Goto(bb22)
}
bb22 = {
Call(_54 = dump_var(1_usize, 45_usize, Move(_45), 43_usize, Move(_43), 34_usize, Move(_34), 8_usize, Move(_8)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_54 = dump_var(1_usize, 49_usize, Move(_49), 22_usize, Move(_22), 20_usize, Move(_20), 47_usize, Move(_47)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_54 = dump_var(1_usize, 50_usize, Move(_50), 28_usize, Move(_28), 40_usize, Move(_40), 24_usize, Move(_24)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_54 = dump_var(1_usize, 2_usize, Move(_2), 9_usize, Move(_9), 11_usize, Move(_11), 39_usize, Move(_39)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_54 = dump_var(1_usize, 6_usize, Move(_6), 42_usize, Move(_42), 37_usize, Move(_37), 55_usize, _55), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: ([isize; 2],),mut _2: ([isize; 2],),mut _3: isize,mut _4: isize,mut _5: [isize; 2],mut _6: ([isize; 2],),mut _7: isize,mut _8: isize) -> u8 {
mir! {
type RET = u8;
let _9: char;
let _10: u8;
let _11: u64;
let _12: isize;
let _13: [isize; 2];
let _14: Adt55;
let _15: bool;
let _16: [isize; 2];
let _17: i128;
let _18: char;
let _19: [char; 2];
let _20: [u32; 5];
let _21: Adt52;
let _22: [u8; 5];
let _23: *const char;
let _24: [u8; 5];
let _25: *const isize;
let _26: isize;
let _27: (i16, i8, *mut [char; 2]);
let _28: Adt43;
let _29: u8;
let _30: char;
let _31: [u8; 5];
let _32: (bool, i32, i32);
let _33: (char,);
let _34: (bool, i32, i32);
let _35: ();
let _36: ();
{
RET = !11_u8;
_8 = _7 - _3;
_4 = _7 >> _8;
_8 = 22347_u16 as isize;
_1.0 = _2.0;
Goto(bb1)
}
bb1 = {
_1.0 = [_4,_3];
_6 = _1;
_7 = _4;
_1 = _2;
_9 = '\u{a0738}';
_2.0 = _1.0;
_3 = !_4;
Call(_6.0 = fn3(_4, _7, _5, _2, _3, _2, _8, _2.0, _7, _7, _3, _3, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _3 - _3;
_1.0 = [_8,_7];
_10 = !RET;
_1 = _2;
_1.0 = [_3,_7];
_11 = 15091334306083956840_u64;
Goto(bb3)
}
bb3 = {
_2.0 = _1.0;
_2.0 = [_3,_3];
_4 = _7;
RET = _10 + _10;
_12 = _4;
_2.0 = [_7,_7];
_5 = [_8,_7];
_4 = _8 ^ _12;
_8 = 3415709310_u32 as isize;
_15 = _7 > _3;
_12 = _7;
Goto(bb4)
}
bb4 = {
_16 = [_4,_4];
_13 = [_4,_12];
RET = _10;
Call(_15 = fn4(_16, _12, _2, _5, _4, _4, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6.0 = _2.0;
_17 = -84477350250798557350006871591565442448_i128;
_16 = [_12,_12];
_9 = '\u{6de42}';
_10 = !RET;
_13 = _2.0;
_2 = (_5,);
_4 = 5_usize as isize;
_13 = [_12,_12];
_9 = '\u{b5c92}';
_3 = -_8;
_11 = 10329440308095620834_u64 ^ 13909019625486751529_u64;
_18 = _9;
_9 = _18;
_6 = (_1.0,);
_19 = [_9,_18];
Goto(bb6)
}
bb6 = {
_16 = [_12,_7];
_6 = _2;
_2.0 = [_7,_3];
_12 = _7 << _7;
_11 = 7474394550742092917_u64 * 1910074973733576039_u64;
_2.0 = _5;
_6 = _1;
_2 = (_1.0,);
_1.0 = _13;
_18 = _9;
_1.0 = [_12,_12];
_16 = [_12,_12];
Goto(bb7)
}
bb7 = {
RET = !_10;
_10 = RET;
RET = !_10;
_4 = _11 as isize;
_13 = [_12,_12];
_15 = true;
_1.0 = [_12,_7];
_20 = [1325810073_u32,2305248170_u32,1411340872_u32,172132197_u32,4078904691_u32];
_12 = _7;
_15 = false | true;
_9 = _18;
_17 = (-73607867170438368782536603421806177680_i128);
_3 = _7 & _7;
_12 = -_3;
_1.0 = _13;
_4 = 6351991357651662665_i64 as isize;
_8 = _4;
_4 = 9074730157845009540_i64 as isize;
RET = !_10;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
266674499750500094680838004009962033776 => bb9,
_ => bb8
}
}
bb8 = {
_2.0 = _1.0;
_2.0 = [_3,_3];
_4 = _7;
RET = _10 + _10;
_12 = _4;
_2.0 = [_7,_7];
_5 = [_8,_7];
_4 = _8 ^ _12;
_8 = 3415709310_u32 as isize;
_15 = _7 > _3;
_12 = _7;
Goto(bb4)
}
bb9 = {
_11 = 13712698476446649218_u64 | 1018416336563331101_u64;
_4 = _12;
_2 = (_16,);
_15 = true | false;
_6.0 = _1.0;
RET = _10 ^ _10;
_12 = _4;
_22 = [_10,RET,_10,RET,RET];
_12 = 2_usize as isize;
_13 = [_4,_3];
RET = _10;
_20 = [1629538095_u32,2133684980_u32,3577164374_u32,3811358880_u32,2154924196_u32];
_25 = core::ptr::addr_of!(_4);
_5 = [(*_25),_7];
Call(RET = fn6(_6.0, _13, _1, _4, _6, _3, _4, _22, _20, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = _10;
(*_25) = -_7;
_5 = [(*_25),_3];
(*_25) = _7 ^ _3;
_2.0 = _5;
_25 = core::ptr::addr_of!((*_25));
Goto(bb11)
}
bb11 = {
_17 = (-35436295802496021344336207844175126632_i128);
_15 = false;
_26 = (*_25) << (*_25);
_24 = [RET,RET,RET,RET,RET];
_20 = [1854982584_u32,940344355_u32,3339336205_u32,628347222_u32,604079168_u32];
_8 = _11 as isize;
_1.0 = [_4,_26];
_2.0 = [(*_25),_26];
_27.1 = 79_i8 >> (*_25);
_22 = [RET,_10,_10,RET,RET];
(*_25) = _15 as isize;
_1 = (_5,);
_22 = [_10,_10,RET,RET,_10];
_9 = _18;
_27.1 = -(-11_i8);
_17 = 138737967950681193534429034353967351671_i128;
_4 = _26;
_29 = !_10;
_20 = [1749864184_u32,1104944932_u32,3959655023_u32,3372440539_u32,1767494100_u32];
match _17 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
138737967950681193534429034353967351671 => bb12,
_ => bb8
}
}
bb12 = {
(*_25) = _26 - _26;
_9 = _18;
_23 = core::ptr::addr_of!(_18);
_24 = _22;
_9 = (*_23);
_24 = _22;
RET = !_29;
_7 = _27.1 as isize;
_26 = (*_25) & (*_25);
RET = 211421756881092429599864408000434556611_u128 as u8;
_2 = (_6.0,);
_27.0 = !(-27604_i16);
_2.0 = [_26,_26];
(*_23) = _9;
_27.2 = core::ptr::addr_of_mut!(_19);
_30 = (*_23);
_2.0 = _13;
(*_25) = _3;
_23 = core::ptr::addr_of!(_9);
_22 = [_29,_29,_10,RET,_10];
_6.0 = _2.0;
_4 = _26;
_4 = _7 >> _26;
_23 = core::ptr::addr_of!((*_23));
_20 = [2557562005_u32,1836951982_u32,2616004659_u32,2716895958_u32,1690090485_u32];
_19 = [(*_23),(*_23)];
match _17 {
138737967950681193534429034353967351671 => bb14,
_ => bb13
}
}
bb13 = {
_7 = _3 - _3;
_1.0 = [_8,_7];
_10 = !RET;
_1 = _2;
_1.0 = [_3,_7];
_11 = 15091334306083956840_u64;
Goto(bb3)
}
bb14 = {
_19 = [(*_23),_18];
_17 = (-135692815548907200296580586439129804087_i128) & (-147260164126648282100312125851739278392_i128);
_20 = [2521822642_u32,1802488529_u32,4084939663_u32,1809957635_u32,130260050_u32];
_10 = !RET;
_34.2 = 324715945_i32;
_20 = [4026149780_u32,3628394847_u32,3226593957_u32,3692812956_u32,3147161801_u32];
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(2_usize, 8_usize, Move(_8), 5_usize, Move(_5), 2_usize, Move(_2), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(2_usize, 3_usize, Move(_3), 7_usize, Move(_7), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(2_usize, 12_usize, Move(_12), 22_usize, Move(_22), 10_usize, Move(_10), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: isize,mut _3: [isize; 2],mut _4: ([isize; 2],),mut _5: isize,mut _6: ([isize; 2],),mut _7: isize,mut _8: [isize; 2],mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: ([isize; 2],)) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _14: [i16; 1];
let _15: Adt46;
let _16: bool;
let _17: ();
let _18: ();
{
_7 = _10;
_3 = _4.0;
_6.0 = [_7,_10];
_1 = _11;
_1 = _9;
_6.0 = [_2,_9];
_4 = _13;
_16 = true;
_12 = _7 | _7;
RET = [_2,_12];
_8 = [_12,_9];
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(3_usize, 9_usize, Move(_9), 1_usize, Move(_1), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(3_usize, 8_usize, Move(_8), 3_usize, Move(_3), 12_usize, Move(_12), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [isize; 2],mut _2: isize,mut _3: ([isize; 2],),mut _4: [isize; 2],mut _5: isize,mut _6: isize,mut _7: isize) -> bool {
mir! {
type RET = bool;
let _8: i32;
let _9: ([isize; 2],);
let _10: Adt51;
let _11: Adt45;
let _12: i8;
let _13: [i16; 1];
let _14: bool;
let _15: (bool, i32, i32);
let _16: [u8; 5];
let _17: usize;
let _18: f64;
let _19: isize;
let _20: *mut [char; 2];
let _21: Adt45;
let _22: ();
let _23: ();
{
_3.0 = _1;
_2 = _7;
_3 = (_1,);
_1 = _3.0;
_3.0 = _1;
RET = false;
RET = !true;
_2 = _7 + _5;
RET = !false;
_3 = (_4,);
RET = !false;
_6 = _2 - _2;
_4 = _3.0;
_8 = (-5356470981809392600_i64) as i32;
_1 = _4;
RET = !false;
RET = true;
_9 = (_4,);
_3.0 = [_6,_5];
_3.0 = [_2,_5];
_8 = 922356524_i32 & 199157770_i32;
_9.0 = [_2,_2];
Call(_9.0 = fn5(_5, _1, _3, _6, _3, _2, _3.0, _6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.0 = [_7,_6];
_5 = -_6;
_7 = -_6;
_6 = -_7;
RET = true;
_5 = 943295800487390275_i64 as isize;
_2 = (-76409446791561854422388496935755727104_i128) as isize;
_9 = (_3.0,);
_4 = _3.0;
_8 = (-1556_i16) as i32;
_8 = 1579662710_i32;
_3 = (_4,);
_5 = !_7;
_3.0 = _4;
_5 = _7;
_7 = _5;
Goto(bb2)
}
bb2 = {
_1 = [_7,_6];
match _8 {
1579662710 => bb3,
_ => bb1
}
}
bb3 = {
_2 = _5;
_1 = [_7,_5];
_2 = _6 << _6;
_5 = 61583_u16 as isize;
_14 = !RET;
_15 = (RET, _8, _8);
match _15.1 {
1579662710 => bb5,
_ => bb4
}
}
bb4 = {
_1 = [_7,_6];
match _8 {
1579662710 => bb3,
_ => bb1
}
}
bb5 = {
_14 = !RET;
_3.0 = _1;
_15.0 = RET ^ _14;
_15 = (RET, _8, _8);
_1 = _9.0;
_12 = _7 as i8;
_3.0 = _9.0;
_15.0 = !_14;
_6 = _2;
_4 = [_7,_2];
_1 = [_5,_2];
_15.2 = _8 + _8;
RET = !_15.0;
_9 = (_4,);
_3.0 = [_7,_2];
_13 = [(-11246_i16)];
_15.1 = !_8;
Goto(bb6)
}
bb6 = {
RET = _15.0;
_15.0 = _14;
_9.0 = [_2,_6];
_13 = [26260_i16];
_8 = _15.2 & _15.2;
_4 = _1;
_6 = _7;
_18 = 231_u8 as f64;
_3.0 = [_7,_2];
_17 = RET as usize;
_8 = _15.1 << _2;
_3 = (_1,);
RET = _2 == _7;
_3 = (_4,);
_16 = [227_u8,184_u8,87_u8,172_u8,108_u8];
_3 = (_4,);
_8 = _15.2 * _15.1;
_15.0 = RET <= RET;
_4 = _1;
_17 = 4702540176592083086558732858308283007_i128 as usize;
Goto(bb7)
}
bb7 = {
Call(_22 = dump_var(4_usize, 13_usize, Move(_13), 17_usize, Move(_17), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_22 = dump_var(4_usize, 4_usize, Move(_4), 3_usize, Move(_3), 15_usize, Move(_15), 23_usize, _23), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: [isize; 2],mut _3: ([isize; 2],),mut _4: isize,mut _5: ([isize; 2],),mut _6: isize,mut _7: [isize; 2],mut _8: isize,mut _9: isize) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _10: &'static u32;
let _11: (usize,);
let _12: [usize; 3];
let _13: char;
let _14: isize;
let _15: ();
let _16: ();
{
RET = _3.0;
_4 = _6 >> _8;
_8 = _6;
_7 = RET;
_5 = (RET,);
_4 = 6967226266802062226_i64 as isize;
_2 = [_6,_1];
_9 = _1 << _8;
_5 = (RET,);
_3 = (_5.0,);
_4 = -_8;
_7 = [_6,_1];
_3 = _5;
RET = [_4,_1];
_1 = 551266205_i32 as isize;
_3 = _5;
_6 = !_9;
_6 = _8;
_5 = (_2,);
_11.0 = 12956319563123527758_usize;
_7 = [_6,_4];
_5.0 = RET;
_8 = _9 | _9;
_4 = _8;
_3 = (_7,);
_1 = !_8;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(5_usize, 3_usize, Move(_3), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(5_usize, 5_usize, Move(_5), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [isize; 2],mut _2: [isize; 2],mut _3: ([isize; 2],),mut _4: isize,mut _5: ([isize; 2],),mut _6: isize,mut _7: isize,mut _8: [u8; 5],mut _9: [u32; 5],mut _10: isize) -> u8 {
mir! {
type RET = u8;
let _11: Adt42;
let _12: [char; 2];
let _13: f64;
let _14: Adt57;
let _15: [i8; 1];
let _16: (char,);
let _17: bool;
let _18: u64;
let _19: f64;
let _20: char;
let _21: ([isize; 2],);
let _22: isize;
let _23: [u32; 5];
let _24: Adt44;
let _25: [usize; 3];
let _26: i8;
let _27: ();
let _28: ();
{
RET = !41_u8;
_8 = [RET,RET,RET,RET,RET];
_3 = (_2,);
_9 = [3716990912_u32,1358415740_u32,2061153278_u32,1404199831_u32,3647879546_u32];
_9 = [4072464487_u32,832244367_u32,799407873_u32,804549301_u32,2295443431_u32];
_9 = [3230395053_u32,265497514_u32,2073532991_u32,4274247563_u32,60478718_u32];
_2 = [_10,_4];
_8 = [RET,RET,RET,RET,RET];
_5.0 = [_10,_6];
_1 = [_7,_4];
_5 = _3;
RET = 165_u8;
_5 = _3;
match RET {
0 => bb1,
1 => bb2,
165 => bb4,
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
_1 = [_4,_4];
_13 = 2466982490107727514_i64 as f64;
RET = 102_u8 - 68_u8;
RET = _13 as u8;
RET = 197_u8 * 160_u8;
_1 = [_4,_6];
_5 = (_1,);
_12 = ['\u{10d0ac}','\u{aba10}'];
_3 = (_5.0,);
_5.0 = _2;
_15 = [(-4_i8)];
_9 = [3369851076_u32,731916483_u32,2410176630_u32,726409980_u32,331441621_u32];
RET = 7690176460480129912_u64 as u8;
_1 = [_6,_4];
RET = !122_u8;
_12 = ['\u{3c1aa}','\u{245b9}'];
_5.0 = _1;
_16.0 = '\u{1c28b}';
_8 = [RET,RET,RET,RET,RET];
_4 = 5995170716903502571_u64 as isize;
RET = 124_u8 | 106_u8;
RET = !108_u8;
_16.0 = '\u{c9fa2}';
_3 = (_2,);
RET = 526310957863391398_u64 as u8;
_7 = _6;
Call(_4 = fn7(_5, _3.0, _3.0, _5.0, _3, _5.0, _1, _2, _5.0, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14.fld1 = core::ptr::addr_of_mut!(_12);
_17 = true;
_16.0 = '\u{89e31}';
RET = (-5570001852459927658_i64) as u8;
_2 = _1;
_5.0 = [_10,_7];
_18 = !11381424609175526696_u64;
_5.0 = _1;
_5 = _3;
_10 = _7 | _7;
_3.0 = _5.0;
RET = 86_u8;
match RET {
0 => bb6,
86 => bb8,
_ => bb7
}
}
bb6 = {
_1 = [_4,_4];
_13 = 2466982490107727514_i64 as f64;
RET = 102_u8 - 68_u8;
RET = _13 as u8;
RET = 197_u8 * 160_u8;
_1 = [_4,_6];
_5 = (_1,);
_12 = ['\u{10d0ac}','\u{aba10}'];
_3 = (_5.0,);
_5.0 = _2;
_15 = [(-4_i8)];
_9 = [3369851076_u32,731916483_u32,2410176630_u32,726409980_u32,331441621_u32];
RET = 7690176460480129912_u64 as u8;
_1 = [_6,_4];
RET = !122_u8;
_12 = ['\u{3c1aa}','\u{245b9}'];
_5.0 = _1;
_16.0 = '\u{1c28b}';
_8 = [RET,RET,RET,RET,RET];
_4 = 5995170716903502571_u64 as isize;
RET = 124_u8 | 106_u8;
RET = !108_u8;
_16.0 = '\u{c9fa2}';
_3 = (_2,);
RET = 526310957863391398_u64 as u8;
_7 = _6;
Call(_4 = fn7(_5, _3.0, _3.0, _5.0, _3, _5.0, _1, _2, _5.0, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
Return()
}
bb8 = {
_14.fld0 = Adt41::Variant3 { fld0: _12 };
Goto(bb9)
}
bb9 = {
SetDiscriminant(_14.fld0, 3);
_3 = (_5.0,);
place!(Field::<[char; 2]>(Variant(_14.fld0, 3), 0)) = [_16.0,_16.0];
_20 = _16.0;
match RET {
0 => bb5,
86 => bb11,
_ => bb10
}
}
bb10 = {
_1 = [_4,_4];
_13 = 2466982490107727514_i64 as f64;
RET = 102_u8 - 68_u8;
RET = _13 as u8;
RET = 197_u8 * 160_u8;
_1 = [_4,_6];
_5 = (_1,);
_12 = ['\u{10d0ac}','\u{aba10}'];
_3 = (_5.0,);
_5.0 = _2;
_15 = [(-4_i8)];
_9 = [3369851076_u32,731916483_u32,2410176630_u32,726409980_u32,331441621_u32];
RET = 7690176460480129912_u64 as u8;
_1 = [_6,_4];
RET = !122_u8;
_12 = ['\u{3c1aa}','\u{245b9}'];
_5.0 = _1;
_16.0 = '\u{1c28b}';
_8 = [RET,RET,RET,RET,RET];
_4 = 5995170716903502571_u64 as isize;
RET = 124_u8 | 106_u8;
RET = !108_u8;
_16.0 = '\u{c9fa2}';
_3 = (_2,);
RET = 526310957863391398_u64 as u8;
_7 = _6;
Call(_4 = fn7(_5, _3.0, _3.0, _5.0, _3, _5.0, _1, _2, _5.0, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_1 = _3.0;
_20 = _16.0;
_11 = Adt42::Variant1 { fld0: _13,fld1: RET,fld2: _16,fld3: (-56_i8),fld4: (-380_i16) };
_8 = [Field::<u8>(Variant(_11, 1), 1),RET,RET,RET,Field::<u8>(Variant(_11, 1), 1)];
_13 = Field::<f64>(Variant(_11, 1), 0);
Goto(bb12)
}
bb12 = {
SetDiscriminant(_14.fld0, 1);
_19 = Field::<f64>(Variant(_11, 1), 0) * Field::<f64>(Variant(_11, 1), 0);
_14.fld1 = core::ptr::addr_of_mut!(_12);
place!(Field::<i16>(Variant(_11, 1), 4)) = -(-15679_i16);
place!(Field::<(bool, i32, i32)>(Variant(_14.fld0, 1), 7)).2 = 66184538_i32 + 404156530_i32;
_21.0 = [_10,_6];
_15 = [121_i8];
place!(Field::<*const char>(Variant(_14.fld0, 1), 5)) = core::ptr::addr_of!(_20);
RET = 6_usize as u8;
_18 = 620336153911149328_u64;
place!(Field::<*const isize>(Variant(_14.fld0, 1), 4)) = core::ptr::addr_of!(_4);
place!(Field::<(bool, i32, i32)>(Variant(_14.fld0, 1), 7)) = (_17, (-1713124118_i32), 495892140_i32);
place!(Field::<(bool, i32, i32)>(Variant(_14.fld0, 1), 7)).0 = _6 > _10;
_8 = [Field::<u8>(Variant(_11, 1), 1),RET,Field::<u8>(Variant(_11, 1), 1),RET,RET];
place!(Field::<char>(Variant(_14.fld0, 1), 1)) = _20;
RET = (-44_i8) as u8;
match Field::<u8>(Variant(_11, 1), 1) {
0 => bb9,
1 => bb13,
2 => bb14,
86 => bb16,
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
Return()
}
bb16 = {
_12 = [Field::<(char,)>(Variant(_11, 1), 2).0,_20];
place!(Field::<i16>(Variant(_11, 1), 4)) = 54224_u16 as i16;
_10 = _7 ^ _7;
_11 = Adt42::Variant1 { fld0: _13,fld1: RET,fld2: _16,fld3: 4_i8,fld4: (-1424_i16) };
place!(Field::<i16>(Variant(_11, 1), 4)) = (-8150_i16) >> _7;
_21.0 = _5.0;
place!(Field::<(bool, i32, i32)>(Variant(_14.fld0, 1), 7)).1 = -Field::<(bool, i32, i32)>(Variant(_14.fld0, 1), 7).2;
_5.0 = [_7,_10];
place!(Field::<u8>(Variant(_11, 1), 1)) = RET + RET;
_6 = !_10;
place!(Field::<[isize; 2]>(Variant(_14.fld0, 1), 6)) = _3.0;
Goto(bb17)
}
bb17 = {
Call(_27 = dump_var(6_usize, 6_usize, Move(_6), 16_usize, Move(_16), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(6_usize, 2_usize, Move(_2), 15_usize, Move(_15), 7_usize, Move(_7), 20_usize, Move(_20)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: ([isize; 2],),mut _2: [isize; 2],mut _3: [isize; 2],mut _4: [isize; 2],mut _5: ([isize; 2],),mut _6: [isize; 2],mut _7: [isize; 2],mut _8: [isize; 2],mut _9: [isize; 2],mut _10: ([isize; 2],)) -> isize {
mir! {
type RET = isize;
let _11: [i64; 8];
let _12: (bool, i32, i32);
let _13: char;
let _14: i32;
let _15: (i16, i8, *mut [char; 2]);
let _16: Adt43;
let _17: *mut u64;
let _18: [isize; 2];
let _19: usize;
let _20: bool;
let _21: (usize,);
let _22: isize;
let _23: bool;
let _24: [i64; 8];
let _25: isize;
let _26: [usize; 3];
let _27: [char; 2];
let _28: (char,);
let _29: ([isize; 2],);
let _30: [i16; 1];
let _31: [i16; 1];
let _32: isize;
let _33: *mut u64;
let _34: f32;
let _35: [i16; 1];
let _36: i32;
let _37: Adt54;
let _38: f64;
let _39: Adt41;
let _40: isize;
let _41: [i64; 8];
let _42: i8;
let _43: u32;
let _44: Adt56;
let _45: *mut [char; 2];
let _46: f64;
let _47: char;
let _48: ([isize; 2],);
let _49: isize;
let _50: f64;
let _51: bool;
let _52: *mut [i8; 1];
let _53: [isize; 2];
let _54: f32;
let _55: [i8; 1];
let _56: f64;
let _57: Adt56;
let _58: u64;
let _59: u8;
let _60: ();
let _61: ();
{
RET = (-6629374194916599092_i64) as isize;
_6 = [RET,RET];
_1.0 = [RET,RET];
Goto(bb1)
}
bb1 = {
_1.0 = [RET,RET];
_5.0 = [RET,RET];
_11 = [6185810848813916503_i64,(-7294939157135125430_i64),(-7379391325875088588_i64),2830416176067808133_i64,(-320580323969167613_i64),7018540601655422898_i64,(-6081238237767710995_i64),(-6678014172353032699_i64)];
_9 = _10.0;
_12.0 = false;
_3 = [RET,RET];
_2 = [RET,RET];
RET = !(-9223372036854775808_isize);
_1 = (_10.0,);
_12 = (true, (-11916878_i32), 1826944264_i32);
_12 = (true, (-194632153_i32), (-234676067_i32));
RET = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_4 = _10.0;
_13 = '\u{915e7}';
RET = (-9223372036854775808_isize) * (-29_isize);
_5.0 = _9;
_4 = [RET,RET];
_12.1 = _12.2 | _12.2;
_7 = _10.0;
match _12.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431533535389 => bb7,
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
_3 = _10.0;
_5.0 = [RET,RET];
RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
_12.2 = 0_usize as i32;
_1.0 = [RET,RET];
_12.0 = false ^ false;
_5.0 = [RET,RET];
_6 = [RET,RET];
_10.0 = [RET,RET];
_14 = _12.1;
_12 = (true, _14, _14);
_10 = _1;
_2 = [RET,RET];
_11 = [4282615155311096419_i64,4972879195932941398_i64,5789412275724365418_i64,(-6918171305742356955_i64),6817568373230853849_i64,6510729381403915143_i64,3869026814151234902_i64,(-8643241225971346231_i64)];
_5 = (_9,);
_8 = [RET,RET];
RET = !9223372036854775807_isize;
_19 = !7021211652734536522_usize;
Call(_14 = core::intrinsics::bswap(_12.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = (_3,);
_21.0 = _19;
_22 = RET;
_8 = _9;
_15.1 = 62_i8;
_1 = (_5.0,);
_10 = (_2,);
_11 = [1447127982678021436_i64,(-6853741274513083198_i64),6613872850507223859_i64,(-551972387309329038_i64),(-7156924878902443550_i64),(-8168900190667224133_i64),3536393845718011119_i64,6965517473348944175_i64];
_23 = !_12.0;
_18 = [RET,_22];
_11 = [6450113689037097835_i64,7243313530099348592_i64,(-5249188433638354317_i64),(-3700246669578731105_i64),(-2665312278838354051_i64),(-1920397161670306696_i64),4411779667622592095_i64,(-6676696630176663591_i64)];
_12 = (_23, _14, _14);
_22 = RET;
_15.0 = 10956504235152089337_u64 as i16;
_12.1 = _14 + _12.2;
_22 = RET & RET;
RET = _22 | _22;
_21 = (_19,);
_10 = (_8,);
_1.0 = [RET,RET];
_25 = !RET;
_20 = !_23;
_6 = [_25,_25];
Call(_8 = fn8(_10, _3, _10, _5.0, _9, _7, _7, _12.1, _9, _5, _12.0, _7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_21 = (_19,);
RET = _22;
_12 = (_20, _14, _14);
_8 = _5.0;
_19 = _21.0 - _21.0;
_23 = _12.0;
_25 = _22;
_12.0 = _12.2 > _12.1;
_12.1 = -_12.2;
_15.0 = 9154_i16;
_14 = 329130873849270972_i64 as i32;
_1 = (_3,);
_3 = _5.0;
_21.0 = _19 | _19;
_19 = !_21.0;
_21.0 = 11962285817031192098_u64 as usize;
_11 = [5543936033798625558_i64,4792092132436220177_i64,6911197416896650772_i64,(-3930104722260510908_i64),(-6334534852162171744_i64),(-7121600640193981871_i64),6563880560417958313_i64,8968442710534085339_i64];
_15.0 = (-18218_i16);
_22 = _25;
_21 = (_19,);
_3 = [RET,RET];
RET = !_22;
_10.0 = _5.0;
Goto(bb10)
}
bb10 = {
_4 = _10.0;
_8 = [_22,RET];
_12.2 = !_12.1;
_29 = _10;
_4 = [RET,_25];
_11 = [4197648679529124997_i64,(-2395734767854358758_i64),8862529717817502542_i64,(-5647796919378162561_i64),(-6310377831977603141_i64),(-8539809809647105612_i64),(-4410375861906545995_i64),(-4318050660392286030_i64)];
RET = 72810081639878814241986374369796567725_u128 as isize;
_28.0 = _13;
_24 = [6065669981010561075_i64,5248293786041674549_i64,(-2761509142751307773_i64),(-1747819551354622878_i64),(-6036558969984050702_i64),6270796510354078030_i64,(-1586512216637933613_i64),(-7924581073048692609_i64)];
Goto(bb11)
}
bb11 = {
_8 = [_22,_22];
_25 = -_22;
_18 = _29.0;
_7 = [_22,RET];
_15.1 = 232540502444539557614203913041504903920_u128 as i8;
_32 = -RET;
_21 = (_19,);
_30 = [_15.0];
_30 = [_15.0];
_23 = _21.0 != _19;
_4 = _29.0;
_20 = !_23;
_26 = [_19,_19,_19];
_9 = _1.0;
_10 = (_9,);
_15.2 = core::ptr::addr_of_mut!(_27);
_11 = _24;
_30 = [_15.0];
_34 = 32198_u16 as f32;
_28 = (_13,);
_2 = _5.0;
_29.0 = _5.0;
_8 = [_25,RET];
_23 = _20;
Goto(bb12)
}
bb12 = {
RET = 41888_u16 as isize;
_38 = _12.2 as f64;
_21.0 = !_19;
_11 = [(-2519797225609546922_i64),821424516610329535_i64,5026577952896990556_i64,(-7264512964097610761_i64),(-7822714507668533219_i64),(-1340710566814401982_i64),8534566700885719559_i64,5758830269063799489_i64];
_25 = _22;
_24 = [7628011753351778959_i64,(-6187826455067945035_i64),(-8841718359670645961_i64),(-8787379065354865564_i64),(-7670456702957583705_i64),6394181717774330022_i64,(-5274987929928658304_i64),8362469380655423852_i64];
_21 = (_19,);
_37 = Adt54::Variant0 { fld0: _30 };
_31 = [_15.0];
_29 = (_2,);
_23 = !_20;
_7 = _5.0;
_44.fld5.fld2 = [_15.0];
_44.fld2 = [3408594349_u32,3607739600_u32,3145441401_u32,3441647796_u32,3717604975_u32];
_15.2 = core::ptr::addr_of_mut!(_27);
_7 = [_25,RET];
_32 = _25;
_44.fld3 = Adt50::Variant1 { fld0: _23,fld1: 85_u8,fld2: _15.1 };
_5 = (_2,);
_12.0 = !_20;
place!(Field::<i8>(Variant(_44.fld3, 1), 2)) = _15.1;
_48.0 = [_32,_32];
_48.0 = _18;
_43 = 3032812435_u32 - 4271749052_u32;
Call(_13 = fn9(_2, _5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_35 = [_15.0];
_49 = _32 - _22;
_35 = [_15.0];
_44.fld0 = !(-73028119417166118346950394452757447558_i128);
_27 = [_28.0,_28.0];
_27 = [_28.0,_13];
_51 = _20 == _23;
_47 = _28.0;
_13 = _28.0;
_37 = Adt54::Variant0 { fld0: _31 };
_28 = (_47,);
_18 = [_32,_49];
_8 = [_49,_32];
place!(Field::<u8>(Variant(_44.fld3, 1), 1)) = _51 as u8;
_44.fld1 = (_15.0, _15.1, _15.2);
_39 = Adt41::Variant3 { fld0: _27 };
_12.0 = _23 | Field::<bool>(Variant(_44.fld3, 1), 0);
_12.1 = _12.2;
_25 = RET;
_44.fld4 = [_21.0,_21.0,_21.0];
_7 = [_49,_22];
match _15.0 {
0 => bb8,
1 => bb9,
2 => bb5,
340282366920938463463374607431768193238 => bb15,
_ => bb14
}
}
bb14 = {
_8 = [_22,_22];
_25 = -_22;
_18 = _29.0;
_7 = [_22,RET];
_15.1 = 232540502444539557614203913041504903920_u128 as i8;
_32 = -RET;
_21 = (_19,);
_30 = [_15.0];
_30 = [_15.0];
_23 = _21.0 != _19;
_4 = _29.0;
_20 = !_23;
_26 = [_19,_19,_19];
_9 = _1.0;
_10 = (_9,);
_15.2 = core::ptr::addr_of_mut!(_27);
_11 = _24;
_30 = [_15.0];
_34 = 32198_u16 as f32;
_28 = (_13,);
_2 = _5.0;
_29.0 = _5.0;
_8 = [_25,RET];
_23 = _20;
Goto(bb12)
}
bb15 = {
_46 = -_38;
_55 = [_15.1];
_44.fld1.2 = core::ptr::addr_of_mut!(place!(Field::<[char; 2]>(Variant(_39, 3), 0)));
_54 = _34 + _34;
_43 = 1250663403_u32 << _21.0;
_48.0 = [_49,_32];
SetDiscriminant(_44.fld3, 0);
_21.0 = !_19;
_40 = _25;
_10 = _29;
_39 = Adt41::Variant0 { fld0: _20,fld1: _44.fld0,fld2: _26,fld3: _44.fld1.1,fld4: _15.0,fld5: _15.2,fld6: (-7066720258296978461_i64) };
_44.fld1.1 = Field::<i16>(Variant(_39, 0), 4) as i8;
Goto(bb16)
}
bb16 = {
Call(_60 = dump_var(7_usize, 31_usize, Move(_31), 26_usize, Move(_26), 24_usize, Move(_24), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_60 = dump_var(7_usize, 10_usize, Move(_10), 12_usize, Move(_12), 25_usize, Move(_25), 40_usize, Move(_40)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_60 = dump_var(7_usize, 13_usize, Move(_13), 48_usize, Move(_48), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_60 = dump_var(7_usize, 27_usize, Move(_27), 5_usize, Move(_5), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_60 = dump_var(7_usize, 32_usize, Move(_32), 30_usize, Move(_30), 61_usize, _61, 61_usize, _61), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: ([isize; 2],),mut _2: [isize; 2],mut _3: ([isize; 2],),mut _4: [isize; 2],mut _5: [isize; 2],mut _6: [isize; 2],mut _7: [isize; 2],mut _8: i32,mut _9: [isize; 2],mut _10: ([isize; 2],),mut _11: bool,mut _12: [isize; 2]) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _13: i32;
let _14: isize;
let _15: Adt50;
let _16: Adt52;
let _17: u128;
let _18: isize;
let _19: (bool, i32, i32);
let _20: *mut i128;
let _21: [i16; 1];
let _22: [i64; 8];
let _23: isize;
let _24: *const isize;
let _25: Adt43;
let _26: [isize; 2];
let _27: isize;
let _28: (*mut i128,);
let _29: ();
let _30: ();
{
_9 = [(-87_isize),57_isize];
_9 = _1.0;
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_12 = _7;
_3.0 = [122_isize,9223372036854775807_isize];
_5 = _10.0;
RET = [9223372036854775807_isize,(-9223372036854775808_isize)];
_3.0 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_10 = (_2,);
_1 = (_5,);
_8 = '\u{bc8af}' as i32;
_11 = true;
_8 = 855280277_i32;
_3 = (_10.0,);
_10 = (_12,);
_7 = _1.0;
_10 = _1;
RET = _7;
RET = _12;
RET = [(-9223372036854775808_isize),9223372036854775807_isize];
_14 = (-9223372036854775808_isize) >> _8;
_19 = (_11, _8, _8);
match _19.1 {
0 => bb1,
1 => bb2,
855280277 => bb4,
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
_14 = 172_u8 as isize;
RET = _5;
_1 = _3;
_14 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
RET = [_14,_14];
_12 = [_14,_14];
_7 = [_14,_14];
_13 = _19.2 + _8;
_22 = [(-5875821374761245774_i64),(-4474303818938997652_i64),(-680489299470058025_i64),(-4367037846495309859_i64),(-2753857070186844512_i64),(-1955960640032215225_i64),843633576931147351_i64,112622105790566051_i64];
_23 = 725044999_u32 as isize;
_21 = [31048_i16];
match _8 {
855280277 => bb5,
_ => bb3
}
}
bb5 = {
_24 = core::ptr::addr_of!(_14);
_10 = (_4,);
_4 = [_14,_14];
match _19.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
855280277 => bb9,
_ => bb8
}
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
_17 = !178876784524459258709840791868925473842_u128;
_9 = [_14,_14];
match _19.2 {
0 => bb1,
1 => bb2,
2 => bb7,
855280277 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_7 = _10.0;
(*_24) = 201_u8 as isize;
match _8 {
0 => bb5,
1 => bb9,
2 => bb8,
3 => bb12,
4 => bb13,
855280277 => bb15,
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
_24 = core::ptr::addr_of!(_14);
_10 = (_4,);
_4 = [_14,_14];
match _19.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
855280277 => bb9,
_ => bb8
}
}
bb15 = {
_10 = (_1.0,);
_21 = [(-24080_i16)];
_3.0 = [_14,(*_24)];
_12 = [_23,(*_24)];
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(8_usize, 17_usize, Move(_17), 11_usize, Move(_11), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(8_usize, 3_usize, Move(_3), 9_usize, Move(_9), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(8_usize, 19_usize, Move(_19), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [isize; 2],mut _2: ([isize; 2],)) -> char {
mir! {
type RET = char;
let _3: [u8; 5];
let _4: i8;
let _5: isize;
let _6: ([isize; 2],);
let _7: char;
let _8: [u8; 5];
let _9: [u8; 5];
let _10: [i64; 8];
let _11: u16;
let _12: [usize; 3];
let _13: u64;
let _14: [usize; 3];
let _15: char;
let _16: i8;
let _17: [u32; 5];
let _18: [i64; 8];
let _19: f64;
let _20: i8;
let _21: [i8; 1];
let _22: char;
let _23: f32;
let _24: ();
let _25: ();
{
RET = '\u{8805}';
_2 = (_1,);
_2 = (_1,);
_2.0 = _1;
_1 = _2.0;
_2.0 = [(-91_isize),100_isize];
_1 = [(-53_isize),(-86_isize)];
_2.0 = _1;
RET = '\u{862f7}';
RET = '\u{dd969}';
RET = '\u{d72bd}';
RET = '\u{d0e65}';
RET = '\u{b2925}';
RET = '\u{15ed5}';
RET = '\u{bb03c}';
_2.0 = [9223372036854775807_isize,66_isize];
RET = '\u{d0650}';
_2 = (_1,);
_2 = (_1,);
RET = '\u{56d78}';
Goto(bb1)
}
bb1 = {
_6.0 = _2.0;
_6 = (_1,);
_2.0 = [9223372036854775807_isize,9223372036854775807_isize];
_7 = RET;
_2.0 = [9223372036854775807_isize,0_isize];
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize];
_5 = 9223372036854775807_isize + (-9223372036854775808_isize);
_6 = (_1,);
Call(_8 = fn10(_1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _7;
_2.0 = [_5,_5];
_4 = false as i8;
RET = _7;
_2.0 = _6.0;
_2 = _6;
_3 = _8;
_4 = !(-104_i8);
RET = _7;
_8 = [44_u8,74_u8,203_u8,248_u8,176_u8];
_2 = (_6.0,);
_7 = RET;
_8 = [87_u8,218_u8,175_u8,181_u8,108_u8];
_12 = [2500206770380035440_usize,14714088860537288738_usize,3_usize];
RET = _7;
_1 = [_5,_5];
_14 = [18118490274229069729_usize,6_usize,15656293437005985305_usize];
_10 = [582803420310704416_i64,5309763152227424742_i64,6654383776435481096_i64,5626249783822515083_i64,8598030290094072830_i64,(-6471614654346100594_i64),(-2246622536803838181_i64),(-9085724897738921377_i64)];
_15 = RET;
Call(_3 = fn17(_2.0, _6.0, _10, _2.0, _6.0, _2, _10, _1, _10, _8, _2, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = 5938452602332577871_u64;
_11 = _13 as u16;
_8 = [118_u8,152_u8,144_u8,29_u8,184_u8];
_7 = _15;
_12 = _14;
_2 = (_1,);
_13 = 5575688538000003136_u64;
_7 = _15;
match _13 {
5575688538000003136 => bb4,
_ => bb2
}
}
bb4 = {
_4 = 11_i8;
RET = _7;
_16 = _4;
_2 = (_6.0,);
_12 = _14;
RET = _7;
_9 = _8;
RET = _15;
_16 = _4 | _4;
_2 = (_6.0,);
_7 = RET;
_1 = _2.0;
_7 = _15;
_18 = [1889027900296617588_i64,(-7762022795772956466_i64),(-4411900660831350494_i64),2131991736231896167_i64,(-6771307228023427368_i64),3028144250817075527_i64,8508805204215054631_i64,6563904935529083729_i64];
_2.0 = [_5,_5];
_6.0 = _1;
_2.0 = _6.0;
_3 = [8_u8,45_u8,81_u8,169_u8,117_u8];
match _4 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
11 => bb13,
_ => bb12
}
}
bb5 = {
_13 = 5938452602332577871_u64;
_11 = _13 as u16;
_8 = [118_u8,152_u8,144_u8,29_u8,184_u8];
_7 = _15;
_12 = _14;
_2 = (_1,);
_13 = 5575688538000003136_u64;
_7 = _15;
match _13 {
5575688538000003136 => bb4,
_ => bb2
}
}
bb6 = {
RET = _7;
_2.0 = [_5,_5];
_4 = false as i8;
RET = _7;
_2.0 = _6.0;
_2 = _6;
_3 = _8;
_4 = !(-104_i8);
RET = _7;
_8 = [44_u8,74_u8,203_u8,248_u8,176_u8];
_2 = (_6.0,);
_7 = RET;
_8 = [87_u8,218_u8,175_u8,181_u8,108_u8];
_12 = [2500206770380035440_usize,14714088860537288738_usize,3_usize];
RET = _7;
_1 = [_5,_5];
_14 = [18118490274229069729_usize,6_usize,15656293437005985305_usize];
_10 = [582803420310704416_i64,5309763152227424742_i64,6654383776435481096_i64,5626249783822515083_i64,8598030290094072830_i64,(-6471614654346100594_i64),(-2246622536803838181_i64),(-9085724897738921377_i64)];
_15 = RET;
Call(_3 = fn17(_2.0, _6.0, _10, _2.0, _6.0, _2, _10, _1, _10, _8, _2, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_6.0 = _2.0;
_6 = (_1,);
_2.0 = [9223372036854775807_isize,9223372036854775807_isize];
_7 = RET;
_2.0 = [9223372036854775807_isize,0_isize];
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize];
_5 = 9223372036854775807_isize + (-9223372036854775808_isize);
_6 = (_1,);
Call(_8 = fn10(_1, _1), ReturnTo(bb2), UnwindUnreachable())
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
_4 = _7 as i8;
_21 = [_16];
_18 = [4258348380118507428_i64,4136129277999605375_i64,5084316620899944030_i64,(-5776415233609882304_i64),(-5305559184247202881_i64),(-6266505549036533231_i64),3671884873909953245_i64,1516985404377317790_i64];
_22 = RET;
_21 = [_16];
_6.0 = [_5,_5];
_14 = [10417444520866430327_usize,6867565664248454252_usize,2614285885345525416_usize];
_14 = _12;
_20 = _16 << _16;
Goto(bb14)
}
bb14 = {
_17 = [2605365697_u32,3116856975_u32,1355417821_u32,1047469358_u32,1632042042_u32];
_16 = _4;
RET = _22;
_15 = RET;
_21 = [_20];
_17 = [4004026132_u32,899831040_u32,2858286524_u32,3935311237_u32,1044312182_u32];
_23 = 85972554954963828489771769252164156117_u128 as f32;
_20 = _16;
_18 = [(-1749713410395220249_i64),(-5034223021175196406_i64),7835666619904992162_i64,(-6547570579944001487_i64),(-8999923098981628603_i64),4299578468084757028_i64,(-4274413475371989126_i64),5212919853247413143_i64];
_3 = [72_u8,224_u8,243_u8,20_u8,66_u8];
_19 = 23520_i16 as f64;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(9_usize, 22_usize, Move(_22), 16_usize, Move(_16), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(9_usize, 4_usize, Move(_4), 2_usize, Move(_2), 10_usize, Move(_10), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(9_usize, 5_usize, Move(_5), 18_usize, Move(_18), 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [isize; 2],mut _2: [isize; 2]) -> [u8; 5] {
mir! {
type RET = [u8; 5];
let _3: f64;
let _4: (bool, i32, i32);
let _5: *mut i128;
let _6: i64;
let _7: Adt50;
let _8: (usize,);
let _9: bool;
let _10: usize;
let _11: [i64; 8];
let _12: char;
let _13: [usize; 3];
let _14: [char; 2];
let _15: f32;
let _16: isize;
let _17: [u8; 5];
let _18: bool;
let _19: ();
let _20: ();
{
RET = [179_u8,199_u8,152_u8,16_u8,222_u8];
_2 = [63_isize,9223372036854775807_isize];
_1 = [(-80_isize),(-72_isize)];
_1 = [9223372036854775807_isize,27_isize];
_1 = [9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [55_u8,160_u8,139_u8,42_u8,210_u8];
RET = [224_u8,86_u8,170_u8,200_u8,142_u8];
_3 = 87_i8 as f64;
RET = [183_u8,187_u8,155_u8,198_u8,174_u8];
RET = [7_u8,20_u8,99_u8,24_u8,231_u8];
RET = [161_u8,76_u8,37_u8,11_u8,169_u8];
_4.1 = (-1631134598_i32) - (-162527869_i32);
RET = [185_u8,156_u8,143_u8,153_u8,131_u8];
_4 = (true, (-401497901_i32), (-1010446037_i32));
_4.1 = _4.0 as i32;
RET = [119_u8,9_u8,201_u8,19_u8,60_u8];
_3 = (-7_i8) as f64;
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [16_isize,9223372036854775807_isize];
_4 = (false, (-2077710795_i32), 1014191492_i32);
_4.1 = 80_u8 as i32;
_4.2 = (-16804_i16) as i32;
RET = [148_u8,85_u8,70_u8,40_u8,159_u8];
_4.2 = _4.1 * _4.1;
_2 = _1;
Call(_4.2 = fn11(_4.0, _4.0, _3, _1, _2, _4.0, _4.0, _3, RET, RET, _4.0, _1, RET, RET, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _2;
_4.2 = !_4.1;
RET = [172_u8,35_u8,101_u8,77_u8,217_u8];
_4.2 = 79_u8 as i32;
_6 = (-6511575249372425556_i64) * (-6062552533106791447_i64);
RET = [39_u8,253_u8,55_u8,247_u8,221_u8];
_4.1 = _4.2 << _6;
_2 = [9223372036854775807_isize,(-104_isize)];
_3 = 230_u8 as f64;
_2 = [9223372036854775807_isize,(-120_isize)];
_4.2 = !_4.1;
_3 = 9223372036854775807_isize as f64;
_3 = 273089481850622770279826541614934619269_u128 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = (true, 286633605_i32, (-53931921_i32));
_2 = [38_isize,57_isize];
_4.2 = _4.1 + _4.1;
_6 = -6765352941465847089_i64;
_3 = (-9223372036854775808_isize) as f64;
_1 = _2;
_4 = (false, 2116780590_i32, (-1787866908_i32));
_4.0 = !false;
_4 = (true, (-1540680275_i32), 1588261835_i32);
_4 = (true, 1488678218_i32, 2102933654_i32);
Goto(bb2)
}
bb2 = {
_2 = [9223372036854775807_isize,9223372036854775807_isize];
_4.2 = _4.1 + _4.1;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize];
RET = [138_u8,196_u8,250_u8,90_u8,214_u8];
_8.0 = 6223082121040151772_usize & 6619404144244136366_usize;
_4.1 = _4.2 ^ _4.2;
_6 = (-4426507722175903937_i64) & (-4020741790176805187_i64);
_8 = (5_usize,);
_4 = (false, (-63535048_i32), (-2117986411_i32));
_4.0 = true;
RET = [131_u8,48_u8,88_u8,142_u8,207_u8];
_4.1 = _4.2;
Goto(bb3)
}
bb3 = {
_4.0 = false ^ true;
_2 = [9223372036854775807_isize,22_isize];
_4 = (true, (-1863213349_i32), (-73016488_i32));
_8 = (373455926179569473_usize,);
match _4.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431695194968 => bb9,
_ => bb8
}
}
bb4 = {
_2 = [9223372036854775807_isize,9223372036854775807_isize];
_4.2 = _4.1 + _4.1;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize];
RET = [138_u8,196_u8,250_u8,90_u8,214_u8];
_8.0 = 6223082121040151772_usize & 6619404144244136366_usize;
_4.1 = _4.2 ^ _4.2;
_6 = (-4426507722175903937_i64) & (-4020741790176805187_i64);
_8 = (5_usize,);
_4 = (false, (-63535048_i32), (-2117986411_i32));
_4.0 = true;
RET = [131_u8,48_u8,88_u8,142_u8,207_u8];
_4.1 = _4.2;
Goto(bb3)
}
bb5 = {
_1 = _2;
_4.2 = !_4.1;
RET = [172_u8,35_u8,101_u8,77_u8,217_u8];
_4.2 = 79_u8 as i32;
_6 = (-6511575249372425556_i64) * (-6062552533106791447_i64);
RET = [39_u8,253_u8,55_u8,247_u8,221_u8];
_4.1 = _4.2 << _6;
_2 = [9223372036854775807_isize,(-104_isize)];
_3 = 230_u8 as f64;
_2 = [9223372036854775807_isize,(-120_isize)];
_4.2 = !_4.1;
_3 = 9223372036854775807_isize as f64;
_3 = 273089481850622770279826541614934619269_u128 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = (true, 286633605_i32, (-53931921_i32));
_2 = [38_isize,57_isize];
_4.2 = _4.1 + _4.1;
_6 = -6765352941465847089_i64;
_3 = (-9223372036854775808_isize) as f64;
_1 = _2;
_4 = (false, 2116780590_i32, (-1787866908_i32));
_4.0 = !false;
_4 = (true, (-1540680275_i32), 1588261835_i32);
_4 = (true, 1488678218_i32, 2102933654_i32);
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
_10 = (-128533392982213694707443660451032418734_i128) as usize;
_4 = (true, 1555506799_i32, (-1690748181_i32));
_8 = (_10,);
_3 = 250_u8 as f64;
_6 = 5357156336236302338_i64 + 3198568833669147652_i64;
_4.0 = true;
_1 = [9223372036854775807_isize,(-24_isize)];
_9 = _4.0;
_8.0 = _10 | _10;
_4.1 = _4.2 << _6;
RET = [169_u8,148_u8,152_u8,221_u8,230_u8];
_4 = (_9, (-14971797_i32), (-387835496_i32));
_3 = 2278947968679369535_u64 as f64;
_7 = Adt50::Variant1 { fld0: _4.0,fld1: 6_u8,fld2: (-77_i8) };
_9 = Field::<bool>(Variant(_7, 1), 0) <= Field::<bool>(Variant(_7, 1), 0);
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
RET = [33_u8,60_u8,82_u8,155_u8,238_u8];
place!(Field::<u8>(Variant(_7, 1), 1)) = 162_u8 & 212_u8;
_1 = _2;
place!(Field::<u8>(Variant(_7, 1), 1)) = 3383395488410245496_u64 as u8;
_4.1 = _4.2;
Call(_2 = core::intrinsics::transmute(_1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8.0 = _10 * _10;
_3 = _8.0 as f64;
_4.1 = _4.2;
_3 = 9875006354883080963_u64 as f64;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_4.2 = _4.1;
place!(Field::<bool>(Variant(_7, 1), 0)) = _4.0;
Goto(bb11)
}
bb11 = {
_3 = (-36_i8) as f64;
place!(Field::<bool>(Variant(_7, 1), 0)) = !_9;
_12 = '\u{10ab0e}';
place!(Field::<i8>(Variant(_7, 1), 2)) = 7282493319999719723_u64 as i8;
place!(Field::<i8>(Variant(_7, 1), 2)) = !54_i8;
match _4.2 {
0 => bb12,
1 => bb13,
340282366920938463463374607431380375960 => bb15,
_ => bb14
}
}
bb12 = {
_8.0 = _10 * _10;
_3 = _8.0 as f64;
_4.1 = _4.2;
_3 = 9875006354883080963_u64 as f64;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_4.2 = _4.1;
place!(Field::<bool>(Variant(_7, 1), 0)) = _4.0;
Goto(bb11)
}
bb13 = {
_1 = _2;
_4.2 = !_4.1;
RET = [172_u8,35_u8,101_u8,77_u8,217_u8];
_4.2 = 79_u8 as i32;
_6 = (-6511575249372425556_i64) * (-6062552533106791447_i64);
RET = [39_u8,253_u8,55_u8,247_u8,221_u8];
_4.1 = _4.2 << _6;
_2 = [9223372036854775807_isize,(-104_isize)];
_3 = 230_u8 as f64;
_2 = [9223372036854775807_isize,(-120_isize)];
_4.2 = !_4.1;
_3 = 9223372036854775807_isize as f64;
_3 = 273089481850622770279826541614934619269_u128 as f64;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = (true, 286633605_i32, (-53931921_i32));
_2 = [38_isize,57_isize];
_4.2 = _4.1 + _4.1;
_6 = -6765352941465847089_i64;
_3 = (-9223372036854775808_isize) as f64;
_1 = _2;
_4 = (false, 2116780590_i32, (-1787866908_i32));
_4.0 = !false;
_4 = (true, (-1540680275_i32), 1588261835_i32);
_4 = (true, 1488678218_i32, 2102933654_i32);
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_2 = _1;
_4.2 = _4.1;
_9 = _4.1 <= _4.2;
_3 = Field::<i8>(Variant(_7, 1), 2) as f64;
_1 = [9223372036854775807_isize,(-23_isize)];
_3 = _6 as f64;
RET = [Field::<u8>(Variant(_7, 1), 1),Field::<u8>(Variant(_7, 1), 1),Field::<u8>(Variant(_7, 1), 1),Field::<u8>(Variant(_7, 1), 1),Field::<u8>(Variant(_7, 1), 1)];
_3 = 272529307927882808938811008230259039510_u128 as f64;
_14 = [_12,_12];
_13 = [_8.0,_10,_8.0];
_8 = (_10,);
_10 = !_8.0;
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(10_usize, 14_usize, Move(_14), 6_usize, Move(_6), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(10_usize, 1_usize, Move(_1), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: bool,mut _2: bool,mut _3: f64,mut _4: [isize; 2],mut _5: [isize; 2],mut _6: bool,mut _7: bool,mut _8: f64,mut _9: [u8; 5],mut _10: [u8; 5],mut _11: bool,mut _12: [isize; 2],mut _13: [u8; 5],mut _14: [u8; 5],mut _15: [isize; 2]) -> i32 {
mir! {
type RET = i32;
let _16: [i64; 8];
let _17: [i16; 1];
let _18: char;
let _19: Adt48;
let _20: u16;
let _21: i64;
let _22: Adt45;
let _23: ([usize; 3], *mut i128, f64);
let _24: u8;
let _25: u16;
let _26: f64;
let _27: f32;
let _28: [isize; 2];
let _29: f32;
let _30: usize;
let _31: isize;
let _32: isize;
let _33: f32;
let _34: i8;
let _35: ([isize; 2],);
let _36: Adt53;
let _37: ();
let _38: ();
{
_4 = [17_isize,9223372036854775807_isize];
_12 = [9223372036854775807_isize,1_isize];
_11 = _2 ^ _6;
_10 = [187_u8,11_u8,65_u8,123_u8,172_u8];
_3 = _8;
_7 = !_11;
_15 = [(-110_isize),(-9223372036854775808_isize)];
_12 = [(-9223372036854775808_isize),101_isize];
_12 = _15;
RET = 1848194629_i32 + 1150786961_i32;
_17 = [(-26792_i16)];
_13 = [61_u8,52_u8,242_u8,29_u8,206_u8];
_18 = '\u{718c9}';
_9 = _13;
_2 = _11 <= _7;
_16 = [7291600597297562582_i64,(-3354646442889888300_i64),2146784016188309975_i64,7295484197899776302_i64,9059707905608089152_i64,(-1022139489393870958_i64),(-3034898501881700504_i64),2953236799256193645_i64];
_1 = _11 > _2;
RET = 1844195191_i32 + 1232394223_i32;
_13 = [13_u8,80_u8,254_u8,95_u8,186_u8];
_5 = [9223372036854775807_isize,69_isize];
_4 = [84_isize,(-9223372036854775808_isize)];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
Goto(bb1)
}
bb1 = {
_16 = [(-1697504849535842218_i64),7167490264567463870_i64,(-4935347791560845118_i64),(-5483244100859884926_i64),(-3329666693377504499_i64),(-4425852576780367280_i64),669741196952493248_i64,6048281814580289187_i64];
_16 = [4095687834971551398_i64,(-3327994108027991823_i64),5441969156644068286_i64,(-8872032097584520560_i64),(-5603425683691651330_i64),6113484102234030776_i64,(-4619977834937880031_i64),(-2901260579048239060_i64)];
_16 = [(-12118222401214651_i64),(-4220592127218935487_i64),3733032740830409602_i64,6941046115104559422_i64,(-8775648350711738300_i64),7389727359037485599_i64,(-6144016562264098781_i64),7193468769090808784_i64];
_11 = _6;
Call(_15 = fn12(_14, _9, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _2 | _2;
_7 = _6;
_15 = [(-39_isize),126_isize];
RET = (-1736392046_i32) >> 32744_i16;
_7 = _2;
_10 = [238_u8,226_u8,71_u8,171_u8,116_u8];
_11 = _1;
_4 = [9_isize,(-9223372036854775808_isize)];
_10 = [55_u8,164_u8,15_u8,20_u8,232_u8];
_10 = [189_u8,76_u8,129_u8,113_u8,111_u8];
_5 = [(-46_isize),(-9223372036854775808_isize)];
_13 = [178_u8,11_u8,138_u8,242_u8,19_u8];
_4 = _12;
_20 = 26598_u16 - 24527_u16;
_8 = 14271740378143614988_usize as f64;
_7 = _11;
_18 = '\u{10aecd}';
Goto(bb3)
}
bb3 = {
_15 = [9223372036854775807_isize,9223372036854775807_isize];
_12 = [(-9223372036854775808_isize),9223372036854775807_isize];
_21 = !(-7237552594414245510_i64);
_3 = -_8;
_14 = _9;
RET = 5120804_i32 - (-181145332_i32);
RET = -434900438_i32;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = 6281972605691241758_usize as f64;
_5 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1;
RET = 587126726_i32 << _20;
RET = 9223372036854775807_isize as i32;
_18 = '\u{1d5b5}';
RET = (-1922091247_i32) ^ 1020119784_i32;
_21 = RET as i64;
_3 = _8 + _8;
_10 = _9;
_21 = (-1080546036969334796_i64);
_15 = _5;
_6 = !_7;
_21 = (-2202346795347431087_i64) | 3285582280435554650_i64;
_7 = !_11;
_17 = [(-6757_i16)];
Goto(bb4)
}
bb4 = {
_10 = _14;
_17 = [(-23844_i16)];
_5 = [9223372036854775807_isize,9223372036854775807_isize];
RET = (-648287162_i32);
_10 = [82_u8,63_u8,208_u8,240_u8,225_u8];
match RET {
340282366920938463463374607431119924294 => bb5,
_ => bb1
}
}
bb5 = {
_20 = _18 as u16;
_24 = RET as u8;
_7 = _2;
_20 = 51921_u16;
_9 = _10;
_7 = _6 != _2;
RET = (-270320163_i32) | (-1122504241_i32);
_12 = _5;
_8 = _3 + _3;
_24 = !251_u8;
_5 = [(-9223372036854775808_isize),41_isize];
match _20 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
51921 => bb11,
_ => bb10
}
}
bb6 = {
_10 = _14;
_17 = [(-23844_i16)];
_5 = [9223372036854775807_isize,9223372036854775807_isize];
RET = (-648287162_i32);
_10 = [82_u8,63_u8,208_u8,240_u8,225_u8];
match RET {
340282366920938463463374607431119924294 => bb5,
_ => bb1
}
}
bb7 = {
_15 = [9223372036854775807_isize,9223372036854775807_isize];
_12 = [(-9223372036854775808_isize),9223372036854775807_isize];
_21 = !(-7237552594414245510_i64);
_3 = -_8;
_14 = _9;
RET = 5120804_i32 - (-181145332_i32);
RET = -434900438_i32;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = 6281972605691241758_usize as f64;
_5 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _1;
RET = 587126726_i32 << _20;
RET = 9223372036854775807_isize as i32;
_18 = '\u{1d5b5}';
RET = (-1922091247_i32) ^ 1020119784_i32;
_21 = RET as i64;
_3 = _8 + _8;
_10 = _9;
_21 = (-1080546036969334796_i64);
_15 = _5;
_6 = !_7;
_21 = (-2202346795347431087_i64) | 3285582280435554650_i64;
_7 = !_11;
_17 = [(-6757_i16)];
Goto(bb4)
}
bb8 = {
_7 = _2 | _2;
_7 = _6;
_15 = [(-39_isize),126_isize];
RET = (-1736392046_i32) >> 32744_i16;
_7 = _2;
_10 = [238_u8,226_u8,71_u8,171_u8,116_u8];
_11 = _1;
_4 = [9_isize,(-9223372036854775808_isize)];
_10 = [55_u8,164_u8,15_u8,20_u8,232_u8];
_10 = [189_u8,76_u8,129_u8,113_u8,111_u8];
_5 = [(-46_isize),(-9223372036854775808_isize)];
_13 = [178_u8,11_u8,138_u8,242_u8,19_u8];
_4 = _12;
_20 = 26598_u16 - 24527_u16;
_8 = 14271740378143614988_usize as f64;
_7 = _11;
_18 = '\u{10aecd}';
Goto(bb3)
}
bb9 = {
_16 = [(-1697504849535842218_i64),7167490264567463870_i64,(-4935347791560845118_i64),(-5483244100859884926_i64),(-3329666693377504499_i64),(-4425852576780367280_i64),669741196952493248_i64,6048281814580289187_i64];
_16 = [4095687834971551398_i64,(-3327994108027991823_i64),5441969156644068286_i64,(-8872032097584520560_i64),(-5603425683691651330_i64),6113484102234030776_i64,(-4619977834937880031_i64),(-2901260579048239060_i64)];
_16 = [(-12118222401214651_i64),(-4220592127218935487_i64),3733032740830409602_i64,6941046115104559422_i64,(-8775648350711738300_i64),7389727359037485599_i64,(-6144016562264098781_i64),7193468769090808784_i64];
_11 = _6;
Call(_15 = fn12(_14, _9, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_2 = _11;
_21 = 3817045568854488147_i64 ^ (-4389035371804732050_i64);
_2 = _7;
_23.2 = _8 - _8;
_14 = [_24,_24,_24,_24,_24];
_15 = [(-9223372036854775808_isize),(-8_isize)];
RET = 1590423945_i32;
_23.0 = [4479134144160104771_usize,3_usize,0_usize];
_23.2 = 58_isize as f64;
_20 = _23.2 as u16;
match RET {
0 => bb1,
1 => bb9,
2 => bb10,
3 => bb4,
4 => bb5,
5 => bb6,
1590423945 => bb12,
_ => bb8
}
}
bb12 = {
_9 = [_24,_24,_24,_24,_24];
_5 = [28_isize,(-112_isize)];
_7 = !_1;
RET = !(-1623427557_i32);
_10 = [_24,_24,_24,_24,_24];
_21 = _1 as i64;
_20 = 12390_u16 - 16504_u16;
_15 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_27 = (-27811_i16) as f32;
_28 = _4;
_1 = _11;
_5 = _15;
_11 = !_6;
_15 = _12;
_23.0 = [2695368277930802186_usize,7_usize,11203773178555223372_usize];
_1 = !_2;
_28 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_23.0 = [6_usize,13675367220427534572_usize,3735827316981329771_usize];
_5 = _15;
_16 = [_21,_21,_21,_21,_21,_21,_21,_21];
_10 = _14;
_25 = _20;
_12 = [9223372036854775807_isize,9223372036854775807_isize];
Goto(bb13)
}
bb13 = {
_13 = [_24,_24,_24,_24,_24];
_5 = [9223372036854775807_isize,9223372036854775807_isize];
_12 = _15;
_8 = -_3;
_30 = 5_usize * 6_usize;
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = 280046045_i32 + (-1585348244_i32);
_24 = !234_u8;
_31 = (-117_isize);
_9 = _13;
_32 = 29796_i16 as isize;
RET = _30 as i32;
_2 = _1;
_15 = [_32,_31];
_29 = -_27;
_8 = _3;
_13 = _14;
_18 = '\u{1c176}';
_32 = _31 + _31;
_34 = -21_i8;
_8 = -_23.2;
_8 = _3;
_29 = _27;
_4 = _28;
_7 = !_2;
_21 = -(-1990133355407540990_i64);
_6 = _1 > _11;
Goto(bb14)
}
bb14 = {
_24 = !202_u8;
_7 = _11;
_33 = -_29;
_25 = _20 >> _30;
_26 = -_3;
_10 = [_24,_24,_24,_24,_24];
_35 = (_15,);
_32 = _1 as isize;
_36.fld2 = [23768_i16];
_24 = 28_u8;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(11_usize, 10_usize, Move(_10), 1_usize, Move(_1), 12_usize, Move(_12), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(11_usize, 30_usize, Move(_30), 15_usize, Move(_15), 2_usize, Move(_2), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(11_usize, 11_usize, Move(_11), 34_usize, Move(_34), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(11_usize, 16_usize, Move(_16), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [u8; 5],mut _2: [u8; 5],mut _3: bool) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _4: f64;
let _5: (char,);
let _6: f32;
let _7: ([usize; 3], *mut i128, f64);
let _8: (usize,);
let _9: Adt49;
let _10: [u32; 5];
let _11: f32;
let _12: f64;
let _13: f32;
let _14: i128;
let _15: *const isize;
let _16: Adt42;
let _17: u128;
let _18: Adt46;
let _19: Adt49;
let _20: Adt41;
let _21: isize;
let _22: [isize; 2];
let _23: [i8; 1];
let _24: char;
let _25: u16;
let _26: (*mut u64, [usize; 3], [u8; 5], [i64; 8], i8);
let _27: *mut [char; 2];
let _28: [char; 2];
let _29: isize;
let _30: f64;
let _31: *mut i128;
let _32: f64;
let _33: f32;
let _34: ([isize; 2],);
let _35: bool;
let _36: *const isize;
let _37: i128;
let _38: u16;
let _39: [u8; 5];
let _40: [usize; 3];
let _41: f64;
let _42: isize;
let _43: u128;
let _44: [u32; 5];
let _45: [i8; 1];
let _46: [u8; 5];
let _47: i32;
let _48: i32;
let _49: (char,);
let _50: [char; 2];
let _51: Adt57;
let _52: isize;
let _53: ();
let _54: ();
{
RET = [9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = _2;
_2 = [70_u8,212_u8,186_u8,241_u8,187_u8];
RET = [(-9223372036854775808_isize),9223372036854775807_isize];
_1 = _2;
_2 = _1;
_4 = 178364364544184100095110045272466664297_u128 as f64;
_4 = (-844405934_i32) as f64;
_5 = ('\u{10b5f6}',);
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,(-47_isize)];
_1 = [25_u8,255_u8,181_u8,228_u8,64_u8];
_1 = _2;
RET = [(-9223372036854775808_isize),109_isize];
RET = [63_isize,(-34_isize)];
Call(_1 = fn13(_2, _3, _3, _3, _2, _3, _3, _3, _3, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-105_isize),9223372036854775807_isize];
_5 = ('\u{c37d3}',);
_5.0 = '\u{cb348}';
_3 = !false;
_5.0 = '\u{63556}';
_3 = true;
_1 = [56_u8,188_u8,29_u8,239_u8,37_u8];
_3 = false & false;
_4 = 5097_u16 as f64;
RET = [(-107_isize),9223372036854775807_isize];
_4 = (-102_i8) as f64;
_2 = [188_u8,73_u8,207_u8,238_u8,60_u8];
_5 = ('\u{d9520}',);
_5.0 = '\u{15756}';
_5 = ('\u{a205c}',);
_2 = [31_u8,168_u8,252_u8,84_u8,186_u8];
_4 = (-1359513988_i32) as f64;
_4 = (-6862252703205829515_i64) as f64;
_6 = (-946942326135240979_i64) as f32;
_5.0 = '\u{94327}';
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.0 = '\u{47754}';
_5 = ('\u{7b001}',);
RET = [(-82_isize),43_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize];
_7.0 = [8297935260339256936_usize,1835105979190808373_usize,14437387702641446534_usize];
RET = [9223372036854775807_isize,(-9223372036854775808_isize)];
Goto(bb2)
}
bb2 = {
_7.0 = [6_usize,2_usize,3_usize];
RET = [(-9223372036854775808_isize),115_isize];
_7.2 = -_4;
_1 = [92_u8,234_u8,144_u8,187_u8,198_u8];
_2 = [57_u8,104_u8,49_u8,243_u8,21_u8];
_3 = false;
_2 = [58_u8,116_u8,47_u8,182_u8,229_u8];
_2 = _1;
_8 = (5478006886065508362_usize,);
_4 = 14237_i16 as f64;
_3 = !true;
_8.0 = !7_usize;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7.2 = -_4;
_8.0 = _6 as usize;
_7.2 = _4;
_5.0 = '\u{c3271}';
_4 = _7.2;
_8.0 = 3724547539909828721_usize ^ 6748369886463269925_usize;
Goto(bb3)
}
bb3 = {
_2 = _1;
_2 = [171_u8,183_u8,140_u8,67_u8,81_u8];
_5 = ('\u{a4962}',);
_8.0 = 16899027765026333417_usize;
_3 = !true;
RET = [9223372036854775807_isize,58_isize];
_11 = _6;
_2 = [113_u8,137_u8,65_u8,71_u8,204_u8];
_10 = [4144844059_u32,148719617_u32,883233573_u32,1167969301_u32,3651062719_u32];
_10 = [849837493_u32,2719250737_u32,1277410400_u32,2571195841_u32,865456303_u32];
_8.0 = 2254182027_u32 as usize;
_7.0 = [_8.0,_8.0,_8.0];
_4 = 147_u8 as f64;
_8 = (4_usize,);
_13 = _11 * _6;
_1 = [80_u8,212_u8,187_u8,128_u8,230_u8];
_8.0 = !13841772944714055839_usize;
_12 = -_4;
_4 = _7.2;
_5 = ('\u{2afb4}',);
_1 = [14_u8,9_u8,193_u8,134_u8,221_u8];
_6 = -_11;
_7.1 = core::ptr::addr_of_mut!(_14);
_7.0 = [_8.0,_8.0,_8.0];
Goto(bb4)
}
bb4 = {
_11 = -_6;
_5.0 = '\u{10af8a}';
_16 = Adt42::Variant1 { fld0: _12,fld1: 50_u8,fld2: _5,fld3: 7_i8,fld4: 5381_i16 };
_6 = _11;
RET = [9223372036854775807_isize,62_isize];
_10 = [2168964653_u32,1596644538_u32,3023795997_u32,2482365190_u32,2925762039_u32];
place!(Field::<(char,)>(Variant(_16, 1), 2)).0 = _5.0;
_7.1 = core::ptr::addr_of_mut!(_14);
_16 = Adt42::Variant1 { fld0: _7.2,fld1: 159_u8,fld2: _5,fld3: (-112_i8),fld4: (-10230_i16) };
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [3842637071_u32,1758415382_u32,4027893390_u32,1925641898_u32,1679814935_u32];
place!(Field::<i16>(Variant(_16, 1), 4)) = 2586_i16 + 19173_i16;
place!(Field::<f64>(Variant(_16, 1), 0)) = 1029_u16 as f64;
place!(Field::<i16>(Variant(_16, 1), 4)) = 2880953688_u32 as i16;
_6 = _11;
place!(Field::<u8>(Variant(_16, 1), 1)) = !133_u8;
_17 = _6 as u128;
_22 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
Goto(bb5)
}
bb5 = {
_1 = _2;
Goto(bb6)
}
bb6 = {
_10 = [357073503_u32,1927293944_u32,2451011042_u32,1515046234_u32,2123504065_u32];
_4 = Field::<f64>(Variant(_16, 1), 0) * Field::<f64>(Variant(_16, 1), 0);
_14 = 109554921605497697364295163319419037605_i128 >> _17;
_4 = _7.2;
_2 = _1;
place!(Field::<u8>(Variant(_16, 1), 1)) = _8.0 as u8;
_26.2 = _2;
place!(Field::<i8>(Variant(_16, 1), 3)) = !86_i8;
place!(Field::<(char,)>(Variant(_16, 1), 2)) = _5;
_23 = [Field::<i8>(Variant(_16, 1), 3)];
place!(Field::<i16>(Variant(_16, 1), 4)) = (-2144448385160578234_i64) as i16;
_7.1 = core::ptr::addr_of_mut!(_14);
RET = _22;
SetDiscriminant(_16, 0);
_5 = ('\u{56ff2}',);
_25 = 3464_u16;
_3 = true ^ true;
_7.1 = core::ptr::addr_of_mut!(_14);
_13 = 7_i8 as f32;
_26.2 = [85_u8,51_u8,35_u8,210_u8,16_u8];
_16 = Adt42::Variant1 { fld0: _4,fld1: 180_u8,fld2: _5,fld3: (-1_i8),fld4: 10841_i16 };
match _25 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
3464 => bb7,
_ => bb5
}
}
bb7 = {
_26.1 = [_8.0,_8.0,_8.0];
place!(Field::<u8>(Variant(_16, 1), 1)) = !187_u8;
_26.4 = _8.0 as i8;
_6 = _8.0 as f32;
_21 = (-12080_i16) as isize;
place!(Field::<u8>(Variant(_16, 1), 1)) = _3 as u8;
_1 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
place!(Field::<i16>(Variant(_16, 1), 4)) = _17 as i16;
_1 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
_27 = core::ptr::addr_of_mut!(_28);
(*_27) = [Field::<(char,)>(Variant(_16, 1), 2).0,_5.0];
_2 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
match _25 {
3464 => bb9,
_ => bb8
}
}
bb8 = {
_7.0 = [6_usize,2_usize,3_usize];
RET = [(-9223372036854775808_isize),115_isize];
_7.2 = -_4;
_1 = [92_u8,234_u8,144_u8,187_u8,198_u8];
_2 = [57_u8,104_u8,49_u8,243_u8,21_u8];
_3 = false;
_2 = [58_u8,116_u8,47_u8,182_u8,229_u8];
_2 = _1;
_8 = (5478006886065508362_usize,);
_4 = 14237_i16 as f64;
_3 = !true;
_8.0 = !7_usize;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7.2 = -_4;
_8.0 = _6 as usize;
_7.2 = _4;
_5.0 = '\u{c3271}';
_4 = _7.2;
_8.0 = 3724547539909828721_usize ^ 6748369886463269925_usize;
Goto(bb3)
}
bb9 = {
_19 = Adt49::Variant2 { fld0: _3,fld1: Field::<u8>(Variant(_16, 1), 1),fld2: _7.0,fld3: _25,fld4: _27 };
_3 = Field::<bool>(Variant(_19, 2), 0) & Field::<bool>(Variant(_19, 2), 0);
SetDiscriminant(_19, 0);
place!(Field::<(char,)>(Variant(_16, 1), 2)).0 = _5.0;
_6 = -_13;
place!(Field::<i8>(Variant(_16, 1), 3)) = -_26.4;
_17 = 1897258106411542925_i64 as u128;
_5 = Field::<(char,)>(Variant(_16, 1), 2);
_21 = 9223372036854775807_isize;
_26.4 = Field::<i8>(Variant(_16, 1), 3);
place!(Field::<[char; 2]>(Variant(_19, 0), 4)) = [Field::<(char,)>(Variant(_16, 1), 2).0,Field::<(char,)>(Variant(_16, 1), 2).0];
(*_27) = Field::<[char; 2]>(Variant(_19, 0), 4);
_1 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
_5.0 = Field::<(char,)>(Variant(_16, 1), 2).0;
_22 = [_21,_21];
_8.0 = _3 as usize;
_28 = [_5.0,_5.0];
place!(Field::<[u8; 5]>(Variant(_19, 0), 3)) = _26.2;
_26.3 = [7493074787067301071_i64,(-1195496517931433251_i64),496566827735131880_i64,(-3178254297748261674_i64),(-4006098236520930030_i64),(-165095179164745454_i64),4179920158682098311_i64,2303342130864363907_i64];
_26.3 = [(-5413190598843129027_i64),205542805248493264_i64,(-4231886188864871572_i64),(-5876106671156120702_i64),8046691302946481078_i64,4474568800640866200_i64,1881709186333030519_i64,(-6787017230293147774_i64)];
_35 = !_3;
_10 = [2885832393_u32,3620843400_u32,3642153150_u32,3323065026_u32,2271048850_u32];
_39 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
Call(_30 = core::intrinsics::transmute(_8.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_12 = Field::<i8>(Variant(_16, 1), 3) as f64;
_25 = 42350_u16;
_26.1 = [_8.0,_8.0,_8.0];
_31 = core::ptr::addr_of_mut!(_14);
_18 = Adt46::Variant2 { fld0: _26.3 };
SetDiscriminant(_18, 2);
_28 = [Field::<(char,)>(Variant(_16, 1), 2).0,Field::<(char,)>(Variant(_16, 1), 2).0];
_26.2 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
_24 = _5.0;
place!(Field::<i8>(Variant(_16, 1), 3)) = _26.4 << _21;
_3 = _35;
_38 = Field::<i16>(Variant(_16, 1), 4) as u16;
SetDiscriminant(_16, 0);
place!(Field::<[i64; 8]>(Variant(_18, 2), 0)) = [(-5287770898844217723_i64),(-500515829324954508_i64),3040908540454923946_i64,8918158418126184307_i64,(-1781120299765344669_i64),6713038450453066568_i64,(-3186021068875664002_i64),(-5676775619181493889_i64)];
_32 = -_30;
place!(Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5)).1 = _26.4;
_5 = (_24,);
place!(Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5)) = (27084_i16, _26.4, _27);
RET = [_21,_21];
_25 = _24 as u16;
_38 = !_25;
SetDiscriminant(_18, 0);
Goto(bb11)
}
bb11 = {
_26.4 = Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5).1 & Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5).1;
_34 = (_22,);
(*_27) = Field::<[char; 2]>(Variant(_19, 0), 4);
place!(Field::<*mut [char; 2]>(Variant(_16, 0), 1)) = core::ptr::addr_of_mut!((*_27));
_22 = [_21,_21];
_3 = !_35;
place!(Field::<(*mut u64, [usize; 3], [u8; 5], [i64; 8], i8)>(Variant(_18, 0), 3)).4 = _32 as i8;
place!(Field::<(bool, i32, i32)>(Variant(_18, 0), 1)) = (_35, (-1780137813_i32), (-2131227271_i32));
_14 = 133909024912156018049498964649561013589_i128;
_24 = _5.0;
place!(Field::<bool>(Variant(_18, 0), 0)) = !_35;
_26.2 = Field::<[u8; 5]>(Variant(_19, 0), 3);
_14 = _3 as i128;
_39 = [81_u8,11_u8,100_u8,187_u8,22_u8];
_42 = !_21;
_1 = _39;
_7.1 = core::ptr::addr_of_mut!(_37);
_29 = -_42;
place!(Field::<(*mut u64, [usize; 3], [u8; 5], [i64; 8], i8)>(Variant(_18, 0), 3)).2 = [98_u8,58_u8,42_u8,134_u8,53_u8];
place!(Field::<f32>(Variant(_18, 0), 2)) = _6;
_33 = _6;
Goto(bb12)
}
bb12 = {
_29 = !_21;
_23 = [_26.4];
_8.0 = 4_usize << Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5).0;
_20 = Adt41::Variant3 { fld0: (*_27) };
_38 = _25 ^ _25;
match Field::<(bool, i32, i32)>(Variant(_18, 0), 1).1 {
0 => bb5,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
340282366920938463463374607429988073643 => bb20,
_ => bb19
}
}
bb13 = {
_26.4 = Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5).1 & Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5).1;
_34 = (_22,);
(*_27) = Field::<[char; 2]>(Variant(_19, 0), 4);
place!(Field::<*mut [char; 2]>(Variant(_16, 0), 1)) = core::ptr::addr_of_mut!((*_27));
_22 = [_21,_21];
_3 = !_35;
place!(Field::<(*mut u64, [usize; 3], [u8; 5], [i64; 8], i8)>(Variant(_18, 0), 3)).4 = _32 as i8;
place!(Field::<(bool, i32, i32)>(Variant(_18, 0), 1)) = (_35, (-1780137813_i32), (-2131227271_i32));
_14 = 133909024912156018049498964649561013589_i128;
_24 = _5.0;
place!(Field::<bool>(Variant(_18, 0), 0)) = !_35;
_26.2 = Field::<[u8; 5]>(Variant(_19, 0), 3);
_14 = _3 as i128;
_39 = [81_u8,11_u8,100_u8,187_u8,22_u8];
_42 = !_21;
_1 = _39;
_7.1 = core::ptr::addr_of_mut!(_37);
_29 = -_42;
place!(Field::<(*mut u64, [usize; 3], [u8; 5], [i64; 8], i8)>(Variant(_18, 0), 3)).2 = [98_u8,58_u8,42_u8,134_u8,53_u8];
place!(Field::<f32>(Variant(_18, 0), 2)) = _6;
_33 = _6;
Goto(bb12)
}
bb14 = {
_7.0 = [6_usize,2_usize,3_usize];
RET = [(-9223372036854775808_isize),115_isize];
_7.2 = -_4;
_1 = [92_u8,234_u8,144_u8,187_u8,198_u8];
_2 = [57_u8,104_u8,49_u8,243_u8,21_u8];
_3 = false;
_2 = [58_u8,116_u8,47_u8,182_u8,229_u8];
_2 = _1;
_8 = (5478006886065508362_usize,);
_4 = 14237_i16 as f64;
_3 = !true;
_8.0 = !7_usize;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7.2 = -_4;
_8.0 = _6 as usize;
_7.2 = _4;
_5.0 = '\u{c3271}';
_4 = _7.2;
_8.0 = 3724547539909828721_usize ^ 6748369886463269925_usize;
Goto(bb3)
}
bb15 = {
_2 = _1;
_2 = [171_u8,183_u8,140_u8,67_u8,81_u8];
_5 = ('\u{a4962}',);
_8.0 = 16899027765026333417_usize;
_3 = !true;
RET = [9223372036854775807_isize,58_isize];
_11 = _6;
_2 = [113_u8,137_u8,65_u8,71_u8,204_u8];
_10 = [4144844059_u32,148719617_u32,883233573_u32,1167969301_u32,3651062719_u32];
_10 = [849837493_u32,2719250737_u32,1277410400_u32,2571195841_u32,865456303_u32];
_8.0 = 2254182027_u32 as usize;
_7.0 = [_8.0,_8.0,_8.0];
_4 = 147_u8 as f64;
_8 = (4_usize,);
_13 = _11 * _6;
_1 = [80_u8,212_u8,187_u8,128_u8,230_u8];
_8.0 = !13841772944714055839_usize;
_12 = -_4;
_4 = _7.2;
_5 = ('\u{2afb4}',);
_1 = [14_u8,9_u8,193_u8,134_u8,221_u8];
_6 = -_11;
_7.1 = core::ptr::addr_of_mut!(_14);
_7.0 = [_8.0,_8.0,_8.0];
Goto(bb4)
}
bb16 = {
_7.0 = [6_usize,2_usize,3_usize];
RET = [(-9223372036854775808_isize),115_isize];
_7.2 = -_4;
_1 = [92_u8,234_u8,144_u8,187_u8,198_u8];
_2 = [57_u8,104_u8,49_u8,243_u8,21_u8];
_3 = false;
_2 = [58_u8,116_u8,47_u8,182_u8,229_u8];
_2 = _1;
_8 = (5478006886065508362_usize,);
_4 = 14237_i16 as f64;
_3 = !true;
_8.0 = !7_usize;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7.2 = -_4;
_8.0 = _6 as usize;
_7.2 = _4;
_5.0 = '\u{c3271}';
_4 = _7.2;
_8.0 = 3724547539909828721_usize ^ 6748369886463269925_usize;
Goto(bb3)
}
bb17 = {
_26.1 = [_8.0,_8.0,_8.0];
place!(Field::<u8>(Variant(_16, 1), 1)) = !187_u8;
_26.4 = _8.0 as i8;
_6 = _8.0 as f32;
_21 = (-12080_i16) as isize;
place!(Field::<u8>(Variant(_16, 1), 1)) = _3 as u8;
_1 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
place!(Field::<i16>(Variant(_16, 1), 4)) = _17 as i16;
_1 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
_27 = core::ptr::addr_of_mut!(_28);
(*_27) = [Field::<(char,)>(Variant(_16, 1), 2).0,_5.0];
_2 = [Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1),Field::<u8>(Variant(_16, 1), 1)];
match _25 {
3464 => bb9,
_ => bb8
}
}
bb18 = {
_11 = -_6;
_5.0 = '\u{10af8a}';
_16 = Adt42::Variant1 { fld0: _12,fld1: 50_u8,fld2: _5,fld3: 7_i8,fld4: 5381_i16 };
_6 = _11;
RET = [9223372036854775807_isize,62_isize];
_10 = [2168964653_u32,1596644538_u32,3023795997_u32,2482365190_u32,2925762039_u32];
place!(Field::<(char,)>(Variant(_16, 1), 2)).0 = _5.0;
_7.1 = core::ptr::addr_of_mut!(_14);
_16 = Adt42::Variant1 { fld0: _7.2,fld1: 159_u8,fld2: _5,fld3: (-112_i8),fld4: (-10230_i16) };
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_10 = [3842637071_u32,1758415382_u32,4027893390_u32,1925641898_u32,1679814935_u32];
place!(Field::<i16>(Variant(_16, 1), 4)) = 2586_i16 + 19173_i16;
place!(Field::<f64>(Variant(_16, 1), 0)) = 1029_u16 as f64;
place!(Field::<i16>(Variant(_16, 1), 4)) = 2880953688_u32 as i16;
_6 = _11;
place!(Field::<u8>(Variant(_16, 1), 1)) = !133_u8;
_17 = _6 as u128;
_22 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
Goto(bb5)
}
bb19 = {
_1 = _2;
Goto(bb6)
}
bb20 = {
place!(Field::<[usize; 3]>(Variant(_19, 0), 2)) = _26.1;
place!(Field::<[char; 2]>(Variant(_20, 3), 0)) = [_5.0,_5.0];
place!(Field::<[char; 2]>(Variant(_19, 0), 4)) = (*_27);
_41 = Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5).0 as f64;
place!(Field::<[u8; 5]>(Variant(_19, 0), 3)) = _26.2;
place!(Field::<f32>(Variant(_18, 0), 2)) = -_6;
place!(Field::<i64>(Variant(_19, 0), 1)) = !(-6767501384239854544_i64);
_11 = Field::<i64>(Variant(_19, 0), 1) as f32;
_38 = _24 as u16;
_35 = !Field::<(bool, i32, i32)>(Variant(_18, 0), 1).0;
_33 = _25 as f32;
_3 = _35;
_38 = _25;
_30 = _32;
place!(Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5)).1 = _26.4 * Field::<(*mut u64, [usize; 3], [u8; 5], [i64; 8], i8)>(Variant(_18, 0), 3).4;
_29 = _35 as isize;
_39 = Field::<[u8; 5]>(Variant(_19, 0), 3);
place!(Field::<(i16, i8, *mut [char; 2])>(Variant(_16, 0), 5)) = (16202_i16, _26.4, _27);
_18 = Adt46::Variant2 { fld0: _26.3 };
_3 = _25 >= _38;
RET = _34.0;
place!(Field::<i64>(Variant(_19, 0), 1)) = 54706509602908762_i64;
_11 = _13 - _6;
_36 = core::ptr::addr_of!(_52);
Goto(bb21)
}
bb21 = {
Call(_53 = dump_var(12_usize, 39_usize, Move(_39), 14_usize, Move(_14), 38_usize, Move(_38), 3_usize, Move(_3)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_53 = dump_var(12_usize, 42_usize, Move(_42), 21_usize, Move(_21), 24_usize, Move(_24), 23_usize, Move(_23)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_53 = dump_var(12_usize, 1_usize, Move(_1), 28_usize, Move(_28), 54_usize, _54, 54_usize, _54), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [u8; 5],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: [u8; 5],mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool) -> [u8; 5] {
mir! {
type RET = [u8; 5];
let _12: bool;
let _13: [usize; 3];
let _14: Adt43;
let _15: [u8; 5];
let _16: f32;
let _17: f32;
let _18: [i16; 1];
let _19: Adt47;
let _20: usize;
let _21: Adt49;
let _22: [isize; 2];
let _23: [isize; 2];
let _24: u16;
let _25: &'static u32;
let _26: [u8; 5];
let _27: f32;
let _28: u128;
let _29: f64;
let _30: [usize; 3];
let _31: i8;
let _32: *mut u64;
let _33: [u8; 5];
let _34: [usize; 3];
let _35: isize;
let _36: [isize; 2];
let _37: f64;
let _38: [u8; 5];
let _39: (char,);
let _40: i64;
let _41: [i8; 1];
let _42: f64;
let _43: [i16; 1];
let _44: (bool, i32, i32);
let _45: f64;
let _46: i128;
let _47: (*mut i128,);
let _48: (bool, i32, i32);
let _49: Adt52;
let _50: f64;
let _51: isize;
let _52: isize;
let _53: isize;
let _54: [usize; 3];
let _55: u8;
let _56: ();
let _57: ();
{
_5 = [14_u8,129_u8,171_u8,174_u8,141_u8];
RET = [115_u8,123_u8,40_u8,214_u8,187_u8];
_9 = _7 >= _6;
_8 = !_7;
_7 = _6;
_3 = _7;
_5 = [103_u8,130_u8,232_u8,127_u8,156_u8];
RET = _1;
RET = _1;
_7 = _2;
_9 = !_11;
_11 = !_9;
RET = [234_u8,205_u8,77_u8,82_u8,2_u8];
_15 = [64_u8,113_u8,115_u8,113_u8,191_u8];
_11 = _10;
_10 = !_6;
_6 = !_3;
_13 = [6_usize,14918282580965317989_usize,3315193687121739094_usize];
_10 = !_3;
_2 = !_7;
_8 = _11;
RET = [67_u8,160_u8,253_u8,5_u8,151_u8];
RET = [225_u8,60_u8,202_u8,127_u8,20_u8];
_16 = (-16_isize) as f32;
_10 = !_8;
Goto(bb1)
}
bb1 = {
_3 = _8 != _11;
_15 = RET;
RET = [46_u8,162_u8,80_u8,180_u8,164_u8];
_5 = [207_u8,238_u8,234_u8,21_u8,212_u8];
RET = [180_u8,83_u8,77_u8,227_u8,107_u8];
RET = [68_u8,206_u8,158_u8,219_u8,32_u8];
_17 = 4985249872221808224_i64 as f32;
_4 = _8 > _3;
_6 = !_4;
_18 = [(-21276_i16)];
_3 = !_4;
Goto(bb2)
}
bb2 = {
_16 = _17 * _17;
_9 = _3 > _3;
_10 = !_9;
_9 = !_2;
_6 = _10 ^ _3;
_12 = _8;
_11 = _10 >= _4;
_8 = !_10;
_9 = _11 > _8;
_7 = _9 != _11;
_10 = _6;
_11 = _8 > _8;
_11 = _8 <= _10;
_17 = _16;
_16 = 50901_u16 as f32;
_8 = !_11;
_6 = !_3;
_7 = _8;
_11 = _2 & _4;
_15 = [72_u8,1_u8,241_u8,214_u8,141_u8];
Goto(bb3)
}
bb3 = {
_22 = [(-9223372036854775808_isize),9223372036854775807_isize];
RET = _15;
_9 = !_10;
_11 = !_9;
_22 = [(-9223372036854775808_isize),(-29_isize)];
_1 = [143_u8,218_u8,146_u8,96_u8,251_u8];
RET = _15;
_23 = _22;
_16 = 3_i8 as f32;
_3 = _7;
RET = [208_u8,61_u8,200_u8,249_u8,81_u8];
_11 = _6;
_10 = _6;
_12 = _7 > _9;
_7 = _8;
_22 = _23;
Goto(bb4)
}
bb4 = {
RET = [184_u8,255_u8,7_u8,83_u8,243_u8];
_18 = [13381_i16];
_13 = [1318708372073750691_usize,3_usize,3580736305569001813_usize];
_18 = [(-32037_i16)];
_20 = 98817599394620847563793578239653174931_u128 as usize;
_13 = [_20,_20,_20];
_16 = -_17;
_20 = (-2089636741_i32) as usize;
_9 = _2;
_23 = _22;
_22 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_28 = 5565021133261653136745780954695623987_u128;
_11 = !_12;
_27 = _28 as f32;
_24 = 12458_u16;
_8 = _12 >= _11;
_12 = _11;
_11 = _8;
_9 = _6 < _6;
_28 = 3996492187_u32 as u128;
_29 = 2571741740537769528_u64 as f64;
_18 = [(-25153_i16)];
Goto(bb5)
}
bb5 = {
_11 = _12;
_18 = [16651_i16];
_28 = 16262833015688400315762559680400713300_u128;
_17 = _16;
RET = [205_u8,5_u8,219_u8,251_u8,223_u8];
_2 = !_10;
_11 = _8 | _7;
_7 = _11;
_1 = [63_u8,223_u8,85_u8,234_u8,20_u8];
_15 = _1;
_26 = [14_u8,209_u8,55_u8,108_u8,43_u8];
_11 = !_9;
_13 = [_20,_20,_20];
RET = [243_u8,24_u8,138_u8,161_u8,109_u8];
_1 = RET;
_2 = _8;
_30 = [_20,_20,_20];
_27 = _16 * _17;
_5 = [158_u8,102_u8,108_u8,208_u8,156_u8];
_17 = -_16;
_20 = _7 as usize;
_17 = -_27;
_7 = _3;
_17 = -_27;
Goto(bb6)
}
bb6 = {
_29 = _17 as f64;
_15 = [146_u8,73_u8,106_u8,58_u8,143_u8];
_33 = [240_u8,159_u8,211_u8,245_u8,216_u8];
_18 = [4072_i16];
Goto(bb7)
}
bb7 = {
_35 = 75048649693263917308248237695098870891_i128 as isize;
_29 = _17 as f64;
_36 = [_35,_35];
_31 = 111_i8 - (-69_i8);
_6 = !_2;
_16 = -_17;
_1 = [212_u8,248_u8,46_u8,180_u8,231_u8];
_29 = _24 as f64;
_15 = _1;
_7 = _9;
_36 = [_35,_35];
_16 = _27;
Goto(bb8)
}
bb8 = {
_26 = _33;
_9 = !_2;
_37 = -_29;
_39 = ('\u{11407}',);
_7 = _8;
_1 = _5;
_8 = _11 | _4;
_1 = [192_u8,2_u8,251_u8,94_u8,73_u8];
_34 = [_20,_20,_20];
Call(_23 = fn14(_20, _3, _9, _20, _5, _20, _8, _20, _5, _6, _9, _7, _7, _34), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_24 = !17715_u16;
match _28 {
0 => bb7,
1 => bb2,
2 => bb3,
16262833015688400315762559680400713300 => bb10,
_ => bb8
}
}
bb10 = {
_22 = _23;
_40 = (-5377144956498486492_i64) << _20;
_2 = _12;
_35 = _24 as isize;
_33 = [43_u8,14_u8,254_u8,119_u8,52_u8];
_12 = !_4;
_44 = (_7, 1212875211_i32, 1791212995_i32);
_29 = _37 - _37;
_43 = [13915_i16];
_11 = _7 ^ _3;
_43 = [2825_i16];
RET = [231_u8,208_u8,75_u8,128_u8,65_u8];
_39.0 = '\u{3a7d4}';
_18 = [4635_i16];
_11 = _2;
Call(_38 = core::intrinsics::transmute(_1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_41 = [_31];
_19.fld0 = core::ptr::addr_of_mut!(_46);
_48 = _44;
_16 = _17;
_16 = -_17;
_20 = _29 as usize;
_15 = _5;
_39 = ('\u{f17bb}',);
_17 = 180_u8 as f32;
_18 = [9122_i16];
_23 = [_35,_35];
_35 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_33 = [2_u8,4_u8,125_u8,20_u8,83_u8];
_42 = -_37;
_44.0 = _11;
_34 = [_20,_20,_20];
match _48.2 {
0 => bb8,
1 => bb10,
1791212995 => bb13,
_ => bb12
}
}
bb12 = {
_24 = !17715_u16;
match _28 {
0 => bb7,
1 => bb2,
2 => bb3,
16262833015688400315762559680400713300 => bb10,
_ => bb8
}
}
bb13 = {
RET = [117_u8,180_u8,231_u8,240_u8,75_u8];
_28 = 290500982744010211177718331675824506941_u128;
_29 = _24 as f64;
_48.0 = !_12;
_48.2 = _48.1 << _44.2;
_38 = [42_u8,240_u8,252_u8,58_u8,183_u8];
_28 = 272538487518605571478905252724657145347_u128;
_41 = [_31];
_33 = [79_u8,96_u8,220_u8,208_u8,228_u8];
_12 = _11;
_34 = [_20,_20,_20];
match _48.1 {
0 => bb14,
1212875211 => bb16,
_ => bb15
}
}
bb14 = {
_26 = _33;
_9 = !_2;
_37 = -_29;
_39 = ('\u{11407}',);
_7 = _8;
_1 = _5;
_8 = _11 | _4;
_1 = [192_u8,2_u8,251_u8,94_u8,73_u8];
_34 = [_20,_20,_20];
Call(_23 = fn14(_20, _3, _9, _20, _5, _20, _8, _20, _5, _6, _9, _7, _7, _34), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_35 = 75048649693263917308248237695098870891_i128 as isize;
_29 = _17 as f64;
_36 = [_35,_35];
_31 = 111_i8 - (-69_i8);
_6 = !_2;
_16 = -_17;
_1 = [212_u8,248_u8,46_u8,180_u8,231_u8];
_29 = _24 as f64;
_15 = _1;
_7 = _9;
_36 = [_35,_35];
_16 = _27;
Goto(bb8)
}
bb16 = {
_10 = !_3;
_48 = _44;
_1 = [114_u8,42_u8,156_u8,79_u8,202_u8];
_38 = [80_u8,226_u8,101_u8,78_u8,146_u8];
_15 = [156_u8,117_u8,71_u8,232_u8,77_u8];
_37 = _42 + _29;
_45 = _37 * _42;
_44.0 = _7;
_30 = [_20,_20,_20];
_12 = _2;
_26 = [13_u8,16_u8,82_u8,130_u8,160_u8];
RET = [247_u8,146_u8,71_u8,232_u8,183_u8];
_12 = _11;
_31 = (-108_i8) << _48.1;
_6 = _8;
Goto(bb17)
}
bb17 = {
Call(_56 = dump_var(13_usize, 13_usize, Move(_13), 11_usize, Move(_11), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(13_usize, 26_usize, Move(_26), 20_usize, Move(_20), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(13_usize, 41_usize, Move(_41), 31_usize, Move(_31), 12_usize, Move(_12), 39_usize, Move(_39)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_56 = dump_var(13_usize, 23_usize, Move(_23), 35_usize, Move(_35), 43_usize, Move(_43), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_56 = dump_var(13_usize, 24_usize, Move(_24), 57_usize, _57, 57_usize, _57, 57_usize, _57), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: usize,mut _2: bool,mut _3: bool,mut _4: usize,mut _5: [u8; 5],mut _6: usize,mut _7: bool,mut _8: usize,mut _9: [u8; 5],mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: [usize; 3]) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _15: f32;
let _16: (bool, i32, i32);
let _17: [u8; 5];
let _18: f32;
let _19: isize;
let _20: *mut [char; 2];
let _21: [i64; 8];
let _22: char;
let _23: Adt46;
let _24: f64;
let _25: [u32; 5];
let _26: (bool, i32, i32);
let _27: (bool, i32, i32);
let _28: [i64; 8];
let _29: [char; 2];
let _30: f32;
let _31: Adt53;
let _32: i8;
let _33: [i8; 1];
let _34: ();
let _35: ();
{
_10 = _3;
_10 = !_3;
_12 = _3;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7 = !_11;
_11 = !_12;
_15 = 40_i8 as f32;
_3 = _11 == _11;
_13 = _4 != _8;
RET = [9223372036854775807_isize,(-9223372036854775808_isize)];
Call(_3 = fn15(_7, _12, _11, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = _1 as f32;
_15 = 165_u8 as f32;
Goto(bb2)
}
bb2 = {
_15 = 1466195143_u32 as f32;
Goto(bb3)
}
bb3 = {
_5 = [151_u8,228_u8,190_u8,66_u8,236_u8];
_1 = _6 * _6;
_15 = 32869132_i32 as f32;
_15 = 19112_u16 as f32;
_13 = !_10;
_16.0 = !_11;
_16.2 = (-1450158733_i32) + 1854483121_i32;
_11 = _2 > _2;
_5 = [102_u8,160_u8,147_u8,191_u8,185_u8];
_6 = !_1;
_3 = _2;
_11 = !_13;
_16.0 = !_7;
_17 = [198_u8,173_u8,34_u8,23_u8,151_u8];
_14 = [_6,_8,_4];
_18 = _15;
_13 = _7;
RET = [9223372036854775807_isize,9223372036854775807_isize];
_11 = !_12;
_7 = !_3;
Goto(bb4)
}
bb4 = {
_13 = _7 ^ _16.0;
_14 = [_6,_1,_6];
_2 = !_16.0;
_7 = !_13;
Call(_4 = fn16(_7, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = _3 > _11;
_3 = _8 >= _8;
_18 = -_15;
_16 = (_7, 1143169213_i32, 1619503364_i32);
_21 = [346536949107607138_i64,(-1136138395814600861_i64),5584587407567511234_i64,(-2757224239280681944_i64),7091367230772954086_i64,(-311001972653718578_i64),4263190563630875274_i64,(-1605610202174572514_i64)];
_11 = _16.2 <= _16.1;
_11 = _7;
_11 = _2;
_5 = _17;
_14 = [_6,_6,_1];
_17 = _9;
_16 = (_13, (-1091061037_i32), 1511957816_i32);
_11 = _13;
_19 = 9223372036854775807_isize;
_21 = [(-2659552471354584035_i64),(-3679770874999839086_i64),8153962828662447261_i64,(-3730866097857442204_i64),(-1154725736239995304_i64),2069856956561106468_i64,2698604362882643622_i64,469036203458668750_i64];
_26.2 = -_16.1;
_24 = _18 as f64;
_26.2 = _16.2 * _16.2;
_22 = '\u{5f6df}';
_26.1 = _16.2;
_23 = Adt46::Variant2 { fld0: _21 };
_4 = (-168127434324709766286115503198795071047_i128) as usize;
match _16.1 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
340282366920938463463374607430677150419 => bb9,
_ => bb8
}
}
bb6 = {
_15 = _1 as f32;
_15 = 165_u8 as f32;
Goto(bb2)
}
bb7 = {
_5 = [151_u8,228_u8,190_u8,66_u8,236_u8];
_1 = _6 * _6;
_15 = 32869132_i32 as f32;
_15 = 19112_u16 as f32;
_13 = !_10;
_16.0 = !_11;
_16.2 = (-1450158733_i32) + 1854483121_i32;
_11 = _2 > _2;
_5 = [102_u8,160_u8,147_u8,191_u8,185_u8];
_6 = !_1;
_3 = _2;
_11 = !_13;
_16.0 = !_7;
_17 = [198_u8,173_u8,34_u8,23_u8,151_u8];
_14 = [_6,_8,_4];
_18 = _15;
_13 = _7;
RET = [9223372036854775807_isize,9223372036854775807_isize];
_11 = !_12;
_7 = !_3;
Goto(bb4)
}
bb8 = {
_15 = 1466195143_u32 as f32;
Goto(bb3)
}
bb9 = {
_26.1 = !_16.2;
_15 = _18 + _18;
_26 = (_7, _16.2, _16.2);
_23 = Adt46::Variant2 { fld0: _21 };
_16 = (_10, _26.2, _26.1);
_19 = !9223372036854775807_isize;
_8 = _19 as usize;
_17 = [113_u8,16_u8,160_u8,181_u8,83_u8];
_12 = !_11;
_4 = _1 ^ _6;
_8 = !_6;
place!(Field::<[i64; 8]>(Variant(_23, 2), 0)) = [2080528175009525214_i64,7925852047929372700_i64,(-6139461776122248200_i64),(-3592844510876161240_i64),3616856506925710701_i64,(-3126521948541000838_i64),7270363848214606202_i64,4620424194021360486_i64];
place!(Field::<[i64; 8]>(Variant(_23, 2), 0)) = [(-405246869705531438_i64),795969043618865195_i64,(-8613709236569367427_i64),(-3555641922314502872_i64),6147152077381957775_i64,4791708950561578064_i64,3016989072340919820_i64,5130874663376658998_i64];
_14 = [_4,_8,_8];
_5 = [197_u8,86_u8,253_u8,55_u8,66_u8];
_11 = _12 | _16.0;
_4 = !_6;
RET = [_19,_19];
_15 = -_18;
_27 = _16;
_20 = core::ptr::addr_of_mut!(_29);
_16.0 = _13;
_26.0 = _2 ^ _13;
_25 = [3983471188_u32,3497290455_u32,4147663370_u32,3902769435_u32,1793769618_u32];
Goto(bb10)
}
bb10 = {
_5 = _17;
_11 = !_2;
_27.1 = -_26.1;
_13 = !_10;
_26.1 = 51_i8 as i32;
SetDiscriminant(_23, 2);
_28 = _21;
_29 = [_22,_22];
_26.2 = 61_i8 as i32;
(*_20) = [_22,_22];
_26 = _16;
Goto(bb11)
}
bb11 = {
_19 = -(-9223372036854775808_isize);
_12 = !_2;
Goto(bb12)
}
bb12 = {
_10 = !_26.0;
_9 = _5;
_12 = _27.0 ^ _10;
_3 = _6 != _6;
_18 = _15 + _15;
_8 = _6 * _6;
_4 = _6 | _6;
_10 = _12;
RET = [_19,_19];
_6 = _1;
_2 = _3;
_26 = (_12, _27.1, _27.2);
_20 = core::ptr::addr_of_mut!((*_20));
_12 = _7;
_26.1 = _22 as i32;
place!(Field::<[i64; 8]>(Variant(_23, 2), 0)) = [(-7713100221523475442_i64),(-1584976847363862322_i64),6042109202498028498_i64,(-8158299667822609887_i64),(-651579252275036458_i64),6198602293679319747_i64,3175052336975442813_i64,(-9178754644862817739_i64)];
_30 = -_18;
place!(Field::<[i64; 8]>(Variant(_23, 2), 0)) = _21;
_24 = _30 as f64;
_16.2 = _27.1 >> _27.2;
_17 = [185_u8,101_u8,71_u8,27_u8,210_u8];
RET = [_19,_19];
RET = [_19,_19];
_19 = 27_isize;
Goto(bb13)
}
bb13 = {
_32 = -(-92_i8);
_22 = '\u{dda72}';
match _16.1 {
0 => bb12,
1 => bb14,
2 => bb15,
1511957816 => bb17,
_ => bb16
}
}
bb14 = {
_5 = [151_u8,228_u8,190_u8,66_u8,236_u8];
_1 = _6 * _6;
_15 = 32869132_i32 as f32;
_15 = 19112_u16 as f32;
_13 = !_10;
_16.0 = !_11;
_16.2 = (-1450158733_i32) + 1854483121_i32;
_11 = _2 > _2;
_5 = [102_u8,160_u8,147_u8,191_u8,185_u8];
_6 = !_1;
_3 = _2;
_11 = !_13;
_16.0 = !_7;
_17 = [198_u8,173_u8,34_u8,23_u8,151_u8];
_14 = [_6,_8,_4];
_18 = _15;
_13 = _7;
RET = [9223372036854775807_isize,9223372036854775807_isize];
_11 = !_12;
_7 = !_3;
Goto(bb4)
}
bb15 = {
_13 = _3 > _11;
_3 = _8 >= _8;
_18 = -_15;
_16 = (_7, 1143169213_i32, 1619503364_i32);
_21 = [346536949107607138_i64,(-1136138395814600861_i64),5584587407567511234_i64,(-2757224239280681944_i64),7091367230772954086_i64,(-311001972653718578_i64),4263190563630875274_i64,(-1605610202174572514_i64)];
_11 = _16.2 <= _16.1;
_11 = _7;
_11 = _2;
_5 = _17;
_14 = [_6,_6,_1];
_17 = _9;
_16 = (_13, (-1091061037_i32), 1511957816_i32);
_11 = _13;
_19 = 9223372036854775807_isize;
_21 = [(-2659552471354584035_i64),(-3679770874999839086_i64),8153962828662447261_i64,(-3730866097857442204_i64),(-1154725736239995304_i64),2069856956561106468_i64,2698604362882643622_i64,469036203458668750_i64];
_26.2 = -_16.1;
_24 = _18 as f64;
_26.2 = _16.2 * _16.2;
_22 = '\u{5f6df}';
_26.1 = _16.2;
_23 = Adt46::Variant2 { fld0: _21 };
_4 = (-168127434324709766286115503198795071047_i128) as usize;
match _16.1 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
340282366920938463463374607430677150419 => bb9,
_ => bb8
}
}
bb16 = {
_5 = [151_u8,228_u8,190_u8,66_u8,236_u8];
_1 = _6 * _6;
_15 = 32869132_i32 as f32;
_15 = 19112_u16 as f32;
_13 = !_10;
_16.0 = !_11;
_16.2 = (-1450158733_i32) + 1854483121_i32;
_11 = _2 > _2;
_5 = [102_u8,160_u8,147_u8,191_u8,185_u8];
_6 = !_1;
_3 = _2;
_11 = !_13;
_16.0 = !_7;
_17 = [198_u8,173_u8,34_u8,23_u8,151_u8];
_14 = [_6,_8,_4];
_18 = _15;
_13 = _7;
RET = [9223372036854775807_isize,9223372036854775807_isize];
_11 = !_12;
_7 = !_3;
Goto(bb4)
}
bb17 = {
_17 = [155_u8,149_u8,95_u8,179_u8,199_u8];
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(14_usize, 13_usize, Move(_13), 12_usize, Move(_12), 28_usize, Move(_28), 29_usize, Move(_29)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(14_usize, 1_usize, Move(_1), 19_usize, Move(_19), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(14_usize, 27_usize, Move(_27), 17_usize, Move(_17), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool) -> bool {
mir! {
type RET = bool;
let _5: Adt49;
let _6: isize;
let _7: ();
let _8: ();
{
RET = !_3;
_2 = _1;
_4 = RET | _2;
_3 = !_4;
_1 = _2;
_1 = !_4;
_4 = RET;
_3 = RET;
_4 = !_1;
_4 = _3;
_6 = 3321086527275529897_i64 as isize;
RET = _2 == _3;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(15_usize, 6_usize, Move(_6), 2_usize, Move(_2), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: f32) -> usize {
mir! {
type RET = usize;
let _3: ([isize; 2],);
let _4: [i16; 1];
let _5: &'static u32;
let _6: i128;
let _7: u8;
let _8: [i64; 8];
let _9: ([usize; 3], *mut i128, f64);
let _10: f64;
let _11: f64;
let _12: u64;
let _13: isize;
let _14: f64;
let _15: isize;
let _16: Adt55;
let _17: ([isize; 2],);
let _18: char;
let _19: f32;
let _20: u128;
let _21: bool;
let _22: u8;
let _23: i8;
let _24: [u8; 5];
let _25: f32;
let _26: *mut i64;
let _27: (bool, i32, i32);
let _28: [u32; 5];
let _29: [u32; 5];
let _30: Adt47;
let _31: f64;
let _32: isize;
let _33: isize;
let _34: isize;
let _35: ([isize; 2],);
let _36: [usize; 3];
let _37: bool;
let _38: ();
let _39: ();
{
_2 = 85_u8 as f32;
RET = 725163948740209379_usize;
match RET {
0 => bb1,
725163948740209379 => bb3,
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
_4 = [3632_i16];
RET = 31304_u16 as usize;
_1 = !true;
RET = 207094610401097418_usize & 3_usize;
RET = 7696641230013222018_usize;
_1 = false;
_4 = [(-2695_i16)];
_4 = [14903_i16];
RET = !3661887914807197224_usize;
_3.0 = [(-9223372036854775808_isize),(-11_isize)];
RET = !6_usize;
RET = 42114_u16 as usize;
_2 = (-1546780806_i32) as f32;
_3.0 = [(-9223372036854775808_isize),9223372036854775807_isize];
_4 = [8445_i16];
RET = 228_u8 as usize;
_4 = [5442_i16];
_6 = 148610027658395238673170889031836568585_i128;
_3.0 = [(-9223372036854775808_isize),45_isize];
RET = !1_usize;
_8 = [6735032931826138108_i64,8781347347450273442_i64,390062414839060336_i64,(-7999229798119148574_i64),2696894213696131873_i64,7884715312539045049_i64,8107747095430500087_i64,2371051430801967337_i64];
_3.0 = [9223372036854775807_isize,9223372036854775807_isize];
_8 = [(-6043979180669056963_i64),(-1184969175977275841_i64),2177105366331334823_i64,4475040675672865347_i64,(-2409607262589079755_i64),4789028748734273825_i64,193894701575937817_i64,280390208844067175_i64];
_7 = 13116513536272289798_u64 as u8;
_3.0 = [(-60_isize),(-66_isize)];
RET = !3373196306750259932_usize;
_6 = (-165031834666226826833975300517452244111_i128) * (-152318836974374089778595355404261685442_i128);
Goto(bb4)
}
bb4 = {
_3.0 = [9223372036854775807_isize,100_isize];
_1 = !true;
RET = 4910861975808226538_usize;
_2 = (-819623353_i32) as f32;
_6 = (-165562933040176829063900116787569691251_i128);
_9.2 = _2 as f64;
_8 = [(-912891088339024235_i64),4120430734414931850_i64,(-6706490505767926589_i64),(-5844749885370178215_i64),(-2371410544974336971_i64),7845810733729499247_i64,132204425163279909_i64,(-7186617096205170955_i64)];
_1 = false ^ false;
_1 = true;
_8 = [(-3342732488845603640_i64),(-318680739262359541_i64),(-4135464375263986063_i64),(-1838511378696045965_i64),(-7466338325755916448_i64),(-5612445796640935114_i64),(-3214723366760551688_i64),6865738161928456481_i64];
RET = 3_usize - 2851758996584445835_usize;
_10 = 3145242642000064519_u64 as f64;
_9.0 = [RET,RET,RET];
RET = 16208682505538617418_usize;
_4 = [9499_i16];
_2 = _10 as f32;
_6 = 119943952020663276772064802927997683904_i128;
_8 = [5507280801167742884_i64,746914136235200635_i64,1337961337450256805_i64,(-426725137101296465_i64),1836401253682442733_i64,4627492357358283495_i64,(-6681907082229810574_i64),8505331381112659463_i64];
_9.2 = -_10;
_6 = 11142447350761088851485637706231404806_i128;
Goto(bb5)
}
bb5 = {
_9.2 = _10 + _10;
_12 = 4712832479519311670_u64 >> _7;
_3.0 = [9223372036854775807_isize,9223372036854775807_isize];
_6 = 1287379376_u32 as i128;
_6 = 20790184035915024419727188830771765974_u128 as i128;
_13 = _12 as isize;
_11 = _10 + _9.2;
_8 = [(-2147690192717751947_i64),548697605598256574_i64,536277070236421272_i64,331209271766148653_i64,(-6115909632068206829_i64),5944521835164776315_i64,547479893911683198_i64,(-506032233027811619_i64)];
_7 = !255_u8;
_6 = 136089952475866182015185862902965301628_i128 ^ 116088550584142926745382301099879187012_i128;
_3.0 = [_13,_13];
_9.0 = [RET,RET,RET];
Goto(bb6)
}
bb6 = {
_14 = _11;
_6 = -(-108112732321136663461738925072354401383_i128);
RET = _12 as usize;
Goto(bb7)
}
bb7 = {
_13 = (-9223372036854775808_isize);
RET = !4_usize;
_9.2 = -_11;
_14 = -_9.2;
RET = !9012616877926173799_usize;
_9.2 = _11;
RET = 1672777132216475119_usize - 6130944072240254172_usize;
_11 = _14 * _9.2;
_3.0 = [_13,_13];
_9.2 = _11 + _14;
_1 = !true;
_13 = -9223372036854775807_isize;
_2 = (-10_i8) as f32;
_17.0 = [_13,_13];
_9.2 = (-66897018_i32) as f64;
_15 = _7 as isize;
_14 = -_11;
_14 = _2 as f64;
_9.1 = core::ptr::addr_of_mut!(_6);
_8 = [3518664803070060400_i64,8438774689428978829_i64,4802572890689550384_i64,6484624381484809067_i64,(-670726365551814180_i64),6034775905527258694_i64,1795940838678427190_i64,6891191076010633257_i64];
_17.0 = [_15,_15];
_17 = (_3.0,);
_4 = [24847_i16];
_9.2 = _11 * _10;
Goto(bb8)
}
bb8 = {
_8 = [1446061367875805285_i64,(-8758087025145914689_i64),6498610720485505007_i64,(-8469500492739043853_i64),7783108636134481104_i64,(-4830760138401460649_i64),2062803100319810838_i64,(-882578044342043809_i64)];
_6 = RET as i128;
_18 = '\u{4f270}';
_10 = _14;
_17 = _3;
_3.0 = _17.0;
_3 = (_17.0,);
_13 = _15;
_13 = _15 >> RET;
_13 = RET as isize;
_18 = '\u{10e330}';
Goto(bb9)
}
bb9 = {
_18 = '\u{26bdd}';
_13 = -_15;
_8 = [(-3966247336087769357_i64),2665083089168834220_i64,7695943775320400086_i64,5215565555217057952_i64,(-2647978678523216848_i64),7742373013878456596_i64,(-3293285848382791883_i64),6254064294784490559_i64];
RET = !14944121176430004799_usize;
_19 = _2 - _2;
_10 = _19 as f64;
_21 = !_1;
_10 = _9.2 + _9.2;
_17.0 = [_13,_15];
_4 = [(-20310_i16)];
_2 = -_19;
_2 = _19;
_14 = 159491749005065441806370523661201423329_u128 as f64;
_14 = _10;
_14 = -_10;
Goto(bb10)
}
bb10 = {
_15 = _13 >> _7;
_8 = [(-7802860357550411161_i64),2312342066356551872_i64,4828262297036911304_i64,(-461679213950859075_i64),(-4977332623990915245_i64),(-1566973881427803584_i64),(-4968936031473154369_i64),(-953109307076487765_i64)];
_22 = 47105904865024490059603826916843015973_u128 as u8;
_14 = _11;
_3.0 = _17.0;
_1 = !_21;
_10 = _9.2;
RET = !1863193974767023782_usize;
_10 = _14 - _9.2;
_24 = [_7,_7,_7,_22,_22];
_9.1 = core::ptr::addr_of_mut!(_6);
_14 = _9.2;
_19 = -_2;
_9.0 = [RET,RET,RET];
_9.0 = [RET,RET,RET];
RET = _2 as usize;
_17.0 = [_13,_13];
_17.0 = [_13,_13];
_15 = _13 | _13;
_17.0 = _3.0;
_25 = _6 as f32;
Goto(bb11)
}
bb11 = {
_25 = _19 * _2;
_27.1 = (-1535343436_i32) + (-734005512_i32);
_21 = !_1;
_24 = [_22,_7,_7,_22,_22];
_13 = !_15;
_23 = !90_i8;
_13 = _15 * _15;
_12 = !16270138653390200642_u64;
_14 = _10 + _11;
_4 = [(-20354_i16)];
Goto(bb12)
}
bb12 = {
_27 = (_21, (-472229150_i32), 879132992_i32);
_10 = -_14;
_20 = !253285613696383621753646580856789838383_u128;
_10 = _9.2;
_12 = 4415192590175130866_u64 - 718793261405678518_u64;
_8 = [(-1984921326443822561_i64),6969963490139775419_i64,5399366916891894179_i64,7527214538725371094_i64,4834178408617331742_i64,(-129403082884937573_i64),5820288627719304236_i64,155559350486105335_i64];
_17 = _3;
_30.fld0 = core::ptr::addr_of_mut!(_6);
_13 = -_15;
_27.2 = _27.1;
_10 = -_14;
_4 = [30108_i16];
_19 = -_25;
_23 = -(-101_i8);
Goto(bb13)
}
bb13 = {
RET = 59726_u16 as usize;
_9.2 = _14;
_17.0 = [_15,_15];
_9.2 = _14;
_27.1 = _19 as i32;
_27.0 = !_21;
_3 = _17;
_33 = -_13;
_28 = [424224963_u32,1907073729_u32,1411249072_u32,435103301_u32,1399063978_u32];
_35.0 = _17.0;
_32 = _13;
_25 = _19 + _19;
_32 = RET as isize;
_32 = _1 as isize;
RET = _27.1 as usize;
match _27.2 {
0 => bb14,
340282366920938463463374607431295982306 => bb16,
_ => bb15
}
}
bb14 = {
_14 = _11;
_6 = -(-108112732321136663461738925072354401383_i128);
RET = _12 as usize;
Goto(bb7)
}
bb15 = {
_15 = _13 >> _7;
_8 = [(-7802860357550411161_i64),2312342066356551872_i64,4828262297036911304_i64,(-461679213950859075_i64),(-4977332623990915245_i64),(-1566973881427803584_i64),(-4968936031473154369_i64),(-953109307076487765_i64)];
_22 = 47105904865024490059603826916843015973_u128 as u8;
_14 = _11;
_3.0 = _17.0;
_1 = !_21;
_10 = _9.2;
RET = !1863193974767023782_usize;
_10 = _14 - _9.2;
_24 = [_7,_7,_7,_22,_22];
_9.1 = core::ptr::addr_of_mut!(_6);
_14 = _9.2;
_19 = -_2;
_9.0 = [RET,RET,RET];
_9.0 = [RET,RET,RET];
RET = _2 as usize;
_17.0 = [_13,_13];
_17.0 = [_13,_13];
_15 = _13 | _13;
_17.0 = _3.0;
_25 = _6 as f32;
Goto(bb11)
}
bb16 = {
RET = _6 as usize;
_6 = 59876597065856430480629126546235211327_i128;
_27.0 = _9.2 >= _9.2;
_36 = _9.0;
Goto(bb17)
}
bb17 = {
Call(_38 = dump_var(16_usize, 33_usize, Move(_33), 15_usize, Move(_15), 32_usize, Move(_32), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(16_usize, 36_usize, Move(_36), 8_usize, Move(_8), 20_usize, Move(_20), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(16_usize, 12_usize, Move(_12), 1_usize, Move(_1), 4_usize, Move(_4), 39_usize, _39), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [isize; 2],mut _2: [isize; 2],mut _3: [i64; 8],mut _4: [isize; 2],mut _5: [isize; 2],mut _6: ([isize; 2],),mut _7: [i64; 8],mut _8: [isize; 2],mut _9: [i64; 8],mut _10: [u8; 5],mut _11: ([isize; 2],),mut _12: [i64; 8]) -> [u8; 5] {
mir! {
type RET = [u8; 5];
let _13: [char; 2];
let _14: [u8; 5];
let _15: (i16, i8, *mut [char; 2]);
let _16: bool;
let _17: (char,);
let _18: Adt51;
let _19: [i64; 8];
let _20: char;
let _21: Adt57;
let _22: char;
let _23: f64;
let _24: char;
let _25: isize;
let _26: [u8; 5];
let _27: bool;
let _28: [i16; 1];
let _29: [usize; 3];
let _30: u8;
let _31: bool;
let _32: u32;
let _33: f32;
let _34: bool;
let _35: Adt56;
let _36: *mut [char; 2];
let _37: Adt50;
let _38: [usize; 3];
let _39: [usize; 3];
let _40: u64;
let _41: [char; 2];
let _42: i128;
let _43: Adt41;
let _44: ();
let _45: ();
{
_9 = [6550020564548538470_i64,(-7489350553602349112_i64),(-3686612573682400924_i64),8375219662486166561_i64,(-659498889190279053_i64),(-3823233313393514857_i64),(-2075602854448478341_i64),6579283553044863284_i64];
RET = [2_u8,121_u8,246_u8,145_u8,161_u8];
_9 = [8745974162200594050_i64,(-4397070324573069830_i64),(-5616960066457607781_i64),4647785200570539607_i64,4082931771262094255_i64,1044254951484259958_i64,5829216773556168268_i64,(-7761071979935977483_i64)];
_11 = (_1,);
_10 = RET;
_12 = _7;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_12 = [(-1592479445820941453_i64),(-5609432132584932607_i64),680352375461106192_i64,(-1323903655162206409_i64),(-1969094426623005561_i64),5624374639271698812_i64,974547118679561093_i64,3273767092589173927_i64];
_10 = [247_u8,229_u8,49_u8,127_u8,5_u8];
_11 = _6;
_6 = _11;
_12 = [(-7613638633878719449_i64),6726822351592970376_i64,4763604598071957812_i64,106934033771487020_i64,269823987298355537_i64,8816112692442560986_i64,(-7723233420745630491_i64),8236775732351992686_i64];
_5 = [(-128_isize),(-9223372036854775808_isize)];
_14 = [128_u8,148_u8,236_u8,179_u8,31_u8];
_8 = [(-76_isize),9223372036854775807_isize];
Call(RET = core::intrinsics::transmute(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _9;
_8 = [9223372036854775807_isize,(-43_isize)];
_9 = [(-4218578608558189546_i64),(-7317273396109489922_i64),(-1685411746740177228_i64),(-1786012935514714946_i64),4115566596244364607_i64,4233406601862746730_i64,2421249119094803948_i64,(-128498419019528727_i64)];
_3 = [(-3957696676030629225_i64),(-2989193667727931339_i64),5317490186807758094_i64,(-4334230943604461224_i64),(-7502660121768131646_i64),1172876555278862256_i64,6653705579155334873_i64,(-2763955591597698392_i64)];
_15.0 = 1238236913_i32 as i16;
_17.0 = '\u{7dd52}';
_13 = [_17.0,_17.0];
_8 = [10_isize,9223372036854775807_isize];
_19 = [(-1954287479575050743_i64),(-821936773798687194_i64),(-4379221626045630031_i64),(-3213810274887742258_i64),(-98801098478111533_i64),(-4814920921519418127_i64),1338371763778660527_i64,(-4495684636837563932_i64)];
RET = _14;
Goto(bb2)
}
bb2 = {
_7 = _9;
_20 = _17.0;
_17 = (_20,);
_1 = _2;
_11.0 = _2;
_21.fld0 = Adt41::Variant3 { fld0: _13 };
_11.0 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_6.0 = _8;
_21.fld1 = core::ptr::addr_of_mut!(_13);
SetDiscriminant(_21.fld0, 2);
_15.1 = (-14_i8);
_15 = ((-11541_i16), 55_i8, _21.fld1);
_16 = _17.0 < _20;
_22 = _17.0;
_2 = [9223372036854775807_isize,(-28_isize)];
place!(Field::<[i8; 1]>(Variant(_21.fld0, 2), 0)) = [_15.1];
_11.0 = _2;
_8 = [9223372036854775807_isize,(-16_isize)];
_11 = _6;
place!(Field::<[i8; 1]>(Variant(_21.fld0, 2), 0)) = [_15.1];
_24 = _20;
place!(Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3)) = _15.2;
_21.fld1 = _15.2;
place!(Field::<[i8; 1]>(Variant(_21.fld0, 2), 0)) = [_15.1];
_21.fld1 = Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3);
place!(Field::<[char; 2]>(Variant(_21.fld0, 2), 1)) = _13;
match _15.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
55 => bb9,
_ => bb8
}
}
bb3 = {
_7 = _9;
_8 = [9223372036854775807_isize,(-43_isize)];
_9 = [(-4218578608558189546_i64),(-7317273396109489922_i64),(-1685411746740177228_i64),(-1786012935514714946_i64),4115566596244364607_i64,4233406601862746730_i64,2421249119094803948_i64,(-128498419019528727_i64)];
_3 = [(-3957696676030629225_i64),(-2989193667727931339_i64),5317490186807758094_i64,(-4334230943604461224_i64),(-7502660121768131646_i64),1172876555278862256_i64,6653705579155334873_i64,(-2763955591597698392_i64)];
_15.0 = 1238236913_i32 as i16;
_17.0 = '\u{7dd52}';
_13 = [_17.0,_17.0];
_8 = [10_isize,9223372036854775807_isize];
_19 = [(-1954287479575050743_i64),(-821936773798687194_i64),(-4379221626045630031_i64),(-3213810274887742258_i64),(-98801098478111533_i64),(-4814920921519418127_i64),1338371763778660527_i64,(-4495684636837563932_i64)];
RET = _14;
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
_15.1 = (-104_i8) ^ (-72_i8);
_15.2 = Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3);
_17 = (_24,);
_9 = [(-7427528799591186827_i64),5202810810207697997_i64,(-3806392658999621112_i64),(-1379192914926078567_i64),(-7612156778989165547_i64),2191431167879989966_i64,(-8493436515371288099_i64),8505245327691855789_i64];
RET = [217_u8,90_u8,148_u8,173_u8,38_u8];
RET = _10;
_27 = _16;
_12 = [3820643782569069648_i64,6519075754382972252_i64,7796690913090333835_i64,2593296638618880958_i64,(-1098827992345691888_i64),(-5200043182145691483_i64),(-312102633029717721_i64),(-6737002463263886829_i64)];
_17.0 = _22;
_11.0 = [(-48_isize),(-9223372036854775808_isize)];
_15 = (863_i16, 88_i8, _21.fld1);
_2 = _1;
_25 = 9223372036854775807_isize;
_29 = [5_usize,0_usize,13356894782942560794_usize];
_35.fld2 = [2820338662_u32,4064234750_u32,1154353830_u32,2531527823_u32,2297131076_u32];
_35.fld5.fld3 = core::ptr::addr_of_mut!(place!(Field::<[i8; 1]>(Variant(_21.fld0, 2), 0)));
Goto(bb10)
}
bb10 = {
_6 = _11;
_35.fld2 = [724472438_u32,3175806960_u32,3404834367_u32,1139493766_u32,2556931952_u32];
_30 = 83_u8;
_35.fld5.fld4 = _15.0 - _15.0;
place!(Field::<[char; 2]>(Variant(_21.fld0, 2), 1)) = [_22,_24];
match _15.1 {
0 => bb7,
1 => bb2,
88 => bb11,
_ => bb5
}
}
bb11 = {
place!(Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3)) = core::ptr::addr_of_mut!(_13);
_22 = _24;
_33 = 1507923462_u32 as f32;
_3 = [657066789783407284_i64,7914873319285056705_i64,(-2205229876419801906_i64),2503692185082898208_i64,(-7698484605104294068_i64),(-1274979856855311072_i64),(-5276234568011287635_i64),2777676551440872340_i64];
_35.fld5.fld4 = _15.0 | _15.0;
_24 = _20;
place!(Field::<[char; 2]>(Variant(_21.fld0, 2), 1)) = [_20,_17.0];
_8 = [_25,_25];
_35.fld5.fld2 = [_15.0];
_34 = _22 == _17.0;
_15 = (_35.fld5.fld4, (-76_i8), Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3));
place!(Field::<i32>(Variant(_21.fld0, 2), 2)) = (-2080540972_i32);
_11 = (_6.0,);
_39 = [12898291905642978034_usize,4_usize,6_usize];
_35.fld5.fld1 = core::ptr::addr_of_mut!(_40);
_35.fld1.1 = !_15.1;
_35.fld0 = -100361965907152619362250348709963449431_i128;
place!(Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3)) = _15.2;
_13 = Field::<[char; 2]>(Variant(_21.fld0, 2), 1);
_37 = Adt50::Variant1 { fld0: _34,fld1: _30,fld2: _15.1 };
_35.fld4 = [2_usize,16509907081752969363_usize,3_usize];
_24 = _17.0;
_15 = (_35.fld5.fld4, Field::<i8>(Variant(_37, 1), 2), Field::<*mut [char; 2]>(Variant(_21.fld0, 2), 3));
_11.0 = [_25,_25];
_17.0 = _24;
_19 = _12;
_41 = [_20,_24];
match Field::<i8>(Variant(_37, 1), 2) {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb9,
4 => bb8,
5 => bb6,
6 => bb12,
340282366920938463463374607431768211380 => bb14,
_ => bb13
}
}
bb12 = {
_7 = _9;
_8 = [9223372036854775807_isize,(-43_isize)];
_9 = [(-4218578608558189546_i64),(-7317273396109489922_i64),(-1685411746740177228_i64),(-1786012935514714946_i64),4115566596244364607_i64,4233406601862746730_i64,2421249119094803948_i64,(-128498419019528727_i64)];
_3 = [(-3957696676030629225_i64),(-2989193667727931339_i64),5317490186807758094_i64,(-4334230943604461224_i64),(-7502660121768131646_i64),1172876555278862256_i64,6653705579155334873_i64,(-2763955591597698392_i64)];
_15.0 = 1238236913_i32 as i16;
_17.0 = '\u{7dd52}';
_13 = [_17.0,_17.0];
_8 = [10_isize,9223372036854775807_isize];
_19 = [(-1954287479575050743_i64),(-821936773798687194_i64),(-4379221626045630031_i64),(-3213810274887742258_i64),(-98801098478111533_i64),(-4814920921519418127_i64),1338371763778660527_i64,(-4495684636837563932_i64)];
RET = _14;
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
SetDiscriminant(_21.fld0, 2);
_38 = _39;
_11 = (_6.0,);
_11.0 = [_25,_25];
_35.fld1.0 = _22 as i16;
_35.fld4 = [2588597147988620347_usize,14826917657798807572_usize,6_usize];
_17.0 = _20;
_28 = [_15.0];
RET = [Field::<u8>(Variant(_37, 1), 1),_30,Field::<u8>(Variant(_37, 1), 1),Field::<u8>(Variant(_37, 1), 1),_30];
_10 = [_30,_30,Field::<u8>(Variant(_37, 1), 1),_30,Field::<u8>(Variant(_37, 1), 1)];
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(17_usize, 1_usize, Move(_1), 6_usize, Move(_6), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(17_usize, 22_usize, Move(_22), 11_usize, Move(_11), 27_usize, Move(_27), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(17_usize, 4_usize, Move(_4), 20_usize, Move(_20), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(17_usize, 29_usize, Move(_29), 41_usize, Move(_41), 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: f64,mut _2: ([isize; 2],),mut _3: [isize; 2],mut _4: f64,mut _5: usize) -> f32 {
mir! {
type RET = f32;
let _6: [i8; 1];
let _7: char;
let _8: [i8; 1];
let _9: f32;
let _10: isize;
let _11: (bool, i32, i32);
let _12: Adt49;
let _13: [u8; 5];
let _14: [u32; 5];
let _15: (usize,);
let _16: bool;
let _17: u32;
let _18: Adt56;
let _19: char;
let _20: char;
let _21: char;
let _22: [usize; 3];
let _23: bool;
let _24: i128;
let _25: [u8; 5];
let _26: u16;
let _27: Adt51;
let _28: *mut [char; 2];
let _29: [u32; 5];
let _30: [char; 2];
let _31: ();
let _32: ();
{
RET = (-21212_i16) as f32;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = [9223372036854775807_isize,(-38_isize)];
_5 = 1_usize;
_2.0 = [_3[_5],_3[_5]];
_6 = [(-103_i8)];
_3 = _2.0;
_2.0 = [_3[_5],_3[_5]];
_1 = 64759_u16 as f64;
_5 = 13504017447122892439_usize;
_7 = '\u{94880}';
_5 = 1_usize >> 8345447645790352557_i64;
Goto(bb1)
}
bb1 = {
_3 = [62_isize,63_isize];
_6 = [(-112_i8)];
RET = 28346_u16 as f32;
_8 = _6;
_3 = [78_isize,9223372036854775807_isize];
_1 = _4 + _4;
_1 = _4;
_6 = _8;
_2 = (_3,);
_4 = -_1;
_4 = _1 * _1;
_7 = '\u{3e57}';
_11.2 = !(-944315780_i32);
_11 = (false, 2036766391_i32, (-783317303_i32));
_11 = (true, 1369028894_i32, 1859393085_i32);
_2 = (_3,);
RET = 15488825151247687913_u64 as f32;
_8 = _6;
_3 = [9223372036854775807_isize,117_isize];
_11.0 = _4 > _4;
_11.0 = !false;
RET = 57_i8 as f32;
Goto(bb2)
}
bb2 = {
_11 = (false, 156117584_i32, 1886108991_i32);
_11.2 = -_11.1;
_2.0 = [9223372036854775807_isize,72_isize];
_9 = RET * RET;
_11.2 = _11.0 as i32;
_13 = [74_u8,25_u8,115_u8,35_u8,185_u8];
_11 = (false, 260636098_i32, (-1834264490_i32));
_5 = 5318875861948712973_usize << _11.1;
RET = _9 - _9;
_11.1 = _11.2 - _11.2;
_13 = [141_u8,169_u8,29_u8,150_u8,130_u8];
_13 = [234_u8,162_u8,250_u8,120_u8,228_u8];
match _11.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607429933946966 => bb9,
_ => bb8
}
}
bb3 = {
_3 = [62_isize,63_isize];
_6 = [(-112_i8)];
RET = 28346_u16 as f32;
_8 = _6;
_3 = [78_isize,9223372036854775807_isize];
_1 = _4 + _4;
_1 = _4;
_6 = _8;
_2 = (_3,);
_4 = -_1;
_4 = _1 * _1;
_7 = '\u{3e57}';
_11.2 = !(-944315780_i32);
_11 = (false, 2036766391_i32, (-783317303_i32));
_11 = (true, 1369028894_i32, 1859393085_i32);
_2 = (_3,);
RET = 15488825151247687913_u64 as f32;
_8 = _6;
_3 = [9223372036854775807_isize,117_isize];
_11.0 = _4 > _4;
_11.0 = !false;
RET = 57_i8 as f32;
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
_7 = '\u{36d58}';
_11.1 = _11.2;
_5 = 6_usize >> _11.1;
_10 = !116_isize;
_15.0 = _11.0 as usize;
_1 = _4;
_11.1 = (-64155842618512880568217147550914015033_i128) as i32;
_5 = 8569700785276546677_i64 as usize;
_18.fld5.fld2 = [(-12299_i16)];
_17 = !4136062811_u32;
_18.fld5.fld4 = 7973339285708010623_u64 as i16;
_9 = RET;
_18.fld0 = -15727818878701718299806407761718943300_i128;
_2 = (_3,);
_18.fld2 = [_17,_17,_17,_17,_17];
_11.1 = _11.2 | _11.2;
_15.0 = !_5;
_18.fld1.1 = (-19_i8);
Goto(bb10)
}
bb10 = {
RET = -_9;
_16 = _4 == _4;
_18.fld5.fld2 = [_18.fld5.fld4];
_1 = _4 + _4;
_13 = [67_u8,154_u8,99_u8,224_u8,111_u8];
_8 = [_18.fld1.1];
_18.fld5.fld2 = [_18.fld5.fld4];
_18.fld5.fld3 = core::ptr::addr_of_mut!(_6);
_15.0 = !_5;
_6 = [_18.fld1.1];
Goto(bb11)
}
bb11 = {
_14 = [_17,_17,_17,_17,_17];
_2 = (_3,);
_18.fld0 = (-150214909349432698361011891688373222440_i128);
_18.fld5.fld2 = [_18.fld5.fld4];
_17 = 345322790_u32 >> _11.1;
_15 = (_5,);
_17 = 18807_u16 as u32;
_14 = [_17,_17,_17,_17,_17];
_11 = (_16, (-960907895_i32), (-561832777_i32));
_18.fld5.fld3 = core::ptr::addr_of_mut!(_6);
RET = _9;
Goto(bb12)
}
bb12 = {
_11.1 = _11.2 & _11.2;
RET = 44537605904212871099350212553917711226_u128 as f32;
_1 = _4;
_11 = (_16, (-1685126292_i32), 1770119575_i32);
_18.fld5.fld4 = 19590_i16;
_11.1 = _11.2 << _11.2;
RET = _9;
RET = _9 - _9;
_26 = 13315_u16 - 27029_u16;
_23 = !_11.0;
_8 = _6;
_24 = _11.2 as i128;
_7 = '\u{61d88}';
_15 = (_5,);
_19 = _7;
_8 = [_18.fld1.1];
_10 = (-9223372036854775808_isize) << _11.1;
_20 = _7;
_11.0 = _23 & _23;
_11 = (_23, 1823954695_i32, (-580697867_i32));
Goto(bb13)
}
bb13 = {
RET = -_9;
_9 = _10 as f32;
_5 = _15.0 - _15.0;
_13 = [170_u8,205_u8,136_u8,68_u8,98_u8];
_18.fld4 = [_15.0,_5,_15.0];
_7 = _19;
_24 = _18.fld0;
_3 = [_10,_10];
_4 = -_1;
RET = _9 - _9;
_18.fld1.2 = core::ptr::addr_of_mut!(_30);
_25 = [113_u8,74_u8,254_u8,210_u8,251_u8];
_30 = [_7,_19];
_21 = _20;
_28 = _18.fld1.2;
RET = -_9;
Goto(bb14)
}
bb14 = {
Call(_31 = dump_var(18_usize, 23_usize, Move(_23), 30_usize, Move(_30), 8_usize, Move(_8), 19_usize, Move(_19)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_31 = dump_var(18_usize, 24_usize, Move(_24), 25_usize, Move(_25), 14_usize, Move(_14), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(18_usize, 10_usize, Move(_10), 26_usize, Move(_26), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{1d58}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box((-26_i8)), std::hint::black_box((-26848_i16)), std::hint::black_box((-40198994_i32)), std::hint::black_box((-263769141709549456_i64)), std::hint::black_box(160098415609273834893408526796819624072_i128), std::hint::black_box(14857020891922148837_usize), std::hint::black_box(212_u8), std::hint::black_box(51770_u16), std::hint::black_box(811600504_u32), std::hint::black_box(10635378092423772422_u64), std::hint::black_box(296400322407180282789339881023689555768_u128));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: bool,
fld1: i128,
fld2: [usize; 3],
fld3: i8,
fld4: i16,
fld5: *mut [char; 2],
fld6: i64,

},
Variant1{
fld0: u8,
fld1: char,
fld2: u32,
fld3: *mut i128,
fld4: *const isize,
fld5: *const char,
fld6: [isize; 2],
fld7: (bool, i32, i32),

},
Variant2{
fld0: [i8; 1],
fld1: [char; 2],
fld2: i32,
fld3: *mut [char; 2],

},
Variant3{
fld0: [char; 2],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u128,
fld1: *mut [char; 2],
fld2: [u8; 5],
fld3: [usize; 3],
fld4: *mut u64,
fld5: (i16, i8, *mut [char; 2]),

},
Variant1{
fld0: f64,
fld1: u8,
fld2: (char,),
fld3: i8,
fld4: i16,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: (i16, i8, *mut [char; 2]),
fld1: *mut i64,
fld2: *mut [i8; 1],
fld3: i8,
fld4: i16,

},
Variant1{
fld0: usize,
fld1: (*mut u64, [usize; 3], [u8; 5], [i64; 8], i8),
fld2: [i64; 8],
fld3: u16,
fld4: u8,
fld5: ([isize; 2],),

},
Variant2{
fld0: u8,
fld1: char,
fld2: (*mut i128,),
fld3: i8,
fld4: f64,
fld5: [u32; 5],
fld6: i64,

},
Variant3{
fld0: usize,
fld1: *mut i64,
fld2: isize,
fld3: *mut i128,
fld4: *mut [i8; 1],
fld5: i32,
fld6: [char; 2],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut u64,
fld1: *mut [char; 2],
fld2: ([usize; 3], *mut i128, f64),
fld3: u128,
fld4: (*mut i128,),
fld5: *mut i64,
fld6: (i16, i8, *mut [char; 2]),

},
Variant1{
fld0: *mut [i8; 1],
fld1: [char; 2],
fld2: Adt42,
fld3: *const char,
fld4: f32,
fld5: (char,),
fld6: [u32; 5],

},
Variant2{
fld0: ([isize; 2],),
fld1: [i16; 1],
fld2: u64,
fld3: *mut i64,

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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: ([usize; 3], *mut i128, f64),
fld1: *const char,
fld2: (usize,),
fld3: (*mut i128,),
fld4: i16,
fld5: *mut i128,
fld6: [u32; 5],

},
Variant1{
fld0: u64,
fld1: char,
fld2: Adt44,

},
Variant2{
fld0: [u8; 5],
fld1: (*mut i128,),
fld2: *mut i128,
fld3: i8,
fld4: (usize,),
fld5: [isize; 2],
fld6: i64,
fld7: u64,

},
Variant3{
fld0: bool,
fld1: i32,
fld2: Adt44,
fld3: *mut i64,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: (bool, i32, i32),
fld2: f32,
fld3: (*mut u64, [usize; 3], [u8; 5], [i64; 8], i8),
fld4: Adt43,

},
Variant1{
fld0: [isize; 2],
fld1: usize,
fld2: Adt44,
fld3: (i16, i8, *mut [char; 2]),
fld4: [usize; 3],
fld5: [i8; 1],
fld6: *mut i128,
fld7: i128,

},
Variant2{
fld0: [i64; 8],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: *mut i128,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt43,
fld1: *mut i128,
fld2: f32,

},
Variant1{
fld0: [i64; 8],
fld1: char,
fld2: Adt43,
fld3: [u32; 5],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt43,
fld1: i64,
fld2: [usize; 3],
fld3: [u8; 5],
fld4: [char; 2],

},
Variant1{
fld0: *mut [i8; 1],
fld1: char,
fld2: [u8; 5],
fld3: [char; 2],

},
Variant2{
fld0: bool,
fld1: u8,
fld2: [usize; 3],
fld3: u16,
fld4: *mut [char; 2],

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
fld0: bool,
fld1: [isize; 2],
fld2: (usize,),
fld3: ([isize; 2],),
fld4: Adt43,
fld5: ([usize; 3], *mut i128, f64),
fld6: [usize; 3],

},
Variant1{
fld0: bool,
fld1: u8,
fld2: i8,

},
Variant2{
fld0: ([isize; 2],),
fld1: [u8; 5],
fld2: u64,
fld3: [isize; 2],
fld4: Adt48,
fld5: *mut u64,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt50,
fld1: (usize,),
fld2: Adt47,
fld3: Adt49,
fld4: (*mut i128,),
fld5: *mut i128,
fld6: Adt41,
fld7: (i16, i8, *mut [char; 2]),

},
Variant1{
fld0: Adt46,
fld1: u8,
fld2: Adt49,

},
Variant2{
fld0: i16,
fld1: Adt45,
fld2: *mut i64,

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
fld0: u64,
fld1: f32,
fld2: u8,
fld3: [i16; 1],
fld4: [usize; 3],
fld5: (*mut u64, [usize; 3], [u8; 5], [i64; 8], i8),
fld6: *const isize,
fld7: f64,

},
Variant1{
fld0: Adt44,
fld1: i128,
fld2: Adt42,

},
Variant2{
fld0: u32,
fld1: [isize; 2],
fld2: u16,
fld3: Adt45,
fld4: *mut u64,
fld5: f32,
fld6: (bool, i32, i32),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: Adt42,
fld1: *mut u64,
fld2: [i16; 1],
fld3: *mut [i8; 1],
fld4: i16,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [i16; 1],

},
Variant1{
fld0: [isize; 2],
fld1: *const char,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *mut [char; 2],
fld1: *mut [i8; 1],
fld2: [isize; 2],

},
Variant1{
fld0: ([usize; 3], *mut i128, f64),
fld1: [i8; 1],
fld2: f32,
fld3: ([isize; 2],),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: i128,
fld1: (i16, i8, *mut [char; 2]),
fld2: [u32; 5],
fld3: Adt50,
fld4: [usize; 3],
fld5: Adt53,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: Adt41,
fld1: *mut [char; 2],
}

