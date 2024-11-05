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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: i64,mut _4: u16,mut _5: u32,mut _6: i32) -> usize {
mir! {
type RET = usize;
let _7: u32;
let _8: i8;
let _9: (u128, i8);
let _10: u16;
let _11: [u16; 5];
let _12: u128;
let _13: bool;
let _14: (u128, i8);
let _15: [i64; 2];
let _16: [usize; 8];
let _17: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32));
let _18: *mut usize;
let _19: Adt49;
let _20: [i64; 2];
let _21: *const char;
let _22: u32;
let _23: isize;
let _24: [u8; 4];
let _25: char;
let _26: isize;
let _27: Adt43;
let _28: isize;
let _29: [bool; 7];
let _30: [u8; 4];
let _31: [u16; 5];
let _32: isize;
let _33: [i64; 2];
let _34: isize;
let _35: [bool; 7];
let _36: (*const char, isize);
let _37: [u8; 7];
let _38: Adt56;
let _39: (u128, [u128; 1], &'static [u8; 4], [isize; 1]);
let _40: [u128; 1];
let _41: f64;
let _42: isize;
let _43: char;
let _44: i64;
let _45: u8;
let _46: [i64; 2];
let _47: f64;
let _48: [i16; 6];
let _49: ();
let _50: ();
{
RET = 6_usize + 14520687301431631760_usize;
_6 = 1054940555_i32 << RET;
_2 = 34938993849262056095591616030587305482_i128 as u8;
_1 = _6 > _6;
_6 = (-624439754_i32);
_4 = 21435_u16 | 16085_u16;
RET = 7_usize;
_3 = 4436081339585808849_i64 ^ 1671088559098146647_i64;
_3 = RET as i64;
_5 = 1372554577_u32;
_6 = 643991041_i32 + 1923609675_i32;
_7 = (-111_i8) as u32;
_9.0 = 128615354450653835289519761725061283560_u128 << _4;
_9.1 = 10_i8 * (-99_i8);
_9.0 = _3 as u128;
Goto(bb1)
}
bb1 = {
_11 = [_4,_4,_4,_4,_4];
_10 = !_4;
_8 = _6 as i8;
_5 = _7 >> _10;
RET = 3_usize + 4578824411777336701_usize;
_3 = !598848072288468871_i64;
RET = 22441_i16 as usize;
_4 = _10;
Goto(bb2)
}
bb2 = {
_7 = _5 << _8;
_8 = !_9.1;
_6 = -649460495_i32;
_1 = !true;
_9.0 = !88832578884813523845067521218116146326_u128;
_12 = !_9.0;
RET = !6121877453934541826_usize;
_9.1 = '\u{4b07a}' as i8;
_3 = !(-7087821762999338587_i64);
_2 = _6 as u8;
_2 = 5_u8;
_13 = !_1;
_14.1 = !_8;
_1 = _13 ^ _13;
RET = !8271066294813364370_usize;
_14 = (_9.0, _8);
_12 = !_9.0;
_9 = (_14.0, _8);
_9.1 = -_14.1;
_6 = 13841005648045013869_u64 as i32;
_9.1 = -_8;
RET = 3219919072246467822_usize;
_10 = _4;
_15 = [_3,_3];
_1 = _13 & _13;
_7 = RET as u32;
Call(_6 = fn1(_1, _4, _9.1, _2, _5, _9, _14.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = _14.0 + _14.0;
_3 = _9.1 as i64;
_12 = !_9.0;
Goto(bb4)
}
bb4 = {
_4 = _10 + _10;
RET = !0_usize;
_10 = _4 | _4;
RET = 4_usize & 1437105785947385091_usize;
_3 = 308743503708610098_i64 & 4208365748927450739_i64;
_13 = _1;
_1 = _13;
_15 = [_3,_3];
_1 = _14.1 == _8;
RET = !8366938481732951953_usize;
_3 = _13 as i64;
_17.2 = (-2456_i16) | 25416_i16;
_17.3.4 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_8 = !_9.1;
_11 = [_4,_4,_10,_10,_10];
_17.2 = 28651_i16 >> _10;
_4 = _10;
_17.3.1 = [_2,_2,_2,_2];
_12 = !_9.0;
_17.2 = _1 as i16;
_9.0 = _4 as u128;
_17.5 = 12428645900244769287_u64 as f32;
_9.0 = !_12;
Goto(bb5)
}
bb5 = {
_19.fld0 = _9.0 as f64;
_3 = -(-7993621599928700707_i64);
_17.2 = 14158_i16;
_17.3.3 = !(-53_isize);
_12 = _1 as u128;
_17.4 = 62843657370107388160392337000289791779_i128;
_10 = _17.4 as u16;
_22 = _12 as u32;
_6 = _17.2 as i32;
_23 = _17.3.3 >> _4;
_17.7.0 = core::ptr::addr_of!(_25);
_25 = '\u{5dd34}';
_17.3.0 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_7 = _5 | _5;
_18 = core::ptr::addr_of_mut!(RET);
_17.6 = _25;
_2 = 159_u8;
_5 = _7 ^ _22;
_17.7.0 = core::ptr::addr_of!(_17.6);
_17.2 = -(-4069_i16);
_17.3.3 = _23 | _23;
_27.fld6 = _2 >> _5;
Goto(bb6)
}
bb6 = {
_24 = [_27.fld6,_27.fld6,_27.fld6,_2];
_14 = (_12, _8);
_27.fld1 = core::ptr::addr_of_mut!(_17.3.0);
_15 = [_3,_3];
_14.1 = 11516517899876922775_u64 as i8;
Goto(bb7)
}
bb7 = {
_4 = !_10;
_9 = _14;
RET = 2_usize;
_17.3.2 = [_17.2,_17.3.4[RET],_17.3.4[RET],_17.3.4[RET],_17.3.4[RET],_17.3.4[RET]];
_17.3.1 = [_24[RET],_24[RET],_27.fld6,_27.fld6];
Goto(bb8)
}
bb8 = {
_17.2 = _17.3.0[RET];
_17.7.1 = _7 as f32;
_15 = [_3,_3];
_21 = core::ptr::addr_of!(_25);
_27.fld2 = (*_21) as u32;
_4 = _11[RET] | _11[RET];
_19.fld0 = _6 as f64;
_17.4 = 119626982410348329709598288767129943623_i128 >> _7;
_27.fld4 = -_17.3.0[RET];
_2 = !_24[RET];
_14.1 = _17.3.0[RET] as i8;
_6 = -(-992627953_i32);
(*_18) = 5_usize;
_14.1 = _17.7.1 as i8;
Call(_17.3.3 = core::intrinsics::transmute(_23), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = !_22;
Call(_17.4 = core::intrinsics::transmute(_14.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_26 = _23;
_16 = [(*_18),(*_18),RET,(*_18),(*_18),(*_18),RET,RET];
_17.3.1 = [_2,_27.fld6,_27.fld6,_2];
(*_18) = !_16[RET];
_6 = 2104341425_i32 ^ 650490782_i32;
_18 = core::ptr::addr_of_mut!((*_18));
_18 = core::ptr::addr_of_mut!(RET);
_28 = _13 as isize;
_1 = !_13;
_17.3.4 = _17.3.2;
_20 = _15;
_13 = !_1;
_27.fld0 = [_17.3.3];
_6 = (-1734944623_i32) & (-1945248262_i32);
match _17.2 {
0 => bb6,
14158 => bb11,
_ => bb7
}
}
bb11 = {
_27.fld5 = _17.4 as usize;
_27.fld1 = core::ptr::addr_of_mut!(_17.3.4);
_17.5 = _17.7.1;
_17.6 = _25;
_17.3.2 = [_27.fld4,_27.fld4,_17.2,_17.2,_17.2,_27.fld4];
_26 = !_17.3.3;
_17.5 = _9.0 as f32;
_17.2 = _27.fld4;
_17.0 = _27.fld5 as i8;
_19.fld0 = _2 as f64;
_17.7.0 = core::ptr::addr_of!(_17.6);
_29 = [_13,_13,_1,_13,_1,_1,_1];
_27.fld6 = _1 as u8;
_5 = _7;
_17.4 = 88699205872505529571002691334906761750_i128 - 42270906971646819631617795437996529833_i128;
_17.1 = _17.2 as i128;
_8 = _14.1;
_19.fld0 = _27.fld4 as f64;
_2 = !_27.fld6;
_17.6 = (*_21);
_28 = _17.3.3;
_9.0 = !_14.0;
_32 = _26 * _23;
Goto(bb12)
}
bb12 = {
_22 = !_7;
_27.fld3 = _17.7.1 as u64;
_17.2 = _27.fld4 & _27.fld4;
_16 = [_27.fld5,RET,_27.fld5,(*_18),_27.fld5,_27.fld5,_27.fld5,(*_18)];
_27.fld6 = !_2;
_17.7.1 = _27.fld5 as f32;
_7 = _26 as u32;
_17.3.2 = _17.3.4;
_17.3.4 = [_27.fld4,_27.fld4,_27.fld4,_17.2,_27.fld4,_17.2];
_17.5 = _3 as f32;
_13 = !_1;
_30 = [_27.fld6,_2,_27.fld6,_27.fld6];
_14.1 = _27.fld3 as i8;
_14 = _9;
Goto(bb13)
}
bb13 = {
_26 = !_23;
_17.3.1 = [_2,_27.fld6,_2,_2];
_36.0 = core::ptr::addr_of!(_17.6);
_35 = [_1,_13,_13,_1,_1,_13,_1];
_9.0 = !_14.0;
_13 = !_1;
_24 = _17.3.1;
_27.fld4 = !_17.2;
_14.1 = -_8;
_17.0 = _9.1 >> _17.3.3;
_24 = _17.3.1;
_4 = _17.2 as u16;
_17.3.1 = [_2,_2,_2,_2];
(*_18) = _27.fld5 * _27.fld5;
(*_18) = !_27.fld5;
_31 = [_4,_4,_4,_4,_4];
_21 = core::ptr::addr_of!((*_21));
_25 = _17.6;
Goto(bb14)
}
bb14 = {
_17.4 = _17.1;
_24 = [_27.fld6,_27.fld6,_27.fld6,_27.fld6];
_39.2 = &_30;
_36 = (_21, _32);
_9.0 = _3 as u128;
_31 = [_4,_4,_10,_4,_4];
(*_21) = _17.6;
_33 = _15;
_14 = (_9.0, _8);
_7 = _22 ^ _22;
_2 = _27.fld6;
_17.0 = _14.1;
_8 = _23 as i8;
_27.fld3 = 18351270763523222523_u64;
_8 = _14.1 - _14.1;
_36.0 = core::ptr::addr_of!(_17.6);
_39.2 = &_24;
_8 = _19.fld0 as i8;
_17.3.3 = _26;
_35 = _29;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(0_usize, 24_usize, Move(_24), 29_usize, Move(_29), 26_usize, Move(_26), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(0_usize, 23_usize, Move(_23), 4_usize, Move(_4), 33_usize, Move(_33), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(0_usize, 20_usize, Move(_20), 9_usize, Move(_9), 25_usize, Move(_25), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(0_usize, 7_usize, Move(_7), 30_usize, Move(_30), 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: u16,mut _3: i8,mut _4: u8,mut _5: u32,mut _6: (u128, i8),mut _7: u128) -> i32 {
mir! {
type RET = i32;
let _8: &'static [u8; 4];
let _9: [usize; 8];
let _10: *mut [i16; 6];
let _11: [i16; 3];
let _12: u64;
let _13: char;
let _14: char;
let _15: (u128, i8);
let _16: Adt53;
let _17: char;
let _18: i128;
let _19: [usize; 8];
let _20: [i16; 3];
let _21: Adt48;
let _22: [usize; 8];
let _23: [bool; 7];
let _24: [i64; 2];
let _25: Adt53;
let _26: (u128, i8);
let _27: ();
let _28: ();
{
RET = (-683062487_i32);
_7 = !_6.0;
_6.1 = _1 as i8;
_7 = _6.0 * _6.0;
_1 = _6.0 >= _7;
RET = -1238574536_i32;
RET = 15756_i16 as i32;
_6 = (_7, _3);
_6 = (_7, _3);
_3 = _1 as i8;
_9 = [10855135092203746514_usize,5587049721711190528_usize,7_usize,5_usize,6_usize,6_usize,6_usize,14094257359310397133_usize];
Call(_5 = fn2(_9, RET, _9, _9, _2, _9, _9, _7, _6.1, _2, _9, _1, _3, _6, _6, _6.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _2 >= _2;
_12 = 5622881063931863429_u64 & 5363845648103976686_u64;
RET = _12 as i32;
_6.0 = _7 - _7;
RET = (-176427577_i32) | (-1949980668_i32);
_11 = [30284_i16,(-18792_i16),(-27745_i16)];
match _4 {
0 => bb2,
5 => bb4,
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
RET = '\u{43c93}' as i32;
RET = -(-1615392879_i32);
_9 = [7_usize,1204134089684936140_usize,10622194269278757125_usize,14586781500288115491_usize,0_usize,0_usize,6_usize,4_usize];
_4 = !253_u8;
_4 = 155_u8 >> _5;
_3 = _6.0 as i8;
_6.0 = _2 as u128;
_11 = [7916_i16,(-29086_i16),(-5933_i16)];
_3 = -_6.1;
RET = _3 as i32;
_6 = (_7, _3);
_7 = !_6.0;
_11 = [29509_i16,(-32306_i16),(-32131_i16)];
_5 = 1741022349_u32 - 52526203_u32;
_6.0 = _7;
Goto(bb5)
}
bb5 = {
_1 = !false;
_11 = [18201_i16,25289_i16,24531_i16];
_12 = !16263070651009657174_u64;
_14 = '\u{67204}';
_5 = 2005857657_u32;
_7 = _6.0 * _6.0;
_4 = 71_u8;
_11 = [(-8098_i16),17124_i16,18554_i16];
_11 = [10738_i16,25253_i16,10088_i16];
RET = !1690462336_i32;
_12 = _14 as u64;
_7 = _6.0;
_1 = _5 == _5;
_12 = !15862548415025102808_u64;
_15.1 = _6.1;
RET = 1163583016_i32 * 227622953_i32;
_7 = !_6.0;
_4 = 114_u8 ^ 229_u8;
_15.0 = _7;
_7 = _1 as u128;
_5 = 1243281054_u32 ^ 3693694021_u32;
_15 = (_7, _6.1);
_1 = _6.0 == _6.0;
_6.1 = _3;
_1 = _6.1 <= _6.1;
_5 = 3548087955_u32 + 2806771771_u32;
RET = !1281627065_i32;
_7 = !_15.0;
Goto(bb6)
}
bb6 = {
RET = _1 as i32;
_5 = !1918494866_u32;
_16.fld2 = [_4,_4,_4,_4];
_3 = !_15.1;
_13 = _14;
_16.fld0 = !12649_i16;
RET = (-390990572_i32);
_6.1 = _7 as i8;
_3 = _6.1 + _15.1;
_6.1 = _3;
_16.fld1 = !(-6653009532204859832_i64);
_17 = _14;
_13 = _17;
_15.0 = _7 << _3;
match RET {
340282366920938463463374607431377220884 => bb8,
_ => bb7
}
}
bb7 = {
_1 = !false;
_11 = [18201_i16,25289_i16,24531_i16];
_12 = !16263070651009657174_u64;
_14 = '\u{67204}';
_5 = 2005857657_u32;
_7 = _6.0 * _6.0;
_4 = 71_u8;
_11 = [(-8098_i16),17124_i16,18554_i16];
_11 = [10738_i16,25253_i16,10088_i16];
RET = !1690462336_i32;
_12 = _14 as u64;
_7 = _6.0;
_1 = _5 == _5;
_12 = !15862548415025102808_u64;
_15.1 = _6.1;
RET = 1163583016_i32 * 227622953_i32;
_7 = !_6.0;
_4 = 114_u8 ^ 229_u8;
_15.0 = _7;
_7 = _1 as u128;
_5 = 1243281054_u32 ^ 3693694021_u32;
_15 = (_7, _6.1);
_1 = _6.0 == _6.0;
_6.1 = _3;
_1 = _6.1 <= _6.1;
_5 = 3548087955_u32 + 2806771771_u32;
RET = !1281627065_i32;
_7 = !_15.0;
Goto(bb6)
}
bb8 = {
_8 = &_16.fld2;
_12 = !5121182341575099480_u64;
_16.fld1 = 3797114483971166700_i64;
_18 = 121725290477378045662329185779754284570_i128;
_4 = !96_u8;
_16.fld1 = (-3726652648417991708_i64) + 8401478202609847920_i64;
_15 = _6;
_1 = false;
_13 = _14;
_2 = 20353_u16 - 45621_u16;
RET = -1387609966_i32;
_3 = _15.1;
_16.fld1 = (-4378349435540123330_i64);
_15 = _6;
_9 = [2707251862374079590_usize,4_usize,18364700908026386325_usize,584616706045269528_usize,14860176384051617843_usize,16316231665570365159_usize,11748156934822133060_usize,5_usize];
_18 = (-74344012131695360055818522179672675247_i128) - (-109605986625335546381629642197919971341_i128);
RET = 1725436534_i32;
_17 = _14;
_6.1 = _15.1;
_14 = _13;
_18 = _14 as i128;
RET = 1563552188_i32;
_11 = [_16.fld0,_16.fld0,_16.fld0];
_2 = !45427_u16;
_1 = !false;
_19 = _9;
_11 = [_16.fld0,_16.fld0,_16.fld0];
Goto(bb9)
}
bb9 = {
_6.1 = _3;
_15.0 = !_6.0;
RET = _16.fld1 as i32;
_6.0 = _15.0 >> _7;
_6.0 = !_7;
_19 = _9;
_7 = _2 as u128;
_6.1 = !_3;
_15 = (_7, _3);
_5 = !492959802_u32;
_3 = _15.1 + _6.1;
_16.fld1 = 2271349010731647158_i64;
_20 = _11;
_22 = _19;
_15.1 = _6.1 - _3;
_14 = _13;
_16.fld2 = [_4,_4,_4,_4];
_16.fld1 = (-1512532658196184813_i64);
_23 = [_1,_1,_1,_1,_1,_1,_1];
_2 = 56894_u16;
_20 = [_16.fld0,_16.fld0,_16.fld0];
match _16.fld1 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
340282366920938463461862074773572026643 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_15.0 = _4 as u128;
_9 = _19;
_23 = [_1,_1,_1,_1,_1,_1,_1];
_14 = _17;
match _16.fld1 {
0 => bb7,
1 => bb2,
2 => bb12,
3 => bb13,
340282366920938463461862074773572026643 => bb15,
_ => bb14
}
}
bb12 = {
RET = _1 as i32;
_5 = !1918494866_u32;
_16.fld2 = [_4,_4,_4,_4];
_3 = !_15.1;
_13 = _14;
_16.fld0 = !12649_i16;
RET = (-390990572_i32);
_6.1 = _7 as i8;
_3 = _6.1 + _15.1;
_6.1 = _3;
_16.fld1 = !(-6653009532204859832_i64);
_17 = _14;
_13 = _17;
_15.0 = _7 << _3;
match RET {
340282366920938463463374607431377220884 => bb8,
_ => bb7
}
}
bb13 = {
_1 = !false;
_11 = [18201_i16,25289_i16,24531_i16];
_12 = !16263070651009657174_u64;
_14 = '\u{67204}';
_5 = 2005857657_u32;
_7 = _6.0 * _6.0;
_4 = 71_u8;
_11 = [(-8098_i16),17124_i16,18554_i16];
_11 = [10738_i16,25253_i16,10088_i16];
RET = !1690462336_i32;
_12 = _14 as u64;
_7 = _6.0;
_1 = _5 == _5;
_12 = !15862548415025102808_u64;
_15.1 = _6.1;
RET = 1163583016_i32 * 227622953_i32;
_7 = !_6.0;
_4 = 114_u8 ^ 229_u8;
_15.0 = _7;
_7 = _1 as u128;
_5 = 1243281054_u32 ^ 3693694021_u32;
_15 = (_7, _6.1);
_1 = _6.0 == _6.0;
_6.1 = _3;
_1 = _6.1 <= _6.1;
_5 = 3548087955_u32 + 2806771771_u32;
RET = !1281627065_i32;
_7 = !_15.0;
Goto(bb6)
}
bb14 = {
Return()
}
bb15 = {
RET = _16.fld0 as i32;
_14 = _13;
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(1_usize, 11_usize, Move(_11), 9_usize, Move(_9), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(1_usize, 15_usize, Move(_15), 3_usize, Move(_3), 19_usize, Move(_19), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(1_usize, 4_usize, Move(_4), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [usize; 8],mut _2: i32,mut _3: [usize; 8],mut _4: [usize; 8],mut _5: u16,mut _6: [usize; 8],mut _7: [usize; 8],mut _8: u128,mut _9: i8,mut _10: u16,mut _11: [usize; 8],mut _12: bool,mut _13: i8,mut _14: (u128, i8),mut _15: (u128, i8),mut _16: i8) -> u32 {
mir! {
type RET = u32;
let _17: [u128; 1];
let _18: Adt58;
let _19: u8;
let _20: Adt52;
let _21: [isize; 1];
let _22: Adt45;
let _23: u64;
let _24: u128;
let _25: isize;
let _26: Adt49;
let _27: Adt48;
let _28: Adt46;
let _29: *mut usize;
let _30: Adt43;
let _31: isize;
let _32: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32));
let _33: [i16; 3];
let _34: Adt52;
let _35: [i16; 3];
let _36: Adt59;
let _37: [u16; 5];
let _38: usize;
let _39: Adt48;
let _40: f32;
let _41: (u128, i8);
let _42: ();
let _43: ();
{
_10 = 9223372036854775807_isize as u16;
_15.0 = 3605049205_u32 as u128;
_2 = !125897477_i32;
_8 = !_14.0;
_17 = [_15.0];
_1 = [0_usize,7_usize,12505942868359966140_usize,5500709493775859812_usize,2_usize,12913479518981500598_usize,845365705233520243_usize,3_usize];
_13 = 81_isize as i8;
RET = 251009883_u32;
_17 = [_14.0];
_13 = -_14.1;
_14.0 = RET as u128;
_10 = _5;
_10 = _5;
_13 = _16;
_3 = _6;
Call(_10 = fn3(_1, _6, _3, _7, _17, _5, _15.1, _4, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !_5;
_5 = !_10;
_14 = _15;
_2 = 872004661_i32 & 1927436229_i32;
Goto(bb2)
}
bb2 = {
_14.0 = '\u{74409}' as u128;
_19 = !204_u8;
RET = 2722627199_u32;
_1 = [7954166646805236487_usize,5927690633861757712_usize,13442495728446031551_usize,10461201848437849845_usize,5_usize,7900716701940093333_usize,5_usize,4039213195221295368_usize];
RET = !3277637816_u32;
_6 = _3;
_9 = _12 as i8;
_15 = _14;
_14.1 = _13 | _9;
_15 = _14;
_2 = 38577691_i32;
_3 = _4;
_14 = _15;
_10 = _5;
_8 = (-7489121795957184864_i64) as u128;
_17 = [_15.0];
_21 = [(-9223372036854775808_isize)];
_9 = RET as i8;
_8 = !_15.0;
_14.1 = _15.1;
Goto(bb3)
}
bb3 = {
_23 = 13887202139308815316_u64;
_14.1 = -_16;
_23 = !7308001982433762823_u64;
RET = !3099073162_u32;
_7 = _11;
_14 = (_15.0, _13);
_21 = [43_isize];
_15.1 = _23 as i8;
RET = 645_i16 as u32;
_1 = [7_usize,2_usize,7876752988557973509_usize,7_usize,4820134058563119532_usize,4_usize,1_usize,18027030826830846049_usize];
_13 = _16 << _19;
_24 = !_15.0;
_15.0 = _8;
_12 = false;
_17 = [_15.0];
_1 = [5_usize,12137038123605512348_usize,16879547025545338276_usize,2079888962335930398_usize,3785859904973199845_usize,10705233667380630669_usize,9251697108123440819_usize,2_usize];
Goto(bb4)
}
bb4 = {
_1 = _4;
_11 = [6_usize,12471144707925377180_usize,1_usize,6_usize,12725458621096790711_usize,2_usize,4_usize,12283752120704667764_usize];
_25 = '\u{b7e75}' as isize;
_2 = _12 as i32;
_15.1 = -_16;
_10 = _5 * _5;
_3 = _1;
_12 = true | false;
_3 = [5_usize,16087086400284161693_usize,5_usize,5_usize,3_usize,0_usize,4_usize,8686628573478520849_usize];
_24 = _15.0;
_5 = 40425772554235267212585135059263573523_i128 as u16;
_11 = _6;
_13 = _14.1;
_14 = (_15.0, _16);
_16 = -_13;
_25 = RET as isize;
_6 = _11;
_15.1 = _16 ^ _14.1;
Goto(bb5)
}
bb5 = {
_15 = (_8, _13);
_10 = _5 ^ _5;
_26.fld0 = 0_usize as f64;
_26.fld0 = (-1026282336035664735_i64) as f64;
_26.fld0 = _19 as f64;
_9 = (-85527952286107783601812772582654018928_i128) as i8;
RET = 15513067_u32;
_8 = !_24;
_10 = _2 as u16;
_8 = _14.0;
_15 = (_8, _16);
_2 = 1270550884_i32;
_16 = '\u{44adc}' as i8;
Goto(bb6)
}
bb6 = {
_30.fld6 = _19 | _19;
_1 = _3;
_13 = -_14.1;
_3 = [7_usize,10916362610474267243_usize,7126635788330649487_usize,5410561436202012882_usize,1370377297125799558_usize,16867629012932478181_usize,1396596236759621457_usize,3642525447245734191_usize];
_12 = !true;
_22 = Adt45::Variant2 { fld0: _21,fld1: _10,fld2: _14 };
_6 = [11338520825447496754_usize,10690459623522657011_usize,2592214810708414937_usize,0_usize,14728869277704370076_usize,9929894571733217343_usize,11198829920146842037_usize,2610780188657688760_usize];
_24 = !Field::<(u128, i8)>(Variant(_22, 2), 2).0;
_30.fld2 = _23 as u32;
place!(Field::<(u128, i8)>(Variant(_22, 2), 2)).1 = -_9;
_15.1 = _13;
_9 = _19 as i8;
_3 = _4;
_30.fld0 = _21;
_15 = (_8, _14.1);
_27 = Adt48::Variant1 { fld0: _17 };
_30.fld1 = core::ptr::addr_of_mut!(_32.3.4);
_10 = _5 + Field::<u16>(Variant(_22, 2), 1);
_1 = _6;
_32.0 = _15.1 | _14.1;
_30.fld4 = _10 as i16;
_8 = Field::<(u128, i8)>(Variant(_22, 2), 2).0 ^ Field::<(u128, i8)>(Variant(_22, 2), 2).0;
_32.3.3 = _25 & _25;
_32.1 = -103427976983876871335051799586447975430_i128;
SetDiscriminant(_27, 1);
_32.6 = '\u{d9b92}';
_32.2 = -_30.fld4;
Goto(bb7)
}
bb7 = {
_32.7.1 = _23 as f32;
_32.3.0 = [_30.fld4,_30.fld4,_32.2,_32.2,_32.2,_30.fld4];
_14 = (_15.0, _15.1);
_32.3.4 = [_30.fld4,_30.fld4,_32.2,_30.fld4,_32.2,_32.2];
_30.fld3 = _23 + _23;
_23 = _30.fld3;
_4 = [15293274569060432277_usize,4_usize,14221140127804862589_usize,5_usize,6292023255085758745_usize,1696702981369900379_usize,6_usize,4721063330182701359_usize];
_13 = _2 as i8;
_32.3.3 = _25 | _25;
_32.3.0 = _32.3.4;
Goto(bb8)
}
bb8 = {
_16 = _32.0;
_5 = !Field::<u16>(Variant(_22, 2), 1);
Goto(bb9)
}
bb9 = {
_32.3.3 = _25;
_32.5 = _32.7.1;
_33 = [_30.fld4,_30.fld4,_30.fld4];
_27 = Adt48::Variant1 { fld0: _17 };
_22 = Adt45::Variant2 { fld0: _21,fld1: _10,fld2: _15 };
_15.1 = Field::<(u128, i8)>(Variant(_22, 2), 2).1;
SetDiscriminant(_27, 0);
_32.5 = _32.7.1;
_19 = _30.fld6 | _30.fld6;
_17 = [Field::<(u128, i8)>(Variant(_22, 2), 2).0];
_7 = _11;
_21 = [_32.3.3];
_27 = Adt48::Variant0 { fld0: _30.fld4 };
_32.3.3 = _30.fld3 as isize;
_26.fld0 = _30.fld6 as f64;
_30.fld2 = 1689754300308423871_usize as u32;
_27 = Adt48::Variant0 { fld0: _30.fld4 };
match RET {
0 => bb10,
1 => bb11,
15513067 => bb13,
_ => bb12
}
}
bb10 = {
_1 = _4;
_11 = [6_usize,12471144707925377180_usize,1_usize,6_usize,12725458621096790711_usize,2_usize,4_usize,12283752120704667764_usize];
_25 = '\u{b7e75}' as isize;
_2 = _12 as i32;
_15.1 = -_16;
_10 = _5 * _5;
_3 = _1;
_12 = true | false;
_3 = [5_usize,16087086400284161693_usize,5_usize,5_usize,3_usize,0_usize,4_usize,8686628573478520849_usize];
_24 = _15.0;
_5 = 40425772554235267212585135059263573523_i128 as u16;
_11 = _6;
_13 = _14.1;
_14 = (_15.0, _16);
_16 = -_13;
_25 = RET as isize;
_6 = _11;
_15.1 = _16 ^ _14.1;
Goto(bb5)
}
bb11 = {
_14.0 = '\u{74409}' as u128;
_19 = !204_u8;
RET = 2722627199_u32;
_1 = [7954166646805236487_usize,5927690633861757712_usize,13442495728446031551_usize,10461201848437849845_usize,5_usize,7900716701940093333_usize,5_usize,4039213195221295368_usize];
RET = !3277637816_u32;
_6 = _3;
_9 = _12 as i8;
_15 = _14;
_14.1 = _13 | _9;
_15 = _14;
_2 = 38577691_i32;
_3 = _4;
_14 = _15;
_10 = _5;
_8 = (-7489121795957184864_i64) as u128;
_17 = [_15.0];
_21 = [(-9223372036854775808_isize)];
_9 = RET as i8;
_8 = !_15.0;
_14.1 = _15.1;
Goto(bb3)
}
bb12 = {
_30.fld6 = _19 | _19;
_1 = _3;
_13 = -_14.1;
_3 = [7_usize,10916362610474267243_usize,7126635788330649487_usize,5410561436202012882_usize,1370377297125799558_usize,16867629012932478181_usize,1396596236759621457_usize,3642525447245734191_usize];
_12 = !true;
_22 = Adt45::Variant2 { fld0: _21,fld1: _10,fld2: _14 };
_6 = [11338520825447496754_usize,10690459623522657011_usize,2592214810708414937_usize,0_usize,14728869277704370076_usize,9929894571733217343_usize,11198829920146842037_usize,2610780188657688760_usize];
_24 = !Field::<(u128, i8)>(Variant(_22, 2), 2).0;
_30.fld2 = _23 as u32;
place!(Field::<(u128, i8)>(Variant(_22, 2), 2)).1 = -_9;
_15.1 = _13;
_9 = _19 as i8;
_3 = _4;
_30.fld0 = _21;
_15 = (_8, _14.1);
_27 = Adt48::Variant1 { fld0: _17 };
_30.fld1 = core::ptr::addr_of_mut!(_32.3.4);
_10 = _5 + Field::<u16>(Variant(_22, 2), 1);
_1 = _6;
_32.0 = _15.1 | _14.1;
_30.fld4 = _10 as i16;
_8 = Field::<(u128, i8)>(Variant(_22, 2), 2).0 ^ Field::<(u128, i8)>(Variant(_22, 2), 2).0;
_32.3.3 = _25 & _25;
_32.1 = -103427976983876871335051799586447975430_i128;
SetDiscriminant(_27, 1);
_32.6 = '\u{d9b92}';
_32.2 = -_30.fld4;
Goto(bb7)
}
bb13 = {
place!(Field::<i16>(Variant(_27, 0), 0)) = _32.2 + _32.2;
_30.fld0 = [_25];
_15.1 = -_32.0;
_7 = _11;
_33 = [Field::<i16>(Variant(_27, 0), 0),Field::<i16>(Variant(_27, 0), 0),_30.fld4];
place!(Field::<i16>(Variant(_27, 0), 0)) = _32.6 as i16;
_32.6 = '\u{4a0b1}';
_5 = _26.fld0 as u16;
_15.1 = Field::<(u128, i8)>(Variant(_22, 2), 2).1 << Field::<(u128, i8)>(Variant(_22, 2), 2).0;
_1 = [4789120985176933274_usize,2_usize,5411060717563947220_usize,12152327162398717001_usize,9957143688226141988_usize,0_usize,3_usize,6_usize];
Goto(bb14)
}
bb14 = {
_30.fld6 = _19;
_11 = [0_usize,9814957290855755923_usize,6_usize,7136448445002302820_usize,0_usize,4_usize,4092357717913037990_usize,1471353756065869698_usize];
_30.fld1 = core::ptr::addr_of_mut!(_32.3.4);
SetDiscriminant(_27, 1);
_35 = [_32.2,_32.2,_32.2];
_30.fld5 = (-136269295944502022_i64) as usize;
_14 = Field::<(u128, i8)>(Variant(_22, 2), 2);
place!(Field::<[u128; 1]>(Variant(_27, 1), 0)) = [_15.0];
_38 = _30.fld5;
_17 = [_8];
_38 = _32.2 as usize;
place!(Field::<u16>(Variant(_22, 2), 1)) = _5;
_24 = _32.6 as u128;
_40 = _32.7.1;
_32.3.0 = _32.3.4;
SetDiscriminant(_22, 1);
SetDiscriminant(_27, 0);
_32.4 = _32.1 + _32.1;
_32.3.4 = _32.3.0;
_32.3.3 = -_25;
_30.fld3 = !_23;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(2_usize, 8_usize, Move(_8), 10_usize, Move(_10), 21_usize, Move(_21), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(2_usize, 25_usize, Move(_25), 9_usize, Move(_9), 12_usize, Move(_12), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(2_usize, 16_usize, Move(_16), 2_usize, Move(_2), 17_usize, Move(_17), 38_usize, Move(_38)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [usize; 8],mut _2: [usize; 8],mut _3: [usize; 8],mut _4: [usize; 8],mut _5: [u128; 1],mut _6: u16,mut _7: i8,mut _8: [usize; 8],mut _9: [usize; 8]) -> u16 {
mir! {
type RET = u16;
let _10: char;
let _11: [i16; 3];
let _12: char;
let _13: Adt49;
let _14: [u8; 7];
let _15: isize;
let _16: [i16; 3];
let _17: [bool; 7];
let _18: &'static [u8; 4];
let _19: *mut [i16; 6];
let _20: u64;
let _21: [u8; 4];
let _22: Adt45;
let _23: Adt50;
let _24: [u128; 1];
let _25: (*const char, isize);
let _26: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]);
let _27: f64;
let _28: f64;
let _29: [u8; 7];
let _30: [i16; 2];
let _31: [u16; 5];
let _32: *const char;
let _33: Adt48;
let _34: [isize; 1];
let _35: isize;
let _36: f32;
let _37: (*const char, f32);
let _38: ();
let _39: ();
{
RET = _6 >> _7;
_2 = _9;
RET = _6;
_6 = RET;
_3 = [7834676657003689029_usize,10705090871446569718_usize,13106233157978820981_usize,1_usize,6_usize,7_usize,3_usize,1_usize];
_10 = '\u{18505}';
_4 = [5_usize,12487200973753744793_usize,5946086365647263301_usize,6_usize,10194020630008036538_usize,3_usize,6205454528855868555_usize,1060066263853142508_usize];
Goto(bb1)
}
bb1 = {
_4 = _3;
_5 = [207057774586280437799260582009478124375_u128];
_5 = [55971197034261000913302765708026018603_u128];
_3 = _1;
_4 = [5468393732406382347_usize,1_usize,10323514887760051984_usize,15833627936162879891_usize,0_usize,4154415564532152093_usize,4_usize,11598339103566298093_usize];
_4 = _8;
RET = 4183932614303682651_usize as u16;
_9 = _3;
_4 = [5_usize,2_usize,4_usize,1_usize,17024387620747697821_usize,5296760438883564433_usize,12059595676802976722_usize,6_usize];
_2 = [7_usize,4_usize,1_usize,17208987418185633134_usize,9156352449310286135_usize,2_usize,1_usize,7_usize];
_9 = _2;
_8 = [7490043978253860470_usize,1_usize,7556944713317097534_usize,9460280486305386565_usize,14616849458723910582_usize,5_usize,7_usize,7977697191420904258_usize];
_9 = [11313002839467264670_usize,6_usize,4_usize,6_usize,5582241956134324465_usize,5293539105780202149_usize,4543451884461665552_usize,1_usize];
_8 = [5_usize,0_usize,438579034870213273_usize,16348569131306646515_usize,3_usize,17551059350693453253_usize,5_usize,14960098679693962930_usize];
_5 = [224440218006717381720206283183352665099_u128];
_10 = '\u{6eff3}';
Call(_4 = fn4(_2, _3, _2, _3, _1, _8, _8, _8, RET, _3, _3, _8, _3, RET, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [3_usize,17217220623712798795_usize,17563690971616876081_usize,17049014417732501852_usize,6142665233815856003_usize,16699245320175591987_usize,15017246098228352712_usize,18102615388313331116_usize];
_1 = _8;
_11 = [19394_i16,14706_i16,(-6081_i16)];
_5 = [216870701491009797324377959618261127528_u128];
_7 = (-20_i8);
_12 = _10;
_2 = [5_usize,0_usize,15841029828979341931_usize,12740675805529751312_usize,7_usize,3_usize,2_usize,2_usize];
_12 = _10;
_7 = 63_i8;
_10 = _12;
_2 = [3_usize,0_usize,3400680028742303391_usize,11367023562462205962_usize,14667980389347683367_usize,7_usize,2_usize,2_usize];
_13.fld0 = 9223372036854775807_isize as f64;
_11 = [6128_i16,18901_i16,19280_i16];
_5 = [258152644231698919804589817805278895078_u128];
_12 = _10;
_13.fld0 = 11781832715615129480_u64 as f64;
_12 = _10;
_6 = !RET;
_11 = [6666_i16,(-32467_i16),28686_i16];
_7 = -(-42_i8);
_2 = [6_usize,7_usize,12848385243391750122_usize,5_usize,8184312321194466614_usize,3_usize,2_usize,2_usize];
_5 = [128514537548527583157766892844906311166_u128];
Goto(bb3)
}
bb3 = {
_3 = _2;
_4 = [3850939418431790517_usize,12838608327746167950_usize,1150955703633113587_usize,2772464849845867423_usize,2_usize,10023419108185075473_usize,0_usize,12121013341515348495_usize];
_13.fld0 = (-1837112883_i32) as f64;
_9 = _4;
_16 = [(-29042_i16),28054_i16,(-6847_i16)];
_5 = [283552904907201495288479720163159609161_u128];
_10 = _12;
_13.fld0 = (-1569864763_i32) as f64;
_8 = _9;
_9 = [110102953461694193_usize,10149193171067256968_usize,3165129878247328128_usize,3258955980953901345_usize,2829484050870331524_usize,6_usize,3_usize,14930933221778900253_usize];
_17 = [false,false,false,false,false,true,true];
_1 = [4_usize,5_usize,6_usize,6640016650196962542_usize,3_usize,4_usize,12058937323553701170_usize,6816034411428442033_usize];
_11 = [14263_i16,32190_i16,16032_i16];
_7 = (-67_i8);
_2 = [17171330974234228843_usize,805738822275814839_usize,14509426114070106738_usize,0_usize,13247036388452568699_usize,14944816606415019588_usize,7254427410269754360_usize,4_usize];
_4 = [14778926277646902404_usize,4390919313823803425_usize,2_usize,0_usize,3177609855393667306_usize,16995874090665492343_usize,11741710135663542628_usize,4513907534947646318_usize];
_7 = _6 as i8;
_6 = 3196436606_u32 as u16;
_15 = (-60_isize);
_10 = _12;
_12 = _10;
_3 = [7_usize,17187221744432013870_usize,1_usize,4_usize,5_usize,5_usize,3_usize,18234288782030218826_usize];
_10 = _12;
RET = _6 * _6;
_11 = _16;
_7 = 123_i8;
match _7 {
0 => bb4,
123 => bb6,
_ => bb5
}
}
bb4 = {
_8 = [3_usize,17217220623712798795_usize,17563690971616876081_usize,17049014417732501852_usize,6142665233815856003_usize,16699245320175591987_usize,15017246098228352712_usize,18102615388313331116_usize];
_1 = _8;
_11 = [19394_i16,14706_i16,(-6081_i16)];
_5 = [216870701491009797324377959618261127528_u128];
_7 = (-20_i8);
_12 = _10;
_2 = [5_usize,0_usize,15841029828979341931_usize,12740675805529751312_usize,7_usize,3_usize,2_usize,2_usize];
_12 = _10;
_7 = 63_i8;
_10 = _12;
_2 = [3_usize,0_usize,3400680028742303391_usize,11367023562462205962_usize,14667980389347683367_usize,7_usize,2_usize,2_usize];
_13.fld0 = 9223372036854775807_isize as f64;
_11 = [6128_i16,18901_i16,19280_i16];
_5 = [258152644231698919804589817805278895078_u128];
_12 = _10;
_13.fld0 = 11781832715615129480_u64 as f64;
_12 = _10;
_6 = !RET;
_11 = [6666_i16,(-32467_i16),28686_i16];
_7 = -(-42_i8);
_2 = [6_usize,7_usize,12848385243391750122_usize,5_usize,8184312321194466614_usize,3_usize,2_usize,2_usize];
_5 = [128514537548527583157766892844906311166_u128];
Goto(bb3)
}
bb5 = {
_4 = _3;
_5 = [207057774586280437799260582009478124375_u128];
_5 = [55971197034261000913302765708026018603_u128];
_3 = _1;
_4 = [5468393732406382347_usize,1_usize,10323514887760051984_usize,15833627936162879891_usize,0_usize,4154415564532152093_usize,4_usize,11598339103566298093_usize];
_4 = _8;
RET = 4183932614303682651_usize as u16;
_9 = _3;
_4 = [5_usize,2_usize,4_usize,1_usize,17024387620747697821_usize,5296760438883564433_usize,12059595676802976722_usize,6_usize];
_2 = [7_usize,4_usize,1_usize,17208987418185633134_usize,9156352449310286135_usize,2_usize,1_usize,7_usize];
_9 = _2;
_8 = [7490043978253860470_usize,1_usize,7556944713317097534_usize,9460280486305386565_usize,14616849458723910582_usize,5_usize,7_usize,7977697191420904258_usize];
_9 = [11313002839467264670_usize,6_usize,4_usize,6_usize,5582241956134324465_usize,5293539105780202149_usize,4543451884461665552_usize,1_usize];
_8 = [5_usize,0_usize,438579034870213273_usize,16348569131306646515_usize,3_usize,17551059350693453253_usize,5_usize,14960098679693962930_usize];
_5 = [224440218006717381720206283183352665099_u128];
_10 = '\u{6eff3}';
Call(_4 = fn4(_2, _3, _2, _3, _1, _8, _8, _8, RET, _3, _3, _8, _3, RET, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_17 = [false,true,true,true,false,true,false];
_17 = [false,true,true,true,true,true,true];
_20 = !252872019871106289_u64;
_9 = _3;
_7 = true as i8;
_7 = 1_i8;
_7 = (-73_i8);
_14 = [152_u8,91_u8,84_u8,151_u8,64_u8,156_u8,242_u8];
RET = _6 - _6;
_18 = &_21;
_14 = [89_u8,116_u8,43_u8,187_u8,184_u8,106_u8,55_u8];
_5 = [93986928997690342347070808268914052537_u128];
_5 = [233487344186865717321913868826130333386_u128];
RET = !_6;
_6 = RET ^ RET;
_24 = [167152188969845450901996081563664790939_u128];
_1 = [2_usize,0_usize,2574591432448376175_usize,0_usize,2_usize,1233354514513722407_usize,2324290953128588019_usize,13534096208322957828_usize];
Goto(bb7)
}
bb7 = {
_13.fld0 = _7 as f64;
_17 = [true,true,true,false,false,true,false];
_12 = _10;
_25.0 = core::ptr::addr_of!(_12);
_26.4 = [24104_i16,5321_i16,(-10499_i16),(-27934_i16),(-1658_i16),10221_i16];
_26.1 = [220_u8,249_u8,90_u8,26_u8];
_6 = 155192465710894028477633318951995615052_u128 as u16;
_25.1 = !_15;
_21 = [75_u8,244_u8,172_u8,245_u8];
_25.0 = core::ptr::addr_of!(_10);
_20 = !13378444033125603604_u64;
_26.0 = [(-26967_i16),6855_i16,13444_i16,(-9638_i16),314_i16,(-8546_i16)];
_18 = &_21;
Goto(bb8)
}
bb8 = {
_16 = [(-20501_i16),15997_i16,2054_i16];
_28 = -_13.fld0;
_26.2 = [(-18950_i16),21694_i16,24966_i16,(-6080_i16),(-21207_i16),795_i16];
_4 = [3639686172284453008_usize,10678589152025475891_usize,8332588976159728658_usize,3850621238213121502_usize,8728873205251761983_usize,4_usize,1_usize,6_usize];
_26.2 = _26.4;
_20 = !2700400983419791688_u64;
RET = _6;
_27 = 52265255_i32 as f64;
_28 = -_13.fld0;
_7 = 104_i8 * (-39_i8);
_13.fld0 = _7 as f64;
_12 = _10;
_9 = [3_usize,10318491205392898267_usize,5_usize,3_usize,3759934601729996814_usize,1_usize,18254607235639121501_usize,2_usize];
_29 = [140_u8,208_u8,114_u8,154_u8,94_u8,32_u8,81_u8];
_26.2 = [(-4490_i16),7080_i16,(-31816_i16),27438_i16,11190_i16,(-2501_i16)];
_13 = Adt49 { fld0: _28 };
_18 = &(*_18);
_19 = core::ptr::addr_of_mut!(_26.4);
_5 = [301626567365930391892431094411544301064_u128];
_26.2 = [(-27215_i16),11131_i16,19824_i16,14356_i16,(-25909_i16),13056_i16];
_12 = _10;
_12 = _10;
_30 = [(-11108_i16),(-11375_i16)];
_18 = &_26.1;
_13 = Adt49 { fld0: _28 };
_31 = [_6,RET,_6,RET,_6];
match _15 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb4,
4 => bb9,
5 => bb10,
340282366920938463463374607431768211396 => bb12,
_ => bb11
}
}
bb9 = {
_3 = _2;
_4 = [3850939418431790517_usize,12838608327746167950_usize,1150955703633113587_usize,2772464849845867423_usize,2_usize,10023419108185075473_usize,0_usize,12121013341515348495_usize];
_13.fld0 = (-1837112883_i32) as f64;
_9 = _4;
_16 = [(-29042_i16),28054_i16,(-6847_i16)];
_5 = [283552904907201495288479720163159609161_u128];
_10 = _12;
_13.fld0 = (-1569864763_i32) as f64;
_8 = _9;
_9 = [110102953461694193_usize,10149193171067256968_usize,3165129878247328128_usize,3258955980953901345_usize,2829484050870331524_usize,6_usize,3_usize,14930933221778900253_usize];
_17 = [false,false,false,false,false,true,true];
_1 = [4_usize,5_usize,6_usize,6640016650196962542_usize,3_usize,4_usize,12058937323553701170_usize,6816034411428442033_usize];
_11 = [14263_i16,32190_i16,16032_i16];
_7 = (-67_i8);
_2 = [17171330974234228843_usize,805738822275814839_usize,14509426114070106738_usize,0_usize,13247036388452568699_usize,14944816606415019588_usize,7254427410269754360_usize,4_usize];
_4 = [14778926277646902404_usize,4390919313823803425_usize,2_usize,0_usize,3177609855393667306_usize,16995874090665492343_usize,11741710135663542628_usize,4513907534947646318_usize];
_7 = _6 as i8;
_6 = 3196436606_u32 as u16;
_15 = (-60_isize);
_10 = _12;
_12 = _10;
_3 = [7_usize,17187221744432013870_usize,1_usize,4_usize,5_usize,5_usize,3_usize,18234288782030218826_usize];
_10 = _12;
RET = _6 * _6;
_11 = _16;
_7 = 123_i8;
match _7 {
0 => bb4,
123 => bb6,
_ => bb5
}
}
bb10 = {
_8 = [3_usize,17217220623712798795_usize,17563690971616876081_usize,17049014417732501852_usize,6142665233815856003_usize,16699245320175591987_usize,15017246098228352712_usize,18102615388313331116_usize];
_1 = _8;
_11 = [19394_i16,14706_i16,(-6081_i16)];
_5 = [216870701491009797324377959618261127528_u128];
_7 = (-20_i8);
_12 = _10;
_2 = [5_usize,0_usize,15841029828979341931_usize,12740675805529751312_usize,7_usize,3_usize,2_usize,2_usize];
_12 = _10;
_7 = 63_i8;
_10 = _12;
_2 = [3_usize,0_usize,3400680028742303391_usize,11367023562462205962_usize,14667980389347683367_usize,7_usize,2_usize,2_usize];
_13.fld0 = 9223372036854775807_isize as f64;
_11 = [6128_i16,18901_i16,19280_i16];
_5 = [258152644231698919804589817805278895078_u128];
_12 = _10;
_13.fld0 = 11781832715615129480_u64 as f64;
_12 = _10;
_6 = !RET;
_11 = [6666_i16,(-32467_i16),28686_i16];
_7 = -(-42_i8);
_2 = [6_usize,7_usize,12848385243391750122_usize,5_usize,8184312321194466614_usize,3_usize,2_usize,2_usize];
_5 = [128514537548527583157766892844906311166_u128];
Goto(bb3)
}
bb11 = {
_8 = [3_usize,17217220623712798795_usize,17563690971616876081_usize,17049014417732501852_usize,6142665233815856003_usize,16699245320175591987_usize,15017246098228352712_usize,18102615388313331116_usize];
_1 = _8;
_11 = [19394_i16,14706_i16,(-6081_i16)];
_5 = [216870701491009797324377959618261127528_u128];
_7 = (-20_i8);
_12 = _10;
_2 = [5_usize,0_usize,15841029828979341931_usize,12740675805529751312_usize,7_usize,3_usize,2_usize,2_usize];
_12 = _10;
_7 = 63_i8;
_10 = _12;
_2 = [3_usize,0_usize,3400680028742303391_usize,11367023562462205962_usize,14667980389347683367_usize,7_usize,2_usize,2_usize];
_13.fld0 = 9223372036854775807_isize as f64;
_11 = [6128_i16,18901_i16,19280_i16];
_5 = [258152644231698919804589817805278895078_u128];
_12 = _10;
_13.fld0 = 11781832715615129480_u64 as f64;
_12 = _10;
_6 = !RET;
_11 = [6666_i16,(-32467_i16),28686_i16];
_7 = -(-42_i8);
_2 = [6_usize,7_usize,12848385243391750122_usize,5_usize,8184312321194466614_usize,3_usize,2_usize,2_usize];
_5 = [128514537548527583157766892844906311166_u128];
Goto(bb3)
}
bb12 = {
_31 = [RET,_6,_6,RET,_6];
_13.fld0 = -_28;
_19 = core::ptr::addr_of_mut!(_26.2);
_26.0 = _26.4;
_19 = core::ptr::addr_of_mut!(_26.2);
Goto(bb13)
}
bb13 = {
(*_19) = [(-11324_i16),22201_i16,18443_i16,23001_i16,29817_i16,6976_i16];
_13.fld0 = -_28;
_26.3 = _15 * _15;
_25.0 = core::ptr::addr_of!(_12);
_34 = [_26.3];
_4 = _1;
_35 = _25.1;
_25.0 = core::ptr::addr_of!(_12);
_36 = _27 as f32;
_26.2 = _26.0;
_18 = &_26.1;
_6 = RET & RET;
_36 = _26.3 as f32;
_34 = [_26.3];
_2 = _8;
_29 = [219_u8,93_u8,8_u8,165_u8,47_u8,185_u8,202_u8];
_37.0 = _25.0;
Goto(bb14)
}
bb14 = {
_8 = [6_usize,5_usize,6563751342828300792_usize,1583879816882071253_usize,3_usize,2_usize,7_usize,2_usize];
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(3_usize, 29_usize, Move(_29), 31_usize, Move(_31), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(3_usize, 11_usize, Move(_11), 20_usize, Move(_20), 12_usize, Move(_12), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(3_usize, 35_usize, Move(_35), 4_usize, Move(_4), 30_usize, Move(_30), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [usize; 8],mut _2: [usize; 8],mut _3: [usize; 8],mut _4: [usize; 8],mut _5: [usize; 8],mut _6: [usize; 8],mut _7: [usize; 8],mut _8: [usize; 8],mut _9: u16,mut _10: [usize; 8],mut _11: [usize; 8],mut _12: [usize; 8],mut _13: [usize; 8],mut _14: u16,mut _15: char) -> [usize; 8] {
mir! {
type RET = [usize; 8];
let _16: isize;
let _17: ((*const char, f32),);
let _18: [isize; 1];
let _19: u16;
let _20: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]);
let _21: [u128; 1];
let _22: Adt48;
let _23: [bool; 7];
let _24: [bool; 7];
let _25: Adt49;
let _26: Adt49;
let _27: i16;
let _28: [i16; 6];
let _29: i32;
let _30: [bool; 7];
let _31: Adt57;
let _32: [i16; 6];
let _33: *const char;
let _34: [u8; 4];
let _35: i16;
let _36: isize;
let _37: [isize; 1];
let _38: f64;
let _39: *const char;
let _40: isize;
let _41: [i16; 2];
let _42: [u128; 1];
let _43: ();
let _44: ();
{
_11 = _8;
_10 = _7;
_14 = _9 + _9;
RET = _1;
_14 = 37_i8 as u16;
_5 = [3_usize,2_usize,0_usize,911966605341695416_usize,12620430201544422951_usize,4841925311814164349_usize,5_usize,12231219572512005037_usize];
_6 = _11;
_1 = [6478846240887789321_usize,10618375478827886699_usize,2_usize,3060270323759436353_usize,5903761888089087231_usize,3_usize,2207519425981490759_usize,2_usize];
_16 = (-76_isize);
_14 = _9;
_11 = [5_usize,1027616244779751117_usize,2204430921742444133_usize,13592439122735128880_usize,0_usize,8542906236823585154_usize,4_usize,6_usize];
_12 = _1;
_11 = _3;
_17.0.1 = 360138019_u32 as f32;
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211380 => bb5,
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
_3 = _6;
_14 = !_9;
_16 = 9223372036854775807_isize << _9;
_15 = '\u{2cfa8}';
_15 = '\u{60d2}';
_20.4 = [(-12660_i16),7618_i16,(-22926_i16),15229_i16,(-2381_i16),12949_i16];
Call(RET = fn5(_13, _20.4, _13, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_20.2 = [2576_i16,18777_i16,(-27823_i16),(-16122_i16),10179_i16,(-10359_i16)];
_17.0.0 = core::ptr::addr_of!(_15);
_16 = 555031202896253001_u64 as isize;
RET = [4_usize,15946417299335560595_usize,8524133793011798372_usize,10985122282655386874_usize,2950864559740152480_usize,9621653926710621365_usize,9057920546777285409_usize,13694057344402625323_usize];
Goto(bb7)
}
bb7 = {
_15 = '\u{3dc07}';
_12 = _7;
_23 = [false,true,true,true,false,true,true];
_15 = '\u{82619}';
_18 = [_16];
_21 = [272315722477585690663941200660594606711_u128];
_20.3 = -_16;
_11 = [9245899272407042918_usize,5_usize,12323240837236057255_usize,6_usize,16680224363981908968_usize,8700479819721504593_usize,4_usize,17388976965127590747_usize];
_19 = !_14;
_17.0.1 = 80_u8 as f32;
_20.0 = [1200_i16,(-2247_i16),(-8597_i16),19020_i16,(-3475_i16),17555_i16];
_20.1 = [178_u8,120_u8,226_u8,127_u8];
_6 = _7;
_17.0.1 = 202637815930612570728801688220240651785_u128 as f32;
_17.0.1 = _14 as f32;
_24 = [true,true,true,true,false,false,false];
_22 = Adt48::Variant1 { fld0: _21 };
_13 = [11783345202632487_usize,12995435997799114581_usize,13372532487251631439_usize,0_usize,9556667327898683212_usize,8686967408145162899_usize,1_usize,0_usize];
_10 = RET;
_6 = [7296843560500528280_usize,17745906150795658713_usize,5_usize,6_usize,7_usize,13892732226502107825_usize,13190994910648036867_usize,5915777122966793035_usize];
Goto(bb8)
}
bb8 = {
_20.1 = [1_u8,73_u8,212_u8,184_u8];
_20.2 = _20.4;
_15 = '\u{a9e62}';
SetDiscriminant(_22, 0);
_4 = _10;
Goto(bb9)
}
bb9 = {
_22 = Adt48::Variant0 { fld0: 12039_i16 };
place!(Field::<i16>(Variant(_22, 0), 0)) = (-29536_i16) & 25148_i16;
_27 = Field::<i16>(Variant(_22, 0), 0);
_11 = _2;
Goto(bb10)
}
bb10 = {
_20.0 = [Field::<i16>(Variant(_22, 0), 0),_27,_27,Field::<i16>(Variant(_22, 0), 0),_27,_27];
_10 = [12325004375551420279_usize,5504047663976424495_usize,18310284862023916233_usize,295661467304021317_usize,1373528337807466925_usize,8357642418560144069_usize,4_usize,2_usize];
_1 = [5614218862544575311_usize,3_usize,12275774597597981040_usize,2_usize,0_usize,4_usize,3333444789568329146_usize,4339181568462978922_usize];
Goto(bb11)
}
bb11 = {
SetDiscriminant(_22, 1);
_22 = Adt48::Variant0 { fld0: _27 };
_21 = [116279559021222392086358024783430979876_u128];
_25.fld0 = _20.3 as f64;
_28 = [Field::<i16>(Variant(_22, 0), 0),_27,Field::<i16>(Variant(_22, 0), 0),Field::<i16>(Variant(_22, 0), 0),Field::<i16>(Variant(_22, 0), 0),_27];
RET = _5;
_6 = _4;
_19 = _9 - _9;
_1 = [7_usize,3_usize,5_usize,2_usize,9607285441929795376_usize,12209950923439321995_usize,5_usize,11475969701815961828_usize];
_12 = RET;
SetDiscriminant(_22, 1);
_22 = Adt48::Variant1 { fld0: _21 };
_17.0.0 = core::ptr::addr_of!(_15);
Goto(bb12)
}
bb12 = {
_20.1 = [6_u8,221_u8,241_u8,21_u8];
_25.fld0 = _19 as f64;
Goto(bb13)
}
bb13 = {
_20.4 = [_27,_27,_27,_27,_27,_27];
_6 = _1;
_25.fld0 = (-73771053661554287913739982607338575104_i128) as f64;
_7 = [0_usize,2981442159589959039_usize,11647797273066886677_usize,6_usize,1_usize,4_usize,10772865670489664243_usize,6_usize];
_20.0 = [_27,_27,_27,_27,_27,_27];
_24 = [false,false,true,true,false,false,true];
_29 = 972750536_i32 & 1162996573_i32;
_16 = _20.3;
_26 = Adt49 { fld0: _25.fld0 };
_10 = _12;
_28 = [_27,_27,_27,_27,_27,_27];
_34 = [202_u8,17_u8,250_u8,217_u8];
_33 = _17.0.0;
_33 = _17.0.0;
SetDiscriminant(_22, 1);
Goto(bb14)
}
bb14 = {
_30 = [true,true,false,true,true,true,false];
place!(Field::<[u128; 1]>(Variant(_22, 1), 0)) = [142517898878538190269269211067250641355_u128];
_37 = [_20.3];
_30 = [true,false,false,false,false,true,false];
_20.3 = _16;
_35 = _19 as i16;
RET = [14137489781065881943_usize,4_usize,3_usize,2_usize,17160187922728182279_usize,2285298543417885446_usize,11477313364545798060_usize,16746138043917323758_usize];
_17.0.0 = core::ptr::addr_of!((*_33));
_20 = (_28, _34, _28, _16, _28);
_34 = [50_u8,240_u8,121_u8,249_u8];
_18 = _37;
_16 = _20.3;
(*_33) = '\u{c6942}';
(*_33) = '\u{9a7ef}';
_28 = [_27,_27,_27,_27,_27,_35];
SetDiscriminant(_22, 1);
_34 = _20.1;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(4_usize, 37_usize, Move(_37), 5_usize, Move(_5), 4_usize, Move(_4), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(4_usize, 16_usize, Move(_16), 30_usize, Move(_30), 18_usize, Move(_18), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(4_usize, 24_usize, Move(_24), 23_usize, Move(_23), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(4_usize, 35_usize, Move(_35), 8_usize, Move(_8), 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [usize; 8],mut _2: [i16; 6],mut _3: [usize; 8],mut _4: [usize; 8]) -> [usize; 8] {
mir! {
type RET = [usize; 8];
let _5: (u128, [u128; 1], &'static [u8; 4], [isize; 1]);
let _6: Adt49;
let _7: [usize; 8];
let _8: [usize; 8];
let _9: isize;
let _10: isize;
let _11: [bool; 7];
let _12: u32;
let _13: isize;
let _14: u32;
let _15: [i16; 2];
let _16: isize;
let _17: [u8; 4];
let _18: &'static [u8; 4];
let _19: [bool; 7];
let _20: i32;
let _21: isize;
let _22: bool;
let _23: Adt55;
let _24: Adt45;
let _25: u128;
let _26: isize;
let _27: (u128, [u128; 1], &'static [u8; 4], [isize; 1]);
let _28: f32;
let _29: char;
let _30: [u128; 1];
let _31: (*mut f64, *const char, u32, f32, u128);
let _32: u32;
let _33: Adt52;
let _34: f32;
let _35: isize;
let _36: Adt58;
let _37: ();
let _38: ();
{
RET = _1;
_2 = [(-18423_i16),29210_i16,(-31194_i16),(-10463_i16),27692_i16,(-13529_i16)];
_1 = [2_usize,6_usize,3_usize,2_usize,5_usize,2_usize,3510014298710280122_usize,1_usize];
_1 = [3_usize,4797118958974287771_usize,2423634670778020714_usize,7_usize,14486450318427268936_usize,5_usize,2778337820273158887_usize,5_usize];
_5.0 = 127279443290875255666752555500213649530_u128 + 162415008499429113614534852075881501941_u128;
_5.3 = [22_isize];
_5.1 = [_5.0];
_5.3 = [(-9223372036854775808_isize)];
RET = _4;
_5.1 = [_5.0];
_4 = [10283689842284441080_usize,8116950846397995925_usize,12560472834404019110_usize,4436571252851846062_usize,6_usize,4_usize,13887777076497751072_usize,8694286425132201128_usize];
_6.fld0 = (-4029235462743940873_i64) as f64;
_5.0 = 203460618362918263443871753210427877967_u128;
_6.fld0 = 9223372036854775807_isize as f64;
_2 = [(-12295_i16),(-26113_i16),(-13765_i16),17174_i16,(-21859_i16),(-10513_i16)];
_6.fld0 = 13278414267342641536_u64 as f64;
_4 = [4_usize,6917024110608715170_usize,13669237761759398363_usize,1_usize,8927173408330871245_usize,7_usize,13992761219740432651_usize,1182102142045220335_usize];
_6.fld0 = (-79127409686544220455334090006312957510_i128) as f64;
Goto(bb1)
}
bb1 = {
_6.fld0 = 25610_i16 as f64;
_3 = [10947694616143249906_usize,6_usize,5399499304088834634_usize,0_usize,7_usize,0_usize,9512473142856501464_usize,3_usize];
_5.1 = [_5.0];
RET = [4_usize,9603230567359692222_usize,11557036574434801764_usize,1601706723713411813_usize,15269243014605599462_usize,6082520914693219649_usize,1_usize,5_usize];
_7 = [16013651774980683594_usize,12086096495134147818_usize,2_usize,2_usize,3_usize,8617098552725375644_usize,9072801449995253502_usize,253324764486891726_usize];
_9 = (-9223372036854775808_isize);
_2 = [(-13082_i16),(-15696_i16),11310_i16,24055_i16,13508_i16,(-28374_i16)];
_6.fld0 = _9 as f64;
_6.fld0 = _9 as f64;
_1 = _7;
_7 = [3_usize,16315738653521170086_usize,2_usize,12506140382419694620_usize,4_usize,3_usize,7_usize,7_usize];
_7 = [3751843835938927799_usize,3_usize,6528566570255650865_usize,9356499726360823145_usize,0_usize,2711502180349184692_usize,14036047923582697800_usize,6_usize];
Goto(bb2)
}
bb2 = {
_6.fld0 = 34011_u16 as f64;
_10 = _9 >> _9;
_10 = 1057750045_u32 as isize;
_3 = _7;
_7 = [10810218642723665080_usize,16910426282382058837_usize,9820181682691809496_usize,13363202913290927973_usize,1_usize,4793740863423194589_usize,2_usize,13780934442435411475_usize];
_2 = [(-6295_i16),(-29138_i16),26519_i16,29896_i16,7784_i16,(-20187_i16)];
_3 = [13389035356361476421_usize,0_usize,8877671484132418338_usize,7134864137259398268_usize,7_usize,4_usize,184567217717164601_usize,776406951791894700_usize];
_11 = [false,true,false,false,false,false,false];
_3 = [15779384726386764427_usize,4_usize,13223257904801049183_usize,9053380405234052373_usize,3_usize,18197759466636592816_usize,3206473310456348399_usize,3_usize];
_4 = [510059177471671449_usize,3_usize,9762735528536620588_usize,8889764152373944241_usize,3_usize,5_usize,1_usize,14923446298767289283_usize];
_5.1 = [_5.0];
_8 = _3;
_10 = _9;
_4 = _7;
Goto(bb3)
}
bb3 = {
_15 = [(-22953_i16),(-26834_i16)];
_2 = [(-22922_i16),16556_i16,16661_i16,(-30144_i16),5100_i16,(-26622_i16)];
_11 = [false,true,false,false,true,false,true];
_5.3 = [_10];
_14 = 1663441957_u32 & 1414367447_u32;
_8 = [6_usize,1635337365583905050_usize,9221695189450027927_usize,4357589785011414804_usize,1_usize,2_usize,565836322922215474_usize,15656985459118978871_usize];
_10 = -_9;
_15 = [1759_i16,(-14196_i16)];
_2 = [32616_i16,2302_i16,26396_i16,10521_i16,30891_i16,27317_i16];
_12 = _14;
_7 = [3_usize,4_usize,10138527053927617822_usize,7594730827469714775_usize,7315402930631238403_usize,6_usize,7_usize,1585669393110183030_usize];
_13 = _9 & _10;
_1 = [1_usize,0_usize,7_usize,3_usize,13970764191932856228_usize,16129138978557350240_usize,3_usize,1870274239664638967_usize];
_9 = -_10;
_3 = [4_usize,9979619467802423266_usize,2_usize,2_usize,3770240919778017199_usize,12504441891381909743_usize,0_usize,6928472206059795102_usize];
_17 = [76_u8,234_u8,159_u8,192_u8];
_18 = &_17;
_4 = RET;
Goto(bb4)
}
bb4 = {
_3 = [2_usize,14788201109378478409_usize,0_usize,11123696555519305651_usize,5_usize,1_usize,2_usize,3_usize];
_19 = _11;
Goto(bb5)
}
bb5 = {
_21 = 3_usize as isize;
_5.3 = [_13];
_5.3 = [_9];
_11 = [false,false,false,false,true,false,true];
_10 = _13 + _13;
match _5.0 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
203460618362918263443871753210427877967 => bb10,
_ => bb9
}
}
bb6 = {
_3 = [2_usize,14788201109378478409_usize,0_usize,11123696555519305651_usize,5_usize,1_usize,2_usize,3_usize];
_19 = _11;
Goto(bb5)
}
bb7 = {
_15 = [(-22953_i16),(-26834_i16)];
_2 = [(-22922_i16),16556_i16,16661_i16,(-30144_i16),5100_i16,(-26622_i16)];
_11 = [false,true,false,false,true,false,true];
_5.3 = [_10];
_14 = 1663441957_u32 & 1414367447_u32;
_8 = [6_usize,1635337365583905050_usize,9221695189450027927_usize,4357589785011414804_usize,1_usize,2_usize,565836322922215474_usize,15656985459118978871_usize];
_10 = -_9;
_15 = [1759_i16,(-14196_i16)];
_2 = [32616_i16,2302_i16,26396_i16,10521_i16,30891_i16,27317_i16];
_12 = _14;
_7 = [3_usize,4_usize,10138527053927617822_usize,7594730827469714775_usize,7315402930631238403_usize,6_usize,7_usize,1585669393110183030_usize];
_13 = _9 & _10;
_1 = [1_usize,0_usize,7_usize,3_usize,13970764191932856228_usize,16129138978557350240_usize,3_usize,1870274239664638967_usize];
_9 = -_10;
_3 = [4_usize,9979619467802423266_usize,2_usize,2_usize,3770240919778017199_usize,12504441891381909743_usize,0_usize,6928472206059795102_usize];
_17 = [76_u8,234_u8,159_u8,192_u8];
_18 = &_17;
_4 = RET;
Goto(bb4)
}
bb8 = {
_6.fld0 = 34011_u16 as f64;
_10 = _9 >> _9;
_10 = 1057750045_u32 as isize;
_3 = _7;
_7 = [10810218642723665080_usize,16910426282382058837_usize,9820181682691809496_usize,13363202913290927973_usize,1_usize,4793740863423194589_usize,2_usize,13780934442435411475_usize];
_2 = [(-6295_i16),(-29138_i16),26519_i16,29896_i16,7784_i16,(-20187_i16)];
_3 = [13389035356361476421_usize,0_usize,8877671484132418338_usize,7134864137259398268_usize,7_usize,4_usize,184567217717164601_usize,776406951791894700_usize];
_11 = [false,true,false,false,false,false,false];
_3 = [15779384726386764427_usize,4_usize,13223257904801049183_usize,9053380405234052373_usize,3_usize,18197759466636592816_usize,3206473310456348399_usize,3_usize];
_4 = [510059177471671449_usize,3_usize,9762735528536620588_usize,8889764152373944241_usize,3_usize,5_usize,1_usize,14923446298767289283_usize];
_5.1 = [_5.0];
_8 = _3;
_10 = _9;
_4 = _7;
Goto(bb3)
}
bb9 = {
_6.fld0 = 25610_i16 as f64;
_3 = [10947694616143249906_usize,6_usize,5399499304088834634_usize,0_usize,7_usize,0_usize,9512473142856501464_usize,3_usize];
_5.1 = [_5.0];
RET = [4_usize,9603230567359692222_usize,11557036574434801764_usize,1601706723713411813_usize,15269243014605599462_usize,6082520914693219649_usize,1_usize,5_usize];
_7 = [16013651774980683594_usize,12086096495134147818_usize,2_usize,2_usize,3_usize,8617098552725375644_usize,9072801449995253502_usize,253324764486891726_usize];
_9 = (-9223372036854775808_isize);
_2 = [(-13082_i16),(-15696_i16),11310_i16,24055_i16,13508_i16,(-28374_i16)];
_6.fld0 = _9 as f64;
_6.fld0 = _9 as f64;
_1 = _7;
_7 = [3_usize,16315738653521170086_usize,2_usize,12506140382419694620_usize,4_usize,3_usize,7_usize,7_usize];
_7 = [3751843835938927799_usize,3_usize,6528566570255650865_usize,9356499726360823145_usize,0_usize,2711502180349184692_usize,14036047923582697800_usize,6_usize];
Goto(bb2)
}
bb10 = {
_19 = [true,true,false,false,false,false,false];
_7 = [0_usize,9742148010816319344_usize,2795930967935943965_usize,2983024530670336398_usize,5_usize,6807621134069182869_usize,6_usize,7_usize];
_20 = true as i32;
RET = _8;
RET = [3_usize,18219382963357711218_usize,1279311830650306315_usize,5_usize,8816512117357185853_usize,1539209905685687655_usize,3_usize,3567888229832684372_usize];
_5.1 = [_5.0];
_13 = _10 - _10;
_16 = _14 as isize;
_22 = true;
_22 = false;
_23.fld1.fld2 = [186_u8,124_u8,201_u8,165_u8];
_23.fld1.fld2 = (*_18);
_1 = RET;
_15 = [(-27233_i16),22427_i16];
_5.3 = [_13];
_2 = [23554_i16,(-11185_i16),3589_i16,(-28432_i16),29888_i16,30338_i16];
_23.fld2.1 = _10 as f32;
_23.fld4 = _14 | _12;
_12 = !_14;
_23.fld3 = [(-25302_i16),(-25124_i16),(-25558_i16)];
Call(_25 = fn6(_5.3, Move(_18), _10, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = [_22,_22,_22,_22,_22,_22,_22];
_16 = !_13;
_23.fld4 = _12 - _12;
_27.0 = _23.fld2.1 as u128;
RET = [13092277727664631820_usize,1_usize,17980232289156682908_usize,3_usize,0_usize,4786675128212659617_usize,4537242655701771435_usize,2049280509725413041_usize];
_27.3 = [_13];
_6.fld0 = 7333816961813645299_i64 as f64;
match _5.0 {
0 => bb10,
203460618362918263443871753210427877967 => bb12,
_ => bb7
}
}
bb12 = {
_28 = _23.fld2.1;
_8 = [7_usize,14135332840337836274_usize,13605124671727231760_usize,3848387027026995800_usize,4_usize,5_usize,4_usize,0_usize];
_5.2 = &_23.fld1.fld2;
_9 = (-37_i8) as isize;
_7 = _4;
_14 = _23.fld4 + _23.fld4;
_5.0 = _27.0;
_19 = [_22,_22,_22,_22,_22,_22,_22];
_23.fld2.0 = core::ptr::addr_of!(_29);
_23.fld1.fld0 = -(-15679_i16);
RET = [3_usize,17780123494405333496_usize,1_usize,5711792999888491537_usize,2_usize,0_usize,8760300293084214485_usize,9604536271613257223_usize];
_12 = _23.fld4 - _14;
_30 = _5.1;
_23.fld1.fld2 = [198_u8,200_u8,166_u8,84_u8];
_10 = -_13;
_27.1 = [_5.0];
Goto(bb13)
}
bb13 = {
_29 = '\u{fe83b}';
_18 = &_23.fld1.fld2;
_23.fld6 = _27.0 as i64;
Call(_29 = fn17(_16, _12, _3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_8 = [16124236369443714473_usize,7_usize,15234890910827472100_usize,6_usize,3946762061713953288_usize,10855629812254707347_usize,7_usize,13597050695121542167_usize];
_23.fld0 = _17;
_26 = _16 - _16;
_27.2 = &_23.fld1.fld2;
_7 = [5_usize,12566840677231173491_usize,2_usize,289441699614980268_usize,6_usize,2_usize,1_usize,4_usize];
_27.0 = 11204_u16 as u128;
_5 = (_25, _27.1, Move(_27.2), _27.3);
_25 = 86_i8 as u128;
_3 = [3_usize,0_usize,14430688240957415948_usize,14620425663691958324_usize,0_usize,15254424171283849178_usize,15712939510417696632_usize,16766709713760220274_usize];
_8 = [17813729628971483557_usize,4_usize,6474523644161503718_usize,5_usize,13359839087313569970_usize,6_usize,3987261368791950458_usize,12947820597930740501_usize];
_31.0 = core::ptr::addr_of_mut!(_6.fld0);
_27.3 = _5.3;
_19 = [_22,_22,_22,_22,_22,_22,_22];
_2 = [_23.fld1.fld0,_23.fld1.fld0,_23.fld1.fld0,_23.fld1.fld0,_23.fld1.fld0,_23.fld1.fld0];
_30 = [_27.0];
_31.4 = _25 - _5.0;
_30 = [_25];
_34 = _23.fld6 as f32;
_31.3 = -_23.fld2.1;
_27 = Move(_5);
_31.1 = _23.fld2.0;
_1 = [11091731655825709917_usize,7_usize,11958338082853852777_usize,484604739311894094_usize,1074171738603763661_usize,6_usize,1_usize,0_usize];
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(5_usize, 11_usize, Move(_11), 16_usize, Move(_16), 30_usize, Move(_30), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(5_usize, 14_usize, Move(_14), 29_usize, Move(_29), 22_usize, Move(_22), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(5_usize, 12_usize, Move(_12), 19_usize, Move(_19), 4_usize, Move(_4), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [isize; 1],mut _2: &'static [u8; 4],mut _3: isize,mut _4: [usize; 8]) -> u128 {
mir! {
type RET = u128;
let _5: char;
let _6: f32;
let _7: char;
let _8: isize;
let _9: [i64; 2];
let _10: u64;
let _11: f64;
let _12: [u128; 1];
let _13: [isize; 1];
let _14: usize;
let _15: f32;
let _16: isize;
let _17: char;
let _18: [u16; 5];
let _19: i16;
let _20: usize;
let _21: (u128, i8);
let _22: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]);
let _23: *mut [i16; 6];
let _24: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)));
let _25: [i16; 2];
let _26: [i16; 6];
let _27: Adt53;
let _28: Adt46;
let _29: i32;
let _30: Adt55;
let _31: [i64; 2];
let _32: u16;
let _33: [i16; 6];
let _34: (u128, [u128; 1], &'static [u8; 4], [isize; 1]);
let _35: ();
let _36: ();
{
Call(RET = fn7(_3, _1, Move(_2), _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !157410372459323440860126870512427176447_u128;
RET = 293502486859952237476841987339122868899_u128 & 257275465682402397097250346830222902568_u128;
_5 = '\u{58f0a}';
RET = 175732024889522160216376869016765665399_u128;
_1 = [_3];
RET = _3 as u128;
_3 = (-9223372036854775808_isize) << RET;
_4 = [2_usize,4_usize,18018592742659328388_usize,5063846746500761600_usize,16323687375898587739_usize,13305408524852699220_usize,6_usize,12982328385432226077_usize];
_1 = [_3];
_4 = [3_usize,13861637030489096805_usize,4883013669871417249_usize,3384366095142044890_usize,13305024427353415857_usize,7_usize,4_usize,2_usize];
_3 = 9223372036854775807_isize + 9223372036854775807_isize;
_4 = [8547064145228610653_usize,17359677587823865041_usize,15113878526371094118_usize,4208259358765559627_usize,7_usize,7633431190745082442_usize,6378341681690742754_usize,15358589422517229812_usize];
_5 = '\u{268ec}';
RET = 16936212269909725580454987461818549175_u128;
_4 = [8493568918174404361_usize,615907985615180534_usize,2_usize,16250774877952937010_usize,1282023557477207629_usize,3042713002029222698_usize,1294753532838387977_usize,4_usize];
_6 = (-94284604177968371258174371218009213918_i128) as f32;
_7 = _5;
RET = !143951722937999915609608466957617752199_u128;
_9 = [3279820650052701612_i64,2945634610661815514_i64];
_4 = [7955568780884040768_usize,5_usize,4_usize,2_usize,1_usize,15786047771586787596_usize,14747781643865403894_usize,6_usize];
Goto(bb2)
}
bb2 = {
_9 = [4943850012063679476_i64,7153265606379951265_i64];
_5 = _7;
_8 = RET as isize;
RET = 12994608934926304471_u64 as u128;
_10 = _8 as u64;
_7 = _5;
_5 = _7;
_5 = _7;
_9 = [(-6943746088518017765_i64),8781906032268650345_i64];
Goto(bb3)
}
bb3 = {
_1 = [_8];
_8 = _3;
_5 = _7;
_6 = _3 as f32;
_9 = [5422592883760928932_i64,(-2392027926239854647_i64)];
_12 = [RET];
_11 = (-10_i8) as f64;
_7 = _5;
RET = 125912946752733600624830570576422495742_u128;
_14 = !4110608633050519561_usize;
_13 = [_8];
_11 = 89952979546878067900641557879951257666_i128 as f64;
_9 = [2640893471927704564_i64,(-1032806489917021710_i64)];
_5 = _7;
_13 = _1;
_11 = 113_u8 as f64;
_12 = [RET];
_5 = _7;
_15 = (-101_i8) as f32;
RET = _14 as u128;
Goto(bb4)
}
bb4 = {
_13 = _1;
_11 = _15 as f64;
_5 = _7;
_6 = _10 as f32;
RET = 68278310316490394159181278618544609778_u128;
_7 = _5;
_1 = [_3];
_17 = _5;
_8 = 48_i8 as isize;
_8 = _3;
_16 = 57175_u16 as isize;
_5 = _17;
_14 = 26527_u16 as usize;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_9 = [271581440635790356_i64,4371707105248495975_i64];
_14 = 11617178782576487530_usize + 397609680580537588_usize;
_18 = [17448_u16,57331_u16,8782_u16,16853_u16,5460_u16];
_14 = _10 as usize;
_6 = _15;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_12 = [RET];
_5 = _7;
_12 = [RET];
_16 = _8 ^ _3;
_20 = _14;
Goto(bb5)
}
bb5 = {
_12 = [RET];
_10 = !4834928608882644041_u64;
RET = 113567619539630682756155662865596906453_u128;
_19 = (-12253_i16);
_17 = _7;
_3 = 144_u8 as isize;
_8 = _10 as isize;
_8 = !_16;
_21.1 = -97_i8;
_14 = _20 << _10;
_22.0 = [_19,_19,_19,_19,_19,_19];
_10 = 5078445246950584218_u64 >> _16;
_20 = _14 - _14;
_22.0 = [_19,_19,_19,_19,_19,_19];
Goto(bb6)
}
bb6 = {
RET = !234369780090838920323996004879953452478_u128;
_5 = _17;
_21.1 = -85_i8;
_4 = [_20,_20,_20,_20,_14,_20,_14,_14];
_21 = (RET, (-116_i8));
_22.2 = [_19,_19,_19,_19,_19,_19];
_11 = 249_u8 as f64;
match _21.1 {
0 => bb7,
340282366920938463463374607431768211340 => bb9,
_ => bb8
}
}
bb7 = {
_12 = [RET];
_10 = !4834928608882644041_u64;
RET = 113567619539630682756155662865596906453_u128;
_19 = (-12253_i16);
_17 = _7;
_3 = 144_u8 as isize;
_8 = _10 as isize;
_8 = !_16;
_21.1 = -97_i8;
_14 = _20 << _10;
_22.0 = [_19,_19,_19,_19,_19,_19];
_10 = 5078445246950584218_u64 >> _16;
_20 = _14 - _14;
_22.0 = [_19,_19,_19,_19,_19,_19];
Goto(bb6)
}
bb8 = {
RET = !157410372459323440860126870512427176447_u128;
RET = 293502486859952237476841987339122868899_u128 & 257275465682402397097250346830222902568_u128;
_5 = '\u{58f0a}';
RET = 175732024889522160216376869016765665399_u128;
_1 = [_3];
RET = _3 as u128;
_3 = (-9223372036854775808_isize) << RET;
_4 = [2_usize,4_usize,18018592742659328388_usize,5063846746500761600_usize,16323687375898587739_usize,13305408524852699220_usize,6_usize,12982328385432226077_usize];
_1 = [_3];
_4 = [3_usize,13861637030489096805_usize,4883013669871417249_usize,3384366095142044890_usize,13305024427353415857_usize,7_usize,4_usize,2_usize];
_3 = 9223372036854775807_isize + 9223372036854775807_isize;
_4 = [8547064145228610653_usize,17359677587823865041_usize,15113878526371094118_usize,4208259358765559627_usize,7_usize,7633431190745082442_usize,6378341681690742754_usize,15358589422517229812_usize];
_5 = '\u{268ec}';
RET = 16936212269909725580454987461818549175_u128;
_4 = [8493568918174404361_usize,615907985615180534_usize,2_usize,16250774877952937010_usize,1282023557477207629_usize,3042713002029222698_usize,1294753532838387977_usize,4_usize];
_6 = (-94284604177968371258174371218009213918_i128) as f32;
_7 = _5;
RET = !143951722937999915609608466957617752199_u128;
_9 = [3279820650052701612_i64,2945634610661815514_i64];
_4 = [7955568780884040768_usize,5_usize,4_usize,2_usize,1_usize,15786047771586787596_usize,14747781643865403894_usize,6_usize];
Goto(bb2)
}
bb9 = {
_8 = !_16;
_4 = [_20,_14,_14,_20,_14,_20,_20,_14];
_21.1 = 26_i8;
_2 = &_22.1;
_9 = [(-5058303455817155038_i64),5742674094876997094_i64];
_22.3 = 140328537203291946617219572391324157408_i128 as isize;
_3 = 92543419972087346230507154342631341202_i128 as isize;
_22.1 = [197_u8,66_u8,76_u8,150_u8];
_22.4 = _22.2;
_24.1.0 = RET as i8;
_24.1.3 = (_22.4, _22.1, _22.0, _8, _22.4);
_24.1.7.0 = core::ptr::addr_of!(_5);
_22.1 = [49_u8,13_u8,150_u8,146_u8];
_24.1.3.3 = 18815_u16 as isize;
_24.0 = core::ptr::addr_of!(_7);
_24.1.7 = (_24.0, _15);
_9 = [4875269902254361421_i64,426476213140374353_i64];
_24.1.7.0 = core::ptr::addr_of!(_7);
_22.0 = _22.2;
_15 = _24.1.7.1;
_21.0 = RET;
_24.1.3.1 = [183_u8,221_u8,200_u8,19_u8];
_24.1.4 = _21.0 as i128;
Call(_27.fld1 = fn14(_5, _10, _1, _18), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_22.1 = [78_u8,6_u8,41_u8,232_u8];
_26 = [_19,_19,_19,_19,_19,_19];
_24.1.3.4 = _24.1.3.0;
_11 = (-1177322858_i32) as f64;
_30.fld6 = !_27.fld1;
_3 = _8;
_8 = _22.3;
Goto(bb11)
}
bb11 = {
_2 = &_30.fld1.fld2;
_30.fld1.fld1 = -_27.fld1;
_24.1.1 = _21.1 as i128;
_29 = -1447672971_i32;
_13 = _1;
match _19 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768199203 => bb16,
_ => bb15
}
}
bb12 = {
_22.1 = [78_u8,6_u8,41_u8,232_u8];
_26 = [_19,_19,_19,_19,_19,_19];
_24.1.3.4 = _24.1.3.0;
_11 = (-1177322858_i32) as f64;
_30.fld6 = !_27.fld1;
_3 = _8;
_8 = _22.3;
Goto(bb11)
}
bb13 = {
_8 = !_16;
_4 = [_20,_14,_14,_20,_14,_20,_20,_14];
_21.1 = 26_i8;
_2 = &_22.1;
_9 = [(-5058303455817155038_i64),5742674094876997094_i64];
_22.3 = 140328537203291946617219572391324157408_i128 as isize;
_3 = 92543419972087346230507154342631341202_i128 as isize;
_22.1 = [197_u8,66_u8,76_u8,150_u8];
_22.4 = _22.2;
_24.1.0 = RET as i8;
_24.1.3 = (_22.4, _22.1, _22.0, _8, _22.4);
_24.1.7.0 = core::ptr::addr_of!(_5);
_22.1 = [49_u8,13_u8,150_u8,146_u8];
_24.1.3.3 = 18815_u16 as isize;
_24.0 = core::ptr::addr_of!(_7);
_24.1.7 = (_24.0, _15);
_9 = [4875269902254361421_i64,426476213140374353_i64];
_24.1.7.0 = core::ptr::addr_of!(_7);
_22.0 = _22.2;
_15 = _24.1.7.1;
_21.0 = RET;
_24.1.3.1 = [183_u8,221_u8,200_u8,19_u8];
_24.1.4 = _21.0 as i128;
Call(_27.fld1 = fn14(_5, _10, _1, _18), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
_13 = _1;
_11 = _15 as f64;
_5 = _7;
_6 = _10 as f32;
RET = 68278310316490394159181278618544609778_u128;
_7 = _5;
_1 = [_3];
_17 = _5;
_8 = 48_i8 as isize;
_8 = _3;
_16 = 57175_u16 as isize;
_5 = _17;
_14 = 26527_u16 as usize;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_9 = [271581440635790356_i64,4371707105248495975_i64];
_14 = 11617178782576487530_usize + 397609680580537588_usize;
_18 = [17448_u16,57331_u16,8782_u16,16853_u16,5460_u16];
_14 = _10 as usize;
_6 = _15;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_12 = [RET];
_5 = _7;
_12 = [RET];
_16 = _8 ^ _3;
_20 = _14;
Goto(bb5)
}
bb15 = {
_12 = [RET];
_10 = !4834928608882644041_u64;
RET = 113567619539630682756155662865596906453_u128;
_19 = (-12253_i16);
_17 = _7;
_3 = 144_u8 as isize;
_8 = _10 as isize;
_8 = !_16;
_21.1 = -97_i8;
_14 = _20 << _10;
_22.0 = [_19,_19,_19,_19,_19,_19];
_10 = 5078445246950584218_u64 >> _16;
_20 = _14 - _14;
_22.0 = [_19,_19,_19,_19,_19,_19];
Goto(bb6)
}
bb16 = {
_30.fld4 = _19 as u32;
_2 = &(*_2);
_24.0 = core::ptr::addr_of!(_17);
_30.fld2.0 = _24.0;
_7 = _5;
_30.fld1.fld3 = Adt47::Variant0 { fld0: _10,fld1: _24.1.7.0 };
_22.3 = _24.1.7.1 as isize;
place!(Field::<u64>(Variant(_30.fld1.fld3, 0), 0)) = _10 & _10;
_24.1.7 = (_24.0, _6);
_29 = _5 as i32;
_26 = [_19,_19,_19,_19,_19,_19];
_22.2 = [_19,_19,_19,_19,_19,_19];
_33 = _24.1.3.4;
SetDiscriminant(_30.fld1.fld3, 2);
place!(Field::<((*const char, f32),)>(Variant(_30.fld1.fld3, 2), 4)).0 = _24.1.7;
_30.fld2.1 = _15;
_24.1.5 = Field::<((*const char, f32),)>(Variant(_30.fld1.fld3, 2), 4).0.1;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_30.fld1.fld3, 2), 1)).1 = (_21.1, _24.1.1, _19, _22, _24.1.4, _15, _7, Field::<((*const char, f32),)>(Variant(_30.fld1.fld3, 2), 4).0);
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_30.fld1.fld3, 2), 1)).1.2 = -_19;
_30.fld1.fld0 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_30.fld1.fld3, 2), 1).1.2 | Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_30.fld1.fld3, 2), 1).1.2;
_24.1.6 = _17;
place!(Field::<bool>(Variant(_30.fld1.fld3, 2), 0)) = true;
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(6_usize, 14_usize, Move(_14), 33_usize, Move(_33), 13_usize, Move(_13), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(6_usize, 18_usize, Move(_18), 17_usize, Move(_17), 29_usize, Move(_29), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(6_usize, 7_usize, Move(_7), 19_usize, Move(_19), 36_usize, _36, 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: [isize; 1],mut _3: &'static [u8; 4],mut _4: [usize; 8]) -> u128 {
mir! {
type RET = u128;
let _5: (u128, i8);
let _6: Adt48;
let _7: u8;
let _8: Adt54;
let _9: [i16; 6];
let _10: f64;
let _11: Adt52;
let _12: [isize; 1];
let _13: [i16; 6];
let _14: Adt44;
let _15: Adt44;
let _16: char;
let _17: Adt47;
let _18: [u8; 7];
let _19: f32;
let _20: i8;
let _21: Adt56;
let _22: [bool; 7];
let _23: [i64; 2];
let _24: isize;
let _25: f64;
let _26: Adt57;
let _27: *mut f64;
let _28: i32;
let _29: isize;
let _30: (*const char, f32);
let _31: isize;
let _32: [isize; 1];
let _33: isize;
let _34: ();
let _35: ();
{
_1 = !9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_1 = !(-12_isize);
RET = 11177821609070542652_u64 as u128;
RET = _1 as u128;
RET = !169560846549269989330399288062981217295_u128;
_3 = &(*_3);
_3 = &(*_3);
_4 = [1_usize,4_usize,12086441340708062361_usize,4_usize,16130323453952923163_usize,7_usize,2_usize,17054089176200254879_usize];
_5.0 = !RET;
Goto(bb2)
}
bb2 = {
_5 = (RET, 118_i8);
_5 = (RET, 65_i8);
_5.0 = RET;
_3 = &(*_3);
_2 = [_1];
_3 = &(*_3);
_5.0 = !RET;
_5 = (RET, 85_i8);
_5 = (RET, 54_i8);
RET = 5_usize as u128;
RET = !_5.0;
_3 = &(*_3);
RET = _5.0;
match _5.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
54 => bb7,
_ => bb6
}
}
bb3 = {
_1 = !(-12_isize);
RET = 11177821609070542652_u64 as u128;
RET = _1 as u128;
RET = !169560846549269989330399288062981217295_u128;
_3 = &(*_3);
_3 = &(*_3);
_4 = [1_usize,4_usize,12086441340708062361_usize,4_usize,16130323453952923163_usize,7_usize,2_usize,17054089176200254879_usize];
_5.0 = !RET;
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
_7 = 157_u8 + 108_u8;
RET = !_5.0;
_7 = 197_u8;
_2 = [_1];
_3 = &(*_3);
RET = _5.0 * _5.0;
_3 = &(*_3);
_2 = [_1];
_9 = [8270_i16,6094_i16,4665_i16,31059_i16,2442_i16,(-24193_i16)];
_5 = (RET, (-20_i8));
_2 = [_1];
_7 = false as u8;
_2 = [_1];
RET = _5.0;
_5 = (RET, (-96_i8));
_6 = Adt48::Variant0 { fld0: 12755_i16 };
_6 = Adt48::Variant0 { fld0: (-17335_i16) };
_10 = _5.1 as f64;
_3 = &(*_3);
place!(Field::<i16>(Variant(_6, 0), 0)) = (-3519_i16) & 31146_i16;
_6 = Adt48::Variant0 { fld0: 2706_i16 };
_6 = Adt48::Variant0 { fld0: 15755_i16 };
_7 = _5.1 as u8;
_3 = &(*_3);
_3 = &(*_3);
Call(_9 = fn8(_5.1, (*_3), _5, _5.1, (*_3), (*_3), _4, _5.1, _1, _2, _7, _1, (*_3), (*_3), _4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3 = &(*_3);
_9 = [26868_i16,(-2544_i16),(-3997_i16),30092_i16,3212_i16,20462_i16];
Call(_6 = fn13(_10, _1, (*_3), _9, _4, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = _5.0;
_3 = &(*_3);
_3 = &(*_3);
_1 = 27473_i16 as isize;
_6 = Adt48::Variant0 { fld0: 21610_i16 };
_3 = &(*_3);
RET = _5.0 + _5.0;
_13 = [30328_i16,(-21893_i16),21403_i16,(-17273_i16),20508_i16,13488_i16];
_4 = [0_usize,1_usize,1_usize,8136319930635560605_usize,13473347076302338535_usize,6109762828483703294_usize,14229079993363598323_usize,2971658520753011088_usize];
RET = _5.0;
_5 = (RET, 57_i8);
place!(Field::<i16>(Variant(_6, 0), 0)) = -(-10044_i16);
_9 = [Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0)];
RET = 6861936746163124204_i64 as u128;
_10 = 2334982287_u32 as f64;
_12 = _2;
_5.1 = -(-113_i8);
SetDiscriminant(_6, 0);
_16 = '\u{8a339}';
_13 = [31961_i16,13515_i16,29765_i16,(-6989_i16),(-14405_i16),(-18790_i16)];
place!(Field::<i16>(Variant(_6, 0), 0)) = (-498986770669517697_i64) as i16;
place!(Field::<i16>(Variant(_6, 0), 0)) = -(-7072_i16);
_9 = [Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0)];
_3 = &(*_3);
_4 = [6_usize,3_usize,4_usize,6_usize,14346095394516311898_usize,9200079556266306634_usize,3_usize,3_usize];
_7 = 105853879013451584774394016227923454486_i128 as u8;
_4 = [5_usize,5557179483200419181_usize,15264526587623905835_usize,6907024330193817338_usize,0_usize,2_usize,6_usize,3_usize];
_4 = [2890649451946059941_usize,5_usize,1_usize,12947287766768885263_usize,12353954820200156872_usize,5_usize,18209781465560251132_usize,10063960535213668525_usize];
Call(_1 = core::intrinsics::bswap((-35_isize)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9 = [Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0)];
_5 = (RET, (-126_i8));
_18 = [_7,_7,_7,_7,_7,_7,_7];
_3 = &(*_3);
_16 = '\u{ad100}';
_9 = [Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0),Field::<i16>(Variant(_6, 0), 0)];
_5.0 = 611387712644809175_i64 as u128;
_19 = 10249488481005134750_u64 as f32;
_22 = [false,false,true,false,false,true,true];
_7 = 197_u8 + 62_u8;
Goto(bb11)
}
bb11 = {
_20 = _5.1;
Goto(bb12)
}
bb12 = {
_20 = _5.1 + _5.1;
_5 = (RET, _20);
_10 = _1 as f64;
_13 = _9;
place!(Field::<i16>(Variant(_6, 0), 0)) = 14262_i16;
SetDiscriminant(_6, 0);
Goto(bb13)
}
bb13 = {
_20 = 14878068829302711080_u64 as i8;
_4 = [6_usize,486015385921122468_usize,6_usize,0_usize,8473980134421209220_usize,4300364132921592391_usize,7_usize,9596604838792357818_usize];
_22 = [true,false,false,false,true,true,false];
_3 = &(*_3);
_23 = [(-8262597554851533921_i64),354120306661864258_i64];
_12 = [_1];
Goto(bb14)
}
bb14 = {
_12 = [_1];
_29 = _1;
_22 = [false,true,true,false,false,false,true];
_27 = core::ptr::addr_of_mut!(_10);
_3 = &(*_3);
_31 = -_1;
_3 = &(*_3);
_29 = _1 + _31;
_16 = '\u{24342}';
_30.0 = core::ptr::addr_of!(_16);
_30.1 = _19;
_5.1 = _20;
_5 = (RET, _20);
_3 = &(*_3);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(7_usize, 29_usize, Move(_29), 22_usize, Move(_22), 7_usize, Move(_7), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(7_usize, 5_usize, Move(_5), 1_usize, Move(_1), 12_usize, Move(_12), 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i8,mut _2: [u8; 4],mut _3: (u128, i8),mut _4: i8,mut _5: [u8; 4],mut _6: [u8; 4],mut _7: [usize; 8],mut _8: i8,mut _9: isize,mut _10: [isize; 1],mut _11: u8,mut _12: isize,mut _13: [u8; 4],mut _14: [u8; 4],mut _15: [usize; 8]) -> [i16; 6] {
mir! {
type RET = [i16; 6];
let _16: f64;
let _17: [i64; 2];
let _18: [usize; 8];
let _19: Adt59;
let _20: [i64; 2];
let _21: u16;
let _22: u128;
let _23: isize;
let _24: f64;
let _25: Adt55;
let _26: [i16; 3];
let _27: bool;
let _28: [bool; 7];
let _29: char;
let _30: [u8; 7];
let _31: Adt49;
let _32: isize;
let _33: (u128, [u128; 1], &'static [u8; 4], [isize; 1]);
let _34: [u16; 5];
let _35: [i16; 6];
let _36: bool;
let _37: u128;
let _38: bool;
let _39: [u16; 5];
let _40: [u8; 7];
let _41: Adt58;
let _42: [u8; 7];
let _43: Adt49;
let _44: isize;
let _45: bool;
let _46: ();
let _47: ();
{
_9 = !_12;
_8 = -_4;
_15 = [2_usize,14240380295959124654_usize,4_usize,4452253852425149532_usize,3116558206727519396_usize,11942903909120188056_usize,6_usize,5699665203002539263_usize];
_12 = _9;
_1 = _3.1;
_14 = [_11,_11,_11,_11];
_10 = [_12];
_8 = _3.1;
RET = [24090_i16,(-17609_i16),(-10680_i16),(-27265_i16),29360_i16,29720_i16];
_15 = _7;
_7 = _15;
_12 = _9 << _3.0;
_3 = (306015705847892542935096682290158742457_u128, _1);
_17 = [(-931667640615404502_i64),(-961904837423973947_i64)];
Goto(bb1)
}
bb1 = {
_1 = _3.0 as i8;
_18 = [2984864604149192458_usize,12807558286113890502_usize,17026219463847057021_usize,17398247088428171484_usize,4_usize,3_usize,11074571914372270244_usize,16003501299910398391_usize];
_17 = [(-3990290017062290513_i64),(-5817086025613927377_i64)];
_2 = _5;
_8 = _4 - _3.1;
_1 = false as i8;
_6 = [_11,_11,_11,_11];
_22 = _3.0;
_10 = [_12];
RET = [14409_i16,6227_i16,13060_i16,10960_i16,4570_i16,(-6983_i16)];
_13 = _2;
_12 = !_9;
_22 = _3.0;
match _3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
306015705847892542935096682290158742457 => bb6,
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
_24 = _3.1 as f64;
_20 = [4972834539383841995_i64,(-4228424301685267056_i64)];
RET = [(-26648_i16),(-2625_i16),(-31718_i16),(-6029_i16),(-22812_i16),32235_i16];
_23 = _9 & _9;
_23 = _9 >> _11;
_23 = _12;
_21 = _8 as u16;
_11 = 153_u8;
_11 = !247_u8;
_2 = _13;
_5 = _13;
_17 = [5206462961916728633_i64,2162413451463181962_i64];
_18 = [4759907922985037920_usize,138836307245401577_usize,6_usize,11624149665345769829_usize,3_usize,3666886238153390736_usize,5376834035986129540_usize,0_usize];
_21 = _8 as u16;
_15 = [12785122406990390102_usize,1_usize,3_usize,7_usize,1_usize,14163154342189858983_usize,10031125080249320414_usize,15292254568063392945_usize];
_6 = [_11,_11,_11,_11];
_3.0 = _22;
_8 = _3.1;
_23 = -_12;
_25.fld1.fld0 = (-26876_i16);
_24 = _11 as f64;
_18 = _15;
_10 = [_23];
_13 = [_11,_11,_11,_11];
_20 = [6805238757086012011_i64,(-7742774037223440117_i64)];
_3.0 = _22;
Call(_25.fld1.fld3 = fn9(_8, _7, _18, _3.0, _3, _7, _8, _7, _4, _18, _15, _3.0, _8, _5, _6, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = _23 + _12;
_17 = [1334717687216950364_i64,(-7428440144485560208_i64)];
_25.fld1.fld2 = _5;
_16 = _24 * _24;
SetDiscriminant(_25.fld1.fld3, 0);
_2 = _25.fld1.fld2;
_6 = [_11,_11,_11,_11];
_21 = 38912_u16;
_7 = [0_usize,4966130489734285488_usize,17389987280166974861_usize,3648613097602967389_usize,2_usize,1_usize,10684000282400806818_usize,7_usize];
_16 = 16366670346113905739_u64 as f64;
_8 = _1;
RET = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_25.fld3 = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_25.fld4 = !425308997_u32;
_12 = false as isize;
_17 = _20;
_21 = !49546_u16;
_25.fld1.fld0 = !(-11851_i16);
_2 = [_11,_11,_11,_11];
RET = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_17 = _20;
RET = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_26 = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
match _4 {
340282366920938463463374607431768211360 => bb9,
_ => bb8
}
}
bb8 = {
_1 = _3.0 as i8;
_18 = [2984864604149192458_usize,12807558286113890502_usize,17026219463847057021_usize,17398247088428171484_usize,4_usize,3_usize,11074571914372270244_usize,16003501299910398391_usize];
_17 = [(-3990290017062290513_i64),(-5817086025613927377_i64)];
_2 = _5;
_8 = _4 - _3.1;
_1 = false as i8;
_6 = [_11,_11,_11,_11];
_22 = _3.0;
_10 = [_12];
RET = [14409_i16,6227_i16,13060_i16,10960_i16,4570_i16,(-6983_i16)];
_13 = _2;
_12 = !_9;
_22 = _3.0;
match _3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
306015705847892542935096682290158742457 => bb6,
_ => bb5
}
}
bb9 = {
_26 = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_25.fld1.fld1 = (-2042950773530800464_i64);
Goto(bb10)
}
bb10 = {
_8 = -_4;
_32 = -_9;
_29 = '\u{ed4fa}';
_5 = [_11,_11,_11,_11];
_25.fld1.fld2 = [_11,_11,_11,_11];
_25.fld4 = 2957141599_u32;
_29 = '\u{64dd5}';
_31.fld0 = _24;
_29 = '\u{cbff5}';
_29 = '\u{98835}';
Goto(bb11)
}
bb11 = {
RET = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_32 = -_23;
_27 = !false;
_25.fld0 = [_11,_11,_11,_11];
_18 = [3_usize,16239460047864117995_usize,17198907463158718753_usize,5463486754653138307_usize,1_usize,2_usize,3_usize,4707763446084032655_usize];
_17 = [_25.fld1.fld1,_25.fld1.fld1];
_17 = _20;
_25.fld2.0 = core::ptr::addr_of!(_29);
_1 = -_3.1;
_13 = _25.fld1.fld2;
_33.1 = [_22];
_35 = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_29 = '\u{6fd7e}';
place!(Field::<*const char>(Variant(_25.fld1.fld3, 0), 1)) = core::ptr::addr_of!(_29);
_33.0 = _3.0;
_11 = _3.0 as u8;
_17 = [_25.fld1.fld1,_25.fld1.fld1];
_38 = _27;
_25.fld4 = 3905509523_u32 | 1379530980_u32;
RET = [_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0,_25.fld1.fld0];
_37 = _33.0;
_28 = [_38,_27,_27,_27,_38,_38,_27];
Call(_25.fld4 = fn12(_7, _3.0, _37, _31, _18, _10, _3.0, _15, _3.0, _15, _25.fld1.fld2, _3.0, _3.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_16 = _24 * _24;
_24 = _25.fld1.fld1 as f64;
_25.fld0 = _2;
_25.fld2.1 = 3_usize as f32;
_11 = 127_u8;
_25.fld1.fld2 = [_11,_11,_11,_11];
_25.fld1.fld3 = Adt47::Variant0 { fld0: 15642170473707919813_u64,fld1: _25.fld2.0 };
_29 = '\u{751a1}';
_31 = Adt49 { fld0: _16 };
_36 = !_27;
Goto(bb13)
}
bb13 = {
place!(Field::<u64>(Variant(_25.fld1.fld3, 0), 0)) = 5142671128670778178_u64;
_25.fld6 = -_25.fld1.fld1;
place!(Field::<u64>(Variant(_25.fld1.fld3, 0), 0)) = 5234896095139968791_u64 - 8111107548071430789_u64;
_25.fld1.fld0 = -(-22770_i16);
_37 = _3.0 << _3.0;
_40 = [_11,_11,_11,_11,_11,_11,_11];
_33.3 = [_23];
_3.0 = _27 as u128;
place!(Field::<u64>(Variant(_25.fld1.fld3, 0), 0)) = 11456399633189684006_u64;
_33.2 = &_13;
_1 = -_4;
_30 = [_11,_11,_11,_11,_11,_11,_11];
_32 = !_23;
_34 = [_21,_21,_21,_21,_21];
_4 = _1;
_34 = [_21,_21,_21,_21,_21];
_43 = Adt49 { fld0: _31.fld0 };
_25.fld0 = [_11,_11,_11,_11];
match Field::<u64>(Variant(_25.fld1.fld3, 0), 0) {
0 => bb8,
1 => bb2,
2 => bb12,
3 => bb14,
11456399633189684006 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_24 = _3.1 as f64;
_20 = [4972834539383841995_i64,(-4228424301685267056_i64)];
RET = [(-26648_i16),(-2625_i16),(-31718_i16),(-6029_i16),(-22812_i16),32235_i16];
_23 = _9 & _9;
_23 = _9 >> _11;
_23 = _12;
_21 = _8 as u16;
_11 = 153_u8;
_11 = !247_u8;
_2 = _13;
_5 = _13;
_17 = [5206462961916728633_i64,2162413451463181962_i64];
_18 = [4759907922985037920_usize,138836307245401577_usize,6_usize,11624149665345769829_usize,3_usize,3666886238153390736_usize,5376834035986129540_usize,0_usize];
_21 = _8 as u16;
_15 = [12785122406990390102_usize,1_usize,3_usize,7_usize,1_usize,14163154342189858983_usize,10031125080249320414_usize,15292254568063392945_usize];
_6 = [_11,_11,_11,_11];
_3.0 = _22;
_8 = _3.1;
_23 = -_12;
_25.fld1.fld0 = (-26876_i16);
_24 = _11 as f64;
_18 = _15;
_10 = [_23];
_13 = [_11,_11,_11,_11];
_20 = [6805238757086012011_i64,(-7742774037223440117_i64)];
_3.0 = _22;
Call(_25.fld1.fld3 = fn9(_8, _7, _18, _3.0, _3, _7, _8, _7, _4, _18, _15, _3.0, _8, _5, _6, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb16 = {
place!(Field::<u64>(Variant(_25.fld1.fld3, 0), 0)) = !15344227784644244571_u64;
_3.0 = _37 ^ _33.0;
_25.fld1.fld3 = Adt47::Variant0 { fld0: 8382592060870615047_u64,fld1: _25.fld2.0 };
_39 = [_21,_21,_21,_21,_21];
_36 = _11 < _11;
_24 = _25.fld6 as f64;
_28 = [_27,_27,_27,_36,_36,_36,_36];
_25.fld6 = !_25.fld1.fld1;
_42 = [_11,_11,_11,_11,_11,_11,_11];
_25.fld1.fld1 = _25.fld6;
_14 = [_11,_11,_11,_11];
_8 = _1;
_13 = [_11,_11,_11,_11];
_2 = [_11,_11,_11,_11];
_14 = _5;
_24 = _3.0 as f64;
_43 = Adt49 { fld0: _24 };
_3.0 = _25.fld2.1 as u128;
Goto(bb17)
}
bb17 = {
Call(_46 = dump_var(8_usize, 4_usize, Move(_4), 7_usize, Move(_7), 27_usize, Move(_27), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(8_usize, 14_usize, Move(_14), 23_usize, Move(_23), 37_usize, Move(_37), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(8_usize, 2_usize, Move(_2), 39_usize, Move(_39), 22_usize, Move(_22), 35_usize, Move(_35)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_46 = dump_var(8_usize, 40_usize, Move(_40), 10_usize, Move(_10), 11_usize, Move(_11), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_46 = dump_var(8_usize, 42_usize, Move(_42), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i8,mut _2: [usize; 8],mut _3: [usize; 8],mut _4: u128,mut _5: (u128, i8),mut _6: [usize; 8],mut _7: i8,mut _8: [usize; 8],mut _9: i8,mut _10: [usize; 8],mut _11: [usize; 8],mut _12: u128,mut _13: i8,mut _14: [u8; 4],mut _15: [u8; 4],mut _16: [usize; 8]) -> Adt47 {
mir! {
type RET = Adt47;
let _17: i32;
let _18: Adt43;
let _19: i128;
let _20: isize;
let _21: [bool; 7];
let _22: f64;
let _23: isize;
let _24: u8;
let _25: f32;
let _26: i16;
let _27: (*const char, isize);
let _28: Adt51;
let _29: [u16; 5];
let _30: i128;
let _31: usize;
let _32: i16;
let _33: [isize; 1];
let _34: u8;
let _35: u8;
let _36: f32;
let _37: Adt48;
let _38: isize;
let _39: f64;
let _40: &'static [u8; 4];
let _41: char;
let _42: [bool; 7];
let _43: char;
let _44: &'static [u8; 4];
let _45: u128;
let _46: [u128; 1];
let _47: [usize; 8];
let _48: i32;
let _49: isize;
let _50: (*const char, isize);
let _51: (*mut f64, *const char, u32, f32, u128);
let _52: u8;
let _53: Adt48;
let _54: [u8; 4];
let _55: ();
let _56: ();
{
_16 = [5_usize,5_usize,1_usize,0_usize,11945848253410765410_usize,8159398401522586984_usize,1_usize,333069826170061066_usize];
_15 = _14;
_3 = _16;
_7 = 11263756799266050617_u64 as i8;
_18.fld6 = 35_u8 >> _5.0;
_5.1 = 47512_u16 as i8;
_18.fld5 = 8303080257032054404_i64 as usize;
_4 = _12 ^ _5.0;
_18.fld0 = [9223372036854775807_isize];
_18.fld2 = 2747293222_u32 ^ 2024329899_u32;
_3 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_9 = _1 | _5.1;
_4 = 156798334372343109244784857882383580634_i128 as u128;
_17 = _1 as i32;
_19 = 22173506114352964209098426460138474619_i128;
_5 = (_12, _13);
_20 = 9223372036854775807_isize * 9223372036854775807_isize;
_3 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_1 = _13;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211360 => bb8,
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
_4 = !_5.0;
_11 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_1 = !_13;
_8 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_16 = _10;
_14 = _15;
match _5.0 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
306015705847892542935096682290158742457 => bb14,
_ => bb13
}
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
Return()
}
bb14 = {
_13 = _5.1;
_14 = _15;
_12 = _4 ^ _4;
_18.fld3 = _18.fld2 as u64;
_2 = _10;
_18.fld3 = 380533323759494898_u64 << _5.0;
_10 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
Goto(bb15)
}
bb15 = {
_21 = [false,false,true,false,true,true,true];
_15 = _14;
_18.fld2 = _13 as u32;
_6 = _2;
Call(_18.fld5 = core::intrinsics::transmute(_18.fld3), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_22 = (-2318058753941338762_i64) as f64;
_23 = !_20;
_18.fld4 = _17 as i16;
_1 = _9 + _9;
_10 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_11 = _6;
_4 = !_12;
_3 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_19 = 112101807617811600908461284267378355070_i128;
_1 = _5.1 + _9;
_19 = 8509191717970245447672261916064125453_i128;
_5.0 = _12 | _4;
_14 = [_18.fld6,_18.fld6,_18.fld6,_18.fld6];
_15 = [_18.fld6,_18.fld6,_18.fld6,_18.fld6];
_8 = _6;
_16 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_18.fld0 = [_23];
_18.fld5 = 7_usize;
_22 = _23 as f64;
_18.fld0 = [_20];
_24 = _18.fld6 - _18.fld6;
_5 = (_4, _1);
_25 = _18.fld6 as f32;
_2 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_18.fld0 = [_23];
_18.fld0 = [_23];
_13 = _22 as i8;
Goto(bb17)
}
bb17 = {
_26 = _22 as i16;
_6 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_5.1 = _1 + _1;
_3 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_5.1 = -_1;
_8 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_18.fld6 = _24 << _24;
_20 = _23;
_14 = [_24,_18.fld6,_18.fld6,_18.fld6];
_18.fld5 = 6617215651115733361_usize;
_17 = (-1585706808_i32) << _13;
_5.1 = _9 - _13;
_18.fld3 = 15716110081843811049_u64;
Goto(bb18)
}
bb18 = {
_5.0 = _4 + _12;
_24 = _18.fld6;
_14 = [_18.fld6,_24,_24,_24];
_22 = _24 as f64;
_29 = [56908_u16,22724_u16,63265_u16,54941_u16,19566_u16];
_18.fld5 = 5_usize & 6_usize;
_18.fld3 = 4131347105993590552_u64;
_15 = [_24,_18.fld6,_24,_18.fld6];
_24 = !_18.fld6;
_25 = _17 as f32;
_29 = [7900_u16,28083_u16,16787_u16,31042_u16,56994_u16];
_2 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_23 = _20;
_32 = _12 as i16;
_30 = _19;
_16 = _8;
_27.1 = !_20;
_7 = !_13;
_3 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_13 = _1;
_14 = [_18.fld6,_18.fld6,_24,_18.fld6];
_31 = _18.fld5;
_27.1 = _17 as isize;
_16 = [_31,_18.fld5,_18.fld5,_18.fld5,_31,_18.fld5,_18.fld5,_31];
_32 = _18.fld4 << _5.0;
Goto(bb19)
}
bb19 = {
_32 = -_18.fld4;
_32 = _18.fld4 >> _4;
_9 = 25342_u16 as i8;
Goto(bb20)
}
bb20 = {
_34 = _18.fld6;
_4 = !_5.0;
_34 = !_18.fld6;
_29 = [50508_u16,3700_u16,27706_u16,62389_u16,60656_u16];
_16 = [_18.fld5,_31,_31,_31,_31,_31,_18.fld5,_31];
_18.fld4 = _32;
_3 = _2;
_16 = _2;
_29 = [23337_u16,16539_u16,60683_u16,59526_u16,30236_u16];
_18.fld6 = !_24;
_5 = (_12, _1);
_34 = _24;
_18.fld2 = 1841980591_u32;
_18.fld6 = !_34;
_18.fld0 = [_27.1];
_22 = _12 as f64;
_34 = _24;
_36 = _25 * _25;
match _18.fld3 {
0 => bb15,
4131347105993590552 => bb21,
_ => bb3
}
}
bb21 = {
_5 = (_12, _7);
_29 = [9685_u16,4939_u16,602_u16,27329_u16,48854_u16];
_12 = _17 as u128;
_22 = _18.fld4 as f64;
match _18.fld3 {
0 => bb12,
1 => bb22,
4131347105993590552 => bb24,
_ => bb23
}
}
bb22 = {
Return()
}
bb23 = {
Return()
}
bb24 = {
_28 = Adt51::Variant2 { fld0: _29,fld1: _5.1 };
_14 = _15;
_18.fld0 = [_27.1];
place!(Field::<[u16; 5]>(Variant(_28, 2), 0)) = [47420_u16,23841_u16,32472_u16,38984_u16,15197_u16];
_31 = _18.fld5;
_5 = (_12, _1);
_25 = _30 as f32;
_39 = -_22;
_2 = [_31,_18.fld5,_18.fld5,_31,_18.fld5,_31,_31,_18.fld5];
_30 = _19;
_7 = _22 as i8;
_11 = [_31,_18.fld5,_31,_31,_31,_18.fld5,_31,_31];
_19 = _30;
_18.fld5 = !_31;
_11 = [_31,_31,_31,_18.fld5,_18.fld5,_31,_31,_18.fld5];
SetDiscriminant(_28, 1);
_25 = _36;
Goto(bb25)
}
bb25 = {
_5.1 = -_1;
_20 = _27.1;
_38 = -_20;
_39 = _22 * _22;
_45 = _4;
_15 = [_24,_24,_34,_18.fld6];
_27.0 = core::ptr::addr_of!(_41);
_39 = _22 + _22;
_5 = (_4, _7);
Call(_13 = fn10(_25, _20, _5.0, _7, _18.fld6, _27, _39), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
place!(Field::<i128>(Variant(_28, 1), 4)) = _19;
_42 = [false,true,false,false,true,false,true];
_22 = _39 + _39;
_43 = '\u{8d7f7}';
Goto(bb27)
}
bb27 = {
_10 = _3;
_25 = -_36;
_41 = _43;
_32 = _18.fld4 & _18.fld4;
_35 = !_24;
_27.0 = core::ptr::addr_of!(place!(Field::<char>(Variant(_28, 1), 1)));
_14 = [_35,_34,_35,_35];
_18.fld4 = _32 + _26;
_30 = Field::<i128>(Variant(_28, 1), 4);
_5.1 = -_13;
_48 = _17 << _45;
match Field::<i128>(Variant(_28, 1), 4) {
0 => bb7,
1 => bb8,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
6 => bb32,
8509191717970245447672261916064125453 => bb34,
_ => bb33
}
}
bb28 = {
place!(Field::<i128>(Variant(_28, 1), 4)) = _19;
_42 = [false,true,false,false,true,false,true];
_22 = _39 + _39;
_43 = '\u{8d7f7}';
Goto(bb27)
}
bb29 = {
_22 = (-2318058753941338762_i64) as f64;
_23 = !_20;
_18.fld4 = _17 as i16;
_1 = _9 + _9;
_10 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_11 = _6;
_4 = !_12;
_3 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_19 = 112101807617811600908461284267378355070_i128;
_1 = _5.1 + _9;
_19 = 8509191717970245447672261916064125453_i128;
_5.0 = _12 | _4;
_14 = [_18.fld6,_18.fld6,_18.fld6,_18.fld6];
_15 = [_18.fld6,_18.fld6,_18.fld6,_18.fld6];
_8 = _6;
_16 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_18.fld0 = [_23];
_18.fld5 = 7_usize;
_22 = _23 as f64;
_18.fld0 = [_20];
_24 = _18.fld6 - _18.fld6;
_5 = (_4, _1);
_25 = _18.fld6 as f32;
_2 = [_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5,_18.fld5];
_18.fld0 = [_23];
_18.fld0 = [_23];
_13 = _22 as i8;
Goto(bb17)
}
bb30 = {
Return()
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
Return()
}
bb34 = {
_47 = [_31,_18.fld5,_31,_18.fld5,_31,_31,_31,_31];
_51.0 = core::ptr::addr_of_mut!(_39);
_51.4 = _13 as u128;
_39 = 4369585439402579808_i64 as f64;
_50.0 = core::ptr::addr_of!(place!(Field::<char>(Variant(_28, 1), 1)));
_45 = !_4;
_11 = [_18.fld5,_31,_18.fld5,_31,_18.fld5,_31,_31,_18.fld5];
_38 = _22 as isize;
_25 = _22 as f32;
_11 = _16;
_31 = _18.fld5;
_53 = Adt48::Variant0 { fld0: _32 };
place!(Field::<Adt48>(Variant(_28, 1), 2)) = Adt48::Variant0 { fld0: _18.fld4 };
_38 = !_20;
RET = Adt47::Variant0 { fld0: _18.fld3,fld1: _27.0 };
Goto(bb35)
}
bb35 = {
Call(_55 = dump_var(9_usize, 48_usize, Move(_48), 41_usize, Move(_41), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_55 = dump_var(9_usize, 21_usize, Move(_21), 43_usize, Move(_43), 15_usize, Move(_15), 20_usize, Move(_20)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_55 = dump_var(9_usize, 24_usize, Move(_24), 6_usize, Move(_6), 26_usize, Move(_26), 1_usize, Move(_1)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_55 = dump_var(9_usize, 34_usize, Move(_34), 13_usize, Move(_13), 4_usize, Move(_4), 30_usize, Move(_30)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_55 = dump_var(9_usize, 29_usize, Move(_29), 11_usize, Move(_11), 56_usize, _56, 56_usize, _56), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: f32,mut _2: isize,mut _3: u128,mut _4: i8,mut _5: u8,mut _6: (*const char, isize),mut _7: f64) -> i8 {
mir! {
type RET = i8;
let _8: Adt45;
let _9: Adt58;
let _10: char;
let _11: Adt49;
let _12: [i16; 6];
let _13: isize;
let _14: Adt44;
let _15: (*const char, f32);
let _16: (*mut f64, *const char, u32, f32, u128);
let _17: i8;
let _18: char;
let _19: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32));
let _20: ((*const char, f32),);
let _21: (*mut f64, *const char, u32, f32, u128);
let _22: [u8; 7];
let _23: [bool; 7];
let _24: (*const char, isize);
let _25: u16;
let _26: Adt43;
let _27: (u128, i8);
let _28: ();
let _29: ();
{
_2 = true as isize;
Goto(bb1)
}
bb1 = {
_7 = 9613315007734824798_u64 as f64;
_7 = _3 as f64;
RET = _1 as i8;
_5 = _7 as u8;
_2 = -_6.1;
_7 = 15862934266269723175_u64 as f64;
_2 = _6.1;
_1 = 2533415157_u32 as f32;
_4 = RET;
_4 = RET + RET;
_4 = RET;
_5 = 185_u8 >> _3;
_5 = !108_u8;
_7 = (-8775164627636837489_i64) as f64;
RET = _4 << _2;
RET = _4;
_6.0 = core::ptr::addr_of!(_10);
_11 = Adt49 { fld0: _7 };
Call(RET = fn11(_6.1, _4, _6, _3, _3, _2, _3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6.0 = core::ptr::addr_of!(_10);
_6.1 = 5471535295458341408_i64 as isize;
RET = 30478_u16 as i8;
_10 = '\u{9b8ef}';
RET = -_4;
RET = -_4;
_1 = (-376872232_i32) as f32;
RET = -_4;
_10 = '\u{7583b}';
_6.1 = -_2;
_1 = _5 as f32;
_2 = RET as isize;
_6.0 = core::ptr::addr_of!(_10);
_13 = _6.1;
_11 = Adt49 { fld0: _7 };
_12 = [2300_i16,1318_i16,5220_i16,20785_i16,10755_i16,224_i16];
RET = _7 as i8;
_3 = 20125476404025076276871244068269530626_u128 & 179047904646143019680737746899492660789_u128;
_1 = _2 as f32;
_13 = _2;
RET = !_4;
Goto(bb3)
}
bb3 = {
_5 = 146_u8;
_10 = '\u{1c66d}';
_7 = -_11.fld0;
_6.0 = core::ptr::addr_of!(_10);
RET = _3 as i8;
RET = _4;
RET = -_4;
_4 = RET - RET;
_15.1 = _5 as f32;
_5 = 23993_u16 as u8;
_15.1 = _1 * _1;
Goto(bb4)
}
bb4 = {
_12 = [9635_i16,(-25793_i16),(-25232_i16),(-26652_i16),1381_i16,178_i16];
_18 = _10;
_12 = [15159_i16,(-15669_i16),24803_i16,17218_i16,(-30123_i16),13308_i16];
_15.0 = _6.0;
_16.0 = core::ptr::addr_of_mut!(_11.fld0);
_19.5 = -_15.1;
_19.5 = _1;
_19.3.0 = _12;
_19.1 = 15081856374775829108_usize as i128;
Call(_6.1 = core::intrinsics::bswap(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = -_19.5;
_16.3 = 7880819826169443775_i64 as f32;
_19.7.1 = _3 as f32;
_19.7.1 = _19.5 * _15.1;
_15.1 = 5_usize as f32;
_19.7.0 = _15.0;
_6.0 = _19.7.0;
RET = _4;
_15.1 = _19.5;
RET = _18 as i8;
_20.0.1 = _3 as f32;
_19.6 = _10;
_21.0 = _16.0;
_15.1 = _19.7.1;
_3 = _5 as u128;
_19.7 = (_15.0, _15.1);
_19.3.1 = [_5,_5,_5,_5];
Goto(bb6)
}
bb6 = {
_19.2 = (-15266_i16);
_1 = _19.5;
_16.4 = _4 as u128;
_21.3 = _1;
_19.7.0 = _15.0;
_1 = _19.7.1;
_19.6 = _18;
_12 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_15.1 = -_19.7.1;
_15.1 = _19.7.1;
_19.7.0 = _6.0;
_20 = (_19.7,);
_16 = (_21.0, _6.0, 2628217369_u32, _19.7.1, _3);
_18 = _19.6;
_15.1 = _16.3 - _20.0.1;
RET = _4 + _4;
_13 = _2 << RET;
_19.6 = _10;
_21.4 = _19.7.1 as u128;
_19.3.4 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_24.1 = _13 - _13;
_21.2 = _10 as u32;
_26.fld1 = core::ptr::addr_of_mut!(_19.3.4);
Goto(bb7)
}
bb7 = {
Call(_28 = dump_var(10_usize, 12_usize, Move(_12), 10_usize, Move(_10), 18_usize, Move(_18), 3_usize, Move(_3)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: i8,mut _3: (*const char, isize),mut _4: u128,mut _5: u128,mut _6: isize,mut _7: u128,mut _8: i8) -> i8 {
mir! {
type RET = i8;
let _9: Adt49;
let _10: f64;
let _11: u128;
let _12: [bool; 7];
let _13: f32;
let _14: [u8; 4];
let _15: f64;
let _16: bool;
let _17: ();
let _18: ();
{
_8 = (-1987717076_i32) as i8;
RET = !_2;
_3.1 = 420468771_i32 as isize;
_4 = (-1832811392744232677_i64) as u128;
_6 = -_3.1;
_5 = _7;
RET = !_8;
Goto(bb1)
}
bb1 = {
RET = _2 << _5;
_5 = !_7;
RET = -_2;
_8 = 76_u8 as i8;
_5 = !_7;
_8 = !_2;
_1 = 5686621956096680214_usize as isize;
Goto(bb2)
}
bb2 = {
_4 = _7;
_6 = (-4039_i16) as isize;
RET = !_8;
RET = _8;
_5 = _7 - _7;
_6 = _3.1;
_10 = (-45789587948258484794270873392235667095_i128) as f64;
Call(_3.1 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3.1 = _6;
_6 = 12732_u16 as isize;
_8 = _2 << _7;
_9 = Adt49 { fld0: _10 };
RET = _10 as i8;
RET = _8;
_4 = !_5;
RET = _8;
_9 = Adt49 { fld0: _10 };
_12 = [true,true,true,true,true,true,true];
_8 = RET;
_4 = _7 - _5;
Goto(bb4)
}
bb4 = {
Call(_17 = dump_var(11_usize, 2_usize, Move(_2), 8_usize, Move(_8), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [usize; 8],mut _2: u128,mut _3: u128,mut _4: Adt49,mut _5: [usize; 8],mut _6: [isize; 1],mut _7: u128,mut _8: [usize; 8],mut _9: u128,mut _10: [usize; 8],mut _11: [u8; 4],mut _12: u128,mut _13: i8) -> u32 {
mir! {
type RET = u32;
let _14: bool;
let _15: f64;
let _16: *mut [i16; 6];
let _17: Adt58;
let _18: (*mut f64, *const char, u32, f32, u128);
let _19: [i64; 2];
let _20: Adt55;
let _21: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]);
let _22: i64;
let _23: Adt49;
let _24: char;
let _25: *const char;
let _26: char;
let _27: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)));
let _28: i32;
let _29: Adt47;
let _30: *mut usize;
let _31: u128;
let _32: Adt48;
let _33: ();
let _34: ();
{
_7 = _12;
_15 = _4.fld0;
_6 = [7_isize];
_1 = [2717110842247485725_usize,16612030000432647248_usize,4150724990175895460_usize,6615197819900006519_usize,10002629276598376201_usize,7_usize,3_usize,7_usize];
_1 = _10;
_14 = _13 == _13;
_7 = _9 * _2;
_3 = _7 + _12;
_7 = !_3;
_4.fld0 = -_15;
_1 = [7_usize,1801065770717653974_usize,3_usize,1_usize,5_usize,948545430295981678_usize,4_usize,16731518357763279606_usize];
_1 = [3_usize,6479629450295342427_usize,3_usize,5_usize,17365031932094043283_usize,3_usize,3_usize,7_usize];
_9 = _12;
Goto(bb1)
}
bb1 = {
_15 = _4.fld0 + _4.fld0;
_8 = _5;
_20.fld2.1 = 7909085298416111949003099428636869895_i128 as f32;
RET = 1061041519_u32 | 4216263413_u32;
_21.1 = _11;
_2 = _7 * _3;
_18.0 = core::ptr::addr_of_mut!(_4.fld0);
_21.2 = [13945_i16,(-29456_i16),26867_i16,10258_i16,(-25488_i16),11870_i16];
_21.3 = !9223372036854775807_isize;
_20.fld6 = (-3451150874962813998_i64);
_20.fld4 = (-6880_i16) as u32;
_21.0 = [26127_i16,(-11301_i16),(-12286_i16),13524_i16,15552_i16,(-20744_i16)];
_20.fld4 = 241_u8 as u32;
match _20.fld6 {
340282366920938463459923456556805397458 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_1 = [624577103643076276_usize,5_usize,7_usize,1_usize,3_usize,1066642977769185385_usize,1_usize,16696738458052757538_usize];
_20.fld1.fld1 = (-22960_i16) as i64;
_14 = false;
_20.fld4 = 81209467171126038709349848431542890727_i128 as u32;
_15 = _4.fld0 * _4.fld0;
_12 = _15 as u128;
_21.2 = _21.0;
_10 = [4_usize,0_usize,5_usize,5_usize,4_usize,5385846905585406950_usize,12245472483914902031_usize,2_usize];
_14 = !true;
_23.fld0 = -_15;
_18.0 = core::ptr::addr_of_mut!(_4.fld0);
_20.fld1.fld0 = (-12235_i16);
_20.fld0 = _21.1;
_16 = core::ptr::addr_of_mut!(_21.4);
_13 = (-111_i8);
RET = !_20.fld4;
_7 = !_9;
_20.fld2.0 = core::ptr::addr_of!(_24);
_20.fld2.1 = _20.fld1.fld1 as f32;
_10 = [9696938093202689166_usize,5045799579063922891_usize,3_usize,3_usize,16688093274239083748_usize,5_usize,12619533251852887250_usize,0_usize];
Goto(bb4)
}
bb4 = {
_12 = _13 as u128;
_20.fld0 = [2_u8,6_u8,86_u8,126_u8];
_7 = !_12;
_19 = [_20.fld1.fld1,_20.fld1.fld1];
_21.4 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
Goto(bb5)
}
bb5 = {
_24 = '\u{55a1a}';
_25 = _20.fld2.0;
_14 = true;
_16 = core::ptr::addr_of_mut!(_21.2);
_23.fld0 = -_4.fld0;
_9 = !_3;
_18.4 = _2 >> _2;
_27.1.3.3 = _14 as isize;
_13 = _14 as i8;
_20.fld3 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
match _20.fld6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463459923456556805397458 => bb10,
_ => bb9
}
}
bb6 = {
_12 = _13 as u128;
_20.fld0 = [2_u8,6_u8,86_u8,126_u8];
_7 = !_12;
_19 = [_20.fld1.fld1,_20.fld1.fld1];
_21.4 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
Goto(bb5)
}
bb7 = {
_1 = [624577103643076276_usize,5_usize,7_usize,1_usize,3_usize,1066642977769185385_usize,1_usize,16696738458052757538_usize];
_20.fld1.fld1 = (-22960_i16) as i64;
_14 = false;
_20.fld4 = 81209467171126038709349848431542890727_i128 as u32;
_15 = _4.fld0 * _4.fld0;
_12 = _15 as u128;
_21.2 = _21.0;
_10 = [4_usize,0_usize,5_usize,5_usize,4_usize,5385846905585406950_usize,12245472483914902031_usize,2_usize];
_14 = !true;
_23.fld0 = -_15;
_18.0 = core::ptr::addr_of_mut!(_4.fld0);
_20.fld1.fld0 = (-12235_i16);
_20.fld0 = _21.1;
_16 = core::ptr::addr_of_mut!(_21.4);
_13 = (-111_i8);
RET = !_20.fld4;
_7 = !_9;
_20.fld2.0 = core::ptr::addr_of!(_24);
_20.fld2.1 = _20.fld1.fld1 as f32;
_10 = [9696938093202689166_usize,5045799579063922891_usize,3_usize,3_usize,16688093274239083748_usize,5_usize,12619533251852887250_usize,0_usize];
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
_15 = _4.fld0 + _4.fld0;
_8 = _5;
_20.fld2.1 = 7909085298416111949003099428636869895_i128 as f32;
RET = 1061041519_u32 | 4216263413_u32;
_21.1 = _11;
_2 = _7 * _3;
_18.0 = core::ptr::addr_of_mut!(_4.fld0);
_21.2 = [13945_i16,(-29456_i16),26867_i16,10258_i16,(-25488_i16),11870_i16];
_21.3 = !9223372036854775807_isize;
_20.fld6 = (-3451150874962813998_i64);
_20.fld4 = (-6880_i16) as u32;
_21.0 = [26127_i16,(-11301_i16),(-12286_i16),13524_i16,15552_i16,(-20744_i16)];
_20.fld4 = 241_u8 as u32;
match _20.fld6 {
340282366920938463459923456556805397458 => bb3,
_ => bb2
}
}
bb10 = {
_20.fld2.1 = 100_u8 as f32;
match _20.fld1.fld0 {
0 => bb11,
1 => bb12,
340282366920938463463374607431768199221 => bb14,
_ => bb13
}
}
bb11 = {
_12 = _13 as u128;
_20.fld0 = [2_u8,6_u8,86_u8,126_u8];
_7 = !_12;
_19 = [_20.fld1.fld1,_20.fld1.fld1];
_21.4 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
Goto(bb5)
}
bb12 = {
Return()
}
bb13 = {
_1 = [624577103643076276_usize,5_usize,7_usize,1_usize,3_usize,1066642977769185385_usize,1_usize,16696738458052757538_usize];
_20.fld1.fld1 = (-22960_i16) as i64;
_14 = false;
_20.fld4 = 81209467171126038709349848431542890727_i128 as u32;
_15 = _4.fld0 * _4.fld0;
_12 = _15 as u128;
_21.2 = _21.0;
_10 = [4_usize,0_usize,5_usize,5_usize,4_usize,5385846905585406950_usize,12245472483914902031_usize,2_usize];
_14 = !true;
_23.fld0 = -_15;
_18.0 = core::ptr::addr_of_mut!(_4.fld0);
_20.fld1.fld0 = (-12235_i16);
_20.fld0 = _21.1;
_16 = core::ptr::addr_of_mut!(_21.4);
_13 = (-111_i8);
RET = !_20.fld4;
_7 = !_9;
_20.fld2.0 = core::ptr::addr_of!(_24);
_20.fld2.1 = _20.fld1.fld1 as f32;
_10 = [9696938093202689166_usize,5045799579063922891_usize,3_usize,3_usize,16688093274239083748_usize,5_usize,12619533251852887250_usize,0_usize];
Goto(bb4)
}
bb14 = {
_21.2 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
_20.fld2.0 = core::ptr::addr_of!(_27.1.6);
_20.fld4 = RET;
_18.1 = core::ptr::addr_of!(_27.1.6);
_20.fld2.0 = _25;
_27.1.3.3 = !_21.3;
_15 = -_4.fld0;
_21.4 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
_18.3 = 1554995666_i32 as f32;
_27.1.3.1 = [190_u8,140_u8,125_u8,13_u8];
_27.1.3.2 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
_21.3 = 75_u8 as isize;
_27.1.3.0 = _21.4;
_27.1.5 = _20.fld2.1 + _20.fld2.1;
_20.fld2 = (_25, _27.1.5);
_20.fld2 = (_25, _18.3);
_21 = (_27.1.3.0, _27.1.3.1, _27.1.3.2, _27.1.3.3, _27.1.3.0);
(*_25) = '\u{104087}';
_27.0 = _20.fld2.0;
_11 = [169_u8,136_u8,147_u8,74_u8];
_21.2 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(12_usize, 14_usize, Move(_14), 10_usize, Move(_10), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(12_usize, 12_usize, Move(_12), 13_usize, Move(_13), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: f64,mut _2: isize,mut _3: [u8; 4],mut _4: [i16; 6],mut _5: [usize; 8],mut _6: [isize; 1]) -> Adt48 {
mir! {
type RET = Adt48;
let _7: Adt52;
let _8: *mut [i16; 6];
let _9: [isize; 1];
let _10: isize;
let _11: Adt46;
let _12: (u128, i8);
let _13: i128;
let _14: Adt49;
let _15: [i16; 6];
let _16: char;
let _17: isize;
let _18: ();
let _19: ();
{
_4 = [(-1036_i16),(-17213_i16),9743_i16,(-7929_i16),17952_i16,(-18288_i16)];
_4 = [29373_i16,(-7084_i16),11422_i16,(-8818_i16),6115_i16,(-10130_i16)];
_5 = [3_usize,7_usize,15819346412018985797_usize,17199281551697986440_usize,17006007897099811925_usize,5_usize,1096845840410235808_usize,0_usize];
_1 = 219731239491124963705844607762261221917_u128 as f64;
_8 = core::ptr::addr_of_mut!(_4);
_1 = 119818737088615579081264357293261140036_i128 as f64;
(*_8) = [32322_i16,26173_i16,25450_i16,9299_i16,(-16254_i16),16114_i16];
_1 = 32922_u16 as f64;
_8 = core::ptr::addr_of_mut!(_4);
(*_8) = [(-8550_i16),6116_i16,(-2696_i16),12485_i16,19026_i16,15431_i16];
_1 = 688586991_u32 as f64;
_1 = 1459358314418192466_i64 as f64;
RET = Adt48::Variant0 { fld0: (-32056_i16) };
Goto(bb1)
}
bb1 = {
_5 = [544159022498244451_usize,12953891835661005239_usize,0_usize,1752125521558553727_usize,2_usize,5_usize,8663197461002483439_usize,17860658716862797346_usize];
_12.1 = 51_i8;
Goto(bb2)
}
bb2 = {
_6 = [_2];
_9 = [_2];
_3 = [95_u8,206_u8,242_u8,176_u8];
(*_8) = [(-3627_i16),8777_i16,(-8501_i16),(-28247_i16),(-27052_i16),(-3047_i16)];
_12.1 = !(-85_i8);
_12 = (184561503986928707121913096651544727849_u128, 91_i8);
_6 = [_2];
_4 = [(-30234_i16),(-10813_i16),6603_i16,(-10888_i16),14937_i16,7248_i16];
_10 = _2 & _2;
RET = Adt48::Variant0 { fld0: 4508_i16 };
place!(Field::<i16>(Variant(RET, 0), 0)) = 18078_i16;
_8 = core::ptr::addr_of_mut!((*_8));
RET = Adt48::Variant0 { fld0: 29439_i16 };
(*_8) = [32620_i16,31445_i16,888_i16,16509_i16,31617_i16,(-29917_i16)];
(*_8) = [(-17475_i16),(-11802_i16),(-28772_i16),19355_i16,(-17414_i16),4851_i16];
_13 = -86097854010534508235730658655518148090_i128;
_1 = 19250_u16 as f64;
match _12.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
91 => bb9,
_ => bb8
}
}
bb3 = {
_5 = [544159022498244451_usize,12953891835661005239_usize,0_usize,1752125521558553727_usize,2_usize,5_usize,8663197461002483439_usize,17860658716862797346_usize];
_12.1 = 51_i8;
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
(*_8) = [15634_i16,(-32731_i16),(-28306_i16),(-3637_i16),8774_i16,19276_i16];
_6 = _9;
_8 = core::ptr::addr_of_mut!((*_8));
_4 = [(-28168_i16),(-3975_i16),(-19747_i16),(-27926_i16),(-32749_i16),(-8737_i16)];
match _12.1 {
91 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
(*_8) = [(-26697_i16),(-31189_i16),(-12844_i16),(-29756_i16),(-12981_i16),1768_i16];
match _12.0 {
0 => bb3,
1 => bb12,
184561503986928707121913096651544727849 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_6 = [_2];
_9 = [_2];
_3 = [95_u8,206_u8,242_u8,176_u8];
(*_8) = [(-3627_i16),8777_i16,(-8501_i16),(-28247_i16),(-27052_i16),(-3047_i16)];
_12.1 = !(-85_i8);
_12 = (184561503986928707121913096651544727849_u128, 91_i8);
_6 = [_2];
_4 = [(-30234_i16),(-10813_i16),6603_i16,(-10888_i16),14937_i16,7248_i16];
_10 = _2 & _2;
RET = Adt48::Variant0 { fld0: 4508_i16 };
place!(Field::<i16>(Variant(RET, 0), 0)) = 18078_i16;
_8 = core::ptr::addr_of_mut!((*_8));
RET = Adt48::Variant0 { fld0: 29439_i16 };
(*_8) = [32620_i16,31445_i16,888_i16,16509_i16,31617_i16,(-29917_i16)];
(*_8) = [(-17475_i16),(-11802_i16),(-28772_i16),19355_i16,(-17414_i16),4851_i16];
_13 = -86097854010534508235730658655518148090_i128;
_1 = 19250_u16 as f64;
match _12.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
91 => bb9,
_ => bb8
}
}
bb14 = {
_14 = Adt49 { fld0: _1 };
_12.0 = (-461933470_i32) as u128;
place!(Field::<i16>(Variant(RET, 0), 0)) = -8200_i16;
_13 = -(-45888349633617555626976183886934235061_i128);
RET = Adt48::Variant0 { fld0: (-32383_i16) };
_12 = (159596334664398911221576470475603340546_u128, (-45_i8));
place!(Field::<i16>(Variant(RET, 0), 0)) = -(-24835_i16);
SetDiscriminant(RET, 1);
_5 = [3485551851625632211_usize,9167675866105282224_usize,17378085654787266179_usize,7_usize,7_usize,6_usize,6_usize,215670683594433425_usize];
Call(_14.fld0 = core::intrinsics::transmute(_6), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_14 = Adt49 { fld0: _1 };
_9 = [_2];
(*_8) = [(-30467_i16),(-27885_i16),(-1490_i16),(-19426_i16),13130_i16,11167_i16];
place!(Field::<[u128; 1]>(Variant(RET, 1), 0)) = [_12.0];
_2 = -_10;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = [_2];
place!(Field::<[u128; 1]>(Variant(RET, 1), 0)) = [_12.0];
_2 = _10 << _12.0;
_14 = Adt49 { fld0: _1 };
_12.1 = 8_i8 + 101_i8;
_15 = [426_i16,12403_i16,(-9263_i16),29877_i16,18366_i16,19472_i16];
_12.0 = 1281468101672457064484229034666992797_u128;
_5 = [4_usize,1_usize,7035503844182883609_usize,2_usize,5_usize,6_usize,0_usize,4_usize];
(*_8) = [(-12220_i16),29998_i16,24832_i16,(-3364_i16),(-19033_i16),22762_i16];
(*_8) = [25849_i16,(-28557_i16),15155_i16,10007_i16,21778_i16,(-15989_i16)];
_4 = [11760_i16,28436_i16,(-13766_i16),(-24880_i16),(-29346_i16),(-24140_i16)];
(*_8) = [(-7701_i16),21276_i16,3825_i16,(-11063_i16),(-11776_i16),5693_i16];
_12.1 = 72_i8 << _2;
_15 = [31363_i16,(-8052_i16),(-15115_i16),24971_i16,(-19981_i16),(-5308_i16)];
(*_8) = [31552_i16,17545_i16,(-20115_i16),25137_i16,(-17886_i16),8420_i16];
Goto(bb16)
}
bb16 = {
Call(_18 = dump_var(13_usize, 9_usize, Move(_9), 15_usize, Move(_15), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_18 = dump_var(13_usize, 2_usize, Move(_2), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: char,mut _2: u64,mut _3: [isize; 1],mut _4: [u16; 5]) -> i64 {
mir! {
type RET = i64;
let _5: isize;
let _6: *mut [i16; 6];
let _7: u16;
let _8: isize;
let _9: [i16; 3];
let _10: f64;
let _11: [u16; 5];
let _12: [i64; 2];
let _13: [i16; 3];
let _14: Adt44;
let _15: [u8; 7];
let _16: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]);
let _17: ();
let _18: ();
{
RET = !4433007802054655654_i64;
_3 = [(-9223372036854775808_isize)];
_1 = '\u{26a95}';
Goto(bb1)
}
bb1 = {
_4 = [17999_u16,60214_u16,7629_u16,23494_u16,31824_u16];
_5 = (-9223372036854775808_isize);
_4 = [28161_u16,28075_u16,44606_u16,11119_u16,42059_u16];
Call(_3 = fn15(RET, _2, _2, _2, _2, _2, _2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !664073290527130229_i64;
_3 = [_5];
_3 = [_5];
_3 = [_5];
RET = 8610405130889771833_i64 - (-3807653921312465748_i64);
_2 = 2911204249777952231_u64 * 11705530331090798696_u64;
_2 = 15913677053965603507_u64;
_3 = [_5];
RET = (-1315898192730539504_i64);
_1 = '\u{1096ac}';
_3 = [_5];
_7 = 41829_u16 + 5716_u16;
_3 = [_5];
_3 = [_5];
_1 = '\u{61d30}';
RET = false as i64;
RET = _1 as i64;
RET = 1719091986_i32 as i64;
RET = 6040386802153852579_i64 ^ (-8068248351546538002_i64);
_7 = !10217_u16;
_5 = (-9223372036854775808_isize);
_8 = _5;
_8 = -_5;
match _5 {
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
_4 = [17999_u16,60214_u16,7629_u16,23494_u16,31824_u16];
_5 = (-9223372036854775808_isize);
_4 = [28161_u16,28075_u16,44606_u16,11119_u16,42059_u16];
Call(_3 = fn15(RET, _2, _2, _2, _2, _2, _2, _1), ReturnTo(bb2), UnwindUnreachable())
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
_2 = 3328094397564684680_u64;
_8 = 4513520380008713970_usize as isize;
RET = !7063735296886471375_i64;
_4 = [_7,_7,_7,_7,_7];
_2 = 5481847084085685858_u64;
_1 = '\u{37be}';
_12 = [RET,RET];
_9 = [(-6759_i16),15939_i16,(-8798_i16)];
_11 = [_7,_7,_7,_7,_7];
_9 = [(-4277_i16),22800_i16,14375_i16];
_3 = [_8];
_10 = _7 as f64;
_4 = _11;
_5 = !_8;
_2 = !14590827607411227265_u64;
_13 = _9;
_11 = [_7,_7,_7,_7,_7];
_10 = 3921204777_u32 as f64;
_12 = [RET,RET];
Call(_2 = fn16(_12, _13, _9, _9, _10, _13, _5, _4, _8, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = _8 ^ _8;
_1 = '\u{4e60c}';
_12 = [RET,RET];
_1 = '\u{bc639}';
Goto(bb11)
}
bb11 = {
_4 = [_7,_7,_7,_7,_7];
_12 = [RET,RET];
_1 = '\u{95be9}';
_1 = '\u{24c9b}';
Call(_2 = core::intrinsics::bswap(4523652074615916824_u64), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7 = !5569_u16;
RET = 928187434914868999_i64 & (-7840964699171616429_i64);
_1 = '\u{77097}';
_7 = 60283_u16 * 43298_u16;
_7 = 57871_u16 & 48560_u16;
_11 = _4;
_7 = !20764_u16;
RET = (-4490681448237675775_i64);
_8 = 325105543216801860126152318119254380570_u128 as isize;
_4 = [_7,_7,_7,_7,_7];
_4 = [_7,_7,_7,_7,_7];
RET = 6_usize as i64;
RET = -(-4182187487525090229_i64);
_3 = [_5];
_5 = -_8;
_9 = [(-13147_i16),(-15208_i16),(-3119_i16)];
_1 = '\u{5e50e}';
RET = (-3822966156272648579_i64);
_4 = _11;
_12 = [RET,RET];
_13 = [(-29506_i16),(-28825_i16),(-5924_i16)];
_15 = [79_u8,135_u8,198_u8,164_u8,156_u8,115_u8,154_u8];
_4 = _11;
RET = 8097774106394792_i64;
_9 = [1816_i16,(-2178_i16),(-30709_i16)];
_2 = _5 as u64;
_11 = [_7,_7,_7,_7,_7];
_3 = [_5];
_7 = 91227014364819854774817547634358228887_u128 as u16;
match RET {
0 => bb11,
1 => bb3,
2 => bb13,
3 => bb14,
8097774106394792 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_5 = _8 ^ _8;
_1 = '\u{4e60c}';
_12 = [RET,RET];
_1 = '\u{bc639}';
Goto(bb11)
}
bb15 = {
RET = !664073290527130229_i64;
_3 = [_5];
_3 = [_5];
_3 = [_5];
RET = 8610405130889771833_i64 - (-3807653921312465748_i64);
_2 = 2911204249777952231_u64 * 11705530331090798696_u64;
_2 = 15913677053965603507_u64;
_3 = [_5];
RET = (-1315898192730539504_i64);
_1 = '\u{1096ac}';
_3 = [_5];
_7 = 41829_u16 + 5716_u16;
_3 = [_5];
_3 = [_5];
_1 = '\u{61d30}';
RET = false as i64;
RET = _1 as i64;
RET = 1719091986_i32 as i64;
RET = 6040386802153852579_i64 ^ (-8068248351546538002_i64);
_7 = !10217_u16;
_5 = (-9223372036854775808_isize);
_8 = _5;
_8 = -_5;
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb16 = {
_13 = _9;
_12 = [RET,RET];
RET = 6270859795703778714_i64 - (-7760893279249136526_i64);
_5 = _8 & _8;
_10 = (-1381857054_i32) as f64;
Goto(bb17)
}
bb17 = {
Call(_17 = dump_var(14_usize, 7_usize, Move(_7), 5_usize, Move(_5), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_17 = dump_var(14_usize, 11_usize, Move(_11), 2_usize, Move(_2), 18_usize, _18, 18_usize, _18), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i64,mut _2: u64,mut _3: u64,mut _4: u64,mut _5: u64,mut _6: u64,mut _7: u64,mut _8: char) -> [isize; 1] {
mir! {
type RET = [isize; 1];
let _9: u16;
let _10: char;
let _11: [u16; 5];
let _12: bool;
let _13: [i64; 2];
let _14: [u8; 4];
let _15: f32;
let _16: isize;
let _17: isize;
let _18: (*mut f64, *const char, u32, f32, u128);
let _19: [u128; 1];
let _20: Adt55;
let _21: f64;
let _22: Adt48;
let _23: Adt57;
let _24: *const i8;
let _25: bool;
let _26: u64;
let _27: Adt48;
let _28: u64;
let _29: i128;
let _30: [u8; 7];
let _31: Adt49;
let _32: (u128, i8);
let _33: [i64; 2];
let _34: isize;
let _35: f64;
let _36: [u128; 1];
let _37: [i16; 2];
let _38: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)));
let _39: isize;
let _40: f32;
let _41: ();
let _42: ();
{
_2 = !_7;
_8 = '\u{11778}';
_7 = !_3;
_9 = !51705_u16;
_5 = !_6;
_5 = !_4;
_8 = '\u{3debb}';
_7 = _6 & _4;
_3 = _5 >> _7;
_1 = (-8125424810616025136_i64) - (-3644610618089248579_i64);
RET = [(-9223372036854775808_isize)];
_8 = '\u{73d1e}';
_7 = _3;
_9 = 6497_u16;
RET = [9223372036854775807_isize];
_4 = !_6;
_2 = !_5;
_9 = 40801_u16;
_8 = '\u{4774e}';
RET = [9223372036854775807_isize];
_9 = !27074_u16;
_8 = '\u{71013}';
Goto(bb1)
}
bb1 = {
_5 = !_6;
RET = [(-9223372036854775808_isize)];
_3 = _2 + _2;
_7 = _2 - _3;
_3 = !_2;
_5 = _6;
_9 = _1 as u16;
_6 = !_5;
_10 = _8;
_5 = _7;
_3 = _7;
_1 = 8724399662958819899_i64;
_3 = !_7;
_2 = _3 + _6;
_4 = _7;
_5 = !_7;
_6 = _3;
Call(_7 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _10;
_8 = _10;
_12 = !false;
_11 = [_9,_9,_9,_9,_9];
RET = [(-9223372036854775808_isize)];
_10 = _8;
_9 = 19683_u16;
_3 = _10 as u64;
_11 = [_9,_9,_9,_9,_9];
_12 = !false;
_2 = !_4;
_4 = !_2;
RET = [(-9223372036854775808_isize)];
RET = [106_isize];
_1 = !2850499369153773705_i64;
Goto(bb3)
}
bb3 = {
RET = [44_isize];
_15 = 1646152330_u32 as f32;
RET = [(-9_isize)];
_14 = [41_u8,33_u8,77_u8,5_u8];
RET = [9223372036854775807_isize];
match _9 {
0 => bb1,
19683 => bb4,
_ => bb2
}
}
bb4 = {
_10 = _8;
_13 = [_1,_1];
_7 = _4 - _4;
_1 = !(-8161139945944948580_i64);
_6 = 103_u8 as u64;
_13 = [_1,_1];
_13 = [_1,_1];
_1 = 4991917739804759176_i64;
_2 = _4 & _3;
_1 = !5366785143624232375_i64;
_14 = [145_u8,77_u8,68_u8,147_u8];
_6 = _5 * _2;
_9 = !42917_u16;
_6 = 1877702437_i32 as u64;
_5 = !_3;
RET = [(-9223372036854775808_isize)];
_12 = !true;
Goto(bb5)
}
bb5 = {
_8 = _10;
_11 = [_9,_9,_9,_9,_9];
_6 = !_2;
_17 = 28_isize + 52_isize;
RET = [_17];
Call(_9 = core::intrinsics::bswap(62278_u16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = _6 & _2;
_16 = -_17;
_7 = 4_u8 as u64;
_13 = [_1,_1];
_5 = 17_u8 as u64;
_11 = [_9,_9,_9,_9,_9];
_16 = _17;
_1 = -3748847000107356380_i64;
RET = [_17];
_12 = _6 < _5;
_20.fld4 = !1783020486_u32;
_17 = _16;
_20.fld6 = _17 as i64;
_16 = !_17;
_7 = _8 as u64;
_12 = !true;
RET = [_16];
_21 = (-13382_i16) as f64;
_18.4 = 2469292540750379422_usize as u128;
_13 = [_20.fld6,_20.fld6];
_20.fld6 = !_1;
_5 = !_4;
_20.fld1.fld2 = [118_u8,6_u8,58_u8,90_u8];
_20.fld1.fld0 = !2140_i16;
Goto(bb7)
}
bb7 = {
_11 = [_9,_9,_9,_9,_9];
_18.2 = !_20.fld4;
_15 = _17 as f32;
_1 = -_20.fld6;
_18.0 = core::ptr::addr_of_mut!(_21);
_10 = _8;
_11 = [_9,_9,_9,_9,_9];
_20.fld4 = _18.2 ^ _18.2;
_20.fld3 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
_18.1 = core::ptr::addr_of!(_10);
_10 = _8;
RET = [_16];
_3 = !_5;
_10 = _8;
_18.2 = _9 as u32;
_20.fld2.0 = _18.1;
Goto(bb8)
}
bb8 = {
_5 = !_6;
_11 = [_9,_9,_9,_9,_9];
_11 = [_9,_9,_9,_9,_9];
_20.fld0 = _14;
_20.fld4 = _18.2;
_19 = [_18.4];
_20.fld4 = _18.2;
_5 = _18.4 as u64;
_8 = _10;
_4 = !_7;
_21 = _18.2 as f64;
_14 = _20.fld0;
_6 = _2;
_3 = 7_usize as u64;
_17 = -_16;
_5 = _12 as u64;
_20.fld3 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
RET = [_17];
_4 = 102981886011070556719398383112554826123_i128 as u64;
_5 = _9 as u64;
_21 = _6 as f64;
_14 = [205_u8,191_u8,96_u8,73_u8];
RET = [_17];
_19 = [_18.4];
_18.2 = _20.fld4;
Call(_17 = core::intrinsics::bswap(_16), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_20.fld1.fld1 = !_20.fld6;
_6 = !_2;
_2 = _6 - _6;
_1 = _20.fld6 & _20.fld1.fld1;
_20.fld2 = (_18.1, _15);
_20.fld0 = _14;
_18.0 = core::ptr::addr_of_mut!(_21);
_25 = !_12;
_20.fld1.fld0 = 6109_i16;
_20.fld1.fld0 = -15868_i16;
_20.fld2 = (_18.1, _15);
_13 = [_1,_1];
_18.2 = _20.fld4;
_12 = _25;
RET = [_17];
_19 = [_18.4];
_20.fld2.0 = core::ptr::addr_of!(_8);
_20.fld1.fld0 = -(-8841_i16);
_19 = [_18.4];
_18.3 = _15;
_12 = _6 >= _2;
_20.fld1.fld1 = -_20.fld6;
_13 = [_20.fld6,_1];
_14 = [239_u8,156_u8,135_u8,142_u8];
_15 = _20.fld2.1;
_6 = _2 * _2;
_20.fld0 = _20.fld1.fld2;
_20.fld1.fld0 = -1715_i16;
_5 = _6 * _6;
Goto(bb10)
}
bb10 = {
_29 = (-90658751893623801469968824015545783659_i128) | 2741164677052024532780133068856784682_i128;
_20.fld4 = _21 as u32;
_27 = Adt48::Variant1 { fld0: _19 };
_20.fld2.1 = -_18.3;
_12 = !_25;
SetDiscriminant(_27, 1);
_31 = Adt49 { fld0: _21 };
_20.fld0 = [113_u8,166_u8,60_u8,233_u8];
place!(Field::<[u128; 1]>(Variant(_27, 1), 0)) = [_18.4];
_20.fld2.1 = -_15;
_28 = _2 | _5;
_30 = [235_u8,130_u8,241_u8,244_u8,228_u8,124_u8,149_u8];
_6 = !_28;
_19 = [_18.4];
_20.fld6 = _10 as i64;
_10 = _8;
_4 = _5 & _2;
_11 = [_9,_9,_9,_9,_9];
Goto(bb11)
}
bb11 = {
_18.3 = _15 + _15;
_6 = _15 as u64;
_18.4 = 168037381755811012047720883738804886814_u128 * 305540901413241893837451092764572358733_u128;
_22 = Move(_27);
_15 = _20.fld2.1 * _18.3;
SetDiscriminant(_22, 1);
Goto(bb12)
}
bb12 = {
_8 = _10;
_26 = _4 * _5;
_13 = [_1,_20.fld6];
Call(_21 = core::intrinsics::fmaf64(_31.fld0, _31.fld0, _31.fld0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18.4 = _8 as u128;
_9 = !50141_u16;
_6 = _8 as u64;
_15 = _20.fld2.1 - _18.3;
_20.fld4 = (-2093030431_i32) as u32;
_18.1 = _20.fld2.0;
_31.fld0 = _18.4 as f64;
_12 = _4 < _4;
_33 = [_20.fld1.fld1,_20.fld6];
_13 = [_20.fld6,_1];
_31.fld0 = -_21;
_35 = _31.fld0 + _31.fld0;
_3 = _2;
_30 = [49_u8,226_u8,11_u8,177_u8,22_u8,45_u8,244_u8];
_20.fld3 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0];
_5 = !_2;
_2 = _20.fld1.fld0 as u64;
_20.fld1.fld0 = 10401_i16 - 12584_i16;
_22 = Adt48::Variant1 { fld0: _19 };
_20.fld1.fld1 = _1 | _20.fld6;
_17 = _16 + _16;
_38.1.2 = _18.4 as i16;
_18.1 = core::ptr::addr_of!(_38.1.6);
SetDiscriminant(_22, 0);
place!(Field::<i16>(Variant(_22, 0), 0)) = _12 as i16;
_37 = [Field::<i16>(Variant(_22, 0), 0),Field::<i16>(Variant(_22, 0), 0)];
Goto(bb14)
}
bb14 = {
_38.1.7.0 = core::ptr::addr_of!(_38.1.6);
place!(Field::<i16>(Variant(_22, 0), 0)) = -_20.fld1.fld0;
_18.1 = _20.fld2.0;
_28 = !_26;
_9 = 32159_u16;
_36 = [_18.4];
_38.1.3.2 = [Field::<i16>(Variant(_22, 0), 0),_20.fld1.fld0,_38.1.2,_20.fld1.fld0,_38.1.2,_38.1.2];
_38.1.3.0 = [_20.fld1.fld0,_20.fld1.fld0,_20.fld1.fld0,Field::<i16>(Variant(_22, 0), 0),_20.fld1.fld0,_38.1.2];
_32.0 = !_18.4;
_38.1.7.1 = 14158283328418479833_usize as f32;
_7 = _20.fld1.fld0 as u64;
_20.fld1.fld0 = -Field::<i16>(Variant(_22, 0), 0);
SetDiscriminant(_22, 0);
_38.1.1 = _29;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(15_usize, 3_usize, Move(_3), 37_usize, Move(_37), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(15_usize, 25_usize, Move(_25), 14_usize, Move(_14), 19_usize, Move(_19), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(15_usize, 1_usize, Move(_1), 13_usize, Move(_13), 29_usize, Move(_29), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i64; 2],mut _2: [i16; 3],mut _3: [i16; 3],mut _4: [i16; 3],mut _5: f64,mut _6: [i16; 3],mut _7: isize,mut _8: [u16; 5],mut _9: isize,mut _10: isize) -> u64 {
mir! {
type RET = u64;
let _11: [usize; 8];
let _12: isize;
let _13: isize;
let _14: [i16; 3];
let _15: [isize; 1];
let _16: bool;
let _17: i32;
let _18: *mut [i16; 6];
let _19: bool;
let _20: i32;
let _21: [i16; 2];
let _22: i128;
let _23: [i16; 3];
let _24: [u128; 1];
let _25: [usize; 8];
let _26: [i16; 6];
let _27: isize;
let _28: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32));
let _29: f64;
let _30: f32;
let _31: [i64; 2];
let _32: Adt43;
let _33: (u128, i8);
let _34: [isize; 1];
let _35: f64;
let _36: Adt49;
let _37: [usize; 8];
let _38: f32;
let _39: (u128, i8);
let _40: isize;
let _41: ();
let _42: ();
{
_10 = 131_u8 as isize;
_8 = [35675_u16,13190_u16,10578_u16,34411_u16,32109_u16];
RET = (-64_i8) as u64;
RET = 20655_i16 as u64;
_3 = [2086_i16,12017_i16,(-1801_i16)];
_10 = _9;
Goto(bb1)
}
bb1 = {
_1 = [(-6691414579493635108_i64),4654846364121491258_i64];
_8 = [62555_u16,48223_u16,60350_u16,62174_u16,16341_u16];
_11 = [3_usize,16808359063993967016_usize,7924581535420270655_usize,1_usize,3_usize,13295985395117878303_usize,7_usize,5723662454677768675_usize];
_8 = [25437_u16,23375_u16,54307_u16,46917_u16,47009_u16];
_11 = [7_usize,2_usize,5724604580130899716_usize,7_usize,1_usize,2_usize,4_usize,1_usize];
RET = 13903494058161096107_u64 - 16359910967030073769_u64;
_7 = _10 & _10;
_2 = [(-13993_i16),(-7334_i16),(-20651_i16)];
_1 = [8388166992093852791_i64,(-6566695324944751079_i64)];
_8 = [9510_u16,44130_u16,7176_u16,23738_u16,13981_u16];
_1 = [(-8733344978756646299_i64),2115943709923659396_i64];
_1 = [1834036987778666535_i64,3938555319752988797_i64];
_12 = 748101177_i32 as isize;
RET = 39740684322029604413617229078367356997_u128 as u64;
_4 = _3;
_13 = _10;
_9 = _12;
_15 = [_7];
Goto(bb2)
}
bb2 = {
_2 = [(-27043_i16),28705_i16,(-21416_i16)];
RET = (-57_i8) as u64;
_4 = [5593_i16,(-7383_i16),9501_i16];
_10 = _7 ^ _7;
_16 = RET != RET;
_10 = _13 * _13;
_2 = [(-17125_i16),(-9187_i16),(-11716_i16)];
_14 = [14416_i16,21663_i16,(-32577_i16)];
_1 = [(-8827878877315810136_i64),(-6878327900135154251_i64)];
_20 = _16 as i32;
_22 = 76354597535550796025428089974740211008_i128;
_9 = '\u{1192a}' as isize;
match _22 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
76354597535550796025428089974740211008 => bb10,
_ => bb9
}
}
bb3 = {
_1 = [(-6691414579493635108_i64),4654846364121491258_i64];
_8 = [62555_u16,48223_u16,60350_u16,62174_u16,16341_u16];
_11 = [3_usize,16808359063993967016_usize,7924581535420270655_usize,1_usize,3_usize,13295985395117878303_usize,7_usize,5723662454677768675_usize];
_8 = [25437_u16,23375_u16,54307_u16,46917_u16,47009_u16];
_11 = [7_usize,2_usize,5724604580130899716_usize,7_usize,1_usize,2_usize,4_usize,1_usize];
RET = 13903494058161096107_u64 - 16359910967030073769_u64;
_7 = _10 & _10;
_2 = [(-13993_i16),(-7334_i16),(-20651_i16)];
_1 = [8388166992093852791_i64,(-6566695324944751079_i64)];
_8 = [9510_u16,44130_u16,7176_u16,23738_u16,13981_u16];
_1 = [(-8733344978756646299_i64),2115943709923659396_i64];
_1 = [1834036987778666535_i64,3938555319752988797_i64];
_12 = 748101177_i32 as isize;
RET = 39740684322029604413617229078367356997_u128 as u64;
_4 = _3;
_13 = _10;
_9 = _12;
_15 = [_7];
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
_11 = [4_usize,12225802318269181928_usize,5857752592337571329_usize,7982915891997653098_usize,4_usize,6_usize,1_usize,6_usize];
_21 = [(-671_i16),27062_i16];
_23 = [15014_i16,(-5389_i16),24535_i16];
RET = 14570905254678071361_u64 & 3799857387459423881_u64;
_15 = [_12];
_6 = [12020_i16,(-122_i16),(-31509_i16)];
_6 = [7347_i16,24400_i16,(-18854_i16)];
_19 = _16;
_6 = [(-25079_i16),29387_i16,(-9419_i16)];
_22 = (-48022831417781407443678680904226770764_i128) | (-152949799057536991855943213996212462878_i128);
_13 = _10 & _10;
_2 = [(-26075_i16),3945_i16,(-9643_i16)];
_17 = _20;
_20 = _17 << _13;
_23 = _4;
Goto(bb11)
}
bb11 = {
_19 = !_16;
_23 = [18312_i16,10458_i16,(-5619_i16)];
_14 = [(-20757_i16),(-10930_i16),3968_i16];
_2 = [30342_i16,(-28390_i16),19813_i16];
_9 = 84_u8 as isize;
_11 = [6790609590701632162_usize,2_usize,6_usize,2_usize,5_usize,5_usize,0_usize,5_usize];
_26 = [18177_i16,13214_i16,(-3983_i16),(-6942_i16),23313_i16,(-17397_i16)];
_8 = [34591_u16,11677_u16,57373_u16,22762_u16,49514_u16];
RET = !11772061132708888278_u64;
_25 = [16920447460440374291_usize,14754609606548052255_usize,4_usize,4731005925054872351_usize,6_usize,4_usize,3221364129880108040_usize,1_usize];
_2 = [(-22586_i16),(-32_i16),6191_i16];
_4 = [(-20723_i16),(-13651_i16),(-25390_i16)];
_2 = [(-29179_i16),24178_i16,(-28152_i16)];
_2 = _4;
RET = _5 as u64;
_20 = !_17;
Goto(bb12)
}
bb12 = {
_13 = '\u{20a09}' as isize;
_8 = [11038_u16,49186_u16,53415_u16,30832_u16,31976_u16];
_28.3.3 = -_7;
_27 = _12;
_16 = _19;
Goto(bb13)
}
bb13 = {
_28.3.1 = [68_u8,27_u8,176_u8,155_u8];
_5 = _9 as f64;
_27 = _22 as isize;
_28.3.2 = [(-30024_i16),15354_i16,(-23135_i16),(-6576_i16),(-27876_i16),11716_i16];
_28.2 = (-11175_i16) | (-31784_i16);
_27 = !_10;
_28.0 = -(-74_i8);
_31 = [(-4018742698323073369_i64),2924461615568762192_i64];
_28.2 = -19095_i16;
_10 = !_9;
_19 = _7 > _27;
_28.7.1 = _22 as f32;
_28.2 = (-2200_i16);
_11 = [4_usize,4_usize,7208522947844339875_usize,7926972468428042380_usize,10366869897334023784_usize,1734134202206577884_usize,2_usize,7_usize];
_30 = _28.7.1;
_5 = 8298001283283559073_i64 as f64;
match _28.2 {
0 => bb11,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
340282366920938463463374607431768209256 => bb20,
_ => bb19
}
}
bb14 = {
Return()
}
bb15 = {
_19 = !_16;
_23 = [18312_i16,10458_i16,(-5619_i16)];
_14 = [(-20757_i16),(-10930_i16),3968_i16];
_2 = [30342_i16,(-28390_i16),19813_i16];
_9 = 84_u8 as isize;
_11 = [6790609590701632162_usize,2_usize,6_usize,2_usize,5_usize,5_usize,0_usize,5_usize];
_26 = [18177_i16,13214_i16,(-3983_i16),(-6942_i16),23313_i16,(-17397_i16)];
_8 = [34591_u16,11677_u16,57373_u16,22762_u16,49514_u16];
RET = !11772061132708888278_u64;
_25 = [16920447460440374291_usize,14754609606548052255_usize,4_usize,4731005925054872351_usize,6_usize,4_usize,3221364129880108040_usize,1_usize];
_2 = [(-22586_i16),(-32_i16),6191_i16];
_4 = [(-20723_i16),(-13651_i16),(-25390_i16)];
_2 = [(-29179_i16),24178_i16,(-28152_i16)];
_2 = _4;
RET = _5 as u64;
_20 = !_17;
Goto(bb12)
}
bb16 = {
_2 = [(-27043_i16),28705_i16,(-21416_i16)];
RET = (-57_i8) as u64;
_4 = [5593_i16,(-7383_i16),9501_i16];
_10 = _7 ^ _7;
_16 = RET != RET;
_10 = _13 * _13;
_2 = [(-17125_i16),(-9187_i16),(-11716_i16)];
_14 = [14416_i16,21663_i16,(-32577_i16)];
_1 = [(-8827878877315810136_i64),(-6878327900135154251_i64)];
_20 = _16 as i32;
_22 = 76354597535550796025428089974740211008_i128;
_9 = '\u{1192a}' as isize;
match _22 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
76354597535550796025428089974740211008 => bb10,
_ => bb9
}
}
bb17 = {
Return()
}
bb18 = {
_1 = [(-6691414579493635108_i64),4654846364121491258_i64];
_8 = [62555_u16,48223_u16,60350_u16,62174_u16,16341_u16];
_11 = [3_usize,16808359063993967016_usize,7924581535420270655_usize,1_usize,3_usize,13295985395117878303_usize,7_usize,5723662454677768675_usize];
_8 = [25437_u16,23375_u16,54307_u16,46917_u16,47009_u16];
_11 = [7_usize,2_usize,5724604580130899716_usize,7_usize,1_usize,2_usize,4_usize,1_usize];
RET = 13903494058161096107_u64 - 16359910967030073769_u64;
_7 = _10 & _10;
_2 = [(-13993_i16),(-7334_i16),(-20651_i16)];
_1 = [8388166992093852791_i64,(-6566695324944751079_i64)];
_8 = [9510_u16,44130_u16,7176_u16,23738_u16,13981_u16];
_1 = [(-8733344978756646299_i64),2115943709923659396_i64];
_1 = [1834036987778666535_i64,3938555319752988797_i64];
_12 = 748101177_i32 as isize;
RET = 39740684322029604413617229078367356997_u128 as u64;
_4 = _3;
_13 = _10;
_9 = _12;
_15 = [_7];
Goto(bb2)
}
bb19 = {
Return()
}
bb20 = {
_6 = [_28.2,_28.2,_28.2];
RET = 631657214486579456_u64 << _20;
_28.2 = (-5074_i16) * (-19180_i16);
_28.3.0 = [_28.2,_28.2,_28.2,_28.2,_28.2,_28.2];
_5 = RET as f64;
_32.fld5 = 7_usize;
_32.fld4 = _28.2;
_32.fld3 = _32.fld5 as u64;
_32.fld0 = _15;
_28.3.2 = _26;
_28.3.3 = _27 << RET;
Goto(bb21)
}
bb21 = {
Call(_41 = dump_var(16_usize, 12_usize, Move(_12), 15_usize, Move(_15), 31_usize, Move(_31), 27_usize, Move(_27)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_41 = dump_var(16_usize, 3_usize, Move(_3), 26_usize, Move(_26), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_41 = dump_var(16_usize, 2_usize, Move(_2), 10_usize, Move(_10), 21_usize, Move(_21), 1_usize, Move(_1)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: u32,mut _3: [usize; 8]) -> char {
mir! {
type RET = char;
let _4: [i16; 3];
let _5: usize;
let _6: isize;
let _7: isize;
let _8: (*mut f64, *const char, u32, f32, u128);
let _9: u8;
let _10: Adt44;
let _11: Adt55;
let _12: bool;
let _13: [u16; 5];
let _14: [u8; 4];
let _15: isize;
let _16: bool;
let _17: [isize; 1];
let _18: char;
let _19: bool;
let _20: i16;
let _21: Adt48;
let _22: f64;
let _23: [u8; 7];
let _24: isize;
let _25: [u128; 1];
let _26: Adt54;
let _27: bool;
let _28: u16;
let _29: bool;
let _30: u16;
let _31: char;
let _32: f32;
let _33: isize;
let _34: (u128, i8);
let _35: (*mut f64, *const char, u32, f32, u128);
let _36: (u128, i8);
let _37: [u16; 5];
let _38: ();
let _39: ();
{
RET = '\u{104ccb}';
RET = '\u{6bb9f}';
_3 = [0_usize,2_usize,1_usize,308233374598817503_usize,6_usize,4823289685296319817_usize,8919770976826695280_usize,7_usize];
_4 = [11084_i16,(-27409_i16),32364_i16];
RET = '\u{5b8df}';
_3 = [3_usize,1_usize,118281351259146835_usize,0_usize,6_usize,0_usize,1_usize,460486770918772771_usize];
RET = '\u{590e2}';
RET = '\u{c4b45}';
_4 = [(-7437_i16),6588_i16,12791_i16];
_4 = [17468_i16,(-30701_i16),32195_i16];
_6 = _1;
_5 = 9575588124820238543112195439487086012_i128 as usize;
_4 = [31581_i16,26960_i16,24953_i16];
_6 = _1;
_1 = 10846_i16 as isize;
RET = '\u{5186f}';
_1 = true as isize;
_4 = [(-20387_i16),29828_i16,(-24060_i16)];
_2 = !1093490262_u32;
_6 = _1;
_6 = _1 | _1;
_6 = !_1;
_4 = [8777_i16,17970_i16,22540_i16];
Goto(bb1)
}
bb1 = {
RET = '\u{4d422}';
_4 = [(-16629_i16),(-7083_i16),(-68_i16)];
RET = '\u{7064d}';
_2 = 2337768798_u32 + 1052371740_u32;
_2 = 4162092028_u32;
RET = '\u{6885b}';
_6 = _1 * _1;
RET = '\u{6ff77}';
_2 = !2032257669_u32;
RET = '\u{12eae}';
_7 = !_6;
_4 = [2476_i16,(-16558_i16),(-16214_i16)];
RET = '\u{949b5}';
Call(RET = fn18(_3, _7, _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [32477_i16,(-24044_i16),25141_i16];
_1 = !_7;
_7 = _1 - _6;
_4 = [4248_i16,27248_i16,(-20216_i16)];
_7 = -_1;
_6 = _7;
_11.fld1.fld1 = -(-998410226842648812_i64);
_5 = 1_usize;
_11.fld3[_5] = _4[_5] << _6;
_12 = !false;
_11.fld3[_5] = !_4[_5];
_8.2 = RET as u32;
RET = '\u{e2290}';
_11.fld1.fld0 = 1779381250_i32 as i16;
Goto(bb3)
}
bb3 = {
_11.fld0 = [170_u8,123_u8,113_u8,80_u8];
_11.fld2.0 = core::ptr::addr_of!(RET);
_11.fld3[_5] = 248413455316403736747978261138499888943_u128 as i16;
_11.fld6 = -_11.fld1.fld1;
_11.fld0[_5] = _11.fld1.fld1 as u8;
Goto(bb4)
}
bb4 = {
_11.fld6 = _11.fld1.fld1;
Goto(bb5)
}
bb5 = {
_8.4 = 201114735755030285906178415852983802619_u128 ^ 63162427724034365866427474117108425211_u128;
_13 = [51885_u16,36157_u16,61518_u16,6637_u16,54806_u16];
_9 = _13[_5] as u8;
_11.fld2.1 = 11937061884442803658_u64 as f32;
_11.fld1.fld3 = Adt47::Variant0 { fld0: 12516603094082275122_u64,fld1: _11.fld2.0 };
Goto(bb6)
}
bb6 = {
_13 = [16172_u16,62963_u16,13964_u16,31188_u16,56738_u16];
_11.fld0 = [_9,_9,_9,_9];
_8.1 = Field::<*const char>(Variant(_11.fld1.fld3, 0), 1);
_11.fld4 = _2 >> _11.fld0[_5];
_11.fld3 = [_4[_5],_4[_5],_4[_5]];
RET = '\u{118fc}';
_16 = _12 | _12;
_11.fld1.fld3 = Adt47::Variant0 { fld0: 9523243630874562880_u64,fld1: _8.1 };
_11.fld4 = !_2;
_11.fld6 = !_11.fld1.fld1;
RET = '\u{16dc1}';
_11.fld3[_5] = -_4[_5];
Goto(bb7)
}
bb7 = {
_11.fld0[_5] = _8.4 as u8;
_14 = [_11.fld0[_5],_9,_9,_9];
_11.fld1.fld2[_5] = _5 as u8;
_17 = [_6];
_11.fld3 = _4;
_11.fld1.fld2 = [_14[_5],_9,_14[_5],_14[_5]];
place!(Field::<*const char>(Variant(_11.fld1.fld3, 0), 1)) = _8.1;
_11.fld0[_5] = !_14[_5];
_11.fld0[_5] = !_11.fld1.fld2[_5];
_14 = [_11.fld1.fld2[_5],_9,_9,_9];
_1 = _6;
_2 = !_8.2;
_22 = (-95_i8) as f64;
RET = '\u{102b15}';
place!(Field::<u64>(Variant(_11.fld1.fld3, 0), 0)) = 3156321948602570299_u64 & 10488105946559546367_u64;
_11.fld0[_5] = _14[_5];
_16 = _13[_5] == _13[_5];
_21 = Adt48::Variant0 { fld0: _11.fld3[_5] };
_11.fld0[_5] = _14[_5] << _11.fld4;
_16 = _12;
_6 = _7 | _7;
_13[_5] = 59455_u16 + 13439_u16;
_8.0 = core::ptr::addr_of_mut!(_22);
_8.3 = _11.fld2.1 - _11.fld2.1;
_11.fld2.1 = _8.3;
_2 = _13[_5] as u32;
Goto(bb8)
}
bb8 = {
_11.fld3[_5] = !_4[_5];
_19 = _12;
_13[_5] = 37781_u16 ^ 19640_u16;
_23[_5] = _3[_5] as u8;
_6 = -_7;
_13 = [48287_u16,28565_u16,16893_u16,30262_u16,29977_u16];
place!(Field::<u64>(Variant(_11.fld1.fld3, 0), 0)) = _3[_5] as u64;
_8.3 = _2 as f32;
place!(Field::<*const char>(Variant(_11.fld1.fld3, 0), 1)) = core::ptr::addr_of!(RET);
place!(Field::<u64>(Variant(_11.fld1.fld3, 0), 0)) = 187051303909844856_u64 << _3[_5];
_23 = [_11.fld1.fld2[_5],_11.fld1.fld2[_5],_11.fld1.fld2[_5],_11.fld1.fld2[_5],_14[_5],_11.fld0[_5],_11.fld1.fld2[_5]];
_2 = _8.2 * _8.2;
RET = '\u{4c47e}';
_15 = -_6;
_13 = [46756_u16,46185_u16,64738_u16,39173_u16,47476_u16];
SetDiscriminant(_11.fld1.fld3, 2);
SetDiscriminant(_21, 1);
place!(Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4)).0 = (_11.fld2.0, _8.3);
_14[_5] = _9;
_11.fld6 = _11.fld1.fld1 + _11.fld1.fld1;
_6 = _1 + _15;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.3 = -_6;
_4[_5] = -_11.fld3[_5];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).0 = core::ptr::addr_of!(_18);
_24 = _15;
Call(place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.4[_5] = core::intrinsics::transmute(_13[_5]), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11.fld2.1 = 813338429_i32 as f32;
_11.fld2.0 = Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4).0.0;
_13[_5] = !23848_u16;
_27 = _3[_5] < _3[_5];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.1 = !94559164569190946174446834477569990618_i128;
_8.4 = 102761754790041560068615189487583558442_u128 - 4390729744944649734138410125767396399_u128;
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
place!(Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2)) = [_4[_5],Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.4[_5],_11.fld3[_5],_4[_5],Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.4[_5],Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.4[_5]];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.5 = Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4).0.1;
_18 = RET;
_11.fld1.fld0 = Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2)[_5] * _11.fld3[_5];
_23 = [_11.fld1.fld2[_5],_14[_5],_9,_11.fld1.fld2[_5],_11.fld0[_5],_11.fld0[_5],_14[_5]];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.7.0 = core::ptr::addr_of!(_18);
place!(Field::<[i64; 2]>(Variant(_11.fld1.fld3, 2), 6))[_5] = _9 as i64;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.0[_5] = RET as i16;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.2[_5] = !Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.4[_5];
Goto(bb10)
}
bb10 = {
_13 = [5309_u16,48182_u16,4060_u16,39870_u16,42887_u16];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.0 = (-71_i8);
place!(Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4)).0.1 = _3[_5] as f32;
_8.4 = !198793546536196620765145128609739361923_u128;
_5 = _3[_5] + _3[_5];
_11.fld0 = [_9,_9,_9,_9];
Goto(bb11)
}
bb11 = {
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.7 = Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4).0;
_23 = [_9,_9,_9,_9,_9,_9,_9];
_12 = _16 ^ _27;
_11.fld4 = _11.fld6 as u32;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.2 = -_11.fld1.fld0;
_7 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.0 as isize;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.6 = _18;
_25 = [_8.4];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.0 = (-114_i8) + 15_i8;
_8.3 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.5 + Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4).0.1;
place!(Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2)) = [Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,_11.fld1.fld0,_11.fld1.fld0,_11.fld1.fld0,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2];
_9 = 251_u8;
_21 = Adt48::Variant0 { fld0: Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2 };
_29 = _11.fld1.fld0 > Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2;
place!(Field::<[i64; 2]>(Variant(_11.fld1.fld3, 2), 6)) = [_11.fld6,_11.fld6];
_33 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.3 & _6;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.0 = [Field::<i16>(Variant(_21, 0), 0),_11.fld1.fld0,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<i16>(Variant(_21, 0), 0),Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2];
_11.fld1.fld2 = [_9,_9,_9,_9];
match _9 {
0 => bb4,
251 => bb13,
_ => bb12
}
}
bb12 = {
_13 = [5309_u16,48182_u16,4060_u16,39870_u16,42887_u16];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.0 = (-71_i8);
place!(Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4)).0.1 = _3[_5] as f32;
_8.4 = !198793546536196620765145128609739361923_u128;
_5 = _3[_5] + _3[_5];
_11.fld0 = [_9,_9,_9,_9];
Goto(bb11)
}
bb13 = {
SetDiscriminant(_21, 1);
_14 = [_9,_9,_9,_9];
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.7.0 = _8.1;
_32 = _11.fld6 as f32;
_3 = [_5,_5,_5,_5,_5,_5,_5,_5];
_19 = _27;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.0 = [Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,_11.fld1.fld0,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,_11.fld1.fld0,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2];
_13 = [46205_u16,42050_u16,5058_u16,34712_u16,32224_u16];
place!(Field::<[u128; 1]>(Variant(_21, 1), 0)) = _25;
place!(Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4)).0.0 = core::ptr::addr_of!(place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.6);
_22 = _2 as f64;
_11.fld3 = [Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,_11.fld1.fld0,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2];
_11.fld0 = _14;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3 = (Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2), _11.fld0, Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2), _1, Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2));
_8.0 = core::ptr::addr_of_mut!(_22);
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.4 = [_11.fld1.fld0,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2,Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2];
place!(Field::<[u128; 1]>(Variant(_11.fld1.fld3, 2), 5)) = Field::<[u128; 1]>(Variant(_21, 1), 0);
_35.1 = _8.1;
_32 = _8.3;
_7 = _33;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.2 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.4;
_8.3 = _8.4 as f32;
Goto(bb14)
}
bb14 = {
_7 = _33;
_28 = 2543_u16 | 16657_u16;
_8.1 = _35.1;
place!(Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2)) = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.3.2;
SetDiscriminant(_21, 1);
_11.fld2 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.7;
_14 = [_9,_9,_9,_9];
place!(Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4)) = (Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.7,);
_25 = Field::<[u128; 1]>(Variant(_11.fld1.fld3, 2), 5);
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.4 = _8.4 as i128;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.6 = _18;
_35.1 = _8.1;
_11.fld1.fld0 = -Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.2;
_11.fld1.fld1 = !_11.fld6;
_35.2 = !_2;
_32 = Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1).1.7.1 * Field::<((*const char, f32),)>(Variant(_11.fld1.fld3, 2), 4).0.1;
_28 = !7249_u16;
place!(Field::<(*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)))>(Variant(_11.fld1.fld3, 2), 1)).1.3.4 = Field::<[i16; 6]>(Variant(_11.fld1.fld3, 2), 2);
_35.0 = core::ptr::addr_of_mut!(_22);
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(17_usize, 13_usize, Move(_13), 28_usize, Move(_28), 33_usize, Move(_33), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(17_usize, 4_usize, Move(_4), 15_usize, Move(_15), 29_usize, Move(_29), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(17_usize, 6_usize, Move(_6), 3_usize, Move(_3), 23_usize, Move(_23), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [usize; 8],mut _2: isize,mut _3: [i16; 3],mut _4: isize) -> char {
mir! {
type RET = char;
let _5: u16;
let _6: [u8; 4];
let _7: [u16; 5];
let _8: bool;
let _9: (u128, i8);
let _10: f64;
let _11: bool;
let _12: u8;
let _13: Adt58;
let _14: u64;
let _15: Adt56;
let _16: [i16; 3];
let _17: (u128, i8);
let _18: f64;
let _19: Adt52;
let _20: f64;
let _21: i32;
let _22: i32;
let _23: isize;
let _24: u64;
let _25: Adt47;
let _26: u64;
let _27: [i16; 3];
let _28: f32;
let _29: ();
let _30: ();
{
_5 = 16987_u16;
RET = '\u{a57b0}';
_1 = [4037272057595104958_usize,14412417815734675894_usize,6_usize,0_usize,13957412887647104986_usize,7333812471515949031_usize,3224705546539142619_usize,1_usize];
_6 = [4_u8,2_u8,59_u8,22_u8];
_5 = 12131_u16;
RET = '\u{8c9e7}';
_2 = 240500449313707165791060219553485780547_u128 as isize;
_2 = _4 * _4;
_5 = 59107_u16 + 35261_u16;
_6 = [83_u8,200_u8,3_u8,80_u8];
RET = '\u{8eab7}';
_2 = -_4;
_3 = [(-15885_i16),6210_i16,(-10363_i16)];
_3 = [11595_i16,12790_i16,(-10770_i16)];
_1 = [4_usize,1_usize,1_usize,7_usize,7_usize,1_usize,2074406039227982827_usize,3_usize];
_3 = [18316_i16,(-16685_i16),(-8637_i16)];
_5 = 63982_u16 + 44609_u16;
_1 = [0_usize,3_usize,12473327485927910721_usize,0_usize,16262637320001412271_usize,2_usize,1_usize,7_usize];
_2 = -_4;
_5 = 46620_u16;
_3 = [(-31017_i16),(-8441_i16),(-8004_i16)];
Call(_7 = fn19(_1, _1, _2, _1, _2, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _4;
_1 = [13801101378096219093_usize,6_usize,6_usize,7600872231949913232_usize,10967057620177403830_usize,1_usize,2_usize,4_usize];
_3 = [(-3439_i16),18688_i16,20025_i16];
_8 = !false;
_9.0 = !216040003292047083662794865116864160848_u128;
_9 = (94722308613805822078341156641184821322_u128, (-125_i8));
_1 = [8888949035061209685_usize,10636953285887857435_usize,2_usize,8737451970333458943_usize,7_usize,2_usize,11925252118663832915_usize,3_usize];
_9 = (6094880650130819428854488438442913267_u128, (-65_i8));
_4 = _2 + _2;
_8 = false;
_7 = [_5,_5,_5,_5,_5];
_9.1 = _8 as i8;
_9.1 = (-32_i8) << _5;
_10 = 3614198646_u32 as f64;
_10 = _9.0 as f64;
_12 = 200_u8 >> _4;
_10 = (-22104604682218919890496846416489170297_i128) as f64;
_7 = [_5,_5,_5,_5,_5];
RET = '\u{66a51}';
_11 = !_8;
_9.0 = 265499289590742135625983305157635406784_u128;
_8 = _11;
_3 = [21936_i16,(-16480_i16),(-26459_i16)];
match _9.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
265499289590742135625983305157635406784 => bb7,
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
_17.1 = _9.1 * _9.1;
_14 = 11919933242832175803_u64 | 1989182722287626273_u64;
_9.0 = 157600048689823889819767783056106529815_u128;
_16 = _3;
_21 = (-1933049443_i32);
_9 = (175765889570535795809875466699565811920_u128, _17.1);
_14 = _11 as u64;
_4 = _21 as isize;
_6 = [_12,_12,_12,_12];
_21 = _12 as i32;
_17.0 = _9.0;
_21 = _12 as i32;
_7 = [_5,_5,_5,_5,_5];
RET = '\u{d32e8}';
_5 = 22764_u16;
_4 = _2 - _2;
match _9.0 {
0 => bb3,
1 => bb6,
2 => bb8,
3 => bb9,
4 => bb10,
175765889570535795809875466699565811920 => bb12,
_ => bb11
}
}
bb8 = {
_2 = _4;
_1 = [13801101378096219093_usize,6_usize,6_usize,7600872231949913232_usize,10967057620177403830_usize,1_usize,2_usize,4_usize];
_3 = [(-3439_i16),18688_i16,20025_i16];
_8 = !false;
_9.0 = !216040003292047083662794865116864160848_u128;
_9 = (94722308613805822078341156641184821322_u128, (-125_i8));
_1 = [8888949035061209685_usize,10636953285887857435_usize,2_usize,8737451970333458943_usize,7_usize,2_usize,11925252118663832915_usize,3_usize];
_9 = (6094880650130819428854488438442913267_u128, (-65_i8));
_4 = _2 + _2;
_8 = false;
_7 = [_5,_5,_5,_5,_5];
_9.1 = _8 as i8;
_9.1 = (-32_i8) << _5;
_10 = 3614198646_u32 as f64;
_10 = _9.0 as f64;
_12 = 200_u8 >> _4;
_10 = (-22104604682218919890496846416489170297_i128) as f64;
_7 = [_5,_5,_5,_5,_5];
RET = '\u{66a51}';
_11 = !_8;
_9.0 = 265499289590742135625983305157635406784_u128;
_8 = _11;
_3 = [21936_i16,(-16480_i16),(-26459_i16)];
match _9.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
265499289590742135625983305157635406784 => bb7,
_ => bb6
}
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
Goto(bb13)
}
bb13 = {
_21 = !(-836159838_i32);
_1 = [6_usize,1_usize,1317143818771248674_usize,8593979136294376056_usize,5280455674265563313_usize,4_usize,968963240617395083_usize,4_usize];
_6 = [_12,_12,_12,_12];
_21 = (-1851220048_i32);
_26 = _21 as u64;
_16 = _3;
_12 = 1_u8 << _17.0;
match _9.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb12,
175765889570535795809875466699565811920 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_27 = [(-28274_i16),(-28395_i16),(-17231_i16)];
_7 = [_5,_5,_5,_5,_5];
_12 = _9.1 as u8;
_23 = _11 as isize;
_8 = _11;
_21 = 1001315816_i32 + (-1499764394_i32);
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(18_usize, 23_usize, Move(_23), 21_usize, Move(_21), 1_usize, Move(_1), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(18_usize, 7_usize, Move(_7), 6_usize, Move(_6), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(18_usize, 17_usize, Move(_17), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [usize; 8],mut _2: [usize; 8],mut _3: isize,mut _4: [usize; 8],mut _5: isize,mut _6: [u8; 4]) -> [u16; 5] {
mir! {
type RET = [u16; 5];
let _7: Adt56;
let _8: [u16; 5];
let _9: bool;
let _10: u32;
let _11: isize;
let _12: u128;
let _13: (u128, i8);
let _14: [i16; 2];
let _15: [usize; 8];
let _16: u8;
let _17: i8;
let _18: [u8; 4];
let _19: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32));
let _20: [isize; 1];
let _21: [isize; 1];
let _22: i8;
let _23: i128;
let _24: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32));
let _25: isize;
let _26: i16;
let _27: [bool; 7];
let _28: isize;
let _29: f64;
let _30: u32;
let _31: [u128; 1];
let _32: Adt49;
let _33: f64;
let _34: [u8; 4];
let _35: isize;
let _36: i64;
let _37: Adt56;
let _38: Adt58;
let _39: ();
let _40: ();
{
_5 = 258153103978218997832438953740048319343_u128 as isize;
_6 = [166_u8,188_u8,251_u8,220_u8];
_1 = [2_usize,841803437876596330_usize,2596661960309748575_usize,6_usize,5_usize,7_usize,8139185071951370985_usize,0_usize];
_4 = [5972004340110831339_usize,11409280007961123105_usize,8441957656897579308_usize,5579722357650035935_usize,5165576820161672587_usize,1023161518073844742_usize,5_usize,8865594949170819085_usize];
_3 = _5 << _5;
_9 = false;
_4 = _1;
_4 = _1;
_10 = '\u{f37a6}' as u32;
Goto(bb1)
}
bb1 = {
_1 = [5_usize,8432155201954083076_usize,8873261409870044396_usize,4_usize,4_usize,9002333500432418221_usize,4113783306359460638_usize,5_usize];
_10 = (-20415_i16) as u32;
RET = [17886_u16,21778_u16,55262_u16,24202_u16,35335_u16];
RET = [48303_u16,47405_u16,10932_u16,5533_u16,18388_u16];
_3 = '\u{4b454}' as isize;
RET = [40541_u16,46214_u16,36272_u16,20644_u16,32941_u16];
_4 = [11631952269587839573_usize,7_usize,1_usize,9472555003247629078_usize,17125599299519305749_usize,3263923430004507160_usize,3593037174124192460_usize,0_usize];
RET = [10929_u16,8811_u16,46220_u16,34196_u16,36381_u16];
_3 = _5;
_1 = [6_usize,2_usize,2998631631038934538_usize,3_usize,18260219713440148499_usize,2_usize,6_usize,7_usize];
RET = [48708_u16,6811_u16,4831_u16,4708_u16,18069_u16];
_9 = true;
_4 = [1_usize,4_usize,7_usize,2_usize,4_usize,16283462712431338075_usize,7_usize,13739735180107089566_usize];
_1 = [6_usize,6166873388500904820_usize,1_usize,11290423009449603342_usize,14759081935763278920_usize,11517303293427913982_usize,13935622400166226533_usize,4_usize];
_9 = _3 > _5;
_10 = 2295713543_u32 * 3663176000_u32;
_1 = [3_usize,8442184139311009786_usize,2824392580251576937_usize,5_usize,0_usize,17251573294915424877_usize,1_usize,6_usize];
_2 = [11479574602288395640_usize,6_usize,5_usize,8107655531203184458_usize,2_usize,4571613388536480252_usize,15165115317116433251_usize,1_usize];
_11 = _3;
_5 = 58993639140249384877418983320926010162_i128 as isize;
_10 = !134727554_u32;
_3 = _5;
RET = [33214_u16,50422_u16,34396_u16,15519_u16,42103_u16];
_2 = _1;
_10 = !21652430_u32;
Goto(bb2)
}
bb2 = {
_9 = true | false;
RET = [24451_u16,55593_u16,34079_u16,18228_u16,49107_u16];
_12 = !131994837505568078542800611499508724686_u128;
_5 = -_11;
_13.1 = (-103_i8);
_4 = [5618654365743297127_usize,7_usize,5_usize,9574158052111953941_usize,1_usize,12522654127342847489_usize,14284436026915550476_usize,8632002499512170520_usize];
_4 = [12538874664831164294_usize,17844914480679748350_usize,2_usize,0_usize,7_usize,16929936181138173713_usize,0_usize,2_usize];
_13 = (_12, (-116_i8));
_3 = _5 & _5;
_2 = [3_usize,1437259531987107361_usize,2_usize,1_usize,11056158663225955741_usize,10807634529200153948_usize,3_usize,4337426477929628466_usize];
_13.1 = 0_usize as i8;
_12 = _13.0 | _13.0;
_13 = (_12, (-62_i8));
_1 = [4073574098315480487_usize,6179639181713077862_usize,2024583706288012424_usize,7336021013746065836_usize,1041173561961800046_usize,1_usize,1995084555941749904_usize,4_usize];
Goto(bb3)
}
bb3 = {
_13.0 = !_12;
_3 = !_11;
_8 = [36937_u16,4293_u16,4663_u16,36776_u16,24115_u16];
_9 = !false;
_8 = RET;
_3 = -_5;
_13 = (_12, 46_i8);
_16 = !243_u8;
RET = [37283_u16,65221_u16,55285_u16,50846_u16,19826_u16];
_19.7.1 = (-19260_i16) as f32;
_19.3.0 = [12749_i16,12148_i16,(-7277_i16),32070_i16,10719_i16,23994_i16];
_19.3.3 = _5;
_9 = true;
_16 = !204_u8;
_19.3.3 = -_5;
_19.0 = (-133857714701905765832856258925538135515_i128) as i8;
_19.6 = '\u{bd9ba}';
_13.1 = !_19.0;
_21 = [_11];
_19.0 = _13.1;
_18 = [_16,_16,_16,_16];
_19.6 = '\u{d0d7a}';
_12 = (-60611991710758725027758063189549954035_i128) as u128;
Goto(bb4)
}
bb4 = {
_19.1 = _13.0 as i128;
_13.0 = (-1267145179_i32) as u128;
_19.3.2 = [(-13336_i16),(-30986_i16),22337_i16,12134_i16,(-13115_i16),(-17891_i16)];
_19.1 = 86216808566766987241067441395937881066_i128 & (-152599812952170377185699327952163748783_i128);
_19.3.2 = [9731_i16,30103_i16,2779_i16,(-24069_i16),(-29255_i16),(-22810_i16)];
_19.6 = '\u{abdea}';
_20 = [_5];
_24.3 = (_19.3.2, _6, _19.3.2, _3, _19.3.2);
_13 = (_12, _19.0);
Goto(bb5)
}
bb5 = {
_16 = 68_u8;
_24.7.0 = core::ptr::addr_of!(_24.6);
_24.3.4 = [(-2306_i16),21783_i16,20052_i16,(-21473_i16),2349_i16,(-22660_i16)];
_24.7.0 = core::ptr::addr_of!(_19.6);
_19.2 = _9 as i16;
_19.5 = _12 as f32;
_2 = [6_usize,6951624621506120702_usize,3_usize,3813353680604718189_usize,2393443728643096761_usize,1_usize,4545078771580969923_usize,7534177277548487084_usize];
_15 = [3934251236587382225_usize,15823998721003854853_usize,15914357048658712909_usize,15079974806049191277_usize,12512874320328813189_usize,0_usize,7_usize,2_usize];
_14 = [_19.2,_19.2];
match _16 {
0 => bb4,
1 => bb3,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
68 => bb11,
_ => bb10
}
}
bb6 = {
_19.1 = _13.0 as i128;
_13.0 = (-1267145179_i32) as u128;
_19.3.2 = [(-13336_i16),(-30986_i16),22337_i16,12134_i16,(-13115_i16),(-17891_i16)];
_19.1 = 86216808566766987241067441395937881066_i128 & (-152599812952170377185699327952163748783_i128);
_19.3.2 = [9731_i16,30103_i16,2779_i16,(-24069_i16),(-29255_i16),(-22810_i16)];
_19.6 = '\u{abdea}';
_20 = [_5];
_24.3 = (_19.3.2, _6, _19.3.2, _3, _19.3.2);
_13 = (_12, _19.0);
Goto(bb5)
}
bb7 = {
_13.0 = !_12;
_3 = !_11;
_8 = [36937_u16,4293_u16,4663_u16,36776_u16,24115_u16];
_9 = !false;
_8 = RET;
_3 = -_5;
_13 = (_12, 46_i8);
_16 = !243_u8;
RET = [37283_u16,65221_u16,55285_u16,50846_u16,19826_u16];
_19.7.1 = (-19260_i16) as f32;
_19.3.0 = [12749_i16,12148_i16,(-7277_i16),32070_i16,10719_i16,23994_i16];
_19.3.3 = _5;
_9 = true;
_16 = !204_u8;
_19.3.3 = -_5;
_19.0 = (-133857714701905765832856258925538135515_i128) as i8;
_19.6 = '\u{bd9ba}';
_13.1 = !_19.0;
_21 = [_11];
_19.0 = _13.1;
_18 = [_16,_16,_16,_16];
_19.6 = '\u{d0d7a}';
_12 = (-60611991710758725027758063189549954035_i128) as u128;
Goto(bb4)
}
bb8 = {
_9 = true | false;
RET = [24451_u16,55593_u16,34079_u16,18228_u16,49107_u16];
_12 = !131994837505568078542800611499508724686_u128;
_5 = -_11;
_13.1 = (-103_i8);
_4 = [5618654365743297127_usize,7_usize,5_usize,9574158052111953941_usize,1_usize,12522654127342847489_usize,14284436026915550476_usize,8632002499512170520_usize];
_4 = [12538874664831164294_usize,17844914480679748350_usize,2_usize,0_usize,7_usize,16929936181138173713_usize,0_usize,2_usize];
_13 = (_12, (-116_i8));
_3 = _5 & _5;
_2 = [3_usize,1437259531987107361_usize,2_usize,1_usize,11056158663225955741_usize,10807634529200153948_usize,3_usize,4337426477929628466_usize];
_13.1 = 0_usize as i8;
_12 = _13.0 | _13.0;
_13 = (_12, (-62_i8));
_1 = [4073574098315480487_usize,6179639181713077862_usize,2024583706288012424_usize,7336021013746065836_usize,1041173561961800046_usize,1_usize,1995084555941749904_usize,4_usize];
Goto(bb3)
}
bb9 = {
_1 = [5_usize,8432155201954083076_usize,8873261409870044396_usize,4_usize,4_usize,9002333500432418221_usize,4113783306359460638_usize,5_usize];
_10 = (-20415_i16) as u32;
RET = [17886_u16,21778_u16,55262_u16,24202_u16,35335_u16];
RET = [48303_u16,47405_u16,10932_u16,5533_u16,18388_u16];
_3 = '\u{4b454}' as isize;
RET = [40541_u16,46214_u16,36272_u16,20644_u16,32941_u16];
_4 = [11631952269587839573_usize,7_usize,1_usize,9472555003247629078_usize,17125599299519305749_usize,3263923430004507160_usize,3593037174124192460_usize,0_usize];
RET = [10929_u16,8811_u16,46220_u16,34196_u16,36381_u16];
_3 = _5;
_1 = [6_usize,2_usize,2998631631038934538_usize,3_usize,18260219713440148499_usize,2_usize,6_usize,7_usize];
RET = [48708_u16,6811_u16,4831_u16,4708_u16,18069_u16];
_9 = true;
_4 = [1_usize,4_usize,7_usize,2_usize,4_usize,16283462712431338075_usize,7_usize,13739735180107089566_usize];
_1 = [6_usize,6166873388500904820_usize,1_usize,11290423009449603342_usize,14759081935763278920_usize,11517303293427913982_usize,13935622400166226533_usize,4_usize];
_9 = _3 > _5;
_10 = 2295713543_u32 * 3663176000_u32;
_1 = [3_usize,8442184139311009786_usize,2824392580251576937_usize,5_usize,0_usize,17251573294915424877_usize,1_usize,6_usize];
_2 = [11479574602288395640_usize,6_usize,5_usize,8107655531203184458_usize,2_usize,4571613388536480252_usize,15165115317116433251_usize,1_usize];
_11 = _3;
_5 = 58993639140249384877418983320926010162_i128 as isize;
_10 = !134727554_u32;
_3 = _5;
RET = [33214_u16,50422_u16,34396_u16,15519_u16,42103_u16];
_2 = _1;
_10 = !21652430_u32;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_25 = !_24.3.3;
_22 = _19.0;
_24.1 = _19.1;
_24.3.3 = _3;
RET = [18507_u16,18709_u16,34183_u16,20550_u16,2832_u16];
_24.3.2 = _24.3.0;
_19.3 = (_24.3.0, _18, _24.3.4, _3, _24.3.4);
_9 = _24.3.3 < _3;
_19.7.0 = _24.7.0;
_4 = [0_usize,17814284512200855291_usize,3149745491331268309_usize,853169117465885483_usize,4658772215699215011_usize,4_usize,0_usize,15381029572814299278_usize];
_24.6 = _19.6;
_24.7.1 = _19.5 - _19.5;
_23 = _19.1;
_29 = _19.2 as f64;
_19.3.3 = _16 as isize;
_24.3.0 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_11 = -_24.3.3;
_23 = _19.1;
Goto(bb12)
}
bb12 = {
_30 = !_10;
_24.7.0 = _19.7.0;
RET = _8;
_23 = 6495488158002921391_i64 as i128;
_24.1 = _23 & _23;
_24.0 = _3 as i8;
match _16 {
0 => bb11,
1 => bb13,
68 => bb15,
_ => bb14
}
}
bb13 = {
_16 = 68_u8;
_24.7.0 = core::ptr::addr_of!(_24.6);
_24.3.4 = [(-2306_i16),21783_i16,20052_i16,(-21473_i16),2349_i16,(-22660_i16)];
_24.7.0 = core::ptr::addr_of!(_19.6);
_19.2 = _9 as i16;
_19.5 = _12 as f32;
_2 = [6_usize,6951624621506120702_usize,3_usize,3813353680604718189_usize,2393443728643096761_usize,1_usize,4545078771580969923_usize,7534177277548487084_usize];
_15 = [3934251236587382225_usize,15823998721003854853_usize,15914357048658712909_usize,15079974806049191277_usize,12512874320328813189_usize,0_usize,7_usize,2_usize];
_14 = [_19.2,_19.2];
match _16 {
0 => bb4,
1 => bb3,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
68 => bb11,
_ => bb10
}
}
bb14 = {
_19.1 = _13.0 as i128;
_13.0 = (-1267145179_i32) as u128;
_19.3.2 = [(-13336_i16),(-30986_i16),22337_i16,12134_i16,(-13115_i16),(-17891_i16)];
_19.1 = 86216808566766987241067441395937881066_i128 & (-152599812952170377185699327952163748783_i128);
_19.3.2 = [9731_i16,30103_i16,2779_i16,(-24069_i16),(-29255_i16),(-22810_i16)];
_19.6 = '\u{abdea}';
_20 = [_5];
_24.3 = (_19.3.2, _6, _19.3.2, _3, _19.3.2);
_13 = (_12, _19.0);
Goto(bb5)
}
bb15 = {
_3 = _23 as isize;
_31 = [_13.0];
_26 = _12 as i16;
_28 = _19.3.3 ^ _5;
_10 = _25 as u32;
_23 = 15929890144042896205_usize as i128;
_32 = Adt49 { fld0: _29 };
_35 = _19.6 as isize;
_19 = (_22, _24.1, _26, _24.3, _24.1, _24.7.1, _24.6, _24.7);
_24.6 = _19.6;
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(19_usize, 12_usize, Move(_12), 14_usize, Move(_14), 30_usize, Move(_30), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(19_usize, 35_usize, Move(_35), 9_usize, Move(_9), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(19_usize, 1_usize, Move(_1), 18_usize, Move(_18), 28_usize, Move(_28), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(19_usize, 20_usize, Move(_20), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(44_u8), std::hint::black_box(7919975931349448660_i64), std::hint::black_box(13407_u16), std::hint::black_box(197991506_u32), std::hint::black_box((-437914205_i32)));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: [isize; 1],
fld1: *mut [i16; 6],
fld2: u32,
fld3: u64,
fld4: i16,
fld5: usize,
fld6: u8,
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [bool; 7],
fld1: [u128; 1],
fld2: u64,
fld3: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32))),
fld4: (*mut f64, *const char, u32, f32, u128),
fld5: *mut f64,
fld6: i64,
fld7: [i16; 6],

},
Variant1{
fld0: usize,
fld1: *mut f64,
fld2: [i16; 2],
fld3: *mut [i16; 6],
fld4: i128,
fld5: [usize; 8],

},
Variant2{
fld0: usize,
fld1: [u128; 1],
fld2: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32))),
fld3: i8,
fld4: [u8; 4],
fld5: (*mut f64, *const char, u32, f32, u128),

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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [usize; 8],
fld1: *const char,
fld2: f32,
fld3: *mut f64,
fld4: [u8; 7],
fld5: u32,
fld6: [isize; 1],

},
Variant1{
fld0: *mut [i16; 6],
fld1: usize,
fld2: *const char,

},
Variant2{
fld0: [isize; 1],
fld1: u16,
fld2: (u128, i8),

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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [u8; 4],
fld1: [usize; 8],
fld2: [bool; 7],
fld3: *mut usize,

},
Variant1{
fld0: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld1: Adt45,
fld2: *mut [i16; 6],
fld3: u16,
fld4: [u128; 1],
fld5: Adt44,
fld6: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32))),

},
Variant2{
fld0: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]),

}}
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
fld0: u64,
fld1: *const char,

},
Variant1{
fld0: [bool; 7],
fld1: u128,
fld2: isize,
fld3: [u128; 1],
fld4: f32,
fld5: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]),
fld6: (u128, i8),

},
Variant2{
fld0: bool,
fld1: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32))),
fld2: [i16; 6],
fld3: Adt44,
fld4: ((*const char, f32),),
fld5: [u128; 1],
fld6: [i64; 2],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: i16,

},
Variant1{
fld0: [u128; 1],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: f64,
}
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
fld0: bool,
fld1: Adt46,
fld2: i16,

},
Variant1{
fld0: (*const char, isize),
fld1: char,
fld2: (u128, i8),
fld3: Adt49,

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]),
fld2: Adt46,
fld3: [i16; 3],
fld4: Adt45,
fld5: *mut usize,
fld6: [u16; 5],
fld7: Adt47,

},
Variant1{
fld0: usize,
fld1: char,
fld2: Adt48,
fld3: u32,
fld4: i128,
fld5: u64,

},
Variant2{
fld0: [u16; 5],
fld1: i8,

},
Variant3{
fld0: [i16; 6],
fld1: [u16; 5],
fld2: Adt46,
fld3: (*mut f64, *const char, u32, f32, u128),
fld4: [i16; 2],
fld5: i64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
fld0: Adt48,
fld1: (u128, i8),
fld2: Adt46,
fld3: u32,
fld4: i16,
fld5: i32,
fld6: u16,

},
Variant1{
fld0: [u16; 5],
fld1: *mut [i16; 6],
fld2: f32,

},
Variant2{
fld0: [bool; 7],
fld1: (*const char, f32),
fld2: u32,
fld3: Adt48,
fld4: [usize; 8],
fld5: Adt44,
fld6: *mut [i16; 6],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: i16,
fld1: i64,
fld2: [u8; 4],
fld3: Adt47,
}
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
fld0: [u8; 4],
fld1: i128,
fld2: Adt46,
fld3: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld4: [i16; 6],

},
Variant1{
fld0: [u128; 1],
fld1: Adt52,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [u8; 4],
fld1: Adt53,
fld2: (*const char, f32),
fld3: [i16; 3],
fld4: u32,
fld5: Adt52,
fld6: i64,
fld7: Adt50,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: Adt44,

},
Variant1{
fld0: Adt48,
fld1: u16,
fld2: *const char,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]),
fld1: char,
fld2: f64,
fld3: [bool; 7],
fld4: *mut f64,

},
Variant1{
fld0: bool,
fld1: Adt46,
fld2: [bool; 7],
fld3: Adt47,
fld4: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld5: f64,
fld6: [u8; 4],
fld7: [isize; 1],

},
Variant2{
fld0: u8,
fld1: ((*const char, f32),),
fld2: (*mut f64, *const char, u32, f32, u128),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [i16; 2],
fld1: u32,
fld2: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld3: *mut usize,
fld4: [u128; 1],
fld5: [i64; 2],
fld6: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]),
fld7: ((*const char, f32),),

},
Variant1{
fld0: (*mut f64, *const char, u32, f32, u128),
fld1: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld2: [i64; 2],
fld3: (*const char, (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32))),
fld4: i16,
fld5: [isize; 1],

},
Variant2{
fld0: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld1: Adt44,
fld2: (*const char, isize),

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: u64,
fld1: [i16; 6],
fld2: ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]),
fld3: Adt43,
fld4: [isize; 1],
fld5: Adt58,
fld6: *mut f64,

},
Variant1{
fld0: (i8, i128, i16, ([i16; 6], [u8; 4], [i16; 6], isize, [i16; 6]), i128, f32, char, (*const char, f32)),
fld1: [u16; 5],
fld2: (*const char, f32),
fld3: *mut [i16; 6],
fld4: (*mut f64, *const char, u32, f32, u128),
fld5: [i16; 6],
fld6: f64,
fld7: *mut usize,

},
Variant2{
fld0: i128,
fld1: [u128; 1],
fld2: isize,
fld3: i8,

}}

