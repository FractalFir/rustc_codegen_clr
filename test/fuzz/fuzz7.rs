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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> usize {
mir! {
type RET = usize;
let _15: u16;
let _16: char;
let _17: u16;
let _18: char;
let _19: f64;
let _20: (u64, i128, isize);
let _21: [u128; 5];
let _22: char;
let _23: u64;
let _24: i16;
let _25: Adt57;
let _26: [i128; 3];
let _27: Adt49;
let _28: [i64; 2];
let _29: (u64, i128, isize);
let _30: f64;
let _31: [char; 1];
let _32: f64;
let _33: f64;
let _34: (i32, i128);
let _35: [bool; 1];
let _36: (i32, i128);
let _37: (i32, i128);
let _38: [u128; 5];
let _39: Adt57;
let _40: i64;
let _41: ();
let _42: ();
{
_1 = false;
_8 = !(-109860883394670717655164685005606479075_i128);
_14 = 62667_u16 as u128;
_6 = (-1223699288_i32) << _14;
_5 = -29750_i16;
_12 = _6 as u32;
_16 = '\u{319ab}';
_4 = (-50_i8) << _12;
_2 = _16;
Goto(bb1)
}
bb1 = {
_7 = !1061029002229257806_i64;
_3 = (-9223372036854775808_isize);
_8 = (-35617543568345792871968631203560807436_i128);
_17 = !48915_u16;
_13 = 12296166995936337805_u64;
_20.2 = _3 + _3;
_12 = !1739523564_u32;
_3 = _20.2 ^ _20.2;
_18 = _16;
_12 = _5 as u32;
_19 = 3_usize as f64;
_15 = _5 as u16;
_20.2 = _3 - _3;
_15 = _12 as u16;
_20.2 = _3;
_4 = 20_i8 - (-100_i8);
_20.0 = _13;
_20.2 = !_3;
RET = 8413293791410142843_usize >> _3;
_9 = RET;
_6 = (-1253504174_i32) ^ 583954925_i32;
_20.2 = -_3;
_6 = (-189219100_i32);
_20 = (_13, _8, _3);
_10 = 42_u8 >> _4;
_4 = (-47_i8);
_12 = !1284312602_u32;
_12 = 3635640262_u32 >> RET;
match _8 {
304664823352592670591405976228207404020 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET = _9 * _9;
_2 = _18;
_10 = 32_u8;
_11 = _15;
_3 = _20.2;
_21 = [_14,_14,_14,_14,_14];
_18 = _16;
_1 = true | true;
_4 = 42_i8 & 70_i8;
_2 = _18;
_20.1 = _8 * _8;
_18 = _2;
_20 = (_13, _8, _3);
_18 = _16;
_1 = !true;
_6 = !1308602270_i32;
_6 = _7 as i32;
Call(RET = fn1(_20.2, _10, _20, _20, _20, _12, _20, _3, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = (-97_i8);
RET = _9;
Call(_20.1 = core::intrinsics::transmute(_8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20.0 = !_13;
_4 = -(-42_i8);
RET = _9;
_3 = _20.2 * _20.2;
_21 = [_14,_14,_14,_14,_14];
_15 = !_17;
_15 = _17;
_23 = !_13;
_17 = _11;
_16 = _2;
_10 = !236_u8;
_16 = _18;
_26 = [_8,_20.1,_8];
_24 = _5 * _5;
_20 = (_13, _8, _3);
_4 = -88_i8;
_23 = _13 << _3;
_3 = -_20.2;
_20.1 = _1 as i128;
_12 = 3207248158_u32 << _20.2;
_16 = _18;
_26 = [_20.1,_8,_8];
_10 = 195_u8;
Goto(bb6)
}
bb6 = {
_17 = _11;
_19 = _20.2 as f64;
_15 = !_17;
_16 = _18;
_20.1 = _8;
_3 = _20.2;
_18 = _2;
_23 = _12 as u64;
RET = !_9;
_27.fld2 = _19 + _19;
_27.fld0.1 = [_7,_7];
_20.1 = _8;
_5 = -_24;
_20.2 = -_3;
_14 = 3825783610107805990791498543484988350_u128 >> _20.2;
_12 = 3890117011_u32 << _20.2;
_20.0 = !_23;
_6 = 1359380881_i32 + (-1352779727_i32);
_17 = _15 * _11;
_27.fld0.0 = [_7,_7];
Call(_27.fld0.0 = fn4(_20, _27.fld2, _16, _11, _20.0, _27.fld2, _3, _3, _19, _3, _20.2, _20, _7, _20.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_27.fld3.0 = [_11,_17,_17,_11,_17,_11];
_24 = _1 as i16;
_9 = _1 as usize;
_15 = !_11;
_27.fld2 = _19;
_26 = [_20.1,_8,_20.1];
_5 = _24 - _24;
_26 = [_8,_8,_20.1];
_8 = _17 as i128;
_27.fld0.0 = [_7,_7];
_16 = _18;
_22 = _16;
_2 = _16;
_29.1 = _8;
_5 = _24 >> _20.2;
_1 = !false;
_17 = _5 as u16;
_27.fld0.1 = _27.fld0.0;
_2 = _18;
Goto(bb8)
}
bb8 = {
_29.1 = -_20.1;
_23 = !_20.0;
_29.0 = _17 as u64;
_29 = (_20.0, _20.1, _3);
_23 = _7 as u64;
Goto(bb9)
}
bb9 = {
_30 = _14 as f64;
_4 = (-124_i8);
_27.fld0.1 = _27.fld0.0;
_28 = [_7,_7];
Goto(bb10)
}
bb10 = {
_20.1 = _8 >> _29.2;
_24 = _5;
_27.fld1 = [_11,_11,_17,_17,_17];
_9 = _15 as usize;
_20.2 = -_29.2;
Goto(bb11)
}
bb11 = {
_7 = _27.fld2 as i64;
_33 = _6 as f64;
_15 = _14 as u16;
_34 = (_6, _20.1);
_22 = _18;
_31 = [_16];
match _10 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb12,
195 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_30 = _14 as f64;
_4 = (-124_i8);
_27.fld0.1 = _27.fld0.0;
_28 = [_7,_7];
Goto(bb10)
}
bb14 = {
_21 = [_14,_14,_14,_14,_14];
_33 = _19 - _19;
_30 = -_33;
_11 = _17 & _17;
_17 = _12 as u16;
_7 = (-6896293814672293571_i64);
_23 = _6 as u64;
_34 = (_6, _8);
_29.0 = _20.0;
_27.fld2 = _33;
_2 = _22;
_31 = [_16];
_22 = _16;
_3 = _20.2;
_32 = _30;
_1 = !true;
_23 = _1 as u64;
_20.0 = _29.0;
_11 = _15;
_29.1 = _20.1 + _8;
_36 = _34;
_37 = (_36.0, _29.1);
_13 = _12 as u64;
_37 = (_36.0, _29.1);
_33 = _32;
_5 = _24 & _24;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(0_usize, 26_usize, Move(_26), 20_usize, Move(_20), 37_usize, Move(_37), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(0_usize, 5_usize, Move(_5), 4_usize, Move(_4), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(0_usize, 21_usize, Move(_21), 34_usize, Move(_34), 31_usize, Move(_31), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(0_usize, 13_usize, Move(_13), 8_usize, Move(_8), 3_usize, Move(_3), 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: u8,mut _3: (u64, i128, isize),mut _4: (u64, i128, isize),mut _5: (u64, i128, isize),mut _6: u32,mut _7: (u64, i128, isize),mut _8: isize,mut _9: u32) -> usize {
mir! {
type RET = usize;
let _10: f32;
let _11: i8;
let _12: i8;
let _13: bool;
let _14: [u32; 2];
let _15: u16;
let _16: f64;
let _17: isize;
let _18: ([i64; 2], [i64; 2]);
let _19: i128;
let _20: Adt58;
let _21: char;
let _22: ();
let _23: ();
{
_9 = _6;
_4 = (_3.0, _3.1, _7.2);
_2 = !51_u8;
_7.1 = _5.1;
_3.1 = -_5.1;
_9 = _4.2 as u32;
_7 = _4;
_4.1 = _9 as i128;
_1 = _3.2;
_6 = 4459_i16 as u32;
_7 = (_4.0, _4.1, _4.2);
match _4.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
12296166995936337805 => bb7,
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
RET = !4_usize;
_4.0 = 287721248553249199831432557045844100689_u128 as u64;
_5.2 = _7.2;
_3.1 = false as i128;
_3 = (_4.0, _4.1, _7.2);
RET = !5647573151415972695_usize;
_4.1 = _5.1 - _3.1;
RET = 14830249847091894690_usize;
_7 = (_3.0, _4.1, _8);
_4 = _7;
_5.2 = 33835_u16 as isize;
_3.1 = 6742596640366319084_i64 as i128;
_7.1 = 18667_i16 as i128;
_3.2 = _1;
Call(_10 = fn2(_1, _4.1, _5.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = !16009341469895701719_usize;
_2 = 122_u8;
_2 = RET as u8;
_7.1 = false as i128;
_7.1 = _4.1;
_3 = (_7.0, _7.1, _7.2);
_4.1 = _10 as i128;
_3.1 = _2 as i128;
_9 = _6;
_4.0 = (-3827369656397860825_i64) as u64;
_1 = _8;
_3 = (_4.0, _7.1, _4.2);
_2 = 136_u8 | 211_u8;
_3.2 = _4.2;
Goto(bb9)
}
bb9 = {
_3.2 = true as isize;
_6 = (-1398739386_i32) as u32;
_7 = (_5.0, _5.1, _1);
_4.0 = _7.0;
_7 = (_5.0, _4.1, _8);
Goto(bb10)
}
bb10 = {
_6 = 79_i8 as u32;
_7.2 = _8 >> _4.2;
_6 = _7.2 as u32;
_4 = (_5.0, _5.1, _7.2);
_3.0 = _7.0;
_4 = (_5.0, _3.1, _7.2);
_11 = RET as i8;
_7.0 = _3.0 ^ _4.0;
_9 = !_6;
_11 = !10_i8;
_5.1 = '\u{41a67}' as i128;
_4.0 = RET as u64;
_12 = !_11;
_7.2 = _5.2;
_3 = (_4.0, _4.1, _8);
_9 = !_6;
RET = _4.2 as usize;
RET = 4_usize;
RET = 1731137663400837505_usize | 2_usize;
RET = 17260198000544024542_usize;
_4 = _3;
_1 = _5.2;
_5 = (_7.0, _4.1, _8);
_7.1 = _5.0 as i128;
_2 = !150_u8;
_16 = _1 as f64;
match RET {
0 => bb6,
17260198000544024542 => bb11,
_ => bb4
}
}
bb11 = {
_4.1 = _5.1 | _5.1;
_14 = [_6,_6];
_3.1 = -_4.1;
_5.1 = _4.1 - _3.1;
_13 = !false;
_14 = [_9,_6];
_19 = -_5.1;
_7.0 = !_5.0;
_7 = (_5.0, _3.1, _3.2);
_18.1 = [(-2328732026982544567_i64),(-7184836559795096619_i64)];
_5.2 = -_3.2;
_13 = true;
_3 = (_5.0, _19, _4.2);
_5.1 = _3.1 * _19;
_7.1 = _5.1 & _5.1;
match RET {
0 => bb10,
1 => bb2,
17260198000544024542 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_7.2 = _3.2 + _3.2;
_9 = _7.1 as u32;
Call(_5.1 = fn3(_9, _9, _4, _19, _7, _9, _7, _9), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_18.0 = _18.1;
_19 = _3.1;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(1_usize, 1_usize, Move(_1), 3_usize, Move(_3), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(1_usize, 19_usize, Move(_19), 5_usize, Move(_5), 18_usize, Move(_18), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: i128,mut _3: i128) -> f32 {
mir! {
type RET = f32;
let _4: [u128; 5];
let _5: Adt56;
let _6: f64;
let _7: isize;
let _8: *const isize;
let _9: [i64; 2];
let _10: i16;
let _11: f64;
let _12: isize;
let _13: *mut i128;
let _14: *mut &'static u16;
let _15: (u32, u8);
let _16: *mut *mut i16;
let _17: Adt55;
let _18: [bool; 1];
let _19: isize;
let _20: Adt62;
let _21: isize;
let _22: char;
let _23: Adt58;
let _24: Adt60;
let _25: f64;
let _26: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]);
let _27: isize;
let _28: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]);
let _29: f64;
let _30: f32;
let _31: i64;
let _32: u16;
let _33: (u64, i128, isize);
let _34: i8;
let _35: Adt48;
let _36: (([i64; 2], [i64; 2]), f32, (i32, i128));
let _37: f32;
let _38: *const ([u16; 6],);
let _39: (u32, u8);
let _40: f64;
let _41: u8;
let _42: [char; 1];
let _43: f64;
let _44: isize;
let _45: [u16; 5];
let _46: i16;
let _47: ();
let _48: ();
{
RET = 210522159727599099657390589139637806313_u128 as f32;
_4 = [158550142303829522814212836160840778777_u128,137291830465038087653774865921593333856_u128,131164103756568344582502269777367063521_u128,302504887295511475657545022145933246554_u128,93772485105061513150447009005415539764_u128];
_2 = 1846167295208975960038047771168977448_u128 as i128;
_2 = !_3;
_1 = (-9223372036854775808_isize);
RET = _1 as f32;
RET = 4549532562409256453_u64 as f32;
_4 = [191323793988119540115144802146388215816_u128,178219945603039037678786409840629695137_u128,326351241778990546766187169063689091442_u128,56937456770593890170832710819623767538_u128,27629246867831604935692631795445815161_u128];
_2 = _3 & _3;
RET = 17628_i16 as f32;
_6 = 214779980854582657583469228200124198202_u128 as f64;
_7 = -_1;
RET = 3942182101_u32 as f32;
_4 = [220505082199091683482217414747109144522_u128,220646250965825473705326641796580117709_u128,184381874046166289502358681699796256680_u128,3552894617299287016604749909931328419_u128,50618103743253784229735365036119634918_u128];
match _1 {
0 => bb1,
340282366920938463454151235394913435648 => bb3,
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
RET = _1 as f32;
_1 = _7;
_1 = _6 as isize;
_6 = 12055808061267371340_usize as f64;
RET = (-1618082417_i32) as f32;
_2 = _3 << _1;
_7 = '\u{a6cf7}' as isize;
RET = (-55_i8) as f32;
_2 = _3;
RET = 7723_i16 as f32;
_8 = core::ptr::addr_of!(_1);
_7 = (*_8) >> _1;
RET = 47237_u16 as f32;
RET = 10_i8 as f32;
(*_8) = 6709362365923946025_i64 as isize;
(*_8) = _7;
_1 = _2 as isize;
_6 = RET as f64;
RET = _6 as f32;
_10 = (-143078401923738548_i64) as i16;
_8 = core::ptr::addr_of!((*_8));
_3 = 1531517027_i32 as i128;
Goto(bb4)
}
bb4 = {
_1 = _7;
_11 = -_6;
_7 = (*_8);
_10 = (-26550_i16);
_9 = [2809493170151188881_i64,(-3267374436845770569_i64)];
_12 = _1;
_4 = [336657395029844570127649817683917816674_u128,2168217796455665467426850284034241246_u128,41895915382598458734950771141041197765_u128,85429438690468173867633471045382373608_u128,98236407782196017911968621616228074630_u128];
_4 = [186669868282899487243750011675354234782_u128,58039037543700543349342305228801080398_u128,315002854386647184045016551261190202101_u128,221811581494634201147088115404342214627_u128,269283707604093504283279206249277284744_u128];
_9 = [(-7464740080390028126_i64),(-1864342565098641419_i64)];
_13 = core::ptr::addr_of_mut!(_3);
RET = _6 as f32;
_2 = (*_13);
(*_8) = _7 << _2;
_4 = [15198559581974634974871423042414527744_u128,22245706791601382853973146553498300504_u128,236040203005080890279966977510171351745_u128,90118478951519496345297409360124776929_u128,292145736583569635519763183515715982757_u128];
RET = _7 as f32;
_15.1 = 234_u8;
RET = 0_usize as f32;
(*_8) = _11 as isize;
RET = _10 as f32;
_13 = core::ptr::addr_of_mut!(_2);
_9 = [(-7992614583987393944_i64),(-3086981353572991892_i64)];
RET = (*_13) as f32;
match _10 {
0 => bb1,
340282366920938463463374607431768184906 => bb5,
_ => bb3
}
}
bb5 = {
_15.0 = !3818368733_u32;
_1 = _12;
(*_13) = 17_i8 as i128;
_15.1 = 225_u8 & 214_u8;
_15.0 = _2 as u32;
_12 = -(*_8);
_10 = !(-30727_i16);
(*_13) = _3 >> _3;
_15.0 = 734725691_u32 >> _2;
(*_13) = -_3;
_15 = (1409540721_u32, 125_u8);
_18 = [false];
_1 = _7;
_13 = core::ptr::addr_of_mut!((*_13));
_6 = _11 - _11;
_18 = [false];
_18 = [true];
Call(_21 = core::intrinsics::transmute(_7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13 = core::ptr::addr_of_mut!((*_13));
_7 = (*_8) - _1;
RET = 263675924957213631956389136970795673585_u128 as f32;
(*_13) = 13630809070356202943_usize as i128;
_15.1 = _6 as u8;
_19 = _12 >> _1;
_12 = _19;
_4 = [203915474673251388077695813271062959844_u128,137381123774794044433928568697009600064_u128,287699581199771813700943585984695124356_u128,100076435261679201673561957872580845542_u128,68750895685615818648964228449226235023_u128];
_10 = (-32212_i16);
(*_8) = _12 * _12;
(*_8) = 4823475691947239979_usize as isize;
_15.1 = _7 as u8;
_7 = -(*_8);
_13 = core::ptr::addr_of_mut!(_2);
_4 = [201779626985374969899869787010859217597_u128,246797355368811647988509904425855590199_u128,232806664834456891008758294283925641410_u128,144413907205588616093274278103690869483_u128,49806115457984321284396501217523476609_u128];
_23.fld0 = [_15.1];
_15.1 = !203_u8;
Call((*_8) = core::intrinsics::transmute(_19), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_13) = _3;
_10 = 13060_i16;
_23.fld0 = [_15.1];
_4 = [291679532092512541222906920519749855690_u128,134143581656448999333166374214608978696_u128,62890743578917207696923017421298835392_u128,243556997239023806852290000933427675174_u128,265761642882070172618764939959987066073_u128];
_24.fld2.0 = (_9, _9);
_26.3 = _24.fld2.0;
_24.fld0 = _15.0;
_24.fld2.0.0 = _26.3.1;
_24.fld1.fld0 = [_15.1];
_24.fld0 = _15.0 - _15.0;
_26.0 = _24.fld0 % _15.0;
_7 = _19 >> _3;
_11 = _6 * _6;
_10 = (-16074_i16) ^ (-17986_i16);
_24.fld1.fld1 = [(*_13),(*_13),(*_13)];
(*_13) = 2666075074747837044_i64 as i128;
_26.3.0 = [4266394253167160085_i64,(-8479585839296235981_i64)];
_28.3.0 = [6418425460415583592_i64,(-6855873788443508855_i64)];
_27 = _21 + _12;
_25 = _11;
_28.2.1 = (*_13) & _2;
match _15.0 {
0 => bb3,
1 => bb2,
2 => bb8,
1409540721 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
_15.0 = !3818368733_u32;
_1 = _12;
(*_13) = 17_i8 as i128;
_15.1 = 225_u8 & 214_u8;
_15.0 = _2 as u32;
_12 = -(*_8);
_10 = !(-30727_i16);
(*_13) = _3 >> _3;
_15.0 = 734725691_u32 >> _2;
(*_13) = -_3;
_15 = (1409540721_u32, 125_u8);
_18 = [false];
_1 = _7;
_13 = core::ptr::addr_of_mut!((*_13));
_6 = _11 - _11;
_18 = [false];
_18 = [true];
Call(_21 = core::intrinsics::transmute(_7), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_26.5 = [(-5639675360046556309_i64),8068921137606109240_i64];
_24.fld2.2.0 = 1861293411_i32;
_28.2.0 = !16728532147685861530_u64;
(*_8) = _7;
_24.fld2.0.1 = [5304471782163630501_i64,7969964962002178752_i64];
_13 = core::ptr::addr_of_mut!(_33.1);
_24.fld0 = _15.0 * _26.0;
_12 = (*_8);
_9 = _26.5;
_26.2.2 = -_19;
_24.fld2.0 = (_9, _9);
_15 = (_24.fld0, 198_u8);
RET = (-9_i8) as f32;
_21 = 49491_u16 as isize;
match _15.1 {
198 => bb11,
_ => bb5
}
}
bb11 = {
_31 = (-7737597509995772381_i64);
_30 = RET;
_33 = (_28.2.0, _28.2.1, _1);
_28.1 = !_15.0;
_28.5 = [_31,_31];
_28.2 = (_33.0, _33.1, _12);
_3 = (*_13) - (*_13);
_18 = [false];
_28 = (_15.0, _15.0, _33, _26.3, _4, _9);
_26.1 = !_28.0;
_26.2.0 = RET as u64;
_36.0.1 = [_31,_31];
_36.0 = (_24.fld2.0.0, _28.3.1);
_21 = _1 >> _3;
_27 = _12;
_19 = -_33.2;
_28.0 = _26.1 & _28.1;
_28.3.1 = [_31,_31];
_26.3.1 = _24.fld2.0.1;
_26.4 = _4;
_26.3 = (_36.0.0, _36.0.0);
match _15.1 {
198 => bb13,
_ => bb12
}
}
bb12 = {
RET = _1 as f32;
_1 = _7;
_1 = _6 as isize;
_6 = 12055808061267371340_usize as f64;
RET = (-1618082417_i32) as f32;
_2 = _3 << _1;
_7 = '\u{a6cf7}' as isize;
RET = (-55_i8) as f32;
_2 = _3;
RET = 7723_i16 as f32;
_8 = core::ptr::addr_of!(_1);
_7 = (*_8) >> _1;
RET = 47237_u16 as f32;
RET = 10_i8 as f32;
(*_8) = 6709362365923946025_i64 as isize;
(*_8) = _7;
_1 = _2 as isize;
_6 = RET as f64;
RET = _6 as f32;
_10 = (-143078401923738548_i64) as i16;
_8 = core::ptr::addr_of!((*_8));
_3 = 1531517027_i32 as i128;
Goto(bb4)
}
bb13 = {
_23 = Adt58 { fld0: _24.fld1.fld0,fld1: _24.fld1.fld1 };
_39.1 = _15.1;
_24.fld1.fld1 = _23.fld1;
RET = _30;
_37 = -_30;
_26.2.2 = _21 | _7;
_26 = _28;
_29 = -_11;
_26.5 = [_31,_31];
_22 = '\u{e7966}';
match _24.fld2.2.0 {
1861293411 => bb15,
_ => bb14
}
}
bb14 = {
_15.0 = !3818368733_u32;
_1 = _12;
(*_13) = 17_i8 as i128;
_15.1 = 225_u8 & 214_u8;
_15.0 = _2 as u32;
_12 = -(*_8);
_10 = !(-30727_i16);
(*_13) = _3 >> _3;
_15.0 = 734725691_u32 >> _2;
(*_13) = -_3;
_15 = (1409540721_u32, 125_u8);
_18 = [false];
_1 = _7;
_13 = core::ptr::addr_of_mut!((*_13));
_6 = _11 - _11;
_18 = [false];
_18 = [true];
Call(_21 = core::intrinsics::transmute(_7), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_12 = _21 | _1;
_24.fld2.1 = _39.1 as f32;
_2 = _28.2.1 + _3;
_37 = _15.1 as f32;
_32 = 40845_u16 * 3656_u16;
_24.fld2.0 = (_26.5, _36.0.0);
_36.2.0 = _24.fld2.2.0;
_7 = _19;
_28.2.2 = !_12;
_34 = 69_i8 | 117_i8;
_24.fld0 = !_26.0;
_28.1 = _26.1;
_26.3.1 = [_31,_31];
_22 = '\u{80df7}';
(*_8) = _26.2.2;
_24.fld2.0 = (_26.3.0, _36.0.0);
_23.fld0 = [_39.1];
_33.2 = _28.2.2 << _21;
_32 = !43989_u16;
_24.fld2.2 = (_36.2.0, (*_13));
_36 = (_26.3, RET, _24.fld2.2);
_39 = (_26.0, _15.1);
_28.2 = (_26.2.0, _2, _33.2);
_7 = -(*_8);
_36 = _24.fld2;
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(2_usize, 27_usize, Move(_27), 12_usize, Move(_12), 22_usize, Move(_22), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(2_usize, 26_usize, Move(_26), 33_usize, Move(_33), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(2_usize, 34_usize, Move(_34), 2_usize, Move(_2), 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u32,mut _2: u32,mut _3: (u64, i128, isize),mut _4: i128,mut _5: (u64, i128, isize),mut _6: u32,mut _7: (u64, i128, isize),mut _8: u32) -> i128 {
mir! {
type RET = i128;
let _9: i64;
let _10: *mut &'static u16;
let _11: (u32, u8);
let _12: ();
let _13: ();
{
_1 = '\u{bef75}' as u32;
_2 = _6 ^ _6;
_3.0 = _7.0 & _7.0;
_8 = '\u{1b75}' as u32;
RET = _7.1 - _5.1;
RET = !_4;
_7.2 = !_5.2;
_7.2 = '\u{1c53f}' as isize;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(3_usize, 3_usize, Move(_3), 4_usize, Move(_4), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (u64, i128, isize),mut _2: f64,mut _3: char,mut _4: u16,mut _5: u64,mut _6: f64,mut _7: isize,mut _8: isize,mut _9: f64,mut _10: isize,mut _11: isize,mut _12: (u64, i128, isize),mut _13: i64,mut _14: isize) -> [i64; 2] {
mir! {
type RET = [i64; 2];
let _15: f32;
let _16: [u16; 5];
let _17: *const isize;
let _18: usize;
let _19: [u128; 5];
let _20: *mut [i128; 3];
let _21: isize;
let _22: [u16; 5];
let _23: isize;
let _24: Adt54;
let _25: isize;
let _26: f64;
let _27: i16;
let _28: [u16; 6];
let _29: u64;
let _30: i8;
let _31: [i64; 2];
let _32: isize;
let _33: [u32; 2];
let _34: f32;
let _35: Adt62;
let _36: Adt53;
let _37: (u64, i128, isize);
let _38: [u8; 5];
let _39: bool;
let _40: [u32; 2];
let _41: *const isize;
let _42: char;
let _43: ();
let _44: ();
{
_15 = (-260174270_i32) as f32;
_9 = _2 * _2;
_11 = _7 | _10;
_4 = !858_u16;
RET = [_13,_13];
_12.2 = (-601872345_i32) as isize;
_10 = _14;
Call(_9 = fn5(_1, _8, _8, _1.2, _7, _1, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !_1.0;
_12.1 = !_1.1;
_11 = _8;
_5 = !_12.0;
_12.2 = _11;
_1.0 = !_5;
_12.0 = _12.1 as u64;
_17 = core::ptr::addr_of!(_1.2);
_16 = [_4,_4,_4,_4,_4];
_12.1 = -_1.1;
_6 = -_2;
_8 = 127_u8 as isize;
_12.0 = _5;
Call(_12.1 = fn6(_17, (*_17), _17, _14, _11, _1.2, _2, _1.2, _12.2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = (_5, _12.1, _14);
_1 = (_5, _12.1, _12.2);
_9 = _15 as f64;
_3 = '\u{2deda}';
_5 = _1.1 as u64;
_5 = _12.0 & _1.0;
_3 = '\u{51713}';
_14 = _11;
RET = [_13,_13];
_12.1 = _13 as i128;
_19 = [132249969412739934933165861141086995485_u128,30630751341353200233891346040562106209_u128,283681553194924746395460563435734841147_u128,199956016212501604825699729009801712340_u128,242664234222019515317090511062402436948_u128];
(*_17) = -_12.2;
(*_17) = _14 * _12.2;
_1 = (_5, _12.1, _11);
_7 = -_10;
_1.0 = !_5;
_12 = (_5, _1.1, _10);
_11 = -(*_17);
_1.2 = _7;
_11 = 55362589_i32 as isize;
_2 = -_6;
Call((*_17) = core::intrinsics::bswap(_12.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = (*_17);
_18 = 2_usize << _5;
_15 = 210_u8 as f32;
_18 = !4_usize;
_11 = 882984134_i32 as isize;
_6 = _12.1 as f64;
Call(_16 = fn8(_12.0, (*_17), _1.2, _1.0, (*_17), _5, _2, _14, _1.2, _2, _10, _12.0, _1, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = _1.0 - _1.0;
_25 = -(*_17);
_17 = core::ptr::addr_of!(_14);
_8 = -_10;
_4 = !41384_u16;
_15 = 24240758846156189555864373935487201972_u128 as f32;
_21 = _12.1 as isize;
_19 = [331631115501156690812888156155896311178_u128,252093779167661818759286943307118133819_u128,322268142397151962500496647180344823195_u128,64801035546243322948275922874031787227_u128,296950569454246069386398957547794459647_u128];
_23 = _25 * (*_17);
_27 = 5001_i16;
_12.0 = !_5;
Goto(bb5)
}
bb5 = {
_14 = _10 - _7;
RET = [_13,_13];
_16 = [_4,_4,_4,_4,_4];
Call(_1.1 = fn13(_17, (*_17), _1.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = '\u{3ebfe}';
_25 = _15 as isize;
RET = [_13,_13];
_5 = !_1.0;
_12.0 = _12.1 as u64;
_12 = _1;
_12 = (_1.0, _1.1, _14);
_1.2 = (*_17);
_12.0 = _12.1 as u64;
_27 = -(-24420_i16);
_25 = -_14;
_23 = -_12.2;
_34 = _15 * _15;
_21 = _25 << _14;
_6 = _2;
_27 = 7379_i16;
match _27 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
7379 => bb13,
_ => bb12
}
}
bb7 = {
_14 = _10 - _7;
RET = [_13,_13];
_16 = [_4,_4,_4,_4,_4];
Call(_1.1 = fn13(_17, (*_17), _1.0), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_5 = _1.0 - _1.0;
_25 = -(*_17);
_17 = core::ptr::addr_of!(_14);
_8 = -_10;
_4 = !41384_u16;
_15 = 24240758846156189555864373935487201972_u128 as f32;
_21 = _12.1 as isize;
_19 = [331631115501156690812888156155896311178_u128,252093779167661818759286943307118133819_u128,322268142397151962500496647180344823195_u128,64801035546243322948275922874031787227_u128,296950569454246069386398957547794459647_u128];
_23 = _25 * (*_17);
_27 = 5001_i16;
_12.0 = !_5;
Goto(bb5)
}
bb9 = {
_11 = (*_17);
_18 = 2_usize << _5;
_15 = 210_u8 as f32;
_18 = !4_usize;
_11 = 882984134_i32 as isize;
_6 = _12.1 as f64;
Call(_16 = fn8(_12.0, (*_17), _1.2, _1.0, (*_17), _5, _2, _14, _1.2, _2, _10, _12.0, _1, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_1 = (_5, _12.1, _14);
_1 = (_5, _12.1, _12.2);
_9 = _15 as f64;
_3 = '\u{2deda}';
_5 = _1.1 as u64;
_5 = _12.0 & _1.0;
_3 = '\u{51713}';
_14 = _11;
RET = [_13,_13];
_12.1 = _13 as i128;
_19 = [132249969412739934933165861141086995485_u128,30630751341353200233891346040562106209_u128,283681553194924746395460563435734841147_u128,199956016212501604825699729009801712340_u128,242664234222019515317090511062402436948_u128];
(*_17) = -_12.2;
(*_17) = _14 * _12.2;
_1 = (_5, _12.1, _11);
_7 = -_10;
_1.0 = !_5;
_12 = (_5, _1.1, _10);
_11 = -(*_17);
_1.2 = _7;
_11 = 55362589_i32 as isize;
_2 = -_6;
Call((*_17) = core::intrinsics::bswap(_12.2), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_5 = !_1.0;
_12.1 = !_1.1;
_11 = _8;
_5 = !_12.0;
_12.2 = _11;
_1.0 = !_5;
_12.0 = _12.1 as u64;
_17 = core::ptr::addr_of!(_1.2);
_16 = [_4,_4,_4,_4,_4];
_12.1 = -_1.1;
_6 = -_2;
_8 = 127_u8 as isize;
_12.0 = _5;
Call(_12.1 = fn6(_17, (*_17), _17, _14, _11, _1.2, _2, _1.2, _12.2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_19 = [70632597329954001965517989559079715435_u128,267081479749143977374933383106085950660_u128,336515081682356839681017601490762029813_u128,51264521206020801864092176039816398463_u128,31018350424805910017784237466362117768_u128];
_31 = RET;
_1.1 = _12.1 & _12.1;
_9 = _2 * _2;
_9 = -_6;
_18 = 6_usize | 3265024787130137075_usize;
_15 = -_34;
_18 = 0_usize;
_21 = (-31_i8) as isize;
_16[_18] = _4;
RET[_18] = _31[_18] >> _14;
_40 = [2510609630_u32,2460878238_u32];
_10 = -_1.2;
_32 = _16[_18] as isize;
_32 = !_1.2;
_4 = !_16[_18];
_12 = (_1.0, _1.1, _32);
_31[_18] = RET[_18] & _13;
_6 = _1.1 as f64;
_12.1 = _1.2 as i128;
(*_17) = -_12.2;
_40[_18] = 1057304061_u32 << _10;
_10 = -_1.2;
Goto(bb14)
}
bb14 = {
Call(_43 = dump_var(4_usize, 23_usize, Move(_23), 1_usize, Move(_1), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_43 = dump_var(4_usize, 13_usize, Move(_13), 7_usize, Move(_7), 32_usize, Move(_32), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(4_usize, 27_usize, Move(_27), 21_usize, Move(_21), 44_usize, _44, 44_usize, _44), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (u64, i128, isize),mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: (u64, i128, isize),mut _7: isize) -> f64 {
mir! {
type RET = f64;
let _8: (([i64; 2], [i64; 2]), f32, (i32, i128));
let _9: f32;
let _10: [char; 1];
let _11: Adt46;
let _12: u8;
let _13: i16;
let _14: *mut *mut i16;
let _15: ();
let _16: ();
{
RET = 2948489565016902340_i64 as f64;
_8.2.1 = _1.1;
_6.0 = !_1.0;
_1.2 = 100_i8 as isize;
_8.0.0 = [1720227263765100758_i64,6648805110330207448_i64];
_8.0.1 = [(-318385285119068263_i64),2157702671708534289_i64];
_2 = -_6.2;
_6.2 = _5;
_8.1 = 8361868828433361989_usize as f32;
_9 = -_8.1;
_4 = _3 & _6.2;
_8.2 = ((-1443034349_i32), _6.1);
Call(_4 = core::intrinsics::transmute(_1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.1 = '\u{fc6f3}' as i128;
_6.1 = _4 as i128;
_11.fld1.0 = _8.2.0 as u32;
_5 = !_2;
match _8.2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607430325177107 => bb10,
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
_6 = (_1.0, _1.1, _4);
_5 = -_4;
_11.fld2 = 28497_i16 as usize;
_11.fld1.1 = _11.fld1.0 >> _1.0;
_1.2 = _3 * _7;
_12 = 217_u8 - 108_u8;
_8.2 = (1709085402_i32, _1.1);
_11.fld4 = [_8.2.1,_8.2.1,_8.2.1];
match _6.1 {
0 => bb5,
1 => bb7,
2 => bb11,
3 => bb12,
304664823352592670591405976228207404020 => bb14,
_ => bb13
}
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
_11.fld1.4 = [130614311946054554399263876344864772392_u128,87941424925551604097360009748824361377_u128,123735555796328556191014462378843148438_u128,332505802436834846569293397439973104438_u128,140595909874466845453923022099534108751_u128];
_10 = ['\u{c858b}'];
_11.fld1.3 = (_8.0.1, _8.0.1);
_11.fld1.3.0 = [(-4274549488645805503_i64),1560163210829272892_i64];
_11.fld3 = (_8.2.1, _6.0, _8.2.0);
_13 = -(-4738_i16);
_11.fld0 = _1.0 & _11.fld3.1;
RET = _9 as f64;
_9 = -_8.1;
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(5_usize, 1_usize, Move(_1), 6_usize, Move(_6), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_15 = dump_var(5_usize, 13_usize, Move(_13), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *const isize,mut _2: isize,mut _3: *const isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: f64,mut _8: isize,mut _9: isize,mut _10: f64) -> i128 {
mir! {
type RET = i128;
let _11: (&'static u16,);
let _12: f64;
let _13: ([u16; 6],);
let _14: (u32, u8);
let _15: ([u16; 6],);
let _16: &'static u16;
let _17: [i64; 2];
let _18: *mut [i128; 3];
let _19: f32;
let _20: [u32; 2];
let _21: Adt55;
let _22: i64;
let _23: u64;
let _24: *mut i16;
let _25: *mut [i128; 3];
let _26: *mut &'static u16;
let _27: f32;
let _28: Adt54;
let _29: ([u16; 6],);
let _30: *mut *mut i16;
let _31: [char; 1];
let _32: [i128; 3];
let _33: i16;
let _34: [u16; 6];
let _35: Adt56;
let _36: *const isize;
let _37: Adt58;
let _38: [char; 1];
let _39: [i64; 2];
let _40: ();
let _41: ();
{
_7 = -_10;
RET = 121606055840782200505630637237778597118_i128;
(*_3) = _4 << _5;
RET = 98636862683088426015744872238505625482_i128 | 98358906507083909576265433399398808469_i128;
_7 = _10 - _10;
_3 = core::ptr::addr_of!((*_3));
_12 = -_7;
_8 = !(*_1);
RET = (-128756262599043704783181540416182373529_i128);
_5 = _4 << (*_1);
_14.0 = (*_3) as u32;
Goto(bb1)
}
bb1 = {
_5 = 14348_i16 as isize;
(*_3) = (-276076352_i32) as isize;
_5 = _4;
_9 = (-3070_i16) as isize;
_2 = !_8;
_8 = _12 as isize;
_9 = !_2;
_14.0 = RET as u32;
_7 = _12 + _10;
_12 = _7;
_14.1 = 66_u8;
_10 = _14.1 as f64;
_7 = _12 * _12;
_14.0 = 2056392031_u32 & 78146661_u32;
_9 = _8;
_5 = -_9;
_9 = 12575_u16 as isize;
_12 = _7;
Goto(bb2)
}
bb2 = {
_14.1 = 797678825_i32 as u8;
_15.0 = [59687_u16,30976_u16,63593_u16,15896_u16,12900_u16,38515_u16];
_14 = (1130542620_u32, 242_u8);
_14.1 = !184_u8;
_13 = _15;
(*_1) = _6 + _6;
_8 = _5;
_15.0 = [20889_u16,12276_u16,46873_u16,22268_u16,17685_u16,18214_u16];
RET = _14.0 as i128;
_5 = -(*_1);
_10 = -_12;
RET = 112563764925439639568109636555397605903_i128;
_13.0 = _15.0;
_4 = 9934_u16 as isize;
(*_1) = _8;
_13 = (_15.0,);
_2 = (*_3);
_15 = (_13.0,);
(*_1) = _2;
(*_1) = -_8;
_14.1 = !131_u8;
_6 = (*_3);
_2 = (*_3) + (*_3);
RET = 135181317050676869494944762722258975536_i128;
match _14.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1130542620 => bb8,
_ => bb7
}
}
bb3 = {
_5 = 14348_i16 as isize;
(*_3) = (-276076352_i32) as isize;
_5 = _4;
_9 = (-3070_i16) as isize;
_2 = !_8;
_8 = _12 as isize;
_9 = !_2;
_14.0 = RET as u32;
_7 = _12 + _10;
_12 = _7;
_14.1 = 66_u8;
_10 = _14.1 as f64;
_7 = _12 * _12;
_14.0 = 2056392031_u32 & 78146661_u32;
_9 = _8;
_5 = -_9;
_9 = 12575_u16 as isize;
_12 = _7;
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
_9 = _2;
(*_3) = !_9;
_10 = 87686907078599828748052964121211582371_u128 as f64;
_4 = (-364517036_i32) as isize;
_3 = core::ptr::addr_of!(_8);
_17 = [(-85539123033560038_i64),(-2458154221153627803_i64)];
_15 = _13;
_14.1 = 139_u8 << _6;
(*_3) = (-297667126_i32) as isize;
(*_1) = _2 * _9;
(*_1) = 13680590158280825148_u64 as isize;
_10 = _7 + _7;
_12 = -_7;
_15.0 = [63331_u16,20999_u16,15013_u16,28103_u16,18041_u16,12212_u16];
_19 = 23801_u16 as f32;
_1 = _3;
_4 = _6 - _6;
_19 = (-29053_i16) as f32;
_19 = (-6393162733518684065_i64) as f32;
_1 = core::ptr::addr_of!((*_3));
RET = (-4115655228623145987_i64) as i128;
_7 = _12;
(*_3) = _2 + _4;
_14 = (1520227930_u32, 37_u8);
_3 = _1;
Call(_5 = fn7((*_3), (*_1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_3) = _6 + _2;
_13 = (_15.0,);
_7 = _12;
_13.0 = [37703_u16,17718_u16,4922_u16,545_u16,63781_u16,20304_u16];
_13 = (_15.0,);
_20 = [_14.0,_14.0];
RET = -126430588360497125081117455236221839473_i128;
_4 = (*_1);
_7 = 11500052074398535360_u64 as f64;
Call(_15.0 = core::intrinsics::transmute(_13.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = _3;
_13 = (_15.0,);
_12 = _10;
match _14.0 {
0 => bb7,
1520227930 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
RET = 97481508323319807277545986271523655130_i128 + 5801144076267567334115708960934642161_i128;
_13 = (_15.0,);
(*_1) = _5;
_13.0 = [61044_u16,32904_u16,37581_u16,62861_u16,48337_u16,6779_u16];
_14.0 = 2477296931_u32;
_22 = (-9208502119656151413_i64);
_15.0 = [2247_u16,63829_u16,31549_u16,1378_u16,64639_u16,61866_u16];
_14.0 = 3739367929_u32 >> (*_3);
_22 = -(-3174982136555730238_i64);
_6 = _9 >> _5;
(*_1) = _2 | _6;
_20 = [_14.0,_14.0];
RET = (-22133647740346756571932338873555776353_i128);
_26 = core::ptr::addr_of_mut!(_11.0);
_23 = 2267106472252844156_u64;
(*_1) = !_5;
_20 = [_14.0,_14.0];
_19 = _23 as f32;
match _14.1 {
0 => bb5,
37 => bb13,
_ => bb4
}
}
bb13 = {
_8 = -_9;
_12 = _10;
_13 = _15;
_32 = [RET,RET,RET];
_17 = [_22,_22];
_23 = 13960034497042595302_u64 & 14002726965723014310_u64;
(*_1) = 6_usize as isize;
_29.0 = [45779_u16,47646_u16,36012_u16,18537_u16,2501_u16,25247_u16];
_27 = _9 as f32;
_30 = core::ptr::addr_of_mut!(_24);
Goto(bb14)
}
bb14 = {
_14 = (269318575_u32, 117_u8);
_33 = (-7823_i16) - (-6567_i16);
_15.0 = [20384_u16,63994_u16,16003_u16,48770_u16,18452_u16,52941_u16];
_34 = [11706_u16,41348_u16,52644_u16,38261_u16,9328_u16,61922_u16];
RET = !(-69212018497456414418245127481059197366_i128);
RET = !158001752868904272465840950863275764523_i128;
_7 = _10;
RET = 38090522689085820236281755699512719842_i128;
_31 = ['\u{a26f3}'];
_4 = !_2;
_3 = core::ptr::addr_of!(_8);
_14 = (581955565_u32, 199_u8);
_6 = -_2;
(*_3) = _5;
_13.0 = _34;
_17 = [_22,_22];
_24 = core::ptr::addr_of_mut!(_33);
_8 = (*_24) as isize;
_20 = [_14.0,_14.0];
_32 = [RET,RET,RET];
_18 = core::ptr::addr_of_mut!(_32);
_25 = _18;
(*_25) = [RET,RET,RET];
_38 = ['\u{a37c}'];
_13.0 = [9569_u16,58058_u16,47287_u16,21227_u16,8781_u16,10902_u16];
_14.0 = _7 as u32;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(6_usize, 38_usize, Move(_38), 13_usize, Move(_13), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(6_usize, 20_usize, Move(_20), 32_usize, Move(_32), 2_usize, Move(_2), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(6_usize, 9_usize, Move(_9), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: i8;
let _4: f32;
let _5: Adt54;
let _6: i8;
let _7: ();
let _8: ();
{
RET = _2 + _2;
RET = _2;
RET = -_2;
RET = _2 * _1;
RET = _2 & _2;
_1 = RET;
_1 = RET ^ _2;
_4 = (-17602_i16) as f32;
_3 = 56549_u16 as i8;
_3 = -84_i8;
RET = _4 as isize;
_1 = 1692852674_i32 as isize;
RET = _2;
_4 = 30631_u16 as f32;
_4 = (-969_i16) as f32;
_1 = -_2;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(7_usize, 2_usize, Move(_2), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u64,mut _2: isize,mut _3: isize,mut _4: u64,mut _5: isize,mut _6: u64,mut _7: f64,mut _8: isize,mut _9: isize,mut _10: f64,mut _11: isize,mut _12: u64,mut _13: (u64, i128, isize),mut _14: u64) -> [u16; 5] {
mir! {
type RET = [u16; 5];
let _15: bool;
let _16: *mut &'static u16;
let _17: *mut &'static u16;
let _18: Adt61;
let _19: u32;
let _20: *mut i16;
let _21: f32;
let _22: u16;
let _23: bool;
let _24: [u128; 5];
let _25: bool;
let _26: char;
let _27: u8;
let _28: u32;
let _29: bool;
let _30: *mut &'static u16;
let _31: char;
let _32: ();
let _33: ();
{
_13 = (_6, 11673116979163782653368335763645419046_i128, _2);
_14 = _12 << _6;
_11 = _3;
_9 = _11 + _11;
_3 = -_2;
_11 = _5;
RET = [39433_u16,19730_u16,21066_u16,65154_u16,13170_u16];
match _13.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
11673116979163782653368335763645419046 => bb6,
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
_14 = !_13.0;
_13.0 = !_4;
_10 = -_7;
_12 = _13.0;
_13 = (_6, (-60564817378733068945819511003297686949_i128), _8);
RET = [10170_u16,23027_u16,47175_u16,6577_u16,9823_u16];
_5 = _3 + _9;
_13.0 = _1;
_7 = _10 + _10;
_15 = !true;
RET = [43957_u16,24174_u16,58677_u16,8877_u16,58816_u16];
_1 = _12;
_7 = -_10;
_5 = _3 * _2;
_8 = -_11;
_9 = _15 as isize;
RET = [18346_u16,22418_u16,13416_u16,5150_u16,61008_u16];
_13.0 = !_4;
_6 = (-32611_i16) as u64;
_13.1 = !40415562939092965722117992858297379605_i128;
_10 = _7 - _7;
Call(_6 = fn9(_2, _13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _8;
_14 = _6;
_13.2 = _8;
_1 = !_13.0;
_12 = _4 << _13.2;
_1 = _12;
_11 = 4321318849421606007_usize as isize;
RET = [57512_u16,51633_u16,53629_u16,10617_u16,28067_u16];
_12 = !_6;
_3 = _8;
_13 = (_6, 74554003143875424355177508102132958986_i128, _3);
_10 = -_7;
_10 = 174609487945672576604891559331359290759_u128 as f64;
RET = [29638_u16,37094_u16,3648_u16,59165_u16,54074_u16];
_10 = _7 * _7;
_15 = !false;
RET = [10276_u16,14469_u16,16934_u16,247_u16,28_u16];
RET = [63673_u16,26539_u16,21531_u16,27768_u16,41335_u16];
RET = [20084_u16,35612_u16,56073_u16,29863_u16,38363_u16];
_14 = _1 * _1;
_4 = _7 as u64;
match _13.1 {
0 => bb1,
1 => bb4,
2 => bb6,
74554003143875424355177508102132958986 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_13.2 = -_5;
_13.2 = 104_i8 as isize;
_5 = 15399_i16 as isize;
_8 = _2;
_8 = _2 << _14;
_18.fld1 = _10;
_13 = (_14, 143511068740219758073785998600078563788_i128, _3);
_1 = !_14;
_18.fld2 = ['\u{f7ac4}'];
_5 = -_3;
_12 = _1 ^ _6;
_18.fld0 = core::ptr::addr_of_mut!(_18.fld6);
_3 = !_8;
_11 = 62300_u16 as isize;
_18.fld5 = 1492572907_i32;
_18.fld4 = RET;
_10 = -_18.fld1;
_14 = _8 as u64;
_11 = _8;
_18.fld2 = ['\u{e6ab9}'];
Goto(bb10)
}
bb10 = {
_5 = _11 - _3;
_22 = 53185_u16;
_7 = _10 * _10;
_6 = _13.0 ^ _14;
Call(_11 = fn12(_7, _8, _6, _12, _6, _12, _13.1, _3, _3, _14, _6, _12, _13, _13.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = 1617218582_u32 as isize;
_18.fld5 = (-9218_i16) as i32;
_13 = (_6, 1584898251092309965378468091809539251_i128, _8);
_10 = _7 - _7;
_23 = _7 != _7;
_6 = _12 - _14;
_11 = _5 << _14;
_6 = _13.0 >> _13.0;
_5 = _11 + _8;
_11 = _5;
Call(_13.0 = core::intrinsics::transmute(_14), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_13.0 = _1 ^ _6;
_6 = _22 as u64;
_22 = 31994_u16 | 2088_u16;
RET = [_22,_22,_22,_22,_22];
_23 = _5 != _3;
_18.fld1 = _7 * _7;
_8 = _3;
_13.0 = _1;
_18.fld2 = ['\u{9e0a4}'];
_3 = _13.2;
_8 = _3 | _13.2;
Goto(bb13)
}
bb13 = {
_8 = _11;
RET = _18.fld4;
RET = [_22,_22,_22,_22,_22];
_18.fld2 = ['\u{aff}'];
_21 = _10 as f32;
_18.fld5 = '\u{544e8}' as i32;
_5 = -_3;
match _13.1 {
0 => bb12,
1 => bb10,
2 => bb11,
3 => bb6,
4 => bb14,
1584898251092309965378468091809539251 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_11 = 1617218582_u32 as isize;
_18.fld5 = (-9218_i16) as i32;
_13 = (_6, 1584898251092309965378468091809539251_i128, _8);
_10 = _7 - _7;
_23 = _7 != _7;
_6 = _12 - _14;
_11 = _5 << _14;
_6 = _13.0 >> _13.0;
_5 = _11 + _8;
_11 = _5;
Call(_13.0 = core::intrinsics::transmute(_14), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
_18.fld2 = ['\u{84757}'];
_2 = _13.1 as isize;
_15 = _23;
_14 = _23 as u64;
_19 = 3424893970_u32;
_4 = _18.fld5 as u64;
_15 = !_23;
_15 = _23 >= _23;
_2 = _5 - _8;
_25 = _15;
_18.fld0 = core::ptr::addr_of_mut!(_20);
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(8_usize, 11_usize, Move(_11), 13_usize, Move(_13), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(8_usize, 15_usize, Move(_15), 5_usize, Move(_5), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: (u64, i128, isize)) -> u64 {
mir! {
type RET = u64;
let _3: (u64, i128, isize);
let _4: Adt59;
let _5: [u16; 6];
let _6: i8;
let _7: char;
let _8: i16;
let _9: bool;
let _10: (i128, u64, i32);
let _11: bool;
let _12: isize;
let _13: Adt49;
let _14: Adt51;
let _15: usize;
let _16: (i64, i64);
let _17: f32;
let _18: isize;
let _19: u32;
let _20: Adt62;
let _21: i64;
let _22: Adt52;
let _23: ();
let _24: ();
{
RET = _2.0;
_1 = _2.2;
_3.1 = _2.1 & _2.1;
_3.2 = _1;
_3.0 = _2.0 ^ _2.0;
_3 = _2;
RET = (-599295317_i32) as u64;
_3.0 = _2.0 ^ _2.0;
_3.0 = _2.1 as u64;
_2.1 = _3.1 - _3.1;
_2.0 = _3.0;
_3.0 = _2.0;
_3.1 = _2.1;
_3.1 = (-3230745908649975708_i64) as i128;
Call(_2.1 = core::intrinsics::transmute(_3.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
Goto(bb2)
}
bb2 = {
_6 = -56_i8;
RET = _2.0;
_3.0 = !_2.0;
_1 = _3.2 * _2.2;
Call(_5 = fn10(_3, _3, _1, _3, _1, _2.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = '\u{24335}';
_3.0 = RET;
_3.0 = RET << RET;
_2.2 = _3.2;
_3.0 = 163873956340929944222545981755747901201_u128 as u64;
_8 = !21642_i16;
_7 = '\u{5efc8}';
_3.0 = RET;
_7 = '\u{c08e5}';
_1 = -_3.2;
_2.0 = RET - RET;
_3.2 = _2.2;
_2.0 = !_3.0;
_9 = !true;
_3.2 = _9 as isize;
_3.0 = !RET;
_6 = !98_i8;
_8 = 13349_i16 | 4183_i16;
_9 = _3.0 == _2.0;
_3.0 = _2.0 >> _2.0;
_7 = '\u{d39f2}';
RET = _3.0 & _2.0;
_10.2 = 1730475763_i32;
_10 = (_3.1, _3.0, 1042387000_i32);
Call(_10.0 = fn11(_3.0, _1, _2, _2.2, _1, _2.2, _9, _5, _1, _1, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10.1 = RET ^ RET;
_1 = _9 as isize;
_11 = !_9;
_13.fld1 = [27767_u16,1085_u16,19756_u16,24751_u16,24890_u16];
_2.2 = !_1;
_3.0 = 3230038876_u32 as u64;
_2.0 = _10.1 ^ RET;
RET = !_2.0;
_2.2 = _3.2 ^ _3.2;
_13.fld0.1 = [(-7535166391667809860_i64),3276754833242884453_i64];
_13.fld0.0 = [(-265071522709032731_i64),2036336950226240980_i64];
_13.fld3 = (_5,);
_10.2 = (-1537582990_i32) * 550353213_i32;
_16.1 = _7 as i64;
_2.2 = -_1;
Call(_15 = core::intrinsics::bswap(3_usize), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13.fld3 = (_5,);
_5 = _13.fld3.0;
_2.1 = !_10.0;
_2.2 = _2.0 as isize;
_13.fld0.1 = _13.fld0.0;
_10.1 = _6 as u64;
_3 = _2;
_3.1 = _10.0 | _10.0;
_13.fld2 = _3.2 as f64;
_2.2 = !_3.2;
_1 = _3.2 ^ _2.2;
_16.0 = _16.1;
_16.1 = -_16.0;
_2.2 = _1 & _3.2;
_3.0 = RET >> _10.0;
_12 = -_2.2;
_9 = _2.0 != RET;
_17 = _16.0 as f32;
_16.0 = _16.1 * _16.1;
_2 = (_3.0, _3.1, _3.2);
_13.fld1 = [56493_u16,52795_u16,59389_u16,39834_u16,47553_u16];
Goto(bb6)
}
bb6 = {
_7 = '\u{4a9cb}';
_3.2 = _1;
_2.2 = _1;
_6 = 36_i8;
_12 = _3.2 + _3.2;
_10.1 = _2.0;
_12 = _2.2;
_3.2 = _13.fld2 as isize;
_1 = _3.0 as isize;
_10.2 = 1732144604_i32;
_13.fld0.0 = [_16.1,_16.0];
_1 = _2.2;
_2.0 = _3.0;
_15 = _17 as usize;
_9 = !_11;
Goto(bb7)
}
bb7 = {
_13.fld3 = (_5,);
_6 = (-25_i8) | 8_i8;
_13.fld0.1 = [_16.1,_16.1];
_18 = !_1;
_6 = -(-44_i8);
RET = !_10.1;
_5 = _13.fld3.0;
_3.0 = 45202185855443604256542734291301282116_u128 as u64;
_6 = 107_i8 | (-58_i8);
_3.0 = _10.1 & RET;
_3 = (RET, _2.1, _12);
_3.0 = RET;
_12 = !_1;
_16 = ((-9135139131137436551_i64), 3391416621919924482_i64);
RET = !_10.1;
_13.fld0.0 = _13.fld0.1;
_11 = _12 != _18;
_16 = ((-958354310783746997_i64), (-6728782748679089007_i64));
_2.1 = !_10.0;
_2 = _3;
_19 = 3719432308_u32 | 2182042472_u32;
_3.0 = _2.0;
_3.1 = _10.1 as i128;
_13.fld3.0 = [38652_u16,8268_u16,22309_u16,18719_u16,33914_u16,53603_u16];
_3.1 = !_2.1;
Goto(bb8)
}
bb8 = {
Call(_23 = dump_var(9_usize, 19_usize, Move(_19), 11_usize, Move(_11), 15_usize, Move(_15), 9_usize, Move(_9)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_23 = dump_var(9_usize, 5_usize, Move(_5), 8_usize, Move(_8), 16_usize, Move(_16), 24_usize, _24), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (u64, i128, isize),mut _2: (u64, i128, isize),mut _3: isize,mut _4: (u64, i128, isize),mut _5: isize,mut _6: isize) -> [u16; 6] {
mir! {
type RET = [u16; 6];
let _7: (i32, i128);
let _8: i64;
let _9: [bool; 1];
let _10: [u32; 2];
let _11: [u128; 5];
let _12: Adt49;
let _13: i8;
let _14: f64;
let _15: (i64, i64);
let _16: (i64, i64);
let _17: Adt62;
let _18: [u8; 1];
let _19: [u8; 1];
let _20: i32;
let _21: isize;
let _22: f64;
let _23: char;
let _24: bool;
let _25: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]);
let _26: (i32, i128);
let _27: i64;
let _28: *mut i16;
let _29: ();
let _30: ();
{
_2.1 = 5899_i16 as i128;
_2.0 = !_4.0;
RET = [27434_u16,8271_u16,8339_u16,8333_u16,47622_u16,62661_u16];
_1 = (_4.0, _4.1, _5);
_1 = (_4.0, _4.1, _6);
_2.2 = _6;
_5 = _1.2 ^ _6;
_1.1 = !_2.1;
_7.1 = _2.1 & _4.1;
_1 = (_4.0, _4.1, _2.2);
_1.0 = _2.0;
_1.0 = true as u64;
RET = [51950_u16,42041_u16,24126_u16,49509_u16,17234_u16,21536_u16];
_3 = _6;
_7 = ((-1100976063_i32), _1.1);
_1.0 = _2.0 | _2.0;
RET = [44392_u16,825_u16,52185_u16,39711_u16,52961_u16,53480_u16];
_7.0 = 1719088514_i32;
_2.0 = !_4.0;
_7.0 = _2.1 as i32;
_8 = (-1598711269024170618_i64) + (-1319793742025349506_i64);
_1.2 = -_4.2;
_4.0 = _2.0 ^ _2.0;
_2 = (_4.0, _7.1, _5);
Goto(bb1)
}
bb1 = {
_2 = (_1.0, _7.1, _3);
RET = [47215_u16,527_u16,34304_u16,61865_u16,26400_u16,48799_u16];
_4.2 = !_3;
Goto(bb2)
}
bb2 = {
RET = [40943_u16,20544_u16,34993_u16,45810_u16,7004_u16,39317_u16];
_4.0 = !_2.0;
_6 = _4.2;
_2 = _4;
_1.0 = 15259973_u32 as u64;
_4.2 = -_5;
_11 = [26410741750927477462903117408886418671_u128,297760224081437748161346248271530893296_u128,68792540663933760106271140891250565506_u128,178628257592667121077078984039059519843_u128,251857520292218126802754559798015057654_u128];
_2.0 = _1.0;
_8 = 5819777287775095822_i64;
_3 = _5;
_6 = !_3;
_1.2 = 7_usize as isize;
_9 = [true];
_6 = -_3;
_1.0 = _2.0 | _2.0;
_1.2 = _4.2 + _2.2;
_12.fld3 = (RET,);
_9 = [true];
_1.2 = _5;
match _8 {
5819777287775095822 => bb3,
_ => bb1
}
}
bb3 = {
_1.1 = -_2.1;
_4.1 = _7.1 & _7.1;
_1.1 = 7446_i16 as i128;
match _8 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5819777287775095822 => bb8,
_ => bb7
}
}
bb4 = {
RET = [40943_u16,20544_u16,34993_u16,45810_u16,7004_u16,39317_u16];
_4.0 = !_2.0;
_6 = _4.2;
_2 = _4;
_1.0 = 15259973_u32 as u64;
_4.2 = -_5;
_11 = [26410741750927477462903117408886418671_u128,297760224081437748161346248271530893296_u128,68792540663933760106271140891250565506_u128,178628257592667121077078984039059519843_u128,251857520292218126802754559798015057654_u128];
_2.0 = _1.0;
_8 = 5819777287775095822_i64;
_3 = _5;
_6 = !_3;
_1.2 = 7_usize as isize;
_9 = [true];
_6 = -_3;
_1.0 = _2.0 | _2.0;
_1.2 = _4.2 + _2.2;
_12.fld3 = (RET,);
_9 = [true];
_1.2 = _5;
match _8 {
5819777287775095822 => bb3,
_ => bb1
}
}
bb5 = {
_2 = (_1.0, _7.1, _3);
RET = [47215_u16,527_u16,34304_u16,61865_u16,26400_u16,48799_u16];
_4.2 = !_3;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_1.2 = !_3;
_4.0 = 24367_i16 as u64;
RET = [58232_u16,24208_u16,18510_u16,28741_u16,29313_u16,41954_u16];
_2 = (_4.0, _7.1, _3);
RET = _12.fld3.0;
_14 = 56_i8 as f64;
_7 = (1153598114_i32, _4.1);
_1.1 = !_4.1;
_7 = ((-1156057530_i32), _2.1);
_12.fld2 = _14;
_11 = [42764432573779690654833386544668919402_u128,336206759809254604905831098976850494235_u128,229530548340862805949153912071062003727_u128,188316718915144161970378745531500191553_u128,199237331460269170027775870093918356688_u128];
_10 = [3416132843_u32,2738197157_u32];
_14 = _12.fld2 + _12.fld2;
_3 = 87_u8 as isize;
_1.0 = _2.0 ^ _2.0;
_1.1 = _4.1 + _4.1;
_12.fld1 = [35409_u16,62044_u16,59667_u16,33575_u16,44181_u16];
_4 = (_2.0, _1.1, _5);
_16.0 = _8 - _8;
_12.fld3.0 = [51916_u16,20402_u16,43608_u16,44913_u16,42285_u16,62692_u16];
RET = [18246_u16,42007_u16,19471_u16,4703_u16,2766_u16,51497_u16];
Goto(bb9)
}
bb9 = {
_1.2 = _2.2;
_10 = [2352866345_u32,1583559240_u32];
_7.1 = _2.1 - _4.1;
_6 = _4.2 & _5;
_12.fld0.1 = [_16.0,_16.0];
_12.fld3 = (RET,);
_16 = (_8, _8);
Call(_4.1 = core::intrinsics::bswap(_7.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7 = (1859640582_i32, _1.1);
_2.1 = _1.1;
_13 = (-100_i8);
_13 = 28_i8 | 45_i8;
Goto(bb11)
}
bb11 = {
_1.1 = _4.1;
_5 = _6 * _2.2;
_7.0 = (-1667234431_i32) * (-743347862_i32);
_9 = [true];
_11 = [97273251672837322847422131844572516186_u128,272626304440312301490129949727517235898_u128,88612182280510888000208195595105948332_u128,201980558417625471093104600069518755255_u128,335314730284916102790978894297427816594_u128];
_14 = _16.1 as f64;
_2.0 = _1.0;
_4.2 = _5;
_12.fld3.0 = [6460_u16,13969_u16,8244_u16,16424_u16,60620_u16,44479_u16];
_21 = -_4.2;
_17 = Adt62::Variant3 { fld0: _11 };
_12.fld0.0 = _12.fld0.1;
_16 = (_8, _8);
_4.1 = -_7.1;
_23 = '\u{c5ea0}';
match _16.1 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
5819777287775095822 => bb17,
_ => bb16
}
}
bb12 = {
_2 = (_1.0, _7.1, _3);
RET = [47215_u16,527_u16,34304_u16,61865_u16,26400_u16,48799_u16];
_4.2 = !_3;
Goto(bb2)
}
bb13 = {
_1.2 = _2.2;
_10 = [2352866345_u32,1583559240_u32];
_7.1 = _2.1 - _4.1;
_6 = _4.2 & _5;
_12.fld0.1 = [_16.0,_16.0];
_12.fld3 = (RET,);
_16 = (_8, _8);
Call(_4.1 = core::intrinsics::bswap(_7.1), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
RET = [40943_u16,20544_u16,34993_u16,45810_u16,7004_u16,39317_u16];
_4.0 = !_2.0;
_6 = _4.2;
_2 = _4;
_1.0 = 15259973_u32 as u64;
_4.2 = -_5;
_11 = [26410741750927477462903117408886418671_u128,297760224081437748161346248271530893296_u128,68792540663933760106271140891250565506_u128,178628257592667121077078984039059519843_u128,251857520292218126802754559798015057654_u128];
_2.0 = _1.0;
_8 = 5819777287775095822_i64;
_3 = _5;
_6 = !_3;
_1.2 = 7_usize as isize;
_9 = [true];
_6 = -_3;
_1.0 = _2.0 | _2.0;
_1.2 = _4.2 + _2.2;
_12.fld3 = (RET,);
_9 = [true];
_1.2 = _5;
match _8 {
5819777287775095822 => bb3,
_ => bb1
}
}
bb15 = {
Return()
}
bb16 = {
RET = [40943_u16,20544_u16,34993_u16,45810_u16,7004_u16,39317_u16];
_4.0 = !_2.0;
_6 = _4.2;
_2 = _4;
_1.0 = 15259973_u32 as u64;
_4.2 = -_5;
_11 = [26410741750927477462903117408886418671_u128,297760224081437748161346248271530893296_u128,68792540663933760106271140891250565506_u128,178628257592667121077078984039059519843_u128,251857520292218126802754559798015057654_u128];
_2.0 = _1.0;
_8 = 5819777287775095822_i64;
_3 = _5;
_6 = !_3;
_1.2 = 7_usize as isize;
_9 = [true];
_6 = -_3;
_1.0 = _2.0 | _2.0;
_1.2 = _4.2 + _2.2;
_12.fld3 = (RET,);
_9 = [true];
_1.2 = _5;
match _8 {
5819777287775095822 => bb3,
_ => bb1
}
}
bb17 = {
_16.1 = _16.0 << _2.1;
_25.2.1 = _4.1;
_2 = (_1.0, _25.2.1, _21);
_4.0 = _23 as u64;
_25.3 = (_12.fld0.0, _12.fld0.0);
_26 = _7;
_4.1 = _26.1;
RET = _12.fld3.0;
_19 = [76_u8];
SetDiscriminant(_17, 0);
Goto(bb18)
}
bb18 = {
Call(_29 = dump_var(10_usize, 13_usize, Move(_13), 3_usize, Move(_3), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(10_usize, 11_usize, Move(_11), 7_usize, Move(_7), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u64,mut _2: isize,mut _3: (u64, i128, isize),mut _4: isize,mut _5: isize,mut _6: isize,mut _7: bool,mut _8: [u16; 6],mut _9: isize,mut _10: isize,mut _11: isize) -> i128 {
mir! {
type RET = i128;
let _12: Adt60;
let _13: ([i64; 2], [i64; 2]);
let _14: [bool; 1];
let _15: Adt58;
let _16: bool;
let _17: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]);
let _18: char;
let _19: (i64, i64);
let _20: (([i64; 2], [i64; 2]), f32, (i32, i128));
let _21: bool;
let _22: f32;
let _23: Adt54;
let _24: [i64; 2];
let _25: Adt51;
let _26: f32;
let _27: Adt60;
let _28: f32;
let _29: ([i64; 2], [i64; 2]);
let _30: ();
let _31: ();
{
_12.fld1.fld0 = [124_u8];
_12.fld1.fld1 = [_3.1,_3.1,_3.1];
_12.fld0 = !3834280885_u32;
_1 = !_3.0;
_3.2 = _3.1 as isize;
_12.fld0 = 3188006294_u32;
_8 = [20985_u16,17040_u16,36051_u16,39474_u16,52402_u16,24338_u16];
_3.0 = !_1;
_10 = 10912_i16 as isize;
RET = _1 as i128;
_12.fld0 = 3041583358_u32;
_12.fld2.2.1 = _7 as i128;
_12.fld0 = 2611847368_u32 << _11;
_12.fld2.2.1 = -_3.1;
_14 = [_7];
_12.fld2.1 = (-1245714602_i32) as f32;
_14 = [_7];
_12.fld0 = 618994009_u32;
Goto(bb1)
}
bb1 = {
_12.fld2.0.0 = [(-3032576247079576752_i64),(-5399658321869673510_i64)];
_12.fld2.2 = ((-711023463_i32), _3.1);
_14 = [_7];
_15 = Adt58 { fld0: _12.fld1.fld0,fld1: _12.fld1.fld1 };
_13.1 = [(-6484295340825209495_i64),1785597149202336484_i64];
_12.fld1.fld1 = [_12.fld2.2.1,_3.1,_3.1];
_12.fld2.0.1 = [2419648095274192125_i64,(-2190260699758124335_i64)];
_3.2 = _6;
_3.0 = !_1;
_4 = 1_usize as isize;
_17.2.2 = _9;
Call(_17.2.1 = core::intrinsics::transmute(RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.3 = _12.fld2.0;
_6 = _9;
_13.0 = [(-1934342426484632176_i64),5225620165120577785_i64];
_17.1 = !_12.fld0;
_11 = (-20844_i16) as isize;
_1 = _3.0 << _17.2.2;
_17.5 = [(-3426687856128355284_i64),(-4659225571029525849_i64)];
_3.1 = RET;
_16 = _7;
_18 = '\u{b318b}';
match _12.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
618994009 => bb9,
_ => bb8
}
}
bb3 = {
_12.fld2.0.0 = [(-3032576247079576752_i64),(-5399658321869673510_i64)];
_12.fld2.2 = ((-711023463_i32), _3.1);
_14 = [_7];
_15 = Adt58 { fld0: _12.fld1.fld0,fld1: _12.fld1.fld1 };
_13.1 = [(-6484295340825209495_i64),1785597149202336484_i64];
_12.fld1.fld1 = [_12.fld2.2.1,_3.1,_3.1];
_12.fld2.0.1 = [2419648095274192125_i64,(-2190260699758124335_i64)];
_3.2 = _6;
_3.0 = !_1;
_4 = 1_usize as isize;
_17.2.2 = _9;
Call(_17.2.1 = core::intrinsics::transmute(RET), ReturnTo(bb2), UnwindUnreachable())
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
_12.fld1.fld1 = [RET,_3.1,_3.1];
_12.fld2.2.0 = (-1625670994_i32) + 184369816_i32;
_15 = Adt58 { fld0: _12.fld1.fld0,fld1: _12.fld1.fld1 };
_7 = !_16;
_12.fld2.2 = ((-1486458994_i32), _17.2.1);
_17.0 = _17.1 + _12.fld0;
_12.fld2.0 = (_13.0, _17.5);
_4 = _6;
_9 = 107_u8 as isize;
_15 = _12.fld1;
_20.2.1 = _17.2.1 | _12.fld2.2.1;
_5 = _2;
_20.1 = _17.2.1 as f32;
_20.2 = (_12.fld2.2.0, _3.1);
RET = _20.2.1 & _12.fld2.2.1;
_19.1 = !5470874202530705387_i64;
_19.0 = 814_u16 as i64;
_20.0.0 = _17.3.1;
_15.fld1 = [_3.1,_17.2.1,_12.fld2.2.1];
_12.fld2.1 = _20.1;
RET = _3.1;
_17.2.0 = _1;
_7 = _16 | _16;
_12.fld2.2.0 = _20.2.0 >> _12.fld2.2.1;
_12.fld2.2.1 = _3.1 | _20.2.1;
_6 = !_3.2;
_12.fld2.0.0 = [_19.1,_19.1];
_20.0.1 = [_19.1,_19.1];
_12.fld2.1 = _12.fld2.2.1 as f32;
match _20.2.0 {
0 => bb8,
1 => bb2,
340282366920938463463374607430281752462 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_21 = !_7;
_17.5 = _20.0.0;
_12.fld2.2.1 = RET << _12.fld2.2.0;
_14 = [_7];
_15.fld1 = [_12.fld2.2.1,_17.2.1,_20.2.1];
_20.2 = (_12.fld2.2.0, _12.fld2.2.1);
_4 = _2 | _5;
_15.fld1 = [_12.fld2.2.1,_12.fld2.2.1,_12.fld2.2.1];
_9 = -_2;
_20.0.1 = _17.3.0;
_22 = 53112_u16 as f32;
_12.fld2.2 = _20.2;
_7 = !_21;
_20.2.1 = _12.fld2.2.1 | _3.1;
_20 = _12.fld2;
_17.5 = _12.fld2.0.1;
_20.2 = (_12.fld2.2.0, _12.fld2.2.1);
_13 = (_20.0.0, _12.fld2.0.1);
_8 = [28602_u16,54756_u16,55559_u16,5615_u16,48288_u16,35859_u16];
_13.0 = _17.3.1;
_12.fld2.2.1 = -RET;
_12.fld0 = _18 as u32;
_10 = _17.2.2;
_18 = '\u{a9ea6}';
Goto(bb12)
}
bb12 = {
_12.fld2.2.0 = _20.2.0 | _20.2.0;
_2 = 0_i8 as isize;
_17.3.1 = _12.fld2.0.1;
_20 = _12.fld2;
_12.fld2.0 = (_17.5, _17.5);
_3.2 = _20.2.0 as isize;
_17.3 = _20.0;
_20.0.0 = [_19.0,_19.0];
_14 = [_16];
_15.fld0 = [97_u8];
_16 = !_21;
_12.fld1.fld0 = _15.fld0;
_12.fld2.2 = (_20.2.0, _3.1);
_13.0 = [_19.0,_19.0];
_17.4 = [29749636996316193411425985115354741488_u128,209830747411155298546176557532825142077_u128,126573352475763062359244206231751127050_u128,90482805221844973310318710329369978594_u128,182421450092770749278479473914537270015_u128];
_19.1 = -_19.0;
_20.2.0 = _17.2.0 as i32;
_17.0 = 163923588468433832576208397013816696395_u128 as u32;
_12.fld2.2.0 = _20.2.0;
_13 = (_12.fld2.0.0, _20.0.0);
_17.5 = [_19.0,_19.1];
_12.fld1 = _15;
_5 = _4;
_17.2.2 = 15427_u16 as isize;
_27.fld1.fld0 = [172_u8];
_26 = _20.1 - _20.1;
Goto(bb13)
}
bb13 = {
_17.2.1 = _20.2.1;
Call(_12.fld2.2.1 = core::intrinsics::transmute(_17.3.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_27.fld1.fld0 = _15.fld0;
_15.fld0 = [252_u8];
_3.1 = _17.2.1;
_20.1 = 120528440889748466427834651568283134990_u128 as f32;
_13.1 = [_19.1,_19.0];
_20.0.1 = [_19.0,_19.1];
_6 = _12.fld2.1 as isize;
_27.fld1.fld1 = _15.fld1;
_6 = _3.2;
_17.2.0 = _1;
RET = 20596_u16 as i128;
RET = _20.2.1 >> _17.2.0;
_12.fld1.fld1 = [RET,RET,RET];
_27.fld2.0.1 = [_19.0,_19.1];
_24 = [_19.1,_19.0];
_12.fld2.2 = (_20.2.0, RET);
_17.2.0 = _1;
_27.fld2.2.1 = !_12.fld2.2.1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(11_usize, 6_usize, Move(_6), 3_usize, Move(_3), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(11_usize, 1_usize, Move(_1), 18_usize, Move(_18), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(11_usize, 9_usize, Move(_9), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: f64,mut _2: isize,mut _3: u64,mut _4: u64,mut _5: u64,mut _6: u64,mut _7: i128,mut _8: isize,mut _9: isize,mut _10: u64,mut _11: u64,mut _12: u64,mut _13: (u64, i128, isize),mut _14: i128) -> isize {
mir! {
type RET = isize;
let _15: [u8; 5];
let _16: [u32; 2];
let _17: *mut i128;
let _18: (([i64; 2], [i64; 2]), f32, (i32, i128));
let _19: f64;
let _20: ();
let _21: ();
{
_4 = _3 + _3;
_11 = !_12;
_1 = 118_i8 as f64;
_13 = (_3, _14, _8);
_2 = !_13.2;
RET = _13.1 as isize;
_11 = _10;
RET = -_2;
_3 = _5 | _6;
_3 = 6013827164065865665_usize as u64;
_5 = _6 ^ _11;
_15 = [250_u8,162_u8,157_u8,64_u8,38_u8];
_10 = _6 | _5;
_16 = [3077632078_u32,1719355290_u32];
_7 = _13.1;
_2 = !_9;
_13 = (_11, _14, RET);
_13 = (_6, _14, _2);
_18.2.1 = !_13.1;
_18.2.0 = -(-1325353002_i32);
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(12_usize, 15_usize, Move(_15), 2_usize, Move(_2), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_20 = dump_var(12_usize, 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5), 21_usize, _21), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *const isize,mut _2: isize,mut _3: u64) -> i128 {
mir! {
type RET = i128;
let _4: char;
let _5: i64;
let _6: [u8; 5];
let _7: f32;
let _8: [u32; 2];
let _9: [u16; 5];
let _10: u32;
let _11: isize;
let _12: [u16; 6];
let _13: Adt48;
let _14: [u8; 5];
let _15: ([i128; 3], *mut i128);
let _16: ([i64; 2], [i64; 2]);
let _17: char;
let _18: Adt58;
let _19: (i128, u64, i32);
let _20: Adt56;
let _21: (i32, i128);
let _22: bool;
let _23: char;
let _24: isize;
let _25: bool;
let _26: isize;
let _27: f32;
let _28: (u32, u8);
let _29: isize;
let _30: isize;
let _31: ([i64; 2], [i64; 2]);
let _32: [u32; 2];
let _33: &'static u16;
let _34: [u32; 2];
let _35: [char; 1];
let _36: *mut &'static u16;
let _37: Adt60;
let _38: ();
let _39: ();
{
RET = true as i128;
_1 = core::ptr::addr_of!((*_1));
_1 = core::ptr::addr_of!((*_1));
_3 = !5789671079926976510_u64;
_2 = 60548_u16 as isize;
_2 = (*_1);
RET = !117707066329179727336813691008886903150_i128;
RET = -23471904086503559587348465768990888582_i128;
_5 = -(-5849549709680822964_i64);
(*_1) = -_2;
RET = (-7067655638155126811359038995210509460_i128) - 36215278412932635334828134193120493581_i128;
_5 = 1472496536_u32 as i64;
_5 = 4_usize as i64;
_2 = (*_1) | (*_1);
_4 = '\u{d6b58}';
Call(_6 = fn14((*_1), (*_1), _1, _2, _2, _2, (*_1), _2, _2, _1, _4, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_1) = _2 & _2;
_5 = 77_u8 as i64;
_3 = 53_i8 as u64;
(*_1) = _2 >> _2;
_7 = 176_u8 as f32;
_5 = _7 as i64;
RET = 110414730040517970301524173717730419621_i128 ^ (-67850880239332333323398106529279947280_i128);
_1 = core::ptr::addr_of!((*_1));
_7 = RET as f32;
_6 = [72_u8,213_u8,160_u8,70_u8,188_u8];
_2 = (*_1) - (*_1);
RET = 143502764279761318255944661703533468638_i128;
_1 = core::ptr::addr_of!((*_1));
_6 = [152_u8,70_u8,99_u8,194_u8,70_u8];
_7 = 336526050052951381760330658680750160568_u128 as f32;
RET = 14009520627654507685_usize as i128;
_1 = core::ptr::addr_of!(_2);
(*_1) = 20_isize << _3;
_8 = [2314209171_u32,2536529792_u32];
_7 = 71_u8 as f32;
_2 = 35_i8 as isize;
_2 = -(-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_6 = [206_u8,172_u8,220_u8,49_u8,164_u8];
RET = (-77053048769067319350866210808185619535_i128) - 98016155201722250307261798671009938809_i128;
Call(_5 = core::intrinsics::transmute((*_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = [19368_u16,14416_u16,57183_u16,63303_u16,45994_u16];
_10 = 3455157001_u32;
(*_1) = 9223372036854775807_isize;
_4 = '\u{ec9e7}';
_10 = 2663745998_u32;
_9 = [20407_u16,43575_u16,17850_u16,32198_u16,35820_u16];
match (*_1) {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb4 = {
_10 = !2736710644_u32;
_4 = '\u{64d7c}';
_2 = (-9223372036854775808_isize);
RET = !158375612789649420479413409363221377481_i128;
_12 = [717_u16,18173_u16,46020_u16,35312_u16,18597_u16,43709_u16];
_6 = [255_u8,27_u8,195_u8,162_u8,185_u8];
_11 = (*_1);
_2 = _5 as isize;
_8 = [_10,_10];
(*_1) = _11 - _11;
_1 = core::ptr::addr_of!((*_1));
Goto(bb5)
}
bb5 = {
_15.1 = core::ptr::addr_of_mut!(RET);
_2 = !_11;
_15.0 = [RET,RET,RET];
_7 = _10 as f32;
RET = 217_u8 as i128;
_14 = [250_u8,87_u8,231_u8,77_u8,208_u8];
_16.0 = [_5,_5];
_16.1 = [_5,_5];
_1 = core::ptr::addr_of!(_2);
_10 = !3634937578_u32;
_2 = _11 << _5;
_17 = _4;
_3 = _10 as u64;
_7 = _5 as f32;
(*_1) = _11 << _5;
_19.2 = (-451852275_i32) - (-988808641_i32);
_15.1 = core::ptr::addr_of_mut!(RET);
_21.1 = -RET;
_19.1 = _3 | _3;
_10 = 1425135511_u32 - 3415786058_u32;
Goto(bb6)
}
bb6 = {
_18.fld0 = [218_u8];
_18.fld1 = _15.0;
_21 = (_19.2, RET);
_11 = (*_1);
_3 = _4 as u64;
_16.1 = _16.0;
_15.0 = [_21.1,RET,RET];
_16.1 = _16.0;
_19.2 = !_21.0;
_19.2 = !_21.0;
_3 = !_19.1;
_5 = !2503383825293010920_i64;
_8 = [_10,_10];
RET = !_21.1;
_19 = (_21.1, _3, _21.0);
_18.fld1 = [_19.0,_19.0,RET];
_2 = _5 as isize;
_15.0 = [RET,_19.0,_19.0];
_11 = 312410209627205034622199954533448143228_u128 as isize;
Goto(bb7)
}
bb7 = {
RET = _21.1;
_7 = 225_u8 as f32;
RET = !_19.0;
_6 = [96_u8,225_u8,20_u8,92_u8,208_u8];
_18.fld1 = [_21.1,_21.1,_19.0];
_2 = 6168_u16 as isize;
_7 = _5 as f32;
(*_1) = _11 | _11;
(*_1) = _19.2 as isize;
_6 = [191_u8,9_u8,17_u8,19_u8,115_u8];
_21.1 = RET * _19.0;
_24 = !(*_1);
_19 = (_21.1, _3, _21.0);
_15.1 = core::ptr::addr_of_mut!(RET);
_3 = !_19.1;
(*_1) = _24 - _24;
(*_1) = 98129727943155669890036010287419107350_u128 as isize;
_19.0 = !_21.1;
_7 = 58_i8 as f32;
Goto(bb8)
}
bb8 = {
_16.0 = [_5,_5];
_26 = _7 as isize;
_15.0 = [_19.0,_19.0,_19.0];
_16.0 = [_5,_5];
_4 = _17;
_19.2 = _21.0;
_16.1 = _16.0;
_25 = !false;
_19.0 = RET;
_14 = _6;
_5 = 9097927629856721308_i64;
_14 = [246_u8,63_u8,2_u8,182_u8,117_u8];
_25 = _3 != _3;
_28.0 = _7 as u32;
(*_1) = _26;
_16.0 = [_5,_5];
_28 = (_10, 220_u8);
_14 = [_28.1,_28.1,_28.1,_28.1,_28.1];
_15.0 = [_19.0,_21.1,_21.1];
match _5 {
0 => bb1,
1 => bb2,
2 => bb9,
9097927629856721308 => bb11,
_ => bb10
}
}
bb9 = {
_15.1 = core::ptr::addr_of_mut!(RET);
_2 = !_11;
_15.0 = [RET,RET,RET];
_7 = _10 as f32;
RET = 217_u8 as i128;
_14 = [250_u8,87_u8,231_u8,77_u8,208_u8];
_16.0 = [_5,_5];
_16.1 = [_5,_5];
_1 = core::ptr::addr_of!(_2);
_10 = !3634937578_u32;
_2 = _11 << _5;
_17 = _4;
_3 = _10 as u64;
_7 = _5 as f32;
(*_1) = _11 << _5;
_19.2 = (-451852275_i32) - (-988808641_i32);
_15.1 = core::ptr::addr_of_mut!(RET);
_21.1 = -RET;
_19.1 = _3 | _3;
_10 = 1425135511_u32 - 3415786058_u32;
Goto(bb6)
}
bb10 = {
_9 = [19368_u16,14416_u16,57183_u16,63303_u16,45994_u16];
_10 = 3455157001_u32;
(*_1) = 9223372036854775807_isize;
_4 = '\u{ec9e7}';
_10 = 2663745998_u32;
_9 = [20407_u16,43575_u16,17850_u16,32198_u16,35820_u16];
match (*_1) {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb11 = {
_19.1 = !_3;
_7 = _5 as f32;
_14 = [_28.1,_28.1,_28.1,_28.1,_28.1];
_10 = _28.0;
_15.0 = _18.fld1;
_19.1 = !_3;
_11 = !_24;
_26 = _19.2 as isize;
_4 = _17;
_3 = 773_i16 as u64;
_25 = !true;
_22 = _25;
_23 = _4;
_31 = _16;
_3 = !_19.1;
_17 = _4;
_19.2 = -_21.0;
_27 = -_7;
_16 = (_31.1, _31.1);
_27 = _7;
_11 = -_24;
_30 = _26 - _24;
_17 = _23;
_31.0 = [_5,_5];
_31.0 = [_5,_5];
_28 = (_10, 156_u8);
match _5 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb12,
9097927629856721308 => bb14,
_ => bb13
}
}
bb12 = {
_9 = [19368_u16,14416_u16,57183_u16,63303_u16,45994_u16];
_10 = 3455157001_u32;
(*_1) = 9223372036854775807_isize;
_4 = '\u{ec9e7}';
_10 = 2663745998_u32;
_9 = [20407_u16,43575_u16,17850_u16,32198_u16,35820_u16];
match (*_1) {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb13 = {
_15.1 = core::ptr::addr_of_mut!(RET);
_2 = !_11;
_15.0 = [RET,RET,RET];
_7 = _10 as f32;
RET = 217_u8 as i128;
_14 = [250_u8,87_u8,231_u8,77_u8,208_u8];
_16.0 = [_5,_5];
_16.1 = [_5,_5];
_1 = core::ptr::addr_of!(_2);
_10 = !3634937578_u32;
_2 = _11 << _5;
_17 = _4;
_3 = _10 as u64;
_7 = _5 as f32;
(*_1) = _11 << _5;
_19.2 = (-451852275_i32) - (-988808641_i32);
_15.1 = core::ptr::addr_of_mut!(RET);
_21.1 = -RET;
_19.1 = _3 | _3;
_10 = 1425135511_u32 - 3415786058_u32;
Goto(bb6)
}
bb14 = {
_5 = (-29748_i16) as i64;
_10 = _28.0 - _28.0;
_4 = _23;
RET = _21.1 + _19.0;
_29 = _11;
RET = _21.1;
_11 = _29 & _26;
_26 = _2;
_21.1 = RET;
_37.fld2.1 = _27 - _7;
_28 = (_10, 162_u8);
_2 = _29 >> _21.1;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(13_usize, 16_usize, Move(_16), 30_usize, Move(_30), 12_usize, Move(_12), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(13_usize, 26_usize, Move(_26), 9_usize, Move(_9), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(13_usize, 31_usize, Move(_31), 22_usize, Move(_22), 3_usize, Move(_3), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: isize,mut _3: *const isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: *const isize,mut _11: char,mut _12: isize,mut _13: isize) -> [u8; 5] {
mir! {
type RET = [u8; 5];
let _14: u16;
let _15: [u8; 5];
let _16: (([i64; 2], [i64; 2]), f32, (i32, i128));
let _17: Adt51;
let _18: (u64, i128, isize);
let _19: isize;
let _20: Adt61;
let _21: Adt61;
let _22: *mut i128;
let _23: *mut *mut i16;
let _24: Adt55;
let _25: ();
let _26: ();
{
RET = [1_u8,130_u8,96_u8,86_u8,141_u8];
_7 = _9 + _9;
_2 = -_7;
_5 = _2;
(*_10) = _7 - _5;
_3 = _10;
_13 = !_2;
RET = [160_u8,22_u8,76_u8,73_u8,245_u8];
_6 = (*_10);
_4 = !_13;
_11 = '\u{e3bf4}';
_3 = _10;
_10 = core::ptr::addr_of!((*_3));
(*_10) = !_13;
(*_10) = _2 + _4;
_2 = -(*_10);
_12 = !(*_3);
Goto(bb1)
}
bb1 = {
_7 = (*_10);
_12 = (*_3) | _2;
_4 = 1_usize as isize;
(*_10) = _13;
_14 = 336537889241285708594741096220851540328_u128 as u16;
_2 = -_12;
_6 = _5 | (*_3);
Goto(bb2)
}
bb2 = {
RET = [89_u8,231_u8,78_u8,207_u8,86_u8];
_10 = _3;
_8 = _2 << _5;
_4 = (*_10);
_13 = _7;
_1 = _4;
(*_3) = -_12;
(*_10) = -_1;
RET = [107_u8,50_u8,141_u8,56_u8,255_u8];
_16.0.1 = [(-4978737280658003104_i64),8816137032773092141_i64];
_6 = _12;
RET = [78_u8,50_u8,172_u8,58_u8,86_u8];
_1 = !_8;
Call(_16 = fn15(_4, _12, _4, _10, _3, (*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = '\u{758c4}';
_5 = _7;
_13 = _8 + _8;
_6 = _8;
_5 = _6 ^ (*_3);
_16.2.0 = 2170102542_u32 as i32;
_16.2.0 = (-1840877190_i32);
_15 = [169_u8,18_u8,90_u8,122_u8,64_u8];
_16.0.1 = [(-4672272056884216456_i64),1579377595451036717_i64];
_15 = [95_u8,164_u8,245_u8,194_u8,201_u8];
_12 = true as isize;
_16.1 = 29814_i16 as f32;
_1 = _13;
_5 = !_7;
_16.2 = (735683815_i32, (-74082557605803353039874459110735453212_i128));
(*_3) = _4 | _5;
_4 = _9;
_1 = -(*_3);
_8 = _7;
_16.0.1 = [(-6656161832311362021_i64),4469038169154847190_i64];
_14 = 56961_u16 - 36586_u16;
match _16.2.1 {
0 => bb1,
1 => bb4,
266199809315135110423500148321032758244 => bb6,
_ => bb5
}
}
bb4 = {
RET = [89_u8,231_u8,78_u8,207_u8,86_u8];
_10 = _3;
_8 = _2 << _5;
_4 = (*_10);
_13 = _7;
_1 = _4;
(*_3) = -_12;
(*_10) = -_1;
RET = [107_u8,50_u8,141_u8,56_u8,255_u8];
_16.0.1 = [(-4978737280658003104_i64),8816137032773092141_i64];
_6 = _12;
RET = [78_u8,50_u8,172_u8,58_u8,86_u8];
_1 = !_8;
Call(_16 = fn15(_4, _12, _4, _10, _3, (*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7 = (*_10);
_12 = (*_3) | _2;
_4 = 1_usize as isize;
(*_10) = _13;
_14 = 336537889241285708594741096220851540328_u128 as u16;
_2 = -_12;
_6 = _5 | (*_3);
Goto(bb2)
}
bb6 = {
_16.2.0 = (-1539512402_i32);
(*_3) = !_13;
_6 = _5;
RET = [36_u8,108_u8,72_u8,68_u8,70_u8];
RET = [228_u8,57_u8,9_u8,130_u8,19_u8];
match _16.2.0 {
340282366920938463463374607430228699054 => bb8,
_ => bb7
}
}
bb7 = {
RET = [89_u8,231_u8,78_u8,207_u8,86_u8];
_10 = _3;
_8 = _2 << _5;
_4 = (*_10);
_13 = _7;
_1 = _4;
(*_3) = -_12;
(*_10) = -_1;
RET = [107_u8,50_u8,141_u8,56_u8,255_u8];
_16.0.1 = [(-4978737280658003104_i64),8816137032773092141_i64];
_6 = _12;
RET = [78_u8,50_u8,172_u8,58_u8,86_u8];
_1 = !_8;
Call(_16 = fn15(_4, _12, _4, _10, _3, (*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_16.1 = 7_usize as f32;
_9 = 82_u8 as isize;
Call(_1 = core::intrinsics::bswap(_2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = -_1;
_16.2 = (303443411_i32, 157910495619012375850217244740945699739_i128);
_1 = _7 >> (*_10);
_14 = 6199_i16 as u16;
_16.1 = (-5716_i16) as f32;
(*_10) = -_6;
(*_3) = !_7;
_10 = _3;
_14 = _6 as u16;
_7 = !_8;
_14 = !1917_u16;
_16.2.1 = 61944918584772925928495607929638744067_i128 ^ (-152489018746381581350044294485833597360_i128);
RET = [111_u8,85_u8,205_u8,160_u8,47_u8];
(*_10) = 16867775435551175332_usize as isize;
_6 = 1698917935_u32 as isize;
_7 = 1803156920053708413_usize as isize;
_1 = _4;
(*_10) = !_2;
_6 = _2;
Goto(bb10)
}
bb10 = {
_18 = (15144428874769159440_u64, _16.2.1, _13);
(*_10) = _8;
(*_3) = _2;
_9 = !(*_3);
_3 = _10;
_14 = !61411_u16;
_9 = 5367013411691201646_usize as isize;
_21.fld2 = [_11];
_16.2 = ((-946129956_i32), _18.1);
_13 = _6;
_20.fld5 = _16.2.0 - _16.2.0;
_13 = (*_3) << _2;
(*_10) = _13 + _18.2;
match _18.0 {
0 => bb8,
1 => bb9,
2 => bb3,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
15144428874769159440 => bb16,
_ => bb15
}
}
bb11 = {
RET = [89_u8,231_u8,78_u8,207_u8,86_u8];
_10 = _3;
_8 = _2 << _5;
_4 = (*_10);
_13 = _7;
_1 = _4;
(*_3) = -_12;
(*_10) = -_1;
RET = [107_u8,50_u8,141_u8,56_u8,255_u8];
_16.0.1 = [(-4978737280658003104_i64),8816137032773092141_i64];
_6 = _12;
RET = [78_u8,50_u8,172_u8,58_u8,86_u8];
_1 = !_8;
Call(_16 = fn15(_4, _12, _4, _10, _3, (*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_7 = (*_10);
_12 = (*_3) | _2;
_4 = 1_usize as isize;
(*_10) = _13;
_14 = 336537889241285708594741096220851540328_u128 as u16;
_2 = -_12;
_6 = _5 | (*_3);
Goto(bb2)
}
bb13 = {
RET = [89_u8,231_u8,78_u8,207_u8,86_u8];
_10 = _3;
_8 = _2 << _5;
_4 = (*_10);
_13 = _7;
_1 = _4;
(*_3) = -_12;
(*_10) = -_1;
RET = [107_u8,50_u8,141_u8,56_u8,255_u8];
_16.0.1 = [(-4978737280658003104_i64),8816137032773092141_i64];
_6 = _12;
RET = [78_u8,50_u8,172_u8,58_u8,86_u8];
_1 = !_8;
Call(_16 = fn15(_4, _12, _4, _10, _3, (*_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_11 = '\u{758c4}';
_5 = _7;
_13 = _8 + _8;
_6 = _8;
_5 = _6 ^ (*_3);
_16.2.0 = 2170102542_u32 as i32;
_16.2.0 = (-1840877190_i32);
_15 = [169_u8,18_u8,90_u8,122_u8,64_u8];
_16.0.1 = [(-4672272056884216456_i64),1579377595451036717_i64];
_15 = [95_u8,164_u8,245_u8,194_u8,201_u8];
_12 = true as isize;
_16.1 = 29814_i16 as f32;
_1 = _13;
_5 = !_7;
_16.2 = (735683815_i32, (-74082557605803353039874459110735453212_i128));
(*_3) = _4 | _5;
_4 = _9;
_1 = -(*_3);
_8 = _7;
_16.0.1 = [(-6656161832311362021_i64),4469038169154847190_i64];
_14 = 56961_u16 - 36586_u16;
match _16.2.1 {
0 => bb1,
1 => bb4,
266199809315135110423500148321032758244 => bb6,
_ => bb5
}
}
bb15 = {
_7 = (*_10);
_12 = (*_3) | _2;
_4 = 1_usize as isize;
(*_10) = _13;
_14 = 336537889241285708594741096220851540328_u128 as u16;
_2 = -_12;
_6 = _5 | (*_3);
Goto(bb2)
}
bb16 = {
_21.fld2 = [_11];
_22 = core::ptr::addr_of_mut!(_16.2.1);
_21.fld4 = [_14,_14,_14,_14,_14];
(*_22) = 1269688393_u32 as i128;
_19 = _2 * (*_3);
_19 = -(*_3);
_5 = (*_3) & (*_3);
_20.fld0 = core::ptr::addr_of_mut!(_21.fld6);
_16.1 = 65096308264396190092331565329258163477_u128 as f32;
_22 = core::ptr::addr_of_mut!((*_22));
_15 = [244_u8,221_u8,52_u8,80_u8,108_u8];
_1 = _18.2;
_21.fld5 = -_20.fld5;
_9 = _14 as isize;
_5 = _1 | (*_3);
_16.0.1 = [(-7400589325586384026_i64),5512208507706410943_i64];
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(14_usize, 11_usize, Move(_11), 6_usize, Move(_6), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_25 = dump_var(14_usize, 2_usize, Move(_2), 15_usize, Move(_15), 14_usize, Move(_14), 26_usize, _26), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: *const isize,mut _5: *const isize,mut _6: isize) -> (([i64; 2], [i64; 2]), f32, (i32, i128)) {
mir! {
type RET = (([i64; 2], [i64; 2]), f32, (i32, i128));
let _7: bool;
let _8: (([i64; 2], [i64; 2]), f32, (i32, i128));
let _9: [bool; 1];
let _10: u8;
let _11: [u16; 5];
let _12: *mut i16;
let _13: ();
let _14: ();
{
RET.0.1 = [(-1701100560470985128_i64),(-4419089266913993170_i64)];
_8.0.0 = [(-975380815400872069_i64),9158357793003700420_i64];
_8.0 = (RET.0.1, RET.0.1);
RET.0 = (_8.0.1, _8.0.1);
_8.2.1 = 44323376264662923996376877273803005757_i128;
RET.2.1 = _8.2.1 * _8.2.1;
_8.0.0 = RET.0.0;
_7 = !false;
RET.2 = (120338050_i32, _8.2.1);
_2 = _8.2.1 as isize;
_11 = [14923_u16,4347_u16,48595_u16,35542_u16,7258_u16];
RET.2.1 = _8.2.1 ^ _8.2.1;
RET.2.0 = RET.2.1 as i32;
match _8.2.1 {
0 => bb1,
44323376264662923996376877273803005757 => bb3,
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
(*_4) = _1 << _3;
(*_5) = _3 & _1;
_10 = RET.2.0 as u8;
_8.0.1 = [(-1471008868380121058_i64),(-129167950015381196_i64)];
RET.1 = (-55_i8) as f32;
_8.0.0 = [4597523253236397085_i64,4687875940156182008_i64];
RET.2.1 = !_8.2.1;
_10 = 126_u8 ^ 233_u8;
_10 = _7 as u8;
_8 = (RET.0, RET.1, RET.2);
_11 = [46984_u16,62558_u16,44561_u16,62829_u16,12298_u16];
_10 = _7 as u8;
RET.0 = _8.0;
RET.2.0 = _8.2.0 << (*_5);
Goto(bb4)
}
bb4 = {
Call(_13 = dump_var(15_usize, 10_usize, Move(_10), 2_usize, Move(_2), 7_usize, Move(_7), 14_usize, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{adf55}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(27_i8), std::hint::black_box(21488_i16), std::hint::black_box(1013731557_i32), std::hint::black_box((-8993892076270718372_i64)), std::hint::black_box((-100144254909947223418195988089852022583_i128)), std::hint::black_box(8988669696532879092_usize), std::hint::black_box(203_u8), std::hint::black_box(34499_u16), std::hint::black_box(68980676_u32), std::hint::black_box(17641858829221987605_u64), std::hint::black_box(45900249186816513842453732403219452231_u128));
                
            }
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: u64,
fld1: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]),
fld2: usize,
fld3: (i128, u64, i32),
fld4: [i128; 3],
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [i64; 2],
fld1: char,
fld2: [char; 1],
fld3: u16,
fld4: [u8; 1],

},
Variant1{
fld0: *const isize,
fld1: [u128; 5],
fld2: *mut i128,
fld3: f32,
fld4: *mut [i128; 3],
fld5: ([i64; 2], [i64; 2]),

},
Variant2{
fld0: *mut i128,
fld1: char,
fld2: u128,
fld3: [bool; 1],

},
Variant3{
fld0: u16,
fld1: u8,
fld2: [char; 1],
fld3: [u8; 5],
fld4: ([i128; 3], *mut i128),
fld5: f32,
fld6: [u16; 5],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [u8; 1],
fld1: *mut [i128; 3],
fld2: isize,
fld3: (i32, i128),
fld4: (i128, u64, i32),

},
Variant1{
fld0: [i128; 3],
fld1: (i128, u64, i32),
fld2: f64,
fld3: i8,
fld4: i16,
fld5: (i32, i128),

},
Variant2{
fld0: Adt47,
fld1: (([i64; 2], [i64; 2]), f32, (i32, i128)),
fld2: ([u16; 6],),
fld3: i8,
fld4: Adt46,
fld5: i32,
fld6: *const isize,
fld7: *mut *mut i16,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: ([i64; 2], [i64; 2]),
fld1: [u16; 5],
fld2: f64,
fld3: ([u16; 6],),
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: ([i64; 2], [i64; 2]),
fld1: Adt47,
fld2: isize,
fld3: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]),
fld4: (i32, i128),
fld5: i32,

},
Variant1{
fld0: Adt49,
fld1: [u16; 6],

},
Variant2{
fld0: (i64, i64),
fld1: [char; 1],
fld2: ([i64; 2], [i64; 2]),

},
Variant3{
fld0: [i64; 2],
fld1: [u8; 1],
fld2: ([i128; 3], *mut i128),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u16,
fld1: usize,
fld2: (i64, i64),
fld3: f64,

},
Variant1{
fld0: ([u16; 6],),
fld1: (([i64; 2], [i64; 2]), f32, (i32, i128)),
fld2: u8,
fld3: *mut i16,
fld4: u128,
fld5: *const isize,

},
Variant2{
fld0: [u32; 2],
fld1: (u32, u32, (u64, i128, isize), ([i64; 2], [i64; 2]), [u128; 5], [i64; 2]),
fld2: (([i64; 2], [i64; 2]), f32, (i32, i128)),
fld3: i32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [i64; 2],
fld1: f64,
fld2: Adt50,
fld3: *mut *mut i16,
fld4: (u64, i128, isize),
fld5: [u8; 5],
fld6: i64,

},
Variant1{
fld0: ([i128; 3], *mut i128),
fld1: ([u16; 6],),
fld2: Adt48,
fld3: [u16; 5],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [char; 1],
fld1: [u32; 2],

},
Variant1{
fld0: [u8; 1],
fld1: u8,
fld2: f64,
fld3: [u8; 5],
fld4: Adt48,
fld5: (([i64; 2], [i64; 2]), f32, (i32, i128)),
fld6: u128,
fld7: [u128; 5],

},
Variant2{
fld0: *mut i16,
fld1: (u64, i128, isize),
fld2: f32,
fld3: [u8; 1],
fld4: Adt50,

},
Variant3{
fld0: Adt46,
fld1: char,
fld2: Adt51,
fld3: i8,
fld4: Adt52,
fld5: i32,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: f64,
fld1: char,
fld2: Adt48,
fld3: *mut i16,
fld4: i16,
fld5: Adt53,

},
Variant1{
fld0: *const ([u16; 6],),
fld1: [u8; 5],
fld2: Adt46,
fld3: ([i128; 3], *mut i128),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [i64; 2],
fld1: u8,
fld2: f32,
fld3: u128,
fld4: Adt48,
fld5: i32,
fld6: *mut i128,

},
Variant1{
fld0: bool,
fld1: *const ([u16; 6],),
fld2: isize,
fld3: [i128; 3],
fld4: ([i64; 2], [i64; 2]),
fld5: i32,
fld6: i64,
fld7: *mut i16,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: ([i64; 2], [i64; 2]),
fld1: Adt48,
fld2: Adt47,
fld3: *mut *mut i16,
fld4: u16,

},
Variant1{
fld0: Adt47,
fld1: Adt54,

},
Variant2{
fld0: [bool; 1],
fld1: Adt53,
fld2: *mut i16,
fld3: *const ([u16; 6],),
fld4: *const isize,
fld5: [i64; 2],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *const ([u16; 6],),
fld1: usize,

},
Variant1{
fld0: f32,

},
Variant2{
fld0: bool,
fld1: Adt56,
fld2: [u8; 1],
fld3: (i64, i64),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: [u8; 1],
fld1: [i128; 3],
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: ([i64; 2], [i64; 2]),
fld1: i64,
fld2: isize,

},
Variant1{
fld0: i64,
fld1: [u8; 1],
fld2: i16,
fld3: *mut i16,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: u32,
fld1: Adt58,
fld2: (([i64; 2], [i64; 2]), f32, (i32, i128)),
fld3: *mut *mut i16,
}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt61{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt61 {
fld0: *mut *mut i16,
fld1: f64,
fld2: [char; 1],
fld3: Adt54,
fld4: [u16; 5],
fld5: i32,
fld6: *mut i16,
}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: Adt58,
fld1: u64,
fld2: u16,
fld3: i8,
fld4: [u128; 5],

},
Variant1{
fld0: Adt51,
fld1: Adt61,
fld2: *const ([u16; 6],),
fld3: [u16; 5],

},
Variant2{
fld0: [u128; 5],
fld1: (u32, u8),
fld2: [u16; 5],
fld3: f64,

},
Variant3{
fld0: [u128; 5],

}}

