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
pub fn fn0(mut _1: i16,mut _2: char,mut _3: isize) -> f32 {
mir! {
type RET = f32;
let _4: isize;
let _5: f64;
let _6: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _7: (([isize; 1], (char, f64), f32), ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), [u16; 6]);
let _8: Adt31;
let _9: bool;
let _10: i64;
let _11: bool;
let _12: Adt52;
let _13: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31);
let _14: usize;
let _15: f64;
let _16: (u8, u64, [i32; 4], (char, u64, *mut isize));
let _17: (u8, u64, [i32; 4], (char, u64, *mut isize));
let _18: isize;
let _19: (i8, isize, Adt18);
let _20: u64;
let _21: &'static [bool; 3];
let _22: f64;
let _23: f64;
let _24: &'static isize;
let _25: ();
let _26: ();
{
RET = (-5861683883862047392_i64) as f32;
RET = (-1560475387_i32) as f32;
_4 = 4080696418_u32 as isize;
_3 = _4;
_5 = 28720514_i32 as f64;
RET = 194_u8 as f32;
Goto(bb1)
}
bb1 = {
RET = _5 as f32;
_5 = 74218669_i32 as f64;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 298160451354682795704443670238562059850_u128 };
_7.1.2.fld1.1 = 14090969541776406752_u64 & 4831399397294418400_u64;
_7.1.2.fld4 = [8_i8,(-6_i8),44_i8,(-58_i8)];
_7.1.0 = [_7.1.2.fld1.1];
_7.0.0 = [_4];
RET = (-30630_i16) as f32;
place!(Field::<u128>(Variant(_7.1.1.2, 2), 1)) = !231425898831953142884083619374321252367_u128;
_7.0.0 = [_3];
_7.1.2.fld1.1 = !16408082988837807664_u64;
_7.1.2.fld1.0 = '\u{c0b2}';
_7.1.1.2 = Adt18::Variant1 { fld0: _7.1.2.fld1.1,fld1: _5,fld2: _4,fld3: _7.0.0,fld4: 1898070032492953975_i64,fld5: RET };
Call(_7.1.2.fld6 = fn1(Field::<isize>(Variant(_7.1.1.2, 1), 2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.1.2.fld1.1 = !Field::<u64>(Variant(_7.1.1.2, 1), 0);
_7.0.1 = (_7.1.2.fld1.0, _5);
_7.1.2.fld7 = core::ptr::addr_of_mut!(_8.fld6.1);
_8.fld5.0 = _7.1.2.fld1.0 as i128;
_7.1.1 = _7.1.2.fld6;
_7.1.2.fld5 = (_8.fld5.0,);
_7.1.2.fld3 = !_7.1.1.0;
_8.fld3 = _7.1.2.fld3 ^ _7.1.2.fld6.0;
_8.fld1.0 = _7.1.2.fld1.0;
_7.1.2.fld6 = _7.1.1;
_9 = Field::<bool>(Variant(_7.1.2.fld6.2, 2), 0);
_7.1.1.2 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_7.1.2.fld6.2, 2), 0) };
_7.1.2.fld6 = (_7.1.1.0, _7.1.1.1, _7.1.1.2);
_8.fld6.2 = _7.1.1.2;
_8.fld4 = [_8.fld3,_8.fld3,_8.fld3,_7.1.2.fld3];
_6 = core::ptr::addr_of!(_7.1);
(*_6).2.fld3 = 7_usize as i8;
Goto(bb3)
}
bb3 = {
(*_6).3 = Adt20::Variant1 { fld0: _7.0.1.1,fld1: _8.fld1.0,fld2: 78521255211072919880435381289649380523_u128,fld3: _7.1.2.fld6.0,fld4: (*_6).2.fld1.1,fld5: 1042235602_i32,fld6: 2607589356_u32 };
_7.1.2.fld3 = (*_6).1.0 | (*_6).1.0;
place!(Field::<i32>(Variant((*_6).3, 1), 5)) = 1872068985_i32;
(*_6).2.fld0 = 6333833416272591051_usize;
_8.fld6 = (Field::<i8>(Variant((*_6).3, 1), 3), _7.1.2.fld6.1, (*_6).1.2);
place!(Field::<u32>(Variant(_7.1.3, 1), 6)) = !426440895_u32;
place!(Field::<bool>(Variant(_8.fld6.2, 3), 0)) = !Field::<bool>(Variant((*_6).1.2, 3), 0);
(*_6).1.0 = !Field::<i8>(Variant((*_6).3, 1), 3);
_8.fld6.1 = !_7.1.2.fld6.1;
place!(Field::<u128>(Variant((*_6).3, 1), 2)) = 137351534645216808071767689835893942736_u128;
_8.fld1.0 = (*_6).2.fld1.0;
(*_6).3 = Adt20::Variant1 { fld0: _5,fld1: (*_6).2.fld1.0,fld2: 137733279789555128612280800558900340211_u128,fld3: _7.1.2.fld6.0,fld4: (*_6).2.fld1.1,fld5: (-1366343271_i32),fld6: 36864967_u32 };
_7.1.3 = Adt20::Variant1 { fld0: _5,fld1: _7.0.1.0,fld2: 296600941255774757308428971775998560848_u128,fld3: (*_6).2.fld3,fld4: (*_6).2.fld1.1,fld5: (-616483022_i32),fld6: 1098307107_u32 };
place!(Field::<i32>(Variant((*_6).3, 1), 5)) = -402767297_i32;
_7.2 = [5255_u16,27087_u16,54240_u16,56164_u16,8083_u16,13957_u16];
_7.1.3 = Adt20::Variant0 { fld0: _8.fld6.2,fld1: _7.0.0,fld2: (*_6).2.fld1.1,fld3: _7.1.2.fld3,fld4: 14282_i16,fld5: 3521790667_u32,fld6: 3921524295018391391_i64,fld7: (*_6).2.fld5.0 };
place!(Field::<i64>(Variant((*_6).3, 0), 6)) = (-7082776736655539330_i64) + (-4621661494127979578_i64);
place!(Field::<i8>(Variant(_7.1.3, 0), 3)) = (*_6).1.0;
_13.1.2.fld6.1 = _5 as isize;
Goto(bb4)
}
bb4 = {
place!(Field::<i64>(Variant(_7.1.3, 0), 6)) = _7.1.2.fld1.0 as i64;
_13.1.2.fld1.0 = (*_6).2.fld1.0;
_13.3.fld1.1 = Field::<u64>(Variant(_7.1.3, 0), 2);
_8.fld2 = Field::<[isize; 1]>(Variant((*_6).3, 0), 1);
match _7.1.2.fld0 {
6333833416272591051 => bb6,
_ => bb5
}
}
bb5 = {
RET = _5 as f32;
_5 = 74218669_i32 as f64;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 298160451354682795704443670238562059850_u128 };
_7.1.2.fld1.1 = 14090969541776406752_u64 & 4831399397294418400_u64;
_7.1.2.fld4 = [8_i8,(-6_i8),44_i8,(-58_i8)];
_7.1.0 = [_7.1.2.fld1.1];
_7.0.0 = [_4];
RET = (-30630_i16) as f32;
place!(Field::<u128>(Variant(_7.1.1.2, 2), 1)) = !231425898831953142884083619374321252367_u128;
_7.0.0 = [_3];
_7.1.2.fld1.1 = !16408082988837807664_u64;
_7.1.2.fld1.0 = '\u{c0b2}';
_7.1.1.2 = Adt18::Variant1 { fld0: _7.1.2.fld1.1,fld1: _5,fld2: _4,fld3: _7.0.0,fld4: 1898070032492953975_i64,fld5: RET };
Call(_7.1.2.fld6 = fn1(Field::<isize>(Variant(_7.1.1.2, 1), 2)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_13.3.fld6.2 = (*_6).1.2;
_3 = (*_6).2.fld1.1 as isize;
_13.3.fld3 = _7.1.2.fld6.1 as i8;
(*_6).2.fld5 = _8.fld5;
(*_6).2.fld4 = [Field::<i8>(Variant((*_6).3, 0), 3),(*_6).2.fld6.0,(*_6).2.fld6.0,_7.1.1.0];
place!(Field::<i8>(Variant((*_6).3, 0), 3)) = (*_6).2.fld3 >> (*_6).2.fld6.1;
(*_6).2.fld1 = (_7.0.1.0, Field::<u64>(Variant((*_6).3, 0), 2), Move((*_6).2.fld7));
_16.3.0 = _8.fld1.0;
SetDiscriminant(Field::<Adt18>(Variant((*_6).3, 0), 0), 2);
Goto(bb7)
}
bb7 = {
place!(Field::<u32>(Variant((*_6).3, 0), 5)) = 2999753354_u32 * 3069284551_u32;
_4 = !_7.1.1.1;
_17.3.2 = core::ptr::addr_of_mut!((*_6).1.1);
place!(Field::<bool>(Variant((*_6).1.2, 3), 0)) = !Field::<bool>(Variant(_7.1.2.fld6.2, 3), 0);
_13.1.2.fld5.0 = _8.fld5.0 ^ (*_6).2.fld5.0;
_13.1.2.fld4 = _7.1.2.fld4;
_7.1.2.fld1.0 = _16.3.0;
match _7.1.2.fld0 {
6333833416272591051 => bb9,
_ => bb8
}
}
bb8 = {
place!(Field::<i64>(Variant(_7.1.3, 0), 6)) = _7.1.2.fld1.0 as i64;
_13.1.2.fld1.0 = (*_6).2.fld1.0;
_13.3.fld1.1 = Field::<u64>(Variant(_7.1.3, 0), 2);
_8.fld2 = Field::<[isize; 1]>(Variant((*_6).3, 0), 1);
match _7.1.2.fld0 {
6333833416272591051 => bb6,
_ => bb5
}
}
bb9 = {
_13.1.2.fld5.0 = (*_6).2.fld1.0 as i128;
(*_6).1.1 = (*_6).2.fld6.1 * _4;
place!(Field::<u64>(Variant((*_6).3, 0), 2)) = (*_6).2.fld1.1 - _13.3.fld1.1;
place!(Field::<u128>(Variant(place!(Field::<Adt18>(Variant((*_6).3, 0), 0)), 2), 1)) = _13.1.2.fld6.1 as u128;
place!(Field::<Adt18>(Variant(_7.1.3, 0), 0)) = _7.1.1.2;
_16.3.2 = core::ptr::addr_of_mut!(_7.1.2.fld6.1);
(*_6).0 = [Field::<u64>(Variant((*_6).3, 0), 2)];
Goto(bb10)
}
bb10 = {
(*_6).0 = [Field::<u64>(Variant(_7.1.3, 0), 2)];
_13.3.fld0 = 60997_u16 as usize;
_13.1.2.fld3 = (*_6).2.fld5.0 as i8;
_13.1.2.fld6.1 = (*_6).1.1;
(*_6).1 = (Field::<i8>(Variant(_7.1.3, 0), 3), _13.1.2.fld6.1, (*_6).2.fld6.2);
_7.1.2.fld6 = (Field::<i8>(Variant((*_6).3, 0), 3), _7.1.1.1, _13.3.fld6.2);
_16.0 = !95_u8;
match (*_6).2.fld0 {
0 => bb6,
1 => bb5,
6333833416272591051 => bb12,
_ => bb11
}
}
bb11 = {
RET = _5 as f32;
_5 = 74218669_i32 as f64;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 298160451354682795704443670238562059850_u128 };
_7.1.2.fld1.1 = 14090969541776406752_u64 & 4831399397294418400_u64;
_7.1.2.fld4 = [8_i8,(-6_i8),44_i8,(-58_i8)];
_7.1.0 = [_7.1.2.fld1.1];
_7.0.0 = [_4];
RET = (-30630_i16) as f32;
place!(Field::<u128>(Variant(_7.1.1.2, 2), 1)) = !231425898831953142884083619374321252367_u128;
_7.0.0 = [_3];
_7.1.2.fld1.1 = !16408082988837807664_u64;
_7.1.2.fld1.0 = '\u{c0b2}';
_7.1.1.2 = Adt18::Variant1 { fld0: _7.1.2.fld1.1,fld1: _5,fld2: _4,fld3: _7.0.0,fld4: 1898070032492953975_i64,fld5: RET };
Call(_7.1.2.fld6 = fn1(Field::<isize>(Variant(_7.1.1.2, 1), 2)), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
(*_6).3 = Adt20::Variant0 { fld0: (*_6).1.2,fld1: _7.0.0,fld2: _13.3.fld1.1,fld3: (*_6).2.fld6.0,fld4: 998_i16,fld5: 2145030039_u32,fld6: 6652532718089920520_i64,fld7: _13.1.2.fld5.0 };
_16.3.1 = Field::<u64>(Variant((*_6).3, 0), 2);
_13.1.0 = [(*_6).2.fld1.1];
_13.3.fld5 = (Field::<i128>(Variant((*_6).3, 0), 7),);
_13.3.fld6 = (Field::<i8>(Variant((*_6).3, 0), 3), (*_6).1.1, _7.1.2.fld6.2);
(*_6).2.fld1.2 = Move(_17.3.2);
_7.1.2.fld1.0 = _16.3.0;
Call(_17 = fn19((*_6).2.fld6.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_7.1.2.fld5 = (_13.3.fld5.0,);
match (*_6).2.fld0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6333833416272591051 => bb21,
_ => bb20
}
}
bb14 = {
(*_6).3 = Adt20::Variant0 { fld0: (*_6).1.2,fld1: _7.0.0,fld2: _13.3.fld1.1,fld3: (*_6).2.fld6.0,fld4: 998_i16,fld5: 2145030039_u32,fld6: 6652532718089920520_i64,fld7: _13.1.2.fld5.0 };
_16.3.1 = Field::<u64>(Variant((*_6).3, 0), 2);
_13.1.0 = [(*_6).2.fld1.1];
_13.3.fld5 = (Field::<i128>(Variant((*_6).3, 0), 7),);
_13.3.fld6 = (Field::<i8>(Variant((*_6).3, 0), 3), (*_6).1.1, _7.1.2.fld6.2);
(*_6).2.fld1.2 = Move(_17.3.2);
_7.1.2.fld1.0 = _16.3.0;
Call(_17 = fn19((*_6).2.fld6.1), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
RET = _5 as f32;
_5 = 74218669_i32 as f64;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 298160451354682795704443670238562059850_u128 };
_7.1.2.fld1.1 = 14090969541776406752_u64 & 4831399397294418400_u64;
_7.1.2.fld4 = [8_i8,(-6_i8),44_i8,(-58_i8)];
_7.1.0 = [_7.1.2.fld1.1];
_7.0.0 = [_4];
RET = (-30630_i16) as f32;
place!(Field::<u128>(Variant(_7.1.1.2, 2), 1)) = !231425898831953142884083619374321252367_u128;
_7.0.0 = [_3];
_7.1.2.fld1.1 = !16408082988837807664_u64;
_7.1.2.fld1.0 = '\u{c0b2}';
_7.1.1.2 = Adt18::Variant1 { fld0: _7.1.2.fld1.1,fld1: _5,fld2: _4,fld3: _7.0.0,fld4: 1898070032492953975_i64,fld5: RET };
Call(_7.1.2.fld6 = fn1(Field::<isize>(Variant(_7.1.1.2, 1), 2)), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
(*_6).0 = [Field::<u64>(Variant(_7.1.3, 0), 2)];
_13.3.fld0 = 60997_u16 as usize;
_13.1.2.fld3 = (*_6).2.fld5.0 as i8;
_13.1.2.fld6.1 = (*_6).1.1;
(*_6).1 = (Field::<i8>(Variant(_7.1.3, 0), 3), _13.1.2.fld6.1, (*_6).2.fld6.2);
_7.1.2.fld6 = (Field::<i8>(Variant((*_6).3, 0), 3), _7.1.1.1, _13.3.fld6.2);
_16.0 = !95_u8;
match (*_6).2.fld0 {
0 => bb6,
1 => bb5,
6333833416272591051 => bb12,
_ => bb11
}
}
bb17 = {
_13.1.2.fld5.0 = (*_6).2.fld1.0 as i128;
(*_6).1.1 = (*_6).2.fld6.1 * _4;
place!(Field::<u64>(Variant((*_6).3, 0), 2)) = (*_6).2.fld1.1 - _13.3.fld1.1;
place!(Field::<u128>(Variant(place!(Field::<Adt18>(Variant((*_6).3, 0), 0)), 2), 1)) = _13.1.2.fld6.1 as u128;
place!(Field::<Adt18>(Variant(_7.1.3, 0), 0)) = _7.1.1.2;
_16.3.2 = core::ptr::addr_of_mut!(_7.1.2.fld6.1);
(*_6).0 = [Field::<u64>(Variant((*_6).3, 0), 2)];
Goto(bb10)
}
bb18 = {
place!(Field::<i64>(Variant(_7.1.3, 0), 6)) = _7.1.2.fld1.0 as i64;
_13.1.2.fld1.0 = (*_6).2.fld1.0;
_13.3.fld1.1 = Field::<u64>(Variant(_7.1.3, 0), 2);
_8.fld2 = Field::<[isize; 1]>(Variant((*_6).3, 0), 1);
match _7.1.2.fld0 {
6333833416272591051 => bb6,
_ => bb5
}
}
bb19 = {
place!(Field::<u32>(Variant((*_6).3, 0), 5)) = 2999753354_u32 * 3069284551_u32;
_4 = !_7.1.1.1;
_17.3.2 = core::ptr::addr_of_mut!((*_6).1.1);
place!(Field::<bool>(Variant((*_6).1.2, 3), 0)) = !Field::<bool>(Variant(_7.1.2.fld6.2, 3), 0);
_13.1.2.fld5.0 = _8.fld5.0 ^ (*_6).2.fld5.0;
_13.1.2.fld4 = _7.1.2.fld4;
_7.1.2.fld1.0 = _16.3.0;
match _7.1.2.fld0 {
6333833416272591051 => bb9,
_ => bb8
}
}
bb20 = {
_13.3.fld6.2 = (*_6).1.2;
_3 = (*_6).2.fld1.1 as isize;
_13.3.fld3 = _7.1.2.fld6.1 as i8;
(*_6).2.fld5 = _8.fld5;
(*_6).2.fld4 = [Field::<i8>(Variant((*_6).3, 0), 3),(*_6).2.fld6.0,(*_6).2.fld6.0,_7.1.1.0];
place!(Field::<i8>(Variant((*_6).3, 0), 3)) = (*_6).2.fld3 >> (*_6).2.fld6.1;
(*_6).2.fld1 = (_7.0.1.0, Field::<u64>(Variant((*_6).3, 0), 2), Move((*_6).2.fld7));
_16.3.0 = _8.fld1.0;
SetDiscriminant(Field::<Adt18>(Variant((*_6).3, 0), 0), 2);
Goto(bb7)
}
bb21 = {
place!(Field::<i8>(Variant(_7.1.3, 0), 3)) = -(*_6).2.fld3;
place!(Field::<i64>(Variant((*_6).3, 0), 6)) = _7.1.2.fld0 as i64;
_8 = Adt31 { fld0: _7.1.2.fld0,fld1: Move((*_6).2.fld1),fld2: Field::<[isize; 1]>(Variant((*_6).3, 0), 1),fld3: _7.1.1.0,fld4: _7.1.2.fld4,fld5: _13.3.fld5,fld6: (*_6).2.fld6,fld7: Move(_16.3.2) };
place!(Field::<bool>(Variant((*_6).2.fld6.2, 3), 0)) = Field::<bool>(Variant((*_6).1.2, 3), 0);
(*_6).2.fld1.2 = core::ptr::addr_of_mut!((*_6).1.1);
_13.3.fld1 = Move(_17.3);
_17.3.1 = 54639_u16 as u64;
(*_6).2.fld6.2 = Adt18::Variant0 { fld0: _17.0,fld1: _13.3.fld5.0,fld2: 24597_u16 };
SetDiscriminant(Field::<Adt18>(Variant(_7.1.3, 0), 0), 1);
Goto(bb22)
}
bb22 = {
Call(_25 = dump_var(0_usize, 4_usize, Move(_4), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize) -> (i8, isize, Adt18) {
mir! {
type RET = (i8, isize, Adt18);
let _2: u128;
let _3: f64;
let _4: &'static isize;
let _5: [usize; 2];
let _6: isize;
let _7: char;
let _8: i32;
let _9: *mut isize;
let _10: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _11: Adt62;
let _12: ((&'static &'static u8, usize), &'static &'static u8, *const u16, (i128,));
let _13: [i32; 5];
let _14: isize;
let _15: i32;
let _16: [u64; 1];
let _17: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _18: ();
let _19: ();
{
RET.0 = 72_i8 >> _1;
_1 = (-26_isize) ^ 45_isize;
RET.1 = -_1;
_2 = 184471235198994461628231342177468312798_u128 ^ 337906379079069524427043981897318327501_u128;
Goto(bb1)
}
bb1 = {
RET.0 = 17820847099516213911_usize as i8;
_2 = 271278150756431932221486946694713577917_u128 & 36721402185901929092156703203525692725_u128;
RET.0 = (-40_i8);
RET.1 = _1;
_1 = RET.1;
_2 = 206165386753532554226685540624766411173_u128 | 68835754827304157772407204450452939830_u128;
match RET.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211416 => bb9,
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
_3 = RET.0 as f64;
RET.2 = Adt18::Variant3 { fld0: false };
RET.1 = _1 + _1;
RET.1 = _1 + _1;
RET.2 = Adt18::Variant2 { fld0: true,fld1: _2 };
_4 = &_1;
_4 = &(*_4);
_5 = [344774410406797303_usize,15421581477168466828_usize];
place!(Field::<u128>(Variant(RET.2, 2), 1)) = !_2;
place!(Field::<bool>(Variant(RET.2, 2), 0)) = (*_4) < RET.1;
_2 = Field::<u128>(Variant(RET.2, 2), 1);
_4 = &_1;
RET.0 = 494516970606722340_i64 as i8;
RET.2 = Adt18::Variant0 { fld0: 76_u8,fld1: 18166525400272524878459556865227010524_i128,fld2: 44505_u16 };
RET.2 = Adt18::Variant0 { fld0: 160_u8,fld1: (-93643042826774890100539640853563436541_i128),fld2: 21535_u16 };
_3 = 4478562850027180188_usize as f64;
RET.2 = Adt18::Variant3 { fld0: true };
RET.0 = 99_i8;
_4 = &(*_4);
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_3 = 97_u8 as f64;
_7 = '\u{99b89}';
_10.2.3.0 = [4_usize,5585781460594852002_usize];
_10.1.2.fld5 = ((-68287210355678425858222271614104311264_i128),);
Call(_10.1.0 = fn2(Field::<bool>(Variant(RET.2, 3), 0), _7, RET.1, RET, Field::<bool>(Variant(RET.2, 3), 0), (*_4), RET.1, Field::<bool>(Variant(RET.2, 3), 0), RET, _7, RET, RET), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10.1.2.fld6.0 = !RET.0;
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_10.1.2.fld4 = [_10.1.2.fld6.0,_10.1.2.fld6.0,_10.1.2.fld6.0,RET.0];
_10.1.2.fld6.1 = !(*_4);
_10.1.2.fld6.0 = RET.0 * RET.0;
_10.1.2.fld1.2 = core::ptr::addr_of_mut!(_10.1.2.fld6.1);
_10.1.1.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(RET.2, 3), 0),fld1: _2 };
_10.1.2.fld3 = Field::<bool>(Variant(_10.1.1.2, 2), 0) as i8;
_10.1.2.fld1.0 = _7;
_11 = Adt62::Variant2 { fld0: 11_u8 };
_10.1.2.fld6 = RET;
match _10.1.2.fld6.0 {
0 => bb3,
1 => bb11,
99 => bb13,
_ => bb12
}
}
bb11 = {
_3 = RET.0 as f64;
RET.2 = Adt18::Variant3 { fld0: false };
RET.1 = _1 + _1;
RET.1 = _1 + _1;
RET.2 = Adt18::Variant2 { fld0: true,fld1: _2 };
_4 = &_1;
_4 = &(*_4);
_5 = [344774410406797303_usize,15421581477168466828_usize];
place!(Field::<u128>(Variant(RET.2, 2), 1)) = !_2;
place!(Field::<bool>(Variant(RET.2, 2), 0)) = (*_4) < RET.1;
_2 = Field::<u128>(Variant(RET.2, 2), 1);
_4 = &_1;
RET.0 = 494516970606722340_i64 as i8;
RET.2 = Adt18::Variant0 { fld0: 76_u8,fld1: 18166525400272524878459556865227010524_i128,fld2: 44505_u16 };
RET.2 = Adt18::Variant0 { fld0: 160_u8,fld1: (-93643042826774890100539640853563436541_i128),fld2: 21535_u16 };
_3 = 4478562850027180188_usize as f64;
RET.2 = Adt18::Variant3 { fld0: true };
RET.0 = 99_i8;
_4 = &(*_4);
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_3 = 97_u8 as f64;
_7 = '\u{99b89}';
_10.2.3.0 = [4_usize,5585781460594852002_usize];
_10.1.2.fld5 = ((-68287210355678425858222271614104311264_i128),);
Call(_10.1.0 = fn2(Field::<bool>(Variant(RET.2, 3), 0), _7, RET.1, RET, Field::<bool>(Variant(RET.2, 3), 0), (*_4), RET.1, Field::<bool>(Variant(RET.2, 3), 0), RET, _7, RET, RET), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_10.0 = &_6;
_10.1.1.0 = !_10.1.2.fld6.0;
RET.1 = 1057804340_u32 as isize;
place!(Field::<u8>(Variant(_11, 2), 0)) = !235_u8;
_10.2.2 = [Field::<bool>(Variant(_10.1.2.fld6.2, 3), 0),Field::<bool>(Variant(_10.1.2.fld6.2, 3), 0),Field::<bool>(Variant(_10.1.2.fld6.2, 3), 0)];
_13 = [867990028_i32,(-2088442170_i32),(-1065803511_i32),(-1444399272_i32),1754806327_i32];
_10.1.2.fld0 = !2_usize;
_3 = (-9064132433421078841_i64) as f64;
_10.1.2.fld4 = [_10.1.2.fld6.0,_10.1.1.0,RET.0,RET.0];
_10.1.2.fld6.0 = -_10.1.1.0;
RET.1 = _10.1.2.fld6.1;
_14 = _10.1.2.fld6.1;
Goto(bb14)
}
bb14 = {
RET = (_10.1.2.fld6.0, _10.1.2.fld6.1, _10.1.1.2);
_12.0.1 = _10.1.2.fld0;
_10.1.2.fld0 = 2805890603_u32 as usize;
_10.1.2.fld0 = _12.0.1 ^ _12.0.1;
place!(Field::<u128>(Variant(_10.1.1.2, 2), 1)) = _2 - Field::<u128>(Variant(RET.2, 2), 1);
_10.2.1 = _10.1.1.0;
_10.1.2.fld5.0 = -(-1757008076828996130336707704606662472_i128);
SetDiscriminant(_11, 2);
_10.1.2.fld7 = core::ptr::addr_of_mut!(_14);
_6 = _10.1.2.fld6.1;
_10.2.0 = 50_u8 & 16_u8;
_10.1.1.2 = RET.2;
RET.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_10.1.2.fld6.2, 3), 0),fld1: _2 };
place!(Field::<bool>(Variant(_10.1.1.2, 2), 0)) = RET.1 > _10.1.2.fld6.1;
_12.0.1 = !_10.1.2.fld0;
place!(Field::<u8>(Variant(_11, 2), 0)) = _10.2.0 & _10.2.0;
RET.0 = _10.2.1;
_3 = _10.1.2.fld6.1 as f64;
_15 = (-574332960_i32) - 810067305_i32;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(1_usize, 5_usize, Move(_5), 7_usize, Move(_7), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: char,mut _3: isize,mut _4: (i8, isize, Adt18),mut _5: bool,mut _6: isize,mut _7: isize,mut _8: bool,mut _9: (i8, isize, Adt18),mut _10: char,mut _11: (i8, isize, Adt18),mut _12: (i8, isize, Adt18)) -> [u64; 1] {
mir! {
type RET = [u64; 1];
let _13: isize;
let _14: isize;
let _15: [i128; 4];
let _16: f32;
let _17: &'static isize;
let _18: isize;
let _19: isize;
let _20: isize;
let _21: bool;
let _22: (&'static &'static u8, usize);
let _23: bool;
let _24: isize;
let _25: f64;
let _26: (i32, [u32; 6], [char; 1], Adt18);
let _27: (u8, i8, [bool; 3], ([usize; 2],));
let _28: Adt62;
let _29: &'static [i32; 4];
let _30: f64;
let _31: [i32; 5];
let _32: &'static isize;
let _33: u64;
let _34: ([u32; 8], (i8, isize, Adt18));
let _35: (i64, u8, u64);
let _36: &'static i8;
let _37: &'static (&'static &'static u8, usize);
let _38: usize;
let _39: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _40: (&'static [i32; 4], &'static &'static u8, (u8, u64, [i32; 4], (char, u64, *mut isize)));
let _41: ();
let _42: ();
{
_10 = _2;
place!(Field::<bool>(Variant(_9.2, 3), 0)) = Field::<bool>(Variant(_4.2, 3), 0) & Field::<bool>(Variant(_12.2, 3), 0);
_2 = _10;
_13 = _4.1;
_9.1 = _7;
place!(Field::<bool>(Variant(_4.2, 3), 0)) = _13 > _11.1;
_9 = _12;
Call(_9.2 = fn3(_8, _2, _9.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [4788471600954955420_u64];
_11 = (_4.0, _12.1, _9.2);
_4.2 = _12.2;
SetDiscriminant(_11.2, 0);
_2 = _10;
_12.1 = _3;
_9.1 = -_13;
place!(Field::<bool>(Variant(_9.2, 2), 0)) = !_1;
_5 = _8 & _1;
place!(Field::<u16>(Variant(_11.2, 0), 2)) = 38790_u16;
place!(Field::<u16>(Variant(_11.2, 0), 2)) = 32574_u16 + 31093_u16;
place!(Field::<u8>(Variant(_11.2, 0), 0)) = !240_u8;
_11.1 = _4.1 - _13;
place!(Field::<u8>(Variant(_11.2, 0), 0)) = 27_u8;
place!(Field::<i128>(Variant(_11.2, 0), 1)) = 133797324776026147564931595256402310375_i128;
_11.0 = _12.0;
_6 = _7 & _4.1;
_16 = Field::<u128>(Variant(_9.2, 2), 1) as f32;
place!(Field::<i128>(Variant(_11.2, 0), 1)) = 75332617544336804666421360769355132567_i128 + 64951674278232442776797924857620830649_i128;
_17 = &_19;
_19 = _6;
place!(Field::<bool>(Variant(_4.2, 3), 0)) = _6 == _19;
Call(RET = fn10(_19, _11, _19, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11.2 = Adt18::Variant2 { fld0: _1,fld1: Field::<u128>(Variant(_9.2, 2), 1) };
_4 = (_11.0, _19, _9.2);
_11.0 = !_4.0;
_13 = !_4.1;
_4.0 = _12.0;
_9 = (_4.0, _19, _4.2);
_9.1 = !_11.1;
Call(_4 = fn16(_11.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = (_4.0, _11.1, _4.2);
_4.1 = Field::<bool>(Variant(_12.2, 3), 0) as isize;
place!(Field::<bool>(Variant(_9.2, 2), 0)) = _19 < _11.1;
Goto(bb4)
}
bb4 = {
place!(Field::<u128>(Variant(_11.2, 2), 1)) = _16 as u128;
_4.1 = _6;
_15 = [(-84418598149058027534454230071919965262_i128),(-37631340971291779788485029215942274239_i128),665121892877370174168813374669069240_i128,(-66588878744600794540549346401242909004_i128)];
_4.1 = 15871_u16 as isize;
match _9.0 {
99 => bb5,
_ => bb3
}
}
bb5 = {
place!(Field::<bool>(Variant(_4.2, 3), 0)) = _9.1 == _13;
_19 = -_12.1;
_11 = _12;
_11 = (_12.0, _19, _12.2);
_6 = !_11.1;
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
99 => bb8,
_ => bb7
}
}
bb6 = {
place!(Field::<u128>(Variant(_11.2, 2), 1)) = _16 as u128;
_4.1 = _6;
_15 = [(-84418598149058027534454230071919965262_i128),(-37631340971291779788485029215942274239_i128),665121892877370174168813374669069240_i128,(-66588878744600794540549346401242909004_i128)];
_4.1 = 15871_u16 as isize;
match _9.0 {
99 => bb5,
_ => bb3
}
}
bb7 = {
_11.2 = Adt18::Variant2 { fld0: _1,fld1: Field::<u128>(Variant(_9.2, 2), 1) };
_4 = (_11.0, _19, _9.2);
_11.0 = !_4.0;
_13 = !_4.1;
_4.0 = _12.0;
_9 = (_4.0, _19, _4.2);
_9.1 = !_11.1;
Call(_4 = fn16(_11.1), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_12.0 = _10 as i8;
_2 = _10;
_14 = _12.1 >> _11.1;
place!(Field::<bool>(Variant(_11.2, 3), 0)) = Field::<bool>(Variant(_4.2, 3), 0);
_9 = _4;
Goto(bb9)
}
bb9 = {
_20 = 51731_u16 as isize;
_4.0 = _2 as i8;
_12.0 = _9.0 | _11.0;
_15 = [(-80888578651134306855183417426839909494_i128),(-151595874259189823158521446106086619399_i128),44267377724773255607679770473507993290_i128,(-165058200312916167740658900305571976476_i128)];
_4.2 = _12.2;
_11 = (_12.0, _6, _9.2);
SetDiscriminant(_12.2, 0);
_27.1 = 7899168327072866072_u64 as i8;
_25 = 31051_u16 as f64;
_12.2 = Adt18::Variant0 { fld0: 136_u8,fld1: (-132495756791678149825294688885801229851_i128),fld2: 47328_u16 };
_12.2 = _4.2;
_26.1 = [1291308759_u32,2564021142_u32,2231527752_u32,3689269507_u32,341158265_u32,3888025462_u32];
_23 = _14 == _14;
_11.2 = Adt18::Variant0 { fld0: 236_u8,fld1: (-85143018037726559015704628388168966889_i128),fld2: 36940_u16 };
Call(_27.1 = core::intrinsics::transmute(_11.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6 = !_12.1;
SetDiscriminant(_9.2, 0);
place!(Field::<u8>(Variant(_9.2, 0), 0)) = 43_u8 + 78_u8;
_11 = (_12.0, _12.1, _12.2);
_11 = _4;
_21 = Field::<bool>(Variant(_11.2, 3), 0) | _23;
place!(Field::<u8>(Variant(_9.2, 0), 0)) = 15_u8 >> _12.0;
_9.1 = 9140_u16 as isize;
_4.1 = !_6;
_27.2 = [_21,_21,_23];
place!(Field::<bool>(Variant(_4.2, 3), 0)) = !_21;
place!(Field::<bool>(Variant(_11.2, 3), 0)) = !Field::<bool>(Variant(_4.2, 3), 0);
Goto(bb11)
}
bb11 = {
_4 = _12;
SetDiscriminant(_12.2, 1);
_12 = _4;
_4.2 = Adt18::Variant0 { fld0: Field::<u8>(Variant(_9.2, 0), 0),fld1: 97066058219648418130030132501080405152_i128,fld2: 13719_u16 };
_4 = (_12.0, _19, _11.2);
_18 = _12.1;
_27.1 = _12.0;
_4.0 = _27.1 * _12.0;
_25 = 17624824496046647721_u64 as f64;
_12.2 = _4.2;
_23 = Field::<bool>(Variant(_12.2, 3), 0) & Field::<bool>(Variant(_4.2, 3), 0);
_28 = Adt62::Variant2 { fld0: Field::<u8>(Variant(_9.2, 0), 0) };
_24 = !_19;
_13 = 3109378786246607936_u64 as isize;
_9.2 = Adt18::Variant2 { fld0: _21,fld1: 199049184381800134976415965391673586210_u128 };
_23 = _21;
_30 = _25 - _25;
_26.2 = [_10];
Goto(bb12)
}
bb12 = {
_21 = !Field::<bool>(Variant(_12.2, 3), 0);
place!(Field::<u128>(Variant(_9.2, 2), 1)) = 204307274330737702947311473115359684755_u128 | 208590627493916556506258642074293719930_u128;
_9.2 = _11.2;
place!(Field::<u8>(Variant(_28, 2), 0)) = 239_u8;
_32 = &_19;
_17 = Move(_32);
_4.0 = _12.0;
_1 = Field::<bool>(Variant(_11.2, 3), 0);
_32 = &_4.1;
_12.1 = _30 as isize;
_27.0 = !Field::<u8>(Variant(_28, 2), 0);
_22.1 = _30 as usize;
match Field::<u8>(Variant(_28, 2), 0) {
239 => bb13,
_ => bb9
}
}
bb13 = {
_5 = _23 < _21;
_12.2 = _9.2;
_20 = (-53958293528575314775684914536223153493_i128) as isize;
_31 = [2062248457_i32,(-1543717542_i32),(-1021786407_i32),(-329122538_i32),(-1104953720_i32)];
_34.0 = [1399574001_u32,2606565874_u32,3054790043_u32,1400883076_u32,1053003205_u32,3669227267_u32,1131492880_u32,702432677_u32];
_26.2 = [_10];
_34.1.1 = (*_32) - _4.1;
_32 = &_4.1;
SetDiscriminant(_9.2, 0);
_9 = (_12.0, (*_32), _4.2);
_9.1 = (*_32);
_18 = -_34.1.1;
_9 = (_12.0, _14, _12.2);
_12.1 = 5569662161643405833_i64 as isize;
_8 = _5;
_11.2 = _4.2;
_32 = &_9.1;
_9.1 = -_12.1;
place!(Field::<bool>(Variant(_12.2, 3), 0)) = _1 <= _8;
Call(_11.1 = fn18(_18, _27.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_26.3 = _4.2;
_30 = _22.1 as f64;
_11.2 = Adt18::Variant3 { fld0: _21 };
_14 = _11.1 << _12.0;
_26.3 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_11.2, 3), 0) };
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(2_usize, 19_usize, Move(_19), 24_usize, Move(_24), 31_usize, Move(_31), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(2_usize, 2_usize, Move(_2), 3_usize, Move(_3), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(2_usize, 1_usize, Move(_1), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: char,mut _3: isize) -> Adt18 {
mir! {
type RET = Adt18;
let _4: i32;
let _5: Adt20;
let _6: [char; 1];
let _7: ([u32; 8], (i8, isize, Adt18));
let _8: Adt18;
let _9: Adt47;
let _10: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _11: bool;
let _12: f64;
let _13: ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _14: &'static &'static [i32; 4];
let _15: (char, f64);
let _16: u64;
let _17: u32;
let _18: bool;
let _19: [u32; 8];
let _20: Adt77;
let _21: [u64; 1];
let _22: (Adt65, (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31), (i8, isize, Adt18));
let _23: bool;
let _24: i32;
let _25: i64;
let _26: isize;
let _27: char;
let _28: f32;
let _29: *mut [i16; 5];
let _30: ();
let _31: ();
{
_2 = '\u{e624c}';
RET = Adt18::Variant3 { fld0: _1 };
SetDiscriminant(RET, 3);
RET = Adt18::Variant3 { fld0: _1 };
place!(Field::<bool>(Variant(RET, 3), 0)) = !_1;
_2 = '\u{7b7ee}';
RET = Adt18::Variant0 { fld0: 25_u8,fld1: 73051112541892382656135366761775168209_i128,fld2: 21481_u16 };
place!(Field::<u8>(Variant(RET, 0), 0)) = 120_u8;
place!(Field::<u16>(Variant(RET, 0), 2)) = 6932_u16 - 7903_u16;
Call(place!(Field::<u8>(Variant(RET, 0), 0)) = fn4(Field::<u16>(Variant(RET, 0), 2), _3, _3, _3, _3, Field::<u16>(Variant(RET, 0), 2), _3, Field::<u16>(Variant(RET, 0), 2), _1, _3, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = Adt18::Variant2 { fld0: _1,fld1: 97828030814941312980790629331533124109_u128 };
RET = Adt18::Variant0 { fld0: 116_u8,fld1: (-116703166181281910562214351594856625722_i128),fld2: 31882_u16 };
place!(Field::<u16>(Variant(RET, 0), 2)) = 1542_u16;
place!(Field::<i128>(Variant(RET, 0), 1)) = 106232042448802485960791119296255373384_i128;
_7.0 = [1202269865_u32,1847475508_u32,2387841050_u32,368802725_u32,4068719814_u32,1356055491_u32,2748602724_u32,3031944936_u32];
match Field::<u16>(Variant(RET, 0), 2) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1542 => bb7,
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
Goto(bb8)
}
bb8 = {
_12 = (-20346_i16) as f64;
RET = Adt18::Variant0 { fld0: 152_u8,fld1: 14560180601818692752120371209093437327_i128,fld2: 63282_u16 };
place!(Field::<u16>(Variant(RET, 0), 2)) = 1270550392093863340_usize as u16;
_11 = _1;
_4 = _3 as i32;
place!(Field::<i128>(Variant(RET, 0), 1)) = !105674319376503205519259153956979812693_i128;
_7.1.2 = Adt18::Variant3 { fld0: _11 };
_7.1.1 = !_3;
_7.0 = [2503304883_u32,58087092_u32,1064503148_u32,1093180742_u32,1562051425_u32,3733166540_u32,2400736186_u32,2330469589_u32];
place!(Field::<u16>(Variant(RET, 0), 2)) = Field::<i128>(Variant(RET, 0), 1) as u16;
_8 = _7.1.2;
Goto(bb9)
}
bb9 = {
_2 = '\u{5ace7}';
_13.1.0 = !(-42_i8);
_10 = core::ptr::addr_of!(_13);
_13.2.fld7 = core::ptr::addr_of_mut!((*_10).2.fld6.1);
_13.2.fld6.0 = (*_10).1.0 - _13.1.0;
_13.3 = Adt20::Variant1 { fld0: _12,fld1: _2,fld2: 102301373407339123711722929847798611321_u128,fld3: (*_10).1.0,fld4: 6353583746104328057_u64,fld5: _4,fld6: 1487631190_u32 };
(*_10).2.fld2 = [_3];
(*_10).2.fld5 = (Field::<i128>(Variant(RET, 0), 1),);
SetDiscriminant(_7.1.2, 0);
place!(Field::<u64>(Variant((*_10).3, 1), 4)) = 6_usize as u64;
(*_10).0 = [Field::<u64>(Variant((*_10).3, 1), 4)];
(*_10).2.fld6.2 = Adt18::Variant3 { fld0: _1 };
place!(Field::<u128>(Variant(_13.3, 1), 2)) = _7.1.1 as u128;
(*_10).0 = [Field::<u64>(Variant(_13.3, 1), 4)];
(*_10).2.fld1.0 = _2;
_7.1.2 = (*_10).2.fld6.2;
place!(Field::<u16>(Variant(RET, 0), 2)) = !54373_u16;
Call(place!(Field::<u32>(Variant((*_10).3, 1), 6)) = fn5(Move(_10), Field::<bool>(Variant(_7.1.2, 3), 0), Field::<i128>(Variant(RET, 0), 1), (*_10).2.fld2, Field::<u64>(Variant((*_10).3, 1), 4), _7.0, _4, Field::<u128>(Variant((*_10).3, 1), 2), _7.1.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
SetDiscriminant(_7.1.2, 1);
_11 = _1;
place!(Field::<[isize; 1]>(Variant(_7.1.2, 1), 3)) = _13.2.fld2;
_4 = _2 as i32;
_13.2.fld5.0 = -Field::<i128>(Variant(RET, 0), 1);
Goto(bb11)
}
bb11 = {
place!(Field::<isize>(Variant(_7.1.2, 1), 2)) = _7.1.1;
place!(Field::<f32>(Variant(_7.1.2, 1), 5)) = Field::<u128>(Variant(_13.3, 1), 2) as f32;
_19 = [Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6),Field::<u32>(Variant(_13.3, 1), 6)];
place!(Field::<u8>(Variant(RET, 0), 0)) = Field::<u32>(Variant(_13.3, 1), 6) as u8;
_13.1.1 = Field::<bool>(Variant(_13.2.fld6.2, 3), 0) as isize;
_13.2.fld1.1 = Field::<u64>(Variant(_13.3, 1), 4);
Call(_17 = fn6(), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_20.fld3 = !25388_i16;
_16 = Field::<u64>(Variant(_13.3, 1), 4);
_12 = Field::<f32>(Variant(_7.1.2, 1), 5) as f64;
place!(Field::<f64>(Variant(_13.3, 1), 0)) = Field::<u8>(Variant(RET, 0), 0) as f64;
_22.1.3.fld6 = (_13.2.fld6.0, _7.1.1, RET);
place!(Field::<u8>(Variant(RET, 0), 0)) = _13.2.fld1.0 as u8;
_22.0.fld6 = [15380915985387669435_usize,3_usize,4999944593699631104_usize,5_usize,6_usize,13186217683452666074_usize,1_usize];
_22.1.1.1.1 = _3;
_15.1 = -_12;
_13.2.fld1.0 = _2;
_7 = (_19, _22.1.3.fld6);
_7.1.1 = Field::<u8>(Variant(_7.1.2, 0), 0) as isize;
Goto(bb13)
}
bb13 = {
_22.1.3.fld1.0 = _13.2.fld1.0;
_7.1.1 = -_3;
SetDiscriminant(_22.1.3.fld6.2, 0);
_22.1.3.fld1.1 = _16;
SetDiscriminant(RET, 1);
_22.1.1.2.fld6.2 = _13.2.fld6.2;
_13.2.fld4 = [_13.1.0,_13.2.fld6.0,_13.2.fld6.0,_22.1.3.fld6.0];
place!(Field::<u16>(Variant(_22.1.3.fld6.2, 0), 2)) = Field::<u16>(Variant(_7.1.2, 0), 2);
_22.1.1.2.fld5 = (Field::<i128>(Variant(_7.1.2, 0), 1),);
_13.1.0 = -_22.1.3.fld6.0;
_22.1.3.fld1.0 = _13.2.fld1.0;
_7.1.1 = !_3;
place!(Field::<f64>(Variant(_13.3, 1), 0)) = Field::<u32>(Variant(_13.3, 1), 6) as f64;
_6 = [_22.1.3.fld1.0];
_22.0.fld1 = Field::<bool>(Variant(_8, 3), 0) as u64;
_7.1 = (_22.1.3.fld6.0, _22.1.1.1.1, _8);
_22.0.fld2.0 = _13.2.fld5.0 >> _22.1.1.1.1;
_22.1.3.fld4 = [_22.1.3.fld6.0,_13.1.0,_22.1.3.fld6.0,_13.2.fld6.0];
_11 = !Field::<bool>(Variant(_7.1.2, 3), 0);
_22.1.1.2.fld1.0 = _22.1.3.fld1.0;
_22.1.1.1.0 = _22.1.3.fld6.0 >> Field::<i32>(Variant(_13.3, 1), 5);
place!(Field::<f32>(Variant(RET, 1), 5)) = 90_u8 as f32;
Call(_22.1.1.1 = fn7(_22.0.fld2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22.0.fld6 = [4_usize,4_usize,4_usize,10267100875848967786_usize,7136272969854871536_usize,5_usize,6641159685469147702_usize];
_22.1.1.2.fld1.2 = core::ptr::addr_of_mut!(_22.1.1.1.1);
RET = _22.1.1.1.2;
_22.1.1.2.fld6.1 = !_7.1.1;
_13.3 = Adt20::Variant1 { fld0: _12,fld1: _13.2.fld1.0,fld2: Field::<u128>(Variant(RET, 2), 1),fld3: _13.1.0,fld4: _13.2.fld1.1,fld5: _4,fld6: _17 };
_22.0.fld5 = _15.1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(3_usize, 4_usize, Move(_4), 19_usize, Move(_19), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u16,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u16,mut _7: isize,mut _8: u16,mut _9: bool,mut _10: isize,mut _11: char,mut _12: char) -> u8 {
mir! {
type RET = u8;
let _13: (&'static [i32; 4], &'static &'static u8, (u8, u64, [i32; 4], (char, u64, *mut isize)));
let _14: (i64, u8, u64);
let _15: isize;
let _16: &'static [u16; 1];
let _17: u128;
let _18: f32;
let _19: ();
let _20: ();
{
_4 = _3;
RET = 68_u8 * 48_u8;
_7 = _5;
_4 = !_3;
_3 = _5;
_7 = _3 ^ _5;
_9 = _10 <= _2;
_8 = (-693864155594136706_i64) as u16;
_5 = !_2;
_5 = _10;
_12 = _11;
_10 = _4 ^ _4;
_1 = !_6;
_6 = _1;
_13.2.3.2 = core::ptr::addr_of_mut!(_7);
_13.2.2 = [227819107_i32,69171557_i32,1178096640_i32,(-1278877904_i32)];
_13.2.3.2 = core::ptr::addr_of_mut!(_4);
_13.2.1 = 922168561118827421_u64 << _5;
_6 = 9810935854594757206_usize as u16;
Goto(bb1)
}
bb1 = {
_2 = _7 * _5;
_15 = _9 as isize;
_9 = !false;
_14.2 = _13.2.1 * _13.2.1;
_13.2.0 = _14.2 as u8;
_13.2.3.1 = _14.2 * _14.2;
_13.2.2 = [2127957776_i32,1993948873_i32,(-16468011_i32),(-112656490_i32)];
_7 = 5513416587885902771_i64 as isize;
_13.2.3.1 = _14.2;
Goto(bb2)
}
bb2 = {
_14 = ((-1924725539991490927_i64), _13.2.0, _13.2.3.1);
_13.2.2 = [203956288_i32,183483082_i32,(-1680211842_i32),(-1490649745_i32)];
_1 = _8;
RET = _13.2.3.1 as u8;
_12 = _11;
_13.0 = &_13.2.2;
_6 = _8 + _1;
_13.2.3.0 = _11;
_1 = _6;
_5 = _14.0 as isize;
_13.0 = &_13.2.2;
RET = !_13.2.0;
_13.2.3.0 = _11;
_5 = _3;
_11 = _13.2.3.0;
_14.0 = -(-3711611394185512462_i64);
_8 = !_1;
_13.2.3.1 = !_14.2;
_1 = 1304252762_i32 as u16;
_3 = _2;
RET = _13.2.0;
_14.2 = _13.2.3.1 + _13.2.1;
_14.2 = _13.2.1 + _13.2.1;
_11 = _12;
RET = _13.2.0 + _13.2.0;
_13.0 = &_13.2.2;
_17 = !328038495345205661894291923199666084391_u128;
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(4_usize, 2_usize, Move(_2), 17_usize, Move(_17), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_19 = dump_var(4_usize, 15_usize, Move(_15), 3_usize, Move(_3), 9_usize, Move(_9), 20_usize, _20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20),mut _2: bool,mut _3: i128,mut _4: [isize; 1],mut _5: u64,mut _6: [u32; 8],mut _7: i32,mut _8: u128,mut _9: isize) -> u32 {
mir! {
type RET = u32;
let _10: &'static &'static u8;
let _11: u8;
let _12: *mut [i16; 5];
let _13: &'static i8;
let _14: [i32; 4];
let _15: (i32, [u32; 6], [char; 1], Adt18);
let _16: [u64; 5];
let _17: f32;
let _18: u8;
let _19: u32;
let _20: *mut isize;
let _21: ();
let _22: ();
{
_4 = [_9];
_4 = [_9];
_5 = 6181269930785446902_u64;
RET = 2436218743_u32 * 1793637115_u32;
_7 = 822122999_i32 + 1176365170_i32;
_2 = _7 >= _7;
_4 = [_9];
_9 = (-9223372036854775808_isize);
RET = !1234729923_u32;
_11 = 153_u8;
RET = _5 as u32;
RET = !619052804_u32;
_5 = 18091803455674370551_u64 >> _3;
_5 = _8 as u64;
_3 = !(-47177343688905700642141087082091706478_i128);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = 167036216054529521353525319666274834050_u128;
_9 = RET as isize;
_4 = [_9];
_7 = 341732239_i32 * 198118856_i32;
_4 = [_9];
_2 = !false;
_3 = 6643442528025760762_i64 as i128;
_3 = 86325060224703271252304511043365483555_i128;
_14 = [_7,_7,_7,_7];
Call(RET = core::intrinsics::bswap(2550428152_u32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = [_9];
_2 = true;
_7 = 691848972_i32;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
86325060224703271252304511043365483555 => bb10,
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
_8 = !210550074840953964013249872837820933076_u128;
_8 = !202175977261521867555027506099926445848_u128;
_9 = (-9223372036854775808_isize);
_5 = 7499579729963802706_u64 * 2257338516300334311_u64;
match _9 {
340282366920938463454151235394913435648 => bb11,
_ => bb7
}
}
bb11 = {
_9 = _3 as isize;
_9 = (-9223372036854775808_isize) >> _11;
_2 = !true;
match _7 {
691848972 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = 276812461820658181958676002693220652062_u128;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [_9];
_15.3 = Adt18::Variant2 { fld0: _2,fld1: _8 };
_15.1 = [RET,RET,RET,RET,RET,RET];
_15.3 = Adt18::Variant3 { fld0: _2 };
_9 = 16642877874584682951_usize as isize;
_16 = [_5,_5,_5,_5,_5];
_15.1 = [RET,RET,RET,RET,RET,RET];
_15.2 = ['\u{fe2eb}'];
_17 = _11 as f32;
_4 = [_9];
RET = 622488749_u32 + 3224752820_u32;
RET = !98311303_u32;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = 90_u8;
place!(Field::<bool>(Variant(_15.3, 3), 0)) = _2;
place!(Field::<bool>(Variant(_15.3, 3), 0)) = _11 >= _11;
_19 = 73_i8 as u32;
Goto(bb14)
}
bb14 = {
_11 = 34_u8;
_7 = 2019099743_i32 << _8;
_3 = 3329043748803094806465651229942158665_i128;
SetDiscriminant(_15.3, 3);
_16 = [_5,_5,_5,_5,_5];
_15.2 = ['\u{637cd}'];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(5_usize, 2_usize, Move(_2), 14_usize, Move(_14), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(5_usize, 16_usize, Move(_16), 4_usize, Move(_4), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6() -> u32 {
mir! {
type RET = u32;
let _1: isize;
let _2: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31);
let _3: &'static isize;
let _4: [u16; 1];
let _5: ([isize; 1], (char, f64), f32);
let _6: char;
let _7: i64;
let _8: f64;
let _9: i8;
let _10: [i8; 1];
let _11: u8;
let _12: (([isize; 1], (char, f64), f32), ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), [u16; 6]);
let _13: char;
let _14: &'static &'static u8;
let _15: u16;
let _16: bool;
let _17: u128;
let _18: f32;
let _19: bool;
let _20: f64;
let _21: f32;
let _22: (i64, u8, u64);
let _23: u128;
let _24: [usize; 1];
let _25: ();
let _26: ();
{
RET = !2777941989_u32;
RET = 25131_i16 as u32;
_1 = (-9223372036854775808_isize);
_2.3.fld4 = [(-113_i8),(-63_i8),(-3_i8),26_i8];
_2.3.fld7 = core::ptr::addr_of_mut!(_2.3.fld6.1);
_2.1.2.fld5.0 = false as i128;
_2.3.fld1.2 = core::ptr::addr_of_mut!(_2.1.1.1);
match _1 {
340282366920938463454151235394913435648 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2.1.1.0 = 104_i8;
_2.3.fld5 = (_2.1.2.fld5.0,);
_2.1.0 = [142264811994587717_u64];
_2.3.fld6.0 = true as i8;
_2.1.2.fld1.1 = 8065411773018508061_u64 << _2.1.2.fld5.0;
_2.1.2.fld7 = core::ptr::addr_of_mut!(_1);
_2.1.2.fld4 = [_2.3.fld6.0,_2.1.1.0,_2.1.1.0,_2.1.1.0];
_1 = (-9223372036854775808_isize) << RET;
_3 = &_2.1.1.1;
_5.0 = [_1];
_2.1.2.fld1.0 = '\u{28927}';
_2.3.fld3 = 4483_i16 as i8;
_3 = &_2.1.2.fld6.1;
_2.1.2.fld6.2 = Adt18::Variant0 { fld0: 117_u8,fld1: _2.3.fld5.0,fld2: 30935_u16 };
_2.0 = 293_i16 * (-21302_i16);
_2.1.1.1 = _1 - _1;
_5.1.0 = _2.1.2.fld1.0;
_2.1.2.fld6.1 = _2.1.1.1 >> _2.1.1.1;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = !_2.1.2.fld5.0;
place!(Field::<u16>(Variant(_2.1.2.fld6.2, 0), 2)) = 48634_u16 + 46052_u16;
_3 = &_2.1.2.fld6.1;
Call(_2.0 = core::intrinsics::bswap((-6872_i16)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2.1.1.1 = (*_3);
_2.1.2.fld1 = (_5.1.0, 17232535106167267497_u64, Move(_2.1.2.fld7));
_1 = false as isize;
_2.3.fld1.0 = _5.1.0;
_2.1.2.fld7 = Move(_2.3.fld1.2);
_8 = RET as f64;
_12.1.1.1 = _2.1.1.1 >> _2.0;
_13 = _5.1.0;
_12.1.2.fld1.2 = Move(_2.1.2.fld1.2);
_2.1.2.fld3 = _2.3.fld3;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = 109_u8 as i128;
Goto(bb4)
}
bb4 = {
_2.3.fld4 = [_2.1.1.0,_2.1.2.fld3,_2.1.2.fld3,_2.1.2.fld3];
_2.1.2.fld6.0 = _2.3.fld3;
match _2.1.2.fld1.1 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
17232535106167267497 => bb8,
_ => bb7
}
}
bb5 = {
_2.1.1.1 = (*_3);
_2.1.2.fld1 = (_5.1.0, 17232535106167267497_u64, Move(_2.1.2.fld7));
_1 = false as isize;
_2.3.fld1.0 = _5.1.0;
_2.1.2.fld7 = Move(_2.3.fld1.2);
_8 = RET as f64;
_12.1.1.1 = _2.1.1.1 >> _2.0;
_13 = _5.1.0;
_12.1.2.fld1.2 = Move(_2.1.2.fld1.2);
_2.1.2.fld3 = _2.3.fld3;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = 109_u8 as i128;
Goto(bb4)
}
bb6 = {
_2.1.1.0 = 104_i8;
_2.3.fld5 = (_2.1.2.fld5.0,);
_2.1.0 = [142264811994587717_u64];
_2.3.fld6.0 = true as i8;
_2.1.2.fld1.1 = 8065411773018508061_u64 << _2.1.2.fld5.0;
_2.1.2.fld7 = core::ptr::addr_of_mut!(_1);
_2.1.2.fld4 = [_2.3.fld6.0,_2.1.1.0,_2.1.1.0,_2.1.1.0];
_1 = (-9223372036854775808_isize) << RET;
_3 = &_2.1.1.1;
_5.0 = [_1];
_2.1.2.fld1.0 = '\u{28927}';
_2.3.fld3 = 4483_i16 as i8;
_3 = &_2.1.2.fld6.1;
_2.1.2.fld6.2 = Adt18::Variant0 { fld0: 117_u8,fld1: _2.3.fld5.0,fld2: 30935_u16 };
_2.0 = 293_i16 * (-21302_i16);
_2.1.1.1 = _1 - _1;
_5.1.0 = _2.1.2.fld1.0;
_2.1.2.fld6.1 = _2.1.1.1 >> _2.1.1.1;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = !_2.1.2.fld5.0;
place!(Field::<u16>(Variant(_2.1.2.fld6.2, 0), 2)) = 48634_u16 + 46052_u16;
_3 = &_2.1.2.fld6.1;
Call(_2.0 = core::intrinsics::bswap((-6872_i16)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
Return()
}
bb8 = {
_2.3.fld2 = [(*_3)];
_12.1.2.fld7 = Move(_2.1.2.fld7);
_2.1.2.fld1.2 = core::ptr::addr_of_mut!((*_3));
_2.1.0 = [_2.1.2.fld1.1];
_12.1.2.fld1 = (_2.3.fld1.0, _2.1.2.fld1.1, Move(_12.1.2.fld7));
_12.1.1.1 = (-3727101658391898606_i64) as isize;
_2.3.fld6.2 = Adt18::Variant0 { fld0: 232_u8,fld1: _2.3.fld5.0,fld2: Field::<u16>(Variant(_2.1.2.fld6.2, 0), 2) };
_12.1.2.fld3 = _2.3.fld6.0 ^ _2.1.2.fld3;
_12.0.2 = _2.3.fld3 as f32;
_2.3.fld1.1 = _2.1.2.fld1.1 | _12.1.2.fld1.1;
_2.1.1.2 = Adt18::Variant3 { fld0: false };
_7 = false as i64;
_2.1.2.fld5.0 = !Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1);
_2.1.2.fld6.0 = _12.1.2.fld3;
_12.1.2.fld6.1 = _2.1.1.1 - _1;
_2.1.3 = Adt20::Variant1 { fld0: _8,fld1: _13,fld2: 8827619001100151411779546284807869182_u128,fld3: _2.1.2.fld3,fld4: _2.1.2.fld1.1,fld5: 704652570_i32,fld6: RET };
_2.1.2.fld7 = Move(_2.1.2.fld1.2);
place!(Field::<bool>(Variant(_2.1.1.2, 3), 0)) = Field::<u64>(Variant(_2.1.3, 1), 4) > _2.3.fld1.1;
place!(Field::<i128>(Variant(_2.3.fld6.2, 0), 1)) = Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1) & _2.1.2.fld5.0;
_12.1.2.fld7 = Move(_12.1.2.fld1.2);
_2.1.2.fld6.0 = _12.1.2.fld3;
_12.1.2.fld7 = core::ptr::addr_of_mut!((*_3));
match Field::<u64>(Variant(_2.1.3, 1), 4) {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
17232535106167267497 => bb14,
_ => bb13
}
}
bb9 = {
Return()
}
bb10 = {
_2.1.1.0 = 104_i8;
_2.3.fld5 = (_2.1.2.fld5.0,);
_2.1.0 = [142264811994587717_u64];
_2.3.fld6.0 = true as i8;
_2.1.2.fld1.1 = 8065411773018508061_u64 << _2.1.2.fld5.0;
_2.1.2.fld7 = core::ptr::addr_of_mut!(_1);
_2.1.2.fld4 = [_2.3.fld6.0,_2.1.1.0,_2.1.1.0,_2.1.1.0];
_1 = (-9223372036854775808_isize) << RET;
_3 = &_2.1.1.1;
_5.0 = [_1];
_2.1.2.fld1.0 = '\u{28927}';
_2.3.fld3 = 4483_i16 as i8;
_3 = &_2.1.2.fld6.1;
_2.1.2.fld6.2 = Adt18::Variant0 { fld0: 117_u8,fld1: _2.3.fld5.0,fld2: 30935_u16 };
_2.0 = 293_i16 * (-21302_i16);
_2.1.1.1 = _1 - _1;
_5.1.0 = _2.1.2.fld1.0;
_2.1.2.fld6.1 = _2.1.1.1 >> _2.1.1.1;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = !_2.1.2.fld5.0;
place!(Field::<u16>(Variant(_2.1.2.fld6.2, 0), 2)) = 48634_u16 + 46052_u16;
_3 = &_2.1.2.fld6.1;
Call(_2.0 = core::intrinsics::bswap((-6872_i16)), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_2.1.1.1 = (*_3);
_2.1.2.fld1 = (_5.1.0, 17232535106167267497_u64, Move(_2.1.2.fld7));
_1 = false as isize;
_2.3.fld1.0 = _5.1.0;
_2.1.2.fld7 = Move(_2.3.fld1.2);
_8 = RET as f64;
_12.1.1.1 = _2.1.1.1 >> _2.0;
_13 = _5.1.0;
_12.1.2.fld1.2 = Move(_2.1.2.fld1.2);
_2.1.2.fld3 = _2.3.fld3;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = 109_u8 as i128;
Goto(bb4)
}
bb12 = {
_2.3.fld4 = [_2.1.1.0,_2.1.2.fld3,_2.1.2.fld3,_2.1.2.fld3];
_2.1.2.fld6.0 = _2.3.fld3;
match _2.1.2.fld1.1 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
17232535106167267497 => bb8,
_ => bb7
}
}
bb13 = {
_2.1.1.1 = (*_3);
_2.1.2.fld1 = (_5.1.0, 17232535106167267497_u64, Move(_2.1.2.fld7));
_1 = false as isize;
_2.3.fld1.0 = _5.1.0;
_2.1.2.fld7 = Move(_2.3.fld1.2);
_8 = RET as f64;
_12.1.1.1 = _2.1.1.1 >> _2.0;
_13 = _5.1.0;
_12.1.2.fld1.2 = Move(_2.1.2.fld1.2);
_2.1.2.fld3 = _2.3.fld3;
place!(Field::<i128>(Variant(_2.1.2.fld6.2, 0), 1)) = 109_u8 as i128;
Goto(bb4)
}
bb14 = {
_12.1.2.fld7 = core::ptr::addr_of_mut!(_2.1.2.fld6.1);
_19 = Field::<bool>(Variant(_2.1.1.2, 3), 0);
_5.1 = (_12.1.2.fld1.0, _8);
_16 = !Field::<bool>(Variant(_2.1.1.2, 3), 0);
_12.1.2.fld2 = [_2.1.1.1];
_2.0 = Field::<u16>(Variant(_2.1.2.fld6.2, 0), 2) as i16;
_2.3.fld4 = [_2.1.2.fld6.0,_2.1.2.fld6.0,_2.1.2.fld6.0,_12.1.2.fld3];
_6 = _12.1.2.fld1.0;
_12.1.1 = (_12.1.2.fld3, (*_3), _2.1.1.2);
_2.1.2.fld0 = _12.0.2 as usize;
SetDiscriminant(_2.1.1.2, 2);
_11 = 255_u8 * 57_u8;
_2.1.2.fld4 = [_12.1.2.fld3,_2.3.fld6.0,_12.1.1.0,_12.1.1.0];
_12.0.1 = _5.1;
_2.1.2.fld2 = [_2.1.1.1];
_5 = (_2.3.fld2, _12.0.1, _12.0.2);
_2.1.2.fld1.2 = core::ptr::addr_of_mut!(_12.1.1.1);
_22.0 = _7;
place!(Field::<i32>(Variant(_2.1.3, 1), 5)) = !(-1727118250_i32);
_18 = -_12.0.2;
_2.1.2.fld1.1 = _12.1.2.fld1.1 * _2.3.fld1.1;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(6_usize, 16_usize, Move(_16), 6_usize, Move(_6), 19_usize, Move(_19), 26_usize, _26), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: (i128,)) -> (i8, isize, Adt18) {
mir! {
type RET = (i8, isize, Adt18);
let _2: i8;
let _3: bool;
let _4: f32;
let _5: u32;
let _6: isize;
let _7: [i16; 5];
let _8: *mut [i16; 5];
let _9: isize;
let _10: &'static &'static u8;
let _11: &'static i8;
let _12: u128;
let _13: ((&'static &'static u8, usize), &'static &'static u8, *const u16, (i128,));
let _14: char;
let _15: Adt62;
let _16: f32;
let _17: isize;
let _18: i128;
let _19: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _20: f32;
let _21: ();
let _22: ();
{
RET.0 = 97_i8;
RET.1 = 26_isize;
RET.2 = Adt18::Variant0 { fld0: 75_u8,fld1: _1.0,fld2: 56924_u16 };
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 2_usize as u8;
RET.1 = -38_isize;
_2 = RET.0;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 71_u8;
_3 = _1.0 >= Field::<i128>(Variant(RET.2, 0), 1);
RET.0 = !_2;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = !97_u8;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = '\u{8d660}' as u16;
RET.0 = _2;
RET.1 = (-9223372036854775808_isize) << Field::<i128>(Variant(RET.2, 0), 1);
SetDiscriminant(RET.2, 0);
place!(Field::<i128>(Variant(RET.2, 0), 1)) = (-8969399180061883348_i64) as i128;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = !249_u8;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = _1.0 as u16;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = _1.0 + _1.0;
RET.1 = 9223372036854775807_isize | 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
place!(Field::<i128>(Variant(RET.2, 0), 1)) = _1.0 | _1.0;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = !154_u8;
RET.1 = (-9223372036854775808_isize);
SetDiscriminant(RET.2, 0);
RET.2 = Adt18::Variant3 { fld0: _3 };
_4 = RET.1 as f32;
RET.0 = !_2;
Goto(bb2)
}
bb2 = {
SetDiscriminant(RET.2, 0);
place!(Field::<u16>(Variant(RET.2, 0), 2)) = !42547_u16;
_4 = RET.0 as f32;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 33402_u16;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = Field::<u16>(Variant(RET.2, 0), 2) as u8;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = RET.0 as i128;
RET.2 = Adt18::Variant3 { fld0: _3 };
_5 = 237_u8 as u32;
RET.0 = 60815_u16 as i8;
_6 = RET.1;
_8 = core::ptr::addr_of_mut!(_7);
_11 = &RET.0;
SetDiscriminant(RET.2, 1);
place!(Field::<i64>(Variant(RET.2, 1), 4)) = _1.0 as i64;
_11 = &_2;
place!(Field::<f32>(Variant(RET.2, 1), 5)) = _4 * _4;
_2 = RET.0 - RET.0;
match RET.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb3 = {
place!(Field::<i128>(Variant(RET.2, 0), 1)) = _1.0 | _1.0;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = !154_u8;
RET.1 = (-9223372036854775808_isize);
SetDiscriminant(RET.2, 0);
RET.2 = Adt18::Variant3 { fld0: _3 };
_4 = RET.1 as f32;
RET.0 = !_2;
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
place!(Field::<f64>(Variant(RET.2, 1), 1)) = 96_u8 as f64;
_7 = [29317_i16,(-20332_i16),(-28711_i16),(-19364_i16),(-26619_i16)];
place!(Field::<i64>(Variant(RET.2, 1), 4)) = 6708021014930319366_i64 - 734915086232410353_i64;
place!(Field::<isize>(Variant(RET.2, 1), 2)) = !RET.1;
place!(Field::<[isize; 1]>(Variant(RET.2, 1), 3)) = [_6];
RET.1 = _6 * Field::<isize>(Variant(RET.2, 1), 2);
_6 = '\u{8783e}' as isize;
Goto(bb11)
}
bb11 = {
(*_8) = [(-19063_i16),17812_i16,(-17180_i16),27936_i16,(-14299_i16)];
RET.0 = _3 as i8;
_2 = (-1758625147_i32) as i8;
_14 = '\u{2a239}';
_8 = core::ptr::addr_of_mut!(_7);
place!(Field::<i64>(Variant(RET.2, 1), 4)) = !(-7703656502362620726_i64);
place!(Field::<f64>(Variant(RET.2, 1), 1)) = 301800143315954308399913146511805799650_u128 as f64;
place!(Field::<f64>(Variant(RET.2, 1), 1)) = 39815_u16 as f64;
place!(Field::<isize>(Variant(RET.2, 1), 2)) = RET.0 as isize;
place!(Field::<f32>(Variant(RET.2, 1), 5)) = _4;
_12 = 84217555893654991949301634101499577526_u128 * 11630191421072391166264655112372844488_u128;
_11 = &RET.0;
RET.2 = Adt18::Variant0 { fld0: 58_u8,fld1: _1.0,fld2: 2898_u16 };
_6 = -RET.1;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 3848_u16 - 27296_u16;
RET.1 = _6;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = 99_u8 as i128;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 45_u8;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 13099171641733928566_u64 as u16;
_1.0 = _6 as i128;
_13.3.0 = _1.0;
_7 = [(-23996_i16),2047_i16,(-19572_i16),(-14515_i16),(-23049_i16)];
SetDiscriminant(RET.2, 0);
_8 = core::ptr::addr_of_mut!((*_8));
RET.2 = Adt18::Variant0 { fld0: 32_u8,fld1: _13.3.0,fld2: 48106_u16 };
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 34412_u16 << _12;
_1.0 = _4 as i128;
_11 = &RET.0;
Goto(bb12)
}
bb12 = {
_1.0 = Field::<i128>(Variant(RET.2, 0), 1);
_8 = core::ptr::addr_of_mut!(_7);
_13.2 = core::ptr::addr_of!(place!(Field::<u16>(Variant(RET.2, 0), 2)));
place!(Field::<u8>(Variant(RET.2, 0), 0)) = _4 as u8;
_1 = (Field::<i128>(Variant(RET.2, 0), 1),);
_9 = RET.1;
_13.3 = (Field::<i128>(Variant(RET.2, 0), 1),);
_13.0.1 = (*_11) as usize;
_18 = _13.3.0;
_15 = Adt62::Variant1 { fld0: 2452340424889958910_u64 };
RET.2 = Adt18::Variant3 { fld0: _3 };
Call(_4 = fn8(RET.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_2 = (-23034_i16) as i8;
RET.2 = Adt18::Variant0 { fld0: 198_u8,fld1: _18,fld2: 36268_u16 };
place!(Field::<u16>(Variant(RET.2, 0), 2)) = !14295_u16;
place!(Field::<u64>(Variant(_15, 1), 0)) = 8266328566251788449_u64 * 3502384019829686230_u64;
_16 = _4;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = (-5011980460053444636_i64) as i128;
RET.1 = 187_u8 as isize;
SetDiscriminant(_15, 0);
Call(RET.1 = core::intrinsics::bswap(_6), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET.2 = Adt18::Variant2 { fld0: _3,fld1: _12 };
place!(Field::<(u8, i8, [bool; 3], ([usize; 2],))>(Variant(_15, 0), 2)).2 = [_3,Field::<bool>(Variant(RET.2, 2), 0),Field::<bool>(Variant(RET.2, 2), 0)];
_7 = [(-32471_i16),14228_i16,10929_i16,(-19690_i16),18930_i16];
place!(Field::<(i8, isize, Adt18)>(Variant(_15, 0), 4)).2 = RET.2;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(7_usize, 1_usize, Move(_1), 7_usize, Move(_7), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(7_usize, 14_usize, Move(_14), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: Adt18) -> f32 {
mir! {
type RET = f32;
let _2: bool;
let _3: (i128,);
let _4: isize;
let _5: [i8; 4];
let _6: [i64; 6];
let _7: &'static [bool; 3];
let _8: [u32; 8];
let _9: &'static u8;
let _10: &'static Adt20;
let _11: Adt47;
let _12: ();
let _13: ();
{
RET = 7137646305991747277_i64 as f32;
RET = 17492_u16 as f32;
place!(Field::<bool>(Variant(_1, 3), 0)) = RET <= RET;
Goto(bb1)
}
bb1 = {
RET = 35_i8 as f32;
_1 = Adt18::Variant3 { fld0: true };
RET = 1676428362480818430_i64 as f32;
place!(Field::<bool>(Variant(_1, 3), 0)) = false;
Goto(bb2)
}
bb2 = {
place!(Field::<bool>(Variant(_1, 3), 0)) = true;
_1 = Adt18::Variant2 { fld0: true,fld1: 268724022952130425337465018357738165110_u128 };
place!(Field::<u128>(Variant(_1, 2), 1)) = !142577703679204949055164362364762669119_u128;
RET = Field::<u128>(Variant(_1, 2), 1) as f32;
_3 = (103187387777056597428423715862971629268_i128,);
place!(Field::<bool>(Variant(_1, 2), 0)) = !true;
_1 = Adt18::Variant3 { fld0: false };
place!(Field::<bool>(Variant(_1, 3), 0)) = RET > RET;
_2 = Field::<bool>(Variant(_1, 3), 0) | Field::<bool>(Variant(_1, 3), 0);
RET = (-24_i8) as f32;
_4 = (-1409306769_i32) as isize;
place!(Field::<bool>(Variant(_1, 3), 0)) = _2;
_5 = [(-126_i8),107_i8,(-119_i8),100_i8];
Call(RET = fn9(_5, _2, _1, _5, _2, _1, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = [(-1329003089025120705_i64),3896572503050055688_i64,3945436774944349428_i64,1524535714252457508_i64,(-5773135492283762010_i64),(-7827485892278026636_i64)];
_3 = (19685824419287063797014558404994316813_i128,);
_2 = !Field::<bool>(Variant(_1, 3), 0);
_4 = -(-9223372036854775808_isize);
_3.0 = (-48223228206429150082564476402013785235_i128);
_2 = Field::<bool>(Variant(_1, 3), 0);
_1 = Adt18::Variant2 { fld0: _2,fld1: 200511652289761589638877654449882203732_u128 };
place!(Field::<u128>(Variant(_1, 2), 1)) = 80370730027130909138012896639255322093_u128 ^ 128233451810652872124167690651345858774_u128;
RET = Field::<u128>(Variant(_1, 2), 1) as f32;
_1 = Adt18::Variant3 { fld0: _2 };
_4 = 13_isize;
_8 = [561026216_u32,3515249945_u32,3477714928_u32,2341440768_u32,502277300_u32,2505861181_u32,1614643752_u32,2020246373_u32];
RET = 6635_u16 as f32;
_5 = [(-60_i8),(-85_i8),3_i8,112_i8];
_1 = Adt18::Variant0 { fld0: 183_u8,fld1: _3.0,fld2: 59401_u16 };
RET = 46_u8 as f32;
match Field::<i128>(Variant(_1, 0), 1) {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
292059138714509313380810131029754426221 => bb12,
_ => bb11
}
}
bb4 = {
place!(Field::<bool>(Variant(_1, 3), 0)) = true;
_1 = Adt18::Variant2 { fld0: true,fld1: 268724022952130425337465018357738165110_u128 };
place!(Field::<u128>(Variant(_1, 2), 1)) = !142577703679204949055164362364762669119_u128;
RET = Field::<u128>(Variant(_1, 2), 1) as f32;
_3 = (103187387777056597428423715862971629268_i128,);
place!(Field::<bool>(Variant(_1, 2), 0)) = !true;
_1 = Adt18::Variant3 { fld0: false };
place!(Field::<bool>(Variant(_1, 3), 0)) = RET > RET;
_2 = Field::<bool>(Variant(_1, 3), 0) | Field::<bool>(Variant(_1, 3), 0);
RET = (-24_i8) as f32;
_4 = (-1409306769_i32) as isize;
place!(Field::<bool>(Variant(_1, 3), 0)) = _2;
_5 = [(-126_i8),107_i8,(-119_i8),100_i8];
Call(RET = fn9(_5, _2, _1, _5, _2, _1, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = 35_i8 as f32;
_1 = Adt18::Variant3 { fld0: true };
RET = 1676428362480818430_i64 as f32;
place!(Field::<bool>(Variant(_1, 3), 0)) = false;
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
Return()
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
place!(Field::<u16>(Variant(_1, 0), 2)) = 58018_u16;
_3 = (Field::<i128>(Variant(_1, 0), 1),);
_4 = -(-9223372036854775808_isize);
place!(Field::<u16>(Variant(_1, 0), 2)) = 16694_u16;
_9 = &place!(Field::<u8>(Variant(_1, 0), 0));
match Field::<u16>(Variant(_1, 0), 2) {
0 => bb4,
1 => bb8,
2 => bb13,
3 => bb14,
4 => bb15,
16694 => bb17,
_ => bb16
}
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<bool>(Variant(_1, 3), 0)) = true;
_1 = Adt18::Variant2 { fld0: true,fld1: 268724022952130425337465018357738165110_u128 };
place!(Field::<u128>(Variant(_1, 2), 1)) = !142577703679204949055164362364762669119_u128;
RET = Field::<u128>(Variant(_1, 2), 1) as f32;
_3 = (103187387777056597428423715862971629268_i128,);
place!(Field::<bool>(Variant(_1, 2), 0)) = !true;
_1 = Adt18::Variant3 { fld0: false };
place!(Field::<bool>(Variant(_1, 3), 0)) = RET > RET;
_2 = Field::<bool>(Variant(_1, 3), 0) | Field::<bool>(Variant(_1, 3), 0);
RET = (-24_i8) as f32;
_4 = (-1409306769_i32) as isize;
place!(Field::<bool>(Variant(_1, 3), 0)) = _2;
_5 = [(-126_i8),107_i8,(-119_i8),100_i8];
Call(RET = fn9(_5, _2, _1, _5, _2, _1, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
place!(Field::<bool>(Variant(_1, 3), 0)) = true;
_1 = Adt18::Variant2 { fld0: true,fld1: 268724022952130425337465018357738165110_u128 };
place!(Field::<u128>(Variant(_1, 2), 1)) = !142577703679204949055164362364762669119_u128;
RET = Field::<u128>(Variant(_1, 2), 1) as f32;
_3 = (103187387777056597428423715862971629268_i128,);
place!(Field::<bool>(Variant(_1, 2), 0)) = !true;
_1 = Adt18::Variant3 { fld0: false };
place!(Field::<bool>(Variant(_1, 3), 0)) = RET > RET;
_2 = Field::<bool>(Variant(_1, 3), 0) | Field::<bool>(Variant(_1, 3), 0);
RET = (-24_i8) as f32;
_4 = (-1409306769_i32) as isize;
place!(Field::<bool>(Variant(_1, 3), 0)) = _2;
_5 = [(-126_i8),107_i8,(-119_i8),100_i8];
Call(RET = fn9(_5, _2, _1, _5, _2, _1, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
RET = 35_i8 as f32;
_1 = Adt18::Variant3 { fld0: true };
RET = 1676428362480818430_i64 as f32;
place!(Field::<bool>(Variant(_1, 3), 0)) = false;
Goto(bb2)
}
bb17 = {
_2 = RET > RET;
_9 = &place!(Field::<u8>(Variant(_1, 0), 0));
_1 = Adt18::Variant0 { fld0: 182_u8,fld1: _3.0,fld2: 30_u16 };
_4 = (-9223372036854775808_isize) >> Field::<i128>(Variant(_1, 0), 1);
Goto(bb18)
}
bb18 = {
Call(_12 = dump_var(8_usize, 3_usize, Move(_3), 6_usize, Move(_6), 4_usize, Move(_4), 13_usize, _13), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i8; 4],mut _2: bool,mut _3: Adt18,mut _4: [i8; 4],mut _5: bool,mut _6: Adt18,mut _7: bool) -> f32 {
mir! {
type RET = f32;
let _8: u16;
let _9: u32;
let _10: i32;
let _11: isize;
let _12: [u16; 6];
let _13: Adt77;
let _14: &'static [u16; 1];
let _15: bool;
let _16: ([isize; 1], [u16; 7], i128, [i128; 7]);
let _17: &'static u8;
let _18: i128;
let _19: f64;
let _20: char;
let _21: u8;
let _22: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _23: char;
let _24: [u16; 7];
let _25: ();
let _26: ();
{
place!(Field::<bool>(Variant(_3, 3), 0)) = _7 != Field::<bool>(Variant(_6, 3), 0);
_3 = Adt18::Variant2 { fld0: _7,fld1: 4426711003242684165351819802043760275_u128 };
place!(Field::<u128>(Variant(_3, 2), 1)) = 273100641989897810907277308102850511486_u128 >> 193211247606117559149958812430181244691_u128;
_5 = Field::<bool>(Variant(_3, 2), 0);
_8 = !29966_u16;
_7 = !_5;
RET = (-1353_i16) as f32;
_6 = _3;
_5 = !Field::<bool>(Variant(_6, 2), 0);
RET = (-105_i8) as f32;
_10 = (-1642581164_i32) | 484088016_i32;
place!(Field::<u128>(Variant(_3, 2), 1)) = Field::<u128>(Variant(_6, 2), 1);
RET = 2398407848_u32 as f32;
place!(Field::<bool>(Variant(_6, 2), 0)) = _7;
_12 = [_8,_8,_8,_8,_8,_8];
place!(Field::<bool>(Variant(_3, 2), 0)) = Field::<bool>(Variant(_6, 2), 0) < _7;
_11 = (-9223372036854775808_isize);
_13.fld0 = Field::<bool>(Variant(_6, 2), 0) != _2;
_9 = 3159256284_u32;
_2 = !_5;
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_4 = _1;
_1 = [(-31_i8),(-84_i8),(-18_i8),(-70_i8)];
Goto(bb1)
}
bb1 = {
RET = _8 as f32;
_13.fld0 = _7 ^ Field::<bool>(Variant(_3, 2), 0);
place!(Field::<bool>(Variant(_3, 2), 0)) = !_13.fld0;
_15 = _5;
place!(Field::<u128>(Variant(_6, 2), 1)) = !Field::<u128>(Variant(_3, 2), 1);
place!(Field::<bool>(Variant(_3, 2), 0)) = _13.fld0 >= _13.fld0;
place!(Field::<bool>(Variant(_6, 2), 0)) = Field::<bool>(Variant(_3, 2), 0) ^ Field::<bool>(Variant(_3, 2), 0);
_16.2 = (-17_i8) as i128;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
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
_1 = [125_i8,(-124_i8),46_i8,(-68_i8)];
_15 = !Field::<bool>(Variant(_3, 2), 0);
_16.2 = -(-77367688872300288038328442017555429716_i128);
_13.fld0 = !Field::<bool>(Variant(_6, 2), 0);
_16.0 = [_11];
place!(Field::<u128>(Variant(_6, 2), 1)) = !Field::<u128>(Variant(_3, 2), 1);
RET = (-2853475561671315485_i64) as f32;
place!(Field::<bool>(Variant(_3, 2), 0)) = Field::<bool>(Variant(_6, 2), 0);
_6 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_3, 2), 0),fld1: Field::<u128>(Variant(_3, 2), 1) };
_7 = Field::<bool>(Variant(_6, 2), 0);
_7 = _13.fld0;
_16.2 = -5103406423395990687345511530826107472_i128;
_13.fld0 = Field::<bool>(Variant(_3, 2), 0);
_9 = Field::<u128>(Variant(_3, 2), 1) as u32;
_13.fld2 = Field::<u128>(Variant(_3, 2), 1) as f64;
_13.fld0 = Field::<bool>(Variant(_3, 2), 0) != _15;
RET = _11 as f32;
match _11 {
0 => bb4,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_3 = _6;
_15 = _5;
_15 = !_13.fld0;
_6 = Adt18::Variant0 { fld0: 153_u8,fld1: _16.2,fld2: _8 };
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_1 = [88_i8,82_i8,(-20_i8),(-8_i8)];
_16.3 = [_16.2,_16.2,_16.2,_16.2,Field::<i128>(Variant(_6, 0), 1),_16.2,_16.2];
_13.fld3 = _7 as i16;
place!(Field::<bool>(Variant(_3, 2), 0)) = _7;
_3 = Adt18::Variant2 { fld0: _7,fld1: 336310534348203707415729832172303638261_u128 };
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_6 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_3, 2), 0) };
_17 = &_21;
_22.1.2.fld4 = [0_i8,(-88_i8),(-42_i8),12_i8];
_22.3 = core::ptr::addr_of!(_22.1);
_21 = 184_u8 & 125_u8;
_9 = _10 as u32;
place!(Field::<bool>(Variant(_3, 2), 0)) = _7 <= _13.fld0;
_13.fld3 = 14853_i16;
_22.0 = &_11;
Goto(bb11)
}
bb11 = {
_18 = _16.2;
_1 = _22.1.2.fld4;
_19 = _13.fld2;
_22.1.2.fld6.0 = !(-127_i8);
_16.1 = [_8,_8,_8,_8,_8,_8,_8];
_10 = !(-2077764182_i32);
SetDiscriminant(_6, 2);
match _11 {
0 => bb12,
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
}
bb12 = {
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_3 = _6;
_15 = _5;
_15 = !_13.fld0;
_6 = Adt18::Variant0 { fld0: 153_u8,fld1: _16.2,fld2: _8 };
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_1 = [88_i8,82_i8,(-20_i8),(-8_i8)];
_16.3 = [_16.2,_16.2,_16.2,_16.2,Field::<i128>(Variant(_6, 0), 1),_16.2,_16.2];
_13.fld3 = _7 as i16;
place!(Field::<bool>(Variant(_3, 2), 0)) = _7;
_3 = Adt18::Variant2 { fld0: _7,fld1: 336310534348203707415729832172303638261_u128 };
_13.fld1 = [_9,_9,_9,_9,_9,_9];
_6 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_3, 2), 0) };
_17 = &_21;
_22.1.2.fld4 = [0_i8,(-88_i8),(-42_i8),12_i8];
_22.3 = core::ptr::addr_of!(_22.1);
_21 = 184_u8 & 125_u8;
_9 = _10 as u32;
place!(Field::<bool>(Variant(_3, 2), 0)) = _7 <= _13.fld0;
_13.fld3 = 14853_i16;
_22.0 = &_11;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<bool>(Variant(_6, 2), 0)) = _13.fld0;
_17 = &_22.2.0;
_22.1.2.fld1.2 = core::ptr::addr_of_mut!(_22.1.2.fld6.1);
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(9_usize, 4_usize, Move(_4), 12_usize, Move(_12), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(9_usize, 8_usize, Move(_8), 10_usize, Move(_10), 5_usize, Move(_5), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: (i8, isize, Adt18),mut _3: isize,mut _4: bool) -> [u64; 1] {
mir! {
type RET = [u64; 1];
let _5: f64;
let _6: (Adt65, (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31), (i8, isize, Adt18));
let _7: &'static isize;
let _8: (char, u64, *mut isize);
let _9: (char, f64);
let _10: char;
let _11: &'static u8;
let _12: &'static i16;
let _13: (char, f64);
let _14: &'static i16;
let _15: &'static &'static [i32; 4];
let _16: &'static (&'static &'static u8, usize);
let _17: i8;
let _18: u32;
let _19: Adt20;
let _20: (&'static &'static u8, usize);
let _21: char;
let _22: f32;
let _23: Adt62;
let _24: i8;
let _25: (([isize; 1], (char, f64), f32), ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), [u16; 6]);
let _26: ();
let _27: ();
{
RET = [9988108085586758549_u64];
RET = [9931499120319541400_u64];
_4 = true;
_5 = Field::<u8>(Variant(_2.2, 0), 0) as f64;
SetDiscriminant(_2.2, 1);
_6.1.3.fld4 = [_2.0,_2.0,_2.0,_2.0];
_6.0.fld1 = 158040114720680004073496457124260403515_u128 as u64;
Goto(bb1)
}
bb1 = {
_6.1.2 = Adt20::Variant1 { fld0: _5,fld1: '\u{ac15f}',fld2: 18204108253815054312684070737901392475_u128,fld3: _2.0,fld4: _6.0.fld1,fld5: 1754690983_i32,fld6: 3827083727_u32 };
_6.1.1.2.fld4 = [Field::<i8>(Variant(_6.1.2, 1), 3),_2.0,_2.0,_2.0];
_6.1.3.fld6.2 = Adt18::Variant0 { fld0: 65_u8,fld1: (-43964835729820875864920563714976118974_i128),fld2: 50707_u16 };
_6.1.1.2.fld6.0 = Field::<i8>(Variant(_6.1.2, 1), 3) << _1;
RET = [Field::<u64>(Variant(_6.1.2, 1), 4)];
_6.1.3.fld1.2 = core::ptr::addr_of_mut!(_6.2.1);
_6.1.1.2.fld2 = [_2.1];
_6.0.fld4 = [(-4045676405839514713_i64),8556649917085106286_i64,(-6757892941593419113_i64),2722995608071099201_i64,1892043126144565716_i64,(-1897555992878034971_i64)];
_6.1.1.2.fld1.0 = '\u{d793}';
_6.1.1.2.fld1.1 = 250_u8 as u64;
_6.1.3.fld2 = [_2.1];
match Field::<i8>(Variant(_6.1.2, 1), 3) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
99 => bb8,
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
_6.2.2 = Adt18::Variant0 { fld0: 88_u8,fld1: (-52481635411103222262878712670748604241_i128),fld2: 14421_u16 };
_6.1.1.1.0 = -_6.1.1.2.fld6.0;
_6.1.1.1.2 = Adt18::Variant3 { fld0: _4 };
_6.1.1.0 = RET;
_3 = _2.1 | _2.1;
_12 = &_6.1.0;
place!(Field::<i32>(Variant(_6.1.2, 1), 5)) = -(-1226287271_i32);
place!(Field::<u8>(Variant(_6.2.2, 0), 0)) = 215_u8;
_9.1 = _5 + _5;
_6.0.fld3 = 3531178303_u32;
_13.0 = _6.1.1.2.fld1.0;
_6.0.fld2.0 = (-25939952543306178462137509597375354111_i128);
_6.0.fld6 = [8853579617871966351_usize,7_usize,4_usize,5480835812645647859_usize,4_usize,4_usize,16162075952892945675_usize];
place!(Field::<u16>(Variant(_6.2.2, 0), 2)) = _13.0 as u16;
place!(Field::<u8>(Variant(_6.2.2, 0), 0)) = 169_u8;
place!(Field::<f64>(Variant(_6.1.2, 1), 0)) = -_9.1;
_6.1.3.fld6 = (_6.1.1.2.fld6.0, _3, _6.1.1.1.2);
Call(_6.1.0 = core::intrinsics::bswap(9940_i16), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11 = &place!(Field::<u8>(Variant(_6.2.2, 0), 0));
_13.1 = -Field::<f64>(Variant(_6.1.2, 1), 0);
_6.2.0 = !_6.1.1.1.0;
_6.2.0 = _6.1.1.2.fld6.0;
_6.1.1.1 = (_6.2.0, _3, _6.1.3.fld6.2);
_13.0 = _6.1.1.2.fld1.0;
_6.1.1.2.fld6 = _6.1.3.fld6;
_9.0 = _6.1.1.2.fld1.0;
_2.1 = _6.1.1.2.fld6.1 >> _6.1.1.1.1;
_6.1.1.0 = RET;
place!(Field::<u32>(Variant(_6.1.2, 1), 6)) = _6.0.fld3 * _6.0.fld3;
place!(Field::<isize>(Variant(_2.2, 1), 2)) = _6.1.1.2.fld6.1 & _2.1;
SetDiscriminant(_6.1.3.fld6.2, 3);
_10 = _6.1.1.2.fld1.0;
SetDiscriminant(_6.1.1.1.2, 3);
_8.0 = _9.0;
place!(Field::<f64>(Variant(_6.1.2, 1), 0)) = _5;
_6.1.1.2.fld0 = _6.2.0 as usize;
_11 = &(*_11);
place!(Field::<bool>(Variant(_6.1.3.fld6.2, 3), 0)) = Field::<isize>(Variant(_2.2, 1), 2) > _2.1;
_5 = -_9.1;
_7 = &_6.1.3.fld6.1;
_17 = 3246_i16 as i8;
_6.1.1.2.fld5.0 = !_6.0.fld2.0;
_6.1.3.fld4 = [_6.2.0,_6.1.1.1.0,_6.1.1.2.fld6.0,_2.0];
place!(Field::<i64>(Variant(_2.2, 1), 4)) = 271654982464002865708166355191952868886_u128 as i64;
Call(place!(Field::<f32>(Variant(_2.2, 1), 5)) = core::intrinsics::transmute(_8.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6.1.1.2.fld5 = (_6.0.fld2.0,);
_18 = !Field::<u32>(Variant(_6.1.2, 1), 6);
_6.2.2 = _6.1.3.fld6.2;
place!(Field::<u64>(Variant(_6.1.2, 1), 4)) = _6.1.1.2.fld1.1 * _6.0.fld1;
place!(Field::<f64>(Variant(_6.1.2, 1), 0)) = -_5;
_6.1.1.2.fld4 = _6.1.3.fld4;
_6.2.1 = !_2.1;
match _6.0.fld3 {
3531178303 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_8.2 = Move(_6.1.3.fld1.2);
SetDiscriminant(_6.1.3.fld6.2, 1);
_12 = &_6.1.0;
_6.1.3.fld5.0 = 60181_u16 as i128;
_6.1.3.fld3 = _6.2.0 + _6.2.0;
_6.1.3.fld1.2 = core::ptr::addr_of_mut!(_6.1.1.2.fld6.1);
place!(Field::<u64>(Variant(_6.1.3.fld6.2, 1), 0)) = !Field::<u64>(Variant(_6.1.2, 1), 4);
_8 = (_13.0, Field::<u64>(Variant(_6.1.2, 1), 4), Move(_6.1.3.fld1.2));
_6.0.fld0 = _6.1.1.2.fld6.0 >= _6.2.0;
place!(Field::<i64>(Variant(_6.1.3.fld6.2, 1), 4)) = Field::<i32>(Variant(_6.1.2, 1), 5) as i64;
place!(Field::<i8>(Variant(_6.1.2, 1), 3)) = _6.1.1.2.fld6.0;
place!(Field::<f64>(Variant(_6.1.3.fld6.2, 1), 1)) = 47740_u16 as f64;
_9 = _13;
Goto(bb13)
}
bb13 = {
_6.1.3.fld4 = [_6.1.1.1.0,_6.1.1.1.0,_6.2.0,_6.1.3.fld6.0];
_6.1.3.fld6.1 = _6.1.1.1.1;
Call(_6.0.fld2 = fn11(), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_6.1.3.fld7 = Move(_8.2);
_6.1.1.2.fld7 = core::ptr::addr_of_mut!(_6.2.1);
_6.0.fld3 = !_18;
_6.1.1.2.fld1.0 = _10;
_6.1.1.1.2 = Adt18::Variant0 { fld0: 142_u8,fld1: _6.1.1.2.fld5.0,fld2: 42949_u16 };
place!(Field::<u128>(Variant(_6.1.2, 1), 2)) = Field::<i64>(Variant(_6.1.3.fld6.2, 1), 4) as u128;
_6.1.1.2.fld1.2 = core::ptr::addr_of_mut!(_3);
place!(Field::<f32>(Variant(_6.1.3.fld6.2, 1), 5)) = Field::<f32>(Variant(_2.2, 1), 5) - Field::<f32>(Variant(_2.2, 1), 5);
_6.1.1.2.fld0 = 6_usize >> _6.0.fld3;
place!(Field::<[isize; 1]>(Variant(_6.1.3.fld6.2, 1), 3)) = [Field::<isize>(Variant(_2.2, 1), 2)];
place!(Field::<f32>(Variant(_2.2, 1), 5)) = Field::<f32>(Variant(_6.1.3.fld6.2, 1), 5);
_6.1.1.2.fld3 = !_6.1.3.fld6.0;
_2.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_6.2.2, 3), 0),fld1: Field::<u128>(Variant(_6.1.2, 1), 2) };
_6.1.3.fld7 = core::ptr::addr_of_mut!(_3);
_16 = &_20;
place!(Field::<f32>(Variant(_6.1.3.fld6.2, 1), 5)) = 30105_u16 as f32;
place!(Field::<f64>(Variant(_6.1.3.fld6.2, 1), 1)) = Field::<f64>(Variant(_6.1.2, 1), 0);
place!(Field::<i64>(Variant(_6.1.3.fld6.2, 1), 4)) = !(-5868336725237729656_i64);
_6.1.1.3 = Adt20::Variant1 { fld0: _9.1,fld1: _9.0,fld2: Field::<u128>(Variant(_6.1.2, 1), 2),fld3: _6.1.1.2.fld6.0,fld4: _6.1.1.2.fld1.1,fld5: Field::<i32>(Variant(_6.1.2, 1), 5),fld6: _18 };
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(10_usize, 17_usize, Move(_17), 10_usize, Move(_10), 4_usize, Move(_4), 27_usize, _27), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> (i128,) {
mir! {
type RET = (i128,);
let _1: isize;
let _2: f64;
let _3: *const usize;
let _4: &'static [u16; 1];
let _5: f32;
let _6: u64;
let _7: &'static [u16; 1];
let _8: ([u32; 8], (i8, isize, Adt18));
let _9: char;
let _10: isize;
let _11: bool;
let _12: ([usize; 2],);
let _13: (u8, u64, [i32; 4], (char, u64, *mut isize));
let _14: f64;
let _15: [bool; 3];
let _16: ([isize; 1], (char, f64), f32);
let _17: u8;
let _18: f32;
let _19: isize;
let _20: (Adt65, (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31), (i8, isize, Adt18));
let _21: (i8, isize, Adt18);
let _22: isize;
let _23: [u64; 5];
let _24: f64;
let _25: i8;
let _26: isize;
let _27: &'static u8;
let _28: f32;
let _29: isize;
let _30: u32;
let _31: (u8, i8, [bool; 3], ([usize; 2],));
let _32: &'static (&'static &'static u8, usize);
let _33: f64;
let _34: i64;
let _35: (i8, isize, Adt18);
let _36: (char, u64, *mut isize);
let _37: (u8, u64, [i32; 4], (char, u64, *mut isize));
let _38: *const usize;
let _39: usize;
let _40: u32;
let _41: ([usize; 2],);
let _42: f32;
let _43: i8;
let _44: ();
let _45: ();
{
RET = (46023341928019541691071378016054416188_i128,);
RET = ((-65669670104517053083661748611141736708_i128),);
RET = (156021385897794121239623641283639477706_i128,);
RET = (169397048009904696406031844225825766099_i128,);
RET = (10383215272269526623448540915876576421_i128,);
RET.0 = 125_isize as i128;
RET = ((-28284854967219088602267854475628831770_i128),);
RET = (20272063563889321814042239717559566556_i128,);
RET.0 = (-77103596952088012285958416214407613308_i128) * 161840151458845922813864809839455475265_i128;
RET = ((-104045524743133694642986808904630392734_i128),);
RET.0 = 90801782174921025970576784803295421214_i128;
_1 = -9223372036854775807_isize;
RET = (24471627098710657618063129392797229386_i128,);
RET = (76881612301818470868064454128640937842_i128,);
RET.0 = 3679775100674865068_i64 as i128;
RET = ((-23177907373737099875279676189487536932_i128),);
Goto(bb1)
}
bb1 = {
RET.0 = !114938625589264490189940593482882578686_i128;
_1 = 6_usize as isize;
_2 = 4871871273988608133_i64 as f64;
RET.0 = !(-66866138097475281225143902515010120366_i128);
RET = (166115643341149773329612097317859315799_i128,);
_2 = (-1616509713_i32) as f64;
RET = (95991060810029938790414871931723282980_i128,);
Goto(bb2)
}
bb2 = {
_1 = 9223372036854775807_isize;
_6 = 2720301576065771412_u64 << _1;
_9 = '\u{169d8}';
_5 = 171_u8 as f32;
_5 = 277910331723157488986058641642177561230_u128 as f32;
RET.0 = (-69596192357498628849754726361347983664_i128);
RET = (36400494098649285009710875358695915445_i128,);
RET = ((-146306981693580466258618094980965494_i128),);
_8.1.1 = _1 - _1;
_5 = 31831_u16 as f32;
_12.0 = [3214209203554867488_usize,1642656542812243618_usize];
_9 = '\u{9dfa1}';
_13.0 = 874276210_i32 as u8;
_11 = RET.0 == RET.0;
_13.2 = [1836078931_i32,(-349874401_i32),(-875262769_i32),1729509997_i32];
RET = ((-33298865144038592218253549161930657993_i128),);
Goto(bb3)
}
bb3 = {
_10 = _8.1.1;
_6 = !8246306770703300183_u64;
_13.0 = 128_u8;
RET.0 = -6832384009118030024308212137370274449_i128;
_14 = -_2;
_9 = '\u{b47ef}';
_14 = _2 + _2;
_16.1.1 = _14 * _14;
_8.1.0 = (-6617171882459774605_i64) as i8;
_2 = _14 + _14;
_8.1.2 = Adt18::Variant2 { fld0: _11,fld1: 294770469359518734186887108438341864574_u128 };
_20.1.1.2.fld5.0 = !RET.0;
_16.2 = -_5;
_13.1 = _6 - _6;
Call(_21.0 = fn12(_12.0, _2, _13.2, _12.0, _16.1.1, _8.1.1, _9, _10, _12.0, _13.2, _12.0, _8.1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = _16.2;
_20.1.0 = 814056178_u32 as i16;
_15 = [_11,_11,_11];
_20.1.1.2.fld1.2 = core::ptr::addr_of_mut!(_20.1.3.fld6.1);
RET = (_20.1.1.2.fld5.0,);
_20.0.fld2.0 = _21.0 as i128;
_20.1.1.2.fld4 = [_21.0,_21.0,_21.0,_21.0];
_16.0 = [_8.1.1];
_21.2 = Adt18::Variant1 { fld0: _13.1,fld1: _16.1.1,fld2: _8.1.1,fld3: _16.0,fld4: 2706474590601456286_i64,fld5: _5 };
Goto(bb5)
}
bb5 = {
RET = _20.1.1.2.fld5;
_18 = _16.2 - _16.2;
_20.1.3.fld6.2 = Adt18::Variant1 { fld0: _6,fld1: Field::<f64>(Variant(_21.2, 1), 1),fld2: _10,fld3: _16.0,fld4: (-1286971360557064597_i64),fld5: _18 };
_20.1.3.fld4 = _20.1.1.2.fld4;
_20.0.fld4 = [3793086194164275499_i64,4085602834355378204_i64,(-7316092402470602106_i64),2306612121143292264_i64,1109065226875190022_i64,(-7757876776083872034_i64)];
_20.2.2 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_8.1.2, 2), 0) };
_22 = -_10;
SetDiscriminant(_20.2.2, 3);
_20.1.1.0 = [_13.1];
place!(Field::<i64>(Variant(_21.2, 1), 4)) = 4337131851527052572_i64 | (-3580508011548464297_i64);
_20.1.3.fld3 = 7726_u16 as i8;
_13.3.1 = !_6;
place!(Field::<f64>(Variant(_21.2, 1), 1)) = _2;
_20.1.3.fld7 = core::ptr::addr_of_mut!(_8.1.1);
_23 = [_13.1,Field::<u64>(Variant(_21.2, 1), 0),Field::<u64>(Variant(_20.1.3.fld6.2, 1), 0),Field::<u64>(Variant(_20.1.3.fld6.2, 1), 0),_6];
_8.1 = (_21.0, _22, _21.2);
_20.1.1.2.fld3 = _21.0 | _21.0;
Goto(bb6)
}
bb6 = {
_20.1.1.2.fld4 = _20.1.3.fld4;
_20.1.3.fld1.0 = _9;
match _1 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb7 = {
_10 = _8.1.1;
_6 = !8246306770703300183_u64;
_13.0 = 128_u8;
RET.0 = -6832384009118030024308212137370274449_i128;
_14 = -_2;
_9 = '\u{b47ef}';
_14 = _2 + _2;
_16.1.1 = _14 * _14;
_8.1.0 = (-6617171882459774605_i64) as i8;
_2 = _14 + _14;
_8.1.2 = Adt18::Variant2 { fld0: _11,fld1: 294770469359518734186887108438341864574_u128 };
_20.1.1.2.fld5.0 = !RET.0;
_16.2 = -_5;
_13.1 = _6 - _6;
Call(_21.0 = fn12(_12.0, _2, _13.2, _12.0, _16.1.1, _8.1.1, _9, _10, _12.0, _13.2, _12.0, _8.1.1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_5 = _16.2;
_20.1.0 = 814056178_u32 as i16;
_15 = [_11,_11,_11];
_20.1.1.2.fld1.2 = core::ptr::addr_of_mut!(_20.1.3.fld6.1);
RET = (_20.1.1.2.fld5.0,);
_20.0.fld2.0 = _21.0 as i128;
_20.1.1.2.fld4 = [_21.0,_21.0,_21.0,_21.0];
_16.0 = [_8.1.1];
_21.2 = Adt18::Variant1 { fld0: _13.1,fld1: _16.1.1,fld2: _8.1.1,fld3: _16.0,fld4: 2706474590601456286_i64,fld5: _5 };
Goto(bb5)
}
bb9 = {
_16.1.1 = _14 - Field::<f64>(Variant(_20.1.3.fld6.2, 1), 1);
_13.3.2 = core::ptr::addr_of_mut!(_22);
_21.1 = _20.1.3.fld1.0 as isize;
_20.1.3.fld1.2 = core::ptr::addr_of_mut!(_8.1.1);
_20.1.3.fld5.0 = _20.0.fld2.0 ^ _20.0.fld2.0;
_20.0.fld3 = Field::<f32>(Variant(_8.1.2, 1), 5) as u32;
_26 = _8.1.1 * Field::<isize>(Variant(_21.2, 1), 2);
_21.1 = -Field::<isize>(Variant(_8.1.2, 1), 2);
_20.1.1.1.1 = _26;
_20.1.3.fld2 = [_10];
_20.2 = (_8.1.0, Field::<isize>(Variant(_21.2, 1), 2), _8.1.2);
_13.3 = (_9, Field::<u64>(Variant(_20.2.2, 1), 0), Move(_20.1.3.fld7));
_20.1.1.2.fld6 = (_21.0, _22, _21.2);
place!(Field::<[isize; 1]>(Variant(_20.2.2, 1), 3)) = Field::<[isize; 1]>(Variant(_20.1.3.fld6.2, 1), 3);
RET.0 = _13.3.0 as i128;
Call(_13.1 = core::intrinsics::transmute(_20.1.1.1.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4)) = _20.2.0 as i64;
_13.1 = _13.3.1 - _6;
_18 = Field::<f64>(Variant(_8.1.2, 1), 1) as f32;
_20.1.3.fld2 = [_26];
_31.1 = _20.2.0 << Field::<u64>(Variant(_20.2.2, 1), 0);
_29 = (-1187228378_i32) as isize;
_20.1.1.1 = _21;
place!(Field::<i64>(Variant(_8.1.2, 1), 4)) = Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4) ^ Field::<i64>(Variant(_21.2, 1), 4);
_8.1 = (_20.1.1.2.fld3, _22, _20.2.2);
_21.0 = _20.2.0;
_31.3.0 = [7_usize,15587493429300557994_usize];
place!(Field::<f32>(Variant(_21.2, 1), 5)) = -_18;
_20.0.fld1 = Field::<u64>(Variant(_20.1.1.2.fld6.2, 1), 0);
_16.1 = (_9, Field::<f64>(Variant(_20.1.1.1.2, 1), 1));
_20.1.1.1.2 = Adt18::Variant0 { fld0: _13.0,fld1: _20.1.3.fld5.0,fld2: 23012_u16 };
_20.1.3.fld7 = Move(_13.3.2);
_20.0.fld4 = [Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4),Field::<i64>(Variant(_20.2.2, 1), 4),Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4),Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4),Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4),Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4)];
_13.3.2 = Move(_20.1.3.fld1.2);
_21.2 = _8.1.2;
_18 = _20.0.fld3 as f32;
_16.2 = -Field::<f32>(Variant(_20.1.3.fld6.2, 1), 5);
Goto(bb11)
}
bb11 = {
_20.2.1 = Field::<isize>(Variant(_21.2, 1), 2);
_16.2 = Field::<f32>(Variant(_20.1.3.fld6.2, 1), 5) + Field::<f32>(Variant(_20.1.3.fld6.2, 1), 5);
_26 = _10;
_25 = -_20.1.1.2.fld6.0;
_20.2 = _8.1;
place!(Field::<isize>(Variant(_20.2.2, 1), 2)) = _20.1.3.fld1.0 as isize;
_20.1.3.fld1.2 = Move(_13.3.2);
_20.1.1.2.fld4 = [_25,_31.1,_8.1.0,_8.1.0];
_20.1.1.2.fld6.0 = -_21.0;
_20.1.1.3 = Adt20::Variant0 { fld0: _20.1.3.fld6.2,fld1: Field::<[isize; 1]>(Variant(_20.1.1.2.fld6.2, 1), 3),fld2: _6,fld3: _20.1.1.1.0,fld4: _20.1.0,fld5: _20.0.fld3,fld6: Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4),fld7: _20.1.3.fld5.0 };
place!(Field::<f32>(Variant(_20.1.3.fld6.2, 1), 5)) = 5_usize as f32;
place!(Field::<i128>(Variant(_20.1.1.1.2, 0), 1)) = _20.1.3.fld5.0 | _20.1.3.fld5.0;
_23 = [_20.0.fld1,_13.1,_13.1,Field::<u64>(Variant(_8.1.2, 1), 0),Field::<u64>(Variant(_8.1.2, 1), 0)];
_31.0 = Field::<u8>(Variant(_20.1.1.1.2, 0), 0) & Field::<u8>(Variant(_20.1.1.1.2, 0), 0);
_20.0.fld5 = _14 - Field::<f64>(Variant(_20.1.3.fld6.2, 1), 1);
_20.1.2 = Move(_20.1.1.3);
_8.1.0 = _20.1.1.2.fld6.0 & _20.1.1.2.fld6.0;
place!(Field::<u64>(Variant(_21.2, 1), 0)) = !Field::<u64>(Variant(_8.1.2, 1), 0);
Goto(bb12)
}
bb12 = {
_36 = (_20.1.3.fld1.0, Field::<u64>(Variant(_21.2, 1), 0), Move(_20.1.3.fld1.2));
_20.1.1.2.fld1.0 = _9;
SetDiscriminant(_20.1.1.2.fld6.2, 1);
_16.2 = Field::<f32>(Variant(_21.2, 1), 5);
_31.0 = Field::<f64>(Variant(Field::<Adt18>(Variant(_20.1.2, 0), 0), 1), 1) as u8;
place!(Field::<f64>(Variant(_20.2.2, 1), 1)) = Field::<f64>(Variant(_21.2, 1), 1);
_31 = (Field::<u8>(Variant(_20.1.1.1.2, 0), 0), _20.1.1.2.fld6.0, _15, _12);
place!(Field::<i8>(Variant(_20.1.2, 0), 3)) = _20.1.3.fld3 - _20.1.1.1.0;
_28 = Field::<f32>(Variant(_8.1.2, 1), 5);
_35.2 = Adt18::Variant2 { fld0: _11,fld1: 67589241198722144700281412460979662791_u128 };
Goto(bb13)
}
bb13 = {
_37 = (_31.0, Field::<u64>(Variant(_8.1.2, 1), 0), _13.2, Move(_36));
_11 = Field::<bool>(Variant(_35.2, 2), 0);
place!(Field::<i64>(Variant(place!(Field::<Adt18>(Variant(_20.1.2, 0), 0)), 1), 4)) = !Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4);
_16.1.1 = Field::<f64>(Variant(_8.1.2, 1), 1);
_31.3 = (_12.0,);
place!(Field::<u8>(Variant(_20.1.1.1.2, 0), 0)) = _13.1 as u8;
SetDiscriminant(_8.1.2, 0);
place!(Field::<u8>(Variant(_8.1.2, 0), 0)) = _20.0.fld3 as u8;
place!(Field::<[isize; 1]>(Variant(place!(Field::<Adt18>(Variant(_20.1.2, 0), 0)), 1), 3)) = [_22];
_8.1.0 = !_31.1;
_20.1.1.2.fld1 = (_13.3.0, _37.1, Move(_37.3.2));
_13.3.2 = Move(_20.1.3.fld7);
place!(Field::<i16>(Variant(_20.1.2, 0), 4)) = _20.1.0 * _20.1.0;
place!(Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4)) = Field::<i64>(Variant(Field::<Adt18>(Variant(_20.1.2, 0), 0), 1), 4) * Field::<i64>(Variant(_20.1.2, 0), 6);
_20.1.3.fld1.1 = _6 - Field::<u64>(Variant(Field::<Adt18>(Variant(_20.1.2, 0), 0), 1), 0);
_20.1.3.fld1.0 = _9;
SetDiscriminant(_20.1.2, 0);
_13.1 = !_6;
Goto(bb14)
}
bb14 = {
_35.1 = Field::<i64>(Variant(_20.1.3.fld6.2, 1), 4) as isize;
_34 = _20.0.fld3 as i64;
_13.1 = !_20.0.fld1;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(11_usize, 15_usize, Move(_15), 10_usize, Move(_10), 34_usize, Move(_34), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(11_usize, 22_usize, Move(_22), 11_usize, Move(_11), 6_usize, Move(_6), 45_usize, _45), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [usize; 2],mut _2: f64,mut _3: [i32; 4],mut _4: [usize; 2],mut _5: f64,mut _6: isize,mut _7: char,mut _8: isize,mut _9: [usize; 2],mut _10: [i32; 4],mut _11: [usize; 2],mut _12: isize) -> i8 {
mir! {
type RET = i8;
let _13: f32;
let _14: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _15: Adt47;
let _16: Adt52;
let _17: u64;
let _18: char;
let _19: char;
let _20: isize;
let _21: [char; 1];
let _22: f32;
let _23: isize;
let _24: char;
let _25: Adt31;
let _26: f32;
let _27: &'static &'static u8;
let _28: isize;
let _29: f32;
let _30: [usize; 2];
let _31: isize;
let _32: *const usize;
let _33: i16;
let _34: &'static u8;
let _35: f64;
let _36: [i8; 4];
let _37: u128;
let _38: i128;
let _39: Adt31;
let _40: Adt77;
let _41: u32;
let _42: *mut isize;
let _43: char;
let _44: ();
let _45: ();
{
_11 = [0_usize,13185086267678276182_usize];
RET = (-96_i8) * (-103_i8);
_2 = _5;
_2 = _5 + _5;
_9 = [5_usize,8077010729726765953_usize];
_11 = [4_usize,2_usize];
RET = 112_i8;
_4 = [5_usize,5_usize];
_3 = _10;
_3 = [1281843854_i32,1151858025_i32,(-216667905_i32),(-1201253664_i32)];
_6 = _12;
_12 = (-17189_i16) as isize;
_10 = [658494600_i32,529974082_i32,(-1947659554_i32),(-1985067740_i32)];
_5 = _2 + _2;
_13 = 1874461963_i32 as f32;
_12 = _7 as isize;
RET = !72_i8;
_7 = '\u{1081d6}';
_14.1.2.fld1.1 = !13686017738958474174_u64;
_14.3 = core::ptr::addr_of!(_14.1);
_11 = [2_usize,3_usize];
_14.1.2.fld6.1 = !_12;
_4 = _1;
_14.1.2.fld4 = [RET,RET,RET,RET];
_14.1.2.fld2 = [_14.1.2.fld6.1];
Goto(bb1)
}
bb1 = {
_14.2.0 = _14.1.2.fld1.1 as u8;
_13 = (-19754_i16) as f32;
_14.1.2.fld3 = _13 as i8;
_17 = _14.1.2.fld1.1;
_9 = _11;
_14.2.2 = [true,true,false];
_14.1.2.fld5.0 = (-15564509251975020927406335415556555914_i128) & 52290773513491956046248320278976324413_i128;
_14.1.2.fld1.0 = _7;
Goto(bb2)
}
bb2 = {
_14.3 = core::ptr::addr_of!(_14.1);
_1 = [9957060019886665478_usize,3_usize];
_16 = Adt52::Variant2 { fld0: _14.1.2.fld5 };
_14.1.1.2 = Adt18::Variant1 { fld0: _17,fld1: _5,fld2: _8,fld3: _14.1.2.fld2,fld4: 1788787433446101234_i64,fld5: _13 };
_14.1.2.fld2 = [Field::<isize>(Variant(_14.1.1.2, 1), 2)];
_14.2.0 = !126_u8;
place!(Field::<u64>(Variant(_14.1.1.2, 1), 0)) = !_14.1.2.fld1.1;
_14.1.2.fld6.2 = Adt18::Variant3 { fld0: false };
_14.1.1.0 = !_14.1.2.fld3;
_3 = [(-1309746390_i32),(-1890890463_i32),261317402_i32,(-1503154384_i32)];
_4 = [6453134652495073958_usize,10984880957015073089_usize];
Goto(bb3)
}
bb3 = {
place!(Field::<u64>(Variant(_14.1.1.2, 1), 0)) = (-1087203955_i32) as u64;
Goto(bb4)
}
bb4 = {
_14.1.2.fld6.2 = Adt18::Variant2 { fld0: true,fld1: 88291636522196653324103979017291031992_u128 };
SetDiscriminant(_16, 0);
Goto(bb5)
}
bb5 = {
_14.1.3 = Adt20::Variant1 { fld0: _5,fld1: _7,fld2: 316906566685499987354598134874333036349_u128,fld3: _14.1.2.fld3,fld4: _17,fld5: (-469823325_i32),fld6: 3341492402_u32 };
_4 = [6849992726327687317_usize,1_usize];
place!(Field::<[isize; 1]>(Variant(_14.1.1.2, 1), 3)) = _14.1.2.fld2;
_14.2.1 = Field::<i8>(Variant(_14.1.3, 1), 3) & Field::<i8>(Variant(_14.1.3, 1), 3);
_19 = _7;
place!(Field::<i64>(Variant(_14.1.1.2, 1), 4)) = 787838746_i32 as i64;
_14.2.2 = [true,false,true];
_8 = !Field::<isize>(Variant(_14.1.1.2, 1), 2);
_5 = 53849_u16 as f64;
place!(Field::<i64>(Variant(_14.1.1.2, 1), 4)) = -(-2185648396242096749_i64);
_14.1.0 = [_17];
Goto(bb6)
}
bb6 = {
_14.1.0 = [Field::<u64>(Variant(_14.1.1.2, 1), 0)];
_14.1.2.fld0 = 6_usize * 4_usize;
place!(Field::<u32>(Variant(_14.1.3, 1), 6)) = 55965740_u32 * 2239205376_u32;
_14.3 = core::ptr::addr_of!(_14.1);
_25.fld6.2 = _14.1.1.2;
Goto(bb7)
}
bb7 = {
_25.fld1.1 = _17 - Field::<u64>(Variant(_14.1.3, 1), 4);
_14.1.2.fld6 = (_14.2.1, Field::<isize>(Variant(_25.fld6.2, 1), 2), _25.fld6.2);
_14.1.2.fld2 = Field::<[isize; 1]>(Variant(_14.1.2.fld6.2, 1), 3);
_25.fld1.2 = core::ptr::addr_of_mut!(_12);
_14.1.1.2 = _25.fld6.2;
RET = 15537_i16 as i8;
place!(Field::<u128>(Variant(_14.1.3, 1), 2)) = 53298_u16 as u128;
_14.1.2.fld1.2 = core::ptr::addr_of_mut!(_14.1.2.fld6.1);
_23 = !_14.1.2.fld6.1;
_25.fld1.2 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_14.1.2.fld6.2, 1), 2)));
_25.fld7 = core::ptr::addr_of_mut!(_12);
_16 = Adt52::Variant0 { fld0: _14.1.1.0 };
_25 = Adt31 { fld0: _14.1.2.fld0,fld1: Move(_14.1.2.fld1),fld2: Field::<[isize; 1]>(Variant(_14.1.2.fld6.2, 1), 3),fld3: _14.1.2.fld3,fld4: _14.1.2.fld4,fld5: _14.1.2.fld5,fld6: _14.1.2.fld6,fld7: Move(_14.1.2.fld1.2) };
_14.1.2.fld1.2 = Move(_25.fld1.2);
_25.fld1 = (_7, Field::<u64>(Variant(_14.1.1.2, 1), 0), Move(_14.1.2.fld1.2));
_12 = Field::<i64>(Variant(_14.1.1.2, 1), 4) as isize;
_14.1.2.fld5 = (_25.fld5.0,);
_18 = _25.fld1.0;
_14.1.2.fld1 = Move(_25.fld1);
_14.3 = core::ptr::addr_of!(_14.1);
_25.fld6 = _14.1.2.fld6;
_14.1.2.fld1 = (Field::<char>(Variant(_14.1.3, 1), 1), Field::<u64>(Variant(_14.1.3, 1), 4), Move(_25.fld7));
_25.fld6.0 = _14.1.2.fld3 | Field::<i8>(Variant(_14.1.3, 1), 3);
Call(place!(Field::<f64>(Variant(_14.1.3, 1), 0)) = core::intrinsics::transmute(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_32 = core::ptr::addr_of!(_25.fld0);
_14.2.2 = [false,false,false];
place!(Field::<i64>(Variant(_25.fld6.2, 1), 4)) = Field::<i64>(Variant(_14.1.2.fld6.2, 1), 4) & Field::<i64>(Variant(_14.1.2.fld6.2, 1), 4);
place!(Field::<isize>(Variant(_14.1.2.fld6.2, 1), 2)) = Field::<isize>(Variant(_14.1.1.2, 1), 2);
place!(Field::<f32>(Variant(_14.1.2.fld6.2, 1), 5)) = _14.1.2.fld1.1 as f32;
_14.2.3.0 = [_14.1.2.fld0,(*_32)];
_14.1.2.fld7 = Move(_14.1.2.fld1.2);
_14.1.2.fld1.0 = _18;
_17 = !Field::<u64>(Variant(_14.1.2.fld6.2, 1), 0);
place!(Field::<u64>(Variant(_14.1.1.2, 1), 0)) = Field::<u64>(Variant(_14.1.3, 1), 4);
place!(Field::<i8>(Variant(_14.1.3, 1), 3)) = _14.2.1;
_14.1.1.1 = _14.2.1 as isize;
place!(Field::<u64>(Variant(_14.1.3, 1), 4)) = 33257_u16 as u64;
Call(_14.1.3 = fn13(_14.1.1.2, Move(_14.3), _25.fld6.2, _14.1.1.2, _14.1.1, _14.1.1, Field::<f64>(Variant(_25.fld6.2, 1), 1), _14.2.2, _13), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_28 = _25.fld6.1;
_25.fld6.1 = _14.1.2.fld6.1;
_34 = &_14.2.0;
_14.0 = &place!(Field::<isize>(Variant(_14.1.2.fld6.2, 1), 2));
_3 = [Field::<i32>(Variant(_14.1.3, 1), 5),Field::<i32>(Variant(_14.1.3, 1), 5),Field::<i32>(Variant(_14.1.3, 1), 5),Field::<i32>(Variant(_14.1.3, 1), 5)];
RET = _25.fld6.0;
_17 = Field::<u64>(Variant(_14.1.2.fld6.2, 1), 0) - Field::<u64>(Variant(_14.1.3, 1), 4);
_25.fld1.2 = core::ptr::addr_of_mut!(_6);
_14.1.2.fld1.1 = !Field::<u64>(Variant(_14.1.1.2, 1), 0);
_26 = -_13;
_14.1.2.fld4 = _25.fld4;
place!(Field::<i64>(Variant(_14.1.2.fld6.2, 1), 4)) = Field::<i64>(Variant(_25.fld6.2, 1), 4) | Field::<i64>(Variant(_25.fld6.2, 1), 4);
place!(Field::<u32>(Variant(_14.1.3, 1), 6)) = _14.2.0 as u32;
_13 = Field::<f32>(Variant(_14.1.1.2, 1), 5);
_24 = Field::<char>(Variant(_14.1.3, 1), 1);
_25.fld6.0 = _25.fld5.0 as i8;
_32 = core::ptr::addr_of!(_14.1.2.fld0);
Goto(bb10)
}
bb10 = {
_14.1.2.fld1.2 = Move(_14.1.2.fld7);
_29 = -Field::<f32>(Variant(_14.1.1.2, 1), 5);
place!(Field::<i64>(Variant(_14.1.2.fld6.2, 1), 4)) = _5 as i64;
_14.1.1.0 = RET;
_28 = -_14.1.1.1;
_26 = Field::<f32>(Variant(_14.1.2.fld6.2, 1), 5);
place!(Field::<f64>(Variant(_14.1.2.fld6.2, 1), 1)) = -Field::<f64>(Variant(_25.fld6.2, 1), 1);
place!(Field::<f32>(Variant(_14.1.2.fld6.2, 1), 5)) = Field::<f32>(Variant(_14.1.1.2, 1), 5) + _29;
_25.fld1 = Move(_14.1.2.fld1);
RET = _14.1.2.fld6.0;
_33 = Field::<char>(Variant(_14.1.3, 1), 1) as i16;
match Field::<u128>(Variant(_14.1.3, 1), 2) {
0 => bb1,
1 => bb2,
2 => bb3,
158186856023052464941447787271662489873 => bb11,
_ => bb8
}
}
bb11 = {
place!(Field::<i32>(Variant(_14.1.3, 1), 5)) = (-1564970676_i32) + 1058924955_i32;
_34 = &(*_34);
_25.fld6 = _14.1.2.fld6;
_33 = 49143_u16 as i16;
place!(Field::<i64>(Variant(_25.fld6.2, 1), 4)) = Field::<i64>(Variant(_14.1.2.fld6.2, 1), 4);
_19 = Field::<char>(Variant(_14.1.3, 1), 1);
_6 = _19 as isize;
place!(Field::<f64>(Variant(_25.fld6.2, 1), 1)) = _2;
_14.1.2.fld6 = (_14.2.1, _8, _14.1.1.2);
SetDiscriminant(_14.1.3, 1);
_25.fld3 = -_14.1.2.fld6.0;
_41 = _6 as u32;
_11 = [(*_32),_25.fld0];
SetDiscriminant(_14.1.1.2, 0);
Call(_39.fld3 = fn15(_14.1.2.fld6.0, _14.1.2.fld6.2, _14.1.2.fld6, _6, Field::<[isize; 1]>(Variant(_25.fld6.2, 1), 3), _29, _25.fld6.1, _14.1.2.fld6, Field::<f64>(Variant(_14.1.2.fld6.2, 1), 1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_24 = _18;
_40.fld0 = true;
_25.fld4 = _14.1.2.fld4;
Goto(bb13)
}
bb13 = {
_39.fld6.2 = _14.1.2.fld6.2;
_39.fld4 = _25.fld4;
_14.1.2.fld6.1 = Field::<isize>(Variant(_39.fld6.2, 1), 2) * _23;
_30 = [_14.1.2.fld0,_14.1.2.fld0];
place!(Field::<char>(Variant(_14.1.3, 1), 1)) = _18;
_30 = _11;
_14.1.1.2 = Adt18::Variant0 { fld0: _14.2.0,fld1: _14.1.2.fld5.0,fld2: 1149_u16 };
_25.fld2 = [_6];
place!(Field::<i64>(Variant(_39.fld6.2, 1), 4)) = Field::<i128>(Variant(_14.1.1.2, 0), 1) as i64;
_39.fld1.0 = _19;
SetDiscriminant(_25.fld6.2, 2);
RET = _39.fld3 << Field::<isize>(Variant(_14.1.2.fld6.2, 1), 2);
SetDiscriminant(_16, 2);
place!(Field::<u128>(Variant(_14.1.3, 1), 2)) = 9704796402728471473207236600928843787_u128 ^ 21248990641464568459833192108532908923_u128;
SetDiscriminant(_14.1.2.fld6.2, 0);
SetDiscriminant(_39.fld6.2, 2);
_14.1.2.fld1.1 = (-4215903619794699728_i64) as u64;
_25.fld3 = -_39.fld3;
_42 = Move(_25.fld1.2);
_25.fld6.2 = Adt18::Variant1 { fld0: _17,fld1: _2,fld2: _14.1.2.fld6.1,fld3: _25.fld2,fld4: (-5165677943375322695_i64),fld5: _29 };
place!(Field::<u16>(Variant(_14.1.1.2, 0), 2)) = 5123_u16 * 1192_u16;
_14.1.2.fld1.2 = Move(_42);
_22 = -_26;
_27 = &_34;
SetDiscriminant(_14.1.1.2, 2);
_34 = &_14.2.0;
_27 = &_34;
_38 = _25.fld5.0 >> _14.1.1.1;
place!(Field::<u64>(Variant(_14.1.3, 1), 4)) = _40.fld0 as u64;
Goto(bb14)
}
bb14 = {
Call(_44 = dump_var(12_usize, 6_usize, Move(_6), 9_usize, Move(_9), 33_usize, Move(_33), 38_usize, Move(_38)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_44 = dump_var(12_usize, 3_usize, Move(_3), 23_usize, Move(_23), 30_usize, Move(_30), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(12_usize, 18_usize, Move(_18), 11_usize, Move(_11), 45_usize, _45, 45_usize, _45), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: Adt18,mut _2: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20),mut _3: Adt18,mut _4: Adt18,mut _5: (i8, isize, Adt18),mut _6: (i8, isize, Adt18),mut _7: f64,mut _8: [bool; 3],mut _9: f32) -> Adt20 {
mir! {
type RET = Adt20;
let _10: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _11: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31);
let _12: ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _13: char;
let _14: (i8, isize, Adt18);
let _15: (&'static [i32; 4], &'static &'static u8, (u8, u64, [i32; 4], (char, u64, *mut isize)));
let _16: (&'static [i32; 4], &'static &'static u8, (u8, u64, [i32; 4], (char, u64, *mut isize)));
let _17: f64;
let _18: *mut u128;
let _19: u16;
let _20: u16;
let _21: (i128,);
let _22: (([isize; 1], (char, f64), f32), ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), [u16; 6]);
let _23: isize;
let _24: Adt18;
let _25: f32;
let _26: u16;
let _27: i128;
let _28: [bool; 7];
let _29: &'static isize;
let _30: isize;
let _31: isize;
let _32: &'static i64;
let _33: f64;
let _34: [u32; 8];
let _35: ();
let _36: ();
{
_8 = [true,false,false];
_10.1.2.fld3 = _5.0 + _5.0;
_6.2 = _4;
_10.2.3.0 = [1_usize,4490301780352974816_usize];
_5.1 = 116_u8 as isize;
place!(Field::<i64>(Variant(_4, 1), 4)) = Field::<i64>(Variant(_3, 1), 4) - Field::<i64>(Variant(_5.2, 1), 4);
_11.2 = Adt20::Variant1 { fld0: _7,fld1: '\u{37787}',fld2: 47657328297715166167984645660659209195_u128,fld3: _5.0,fld4: Field::<u64>(Variant(_5.2, 1), 0),fld5: (-1356993178_i32),fld6: 2458943656_u32 };
_10.1.2.fld6.0 = !_6.0;
_10.1.2.fld6 = _6;
_11.3.fld5 = (14136949920544378657408912668466018052_i128,);
place!(Field::<[isize; 1]>(Variant(_1, 1), 3)) = [_5.1];
_11.3.fld7 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_3, 1), 2)));
_11.3.fld1 = ('\u{b9806}', Field::<u64>(Variant(_5.2, 1), 0), Move(_11.3.fld7));
place!(Field::<u64>(Variant(_10.1.2.fld6.2, 1), 0)) = Field::<u64>(Variant(_3, 1), 0) + _11.3.fld1.1;
Call(place!(Field::<f64>(Variant(_1, 1), 1)) = fn14(_7, _4, _5, _3, Field::<f64>(Variant(_10.1.2.fld6.2, 1), 1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.1.2.fld5 = (_11.3.fld5.0,);
_12.1 = (_10.1.2.fld6.0, Field::<isize>(Variant(_6.2, 1), 2), _4);
_11.3.fld4 = [_6.0,_5.0,Field::<i8>(Variant(_11.2, 1), 3),_5.0];
_5 = (_10.1.2.fld6.0, Field::<isize>(Variant(_1, 1), 2), _4);
_11.3.fld7 = core::ptr::addr_of_mut!(_10.1.1.1);
SetDiscriminant(_4, 1);
Goto(bb2)
}
bb2 = {
_10.3 = Move(_2);
_11.1.0 = [Field::<u64>(Variant(_1, 1), 0)];
_11.1.2.fld7 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_4, 1), 2)));
_16.2.3 = (_11.3.fld1.0, Field::<u64>(Variant(_6.2, 1), 0), Move(_11.3.fld1.2));
_12.2.fld1.2 = core::ptr::addr_of_mut!(_10.1.1.1);
Call(_6.0 = core::intrinsics::bswap(_10.1.2.fld3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _1;
SetDiscriminant(_12.1.2, 2);
_12.2.fld4 = [_10.1.2.fld6.0,_10.1.2.fld3,_6.0,_10.1.2.fld6.0];
_11.3.fld6.2 = Adt18::Variant1 { fld0: Field::<u64>(Variant(_4, 1), 0),fld1: Field::<f64>(Variant(_6.2, 1), 1),fld2: _10.1.2.fld6.1,fld3: Field::<[isize; 1]>(Variant(_3, 1), 3),fld4: Field::<i64>(Variant(_6.2, 1), 4),fld5: Field::<f32>(Variant(_3, 1), 5) };
_10.1.2.fld1.1 = Field::<u64>(Variant(_3, 1), 0);
_12.2.fld1.1 = Field::<u64>(Variant(_1, 1), 0);
place!(Field::<u64>(Variant(_10.1.2.fld6.2, 1), 0)) = !Field::<u64>(Variant(_5.2, 1), 0);
place!(Field::<char>(Variant(_11.2, 1), 1)) = _16.2.3.0;
RET = Adt20::Variant1 { fld0: _7,fld1: _16.2.3.0,fld2: 82095233611743315921825085159775981956_u128,fld3: _10.1.2.fld6.0,fld4: Field::<u64>(Variant(_11.2, 1), 4),fld5: 433696178_i32,fld6: 3071213612_u32 };
place!(Field::<i32>(Variant(RET, 1), 5)) = 1710174565_i32;
_10.1.2.fld6 = (_6.0, Field::<isize>(Variant(_5.2, 1), 2), _1);
_10.1.2.fld1.0 = Field::<char>(Variant(_11.2, 1), 1);
_12.2.fld6.1 = _6.1 + _6.1;
_5.2 = _11.3.fld6.2;
_10.2.1 = _12.1.0 << Field::<isize>(Variant(_3, 1), 2);
_17 = Field::<f64>(Variant(_1, 1), 1);
_10.0 = &_11.1.2.fld6.1;
RET = Adt20::Variant1 { fld0: _7,fld1: Field::<char>(Variant(_11.2, 1), 1),fld2: 27841685020810994978056157517551140672_u128,fld3: _12.1.0,fld4: Field::<u64>(Variant(_11.3.fld6.2, 1), 0),fld5: 1644697534_i32,fld6: 3482298753_u32 };
_12.2.fld5 = (_11.3.fld5.0,);
_11.1.2.fld0 = 2_usize * 3343637749165000820_usize;
_19 = !25462_u16;
_12.0 = _11.1.0;
Call(place!(Field::<isize>(Variant(_10.1.2.fld6.2, 1), 2)) = core::intrinsics::bswap(Field::<isize>(Variant(_6.2, 1), 2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
place!(Field::<u64>(Variant(_5.2, 1), 0)) = _11.3.fld1.1;
_11.1.1 = _6;
_16.2.1 = Field::<u64>(Variant(_11.2, 1), 4);
place!(Field::<[isize; 1]>(Variant(_4, 1), 3)) = [_12.1.1];
place!(Field::<f64>(Variant(_4, 1), 1)) = -_17;
_14.0 = _11.3.fld5.0 as i8;
_22.0.1 = (_10.1.2.fld1.0, Field::<f64>(Variant(_1, 1), 1));
_11.3.fld6.0 = Field::<isize>(Variant(_1, 1), 2) as i8;
_15.2.0 = 780304469_i32 as u8;
_12.1.2 = Adt18::Variant3 { fld0: true };
SetDiscriminant(_1, 1);
_22.1.1 = (_11.1.1.0, Field::<isize>(Variant(_3, 1), 2), _3);
SetDiscriminant(_3, 2);
_12.1.2 = Adt18::Variant0 { fld0: _15.2.0,fld1: _11.3.fld5.0,fld2: _19 };
_11.1.2.fld1.0 = Field::<char>(Variant(RET, 1), 1);
_15.2.3.0 = _11.3.fld1.0;
_16.0 = &_15.2.2;
_10.1.1.2 = Adt18::Variant2 { fld0: false,fld1: 69537183811161509767237382251947194528_u128 };
_16.2.3.0 = _15.2.3.0;
SetDiscriminant(_10.1.2.fld6.2, 1);
Goto(bb5)
}
bb5 = {
_22.1.2.fld6.2 = Adt18::Variant1 { fld0: _12.2.fld1.1,fld1: Field::<f64>(Variant(_11.2, 1), 0),fld2: _10.1.2.fld6.1,fld3: Field::<[isize; 1]>(Variant(_4, 1), 3),fld4: Field::<i64>(Variant(_22.1.1.2, 1), 4),fld5: Field::<f32>(Variant(_11.1.1.2, 1), 5) };
SetDiscriminant(_5.2, 0);
place!(Field::<f32>(Variant(_22.1.1.2, 1), 5)) = Field::<f32>(Variant(_11.3.fld6.2, 1), 5);
match Field::<i128>(Variant(_12.1.2, 0), 1) {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
14136949920544378657408912668466018052 => bb13,
_ => bb12
}
}
bb6 = {
place!(Field::<u64>(Variant(_5.2, 1), 0)) = _11.3.fld1.1;
_11.1.1 = _6;
_16.2.1 = Field::<u64>(Variant(_11.2, 1), 4);
place!(Field::<[isize; 1]>(Variant(_4, 1), 3)) = [_12.1.1];
place!(Field::<f64>(Variant(_4, 1), 1)) = -_17;
_14.0 = _11.3.fld5.0 as i8;
_22.0.1 = (_10.1.2.fld1.0, Field::<f64>(Variant(_1, 1), 1));
_11.3.fld6.0 = Field::<isize>(Variant(_1, 1), 2) as i8;
_15.2.0 = 780304469_i32 as u8;
_12.1.2 = Adt18::Variant3 { fld0: true };
SetDiscriminant(_1, 1);
_22.1.1 = (_11.1.1.0, Field::<isize>(Variant(_3, 1), 2), _3);
SetDiscriminant(_3, 2);
_12.1.2 = Adt18::Variant0 { fld0: _15.2.0,fld1: _11.3.fld5.0,fld2: _19 };
_11.1.2.fld1.0 = Field::<char>(Variant(RET, 1), 1);
_15.2.3.0 = _11.3.fld1.0;
_16.0 = &_15.2.2;
_10.1.1.2 = Adt18::Variant2 { fld0: false,fld1: 69537183811161509767237382251947194528_u128 };
_16.2.3.0 = _15.2.3.0;
SetDiscriminant(_10.1.2.fld6.2, 1);
Goto(bb5)
}
bb7 = {
_4 = _1;
SetDiscriminant(_12.1.2, 2);
_12.2.fld4 = [_10.1.2.fld6.0,_10.1.2.fld3,_6.0,_10.1.2.fld6.0];
_11.3.fld6.2 = Adt18::Variant1 { fld0: Field::<u64>(Variant(_4, 1), 0),fld1: Field::<f64>(Variant(_6.2, 1), 1),fld2: _10.1.2.fld6.1,fld3: Field::<[isize; 1]>(Variant(_3, 1), 3),fld4: Field::<i64>(Variant(_6.2, 1), 4),fld5: Field::<f32>(Variant(_3, 1), 5) };
_10.1.2.fld1.1 = Field::<u64>(Variant(_3, 1), 0);
_12.2.fld1.1 = Field::<u64>(Variant(_1, 1), 0);
place!(Field::<u64>(Variant(_10.1.2.fld6.2, 1), 0)) = !Field::<u64>(Variant(_5.2, 1), 0);
place!(Field::<char>(Variant(_11.2, 1), 1)) = _16.2.3.0;
RET = Adt20::Variant1 { fld0: _7,fld1: _16.2.3.0,fld2: 82095233611743315921825085159775981956_u128,fld3: _10.1.2.fld6.0,fld4: Field::<u64>(Variant(_11.2, 1), 4),fld5: 433696178_i32,fld6: 3071213612_u32 };
place!(Field::<i32>(Variant(RET, 1), 5)) = 1710174565_i32;
_10.1.2.fld6 = (_6.0, Field::<isize>(Variant(_5.2, 1), 2), _1);
_10.1.2.fld1.0 = Field::<char>(Variant(_11.2, 1), 1);
_12.2.fld6.1 = _6.1 + _6.1;
_5.2 = _11.3.fld6.2;
_10.2.1 = _12.1.0 << Field::<isize>(Variant(_3, 1), 2);
_17 = Field::<f64>(Variant(_1, 1), 1);
_10.0 = &_11.1.2.fld6.1;
RET = Adt20::Variant1 { fld0: _7,fld1: Field::<char>(Variant(_11.2, 1), 1),fld2: 27841685020810994978056157517551140672_u128,fld3: _12.1.0,fld4: Field::<u64>(Variant(_11.3.fld6.2, 1), 0),fld5: 1644697534_i32,fld6: 3482298753_u32 };
_12.2.fld5 = (_11.3.fld5.0,);
_11.1.2.fld0 = 2_usize * 3343637749165000820_usize;
_19 = !25462_u16;
_12.0 = _11.1.0;
Call(place!(Field::<isize>(Variant(_10.1.2.fld6.2, 1), 2)) = core::intrinsics::bswap(Field::<isize>(Variant(_6.2, 1), 2)), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_10.3 = Move(_2);
_11.1.0 = [Field::<u64>(Variant(_1, 1), 0)];
_11.1.2.fld7 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_4, 1), 2)));
_16.2.3 = (_11.3.fld1.0, Field::<u64>(Variant(_6.2, 1), 0), Move(_11.3.fld1.2));
_12.2.fld1.2 = core::ptr::addr_of_mut!(_10.1.1.1);
Call(_6.0 = core::intrinsics::bswap(_10.1.2.fld3), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_10.1.2.fld5 = (_11.3.fld5.0,);
_12.1 = (_10.1.2.fld6.0, Field::<isize>(Variant(_6.2, 1), 2), _4);
_11.3.fld4 = [_6.0,_5.0,Field::<i8>(Variant(_11.2, 1), 3),_5.0];
_5 = (_10.1.2.fld6.0, Field::<isize>(Variant(_1, 1), 2), _4);
_11.3.fld7 = core::ptr::addr_of_mut!(_10.1.1.1);
SetDiscriminant(_4, 1);
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
_11.1.2.fld6 = (_11.3.fld6.0, Field::<isize>(Variant(_4, 1), 2), _11.1.1.2);
place!(Field::<f64>(Variant(_22.1.1.2, 1), 1)) = -Field::<f64>(Variant(_6.2, 1), 1);
_22.1.2.fld1.1 = _16.2.1 * Field::<u64>(Variant(_11.1.2.fld6.2, 1), 0);
_11.1.2.fld1.2 = Move(_11.1.2.fld7);
_11.2 = Adt20::Variant1 { fld0: Field::<f64>(Variant(_4, 1), 1),fld1: _11.1.2.fld1.0,fld2: 166573143366914992140834607581467483433_u128,fld3: _22.1.1.0,fld4: _12.2.fld1.1,fld5: 1959465989_i32,fld6: 3756852867_u32 };
_6.0 = _11.1.2.fld6.0;
place!(Field::<f64>(Variant(_22.1.2.fld6.2, 1), 1)) = (-1246_i16) as f64;
place!(Field::<f32>(Variant(_22.1.2.fld6.2, 1), 5)) = Field::<isize>(Variant(_22.1.2.fld6.2, 1), 2) as f32;
place!(Field::<[isize; 1]>(Variant(_22.1.2.fld6.2, 1), 3)) = [_12.1.1];
_15.2.0 = (-1863491267_i32) as u8;
_12.1.2 = Adt18::Variant3 { fld0: false };
_5 = (_10.1.2.fld3, Field::<isize>(Variant(_11.1.1.2, 1), 2), _4);
place!(Field::<u128>(Variant(_10.1.1.2, 2), 1)) = 80055319183728582902560035941321346022_u128;
_12.2.fld5 = (_11.3.fld5.0,);
_10.1.1 = (_12.1.0, Field::<isize>(Variant(_5.2, 1), 2), _4);
match _11.3.fld5.0 {
0 => bb10,
14136949920544378657408912668466018052 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_22.1.2.fld6.2 = _11.1.1.2;
_11.1.1.1 = Field::<isize>(Variant(_22.1.2.fld6.2, 1), 2);
place!(Field::<bool>(Variant(_3, 2), 0)) = Field::<f64>(Variant(_5.2, 1), 1) <= _22.0.1.1;
_10.1.2.fld4 = _11.3.fld4;
_16.2.3.0 = _11.3.fld1.0;
_22.0.0 = [Field::<isize>(Variant(_5.2, 1), 2)];
place!(Field::<[isize; 1]>(Variant(_6.2, 1), 3)) = Field::<[isize; 1]>(Variant(_11.3.fld6.2, 1), 3);
_11.1.1.1 = -_22.1.1.1;
match _11.3.fld5.0 {
14136949920544378657408912668466018052 => bb17,
_ => bb16
}
}
bb16 = {
Return()
}
bb17 = {
_11.1.1.1 = 25322_i16 as isize;
place!(Field::<bool>(Variant(_12.1.2, 3), 0)) = Field::<bool>(Variant(_3, 2), 0);
_10.1.2.fld3 = _11.1.2.fld0 as i8;
_10.1.2.fld6.2 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_3, 2), 0) };
_8 = [Field::<bool>(Variant(_3, 2), 0),Field::<bool>(Variant(_10.1.2.fld6.2, 3), 0),Field::<bool>(Variant(_10.1.2.fld6.2, 3), 0)];
_5.1 = _19 as isize;
_10.0 = &_23;
_18 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_11.2, 1), 2)));
_20 = !_19;
_8 = [Field::<bool>(Variant(_3, 2), 0),Field::<bool>(Variant(_12.1.2, 3), 0),Field::<bool>(Variant(_3, 2), 0)];
place!(Field::<u128>(Variant(_11.2, 1), 2)) = 158186856023052464941447787271662489873_u128;
_11.0 = (-18897_i16);
_11.3.fld6.0 = Field::<f64>(Variant(_22.1.2.fld6.2, 1), 1) as i8;
place!(Field::<i64>(Variant(_10.1.1.2, 1), 4)) = Field::<i64>(Variant(_22.1.1.2, 1), 4);
place!(Field::<u32>(Variant(RET, 1), 6)) = !3533582467_u32;
_6.1 = -Field::<isize>(Variant(_4, 1), 2);
place!(Field::<i32>(Variant(RET, 1), 5)) = (-1775112438_i32) & 1688067213_i32;
_5.0 = _11.3.fld6.0 - _11.3.fld6.0;
_11.0 = (-10587_i16) & 14456_i16;
_20 = _19;
_11.3.fld2 = [Field::<isize>(Variant(_22.1.2.fld6.2, 1), 2)];
SetDiscriminant(_22.1.1.2, 2);
_15.2.3 = Move(_16.2.3);
match _10.1.2.fld5.0 {
0 => bb12,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb13,
5 => bb9,
14136949920544378657408912668466018052 => bb18,
_ => bb7
}
}
bb18 = {
place!(Field::<u128>(Variant(RET, 1), 2)) = (*_18);
_21 = (_10.1.2.fld5.0,);
_11.1.2.fld7 = Move(_15.2.3.2);
Goto(bb19)
}
bb19 = {
Call(_35 = dump_var(13_usize, 21_usize, Move(_21), 20_usize, Move(_20), 36_usize, _36, 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: f64,mut _2: Adt18,mut _3: (i8, isize, Adt18),mut _4: Adt18,mut _5: f64) -> f64 {
mir! {
type RET = f64;
let _6: f32;
let _7: ([isize; 1], (char, f64), f32);
let _8: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31);
let _9: u128;
let _10: ();
let _11: ();
{
place!(Field::<f64>(Variant(_2, 1), 1)) = Field::<i64>(Variant(_4, 1), 4) as f64;
_5 = Field::<f64>(Variant(_4, 1), 1) + _1;
_6 = -Field::<f32>(Variant(_2, 1), 5);
RET = _5;
_6 = Field::<isize>(Variant(_4, 1), 2) as f32;
place!(Field::<i64>(Variant(_2, 1), 4)) = Field::<i64>(Variant(_4, 1), 4);
SetDiscriminant(_2, 0);
place!(Field::<f64>(Variant(_4, 1), 1)) = _3.0 as f64;
place!(Field::<f32>(Variant(_4, 1), 5)) = Field::<f32>(Variant(_3.2, 1), 5);
RET = _1 - _1;
place!(Field::<[isize; 1]>(Variant(_4, 1), 3)) = [Field::<isize>(Variant(_4, 1), 2)];
SetDiscriminant(_4, 1);
_8.1.2.fld0 = 189_u8 as usize;
place!(Field::<f32>(Variant(_4, 1), 5)) = _6;
_8.1.2.fld6.0 = _3.0;
_8.1.2.fld7 = core::ptr::addr_of_mut!(_8.3.fld6.1);
_8.3.fld3 = _3.0;
_8.1.2.fld6.2 = Adt18::Variant0 { fld0: 57_u8,fld1: 74446534873571777252938372206939327705_i128,fld2: 64097_u16 };
_8.1.2.fld7 = core::ptr::addr_of_mut!(_8.1.1.1);
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i8,mut _2: Adt18,mut _3: (i8, isize, Adt18),mut _4: isize,mut _5: [isize; 1],mut _6: f32,mut _7: isize,mut _8: (i8, isize, Adt18),mut _9: f64) -> i8 {
mir! {
type RET = i8;
let _10: char;
let _11: [usize; 1];
let _12: [usize; 7];
let _13: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _14: u32;
let _15: (i8, isize, Adt18);
let _16: *const u16;
let _17: i32;
let _18: &'static u8;
let _19: f64;
let _20: [usize; 1];
let _21: f64;
let _22: u64;
let _23: (u8, u64, [i32; 4], (char, u64, *mut isize));
let _24: [char; 1];
let _25: f32;
let _26: isize;
let _27: (u8, i8, [bool; 3], ([usize; 2],));
let _28: u64;
let _29: &'static (&'static &'static u8, usize);
let _30: f64;
let _31: ();
let _32: ();
{
place!(Field::<f32>(Variant(_3.2, 1), 5)) = 3411184624_u32 as f32;
_8.1 = Field::<isize>(Variant(_8.2, 1), 2);
_3 = (_8.0, _7, _8.2);
_8 = (_1, _4, _3.2);
_3.2 = Adt18::Variant3 { fld0: true };
_3.1 = _4;
place!(Field::<bool>(Variant(_3.2, 3), 0)) = false;
RET = _1;
_7 = Field::<isize>(Variant(_2, 1), 2);
RET = Field::<u64>(Variant(_8.2, 1), 0) as i8;
RET = !_8.0;
_7 = 140_u8 as isize;
SetDiscriminant(_8.2, 3);
_10 = '\u{a2b43}';
RET = _8.0;
_2 = _3.2;
_15.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_3.2, 3), 0),fld1: 120390575283663103641117143469415069475_u128 };
SetDiscriminant(_3.2, 3);
_15.1 = _4;
Goto(bb1)
}
bb1 = {
_8.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_15.2, 2), 0),fld1: 13075009150954244367384895314197272168_u128 };
place!(Field::<u128>(Variant(_15.2, 2), 1)) = 312460389355031113035172892553739961364_u128;
place!(Field::<u128>(Variant(_8.2, 2), 1)) = Field::<u128>(Variant(_15.2, 2), 1) >> _8.0;
place!(Field::<u128>(Variant(_15.2, 2), 1)) = 224_u8 as u128;
_17 = 1021492475_i32 - (-1668794442_i32);
_7 = -_4;
_3 = (RET, _4, _8.2);
_19 = _8.0 as f64;
_20 = [6828956495404813545_usize];
_8 = (_1, _7, _2);
_12 = [2_usize,9432759981034399824_usize,0_usize,174638285710343225_usize,2_usize,1_usize,6384767859449276097_usize];
_8.1 = _3.1;
_3.2 = Adt18::Variant1 { fld0: 5252826582414425409_u64,fld1: _9,fld2: _3.1,fld3: _5,fld4: (-815102732609790010_i64),fld5: _6 };
_7 = _15.1;
_7 = 41264_u16 as isize;
place!(Field::<f64>(Variant(_3.2, 1), 1)) = -_9;
_9 = -Field::<f64>(Variant(_3.2, 1), 1);
_23.1 = 10599938788735799569_u64;
_15 = (RET, _3.1, _8.2);
_17 = (-757819608_i32) ^ 844532988_i32;
_3 = _15;
_10 = '\u{6708c}';
match _23.1 {
10599938788735799569 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
place!(Field::<bool>(Variant(_3.2, 3), 0)) = _4 == _8.1;
_18 = &_23.0;
_15.1 = 100173487466846196665316665985242830885_i128 as isize;
_6 = 22504_u16 as f32;
_23.2 = [_17,_17,_17,_17];
_23.0 = 218_u8 - 78_u8;
_15.0 = _17 as i8;
SetDiscriminant(_2, 2);
_24 = [_10];
_10 = '\u{1091ea}';
_7 = _3.0 as isize;
_23.1 = !10914119027252381504_u64;
_2 = _3.2;
_18 = &_23.0;
_15.1 = !_4;
_6 = (*_18) as f32;
_15.0 = -_1;
_10 = '\u{7aca1}';
_23.3.1 = _6 as u64;
Call(RET = core::intrinsics::bswap(_15.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_21 = _17 as f64;
place!(Field::<bool>(Variant(_8.2, 3), 0)) = Field::<bool>(Variant(_3.2, 3), 0);
_27.1 = !_15.0;
_11 = _20;
SetDiscriminant(_8.2, 1);
place!(Field::<i64>(Variant(_8.2, 1), 4)) = !4373812524512756705_i64;
SetDiscriminant(_3.2, 2);
_27.1 = _3.0 << _8.1;
_15.0 = _8.0;
_14 = (*_18) as u32;
_8.1 = _10 as isize;
_15.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_2, 3), 0),fld1: 86297666128021082913807899132428931152_u128 };
_11 = [769147380901385992_usize];
SetDiscriminant(_2, 0);
place!(Field::<f32>(Variant(_8.2, 1), 5)) = -_6;
_22 = _23.3.1 << _3.1;
Goto(bb5)
}
bb5 = {
place!(Field::<[isize; 1]>(Variant(_8.2, 1), 3)) = [_8.1];
RET = -_8.0;
place!(Field::<bool>(Variant(_15.2, 2), 0)) = !true;
_8.0 = _6 as i8;
_3.1 = -_4;
_25 = Field::<f32>(Variant(_8.2, 1), 5) * Field::<f32>(Variant(_8.2, 1), 5);
_27.0 = (*_18);
_26 = _15.1;
_30 = _19 + _9;
_18 = &_23.0;
_3.0 = _15.1 as i8;
_23.0 = !_27.0;
_23.3.0 = _10;
_25 = Field::<f32>(Variant(_8.2, 1), 5) + _6;
place!(Field::<u8>(Variant(_2, 0), 0)) = _27.0;
_3.0 = _27.1 & _27.1;
place!(Field::<u64>(Variant(_8.2, 1), 0)) = !_23.3.1;
_8.2 = Adt18::Variant1 { fld0: _23.3.1,fld1: _9,fld2: _26,fld3: _5,fld4: 3510821020247188878_i64,fld5: _6 };
RET = -_3.0;
_21 = _30 - _9;
_23.3.2 = core::ptr::addr_of_mut!(_7);
_1 = _3.0 >> _3.0;
_8.0 = _27.1 * _3.0;
Goto(bb6)
}
bb6 = {
Call(_31 = dump_var(15_usize, 4_usize, Move(_4), 7_usize, Move(_7), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_31 = dump_var(15_usize, 10_usize, Move(_10), 11_usize, Move(_11), 32_usize, _32, 32_usize, _32), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize) -> (i8, isize, Adt18) {
mir! {
type RET = (i8, isize, Adt18);
let _2: f32;
let _3: ([usize; 2],);
let _4: Adt62;
let _5: isize;
let _6: ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _7: ([u32; 8], (i8, isize, Adt18));
let _8: ([u32; 8], (i8, isize, Adt18));
let _9: [u64; 5];
let _10: [i8; 4];
let _11: &'static i8;
let _12: isize;
let _13: (Adt65, (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31), (i8, isize, Adt18));
let _14: *mut u128;
let _15: i128;
let _16: f32;
let _17: f32;
let _18: (char, f64);
let _19: &'static (&'static &'static u8, usize);
let _20: (char, u64, *mut isize);
let _21: u8;
let _22: ([usize; 7], *const usize, [i128; 4], [usize; 7]);
let _23: (i32, [u32; 6], [char; 1], Adt18);
let _24: [u32; 8];
let _25: &'static &'static [i32; 4];
let _26: u8;
let _27: [bool; 3];
let _28: char;
let _29: bool;
let _30: i8;
let _31: *const u16;
let _32: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _33: u8;
let _34: [i16; 5];
let _35: bool;
let _36: &'static isize;
let _37: u16;
let _38: &'static [u16; 1];
let _39: Adt77;
let _40: bool;
let _41: bool;
let _42: &'static i16;
let _43: &'static Adt20;
let _44: i128;
let _45: &'static &'static [i32; 4];
let _46: &'static [bool; 3];
let _47: ();
let _48: ();
{
RET.0 = (-109_i8);
RET.0 = 123_u8 as i8;
RET.2 = Adt18::Variant3 { fld0: false };
RET.2 = Adt18::Variant0 { fld0: 65_u8,fld1: (-147633398490789683840066098901885884236_i128),fld2: 23950_u16 };
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 60867_u16;
match Field::<u16>(Variant(RET.2, 0), 2) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
60867 => bb8,
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
place!(Field::<u8>(Variant(RET.2, 0), 0)) = !230_u8;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 77_u8 ^ 90_u8;
_1 = 9223372036854775807_isize;
RET.1 = RET.0 as isize;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 540104291_i32 as u16;
RET.1 = _1 & _1;
_2 = 188329059498192893762334789077895563995_u128 as f32;
RET.2 = Adt18::Variant0 { fld0: 173_u8,fld1: 119735389477493673090395919498355121728_i128,fld2: 12752_u16 };
RET.1 = !_1;
_3.0 = [2_usize,3_usize];
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 134993626246833999287629767418047496474_u128 as u16;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = (-24472115483202399426302432045973737311_i128);
place!(Field::<u8>(Variant(RET.2, 0), 0)) = RET.0 as u8;
RET.2 = Adt18::Variant0 { fld0: 220_u8,fld1: (-102836274138654447888895569946573362384_i128),fld2: 39948_u16 };
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 2404677191_u32 as u8;
RET.2 = Adt18::Variant3 { fld0: true };
place!(Field::<bool>(Variant(RET.2, 3), 0)) = _2 >= _2;
RET.0 = _1 as i8;
_4 = Adt62::Variant2 { fld0: 75_u8 };
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_4 = Adt62::Variant2 { fld0: 184_u8 };
_3.0 = [5_usize,0_usize];
place!(Field::<u8>(Variant(_4, 2), 0)) = 635572255_u32 as u8;
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_3.0 = [15673598233241280428_usize,4_usize];
_5 = 16714194115849285395_u64 as isize;
_4 = Adt62::Variant1 { fld0: 11925068795794094259_u64 };
RET.1 = _5 ^ _5;
Goto(bb9)
}
bb9 = {
RET.1 = _5 - _5;
_3.0 = [4724511158903366680_usize,9224826286015239490_usize];
_5 = _1 << RET.1;
place!(Field::<u64>(Variant(_4, 1), 0)) = 17126023587901144661_u64 ^ 14935932822470546557_u64;
place!(Field::<bool>(Variant(RET.2, 3), 0)) = false;
Goto(bb10)
}
bb10 = {
RET.1 = -_1;
_4 = Adt62::Variant1 { fld0: 2552396442992788199_u64 };
RET.1 = _5 + _5;
_1 = RET.1;
RET.1 = _1 * _1;
_2 = 3091_u16 as f32;
SetDiscriminant(RET.2, 3);
RET.2 = Adt18::Variant0 { fld0: 94_u8,fld1: (-37383962134042517016409099551814818061_i128),fld2: 35260_u16 };
_6.2.fld5 = ((-24550168765933830716540669367483899674_i128),);
_6.2.fld1.2 = core::ptr::addr_of_mut!(_5);
_6.2.fld1.1 = 7334148488667129403_u64;
place!(Field::<u64>(Variant(_4, 1), 0)) = _6.2.fld1.1;
RET.2 = Adt18::Variant0 { fld0: 25_u8,fld1: _6.2.fld5.0,fld2: 38949_u16 };
_6.2.fld1.2 = core::ptr::addr_of_mut!(_6.1.1);
Goto(bb11)
}
bb11 = {
SetDiscriminant(_4, 2);
place!(Field::<u16>(Variant(RET.2, 0), 2)) = RET.0 as u16;
_7.0 = [2443437278_u32,248265526_u32,3460074177_u32,1430587602_u32,510923467_u32,3635765643_u32,2446083425_u32,3586942389_u32];
_6.2.fld1.0 = '\u{f0b86}';
_6.2.fld0 = 2_usize | 4_usize;
_6.2.fld3 = Field::<u16>(Variant(RET.2, 0), 2) as i8;
_6.0 = [_6.2.fld1.1];
RET.0 = _6.2.fld3 & _6.2.fld3;
_7.1.0 = !RET.0;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 17836_u16;
place!(Field::<u8>(Variant(_4, 2), 0)) = 33_u8;
_6.2.fld6.0 = !_7.1.0;
match _6.2.fld1.1 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
7334148488667129403 => bb17,
_ => bb16
}
}
bb12 = {
RET.1 = -_1;
_4 = Adt62::Variant1 { fld0: 2552396442992788199_u64 };
RET.1 = _5 + _5;
_1 = RET.1;
RET.1 = _1 * _1;
_2 = 3091_u16 as f32;
SetDiscriminant(RET.2, 3);
RET.2 = Adt18::Variant0 { fld0: 94_u8,fld1: (-37383962134042517016409099551814818061_i128),fld2: 35260_u16 };
_6.2.fld5 = ((-24550168765933830716540669367483899674_i128),);
_6.2.fld1.2 = core::ptr::addr_of_mut!(_5);
_6.2.fld1.1 = 7334148488667129403_u64;
place!(Field::<u64>(Variant(_4, 1), 0)) = _6.2.fld1.1;
RET.2 = Adt18::Variant0 { fld0: 25_u8,fld1: _6.2.fld5.0,fld2: 38949_u16 };
_6.2.fld1.2 = core::ptr::addr_of_mut!(_6.1.1);
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<u8>(Variant(RET.2, 0), 0)) = !230_u8;
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 77_u8 ^ 90_u8;
_1 = 9223372036854775807_isize;
RET.1 = RET.0 as isize;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 540104291_i32 as u16;
RET.1 = _1 & _1;
_2 = 188329059498192893762334789077895563995_u128 as f32;
RET.2 = Adt18::Variant0 { fld0: 173_u8,fld1: 119735389477493673090395919498355121728_i128,fld2: 12752_u16 };
RET.1 = !_1;
_3.0 = [2_usize,3_usize];
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 134993626246833999287629767418047496474_u128 as u16;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = (-24472115483202399426302432045973737311_i128);
place!(Field::<u8>(Variant(RET.2, 0), 0)) = RET.0 as u8;
RET.2 = Adt18::Variant0 { fld0: 220_u8,fld1: (-102836274138654447888895569946573362384_i128),fld2: 39948_u16 };
place!(Field::<u8>(Variant(RET.2, 0), 0)) = 2404677191_u32 as u8;
RET.2 = Adt18::Variant3 { fld0: true };
place!(Field::<bool>(Variant(RET.2, 3), 0)) = _2 >= _2;
RET.0 = _1 as i8;
_4 = Adt62::Variant2 { fld0: 75_u8 };
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_4 = Adt62::Variant2 { fld0: 184_u8 };
_3.0 = [5_usize,0_usize];
place!(Field::<u8>(Variant(_4, 2), 0)) = 635572255_u32 as u8;
place!(Field::<bool>(Variant(RET.2, 3), 0)) = true;
_3.0 = [15673598233241280428_usize,4_usize];
_5 = 16714194115849285395_u64 as isize;
_4 = Adt62::Variant1 { fld0: 11925068795794094259_u64 };
RET.1 = _5 ^ _5;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
place!(Field::<u16>(Variant(RET.2, 0), 2)) = _7.1.0 as u16;
place!(Field::<u8>(Variant(_4, 2), 0)) = !58_u8;
RET.2 = Adt18::Variant2 { fld0: false,fld1: 267453142263407663253995374414226897094_u128 };
Call(_6.2.fld1.2 = fn17(_5, _6.2.fld5.0, _6.2.fld5.0, _1, _1, _2, _1, RET.0, RET.1, _6.2.fld1.1, _7.0), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_13.1.1.1.0 = 6094010245324027870_i64 as i8;
_6.2.fld7 = Move(_6.2.fld1.2);
_13.2.0 = !_6.2.fld3;
_9 = [_6.2.fld1.1,_6.2.fld1.1,_6.2.fld1.1,_6.2.fld1.1,_6.2.fld1.1];
place!(Field::<u128>(Variant(RET.2, 2), 1)) = !329687056600291145009975906030643199017_u128;
_6.2.fld1.2 = Move(_6.2.fld7);
_7.0 = [760089702_u32,3400039666_u32,1402727143_u32,3896118576_u32,36137924_u32,2406693419_u32,95746348_u32,1550595818_u32];
_6.2.fld7 = Move(_6.2.fld1.2);
_13.0.fld4 = [(-2976840861919100033_i64),(-8585953813806931660_i64),5399543213904890211_i64,(-9211820837476670024_i64),3253967551173591571_i64,(-7688850223067641911_i64)];
_11 = &_8.1.0;
_6.2.fld3 = _13.2.0;
_6.2.fld3 = (-1018828389_i32) as i8;
_10 = [_7.1.0,_13.2.0,_13.2.0,_6.2.fld6.0];
place!(Field::<u128>(Variant(RET.2, 2), 1)) = !82233481421339386773850789556372641752_u128;
_13.1.3.fld4 = [RET.0,RET.0,_6.2.fld6.0,_13.1.1.1.0];
_5 = !RET.1;
_7.1.0 = _6.2.fld0 as i8;
_10 = [RET.0,_6.2.fld3,_6.2.fld6.0,_13.1.1.1.0];
_12 = RET.1;
_5 = !RET.1;
Goto(bb19)
}
bb19 = {
_6.2.fld1.1 = 3442056548902183568_i64 as u64;
_13.1.1.2.fld1.0 = _6.2.fld1.0;
_13.1.3.fld1.1 = _6.2.fld1.1;
_15 = _6.2.fld5.0 ^ _6.2.fld5.0;
_8.0 = _7.0;
_13.1.0 = (-13622_i16) * (-30431_i16);
_14 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(RET.2, 2), 1)));
_11 = &_13.1.3.fld3;
_13.1.1.2.fld3 = _13.1.1.1.0 ^ _13.1.1.1.0;
_13.1.3.fld6.1 = _5;
Goto(bb20)
}
bb20 = {
_13.1.1.2.fld6.0 = -_7.1.0;
_10 = [_13.1.1.2.fld3,_13.2.0,_13.1.1.2.fld3,_6.2.fld6.0];
_13.1.3.fld6.0 = _6.2.fld6.0 & _13.2.0;
Goto(bb21)
}
bb21 = {
_13.1.3.fld5.0 = -_15;
_6.2.fld6.0 = !_13.1.1.2.fld6.0;
_6.1.0 = -_13.1.3.fld6.0;
_6.1.2 = Adt18::Variant0 { fld0: Field::<u8>(Variant(_4, 2), 0),fld1: _13.1.3.fld5.0,fld2: 42300_u16 };
RET.2 = Adt18::Variant0 { fld0: Field::<u8>(Variant(_6.1.2, 0), 0),fld1: _15,fld2: 32852_u16 };
_13.1.1.2.fld6.2 = Adt18::Variant3 { fld0: true };
_13.1.3.fld4 = [_13.1.1.1.0,_6.2.fld6.0,_13.1.1.2.fld3,_13.1.1.2.fld6.0];
_13.2.1 = _12;
_13.0.fld3 = 2518962362_u32;
_13.1.1.2.fld6.2 = Adt18::Variant2 { fld0: false,fld1: 100356333660971407403605516775345233915_u128 };
_13.1.1.1.1 = !_1;
_13.1.1.0 = _6.0;
place!(Field::<i128>(Variant(RET.2, 0), 1)) = Field::<i128>(Variant(_6.1.2, 0), 1) * Field::<i128>(Variant(_6.1.2, 0), 1);
_13.1.3.fld4 = [_13.1.3.fld6.0,_13.1.3.fld6.0,RET.0,_7.1.0];
_13.1.1.1.1 = 2984841973370163659_i64 as isize;
Call(_13.1.3.fld0 = core::intrinsics::transmute(RET.1), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_13.1.3.fld3 = _6.2.fld6.0 << _5;
_13.1.3.fld0 = _6.2.fld0;
_13.1.1.2.fld4 = [_13.1.1.2.fld3,_13.1.3.fld3,_13.1.3.fld3,_13.1.3.fld3];
_7.1.2 = Adt18::Variant0 { fld0: Field::<u8>(Variant(RET.2, 0), 0),fld1: _15,fld2: 342_u16 };
_20 = (_6.2.fld1.0, _13.1.3.fld1.1, Move(_6.2.fld7));
place!(Field::<i128>(Variant(RET.2, 0), 1)) = !_13.1.3.fld5.0;
_13.1.1.2.fld1 = (_20.0, _13.1.3.fld1.1, Move(_20.2));
Goto(bb23)
}
bb23 = {
_13.0.fld6 = [_6.2.fld0,_13.1.3.fld0,_13.1.3.fld0,_6.2.fld0,_13.1.3.fld0,_13.1.3.fld0,_6.2.fld0];
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 956795822_i32 as u16;
_13.2.1 = !_1;
_13.0.fld0 = _13.2.1 != _1;
_18.1 = Field::<u8>(Variant(RET.2, 0), 0) as f64;
SetDiscriminant(RET.2, 1);
_13.1.3.fld1.0 = _20.0;
_13.1.1.1.0 = !_13.1.1.2.fld3;
SetDiscriminant(_4, 2);
_8.1.2 = Adt18::Variant3 { fld0: _13.0.fld0 };
RET.1 = Field::<u8>(Variant(_7.1.2, 0), 0) as isize;
RET.0 = _13.1.3.fld3;
_16 = -_2;
_23.0 = -(-655795983_i32);
_6.2.fld6.2 = Adt18::Variant3 { fld0: _13.0.fld0 };
_13.1.1.2.fld2 = [_5];
_13.1.1.2.fld7 = core::ptr::addr_of_mut!(_6.1.1);
_2 = 159456177333245769308268505641600828988_u128 as f32;
_6.2.fld2 = _13.1.1.2.fld2;
_21 = Field::<u8>(Variant(_7.1.2, 0), 0) << _13.1.1.1.1;
place!(Field::<[isize; 1]>(Variant(RET.2, 1), 3)) = _6.2.fld2;
_13.1.1.2.fld0 = _13.1.3.fld0;
_6.0 = _13.1.1.0;
_6.2.fld1 = (_13.1.3.fld1.0, _13.1.1.2.fld1.1, Move(_13.1.1.2.fld1.2));
place!(Field::<bool>(Variant(_13.1.1.2.fld6.2, 2), 0)) = Field::<bool>(Variant(_8.1.2, 3), 0);
_13.1.3.fld1 = (_20.0, _13.1.1.2.fld1.1, Move(_6.2.fld1.2));
place!(Field::<isize>(Variant(RET.2, 1), 2)) = _13.1.3.fld6.1 ^ _13.2.1;
match _6.2.fld5.0 {
0 => bb19,
1 => bb16,
2 => bb20,
3 => bb4,
315732198155004632746833938064284311782 => bb24,
_ => bb5
}
}
bb24 = {
place!(Field::<i128>(Variant(_7.1.2, 0), 1)) = _15;
match _13.0.fld3 {
0 => bb8,
1 => bb9,
2 => bb17,
3 => bb18,
4 => bb25,
5 => bb26,
2518962362 => bb28,
_ => bb27
}
}
bb25 = {
Return()
}
bb26 = {
RET.1 = -_1;
_4 = Adt62::Variant1 { fld0: 2552396442992788199_u64 };
RET.1 = _5 + _5;
_1 = RET.1;
RET.1 = _1 * _1;
_2 = 3091_u16 as f32;
SetDiscriminant(RET.2, 3);
RET.2 = Adt18::Variant0 { fld0: 94_u8,fld1: (-37383962134042517016409099551814818061_i128),fld2: 35260_u16 };
_6.2.fld5 = ((-24550168765933830716540669367483899674_i128),);
_6.2.fld1.2 = core::ptr::addr_of_mut!(_5);
_6.2.fld1.1 = 7334148488667129403_u64;
place!(Field::<u64>(Variant(_4, 1), 0)) = _6.2.fld1.1;
RET.2 = Adt18::Variant0 { fld0: 25_u8,fld1: _6.2.fld5.0,fld2: 38949_u16 };
_6.2.fld1.2 = core::ptr::addr_of_mut!(_6.1.1);
Goto(bb11)
}
bb27 = {
SetDiscriminant(_4, 2);
place!(Field::<u16>(Variant(RET.2, 0), 2)) = RET.0 as u16;
_7.0 = [2443437278_u32,248265526_u32,3460074177_u32,1430587602_u32,510923467_u32,3635765643_u32,2446083425_u32,3586942389_u32];
_6.2.fld1.0 = '\u{f0b86}';
_6.2.fld0 = 2_usize | 4_usize;
_6.2.fld3 = Field::<u16>(Variant(RET.2, 0), 2) as i8;
_6.0 = [_6.2.fld1.1];
RET.0 = _6.2.fld3 & _6.2.fld3;
_7.1.0 = !RET.0;
place!(Field::<u16>(Variant(RET.2, 0), 2)) = 17836_u16;
place!(Field::<u8>(Variant(_4, 2), 0)) = 33_u8;
_6.2.fld6.0 = !_7.1.0;
match _6.2.fld1.1 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
7334148488667129403 => bb17,
_ => bb16
}
}
bb28 = {
place!(Field::<u64>(Variant(RET.2, 1), 0)) = _20.1 - _13.1.3.fld1.1;
RET.2 = Adt18::Variant0 { fld0: _21,fld1: Field::<i128>(Variant(_7.1.2, 0), 1),fld2: 55421_u16 };
_9 = [_13.1.3.fld1.1,_20.1,_13.1.1.2.fld1.1,_13.1.1.2.fld1.1,_13.1.3.fld1.1];
SetDiscriminant(_8.1.2, 3);
_13.1.1.2.fld2 = [_12];
SetDiscriminant(_6.2.fld6.2, 1);
_8.1.0 = -RET.0;
_13.1.1.1.1 = !_12;
place!(Field::<isize>(Variant(_6.2.fld6.2, 1), 2)) = _13.2.1;
_20 = (_6.2.fld1.0, _6.2.fld1.1, Move(_13.1.3.fld1.2));
_13.0.fld1 = !_20.1;
_27 = [_13.0.fld0,Field::<bool>(Variant(_13.1.1.2.fld6.2, 2), 0),_13.0.fld0];
RET.0 = _13.1.3.fld3 * _8.1.0;
_13.1.3.fld1.0 = _6.2.fld1.0;
_12 = _13.1.3.fld6.1 | _13.1.1.1.1;
_13.1.1.2.fld5 = (Field::<i128>(Variant(RET.2, 0), 1),);
_13.0.fld2.0 = 12352_u16 as i128;
_11 = &_8.1.0;
_32.1.3 = Adt20::Variant1 { fld0: _18.1,fld1: _13.1.1.2.fld1.0,fld2: 269211400561769568826335536878690835319_u128,fld3: _13.1.3.fld3,fld4: _20.1,fld5: _23.0,fld6: _13.0.fld3 };
_7.0 = [Field::<u32>(Variant(_32.1.3, 1), 6),Field::<u32>(Variant(_32.1.3, 1), 6),Field::<u32>(Variant(_32.1.3, 1), 6),_13.0.fld3,Field::<u32>(Variant(_32.1.3, 1), 6),_13.0.fld3,Field::<u32>(Variant(_32.1.3, 1), 6),Field::<u32>(Variant(_32.1.3, 1), 6)];
_32.1.2.fld5.0 = !_13.1.3.fld5.0;
_20.2 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(_6.2.fld6.2, 1), 2)));
Goto(bb29)
}
bb29 = {
_6.1.1 = -_12;
_28 = _13.1.1.2.fld1.0;
_13.1.1.3 = Adt20::Variant1 { fld0: Field::<f64>(Variant(_32.1.3, 1), 0),fld1: _6.2.fld1.0,fld2: 104828938979396098850406409138644345795_u128,fld3: Field::<i8>(Variant(_32.1.3, 1), 3),fld4: _13.1.3.fld1.1,fld5: Field::<i32>(Variant(_32.1.3, 1), 5),fld6: _13.0.fld3 };
_32.1.1.0 = -(*_11);
_13.1.3.fld6.0 = Field::<i32>(Variant(_32.1.3, 1), 5) as i8;
RET.0 = (*_11) * Field::<i8>(Variant(_13.1.1.3, 1), 3);
_7.1.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_13.1.1.2.fld6.2, 2), 0),fld1: 289871309121757043836417763968300456509_u128 };
_32.3 = core::ptr::addr_of!(_6);
_7.1.2 = Adt18::Variant2 { fld0: _13.0.fld0,fld1: 223304156694545948664421505770842706434_u128 };
_32.2.0 = Field::<u8>(Variant(RET.2, 0), 0) << _8.1.0;
_32.1.2.fld1.0 = _28;
Goto(bb30)
}
bb30 = {
place!(Field::<isize>(Variant(_6.2.fld6.2, 1), 2)) = _13.1.1.2.fld1.0 as isize;
_13.1.3.fld1 = (_20.0, Field::<u64>(Variant(_32.1.3, 1), 4), Move(_13.1.1.2.fld7));
_8.1.1 = _12 << _13.1.1.2.fld6.0;
_13.1.1.1.1 = _5;
_23.3 = Adt18::Variant1 { fld0: Field::<u64>(Variant(_32.1.3, 1), 4),fld1: Field::<f64>(Variant(_32.1.3, 1), 0),fld2: _6.1.1,fld3: _6.2.fld2,fld4: 7587442674625807797_i64,fld5: _2 };
_6.2.fld4 = [RET.0,RET.0,Field::<i8>(Variant(_32.1.3, 1), 3),_13.1.3.fld3];
_13.1.2 = Adt20::Variant1 { fld0: Field::<f64>(Variant(_32.1.3, 1), 0),fld1: _32.1.2.fld1.0,fld2: 17576514883174932464683314127932314361_u128,fld3: _8.1.0,fld4: _13.1.3.fld1.1,fld5: Field::<i32>(Variant(_13.1.1.3, 1), 5),fld6: Field::<u32>(Variant(_32.1.3, 1), 6) };
place!(Field::<u64>(Variant(_13.1.2, 1), 4)) = _13.1.1.2.fld1.1;
_26 = _13.1.0 as u8;
_28 = Field::<char>(Variant(_32.1.3, 1), 1);
_39.fld0 = Field::<bool>(Variant(_7.1.2, 2), 0);
_32.1.1.1 = _12 * _6.1.1;
_32.1.0 = _6.0;
_32.1.2.fld6.1 = _32.1.1.1 - _32.1.1.1;
Goto(bb31)
}
bb31 = {
RET.2 = Adt18::Variant3 { fld0: _39.fld0 };
_32.1.2.fld5.0 = _15;
_13.1.1.2.fld3 = Field::<i8>(Variant(_32.1.3, 1), 3);
Goto(bb32)
}
bb32 = {
Call(_47 = dump_var(16_usize, 12_usize, Move(_12), 10_usize, Move(_10), 27_usize, Move(_27), 21_usize, Move(_21)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_47 = dump_var(16_usize, 28_usize, Move(_28), 48_usize, _48, 48_usize, _48, 48_usize, _48), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: i128,mut _3: i128,mut _4: isize,mut _5: isize,mut _6: f32,mut _7: isize,mut _8: i8,mut _9: isize,mut _10: u64,mut _11: [u32; 8]) -> *mut isize {
mir! {
type RET = *mut isize;
let _12: [i32; 5];
let _13: &'static i8;
let _14: u64;
let _15: i32;
let _16: i8;
let _17: [u32; 6];
let _18: [u32; 8];
let _19: f64;
let _20: i16;
let _21: isize;
let _22: isize;
let _23: i16;
let _24: char;
let _25: i16;
let _26: isize;
let _27: [i32; 5];
let _28: ([usize; 2],);
let _29: f64;
let _30: isize;
let _31: f64;
let _32: [char; 1];
let _33: ();
let _34: ();
{
_9 = _4 >> _7;
RET = core::ptr::addr_of_mut!(_1);
_10 = 13488156164905917720_u64 | 923729488209348625_u64;
_3 = _2 & _2;
_9 = (*RET);
_12 = [218108032_i32,1965819024_i32,(-793197298_i32),(-2065985268_i32),469634681_i32];
RET = core::ptr::addr_of_mut!(_4);
(*RET) = _5;
_4 = _7;
_5 = !(*RET);
(*RET) = _9 | _9;
_1 = !(*RET);
_6 = _10 as f32;
RET = core::ptr::addr_of_mut!(_4);
_13 = &_8;
_10 = 9561032433846355746_u64 - 3497577597915419009_u64;
(*RET) = _9 | _5;
_3 = _2;
_15 = (-1865217047_i32);
_11 = [4007353877_u32,2423010042_u32,3660505853_u32,3881655441_u32,1740253660_u32,2806349055_u32,134954177_u32,4046727724_u32];
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
315732198155004632746833938064284311782 => bb6,
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
RET = core::ptr::addr_of_mut!(_7);
_8 = (-88_i8) + 105_i8;
_4 = !_1;
_5 = _8 as isize;
_16 = _8;
(*RET) = _1;
_8 = !_16;
_15 = (-769918008_i32) - (-193328211_i32);
_15 = '\u{bd3cf}' as i32;
_9 = 39845_u16 as isize;
_17 = [1612555732_u32,1673750788_u32,2167874745_u32,1750668618_u32,611215353_u32,3278965146_u32];
Goto(bb7)
}
bb7 = {
_13 = &_16;
_4 = (*RET) >> _10;
_6 = 9722211821960135690_usize as f32;
_21 = _7;
_10 = _6 as u64;
_5 = _7 | _1;
_14 = _10;
_22 = (*_13) as isize;
_12 = [_15,_15,_15,_15,_15];
_18 = _11;
_12 = [_15,_15,_15,_15,_15];
_6 = 960043396493040542_usize as f32;
(*RET) = _4 - _5;
_19 = 36108_u16 as f64;
_12 = [_15,_15,_15,_15,_15];
_23 = 1699_i16;
_1 = _15 as isize;
_3 = _2 | _2;
_16 = 7_usize as i8;
_13 = &_8;
_10 = _6 as u64;
_25 = _23 | _23;
_4 = -(*RET);
Goto(bb8)
}
bb8 = {
_21 = _5;
_9 = _21 >> _7;
_25 = _23 - _23;
(*RET) = _25 as isize;
_26 = _9;
_5 = 56186_u16 as isize;
_14 = _10;
_18 = [2496067213_u32,2324685380_u32,3098887282_u32,3745631880_u32,3501921369_u32,3739898327_u32,1082269100_u32,1801266159_u32];
_6 = _3 as f32;
_5 = !(*RET);
_4 = _14 as isize;
_24 = '\u{706dc}';
_30 = _26;
(*RET) = 22843_u16 as isize;
_15 = !(-1729525182_i32);
_7 = _9;
_15 = 264129758_i32;
_31 = _8 as f64;
_30 = _26 << _7;
_21 = (*RET) << _9;
_27 = [_15,_15,_15,_15,_15];
match _23 {
1699 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_25 = -_23;
_28.0 = [5_usize,7994860100043632808_usize];
(*RET) = _30;
_17 = [1641721912_u32,3168065534_u32,3321175975_u32,1463924163_u32,3378765433_u32,1114440446_u32];
RET = core::ptr::addr_of_mut!(_9);
(*RET) = !_30;
(*RET) = _30;
_3 = _19 as i128;
Goto(bb11)
}
bb11 = {
Call(_33 = dump_var(17_usize, 24_usize, Move(_24), 26_usize, Move(_26), 27_usize, Move(_27), 2_usize, Move(_2)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_33 = dump_var(17_usize, 7_usize, Move(_7), 25_usize, Move(_25), 4_usize, Move(_4), 16_usize, Move(_16)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_33 = dump_var(17_usize, 1_usize, Move(_1), 3_usize, Move(_3), 15_usize, Move(_15), 18_usize, Move(_18)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: [bool; 3]) -> isize {
mir! {
type RET = isize;
let _3: Adt18;
let _4: Adt47;
let _5: (i64, u8, u64);
let _6: char;
let _7: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _8: (&'static isize, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), (u8, i8, [bool; 3], ([usize; 2],)), *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20));
let _9: (char, u64, *mut isize);
let _10: [u16; 7];
let _11: usize;
let _12: (i8, isize, Adt18);
let _13: ([usize; 7], *const usize, [i128; 4], [usize; 7]);
let _14: (i128,);
let _15: bool;
let _16: [i8; 1];
let _17: &'static [u16; 1];
let _18: *const ([u64; 1], (i8, isize, Adt18), Adt31, Adt20);
let _19: ([usize; 2],);
let _20: char;
let _21: isize;
let _22: f32;
let _23: ();
let _24: ();
{
_3 = Adt18::Variant0 { fld0: 68_u8,fld1: (-24221412814027319193005969895542729045_i128),fld2: 28531_u16 };
RET = 47979991606260615821471285820865786335_i128 as isize;
_3 = Adt18::Variant0 { fld0: 175_u8,fld1: (-144456215701317636099938018427442123020_i128),fld2: 44323_u16 };
place!(Field::<i128>(Variant(_3, 0), 1)) = (-27882801222420923898991393230204147730_i128) * 125023281087082284841437616933956494698_i128;
place!(Field::<u8>(Variant(_3, 0), 0)) = !109_u8;
_5.1 = 0_usize as u8;
_7.2.0 = !_5.1;
_7.1.2.fld6.1 = _1 * RET;
_7.2.1 = !(-31_i8);
_8.2.2 = [true,true,false];
_9.2 = core::ptr::addr_of_mut!(_7.1.1.1);
_3 = Adt18::Variant3 { fld0: true };
_7.0 = &RET;
_8.1.2.fld5 = ((-147115408923596992995020493150732951105_i128),);
_8.1.2.fld5 = ((-89774386564564402972957003957289783458_i128),);
Goto(bb1)
}
bb1 = {
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(_8.1.2.fld6.1);
_8.2.1 = !_7.2.1;
_7.1.2.fld5 = (_8.1.2.fld5.0,);
_3 = Adt18::Variant0 { fld0: _7.2.0,fld1: _8.1.2.fld5.0,fld2: 55614_u16 };
_7.1.2.fld1.1 = 2315936993_u32 as u64;
_7.3 = core::ptr::addr_of!(_8.1);
_8.1.1.1 = (-957769061_i32) as isize;
_8.1.2.fld6.1 = !_1;
_8.1.2.fld7 = core::ptr::addr_of_mut!(_1);
_8.1.1.0 = _8.2.1;
_7.1.2.fld1.0 = '\u{88322}';
_1 = _8.1.2.fld6.1 - _8.1.2.fld6.1;
_8.1.2.fld6.0 = _7.2.1 & _8.2.1;
_7.1.2.fld6.1 = 5_usize as isize;
_8.1.2.fld0 = 10966072566584260649_usize;
_8.2.3.0 = [_8.1.2.fld0,_8.1.2.fld0];
match _8.1.2.fld0 {
10966072566584260649 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_10 = [43364_u16,42756_u16,52563_u16,18271_u16,7279_u16,34016_u16,45635_u16];
_3 = Adt18::Variant3 { fld0: true };
_13.2 = [_8.1.2.fld5.0,_7.1.2.fld5.0,_8.1.2.fld5.0,_8.1.2.fld5.0];
_8.1.2.fld3 = _8.1.2.fld6.1 as i8;
_8.1.2.fld1.2 = core::ptr::addr_of_mut!(_8.1.1.1);
_7.3 = core::ptr::addr_of!(_8.1);
_7.2.3.0 = [_8.1.2.fld0,_8.1.2.fld0];
_14.0 = !_7.1.2.fld5.0;
_8.2.3 = (_7.2.3.0,);
_8.1.2.fld1 = (_7.1.2.fld1.0, _7.1.2.fld1.1, Move(_8.1.2.fld7));
_1 = -_8.1.2.fld6.1;
_12.1 = _7.1.2.fld1.1 as isize;
_7.2 = (_5.1, _8.2.1, _2, _8.2.3);
_6 = _7.1.2.fld1.0;
_7.2.0 = !_5.1;
_12.2 = Adt18::Variant2 { fld0: true,fld1: 298389114859896209535907619524902256734_u128 };
_5 = (2889748557367650791_i64, _7.2.0, _8.1.2.fld1.1);
Goto(bb4)
}
bb4 = {
_8.2.1 = _8.1.2.fld3;
_7.1.1.0 = -_8.1.2.fld3;
_8.2.3 = (_7.2.3.0,);
_1 = _8.1.2.fld6.1 + _8.1.1.1;
_7.1.2.fld7 = Move(_8.1.2.fld1.2);
_8.1.2.fld6.0 = _8.1.2.fld1.0 as i8;
_9.1 = _8.1.2.fld5.0 as u64;
_12.0 = _7.1.1.0;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 176329635484906943227683102152147037970_u128 };
_5.2 = 1900237359_u32 as u64;
place!(Field::<bool>(Variant(_3, 3), 0)) = _8.2.1 > _12.0;
_8.1.2.fld1 = (_6, _9.1, Move(_7.1.2.fld7));
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(_7.1.2.fld6.1);
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(RET);
place!(Field::<bool>(Variant(_3, 3), 0)) = !false;
Goto(bb5)
}
bb5 = {
_8.1.2.fld6.2 = Adt18::Variant3 { fld0: Field::<bool>(Variant(_3, 3), 0) };
_14 = _8.1.2.fld5;
_7.1.2.fld6.0 = 313347872391123777558701950576385126462_u128 as i8;
_13.3 = [_8.1.2.fld0,_8.1.2.fld0,_8.1.2.fld0,_8.1.2.fld0,_8.1.2.fld0,_8.1.2.fld0,_8.1.2.fld0];
_7.0 = &_7.1.1.1;
_8.2.1 = 90969861338768014236614764204351115576_u128 as i8;
_12 = (_8.1.2.fld3, _8.1.2.fld6.1, _3);
_7.1.2.fld1.1 = !_8.1.2.fld1.1;
_8.1.0 = [_5.2];
_8.1.2.fld5 = (_7.1.2.fld5.0,);
_7.1.2.fld6 = _12;
place!(Field::<bool>(Variant(_7.1.2.fld6.2, 3), 0)) = Field::<bool>(Variant(_12.2, 3), 0) ^ Field::<bool>(Variant(_3, 3), 0);
_8.1.1.2 = _12.2;
place!(Field::<bool>(Variant(_8.1.2.fld6.2, 3), 0)) = Field::<bool>(Variant(_3, 3), 0);
_8.1.1.1 = _8.1.2.fld6.1;
SetDiscriminant(_3, 1);
_7.1.2.fld2 = [_8.1.1.1];
place!(Field::<u64>(Variant(_3, 1), 0)) = _9.1;
_8.1.1.1 = 51745_u16 as isize;
_10 = [23259_u16,35114_u16,5455_u16,52509_u16,38997_u16,32731_u16,39493_u16];
_5 = (8419198337665222474_i64, _7.2.0, _9.1);
_9.0 = _8.1.2.fld1.0;
_8.2.0 = !_7.2.0;
_8.2.1 = _8.1.2.fld3;
_7.1.2.fld4 = [_8.2.1,_7.1.1.0,_7.1.2.fld6.0,_7.1.2.fld6.0];
_8.1.2.fld6.2 = Adt18::Variant2 { fld0: Field::<bool>(Variant(_7.1.2.fld6.2, 3), 0),fld1: 265824664394172794571516169583356532384_u128 };
match _8.1.2.fld5.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
250507980356374060490417603474478427998 => bb11,
_ => bb10
}
}
bb6 = {
_8.2.1 = _8.1.2.fld3;
_7.1.1.0 = -_8.1.2.fld3;
_8.2.3 = (_7.2.3.0,);
_1 = _8.1.2.fld6.1 + _8.1.1.1;
_7.1.2.fld7 = Move(_8.1.2.fld1.2);
_8.1.2.fld6.0 = _8.1.2.fld1.0 as i8;
_9.1 = _8.1.2.fld5.0 as u64;
_12.0 = _7.1.1.0;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 176329635484906943227683102152147037970_u128 };
_5.2 = 1900237359_u32 as u64;
place!(Field::<bool>(Variant(_3, 3), 0)) = _8.2.1 > _12.0;
_8.1.2.fld1 = (_6, _9.1, Move(_7.1.2.fld7));
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(_7.1.2.fld6.1);
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(RET);
place!(Field::<bool>(Variant(_3, 3), 0)) = !false;
Goto(bb5)
}
bb7 = {
_10 = [43364_u16,42756_u16,52563_u16,18271_u16,7279_u16,34016_u16,45635_u16];
_3 = Adt18::Variant3 { fld0: true };
_13.2 = [_8.1.2.fld5.0,_7.1.2.fld5.0,_8.1.2.fld5.0,_8.1.2.fld5.0];
_8.1.2.fld3 = _8.1.2.fld6.1 as i8;
_8.1.2.fld1.2 = core::ptr::addr_of_mut!(_8.1.1.1);
_7.3 = core::ptr::addr_of!(_8.1);
_7.2.3.0 = [_8.1.2.fld0,_8.1.2.fld0];
_14.0 = !_7.1.2.fld5.0;
_8.2.3 = (_7.2.3.0,);
_8.1.2.fld1 = (_7.1.2.fld1.0, _7.1.2.fld1.1, Move(_8.1.2.fld7));
_1 = -_8.1.2.fld6.1;
_12.1 = _7.1.2.fld1.1 as isize;
_7.2 = (_5.1, _8.2.1, _2, _8.2.3);
_6 = _7.1.2.fld1.0;
_7.2.0 = !_5.1;
_12.2 = Adt18::Variant2 { fld0: true,fld1: 298389114859896209535907619524902256734_u128 };
_5 = (2889748557367650791_i64, _7.2.0, _8.1.2.fld1.1);
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(_8.1.2.fld6.1);
_8.2.1 = !_7.2.1;
_7.1.2.fld5 = (_8.1.2.fld5.0,);
_3 = Adt18::Variant0 { fld0: _7.2.0,fld1: _8.1.2.fld5.0,fld2: 55614_u16 };
_7.1.2.fld1.1 = 2315936993_u32 as u64;
_7.3 = core::ptr::addr_of!(_8.1);
_8.1.1.1 = (-957769061_i32) as isize;
_8.1.2.fld6.1 = !_1;
_8.1.2.fld7 = core::ptr::addr_of_mut!(_1);
_8.1.1.0 = _8.2.1;
_7.1.2.fld1.0 = '\u{88322}';
_1 = _8.1.2.fld6.1 - _8.1.2.fld6.1;
_8.1.2.fld6.0 = _7.2.1 & _8.2.1;
_7.1.2.fld6.1 = 5_usize as isize;
_8.1.2.fld0 = 10966072566584260649_usize;
_8.2.3.0 = [_8.1.2.fld0,_8.1.2.fld0];
match _8.1.2.fld0 {
10966072566584260649 => bb3,
_ => bb2
}
}
bb10 = {
Return()
}
bb11 = {
match _8.1.2.fld0 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb8,
10966072566584260649 => bb13,
_ => bb12
}
}
bb12 = {
_8.2.1 = _8.1.2.fld3;
_7.1.1.0 = -_8.1.2.fld3;
_8.2.3 = (_7.2.3.0,);
_1 = _8.1.2.fld6.1 + _8.1.1.1;
_7.1.2.fld7 = Move(_8.1.2.fld1.2);
_8.1.2.fld6.0 = _8.1.2.fld1.0 as i8;
_9.1 = _8.1.2.fld5.0 as u64;
_12.0 = _7.1.1.0;
_7.1.1.2 = Adt18::Variant2 { fld0: true,fld1: 176329635484906943227683102152147037970_u128 };
_5.2 = 1900237359_u32 as u64;
place!(Field::<bool>(Variant(_3, 3), 0)) = _8.2.1 > _12.0;
_8.1.2.fld1 = (_6, _9.1, Move(_7.1.2.fld7));
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(_7.1.2.fld6.1);
_7.1.2.fld1.2 = core::ptr::addr_of_mut!(RET);
place!(Field::<bool>(Variant(_3, 3), 0)) = !false;
Goto(bb5)
}
bb13 = {
place!(Field::<f64>(Variant(_3, 1), 1)) = _7.1.2.fld5.0 as f64;
_12.0 = -_8.1.2.fld3;
_8.1.2.fld0 = 16947608141105634673_usize >> _7.1.1.0;
_8.1.1.0 = !_7.1.2.fld6.0;
RET = !_8.1.2.fld6.1;
place!(Field::<bool>(Variant(_7.1.1.2, 2), 0)) = !Field::<bool>(Variant(_12.2, 3), 0);
_7.1.1.2 = _7.1.2.fld6.2;
_7.2.3.0 = [_8.1.2.fld0,_8.1.2.fld0];
SetDiscriminant(_7.1.1.2, 0);
_7.1.2.fld7 = core::ptr::addr_of_mut!(_12.1);
_8.1.2.fld6.0 = _7.1.1.0;
place!(Field::<u8>(Variant(_7.1.1.2, 0), 0)) = !_8.2.0;
_5.2 = _7.1.2.fld1.0 as u64;
place!(Field::<i128>(Variant(_7.1.1.2, 0), 1)) = !_8.1.2.fld5.0;
_7.1.2.fld5 = (_14.0,);
_7.2.3 = (_8.2.3.0,);
place!(Field::<bool>(Variant(_8.1.1.2, 3), 0)) = Field::<bool>(Variant(_12.2, 3), 0);
Goto(bb14)
}
bb14 = {
Call(_23 = dump_var(18_usize, 1_usize, Move(_1), 14_usize, Move(_14), 2_usize, Move(_2), 24_usize, _24), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize) -> (u8, u64, [i32; 4], (char, u64, *mut isize)) {
mir! {
type RET = (u8, u64, [i32; 4], (char, u64, *mut isize));
let _2: &'static &'static u8;
let _3: Adt65;
let _4: &'static i64;
let _5: i64;
let _6: &'static i16;
let _7: (i128,);
let _8: (Adt65, (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31), (i8, isize, Adt18));
let _9: Adt47;
let _10: *const u16;
let _11: f32;
let _12: i8;
let _13: i16;
let _14: &'static i16;
let _15: Adt31;
let _16: [usize; 1];
let _17: u8;
let _18: &'static (&'static &'static u8, usize);
let _19: i8;
let _20: [bool; 3];
let _21: (i8, isize, Adt18);
let _22: i16;
let _23: [usize; 2];
let _24: u8;
let _25: Adt20;
let _26: u32;
let _27: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31);
let _28: Adt65;
let _29: f32;
let _30: char;
let _31: isize;
let _32: [i128; 7];
let _33: i128;
let _34: [u16; 1];
let _35: &'static i8;
let _36: u16;
let _37: i16;
let _38: Adt77;
let _39: Adt20;
let _40: bool;
let _41: ();
let _42: ();
{
RET.3.1 = 8926_i16 as u64;
RET.3.2 = core::ptr::addr_of_mut!(_1);
RET.1 = 6903281646306240785_i64 as u64;
RET.3.1 = !RET.1;
Goto(bb1)
}
bb1 = {
_3.fld2 = ((-44825267833982662894846756671961145735_i128),);
_3.fld2.0 = (-33294315939711434936147301759317489657_i128);
_8.1.1.1.0 = (-21_i8) * (-95_i8);
_8.0.fld1 = 5_usize as u64;
_8.1.3.fld7 = Move(RET.3.2);
_3.fld6 = [10755096733127611237_usize,7_usize,6319643728493290067_usize,7_usize,7_usize,5_usize,4_usize];
_8.1.3.fld4 = [_8.1.1.1.0,_8.1.1.1.0,_8.1.1.1.0,_8.1.1.1.0];
RET.0 = 236_u8;
Goto(bb2)
}
bb2 = {
_8.1.1.2.fld4 = [_8.1.1.1.0,_8.1.1.1.0,_8.1.1.1.0,_8.1.1.1.0];
_8.1.3.fld1 = ('\u{8b47a}', RET.3.1, Move(_8.1.3.fld7));
_8.0.fld2.0 = -_3.fld2.0;
_8.1.1.2.fld1 = (_8.1.3.fld1.0, _8.1.3.fld1.1, Move(_8.1.3.fld1.2));
_3.fld3 = 15586566636687125080_usize as u32;
_8.0.fld1 = _8.1.3.fld1.1 ^ RET.1;
_8.1.3.fld6.1 = (-7703793531450759282_i64) as isize;
_3.fld3 = 3033851510_u32 & 1195796954_u32;
_8.1.1.1.0 = !96_i8;
_8.1.1.2.fld0 = 6_usize ^ 1_usize;
_8.1.3.fld3 = false as i8;
_8.1.1.2.fld6.0 = _8.1.3.fld3 * _8.1.3.fld3;
_8.1.3.fld6.1 = !_1;
RET.3.0 = _8.1.3.fld1.0;
_3.fld1 = _8.1.3.fld1.1 * RET.1;
_8.1.1.2.fld5.0 = _3.fld2.0 + _3.fld2.0;
_8.0.fld0 = !true;
_8.0.fld1 = _3.fld1;
_3.fld0 = _8.1.1.2.fld5.0 > _8.0.fld2.0;
_3.fld2 = (_8.0.fld2.0,);
_15.fld1 = Move(_8.1.1.2.fld1);
Goto(bb3)
}
bb3 = {
_15.fld6.1 = _1 * _8.1.3.fld6.1;
_8.1.1.0 = [_8.1.3.fld1.1];
_8.1.3.fld5.0 = _8.0.fld2.0;
_8.1.3.fld6.2 = Adt18::Variant2 { fld0: _8.0.fld0,fld1: 267971251730911638152410241159845242333_u128 };
_8.0.fld2.0 = !_8.1.1.2.fld5.0;
_3.fld5 = 16480_u16 as f64;
_3.fld4 = [6772498057033056319_i64,(-707603014109679965_i64),(-8841981299155791557_i64),(-8403387476698797145_i64),(-7402220847471389751_i64),(-7625124363437778145_i64)];
_8.2.0 = -_8.1.1.2.fld6.0;
place!(Field::<u128>(Variant(_8.1.3.fld6.2, 2), 1)) = !101846666105295180225027996352896957622_u128;
_11 = (-4546554791636442395_i64) as f32;
_8.0.fld5 = _8.1.1.2.fld0 as f64;
_8.1.1.2.fld4 = _8.1.3.fld4;
match RET.0 {
236 => bb4,
_ => bb2
}
}
bb4 = {
_8.0.fld4 = [4248169545217220374_i64,3726551385315934933_i64,(-8268200116929775955_i64),625151915529572303_i64,2298146468624292615_i64,(-1057427924640948130_i64)];
RET.2 = [921962968_i32,1970773020_i32,2110000377_i32,567580329_i32];
_8.1.1.1.2 = _8.1.3.fld6.2;
_8.1.3.fld4 = _8.1.1.2.fld4;
Goto(bb5)
}
bb5 = {
_14 = &_8.1.0;
_15.fld2 = [_8.1.3.fld6.1];
Goto(bb6)
}
bb6 = {
_6 = &(*_14);
RET.1 = !_8.1.3.fld1.1;
_4 = &_5;
_8.1.3.fld6 = (_8.2.0, _1, _8.1.1.1.2);
_8.1.1.2.fld1.1 = _15.fld1.1;
_8.1.3.fld6.1 = -_15.fld6.1;
_5 = -(-3185519676558724449_i64);
_8.1.3.fld1.2 = Move(_15.fld1.2);
_8.1.3.fld7 = core::ptr::addr_of_mut!(_15.fld6.1);
_15.fld6.1 = _1 - _8.1.3.fld6.1;
_8.0.fld1 = !_3.fld1;
_8.1.3.fld6.0 = _8.1.1.2.fld6.0;
_21 = (_8.1.1.1.0, _1, _8.1.3.fld6.2);
RET.3.1 = _8.0.fld1 & RET.1;
Goto(bb7)
}
bb7 = {
_19 = _8.1.3.fld3;
_17 = !RET.0;
_11 = 16517_i16 as f32;
place!(Field::<u128>(Variant(_21.2, 2), 1)) = 2085868277_i32 as u128;
_8.1.3.fld1.1 = !RET.3.1;
_8.1.1.2.fld7 = Move(_8.1.3.fld7);
_8.1.1.2.fld5 = (_8.0.fld2.0,);
RET.3.0 = _15.fld1.0;
_6 = &_22;
RET.3.0 = _8.1.3.fld1.0;
RET.0 = _17 >> _1;
_8.0.fld3 = _3.fld3 >> _15.fld6.1;
_11 = 31980_i16 as f32;
Goto(bb8)
}
bb8 = {
RET.3.2 = Move(_8.1.3.fld1.2);
_8.1.1.2.fld2 = _15.fld2;
_20 = [_3.fld0,Field::<bool>(Variant(_8.1.3.fld6.2, 2), 0),Field::<bool>(Variant(_21.2, 2), 0)];
place!(Field::<u128>(Variant(_21.2, 2), 1)) = Field::<u128>(Variant(_8.1.1.1.2, 2), 1);
_8.1.1.0 = [RET.3.1];
RET.3 = (_15.fld1.0, _8.0.fld1, Move(_8.1.1.2.fld7));
place!(Field::<bool>(Variant(_8.1.1.1.2, 2), 0)) = !_8.0.fld0;
_8.0.fld2.0 = _8.1.1.2.fld5.0;
_19 = RET.3.0 as i8;
_13 = 19203_i16 + 18706_i16;
_3.fld4 = [_5,_5,_5,_5,_5,_5];
place!(Field::<u128>(Variant(_8.1.3.fld6.2, 2), 1)) = Field::<u128>(Variant(_8.1.1.1.2, 2), 1) >> _1;
RET.3.1 = _8.0.fld1 ^ _15.fld1.1;
_8.1.1.2.fld1.1 = RET.0 as u64;
_8.1.3.fld2 = _15.fld2;
_8.2.2 = Adt18::Variant1 { fld0: _8.1.1.2.fld1.1,fld1: _8.0.fld5,fld2: _15.fld6.1,fld3: _8.1.1.2.fld2,fld4: _5,fld5: _11 };
_8.1.1.1.1 = _8.0.fld2.0 as isize;
_27.1.3 = Adt20::Variant0 { fld0: _8.2.2,fld1: _8.1.3.fld2,fld2: Field::<u64>(Variant(_8.2.2, 1), 0),fld3: _19,fld4: _13,fld5: _8.0.fld3,fld6: _5,fld7: _8.1.3.fld5.0 };
_15.fld1.1 = !_8.1.3.fld1.1;
_27.3.fld6.0 = -Field::<i8>(Variant(_27.1.3, 0), 3);
_13 = Field::<i16>(Variant(_27.1.3, 0), 4) & Field::<i16>(Variant(_27.1.3, 0), 4);
SetDiscriminant(_8.1.1.1.2, 2);
_27.1.2.fld2 = [Field::<isize>(Variant(_8.2.2, 1), 2)];
_15.fld7 = Move(RET.3.2);
_7.0 = -Field::<i128>(Variant(_27.1.3, 0), 7);
SetDiscriminant(_8.2.2, 3);
Goto(bb9)
}
bb9 = {
_3.fld5 = _8.1.1.2.fld0 as f64;
_27.3.fld2 = [_8.1.3.fld6.1];
_28.fld4 = _8.0.fld4;
place!(Field::<u128>(Variant(_8.1.1.1.2, 2), 1)) = Field::<u128>(Variant(_8.1.3.fld6.2, 2), 1);
_3.fld2.0 = Field::<i128>(Variant(_27.1.3, 0), 7);
_27.3.fld1.1 = !Field::<u64>(Variant(_27.1.3, 0), 2);
_15.fld5.0 = -_8.1.1.2.fld5.0;
_11 = Field::<i64>(Variant(_27.1.3, 0), 6) as f32;
_8.1.3.fld6 = (_8.1.1.1.0, Field::<isize>(Variant(Field::<Adt18>(Variant(_27.1.3, 0), 0), 1), 2), Field::<Adt18>(Variant(_27.1.3, 0), 0));
_27.1.1.1 = !_8.1.3.fld6.1;
Goto(bb10)
}
bb10 = {
_8.1.1.2.fld2 = _27.1.2.fld2;
_15.fld5.0 = Field::<i128>(Variant(_27.1.3, 0), 7);
RET.2 = [605010701_i32,(-486209395_i32),(-894212504_i32),(-1975349722_i32)];
_27.3.fld6.1 = _15.fld6.1 * _27.1.1.1;
_6 = &_8.1.0;
_27.3.fld6.0 = _8.1.1.2.fld0 as i8;
_27.3.fld7 = Move(_15.fld7);
RET.3 = (_15.fld1.0, _27.3.fld1.1, Move(_27.3.fld7));
_8.1.3.fld1.0 = _15.fld1.0;
_8.1.3.fld1 = (RET.3.0, Field::<u64>(Variant(Field::<Adt18>(Variant(_27.1.3, 0), 0), 1), 0), Move(RET.3.2));
_27.1.2.fld2 = [_27.3.fld6.1];
_8.1.1.2.fld6 = (_19, Field::<isize>(Variant(Field::<Adt18>(Variant(_27.1.3, 0), 0), 1), 2), _8.1.3.fld6.2);
_24 = RET.0 + RET.0;
_27.1.2 = Adt31 { fld0: _8.1.1.2.fld0,fld1: Move(_8.1.3.fld1),fld2: _27.3.fld2,fld3: _8.1.1.2.fld6.0,fld4: _8.1.3.fld4,fld5: _8.0.fld2,fld6: _8.1.1.2.fld6,fld7: Move(_8.1.3.fld1.2) };
place!(Field::<u32>(Variant(_27.1.3, 0), 5)) = !_8.0.fld3;
_3.fld1 = Field::<f64>(Variant(_8.1.1.2.fld6.2, 1), 1) as u64;
SetDiscriminant(Field::<Adt18>(Variant(_27.1.3, 0), 0), 1);
_8.1.3 = Adt31 { fld0: _8.1.1.2.fld0,fld1: Move(_27.1.2.fld1),fld2: _27.3.fld2,fld3: _27.1.2.fld3,fld4: _27.1.2.fld4,fld5: _15.fld5,fld6: _8.1.1.2.fld6,fld7: Move(_27.1.2.fld1.2) };
_15.fld0 = !_8.1.1.2.fld0;
_8.0.fld0 = _3.fld0;
RET.1 = _3.fld1 | RET.3.1;
Goto(bb11)
}
bb11 = {
_32 = [_15.fld5.0,_15.fld5.0,_8.0.fld2.0,_8.0.fld2.0,_27.1.2.fld5.0,_27.1.2.fld5.0,Field::<i128>(Variant(_27.1.3, 0), 7)];
_8.0.fld5 = Field::<f64>(Variant(_8.1.1.2.fld6.2, 1), 1);
_28.fld6 = [_8.1.1.2.fld0,_27.1.2.fld0,_27.1.2.fld0,_8.1.3.fld0,_27.1.2.fld0,_8.1.1.2.fld0,_27.1.2.fld0];
_8.1.2 = Adt20::Variant0 { fld0: _8.1.1.2.fld6.2,fld1: _8.1.3.fld2,fld2: Field::<u64>(Variant(_8.1.1.2.fld6.2, 1), 0),fld3: _27.1.2.fld3,fld4: _13,fld5: Field::<u32>(Variant(_27.1.3, 0), 5),fld6: Field::<i64>(Variant(_8.1.1.2.fld6.2, 1), 4),fld7: _8.1.3.fld5.0 };
_8.1.1.1.1 = !_15.fld6.1;
_27.1.2.fld0 = _8.1.1.2.fld0 & _8.1.1.2.fld0;
_27.3.fld3 = _8.1.3.fld6.0;
place!(Field::<f32>(Variant(_8.1.3.fld6.2, 1), 5)) = Field::<f32>(Variant(Field::<Adt18>(Variant(_8.1.2, 0), 0), 1), 5) - Field::<f32>(Variant(_8.1.1.2.fld6.2, 1), 5);
_27.1.1.2 = Field::<Adt18>(Variant(_8.1.2, 0), 0);
place!(Field::<i64>(Variant(_27.1.1.2, 1), 4)) = Field::<i64>(Variant(_8.1.3.fld6.2, 1), 4) * Field::<i64>(Variant(Field::<Adt18>(Variant(_8.1.2, 0), 0), 1), 4);
Goto(bb12)
}
bb12 = {
RET.3.2 = Move(_8.1.3.fld1.2);
_8.1.1.2.fld1 = Move(RET.3);
_27.3.fld6 = (Field::<i8>(Variant(_27.1.3, 0), 3), _8.1.3.fld6.1, Field::<Adt18>(Variant(_8.1.2, 0), 0));
_26 = _8.0.fld3 - Field::<u32>(Variant(_8.1.2, 0), 5);
place!(Field::<f32>(Variant(_27.3.fld6.2, 1), 5)) = Field::<u128>(Variant(_8.1.1.1.2, 2), 1) as f32;
_15.fld1.2 = Move(_27.1.2.fld7);
_30 = _8.1.3.fld1.0;
place!(Field::<i128>(Variant(_27.1.3, 0), 7)) = _13 as i128;
_15.fld6.0 = _24 as i8;
RET.1 = !Field::<u64>(Variant(_8.1.3.fld6.2, 1), 0);
place!(Field::<isize>(Variant(place!(Field::<Adt18>(Variant(_8.1.2, 0), 0)), 1), 2)) = Field::<isize>(Variant(_27.1.1.2, 1), 2) & Field::<isize>(Variant(_27.1.1.2, 1), 2);
_33 = _8.1.1.2.fld5.0;
place!(Field::<[isize; 1]>(Variant(_27.3.fld6.2, 1), 3)) = [Field::<isize>(Variant(_27.1.1.2, 1), 2)];
_8.1.1.2.fld0 = _8.1.3.fld0;
place!(Field::<Adt18>(Variant(_27.1.3, 0), 0)) = Adt18::Variant0 { fld0: _24,fld1: _27.1.2.fld5.0,fld2: 53596_u16 };
place!(Field::<isize>(Variant(place!(Field::<Adt18>(Variant(_8.1.2, 0), 0)), 1), 2)) = Field::<isize>(Variant(_27.1.2.fld6.2, 1), 2) ^ Field::<isize>(Variant(_8.1.1.2.fld6.2, 1), 2);
place!(Field::<[isize; 1]>(Variant(_8.1.1.2.fld6.2, 1), 3)) = [Field::<isize>(Variant(_27.1.1.2, 1), 2)];
_8.2.2 = _8.1.1.2.fld6.2;
RET.3.1 = Field::<f64>(Variant(_8.1.3.fld6.2, 1), 1) as u64;
_38.fld2 = Field::<f64>(Variant(_27.3.fld6.2, 1), 1);
Goto(bb13)
}
bb13 = {
RET.3.1 = Field::<i64>(Variant(_27.1.1.2, 1), 4) as u64;
place!(Field::<isize>(Variant(_8.2.2, 1), 2)) = -Field::<isize>(Variant(_27.3.fld6.2, 1), 2);
place!(Field::<i64>(Variant(_8.2.2, 1), 4)) = !_5;
place!(Field::<isize>(Variant(_8.2.2, 1), 2)) = _27.1.2.fld0 as isize;
_8.1.1.2.fld1 = Move(_15.fld1);
_30 = _8.1.3.fld1.0;
RET.3 = (_8.1.1.2.fld1.0, Field::<u64>(Variant(_8.1.2, 0), 2), Move(_8.1.3.fld7));
_8.1.1.1.0 = Field::<i128>(Variant(_27.1.3, 0), 7) as i8;
_27.1.2.fld1.2 = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(place!(Field::<Adt18>(Variant(_8.1.2, 0), 0)), 1), 2)));
Goto(bb14)
}
bb14 = {
Call(_41 = dump_var(19_usize, 33_usize, Move(_33), 24_usize, Move(_24), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_41 = dump_var(19_usize, 13_usize, Move(_13), 7_usize, Move(_7), 42_usize, _42, 42_usize, _42), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-1206_i16)), std::hint::black_box('\u{7bc17}'), std::hint::black_box(94_isize));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: u8,
fld1: i128,
fld2: u16,

},
Variant1{
fld0: u64,
fld1: f64,
fld2: isize,
fld3: [isize; 1],
fld4: i64,
fld5: f32,

},
Variant2{
fld0: bool,
fld1: u128,

},
Variant3{
fld0: bool,

}}
impl PrintFDebug for Adt20{
	unsafe fn printf_debug(&self){unsafe{printf("Adt20::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt20 {
Variant0{
fld0: Adt18,
fld1: [isize; 1],
fld2: u64,
fld3: i8,
fld4: i16,
fld5: u32,
fld6: i64,
fld7: i128,

},
Variant1{
fld0: f64,
fld1: char,
fld2: u128,
fld3: i8,
fld4: u64,
fld5: i32,
fld6: u32,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt31{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt31 {
fld0: usize,
fld1: (char, u64, *mut isize),
fld2: [isize; 1],
fld3: i8,
fld4: [i8; 4],
fld5: (i128,),
fld6: (i8, isize, Adt18),
fld7: *mut isize,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [usize; 2],
fld1: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31),
fld2: Adt18,

},
Variant1{
fld0: bool,
fld1: f32,
fld2: isize,
fld3: [isize; 1],
fld4: (char, u64, *mut isize),
fld5: ([u64; 1], (i8, isize, Adt18), Adt31, Adt20),

},
Variant2{
fld0: (i16, ([u64; 1], (i8, isize, Adt18), Adt31, Adt20), Adt20, Adt31),
fld1: i64,
fld2: *const u16,
fld3: Adt31,
fld4: [u64; 1],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: i8,

},
Variant1{
fld0: [u32; 8],
fld1: (char, f64),
fld2: u64,
fld3: [i64; 6],
fld4: [isize; 1],
fld5: (i8, isize, Adt18),

},
Variant2{
fld0: (i128,),

},
Variant3{
fld0: [isize; 1],
fld1: i128,
fld2: Adt18,
fld3: *const u16,
fld4: (u8, u64, [i32; 4], (char, u64, *mut isize)),
fld5: [u64; 1],
fld6: [u32; 8],

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: bool,
fld1: Adt52,
fld2: (u8, i8, [bool; 3], ([usize; 2],)),
fld3: Adt18,
fld4: (i8, isize, Adt18),
fld5: Adt31,

},
Variant1{
fld0: u64,

},
Variant2{
fld0: u8,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt65{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt65 {
fld0: bool,
fld1: u64,
fld2: (i128,),
fld3: u32,
fld4: [i64; 6],
fld5: f64,
fld6: [usize; 7],
}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt77{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt77 {
fld0: bool,
fld1: [u32; 6],
fld2: f64,
fld3: i16,
}

