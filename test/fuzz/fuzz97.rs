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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32) -> *mut [isize; 7] {
mir! {
type RET = *mut [isize; 7];
let _13: (&'static &'static usize, i8, (isize, i32), u8);
let _14: f64;
let _15: &'static [isize; 3];
let _16: i128;
let _17: (i32, bool, u32, i64);
let _18: *mut Adt19;
let _19: &'static &'static usize;
let _20: [u8; 4];
let _21: [i32; 1];
let _22: f32;
let _23: ((&'static &'static usize, i8, (isize, i32), u8), [char; 5], bool, *const *const u8);
let _24: Adt54;
let _25: *mut *const u8;
let _26: &'static &'static [isize; 3];
let _27: f64;
let _28: *mut *const u8;
let _29: *const *const u8;
let _30: &'static f64;
let _31: &'static usize;
let _32: *mut u16;
let _33: [i128; 8];
let _34: (&'static &'static usize, i8, (isize, i32), u8);
let _35: u64;
let _36: ((&'static &'static usize, i8, (isize, i32), u8), [char; 5], bool, *const *const u8);
let _37: char;
let _38: *const &'static Adt46;
let _39: f32;
let _40: *mut f64;
let _41: Adt52;
let _42: [i128; 4];
let _43: usize;
let _44: [char; 5];
let _45: u128;
let _46: u32;
let _47: i32;
let _48: &'static usize;
let _49: i16;
let _50: ([i16; 6],);
let _51: [i32; 1];
let _52: *const usize;
let _53: bool;
let _54: (f32, [i16; 6], i16, i16);
let _55: char;
let _56: isize;
let _57: *mut u16;
let _58: bool;
let _59: u8;
let _60: f32;
let _61: [u8; 4];
let _62: (&'static [i32; 1],);
let _63: ([i16; 6],);
let _64: *mut [isize; 7];
let _65: (i8, f32, f64);
let _66: char;
let _67: isize;
let _68: isize;
let _69: f64;
let _70: &'static f64;
let _71: i32;
let _72: (isize, i32);
let _73: char;
let _74: &'static i128;
let _75: i32;
let _76: (i8, f32, f64);
let _77: f32;
let _78: i32;
let _79: u64;
let _80: &'static [u128; 1];
let _81: [isize; 7];
let _82: [i128; 4];
let _83: isize;
let _84: (i8, f32, f64);
let _85: char;
let _86: [u8; 4];
let _87: u8;
let _88: i128;
let _89: isize;
let _90: f64;
let _91: &'static [isize; 7];
let _92: &'static f64;
let _93: *const u8;
let _94: ();
let _95: ();
{
_7 = 8643354703809785873_u64 << (-90633510376856576246788084535523432808_i128);
_2 = (-88_isize) as u128;
_12 = !4215116959_u32;
_1 = !true;
_5 = _7 as i16;
_5 = (-19706_i16) - (-25487_i16);
_4 = 43_i8 | 53_i8;
_5 = 0_usize as i16;
_13.3 = _2 as u8;
_4 = 105_i8;
_13.2.0 = -9223372036854775807_isize;
_13.1 = !_4;
_7 = 12508666120376000636_u64;
_13.1 = _4 - _4;
_13.2.1 = 167160169806181897486452927787228258840_i128 as i32;
_7 = 4786032752990230907_u64 << _5;
_8 = (-97199869280729364933183286117793977205_i128);
_13.2 = ((-46_isize), 29421777_i32);
_5 = 14334_i16;
_17.3 = (-6397010905856072525_i64) + (-8467657356738848186_i64);
_3 = _13.2.0;
_13.3 = !129_u8;
_11 = !42429_u16;
Call(_9 = fn1(_7, _13.1, _3, _11, _13.2, _8, _3, _5, _13.2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _13.2.1 as usize;
_4 = _13.1 >> _13.2.0;
_12 = 2795964907_u32;
_13.3 = 254_u8 & 149_u8;
_16 = !_8;
_20 = [_13.3,_13.3,_13.3,_13.3];
_5 = 3250_i16;
_2 = !299306815622922433948521066207694783537_u128;
_2 = !83328564056331510728521952775911283621_u128;
_6 = _3 as i32;
_13.2.0 = _1 as isize;
_13.2.1 = !_6;
_22 = _16 as f32;
_17.0 = _6 << _5;
_20 = [_13.3,_13.3,_13.3,_13.3];
Goto(bb2)
}
bb2 = {
_17 = (_6, _1, _12, 7372657433507967876_i64);
_1 = !_17.1;
_11 = 17816_u16;
_4 = _13.1;
_14 = _3 as f64;
_23.0.3 = !_13.3;
_2 = !280740923723440219366277802553743125923_u128;
_6 = _13.2.1;
_23.0.2.0 = _5 as isize;
_23.1 = ['\u{9912e}','\u{5abc7}','\u{8ffd6}','\u{5e77c}','\u{73743}'];
Call(_23.0.1 = fn3(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_23.0.2 = (_3, _17.0);
_23.0.1 = _4 ^ _4;
_21 = [_17.0];
_9 = !977615764393216356_usize;
_16 = _8;
_17.3 = (-2664330392987237849_i64);
_12 = !_17.2;
_23.2 = _1;
_6 = -_17.0;
_13.2.0 = _23.0.2.0;
_9 = 5_usize;
_26 = &_15;
_26 = &(*_26);
_8 = _16 ^ _16;
_27 = -_14;
_8 = _16;
_23.0.2 = (_3, _6);
match _5 {
0 => bb4,
3250 => bb6,
_ => bb5
}
}
bb4 = {
_17 = (_6, _1, _12, 7372657433507967876_i64);
_1 = !_17.1;
_11 = 17816_u16;
_4 = _13.1;
_14 = _3 as f64;
_23.0.3 = !_13.3;
_2 = !280740923723440219366277802553743125923_u128;
_6 = _13.2.1;
_23.0.2.0 = _5 as isize;
_23.1 = ['\u{9912e}','\u{5abc7}','\u{8ffd6}','\u{5e77c}','\u{73743}'];
Call(_23.0.1 = fn3(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_9 = _13.2.1 as usize;
_4 = _13.1 >> _13.2.0;
_12 = 2795964907_u32;
_13.3 = 254_u8 & 149_u8;
_16 = !_8;
_20 = [_13.3,_13.3,_13.3,_13.3];
_5 = 3250_i16;
_2 = !299306815622922433948521066207694783537_u128;
_2 = !83328564056331510728521952775911283621_u128;
_6 = _3 as i32;
_13.2.0 = _1 as isize;
_13.2.1 = !_6;
_22 = _16 as f32;
_17.0 = _6 << _5;
_20 = [_13.3,_13.3,_13.3,_13.3];
Goto(bb2)
}
bb6 = {
_23.0.3 = !_13.3;
_3 = _27 as isize;
_4 = _23.0.1;
_17.2 = !_12;
_23.0.2 = (_3, _13.2.1);
Goto(bb7)
}
bb7 = {
_23.0.2 = (_13.2.0, _13.2.1);
_10 = !_13.3;
_17.0 = -_13.2.1;
_10 = _2 as u8;
_13.2.1 = _17.0 + _6;
_17.0 = -_23.0.2.1;
_7 = 6527369212963284988_u64;
Goto(bb8)
}
bb8 = {
_22 = _5 as f32;
_8 = _17.1 as i128;
_11 = _27 as u16;
_13.3 = _23.0.3;
_26 = &(*_26);
_17.0 = _13.2.1;
_20 = [_13.3,_13.3,_10,_13.3];
_7 = 4700156282244875052_u64 - 3777355852408376168_u64;
_23.2 = _13.2.1 >= _13.2.1;
_23.0.2 = (_3, _13.2.1);
_23.0.3 = _13.3;
_13.2.1 = _6 << _17.0;
_20 = [_23.0.3,_10,_10,_10];
Goto(bb9)
}
bb9 = {
_17.1 = _4 < _4;
_6 = _23.0.2.1;
_17.3 = 1471097427872379690_i64;
Goto(bb10)
}
bb10 = {
_4 = _17.3 as i8;
_22 = _23.0.2.1 as f32;
_14 = -_27;
_23.2 = _14 == _27;
_17 = (_13.2.1, _23.2, _12, (-8030104250308180528_i64));
_13.2 = (_3, _17.0);
_3 = _13.2.0;
_13.2.0 = _11 as isize;
_9 = 9321855581843588102_usize;
_10 = !_13.3;
_23.0.2.0 = _11 as isize;
_14 = _23.0.1 as f64;
_10 = _13.3 >> _4;
_16 = _8;
_22 = _7 as f32;
_23.0.0 = &_31;
_13 = Move(_23.0);
_10 = _12 as u8;
_13.2.0 = _3 | _3;
_34.0 = &_31;
_20 = [_13.3,_13.3,_13.3,_13.3];
_9 = 9388831171853564857_usize | 2328818086597147715_usize;
_34.2.0 = _7 as isize;
Goto(bb11)
}
bb11 = {
_23.0.3 = _13.3;
_2 = !219510038433903986850847822491197615537_u128;
_3 = -_13.2.0;
_36.0.0 = &_31;
_23.1 = ['\u{7f5c3}','\u{ebe23}','\u{74c4c}','\u{ca031}','\u{cd2d5}'];
_23.0.0 = &_31;
_31 = &_9;
_32 = core::ptr::addr_of_mut!(_11);
_17.3 = -(-7211149697546486198_i64);
_34.1 = -_13.1;
_36.0.3 = _13.3 * _13.3;
_34.3 = _2 as u8;
_36.1 = ['\u{f99ea}','\u{9e6af}','\u{10d44b}','\u{26c00}','\u{2307e}'];
_17.0 = _16 as i32;
_14 = -_27;
_7 = _14 as u64;
_36.0.0 = &_31;
_23.0.1 = (*_31) as i8;
_7 = _13.2.0 as u64;
_34.0 = Move(_36.0.0);
_17.0 = !_13.2.1;
match _5 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb12,
3250 => bb14,
_ => bb13
}
}
bb12 = {
_9 = _13.2.1 as usize;
_4 = _13.1 >> _13.2.0;
_12 = 2795964907_u32;
_13.3 = 254_u8 & 149_u8;
_16 = !_8;
_20 = [_13.3,_13.3,_13.3,_13.3];
_5 = 3250_i16;
_2 = !299306815622922433948521066207694783537_u128;
_2 = !83328564056331510728521952775911283621_u128;
_6 = _3 as i32;
_13.2.0 = _1 as isize;
_13.2.1 = !_6;
_22 = _16 as f32;
_17.0 = _6 << _5;
_20 = [_13.3,_13.3,_13.3,_13.3];
Goto(bb2)
}
bb13 = {
_17.1 = _4 < _4;
_6 = _23.0.2.1;
_17.3 = 1471097427872379690_i64;
Goto(bb10)
}
bb14 = {
_36.0.1 = _11 as i8;
_10 = _13.3 - _23.0.3;
_23.0.2 = _13.2;
_23.0.2.0 = !_34.2.0;
_36.0.2.0 = !_3;
_34.2.1 = _13.2.1;
_13.1 = !_36.0.1;
_19 = &_31;
_34.2.1 = _6;
_13.3 = _16 as u8;
_33 = [_16,_16,_16,_8,_16,_16,_16,_8];
_23.0.2 = _13.2;
_20 = [_36.0.3,_34.3,_36.0.3,_13.3];
_26 = &(*_26);
_9 = !5635984190368146580_usize;
_34 = (Move(_19), _13.1, _23.0.2, _10);
_13.1 = _27 as i8;
_13.1 = _36.0.1;
_31 = &_9;
_23.0.1 = _34.1 + _36.0.1;
_34.2.0 = _36.0.2.0 | _13.2.0;
_36.0.2.1 = _23.0.2.1;
_34.0 = &_31;
_37 = '\u{105932}';
_30 = &_27;
Goto(bb15)
}
bb15 = {
_34.2.1 = _23.0.2.1;
_23.0 = (Move(_34.0), _13.1, _34.2, _13.3);
_13.2.0 = _3;
_23.0.1 = _36.0.1 >> _23.0.2.1;
_11 = 2209_u16;
_34.1 = _13.1 << _23.0.2.0;
_13.1 = _23.0.1 + _4;
_12 = _17.2 + _17.2;
_21 = [_23.0.2.1];
_9 = 6_usize * 5_usize;
_17.1 = !_23.2;
_30 = &_27;
_17.1 = _23.2;
_17.0 = -_36.0.2.1;
_23.1 = [_37,_37,_37,_37,_37];
_5 = (-23621_i16) + (-397_i16);
_36.0.1 = _2 as i8;
_23.0.2.0 = _34.2.0;
_35 = !_7;
Goto(bb16)
}
bb16 = {
_48 = &_43;
_23.1 = [_37,_37,_37,_37,_37];
_17 = (_34.2.1, _23.2, _12, (-3849943667700070900_i64));
_13.2 = (_23.0.2.0, _17.0);
_36.0.0 = &_31;
_36.0.1 = _4;
_13 = (Move(_36.0.0), _23.0.1, _23.0.2, _10);
_5 = 11749_i16;
_19 = &_31;
_36.0.2.0 = _23.0.2.0 * _3;
_23.0 = (Move(_19), _34.1, _36.0.2, _13.3);
_17.2 = _23.0.1 as u32;
match _17.3 {
0 => bb10,
1 => bb5,
2 => bb3,
340282366920938463459524663764068140556 => bb18,
_ => bb17
}
}
bb17 = {
_17.1 = _4 < _4;
_6 = _23.0.2.1;
_17.3 = 1471097427872379690_i64;
Goto(bb10)
}
bb18 = {
_46 = _17.2 | _12;
_9 = _5 as usize;
_17.3 = 1150631766460297539_i64 & 327325320240124209_i64;
_23.0.0 = &_31;
_19 = &_31;
_47 = _17.0 + _17.0;
Goto(bb19)
}
bb19 = {
_23.2 = _17.1;
_35 = _7;
_34.0 = &(*_19);
_23.0.2 = (_36.0.2.0, _17.0);
_8 = _13.1 as i128;
_32 = core::ptr::addr_of_mut!(_11);
_34.1 = _36.0.1 & _23.0.1;
_44 = [_37,_37,_37,_37,_37];
_48 = &_43;
_51 = [_13.2.1];
_40 = core::ptr::addr_of_mut!((*_30));
_35 = _7;
_13 = (Move(_34.0), _34.1, _36.0.2, _23.0.3);
_35 = _7 << _46;
_43 = _2 as usize;
_2 = _5 as u128;
(*_32) = 59817_u16;
match _11 {
0 => bb6,
1 => bb4,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
6 => bb24,
59817 => bb26,
_ => bb25
}
}
bb20 = {
_46 = _17.2 | _12;
_9 = _5 as usize;
_17.3 = 1150631766460297539_i64 & 327325320240124209_i64;
_23.0.0 = &_31;
_19 = &_31;
_47 = _17.0 + _17.0;
Goto(bb19)
}
bb21 = {
_9 = _13.2.1 as usize;
_4 = _13.1 >> _13.2.0;
_12 = 2795964907_u32;
_13.3 = 254_u8 & 149_u8;
_16 = !_8;
_20 = [_13.3,_13.3,_13.3,_13.3];
_5 = 3250_i16;
_2 = !299306815622922433948521066207694783537_u128;
_2 = !83328564056331510728521952775911283621_u128;
_6 = _3 as i32;
_13.2.0 = _1 as isize;
_13.2.1 = !_6;
_22 = _16 as f32;
_17.0 = _6 << _5;
_20 = [_13.3,_13.3,_13.3,_13.3];
Goto(bb2)
}
bb22 = {
_48 = &_43;
_23.1 = [_37,_37,_37,_37,_37];
_17 = (_34.2.1, _23.2, _12, (-3849943667700070900_i64));
_13.2 = (_23.0.2.0, _17.0);
_36.0.0 = &_31;
_36.0.1 = _4;
_13 = (Move(_36.0.0), _23.0.1, _23.0.2, _10);
_5 = 11749_i16;
_19 = &_31;
_36.0.2.0 = _23.0.2.0 * _3;
_23.0 = (Move(_19), _34.1, _36.0.2, _13.3);
_17.2 = _23.0.1 as u32;
match _17.3 {
0 => bb10,
1 => bb5,
2 => bb3,
340282366920938463459524663764068140556 => bb18,
_ => bb17
}
}
bb23 = {
_17 = (_6, _1, _12, 7372657433507967876_i64);
_1 = !_17.1;
_11 = 17816_u16;
_4 = _13.1;
_14 = _3 as f64;
_23.0.3 = !_13.3;
_2 = !280740923723440219366277802553743125923_u128;
_6 = _13.2.1;
_23.0.2.0 = _5 as isize;
_23.1 = ['\u{9912e}','\u{5abc7}','\u{8ffd6}','\u{5e77c}','\u{73743}'];
Call(_23.0.1 = fn3(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb24 = {
_23.0.3 = _13.3;
_2 = !219510038433903986850847822491197615537_u128;
_3 = -_13.2.0;
_36.0.0 = &_31;
_23.1 = ['\u{7f5c3}','\u{ebe23}','\u{74c4c}','\u{ca031}','\u{cd2d5}'];
_23.0.0 = &_31;
_31 = &_9;
_32 = core::ptr::addr_of_mut!(_11);
_17.3 = -(-7211149697546486198_i64);
_34.1 = -_13.1;
_36.0.3 = _13.3 * _13.3;
_34.3 = _2 as u8;
_36.1 = ['\u{f99ea}','\u{9e6af}','\u{10d44b}','\u{26c00}','\u{2307e}'];
_17.0 = _16 as i32;
_14 = -_27;
_7 = _14 as u64;
_36.0.0 = &_31;
_23.0.1 = (*_31) as i8;
_7 = _13.2.0 as u64;
_34.0 = Move(_36.0.0);
_17.0 = !_13.2.1;
match _5 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb12,
3250 => bb14,
_ => bb13
}
}
bb25 = {
_17 = (_6, _1, _12, 7372657433507967876_i64);
_1 = !_17.1;
_11 = 17816_u16;
_4 = _13.1;
_14 = _3 as f64;
_23.0.3 = !_13.3;
_2 = !280740923723440219366277802553743125923_u128;
_6 = _13.2.1;
_23.0.2.0 = _5 as isize;
_23.1 = ['\u{9912e}','\u{5abc7}','\u{8ffd6}','\u{5e77c}','\u{73743}'];
Call(_23.0.1 = fn3(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb26 = {
_55 = _37;
_17.3 = (-6614260083885092951_i64);
_17.0 = !_36.0.2.1;
_50.0 = [_5,_5,_5,_5,_5,_5];
_14 = -(*_40);
_40 = core::ptr::addr_of_mut!(_27);
_13.0 = &(*_19);
_17.2 = _46;
_13.1 = _34.1 ^ _23.0.1;
_23.0.0 = Move(_19);
Goto(bb27)
}
bb27 = {
_26 = &_15;
_54 = (_22, _50.0, _5, _5);
_34.2 = (_36.0.2.0, _13.2.1);
_47 = _6;
_19 = Move(_13.0);
(*_32) = 14009_u16 ^ 4810_u16;
_44 = [_55,_55,_37,_37,_55];
_48 = &_9;
_30 = &(*_30);
_30 = &(*_30);
(*_32) = !1685_u16;
_10 = _36.0.3 | _36.0.3;
_47 = !_13.2.1;
_54.2 = _54.0 as i16;
match _5 {
11749 => bb29,
_ => bb28
}
}
bb28 = {
_17 = (_6, _1, _12, 7372657433507967876_i64);
_1 = !_17.1;
_11 = 17816_u16;
_4 = _13.1;
_14 = _3 as f64;
_23.0.3 = !_13.3;
_2 = !280740923723440219366277802553743125923_u128;
_6 = _13.2.1;
_23.0.2.0 = _5 as isize;
_23.1 = ['\u{9912e}','\u{5abc7}','\u{8ffd6}','\u{5e77c}','\u{73743}'];
Call(_23.0.1 = fn3(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb29 = {
_19 = &_31;
_62.0 = &_21;
_45 = _2 << _13.1;
_50.0 = _54.1;
_23.0.2.1 = _35 as i32;
_13.2 = (_34.2.0, _23.0.2.1);
_13 = (Move(_19), _34.1, _36.0.2, _23.0.3);
Goto(bb30)
}
bb30 = {
_58 = _23.2;
_17.1 = _23.2 & _58;
_36.0.2 = (_3, _34.2.1);
_54.3 = _22 as i16;
_23.0.1 = _34.2.0 as i8;
_9 = !_43;
_63 = (_50.0,);
_37 = _55;
_6 = _47;
_33 = [_8,_8,_8,_8,_8,_8,_8,_8];
_65.2 = _11 as f64;
_54 = (_22, _50.0, _5, _5);
_19 = &_31;
_59 = !_13.3;
_13.0 = &(*_19);
_65 = (_13.1, _22, _27);
_13.1 = _58 as i8;
_63.0 = _50.0;
_13.0 = &_31;
_72.0 = _17.1 as isize;
_52 = core::ptr::addr_of!(_43);
_30 = &_69;
_61 = [_13.3,_10,_13.3,_10];
_35 = _7;
_21 = [_23.0.2.1];
Goto(bb31)
}
bb31 = {
_13.2.1 = _34.1 as i32;
_36.0 = (Move(_19), _23.0.1, _34.2, _10);
_23.1 = _36.1;
_23.0.2 = (_13.2.0, _17.0);
_33 = [_8,_8,_8,_8,_8,_16,_8,_8];
_34.0 = &_48;
_32 = core::ptr::addr_of_mut!((*_32));
_52 = core::ptr::addr_of!(_43);
_13 = (Move(_34.0), _65.0, _34.2, _36.0.3);
_50 = (_54.1,);
_40 = core::ptr::addr_of_mut!((*_30));
_34.2.1 = _9 as i32;
_26 = &(*_26);
_46 = _17.2;
_67 = -_34.2.0;
_30 = &_76.2;
_23.0.3 = _13.3 ^ _13.3;
_74 = &_8;
_54.0 = -_22;
match _54.3 {
11749 => bb33,
_ => bb32
}
}
bb32 = {
_36.0.1 = _11 as i8;
_10 = _13.3 - _23.0.3;
_23.0.2 = _13.2;
_23.0.2.0 = !_34.2.0;
_36.0.2.0 = !_3;
_34.2.1 = _13.2.1;
_13.1 = !_36.0.1;
_19 = &_31;
_34.2.1 = _6;
_13.3 = _16 as u8;
_33 = [_16,_16,_16,_8,_16,_16,_16,_8];
_23.0.2 = _13.2;
_20 = [_36.0.3,_34.3,_36.0.3,_13.3];
_26 = &(*_26);
_9 = !5635984190368146580_usize;
_34 = (Move(_19), _13.1, _23.0.2, _10);
_13.1 = _27 as i8;
_13.1 = _36.0.1;
_31 = &_9;
_23.0.1 = _34.1 + _36.0.1;
_34.2.0 = _36.0.2.0 | _13.2.0;
_36.0.2.1 = _23.0.2.1;
_34.0 = &_31;
_37 = '\u{105932}';
_30 = &_27;
Goto(bb15)
}
bb33 = {
_21 = _51;
_53 = !_23.2;
match _17.3 {
0 => bb26,
340282366920938463456760347347883118505 => bb35,
_ => bb34
}
}
bb34 = {
_4 = _17.3 as i8;
_22 = _23.0.2.1 as f32;
_14 = -_27;
_23.2 = _14 == _27;
_17 = (_13.2.1, _23.2, _12, (-8030104250308180528_i64));
_13.2 = (_3, _17.0);
_3 = _13.2.0;
_13.2.0 = _11 as isize;
_9 = 9321855581843588102_usize;
_10 = !_13.3;
_23.0.2.0 = _11 as isize;
_14 = _23.0.1 as f64;
_10 = _13.3 >> _4;
_16 = _8;
_22 = _7 as f32;
_23.0.0 = &_31;
_13 = Move(_23.0);
_10 = _12 as u8;
_13.2.0 = _3 | _3;
_34.0 = &_31;
_20 = [_13.3,_13.3,_13.3,_13.3];
_9 = 9388831171853564857_usize | 2328818086597147715_usize;
_34.2.0 = _7 as isize;
Goto(bb11)
}
bb35 = {
_3 = _23.0.2.0 << _72.0;
_26 = &(*_26);
_9 = !(*_52);
_66 = _55;
_65.1 = _54.0 - _54.0;
_34.2 = (_13.2.0, _23.0.2.1);
_23.1 = [_66,_37,_66,_66,_37];
_31 = &(*_52);
_17.2 = _46;
_54.0 = _7 as f32;
Goto(bb36)
}
bb36 = {
_62.0 = &_21;
_50.0 = _54.1;
_36.2 = !_23.2;
_56 = _72.0 >> _3;
_23.2 = !_36.2;
_63.0 = _54.1;
_23.0.0 = &_48;
_49 = _54.3;
_17.1 = _53;
_34.0 = Move(_23.0.0);
_36.0.2 = (_3, _17.0);
_23.0.2.0 = !_67;
_21 = [_23.0.2.1];
_61 = _20;
_85 = _66;
_77 = _65.1;
_36.0.2.0 = _56;
_70 = &_76.2;
_68 = _13.2.0;
_36.0.2 = (_13.2.0, _13.2.1);
_34.0 = &_31;
_34.2.1 = (*_52) as i32;
_81 = [_56,_72.0,_23.0.2.0,_56,_56,_56,_3];
_17.0 = _6 & _23.0.2.1;
_10 = (*_32) as u8;
_1 = _36.2;
Goto(bb37)
}
bb37 = {
match _54.2 {
0 => bb28,
1 => bb24,
2 => bb25,
3 => bb11,
4 => bb14,
11749 => bb38,
_ => bb32
}
}
bb38 = {
_65.1 = _77;
_23.0.2.0 = _36.0.2.0;
_12 = !_46;
_26 = &(*_26);
_88 = -_8;
_63 = (_54.1,);
_27 = _65.2 + _65.2;
_42 = [_88,_88,(*_74),_8];
_65.0 = !_34.1;
_13.0 = &_48;
_23.0.2.0 = _67;
_84.1 = _22;
_11 = !18765_u16;
_76.1 = _45 as f32;
RET = core::ptr::addr_of_mut!(_81);
_48 = &(*_52);
_82 = _42;
_78 = _14 as i32;
_62.0 = &_51;
_63 = _50;
Goto(bb39)
}
bb39 = {
Call(_94 = dump_var(0_usize, 49_usize, Move(_49), 66_usize, Move(_66), 17_usize, Move(_17), 56_usize, Move(_56)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_94 = dump_var(0_usize, 78_usize, Move(_78), 53_usize, Move(_53), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_94 = dump_var(0_usize, 8_usize, Move(_8), 59_usize, Move(_59), 50_usize, Move(_50), 37_usize, Move(_37)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_94 = dump_var(0_usize, 5_usize, Move(_5), 10_usize, Move(_10), 43_usize, Move(_43), 68_usize, Move(_68)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_94 = dump_var(0_usize, 42_usize, Move(_42), 9_usize, Move(_9), 12_usize, Move(_12), 58_usize, Move(_58)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_94 = dump_var(0_usize, 3_usize, Move(_3), 95_usize, _95, 95_usize, _95, 95_usize, _95), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u64,mut _2: i8,mut _3: isize,mut _4: u16,mut _5: (isize, i32),mut _6: i128,mut _7: isize,mut _8: i16,mut _9: (isize, i32),mut _10: isize) -> usize {
mir! {
type RET = usize;
let _11: [i128; 8];
let _12: *mut Adt19;
let _13: &'static [u128; 1];
let _14: (&'static [isize; 3], [i32; 1]);
let _15: isize;
let _16: [i8; 4];
let _17: u64;
let _18: f64;
let _19: (isize, i8, f32);
let _20: ();
let _21: ();
{
_7 = _5.0 - _10;
_2 = (-13_i8);
_10 = _7 ^ _9.0;
_2 = (-20_i8);
_9.1 = false as i32;
_1 = 16309680329555406561_u64 | 2355684117491380058_u64;
_9 = (_7, _5.1);
RET = 53_u8 as usize;
_1 = !13126884050100919468_u64;
RET = !0_usize;
RET = 5878489278610701128_usize;
_8 = 26070_i16 << _10;
_4 = !47388_u16;
_5 = _9;
RET = 15979474059710058106_usize << _5.1;
_10 = false as isize;
_10 = _9.0;
_9.0 = _10 - _3;
_2 = 69_i8 >> _5.0;
_2 = (-24_i8);
_10 = true as isize;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9.1 = _5.1;
RET = !4_usize;
_8 = (-28109_i16);
_1 = 167121338064870686015686538206976652694_u128 as u64;
_2 = !20_i8;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211410 => bb6,
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
_9 = _5;
_10 = !_5.0;
_5.1 = _9.1 & _9.1;
_3 = '\u{ca603}' as isize;
_9 = (_3, _5.1);
_10 = (-6336380618612711783_i64) as isize;
Goto(bb7)
}
bb7 = {
_8 = 8394_i16;
_1 = _9.1 as u64;
_5.1 = !_9.1;
RET = 3_usize << _4;
_9.0 = '\u{8fd35}' as isize;
_5.1 = 191116916044398344701490549202884816802_u128 as i32;
_5.1 = _9.1;
_5.0 = 5440281131874020223_i64 as isize;
_9.0 = RET as isize;
_1 = RET as u64;
_8 = RET as i16;
_3 = _9.0 + _7;
_9 = _5;
_5 = (_3, _9.1);
_5 = _9;
_9 = _5;
_10 = _1 as isize;
_9.1 = _4 as i32;
_4 = 44712_u16;
_1 = 5215497548351945252_u64;
_5 = (_7, _9.1);
_5.0 = !_10;
_9.0 = _9.1 as isize;
_10 = _3;
_9.0 = _3 ^ _5.0;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
Call(_10 = fn2(_7, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_2 = _4 as i8;
_8 = _2 as i16;
_4 = !13503_u16;
_5.0 = _5.1 as isize;
_9 = _5;
_6 = !18964524043389368126302303230110231819_i128;
_5 = (_10, _9.1);
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_2 = 85_i8 << _10;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5.1 = _9.1;
_8 = _1 as i16;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
RET = 17164437471388162188_usize >> _5.0;
_7 = _5.0;
_9.1 = _5.1;
_8 = -(-15746_i16);
_1 = _4 as u64;
_11 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9.0 = _10;
_1 = 7946150704209241264_u64 ^ 9962810801689893243_u64;
_19.2 = _1 as f32;
_19.2 = RET as f32;
_7 = !_5.0;
Goto(bb9)
}
bb9 = {
Call(_20 = dump_var(1_usize, 11_usize, Move(_11), 8_usize, Move(_8), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_20 = dump_var(1_usize, 7_usize, Move(_7), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: f64;
let _4: i128;
let _5: i8;
let _6: &'static [isize; 3];
let _7: i64;
let _8: *mut Adt19;
let _9: &'static [char; 5];
let _10: *const &'static Adt46;
let _11: *const usize;
let _12: [i128; 8];
let _13: ();
let _14: ();
{
RET = -_2;
_2 = RET;
RET = -_2;
_1 = _2 + RET;
Goto(bb1)
}
bb1 = {
_4 = -(-156517856377022482189363065216909693192_i128);
_3 = 10267952802771119275_u64 as f64;
_3 = _2 as f64;
_5 = 16_i8;
_4 = '\u{5c49e}' as i128;
_3 = 167_u8 as f64;
_2 = _5 as isize;
_5 = !(-77_i8);
_3 = 140448603330074878982799684206189418150_u128 as f64;
_4 = -122834372162350167549064627141682031282_i128;
_1 = -_2;
_1 = RET - RET;
_2 = _1;
_1 = _2 ^ _2;
_7 = (-2131859477486079248_i64);
_5 = 2666999186652708729_u64 as i8;
_5 = 78_i8 ^ 114_i8;
RET = _1 + _1;
RET = !_1;
RET = _2;
RET = 3830404090_u32 as isize;
_4 = 13164831588777686471617439263548022800_i128;
Call(_7 = core::intrinsics::bswap((-6766817756168052555_i64)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = 37169393199805349290375277394743634663_u128 as isize;
RET = _2 & _2;
_1 = _3 as isize;
_4 = 31974518798009358689098709778485216320_i128 >> RET;
_2 = RET;
_2 = RET * RET;
_2 = -RET;
_7 = 8250395837134978819_i64 ^ (-5702972138852697149_i64);
_1 = RET - RET;
_5 = !(-20_i8);
_4 = (-149810029648781741270107723989863028065_i128);
RET = 59904_u16 as isize;
_4 = -(-79784624763030015221180090605074762813_i128);
_4 = 146144171713194380496490797426314084404_i128 ^ (-145479138147953529063081830532292214743_i128);
RET = _2;
_1 = _2 | _2;
RET = _1;
_7 = !1627392801688022161_i64;
_1 = '\u{f61de}' as isize;
RET = _2 * _2;
_5 = 101_i8;
_12 = [_4,_4,_4,_4,_4,_4,_4,_4];
Goto(bb3)
}
bb3 = {
Call(_13 = dump_var(2_usize, 5_usize, Move(_5), 1_usize, Move(_1), 7_usize, Move(_7), 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u16) -> i8 {
mir! {
type RET = i8;
let _2: Adt46;
let _3: (i32, bool, u32, i64);
let _4: isize;
let _5: &'static [isize; 3];
let _6: *mut [isize; 7];
let _7: (&'static [isize; 3], [i32; 1]);
let _8: [isize; 7];
let _9: f64;
let _10: [isize; 7];
let _11: f32;
let _12: *const u8;
let _13: (f32, u32, (f32, [i16; 6], i16, i16));
let _14: *mut Adt19;
let _15: *const [char; 5];
let _16: i64;
let _17: *const [char; 5];
let _18: ((&'static &'static usize, i8, (isize, i32), u8), [char; 5], bool, *const *const u8);
let _19: u16;
let _20: *mut i32;
let _21: f64;
let _22: isize;
let _23: [i128; 8];
let _24: bool;
let _25: usize;
let _26: &'static [i128; 4];
let _27: bool;
let _28: [i16; 6];
let _29: usize;
let _30: isize;
let _31: [u64; 8];
let _32: &'static i64;
let _33: (&'static [isize; 3], [i32; 1]);
let _34: *const *const u8;
let _35: *mut f64;
let _36: isize;
let _37: isize;
let _38: isize;
let _39: [i32; 1];
let _40: &'static &'static usize;
let _41: *const &'static Adt46;
let _42: &'static Adt46;
let _43: [i8; 4];
let _44: (&'static &'static usize, i8, (isize, i32), u8);
let _45: *mut i32;
let _46: u16;
let _47: Adt25;
let _48: u8;
let _49: &'static [isize; 3];
let _50: isize;
let _51: isize;
let _52: char;
let _53: usize;
let _54: u8;
let _55: isize;
let _56: u64;
let _57: ();
let _58: ();
{
RET = (-103_i8);
RET = (-78_i8);
RET = !(-8_i8);
_1 = 7545_u16;
_1 = !2158_u16;
RET = 65_i8 & (-9_i8);
RET = 121_i8;
RET = 82_i8;
RET = (-38_i8) ^ 105_i8;
RET = _1 as i8;
RET = 116_i8 << _1;
RET = -(-116_i8);
_3.3 = (-5880157083821833770_i64);
_3.2 = 1666831862_u32 + 950960081_u32;
_3 = (710837062_i32, false, 1541270148_u32, 8683855209171840798_i64);
RET = 58_i8 & 66_i8;
_3.3 = (-1498990206360176758_i64);
RET = (-110_i8);
_3.0 = (-962124435_i32) + 972120486_i32;
_3 = (356722313_i32, true, 1884837527_u32, (-9201768128673629674_i64));
_3.2 = !2360138972_u32;
_1 = '\u{e5e75}' as u16;
_3.0 = (-815323591_i32);
Goto(bb1)
}
bb1 = {
_3.2 = !1764207131_u32;
_4 = (-9223372036854775808_isize);
_3.2 = _4 as u32;
RET = -(-71_i8);
_7.1 = [_3.0];
_4 = (-9223372036854775808_isize);
RET = 69_i8;
_3.1 = !false;
_1 = !12911_u16;
RET = 115_i8 + 29_i8;
_3.3 = (-4748609806800546510_i64) ^ (-8328187331274696816_i64);
_3.3 = 6669621516604791954_i64 * 5830949766823107352_i64;
RET = _3.2 as i8;
_8 = [_4,_4,_4,_4,_4,_4,_4];
_1 = 61880_u16 >> _3.3;
_3.1 = !true;
_8 = [_4,_4,_4,_4,_4,_4,_4];
_3.1 = _1 > _1;
_6 = core::ptr::addr_of_mut!(_8);
_3.2 = 1472544693_u32 + 2396876868_u32;
RET = 69_i8;
_3.0 = !(-1110740359_i32);
_3.0 = 1585341002_i32;
Call(_3.0 = core::intrinsics::bswap((-1170820933_i32)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = 55128036505977745940150445396929805816_u128 as f64;
_3.1 = _9 > _9;
(*_6) = [_4,_4,_4,_4,_4,_4,_4];
(*_6) = [_4,_4,_4,_4,_4,_4,_4];
_10 = [_4,_4,_4,_4,_4,_4,_4];
_13.0 = 7790883992075763400_u64 as f32;
_13.2.1 = [(-11530_i16),(-19637_i16),13325_i16,6694_i16,(-6036_i16),(-7012_i16)];
_13.2.1 = [(-7381_i16),(-14643_i16),17614_i16,1078_i16,5606_i16,1510_i16];
_11 = -_13.0;
_13.1 = !_3.2;
_10 = [_4,_4,_4,_4,_4,_4,_4];
_11 = 23840_i16 as f32;
_9 = _4 as f64;
_13.2.3 = !4403_i16;
_3.2 = _13.1;
(*_6) = _10;
Goto(bb3)
}
bb3 = {
_7.1 = [_3.0];
_6 = core::ptr::addr_of_mut!(_10);
_3.1 = false;
_13.2.0 = _11 * _13.0;
_4 = (-99_isize) ^ (-9223372036854775808_isize);
RET = (-8_i8);
_13.2.0 = _13.0;
_18.3 = core::ptr::addr_of!(_12);
RET = -46_i8;
_13.2.3 = 29340_i16 + 29145_i16;
_18.2 = _3.1 ^ _3.1;
RET = 6_i8 << _13.1;
_18.0.2 = (_4, _3.0);
_3.1 = !_18.2;
_13.2.2 = -_13.2.3;
_18.0.2.0 = _4 - _4;
_10 = [_18.0.2.0,_18.0.2.0,_4,_18.0.2.0,_18.0.2.0,_18.0.2.0,_18.0.2.0];
_13.2.3 = _13.2.2;
_13.2.1 = [_13.2.3,_13.2.3,_13.2.2,_13.2.3,_13.2.2,_13.2.2];
_16 = _3.3;
_18.1 = ['\u{21b58}','\u{8e87a}','\u{8756b}','\u{a6e90}','\u{ed2a8}'];
_13.2.3 = -_13.2.2;
_3 = (_18.0.2.1, _18.2, _13.1, _16);
_12 = core::ptr::addr_of!(_18.0.3);
_17 = core::ptr::addr_of!(_18.1);
_1 = 7912_u16 * 61531_u16;
_1 = !14073_u16;
Goto(bb4)
}
bb4 = {
_3.0 = _18.0.2.1;
_3.2 = _13.1 ^ _13.1;
_18.1 = ['\u{94f52}','\u{f4e05}','\u{31963}','\u{5768d}','\u{23fef}'];
_15 = Move(_17);
RET = !(-121_i8);
(*_6) = [_18.0.2.0,_18.0.2.0,_18.0.2.0,_18.0.2.0,_18.0.2.0,_4,_4];
_13.2.2 = _13.2.3 * _13.2.3;
(*_12) = _3.3 as u8;
_13.1 = _3.2 & _3.2;
RET = 113_i8;
_8 = (*_6);
(*_6) = _8;
_17 = core::ptr::addr_of!(_18.1);
_18.3 = core::ptr::addr_of!(_12);
_18.0.2.0 = _4;
_17 = Move(_15);
_3.3 = 132658731739786413072954941865721880492_i128 as i64;
match RET {
113 => bb5,
_ => bb1
}
}
bb5 = {
_23 = [(-56624996368366609247947862259009027247_i128),119375649413455214727378996020782172574_i128,(-89597870584099156599371174050824552033_i128),(-133509438492684623780038676679490413346_i128),(-29598442662828768452929486220471460939_i128),(-7845657726508799233045697419381056510_i128),159875611327271580626931170091788060153_i128,95385338534676075279996957171966481425_i128];
RET = (-13_i8) & (-105_i8);
_9 = _13.1 as f64;
_18.1 = ['\u{a416b}','\u{de7fa}','\u{473b0}','\u{abf62}','\u{23b05}'];
_3.0 = _18.0.2.1 & _18.0.2.1;
_18.0.2.1 = _13.1 as i32;
_15 = core::ptr::addr_of!(_18.1);
_4 = _18.0.2.0;
(*_12) = _3.0 as u8;
_21 = _13.2.2 as f64;
_18.2 = _3.1;
_18.0.1 = _13.1 as i8;
_18.0.3 = 96_u8;
_24 = _3.1;
_28 = _13.2.1;
Call(_25 = fn4(_18.0.2, Move(_6), _18.0.2.0, (*_15), Move(_17), (*_15), _11, _18.0.2.1, _13.1, RET, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13.2.3 = _13.2.2;
_6 = core::ptr::addr_of_mut!(_10);
_3.0 = !_18.0.2.1;
_3 = (_18.0.2.1, _18.2, _13.1, _16);
_22 = !_18.0.2.0;
_13.2.2 = _18.0.2.0 as i16;
_4 = _18.0.2.0;
_18.0.2.0 = _22;
_4 = -_22;
RET = _13.2.3 as i8;
(*_6) = _8;
_11 = 4750372613381547135_u64 as f32;
_18.0.2.1 = _3.0;
_19 = _13.1 as u16;
_19 = _21 as u16;
(*_15) = ['\u{17df0}','\u{55b45}','\u{d9e69}','\u{80c53}','\u{2e574}'];
_25 = _13.2.3 as usize;
_24 = !_3.1;
_17 = Move(_15);
_8 = [_22,_22,_4,_18.0.2.0,_4,_4,_22];
_18.0.3 = 231_u8 >> _25;
_1 = _19;
_13.2.3 = -_13.2.2;
_18.0.1 = RET;
Call(_25 = fn5(_4, Move(_6), _3.1, _11, _18.0.2, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_31 = [3142287182365931180_u64,13052541294654602037_u64,18387514563783650170_u64,11307741727254302699_u64,3119209077251009681_u64,15579485905490006919_u64,17648637800479530845_u64,2986923792792287239_u64];
_6 = core::ptr::addr_of_mut!(_8);
_21 = _9;
_24 = !_3.1;
_3 = (_18.0.2.1, _24, _13.1, _16);
_1 = _25 as u16;
_20 = core::ptr::addr_of_mut!(_18.0.2.1);
_32 = &_16;
_13.1 = _3.2 << (*_12);
_3.2 = _13.1;
_18.0.2.1 = _24 as i32;
_18.0.2.1 = _3.0;
_13.2.0 = _13.0 - _11;
Goto(bb8)
}
bb8 = {
(*_6) = _10;
_28 = [_13.2.2,_13.2.2,_13.2.3,_13.2.3,_13.2.2,_13.2.2];
RET = -_18.0.1;
(*_6) = _10;
_3.3 = (*_32);
_3.1 = !_24;
_15 = core::ptr::addr_of!(_18.1);
_35 = core::ptr::addr_of_mut!(_9);
_25 = (*_20) as usize;
_3.1 = !_18.2;
RET = -_18.0.1;
_35 = core::ptr::addr_of_mut!(_21);
_3.2 = _13.1;
RET = _18.0.1 + _18.0.1;
_3 = ((*_20), _18.2, _13.1, (*_32));
_13.2.3 = _13.2.2;
Goto(bb9)
}
bb9 = {
_34 = core::ptr::addr_of!(_12);
_16 = !_3.3;
(*_6) = [_22,_22,_18.0.2.0,_4,_22,_4,_18.0.2.0];
_6 = core::ptr::addr_of_mut!(_8);
_18.3 = core::ptr::addr_of!((*_34));
Call(_18.0.1 = core::intrinsics::transmute(RET), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_36 = 18053733825235855044483519503864739898_u128 as isize;
RET = !_18.0.1;
_33.1 = _7.1;
_38 = !_4;
(*_15) = ['\u{c2107}','\u{9390}','\u{1eab2}','\u{4e093}','\u{b9de}'];
_18.0.1 = _18.2 as i8;
(*_15) = ['\u{1e69a}','\u{46f58}','\u{272c6}','\u{ab44d}','\u{c5ce7}'];
(*_20) = _13.2.0 as i32;
_3.0 = !_18.0.2.1;
_3.1 = RET == RET;
_18.2 = _3.1 & _24;
Goto(bb11)
}
bb11 = {
_12 = core::ptr::addr_of!(_18.0.3);
_3.0 = (*_20);
(*_35) = -_9;
_13.2.3 = _18.0.2.0 as i16;
_13.2.0 = _11 * _11;
_10 = (*_6);
_42 = &_2;
_3.3 = _16;
_32 = &_3.3;
_39 = _33.1;
_29 = _25;
_46 = !_1;
_13.2.0 = -_13.0;
_47.fld0 = _13.2.1;
(*_15) = ['\u{cf0bc}','\u{fffad}','\u{2d71e}','\u{5c3b7}','\u{1fbd1}'];
_20 = core::ptr::addr_of_mut!(_47.fld2);
(*_34) = core::ptr::addr_of!((*_12));
_47.fld0 = [_13.2.2,_13.2.3,_13.2.2,_13.2.2,_13.2.3,_13.2.2];
_37 = _16 as isize;
_13.2.2 = _13.2.3 | _13.2.3;
Call(_11 = fn14(Move(_32), _38, _31, _18.0.3, _3.1, Move((*_34)), RET, (*_35), _18.0.2, Move(_17)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_36 = _37 >> _3.2;
RET = _18.0.1;
_13.2.0 = _13.0 * _11;
_33.1 = [_3.0];
_33.1 = _7.1;
(*_34) = core::ptr::addr_of!(_44.3);
_18.0.1 = RET - RET;
_12 = core::ptr::addr_of!((*_12));
_50 = _36 * _22;
_13.0 = _13.2.0;
_30 = !_36;
(*_20) = _18.0.2.1 - _3.0;
_51 = _50 & _30;
_18.1 = ['\u{acfb7}','\u{e73cd}','\u{31777}','\u{8c5e8}','\u{3b821}'];
RET = _18.0.1;
_32 = &_3.3;
_18.0.2.0 = _18.2 as isize;
_16 = 2325143236107756365_u64 as i64;
Call((*_12) = fn15(Move(_32), _16, _18.0.3, _51, RET, (*_15), _18.0.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_34) = core::ptr::addr_of!((*_12));
_13.2.1 = [_13.2.2,_13.2.2,_13.2.3,_13.2.2,_13.2.2,_13.2.2];
(*_6) = _10;
_18.0.3 = !_44.3;
_13.0 = _11;
_27 = !_3.1;
(*_34) = core::ptr::addr_of!(_18.0.3);
_13.0 = 132849642605839886543459270338780424595_i128 as f32;
_33.1 = _39;
_22 = _30;
Call(_23 = fn16(Move(_12), (*_12), _44.3, (*_15), (*_12), Move(_34), Move(_6), Move(_18.3), _16, _50), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_35) = _3.2 as f64;
_44.1 = _13.2.0 as i8;
_3.0 = (*_20) >> _18.0.2.0;
_13.2.1 = [_13.2.2,_13.2.2,_13.2.2,_13.2.3,_13.2.2,_13.2.3];
_6 = core::ptr::addr_of_mut!(_10);
_28 = [_13.2.2,_13.2.3,_13.2.3,_13.2.3,_13.2.3,_13.2.2];
_18.0.2.1 = -_3.0;
_25 = _29;
_43 = [_44.1,_44.1,_18.0.1,RET];
_44.2.0 = RET as isize;
_9 = (*_35) * (*_35);
_45 = core::ptr::addr_of_mut!(_3.0);
_50 = _30;
_3.3 = _16 + _16;
_19 = !_46;
_18.3 = core::ptr::addr_of!(_12);
_41 = core::ptr::addr_of!(_42);
_10 = [_50,_30,_51,_4,_18.0.2.0,_50,_30];
Goto(bb15)
}
bb15 = {
Call(_57 = dump_var(3_usize, 16_usize, Move(_16), 24_usize, Move(_24), 29_usize, Move(_29), 38_usize, Move(_38)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_57 = dump_var(3_usize, 27_usize, Move(_27), 28_usize, Move(_28), 31_usize, Move(_31), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(3_usize, 30_usize, Move(_30), 22_usize, Move(_22), 37_usize, Move(_37), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (isize, i32),mut _2: *mut [isize; 7],mut _3: isize,mut _4: [char; 5],mut _5: *const [char; 5],mut _6: [char; 5],mut _7: f32,mut _8: i32,mut _9: u32,mut _10: i8,mut _11: (f32, u32, (f32, [i16; 6], i16, i16))) -> usize {
mir! {
type RET = usize;
let _12: &'static [isize; 3];
let _13: Adt25;
let _14: i8;
let _15: [char; 5];
let _16: f64;
let _17: Adt25;
let _18: i8;
let _19: bool;
let _20: i64;
let _21: f32;
let _22: [u128; 2];
let _23: ();
let _24: ();
{
_11.1 = _9 * _9;
_6 = _4;
RET = 7086791531947257862_usize;
_11.2.0 = -_11.0;
_9 = _11.1 & _11.1;
_1.0 = !_3;
_11.1 = _9 + _9;
_11.0 = _11.2.0 + _11.2.0;
_1.1 = _10 as i32;
_1.0 = _3;
_1.1 = RET as i32;
_14 = !_10;
_16 = 163_u8 as f64;
_17 = Adt25 { fld0: _11.2.1,fld1: 217848650369872619812487751681038729511_u128,fld2: _8 };
_11.2.3 = _11.2.2 & _11.2.2;
_13.fld2 = !_17.fld2;
_13.fld2 = _17.fld1 as i32;
_19 = true;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
7086791531947257862 => bb6,
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
_11.2 = (_7, _17.fld0, (-5282_i16), (-107_i16));
_4 = _6;
_11.2.2 = _11.2.3;
_11.2.0 = _11.0 - _7;
_7 = _11.0;
_13 = Adt25 { fld0: _17.fld0,fld1: _17.fld1,fld2: _8 };
_15 = _6;
_16 = RET as f64;
_13.fld0 = _17.fld0;
_11.2.3 = _11.2.2;
_14 = _10 << _11.1;
_17.fld1 = _13.fld1 + _13.fld1;
_17.fld1 = !_13.fld1;
RET = _14 as usize;
_15 = ['\u{cae80}','\u{72691}','\u{656ee}','\u{869e9}','\u{442bd}'];
Goto(bb7)
}
bb7 = {
Call(_23 = dump_var(4_usize, 19_usize, Move(_19), 15_usize, Move(_15), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_23 = dump_var(4_usize, 9_usize, Move(_9), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: *mut [isize; 7],mut _3: bool,mut _4: f32,mut _5: (isize, i32),mut _6: u16) -> usize {
mir! {
type RET = usize;
let _7: isize;
let _8: f64;
let _9: isize;
let _10: char;
let _11: i128;
let _12: i128;
let _13: &'static Adt46;
let _14: [i8; 4];
let _15: (isize, i8, f32);
let _16: (i32, bool, u32, i64);
let _17: f64;
let _18: bool;
let _19: [i128; 8];
let _20: &'static [isize; 3];
let _21: (*mut *const u8, &'static &'static i128, i64, *mut u16);
let _22: (bool,);
let _23: &'static [isize; 3];
let _24: u16;
let _25: ((isize, i32), Adt19, i8, *mut *const u8);
let _26: (i32, bool, u32, i64);
let _27: (&'static [i32; 1],);
let _28: bool;
let _29: bool;
let _30: char;
let _31: bool;
let _32: isize;
let _33: (&'static &'static usize, i8, (isize, i32), u8);
let _34: &'static i128;
let _35: Adt54;
let _36: u32;
let _37: f32;
let _38: u32;
let _39: *const &'static Adt46;
let _40: u64;
let _41: ();
let _42: ();
{
RET = 0_usize;
_6 = 62188_u16;
_5.1 = _5.0 as i32;
_5.0 = 1789787749835312252_i64 as isize;
_1 = !_5.0;
_3 = !false;
_5.1 = 1535694222_i32 * (-119526807_i32);
RET = !1_usize;
_5 = (_1, 1179908161_i32);
Goto(bb1)
}
bb1 = {
_7 = !_1;
_7 = '\u{bdccd}' as isize;
_1 = 251_u8 as isize;
_5.0 = 17652953762018816667_u64 as isize;
_5.1 = 1600429375_i32 << _6;
_4 = 324385801_u32 as f32;
_6 = 5241269569320279446_u64 as u16;
_5 = (_7, (-245271847_i32));
_5 = (_7, (-1426709210_i32));
_5 = (_7, 986524919_i32);
_7 = -_1;
_6 = !31374_u16;
Goto(bb2)
}
bb2 = {
_1 = 12198389870581401450_u64 as isize;
_6 = !23751_u16;
_9 = _3 as isize;
_7 = _5.0;
RET = 166_u8 as usize;
_5.1 = (-887433660_i32) ^ 1460267238_i32;
Goto(bb3)
}
bb3 = {
_8 = _4 as f64;
_4 = 2528374578_u32 as f32;
_12 = -125557795377449609560571800489928044685_i128;
Call(RET = fn6(Move(_2), _5, _3, _4, _5.0, _5, _1, _1, _5.1, _9, _5.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = RET > RET;
_12 = _8 as i128;
_15.0 = _1 | _5.0;
_10 = '\u{c9546}';
_9 = !_7;
_11 = -_12;
_15.2 = -_4;
_16.0 = _4 as i32;
_8 = RET as f64;
_16.1 = _4 > _15.2;
RET = 16924623338692028860_usize + 14655433900238726709_usize;
_16.3 = -(-3117336607767514852_i64);
_15.1 = !(-99_i8);
_1 = _5.0;
_17 = _16.0 as f64;
_6 = !45102_u16;
_16.2 = 507675926_u32;
_9 = _15.0 * _7;
_16.0 = _15.0 as i32;
_16.2 = 2880603352_u32 >> _9;
Goto(bb5)
}
bb5 = {
_5 = (_7, _16.0);
_16.3 = -7600376146495270089_i64;
_5.1 = 41_u8 as i32;
_5.0 = -_7;
_4 = _15.2 - _15.2;
_25.0.1 = _16.0 & _16.0;
_22.0 = !_3;
_26.0 = _25.0.1;
_16.2 = _4 as u32;
_8 = _17;
_26.2 = _25.0.1 as u32;
_24 = !_6;
_16.2 = _26.2 + _26.2;
_16.1 = _26.0 < _26.0;
_26.0 = _25.0.1 ^ _16.0;
_22.0 = !_16.1;
Call(_18 = fn8(_16.2, _16.1, _22, _22.0, _16, _22, _9, _15, _16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = (_25.0.1, _18, _26.2, (-7965424482143245152_i64));
_15 = (_7, 63_i8, _4);
_15.1 = 52_i8;
_15 = (_9, 44_i8, _4);
_5 = (_15.0, _25.0.1);
_5 = (_15.0, _26.0);
_16.3 = 3608166468643392106_i64;
_26.1 = !_18;
_22 = (_26.1,);
_11 = -_12;
_21.2 = _16.3 | _16.3;
_5.0 = _15.1 as isize;
_14 = [_15.1,_15.1,_15.1,_15.1];
_4 = _5.1 as f32;
_19 = [_12,_11,_12,_11,_11,_11,_12,_12];
_7 = _9;
_31 = _16.1 <= _22.0;
_21.3 = core::ptr::addr_of_mut!(_6);
_28 = !_26.1;
_21.3 = core::ptr::addr_of_mut!(_6);
_10 = '\u{9901c}';
_11 = 9252_i16 as i128;
match _15.1 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
44 => bb11,
_ => bb10
}
}
bb7 = {
_7 = !_1;
_7 = '\u{bdccd}' as isize;
_1 = 251_u8 as isize;
_5.0 = 17652953762018816667_u64 as isize;
_5.1 = 1600429375_i32 << _6;
_4 = 324385801_u32 as f32;
_6 = 5241269569320279446_u64 as u16;
_5 = (_7, (-245271847_i32));
_5 = (_7, (-1426709210_i32));
_5 = (_7, 986524919_i32);
_7 = -_1;
_6 = !31374_u16;
Goto(bb2)
}
bb8 = {
_3 = RET > RET;
_12 = _8 as i128;
_15.0 = _1 | _5.0;
_10 = '\u{c9546}';
_9 = !_7;
_11 = -_12;
_15.2 = -_4;
_16.0 = _4 as i32;
_8 = RET as f64;
_16.1 = _4 > _15.2;
RET = 16924623338692028860_usize + 14655433900238726709_usize;
_16.3 = -(-3117336607767514852_i64);
_15.1 = !(-99_i8);
_1 = _5.0;
_17 = _16.0 as f64;
_6 = !45102_u16;
_16.2 = 507675926_u32;
_9 = _15.0 * _7;
_16.0 = _15.0 as i32;
_16.2 = 2880603352_u32 >> _9;
Goto(bb5)
}
bb9 = {
_8 = _4 as f64;
_4 = 2528374578_u32 as f32;
_12 = -125557795377449609560571800489928044685_i128;
Call(RET = fn6(Move(_2), _5, _3, _4, _5.0, _5, _1, _1, _5.1, _9, _5.0), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_1 = 12198389870581401450_u64 as isize;
_6 = !23751_u16;
_9 = _3 as isize;
_7 = _5.0;
RET = 166_u8 as usize;
_5.1 = (-887433660_i32) ^ 1460267238_i32;
Goto(bb3)
}
bb11 = {
_26.3 = _16.3 >> _16.2;
_14 = [_15.1,_15.1,_15.1,_15.1];
_33.2.0 = _15.0 & _5.0;
_26.3 = _21.2 | _16.3;
_3 = !_31;
_25.0.0 = _5.0;
_32 = -_25.0.0;
_12 = _21.2 as i128;
RET = 8891459872447862480_usize >> _5.0;
_21.1 = &_34;
_36 = _16.2;
_33.2 = (_5.0, _5.1);
_16.2 = !_26.2;
_22 = (_26.1,);
_21.3 = core::ptr::addr_of_mut!(_6);
Goto(bb12)
}
bb12 = {
_14 = [_15.1,_15.1,_15.1,_15.1];
_10 = '\u{7f646}';
_16 = (_33.2.1, _18, _26.2, _21.2);
RET = 5_usize;
_18 = _16.1 != _3;
_37 = (-13842_i16) as f32;
_33.1 = (-22208_i16) as i8;
_14 = [_15.1,_15.1,_15.1,_15.1];
_5.0 = (-11504_i16) as isize;
_30 = _10;
RET = 6411860839955480384_usize;
_31 = _16.1;
_17 = -_8;
_30 = _10;
_26.3 = _21.2;
_33.2 = _5;
Call(_26.1 = fn13(_3, _26.0, _22.0, _25.0, _18, _31, Move(_21.3), _22.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_25.0.1 = -_33.2.1;
_5 = (_25.0.0, _25.0.1);
_11 = _6 as i128;
_34 = &_11;
_38 = 23758_i16 as u32;
RET = 15221385730418641067_usize | 17651754583089693451_usize;
match _15.1 {
0 => bb14,
44 => bb16,
_ => bb15
}
}
bb14 = {
_3 = RET > RET;
_12 = _8 as i128;
_15.0 = _1 | _5.0;
_10 = '\u{c9546}';
_9 = !_7;
_11 = -_12;
_15.2 = -_4;
_16.0 = _4 as i32;
_8 = RET as f64;
_16.1 = _4 > _15.2;
RET = 16924623338692028860_usize + 14655433900238726709_usize;
_16.3 = -(-3117336607767514852_i64);
_15.1 = !(-99_i8);
_1 = _5.0;
_17 = _16.0 as f64;
_6 = !45102_u16;
_16.2 = 507675926_u32;
_9 = _15.0 * _7;
_16.0 = _15.0 as i32;
_16.2 = 2880603352_u32 >> _9;
Goto(bb5)
}
bb15 = {
_7 = !_1;
_7 = '\u{bdccd}' as isize;
_1 = 251_u8 as isize;
_5.0 = 17652953762018816667_u64 as isize;
_5.1 = 1600429375_i32 << _6;
_4 = 324385801_u32 as f32;
_6 = 5241269569320279446_u64 as u16;
_5 = (_7, (-245271847_i32));
_5 = (_7, (-1426709210_i32));
_5 = (_7, 986524919_i32);
_7 = -_1;
_6 = !31374_u16;
Goto(bb2)
}
bb16 = {
_31 = _18 ^ _3;
_15 = (_1, _33.1, _4);
_25.2 = _8 as i8;
_3 = _26.1;
_26.3 = !_21.2;
_22.0 = _31;
_25.0 = (_32, _5.1);
_29 = !_18;
_33.3 = _16.0 as u8;
_33.1 = 2313_i16 as i8;
_26 = (_16.0, _16.1, _16.2, _16.3);
_4 = _15.2;
_25.2 = !_33.1;
_16.3 = _6 as i64;
_33.2.0 = _25.0.0 << _33.2.1;
_25.1 = Adt19::Variant1 { fld0: _31,fld1: 13312563343347034263_u64,fld2: RET,fld3: _33.1,fld4: _36,fld5: _33.2,fld6: _17,fld7: _12 };
place!(Field::<i8>(Variant(_25.1, 1), 3)) = !_25.2;
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(5_usize, 28_usize, Move(_28), 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(5_usize, 30_usize, Move(_30), 1_usize, Move(_1), 38_usize, Move(_38), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(5_usize, 9_usize, Move(_9), 24_usize, Move(_24), 11_usize, Move(_11), 42_usize, _42), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut [isize; 7],mut _2: (isize, i32),mut _3: bool,mut _4: f32,mut _5: isize,mut _6: (isize, i32),mut _7: isize,mut _8: isize,mut _9: i32,mut _10: isize,mut _11: isize) -> usize {
mir! {
type RET = usize;
let _12: &'static [i128; 4];
let _13: f64;
let _14: u128;
let _15: f64;
let _16: i8;
let _17: i64;
let _18: i8;
let _19: [u64; 8];
let _20: isize;
let _21: (f32, [i16; 6], i16, i16);
let _22: i16;
let _23: i32;
let _24: (&'static [i32; 1],);
let _25: (f32, u32, (f32, [i16; 6], i16, i16));
let _26: isize;
let _27: [i128; 4];
let _28: Adt52;
let _29: isize;
let _30: &'static &'static usize;
let _31: ();
let _32: ();
{
_2 = (_8, _9);
_4 = (-4672_i16) as f32;
RET = !944607368088534463_usize;
_16 = !110_i8;
_3 = false;
_13 = RET as f64;
_2.1 = _3 as i32;
Goto(bb1)
}
bb1 = {
_6.0 = 97_u8 as isize;
_9 = _7 as i32;
_17 = 8691824616169539288_i64 - (-5299606683517576137_i64);
_19 = [16310805365476606342_u64,9266785561248309966_u64,745778236486080008_u64,16346810933916003093_u64,11833462895749115856_u64,5963969509133079754_u64,1589245519275803841_u64,6011486107964651392_u64];
_6.1 = _2.1 | _9;
_2.1 = _17 as i32;
_19 = [293824311926814032_u64,11671018124466283389_u64,16750124078710567290_u64,16219510720112318305_u64,11727164804427808867_u64,11388005214433300110_u64,12335143759967544908_u64,11656747737490581193_u64];
_16 = _13 as i8;
_15 = 31_u8 as f64;
_9 = -_2.1;
_2 = (_11, _6.1);
_20 = _6.0;
_13 = _17 as f64;
_8 = _10 * _2.0;
_6.0 = _10;
_18 = 15_u8 as i8;
_4 = 23230_i16 as f32;
_11 = 11992573064786499787_u64 as isize;
_15 = _13 * _13;
Goto(bb2)
}
bb2 = {
RET = 8808592326563291726_usize + 618573980500148402_usize;
_18 = 766734894_u32 as i8;
_15 = _13 * _13;
_9 = -_6.1;
_2 = _6;
_3 = true;
RET = 150_u8 as usize;
_11 = _7;
_4 = 163023697858213963657544918498995976524_u128 as f32;
_8 = _5 >> _11;
_6.0 = -_8;
_7 = _4 as isize;
RET = _13 as usize;
RET = 15376542865043940338_usize * 18313461479032258781_usize;
_13 = _15;
_7 = !_5;
_11 = _8;
_6.1 = !_9;
_11 = _20;
_3 = true;
_5 = !_6.0;
_7 = 246_u8 as isize;
Call(_21.0 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = [4170523532871944466_u64,7813224884841235140_u64,6124814044282126304_u64,9230616302399925283_u64,5547429400310405561_u64,5536224989107360381_u64,2211767850546153599_u64,3106436210467308518_u64];
_9 = _7 as i32;
_10 = 40349_u16 as isize;
_6 = (_2.0, _2.1);
_2.1 = _9 * _6.1;
_10 = RET as isize;
_11 = -_2.0;
_13 = _21.0 as f64;
_8 = _10 << _11;
_15 = -_13;
RET = _17 as usize;
Goto(bb4)
}
bb4 = {
_21.2 = !27077_i16;
_25.2.0 = _21.0 + _21.0;
_10 = -_8;
Call(_11 = fn7(_10, _5, _10, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = true;
_2.0 = -_11;
_19 = [10670940055023862808_u64,3500388154877126843_u64,10055301299114460322_u64,18394510502364785511_u64,1108195001875512454_u64,14766303474205243629_u64,9573049205798725534_u64,2010049580755540900_u64];
_6.0 = !_11;
_21.1 = [_21.2,_21.2,_21.2,_21.2,_21.2,_21.2];
_13 = 3073403474_u32 as f64;
_19 = [10272845362956591017_u64,13976973918740222204_u64,11448933703807463217_u64,4501583135762176349_u64,7294745933696009727_u64,14400397000789382221_u64,971000233080002320_u64,7554503564586166633_u64];
_6 = (_11, _9);
_21.3 = -_21.2;
_25.2.1 = [_21.2,_21.3,_21.3,_21.2,_21.3,_21.3];
_25.2.2 = _3 as i16;
_4 = -_25.2.0;
_23 = _9;
_25 = (_4, 3063235174_u32, _21);
RET = 61637_u16 as usize;
match _25.1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
3063235174 => bb14,
_ => bb13
}
}
bb6 = {
_21.2 = !27077_i16;
_25.2.0 = _21.0 + _21.0;
_10 = -_8;
Call(_11 = fn7(_10, _5, _10, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_19 = [4170523532871944466_u64,7813224884841235140_u64,6124814044282126304_u64,9230616302399925283_u64,5547429400310405561_u64,5536224989107360381_u64,2211767850546153599_u64,3106436210467308518_u64];
_9 = _7 as i32;
_10 = 40349_u16 as isize;
_6 = (_2.0, _2.1);
_2.1 = _9 * _6.1;
_10 = RET as isize;
_11 = -_2.0;
_13 = _21.0 as f64;
_8 = _10 << _11;
_15 = -_13;
RET = _17 as usize;
Goto(bb4)
}
bb8 = {
RET = 8808592326563291726_usize + 618573980500148402_usize;
_18 = 766734894_u32 as i8;
_15 = _13 * _13;
_9 = -_6.1;
_2 = _6;
_3 = true;
RET = 150_u8 as usize;
_11 = _7;
_4 = 163023697858213963657544918498995976524_u128 as f32;
_8 = _5 >> _11;
_6.0 = -_8;
_7 = _4 as isize;
RET = _13 as usize;
RET = 15376542865043940338_usize * 18313461479032258781_usize;
_13 = _15;
_7 = !_5;
_11 = _8;
_6.1 = !_9;
_11 = _20;
_3 = true;
_5 = !_6.0;
_7 = 246_u8 as isize;
Call(_21.0 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_6.0 = 97_u8 as isize;
_9 = _7 as i32;
_17 = 8691824616169539288_i64 - (-5299606683517576137_i64);
_19 = [16310805365476606342_u64,9266785561248309966_u64,745778236486080008_u64,16346810933916003093_u64,11833462895749115856_u64,5963969509133079754_u64,1589245519275803841_u64,6011486107964651392_u64];
_6.1 = _2.1 | _9;
_2.1 = _17 as i32;
_19 = [293824311926814032_u64,11671018124466283389_u64,16750124078710567290_u64,16219510720112318305_u64,11727164804427808867_u64,11388005214433300110_u64,12335143759967544908_u64,11656747737490581193_u64];
_16 = _13 as i8;
_15 = 31_u8 as f64;
_9 = -_2.1;
_2 = (_11, _6.1);
_20 = _6.0;
_13 = _17 as f64;
_8 = _10 * _2.0;
_6.0 = _10;
_18 = 15_u8 as i8;
_4 = 23230_i16 as f32;
_11 = 11992573064786499787_u64 as isize;
_15 = _13 * _13;
Goto(bb2)
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
_13 = _15 * _15;
_6 = (_2.0, _2.1);
_2.1 = 47_u8 as i32;
_21.1 = _25.2.1;
_10 = _2.0;
_21.1 = _25.2.1;
_25.2 = (_25.0, _21.1, _21.2, _21.3);
_15 = -_13;
_3 = !false;
_25.2 = (_25.0, _21.1, _21.3, _21.2);
_26 = -_2.0;
_25.0 = _4 + _4;
_27 = [95025141642482158963004702690608816833_i128,(-69186463613068402734493857095815011487_i128),125367494215081795073830085644963281758_i128,31479207893646969532325010037142505341_i128];
_25.1 = 3536355611_u32 << _11;
_25.2.1 = [_21.3,_21.2,_21.2,_21.2,_21.2,_25.2.2];
_8 = !_6.0;
_26 = _16 as isize;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(6_usize, 3_usize, Move(_3), 9_usize, Move(_9), 11_usize, Move(_11), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(6_usize, 2_usize, Move(_2), 20_usize, Move(_20), 17_usize, Move(_17), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: &'static usize;
let _6: Adt76;
let _7: u64;
let _8: f64;
let _9: [u8; 4];
let _10: char;
let _11: isize;
let _12: ();
let _13: ();
{
RET = -_1;
RET = _2 | _3;
RET = _2;
_1 = !_4;
_2 = 14019332499918041794_usize as isize;
_3 = 20053_i16 as isize;
_2 = 245_u8 as isize;
_4 = RET << _2;
_3 = (-7024364128335278946_i64) as isize;
_1 = -RET;
RET = _4;
_1 = RET | _4;
_4 = _1;
_4 = _1;
_3 = '\u{11d7d}' as isize;
_3 = _1;
RET = '\u{22e34}' as isize;
RET = _4;
_7 = 3080014298679291887_u64 >> RET;
_8 = _7 as f64;
_3 = _4 * RET;
_2 = 195_u8 as isize;
Goto(bb1)
}
bb1 = {
RET = _3 << _3;
_1 = -_3;
_7 = !17203041055544599379_u64;
_9 = [34_u8,189_u8,66_u8,181_u8];
_8 = 36_u8 as f64;
_10 = '\u{ea3f5}';
_8 = 64_i8 as f64;
RET = _1;
_4 = _7 as isize;
_8 = (-1956259507_i32) as f64;
_1 = _7 as isize;
_1 = _3;
_9 = [157_u8,166_u8,236_u8,141_u8];
_7 = 16816284644796212348_u64;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(7_usize, 1_usize, Move(_1), 10_usize, Move(_10), 9_usize, Move(_9), 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u32,mut _2: bool,mut _3: (bool,),mut _4: bool,mut _5: (i32, bool, u32, i64),mut _6: (bool,),mut _7: isize,mut _8: (isize, i8, f32),mut _9: (i32, bool, u32, i64)) -> bool {
mir! {
type RET = bool;
let _10: char;
let _11: isize;
let _12: bool;
let _13: (u128, &'static [char; 5]);
let _14: &'static f64;
let _15: isize;
let _16: f64;
let _17: isize;
let _18: [u128; 1];
let _19: Adt51;
let _20: isize;
let _21: f32;
let _22: [i128; 8];
let _23: ();
let _24: ();
{
RET = !_2;
_4 = _9.2 == _9.2;
_5.0 = 13254257209627723310_u64 as i32;
Goto(bb1)
}
bb1 = {
_8.2 = _8.1 as f32;
_6.0 = !_5.1;
_3.0 = _4;
_2 = _3.0 ^ _4;
_7 = !_8.0;
_3.0 = _2 ^ _2;
_8.1 = 16782591474734123340_usize as i8;
_5.0 = !_9.0;
_5.2 = (-39791483030969396521318919261813258719_i128) as u32;
_9 = (_5.0, _2, _1, _5.3);
_5 = _9;
_6 = _3;
_9.0 = _5.0;
_4 = !_5.1;
_12 = _4 <= _2;
_8.2 = (-161477186737814610349725230827925812516_i128) as f32;
Goto(bb2)
}
bb2 = {
_5.3 = -_9.3;
_13.0 = (-7214_i16) as u128;
_3.0 = _5.1 ^ _6.0;
_6.0 = !_9.1;
_5 = (_9.0, _6.0, _9.2, _9.3);
Call(_9.3 = fn9(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = '\u{df822}';
_11 = _7;
_14 = &_16;
RET = _12;
_5.3 = _9.3 >> _9.2;
_8.1 = 92_i8 ^ 80_i8;
_9.2 = _5.2;
_10 = '\u{83589}';
_16 = 31_u8 as f64;
_8.1 = 15_i8;
_20 = _8.0;
_3 = (_6.0,);
_10 = '\u{232af}';
_9.3 = _5.3;
_17 = _12 as isize;
_3 = _6;
_15 = _17;
_18 = [_13.0];
_14 = &_16;
_7 = !_15;
_4 = RET;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(8_usize, 7_usize, Move(_7), 11_usize, Move(_11), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(8_usize, 1_usize, Move(_1), 9_usize, Move(_9), 3_usize, Move(_3), 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: (bool,)) -> i64 {
mir! {
type RET = i64;
let _2: bool;
let _3: &'static &'static i128;
let _4: ((isize, i32), Adt19, i8, *mut *const u8);
let _5: f32;
let _6: f64;
let _7: [isize; 7];
let _8: [i128; 8];
let _9: &'static f64;
let _10: [u8; 4];
let _11: *mut f64;
let _12: [i16; 6];
let _13: &'static u16;
let _14: &'static [isize; 3];
let _15: f64;
let _16: [u32; 5];
let _17: [char; 5];
let _18: &'static [i128; 4];
let _19: u64;
let _20: (*mut *const u8, &'static &'static i128, i64, *mut u16);
let _21: (isize, i32);
let _22: ([i16; 6],);
let _23: isize;
let _24: *const *const u8;
let _25: u8;
let _26: bool;
let _27: Adt25;
let _28: f32;
let _29: &'static Adt46;
let _30: Adt49;
let _31: [char; 5];
let _32: u128;
let _33: &'static &'static i128;
let _34: ();
let _35: ();
{
RET = (-1923194332017929324_i64);
RET = !(-3333666120481082739_i64);
_1.0 = false;
_1 = (true,);
RET = (-7_i8) as i64;
RET = '\u{107360}' as i64;
_2 = _1.0;
_2 = _1.0;
_2 = _1.0;
_1 = (_2,);
_1.0 = _2;
RET = (-8010880260775389225_i64) * 7886077129529738464_i64;
_1.0 = !_2;
RET = (-4906506986794655434_i64);
_1.0 = _2;
RET = 4584080650276309674_i64;
RET = _1.0 as i64;
RET = (-6787282977233892318_i64);
RET = !2017597128760983109_i64;
_1 = (_2,);
_2 = _1.0 < _1.0;
_4.2 = (-31_i8) ^ (-59_i8);
_4.0.1 = (-1189357825_i32) - 1967628746_i32;
Call(_4.3 = fn10(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.0 = (41_isize, 1497523802_i32);
_4.2 = !122_i8;
_2 = _1.0;
match _4.0.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1497523802 => bb8,
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
_4.1 = Adt19::Variant0 { fld0: _1.0 };
RET = (-73858549327421945347473073249427316593_i128) as i64;
place!(Field::<bool>(Variant(_4.1, 0), 0)) = _2 | _2;
_5 = RET as f32;
_4.0.1 = (-854388602_i32) + (-105756194_i32);
_4.0.0 = _5 as isize;
_5 = 16639_u16 as f32;
_7 = [_4.0.0,_4.0.0,_4.0.0,_4.0.0,_4.0.0,_4.0.0,_4.0.0];
SetDiscriminant(_4.1, 2);
place!(Field::<bool>(Variant(_4.1, 2), 0)) = _2;
_9 = &place!(Field::<(i8, f32, f64)>(Variant(_4.1, 2), 4)).2;
place!(Field::<char>(Variant(_4.1, 2), 1)) = '\u{100a3b}';
RET = 6202573591139507259_i64;
place!(Field::<(isize, i32)>(Variant(_4.1, 2), 7)).1 = !_4.0.1;
place!(Field::<(i8, f32, f64)>(Variant(_4.1, 2), 4)).2 = (-62831820647708725583468203832353533997_i128) as f64;
place!(Field::<isize>(Variant(_4.1, 2), 2)) = !_4.0.0;
_8 = [99668734536675145981225066899616310674_i128,123247670733978600357372109779609948206_i128,(-67682910645666605165495980716541702776_i128),(-73831485994099854278882742502894062982_i128),(-51979829748862922168448535187885910515_i128),(-18491552705855939522215013332205049291_i128),79568696532695594919782526301385259878_i128,12759663423579208525609997973073446485_i128];
place!(Field::<(i8, f32, f64)>(Variant(_4.1, 2), 4)).1 = _4.0.0 as f32;
place!(Field::<u32>(Variant(_4.1, 2), 3)) = 682654436_u32;
_11 = core::ptr::addr_of_mut!(_6);
_8 = [(-30487489090562437349679415422707913874_i128),17321343772711600662490046001067908665_i128,(-64881498799867377245208501372366842460_i128),(-49032763152588501711211494490298324766_i128),(-114626171615242347993488561087273954756_i128),165527268982678962396803235113687834863_i128,153571124359342965636690558512739007463_i128,(-29778312655578408800592618331164132333_i128)];
Goto(bb9)
}
bb9 = {
_4.1 = Adt19::Variant0 { fld0: _1.0 };
_8 = [(-31767707879870961760545471741359020810_i128),(-98021196053743240901977463499649673991_i128),(-7489797137532993103963292506708173778_i128),(-96258911480736699565444267131710522149_i128),(-164218815895175511708530600801189787392_i128),49960571440936431792337551396076595132_i128,39513996710075547228482923420320767172_i128,(-131642600594570804677477016885850635355_i128)];
place!(Field::<bool>(Variant(_4.1, 0), 0)) = _2;
_4.0.1 = (-1334072868_i32) << RET;
_10 = [223_u8,180_u8,218_u8,231_u8];
(*_11) = _4.2 as f64;
_4.2 = 6_i8;
_1.0 = _2;
_6 = 552904588_u32 as f64;
_9 = &_6;
_12 = [(-21789_i16),(-5657_i16),(-15906_i16),26120_i16,15242_i16,25500_i16];
(*_11) = _4.2 as f64;
_4.0.0 = -9223372036854775807_isize;
_15 = _4.0.0 as f64;
_1.0 = !Field::<bool>(Variant(_4.1, 0), 0);
_1.0 = _15 < _15;
place!(Field::<bool>(Variant(_4.1, 0), 0)) = !_1.0;
_8 = [107796532877653368956378409442127648252_i128,(-102786599888324489428118291338798573174_i128),(-162564225321853426677842133461614378975_i128),(-153580654361969703537774576425045270035_i128),(-130833046271368824022779800536636185976_i128),(-131285444716571339928432355743065239491_i128),(-159645768456876532468252013066126656499_i128),(-80523029168526944259113155260254499380_i128)];
_11 = core::ptr::addr_of_mut!(_15);
(*_11) = _4.0.0 as f64;
_8 = [(-45531629936734269165037189512538459365_i128),(-81799286560106856325805134805914951407_i128),(-7138685175086196554385920561549287585_i128),116703631420413120998485149880625382653_i128,40141532005350942922890718921517396581_i128,18673334369878003754115009846167899718_i128,130746068809681954279548629284093498979_i128,87879776272010423312835071947584966441_i128];
place!(Field::<bool>(Variant(_4.1, 0), 0)) = _1.0;
_1.0 = (*_11) >= (*_11);
_5 = 32934_u16 as f32;
_6 = (*_11) + (*_11);
_9 = &(*_11);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
6202573591139507259 => bb10,
_ => bb8
}
}
bb10 = {
_4.0.1 = -(-1779912153_i32);
RET = -(-2074421968024552862_i64);
_5 = 13541_i16 as f32;
_16 = [2783752916_u32,3070347318_u32,2350526491_u32,3395390113_u32,1900358340_u32];
_16 = [1716263058_u32,4033529672_u32,3812965329_u32,2245622889_u32,3393210326_u32];
_16 = [263298565_u32,3464215287_u32,3727879154_u32,2965619019_u32,1351204530_u32];
(*_11) = _6 - _6;
_19 = 17075150612167633113_u64;
_19 = _1.0 as u64;
_19 = !12533134266018127927_u64;
_21.1 = _4.0.1 & _4.0.1;
_10 = [211_u8,77_u8,88_u8,187_u8];
_21.0 = _5 as isize;
match _4.2 {
0 => bb1,
1 => bb2,
2 => bb6,
6 => bb11,
_ => bb4
}
}
bb11 = {
_11 = core::ptr::addr_of_mut!(_6);
_9 = &(*_11);
_12 = [(-13508_i16),(-28834_i16),17807_i16,28299_i16,(-22530_i16),5532_i16];
_15 = (*_9);
_11 = core::ptr::addr_of_mut!(_15);
_4.0 = (_21.0, _21.1);
SetDiscriminant(_4.1, 0);
_20.2 = RET * RET;
_9 = &(*_11);
_21.0 = _4.0.0;
Goto(bb12)
}
bb12 = {
_1 = (_2,);
_11 = core::ptr::addr_of_mut!((*_9));
_16 = [3964897266_u32,1071805673_u32,1091722862_u32,4129037765_u32,64986656_u32];
_8 = [(-166443859523709806505185794684284306759_i128),113559354364830466091564002572905648748_i128,(-35730069020097362549800151134750588940_i128),(-10634544165090379179378116340602416199_i128),17949877120646541028022596286499175066_i128,28347076516191999598148926778528968212_i128,(-117499301167845253670198760728438111001_i128),(-56245664529567493785742027211541607498_i128)];
_5 = (*_9) as f32;
_4.0.1 = !_21.1;
_4.0.1 = !_21.1;
_2 = _4.0.0 == _21.0;
_15 = _21.0 as f64;
_10 = [7_u8,41_u8,162_u8,98_u8];
_21.0 = _4.0.0;
_25 = !118_u8;
place!(Field::<bool>(Variant(_4.1, 0), 0)) = _2;
_9 = &_6;
SetDiscriminant(_4.1, 0);
_19 = _20.2 as u64;
_17 = ['\u{2313e}','\u{172f8}','\u{f65c2}','\u{5745b}','\u{6eeb5}'];
_8 = [(-153612976468624840871623131664084907120_i128),(-133508050746462641391351700906469133750_i128),(-16726506807267236496829627772913029642_i128),49544390363491700062381357446528627717_i128,140118908113028242267847074092442642159_i128,70617035672045250459739744515734422974_i128,(-148393604930227359094587169749399086945_i128),(-79304636317314513875365212663725984313_i128)];
_15 = 148595870732297171911898937977718977491_u128 as f64;
_27.fld2 = _4.0.1 >> _19;
_9 = &(*_9);
_22.0 = _12;
match _4.2 {
0 => bb6,
1 => bb5,
2 => bb3,
3 => bb13,
6 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_8 = [67903540759311027092191913833801045856_i128,(-112554242222388100649839347601554257991_i128),(-24605043384565738395171796000670110230_i128),10197122515334215852460526931455337239_i128,159960098390604027750123669297022264078_i128,(-44970205434690767300805451590524337510_i128),8468177195628930160335719915702637708_i128,(-41192733508863910922282939850008023493_i128)];
_26 = _2;
_19 = _2 as u64;
_4.0 = _21;
_17 = ['\u{6e1e4}','\u{78e4e}','\u{b3885}','\u{1fd29}','\u{c2d3c}'];
_27.fld0 = _22.0;
_27 = Adt25 { fld0: _22.0,fld1: 21775118847087277807092448423699011725_u128,fld2: _21.1 };
_22.0 = [(-7401_i16),31354_i16,3081_i16,(-23856_i16),(-2633_i16),(-17950_i16)];
_27.fld2 = (*_9) as i32;
_23 = _27.fld1 as isize;
_26 = _2;
_31 = _17;
RET = -_20.2;
_4.0 = _21;
_27.fld0 = [3800_i16,(-29876_i16),(-27492_i16),(-7562_i16),(-25849_i16),(-23131_i16)];
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(9_usize, 17_usize, Move(_17), 1_usize, Move(_1), 16_usize, Move(_16), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(9_usize, 22_usize, Move(_22), 31_usize, Move(_31), 10_usize, Move(_10), 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10() -> *mut *const u8 {
mir! {
type RET = *mut *const u8;
let _1: char;
let _2: isize;
let _3: f64;
let _4: i16;
let _5: bool;
let _6: isize;
let _7: &'static [char; 5];
let _8: [u128; 1];
let _9: *mut Adt19;
let _10: (&'static [i32; 1],);
let _11: ([i16; 6],);
let _12: f32;
let _13: u8;
let _14: (*mut *const u8, &'static &'static i128, i64, *mut u16);
let _15: (u128, &'static [char; 5]);
let _16: (isize, i8, f32);
let _17: i32;
let _18: char;
let _19: &'static [isize; 7];
let _20: ();
let _21: ();
{
_1 = '\u{e5cb4}';
_1 = '\u{91709}';
_2 = 109_u8 as isize;
_1 = '\u{ac6f9}';
_4 = (-12306_i16);
_2 = (-63_isize) * (-101_isize);
_3 = 15252840688003630289_usize as f64;
_3 = 182_u8 as f64;
_4 = !27933_i16;
_1 = '\u{10ddfa}';
_1 = '\u{66ce8}';
_5 = !false;
_3 = 4209300485090374158156016611475074520_u128 as f64;
_1 = '\u{abbc9}';
_1 = '\u{259f0}';
_4 = -21085_i16;
_5 = false;
_3 = _2 as f64;
_6 = _2;
_3 = 2636299965914058597_usize as f64;
_5 = _1 >= _1;
_2 = (-663256027_i32) as isize;
_4 = 20680_i16 ^ 30761_i16;
_3 = (-1450473223_i32) as f64;
Call(_4 = core::intrinsics::bswap((-1873_i16)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [231511683613002679899120498814434411989_u128];
_6 = -_2;
_5 = !false;
_1 = '\u{c4e07}';
_8 = [277808388918263855952890011427361997074_u128];
_1 = '\u{1a63}';
_4 = 13132_i16;
_8 = [44977595875694384766874420486900302797_u128];
_2 = _6 | _6;
_1 = '\u{d74ec}';
_2 = _6;
_4 = !(-28513_i16);
_6 = 41495_u16 as isize;
_6 = 160060995388113394671179124958395204849_u128 as isize;
Goto(bb2)
}
bb2 = {
_6 = _2 * _2;
_1 = '\u{7af99}';
_2 = _6;
_5 = !false;
_5 = !true;
_2 = _6 + _6;
_1 = '\u{500}';
_2 = 116_u8 as isize;
_8 = [157435371887317678155990102940530330223_u128];
_1 = '\u{f6338}';
Call(RET = fn11(_3, _8, _2, _1, _1, _6, _4, _2, _8, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = (-156541216122983417_i64) as i16;
_4 = 69590592_u32 as i16;
_5 = true;
_6 = 73035171398020177282594524409158158843_i128 as isize;
_4 = 12322_i16 * 740_i16;
_6 = _2;
_1 = '\u{2670e}';
_5 = !true;
_4 = 15521_i16;
_8 = [143873483193029386961792483105915782726_u128];
_3 = 80046551283472692849558788572637441829_u128 as f64;
_6 = _2;
_3 = 1751999298638388152_i64 as f64;
_1 = '\u{ee54f}';
_1 = '\u{8f8c2}';
_4 = 1924_i16;
_2 = _6;
_8 = [163736899557152441058179376194046651100_u128];
_6 = _2 >> _2;
_11.0 = [_4,_4,_4,_4,_4,_4];
_11.0 = [_4,_4,_4,_4,_4,_4];
_3 = _2 as f64;
_1 = '\u{a7dbd}';
match _4 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
1924 => bb9,
_ => bb8
}
}
bb4 = {
_6 = _2 * _2;
_1 = '\u{7af99}';
_2 = _6;
_5 = !false;
_5 = !true;
_2 = _6 + _6;
_1 = '\u{500}';
_2 = 116_u8 as isize;
_8 = [157435371887317678155990102940530330223_u128];
_1 = '\u{f6338}';
Call(RET = fn11(_3, _8, _2, _1, _1, _6, _4, _2, _8, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_8 = [231511683613002679899120498814434411989_u128];
_6 = -_2;
_5 = !false;
_1 = '\u{c4e07}';
_8 = [277808388918263855952890011427361997074_u128];
_1 = '\u{1a63}';
_4 = 13132_i16;
_8 = [44977595875694384766874420486900302797_u128];
_2 = _6 | _6;
_1 = '\u{d74ec}';
_2 = _6;
_4 = !(-28513_i16);
_6 = 41495_u16 as isize;
_6 = 160060995388113394671179124958395204849_u128 as isize;
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
_4 = _3 as i16;
_4 = (-17448_i16) * 16668_i16;
_1 = '\u{d828c}';
_2 = _6;
_8 = [212724199601481673150380018178020677375_u128];
_8 = [147926320512038734478718338256464087854_u128];
_6 = _2 << _2;
_5 = false | true;
_6 = _4 as isize;
_5 = true ^ true;
_1 = '\u{ec96c}';
_11.0 = [_4,_4,_4,_4,_4,_4];
Goto(bb10)
}
bb10 = {
_6 = 16687629285961935494_u64 as isize;
_2 = !_6;
_3 = 260614224859566212228214118248279276694_u128 as f64;
_5 = _1 <= _1;
_4 = _3 as i16;
_3 = 2354545851229594564_i64 as f64;
_13 = !132_u8;
_12 = (-3464343937159916983709326054767623798_i128) as f32;
_4 = 11075_i16 ^ 13238_i16;
Call(_12 = fn12(_11.0, _11.0, _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_2 = _6;
_13 = 111_u8;
_11.0 = [_4,_4,_4,_4,_4,_4];
_12 = 111699428132257605093519126710044767632_u128 as f32;
_11.0 = [_4,_4,_4,_4,_4,_4];
match _13 {
0 => bb12,
1 => bb13,
2 => bb14,
111 => bb16,
_ => bb15
}
}
bb12 = {
_8 = [231511683613002679899120498814434411989_u128];
_6 = -_2;
_5 = !false;
_1 = '\u{c4e07}';
_8 = [277808388918263855952890011427361997074_u128];
_1 = '\u{1a63}';
_4 = 13132_i16;
_8 = [44977595875694384766874420486900302797_u128];
_2 = _6 | _6;
_1 = '\u{d74ec}';
_2 = _6;
_4 = !(-28513_i16);
_6 = 41495_u16 as isize;
_6 = 160060995388113394671179124958395204849_u128 as isize;
Goto(bb2)
}
bb13 = {
_6 = _2 * _2;
_1 = '\u{7af99}';
_2 = _6;
_5 = !false;
_5 = !true;
_2 = _6 + _6;
_1 = '\u{500}';
_2 = 116_u8 as isize;
_8 = [157435371887317678155990102940530330223_u128];
_1 = '\u{f6338}';
Call(RET = fn11(_3, _8, _2, _1, _1, _6, _4, _2, _8, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_8 = [231511683613002679899120498814434411989_u128];
_6 = -_2;
_5 = !false;
_1 = '\u{c4e07}';
_8 = [277808388918263855952890011427361997074_u128];
_1 = '\u{1a63}';
_4 = 13132_i16;
_8 = [44977595875694384766874420486900302797_u128];
_2 = _6 | _6;
_1 = '\u{d74ec}';
_2 = _6;
_4 = !(-28513_i16);
_6 = 41495_u16 as isize;
_6 = 160060995388113394671179124958395204849_u128 as isize;
Goto(bb2)
}
bb16 = {
_14.2 = 1787969451990929765_i64 | 2051938911610734502_i64;
_8 = [116573006272118641504749794988343677441_u128];
_6 = _2 + _2;
_16.2 = -_12;
_11.0 = [_4,_4,_4,_4,_4,_4];
_16.1 = (-122_i8);
_4 = (-24160_i16);
_3 = 17952198312269979062_u64 as f64;
_11.0 = [_4,_4,_4,_4,_4,_4];
_17 = 295414302_i32;
_16 = (_6, (-40_i8), _12);
_16.2 = _4 as f32;
_13 = _17 as u8;
_1 = '\u{8da3c}';
_1 = '\u{5fc55}';
_3 = _13 as f64;
_17 = (-2593829_i32);
_8 = [165645412052437882707441476218157602066_u128];
_17 = -(-1023276561_i32);
_3 = 2_usize as f64;
_16.2 = _16.1 as f32;
_15.0 = 1114616491_u32 as u128;
_14.2 = 1115267259555673260_i64;
Goto(bb17)
}
bb17 = {
Call(_20 = dump_var(10_usize, 4_usize, Move(_4), 8_usize, Move(_8), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: f64,mut _2: [u128; 1],mut _3: isize,mut _4: char,mut _5: char,mut _6: isize,mut _7: i16,mut _8: isize,mut _9: [u128; 1],mut _10: isize) -> *mut *const u8 {
mir! {
type RET = *mut *const u8;
let _11: u128;
let _12: [i16; 7];
let _13: (i32, bool, u32, i64);
let _14: u16;
let _15: *const [char; 5];
let _16: (i32, bool, u32, i64);
let _17: (isize, i32);
let _18: isize;
let _19: (i32, bool, u32, i64);
let _20: *const u8;
let _21: u128;
let _22: Adt76;
let _23: [i16; 6];
let _24: bool;
let _25: ();
let _26: ();
{
_9 = _2;
_3 = (-13_i8) as isize;
_3 = -_10;
_6 = (-108327053433940332866376344141137210759_i128) as isize;
_7 = !(-22187_i16);
_8 = _3;
_9 = _2;
_3 = _8;
_9 = _2;
_3 = (-67_i8) as isize;
_5 = _4;
_3 = 691663901_i32 as isize;
_4 = _5;
_8 = _10;
_10 = _3 >> _8;
Goto(bb1)
}
bb1 = {
_10 = !_6;
_13.0 = !(-1878797625_i32);
_7 = _4 as i16;
Goto(bb2)
}
bb2 = {
_13 = (15258496_i32, false, 2826689788_u32, (-6004465702377923551_i64));
_1 = 10308770361978233267_usize as f64;
_11 = 238995645999994703866034530689745384449_u128;
_1 = 62774_u16 as f64;
_10 = _13.1 as isize;
_13.0 = -328193340_i32;
_14 = _13.1 as u16;
_8 = !_10;
_11 = 299262598358081223400900228750512117746_u128 >> _13.0;
_13.1 = _13.2 < _13.2;
_13.2 = !2229217860_u32;
_13.2 = 1308860784_u32;
_2 = [_11];
_16.3 = _13.3 + _13.3;
_12 = [_7,_7,_7,_7,_7,_7,_7];
_11 = 19079662291625436967699911848799441057_u128;
_13.3 = _16.3;
_16 = (_13.0, _13.1, _13.2, _13.3);
_11 = !309797613077206112944467296462455627578_u128;
_12 = [_7,_7,_7,_7,_7,_7,_7];
_7 = 9641880570065345752_u64 as i16;
_13 = (_16.0, _16.1, _16.2, _16.3);
_1 = 13300386825734986754_u64 as f64;
_19.0 = 163_u8 as i32;
match _13.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1308860784 => bb10,
_ => bb9
}
}
bb3 = {
_10 = !_6;
_13.0 = !(-1878797625_i32);
_7 = _4 as i16;
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
_17 = (_10, _16.0);
_16.0 = _17.1 - _17.1;
_21 = !_11;
_17 = (_8, _13.0);
_17.1 = _13.0;
_19.3 = _16.3 & _16.3;
_7 = (-7293_i16) - 21558_i16;
match _13.2 {
0 => bb3,
1 => bb8,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
1308860784 => bb17,
_ => bb16
}
}
bb11 = {
Return()
}
bb12 = {
_10 = !_6;
_13.0 = !(-1878797625_i32);
_7 = _4 as i16;
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
_10 = !_6;
_13.0 = !(-1878797625_i32);
_7 = _4 as i16;
Goto(bb2)
}
bb15 = {
_13 = (15258496_i32, false, 2826689788_u32, (-6004465702377923551_i64));
_1 = 10308770361978233267_usize as f64;
_11 = 238995645999994703866034530689745384449_u128;
_1 = 62774_u16 as f64;
_10 = _13.1 as isize;
_13.0 = -328193340_i32;
_14 = _13.1 as u16;
_8 = !_10;
_11 = 299262598358081223400900228750512117746_u128 >> _13.0;
_13.1 = _13.2 < _13.2;
_13.2 = !2229217860_u32;
_13.2 = 1308860784_u32;
_2 = [_11];
_16.3 = _13.3 + _13.3;
_12 = [_7,_7,_7,_7,_7,_7,_7];
_11 = 19079662291625436967699911848799441057_u128;
_13.3 = _16.3;
_16 = (_13.0, _13.1, _13.2, _13.3);
_11 = !309797613077206112944467296462455627578_u128;
_12 = [_7,_7,_7,_7,_7,_7,_7];
_7 = 9641880570065345752_u64 as i16;
_13 = (_16.0, _16.1, _16.2, _16.3);
_1 = 13300386825734986754_u64 as f64;
_19.0 = 163_u8 as i32;
match _13.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1308860784 => bb10,
_ => bb9
}
}
bb16 = {
Return()
}
bb17 = {
_18 = _10 >> _17.0;
_14 = !22009_u16;
_18 = !_17.0;
_17.1 = -_19.0;
_4 = _5;
RET = core::ptr::addr_of_mut!(_20);
_23 = [_7,_7,_7,_7,_7,_7];
_16.3 = _1 as i64;
_8 = _7 as isize;
_14 = 1708_u16 >> _10;
_23 = [_7,_7,_7,_7,_7,_7];
_23 = [_7,_7,_7,_7,_7,_7];
_23 = [_7,_7,_7,_7,_7,_7];
_18 = _17.0;
_19.0 = _17.1;
_13.3 = _16.1 as i64;
_4 = _5;
_2 = [_21];
_8 = _3 - _10;
Goto(bb18)
}
bb18 = {
Call(_25 = dump_var(11_usize, 21_usize, Move(_21), 3_usize, Move(_3), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_25 = dump_var(11_usize, 5_usize, Move(_5), 7_usize, Move(_7), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_25 = dump_var(11_usize, 14_usize, Move(_14), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [i16; 6],mut _2: [i16; 6],mut _3: char) -> f32 {
mir! {
type RET = f32;
let _4: [char; 5];
let _5: (u128, &'static [char; 5]);
let _6: u32;
let _7: u64;
let _8: u32;
let _9: f32;
let _10: f64;
let _11: (f32, u32, (f32, [i16; 6], i16, i16));
let _12: i16;
let _13: u32;
let _14: *mut Adt19;
let _15: &'static [isize; 3];
let _16: isize;
let _17: isize;
let _18: &'static [i128; 4];
let _19: &'static &'static usize;
let _20: char;
let _21: ();
let _22: ();
{
_3 = '\u{3e7ba}';
RET = 2703339114471443833_i64 as f32;
_2 = _1;
_2 = _1;
_4 = [_3,_3,_3,_3,_3];
RET = 245_u8 as f32;
_1 = _2;
RET = 440531080_u32 as f32;
_4 = [_3,_3,_3,_3,_3];
_4 = [_3,_3,_3,_3,_3];
Goto(bb1)
}
bb1 = {
_5.0 = 13588109595241841279922966210580047426_u128;
RET = (-37_isize) as f32;
RET = 76_i8 as f32;
RET = 186_u8 as f32;
_5.0 = 223771151122355211266255788140270924888_u128;
RET = (-6672934780475231085_i64) as f32;
_4 = [_3,_3,_3,_3,_3];
RET = (-75_i8) as f32;
_6 = !3124017252_u32;
_4 = [_3,_3,_3,_3,_3];
_5.1 = &_4;
_5.1 = &_4;
_5.1 = &_4;
_5.0 = 857704470_i32 as u128;
_6 = 3346823692_u32;
_1 = [31060_i16,16781_i16,(-31190_i16),(-2583_i16),25970_i16,(-11779_i16)];
_5.1 = &_4;
RET = 4637642552880887778_u64 as f32;
RET = 35740_u16 as f32;
_5.1 = &_4;
_6 = 10808982077424195192_u64 as u32;
_7 = 11904539198451314085_u64 - 11038527553342416685_u64;
RET = _7 as f32;
_7 = !14418773485707764561_u64;
Goto(bb2)
}
bb2 = {
_4 = [_3,_3,_3,_3,_3];
RET = _7 as f32;
RET = 171_u8 as f32;
_5.1 = &_4;
_5.0 = 261820443905536470145446990293819763740_u128 * 25548445970949892372781320369642271009_u128;
_5.1 = &_4;
_5.0 = 63478755795969882977048606008250333737_u128 - 16435390790731423532769914946569217289_u128;
_5.1 = &_4;
_6 = 612937883_u32;
_7 = RET as u64;
_5.1 = &_4;
_9 = RET;
_9 = (-6878564210159589425997181277834858270_i128) as f32;
_8 = _6 ^ _6;
_8 = !_6;
_2 = [(-10364_i16),29939_i16,18236_i16,15070_i16,(-13767_i16),(-23038_i16)];
_9 = -RET;
_8 = !_6;
_5.1 = &_4;
match _6 {
612937883 => bb4,
_ => bb3
}
}
bb3 = {
_5.0 = 13588109595241841279922966210580047426_u128;
RET = (-37_isize) as f32;
RET = 76_i8 as f32;
RET = 186_u8 as f32;
_5.0 = 223771151122355211266255788140270924888_u128;
RET = (-6672934780475231085_i64) as f32;
_4 = [_3,_3,_3,_3,_3];
RET = (-75_i8) as f32;
_6 = !3124017252_u32;
_4 = [_3,_3,_3,_3,_3];
_5.1 = &_4;
_5.1 = &_4;
_5.1 = &_4;
_5.0 = 857704470_i32 as u128;
_6 = 3346823692_u32;
_1 = [31060_i16,16781_i16,(-31190_i16),(-2583_i16),25970_i16,(-11779_i16)];
_5.1 = &_4;
RET = 4637642552880887778_u64 as f32;
RET = 35740_u16 as f32;
_5.1 = &_4;
_6 = 10808982077424195192_u64 as u32;
_7 = 11904539198451314085_u64 - 11038527553342416685_u64;
RET = _7 as f32;
_7 = !14418773485707764561_u64;
Goto(bb2)
}
bb4 = {
_8 = !_6;
_8 = _6;
_4 = [_3,_3,_3,_3,_3];
_11.2.3 = (-9413_i16);
_12 = !_11.2.3;
_5.1 = &_4;
_6 = !_8;
_3 = '\u{7ae7}';
_5.0 = 87290862785403401612170814148443880521_u128;
_3 = '\u{e77a1}';
match _8 {
0 => bb3,
612937883 => bb5,
_ => bb2
}
}
bb5 = {
_11.2 = (RET, _1, _12, _12);
_5.0 = 317657281380957401906322262838760465208_u128;
_5.0 = 96293823720975630662398804876802707098_u128 >> _11.2.2;
_5.1 = &_4;
RET = _9 * _9;
_11.2.3 = _12 - _11.2.2;
_11.0 = -RET;
_7 = 12474668333055536980_u64 << _8;
_17 = -9223372036854775807_isize;
_4 = [_3,_3,_3,_3,_3];
_13 = 99_u8 as u32;
_6 = !_8;
_13 = !_6;
_4 = [_3,_3,_3,_3,_3];
_17 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_3 = '\u{a912d}';
_16 = _17 << _5.0;
match _8 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
612937883 => bb14,
_ => bb13
}
}
bb6 = {
_8 = !_6;
_8 = _6;
_4 = [_3,_3,_3,_3,_3];
_11.2.3 = (-9413_i16);
_12 = !_11.2.3;
_5.1 = &_4;
_6 = !_8;
_3 = '\u{7ae7}';
_5.0 = 87290862785403401612170814148443880521_u128;
_3 = '\u{e77a1}';
match _8 {
0 => bb3,
612937883 => bb5,
_ => bb2
}
}
bb7 = {
_5.0 = 13588109595241841279922966210580047426_u128;
RET = (-37_isize) as f32;
RET = 76_i8 as f32;
RET = 186_u8 as f32;
_5.0 = 223771151122355211266255788140270924888_u128;
RET = (-6672934780475231085_i64) as f32;
_4 = [_3,_3,_3,_3,_3];
RET = (-75_i8) as f32;
_6 = !3124017252_u32;
_4 = [_3,_3,_3,_3,_3];
_5.1 = &_4;
_5.1 = &_4;
_5.1 = &_4;
_5.0 = 857704470_i32 as u128;
_6 = 3346823692_u32;
_1 = [31060_i16,16781_i16,(-31190_i16),(-2583_i16),25970_i16,(-11779_i16)];
_5.1 = &_4;
RET = 4637642552880887778_u64 as f32;
RET = 35740_u16 as f32;
_5.1 = &_4;
_6 = 10808982077424195192_u64 as u32;
_7 = 11904539198451314085_u64 - 11038527553342416685_u64;
RET = _7 as f32;
_7 = !14418773485707764561_u64;
Goto(bb2)
}
bb8 = {
_4 = [_3,_3,_3,_3,_3];
RET = _7 as f32;
RET = 171_u8 as f32;
_5.1 = &_4;
_5.0 = 261820443905536470145446990293819763740_u128 * 25548445970949892372781320369642271009_u128;
_5.1 = &_4;
_5.0 = 63478755795969882977048606008250333737_u128 - 16435390790731423532769914946569217289_u128;
_5.1 = &_4;
_6 = 612937883_u32;
_7 = RET as u64;
_5.1 = &_4;
_9 = RET;
_9 = (-6878564210159589425997181277834858270_i128) as f32;
_8 = _6 ^ _6;
_8 = !_6;
_2 = [(-10364_i16),29939_i16,18236_i16,15070_i16,(-13767_i16),(-23038_i16)];
_9 = -RET;
_8 = !_6;
_5.1 = &_4;
match _6 {
612937883 => bb4,
_ => bb3
}
}
bb9 = {
_5.0 = 13588109595241841279922966210580047426_u128;
RET = (-37_isize) as f32;
RET = 76_i8 as f32;
RET = 186_u8 as f32;
_5.0 = 223771151122355211266255788140270924888_u128;
RET = (-6672934780475231085_i64) as f32;
_4 = [_3,_3,_3,_3,_3];
RET = (-75_i8) as f32;
_6 = !3124017252_u32;
_4 = [_3,_3,_3,_3,_3];
_5.1 = &_4;
_5.1 = &_4;
_5.1 = &_4;
_5.0 = 857704470_i32 as u128;
_6 = 3346823692_u32;
_1 = [31060_i16,16781_i16,(-31190_i16),(-2583_i16),25970_i16,(-11779_i16)];
_5.1 = &_4;
RET = 4637642552880887778_u64 as f32;
RET = 35740_u16 as f32;
_5.1 = &_4;
_6 = 10808982077424195192_u64 as u32;
_7 = 11904539198451314085_u64 - 11038527553342416685_u64;
RET = _7 as f32;
_7 = !14418773485707764561_u64;
Goto(bb2)
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
_1 = [_12,_11.2.2,_11.2.2,_12,_11.2.3,_11.2.3];
_4 = [_3,_3,_3,_3,_3];
_17 = _7 as isize;
_4 = [_3,_3,_3,_3,_3];
_11.2 = (_11.0, _2, _12, _12);
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(12_usize, 3_usize, Move(_3), 17_usize, Move(_17), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(12_usize, 16_usize, Move(_16), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: i32,mut _3: bool,mut _4: (isize, i32),mut _5: bool,mut _6: bool,mut _7: *mut u16,mut _8: bool) -> bool {
mir! {
type RET = bool;
let _9: Adt54;
let _10: &'static [isize; 3];
let _11: Adt19;
let _12: [u128; 2];
let _13: ();
let _14: ();
{
_2 = _4.1;
_8 = !_6;
RET = _5 & _6;
_4.1 = _2 >> _2;
_8 = _3;
RET = _1;
_4.1 = RET as i32;
_1 = !RET;
_6 = _5 & _3;
_2 = _1 as i32;
_4.1 = -_2;
_4.1 = _2;
RET = _5;
_8 = _1;
_3 = _5;
RET = _4.1 == _4.1;
_2 = _4.1 - _4.1;
RET = _1;
_4.0 = _2 as isize;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(13_usize, 3_usize, Move(_3), 2_usize, Move(_2), 5_usize, Move(_5), 14_usize, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: &'static i64,mut _2: isize,mut _3: [u64; 8],mut _4: u8,mut _5: bool,mut _6: *const u8,mut _7: i8,mut _8: f64,mut _9: (isize, i32),mut _10: *const [char; 5]) -> f32 {
mir! {
type RET = f32;
let _11: &'static Adt46;
let _12: bool;
let _13: usize;
let _14: ((&'static &'static usize, i8, (isize, i32), u8), [char; 5], bool, *const *const u8);
let _15: i64;
let _16: &'static usize;
let _17: [u128; 1];
let _18: [i16; 7];
let _19: (&'static &'static usize, i8, (isize, i32), u8);
let _20: Adt46;
let _21: &'static i128;
let _22: f32;
let _23: [isize; 3];
let _24: f32;
let _25: i128;
let _26: bool;
let _27: char;
let _28: ();
let _29: ();
{
_9 = (_2, 2011511114_i32);
_5 = _8 < _8;
_3 = [18296998080740441365_u64,14973565426193232596_u64,14672957357468592507_u64,18039244214972162487_u64,13026536414899599945_u64,6893470713720119683_u64,13943189584749003826_u64,16916459969283864109_u64];
_8 = 13101_u16 as f64;
_9.1 = (-978254386_i32) * 1690727633_i32;
_3 = [11257789380141778407_u64,12202904266588429015_u64,9111443270333640870_u64,13476157056314186215_u64,12876993608268333695_u64,11308107970063394798_u64,16977558201119217721_u64,2603364568388078572_u64];
RET = 13953418607841864721_usize as f32;
_9.0 = _4 as isize;
_6 = core::ptr::addr_of!(_4);
(*_6) = '\u{b95b7}' as u8;
Goto(bb1)
}
bb1 = {
_2 = !_9.0;
_9.1 = (-654771706_i32);
_2 = !_9.0;
_3 = [504971400767139761_u64,9360000801569112179_u64,9185279420982374122_u64,5960424541774840475_u64,14479141581040860181_u64,10704986029486540178_u64,10984648467567894521_u64,13111439901544722643_u64];
_9 = (_2, 1788493485_i32);
_7 = (-124_i8) & 74_i8;
_14.0.3 = 8957954431713973650_i64 as u8;
(*_6) = _14.0.3 + _14.0.3;
_4 = !_14.0.3;
_2 = !_9.0;
_14.0.0 = &_16;
_9.0 = 1367917067_u32 as isize;
RET = 19831778694720901787776153124674850323_i128 as f32;
_16 = &_13;
_14.0.3 = _5 as u8;
match _9.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1788493485 => bb7,
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
_14.3 = core::ptr::addr_of!(_6);
RET = _8 as f32;
_14.0.1 = !_7;
_10 = core::ptr::addr_of!(_14.1);
_17 = [120968952864681100161530072904377890353_u128];
_12 = !_5;
_14.0.2.1 = _9.1;
(*_10) = ['\u{57fa1}','\u{cdb06}','\u{fcf8a}','\u{c70e}','\u{cb5a6}'];
_19.2.1 = !_9.1;
_19.0 = &_16;
Goto(bb8)
}
bb8 = {
_18 = [(-21498_i16),30407_i16,13060_i16,14393_i16,(-13810_i16),(-27162_i16),5381_i16];
_4 = !_14.0.3;
_9.1 = -_14.0.2.1;
match _14.0.2.1 {
0 => bb9,
1 => bb10,
2 => bb11,
1788493485 => bb13,
_ => bb12
}
}
bb9 = {
Return()
}
bb10 = {
_2 = !_9.0;
_9.1 = (-654771706_i32);
_2 = !_9.0;
_3 = [504971400767139761_u64,9360000801569112179_u64,9185279420982374122_u64,5960424541774840475_u64,14479141581040860181_u64,10704986029486540178_u64,10984648467567894521_u64,13111439901544722643_u64];
_9 = (_2, 1788493485_i32);
_7 = (-124_i8) & 74_i8;
_14.0.3 = 8957954431713973650_i64 as u8;
(*_6) = _14.0.3 + _14.0.3;
_4 = !_14.0.3;
_2 = !_9.0;
_14.0.0 = &_16;
_9.0 = 1367917067_u32 as isize;
RET = 19831778694720901787776153124674850323_i128 as f32;
_16 = &_13;
_14.0.3 = _5 as u8;
match _9.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1788493485 => bb7,
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
_2 = -_9.0;
_19.3 = '\u{51e1a}' as u8;
_19.2.1 = _2 as i32;
_6 = core::ptr::addr_of!((*_6));
_3 = [14192623403653288873_u64,10388785718903409672_u64,12551771119109300632_u64,18080406834260388180_u64,3031833959726264105_u64,11665231614736569682_u64,17695487056351473449_u64,9971566685326770160_u64];
_14.0.0 = &_16;
_19.2 = (_2, _9.1);
_19.1 = _14.0.1;
_14.0.2.0 = !_9.0;
_15 = 8198148354123788674_i64 << _14.0.3;
_4 = _14.0.3 - _14.0.3;
_14.0.0 = &_16;
match _14.0.2.1 {
0 => bb1,
1 => bb8,
1788493485 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_14.2 = _5;
_14.0.0 = &_16;
_17 = [247937448584760835989241192842549680731_u128];
(*_6) = _14.0.3;
(*_6) = !_14.0.3;
_13 = 2421979375148441151_usize;
_14.0.2.0 = RET as isize;
_14.0.2.1 = _9.1 + _19.2.1;
_25 = -76755184141351093424474418227540121226_i128;
_14.0.0 = &_16;
_19.2.0 = _8 as isize;
_14.0 = Move(_19);
_19.2 = (_14.0.2.0, _9.1);
_7 = 39983_u16 as i8;
_19.1 = _4 as i8;
_23 = [_19.2.0,_14.0.2.0,_2];
(*_6) = _25 as u8;
_26 = !_5;
_7 = _9.1 as i8;
_14.0.2.0 = _14.2 as isize;
_22 = RET;
_19.2.0 = _14.0.2.0;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(14_usize, 17_usize, Move(_17), 13_usize, Move(_13), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(14_usize, 18_usize, Move(_18), 12_usize, Move(_12), 15_usize, Move(_15), 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: &'static i64,mut _2: i64,mut _3: u8,mut _4: isize,mut _5: i8,mut _6: [char; 5],mut _7: (isize, i32)) -> u8 {
mir! {
type RET = u8;
let _8: i128;
let _9: bool;
let _10: f64;
let _11: [i8; 4];
let _12: ();
let _13: ();
{
RET = !_3;
RET = !_3;
_7.1 = (-115936266_i32);
_1 = &_2;
_7.0 = _4 | _4;
_6 = ['\u{c50f}','\u{7ed83}','\u{e7f80}','\u{b7faf}','\u{f0bf7}'];
_7.1 = 983360813_i32 | (-1372769466_i32);
_2 = (-1343956369856396914_i64);
_1 = &_2;
_2 = _5 as i64;
_1 = &_2;
_3 = _7.0 as u8;
_8 = !(-169182639481663045634451385800308168699_i128);
_2 = (-8280855678716817794_i64);
Goto(bb1)
}
bb1 = {
_8 = 12355864609060003260382224381987551007_i128 | 25336227141095153171350615404985610296_i128;
RET = _3;
_1 = &_2;
_4 = !_7.0;
_7.1 = (-1122668265_i32);
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(15_usize, 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5), 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *const u8,mut _2: u8,mut _3: u8,mut _4: [char; 5],mut _5: u8,mut _6: *const *const u8,mut _7: *mut [isize; 7],mut _8: *const *const u8,mut _9: i64,mut _10: isize) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _11: i128;
let _12: &'static i128;
let _13: (f32, [i16; 6], i16, i16);
let _14: &'static [char; 5];
let _15: u8;
let _16: bool;
let _17: [u32; 5];
let _18: u64;
let _19: char;
let _20: [i128; 4];
let _21: f32;
let _22: Adt54;
let _23: [u128; 2];
let _24: &'static &'static usize;
let _25: Adt25;
let _26: u128;
let _27: ([i16; 6],);
let _28: [i8; 4];
let _29: ();
let _30: ();
{
_1 = core::ptr::addr_of!(_5);
(*_1) = !_2;
_5 = _3;
RET = [(-164132964118457662939519108266017545291_i128),(-76506555049966082590057883500926471363_i128),(-102871138574971490671232986400653905764_i128),10775071911241996418547521778808633908_i128,(-29925741785733315526988245159493280957_i128),23946555806388449676404569136508209693_i128,120828266170354359983021430785592561955_i128,83043027424210281552737520827354483102_i128];
_1 = core::ptr::addr_of!((*_1));
RET = [48179938760631428074715640791904388583_i128,160656810967662978492148872742401550145_i128,(-56653219790335355045036952166511731733_i128),(-129113928992106301246246613302953719612_i128),157633979200741818117853489254920646975_i128,(-98865070344290053147563658281896573457_i128),62804749961473347179255727911557283941_i128,23271407295585291654938160226326352141_i128];
_6 = core::ptr::addr_of!(_1);
_11 = 88756223055055519575816845551534312367_i128;
_6 = Move(_8);
_11 = 329191740189181369070359592034783221494_u128 as i128;
_2 = false as u8;
Goto(bb1)
}
bb1 = {
(*_1) = !_3;
_6 = core::ptr::addr_of!(_1);
_9 = (-4116224247024801386_i64) | (-7393326891925689253_i64);
_8 = Move(_6);
_6 = core::ptr::addr_of!(_1);
(*_1) = 107370735079508026810760386420524759104_u128 as u8;
_10 = 9223372036854775807_isize;
_4 = ['\u{fc16c}','\u{98e11}','\u{2430a}','\u{1031f0}','\u{9f73f}'];
_3 = !_5;
(*_6) = core::ptr::addr_of!(_3);
_13.1 = [(-22135_i16),(-26139_i16),19777_i16,(-13239_i16),30574_i16,6644_i16];
Goto(bb2)
}
bb2 = {
(*_1) = 84647687_i32 as u8;
_3 = !_2;
(*_1) = 1624961590_u32 as u8;
(*_6) = core::ptr::addr_of!(_15);
_14 = &_4;
_13.3 = (-2986_i16);
_13.2 = -_13.3;
_8 = core::ptr::addr_of!(_1);
_6 = core::ptr::addr_of!((*_8));
(*_1) = _5;
_8 = Move(_6);
_12 = &_11;
_17 = [877948909_u32,1653165418_u32,1513268890_u32,3902891824_u32,2087240917_u32];
_13.0 = 170556539737051499609260080366634640609_u128 as f32;
(*_1) = 73464927280771660237654651183746606201_u128 as u8;
RET = [_11,(*_12),(*_12),(*_12),(*_12),_11,(*_12),(*_12)];
_9 = (*_12) as i64;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb3 = {
(*_1) = !_3;
_6 = core::ptr::addr_of!(_1);
_9 = (-4116224247024801386_i64) | (-7393326891925689253_i64);
_8 = Move(_6);
_6 = core::ptr::addr_of!(_1);
(*_1) = 107370735079508026810760386420524759104_u128 as u8;
_10 = 9223372036854775807_isize;
_4 = ['\u{fc16c}','\u{98e11}','\u{2430a}','\u{1031f0}','\u{9f73f}'];
_3 = !_5;
(*_6) = core::ptr::addr_of!(_3);
_13.1 = [(-22135_i16),(-26139_i16),19777_i16,(-13239_i16),30574_i16,6644_i16];
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
_6 = core::ptr::addr_of!(_1);
_4 = ['\u{c84cd}','\u{10f5e}','\u{83853}','\u{b4bb5}','\u{f4807}'];
_13.2 = _13.3 + _13.3;
_8 = core::ptr::addr_of!((*_6));
match _10 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
9223372036854775807 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_5 = !_3;
_11 = 152019322384268407566923003361343072236_i128;
_13.0 = (*_1) as f32;
(*_8) = core::ptr::addr_of!(_3);
_20 = [_11,_11,_11,_11];
(*_1) = !_5;
_9 = !1113386834271400969_i64;
_6 = core::ptr::addr_of!((*_8));
_13.1 = [_13.3,_13.3,_13.2,_13.2,_13.2,_13.2];
_16 = _3 >= _2;
(*_6) = core::ptr::addr_of!(_2);
_12 = &_11;
(*_8) = core::ptr::addr_of!(_5);
_20 = [_11,_11,_11,_11];
(*_1) = !_2;
(*_6) = core::ptr::addr_of!(_15);
_19 = '\u{6181b}';
(*_1) = _3 << _3;
_13.3 = _13.2;
_16 = false;
_23 = [339584083290497494364163248379012556959_u128,161972962920263289705824295334716645280_u128];
_18 = 5050630543874231017_u64;
_2 = _15;
(*_1) = _2;
_17 = [1856832950_u32,1764095307_u32,826311552_u32,1402065744_u32,3245596764_u32];
_13.3 = _13.0 as i16;
_8 = core::ptr::addr_of!((*_6));
match _18 {
0 => bb10,
1 => bb11,
5050630543874231017 => bb13,
_ => bb12
}
}
bb10 = {
(*_1) = 84647687_i32 as u8;
_3 = !_2;
(*_1) = 1624961590_u32 as u8;
(*_6) = core::ptr::addr_of!(_15);
_14 = &_4;
_13.3 = (-2986_i16);
_13.2 = -_13.3;
_8 = core::ptr::addr_of!(_1);
_6 = core::ptr::addr_of!((*_8));
(*_1) = _5;
_8 = Move(_6);
_12 = &_11;
_17 = [877948909_u32,1653165418_u32,1513268890_u32,3902891824_u32,2087240917_u32];
_13.0 = 170556539737051499609260080366634640609_u128 as f32;
(*_1) = 73464927280771660237654651183746606201_u128 as u8;
RET = [_11,(*_12),(*_12),(*_12),(*_12),_11,(*_12),(*_12)];
_9 = (*_12) as i64;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
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
_11 = !(-73641909967245851273716106124479624562_i128);
match _18 {
0 => bb10,
1 => bb14,
2 => bb15,
3 => bb16,
5050630543874231017 => bb18,
_ => bb17
}
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
_6 = core::ptr::addr_of!(_1);
_4 = ['\u{c84cd}','\u{10f5e}','\u{83853}','\u{b4bb5}','\u{f4807}'];
_13.2 = _13.3 + _13.3;
_8 = core::ptr::addr_of!((*_6));
match _10 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
9223372036854775807 => bb9,
_ => bb8
}
}
bb18 = {
_21 = _13.0;
_9 = 54852_u16 as i64;
_25.fld2 = !(-585321893_i32);
_15 = _3;
_19 = '\u{494cf}';
_26 = 152625671433449116157894956286475345582_u128 * 115906755642126072478661128953574992044_u128;
_20 = [_11,_11,_11,_11];
_2 = _15;
_9 = _13.0 as i64;
_17 = [2117775769_u32,22521117_u32,3005876301_u32,3295438906_u32,3787393151_u32];
_15 = _2 ^ _5;
_25.fld1 = !_26;
_6 = Move(_8);
_25 = Adt25 { fld0: _13.1,fld1: _26,fld2: (-9542963_i32) };
_6 = core::ptr::addr_of!(_1);
_8 = core::ptr::addr_of!((*_6));
_11 = !(-126981963032784200634909602840652630398_i128);
_15 = (-5_i8) as u8;
_21 = _13.0 * _13.0;
_13.0 = _11 as f32;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
(*_6) = core::ptr::addr_of!((*_1));
Goto(bb19)
}
bb19 = {
Call(_29 = dump_var(16_usize, 17_usize, Move(_17), 4_usize, Move(_4), 11_usize, Move(_11), 26_usize, Move(_26)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_29 = dump_var(16_usize, 20_usize, Move(_20), 10_usize, Move(_10), 5_usize, Move(_5), 30_usize, _30), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(164717463212381343074759925478725920730_u128), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(97_i8), std::hint::black_box(27337_i16), std::hint::black_box(1970386665_i32), std::hint::black_box(2762551790223986180_u64), std::hint::black_box(15382172948728666836516865427391228337_i128), std::hint::black_box(3_usize), std::hint::black_box(43_u8), std::hint::black_box(35516_u16), std::hint::black_box(353034865_u32));
                
            }
impl PrintFDebug for Adt19{
	unsafe fn printf_debug(&self){unsafe{printf("Adt19::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt19 {
Variant0{
fld0: bool,

},
Variant1{
fld0: bool,
fld1: u64,
fld2: usize,
fld3: i8,
fld4: u32,
fld5: (isize, i32),
fld6: f64,
fld7: i128,

},
Variant2{
fld0: bool,
fld1: char,
fld2: isize,
fld3: u32,
fld4: (i8, f32, f64),
fld5: i32,
fld6: i64,
fld7: (isize, i32),

},
Variant3{
fld0: f32,
fld1: (isize, i32),
fld2: (i8, f32, f64),
fld3: i8,
fld4: u16,
fld5: usize,
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt25{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt25{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt25 {
fld0: [i16; 6],
fld1: u128,
fld2: i32,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f64,
fld1: *const *mut u16,
fld2: usize,
fld3: *const usize,
fld4: *const u8,
fld5: [i128; 4],

},
Variant1{
fld0: [i128; 4],

},
Variant2{
fld0: f64,
fld1: u8,
fld2: *mut Adt19,
fld3: usize,
fld4: *mut f64,

},
Variant3{
fld0: (i8, f32, f64),
fld1: [isize; 7],
fld2: Adt25,
fld3: i8,
fld4: i16,
fld5: u32,
fld6: u8,
fld7: usize,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [i128; 4],
fld1: u8,
fld2: isize,

},
Variant1{
fld0: f64,
fld1: (isize, i8, f32),
fld2: u16,
fld3: [i32; 1],
fld4: i16,
fld5: *mut *const u8,
fld6: Adt19,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt25,
fld1: u32,
fld2: isize,

},
Variant1{
fld0: u8,
fld1: [i16; 6],
fld2: *const u8,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *mut u16,
fld1: [isize; 3],
fld2: *mut f64,

},
Variant1{
fld0: u32,
fld1: char,
fld2: (f32, [i16; 6], i16, i16),
fld3: *const *mut u16,
fld4: f64,
fld5: [char; 5],
fld6: u64,

},
Variant2{
fld0: bool,
fld1: [char; 5],
fld2: (isize, i32),
fld3: u8,
fld4: [i128; 8],
fld5: i32,
fld6: i64,

},
Variant3{
fld0: [i128; 4],
fld1: Adt25,
fld2: (isize, i32),
fld3: [isize; 3],
fld4: i16,
fld5: i32,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [i128; 4],
fld1: Adt49,
fld2: f64,
fld3: Adt46,
fld4: *mut u16,
fld5: f32,

},
Variant1{
fld0: (i32, bool, u32, i64),
fld1: Adt51,
fld2: [i128; 4],

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){unsafe{printf("Adt76::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt76 {
Variant0{
fld0: *const u8,
fld1: [u64; 8],

},
Variant1{
fld0: *mut Adt19,
fld1: (bool,),
fld2: *const *mut u16,
fld3: [isize; 7],
fld4: [i8; 4],

}}

