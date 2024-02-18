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
pub fn fn0(mut _1: u128,mut _2: char,mut _3: u64,mut _4: u32,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8) -> [usize; 2] {
mir! {
type RET = [usize; 2];
let _11: u128;
let _12: *const f32;
let _13: bool;
let _14: bool;
let _15: [i8; 3];
let _16: [bool; 8];
let _17: u64;
let _18: u16;
let _19: bool;
let _20: f64;
let _21: char;
let _22: bool;
let _23: usize;
let _24: f32;
let _25: u128;
let _26: [u64; 6];
let _27: (u64, &'static i8, *const f32);
let _28: char;
let _29: [usize; 8];
let _30: isize;
let _31: &'static u128;
let _32: ([i8; 3], *mut ([bool; 8], (f32,), &'static (u16, i16, i64)), *const [u128; 7], &'static u16);
let _33: [usize; 2];
let _34: &'static i64;
let _35: &'static u128;
let _36: [u32; 1];
let _37: [bool; 5];
let _38: i64;
let _39: i64;
let _40: i8;
let _41: isize;
let _42: &'static (u16, i16, i64);
let _43: u16;
let _44: *const [u128; 7];
let _45: (f64,);
let _46: *const [usize; 2];
let _47: bool;
let _48: [bool; 8];
let _49: ();
let _50: ();
{
_9 = 2308858870_u32 as usize;
_9 = 1_usize;
_3 = 342_u16 as u64;
_11 = !136348337186328122682336166122797655392_u128;
RET = [_9,_9];
_5 = 280_i16;
_5 = !26865_i16;
_9 = !RET[_9];
_6 = !1225590578_i32;
_5 = '\u{fbf08}' as i16;
_10 = 98_u8;
_5 = 35311_u16 as i16;
Call(_1 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = 32834945611552362184966995670734864865_i128;
_11 = _1;
_8 = true as i128;
_2 = '\u{de9dc}';
_9 = 7397529514867342046_usize;
Call(_2 = fn1(_5, _3, _5, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = '\u{102135}';
RET = [_9,_9];
RET = [_9,_9];
_4 = 2313271124_u32;
_8 = 70378709701948944340224729025490144616_i128 | (-7025886385962505263532882128101583060_i128);
_13 = _3 <= _3;
RET = [_9,_9];
_1 = _8 as u128;
_14 = _13;
_8 = (-167912557654054841210490437132938152377_i128);
_4 = 3616487990_u32;
RET = [_9,_9];
RET = [_9,_9];
_6 = (-1418820081_i32) | 1540253004_i32;
_1 = !_11;
_5 = _11 as i16;
_15 = [39_i8,(-6_i8),(-85_i8)];
_13 = !_14;
_5 = 17066_i16;
_2 = '\u{636c3}';
_1 = !_11;
_2 = '\u{64664}';
_11 = _1 << _10;
Goto(bb3)
}
bb3 = {
_7 = (-8363071625035876950_i64) ^ 7899024831863995776_i64;
RET = [_9,_9];
_11 = (-9223372036854775808_isize) as u128;
_14 = _13 ^ _13;
_7 = !(-2406941222057113629_i64);
match _9 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
7397529514867342046 => bb7,
_ => bb6
}
}
bb4 = {
_2 = '\u{102135}';
RET = [_9,_9];
RET = [_9,_9];
_4 = 2313271124_u32;
_8 = 70378709701948944340224729025490144616_i128 | (-7025886385962505263532882128101583060_i128);
_13 = _3 <= _3;
RET = [_9,_9];
_1 = _8 as u128;
_14 = _13;
_8 = (-167912557654054841210490437132938152377_i128);
_4 = 3616487990_u32;
RET = [_9,_9];
RET = [_9,_9];
_6 = (-1418820081_i32) | 1540253004_i32;
_1 = !_11;
_5 = _11 as i16;
_15 = [39_i8,(-6_i8),(-85_i8)];
_13 = !_14;
_5 = 17066_i16;
_2 = '\u{636c3}';
_1 = !_11;
_2 = '\u{64664}';
_11 = _1 << _10;
Goto(bb3)
}
bb5 = {
_8 = 32834945611552362184966995670734864865_i128;
_11 = _1;
_8 = true as i128;
_2 = '\u{de9dc}';
_9 = 7397529514867342046_usize;
Call(_2 = fn1(_5, _3, _5, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_15 = [(-21_i8),(-4_i8),(-67_i8)];
_15 = [120_i8,(-78_i8),70_i8];
_14 = _13;
_10 = !212_u8;
_5 = (-29455_i16);
_5 = 1081_i16 << _7;
_18 = 5231_u16 & 26264_u16;
_8 = -(-22465831410435671773081626277890426238_i128);
_13 = _8 <= _8;
_5 = 31256_i16 - 16798_i16;
_4 = !1086291614_u32;
_1 = _3 as u128;
_19 = !_14;
Goto(bb8)
}
bb8 = {
_16 = [_19,_13,_14,_14,_14,_19,_13,_14];
_6 = -212311208_i32;
_21 = _2;
_8 = (-133924989323947892836401536602620212884_i128);
_13 = _14;
_20 = _6 as f64;
_10 = 226_u8;
_6 = _18 as i32;
_22 = _19 <= _19;
_22 = _13;
RET = [_9,_9];
_21 = _2;
_5 = (-7692_i16) - (-4403_i16);
_16 = [_14,_14,_19,_19,_13,_13,_14,_14];
Goto(bb9)
}
bb9 = {
_23 = _9;
_22 = _19;
_6 = _21 as i32;
_9 = _23 << _18;
RET = [_9,_9];
RET = [_23,_23];
_27.2 = core::ptr::addr_of!(_24);
_10 = !164_u8;
_9 = !_23;
_12 = core::ptr::addr_of!(_24);
_23 = _9;
_28 = _21;
_6 = (-683620691_i32) - 663946761_i32;
RET = [_23,_9];
Goto(bb10)
}
bb10 = {
_25 = _7 as u128;
_21 = _2;
(*_12) = (-86_i8) as f32;
_18 = 36417_u16 ^ 47235_u16;
_6 = (-34110355_i32);
_2 = _28;
_18 = 12528_u16;
_17 = _25 as u64;
_32.3 = &_18;
_33 = RET;
_4 = 165589173_u32;
_18 = !19514_u16;
_33 = [_23,_9];
_31 = &_11;
_19 = _22;
_3 = _17;
_32.3 = &_18;
_13 = _22;
_25 = (*_31) ^ (*_31);
_35 = &_1;
RET = [_9,_9];
_29 = [_9,_23,_9,_9,_9,_9,_23,_23];
RET = [_23,_23];
_7 = (-8038516136056893742_i64);
_4 = _20 as u32;
_13 = !_19;
Goto(bb11)
}
bb11 = {
_32.0 = [108_i8,28_i8,110_i8];
_6 = _9 as i32;
_30 = _8 as isize;
_8 = _18 as i128;
_34 = &_7;
_31 = Move(_35);
_28 = _21;
_29 = [_9,_9,_23,_23,_9,_9,_9,_9];
_35 = &_25;
_31 = Move(_35);
_31 = &_1;
_10 = _18 as u8;
_14 = !_22;
_1 = _19 as u128;
_7 = (-5715685989742454281_i64);
_31 = &_1;
_7 = _5 as i64;
_25 = !(*_31);
_8 = _2 as i128;
_35 = &_11;
_19 = _13;
_13 = _22 | _19;
_40 = !82_i8;
_27.0 = _21 as u64;
Goto(bb12)
}
bb12 = {
_27.2 = core::ptr::addr_of!((*_12));
_39 = !_7;
_27.1 = &_40;
RET = [_23,_9];
_30 = (-9223372036854775808_isize) << _23;
_27.2 = core::ptr::addr_of!((*_12));
_1 = !(*_35);
_15 = [_40,_40,_40];
_16 = [_22,_13,_22,_13,_13,_14,_19,_19];
_46 = core::ptr::addr_of!(RET);
_33 = [_23,_23];
(*_46) = [_9,_9];
_27.2 = Move(_12);
_15 = [_40,_40,_40];
_43 = _18 & _18;
_37 = [_19,_22,_13,_19,_22];
_27.0 = _17;
_18 = _23 as u16;
_43 = !_18;
_45.0 = -_20;
_32.3 = &_43;
RET = [_9,_23];
_41 = _40 as isize;
Goto(bb13)
}
bb13 = {
_32.3 = &_18;
Goto(bb14)
}
bb14 = {
_36 = [_4];
_41 = _30 * _30;
_31 = Move(_35);
_12 = core::ptr::addr_of!(_24);
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(0_usize, 11_usize, Move(_11), 37_usize, Move(_37), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(0_usize, 23_usize, Move(_23), 39_usize, Move(_39), 14_usize, Move(_14), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(0_usize, 2_usize, Move(_2), 8_usize, Move(_8), 30_usize, Move(_30), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(0_usize, 21_usize, Move(_21), 43_usize, Move(_43), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i16,mut _2: u64,mut _3: i16,mut _4: u8) -> char {
mir! {
type RET = char;
let _5: [bool; 8];
let _6: (u16, i16, i64);
let _7: isize;
let _8: *mut [i8; 3];
let _9: [u128; 7];
let _10: u8;
let _11: *const [usize; 2];
let _12: u16;
let _13: f32;
let _14: i32;
let _15: i16;
let _16: isize;
let _17: *mut [i8; 3];
let _18: Adt46;
let _19: [i8; 3];
let _20: [bool; 8];
let _21: (u16, i16, i64);
let _22: &'static i64;
let _23: i8;
let _24: [usize; 8];
let _25: (f32,);
let _26: [i16; 8];
let _27: [u128; 7];
let _28: ();
let _29: ();
{
_1 = (-6442555096739768801_i64) as i16;
RET = '\u{f07d1}';
_4 = 199_u8;
_4 = !48_u8;
RET = '\u{135d}';
_5 = [true,true,false,false,false,false,true,true];
RET = '\u{10d6a6}';
_5 = [false,true,false,false,true,false,true,false];
_3 = !_1;
Goto(bb1)
}
bb1 = {
RET = '\u{cae69}';
_4 = !216_u8;
_6 = (37129_u16, _3, (-7639227269163395708_i64));
_6.0 = (-1217770674_i32) as u16;
_6.0 = 44829_u16;
_6.0 = 47363_u16;
_4 = _6.2 as u8;
_6.0 = 52286_u16;
_2 = 9625753716908844395_u64 + 2902244695365629369_u64;
_3 = !_1;
_5 = [false,false,false,true,false,false,true,true];
_3 = -_6.1;
Goto(bb2)
}
bb2 = {
_6 = (41601_u16, _1, (-8380091683698627856_i64));
_7 = !9223372036854775807_isize;
_9 = [159962594495305236278436513751708965300_u128,268153651220286994365697683198218488459_u128,145436785622992879365746569650187494523_u128,205616248088494774111658407467177016183_u128,100308566224539111249091389062833518421_u128,254012333136186343803491407005212567917_u128,295429951388452240454358098761488719805_u128];
_1 = _2 as i16;
_6.1 = _4 as i16;
_2 = _7 as u64;
match _6.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454994515748069583600 => bb8,
_ => bb7
}
}
bb3 = {
RET = '\u{cae69}';
_4 = !216_u8;
_6 = (37129_u16, _3, (-7639227269163395708_i64));
_6.0 = (-1217770674_i32) as u16;
_6.0 = 44829_u16;
_6.0 = 47363_u16;
_4 = _6.2 as u8;
_6.0 = 52286_u16;
_2 = 9625753716908844395_u64 + 2902244695365629369_u64;
_3 = !_1;
_5 = [false,false,false,true,false,false,true,true];
_3 = -_6.1;
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
_3 = 77_i8 as i16;
_9 = [269911123666073444150578195888362641977_u128,276940042549690679909404443692723393462_u128,30459387595831966394512829287675774718_u128,340264885639359542462166237281710101694_u128,332892314098121190187245114054019509714_u128,298354182882579517051099162320857047978_u128,73378032304118395315756316332371893202_u128];
_6 = (54259_u16, _3, (-8543279950265772403_i64));
_3 = _6.1;
_4 = 141_u8;
_4 = 252_u8;
_12 = _6.0 + _6.0;
_12 = _6.0 / _6.0;
_14 = (-164813399_i32) & (-2019092602_i32);
RET = '\u{d856b}';
_14 = (-779462258_i32);
_10 = _4 - _4;
_4 = _10 - _10;
_6.0 = !_12;
Goto(bb9)
}
bb9 = {
_6.0 = (-123085817639185386918292832333163625441_i128) as u16;
_1 = _3 & _3;
_10 = _4;
Call(_10 = core::intrinsics::transmute(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9 = [19117205092871852579031331251608764927_u128,76512704381656887016330043294882594991_u128,72024973524273169622790659764623263082_u128,62293737895807102547678575235424905048_u128,201318904765717616886662988293105817235_u128,201934188329134426477826631934951837127_u128,78165699278457900081729218728783224088_u128];
_13 = 55_i8 as f32;
_14 = 169349994900393181485922987390730190155_u128 as i32;
RET = '\u{86408}';
_5 = [true,false,false,true,false,false,false,false];
_6.2 = 87698112361399367798572952813180069298_i128 as i64;
_6 = (_12, _1, (-7807759981057787376_i64));
_6 = (_12, _1, (-3346385450624229394_i64));
_14 = -995266400_i32;
_3 = _1;
RET = '\u{761f9}';
_9 = [245130175888819059271220739675460346213_u128,207662572150165421688931127244702891225_u128,120385719770609747024223326972525925922_u128,146978731331372491878633925180465977581_u128,225332160971121857606834442265097660867_u128,16145226342854259002762976105261099753_u128,71691137129140163090871613378016359393_u128];
_13 = _6.0 as f32;
_12 = _6.0;
_12 = _6.0 ^ _6.0;
RET = '\u{b02b5}';
RET = '\u{103efc}';
RET = '\u{1071a7}';
_6.2 = 1629978926_u32 as i64;
_2 = 14314468221023841047_u64 + 12477754186838831341_u64;
_5 = [true,true,false,true,true,false,true,false];
_15 = 1773733172_u32 as i16;
_1 = _13 as i16;
_6.0 = _12;
_3 = _2 as i16;
_7 = 118_isize;
Goto(bb11)
}
bb11 = {
_6.0 = !_12;
_14 = (-524729931_i32);
Call(RET = fn2(_13, _6, _1, _6, _6, _5, _6, _10, _2, _5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5 = [false,true,false,false,false,true,true,true];
Call(_17 = fn4(_6.1, _6.0, _5, _6.0, _9, _9, _6.0, _4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6.2 = (-1429513710665262258_i64) & (-8779092408792080074_i64);
_19 = [(-75_i8),45_i8,82_i8];
_12 = _6.0;
Goto(bb14)
}
bb14 = {
_19 = [0_i8,21_i8,66_i8];
_6.0 = _12 * _12;
_4 = _10 & _10;
_2 = 725195167_u32 as u64;
_21 = (_6.0, _1, _6.2);
_6 = _21;
_6.1 = _1;
_1 = _15 << _6.1;
_6.1 = _1 * _1;
_5 = [true,false,true,true,false,true,true,false];
_7 = _13 as isize;
_6.0 = !_21.0;
_6.0 = !_21.0;
_20 = [true,true,true,true,false,true,false,false];
_17 = core::ptr::addr_of_mut!(_19);
_10 = _7 as u8;
_25.0 = _14 as f32;
_16 = _7 << _1;
_23 = RET as i8;
_15 = !_21.1;
_25.0 = _13;
_16 = _7 & _7;
_22 = &_21.2;
_26 = [_6.1,_1,_1,_6.1,_1,_6.1,_1,_1];
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(1_usize, 2_usize, Move(_2), 19_usize, Move(_19), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(1_usize, 7_usize, Move(_7), 21_usize, Move(_21), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(1_usize, 14_usize, Move(_14), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: f32,mut _2: (u16, i16, i64),mut _3: i16,mut _4: (u16, i16, i64),mut _5: (u16, i16, i64),mut _6: [bool; 8],mut _7: (u16, i16, i64),mut _8: u8,mut _9: u64,mut _10: [bool; 8]) -> char {
mir! {
type RET = char;
let _11: [usize; 8];
let _12: (u16, f32);
let _13: [bool; 5];
let _14: isize;
let _15: u32;
let _16: [bool; 5];
let _17: bool;
let _18: ();
let _19: ();
{
RET = '\u{371ff}';
_7.1 = 54157470446635152974818786545606101606_u128 as i16;
_3 = true as i16;
_7.1 = _4.1;
RET = '\u{cfdbc}';
_4.1 = _7.1;
_4.1 = !_5.1;
_3 = (-95_i8) as i16;
_2.0 = !_5.0;
_5.0 = 99570322080690680808083145861704551176_u128 as u16;
_9 = 2159999426610993448_u64;
Goto(bb1)
}
bb1 = {
_5.1 = _8 as i16;
_1 = _9 as f32;
_2.2 = _9 as i64;
_8 = 3_u8;
_7.0 = 225868150525433850261564144622210345181_u128 as u16;
_10 = _6;
_5.2 = _2.2;
_8 = 187_u8 >> _2.1;
_2.2 = _9 as i64;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2159999426610993448 => bb10,
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
_12.0 = _7.0 << _5.1;
_7.1 = -_4.1;
_13 = [true,true,false,false,true];
_12 = (_7.0, _1);
Call(_12 = fn3(_13, _4, _9, _13, _2, _2, _4, _5.1, _4, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7.2 = _12.1 as i64;
_7.0 = _2.0;
_7.0 = _9 as u16;
_5.0 = _12.0 & _4.0;
match _9 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
2159999426610993448 => bb17,
_ => bb16
}
}
bb12 = {
_5.1 = _8 as i16;
_1 = _9 as f32;
_2.2 = _9 as i64;
_8 = 3_u8;
_7.0 = 225868150525433850261564144622210345181_u128 as u16;
_10 = _6;
_5.2 = _2.2;
_8 = 187_u8 >> _2.1;
_2.2 = _9 as i64;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2159999426610993448 => bb10,
_ => bb9
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
Return()
}
bb17 = {
_8 = 146_u8 ^ 83_u8;
_5.0 = !_12.0;
_14 = 9223372036854775807_isize;
_7.2 = _2.2;
_6 = [true,true,false,true,false,true,true,true];
_3 = _4.1 & _2.1;
_5.0 = _12.0;
_4 = (_5.0, _7.1, _2.2);
_5.1 = _3 >> _12.0;
_4.2 = _5.2;
_7.2 = !_5.2;
_2.0 = _12.0 - _12.0;
_5.1 = _7.1;
_12.0 = _5.0;
_8 = 106_u8;
_7.2 = _2.2;
_5.0 = _2.0;
_6 = [false,true,false,false,true,true,false,false];
_4.0 = true as u16;
_7 = (_5.0, _5.1, _4.2);
Goto(bb18)
}
bb18 = {
Call(_18 = dump_var(2_usize, 14_usize, Move(_14), 6_usize, Move(_6), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_18 = dump_var(2_usize, 13_usize, Move(_13), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [bool; 5],mut _2: (u16, i16, i64),mut _3: u64,mut _4: [bool; 5],mut _5: (u16, i16, i64),mut _6: (u16, i16, i64),mut _7: (u16, i16, i64),mut _8: i16,mut _9: (u16, i16, i64),mut _10: [bool; 8]) -> (u16, f32) {
mir! {
type RET = (u16, f32);
let _11: isize;
let _12: [i8; 3];
let _13: [u32; 1];
let _14: isize;
let _15: (f64,);
let _16: char;
let _17: char;
let _18: u8;
let _19: ();
let _20: ();
{
_6 = _5;
_5.2 = !_7.2;
RET.0 = _9.0 & _6.0;
_2.2 = -_6.2;
_1 = [false,false,false,true,false];
RET.0 = (-526063104_i32) as u16;
_11 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_9.2 = _7.2;
_11 = -(-9223372036854775808_isize);
_3 = 13000402896716979685_u64;
_6.1 = _5.1 * _5.1;
_13 = [4212268618_u32];
_7 = (_6.0, _8, _2.2);
RET.0 = false as u16;
_5 = (_7.0, _2.1, _7.2);
_11 = 9223372036854775807_isize >> _7.0;
Goto(bb1)
}
bb1 = {
_4 = [false,true,false,false,true];
_12 = [(-71_i8),(-15_i8),(-126_i8)];
_6.1 = _9.1;
_6.0 = _5.0;
_9.0 = _7.0 & _5.0;
_8 = _7.1;
_2 = _9;
_13 = [2553387289_u32];
RET.0 = _2.0 + _7.0;
RET.1 = 213_u8 as f32;
_5.1 = _6.1;
_16 = '\u{bf2d6}';
_14 = _11;
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(3_usize, 2_usize, Move(_2), 6_usize, Move(_6), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(3_usize, 3_usize, Move(_3), 16_usize, Move(_16), 5_usize, Move(_5), 20_usize, _20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i16,mut _2: u16,mut _3: [bool; 8],mut _4: u16,mut _5: [u128; 7],mut _6: [u128; 7],mut _7: u16,mut _8: u8) -> *mut [i8; 3] {
mir! {
type RET = *mut [i8; 3];
let _9: [u128; 3];
let _10: &'static u128;
let _11: f64;
let _12: *mut [i8; 3];
let _13: u32;
let _14: u128;
let _15: isize;
let _16: usize;
let _17: u32;
let _18: isize;
let _19: Adt73;
let _20: &'static (f32,);
let _21: [u128; 7];
let _22: [u64; 6];
let _23: [u32; 8];
let _24: *mut u8;
let _25: i8;
let _26: f32;
let _27: u16;
let _28: i64;
let _29: bool;
let _30: f64;
let _31: &'static i64;
let _32: isize;
let _33: bool;
let _34: Adt71;
let _35: i32;
let _36: bool;
let _37: isize;
let _38: Adt65;
let _39: bool;
let _40: [char; 7];
let _41: (*const f32, &'static (f32,), *const &'static (u16, f32), usize);
let _42: (u16, i16, i64);
let _43: isize;
let _44: (f32,);
let _45: char;
let _46: i16;
let _47: *mut u16;
let _48: [u64; 6];
let _49: [u128; 3];
let _50: *mut isize;
let _51: f64;
let _52: (&'static i32, [usize; 8], &'static [usize; 2], &'static (u16, f32));
let _53: f64;
let _54: Adt29;
let _55: u8;
let _56: (&'static u128, &'static i16, *mut u16);
let _57: [usize; 8];
let _58: Adt55;
let _59: [u64; 6];
let _60: (&'static i32, [usize; 8], &'static [usize; 2], &'static (u16, f32));
let _61: *mut u8;
let _62: char;
let _63: u16;
let _64: u64;
let _65: ([usize; 4], [u32; 8], Adt55, u64);
let _66: [usize; 2];
let _67: i64;
let _68: bool;
let _69: i64;
let _70: (&'static u128, &'static i16, *mut u16);
let _71: [i8; 3];
let _72: isize;
let _73: f32;
let _74: *const &'static (u16, f32);
let _75: ([bool; 8], (f32,), &'static (u16, i16, i64));
let _76: &'static *const [u128; 7];
let _77: (&'static (u16, i16, i64), i16, *const [u128; 7]);
let _78: ();
let _79: ();
{
_7 = _2 << _2;
_6 = [295481100284074784727712989678245248425_u128,339567303322087659179614807109247507781_u128,129440474265405219056205641391194483988_u128,320014486526139935927735317690186044013_u128,253251586820868650979876094555293013928_u128,277062242603729289973759375595629622520_u128,292090800786752573560494973515685558182_u128];
_9 = [286761950925150595006503913232185126284_u128,135853713186943815130348774247742401308_u128,175626209244592628301604361861029480866_u128];
_5 = [112814465842411554689459864976833042945_u128,80170072276864114819668793030104787613_u128,339942393716575637417357103684169907638_u128,21958573537314707190363608546361917847_u128,122619671469554814924984504405748646098_u128,237473512721888000346556593681260412153_u128,324501996530008313439853510027015014394_u128];
_2 = _4;
_4 = _2;
_7 = _4 & _4;
_5 = [185280388823148301632929902392646592244_u128,339118915391546033820492003497199681499_u128,139827473036409646493369200909778530102_u128,55218652524852133419262999816496469186_u128,281012354499203298441120884058972127109_u128,101942287896149243456994886657590375221_u128,268710376850165047008163337246692136431_u128];
_3 = [true,false,true,true,true,false,true,false];
_1 = 34852049001131153415991363464987781845_i128 as i16;
_5 = [35871818470929764278935239922631613445_u128,264452785256267665785763231281738645326_u128,23026679842892574078319283106887215172_u128,335072882841254052216008099213347238788_u128,333124508556328001398577285854567802851_u128,121093372869398887275981081739523894548_u128,250309604025037937538537229655873637197_u128];
_1 = 16934243496683444743_usize as i16;
_7 = _2;
_6 = [169231012710404758991178545378999649197_u128,338612230456180835711443961858086339223_u128,193363644294569836399271327743083339497_u128,45962291084596497777944975231983014417_u128,15390879673937542469238479261691673453_u128,168685802740904665371664595950014655311_u128,159904394713740718379881163028768635530_u128];
_2 = _4 - _7;
_9 = [213260825606839130593103776460237780145_u128,277968927138497409823699125233648277124_u128,323777402568175002216827613619511514463_u128];
_6 = _5;
_3 = [true,false,false,true,false,true,false,false];
_1 = '\u{f8d77}' as i16;
_11 = 9006112019717487237_i64 as f64;
_8 = (-9223372036854775808_isize) as u8;
_8 = 166_u8 * 177_u8;
_1 = -(-6701_i16);
_11 = (-3291942372167146709_i64) as f64;
Goto(bb1)
}
bb1 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _6;
_2 = _7;
_1 = !7980_i16;
_14 = 307145933453523104434160118265725148341_u128;
_6 = [_14,_14,_14,_14,_14,_14,_14];
_13 = 1401445628_u32;
_14 = !232792849000571461380180391564388010823_u128;
_9 = [_14,_14,_14];
Call(_1 = fn7(_4, _5, _3, _4, _2, _5, _3, _5, _7, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = [_14,_14,_14];
_9 = [_14,_14,_14];
_1 = 22891_i16;
_5 = [_14,_14,_14,_14,_14,_14,_14];
_3 = [true,true,true,true,true,true,true,false];
_7 = !_2;
_14 = 13890779287232148462_u64 as u128;
_5 = [_14,_14,_14,_14,_14,_14,_14];
_14 = _1 as u128;
_19 = Adt73::Variant3 { fld0: '\u{80f4}' };
_13 = 3044869409436489268_usize as u32;
_7 = !_2;
_18 = !24_isize;
_13 = !116250161_u32;
_2 = _4;
_19 = Adt73::Variant3 { fld0: '\u{94b94}' };
_13 = 2650669833_u32 + 3449240454_u32;
Goto(bb4)
}
bb4 = {
_9 = [_14,_14,_14];
_16 = !0_usize;
_16 = '\u{c6217}' as usize;
_19 = Adt73::Variant3 { fld0: '\u{ba394}' };
_3 = [true,true,true,false,true,true,true,false];
_18 = _4 as isize;
_16 = 3_usize ^ 7_usize;
_6 = [_14,_14,_14,_14,_14,_14,_14];
_10 = &_14;
_6 = [_14,(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb5)
}
bb5 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_17 = _13 ^ _13;
_14 = _16 as u128;
_22 = [10793610181809238789_u64,7484116611255211663_u64,11631691734069631327_u64,3678772986866003606_u64,5731002447410780240_u64,7092889803616492322_u64];
_5 = _21;
_13 = _17;
_15 = -_18;
_24 = core::ptr::addr_of_mut!(_8);
_9 = [_14,_14,_14];
_5 = [_14,_14,_14,_14,_14,_14,_14];
_3 = [true,false,true,true,false,false,false,true];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{674e7}';
_8 = !9_u8;
_23 = [_13,_17,_13,_17,_17,_13,_13,_13];
_2 = _16 as u16;
(*_24) = 54_u8;
_21 = _6;
_2 = _4 >> _18;
SetDiscriminant(_19, 3);
_5 = [_14,_14,_14,_14,_14,_14,_14];
_14 = 70006418238234330224221889622650465164_u128 << _4;
_10 = &_14;
_22 = [16568238632770853024_u64,6041173028285251147_u64,9113939346522189494_u64,6086083047889193269_u64,5378851634664413794_u64,15233953863266478482_u64];
_11 = _2 as f64;
_6 = _21;
_2 = _7 | _7;
_2 = _4;
_21 = [(*_10),(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb7)
}
bb7 = {
(*_24) = !97_u8;
_7 = '\u{4bad8}' as u16;
_13 = _17 & _17;
_8 = 21_u8;
_23 = [_13,_13,_17,_17,_13,_13,_17,_13];
_10 = &_14;
_7 = !_2;
_2 = !_7;
_7 = _2 - _2;
match _8 {
0 => bb5,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
21 => bb13,
_ => bb12
}
}
bb8 = {
_17 = _13 ^ _13;
_14 = _16 as u128;
_22 = [10793610181809238789_u64,7484116611255211663_u64,11631691734069631327_u64,3678772986866003606_u64,5731002447410780240_u64,7092889803616492322_u64];
_5 = _21;
_13 = _17;
_15 = -_18;
_24 = core::ptr::addr_of_mut!(_8);
_9 = [_14,_14,_14];
_5 = [_14,_14,_14,_14,_14,_14,_14];
_3 = [true,false,true,true,false,false,false,true];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{674e7}';
_8 = !9_u8;
_23 = [_13,_17,_13,_17,_17,_13,_13,_13];
_2 = _16 as u16;
(*_24) = 54_u8;
_21 = _6;
_2 = _4 >> _18;
SetDiscriminant(_19, 3);
_5 = [_14,_14,_14,_14,_14,_14,_14];
_14 = 70006418238234330224221889622650465164_u128 << _4;
_10 = &_14;
_22 = [16568238632770853024_u64,6041173028285251147_u64,9113939346522189494_u64,6086083047889193269_u64,5378851634664413794_u64,15233953863266478482_u64];
_11 = _2 as f64;
_6 = _21;
_2 = _7 | _7;
_2 = _4;
_21 = [(*_10),(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb7)
}
bb9 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_9 = [_14,_14,_14];
_16 = !0_usize;
_16 = '\u{c6217}' as usize;
_19 = Adt73::Variant3 { fld0: '\u{ba394}' };
_3 = [true,true,true,false,true,true,true,false];
_18 = _4 as isize;
_16 = 3_usize ^ 7_usize;
_6 = [_14,_14,_14,_14,_14,_14,_14];
_10 = &_14;
_6 = [_14,(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb5)
}
bb11 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_5 = _6;
_2 = _7;
_1 = !7980_i16;
_14 = 307145933453523104434160118265725148341_u128;
_6 = [_14,_14,_14,_14,_14,_14,_14];
_13 = 1401445628_u32;
_14 = !232792849000571461380180391564388010823_u128;
_9 = [_14,_14,_14];
Call(_1 = fn7(_4, _5, _3, _4, _2, _5, _3, _5, _7, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{ed04b}';
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_2 = !_7;
_23 = [_13,_13,_13,_13,_17,_17,_17,_13];
_11 = _13 as f64;
_17 = _13;
_23 = [_17,_17,_13,_17,_13,_13,_17,_17];
_25 = (-90_i8) + (-18_i8);
SetDiscriminant(_19, 3);
(*_24) = 145_u8;
_13 = _17;
_27 = 1158679077_i32 as u16;
_24 = core::ptr::addr_of_mut!((*_24));
(*_24) = 140_u8;
(*_24) = !13_u8;
_6 = [_14,(*_10),(*_10),(*_10),(*_10),_14,(*_10)];
Call(_21 = fn19(_2, _17, _7), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_21 = [_14,_14,(*_10),_14,_14,(*_10),(*_10)];
_25 = -(-106_i8);
Call(_2 = core::intrinsics::bswap(_4), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_26 = _17 as f32;
_4 = !_2;
_3 = [true,false,true,false,false,false,true,false];
_5 = _21;
_18 = (*_24) as isize;
_9 = [(*_10),(*_10),_14];
_28 = _26 as i64;
_9 = [(*_10),_14,(*_10)];
_16 = _13 as usize;
_30 = -_11;
_4 = !_7;
_18 = -_15;
_28 = (-6047935118366440260_i64);
_34.fld5.fld5 = _4 >> (*_10);
_34.fld5.fld1 = _17;
_34.fld5.fld3 = _34.fld5.fld1 as i8;
_34.fld3 = (_34.fld5.fld5, _26);
_34.fld4 = core::ptr::addr_of_mut!(_34.fld5.fld5);
_26 = _17 as f32;
_29 = false;
_34.fld5.fld5 = _34.fld3.0;
Goto(bb16)
}
bb16 = {
_24 = core::ptr::addr_of_mut!((*_24));
_34.fld7 = [_16,_16];
_32 = (-145126124570893226831001352438085333162_i128) as isize;
_23 = [_17,_34.fld5.fld1,_13,_34.fld5.fld1,_17,_17,_13,_34.fld5.fld1];
_32 = !_15;
_34.fld4 = core::ptr::addr_of_mut!(_4);
_3 = [_29,_29,_29,_29,_29,_29,_29,_29];
_3 = [_29,_29,_29,_29,_29,_29,_29,_29];
match _28 {
0 => bb17,
340282366920938463457326672313401771196 => bb19,
_ => bb18
}
}
bb17 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb18 = {
_17 = _13 ^ _13;
_14 = _16 as u128;
_22 = [10793610181809238789_u64,7484116611255211663_u64,11631691734069631327_u64,3678772986866003606_u64,5731002447410780240_u64,7092889803616492322_u64];
_5 = _21;
_13 = _17;
_15 = -_18;
_24 = core::ptr::addr_of_mut!(_8);
_9 = [_14,_14,_14];
_5 = [_14,_14,_14,_14,_14,_14,_14];
_3 = [true,false,true,true,false,false,false,true];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{674e7}';
_8 = !9_u8;
_23 = [_13,_17,_13,_17,_17,_13,_13,_13];
_2 = _16 as u16;
(*_24) = 54_u8;
_21 = _6;
_2 = _4 >> _18;
SetDiscriminant(_19, 3);
_5 = [_14,_14,_14,_14,_14,_14,_14];
_14 = 70006418238234330224221889622650465164_u128 << _4;
_10 = &_14;
_22 = [16568238632770853024_u64,6041173028285251147_u64,9113939346522189494_u64,6086083047889193269_u64,5378851634664413794_u64,15233953863266478482_u64];
_11 = _2 as f64;
_6 = _21;
_2 = _7 | _7;
_2 = _4;
_21 = [(*_10),(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb7)
}
bb19 = {
_37 = _18;
_2 = _28 as u16;
_29 = !true;
_34.fld5.fld2 = _13 as u128;
match _28 {
0 => bb3,
1 => bb20,
340282366920938463457326672313401771196 => bb22,
_ => bb21
}
}
bb20 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb21 = {
_9 = [_14,_14,_14];
_16 = !0_usize;
_16 = '\u{c6217}' as usize;
_19 = Adt73::Variant3 { fld0: '\u{ba394}' };
_3 = [true,true,true,false,true,true,true,false];
_18 = _4 as isize;
_16 = 3_usize ^ 7_usize;
_6 = [_14,_14,_14,_14,_14,_14,_14];
_10 = &_14;
_6 = [_14,(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb5)
}
bb22 = {
_24 = core::ptr::addr_of_mut!((*_24));
_42.1 = _1;
_44.0 = _26 + _34.fld3.1;
_34.fld5.fld2 = _17 as u128;
_14 = 15475796131335900346_u64 as u128;
_28 = (-1141727409076445570_i64) | 4036304659303346960_i64;
_43 = !_37;
_41.3 = _16 - _16;
Goto(bb23)
}
bb23 = {
_34.fld5.fld0 = 14127743409929055631_u64;
_5 = [_14,_34.fld5.fld2,_14,_34.fld5.fld2,_14,_14,_34.fld5.fld2];
(*_24) = _28 as u8;
match _34.fld5.fld0 {
0 => bb6,
1 => bb14,
2 => bb7,
3 => bb16,
14127743409929055631 => bb24,
_ => bb21
}
}
bb24 = {
_25 = _34.fld5.fld3 - _34.fld5.fld3;
_41.1 = &_44;
_20 = Move(_41.1);
_17 = _13;
_17 = !_34.fld5.fld1;
_26 = _44.0 * _44.0;
_33 = _29;
_34.fld0 = _7 <= _7;
_35 = (-1888739927_i32);
_34.fld5.fld3 = _25;
_36 = !_34.fld0;
_31 = &_42.2;
_43 = _41.3 as isize;
_42.1 = _1;
_34.fld5.fld3 = 36067114214603089086313334844109901979_i128 as i8;
_5 = [_34.fld5.fld2,_34.fld5.fld2,_34.fld5.fld2,_34.fld5.fld2,_34.fld5.fld2,_34.fld5.fld2,_14];
_15 = _18;
match _35 {
0 => bb8,
1 => bb13,
2 => bb17,
3 => bb25,
4 => bb26,
5 => bb27,
340282366920938463463374607429879471529 => bb29,
_ => bb28
}
}
bb25 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb26 = {
(*_24) = !97_u8;
_7 = '\u{4bad8}' as u16;
_13 = _17 & _17;
_8 = 21_u8;
_23 = [_13,_13,_17,_17,_13,_13,_17,_13];
_10 = &_14;
_7 = !_2;
_2 = !_7;
_7 = _2 - _2;
match _8 {
0 => bb5,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
21 => bb13,
_ => bb12
}
}
bb27 = {
_37 = _18;
_2 = _28 as u16;
_29 = !true;
_34.fld5.fld2 = _13 as u128;
match _28 {
0 => bb3,
1 => bb20,
340282366920938463457326672313401771196 => bb22,
_ => bb21
}
}
bb28 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb29 = {
_4 = _7;
_42.0 = _27 << _16;
_28 = !(-2390518888693264077_i64);
_34.fld5.fld7 = -80187260335660487358006989522230900406_i128;
_14 = _41.3 as u128;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{a0f1a}';
_22 = [_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0];
_10 = &_34.fld5.fld2;
Goto(bb30)
}
bb30 = {
_29 = _34.fld5.fld5 <= _34.fld5.fld5;
_8 = !234_u8;
_34.fld2 = _21;
_52.1 = [_41.3,_41.3,_41.3,_41.3,_16,_16,_16,_41.3];
_48 = _22;
_43 = _18 - _37;
_36 = _34.fld0;
_34.fld3 = (_34.fld5.fld5, _44.0);
_47 = Move(_34.fld4);
_44 = (_26,);
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{3b4dd}';
_54.fld7 = _14 as i128;
_34.fld2 = _5;
_52.0 = &_35;
_34.fld5.fld2 = _14;
_34.fld5.fld3 = -_25;
_53 = -_11;
_41.1 = &_44;
_34.fld4 = Move(_47);
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{c3354}';
_53 = _11;
_54.fld5 = _34.fld5.fld5 >> _34.fld3.0;
_46 = _4 as i16;
_33 = _43 != _43;
match _34.fld5.fld0 {
0 => bb16,
1 => bb27,
14127743409929055631 => bb32,
_ => bb31
}
}
bb31 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb32 = {
_54.fld3 = _25 + _34.fld5.fld3;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{51bbc}';
_51 = _44.0 as f64;
_33 = _34.fld0 == _36;
_45 = Field::<char>(Variant(_19, 3), 0);
_40 = [_45,_45,_45,Field::<char>(Variant(_19, 3), 0),_45,_45,Field::<char>(Variant(_19, 3), 0)];
_7 = !_34.fld5.fld5;
_14 = _34.fld5.fld2;
match _34.fld5.fld0 {
0 => bb23,
1 => bb18,
2 => bb28,
14127743409929055631 => bb34,
_ => bb33
}
}
bb33 = {
_17 = _13 ^ _13;
_14 = _16 as u128;
_22 = [10793610181809238789_u64,7484116611255211663_u64,11631691734069631327_u64,3678772986866003606_u64,5731002447410780240_u64,7092889803616492322_u64];
_5 = _21;
_13 = _17;
_15 = -_18;
_24 = core::ptr::addr_of_mut!(_8);
_9 = [_14,_14,_14];
_5 = [_14,_14,_14,_14,_14,_14,_14];
_3 = [true,false,true,true,false,false,false,true];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{674e7}';
_8 = !9_u8;
_23 = [_13,_17,_13,_17,_17,_13,_13,_13];
_2 = _16 as u16;
(*_24) = 54_u8;
_21 = _6;
_2 = _4 >> _18;
SetDiscriminant(_19, 3);
_5 = [_14,_14,_14,_14,_14,_14,_14];
_14 = 70006418238234330224221889622650465164_u128 << _4;
_10 = &_14;
_22 = [16568238632770853024_u64,6041173028285251147_u64,9113939346522189494_u64,6086083047889193269_u64,5378851634664413794_u64,15233953863266478482_u64];
_11 = _2 as f64;
_6 = _21;
_2 = _7 | _7;
_2 = _4;
_21 = [(*_10),(*_10),(*_10),(*_10),(*_10),(*_10),(*_10)];
Goto(bb7)
}
bb34 = {
_58.fld0 = core::ptr::addr_of_mut!(_54.fld1);
_53 = -_51;
_34.fld3.1 = -_26;
_34.fld4 = core::ptr::addr_of_mut!(_54.fld5);
_54.fld6 = Adt18::Variant1 { fld0: _8,fld1: _34.fld5.fld2 };
(*_24) = _54.fld7 as u8;
_54.fld4 = _46 << (*_24);
_41.0 = core::ptr::addr_of!(_34.fld3.1);
_34.fld2 = [_14,Field::<u128>(Variant(_54.fld6, 1), 1),Field::<u128>(Variant(_54.fld6, 1), 1),_34.fld5.fld2,_14,_34.fld5.fld2,Field::<u128>(Variant(_54.fld6, 1), 1)];
_42.2 = _28;
_34.fld2 = [Field::<u128>(Variant(_54.fld6, 1), 1),Field::<u128>(Variant(_54.fld6, 1), 1),_14,_14,_14,Field::<u128>(Variant(_54.fld6, 1), 1),Field::<u128>(Variant(_54.fld6, 1), 1)];
_44 = (_34.fld3.1,);
_58.fld1.fld1 = _34.fld5.fld1;
_58.fld1.fld2 = _34.fld5.fld0 as u128;
_63 = _7 ^ _42.0;
_39 = _43 == _15;
SetDiscriminant(_19, 1);
_35 = 239426231_i32;
Goto(bb35)
}
bb35 = {
_44.0 = _26;
_35 = 1362789726_i32 + 1120725693_i32;
_34.fld5.fld2 = _14;
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld3.0 = _63;
_65.2.fld1.fld3 = _54.fld3;
_34.fld5 = Adt29 { fld0: 7157814292196800720_u64,fld1: _13,fld2: _14,fld3: _54.fld3,fld4: _46,fld5: _4,fld6: Move(_54.fld6),fld7: _54.fld7 };
_48 = [_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0,_34.fld5.fld0];
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld5.fld6 = Move(_34.fld5.fld6);
_15 = _45 as isize;
_58.fld1.fld2 = _28 as u128;
_5 = [_34.fld5.fld2,_58.fld1.fld2,_34.fld5.fld2,_34.fld5.fld2,_34.fld5.fld2,Field::<u128>(Variant(Field::<Adt71>(Variant(_19, 1), 1).fld5.fld6, 1), 1),Field::<u128>(Variant(Field::<Adt71>(Variant(_19, 1), 1).fld5.fld6, 1), 1)];
_61 = core::ptr::addr_of_mut!(_8);
_34.fld3.1 = _26;
SetDiscriminant(Field::<Adt71>(Variant(_19, 1), 1).fld5.fld6, 2);
_58.fld1.fld3 = _65.2.fld1.fld3 ^ _54.fld3;
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld3 = _34.fld3;
_60.1 = [_41.3,_41.3,_41.3,_41.3,_41.3,_41.3,_41.3,_41.3];
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld5.fld6 = Adt18::Variant1 { fld0: (*_61),fld1: _34.fld5.fld2 };
_50 = core::ptr::addr_of_mut!(_18);
_34.fld5.fld1 = _58.fld1.fld1;
_34.fld5.fld6 = Move(Field::<Adt71>(Variant(_19, 1), 1).fld5.fld6);
match _34.fld5.fld0 {
7157814292196800720 => bb36,
_ => bb2
}
}
bb36 = {
_65.2.fld1.fld1 = !_13;
_15 = (*_50) & (*_50);
_25 = !_34.fld5.fld3;
_65.2.fld1 = Adt29 { fld0: _34.fld5.fld0,fld1: _34.fld5.fld1,fld2: Field::<u128>(Variant(_34.fld5.fld6, 1), 1),fld3: _54.fld3,fld4: _46,fld5: _63,fld6: Move(_34.fld5.fld6),fld7: _34.fld5.fld7 };
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld5.fld6 = Move(_65.2.fld1.fld6);
match _65.2.fld1.fld0 {
0 => bb37,
1 => bb38,
2 => bb39,
3 => bb40,
7157814292196800720 => bb42,
_ => bb41
}
}
bb37 = {
_24 = core::ptr::addr_of_mut!((*_24));
_34.fld7 = [_16,_16];
_32 = (-145126124570893226831001352438085333162_i128) as isize;
_23 = [_17,_34.fld5.fld1,_13,_34.fld5.fld1,_17,_17,_13,_34.fld5.fld1];
_32 = !_15;
_34.fld4 = core::ptr::addr_of_mut!(_4);
_3 = [_29,_29,_29,_29,_29,_29,_29,_29];
_3 = [_29,_29,_29,_29,_29,_29,_29,_29];
match _28 {
0 => bb17,
340282366920938463457326672313401771196 => bb19,
_ => bb18
}
}
bb38 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb39 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb40 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb41 = {
_13 = 2163032740_u32 << _4;
_9 = [258633937838048731137832839108663461138_u128,307932536186509937000576520038049453602_u128,133285564705119207641807913834490415747_u128];
_8 = 160_u8;
_8 = 133_u8 ^ 15_u8;
_7 = !_4;
Call(_2 = fn5(_7, _9, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb42 = {
_66 = [_41.3,_41.3];
_31 = &_34.fld6;
_58.fld1 = Adt29 { fld0: _65.2.fld1.fld0,fld1: _17,fld2: _65.2.fld1.fld2,fld3: _34.fld5.fld3,fld4: _34.fld5.fld4,fld5: _4,fld6: Move(Field::<Adt71>(Variant(_19, 1), 1).fld5.fld6),fld7: _54.fld7 };
_34.fld6 = _43 as i64;
_65.0 = [_41.3,_41.3,_41.3,_16];
_54 = Adt29 { fld0: _34.fld5.fld0,fld1: _58.fld1.fld1,fld2: _65.2.fld1.fld2,fld3: _65.2.fld1.fld3,fld4: _65.2.fld1.fld4,fld5: _7,fld6: Move(_58.fld1.fld6),fld7: _34.fld5.fld7 };
_58.fld5 = [_16,_41.3];
_41.0 = core::ptr::addr_of!(place!(Field::<Adt71>(Variant(_19, 1), 1)).fld3.1);
(*_61) = Field::<u8>(Variant(_54.fld6, 1), 0) >> _34.fld3.0;
place!(Field::<u128>(Variant(_54.fld6, 1), 1)) = _14;
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld5.fld2 = _65.2.fld1.fld2 + _54.fld2;
_60.2 = &_58.fld5;
_65.3 = !_58.fld1.fld0;
_34.fld5.fld4 = (*_61) as i16;
_58.fld1.fld5 = !_42.0;
_60.2 = &_66;
_56.2 = core::ptr::addr_of_mut!(_65.2.fld1.fld5);
_49 = [_34.fld5.fld2,Field::<Adt71>(Variant(_19, 1), 1).fld5.fld2,Field::<u128>(Variant(_54.fld6, 1), 1)];
_58.fld1.fld7 = _54.fld7 & _34.fld5.fld7;
_60.0 = &_35;
match _54.fld0 {
0 => bb43,
7157814292196800720 => bb45,
_ => bb44
}
}
bb43 = {
_1 = 28_i8 as i16;
_1 = !22531_i16;
_14 = !305666364996363180508997454240254021858_u128;
_11 = _14 as f64;
_10 = &_14;
_15 = (*_10) as isize;
_11 = _8 as f64;
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{784c1}';
_10 = &(*_10);
_1 = 2586750184227097428_u64 as i16;
_22 = [633438533883164810_u64,2829137574498415661_u64,10633142348948362556_u64,5694947344130588068_u64,6801602390179607068_u64,17686338378907282995_u64];
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{4968c}';
_13 = _7 as u32;
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_15 = _18;
_5 = [(*_10),_14,(*_10),(*_10),_14,(*_10),_14];
_17 = 16425557164226109956_u64 as u32;
_6 = [_14,_14,_14,_14,(*_10),(*_10),(*_10)];
_13 = _17;
Call(_12 = fn9(_22, _4, _2, _3, _21, _5, _22, _15), ReturnTo(bb6), UnwindUnreachable())
}
bb44 = {
place!(Field::<char>(Variant(_19, 3), 0)) = '\u{ed04b}';
_21 = [(*_10),_14,(*_10),(*_10),(*_10),(*_10),_14];
_2 = !_7;
_23 = [_13,_13,_13,_13,_17,_17,_17,_13];
_11 = _13 as f64;
_17 = _13;
_23 = [_17,_17,_13,_17,_13,_13,_17,_17];
_25 = (-90_i8) + (-18_i8);
SetDiscriminant(_19, 3);
(*_24) = 145_u8;
_13 = _17;
_27 = 1158679077_i32 as u16;
_24 = core::ptr::addr_of_mut!((*_24));
(*_24) = 140_u8;
(*_24) = !13_u8;
_6 = [_14,(*_10),(*_10),(*_10),(*_10),_14,(*_10)];
Call(_21 = fn19(_2, _17, _7), ReturnTo(bb14), UnwindUnreachable())
}
bb45 = {
_68 = _33 == _29;
_60.3 = &place!(Field::<Adt71>(Variant(_19, 1), 1)).fld3;
_65.2.fld0 = core::ptr::addr_of_mut!(_58.fld1.fld1);
_56.2 = Move(_34.fld4);
_49 = _9;
_34.fld0 = !_36;
_24 = Move(_61);
_58.fld1.fld5 = !_63;
_60.1 = [_41.3,_41.3,_41.3,_16,_16,_16,_41.3,_41.3];
_17 = _58.fld1.fld1 + _54.fld1;
_58.fld4 = -_54.fld7;
_42.0 = _41.3 as u16;
_65.2.fld1.fld6 = Adt18::Variant1 { fld0: _8,fld1: _14 };
_34.fld4 = Move(_56.2);
_52.3 = Move(_60.3);
_44 = (_34.fld3.1,);
Call(_59 = core::intrinsics::transmute(_48), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
SetDiscriminant(_65.2.fld1.fld6, 0);
_58.fld1 = Adt29 { fld0: _34.fld5.fld0,fld1: _34.fld5.fld1,fld2: Field::<Adt71>(Variant(_19, 1), 1).fld5.fld2,fld3: _65.2.fld1.fld3,fld4: _54.fld4,fld5: _54.fld5,fld6: Move(_54.fld6),fld7: _65.2.fld1.fld7 };
_26 = -_44.0;
_56.2 = Move(_34.fld4);
_34.fld5.fld5 = _11 as u16;
_42.2 = _28 ^ _34.fld6;
place!(Field::<Adt71>(Variant(_19, 1), 1)).fld0 = _32 < (*_50);
(*_50) = _15;
_75.2 = &_42;
_19 = Adt73::Variant3 { fld0: _45 };
RET = core::ptr::addr_of_mut!(_71);
_70.1 = &_42.1;
_77.2 = core::ptr::addr_of!(_34.fld2);
_75.1.0 = _34.fld5.fld3 as f32;
_58.fld1.fld7 = _54.fld7 & _34.fld5.fld7;
_70.2 = Move(_56.2);
_12 = core::ptr::addr_of_mut!(_71);
_52.1 = [_16,_41.3,_41.3,_41.3,_16,_41.3,_16,_16];
_60.3 = &_34.fld3;
_58.fld0 = Move(_65.2.fld0);
Goto(bb47)
}
bb47 = {
Call(_78 = dump_var(4_usize, 37_usize, Move(_37), 63_usize, Move(_63), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_78 = dump_var(4_usize, 3_usize, Move(_3), 7_usize, Move(_7), 68_usize, Move(_68), 23_usize, Move(_23)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_78 = dump_var(4_usize, 6_usize, Move(_6), 15_usize, Move(_15), 29_usize, Move(_29), 25_usize, Move(_25)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_78 = dump_var(4_usize, 33_usize, Move(_33), 46_usize, Move(_46), 28_usize, Move(_28), 27_usize, Move(_27)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_78 = dump_var(4_usize, 35_usize, Move(_35), 39_usize, Move(_39), 16_usize, Move(_16), 79_usize, _79), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: u16,mut _2: [u128; 3],mut _3: u16) -> u16 {
mir! {
type RET = u16;
let _4: [char; 7];
let _5: f64;
let _6: i16;
let _7: *mut (f32,);
let _8: ();
let _9: ();
{
RET = !_1;
_1 = !RET;
_2 = [183411783391827873660865603724414849971_u128,105531358002295341101833945532911233969_u128,206125242142695370931936181205533563785_u128];
RET = !_1;
_1 = _3;
_4 = ['\u{e7b62}','\u{b16b9}','\u{ecc99}','\u{47d1f}','\u{9f9d0}','\u{d4de}','\u{ecac4}'];
_5 = 3545610299_u32 as f64;
_2 = [274886749907298544183722392270075712956_u128,291049338226001185715179691380349291040_u128,142591411235824239770362686741244971321_u128];
_2 = [177324391340680215461612390210436315069_u128,222557899959178314332531135987756577231_u128,216577940377856369343589020250961873726_u128];
_2 = [46158150912541402470971639152024762808_u128,166359321967471092307748458799992881894_u128,268577588615355175556288123090929324602_u128];
_2 = [116871583448805161324356086470667416047_u128,8085840346361627678976037068992184768_u128,42094443195766384793222669825117898635_u128];
_2 = [143142521309402343010387018263134298728_u128,62022240383431496982377975759169646551_u128,46300413855251681977677640796364719175_u128];
RET = !_1;
RET = (-39020258728826626705983331438461339433_i128) as u16;
_3 = _1 - _1;
_5 = 12551205069161730929_u64 as f64;
_1 = !_3;
_1 = !_3;
_3 = _1 << _1;
_1 = 209_u8 as u16;
Call(_1 = fn6(_3, _3, _3, _3, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = ['\u{63c9e}','\u{1050a7}','\u{cd8b1}','\u{811e6}','\u{3b080}','\u{97a21}','\u{ff7bc}'];
RET = _3 | _1;
_3 = RET;
_1 = _3 - RET;
RET = _3 & _1;
RET = _3;
_4 = ['\u{d68de}','\u{8223b}','\u{6f7f1}','\u{9a104}','\u{c1ca6}','\u{2d149}','\u{7d0d}'];
_6 = !(-32191_i16);
RET = !_3;
_4 = ['\u{f2fe2}','\u{6ee22}','\u{e4f08}','\u{2359a}','\u{f14f4}','\u{e7b7a}','\u{3bf7c}'];
_2 = [25997635052453765433024114978225617030_u128,272918347483507848409834020646536168483_u128,14826184649933225256705569129980796943_u128];
_3 = !RET;
RET = _3;
RET = (-106_isize) as u16;
_2 = [151793072563686319050990978820950284500_u128,101342293566729957099423048829648531671_u128,326673919822728377978972865393921930603_u128];
RET = _1 - _1;
_4 = ['\u{69e74}','\u{108c1b}','\u{1039b6}','\u{4aa82}','\u{14273}','\u{c608e}','\u{bc43c}'];
_4 = ['\u{9d7}','\u{63946}','\u{c0f7a}','\u{51a2}','\u{f25b0}','\u{107269}','\u{4af22}'];
_5 = _3 as f64;
RET = '\u{b9201}' as u16;
_3 = !_1;
_5 = 15609430609809658650_u64 as f64;
RET = _3;
_2 = [177221000202818083330592633603777776735_u128,133113689727342356967343336595510667376_u128,236483394559873624318272778820010881881_u128];
RET = !_1;
_3 = !RET;
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(5_usize, 4_usize, Move(_4), 6_usize, Move(_6), 9_usize, _9, 9_usize, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u16) -> u16 {
mir! {
type RET = u16;
let _7: ();
let _8: ();
{
RET = _4;
_6 = !RET;
_5 = '\u{1d492}' as u16;
RET = _3;
RET = !_3;
_3 = '\u{1a55d}' as u16;
_6 = _4 - _4;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(6_usize, 1_usize, Move(_1), 2_usize, Move(_2), 5_usize, Move(_5), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u16,mut _2: [u128; 7],mut _3: [bool; 8],mut _4: u16,mut _5: u16,mut _6: [u128; 7],mut _7: [bool; 8],mut _8: [u128; 7],mut _9: u16,mut _10: [bool; 8]) -> i16 {
mir! {
type RET = i16;
let _11: u32;
let _12: i8;
let _13: i128;
let _14: [bool; 5];
let _15: &'static (f32,);
let _16: ([bool; 8], (f32,), &'static (u16, i16, i64));
let _17: char;
let _18: (u16, f32);
let _19: &'static i8;
let _20: f64;
let _21: &'static i8;
let _22: u128;
let _23: Adt55;
let _24: *mut u32;
let _25: i32;
let _26: f32;
let _27: [i16; 3];
let _28: [u128; 7];
let _29: bool;
let _30: u64;
let _31: *mut isize;
let _32: *const [usize; 2];
let _33: &'static i128;
let _34: i64;
let _35: *mut u16;
let _36: ();
let _37: ();
{
Call(_7 = fn8(_3, _5, _6, _4, _5, _10, _10, _9, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [true,false,true,true,false,true,false,false];
_7 = [false,true,false,true,true,true,true,true];
RET = (-3487049408519417878_i64) as i16;
_6 = _8;
_4 = (-9223372036854775808_isize) as u16;
_11 = 882495797_u32 - 4032521292_u32;
_11 = RET as u32;
_8 = _6;
_11 = 2570619978_u32;
RET = !(-17774_i16);
_8 = [243498916899926592838747957041301268064_u128,96847416691730026962564359041879725952_u128,193486479255293993964760979322546892266_u128,48598638739567739760017628574838478491_u128,256539694101141533945373868339489512859_u128,7318430143083992590590953943351592070_u128,177329750306759430121884977042025332700_u128];
_4 = '\u{eecfc}' as u16;
_8 = [65038210005935602448699027242614575962_u128,326155300065222652277501343397539448368_u128,789202632127673870306841414076269213_u128,194661224626533032755757446146468371080_u128,133361583190665858246201243202857977689_u128,266420729581077804884875282678876425147_u128,177918540817062169404563885368595251169_u128];
_7 = _3;
_12 = !(-2_i8);
_8 = [322120698841035736446428854022427181898_u128,180243174542567309330158182000620972390_u128,282379867638980952505613646567472578742_u128,69846742688138804148912809583536818817_u128,106743606956028523103728969418841701833_u128,284660270792326274052024023602570082729_u128,117023817534211298358003711323637695166_u128];
_3 = [true,true,false,true,true,true,true,false];
_14 = [false,false,true,false,true];
_5 = (-23446035544342155055820008518080883664_i128) as u16;
_9 = _12 as u16;
_9 = _1 & _1;
_10 = [false,true,false,false,false,true,false,true];
_16.0 = [false,true,false,true,true,true,true,true];
_11 = false as u32;
Goto(bb2)
}
bb2 = {
RET = -8577_i16;
_15 = &_16.1;
_1 = _5;
_8 = _2;
_1 = !_9;
_15 = &(*_15);
_6 = _8;
_17 = '\u{dc067}';
_16.1.0 = RET as f32;
_1 = _9;
_16.0 = _10;
_1 = !_9;
_13 = _17 as i128;
RET = 21472_i16 - 14552_i16;
_15 = &_16.1;
_1 = _5;
_11 = 3264849725_u32;
_4 = 1677699703_i32 as u16;
_3 = [false,false,false,false,true,false,true,false];
_7 = _3;
_3 = [true,true,false,false,true,false,false,true];
_18.1 = _16.1.0;
_16.1.0 = _18.1;
_3 = [false,false,false,false,false,false,false,true];
_3 = _7;
match _11 {
0 => bb1,
3264849725 => bb4,
_ => bb3
}
}
bb3 = {
_3 = [true,false,true,true,false,true,false,false];
_7 = [false,true,false,true,true,true,true,true];
RET = (-3487049408519417878_i64) as i16;
_6 = _8;
_4 = (-9223372036854775808_isize) as u16;
_11 = 882495797_u32 - 4032521292_u32;
_11 = RET as u32;
_8 = _6;
_11 = 2570619978_u32;
RET = !(-17774_i16);
_8 = [243498916899926592838747957041301268064_u128,96847416691730026962564359041879725952_u128,193486479255293993964760979322546892266_u128,48598638739567739760017628574838478491_u128,256539694101141533945373868339489512859_u128,7318430143083992590590953943351592070_u128,177329750306759430121884977042025332700_u128];
_4 = '\u{eecfc}' as u16;
_8 = [65038210005935602448699027242614575962_u128,326155300065222652277501343397539448368_u128,789202632127673870306841414076269213_u128,194661224626533032755757446146468371080_u128,133361583190665858246201243202857977689_u128,266420729581077804884875282678876425147_u128,177918540817062169404563885368595251169_u128];
_7 = _3;
_12 = !(-2_i8);
_8 = [322120698841035736446428854022427181898_u128,180243174542567309330158182000620972390_u128,282379867638980952505613646567472578742_u128,69846742688138804148912809583536818817_u128,106743606956028523103728969418841701833_u128,284660270792326274052024023602570082729_u128,117023817534211298358003711323637695166_u128];
_3 = [true,true,false,true,true,true,true,false];
_14 = [false,false,true,false,true];
_5 = (-23446035544342155055820008518080883664_i128) as u16;
_9 = _12 as u16;
_9 = _1 & _1;
_10 = [false,true,false,false,false,true,false,true];
_16.0 = [false,true,false,true,true,true,true,true];
_11 = false as u32;
Goto(bb2)
}
bb4 = {
_1 = 8711924520684088941_usize as u16;
_3 = _10;
_15 = &_16.1;
_14 = [false,true,false,true,false];
_4 = RET as u16;
_8 = _6;
_18 = (_9, (*_15).0);
_18 = (_9, (*_15).0);
_20 = (*_15).0 as f64;
_18.1 = (*_15).0 - (*_15).0;
RET = (-6025_i16) * 4676_i16;
_19 = &_12;
_6 = [255088402687037401532174330247129490833_u128,91846082471453548298048659286864592867_u128,338531969243269842082376588244674588181_u128,323978115562512762174711046230508426728_u128,239368395902095572426632471747343107198_u128,28437872329465762075250774337393281273_u128,169555038400194125178800601189294080609_u128];
_16.0 = [false,false,true,true,true,true,true,true];
match _11 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
3264849725 => bb10,
_ => bb9
}
}
bb5 = {
_3 = [true,false,true,true,false,true,false,false];
_7 = [false,true,false,true,true,true,true,true];
RET = (-3487049408519417878_i64) as i16;
_6 = _8;
_4 = (-9223372036854775808_isize) as u16;
_11 = 882495797_u32 - 4032521292_u32;
_11 = RET as u32;
_8 = _6;
_11 = 2570619978_u32;
RET = !(-17774_i16);
_8 = [243498916899926592838747957041301268064_u128,96847416691730026962564359041879725952_u128,193486479255293993964760979322546892266_u128,48598638739567739760017628574838478491_u128,256539694101141533945373868339489512859_u128,7318430143083992590590953943351592070_u128,177329750306759430121884977042025332700_u128];
_4 = '\u{eecfc}' as u16;
_8 = [65038210005935602448699027242614575962_u128,326155300065222652277501343397539448368_u128,789202632127673870306841414076269213_u128,194661224626533032755757446146468371080_u128,133361583190665858246201243202857977689_u128,266420729581077804884875282678876425147_u128,177918540817062169404563885368595251169_u128];
_7 = _3;
_12 = !(-2_i8);
_8 = [322120698841035736446428854022427181898_u128,180243174542567309330158182000620972390_u128,282379867638980952505613646567472578742_u128,69846742688138804148912809583536818817_u128,106743606956028523103728969418841701833_u128,284660270792326274052024023602570082729_u128,117023817534211298358003711323637695166_u128];
_3 = [true,true,false,true,true,true,true,false];
_14 = [false,false,true,false,true];
_5 = (-23446035544342155055820008518080883664_i128) as u16;
_9 = _12 as u16;
_9 = _1 & _1;
_10 = [false,true,false,false,false,true,false,true];
_16.0 = [false,true,false,true,true,true,true,true];
_11 = false as u32;
Goto(bb2)
}
bb6 = {
RET = -8577_i16;
_15 = &_16.1;
_1 = _5;
_8 = _2;
_1 = !_9;
_15 = &(*_15);
_6 = _8;
_17 = '\u{dc067}';
_16.1.0 = RET as f32;
_1 = _9;
_16.0 = _10;
_1 = !_9;
_13 = _17 as i128;
RET = 21472_i16 - 14552_i16;
_15 = &_16.1;
_1 = _5;
_11 = 3264849725_u32;
_4 = 1677699703_i32 as u16;
_3 = [false,false,false,false,true,false,true,false];
_7 = _3;
_3 = [true,true,false,false,true,false,false,true];
_18.1 = _16.1.0;
_16.1.0 = _18.1;
_3 = [false,false,false,false,false,false,false,true];
_3 = _7;
match _11 {
0 => bb1,
3264849725 => bb4,
_ => bb3
}
}
bb7 = {
_3 = [true,false,true,true,false,true,false,false];
_7 = [false,true,false,true,true,true,true,true];
RET = (-3487049408519417878_i64) as i16;
_6 = _8;
_4 = (-9223372036854775808_isize) as u16;
_11 = 882495797_u32 - 4032521292_u32;
_11 = RET as u32;
_8 = _6;
_11 = 2570619978_u32;
RET = !(-17774_i16);
_8 = [243498916899926592838747957041301268064_u128,96847416691730026962564359041879725952_u128,193486479255293993964760979322546892266_u128,48598638739567739760017628574838478491_u128,256539694101141533945373868339489512859_u128,7318430143083992590590953943351592070_u128,177329750306759430121884977042025332700_u128];
_4 = '\u{eecfc}' as u16;
_8 = [65038210005935602448699027242614575962_u128,326155300065222652277501343397539448368_u128,789202632127673870306841414076269213_u128,194661224626533032755757446146468371080_u128,133361583190665858246201243202857977689_u128,266420729581077804884875282678876425147_u128,177918540817062169404563885368595251169_u128];
_7 = _3;
_12 = !(-2_i8);
_8 = [322120698841035736446428854022427181898_u128,180243174542567309330158182000620972390_u128,282379867638980952505613646567472578742_u128,69846742688138804148912809583536818817_u128,106743606956028523103728969418841701833_u128,284660270792326274052024023602570082729_u128,117023817534211298358003711323637695166_u128];
_3 = [true,true,false,true,true,true,true,false];
_14 = [false,false,true,false,true];
_5 = (-23446035544342155055820008518080883664_i128) as u16;
_9 = _12 as u16;
_9 = _1 & _1;
_10 = [false,true,false,false,false,true,false,true];
_16.0 = [false,true,false,true,true,true,true,true];
_11 = false as u32;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
RET = (-17625_i16);
_14 = [true,true,false,true,false];
_23.fld2 = _14;
_23.fld0 = core::ptr::addr_of_mut!(_23.fld1.fld1);
_21 = Move(_19);
_23.fld5 = [5_usize,14585207633186800473_usize];
_19 = &_23.fld3;
_23.fld1.fld2 = 58016324215918297149470106919722788918_u128;
_23.fld1.fld3 = _11 as i8;
_23.fld1.fld3 = _12 | _12;
Goto(bb11)
}
bb11 = {
_19 = &(*_19);
_17 = '\u{2af70}';
_15 = &_16.1;
_16.1 = (_18.1,);
_23.fld3 = _23.fld1.fld3 - _23.fld1.fld3;
_16.1.0 = -_18.1;
_18.1 = _16.1.0;
_23.fld1.fld3 = !_23.fld3;
_23.fld1.fld7 = _13 - _13;
RET = 27440_i16;
_22 = _23.fld1.fld2 << _18.0;
_7 = _3;
_23.fld1.fld2 = _22;
_23.fld1.fld1 = !_11;
_26 = _16.1.0;
_15 = &_16.1;
_13 = _23.fld1.fld7 & _23.fld1.fld7;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb10,
6 => bb7,
27440 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_20 = (-1886138073_i32) as f64;
_6 = _8;
_6 = [_22,_22,_22,_23.fld1.fld2,_22,_23.fld1.fld2,_23.fld1.fld2];
_28 = _2;
_23.fld1.fld4 = RET & RET;
_23.fld4 = _13 * _23.fld1.fld7;
_23.fld1.fld0 = _23.fld1.fld7 as u64;
_18 = (_9, _26);
_16.0 = _7;
_4 = _9 << _18.0;
_12 = (-115_isize) as i8;
_21 = &_23.fld3;
Goto(bb14)
}
bb14 = {
_9 = _4 >> _23.fld1.fld2;
_15 = &_16.1;
_1 = false as u16;
_24 = core::ptr::addr_of_mut!(_23.fld1.fld1);
_16.0 = [true,true,false,false,false,false,false,false];
_23.fld5 = [10547575183973781965_usize,17915513967240951211_usize];
_16.1 = (_18.1,);
_17 = '\u{66b4a}';
_23.fld1.fld6 = Adt18::Variant2 { fld0: _26,fld1: _17,fld2: _20,fld3: 160_u8,fld4: _23.fld1.fld4,fld5: (-89440871_i32),fld6: _23.fld1.fld0,fld7: _23.fld1.fld7 };
place!(Field::<i128>(Variant(_23.fld1.fld6, 2), 7)) = _23.fld4 - _23.fld4;
_21 = &_23.fld3;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(7_usize, 3_usize, Move(_3), 6_usize, Move(_6), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(7_usize, 28_usize, Move(_28), 10_usize, Move(_10), 4_usize, Move(_4), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [bool; 8],mut _2: u16,mut _3: [u128; 7],mut _4: u16,mut _5: u16,mut _6: [bool; 8],mut _7: [bool; 8],mut _8: u16,mut _9: u16) -> [bool; 8] {
mir! {
type RET = [bool; 8];
let _10: isize;
let _11: Adt71;
let _12: f64;
let _13: isize;
let _14: char;
let _15: ();
let _16: ();
{
_8 = _2 ^ _2;
_11.fld4 = core::ptr::addr_of_mut!(_4);
Goto(bb1)
}
bb1 = {
_2 = _8 + _8;
_11.fld2 = _3;
_11.fld0 = _5 < _8;
_5 = _2 & _2;
RET = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
RET = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_11.fld7 = [2_usize,4_usize];
_7 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13 = !(-9223372036854775808_isize);
_12 = (-51_i8) as f64;
_11.fld5.fld2 = 283301542158869962516050983589837281239_u128 * 127027947017057191804176392317049063542_u128;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(8_usize, 1_usize, Move(_1), 7_usize, Move(_7), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(8_usize, 4_usize, Move(_4), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [u64; 6],mut _2: u16,mut _3: u16,mut _4: [bool; 8],mut _5: [u128; 7],mut _6: [u128; 7],mut _7: [u64; 6],mut _8: isize) -> *mut [i8; 3] {
mir! {
type RET = *mut [i8; 3];
let _9: i8;
let _10: Adt18;
let _11: *mut u8;
let _12: &'static i128;
let _13: bool;
let _14: Adt29;
let _15: &'static (f32,);
let _16: i8;
let _17: [usize; 8];
let _18: u64;
let _19: isize;
let _20: u32;
let _21: isize;
let _22: [u64; 6];
let _23: char;
let _24: &'static bool;
let _25: [usize; 2];
let _26: *const f32;
let _27: [u128; 6];
let _28: &'static &'static f32;
let _29: Adt18;
let _30: i64;
let _31: isize;
let _32: f64;
let _33: f32;
let _34: &'static u128;
let _35: (f32,);
let _36: [u32; 8];
let _37: u64;
let _38: i8;
let _39: i8;
let _40: (*const f32, &'static (f32,), *const &'static (u16, f32), usize);
let _41: *const [u128; 7];
let _42: u32;
let _43: ([i8; 3], *mut ([bool; 8], (f32,), &'static (u16, i16, i64)), *const [u128; 7], &'static u16);
let _44: ();
let _45: ();
{
_3 = _2;
_3 = !_2;
_1 = [4930530995211925347_u64,8954946086024179071_u64,12601435879590750290_u64,4226761894067422969_u64,17613681322937470353_u64,9237490260153415883_u64];
_1 = [2654099512167781808_u64,3916472337781081083_u64,10083869230533613337_u64,1172324969723389376_u64,4524744204246920777_u64,13208331532306118357_u64];
_4 = [false,false,false,false,true,true,true,false];
Goto(bb1)
}
bb1 = {
_9 = (-29_i8) - 63_i8;
_5 = _6;
_1 = [4261278237060258074_u64,7909364472815235262_u64,10423305825037318142_u64,12150287350693452508_u64,4150850181172186970_u64,17388612445200234209_u64];
_4 = [true,false,false,false,false,false,true,true];
_7 = _1;
_6 = _5;
_2 = !_3;
_1 = [2558885227466818707_u64,16172911243438976395_u64,10245623731481845694_u64,9019164622481252505_u64,8122031963670472956_u64,3382707833673588071_u64];
Goto(bb2)
}
bb2 = {
_8 = (-9223372036854775808_isize);
_8 = (-9223372036854775808_isize) & 123_isize;
_4 = [false,true,false,true,true,true,false,false];
_5 = [38554020165552912611187702850622481710_u128,250204832810357071753014828624612136157_u128,211820023770986529326947793999498901254_u128,127454851163617659116914816491021932107_u128,297860056830380732277535501338727603998_u128,108196265590905934636473620061409766438_u128,126244868177980611165856066275227783659_u128];
_7 = [13695783564814878061_u64,15043027833973306968_u64,6246057828552178353_u64,13778007288349235189_u64,5065135292230782772_u64,1809276860565852560_u64];
_5 = [137037036642647135348886647058015465356_u128,190849623569486706930602377684789416733_u128,16051363885815352119272183998655761728_u128,245511600450738935983164178612851596071_u128,276906055014332078130542484069844651509_u128,312984339956177987592828435918159086189_u128,143117837382614223595613358570817246271_u128];
_2 = _3 & _3;
_7 = [6716654024454278142_u64,4001112798698988282_u64,12538076831657269938_u64,4758981723232650352_u64,6961799945021574801_u64,16452701399757352996_u64];
_5 = [144021808278219139806883944463360387699_u128,146637022440473232726776308906818580323_u128,339074054812508353236115021005018392286_u128,100475401847705181174400435978814724791_u128,189643443321316762904084148469695707297_u128,334216869316573768487937866281381123394_u128,263162796652725483649958748650645569450_u128];
_1 = _7;
_1 = _7;
_14.fld4 = (-24826_i16) | 28227_i16;
_1 = _7;
_14.fld2 = !107181378584150408483898464727759858581_u128;
_14.fld3 = _9 * _9;
_7 = [17916489740723518718_u64,15757357324819068124_u64,14092982217970508503_u64,16172105145085897240_u64,476530342902908460_u64,10141776941738185838_u64];
_14.fld2 = !254645825612852153502228964794927246182_u128;
_14.fld2 = 242828252971999885581091834941223870951_u128 * 289018326979201316075060063985423943017_u128;
_12 = &_14.fld7;
_1 = _7;
_14.fld7 = (-125820360056833198301904019566923401122_i128);
match _14.fld7 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
214462006864105265161470587864844810334 => bb8,
_ => bb7
}
}
bb3 = {
_9 = (-29_i8) - 63_i8;
_5 = _6;
_1 = [4261278237060258074_u64,7909364472815235262_u64,10423305825037318142_u64,12150287350693452508_u64,4150850181172186970_u64,17388612445200234209_u64];
_4 = [true,false,false,false,false,false,true,true];
_7 = _1;
_6 = _5;
_2 = !_3;
_1 = [2558885227466818707_u64,16172911243438976395_u64,10245623731481845694_u64,9019164622481252505_u64,8122031963670472956_u64,3382707833673588071_u64];
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
_14.fld0 = !1977439707444243702_u64;
_14.fld4 = (-28086_i16) ^ (-30412_i16);
match _14.fld7 {
0 => bb1,
1 => bb7,
214462006864105265161470587864844810334 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_14.fld1 = 1326452504_u32;
_3 = (-2740694974287571131_i64) as u16;
_14.fld3 = _9 >> _14.fld7;
_8 = -(-123_isize);
_14.fld5 = !_3;
_18 = true as u64;
_12 = &_14.fld7;
_14.fld2 = !321514751662686939957676236714187807095_u128;
_3 = !_2;
_17 = [3_usize,1_usize,5_usize,7_usize,4776416377691722795_usize,7_usize,2121160468371922647_usize,18008711780591035094_usize];
_16 = _14.fld3 - _9;
_20 = false as u32;
_14.fld4 = (-25614_i16) >> _3;
_13 = true | true;
_14.fld3 = _14.fld1 as i8;
_19 = 1266792636_i32 as isize;
_14.fld7 = 58784233695655167177500130958078811656_i128 + (-40269091051836669314883732304775757300_i128);
Goto(bb11)
}
bb11 = {
_8 = -_19;
_14.fld1 = _20;
Goto(bb12)
}
bb12 = {
_20 = _14.fld1;
_16 = _14.fld3 << _14.fld4;
_10 = Adt18::Variant1 { fld0: 225_u8,fld1: _14.fld2 };
_22 = _7;
_21 = !_8;
_24 = &_13;
place!(Field::<u8>(Variant(_10, 1), 0)) = 31_u8 << _14.fld4;
Goto(bb13)
}
bb13 = {
_19 = -_8;
Call(_21 = fn10(Move(_24), _14.fld4, Move(_10), _4, _1, _16, _3, _14.fld4, _3, _3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = _7;
_14.fld4 = 8820564274900397495_i64 as i16;
Call(_2 = core::intrinsics::bswap(_3), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Goto(bb16)
}
bb16 = {
_7 = [_14.fld0,_18,_14.fld0,_18,_18,_14.fld0];
_6 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_1 = [_14.fld0,_18,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
_14.fld3 = !_16;
_14.fld0 = _18;
_12 = &_14.fld7;
_17 = [0_usize,1_usize,14238573715893534013_usize,8559852906581585836_usize,2_usize,2_usize,6_usize,3_usize];
_2 = _3;
_14.fld0 = _13 as u64;
_16 = !_14.fld3;
_13 = !true;
_25 = [1_usize,12860477126933041546_usize];
_6 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_27 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_10 = Adt18::Variant1 { fld0: 159_u8,fld1: _14.fld2 };
_14.fld2 = Field::<u128>(Variant(_10, 1), 1);
_23 = '\u{5a1ff}';
_4 = [_13,_13,_13,_13,_13,_13,_13,_13];
_19 = _21 >> _14.fld3;
_9 = _14.fld3 + _16;
Goto(bb17)
}
bb17 = {
_3 = 114_u8 as u16;
_22 = [_14.fld0,_14.fld0,_14.fld0,_18,_18,_18];
_12 = &(*_12);
_12 = &_14.fld7;
_14.fld7 = _13 as i128;
_11 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_10, 1), 0)));
_14.fld5 = _2;
_9 = _16 - _16;
_19 = _21;
_12 = &_14.fld7;
_20 = !_14.fld1;
_13 = _19 <= _19;
_3 = !_14.fld5;
_7 = [_14.fld0,_14.fld0,_14.fld0,_18,_14.fld0,_18];
_16 = _14.fld3;
place!(Field::<u8>(Variant(_10, 1), 0)) = 172_u8;
_13 = !true;
_14.fld6 = Adt18::Variant1 { fld0: (*_11),fld1: _14.fld2 };
_18 = _23 as u64;
Call(_14.fld2 = core::intrinsics::bswap(Field::<u128>(Variant(_14.fld6, 1), 1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_13 = false;
_1 = [_18,_14.fld0,_18,_14.fld0,_18,_14.fld0];
_27 = [Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1)];
_14.fld3 = _9 << _9;
_14.fld7 = 33194242894321691948562077691618626585_i128 & 109240585821121784119551281881100951647_i128;
_5 = _6;
_17 = [6233694268752187850_usize,9451094314396966076_usize,1_usize,17742900682553993848_usize,10048897383744266096_usize,4465087115768188767_usize,1_usize,7836754445709077900_usize];
place!(Field::<u8>(Variant(_10, 1), 0)) = Field::<u8>(Variant(_14.fld6, 1), 0);
_14.fld0 = _18;
_14.fld7 = _14.fld3 as i128;
_3 = !_14.fld5;
_16 = -_9;
_14.fld7 = 142775077569140443152668319441374032888_i128;
_3 = _14.fld5 & _2;
_4 = [_13,_13,_13,_13,_13,_13,_13,_13];
_25 = [1_usize,579864682472976008_usize];
_4 = [_13,_13,_13,_13,_13,_13,_13,_13];
Goto(bb19)
}
bb19 = {
_5 = [Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_10, 1), 1)];
_10 = Move(_14.fld6);
_12 = &_14.fld7;
_30 = 2287892143564416042_i64;
SetDiscriminant(_10, 1);
_23 = '\u{56ab2}';
_24 = &_13;
_31 = _19 << _2;
_31 = !_19;
_2 = _3;
_7 = _1;
_33 = _18 as f32;
_12 = &(*_12);
_11 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_10, 1), 0)));
_14.fld5 = _31 as u16;
_12 = &(*_12);
_8 = -_21;
_35.0 = _33;
_14.fld6 = Adt18::Variant1 { fld0: 208_u8,fld1: _14.fld2 };
(*_11) = 117_u8;
place!(Field::<u8>(Variant(_14.fld6, 1), 0)) = !Field::<u8>(Variant(_10, 1), 0);
_33 = _35.0;
_14.fld5 = _30 as u16;
_14.fld1 = _20 & _20;
_36 = [_14.fld1,_14.fld1,_14.fld1,_20,_14.fld1,_20,_20,_14.fld1];
match Field::<u8>(Variant(_10, 1), 0) {
0 => bb6,
1 => bb2,
2 => bb14,
3 => bb20,
4 => bb21,
117 => bb23,
_ => bb22
}
}
bb20 = {
_14.fld0 = !1977439707444243702_u64;
_14.fld4 = (-28086_i16) ^ (-30412_i16);
match _14.fld7 {
0 => bb1,
1 => bb7,
214462006864105265161470587864844810334 => bb10,
_ => bb9
}
}
bb21 = {
_19 = -_8;
Call(_21 = fn10(Move(_24), _14.fld4, Move(_10), _4, _1, _16, _3, _14.fld4, _3, _3), ReturnTo(bb14), UnwindUnreachable())
}
bb22 = {
_7 = [_14.fld0,_18,_14.fld0,_18,_18,_14.fld0];
_6 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_1 = [_14.fld0,_18,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
_14.fld3 = !_16;
_14.fld0 = _18;
_12 = &_14.fld7;
_17 = [0_usize,1_usize,14238573715893534013_usize,8559852906581585836_usize,2_usize,2_usize,6_usize,3_usize];
_2 = _3;
_14.fld0 = _13 as u64;
_16 = !_14.fld3;
_13 = !true;
_25 = [1_usize,12860477126933041546_usize];
_6 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_27 = [_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2,_14.fld2];
_10 = Adt18::Variant1 { fld0: 159_u8,fld1: _14.fld2 };
_14.fld2 = Field::<u128>(Variant(_10, 1), 1);
_23 = '\u{5a1ff}';
_4 = [_13,_13,_13,_13,_13,_13,_13,_13];
_19 = _21 >> _14.fld3;
_9 = _14.fld3 + _16;
Goto(bb17)
}
bb23 = {
_14.fld1 = _14.fld3 as u32;
_18 = !_14.fld0;
place!(Field::<u8>(Variant(_10, 1), 0)) = Field::<u8>(Variant(_14.fld6, 1), 0);
_35.0 = _33;
_15 = &_35;
_7 = [_18,_18,_14.fld0,_14.fld0,_14.fld0,_18];
_14.fld5 = _2;
(*_11) = Field::<u8>(Variant(_14.fld6, 1), 0);
_14.fld3 = -_16;
place!(Field::<u128>(Variant(_10, 1), 1)) = _14.fld2 * Field::<u128>(Variant(_14.fld6, 1), 1);
_14.fld1 = _20 & _20;
_15 = &_35;
_27 = [Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_14.fld6, 1), 1),Field::<u128>(Variant(_10, 1), 1)];
_26 = core::ptr::addr_of!(_35.0);
place!(Field::<u8>(Variant(_10, 1), 0)) = !Field::<u8>(Variant(_14.fld6, 1), 0);
_27 = [Field::<u128>(Variant(_10, 1), 1),_14.fld2,_14.fld2,Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1),Field::<u128>(Variant(_10, 1), 1)];
_33 = _35.0 * (*_15).0;
match _30 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb24,
2287892143564416042 => bb26,
_ => bb25
}
}
bb24 = {
_14.fld0 = !1977439707444243702_u64;
_14.fld4 = (-28086_i16) ^ (-30412_i16);
match _14.fld7 {
0 => bb1,
1 => bb7,
214462006864105265161470587864844810334 => bb10,
_ => bb9
}
}
bb25 = {
_19 = -_8;
Call(_21 = fn10(Move(_24), _14.fld4, Move(_10), _4, _1, _16, _3, _14.fld4, _3, _3), ReturnTo(bb14), UnwindUnreachable())
}
bb26 = {
_7 = _1;
_17 = [2443256178793922799_usize,1401491047228372537_usize,6_usize,1_usize,11738294698691220361_usize,10465359199762246097_usize,5_usize,6_usize];
_1 = [_14.fld0,_18,_18,_18,_18,_18];
Goto(bb27)
}
bb27 = {
place!(Field::<u8>(Variant(_14.fld6, 1), 0)) = (-551493328_i32) as u8;
_36 = [_20,_14.fld1,_14.fld1,_20,_14.fld1,_14.fld1,_14.fld1,_14.fld1];
_15 = &_35;
_14.fld6 = Move(_10);
_12 = &_14.fld7;
_15 = &(*_15);
place!(Field::<u8>(Variant(_14.fld6, 1), 0)) = _14.fld1 as u8;
_12 = &_14.fld7;
_33 = _14.fld4 as f32;
_14.fld3 = 786052858_i32 as i8;
_31 = -_19;
_20 = _14.fld1;
_9 = _16 >> _31;
_42 = Field::<u8>(Variant(_14.fld6, 1), 0) as u32;
_14.fld2 = Field::<u128>(Variant(_14.fld6, 1), 1);
_1 = [_18,_14.fld0,_18,_14.fld0,_18,_14.fld0];
(*_11) = Field::<u8>(Variant(_14.fld6, 1), 0) * Field::<u8>(Variant(_14.fld6, 1), 0);
RET = core::ptr::addr_of_mut!(_43.0);
_14.fld6 = Adt18::Variant1 { fld0: (*_11),fld1: _14.fld2 };
Goto(bb28)
}
bb28 = {
Call(_44 = dump_var(9_usize, 6_usize, Move(_6), 13_usize, Move(_13), 19_usize, Move(_19), 30_usize, Move(_30)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_44 = dump_var(9_usize, 5_usize, Move(_5), 21_usize, Move(_21), 22_usize, Move(_22), 36_usize, Move(_36)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_44 = dump_var(9_usize, 4_usize, Move(_4), 3_usize, Move(_3), 1_usize, Move(_1), 42_usize, Move(_42)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: &'static bool,mut _2: i16,mut _3: Adt18,mut _4: [bool; 8],mut _5: [u64; 6],mut _6: i8,mut _7: u16,mut _8: i16,mut _9: u16,mut _10: u16) -> isize {
mir! {
type RET = isize;
let _11: [u128; 6];
let _12: u8;
let _13: (f32,);
let _14: [u64; 6];
let _15: Adt65;
let _16: isize;
let _17: u128;
let _18: [i16; 8];
let _19: u128;
let _20: f32;
let _21: bool;
let _22: isize;
let _23: *mut [i8; 3];
let _24: ([i8; 3], *mut ([bool; 8], (f32,), &'static (u16, i16, i64)), *const [u128; 7], &'static u16);
let _25: [char; 7];
let _26: [i16; 8];
let _27: *mut *const [u128; 7];
let _28: ();
let _29: ();
{
_5 = [4720297457853941697_u64,11607009226436905989_u64,7102314803547531360_u64,4160796538190904631_u64,15403270050116320210_u64,1027065116478959887_u64];
place!(Field::<u8>(Variant(_3, 1), 0)) = 115289709162485817111174151059625452014_i128 as u8;
_8 = -_2;
RET = -9223372036854775807_isize;
RET = 83_isize;
_13.0 = Field::<u128>(Variant(_3, 1), 1) as f32;
_12 = RET as u8;
_5 = [9194298431982482972_u64,3576597755611256789_u64,1962728591158293502_u64,15156340325776840498_u64,8357771994394839838_u64,15827607942910075067_u64];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
83 => bb6,
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
place!(Field::<u8>(Variant(_3, 1), 0)) = _12;
place!(Field::<u8>(Variant(_3, 1), 0)) = _12 | _12;
_11 = [Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1)];
_8 = -_2;
_2 = -_8;
_2 = (-533838209492249357_i64) as i16;
Call(_10 = fn11(_6, _2, _6, _7, _9, _4, _7, _7, _9, _9, _8, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = !_10;
_10 = _7;
_6 = 107_i8;
_2 = 18227779991947266690_u64 as i16;
_10 = _9 << _8;
_6 = -(-36_i8);
Call(_6 = core::intrinsics::bswap(51_i8), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14 = _5;
place!(Field::<u8>(Variant(_3, 1), 0)) = _12 >> _9;
_14 = [10842460974276648059_u64,4189761131782236314_u64,10385487499289153816_u64,12404830635850007767_u64,1347424973214262373_u64,12032379486704833950_u64];
Goto(bb9)
}
bb9 = {
place!(Field::<u8>(Variant(_3, 1), 0)) = _12 << _10;
_8 = _2 >> _7;
_4 = [false,true,false,false,false,false,true,false];
_4 = [false,false,true,false,true,true,false,false];
_10 = _7;
_8 = false as i16;
_8 = _2;
_11 = [Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1)];
_14 = [2504776057845557897_u64,1622828525124042025_u64,18377006384026530252_u64,14671192868652086229_u64,420650294876631943_u64,1852353708327627201_u64];
_5 = [9013220679814720445_u64,14366149101528343007_u64,12153292934881519680_u64,7387640077783337627_u64,12790318569526705877_u64,18281695590689103058_u64];
_7 = _10 >> _10;
_16 = RET;
place!(Field::<u8>(Variant(_3, 1), 0)) = _9 as u8;
_18 = [_8,_8,_8,_2,_2,_8,_2,_2];
_2 = _8;
_11 = [Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1),Field::<u128>(Variant(_3, 1), 1)];
_19 = Field::<u128>(Variant(_3, 1), 1);
SetDiscriminant(_3, 2);
place!(Field::<u64>(Variant(_3, 2), 6)) = _6 as u64;
match RET {
0 => bb2,
1 => bb10,
83 => bb12,
_ => bb11
}
}
bb10 = {
_14 = _5;
place!(Field::<u8>(Variant(_3, 1), 0)) = _12 >> _9;
_14 = [10842460974276648059_u64,4189761131782236314_u64,10385487499289153816_u64,12404830635850007767_u64,1347424973214262373_u64,12032379486704833950_u64];
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
RET = _16;
place!(Field::<i128>(Variant(_3, 2), 7)) = -(-140611250703009188287372583470847241416_i128);
_12 = !83_u8;
_17 = _2 as u128;
_13.0 = _10 as f32;
_5 = _14;
_11 = [_19,_19,_19,_17,_17,_17];
_8 = _2;
place!(Field::<i16>(Variant(_3, 2), 4)) = _2;
place!(Field::<i16>(Variant(_3, 2), 4)) = _2 << _10;
_8 = _12 as i16;
place!(Field::<i16>(Variant(_3, 2), 4)) = -_8;
_8 = _2 + _2;
_10 = !_9;
place!(Field::<f32>(Variant(_3, 2), 0)) = Field::<i128>(Variant(_3, 2), 7) as f32;
place!(Field::<u64>(Variant(_3, 2), 6)) = !12848137288636856955_u64;
_6 = 21_i8;
_13 = (Field::<f32>(Variant(_3, 2), 0),);
place!(Field::<f64>(Variant(_3, 2), 2)) = 2367128775_u32 as f64;
Call(_3 = fn12(_9, _5, _7, _4, _9, _5, _4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_11 = [_17,_17,_17,_19,_19,_17];
_18 = [Field::<i16>(Variant(_3, 2), 4),_8,Field::<i16>(Variant(_3, 2), 4),_8,_8,Field::<i16>(Variant(_3, 2), 4),_8,Field::<i16>(Variant(_3, 2), 4)];
_14 = [Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6)];
_17 = _19;
_6 = (-94_i8) & (-36_i8);
_5 = _14;
place!(Field::<u8>(Variant(_3, 2), 3)) = _12;
_7 = !_10;
place!(Field::<f64>(Variant(_3, 2), 2)) = Field::<i128>(Variant(_3, 2), 7) as f64;
_22 = RET;
_20 = Field::<i32>(Variant(_3, 2), 5) as f32;
_13.0 = Field::<f32>(Variant(_3, 2), 0) - _20;
place!(Field::<u64>(Variant(_3, 2), 6)) = !14952673282336567333_u64;
place!(Field::<char>(Variant(_3, 2), 1)) = '\u{528c2}';
_8 = !Field::<i16>(Variant(_3, 2), 4);
RET = _22;
_22 = _16 | RET;
_22 = _16;
_9 = _10;
_20 = Field::<f32>(Variant(_3, 2), 0) * _13.0;
_7 = _9 - _9;
_13 = (_20,);
_21 = true;
RET = _16 >> _10;
_21 = Field::<i16>(Variant(_3, 2), 4) > _8;
_5 = [Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6),Field::<u64>(Variant(_3, 2), 6)];
_22 = RET << _10;
Goto(bb14)
}
bb14 = {
Call(_28 = dump_var(10_usize, 6_usize, Move(_6), 10_usize, Move(_10), 14_usize, Move(_14), 22_usize, Move(_22)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_28 = dump_var(10_usize, 9_usize, Move(_9), 19_usize, Move(_19), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i8,mut _2: i16,mut _3: i8,mut _4: u16,mut _5: u16,mut _6: [bool; 8],mut _7: u16,mut _8: u16,mut _9: u16,mut _10: u16,mut _11: i16,mut _12: i16) -> u16 {
mir! {
type RET = u16;
let _13: *const [u128; 7];
let _14: isize;
let _15: Adt55;
let _16: ();
let _17: ();
{
_3 = !_1;
_5 = 9223372036854775807_isize as u16;
_8 = !_7;
_12 = 792973288_u32 as i16;
_2 = true as i16;
_11 = 11218274917483911320_u64 as i16;
_5 = _7;
_6 = [false,false,false,true,true,false,true,false];
_7 = _9 << _3;
_11 = _2;
_11 = 10523175918691627647103310617210538026_i128 as i16;
_8 = _1 as u16;
_3 = _1 >> _9;
_2 = !_12;
Goto(bb1)
}
bb1 = {
_7 = _10;
_9 = _4 << _8;
_15.fld1.fld7 = (-97037359633789699345439739137749060333_i128) << _9;
RET = _5;
_4 = _10;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(11_usize, 1_usize, Move(_1), 10_usize, Move(_10), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_16 = dump_var(11_usize, 8_usize, Move(_8), 12_usize, Move(_12), 17_usize, _17, 17_usize, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u16,mut _2: [u64; 6],mut _3: u16,mut _4: [bool; 8],mut _5: u16,mut _6: [u64; 6],mut _7: [bool; 8]) -> Adt18 {
mir! {
type RET = Adt18;
let _8: &'static u16;
let _9: *mut ([bool; 8], (f32,), &'static (u16, i16, i64));
let _10: (f64,);
let _11: f32;
let _12: [i16; 3];
let _13: [u32; 1];
let _14: isize;
let _15: i8;
let _16: i64;
let _17: [u64; 6];
let _18: (f64,);
let _19: isize;
let _20: i64;
let _21: i128;
let _22: isize;
let _23: Adt46;
let _24: (f64,);
let _25: &'static (f32,);
let _26: &'static u128;
let _27: &'static i128;
let _28: (f64,);
let _29: usize;
let _30: *const [u128; 7];
let _31: isize;
let _32: Adt18;
let _33: &'static i16;
let _34: isize;
let _35: Adt55;
let _36: *const [u128; 7];
let _37: u64;
let _38: char;
let _39: *const f64;
let _40: u64;
let _41: Adt73;
let _42: i64;
let _43: Adt18;
let _44: f64;
let _45: u64;
let _46: [u128; 6];
let _47: f64;
let _48: i128;
let _49: f64;
let _50: *const &'static (u16, f32);
let _51: f32;
let _52: f64;
let _53: ();
let _54: ();
{
_3 = !_1;
_4 = [true,true,true,false,true,true,false,false];
_2 = [3089376925242743316_u64,8558567878487229991_u64,12829385864292464706_u64,8594132826068277522_u64,4651610412791331635_u64,10971422734852257094_u64];
_1 = _3 | _5;
_2 = _6;
RET = Adt18::Variant1 { fld0: 214_u8,fld1: 180004821295831370985233692353608339207_u128 };
place!(Field::<u128>(Variant(RET, 1), 1)) = 291936524704665437887792609606881679734_u128;
_7 = _4;
RET = Adt18::Variant1 { fld0: 23_u8,fld1: 51609845427600424129135828633670160348_u128 };
_1 = _3;
_7 = _4;
Goto(bb1)
}
bb1 = {
_8 = &_3;
place!(Field::<u128>(Variant(RET, 1), 1)) = !174366674968579965415923520953765075588_u128;
_6 = [2887030627727263004_u64,17537005246271530721_u64,12280180259048236241_u64,5430210536756460568_u64,9011638458741336216_u64,17407426313591853196_u64];
_7 = _4;
place!(Field::<u8>(Variant(RET, 1), 0)) = !64_u8;
_2 = _6;
_8 = &_5;
place!(Field::<u128>(Variant(RET, 1), 1)) = !174722608132436137664362797420081612271_u128;
_3 = _5;
_7 = _4;
_5 = !_1;
_1 = _3;
_2 = [15532231627009653118_u64,18356262312194668065_u64,12013391964680997737_u64,2068039030801327445_u64,1500496309758549916_u64,15078177827382250797_u64];
place!(Field::<u8>(Variant(RET, 1), 0)) = 34_u8 >> _5;
RET = Adt18::Variant1 { fld0: 249_u8,fld1: 21666094717183242859022728250427145071_u128 };
_5 = 961889631_i32 as u16;
_2 = [13669352788553427294_u64,2973868548374610622_u64,10084263798819228311_u64,6262233752084413989_u64,1027083741405357388_u64,4348906095489933009_u64];
_2 = _6;
Goto(bb2)
}
bb2 = {
place!(Field::<u8>(Variant(RET, 1), 0)) = _3 as u8;
_7 = _4;
_1 = _3 | _3;
place!(Field::<u8>(Variant(RET, 1), 0)) = !54_u8;
_5 = !_1;
_6 = [2049512203771482959_u64,16073529522130584513_u64,13382344377131109179_u64,619247931852184919_u64,17028939653217384826_u64,13811164028231235606_u64];
_10.0 = _1 as f64;
place!(Field::<u128>(Variant(RET, 1), 1)) = 2733124456166934423_i64 as u128;
place!(Field::<u128>(Variant(RET, 1), 1)) = 267246591866817016882538495547228518819_u128 >> _1;
_5 = _1;
place!(Field::<u128>(Variant(RET, 1), 1)) = 123796259440044434643493236223665793561_u128 & 17190091124181572733121497650117970035_u128;
place!(Field::<u128>(Variant(RET, 1), 1)) = 228097732949106425252632610653362146240_u128;
_6 = _2;
_13 = [3446299956_u32];
_5 = false as u16;
place!(Field::<u128>(Variant(RET, 1), 1)) = 268651963259045323284415377166030235828_u128 - 131165624185322963259305089156215815543_u128;
_3 = _1 ^ _1;
_6 = [16616871065623544469_u64,2617785376530687181_u64,12555049750159149282_u64,5414313585655852437_u64,2669175319109963616_u64,2082998545720078671_u64];
_7 = [false,false,false,true,true,false,true,false];
_11 = 2114929744486066405_u64 as f32;
place!(Field::<u8>(Variant(RET, 1), 0)) = 47_u8;
place!(Field::<u128>(Variant(RET, 1), 1)) = 59443395216708999177626514617024674867_u128;
_7 = [true,true,true,false,false,false,true,false];
place!(Field::<u8>(Variant(RET, 1), 0)) = 104_u8;
Call(_14 = fn13(_10, _3, _1, _3, _3, _10, _10.0, _3, _3, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10.0 = Field::<u8>(Variant(RET, 1), 0) as f64;
_6 = _2;
_8 = &_3;
RET = Adt18::Variant1 { fld0: 11_u8,fld1: 309240723612254042675449781409568014728_u128 };
RET = Adt18::Variant1 { fld0: 84_u8,fld1: 292360949740832786451203051847527647138_u128 };
_2 = [6559483420524390259_u64,16147606216574734670_u64,3308003055886407676_u64,181952113997576849_u64,1805862787757370343_u64,9250489748681741377_u64];
_1 = 111_i8 as u16;
_3 = '\u{66638}' as u16;
_3 = _5;
_14 = !(-126_isize);
_4 = [true,false,true,false,false,true,true,false];
_8 = &_1;
_4 = _7;
_12 = [(-29032_i16),27263_i16,30182_i16];
place!(Field::<u128>(Variant(RET, 1), 1)) = !246322133210819453492443888621016687581_u128;
RET = Adt18::Variant2 { fld0: _11,fld1: '\u{af79b}',fld2: _10.0,fld3: 97_u8,fld4: 7370_i16,fld5: 1686197801_i32,fld6: 3343460433877239796_u64,fld7: (-17089457777702621526789966083547612070_i128) };
place!(Field::<i128>(Variant(RET, 2), 7)) = 83651316742988406456388270064282127240_i128 & (-94921649004979459503237768186475618073_i128);
_10 = (Field::<f64>(Variant(RET, 2), 2),);
place!(Field::<u8>(Variant(RET, 2), 3)) = 61_u8;
_8 = &(*_8);
place!(Field::<u64>(Variant(RET, 2), 6)) = 15378349056942546400_u64;
match Field::<u64>(Variant(RET, 2), 6) {
15378349056942546400 => bb5,
_ => bb4
}
}
bb4 = {
_8 = &_3;
place!(Field::<u128>(Variant(RET, 1), 1)) = !174366674968579965415923520953765075588_u128;
_6 = [2887030627727263004_u64,17537005246271530721_u64,12280180259048236241_u64,5430210536756460568_u64,9011638458741336216_u64,17407426313591853196_u64];
_7 = _4;
place!(Field::<u8>(Variant(RET, 1), 0)) = !64_u8;
_2 = _6;
_8 = &_5;
place!(Field::<u128>(Variant(RET, 1), 1)) = !174722608132436137664362797420081612271_u128;
_3 = _5;
_7 = _4;
_5 = !_1;
_1 = _3;
_2 = [15532231627009653118_u64,18356262312194668065_u64,12013391964680997737_u64,2068039030801327445_u64,1500496309758549916_u64,15078177827382250797_u64];
place!(Field::<u8>(Variant(RET, 1), 0)) = 34_u8 >> _5;
RET = Adt18::Variant1 { fld0: 249_u8,fld1: 21666094717183242859022728250427145071_u128 };
_5 = 961889631_i32 as u16;
_2 = [13669352788553427294_u64,2973868548374610622_u64,10084263798819228311_u64,6262233752084413989_u64,1027083741405357388_u64,4348906095489933009_u64];
_2 = _6;
Goto(bb2)
}
bb5 = {
_10 = (Field::<f64>(Variant(RET, 2), 2),);
place!(Field::<i128>(Variant(RET, 2), 7)) = '\u{10bb4}' as i128;
RET = Adt18::Variant1 { fld0: 44_u8,fld1: 81632352094871302097963218203017101260_u128 };
place!(Field::<u8>(Variant(RET, 1), 0)) = 105_u8;
_18.0 = (-15540_i16) as f64;
_13 = [2121925201_u32];
_12 = [24112_i16,16577_i16,(-12631_i16)];
_16 = _18.0 as i64;
_4 = [false,false,false,true,false,false,false,false];
_8 = &_1;
_11 = _16 as f32;
_16 = 1686092629372716467_i64 * (-4559153017670775121_i64);
_19 = Field::<u8>(Variant(RET, 1), 0) as isize;
place!(Field::<u128>(Variant(RET, 1), 1)) = !16796469191060453181546075510106093470_u128;
place!(Field::<u128>(Variant(RET, 1), 1)) = (-1819167477_i32) as u128;
_18.0 = _1 as f64;
_12 = [(-25980_i16),14177_i16,12116_i16];
_22 = -_19;
_10.0 = _18.0;
_18 = (_10.0,);
_5 = (*_8) ^ _1;
match Field::<u8>(Variant(RET, 1), 0) {
105 => bb7,
_ => bb6
}
}
bb6 = {
_8 = &_3;
place!(Field::<u128>(Variant(RET, 1), 1)) = !174366674968579965415923520953765075588_u128;
_6 = [2887030627727263004_u64,17537005246271530721_u64,12280180259048236241_u64,5430210536756460568_u64,9011638458741336216_u64,17407426313591853196_u64];
_7 = _4;
place!(Field::<u8>(Variant(RET, 1), 0)) = !64_u8;
_2 = _6;
_8 = &_5;
place!(Field::<u128>(Variant(RET, 1), 1)) = !174722608132436137664362797420081612271_u128;
_3 = _5;
_7 = _4;
_5 = !_1;
_1 = _3;
_2 = [15532231627009653118_u64,18356262312194668065_u64,12013391964680997737_u64,2068039030801327445_u64,1500496309758549916_u64,15078177827382250797_u64];
place!(Field::<u8>(Variant(RET, 1), 0)) = 34_u8 >> _5;
RET = Adt18::Variant1 { fld0: 249_u8,fld1: 21666094717183242859022728250427145071_u128 };
_5 = 961889631_i32 as u16;
_2 = [13669352788553427294_u64,2973868548374610622_u64,10084263798819228311_u64,6262233752084413989_u64,1027083741405357388_u64,4348906095489933009_u64];
_2 = _6;
Goto(bb2)
}
bb7 = {
_8 = &(*_8);
_24 = _18;
place!(Field::<u128>(Variant(RET, 1), 1)) = 286208291654704149367974911357672417968_u128 * 296118634608466485251212734531548890350_u128;
place!(Field::<u128>(Variant(RET, 1), 1)) = !22831447207439200594870805215283724268_u128;
_18 = (_24.0,);
_24 = (_10.0,);
place!(Field::<u8>(Variant(RET, 1), 0)) = 6_u8 & 71_u8;
_10 = (_24.0,);
_3 = _1 >> _16;
_10.0 = _24.0;
_11 = 1783091674_i32 as f32;
place!(Field::<u8>(Variant(RET, 1), 0)) = 3_u8;
_10 = (_18.0,);
_13 = [584407948_u32];
_10 = (_18.0,);
place!(Field::<u8>(Variant(RET, 1), 0)) = (-17_i8) as u8;
_11 = _24.0 as f32;
place!(Field::<u128>(Variant(RET, 1), 1)) = 129631782491832252627574061829112381772_u128;
_10.0 = _24.0;
_6 = [1260284533941045671_u64,1640433676896439893_u64,7936746306501512588_u64,8123618150481686238_u64,9309941148936762346_u64,3534578590178665193_u64];
_18 = (_24.0,);
match Field::<u128>(Variant(RET, 1), 1) {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
129631782491832252627574061829112381772 => bb8,
_ => bb5
}
}
bb8 = {
_20 = 2_usize as i64;
_17 = [14896144291309942714_u64,5544527629184216665_u64,2291695088556895565_u64,2566215768993822669_u64,1217757697422521501_u64,14930064003036856297_u64];
_17 = [8331819341274176075_u64,5170203421579867071_u64,9861973285379460700_u64,10503277996854858127_u64,4303893036628336006_u64,11720666672403090036_u64];
SetDiscriminant(RET, 2);
RET = Adt18::Variant1 { fld0: 87_u8,fld1: 40272602269028413417005866557507956157_u128 };
_13 = [2979883563_u32];
place!(Field::<u8>(Variant(RET, 1), 0)) = 37_u8;
place!(Field::<u128>(Variant(RET, 1), 1)) = 231108207012818557468825640419796224883_u128;
_6 = [4280110638450509523_u64,2325997685823195221_u64,11192177232638745747_u64,4364567703516088918_u64,838431728024235225_u64,17009526439554133697_u64];
_1 = _3 << _5;
_27 = &_21;
_11 = (-35767671430727723084145801286258783559_i128) as f32;
_17 = _2;
SetDiscriminant(RET, 2);
place!(Field::<i128>(Variant(RET, 2), 7)) = 191_u8 as i128;
_10 = (_18.0,);
_13 = [3792193950_u32];
_21 = 111855250186306430579360898469964675542_u128 as i128;
Call(_20 = core::intrinsics::transmute(_4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<f64>(Variant(RET, 2), 2)) = _10.0;
place!(Field::<i32>(Variant(RET, 2), 5)) = 873822577_i32 ^ 685519199_i32;
_22 = _19 & _14;
place!(Field::<i16>(Variant(RET, 2), 4)) = 6968_i16 * 5393_i16;
_12 = [Field::<i16>(Variant(RET, 2), 4),Field::<i16>(Variant(RET, 2), 4),Field::<i16>(Variant(RET, 2), 4)];
_14 = _22;
_35.fld1.fld7 = _19 as i128;
Goto(bb10)
}
bb10 = {
place!(Field::<i128>(Variant(RET, 2), 7)) = _21;
_31 = 237_u8 as isize;
_35.fld1.fld3 = !93_i8;
_34 = _31;
place!(Field::<f64>(Variant(RET, 2), 2)) = -_10.0;
place!(Field::<f32>(Variant(RET, 2), 0)) = 3572846466_u32 as f32;
Goto(bb11)
}
bb11 = {
_35.fld5 = [6153731231546300102_usize,6_usize];
place!(Field::<f32>(Variant(RET, 2), 0)) = _11;
_18.0 = _10.0 * _10.0;
_13 = [1801072935_u32];
_15 = !_35.fld1.fld3;
place!(Field::<f64>(Variant(RET, 2), 2)) = -_24.0;
Goto(bb12)
}
bb12 = {
_35.fld4 = _35.fld1.fld7 | Field::<i128>(Variant(RET, 2), 7);
_35.fld3 = -_15;
_35.fld1.fld1 = !2670597957_u32;
_44 = _18.0 * _10.0;
_21 = _35.fld1.fld7;
_20 = _16 * _16;
_35.fld1.fld1 = !2503025416_u32;
_32 = Adt18::Variant2 { fld0: _11,fld1: '\u{10da5f}',fld2: _24.0,fld3: 210_u8,fld4: Field::<i16>(Variant(RET, 2), 4),fld5: Field::<i32>(Variant(RET, 2), 5),fld6: 5892064702257163075_u64,fld7: _35.fld4 };
_47 = _44 + _44;
_27 = &place!(Field::<i128>(Variant(RET, 2), 7));
_47 = -Field::<f64>(Variant(RET, 2), 2);
_46 = [306442378649351235730810102256430406103_u128,180876941798520959038288286853607189731_u128,28584736434153335310284232982588678385_u128,194389879139408456178853360409825275125_u128,219771898702032860215103225002362620036_u128,210849224499781775826390600854051325917_u128];
place!(Field::<i32>(Variant(RET, 2), 5)) = Field::<i32>(Variant(_32, 2), 5) >> _3;
_1 = _5;
_17 = _2;
_16 = _1 as i64;
_7 = [true,true,false,true,true,false,true,true];
Goto(bb13)
}
bb13 = {
_18.0 = Field::<f64>(Variant(_32, 2), 2) - Field::<f64>(Variant(_32, 2), 2);
Goto(bb14)
}
bb14 = {
_35.fld1.fld4 = Field::<i16>(Variant(_32, 2), 4);
_35.fld1.fld6 = Adt18::Variant2 { fld0: Field::<f32>(Variant(_32, 2), 0),fld1: '\u{c5fad}',fld2: _44,fld3: 142_u8,fld4: Field::<i16>(Variant(RET, 2), 4),fld5: Field::<i32>(Variant(RET, 2), 5),fld6: 13008218553487016850_u64,fld7: (*_27) };
place!(Field::<u64>(Variant(_32, 2), 6)) = !12870526143875150737_u64;
place!(Field::<i16>(Variant(RET, 2), 4)) = Field::<i16>(Variant(_32, 2), 4) * Field::<i16>(Variant(_35.fld1.fld6, 2), 4);
place!(Field::<f64>(Variant(_32, 2), 2)) = -_47;
place!(Field::<f32>(Variant(RET, 2), 0)) = Field::<f32>(Variant(_35.fld1.fld6, 2), 0) - Field::<f32>(Variant(_32, 2), 0);
place!(Field::<i16>(Variant(_35.fld1.fld6, 2), 4)) = _11 as i16;
_35.fld1.fld7 = -_35.fld4;
_38 = '\u{8e2e6}';
_14 = !_22;
place!(Field::<i32>(Variant(_35.fld1.fld6, 2), 5)) = Field::<i32>(Variant(_32, 2), 5);
place!(Field::<u64>(Variant(_32, 2), 6)) = !892497399761944044_u64;
Call(place!(Field::<char>(Variant(_35.fld1.fld6, 2), 1)) = fn14(Move(_27), _22), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_32 = Adt18::Variant1 { fld0: 44_u8,fld1: 163883318195871772014242910210581610629_u128 };
place!(Field::<u64>(Variant(RET, 2), 6)) = !6902480333923059310_u64;
_35.fld4 = !Field::<i128>(Variant(RET, 2), 7);
_35.fld1.fld4 = !Field::<i16>(Variant(RET, 2), 4);
_14 = _22;
_40 = Field::<u64>(Variant(RET, 2), 6);
place!(Field::<u8>(Variant(_35.fld1.fld6, 2), 3)) = 7_u8 | 83_u8;
_32 = Adt18::Variant2 { fld0: Field::<f32>(Variant(_35.fld1.fld6, 2), 0),fld1: _38,fld2: _44,fld3: Field::<u8>(Variant(_35.fld1.fld6, 2), 3),fld4: Field::<i16>(Variant(_35.fld1.fld6, 2), 4),fld5: Field::<i32>(Variant(RET, 2), 5),fld6: _40,fld7: _21 };
_35.fld2 = [false,true,true,false,true];
Goto(bb16)
}
bb16 = {
place!(Field::<u8>(Variant(RET, 2), 3)) = !Field::<u8>(Variant(_35.fld1.fld6, 2), 3);
_35.fld1.fld6 = Move(_32);
_45 = Field::<i16>(Variant(RET, 2), 4) as u64;
_26 = &_35.fld1.fld2;
Call(place!(Field::<char>(Variant(RET, 2), 1)) = fn18(_4, _35.fld2, _6, Field::<u8>(Variant(RET, 2), 3), _35.fld2, Field::<f64>(Variant(RET, 2), 2), _6, _13, _17, _18.0, _35.fld5), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
place!(Field::<u8>(Variant(_35.fld1.fld6, 2), 3)) = Field::<u8>(Variant(RET, 2), 3) >> _14;
_28.0 = _10.0 + _24.0;
_35.fld0 = core::ptr::addr_of_mut!(_35.fld1.fld1);
_31 = Field::<i128>(Variant(_35.fld1.fld6, 2), 7) as isize;
_44 = Field::<f64>(Variant(_35.fld1.fld6, 2), 2);
place!(Field::<u8>(Variant(RET, 2), 3)) = !Field::<u8>(Variant(_35.fld1.fld6, 2), 3);
_39 = core::ptr::addr_of!(place!(Field::<f64>(Variant(RET, 2), 2)));
Goto(bb18)
}
bb18 = {
Call(_53 = dump_var(12_usize, 19_usize, Move(_19), 40_usize, Move(_40), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(12_usize, 4_usize, Move(_4), 22_usize, Move(_22), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_53 = dump_var(12_usize, 5_usize, Move(_5), 34_usize, Move(_34), 38_usize, Move(_38), 54_usize, _54), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: (f64,),mut _2: u16,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: (f64,),mut _7: f64,mut _8: u16,mut _9: u16,mut _10: (f64,)) -> isize {
mir! {
type RET = isize;
let _11: (u16, f32);
let _12: isize;
let _13: i8;
let _14: f32;
let _15: ();
let _16: ();
{
RET = (-64305811485092143483708410558828847964_i128) as isize;
_5 = !_3;
Call(_8 = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = (-19090_i16) as f64;
_6 = (_7,);
_12 = _9 as isize;
_11.1 = (-141175217559534187553001737930494788054_i128) as f32;
_1 = (_6.0,);
_7 = (-1608302226_i32) as f64;
Goto(bb2)
}
bb2 = {
_13 = -(-77_i8);
_10.0 = 3689870687_u32 as f64;
_6.0 = _1.0;
RET = _12;
_2 = _8;
_13 = 70_i8;
_11.0 = !_4;
_12 = _13 as isize;
_4 = _2 * _11.0;
Goto(bb3)
}
bb3 = {
Call(_15 = dump_var(13_usize, 13_usize, Move(_13), 8_usize, Move(_8), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: &'static i128,mut _2: isize) -> char {
mir! {
type RET = char;
let _3: isize;
let _4: f32;
let _5: &'static (u16, i16, i64);
let _6: [u128; 7];
let _7: f64;
let _8: *mut u16;
let _9: [u32; 8];
let _10: Adt65;
let _11: i128;
let _12: bool;
let _13: char;
let _14: [u32; 8];
let _15: i128;
let _16: bool;
let _17: &'static &'static f32;
let _18: char;
let _19: Adt73;
let _20: (f32,);
let _21: u64;
let _22: isize;
let _23: bool;
let _24: ();
let _25: ();
{
RET = '\u{8eb12}';
RET = '\u{2ba7c}';
_2 = !(-9223372036854775808_isize);
_3 = _2;
_4 = _3 as f32;
_3 = !_2;
RET = '\u{1329c}';
RET = '\u{94c51}';
RET = '\u{8bb37}';
_2 = _3 * _3;
RET = '\u{680a5}';
_4 = 32986952843365667956877450758143997635_i128 as f32;
RET = '\u{e8324}';
_3 = 2515473629315590397_u64 as isize;
_6 = [13814268483283136665519026029339072268_u128,87934184695839586891160845929754299498_u128,335535928664903548195330785118518578444_u128,193047199617438449506490603747490427279_u128,226282682050887438311206975638163355415_u128,319937159642342610601075230429763859148_u128,327609574777807103996051496325503888865_u128];
_3 = -_2;
_6 = [15904351244820006988785970448999683118_u128,56207645797710506656356424646397603794_u128,267276550897032457580894669287404232975_u128,160915817038005222071021765900122970623_u128,21543953098440312614023555490449364276_u128,131359768318619255748633721341330705992_u128,276598467866933262021737060222931254240_u128];
RET = '\u{a8762}';
RET = '\u{aa534}';
RET = '\u{fca21}';
_4 = (-1333008573_i32) as f32;
_4 = _2 as f32;
_7 = _4 as f64;
_4 = 18797_u16 as f32;
_6 = [189680661537134460124432967506936924128_u128,158364090818437753552094588677270421057_u128,29971481594047975012392800793901287391_u128,6260699412304113362410610521611273917_u128,75757028269440884437472054126216817953_u128,8047393691691155244676736962569430894_u128,150655470125713866163955633600576899206_u128];
Goto(bb1)
}
bb1 = {
_2 = _3 + _3;
_9 = [397274814_u32,3338946153_u32,70657397_u32,942210312_u32,1846128319_u32,779210751_u32,983763910_u32,2715643669_u32];
Goto(bb2)
}
bb2 = {
_9 = [4173409187_u32,2801532276_u32,4163401478_u32,77553431_u32,1290511498_u32,1053765985_u32,400468784_u32,3717849759_u32];
_9 = [3458549977_u32,228802139_u32,217477418_u32,341894620_u32,1740561054_u32,3028072024_u32,3168549185_u32,176879149_u32];
_9 = [2864063734_u32,1822767640_u32,2836878909_u32,3458003813_u32,2616174276_u32,260116769_u32,3768316473_u32,1502002080_u32];
RET = '\u{7ac0d}';
_4 = 7253_i16 as f32;
RET = '\u{51290}';
_3 = _2 & _2;
_7 = _4 as f64;
_4 = 4958820735346284853_i64 as f32;
_7 = _4 as f64;
_9 = [2310732906_u32,1966529703_u32,3373391799_u32,3630653748_u32,2690424376_u32,1487495402_u32,2531247426_u32,2020179004_u32];
_9 = [1126289704_u32,4120649486_u32,297037607_u32,4115908903_u32,3622989724_u32,1016349043_u32,294771479_u32,2434861523_u32];
_10 = Adt65::Variant2 { fld0: 187_u8,fld1: RET };
_10 = Adt65::Variant2 { fld0: 201_u8,fld1: RET };
_10 = Adt65::Variant2 { fld0: 79_u8,fld1: RET };
_10 = Adt65::Variant2 { fld0: 28_u8,fld1: RET };
_3 = _2 - _2;
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
place!(Field::<u8>(Variant(_10, 2), 0)) = !90_u8;
_7 = 36519617212395505589835503528380201215_u128 as f64;
_9 = [1600029511_u32,3837648317_u32,664740438_u32,470722374_u32,4181160585_u32,3297543755_u32,1285398376_u32,3701289328_u32];
RET = Field::<char>(Variant(_10, 2), 1);
_2 = -_3;
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
_6 = [251741980922849866232688562731264481472_u128,253534685897314643567027284814751004417_u128,152150349819819322640619641842553223583_u128,137036444570092799423331281413927475029_u128,29098101106697102675973339625878582284_u128,305771149309866905016661698800823098875_u128,232027664908859590659511398710701443628_u128];
_4 = 3705_i16 as f32;
_3 = _2 - _2;
_4 = 17641318910135356252_u64 as f32;
place!(Field::<u8>(Variant(_10, 2), 0)) = !76_u8;
RET = Field::<char>(Variant(_10, 2), 1);
Goto(bb3)
}
bb3 = {
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
RET = Field::<char>(Variant(_10, 2), 1);
_9 = [357927940_u32,623561783_u32,360033619_u32,1198820789_u32,3296835775_u32,24497365_u32,3746877880_u32,1771236736_u32];
_1 = &_11;
_12 = false;
_11 = Field::<char>(Variant(_10, 2), 1) as i128;
_3 = !_2;
_13 = Field::<char>(Variant(_10, 2), 1);
_4 = 3326266987_u32 as f32;
_1 = &_11;
_6 = [198657291697022476290801424194405609379_u128,62608888042413673867318201079753293683_u128,68382626106114990759367032955131047301_u128,141440219392560297060161831400565421893_u128,198663335743309486177961604054947530511_u128,274374624787602827875050247035960760466_u128,285548908938772102706128681887589058805_u128];
_4 = (-38_i8) as f32;
place!(Field::<char>(Variant(_10, 2), 1)) = _13;
_14 = _9;
_11 = (-55801992391256039186481883538295280192_i128);
_13 = RET;
_12 = !true;
_6 = [249645598429887109646575309063416421133_u128,282997512022680218335424133099271737143_u128,115335355640483452867232594412288848436_u128,200819944939786298570096915180023277337_u128,167321975151805301436286169270509485078_u128,271379804969701712886509930845187280047_u128,119987942025996643305664069569540784509_u128];
Goto(bb4)
}
bb4 = {
_1 = &_11;
RET = _13;
_2 = -_3;
_12 = Field::<u8>(Variant(_10, 2), 0) > Field::<u8>(Variant(_10, 2), 0);
_9 = [4108647119_u32,693041114_u32,1150852458_u32,1937596339_u32,2592941019_u32,3347178594_u32,4020140290_u32,2549006979_u32];
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
284480374529682424276892723893472931264 => bb9,
_ => bb8
}
}
bb5 = {
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
RET = Field::<char>(Variant(_10, 2), 1);
_9 = [357927940_u32,623561783_u32,360033619_u32,1198820789_u32,3296835775_u32,24497365_u32,3746877880_u32,1771236736_u32];
_1 = &_11;
_12 = false;
_11 = Field::<char>(Variant(_10, 2), 1) as i128;
_3 = !_2;
_13 = Field::<char>(Variant(_10, 2), 1);
_4 = 3326266987_u32 as f32;
_1 = &_11;
_6 = [198657291697022476290801424194405609379_u128,62608888042413673867318201079753293683_u128,68382626106114990759367032955131047301_u128,141440219392560297060161831400565421893_u128,198663335743309486177961604054947530511_u128,274374624787602827875050247035960760466_u128,285548908938772102706128681887589058805_u128];
_4 = (-38_i8) as f32;
place!(Field::<char>(Variant(_10, 2), 1)) = _13;
_14 = _9;
_11 = (-55801992391256039186481883538295280192_i128);
_13 = RET;
_12 = !true;
_6 = [249645598429887109646575309063416421133_u128,282997512022680218335424133099271737143_u128,115335355640483452867232594412288848436_u128,200819944939786298570096915180023277337_u128,167321975151805301436286169270509485078_u128,271379804969701712886509930845187280047_u128,119987942025996643305664069569540784509_u128];
Goto(bb4)
}
bb6 = {
_9 = [4173409187_u32,2801532276_u32,4163401478_u32,77553431_u32,1290511498_u32,1053765985_u32,400468784_u32,3717849759_u32];
_9 = [3458549977_u32,228802139_u32,217477418_u32,341894620_u32,1740561054_u32,3028072024_u32,3168549185_u32,176879149_u32];
_9 = [2864063734_u32,1822767640_u32,2836878909_u32,3458003813_u32,2616174276_u32,260116769_u32,3768316473_u32,1502002080_u32];
RET = '\u{7ac0d}';
_4 = 7253_i16 as f32;
RET = '\u{51290}';
_3 = _2 & _2;
_7 = _4 as f64;
_4 = 4958820735346284853_i64 as f32;
_7 = _4 as f64;
_9 = [2310732906_u32,1966529703_u32,3373391799_u32,3630653748_u32,2690424376_u32,1487495402_u32,2531247426_u32,2020179004_u32];
_9 = [1126289704_u32,4120649486_u32,297037607_u32,4115908903_u32,3622989724_u32,1016349043_u32,294771479_u32,2434861523_u32];
_10 = Adt65::Variant2 { fld0: 187_u8,fld1: RET };
_10 = Adt65::Variant2 { fld0: 201_u8,fld1: RET };
_10 = Adt65::Variant2 { fld0: 79_u8,fld1: RET };
_10 = Adt65::Variant2 { fld0: 28_u8,fld1: RET };
_3 = _2 - _2;
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
place!(Field::<u8>(Variant(_10, 2), 0)) = !90_u8;
_7 = 36519617212395505589835503528380201215_u128 as f64;
_9 = [1600029511_u32,3837648317_u32,664740438_u32,470722374_u32,4181160585_u32,3297543755_u32,1285398376_u32,3701289328_u32];
RET = Field::<char>(Variant(_10, 2), 1);
_2 = -_3;
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
_6 = [251741980922849866232688562731264481472_u128,253534685897314643567027284814751004417_u128,152150349819819322640619641842553223583_u128,137036444570092799423331281413927475029_u128,29098101106697102675973339625878582284_u128,305771149309866905016661698800823098875_u128,232027664908859590659511398710701443628_u128];
_4 = 3705_i16 as f32;
_3 = _2 - _2;
_4 = 17641318910135356252_u64 as f32;
place!(Field::<u8>(Variant(_10, 2), 0)) = !76_u8;
RET = Field::<char>(Variant(_10, 2), 1);
Goto(bb3)
}
bb7 = {
_2 = _3 + _3;
_9 = [397274814_u32,3338946153_u32,70657397_u32,942210312_u32,1846128319_u32,779210751_u32,983763910_u32,2715643669_u32];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_9 = [2557684678_u32,3361258122_u32,1201933539_u32,526149086_u32,3297027524_u32,2375304452_u32,2340976508_u32,41792359_u32];
_2 = _3 | _3;
_4 = 327197915298838331903151143236951600754_u128 as f32;
_16 = _12 & _12;
_15 = (*_1);
_2 = 63405_u16 as isize;
Call(place!(Field::<u8>(Variant(_10, 2), 0)) = fn15(Move(_1), _6, _6, _14, _13, _3, _3, _9, _3, _14, _2, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18 = Field::<char>(Variant(_10, 2), 1);
_13 = RET;
place!(Field::<char>(Variant(_10, 2), 1)) = _13;
_2 = !_3;
place!(Field::<char>(Variant(_10, 2), 1)) = _13;
_7 = (-1_i8) as f64;
_16 = _12;
_15 = !_11;
_13 = Field::<char>(Variant(_10, 2), 1);
_20.0 = _4;
SetDiscriminant(_10, 3);
place!(Field::<(f64,)>(Variant(_10, 3), 0)) = (_7,);
_16 = !_12;
_15 = -_11;
_7 = _4 as f64;
place!(Field::<[i8; 3]>(Variant(_10, 3), 1)) = [(-120_i8),(-95_i8),(-54_i8)];
_20 = (_4,);
place!(Field::<(f64,)>(Variant(_10, 3), 0)).0 = -_7;
place!(Field::<[i8; 3]>(Variant(_10, 3), 1)) = [(-117_i8),(-128_i8),(-18_i8)];
_6 = [58223366464477881253305492847539705008_u128,114783620356313738272209060640250620856_u128,184015658416667341055463840371752339589_u128,260667029160151067718538421102024784168_u128,97723002610819121660533242312810016809_u128,22401142944893900797548790097193370739_u128,34425404042694089944732330855809624617_u128];
_1 = &_11;
_23 = _2 <= _3;
match (*_1) {
0 => bb11,
284480374529682424276892723893472931264 => bb13,
_ => bb12
}
}
bb11 = {
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
RET = Field::<char>(Variant(_10, 2), 1);
_9 = [357927940_u32,623561783_u32,360033619_u32,1198820789_u32,3296835775_u32,24497365_u32,3746877880_u32,1771236736_u32];
_1 = &_11;
_12 = false;
_11 = Field::<char>(Variant(_10, 2), 1) as i128;
_3 = !_2;
_13 = Field::<char>(Variant(_10, 2), 1);
_4 = 3326266987_u32 as f32;
_1 = &_11;
_6 = [198657291697022476290801424194405609379_u128,62608888042413673867318201079753293683_u128,68382626106114990759367032955131047301_u128,141440219392560297060161831400565421893_u128,198663335743309486177961604054947530511_u128,274374624787602827875050247035960760466_u128,285548908938772102706128681887589058805_u128];
_4 = (-38_i8) as f32;
place!(Field::<char>(Variant(_10, 2), 1)) = _13;
_14 = _9;
_11 = (-55801992391256039186481883538295280192_i128);
_13 = RET;
_12 = !true;
_6 = [249645598429887109646575309063416421133_u128,282997512022680218335424133099271737143_u128,115335355640483452867232594412288848436_u128,200819944939786298570096915180023277337_u128,167321975151805301436286169270509485078_u128,271379804969701712886509930845187280047_u128,119987942025996643305664069569540784509_u128];
Goto(bb4)
}
bb12 = {
place!(Field::<char>(Variant(_10, 2), 1)) = RET;
RET = Field::<char>(Variant(_10, 2), 1);
_9 = [357927940_u32,623561783_u32,360033619_u32,1198820789_u32,3296835775_u32,24497365_u32,3746877880_u32,1771236736_u32];
_1 = &_11;
_12 = false;
_11 = Field::<char>(Variant(_10, 2), 1) as i128;
_3 = !_2;
_13 = Field::<char>(Variant(_10, 2), 1);
_4 = 3326266987_u32 as f32;
_1 = &_11;
_6 = [198657291697022476290801424194405609379_u128,62608888042413673867318201079753293683_u128,68382626106114990759367032955131047301_u128,141440219392560297060161831400565421893_u128,198663335743309486177961604054947530511_u128,274374624787602827875050247035960760466_u128,285548908938772102706128681887589058805_u128];
_4 = (-38_i8) as f32;
place!(Field::<char>(Variant(_10, 2), 1)) = _13;
_14 = _9;
_11 = (-55801992391256039186481883538295280192_i128);
_13 = RET;
_12 = !true;
_6 = [249645598429887109646575309063416421133_u128,282997512022680218335424133099271737143_u128,115335355640483452867232594412288848436_u128,200819944939786298570096915180023277337_u128,167321975151805301436286169270509485078_u128,271379804969701712886509930845187280047_u128,119987942025996643305664069569540784509_u128];
Goto(bb4)
}
bb13 = {
_19 = Adt73::Variant3 { fld0: RET };
_11 = (-8207061636245925822_i64) as i128;
_6 = [211343878636497773004376655112292142912_u128,65645247376951787838480339226928715935_u128,309457917276413648690859032205396047597_u128,188663848799924368197260044401968479241_u128,135486031694077849003098105982400939467_u128,153353210145648225297210758291592658538_u128,110774652239426815886043025858830184867_u128];
_7 = Field::<(f64,)>(Variant(_10, 3), 0).0 * Field::<(f64,)>(Variant(_10, 3), 0).0;
_21 = 2668967166376553025_i64 as u64;
_16 = !_23;
Goto(bb14)
}
bb14 = {
_9 = [1936326595_u32,1777091886_u32,294639556_u32,3355119366_u32,3137562502_u32,3372423130_u32,2423268848_u32,1451131025_u32];
_20.0 = _3 as f32;
place!(Field::<(f64,)>(Variant(_10, 3), 0)).0 = _7;
_7 = Field::<(f64,)>(Variant(_10, 3), 0).0 - Field::<(f64,)>(Variant(_10, 3), 0).0;
_3 = 14040568475336857470_usize as isize;
place!(Field::<(f64,)>(Variant(_10, 3), 0)).0 = -_7;
_7 = -Field::<(f64,)>(Variant(_10, 3), 0).0;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(14_usize, 14_usize, Move(_14), 15_usize, Move(_15), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(14_usize, 13_usize, Move(_13), 16_usize, Move(_16), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static i128,mut _2: [u128; 7],mut _3: [u128; 7],mut _4: [u32; 8],mut _5: char,mut _6: isize,mut _7: isize,mut _8: [u32; 8],mut _9: isize,mut _10: [u32; 8],mut _11: isize,mut _12: isize) -> u8 {
mir! {
type RET = u8;
let _13: char;
let _14: [bool; 5];
let _15: Adt71;
let _16: [usize; 8];
let _17: (u16, f32);
let _18: i8;
let _19: isize;
let _20: [i16; 8];
let _21: bool;
let _22: (&'static u128, &'static i16, *mut u16);
let _23: [usize; 8];
let _24: [u32; 1];
let _25: &'static f32;
let _26: *mut u8;
let _27: f32;
let _28: [usize; 8];
let _29: [u32; 1];
let _30: u32;
let _31: isize;
let _32: char;
let _33: char;
let _34: *const [u128; 7];
let _35: f64;
let _36: char;
let _37: u32;
let _38: [usize; 8];
let _39: f64;
let _40: *mut ([bool; 8], (f32,), &'static (u16, i16, i64));
let _41: Adt65;
let _42: *const [u128; 7];
let _43: [char; 7];
let _44: ();
let _45: ();
{
_9 = _12;
Goto(bb1)
}
bb1 = {
RET = 35_u8 * 106_u8;
_6 = -_7;
_7 = _12;
RET = 229_u8;
_7 = _12 - _9;
_12 = _6 ^ _11;
_12 = _9 << _9;
RET = 170_u8 * 135_u8;
_7 = !_9;
_5 = '\u{8aec1}';
_10 = _4;
Goto(bb2)
}
bb2 = {
_12 = -_7;
_13 = _5;
_2 = [288219872391953228352584844983129629751_u128,297695330560253866001494276896486737785_u128,61636424017803792037574470520525997207_u128,316408890510333297883924518153295644078_u128,333477445608835969694058536136511213597_u128,303892225041591290248758773093861506147_u128,231998388001484454582293631409351490088_u128];
_9 = _7;
_2 = [198988537041117361982392752709547752442_u128,266110950436896027460062659624886983614_u128,92824317674963213527507588443038697349_u128,75573696114786824078651681887900304672_u128,84446097518817919660121253547842342800_u128,234885137415078203231419227876235244895_u128,159514974120214922616676832515249844594_u128];
_5 = _13;
_3 = [16958531196704039681026886947998077143_u128,74459038573281161842600474428921121616_u128,215277001835998776501067691969441103443_u128,210200733425880119449666595357808521740_u128,145483048092106335046639573797297259812_u128,80826756473399018682675706580780885154_u128,294121322466237890398759636212451068972_u128];
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
_15.fld5.fld4 = RET as i16;
_14 = [false,true,true,true,false];
_15.fld5.fld4 = 2577_u16 as i16;
_15.fld5.fld5 = 19958_u16;
_15.fld5.fld1 = RET as u32;
_8 = _10;
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
_15.fld5.fld6 = Adt18::Variant1 { fld0: RET,fld1: 225486098322799758380640840103003256650_u128 };
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
place!(Field::<u128>(Variant(_15.fld5.fld6, 1), 1)) = 31953426140601396640530307741131375982_u128 >> _6;
_10 = _4;
_15.fld5.fld5 = 61367_u16 * 16757_u16;
SetDiscriminant(_15.fld5.fld6, 2);
_4 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
_15.fld3.1 = 5639625365911501247_i64 as f32;
Call(_15.fld5.fld7 = core::intrinsics::bswap((-97740945202099975452899498011412658109_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = [8376954810776281495_usize,16730931217342136688_usize,5_usize,2672550246695331985_usize,1_usize,14172832619357210722_usize,13408331628283084746_usize,10619784321738690746_usize];
_2 = _3;
place!(Field::<f32>(Variant(_15.fld5.fld6, 2), 0)) = -_15.fld3.1;
_15.fld3 = (_15.fld5.fld5, Field::<f32>(Variant(_15.fld5.fld6, 2), 0));
place!(Field::<char>(Variant(_15.fld5.fld6, 2), 1)) = _5;
RET = 125_u8;
_7 = _9;
_4 = _10;
_16 = [3737718553609521893_usize,819885397773362268_usize,3_usize,3_usize,10066162173725308125_usize,5_usize,7277518171372045786_usize,3_usize];
Goto(bb4)
}
bb4 = {
place!(Field::<f64>(Variant(_15.fld5.fld6, 2), 2)) = (-23_i8) as f64;
match RET {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
125 => bb10,
_ => bb9
}
}
bb5 = {
_16 = [8376954810776281495_usize,16730931217342136688_usize,5_usize,2672550246695331985_usize,1_usize,14172832619357210722_usize,13408331628283084746_usize,10619784321738690746_usize];
_2 = _3;
place!(Field::<f32>(Variant(_15.fld5.fld6, 2), 0)) = -_15.fld3.1;
_15.fld3 = (_15.fld5.fld5, Field::<f32>(Variant(_15.fld5.fld6, 2), 0));
place!(Field::<char>(Variant(_15.fld5.fld6, 2), 1)) = _5;
RET = 125_u8;
_7 = _9;
_4 = _10;
_16 = [3737718553609521893_usize,819885397773362268_usize,3_usize,3_usize,10066162173725308125_usize,5_usize,7277518171372045786_usize,3_usize];
Goto(bb4)
}
bb6 = {
_12 = -_7;
_13 = _5;
_2 = [288219872391953228352584844983129629751_u128,297695330560253866001494276896486737785_u128,61636424017803792037574470520525997207_u128,316408890510333297883924518153295644078_u128,333477445608835969694058536136511213597_u128,303892225041591290248758773093861506147_u128,231998388001484454582293631409351490088_u128];
_9 = _7;
_2 = [198988537041117361982392752709547752442_u128,266110950436896027460062659624886983614_u128,92824317674963213527507588443038697349_u128,75573696114786824078651681887900304672_u128,84446097518817919660121253547842342800_u128,234885137415078203231419227876235244895_u128,159514974120214922616676832515249844594_u128];
_5 = _13;
_3 = [16958531196704039681026886947998077143_u128,74459038573281161842600474428921121616_u128,215277001835998776501067691969441103443_u128,210200733425880119449666595357808521740_u128,145483048092106335046639573797297259812_u128,80826756473399018682675706580780885154_u128,294121322466237890398759636212451068972_u128];
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
_15.fld5.fld4 = RET as i16;
_14 = [false,true,true,true,false];
_15.fld5.fld4 = 2577_u16 as i16;
_15.fld5.fld5 = 19958_u16;
_15.fld5.fld1 = RET as u32;
_8 = _10;
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
_15.fld5.fld6 = Adt18::Variant1 { fld0: RET,fld1: 225486098322799758380640840103003256650_u128 };
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
place!(Field::<u128>(Variant(_15.fld5.fld6, 1), 1)) = 31953426140601396640530307741131375982_u128 >> _6;
_10 = _4;
_15.fld5.fld5 = 61367_u16 * 16757_u16;
SetDiscriminant(_15.fld5.fld6, 2);
_4 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
_15.fld3.1 = 5639625365911501247_i64 as f32;
Call(_15.fld5.fld7 = core::intrinsics::bswap((-97740945202099975452899498011412658109_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = 35_u8 * 106_u8;
_6 = -_7;
_7 = _12;
RET = 229_u8;
_7 = _12 - _9;
_12 = _6 ^ _11;
_12 = _9 << _9;
RET = 170_u8 * 135_u8;
_7 = !_9;
_5 = '\u{8aec1}';
_10 = _4;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_15.fld0 = false;
_15.fld7 = [3_usize,3_usize];
place!(Field::<char>(Variant(_15.fld5.fld6, 2), 1)) = _5;
place!(Field::<u8>(Variant(_15.fld5.fld6, 2), 3)) = 7_usize as u8;
_15.fld5.fld7 = _15.fld5.fld1 as i128;
_17.0 = !_15.fld3.0;
_14 = [_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0];
place!(Field::<i16>(Variant(_15.fld5.fld6, 2), 4)) = -_15.fld5.fld4;
place!(Field::<char>(Variant(_15.fld5.fld6, 2), 1)) = _5;
_18 = _15.fld5.fld4 as i8;
_1 = &_15.fld5.fld7;
place!(Field::<i128>(Variant(_15.fld5.fld6, 2), 7)) = (*_1) | (*_1);
_12 = _15.fld3.1 as isize;
_15.fld2 = _3;
_6 = _12;
_13 = Field::<char>(Variant(_15.fld5.fld6, 2), 1);
_23 = _16;
_15.fld5.fld3 = !_18;
_15.fld5.fld2 = 167795400325125136527916240105812207846_u128 - 163071827695812305499907373644064320764_u128;
_15.fld2 = [_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2];
_19 = 4764365450981314432_u64 as isize;
Call(_2 = fn16(Move(_1), _10, _3, _3, _4, _16), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.fld5.fld6 = Adt18::Variant1 { fld0: RET,fld1: _15.fld5.fld2 };
_22.0 = &_15.fld5.fld2;
_15.fld0 = Field::<u8>(Variant(_15.fld5.fld6, 1), 0) != Field::<u8>(Variant(_15.fld5.fld6, 1), 0);
_20 = [_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4];
_9 = _15.fld0 as isize;
_3 = [Field::<u128>(Variant(_15.fld5.fld6, 1), 1),_15.fld5.fld2,_15.fld5.fld2,Field::<u128>(Variant(_15.fld5.fld6, 1), 1),Field::<u128>(Variant(_15.fld5.fld6, 1), 1),_15.fld5.fld2,_15.fld5.fld2];
_14 = [_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0];
place!(Field::<u128>(Variant(_15.fld5.fld6, 1), 1)) = !_15.fld5.fld2;
_18 = !_15.fld5.fld3;
_23 = [3_usize,8669422131316811950_usize,10815537626829856442_usize,7_usize,2_usize,8995434889609657322_usize,5_usize,1_usize];
_20 = [_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4];
_15.fld6 = 6577285983579202735_i64 & (-5759239762142478891_i64);
_15.fld5.fld6 = Adt18::Variant1 { fld0: RET,fld1: _15.fld5.fld2 };
_15.fld3.0 = _15.fld5.fld5;
_15.fld2 = [Field::<u128>(Variant(_15.fld5.fld6, 1), 1),Field::<u128>(Variant(_15.fld5.fld6, 1), 1),Field::<u128>(Variant(_15.fld5.fld6, 1), 1),Field::<u128>(Variant(_15.fld5.fld6, 1), 1),_15.fld5.fld2,Field::<u128>(Variant(_15.fld5.fld6, 1), 1),_15.fld5.fld2];
_18 = -_15.fld5.fld3;
_22.1 = &_15.fld5.fld4;
_25 = &_27;
_15.fld3.0 = _17.0;
_27 = -_15.fld3.1;
_23 = [3_usize,8324055236445512986_usize,10639533687656901827_usize,3_usize,2_usize,6_usize,17363051464796975348_usize,3191892962170240696_usize];
_7 = -_9;
_1 = &_15.fld5.fld7;
Goto(bb12)
}
bb12 = {
_16 = _23;
_15.fld5.fld2 = Field::<u128>(Variant(_15.fld5.fld6, 1), 1);
place!(Field::<u128>(Variant(_15.fld5.fld6, 1), 1)) = _7 as u128;
SetDiscriminant(_15.fld5.fld6, 0);
_20 = [_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4];
_19 = _18 as isize;
_21 = _18 <= _15.fld5.fld3;
_15.fld5.fld7 = RET as i128;
_15.fld6 = 5196989965420653316_i64 - (-8962055840529067854_i64);
_15.fld5.fld1 = _15.fld5.fld7 as u32;
_22.0 = &_15.fld5.fld2;
_4 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
_6 = _9;
_4 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
place!(Field::<u64>(Variant(_15.fld5.fld6, 0), 3)) = 11873864157076035097_u64;
_23 = [0_usize,11256443715043781714_usize,2_usize,10241721129999763556_usize,2_usize,7_usize,6498729612095251344_usize,6795359778640241861_usize];
_15.fld5.fld0 = Field::<u64>(Variant(_15.fld5.fld6, 0), 3) - Field::<u64>(Variant(_15.fld5.fld6, 0), 3);
_32 = _13;
_17.0 = _15.fld5.fld5;
place!(Field::<char>(Variant(_15.fld5.fld6, 0), 1)) = _5;
_15.fld3.1 = -_27;
Goto(bb13)
}
bb13 = {
_4 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
_11 = _15.fld5.fld7 as isize;
_22.2 = core::ptr::addr_of_mut!(_17.0);
_2 = _3;
place!(Field::<f64>(Variant(_15.fld5.fld6, 0), 2)) = _19 as f64;
_29 = [_15.fld5.fld1];
_24 = _29;
_23 = [8096685483435086734_usize,0_usize,5_usize,12211460217026015804_usize,2578603419988303250_usize,12296022756261499029_usize,4152868233916114206_usize,17405998136714507517_usize];
_21 = _15.fld0;
match Field::<u64>(Variant(_15.fld5.fld6, 0), 3) {
11873864157076035097 => bb15,
_ => bb14
}
}
bb14 = {
_12 = -_7;
_13 = _5;
_2 = [288219872391953228352584844983129629751_u128,297695330560253866001494276896486737785_u128,61636424017803792037574470520525997207_u128,316408890510333297883924518153295644078_u128,333477445608835969694058536136511213597_u128,303892225041591290248758773093861506147_u128,231998388001484454582293631409351490088_u128];
_9 = _7;
_2 = [198988537041117361982392752709547752442_u128,266110950436896027460062659624886983614_u128,92824317674963213527507588443038697349_u128,75573696114786824078651681887900304672_u128,84446097518817919660121253547842342800_u128,234885137415078203231419227876235244895_u128,159514974120214922616676832515249844594_u128];
_5 = _13;
_3 = [16958531196704039681026886947998077143_u128,74459038573281161842600474428921121616_u128,215277001835998776501067691969441103443_u128,210200733425880119449666595357808521740_u128,145483048092106335046639573797297259812_u128,80826756473399018682675706580780885154_u128,294121322466237890398759636212451068972_u128];
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
_15.fld5.fld4 = RET as i16;
_14 = [false,true,true,true,false];
_15.fld5.fld4 = 2577_u16 as i16;
_15.fld5.fld5 = 19958_u16;
_15.fld5.fld1 = RET as u32;
_8 = _10;
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
_15.fld5.fld6 = Adt18::Variant1 { fld0: RET,fld1: 225486098322799758380640840103003256650_u128 };
_15.fld4 = core::ptr::addr_of_mut!(_15.fld5.fld5);
place!(Field::<u128>(Variant(_15.fld5.fld6, 1), 1)) = 31953426140601396640530307741131375982_u128 >> _6;
_10 = _4;
_15.fld5.fld5 = 61367_u16 * 16757_u16;
SetDiscriminant(_15.fld5.fld6, 2);
_4 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
_15.fld3.1 = 5639625365911501247_i64 as f32;
Call(_15.fld5.fld7 = core::intrinsics::bswap((-97740945202099975452899498011412658109_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_9 = (-1053405114_i32) as isize;
_15.fld3.1 = _27 + _27;
_15.fld3.0 = 7_usize as u16;
_31 = _9 * _12;
_17 = _15.fld3;
_9 = -_7;
_39 = Field::<f64>(Variant(_15.fld5.fld6, 0), 2) * Field::<f64>(Variant(_15.fld5.fld6, 0), 2);
place!(Field::<u64>(Variant(_15.fld5.fld6, 0), 3)) = _15.fld5.fld0 | _15.fld5.fld0;
_11 = _9 >> _15.fld5.fld3;
_15.fld5.fld7 = (-100996071788670123636583669371754053276_i128);
_2 = [_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2,_15.fld5.fld2];
_34 = core::ptr::addr_of!(_2);
_5 = Field::<char>(Variant(_15.fld5.fld6, 0), 1);
_33 = _13;
_10 = [_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1,_15.fld5.fld1];
_26 = core::ptr::addr_of_mut!(RET);
(*_26) = 46_u8 - 148_u8;
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(15_usize, 6_usize, Move(_6), 4_usize, Move(_4), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(15_usize, 3_usize, Move(_3), 29_usize, Move(_29), 12_usize, Move(_12), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(15_usize, 21_usize, Move(_21), 20_usize, Move(_20), 13_usize, Move(_13), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: &'static i128,mut _2: [u32; 8],mut _3: [u128; 7],mut _4: [u128; 7],mut _5: [u32; 8],mut _6: [usize; 8]) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _7: u64;
let _8: isize;
let _9: &'static i8;
let _10: u16;
let _11: u128;
let _12: [bool; 8];
let _13: *mut u8;
let _14: (&'static i32, [usize; 8], &'static [usize; 2], &'static (u16, f32));
let _15: [u32; 1];
let _16: char;
let _17: *mut ([bool; 8], (f32,), &'static (u16, i16, i64));
let _18: *mut u16;
let _19: usize;
let _20: u128;
let _21: (u16, i16, i64);
let _22: u16;
let _23: (f32,);
let _24: ();
let _25: ();
{
_7 = !1627503502868054728_u64;
RET = [283047865853366515259124022602257731749_u128,28765874694454616157854192593767618257_u128,117124492744826143145819588302891409835_u128,320111612458726578153686170096724131816_u128,42178774765254214994757389556888702817_u128,179218770599503536353962486268988646473_u128,230221916218004290003284162048232365931_u128];
_4 = [332807209516177034419013298254410745824_u128,207239350513799105030601444686179866753_u128,282552220531838010570767467336234785953_u128,254366554269514304202438154094432600851_u128,126416254635897754107764924146339029327_u128,156654350209466677623578856912886801502_u128,3206912063060688134444768200796727399_u128];
_6 = [12103841034445755931_usize,5950178176238051117_usize,2_usize,7868007095847017701_usize,2294550665667713007_usize,5_usize,4_usize,2_usize];
_7 = !5957025786854174726_u64;
_7 = !2544370678958782167_u64;
_8 = !(-104_isize);
_7 = 1247651743656406083_u64;
_6 = [12238444533980593429_usize,7559850944434739218_usize,3_usize,10869973064460969758_usize,16898264975449456918_usize,10534332064896631030_usize,2978814043787059589_usize,18119920097152754166_usize];
_2 = [2938875784_u32,834131300_u32,112975256_u32,3592531933_u32,2929418195_u32,1923869576_u32,3459627827_u32,1351594542_u32];
_6 = [4_usize,3_usize,13907578006181643602_usize,17421925985858182792_usize,5_usize,5_usize,3600042456159071675_usize,2284154967239823155_usize];
_3 = [220568154911387971723799819710508984383_u128,333322624075245842687601715854390547097_u128,331469695030625508707735804339586607699_u128,292679542266520831647897073618363013625_u128,75576594145435400451618060973018821905_u128,241129810895733638842442501925617623596_u128,28213819138363114167684657729690191638_u128];
_4 = [268583118983773836293197356421757445966_u128,149016122530441190866057928787642325801_u128,253482977410230671182280393671191842640_u128,57974738346930727779035158520251656723_u128,304743036367441479134528963143432813002_u128,40528101012141269379649326232478076538_u128,222680208340195616831726130557096510862_u128];
_7 = 6_usize as u64;
_2 = _5;
RET = [114054123453376818390229448018859391424_u128,130247298072095429658399576276850798436_u128,165553773565011218862565310578291553238_u128,178565565194934367151040801613454514442_u128,135120812241334995746253022584591558658_u128,23002446390540555149065054193053119201_u128,338944562529997505738698977206955574350_u128];
_4 = _3;
_8 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_2 = _5;
_2 = [1658515017_u32,950906295_u32,1962578753_u32,1378402699_u32,154116133_u32,1054136102_u32,1755174324_u32,1494232407_u32];
_6 = [4_usize,2_usize,4_usize,5_usize,6_usize,3_usize,3_usize,3_usize];
_3 = [95777487789912387047707105691711789764_u128,202187519023790279648202673090345062812_u128,105196677957369692903164320540235241571_u128,291861303932269018145526831980879044651_u128,99905018121572393012256265521192062855_u128,93497634286335187509554971278085531074_u128,54316826679068536061040256950521266632_u128];
Goto(bb1)
}
bb1 = {
_4 = [126889930608662440841943818391231302478_u128,297940705669656674238408312140604615351_u128,103768379282135699619097072669448300121_u128,313597485399436672257752189943478818839_u128,77760863873142325997717697616035025394_u128,237095525856684127798254444778968517592_u128,48588241956071747410755171391253512490_u128];
_3 = [123890449929238485377997527140144002558_u128,58937793285405723317286680108073003182_u128,172092556262667201244917229193979539226_u128,38078889596605910663087183980407090710_u128,54311289509766289308679242034906047233_u128,134342873002807689464869163903492792378_u128,329758053424884065610037776525882735456_u128];
_8 = (-9223372036854775808_isize) - (-5_isize);
_4 = RET;
_2 = _5;
_10 = _8 as u16;
_6 = [1379397556374483197_usize,13279224653068044634_usize,14302902081852557912_usize,10793253006662919529_usize,5_usize,1_usize,0_usize,0_usize];
_8 = (-9223372036854775808_isize);
_5 = [3070033552_u32,2242606999_u32,3860419202_u32,1327511559_u32,1299919647_u32,1598318313_u32,3777332253_u32,2217913792_u32];
_8 = (-58_isize) - 9223372036854775807_isize;
_3 = [6645132466087078191449641289750084475_u128,116237604861666412388682973311860100919_u128,113324533873274356909112215352856937403_u128,48751159632796907538447492528342202678_u128,80298141578434956989518847485784479293_u128,130953160228613652496194976498424646445_u128,329797719043479379194492273751177324442_u128];
RET = [90198125655910077031676014969618033358_u128,255384206081709305017616251321196592645_u128,118615590102778161522418666707076665648_u128,206308105474567148545032007484266695156_u128,258526192746184074767368508120975580330_u128,206716247629811746752020055512896334061_u128,51296573435452388439708365106506475766_u128];
_3 = [141172557525069070457716561823472709519_u128,265975252766799944321443388833183164412_u128,278313648500671942738988591994504165581_u128,114050885403326144288903676912090912764_u128,235741477860816791772439503764994453156_u128,45182169070836389827627625529326419532_u128,275677679767142339499555770754911739264_u128];
_4 = _3;
_10 = 18828_u16;
_8 = 38_u8 as isize;
_3 = _4;
_7 = 90_i8 as u64;
_4 = [1352885224869815258824107142985251540_u128,16401392561226113896049591063676754269_u128,183582432155060986196789352153014135935_u128,115410593755260755955681682119407658047_u128,278287034997359219737639772572857413994_u128,156410104712643900206815024326402021998_u128,46022345308594929025222890895629590712_u128];
RET = [234170090999244120315736484839465724533_u128,314343296047201773835703996956880980847_u128,58582938328800562469714527077790218226_u128,137527498215823018465808680360077382890_u128,306202577530455208915072503635722929506_u128,15064255464438806651259775839549499659_u128,9007157514422700631919108974807641770_u128];
_7 = !12368895700444822424_u64;
_6 = [12466953381801447681_usize,0_usize,1_usize,7492551239784272633_usize,3273166421208689779_usize,15788342183544487955_usize,14038552114080704479_usize,2_usize];
_6 = [12944787467375531265_usize,1045227276307959213_usize,0_usize,5_usize,17946357323064482745_usize,7_usize,9034463779627894040_usize,3_usize];
RET = [220258754846072405577269694193046828017_u128,78357615813459155119899137676287712038_u128,86941281043144333179405870809121380348_u128,182718648063483629903242328766771304485_u128,317699864769611778646699886463755771339_u128,154033609796331433455643926092861640802_u128,89787905037955502122471848021569742859_u128];
_6 = [18309748621610281063_usize,9847036753379369645_usize,6941066187141526616_usize,3793791536509806340_usize,0_usize,3_usize,2731040544834947223_usize,15268705268367438697_usize];
_12 = [true,false,true,true,false,true,false,false];
_6 = [827345996275896686_usize,9810336693302373342_usize,5_usize,13039692206788679577_usize,5_usize,7_usize,15669729367463505709_usize,13568185199478750627_usize];
Goto(bb2)
}
bb2 = {
_10 = (-25665057621598299322787828795001385508_i128) as u16;
_8 = -(-31_isize);
_4 = [155461777040324391083663249126314104538_u128,323341667339258404826566140644353625126_u128,198491304330253324195242529161000309993_u128,288266157199276297836743107856870500804_u128,212711976154274404189493243940587032512_u128,249481582280452191951472617942747029654_u128,219074107312988441678573067908676496242_u128];
Goto(bb3)
}
bb3 = {
_5 = [2733988979_u32,3333404855_u32,3267513866_u32,3584629132_u32,3473952375_u32,2843691545_u32,4195627598_u32,2568040390_u32];
_12 = [true,false,true,true,false,true,false,true];
_6 = [3_usize,2_usize,6_usize,6_usize,9362291409319586323_usize,9470424390557804554_usize,7_usize,5600401468910629523_usize];
_7 = _8 as u64;
_5 = _2;
_11 = 147080189892491507100507785797891599441_u128 >> _7;
RET = [_11,_11,_11,_11,_11,_11,_11];
RET = [_11,_11,_11,_11,_11,_11,_11];
_7 = 18101456382208292231_u64 * 11883125899172399112_u64;
_12 = [true,false,true,true,true,false,false,false];
_4 = [_11,_11,_11,_11,_11,_11,_11];
_2 = [2892439525_u32,2792069254_u32,1598405275_u32,3514651425_u32,2104831317_u32,4103777723_u32,3679417933_u32,4226958829_u32];
_2 = [3209410888_u32,3468383189_u32,1599824561_u32,733218473_u32,456844969_u32,3573466795_u32,2913659406_u32,1921899622_u32];
_10 = !30874_u16;
_6 = [15872991330698714743_usize,8648323525648979240_usize,3_usize,10532875998387713932_usize,6631857343043700855_usize,5_usize,0_usize,0_usize];
RET = _3;
_10 = (-5496_i16) as u16;
_3 = RET;
RET = _3;
_3 = [_11,_11,_11,_11,_11,_11,_11];
_2 = [917798613_u32,1861131405_u32,2715871642_u32,1614897811_u32,4210135278_u32,1760449496_u32,3954791082_u32,3946161075_u32];
_14.1 = _6;
_7 = !9372364477713040063_u64;
Call(_13 = fn17(_12, _11, _2, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = '\u{dcfab}';
RET = _4;
_8 = -9223372036854775807_isize;
_15 = [2455721406_u32];
RET = [_11,_11,_11,_11,_11,_11,_11];
_8 = 18_isize;
RET = _4;
_7 = 1270274062161035592_u64;
RET = [_11,_11,_11,_11,_11,_11,_11];
_11 = !250731548963507774130919600906730504451_u128;
_4 = RET;
RET = [_11,_11,_11,_11,_11,_11,_11];
_10 = !8979_u16;
_14.1 = [16615095314388310958_usize,2_usize,5_usize,8934609155321760463_usize,17612587237107806742_usize,2168469619469634993_usize,4_usize,5_usize];
RET = _3;
_14.1 = [3_usize,16200011302798639149_usize,2903536641745322643_usize,5_usize,7127568584674477374_usize,1_usize,9506496930414081128_usize,8945581956059163037_usize];
_14.1 = [719159060079051932_usize,0_usize,0_usize,7_usize,17406466446496379012_usize,7531775815165403087_usize,6_usize,5247781447473086366_usize];
_7 = 15065164316756111545_u64;
RET = _3;
_6 = _14.1;
_2 = _5;
_10 = !13790_u16;
_8 = (-10025_i16) as isize;
match _7 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
15065164316756111545 => bb10,
_ => bb9
}
}
bb5 = {
_5 = [2733988979_u32,3333404855_u32,3267513866_u32,3584629132_u32,3473952375_u32,2843691545_u32,4195627598_u32,2568040390_u32];
_12 = [true,false,true,true,false,true,false,true];
_6 = [3_usize,2_usize,6_usize,6_usize,9362291409319586323_usize,9470424390557804554_usize,7_usize,5600401468910629523_usize];
_7 = _8 as u64;
_5 = _2;
_11 = 147080189892491507100507785797891599441_u128 >> _7;
RET = [_11,_11,_11,_11,_11,_11,_11];
RET = [_11,_11,_11,_11,_11,_11,_11];
_7 = 18101456382208292231_u64 * 11883125899172399112_u64;
_12 = [true,false,true,true,true,false,false,false];
_4 = [_11,_11,_11,_11,_11,_11,_11];
_2 = [2892439525_u32,2792069254_u32,1598405275_u32,3514651425_u32,2104831317_u32,4103777723_u32,3679417933_u32,4226958829_u32];
_2 = [3209410888_u32,3468383189_u32,1599824561_u32,733218473_u32,456844969_u32,3573466795_u32,2913659406_u32,1921899622_u32];
_10 = !30874_u16;
_6 = [15872991330698714743_usize,8648323525648979240_usize,3_usize,10532875998387713932_usize,6631857343043700855_usize,5_usize,0_usize,0_usize];
RET = _3;
_10 = (-5496_i16) as u16;
_3 = RET;
RET = _3;
_3 = [_11,_11,_11,_11,_11,_11,_11];
_2 = [917798613_u32,1861131405_u32,2715871642_u32,1614897811_u32,4210135278_u32,1760449496_u32,3954791082_u32,3946161075_u32];
_14.1 = _6;
_7 = !9372364477713040063_u64;
Call(_13 = fn17(_12, _11, _2, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_10 = (-25665057621598299322787828795001385508_i128) as u16;
_8 = -(-31_isize);
_4 = [155461777040324391083663249126314104538_u128,323341667339258404826566140644353625126_u128,198491304330253324195242529161000309993_u128,288266157199276297836743107856870500804_u128,212711976154274404189493243940587032512_u128,249481582280452191951472617942747029654_u128,219074107312988441678573067908676496242_u128];
Goto(bb3)
}
bb7 = {
_4 = [126889930608662440841943818391231302478_u128,297940705669656674238408312140604615351_u128,103768379282135699619097072669448300121_u128,313597485399436672257752189943478818839_u128,77760863873142325997717697616035025394_u128,237095525856684127798254444778968517592_u128,48588241956071747410755171391253512490_u128];
_3 = [123890449929238485377997527140144002558_u128,58937793285405723317286680108073003182_u128,172092556262667201244917229193979539226_u128,38078889596605910663087183980407090710_u128,54311289509766289308679242034906047233_u128,134342873002807689464869163903492792378_u128,329758053424884065610037776525882735456_u128];
_8 = (-9223372036854775808_isize) - (-5_isize);
_4 = RET;
_2 = _5;
_10 = _8 as u16;
_6 = [1379397556374483197_usize,13279224653068044634_usize,14302902081852557912_usize,10793253006662919529_usize,5_usize,1_usize,0_usize,0_usize];
_8 = (-9223372036854775808_isize);
_5 = [3070033552_u32,2242606999_u32,3860419202_u32,1327511559_u32,1299919647_u32,1598318313_u32,3777332253_u32,2217913792_u32];
_8 = (-58_isize) - 9223372036854775807_isize;
_3 = [6645132466087078191449641289750084475_u128,116237604861666412388682973311860100919_u128,113324533873274356909112215352856937403_u128,48751159632796907538447492528342202678_u128,80298141578434956989518847485784479293_u128,130953160228613652496194976498424646445_u128,329797719043479379194492273751177324442_u128];
RET = [90198125655910077031676014969618033358_u128,255384206081709305017616251321196592645_u128,118615590102778161522418666707076665648_u128,206308105474567148545032007484266695156_u128,258526192746184074767368508120975580330_u128,206716247629811746752020055512896334061_u128,51296573435452388439708365106506475766_u128];
_3 = [141172557525069070457716561823472709519_u128,265975252766799944321443388833183164412_u128,278313648500671942738988591994504165581_u128,114050885403326144288903676912090912764_u128,235741477860816791772439503764994453156_u128,45182169070836389827627625529326419532_u128,275677679767142339499555770754911739264_u128];
_4 = _3;
_10 = 18828_u16;
_8 = 38_u8 as isize;
_3 = _4;
_7 = 90_i8 as u64;
_4 = [1352885224869815258824107142985251540_u128,16401392561226113896049591063676754269_u128,183582432155060986196789352153014135935_u128,115410593755260755955681682119407658047_u128,278287034997359219737639772572857413994_u128,156410104712643900206815024326402021998_u128,46022345308594929025222890895629590712_u128];
RET = [234170090999244120315736484839465724533_u128,314343296047201773835703996956880980847_u128,58582938328800562469714527077790218226_u128,137527498215823018465808680360077382890_u128,306202577530455208915072503635722929506_u128,15064255464438806651259775839549499659_u128,9007157514422700631919108974807641770_u128];
_7 = !12368895700444822424_u64;
_6 = [12466953381801447681_usize,0_usize,1_usize,7492551239784272633_usize,3273166421208689779_usize,15788342183544487955_usize,14038552114080704479_usize,2_usize];
_6 = [12944787467375531265_usize,1045227276307959213_usize,0_usize,5_usize,17946357323064482745_usize,7_usize,9034463779627894040_usize,3_usize];
RET = [220258754846072405577269694193046828017_u128,78357615813459155119899137676287712038_u128,86941281043144333179405870809121380348_u128,182718648063483629903242328766771304485_u128,317699864769611778646699886463755771339_u128,154033609796331433455643926092861640802_u128,89787905037955502122471848021569742859_u128];
_6 = [18309748621610281063_usize,9847036753379369645_usize,6941066187141526616_usize,3793791536509806340_usize,0_usize,3_usize,2731040544834947223_usize,15268705268367438697_usize];
_12 = [true,false,true,true,false,true,false,false];
_6 = [827345996275896686_usize,9810336693302373342_usize,5_usize,13039692206788679577_usize,5_usize,7_usize,15669729367463505709_usize,13568185199478750627_usize];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_16 = '\u{4772a}';
_10 = !18553_u16;
_2 = [3731556388_u32,46884943_u32,3453780614_u32,3205531074_u32,2055633329_u32,3076799009_u32,3378463675_u32,1234202402_u32];
_14.1 = _6;
_3 = [_11,_11,_11,_11,_11,_11,_11];
_6 = _14.1;
_18 = core::ptr::addr_of_mut!(_10);
(*_18) = !33132_u16;
_5 = [522329939_u32,2650139143_u32,1451009672_u32,569897418_u32,2366995402_u32,1558013767_u32,324585034_u32,2273134059_u32];
_15 = [1348451660_u32];
RET = _4;
_21.0 = !(*_18);
_2 = _5;
match _7 {
0 => bb1,
1 => bb2,
2 => bb4,
15065164316756111545 => bb12,
_ => bb11
}
}
bb11 = {
_16 = '\u{dcfab}';
RET = _4;
_8 = -9223372036854775807_isize;
_15 = [2455721406_u32];
RET = [_11,_11,_11,_11,_11,_11,_11];
_8 = 18_isize;
RET = _4;
_7 = 1270274062161035592_u64;
RET = [_11,_11,_11,_11,_11,_11,_11];
_11 = !250731548963507774130919600906730504451_u128;
_4 = RET;
RET = [_11,_11,_11,_11,_11,_11,_11];
_10 = !8979_u16;
_14.1 = [16615095314388310958_usize,2_usize,5_usize,8934609155321760463_usize,17612587237107806742_usize,2168469619469634993_usize,4_usize,5_usize];
RET = _3;
_14.1 = [3_usize,16200011302798639149_usize,2903536641745322643_usize,5_usize,7127568584674477374_usize,1_usize,9506496930414081128_usize,8945581956059163037_usize];
_14.1 = [719159060079051932_usize,0_usize,0_usize,7_usize,17406466446496379012_usize,7531775815165403087_usize,6_usize,5247781447473086366_usize];
_7 = 15065164316756111545_u64;
RET = _3;
_6 = _14.1;
_2 = _5;
_10 = !13790_u16;
_8 = (-10025_i16) as isize;
match _7 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
15065164316756111545 => bb10,
_ => bb9
}
}
bb12 = {
_14.1 = [13816224499447982393_usize,2_usize,10400369356356895963_usize,1_usize,6_usize,2_usize,17020401167466331939_usize,5_usize];
_20 = _11;
_5 = _2;
(*_18) = _21.0 | _21.0;
_21 = ((*_18), 22125_i16, 6788061999255642462_i64);
_21.2 = _11 as i64;
match _21.1 {
0 => bb8,
22125 => bb14,
_ => bb13
}
}
bb13 = {
_5 = [2733988979_u32,3333404855_u32,3267513866_u32,3584629132_u32,3473952375_u32,2843691545_u32,4195627598_u32,2568040390_u32];
_12 = [true,false,true,true,false,true,false,true];
_6 = [3_usize,2_usize,6_usize,6_usize,9362291409319586323_usize,9470424390557804554_usize,7_usize,5600401468910629523_usize];
_7 = _8 as u64;
_5 = _2;
_11 = 147080189892491507100507785797891599441_u128 >> _7;
RET = [_11,_11,_11,_11,_11,_11,_11];
RET = [_11,_11,_11,_11,_11,_11,_11];
_7 = 18101456382208292231_u64 * 11883125899172399112_u64;
_12 = [true,false,true,true,true,false,false,false];
_4 = [_11,_11,_11,_11,_11,_11,_11];
_2 = [2892439525_u32,2792069254_u32,1598405275_u32,3514651425_u32,2104831317_u32,4103777723_u32,3679417933_u32,4226958829_u32];
_2 = [3209410888_u32,3468383189_u32,1599824561_u32,733218473_u32,456844969_u32,3573466795_u32,2913659406_u32,1921899622_u32];
_10 = !30874_u16;
_6 = [15872991330698714743_usize,8648323525648979240_usize,3_usize,10532875998387713932_usize,6631857343043700855_usize,5_usize,0_usize,0_usize];
RET = _3;
_10 = (-5496_i16) as u16;
_3 = RET;
RET = _3;
_3 = [_11,_11,_11,_11,_11,_11,_11];
_2 = [917798613_u32,1861131405_u32,2715871642_u32,1614897811_u32,4210135278_u32,1760449496_u32,3954791082_u32,3946161075_u32];
_14.1 = _6;
_7 = !9372364477713040063_u64;
Call(_13 = fn17(_12, _11, _2, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_3 = [_11,_11,_11,_11,_20,_11,_11];
_18 = core::ptr::addr_of_mut!((*_18));
_4 = RET;
RET = _4;
_15 = [1618944587_u32];
_16 = '\u{a5ab0}';
_19 = _8 as usize;
_16 = '\u{d5333}';
_22 = _16 as u16;
_12 = [false,false,true,false,true,false,false,false];
_23.0 = _21.1 as f32;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(16_usize, 11_usize, Move(_11), 8_usize, Move(_8), 3_usize, Move(_3), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(16_usize, 20_usize, Move(_20), 5_usize, Move(_5), 12_usize, Move(_12), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [bool; 8],mut _2: u128,mut _3: [u32; 8],mut _4: [bool; 8]) -> *mut u8 {
mir! {
type RET = *mut u8;
let _5: ([bool; 8], (f32,), &'static (u16, i16, i64));
let _6: f64;
let _7: isize;
let _8: isize;
let _9: f64;
let _10: bool;
let _11: &'static u128;
let _12: &'static (u16, f32);
let _13: [u128; 6];
let _14: bool;
let _15: ([usize; 4], [u32; 8], Adt55, u64);
let _16: isize;
let _17: &'static i64;
let _18: &'static bool;
let _19: f64;
let _20: [u32; 1];
let _21: [bool; 8];
let _22: bool;
let _23: f32;
let _24: u8;
let _25: *mut u16;
let _26: u16;
let _27: char;
let _28: f64;
let _29: [bool; 5];
let _30: ();
let _31: ();
{
_1 = [false,true,true,false,false,false,true,false];
_4 = [true,true,false,true,true,true,false,true];
_2 = 13579960492327824282_u64 as u128;
_2 = 170280228524527193473512039337618854995_u128;
_2 = (-26531_i16) as u128;
_3 = [3859744803_u32,1384880572_u32,3438656180_u32,3887904129_u32,3908868073_u32,2296743558_u32,235425353_u32,1739679977_u32];
_2 = !220593358437967915583970454484430179563_u128;
_4 = _1;
_1 = [true,true,true,false,false,false,false,true];
_1 = [true,false,true,false,true,true,true,false];
_5.0 = [false,true,false,true,false,true,false,true];
_5.1.0 = 9223372036854775807_isize as f32;
_3 = [3727051767_u32,1230937234_u32,3176918015_u32,472651484_u32,2582824669_u32,2769837270_u32,91179510_u32,999481922_u32];
_5.1.0 = 70_u8 as f32;
_4 = [false,false,true,false,true,false,false,false];
_1 = _5.0;
_6 = (-2138431110_i32) as f64;
_4 = [true,true,false,true,false,true,false,false];
_1 = [false,false,true,true,false,false,true,false];
_7 = false as isize;
Goto(bb1)
}
bb1 = {
_3 = [1321524399_u32,481473731_u32,2333584004_u32,792883778_u32,1759925748_u32,2112220705_u32,2569529475_u32,4277673455_u32];
_7 = -(-9223372036854775808_isize);
_3 = [4074078065_u32,2062430285_u32,1858302936_u32,2634511256_u32,2868624064_u32,3158233908_u32,951361551_u32,473669254_u32];
Goto(bb2)
}
bb2 = {
_3 = [902498426_u32,3044621282_u32,3305450209_u32,1821201462_u32,332192489_u32,2226667546_u32,684480565_u32,2061357961_u32];
_6 = 11317109863541346297_u64 as f64;
_2 = !61790566693358764102835776666636795829_u128;
Goto(bb3)
}
bb3 = {
_2 = 173_u8 as u128;
_7 = !(-6_isize);
_5.1.0 = _7 as f32;
_8 = _7 - _7;
Goto(bb4)
}
bb4 = {
_8 = _7 << _7;
_2 = 225414018086231507793657602032819092038_u128 * 277131520121770502510817219663188368231_u128;
_9 = -_6;
_9 = _6 * _6;
_5.0 = _4;
_5.0 = _1;
_6 = _8 as f64;
_9 = _6;
_5.1.0 = 19911_u16 as f32;
_10 = _7 >= _8;
_2 = !144410160345370572895805680304882107325_u128;
Goto(bb5)
}
bb5 = {
_9 = _6;
_11 = &_2;
_6 = -_9;
_5.0 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1 = [_10,_10,_10,_10,_10,_10,_10,_10];
_2 = 290367396076819926725248320168477631908_u128;
_13 = [_2,_2,_2,_2,_2,_2];
_9 = _6 + _6;
_2 = 106799286259202046946701116002195807959_u128;
_14 = _10 & _10;
_5.1.0 = 3046252598486333963_i64 as f32;
Goto(bb6)
}
bb6 = {
_15.2.fld1.fld3 = _2 as i8;
_15.2.fld3 = _15.2.fld1.fld3;
_15.2.fld5 = [6_usize,2_usize];
_15.3 = (-7889970879954357154_i64) as u64;
_5.1.0 = _15.2.fld1.fld3 as f32;
_15.2.fld1.fld1 = _8 as u32;
_13 = [_2,_2,_2,_2,_2,_2];
_15.2.fld2 = [_10,_14,_14,_14,_10];
_3 = [_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1];
_15.2.fld1.fld2 = _2 | _2;
_7 = (-1781284689320599246_i64) as isize;
_15.1 = [_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1];
_11 = &_15.2.fld1.fld2;
_16 = -_7;
_5.0 = [_10,_14,_14,_10,_14,_10,_14,_10];
_18 = &_10;
_19 = 4_usize as f64;
_6 = -_19;
_19 = -_9;
_15.2.fld4 = -142792030822244928383728826547277345430_i128;
_19 = -_6;
_21 = [_14,(*_18),(*_18),_14,(*_18),(*_18),_14,_14];
Goto(bb7)
}
bb7 = {
_15.0 = [13170604072295636181_usize,3_usize,3_usize,14330976157918011228_usize];
match _2 {
0 => bb1,
106799286259202046946701116002195807959 => bb8,
_ => bb6
}
}
bb8 = {
_11 = &_2;
_15.2.fld1.fld4 = (-13355_i16) >> _15.2.fld3;
_15.2.fld1.fld7 = -_15.2.fld4;
_11 = &_2;
_14 = (*_18);
_15.2.fld1.fld5 = !9611_u16;
_21 = _4;
_15.2.fld3 = _9 as i8;
_5.1.0 = (-212543768_i32) as f32;
_11 = &(*_11);
_6 = _19 + _9;
_15.2.fld1.fld0 = _14 as u64;
_15.2.fld1.fld6 = Adt18::Variant1 { fld0: 107_u8,fld1: (*_11) };
_15.2.fld1.fld4 = (-32437_i16) ^ (-17385_i16);
_15.2.fld1.fld5 = 45301_u16 ^ 36617_u16;
_3 = [_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1];
_22 = _16 > _16;
match Field::<u128>(Variant(_15.2.fld1.fld6, 1), 1) {
0 => bb4,
1 => bb2,
2 => bb7,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
106799286259202046946701116002195807959 => bb14,
_ => bb13
}
}
bb9 = {
_3 = [1321524399_u32,481473731_u32,2333584004_u32,792883778_u32,1759925748_u32,2112220705_u32,2569529475_u32,4277673455_u32];
_7 = -(-9223372036854775808_isize);
_3 = [4074078065_u32,2062430285_u32,1858302936_u32,2634511256_u32,2868624064_u32,3158233908_u32,951361551_u32,473669254_u32];
Goto(bb2)
}
bb10 = {
_15.2.fld1.fld3 = _2 as i8;
_15.2.fld3 = _15.2.fld1.fld3;
_15.2.fld5 = [6_usize,2_usize];
_15.3 = (-7889970879954357154_i64) as u64;
_5.1.0 = _15.2.fld1.fld3 as f32;
_15.2.fld1.fld1 = _8 as u32;
_13 = [_2,_2,_2,_2,_2,_2];
_15.2.fld2 = [_10,_14,_14,_14,_10];
_3 = [_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1];
_15.2.fld1.fld2 = _2 | _2;
_7 = (-1781284689320599246_i64) as isize;
_15.1 = [_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1,_15.2.fld1.fld1];
_11 = &_15.2.fld1.fld2;
_16 = -_7;
_5.0 = [_10,_14,_14,_10,_14,_10,_14,_10];
_18 = &_10;
_19 = 4_usize as f64;
_6 = -_19;
_19 = -_9;
_15.2.fld4 = -142792030822244928383728826547277345430_i128;
_19 = -_6;
_21 = [_14,(*_18),(*_18),_14,(*_18),(*_18),_14,_14];
Goto(bb7)
}
bb11 = {
_3 = [902498426_u32,3044621282_u32,3305450209_u32,1821201462_u32,332192489_u32,2226667546_u32,684480565_u32,2061357961_u32];
_6 = 11317109863541346297_u64 as f64;
_2 = !61790566693358764102835776666636795829_u128;
Goto(bb3)
}
bb12 = {
_8 = _7 << _7;
_2 = 225414018086231507793657602032819092038_u128 * 277131520121770502510817219663188368231_u128;
_9 = -_6;
_9 = _6 * _6;
_5.0 = _4;
_5.0 = _1;
_6 = _8 as f64;
_9 = _6;
_5.1.0 = 19911_u16 as f32;
_10 = _7 >= _8;
_2 = !144410160345370572895805680304882107325_u128;
Goto(bb5)
}
bb13 = {
_2 = 173_u8 as u128;
_7 = !(-6_isize);
_5.1.0 = _7 as f32;
_8 = _7 - _7;
Goto(bb4)
}
bb14 = {
_21 = [(*_18),_14,(*_18),_14,(*_18),(*_18),_14,_10];
_15.2.fld0 = core::ptr::addr_of_mut!(_15.2.fld1.fld1);
_15.2.fld1.fld1 = 2929183747_u32;
_15.2.fld5 = [18264761671071537837_usize,4_usize];
_25 = core::ptr::addr_of_mut!(_15.2.fld1.fld5);
_15.0 = [18113999030604920961_usize,6135173922997380280_usize,7_usize,3_usize];
_20 = [_15.2.fld1.fld1];
_24 = _15.2.fld1.fld4 as u8;
_11 = &(*_11);
_15.2.fld1.fld7 = _15.2.fld4;
(*_25) = !31885_u16;
_2 = !Field::<u128>(Variant(_15.2.fld1.fld6, 1), 1);
RET = core::ptr::addr_of_mut!(_24);
_15.1 = _3;
RET = core::ptr::addr_of_mut!((*RET));
_15.2.fld1.fld3 = _15.2.fld3 >> (*RET);
_15.3 = _15.2.fld1.fld0 ^ _15.2.fld1.fld0;
_15.2.fld0 = core::ptr::addr_of_mut!(_15.2.fld1.fld1);
_23 = _6 as f32;
_18 = &(*_18);
_15.2.fld2 = [_22,(*_18),_22,(*_18),_10];
_21 = _4;
_15.2.fld3 = _15.2.fld1.fld1 as i8;
place!(Field::<u8>(Variant(_15.2.fld1.fld6, 1), 0)) = !(*RET);
_5.1 = (_23,);
_15.2.fld1.fld6 = Adt18::Variant2 { fld0: _5.1.0,fld1: '\u{6e853}',fld2: _6,fld3: (*RET),fld4: _15.2.fld1.fld4,fld5: 406318920_i32,fld6: _15.3,fld7: _15.2.fld1.fld7 };
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(17_usize, 4_usize, Move(_4), 3_usize, Move(_3), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(17_usize, 16_usize, Move(_16), 1_usize, Move(_1), 7_usize, Move(_7), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [bool; 8],mut _2: [bool; 5],mut _3: [u64; 6],mut _4: u8,mut _5: [bool; 5],mut _6: f64,mut _7: [u64; 6],mut _8: [u32; 1],mut _9: [u64; 6],mut _10: f64,mut _11: [usize; 2]) -> char {
mir! {
type RET = char;
let _12: &'static [usize; 2];
let _13: bool;
let _14: isize;
let _15: i8;
let _16: u32;
let _17: i8;
let _18: *mut [i8; 3];
let _19: bool;
let _20: &'static [usize; 2];
let _21: Adt18;
let _22: char;
let _23: f32;
let _24: f64;
let _25: &'static (u16, f32);
let _26: ();
let _27: ();
{
_4 = 201_u8;
RET = '\u{55bdd}';
_8 = [881124215_u32];
_2 = _5;
_5 = [false,false,false,true,true];
_12 = &_11;
_11 = [10187876663547959530_usize,10443318403887281273_usize];
_12 = &_11;
_9 = [18370661640544768684_u64,2391682512419660_u64,7524945183149801434_u64,7674720092079118447_u64,2295782391737000127_u64,16786912179898716670_u64];
_10 = _6 * _6;
_13 = !false;
_13 = true;
_12 = &(*_12);
RET = '\u{f41ca}';
_2 = _5;
RET = '\u{644f1}';
_6 = _10 + _10;
Goto(bb1)
}
bb1 = {
_13 = _4 == _4;
_9 = _3;
_4 = 569241870_u32 as u8;
_4 = !15_u8;
_10 = (-49_i8) as f64;
_13 = _6 != _6;
_6 = -_10;
_14 = (-9223372036854775808_isize);
_8 = [2617531783_u32];
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
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
_12 = &(*_12);
_10 = _6;
_7 = [14153855804312755108_u64,12837227356781098837_u64,18435812295381526608_u64,10541155463747818909_u64,6225085878638973287_u64,3061693033367963031_u64];
_11 = [8648055650664073933_usize,10846788446819907873_usize];
_2 = [_13,_13,_13,_13,_13];
_14 = 9223372036854775807_isize;
_3 = [2242974094994541577_u64,4674230618218160920_u64,9952333710703708040_u64,14229811708415796211_u64,466908770603740384_u64,609171379852131221_u64];
_6 = _10 - _10;
_12 = &_11;
_2 = _5;
_16 = 3691809209_u32 | 790045177_u32;
_10 = 184464867539826009747073963560813465204_u128 as f64;
_3 = [10506596444567968442_u64,16721746173414399849_u64,15967153615858193446_u64,9931750199338264836_u64,4670247460581689391_u64,16511463702998734062_u64];
_15 = (-32_i8);
_10 = -_6;
_11 = [4667080378999278382_usize,16396895564669951131_usize];
match _15 {
0 => bb10,
340282366920938463463374607431768211424 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_5 = [_13,_13,_13,_13,_13];
_12 = &_11;
_7 = [13347517310145432971_u64,16610631756935799734_u64,9844795677360862258_u64,15357016082003627767_u64,12620749941257328659_u64,9481741212631283595_u64];
_7 = _9;
_1 = [_13,_13,_13,_13,_13,_13,_13,_13];
_4 = _15 as u8;
Goto(bb13)
}
bb13 = {
_3 = [1607034744891690667_u64,8063704017998125674_u64,14267947386837195711_u64,15062547699835878114_u64,11038785774869562647_u64,15875451888302556832_u64];
_15 = 2_usize as i8;
_3 = _7;
_14 = 120_isize;
_14 = 9223372036854775807_isize;
_4 = 142_u8 + 148_u8;
_14 = !(-16_isize);
RET = '\u{b2338}';
_3 = [11098510485227919016_u64,5722309915473966939_u64,13616435362916603406_u64,2980736970689954120_u64,1430892281708889552_u64,4713428618093656073_u64];
_6 = 29240_i16 as f64;
_20 = &(*_12);
_19 = _13;
RET = '\u{6229}';
_17 = _15;
_16 = _10 as u32;
_22 = RET;
_19 = _13;
_10 = 60778161299298580219322373548039410367_i128 as f64;
_1 = [_19,_19,_13,_19,_13,_19,_19,_19];
_13 = !_19;
RET = _22;
_19 = !_13;
_20 = &(*_20);
_21 = Adt18::Variant1 { fld0: _4,fld1: 84708313607646465137307521349045203915_u128 };
place!(Field::<u128>(Variant(_21, 1), 1)) = 22937848013112356552663966461764139439_u128;
Goto(bb14)
}
bb14 = {
_3 = [7069286608414825453_u64,18436749692308992552_u64,5944297950992327824_u64,16678855123475917791_u64,17185140443142761419_u64,12608158579856924562_u64];
place!(Field::<u128>(Variant(_21, 1), 1)) = (-38375600669044377038804271678560630530_i128) as u128;
_8 = [_16];
RET = _22;
_2 = [_19,_19,_13,_19,_13];
_10 = -_6;
_23 = _15 as f32;
_15 = _17;
_6 = _10;
_2 = [_13,_13,_19,_13,_19];
_19 = Field::<u8>(Variant(_21, 1), 0) < _4;
_13 = _19;
place!(Field::<u8>(Variant(_21, 1), 0)) = !_4;
RET = _22;
_6 = -_10;
_13 = !_19;
_16 = 3324374657_u32;
RET = _22;
_3 = [732823826765039659_u64,15640013657021240079_u64,13623648116963174461_u64,272317711330452238_u64,9472298920743690711_u64,692021586014721100_u64];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(18_usize, 3_usize, Move(_3), 16_usize, Move(_16), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(18_usize, 7_usize, Move(_7), 11_usize, Move(_11), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: u16,mut _2: u32,mut _3: u16) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _4: &'static i128;
let _5: [u64; 6];
let _6: Adt29;
let _7: [bool; 5];
let _8: &'static u16;
let _9: *const *mut ([bool; 8], (f32,), &'static (u16, i16, i64));
let _10: Adt46;
let _11: isize;
let _12: Adt65;
let _13: i16;
let _14: (u16, f32);
let _15: isize;
let _16: (u64, &'static i8, *const f32);
let _17: &'static i32;
let _18: ([bool; 8], (f32,), &'static (u16, i16, i64));
let _19: ();
let _20: ();
{
RET = [202600134293058158216641667447713099846_u128,294235590972925717503314298228666408578_u128,182621620476079513399251362615512841145_u128,267006192887998533403963432490391395928_u128,159816151219994229852122859624003216842_u128,272291273600753474539101748698927886189_u128,137892810456362653287139461212452593081_u128];
Goto(bb1)
}
bb1 = {
RET = [304872237814113768363509236974231077025_u128,104322051250930784746436077215084515915_u128,86290336627578236413290400098837404910_u128,270963845812726720009601407314623602374_u128,209818434002082212036850225483925062983_u128,14719337331031717608015296785925051126_u128,30611416413262474087413143713409955155_u128];
_3 = _1;
_3 = 5_usize as u16;
RET = [148666702044229439062348417186891124635_u128,305579158765481468119093174628694140656_u128,208744077939440042420880414927116135047_u128,22817467733111233907561841042080311550_u128,83033061628368024948956314342661702162_u128,13214560668233054209512128099875765335_u128,84940645929627175792907944221123837011_u128];
_1 = _3;
_2 = 3129187256_u32 ^ 3811591218_u32;
_3 = _1;
RET = [106279588595298962501133845317525185698_u128,162850444626909680801851065447554893801_u128,23770205452301733218722606018190664981_u128,62590313533338000126685685782219787709_u128,138801210522981700493584547239085226371_u128,278089193363073990248475641040205383432_u128,229967843807875816001897393689002092213_u128];
_3 = !_1;
RET = [224903160761983563025088561335110819198_u128,67689425020964405222511945437894282490_u128,167227802724720047349223970982234458361_u128,36712252839903644366901423445231526113_u128,104815103178546612479791210141109591758_u128,191173730705357174073359054040504873331_u128,144934582697388412226306440211609543743_u128];
_1 = _3 << _2;
RET = [329246370768330203673414627136605623185_u128,296454644217015977672129916634157344188_u128,243719922915830184726306795790705019875_u128,217724699068287089205757658550140928623_u128,124363516132665753417006556335586802973_u128,18260270627173028925024398240950500114_u128,124207810088403817877761580007670285507_u128];
_3 = _1 >> _2;
_3 = _1 + _1;
_6.fld3 = 126_i8;
Goto(bb2)
}
bb2 = {
_1 = _3 << _3;
_4 = &_6.fld7;
Goto(bb3)
}
bb3 = {
_6.fld7 = 98704674676301575781752045189845873154_i128;
_6.fld7 = (-142623084251563978658009820427526056719_i128);
_6.fld6 = Adt18::Variant1 { fld0: 203_u8,fld1: 160126310939189866922298428134869520411_u128 };
_6.fld0 = !4456217645301274152_u64;
_5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_6.fld5 = true as u16;
place!(Field::<u128>(Variant(_6.fld6, 1), 1)) = 164164291745059149057917616764680420878_u128;
_6.fld5 = _6.fld0 as u16;
RET = [Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1)];
_6.fld5 = _1 << _1;
match Field::<u128>(Variant(_6.fld6, 1), 1) {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
164164291745059149057917616764680420878 => bb9,
_ => bb8
}
}
bb4 = {
_1 = _3 << _3;
_4 = &_6.fld7;
Goto(bb3)
}
bb5 = {
RET = [304872237814113768363509236974231077025_u128,104322051250930784746436077215084515915_u128,86290336627578236413290400098837404910_u128,270963845812726720009601407314623602374_u128,209818434002082212036850225483925062983_u128,14719337331031717608015296785925051126_u128,30611416413262474087413143713409955155_u128];
_3 = _1;
_3 = 5_usize as u16;
RET = [148666702044229439062348417186891124635_u128,305579158765481468119093174628694140656_u128,208744077939440042420880414927116135047_u128,22817467733111233907561841042080311550_u128,83033061628368024948956314342661702162_u128,13214560668233054209512128099875765335_u128,84940645929627175792907944221123837011_u128];
_1 = _3;
_2 = 3129187256_u32 ^ 3811591218_u32;
_3 = _1;
RET = [106279588595298962501133845317525185698_u128,162850444626909680801851065447554893801_u128,23770205452301733218722606018190664981_u128,62590313533338000126685685782219787709_u128,138801210522981700493584547239085226371_u128,278089193363073990248475641040205383432_u128,229967843807875816001897393689002092213_u128];
_3 = !_1;
RET = [224903160761983563025088561335110819198_u128,67689425020964405222511945437894282490_u128,167227802724720047349223970982234458361_u128,36712252839903644366901423445231526113_u128,104815103178546612479791210141109591758_u128,191173730705357174073359054040504873331_u128,144934582697388412226306440211609543743_u128];
_1 = _3 << _2;
RET = [329246370768330203673414627136605623185_u128,296454644217015977672129916634157344188_u128,243719922915830184726306795790705019875_u128,217724699068287089205757658550140928623_u128,124363516132665753417006556335586802973_u128,18260270627173028925024398240950500114_u128,124207810088403817877761580007670285507_u128];
_3 = _1 >> _2;
_3 = _1 + _1;
_6.fld3 = 126_i8;
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
_6.fld4 = 2430_i16;
_6.fld2 = _6.fld3 as u128;
place!(Field::<u8>(Variant(_6.fld6, 1), 0)) = 120_u8 | 72_u8;
_6.fld5 = !_1;
_1 = _3 >> _6.fld5;
_6.fld5 = _1 | _1;
_6.fld3 = 31_i8 << _6.fld5;
_6.fld5 = _6.fld0 as u16;
RET = [Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),_6.fld2,Field::<u128>(Variant(_6.fld6, 1), 1),_6.fld2,_6.fld2,Field::<u128>(Variant(_6.fld6, 1), 1)];
_7 = [true,false,false,true,true];
_1 = !_3;
RET = [Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),_6.fld2,_6.fld2,Field::<u128>(Variant(_6.fld6, 1), 1),_6.fld2,_6.fld2];
_4 = &_6.fld7;
_6.fld4 = (-5400_i16);
place!(Field::<u128>(Variant(_6.fld6, 1), 1)) = _6.fld2;
match (*_4) {
0 => bb8,
1 => bb2,
2 => bb10,
3 => bb11,
197659282669374484805364787004242154737 => bb13,
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
RET = [304872237814113768363509236974231077025_u128,104322051250930784746436077215084515915_u128,86290336627578236413290400098837404910_u128,270963845812726720009601407314623602374_u128,209818434002082212036850225483925062983_u128,14719337331031717608015296785925051126_u128,30611416413262474087413143713409955155_u128];
_3 = _1;
_3 = 5_usize as u16;
RET = [148666702044229439062348417186891124635_u128,305579158765481468119093174628694140656_u128,208744077939440042420880414927116135047_u128,22817467733111233907561841042080311550_u128,83033061628368024948956314342661702162_u128,13214560668233054209512128099875765335_u128,84940645929627175792907944221123837011_u128];
_1 = _3;
_2 = 3129187256_u32 ^ 3811591218_u32;
_3 = _1;
RET = [106279588595298962501133845317525185698_u128,162850444626909680801851065447554893801_u128,23770205452301733218722606018190664981_u128,62590313533338000126685685782219787709_u128,138801210522981700493584547239085226371_u128,278089193363073990248475641040205383432_u128,229967843807875816001897393689002092213_u128];
_3 = !_1;
RET = [224903160761983563025088561335110819198_u128,67689425020964405222511945437894282490_u128,167227802724720047349223970982234458361_u128,36712252839903644366901423445231526113_u128,104815103178546612479791210141109591758_u128,191173730705357174073359054040504873331_u128,144934582697388412226306440211609543743_u128];
_1 = _3 << _2;
RET = [329246370768330203673414627136605623185_u128,296454644217015977672129916634157344188_u128,243719922915830184726306795790705019875_u128,217724699068287089205757658550140928623_u128,124363516132665753417006556335586802973_u128,18260270627173028925024398240950500114_u128,124207810088403817877761580007670285507_u128];
_3 = _1 >> _2;
_3 = _1 + _1;
_6.fld3 = 126_i8;
Goto(bb2)
}
bb13 = {
SetDiscriminant(_6.fld6, 0);
_1 = _3 + _3;
_4 = &(*_4);
_7 = [true,false,true,true,true];
_7 = [true,true,true,true,false];
place!(Field::<u64>(Variant(_6.fld6, 0), 3)) = _6.fld0 | _6.fld0;
place!(Field::<u64>(Variant(_6.fld6, 0), 3)) = _6.fld0;
_6.fld1 = _6.fld0 as u32;
_1 = !_3;
place!(Field::<*const f64>(Variant(_6.fld6, 0), 0)) = core::ptr::addr_of!(place!(Field::<f64>(Variant(_6.fld6, 0), 2)));
_1 = !_6.fld5;
place!(Field::<u64>(Variant(_6.fld6, 0), 3)) = _6.fld0;
_1 = _3 + _3;
place!(Field::<u128>(Variant(_6.fld6, 0), 4)) = (*_4) as u128;
_2 = _6.fld1;
_8 = &_3;
_6.fld5 = (*_8) * _1;
match _6.fld4 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
340282366920938463463374607431768206056 => bb20,
_ => bb19
}
}
bb14 = {
RET = [304872237814113768363509236974231077025_u128,104322051250930784746436077215084515915_u128,86290336627578236413290400098837404910_u128,270963845812726720009601407314623602374_u128,209818434002082212036850225483925062983_u128,14719337331031717608015296785925051126_u128,30611416413262474087413143713409955155_u128];
_3 = _1;
_3 = 5_usize as u16;
RET = [148666702044229439062348417186891124635_u128,305579158765481468119093174628694140656_u128,208744077939440042420880414927116135047_u128,22817467733111233907561841042080311550_u128,83033061628368024948956314342661702162_u128,13214560668233054209512128099875765335_u128,84940645929627175792907944221123837011_u128];
_1 = _3;
_2 = 3129187256_u32 ^ 3811591218_u32;
_3 = _1;
RET = [106279588595298962501133845317525185698_u128,162850444626909680801851065447554893801_u128,23770205452301733218722606018190664981_u128,62590313533338000126685685782219787709_u128,138801210522981700493584547239085226371_u128,278089193363073990248475641040205383432_u128,229967843807875816001897393689002092213_u128];
_3 = !_1;
RET = [224903160761983563025088561335110819198_u128,67689425020964405222511945437894282490_u128,167227802724720047349223970982234458361_u128,36712252839903644366901423445231526113_u128,104815103178546612479791210141109591758_u128,191173730705357174073359054040504873331_u128,144934582697388412226306440211609543743_u128];
_1 = _3 << _2;
RET = [329246370768330203673414627136605623185_u128,296454644217015977672129916634157344188_u128,243719922915830184726306795790705019875_u128,217724699068287089205757658550140928623_u128,124363516132665753417006556335586802973_u128,18260270627173028925024398240950500114_u128,124207810088403817877761580007670285507_u128];
_3 = _1 >> _2;
_3 = _1 + _1;
_6.fld3 = 126_i8;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_6.fld7 = 98704674676301575781752045189845873154_i128;
_6.fld7 = (-142623084251563978658009820427526056719_i128);
_6.fld6 = Adt18::Variant1 { fld0: 203_u8,fld1: 160126310939189866922298428134869520411_u128 };
_6.fld0 = !4456217645301274152_u64;
_5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_6.fld5 = true as u16;
place!(Field::<u128>(Variant(_6.fld6, 1), 1)) = 164164291745059149057917616764680420878_u128;
_6.fld5 = _6.fld0 as u16;
RET = [Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1),Field::<u128>(Variant(_6.fld6, 1), 1)];
_6.fld5 = _1 << _1;
match Field::<u128>(Variant(_6.fld6, 1), 1) {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
164164291745059149057917616764680420878 => bb9,
_ => bb8
}
}
bb18 = {
_1 = _3 << _3;
_4 = &_6.fld7;
Goto(bb3)
}
bb19 = {
RET = [304872237814113768363509236974231077025_u128,104322051250930784746436077215084515915_u128,86290336627578236413290400098837404910_u128,270963845812726720009601407314623602374_u128,209818434002082212036850225483925062983_u128,14719337331031717608015296785925051126_u128,30611416413262474087413143713409955155_u128];
_3 = _1;
_3 = 5_usize as u16;
RET = [148666702044229439062348417186891124635_u128,305579158765481468119093174628694140656_u128,208744077939440042420880414927116135047_u128,22817467733111233907561841042080311550_u128,83033061628368024948956314342661702162_u128,13214560668233054209512128099875765335_u128,84940645929627175792907944221123837011_u128];
_1 = _3;
_2 = 3129187256_u32 ^ 3811591218_u32;
_3 = _1;
RET = [106279588595298962501133845317525185698_u128,162850444626909680801851065447554893801_u128,23770205452301733218722606018190664981_u128,62590313533338000126685685782219787709_u128,138801210522981700493584547239085226371_u128,278089193363073990248475641040205383432_u128,229967843807875816001897393689002092213_u128];
_3 = !_1;
RET = [224903160761983563025088561335110819198_u128,67689425020964405222511945437894282490_u128,167227802724720047349223970982234458361_u128,36712252839903644366901423445231526113_u128,104815103178546612479791210141109591758_u128,191173730705357174073359054040504873331_u128,144934582697388412226306440211609543743_u128];
_1 = _3 << _2;
RET = [329246370768330203673414627136605623185_u128,296454644217015977672129916634157344188_u128,243719922915830184726306795790705019875_u128,217724699068287089205757658550140928623_u128,124363516132665753417006556335586802973_u128,18260270627173028925024398240950500114_u128,124207810088403817877761580007670285507_u128];
_3 = _1 >> _2;
_3 = _1 + _1;
_6.fld3 = 126_i8;
Goto(bb2)
}
bb20 = {
_6.fld0 = !Field::<u64>(Variant(_6.fld6, 0), 3);
_6.fld5 = (*_8) ^ _1;
_6.fld7 = -16729769752902095686741272983234016916_i128;
_3 = _6.fld5;
place!(Field::<u128>(Variant(_6.fld6, 0), 4)) = _6.fld2;
_11 = !3_isize;
place!(Field::<f64>(Variant(_6.fld6, 0), 2)) = _2 as f64;
_14.0 = _11 as u16;
_6.fld7 = (-148270820121581098647794259216687772760_i128) * (-141450442847211299187815931579781176823_i128);
_5 = [_6.fld0,Field::<u64>(Variant(_6.fld6, 0), 3),_6.fld0,Field::<u64>(Variant(_6.fld6, 0), 3),_6.fld0,_6.fld0];
_15 = !_11;
place!(Field::<*const f64>(Variant(_6.fld6, 0), 0)) = core::ptr::addr_of!(place!(Field::<f64>(Variant(_6.fld6, 0), 2)));
_11 = _15 - _15;
_13 = _6.fld4 ^ _6.fld4;
_10 = Adt46::Variant1 { fld0: true,fld1: RET,fld2: Field::<f64>(Variant(_6.fld6, 0), 2) };
_6.fld1 = Field::<u128>(Variant(_6.fld6, 0), 4) as u32;
_16.1 = &_6.fld3;
_1 = !_3;
_7 = [false,false,true,true,true];
_14.1 = _6.fld7 as f32;
_16.0 = _6.fld3 as u64;
place!(Field::<bool>(Variant(_10, 1), 0)) = true ^ false;
_18.0 = [Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0),Field::<bool>(Variant(_10, 1), 0)];
place!(Field::<[u128; 7]>(Variant(_10, 1), 1)) = RET;
SetDiscriminant(_10, 0);
place!(Field::<u128>(Variant(_6.fld6, 0), 4)) = !_6.fld2;
_6.fld0 = 92_u8 as u64;
Goto(bb21)
}
bb21 = {
Call(_19 = dump_var(19_usize, 11_usize, Move(_11), 2_usize, Move(_2), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(223868997114875175131971992126435277153_u128), std::hint::black_box('\u{27a6a}'), std::hint::black_box(13449621951000082020_u64), std::hint::black_box(3284434050_u32), std::hint::black_box(10749_i16), std::hint::black_box((-2070433840_i32)), std::hint::black_box((-5530256503672652551_i64)), std::hint::black_box((-76983917435381032919589439995829946077_i128)), std::hint::black_box(3_usize), std::hint::black_box(36_u8));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: *const f64,
fld1: char,
fld2: f64,
fld3: u64,
fld4: u128,

},
Variant1{
fld0: u8,
fld1: u128,

},
Variant2{
fld0: f32,
fld1: char,
fld2: f64,
fld3: u8,
fld4: i16,
fld5: i32,
fld6: u64,
fld7: i128,

}}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt29{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt29 {
fld0: u64,
fld1: u32,
fld2: u128,
fld3: i8,
fld4: i16,
fld5: u16,
fld6: Adt18,
fld7: i128,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *const [u128; 7],
fld1: Adt18,
fld2: isize,
fld3: i8,
fld4: [i8; 3],
fld5: u64,
fld6: i64,
fld7: u8,

},
Variant1{
fld0: bool,
fld1: [u128; 7],
fld2: f64,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: *mut u32,
fld1: Adt29,
fld2: [bool; 5],
fld3: i8,
fld4: i128,
fld5: [usize; 2],
}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: *mut [i8; 3],

},
Variant1{
fld0: [usize; 4],
fld1: Adt46,

},
Variant2{
fld0: u8,
fld1: char,

},
Variant3{
fld0: (f64,),
fld1: [i8; 3],

}}
impl PrintFDebug for Adt71{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt71{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt71 {
fld0: bool,
fld1: *mut [i8; 3],
fld2: [u128; 7],
fld3: (u16, f32),
fld4: *mut u16,
fld5: Adt29,
fld6: i64,
fld7: [usize; 2],
}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: Adt71,
fld1: char,
fld2: [u32; 1],
fld3: (u16, f32),
fld4: *mut [i8; 3],
fld5: *const [u128; 7],
fld6: Adt18,

},
Variant1{
fld0: [bool; 5],
fld1: Adt71,
fld2: [usize; 8],
fld3: i8,
fld4: u32,
fld5: *mut *const [u128; 7],
fld6: u8,
fld7: i128,

},
Variant2{
fld0: *mut [i8; 3],
fld1: char,
fld2: Adt65,
fld3: Adt18,
fld4: [u128; 7],
fld5: i32,
fld6: *const f32,
fld7: [u128; 6],

},
Variant3{
fld0: char,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt75{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt75 {
fld0: (u16, f32),
fld1: *mut u16,
}

