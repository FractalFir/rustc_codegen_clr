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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64) -> f64 {
mir! {
type RET = f64;
let _14: u64;
let _15: u128;
let _16: bool;
let _17: usize;
let _18: u128;
let _19: Adt42;
let _20: char;
let _21: bool;
let _22: i64;
let _23: bool;
let _24: Adt40;
let _25: [u16; 1];
let _26: (u8,);
let _27: i32;
let _28: i8;
let _29: isize;
let _30: [u16; 5];
let _31: bool;
let _32: (u32,);
let _33: u8;
let _34: [bool; 1];
let _35: &'static i16;
let _36: Adt48;
let _37: &'static i16;
let _38: (u64, [u16; 5], i16);
let _39: Adt54;
let _40: ();
let _41: ();
{
_1 = true;
Call(_12 = fn1(_1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _1 as usize;
_7 = !(-1497909256721752805_i64);
_3 = 9223372036854775807_isize;
RET = 20010_u16 as f64;
_4 = 163882738758013787951288395688276719309_i128 as i8;
_13 = 3367249988667431453_u64;
_5 = !(-8670_i16);
_11 = !51915_u16;
_8 = (-104063115354497784914278578445033654556_i128);
_12 = !3494545285_u32;
_10 = 39_u8;
_11 = 1690818047_i32 as u16;
Goto(bb2)
}
bb2 = {
_4 = !(-36_i8);
_3 = _10 as isize;
_6 = _8 as i32;
_15 = _7 as u128;
_14 = _13 << _11;
_16 = _1 ^ _1;
RET = _10 as f64;
_17 = _9 ^ _9;
_14 = _13 % _13;
_9 = !_17;
_15 = !279006971139289261401498434063417445493_u128;
_2 = _12 as u128;
_8 = -(-16518011711631511870207106119013117339_i128);
_5 = !(-32081_i16);
_7 = (-3201893858158819594_i64) >> _3;
_10 = !128_u8;
RET = _7 as f64;
_7 = _6 as i64;
Call(_1 = fn13(RET, _12, _17, _3, _17, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17 = _14 as usize;
_15 = !_2;
_18 = _15;
_7 = (-1059842400966834633_i64) & 7390575670563690544_i64;
_14 = _13 - _13;
match _13 {
0 => bb1,
1 => bb2,
2 => bb4,
3367249988667431453 => bb6,
_ => bb5
}
}
bb4 = {
_4 = !(-36_i8);
_3 = _10 as isize;
_6 = _8 as i32;
_15 = _7 as u128;
_14 = _13 << _11;
_16 = _1 ^ _1;
RET = _10 as f64;
_17 = _9 ^ _9;
_14 = _13 % _13;
_9 = !_17;
_15 = !279006971139289261401498434063417445493_u128;
_2 = _12 as u128;
_8 = -(-16518011711631511870207106119013117339_i128);
_5 = !(-32081_i16);
_7 = (-3201893858158819594_i64) >> _3;
_10 = !128_u8;
RET = _7 as f64;
_7 = _6 as i64;
Call(_1 = fn13(RET, _12, _17, _3, _17, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_9 = _1 as usize;
_7 = !(-1497909256721752805_i64);
_3 = 9223372036854775807_isize;
RET = 20010_u16 as f64;
_4 = 163882738758013787951288395688276719309_i128 as i8;
_13 = 3367249988667431453_u64;
_5 = !(-8670_i16);
_11 = !51915_u16;
_8 = (-104063115354497784914278578445033654556_i128);
_12 = !3494545285_u32;
_10 = 39_u8;
_11 = 1690818047_i32 as u16;
Goto(bb2)
}
bb6 = {
_13 = _12 as u64;
_8 = (-6312127874195642663719680097617497396_i128) >> _10;
_5 = 25405_i16;
_8 = (-160202625403545506473511871953105705215_i128);
_15 = _3 as u128;
Goto(bb7)
}
bb7 = {
_20 = '\u{8309f}';
_21 = _1;
RET = _6 as f64;
_3 = !(-9223372036854775808_isize);
_18 = _15 - _2;
_6 = (-877973369_i32) - (-154483715_i32);
_16 = _17 <= _17;
_14 = _13;
_17 = !_9;
_21 = _1 == _16;
RET = _11 as f64;
_3 = !80_isize;
_7 = (-618374378045176381_i64);
_1 = !_21;
_2 = _18 ^ _18;
_12 = 109867371_u32;
_22 = _7;
_15 = _22 as u128;
_1 = !_21;
Goto(bb8)
}
bb8 = {
_20 = '\u{223a}';
_21 = _1;
_12 = 446814064_u32;
RET = _5 as f64;
_2 = _18;
_2 = _11 as u128;
_24.fld0 = _18;
_6 = !1275529445_i32;
_2 = _15;
_24.fld2 = core::ptr::addr_of!(_1);
_20 = '\u{21c24}';
_8 = -(-133289490090782569739750372107430480843_i128);
_3 = (-9223372036854775808_isize) >> _17;
_6 = _3 as i32;
_3 = RET as isize;
_24.fld2 = core::ptr::addr_of!(_1);
Goto(bb9)
}
bb9 = {
_11 = !56602_u16;
_10 = _2 as u8;
_14 = !_13;
_26 = (_10,);
_8 = !164388064359615601637864026606904487882_i128;
_15 = !_18;
_24.fld0 = _1 as u128;
_24.fld0 = _18;
_24.fld0 = _15;
_18 = _2;
_16 = !_21;
_17 = _16 as usize;
_12 = RET as u32;
_23 = _16 & _21;
_25 = [_11];
_6 = (-876353689_i32);
_11 = _2 as u16;
Goto(bb10)
}
bb10 = {
_28 = -_4;
_20 = '\u{ef4ac}';
_17 = _9;
_24.fld1 = !_11;
_22 = _7 << _9;
_30 = [_24.fld1,_11,_24.fld1,_24.fld1,_24.fld1];
_12 = !2226764502_u32;
_15 = !_24.fld0;
_12 = !2878629872_u32;
_27 = _6;
_11 = _24.fld1;
_6 = _27;
_15 = !_18;
_29 = _3;
_3 = !_29;
_22 = _7;
Goto(bb11)
}
bb11 = {
_8 = !(-73951171409909307989983467869081281924_i128);
_9 = _17;
_26 = (_10,);
RET = _3 as f64;
_4 = _9 as i8;
_34 = [_23];
_8 = _11 as i128;
_32 = (_12,);
_2 = !_24.fld0;
_29 = _3;
_35 = &_5;
_2 = _20 as u128;
_2 = !_15;
Goto(bb12)
}
bb12 = {
_26.0 = _10 * _10;
_24.fld1 = _11 & _11;
_21 = _23;
_26.0 = _10;
_30 = [_11,_24.fld1,_24.fld1,_11,_11];
_12 = _32.0 - _32.0;
_29 = _3 & _3;
_26.0 = !_10;
Call(_32.0 = fn15(_15, _34, _34, _34, _21, _24.fld0, _21, (*_35)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_33 = _26.0;
_7 = _22 ^ _22;
_30 = [_24.fld1,_24.fld1,_11,_24.fld1,_24.fld1];
RET = _8 as f64;
_9 = !_17;
_8 = _13 as i128;
_2 = !_24.fld0;
_3 = _27 as isize;
_34 = [_21];
_23 = _16 ^ _21;
_10 = !_33;
_32.0 = !_12;
_32 = (_12,);
_32.0 = _12 & _12;
Goto(bb14)
}
bb14 = {
_17 = _9;
_12 = !_32.0;
_34 = [_21];
_15 = !_24.fld0;
_24.fld1 = _11 | _11;
_15 = _32.0 as u128;
_38 = (_13, _30, (*_35));
_33 = _28 as u8;
_10 = _26.0 - _26.0;
_1 = _16 | _21;
_27 = _6 - _6;
_10 = !_26.0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(0_usize, 32_usize, Move(_32), 34_usize, Move(_34), 26_usize, Move(_26), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(0_usize, 33_usize, Move(_33), 22_usize, Move(_22), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(0_usize, 29_usize, Move(_29), 27_usize, Move(_27), 16_usize, Move(_16), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(0_usize, 6_usize, Move(_6), 2_usize, Move(_2), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> u32 {
mir! {
type RET = u32;
let _9: isize;
let _10: (u8,);
let _11: isize;
let _12: [u16; 1];
let _13: u16;
let _14: Adt47;
let _15: [bool; 2];
let _16: [bool; 1];
let _17: i16;
let _18: Adt52;
let _19: Adt52;
let _20: bool;
let _21: usize;
let _22: char;
let _23: Adt51;
let _24: f32;
let _25: ();
let _26: ();
{
RET = 1588627700_u32;
_9 = !(-9223372036854775808_isize);
_3 = _5 & _4;
_4 = !_6;
_6 = _8 >= _5;
_6 = RET != RET;
_4 = _1 < _6;
_3 = _4;
Call(RET = fn2(_9, _9, _3, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _3;
_10.0 = !18_u8;
RET = 442517710_u32;
_10.0 = _9 as u8;
_12 = [4854_u16];
_8 = !_6;
_7 = _8;
_9 = 17768787503391253810890818693716815195_u128 as isize;
_9 = !34_isize;
_5 = !_4;
_4 = _6 >= _5;
_9 = (-9223372036854775808_isize);
_1 = _2 ^ _7;
_11 = -_9;
RET = 7_usize as u32;
_7 = _4 == _3;
_7 = !_3;
RET = !1456634161_u32;
Call(_2 = fn11(_11, _4, _9, _9, _8, _8, _8, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = !65386_u16;
_13 = _10.0 as u16;
_1 = _5 == _2;
_8 = _1;
match _9 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb3 = {
_4 = _3;
_10.0 = !18_u8;
RET = 442517710_u32;
_10.0 = _9 as u8;
_12 = [4854_u16];
_8 = !_6;
_7 = _8;
_9 = 17768787503391253810890818693716815195_u128 as isize;
_9 = !34_isize;
_5 = !_4;
_4 = _6 >= _5;
_9 = (-9223372036854775808_isize);
_1 = _2 ^ _7;
_11 = -_9;
RET = 7_usize as u32;
_7 = _4 == _3;
_7 = !_3;
RET = !1456634161_u32;
Call(_2 = fn11(_11, _4, _9, _9, _8, _8, _8, _1), ReturnTo(bb2), UnwindUnreachable())
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
_7 = _2;
_7 = _1;
_10.0 = 51_u8;
_8 = _7 <= _2;
Goto(bb11)
}
bb11 = {
_4 = !_1;
_15 = [_8,_4];
_5 = !_8;
_1 = _7 < _8;
_6 = _5;
match _10.0 {
0 => bb7,
1 => bb2,
2 => bb9,
3 => bb4,
51 => bb12,
_ => bb8
}
}
bb12 = {
_12 = [_13];
_6 = _8 >= _1;
_15 = [_7,_1];
RET = 4195772038_u32 * 1865563966_u32;
_1 = _8;
_8 = !_2;
_4 = !_1;
match _10.0 {
0 => bb10,
1 => bb5,
2 => bb13,
3 => bb14,
51 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_7 = _2;
_7 = _1;
_10.0 = 51_u8;
_8 = _7 <= _2;
Goto(bb11)
}
bb15 = {
_4 = _3;
_10.0 = !18_u8;
RET = 442517710_u32;
_10.0 = _9 as u8;
_12 = [4854_u16];
_8 = !_6;
_7 = _8;
_9 = 17768787503391253810890818693716815195_u128 as isize;
_9 = !34_isize;
_5 = !_4;
_4 = _6 >= _5;
_9 = (-9223372036854775808_isize);
_1 = _2 ^ _7;
_11 = -_9;
RET = 7_usize as u32;
_7 = _4 == _3;
_7 = !_3;
RET = !1456634161_u32;
Call(_2 = fn11(_11, _4, _9, _9, _8, _8, _8, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_5 = _1;
_12 = [_13];
_3 = !_1;
_21 = 5317477814839637559_usize;
_10.0 = 5_u8 & 23_u8;
_20 = _6 < _5;
_9 = _11;
RET = 3455967419_u32;
_12 = [_13];
_4 = _20 | _2;
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(1_usize, 2_usize, Move(_2), 1_usize, Move(_1), 8_usize, Move(_8), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_25 = dump_var(1_usize, 10_usize, Move(_10), 9_usize, Move(_9), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: bool,mut _4: bool) -> u32 {
mir! {
type RET = u32;
let _5: Adt54;
let _6: (u32,);
let _7: bool;
let _8: isize;
let _9: [isize; 1];
let _10: f64;
let _11: u128;
let _12: f64;
let _13: (u32,);
let _14: f64;
let _15: char;
let _16: usize;
let _17: Adt50;
let _18: [u16; 1];
let _19: u16;
let _20: i8;
let _21: u128;
let _22: u8;
let _23: i16;
let _24: Adt42;
let _25: f32;
let _26: u128;
let _27: i32;
let _28: usize;
let _29: char;
let _30: isize;
let _31: [u16; 1];
let _32: i32;
let _33: bool;
let _34: [bool; 1];
let _35: char;
let _36: [bool; 1];
let _37: (u8,);
let _38: u32;
let _39: (char, u16, u128);
let _40: Adt45;
let _41: u32;
let _42: ();
let _43: ();
{
RET = 2170711885_u32;
RET = 2053851449_u32;
_1 = _2 >> RET;
_3 = _4 & _4;
RET = !295410987_u32;
_5 = Adt54::Variant1 { fld0: (-95120788086600359906276671425595095098_i128) };
RET = 3780574987_u32 - 3863541822_u32;
RET = !3418864882_u32;
RET = 15021281223315118754_u64 as u32;
_6.0 = RET;
_6.0 = RET;
_6.0 = !RET;
_2 = !_1;
_4 = !_3;
_4 = _3;
_3 = !_4;
RET = _6.0 - _6.0;
place!(Field::<i128>(Variant(_5, 1), 0)) = (-79213012417144060473292524511369990184_i128) + 163050868837022027920911982342488836954_i128;
_6.0 = 6873308193049807090_usize as u32;
_7 = !_3;
_8 = !_2;
_6 = (RET,);
Goto(bb1)
}
bb1 = {
_3 = _7;
_6 = (RET,);
_3 = !_7;
RET = _6.0;
_9 = [_1];
_7 = !_3;
_10 = 141_u8 as f64;
_2 = -_1;
_4 = !_7;
_7 = _3;
RET = (-603676814_i32) as u32;
_3 = _4 <= _4;
_9 = [_2];
_9 = [_8];
_12 = -_10;
_2 = 12212466230520116715_usize as isize;
_8 = _1;
_11 = 37_u8 as u128;
_6.0 = '\u{2c902}' as u32;
RET = _6.0 >> _11;
RET = _6.0 * _6.0;
Call(_12 = fn3(_10, _7, _5, _7, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _8 == _1;
_4 = _3 >= _7;
RET = _1 as u32;
_7 = !_3;
_13 = (RET,);
_6 = (_13.0,);
_14 = -_12;
_6.0 = _8 as u32;
_1 = _8;
_14 = (-7435_i16) as f64;
RET = _6.0;
_13.0 = _6.0 - RET;
_10 = _14;
_7 = _3;
_15 = '\u{340ce}';
_13 = (RET,);
_4 = !_3;
_10 = Field::<i128>(Variant(_5, 1), 0) as f64;
_13 = _6;
RET = _15 as u32;
Goto(bb3)
}
bb3 = {
_13.0 = 3293_u16 as u32;
_11 = !144424203111973729559653596523348350306_u128;
Goto(bb4)
}
bb4 = {
RET = _13.0;
_13.0 = 8125450621512873187_u64 as u32;
_16 = !2_usize;
_13 = (_6.0,);
_8 = (-699472092_i32) as isize;
_4 = _11 <= _11;
_9 = [_1];
_12 = -_14;
_7 = _3;
_4 = _3 & _7;
_9 = [_8];
_9 = [_2];
_1 = -_8;
_7 = _4 < _4;
_5 = Adt54::Variant1 { fld0: 97733918858179799782044533189724556948_i128 };
place!(Field::<i128>(Variant(_5, 1), 0)) = (-122958274783813740956331973909983280570_i128) & (-92466381570748404173511291140309681044_i128);
_1 = _2;
place!(Field::<i128>(Variant(_5, 1), 0)) = 27385516451947539874998833973568837756_i128;
_4 = !_7;
Call(_14 = fn5(Field::<i128>(Variant(_5, 1), 0), _4, _7, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = (-11_i8) - 5_i8;
_19 = 49804_u16;
SetDiscriminant(_5, 0);
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = _11;
_13 = (RET,);
Call(_3 = fn8(_4, _7, _7, RET, _7, RET, _4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = _2;
place!(Field::<bool>(Variant(_5, 0), 0)) = _7 != _7;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = _19 as u128;
_14 = -_10;
_4 = !_7;
place!(Field::<bool>(Variant(_5, 0), 0)) = !_3;
_3 = _7 > Field::<bool>(Variant(_5, 0), 0);
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)) = (_15, _19, _11);
_1 = Field::<bool>(Variant(_5, 0), 0) as isize;
_9 = [_1];
_1 = -_2;
_1 = _2 + _8;
SetDiscriminant(_5, 0);
_1 = _8 + _2;
_6 = (_13.0,);
_12 = _16 as f64;
_9 = [_1];
_1 = _2 | _8;
_21 = _11;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).0 = _15;
_21 = !_11;
_21 = _11 + _11;
_7 = !_4;
place!(Field::<bool>(Variant(_5, 0), 0)) = _7 < _3;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)) = (_15, _19, _21);
_11 = _21 * _21;
_13.0 = _6.0;
match Field::<(char, u16, u128)>(Variant(_5, 0), 1).1 {
49804 => bb7,
_ => bb3
}
}
bb7 = {
_4 = Field::<bool>(Variant(_5, 0), 0) <= Field::<bool>(Variant(_5, 0), 0);
RET = 360077728188835543_i64 as u32;
_3 = Field::<bool>(Variant(_5, 0), 0) <= _4;
_6 = (RET,);
_16 = 10259809918616501762_usize;
_28 = !_16;
_4 = _3 == _3;
_23 = !19448_i16;
_11 = RET as u128;
SetDiscriminant(_5, 1);
match _16 {
0 => bb1,
1 => bb5,
2 => bb3,
10259809918616501762 => bb8,
_ => bb4
}
}
bb8 = {
_27 = !480782137_i32;
_25 = 2296721595935464417_i64 as f32;
_28 = !_16;
_29 = _15;
place!(Field::<i128>(Variant(_5, 1), 0)) = !47875783434358825607916187145897275123_i128;
_22 = _19 as u8;
_29 = _15;
_18 = [_19];
RET = 12424137587210882434_u64 as u32;
_26 = _21;
match _16 {
0 => bb6,
10259809918616501762 => bb10,
_ => bb9
}
}
bb9 = {
_20 = (-11_i8) - 5_i8;
_19 = 49804_u16;
SetDiscriminant(_5, 0);
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = _11;
_13 = (RET,);
Call(_3 = fn8(_4, _7, _7, RET, _7, RET, _4), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_25 = _1 as f32;
RET = !_13.0;
_9 = [_2];
_8 = _1;
_1 = Field::<i128>(Variant(_5, 1), 0) as isize;
_20 = _25 as i8;
_30 = _25 as isize;
_15 = _29;
_1 = -_30;
SetDiscriminant(_5, 0);
place!(Field::<bool>(Variant(_5, 0), 0)) = _3;
_23 = -(-25667_i16);
_3 = _4 ^ Field::<bool>(Variant(_5, 0), 0);
_21 = !_26;
_16 = _21 as usize;
_28 = _19 as usize;
_32 = _27 >> _26;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).0 = _15;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = _3 as u128;
_21 = _26 + Field::<(char, u16, u128)>(Variant(_5, 0), 1).2;
_34 = [_4];
_32 = _27 * _27;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = !_21;
_9 = [_2];
_13 = (RET,);
match _19 {
0 => bb8,
1 => bb7,
2 => bb3,
49804 => bb12,
_ => bb11
}
}
bb11 = {
_27 = !480782137_i32;
_25 = 2296721595935464417_i64 as f32;
_28 = !_16;
_29 = _15;
place!(Field::<i128>(Variant(_5, 1), 0)) = !47875783434358825607916187145897275123_i128;
_22 = _19 as u8;
_29 = _15;
_18 = [_19];
RET = 12424137587210882434_u64 as u32;
_26 = _21;
match _16 {
0 => bb6,
10259809918616501762 => bb10,
_ => bb9
}
}
bb12 = {
_12 = -_14;
_8 = _1;
_19 = 51087_u16;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = !_21;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).0 = _15;
_36 = _34;
Call(_12 = fn9(_26, _6.0, Field::<(char, u16, u128)>(Variant(_5, 0), 1).2, _4, Field::<bool>(Variant(_5, 0), 0), _34, _4, Field::<(char, u16, u128)>(Variant(_5, 0), 1).2, _4, _36, _34, _4, _34, _3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_35 = _15;
_7 = !_3;
match _19 {
0 => bb11,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
51087 => bb20,
_ => bb19
}
}
bb14 = {
_3 = _7;
_6 = (RET,);
_3 = !_7;
RET = _6.0;
_9 = [_1];
_7 = !_3;
_10 = 141_u8 as f64;
_2 = -_1;
_4 = !_7;
_7 = _3;
RET = (-603676814_i32) as u32;
_3 = _4 <= _4;
_9 = [_2];
_9 = [_8];
_12 = -_10;
_2 = 12212466230520116715_usize as isize;
_8 = _1;
_11 = 37_u8 as u128;
_6.0 = '\u{2c902}' as u32;
RET = _6.0 >> _11;
RET = _6.0 * _6.0;
Call(_12 = fn3(_10, _7, _5, _7, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_27 = !480782137_i32;
_25 = 2296721595935464417_i64 as f32;
_28 = !_16;
_29 = _15;
place!(Field::<i128>(Variant(_5, 1), 0)) = !47875783434358825607916187145897275123_i128;
_22 = _19 as u8;
_29 = _15;
_18 = [_19];
RET = 12424137587210882434_u64 as u32;
_26 = _21;
match _16 {
0 => bb6,
10259809918616501762 => bb10,
_ => bb9
}
}
bb16 = {
_1 = _2;
place!(Field::<bool>(Variant(_5, 0), 0)) = _7 != _7;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = _19 as u128;
_14 = -_10;
_4 = !_7;
place!(Field::<bool>(Variant(_5, 0), 0)) = !_3;
_3 = _7 > Field::<bool>(Variant(_5, 0), 0);
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)) = (_15, _19, _11);
_1 = Field::<bool>(Variant(_5, 0), 0) as isize;
_9 = [_1];
_1 = -_2;
_1 = _2 + _8;
SetDiscriminant(_5, 0);
_1 = _8 + _2;
_6 = (_13.0,);
_12 = _16 as f64;
_9 = [_1];
_1 = _2 | _8;
_21 = _11;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).0 = _15;
_21 = !_11;
_21 = _11 + _11;
_7 = !_4;
place!(Field::<bool>(Variant(_5, 0), 0)) = _7 < _3;
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)) = (_15, _19, _21);
_11 = _21 * _21;
_13.0 = _6.0;
match Field::<(char, u16, u128)>(Variant(_5, 0), 1).1 {
49804 => bb7,
_ => bb3
}
}
bb17 = {
_20 = (-11_i8) - 5_i8;
_19 = 49804_u16;
SetDiscriminant(_5, 0);
place!(Field::<(char, u16, u128)>(Variant(_5, 0), 1)).2 = _11;
_13 = (RET,);
Call(_3 = fn8(_4, _7, _7, RET, _7, RET, _4), ReturnTo(bb6), UnwindUnreachable())
}
bb18 = {
_7 = _8 == _1;
_4 = _3 >= _7;
RET = _1 as u32;
_7 = !_3;
_13 = (RET,);
_6 = (_13.0,);
_14 = -_12;
_6.0 = _8 as u32;
_1 = _8;
_14 = (-7435_i16) as f64;
RET = _6.0;
_13.0 = _6.0 - RET;
_10 = _14;
_7 = _3;
_15 = '\u{340ce}';
_13 = (RET,);
_4 = !_3;
_10 = Field::<i128>(Variant(_5, 1), 0) as f64;
_13 = _6;
RET = _15 as u32;
Goto(bb3)
}
bb19 = {
_13.0 = 3293_u16 as u32;
_11 = !144424203111973729559653596523348350306_u128;
Goto(bb4)
}
bb20 = {
_39 = (_15, _19, _11);
Goto(bb21)
}
bb21 = {
Call(_42 = dump_var(2_usize, 15_usize, Move(_15), 9_usize, Move(_9), 27_usize, Move(_27), 7_usize, Move(_7)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_42 = dump_var(2_usize, 23_usize, Move(_23), 34_usize, Move(_34), 30_usize, Move(_30), 2_usize, Move(_2)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_42 = dump_var(2_usize, 29_usize, Move(_29), 16_usize, Move(_16), 28_usize, Move(_28), 22_usize, Move(_22)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_42 = dump_var(2_usize, 13_usize, Move(_13), 35_usize, Move(_35), 43_usize, _43, 43_usize, _43), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f64,mut _2: bool,mut _3: Adt54,mut _4: bool,mut _5: bool) -> f64 {
mir! {
type RET = f64;
let _6: Adt48;
let _7: f32;
let _8: char;
let _9: i128;
let _10: Adt38;
let _11: f64;
let _12: ([bool; 1], (u32,), isize);
let _13: [u16; 5];
let _14: bool;
let _15: i128;
let _16: [bool; 1];
let _17: *const *mut &'static i16;
let _18: ();
let _19: ();
{
RET = _1 * _1;
RET = _1;
_1 = RET + RET;
SetDiscriminant(_3, 1);
_5 = _4;
_3 = Adt54::Variant1 { fld0: 100003710618759567058910730725888527544_i128 };
place!(Field::<i128>(Variant(_3, 1), 0)) = -94821358169261490355763267098446887184_i128;
RET = 57376_u16 as f64;
RET = _1;
_7 = 2459746731_u32 as f32;
RET = _1 * _1;
_7 = (-120_i8) as f32;
_4 = _2;
place!(Field::<i128>(Variant(_3, 1), 0)) = (-58860112107409065487655785646208504572_i128) + 4193335210521471647973757572549936924_i128;
_3 = Adt54::Variant1 { fld0: 101841911489086560592384570712774887663_i128 };
_4 = _2;
RET = _1 + _1;
place!(Field::<i128>(Variant(_3, 1), 0)) = 102169854420144535146116645958970179890_i128;
_8 = '\u{eaa3a}';
SetDiscriminant(_3, 1);
_10.fld4.1.3 = !1015086652_u32;
_10.fld4.1.3 = 2150377699_u32 << 57042_u16;
_10.fld4.1.2.0 = _8;
Goto(bb1)
}
bb1 = {
_10.fld3.1 = 28629_u16 & 58458_u16;
_10.fld1.2 = 113567868400215808144741878233568593166_u128 as i64;
place!(Field::<i128>(Variant(_3, 1), 0)) = -170016435285914735556628193156332112203_i128;
Call(_10.fld1.2 = core::intrinsics::bswap(7245319271165220896_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.fld4.1.2.1 = _10.fld3.1;
SetDiscriminant(_3, 1);
place!(Field::<i128>(Variant(_3, 1), 0)) = !21379245984235931228953442120323475035_i128;
_10.fld4.1.2 = (_8, _10.fld3.1, 68442352031259373048366861822329475430_u128);
_1 = -RET;
_10.fld4.1.2.0 = _8;
_10.fld3.2 = !_10.fld4.1.2.2;
_10.fld1.1 = _1;
_10.fld1.2 = 7009205537844730620_i64;
_10.fld4.0 = 231_u8 ^ 24_u8;
_10.fld4.1.5 = _10.fld3.1;
_10.fld1 = (_10.fld4.1.3, RET, 4258738347682539113_i64);
_10.fld4.1.2.1 = _10.fld4.1.5;
_10.fld4.1.1 = [9223372036854775807_isize];
_10.fld3.0 = _8;
place!(Field::<i128>(Variant(_3, 1), 0)) = _10.fld1.0 as i128;
_10.fld4.1.4 = _10.fld4.0;
_1 = RET;
match _10.fld1.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
4258738347682539113 => bb8,
_ => bb7
}
}
bb3 = {
_10.fld3.1 = 28629_u16 & 58458_u16;
_10.fld1.2 = 113567868400215808144741878233568593166_u128 as i64;
place!(Field::<i128>(Variant(_3, 1), 0)) = -170016435285914735556628193156332112203_i128;
Call(_10.fld1.2 = core::intrinsics::bswap(7245319271165220896_i64), ReturnTo(bb2), UnwindUnreachable())
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
_1 = -_10.fld1.1;
_12.1.0 = _10.fld4.1.3 - _10.fld1.0;
RET = _1 - _1;
_10.fld4.1.1 = [2_isize];
_10.fld2 = _1 - _10.fld1.1;
_10.fld3.2 = !_10.fld4.1.2.2;
Goto(bb9)
}
bb9 = {
_12.2 = !(-9223372036854775808_isize);
Call(_10.fld4.1.3 = fn4(_10.fld1, _10.fld4.1.2, _10.fld4.1.2, _4, _10.fld3.2, _10.fld4.1.2, _10.fld1.2, _10.fld1, _12.1, _10.fld4.1.1, _10.fld1, _12.1, _10.fld1.1, _10.fld4.1.2.2, _10.fld1.1, _10.fld3.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10.fld4.1.3 = 3089042578176236083_u64 as u32;
_10.fld4.0 = _10.fld4.1.4 ^ _10.fld4.1.4;
_10.fld1.1 = -RET;
_12.0 = [_5];
_12.1.0 = _10.fld4.1.3 ^ _10.fld1.0;
_9 = !Field::<i128>(Variant(_3, 1), 0);
_14 = _5;
match _10.fld1.2 {
0 => bb6,
1 => bb8,
2 => bb11,
3 => bb12,
4258738347682539113 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
_1 = -_10.fld1.1;
_12.1.0 = _10.fld4.1.3 - _10.fld1.0;
RET = _1 - _1;
_10.fld4.1.1 = [2_isize];
_10.fld2 = _1 - _10.fld1.1;
_10.fld3.2 = !_10.fld4.1.2.2;
Goto(bb9)
}
bb13 = {
Return()
}
bb14 = {
_10.fld4.1.2.1 = _10.fld4.1.5;
_10.fld4.1.2.0 = _8;
_15 = _14 as i128;
_7 = 1317_i16 as f32;
_12.0 = [_4];
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(3_usize, 12_usize, Move(_12), 14_usize, Move(_14), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: (u32, f64, i64),mut _2: (char, u16, u128),mut _3: (char, u16, u128),mut _4: bool,mut _5: u128,mut _6: (char, u16, u128),mut _7: i64,mut _8: (u32, f64, i64),mut _9: (u32,),mut _10: [isize; 1],mut _11: (u32, f64, i64),mut _12: (u32,),mut _13: f64,mut _14: u128,mut _15: f64,mut _16: u128) -> u32 {
mir! {
type RET = u32;
let _17: char;
let _18: u32;
let _19: isize;
let _20: bool;
let _21: [isize; 1];
let _22: u64;
let _23: u32;
let _24: ();
let _25: ();
{
_1 = (_12.0, _13, _11.2);
_1.1 = _11.2 as f64;
_2.2 = !_14;
_5 = _3.2;
_6 = (_3.0, _3.1, _14);
match _1.2 {
0 => bb1,
4258738347682539113 => bb3,
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
_1.0 = _4 as u32;
match _5 {
0 => bb1,
1 => bb2,
68442352031259373048366861822329475430 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
_6.1 = !_2.1;
_6.2 = _2.1 as u128;
_13 = _15 * _1.1;
RET = !_1.0;
_15 = _11.1;
_10 = [(-9223372036854775808_isize)];
_11 = (_9.0, _8.1, _1.2);
_3.2 = _5 % _5;
_3.2 = _2.2;
_2.2 = 17675503008019473540_usize as u128;
_8.0 = (-59215853504867669545934790831726222517_i128) as u32;
_7 = _1.2;
_9.0 = _7 as u32;
_6.2 = !_2.2;
RET = (-9223372036854775808_isize) as u32;
_6.2 = !_3.2;
_20 = _13 == _1.1;
RET = _11.0;
_1.2 = -_11.2;
_6.2 = !_5;
_12.0 = RET | _1.0;
_11.1 = _15;
Goto(bb6)
}
bb6 = {
_20 = _11.2 == _7;
_10 = [9223372036854775807_isize];
_19 = (-9223372036854775808_isize);
_14 = _19 as u128;
_8.1 = _13;
_11.1 = _19 as f64;
_21 = [_19];
_21 = [_19];
_7 = -_11.2;
_11.0 = _12.0;
_12.0 = !_9.0;
_2.1 = !_6.1;
_22 = _6.1 as u64;
_3.2 = _22 as u128;
_9 = (_11.0,);
RET = _12.0 * _9.0;
_1.1 = _15;
_2.1 = _3.1;
_1.0 = _6.1 as u32;
_5 = _3.2;
_18 = 6_i8 as u32;
_15 = _13 - _8.1;
_20 = _4;
_2.1 = 2034434704_i32 as u16;
_3.2 = _14 << _1.2;
_3.1 = !_6.1;
_12 = (RET,);
Goto(bb7)
}
bb7 = {
Call(_24 = dump_var(4_usize, 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, Move(_12), 18_usize, Move(_18)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_24 = dump_var(4_usize, 3_usize, Move(_3), 16_usize, Move(_16), 21_usize, Move(_21), 2_usize, Move(_2)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i128,mut _2: bool,mut _3: bool,mut _4: bool) -> f64 {
mir! {
type RET = f64;
let _5: bool;
let _6: (u8,);
let _7: Adt38;
let _8: (u32, f64, i64);
let _9: u64;
let _10: isize;
let _11: bool;
let _12: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16);
let _13: [u16; 5];
let _14: i64;
let _15: [bool; 1];
let _16: u8;
let _17: Adt40;
let _18: isize;
let _19: isize;
let _20: u8;
let _21: Adt54;
let _22: ();
let _23: ();
{
_3 = !_4;
RET = 192122917191096513176449709541999161022_u128 as f64;
_3 = _2 ^ _2;
RET = (-7343824872367673115_i64) as f64;
_2 = _3;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
27385516451947539874998833973568837756 => bb9,
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
_4 = !_2;
_1 = !(-4777838052124462065649792480685201323_i128);
_4 = _2 & _3;
RET = 26614_i16 as f64;
_2 = !_4;
_5 = !_4;
_1 = (-6_isize) as i128;
_5 = !_4;
_7.fld4.1.2.0 = '\u{cc8fc}';
_6 = (250_u8,);
_7.fld3 = (_7.fld4.1.2.0, 57738_u16, 119755895463650637263645523342610439521_u128);
_7.fld4.1.4 = !_6.0;
_10 = !59_isize;
_7.fld2 = -RET;
_7.fld4.1.3 = 2306791259_u32;
_7.fld4.1.2.1 = _7.fld3.1 & _7.fld3.1;
_7.fld3 = (_7.fld4.1.2.0, _7.fld4.1.2.1, 86157679161668804269045781092970604395_u128);
_7.fld4.0 = _7.fld4.1.4;
_7.fld3.0 = _7.fld4.1.2.0;
_7.fld1 = (_7.fld4.1.3, _7.fld2, (-7090501349558916000_i64));
_9 = 1697269189043359139_u64 << _7.fld4.0;
_7.fld4.1.2.0 = _7.fld3.0;
RET = _7.fld2;
Goto(bb10)
}
bb10 = {
_8.2 = !_7.fld1.2;
_3 = _2 < _4;
_8 = (_7.fld1.0, _7.fld1.1, _7.fld1.2);
_7.fld1.0 = !_8.0;
Call(_11 = fn6(_5, _3, _2, _2, _6.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_1 = 129056379205845544963737965571946259424_i128;
_11 = _5 != _4;
_7.fld1.2 = _8.2;
_7.fld4.1.2.2 = _7.fld3.1 as u128;
_12.1 = [_10];
Call(_7.fld4.1.2.2 = fn7(_5, _11, _2, _4, _12.1, _11, _3, _11), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = _8.2;
_13 = [_7.fld4.1.2.1,_7.fld4.1.2.1,_7.fld3.1,_7.fld3.1,_7.fld4.1.2.1];
_12.1 = [_10];
RET = -_7.fld2;
_12.0 = core::ptr::addr_of!(_9);
_12.2.2 = !_7.fld3.2;
_7.fld1.1 = RET + _8.1;
_1 = _12.2.2 as i128;
_7.fld4.1.0 = core::ptr::addr_of!(_9);
_5 = _11 <= _11;
_7.fld4.1.4 = _7.fld4.0 >> _7.fld3.1;
_17.fld1 = _7.fld4.1.2.1 | _7.fld3.1;
_7.fld3.1 = _17.fld1;
_12.3 = _17.fld1 as u32;
_12.2.2 = _7.fld3.2;
_7.fld4.1.1 = [_10];
match _14 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
340282366920938463456284106082209295456 => bb19,
_ => bb18
}
}
bb13 = {
Return()
}
bb14 = {
_8.2 = !_7.fld1.2;
_3 = _2 < _4;
_8 = (_7.fld1.0, _7.fld1.1, _7.fld1.2);
_7.fld1.0 = !_8.0;
Call(_11 = fn6(_5, _3, _2, _2, _6.0), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_7.fld4.1.1 = [_10];
_12 = (_7.fld4.1.0, _7.fld4.1.1, _7.fld3, _7.fld4.1.3, _7.fld4.1.4, _7.fld4.1.2.1);
_8.2 = _14;
_14 = _7.fld1.2;
_7.fld1.0 = _12.2.0 as u32;
_1 = 117114593767574354117041270394046432622_i128 | 5714444393443820218079317250493839727_i128;
_8.0 = _10 as u32;
_7.fld4.1 = (_12.0, _12.1, _12.2, _7.fld1.0, _12.4, _17.fld1);
_17.fld0 = _7.fld3.2;
_7.fld4.1.2.1 = _7.fld4.1.2.0 as u16;
_6 = (_12.4,);
_6.0 = _12.4 & _12.4;
_7.fld4.1.2.1 = _17.fld1 & _7.fld3.1;
_20 = _12.4;
_17.fld1 = _9 as u16;
_12.3 = !_7.fld4.1.3;
_16 = !_6.0;
Goto(bb20)
}
bb20 = {
Call(_22 = dump_var(5_usize, 10_usize, Move(_10), 14_usize, Move(_14), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_22 = dump_var(5_usize, 6_usize, Move(_6), 1_usize, Move(_1), 23_usize, _23, 23_usize, _23), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: u8) -> bool {
mir! {
type RET = bool;
let _6: ();
let _7: ();
{
_5 = 120_u8 * 95_u8;
_2 = _1 != _3;
_4 = _2 < _3;
RET = _2;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(6_usize, 1_usize, Move(_1), 5_usize, Move(_5), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: [isize; 1],mut _6: bool,mut _7: bool,mut _8: bool) -> u128 {
mir! {
type RET = u128;
let _9: (u64, [u16; 5], i16);
let _10: u32;
let _11: f32;
let _12: (u64, [u16; 5], i16);
let _13: [u16; 1];
let _14: isize;
let _15: (u64, [u16; 5], i16);
let _16: i32;
let _17: (u32,);
let _18: isize;
let _19: (u32,);
let _20: u32;
let _21: [u16; 1];
let _22: ();
let _23: ();
{
Goto(bb1)
}
bb1 = {
_9.2 = (-17476_i16);
_6 = _1;
_9.2 = -(-30782_i16);
_9.1 = [25401_u16,475_u16,64779_u16,11849_u16,64448_u16];
RET = 235470438282300055829575909970757708332_u128 << _9.2;
_2 = !_7;
RET = 41114987770269498541535452855871122078_u128 ^ 118534290195414835493013838107644616681_u128;
RET = 89268113507758489709367809298654370829_u128;
_9.1 = [20386_u16,34987_u16,51959_u16,22875_u16,34973_u16];
_9.0 = 15010957073055995521_u64 + 2919036754855633090_u64;
match RET {
89268113507758489709367809298654370829 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_9.1 = [55720_u16,36637_u16,43610_u16,53383_u16,29925_u16];
_2 = _1;
_2 = _4;
_10 = 3750001081_u32;
RET = !176857792473346232375853918554794454551_u128;
_9.2 = (-31828_i16);
_8 = _4;
_11 = 20361_u16 as f32;
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_8 = _7;
_9.1 = [52078_u16,11245_u16,27525_u16,35623_u16,8507_u16];
RET = 154364360460259779688599980079369878008_u128 << _9.0;
_9.1 = [18240_u16,30795_u16,20800_u16,14482_u16,65130_u16];
_8 = !_3;
_3 = !_2;
_9.0 = (-96_i8) as u64;
RET = 331062132696176170756910674250539389179_u128 - 332113808584941955879816656667870452340_u128;
_4 = !_1;
_6 = _1;
_3 = _8 > _7;
Goto(bb4)
}
bb4 = {
_1 = !_4;
_9.1 = [39693_u16,9769_u16,36033_u16,56295_u16,58482_u16];
_8 = _4;
_12 = (_9.0, _9.1, _9.2);
_9.0 = _12.0 | _12.0;
_9.1 = [34556_u16,24746_u16,49555_u16,33761_u16,3956_u16];
_12 = (_9.0, _9.1, _9.2);
_7 = _3;
_13 = [62146_u16];
_5 = [9223372036854775807_isize];
_7 = _2;
_6 = _1;
RET = '\u{7f730}' as u128;
_1 = !_8;
_13 = [48494_u16];
_7 = _6;
_9.2 = _12.2;
Goto(bb5)
}
bb5 = {
_2 = _1;
_6 = _3;
_4 = _3 <= _2;
_8 = !_1;
_14 = (-9223372036854775808_isize);
_14 = 9223372036854775807_isize;
_3 = !_7;
_13 = [28830_u16];
_12.1 = [52831_u16,45039_u16,50504_u16,59460_u16,16781_u16];
_4 = !_2;
_9.2 = _9.0 as i16;
_9 = _12;
_10 = 2910241235_u32;
match _14 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb6 = {
_1 = !_4;
_9.1 = [39693_u16,9769_u16,36033_u16,56295_u16,58482_u16];
_8 = _4;
_12 = (_9.0, _9.1, _9.2);
_9.0 = _12.0 | _12.0;
_9.1 = [34556_u16,24746_u16,49555_u16,33761_u16,3956_u16];
_12 = (_9.0, _9.1, _9.2);
_7 = _3;
_13 = [62146_u16];
_5 = [9223372036854775807_isize];
_7 = _2;
_6 = _1;
RET = '\u{7f730}' as u128;
_1 = !_8;
_13 = [48494_u16];
_7 = _6;
_9.2 = _12.2;
Goto(bb5)
}
bb7 = {
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_9.1 = [55720_u16,36637_u16,43610_u16,53383_u16,29925_u16];
_2 = _1;
_2 = _4;
_10 = 3750001081_u32;
RET = !176857792473346232375853918554794454551_u128;
_9.2 = (-31828_i16);
_8 = _4;
_11 = 20361_u16 as f32;
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_8 = _7;
_9.1 = [52078_u16,11245_u16,27525_u16,35623_u16,8507_u16];
RET = 154364360460259779688599980079369878008_u128 << _9.0;
_9.1 = [18240_u16,30795_u16,20800_u16,14482_u16,65130_u16];
_8 = !_3;
_3 = !_2;
_9.0 = (-96_i8) as u64;
RET = 331062132696176170756910674250539389179_u128 - 332113808584941955879816656667870452340_u128;
_4 = !_1;
_6 = _1;
_3 = _8 > _7;
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
_9.2 = (-17476_i16);
_6 = _1;
_9.2 = -(-30782_i16);
_9.1 = [25401_u16,475_u16,64779_u16,11849_u16,64448_u16];
RET = 235470438282300055829575909970757708332_u128 << _9.2;
_2 = !_7;
RET = 41114987770269498541535452855871122078_u128 ^ 118534290195414835493013838107644616681_u128;
RET = 89268113507758489709367809298654370829_u128;
_9.1 = [20386_u16,34987_u16,51959_u16,22875_u16,34973_u16];
_9.0 = 15010957073055995521_u64 + 2919036754855633090_u64;
match RET {
89268113507758489709367809298654370829 => bb3,
_ => bb2
}
}
bb10 = {
_4 = !_1;
_2 = !_6;
_6 = !_8;
_10 = 3136366584_u32 >> RET;
_6 = _2;
_9 = _12;
_12.1 = [11557_u16,972_u16,60578_u16,780_u16,6225_u16];
_3 = !_8;
_9.2 = _11 as i16;
_5 = [_14];
_9 = (_12.0, _12.1, _12.2);
_15.2 = _12.2;
_2 = _7 | _4;
_6 = _1 == _1;
_15.2 = _10 as i16;
_3 = _7 >= _4;
_15 = (_9.0, _12.1, _9.2);
_10 = 1044034079_u32;
_2 = _4;
_14 = (-9223372036854775808_isize);
_9.1 = [27858_u16,54859_u16,45465_u16,15147_u16,57750_u16];
_15.2 = _9.2 | _9.2;
_12.0 = _9.0 - _9.0;
_17 = (_10,);
_6 = !_8;
_17.0 = 4159568851537902128_usize as u32;
match _9.2 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463463374607431768179628 => bb17,
_ => bb16
}
}
bb11 = {
_9.2 = (-17476_i16);
_6 = _1;
_9.2 = -(-30782_i16);
_9.1 = [25401_u16,475_u16,64779_u16,11849_u16,64448_u16];
RET = 235470438282300055829575909970757708332_u128 << _9.2;
_2 = !_7;
RET = 41114987770269498541535452855871122078_u128 ^ 118534290195414835493013838107644616681_u128;
RET = 89268113507758489709367809298654370829_u128;
_9.1 = [20386_u16,34987_u16,51959_u16,22875_u16,34973_u16];
_9.0 = 15010957073055995521_u64 + 2919036754855633090_u64;
match RET {
89268113507758489709367809298654370829 => bb3,
_ => bb2
}
}
bb12 = {
Return()
}
bb13 = {
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_9.1 = [55720_u16,36637_u16,43610_u16,53383_u16,29925_u16];
_2 = _1;
_2 = _4;
_10 = 3750001081_u32;
RET = !176857792473346232375853918554794454551_u128;
_9.2 = (-31828_i16);
_8 = _4;
_11 = 20361_u16 as f32;
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_8 = _7;
_9.1 = [52078_u16,11245_u16,27525_u16,35623_u16,8507_u16];
RET = 154364360460259779688599980079369878008_u128 << _9.0;
_9.1 = [18240_u16,30795_u16,20800_u16,14482_u16,65130_u16];
_8 = !_3;
_3 = !_2;
_9.0 = (-96_i8) as u64;
RET = 331062132696176170756910674250539389179_u128 - 332113808584941955879816656667870452340_u128;
_4 = !_1;
_6 = _1;
_3 = _8 > _7;
Goto(bb4)
}
bb14 = {
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_9.1 = [55720_u16,36637_u16,43610_u16,53383_u16,29925_u16];
_2 = _1;
_2 = _4;
_10 = 3750001081_u32;
RET = !176857792473346232375853918554794454551_u128;
_9.2 = (-31828_i16);
_8 = _4;
_11 = 20361_u16 as f32;
_5 = [(-9223372036854775808_isize)];
_8 = _3;
_8 = _7;
_9.1 = [52078_u16,11245_u16,27525_u16,35623_u16,8507_u16];
RET = 154364360460259779688599980079369878008_u128 << _9.0;
_9.1 = [18240_u16,30795_u16,20800_u16,14482_u16,65130_u16];
_8 = !_3;
_3 = !_2;
_9.0 = (-96_i8) as u64;
RET = 331062132696176170756910674250539389179_u128 - 332113808584941955879816656667870452340_u128;
_4 = !_1;
_6 = _1;
_3 = _8 > _7;
Goto(bb4)
}
bb15 = {
_2 = _1;
_6 = _3;
_4 = _3 <= _2;
_8 = !_1;
_14 = (-9223372036854775808_isize);
_14 = 9223372036854775807_isize;
_3 = !_7;
_13 = [28830_u16];
_12.1 = [52831_u16,45039_u16,50504_u16,59460_u16,16781_u16];
_4 = !_2;
_9.2 = _9.0 as i16;
_9 = _12;
_10 = 2910241235_u32;
match _14 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb16 = {
_9.2 = (-17476_i16);
_6 = _1;
_9.2 = -(-30782_i16);
_9.1 = [25401_u16,475_u16,64779_u16,11849_u16,64448_u16];
RET = 235470438282300055829575909970757708332_u128 << _9.2;
_2 = !_7;
RET = 41114987770269498541535452855871122078_u128 ^ 118534290195414835493013838107644616681_u128;
RET = 89268113507758489709367809298654370829_u128;
_9.1 = [20386_u16,34987_u16,51959_u16,22875_u16,34973_u16];
_9.0 = 15010957073055995521_u64 + 2919036754855633090_u64;
match RET {
89268113507758489709367809298654370829 => bb3,
_ => bb2
}
}
bb17 = {
_9.2 = _15.2 | _12.2;
_9.2 = _15.2 ^ _15.2;
_4 = _2;
_3 = _7 == _2;
_7 = _1 & _6;
_15.0 = _12.0;
_7 = !_6;
_16 = '\u{60661}' as i32;
Goto(bb18)
}
bb18 = {
Call(_22 = dump_var(7_usize, 16_usize, Move(_16), 3_usize, Move(_3), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_22 = dump_var(7_usize, 17_usize, Move(_17), 5_usize, Move(_5), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: u32,mut _5: bool,mut _6: u32,mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: *const *mut &'static i16;
let _9: ();
let _10: ();
{
_6 = !_4;
_5 = _3 ^ _3;
_1 = _3;
_1 = _7 >= _5;
_4 = _6;
_7 = !_1;
_2 = !_1;
RET = !_5;
_2 = !_1;
_1 = !_7;
_2 = _7 & _3;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(8_usize, 3_usize, Move(_3), 7_usize, Move(_7), 2_usize, Move(_2), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u128,mut _2: u32,mut _3: u128,mut _4: bool,mut _5: bool,mut _6: [bool; 1],mut _7: bool,mut _8: u128,mut _9: bool,mut _10: [bool; 1],mut _11: [bool; 1],mut _12: bool,mut _13: [bool; 1],mut _14: bool) -> f64 {
mir! {
type RET = f64;
let _15: char;
let _16: char;
let _17: [u16; 1];
let _18: bool;
let _19: isize;
let _20: f64;
let _21: [bool; 1];
let _22: isize;
let _23: ();
let _24: ();
{
_17 = [13324_u16];
_4 = !_5;
_19 = -(-9223372036854775808_isize);
_18 = !_7;
_3 = _8;
_5 = _9;
_18 = !_9;
RET = 106841493609617904759145075495275824622_i128 as f64;
_14 = _4 >= _4;
_8 = _3 & _3;
_1 = !_3;
_11 = [_12];
_5 = _4;
RET = (-28551_i16) as f64;
_12 = _4 < _14;
RET = 159018628_i32 as f64;
_7 = _18 <= _9;
_1 = _8 | _8;
_9 = !_5;
_2 = !382847858_u32;
_15 = '\u{abde3}';
_16 = _15;
RET = 5946253812848476121_usize as f64;
_3 = _1 | _8;
_18 = !_5;
Goto(bb1)
}
bb1 = {
_17 = [48202_u16];
_13 = [_14];
_5 = _18 < _18;
Call(_1 = fn10(_14, _8, _4, _3, _5, _3, _5, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _1 >> _8;
_11 = _6;
_9 = !_5;
_9 = _5 | _7;
_2 = 3993946826_u32 * 61765893_u32;
_7 = _18;
_14 = _4;
_14 = !_9;
_3 = _1 * _8;
_21 = _11;
RET = 230_u8 as f64;
_1 = !_3;
_16 = _15;
_11 = [_5];
_20 = _1 as f64;
RET = _20 + _20;
_13 = [_9];
_22 = _19;
_22 = 661870868008958504_i64 as isize;
_16 = _15;
_6 = [_12];
_13 = [_9];
_8 = 8329717456640671587_u64 as u128;
_3 = _1 + _1;
_5 = _4 > _12;
_13 = [_14];
_19 = (-43_i8) as isize;
Goto(bb3)
}
bb3 = {
Call(_23 = dump_var(9_usize, 3_usize, Move(_3), 14_usize, Move(_14), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_23 = dump_var(9_usize, 12_usize, Move(_12), 16_usize, Move(_16), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(9_usize, 22_usize, Move(_22), 9_usize, Move(_9), 24_usize, _24, 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: bool,mut _2: u128,mut _3: bool,mut _4: u128,mut _5: bool,mut _6: u128,mut _7: bool,mut _8: [bool; 1]) -> u128 {
mir! {
type RET = u128;
let _9: isize;
let _10: i16;
let _11: *mut &'static i16;
let _12: ();
let _13: ();
{
_2 = _4 - _6;
RET = !_2;
_9 = -(-6_isize);
_8 = [_3];
RET = _6;
_4 = _2;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(10_usize, 7_usize, Move(_7), 1_usize, Move(_1), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: bool,mut _3: isize,mut _4: isize,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> bool {
mir! {
type RET = bool;
let _9: f32;
let _10: Adt38;
let _11: bool;
let _12: u32;
let _13: usize;
let _14: u32;
let _15: [u16; 5];
let _16: u32;
let _17: isize;
let _18: isize;
let _19: Adt42;
let _20: i64;
let _21: ([bool; 1], (u32,), isize);
let _22: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16));
let _23: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16);
let _24: (u32, f64, i64);
let _25: i8;
let _26: isize;
let _27: (u32,);
let _28: ();
let _29: ();
{
_7 = _8 >= _2;
RET = _2 ^ _2;
_5 = RET ^ _7;
_5 = _2;
_8 = !_5;
RET = _2;
_8 = _7;
_2 = !_7;
_6 = _7;
_10.fld4.0 = 1191610203826682867_i64 as u8;
_8 = !_6;
_8 = !_2;
_10.fld1.2 = (-1574101036607500343_i64);
_10.fld3.1 = _6 as u16;
_10.fld4.1.2.0 = '\u{468ae}';
_10.fld4.1.3 = 3220996999_u32;
_10.fld4.1.2.2 = !39404293887329505158100765196331253744_u128;
_4 = _1 & _3;
_10.fld1.0 = !_10.fld4.1.3;
_10.fld3.1 = !46832_u16;
_4 = (-21627_i16) as isize;
_1 = _10.fld1.2 as isize;
_10.fld4.1.5 = _10.fld3.1 + _10.fld3.1;
_9 = (-2014460828_i32) as f32;
_10.fld3 = (_10.fld4.1.2.0, _10.fld4.1.5, _10.fld4.1.2.2);
_2 = _7 != _6;
Goto(bb1)
}
bb1 = {
_6 = _2;
_10.fld1.1 = _3 as f64;
_10.fld4.1.1 = [_4];
_5 = !RET;
_10.fld3.1 = (-81_i8) as u16;
_12 = !_10.fld1.0;
_10.fld4.1.2 = _10.fld3;
Goto(bb2)
}
bb2 = {
_10.fld3.1 = _10.fld4.1.5;
_4 = _3 | _3;
_10.fld4.1.2.2 = _10.fld3.2;
_7 = _10.fld1.2 > _10.fld1.2;
_11 = _6;
_10.fld4.0 = 234_u8 >> _4;
_10.fld4.0 = _10.fld4.1.5 as u8;
Goto(bb3)
}
bb3 = {
_10.fld4.1.1 = [_1];
_11 = _2;
_10.fld4.1.4 = 996039733_i32 as u8;
_6 = !RET;
_10.fld2 = (-1600523111_i32) as f64;
_10.fld3.1 = _10.fld4.1.2.1;
_4 = _3;
_10.fld4.1.1 = [_4];
_18 = !_3;
_14 = _11 as u32;
_13 = 2875383296805767724_usize;
_16 = _14 / _10.fld4.1.3;
_20 = _10.fld1.2 << _14;
_10.fld3.0 = _10.fld4.1.2.0;
_10.fld4.1.5 = _10.fld3.1 * _10.fld3.1;
_18 = _10.fld3.0 as isize;
_13 = !4_usize;
_10.fld2 = -_10.fld1.1;
_4 = _1;
_10.fld3.0 = _10.fld4.1.2.0;
_10.fld4.1.2.2 = _4 as u128;
match _10.fld1.2 {
0 => bb1,
340282366920938463461800506395160711113 => bb4,
_ => bb2
}
}
bb4 = {
_12 = _14;
Call(_16 = fn12(_18, _14, _20, _3, _12, _11, _20, _8, _10.fld3.0, _10.fld3.2, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = !_8;
_10.fld1.0 = 90_i8 as u32;
_9 = _10.fld4.1.2.2 as f32;
_10.fld4.1.4 = _10.fld1.1 as u8;
_10.fld3.0 = _10.fld4.1.2.0;
_8 = _11 ^ _6;
_10.fld1.2 = _20;
_10.fld4.1.2.0 = _10.fld3.0;
_8 = _11;
_1 = -_4;
_10.fld4.1.1 = [_4];
_2 = _18 >= _1;
_10.fld4.0 = _10.fld4.1.4;
_13 = 6_usize;
_10.fld3.0 = _10.fld4.1.2.0;
_13 = 13436453974924379470_usize << _12;
_1 = !_18;
_10.fld4.1.2.2 = !_10.fld3.2;
_12 = _10.fld1.1 as u32;
_10.fld4.1.4 = _10.fld4.0 * _10.fld4.0;
_15 = [_10.fld4.1.5,_10.fld4.1.5,_10.fld3.1,_10.fld4.1.5,_10.fld4.1.5];
_12 = _11 as u32;
_18 = _3;
match _10.fld4.1.3 {
0 => bb4,
1 => bb2,
3220996999 => bb7,
_ => bb6
}
}
bb6 = {
_6 = _2;
_10.fld1.1 = _3 as f64;
_10.fld4.1.1 = [_4];
_5 = !RET;
_10.fld3.1 = (-81_i8) as u16;
_12 = !_10.fld1.0;
_10.fld4.1.2 = _10.fld3;
Goto(bb2)
}
bb7 = {
_10.fld4.1.2.0 = _10.fld3.0;
_21.0 = [_5];
_8 = _5;
_10.fld3.0 = _10.fld4.1.2.0;
RET = _2;
RET = !_5;
_11 = _6;
_23.1 = _10.fld4.1.1;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb8 = {
_6 = _2;
_10.fld1.1 = _3 as f64;
_10.fld4.1.1 = [_4];
_5 = !RET;
_10.fld3.1 = (-81_i8) as u16;
_12 = !_10.fld1.0;
_10.fld4.1.2 = _10.fld3;
Goto(bb2)
}
bb9 = {
_2 = !_8;
_10.fld1.0 = 90_i8 as u32;
_9 = _10.fld4.1.2.2 as f32;
_10.fld4.1.4 = _10.fld1.1 as u8;
_10.fld3.0 = _10.fld4.1.2.0;
_8 = _11 ^ _6;
_10.fld1.2 = _20;
_10.fld4.1.2.0 = _10.fld3.0;
_8 = _11;
_1 = -_4;
_10.fld4.1.1 = [_4];
_2 = _18 >= _1;
_10.fld4.0 = _10.fld4.1.4;
_13 = 6_usize;
_10.fld3.0 = _10.fld4.1.2.0;
_13 = 13436453974924379470_usize << _12;
_1 = !_18;
_10.fld4.1.2.2 = !_10.fld3.2;
_12 = _10.fld1.1 as u32;
_10.fld4.1.4 = _10.fld4.0 * _10.fld4.0;
_15 = [_10.fld4.1.5,_10.fld4.1.5,_10.fld3.1,_10.fld4.1.5,_10.fld4.1.5];
_12 = _11 as u32;
_18 = _3;
match _10.fld4.1.3 {
0 => bb4,
1 => bb2,
3220996999 => bb7,
_ => bb6
}
}
bb10 = {
_12 = _14;
Call(_16 = fn12(_18, _14, _20, _3, _12, _11, _20, _8, _10.fld3.0, _10.fld3.2, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_6 = _2;
_10.fld1.1 = _3 as f64;
_10.fld4.1.1 = [_4];
_5 = !RET;
_10.fld3.1 = (-81_i8) as u16;
_12 = !_10.fld1.0;
_10.fld4.1.2 = _10.fld3;
Goto(bb2)
}
bb12 = {
_10.fld3.1 = _10.fld4.1.5;
_4 = _3 | _3;
_10.fld4.1.2.2 = _10.fld3.2;
_7 = _10.fld1.2 > _10.fld1.2;
_11 = _6;
_10.fld4.0 = 234_u8 >> _4;
_10.fld4.0 = _10.fld4.1.5 as u8;
Goto(bb3)
}
bb13 = {
_23.3 = !_16;
_22.1.5 = _10.fld4.1.2.1;
_23.2.0 = _10.fld3.0;
_10.fld0 = core::ptr::addr_of_mut!(_25);
_25 = !79_i8;
_22.1.2.1 = _22.1.5 + _22.1.5;
_22.1.2 = _10.fld4.1.2;
RET = _6;
_24.1 = _10.fld2 - _10.fld1.1;
_10.fld3.0 = _22.1.2.0;
RET = _23.3 == _23.3;
_10.fld0 = core::ptr::addr_of_mut!(_25);
_10.fld4.1.2.0 = _23.2.0;
_5 = !_8;
Goto(bb14)
}
bb14 = {
Call(_28 = dump_var(11_usize, 11_usize, Move(_11), 6_usize, Move(_6), 2_usize, Move(_2), 15_usize, Move(_15)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_28 = dump_var(11_usize, 14_usize, Move(_14), 20_usize, Move(_20), 16_usize, Move(_16), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: u32,mut _3: i64,mut _4: isize,mut _5: u32,mut _6: bool,mut _7: i64,mut _8: bool,mut _9: char,mut _10: u128,mut _11: u32) -> u32 {
mir! {
type RET = u32;
let _12: (u32, f64, i64);
let _13: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16));
let _14: char;
let _15: Adt43;
let _16: isize;
let _17: i64;
let _18: usize;
let _19: (u32,);
let _20: i32;
let _21: ();
let _22: ();
{
RET = _2 << _5;
_9 = '\u{48a5e}';
_8 = _6;
_2 = _6 as u32;
_6 = _8;
_6 = RET > RET;
_12.0 = 1470_i16 as u32;
RET = _11;
_6 = !_8;
_12.2 = _3;
RET = 0_usize as u32;
_12.1 = (-1990740087_i32) as f64;
_12.1 = _7 as f64;
_13.1.5 = !45726_u16;
_12.0 = 235_u8 as u32;
_13.1.4 = _8 as u8;
_13.1.2.1 = _13.1.5 & _13.1.5;
_6 = _2 == _12.0;
_3 = _12.2;
_4 = 8789301663848397689_u64 as isize;
_13.0 = _13.1.4 + _13.1.4;
_4 = _13.1.4 as isize;
_14 = _9;
_10 = 150375566653022260877317841507679783627_u128 * 77586325814496146319630588638807942103_u128;
_13.1.1 = [_4];
_13.1.3 = _5 | _11;
Call(_13.1.1 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.1.2.0 = _14;
_12.2 = !_7;
_12.2 = _3 << _11;
RET = _13.1.3;
_9 = _13.1.2.0;
_12.2 = _13.1.5 as i64;
_12.2 = _7;
_3 = _4 as i64;
_9 = _14;
_14 = _9;
_18 = 1335129237311714999_usize;
_11 = _5;
_16 = _8 as isize;
_13.1.2 = (_14, _13.1.5, _10);
_7 = !_12.2;
_13.1.4 = _13.0;
Goto(bb2)
}
bb2 = {
Call(_21 = dump_var(12_usize, 7_usize, Move(_7), 4_usize, Move(_4), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_21 = dump_var(12_usize, 14_usize, Move(_14), 2_usize, Move(_2), 8_usize, Move(_8), 22_usize, _22), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: f64,mut _2: u32,mut _3: usize,mut _4: isize,mut _5: usize,mut _6: i64) -> bool {
mir! {
type RET = bool;
let _7: *mut i8;
let _8: Adt53;
let _9: char;
let _10: i32;
let _11: f32;
let _12: char;
let _13: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16);
let _14: i16;
let _15: (u32, f64, i64);
let _16: u32;
let _17: Adt41;
let _18: [bool; 1];
let _19: usize;
let _20: ([bool; 1], (u32,), isize);
let _21: u64;
let _22: i64;
let _23: bool;
let _24: i128;
let _25: [isize; 1];
let _26: usize;
let _27: char;
let _28: [u16; 5];
let _29: isize;
let _30: [bool; 2];
let _31: isize;
let _32: [bool; 1];
let _33: *mut &'static i16;
let _34: u16;
let _35: (u8,);
let _36: &'static i16;
let _37: ([bool; 1], (u32,), isize);
let _38: ([bool; 1], (u32,), isize);
let _39: f32;
let _40: ();
let _41: ();
{
RET = !true;
_6 = 2772758627224038163_i64 & (-869843842900122106_i64);
_3 = _5;
_5 = _3;
_4 = _5 as isize;
_3 = (-136983349730202282521799278893562274211_i128) as usize;
_1 = 188_u8 as f64;
_6 = -2757365252639741691_i64;
Call(_5 = fn14(_3, _4, _4, _4, _4, _1, _4, _4, _4, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !444239662_u32;
_5 = 5326841014766352733_u64 as usize;
_3 = 240_u8 as usize;
_1 = 139_u8 as f64;
_4 = !9223372036854775807_isize;
_3 = _5 & _5;
_9 = '\u{90332}';
_1 = (-1140637718_i32) as f64;
_6 = RET as i64;
RET = false;
RET = false | true;
_3 = _5 | _5;
_1 = 517470677_i32 as f64;
_3 = _5 & _5;
_10 = 362256292_i32 | (-1952865551_i32);
_2 = RET as u32;
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !false;
_3 = _6 as usize;
RET = !true;
_2 = 3825030403_u32 << _6;
_3 = _5;
_1 = _6 as f64;
_1 = (-80_i8) as f64;
_5 = _2 as usize;
_13.4 = !79_u8;
_11 = _1 as f32;
_13.3 = _4 as u32;
_15 = (_2, _1, _6);
_13.1 = [_4];
_15.2 = _6 * _6;
Goto(bb3)
}
bb3 = {
_13.2.0 = _9;
_13.4 = !169_u8;
_15.0 = _2;
_13.0 = core::ptr::addr_of!(_17.fld1);
_14 = (-26089_i16);
_19 = _13.2.0 as usize;
_14 = 20198_i16 ^ 32420_i16;
_17.fld0 = core::ptr::addr_of!(RET);
_10 = (-2108396336_i32);
_17.fld2 = _1 + _15.1;
match _10 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607429659815120 => bb9,
_ => bb8
}
}
bb4 = {
RET = !false;
_3 = _6 as usize;
RET = !true;
_2 = 3825030403_u32 << _6;
_3 = _5;
_1 = _6 as f64;
_1 = (-80_i8) as f64;
_5 = _2 as usize;
_13.4 = !79_u8;
_11 = _1 as f32;
_13.3 = _4 as u32;
_15 = (_2, _1, _6);
_13.1 = [_4];
_15.2 = _6 * _6;
Goto(bb3)
}
bb5 = {
_2 = !444239662_u32;
_5 = 5326841014766352733_u64 as usize;
_3 = 240_u8 as usize;
_1 = 139_u8 as f64;
_4 = !9223372036854775807_isize;
_3 = _5 & _5;
_9 = '\u{90332}';
_1 = (-1140637718_i32) as f64;
_6 = RET as i64;
RET = false;
RET = false | true;
_3 = _5 | _5;
_1 = 517470677_i32 as f64;
_3 = _5 & _5;
_10 = 362256292_i32 | (-1952865551_i32);
_2 = RET as u32;
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
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
_1 = -_17.fld2;
_17.fld0 = core::ptr::addr_of!(RET);
_5 = !_19;
_5 = _4 as usize;
RET = !false;
Call(_1 = core::intrinsics::transmute(_15.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_20.1.0 = _15.0;
_13.2.1 = !16120_u16;
_17.fld3 = _15.0 as u128;
_17.fld1 = RET as u64;
_18 = [RET];
Goto(bb11)
}
bb11 = {
_23 = _17.fld3 <= _17.fld3;
_15 = (_2, _1, _6);
_20.0 = _18;
_21 = _17.fld1 + _17.fld1;
_13.2.2 = !_17.fld3;
_13.0 = core::ptr::addr_of!(_17.fld1);
_20.1.0 = _2 & _15.0;
_6 = _15.2;
_3 = _19;
_20.1 = (_13.3,);
_12 = _13.2.0;
RET = _23;
_9 = _13.2.0;
_10 = _13.2.0 as i32;
_22 = _6;
_17.fld1 = _21 + _21;
_16 = _20.1.0;
RET = !_23;
Goto(bb12)
}
bb12 = {
RET = _23;
_13.5 = 30_i8 as u16;
_12 = _13.2.0;
_25 = [_4];
_15.2 = _13.2.2 as i64;
_13.5 = _13.2.1 << _19;
_4 = _15.0 as isize;
_13.2.0 = _9;
_20.0 = [_23];
_20.1 = (_2,);
_14 = 5636_i16 | (-28879_i16);
_32 = [_23];
_19 = !_5;
_17.fld3 = !_13.2.2;
_17.fld3 = _19 as u128;
RET = _23;
_17.fld2 = _1 * _15.1;
_17.fld2 = _15.1;
Goto(bb13)
}
bb13 = {
_13.2 = (_9, _13.5, _17.fld3);
_31 = _4;
_32 = [_23];
_13.1 = _25;
_25 = [_4];
_24 = _13.2.0 as i128;
_17.fld0 = core::ptr::addr_of!(_23);
_21 = _16 as u64;
_29 = _31;
_30 = [_23,RET];
_3 = !_5;
_17.fld2 = _15.1;
_16 = !_2;
_1 = _15.1 + _15.1;
_17.fld0 = core::ptr::addr_of!(_23);
_15.0 = _16 >> _29;
RET = !_23;
_26 = !_5;
_22 = _23 as i64;
_28 = [_13.2.1,_13.2.1,_13.5,_13.5,_13.5];
_26 = !_3;
_24 = _14 as i128;
_17.fld2 = _15.1 - _1;
Goto(bb14)
}
bb14 = {
_30 = [RET,RET];
_37 = (_20.0, _20.1, _4);
_31 = _15.1 as isize;
_14 = _24 as i16;
_12 = _13.2.0;
_20.1.0 = _15.0 & _16;
_20.0 = _37.0;
_25 = _13.1;
_13.3 = _16;
_37.0 = [_23];
_15.0 = _13.4 as u32;
_10 = !2087706554_i32;
_5 = _19;
_33 = core::ptr::addr_of_mut!(_36);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(13_usize, 23_usize, Move(_23), 2_usize, Move(_2), 18_usize, Move(_18), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(13_usize, 22_usize, Move(_22), 28_usize, Move(_28), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(13_usize, 21_usize, Move(_21), 4_usize, Move(_4), 14_usize, Move(_14), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: usize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: f64,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: u32) -> usize {
mir! {
type RET = usize;
let _11: usize;
let _12: f32;
let _13: [u16; 1];
let _14: Adt54;
let _15: f32;
let _16: [u16; 1];
let _17: Adt49;
let _18: ();
let _19: ();
{
_5 = !_3;
RET = _1 << _1;
_4 = !_7;
_8 = _4 + _2;
_6 = _1 as f64;
_2 = _8 * _4;
_3 = -_8;
_1 = RET;
_1 = !RET;
_7 = _4 & _8;
_10 = !2184803404_u32;
_7 = !_2;
_7 = '\u{d433f}' as isize;
_5 = _2 & _3;
_9 = _5;
_9 = _5;
_6 = 16604762778534076502_u64 as f64;
_2 = _6 as isize;
_10 = 3614074021_u32 * 119661443_u32;
RET = !_1;
_6 = 5690087261835159739_i64 as f64;
_7 = 718434957_i32 as isize;
_5 = _3;
_2 = !_9;
_1 = _6 as usize;
_2 = !_9;
_10 = !3380100116_u32;
_5 = _3;
_2 = -_9;
Call(_9 = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _2;
_4 = -_7;
Goto(bb2)
}
bb2 = {
_11 = _1 - RET;
_4 = _7;
_8 = !_5;
_9 = _4;
Goto(bb3)
}
bb3 = {
RET = _11;
Goto(bb4)
}
bb4 = {
_4 = -_9;
_1 = _11 * _11;
_9 = _7 - _8;
_4 = _7 + _3;
_6 = _1 as f64;
_9 = _7;
_7 = 2085265876016506196_u64 as isize;
_12 = _6 as f32;
_10 = 129_u8 as u32;
_8 = _2 + _4;
_5 = !_8;
_2 = !_4;
_6 = _10 as f64;
_11 = RET << _8;
_5 = _4 - _8;
_9 = _12 as isize;
_1 = _11 + _11;
_8 = 60379_u16 as isize;
_5 = 150_u8 as isize;
RET = _1 << _11;
_6 = _12 as f64;
_6 = _2 as f64;
_13 = [751_u16];
_14 = Adt54::Variant1 { fld0: (-60433501603654352912835127141740623916_i128) };
Goto(bb5)
}
bb5 = {
Call(_18 = dump_var(14_usize, 7_usize, Move(_7), 5_usize, Move(_5), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_18 = dump_var(14_usize, 11_usize, Move(_11), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u128,mut _2: [bool; 1],mut _3: [bool; 1],mut _4: [bool; 1],mut _5: bool,mut _6: u128,mut _7: bool,mut _8: i16) -> u32 {
mir! {
type RET = u32;
let _9: i8;
let _10: Adt43;
let _11: (char, u16, u128);
let _12: (char, u16, u128);
let _13: u8;
let _14: bool;
let _15: Adt44;
let _16: f32;
let _17: [u16; 5];
let _18: i128;
let _19: isize;
let _20: isize;
let _21: usize;
let _22: u128;
let _23: ([bool; 1], (u32,), isize);
let _24: i32;
let _25: u64;
let _26: isize;
let _27: (char, u16, u128);
let _28: Adt51;
let _29: [isize; 1];
let _30: i16;
let _31: (u32,);
let _32: Adt54;
let _33: u128;
let _34: Adt54;
let _35: ();
let _36: ();
{
_4 = [_5];
_4 = _3;
RET = '\u{dce3f}' as u32;
_7 = !_5;
_8 = (-10956_i16) >> _6;
_6 = 14312_u16 as u128;
Goto(bb1)
}
bb1 = {
_2 = [_5];
_3 = _2;
_2 = _4;
_6 = _1 << _8;
_8 = !21601_i16;
_1 = _6 >> _6;
_4 = [_7];
RET = !2070150420_u32;
_11.2 = RET as u128;
_2 = [_7];
_11.2 = _6 ^ _1;
RET = !881047339_u32;
RET = 1569583721_u32 | 1496959799_u32;
_1 = 130424078472721151419843347419737481456_i128 as u128;
_11 = ('\u{7a78a}', 55070_u16, _6);
_12.0 = _11.0;
_12.0 = _11.0;
match _11.1 {
55070 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_9 = _8 as i8;
_2 = [_7];
_3 = [_5];
_15.fld4 = 2406301501013147194_u64 as i128;
_15.fld0.fld1 = 2020527902214185331_u64;
_12.1 = _11.1 % _11.1;
_9 = 64_i8;
RET = 1668884481_u32;
_15.fld2.0 = RET;
_2 = _4;
_2 = [_5];
_15.fld2.0 = RET;
_12 = (_11.0, _11.1, _6);
_12.1 = _11.1 % _11.1;
RET = !_15.fld2.0;
_15.fld2 = (RET,);
_12 = (_11.0, _11.1, _11.2);
_12.1 = _1 as u16;
_11.0 = _12.0;
_13 = 140_u8;
_14 = _15.fld0.fld1 <= _15.fld0.fld1;
_15.fld5 = 754415631_i32;
_4 = [_5];
Goto(bb4)
}
bb4 = {
match _13 {
0 => bb3,
140 => bb5,
_ => bb2
}
}
bb5 = {
_12.1 = _11.1 ^ _11.1;
_12.1 = _11.1;
_14 = !_5;
_17 = [_12.1,_11.1,_12.1,_12.1,_12.1];
_21 = 1_usize - 3_usize;
_12.1 = 1520078942789644962_i64 as u16;
_15.fld1 = _11.0;
_21 = !7_usize;
_18 = !_15.fld4;
_2 = _4;
_15.fld0.fld0 = core::ptr::addr_of!(_14);
_23.0 = [_7];
_15.fld0.fld4 = core::ptr::addr_of_mut!(_9);
_11.2 = _13 as u128;
_15.fld0.fld2 = _15.fld5 as f64;
_12 = (_15.fld1, _11.1, _6);
_23 = (_2, _15.fld2, (-9223372036854775808_isize));
_23.1.0 = RET;
Call(_5 = fn16(_23, _23.2, _23.2, _15.fld0.fld0, _14, _23, _23.2, _23.2, _7, _23.0, _11.1, _23.2, _14), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6 = _12.2;
Goto(bb7)
}
bb7 = {
_15.fld6 = [_12.1,_11.1,_12.1,_11.1,_12.1];
_26 = _12.0 as isize;
_11 = _12;
_23.1 = (RET,);
_27 = _12;
_15.fld0.fld3 = _11.2;
_25 = _12.2 as u64;
_17 = [_27.1,_11.1,_27.1,_27.1,_11.1];
_3 = [_14];
Goto(bb8)
}
bb8 = {
_8 = 24980_i16 - (-19147_i16);
_24 = _21 as i32;
_15.fld0.fld1 = !_25;
_3 = _4;
_11.1 = _27.1;
Goto(bb9)
}
bb9 = {
_27.0 = _12.0;
_24 = _15.fld5 - _15.fld5;
_20 = (-5258536536971699630_i64) as isize;
_18 = !_15.fld4;
match _12.1 {
0 => bb3,
1 => bb8,
2 => bb10,
3 => bb11,
4 => bb12,
55070 => bb14,
_ => bb13
}
}
bb10 = {
_8 = 24980_i16 - (-19147_i16);
_24 = _21 as i32;
_15.fld0.fld1 = !_25;
_3 = _4;
_11.1 = _27.1;
Goto(bb9)
}
bb11 = {
_15.fld6 = [_12.1,_11.1,_12.1,_11.1,_12.1];
_26 = _12.0 as isize;
_11 = _12;
_23.1 = (RET,);
_27 = _12;
_15.fld0.fld3 = _11.2;
_25 = _12.2 as u64;
_17 = [_27.1,_11.1,_27.1,_27.1,_11.1];
_3 = [_14];
Goto(bb8)
}
bb12 = {
match _13 {
0 => bb3,
140 => bb5,
_ => bb2
}
}
bb13 = {
_12.1 = _11.1 ^ _11.1;
_12.1 = _11.1;
_14 = !_5;
_17 = [_12.1,_11.1,_12.1,_12.1,_12.1];
_21 = 1_usize - 3_usize;
_12.1 = 1520078942789644962_i64 as u16;
_15.fld1 = _11.0;
_21 = !7_usize;
_18 = !_15.fld4;
_2 = _4;
_15.fld0.fld0 = core::ptr::addr_of!(_14);
_23.0 = [_7];
_15.fld0.fld4 = core::ptr::addr_of_mut!(_9);
_11.2 = _13 as u128;
_15.fld0.fld2 = _15.fld5 as f64;
_12 = (_15.fld1, _11.1, _6);
_23 = (_2, _15.fld2, (-9223372036854775808_isize));
_23.1.0 = RET;
Call(_5 = fn16(_23, _23.2, _23.2, _15.fld0.fld0, _14, _23, _23.2, _23.2, _7, _23.0, _11.1, _23.2, _14), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_18 = _15.fld4;
_19 = _23.2;
_19 = _23.2;
_15.fld2 = _23.1;
_8 = (-25846_i16);
_15.fld4 = _15.fld0.fld2 as i128;
_19 = _26;
_32 = Adt54::Variant1 { fld0: _15.fld4 };
_26 = _23.2 & _23.2;
_11.2 = _14 as u128;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(15_usize, 5_usize, Move(_5), 20_usize, Move(_20), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(15_usize, 27_usize, Move(_27), 26_usize, Move(_26), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(15_usize, 21_usize, Move(_21), 1_usize, Move(_1), 11_usize, Move(_11), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: ([bool; 1], (u32,), isize),mut _2: isize,mut _3: isize,mut _4: *const bool,mut _5: bool,mut _6: ([bool; 1], (u32,), isize),mut _7: isize,mut _8: isize,mut _9: bool,mut _10: [bool; 1],mut _11: u16,mut _12: isize,mut _13: bool) -> bool {
mir! {
type RET = bool;
let _14: ([bool; 1], (u32,), isize);
let _15: f64;
let _16: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16);
let _17: Adt51;
let _18: f32;
let _19: [bool; 1];
let _20: isize;
let _21: char;
let _22: char;
let _23: Adt47;
let _24: i32;
let _25: u64;
let _26: ();
let _27: ();
{
_4 = core::ptr::addr_of!(_5);
(*_4) = _9 | _9;
_6.1.0 = _1.1.0 | _1.1.0;
_1 = (_10, _6.1, _7);
_1 = (_6.0, _6.1, _6.2);
_8 = _2 & _1.2;
_6 = _1;
_1.0 = [(*_4)];
_11 = !53939_u16;
Goto(bb1)
}
bb1 = {
_14.0 = [(*_4)];
_12 = _2 - _8;
_13 = (*_4);
_14.1.0 = _1.1.0 ^ _6.1.0;
_11 = 35602_u16;
_15 = 8283700287016688245_u64 as f64;
(*_4) = _9;
_16.4 = 191_u8 | 183_u8;
_3 = _8;
_6.0 = _14.0;
match _6.2 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463454151235394913435648 => bb6,
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
_1 = (_14.0, _14.1, _12);
_16.5 = !_11;
_15 = _11 as f64;
_3 = !_1.2;
_19 = _1.0;
_12 = _6.2;
_8 = -_1.2;
_1.1.0 = (-7584109756275552010_i64) as u32;
_9 = !_13;
_14.2 = _3;
_16.2.0 = '\u{b0b4}';
_10 = [(*_4)];
_18 = (-9_i8) as f32;
_14.1 = (_1.1.0,);
_19 = [_9];
_1.1 = (_6.1.0,);
_16.3 = !_1.1.0;
Goto(bb7)
}
bb7 = {
(*_4) = _9;
_19 = [_9];
_16.1 = [_1.2];
_1.1.0 = !_6.1.0;
(*_4) = _9;
_22 = _16.2.0;
_9 = (*_4);
_1 = (_19, _6.1, _8);
RET = !_5;
_14.0 = _19;
_2 = _14.2 * _3;
_16.1 = [_2];
_9 = !(*_4);
_20 = !_2;
_14.0 = [_9];
_3 = _12 * _1.2;
Goto(bb8)
}
bb8 = {
Call(_26 = dump_var(16_usize, 1_usize, Move(_1), 3_usize, Move(_3), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_26 = dump_var(16_usize, 13_usize, Move(_13), 8_usize, Move(_8), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(4579015033643451468151195044910334231_u128), std::hint::black_box(121_isize), std::hint::black_box((-32_i8)), std::hint::black_box(15160_i16), std::hint::black_box((-1140652144_i32)), std::hint::black_box((-7219273926488034912_i64)), std::hint::black_box(50055855897163454719139979425499131449_i128), std::hint::black_box(3_usize), std::hint::black_box(140_u8), std::hint::black_box(53848_u16), std::hint::black_box(3485004157_u32), std::hint::black_box(5036468186402753249_u64));
                
            }
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: *mut i8,
fld1: (u32, f64, i64),
fld2: f64,
fld3: (char, u16, u128),
fld4: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),
}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: u8,
fld1: Adt38,
fld2: *const bool,
fld3: [bool; 1],
fld4: f64,
fld5: u128,
fld6: i64,

},
Variant1{
fld0: (u32, f64, i64),
fld1: [bool; 1],
fld2: [bool; 2],
fld3: i8,
fld4: ([bool; 1], (u32,), isize),
fld5: (u8,),
fld6: Adt38,
fld7: [u16; 5],

},
Variant2{
fld0: bool,
fld1: (char, u16, u128),
fld2: isize,
fld3: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16),
fld4: (u64, [u16; 5], i16),
fld5: Adt38,
fld6: u8,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: u128,
fld1: u16,
fld2: *const bool,
}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: *const bool,
fld1: u64,
fld2: f64,
fld3: u128,
fld4: *mut i8,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (char, u16, u128),
fld1: char,
fld2: (bool, u16, i64, *const u64),
fld3: i8,
fld4: (u32, f64, i64),
fld5: Adt38,

},
Variant1{
fld0: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),
fld1: Adt41,
fld2: f64,
fld3: f32,
fld4: (bool, u16, i64, *const u64),
fld5: *const bool,

},
Variant2{
fld0: u16,
fld1: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16),
fld2: u128,
fld3: [isize; 1],
fld4: u8,
fld5: *const u64,
fld6: i64,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: f64,
fld2: [isize; 1],
fld3: u8,
fld4: [bool; 2],
fld5: (u32, f64, i64),
fld6: i64,

},
Variant1{
fld0: *mut u8,
fld1: (bool, u16, i64, *const u64),
fld2: u32,
fld3: (u64, [u16; 5], i16),
fld4: u128,
fld5: [u16; 5],

},
Variant2{
fld0: u64,
fld1: i8,
fld2: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: Adt41,
fld1: char,
fld2: (u32,),
fld3: Adt43,
fld4: i128,
fld5: i32,
fld6: [u16; 5],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u32,
fld1: [bool; 2],

},
Variant1{
fld0: Adt42,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: u32,
fld2: Adt39,
fld3: Adt44,

},
Variant1{
fld0: (u64, [u16; 5], i16),
fld1: [bool; 2],
fld2: Adt43,
fld3: u8,

},
Variant2{
fld0: (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16),
fld1: usize,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u32,
fld1: [bool; 2],
fld2: i128,
fld3: [u16; 5],
fld4: (bool, u16, i64, *const u64),
fld5: i32,

},
Variant1{
fld0: u64,
fld1: Adt42,
fld2: isize,
fld3: i8,
fld4: [bool; 2],
fld5: (char, u16, u128),
fld6: *const bool,

},
Variant2{
fld0: (u64, [u16; 5], i16),
fld1: Adt39,

},
Variant3{
fld0: Adt42,
fld1: *const u64,

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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [isize; 1],
fld1: (char, u16, u128),
fld2: f64,
fld3: i8,
fld4: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),

},
Variant1{
fld0: Adt43,
fld1: f32,
fld2: isize,
fld3: Adt42,
fld4: [isize; 1],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt40,
fld1: (u8,),
fld2: *const bool,
fld3: u8,
fld4: i16,
fld5: i32,
fld6: (char, u16, u128),

},
Variant1{
fld0: Adt39,
fld1: char,
fld2: Adt38,
fld3: i8,
fld4: f64,

},
Variant2{
fld0: *mut i8,
fld1: i32,
fld2: i64,
fld3: *const u64,
fld4: u8,

},
Variant3{
fld0: *mut u8,
fld1: f64,
fld2: Adt48,
fld3: i8,
fld4: i16,
fld5: Adt40,
fld6: i64,
fld7: Adt45,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f32,
fld1: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),
fld2: u8,
fld3: u16,
fld4: (u32, f64, i64),

},
Variant1{
fld0: Adt48,
fld1: *const bool,
fld2: (u32, f64, i64),

},
Variant2{
fld0: Adt47,
fld1: char,
fld2: (u32, f64, i64),
fld3: i128,
fld4: u16,

},
Variant3{
fld0: *const bool,
fld1: [u16; 1],

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: f32,
fld1: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),
fld2: isize,
fld3: i16,

},
Variant1{
fld0: u64,
fld1: *const u64,
fld2: isize,
fld3: i8,
fld4: (char, u16, u128),
fld5: *mut i8,
fld6: *const bool,
fld7: (u64, [u16; 5], i16),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt38,

},
Variant1{
fld0: [bool; 1],
fld1: char,
fld2: isize,
fld3: i8,
fld4: Adt49,
fld5: *mut i8,
fld6: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),
fld7: (char, u16, u128),

},
Variant2{
fld0: bool,
fld1: [u16; 5],
fld2: Adt49,
fld3: (u32, f64, i64),
fld4: Adt43,
fld5: [bool; 2],
fld6: (char, u16, u128),
fld7: [isize; 1],

},
Variant3{
fld0: bool,
fld1: u16,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
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
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u16; 5],
fld1: (bool, u16, i64, *const u64),
fld2: [bool; 2],
fld3: Adt40,
fld4: i16,
fld5: Adt39,
fld6: Adt48,
fld7: Adt50,

},
Variant1{
fld0: u64,
fld1: (u8, (*const u64, [isize; 1], (char, u16, u128), u32, u8, u16)),
fld2: Adt45,

},
Variant2{
fld0: i8,
fld1: Adt46,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: (char, u16, u128),

},
Variant1{
fld0: i128,

}}

