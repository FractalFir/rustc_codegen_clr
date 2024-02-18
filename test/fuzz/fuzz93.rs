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
pub fn fn0(mut _1: u128,mut _2: char,mut _3: u16,mut _4: u8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: u32) -> f32 {
mir! {
type RET = f32;
let _10: u16;
let _11: isize;
let _12: bool;
let _13: f64;
let _14: ([char; 2], usize);
let _15: isize;
let _16: isize;
let _17: [i32; 2];
let _18: (&'static [bool; 2], i128, [isize; 7], i32);
let _19: Adt81;
let _20: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _21: u128;
let _22: char;
let _23: char;
let _24: isize;
let _25: isize;
let _26: f32;
let _27: Adt60;
let _28: (u32, u128, u64);
let _29: (&'static *mut f64, [i32; 2], u16);
let _30: bool;
let _31: *mut &'static i16;
let _32: f32;
let _33: isize;
let _34: isize;
let _35: Adt60;
let _36: [isize; 3];
let _37: *const Adt35;
let _38: i64;
let _39: bool;
let _40: isize;
let _41: &'static (u128, u64, u8, (i32, bool, bool, f64));
let _42: isize;
let _43: u8;
let _44: i32;
let _45: ([char; 2], usize);
let _46: *const &'static i16;
let _47: bool;
let _48: ();
let _49: ();
{
_4 = 230_u8 & 138_u8;
_8 = (-3295429763458174847821349493177623872_i128);
RET = 53577_u16 as f32;
_5 = 6521_i16;
_9 = 4273744671_u32 << _5;
_2 = '\u{b9c31}';
_6 = 249434560056122364343232271020741156989_u128 as i32;
_2 = '\u{568a7}';
_7 = 8431167935099828485_i64;
Call(_3 = fn1(_2, _5, _9, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = -(-717758806_i32);
_1 = 69406189929901599337537939977910876634_u128;
RET = 7575856754460164147_u64 as f32;
_9 = 3626452835_u32 - 2989928179_u32;
_4 = 191_u8;
_2 = '\u{ba3b3}';
_1 = 69422796057235697159700305430024452769_u128;
_6 = (-13266416_i32);
_9 = !3963690188_u32;
_6 = (-2018260443_i32);
_7 = !(-2990077476226807145_i64);
_6 = _1 as i32;
_4 = 32_u8;
_9 = (-10_i8) as u32;
_8 = 145095731187516688178565428264066811086_i128;
_6 = (-1774228548_i32);
RET = 72_i8 as f32;
_6 = (-568875727_i32) ^ 1772303_i32;
_3 = 0_usize as u16;
Goto(bb2)
}
bb2 = {
RET = 3586375132671071062_usize as f32;
_10 = 573827106561258965_usize as u16;
_10 = _3;
_10 = !_3;
_5 = (-31478_i16) + 19055_i16;
_3 = _10 >> _6;
_12 = !false;
_10 = _7 as u16;
_11 = _7 as isize;
_13 = _3 as f64;
_2 = '\u{fb331}';
RET = (-18_i8) as f32;
_14.1 = _7 as usize;
_14.0 = [_2,_2];
_15 = _11 ^ _11;
_14.1 = 7_usize;
_2 = '\u{949fb}';
match _1 {
0 => bb1,
1 => bb3,
69422796057235697159700305430024452769 => bb5,
_ => bb4
}
}
bb3 = {
_6 = -(-717758806_i32);
_1 = 69406189929901599337537939977910876634_u128;
RET = 7575856754460164147_u64 as f32;
_9 = 3626452835_u32 - 2989928179_u32;
_4 = 191_u8;
_2 = '\u{ba3b3}';
_1 = 69422796057235697159700305430024452769_u128;
_6 = (-13266416_i32);
_9 = !3963690188_u32;
_6 = (-2018260443_i32);
_7 = !(-2990077476226807145_i64);
_6 = _1 as i32;
_4 = 32_u8;
_9 = (-10_i8) as u32;
_8 = 145095731187516688178565428264066811086_i128;
_6 = (-1774228548_i32);
RET = 72_i8 as f32;
_6 = (-568875727_i32) ^ 1772303_i32;
_3 = 0_usize as u16;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
RET = 4154265611532100390_u64 as f32;
_7 = 1688087761007147277_i64;
_2 = '\u{ce951}';
_9 = 3086196814_u32;
_12 = false;
_7 = 8607862870338098354_i64 & 2816271466755282671_i64;
_1 = 226816619460867731907182202438830527845_u128 * 36220128540624571612552498504738599075_u128;
_14.1 = !7_usize;
_10 = !_3;
_16 = _15;
Call(_3 = fn13(RET, _16, _4, _10), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15 = !_16;
_6 = 1267331207_i32;
_3 = !_10;
RET = 5596395170370244682_u64 as f32;
_14.0 = [_2,_2];
_13 = 33_i8 as f64;
_7 = -(-5449848183166169233_i64);
_13 = 17841280315717405046_u64 as f64;
_1 = !250372362874905041832073911295935444051_u128;
RET = _4 as f32;
_15 = _11 - _16;
RET = (-41_i8) as f32;
_10 = _3;
_3 = _4 as u16;
_17 = [_6,_6];
Goto(bb7)
}
bb7 = {
_11 = _15;
_13 = (-93_i8) as f64;
_6 = _9 as i32;
_16 = _11;
_15 = _11 ^ _11;
_11 = !_16;
_5 = !25450_i16;
_14.0 = [_2,_2];
_10 = _3;
_2 = '\u{fd303}';
_7 = 7593728902597767236_i64 * 910364799389949248_i64;
RET = _5 as f32;
_9 = 1016536846_u32;
_13 = _7 as f64;
_2 = '\u{efbb5}';
_5 = !9859_i16;
_8 = !105466530490584486265281742508219043022_i128;
_16 = _15;
RET = _5 as f32;
_9 = 32801195_u32;
_14.1 = _7 as usize;
_4 = 3_u8;
_15 = !_11;
RET = _16 as f32;
_12 = !true;
_5 = (-23272_i16) + (-23671_i16);
_11 = _16 & _16;
Goto(bb8)
}
bb8 = {
_12 = RET >= RET;
_8 = _1 as i128;
_7 = _16 as i64;
_8 = 141813060921498912253943117331726041591_i128;
_14.0 = [_2,_2];
_9 = !2234392318_u32;
_3 = !_10;
_17 = [_6,_6];
_8 = 82756088109523844128336052143229443034_i128 & 102466527417832342883541234660456654414_i128;
_20.1 = [_3];
_20.2 = !_12;
_20.0.2 = _5 * _5;
_18.1 = _8;
_9 = !920302573_u32;
_4 = 203_u8 << _7;
RET = _16 as f32;
_2 = '\u{bee19}';
_1 = _14.1 as u128;
_18.3 = _6;
_8 = _7 as i128;
_20.2 = _12;
_20.0.3.0 = [_8,_8,_8,_8,_18.1];
_20.0.1 = [_2,_2];
_20.3 = -_11;
Goto(bb9)
}
bb9 = {
_20.0.0 = [_18.1,_8,_18.1];
_19 = Adt81::Variant0 { fld0: _4 };
_3 = _11 as u16;
_3 = _10;
_11 = _1 as isize;
_20.0.1 = _14.0;
_20.0.2 = -_5;
SetDiscriminant(_19, 2);
_15 = -_20.3;
place!(Field::<(i32, bool, bool, f64)>(Variant(_19, 2), 7)).0 = _6;
_22 = _2;
_20.0.2 = _5;
_18.2 = [_15,_20.3,_15,_15,_15,_20.3,_11];
_3 = !_10;
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_19, 2), 4)).2 = _14.1 as i128;
Goto(bb10)
}
bb10 = {
place!(Field::<[u16; 1]>(Variant(_19, 2), 2)) = [_10];
place!(Field::<(i32, bool, bool, f64)>(Variant(_19, 2), 7)) = (_6, _20.2, _12, _13);
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_19, 2), 4)).0.1 = [_10];
_15 = _20.3 + _20.3;
Call(_20.0.2 = core::intrinsics::transmute(_10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_10 = _3 - _3;
_19 = Adt81::Variant0 { fld0: _4 };
_18.3 = _20.0.2 as i32;
_16 = _15;
_28.2 = RET as u64;
place!(Field::<u8>(Variant(_19, 0), 0)) = _4 ^ _4;
_26 = RET + RET;
RET = _1 as f32;
_20.3 = -_15;
Goto(bb12)
}
bb12 = {
_28.0 = _9;
_20.0.1 = [_2,_2];
_7 = 7296691282664959275_i64 >> _20.3;
_14 = (_20.0.1, 4_usize);
_20.0.0 = [_8,_8,_8];
_20.0.2 = !_5;
_26 = RET;
_24 = _20.3;
_28 = (_9, _1, 1141542859162253333_u64);
_28.2 = _28.1 as u64;
_8 = _6 as i128;
_29.1 = [_6,_6];
_14.0 = [_22,_22];
_25 = !_20.3;
_17 = _29.1;
_18.3 = _6;
_20.1 = [_3];
_29.1 = [_18.3,_18.3];
_15 = _16 ^ _16;
_25 = _18.3 as isize;
_20.2 = _12;
_22 = _2;
_23 = _2;
_34 = _7 as isize;
_20.0.1 = [_22,_22];
_28.2 = _7 as u64;
Call(_5 = core::intrinsics::transmute(_10), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20.0.1 = [_23,_22];
Goto(bb14)
}
bb14 = {
RET = _7 as f32;
_29.2 = !_3;
_8 = _18.1 | _18.1;
_23 = _22;
_28.2 = 13023241927898361008_u64;
_32 = RET - RET;
_22 = _2;
_8 = -_18.1;
_10 = _3 | _29.2;
_38 = _7 * _7;
_36 = [_15,_20.3,_15];
_9 = _28.0 >> _10;
_5 = _20.0.2;
_40 = -_15;
_30 = _12;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(0_usize, 15_usize, Move(_15), 20_usize, Move(_20), 23_usize, Move(_23), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(0_usize, 36_usize, Move(_36), 3_usize, Move(_3), 24_usize, Move(_24), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(0_usize, 10_usize, Move(_10), 12_usize, Move(_12), 38_usize, Move(_38), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(0_usize, 28_usize, Move(_28), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: i16,mut _3: u32,mut _4: i16) -> u16 {
mir! {
type RET = u16;
let _5: i32;
let _6: [char; 2];
let _7: *mut f32;
let _8: f32;
let _9: f64;
let _10: ();
let _11: ();
{
RET = 20873_u16;
_4 = _2 >> RET;
_3 = 139_u8 as u32;
_6 = [_1,_1];
_5 = 348512936_i32;
_3 = 2905862602_u32;
_2 = _4 + _4;
RET = 44274_u16 * 43049_u16;
_1 = '\u{16fb6}';
_3 = 3646590923_u32 ^ 3763551826_u32;
_4 = 317443934464040511510137246907236374100_u128 as i16;
_4 = !_2;
_6 = [_1,_1];
_3 = 1387267582_u32;
_2 = !_4;
_4 = 111_u8 as i16;
_5 = (-158531538222565734313960465302058021585_i128) as i32;
RET = !55848_u16;
RET = 9469_u16;
RET = 46760_u16 + 17035_u16;
_4 = _2 * _2;
_2 = _4;
RET = 29542_u16 * 94_u16;
_2 = -_4;
_3 = 1302687372_u32;
_5 = -203401485_i32;
RET = 38068_u16 & 31040_u16;
Call(RET = fn2(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 1709959312_i32;
_5 = !(-1662760334_i32);
_1 = '\u{102a77}';
_1 = '\u{b03be}';
_4 = -_2;
_7 = core::ptr::addr_of_mut!(_8);
(*_7) = 232372720641049033216521144935257506823_u128 as f32;
_5 = 3258289061384042519_i64 as i32;
_2 = _4;
_3 = 50848558436497359380688338680431137400_i128 as u32;
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = _4 as f32;
_2 = _4;
(*_7) = 196_u8 as f32;
_5 = 84_u8 as i32;
_9 = 29202019781119923672735934149599331856_i128 as f64;
_6 = [_1,_1];
_1 = '\u{c9a2a}';
_6 = [_1,_1];
_3 = 2102490605_u32;
_5 = 252_u8 as i32;
(*_7) = _4 as f32;
(*_7) = 9223372036854775807_isize as f32;
_3 = 3954835672_u32;
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(1_usize, 4_usize, Move(_4), 2_usize, Move(_2), 3_usize, Move(_3), 11_usize, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i16) -> u16 {
mir! {
type RET = u16;
let _2: u64;
let _3: char;
let _4: [isize; 7];
let _5: *const &'static i16;
let _6: isize;
let _7: (&'static [bool; 2], i128, [isize; 7], i32);
let _8: bool;
let _9: (u128, u64, u8, (i32, bool, bool, f64));
let _10: i128;
let _11: i16;
let _12: u16;
let _13: i32;
let _14: i64;
let _15: [isize; 5];
let _16: [char; 7];
let _17: char;
let _18: u128;
let _19: isize;
let _20: isize;
let _21: isize;
let _22: ([i128; 5],);
let _23: *mut (f64, char);
let _24: (&'static *mut f64, [i32; 2], u16);
let _25: isize;
let _26: (u128, u64, u8, (i32, bool, bool, f64));
let _27: bool;
let _28: isize;
let _29: usize;
let _30: (Adt81, Adt41, (f64, usize, char, u64));
let _31: [usize; 1];
let _32: usize;
let _33: *const i128;
let _34: isize;
let _35: [i128; 3];
let _36: f32;
let _37: ();
let _38: ();
{
RET = !15059_u16;
RET = 3547487128798894838_u64 as u16;
RET = !30114_u16;
RET = !33665_u16;
_1 = (-24935_i16) + 7301_i16;
RET = 1086362679_i32 as u16;
RET = 2611990865215346500_u64 as u16;
RET = 81_i8 as u16;
RET = 49616_u16;
RET = !7170_u16;
RET = 53353_u16 + 7818_u16;
RET = 108913664219755154898869386801578248168_i128 as u16;
RET = 57040_u16 >> _1;
RET = 9266_u16;
_2 = 4459147027178413392_u64;
RET = 63_isize as u16;
_2 = 2009395477_u32 as u64;
_2 = !17018777302277217490_u64;
_2 = 12714327315081976388_u64;
Goto(bb1)
}
bb1 = {
RET = !12253_u16;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,116_isize,(-9223372036854775808_isize),105_isize,(-9223372036854775808_isize)];
_2 = !7492758932887923324_u64;
_3 = '\u{a6786}';
_7.1 = 57373558389680386594098199806119667065_i128;
_7.1 = 1_usize as i128;
_3 = '\u{fa789}';
_3 = '\u{5f56d}';
_6 = (-9223372036854775808_isize);
_6 = (-119_i8) as isize;
_2 = 4598146822056669450_u64;
_1 = 107_u8 as i16;
_2 = 16274980456335730793_u64 & 14410272956082190476_u64;
_7.3 = (-537449181_i32) >> _6;
_8 = !false;
RET = 3453_u16 + 12078_u16;
_9.3.2 = !_8;
_9.3.3 = _7.1 as f64;
_9.0 = _6 as u128;
_7.2 = _4;
_9.3.3 = 3072310830_u32 as f64;
_9.3.0 = -_7.3;
Call(_7.1 = core::intrinsics::transmute(_9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = -_7.1;
_2 = 7239744000187624970_u64;
_9.0 = 227707252974480983937821873403852207020_u128;
_10 = !_7.1;
_2 = 17366772498219458360_u64 << _9.3.0;
_9.3.1 = _9.0 != _9.0;
match _9.0 {
0 => bb1,
1 => bb3,
2 => bb4,
227707252974480983937821873403852207020 => bb6,
_ => bb5
}
}
bb3 = {
RET = !12253_u16;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,116_isize,(-9223372036854775808_isize),105_isize,(-9223372036854775808_isize)];
_2 = !7492758932887923324_u64;
_3 = '\u{a6786}';
_7.1 = 57373558389680386594098199806119667065_i128;
_7.1 = 1_usize as i128;
_3 = '\u{fa789}';
_3 = '\u{5f56d}';
_6 = (-9223372036854775808_isize);
_6 = (-119_i8) as isize;
_2 = 4598146822056669450_u64;
_1 = 107_u8 as i16;
_2 = 16274980456335730793_u64 & 14410272956082190476_u64;
_7.3 = (-537449181_i32) >> _6;
_8 = !false;
RET = 3453_u16 + 12078_u16;
_9.3.2 = !_8;
_9.3.3 = _7.1 as f64;
_9.0 = _6 as u128;
_7.2 = _4;
_9.3.3 = 3072310830_u32 as f64;
_9.3.0 = -_7.3;
Call(_7.1 = core::intrinsics::transmute(_9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
RET = 6476_u16;
RET = !13262_u16;
RET = 48575_u16;
_3 = '\u{73820}';
_9.2 = _1 as u8;
_9.3.0 = _7.3;
_2 = 1559111848726235323_u64 - 11157800280468660909_u64;
_9.1 = _2 * _2;
_3 = '\u{44c67}';
_1 = -(-29681_i16);
_3 = '\u{9ed65}';
_3 = '\u{fd337}';
_10 = _7.1 - _7.1;
_9.1 = _2;
_12 = !RET;
_9.3.1 = _9.3.3 < _9.3.3;
_2 = _3 as u64;
_9.3.1 = !_9.3.2;
_1 = 2_usize as i16;
Call(RET = core::intrinsics::bswap(_12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = '\u{798e2}';
_7.1 = !_10;
_7.1 = !_10;
_10 = _1 as i128;
_8 = _12 > RET;
_12 = _9.3.3 as u16;
Call(_7.1 = fn3(_4, _3, _9.3, _4, _4, _10, _7.2, _8, _4, _6, _9.3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9.1 = !_2;
_11 = _1;
_7.3 = _9.3.0 >> _7.1;
RET = _12;
_10 = _7.1 - _7.1;
_12 = !RET;
_7.2 = [_6,_6,_6,_6,_6,_6,_6];
_17 = _3;
_9.1 = 391929276_u32 as u64;
RET = _12;
_15 = [_6,_6,_6,_6,_6];
_7.1 = _8 as i128;
_9.0 = !309715283719328797355366388915656202136_u128;
_13 = _11 as i32;
Goto(bb9)
}
bb9 = {
_13 = -_7.3;
_14 = _8 as i64;
_14 = !7507254218748306054_i64;
_9.2 = _8 as u8;
_13 = !_7.3;
_18 = _9.0 * _9.0;
RET = _12;
_16 = [_17,_3,_17,_17,_17,_17,_17];
_18 = _9.0 + _9.0;
_11 = -_1;
_9.2 = 238_u8 | 130_u8;
_2 = 2820174912_u32 as u64;
_16 = [_3,_3,_3,_17,_17,_17,_17];
_9.2 = !112_u8;
_9.0 = _18;
_14 = (-81225899679196702_i64);
_9.3.3 = _2 as f64;
_12 = RET;
_14 = 7293550367895384601_i64;
_15 = [_6,_6,_6,_6,_6];
_12 = _13 as u16;
_15 = [_6,_6,_6,_6,_6];
_20 = 8515653942031099084_usize as isize;
_16 = [_3,_17,_3,_17,_3,_17,_3];
_9.3.0 = _13 >> _9.2;
_19 = _6 + _6;
match _14 {
0 => bb8,
1 => bb5,
2 => bb3,
3 => bb4,
7293550367895384601 => bb11,
_ => bb10
}
}
bb10 = {
_10 = -_7.1;
_2 = 7239744000187624970_u64;
_9.0 = 227707252974480983937821873403852207020_u128;
_10 = !_7.1;
_2 = 17366772498219458360_u64 << _9.3.0;
_9.3.1 = _9.0 != _9.0;
match _9.0 {
0 => bb1,
1 => bb3,
2 => bb4,
227707252974480983937821873403852207020 => bb6,
_ => bb5
}
}
bb11 = {
_24.1 = [_9.3.0,_13];
_21 = _6;
_1 = _14 as i16;
_9.3.1 = _8 | _9.3.2;
_18 = _10 as u128;
_10 = -_7.1;
_9.3.2 = _18 >= _18;
_24.1 = [_13,_9.3.0];
_22.0 = [_10,_7.1,_7.1,_7.1,_7.1];
_26.1 = _7.1 as u64;
_26.3.1 = !_9.3.2;
_9.3.3 = _10 as f64;
_1 = _11 * _11;
_21 = 109888330_u32 as isize;
_25 = _14 as isize;
_20 = _6;
Goto(bb12)
}
bb12 = {
_9.1 = !_26.1;
_25 = _6;
_9.3.3 = _9.2 as f64;
_29 = _1 as usize;
_1 = -_11;
_26.3.0 = -_9.3.0;
_11 = _7.1 as i16;
_28 = _19 ^ _6;
_15 = [_28,_25,_6,_28,_6];
_9.3.3 = _1 as f64;
_26.2 = _7.1 as u8;
_9.3.0 = _26.3.1 as i32;
_30.2 = (_9.3.3, _29, _3, _26.1);
_26.3.2 = _26.3.1;
_8 = !_9.3.2;
_20 = !_28;
Goto(bb13)
}
bb13 = {
_21 = _28 * _28;
_7.1 = _9.2 as i128;
_10 = _7.1 + _7.1;
_26 = (_18, _9.1, _9.2, _9.3);
_30.2.0 = _9.3.3 - _9.3.3;
RET = _19 as u16;
_26.3.2 = !_8;
_26 = (_18, _30.2.3, _9.2, _9.3);
_26.3.2 = _8 | _8;
RET = _13 as u16;
_10 = _7.1;
_31 = [_30.2.1];
_9.0 = _26.0;
_17 = _30.2.2;
_12 = !RET;
_9.0 = _26.0 * _18;
_25 = _21 << _26.3.0;
Goto(bb14)
}
bb14 = {
Call(_37 = dump_var(2_usize, 18_usize, Move(_18), 1_usize, Move(_1), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_37 = dump_var(2_usize, 28_usize, Move(_28), 16_usize, Move(_16), 29_usize, Move(_29), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(2_usize, 15_usize, Move(_15), 3_usize, Move(_3), 11_usize, Move(_11), 38_usize, _38), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [isize; 7],mut _2: char,mut _3: (i32, bool, bool, f64),mut _4: [isize; 7],mut _5: [isize; 7],mut _6: i128,mut _7: [isize; 7],mut _8: bool,mut _9: [isize; 7],mut _10: isize,mut _11: (i32, bool, bool, f64)) -> i128 {
mir! {
type RET = i128;
let _12: [i128; 3];
let _13: f64;
let _14: usize;
let _15: Adt22;
let _16: (u32, u128, u64);
let _17: char;
let _18: *mut *const *mut f32;
let _19: (&'static i64, &'static bool);
let _20: ();
let _21: ();
{
_3.3 = (-103_i8) as f64;
_4 = [_10,_10,_10,_10,_10,_10,_10];
_12 = [_6,_6,_6];
_8 = !_3.1;
_10 = _6 as isize;
_8 = _11.2 >= _11.1;
RET = -_6;
_9 = [_10,_10,_10,_10,_10,_10,_10];
_11.3 = _3.3;
_3 = _11;
_7 = _1;
RET = _6 ^ _6;
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.2 = !_3.2;
_11.3 = _3.3;
_1 = _5;
_4 = [_10,_10,_10,_10,_10,_10,_10];
_3.0 = -_11.0;
_11.0 = _3.0 - _3.0;
_1 = [_10,_10,_10,_10,_10,_10,_10];
_11.2 = !_8;
_4 = [_10,_10,_10,_10,_10,_10,_10];
_11 = _3;
_8 = !_3.2;
_13 = 613621719_u32 as f64;
_11.0 = !_3.0;
_11 = (_3.0, _3.1, _3.2, _3.3);
_10 = 67_isize + 38_isize;
_3.0 = _11.0;
_3.0 = RET as i32;
_4 = [_10,_10,_10,_10,_10,_10,_10];
_2 = '\u{4e9f8}';
_10 = _3.2 as isize;
RET = !_6;
Call(RET = fn4(_11, _9, _3, _11.0, _7, _7, _7, _5, _5, _5, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.1 = _8;
_15 = Adt22::Variant1 { fld0: 1652749442231304363_u64,fld1: _3,fld2: _10,fld3: 273077761396771830409573499350411929559_u128 };
_16.0 = 3465613518_u32 ^ 3082927829_u32;
_16.1 = _6 as u128;
place!(Field::<(i32, bool, bool, f64)>(Variant(_15, 1), 1)) = _3;
_16.1 = 260263135299749299381470255852820366126_u128;
_6 = !RET;
_3 = _11;
_14 = 4_usize;
_17 = _2;
place!(Field::<(i32, bool, bool, f64)>(Variant(_15, 1), 1)).0 = _11.0;
place!(Field::<u128>(Variant(_15, 1), 3)) = (-3721300073939584500_i64) as u128;
Goto(bb3)
}
bb3 = {
Call(_20 = dump_var(3_usize, 14_usize, Move(_14), 17_usize, Move(_17), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_20 = dump_var(3_usize, 6_usize, Move(_6), 8_usize, Move(_8), 21_usize, _21, 21_usize, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (i32, bool, bool, f64),mut _2: [isize; 7],mut _3: (i32, bool, bool, f64),mut _4: i32,mut _5: [isize; 7],mut _6: [isize; 7],mut _7: [isize; 7],mut _8: [isize; 7],mut _9: [isize; 7],mut _10: [isize; 7],mut _11: [isize; 7],mut _12: [isize; 7]) -> i128 {
mir! {
type RET = i128;
let _13: [isize; 7];
let _14: u8;
let _15: Adt85;
let _16: i8;
let _17: usize;
let _18: isize;
let _19: f64;
let _20: [i16; 7];
let _21: f32;
let _22: ([i128; 3], [char; 2], i16, ([i128; 5],));
let _23: u32;
let _24: u16;
let _25: char;
let _26: [i16; 7];
let _27: (u32, u128, u64);
let _28: i32;
let _29: f64;
let _30: isize;
let _31: [u16; 1];
let _32: ();
let _33: ();
{
_1.0 = _4 + _4;
_11 = _9;
RET = (-32658793331136733434722876884956335088_i128);
_3.2 = _3.1 ^ _3.1;
_9 = _5;
_8 = _5;
RET = (-73092425725812659899812303887557059717_i128) & (-63359983884689543048641878010843698713_i128);
_1.1 = _1.0 >= _4;
_3.0 = _1.0 ^ _4;
_8 = _9;
_1.3 = -_3.3;
_10 = _5;
RET = 166404125834045012247701182573787600058_i128 | 35618529123266511270812875342938749399_i128;
_4 = _1.0 - _1.0;
_12 = [(-43_isize),9223372036854775807_isize,9223372036854775807_isize,1_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-112_isize)];
_3.1 = !_3.2;
Call(_3 = fn5(_6, _5, _7, _5, _5, _11, _8, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.0 = _4;
_1.1 = _3.1 == _3.1;
_7 = _9;
_4 = !_1.0;
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_16 = !(-87_i8);
_8 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,119_isize,47_isize,(-9223372036854775808_isize),(-119_isize)];
_12 = _7;
_5 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,60_isize,9223372036854775807_isize,9223372036854775807_isize];
_14 = 240_u8 - 103_u8;
_3.2 = _1.1;
_14 = !55_u8;
_18 = 1861607646738113485_usize as isize;
_3.0 = _4;
_4 = _3.0;
_8 = _9;
_4 = _3.0 - _3.0;
_17 = 4277826978005159536_usize & 7_usize;
RET = (-69854812462182041312214510121172263916_i128) ^ (-37888286065836055094161764272800608171_i128);
_14 = _4 as u8;
_9 = [_18,_18,_18,_18,_18,_18,_18];
_11 = _6;
_17 = !7599338186271571145_usize;
_19 = _3.3 - _3.3;
_19 = -_3.3;
_10 = [_18,_18,_18,_18,_18,_18,_18];
_20 = [12776_i16,9911_i16,15664_i16,25904_i16,(-2948_i16),(-1384_i16),(-29578_i16)];
_1.1 = !_3.2;
Goto(bb2)
}
bb2 = {
_11 = [_18,_18,_18,_18,_18,_18,_18];
_3 = (_4, _1.1, _1.1, _19);
_13 = [_18,_18,_18,_18,_18,_18,_18];
_1.1 = _3.2 & _3.1;
_22.2 = (-5072_i16) | (-1967_i16);
_3.3 = _1.3;
_22.3.0 = [RET,RET,RET,RET,RET];
RET = (-122911942452331181865646485733150681922_i128) | 115824223956668499936089396224439737916_i128;
_22.0 = [RET,RET,RET];
_1.1 = !_3.1;
Goto(bb3)
}
bb3 = {
_16 = 74_i8;
_11 = [_18,_18,_18,_18,_18,_18,_18];
_5 = _6;
_22.2 = (-28495_i16) >> _3.0;
_21 = 6518_u16 as f32;
_2 = [_18,_18,_18,_18,_18,_18,_18];
RET = -(-128464205164846081723951181357304685346_i128);
_1.1 = _3.2 > _3.1;
_23 = 15875523453117729466753059469740380809_u128 as u32;
_12 = [_18,_18,_18,_18,_18,_18,_18];
_25 = '\u{34e0a}';
_12 = _5;
RET = 76036050543267173863939807189526815139_i128;
_8 = [_18,_18,_18,_18,_18,_18,_18];
_27.1 = 281770690531365217907573372997170153807_u128 << _14;
_27.1 = 30253442833076481829977431903638627012_u128 << _4;
Call(_22.3 = fn6(_1.0, _3, _1, _1.1, _3.1, _3.0, _3.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1.0 = _4;
_27.0 = _3.1 as u32;
_10 = [_18,_18,_18,_18,_18,_18,_18];
_17 = 5_usize << _14;
RET = 20274618663015310115037019896439538680_i128 << _3.0;
_27.2 = 3519273112182318778_u64 + 61884273777804779_u64;
_22.1 = [_25,_25];
_6 = [_18,_18,_18,_18,_18,_18,_18];
_26 = [_22.2,_22.2,_22.2,_22.2,_22.2,_22.2,_22.2];
_1 = (_4, _3.2, _3.1, _19);
_29 = _1.3 + _3.3;
_27.0 = _23;
_21 = _17 as f32;
_20 = _26;
_18 = (-63_isize) + 9223372036854775807_isize;
_1.1 = _3.2;
_4 = !_1.0;
_24 = 11442_u16;
Goto(bb5)
}
bb5 = {
Call(_32 = dump_var(4_usize, 5_usize, Move(_5), 16_usize, Move(_16), 20_usize, Move(_20), 27_usize, Move(_27)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_32 = dump_var(4_usize, 24_usize, Move(_24), 17_usize, Move(_17), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_32 = dump_var(4_usize, 6_usize, Move(_6), 14_usize, Move(_14), 13_usize, Move(_13), 33_usize, _33), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [isize; 7],mut _2: [isize; 7],mut _3: [isize; 7],mut _4: [isize; 7],mut _5: [isize; 7],mut _6: [isize; 7],mut _7: [isize; 7],mut _8: [isize; 7]) -> (i32, bool, bool, f64) {
mir! {
type RET = (i32, bool, bool, f64);
let _9: bool;
let _10: Adt81;
let _11: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _12: u16;
let _13: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _14: isize;
let _15: Adt85;
let _16: [isize; 7];
let _17: (u16,);
let _18: isize;
let _19: f64;
let _20: ();
let _21: ();
{
RET.3 = 22_i8 as f64;
RET.2 = RET.3 <= RET.3;
RET.0 = 2077616595_i32;
RET.3 = 1100625801_u32 as f64;
_3 = _5;
_2 = [9223372036854775807_isize,9223372036854775807_isize,86_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-64_isize)];
_6 = _8;
_3 = [(-54_isize),(-15_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
RET.3 = 8_u8 as f64;
_2 = [(-41_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),27_isize];
RET.2 = RET.0 != RET.0;
_4 = _7;
RET.1 = !RET.2;
RET.1 = RET.2 < RET.2;
Goto(bb1)
}
bb1 = {
_6 = [(-9223372036854775808_isize),(-52_isize),(-9223372036854775808_isize),31_isize,9223372036854775807_isize,9223372036854775807_isize,(-45_isize)];
RET.1 = !RET.2;
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),20_isize,9223372036854775807_isize,97_isize];
_11.2 = !RET.2;
_8 = _5;
_8 = _4;
RET.3 = 3411939086749494929_u64 as f64;
RET.2 = !_11.2;
_8 = [(-9223372036854775808_isize),(-126_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
RET.2 = !_11.2;
_11.0.3.0 = [170008096273633639439248775452916865845_i128,105245308697937858016456712537022850959_i128,(-112072806610105029229097259766580696575_i128),(-40271471621193818620141792269033753117_i128),(-15739156499028707439052362468461783242_i128)];
_4 = [9223372036854775807_isize,(-52_isize),(-14_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-45_isize)];
RET.1 = RET.2;
_1 = [(-8_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),19_isize,(-9223372036854775808_isize)];
_11.0.0 = [111995318148027787547472716839764220409_i128,128584111385635910289793118591109688386_i128,60015161727904814344577373393148193893_i128];
_8 = [(-41_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-92_isize),87_isize,(-89_isize)];
_8 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-3_isize),24_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-97_isize)];
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),102_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),40_isize,9223372036854775807_isize];
_4 = _6;
_6 = [9223372036854775807_isize,114_isize,(-9223372036854775808_isize),68_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_11.1 = [17287_u16];
_8 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-60_isize),9223372036854775807_isize,(-4_isize),(-9223372036854775808_isize)];
_11.0.1 = ['\u{f24a1}','\u{db0de}'];
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-109_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5 = _3;
_14 = 9223372036854775807_isize;
_13.0.2 = -32312_i16;
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
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
_13.0.3 = _11.0.3;
_11.0.1 = ['\u{9223}','\u{589e8}'];
_13.0 = (_11.0.0, _11.0.1, 22937_i16, _11.0.3);
_11.2 = RET.2;
RET.1 = RET.2 & _11.2;
Goto(bb7)
}
bb7 = {
RET.3 = _14 as f64;
_13.0.3 = (_11.0.3.0,);
_7 = [_14,_14,_14,_14,_14,_14,_14];
RET.1 = _13.0.2 > _13.0.2;
_13.0.1 = _11.0.1;
_13.0.3.0 = [158382939462904848958814216891579167664_i128,(-47262860712945988778396383756554716950_i128),(-151490969362750055204384887711270380432_i128),(-99618907354938990548077478147145169800_i128),(-122854616226997730906957486554031784180_i128)];
_8 = _4;
_11.0 = (_13.0.0, _13.0.1, _13.0.2, _13.0.3);
_9 = RET.1 | RET.1;
RET.1 = !_9;
_5 = _6;
_13 = (_11.0, _11.1, RET.1, _14);
_13.1 = [34613_u16];
_13.1 = _11.1;
_1 = [_13.3,_14,_14,_14,_13.3,_14,_14];
_11.0.2 = 2_usize as i16;
Goto(bb8)
}
bb8 = {
Call(_20 = dump_var(5_usize, 9_usize, Move(_9), 14_usize, Move(_14), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_20 = dump_var(5_usize, 2_usize, Move(_2), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i32,mut _2: (i32, bool, bool, f64),mut _3: (i32, bool, bool, f64),mut _4: bool,mut _5: bool,mut _6: i32,mut _7: bool) -> ([i128; 5],) {
mir! {
type RET = ([i128; 5],);
let _8: (&'static i64, &'static bool);
let _9: &'static (u128, u64, u8, (i32, bool, bool, f64));
let _10: *const u16;
let _11: Adt79;
let _12: [u16; 1];
let _13: ([char; 2], usize);
let _14: u16;
let _15: &'static i64;
let _16: *mut (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _17: *mut (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _18: u8;
let _19: ([i128; 5],);
let _20: bool;
let _21: *mut (f64, char);
let _22: f32;
let _23: [isize; 5];
let _24: u16;
let _25: char;
let _26: f32;
let _27: &'static *const i64;
let _28: f32;
let _29: *const *mut f32;
let _30: *const &'static (u128, u64, u8, (i32, bool, bool, f64));
let _31: Adt22;
let _32: f64;
let _33: bool;
let _34: (f64, char);
let _35: ();
let _36: ();
{
_3.0 = 976345886260851510_i64 as i32;
RET.0 = [(-55204458498281743086125472712335768434_i128),78832101207597255844719439801225016809_i128,62296908100165322506833176688084151938_i128,(-114372511323054348825030148269611419056_i128),130394422092622501285674636060570292106_i128];
RET.0 = [(-38015157506606102554791917980169318283_i128),154313690725078901279842783751570071527_i128,(-167159185856879681707056504244683219830_i128),84385846694461497790196353234270849711_i128,129449458844219041842537297830427695169_i128];
_1 = !_6;
_6 = 94357114257445853342545332679726384331_i128 as i32;
_2 = (_1, _3.1, _3.1, _3.3);
_2.1 = !_7;
Goto(bb1)
}
bb1 = {
_2.1 = _3.1;
_4 = _7;
_6 = _1;
_3.3 = 8986901691061990983_usize as f64;
_2.0 = 231_u8 as i32;
_7 = _4;
_6 = _1;
_3.0 = _6 - _1;
_3.1 = !_2.2;
_4 = !_2.1;
_3.0 = _1 ^ _1;
_3.1 = !_2.2;
_8.1 = &_7;
RET.0 = [(-166799063391107778151308665794332588244_i128),(-44950589833823628350397703418956554116_i128),29543350474354042459837801624887631729_i128,(-125391703377690206988926655860723629191_i128),(-65820812230850637274354916392213284797_i128)];
RET.0 = [(-61529816774933900990372433543784286084_i128),(-163991018557823170215633234417405588158_i128),107579109023844508628375703302169184111_i128,72722360208142535652653131716413754900_i128,(-39724122621336276632328688202415629794_i128)];
Call(_11 = fn7(Move(_8.1), _7, _3, _4, _2.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _3;
place!(Field::<(u8, i8, bool)>(Variant(_11, 0), 3)) = (235_u8, 40_i8, Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).2);
_6 = !_3.0;
RET.0 = [150131567888900440029829612454872003768_i128,105663577447214445107047431957399468258_i128,(-103755085448413862147397660416805459100_i128),(-35711976605357351952796159122857549105_i128),(-5657223646804591856622977437869282832_i128)];
_5 = !_2.1;
_10 = core::ptr::addr_of!(_14);
place!(Field::<[isize; 5]>(Variant(_11, 0), 2)) = [Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3];
RET = (Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).0.3.0,);
SetDiscriminant(_11, 1);
_13.1 = 299333576883832763_usize;
_6 = !_3.0;
_10 = core::ptr::addr_of!(_14);
_13.0 = ['\u{70342}','\u{530c9}'];
RET.0 = [(-86505254169374665703324555254262806197_i128),66108322233006782405491473088709109493_i128,28400659424181511840432922593105864959_i128,(-98849055414895394767964687806843712319_i128),42354362214997682844917711110574118737_i128];
place!(Field::<i8>(Variant(_11, 1), 1)) = 104_i8 << _2.0;
_14 = !30560_u16;
_8.1 = &_3.1;
_13.1 = 5865914989479190765_usize;
match _13.1 {
5865914989479190765 => bb4,
_ => bb3
}
}
bb3 = {
_2.1 = _3.1;
_4 = _7;
_6 = _1;
_3.3 = 8986901691061990983_usize as f64;
_2.0 = 231_u8 as i32;
_7 = _4;
_6 = _1;
_3.0 = _6 - _1;
_3.1 = !_2.2;
_4 = !_2.1;
_3.0 = _1 ^ _1;
_3.1 = !_2.2;
_8.1 = &_7;
RET.0 = [(-166799063391107778151308665794332588244_i128),(-44950589833823628350397703418956554116_i128),29543350474354042459837801624887631729_i128,(-125391703377690206988926655860723629191_i128),(-65820812230850637274354916392213284797_i128)];
RET.0 = [(-61529816774933900990372433543784286084_i128),(-163991018557823170215633234417405588158_i128),107579109023844508628375703302169184111_i128,72722360208142535652653131716413754900_i128,(-39724122621336276632328688202415629794_i128)];
Call(_11 = fn7(Move(_8.1), _7, _3, _4, _2.2), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_3.1 = _5 == _7;
_12 = [(*_10)];
RET.0 = [(-12488788132021393425298056027919618038_i128),(-4560194658716068696803373486888714874_i128),54538510315387853496524528584850825432_i128,22070910537335897671674528882303762428_i128,112290548906690579588959793173395836034_i128];
_12 = [(*_10)];
_3.0 = _4 as i32;
_13.0 = ['\u{f781b}','\u{7108a}'];
_3.2 = !_5;
_3.2 = _7;
_2.3 = -_3.3;
_13.0 = ['\u{5e342}','\u{10f60c}'];
_8.1 = &_2.1;
_18 = 77690832037900397674619211382265681512_u128 as u8;
_19 = (RET.0,);
place!(Field::<[isize; 3]>(Variant(_11, 1), 0)) = [35_isize,9223372036854775807_isize,9223372036854775807_isize];
_4 = !_7;
_5 = _7;
RET = _19;
_8.1 = &_3.2;
RET = (_19.0,);
_20 = !_7;
_2.1 = _20;
match _13.1 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
5865914989479190765 => bb11,
_ => bb10
}
}
bb5 = {
_2.1 = _3.1;
_4 = _7;
_6 = _1;
_3.3 = 8986901691061990983_usize as f64;
_2.0 = 231_u8 as i32;
_7 = _4;
_6 = _1;
_3.0 = _6 - _1;
_3.1 = !_2.2;
_4 = !_2.1;
_3.0 = _1 ^ _1;
_3.1 = !_2.2;
_8.1 = &_7;
RET.0 = [(-166799063391107778151308665794332588244_i128),(-44950589833823628350397703418956554116_i128),29543350474354042459837801624887631729_i128,(-125391703377690206988926655860723629191_i128),(-65820812230850637274354916392213284797_i128)];
RET.0 = [(-61529816774933900990372433543784286084_i128),(-163991018557823170215633234417405588158_i128),107579109023844508628375703302169184111_i128,72722360208142535652653131716413754900_i128,(-39724122621336276632328688202415629794_i128)];
Call(_11 = fn7(Move(_8.1), _7, _3, _4, _2.2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_2 = _3;
place!(Field::<(u8, i8, bool)>(Variant(_11, 0), 3)) = (235_u8, 40_i8, Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).2);
_6 = !_3.0;
RET.0 = [150131567888900440029829612454872003768_i128,105663577447214445107047431957399468258_i128,(-103755085448413862147397660416805459100_i128),(-35711976605357351952796159122857549105_i128),(-5657223646804591856622977437869282832_i128)];
_5 = !_2.1;
_10 = core::ptr::addr_of!(_14);
place!(Field::<[isize; 5]>(Variant(_11, 0), 2)) = [Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3,Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).3];
RET = (Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(_11, 0), 4).0.3.0,);
SetDiscriminant(_11, 1);
_13.1 = 299333576883832763_usize;
_6 = !_3.0;
_10 = core::ptr::addr_of!(_14);
_13.0 = ['\u{70342}','\u{530c9}'];
RET.0 = [(-86505254169374665703324555254262806197_i128),66108322233006782405491473088709109493_i128,28400659424181511840432922593105864959_i128,(-98849055414895394767964687806843712319_i128),42354362214997682844917711110574118737_i128];
place!(Field::<i8>(Variant(_11, 1), 1)) = 104_i8 << _2.0;
_14 = !30560_u16;
_8.1 = &_3.1;
_13.1 = 5865914989479190765_usize;
match _13.1 {
5865914989479190765 => bb4,
_ => bb3
}
}
bb7 = {
_2.1 = _3.1;
_4 = _7;
_6 = _1;
_3.3 = 8986901691061990983_usize as f64;
_2.0 = 231_u8 as i32;
_7 = _4;
_6 = _1;
_3.0 = _6 - _1;
_3.1 = !_2.2;
_4 = !_2.1;
_3.0 = _1 ^ _1;
_3.1 = !_2.2;
_8.1 = &_7;
RET.0 = [(-166799063391107778151308665794332588244_i128),(-44950589833823628350397703418956554116_i128),29543350474354042459837801624887631729_i128,(-125391703377690206988926655860723629191_i128),(-65820812230850637274354916392213284797_i128)];
RET.0 = [(-61529816774933900990372433543784286084_i128),(-163991018557823170215633234417405588158_i128),107579109023844508628375703302169184111_i128,72722360208142535652653131716413754900_i128,(-39724122621336276632328688202415629794_i128)];
Call(_11 = fn7(Move(_8.1), _7, _3, _4, _2.2), ReturnTo(bb2), UnwindUnreachable())
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
_19 = RET;
_3.0 = _13.1 as i32;
_6 = !_1;
_6 = !_1;
_3.2 = _3.1;
_19 = (RET.0,);
_13.1 = !3_usize;
_7 = !_20;
_20 = !_4;
SetDiscriminant(_11, 1);
place!(Field::<[isize; 3]>(Variant(_11, 1), 0)) = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6 = !_2.0;
_13.0 = ['\u{6867a}','\u{95dfe}'];
_7 = _20;
_3.2 = _20 ^ _2.1;
_3.0 = (-9223372036854775808_isize) as i32;
_2.1 = !_20;
_10 = core::ptr::addr_of!(_24);
RET = _19;
_24 = !_14;
_3.3 = _2.3 + _2.3;
(*_10) = 7154799936821650023_i64 as u16;
place!(Field::<i8>(Variant(_11, 1), 1)) = _13.1 as i8;
_2.1 = _4;
Goto(bb12)
}
bb12 = {
RET = (_19.0,);
_19 = (RET.0,);
_14 = _24;
_7 = _3.1 <= _2.1;
_8.1 = &_3.2;
_2.0 = _6;
_6 = !_2.0;
_3.0 = _6 - _6;
_3.3 = -_2.3;
place!(Field::<i8>(Variant(_11, 1), 1)) = 245352258893522656851777658483383208146_u128 as i8;
_13.0 = ['\u{59721}','\u{43dfe}'];
_7 = !_3.1;
_6 = _3.0 >> _2.0;
Goto(bb13)
}
bb13 = {
_3.0 = _6 << _6;
RET = (_19.0,);
_2.1 = !_4;
_2.1 = _20 & _4;
_26 = _13.1 as f32;
_30 = core::ptr::addr_of!(_9);
_28 = 87860354996400087364789969750338886169_u128 as f32;
_22 = -_28;
_26 = -_28;
_26 = _22 + _28;
_25 = '\u{1f781}';
_18 = !121_u8;
place!(Field::<[isize; 3]>(Variant(_11, 1), 0)) = [9223372036854775807_isize,(-41_isize),9223372036854775807_isize];
_13.1 = 2201502904_u32 as usize;
_3.1 = !_20;
_2.0 = !_6;
_24 = _14;
_2.3 = Field::<i8>(Variant(_11, 1), 1) as f64;
_32 = _3.3;
_3.3 = _2.3;
_25 = '\u{34103}';
_26 = -_22;
Goto(bb14)
}
bb14 = {
_3.2 = _3.1 >= _4;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(6_usize, 14_usize, Move(_14), 7_usize, Move(_7), 25_usize, Move(_25), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(6_usize, 1_usize, Move(_1), 4_usize, Move(_4), 36_usize, _36, 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: &'static bool,mut _2: bool,mut _3: (i32, bool, bool, f64),mut _4: bool,mut _5: bool) -> Adt79 {
mir! {
type RET = Adt79;
let _6: f64;
let _7: ();
let _8: ();
{
_3.3 = 13_i8 as f64;
_3.0 = 668935507_i32;
_3.3 = 5326_i16 as f64;
_5 = !_4;
_3.1 = _2 < _2;
_4 = _3.1 > _5;
Call(RET = fn8(_4, _4, _2, _3, _3.1, _4, _5, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<[i32; 2]>(Variant(RET, 0), 1)) = [Field::<i32>(Variant(RET, 0), 5),_3.0];
Goto(bb2)
}
bb2 = {
Call(_7 = dump_var(7_usize, 5_usize, Move(_5), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: (i32, bool, bool, f64),mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> Adt79 {
mir! {
type RET = Adt79;
let _9: (Adt22, [u16; 1], u128, i32);
let _10: ([i128; 5],);
let _11: f64;
let _12: f64;
let _13: &'static i16;
let _14: [char; 2];
let _15: bool;
let _16: *mut (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _17: i8;
let _18: isize;
let _19: char;
let _20: f32;
let _21: (u32, u128, u64);
let _22: isize;
let _23: *mut (f64, char);
let _24: f64;
let _25: (Adt81, Adt41, (f64, usize, char, u64));
let _26: &'static *mut &'static i16;
let _27: &'static i64;
let _28: f32;
let _29: &'static [bool; 2];
let _30: u8;
let _31: f64;
let _32: [isize; 3];
let _33: usize;
let _34: u16;
let _35: *mut (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _36: i16;
let _37: ([char; 2], usize);
let _38: (([i128; 5],),);
let _39: &'static (u128, u64, u8, (i32, bool, bool, f64));
let _40: f64;
let _41: &'static bool;
let _42: [isize; 5];
let _43: f32;
let _44: f64;
let _45: ([i128; 5],);
let _46: *mut (f64, char);
let _47: f32;
let _48: f64;
let _49: &'static &'static *mut f64;
let _50: *mut (f64, char);
let _51: isize;
let _52: Adt79;
let _53: f32;
let _54: i64;
let _55: ((Adt22, [u16; 1], u128, i32), u128, i128, f64);
let _56: (u128, u64, u8, (i32, bool, bool, f64));
let _57: ();
let _58: ();
{
_7 = _2 | _1;
_4.3 = (-8500095766237300963_i64) as f64;
_4.1 = _2 ^ _3;
_3 = _5 & _1;
_9.2 = 75042704684162455078622270365924091422_u128 >> _4.0;
_11 = -_4.3;
_10.0 = [(-46945953238096924064022374577293593200_i128),(-152121176352439196411099787578121996049_i128),(-40905591970526293399028305200897408237_i128),85821727820632940376557713829020513598_i128,101869679968458856006538638771344562201_i128];
_2 = _8;
_12 = _11 + _4.3;
_6 = _7;
_9.3 = _4.0;
_2 = _3 | _6;
_8 = _7 == _4.1;
_5 = _6 == _1;
_4.3 = -_11;
_3 = _6 & _2;
_4.1 = _6;
_10.0 = [(-168452803400626918376809838290195535658_i128),(-131070711575340569874399530492610815534_i128),(-37345487692460623504451606819320499137_i128),(-97783865673480492084554847672935749359_i128),141728516707028460980177668159364351059_i128];
_12 = 234_u8 as f64;
_4.3 = _12 * _12;
_5 = _2;
_2 = _3;
_9.3 = !_4.0;
Goto(bb1)
}
bb1 = {
_2 = !_3;
_9.1 = [12775_u16];
_15 = _3 <= _4.1;
_8 = _3;
_4.3 = _9.2 as f64;
_6 = !_4.1;
_6 = _2 >= _2;
_6 = _1;
_4.3 = -_11;
_6 = !_2;
Goto(bb2)
}
bb2 = {
_21.2 = 2649214685026646981_u64;
_22 = 5209_u16 as isize;
match _21.2 {
0 => bb1,
1 => bb3,
2 => bb4,
2649214685026646981 => bb6,
_ => bb5
}
}
bb3 = {
_2 = !_3;
_9.1 = [12775_u16];
_15 = _3 <= _4.1;
_8 = _3;
_4.3 = _9.2 as f64;
_6 = !_4.1;
_6 = _2 >= _2;
_6 = _1;
_4.3 = -_11;
_6 = !_2;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_21.0 = 935376428_u32;
_21.2 = _4.3 as u64;
_9.0 = Adt22::Variant1 { fld0: _21.2,fld1: _4,fld2: _22,fld3: _9.2 };
_14 = ['\u{bba28}','\u{74eab}'];
_21.1 = Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).3 as u128;
Goto(bb7)
}
bb7 = {
SetDiscriminant(_9.0, 1);
_19 = '\u{3bd35}';
_21.0 = !2009153461_u32;
_22 = -9223372036854775807_isize;
_5 = !_4.1;
_7 = _1 != _8;
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)).1 = _4.1 == _3;
_25.2 = (_11, 3_usize, _19, _21.2);
_20 = 151276835519895806911515352632727699310_i128 as f32;
place!(Field::<isize>(Variant(_9.0, 1), 2)) = !_22;
_21.1 = !_9.2;
_11 = -_4.3;
_6 = _4.1 <= _15;
_9.1 = [8638_u16];
_4 = (_9.3, _7, _1, _11);
_4.2 = _15 <= _15;
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)).0 = 186_u8 as i32;
_25.2.1 = Field::<isize>(Variant(_9.0, 1), 2) as usize;
_20 = _21.1 as f32;
_8 = _6;
Goto(bb8)
}
bb8 = {
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)) = (_9.3, _8, _2, _12);
_20 = 123_i8 as f32;
_28 = _20;
_17 = _21.2 as i8;
_6 = !_2;
_20 = _21.0 as f32;
_3 = _7;
_9.0 = Adt22::Variant1 { fld0: _21.2,fld1: _4,fld2: _22,fld3: _9.2 };
_9.2 = !Field::<u128>(Variant(_9.0, 1), 3);
_25.2.3 = _21.2 & Field::<u64>(Variant(_9.0, 1), 0);
_9.1 = [37704_u16];
_9.3 = !_4.0;
_12 = _25.2.0 * _25.2.0;
_14 = [_25.2.2,_25.2.2];
Goto(bb9)
}
bb9 = {
_30 = _21.1 as u8;
_34 = !46078_u16;
_6 = _1;
_21.1 = _9.2;
_8 = _4.2 <= Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).1;
_9.2 = Field::<u128>(Variant(_9.0, 1), 3) >> Field::<u128>(Variant(_9.0, 1), 3);
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)) = _4;
Call(place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)).1 = fn9(_5, _7, _3, _15, _1, Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).2, _5, Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).2, _6, _4, Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).2, _6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_4 = Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1);
_6 = !_3;
_25.2.1 = 6_usize;
_24 = Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).3 * _11;
SetDiscriminant(_9.0, 1);
_32 = [_22,_22,_22];
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)).0 = _25.2.3 as i32;
match _25.2.1 {
0 => bb3,
6 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_25.0 = Adt81::Variant0 { fld0: _30 };
_18 = !_22;
SetDiscriminant(_25.0, 1);
_31 = _4.3;
_14 = [_19,_19];
_14 = [_25.2.2,_19];
place!(Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3)).0 = _12 as u8;
_38.0.0 = _10.0;
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)) = _4;
match _25.2.1 {
0 => bb9,
1 => bb13,
2 => bb14,
6 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_4 = Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1);
_6 = !_3;
_25.2.1 = 6_usize;
_24 = Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).3 * _11;
SetDiscriminant(_9.0, 1);
_32 = [_22,_22,_22];
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)).0 = _25.2.3 as i32;
match _25.2.1 {
0 => bb3,
6 => bb12,
_ => bb11
}
}
bb15 = {
_2 = !_3;
_9.1 = [12775_u16];
_15 = _3 <= _4.1;
_8 = _3;
_4.3 = _9.2 as f64;
_6 = !_4.1;
_6 = _2 >= _2;
_6 = _1;
_4.3 = -_11;
_6 = !_2;
Goto(bb2)
}
bb16 = {
_38.0.0 = _10.0;
_41 = &_6;
place!(Field::<u128>(Variant(_9.0, 1), 3)) = _9.2 & _21.1;
_7 = (*_41);
_38 = (_10,);
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)) = _4;
place!(Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3)).0 = _30;
_25.0 = Adt81::Variant0 { fld0: _30 };
_22 = _18;
Call(_43 = fn10(Move(_41), Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).1, _4.2, _8, _4.2), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)) = (_4.0, _8, _4.1, _31);
_40 = -Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1).3;
RET = Adt79::Variant1 { fld0: _32,fld1: _17 };
_4.3 = -_40;
_42 = [_18,_18,_22,_22,_22];
_33 = !_25.2.1;
_37.1 = _25.2.1 ^ _25.2.1;
_38.0.0 = [(-102203489071902996206326841792768510197_i128),95888307469199207777823769535698205557_i128,112213717987437341549330392053380304513_i128,163438514346169660027200724981078108171_i128,76288642968955530753246826077041203802_i128];
_37.1 = !_33;
_44 = _43 as f64;
SetDiscriminant(RET, 0);
_42 = [_18,_18,_22,_18,_22];
_4.2 = _2;
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)) = (_9.3, _15, _5, _44);
place!(Field::<u64>(Variant(RET, 0), 0)) = _25.2.3;
_36 = !(-15507_i16);
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).0.1 = [_25.2.2,_25.2.2];
place!(Field::<(u8, i8, bool)>(Variant(RET, 0), 3)).0 = Field::<u8>(Variant(_25.0, 0), 0) << _34;
_16 = core::ptr::addr_of_mut!(place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)));
SetDiscriminant(_25.0, 1);
Goto(bb18)
}
bb18 = {
place!(Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3)) = (_30, _17, _6);
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).0.3 = (_38.0.0,);
place!(Field::<[i32; 2]>(Variant(RET, 0), 1)) = [_9.3,_9.3];
place!(Field::<i32>(Variant(RET, 0), 5)) = _21.0 as i32;
place!(Field::<(i32, bool, bool, f64)>(Variant(_9.0, 1), 1)).2 = !_6;
(*_16).3 = -_18;
place!(Field::<isize>(Variant(_9.0, 1), 2)) = -_18;
place!(Field::<isize>(Variant(_9.0, 1), 2)) = (*_16).3 ^ Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4).3;
_48 = _11;
place!(Field::<(u8, i8, bool)>(Variant(RET, 0), 3)) = Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3);
(*_16).3 = Field::<isize>(Variant(_9.0, 1), 2) + Field::<isize>(Variant(_9.0, 1), 2);
place!(Field::<u64>(Variant(_9.0, 1), 0)) = !Field::<u64>(Variant(RET, 0), 0);
_25.2.0 = _44 - _31;
SetDiscriminant(_9.0, 0);
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).3 = _36 as isize;
_51 = _22 * (*_16).3;
(*_16).0.0 = [(-18903842841175368998272064749938763379_i128),(-165588414354713404345797027937359405185_i128),73893622103619381908609967959791396256_i128];
_40 = _25.2.0 - _25.2.0;
place!(Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3)).1 = Field::<(u8, i8, bool)>(Variant(RET, 0), 3).1;
_34 = 38353_u16;
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).2 = Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3).2 > _6;
(*_16).0.3 = _38.0;
_53 = _43 * _43;
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).0.2 = _36 << _33;
place!(Field::<bool>(Variant(_9.0, 0), 0)) = !_4.2;
Goto(bb19)
}
bb19 = {
place!(Field::<bool>(Variant(_9.0, 0), 0)) = _15;
_48 = _25.2.3 as f64;
place!(Field::<[isize; 5]>(Variant(RET, 0), 2)) = _42;
place!(Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3)).0 = Field::<(u8, i8, bool)>(Variant(RET, 0), 3).0;
_41 = &place!(Field::<(u8, i8, bool)>(Variant(_25.0, 1), 3)).2;
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).2 = _4.1;
_25.2.2 = _19;
_14 = [_19,_19];
(*_16).0.1 = [_19,_25.2.2];
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).1 = [_34];
_40 = _12;
(*_16).0.0 = [143216629439770936122264619988890025579_i128,(-104769142591507399020926927805938276580_i128),948180096911448484104877075013117178_i128];
_55.0.0 = Adt22::Variant1 { fld0: Field::<u64>(Variant(RET, 0), 0),fld1: _4,fld2: (*_16).3,fld3: _9.2 };
_56 = (Field::<u128>(Variant(_55.0.0, 1), 3), Field::<u64>(Variant(_55.0.0, 1), 0), Field::<(u8, i8, bool)>(Variant(RET, 0), 3).0, _4);
(*_16).0.3.0 = _10.0;
(*_16).0.1 = [_25.2.2,_19];
place!(Field::<u128>(Variant(_55.0.0, 1), 3)) = !_56.0;
place!(Field::<(([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize)>(Variant(RET, 0), 4)).0.1 = _14;
(*_16).0.2 = _36 | _36;
Goto(bb20)
}
bb20 = {
Call(_57 = dump_var(8_usize, 18_usize, Move(_18), 51_usize, Move(_51), 3_usize, Move(_3), 36_usize, Move(_36)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_57 = dump_var(8_usize, 34_usize, Move(_34), 22_usize, Move(_22), 10_usize, Move(_10), 15_usize, Move(_15)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_57 = dump_var(8_usize, 1_usize, Move(_1), 38_usize, Move(_38), 6_usize, Move(_6), 58_usize, _58), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: (i32, bool, bool, f64),mut _11: bool,mut _12: bool) -> bool {
mir! {
type RET = bool;
let _13: char;
let _14: (Adt81, Adt41, (f64, usize, char, u64));
let _15: ();
let _16: ();
{
RET = !_7;
_3 = _2;
_8 = !_11;
_4 = _5;
_2 = _12;
_10.3 = 9223372036854775807_isize as f64;
_10.2 = _9;
_12 = _6;
_13 = '\u{acb8e}';
_9 = _3;
_8 = _10.1;
_7 = _3 >= _4;
_8 = !_10.1;
_10.0 = -417685349_i32;
_3 = _9;
_3 = _2;
_13 = '\u{be3}';
_11 = _3 ^ RET;
_12 = !RET;
_11 = !_10.1;
_14.2.1 = !3002887532871784785_usize;
_6 = _7 <= _3;
_6 = _8;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(9_usize, 13_usize, Move(_13), 5_usize, Move(_5), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(9_usize, 9_usize, Move(_9), 8_usize, Move(_8), 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: &'static bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> f32 {
mir! {
type RET = f32;
let _6: (Adt81, Adt41, (f64, usize, char, u64));
let _7: isize;
let _8: (i8, ([i128; 5],));
let _9: i32;
let _10: Adt22;
let _11: Adt35;
let _12: (f64, usize, char, u64);
let _13: &'static i16;
let _14: isize;
let _15: ([i128; 3], [char; 2], i16, ([i128; 5],));
let _16: isize;
let _17: Adt85;
let _18: f64;
let _19: f64;
let _20: isize;
let _21: [i128; 5];
let _22: f32;
let _23: (Adt22, [u16; 1], u128, i32);
let _24: char;
let _25: u8;
let _26: isize;
let _27: u32;
let _28: (([i128; 5],),);
let _29: [i128; 3];
let _30: ((Adt22, [u16; 1], u128, i32), u128, i128, f64);
let _31: [i128; 5];
let _32: usize;
let _33: [i128; 3];
let _34: [char; 2];
let _35: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _36: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _37: &'static i64;
let _38: char;
let _39: &'static bool;
let _40: char;
let _41: [char; 7];
let _42: bool;
let _43: ();
let _44: ();
{
_1 = &_2;
_2 = _4;
_1 = &_5;
_1 = &_2;
RET = 92_u8 as f32;
RET = 4107406454782134862_i64 as f32;
_1 = &(*_1);
_2 = _4;
_1 = &_3;
_1 = &(*_1);
_4 = _5;
_5 = !_4;
RET = (-210711336_i32) as f32;
RET = 114_i8 as f32;
_2 = _3;
_5 = (*_1);
RET = (-4260337043647125047_i64) as f32;
RET = 257859112288061715260892867029069177305_u128 as f32;
_2 = _4;
_2 = !_3;
_7 = (-9223372036854775808_isize) | 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_8.1.0 = [(-59511209704160527699923104922093172215_i128),110520742206863921971209542463284072248_i128,46696427952047191359714993343245400232_i128,(-51628626614805659305705285804128397293_i128),23727233427339249288138793766019361456_i128];
_8.1.0 = [(-3313466702361658259243973346637728958_i128),(-127859283062910259446465142766955405237_i128),26422207270742004241122512712962880779_i128,(-77487998069807636718611653101798464289_i128),39036782051993369009660768219373850272_i128];
_6.2.1 = 3_usize;
_8.0 = 30_i8 << _7;
RET = (-1166285647_i32) as f32;
RET = _6.2.1 as f32;
_9 = 18986_i16 as i32;
match _6.2.1 {
0 => bb2,
3 => bb4,
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
_6.2.2 = '\u{cf93}';
_2 = (*_1) >= _3;
_8.1.0 = [(-55104918723260545196749429648918978268_i128),143283510805390497335976422692796032368_i128,(-63172646170939507621479393522184681231_i128),(-124525364517606368023566256415415941145_i128),(-116428559646668662564974343847860298590_i128)];
_12.0 = 2680068124220399012_i64 as f64;
_6.2 = (_12.0, 3_usize, '\u{f38b3}', 5851664711509091980_u64);
_4 = !(*_1);
_9 = (-541421375_i32);
_6.2.3 = 7448104602404974317_u64 >> _6.2.1;
_6.2.3 = !6599302834760612379_u64;
_12.0 = (-23602_i16) as f64;
match _6.2.1 {
3 => bb5,
_ => bb1
}
}
bb5 = {
_12 = _6.2;
_6.2.3 = _12.3 << _12.1;
_6.2.3 = _12.3 >> _6.2.1;
RET = _8.0 as f32;
_8.0 = 84_i8 | 120_i8;
_15.1 = [_12.2,_12.2];
_1 = &_3;
RET = 102300765832674650967964372681041564742_i128 as f32;
_4 = _3 | _2;
_12 = (_6.2.0, _6.2.1, _6.2.2, _6.2.3);
_14 = -_7;
_12.0 = -_6.2.0;
_2 = _3;
_9 = _12.1 as i32;
_15.3.0 = [(-110022409508859567190965412669371873209_i128),(-4089479129653177362650407245735668725_i128),152979923433198411258380177216028706938_i128,151940255254948106606846075431585881775_i128,(-73287189508042290143311400377108216762_i128)];
Call(_6.2 = fn11(), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6.2 = _12;
_1 = &_5;
RET = _9 as f32;
_6.2.2 = _12.2;
_13 = &_15.2;
_15.2 = (-12537_i16) - (-10904_i16);
_7 = _12.3 as isize;
Goto(bb7)
}
bb7 = {
_6.2.3 = _12.3;
_18 = -_12.0;
_15.1 = [_6.2.2,_12.2];
_12.1 = !_6.2.1;
_15.0 = [111581981923253306235113225843412378282_i128,86178095123984239223653207522059529818_i128,(-41283782023126346869374552298446340494_i128)];
_2 = !(*_1);
_13 = &_15.2;
_16 = _14 - _7;
_12.0 = _6.2.0 * _6.2.0;
_1 = &_3;
_15.0 = [31649888607222607927543533466140503339_i128,(-109948101196884532621992813146725407046_i128),(-145911135856760013394173499207136598024_i128)];
_8.1 = _15.3;
_16 = !_7;
Call(_6.2.3 = core::intrinsics::transmute(_12.3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12.2 = _6.2.2;
_12.3 = _6.2.3;
_6.2 = (_12.0, _12.1, _12.2, _12.3);
_16 = -_7;
_12.0 = _18;
_12 = (_18, _6.2.1, _6.2.2, _6.2.3);
_23.2 = !145852986136226100787003666712505257712_u128;
_15.3 = (_8.1.0,);
_18 = _12.0 - _6.2.0;
_4 = !_3;
_8 = (61_i8, _15.3);
_14 = _16;
_18 = _6.2.0;
_1 = &(*_1);
_23.3 = _9;
_4 = !(*_1);
_12.3 = 1259169351_u32 as u64;
_23.3 = -_9;
_11 = Adt35::Variant1 { fld0: _8.0,fld1: _15.0 };
_22 = RET + RET;
_21 = _8.1.0;
_9 = _23.3 >> _6.2.3;
place!(Field::<[i128; 3]>(Variant(_11, 1), 1)) = [(-142351384254072164734115695307655109104_i128),38793536613835618621059599256435404672_i128,138953739875626928620094873790866183467_i128];
_4 = (*_1);
place!(Field::<[i128; 3]>(Variant(_11, 1), 1)) = [(-43356371970415666924203626600840019236_i128),(-156428414948594000246408751391133300163_i128),28646305818478406285579822575542636620_i128];
Goto(bb9)
}
bb9 = {
_6.2.2 = _12.2;
RET = _9 as f32;
_18 = 3314704962_u32 as f64;
_1 = &(*_1);
_19 = RET as f64;
_6.2.3 = _12.3;
_20 = _14;
_21 = [(-42689360047449354258075844768346137796_i128),(-139828360428043145539712537253901925193_i128),(-20546606215021750755466892151392990157_i128),(-61002583930201951031419508950535671309_i128),118779513162271691936610187925231120892_i128];
_13 = &(*_13);
_13 = &(*_13);
_23.1 = [59504_u16];
Goto(bb10)
}
bb10 = {
_15.3 = (_21,);
_8 = (Field::<i8>(Variant(_11, 1), 0), _15.3);
_16 = _7;
_12.2 = _6.2.2;
_6.2.0 = _19;
place!(Field::<[i128; 3]>(Variant(_11, 1), 1)) = [4866163360929118308861877197038555765_i128,(-128431809943243368520318987738285162270_i128),126854295632531705460322982220549295039_i128];
_13 = &_15.2;
RET = -_22;
match _8.0 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb6,
61 => bb11,
_ => bb7
}
}
bb11 = {
SetDiscriminant(_11, 2);
_5 = (*_1) & (*_1);
place!(Field::<usize>(Variant(_11, 2), 2)) = !_6.2.1;
_1 = &(*_1);
place!(Field::<usize>(Variant(_11, 2), 2)) = _6.2.1;
_6.2 = _12;
_14 = !_20;
place!(Field::<usize>(Variant(_11, 2), 2)) = _6.2.1;
_22 = -RET;
_14 = _7 - _7;
_6.2.1 = !Field::<usize>(Variant(_11, 2), 2);
_15.2 = -4889_i16;
_4 = _2;
_13 = &_15.2;
_22 = RET;
_30.1 = _23.2 - _23.2;
_30.2 = (-409268447310018539_i64) as i128;
_28.0 = (_15.3.0,);
_8.1 = (_28.0.0,);
_30.0.3 = !_23.3;
_5 = (*_1) != _3;
_30.0.2 = _30.1;
_6.2.3 = !_12.3;
Call(place!(Field::<i32>(Variant(_11, 2), 5)) = core::intrinsics::transmute(_9), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_11 = Adt35::Variant1 { fld0: _8.0,fld1: _15.0 };
_14 = _7 - _16;
_33 = _15.0;
_30.0.1 = [6975_u16];
_28.0.0 = [_30.2,_30.2,_30.2,_30.2,_30.2];
_30.0.1 = _23.1;
_24 = _6.2.2;
SetDiscriminant(_11, 2);
_1 = &_35.2;
RET = -_22;
_28 = (_8.1,);
_35.0.0 = [_30.2,_30.2,_30.2];
_6.2.3 = 55_u8 as u64;
_35.0.2 = 143_u8 as i16;
_18 = -_19;
place!(Field::<*const *mut f32>(Variant(_11, 2), 6)) = core::ptr::addr_of!(place!(Field::<*mut f32>(Variant(_11, 2), 7)));
_30.3 = _12.1 as f64;
_35.3 = _2 as isize;
_36.0.3 = (_8.1.0,);
_35.0.1 = _15.1;
place!(Field::<bool>(Variant(_11, 2), 0)) = !_5;
_23.2 = _30.1 + _30.1;
_35.2 = _3;
_27 = 3225708794_u32 & 1632623081_u32;
_9 = -_30.0.3;
_27 = !3079067754_u32;
_36.0.0 = [_30.2,_30.2,_30.2];
_31 = [_30.2,_30.2,_30.2,_30.2,_30.2];
_35 = (_15, _23.1, _3, _14);
_35.0 = (_33, _15.1, (*_13), _15.3);
match _8.0 {
0 => bb8,
1 => bb2,
61 => bb13,
_ => bb4
}
}
bb13 = {
_35.0 = _15;
_6.2.1 = 2330349166811831054_i64 as usize;
place!(Field::<usize>(Variant(_11, 2), 2)) = !_12.1;
_11 = Adt35::Variant1 { fld0: _8.0,fld1: _15.0 };
_19 = _18;
match _8.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
61 => bb21,
_ => bb20
}
}
bb14 = {
Return()
}
bb15 = {
SetDiscriminant(_11, 2);
_5 = (*_1) & (*_1);
place!(Field::<usize>(Variant(_11, 2), 2)) = !_6.2.1;
_1 = &(*_1);
place!(Field::<usize>(Variant(_11, 2), 2)) = _6.2.1;
_6.2 = _12;
_14 = !_20;
place!(Field::<usize>(Variant(_11, 2), 2)) = _6.2.1;
_22 = -RET;
_14 = _7 - _7;
_6.2.1 = !Field::<usize>(Variant(_11, 2), 2);
_15.2 = -4889_i16;
_4 = _2;
_13 = &_15.2;
_22 = RET;
_30.1 = _23.2 - _23.2;
_30.2 = (-409268447310018539_i64) as i128;
_28.0 = (_15.3.0,);
_8.1 = (_28.0.0,);
_30.0.3 = !_23.3;
_5 = (*_1) != _3;
_30.0.2 = _30.1;
_6.2.3 = !_12.3;
Call(place!(Field::<i32>(Variant(_11, 2), 5)) = core::intrinsics::transmute(_9), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
_15.3 = (_21,);
_8 = (Field::<i8>(Variant(_11, 1), 0), _15.3);
_16 = _7;
_12.2 = _6.2.2;
_6.2.0 = _19;
place!(Field::<[i128; 3]>(Variant(_11, 1), 1)) = [4866163360929118308861877197038555765_i128,(-128431809943243368520318987738285162270_i128),126854295632531705460322982220549295039_i128];
_13 = &_15.2;
RET = -_22;
match _8.0 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb6,
61 => bb11,
_ => bb7
}
}
bb17 = {
_6.2.2 = _12.2;
RET = _9 as f32;
_18 = 3314704962_u32 as f64;
_1 = &(*_1);
_19 = RET as f64;
_6.2.3 = _12.3;
_20 = _14;
_21 = [(-42689360047449354258075844768346137796_i128),(-139828360428043145539712537253901925193_i128),(-20546606215021750755466892151392990157_i128),(-61002583930201951031419508950535671309_i128),118779513162271691936610187925231120892_i128];
_13 = &(*_13);
_13 = &(*_13);
_23.1 = [59504_u16];
Goto(bb10)
}
bb18 = {
_12.2 = _6.2.2;
_12.3 = _6.2.3;
_6.2 = (_12.0, _12.1, _12.2, _12.3);
_16 = -_7;
_12.0 = _18;
_12 = (_18, _6.2.1, _6.2.2, _6.2.3);
_23.2 = !145852986136226100787003666712505257712_u128;
_15.3 = (_8.1.0,);
_18 = _12.0 - _6.2.0;
_4 = !_3;
_8 = (61_i8, _15.3);
_14 = _16;
_18 = _6.2.0;
_1 = &(*_1);
_23.3 = _9;
_4 = !(*_1);
_12.3 = 1259169351_u32 as u64;
_23.3 = -_9;
_11 = Adt35::Variant1 { fld0: _8.0,fld1: _15.0 };
_22 = RET + RET;
_21 = _8.1.0;
_9 = _23.3 >> _6.2.3;
place!(Field::<[i128; 3]>(Variant(_11, 1), 1)) = [(-142351384254072164734115695307655109104_i128),38793536613835618621059599256435404672_i128,138953739875626928620094873790866183467_i128];
_4 = (*_1);
place!(Field::<[i128; 3]>(Variant(_11, 1), 1)) = [(-43356371970415666924203626600840019236_i128),(-156428414948594000246408751391133300163_i128),28646305818478406285579822575542636620_i128];
Goto(bb9)
}
bb19 = {
_6.2.2 = '\u{cf93}';
_2 = (*_1) >= _3;
_8.1.0 = [(-55104918723260545196749429648918978268_i128),143283510805390497335976422692796032368_i128,(-63172646170939507621479393522184681231_i128),(-124525364517606368023566256415415941145_i128),(-116428559646668662564974343847860298590_i128)];
_12.0 = 2680068124220399012_i64 as f64;
_6.2 = (_12.0, 3_usize, '\u{f38b3}', 5851664711509091980_u64);
_4 = !(*_1);
_9 = (-541421375_i32);
_6.2.3 = 7448104602404974317_u64 >> _6.2.1;
_6.2.3 = !6599302834760612379_u64;
_12.0 = (-23602_i16) as f64;
match _6.2.1 {
3 => bb5,
_ => bb1
}
}
bb20 = {
_6.2 = _12;
_1 = &_5;
RET = _9 as f32;
_6.2.2 = _12.2;
_13 = &_15.2;
_15.2 = (-12537_i16) - (-10904_i16);
_7 = _12.3 as isize;
Goto(bb7)
}
bb21 = {
_22 = _6.2.3 as f32;
_36.0.2 = _12.1 as i16;
_2 = _4;
_8.0 = Field::<i8>(Variant(_11, 1), 0) * Field::<i8>(Variant(_11, 1), 0);
_28 = (_8.1,);
_27 = (-3617657022478222409_i64) as u32;
_8.1.0 = _36.0.3.0;
_13 = &_35.0.2;
_35.3 = _14;
_36.1 = _30.0.1;
SetDiscriminant(_11, 0);
_12.0 = _19 + _19;
_38 = _24;
_8.1.0 = [_30.2,_30.2,_30.2,_30.2,_30.2];
_30.0.3 = _8.0 as i32;
_6.2.0 = _18 * _18;
_1 = &_5;
_23.2 = _30.1;
_13 = &_15.2;
_8 = ((-39_i8), _36.0.3);
_35.0.3 = (_8.1.0,);
_42 = !_5;
_34 = [_6.2.2,_38];
_27 = 100640476_u32 & 639124791_u32;
_14 = -_7;
_28.0 = _15.3;
Goto(bb22)
}
bb22 = {
Call(_43 = dump_var(10_usize, 14_usize, Move(_14), 4_usize, Move(_4), 42_usize, Move(_42), 20_usize, Move(_20)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_43 = dump_var(10_usize, 27_usize, Move(_27), 16_usize, Move(_16), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_43 = dump_var(10_usize, 33_usize, Move(_33), 2_usize, Move(_2), 44_usize, _44, 44_usize, _44), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> (f64, usize, char, u64) {
mir! {
type RET = (f64, usize, char, u64);
let _1: [i8; 8];
let _2: *const Adt35;
let _3: i64;
let _4: char;
let _5: *const i128;
let _6: u32;
let _7: [i16; 7];
let _8: &'static [bool; 2];
let _9: *mut *const *mut f32;
let _10: char;
let _11: bool;
let _12: i32;
let _13: &'static u16;
let _14: i128;
let _15: i64;
let _16: &'static *const i64;
let _17: [bool; 2];
let _18: (([i128; 5],),);
let _19: i8;
let _20: f64;
let _21: isize;
let _22: i32;
let _23: Adt35;
let _24: [char; 7];
let _25: ();
let _26: ();
{
RET.0 = 44_i8 as f64;
RET.2 = '\u{fb291}';
RET.3 = !6579073377029622790_u64;
RET.0 = (-20977_i16) as f64;
RET.3 = 16977561897435897467_u64 >> 1_usize;
_1 = [(-31_i8),(-39_i8),(-94_i8),69_i8,(-52_i8),53_i8,(-115_i8),(-14_i8)];
RET.2 = '\u{8075b}';
RET.3 = !12467355239592120662_u64;
RET.1 = 8437216279615255936_usize - 13756516990167867516_usize;
RET.0 = 56_u8 as f64;
RET.0 = 19330_i16 as f64;
RET.1 = 52114_u16 as usize;
RET.0 = 9223372036854775807_isize as f64;
Goto(bb1)
}
bb1 = {
RET.1 = 1390546458506130802_usize + 3_usize;
RET.1 = 3_usize * 3384241472774989855_usize;
RET.0 = 20_i8 as f64;
RET.3 = RET.0 as u64;
RET.0 = 16977930473800047767266791082598039976_i128 as f64;
RET.2 = '\u{c7a4a}';
RET.2 = '\u{ae1b1}';
RET.0 = RET.3 as f64;
RET.3 = (-12697_i16) as u64;
RET.0 = (-4241135776088024342_i64) as f64;
RET.1 = false as usize;
RET.2 = '\u{f621e}';
RET.2 = '\u{36006}';
RET.0 = 200_u8 as f64;
RET.1 = 4_usize;
RET.0 = (-169870625474871374788113010086243223190_i128) as f64;
RET.1 = !2_usize;
_1 = [57_i8,73_i8,5_i8,12_i8,24_i8,44_i8,80_i8,(-119_i8)];
RET.0 = 9009111934105723974_i64 as f64;
_1 = [43_i8,37_i8,106_i8,(-43_i8),117_i8,(-52_i8),(-9_i8),43_i8];
RET.2 = '\u{9aa9c}';
RET.2 = '\u{92e33}';
RET.2 = '\u{92573}';
RET.1 = 647764397372950945_usize;
match RET.1 {
0 => bb2,
1 => bb3,
2 => bb4,
647764397372950945 => bb6,
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
RET.2 = '\u{a8f54}';
_4 = RET.2;
RET.2 = _4;
Goto(bb7)
}
bb7 = {
RET.1 = 5_usize + 3_usize;
RET.1 = 7_usize;
RET.3 = 2185_i16 as u64;
Goto(bb8)
}
bb8 = {
_6 = 1195162529_u32;
_1 = [124_i8,(-83_i8),(-31_i8),(-39_i8),62_i8,(-60_i8),59_i8,71_i8];
RET.2 = _4;
RET.2 = _4;
_3 = (-8847799324415122438_i64) & (-1425154037019190912_i64);
_1 = [65_i8,56_i8,96_i8,(-82_i8),67_i8,(-38_i8),(-107_i8),34_i8];
RET.0 = RET.1 as f64;
RET.2 = _4;
RET.1 = 9744887274178223028_usize;
_3 = RET.1 as i64;
RET.1 = !2748909727746999143_usize;
_3 = 4154102501982281332_i64 << RET.3;
_3 = _6 as i64;
RET.2 = _4;
RET.2 = _4;
_1 = [53_i8,(-71_i8),(-64_i8),(-2_i8),120_i8,107_i8,31_i8,17_i8];
RET.3 = !7395101783097030790_u64;
_7 = [(-28330_i16),(-15966_i16),1854_i16,(-11749_i16),(-20997_i16),(-10876_i16),13417_i16];
RET.3 = 11149236458152360656_u64;
_1 = [(-53_i8),(-121_i8),61_i8,34_i8,4_i8,31_i8,116_i8,83_i8];
RET.3 = !18052164176667508353_u64;
Call(_6 = core::intrinsics::transmute(_4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET.2 = _4;
RET.2 = _4;
RET.1 = 7_usize;
RET.1 = 8942954597491817431_usize;
RET.0 = (-32_i8) as f64;
RET.3 = (-1441006615_i32) as u64;
RET.1 = !2_usize;
RET.3 = !1558646097861065485_u64;
RET.3 = _4 as u64;
RET.0 = (-1578654343_i32) as f64;
RET.3 = 13376979391123164137_u64 + 6963606685836210118_u64;
_7 = [(-23631_i16),21637_i16,(-32180_i16),3731_i16,29647_i16,(-28216_i16),10958_i16];
_1 = [37_i8,(-24_i8),(-56_i8),44_i8,(-56_i8),79_i8,127_i8,(-14_i8)];
RET.1 = 29955_u16 as usize;
RET.0 = 5229_i16 as f64;
RET.2 = _4;
RET.3 = 13660413124455718514_u64 & 16142005723872982928_u64;
RET.0 = (-3346_i16) as f64;
RET.0 = RET.3 as f64;
_6 = 2285928054_u32;
Goto(bb10)
}
bb10 = {
RET.1 = !3_usize;
RET.3 = true as u64;
_7 = [29216_i16,(-27118_i16),11476_i16,(-5368_i16),(-8005_i16),(-15806_i16),(-7805_i16)];
RET.2 = _4;
Goto(bb11)
}
bb11 = {
RET.2 = _4;
_3 = 3862684493195969040_i64 & 8015926108124429497_i64;
_1 = [(-35_i8),21_i8,(-56_i8),(-54_i8),(-105_i8),(-6_i8),(-69_i8),(-10_i8)];
Goto(bb12)
}
bb12 = {
_4 = RET.2;
RET.2 = _4;
RET.0 = (-788110505_i32) as f64;
RET.0 = (-9223372036854775808_isize) as f64;
_10 = _4;
RET.0 = (-33205627214656019914135170532975495162_i128) as f64;
_10 = _4;
RET.2 = _10;
RET.3 = 14561888853508172578_u64;
RET.0 = 65_i8 as f64;
_11 = !true;
_7 = [30069_i16,11822_i16,26994_i16,(-19630_i16),2637_i16,22208_i16,(-25904_i16)];
RET.2 = _10;
RET.1 = 10590076273753018240_usize;
RET.3 = 1371210147334881242_u64;
_10 = RET.2;
Call(_4 = fn12(_1, _7, RET.3, _10, _7, _6, _10, RET.1, RET.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_12 = (-1864388938_i32) << _3;
_4 = _10;
_11 = !true;
RET.3 = 9223372036854775807_isize as u64;
RET.3 = 17441181505775334868_u64;
RET.1 = 5_usize >> _12;
RET.0 = (-5_isize) as f64;
_15 = -_3;
RET.0 = RET.1 as f64;
_5 = core::ptr::addr_of!(_14);
(*_5) = 86476399978244798624146152538378773223_i128;
RET.1 = 14377480517470010259_usize << _12;
_12 = -1482745993_i32;
RET.2 = _10;
RET.1 = !521187101747599163_usize;
_1 = [(-85_i8),103_i8,124_i8,89_i8,(-81_i8),(-34_i8),108_i8,17_i8];
_15 = -_3;
_17 = [_11,_11];
_18.0.0 = [(*_5),_14,(*_5),(*_5),(*_5)];
RET.3 = 9068545394373257589_u64 << (*_5);
_7 = [(-12576_i16),(-16822_i16),12398_i16,(-15347_i16),(-28164_i16),2015_i16,(-21719_i16)];
_3 = 83_i8 as i64;
Goto(bb14)
}
bb14 = {
_8 = &_17;
RET.1 = 5_usize | 7_usize;
(*_5) = (-64033507383061529894712014978236451707_i128);
_22 = _12 << _3;
_15 = _3 ^ _3;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(11_usize, 4_usize, Move(_4), 1_usize, Move(_1), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(11_usize, 12_usize, Move(_12), 14_usize, Move(_14), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [i8; 8],mut _2: [i16; 7],mut _3: u64,mut _4: char,mut _5: [i16; 7],mut _6: u32,mut _7: char,mut _8: usize,mut _9: f64) -> char {
mir! {
type RET = char;
let _10: *mut *const *mut f32;
let _11: bool;
let _12: char;
let _13: ([i128; 5],);
let _14: char;
let _15: *mut f32;
let _16: i8;
let _17: [char; 7];
let _18: *const &'static (u128, u64, u8, (i32, bool, bool, f64));
let _19: *const i32;
let _20: i32;
let _21: i64;
let _22: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _23: (i8, ([i128; 5],));
let _24: ();
let _25: ();
{
_7 = _4;
RET = _7;
_3 = _4 as u64;
_2 = [(-1850_i16),30252_i16,(-3837_i16),30565_i16,(-27565_i16),12367_i16,28867_i16];
_9 = 33_u8 as f64;
_1 = [4_i8,(-53_i8),12_i8,(-93_i8),32_i8,54_i8,72_i8,101_i8];
_8 = !12636804015411042543_usize;
_1 = [(-126_i8),51_i8,93_i8,98_i8,108_i8,54_i8,106_i8,(-127_i8)];
_4 = RET;
_12 = RET;
_2 = [(-29850_i16),(-13476_i16),(-19505_i16),(-24305_i16),(-20253_i16),20610_i16,30567_i16];
_8 = 4_usize >> _3;
_9 = 247076943414533866549892861580614711152_u128 as f64;
RET = _12;
_11 = true;
Goto(bb1)
}
bb1 = {
_6 = _8 as u32;
_12 = RET;
_7 = _4;
_3 = 12592794298955895951_u64 << _8;
_5 = _2;
_1 = [110_i8,(-31_i8),74_i8,(-84_i8),(-101_i8),122_i8,(-85_i8),86_i8];
RET = _7;
_9 = _3 as f64;
_5 = _2;
_6 = 3291184931_u32;
_12 = RET;
_14 = RET;
_14 = _7;
_6 = 1187466667_u32 >> _8;
_13.0 = [13018530682810660541899198650738532043_i128,160662174114206391317895364706531185706_i128,124747710167376872976738796603338454206_i128,14870539291076650990647744448784767551_i128,47078887972848271314165155517359064374_i128];
RET = _4;
_11 = !false;
_5 = [3870_i16,168_i16,(-8214_i16),(-30806_i16),28161_i16,(-10864_i16),(-25665_i16)];
_9 = 121_i8 as f64;
_16 = (-47_i8);
_13.0 = [(-55476124912447355699227711818669841663_i128),137480175521354930322992490912395764801_i128,(-79788563283501135105387925058451115046_i128),167929581929208982163262472611689600866_i128,(-69527876423852324953357879439653102157_i128)];
_12 = _4;
_12 = _4;
_5 = [1730_i16,(-11243_i16),(-28019_i16),17312_i16,16123_i16,(-15540_i16),17154_i16];
_14 = _4;
_7 = _12;
_5 = [3577_i16,(-18011_i16),(-22620_i16),(-9763_i16),9340_i16,21465_i16,20235_i16];
_3 = !10392403329209441365_u64;
Goto(bb2)
}
bb2 = {
_14 = _12;
Goto(bb3)
}
bb3 = {
_6 = _3 as u32;
match _16 {
0 => bb1,
1 => bb4,
2 => bb5,
340282366920938463463374607431768211409 => bb7,
_ => bb6
}
}
bb4 = {
_14 = _12;
Goto(bb3)
}
bb5 = {
_6 = _8 as u32;
_12 = RET;
_7 = _4;
_3 = 12592794298955895951_u64 << _8;
_5 = _2;
_1 = [110_i8,(-31_i8),74_i8,(-84_i8),(-101_i8),122_i8,(-85_i8),86_i8];
RET = _7;
_9 = _3 as f64;
_5 = _2;
_6 = 3291184931_u32;
_12 = RET;
_14 = RET;
_14 = _7;
_6 = 1187466667_u32 >> _8;
_13.0 = [13018530682810660541899198650738532043_i128,160662174114206391317895364706531185706_i128,124747710167376872976738796603338454206_i128,14870539291076650990647744448784767551_i128,47078887972848271314165155517359064374_i128];
RET = _4;
_11 = !false;
_5 = [3870_i16,168_i16,(-8214_i16),(-30806_i16),28161_i16,(-10864_i16),(-25665_i16)];
_9 = 121_i8 as f64;
_16 = (-47_i8);
_13.0 = [(-55476124912447355699227711818669841663_i128),137480175521354930322992490912395764801_i128,(-79788563283501135105387925058451115046_i128),167929581929208982163262472611689600866_i128,(-69527876423852324953357879439653102157_i128)];
_12 = _4;
_12 = _4;
_5 = [1730_i16,(-11243_i16),(-28019_i16),17312_i16,16123_i16,(-15540_i16),17154_i16];
_14 = _4;
_7 = _12;
_5 = [3577_i16,(-18011_i16),(-22620_i16),(-9763_i16),9340_i16,21465_i16,20235_i16];
_3 = !10392403329209441365_u64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_5 = _2;
RET = _12;
_7 = _4;
RET = _14;
_5 = [(-17032_i16),(-11034_i16),14434_i16,6409_i16,(-4007_i16),(-22172_i16),(-939_i16)];
_8 = 7_usize;
RET = _12;
_6 = _9 as u32;
_9 = _6 as f64;
RET = _14;
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_12 = _14;
_5 = [(-8912_i16),(-12246_i16),9813_i16,14428_i16,(-26806_i16),(-29823_i16),(-24105_i16)];
_6 = !665734286_u32;
_11 = false ^ true;
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
340282366920938463463374607431768211409 => bb10,
_ => bb9
}
}
bb8 = {
_14 = _12;
Goto(bb3)
}
bb9 = {
_6 = _8 as u32;
_12 = RET;
_7 = _4;
_3 = 12592794298955895951_u64 << _8;
_5 = _2;
_1 = [110_i8,(-31_i8),74_i8,(-84_i8),(-101_i8),122_i8,(-85_i8),86_i8];
RET = _7;
_9 = _3 as f64;
_5 = _2;
_6 = 3291184931_u32;
_12 = RET;
_14 = RET;
_14 = _7;
_6 = 1187466667_u32 >> _8;
_13.0 = [13018530682810660541899198650738532043_i128,160662174114206391317895364706531185706_i128,124747710167376872976738796603338454206_i128,14870539291076650990647744448784767551_i128,47078887972848271314165155517359064374_i128];
RET = _4;
_11 = !false;
_5 = [3870_i16,168_i16,(-8214_i16),(-30806_i16),28161_i16,(-10864_i16),(-25665_i16)];
_9 = 121_i8 as f64;
_16 = (-47_i8);
_13.0 = [(-55476124912447355699227711818669841663_i128),137480175521354930322992490912395764801_i128,(-79788563283501135105387925058451115046_i128),167929581929208982163262472611689600866_i128,(-69527876423852324953357879439653102157_i128)];
_12 = _4;
_12 = _4;
_5 = [1730_i16,(-11243_i16),(-28019_i16),17312_i16,16123_i16,(-15540_i16),17154_i16];
_14 = _4;
_7 = _12;
_5 = [3577_i16,(-18011_i16),(-22620_i16),(-9763_i16),9340_i16,21465_i16,20235_i16];
_3 = !10392403329209441365_u64;
Goto(bb2)
}
bb10 = {
RET = _7;
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = !true;
_14 = _4;
_4 = RET;
_17 = [_14,_7,_12,RET,_14,_12,_4];
_3 = (-104279451228242315518146543205680831487_i128) as u64;
_3 = 9216498106326298987_u64 & 1437334835423906357_u64;
RET = _12;
_6 = 1285899807_u32 - 3898094819_u32;
_11 = true;
_3 = _8 as u64;
_19 = core::ptr::addr_of!(_20);
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb11)
}
bb11 = {
_9 = 15323_u16 as f64;
_1[_8] = -_16;
_7 = _14;
(*_19) = _6 as i32;
_16 = _1[_8];
_5 = [(-15366_i16),139_i16,17551_i16,12600_i16,30894_i16,(-10257_i16),25030_i16];
_21 = _14 as i64;
(*_19) = _3 as i32;
_21 = !5366838835469312034_i64;
Goto(bb12)
}
bb12 = {
_22.3 = (-5251_i16) as isize;
Goto(bb13)
}
bb13 = {
_22.0.3.0 = [48986826802458261705686112657518107218_i128,43776099634159096987324954392960771534_i128,99796147648649797857921290417480825319_i128,86058393522252771245276385221493358420_i128,(-29655175329870793823232745095415283168_i128)];
_22.0.2 = _9 as i16;
_22.0.1 = [_14,_12];
_22.2 = !_11;
_21 = -(-1825855009522939999_i64);
_22.0.0 = [117804006350668591154082685063257703672_i128,128628460109348690852537765123474978467_i128,(-160939550122518599475285756484394739046_i128)];
match _8 {
7 => bb14,
_ => bb8
}
}
bb14 = {
_9 = (-108414173080891844729892198753300865258_i128) as f64;
_23.0 = _1[_8];
_12 = _14;
_4 = _7;
_14 = _7;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(12_usize, 7_usize, Move(_7), 20_usize, Move(_20), 17_usize, Move(_17), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(12_usize, 11_usize, Move(_11), 8_usize, Move(_8), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: f32,mut _2: isize,mut _3: u8,mut _4: u16) -> u16 {
mir! {
type RET = u16;
let _5: ((Adt22, [u16; 1], u128, i32), u128, i128, f64);
let _6: [char; 7];
let _7: (Adt81, Adt41, (f64, usize, char, u64));
let _8: (u32, u128, u64);
let _9: *const Adt35;
let _10: *mut &'static i16;
let _11: isize;
let _12: f64;
let _13: u32;
let _14: *mut *const *mut f32;
let _15: Adt85;
let _16: Adt81;
let _17: i128;
let _18: isize;
let _19: i128;
let _20: u16;
let _21: &'static bool;
let _22: ();
let _23: ();
{
_5.0.3 = 3532718730417039252_i64 as i32;
_5.0.2 = 243975716950871777155150217590823720340_u128 >> _4;
_5.0.1 = [_4];
_3 = 214_u8 + 43_u8;
_5.0.2 = _1 as u128;
_5.1 = _5.0.2;
_5.1 = _5.0.2;
_6 = ['\u{352b5}','\u{f6f01}','\u{e967d}','\u{8be40}','\u{92e9a}','\u{4b830}','\u{43d82}'];
_5.2 = 151898081342429071465428845498571152741_i128;
_5.0.1 = [_4];
_7.2.3 = '\u{86b82}' as u64;
_8 = (2291693296_u32, _5.1, _7.2.3);
_8.0 = 517297786_u32 ^ 1849911477_u32;
_5.3 = _8.0 as f64;
_5.3 = _1 as f64;
RET = _4;
_3 = 107_u8 | 241_u8;
RET = _4 ^ _4;
_7.2 = (_5.3, 4_usize, '\u{44744}', _8.2);
_2 = _1 as isize;
_5.1 = _5.0.2;
_2 = (-94_isize) & (-58_isize);
_8.1 = _5.1;
_5.1 = _5.0.2;
Goto(bb1)
}
bb1 = {
_7.0 = Adt81::Variant0 { fld0: _3 };
_7.2.1 = _2 as usize;
Goto(bb2)
}
bb2 = {
_7.2.2 = '\u{cb38}';
RET = !_4;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _3 + _3;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _8.1 as u8;
_8 = (936690796_u32, _5.0.2, _7.2.3);
_5.1 = !_8.1;
SetDiscriminant(_7.0, 1);
_11 = _2;
_6 = [_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2];
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)) = (_3, 41_i8, false);
_5.0.3 = (-524139378771236986_i64) as i32;
Call(_5.0 = fn14(_7.2.0, RET, _7.2, RET, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _1, _5.2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)) = (_5.0.3, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).2, _7.2.0);
SetDiscriminant(_5.0.0, 1);
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)).0 = _3 + _3;
place!(Field::<u128>(Variant(_5.0.0, 1), 3)) = _8.1;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).0 = _5.0.3 - _5.0.3;
place!(Field::<isize>(Variant(_5.0.0, 1), 2)) = _11 | _2;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).3 = -_7.2.0;
_5.3 = _7.2.1 as f64;
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)) = (_3, 68_i8, true);
place!(Field::<u64>(Variant(_5.0.0, 1), 0)) = !_7.2.3;
place!(Field::<isize>(Variant(_5.0.0, 1), 2)) = -_2;
_5.0.1 = [RET];
match _8.0 {
0 => bb2,
1 => bb4,
2 => bb5,
936690796 => bb7,
_ => bb6
}
}
bb4 = {
_7.2.2 = '\u{cb38}';
RET = !_4;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _3 + _3;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _8.1 as u8;
_8 = (936690796_u32, _5.0.2, _7.2.3);
_5.1 = !_8.1;
SetDiscriminant(_7.0, 1);
_11 = _2;
_6 = [_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2];
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)) = (_3, 41_i8, false);
_5.0.3 = (-524139378771236986_i64) as i32;
Call(_5.0 = fn14(_7.2.0, RET, _7.2, RET, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _1, _5.2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7.0 = Adt81::Variant0 { fld0: _3 };
_7.2.1 = _2 as usize;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_5.0.1 = [RET];
_7.2.3 = Field::<u64>(Variant(_5.0.0, 1), 0);
_7.0 = Adt81::Variant0 { fld0: _3 };
RET = !_4;
_8.2 = _7.2.3;
RET = _4;
_7.2.0 = _5.3;
RET = _4 | _4;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).2 = true;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).3 = _5.0.3 as f64;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).0 = _5.0.3;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)) = (_5.0.3, true, false, _7.2.0);
place!(Field::<u64>(Variant(_5.0.0, 1), 0)) = _8.2 | _8.2;
match _8.0 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
936690796 => bb15,
_ => bb14
}
}
bb8 = {
Return()
}
bb9 = {
_7.0 = Adt81::Variant0 { fld0: _3 };
_7.2.1 = _2 as usize;
Goto(bb2)
}
bb10 = {
_7.2.2 = '\u{cb38}';
RET = !_4;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _3 + _3;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _8.1 as u8;
_8 = (936690796_u32, _5.0.2, _7.2.3);
_5.1 = !_8.1;
SetDiscriminant(_7.0, 1);
_11 = _2;
_6 = [_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2];
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)) = (_3, 41_i8, false);
_5.0.3 = (-524139378771236986_i64) as i32;
Call(_5.0 = fn14(_7.2.0, RET, _7.2, RET, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _1, _5.2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)) = (_5.0.3, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).2, _7.2.0);
SetDiscriminant(_5.0.0, 1);
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)).0 = _3 + _3;
place!(Field::<u128>(Variant(_5.0.0, 1), 3)) = _8.1;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).0 = _5.0.3 - _5.0.3;
place!(Field::<isize>(Variant(_5.0.0, 1), 2)) = _11 | _2;
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).3 = -_7.2.0;
_5.3 = _7.2.1 as f64;
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)) = (_3, 68_i8, true);
place!(Field::<u64>(Variant(_5.0.0, 1), 0)) = !_7.2.3;
place!(Field::<isize>(Variant(_5.0.0, 1), 2)) = -_2;
_5.0.1 = [RET];
match _8.0 {
0 => bb2,
1 => bb4,
2 => bb5,
936690796 => bb7,
_ => bb6
}
}
bb12 = {
_7.2.2 = '\u{cb38}';
RET = !_4;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _3 + _3;
place!(Field::<u8>(Variant(_7.0, 0), 0)) = _8.1 as u8;
_8 = (936690796_u32, _5.0.2, _7.2.3);
_5.1 = !_8.1;
SetDiscriminant(_7.0, 1);
_11 = _2;
_6 = [_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2];
place!(Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3)) = (_3, 41_i8, false);
_5.0.3 = (-524139378771236986_i64) as i32;
Call(_5.0 = fn14(_7.2.0, RET, _7.2, RET, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, _1, _5.2, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1, Field::<(u8, i8, bool)>(Variant(_7.0, 1), 3).1), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_7.0 = Adt81::Variant0 { fld0: _3 };
_7.2.1 = _2 as usize;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
RET = _4 * _4;
_8 = (430279376_u32, _5.0.2, Field::<u64>(Variant(_5.0.0, 1), 0));
_1 = (-2615_i16) as f32;
place!(Field::<isize>(Variant(_5.0.0, 1), 2)) = RET as isize;
RET = Field::<u8>(Variant(_7.0, 0), 0) as u16;
RET = !_4;
_6 = [_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2,_7.2.2];
place!(Field::<isize>(Variant(_5.0.0, 1), 2)) = _8.0 as isize;
_8 = (735050633_u32, Field::<u128>(Variant(_5.0.0, 1), 3), Field::<u64>(Variant(_5.0.0, 1), 0));
_13 = _8.0 ^ _8.0;
_7.2.2 = '\u{8fe03}';
_17 = _5.2;
_8.0 = (-6484012077199280603_i64) as u32;
_7.2.1 = 7_usize << _13;
_8 = (_13, _5.0.2, _7.2.3);
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)).3 = _5.3 * _7.2.0;
_7.2.3 = Field::<u64>(Variant(_5.0.0, 1), 0) ^ Field::<u64>(Variant(_5.0.0, 1), 0);
_7.2.1 = !7_usize;
RET = _4;
_7.0 = Adt81::Variant0 { fld0: _3 };
place!(Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1)) = (_5.0.3, false, true, _5.3);
_7.2.1 = Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1).0 as usize;
_20 = !RET;
_7.2.0 = Field::<(i32, bool, bool, f64)>(Variant(_5.0.0, 1), 1).0 as f64;
RET = _20 * _4;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(13_usize, 11_usize, Move(_11), 17_usize, Move(_17), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: f64,mut _2: u16,mut _3: (f64, usize, char, u64),mut _4: u16,mut _5: i8,mut _6: isize,mut _7: i8,mut _8: f32,mut _9: i128,mut _10: i8,mut _11: i8) -> (Adt22, [u16; 1], u128, i32) {
mir! {
type RET = (Adt22, [u16; 1], u128, i32);
let _12: isize;
let _13: i16;
let _14: *const *mut f32;
let _15: *mut (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _16: (u32, u128, u64);
let _17: bool;
let _18: usize;
let _19: (f64, char);
let _20: bool;
let _21: f64;
let _22: bool;
let _23: [i128; 5];
let _24: &'static i64;
let _25: Adt60;
let _26: f32;
let _27: f32;
let _28: [i128; 5];
let _29: i16;
let _30: u32;
let _31: [i128; 3];
let _32: f64;
let _33: bool;
let _34: u32;
let _35: (&'static *mut f64, [i32; 2], u16);
let _36: char;
let _37: (u128, u64, u8, (i32, bool, bool, f64));
let _38: bool;
let _39: u128;
let _40: &'static i16;
let _41: ([i128; 3], [char; 2], i16, ([i128; 5],));
let _42: isize;
let _43: [usize; 1];
let _44: [i128; 3];
let _45: isize;
let _46: u128;
let _47: *mut f32;
let _48: ();
let _49: ();
{
_3.1 = 95_u8 as usize;
match _11 {
0 => bb1,
1 => bb2,
41 => bb4,
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
RET.2 = !292747937989669279768919301795649972946_u128;
_8 = 3805884717370697202_i64 as f32;
_3.1 = 11914301136212387306_usize << _5;
_3 = (_1, 3241008422169170168_usize, '\u{e69b0}', 761326127366513353_u64);
_2 = !_4;
RET.1 = [_4];
RET.3 = _3.3 as i32;
Goto(bb5)
}
bb5 = {
_3.1 = !8421155554544881413_usize;
_10 = -_5;
_5 = _4 as i8;
RET.1 = [_2];
_1 = -_3.0;
_9 = 126774624238699110967961264758921972181_i128 + 129812359518733598325862122583909617745_i128;
_1 = _3.0;
_3.0 = _1 + _1;
_13 = -10439_i16;
_3 = (_1, 12214911068544156069_usize, '\u{7c54d}', 4217171111122197105_u64);
_16.2 = !_3.3;
RET.3 = 4265797601_u32 as i32;
_8 = _9 as f32;
_7 = _10 | _5;
_18 = _3.1;
_8 = _13 as f32;
Goto(bb6)
}
bb6 = {
RET.2 = 78876632059402527591645378503029596122_u128;
_17 = _16.2 <= _3.3;
_19 = (_1, _3.2);
_19.1 = _3.2;
RET.1 = [_2];
_1 = _3.0;
_3.1 = _18 % _18;
_21 = _8 as f64;
_21 = -_19.0;
_3.0 = _21 * _21;
_3.3 = !_16.2;
_16.2 = !_3.3;
_11 = 1453011130794635479_i64 as i8;
_19.0 = -_3.0;
Goto(bb7)
}
bb7 = {
_22 = !_17;
_20 = RET.2 < RET.2;
Goto(bb8)
}
bb8 = {
RET.3 = (-699600715_i32);
_16 = (2991662693_u32, RET.2, _3.3);
_16 = (148377819_u32, RET.2, _3.3);
_3.0 = _2 as f64;
RET.2 = !_16.1;
RET.2 = _16.1;
_17 = !_20;
Call(RET.1 = core::intrinsics::transmute(_13), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16 = (777208222_u32, RET.2, _3.3);
_2 = _4;
_7 = -_11;
_23 = [_9,_9,_9,_9,_9];
RET.3 = !1714395438_i32;
_5 = _10 & _10;
match _16.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb10,
777208222 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_3.1 = !8421155554544881413_usize;
_10 = -_5;
_5 = _4 as i8;
RET.1 = [_2];
_1 = -_3.0;
_9 = 126774624238699110967961264758921972181_i128 + 129812359518733598325862122583909617745_i128;
_1 = _3.0;
_3.0 = _1 + _1;
_13 = -10439_i16;
_3 = (_1, 12214911068544156069_usize, '\u{7c54d}', 4217171111122197105_u64);
_16.2 = !_3.3;
RET.3 = 4265797601_u32 as i32;
_8 = _9 as f32;
_7 = _10 | _5;
_18 = _3.1;
_8 = _13 as f32;
Goto(bb6)
}
bb12 = {
_16.0 = !2961705449_u32;
Goto(bb13)
}
bb13 = {
_16 = (3322590335_u32, RET.2, _3.3);
_1 = -_3.0;
_12 = _6 + _6;
_10 = _11;
_16 = (1636114669_u32, RET.2, _3.3);
_3.2 = _19.1;
_21 = _19.0 - _1;
_3.0 = _9 as f64;
_16 = (622270635_u32, RET.2, _3.3);
_3.0 = _21;
_16.0 = _3.2 as u32;
_3.1 = _18;
_23 = [_9,_9,_9,_9,_9];
_19 = (_3.0, _3.2);
_7 = -_5;
_12 = _6;
_19.1 = _3.2;
Call(_19.0 = core::intrinsics::transmute(_16.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_13 = (-22773_i16) + (-4507_i16);
_6 = _21 as isize;
_19 = (_3.0, _3.2);
match _3.1 {
0 => bb12,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb6,
12214911068544156069 => bb15,
_ => bb8
}
}
bb15 = {
RET.3 = -324704563_i32;
RET.1 = [_4];
RET.1 = [_4];
RET.3 = (-2122587236_i32) & 2105122167_i32;
_19 = (_3.0, _3.2);
_16.0 = !1707641296_u32;
_26 = 182_u8 as f32;
_4 = 4517584556005951707_i64 as u16;
_17 = _22 | _22;
_28 = [_9,_9,_9,_9,_9];
_16.2 = _3.3 << _18;
_1 = -_21;
_22 = !_17;
_11 = _7;
RET.2 = !_16.1;
_7 = _11;
_3.0 = _26 as f64;
RET.2 = _16.1;
_7 = _11;
Goto(bb16)
}
bb16 = {
_22 = !_17;
_4 = (-9213016161235756197_i64) as u16;
RET.2 = _16.0 as u128;
_12 = _6;
_3.3 = _16.2 << _7;
RET.1 = [_2];
_5 = _11;
_23 = _28;
_9 = (-103883441400553939789775408569903931563_i128) * (-58044635612758870139733084278775076494_i128);
RET.3 = (-1133444026_i32);
_3.3 = !_16.2;
_3 = (_19.0, _18, _19.1, _16.2);
_19.0 = _1;
_31 = [_9,_9,_9];
_21 = -_19.0;
_3 = (_19.0, _18, _19.1, _16.2);
_29 = _13 * _13;
_18 = _3.1 % _3.1;
_16.1 = RET.2;
_33 = _19.0 == _3.0;
_32 = -_3.0;
_34 = _16.0 & _16.0;
_29 = _13 - _13;
_10 = _4 as i8;
Call(_13 = fn15(), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_27 = _26 + _8;
_20 = !_22;
_18 = _6 as usize;
_19.1 = _3.2;
_2 = _4;
_33 = !_17;
_38 = _20;
_28 = [_9,_9,_9,_9,_9];
_37.1 = !_3.3;
_32 = _21;
_3 = (_32, _18, _19.1, _37.1);
_7 = _4 as i8;
_37.0 = RET.2 - RET.2;
_6 = _18 as isize;
match RET.3 {
0 => bb15,
1 => bb2,
2 => bb12,
3 => bb16,
4 => bb5,
5 => bb6,
340282366920938463463374607430634767430 => bb19,
_ => bb18
}
}
bb18 = {
RET.2 = 78876632059402527591645378503029596122_u128;
_17 = _16.2 <= _3.3;
_19 = (_1, _3.2);
_19.1 = _3.2;
RET.1 = [_2];
_1 = _3.0;
_3.1 = _18 % _18;
_21 = _8 as f64;
_21 = -_19.0;
_3.0 = _21 * _21;
_3.3 = !_16.2;
_16.2 = !_3.3;
_11 = 1453011130794635479_i64 as i8;
_19.0 = -_3.0;
Goto(bb7)
}
bb19 = {
_41.0 = [_9,_9,_9];
_26 = _27;
_37.3 = (RET.3, _17, _20, _3.0);
_2 = !_4;
_16.0 = _34 << _3.1;
_5 = _11;
_31 = [_9,_9,_9];
_29 = RET.3 as i16;
_41.3 = (_23,);
_19.0 = -_1;
_3 = (_32, _18, _19.1, _16.2);
_39 = RET.2 << _3.1;
_17 = !_37.3.2;
_35.2 = _12 as u16;
_27 = _26 + _26;
_31 = _41.0;
match _37.3.0 {
340282366920938463463374607430634767430 => bb21,
_ => bb20
}
}
bb20 = {
_16.0 = !2961705449_u32;
Goto(bb13)
}
bb21 = {
_19.1 = _3.2;
_19 = (_3.0, _3.2);
_37.0 = !_39;
_16 = (_34, _37.0, _37.1);
_19.0 = -_3.0;
Goto(bb22)
}
bb22 = {
_39 = _37.0 & _37.0;
_30 = _34;
_45 = _6 * _6;
_42 = !_45;
_41.1 = [_3.2,_3.2];
_16 = (_30, _37.0, _37.1);
_35.2 = !_4;
_26 = _13 as f32;
_37.0 = (-579206087500491249_i64) as u128;
match _37.3.0 {
0 => bb10,
1 => bb11,
340282366920938463463374607430634767430 => bb23,
_ => bb19
}
}
bb23 = {
RET.1 = [_2];
_16.0 = _16.1 as u32;
_31 = _41.0;
_45 = _42;
_47 = core::ptr::addr_of_mut!(_27);
RET.0 = Adt22::Variant1 { fld0: _3.3,fld1: _37.3,fld2: _45,fld3: _37.0 };
Goto(bb24)
}
bb24 = {
Call(_48 = dump_var(14_usize, 6_usize, Move(_6), 42_usize, Move(_42), 20_usize, Move(_20), 38_usize, Move(_38)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_48 = dump_var(14_usize, 18_usize, Move(_18), 22_usize, Move(_22), 31_usize, Move(_31), 45_usize, Move(_45)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_48 = dump_var(14_usize, 23_usize, Move(_23), 17_usize, Move(_17), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_48 = dump_var(14_usize, 10_usize, Move(_10), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15() -> i16 {
mir! {
type RET = i16;
let _1: Adt79;
let _2: i8;
let _3: isize;
let _4: isize;
let _5: [bool; 2];
let _6: u32;
let _7: *const Adt35;
let _8: char;
let _9: Adt41;
let _10: (&'static i64, &'static bool);
let _11: Adt81;
let _12: &'static i64;
let _13: f32;
let _14: u16;
let _15: bool;
let _16: char;
let _17: [usize; 1];
let _18: *const i64;
let _19: Adt22;
let _20: *mut (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize);
let _21: (u32, u128, u64);
let _22: *const &'static (u128, u64, u8, (i32, bool, bool, f64));
let _23: f64;
let _24: i64;
let _25: (i8, ([i128; 5],));
let _26: [char; 2];
let _27: bool;
let _28: ();
let _29: ();
{
RET = (-9223372036854775808_isize) as i16;
Goto(bb1)
}
bb1 = {
RET = -31169_i16;
RET = -(-3053_i16);
RET = false as i16;
RET = (-30249_i16) * 9942_i16;
RET = !(-7725_i16);
RET = (-359_i16);
RET = 12936243976234955682_u64 as i16;
RET = !17981_i16;
RET = !(-5513_i16);
RET = 88_i8 as i16;
RET = (-11229_i16) + 16152_i16;
RET = !14634_i16;
RET = 16905_i16;
RET = (-115_i8) as i16;
RET = (-20124_i16);
_2 = (-90_i8);
RET = _2 as i16;
_2 = (-11_i8);
RET = !4759_i16;
_3 = 164_u8 as isize;
_3 = RET as isize;
_4 = _3 - _3;
RET = !(-18278_i16);
RET = 10588_i16;
_2 = 82683570411133784512244431554762422099_u128 as i8;
RET = !25481_i16;
_2 = (-103_i8) + 1_i8;
_4 = 6483043085449939902_u64 as isize;
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = !23_i8;
RET = (-1627_i16) ^ (-1766_i16);
_2 = 34_i8 & 52_i8;
_6 = !1682868388_u32;
_4 = !_3;
_5 = [false,false];
_3 = !_4;
_4 = -_3;
RET = _2 as i16;
_6 = 2717045146_u32;
_3 = _4;
RET = -19574_i16;
match _6 {
0 => bb3,
1 => bb4,
2 => bb5,
2717045146 => bb7,
_ => bb6
}
}
bb3 = {
RET = -31169_i16;
RET = -(-3053_i16);
RET = false as i16;
RET = (-30249_i16) * 9942_i16;
RET = !(-7725_i16);
RET = (-359_i16);
RET = 12936243976234955682_u64 as i16;
RET = !17981_i16;
RET = !(-5513_i16);
RET = 88_i8 as i16;
RET = (-11229_i16) + 16152_i16;
RET = !14634_i16;
RET = 16905_i16;
RET = (-115_i8) as i16;
RET = (-20124_i16);
_2 = (-90_i8);
RET = _2 as i16;
_2 = (-11_i8);
RET = !4759_i16;
_3 = 164_u8 as isize;
_3 = RET as isize;
_4 = _3 - _3;
RET = !(-18278_i16);
RET = 10588_i16;
_2 = 82683570411133784512244431554762422099_u128 as i8;
RET = !25481_i16;
_2 = (-103_i8) + 1_i8;
_4 = 6483043085449939902_u64 as isize;
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
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
_6 = 983090941_u32 & 1794415853_u32;
RET = (-16319_i16);
RET = !19241_i16;
_2 = (-45_i8) * (-39_i8);
_3 = !_4;
_8 = '\u{4b1c2}';
_2 = (-32_i8) - 67_i8;
_8 = '\u{5b62c}';
_2 = !115_i8;
_8 = '\u{def90}';
_6 = 3889526474_u32 * 758062417_u32;
RET = _8 as i16;
_8 = '\u{42eae}';
_5 = [true,true];
_3 = _4 - _4;
_3 = _4 ^ _4;
_3 = !_4;
_3 = !_4;
_6 = 528120551_u32 + 934145680_u32;
Goto(bb8)
}
bb8 = {
RET = 41185907899676165885774959388073220859_i128 as i16;
_2 = (-79_i8) - (-49_i8);
_6 = 2421099075_u32 & 1524574963_u32;
Goto(bb9)
}
bb9 = {
_11 = Adt81::Variant0 { fld0: 251_u8 };
_2 = 27_i8 << _3;
_6 = 81367077_u32 ^ 4138188768_u32;
RET = (-17206_i16) * 19016_i16;
place!(Field::<u8>(Variant(_11, 0), 0)) = !212_u8;
_4 = !_3;
RET = 27544_u16 as i16;
_5 = [true,true];
RET = !32634_i16;
SetDiscriminant(_11, 0);
_5 = [true,true];
RET = -(-3522_i16);
RET = 14580_i16 << _3;
_4 = 18099570004875633215_u64 as isize;
place!(Field::<u8>(Variant(_11, 0), 0)) = !112_u8;
RET = _2 as i16;
_8 = '\u{3a8bf}';
_3 = _4 * _4;
Goto(bb10)
}
bb10 = {
place!(Field::<u8>(Variant(_11, 0), 0)) = 44_u8 * 243_u8;
_13 = 234898658316972250804745182004307193981_u128 as f32;
place!(Field::<u8>(Variant(_11, 0), 0)) = 35_u8 + 186_u8;
_13 = 43187_u16 as f32;
_13 = 3_usize as f32;
RET = false as i16;
_6 = (-136781991677943782201410568520478430812_i128) as u32;
_8 = '\u{d5268}';
_5 = [false,false];
_3 = RET as isize;
_5 = [false,true];
_5 = [false,true];
_8 = '\u{7ad16}';
_16 = _8;
_6 = !2677470693_u32;
Goto(bb11)
}
bb11 = {
_14 = !55716_u16;
_3 = _4 >> Field::<u8>(Variant(_11, 0), 0);
_10.1 = &_15;
_4 = !_3;
Goto(bb12)
}
bb12 = {
_11 = Adt81::Variant0 { fld0: 108_u8 };
place!(Field::<u8>(Variant(_11, 0), 0)) = 232_u8;
RET = !25311_i16;
_21.1 = 81293568723437525840051482346465546920_u128 << Field::<u8>(Variant(_11, 0), 0);
match Field::<u8>(Variant(_11, 0), 0) {
232 => bb13,
_ => bb5
}
}
bb13 = {
_21.2 = _2 as u64;
_17 = [4_usize];
_21.2 = 2607211909519106273_i64 as u64;
RET = _16 as i16;
_21.0 = _6;
_21.2 = 1925538568472066339_u64 & 12901199540820683724_u64;
_5 = [false,true];
_14 = !12900_u16;
_8 = _16;
place!(Field::<u8>(Variant(_11, 0), 0)) = _2 as u8;
_24 = !4852305960141642642_i64;
_21.2 = 14114899665330053208_u64 * 16553253043033338380_u64;
_10.0 = &_24;
_5 = [false,false];
SetDiscriminant(_11, 2);
RET = false as i16;
_23 = (-1615406247_i32) as f64;
place!(Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7)).3 = _23;
_10.0 = &_24;
_12 = &_24;
_25.1.0 = [(-151596190486803275349637462678792048774_i128),(-91892483951728889900272121941041721704_i128),130599423668737639514460763945258354318_i128,123062637726280553797528193492224405821_i128,(-145299949259058272091228480707419997102_i128)];
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).2 = _21.0 as i128;
_23 = Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7).3;
_13 = _2 as f32;
_2 = 66_i8;
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).0.2 = !_21.1;
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).2 = -(-62821667624872087200164801314917491358_i128);
Goto(bb14)
}
bb14 = {
_10.1 = &_15;
place!(Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7)).2 = false ^ true;
_6 = _21.0;
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).0.1 = [_14];
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).0.3 = (-1064124813_i32) | (-1639059665_i32);
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).3 = RET as f64;
_21.2 = _23 as u64;
_25.1.0 = [Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).2,Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).2,Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).2,Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).2,Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).2];
_21.2 = _2 as u64;
place!(Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7)) = (Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).0.3, true, false, _23);
_12 = &(*_12);
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).1 = Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7).0 as u128;
place!(Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7)).1 = !Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7).2;
_10.1 = &place!(Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7)).1;
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).0.1 = [_14];
_12 = &(*_12);
_2 = !124_i8;
_8 = _16;
place!(Field::<u8>(Variant(_11, 2), 1)) = 93_u8 ^ 251_u8;
_6 = _21.0 ^ _21.0;
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).0.0 = Adt22::Variant1 { fld0: _21.2,fld1: Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7),fld2: _4,fld3: Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).1 };
place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).2 = 129184163010547307376562134396549002800_i128 + 91468852271089623153673863703626290018_i128;
SetDiscriminant(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).0.0, 1);
_26 = [_8,_8];
place!(Field::<(i32, bool, bool, f64)>(Variant(place!(Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4)).0.0, 1), 1)) = Field::<(i32, bool, bool, f64)>(Variant(_11, 2), 7);
place!(Field::<u32>(Variant(_11, 2), 6)) = _13 as u32;
place!(Field::<u32>(Variant(_11, 2), 6)) = Field::<((Adt22, [u16; 1], u128, i32), u128, i128, f64)>(Variant(_11, 2), 4).0.3 as u32;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(15_usize, 17_usize, Move(_17), 24_usize, Move(_24), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(15_usize, 3_usize, Move(_3), 5_usize, Move(_5), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(195585594570389052207647073719991169728_u128), std::hint::black_box('\u{b97}'), std::hint::black_box(63334_u16), std::hint::black_box(217_u8), std::hint::black_box(25174_i16), std::hint::black_box(610669106_i32), std::hint::black_box((-148033332081163934_i64)), std::hint::black_box(77142285952667746155737575737860393778_i128), std::hint::black_box(2430969170_u32));
                
            }
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: bool,
fld1: *mut f32,
fld2: u16,

},
Variant1{
fld0: u64,
fld1: (i32, bool, bool, f64),
fld2: isize,
fld3: u128,

}}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf("Adt35::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: Adt22,

},
Variant1{
fld0: i8,
fld1: [i128; 3],

},
Variant2{
fld0: bool,
fld1: char,
fld2: usize,
fld3: i8,
fld4: *const u16,
fld5: i32,
fld6: *const *mut f32,
fld7: *mut f32,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *mut f64,
fld1: *mut *mut [char; 2],
fld2: i64,
fld3: Adt35,
fld4: [u16; 1],
fld5: (Adt22, [u16; 1], u128, i32),

},
Variant1{
fld0: Adt22,
fld1: char,
fld2: usize,
fld3: ([i128; 5],),

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [u16; 1],

},
Variant1{
fld0: Adt35,
fld1: char,
fld2: isize,
fld3: i8,
fld4: ([i128; 3], [char; 2], i16, ([i128; 5],)),
fld5: ([i128; 5],),
fld6: ([char; 2], usize),
fld7: Adt41,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: *const *mut f32,
fld1: f32,
fld2: u32,
fld3: Adt35,
fld4: i64,
fld5: *mut f32,

},
Variant1{
fld0: [u16; 1],
fld1: Adt22,
fld2: *const *mut f32,

},
Variant2{
fld0: Adt35,
fld1: usize,
fld2: *const u128,

},
Variant3{
fld0: (Adt22, [u16; 1], u128, i32),
fld1: char,
fld2: *const i64,
fld3: u16,
fld4: ([u16; 1], *const u16),
fld5: i32,
fld6: u32,
fld7: i128,

}}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){unsafe{printf("Adt79::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: u64,
fld1: [i32; 2],
fld2: [isize; 5],
fld3: (u8, i8, bool),
fld4: (([i128; 3], [char; 2], i16, ([i128; 5],)), [u16; 1], bool, isize),
fld5: i32,

},
Variant1{
fld0: [isize; 3],
fld1: i8,

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: u8,

},
Variant1{
fld0: i16,
fld1: ([char; 2], usize),
fld2: Adt41,
fld3: (u8, i8, bool),

},
Variant2{
fld0: bool,
fld1: u8,
fld2: [u16; 1],
fld3: *mut *mut [char; 2],
fld4: ((Adt22, [u16; 1], u128, i32), u128, i128, f64),
fld5: *mut (f64, char),
fld6: u32,
fld7: (i32, bool, bool, f64),

}}
impl PrintFDebug for Adt85{
	unsafe fn printf_debug(&self){unsafe{printf("Adt85::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt85 {
Variant0{
fld0: *const Adt35,
fld1: [i16; 7],
fld2: (f64, usize, char, u64),
fld3: u8,
fld4: *const u128,
fld5: Adt79,
fld6: *mut f64,

},
Variant1{
fld0: (u8, i8, bool),

}}

