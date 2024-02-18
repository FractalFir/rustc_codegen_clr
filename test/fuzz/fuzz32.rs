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
pub fn fn0(mut _1: u64,mut _2: u32,mut _3: u8,mut _4: i8,mut _5: i16,mut _6: u128,mut _7: i64,mut _8: i128,mut _9: usize) -> *mut u16 {
mir! {
type RET = *mut u16;
let _10: [u16; 2];
let _11: f64;
let _12: bool;
let _13: bool;
let _14: u128;
let _15: [bool; 6];
let _16: (i128, &'static i8, Adt20);
let _17: ((f32, &'static (u32, *mut bool, u16, *mut bool)),);
let _18: *mut u16;
let _19: i8;
let _20: &'static usize;
let _21: isize;
let _22: (*const u64, [i16; 6]);
let _23: *const [u16; 2];
let _24: i32;
let _25: &'static (u32, *mut bool, u16, *mut bool);
let _26: isize;
let _27: isize;
let _28: isize;
let _29: f32;
let _30: Adt20;
let _31: bool;
let _32: (u32, *mut bool, u16, *mut bool);
let _33: char;
let _34: ((usize, &'static &'static usize), f32, u128, [u64; 6]);
let _35: char;
let _36: &'static [usize; 5];
let _37: char;
let _38: isize;
let _39: ((*mut u16, u32), [bool; 6], isize);
let _40: *mut u16;
let _41: i64;
let _42: isize;
let _43: (u32, *mut bool, u16, *mut bool);
let _44: u16;
let _45: (i128, &'static i8, Adt20);
let _46: &'static i32;
let _47: ((usize, &'static &'static usize), f32, u128, [u64; 6]);
let _48: (char, i8, usize, bool);
let _49: *mut bool;
let _50: *mut u16;
let _51: (*const u64, (u32, *mut bool, u16, *mut bool), f32);
let _52: bool;
let _53: *const u64;
let _54: i8;
let _55: bool;
let _56: [u8; 6];
let _57: Adt29;
let _58: u8;
let _59: ();
let _60: ();
{
_3 = 41216_u16 as u8;
_6 = 1628493647107864931_i64 as u128;
_1 = 1_i8 as u64;
_8 = -(-169289695709992430557220385291747582441_i128);
_4 = (-11_i8) >> _1;
_4 = (-71_i8) ^ 64_i8;
_2 = !3282139838_u32;
_9 = 9978807806371396499_usize;
_7 = 25763_i16 as i64;
_2 = !2760228094_u32;
Goto(bb1)
}
bb1 = {
_10 = [49008_u16,26307_u16];
_8 = 97788262812182407781380786939453502031_i128;
_6 = (-19682_i16) as u128;
_1 = _3 as u64;
_11 = 9223372036854775807_isize as f64;
Goto(bb2)
}
bb2 = {
_6 = _11 as u128;
_5 = 15103_i16;
_5 = 30231_i16 + (-14617_i16);
_14 = _6;
_12 = false;
_3 = 44_u8;
_13 = _12;
_4 = 23_i8;
_9 = !5284146874514196427_usize;
_13 = _12;
_1 = 10658364106090032920_u64 | 10908334655449281707_u64;
_6 = _14 << _3;
_9 = 17422262659863161016_usize;
_1 = _5 as u64;
_13 = _12;
_1 = 7038764006872438041_u64;
_8 = (-33393282059659500859483857953628947831_i128);
_6 = _14 ^ _14;
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
7038764006872438041 => bb10,
_ => bb9
}
}
bb3 = {
_10 = [49008_u16,26307_u16];
_8 = 97788262812182407781380786939453502031_i128;
_6 = (-19682_i16) as u128;
_1 = _3 as u64;
_11 = 9223372036854775807_isize as f64;
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
_9 = (-86509992_i32) as usize;
_1 = 30608_u16 as u64;
_16.1 = &_4;
_16.2.fld4 = _5;
_6 = _4 as u128;
_15 = [_12,_12,_12,_12,_12,_12];
_4 = _13 as i8;
_16.1 = &_4;
_16.0 = _11 as i128;
_16.2.fld2 = !(-9223372036854775808_isize);
_4 = 79_i8;
_11 = 1592670716_i32 as f64;
_17.0.0 = _1 as f32;
_2 = _3 as u32;
_13 = _16.2.fld4 >= _5;
_8 = _16.0;
_6 = _2 as u128;
_16.2.fld0 = _9 << _9;
_10 = [18680_u16,22216_u16];
_7 = 1298124450994338629_i64 & (-6205119726780937693_i64);
_16.2.fld2 = !(-9223372036854775808_isize);
_3 = 1_u8;
_10 = [48864_u16,55606_u16];
_20 = &_16.2.fld0;
_14 = !_6;
Call(_6 = fn1(Move(_20), _7, _3, _16.2.fld0, _16.2.fld0, (*_20), _7, _1, _3, (*_20)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20 = &_9;
_16.2 = Adt20 { fld0: (*_20),fld1: _1,fld2: 9223372036854775807_isize,fld3: _11,fld4: _5,fld5: _3,fld6: _2 };
_16.2 = Adt20 { fld0: (*_20),fld1: _1,fld2: 9223372036854775807_isize,fld3: _11,fld4: _5,fld5: _3,fld6: _2 };
_16.2.fld1 = !_1;
_2 = !_16.2.fld6;
_6 = !_14;
Goto(bb12)
}
bb12 = {
_8 = _16.0 - _16.0;
_22.0 = core::ptr::addr_of!(_1);
_19 = -_4;
_17.0.0 = _9 as f32;
_21 = _9 as isize;
_20 = &(*_20);
_16.2.fld1 = (-1658395202_i32) as u64;
Goto(bb13)
}
bb13 = {
match _16.2.fld2 {
0 => bb8,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb10,
9223372036854775807 => bb14,
_ => bb6
}
}
bb14 = {
_16.0 = _8 + _8;
_2 = !_16.2.fld6;
_16.2.fld3 = -_11;
_8 = _16.0;
_13 = _12;
_14 = _6;
_8 = -_16.0;
_22.1 = [_16.2.fld4,_5,_5,_16.2.fld4,_5,_5];
_1 = _16.2.fld1 << _8;
_26 = _16.2.fld2 * _16.2.fld2;
_22.0 = core::ptr::addr_of!(_1);
_21 = _26;
_24 = -(-1327019512_i32);
_30 = Move(_16.2);
_16.2 = Adt20 { fld0: _9,fld1: _1,fld2: _26,fld3: _11,fld4: _30.fld4,fld5: _3,fld6: _2 };
_30.fld1 = _16.2.fld3 as u64;
_28 = (*_20) as isize;
_19 = _4 * _4;
_12 = !_13;
_4 = !_19;
_16.1 = &_4;
_30.fld1 = !_16.2.fld1;
_29 = _17.0.0;
_30.fld3 = _4 as f64;
_22.1 = [_30.fld4,_16.2.fld4,_16.2.fld4,_5,_16.2.fld4,_5];
Goto(bb15)
}
bb15 = {
_7 = 3944728554833938130_i64;
_26 = _30.fld2 * _16.2.fld2;
_15 = [_12,_12,_13,_13,_12,_13];
_31 = _1 != _1;
_32.0 = _2;
_12 = _13 <= _31;
_34.0.1 = &_20;
_34.0.0 = _30.fld3 as usize;
Goto(bb16)
}
bb16 = {
_18 = core::ptr::addr_of_mut!(_32.2);
(*_18) = 7813_u16;
_17.0.0 = -_29;
_32.1 = core::ptr::addr_of_mut!(_31);
_10 = [(*_18),(*_18)];
_30.fld0 = _9;
_30.fld3 = _4 as f64;
_26 = _21;
_17.0.0 = _4 as f32;
_16.2.fld3 = -_11;
_39.0.0 = core::ptr::addr_of_mut!(_32.2);
_39.2 = _31 as isize;
_7 = 5612755795187524504_i64 ^ 1259831384484598375_i64;
_26 = _16.2.fld2 & _39.2;
_32.1 = core::ptr::addr_of_mut!(_31);
Goto(bb17)
}
bb17 = {
_10 = [(*_18),_32.2];
(*_18) = _24 as u16;
_17.0.1 = &_32;
_34.2 = !_6;
_30.fld0 = !_34.0.0;
_42 = -_39.2;
_15 = [_13,_31,_12,_31,_12,_12];
_30.fld4 = !_5;
RET = Move(_18);
_16.2.fld6 = _24 as u32;
_39.0.0 = Move(RET);
_29 = _17.0.0;
_43.3 = Move(_32.1);
_16.0 = _32.2 as i128;
_39.2 = _21;
_25 = Move(_17.0.1);
_34.3 = [_30.fld1,_1,_16.2.fld1,_1,_30.fld1,_30.fld1];
_3 = _30.fld4 as u8;
match _30.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb15,
5 => bb6,
6 => bb16,
9223372036854775807 => bb19,
_ => bb18
}
}
bb18 = {
Return()
}
bb19 = {
_44 = _32.2;
_43.2 = '\u{d0154}' as u16;
_30.fld2 = _21;
_22.1 = [_16.2.fld4,_16.2.fld4,_30.fld4,_5,_30.fld4,_5];
_10 = [_44,_44];
_35 = '\u{29848}';
_48.0 = _35;
_47 = (Move(_34.0), _17.0.0, _14, _34.3);
_30.fld1 = _1;
_13 = _12;
_47.0.1 = &_20;
_23 = core::ptr::addr_of!(_10);
match _30.fld5 {
0 => bb20,
2 => bb22,
3 => bb23,
1 => bb25,
_ => bb24
}
}
bb20 = {
Return()
}
bb21 = {
_10 = [49008_u16,26307_u16];
_8 = 97788262812182407781380786939453502031_i128;
_6 = (-19682_i16) as u128;
_1 = _3 as u64;
_11 = 9223372036854775807_isize as f64;
Goto(bb2)
}
bb22 = {
Return()
}
bb23 = {
_10 = [49008_u16,26307_u16];
_8 = 97788262812182407781380786939453502031_i128;
_6 = (-19682_i16) as u128;
_1 = _3 as u64;
_11 = 9223372036854775807_isize as f64;
Goto(bb2)
}
bb24 = {
_16.0 = _8 + _8;
_2 = !_16.2.fld6;
_16.2.fld3 = -_11;
_8 = _16.0;
_13 = _12;
_14 = _6;
_8 = -_16.0;
_22.1 = [_16.2.fld4,_5,_5,_16.2.fld4,_5,_5];
_1 = _16.2.fld1 << _8;
_26 = _16.2.fld2 * _16.2.fld2;
_22.0 = core::ptr::addr_of!(_1);
_21 = _26;
_24 = -(-1327019512_i32);
_30 = Move(_16.2);
_16.2 = Adt20 { fld0: _9,fld1: _1,fld2: _26,fld3: _11,fld4: _30.fld4,fld5: _3,fld6: _2 };
_30.fld1 = _16.2.fld3 as u64;
_28 = (*_20) as isize;
_19 = _4 * _4;
_12 = !_13;
_4 = !_19;
_16.1 = &_4;
_30.fld1 = !_16.2.fld1;
_29 = _17.0.0;
_30.fld3 = _4 as f64;
_22.1 = [_30.fld4,_16.2.fld4,_16.2.fld4,_5,_16.2.fld4,_5];
Goto(bb15)
}
bb25 = {
_49 = core::ptr::addr_of_mut!(_13);
_49 = core::ptr::addr_of_mut!(_31);
_45.2.fld4 = !_5;
_20 = &_45.2.fld0;
_23 = core::ptr::addr_of!((*_23));
_40 = Move(_39.0.0);
_39.0.1 = _32.0;
(*_23) = [_44,_44];
_41 = _7;
_45.2 = Move(_16.2);
_30.fld3 = _11 * _45.2.fld3;
_45.1 = Move(_16.1);
_48.0 = _35;
(*_23) = [_43.2,_32.2];
Call(_45.0 = core::intrinsics::transmute(_8), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_16.2.fld4 = _30.fld4;
_16.2.fld1 = _1 | _30.fld1;
_30.fld4 = _5;
_16.2 = Move(_30);
_30.fld0 = !_47.0.0;
_16.2.fld2 = _9 as isize;
_48.3 = _12 ^ (*_49);
_32.0 = _16.2.fld6;
_47.2 = _14 & _34.2;
_35 = _48.0;
_34.1 = _8 as f32;
_34.1 = _17.0.0;
_22.1 = [_5,_45.2.fld4,_16.2.fld4,_45.2.fld4,_16.2.fld4,_16.2.fld4];
_31 = !_48.3;
_47.0.1 = &_20;
_38 = _45.2.fld2;
_30 = Adt20 { fld0: _16.2.fld0,fld1: _1,fld2: _45.2.fld2,fld3: _16.2.fld3,fld4: _45.2.fld4,fld5: _45.2.fld5,fld6: _39.0.1 };
match _45.2.fld5 {
0 => bb15,
1 => bb28,
_ => bb27
}
}
bb27 = {
_10 = [(*_18),_32.2];
(*_18) = _24 as u16;
_17.0.1 = &_32;
_34.2 = !_6;
_30.fld0 = !_34.0.0;
_42 = -_39.2;
_15 = [_13,_31,_12,_31,_12,_12];
_30.fld4 = !_5;
RET = Move(_18);
_16.2.fld6 = _24 as u32;
_39.0.0 = Move(RET);
_29 = _17.0.0;
_43.3 = Move(_32.1);
_16.0 = _32.2 as i128;
_39.2 = _21;
_25 = Move(_17.0.1);
_34.3 = [_30.fld1,_1,_16.2.fld1,_1,_30.fld1,_30.fld1];
_3 = _30.fld4 as u8;
match _30.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb15,
5 => bb6,
6 => bb16,
9223372036854775807 => bb19,
_ => bb18
}
}
bb28 = {
_33 = _48.0;
_34.2 = _24 as u128;
_27 = _14 as isize;
_32.0 = _30.fld6;
RET = Move(_40);
_20 = &_9;
_43.1 = Move(_49);
_4 = !_19;
_34.0.1 = &_20;
_43.2 = _32.2 << _5;
_45.2.fld2 = _24 as isize;
_51.1 = (_30.fld6, Move(_43.1), _32.2, Move(_43.1));
_51.2 = _47.1;
_9 = !_47.0.0;
_45.1 = &_48.1;
_48.0 = _33;
_15 = [_48.3,_12,_31,_48.3,_13,_31];
_49 = core::ptr::addr_of_mut!(_12);
_30.fld5 = _16.2.fld5 / _16.2.fld5;
Goto(bb29)
}
bb29 = {
Call(_59 = dump_var(0_usize, 27_usize, Move(_27), 38_usize, Move(_38), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_59 = dump_var(0_usize, 8_usize, Move(_8), 9_usize, Move(_9), 33_usize, Move(_33), 5_usize, Move(_5)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_59 = dump_var(0_usize, 35_usize, Move(_35), 1_usize, Move(_1), 15_usize, Move(_15), 42_usize, Move(_42)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_59 = dump_var(0_usize, 24_usize, Move(_24), 60_usize, _60, 60_usize, _60, 60_usize, _60), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: &'static usize,mut _2: i64,mut _3: u8,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: i64,mut _8: u64,mut _9: u8,mut _10: usize) -> u128 {
mir! {
type RET = u128;
let _11: i128;
let _12: &'static Adt28;
let _13: f64;
let _14: isize;
let _15: isize;
let _16: i64;
let _17: f64;
let _18: &'static *const *mut Adt20;
let _19: ((usize, &'static &'static usize), f32, u128, [u64; 6]);
let _20: (usize, &'static &'static usize);
let _21: [u8; 6];
let _22: (u16, i32, char, f32);
let _23: i8;
let _24: char;
let _25: f32;
let _26: [i16; 6];
let _27: &'static i32;
let _28: [u64; 6];
let _29: i8;
let _30: u32;
let _31: u64;
let _32: ();
let _33: ();
{
RET = !261079169934741834092596675541859471873_u128;
_1 = &_5;
_5 = RET as usize;
RET = 39130_u16 as u128;
_6 = !_5;
_5 = _4;
RET = 230716638463156250940551928978913810622_u128 * 338667341469300046970747506301215458770_u128;
_5 = _10;
_2 = !_7;
_4 = RET as usize;
RET = _9 as u128;
RET = !218472026645305121274567744233390518575_u128;
_4 = _5 + _10;
_4 = _6 >> _5;
_1 = &_5;
_11 = _4 as i128;
_10 = _5;
_2 = _7 + _7;
RET = 324876601963821814794644587445452421928_u128;
match _9 {
0 => bb1,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
1 => bb8,
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
_11 = _8 as i128;
_11 = RET as i128;
RET = 20832618631149004292059730135309330975_u128;
_11 = (-25902293132650862999624780346492693844_i128) + (-59345877570324789227735704109877150969_i128);
_10 = !_5;
_9 = (-109_i8) as u8;
_1 = &_4;
_6 = !(*_1);
_10 = !_4;
_1 = &_4;
_6 = 4215380656_u32 as usize;
_8 = 1988260973634926642_u64 << _9;
Call(_7 = fn2(_4, _10, _8, _8, _5, _10, _8, _2, _2, _11, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = (*_1) << (*_1);
_14 = (-9223372036854775808_isize);
_9 = !_3;
_17 = _11 as f64;
_16 = !_2;
_13 = 1045409767_u32 as f64;
_15 = -_14;
Goto(bb10)
}
bb10 = {
_14 = _15;
_9 = 43_i8 as u8;
_5 = 38_i8 as usize;
_2 = _7;
RET = 84005489086886293513173643761981992620_u128;
_2 = true as i64;
_3 = _9;
_2 = -_7;
_8 = _15 as u64;
_14 = !_15;
_13 = _3 as f64;
_9 = _3;
_6 = _4 & (*_1);
_4 = _6 << _7;
_15 = _14;
Call(_11 = core::intrinsics::bswap((-124919578150077816894527363456247738554_i128)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9 = _3;
_19.0.1 = &_1;
_15 = _14 << _2;
_13 = -_17;
_9 = !_3;
_20.1 = &_1;
_20.1 = Move(_19.0.1);
_19.0.0 = _4 ^ _10;
_9 = _3 >> _19.0.0;
_15 = _14;
Goto(bb12)
}
bb12 = {
_20.1 = &_1;
_20.0 = _19.0.0;
_22.2 = '\u{49f64}';
_19.2 = 441638596_u32 as u128;
_19.0.1 = &_1;
_22.1 = 1763662973_i32 * (-2015865992_i32);
_6 = _19.0.0 - _19.0.0;
_22.1 = _2 as i32;
match RET {
0 => bb11,
1 => bb9,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
84005489086886293513173643761981992620 => bb19,
_ => bb18
}
}
bb13 = {
_9 = _3;
_19.0.1 = &_1;
_15 = _14 << _2;
_13 = -_17;
_9 = !_3;
_20.1 = &_1;
_20.1 = Move(_19.0.1);
_19.0.0 = _4 ^ _10;
_9 = _3 >> _19.0.0;
_15 = _14;
Goto(bb12)
}
bb14 = {
_14 = _15;
_9 = 43_i8 as u8;
_5 = 38_i8 as usize;
_2 = _7;
RET = 84005489086886293513173643761981992620_u128;
_2 = true as i64;
_3 = _9;
_2 = -_7;
_8 = _15 as u64;
_14 = !_15;
_13 = _3 as f64;
_9 = _3;
_6 = _4 & (*_1);
_4 = _6 << _7;
_15 = _14;
Call(_11 = core::intrinsics::bswap((-124919578150077816894527363456247738554_i128)), ReturnTo(bb11), UnwindUnreachable())
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
_9 = !_3;
_1 = &_20.0;
_19.1 = _11 as f32;
_19.3 = [_8,_8,_8,_8,_8,_8];
_15 = _6 as isize;
_23 = _13 as i8;
RET = !_19.2;
_11 = 65553173958736476912552589524993379042_i128 << _6;
_20.1 = &_1;
_28 = [_8,_8,_8,_8,_8,_8];
_22.1 = RET as i32;
_15 = -_14;
_29 = _23;
_22.3 = _22.1 as f32;
_25 = _19.1 - _19.1;
_19 = (Move(_20), _22.3, RET, _28);
Goto(bb20)
}
bb20 = {
Call(_32 = dump_var(1_usize, 2_usize, Move(_2), 10_usize, Move(_10), 28_usize, Move(_28), 5_usize, Move(_5)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(1_usize, 16_usize, Move(_16), 9_usize, Move(_9), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: usize,mut _2: usize,mut _3: u64,mut _4: u64,mut _5: usize,mut _6: usize,mut _7: u64,mut _8: i64,mut _9: i64,mut _10: i128,mut _11: i64) -> i64 {
mir! {
type RET = i64;
let _12: f64;
let _13: bool;
let _14: bool;
let _15: Adt29;
let _16: isize;
let _17: f64;
let _18: [i16; 5];
let _19: f64;
let _20: i8;
let _21: [u16; 2];
let _22: (i64,);
let _23: *mut u16;
let _24: (i128, &'static i8, Adt20);
let _25: ((usize, &'static &'static usize), f32, u128, [u64; 6]);
let _26: (f32, &'static (u32, *mut bool, u16, *mut bool));
let _27: u16;
let _28: &'static [usize; 5];
let _29: *mut bool;
let _30: i64;
let _31: f32;
let _32: u32;
let _33: u8;
let _34: [usize; 8];
let _35: f64;
let _36: f32;
let _37: &'static [i16; 5];
let _38: i32;
let _39: ();
let _40: ();
{
_4 = _3 + _7;
_6 = 120_i8 as usize;
RET = _8 >> _7;
_10 = !(-94783167213891893730400458424723032719_i128);
RET = _11 ^ _11;
_1 = _5 << _7;
RET = _4 as i64;
_9 = RET;
RET = -_11;
_2 = _1 ^ _5;
RET = _8;
_6 = _1 | _2;
_13 = false;
_1 = _6 << RET;
_12 = 1092144738_i32 as f64;
_4 = _9 as u64;
_6 = _1 + _1;
_14 = !_13;
_12 = _8 as f64;
_10 = 142824615697925376913975346059477755703_i128;
_10 = -(-50866445377047960335901316759091388497_i128);
_4 = _3 + _7;
_1 = !_6;
_8 = _9 ^ RET;
_6 = 116_isize as usize;
_1 = _2;
_14 = !_13;
RET = _11;
Call(_15 = fn3(_12, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = Field::<Adt20>(Variant(_15, 1), 2).fld1 & _4;
_3 = _4 + _4;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld4 = (-8168_i16) << Field::<u32>(Variant(_15, 1), 0);
RET = -_8;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld0 = _2 + _1;
SetDiscriminant(_15, 0);
_16 = 7052_i16 as isize;
_4 = (-1622142326_i32) as u64;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = 244_u8 as i64;
RET = _11 >> _11;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _8;
RET = !Field::<(i64,)>(Variant(_15, 0), 1).0;
_9 = Field::<(i64,)>(Variant(_15, 0), 1).0;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (RET,);
_3 = _4 + _4;
_14 = _13;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_8,);
_18 = [12883_i16,(-20004_i16),(-6621_i16),12766_i16,(-25676_i16)];
place!(Field::<u8>(Variant(_15, 0), 0)) = !159_u8;
Goto(bb2)
}
bb2 = {
_7 = _3 - _4;
place!(Field::<u16>(Variant(_15, 0), 2)) = (-7209_i16) as u16;
RET = !_9;
place!(Field::<u8>(Variant(_15, 0), 0)) = 205_u8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = !_8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = RET;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_9,);
_20 = !(-71_i8);
_10 = 132335983585606839855687844070564326627_i128 >> _4;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _9;
_7 = (-27412_i16) as u64;
_18 = [22971_i16,26460_i16,(-14648_i16),(-20328_i16),(-9730_i16)];
_8 = (-8548_i16) as i64;
_7 = _3;
Call(_20 = core::intrinsics::transmute(Field::<u8>(Variant(_15, 0), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = _1 + _6;
_14 = _13 ^ _13;
_22 = Field::<(i64,)>(Variant(_15, 0), 1);
_13 = _14;
_1 = _6;
_12 = 1265374388_u32 as f64;
_5 = _2 + _2;
_22.0 = _9;
_4 = Field::<u16>(Variant(_15, 0), 2) as u64;
Goto(bb4)
}
bb4 = {
_1 = _2 | _5;
place!(Field::<u16>(Variant(_15, 0), 2)) = !37614_u16;
_25.2 = _10 as u128;
_22.0 = Field::<(i64,)>(Variant(_15, 0), 1).0 & RET;
_25.0.0 = _5;
_4 = !_7;
_24.2.fld5 = !Field::<u8>(Variant(_15, 0), 0);
_17 = -_12;
_22.0 = !_11;
_13 = _25.0.0 <= _1;
_25.2 = 248374870666962110620561897972014371681_u128 | 88250298039478886540415452348886211105_u128;
_24.0 = _7 as i128;
_5 = (-1773414474_i32) as usize;
_24.2.fld5 = Field::<u8>(Variant(_15, 0), 0);
_24.2.fld6 = 899848048_u32;
_25.1 = Field::<u16>(Variant(_15, 0), 2) as f32;
_23 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_15, 0), 2)));
RET = -_9;
_24.2.fld5 = Field::<u8>(Variant(_15, 0), 0) - Field::<u8>(Variant(_15, 0), 0);
_24.2.fld2 = _16;
_24.2.fld6 = 4237362927_u32 & 3088002712_u32;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (RET,);
_24.2.fld1 = _4;
_18 = [(-6426_i16),(-29482_i16),(-13851_i16),29276_i16,32533_i16];
_25.3 = [_4,_24.2.fld1,_3,_4,_3,_3];
_24.2.fld5 = _24.2.fld6 as u8;
SetDiscriminant(_15, 0);
place!(Field::<u16>(Variant(_15, 0), 2)) = 1657_u16 >> _1;
_24.1 = &_20;
Goto(bb5)
}
bb5 = {
_24.2 = Adt20 { fld0: _2,fld1: _7,fld2: _16,fld3: _17,fld4: 19350_i16,fld5: 225_u8,fld6: 715112370_u32 };
_22 = (_11,);
place!(Field::<u8>(Variant(_15, 0), 0)) = '\u{faa7f}' as u8;
_13 = _14;
_23 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_15, 0), 2)));
_26.0 = _24.2.fld4 as f32;
_25.3 = [_24.2.fld1,_7,_24.2.fld1,_24.2.fld1,_3,_4];
_21 = [(*_23),Field::<u16>(Variant(_15, 0), 2)];
match _24.2.fld4 {
0 => bb6,
1 => bb7,
2 => bb8,
19350 => bb10,
_ => bb9
}
}
bb6 = {
_1 = _2 | _5;
place!(Field::<u16>(Variant(_15, 0), 2)) = !37614_u16;
_25.2 = _10 as u128;
_22.0 = Field::<(i64,)>(Variant(_15, 0), 1).0 & RET;
_25.0.0 = _5;
_4 = !_7;
_24.2.fld5 = !Field::<u8>(Variant(_15, 0), 0);
_17 = -_12;
_22.0 = !_11;
_13 = _25.0.0 <= _1;
_25.2 = 248374870666962110620561897972014371681_u128 | 88250298039478886540415452348886211105_u128;
_24.0 = _7 as i128;
_5 = (-1773414474_i32) as usize;
_24.2.fld5 = Field::<u8>(Variant(_15, 0), 0);
_24.2.fld6 = 899848048_u32;
_25.1 = Field::<u16>(Variant(_15, 0), 2) as f32;
_23 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_15, 0), 2)));
RET = -_9;
_24.2.fld5 = Field::<u8>(Variant(_15, 0), 0) - Field::<u8>(Variant(_15, 0), 0);
_24.2.fld2 = _16;
_24.2.fld6 = 4237362927_u32 & 3088002712_u32;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (RET,);
_24.2.fld1 = _4;
_18 = [(-6426_i16),(-29482_i16),(-13851_i16),29276_i16,32533_i16];
_25.3 = [_4,_24.2.fld1,_3,_4,_3,_3];
_24.2.fld5 = _24.2.fld6 as u8;
SetDiscriminant(_15, 0);
place!(Field::<u16>(Variant(_15, 0), 2)) = 1657_u16 >> _1;
_24.1 = &_20;
Goto(bb5)
}
bb7 = {
_2 = _1 + _6;
_14 = _13 ^ _13;
_22 = Field::<(i64,)>(Variant(_15, 0), 1);
_13 = _14;
_1 = _6;
_12 = 1265374388_u32 as f64;
_5 = _2 + _2;
_22.0 = _9;
_4 = Field::<u16>(Variant(_15, 0), 2) as u64;
Goto(bb4)
}
bb8 = {
_7 = _3 - _4;
place!(Field::<u16>(Variant(_15, 0), 2)) = (-7209_i16) as u16;
RET = !_9;
place!(Field::<u8>(Variant(_15, 0), 0)) = 205_u8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = !_8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = RET;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_9,);
_20 = !(-71_i8);
_10 = 132335983585606839855687844070564326627_i128 >> _4;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _9;
_7 = (-27412_i16) as u64;
_18 = [22971_i16,26460_i16,(-14648_i16),(-20328_i16),(-9730_i16)];
_8 = (-8548_i16) as i64;
_7 = _3;
Call(_20 = core::intrinsics::transmute(Field::<u8>(Variant(_15, 0), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_7 = Field::<Adt20>(Variant(_15, 1), 2).fld1 & _4;
_3 = _4 + _4;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld4 = (-8168_i16) << Field::<u32>(Variant(_15, 1), 0);
RET = -_8;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld0 = _2 + _1;
SetDiscriminant(_15, 0);
_16 = 7052_i16 as isize;
_4 = (-1622142326_i32) as u64;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = 244_u8 as i64;
RET = _11 >> _11;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _8;
RET = !Field::<(i64,)>(Variant(_15, 0), 1).0;
_9 = Field::<(i64,)>(Variant(_15, 0), 1).0;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (RET,);
_3 = _4 + _4;
_14 = _13;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_8,);
_18 = [12883_i16,(-20004_i16),(-6621_i16),12766_i16,(-25676_i16)];
place!(Field::<u8>(Variant(_15, 0), 0)) = !159_u8;
Goto(bb2)
}
bb10 = {
_24.2.fld2 = _16 - _16;
_14 = _13;
match _24.2.fld4 {
0 => bb8,
19350 => bb12,
_ => bb11
}
}
bb11 = {
_24.2 = Adt20 { fld0: _2,fld1: _7,fld2: _16,fld3: _17,fld4: 19350_i16,fld5: 225_u8,fld6: 715112370_u32 };
_22 = (_11,);
place!(Field::<u8>(Variant(_15, 0), 0)) = '\u{faa7f}' as u8;
_13 = _14;
_23 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_15, 0), 2)));
_26.0 = _24.2.fld4 as f32;
_25.3 = [_24.2.fld1,_7,_24.2.fld1,_24.2.fld1,_3,_4];
_21 = [(*_23),Field::<u16>(Variant(_15, 0), 2)];
match _24.2.fld4 {
0 => bb6,
1 => bb7,
2 => bb8,
19350 => bb10,
_ => bb9
}
}
bb12 = {
_1 = _25.0.0 >> _24.2.fld6;
_6 = _1;
_30 = 1250515195_i32 as i64;
_5 = !_6;
_19 = -_12;
_24.2.fld6 = _25.2 as u32;
_29 = core::ptr::addr_of_mut!(_13);
match _24.2.fld4 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
19350 => bb19,
_ => bb18
}
}
bb13 = {
_2 = _1 + _6;
_14 = _13 ^ _13;
_22 = Field::<(i64,)>(Variant(_15, 0), 1);
_13 = _14;
_1 = _6;
_12 = 1265374388_u32 as f64;
_5 = _2 + _2;
_22.0 = _9;
_4 = Field::<u16>(Variant(_15, 0), 2) as u64;
Goto(bb4)
}
bb14 = {
_24.2.fld2 = _16 - _16;
_14 = _13;
match _24.2.fld4 {
0 => bb8,
19350 => bb12,
_ => bb11
}
}
bb15 = {
_7 = Field::<Adt20>(Variant(_15, 1), 2).fld1 & _4;
_3 = _4 + _4;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld4 = (-8168_i16) << Field::<u32>(Variant(_15, 1), 0);
RET = -_8;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld0 = _2 + _1;
SetDiscriminant(_15, 0);
_16 = 7052_i16 as isize;
_4 = (-1622142326_i32) as u64;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = 244_u8 as i64;
RET = _11 >> _11;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _8;
RET = !Field::<(i64,)>(Variant(_15, 0), 1).0;
_9 = Field::<(i64,)>(Variant(_15, 0), 1).0;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (RET,);
_3 = _4 + _4;
_14 = _13;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_8,);
_18 = [12883_i16,(-20004_i16),(-6621_i16),12766_i16,(-25676_i16)];
place!(Field::<u8>(Variant(_15, 0), 0)) = !159_u8;
Goto(bb2)
}
bb16 = {
_7 = _3 - _4;
place!(Field::<u16>(Variant(_15, 0), 2)) = (-7209_i16) as u16;
RET = !_9;
place!(Field::<u8>(Variant(_15, 0), 0)) = 205_u8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = !_8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = RET;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_9,);
_20 = !(-71_i8);
_10 = 132335983585606839855687844070564326627_i128 >> _4;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _9;
_7 = (-27412_i16) as u64;
_18 = [22971_i16,26460_i16,(-14648_i16),(-20328_i16),(-9730_i16)];
_8 = (-8548_i16) as i64;
_7 = _3;
Call(_20 = core::intrinsics::transmute(Field::<u8>(Variant(_15, 0), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_7 = Field::<Adt20>(Variant(_15, 1), 2).fld1 & _4;
_3 = _4 + _4;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld4 = (-8168_i16) << Field::<u32>(Variant(_15, 1), 0);
RET = -_8;
place!(Field::<Adt20>(Variant(_15, 1), 2)).fld0 = _2 + _1;
SetDiscriminant(_15, 0);
_16 = 7052_i16 as isize;
_4 = (-1622142326_i32) as u64;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = 244_u8 as i64;
RET = _11 >> _11;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _8;
RET = !Field::<(i64,)>(Variant(_15, 0), 1).0;
_9 = Field::<(i64,)>(Variant(_15, 0), 1).0;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (RET,);
_3 = _4 + _4;
_14 = _13;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_8,);
_18 = [12883_i16,(-20004_i16),(-6621_i16),12766_i16,(-25676_i16)];
place!(Field::<u8>(Variant(_15, 0), 0)) = !159_u8;
Goto(bb2)
}
bb18 = {
_7 = _3 - _4;
place!(Field::<u16>(Variant(_15, 0), 2)) = (-7209_i16) as u16;
RET = !_9;
place!(Field::<u8>(Variant(_15, 0), 0)) = 205_u8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = !_8;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = RET;
place!(Field::<(i64,)>(Variant(_15, 0), 1)) = (_9,);
_20 = !(-71_i8);
_10 = 132335983585606839855687844070564326627_i128 >> _4;
place!(Field::<(i64,)>(Variant(_15, 0), 1)).0 = _9;
_7 = (-27412_i16) as u64;
_18 = [22971_i16,26460_i16,(-14648_i16),(-20328_i16),(-9730_i16)];
_8 = (-8548_i16) as i64;
_7 = _3;
Call(_20 = core::intrinsics::transmute(Field::<u8>(Variant(_15, 0), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_5 = _6;
(*_29) = _14;
_24.2.fld0 = _25.0.0 >> _5;
_6 = _7 as usize;
_25.2 = _26.0 as u128;
_24.2.fld0 = !_2;
_15 = Adt29::Variant0 { fld0: _24.2.fld5,fld1: _22,fld2: 3343_u16 };
_22.0 = _24.2.fld1 as i64;
_13 = !_14;
_10 = _24.0;
_32 = !_24.2.fld6;
_18 = [_24.2.fld4,_24.2.fld4,_24.2.fld4,_24.2.fld4,_24.2.fld4];
_22.0 = RET & RET;
_19 = _24.2.fld3 * _17;
_24.2.fld3 = _19;
_31 = _26.0;
_25.1 = _20 as f32;
_36 = _10 as f32;
_36 = _3 as f32;
Goto(bb20)
}
bb20 = {
Call(_39 = dump_var(2_usize, 16_usize, Move(_16), 5_usize, Move(_5), 7_usize, Move(_7), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(2_usize, 32_usize, Move(_32), 11_usize, Move(_11), 14_usize, Move(_14), 22_usize, Move(_22)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_39 = dump_var(2_usize, 18_usize, Move(_18), 10_usize, Move(_10), 40_usize, _40, 40_usize, _40), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: f64,mut _2: i64) -> Adt29 {
mir! {
type RET = Adt29;
let _3: Adt56;
let _4: [usize; 5];
let _5: (char, i16);
let _6: &'static (u32, *mut bool, u16, *mut bool);
let _7: [u64; 6];
let _8: (u32, *mut bool, u16, *mut bool);
let _9: (i128, *mut u16, i64, [isize; 8]);
let _10: f32;
let _11: (&'static usize,);
let _12: bool;
let _13: f32;
let _14: [i16; 5];
let _15: [usize; 5];
let _16: *const u64;
let _17: u64;
let _18: &'static i8;
let _19: Adt28;
let _20: u32;
let _21: &'static &'static i32;
let _22: (i128, &'static i8, Adt20);
let _23: Adt29;
let _24: bool;
let _25: f32;
let _26: &'static &'static usize;
let _27: char;
let _28: (*mut u16, u32);
let _29: Adt82;
let _30: &'static &'static usize;
let _31: char;
let _32: i8;
let _33: f64;
let _34: isize;
let _35: *mut bool;
let _36: (usize, &'static &'static usize);
let _37: [i16; 5];
let _38: *mut bool;
let _39: [i128; 3];
let _40: (*mut u16, u32);
let _41: *mut f32;
let _42: bool;
let _43: &'static (u32, *mut bool, u16, *mut bool);
let _44: (i128, *mut u16, i64, [isize; 8]);
let _45: [i32; 1];
let _46: ();
let _47: ();
{
_1 = 34_u8 as f64;
_1 = 316348133985854490621776583168850000855_u128 as f64;
_2 = (-6692814357798042352_i64) >> (-1214822807_i32);
_2 = 8906936402956779435_i64;
_2 = 8559663969454183664_i64;
_1 = (-2083413758_i32) as f64;
_4 = [4969850630307375047_usize,9440287278336994194_usize,2_usize,12772049506919170674_usize,3879730742185586325_usize];
_1 = 17168855665240823402_u64 as f64;
_1 = 10198659387888663141_u64 as f64;
_2 = 2829430300530368654_i64 - 2254964253605412819_i64;
_5 = ('\u{30da0}', (-27251_i16));
_1 = _5.1 as f64;
_5.1 = (-21538_i16);
_5.0 = '\u{213b8}';
_2 = (-8139796811573433815_i64) | (-7912845750160257153_i64);
_5.0 = '\u{f3ee1}';
_5 = ('\u{59f19}', 22361_i16);
_5.1 = 6595_i16;
_7 = [10542940390482329451_u64,4760167412720404169_u64,7855294656492968611_u64,14681559710255002253_u64,5213915081738152346_u64,12494126833963251445_u64];
_2 = 1724549724754030149_i64;
Goto(bb1)
}
bb1 = {
_5 = ('\u{64908}', 7757_i16);
_5.0 = '\u{5f7e1}';
_7 = [10671139521140467076_u64,14661481947463458021_u64,2941516442953734085_u64,8008285515624094270_u64,11594463899228838658_u64,4799302520367538035_u64];
_5.0 = '\u{bec58}';
_8.2 = !36375_u16;
_9.2 = _2;
_9.3 = [31_isize,65_isize,(-9223372036854775808_isize),9223372036854775807_isize,23_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_6 = &_8;
_5.1 = 12117_i16;
_8.0 = _5.0 as u32;
_10 = (-107432186232678598409928319355220512633_i128) as f32;
_4 = [1_usize,14510740102008162177_usize,0_usize,4491425044494230470_usize,4_usize];
_8.2 = !1250_u16;
_9.2 = _2 - _2;
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
_5.1 = (-16164_i16) << _8.0;
_9.0 = 57545856609628228692025203815467329223_i128;
_8.3 = core::ptr::addr_of_mut!(_12);
_5.0 = '\u{3687}';
_9.1 = core::ptr::addr_of_mut!(_8.2);
Goto(bb2)
}
bb2 = {
_8.2 = (-227424420_i32) as u16;
_5 = ('\u{2794}', 14078_i16);
_12 = !true;
_6 = &_8;
_16 = core::ptr::addr_of!(_17);
_12 = (*_6).2 != (*_6).2;
Call(_13 = fn4(Move(_9), _7, _2, _4, _5.1, _5.1, _5, _5, _4, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_16) = !10589726992448001584_u64;
_16 = core::ptr::addr_of!((*_16));
_9.1 = core::ptr::addr_of_mut!((*_6).2);
_6 = &_8;
(*_16) = 2216015369057707467_u64 - 1924361615903481711_u64;
_16 = core::ptr::addr_of!(_17);
_8.2 = _1 as u16;
_9.3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-41_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_17 = 2653166422810953459_u64 + 13195623772277804336_u64;
_6 = &_8;
_8.3 = core::ptr::addr_of_mut!(_12);
_13 = _5.1 as f32;
_12 = !false;
_6 = &_8;
(*_16) = !15346835179520728766_u64;
_10 = -_13;
_16 = core::ptr::addr_of!((*_16));
_8.0 = !3403134425_u32;
_15 = [2_usize,17921708107676080928_usize,6_usize,14580215514441816417_usize,7_usize];
Goto(bb4)
}
bb4 = {
_13 = -_10;
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
_12 = _2 < _2;
_9.2 = _5.1 as i64;
_7 = [(*_16),(*_16),(*_16),(*_16),_17,_17];
_9.0 = 35438267818060902172652961744201012478_i128 + (-42167141290711531845509951005546181457_i128);
_8.2 = 42115_u16 - 17331_u16;
_7 = [_17,_17,_17,(*_16),_17,_17];
_12 = false;
_15 = _4;
Goto(bb5)
}
bb5 = {
_8.2 = 50941_u16 | 55687_u16;
_17 = 14366892161263747789_u64;
_17 = 47444645399852277667718325208146267705_u128 as u64;
_4 = [0_usize,14969262069569429469_usize,1_usize,7_usize,8589339023879723120_usize];
Goto(bb6)
}
bb6 = {
_12 = !false;
_7 = [_17,_17,(*_16),_17,_17,(*_16)];
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
_2 = _8.0 as i64;
_8.2 = 29783_u16 & 37823_u16;
_15 = [2_usize,14697116114028064666_usize,18365986147674361645_usize,4_usize,14968636453007919604_usize];
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
(*_16) = (-46_i8) as u64;
_8.3 = core::ptr::addr_of_mut!(_12);
_5.1 = -(-6357_i16);
_1 = 109_u8 as f64;
_8.0 = 675433588_i32 as u32;
_10 = _13 * _13;
_5.1 = 21550_i16 * (-7240_i16);
_8.1 = Move(_8.3);
_17 = 11752438578743323719_u64 + 12215230411146538419_u64;
_13 = _10 + _10;
_7 = [(*_16),_17,(*_16),(*_16),(*_16),(*_16)];
_1 = 2_i8 as f64;
_22.0 = -_9.0;
_25 = _13 - _13;
_22.2.fld1 = 64175009188066419339023705362910968947_u128 as u64;
_22.2.fld3 = -_1;
_5 = ('\u{77321}', 9992_i16);
_10 = _13 * _25;
Goto(bb7)
}
bb7 = {
_22.2.fld3 = _5.1 as f64;
match _5.1 {
0 => bb6,
1 => bb2,
2 => bb4,
3 => bb8,
4 => bb9,
5 => bb10,
9992 => bb12,
_ => bb11
}
}
bb8 = {
_5 = ('\u{64908}', 7757_i16);
_5.0 = '\u{5f7e1}';
_7 = [10671139521140467076_u64,14661481947463458021_u64,2941516442953734085_u64,8008285515624094270_u64,11594463899228838658_u64,4799302520367538035_u64];
_5.0 = '\u{bec58}';
_8.2 = !36375_u16;
_9.2 = _2;
_9.3 = [31_isize,65_isize,(-9223372036854775808_isize),9223372036854775807_isize,23_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_6 = &_8;
_5.1 = 12117_i16;
_8.0 = _5.0 as u32;
_10 = (-107432186232678598409928319355220512633_i128) as f32;
_4 = [1_usize,14510740102008162177_usize,0_usize,4491425044494230470_usize,4_usize];
_8.2 = !1250_u16;
_9.2 = _2 - _2;
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
_5.1 = (-16164_i16) << _8.0;
_9.0 = 57545856609628228692025203815467329223_i128;
_8.3 = core::ptr::addr_of_mut!(_12);
_5.0 = '\u{3687}';
_9.1 = core::ptr::addr_of_mut!(_8.2);
Goto(bb2)
}
bb9 = {
_8.2 = 50941_u16 | 55687_u16;
_17 = 14366892161263747789_u64;
_17 = 47444645399852277667718325208146267705_u128 as u64;
_4 = [0_usize,14969262069569429469_usize,1_usize,7_usize,8589339023879723120_usize];
Goto(bb6)
}
bb10 = {
_13 = -_10;
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
_12 = _2 < _2;
_9.2 = _5.1 as i64;
_7 = [(*_16),(*_16),(*_16),(*_16),_17,_17];
_9.0 = 35438267818060902172652961744201012478_i128 + (-42167141290711531845509951005546181457_i128);
_8.2 = 42115_u16 - 17331_u16;
_7 = [_17,_17,_17,(*_16),_17,_17];
_12 = false;
_15 = _4;
Goto(bb5)
}
bb11 = {
(*_16) = !10589726992448001584_u64;
_16 = core::ptr::addr_of!((*_16));
_9.1 = core::ptr::addr_of_mut!((*_6).2);
_6 = &_8;
(*_16) = 2216015369057707467_u64 - 1924361615903481711_u64;
_16 = core::ptr::addr_of!(_17);
_8.2 = _1 as u16;
_9.3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-41_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_17 = 2653166422810953459_u64 + 13195623772277804336_u64;
_6 = &_8;
_8.3 = core::ptr::addr_of_mut!(_12);
_13 = _5.1 as f32;
_12 = !false;
_6 = &_8;
(*_16) = !15346835179520728766_u64;
_10 = -_13;
_16 = core::ptr::addr_of!((*_16));
_8.0 = !3403134425_u32;
_15 = [2_usize,17921708107676080928_usize,6_usize,14580215514441816417_usize,7_usize];
Goto(bb4)
}
bb12 = {
(*_16) = _22.2.fld1 ^ _22.2.fld1;
_8.1 = core::ptr::addr_of_mut!(_12);
_5.1 = 17286_i16 & 28142_i16;
_20 = _8.0;
_13 = -_10;
_22.2.fld2 = _5.0 as isize;
_22.2.fld1 = _17;
_6 = &_8;
_27 = _5.0;
_11.0 = &_22.2.fld0;
_24 = _12;
_22.2.fld5 = _8.2 as u8;
_12 = _24;
_24 = !_12;
_22.0 = _10 as i128;
_27 = _5.0;
_26 = &_11.0;
_6 = &(*_6);
_9.2 = _2;
_9.2 = -_2;
(*_16) = _22.2.fld3 as u64;
_30 = &_11.0;
Goto(bb13)
}
bb13 = {
_22.2.fld1 = _22.2.fld2 as u64;
_17 = _22.2.fld1;
_14 = [_5.1,_5.1,_5.1,_5.1,_5.1];
_1 = -_22.2.fld3;
_5 = (_27, (-14977_i16));
_24 = !_12;
_22.2.fld0 = 34132313741830779424417251747760018578_u128 as usize;
_6 = &_8;
_4 = [_22.2.fld0,_22.2.fld0,_22.2.fld0,_22.2.fld0,_22.2.fld0];
Goto(bb14)
}
bb14 = {
_8.0 = _20 + _20;
_1 = (-26_i8) as f64;
match _5.1 {
340282366920938463463374607431768196479 => bb16,
_ => bb15
}
}
bb15 = {
_8.2 = 50941_u16 | 55687_u16;
_17 = 14366892161263747789_u64;
_17 = 47444645399852277667718325208146267705_u128 as u64;
_4 = [0_usize,14969262069569429469_usize,1_usize,7_usize,8589339023879723120_usize];
Goto(bb6)
}
bb16 = {
_28.1 = _22.2.fld3 as u32;
Goto(bb17)
}
bb17 = {
_32 = 27_i8 ^ 67_i8;
_22.2.fld4 = _5.1;
match _22.2.fld4 {
0 => bb2,
340282366920938463463374607431768196479 => bb19,
_ => bb18
}
}
bb18 = {
_28.1 = _22.2.fld3 as u32;
Goto(bb17)
}
bb19 = {
_15 = [_22.2.fld0,_22.2.fld0,_22.2.fld0,_22.2.fld0,_22.2.fld0];
_22.2.fld3 = -_1;
_22.1 = &_32;
_27 = _5.0;
_1 = _22.2.fld3 - _22.2.fld3;
(*_16) = _22.2.fld1;
_22.1 = &_32;
_32 = 89_i8 << _22.0;
_7 = [(*_16),_22.2.fld1,_17,(*_16),(*_16),_22.2.fld1];
_19 = Adt28::Variant1 { fld0: _25 };
_8.3 = core::ptr::addr_of_mut!(_24);
_20 = _12 as u32;
_33 = Field::<f32>(Variant(_19, 1), 0) as f64;
_28.1 = _8.0 * _8.0;
match _5.1 {
0 => bb9,
340282366920938463463374607431768196479 => bb20,
_ => bb13
}
}
bb20 = {
_8.3 = core::ptr::addr_of_mut!(_24);
_6 = &_8;
_15 = [_22.2.fld0,_22.2.fld0,_22.2.fld0,_22.2.fld0,_22.2.fld0];
Goto(bb21)
}
bb21 = {
_36.1 = &_11.0;
_17 = !_22.2.fld1;
_22.2.fld1 = (*_16) + (*_16);
_8.0 = !_28.1;
_35 = Move((*_6).3);
_5 = (_27, _22.2.fld4);
SetDiscriminant(_19, 1);
_22.2 = Adt20 { fld0: 5446620519117866869_usize,fld1: _17,fld2: 9223372036854775807_isize,fld3: _33,fld4: _5.1,fld5: 162_u8,fld6: _8.0 };
_36.0 = !_22.2.fld0;
_15 = _4;
place!(Field::<f32>(Variant(_19, 1), 0)) = _25;
Goto(bb22)
}
bb22 = {
_22.2.fld0 = _36.0;
_1 = _33 * _33;
_37 = _14;
_5.1 = !_22.2.fld4;
_35 = Move(_8.1);
_8.2 = 10396_u16 - 61287_u16;
_8.1 = Move(_35);
_1 = _8.0 as f64;
_36.0 = !_22.2.fld0;
_28.0 = core::ptr::addr_of_mut!(_8.2);
_39 = [_22.0,_22.0,_22.0];
_6 = &_8;
_10 = -Field::<f32>(Variant(_19, 1), 0);
Goto(bb23)
}
bb23 = {
RET = Adt29::Variant1 { fld0: _22.2.fld6,fld1: _5.0,fld2: Move(_22.2) };
_20 = Field::<Adt20>(Variant(RET, 1), 2).fld6;
place!(Field::<char>(Variant(RET, 1), 1)) = _27;
_40.0 = core::ptr::addr_of_mut!(_8.2);
place!(Field::<Adt20>(Variant(RET, 1), 2)).fld3 = _33 + _33;
place!(Field::<Adt20>(Variant(RET, 1), 2)).fld2 = -13_isize;
_2 = _32 as i64;
_8.3 = Move((*_6).1);
_28 = (Move(_40.0), _20);
_40.0 = core::ptr::addr_of_mut!(_8.2);
_40 = (Move(_9.1), (*_6).0);
place!(Field::<Adt20>(Variant(RET, 1), 2)).fld3 = _33 - _33;
place!(Field::<u32>(Variant(RET, 1), 0)) = _28.1 << _2;
_22.2.fld1 = (*_16);
_38 = Move(_8.3);
_9.1 = Move(_28.0);
_19 = Adt28::Variant3 { fld0: _5,fld1: Field::<Adt20>(Variant(RET, 1), 2).fld2 };
_8.3 = core::ptr::addr_of_mut!(_42);
_8.3 = Move(_38);
_22.2.fld3 = Field::<Adt20>(Variant(RET, 1), 2).fld3;
Goto(bb24)
}
bb24 = {
Call(_46 = dump_var(3_usize, 12_usize, Move(_12), 24_usize, Move(_24), 14_usize, Move(_14), 32_usize, Move(_32)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_46 = dump_var(3_usize, 27_usize, Move(_27), 37_usize, Move(_37), 5_usize, Move(_5), 47_usize, _47), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (i128, *mut u16, i64, [isize; 8]),mut _2: [u64; 6],mut _3: i64,mut _4: [usize; 5],mut _5: i16,mut _6: i16,mut _7: (char, i16),mut _8: (char, i16),mut _9: [usize; 5],mut _10: f64,mut _11: f64) -> f32 {
mir! {
type RET = f32;
let _12: (i64,);
let _13: (i128, *mut u16, i64, [isize; 8]);
let _14: (i128, *mut u16, i64, [isize; 8]);
let _15: u32;
let _16: &'static [usize; 5];
let _17: isize;
let _18: [u32; 5];
let _19: u8;
let _20: *const [u16; 2];
let _21: u128;
let _22: [i16; 6];
let _23: *const u64;
let _24: i64;
let _25: isize;
let _26: u128;
let _27: char;
let _28: isize;
let _29: *const *mut Adt20;
let _30: &'static &'static usize;
let _31: ();
let _32: ();
{
_4 = [11279389265068821436_usize,12072479413211585185_usize,5_usize,3319535027046297092_usize,10053088970020296464_usize];
_5 = _7.1;
_10 = _11 + _11;
_1.2 = _1.0 as i64;
RET = 29434_u16 as f32;
_1.0 = (-41956439335191305382755299003536045880_i128) << _5;
_8.0 = _7.0;
_8 = _7;
match _8.1 {
0 => bb1,
1 => bb2,
14078 => bb4,
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
_1.2 = -_3;
_1.3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,127_isize,9223372036854775807_isize];
_10 = _11 * _11;
_12 = (_3,);
_4 = [15731178740395716414_usize,3_usize,16010549125611908472_usize,4_usize,6_usize];
_13.1 = Move(_1.1);
_13.3 = [(-35_isize),9223372036854775807_isize,(-12_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,13_isize,103_isize];
_1.1 = Move(_13.1);
_12.0 = _1.0 as i64;
RET = 62687_u16 as f32;
_13.0 = _1.0 >> _3;
_7.0 = _8.0;
_14 = Move(_1);
_13.2 = _12.0 + _12.0;
match _5 {
0 => bb3,
1 => bb5,
2 => bb6,
14078 => bb8,
_ => bb7
}
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
_13.0 = RET as i128;
_11 = _10 + _10;
_17 = _7.1 as isize;
_6 = _12.0 as i16;
_2 = [9650461465973444645_u64,3223792313486878980_u64,12303173071476647846_u64,4679166788250701192_u64,4111245811414723363_u64,1499698870570419912_u64];
_8 = (_7.0, _7.1);
_19 = 1725561807_u32 as u8;
_10 = -_11;
_16 = &_9;
RET = 103_i8 as f32;
_12.0 = _13.2 & _13.2;
_5 = _6;
_2 = [11363292976029086245_u64,8454291650842674539_u64,3439884846983505354_u64,17225986157426730867_u64,10493199905581809556_u64,17624424376065353653_u64];
_24 = 273098091709190092231965494478750586629_u128 as i64;
_22 = [_5,_6,_8.1,_8.1,_7.1,_8.1];
_4 = (*_16);
_1.2 = _13.2;
_13.1 = Move(_14.1);
_9 = [8086992678344753609_usize,1_usize,10797194130573827595_usize,6_usize,4_usize];
Call(_13.1 = fn5(_7.1, _8.0, _13.3, _12.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14.1 = Move(_13.1);
_19 = !131_u8;
match _3 {
0 => bb1,
1 => bb8,
2 => bb10,
3 => bb11,
4 => bb12,
1724549724754030149 => bb14,
_ => bb13
}
}
bb10 = {
_13.0 = RET as i128;
_11 = _10 + _10;
_17 = _7.1 as isize;
_6 = _12.0 as i16;
_2 = [9650461465973444645_u64,3223792313486878980_u64,12303173071476647846_u64,4679166788250701192_u64,4111245811414723363_u64,1499698870570419912_u64];
_8 = (_7.0, _7.1);
_19 = 1725561807_u32 as u8;
_10 = -_11;
_16 = &_9;
RET = 103_i8 as f32;
_12.0 = _13.2 & _13.2;
_5 = _6;
_2 = [11363292976029086245_u64,8454291650842674539_u64,3439884846983505354_u64,17225986157426730867_u64,10493199905581809556_u64,17624424376065353653_u64];
_24 = 273098091709190092231965494478750586629_u128 as i64;
_22 = [_5,_6,_8.1,_8.1,_7.1,_8.1];
_4 = (*_16);
_1.2 = _13.2;
_13.1 = Move(_14.1);
_9 = [8086992678344753609_usize,1_usize,10797194130573827595_usize,6_usize,4_usize];
Call(_13.1 = fn5(_7.1, _8.0, _13.3, _12.0), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
_1.2 = -_3;
_1.3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,127_isize,9223372036854775807_isize];
_10 = _11 * _11;
_12 = (_3,);
_4 = [15731178740395716414_usize,3_usize,16010549125611908472_usize,4_usize,6_usize];
_13.1 = Move(_1.1);
_13.3 = [(-35_isize),9223372036854775807_isize,(-12_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,13_isize,103_isize];
_1.1 = Move(_13.1);
_12.0 = _1.0 as i64;
RET = 62687_u16 as f32;
_13.0 = _1.0 >> _3;
_7.0 = _8.0;
_14 = Move(_1);
_13.2 = _12.0 + _12.0;
match _5 {
0 => bb3,
1 => bb5,
2 => bb6,
14078 => bb8,
_ => bb7
}
}
bb13 = {
Return()
}
bb14 = {
_22 = [_5,_7.1,_7.1,_5,_6,_6];
_18 = [2977503781_u32,1353229747_u32,352532783_u32,656775275_u32,732740734_u32];
_13 = Move(_14);
_14.1 = Move(_13.1);
_25 = _17 - _17;
_25 = _8.0 as isize;
_13.1 = Move(_14.1);
_7.1 = _5;
_1.3 = _13.3;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(4_usize, 22_usize, Move(_22), 2_usize, Move(_2), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(4_usize, 25_usize, Move(_25), 9_usize, Move(_9), 4_usize, Move(_4), 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i16,mut _2: char,mut _3: [isize; 8],mut _4: i64) -> *mut u16 {
mir! {
type RET = *mut u16;
let _5: (u16, i32, char, f32);
let _6: isize;
let _7: ((*mut u16, u32), [bool; 6], isize);
let _8: *mut bool;
let _9: char;
let _10: *mut Adt20;
let _11: Adt82;
let _12: *mut Adt20;
let _13: i128;
let _14: isize;
let _15: *const *mut Adt20;
let _16: char;
let _17: i128;
let _18: (char, i16);
let _19: *const *mut Adt20;
let _20: f32;
let _21: char;
let _22: i32;
let _23: isize;
let _24: isize;
let _25: *mut *mut f32;
let _26: &'static &'static i32;
let _27: i16;
let _28: isize;
let _29: (&'static usize,);
let _30: (usize, &'static &'static usize);
let _31: u32;
let _32: i128;
let _33: i8;
let _34: (&'static usize,);
let _35: char;
let _36: (u32, *mut bool, u16, *mut bool);
let _37: (i128, *mut u16, i64, [isize; 8]);
let _38: ();
let _39: ();
{
_3 = [108_isize,(-97_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-64_isize),(-9223372036854775808_isize)];
RET = core::ptr::addr_of_mut!(_5.0);
_1 = 15524_i16 | 16763_i16;
_1 = 50_i8 as i16;
(*RET) = 41622_u16;
_5.3 = 113088253396054719836167808935650248397_i128 as f32;
_2 = '\u{fa682}';
(*RET) = 1285286519_i32 as u16;
_5.2 = _2;
_6 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
(*RET) = 17526_u16;
_5.1 = (-727315095_i32) * 1930101203_i32;
_5.3 = 2_usize as f32;
RET = core::ptr::addr_of_mut!(_5.0);
(*RET) = 28434_u16;
_2 = _5.2;
Goto(bb2)
}
bb2 = {
_5.3 = 26_i8 as f32;
_5.1 = 1937538648_i32 | 1818698222_i32;
_5.3 = 238_u8 as f32;
_5.0 = _1 as u16;
RET = core::ptr::addr_of_mut!((*RET));
_5.1 = 1760903147_i32 | 793604997_i32;
RET = core::ptr::addr_of_mut!((*RET));
_6 = -9223372036854775807_isize;
(*RET) = !42773_u16;
_5.1 = !1555981006_i32;
(*RET) = 44455_u16;
_7.0 = (Move(RET), 2462808816_u32);
_5.2 = _2;
_7.2 = _6;
_7.2 = !_6;
_7.0.0 = core::ptr::addr_of_mut!(_5.0);
RET = Move(_7.0.0);
Call(_5.0 = core::intrinsics::transmute(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7.0.0 = Move(RET);
_7.1 = [false,false,false,true,true,true];
_7.0.1 = 3281591889_u32;
_1 = -(-26819_i16);
RET = Move(_7.0.0);
_7.0.1 = !1030109694_u32;
_5.0 = 43618_u16 | 35897_u16;
_7.1 = [true,false,true,false,false,true];
_7.0.1 = 294530123_u32 | 522291489_u32;
_7.2 = _1 as isize;
_7.0 = (Move(RET), 53448738_u32);
Goto(bb4)
}
bb4 = {
_9 = _5.2;
_7.2 = _6 & _6;
_5.1 = _5.3 as i32;
RET = Move(_7.0.0);
_7.0.1 = _5.0 as u32;
_5.2 = _2;
_2 = _9;
_9 = _2;
_7.0 = (Move(RET), 345086003_u32);
_3 = [_6,_7.2,_7.2,_7.2,_6,_7.2,_7.2,_7.2];
_1 = !(-14917_i16);
_5.0 = 64720_u16;
RET = Move(_7.0.0);
_7.0.0 = core::ptr::addr_of_mut!(_5.0);
_3 = [_7.2,_7.2,_7.2,_6,_7.2,_7.2,_7.2,_6];
_4 = (-819797358666625253_i64);
_13 = (-83612838283208046373813469594311324730_i128) >> _7.2;
_5.0 = !58406_u16;
_9 = _2;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463462554810073101586203 => bb9,
_ => bb8
}
}
bb5 = {
_7.0.0 = Move(RET);
_7.1 = [false,false,false,true,true,true];
_7.0.1 = 3281591889_u32;
_1 = -(-26819_i16);
RET = Move(_7.0.0);
_7.0.1 = !1030109694_u32;
_5.0 = 43618_u16 | 35897_u16;
_7.1 = [true,false,true,false,false,true];
_7.0.1 = 294530123_u32 | 522291489_u32;
_7.2 = _1 as isize;
_7.0 = (Move(RET), 53448738_u32);
Goto(bb4)
}
bb6 = {
_5.3 = 26_i8 as f32;
_5.1 = 1937538648_i32 | 1818698222_i32;
_5.3 = 238_u8 as f32;
_5.0 = _1 as u16;
RET = core::ptr::addr_of_mut!((*RET));
_5.1 = 1760903147_i32 | 793604997_i32;
RET = core::ptr::addr_of_mut!((*RET));
_6 = -9223372036854775807_isize;
(*RET) = !42773_u16;
_5.1 = !1555981006_i32;
(*RET) = 44455_u16;
_7.0 = (Move(RET), 2462808816_u32);
_5.2 = _2;
_7.2 = _6;
_7.2 = !_6;
_7.0.0 = core::ptr::addr_of_mut!(_5.0);
RET = Move(_7.0.0);
Call(_5.0 = core::intrinsics::transmute(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
(*RET) = 17526_u16;
_5.1 = (-727315095_i32) * 1930101203_i32;
_5.3 = 2_usize as f32;
RET = core::ptr::addr_of_mut!(_5.0);
(*RET) = 28434_u16;
_2 = _5.2;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_7.1 = [false,true,false,true,true,false];
_6 = _5.3 as isize;
_5.0 = 39474_u16 | 33048_u16;
_9 = _2;
_7.0.0 = core::ptr::addr_of_mut!(_5.0);
_9 = _5.2;
_7.0 = (Move(RET), 1279287275_u32);
_14 = _1 as isize;
RET = core::ptr::addr_of_mut!(_5.0);
_6 = !_7.2;
_7.1 = [false,true,false,true,true,false];
_14 = !_6;
_2 = _5.2;
_6 = _7.2 * _7.2;
_5.1 = !2109589764_i32;
_6 = _7.2;
_3 = [_6,_7.2,_14,_6,_14,_14,_7.2,_7.2];
_7.0 = (Move(RET), 3502795267_u32);
_15 = core::ptr::addr_of!(_10);
Call(_11 = fn6(Move(_7.0.0), _7.1, _7.0.1, _14, _2, _7.1, _7.2, _6, _7.1, _7.1, _6, _4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = _5.0 as i64;
_3 = [Field::<isize>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 0), 2),Field::<isize>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 0), 2),Field::<isize>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 0), 2),_14,Field::<isize>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 0), 2),_7.2,Field::<isize>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 0), 2),Field::<isize>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 0), 2)];
place!(Field::<isize>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 0), 2)) = _7.2;
_9 = _2;
_10 = Move(Field::<*mut Adt20>(Variant(_11, 2), 1));
_9 = _2;
_18.1 = _13 as i16;
_1 = _18.1;
_14 = _6;
place!(Field::<i32>(Variant(_11, 2), 3)) = -_5.1;
_7.2 = _6 >> _1;
_5.2 = _2;
_19 = Move(_15);
_20 = _5.3 + _5.3;
_17 = !_13;
place!(Field::<u8>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 0), 0)) = 225_u8;
_21 = _5.2;
Goto(bb11)
}
bb11 = {
_18 = (_5.2, _1);
_19 = core::ptr::addr_of!(_12);
_7.0.0 = core::ptr::addr_of_mut!(_5.0);
_9 = _21;
_17 = _20 as i128;
SetDiscriminant(Field::<Adt62>(Variant(_11, 2), 2), 1);
_21 = _9;
place!(Field::<(u16, i32, char, f32)>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 0)) = _5;
_5 = Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0);
RET = core::ptr::addr_of_mut!(place!(Field::<(u32, *mut bool, u16, *mut bool)>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 4)).2);
_28 = 210834340991418520211822542622309653764_u128 as isize;
_12 = Move(_10);
place!(Field::<i32>(Variant(_11, 2), 3)) = _5.1 * _5.1;
_15 = core::ptr::addr_of!(place!(Field::<*mut Adt20>(Variant(_11, 2), 1)));
_18 = (_2, _1);
_5.1 = Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0).1 & Field::<i32>(Variant(_11, 2), 3);
RET = Move(_7.0.0);
place!(Field::<*mut Adt20>(Variant(_11, 2), 1)) = Move((*_19));
_5 = Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0);
_22 = _4 as i32;
place!(Field::<(u32, *mut bool, u16, *mut bool)>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 4)).1 = core::ptr::addr_of_mut!(place!(Field::<bool>(Variant(_11, 2), 0)));
match _7.0.1 {
3502795267 => bb13,
_ => bb12
}
}
bb12 = {
(*RET) = 17526_u16;
_5.1 = (-727315095_i32) * 1930101203_i32;
_5.3 = 2_usize as f32;
RET = core::ptr::addr_of_mut!(_5.0);
(*RET) = 28434_u16;
_2 = _5.2;
Goto(bb2)
}
bb13 = {
_24 = _7.2 ^ _7.2;
_18 = (Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0).2, _1);
_23 = _24;
_7.0.1 = 4199440820_u32 | 2365391522_u32;
_10 = Move(Field::<*mut Adt20>(Variant(_11, 2), 1));
_28 = !_23;
_24 = 1674148870676233267_usize as isize;
_5.1 = !Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0).1;
(*_19) = Move(_10);
(*_15) = Move((*_19));
_5.3 = 11471164454506512441_u64 as f32;
_22 = Field::<i32>(Variant(_11, 2), 3);
_7.0 = (Move(RET), 3637480790_u32);
RET = Move(_7.0.0);
_7.2 = _24 & _28;
_31 = _7.0.1 % _7.0.1;
place!(Field::<[u16; 2]>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 6)) = [_5.0,_5.0];
_31 = _7.0.1;
(*_19) = Move((*_15));
Call(_32 = core::intrinsics::transmute(_13), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7.1 = [Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0)];
_21 = _18.0;
_10 = Move(_12);
place!(Field::<(u16, i32, char, f32)>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 0)).1 = -_5.1;
_18.1 = !_1;
(*_19) = Move(_10);
_18 = (_9, _1);
_22 = _13 as i32;
place!(Field::<[bool; 6]>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 5)) = [Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0),Field::<bool>(Variant(_11, 2), 0)];
_16 = _5.2;
place!(Field::<(u32, *mut bool, u16, *mut bool)>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 4)).0 = 101_u8 as u32;
_30.0 = 0_usize - 10594623457497792185_usize;
_31 = 67_i8 as u32;
_29.0 = &_30.0;
_20 = Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0).1 as f32;
_2 = _5.2;
_36.1 = Move(Field::<(u32, *mut bool, u16, *mut bool)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 4).1);
_34.0 = &_30.0;
_27 = _30.0 as i16;
place!(Field::<bool>(Variant(_11, 2), 0)) = _18.1 <= _1;
place!(Field::<(u32, *mut bool, u16, *mut bool)>(Variant(place!(Field::<Adt62>(Variant(_11, 2), 2)), 1), 4)).2 = 211_u8 as u16;
_19 = core::ptr::addr_of!(place!(Field::<*mut Adt20>(Variant(_11, 2), 1)));
_37 = (_13, Move(RET), _4, _3);
_36.2 = !Field::<(u16, i32, char, f32)>(Variant(Field::<Adt62>(Variant(_11, 2), 2), 1), 0).0;
RET = Move(_37.1);
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(5_usize, 6_usize, Move(_6), 17_usize, Move(_17), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(5_usize, 13_usize, Move(_13), 23_usize, Move(_23), 16_usize, Move(_16), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(5_usize, 24_usize, Move(_24), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *mut u16,mut _2: [bool; 6],mut _3: u32,mut _4: isize,mut _5: char,mut _6: [bool; 6],mut _7: isize,mut _8: isize,mut _9: [bool; 6],mut _10: [bool; 6],mut _11: isize,mut _12: i64) -> Adt82 {
mir! {
type RET = Adt82;
let _13: f32;
let _14: f64;
let _15: i8;
let _16: *const [u64; 6];
let _17: [u64; 6];
let _18: [u16; 2];
let _19: *mut *mut f32;
let _20: *const (char, i16);
let _21: *mut f32;
let _22: f32;
let _23: usize;
let _24: bool;
let _25: i128;
let _26: char;
let _27: (char, i16);
let _28: char;
let _29: u16;
let _30: ((usize, &'static &'static usize), f32, u128, [u64; 6]);
let _31: [u64; 2];
let _32: [i32; 1];
let _33: (isize,);
let _34: Adt56;
let _35: char;
let _36: u8;
let _37: &'static usize;
let _38: bool;
let _39: Adt46;
let _40: (&'static usize,);
let _41: i16;
let _42: (usize, &'static &'static usize);
let _43: i64;
let _44: bool;
let _45: [usize; 5];
let _46: ((i64,),);
let _47: [usize; 2];
let _48: u64;
let _49: bool;
let _50: i8;
let _51: isize;
let _52: char;
let _53: f64;
let _54: u128;
let _55: [i128; 3];
let _56: isize;
let _57: &'static [i16; 5];
let _58: (u16, i32, char, f32);
let _59: f32;
let _60: isize;
let _61: *mut (char, i8, usize, bool);
let _62: usize;
let _63: &'static &'static i32;
let _64: ((f32, &'static (u32, *mut bool, u16, *mut bool)),);
let _65: *mut u16;
let _66: *mut *mut f32;
let _67: isize;
let _68: [isize; 8];
let _69: isize;
let _70: f32;
let _71: bool;
let _72: char;
let _73: ((usize, &'static &'static usize), f32, u128, [u64; 6]);
let _74: Adt80;
let _75: &'static [i16; 5];
let _76: isize;
let _77: u32;
let _78: [isize; 8];
let _79: *mut *mut f32;
let _80: f32;
let _81: i8;
let _82: ();
let _83: ();
{
_9 = _10;
_4 = _8;
_4 = !_8;
_8 = -_7;
_7 = 15569647483117340380_u64 as isize;
_4 = _11 + _11;
_10 = _6;
_5 = '\u{d4bd9}';
Goto(bb1)
}
bb1 = {
_2 = _9;
_7 = _8;
_8 = _7 & _7;
_9 = [false,true,false,false,false,false];
_3 = !4272603020_u32;
_8 = _4 * _11;
_10 = [false,true,true,false,false,false];
_5 = '\u{d7b4b}';
_13 = 23870977138572908908893425261478111845_i128 as f32;
_9 = [false,false,false,false,true,true];
_5 = '\u{360af}';
_7 = _8;
Goto(bb2)
}
bb2 = {
_10 = [true,true,true,false,false,false];
_6 = [false,false,true,true,true,false];
_12 = 318021519665914092881096742223877024478_u128 as i64;
_4 = _8;
_10 = [false,true,false,true,true,false];
_12 = (-1921322136170915825_i64) + (-5672043092766099482_i64);
_11 = _7 ^ _4;
_14 = 5_usize as f64;
_3 = 14837785967286312628_u64 as u32;
_6 = [true,false,true,false,true,true];
_10 = [false,false,true,false,true,false];
_9 = [false,true,false,false,false,false];
_6 = [false,false,false,true,true,false];
_15 = (-56_i8) - (-83_i8);
_7 = _15 as isize;
_15 = 32583339142449847248702532124252467282_i128 as i8;
_12 = 4799330362828749964_i64 & (-8391288150547617097_i64);
_4 = -_8;
_12 = (-857718337189213427_i64) | (-7622837179155846770_i64);
_6 = [true,true,true,false,false,true];
_15 = -(-32_i8);
_4 = 13335_u16 as isize;
_5 = '\u{a473b}';
_17 = [3249398408437642023_u64,11438974820355897146_u64,8590338255013195717_u64,3556173684095398763_u64,6213579676670103220_u64,1941873855746961275_u64];
_2 = _6;
_11 = _7;
Goto(bb3)
}
bb3 = {
_6 = [false,false,false,true,false,false];
_6 = _2;
Goto(bb4)
}
bb4 = {
_8 = -_4;
_6 = _10;
_21 = core::ptr::addr_of_mut!(_13);
_12 = (-486883871713309989_i64);
_13 = (-10406970769108864012010976616319408601_i128) as f32;
_16 = core::ptr::addr_of!(_17);
_7 = _8;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463462887723560054901467 => bb6,
_ => bb5
}
}
bb5 = {
_2 = _9;
_7 = _8;
_8 = _7 & _7;
_9 = [false,true,false,false,false,false];
_3 = !4272603020_u32;
_8 = _4 * _11;
_10 = [false,true,true,false,false,false];
_5 = '\u{d7b4b}';
_13 = 23870977138572908908893425261478111845_i128 as f32;
_9 = [false,false,false,false,true,true];
_5 = '\u{360af}';
_7 = _8;
Goto(bb2)
}
bb6 = {
_10 = [false,false,false,true,true,false];
_18 = [21069_u16,40947_u16];
_10 = _9;
(*_16) = [7744638561529596631_u64,7610855053763838090_u64,739727265734741473_u64,14590180072539100313_u64,15795089395789338985_u64,15001582812172218402_u64];
_7 = _11;
_3 = 182_u8 as u32;
_22 = -_13;
_4 = 29_u8 as isize;
(*_21) = _22 - _22;
_11 = _8;
_23 = !7_usize;
_8 = 293326736000492365_u64 as isize;
_3 = !2324735447_u32;
_19 = core::ptr::addr_of_mut!(_21);
(*_21) = _22;
_2 = [false,true,false,false,true,false];
_12 = _4 as i64;
(*_16) = [2010521844058322874_u64,5842721718302917980_u64,234954893893680540_u64,4232586015758480198_u64,2853682984234223142_u64,900748133349801556_u64];
(*_19) = core::ptr::addr_of_mut!((*_21));
_13 = -_22;
_4 = _7 - _11;
_11 = _4;
_9 = _2;
_15 = _4 as i8;
_13 = _22;
_2 = [true,true,false,true,false,false];
Goto(bb7)
}
bb7 = {
_21 = core::ptr::addr_of_mut!(_13);
(*_19) = core::ptr::addr_of_mut!((*_21));
_19 = core::ptr::addr_of_mut!((*_19));
_11 = _7;
(*_19) = core::ptr::addr_of_mut!(_13);
_2 = [true,true,false,true,true,false];
_23 = (*_21) as usize;
_6 = [false,true,false,true,true,false];
_5 = '\u{940b6}';
_25 = _5 as i128;
_2 = [true,false,true,false,true,false];
_19 = core::ptr::addr_of_mut!((*_19));
Goto(bb8)
}
bb8 = {
_7 = !_11;
(*_21) = _22 - _22;
_15 = -43_i8;
(*_16) = [14536959790890196079_u64,8630308339268407197_u64,15290946051956257107_u64,1153107612278264448_u64,14573428201158355803_u64,13611847216308866089_u64];
_6 = _10;
_7 = _23 as isize;
_17 = [8185190458345895750_u64,8699609171179715860_u64,3211179918768016416_u64,10495661773069559390_u64,17639759235119661312_u64,9775413406373654930_u64];
(*_19) = core::ptr::addr_of_mut!((*_21));
_20 = core::ptr::addr_of!(_27);
(*_19) = core::ptr::addr_of_mut!((*_21));
_15 = _14 as i8;
_26 = _5;
_27.1 = !(-26698_i16);
_26 = _5;
_9 = [true,false,false,true,true,true];
Goto(bb9)
}
bb9 = {
(*_20) = (_5, 22535_i16);
_21 = core::ptr::addr_of_mut!(_30.1);
(*_20) = (_5, 755_i16);
_30.0.0 = !_23;
(*_16) = [16992764683505526630_u64,2029319963290099979_u64,15396824163372508859_u64,1326632967222498609_u64,1238555034657574903_u64,17260547495770598344_u64];
_5 = (*_20).0;
_18 = [37357_u16,10213_u16];
(*_20).1 = 4248_i16;
_21 = core::ptr::addr_of_mut!(_22);
Goto(bb10)
}
bb10 = {
_26 = _5;
_11 = _4;
_30.1 = _27.1 as f32;
_27 = (_5, 4734_i16);
_8 = _7 << _11;
_20 = core::ptr::addr_of!((*_20));
Call(_8 = fn7(Move(_16)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_20).0 = _26;
_27 = (_26, 16100_i16);
_27.1 = _12 as i16;
_22 = _15 as f32;
_24 = _15 != _15;
_3 = (*_20).0 as u32;
_5 = (*_20).0;
_28 = (*_20).0;
_30.2 = 3888345178983682884_u64 as u128;
_4 = _12 as isize;
_31 = [1115578224932168239_u64,5620301495339732698_u64];
(*_20).0 = _26;
_30.3 = [7769252262613013234_u64,10382841913539402613_u64,6065280345947566273_u64,3842192378938205492_u64,16611386790280790674_u64,11167068756893868215_u64];
(*_20).0 = _5;
(*_19) = core::ptr::addr_of_mut!(_13);
_27.1 = -23580_i16;
_30.1 = -_22;
_15 = !(-28_i8);
_19 = core::ptr::addr_of_mut!(_21);
_14 = _15 as f64;
_27 = (_26, 9523_i16);
_27.0 = _26;
_33 = (_8,);
_15 = !99_i8;
Goto(bb12)
}
bb12 = {
_18 = [22060_u16,65409_u16];
_30.0.0 = _5 as usize;
_15 = !(-71_i8);
_29 = _25 as u16;
_38 = !_24;
_16 = core::ptr::addr_of!(_30.3);
_39.fld2.fld3 = _14 * _14;
_8 = _11;
_35 = _5;
_30.2 = _3 as u128;
_39.fld2.fld5 = 22_u8 & 144_u8;
_39.fld2.fld0 = _15 as usize;
match (*_20).1 {
0 => bb7,
1 => bb4,
2 => bb13,
3 => bb14,
9523 => bb16,
_ => bb15
}
}
bb13 = {
_8 = -_4;
_6 = _10;
_21 = core::ptr::addr_of_mut!(_13);
_12 = (-486883871713309989_i64);
_13 = (-10406970769108864012010976616319408601_i128) as f32;
_16 = core::ptr::addr_of!(_17);
_7 = _8;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463462887723560054901467 => bb6,
_ => bb5
}
}
bb14 = {
_26 = _5;
_11 = _4;
_30.1 = _27.1 as f32;
_27 = (_5, 4734_i16);
_8 = _7 << _11;
_20 = core::ptr::addr_of!((*_20));
Call(_8 = fn7(Move(_16)), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_6 = [false,false,false,true,false,false];
_6 = _2;
Goto(bb4)
}
bb16 = {
_11 = !_8;
(*_20).1 = !(-1796_i16);
_22 = (*_21) - _13;
_8 = _3 as isize;
_16 = core::ptr::addr_of!(_17);
_39.fld2.fld2 = !_11;
(*_21) = -_30.1;
_2 = [_24,_38,_24,_38,_24,_38];
_42.1 = &_37;
_37 = &_42.0;
Goto(bb17)
}
bb17 = {
_39.fld0 = Move(_1);
_49 = _24 & _24;
_18 = [_29,_29];
_28 = _27.0;
(*_20).1 = 12074_i16 - (-7760_i16);
_41 = _15 as i16;
_14 = _39.fld2.fld3;
_39.fld1 = _26;
_39.fld2 = Adt20 { fld0: _23,fld1: 3985529956581584546_u64,fld2: _11,fld3: _14,fld4: _27.1,fld5: 21_u8,fld6: _3 };
_22 = -_13;
_7 = _12 as isize;
_27.1 = _39.fld2.fld4 << _8;
_35 = _28;
(*_20) = (_5, _41);
match _39.fld2.fld5 {
0 => bb13,
1 => bb2,
2 => bb10,
3 => bb9,
4 => bb5,
5 => bb18,
21 => bb20,
_ => bb19
}
}
bb18 = {
_10 = [true,true,true,false,false,false];
_6 = [false,false,true,true,true,false];
_12 = 318021519665914092881096742223877024478_u128 as i64;
_4 = _8;
_10 = [false,true,false,true,true,false];
_12 = (-1921322136170915825_i64) + (-5672043092766099482_i64);
_11 = _7 ^ _4;
_14 = 5_usize as f64;
_3 = 14837785967286312628_u64 as u32;
_6 = [true,false,true,false,true,true];
_10 = [false,false,true,false,true,false];
_9 = [false,true,false,false,false,false];
_6 = [false,false,false,true,true,false];
_15 = (-56_i8) - (-83_i8);
_7 = _15 as isize;
_15 = 32583339142449847248702532124252467282_i128 as i8;
_12 = 4799330362828749964_i64 & (-8391288150547617097_i64);
_4 = -_8;
_12 = (-857718337189213427_i64) | (-7622837179155846770_i64);
_6 = [true,true,true,false,false,true];
_15 = -(-32_i8);
_4 = 13335_u16 as isize;
_5 = '\u{a473b}';
_17 = [3249398408437642023_u64,11438974820355897146_u64,8590338255013195717_u64,3556173684095398763_u64,6213579676670103220_u64,1941873855746961275_u64];
_2 = _6;
_11 = _7;
Goto(bb3)
}
bb19 = {
_21 = core::ptr::addr_of_mut!(_13);
(*_19) = core::ptr::addr_of_mut!((*_21));
_19 = core::ptr::addr_of_mut!((*_19));
_11 = _7;
(*_19) = core::ptr::addr_of_mut!(_13);
_2 = [true,true,false,true,true,false];
_23 = (*_21) as usize;
_6 = [false,true,false,true,true,false];
_5 = '\u{940b6}';
_25 = _5 as i128;
_2 = [true,false,true,false,true,false];
_19 = core::ptr::addr_of_mut!((*_19));
Goto(bb8)
}
bb20 = {
(*_19) = core::ptr::addr_of_mut!(_30.1);
_39.fld2.fld4 = !(*_20).1;
_9 = _2;
_39.fld2.fld1 = (-661643550_i32) as u64;
(*_20).1 = _41;
_5 = _26;
_47 = [_39.fld2.fld0,_30.0.0];
_36 = _39.fld2.fld5;
_50 = _15 | _15;
_39.fld2.fld2 = _23 as isize;
_39.fld1 = _5;
_33 = (_11,);
(*_20).0 = _26;
_7 = _4;
_51 = _26 as isize;
_9 = [_24,_24,_49,_49,_24,_49];
_30.0.1 = &_37;
_54 = _30.1 as u128;
_4 = _33.0;
(*_16) = [_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1];
_50 = _36 as i8;
match _36 {
0 => bb1,
1 => bb6,
2 => bb21,
3 => bb22,
4 => bb23,
21 => bb25,
_ => bb24
}
}
bb21 = {
_18 = [22060_u16,65409_u16];
_30.0.0 = _5 as usize;
_15 = !(-71_i8);
_29 = _25 as u16;
_38 = !_24;
_16 = core::ptr::addr_of!(_30.3);
_39.fld2.fld3 = _14 * _14;
_8 = _11;
_35 = _5;
_30.2 = _3 as u128;
_39.fld2.fld5 = 22_u8 & 144_u8;
_39.fld2.fld0 = _15 as usize;
match (*_20).1 {
0 => bb7,
1 => bb4,
2 => bb13,
3 => bb14,
9523 => bb16,
_ => bb15
}
}
bb22 = {
_26 = _5;
_11 = _4;
_30.1 = _27.1 as f32;
_27 = (_5, 4734_i16);
_8 = _7 << _11;
_20 = core::ptr::addr_of!((*_20));
Call(_8 = fn7(Move(_16)), ReturnTo(bb11), UnwindUnreachable())
}
bb23 = {
_7 = !_11;
(*_21) = _22 - _22;
_15 = -43_i8;
(*_16) = [14536959790890196079_u64,8630308339268407197_u64,15290946051956257107_u64,1153107612278264448_u64,14573428201158355803_u64,13611847216308866089_u64];
_6 = _10;
_7 = _23 as isize;
_17 = [8185190458345895750_u64,8699609171179715860_u64,3211179918768016416_u64,10495661773069559390_u64,17639759235119661312_u64,9775413406373654930_u64];
(*_19) = core::ptr::addr_of_mut!((*_21));
_20 = core::ptr::addr_of!(_27);
(*_19) = core::ptr::addr_of_mut!((*_21));
_15 = _14 as i8;
_26 = _5;
_27.1 = !(-26698_i16);
_26 = _5;
_9 = [true,false,false,true,true,true];
Goto(bb9)
}
bb24 = {
_21 = core::ptr::addr_of_mut!(_13);
(*_19) = core::ptr::addr_of_mut!((*_21));
_19 = core::ptr::addr_of_mut!((*_19));
_11 = _7;
(*_19) = core::ptr::addr_of_mut!(_13);
_2 = [true,true,false,true,true,false];
_23 = (*_21) as usize;
_6 = [false,true,false,true,true,false];
_5 = '\u{940b6}';
_25 = _5 as i128;
_2 = [true,false,true,false,true,false];
_19 = core::ptr::addr_of_mut!((*_19));
Goto(bb8)
}
bb25 = {
_46.0 = (_12,);
(*_16) = [_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1,_39.fld2.fld1];
_4 = _39.fld2.fld2 * _39.fld2.fld2;
_48 = _39.fld2.fld1;
_32 = [1550988057_i32];
Goto(bb26)
}
bb26 = {
_51 = -_4;
_27 = (_5, _41);
_39.fld0 = core::ptr::addr_of_mut!(_29);
(*_20) = (_28, _39.fld2.fld4);
(*_20) = (_35, _39.fld2.fld4);
Goto(bb27)
}
bb27 = {
_12 = -_46.0.0;
_56 = !_33.0;
_39.fld2.fld1 = _4 as u64;
_39.fld3 = _30.2;
_39.fld2.fld0 = _36 as usize;
_39.fld4 = [_39.fld2.fld5,_36,_39.fld2.fld5,_36,_39.fld2.fld5,_39.fld2.fld5];
_58.0 = _29 * _29;
_6 = [_38,_49,_24,_38,_24,_38];
_22 = _30.1;
(*_20).0 = _26;
_59 = _3 as f32;
match _36 {
0 => bb3,
21 => bb28,
_ => bb6
}
}
bb28 = {
(*_20) = (_35, _39.fld2.fld4);
(*_20).1 = _39.fld2.fld0 as i16;
_58 = (_29, 47484100_i32, _28, (*_21));
_20 = core::ptr::addr_of!((*_20));
match _36 {
0 => bb19,
1 => bb9,
2 => bb7,
21 => bb30,
_ => bb29
}
}
bb29 = {
_11 = !_8;
(*_20).1 = !(-1796_i16);
_22 = (*_21) - _13;
_8 = _3 as isize;
_16 = core::ptr::addr_of!(_17);
_39.fld2.fld2 = !_11;
(*_21) = -_30.1;
_2 = [_24,_38,_24,_38,_24,_38];
_42.1 = &_37;
_37 = &_42.0;
Goto(bb17)
}
bb30 = {
_58.0 = _29 - _29;
Goto(bb31)
}
bb31 = {
_43 = _46.0.0;
_23 = _39.fld2.fld0;
_2 = [_24,_24,_24,_49,_49,_49];
_64.0.0 = (*_21) * _13;
(*_21) = -_13;
(*_21) = _58.3 - _64.0.0;
_16 = core::ptr::addr_of!((*_16));
_11 = -_33.0;
_19 = core::ptr::addr_of_mut!((*_19));
_42 = Move(_30.0);
_62 = _58.2 as usize;
match _36 {
0 => bb6,
1 => bb5,
2 => bb10,
3 => bb26,
4 => bb32,
5 => bb33,
21 => bb35,
_ => bb34
}
}
bb32 = {
_39.fld0 = Move(_1);
_49 = _24 & _24;
_18 = [_29,_29];
_28 = _27.0;
(*_20).1 = 12074_i16 - (-7760_i16);
_41 = _15 as i16;
_14 = _39.fld2.fld3;
_39.fld1 = _26;
_39.fld2 = Adt20 { fld0: _23,fld1: 3985529956581584546_u64,fld2: _11,fld3: _14,fld4: _27.1,fld5: 21_u8,fld6: _3 };
_22 = -_13;
_7 = _12 as isize;
_27.1 = _39.fld2.fld4 << _8;
_35 = _28;
(*_20) = (_5, _41);
match _39.fld2.fld5 {
0 => bb13,
1 => bb2,
2 => bb10,
3 => bb9,
4 => bb5,
5 => bb18,
21 => bb20,
_ => bb19
}
}
bb33 = {
_12 = -_46.0.0;
_56 = !_33.0;
_39.fld2.fld1 = _4 as u64;
_39.fld3 = _30.2;
_39.fld2.fld0 = _36 as usize;
_39.fld4 = [_39.fld2.fld5,_36,_39.fld2.fld5,_36,_39.fld2.fld5,_39.fld2.fld5];
_58.0 = _29 * _29;
_6 = [_38,_49,_24,_38,_24,_38];
_22 = _30.1;
(*_20).0 = _26;
_59 = _3 as f32;
match _36 {
0 => bb3,
21 => bb28,
_ => bb6
}
}
bb34 = {
_21 = core::ptr::addr_of_mut!(_13);
(*_19) = core::ptr::addr_of_mut!((*_21));
_19 = core::ptr::addr_of_mut!((*_19));
_11 = _7;
(*_19) = core::ptr::addr_of_mut!(_13);
_2 = [true,true,false,true,true,false];
_23 = (*_21) as usize;
_6 = [false,true,false,true,true,false];
_5 = '\u{940b6}';
_25 = _5 as i128;
_2 = [true,false,true,false,true,false];
_19 = core::ptr::addr_of_mut!((*_19));
Goto(bb8)
}
bb35 = {
_44 = _38 & _49;
_10 = [_49,_44,_44,_44,_24,_44];
_73.1 = (*_21);
_58.1 = -1257722446_i32;
_54 = _30.2;
_26 = _58.2;
(*_20) = (_5, _41);
_33.0 = _39.fld2.fld2;
_42.1 = &_37;
_55 = [_25,_25,_25];
_58.2 = _5;
_15 = _50 | _50;
_48 = _39.fld2.fld1 >> _39.fld2.fld2;
_73.0.1 = &_40.0;
_51 = _56;
_73.2 = _30.2;
_5 = _28;
_64.0.0 = -(*_21);
Goto(bb36)
}
bb36 = {
_59 = _29 as f32;
_46.0 = (_12,);
_39.fld3 = _48 as u128;
_2 = _10;
_15 = !_50;
_30.0.0 = _39.fld2.fld0 << _39.fld2.fld5;
_70 = _73.1 - (*_21);
(*_20).0 = _39.fld1;
_22 = _73.1 - (*_21);
_46.0 = (_12,);
_53 = _39.fld2.fld3;
_5 = _39.fld1;
_17 = [_48,_39.fld2.fld1,_39.fld2.fld1,_48,_39.fld2.fld1,_39.fld2.fld1];
(*_20).1 = _39.fld2.fld4 & _41;
_68 = [_56,_51,_33.0,_56,_56,_4,_4,_7];
_71 = !_49;
_73.3 = [_48,_48,_39.fld2.fld1,_48,_39.fld2.fld1,_48];
_1 = Move(_39.fld0);
_46.0 = (_43,);
(*_20).1 = _39.fld2.fld4;
Call(RET = fn17(Move(_42), _23), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
_46.0 = (_43,);
_12 = Field::<i64>(Variant(Field::<Adt62>(Variant(RET, 2), 2), 0), 1);
_39.fld2.fld4 = _27.1 + (*_20).1;
_53 = _23 as f64;
place!(Field::<u8>(Variant(place!(Field::<Adt62>(Variant(RET, 2), 2)), 0), 0)) = _39.fld2.fld5 % _39.fld2.fld5;
_45 = [_23,_39.fld2.fld0,_39.fld2.fld0,_23,_39.fld2.fld0];
_9 = _10;
_51 = !_39.fld2.fld2;
_27 = (_26, _39.fld2.fld4);
_26 = (*_20).0;
_39.fld4 = [_36,_39.fld2.fld5,_39.fld2.fld5,Field::<u8>(Variant(Field::<Adt62>(Variant(RET, 2), 2), 0), 0),_39.fld2.fld5,_39.fld2.fld5];
_73.1 = _70;
_42.0 = Field::<u8>(Variant(Field::<Adt62>(Variant(RET, 2), 2), 0), 0) as usize;
(*_21) = -_64.0.0;
_6 = [_44,_24,_44,_38,_49,_44];
_65 = core::ptr::addr_of_mut!(_58.0);
_60 = _3 as isize;
_27 = (_26, _39.fld2.fld4);
_22 = _64.0.0;
Goto(bb38)
}
bb38 = {
Call(_82 = dump_var(6_usize, 43_usize, Move(_43), 47_usize, Move(_47), 29_usize, Move(_29), 27_usize, Move(_27)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_82 = dump_var(6_usize, 9_usize, Move(_9), 4_usize, Move(_4), 55_usize, Move(_55), 23_usize, Move(_23)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_82 = dump_var(6_usize, 36_usize, Move(_36), 15_usize, Move(_15), 25_usize, Move(_25), 46_usize, Move(_46)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_82 = dump_var(6_usize, 68_usize, Move(_68), 60_usize, Move(_60), 24_usize, Move(_24), 18_usize, Move(_18)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_82 = dump_var(6_usize, 11_usize, Move(_11), 38_usize, Move(_38), 50_usize, Move(_50), 49_usize, Move(_49)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_82 = dump_var(6_usize, 33_usize, Move(_33), 6_usize, Move(_6), 83_usize, _83, 83_usize, _83), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: *const [u64; 6]) -> isize {
mir! {
type RET = isize;
let _2: f32;
let _3: isize;
let _4: isize;
let _5: [u32; 5];
let _6: (f32, &'static (u32, *mut bool, u16, *mut bool));
let _7: *const [u16; 2];
let _8: i8;
let _9: Adt28;
let _10: char;
let _11: u32;
let _12: [u16; 2];
let _13: (i64,);
let _14: f64;
let _15: bool;
let _16: u128;
let _17: (*const u64, [i16; 6]);
let _18: f64;
let _19: (Adt28, (i64,));
let _20: bool;
let _21: &'static i8;
let _22: u16;
let _23: &'static i8;
let _24: [i16; 5];
let _25: ();
let _26: ();
{
RET = !44_isize;
RET = -(-9223372036854775808_isize);
RET = (-70_isize) * 9223372036854775807_isize;
RET = 9223372036854775807_isize << 8885490989944811868_u64;
RET = !9223372036854775807_isize;
RET = (-28_i8) as isize;
RET = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
RET = 515963387_u32 as isize;
RET = !9223372036854775807_isize;
RET = -9223372036854775807_isize;
RET = 148480581556508755532570933964297096611_i128 as isize;
RET = (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
RET = 92_i8 as isize;
RET = -(-9223372036854775808_isize);
_4 = RET & RET;
RET = !_4;
Goto(bb2)
}
bb2 = {
_3 = _4 + _4;
RET = _3;
RET = _4 + _3;
RET = _4 ^ _3;
RET = _4;
_2 = (-2950168753791621109_i64) as f32;
_4 = 190235036365458189755036154175538181803_u128 as isize;
_4 = 9153825576595098222_usize as isize;
_3 = 16603469353143138837_u64 as isize;
RET = 16217570755213190761_u64 as isize;
_5 = [4159174732_u32,3732897572_u32,2847068252_u32,1019351820_u32,313504424_u32];
RET = !_3;
_9 = Adt28::Variant1 { fld0: _2 };
_2 = Field::<f32>(Variant(_9, 1), 0) * Field::<f32>(Variant(_9, 1), 0);
_8 = -(-90_i8);
Call(_7 = fn8(_5, Move(_1), _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _3;
place!(Field::<f32>(Variant(_9, 1), 0)) = _2 * _2;
_3 = !RET;
_9 = Adt28::Variant0 { fld0: (-28349252394024048841684089011148981334_i128),fld1: '\u{2b85f}',fld2: _3,fld3: _8,fld4: 22236_i16 };
place!(Field::<i8>(Variant(_9, 0), 3)) = 2_usize as i8;
_2 = 44127_u16 as f32;
_5 = [3428462141_u32,1381857074_u32,1966890198_u32,4120999733_u32,71347422_u32];
place!(Field::<isize>(Variant(_9, 0), 2)) = !RET;
place!(Field::<i128>(Variant(_9, 0), 0)) = (-125745468482063693185454216665603564460_i128) - 51956347764688814202981067200281743636_i128;
place!(Field::<i16>(Variant(_9, 0), 4)) = (-3218_i16) << RET;
place!(Field::<i8>(Variant(_9, 0), 3)) = _8 + _8;
RET = _3;
_11 = !1321076083_u32;
place!(Field::<i8>(Variant(_9, 0), 3)) = !_8;
place!(Field::<char>(Variant(_9, 0), 1)) = '\u{37a8a}';
_13.0 = !4774481167356589882_i64;
_2 = 11900479930868874790_u64 as f32;
place!(Field::<i16>(Variant(_9, 0), 4)) = (-12221_i16);
match Field::<i16>(Variant(_9, 0), 4) {
340282366920938463463374607431768199235 => bb4,
_ => bb1
}
}
bb4 = {
_10 = Field::<char>(Variant(_9, 0), 1);
RET = _4;
_14 = Field::<i128>(Variant(_9, 0), 0) as f64;
_8 = !Field::<i8>(Variant(_9, 0), 3);
SetDiscriminant(_9, 0);
place!(Field::<isize>(Variant(_9, 0), 2)) = _4 << _11;
_3 = -_4;
place!(Field::<i128>(Variant(_9, 0), 0)) = -(-38159680885173305112653135785010959714_i128);
place!(Field::<i128>(Variant(_9, 0), 0)) = !12134679473920623861371115761555640189_i128;
_16 = !306524787312062110995845709524953498324_u128;
_12 = [64303_u16,22796_u16];
_4 = -Field::<isize>(Variant(_9, 0), 2);
place!(Field::<i16>(Variant(_9, 0), 4)) = (-599065716_i32) as i16;
place!(Field::<char>(Variant(_9, 0), 1)) = _10;
_6.0 = _2;
_14 = 7711983616492162093_u64 as f64;
_16 = 180685605185823998708378858464463540759_u128;
_5 = [_11,_11,_11,_11,_11];
place!(Field::<isize>(Variant(_9, 0), 2)) = _3;
RET = !_4;
_6.0 = -_2;
_13.0 = _8 as i64;
_8 = (-60_i8) | 121_i8;
place!(Field::<isize>(Variant(_9, 0), 2)) = -_4;
RET = -_4;
RET = !_4;
match _16 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
180685605185823998708378858464463540759 => bb11,
_ => bb10
}
}
bb5 = {
RET = _3;
place!(Field::<f32>(Variant(_9, 1), 0)) = _2 * _2;
_3 = !RET;
_9 = Adt28::Variant0 { fld0: (-28349252394024048841684089011148981334_i128),fld1: '\u{2b85f}',fld2: _3,fld3: _8,fld4: 22236_i16 };
place!(Field::<i8>(Variant(_9, 0), 3)) = 2_usize as i8;
_2 = 44127_u16 as f32;
_5 = [3428462141_u32,1381857074_u32,1966890198_u32,4120999733_u32,71347422_u32];
place!(Field::<isize>(Variant(_9, 0), 2)) = !RET;
place!(Field::<i128>(Variant(_9, 0), 0)) = (-125745468482063693185454216665603564460_i128) - 51956347764688814202981067200281743636_i128;
place!(Field::<i16>(Variant(_9, 0), 4)) = (-3218_i16) << RET;
place!(Field::<i8>(Variant(_9, 0), 3)) = _8 + _8;
RET = _3;
_11 = !1321076083_u32;
place!(Field::<i8>(Variant(_9, 0), 3)) = !_8;
place!(Field::<char>(Variant(_9, 0), 1)) = '\u{37a8a}';
_13.0 = !4774481167356589882_i64;
_2 = 11900479930868874790_u64 as f32;
place!(Field::<i16>(Variant(_9, 0), 4)) = (-12221_i16);
match Field::<i16>(Variant(_9, 0), 4) {
340282366920938463463374607431768199235 => bb4,
_ => bb1
}
}
bb6 = {
_3 = _4 + _4;
RET = _3;
RET = _4 + _3;
RET = _4 ^ _3;
RET = _4;
_2 = (-2950168753791621109_i64) as f32;
_4 = 190235036365458189755036154175538181803_u128 as isize;
_4 = 9153825576595098222_usize as isize;
_3 = 16603469353143138837_u64 as isize;
RET = 16217570755213190761_u64 as isize;
_5 = [4159174732_u32,3732897572_u32,2847068252_u32,1019351820_u32,313504424_u32];
RET = !_3;
_9 = Adt28::Variant1 { fld0: _2 };
_2 = Field::<f32>(Variant(_9, 1), 0) * Field::<f32>(Variant(_9, 1), 0);
_8 = -(-90_i8);
Call(_7 = fn8(_5, Move(_1), _4), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = 92_i8 as isize;
RET = -(-9223372036854775808_isize);
_4 = RET & RET;
RET = !_4;
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
_13.0 = !1990752881249337860_i64;
place!(Field::<isize>(Variant(_9, 0), 2)) = 4193570008447591168_u64 as isize;
_2 = -_6.0;
place!(Field::<i128>(Variant(_9, 0), 0)) = 191_u8 as i128;
_17.1 = [Field::<i16>(Variant(_9, 0), 4),Field::<i16>(Variant(_9, 0), 4),Field::<i16>(Variant(_9, 0), 4),Field::<i16>(Variant(_9, 0), 4),Field::<i16>(Variant(_9, 0), 4),Field::<i16>(Variant(_9, 0), 4)];
place!(Field::<i16>(Variant(_9, 0), 4)) = -(-23784_i16);
_2 = -_6.0;
_9 = Adt28::Variant0 { fld0: 6843853070821813465289017384027059312_i128,fld1: _10,fld2: RET,fld3: _8,fld4: 8789_i16 };
_13 = ((-171759854154007215_i64),);
_6.0 = Field::<i8>(Variant(_9, 0), 3) as f32;
RET = 32407_i16 as isize;
_11 = 663793966_u32 - 96711473_u32;
place!(Field::<i128>(Variant(_9, 0), 0)) = -(-102137294368946887257799221606265492379_i128);
place!(Field::<i16>(Variant(_9, 0), 4)) = 26951_i16 + 25304_i16;
_7 = core::ptr::addr_of!(_12);
_19 = (Move(_9), _13);
_14 = _8 as f64;
match _13.0 {
0 => bb7,
1 => bb2,
2 => bb10,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
340282366920938463463202847577614204241 => bb17,
_ => bb16
}
}
bb12 = {
RET = _3;
place!(Field::<f32>(Variant(_9, 1), 0)) = _2 * _2;
_3 = !RET;
_9 = Adt28::Variant0 { fld0: (-28349252394024048841684089011148981334_i128),fld1: '\u{2b85f}',fld2: _3,fld3: _8,fld4: 22236_i16 };
place!(Field::<i8>(Variant(_9, 0), 3)) = 2_usize as i8;
_2 = 44127_u16 as f32;
_5 = [3428462141_u32,1381857074_u32,1966890198_u32,4120999733_u32,71347422_u32];
place!(Field::<isize>(Variant(_9, 0), 2)) = !RET;
place!(Field::<i128>(Variant(_9, 0), 0)) = (-125745468482063693185454216665603564460_i128) - 51956347764688814202981067200281743636_i128;
place!(Field::<i16>(Variant(_9, 0), 4)) = (-3218_i16) << RET;
place!(Field::<i8>(Variant(_9, 0), 3)) = _8 + _8;
RET = _3;
_11 = !1321076083_u32;
place!(Field::<i8>(Variant(_9, 0), 3)) = !_8;
place!(Field::<char>(Variant(_9, 0), 1)) = '\u{37a8a}';
_13.0 = !4774481167356589882_i64;
_2 = 11900479930868874790_u64 as f32;
place!(Field::<i16>(Variant(_9, 0), 4)) = (-12221_i16);
match Field::<i16>(Variant(_9, 0), 4) {
340282366920938463463374607431768199235 => bb4,
_ => bb1
}
}
bb13 = {
Return()
}
bb14 = {
RET = 92_i8 as isize;
RET = -(-9223372036854775808_isize);
_4 = RET & RET;
RET = !_4;
Goto(bb2)
}
bb15 = {
RET = 92_i8 as isize;
RET = -(-9223372036854775808_isize);
_4 = RET & RET;
RET = !_4;
Goto(bb2)
}
bb16 = {
_3 = _4 + _4;
RET = _3;
RET = _4 + _3;
RET = _4 ^ _3;
RET = _4;
_2 = (-2950168753791621109_i64) as f32;
_4 = 190235036365458189755036154175538181803_u128 as isize;
_4 = 9153825576595098222_usize as isize;
_3 = 16603469353143138837_u64 as isize;
RET = 16217570755213190761_u64 as isize;
_5 = [4159174732_u32,3732897572_u32,2847068252_u32,1019351820_u32,313504424_u32];
RET = !_3;
_9 = Adt28::Variant1 { fld0: _2 };
_2 = Field::<f32>(Variant(_9, 1), 0) * Field::<f32>(Variant(_9, 1), 0);
_8 = -(-90_i8);
Call(_7 = fn8(_5, Move(_1), _4), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_7 = core::ptr::addr_of!((*_7));
_13 = (_19.1.0,);
_3 = -Field::<isize>(Variant(_19.0, 0), 2);
SetDiscriminant(_19.0, 1);
_19.1 = _13;
_10 = '\u{efe2e}';
(*_7) = [19984_u16,28542_u16];
place!(Field::<f32>(Variant(_19.0, 1), 0)) = -_6.0;
_18 = _14;
_11 = !3537574388_u32;
(*_7) = [12204_u16,9696_u16];
_6.0 = 1607127910602776079_u64 as f32;
_3 = _18 as isize;
(*_7) = [40866_u16,3144_u16];
_11 = 3405199105_u32;
_23 = &_8;
_20 = !false;
_19.1 = (_13.0,);
_15 = _14 <= _18;
SetDiscriminant(_19.0, 3);
place!(Field::<(char, i16)>(Variant(_19.0, 3), 0)).1 = -27656_i16;
_20 = !_15;
_19.0 = Adt28::Variant0 { fld0: (-117852207320932405130488103778867878681_i128),fld1: _10,fld2: _4,fld3: _8,fld4: 10956_i16 };
_16 = 167513851563030333205321254943621536876_i128 as u128;
_3 = _6.0 as isize;
place!(Field::<char>(Variant(_19.0, 0), 1)) = _10;
_9 = Adt28::Variant1 { fld0: _2 };
_14 = _18 * _18;
Goto(bb18)
}
bb18 = {
Call(_25 = dump_var(7_usize, 8_usize, Move(_8), 11_usize, Move(_11), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_25 = dump_var(7_usize, 10_usize, Move(_10), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [u32; 5],mut _2: *const [u64; 6],mut _3: isize) -> *const [u16; 2] {
mir! {
type RET = *const [u16; 2];
let _4: bool;
let _5: ();
let _6: ();
{
_3 = 9223372036854775807_isize;
_3 = 3144377332533060902_u64 as isize;
_1 = [2128497003_u32,2236376547_u32,3543867441_u32,4292139281_u32,3472103765_u32];
_1 = [2970357185_u32,2985054780_u32,2992906255_u32,1236575217_u32,1165362683_u32];
Call(RET = fn9(Move(_2)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = false as isize;
_3 = -9223372036854775807_isize;
_1 = [2446783588_u32,2230328483_u32,3663550324_u32,1352868514_u32,601790595_u32];
_3 = (-9223372036854775808_isize);
_3 = 9223372036854775807_isize + (-9223372036854775808_isize);
_3 = !(-37_isize);
_3 = (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
Call(_5 = dump_var(8_usize, 3_usize, Move(_3), 6_usize, _6, 6_usize, _6, 6_usize, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: *const [u64; 6]) -> *const [u16; 2] {
mir! {
type RET = *const [u16; 2];
let _2: [u8; 6];
let _3: &'static i8;
let _4: (Adt28, (i64,));
let _5: u128;
let _6: [bool; 6];
let _7: &'static *const *mut Adt20;
let _8: Adt20;
let _9: bool;
let _10: (char, i8, usize, bool);
let _11: *mut Adt20;
let _12: f64;
let _13: (Adt28, (i64,));
let _14: [u64; 2];
let _15: &'static [i16; 5];
let _16: (i128, *mut u16, i64, [isize; 8]);
let _17: [u32; 5];
let _18: f32;
let _19: &'static usize;
let _20: [bool; 6];
let _21: char;
let _22: isize;
let _23: &'static Adt28;
let _24: &'static f32;
let _25: u32;
let _26: isize;
let _27: [i32; 1];
let _28: u8;
let _29: (u16, i32, char, f32);
let _30: usize;
let _31: i8;
let _32: &'static i8;
let _33: &'static &'static usize;
let _34: (u32, *mut bool, u16, *mut bool);
let _35: isize;
let _36: [i128; 3];
let _37: f64;
let _38: &'static [isize; 8];
let _39: Adt29;
let _40: (char, i8, usize, bool);
let _41: bool;
let _42: [i128; 2];
let _43: isize;
let _44: isize;
let _45: &'static [isize; 8];
let _46: f64;
let _47: bool;
let _48: (u32, *mut bool, u16, *mut bool);
let _49: [i128; 2];
let _50: i16;
let _51: [i128; 2];
let _52: [i16; 6];
let _53: isize;
let _54: (usize, &'static &'static usize);
let _55: isize;
let _56: &'static Adt28;
let _57: isize;
let _58: [u64; 2];
let _59: bool;
let _60: *mut u16;
let _61: *const [u16; 2];
let _62: *mut *mut f32;
let _63: isize;
let _64: [u16; 2];
let _65: u16;
let _66: (char, i8, usize, bool);
let _67: (Adt28, (i64,));
let _68: u128;
let _69: (&'static usize,);
let _70: ((i64,),);
let _71: i64;
let _72: &'static usize;
let _73: isize;
let _74: (&'static usize,);
let _75: (i128, *mut u16, i64, [isize; 8]);
let _76: char;
let _77: f32;
let _78: ();
let _79: ();
{
_2 = [34_u8,35_u8,212_u8,202_u8,4_u8,77_u8];
_4.1 = ((-4467190421800048465_i64),);
_4.0 = Adt28::Variant0 { fld0: 7158099082880935329415453907327875360_i128,fld1: '\u{83f99}',fld2: 47_isize,fld3: (-109_i8),fld4: (-13451_i16) };
_4.1 = ((-5783985843892248087_i64),);
_5 = !213772394300126559632426177015248000531_u128;
_3 = &place!(Field::<i8>(Variant(_4.0, 0), 3));
place!(Field::<i8>(Variant(_4.0, 0), 3)) = -(-41_i8);
place!(Field::<i128>(Variant(_4.0, 0), 0)) = 4_usize as i128;
_6 = [true,true,true,true,true,true];
_8.fld4 = Field::<i8>(Variant(_4.0, 0), 3) as i16;
_8.fld1 = !13609602583483000660_u64;
_8.fld2 = 120901412_i32 as isize;
_8.fld6 = 1526417807_u32 << _8.fld4;
_10 = ('\u{b6c8}', Field::<i8>(Variant(_4.0, 0), 3), 1_usize, true);
_8.fld6 = _4.1.0 as u32;
_8.fld3 = _5 as f64;
Goto(bb1)
}
bb1 = {
_10.3 = false | true;
Goto(bb2)
}
bb2 = {
_10.2 = 221_u8 as usize;
place!(Field::<isize>(Variant(_4.0, 0), 2)) = _8.fld2;
_13.1 = _4.1;
place!(Field::<i16>(Variant(_4.0, 0), 4)) = _10.1 as i16;
place!(Field::<i16>(Variant(_4.0, 0), 4)) = 137_u8 as i16;
_8.fld5 = _8.fld3 as u8;
_13.1.0 = _4.1.0;
_8.fld1 = !15121024950351068817_u64;
_4.1.0 = !_13.1.0;
_8.fld2 = Field::<isize>(Variant(_4.0, 0), 2) & Field::<isize>(Variant(_4.0, 0), 2);
Goto(bb3)
}
bb3 = {
_11 = core::ptr::addr_of_mut!(_8);
_8.fld0 = !_10.2;
_12 = -(*_11).fld3;
(*_11).fld1 = 9145173935002025287_u64;
place!(Field::<i128>(Variant(_4.0, 0), 0)) = 152239939005010995021773734599232374251_i128 * 108866660157623060303493071611309181540_i128;
_16.0 = Field::<i128>(Variant(_4.0, 0), 0);
place!(Field::<char>(Variant(_4.0, 0), 1)) = _10.0;
_10.2 = !(*_11).fld0;
_10.2 = (*_11).fld0 * (*_11).fld0;
_8.fld0 = _5 as usize;
_4.1 = (_13.1.0,);
(*_11).fld5 = 48_u8;
(*_11).fld2 = Field::<isize>(Variant(_4.0, 0), 2);
_3 = &_10.1;
(*_11).fld6 = Field::<i16>(Variant(_4.0, 0), 4) as u32;
(*_11).fld0 = _10.2 | _10.2;
_13 = (Move(_4.0), _4.1);
place!(Field::<i16>(Variant(_13.0, 0), 4)) = !(*_11).fld4;
_5 = 264143524795371199880991488976837145400_u128 | 225478612394165706864428886867624546883_u128;
_14 = [_8.fld1,_8.fld1];
Goto(bb4)
}
bb4 = {
_4.1 = (_13.1.0,);
Call(_10.1 = fn10(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = (Move(_13.0), _13.1);
(*_11).fld4 = !Field::<i16>(Variant(_4.0, 0), 4);
(*_11).fld6 = _4.1.0 as u32;
_4.1 = (_13.1.0,);
_5 = !89403657577746422814332951376869404681_u128;
_16.3 = [_8.fld2,_8.fld2,_8.fld2,Field::<isize>(Variant(_4.0, 0), 2),Field::<isize>(Variant(_4.0, 0), 2),_8.fld2,(*_11).fld2,(*_11).fld2];
_13 = (Move(_4.0), _4.1);
_9 = (*_11).fld5 != (*_11).fld5;
SetDiscriminant(_13.0, 0);
(*_11).fld6 = 2165810022_u32 | 2477084273_u32;
_14 = [(*_11).fld1,(*_11).fld1];
Call(place!(Field::<i8>(Variant(_13.0, 0), 3)) = fn12(), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19 = &(*_11).fld0;
_8.fld2 = 9223372036854775807_isize;
place!(Field::<i16>(Variant(_13.0, 0), 4)) = !_8.fld4;
(*_11).fld2 = 9223372036854775807_isize & (-28_isize);
(*_11).fld0 = !_10.2;
place!(Field::<i8>(Variant(_13.0, 0), 3)) = Field::<i16>(Variant(_13.0, 0), 4) as i8;
_3 = &_10.1;
_21 = _10.0;
_19 = &_10.2;
_8.fld0 = (*_11).fld6 as usize;
(*_11).fld1 = 14983812249099252550_u64 << (*_11).fld4;
_23 = &_13.0;
_10.3 = _9 ^ _9;
(*_11).fld5 = 137_u8 * 41_u8;
_5 = _10.0 as u128;
_10 = (_21, Field::<i8>(Variant(_13.0, 0), 3), (*_11).fld0, _9);
_4.0 = Adt28::Variant0 { fld0: _16.0,fld1: _10.0,fld2: (*_11).fld2,fld3: _10.1,fld4: (*_11).fld4 };
_10.1 = Field::<i8>(Variant((*_23), 0), 3);
(*_11).fld3 = -_12;
_3 = &_10.1;
match _4.1.0 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463457590621587875963369 => bb14,
_ => bb13
}
}
bb7 = {
_4 = (Move(_13.0), _13.1);
(*_11).fld4 = !Field::<i16>(Variant(_4.0, 0), 4);
(*_11).fld6 = _4.1.0 as u32;
_4.1 = (_13.1.0,);
_5 = !89403657577746422814332951376869404681_u128;
_16.3 = [_8.fld2,_8.fld2,_8.fld2,Field::<isize>(Variant(_4.0, 0), 2),Field::<isize>(Variant(_4.0, 0), 2),_8.fld2,(*_11).fld2,(*_11).fld2];
_13 = (Move(_4.0), _4.1);
_9 = (*_11).fld5 != (*_11).fld5;
SetDiscriminant(_13.0, 0);
(*_11).fld6 = 2165810022_u32 | 2477084273_u32;
_14 = [(*_11).fld1,(*_11).fld1];
Call(place!(Field::<i8>(Variant(_13.0, 0), 3)) = fn12(), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_4.1 = (_13.1.0,);
Call(_10.1 = fn10(), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_11 = core::ptr::addr_of_mut!(_8);
_8.fld0 = !_10.2;
_12 = -(*_11).fld3;
(*_11).fld1 = 9145173935002025287_u64;
place!(Field::<i128>(Variant(_4.0, 0), 0)) = 152239939005010995021773734599232374251_i128 * 108866660157623060303493071611309181540_i128;
_16.0 = Field::<i128>(Variant(_4.0, 0), 0);
place!(Field::<char>(Variant(_4.0, 0), 1)) = _10.0;
_10.2 = !(*_11).fld0;
_10.2 = (*_11).fld0 * (*_11).fld0;
_8.fld0 = _5 as usize;
_4.1 = (_13.1.0,);
(*_11).fld5 = 48_u8;
(*_11).fld2 = Field::<isize>(Variant(_4.0, 0), 2);
_3 = &_10.1;
(*_11).fld6 = Field::<i16>(Variant(_4.0, 0), 4) as u32;
(*_11).fld0 = _10.2 | _10.2;
_13 = (Move(_4.0), _4.1);
place!(Field::<i16>(Variant(_13.0, 0), 4)) = !(*_11).fld4;
_5 = 264143524795371199880991488976837145400_u128 | 225478612394165706864428886867624546883_u128;
_14 = [_8.fld1,_8.fld1];
Goto(bb4)
}
bb10 = {
_10.2 = 221_u8 as usize;
place!(Field::<isize>(Variant(_4.0, 0), 2)) = _8.fld2;
_13.1 = _4.1;
place!(Field::<i16>(Variant(_4.0, 0), 4)) = _10.1 as i16;
place!(Field::<i16>(Variant(_4.0, 0), 4)) = 137_u8 as i16;
_8.fld5 = _8.fld3 as u8;
_13.1.0 = _4.1.0;
_8.fld1 = !15121024950351068817_u64;
_4.1.0 = !_13.1.0;
_8.fld2 = Field::<isize>(Variant(_4.0, 0), 2) & Field::<isize>(Variant(_4.0, 0), 2);
Goto(bb3)
}
bb11 = {
_10.3 = false | true;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
(*_11).fld1 = 5381980345107548810_u64 * 8599462118622368754_u64;
_22 = -_8.fld2;
SetDiscriminant(_4.0, 0);
_17 = [(*_11).fld6,(*_11).fld6,(*_11).fld6,(*_11).fld6,_8.fld6];
(*_11).fld1 = 9993979123614561857_u64;
place!(Field::<i128>(Variant(_4.0, 0), 0)) = _16.0;
_20 = [_9,_9,_10.3,_9,_9,_10.3];
_11 = core::ptr::addr_of_mut!((*_11));
_8.fld0 = _10.2 + _10.2;
place!(Field::<i8>(Variant(_4.0, 0), 3)) = Field::<i8>(Variant((*_23), 0), 3);
_14 = [_8.fld1,(*_11).fld1];
_29.0 = 16300_u16 | 40386_u16;
match _4.1.0 {
0 => bb9,
1 => bb13,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb15,
340282366920938463457590621587875963369 => bb17,
_ => bb16
}
}
bb15 = {
_4.1 = (_13.1.0,);
Call(_10.1 = fn10(), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_11 = core::ptr::addr_of_mut!(_8);
_8.fld0 = !_10.2;
_12 = -(*_11).fld3;
(*_11).fld1 = 9145173935002025287_u64;
place!(Field::<i128>(Variant(_4.0, 0), 0)) = 152239939005010995021773734599232374251_i128 * 108866660157623060303493071611309181540_i128;
_16.0 = Field::<i128>(Variant(_4.0, 0), 0);
place!(Field::<char>(Variant(_4.0, 0), 1)) = _10.0;
_10.2 = !(*_11).fld0;
_10.2 = (*_11).fld0 * (*_11).fld0;
_8.fld0 = _5 as usize;
_4.1 = (_13.1.0,);
(*_11).fld5 = 48_u8;
(*_11).fld2 = Field::<isize>(Variant(_4.0, 0), 2);
_3 = &_10.1;
(*_11).fld6 = Field::<i16>(Variant(_4.0, 0), 4) as u32;
(*_11).fld0 = _10.2 | _10.2;
_13 = (Move(_4.0), _4.1);
place!(Field::<i16>(Variant(_13.0, 0), 4)) = !(*_11).fld4;
_5 = 264143524795371199880991488976837145400_u128 | 225478612394165706864428886867624546883_u128;
_14 = [_8.fld1,_8.fld1];
Goto(bb4)
}
bb17 = {
_9 = !_10.3;
_20 = [_10.3,_9,_9,_9,_9,_10.3];
_9 = (*_3) <= Field::<i8>(Variant(_13.0, 0), 3);
(*_11).fld6 = 574721386_u32;
_29.2 = _10.0;
(*_11).fld2 = _22 | _22;
(*_11).fld5 = 200_u8 >> _10.2;
_10.2 = _8.fld0;
_28 = !(*_11).fld5;
place!(Field::<char>(Variant(_4.0, 0), 1)) = _21;
_24 = &_18;
_10.3 = _9;
_10.2 = (*_11).fld0 << (*_11).fld2;
_34.3 = core::ptr::addr_of_mut!(_9);
_27 = [2054321807_i32];
place!(Field::<i8>(Variant(_13.0, 0), 3)) = -Field::<i8>(Variant(_4.0, 0), 3);
_29.1 = (*_11).fld6 as i32;
_8.fld3 = _8.fld2 as f64;
_16.0 = Field::<i128>(Variant(_4.0, 0), 0);
_23 = &_13.0;
(*_11).fld1 = 9850166183622174294_u64 & 14389662950690630969_u64;
_30 = (*_11).fld0;
Goto(bb18)
}
bb18 = {
place!(Field::<i16>(Variant(_13.0, 0), 4)) = -(*_11).fld4;
_8.fld4 = !Field::<i16>(Variant(_13.0, 0), 4);
_17 = [(*_11).fld6,(*_11).fld6,(*_11).fld6,(*_11).fld6,_8.fld6];
_10 = (_21, Field::<i8>(Variant(_4.0, 0), 3), _30, _9);
Goto(bb19)
}
bb19 = {
_19 = &_30;
_34.3 = core::ptr::addr_of_mut!(_9);
_33 = &_19;
place!(Field::<i16>(Variant(_4.0, 0), 4)) = Field::<i128>(Variant(_4.0, 0), 0) as i16;
place!(Field::<isize>(Variant(_13.0, 0), 2)) = _9 as isize;
place!(Field::<isize>(Variant(_4.0, 0), 2)) = _8.fld2 + (*_11).fld2;
SetDiscriminant(_4.0, 1);
(*_11).fld3 = _12 - _12;
(*_11).fld4 = Field::<i16>(Variant(_13.0, 0), 4) | Field::<i16>(Variant(_13.0, 0), 4);
_20 = [_10.3,_10.3,_10.3,_9,_9,_10.3];
_16.1 = core::ptr::addr_of_mut!(_29.0);
_38 = &_16.3;
_8.fld1 = 8372942686195816603_u64;
_29.3 = _29.0 as f32;
_8.fld6 = 2330903925_u32;
_3 = &_10.1;
place!(Field::<char>(Variant(_13.0, 0), 1)) = _10.0;
_4.1.0 = _13.1.0 + _13.1.0;
_27 = [_29.1];
_22 = -(*_11).fld2;
(*_11).fld1 = 13653682156791023310_u64 * 3699537287836685558_u64;
_34.3 = core::ptr::addr_of_mut!(_41);
_5 = 263621234369286730640514795335616232999_u128 * 213521118209098109641819457792998865937_u128;
_35 = _8.fld2;
_40.0 = Field::<char>(Variant(_13.0, 0), 1);
_13.1.0 = _4.1.0 ^ _4.1.0;
Goto(bb20)
}
bb20 = {
(*_11).fld0 = (*_19) + (*_19);
_40.1 = _10.0 as i8;
_19 = &(*_19);
_12 = -(*_11).fld3;
_21 = _29.2;
_26 = _29.0 as isize;
(*_11).fld0 = _10.2;
_8.fld6 = !3683205417_u32;
_18 = _29.3;
_13.1.0 = _4.1.0;
_16.2 = !_13.1.0;
_33 = &_19;
Call(_13.1 = fn14(Move(_33), Move(_38), Move(_3), (*_19), _8.fld5, _30, _8.fld2, Move((*_11)), _4.1), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_46 = _30 as f64;
(*_11).fld0 = !_10.2;
_8.fld2 = -_22;
_4.1.0 = -_13.1.0;
_45 = &_16.3;
(*_11).fld1 = _10.1 as u64;
_48.1 = core::ptr::addr_of_mut!(_9);
_34.2 = _29.0 << _8.fld0;
_48.0 = 1783789642_u32;
_5 = _12 as u128;
_29.2 = Field::<char>(Variant(_13.0, 0), 1);
_36 = [_16.0,_16.0,_16.0];
_4.1 = (_13.1.0,);
(*_11).fld6 = !_48.0;
_8.fld5 = !_28;
(*_11).fld4 = _18 as i16;
_40.1 = Field::<i8>(Variant(_13.0, 0), 3) >> (*_11).fld0;
Call(_30 = core::intrinsics::transmute((*_11).fld0), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_40 = (Field::<char>(Variant(_13.0, 0), 1), Field::<i8>(Variant((*_23), 0), 3), _30, _9);
_8.fld4 = Field::<i16>(Variant(_13.0, 0), 4);
_34.3 = Move(_48.1);
_44 = (*_11).fld2 & _26;
_8.fld3 = _46 + _12;
_48 = ((*_11).fld6, Move(_34.3), _34.2, Move(_34.3));
_8.fld6 = _48.0 >> Field::<i8>(Variant(_13.0, 0), 3);
_29 = (_48.2, (-877570953_i32), _10.0, _18);
(*_11).fld1 = (*_11).fld3 as u64;
place!(Field::<f32>(Variant(_4.0, 1), 0)) = -_29.3;
_10.0 = _29.2;
_29 = (_34.2, (-317817267_i32), _40.0, Field::<f32>(Variant(_4.0, 1), 0));
_14 = [(*_11).fld1,_8.fld1];
_19 = &(*_11).fld0;
_41 = _40.3;
_10.0 = _29.2;
_48.2 = _34.2 ^ _34.2;
_35 = _22;
_9 = _10.3;
(*_11).fld1 = 16460451614237886675_u64 | 8273454708208022277_u64;
_29 = (_48.2, 2127570441_i32, Field::<char>(Variant(_13.0, 0), 1), Field::<f32>(Variant(_4.0, 1), 0));
_49 = [_16.0,_16.0];
Goto(bb23)
}
bb23 = {
_13 = (Move(_4.0), _4.1);
_44 = _35;
_10.3 = _9 | _41;
_48.0 = !(*_11).fld6;
_51 = [_16.0,_16.0];
_53 = -(*_11).fld2;
_8.fld6 = _35 as u32;
_28 = (*_11).fld5;
_54.0 = (*_11).fld6 as usize;
_45 = &(*_45);
_54.1 = &_19;
_22 = (*_11).fld4 as isize;
_8.fld1 = 14407566350204641911_u64;
_35 = _26;
_40.2 = (*_19) + (*_11).fld0;
_16.0 = (-15774556751028064272622180163115216020_i128);
_48.1 = core::ptr::addr_of_mut!(_10.3);
_8.fld5 = _28 & _28;
_52 = [(*_11).fld4,_8.fld4,_8.fld4,_8.fld4,(*_11).fld4,_8.fld4];
_33 = Move(_54.1);
Goto(bb24)
}
bb24 = {
_47 = _10.3 | _10.3;
_25 = _8.fld3 as u32;
_32 = &_31;
_23 = &_13.0;
_22 = _44;
(*_11).fld5 = _28 - _28;
_56 = Move(_23);
_23 = &_4.0;
_41 = _10.3;
_43 = (*_11).fld2 & (*_11).fld2;
(*_11).fld2 = _43;
_40.0 = _10.0;
_55 = -(*_11).fld2;
_48.2 = _29.0 >> _53;
_34.2 = _48.2 * _29.0;
_30 = !_54.0;
_49 = [_16.0,_16.0];
_4 = (Move(_13.0), _13.1);
_29 = (_48.2, 1324226853_i32, _40.0, _18);
_24 = &_18;
_34 = (_25, Move(_48.1), _48.2, Move(_48.3));
_8.fld0 = _40.2 << (*_11).fld6;
_9 = _41 | _47;
(*_11).fld2 = _29.1 as isize;
(*_11).fld3 = (*_11).fld1 as f64;
SetDiscriminant(_4.0, 2);
(*_11).fld2 = _47 as isize;
match _16.0 {
0 => bb25,
324507810169910399190752427268652995436 => bb27,
_ => bb26
}
}
bb25 = {
_10.3 = false | true;
Goto(bb2)
}
bb26 = {
_19 = &_30;
_34.3 = core::ptr::addr_of_mut!(_9);
_33 = &_19;
place!(Field::<i16>(Variant(_4.0, 0), 4)) = Field::<i128>(Variant(_4.0, 0), 0) as i16;
place!(Field::<isize>(Variant(_13.0, 0), 2)) = _9 as isize;
place!(Field::<isize>(Variant(_4.0, 0), 2)) = _8.fld2 + (*_11).fld2;
SetDiscriminant(_4.0, 1);
(*_11).fld3 = _12 - _12;
(*_11).fld4 = Field::<i16>(Variant(_13.0, 0), 4) | Field::<i16>(Variant(_13.0, 0), 4);
_20 = [_10.3,_10.3,_10.3,_9,_9,_10.3];
_16.1 = core::ptr::addr_of_mut!(_29.0);
_38 = &_16.3;
_8.fld1 = 8372942686195816603_u64;
_29.3 = _29.0 as f32;
_8.fld6 = 2330903925_u32;
_3 = &_10.1;
place!(Field::<char>(Variant(_13.0, 0), 1)) = _10.0;
_4.1.0 = _13.1.0 + _13.1.0;
_27 = [_29.1];
_22 = -(*_11).fld2;
(*_11).fld1 = 13653682156791023310_u64 * 3699537287836685558_u64;
_34.3 = core::ptr::addr_of_mut!(_41);
_5 = 263621234369286730640514795335616232999_u128 * 213521118209098109641819457792998865937_u128;
_35 = _8.fld2;
_40.0 = Field::<char>(Variant(_13.0, 0), 1);
_13.1.0 = _4.1.0 ^ _4.1.0;
Goto(bb20)
}
bb27 = {
_58 = _14;
_6 = _20;
_48.0 = _25 * (*_11).fld6;
place!(Field::<usize>(Variant(_4.0, 2), 4)) = _9 as usize;
_30 = _8.fld0 & (*_11).fld0;
_16.1 = core::ptr::addr_of_mut!(_48.2);
_19 = &_40.2;
_11 = core::ptr::addr_of_mut!(place!(Field::<Adt20>(Variant(_4.0, 2), 2)));
_55 = !_43;
(*_11) = Adt20 { fld0: _8.fld0,fld1: _8.fld1,fld2: _55,fld3: _46,fld4: _8.fld4,fld5: _8.fld5,fld6: _48.0 };
(*_11).fld4 = _4.1.0 as i16;
_29 = (_48.2, 849581651_i32, _40.0, (*_24));
_56 = &_4.0;
_47 = Field::<Adt20>(Variant(_4.0, 2), 2).fld4 > Field::<Adt20>(Variant((*_56), 2), 2).fld4;
place!(Field::<usize>(Variant(_4.0, 2), 4)) = (*_11).fld0 ^ _54.0;
_41 = !_47;
_48.1 = Move(_34.1);
_59 = _8.fld6 <= _48.0;
_45 = &(*_45);
(*_11).fld5 = _16.0 as u8;
_8.fld4 = (*_11).fld4;
_32 = &(*_32);
place!(Field::<Adt20>(Variant(_4.0, 2), 2)).fld6 = _48.0;
_8.fld1 = !Field::<Adt20>(Variant(_4.0, 2), 2).fld1;
_46 = -Field::<Adt20>(Variant((*_56), 2), 2).fld3;
_60 = core::ptr::addr_of_mut!(_34.2);
place!(Field::<usize>(Variant(_4.0, 2), 4)) = (*_11).fld0 - Field::<Adt20>(Variant((*_56), 2), 2).fld0;
Goto(bb28)
}
bb28 = {
(*_11) = Move(_8);
_24 = &_18;
(*_60) = !_29.0;
_66.1 = _10.1;
_48.0 = !Field::<Adt20>(Variant(_4.0, 2), 2).fld6;
_16.3 = [_44,Field::<Adt20>(Variant(_4.0, 2), 2).fld2,_35,_43,_55,_44,(*_11).fld2,Field::<Adt20>(Variant(_4.0, 2), 2).fld2];
_66.2 = !_30;
_12 = (*_24) as f64;
_49 = [_16.0,_16.0];
_12 = (*_11).fld1 as f64;
_29.2 = _10.0;
_37 = _46;
_29.0 = _48.2;
_22 = _55 >> Field::<Adt20>(Variant(_4.0, 2), 2).fld4;
_3 = &_66.1;
_64 = [_29.0,_34.2];
_51 = [_16.0,_16.0];
_34.1 = Move(_34.3);
_68 = _5;
_45 = &_16.3;
_61 = core::ptr::addr_of!(_64);
(*_60) = !_29.0;
Goto(bb29)
}
bb29 = {
_66 = (_29.2, _10.1, (*_11).fld0, _10.3);
(*_61) = [_29.0,_48.2];
(*_60) = !_48.2;
_69.0 = Move(_19);
_68 = !_5;
(*_11).fld5 = !_28;
_29.2 = _21;
_24 = &_29.3;
place!(Field::<char>(Variant(_4.0, 2), 1)) = _10.0;
_67.0 = Adt28::Variant1 { fld0: (*_24) };
_54.1 = &_72;
RET = core::ptr::addr_of!((*_61));
_48.3 = core::ptr::addr_of_mut!(_10.3);
_24 = &(*_24);
(*_61) = [(*_60),_48.2];
_26 = _10.1 as isize;
_31 = _10.1 & _66.1;
_8.fld0 = Field::<usize>(Variant(_4.0, 2), 4);
_13 = (Move(_67.0), _4.1);
_16.0 = 21928079697229260685173531306647480902_i128;
(*RET) = [(*_60),(*_60)];
_55 = (*_11).fld2 >> _4.1.0;
Goto(bb30)
}
bb30 = {
Call(_78 = dump_var(9_usize, 53_usize, Move(_53), 55_usize, Move(_55), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_78 = dump_var(9_usize, 40_usize, Move(_40), 64_usize, Move(_64), 52_usize, Move(_52), 43_usize, Move(_43)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_78 = dump_var(9_usize, 25_usize, Move(_25), 27_usize, Move(_27), 59_usize, Move(_59), 28_usize, Move(_28)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_78 = dump_var(9_usize, 26_usize, Move(_26), 10_usize, Move(_10), 20_usize, Move(_20), 35_usize, Move(_35)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10() -> i8 {
mir! {
type RET = i8;
let _1: [u128; 6];
let _2: ((f32, &'static (u32, *mut bool, u16, *mut bool)),);
let _3: *mut f32;
let _4: f32;
let _5: &'static f32;
let _6: i16;
let _7: isize;
let _8: bool;
let _9: i32;
let _10: (isize,);
let _11: &'static i8;
let _12: usize;
let _13: Adt28;
let _14: (isize,);
let _15: f64;
let _16: i8;
let _17: u128;
let _18: isize;
let _19: f64;
let _20: (char, i8, usize, bool);
let _21: ();
let _22: ();
{
RET = 224230671445792511023399132604782215942_u128 as i8;
_1 = [279094251178834651607831866487653949156_u128,238786592501958140862513245455464639805_u128,254661221795564841329717485764822705912_u128,59352677047814944583236344881591365348_u128,150096750793027000127592039462661625667_u128,19848041719916105701462328122654830774_u128];
_2.0.0 = 10139628168032366308_usize as f32;
RET = 31_i8 >> 131406928413647467651429886687326378502_i128;
RET = 12_i8 & (-122_i8);
RET = (-63_i8) | (-35_i8);
_1 = [83048619556928378339534148544687210993_u128,304737400127519736496125592379932368311_u128,227854832309646453242440163843154774767_u128,168060929278726089417911894013411189273_u128,305943031918185915366406882668096276951_u128,7082659918311337304951155717787241475_u128];
_3 = core::ptr::addr_of_mut!(_2.0.0);
RET = 9_i8 ^ 41_i8;
_1 = [78384052078802269634453203833570351720_u128,226086150647593189812871872028250970961_u128,38845346433409553516724841786212273324_u128,65534834869372232821604402189383976435_u128,66534407927393930518633577782951623627_u128,280146168061877409040385363202154951069_u128];
_3 = core::ptr::addr_of_mut!(_2.0.0);
(*_3) = 12414599216162094213_u64 as f32;
(*_3) = 44_u8 as f32;
RET = (-9223372036854775808_isize) as i8;
(*_3) = 240_u8 as f32;
_1 = [284734668936697582136899360348041296348_u128,6013787093060076080020489548119509272_u128,140779111611279283115437483306040067397_u128,286431153298817239038780368740098360154_u128,64318321908247836695307862958254635225_u128,139427298342800066492516159248058039661_u128];
_2.0.0 = 543155878_i32 as f32;
_5 = &(*_3);
RET = (-48_i8);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211408 => bb9,
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
_1 = [181239568344897689644841339591503941479_u128,324088416777697218554632582086377126512_u128,101596420624195426734738695089520661604_u128,144095118365590380117856103920279761164_u128,109851285141628192361730292189081213239_u128,75676215129557194465652578148803019107_u128];
_2.0.0 = 141_u8 as f32;
RET = 161874461408299067644892508804079633429_i128 as i8;
_5 = &(*_3);
(*_3) = 1468846964675875511_u64 as f32;
_5 = &(*_3);
_1 = [83111129249559663843214533277145834998_u128,301710413366871994643514148834730399443_u128,52776837184370933680053354669637538558_u128,58673386451601953993843876171654278591_u128,316258298778884726816104232269329946593_u128,243749869736378359668578227879700199957_u128];
RET = (-122_i8) - (-14_i8);
(*_3) = RET as f32;
_5 = &_4;
_4 = 1508917680_u32 as f32;
_4 = -(*_3);
_4 = (*_3);
(*_3) = (-150239851797160587856214855411206204474_i128) as f32;
RET = (-32_i8);
_3 = core::ptr::addr_of_mut!(_4);
_2.0.0 = _4;
_4 = _2.0.0;
_5 = &_4;
_2.0.0 = 159595798969393791788525936061629931154_i128 as f32;
_3 = core::ptr::addr_of_mut!((*_5));
_5 = &_2.0.0;
RET = !(-73_i8);
_4 = (*_5) - (*_5);
RET = !81_i8;
_5 = &_4;
_4 = 2987092830_u32 as f32;
_4 = _2.0.0;
_3 = core::ptr::addr_of_mut!(_4);
(*_3) = _2.0.0 - _2.0.0;
Goto(bb10)
}
bb10 = {
(*_3) = -_2.0.0;
_6 = (-15961_i16);
(*_3) = 7141315790587248656_u64 as f32;
_6 = (-6227_i16) + 27267_i16;
RET = 38_i8 ^ (-11_i8);
_6 = (-22278_i16);
_3 = core::ptr::addr_of_mut!(_2.0.0);
_5 = &(*_3);
_6 = (-31015_i16) + (-28056_i16);
_8 = (*_3) >= _2.0.0;
RET = _6 as i8;
_5 = &(*_5);
_7 = _6 as isize;
_3 = core::ptr::addr_of_mut!(_4);
_8 = !true;
_6 = 203053566471541750888463733759849978055_u128 as i16;
RET = !37_i8;
_7 = !54_isize;
_8 = true & false;
RET = (-5_i8);
_3 = core::ptr::addr_of_mut!(_4);
_10.0 = _7;
RET = 1790406597_u32 as i8;
_9 = _2.0.0 as i32;
_5 = &(*_5);
Goto(bb11)
}
bb11 = {
_9 = (-2025354102_i32);
_8 = true ^ false;
_1 = [335723394489706354292494189265973918341_u128,201567584759873888781932014982671989828_u128,121984162409809664045094084326627481662_u128,18892841171632922068479811276928510453_u128,62190674987868809696827928004887547661_u128,30331038059247940756932051832674889761_u128];
_5 = &_4;
_12 = 2_usize;
_12 = !5_usize;
_4 = _9 as f32;
_11 = &RET;
_11 = &RET;
_5 = &(*_3);
(*_3) = _6 as f32;
(*_3) = _9 as f32;
RET = -90_i8;
_5 = &(*_3);
_12 = 3_usize - 5897881966262153880_usize;
_12 = _8 as usize;
_4 = _2.0.0;
RET = (-43_i8) << _10.0;
_5 = &(*_3);
_10.0 = 253332614_u32 as isize;
_8 = false;
RET = 489168377_u32 as i8;
_10 = (_7,);
(*_3) = -_2.0.0;
(*_3) = _2.0.0;
_3 = core::ptr::addr_of_mut!((*_3));
_12 = _10.0 as usize;
_10 = (_7,);
_8 = !false;
_10 = (_7,);
Goto(bb12)
}
bb12 = {
RET = 2371879388671946387757092515668990055_i128 as i8;
_2.0.0 = 34181_u16 as f32;
_12 = 2_usize + 0_usize;
RET = -119_i8;
_3 = core::ptr::addr_of_mut!((*_3));
_10.0 = 5021911064409494544_u64 as isize;
_8 = _7 > _7;
Call(_4 = fn11(_10, _2.0.0, _7, _10.0, _10.0, _1, _9, Move(_3), _12, _12), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10.0 = _7;
_4 = _2.0.0 * _2.0.0;
_3 = core::ptr::addr_of_mut!(_2.0.0);
_11 = &RET;
(*_3) = _12 as f32;
RET = -(-24_i8);
_12 = 5_usize & 11299886074883405583_usize;
(*_3) = _4;
_3 = core::ptr::addr_of_mut!(_2.0.0);
_3 = core::ptr::addr_of_mut!((*_3));
RET = -(-29_i8);
RET = (-1_i8) ^ (-118_i8);
(*_3) = _4;
_7 = -_10.0;
_5 = &_2.0.0;
(*_3) = _4;
_10.0 = 6294057952331592521_i64 as isize;
_10 = (_7,);
_3 = core::ptr::addr_of_mut!(_4);
(*_3) = _2.0.0 * _2.0.0;
_5 = &(*_3);
_4 = 101556105710129399907430271161620057141_u128 as f32;
_5 = &_4;
RET = (-25_i8) * (-14_i8);
_8 = !true;
RET = !72_i8;
Goto(bb14)
}
bb14 = {
_3 = core::ptr::addr_of_mut!((*_3));
_9 = (-561759663_i32);
_1 = [286920819843200041561853248109749550900_u128,291075289748362046761030592378663568195_u128,288167892863388605562220074549830322342_u128,226764523678629607996540329858354807424_u128,287127719590619946547301047868533219226_u128,119345358359218432444752435164402511268_u128];
_5 = &(*_3);
_9 = 953752068_i32;
_5 = &(*_3);
_5 = &(*_3);
_3 = core::ptr::addr_of_mut!(_4);
_4 = _2.0.0;
_9 = _4 as i32;
_2.0.0 = _4 + _4;
_14.0 = !_10.0;
_1 = [234876993876590108306211694538853986973_u128,283395761713918438010703665154786629294_u128,54459337635547120413074402470885821525_u128,271486127269331969785765799960009571447_u128,39512790881099033310123270362941935407_u128,207934333624663899718858160614494369229_u128];
RET = (-105_i8);
_6 = 9056755610672790767_i64 as i16;
_1 = [131614113932345645369200226507606256430_u128,226970061920941454690247349250915981886_u128,266334895836033079385174758866907648209_u128,323476887919645726052030227678145713024_u128,136606918732246086929554329916975306973_u128,10228520035821674839794049805283561708_u128];
_15 = _9 as f64;
_3 = core::ptr::addr_of_mut!(_2.0.0);
_5 = &_4;
_5 = &_2.0.0;
_9 = 175100313251216327983550618690916465304_u128 as i32;
_17 = 320029267198123514723720016311270944379_u128;
_5 = &(*_5);
RET = (-24_i8);
RET = (-3_i8) << _17;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(10_usize, 12_usize, Move(_12), 10_usize, Move(_10), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: (isize,),mut _2: f32,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [u128; 6],mut _7: i32,mut _8: *mut f32,mut _9: usize,mut _10: usize) -> f32 {
mir! {
type RET = f32;
let _11: f64;
let _12: f64;
let _13: char;
let _14: char;
let _15: f64;
let _16: f32;
let _17: [bool; 6];
let _18: i128;
let _19: bool;
let _20: bool;
let _21: [u8; 6];
let _22: f64;
let _23: char;
let _24: Adt80;
let _25: bool;
let _26: bool;
let _27: Adt56;
let _28: bool;
let _29: Adt56;
let _30: usize;
let _31: char;
let _32: f32;
let _33: (f32, &'static (u32, *mut bool, u16, *mut bool));
let _34: f32;
let _35: ();
let _36: ();
{
RET = _2 + _2;
RET = _2 + _2;
_2 = RET;
_7 = 66_i8 as i32;
_4 = !_1.0;
_1 = (_3,);
Call(_9 = core::intrinsics::bswap(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 3640613526253491279_u64 as f64;
_5 = 38679_u16 as isize;
_5 = 3213_u16 as isize;
_5 = _1.0 - _1.0;
_8 = core::ptr::addr_of_mut!(_2);
_11 = 5068300902859990899_i64 as f64;
_2 = RET - RET;
RET = 8189698180275528276494676685826898438_u128 as f32;
_6 = [175685582883775483263676903330552218765_u128,339532828340198634871414790669300061704_u128,192364464147091333401031026315937203440_u128,245817529786964267418530789606859650850_u128,77102746144583456120512618312548570296_u128,278025089056058152666740161998523008177_u128];
_9 = _10 << _7;
Goto(bb2)
}
bb2 = {
RET = -_2;
_10 = _9 >> _9;
(*_8) = -RET;
_12 = -_11;
_8 = core::ptr::addr_of_mut!((*_8));
_2 = RET - RET;
_7 = (-1391272684_i32) >> _4;
_7 = -1817565052_i32;
_5 = !_3;
_11 = 70319914496999012226723190213119290955_u128 as f64;
_4 = _1.0;
_19 = true;
_7 = 441574309_i32 << _10;
Goto(bb3)
}
bb3 = {
(*_8) = RET;
_17 = [_19,_19,_19,_19,_19,_19];
RET = _3 as f32;
RET = _7 as f32;
_17 = [_19,_19,_19,_19,_19,_19];
_15 = _4 as f64;
_16 = RET - RET;
_1.0 = 2785544274_u32 as isize;
_6 = [80814804406658822721143261580746406942_u128,197303423051948461932993232111471734827_u128,327446865060108731849642143239669843236_u128,129258967380782616667223127070132046643_u128,339564935018190204568297565119746781480_u128,191457462123388915350115597947534886161_u128];
_18 = 53736714676320239018169960966969284547_i128;
_19 = !false;
_9 = _16 as usize;
_13 = '\u{10a1fc}';
_4 = _1.0 | _3;
_19 = true | false;
_2 = _16;
_7 = 1760537303_i32;
_23 = _13;
_21 = [238_u8,171_u8,199_u8,168_u8,60_u8,221_u8];
(*_8) = -RET;
_23 = _13;
_8 = core::ptr::addr_of_mut!((*_8));
_17 = [_19,_19,_19,_19,_19,_19];
_18 = 108374688548223714355593885293540054893_i128 | 167058361265288501775141195238198100535_i128;
_14 = _23;
Goto(bb4)
}
bb4 = {
_9 = _10 + _10;
(*_8) = _16 * _16;
_6 = [222972593002955733297780032883070844214_u128,184958950659905215297457101064331741324_u128,187572757833878328865557414892164729053_u128,336463490414952375932440862893038128870_u128,146429793192230300933983192644779210506_u128,211515625109846248729110718882041325566_u128];
_3 = _1.0;
_6 = [100222567838133572848191223601056378599_u128,258171015663980767558393197382768246562_u128,248431905095294524598819398327428521467_u128,316565728120431969966698183668834683845_u128,254647959783607269743765710572247589925_u128,72110444835580807336582438894323718079_u128];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
1760537303 => bb7,
_ => bb6
}
}
bb5 = {
_11 = 3640613526253491279_u64 as f64;
_5 = 38679_u16 as isize;
_5 = 3213_u16 as isize;
_5 = _1.0 - _1.0;
_8 = core::ptr::addr_of_mut!(_2);
_11 = 5068300902859990899_i64 as f64;
_2 = RET - RET;
RET = 8189698180275528276494676685826898438_u128 as f32;
_6 = [175685582883775483263676903330552218765_u128,339532828340198634871414790669300061704_u128,192364464147091333401031026315937203440_u128,245817529786964267418530789606859650850_u128,77102746144583456120512618312548570296_u128,278025089056058152666740161998523008177_u128];
_9 = _10 << _7;
Goto(bb2)
}
bb6 = {
RET = -_2;
_10 = _9 >> _9;
(*_8) = -RET;
_12 = -_11;
_8 = core::ptr::addr_of_mut!((*_8));
_2 = RET - RET;
_7 = (-1391272684_i32) >> _4;
_7 = -1817565052_i32;
_5 = !_3;
_11 = 70319914496999012226723190213119290955_u128 as f64;
_4 = _1.0;
_19 = true;
_7 = 441574309_i32 << _10;
Goto(bb3)
}
bb7 = {
_16 = 25_u8 as f32;
_8 = core::ptr::addr_of_mut!(RET);
_11 = _12;
_5 = _10 as isize;
_8 = core::ptr::addr_of_mut!((*_8));
Goto(bb8)
}
bb8 = {
_23 = _13;
(*_8) = -_2;
_21 = [11_u8,88_u8,155_u8,78_u8,28_u8,64_u8];
_3 = _12 as isize;
_1 = (_4,);
_19 = true;
_22 = _15;
_17 = [_19,_19,_19,_19,_19,_19];
_20 = RET != RET;
_26 = _20;
(*_8) = _2;
_28 = _26 | _20;
_13 = _14;
RET = 50_u8 as f32;
Goto(bb9)
}
bb9 = {
_22 = _12 * _12;
_4 = 10220978566830184860_u64 as isize;
_25 = !_26;
_3 = 3217686922228205272_i64 as isize;
_10 = (*_8) as usize;
_2 = _9 as f32;
_4 = _5 - _5;
_31 = _14;
_20 = _28;
_10 = !_9;
_7 = (-774447580_i32) - 1500642654_i32;
_7 = 2002333438_i32 - (-552282023_i32);
_7 = (-82_i8) as i32;
Goto(bb10)
}
bb10 = {
_26 = _20;
(*_8) = _2;
_10 = _7 as usize;
_34 = -(*_8);
_33.0 = _2;
_9 = !_10;
_8 = core::ptr::addr_of_mut!(_32);
_9 = _10;
_13 = _23;
_31 = _13;
(*_8) = _34 * _2;
Goto(bb11)
}
bb11 = {
Call(_35 = dump_var(11_usize, 3_usize, Move(_3), 23_usize, Move(_23), 21_usize, Move(_21), 18_usize, Move(_18)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_35 = dump_var(11_usize, 9_usize, Move(_9), 26_usize, Move(_26), 31_usize, Move(_31), 10_usize, Move(_10)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_35 = dump_var(11_usize, 19_usize, Move(_19), 25_usize, Move(_25), 36_usize, _36, 36_usize, _36), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12() -> i8 {
mir! {
type RET = i8;
let _1: *mut bool;
let _2: char;
let _3: u64;
let _4: &'static [i16; 5];
let _5: bool;
let _6: u128;
let _7: [usize; 2];
let _8: f64;
let _9: bool;
let _10: isize;
let _11: char;
let _12: f32;
let _13: char;
let _14: ((i64,),);
let _15: [usize; 8];
let _16: bool;
let _17: bool;
let _18: f64;
let _19: isize;
let _20: i64;
let _21: i16;
let _22: f32;
let _23: Adt46;
let _24: usize;
let _25: &'static [usize; 5];
let _26: (usize, &'static &'static usize);
let _27: [u64; 6];
let _28: (i128, *mut u16, i64, [isize; 8]);
let _29: isize;
let _30: [u64; 6];
let _31: f32;
let _32: ();
let _33: ();
{
RET = 177048563520763015738324652154998439347_u128 as i8;
RET = -(-70_i8);
RET = (-13_i8);
match RET {
0 => bb1,
340282366920938463463374607431768211443 => bb3,
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
_2 = '\u{d9b5b}';
RET = 158546655417738019565762131537656348256_i128 as i8;
_3 = 3182625324711855838_i64 as u64;
Goto(bb4)
}
bb4 = {
RET = (-10335_i16) as i8;
_2 = '\u{10a2c0}';
_3 = 859140705479167331_u64 * 14128590511217129526_u64;
RET = 230_u8 as i8;
Goto(bb5)
}
bb5 = {
_3 = 5461867000304234777_u64;
_1 = core::ptr::addr_of_mut!(_5);
RET = (-11_i8) | (-36_i8);
_3 = 1988713015743825449_i64 as u64;
(*_1) = !true;
RET = 19_i8;
_5 = false;
(*_1) = true;
RET = _3 as i8;
_2 = '\u{3bea4}';
_1 = core::ptr::addr_of_mut!((*_1));
_5 = !true;
_2 = '\u{e5fa4}';
_2 = '\u{10d097}';
(*_1) = false;
_7 = [2604100220055316656_usize,1_usize];
_8 = 6_usize as f64;
_7 = [5_usize,6_usize];
_2 = '\u{b4abb}';
_6 = 249586936875239128813954783718701233827_u128;
RET = (-1919512387_i32) as i8;
RET = 28_i8;
_2 = '\u{47440}';
_2 = '\u{44ea2}';
_3 = !16980095461769243140_u64;
(*_1) = RET != RET;
_9 = (*_1);
_8 = 67256480855494840736886494105177835039_i128 as f64;
Goto(bb6)
}
bb6 = {
_3 = !5068304334487240896_u64;
(*_1) = _9 ^ _9;
_7 = [7671385034263387917_usize,3_usize];
Goto(bb7)
}
bb7 = {
_9 = !(*_1);
(*_1) = _9;
_6 = 226141563435522942242589388592445249419_u128 >> _3;
_2 = '\u{38307}';
_9 = _5 | (*_1);
match RET {
28 => bb8,
_ => bb5
}
}
bb8 = {
RET = !(-49_i8);
_5 = _9 >= _9;
(*_1) = _9;
_5 = _9;
_3 = 6631156136892193575_u64 ^ 10367192992493623397_u64;
_3 = 13682621081445926566_u64 - 13018291684872217121_u64;
(*_1) = !_9;
RET = 9_i8;
_10 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_10 = 9223372036854775807_isize * 97_isize;
(*_1) = !_9;
_11 = _2;
(*_1) = _9;
_3 = !6622411145634703773_u64;
RET = (-53_i8) | (-36_i8);
_13 = _2;
_1 = core::ptr::addr_of_mut!(_9);
_7 = [10302530426633708449_usize,4_usize];
Call(_14.0 = fn13(_13, _9, _11, Move(_1), _6), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = 181_u8 as i8;
_1 = core::ptr::addr_of_mut!(_9);
(*_1) = _5;
_14.0 = ((-3378917713753651251_i64),);
_15 = [0_usize,11492332145298684578_usize,7920450905221674961_usize,10128848745970182802_usize,2_usize,0_usize,15744412373106889375_usize,1_usize];
_10 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_5 = _9;
_6 = 114144941653326870114192722734263242163_u128;
_12 = 5_usize as f32;
_15 = [2_usize,1_usize,5_usize,5915664414288598643_usize,0_usize,14170041913926737770_usize,6_usize,15873476047832185038_usize];
_15 = [0_usize,4061590680088834054_usize,2_usize,1_usize,1_usize,604252033015215585_usize,1_usize,16733516016890434378_usize];
_1 = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb10)
}
bb10 = {
_9 = _5;
(*_1) = _5 ^ _5;
_14.0 = (7005817713624688601_i64,);
RET = (-106_i8) + (-16_i8);
_13 = _11;
RET = _14.0.0 as i8;
_8 = (-826710931_i32) as f64;
_8 = 4629186903528575286_usize as f64;
_11 = _2;
_9 = !_5;
_16 = (*_1);
(*_1) = !_16;
_7 = [3_usize,5_usize];
Goto(bb11)
}
bb11 = {
_14.0 = (8391478569959755356_i64,);
_7 = [4_usize,9362294124885357895_usize];
_9 = !_16;
_14.0.0 = 8960406222907928646_i64 << _3;
(*_1) = !_16;
_6 = _8 as u128;
_8 = _10 as f64;
(*_1) = _12 <= _12;
_8 = 46_u8 as f64;
_13 = _11;
(*_1) = _16 | _5;
_19 = -_10;
_8 = 53_u8 as f64;
_14.0.0 = _9 as i64;
_19 = 52905_u16 as isize;
RET = (-30_i8) - (-29_i8);
_23.fld2.fld6 = 1623878759_u32;
_23.fld3 = _6 ^ _6;
match _23.fld2.fld6 {
0 => bb9,
1 => bb4,
1623878759 => bb12,
_ => bb3
}
}
bb12 = {
_23.fld2.fld2 = _19;
(*_1) = !_5;
_23.fld1 = _11;
_23.fld2.fld4 = !(-15248_i16);
_23.fld1 = _11;
_13 = _11;
_23.fld2.fld3 = RET as f64;
_23.fld2.fld4 = (-17234_i16);
_12 = 18945_u16 as f32;
_16 = _23.fld2.fld6 == _23.fld2.fld6;
RET = 110_i8 ^ 48_i8;
_9 = !_16;
_7 = [5_usize,5_usize];
_23.fld2.fld5 = 48_u8;
_23.fld2.fld2 = _10 + _10;
_2 = _13;
Goto(bb13)
}
bb13 = {
_27 = [_3,_3,_3,_3,_3,_3];
_18 = _23.fld2.fld3;
_22 = -_12;
_23.fld2.fld0 = _23.fld2.fld3 as usize;
Goto(bb14)
}
bb14 = {
_14.0 = ((-5526154778090441531_i64),);
_12 = _22 * _22;
_1 = core::ptr::addr_of_mut!(_16);
_14.0.0 = (-4055212778272542739_i64) ^ 3191624642792974833_i64;
_1 = core::ptr::addr_of_mut!(_16);
_28.0 = -91990875695607898642710507373803672672_i128;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(12_usize, 10_usize, Move(_10), 11_usize, Move(_11), 19_usize, Move(_19), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(12_usize, 14_usize, Move(_14), 15_usize, Move(_15), 9_usize, Move(_9), 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: bool,mut _3: char,mut _4: *mut bool,mut _5: u128) -> (i64,) {
mir! {
type RET = (i64,);
let _6: ((f32, &'static (u32, *mut bool, u16, *mut bool)),);
let _7: bool;
let _8: ((f32, &'static (u32, *mut bool, u16, *mut bool)),);
let _9: &'static [usize; 5];
let _10: usize;
let _11: isize;
let _12: &'static (u32, *mut bool, u16, *mut bool);
let _13: usize;
let _14: char;
let _15: [u128; 6];
let _16: *mut f32;
let _17: u32;
let _18: [i128; 2];
let _19: f64;
let _20: *mut f32;
let _21: i32;
let _22: ();
let _23: ();
{
_1 = _3;
RET.0 = 9676688912973749259_usize as i64;
_2 = !false;
_3 = _1;
RET = (8700580475282617014_i64,);
_5 = 106937340510738237182720119690995843182_u128;
_6.0.0 = (-1624615462_i32) as f32;
RET = (6728897931493699267_i64,);
_4 = core::ptr::addr_of_mut!(_2);
RET.0 = -4366650651165485189_i64;
RET.0 = (-8724712769833854307_i64);
_4 = core::ptr::addr_of_mut!((*_4));
_3 = _1;
_4 = core::ptr::addr_of_mut!(_2);
_1 = _3;
_1 = _3;
_3 = _1;
RET.0 = 868001324_u32 as i64;
_1 = _3;
RET.0 = (-5650061800477041336_i64) << _5;
_4 = core::ptr::addr_of_mut!((*_4));
match _5 {
106937340510738237182720119690995843182 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_1 = _3;
_5 = 281973723232962035021474879007882927238_u128 * 205939849159627410402112783729300725173_u128;
_1 = _3;
_5 = !102386797260997147117697836065121835663_u128;
RET.0 = -7349170421686465987_i64;
_4 = core::ptr::addr_of_mut!(_2);
_3 = _1;
RET.0 = -(-3243243243128802394_i64);
_5 = 150840798691872276155279799098976376377_u128 ^ 218418145855945164127832431894667746842_u128;
_1 = _3;
_2 = false | false;
RET = ((-6202731576153891605_i64),);
RET = ((-7670450502023103159_i64),);
_6.0.0 = 12279845627751789079_usize as f32;
RET = ((-2662222147885875135_i64),);
RET = ((-5860717787356832308_i64),);
_6.0.0 = 15496525355572846945_u64 as f32;
_7 = _2;
(*_4) = _5 == _5;
_3 = _1;
RET.0 = !2125131992365983916_i64;
Goto(bb3)
}
bb3 = {
RET = (5365497348232014793_i64,);
(*_4) = !_7;
_11 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_3 = _1;
_14 = _3;
_14 = _3;
_7 = (*_4) != (*_4);
_13 = (-21977_i16) as usize;
_10 = _13;
match RET.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
5365497348232014793 => bb9,
_ => bb8
}
}
bb4 = {
_1 = _3;
_5 = 281973723232962035021474879007882927238_u128 * 205939849159627410402112783729300725173_u128;
_1 = _3;
_5 = !102386797260997147117697836065121835663_u128;
RET.0 = -7349170421686465987_i64;
_4 = core::ptr::addr_of_mut!(_2);
_3 = _1;
RET.0 = -(-3243243243128802394_i64);
_5 = 150840798691872276155279799098976376377_u128 ^ 218418145855945164127832431894667746842_u128;
_1 = _3;
_2 = false | false;
RET = ((-6202731576153891605_i64),);
RET = ((-7670450502023103159_i64),);
_6.0.0 = 12279845627751789079_usize as f32;
RET = ((-2662222147885875135_i64),);
RET = ((-5860717787356832308_i64),);
_6.0.0 = 15496525355572846945_u64 as f32;
_7 = _2;
(*_4) = _5 == _5;
_3 = _1;
RET.0 = !2125131992365983916_i64;
Goto(bb3)
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
_8.0.0 = _6.0.0;
_2 = !_7;
_2 = !_7;
_3 = _1;
(*_4) = _7;
_4 = core::ptr::addr_of_mut!(_2);
_11 = 93_isize;
_1 = _3;
_2 = !_7;
_11 = 82_isize;
(*_4) = _7 & _7;
_1 = _14;
_7 = (*_4);
_3 = _1;
RET = ((-1313763929037569773_i64),);
(*_4) = !_7;
_10 = !_13;
_16 = core::ptr::addr_of_mut!(_6.0.0);
(*_4) = !_7;
_17 = 865925477_u32 >> _13;
_14 = _1;
_8.0.0 = (*_16);
_18 = [155847790604593816038522754669242535388_i128,152634913490608737964983852049787658977_i128];
_16 = core::ptr::addr_of_mut!((*_16));
(*_16) = -_8.0.0;
Goto(bb10)
}
bb10 = {
_3 = _14;
(*_16) = _8.0.0;
_5 = 18115_u16 as u128;
_3 = _1;
RET = ((-7678228709090067517_i64),);
(*_4) = _7;
_10 = _13;
match _11 {
0 => bb11,
1 => bb12,
2 => bb13,
82 => bb15,
_ => bb14
}
}
bb11 = {
_1 = _3;
_5 = 281973723232962035021474879007882927238_u128 * 205939849159627410402112783729300725173_u128;
_1 = _3;
_5 = !102386797260997147117697836065121835663_u128;
RET.0 = -7349170421686465987_i64;
_4 = core::ptr::addr_of_mut!(_2);
_3 = _1;
RET.0 = -(-3243243243128802394_i64);
_5 = 150840798691872276155279799098976376377_u128 ^ 218418145855945164127832431894667746842_u128;
_1 = _3;
_2 = false | false;
RET = ((-6202731576153891605_i64),);
RET = ((-7670450502023103159_i64),);
_6.0.0 = 12279845627751789079_usize as f32;
RET = ((-2662222147885875135_i64),);
RET = ((-5860717787356832308_i64),);
_6.0.0 = 15496525355572846945_u64 as f32;
_7 = _2;
(*_4) = _5 == _5;
_3 = _1;
RET.0 = !2125131992365983916_i64;
Goto(bb3)
}
bb12 = {
RET = (5365497348232014793_i64,);
(*_4) = !_7;
_11 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_3 = _1;
_14 = _3;
_14 = _3;
_7 = (*_4) != (*_4);
_13 = (-21977_i16) as usize;
_10 = _13;
match RET.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
5365497348232014793 => bb9,
_ => bb8
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_8.0.0 = (*_16) + (*_16);
_13 = _10;
_4 = core::ptr::addr_of_mut!((*_4));
_16 = core::ptr::addr_of_mut!((*_16));
_8.0.0 = -(*_16);
_7 = !(*_4);
_19 = _5 as f64;
_6.0.0 = _8.0.0;
_6.0.0 = _8.0.0 * _8.0.0;
_8.0.0 = -_6.0.0;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(13_usize, 10_usize, Move(_10), 18_usize, Move(_18), 1_usize, Move(_1), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(13_usize, 17_usize, Move(_17), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static &'static usize,mut _2: &'static [isize; 8],mut _3: &'static i8,mut _4: usize,mut _5: u8,mut _6: usize,mut _7: isize,mut _8: Adt20,mut _9: (i64,)) -> (i64,) {
mir! {
type RET = (i64,);
let _10: f64;
let _11: usize;
let _12: Adt28;
let _13: &'static i32;
let _14: isize;
let _15: Adt62;
let _16: *mut u16;
let _17: *const (char, i16);
let _18: ();
let _19: ();
{
RET.0 = false as i64;
_7 = _8.fld2 + _8.fld2;
_8.fld3 = _5 as f64;
_8.fld4 = 25053_i16;
_8.fld6 = !2738870654_u32;
_8.fld4 = -6879_i16;
_8.fld2 = _7;
_11 = _8.fld0;
_8.fld2 = 9196_u16 as isize;
_10 = -_8.fld3;
_9 = (RET.0,);
RET = _9;
_9 = (RET.0,);
_8.fld4 = 79_i16;
match _8.fld4 {
0 => bb1,
79 => bb3,
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
_8.fld2 = _7;
_11 = !_6;
_4 = !_6;
RET = (_9.0,);
_8.fld1 = !9298913540449905170_u64;
_5 = _9.0 as u8;
_11 = _6 << _8.fld2;
RET.0 = !_9.0;
_8 = Adt20 { fld0: _6,fld1: 3592361687899533809_u64,fld2: _7,fld3: _10,fld4: 5232_i16,fld5: _5,fld6: 1272573844_u32 };
_8.fld0 = (-23935289174476658273098116090715918325_i128) as usize;
_7 = 20567_u16 as isize;
_8.fld1 = 9345068869412047231_u64 - 13255035927712124748_u64;
_8.fld5 = _8.fld3 as u8;
_9 = (RET.0,);
_8.fld1 = _11 as u64;
_8.fld4 = 11476_i16;
_12 = Adt28::Variant0 { fld0: 125867236668796874027133417014611508606_i128,fld1: '\u{36712}',fld2: _8.fld2,fld3: (-87_i8),fld4: _8.fld4 };
RET.0 = true as i64;
_15 = Adt62::Variant0 { fld0: _8.fld5,fld1: RET.0,fld2: _8.fld2 };
place!(Field::<char>(Variant(_12, 0), 1)) = '\u{de361}';
_3 = &place!(Field::<i8>(Variant(_12, 0), 3));
RET = (_9.0,);
_8.fld2 = Field::<isize>(Variant(_15, 0), 2) << _11;
_4 = _11;
Call(place!(Field::<i128>(Variant(_12, 0), 0)) = fn15(Move(_3), _8.fld6, _8.fld2, Field::<isize>(Variant(_12, 0), 2), _8.fld2, _8.fld1, Field::<u8>(Variant(_15, 0), 0), _8.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
place!(Field::<i8>(Variant(_12, 0), 3)) = 49_i8 | 125_i8;
_7 = -_8.fld2;
_5 = !_8.fld5;
_7 = -Field::<isize>(Variant(_12, 0), 2);
place!(Field::<i64>(Variant(_15, 0), 1)) = !_9.0;
place!(Field::<u8>(Variant(_15, 0), 0)) = !_8.fld5;
RET = (Field::<i64>(Variant(_15, 0), 1),);
RET.0 = _8.fld6 as i64;
place!(Field::<char>(Variant(_12, 0), 1)) = '\u{31523}';
_11 = !_4;
Goto(bb5)
}
bb5 = {
Call(_18 = dump_var(14_usize, 7_usize, Move(_7), 5_usize, Move(_5), 11_usize, Move(_11), 19_usize, _19), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: &'static i8,mut _2: u32,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u64,mut _7: u8,mut _8: isize) -> i128 {
mir! {
type RET = i128;
let _9: i16;
let _10: i32;
let _11: u128;
let _12: *mut f32;
let _13: isize;
let _14: (i64,);
let _15: [u64; 2];
let _16: (u16, i32, char, f32);
let _17: *const u64;
let _18: bool;
let _19: char;
let _20: *mut bool;
let _21: i8;
let _22: bool;
let _23: u16;
let _24: ();
let _25: ();
{
_3 = _5;
_3 = -_8;
_3 = -_5;
_2 = 1677883574_u32;
RET = (-70813623086578072762360257814965802748_i128) * (-134450225009456994655319178519043020619_i128);
_6 = 13432377455649269330_u64;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
13432377455649269330 => bb7,
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
_5 = _8;
_7 = 39_u8 ^ 225_u8;
_3 = -_8;
_9 = '\u{1036ff}' as i16;
_7 = !142_u8;
_4 = 0_usize as isize;
_5 = _4;
_10 = 1283594648_i32 ^ (-1346548818_i32);
_5 = false as isize;
_7 = (-4292698808773640392_i64) as u8;
_5 = -_3;
_10 = (-1925163992_i32) << _8;
RET = _9 as i128;
_5 = _4;
_3 = 1_usize as isize;
_11 = 131810277370689505016090720893037431312_u128 | 331600740756573016597471591133035835633_u128;
_6 = !2311178416163886523_u64;
Goto(bb8)
}
bb8 = {
RET = 76428664245446802753793303725106372332_i128 & (-79736471867175039543250820648041775143_i128);
_11 = 307425933060170553227811815957778438509_u128 ^ 69146673402843041999771389330553764089_u128;
_9 = _6 as i16;
_4 = _10 as isize;
match _2 {
0 => bb9,
1 => bb10,
2 => bb11,
1677883574 => bb13,
_ => bb12
}
}
bb9 = {
_5 = _8;
_7 = 39_u8 ^ 225_u8;
_3 = -_8;
_9 = '\u{1036ff}' as i16;
_7 = !142_u8;
_4 = 0_usize as isize;
_5 = _4;
_10 = 1283594648_i32 ^ (-1346548818_i32);
_5 = false as isize;
_7 = (-4292698808773640392_i64) as u8;
_5 = -_3;
_10 = (-1925163992_i32) << _8;
RET = _9 as i128;
_5 = _4;
_3 = 1_usize as isize;
_11 = 131810277370689505016090720893037431312_u128 | 331600740756573016597471591133035835633_u128;
_6 = !2311178416163886523_u64;
Goto(bb8)
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
_6 = _4 as u64;
_9 = _10 as i16;
_3 = _8 | _8;
_11 = 70910456060533009715089675217416048694_u128;
_14 = ((-916734673222333272_i64),);
_7 = 120_u8 - 166_u8;
_14 = (9160192928829460963_i64,);
RET = (-104088705923074089198897641056884208360_i128) - 108386921106617622620257922074603985396_i128;
_16.1 = -_10;
_12 = core::ptr::addr_of_mut!(_16.3);
_6 = 7784388404564799948_u64 >> _9;
_2 = 1721487688_u32 << _3;
_2 = 3447315229_u32;
_16.0 = !44369_u16;
_17 = core::ptr::addr_of!(_6);
_16.0 = _10 as u16;
_15 = [_6,_6];
RET = -(-72680246578908507447542816802455729369_i128);
(*_17) = 7945631529791490873_u64;
_13 = _3;
(*_17) = 4781858232110573877_u64 - 4768728343035236825_u64;
_2 = 754760789_u32 << _4;
Call((*_12) = fn16(Move(_17)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_19 = '\u{9b53a}';
_19 = '\u{ff1e4}';
_16.2 = _19;
_15 = [_6,_6];
_9 = false as i16;
_13 = !_3;
_4 = _8 | _13;
_4 = _8 << _16.1;
_16.3 = _9 as f32;
_10 = !_16.1;
_18 = !true;
(*_12) = _14.0 as f32;
_5 = _7 as isize;
_17 = core::ptr::addr_of!(_6);
_15 = [_6,_6];
_4 = 4_usize as isize;
_6 = 111_i8 as u64;
_3 = _13 << _16.0;
_13 = 13613729557228916456_usize as isize;
_1 = &_21;
_21 = 3_i8;
_14 = (1770497158399072054_i64,);
_6 = 17901688903264954638_u64;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(15_usize, 3_usize, Move(_3), 11_usize, Move(_11), 10_usize, Move(_10), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(15_usize, 5_usize, Move(_5), 7_usize, Move(_7), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *const u64) -> f32 {
mir! {
type RET = f32;
let _2: u64;
let _3: ((*mut u16, u32), [bool; 6], isize);
let _4: &'static i32;
let _5: u16;
let _6: f32;
let _7: f32;
let _8: [isize; 8];
let _9: isize;
let _10: [u64; 6];
let _11: (*const u64, [i16; 6]);
let _12: &'static [i16; 5];
let _13: ();
let _14: ();
{
RET = 33742_u16 as f32;
RET = 1411284975_u32 as f32;
_2 = 17231507021182217985_u64;
_3.1 = [false,false,false,true,true,false];
_3.1 = [false,true,true,false,true,false];
_3.0.1 = !1983896462_u32;
_3.2 = (-34_isize) ^ (-91_isize);
_3.1 = [false,false,true,true,true,false];
_1 = core::ptr::addr_of!(_2);
_3.0.0 = core::ptr::addr_of_mut!(_5);
_3.0.1 = '\u{1d12f}' as u32;
_3.2 = 69_isize | 34_isize;
(*_1) = 13524517033766019025_u64 - 13023998351760170556_u64;
_3.0.0 = core::ptr::addr_of_mut!(_5);
_2 = 16358628212558897534_u64 << _3.0.1;
_2 = 18394454785016536163_u64 * 10457347591271182733_u64;
_2 = !16541918563386781556_u64;
_3.2 = !9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_3.0.1 = 290310602_i32 as u32;
_6 = _3.0.1 as f32;
_5 = 41946_u16 & 55742_u16;
RET = 166842447679764968684060963382298466263_u128 as f32;
_7 = _3.2 as f32;
RET = -_7;
_2 = 1663007051_i32 as u64;
_3.0.0 = core::ptr::addr_of_mut!(_5);
_3.0.1 = !3252886499_u32;
_3.1 = [true,true,false,true,true,true];
_3.0.1 = !2970204807_u32;
RET = _6 - _7;
_1 = core::ptr::addr_of!(_2);
_6 = (*_1) as f32;
_3.1 = [false,true,true,false,false,true];
_3.1 = [true,false,false,true,true,true];
_3.0.1 = !3865544462_u32;
_9 = 6_usize as isize;
_3.0.0 = core::ptr::addr_of_mut!(_5);
_9 = _3.2;
(*_1) = 14253315445980006162_u64;
_9 = -_3.2;
match (*_1) {
0 => bb2,
14253315445980006162 => bb4,
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
(*_1) = !17948391980568449652_u64;
_3.1 = [true,true,true,false,false,false];
_8 = [_3.2,_9,_9,_9,_9,_9,_9,_3.2];
(*_1) = 2468249588985041743_u64;
match (*_1) {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
2468249588985041743 => bb13,
_ => bb12
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_3.0.1 = 290310602_i32 as u32;
_6 = _3.0.1 as f32;
_5 = 41946_u16 & 55742_u16;
RET = 166842447679764968684060963382298466263_u128 as f32;
_7 = _3.2 as f32;
RET = -_7;
_2 = 1663007051_i32 as u64;
_3.0.0 = core::ptr::addr_of_mut!(_5);
_3.0.1 = !3252886499_u32;
_3.1 = [true,true,false,true,true,true];
_3.0.1 = !2970204807_u32;
RET = _6 - _7;
_1 = core::ptr::addr_of!(_2);
_6 = (*_1) as f32;
_3.1 = [false,true,true,false,false,true];
_3.1 = [true,false,false,true,true,true];
_3.0.1 = !3865544462_u32;
_9 = 6_usize as isize;
_3.0.0 = core::ptr::addr_of_mut!(_5);
_9 = _3.2;
(*_1) = 14253315445980006162_u64;
_9 = -_3.2;
match (*_1) {
0 => bb2,
14253315445980006162 => bb4,
_ => bb3
}
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
_3.2 = 78_i8 as isize;
_2 = 8161436715739306289_u64 & 1629349300540146832_u64;
_10 = [_2,(*_1),(*_1),_2,_2,(*_1)];
_8 = [_9,_9,_3.2,_3.2,_9,_9,_3.2,_3.2];
(*_1) = 4585681118728663721_u64 - 2792302502759247189_u64;
_2 = !1940146105416353085_u64;
_8 = [_9,_3.2,_3.2,_3.2,_3.2,_9,_3.2,_9];
_6 = -_7;
(*_1) = !2943033228163163691_u64;
_6 = 3277_i16 as f32;
_9 = -_3.2;
Goto(bb14)
}
bb14 = {
_3.0.1 = _3.2 as u32;
_8 = [_9,_9,_3.2,_9,_9,_3.2,_3.2,_3.2];
_3.1 = [true,false,false,false,true,false];
_11.0 = core::ptr::addr_of!((*_1));
Goto(bb15)
}
bb15 = {
Call(_13 = dump_var(16_usize, 10_usize, Move(_10), 9_usize, Move(_9), 14_usize, _14, 14_usize, _14), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (usize, &'static &'static usize),mut _2: usize) -> Adt82 {
mir! {
type RET = Adt82;
let _3: *const i128;
let _4: (i128, *mut u16, i64, [isize; 8]);
let _5: isize;
let _6: &'static &'static usize;
let _7: u64;
let _8: f32;
let _9: char;
let _10: [isize; 8];
let _11: [usize; 8];
let _12: &'static usize;
let _13: f64;
let _14: bool;
let _15: &'static (u32, *mut bool, u16, *mut bool);
let _16: &'static &'static i32;
let _17: *const *mut Adt20;
let _18: *mut Adt20;
let _19: char;
let _20: &'static [i16; 5];
let _21: f64;
let _22: f32;
let _23: ((f32, &'static (u32, *mut bool, u16, *mut bool)),);
let _24: bool;
let _25: u32;
let _26: char;
let _27: isize;
let _28: i16;
let _29: Adt29;
let _30: i32;
let _31: Adt62;
let _32: (usize, &'static &'static usize);
let _33: u16;
let _34: (i64,);
let _35: (i128, &'static i8, Adt20);
let _36: isize;
let _37: Adt28;
let _38: i8;
let _39: &'static f32;
let _40: (i64,);
let _41: char;
let _42: [u64; 6];
let _43: isize;
let _44: [i128; 2];
let _45: *const i128;
let _46: *const [u64; 6];
let _47: isize;
let _48: u128;
let _49: isize;
let _50: [i16; 5];
let _51: ();
let _52: ();
{
_2 = 9223372036854775807_isize as usize;
_4.2 = (-6134289526742056830_i64) & (-5021388230195189854_i64);
_4.3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),40_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4.3 = [(-106_isize),114_isize,(-9223372036854775808_isize),69_isize,(-103_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_3 = core::ptr::addr_of!(_4.0);
(*_3) = (-127948771260289791380905589492430616670_i128);
_7 = !600661259273005271_u64;
(*_3) = 17642_i16 as i128;
_2 = !_1.0;
(*_3) = (-95174522772196653274025358275836935954_i128) << _7;
_7 = 1329955499_i32 as u64;
_8 = _1.0 as f32;
(*_3) = (-65814478055070153041102862860120002927_i128) - (-98437232402913330643298153768097670242_i128);
(*_3) = -(-129657188952762403916045086738389132870_i128);
_7 = _8 as u64;
_4.0 = (-69725740651432821902551701267427713836_i128);
_2 = !_1.0;
_4.2 = !(-4595325624688906030_i64);
match (*_3) {
270556626269505641560822906164340497620 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_4.2 = _7 as i64;
(*_3) = 48456812844922376720218230168859044032_i128;
_3 = core::ptr::addr_of!((*_3));
match (*_3) {
0 => bb1,
1 => bb3,
48456812844922376720218230168859044032 => bb5,
_ => bb4
}
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
_8 = 147_u8 as f32;
_2 = !_1.0;
_9 = '\u{1cd38}';
(*_3) = (-587134024210485676206266735775432576_i128) | 86238329842923238148455660459557759988_i128;
Goto(bb6)
}
bb6 = {
_11 = [_1.0,_2,_1.0,_1.0,_2,_2,_2,_1.0];
Call((*_3) = core::intrinsics::bswap(123399955293979069943322474766939285666_i128), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = core::ptr::addr_of!((*_3));
_1.0 = !_2;
_7 = 13641729940333839413_u64 & 14620039145536544377_u64;
_4.0 = 57244304869001358727027671111325006033_i128;
_1.0 = !_2;
_7 = 2748489587_u32 as u64;
_9 = '\u{e89a2}';
_5 = !9223372036854775807_isize;
_2 = _1.0 << _4.0;
_10 = [_5,_5,_5,_5,_5,_5,_5,_5];
_7 = _9 as u64;
_2 = _1.0;
_7 = 2693116257767005045_u64 | 12434070302197402490_u64;
_12 = &_1.0;
Goto(bb8)
}
bb8 = {
_1.0 = _2 ^ _2;
_6 = &_12;
_12 = &_1.0;
_2 = !_1.0;
_4.3 = [_5,_5,_5,_5,_5,_5,_5,_5];
(*_3) = _7 as i128;
_10 = _4.3;
(*_3) = 63322913643641882877419573085896106323_i128;
_10 = [_5,_5,_5,_5,_5,_5,_5,_5];
_13 = _8 as f64;
_6 = &_12;
_10 = [_5,_5,_5,_5,_5,_5,_5,_5];
(*_3) = 85491827985889080078514957741887451513_i128 & 15632300927045578494240723043400764284_i128;
_6 = &_12;
(*_3) = -169970717600341149435102890061937349399_i128;
Goto(bb9)
}
bb9 = {
_4.0 = (-152963561619120119390533989204762110855_i128);
Goto(bb10)
}
bb10 = {
_1.0 = !_2;
_8 = 838919108_u32 as f32;
_4.0 = (-20376027787311925674161883133183099499_i128);
(*_3) = 1941498881_i32 as i128;
_14 = false;
_4.2 = 4047564664847829119_i64;
_8 = 9493_u16 as f32;
_14 = false;
Goto(bb11)
}
bb11 = {
_6 = &_12;
_17 = core::ptr::addr_of!(_18);
_12 = &_1.0;
_4.3 = _10;
_7 = 71_u8 as u64;
_6 = &_12;
(*_3) = 146984401581797208610005561521974925987_i128 << (*_12);
_1 = (_2, Move(_6));
_8 = _4.2 as f32;
_19 = _9;
_2 = _1.0;
_8 = 59_u8 as f32;
_6 = &_12;
match _4.2 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb5,
4 => bb12,
4047564664847829119 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_8 = 147_u8 as f32;
_2 = !_1.0;
_9 = '\u{1cd38}';
(*_3) = (-587134024210485676206266735775432576_i128) | 86238329842923238148455660459557759988_i128;
Goto(bb6)
}
bb14 = {
_2 = !_1.0;
_6 = &(*_6);
(*_3) = 151990421297872887069756310423856427978_u128 as i128;
_4.2 = _8 as i64;
_2 = _4.2 as usize;
_5 = -21_isize;
_19 = _9;
_3 = core::ptr::addr_of!((*_3));
_14 = true;
_13 = (-20404_i16) as f64;
_4.0 = 22443_i16 as i128;
_4.0 = (-18895162272368786549257816424205075266_i128);
_1.1 = &(*_6);
_9 = _19;
_8 = 138_u8 as f32;
(*_3) = 153993730377576518719680315268001764887_i128;
_6 = &(*_6);
_1.1 = &(*_6);
match _4.0 {
153993730377576518719680315268001764887 => bb16,
_ => bb15
}
}
bb15 = {
_8 = 147_u8 as f32;
_2 = !_1.0;
_9 = '\u{1cd38}';
(*_3) = (-587134024210485676206266735775432576_i128) | 86238329842923238148455660459557759988_i128;
Goto(bb6)
}
bb16 = {
_5 = 9223372036854775807_isize << _4.2;
_3 = core::ptr::addr_of!((*_3));
(*_3) = !3223325599651187387081822668884318439_i128;
_13 = _4.2 as f64;
_5 = (-9223372036854775808_isize);
(*_3) = (-88863087626730255468395457682693772982_i128);
_22 = _13 as f32;
_2 = _8 as usize;
_1.1 = &(*_6);
_6 = &_12;
_1.0 = !_2;
_7 = !4950346751170515909_u64;
_2 = !_1.0;
_9 = _19;
_19 = _9;
(*_3) = _4.2 as i128;
_4.2 = 5534948314054454331_i64 & (-4621730442540253009_i64);
_2 = _1.0;
_6 = &(*_6);
_9 = _19;
_6 = &(*_6);
_5 = -11_isize;
Goto(bb17)
}
bb17 = {
_21 = 33217476181007141369591410087783518290_u128 as f64;
_24 = !_14;
_21 = _13;
_27 = _5;
_26 = _19;
_25 = 2022482104_u32 ^ 1196976008_u32;
_2 = _1.0;
(*_3) = _25 as i128;
_21 = -_13;
_12 = &_2;
_1.0 = !_2;
_5 = _27 >> _7;
_26 = _9;
_14 = !_24;
_28 = -32530_i16;
_7 = !7193106536330680240_u64;
_28 = (-14090_i16) * (-4611_i16);
Call(_23.0.0 = fn18(Move(_12), _4.3), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_14 = _24;
_28 = 17405_i16;
_32.0 = _2 << _1.0;
(*_3) = (-81347547279358853464011831784477037918_i128);
_22 = -_23.0.0;
_31 = Adt62::Variant0 { fld0: 212_u8,fld1: _4.2,fld2: _5 };
_26 = _19;
_10 = _4.3;
_25 = !1129276104_u32;
_23.0.0 = -_22;
_1.1 = &_12;
_35.2.fld4 = _28;
_26 = _19;
match (*_3) {
0 => bb13,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb7,
258934819641579609999362775647291173538 => bb20,
_ => bb19
}
}
bb19 = {
_2 = !_1.0;
_6 = &(*_6);
(*_3) = 151990421297872887069756310423856427978_u128 as i128;
_4.2 = _8 as i64;
_2 = _4.2 as usize;
_5 = -21_isize;
_19 = _9;
_3 = core::ptr::addr_of!((*_3));
_14 = true;
_13 = (-20404_i16) as f64;
_4.0 = 22443_i16 as i128;
_4.0 = (-18895162272368786549257816424205075266_i128);
_1.1 = &(*_6);
_9 = _19;
_8 = 138_u8 as f32;
(*_3) = 153993730377576518719680315268001764887_i128;
_6 = &(*_6);
_1.1 = &(*_6);
match _4.0 {
153993730377576518719680315268001764887 => bb16,
_ => bb15
}
}
bb20 = {
(*_3) = !(-80344322181116893017429805611609259791_i128);
_23.0.0 = _22 - _22;
_23.0.0 = _4.0 as f32;
_35.2 = Adt20 { fld0: _32.0,fld1: _7,fld2: Field::<isize>(Variant(_31, 0), 2),fld3: _21,fld4: _28,fld5: 102_u8,fld6: _25 };
Goto(bb21)
}
bb21 = {
_35.0 = (*_3) ^ (*_3);
_4.0 = (-1000510162_i32) as i128;
_7 = _35.2.fld1 & _35.2.fld1;
_19 = _9;
place!(Field::<i64>(Variant(_31, 0), 1)) = _4.2 + _4.2;
(*_17) = core::ptr::addr_of_mut!(_35.2);
(*_18).fld6 = _25 & _25;
(*_18) = Adt20 { fld0: _1.0,fld1: _7,fld2: Field::<isize>(Variant(_31, 0), 2),fld3: _21,fld4: _28,fld5: 116_u8,fld6: _25 };
_27 = (*_3) as isize;
_14 = _24;
_35.2.fld1 = _26 as u64;
_35.2 = Adt20 { fld0: _32.0,fld1: _7,fld2: _5,fld3: _13,fld4: _28,fld5: 234_u8,fld6: _25 };
_9 = _26;
place!(Field::<u8>(Variant(_31, 0), 0)) = _35.2.fld5 >> (*_18).fld5;
_32 = (_35.2.fld0, Move(_1.1));
_34.0 = (*_18).fld5 as i64;
_25 = !(*_18).fld6;
_34.0 = Field::<i64>(Variant(_31, 0), 1);
_33 = 46578_u16 * 52230_u16;
_32.0 = (*_18).fld0;
_35.0 = (*_3) - (*_3);
_35.2 = Adt20 { fld0: _1.0,fld1: _7,fld2: Field::<isize>(Variant(_31, 0), 2),fld3: _21,fld4: _28,fld5: Field::<u8>(Variant(_31, 0), 0),fld6: _25 };
_30 = 1483268781_i32;
_2 = !_1.0;
_35.2 = Adt20 { fld0: _32.0,fld1: _7,fld2: Field::<isize>(Variant(_31, 0), 2),fld3: _21,fld4: _28,fld5: Field::<u8>(Variant(_31, 0), 0),fld6: _25 };
_11 = [(*_18).fld0,(*_18).fld0,_32.0,_32.0,_35.2.fld0,_32.0,_2,_35.2.fld0];
_36 = (*_18).fld2;
match _30 {
0 => bb1,
1 => bb13,
2 => bb20,
3 => bb10,
4 => bb5,
5 => bb7,
6 => bb22,
1483268781 => bb24,
_ => bb23
}
}
bb22 = {
_11 = [_1.0,_2,_1.0,_1.0,_2,_2,_2,_1.0];
Call((*_3) = core::intrinsics::bswap(123399955293979069943322474766939285666_i128), ReturnTo(bb7), UnwindUnreachable())
}
bb23 = {
_5 = 9223372036854775807_isize << _4.2;
_3 = core::ptr::addr_of!((*_3));
(*_3) = !3223325599651187387081822668884318439_i128;
_13 = _4.2 as f64;
_5 = (-9223372036854775808_isize);
(*_3) = (-88863087626730255468395457682693772982_i128);
_22 = _13 as f32;
_2 = _8 as usize;
_1.1 = &(*_6);
_6 = &_12;
_1.0 = !_2;
_7 = !4950346751170515909_u64;
_2 = !_1.0;
_9 = _19;
_19 = _9;
(*_3) = _4.2 as i128;
_4.2 = 5534948314054454331_i64 & (-4621730442540253009_i64);
_2 = _1.0;
_6 = &(*_6);
_9 = _19;
_6 = &(*_6);
_5 = -11_isize;
Goto(bb17)
}
bb24 = {
_1.1 = &_12;
(*_18).fld1 = !_7;
_4.0 = -_35.0;
_32.0 = !_35.2.fld0;
_7 = _35.2.fld3 as u64;
(*_18).fld1 = (*_18).fld0 as u64;
_19 = _9;
_25 = (*_18).fld6 << (*_18).fld2;
(*_18).fld2 = (*_18).fld1 as isize;
_4.3 = _10;
(*_18).fld3 = _13 - _21;
_4.3 = [Field::<isize>(Variant(_31, 0), 2),Field::<isize>(Variant(_31, 0), 2),_36,(*_18).fld2,Field::<isize>(Variant(_31, 0), 2),_35.2.fld2,_5,(*_18).fld2];
_5 = _30 as isize;
_35.0 = -_4.0;
_32.1 = Move(_1.1);
_48 = 173660730826401970089036204390344894981_u128;
RET = Adt82::Variant2 { fld0: _14,fld1: Move((*_17)),fld2: Move(_31),fld3: _30 };
place!(Field::<*mut Adt20>(Variant(RET, 2), 1)) = core::ptr::addr_of_mut!(_35.2);
place!(Field::<u8>(Variant(place!(Field::<Adt62>(Variant(RET, 2), 2)), 0), 0)) = _35.2.fld5;
_14 = Field::<bool>(Variant(RET, 2), 0);
Goto(bb25)
}
bb25 = {
Call(_51 = dump_var(17_usize, 27_usize, Move(_27), 14_usize, Move(_14), 34_usize, Move(_34), 2_usize, Move(_2)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_51 = dump_var(17_usize, 7_usize, Move(_7), 24_usize, Move(_24), 48_usize, Move(_48), 10_usize, Move(_10)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_51 = dump_var(17_usize, 11_usize, Move(_11), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: &'static usize,mut _2: [isize; 8]) -> f32 {
mir! {
type RET = f32;
let _3: &'static *const *mut Adt20;
let _4: Adt20;
let _5: (*const u64, [i16; 6]);
let _6: i64;
let _7: (u32, *mut bool, u16, *mut bool);
let _8: &'static [i16; 5];
let _9: u8;
let _10: i32;
let _11: (*const u64, (u32, *mut bool, u16, *mut bool), f32);
let _12: char;
let _13: Adt56;
let _14: isize;
let _15: usize;
let _16: u16;
let _17: [u16; 2];
let _18: *mut u16;
let _19: Adt28;
let _20: (*const u64, [i16; 6]);
let _21: (u32, *mut bool, u16, *mut bool);
let _22: [i128; 3];
let _23: Adt80;
let _24: char;
let _25: u16;
let _26: (*mut u16, u32);
let _27: f64;
let _28: (f32, &'static (u32, *mut bool, u16, *mut bool));
let _29: Adt80;
let _30: f32;
let _31: [i128; 3];
let _32: u128;
let _33: ((*mut u16, u32), [bool; 6], isize);
let _34: f32;
let _35: u128;
let _36: ();
let _37: ();
{
RET = 157406403470766347266018396207947718712_i128 as f32;
_2 = [(-9223372036854775808_isize),11_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,116_isize,(-9223372036854775808_isize)];
_2 = [(-9223372036854775808_isize),(-56_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-94_isize),(-3_isize),9223372036854775807_isize];
RET = 22280_i16 as f32;
RET = (-1795195508_i32) as f32;
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),28_isize,9223372036854775807_isize];
_4.fld1 = !1686422500452330401_u64;
_1 = &_4.fld0;
_4.fld6 = (-15291616543519525182367042767579621571_i128) as u32;
_4.fld4 = (-20566_i16) ^ (-12785_i16);
_5.0 = core::ptr::addr_of!(_4.fld1);
_4.fld5 = 8713649916946927409_usize as u8;
_4.fld3 = _4.fld4 as f64;
_1 = &_4.fld0;
_4.fld2 = !(-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_7.0 = _4.fld6 << _4.fld1;
_4.fld3 = _4.fld2 as f64;
_4.fld0 = 1142059995_i32 as usize;
_7.2 = _4.fld3 as u16;
_4.fld1 = 9491275923408608784_u64;
_6 = 6223333409337013131_i64 >> _7.0;
_5.0 = core::ptr::addr_of!(_4.fld1);
_9 = _4.fld5 - _4.fld5;
RET = (-152671900221708163888370042468137831679_i128) as f32;
_11.1.2 = _7.2 >> _7.0;
_4.fld0 = !3_usize;
_4.fld0 = RET as usize;
_4.fld2 = (-15_isize) * (-9223372036854775808_isize);
_4.fld0 = 12392319486488570500_usize & 4_usize;
_11.2 = 125380235719155979660368031194730777665_u128 as f32;
_1 = &_4.fld0;
match _4.fld1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9491275923408608784 => bb9,
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
_4.fld0 = 9036620887071598468_usize + 5_usize;
_6 = 199987709886361441_i64;
_4.fld4 = (-1829_i16) + 5395_i16;
_6 = -(-5565649662529340202_i64);
RET = _11.2;
_5.1 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
RET = _11.2 - _11.2;
_6 = (-9002625356186922631_i64);
RET = _4.fld2 as f32;
_12 = '\u{1618}';
_2 = [_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2];
_4.fld1 = 14651147890957341341_u64;
_4.fld6 = !_7.0;
_7.2 = _11.1.2;
_12 = '\u{62e18}';
_11.2 = _4.fld4 as f32;
_2 = [_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2,_4.fld2];
_15 = _4.fld0 + _4.fld0;
_4.fld6 = !_7.0;
RET = _11.2 * _11.2;
Goto(bb10)
}
bb10 = {
_1 = &_4.fld0;
_4.fld3 = _11.1.2 as f64;
_4.fld6 = _7.0 | _7.0;
_14 = _4.fld2;
_11.2 = _4.fld4 as f32;
_4.fld0 = _15;
_6 = (-4797816130693223307_i64);
_4.fld2 = !_14;
_7.2 = !_11.1.2;
_11.2 = _4.fld4 as f32;
_7.0 = _14 as u32;
_12 = '\u{2c657}';
_10 = 298449241_i32;
_9 = _4.fld5;
_2 = [_14,_14,_14,_4.fld2,_14,_14,_4.fld2,_4.fld2];
_7.2 = _11.1.2 ^ _11.1.2;
_4.fld0 = _12 as usize;
_5.0 = core::ptr::addr_of!(_4.fld1);
_11.0 = Move(_5.0);
_11.0 = core::ptr::addr_of!(_4.fld1);
Call(_17 = core::intrinsics::transmute(_4.fld6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15 = !_4.fld0;
_4.fld5 = !_9;
_18 = core::ptr::addr_of_mut!(_16);
_5.1 = [_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4,_4.fld4];
(*_18) = _11.1.2 - _7.2;
_15 = !_4.fld0;
_4.fld0 = _10 as usize;
_16 = !_7.2;
_4.fld3 = 324700108569646071678295280578739720088_u128 as f64;
(*_18) = _7.2 + _11.1.2;
_19 = Adt28::Variant2 { fld0: _10,fld1: _12,fld2: Move(_4),fld3: _4.fld3,fld4: _4.fld0 };
_2 = [Field::<Adt20>(Variant(_19, 2), 2).fld2,Field::<Adt20>(Variant(_19, 2), 2).fld2,Field::<Adt20>(Variant(_19, 2), 2).fld2,_14,_14,Field::<Adt20>(Variant(_19, 2), 2).fld2,Field::<Adt20>(Variant(_19, 2), 2).fld2,_14];
place!(Field::<Adt20>(Variant(_19, 2), 2)).fld2 = _14;
RET = -_11.2;
RET = 90_i8 as f32;
_4.fld5 = 120_i8 as u8;
place!(Field::<Adt20>(Variant(_19, 2), 2)).fld5 = 47051110191204098212609548854292770731_i128 as u8;
_5.0 = core::ptr::addr_of!(place!(Field::<Adt20>(Variant(_19, 2), 2)).fld1);
_19 = Adt28::Variant1 { fld0: _11.2 };
_12 = '\u{67094}';
_19 = Adt28::Variant1 { fld0: _11.2 };
_18 = core::ptr::addr_of_mut!(_11.1.2);
_4.fld3 = _9 as f64;
_15 = 4_usize;
Goto(bb12)
}
bb12 = {
_20 = Move(_5);
Call(_21.2 = core::intrinsics::bswap(_7.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_21.2 = _7.0 as u16;
RET = _4.fld5 as f32;
_14 = _2[_15];
_4.fld1 = 4528724864191554538_u64 - 12412121325343438327_u64;
_25 = !(*_18);
(*_18) = _7.0 as u16;
_26.0 = Move(_18);
_4.fld0 = _15;
_7.0 = !2641334696_u32;
_2 = [_14,_14,_14,_14,_14,_14,_14,_14];
_7.0 = !75161469_u32;
_12 = '\u{d749f}';
_5.0 = Move(_20.0);
_1 = &_4.fld0;
_6 = -5446463804486618423_i64;
_16 = 3_i8 as u16;
_11.0 = Move(_5.0);
_22 = [107865091163958754144089273376911466207_i128,(-164996480241686935089238214384398958042_i128),33224657535831877127660553203415308229_i128];
place!(Field::<f32>(Variant(_19, 1), 0)) = -_11.2;
_4.fld3 = _6 as f64;
_2 = [_14,_14,_14,_14,_14,_14,_14,_14];
Goto(bb14)
}
bb14 = {
_5 = (Move(_11.0), _20.1);
SetDiscriminant(_19, 2);
_18 = Move(_26.0);
_20.1[_15] = _5.1[_15];
_25 = _21.2;
_28.0 = -RET;
place!(Field::<char>(Variant(_19, 2), 1)) = _12;
_4.fld6 = _7.0 - _7.0;
_7.2 = !_16;
_5.1 = [_20.1[_15],_20.1[_15],_20.1[_15],_20.1[_15],_20.1[_15],_20.1[_15]];
place!(Field::<Adt20>(Variant(_19, 2), 2)).fld1 = _4.fld1 - _4.fld1;
place!(Field::<Adt20>(Variant(_19, 2), 2)) = Adt20 { fld0: _4.fld0,fld1: _4.fld1,fld2: _2[_15],fld3: _4.fld3,fld4: _5.1[_15],fld5: _4.fld5,fld6: _4.fld6 };
_11.1.2 = _25 - _7.2;
place!(Field::<Adt20>(Variant(_19, 2), 2)).fld4 = _5.1[_15];
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(18_usize, 9_usize, Move(_9), 17_usize, Move(_17), 25_usize, Move(_25), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(18_usize, 10_usize, Move(_10), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(375063687130317553_u64), std::hint::black_box(479022407_u32), std::hint::black_box(116_u8), std::hint::black_box((-74_i8)), std::hint::black_box((-12216_i16)), std::hint::black_box(276333874003999377648204148075637731373_u128), std::hint::black_box((-2850529159985006544_i64)), std::hint::black_box(93127093516809899760732225189838425486_i128), std::hint::black_box(6_usize));
                
            }
impl PrintFDebug for Adt20{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt20{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt20 {
fld0: usize,
fld1: u64,
fld2: isize,
fld3: f64,
fld4: i16,
fld5: u8,
fld6: u32,
}
impl PrintFDebug for Adt28{
	unsafe fn printf_debug(&self){unsafe{printf("Adt28::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt28 {
Variant0{
fld0: i128,
fld1: char,
fld2: isize,
fld3: i8,
fld4: i16,

},
Variant1{
fld0: f32,

},
Variant2{
fld0: i32,
fld1: char,
fld2: Adt20,
fld3: f64,
fld4: usize,

},
Variant3{
fld0: (char, i16),
fld1: isize,

}}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){unsafe{printf("Adt29::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt29 {
Variant0{
fld0: u8,
fld1: (i64,),
fld2: u16,

},
Variant1{
fld0: u32,
fld1: char,
fld2: Adt20,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: *mut u16,
fld1: char,
fld2: Adt20,
fld3: u128,
fld4: [u8; 6],
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [isize; 8],
fld1: *mut bool,
fld2: *const u64,
fld3: [u128; 6],
fld4: [usize; 2],
fld5: (char, i16),

},
Variant1{
fld0: (i64,),
fld1: (*const u64, (u32, *mut bool, u16, *mut bool), f32),

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: u8,
fld1: i64,
fld2: isize,

},
Variant1{
fld0: (u16, i32, char, f32),
fld1: Adt56,
fld2: u8,
fld3: [i128; 3],
fld4: (u32, *mut bool, u16, *mut bool),
fld5: [bool; 6],
fld6: [u16; 2],

},
Variant2{
fld0: *const i128,
fld1: [u16; 2],
fld2: [i128; 3],
fld3: ((i64,),),
fld4: (u16, i32, char, f32),

},
Variant3{
fld0: Adt28,
fld1: char,
fld2: *mut Adt20,
fld3: f64,
fld4: [u16; 2],
fld5: Adt46,

}}
impl PrintFDebug for Adt80{
	unsafe fn printf_debug(&self){unsafe{printf("Adt80::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt80 {
Variant0{
fld0: [usize; 2],
fld1: [u128; 6],
fld2: u128,
fld3: u32,

},
Variant1{
fld0: [u32; 5],
fld1: *const (char, i16),
fld2: (*const u64, [i16; 6]),
fld3: i8,
fld4: (i128,),

},
Variant2{
fld0: (*const u64, (u32, *mut bool, u16, *mut bool), f32),
fld1: [u8; 6],

},
Variant3{
fld0: [usize; 8],
fld1: *const *mut Adt20,
fld2: (u16, i32, char, f32),
fld3: u128,
fld4: [i128; 2],
fld5: Adt56,
fld6: i64,

}}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){unsafe{printf("Adt82::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt82 {
Variant0{
fld0: [usize; 2],
fld1: (isize,),
fld2: usize,
fld3: [isize; 8],

},
Variant1{
fld0: Adt28,

},
Variant2{
fld0: bool,
fld1: *mut Adt20,
fld2: Adt62,
fld3: i32,

},
Variant3{
fld0: *const [u64; 6],
fld1: ((i64,),),
fld2: [u64; 6],
fld3: [usize; 5],
fld4: (Adt28, (i64,)),
fld5: (char, i8, usize, bool),

}}

