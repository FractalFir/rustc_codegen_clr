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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: u128,mut _4: u32,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16) -> usize {
mir! {
type RET = usize;
let _12: &'static i16;
let _13: f64;
let _14: u64;
let _15: Adt61;
let _16: [bool; 8];
let _17: u64;
let _18: isize;
let _19: &'static i16;
let _20: [usize; 7];
let _21: (*const u8, [i16; 1]);
let _22: char;
let _23: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _24: ();
let _25: ();
{
_8 = -(-153097372554286721051478385362531245143_i128);
_2 = '\u{a9af3}';
_8 = 135448649805359681225089997624897713646_i128 >> 1252_i16;
_5 = 321094249698701741542725053635550671855_u128 as i16;
_12 = &_5;
_3 = 164362498408515631783995583475577001767_u128 >> _8;
_8 = !(-6190354358111301421433128786588062370_i128);
Call(_10 = fn1(Move(_12), _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = &_5;
_4 = 4292502891_u32 + 1873036197_u32;
_9 = !17505983133631967124_usize;
_13 = 9411016937191359403_u64 as f64;
_10 = 5_u8;
_11 = 52458_u16 - 45138_u16;
_11 = _5 as u16;
RET = _9 >> _5;
_3 = 71591209634443393653389814601300673580_u128;
_11 = 51346_u16 | 36705_u16;
_3 = (-5648315135371479088_i64) as u128;
_1 = !7914779618546633458_u64;
_13 = 58_i8 as f64;
_10 = !123_u8;
_10 = (*_12) as u8;
_10 = (-113_i8) as u8;
RET = !_9;
_10 = 149_u8;
Goto(bb2)
}
bb2 = {
_5 = (-21546_i16) << _1;
match _10 {
0 => bb3,
149 => bb5,
_ => bb4
}
}
bb3 = {
_12 = &_5;
_4 = 4292502891_u32 + 1873036197_u32;
_9 = !17505983133631967124_usize;
_13 = 9411016937191359403_u64 as f64;
_10 = 5_u8;
_11 = 52458_u16 - 45138_u16;
_11 = _5 as u16;
RET = _9 >> _5;
_3 = 71591209634443393653389814601300673580_u128;
_11 = 51346_u16 | 36705_u16;
_3 = (-5648315135371479088_i64) as u128;
_1 = !7914779618546633458_u64;
_13 = 58_i8 as f64;
_10 = !123_u8;
_10 = (*_12) as u8;
_10 = (-113_i8) as u8;
RET = !_9;
_10 = 149_u8;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_10 = _13 as u8;
_13 = (-91_isize) as f64;
_12 = &_5;
_7 = -(-4586632905261613141_i64);
_3 = (*_12) as u128;
_13 = _7 as f64;
_6 = 116_i8 as i32;
_5 = 47_i8 as i16;
_3 = !267799623591071572356232540933762292981_u128;
_4 = 1842896522_u32;
_14 = !_1;
_13 = _5 as f64;
_2 = '\u{38f74}';
_16 = [false,true,false,true,true,false,true,true];
_3 = false as u128;
_8 = 116637845377569768633494662796748608195_i128;
_8 = _5 as i128;
_10 = _11 as u8;
match _4 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
1842896522 => bb14,
_ => bb13
}
}
bb6 = {
Return()
}
bb7 = {
_12 = &_5;
_4 = 4292502891_u32 + 1873036197_u32;
_9 = !17505983133631967124_usize;
_13 = 9411016937191359403_u64 as f64;
_10 = 5_u8;
_11 = 52458_u16 - 45138_u16;
_11 = _5 as u16;
RET = _9 >> _5;
_3 = 71591209634443393653389814601300673580_u128;
_11 = 51346_u16 | 36705_u16;
_3 = (-5648315135371479088_i64) as u128;
_1 = !7914779618546633458_u64;
_13 = 58_i8 as f64;
_10 = !123_u8;
_10 = (*_12) as u8;
_10 = (-113_i8) as u8;
RET = !_9;
_10 = 149_u8;
Goto(bb2)
}
bb8 = {
_5 = (-21546_i16) << _1;
match _10 {
0 => bb3,
149 => bb5,
_ => bb4
}
}
bb9 = {
_12 = &_5;
_4 = 4292502891_u32 + 1873036197_u32;
_9 = !17505983133631967124_usize;
_13 = 9411016937191359403_u64 as f64;
_10 = 5_u8;
_11 = 52458_u16 - 45138_u16;
_11 = _5 as u16;
RET = _9 >> _5;
_3 = 71591209634443393653389814601300673580_u128;
_11 = 51346_u16 | 36705_u16;
_3 = (-5648315135371479088_i64) as u128;
_1 = !7914779618546633458_u64;
_13 = 58_i8 as f64;
_10 = !123_u8;
_10 = (*_12) as u8;
_10 = (-113_i8) as u8;
RET = !_9;
_10 = 149_u8;
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
_17 = (-9223372036854775808_isize) as u64;
_4 = 3168707267_u32 & 3723974238_u32;
_2 = '\u{e15da}';
_12 = &_5;
_9 = !RET;
_12 = &(*_12);
RET = !_9;
_5 = (-8244_i16) << _10;
_20 = [RET,_9,_9,_9,RET,RET,_9];
_14 = _4 as u64;
_1 = _17;
_12 = &_5;
_13 = _10 as f64;
_21.1 = [(*_12)];
_7 = 5358142200150589591_i64 ^ (-6267171475959250919_i64);
_12 = &_5;
_1 = _9 as u64;
_15 = Adt61::Variant0 { fld0: RET };
_21.1 = [_5];
_7 = -(-5138171875979012_i64);
_12 = &(*_12);
SetDiscriminant(_15, 0);
_20 = [_9,_9,_9,RET,RET,_9,RET];
_19 = &(*_12);
place!(Field::<usize>(Variant(_15, 0), 0)) = _2 as usize;
SetDiscriminant(_15, 0);
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(0_usize, 3_usize, Move(_3), 2_usize, Move(_2), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(0_usize, 11_usize, Move(_11), 1_usize, Move(_1), 5_usize, Move(_5), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: &'static i16,mut _2: i16,mut _3: i16) -> u8 {
mir! {
type RET = u8;
let _4: ([bool; 8], Adt33);
let _5: (usize, u64);
let _6: (char, *mut u8);
let _7: f64;
let _8: u32;
let _9: f32;
let _10: char;
let _11: Adt32;
let _12: *mut *mut u8;
let _13: i8;
let _14: isize;
let _15: (u128, char, bool, u8);
let _16: f32;
let _17: &'static *mut u8;
let _18: Adt32;
let _19: [char; 8];
let _20: *mut u32;
let _21: &'static Adt29;
let _22: f64;
let _23: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _24: &'static Adt29;
let _25: *const Adt29;
let _26: *mut isize;
let _27: (f32, u32, f32, [i16; 5]);
let _28: i128;
let _29: u128;
let _30: isize;
let _31: char;
let _32: bool;
let _33: &'static *const (u128, char, bool, u8);
let _34: f64;
let _35: i128;
let _36: (isize,);
let _37: &'static *const (u128, char, bool, u8);
let _38: isize;
let _39: u32;
let _40: ();
let _41: ();
{
RET = 44_u8 - 11_u8;
RET = !209_u8;
_1 = &_3;
RET = 182_u8;
_3 = !_2;
RET = 3018903025_u32 as u8;
_4.1.fld4 = 65371_u16 & 45615_u16;
RET = 71_u8;
_4.1.fld4 = 2822_u16;
_4.1.fld0 = _3 < _3;
_4.1.fld0 = !false;
_1 = &_2;
_4.1.fld2 = 7714895869166251316_i64 as f64;
RET = 32_isize as u8;
_4.1.fld4 = 21733_u16;
_4.1.fld4 = !16033_u16;
RET = 71_u8 ^ 20_u8;
_4.1.fld0 = false;
_5.0 = 13049120108197595578_usize >> _4.1.fld4;
_5.0 = !11792723058467436191_usize;
_5 = (12441204061420439086_usize, 7151905082753620000_u64);
match _5.1 {
0 => bb1,
7151905082753620000 => bb3,
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
_4.1.fld2 = 1334236924_u32 as f64;
_4.1.fld3 = [(-167239686670599597654835048098319132599_i128),3653016098529089097125544133163462903_i128,131034138014297341746256943570877325776_i128,102017364791194178651690756768525739292_i128,(-75745917445725379273133207838323842245_i128),33803686332035746642104032357663678662_i128];
_5 = (13662410071018590461_usize, 10498020383734774716_u64);
_4.1.fld3 = [142421524440194634555971733904581101905_i128,(-49713806847111218278141903140026117677_i128),(-45275264680335596236649813104090631176_i128),51163014677524297428866975291221900646_i128,(-152924272771016765409600130286304913524_i128),72322310720642477972922405009386847565_i128];
_4.1.fld0 = false;
_4.1.fld5 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
Call(_4.1 = fn2(Move(_1), _5, _5.0, _5.1, _5.1, _5, _5.1, _5.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = -_4.1.fld2;
_6.1 = core::ptr::addr_of_mut!(RET);
_4.1.fld4 = 62198_u16 + 56879_u16;
_4.1.fld3 = [146227047953180862940434954707717138153_i128,97825826649661099019689224090043621340_i128,(-23993416307187033857378452621360497216_i128),(-8502646649736688790138949343888356239_i128),(-153413406642413506562883426000594379969_i128),49816580559305358217737695128147301728_i128];
_3 = -_2;
Goto(bb5)
}
bb5 = {
_4.1.fld2 = 98264647058005928213344929770939867384_u128 as f64;
_6.0 = '\u{b392e}';
_4.1.fld5 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
RET = _5.0 as u8;
_8 = !1022016535_u32;
_1 = &_2;
_6.0 = '\u{590f}';
_5.1 = (-33_i8) as u64;
Goto(bb6)
}
bb6 = {
_4.1.fld2 = -_7;
RET = _4.1.fld2 as u8;
_9 = _4.1.fld4 as f32;
_1 = &(*_1);
RET = 98_u8;
_5 = (4_usize, 4997427998412851147_u64);
_4.1.fld5 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
Goto(bb7)
}
bb7 = {
_5.0 = 6_usize;
_2 = _9 as i16;
_4.1.fld0 = _5.0 <= _5.0;
_5.0 = !7_usize;
_4.0 = [_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0];
_4.1.fld2 = _7 + _7;
_6.0 = '\u{5a812}';
_3 = _6.0 as i16;
Goto(bb8)
}
bb8 = {
_5 = (1009646496492206152_usize, 1712093746073504555_u64);
_3 = _2;
Goto(bb9)
}
bb9 = {
_6.0 = '\u{89f24}';
_4.1.fld5 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_5 = (4644813744890746647_usize, 17955187242506259618_u64);
_4.1.fld4 = !13996_u16;
_3 = -_2;
_2 = _3 & _3;
_4.1.fld0 = !false;
_4.1.fld0 = true;
_4.1.fld5 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_5.1 = 11376399104660897746_u64;
_8 = _4.1.fld2 as u32;
_4.1.fld2 = -_7;
_1 = &_2;
_4.1.fld3 = [(-38507214813484004633340144771436841009_i128),13314385592799859492895323811333210757_i128,87559068980839704148980681999744962109_i128,125224272590826019898380628646795930020_i128,(-62875928590955045635687482252693949429_i128),(-94778707701959595803491043275477326663_i128)];
_4.1.fld0 = !true;
_4.1.fld0 = true;
_6.1 = core::ptr::addr_of_mut!(RET);
_5.0 = 10204063971991663608_usize;
_4.1.fld2 = _7 + _7;
_4.1.fld1 = core::ptr::addr_of_mut!(_6.1);
match _5.1 {
0 => bb3,
11376399104660897746 => bb10,
_ => bb2
}
}
bb10 = {
_13 = !(-66_i8);
_13 = (-84_i8);
_4.0 = [_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0];
_3 = _2;
_15.2 = !_4.1.fld0;
RET = _5.1 as u8;
_4.1.fld3 = [(-77760237688620296218462204466576817962_i128),(-43241629173906264785393268157320792774_i128),19997790735206533466720324477205876236_i128,(-147238074249983919580927841057140421716_i128),66790803196703100034107436419361219000_i128,(-78735081147162179745610750155393895031_i128)];
Goto(bb11)
}
bb11 = {
_14 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_16 = _9 * _9;
_4.0 = [_4.1.fld0,_15.2,_15.2,_4.1.fld0,_15.2,_4.1.fld0,_15.2,_4.1.fld0];
_5.1 = _4.1.fld4 as u64;
RET = !6_u8;
_17 = &_6.1;
_5 = (2_usize, 9176501082437697931_u64);
_3 = (*_1);
_4.1.fld5 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_2 = 1080993899_i32 as i16;
_15.1 = _6.0;
_10 = _6.0;
_15.2 = _4.1.fld0 & _4.1.fld0;
_2 = -_3;
_5.1 = !5475658697889637472_u64;
_3 = _2 << _2;
_4.1.fld1 = core::ptr::addr_of_mut!(_6.1);
_5 = (6_usize, 16046515904146033496_u64);
_20 = core::ptr::addr_of_mut!(_8);
_18 = Adt32::Variant2 { fld0: _5.1 };
_16 = _9 - _9;
Goto(bb12)
}
bb12 = {
_16 = _9;
_4.1.fld2 = -_7;
_5.0 = 7_usize;
RET = 183_u8;
_13 = 43_i8 | 12_i8;
RET = 134_u8;
_3 = _5.1 as i16;
_14 = (-8_isize);
_13 = (-108_i8) | 44_i8;
_19 = [_10,_10,_6.0,_6.0,_6.0,_15.1,_6.0,_10];
_4.1.fld3 = [76387790889719515148295238653794714834_i128,84041532568623955022053966273654233464_i128,(-108014503694492578184933043950955667563_i128),(-131919771026732210984282724687625257126_i128),(-34748437150403587850388027194098210105_i128),118973920682290558035569269595596922174_i128];
_11 = Move(_18);
_22 = -_4.1.fld2;
_15.2 = !_4.1.fld0;
(*_20) = !4283797919_u32;
_9 = -_16;
_4.1.fld3 = [(-117079186058483820899541952974165459986_i128),21997148789479490489136969037161106707_i128,132400102278146076067084716502557401385_i128,(-34399884379935508048438399990235666543_i128),23857871730418995237973474098008861424_i128,(-153558247481750515589324319045028758079_i128)];
_1 = &_3;
_13 = 1_i8 - 34_i8;
_4.1.fld2 = -_22;
_1 = &(*_1);
_15.3 = !RET;
_26 = core::ptr::addr_of_mut!(_14);
match (*_26) {
340282366920938463463374607431768211448 => bb13,
_ => bb11
}
}
bb13 = {
_15.2 = !_4.1.fld0;
_18 = Move(_11);
(*_26) = _4.1.fld0 as isize;
(*_26) = !(-115_isize);
_16 = _9 - _9;
_27.2 = _16 * _9;
_20 = core::ptr::addr_of_mut!(_8);
_12 = Move(_4.1.fld1);
_4.1.fld2 = _7 + _7;
_4.1.fld1 = core::ptr::addr_of_mut!((*_17));
_4.0 = [_4.1.fld0,_4.1.fld0,_15.2,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0,_4.1.fld0];
_4.0 = [_4.1.fld0,_15.2,_15.2,_4.1.fld0,_4.1.fld0,_15.2,_4.1.fld0,_4.1.fld0];
_4.0 = [_15.2,_15.2,_15.2,_4.1.fld0,_15.2,_4.1.fld0,_4.1.fld0,_4.1.fld0];
_15 = (257200057771276971296239255953457607149_u128, _6.0, _4.1.fld0, RET);
_15.2 = !_4.1.fld0;
SetDiscriminant(_18, 2);
_19 = [_10,_10,_15.1,_10,_10,_6.0,_15.1,_6.0];
_16 = _27.2;
_15.3 = _10 as u8;
_31 = _6.0;
Goto(bb14)
}
bb14 = {
_16 = -_27.2;
_15.2 = _10 <= _15.1;
_11 = Adt32::Variant2 { fld0: _5.1 };
_29 = _15.0;
_13 = !10_i8;
_2 = (*_1) * (*_1);
_29 = (-37339612185263861076705700494371572326_i128) as u128;
_27.2 = -_9;
_15.3 = !RET;
place!(Field::<u64>(Variant(_11, 2), 0)) = _5.1 << RET;
_4.1.fld0 = _2 != (*_1);
_35 = (-31237543419130072085773260317184275341_i128);
_2 = _3 << _14;
_13 = -(-39_i8);
_27.0 = -_16;
_4.1.fld1 = core::ptr::addr_of_mut!((*_17));
_5 = (0_usize, Field::<u64>(Variant(_11, 2), 0));
_10 = _31;
_7 = _22;
_13 = -(-96_i8);
_5 = (0_usize, Field::<u64>(Variant(_11, 2), 0));
_16 = -_9;
_20 = core::ptr::addr_of_mut!(_27.1);
_15.1 = _6.0;
SetDiscriminant(_11, 1);
_26 = core::ptr::addr_of_mut!(_36.0);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(1_usize, 2_usize, Move(_2), 8_usize, Move(_8), 5_usize, Move(_5), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(1_usize, 31_usize, Move(_31), 15_usize, Move(_15), 41_usize, _41, 41_usize, _41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: &'static i16,mut _2: (usize, u64),mut _3: usize,mut _4: u64,mut _5: u64,mut _6: (usize, u64),mut _7: u64,mut _8: u64) -> Adt33 {
mir! {
type RET = Adt33;
let _9: [i32; 3];
let _10: char;
let _11: usize;
let _12: &'static u8;
let _13: (usize, u64);
let _14: i8;
let _15: *const Adt29;
let _16: (f32, u32, f32, [i16; 5]);
let _17: i64;
let _18: [i16; 1];
let _19: u16;
let _20: usize;
let _21: f32;
let _22: f32;
let _23: *mut isize;
let _24: u128;
let _25: *const Adt29;
let _26: [u8; 5];
let _27: *mut &'static u128;
let _28: (u128, char, bool, u8);
let _29: i128;
let _30: Adt29;
let _31: &'static [u64; 7];
let _32: &'static [u64; 7];
let _33: &'static ([i16; 1],);
let _34: u8;
let _35: &'static u128;
let _36: f32;
let _37: [char; 8];
let _38: [i8; 5];
let _39: i16;
let _40: *mut *mut u8;
let _41: u16;
let _42: *const Adt29;
let _43: &'static [i16; 1];
let _44: Adt29;
let _45: isize;
let _46: u8;
let _47: char;
let _48: ([char; 8],);
let _49: [char; 8];
let _50: i64;
let _51: &'static i16;
let _52: f64;
let _53: isize;
let _54: u32;
let _55: [i32; 3];
let _56: isize;
let _57: i16;
let _58: *const Adt29;
let _59: isize;
let _60: isize;
let _61: i64;
let _62: u8;
let _63: Adt21;
let _64: isize;
let _65: (char, [u32; 8]);
let _66: bool;
let _67: f64;
let _68: (usize, u64);
let _69: ([char; 8],);
let _70: &'static *const (u128, char, bool, u8);
let _71: (char, *mut u8);
let _72: isize;
let _73: f64;
let _74: isize;
let _75: [bool; 8];
let _76: ();
let _77: ();
{
RET.fld5 = [_5,_8,_7,_4,_2.1,_2.1,_5];
RET.fld3 = [(-142908223504860061545711823369013716254_i128),80426974535841506612648052453768645996_i128,(-165541533106429550012868354627386680981_i128),26856068079902044873619927339333674249_i128,50124638879969056921451252397893764637_i128,56359987250147707215550266975258590066_i128];
RET.fld4 = 16342_u16 - 49804_u16;
RET.fld0 = !false;
_6 = (_3, _8);
RET.fld2 = 273991853564506899565000983108624193697_u128 as f64;
_6.1 = (-6_i8) as u64;
RET.fld0 = !false;
_8 = 9223372036854775807_isize as u64;
Goto(bb1)
}
bb1 = {
_6.0 = _3 & _3;
_8 = !_6.1;
RET.fld5 = [_2.1,_7,_2.1,_8,_2.1,_5,_2.1];
_2 = _6;
_6.0 = _2.0 << _7;
RET.fld5 = [_4,_8,_2.1,_7,_2.1,_5,_6.1];
Call(_8 = fn3(RET.fld5, _2, RET.fld3, _2.0, _2.1, _6.0, RET.fld3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb3 = {
RET.fld3 = [18610788397303155847628675337502142037_i128,(-52530838198014074922287387094996065313_i128),95625546437379563370485428296528183493_i128,213593986745348845538007140213234_i128,151720419135436188105384189998389653463_i128,(-107753457355491146981309225519546036181_i128)];
RET.fld5 = [_5,_7,_8,_8,_5,_5,_7];
RET.fld3 = [(-122921217584939564190575773580585000735_i128),(-157408735733363305607640939782023085989_i128),(-131445454356986944780100482679615270581_i128),(-135739122555705161876481134525444401585_i128),162232781433680587182885388452505595424_i128,(-37917376465049059052320907365446056174_i128)];
RET.fld3 = [141254776058822437733468325966209238421_i128,(-108399194588423966326390425259343675835_i128),(-129633311124674525912389685823233591255_i128),36369153051319020456603267900416899627_i128,(-26793860580570612109371983667886517646_i128),(-85784132763775101069089286701714188612_i128)];
RET.fld5 = [_8,_5,_2.1,_8,_8,_8,_8];
RET.fld3 = [(-122116691002733293801970549755951939017_i128),(-43387169805318054021470259546598874871_i128),(-152373005172875068984738702501363691656_i128),169998187267577613024467666590147982472_i128,121774019772141639999786416136194015036_i128,159235528408815175823178610488670042343_i128];
RET.fld3 = [51544667096794795743017568326411952977_i128,12651493487949683039166052274823471158_i128,(-154139576694292432852910972278559846376_i128),(-115498521457677891618740024409424933948_i128),(-38264552055329655883500388624632412418_i128),(-24735125787546620988872173374851903991_i128)];
RET.fld5 = [_7,_8,_7,_8,_6.1,_8,_8];
_2 = (_6.0, _8);
_6 = _2;
RET.fld0 = false;
_8 = 8935531020917574929_i64 as u64;
RET.fld5 = [_2.1,_6.1,_2.1,_6.1,_5,_6.1,_4];
RET.fld2 = (-11167_i16) as f64;
_8 = _6.1 - _2.1;
RET.fld4 = 26602_u16 * 53073_u16;
RET.fld4 = 5714457371505588070_i64 as u16;
RET.fld2 = (-108_i8) as f64;
_8 = 100_u8 as u64;
Goto(bb4)
}
bb4 = {
_9 = [(-889315560_i32),338429871_i32,1834247226_i32];
_2.1 = _6.1;
_6.1 = 51456684_i32 as u64;
_2.1 = !_4;
RET.fld4 = 576042662_u32 as u16;
_3 = !_6.0;
RET.fld2 = 645300955_i32 as f64;
RET.fld2 = (-33007485532046488581170137031845260057_i128) as f64;
RET.fld0 = _2.0 == _2.0;
_4 = !_2.1;
_5 = !_2.1;
_9 = [2022598592_i32,2011718797_i32,695683292_i32];
_5 = !_7;
_2 = _6;
_2.1 = _8 % _7;
RET.fld4 = !40566_u16;
match _7 {
10498020383734774716 => bb6,
_ => bb5
}
}
bb5 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb6 = {
_4 = RET.fld0 as u64;
_13.0 = !_3;
_2 = _6;
_11 = 52_i8 as usize;
RET.fld4 = _4 as u16;
_3 = _6.0 - _6.0;
Goto(bb7)
}
bb7 = {
_7 = _5 - _4;
_2.1 = !_7;
Goto(bb8)
}
bb8 = {
_13.0 = (-9223372036854775808_isize) as usize;
RET.fld5 = [_7,_7,_2.1,_7,_7,_7,_4];
RET.fld2 = (-23486_i16) as f64;
_7 = RET.fld0 as u64;
RET.fld2 = (-2021518479101652697_i64) as f64;
RET.fld0 = true;
RET.fld4 = 144251369760555524353140873022170258255_u128 as u16;
RET.fld5 = [_7,_2.1,_2.1,_7,_2.1,_4,_5];
RET.fld0 = _2.1 < _7;
_7 = _6.0 as u64;
Goto(bb9)
}
bb9 = {
_13 = (_2.0, _4);
_2.1 = _7 - _5;
_16.3 = [(-8290_i16),(-13602_i16),(-15390_i16),30901_i16,16492_i16];
_5 = 18344_i16 as u64;
RET.fld3 = [146150220635077170559634942311194756956_i128,95232860729226422898298809277723112128_i128,(-56067929022067591159655995886147092377_i128),128737729000693884148282595503437542290_i128,(-42698649550155865433158898950702334912_i128),113110406471171151156809402133021478553_i128];
_10 = '\u{10f919}';
RET.fld4 = !50740_u16;
_11 = _3 << _13.0;
_7 = !_13.1;
_13.1 = _8;
_16.0 = 38018256895568658590306965081247181310_i128 as f32;
_16.2 = _16.0 - _16.0;
_13.1 = !_4;
RET.fld3 = [19372252978184639927991437240253387881_i128,(-90370248252395118413630713590449568870_i128),165257500922385092407174744176575715491_i128,139149279248905777122684065136688074113_i128,64799204631160757890416510728735606162_i128,134145864890182731962223140045559992822_i128];
_11 = (-526295022_i32) as usize;
RET.fld0 = !true;
_13.0 = !_2.0;
_17 = -(-945757759799074019_i64);
_2 = (_3, _13.1);
RET.fld5 = [_7,_7,_13.1,_13.1,_2.1,_4,_2.1];
_13 = (_2.0, _7);
_18 = [5378_i16];
Goto(bb10)
}
bb10 = {
_20 = _2.0;
_10 = '\u{1bf87}';
_14 = (-10_i8);
RET.fld0 = true;
RET.fld2 = _16.2 as f64;
_18 = [20181_i16];
_22 = _16.0;
_11 = _20;
RET.fld0 = false;
_21 = _16.0 * _16.0;
_16.2 = -_21;
RET.fld2 = 714016586_u32 as f64;
_2.0 = _20 | _11;
RET.fld3 = [(-113405760473967554456204376468351215989_i128),(-55393266039853907008306685247464082645_i128),(-55534597979891481879228937369757097264_i128),150254802311505353646843659845778386024_i128,(-75373794925796267831038106503683878915_i128),(-74199281820236404669210532713074553953_i128)];
_2.1 = !_7;
_6.0 = _11 - _2.0;
_4 = !_13.1;
_8 = _4;
_6.0 = !_11;
RET.fld5 = [_13.1,_4,_2.1,_8,_5,_7,_8];
_2.1 = _8 << _6.0;
RET.fld0 = _8 > _8;
_16.0 = (-93780058871757226487524566215628193086_i128) as f32;
_16.3 = [10543_i16,31690_i16,26162_i16,4086_i16,(-13432_i16)];
_10 = '\u{2325c}';
RET.fld0 = !false;
_11 = RET.fld2 as usize;
match _14 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb11,
4 => bb12,
340282366920938463463374607431768211446 => bb14,
_ => bb13
}
}
bb11 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb12 = {
RET.fld3 = [18610788397303155847628675337502142037_i128,(-52530838198014074922287387094996065313_i128),95625546437379563370485428296528183493_i128,213593986745348845538007140213234_i128,151720419135436188105384189998389653463_i128,(-107753457355491146981309225519546036181_i128)];
RET.fld5 = [_5,_7,_8,_8,_5,_5,_7];
RET.fld3 = [(-122921217584939564190575773580585000735_i128),(-157408735733363305607640939782023085989_i128),(-131445454356986944780100482679615270581_i128),(-135739122555705161876481134525444401585_i128),162232781433680587182885388452505595424_i128,(-37917376465049059052320907365446056174_i128)];
RET.fld3 = [141254776058822437733468325966209238421_i128,(-108399194588423966326390425259343675835_i128),(-129633311124674525912389685823233591255_i128),36369153051319020456603267900416899627_i128,(-26793860580570612109371983667886517646_i128),(-85784132763775101069089286701714188612_i128)];
RET.fld5 = [_8,_5,_2.1,_8,_8,_8,_8];
RET.fld3 = [(-122116691002733293801970549755951939017_i128),(-43387169805318054021470259546598874871_i128),(-152373005172875068984738702501363691656_i128),169998187267577613024467666590147982472_i128,121774019772141639999786416136194015036_i128,159235528408815175823178610488670042343_i128];
RET.fld3 = [51544667096794795743017568326411952977_i128,12651493487949683039166052274823471158_i128,(-154139576694292432852910972278559846376_i128),(-115498521457677891618740024409424933948_i128),(-38264552055329655883500388624632412418_i128),(-24735125787546620988872173374851903991_i128)];
RET.fld5 = [_7,_8,_7,_8,_6.1,_8,_8];
_2 = (_6.0, _8);
_6 = _2;
RET.fld0 = false;
_8 = 8935531020917574929_i64 as u64;
RET.fld5 = [_2.1,_6.1,_2.1,_6.1,_5,_6.1,_4];
RET.fld2 = (-11167_i16) as f64;
_8 = _6.1 - _2.1;
RET.fld4 = 26602_u16 * 53073_u16;
RET.fld4 = 5714457371505588070_i64 as u16;
RET.fld2 = (-108_i8) as f64;
_8 = 100_u8 as u64;
Goto(bb4)
}
bb13 = {
_6.0 = _3 & _3;
_8 = !_6.1;
RET.fld5 = [_2.1,_7,_2.1,_8,_2.1,_5,_2.1];
_2 = _6;
_6.0 = _2.0 << _7;
RET.fld5 = [_4,_8,_2.1,_7,_2.1,_5,_6.1];
Call(_8 = fn3(RET.fld5, _2, RET.fld3, _2.0, _2.1, _6.0, RET.fld3), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
RET.fld4 = !45433_u16;
_13 = _6;
_21 = -_16.2;
_20 = !_2.0;
_21 = _22;
_5 = RET.fld0 as u64;
_5 = _8;
Goto(bb15)
}
bb15 = {
_6 = _13;
_16.1 = !3886465621_u32;
_3 = _20 * _6.0;
_6.1 = _20 as u64;
_16.1 = !1258511440_u32;
RET.fld5 = [_2.1,_5,_6.1,_5,_6.1,_2.1,_2.1];
_3 = _16.1 as usize;
RET.fld0 = !true;
_16.0 = (-1202016024_i32) as f32;
_2.1 = _8 - _6.1;
RET.fld0 = false & false;
_2.1 = !_4;
_16.3 = [(-5271_i16),(-25594_i16),(-18754_i16),(-25774_i16),29740_i16];
_7 = !_8;
_22 = _16.0 * _21;
_6.0 = (-1196549879_i32) as usize;
_16.3 = [(-23734_i16),(-12427_i16),18537_i16,(-22982_i16),(-5116_i16)];
_17 = 202047098051663209_i64 * (-1737097122145965861_i64);
_18 = [5068_i16];
_6.0 = RET.fld2 as usize;
_4 = !_8;
_16.2 = _22 + _22;
_13.0 = _20 >> _7;
_6.0 = _13.0;
_6 = (_13.0, _8);
_16.1 = !2532328000_u32;
_17 = 9119833108582014023_i64;
_4 = _17 as u64;
Goto(bb16)
}
bb16 = {
_18 = [(-21448_i16)];
RET.fld2 = 301744382_i32 as f64;
RET.fld0 = !false;
_16.1 = !2428305409_u32;
_11 = _5 as usize;
_16.3 = [(-18796_i16),22065_i16,15915_i16,(-20547_i16),16438_i16];
_14 = _20 as i8;
_24 = 19695574456622391390428332558768032103_u128 | 91062590422536518209915960340877023105_u128;
_3 = RET.fld0 as usize;
_6.0 = _2.0 & _13.0;
_10 = '\u{bdcce}';
_2 = (_11, _4);
_26 = [189_u8,82_u8,247_u8,254_u8,236_u8];
RET.fld0 = !true;
_13.1 = _6.0 as u64;
RET.fld4 = 36093_u16 | 31177_u16;
_16.2 = 77_isize as f32;
_24 = !217807929424997967003208377398494597464_u128;
_3 = _16.1 as usize;
_2 = _13;
_16.1 = RET.fld2 as u32;
_17 = !1808725734035060721_i64;
_8 = _13.1 & _6.1;
_22 = _21;
_11 = _6.0;
_2.0 = _20;
Goto(bb17)
}
bb17 = {
_28.1 = _10;
_13.0 = _11;
_12 = &_28.3;
_6.1 = _8 << _7;
_25 = core::ptr::addr_of!(_30);
_15 = core::ptr::addr_of!((*_25));
_15 = core::ptr::addr_of!((*_25));
_28.0 = _24;
Call(_29 = core::intrinsics::bswap(110474738177823581406548496513275146743_i128), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
RET.fld4 = _24 as u16;
_32 = &RET.fld5;
Goto(bb19)
}
bb19 = {
_6.1 = _22 as u64;
_34 = !43_u8;
Goto(bb20)
}
bb20 = {
_8 = !_7;
_25 = core::ptr::addr_of!(_30);
_16.2 = -_21;
_28.3 = !_34;
_36 = _16.2 - _21;
_8 = _2.1;
_14 = (-103_i8);
_19 = RET.fld4;
_28.2 = !RET.fld0;
_28.1 = _10;
RET.fld5 = [_8,_5,_13.1,_8,_2.1,_13.1,_8];
_16.0 = _16.2;
Goto(bb21)
}
bb21 = {
_36 = -_16.0;
_7 = _8;
_2.0 = _6.0;
_13 = _6;
_27 = core::ptr::addr_of_mut!(_35);
_21 = _34 as f32;
RET.fld4 = _28.0 as u16;
_24 = !_28.0;
_28.1 = _10;
_14 = (-9223372036854775808_isize) as i8;
Goto(bb22)
}
bb22 = {
_17 = _28.0 as i64;
(*_27) = &_24;
_7 = _8;
_7 = RET.fld0 as u64;
_2 = _6;
_12 = &_28.3;
_31 = &RET.fld5;
_16.3 = [6582_i16,(-2583_i16),(-29327_i16),(-14929_i16),(-18601_i16)];
_28.3 = !_34;
_15 = core::ptr::addr_of!(_44);
RET.fld4 = _19 & _19;
_38 = [_14,_14,_14,_14,_14];
Goto(bb23)
}
bb23 = {
_42 = core::ptr::addr_of!((*_15));
_31 = &(*_31);
_12 = &_34;
_36 = _22;
_31 = &RET.fld5;
RET.fld4 = (-37_isize) as u16;
_16.2 = (*_35) as f32;
_31 = &(*_31);
Goto(bb24)
}
bb24 = {
RET.fld0 = _28.2;
_28.3 = _34;
_20 = _2.0 >> _11;
_31 = &(*_31);
RET.fld5 = [_5,_8,_8,_8,_8,_8,_5];
_23 = core::ptr::addr_of_mut!(_45);
_4 = _8;
_38 = [_14,_14,_14,_14,_14];
Goto(bb25)
}
bb25 = {
_16.0 = _21 + _36;
_18 = [(-5064_i16)];
_4 = !_8;
RET.fld3 = [(-117274486152984899134660427684841296846_i128),93151694748913669202098188245817939056_i128,37170446612693312920668507393245901941_i128,74295530182329480751082710879551390616_i128,(-61322426880750802900023708922616604436_i128),147678353993511234413893271249210743990_i128];
_6 = (_2.0, _4);
_21 = -_16.0;
(*_23) = -11_isize;
_13.0 = RET.fld4 as usize;
RET.fld2 = (-1260104539_i32) as f64;
_9 = [(-987437019_i32),1694686902_i32,(-1650610655_i32)];
_16.3 = [14647_i16,(-24775_i16),(-13312_i16),32174_i16,(-10854_i16)];
_34 = _28.3;
_37 = [_10,_28.1,_28.1,_10,_10,_10,_10,_10];
_42 = core::ptr::addr_of!((*_15));
_20 = _6.0;
_5 = _28.3 as u64;
_10 = _28.1;
_26 = [_34,_28.3,_34,_34,_28.3];
_26 = [_28.3,_28.3,_34,_28.3,_34];
_2.1 = _4 >> _11;
RET.fld3 = [86032024625665764694955597246808558410_i128,(-38675312479230448738511384083494341704_i128),(-8944643253726857492197824733927060650_i128),121697593769330251494928326911667740871_i128,(-161311820811302904021664941429988804328_i128),(-59435109783435701072119150792608117730_i128)];
(*_23) = 30_isize >> _20;
Goto(bb26)
}
bb26 = {
(*_23) = 9223372036854775807_isize >> _2.1;
_49 = [_28.1,_10,_28.1,_10,_10,_10,_28.1,_10];
_28.3 = _34 + _34;
_48 = (_37,);
_52 = _28.3 as f64;
_37 = [_28.1,_10,_10,_10,_28.1,_10,_10,_28.1];
_51 = &_39;
RET.fld3 = [(-20580779231888490424860717764589744392_i128),(-87407337743680116596037609726559859610_i128),(-30864404190596936632884252561831201657_i128),87974975731394642881281660932871495752_i128,(-115353451890856438330571457944358034136_i128),(-58384845954760802548199877220487814627_i128)];
_46 = _28.3;
_16.0 = _2.0 as f32;
_45 = 100_isize;
_19 = RET.fld4;
RET.fld4 = _19;
_3 = _20;
_36 = _16.0 - _16.0;
_12 = &_46;
_15 = core::ptr::addr_of!((*_25));
_27 = core::ptr::addr_of_mut!((*_27));
_14 = (-48_i8);
_43 = &_18;
RET.fld3 = [140890920289082003632225821248369630199_i128,(-127396329231510171674584789825979735102_i128),131864715948768185409690547596474739443_i128,144242710109601089448097458571179506967_i128,(-123461420553289321663247214443849807102_i128),(-112892610911628016661343504383805162991_i128)];
_44 = Adt29::Variant2 { fld0: (*_12) };
Goto(bb27)
}
bb27 = {
_13.0 = _28.1 as usize;
_13.0 = (*_23) as usize;
_39 = _45 as i16;
_16.2 = _16.0;
_1 = &_39;
_37 = _49;
_4 = _8 + _6.1;
(*_25) = Adt29::Variant2 { fld0: (*_12) };
_46 = _10 as u8;
Goto(bb28)
}
bb28 = {
_50 = !_17;
_30 = Move(_44);
_26 = [_46,Field::<u8>(Variant((*_25), 2), 0),Field::<u8>(Variant((*_25), 2), 0),_46,Field::<u8>(Variant((*_25), 2), 0)];
RET.fld0 = !_28.2;
(*_27) = &(*_35);
_34 = Field::<u8>(Variant((*_25), 2), 0) << _3;
place!(Field::<u8>(Variant((*_15), 2), 0)) = _34;
_52 = RET.fld2;
_28.0 = _8 as u128;
_54 = _14 as u32;
_50 = !_17;
RET.fld4 = _19;
_25 = core::ptr::addr_of!(_44);
_2.1 = _6.1 & _4;
_24 = _28.0 << _4;
_56 = !_45;
_10 = _28.1;
RET.fld4 = _19;
(*_23) = _56 >> Field::<u8>(Variant(_30, 2), 0);
_57 = (*_1);
_53 = _28.0 as isize;
_44 = Move((*_15));
_57 = !(*_1);
_53 = !_45;
_10 = _28.1;
_11 = _16.2 as usize;
match _14 {
0 => bb29,
1 => bb30,
2 => bb31,
340282366920938463463374607431768211408 => bb33,
_ => bb32
}
}
bb29 = {
_13 = (_2.0, _4);
_2.1 = _7 - _5;
_16.3 = [(-8290_i16),(-13602_i16),(-15390_i16),30901_i16,16492_i16];
_5 = 18344_i16 as u64;
RET.fld3 = [146150220635077170559634942311194756956_i128,95232860729226422898298809277723112128_i128,(-56067929022067591159655995886147092377_i128),128737729000693884148282595503437542290_i128,(-42698649550155865433158898950702334912_i128),113110406471171151156809402133021478553_i128];
_10 = '\u{10f919}';
RET.fld4 = !50740_u16;
_11 = _3 << _13.0;
_7 = !_13.1;
_13.1 = _8;
_16.0 = 38018256895568658590306965081247181310_i128 as f32;
_16.2 = _16.0 - _16.0;
_13.1 = !_4;
RET.fld3 = [19372252978184639927991437240253387881_i128,(-90370248252395118413630713590449568870_i128),165257500922385092407174744176575715491_i128,139149279248905777122684065136688074113_i128,64799204631160757890416510728735606162_i128,134145864890182731962223140045559992822_i128];
_11 = (-526295022_i32) as usize;
RET.fld0 = !true;
_13.0 = !_2.0;
_17 = -(-945757759799074019_i64);
_2 = (_3, _13.1);
RET.fld5 = [_7,_7,_13.1,_13.1,_2.1,_4,_2.1];
_13 = (_2.0, _7);
_18 = [5378_i16];
Goto(bb10)
}
bb30 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb31 = {
_42 = core::ptr::addr_of!((*_15));
_31 = &(*_31);
_12 = &_34;
_36 = _22;
_31 = &RET.fld5;
RET.fld4 = (-37_isize) as u16;
_16.2 = (*_35) as f32;
_31 = &(*_31);
Goto(bb24)
}
bb32 = {
_20 = _2.0;
_10 = '\u{1bf87}';
_14 = (-10_i8);
RET.fld0 = true;
RET.fld2 = _16.2 as f64;
_18 = [20181_i16];
_22 = _16.0;
_11 = _20;
RET.fld0 = false;
_21 = _16.0 * _16.0;
_16.2 = -_21;
RET.fld2 = 714016586_u32 as f64;
_2.0 = _20 | _11;
RET.fld3 = [(-113405760473967554456204376468351215989_i128),(-55393266039853907008306685247464082645_i128),(-55534597979891481879228937369757097264_i128),150254802311505353646843659845778386024_i128,(-75373794925796267831038106503683878915_i128),(-74199281820236404669210532713074553953_i128)];
_2.1 = !_7;
_6.0 = _11 - _2.0;
_4 = !_13.1;
_8 = _4;
_6.0 = !_11;
RET.fld5 = [_13.1,_4,_2.1,_8,_5,_7,_8];
_2.1 = _8 << _6.0;
RET.fld0 = _8 > _8;
_16.0 = (-93780058871757226487524566215628193086_i128) as f32;
_16.3 = [10543_i16,31690_i16,26162_i16,4086_i16,(-13432_i16)];
_10 = '\u{2325c}';
RET.fld0 = !false;
_11 = RET.fld2 as usize;
match _14 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb11,
4 => bb12,
340282366920938463463374607431768211446 => bb14,
_ => bb13
}
}
bb33 = {
(*_27) = &_28.0;
SetDiscriminant((*_25), 3);
_48.0 = _49;
RET.fld3 = [(-164851111630335094430757176534948603339_i128),54385614098329248769593474567184165363_i128,55515450358227088103101855485314913640_i128,(-108689024469032372563012018762392216845_i128),60182632392998931019943204848377862875_i128,(-66703170405369558294022508865805313001_i128)];
Goto(bb34)
}
bb34 = {
place!(Field::<*mut *mut *mut u8>(Variant((*_25), 3), 0)) = core::ptr::addr_of_mut!(place!(Field::<*mut *mut u8>(Variant(_44, 3), 1)));
(*_23) = _53;
_63 = Adt21 { fld0: _34,fld1: (*_43),fld2: (-96324077659975637038174995158907081222_i128),fld3: _14 };
_2.1 = _16.1 as u64;
_2.1 = _6.1 & _8;
_65.1 = [_54,_16.1,_54,_54,_54,_16.1,_16.1,_54];
_16.3 = [_57,(*_1),(*_1),_57,_39];
place!(Field::<[i16; 1]>(Variant((*_25), 3), 4)) = _18;
RET.fld0 = _24 <= (*_35);
(*_23) = _53;
_28.1 = _10;
RET.fld4 = _19;
_60 = _53;
_8 = _4 * _4;
RET.fld4 = _63.fld3 as u16;
_63.fld2 = (-73403866879126312690158979584548738170_i128);
place!(Field::<i32>(Variant((*_25), 3), 3)) = 286334437_i32 << _2.0;
_51 = Move(_1);
_55 = _9;
_41 = _19;
(*_42) = Adt29::Variant2 { fld0: _34 };
_30 = Move((*_25));
_63.fld0 = !Field::<u8>(Variant(_30, 2), 0);
_64 = _60;
_26 = [Field::<u8>(Variant((*_15), 2), 0),Field::<u8>(Variant((*_15), 2), 0),Field::<u8>(Variant((*_15), 2), 0),Field::<u8>(Variant((*_15), 2), 0),Field::<u8>(Variant((*_15), 2), 0)];
match _63.fld2 {
0 => bb4,
1 => bb35,
2 => bb36,
3 => bb37,
4 => bb38,
5 => bb39,
6 => bb40,
266878500041812150773215627847219473286 => bb42,
_ => bb41
}
}
bb35 = {
_36 = -_16.0;
_7 = _8;
_2.0 = _6.0;
_13 = _6;
_27 = core::ptr::addr_of_mut!(_35);
_21 = _34 as f32;
RET.fld4 = _28.0 as u16;
_24 = !_28.0;
_28.1 = _10;
_14 = (-9223372036854775808_isize) as i8;
Goto(bb22)
}
bb36 = {
RET.fld4 = _24 as u16;
_32 = &RET.fld5;
Goto(bb19)
}
bb37 = {
_6.0 = _3 & _3;
_8 = !_6.1;
RET.fld5 = [_2.1,_7,_2.1,_8,_2.1,_5,_2.1];
_2 = _6;
_6.0 = _2.0 << _7;
RET.fld5 = [_4,_8,_2.1,_7,_2.1,_5,_6.1];
Call(_8 = fn3(RET.fld5, _2, RET.fld3, _2.0, _2.1, _6.0, RET.fld3), ReturnTo(bb2), UnwindUnreachable())
}
bb38 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb39 = {
_18 = [(-21448_i16)];
RET.fld2 = 301744382_i32 as f64;
RET.fld0 = !false;
_16.1 = !2428305409_u32;
_11 = _5 as usize;
_16.3 = [(-18796_i16),22065_i16,15915_i16,(-20547_i16),16438_i16];
_14 = _20 as i8;
_24 = 19695574456622391390428332558768032103_u128 | 91062590422536518209915960340877023105_u128;
_3 = RET.fld0 as usize;
_6.0 = _2.0 & _13.0;
_10 = '\u{bdcce}';
_2 = (_11, _4);
_26 = [189_u8,82_u8,247_u8,254_u8,236_u8];
RET.fld0 = !true;
_13.1 = _6.0 as u64;
RET.fld4 = 36093_u16 | 31177_u16;
_16.2 = 77_isize as f32;
_24 = !217807929424997967003208377398494597464_u128;
_3 = _16.1 as usize;
_2 = _13;
_16.1 = RET.fld2 as u32;
_17 = !1808725734035060721_i64;
_8 = _13.1 & _6.1;
_22 = _21;
_11 = _6.0;
_2.0 = _20;
Goto(bb17)
}
bb40 = {
RET.fld4 = !45433_u16;
_13 = _6;
_21 = -_16.2;
_20 = !_2.0;
_21 = _22;
_5 = RET.fld0 as u64;
_5 = _8;
Goto(bb15)
}
bb41 = {
RET.fld0 = _28.2;
_28.3 = _34;
_20 = _2.0 >> _11;
_31 = &(*_31);
RET.fld5 = [_5,_8,_8,_8,_8,_8,_5];
_23 = core::ptr::addr_of_mut!(_45);
_4 = _8;
_38 = [_14,_14,_14,_14,_14];
Goto(bb25)
}
bb42 = {
RET.fld5 = [_2.1,_4,_2.1,_2.1,_8,_8,_2.1];
_31 = &RET.fld5;
_69 = (_48.0,);
RET.fld4 = Field::<u8>(Variant(_30, 2), 0) as u16;
_22 = _36;
_65.0 = _28.1;
SetDiscriminant((*_15), 0);
place!(Field::<u64>(Variant(_30, 0), 2)) = _2.1;
place!(Field::<Adt21>(Variant((*_15), 0), 3)).fld1 = (*_43);
place!(Field::<u128>(Variant(_30, 0), 1)) = _50 as u128;
place!(Field::<i16>(Variant((*_15), 0), 4)) = _39 * _39;
place!(Field::<u128>(Variant((*_15), 0), 1)) = (*_35) + (*_35);
_15 = core::ptr::addr_of!((*_42));
match _63.fld2 {
0 => bb21,
1 => bb30,
2 => bb43,
3 => bb44,
4 => bb45,
5 => bb46,
6 => bb47,
266878500041812150773215627847219473286 => bb49,
_ => bb48
}
}
bb43 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb44 = {
(*_23) = 9223372036854775807_isize >> _2.1;
_49 = [_28.1,_10,_28.1,_10,_10,_10,_28.1,_10];
_28.3 = _34 + _34;
_48 = (_37,);
_52 = _28.3 as f64;
_37 = [_28.1,_10,_10,_10,_28.1,_10,_10,_28.1];
_51 = &_39;
RET.fld3 = [(-20580779231888490424860717764589744392_i128),(-87407337743680116596037609726559859610_i128),(-30864404190596936632884252561831201657_i128),87974975731394642881281660932871495752_i128,(-115353451890856438330571457944358034136_i128),(-58384845954760802548199877220487814627_i128)];
_46 = _28.3;
_16.0 = _2.0 as f32;
_45 = 100_isize;
_19 = RET.fld4;
RET.fld4 = _19;
_3 = _20;
_36 = _16.0 - _16.0;
_12 = &_46;
_15 = core::ptr::addr_of!((*_25));
_27 = core::ptr::addr_of_mut!((*_27));
_14 = (-48_i8);
_43 = &_18;
RET.fld3 = [140890920289082003632225821248369630199_i128,(-127396329231510171674584789825979735102_i128),131864715948768185409690547596474739443_i128,144242710109601089448097458571179506967_i128,(-123461420553289321663247214443849807102_i128),(-112892610911628016661343504383805162991_i128)];
_44 = Adt29::Variant2 { fld0: (*_12) };
Goto(bb27)
}
bb45 = {
_17 = _28.0 as i64;
(*_27) = &_24;
_7 = _8;
_7 = RET.fld0 as u64;
_2 = _6;
_12 = &_28.3;
_31 = &RET.fld5;
_16.3 = [6582_i16,(-2583_i16),(-29327_i16),(-14929_i16),(-18601_i16)];
_28.3 = !_34;
_15 = core::ptr::addr_of!(_44);
RET.fld4 = _19 & _19;
_38 = [_14,_14,_14,_14,_14];
Goto(bb23)
}
bb46 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb47 = {
RET.fld5 = [_8,_8,_8,_8,_7,_8,_8];
_3 = 324101906_i32 as usize;
Goto(bb3)
}
bb48 = {
_6.0 = _3 & _3;
_8 = !_6.1;
RET.fld5 = [_2.1,_7,_2.1,_8,_2.1,_5,_2.1];
_2 = _6;
_6.0 = _2.0 << _7;
RET.fld5 = [_4,_8,_2.1,_7,_2.1,_5,_6.1];
Call(_8 = fn3(RET.fld5, _2, RET.fld3, _2.0, _2.1, _6.0, RET.fld3), ReturnTo(bb2), UnwindUnreachable())
}
bb49 = {
_11 = !_2.0;
Goto(bb50)
}
bb50 = {
_1 = &place!(Field::<i16>(Variant(_30, 0), 4));
_20 = _6.0 - _2.0;
_68.0 = _14 as usize;
place!(Field::<Adt21>(Variant(_30, 0), 3)) = _63;
_58 = core::ptr::addr_of!((*_15));
_59 = !_64;
(*_23) = _53;
_6.0 = _17 as usize;
_22 = _36;
RET.fld5 = [_2.1,_6.1,_4,_6.1,_4,_2.1,_8];
_28 = (_24, _10, RET.fld0, Field::<Adt21>(Variant(_30, 0), 3).fld0);
place!(Field::<Adt21>(Variant(_30, 0), 3)).fld0 = _34 | _28.3;
(*_42) = Adt29::Variant2 { fld0: _34 };
RET.fld5 = [_6.1,_2.1,_6.1,_2.1,_8,_8,Field::<u64>(Variant(_30, 0), 2)];
_1 = &_39;
_47 = _65.0;
_1 = &_39;
RET.fld1 = core::ptr::addr_of_mut!(place!(Field::<*mut u8>(Variant(_30, 0), 5)));
Goto(bb51)
}
bb51 = {
Call(_76 = dump_var(2_usize, 41_usize, Move(_41), 39_usize, Move(_39), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_76 = dump_var(2_usize, 10_usize, Move(_10), 60_usize, Move(_60), 28_usize, Move(_28), 18_usize, Move(_18)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_76 = dump_var(2_usize, 5_usize, Move(_5), 7_usize, Move(_7), 69_usize, Move(_69), 56_usize, Move(_56)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_76 = dump_var(2_usize, 38_usize, Move(_38), 2_usize, Move(_2), 54_usize, Move(_54), 53_usize, Move(_53)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_76 = dump_var(2_usize, 6_usize, Move(_6), 47_usize, Move(_47), 20_usize, Move(_20), 59_usize, Move(_59)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [u64; 7],mut _2: (usize, u64),mut _3: [i128; 6],mut _4: usize,mut _5: u64,mut _6: usize,mut _7: [i128; 6]) -> u64 {
mir! {
type RET = u64;
let _8: isize;
let _9: i32;
let _10: u64;
let _11: *const &'static *mut u8;
let _12: [u32; 8];
let _13: isize;
let _14: isize;
let _15: [char; 8];
let _16: [u64; 7];
let _17: isize;
let _18: &'static Adt40;
let _19: bool;
let _20: u8;
let _21: *const u16;
let _22: u64;
let _23: char;
let _24: &'static usize;
let _25: (char, *mut u8);
let _26: u128;
let _27: bool;
let _28: *mut *mut u8;
let _29: (u128, char, bool, u8);
let _30: &'static [u8; 5];
let _31: &'static (f32, u32, f32, [i16; 5]);
let _32: ([i16; 1],);
let _33: u8;
let _34: *const Adt29;
let _35: f32;
let _36: [i8; 5];
let _37: [i16; 5];
let _38: isize;
let _39: f32;
let _40: [u64; 7];
let _41: [u8; 5];
let _42: i64;
let _43: f32;
let _44: ();
let _45: ();
{
RET = _2.1;
_7 = [81187469365641376083784021276327701015_i128,(-84755912831883494811832990044135123876_i128),49420976084866963402982646824048134593_i128,(-43838930032709617458180814933667963482_i128),(-74759768649790893347513268768264527470_i128),(-96156383863866315457709182327524266593_i128)];
_2.0 = '\u{4e02f}' as usize;
_9 = (-137256101641198543563129864492554603193_i128) as i32;
_7 = _3;
Goto(bb1)
}
bb1 = {
_9 = (-1673806648_i32) | (-1246790884_i32);
RET = 22419_i16 as u64;
_1 = [_2.1,_2.1,_5,_2.1,_5,_2.1,_2.1];
_1 = [_2.1,_2.1,_5,_5,RET,RET,RET];
_10 = _2.1 & RET;
_2.1 = _10 | _10;
_2.1 = !_10;
_6 = 110562568920593175823199221749622625934_i128 as usize;
_9 = -(-508029662_i32);
RET = !_2.1;
_9 = !2079637825_i32;
_9 = 1072922559_i32 & (-2101247935_i32);
_2 = (_4, _5);
RET = _10;
_3 = [126252903048256385886952379905276736835_i128,132437695833039029980824192196145123674_i128,(-69572877358129096853892266639291448767_i128),(-67346446399808718451932414549018109009_i128),(-34393322770828996083574185394133998629_i128),42206049163353069841685917895807866384_i128];
_2.1 = _10 | _10;
_7 = _3;
_2.0 = _4;
_7 = _3;
_8 = -122_isize;
Goto(bb2)
}
bb2 = {
_5 = RET;
_12 = [3344522142_u32,2089331458_u32,3527746742_u32,3506371714_u32,2798292375_u32,1006629672_u32,2489124506_u32,3180720144_u32];
Call(_11 = fn4(RET, _2.1, _2.1, _2.1, _2, _2.1, _7, _3, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = [_10,_5,_5,_10,RET,_2.1,_10];
_2.1 = (-6932874713835647135_i64) as u64;
_6 = !_4;
RET = _5;
Goto(bb4)
}
bb4 = {
_4 = !_2.0;
_7 = [137639976006361629798318968313410988078_i128,(-139090277965547662969913538652843745196_i128),(-62295769257532649409601079368476038372_i128),(-13394963472391683506769119904523835125_i128),(-141172169417624174948375635868410937095_i128),33129849127283877431266761238055588176_i128];
_14 = _8 ^ _8;
_6 = _2.1 as usize;
_15 = ['\u{10c216}','\u{33f40}','\u{2d4a3}','\u{913e1}','\u{105152}','\u{460c0}','\u{89fa7}','\u{f0f12}'];
_8 = 9093_u16 as isize;
_4 = !_2.0;
_12 = [3152371580_u32,2570449043_u32,484804515_u32,4227249009_u32,2244106906_u32,179215108_u32,2708048046_u32,1070969617_u32];
_14 = !_8;
_14 = 60051_u16 as isize;
RET = (-31997_i16) as u64;
_2.0 = _4;
_13 = _14;
RET = _10;
_5 = !_10;
_6 = _2.0;
_2 = (_4, _10);
Call(_10 = core::intrinsics::transmute(_2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2.1 = !_5;
_17 = -_8;
_8 = _13 << _4;
_16 = _1;
_4 = _2.0;
_6 = _2.0 | _2.0;
_5 = !_10;
_4 = !_6;
RET = (-74572397286918597884055127677455754883_i128) as u64;
_17 = 250_u8 as isize;
_9 = -(-592894559_i32);
_16 = _1;
_17 = _14;
_2.0 = _6 >> _5;
_16 = [_5,_2.1,_5,_5,_5,_2.1,_10];
Goto(bb6)
}
bb6 = {
_3 = _7;
RET = _10 & _5;
_2 = (_4, RET);
_20 = _9 as u8;
_20 = true as u8;
_17 = _8 & _13;
_25.0 = '\u{4e3c}';
_16 = _1;
_4 = _17 as usize;
_25.1 = core::ptr::addr_of_mut!(_20);
_23 = _25.0;
_24 = &_2.0;
_3 = [(-100767955426524131846373829030049141986_i128),(-45574113702407202944278729355896716009_i128),158207254709222224790665848890500480298_i128,(-146143184461068377968683356868417917471_i128),(-136465541167229463619856565792161804068_i128),94571726344305754734860136837658377273_i128];
_13 = _17 << (*_24);
_23 = _25.0;
_15 = [_23,_25.0,_23,_25.0,_23,_25.0,_23,_25.0];
_16 = [_2.1,_2.1,_2.1,_2.1,_2.1,_5,_10];
_26 = 29878912542821994466571310992621369552_u128;
_19 = false;
_13 = 1013336744_u32 as isize;
_17 = _14;
_22 = _5 + RET;
_28 = core::ptr::addr_of_mut!(_25.1);
_4 = !(*_24);
_22 = _5 << _8;
Goto(bb7)
}
bb7 = {
_24 = &(*_24);
RET = !_22;
_22 = _10;
_4 = (*_24);
_9 = -(-1280530849_i32);
Goto(bb8)
}
bb8 = {
_14 = _17;
_28 = core::ptr::addr_of_mut!(_25.1);
_4 = !_2.0;
RET = !_5;
(*_28) = core::ptr::addr_of_mut!(_20);
_25.1 = core::ptr::addr_of_mut!(_29.3);
_1 = [RET,_10,_22,_2.1,_22,_5,_5];
(*_28) = core::ptr::addr_of_mut!(_20);
_9 = -268717889_i32;
_29.2 = (*_24) >= (*_24);
_25.0 = _23;
_26 = !171873737024391206861197890403803606545_u128;
_29.1 = _25.0;
_13 = _8;
_33 = _20 | _20;
_27 = !_29.2;
_17 = (-153333655554778020281665814379317440709_i128) as isize;
_2.1 = _10 | _10;
_26 = !198376668309250689062036393586807929897_u128;
_33 = _20 + _20;
_29 = (_26, _25.0, _27, _20);
_1 = [_10,RET,_5,_10,_22,RET,_10];
_36 = [(-53_i8),(-37_i8),(-44_i8),27_i8,(-83_i8)];
Goto(bb9)
}
bb9 = {
_28 = core::ptr::addr_of_mut!(_25.1);
Goto(bb10)
}
bb10 = {
_5 = !_22;
_6 = (*_24);
_2.1 = (-55_i8) as u64;
_25.1 = core::ptr::addr_of_mut!(_33);
(*_28) = core::ptr::addr_of_mut!(_29.3);
_32.0 = [22148_i16];
_29 = (_26, _23, _19, _33);
_3 = [111429683016504242902268085883313467495_i128,(-29687863881379551117502028601677217960_i128),(-36963837002605460027527254471787224601_i128),(-40600031494761693649905437900580655803_i128),64792930137800488870153520719291527069_i128,137875294482002311218052182538114018139_i128];
_32.0 = [25570_i16];
_25.1 = core::ptr::addr_of_mut!(_33);
RET = _5 | _22;
_29.3 = _33 ^ _33;
_26 = (-11806_i16) as u128;
_20 = _33 ^ _29.3;
_4 = (*_24);
Goto(bb11)
}
bb11 = {
_37 = [13759_i16,22923_i16,(-12300_i16),12608_i16,1837_i16];
_33 = _29.3 + _20;
_26 = _29.0;
_35 = _29.3 as f32;
_7 = _3;
_8 = (-4753075634563756234_i64) as isize;
_38 = 23353_u16 as isize;
_24 = &(*_24);
_39 = _35;
_1 = [RET,_2.1,RET,RET,_2.1,RET,_5];
_6 = _4 ^ _4;
_29.1 = _25.0;
Goto(bb12)
}
bb12 = {
_8 = -_38;
_9 = (-916241291_i32);
_1 = [RET,RET,_5,_10,RET,RET,_10];
_15 = [_29.1,_25.0,_25.0,_23,_29.1,_29.1,_25.0,_25.0];
_29.1 = _23;
_2.0 = _6 * _6;
_37 = [(-30094_i16),7911_i16,20229_i16,(-15237_i16),3886_i16];
_39 = -_35;
_14 = _13 - _13;
_33 = _20 << RET;
_12 = [2770222505_u32,118465927_u32,178929736_u32,1191446734_u32,3507821724_u32,61651430_u32,998523051_u32,2954184820_u32];
_40 = [_5,_22,RET,_22,_5,_5,RET];
_15 = [_25.0,_23,_29.1,_25.0,_29.1,_29.1,_23,_23];
_5 = _10;
(*_28) = core::ptr::addr_of_mut!(_20);
_32.0 = [(-19025_i16)];
match _9 {
340282366920938463463374607430851970165 => bb13,
_ => bb7
}
}
bb13 = {
_35 = 3973975489083451126_i64 as f32;
match _9 {
0 => bb14,
340282366920938463463374607430851970165 => bb16,
_ => bb15
}
}
bb14 = {
_5 = !_22;
_6 = (*_24);
_2.1 = (-55_i8) as u64;
_25.1 = core::ptr::addr_of_mut!(_33);
(*_28) = core::ptr::addr_of_mut!(_29.3);
_32.0 = [22148_i16];
_29 = (_26, _23, _19, _33);
_3 = [111429683016504242902268085883313467495_i128,(-29687863881379551117502028601677217960_i128),(-36963837002605460027527254471787224601_i128),(-40600031494761693649905437900580655803_i128),64792930137800488870153520719291527069_i128,137875294482002311218052182538114018139_i128];
_32.0 = [25570_i16];
_25.1 = core::ptr::addr_of_mut!(_33);
RET = _5 | _22;
_29.3 = _33 ^ _33;
_26 = (-11806_i16) as u128;
_20 = _33 ^ _29.3;
_4 = (*_24);
Goto(bb11)
}
bb15 = {
_4 = !_2.0;
_7 = [137639976006361629798318968313410988078_i128,(-139090277965547662969913538652843745196_i128),(-62295769257532649409601079368476038372_i128),(-13394963472391683506769119904523835125_i128),(-141172169417624174948375635868410937095_i128),33129849127283877431266761238055588176_i128];
_14 = _8 ^ _8;
_6 = _2.1 as usize;
_15 = ['\u{10c216}','\u{33f40}','\u{2d4a3}','\u{913e1}','\u{105152}','\u{460c0}','\u{89fa7}','\u{f0f12}'];
_8 = 9093_u16 as isize;
_4 = !_2.0;
_12 = [3152371580_u32,2570449043_u32,484804515_u32,4227249009_u32,2244106906_u32,179215108_u32,2708048046_u32,1070969617_u32];
_14 = !_8;
_14 = 60051_u16 as isize;
RET = (-31997_i16) as u64;
_2.0 = _4;
_13 = _14;
RET = _10;
_5 = !_10;
_6 = _2.0;
_2 = (_4, _10);
Call(_10 = core::intrinsics::transmute(_2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
Goto(bb17)
}
bb17 = {
Call(_44 = dump_var(3_usize, 10_usize, Move(_10), 36_usize, Move(_36), 27_usize, Move(_27), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(3_usize, 33_usize, Move(_33), 12_usize, Move(_12), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(3_usize, 22_usize, Move(_22), 5_usize, Move(_5), 38_usize, Move(_38), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(3_usize, 40_usize, Move(_40), 9_usize, Move(_9), 45_usize, _45, 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u64,mut _2: u64,mut _3: u64,mut _4: u64,mut _5: (usize, u64),mut _6: u64,mut _7: [i128; 6],mut _8: [i128; 6],mut _9: i32) -> *const &'static *mut u8 {
mir! {
type RET = *const &'static *mut u8;
let _10: Adt83;
let _11: [usize; 1];
let _12: (char, [u32; 8]);
let _13: [u8; 5];
let _14: isize;
let _15: i16;
let _16: u128;
let _17: [usize; 7];
let _18: isize;
let _19: i64;
let _20: &'static u8;
let _21: i16;
let _22: &'static (f32, u32, f32, [i16; 5]);
let _23: [char; 8];
let _24: u64;
let _25: *const u16;
let _26: isize;
let _27: *const *mut u8;
let _28: [u64; 3];
let _29: [usize; 7];
let _30: isize;
let _31: isize;
let _32: [char; 8];
let _33: [i32; 3];
let _34: u8;
let _35: ([bool; 8], Adt33);
let _36: isize;
let _37: [u8; 5];
let _38: [u8; 5];
let _39: (u128, char, bool, u8);
let _40: i8;
let _41: i16;
let _42: isize;
let _43: f64;
let _44: &'static *mut u8;
let _45: f64;
let _46: [u8; 5];
let _47: (char, [u32; 8]);
let _48: bool;
let _49: f64;
let _50: Adt21;
let _51: (usize, u64);
let _52: ([i16; 1],);
let _53: ([bool; 8], Adt33);
let _54: ();
let _55: ();
{
_8 = [165298639344171815821361767772376558883_i128,131120073861098855419268799114687424299_i128,(-82927154083125651394343412631486781499_i128),(-57378313700022992301953360517975065072_i128),(-107135129632730268814300234929261677939_i128),(-128190147514872486041498849982762121282_i128)];
_1 = !_3;
_5.0 = 7292_i16 as usize;
_1 = 118_isize as u64;
_12.0 = '\u{3edb1}';
_13 = [25_u8,77_u8,172_u8,32_u8,190_u8];
_5 = (3_usize, _3);
Call(_10 = fn5(_5.0, _2, _5.0, _5.0, _8, _13, _8, _5.0, _7, _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0 * _5.0;
_13 = [102_u8,185_u8,45_u8,178_u8,95_u8];
_12.1 = [2952800849_u32,73452686_u32,4013184169_u32,1326314646_u32,629743973_u32,3511485789_u32,2871478760_u32,2898089434_u32];
_12.1 = Field::<[u32; 8]>(Variant(_10, 2), 5);
_9 = 149324289_i32 & (-1020797059_i32);
_4 = _6 << Field::<usize>(Variant(_10, 2), 6);
_1 = !_4;
_6 = true as u64;
place!(Field::<isize>(Variant(_10, 2), 2)) = 411173534_u32 as isize;
_14 = !Field::<isize>(Variant(_10, 2), 2);
_9 = (-698326010_i32) * 746383164_i32;
_5.1 = 4231650337_u32 as u64;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_9 = -363203776_i32;
_12.0 = '\u{626f9}';
_12.1 = [4225978688_u32,4266611131_u32,1903135252_u32,3607579623_u32,1391723110_u32,3160054185_u32,1438405523_u32,2544431035_u32];
_1 = (-99_i8) as u64;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = [495463776_u32,1666856912_u32,1029108613_u32,839495250_u32,3128201762_u32,2795457083_u32,2606491887_u32,1247421198_u32];
_14 = Field::<isize>(Variant(_10, 2), 2);
place!(Field::<u16>(Variant(_10, 2), 7)) = Field::<u64>(Variant(_10, 2), 3) as u16;
_2 = Field::<u64>(Variant(_10, 2), 3);
_1 = Field::<u64>(Variant(_10, 2), 3);
_5.1 = _3;
_13 = [219_u8,119_u8,254_u8,62_u8,120_u8];
_12.0 = '\u{fe812}';
_15 = !28190_i16;
_6 = !_1;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_12 = ('\u{b141c}', Field::<[u32; 8]>(Variant(_10, 2), 5));
Goto(bb2)
}
bb2 = {
_3 = 2535378745_u32 as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_7 = [(-70907356995827191283930641196029438124_i128),106070498632568177259586936142927510173_i128,145478853003906753461981298681468167455_i128,136528623757920537236821363664859908813_i128,105966708640447961590809233386703952268_i128,83819929240201067808626120788130246016_i128];
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_2,_2,_5.1,_2];
SetDiscriminant(_10, 2);
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<u16>(Variant(_10, 2), 7)) = 52846_u16 - 51757_u16;
_5 = (17903207808223627750_usize, _2);
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [241_u8,78_u8,48_u8,102_u8,147_u8];
_14 = 54_isize + 9223372036854775807_isize;
place!(Field::<u16>(Variant(_10, 2), 7)) = 38865_u16;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = _12.1;
_3 = _2;
_2 = _1 >> _4;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = _13;
place!(Field::<u64>(Variant(_10, 2), 3)) = !_2;
_1 = _5.0 as u64;
_8 = _7;
place!(Field::<u64>(Variant(_10, 2), 3)) = _5.1 << _5.0;
_3 = !Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u64>(Variant(_10, 2), 3)) = _6 + _6;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
place!(Field::<usize>(Variant(_10, 2), 6)) = !_5.0;
_12.1 = [592017428_u32,405998851_u32,754301791_u32,3247359306_u32,2687405030_u32,1063683495_u32,3370441193_u32,2387077799_u32];
_5 = (Field::<usize>(Variant(_10, 2), 6), _1);
Goto(bb3)
}
bb3 = {
_8 = [158113965210544727541445574732119771434_i128,(-153016139286533125166913644660507195564_i128),(-71821879728899168056767085761418081291_i128),(-139056321174269898495504015831230622485_i128),153494071497702763395775766500981384042_i128,123059283665656693475455048362895226556_i128];
_6 = _2;
place!(Field::<u16>(Variant(_10, 2), 7)) = 58036_u16;
place!(Field::<isize>(Variant(_10, 2), 2)) = 77_i8 as isize;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<isize>(Variant(_10, 2), 2)) = _14;
_13 = [184_u8,109_u8,70_u8,8_u8,51_u8];
_17 = [_5.0,Field::<usize>(Variant(_10, 2), 6),Field::<usize>(Variant(_10, 2), 6),_5.0,Field::<usize>(Variant(_10, 2), 6),_5.0,Field::<usize>(Variant(_10, 2), 6)];
_5 = (Field::<usize>(Variant(_10, 2), 6), _3);
_16 = _12.0 as u128;
Goto(bb4)
}
bb4 = {
_6 = _3;
_18 = _14 * _14;
place!(Field::<u16>(Variant(_10, 2), 7)) = !2234_u16;
_9 = -(-1503528308_i32);
_23 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
Goto(bb5)
}
bb5 = {
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_4 = _3 - _1;
_1 = Field::<u64>(Variant(_10, 2), 3) >> _2;
_23 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_24 = _3 & _1;
_19 = (-5391228788683907826_i64);
_21 = 235_u8 as i16;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_21,_15,_21,_15,_21];
_18 = _2 as isize;
_7 = _8;
place!(Field::<isize>(Variant(_10, 2), 2)) = -_18;
_8 = _7;
place!(Field::<isize>(Variant(_10, 2), 2)) = _18;
_8 = _7;
_5 = (Field::<usize>(Variant(_10, 2), 6), Field::<u64>(Variant(_10, 2), 3));
_24 = _19 as u64;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
_1 = _2 - Field::<u64>(Variant(_10, 2), 3);
_18 = 172_u8 as isize;
place!(Field::<isize>(Variant(_10, 2), 2)) = -_14;
_1 = _3;
_24 = true as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_21,_15,_21];
_2 = _4 + _4;
_7 = [109565046502877888797644504143562365578_i128,56996989120740662118593426944976479735_i128,20291986428612148079451108584728202388_i128,(-82228519181222230763698142113006294909_i128),(-38150948222285115533962739793534743813_i128),153921155004165719237371880788304290806_i128];
_7 = _8;
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [_6,_6,Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_5.1,_2];
match _19 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463457983378643084303630 => bb14,
_ => bb13
}
}
bb6 = {
_6 = _3;
_18 = _14 * _14;
place!(Field::<u16>(Variant(_10, 2), 7)) = !2234_u16;
_9 = -(-1503528308_i32);
_23 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
Goto(bb5)
}
bb7 = {
_8 = [158113965210544727541445574732119771434_i128,(-153016139286533125166913644660507195564_i128),(-71821879728899168056767085761418081291_i128),(-139056321174269898495504015831230622485_i128),153494071497702763395775766500981384042_i128,123059283665656693475455048362895226556_i128];
_6 = _2;
place!(Field::<u16>(Variant(_10, 2), 7)) = 58036_u16;
place!(Field::<isize>(Variant(_10, 2), 2)) = 77_i8 as isize;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<isize>(Variant(_10, 2), 2)) = _14;
_13 = [184_u8,109_u8,70_u8,8_u8,51_u8];
_17 = [_5.0,Field::<usize>(Variant(_10, 2), 6),Field::<usize>(Variant(_10, 2), 6),_5.0,Field::<usize>(Variant(_10, 2), 6),_5.0,Field::<usize>(Variant(_10, 2), 6)];
_5 = (Field::<usize>(Variant(_10, 2), 6), _3);
_16 = _12.0 as u128;
Goto(bb4)
}
bb8 = {
_3 = 2535378745_u32 as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_7 = [(-70907356995827191283930641196029438124_i128),106070498632568177259586936142927510173_i128,145478853003906753461981298681468167455_i128,136528623757920537236821363664859908813_i128,105966708640447961590809233386703952268_i128,83819929240201067808626120788130246016_i128];
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_2,_2,_5.1,_2];
SetDiscriminant(_10, 2);
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<u16>(Variant(_10, 2), 7)) = 52846_u16 - 51757_u16;
_5 = (17903207808223627750_usize, _2);
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [241_u8,78_u8,48_u8,102_u8,147_u8];
_14 = 54_isize + 9223372036854775807_isize;
place!(Field::<u16>(Variant(_10, 2), 7)) = 38865_u16;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = _12.1;
_3 = _2;
_2 = _1 >> _4;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = _13;
place!(Field::<u64>(Variant(_10, 2), 3)) = !_2;
_1 = _5.0 as u64;
_8 = _7;
place!(Field::<u64>(Variant(_10, 2), 3)) = _5.1 << _5.0;
_3 = !Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u64>(Variant(_10, 2), 3)) = _6 + _6;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
place!(Field::<usize>(Variant(_10, 2), 6)) = !_5.0;
_12.1 = [592017428_u32,405998851_u32,754301791_u32,3247359306_u32,2687405030_u32,1063683495_u32,3370441193_u32,2387077799_u32];
_5 = (Field::<usize>(Variant(_10, 2), 6), _1);
Goto(bb3)
}
bb9 = {
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0 * _5.0;
_13 = [102_u8,185_u8,45_u8,178_u8,95_u8];
_12.1 = [2952800849_u32,73452686_u32,4013184169_u32,1326314646_u32,629743973_u32,3511485789_u32,2871478760_u32,2898089434_u32];
_12.1 = Field::<[u32; 8]>(Variant(_10, 2), 5);
_9 = 149324289_i32 & (-1020797059_i32);
_4 = _6 << Field::<usize>(Variant(_10, 2), 6);
_1 = !_4;
_6 = true as u64;
place!(Field::<isize>(Variant(_10, 2), 2)) = 411173534_u32 as isize;
_14 = !Field::<isize>(Variant(_10, 2), 2);
_9 = (-698326010_i32) * 746383164_i32;
_5.1 = 4231650337_u32 as u64;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_9 = -363203776_i32;
_12.0 = '\u{626f9}';
_12.1 = [4225978688_u32,4266611131_u32,1903135252_u32,3607579623_u32,1391723110_u32,3160054185_u32,1438405523_u32,2544431035_u32];
_1 = (-99_i8) as u64;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = [495463776_u32,1666856912_u32,1029108613_u32,839495250_u32,3128201762_u32,2795457083_u32,2606491887_u32,1247421198_u32];
_14 = Field::<isize>(Variant(_10, 2), 2);
place!(Field::<u16>(Variant(_10, 2), 7)) = Field::<u64>(Variant(_10, 2), 3) as u16;
_2 = Field::<u64>(Variant(_10, 2), 3);
_1 = Field::<u64>(Variant(_10, 2), 3);
_5.1 = _3;
_13 = [219_u8,119_u8,254_u8,62_u8,120_u8];
_12.0 = '\u{fe812}';
_15 = !28190_i16;
_6 = !_1;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_12 = ('\u{b141c}', Field::<[u32; 8]>(Variant(_10, 2), 5));
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
_23 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_14 = _19 as isize;
_3 = _6 * Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u16>(Variant(_10, 2), 7)) = 9914_u16;
_15 = !_21;
_12.1 = [460724430_u32,160515327_u32,2294328946_u32,3116885505_u32,803345076_u32,2692266170_u32,1199430878_u32,1870528021_u32];
_12 = ('\u{10f967}', Field::<[u32; 8]>(Variant(_10, 2), 5));
_12.1 = Field::<[u32; 8]>(Variant(_10, 2), 5);
_18 = _14;
_25 = core::ptr::addr_of!(place!(Field::<u16>(Variant(_10, 2), 7)));
place!(Field::<usize>(Variant(_10, 2), 6)) = 175_u8 as usize;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0 * _5.0;
_16 = 152711683387726425808524621355107372271_u128;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [58_u8,134_u8,190_u8,185_u8,79_u8];
_8 = [(-162843483943460166672549621121067991727_i128),(-31926687278538186144304107197094142991_i128),40809259866168051890302963980126466564_i128,(-63515068346318181770616159154181705133_i128),(-94235591965554648682987225016468852168_i128),(-1580336257186530242206898982775468310_i128)];
_15 = -_21;
_7 = _8;
_9 = (-1599327939_i32);
_5.1 = _9 as u64;
(*_25) = _12.0 as u16;
place!(Field::<isize>(Variant(_10, 2), 2)) = _14;
_12 = ('\u{7d912}', Field::<[u32; 8]>(Variant(_10, 2), 5));
_18 = false as isize;
SetDiscriminant(_10, 3);
_6 = 141019966329697375238095980701451799901_i128 as u64;
_26 = _18;
place!(Field::<[char; 8]>(Variant(_10, 3), 4)) = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
match _9 {
0 => bb11,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
340282366920938463463374607430168883517 => bb16,
_ => bb15
}
}
bb15 = {
_3 = 2535378745_u32 as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_7 = [(-70907356995827191283930641196029438124_i128),106070498632568177259586936142927510173_i128,145478853003906753461981298681468167455_i128,136528623757920537236821363664859908813_i128,105966708640447961590809233386703952268_i128,83819929240201067808626120788130246016_i128];
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_2,_2,_5.1,_2];
SetDiscriminant(_10, 2);
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<u16>(Variant(_10, 2), 7)) = 52846_u16 - 51757_u16;
_5 = (17903207808223627750_usize, _2);
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [241_u8,78_u8,48_u8,102_u8,147_u8];
_14 = 54_isize + 9223372036854775807_isize;
place!(Field::<u16>(Variant(_10, 2), 7)) = 38865_u16;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = _12.1;
_3 = _2;
_2 = _1 >> _4;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = _13;
place!(Field::<u64>(Variant(_10, 2), 3)) = !_2;
_1 = _5.0 as u64;
_8 = _7;
place!(Field::<u64>(Variant(_10, 2), 3)) = _5.1 << _5.0;
_3 = !Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u64>(Variant(_10, 2), 3)) = _6 + _6;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
place!(Field::<usize>(Variant(_10, 2), 6)) = !_5.0;
_12.1 = [592017428_u32,405998851_u32,754301791_u32,3247359306_u32,2687405030_u32,1063683495_u32,3370441193_u32,2387077799_u32];
_5 = (Field::<usize>(Variant(_10, 2), 6), _1);
Goto(bb3)
}
bb16 = {
_9 = (-1430577886_i32) | (-886275732_i32);
_24 = _3 ^ _2;
_19 = 8666783326648580617_i64 | 5225985610560255952_i64;
place!(Field::<u64>(Variant(_10, 3), 2)) = _5.0 as u64;
_28 = [_4,_24,_2];
place!(Field::<[u64; 3]>(Variant(_10, 3), 5)) = [_4,Field::<u64>(Variant(_10, 3), 2),_1];
_8 = _7;
_34 = !38_u8;
_7 = _8;
_32 = Field::<[char; 8]>(Variant(_10, 3), 4);
match _16 {
0 => bb17,
152711683387726425808524621355107372271 => bb19,
_ => bb18
}
}
bb17 = {
_3 = 2535378745_u32 as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_7 = [(-70907356995827191283930641196029438124_i128),106070498632568177259586936142927510173_i128,145478853003906753461981298681468167455_i128,136528623757920537236821363664859908813_i128,105966708640447961590809233386703952268_i128,83819929240201067808626120788130246016_i128];
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_2,_2,_5.1,_2];
SetDiscriminant(_10, 2);
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<u16>(Variant(_10, 2), 7)) = 52846_u16 - 51757_u16;
_5 = (17903207808223627750_usize, _2);
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [241_u8,78_u8,48_u8,102_u8,147_u8];
_14 = 54_isize + 9223372036854775807_isize;
place!(Field::<u16>(Variant(_10, 2), 7)) = 38865_u16;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = _12.1;
_3 = _2;
_2 = _1 >> _4;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = _13;
place!(Field::<u64>(Variant(_10, 2), 3)) = !_2;
_1 = _5.0 as u64;
_8 = _7;
place!(Field::<u64>(Variant(_10, 2), 3)) = _5.1 << _5.0;
_3 = !Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u64>(Variant(_10, 2), 3)) = _6 + _6;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
place!(Field::<usize>(Variant(_10, 2), 6)) = !_5.0;
_12.1 = [592017428_u32,405998851_u32,754301791_u32,3247359306_u32,2687405030_u32,1063683495_u32,3370441193_u32,2387077799_u32];
_5 = (Field::<usize>(Variant(_10, 2), 6), _1);
Goto(bb3)
}
bb18 = {
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0 * _5.0;
_13 = [102_u8,185_u8,45_u8,178_u8,95_u8];
_12.1 = [2952800849_u32,73452686_u32,4013184169_u32,1326314646_u32,629743973_u32,3511485789_u32,2871478760_u32,2898089434_u32];
_12.1 = Field::<[u32; 8]>(Variant(_10, 2), 5);
_9 = 149324289_i32 & (-1020797059_i32);
_4 = _6 << Field::<usize>(Variant(_10, 2), 6);
_1 = !_4;
_6 = true as u64;
place!(Field::<isize>(Variant(_10, 2), 2)) = 411173534_u32 as isize;
_14 = !Field::<isize>(Variant(_10, 2), 2);
_9 = (-698326010_i32) * 746383164_i32;
_5.1 = 4231650337_u32 as u64;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_9 = -363203776_i32;
_12.0 = '\u{626f9}';
_12.1 = [4225978688_u32,4266611131_u32,1903135252_u32,3607579623_u32,1391723110_u32,3160054185_u32,1438405523_u32,2544431035_u32];
_1 = (-99_i8) as u64;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = [495463776_u32,1666856912_u32,1029108613_u32,839495250_u32,3128201762_u32,2795457083_u32,2606491887_u32,1247421198_u32];
_14 = Field::<isize>(Variant(_10, 2), 2);
place!(Field::<u16>(Variant(_10, 2), 7)) = Field::<u64>(Variant(_10, 2), 3) as u16;
_2 = Field::<u64>(Variant(_10, 2), 3);
_1 = Field::<u64>(Variant(_10, 2), 3);
_5.1 = _3;
_13 = [219_u8,119_u8,254_u8,62_u8,120_u8];
_12.0 = '\u{fe812}';
_15 = !28190_i16;
_6 = !_1;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_12 = ('\u{b141c}', Field::<[u32; 8]>(Variant(_10, 2), 5));
Goto(bb2)
}
bb19 = {
_14 = _18 | _18;
place!(Field::<(char, [u32; 8])>(Variant(_10, 3), 3)) = (_12.0, _12.1);
_5 = (7868714195279935785_usize, _3);
_17 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_30 = _18 | _18;
_5.0 = _19 as usize;
_20 = &_34;
_36 = _30;
_29 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_32 = _23;
_7 = [91561010563604433911339015543786740185_i128,(-32204013645885437761094183319482505677_i128),155160031034341941343394788654818153589_i128,146164631249741669053807917936944844340_i128,127075071445334935914678065826481766514_i128,111368656527146455407104392621194385401_i128];
_14 = _18 + _26;
match _16 {
0 => bb16,
1 => bb20,
2 => bb21,
3 => bb22,
152711683387726425808524621355107372271 => bb24,
_ => bb23
}
}
bb20 = {
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0 * _5.0;
_13 = [102_u8,185_u8,45_u8,178_u8,95_u8];
_12.1 = [2952800849_u32,73452686_u32,4013184169_u32,1326314646_u32,629743973_u32,3511485789_u32,2871478760_u32,2898089434_u32];
_12.1 = Field::<[u32; 8]>(Variant(_10, 2), 5);
_9 = 149324289_i32 & (-1020797059_i32);
_4 = _6 << Field::<usize>(Variant(_10, 2), 6);
_1 = !_4;
_6 = true as u64;
place!(Field::<isize>(Variant(_10, 2), 2)) = 411173534_u32 as isize;
_14 = !Field::<isize>(Variant(_10, 2), 2);
_9 = (-698326010_i32) * 746383164_i32;
_5.1 = 4231650337_u32 as u64;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_9 = -363203776_i32;
_12.0 = '\u{626f9}';
_12.1 = [4225978688_u32,4266611131_u32,1903135252_u32,3607579623_u32,1391723110_u32,3160054185_u32,1438405523_u32,2544431035_u32];
_1 = (-99_i8) as u64;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = [495463776_u32,1666856912_u32,1029108613_u32,839495250_u32,3128201762_u32,2795457083_u32,2606491887_u32,1247421198_u32];
_14 = Field::<isize>(Variant(_10, 2), 2);
place!(Field::<u16>(Variant(_10, 2), 7)) = Field::<u64>(Variant(_10, 2), 3) as u16;
_2 = Field::<u64>(Variant(_10, 2), 3);
_1 = Field::<u64>(Variant(_10, 2), 3);
_5.1 = _3;
_13 = [219_u8,119_u8,254_u8,62_u8,120_u8];
_12.0 = '\u{fe812}';
_15 = !28190_i16;
_6 = !_1;
_11 = [Field::<usize>(Variant(_10, 2), 6)];
_12 = ('\u{b141c}', Field::<[u32; 8]>(Variant(_10, 2), 5));
Goto(bb2)
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
_3 = 2535378745_u32 as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_7 = [(-70907356995827191283930641196029438124_i128),106070498632568177259586936142927510173_i128,145478853003906753461981298681468167455_i128,136528623757920537236821363664859908813_i128,105966708640447961590809233386703952268_i128,83819929240201067808626120788130246016_i128];
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_2,_2,_5.1,_2];
SetDiscriminant(_10, 2);
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<u16>(Variant(_10, 2), 7)) = 52846_u16 - 51757_u16;
_5 = (17903207808223627750_usize, _2);
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [241_u8,78_u8,48_u8,102_u8,147_u8];
_14 = 54_isize + 9223372036854775807_isize;
place!(Field::<u16>(Variant(_10, 2), 7)) = 38865_u16;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = _12.1;
_3 = _2;
_2 = _1 >> _4;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = _13;
place!(Field::<u64>(Variant(_10, 2), 3)) = !_2;
_1 = _5.0 as u64;
_8 = _7;
place!(Field::<u64>(Variant(_10, 2), 3)) = _5.1 << _5.0;
_3 = !Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u64>(Variant(_10, 2), 3)) = _6 + _6;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
place!(Field::<usize>(Variant(_10, 2), 6)) = !_5.0;
_12.1 = [592017428_u32,405998851_u32,754301791_u32,3247359306_u32,2687405030_u32,1063683495_u32,3370441193_u32,2387077799_u32];
_5 = (Field::<usize>(Variant(_10, 2), 6), _1);
Goto(bb3)
}
bb24 = {
_30 = !_14;
_26 = -_36;
_21 = _15;
_12.0 = Field::<(char, [u32; 8])>(Variant(_10, 3), 3).0;
_35.1.fld2 = _19 as f64;
_35.1.fld3 = [149966764777932632932707894651422936726_i128,(-126972471436588018279839218743050969403_i128),(-136839666525981730971935203622468097870_i128),(-120485937591941094260677331439736282762_i128),(-169502479727057888421590283311421956148_i128),408696178910184619017928661517979054_i128];
_39.0 = !_16;
_31 = -_30;
place!(Field::<u128>(Variant(_10, 3), 6)) = _16;
_12.0 = Field::<(char, [u32; 8])>(Variant(_10, 3), 3).0;
match _16 {
0 => bb15,
152711683387726425808524621355107372271 => bb26,
_ => bb25
}
}
bb25 = {
_3 = 2535378745_u32 as u64;
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
_7 = [(-70907356995827191283930641196029438124_i128),106070498632568177259586936142927510173_i128,145478853003906753461981298681468167455_i128,136528623757920537236821363664859908813_i128,105966708640447961590809233386703952268_i128,83819929240201067808626120788130246016_i128];
place!(Field::<[u64; 7]>(Variant(_10, 2), 4)) = [Field::<u64>(Variant(_10, 2), 3),_2,Field::<u64>(Variant(_10, 2), 3),_2,_2,_5.1,_2];
SetDiscriminant(_10, 2);
place!(Field::<[i16; 5]>(Variant(_10, 2), 0)) = [_15,_15,_15,_15,_15];
place!(Field::<u16>(Variant(_10, 2), 7)) = 52846_u16 - 51757_u16;
_5 = (17903207808223627750_usize, _2);
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [241_u8,78_u8,48_u8,102_u8,147_u8];
_14 = 54_isize + 9223372036854775807_isize;
place!(Field::<u16>(Variant(_10, 2), 7)) = 38865_u16;
place!(Field::<[u32; 8]>(Variant(_10, 2), 5)) = _12.1;
_3 = _2;
_2 = _1 >> _4;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = _13;
place!(Field::<u64>(Variant(_10, 2), 3)) = !_2;
_1 = _5.0 as u64;
_8 = _7;
place!(Field::<u64>(Variant(_10, 2), 3)) = _5.1 << _5.0;
_3 = !Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u64>(Variant(_10, 2), 3)) = _6 + _6;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0;
place!(Field::<usize>(Variant(_10, 2), 6)) = !_5.0;
_12.1 = [592017428_u32,405998851_u32,754301791_u32,3247359306_u32,2687405030_u32,1063683495_u32,3370441193_u32,2387077799_u32];
_5 = (Field::<usize>(Variant(_10, 2), 6), _1);
Goto(bb3)
}
bb26 = {
_19 = false as i64;
_7 = [(-39950805215908651778682113486519392147_i128),102341588281891257146761105708222446273_i128,(-132455849236460158120040458481183320961_i128),110424137067641873703791308189816338301_i128,(-25158846570304118700694365764510852388_i128),97588513602630269026960015371827348700_i128];
_23 = _32;
_39.3 = (*_20);
_24 = _4 ^ _2;
_35.1.fld4 = !40449_u16;
place!(Field::<[u64; 3]>(Variant(_10, 3), 5)) = _28;
_17 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
place!(Field::<[u64; 3]>(Variant(_10, 3), 5)) = [Field::<u64>(Variant(_10, 3), 2),_5.1,_5.1];
_3 = _4 | _4;
_12.1 = [486370184_u32,2514264859_u32,2391485045_u32,814470821_u32,1694964963_u32,1505600772_u32,3455884464_u32,128490380_u32];
_20 = &_39.3;
_39.0 = !Field::<u128>(Variant(_10, 3), 6);
place!(Field::<u64>(Variant(_10, 3), 2)) = true as u64;
_9 = 1050162779_i32 - 276228480_i32;
_37 = [_34,_34,_39.3,(*_20),(*_20)];
_31 = _30 | _36;
Goto(bb27)
}
bb27 = {
_39.2 = false ^ true;
place!(Field::<[char; 8]>(Variant(_10, 3), 4)) = [_12.0,_12.0,Field::<(char, [u32; 8])>(Variant(_10, 3), 3).0,_12.0,Field::<(char, [u32; 8])>(Variant(_10, 3), 3).0,_12.0,_12.0,_12.0];
_35.1.fld2 = _16 as f64;
_35.1.fld0 = _39.2;
_8 = [124710045021299534968149706290985812680_i128,(-42315572333248736222776492841374591064_i128),(-135848693588715249676190189488127037188_i128),124122088967859852515360760697855026469_i128,(-96226444969058496877959647142129242787_i128),52387359044482683862454087647694818214_i128];
_38 = [_34,(*_20),(*_20),_34,_39.3];
_2 = _9 as u64;
_39.1 = _12.0;
_40 = 87_i8;
_32 = [Field::<(char, [u32; 8])>(Variant(_10, 3), 3).0,_39.1,_12.0,_12.0,_39.1,Field::<(char, [u32; 8])>(Variant(_10, 3), 3).0,_12.0,_39.1];
place!(Field::<[i16; 1]>(Variant(_10, 3), 1)) = [_15];
_29 = _17;
_35.1.fld3 = [(-80384740957545446885216283898492559775_i128),(-167418690610423310060076305995715502302_i128),58039277713212940032076346486416026861_i128,(-26915514419793598584677110429954808665_i128),(-41194780419963715535562882730972762569_i128),13697723764444628127352956772619481355_i128];
_33 = [_9,_9,_9];
_42 = _31 + _31;
_35.1.fld3 = _8;
_37 = [(*_20),(*_20),_39.3,_39.3,(*_20)];
place!(Field::<(char, [u32; 8])>(Variant(_10, 3), 3)).1 = [312511008_u32,2369809677_u32,3740192934_u32,3216047888_u32,585504353_u32,2946896931_u32,1007711980_u32,1901817260_u32];
_31 = _30 << _5.0;
place!(Field::<[u64; 3]>(Variant(_10, 3), 5)) = _28;
match _16 {
0 => bb1,
1 => bb15,
2 => bb13,
3 => bb28,
152711683387726425808524621355107372271 => bb30,
_ => bb29
}
}
bb28 = {
_23 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_14 = _19 as isize;
_3 = _6 * Field::<u64>(Variant(_10, 2), 3);
place!(Field::<u16>(Variant(_10, 2), 7)) = 9914_u16;
_15 = !_21;
_12.1 = [460724430_u32,160515327_u32,2294328946_u32,3116885505_u32,803345076_u32,2692266170_u32,1199430878_u32,1870528021_u32];
_12 = ('\u{10f967}', Field::<[u32; 8]>(Variant(_10, 2), 5));
_12.1 = Field::<[u32; 8]>(Variant(_10, 2), 5);
_18 = _14;
_25 = core::ptr::addr_of!(place!(Field::<u16>(Variant(_10, 2), 7)));
place!(Field::<usize>(Variant(_10, 2), 6)) = 175_u8 as usize;
place!(Field::<usize>(Variant(_10, 2), 6)) = _5.0 * _5.0;
_16 = 152711683387726425808524621355107372271_u128;
place!(Field::<[u8; 5]>(Variant(_10, 2), 1)) = [58_u8,134_u8,190_u8,185_u8,79_u8];
_8 = [(-162843483943460166672549621121067991727_i128),(-31926687278538186144304107197094142991_i128),40809259866168051890302963980126466564_i128,(-63515068346318181770616159154181705133_i128),(-94235591965554648682987225016468852168_i128),(-1580336257186530242206898982775468310_i128)];
_15 = -_21;
_7 = _8;
_9 = (-1599327939_i32);
_5.1 = _9 as u64;
(*_25) = _12.0 as u16;
place!(Field::<isize>(Variant(_10, 2), 2)) = _14;
_12 = ('\u{7d912}', Field::<[u32; 8]>(Variant(_10, 2), 5));
_18 = false as isize;
SetDiscriminant(_10, 3);
_6 = 141019966329697375238095980701451799901_i128 as u64;
_26 = _18;
place!(Field::<[char; 8]>(Variant(_10, 3), 4)) = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
match _9 {
0 => bb11,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
340282366920938463463374607430168883517 => bb16,
_ => bb15
}
}
bb29 = {
_6 = _3;
_18 = _14 * _14;
place!(Field::<u16>(Variant(_10, 2), 7)) = !2234_u16;
_9 = -(-1503528308_i32);
_23 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
Goto(bb5)
}
bb30 = {
_19 = (-4801487812274511940_i64) & 6689775992706512077_i64;
RET = core::ptr::addr_of!(_44);
_39.3 = !_34;
_39.2 = _35.1.fld0;
_49 = _35.1.fld2 - _35.1.fld2;
_35.1.fld2 = _49;
_35.0 = [_39.2,_35.1.fld0,_39.2,_39.2,_35.1.fld0,_39.2,_39.2,_35.1.fld0];
_35.1.fld5 = [_24,_1,_5.1,_3,_1,_1,_3];
_14 = _26 | _30;
_43 = -_49;
_4 = !_3;
_24 = !_5.1;
place!(Field::<[u64; 3]>(Variant(_10, 3), 5)) = [_24,_24,_1];
_40 = 2525494869_u32 as i8;
_47 = (_39.1, Field::<(char, [u32; 8])>(Variant(_10, 3), 3).1);
place!(Field::<u128>(Variant(_10, 3), 6)) = !_39.0;
_28 = [_24,_4,_24];
_29 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
Goto(bb31)
}
bb31 = {
Call(_54 = dump_var(4_usize, 29_usize, Move(_29), 2_usize, Move(_2), 3_usize, Move(_3), 15_usize, Move(_15)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_54 = dump_var(4_usize, 40_usize, Move(_40), 37_usize, Move(_37), 30_usize, Move(_30), 26_usize, Move(_26)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_54 = dump_var(4_usize, 16_usize, Move(_16), 13_usize, Move(_13), 11_usize, Move(_11), 38_usize, Move(_38)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_54 = dump_var(4_usize, 31_usize, Move(_31), 36_usize, Move(_36), 28_usize, Move(_28), 47_usize, Move(_47)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Call(_54 = dump_var(4_usize, 42_usize, Move(_42), 39_usize, Move(_39), 55_usize, _55, 55_usize, _55), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: usize,mut _2: u64,mut _3: usize,mut _4: usize,mut _5: [i128; 6],mut _6: [u8; 5],mut _7: [i128; 6],mut _8: usize,mut _9: [i128; 6],mut _10: (usize, u64),mut _11: (usize, u64)) -> Adt83 {
mir! {
type RET = Adt83;
let _12: *const u16;
let _13: u128;
let _14: u128;
let _15: i128;
let _16: (*const u8, [i16; 1]);
let _17: &'static [u8; 5];
let _18: i64;
let _19: Adt83;
let _20: f64;
let _21: f32;
let _22: f32;
let _23: (*const u8, [i16; 1]);
let _24: (f32, u32, f32, [i16; 5]);
let _25: *mut u8;
let _26: &'static *const (u128, char, bool, u8);
let _27: &'static i128;
let _28: Adt33;
let _29: f32;
let _30: bool;
let _31: f32;
let _32: u64;
let _33: isize;
let _34: f32;
let _35: isize;
let _36: [i16; 5];
let _37: (usize, u64);
let _38: u8;
let _39: (f32, u32, f32, [i16; 5]);
let _40: *const &'static *mut u8;
let _41: *mut &'static u128;
let _42: i64;
let _43: bool;
let _44: i128;
let _45: u32;
let _46: &'static ([i16; 1],);
let _47: bool;
let _48: isize;
let _49: i32;
let _50: &'static Adt29;
let _51: [usize; 7];
let _52: *mut (u128, char, bool, u8);
let _53: &'static [i16; 1];
let _54: bool;
let _55: f64;
let _56: [i128; 6];
let _57: [usize; 1];
let _58: *const u8;
let _59: *mut &'static u128;
let _60: isize;
let _61: usize;
let _62: &'static u128;
let _63: isize;
let _64: char;
let _65: u16;
let _66: &'static Adt40;
let _67: bool;
let _68: ([bool; 8], Adt33);
let _69: u128;
let _70: u8;
let _71: *const *mut u8;
let _72: isize;
let _73: (char, [u32; 8]);
let _74: isize;
let _75: isize;
let _76: isize;
let _77: f32;
let _78: Adt29;
let _79: isize;
let _80: isize;
let _81: f32;
let _82: &'static Adt40;
let _83: &'static [u8; 5];
let _84: isize;
let _85: [i16; 5];
let _86: isize;
let _87: i128;
let _88: u16;
let _89: i16;
let _90: u8;
let _91: i64;
let _92: (*const u8, [i16; 1]);
let _93: [usize; 7];
let _94: ();
let _95: ();
{
_11.0 = _4 / _3;
_1 = _10.0;
_9[_3] = _10.0 as i128;
_10 = _11;
_10.0 = _4 % _1;
_1 = !_3;
_3 = 1931447188_u32 as usize;
_13 = 113908433526190784532031941078290824885_u128;
_8 = !_11.0;
_5 = [_9[_4],_7[_4],_7[_4],_7[_4],_9[_4],_9[_4]];
_2 = !_10.1;
Call(_11 = fn6(_4, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _10.1;
_1 = _8;
_10.0 = _8 % _4;
_5 = [145806104838125746248301206892401051503_i128,68590850088924064294775272469620817349_i128,(-43959873980782442045039592975028533007_i128),128167140642399118851179400830244626558_i128,(-127116630709728791966588401574366227786_i128),(-69467786888637298148620332102742266856_i128)];
_13 = 152197333140124041572295131023397973970_u128;
_10 = (_8, _11.1);
_10 = (_1, _2);
_6 = [19_u8,11_u8,20_u8,172_u8,192_u8];
_5 = _9;
_2 = 140_u8 as u64;
_11.1 = _10.1;
_15 = (-11960488303832823872591596974955755819_i128) * 122907881848689301351188618007339422355_i128;
_10.1 = 13820_i16 as u64;
_6 = [242_u8,128_u8,196_u8,146_u8,147_u8];
_15 = !(-8394825889472382834843576998646542025_i128);
_5 = [_15,_15,_15,_15,_15,_15];
_17 = &_6;
_16.1 = [10454_i16];
_21 = 11464_i16 as f32;
Goto(bb2)
}
bb2 = {
_7 = [_15,_15,_15,_15,_15,_15];
_21 = 344185430_i32 as f32;
_22 = _21;
_23.1 = [29553_i16];
Goto(bb3)
}
bb3 = {
_20 = 2425449671_u32 as f64;
_13 = 28715734608465491924837765624136486001_u128 << _8;
_24.3 = [14769_i16,8016_i16,(-13313_i16),(-1546_i16),(-28973_i16)];
_23.1 = [17632_i16];
_22 = _21 - _21;
_10.0 = _22 as usize;
_1 = !_8;
_23.1 = _16.1;
_7 = _9;
_27 = &_15;
_24.1 = !772679808_u32;
_15 = '\u{5dc8b}' as i128;
_24.2 = -_22;
_24.0 = _22 - _22;
_30 = !false;
_9 = _7;
_9 = _7;
_27 = &_15;
_15 = (-65047614594928359455833859378287163194_i128) << _11.1;
_14 = !_13;
_28.fld3 = [_15,_15,_15,_15,_15,_15];
_29 = -_24.0;
_28.fld5 = [_2,_11.1,_11.1,_10.1,_11.1,_10.1,_11.1];
_1 = _8 | _8;
_1 = _11.0;
match _4 {
0 => bb4,
1 => bb5,
2 => bb6,
4 => bb8,
5 => bb9,
6 => bb10,
3 => bb12,
_ => bb11
}
}
bb4 = {
_7 = [_15,_15,_15,_15,_15,_15];
_21 = 344185430_i32 as f32;
_22 = _21;
_23.1 = [29553_i16];
Goto(bb3)
}
bb5 = {
_2 = _10.1;
_1 = _8;
_10.0 = _8 % _4;
_5 = [145806104838125746248301206892401051503_i128,68590850088924064294775272469620817349_i128,(-43959873980782442045039592975028533007_i128),128167140642399118851179400830244626558_i128,(-127116630709728791966588401574366227786_i128),(-69467786888637298148620332102742266856_i128)];
_13 = 152197333140124041572295131023397973970_u128;
_10 = (_8, _11.1);
_10 = (_1, _2);
_6 = [19_u8,11_u8,20_u8,172_u8,192_u8];
_5 = _9;
_2 = 140_u8 as u64;
_11.1 = _10.1;
_15 = (-11960488303832823872591596974955755819_i128) * 122907881848689301351188618007339422355_i128;
_10.1 = 13820_i16 as u64;
_6 = [242_u8,128_u8,196_u8,146_u8,147_u8];
_15 = !(-8394825889472382834843576998646542025_i128);
_5 = [_15,_15,_15,_15,_15,_15];
_17 = &_6;
_16.1 = [10454_i16];
_21 = 11464_i16 as f32;
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
_28.fld2 = _15 as f64;
_12 = core::ptr::addr_of!(_28.fld4);
_10.1 = 18575_i16 as u64;
_24.1 = _13 as u32;
_3 = _15 as usize;
Call(_12 = fn9(Move(_17)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_31 = _29 - _24.2;
_28.fld0 = _30 ^ _30;
_24.1 = '\u{4868b}' as u32;
_32 = _11.1;
_30 = _28.fld0;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_20 = _28.fld2;
_28.fld4 = 5494_u16 - 54738_u16;
_13 = _14 | _14;
_17 = &_6;
_23.1 = _16.1;
_28.fld2 = 1958052403_i32 as f64;
_11 = (_4, _32);
_22 = _31 - _29;
_5 = _9;
match _11.0 {
0 => bb6,
1 => bb5,
3 => bb14,
_ => bb11
}
}
bb14 = {
_14 = (-110_i8) as u128;
_29 = _24.2;
_10 = _11;
_11.0 = 88_i8 as usize;
_35 = !9223372036854775807_isize;
_11.0 = _8 + _4;
_32 = _13 as u64;
match _4 {
0 => bb7,
1 => bb13,
2 => bb4,
3 => bb17,
_ => bb16
}
}
bb15 = {
_31 = _29 - _24.2;
_28.fld0 = _30 ^ _30;
_24.1 = '\u{4868b}' as u32;
_32 = _11.1;
_30 = _28.fld0;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_20 = _28.fld2;
_28.fld4 = 5494_u16 - 54738_u16;
_13 = _14 | _14;
_17 = &_6;
_23.1 = _16.1;
_28.fld2 = 1958052403_i32 as f64;
_11 = (_4, _32);
_22 = _31 - _29;
_5 = _9;
match _11.0 {
0 => bb6,
1 => bb5,
3 => bb14,
_ => bb11
}
}
bb16 = {
_7 = [_15,_15,_15,_15,_15,_15];
_21 = 344185430_i32 as f32;
_22 = _21;
_23.1 = [29553_i16];
Goto(bb3)
}
bb17 = {
_27 = &_15;
_16.1 = [(-13005_i16)];
_34 = -_22;
_12 = core::ptr::addr_of!(_28.fld4);
_11 = _10;
_28.fld4 = 58070_u16;
_28.fld4 = _28.fld0 as u16;
_29 = -_22;
Goto(bb18)
}
bb18 = {
_28.fld4 = 1663324038_i32 as u16;
_3 = _28.fld4 as usize;
(*_12) = 49501_u16;
_33 = _30 as isize;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_11 = _10;
_28.fld5 = [_32,_11.1,_10.1,_32,_32,_32,_10.1];
match _4 {
0 => bb16,
1 => bb10,
2 => bb3,
4 => bb14,
5 => bb6,
3 => bb19,
_ => bb7
}
}
bb19 = {
_21 = -_34;
_37.1 = _32;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_27 = &(*_27);
_25 = core::ptr::addr_of_mut!(_38);
_24.0 = _21 * _21;
_37.1 = !_32;
_37.0 = _4;
_24.3 = [25099_i16,(-23581_i16),(-21399_i16),3668_i16,10260_i16];
_24.0 = _29 * _34;
_25 = core::ptr::addr_of_mut!(_38);
_10.0 = _15 as usize;
_5 = _28.fld3;
_18 = (-6671665489617069960_i64) - 3961172685837468053_i64;
_10.0 = (*_12) as usize;
_23.0 = core::ptr::addr_of!(_38);
_39.3 = _24.3;
_27 = &_15;
(*_12) = !18166_u16;
_16.0 = core::ptr::addr_of!((*_25));
_36 = [3065_i16,7095_i16,8275_i16,(-26596_i16),29799_i16];
match _4 {
3 => bb20,
_ => bb13
}
}
bb20 = {
_34 = _29;
_11.0 = !_1;
_39.2 = _24.0 - _24.0;
_20 = _22 as f64;
_29 = _34;
match _4 {
0 => bb1,
1 => bb9,
2 => bb16,
4 => bb14,
3 => bb21,
_ => bb15
}
}
bb21 = {
_39.3 = _24.3;
_39.0 = _24.1 as f32;
_31 = -_34;
_21 = _39.2;
_2 = _37.1;
_7 = [(*_27),_15,(*_27),(*_27),_15,_15];
(*_12) = 2171_u16 & 40496_u16;
_43 = !_30;
_24.3 = [(-22341_i16),(-19850_i16),11938_i16,(-27572_i16),7509_i16];
_28.fld1 = core::ptr::addr_of_mut!(_25);
_37 = _11;
_5 = [(*_27),(*_27),(*_27),_15,_15,(*_27)];
_13 = _14;
_42 = _18 << _32;
_30 = _43;
_21 = _22;
_35 = _33;
match _4 {
0 => bb7,
3 => bb22,
_ => bb4
}
}
bb22 = {
_44 = !(*_27);
_43 = _28.fld0;
_28.fld3 = _7;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_30 = !_43;
_13 = !_14;
_24.0 = _39.2;
_4 = !_8;
_24.1 = 2694755649_u32;
_21 = -_31;
_39.1 = _24.1 + _24.1;
_39.3 = [30325_i16,29779_i16,17015_i16,20663_i16,(-23312_i16)];
(*_25) = _35 as u8;
_24.3 = [13900_i16,27948_i16,(-17868_i16),(-25399_i16),760_i16];
_39.2 = -_21;
_39.0 = _39.2 * _22;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_13 = _14 + _14;
_49 = -1425857074_i32;
_28.fld1 = core::ptr::addr_of_mut!(_25);
_18 = _15 as i64;
_12 = core::ptr::addr_of!(_28.fld4);
Goto(bb23)
}
bb23 = {
_11 = (_8, _37.1);
_47 = !_28.fld0;
_1 = _4 - _11.0;
_11.1 = _2 + _2;
_28.fld4 = 26980_u16 + 10808_u16;
(*_12) = 14450_u16;
_37 = (_1, _11.1);
_1 = _33 as usize;
_24 = _39;
Goto(bb24)
}
bb24 = {
_47 = _28.fld0;
_49 = _33 as i32;
_28.fld4 = 18185_u16;
(*_12) = 30593_u16;
_39 = (_24.2, _24.1, _24.0, _36);
match _28.fld4 {
0 => bb25,
30593 => bb27,
_ => bb26
}
}
bb25 = {
Return()
}
bb26 = {
_20 = 2425449671_u32 as f64;
_13 = 28715734608465491924837765624136486001_u128 << _8;
_24.3 = [14769_i16,8016_i16,(-13313_i16),(-1546_i16),(-28973_i16)];
_23.1 = [17632_i16];
_22 = _21 - _21;
_10.0 = _22 as usize;
_1 = !_8;
_23.1 = _16.1;
_7 = _9;
_27 = &_15;
_24.1 = !772679808_u32;
_15 = '\u{5dc8b}' as i128;
_24.2 = -_22;
_24.0 = _22 - _22;
_30 = !false;
_9 = _7;
_9 = _7;
_27 = &_15;
_15 = (-65047614594928359455833859378287163194_i128) << _11.1;
_14 = !_13;
_28.fld3 = [_15,_15,_15,_15,_15,_15];
_29 = -_24.0;
_28.fld5 = [_2,_11.1,_11.1,_10.1,_11.1,_10.1,_11.1];
_1 = _8 | _8;
_1 = _11.0;
match _4 {
0 => bb4,
1 => bb5,
2 => bb6,
4 => bb8,
5 => bb9,
6 => bb10,
3 => bb12,
_ => bb11
}
}
bb27 = {
_4 = _37.0;
_28.fld5 = [_11.1,_32,_32,_32,_11.1,_11.1,_10.1];
_25 = core::ptr::addr_of_mut!((*_25));
_29 = _22;
_48 = _33;
_39.0 = -_39.2;
_45 = !_24.1;
_10 = _37;
_9 = [(*_27),_15,(*_27),(*_27),(*_27),(*_27)];
_47 = _30;
_43 = _30;
_16 = (Move(_23.0), _23.1);
_28.fld4 = (-18496_i16) as u16;
_39.0 = _13 as f32;
_30 = !_47;
_24.0 = _49 as f32;
Goto(bb28)
}
bb28 = {
_39 = (_24.2, _45, _34, _36);
_10.0 = !_8;
_7 = [_15,_15,_44,_44,(*_27),_44];
_23 = (Move(_16.0), _16.1);
_36 = _39.3;
_43 = !_47;
_10 = _11;
_7 = [_15,(*_27),(*_27),(*_27),(*_27),_44];
_36 = [(-12622_i16),24605_i16,(-28608_i16),(-11052_i16),26683_i16];
_36 = _39.3;
_60 = _35;
_61 = !_37.0;
_54 = !_30;
_28.fld5 = [_32,_32,_10.1,_10.1,_11.1,_11.1,_37.1];
_58 = Move(_23.0);
_20 = _28.fld2 + _28.fld2;
_28.fld0 = _47;
_28.fld0 = !_54;
_10.0 = _61 ^ _61;
_16.0 = Move(_58);
_21 = _61 as f32;
_31 = _49 as f32;
Goto(bb29)
}
bb29 = {
_8 = _30 as usize;
_59 = core::ptr::addr_of_mut!(_62);
_7 = [(*_27),(*_27),_15,_44,_44,_44];
Goto(bb30)
}
bb30 = {
_39.3 = _36;
_39 = (_29, _24.1, _22, _36);
_39.2 = _35 as f32;
_17 = &(*_17);
Goto(bb31)
}
bb31 = {
_55 = _28.fld2;
_12 = core::ptr::addr_of!(_28.fld4);
_23.0 = core::ptr::addr_of!((*_25));
_63 = _48 | _33;
_39.0 = _33 as f32;
_11 = _10;
_4 = _11.0 * _10.0;
_2 = !_10.1;
(*_59) = &_14;
_23 = Move(_16);
Goto(bb32)
}
bb32 = {
_59 = core::ptr::addr_of_mut!(_62);
_49 = 1488958680_i32 >> _10.0;
_28.fld4 = !20142_u16;
_24.0 = -_34;
_30 = !_43;
_61 = !_11.0;
_3 = _4;
_28.fld2 = _13 as f64;
_68.1.fld0 = _49 > _49;
Goto(bb33)
}
bb33 = {
_68.0 = [_68.1.fld0,_68.1.fld0,_68.1.fld0,_68.1.fld0,_68.1.fld0,_68.1.fld0,_68.1.fld0,_68.1.fld0];
_68.1 = Move(_28);
_64 = '\u{6a04e}';
_61 = _3;
_28.fld0 = _68.1.fld0;
_27 = &_15;
_49 = _30 as i32;
_34 = _39.2;
_23.1 = [29738_i16];
_28.fld1 = Move(_68.1.fld1);
_31 = -_24.2;
_14 = _13;
_18 = -_42;
_11.0 = _61;
_17 = &_6;
_51 = [_4,_37.0,_4,_10.0,_61,_10.0,_4];
_45 = _24.1 - _24.1;
Call(_33 = core::intrinsics::transmute(_61), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
(*_12) = _68.1.fld4;
_16.1 = [20899_i16];
_58 = core::ptr::addr_of!((*_25));
_71 = core::ptr::addr_of!(_25);
_38 = 136_u8;
_13 = _64 as u128;
_42 = -_18;
_29 = -_21;
(*_59) = &_13;
(*_59) = &(*_62);
_53 = &_23.1;
_67 = _43 | _28.fld0;
_10 = (_4, _2);
_23 = (Move(_58), _16.1);
Goto(bb35)
}
bb35 = {
_68.1.fld1 = core::ptr::addr_of_mut!((*_71));
_34 = _24.0 + _22;
_8 = _4 & _4;
_58 = core::ptr::addr_of!((*_25));
_28.fld5 = [_32,_11.1,_2,_37.1,_2,_2,_10.1];
_68.1.fld4 = _28.fld4 * _28.fld4;
_20 = _8 as f64;
_28.fld0 = _33 < _33;
_2 = _32;
_33 = !_63;
_17 = &(*_17);
_10 = (_3, _37.1);
(*_71) = core::ptr::addr_of_mut!(_38);
_70 = !(*_58);
_62 = &(*_62);
_28 = Adt33 { fld0: _68.1.fld0,fld1: Move(_68.1.fld1),fld2: _68.1.fld2,fld3: _7,fld4: _68.1.fld4,fld5: _68.1.fld5 };
_17 = &_6;
Goto(bb36)
}
bb36 = {
(*_25) = _70;
_17 = &(*_17);
_28.fld2 = _20;
_76 = -_60;
_29 = -_34;
_61 = !_11.0;
(*_59) = &_14;
_22 = _76 as f32;
_8 = _28.fld2 as usize;
_16.0 = core::ptr::addr_of!((*_25));
_28.fld1 = core::ptr::addr_of_mut!(_25);
_11.0 = _4;
_73.0 = _64;
_44 = _15;
_1 = _73.0 as usize;
_68.0 = [_68.1.fld0,_67,_30,_30,_67,_68.1.fld0,_68.1.fld0,_30];
Goto(bb37)
}
bb37 = {
_53 = &_23.1;
_39 = (_29, _24.1, _24.2, _36);
_28.fld4 = _73.0 as u16;
_45 = _70 as u32;
_17 = &(*_17);
_61 = _18 as usize;
_29 = _21 + _34;
_59 = core::ptr::addr_of_mut!((*_59));
_64 = _73.0;
_77 = -_24.2;
_36 = [(-1767_i16),(-5330_i16),18131_i16,(-22667_i16),(-9221_i16)];
_44 = !(*_27);
_27 = &(*_27);
_72 = _70 as isize;
_37 = (_4, _10.1);
_34 = -_39.2;
_81 = _70 as f32;
_27 = &_15;
_23.0 = Move(_16.0);
_75 = !_35;
_37 = (_10.0, _11.1);
_56 = _68.1.fld3;
_50 = &_78;
_24 = (_39.0, _39.1, _21, _36);
_10.1 = _48 as u64;
Goto(bb38)
}
bb38 = {
_62 = &_13;
_78 = Adt29::Variant2 { fld0: (*_25) };
Goto(bb39)
}
bb39 = {
(*_58) = !_70;
Call((*_25) = core::intrinsics::bswap(Field::<u8>(Variant(_78, 2), 0)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
_67 = _43;
_56 = [_15,(*_27),(*_27),_15,(*_27),(*_27)];
_59 = core::ptr::addr_of_mut!((*_59));
_6 = [(*_58),(*_25),_70,(*_25),(*_25)];
_39.2 = _34 + _29;
SetDiscriminant(_78, 2);
_84 = -_35;
_86 = !_84;
_24.1 = _14 as u32;
_68.1.fld3 = _5;
_39.0 = -_34;
_74 = -_48;
_34 = _38 as f32;
_73.1 = [_39.1,_24.1,_39.1,_24.1,_39.1,_39.1,_24.1,_39.1];
_68.1.fld1 = Move(_28.fld1);
_85 = [12366_i16,8356_i16,31451_i16,(-12951_i16),(-27523_i16)];
_11 = (_8, _32);
RET = Adt83::Variant2 { fld0: _36,fld1: _6,fld2: _63,fld3: _37.1,fld4: _28.fld5,fld5: _73.1,fld6: _11.0,fld7: _68.1.fld4 };
_62 = &_69;
Goto(bb41)
}
bb41 = {
Call(_94 = dump_var(5_usize, 45_usize, Move(_45), 14_usize, Move(_14), 5_usize, Move(_5), 67_usize, Move(_67)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_94 = dump_var(5_usize, 11_usize, Move(_11), 35_usize, Move(_35), 13_usize, Move(_13), 74_usize, Move(_74)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_94 = dump_var(5_usize, 48_usize, Move(_48), 84_usize, Move(_84), 76_usize, Move(_76), 47_usize, Move(_47)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_94 = dump_var(5_usize, 1_usize, Move(_1), 42_usize, Move(_42), 6_usize, Move(_6), 56_usize, Move(_56)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_94 = dump_var(5_usize, 18_usize, Move(_18), 7_usize, Move(_7), 10_usize, Move(_10), 43_usize, Move(_43)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_94 = dump_var(5_usize, 61_usize, Move(_61), 15_usize, Move(_15), 33_usize, Move(_33), 95_usize, _95), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: usize,mut _2: [i128; 6]) -> (usize, u64) {
mir! {
type RET = (usize, u64);
let _3: Adt29;
let _4: &'static usize;
let _5: *mut i8;
let _6: ([char; 8],);
let _7: f32;
let _8: isize;
let _9: *mut *mut *mut u8;
let _10: f64;
let _11: &'static Adt40;
let _12: [i8; 5];
let _13: Adt21;
let _14: [u64; 3];
let _15: [i16; 5];
let _16: (u128, char, bool, u8);
let _17: *mut isize;
let _18: &'static [u64; 7];
let _19: bool;
let _20: &'static usize;
let _21: *mut i16;
let _22: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _23: [i16; 1];
let _24: u128;
let _25: *mut u32;
let _26: ();
let _27: ();
{
_1 = 4037143245804916658_usize;
RET = (_1, 3921791950084684692_u64);
RET = (_1, 6532959524523466765_u64);
RET = (_1, 13077939806693067238_u64);
_1 = 27273_u16 as usize;
RET.1 = 7778205376649900757_u64;
RET.0 = _1;
RET.1 = 3276773111444648096_u64;
RET.0 = !_1;
_3 = Adt29::Variant2 { fld0: 36_u8 };
RET.0 = !_1;
place!(Field::<u8>(Variant(_3, 2), 0)) = 31_u8;
Goto(bb1)
}
bb1 = {
_3 = Adt29::Variant2 { fld0: 29_u8 };
RET.1 = 17189004608507992411_u64 * 10359002345460026352_u64;
RET.1 = !17017953344972386039_u64;
RET = (_1, 2335054074924245914_u64);
RET = (_1, 14007780024703853522_u64);
_2 = [(-142799144865250837980905898718334105976_i128),74125922532512929446839652465690407015_i128,(-90813643400039383838335342041961326733_i128),101948803830462100571561960954321488129_i128,88141683491655797508580951447242444988_i128,60919927254557965062868754228895350849_i128];
RET.0 = _1 >> RET.1;
_1 = RET.0 << RET.0;
RET.1 = 15650189504998942468_u64 - 2431169897964808748_u64;
RET.0 = (-53_i8) as usize;
place!(Field::<u8>(Variant(_3, 2), 0)) = 136_u8 - 186_u8;
RET.0 = false as usize;
Goto(bb2)
}
bb2 = {
_4 = &_1;
RET = (_1, 5768268580608953844_u64);
RET.0 = !_1;
RET.1 = 94_i8 as u64;
RET.1 = 17577021010112610073_u64;
Goto(bb3)
}
bb3 = {
_6.0 = ['\u{d4956}','\u{da19e}','\u{de2df}','\u{101712}','\u{4fbaf}','\u{8fec7}','\u{826e7}','\u{ae2f4}'];
_1 = RET.0;
_4 = &_1;
RET.1 = 3283398159642523888_u64 >> (*_4);
RET.1 = 9129264093387149239_u64;
_4 = &_1;
_1 = RET.0;
_7 = 146050469943848396868552605951275607912_i128 as f32;
RET = (_1, 15685137469592842809_u64);
RET = (_1, 5521905004214775230_u64);
RET.1 = 3084256540_u32 as u64;
_4 = &RET.0;
_7 = 1129741961_u32 as f32;
_4 = &_1;
RET.0 = 84_isize as usize;
place!(Field::<u8>(Variant(_3, 2), 0)) = (-71_i8) as u8;
_1 = 13313963268061283489638134429469605834_i128 as usize;
_7 = 1430719221_i32 as f32;
place!(Field::<u8>(Variant(_3, 2), 0)) = !220_u8;
_8 = (-9223372036854775808_isize);
_10 = (-1153269015_i32) as f64;
_3 = Adt29::Variant2 { fld0: 154_u8 };
place!(Field::<u8>(Variant(_3, 2), 0)) = 45_u8;
RET = (_1, 14091676736575108020_u64);
_1 = !RET.0;
_4 = &_1;
match _8 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb4 = {
_4 = &_1;
RET = (_1, 5768268580608953844_u64);
RET.0 = !_1;
RET.1 = 94_i8 as u64;
RET.1 = 17577021010112610073_u64;
Goto(bb3)
}
bb5 = {
_3 = Adt29::Variant2 { fld0: 29_u8 };
RET.1 = 17189004608507992411_u64 * 10359002345460026352_u64;
RET.1 = !17017953344972386039_u64;
RET = (_1, 2335054074924245914_u64);
RET = (_1, 14007780024703853522_u64);
_2 = [(-142799144865250837980905898718334105976_i128),74125922532512929446839652465690407015_i128,(-90813643400039383838335342041961326733_i128),101948803830462100571561960954321488129_i128,88141683491655797508580951447242444988_i128,60919927254557965062868754228895350849_i128];
RET.0 = _1 >> RET.1;
_1 = RET.0 << RET.0;
RET.1 = 15650189504998942468_u64 - 2431169897964808748_u64;
RET.0 = (-53_i8) as usize;
place!(Field::<u8>(Variant(_3, 2), 0)) = 136_u8 - 186_u8;
RET.0 = false as usize;
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
_1 = _8 as usize;
_7 = _1 as f32;
RET.0 = 69193939489978428364807461685169002101_u128 as usize;
RET = (_1, 2917421646014964418_u64);
_4 = &_1;
_4 = &(*_4);
RET = (_1, 18017782812216193469_u64);
_4 = &RET.0;
_4 = &(*_4);
_1 = '\u{3d44d}' as usize;
_2 = [43946039256021906003757028800997147042_i128,18596080394524094473196329207642891561_i128,108425246674155578434682306770055357376_i128,109092472719450733730335619521211682733_i128,(-52501620945590534814167871566827362402_i128),(-125755928580726504595746384454199283071_i128)];
place!(Field::<u8>(Variant(_3, 2), 0)) = !201_u8;
RET.1 = !5939107395641142597_u64;
_1 = 6348217742177024506_i64 as usize;
_6.0 = ['\u{102872}','\u{9e445}','\u{e605f}','\u{1402c}','\u{2ea42}','\u{41d96}','\u{1821e}','\u{91e78}'];
_4 = &(*_4);
RET = (_1, 7275790442003003871_u64);
_8 = -9223372036854775807_isize;
_6.0 = ['\u{2616c}','\u{3f81d}','\u{133b7}','\u{d79e}','\u{91c77}','\u{db8ea}','\u{9aabf}','\u{b416c}'];
_7 = _1 as f32;
_4 = &RET.0;
RET.1 = !4024465215902095929_u64;
RET.1 = (-16_i8) as u64;
_7 = RET.0 as f32;
_2 = [32261181763154760220026914960545153371_i128,(-107512463663500071307565617678487221682_i128),53295682673627477581814133090964545491_i128,136411330971968947045244738168591590749_i128,108520551911154458529043999110026200760_i128,(-61990127333876390348857652966224146673_i128)];
_2 = [148904507280914591686190649856207762177_i128,148357798796610566301348000496947463813_i128,(-7978602719354183507291167259259923947_i128),(-31814429843526496258957079115768703545_i128),(-116341595509835084285710742883369883888_i128),(-70194666417827376811512459276373103653_i128)];
_7 = RET.1 as f32;
Goto(bb10)
}
bb10 = {
RET = (_1, 4813256204314132983_u64);
place!(Field::<u8>(Variant(_3, 2), 0)) = !189_u8;
_8 = 129636814157015586691750081447175767313_i128 as isize;
_10 = 10010_u16 as f64;
_2 = [(-31560230518795120071650852224666092346_i128),(-8424497073453714743031024862843922552_i128),(-64758159625432259554227938629342881078_i128),(-85410804036297242436785750525988043654_i128),61756337338505715721296855322206745800_i128,(-11781467300027854522664449985343380270_i128)];
_13.fld3 = !(-110_i8);
_6.0 = ['\u{58f49}','\u{12711}','\u{e43f4}','\u{a5d5f}','\u{e9004}','\u{9914f}','\u{aec2c}','\u{20890}'];
_8 = _13.fld3 as isize;
_7 = RET.1 as f32;
RET.0 = _1 | _1;
_13.fld0 = !Field::<u8>(Variant(_3, 2), 0);
_10 = _13.fld0 as f64;
_10 = Field::<u8>(Variant(_3, 2), 0) as f64;
_16.2 = false | true;
_13.fld2 = (-89010411721684129037365729982961700744_i128);
_5 = core::ptr::addr_of_mut!(_13.fld3);
_17 = core::ptr::addr_of_mut!(_8);
RET = (_1, 6670411160531325545_u64);
RET = (_1, 313974278157736293_u64);
Call(_13.fld1 = fn7(_6, _10, _6, _2, _6, _13.fld2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_17 = core::ptr::addr_of_mut!((*_17));
_6.0 = ['\u{1e298}','\u{9b9f1}','\u{aa463}','\u{1536d}','\u{fba1e}','\u{58829}','\u{69194}','\u{b078d}'];
RET = (_1, 15998155862901286452_u64);
_16 = (3957689308928023013934378096599326051_u128, '\u{a47b7}', false, Field::<u8>(Variant(_3, 2), 0));
_8 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_1 = !RET.0;
_14 = [RET.1,RET.1,RET.1];
SetDiscriminant(_3, 2);
_15 = [6918_i16,25559_i16,13631_i16,3987_i16,18511_i16];
(*_5) = _13.fld2 as i8;
_13.fld3 = 695_u16 as i8;
RET.1 = 17934358007235448521_u64;
(*_5) = (-92_i8);
_13.fld2 = (-83359316989902426738024839714439268167_i128) + (-89783461235082895029859321761749280016_i128);
_13.fld3 = 78_i8;
match _16.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
3957689308928023013934378096599326051 => bb13,
_ => bb12
}
}
bb12 = {
_1 = _8 as usize;
_7 = _1 as f32;
RET.0 = 69193939489978428364807461685169002101_u128 as usize;
RET = (_1, 2917421646014964418_u64);
_4 = &_1;
_4 = &(*_4);
RET = (_1, 18017782812216193469_u64);
_4 = &RET.0;
_4 = &(*_4);
_1 = '\u{3d44d}' as usize;
_2 = [43946039256021906003757028800997147042_i128,18596080394524094473196329207642891561_i128,108425246674155578434682306770055357376_i128,109092472719450733730335619521211682733_i128,(-52501620945590534814167871566827362402_i128),(-125755928580726504595746384454199283071_i128)];
place!(Field::<u8>(Variant(_3, 2), 0)) = !201_u8;
RET.1 = !5939107395641142597_u64;
_1 = 6348217742177024506_i64 as usize;
_6.0 = ['\u{102872}','\u{9e445}','\u{e605f}','\u{1402c}','\u{2ea42}','\u{41d96}','\u{1821e}','\u{91e78}'];
_4 = &(*_4);
RET = (_1, 7275790442003003871_u64);
_8 = -9223372036854775807_isize;
_6.0 = ['\u{2616c}','\u{3f81d}','\u{133b7}','\u{d79e}','\u{91c77}','\u{db8ea}','\u{9aabf}','\u{b416c}'];
_7 = _1 as f32;
_4 = &RET.0;
RET.1 = !4024465215902095929_u64;
RET.1 = (-16_i8) as u64;
_7 = RET.0 as f32;
_2 = [32261181763154760220026914960545153371_i128,(-107512463663500071307565617678487221682_i128),53295682673627477581814133090964545491_i128,136411330971968947045244738168591590749_i128,108520551911154458529043999110026200760_i128,(-61990127333876390348857652966224146673_i128)];
_2 = [148904507280914591686190649856207762177_i128,148357798796610566301348000496947463813_i128,(-7978602719354183507291167259259923947_i128),(-31814429843526496258957079115768703545_i128),(-116341595509835084285710742883369883888_i128),(-70194666417827376811512459276373103653_i128)];
_7 = RET.1 as f32;
Goto(bb10)
}
bb13 = {
place!(Field::<u8>(Variant(_3, 2), 0)) = _13.fld0 << _8;
_7 = Field::<u8>(Variant(_3, 2), 0) as f32;
place!(Field::<u8>(Variant(_3, 2), 0)) = _13.fld0;
_10 = RET.1 as f64;
_13.fld2 = Field::<u8>(Variant(_3, 2), 0) as i128;
_15 = [(-10421_i16),6615_i16,20840_i16,20607_i16,10878_i16];
_10 = (-175840227_i32) as f64;
_4 = &RET.0;
_16.3 = _13.fld0;
_19 = !_16.2;
_1 = RET.0;
(*_17) = (-9223372036854775808_isize);
_13.fld3 = -(-87_i8);
_17 = core::ptr::addr_of_mut!((*_17));
RET.0 = _1 + _1;
RET.1 = !14496742704330487281_u64;
_7 = 3592_u16 as f32;
(*_17) = -9223372036854775807_isize;
_20 = &_1;
_16 = (210139744429993741222366199914759857726_u128, '\u{e6cbc}', _19, _13.fld0);
_10 = RET.1 as f64;
_2 = [_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2];
(*_5) = 107_i8 - 124_i8;
Goto(bb14)
}
bb14 = {
_19 = !_16.2;
_20 = &(*_20);
SetDiscriminant(_3, 0);
place!(Field::<bool>(Variant(_3, 0), 0)) = _19;
place!(Field::<i64>(Variant(_3, 0), 6)) = 586553401_i32 as i64;
place!(Field::<*mut u8>(Variant(_3, 0), 5)) = core::ptr::addr_of_mut!(_13.fld0);
_2 = [_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2];
place!(Field::<u128>(Variant(_3, 0), 1)) = _16.0;
_7 = _8 as f32;
_16.0 = !Field::<u128>(Variant(_3, 0), 1);
place!(Field::<Adt21>(Variant(_3, 0), 3)) = Adt21 { fld0: _16.3,fld1: _13.fld1,fld2: _13.fld2,fld3: (*_5) };
_8 = 68_isize;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(6_usize, 1_usize, Move(_1), 6_usize, Move(_6), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: ([char; 8],),mut _2: f64,mut _3: ([char; 8],),mut _4: [i128; 6],mut _5: ([char; 8],),mut _6: i128) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _7: usize;
let _8: (char, [u32; 8]);
let _9: [u64; 3];
let _10: bool;
let _11: char;
let _12: *mut (*const u8, *const u8, (u128, char, bool, u8));
let _13: f64;
let _14: u16;
let _15: f32;
let _16: Adt32;
let _17: ([bool; 8], Adt33);
let _18: i64;
let _19: f32;
let _20: isize;
let _21: [usize; 7];
let _22: f64;
let _23: u16;
let _24: bool;
let _25: char;
let _26: (f32, u32, f32, [i16; 5]);
let _27: usize;
let _28: bool;
let _29: ();
let _30: ();
{
RET = [24531_i16];
_5.0 = ['\u{8f77a}','\u{d1f7f}','\u{747a9}','\u{932f9}','\u{9bfa4}','\u{ff188}','\u{d48fe}','\u{b80df}'];
_3 = _1;
_1.0 = ['\u{91904}','\u{ca555}','\u{794a5}','\u{98a4e}','\u{873ac}','\u{753e4}','\u{6b7af}','\u{9c25a}'];
_5 = (_3.0,);
_5 = _1;
_3 = (_1.0,);
RET = [(-4194_i16)];
_1 = (_5.0,);
_4 = [_6,_6,_6,_6,_6,_6];
_3.0 = ['\u{d7e89}','\u{ec9ec}','\u{2a539}','\u{9b55b}','\u{744df}','\u{9f32a}','\u{70641}','\u{807ee}'];
_7 = 3_usize;
_1.0 = _3.0;
_5.0 = [_1.0[_7],_1.0[_7],_1.0[_7],_3.0[_7],_1.0[_7],_3.0[_7],_1.0[_7],_1.0[_7]];
_8.1 = [3427201167_u32,1686582247_u32,695791204_u32,2129609316_u32,1656090173_u32,945556297_u32,359619015_u32,2822363177_u32];
_8.1 = [1806701986_u32,3504805624_u32,4063591394_u32,1209970206_u32,1306679707_u32,1439558362_u32,4185508939_u32,324113238_u32];
Goto(bb1)
}
bb1 = {
_8.0 = _1.0[_7];
_3.0[_7] = _1.0[_7];
_5.0[_7] = _1.0[_7];
_5.0[_7] = _8.0;
_4 = [_6,_6,_6,_6,_6,_6];
_1.0 = [_5.0[_7],_5.0[_7],_3.0[_7],_3.0[_7],_5.0[_7],_8.0,_8.0,_8.0];
_4[_7] = _6 | _6;
_4[_7] = _6 - _6;
_9 = [4259366945354810533_u64,3284414523396417379_u64,9585263769731875058_u64];
_6 = _4[_7] ^ _4[_7];
_7 = !2_usize;
_2 = (-9038251186225011045_i64) as f64;
_8.0 = '\u{104636}';
RET = [18137_i16];
_6 = !(-114901083627456424225461526378297750308_i128);
_8.0 = '\u{9591}';
_10 = !false;
_2 = 39452231_u32 as f64;
_1.0 = _3.0;
_4 = [_6,_6,_6,_6,_6,_6];
_11 = _8.0;
_7 = 662316445340990452_usize + 8208784789246785920_usize;
_8.1 = [1584764708_u32,1161172408_u32,409218787_u32,2124958347_u32,3089129268_u32,2776893085_u32,3481632169_u32,2020266764_u32];
_1 = (_5.0,);
Call(_8 = fn8(_1, _1, _3, _3.0, _1, _5, _5, _1.0, _7, _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.0 = _5.0;
_6 = _8.0 as i128;
_1 = _3;
_10 = !false;
_11 = _8.0;
Goto(bb3)
}
bb3 = {
_10 = false;
_1.0 = [_8.0,_11,_8.0,_8.0,_8.0,_8.0,_11,_8.0];
_8.1 = [2180436921_u32,4028693321_u32,3925794209_u32,1514829670_u32,1225162251_u32,751880815_u32,61367300_u32,2937665224_u32];
_7 = 17592946073474892454_usize - 4_usize;
_15 = 1_i8 as f32;
_16 = Adt32::Variant2 { fld0: 10794982761807202420_u64 };
RET = [7731_i16];
_16 = Adt32::Variant0 { fld0: _15,fld1: _11,fld2: _7 };
_14 = 5898_u16;
_17.1.fld4 = _11 as u16;
_15 = (-530123315_i32) as f32;
_17.1.fld5 = [10251944630656725423_u64,5411103583080013729_u64,6323304678963346684_u64,3473148000711981369_u64,7684329878380934257_u64,2748807563151415830_u64,13410582000825187807_u64];
match _14 {
0 => bb1,
5898 => bb5,
_ => bb4
}
}
bb4 = {
_8.0 = _1.0[_7];
_3.0[_7] = _1.0[_7];
_5.0[_7] = _1.0[_7];
_5.0[_7] = _8.0;
_4 = [_6,_6,_6,_6,_6,_6];
_1.0 = [_5.0[_7],_5.0[_7],_3.0[_7],_3.0[_7],_5.0[_7],_8.0,_8.0,_8.0];
_4[_7] = _6 | _6;
_4[_7] = _6 - _6;
_9 = [4259366945354810533_u64,3284414523396417379_u64,9585263769731875058_u64];
_6 = _4[_7] ^ _4[_7];
_7 = !2_usize;
_2 = (-9038251186225011045_i64) as f64;
_8.0 = '\u{104636}';
RET = [18137_i16];
_6 = !(-114901083627456424225461526378297750308_i128);
_8.0 = '\u{9591}';
_10 = !false;
_2 = 39452231_u32 as f64;
_1.0 = _3.0;
_4 = [_6,_6,_6,_6,_6,_6];
_11 = _8.0;
_7 = 662316445340990452_usize + 8208784789246785920_usize;
_8.1 = [1584764708_u32,1161172408_u32,409218787_u32,2124958347_u32,3089129268_u32,2776893085_u32,3481632169_u32,2020266764_u32];
_1 = (_5.0,);
Call(_8 = fn8(_1, _1, _3, _3.0, _1, _5, _5, _1.0, _7, _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
_17.1.fld2 = -_2;
_17.1.fld0 = _10 | _10;
_15 = 159124218270111564223249425793772740108_u128 as f32;
_1.0 = [_8.0,_8.0,Field::<char>(Variant(_16, 0), 1),_8.0,_8.0,Field::<char>(Variant(_16, 0), 1),_11,_11];
_2 = _17.1.fld2;
_13 = -_17.1.fld2;
_17.1.fld2 = -_2;
_17.1.fld2 = _13 * _13;
match _14 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
5898 => bb13,
_ => bb12
}
}
bb6 = {
_8.0 = _1.0[_7];
_3.0[_7] = _1.0[_7];
_5.0[_7] = _1.0[_7];
_5.0[_7] = _8.0;
_4 = [_6,_6,_6,_6,_6,_6];
_1.0 = [_5.0[_7],_5.0[_7],_3.0[_7],_3.0[_7],_5.0[_7],_8.0,_8.0,_8.0];
_4[_7] = _6 | _6;
_4[_7] = _6 - _6;
_9 = [4259366945354810533_u64,3284414523396417379_u64,9585263769731875058_u64];
_6 = _4[_7] ^ _4[_7];
_7 = !2_usize;
_2 = (-9038251186225011045_i64) as f64;
_8.0 = '\u{104636}';
RET = [18137_i16];
_6 = !(-114901083627456424225461526378297750308_i128);
_8.0 = '\u{9591}';
_10 = !false;
_2 = 39452231_u32 as f64;
_1.0 = _3.0;
_4 = [_6,_6,_6,_6,_6,_6];
_11 = _8.0;
_7 = 662316445340990452_usize + 8208784789246785920_usize;
_8.1 = [1584764708_u32,1161172408_u32,409218787_u32,2124958347_u32,3089129268_u32,2776893085_u32,3481632169_u32,2020266764_u32];
_1 = (_5.0,);
Call(_8 = fn8(_1, _1, _3, _3.0, _1, _5, _5, _1.0, _7, _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_10 = false;
_1.0 = [_8.0,_11,_8.0,_8.0,_8.0,_8.0,_11,_8.0];
_8.1 = [2180436921_u32,4028693321_u32,3925794209_u32,1514829670_u32,1225162251_u32,751880815_u32,61367300_u32,2937665224_u32];
_7 = 17592946073474892454_usize - 4_usize;
_15 = 1_i8 as f32;
_16 = Adt32::Variant2 { fld0: 10794982761807202420_u64 };
RET = [7731_i16];
_16 = Adt32::Variant0 { fld0: _15,fld1: _11,fld2: _7 };
_14 = 5898_u16;
_17.1.fld4 = _11 as u16;
_15 = (-530123315_i32) as f32;
_17.1.fld5 = [10251944630656725423_u64,5411103583080013729_u64,6323304678963346684_u64,3473148000711981369_u64,7684329878380934257_u64,2748807563151415830_u64,13410582000825187807_u64];
match _14 {
0 => bb1,
5898 => bb5,
_ => bb4
}
}
bb8 = {
_3.0 = _5.0;
_6 = _8.0 as i128;
_1 = _3;
_10 = !false;
_11 = _8.0;
Goto(bb3)
}
bb9 = {
_8.0 = _1.0[_7];
_3.0[_7] = _1.0[_7];
_5.0[_7] = _1.0[_7];
_5.0[_7] = _8.0;
_4 = [_6,_6,_6,_6,_6,_6];
_1.0 = [_5.0[_7],_5.0[_7],_3.0[_7],_3.0[_7],_5.0[_7],_8.0,_8.0,_8.0];
_4[_7] = _6 | _6;
_4[_7] = _6 - _6;
_9 = [4259366945354810533_u64,3284414523396417379_u64,9585263769731875058_u64];
_6 = _4[_7] ^ _4[_7];
_7 = !2_usize;
_2 = (-9038251186225011045_i64) as f64;
_8.0 = '\u{104636}';
RET = [18137_i16];
_6 = !(-114901083627456424225461526378297750308_i128);
_8.0 = '\u{9591}';
_10 = !false;
_2 = 39452231_u32 as f64;
_1.0 = _3.0;
_4 = [_6,_6,_6,_6,_6,_6];
_11 = _8.0;
_7 = 662316445340990452_usize + 8208784789246785920_usize;
_8.1 = [1584764708_u32,1161172408_u32,409218787_u32,2124958347_u32,3089129268_u32,2776893085_u32,3481632169_u32,2020266764_u32];
_1 = (_5.0,);
Call(_8 = fn8(_1, _1, _3, _3.0, _1, _5, _5, _1.0, _7, _3.0), ReturnTo(bb2), UnwindUnreachable())
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
_17.0 = [_17.1.fld0,_17.1.fld0,_10,_10,_10,_10,_17.1.fld0,_17.1.fld0];
_19 = -_15;
_17.1.fld3 = [_6,_6,_6,_6,_6,_6];
_15 = Field::<f32>(Variant(_16, 0), 0) - _19;
_16 = Adt32::Variant2 { fld0: 10123410615051825201_u64 };
_1.0 = _3.0;
place!(Field::<u64>(Variant(_16, 2), 0)) = _8.0 as u64;
_17.1.fld4 = !_14;
_9 = [Field::<u64>(Variant(_16, 2), 0),Field::<u64>(Variant(_16, 2), 0),Field::<u64>(Variant(_16, 2), 0)];
SetDiscriminant(_16, 0);
_17.1.fld0 = !_10;
_17.1.fld2 = -_13;
_8.0 = _11;
_5.0 = [_11,_8.0,_11,_11,_11,_8.0,_8.0,_8.0];
_3.0 = _1.0;
_4 = _17.1.fld3;
match _14 {
0 => bb7,
1 => bb12,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
5898 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_8.0 = _1.0[_7];
_3.0[_7] = _1.0[_7];
_5.0[_7] = _1.0[_7];
_5.0[_7] = _8.0;
_4 = [_6,_6,_6,_6,_6,_6];
_1.0 = [_5.0[_7],_5.0[_7],_3.0[_7],_3.0[_7],_5.0[_7],_8.0,_8.0,_8.0];
_4[_7] = _6 | _6;
_4[_7] = _6 - _6;
_9 = [4259366945354810533_u64,3284414523396417379_u64,9585263769731875058_u64];
_6 = _4[_7] ^ _4[_7];
_7 = !2_usize;
_2 = (-9038251186225011045_i64) as f64;
_8.0 = '\u{104636}';
RET = [18137_i16];
_6 = !(-114901083627456424225461526378297750308_i128);
_8.0 = '\u{9591}';
_10 = !false;
_2 = 39452231_u32 as f64;
_1.0 = _3.0;
_4 = [_6,_6,_6,_6,_6,_6];
_11 = _8.0;
_7 = 662316445340990452_usize + 8208784789246785920_usize;
_8.1 = [1584764708_u32,1161172408_u32,409218787_u32,2124958347_u32,3089129268_u32,2776893085_u32,3481632169_u32,2020266764_u32];
_1 = (_5.0,);
Call(_8 = fn8(_1, _1, _3, _3.0, _1, _5, _5, _1.0, _7, _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
_8.0 = _1.0[_7];
_3.0[_7] = _1.0[_7];
_5.0[_7] = _1.0[_7];
_5.0[_7] = _8.0;
_4 = [_6,_6,_6,_6,_6,_6];
_1.0 = [_5.0[_7],_5.0[_7],_3.0[_7],_3.0[_7],_5.0[_7],_8.0,_8.0,_8.0];
_4[_7] = _6 | _6;
_4[_7] = _6 - _6;
_9 = [4259366945354810533_u64,3284414523396417379_u64,9585263769731875058_u64];
_6 = _4[_7] ^ _4[_7];
_7 = !2_usize;
_2 = (-9038251186225011045_i64) as f64;
_8.0 = '\u{104636}';
RET = [18137_i16];
_6 = !(-114901083627456424225461526378297750308_i128);
_8.0 = '\u{9591}';
_10 = !false;
_2 = 39452231_u32 as f64;
_1.0 = _3.0;
_4 = [_6,_6,_6,_6,_6,_6];
_11 = _8.0;
_7 = 662316445340990452_usize + 8208784789246785920_usize;
_8.1 = [1584764708_u32,1161172408_u32,409218787_u32,2124958347_u32,3089129268_u32,2776893085_u32,3481632169_u32,2020266764_u32];
_1 = (_5.0,);
Call(_8 = fn8(_1, _1, _3, _3.0, _1, _5, _5, _1.0, _7, _3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_3.0 = _5.0;
_6 = _8.0 as i128;
_1 = _3;
_10 = !false;
_11 = _8.0;
Goto(bb3)
}
bb19 = {
_23 = 14455031563385603025_u64 as u16;
place!(Field::<f32>(Variant(_16, 0), 0)) = _15;
_7 = 7222063731418953910_usize | 0_usize;
_21 = [_7,_7,_7,_7,_7,_7,_7];
_15 = Field::<f32>(Variant(_16, 0), 0);
_17.1.fld0 = !_10;
_17.1.fld0 = _10;
_24 = _15 > _15;
_6 = 149675153631983144735111086315630679927_i128 << _14;
_19 = Field::<f32>(Variant(_16, 0), 0) - _15;
RET = [29284_i16];
place!(Field::<usize>(Variant(_16, 0), 2)) = 18156_i16 as usize;
_26.3 = [(-16703_i16),3445_i16,(-30751_i16),(-21876_i16),(-14616_i16)];
_17.1.fld3 = [_6,_6,_6,_6,_6,_6];
_26.1 = _24 as u32;
_8.1 = [_26.1,_26.1,_26.1,_26.1,_26.1,_26.1,_26.1,_26.1];
_15 = _19;
_5.0 = [_11,_8.0,_11,_11,_11,_11,_11,_11];
_26.3 = [13244_i16,31809_i16,32110_i16,(-7410_i16),(-212_i16)];
_3 = (_5.0,);
_17.1.fld0 = !_24;
_11 = _8.0;
_22 = _17.1.fld2;
_11 = _8.0;
Goto(bb20)
}
bb20 = {
Call(_29 = dump_var(7_usize, 3_usize, Move(_3), 23_usize, Move(_23), 5_usize, Move(_5), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_29 = dump_var(7_usize, 24_usize, Move(_24), 11_usize, Move(_11), 4_usize, Move(_4), 30_usize, _30), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: ([char; 8],),mut _2: ([char; 8],),mut _3: ([char; 8],),mut _4: [char; 8],mut _5: ([char; 8],),mut _6: ([char; 8],),mut _7: ([char; 8],),mut _8: [char; 8],mut _9: usize,mut _10: [char; 8]) -> (char, [u32; 8]) {
mir! {
type RET = (char, [u32; 8]);
let _11: (f32, u32, f32, [i16; 5]);
let _12: [i16; 5];
let _13: isize;
let _14: f32;
let _15: i32;
let _16: f32;
let _17: ([bool; 8], Adt33);
let _18: ([char; 8],);
let _19: [char; 8];
let _20: u16;
let _21: *mut (u128, char, bool, u8);
let _22: [u32; 8];
let _23: [i16; 5];
let _24: char;
let _25: f32;
let _26: i8;
let _27: *const &'static f32;
let _28: usize;
let _29: char;
let _30: u64;
let _31: Adt40;
let _32: bool;
let _33: [i8; 5];
let _34: isize;
let _35: (char, [u32; 8]);
let _36: ();
let _37: ();
{
_6 = (_5.0,);
_9 = 5568434034205171153_usize;
RET.1 = [350462899_u32,1812118728_u32,869340721_u32,282712186_u32,2648651154_u32,1641142118_u32,831595147_u32,3730027474_u32];
_7 = _2;
RET.0 = '\u{1f5d6}';
_11.3 = [(-30750_i16),(-27027_i16),(-8953_i16),20580_i16,(-15357_i16)];
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_11.2 = 124_isize as f32;
_7 = (_6.0,);
_11.0 = 95_u8 as f32;
_10 = _2.0;
_5 = (_10,);
_1.0 = _10;
_11.0 = 249_u8 as f32;
RET.0 = '\u{c4dfe}';
_1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_5.0 = _10;
Goto(bb1)
}
bb1 = {
RET.0 = '\u{436af}';
RET.1 = [2652056875_u32,860452039_u32,903071173_u32,1957750404_u32,1054403397_u32,1805588286_u32,3250564137_u32,1188756044_u32];
_6 = (_5.0,);
_12 = [(-20877_i16),25991_i16,(-7676_i16),18511_i16,(-9623_i16)];
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.0 = '\u{593fb}';
_4 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_18 = _7;
_17.1.fld0 = _9 <= _9;
_2.0 = _10;
_2.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.0 = [_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0];
_11.3 = _12;
_16 = _11.0 * _11.2;
Goto(bb2)
}
bb2 = {
_16 = _11.0 - _11.2;
_17.1.fld5 = [9753631863740621905_u64,1808494214696683583_u64,2339936855083091190_u64,8732633479647586_u64,1480874611578094721_u64,15444352623220218603_u64,2622929483359434407_u64];
_15 = 1705523093_i32;
_9 = 96_i8 as usize;
_7.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.1 = [4066669942_u32,4080415279_u32,3999991149_u32,789869856_u32,753263454_u32,1261008569_u32,1214366212_u32,1832000000_u32];
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = 34544_u16 >> _9;
RET.0 = '\u{8a2c5}';
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = (-9223372036854775808_isize) as u16;
_5 = _6;
_13 = _15 as isize;
_17.1.fld3 = [(-18264719730689435266621155236664570341_i128),63599011816664702239045434572638333503_i128,125691380559121488173448068024678112580_i128,(-153933208459423447024999661601397781919_i128),(-10336893234423621358372577446090649217_i128),134033578507442341267046460197655869081_i128];
_17.1.fld2 = (-104_i8) as f64;
_3 = (_5.0,);
_16 = _11.0 - _11.0;
_11.3 = [(-8549_i16),(-19199_i16),25121_i16,(-5686_i16),25671_i16];
_6 = (_5.0,);
_11 = (_16, 1235546143_u32, _16, _12);
_5.0 = _18.0;
RET.0 = '\u{3cef9}';
_13 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_10 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld3 = [60986764944401454762934349591477219493_i128,(-75582064466269095276537161072984711831_i128),19995480068067021479126149990919627102_i128,(-95053779389415527727722731852111638529_i128),(-119169832030741508252978717961602918559_i128),(-125254982598676135280163729584626956710_i128)];
_1 = (_7.0,);
_17.1.fld5 = [3545035727452739482_u64,10642885877578278671_u64,12821232396271984098_u64,7835582444689686676_u64,12103349715887219319_u64,4700984606782612612_u64,16231702945817179711_u64];
_18.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Call(_20 = core::intrinsics::bswap(_17.1.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.0 = '\u{dadfc}';
_7.0 = _19;
RET.1 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_22 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_14 = _16 - _16;
_9 = 24_u8 as usize;
_18 = (_3.0,);
_5.0 = _3.0;
_11.2 = 3172127818648829744_u64 as f32;
_6 = (_3.0,);
_11.2 = -_11.0;
_11 = (_16, 897554782_u32, _16, _12);
RET.0 = '\u{816de}';
_17.1.fld2 = 7131124636635145147_u64 as f64;
_7.0 = _18.0;
_1.0 = _6.0;
_18 = (_1.0,);
Goto(bb4)
}
bb4 = {
_25 = -_14;
_11.1 = 3587185526_u32;
Goto(bb5)
}
bb5 = {
_17.1.fld4 = 42594_u16;
_26 = RET.0 as i8;
_15 = !(-938519573_i32);
_13 = 739204599166203095_i64 as isize;
_17.0 = [_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0];
_5.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Goto(bb6)
}
bb6 = {
_5 = (_6.0,);
_11.3 = _12;
_7 = (_18.0,);
_5.0 = _6.0;
_23 = [(-14070_i16),(-32202_i16),(-3022_i16),(-14100_i16),(-26068_i16)];
_1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
match _11.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
3587185526 => bb10,
_ => bb9
}
}
bb7 = {
_17.1.fld4 = 42594_u16;
_26 = RET.0 as i8;
_15 = !(-938519573_i32);
_13 = 739204599166203095_i64 as isize;
_17.0 = [_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0];
_5.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Goto(bb6)
}
bb8 = {
_16 = _11.0 - _11.2;
_17.1.fld5 = [9753631863740621905_u64,1808494214696683583_u64,2339936855083091190_u64,8732633479647586_u64,1480874611578094721_u64,15444352623220218603_u64,2622929483359434407_u64];
_15 = 1705523093_i32;
_9 = 96_i8 as usize;
_7.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.1 = [4066669942_u32,4080415279_u32,3999991149_u32,789869856_u32,753263454_u32,1261008569_u32,1214366212_u32,1832000000_u32];
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = 34544_u16 >> _9;
RET.0 = '\u{8a2c5}';
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = (-9223372036854775808_isize) as u16;
_5 = _6;
_13 = _15 as isize;
_17.1.fld3 = [(-18264719730689435266621155236664570341_i128),63599011816664702239045434572638333503_i128,125691380559121488173448068024678112580_i128,(-153933208459423447024999661601397781919_i128),(-10336893234423621358372577446090649217_i128),134033578507442341267046460197655869081_i128];
_17.1.fld2 = (-104_i8) as f64;
_3 = (_5.0,);
_16 = _11.0 - _11.0;
_11.3 = [(-8549_i16),(-19199_i16),25121_i16,(-5686_i16),25671_i16];
_6 = (_5.0,);
_11 = (_16, 1235546143_u32, _16, _12);
_5.0 = _18.0;
RET.0 = '\u{3cef9}';
_13 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_10 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld3 = [60986764944401454762934349591477219493_i128,(-75582064466269095276537161072984711831_i128),19995480068067021479126149990919627102_i128,(-95053779389415527727722731852111638529_i128),(-119169832030741508252978717961602918559_i128),(-125254982598676135280163729584626956710_i128)];
_1 = (_7.0,);
_17.1.fld5 = [3545035727452739482_u64,10642885877578278671_u64,12821232396271984098_u64,7835582444689686676_u64,12103349715887219319_u64,4700984606782612612_u64,16231702945817179711_u64];
_18.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Call(_20 = core::intrinsics::bswap(_17.1.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.0 = '\u{dadfc}';
_7.0 = _19;
RET.1 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_22 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_14 = _16 - _16;
_9 = 24_u8 as usize;
_18 = (_3.0,);
_5.0 = _3.0;
_11.2 = 3172127818648829744_u64 as f32;
_6 = (_3.0,);
_11.2 = -_11.0;
_11 = (_16, 897554782_u32, _16, _12);
RET.0 = '\u{816de}';
_17.1.fld2 = 7131124636635145147_u64 as f64;
_7.0 = _18.0;
_1.0 = _6.0;
_18 = (_1.0,);
Goto(bb4)
}
bb10 = {
_18 = _3;
_22 = RET.1;
_11.2 = (-113352466577639164563191541269795834878_i128) as f32;
RET.1 = _22;
_23 = _12;
_28 = _11.1 as usize;
_17.1.fld2 = _17.1.fld4 as f64;
match _11.1 {
0 => bb9,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
3587185526 => bb18,
_ => bb17
}
}
bb11 = {
_16 = _11.0 - _11.2;
_17.1.fld5 = [9753631863740621905_u64,1808494214696683583_u64,2339936855083091190_u64,8732633479647586_u64,1480874611578094721_u64,15444352623220218603_u64,2622929483359434407_u64];
_15 = 1705523093_i32;
_9 = 96_i8 as usize;
_7.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.1 = [4066669942_u32,4080415279_u32,3999991149_u32,789869856_u32,753263454_u32,1261008569_u32,1214366212_u32,1832000000_u32];
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = 34544_u16 >> _9;
RET.0 = '\u{8a2c5}';
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = (-9223372036854775808_isize) as u16;
_5 = _6;
_13 = _15 as isize;
_17.1.fld3 = [(-18264719730689435266621155236664570341_i128),63599011816664702239045434572638333503_i128,125691380559121488173448068024678112580_i128,(-153933208459423447024999661601397781919_i128),(-10336893234423621358372577446090649217_i128),134033578507442341267046460197655869081_i128];
_17.1.fld2 = (-104_i8) as f64;
_3 = (_5.0,);
_16 = _11.0 - _11.0;
_11.3 = [(-8549_i16),(-19199_i16),25121_i16,(-5686_i16),25671_i16];
_6 = (_5.0,);
_11 = (_16, 1235546143_u32, _16, _12);
_5.0 = _18.0;
RET.0 = '\u{3cef9}';
_13 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_10 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld3 = [60986764944401454762934349591477219493_i128,(-75582064466269095276537161072984711831_i128),19995480068067021479126149990919627102_i128,(-95053779389415527727722731852111638529_i128),(-119169832030741508252978717961602918559_i128),(-125254982598676135280163729584626956710_i128)];
_1 = (_7.0,);
_17.1.fld5 = [3545035727452739482_u64,10642885877578278671_u64,12821232396271984098_u64,7835582444689686676_u64,12103349715887219319_u64,4700984606782612612_u64,16231702945817179711_u64];
_18.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Call(_20 = core::intrinsics::bswap(_17.1.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_16 = _11.0 - _11.2;
_17.1.fld5 = [9753631863740621905_u64,1808494214696683583_u64,2339936855083091190_u64,8732633479647586_u64,1480874611578094721_u64,15444352623220218603_u64,2622929483359434407_u64];
_15 = 1705523093_i32;
_9 = 96_i8 as usize;
_7.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.1 = [4066669942_u32,4080415279_u32,3999991149_u32,789869856_u32,753263454_u32,1261008569_u32,1214366212_u32,1832000000_u32];
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = 34544_u16 >> _9;
RET.0 = '\u{8a2c5}';
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld4 = (-9223372036854775808_isize) as u16;
_5 = _6;
_13 = _15 as isize;
_17.1.fld3 = [(-18264719730689435266621155236664570341_i128),63599011816664702239045434572638333503_i128,125691380559121488173448068024678112580_i128,(-153933208459423447024999661601397781919_i128),(-10336893234423621358372577446090649217_i128),134033578507442341267046460197655869081_i128];
_17.1.fld2 = (-104_i8) as f64;
_3 = (_5.0,);
_16 = _11.0 - _11.0;
_11.3 = [(-8549_i16),(-19199_i16),25121_i16,(-5686_i16),25671_i16];
_6 = (_5.0,);
_11 = (_16, 1235546143_u32, _16, _12);
_5.0 = _18.0;
RET.0 = '\u{3cef9}';
_13 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_8 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_10 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_17.1.fld3 = [60986764944401454762934349591477219493_i128,(-75582064466269095276537161072984711831_i128),19995480068067021479126149990919627102_i128,(-95053779389415527727722731852111638529_i128),(-119169832030741508252978717961602918559_i128),(-125254982598676135280163729584626956710_i128)];
_1 = (_7.0,);
_17.1.fld5 = [3545035727452739482_u64,10642885877578278671_u64,12821232396271984098_u64,7835582444689686676_u64,12103349715887219319_u64,4700984606782612612_u64,16231702945817179711_u64];
_18.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Call(_20 = core::intrinsics::bswap(_17.1.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_17.1.fld4 = 42594_u16;
_26 = RET.0 as i8;
_15 = !(-938519573_i32);
_13 = 739204599166203095_i64 as isize;
_17.0 = [_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0];
_5.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Goto(bb6)
}
bb14 = {
_5 = (_6.0,);
_11.3 = _12;
_7 = (_18.0,);
_5.0 = _6.0;
_23 = [(-14070_i16),(-32202_i16),(-3022_i16),(-14100_i16),(-26068_i16)];
_1.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
match _11.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
3587185526 => bb10,
_ => bb9
}
}
bb15 = {
_17.1.fld4 = 42594_u16;
_26 = RET.0 as i8;
_15 = !(-938519573_i32);
_13 = 739204599166203095_i64 as isize;
_17.0 = [_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0,_17.1.fld0];
_5.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
Goto(bb6)
}
bb16 = {
_25 = -_14;
_11.1 = 3587185526_u32;
Goto(bb5)
}
bb17 = {
_19 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.0 = '\u{dadfc}';
_7.0 = _19;
RET.1 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_22 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_14 = _16 - _16;
_9 = 24_u8 as usize;
_18 = (_3.0,);
_5.0 = _3.0;
_11.2 = 3172127818648829744_u64 as f32;
_6 = (_3.0,);
_11.2 = -_11.0;
_11 = (_16, 897554782_u32, _16, _12);
RET.0 = '\u{816de}';
_17.1.fld2 = 7131124636635145147_u64 as f64;
_7.0 = _18.0;
_1.0 = _6.0;
_18 = (_1.0,);
Goto(bb4)
}
bb18 = {
_3 = (_5.0,);
_1 = (_5.0,);
_17.1.fld3 = [113766968450781733997250889365154137610_i128,26631344988359208394769694425356675010_i128,(-49816701297928211491693818051177975852_i128),118767931310039161695890947476872031100_i128,(-38791259266926637926113502560996074001_i128),(-43887178063589834653628504922875667833_i128)];
_12 = [(-2552_i16),(-29057_i16),(-8182_i16),7701_i16,851_i16];
_4 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_20 = _17.1.fld4;
_32 = RET.0 >= RET.0;
_13 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_17.1.fld5 = [9214692132040164130_u64,11474297616440659277_u64,14939790432250935770_u64,13858855622759310223_u64,14964197319750027859_u64,1306862639185564763_u64,8730520768587220673_u64];
_11 = (_25, 140578633_u32, _14, _12);
_18 = _3;
_11.0 = _11.1 as f32;
_33 = [_26,_26,_26,_26,_26];
_11.0 = _17.1.fld2 as f32;
_35.1 = [_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_9 = _26 as usize;
_30 = 12414628777759344252_u64 - 7240554193835688338_u64;
Goto(bb19)
}
bb19 = {
Call(_36 = dump_var(8_usize, 33_usize, Move(_33), 19_usize, Move(_19), 6_usize, Move(_6), 22_usize, Move(_22)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_36 = dump_var(8_usize, 26_usize, Move(_26), 10_usize, Move(_10), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_36 = dump_var(8_usize, 23_usize, Move(_23), 9_usize, Move(_9), 28_usize, Move(_28), 37_usize, _37), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: &'static [u8; 5]) -> *const u16 {
mir! {
type RET = *const u16;
let _2: (usize, u64);
let _3: char;
let _4: u8;
let _5: char;
let _6: (f32, u32, f32, [i16; 5]);
let _7: [i16; 1];
let _8: &'static Adt29;
let _9: *mut i8;
let _10: (isize,);
let _11: *mut &'static u128;
let _12: (char, [u32; 8]);
let _13: isize;
let _14: Adt40;
let _15: Adt21;
let _16: i64;
let _17: isize;
let _18: &'static *mut u8;
let _19: *mut isize;
let _20: char;
let _21: f32;
let _22: ([i16; 1],);
let _23: usize;
let _24: [i32; 3];
let _25: [i8; 5];
let _26: [i32; 3];
let _27: [i32; 3];
let _28: isize;
let _29: *mut (u128, char, bool, u8);
let _30: (i128, &'static u128, (f32, u32, f32, [i16; 5]), [i128; 6]);
let _31: u8;
let _32: u32;
let _33: char;
let _34: i64;
let _35: (*const u8, [i16; 1]);
let _36: char;
let _37: char;
let _38: isize;
let _39: f32;
let _40: isize;
let _41: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _42: Adt29;
let _43: *mut i8;
let _44: &'static f32;
let _45: &'static [u8; 5];
let _46: i32;
let _47: Adt40;
let _48: &'static [u8; 5];
let _49: i64;
let _50: i64;
let _51: bool;
let _52: isize;
let _53: Adt40;
let _54: *const &'static f32;
let _55: u8;
let _56: [usize; 7];
let _57: [usize; 1];
let _58: &'static ([i16; 1],);
let _59: Adt29;
let _60: isize;
let _61: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _62: u64;
let _63: (isize,);
let _64: isize;
let _65: *mut i8;
let _66: i8;
let _67: isize;
let _68: *const u8;
let _69: &'static *const (u128, char, bool, u8);
let _70: Adt84;
let _71: *mut i8;
let _72: *mut *mut u8;
let _73: &'static f32;
let _74: isize;
let _75: isize;
let _76: *mut (*const u8, *const u8, (u128, char, bool, u8));
let _77: bool;
let _78: i16;
let _79: isize;
let _80: f64;
let _81: &'static [u8; 5];
let _82: (char,);
let _83: *mut i8;
let _84: [i128; 6];
let _85: isize;
let _86: u128;
let _87: f64;
let _88: bool;
let _89: f64;
let _90: u16;
let _91: char;
let _92: *mut &'static u128;
let _93: f32;
let _94: (char, *mut u8);
let _95: [usize; 7];
let _96: [usize; 7];
let _97: ();
let _98: ();
{
_2.1 = 1732291371711560346_u64 & 11715411401712158353_u64;
_2.0 = !2125892236433677732_usize;
_2.1 = 21_u8 as u64;
_3 = '\u{b5ccb}';
_2.0 = 15294102247684766265_usize;
_2 = (15632533843526096866_usize, 14715018464782980340_u64);
_3 = '\u{e0a56}';
_3 = '\u{501b6}';
_2.1 = 462574733349188668_u64;
_2 = (1_usize, 4678701339911219578_u64);
_3 = '\u{3b935}';
_5 = _3;
_5 = _3;
_4 = !94_u8;
_6.0 = _4 as f32;
_6.3 = [(-215_i16),9882_i16,(-18478_i16),(-21760_i16),1514_i16];
_2.0 = !4024282053291486305_usize;
_5 = _3;
_6.2 = 2688104586145845345_i64 as f32;
Goto(bb1)
}
bb1 = {
_5 = _3;
_6.3 = [20408_i16,(-2518_i16),17848_i16,(-26548_i16),29132_i16];
_6.2 = _2.1 as f32;
_4 = 195_u8;
_7 = [20599_i16];
_3 = _5;
_6.1 = !2605752422_u32;
_6.2 = (-6285804091547278111_i64) as f32;
_6.2 = _6.0;
_2.1 = 14724433105101609292_u64 ^ 17705250608171702865_u64;
_6.2 = -_6.0;
_6.2 = _2.0 as f32;
_2.0 = 6_usize & 13307886522149624922_usize;
_6.3 = [(-9366_i16),(-13561_i16),(-8941_i16),(-5418_i16),22952_i16];
_6.3 = [(-30826_i16),(-2528_i16),6757_i16,(-1756_i16),7519_i16];
_6.0 = -_6.2;
_7 = [27770_i16];
_2.1 = 2600_u16 as u64;
_3 = _5;
_10.0 = (-1135071035_i32) as isize;
_10.0 = 23478_i16 as isize;
_6.1 = 263882067_u32 & 863544770_u32;
_6.1 = !1492319459_u32;
_2.0 = (-3846753344977284227_i64) as usize;
_7 = [3013_i16];
_6.1 = !3226518580_u32;
_10 = ((-33_isize),);
Goto(bb2)
}
bb2 = {
_2.1 = 8642066942633737222_u64 * 5111048117307863658_u64;
_6.0 = _6.2 + _6.2;
_2 = (11987054168078649445_usize, 13074142264302349148_u64);
_2 = (3_usize, 2149418893700218143_u64);
_6.1 = 3576522834_u32 ^ 3825406929_u32;
_7 = [(-21382_i16)];
_10.0 = _2.1 as isize;
_5 = _3;
_4 = 47_u8;
_6.1 = 240003547_u32 >> _10.0;
_2.1 = !3795960017324701778_u64;
_3 = _5;
_12.0 = _5;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
_3 = _5;
_2 = (0_usize, 8450311067998312541_u64);
_3 = _12.0;
_3 = _12.0;
_15.fld3 = 3_i8 * (-38_i8);
_13 = !_10.0;
_13 = _10.0;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
Call(_9 = fn10(_10.0, _12, _10, _10.0, _12.1, _2.0, _2.1, _5, _6.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2.1 = 9700030073892390526_u64;
_15.fld1 = _7;
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 129612103560573138036556337316018713051_i128,fld3: (-59_i8) };
_17 = _10.0 * _10.0;
_7 = [(-27782_i16)];
_2.0 = 6_usize;
_2 = (4_usize, 181334022440294282_u64);
_6.3 = [(-21724_i16),25999_i16,(-30320_i16),1521_i16,(-16675_i16)];
_6.2 = _6.0 + _6.0;
_2.0 = !3_usize;
_15.fld1 = _7;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
_10.0 = _2.1 as isize;
_2 = (4_usize, 7777377113966708973_u64);
_4 = _15.fld2 as u8;
_4 = _15.fld0 * _15.fld0;
_15.fld1 = [(-13854_i16)];
_3 = _5;
_15.fld0 = _4 + _4;
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 35817995748892598635105051450172554226_i128,fld3: (-97_i8) };
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 138699162145786599351261905333998704433_i128,fld3: (-10_i8) };
_15.fld2 = (-863841623_i32) as i128;
_20 = _5;
_5 = _12.0;
_10 = (_13,);
_20 = _3;
Goto(bb4)
}
bb4 = {
_2.0 = !5582986626315139140_usize;
_6.2 = -_6.0;
_7 = [27450_i16];
_10 = (_13,);
_10 = (_17,);
_3 = _12.0;
_16 = 3521099574830401004_i64 ^ 4244407627784384607_i64;
Goto(bb5)
}
bb5 = {
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 78868187822126158729082113622677430438_i128,fld3: (-90_i8) };
_16 = 8487278687559642643_i64;
_25 = [_15.fld3,_15.fld3,_15.fld3,_15.fld3,_15.fld3];
_6.1 = _15.fld3 as u32;
_19 = core::ptr::addr_of_mut!(_13);
_17 = (*_19) >> _10.0;
_22.0 = _15.fld1;
_23 = _2.0 - _2.0;
_2 = (_23, 13711076299643779659_u64);
_19 = core::ptr::addr_of_mut!((*_19));
_21 = -_6.2;
_7 = _15.fld1;
_10 = (_17,);
_6.1 = 1965495372_u32;
_13 = _15.fld3 as isize;
_2 = (_23, 16000484349662214703_u64);
_22 = (_7,);
_15.fld3 = -37_i8;
_4 = _23 as u8;
(*_19) = 216520936602995704624496447669971404640_u128 as isize;
Goto(bb6)
}
bb6 = {
_28 = _17 << (*_19);
_10 = (_28,);
_22.0 = _7;
_26 = [1752068306_i32,(-1009655184_i32),599084460_i32];
_24 = _26;
_24 = [(-1362861985_i32),69965984_i32,393668891_i32];
_23 = _2.0;
_2 = (_23, 7643581120976036485_u64);
_21 = -_6.2;
_9 = core::ptr::addr_of_mut!(_15.fld3);
_25 = [(*_9),_15.fld3,(*_9),(*_9),(*_9)];
_27 = [480814479_i32,(-966972221_i32),880878656_i32];
_6.1 = 3463884318_u32;
_15 = Adt21 { fld0: _4,fld1: _7,fld2: (-123649503629491629449660773216859594958_i128),fld3: (-101_i8) };
_2.1 = 11870049279788055361_u64;
_2.0 = _23;
_15.fld2 = 47295332116481561809464183705559469873_i128 - (-25910078701181569647679162052261998543_i128);
_2.1 = !1652756660457150414_u64;
_23 = _2.0 & _2.0;
(*_9) = 120_i8 - 117_i8;
_6.0 = -_6.2;
_25 = [(*_9),(*_9),_15.fld3,(*_9),(*_9)];
_30.3 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
match _16 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
8487278687559642643 => bb12,
_ => bb11
}
}
bb7 = {
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 78868187822126158729082113622677430438_i128,fld3: (-90_i8) };
_16 = 8487278687559642643_i64;
_25 = [_15.fld3,_15.fld3,_15.fld3,_15.fld3,_15.fld3];
_6.1 = _15.fld3 as u32;
_19 = core::ptr::addr_of_mut!(_13);
_17 = (*_19) >> _10.0;
_22.0 = _15.fld1;
_23 = _2.0 - _2.0;
_2 = (_23, 13711076299643779659_u64);
_19 = core::ptr::addr_of_mut!((*_19));
_21 = -_6.2;
_7 = _15.fld1;
_10 = (_17,);
_6.1 = 1965495372_u32;
_13 = _15.fld3 as isize;
_2 = (_23, 16000484349662214703_u64);
_22 = (_7,);
_15.fld3 = -37_i8;
_4 = _23 as u8;
(*_19) = 216520936602995704624496447669971404640_u128 as isize;
Goto(bb6)
}
bb8 = {
_2.0 = !5582986626315139140_usize;
_6.2 = -_6.0;
_7 = [27450_i16];
_10 = (_13,);
_10 = (_17,);
_3 = _12.0;
_16 = 3521099574830401004_i64 ^ 4244407627784384607_i64;
Goto(bb5)
}
bb9 = {
_2.1 = 9700030073892390526_u64;
_15.fld1 = _7;
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 129612103560573138036556337316018713051_i128,fld3: (-59_i8) };
_17 = _10.0 * _10.0;
_7 = [(-27782_i16)];
_2.0 = 6_usize;
_2 = (4_usize, 181334022440294282_u64);
_6.3 = [(-21724_i16),25999_i16,(-30320_i16),1521_i16,(-16675_i16)];
_6.2 = _6.0 + _6.0;
_2.0 = !3_usize;
_15.fld1 = _7;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
_10.0 = _2.1 as isize;
_2 = (4_usize, 7777377113966708973_u64);
_4 = _15.fld2 as u8;
_4 = _15.fld0 * _15.fld0;
_15.fld1 = [(-13854_i16)];
_3 = _5;
_15.fld0 = _4 + _4;
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 35817995748892598635105051450172554226_i128,fld3: (-97_i8) };
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 138699162145786599351261905333998704433_i128,fld3: (-10_i8) };
_15.fld2 = (-863841623_i32) as i128;
_20 = _5;
_5 = _12.0;
_10 = (_13,);
_20 = _3;
Goto(bb4)
}
bb10 = {
_2.1 = 8642066942633737222_u64 * 5111048117307863658_u64;
_6.0 = _6.2 + _6.2;
_2 = (11987054168078649445_usize, 13074142264302349148_u64);
_2 = (3_usize, 2149418893700218143_u64);
_6.1 = 3576522834_u32 ^ 3825406929_u32;
_7 = [(-21382_i16)];
_10.0 = _2.1 as isize;
_5 = _3;
_4 = 47_u8;
_6.1 = 240003547_u32 >> _10.0;
_2.1 = !3795960017324701778_u64;
_3 = _5;
_12.0 = _5;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
_3 = _5;
_2 = (0_usize, 8450311067998312541_u64);
_3 = _12.0;
_3 = _12.0;
_15.fld3 = 3_i8 * (-38_i8);
_13 = !_10.0;
_13 = _10.0;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
Call(_9 = fn10(_10.0, _12, _10, _10.0, _12.1, _2.0, _2.1, _5, _6.1), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_5 = _3;
_6.3 = [20408_i16,(-2518_i16),17848_i16,(-26548_i16),29132_i16];
_6.2 = _2.1 as f32;
_4 = 195_u8;
_7 = [20599_i16];
_3 = _5;
_6.1 = !2605752422_u32;
_6.2 = (-6285804091547278111_i64) as f32;
_6.2 = _6.0;
_2.1 = 14724433105101609292_u64 ^ 17705250608171702865_u64;
_6.2 = -_6.0;
_6.2 = _2.0 as f32;
_2.0 = 6_usize & 13307886522149624922_usize;
_6.3 = [(-9366_i16),(-13561_i16),(-8941_i16),(-5418_i16),22952_i16];
_6.3 = [(-30826_i16),(-2528_i16),6757_i16,(-1756_i16),7519_i16];
_6.0 = -_6.2;
_7 = [27770_i16];
_2.1 = 2600_u16 as u64;
_3 = _5;
_10.0 = (-1135071035_i32) as isize;
_10.0 = 23478_i16 as isize;
_6.1 = 263882067_u32 & 863544770_u32;
_6.1 = !1492319459_u32;
_2.0 = (-3846753344977284227_i64) as usize;
_7 = [3013_i16];
_6.1 = !3226518580_u32;
_10 = ((-33_isize),);
Goto(bb2)
}
bb12 = {
_16 = (-12167_i16) as i64;
_31 = _16 as u8;
_28 = (*_19);
_26 = [(-1432225324_i32),1928983932_i32,(-1859644016_i32)];
_20 = _12.0;
_30.2.1 = _6.1;
_30.3 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_6.1 = _30.2.1 % _30.2.1;
_30.2.2 = 32539066_i32 as f32;
_30.2.3 = _6.3;
_35.0 = core::ptr::addr_of!(_15.fld0);
_22 = (_7,);
_31 = _30.2.1 as u8;
_35.1 = [(-28628_i16)];
_10.0 = -_17;
Goto(bb13)
}
bb13 = {
_34 = _16 + _16;
_30.2 = (_6.2, _6.1, _6.0, _6.3);
_15.fld2 = 167135298464725811681865261587145248684_i128 * (-148320524935656201863971911388576248442_i128);
_33 = _5;
_22 = (_7,);
_17 = _15.fld2 as isize;
_7 = _35.1;
_22.0 = _7;
_15.fld3 = !(-64_i8);
_9 = core::ptr::addr_of_mut!((*_9));
(*_19) = (-8901_i16) as isize;
_22.0 = [(-8193_i16)];
_26 = [1090256861_i32,1589966394_i32,(-728982002_i32)];
_4 = _15.fld0;
_6 = _30.2;
_38 = _10.0 & (*_19);
Goto(bb14)
}
bb14 = {
_13 = _10.0 * _10.0;
_12.1 = [_6.1,_30.2.1,_30.2.1,_30.2.1,_6.1,_6.1,_30.2.1,_6.1];
_35.0 = core::ptr::addr_of!(_15.fld0);
_37 = _20;
_8 = &_42;
(*_19) = _10.0;
_19 = core::ptr::addr_of_mut!(_13);
_24 = [733315128_i32,(-804982560_i32),(-425328523_i32)];
_3 = _5;
_4 = _15.fld0;
(*_9) = 11_i8 + 85_i8;
_5 = _20;
_36 = _33;
_10 = ((*_19),);
_24 = [966899161_i32,(-1318707190_i32),(-1128136353_i32)];
(*_9) = 14602_u16 as i8;
_20 = _36;
(*_19) = _38;
_6.1 = !_30.2.1;
_17 = !_13;
_37 = _33;
_32 = _30.2.1 * _6.1;
Goto(bb15)
}
bb15 = {
_35.1 = _7;
(*_9) = -100_i8;
_34 = !_16;
_11 = core::ptr::addr_of_mut!(_30.1);
_2.1 = 1339475975031239249_u64 & 12190684105931502817_u64;
(*_19) = !_17;
_30.2.1 = !_32;
_38 = (*_19);
_2 = (_23, 6692546224907919774_u64);
_5 = _36;
_12.1 = [_6.1,_30.2.1,_30.2.1,_32,_32,_30.2.1,_6.1,_30.2.1];
_6.3 = [8413_i16,28520_i16,14541_i16,(-27164_i16),31966_i16];
_30.2.1 = _6.1 ^ _32;
_10 = (_13,);
_12.0 = _3;
_10.0 = _38 + _38;
_46 = 663246499_i32 >> _38;
_34 = -_16;
_43 = Move(_9);
_30.0 = _15.fld2 & _15.fld2;
Goto(bb16)
}
bb16 = {
_20 = _33;
_4 = _15.fld0 | _31;
_6.2 = _30.2.2 * _30.2.2;
_15.fld1 = [6707_i16];
_15.fld0 = _4;
_16 = _34 >> _2.0;
_30.3 = [_30.0,_15.fld2,_30.0,_15.fld2,_30.0,_30.0];
_50 = _16;
_50 = _16;
_50 = !_16;
_20 = _5;
_30.2.2 = _6.0 - _6.2;
_33 = _36;
_15.fld3 = _15.fld2 as i8;
_16 = !_50;
_30.2.1 = _32 & _32;
_30.3 = [_30.0,_30.0,_30.0,_30.0,_30.0,_15.fld2];
Goto(bb17)
}
bb17 = {
_2.0 = _30.2.2 as usize;
_2 = (_23, 5918512568075661219_u64);
_10 = ((*_19),);
_30.3 = [_15.fld2,_15.fld2,_30.0,_30.0,_30.0,_30.0];
_30.2.0 = -_21;
_46 = _50 as i32;
_27 = [_46,_46,_46];
Goto(bb18)
}
bb18 = {
_35.0 = core::ptr::addr_of!(_15.fld0);
_22.0 = _35.1;
_51 = (*_19) <= _13;
_24 = [_46,_46,_46];
_15.fld2 = 53386_u16 as i128;
_38 = _17 << _30.0;
Goto(bb19)
}
bb19 = {
_5 = _3;
_9 = core::ptr::addr_of_mut!(_15.fld3);
_2.1 = !13908454396627464872_u64;
_35.1 = _7;
_6 = _30.2;
_39 = _21;
_40 = _13 + (*_19);
Goto(bb20)
}
bb20 = {
_23 = _32 as usize;
_52 = _13;
_42 = Adt29::Variant2 { fld0: _15.fld0 };
Goto(bb21)
}
bb21 = {
_30.2.2 = _2.0 as f32;
_19 = core::ptr::addr_of_mut!(_17);
_33 = _3;
_13 = !_38;
_30.0 = _15.fld2;
(*_9) = (-115_i8);
_13 = !_38;
_57 = [_2.0];
_12.1 = [_6.1,_6.1,_30.2.1,_6.1,_32,_6.1,_32,_6.1];
_44 = &_6.2;
_2.0 = _23;
_30.2 = (_21, _32, _39, _6.3);
_32 = _6.1;
_15.fld2 = _33 as i128;
_57 = [_2.0];
_19 = core::ptr::addr_of_mut!(_28);
_2.0 = _23 | _23;
_31 = Field::<u8>(Variant(_42, 2), 0) + Field::<u8>(Variant(_42, 2), 0);
_17 = _52 >> _38;
_7 = _15.fld1;
_22 = (_7,);
_50 = _2.0 as i64;
Call((*_19) = core::intrinsics::transmute(_13), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_15.fld1 = [(-25496_i16)];
_2 = (_23, 9492274184384712568_u64);
_27 = _24;
_17 = _40;
_28 = -_17;
SetDiscriminant(_42, 3);
(*_19) = !_13;
_2.0 = _23 ^ _23;
_60 = !_38;
_54 = core::ptr::addr_of!(_44);
_2.0 = _23;
(*_54) = &_30.2.0;
_2 = (_23, 15144415428499822675_u64);
(*_19) = -_38;
_28 = -_52;
_10.0 = -_17;
_22.0 = _35.1;
Goto(bb23)
}
bb23 = {
_2.0 = _23;
_33 = _12.0;
_15.fld1 = [27034_i16];
(*_19) = -_60;
_12.1 = [_6.1,_32,_32,_32,_32,_6.1,_32,_32];
_6.3 = _30.2.3;
_57 = [_2.0];
_39 = _21;
_8 = &_59;
_8 = &_42;
match _2.1 {
0 => bb11,
15144415428499822675 => bb25,
_ => bb24
}
}
bb24 = {
_2.1 = 8642066942633737222_u64 * 5111048117307863658_u64;
_6.0 = _6.2 + _6.2;
_2 = (11987054168078649445_usize, 13074142264302349148_u64);
_2 = (3_usize, 2149418893700218143_u64);
_6.1 = 3576522834_u32 ^ 3825406929_u32;
_7 = [(-21382_i16)];
_10.0 = _2.1 as isize;
_5 = _3;
_4 = 47_u8;
_6.1 = 240003547_u32 >> _10.0;
_2.1 = !3795960017324701778_u64;
_3 = _5;
_12.0 = _5;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
_3 = _5;
_2 = (0_usize, 8450311067998312541_u64);
_3 = _12.0;
_3 = _12.0;
_15.fld3 = 3_i8 * (-38_i8);
_13 = !_10.0;
_13 = _10.0;
_12.1 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_6.1];
Call(_9 = fn10(_10.0, _12, _10, _10.0, _12.1, _2.0, _2.1, _5, _6.1), ReturnTo(bb3), UnwindUnreachable())
}
bb25 = {
_30.2.2 = -_6.2;
_36 = _33;
_4 = _51 as u8;
_9 = core::ptr::addr_of_mut!((*_9));
_35.0 = core::ptr::addr_of!(_15.fld0);
match _2.1 {
0 => bb26,
1 => bb27,
2 => bb28,
3 => bb29,
4 => bb30,
15144415428499822675 => bb32,
_ => bb31
}
}
bb26 = {
_35.0 = core::ptr::addr_of!(_15.fld0);
_22.0 = _35.1;
_51 = (*_19) <= _13;
_24 = [_46,_46,_46];
_15.fld2 = 53386_u16 as i128;
_38 = _17 << _30.0;
Goto(bb19)
}
bb27 = {
_2.0 = !5582986626315139140_usize;
_6.2 = -_6.0;
_7 = [27450_i16];
_10 = (_13,);
_10 = (_17,);
_3 = _12.0;
_16 = 3521099574830401004_i64 ^ 4244407627784384607_i64;
Goto(bb5)
}
bb28 = {
_15 = Adt21 { fld0: _4,fld1: _7,fld2: 78868187822126158729082113622677430438_i128,fld3: (-90_i8) };
_16 = 8487278687559642643_i64;
_25 = [_15.fld3,_15.fld3,_15.fld3,_15.fld3,_15.fld3];
_6.1 = _15.fld3 as u32;
_19 = core::ptr::addr_of_mut!(_13);
_17 = (*_19) >> _10.0;
_22.0 = _15.fld1;
_23 = _2.0 - _2.0;
_2 = (_23, 13711076299643779659_u64);
_19 = core::ptr::addr_of_mut!((*_19));
_21 = -_6.2;
_7 = _15.fld1;
_10 = (_17,);
_6.1 = 1965495372_u32;
_13 = _15.fld3 as isize;
_2 = (_23, 16000484349662214703_u64);
_22 = (_7,);
_15.fld3 = -37_i8;
_4 = _23 as u8;
(*_19) = 216520936602995704624496447669971404640_u128 as isize;
Goto(bb6)
}
bb29 = {
_16 = (-12167_i16) as i64;
_31 = _16 as u8;
_28 = (*_19);
_26 = [(-1432225324_i32),1928983932_i32,(-1859644016_i32)];
_20 = _12.0;
_30.2.1 = _6.1;
_30.3 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_6.1 = _30.2.1 % _30.2.1;
_30.2.2 = 32539066_i32 as f32;
_30.2.3 = _6.3;
_35.0 = core::ptr::addr_of!(_15.fld0);
_22 = (_7,);
_31 = _30.2.1 as u8;
_35.1 = [(-28628_i16)];
_10.0 = -_17;
Goto(bb13)
}
bb30 = {
_5 = _3;
_6.3 = [20408_i16,(-2518_i16),17848_i16,(-26548_i16),29132_i16];
_6.2 = _2.1 as f32;
_4 = 195_u8;
_7 = [20599_i16];
_3 = _5;
_6.1 = !2605752422_u32;
_6.2 = (-6285804091547278111_i64) as f32;
_6.2 = _6.0;
_2.1 = 14724433105101609292_u64 ^ 17705250608171702865_u64;
_6.2 = -_6.0;
_6.2 = _2.0 as f32;
_2.0 = 6_usize & 13307886522149624922_usize;
_6.3 = [(-9366_i16),(-13561_i16),(-8941_i16),(-5418_i16),22952_i16];
_6.3 = [(-30826_i16),(-2528_i16),6757_i16,(-1756_i16),7519_i16];
_6.0 = -_6.2;
_7 = [27770_i16];
_2.1 = 2600_u16 as u64;
_3 = _5;
_10.0 = (-1135071035_i32) as isize;
_10.0 = 23478_i16 as isize;
_6.1 = 263882067_u32 & 863544770_u32;
_6.1 = !1492319459_u32;
_2.0 = (-3846753344977284227_i64) as usize;
_7 = [3013_i16];
_6.1 = !3226518580_u32;
_10 = ((-33_isize),);
Goto(bb2)
}
bb31 = {
_34 = _16 + _16;
_30.2 = (_6.2, _6.1, _6.0, _6.3);
_15.fld2 = 167135298464725811681865261587145248684_i128 * (-148320524935656201863971911388576248442_i128);
_33 = _5;
_22 = (_7,);
_17 = _15.fld2 as isize;
_7 = _35.1;
_22.0 = _7;
_15.fld3 = !(-64_i8);
_9 = core::ptr::addr_of_mut!((*_9));
(*_19) = (-8901_i16) as isize;
_22.0 = [(-8193_i16)];
_26 = [1090256861_i32,1589966394_i32,(-728982002_i32)];
_4 = _15.fld0;
_6 = _30.2;
_38 = _10.0 & (*_19);
Goto(bb14)
}
bb32 = {
_57 = [_23];
_15.fld3 = -(-121_i8);
_65 = core::ptr::addr_of_mut!(_15.fld3);
_63.0 = !(*_19);
_36 = _3;
_54 = core::ptr::addr_of!((*_54));
_17 = _15.fld2 as isize;
_2 = (_23, 9720466774310388991_u64);
_55 = _4;
_23 = _2.0;
_8 = &(*_8);
_30.2.3 = _6.3;
place!(Field::<i32>(Variant(_42, 3), 3)) = !_46;
place!(Field::<i32>(Variant(_42, 3), 3)) = _46 ^ _46;
_31 = _4;
_8 = &_42;
_6 = ((*_44), _32, _30.2.2, _30.2.3);
_6.2 = (-29199_i16) as f32;
_40 = _17 << _38;
_12.0 = _33;
_58 = &_22;
_30.2.0 = _6.0;
_15.fld3 = _33 as i8;
match _2.1 {
0 => bb10,
1 => bb5,
9720466774310388991 => bb33,
_ => bb26
}
}
bb33 = {
(*_19) = -_13;
_6.1 = _32 << Field::<i32>(Variant((*_8), 3), 3);
_42 = Adt29::Variant2 { fld0: _4 };
_59 = Adt29::Variant2 { fld0: _55 };
(*_9) = 43_i8 | 37_i8;
_50 = -_16;
_56 = [_23,_2.0,_2.0,_23,_2.0,_23,_23];
Goto(bb34)
}
bb34 = {
_15.fld3 = _31 as i8;
_6.3 = [7252_i16,(-1096_i16),(-11525_i16),4426_i16,(-14717_i16)];
_15.fld1 = [8141_i16];
SetDiscriminant(_59, 1);
_67 = _60;
_8 = &_59;
_27 = _24;
_68 = Move(_35.0);
_9 = core::ptr::addr_of_mut!(_66);
SetDiscriminant(_42, 2);
_62 = _23 as u64;
_36 = _37;
_49 = _46 as i64;
_30.2.3 = [17225_i16,(-20948_i16),(-23611_i16),14784_i16,4797_i16];
_17 = !_67;
_12.1 = [_6.1,_32,_32,_6.1,_32,_30.2.1,_6.1,_30.2.1];
_6.1 = _32 ^ _32;
_40 = _10.0;
Call(_30.2.2 = core::intrinsics::transmute(_30.2.1), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
_31 = _4;
_30.0 = _15.fld2 & _15.fld2;
(*_9) = (*_65) << _28;
place!(Field::<*mut u8>(Variant(_59, 1), 1)) = core::ptr::addr_of_mut!(_31);
_43 = Move(_9);
_71 = core::ptr::addr_of_mut!(_66);
_27 = [_46,_46,_46];
_9 = core::ptr::addr_of_mut!((*_71));
Goto(bb36)
}
bb36 = {
_43 = core::ptr::addr_of_mut!((*_71));
_70 = Adt84 { fld0: _25,fld1: Move(Field::<*mut u8>(Variant(_59, 1), 1)) };
_50 = _16;
_6.3 = [(-5791_i16),(-23754_i16),(-22145_i16),19054_i16,8387_i16];
place!(Field::<*mut u8>(Variant(_59, 1), 1)) = core::ptr::addr_of_mut!(_15.fld0);
_35.1 = [28305_i16];
_6.1 = _32 >> _46;
place!(Field::<u8>(Variant(_42, 2), 0)) = _4;
_30.2 = (_21, _32, _21, _6.3);
_3 = _37;
_30.2.3 = [(-3169_i16),28269_i16,17506_i16,(-29737_i16),13451_i16];
_73 = &_6.2;
_6 = (_21, _30.2.1, _39, _30.2.3);
_35 = (Move(_68), (*_58).0);
_64 = -(*_19);
_3 = _5;
Goto(bb37)
}
bb37 = {
(*_71) = (*_65);
_15.fld0 = !_31;
_30.2.1 = !_6.1;
_2 = (_23, _62);
_30.3 = [_15.fld2,_30.0,_30.0,_30.0,_15.fld2,_15.fld2];
_15 = Adt21 { fld0: Field::<u8>(Variant(_42, 2), 0),fld1: _22.0,fld2: _30.0,fld3: _66 };
place!(Field::<f32>(Variant(_59, 1), 0)) = _46 as f32;
_21 = -Field::<f32>(Variant(_59, 1), 0);
_22 = (_7,);
(*_65) = 31700_i16 as i8;
_27 = [_46,_46,_46];
(*_9) = !(*_65);
_32 = !_6.1;
SetDiscriminant(_42, 3);
_70.fld0 = _25;
_30.2.3 = _6.3;
_37 = _3;
_15.fld3 = (*_71) << _15.fld0;
place!(Field::<*mut *mut u8>(Variant(_42, 3), 1)) = core::ptr::addr_of_mut!(_70.fld1);
_70 = Adt84 { fld0: _25,fld1: Move(Field::<*mut u8>(Variant(_59, 1), 1)) };
Goto(bb38)
}
bb38 = {
_6.0 = -_30.2.0;
(*_54) = &place!(Field::<f32>(Variant(_59, 1), 0));
_71 = core::ptr::addr_of_mut!((*_71));
place!(Field::<*mut *mut u8>(Variant(_59, 1), 3)) = Move(Field::<*mut *mut u8>(Variant(_42, 3), 1));
_34 = _16 + _50;
_36 = _37;
_27 = [_46,_46,_46];
_78 = -26047_i16;
_75 = _40;
_35.1 = _22.0;
(*_54) = &_30.2.0;
place!(Field::<*const u8>(Variant(_42, 3), 2)) = core::ptr::addr_of!(_4);
place!(Field::<*mut *mut *mut u8>(Variant(_42, 3), 0)) = core::ptr::addr_of_mut!(place!(Field::<*mut *mut u8>(Variant(_59, 1), 3)));
_28 = _64 & _40;
(*_43) = _51 as i8;
_30.0 = _15.fld2;
_7 = [_78];
_73 = &place!(Field::<f32>(Variant(_59, 1), 0));
_15.fld0 = !_4;
(*_54) = Move(_73);
place!(Field::<*mut *mut *mut u8>(Variant(_42, 3), 0)) = core::ptr::addr_of_mut!(_72);
(*_9) = (*_65);
(*_71) = (*_65) & _15.fld3;
_5 = _33;
Goto(bb39)
}
bb39 = {
_11 = core::ptr::addr_of_mut!((*_11));
_25 = _70.fld0;
_80 = _55 as f64;
_64 = (*_19);
_40 = _10.0;
_52 = !_10.0;
_32 = !_30.2.1;
_59 = Adt29::Variant0 { fld0: _51,fld1: 81484965266130755304163072371634504148_u128,fld2: _2.1,fld3: _15,fld4: _78,fld5: Move(_70.fld1),fld6: _50 };
_2 = (_23, Field::<u64>(Variant(_59, 0), 2));
_35.0 = Move(Field::<*const u8>(Variant(_42, 3), 2));
_15.fld1 = [_78];
_18 = &_70.fld1;
_61 = core::ptr::addr_of!(_76);
_58 = &_22;
_85 = _52 | _13;
_30.2 = (_21, _32, _6.2, _6.3);
(*_9) = (*_65) * (*_65);
_33 = _5;
_58 = &(*_58);
_3 = _37;
_78 = Field::<i16>(Variant(_59, 0), 4);
_6.1 = _46 as u32;
_10 = _63;
_72 = core::ptr::addr_of_mut!(_70.fld1);
place!(Field::<[i16; 1]>(Variant(_42, 3), 4)) = Field::<Adt21>(Variant(_59, 0), 3).fld1;
Goto(bb40)
}
bb40 = {
_8 = &_59;
_64 = _13;
_2.1 = Field::<u64>(Variant((*_8), 0), 2) & Field::<u64>(Variant(_59, 0), 2);
_15.fld1 = [Field::<i16>(Variant((*_8), 0), 4)];
_2 = (_23, Field::<u64>(Variant(_59, 0), 2));
_58 = &(*_58);
_44 = &_6.0;
_21 = _30.2.0 - (*_44);
_7 = [Field::<i16>(Variant(_59, 0), 4)];
place!(Field::<u64>(Variant(_59, 0), 2)) = _62;
_80 = _30.0 as f64;
place!(Field::<Adt21>(Variant(_59, 0), 3)).fld1 = [_78];
_28 = _23 as isize;
place!(Field::<i16>(Variant(_59, 0), 4)) = !_78;
_30.0 = _34 as i128;
_11 = core::ptr::addr_of_mut!((*_11));
(*_9) = (*_65);
place!(Field::<Adt21>(Variant(_59, 0), 3)).fld1 = [_78];
_30.1 = &_86;
_74 = -_40;
place!(Field::<i32>(Variant(_42, 3), 3)) = _46;
_38 = !_64;
_77 = (*_43) != _15.fld3;
place!(Field::<*const u8>(Variant(_42, 3), 2)) = Move(_35.0);
Goto(bb41)
}
bb41 = {
(*_65) = (*_71) - (*_71);
_68 = core::ptr::addr_of!(_55);
_70.fld1 = core::ptr::addr_of_mut!(place!(Field::<Adt21>(Variant((*_8), 0), 3)).fld0);
_63.0 = Field::<Adt21>(Variant((*_8), 0), 3).fld2 as isize;
_21 = _63.0 as f32;
Call(_26 = core::intrinsics::transmute(_24), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
_34 = Field::<i64>(Variant((*_8), 0), 6) + Field::<i64>(Variant(_59, 0), 6);
place!(Field::<Adt21>(Variant(_59, 0), 3)) = Adt21 { fld0: (*_68),fld1: _22.0,fld2: _30.0,fld3: (*_65) };
_77 = Field::<bool>(Variant((*_8), 0), 0);
_54 = core::ptr::addr_of!(_73);
_57 = [_23];
_4 = (*_68) - _15.fld0;
place!(Field::<Adt21>(Variant(_59, 0), 3)) = Adt21 { fld0: _55,fld1: Field::<[i16; 1]>(Variant(_42, 3), 4),fld2: _30.0,fld3: (*_71) };
_49 = _2.0 as i64;
_30.2.0 = -_6.0;
(*_54) = Move(_44);
_72 = core::ptr::addr_of_mut!(_70.fld1);
_77 = Field::<bool>(Variant((*_8), 0), 0);
Call((*_68) = core::intrinsics::transmute(Field::<Adt21>(Variant(_59, 0), 3).fld0), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_62 = _2.1 * _2.1;
_9 = Move(_71);
_56 = [_2.0,_23,_23,_2.0,_2.0,_2.0,_2.0];
_6.3 = [Field::<i16>(Variant(_59, 0), 4),Field::<i16>(Variant(_59, 0), 4),_78,_78,_78];
_22 = (_7,);
_90 = 50979_u16 >> (*_65);
(*_68) = !_4;
_26 = [_46,Field::<i32>(Variant(_42, 3), 3),_46];
_64 = -_85;
_57 = [_2.0];
_82.0 = _37;
_33 = _36;
Goto(bb44)
}
bb44 = {
(*_43) = (*_65) & (*_65);
_91 = _36;
_52 = -_40;
_17 = _64 + _75;
_58 = &_22;
(*_72) = Move(Field::<*mut u8>(Variant((*_8), 0), 5));
(*_68) = _15.fld0;
_83 = Move(_43);
_93 = _46 as f32;
RET = core::ptr::addr_of!(_90);
place!(Field::<Adt21>(Variant(_59, 0), 3)).fld3 = _66;
_18 = &_94.1;
place!(Field::<i16>(Variant(_59, 0), 4)) = _78;
_19 = core::ptr::addr_of_mut!((*_19));
(*_65) = Field::<Adt21>(Variant(_59, 0), 3).fld3 ^ Field::<Adt21>(Variant(_59, 0), 3).fld3;
_75 = !_10.0;
_16 = _15.fld2 as i64;
(*_54) = &_6.0;
(*_11) = &_86;
_59 = Adt29::Variant2 { fld0: (*_68) };
Goto(bb45)
}
bb45 = {
Call(_97 = dump_var(9_usize, 62_usize, Move(_62), 13_usize, Move(_13), 22_usize, Move(_22), 82_usize, Move(_82)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_97 = dump_var(9_usize, 50_usize, Move(_50), 85_usize, Move(_85), 36_usize, Move(_36), 37_usize, Move(_37)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_97 = dump_var(9_usize, 91_usize, Move(_91), 52_usize, Move(_52), 24_usize, Move(_24), 10_usize, Move(_10)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_97 = dump_var(9_usize, 20_usize, Move(_20), 56_usize, Move(_56), 34_usize, Move(_34), 26_usize, Move(_26)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_97 = dump_var(9_usize, 40_usize, Move(_40), 64_usize, Move(_64), 27_usize, Move(_27), 25_usize, Move(_25)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_97 = dump_var(9_usize, 12_usize, Move(_12), 4_usize, Move(_4), 7_usize, Move(_7), 67_usize, Move(_67)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: (char, [u32; 8]),mut _3: (isize,),mut _4: isize,mut _5: [u32; 8],mut _6: usize,mut _7: u64,mut _8: char,mut _9: u32) -> *mut i8 {
mir! {
type RET = *mut i8;
let _10: isize;
let _11: &'static (f32, u32, f32, [i16; 5]);
let _12: [i128; 6];
let _13: isize;
let _14: isize;
let _15: *const &'static f32;
let _16: [i32; 3];
let _17: *mut isize;
let _18: f32;
let _19: f64;
let _20: f64;
let _21: [u64; 7];
let _22: ([char; 8],);
let _23: i64;
let _24: f64;
let _25: u8;
let _26: u128;
let _27: *const u8;
let _28: &'static *const (u128, char, bool, u8);
let _29: char;
let _30: isize;
let _31: *const &'static f32;
let _32: &'static Adt40;
let _33: ();
let _34: ();
{
_5[_6] = _9 ^ _2.1[_6];
_2.1 = [_5[_6],_5[_6],_9,_5[_6],_5[_6],_5[_6],_5[_6],_5[_6]];
_6 = 2_usize ^ 2613656884814598491_usize;
_7 = 17844262792244627009_u64;
_9 = 2715327389_u32 * 89081167_u32;
Goto(bb1)
}
bb1 = {
_2 = (_8, _5);
_10 = _1 - _4;
_7 = !2694907671067935897_u64;
_4 = _3.0 - _1;
_3.0 = _10 << _9;
_8 = _2.0;
_8 = _2.0;
_7 = 7457015831948512127_u64 >> _3.0;
_5 = _2.1;
_4 = _3.0;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = _3.0 + _1;
_2.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = 628_i16 as isize;
_9 = !879280633_u32;
_6 = 6_usize;
_5 = [_2.1[_6],_2.1[_6],_2.1[_6],_2.1[_6],_9,_2.1[_6],_9,_9];
_8 = _2.0;
_9 = _2.1[_6] & _2.1[_6];
Call(RET = fn11(_4, _1, _3, _2.1, _3.0, _9, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _2.0;
_13 = (-1308198251_i32) as isize;
_1 = _10;
_4 = _3.0 * _1;
_8 = _2.0;
_12 = [(-97431883199208792173229944464401728810_i128),(-116270357785759990619421680006051454181_i128),137123806905121840248695127327382120921_i128,116771846114089130454446403637176836261_i128,101479161432584608057002828477199956160_i128,127211938363371933523376773524525394690_i128];
_12 = [74206406058871798198376839792294894593_i128,95399009158214222610607373218489749822_i128,(-43832822361998367401906393979187166110_i128),41507050938542580864842032572607113282_i128,129614960466374382295250096749302584290_i128,106411883934394368011823653841896677275_i128];
_9 = true as u32;
_9 = 4175344023_u32;
Goto(bb3)
}
bb3 = {
_2 = (_8, _5);
_3 = (_1,);
_3.0 = !_4;
_8 = _2.0;
_14 = -_4;
_14 = _3.0;
_13 = !_3.0;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_2.1 = _5;
_6 = 2609229232647787999_usize << _14;
_13 = 53577375185112840491387835322296463736_i128 as isize;
_3 = (_14,);
_1 = _8 as isize;
_12 = [(-150301201873595347074385258591835930983_i128),48943150419955769697329379916444836966_i128,80893642143115228227045300177954217070_i128,(-109179241059027394985739456607915951086_i128),(-150447977138241557805808938051951622646_i128),(-15166867965772899116562957137517577432_i128)];
_4 = (-18_i8) as isize;
_14 = _3.0 >> _6;
_8 = _2.0;
_6 = 5_usize ^ 1679939621672524642_usize;
_3.0 = (-156914070283998834101864105976852226948_i128) as isize;
_10 = _14;
_2.0 = _8;
_10 = _14;
_8 = _2.0;
_4 = _7 as isize;
_3 = (_14,);
_3 = (_10,);
_6 = !10641420483839645465_usize;
_2 = (_8, _5);
_9 = !789508757_u32;
Goto(bb4)
}
bb4 = {
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = _7 as isize;
_4 = !_10;
_14 = _4 - _3.0;
_9 = !319265461_u32;
_16 = [(-797859859_i32),(-1966500524_i32),(-1485819969_i32)];
_16 = [(-864438677_i32),1346921739_i32,(-1438040092_i32)];
_8 = _2.0;
_3.0 = _14;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = _7 as isize;
_8 = _2.0;
_4 = 153_u8 as isize;
_7 = 3370706590982514198_u64;
_8 = _2.0;
_12 = [95401029413335352097858773625549711835_i128,57764068008488439037029941596798773194_i128,57614620376488905433469238977232555818_i128,(-21856462500744985989741636841462950174_i128),17438734004800841551241529428052650423_i128,(-96843372574284634174022842432524758228_i128)];
_2.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = _14;
_16 = [(-1837683258_i32),2013697003_i32,(-2078681511_i32)];
_2 = (_8, _5);
_7 = !12751011057585610505_u64;
_13 = (-3700671649590560762_i64) as isize;
_8 = _2.0;
_3.0 = _1 ^ _1;
_3.0 = !_10;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17 = core::ptr::addr_of_mut!(_4);
_1 = !(*_17);
Goto(bb5)
}
bb5 = {
_13 = -_3.0;
_2 = (_8, _5);
_6 = 5708787964603040978_usize;
_7 = !13983458394905333119_u64;
_10 = !_14;
_13 = 109330838427398955120901319770342101309_i128 as isize;
_12 = [(-53782372731724296990939877724906641802_i128),(-67842896540091883418087932449050979389_i128),(-77212129126383902184112529147960894355_i128),(-29035588736475357617455383764023747562_i128),(-77080093922256718375647305009692589948_i128),(-90724417156212329893332845550524645485_i128)];
_13 = false as isize;
_9 = 247114114_u32 & 4103630609_u32;
_13 = _1;
_4 = _14;
_1 = _8 as isize;
(*_17) = -_14;
_16 = [(-435036379_i32),715299882_i32,(-58638430_i32)];
_19 = _9 as f64;
_8 = _2.0;
_9 = 520381875_u32;
_13 = _4;
(*_17) = !_14;
_17 = core::ptr::addr_of_mut!(_10);
(*_17) = _13;
_2.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_18 = (-30781119012786403355195710445721781403_i128) as f32;
Goto(bb6)
}
bb6 = {
_4 = _14 << (*_17);
_22.0 = [_2.0,_8,_2.0,_8,_8,_2.0,_2.0,_2.0];
_12 = [62716044713857626604154433783098508768_i128,(-111931310543240163203983338116753975530_i128),37806656997367751400054554950870965128_i128,29599573133802053954991169365402777090_i128,23836974435174281495838461352729958907_i128,140201489212053930861357222245567243817_i128];
_13 = _4;
_9 = 3731366177_u32;
_8 = _2.0;
_21 = [_7,_7,_7,_7,_7,_7,_7];
_21 = [_7,_7,_7,_7,_7,_7,_7];
_24 = -_19;
_12 = [(-119093870144323442464674209105951791191_i128),139921738397954505758176447866478242062_i128,68934792459601686610117834740775887058_i128,(-146042145619137189986809470928319456388_i128),(-143908143262278977012389267460502519650_i128),79041829613773918293397384991873318037_i128];
_25 = 1348763565_i32 as u8;
_14 = (-92597314868290964192849727998628485081_i128) as isize;
_23 = (-7102046560989240712_i64);
_3.0 = _13 | _13;
Goto(bb7)
}
bb7 = {
_21 = [_7,_7,_7,_7,_7,_7,_7];
_26 = !151387800292969469222968442577564276044_u128;
_16 = [(-1301323147_i32),(-1649857222_i32),(-206519543_i32)];
_19 = -_24;
_17 = core::ptr::addr_of_mut!((*_17));
match _9 {
0 => bb8,
3731366177 => bb10,
_ => bb9
}
}
bb8 = {
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = _7 as isize;
_4 = !_10;
_14 = _4 - _3.0;
_9 = !319265461_u32;
_16 = [(-797859859_i32),(-1966500524_i32),(-1485819969_i32)];
_16 = [(-864438677_i32),1346921739_i32,(-1438040092_i32)];
_8 = _2.0;
_3.0 = _14;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = _7 as isize;
_8 = _2.0;
_4 = 153_u8 as isize;
_7 = 3370706590982514198_u64;
_8 = _2.0;
_12 = [95401029413335352097858773625549711835_i128,57764068008488439037029941596798773194_i128,57614620376488905433469238977232555818_i128,(-21856462500744985989741636841462950174_i128),17438734004800841551241529428052650423_i128,(-96843372574284634174022842432524758228_i128)];
_2.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = _14;
_16 = [(-1837683258_i32),2013697003_i32,(-2078681511_i32)];
_2 = (_8, _5);
_7 = !12751011057585610505_u64;
_13 = (-3700671649590560762_i64) as isize;
_8 = _2.0;
_3.0 = _1 ^ _1;
_3.0 = !_10;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17 = core::ptr::addr_of_mut!(_4);
_1 = !(*_17);
Goto(bb5)
}
bb9 = {
_2 = (_8, _5);
_10 = _1 - _4;
_7 = !2694907671067935897_u64;
_4 = _3.0 - _1;
_3.0 = _10 << _9;
_8 = _2.0;
_8 = _2.0;
_7 = 7457015831948512127_u64 >> _3.0;
_5 = _2.1;
_4 = _3.0;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = _3.0 + _1;
_2.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = 628_i16 as isize;
_9 = !879280633_u32;
_6 = 6_usize;
_5 = [_2.1[_6],_2.1[_6],_2.1[_6],_2.1[_6],_9,_2.1[_6],_9,_9];
_8 = _2.0;
_9 = _2.1[_6] & _2.1[_6];
Call(RET = fn11(_4, _1, _3, _2.1, _3.0, _9, _7, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_10 = _13;
_18 = 24066_i16 as f32;
_4 = -(*_17);
_8 = _2.0;
_18 = 55143_u16 as f32;
_3.0 = _10 & (*_17);
(*_17) = _26 as isize;
_9 = _6 as u32;
_14 = -_4;
_20 = -_19;
_8 = _2.0;
_8 = _2.0;
_2 = (_8, _5);
_22.0 = [_2.0,_2.0,_2.0,_8,_8,_8,_2.0,_2.0];
_22.0 = [_2.0,_2.0,_2.0,_8,_8,_8,_8,_8];
_7 = _25 as u64;
match _23 {
0 => bb3,
340282366920938463456272560870778970744 => bb11,
_ => bb7
}
}
bb11 = {
_3.0 = !_4;
_18 = (-163367556541082430271939161188669557833_i128) as f32;
_14 = _3.0 & _13;
_27 = core::ptr::addr_of!(_25);
Goto(bb12)
}
bb12 = {
_13 = _3.0 << _4;
_2 = (_8, _5);
_3 = (_4,);
_4 = !_3.0;
_29 = _8;
(*_17) = -_13;
_14 = _8 as isize;
_4 = _7 as isize;
match _23 {
0 => bb13,
340282366920938463456272560870778970744 => bb15,
_ => bb14
}
}
bb13 = {
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = _7 as isize;
_4 = !_10;
_14 = _4 - _3.0;
_9 = !319265461_u32;
_16 = [(-797859859_i32),(-1966500524_i32),(-1485819969_i32)];
_16 = [(-864438677_i32),1346921739_i32,(-1438040092_i32)];
_8 = _2.0;
_3.0 = _14;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = _7 as isize;
_8 = _2.0;
_4 = 153_u8 as isize;
_7 = 3370706590982514198_u64;
_8 = _2.0;
_12 = [95401029413335352097858773625549711835_i128,57764068008488439037029941596798773194_i128,57614620376488905433469238977232555818_i128,(-21856462500744985989741636841462950174_i128),17438734004800841551241529428052650423_i128,(-96843372574284634174022842432524758228_i128)];
_2.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = _14;
_16 = [(-1837683258_i32),2013697003_i32,(-2078681511_i32)];
_2 = (_8, _5);
_7 = !12751011057585610505_u64;
_13 = (-3700671649590560762_i64) as isize;
_8 = _2.0;
_3.0 = _1 ^ _1;
_3.0 = !_10;
_5 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17 = core::ptr::addr_of_mut!(_4);
_1 = !(*_17);
Goto(bb5)
}
bb14 = {
_10 = _13;
_18 = 24066_i16 as f32;
_4 = -(*_17);
_8 = _2.0;
_18 = 55143_u16 as f32;
_3.0 = _10 & (*_17);
(*_17) = _26 as isize;
_9 = _6 as u32;
_14 = -_4;
_20 = -_19;
_8 = _2.0;
_8 = _2.0;
_2 = (_8, _5);
_22.0 = [_2.0,_2.0,_2.0,_8,_8,_8,_2.0,_2.0];
_22.0 = [_2.0,_2.0,_2.0,_8,_8,_8,_8,_8];
_7 = _25 as u64;
match _23 {
0 => bb3,
340282366920938463456272560870778970744 => bb11,
_ => bb7
}
}
bb15 = {
(*_17) = -_13;
_20 = _24 * _19;
_16 = [(-1602410729_i32),(-894193083_i32),2117775516_i32];
_13 = (*_17);
_2.0 = _8;
_23 = (-7237577758078406901_i64) >> (*_17);
_1 = -(*_17);
_29 = _2.0;
(*_27) = 107_u8;
(*_17) = !_1;
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(10_usize, 16_usize, Move(_16), 1_usize, Move(_1), 21_usize, Move(_21), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(10_usize, 4_usize, Move(_4), 22_usize, Move(_22), 3_usize, Move(_3), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(10_usize, 5_usize, Move(_5), 29_usize, Move(_29), 34_usize, _34, 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: (isize,),mut _4: [u32; 8],mut _5: isize,mut _6: u32,mut _7: u64,mut _8: u32) -> *mut i8 {
mir! {
type RET = *mut i8;
let _9: *mut isize;
let _10: [char; 8];
let _11: *const Adt29;
let _12: [usize; 7];
let _13: u64;
let _14: (char,);
let _15: *mut i16;
let _16: [usize; 1];
let _17: u128;
let _18: (u128, char, bool, u8);
let _19: i32;
let _20: char;
let _21: *mut (u128, char, bool, u8);
let _22: Adt84;
let _23: *const &'static f32;
let _24: isize;
let _25: *const &'static *mut u8;
let _26: [u8; 5];
let _27: [i8; 5];
let _28: bool;
let _29: *mut i8;
let _30: (char, *mut u8);
let _31: [bool; 8];
let _32: (f32, u32, f32, [i16; 5]);
let _33: u128;
let _34: Adt32;
let _35: f64;
let _36: &'static [i16; 1];
let _37: &'static (f32, u32, f32, [i16; 5]);
let _38: [i8; 5];
let _39: isize;
let _40: [u32; 8];
let _41: &'static Adt29;
let _42: *mut (u128, char, bool, u8);
let _43: isize;
let _44: f32;
let _45: char;
let _46: Adt21;
let _47: u128;
let _48: &'static (f32, u32, f32, [i16; 5]);
let _49: isize;
let _50: *const (u128, char, bool, u8);
let _51: *const &'static *mut u8;
let _52: i64;
let _53: *const (u128, char, bool, u8);
let _54: usize;
let _55: ();
let _56: ();
{
_5 = _3.0;
_6 = _8;
_3 = (_2,);
_2 = false as isize;
_4 = [_8,_6,_6,_8,_8,_8,_6,_8];
_4 = [_6,_6,_6,_8,_8,_8,_6,_6];
_6 = !_8;
_5 = 29135_u16 as isize;
_7 = 63_u8 as u64;
_7 = !12711468024319865712_u64;
_3 = (_1,);
_6 = (-151736854972709567052401698576613705208_i128) as u32;
_4 = [_8,_8,_6,_6,_8,_8,_8,_8];
_7 = 12478471136048310518_u64;
_9 = core::ptr::addr_of_mut!(_1);
(*_9) = 7_usize as isize;
(*_9) = _2;
(*_9) = (-56_i8) as isize;
_5 = (*_9) & _2;
_2 = -_5;
(*_9) = _5 | _2;
_9 = core::ptr::addr_of_mut!(_2);
_1 = 15788_u16 as isize;
_2 = 7_usize as isize;
(*_9) = _1 * _5;
(*_9) = -_1;
_4 = [_8,_8,_8,_8,_8,_6,_8,_8];
_6 = (-126_i8) as u32;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
12478471136048310518 => bb6,
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
_1 = 70_u8 as isize;
_3.0 = _5;
_5 = 116_i8 as isize;
_4 = [_8,_8,_8,_8,_8,_6,_8,_8];
_3 = (_5,);
_3.0 = -_5;
(*_9) = -_3.0;
_9 = core::ptr::addr_of_mut!(_1);
_7 = 6102639390057966952_u64 | 11436006893557461933_u64;
_2 = (*_9);
(*_9) = 966049982959778333808021894616209930_i128 as isize;
(*_9) = false as isize;
_1 = -_5;
_5 = _3.0;
_7 = 3127458936686253746_u64 << _1;
_10 = ['\u{be0b2}','\u{8a07f}','\u{28dad}','\u{fcdb3}','\u{1d1f2}','\u{71b6e}','\u{36ad8}','\u{e3587}'];
Call((*_9) = fn12(_8, _8, _4, _4, _10, _7, _10, _2, _8, _10, _10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3.0 = 743414912_i32 as isize;
(*_9) = (-12708_i16) as isize;
_7 = 16281927124804302048_u64;
_12 = [11868916131836535385_usize,15813374732252615991_usize,2_usize,2_usize,5_usize,13588548493242420494_usize,7_usize];
_9 = core::ptr::addr_of_mut!((*_9));
_9 = core::ptr::addr_of_mut!(_3.0);
_4 = [_6,_8,_8,_6,_8,_8,_8,_8];
(*_9) = _5 >> _7;
_13 = _7 & _7;
(*_9) = _5;
match _7 {
0 => bb5,
1 => bb2,
2 => bb6,
3 => bb4,
16281927124804302048 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_1 = !_5;
(*_9) = _2;
_5 = -(*_9);
_1 = '\u{105940}' as isize;
_2 = (*_9) - (*_9);
_3.0 = !_2;
_9 = core::ptr::addr_of_mut!(_2);
_7 = _13 * _13;
_14 = ('\u{d8f1f}',);
_13 = 1384974621_i32 as u64;
_14.0 = '\u{d5fdd}';
_6 = _8;
_14 = ('\u{c0f31}',);
_5 = _1 - _1;
_6 = !_8;
_6 = _8 - _8;
_5 = 10830966447933418615_usize as isize;
Goto(bb10)
}
bb10 = {
_2 = !_3.0;
_9 = core::ptr::addr_of_mut!(_2);
_6 = 226_u8 as u32;
_3 = (_2,);
_17 = 257416160115718182821450456349317038102_u128 << _8;
_12 = [5_usize,4_usize,0_usize,1_usize,5414068017658976314_usize,10334135048265736203_usize,16875222351925912320_usize];
_9 = core::ptr::addr_of_mut!(_2);
_13 = !_7;
_4 = [_8,_8,_8,_8,_6,_8,_6,_8];
_17 = 300895819935210683330095076197752087857_u128;
match _17 {
300895819935210683330095076197752087857 => bb11,
_ => bb6
}
}
bb11 = {
_1 = !(*_9);
_18.2 = !true;
_12 = [3_usize,13219050636250016871_usize,8643851524538190721_usize,16470341520702080810_usize,1_usize,15985977539393140367_usize,5_usize];
_18.0 = 3_usize as u128;
_19 = -184733551_i32;
_12 = [12896254220745150346_usize,0_usize,8526185592005766648_usize,0_usize,3379690608087719455_usize,6_usize,12243838856424280323_usize];
_18.2 = true;
_20 = _14.0;
_4 = [_8,_8,_6,_8,_8,_8,_8,_8];
_18.1 = _14.0;
Goto(bb12)
}
bb12 = {
_16 = [0_usize];
_18.0 = !_17;
_3 = ((*_9),);
_21 = core::ptr::addr_of_mut!(_18);
_1 = 117_u8 as isize;
_18.1 = _14.0;
_5 = (*_9);
_16 = [17094295769888220449_usize];
_18.1 = _20;
_24 = !_2;
_17 = !(*_21).0;
_13 = 54_u8 as u64;
_12 = [7876408851603093981_usize,3_usize,4028838250095157489_usize,7917799038459437431_usize,2804956267050184587_usize,6125233283197693620_usize,3_usize];
_19 = -(-335739437_i32);
_18.1 = _20;
_14.0 = _20;
_26 = [12_u8,35_u8,57_u8,214_u8,116_u8];
(*_21).3 = (-998904709896431493_i64) as u8;
_12 = [4_usize,2794256470607340684_usize,0_usize,4805681369472054795_usize,18163357009824824959_usize,0_usize,6_usize];
Goto(bb13)
}
bb13 = {
(*_21).0 = _17;
_24 = (*_9);
_28 = !(*_21).2;
(*_21).1 = _14.0;
(*_21) = (_17, _14.0, _28, 139_u8);
_16 = [1_usize];
_13 = !_7;
_28 = (*_21).0 >= _18.0;
_17 = _18.0;
_3.0 = (*_9) & _1;
_28 = !(*_21).2;
(*_21).3 = 22_u8;
_32.2 = (-16_i8) as f32;
Goto(bb14)
}
bb14 = {
(*_21) = (_17, _14.0, _28, 79_u8);
_14 = (_20,);
_17 = (*_21).0 - _18.0;
(*_21).2 = !_28;
_35 = 6_usize as f64;
Goto(bb15)
}
bb15 = {
(*_9) = -_24;
_18 = (_17, _20, _28, 44_u8);
_22.fld1 = core::ptr::addr_of_mut!((*_21).3);
_30.0 = _18.1;
_22.fld0 = [(-127_i8),(-33_i8),(-112_i8),95_i8,22_i8];
_2 = _3.0;
_32.1 = _6;
Goto(bb16)
}
bb16 = {
_14 = (_18.1,);
_24 = -(*_9);
_22.fld1 = core::ptr::addr_of_mut!(_18.3);
(*_21).3 = 35_u8;
(*_21).0 = !_17;
_35 = 767688499436742325_i64 as f64;
match _18.3 {
0 => bb7,
35 => bb17,
_ => bb3
}
}
bb17 = {
_32.3 = [6038_i16,(-17629_i16),13538_i16,25187_i16,(-27876_i16)];
_26 = [_18.3,(*_21).3,(*_21).3,_18.3,(*_21).3];
match (*_21).3 {
0 => bb6,
1 => bb13,
2 => bb16,
3 => bb12,
4 => bb9,
5 => bb18,
35 => bb20,
_ => bb19
}
}
bb18 = {
Return()
}
bb19 = {
(*_21).0 = _17;
_24 = (*_9);
_28 = !(*_21).2;
(*_21).1 = _14.0;
(*_21) = (_17, _14.0, _28, 139_u8);
_16 = [1_usize];
_13 = !_7;
_28 = (*_21).0 >= _18.0;
_17 = _18.0;
_3.0 = (*_9) & _1;
_28 = !(*_21).2;
(*_21).3 = 22_u8;
_32.2 = (-16_i8) as f32;
Goto(bb14)
}
bb20 = {
_14 = (_18.1,);
_33 = !_17;
_37 = &_32;
(*_9) = -_3.0;
_4 = [_6,_8,_6,_32.1,_8,_8,_8,_6];
_27 = [(-23_i8),4_i8,87_i8,12_i8,(-90_i8)];
_12 = [2_usize,13825683354075490477_usize,7660429105187274452_usize,5_usize,9740703416243367456_usize,7_usize,5_usize];
_8 = (*_37).1;
_16 = [0_usize];
_18.3 = !223_u8;
_28 = !(*_21).2;
_40 = _4;
_16 = [2_usize];
_32.3 = [(-17344_i16),(-28071_i16),18064_i16,17655_i16,29967_i16];
_30 = (_20, Move(_22.fld1));
Goto(bb21)
}
bb21 = {
_30.0 = _18.1;
_22.fld0 = [115_i8,(-34_i8),(-116_i8),(-37_i8),(-63_i8)];
_32.2 = (*_21).0 as f32;
_34 = Adt32::Variant2 { fld0: _7 };
(*_21) = (_17, _20, _28, 93_u8);
_35 = (-9217_i16) as f64;
(*_21) = (_33, _20, _28, 232_u8);
match (*_21).3 {
0 => bb5,
1 => bb17,
232 => bb23,
_ => bb22
}
}
bb22 = {
(*_21).0 = _17;
_24 = (*_9);
_28 = !(*_21).2;
(*_21).1 = _14.0;
(*_21) = (_17, _14.0, _28, 139_u8);
_16 = [1_usize];
_13 = !_7;
_28 = (*_21).0 >= _18.0;
_17 = _18.0;
_3.0 = (*_9) & _1;
_28 = !(*_21).2;
(*_21).3 = 22_u8;
_32.2 = (-16_i8) as f32;
Goto(bb14)
}
bb23 = {
_22.fld1 = Move(_30.1);
match _18.3 {
0 => bb24,
1 => bb25,
232 => bb27,
_ => bb26
}
}
bb24 = {
Return()
}
bb25 = {
_2 = !_3.0;
_9 = core::ptr::addr_of_mut!(_2);
_6 = 226_u8 as u32;
_3 = (_2,);
_17 = 257416160115718182821450456349317038102_u128 << _8;
_12 = [5_usize,4_usize,0_usize,1_usize,5414068017658976314_usize,10334135048265736203_usize,16875222351925912320_usize];
_9 = core::ptr::addr_of_mut!(_2);
_13 = !_7;
_4 = [_8,_8,_8,_8,_6,_8,_6,_8];
_17 = 300895819935210683330095076197752087857_u128;
match _17 {
300895819935210683330095076197752087857 => bb11,
_ => bb6
}
}
bb26 = {
_1 = !(*_9);
_18.2 = !true;
_12 = [3_usize,13219050636250016871_usize,8643851524538190721_usize,16470341520702080810_usize,1_usize,15985977539393140367_usize,5_usize];
_18.0 = 3_usize as u128;
_19 = -184733551_i32;
_12 = [12896254220745150346_usize,0_usize,8526185592005766648_usize,0_usize,3379690608087719455_usize,6_usize,12243838856424280323_usize];
_18.2 = true;
_20 = _14.0;
_4 = [_8,_8,_6,_8,_8,_8,_8,_8];
_18.1 = _14.0;
Goto(bb12)
}
bb27 = {
_21 = core::ptr::addr_of_mut!(_18);
(*_21).3 = !249_u8;
(*_21).2 = _28;
_46.fld3 = -16_i8;
_3 = ((*_9),);
_35 = _3.0 as f64;
_30 = (_14.0, Move(_22.fld1));
_26 = [(*_21).3,_18.3,(*_21).3,_18.3,(*_21).3];
_17 = (*_21).0 << (*_9);
_22.fld1 = core::ptr::addr_of_mut!((*_21).3);
_40 = _4;
_36 = &_46.fld1;
SetDiscriminant(_34, 2);
_45 = _30.0;
(*_21) = (_33, _30.0, _28, 5_u8);
_26 = [(*_21).3,(*_21).3,(*_21).3,(*_21).3,(*_21).3];
(*_21) = (_17, _20, _28, 195_u8);
_33 = _18.0 | _17;
_42 = core::ptr::addr_of_mut!((*_21));
_33 = _13 as u128;
(*_21).1 = _20;
match (*_21).3 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
195 => bb35,
_ => bb34
}
}
bb28 = {
_1 = !(*_9);
_18.2 = !true;
_12 = [3_usize,13219050636250016871_usize,8643851524538190721_usize,16470341520702080810_usize,1_usize,15985977539393140367_usize,5_usize];
_18.0 = 3_usize as u128;
_19 = -184733551_i32;
_12 = [12896254220745150346_usize,0_usize,8526185592005766648_usize,0_usize,3379690608087719455_usize,6_usize,12243838856424280323_usize];
_18.2 = true;
_20 = _14.0;
_4 = [_8,_8,_6,_8,_8,_8,_8,_8];
_18.1 = _14.0;
Goto(bb12)
}
bb29 = {
Return()
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
_32.3 = [6038_i16,(-17629_i16),13538_i16,25187_i16,(-27876_i16)];
_26 = [_18.3,(*_21).3,(*_21).3,_18.3,(*_21).3];
match (*_21).3 {
0 => bb6,
1 => bb13,
2 => bb16,
3 => bb12,
4 => bb9,
5 => bb18,
35 => bb20,
_ => bb19
}
}
bb34 = {
(*_21).0 = _17;
_24 = (*_9);
_28 = !(*_21).2;
(*_21).1 = _14.0;
(*_21) = (_17, _14.0, _28, 139_u8);
_16 = [1_usize];
_13 = !_7;
_28 = (*_21).0 >= _18.0;
_17 = _18.0;
_3.0 = (*_9) & _1;
_28 = !(*_21).2;
(*_21).3 = 22_u8;
_32.2 = (-16_i8) as f32;
Goto(bb14)
}
bb35 = {
_30.1 = core::ptr::addr_of_mut!(_18.3);
place!(Field::<u64>(Variant(_34, 2), 0)) = _2 as u64;
_26 = [(*_21).3,(*_42).3,(*_42).3,(*_42).3,(*_21).3];
_16 = [653563947398580024_usize];
_46.fld0 = (*_37).1 as u8;
SetDiscriminant(_34, 3);
RET = core::ptr::addr_of_mut!(_46.fld3);
_45 = (*_21).1;
(*_21).1 = _45;
(*_21) = (_17, _14.0, _28, _46.fld0);
_48 = &_32;
_34 = Adt32::Variant0 { fld0: (*_48).2,fld1: _14.0,fld2: 7_usize };
(*_42).3 = Field::<char>(Variant(_34, 0), 1) as u8;
_33 = (*_48).1 as u128;
_46.fld1 = [5208_i16];
(*_42).2 = _28;
_49 = (*_42).0 as isize;
(*_21) = (_17, _45, _28, _46.fld0);
_13 = !_7;
(*_42) = (_17, Field::<char>(Variant(_34, 0), 1), _28, _46.fld0);
_3.0 = -_2;
_31 = [_18.2,(*_42).2,(*_42).2,(*_21).2,(*_21).2,(*_21).2,(*_21).2,(*_21).2];
_19 = (-234960192_i32) ^ 738815083_i32;
_14 = ((*_42).1,);
_6 = _8;
_40 = _4;
Goto(bb36)
}
bb36 = {
Call(_55 = dump_var(11_usize, 10_usize, Move(_10), 13_usize, Move(_13), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_55 = dump_var(11_usize, 27_usize, Move(_27), 5_usize, Move(_5), 4_usize, Move(_4), 16_usize, Move(_16)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_55 = dump_var(11_usize, 3_usize, Move(_3), 45_usize, Move(_45), 40_usize, Move(_40), 7_usize, Move(_7)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_55 = dump_var(11_usize, 6_usize, Move(_6), 56_usize, _56, 56_usize, _56, 56_usize, _56), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u32,mut _2: u32,mut _3: [u32; 8],mut _4: [u32; 8],mut _5: [char; 8],mut _6: u64,mut _7: [char; 8],mut _8: isize,mut _9: u32,mut _10: [char; 8],mut _11: [char; 8]) -> isize {
mir! {
type RET = isize;
let _12: [u8; 5];
let _13: [i128; 6];
let _14: isize;
let _15: [usize; 1];
let _16: [i32; 3];
let _17: Adt83;
let _18: Adt32;
let _19: f32;
let _20: bool;
let _21: char;
let _22: [usize; 7];
let _23: &'static u8;
let _24: [u64; 3];
let _25: (usize, u64);
let _26: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _27: u128;
let _28: [i32; 3];
let _29: &'static f32;
let _30: &'static f32;
let _31: isize;
let _32: usize;
let _33: &'static usize;
let _34: &'static *mut u8;
let _35: [char; 8];
let _36: [i16; 1];
let _37: u8;
let _38: u8;
let _39: ([char; 8],);
let _40: *mut &'static u128;
let _41: isize;
let _42: [bool; 8];
let _43: [char; 8];
let _44: bool;
let _45: isize;
let _46: Adt29;
let _47: &'static Adt29;
let _48: (char, [u32; 8]);
let _49: bool;
let _50: ();
let _51: ();
{
_7 = ['\u{7e158}','\u{b0d4d}','\u{5c7d7}','\u{28ea9}','\u{20e06}','\u{10e87e}','\u{2b877}','\u{ec8e0}'];
_2 = _1 ^ _9;
RET = !_8;
RET = _8;
_7 = ['\u{7a711}','\u{af596}','\u{8f11e}','\u{a6b7d}','\u{eab91}','\u{945e6}','\u{72ca3}','\u{b2265}'];
_12 = [21_u8,157_u8,113_u8,202_u8,130_u8];
_12 = [219_u8,80_u8,5_u8,131_u8,20_u8];
_7 = ['\u{aebeb}','\u{cc630}','\u{78992}','\u{84fdb}','\u{d0ff3}','\u{31291}','\u{29e91}','\u{b2c77}'];
_4 = _3;
Goto(bb1)
}
bb1 = {
_1 = 170900402885785780500783388947699681636_u128 as u32;
_1 = !_2;
_14 = _6 as isize;
_3 = [_9,_1,_1,_1,_2,_2,_1,_1];
_10 = ['\u{80d1f}','\u{e677c}','\u{cd2e2}','\u{b574a}','\u{ea09b}','\u{1d6ba}','\u{de996}','\u{5800d}'];
_2 = !_9;
RET = _14;
_8 = -RET;
_13 = [(-9646670245742554943329512639726491794_i128),(-116940112161649352786189999627403523652_i128),121490016948030265744517891969381989517_i128,116847678592691288338370725469373615393_i128,113708320432602112779047127095217901220_i128,(-54316884762688046814689928581314662777_i128)];
RET = (-1129957852_i32) as isize;
Goto(bb2)
}
bb2 = {
_3 = _4;
_16 = [1887881835_i32,1200700438_i32,(-1234615781_i32)];
_2 = !_9;
_2 = _9;
RET = (-1123250920_i32) as isize;
_20 = false;
_19 = (-809486855_i32) as f32;
_6 = 15130170057710755955_u64;
_22 = [12596961159287343666_usize,6385292955700874529_usize,4682429995682449107_usize,7_usize,1_usize,5323520712232717439_usize,9510927524531991954_usize];
_8 = !_14;
_22 = [17236394378888376728_usize,1_usize,3617489795139194466_usize,6_usize,12504274474370771502_usize,6_usize,11859649023143218880_usize];
_4 = [_1,_1,_1,_1,_9,_1,_2,_9];
_24 = [_6,_6,_6];
_13 = [84461200696348644882375088960854783519_i128,(-154705175150894718081448150910125302823_i128),(-99038162951651193900000919258172986116_i128),127283814821957806956804860527128483123_i128,(-81321668378058622040600713474809918602_i128),136546955154341515971540599502071720140_i128];
RET = _8 & _14;
_7 = _11;
_13 = [14527353619399191373508852871728760733_i128,(-31365398603900433433184091683212047818_i128),(-40372931320621191717941855476016180802_i128),102614556159031578053989002951294921171_i128,(-103963621549342478528529349824942237344_i128),122966052872606995857253088075836156517_i128];
_2 = 118475198_i32 as u32;
_14 = -_8;
Goto(bb3)
}
bb3 = {
_20 = !true;
_18 = Adt32::Variant2 { fld0: _6 };
_25.0 = 49_i8 as usize;
_10 = _11;
_24 = [Field::<u64>(Variant(_18, 2), 0),Field::<u64>(Variant(_18, 2), 0),_6];
_11 = ['\u{425c8}','\u{dc112}','\u{7595f}','\u{f6690}','\u{109b1a}','\u{6f47b}','\u{7716e}','\u{36114}'];
_24 = [Field::<u64>(Variant(_18, 2), 0),_6,_6];
_21 = '\u{8e0b8}';
_1 = _9 << _9;
_4 = [_2,_1,_1,_1,_1,_9,_9,_1];
_19 = _6 as f32;
_3 = [_1,_1,_1,_1,_1,_9,_1,_1];
_15 = [_25.0];
_4 = [_9,_1,_1,_1,_1,_9,_1,_1];
_15 = [_25.0];
_11 = [_21,_21,_21,_21,_21,_21,_21,_21];
SetDiscriminant(_18, 1);
_8 = _19 as isize;
match _6 {
0 => bb1,
1 => bb4,
2 => bb5,
15130170057710755955 => bb7,
_ => bb6
}
}
bb4 = {
_3 = _4;
_16 = [1887881835_i32,1200700438_i32,(-1234615781_i32)];
_2 = !_9;
_2 = _9;
RET = (-1123250920_i32) as isize;
_20 = false;
_19 = (-809486855_i32) as f32;
_6 = 15130170057710755955_u64;
_22 = [12596961159287343666_usize,6385292955700874529_usize,4682429995682449107_usize,7_usize,1_usize,5323520712232717439_usize,9510927524531991954_usize];
_8 = !_14;
_22 = [17236394378888376728_usize,1_usize,3617489795139194466_usize,6_usize,12504274474370771502_usize,6_usize,11859649023143218880_usize];
_4 = [_1,_1,_1,_1,_9,_1,_2,_9];
_24 = [_6,_6,_6];
_13 = [84461200696348644882375088960854783519_i128,(-154705175150894718081448150910125302823_i128),(-99038162951651193900000919258172986116_i128),127283814821957806956804860527128483123_i128,(-81321668378058622040600713474809918602_i128),136546955154341515971540599502071720140_i128];
RET = _8 & _14;
_7 = _11;
_13 = [14527353619399191373508852871728760733_i128,(-31365398603900433433184091683212047818_i128),(-40372931320621191717941855476016180802_i128),102614556159031578053989002951294921171_i128,(-103963621549342478528529349824942237344_i128),122966052872606995857253088075836156517_i128];
_2 = 118475198_i32 as u32;
_14 = -_8;
Goto(bb3)
}
bb5 = {
_1 = 170900402885785780500783388947699681636_u128 as u32;
_1 = !_2;
_14 = _6 as isize;
_3 = [_9,_1,_1,_1,_2,_2,_1,_1];
_10 = ['\u{80d1f}','\u{e677c}','\u{cd2e2}','\u{b574a}','\u{ea09b}','\u{1d6ba}','\u{de996}','\u{5800d}'];
_2 = !_9;
RET = _14;
_8 = -RET;
_13 = [(-9646670245742554943329512639726491794_i128),(-116940112161649352786189999627403523652_i128),121490016948030265744517891969381989517_i128,116847678592691288338370725469373615393_i128,113708320432602112779047127095217901220_i128,(-54316884762688046814689928581314662777_i128)];
RET = (-1129957852_i32) as isize;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_28 = [(-639364270_i32),643159228_i32,1935130352_i32];
place!(Field::<i32>(Variant(_18, 1), 1)) = 1284790451_i32 << _1;
_6 = !13657287477943921911_u64;
place!(Field::<u16>(Variant(_18, 1), 4)) = 16931_u16 + 63040_u16;
_27 = 36572356068176929023032446993952281450_u128 & 246441521232778933834927788228397046651_u128;
_6 = 9998245021806776029_u64 * 16158469256663483954_u64;
_30 = &_19;
RET = _14;
_4 = [_1,_9,_1,_1,_1,_9,_1,_2];
place!(Field::<u128>(Variant(_18, 1), 2)) = _27;
_13 = [162486157063387123556516256544960519272_i128,96423042444747584555233192943065733216_i128,71896905991874835320378977407496903092_i128,(-96844050061429761001977903152389949699_i128),(-66010014193324327034156546455388203648_i128),13968577119549523094439235548345512380_i128];
place!(Field::<bool>(Variant(_18, 1), 0)) = _20;
_11 = [_21,_21,_21,_21,_21,_21,_21,_21];
_15 = [_25.0];
place!(Field::<u128>(Variant(_18, 1), 2)) = _27 & _27;
_13 = [(-167137220158338738086375242874047501205_i128),79844500011084190933174774331894766938_i128,(-147617997014380143168953497838231168469_i128),67064010770398069666299893815130577451_i128,(-25724326920363969106119293403782945549_i128),(-79116061321998464996293195689805889566_i128)];
_25 = (8109974095779137013_usize, _6);
_13 = [92852421901055622296044064346242019377_i128,(-8008167867347767211632599511026305384_i128),(-153773102760947275390450258569168962504_i128),152386119419745887470536683221307049020_i128,113267073394005578934680127617401595344_i128,(-145567346829758812586534899141416097665_i128)];
_15 = [_25.0];
_1 = Field::<u128>(Variant(_18, 1), 2) as u32;
_16 = [Field::<i32>(Variant(_18, 1), 1),Field::<i32>(Variant(_18, 1), 1),Field::<i32>(Variant(_18, 1), 1)];
Call(_9 = fn13(Move(_30), _4, _10, _28, _10, Field::<i32>(Variant(_18, 1), 1)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_29 = &_19;
_33 = &_25.0;
_25.0 = !2_usize;
_25.1 = _6 << RET;
_32 = !_25.0;
_5 = [_21,_21,_21,_21,_21,_21,_21,_21];
_13 = [(-150682137999757672628319362646041518825_i128),(-110126764679318793158488717108642121741_i128),22307302999201406051385213358225492144_i128,33887695279461155764468017158223803790_i128,(-81564648124344254973760341196168496978_i128),(-134047652214347378443053179190532510143_i128)];
_16 = _28;
_7 = [_21,_21,_21,_21,_21,_21,_21,_21];
RET = _14 + _8;
_19 = 3963_i16 as f32;
_25.1 = _6;
_30 = &_19;
_27 = Field::<u128>(Variant(_18, 1), 2) << Field::<u128>(Variant(_18, 1), 2);
_27 = !Field::<u128>(Variant(_18, 1), 2);
_13 = [77056829951734645330217050717930217794_i128,(-6391293803326772452879153927810538940_i128),(-47447929032203361426090430959865148376_i128),103486098251703382203192242798736259429_i128,76425252556658738706872617049782788217_i128,(-157692848589612489368572157129734082987_i128)];
_31 = RET;
_25.0 = (*_30) as usize;
_15 = [_32];
Goto(bb9)
}
bb9 = {
_31 = Field::<i32>(Variant(_18, 1), 1) as isize;
_27 = Field::<u128>(Variant(_18, 1), 2) & Field::<u128>(Variant(_18, 1), 2);
_8 = (-6366592377691216466441010941313038457_i128) as isize;
Call(RET = core::intrinsics::transmute(_31), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Goto(bb11)
}
bb11 = {
_21 = '\u{8b319}';
_33 = &_32;
_11 = [_21,_21,_21,_21,_21,_21,_21,_21];
_36 = [(-22233_i16)];
place!(Field::<*const u8>(Variant(_18, 1), 3)) = core::ptr::addr_of!(_37);
_36 = [24908_i16];
_25.1 = !_6;
_12 = [182_u8,112_u8,4_u8,59_u8,49_u8];
place!(Field::<i32>(Variant(_18, 1), 1)) = (-19766_i16) as i32;
_32 = _25.0;
Call(_37 = core::intrinsics::transmute(_20), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_33 = &_32;
_19 = _2 as f32;
RET = _14 ^ _14;
_27 = Field::<u128>(Variant(_18, 1), 2);
_38 = _37 | _37;
_10 = _5;
place!(Field::<i32>(Variant(_18, 1), 1)) = 1805478032_i32 & 1263666499_i32;
_27 = Field::<u128>(Variant(_18, 1), 2);
_10 = [_21,_21,_21,_21,_21,_21,_21,_21];
_39.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
place!(Field::<u16>(Variant(_18, 1), 4)) = 13015_u16;
_35 = [_21,_21,_21,_21,_21,_21,_21,_21];
_25.0 = (*_33);
_13 = [167004878686071560164899683160037779258_i128,155299357166433234773191636554375468195_i128,75246875710028062605499221310812582788_i128,(-17541109676599347393908476548719418886_i128),122893068106032344053180279456960327074_i128,92283708115207863240584793444592814209_i128];
_4 = [_1,_1,_1,_2,_1,_9,_1,_1];
_25.0 = _19 as usize;
_23 = &_37;
_28 = [Field::<i32>(Variant(_18, 1), 1),Field::<i32>(Variant(_18, 1), 1),Field::<i32>(Variant(_18, 1), 1)];
_36 = [(-4943_i16)];
_20 = Field::<bool>(Variant(_18, 1), 0) | Field::<bool>(Variant(_18, 1), 0);
_25.0 = (*_33) + _32;
place!(Field::<bool>(Variant(_18, 1), 0)) = !_20;
_2 = _9 >> _25.1;
_7 = [_21,_21,_21,_21,_21,_21,_21,_21];
match Field::<u16>(Variant(_18, 1), 4) {
13015 => bb14,
_ => bb13
}
}
bb13 = {
_21 = '\u{8b319}';
_33 = &_32;
_11 = [_21,_21,_21,_21,_21,_21,_21,_21];
_36 = [(-22233_i16)];
place!(Field::<*const u8>(Variant(_18, 1), 3)) = core::ptr::addr_of!(_37);
_36 = [24908_i16];
_25.1 = !_6;
_12 = [182_u8,112_u8,4_u8,59_u8,49_u8];
place!(Field::<i32>(Variant(_18, 1), 1)) = (-19766_i16) as i32;
_32 = _25.0;
Call(_37 = core::intrinsics::transmute(_20), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_10 = [_21,_21,_21,_21,_21,_21,_21,_21];
_31 = RET;
_14 = 13_i8 as isize;
_21 = '\u{1e184}';
SetDiscriminant(_18, 2);
_7 = _11;
_37 = !_38;
_14 = _31;
_27 = 35974590924272969033825944647257641471_u128 | 35127991215147274787938721560503782789_u128;
place!(Field::<u64>(Variant(_18, 2), 0)) = _19 as u64;
_32 = _25.0 & _25.0;
_7 = [_21,_21,_21,_21,_21,_21,_21,_21];
_12 = [_38,_38,_37,_38,_38];
_42 = [_20,_20,_20,_20,_20,_20,_20,_20];
_45 = !_14;
_25.1 = _6;
_33 = &_25.0;
_19 = _38 as f32;
_23 = &_38;
_14 = _31;
_43 = [_21,_21,_21,_21,_21,_21,_21,_21];
_19 = _27 as f32;
_35 = [_21,_21,_21,_21,_21,_21,_21,_21];
_12 = [(*_23),_38,_37,(*_23),(*_23)];
_7 = _5;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(12_usize, 22_usize, Move(_22), 39_usize, Move(_39), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(12_usize, 38_usize, Move(_38), 42_usize, Move(_42), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(12_usize, 10_usize, Move(_10), 12_usize, Move(_12), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(12_usize, 32_usize, Move(_32), 9_usize, Move(_9), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: &'static f32,mut _2: [u32; 8],mut _3: [char; 8],mut _4: [i32; 3],mut _5: [char; 8],mut _6: i32) -> u32 {
mir! {
type RET = u32;
let _7: f64;
let _8: usize;
let _9: isize;
let _10: [i16; 5];
let _11: u32;
let _12: u128;
let _13: &'static [i16; 1];
let _14: (char,);
let _15: *mut *mut *mut u8;
let _16: u128;
let _17: u64;
let _18: *const u16;
let _19: *const (u128, char, bool, u8);
let _20: char;
let _21: [usize; 1];
let _22: &'static i128;
let _23: [u64; 3];
let _24: isize;
let _25: bool;
let _26: [i16; 5];
let _27: char;
let _28: &'static *mut u8;
let _29: f32;
let _30: [i32; 3];
let _31: f64;
let _32: Adt33;
let _33: u16;
let _34: Adt32;
let _35: [u32; 8];
let _36: isize;
let _37: (char,);
let _38: i16;
let _39: &'static usize;
let _40: char;
let _41: ();
let _42: ();
{
RET = 2933459222_u32 ^ 4047437755_u32;
_8 = 4420937810434249988_usize;
_7 = 67_isize as f64;
_4 = [_6,_6,_6];
_7 = RET as f64;
_8 = '\u{3097}' as usize;
_8 = !7_usize;
_8 = 2_usize + 6_usize;
RET = 620304952_u32 & 3168970442_u32;
RET = !950048716_u32;
_10 = [68_i16,26333_i16,(-10783_i16),(-23940_i16),(-13321_i16)];
_4 = [_6,_6,_6];
_3 = ['\u{f19b6}','\u{ab814}','\u{ca96b}','\u{69714}','\u{3ffc1}','\u{7ac5e}','\u{28fb4}','\u{e331f}'];
_4 = [_6,_6,_6];
_9 = (-14_isize) - (-9223372036854775808_isize);
_11 = 25934_u16 as u32;
_6 = (-469924924_i32);
Call(RET = core::intrinsics::transmute(_11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = !937679750_i32;
_8 = !5_usize;
_6 = '\u{f4af8}' as i32;
_7 = _9 as f64;
_11 = '\u{49de0}' as u32;
Goto(bb2)
}
bb2 = {
_4 = [_6,_6,_6];
_5 = _3;
_3 = ['\u{a31ae}','\u{a6b}','\u{ad6e9}','\u{f0295}','\u{243bb}','\u{1900e}','\u{c953e}','\u{88e6d}'];
_8 = 2_usize | 17927831869341845346_usize;
_2 = [_11,_11,RET,_11,RET,RET,RET,RET];
_12 = !11185202647642662732195305806788452186_u128;
_2 = [_11,_11,RET,_11,_11,_11,_11,RET];
RET = _11;
_8 = 2_usize;
_10 = [21847_i16,29803_i16,(-26465_i16),287_i16,(-23904_i16)];
_10 = [(-20554_i16),8830_i16,(-6725_i16),(-24727_i16),(-6548_i16)];
RET = true as u32;
_3[_8] = _5[_8];
_8 = 7967705500560497921_usize;
_12 = 312956661942521928299270392476760424359_u128;
Call(_14.0 = fn14(_12, _10, _10, _10, _5, _6, _10, _3, _3, _5, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _11 & _11;
_4 = [_6,_6,_6];
_8 = _7 as usize;
_16 = _12 | _12;
_14.0 = '\u{a5787}';
_4 = [_6,_6,_6];
_5 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_6 = -1633461309_i32;
_14.0 = '\u{8a6a}';
RET = _11 << _6;
_14.0 = '\u{90937}';
_8 = (-45_i8) as usize;
_7 = _6 as f64;
_17 = !13031209298635796691_u64;
_4 = [_6,_6,_6];
_9 = !(-9223372036854775808_isize);
_12 = !_16;
_11 = !RET;
_17 = !14038995525842008884_u64;
_8 = 7719501007850278014_usize;
_16 = !_12;
_14 = ('\u{b3bef}',);
_6 = 1288906531_i32 >> _11;
_10 = [(-22511_i16),6118_i16,(-8098_i16),(-23578_i16),(-11271_i16)];
_7 = _6 as f64;
RET = 56_u8 as u32;
Call(_7 = core::intrinsics::transmute(_9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = [_6,_6,_6];
_2 = [RET,_11,_11,RET,_11,RET,_11,_11];
_10 = [(-19953_i16),(-18268_i16),(-16288_i16),(-17311_i16),(-4174_i16)];
_2 = [_11,_11,RET,RET,_11,_11,RET,RET];
_8 = !1834895713602967854_usize;
_16 = _12;
_20 = _14.0;
Call(_9 = fn15(_3, _3, _3, _3, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = [_11,_11,_11,RET,RET,_11,_11,RET];
_8 = !7221509509531088942_usize;
_14.0 = _20;
_21 = [_8];
_8 = !6_usize;
_10 = [15589_i16,18865_i16,(-985_i16),(-9647_i16),(-24877_i16)];
_6 = 12457_u16 as i32;
_4 = [_6,_6,_6];
_11 = RET;
_20 = _14.0;
_17 = !12314243963646086571_u64;
_2 = [_11,RET,_11,_11,_11,RET,_11,RET];
_16 = false as u128;
Call(_9 = core::intrinsics::bswap(119_isize), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6 = _20 as i32;
_4 = [_6,_6,_6];
_21 = [_8];
_7 = _8 as f64;
_14 = (_20,);
Call(RET = fn16(_14.0, _12, _3, _3, _20, _3, _10, _2, _16, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = [_6,_6,_6];
Goto(bb8)
}
bb8 = {
_10 = [28732_i16,(-11216_i16),25951_i16,10481_i16,15658_i16];
_16 = 2351_i16 as u128;
_17 = 9097249575000170851_u64 << _16;
_12 = _16;
_23 = [_17,_17,_17];
_24 = _9 - _9;
_2 = [RET,RET,RET,_11,RET,RET,RET,RET];
RET = _11;
_25 = true;
_23 = [_17,_17,_17];
_9 = _24;
_24 = _9;
_17 = 13355114758278527756_u64 | 2086555848438436173_u64;
RET = _11 + _11;
_21 = [_8];
RET = _11;
Goto(bb9)
}
bb9 = {
_11 = !RET;
_21 = [_8];
_6 = 69_i8 as i32;
_5 = _3;
_21 = [_8];
_12 = _8 as u128;
_2 = [_11,_11,_11,_11,RET,_11,_11,RET];
_2 = [RET,_11,RET,RET,_11,_11,RET,_11];
_10 = [21777_i16,567_i16,2563_i16,7456_i16,(-19487_i16)];
Goto(bb10)
}
bb10 = {
_31 = _7 - _7;
RET = _12 as u32;
_11 = !RET;
RET = _11;
Goto(bb11)
}
bb11 = {
_9 = -_24;
_26 = _10;
_30 = [_6,_6,_6];
_27 = _20;
_18 = core::ptr::addr_of!(_32.fld4);
_32.fld2 = -_31;
_14.0 = _27;
_26 = _10;
_11 = !RET;
_8 = !8495097921239712450_usize;
_1 = &_29;
_32.fld0 = _25;
_15 = core::ptr::addr_of_mut!(_32.fld1);
(*_18) = 58781_u16;
_32.fld4 = 17495_u16 & 51294_u16;
_17 = _20 as u64;
(*_18) = 63169_u16;
(*_18) = _31 as u16;
_32.fld3 = [(-91206045164925012731822533137950815097_i128),147995910658946571805419905511945315797_i128,55272126430144422781989586781350790140_i128,(-60372264667879219978885060685039304605_i128),(-147802422630380962260940328808548655486_i128),(-89660467755678970506382629429122399731_i128)];
_17 = _12 as u64;
Goto(bb12)
}
bb12 = {
(*_18) = 47007_u16 ^ 49179_u16;
_14.0 = _20;
_17 = !5927416071514335638_u64;
_31 = _7 - _32.fld2;
_26 = [2400_i16,11307_i16,29452_i16,26248_i16,25402_i16];
_10 = _26;
_15 = core::ptr::addr_of_mut!((*_15));
_8 = !1_usize;
RET = !_11;
_16 = _12;
_11 = RET & RET;
_3 = [_20,_27,_14.0,_14.0,_27,_20,_14.0,_14.0];
_25 = !_32.fld0;
_35 = _2;
_10 = [26606_i16,9490_i16,13137_i16,(-12236_i16),(-21842_i16)];
(*_18) = 27431_u16;
_11 = !RET;
_32.fld5 = [_17,_17,_17,_17,_17,_17,_17];
_26 = [31192_i16,18917_i16,(-31002_i16),1132_i16,32753_i16];
match _32.fld4 {
0 => bb11,
1 => bb7,
2 => bb3,
3 => bb10,
4 => bb13,
5 => bb14,
27431 => bb16,
_ => bb15
}
}
bb13 = {
_4 = [_6,_6,_6];
_2 = [RET,_11,_11,RET,_11,RET,_11,_11];
_10 = [(-19953_i16),(-18268_i16),(-16288_i16),(-17311_i16),(-4174_i16)];
_2 = [_11,_11,RET,RET,_11,_11,RET,RET];
_8 = !1834895713602967854_usize;
_16 = _12;
_20 = _14.0;
Call(_9 = fn15(_3, _3, _3, _3, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb14 = {
_31 = _7 - _7;
RET = _12 as u32;
_11 = !RET;
RET = _11;
Goto(bb11)
}
bb15 = {
_6 = !937679750_i32;
_8 = !5_usize;
_6 = '\u{f4af8}' as i32;
_7 = _9 as f64;
_11 = '\u{49de0}' as u32;
Goto(bb2)
}
bb16 = {
_32.fld3 = [2121897411039093329121706542794161560_i128,(-165844019261691320727103944531293933375_i128),(-116907416395211093365687156911249253450_i128),56470822052486107231718837476613846419_i128,135592502174989868081423704359409277971_i128,(-155633254084843922928474632028183031001_i128)];
_33 = _32.fld4 << (*_18);
_14 = (_27,);
(*_18) = !_33;
(*_18) = _33 ^ _33;
_21 = [_8];
_37 = (_27,);
_10 = _26;
_5 = [_27,_14.0,_14.0,_20,_27,_27,_27,_27];
_32.fld0 = !_25;
Goto(bb17)
}
bb17 = {
Call(_41 = dump_var(13_usize, 33_usize, Move(_33), 6_usize, Move(_6), 12_usize, Move(_12), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(13_usize, 23_usize, Move(_23), 27_usize, Move(_27), 5_usize, Move(_5), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(13_usize, 9_usize, Move(_9), 11_usize, Move(_11), 10_usize, Move(_10), 20_usize, Move(_20)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: u128,mut _2: [i16; 5],mut _3: [i16; 5],mut _4: [i16; 5],mut _5: [char; 8],mut _6: i32,mut _7: [i16; 5],mut _8: [char; 8],mut _9: [char; 8],mut _10: [char; 8],mut _11: [char; 8],mut _12: [char; 8]) -> char {
mir! {
type RET = char;
let _13: (*const u8, [i16; 1]);
let _14: *mut &'static u128;
let _15: i16;
let _16: i8;
let _17: f32;
let _18: [u64; 7];
let _19: isize;
let _20: *mut isize;
let _21: [usize; 1];
let _22: [i8; 5];
let _23: isize;
let _24: Adt29;
let _25: usize;
let _26: isize;
let _27: &'static usize;
let _28: [u64; 7];
let _29: ();
let _30: ();
{
RET = '\u{a9027}';
_3 = _4;
_3 = [(-293_i16),(-12488_i16),1806_i16,27281_i16,(-7762_i16)];
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
312956661942521928299270392476760424359 => bb7,
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
_8 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = _4;
_3 = _2;
RET = '\u{106746}';
Goto(bb8)
}
bb8 = {
_8 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = _2;
_7 = [(-32509_i16),15762_i16,(-22382_i16),(-5328_i16),(-30832_i16)];
_16 = (-98_i8);
_2 = [2868_i16,21833_i16,(-22440_i16),(-2227_i16),10179_i16];
_5 = _12;
RET = '\u{5a3c7}';
_4 = _3;
_13.1 = [(-22023_i16)];
_7 = [(-21415_i16),(-3898_i16),16223_i16,(-16717_i16),29007_i16];
_19 = -9223372036854775807_isize;
_15 = 17313_i16 >> _1;
_13.1 = [_15];
_17 = 25758_u16 as f32;
_19 = (-9223372036854775808_isize);
match _1 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
312956661942521928299270392476760424359 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = _11;
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
_19 = _1 as isize;
_12 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = _7;
RET = '\u{e0a73}';
_17 = 11076539494065648007_u64 as f32;
_15 = _1 as i16;
_9 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = _10;
match _1 {
0 => bb2,
1 => bb11,
2 => bb12,
312956661942521928299270392476760424359 => bb14,
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
_25 = (-3836704423197984862_i64) as usize;
_18 = [2571267218851227306_u64,2316348534021819111_u64,15876995158378673609_u64,6968045807029352436_u64,2035447248426740991_u64,1897362537530853883_u64,9455205716801985836_u64];
_10 = _11;
_22 = [_16,_16,_16,_16,_16];
_1 = 41360009462972580906252395771509084601_u128 * 329722124711063093434439135327426510031_u128;
_20 = core::ptr::addr_of_mut!(_23);
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(14_usize, 2_usize, Move(_2), 3_usize, Move(_3), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(14_usize, 1_usize, Move(_1), 6_usize, Move(_6), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(14_usize, 19_usize, Move(_19), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [char; 8],mut _2: [char; 8],mut _3: [char; 8],mut _4: [char; 8],mut _5: [char; 8]) -> isize {
mir! {
type RET = isize;
let _6: u8;
let _7: u32;
let _8: &'static i128;
let _9: (u128, char, bool, u8);
let _10: isize;
let _11: &'static u128;
let _12: Adt29;
let _13: *mut i8;
let _14: i128;
let _15: isize;
let _16: Adt32;
let _17: &'static Adt40;
let _18: ();
let _19: ();
{
RET = (-9223372036854775808_isize) >> (-144706624306729845303555255159233100133_i128);
_2 = ['\u{8e6f0}','\u{2ad39}','\u{87b7f}','\u{f748}','\u{6217f}','\u{b8153}','\u{561f3}','\u{df17a}'];
RET = -(-9223372036854775808_isize);
RET = 9223372036854775807_isize | (-9223372036854775808_isize);
_6 = 143334135966734678580225786516527084065_u128 as u8;
RET = (-9223372036854775808_isize);
_2 = ['\u{c90ac}','\u{12d82}','\u{9f497}','\u{2e819}','\u{7f28b}','\u{7da0c}','\u{103c50}','\u{3b4fb}'];
RET = 5495231099598821880_u64 as isize;
_7 = RET as u32;
RET = -9223372036854775807_isize;
_2 = ['\u{97407}','\u{5518d}','\u{45a75}','\u{feae7}','\u{1271c}','\u{506db}','\u{7731c}','\u{f871}'];
_3 = ['\u{f291c}','\u{b35e4}','\u{5253}','\u{2e377}','\u{eacaf}','\u{e10ee}','\u{6eb5}','\u{b4e71}'];
_2 = _4;
_6 = 42_u8;
_1 = ['\u{93af7}','\u{64fc7}','\u{222bf}','\u{a3344}','\u{b15b2}','\u{defde}','\u{100994}','\u{9e75d}'];
_2 = ['\u{cbd26}','\u{50590}','\u{979a}','\u{108ad1}','\u{a7459}','\u{76990}','\u{6e035}','\u{c7b56}'];
RET = (-36_isize);
_9 = (218080323691637006877374674724576884803_u128, '\u{d52ac}', false, _6);
_9.1 = '\u{3c751}';
_3 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_9.0 = 34195794241380717087698515096576279411_u128 * 312015726295851146751603819627406894178_u128;
_6 = _9.3;
RET = (-5565_i16) as isize;
RET = -(-9223372036854775808_isize);
_9 = (49321630128474835075023676643911701264_u128, '\u{a108c}', false, _6);
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
49321630128474835075023676643911701264 => bb6,
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
_10 = 3_i8 as isize;
_9.0 = 144742590304554419577285027170921241950_u128;
match _9.3 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
42 => bb12,
_ => bb11
}
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
RET = !_10;
_9.2 = !true;
RET = _9.1 as isize;
_1 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_9.3 = !_6;
_5 = _4;
_11 = &_9.0;
RET = 11302562926482173257_u64 as isize;
_11 = &(*_11);
_10 = 3_usize as isize;
_11 = &(*_11);
_9.2 = false;
_9.1 = '\u{9a548}';
_10 = (-2070154419759384042_i64) as isize;
_9 = (145318528440658956609380428647825301628_u128, '\u{33d5e}', true, _6);
RET = 5740490917013351190_usize as isize;
_9.1 = '\u{cf922}';
_9.2 = !true;
_11 = &_9.0;
_8 = &_14;
_10 = !RET;
_10 = _9.3 as isize;
_11 = &(*_11);
match _9.3 {
0 => bb13,
1 => bb14,
42 => bb16,
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
_1 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_11 = &_9.0;
_9 = (32850584220385073385771798099361715984_u128, '\u{b3e48}', true, _6);
_7 = (-81464550340347234050093440831877973342_i128) as u32;
_9.1 = '\u{105516}';
_2 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_9 = (263508743372518773413741838328822512839_u128, '\u{e2ec3}', false, _6);
_11 = &_9.0;
RET = _10;
_15 = !RET;
_16 = Adt32::Variant2 { fld0: 7052059157736204733_u64 };
_8 = &(*_8);
_1 = _5;
RET = _10;
_9.3 = !_6;
place!(Field::<u64>(Variant(_16, 2), 0)) = !11438585130336881029_u64;
_4 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
Goto(bb17)
}
bb17 = {
Call(_18 = dump_var(15_usize, 5_usize, Move(_5), 6_usize, Move(_6), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_18 = dump_var(15_usize, 15_usize, Move(_15), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: char,mut _2: u128,mut _3: [char; 8],mut _4: [char; 8],mut _5: char,mut _6: [char; 8],mut _7: [i16; 5],mut _8: [u32; 8],mut _9: u128,mut _10: [char; 8]) -> u32 {
mir! {
type RET = u32;
let _11: f64;
let _12: u32;
let _13: [i8; 5];
let _14: isize;
let _15: char;
let _16: char;
let _17: i128;
let _18: Adt32;
let _19: i128;
let _20: f64;
let _21: &'static (f32, u32, f32, [i16; 5]);
let _22: [bool; 8];
let _23: [u32; 8];
let _24: u64;
let _25: f64;
let _26: *mut &'static u128;
let _27: [u32; 8];
let _28: (f32, u32, f32, [i16; 5]);
let _29: [usize; 1];
let _30: char;
let _31: &'static [u8; 5];
let _32: (char, [u32; 8]);
let _33: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _34: ();
let _35: ();
{
_6 = [_1,_5,_1,_5,_5,_5,_1,_1];
_3 = [_1,_1,_5,_1,_1,_5,_5,_1];
_6 = _10;
RET = 1148287173_u32 & 1311336002_u32;
_8 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = 3326559369_u32;
_3 = [_1,_1,_5,_1,_1,_1,_5,_1];
Goto(bb1)
}
bb1 = {
_11 = 156161879957556804227754695525205513276_i128 as f64;
_9 = !_2;
_7 = [15625_i16,(-22290_i16),(-12627_i16),(-24095_i16),(-17787_i16)];
_2 = !_9;
_10 = [_1,_1,_5,_5,_1,_5,_1,_5];
_6 = [_5,_1,_5,_1,_1,_5,_5,_1];
_12 = RET ^ RET;
_6 = [_5,_5,_1,_1,_1,_1,_5,_5];
_7 = [6385_i16,(-4745_i16),(-1599_i16),7842_i16,(-5911_i16)];
_3 = [_1,_1,_5,_1,_1,_5,_5,_5];
RET = _12;
_7 = [2498_i16,(-7748_i16),430_i16,(-3735_i16),2542_i16];
_7 = [24289_i16,31193_i16,(-27647_i16),29849_i16,(-8298_i16)];
_3 = _4;
_7 = [22644_i16,25583_i16,(-2890_i16),(-16772_i16),19096_i16];
_1 = _5;
_12 = 12040332556243225846_usize as u32;
Goto(bb2)
}
bb2 = {
_7 = [27900_i16,(-10782_i16),26042_i16,22807_i16,16259_i16];
RET = !_12;
_7 = [17671_i16,(-17626_i16),2214_i16,16622_i16,23175_i16];
Goto(bb3)
}
bb3 = {
RET = 7_usize as u32;
_6 = [_5,_1,_1,_1,_5,_5,_1,_1];
_3 = _4;
_10 = _4;
_2 = true as u128;
_8 = [_12,RET,_12,_12,_12,_12,RET,RET];
RET = !_12;
_8 = [RET,_12,_12,RET,RET,_12,RET,_12];
_11 = 27406_u16 as f64;
Goto(bb4)
}
bb4 = {
_11 = (-114_i8) as f64;
_9 = !_2;
_9 = _2 + _2;
_7 = [(-476_i16),(-11759_i16),30745_i16,23155_i16,(-11842_i16)];
Goto(bb5)
}
bb5 = {
Call(_9 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1 = _5;
Goto(bb7)
}
bb7 = {
_8 = [_12,_12,_12,_12,_12,RET,RET,_12];
RET = _12 ^ _12;
_12 = !RET;
_2 = _9 << RET;
_5 = _1;
_3 = [_1,_5,_1,_5,_1,_5,_1,_1];
_2 = !_9;
_5 = _1;
_7 = [(-27997_i16),(-28696_i16),25948_i16,5510_i16,29326_i16];
_6 = [_5,_5,_1,_1,_5,_1,_5,_1];
RET = !_12;
_4 = _10;
_11 = 2166586385925970356_u64 as f64;
_14 = 9223372036854775807_isize >> RET;
_16 = _1;
_14 = 9223372036854775807_isize;
Call(_15 = fn17(_4, _10, _7, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14 = !(-9223372036854775808_isize);
_9 = _2 & _2;
_6 = [_15,_16,_1,_1,_1,_1,_5,_16];
_6 = [_5,_1,_16,_16,_1,_1,_5,_16];
_1 = _15;
_12 = 15331_i16 as u32;
_11 = 4373947882985873740_i64 as f64;
_13 = [(-71_i8),120_i8,7_i8,4_i8,(-126_i8)];
_20 = _11;
RET = _9 as u32;
_13 = [(-89_i8),(-82_i8),(-124_i8),37_i8,102_i8];
_8 = [RET,RET,_12,_12,RET,_12,_12,RET];
_13 = [(-101_i8),(-49_i8),(-66_i8),0_i8,3_i8];
_1 = _5;
_12 = RET >> RET;
_2 = !_9;
_4 = [_15,_15,_16,_16,_15,_5,_5,_15];
Goto(bb9)
}
bb9 = {
_17 = !65758841138586578497165276599967443951_i128;
_20 = _11;
_19 = (-4927_i16) as i128;
_7 = [1713_i16,23807_i16,(-9994_i16),(-22485_i16),5162_i16];
_23 = _8;
_5 = _15;
_12 = (-5581776535097884717_i64) as u32;
_16 = _1;
_8 = _23;
RET = _12 >> _2;
_24 = 4409373490197959139_u64;
_12 = !RET;
_6 = [_16,_16,_5,_5,_5,_16,_16,_1];
_3 = [_5,_15,_1,_15,_16,_15,_15,_1];
_16 = _1;
_6 = [_16,_5,_5,_5,_1,_1,_5,_5];
_7 = [24075_i16,(-15138_i16),1813_i16,25054_i16,(-17559_i16)];
_7 = [(-25370_i16),24903_i16,(-11107_i16),21973_i16,(-10420_i16)];
_6 = _10;
Goto(bb10)
}
bb10 = {
_16 = _5;
_13 = [(-44_i8),(-8_i8),(-63_i8),(-30_i8),(-111_i8)];
_28.1 = !RET;
_4 = [_15,_15,_1,_15,_5,_5,_15,_15];
_22 = [false,true,false,true,false,false,true,true];
_25 = -_20;
_28.2 = RET as f32;
match _24 {
0 => bb11,
1 => bb12,
2 => bb13,
4409373490197959139 => bb15,
_ => bb14
}
}
bb11 = {
Call(_9 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
RET = 7_usize as u32;
_6 = [_5,_1,_1,_1,_5,_5,_1,_1];
_3 = _4;
_10 = _4;
_2 = true as u128;
_8 = [_12,RET,_12,_12,_12,_12,RET,RET];
RET = !_12;
_8 = [RET,_12,_12,RET,RET,_12,RET,_12];
_11 = 27406_u16 as f64;
Goto(bb4)
}
bb13 = {
_8 = [_12,_12,_12,_12,_12,RET,RET,_12];
RET = _12 ^ _12;
_12 = !RET;
_2 = _9 << RET;
_5 = _1;
_3 = [_1,_5,_1,_5,_1,_5,_1,_1];
_2 = !_9;
_5 = _1;
_7 = [(-27997_i16),(-28696_i16),25948_i16,5510_i16,29326_i16];
_6 = [_5,_5,_1,_1,_5,_1,_5,_1];
RET = !_12;
_4 = _10;
_11 = 2166586385925970356_u64 as f64;
_14 = 9223372036854775807_isize >> RET;
_16 = _1;
_14 = 9223372036854775807_isize;
Call(_15 = fn17(_4, _10, _7, _10), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_1 = _5;
Goto(bb7)
}
bb15 = {
_28.0 = _28.2;
_8 = [_12,_28.1,_28.1,_12,_28.1,_12,RET,RET];
_22 = [true,false,false,true,true,false,true,false];
_28.2 = -_28.0;
_11 = -_25;
_29 = [5_usize];
_12 = 68_u8 as u32;
RET = _28.1 >> _19;
_7 = [5263_i16,(-31618_i16),(-4162_i16),24960_i16,(-15473_i16)];
_19 = !_17;
_12 = _19 as u32;
_28.0 = _28.2;
_28.3 = _7;
_22 = [true,false,false,false,false,true,false,false];
_13 = [20_i8,(-47_i8),(-33_i8),(-109_i8),8_i8];
_14 = _17 as isize;
_9 = _24 as u128;
_14 = 9223372036854775807_isize >> RET;
_27 = [_12,RET,_28.1,_28.1,_12,RET,RET,_12];
_9 = !_2;
_6 = [_16,_15,_1,_1,_15,_16,_16,_1];
_29 = [4_usize];
_3 = [_1,_5,_5,_15,_5,_15,_15,_1];
_29 = [5_usize];
_28.2 = (-13468_i16) as f32;
_17 = _20 as i128;
_30 = _5;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(16_usize, 24_usize, Move(_24), 27_usize, Move(_27), 19_usize, Move(_19), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(16_usize, 13_usize, Move(_13), 15_usize, Move(_15), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(16_usize, 1_usize, Move(_1), 7_usize, Move(_7), 4_usize, Move(_4), 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [char; 8],mut _2: [char; 8],mut _3: [i16; 5],mut _4: [char; 8]) -> char {
mir! {
type RET = char;
let _5: [u64; 7];
let _6: u16;
let _7: isize;
let _8: u16;
let _9: isize;
let _10: ([bool; 8], Adt33);
let _11: char;
let _12: [usize; 7];
let _13: bool;
let _14: &'static (f32, u32, f32, [i16; 5]);
let _15: (isize,);
let _16: u64;
let _17: &'static u128;
let _18: u8;
let _19: *const *mut (*const u8, *const u8, (u128, char, bool, u8));
let _20: (i128, &'static u128, (f32, u32, f32, [i16; 5]), [i128; 6]);
let _21: bool;
let _22: f32;
let _23: (*const u8, [i16; 1]);
let _24: i32;
let _25: &'static [i16; 1];
let _26: isize;
let _27: bool;
let _28: isize;
let _29: [usize; 1];
let _30: (char, [u32; 8]);
let _31: isize;
let _32: ();
let _33: ();
{
_1 = _4;
RET = '\u{f5ea3}';
_5 = [12435273708739667892_u64,3840451103763033334_u64,733171439759406379_u64,1382096313740674309_u64,1389116479324526169_u64,8975237377105278479_u64,1701123573905065760_u64];
_3 = [14_i16,26750_i16,(-24255_i16),28147_i16,(-27390_i16)];
_1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [6600899527516515405_u64,12878926641255848603_u64,14730698422883125184_u64,18288334527081901450_u64,6822052740150818653_u64,8468339752530631978_u64,12738546306999187157_u64];
RET = '\u{10cced}';
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_2 = _4;
_1 = _4;
RET = '\u{f7718}';
_5 = [1337448581750311412_u64,13021782298562224031_u64,16432796402965769738_u64,4044107026026048611_u64,1261636523918091388_u64,4677394274013953559_u64,16501251847437511607_u64];
Call(RET = fn18(_5, _3, _3, _4, _1, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [12873_i16,(-13892_i16),(-11978_i16),28767_i16,16098_i16];
_1 = _4;
RET = '\u{93090}';
_3 = [(-30577_i16),17589_i16,18177_i16,16136_i16,22026_i16];
_1 = _2;
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_6 = !14613_u16;
_5 = [17402459486809842090_u64,1263946634606795330_u64,1612020480177502473_u64,2686623250403282144_u64,11575993928224105638_u64,17532776129330072416_u64,5646556574539022251_u64];
_4 = _1;
RET = '\u{2d77a}';
_5 = [7745364159501989036_u64,4839658919578890824_u64,6614402339967478115_u64,12181953755195427260_u64,2678126637404352466_u64,12564717901060210109_u64,4883816790696106796_u64];
_6 = 52504_u16 << 121938121574803893732955783879632922496_u128;
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = _4;
RET = '\u{68271}';
_5 = [11573877597701554306_u64,10437362573802838071_u64,15117013471780544625_u64,13529645159340126978_u64,15000524373278560269_u64,11938236459322347892_u64,1597268132619599869_u64];
Goto(bb2)
}
bb2 = {
_6 = 40603_u16 | 55236_u16;
_3 = [24784_i16,(-25255_i16),(-5226_i16),7654_i16,1891_i16];
_6 = !34055_u16;
_6 = 91550402_i32 as u16;
_6 = !12152_u16;
_6 = !47576_u16;
RET = '\u{1069d0}';
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = '\u{cc525}';
_5 = [3277361214790649993_u64,14204089138224701449_u64,18178281763850841658_u64,16993382945880489599_u64,11432890620403776711_u64,4649801344725074867_u64,5503852124480315503_u64];
_1 = _2;
_2 = _1;
_7 = -43_isize;
Goto(bb3)
}
bb3 = {
_4 = _2;
_7 = (-9223372036854775808_isize) * 9223372036854775807_isize;
RET = '\u{70d65}';
_4 = _2;
RET = '\u{1a8a7}';
_7 = 9223372036854775807_isize + (-9223372036854775808_isize);
RET = '\u{eaf0d}';
_7 = -(-9223372036854775808_isize);
_8 = _6;
_3 = [20047_i16,(-13081_i16),8759_i16,(-32371_i16),20798_i16];
_4 = _2;
_6 = !_8;
Goto(bb4)
}
bb4 = {
_3 = [(-3898_i16),(-12858_i16),(-28027_i16),(-31172_i16),(-6855_i16)];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = 1920085947889577241_i64 as isize;
_7 = 106_isize;
RET = '\u{5fbd}';
_10.1.fld4 = _8 - _6;
_10.1.fld5 = [9990865952471282966_u64,7597406865466782766_u64,9748317063218124006_u64,16700589103438023769_u64,12707747133543420806_u64,17950337093586131711_u64,13489978976891081510_u64];
_4 = _2;
_8 = _10.1.fld4 + _10.1.fld4;
_9 = _7;
RET = '\u{23e1}';
RET = '\u{f50fa}';
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = _7;
_2 = _1;
_5 = _10.1.fld5;
_10.1.fld4 = (-129433720748737107757317678367282863880_i128) as u16;
_10.1.fld2 = 13857259067835768095_usize as f64;
_8 = 3579589036_u32 as u16;
RET = '\u{ff938}';
_3 = [(-441_i16),(-31336_i16),22936_i16,2975_i16,(-26917_i16)];
_10.1.fld3 = [109373921844859064450295109170408990621_i128,(-70853510493605699414719150522755448343_i128),(-130529336929092833566684954696658010704_i128),(-13209128998604717708014000464131415558_i128),35833999063539060744355652252246201032_i128,90743433265675270935194667854693613196_i128];
_11 = RET;
Goto(bb5)
}
bb5 = {
_4 = _2;
_4 = [_11,RET,RET,RET,_11,_11,_11,RET];
_1 = [_11,_11,_11,RET,_11,_11,RET,RET];
_10.1.fld4 = !_8;
_8 = _7 as u16;
_16 = 196_u8 as u64;
_10.1.fld4 = 0_usize as u16;
_10.1.fld0 = RET < _11;
match _7 {
0 => bb4,
106 => bb6,
_ => bb3
}
}
bb6 = {
_11 = RET;
_13 = !_10.1.fld0;
_10.1.fld5 = [_16,_16,_16,_16,_16,_16,_16];
_3 = [(-29038_i16),(-24485_i16),17884_i16,7229_i16,(-4814_i16)];
Goto(bb7)
}
bb7 = {
_12 = [1_usize,1_usize,547925168811427067_usize,13749546372863381882_usize,2251426313084459024_usize,11592693684894253832_usize,5396042784402380399_usize];
_9 = _7;
_3 = [25346_i16,22387_i16,28366_i16,(-30613_i16),(-15506_i16)];
_10.1.fld0 = !_13;
_7 = _9;
_10.1.fld3 = [27686409326326317327694526845883836459_i128,51147389018352391222010709659325689983_i128,134490288711276201133657613459692370822_i128,25955571190705511253313558652273676358_i128,(-69516887747594348806833404630882151096_i128),18450312102674240124187295604003372907_i128];
_2 = _1;
_5 = _10.1.fld5;
_5 = [_16,_16,_16,_16,_16,_16,_16];
_10.1.fld3 = [(-114938577821451609101694589938717163851_i128),(-156146328857257130524152604963769961086_i128),(-79504379100310925906721084712169297777_i128),(-72174800029578309460510456041196545855_i128),141848513842494133576332517702068046040_i128,(-19440816205441383779645916258157539956_i128)];
_15 = (_7,);
_18 = 4_usize as u8;
_10.0 = [_13,_13,_13,_10.1.fld0,_13,_13,_10.1.fld0,_10.1.fld0];
_4 = _2;
_20.2.2 = _6 as f32;
Goto(bb8)
}
bb8 = {
RET = _11;
_20.2.0 = -_20.2.2;
_9 = _7 * _15.0;
_13 = _10.1.fld0 ^ _10.1.fld0;
_11 = RET;
_18 = !249_u8;
_21 = _13;
_2 = [_11,_11,RET,RET,_11,_11,_11,_11];
_23.0 = core::ptr::addr_of!(_18);
_2 = _4;
_16 = 6723829776415029635_u64;
RET = _11;
_10.1.fld0 = _18 >= _18;
_24 = (-1582104445_i32) ^ (-670378748_i32);
_20.2.1 = 2932258472_u32;
_10.1.fld3 = [(-42350574974768021017722855146174451033_i128),127977530748866463337714995396930749816_i128,66432282145587949196966578645328672647_i128,31496202647746660521178312285058556466_i128,(-23543664503157453018012623363786107626_i128),63884325510335273593976879376828358677_i128];
_20.2.3 = _3;
_20.0 = (-143759618771151364790860397613734774563_i128);
_22 = -_20.2.2;
_20.2.0 = _10.1.fld4 as f32;
_25 = &_23.1;
_12 = [5348100832844475007_usize,1_usize,4_usize,6929336206267853041_usize,3_usize,7_usize,6_usize];
_24 = _16 as i32;
_20.3 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
match _20.2.1 {
0 => bb1,
1 => bb5,
2 => bb9,
3 => bb10,
4 => bb11,
2932258472 => bb13,
_ => bb12
}
}
bb9 = {
_12 = [1_usize,1_usize,547925168811427067_usize,13749546372863381882_usize,2251426313084459024_usize,11592693684894253832_usize,5396042784402380399_usize];
_9 = _7;
_3 = [25346_i16,22387_i16,28366_i16,(-30613_i16),(-15506_i16)];
_10.1.fld0 = !_13;
_7 = _9;
_10.1.fld3 = [27686409326326317327694526845883836459_i128,51147389018352391222010709659325689983_i128,134490288711276201133657613459692370822_i128,25955571190705511253313558652273676358_i128,(-69516887747594348806833404630882151096_i128),18450312102674240124187295604003372907_i128];
_2 = _1;
_5 = _10.1.fld5;
_5 = [_16,_16,_16,_16,_16,_16,_16];
_10.1.fld3 = [(-114938577821451609101694589938717163851_i128),(-156146328857257130524152604963769961086_i128),(-79504379100310925906721084712169297777_i128),(-72174800029578309460510456041196545855_i128),141848513842494133576332517702068046040_i128,(-19440816205441383779645916258157539956_i128)];
_15 = (_7,);
_18 = 4_usize as u8;
_10.0 = [_13,_13,_13,_10.1.fld0,_13,_13,_10.1.fld0,_10.1.fld0];
_4 = _2;
_20.2.2 = _6 as f32;
Goto(bb8)
}
bb10 = {
_11 = RET;
_13 = !_10.1.fld0;
_10.1.fld5 = [_16,_16,_16,_16,_16,_16,_16];
_3 = [(-29038_i16),(-24485_i16),17884_i16,7229_i16,(-4814_i16)];
Goto(bb7)
}
bb11 = {
_4 = _2;
_4 = [_11,RET,RET,RET,_11,_11,_11,RET];
_1 = [_11,_11,_11,RET,_11,_11,RET,RET];
_10.1.fld4 = !_8;
_8 = _7 as u16;
_16 = 196_u8 as u64;
_10.1.fld4 = 0_usize as u16;
_10.1.fld0 = RET < _11;
match _7 {
0 => bb4,
106 => bb6,
_ => bb3
}
}
bb12 = {
_3 = [(-3898_i16),(-12858_i16),(-28027_i16),(-31172_i16),(-6855_i16)];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = 1920085947889577241_i64 as isize;
_7 = 106_isize;
RET = '\u{5fbd}';
_10.1.fld4 = _8 - _6;
_10.1.fld5 = [9990865952471282966_u64,7597406865466782766_u64,9748317063218124006_u64,16700589103438023769_u64,12707747133543420806_u64,17950337093586131711_u64,13489978976891081510_u64];
_4 = _2;
_8 = _10.1.fld4 + _10.1.fld4;
_9 = _7;
RET = '\u{23e1}';
RET = '\u{f50fa}';
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = _7;
_2 = _1;
_5 = _10.1.fld5;
_10.1.fld4 = (-129433720748737107757317678367282863880_i128) as u16;
_10.1.fld2 = 13857259067835768095_usize as f64;
_8 = 3579589036_u32 as u16;
RET = '\u{ff938}';
_3 = [(-441_i16),(-31336_i16),22936_i16,2975_i16,(-26917_i16)];
_10.1.fld3 = [109373921844859064450295109170408990621_i128,(-70853510493605699414719150522755448343_i128),(-130529336929092833566684954696658010704_i128),(-13209128998604717708014000464131415558_i128),35833999063539060744355652252246201032_i128,90743433265675270935194667854693613196_i128];
_11 = RET;
Goto(bb5)
}
bb13 = {
_10.0 = [_21,_13,_21,_21,_13,_21,_21,_10.1.fld0];
_23.1 = [(-12334_i16)];
_10.1.fld0 = !_21;
_20.2.3 = [(-4920_i16),(-9933_i16),(-1772_i16),28654_i16,(-3221_i16)];
_22 = _20.2.0;
_4 = [RET,_11,_11,RET,RET,RET,_11,RET];
_10.1.fld4 = _8;
_14 = &_20.2;
_12 = [12337930095791082668_usize,7792264216166271396_usize,3_usize,17657202101526984343_usize,14004886028600971861_usize,16655572083169222580_usize,5_usize];
_24 = 1787059388_i32;
_14 = &_20.2;
_21 = _8 > _8;
_10.1.fld0 = _21 ^ _21;
_1 = _4;
_24 = 650082476_i32 - 1103994926_i32;
_10.0 = [_13,_13,_21,_21,_21,_10.1.fld0,_10.1.fld0,_21];
_4 = [RET,RET,_11,RET,RET,_11,_11,RET];
_12 = [5_usize,1_usize,1722584552009522673_usize,4_usize,2616083078174933477_usize,1_usize,1_usize];
_20.2.3 = _3;
_20.2 = (_22, 472923854_u32, _22, _3);
_28 = -_15.0;
_21 = !_13;
match _7 {
0 => bb1,
1 => bb14,
2 => bb15,
106 => bb17,
_ => bb16
}
}
bb14 = {
_12 = [1_usize,1_usize,547925168811427067_usize,13749546372863381882_usize,2251426313084459024_usize,11592693684894253832_usize,5396042784402380399_usize];
_9 = _7;
_3 = [25346_i16,22387_i16,28366_i16,(-30613_i16),(-15506_i16)];
_10.1.fld0 = !_13;
_7 = _9;
_10.1.fld3 = [27686409326326317327694526845883836459_i128,51147389018352391222010709659325689983_i128,134490288711276201133657613459692370822_i128,25955571190705511253313558652273676358_i128,(-69516887747594348806833404630882151096_i128),18450312102674240124187295604003372907_i128];
_2 = _1;
_5 = _10.1.fld5;
_5 = [_16,_16,_16,_16,_16,_16,_16];
_10.1.fld3 = [(-114938577821451609101694589938717163851_i128),(-156146328857257130524152604963769961086_i128),(-79504379100310925906721084712169297777_i128),(-72174800029578309460510456041196545855_i128),141848513842494133576332517702068046040_i128,(-19440816205441383779645916258157539956_i128)];
_15 = (_7,);
_18 = 4_usize as u8;
_10.0 = [_13,_13,_13,_10.1.fld0,_13,_13,_10.1.fld0,_10.1.fld0];
_4 = _2;
_20.2.2 = _6 as f32;
Goto(bb8)
}
bb15 = {
_4 = _2;
_4 = [_11,RET,RET,RET,_11,_11,_11,RET];
_1 = [_11,_11,_11,RET,_11,_11,RET,RET];
_10.1.fld4 = !_8;
_8 = _7 as u16;
_16 = 196_u8 as u64;
_10.1.fld4 = 0_usize as u16;
_10.1.fld0 = RET < _11;
match _7 {
0 => bb4,
106 => bb6,
_ => bb3
}
}
bb16 = {
_6 = 40603_u16 | 55236_u16;
_3 = [24784_i16,(-25255_i16),(-5226_i16),7654_i16,1891_i16];
_6 = !34055_u16;
_6 = 91550402_i32 as u16;
_6 = !12152_u16;
_6 = !47576_u16;
RET = '\u{1069d0}';
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = '\u{cc525}';
_5 = [3277361214790649993_u64,14204089138224701449_u64,18178281763850841658_u64,16993382945880489599_u64,11432890620403776711_u64,4649801344725074867_u64,5503852124480315503_u64];
_1 = _2;
_2 = _1;
_7 = -43_isize;
Goto(bb3)
}
bb17 = {
_16 = 18339790795552240191_u64 * 16305814516044799117_u64;
_27 = _10.1.fld0 <= _13;
RET = _11;
_9 = _7;
_1 = [RET,RET,_11,_11,_11,RET,_11,_11];
_20.0 = -(-49034588926761416802966694547979930541_i128);
_7 = _11 as isize;
_20.2.2 = 256536322042451601622734255132188074456_u128 as f32;
_28 = _15.0;
RET = _11;
_14 = &_20.2;
_4 = _1;
_10.1.fld0 = (*_14).0 < _22;
_14 = &_20.2;
_28 = _7 | _15.0;
_23.0 = core::ptr::addr_of!(_18);
_20.0 = (-84099778710593016707190277140501028627_i128) + 138525149846974148020913498577191529155_i128;
_10.0 = [_27,_27,_27,_21,_13,_27,_10.1.fld0,_27];
_30.1 = [(*_14).1,(*_14).1,(*_14).1,(*_14).1,(*_14).1,_20.2.1,(*_14).1,(*_14).1];
Goto(bb18)
}
bb18 = {
Call(_32 = dump_var(17_usize, 6_usize, Move(_6), 1_usize, Move(_1), 27_usize, Move(_27), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_32 = dump_var(17_usize, 12_usize, Move(_12), 16_usize, Move(_16), 9_usize, Move(_9), 28_usize, Move(_28)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(17_usize, 2_usize, Move(_2), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [u64; 7],mut _2: [i16; 5],mut _3: [i16; 5],mut _4: [char; 8],mut _5: [char; 8],mut _6: [char; 8],mut _7: [i16; 5]) -> char {
mir! {
type RET = char;
let _8: bool;
let _9: i64;
let _10: *const (u128, char, bool, u8);
let _11: *mut (u128, char, bool, u8);
let _12: isize;
let _13: (char, [u32; 8]);
let _14: i128;
let _15: [u64; 7];
let _16: [u8; 5];
let _17: (f32, u32, f32, [i16; 5]);
let _18: [i32; 3];
let _19: i16;
let _20: [usize; 7];
let _21: *mut i8;
let _22: [u64; 7];
let _23: i8;
let _24: f32;
let _25: [usize; 7];
let _26: f32;
let _27: [usize; 1];
let _28: f32;
let _29: &'static [u8; 5];
let _30: ();
let _31: ();
{
RET = '\u{a799e}';
_2 = [(-24616_i16),15748_i16,(-3312_i16),(-5965_i16),(-24757_i16)];
_7 = [3758_i16,29658_i16,18861_i16,(-24497_i16),18529_i16];
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [23261_i16,6781_i16,12915_i16,(-26206_i16),15620_i16];
RET = '\u{1db99}';
RET = '\u{766bc}';
_8 = false;
_3 = _2;
Goto(bb1)
}
bb1 = {
_7 = _3;
_2 = _3;
RET = '\u{5c49c}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = 802615927020803692_i64;
RET = '\u{1835f}';
_3 = [(-30421_i16),(-6395_i16),(-19392_i16),(-30741_i16),(-3940_i16)];
_1 = [5195198476088284239_u64,5833703278307171106_u64,7883114084801966925_u64,15499201268412128683_u64,10123496117258715421_u64,755899370251487501_u64,5201124007841457950_u64];
_9 = 0_usize as i64;
_4 = _6;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = !1874578670369497055_i64;
_2 = [(-27371_i16),7805_i16,16480_i16,(-3649_i16),(-23515_i16)];
_3 = [(-17640_i16),(-4561_i16),24802_i16,(-1540_i16),18210_i16];
_3 = [8734_i16,14090_i16,24381_i16,(-17647_i16),17715_i16];
RET = '\u{f9482}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [16941819514123905767_u64,17260153850119098670_u64,8950616080389913909_u64,17630680907626090375_u64,14440508285269156989_u64,17187119506639454871_u64,6233701724359206200_u64];
Goto(bb2)
}
bb2 = {
_9 = -812964753675626542_i64;
RET = '\u{ddc2e}';
_2 = _3;
_9 = 1068961922116618064_i64;
_2 = _7;
_1 = [2278092974320643581_u64,9395488695712593214_u64,2463310611010146419_u64,836623264770448710_u64,5380021277768804233_u64,8723830160738450828_u64,1160085681929680269_u64];
_9 = 3529115869453107038_i64 & 6259432257160356314_i64;
_2 = [(-5393_i16),2685_i16,4654_i16,(-3196_i16),16352_i16];
_9 = (-7031801812319897020_i64) - (-2788337330263633245_i64);
_3 = [23952_i16,19257_i16,11346_i16,(-8539_i16),24289_i16];
_3 = _7;
_1 = [8184244366993551704_u64,16579964359537814260_u64,12363758099137140354_u64,12622785634781274907_u64,454234983739744388_u64,13012997351427452841_u64,11773923941500511363_u64];
_12 = 18_isize;
RET = '\u{f6e9d}';
RET = '\u{42761}';
_3 = _2;
_7 = [(-6586_i16),(-10915_i16),(-24534_i16),25040_i16,(-18454_i16)];
_1 = [3836789486042161407_u64,4637195007420761267_u64,14800081743802543870_u64,17355412463468030323_u64,2749968037383922131_u64,18435291383324065553_u64,1153802875233954225_u64];
_7 = [4183_i16,(-15013_i16),(-23208_i16),(-28950_i16),18567_i16];
_8 = true;
_6 = _5;
_2 = [(-21323_i16),17587_i16,(-9849_i16),11775_i16,(-2740_i16)];
RET = '\u{cadf3}';
RET = '\u{93b82}';
_8 = false;
Call(_3 = fn19(RET, _1, _12, _2, _7, _9, _7, _1, _2, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = 121141832804317356634432034500108169444_i128;
_4 = _6;
_1 = [5430760670393595607_u64,8961426817511766495_u64,3284322010071426936_u64,2784909505061114743_u64,11648661423470256173_u64,5258562042131617252_u64,6529357913564864158_u64];
_1 = [3322673924998197786_u64,6751372048066547537_u64,6689302532738995546_u64,12424927818777542952_u64,11764480846907061236_u64,13712162511102296774_u64,9002329387107981341_u64];
_2 = [(-9881_i16),17956_i16,(-13118_i16),14985_i16,416_i16];
_13.0 = RET;
_4 = [_13.0,RET,RET,RET,RET,RET,RET,_13.0];
_13.1 = [305835142_u32,645464205_u32,3290019337_u32,1063350737_u32,4283235470_u32,881670172_u32,2802553157_u32,3355202376_u32];
_13.1 = [567906415_u32,2250843518_u32,2535296344_u32,3313654827_u32,1020414462_u32,719316470_u32,2504905465_u32,545331502_u32];
match _12 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
18 => bb7,
_ => bb6
}
}
bb4 = {
_9 = -812964753675626542_i64;
RET = '\u{ddc2e}';
_2 = _3;
_9 = 1068961922116618064_i64;
_2 = _7;
_1 = [2278092974320643581_u64,9395488695712593214_u64,2463310611010146419_u64,836623264770448710_u64,5380021277768804233_u64,8723830160738450828_u64,1160085681929680269_u64];
_9 = 3529115869453107038_i64 & 6259432257160356314_i64;
_2 = [(-5393_i16),2685_i16,4654_i16,(-3196_i16),16352_i16];
_9 = (-7031801812319897020_i64) - (-2788337330263633245_i64);
_3 = [23952_i16,19257_i16,11346_i16,(-8539_i16),24289_i16];
_3 = _7;
_1 = [8184244366993551704_u64,16579964359537814260_u64,12363758099137140354_u64,12622785634781274907_u64,454234983739744388_u64,13012997351427452841_u64,11773923941500511363_u64];
_12 = 18_isize;
RET = '\u{f6e9d}';
RET = '\u{42761}';
_3 = _2;
_7 = [(-6586_i16),(-10915_i16),(-24534_i16),25040_i16,(-18454_i16)];
_1 = [3836789486042161407_u64,4637195007420761267_u64,14800081743802543870_u64,17355412463468030323_u64,2749968037383922131_u64,18435291383324065553_u64,1153802875233954225_u64];
_7 = [4183_i16,(-15013_i16),(-23208_i16),(-28950_i16),18567_i16];
_8 = true;
_6 = _5;
_2 = [(-21323_i16),17587_i16,(-9849_i16),11775_i16,(-2740_i16)];
RET = '\u{cadf3}';
RET = '\u{93b82}';
_8 = false;
Call(_3 = fn19(RET, _1, _12, _2, _7, _9, _7, _1, _2, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7 = _3;
_2 = _3;
RET = '\u{5c49c}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = 802615927020803692_i64;
RET = '\u{1835f}';
_3 = [(-30421_i16),(-6395_i16),(-19392_i16),(-30741_i16),(-3940_i16)];
_1 = [5195198476088284239_u64,5833703278307171106_u64,7883114084801966925_u64,15499201268412128683_u64,10123496117258715421_u64,755899370251487501_u64,5201124007841457950_u64];
_9 = 0_usize as i64;
_4 = _6;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = !1874578670369497055_i64;
_2 = [(-27371_i16),7805_i16,16480_i16,(-3649_i16),(-23515_i16)];
_3 = [(-17640_i16),(-4561_i16),24802_i16,(-1540_i16),18210_i16];
_3 = [8734_i16,14090_i16,24381_i16,(-17647_i16),17715_i16];
RET = '\u{f9482}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [16941819514123905767_u64,17260153850119098670_u64,8950616080389913909_u64,17630680907626090375_u64,14440508285269156989_u64,17187119506639454871_u64,6233701724359206200_u64];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_4 = [_13.0,_13.0,RET,RET,RET,RET,_13.0,RET];
_4 = [RET,_13.0,RET,RET,RET,RET,RET,_13.0];
_6 = _4;
_2 = _3;
RET = _13.0;
_15 = _1;
_4 = [RET,RET,_13.0,RET,RET,RET,RET,RET];
_12 = !(-9223372036854775808_isize);
_5 = _6;
_7 = _2;
_16 = [8_u8,97_u8,21_u8,20_u8,60_u8];
RET = _13.0;
RET = _13.0;
_14 = (-158970392713345891861093969239027098070_i128);
_12 = 25776_i16 as isize;
_17.0 = (-66_i8) as f32;
_3 = _2;
_17.1 = _8 as u32;
RET = _13.0;
_9 = !(-3917405729102941875_i64);
_17.3 = _3;
Goto(bb8)
}
bb8 = {
RET = _13.0;
_7 = [(-28174_i16),16758_i16,(-1222_i16),22634_i16,12494_i16];
_13.1 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_2 = _3;
_17.3 = [(-23372_i16),(-25344_i16),(-28169_i16),10167_i16,10115_i16];
_18 = [(-256732050_i32),1585578374_i32,25647899_i32];
_15 = [1218676316501485766_u64,5263576699246677024_u64,3475415053256169265_u64,1778508388622577590_u64,9717538923933159932_u64,2347585589158465869_u64,11319729797371476317_u64];
_14 = (-100144004547433690819144781427924050889_i128) ^ (-58411309543232297316507935387556645077_i128);
_12 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_13.0 = RET;
_19 = 231_u8 as i16;
RET = _13.0;
_14 = _8 as i128;
RET = _13.0;
RET = _13.0;
_18 = [(-1768327558_i32),2004172946_i32,(-1410478067_i32)];
_7 = [_19,_19,_19,_19,_19];
_2 = _7;
_24 = 195814393022486952468340501326429023206_u128 as f32;
_17 = (_24, 498459138_u32, _24, _2);
_3 = [_19,_19,_19,_19,_19];
_15 = _1;
_23 = 4_i8;
match _17.1 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb5,
498459138 => bb10,
_ => bb9
}
}
bb9 = {
_7 = _3;
_2 = _3;
RET = '\u{5c49c}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = 802615927020803692_i64;
RET = '\u{1835f}';
_3 = [(-30421_i16),(-6395_i16),(-19392_i16),(-30741_i16),(-3940_i16)];
_1 = [5195198476088284239_u64,5833703278307171106_u64,7883114084801966925_u64,15499201268412128683_u64,10123496117258715421_u64,755899370251487501_u64,5201124007841457950_u64];
_9 = 0_usize as i64;
_4 = _6;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = !1874578670369497055_i64;
_2 = [(-27371_i16),7805_i16,16480_i16,(-3649_i16),(-23515_i16)];
_3 = [(-17640_i16),(-4561_i16),24802_i16,(-1540_i16),18210_i16];
_3 = [8734_i16,14090_i16,24381_i16,(-17647_i16),17715_i16];
RET = '\u{f9482}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [16941819514123905767_u64,17260153850119098670_u64,8950616080389913909_u64,17630680907626090375_u64,14440508285269156989_u64,17187119506639454871_u64,6233701724359206200_u64];
Goto(bb2)
}
bb10 = {
_21 = core::ptr::addr_of_mut!(_23);
_9 = (-6057305583321488117_i64) << _17.1;
_3 = _2;
_20 = [1744109051876213242_usize,16468578998827392123_usize,6_usize,1_usize,89585192676741077_usize,7313934133383297861_usize,17082293205697361593_usize];
_25 = [4794639186087579202_usize,2_usize,15827934516533393410_usize,4_usize,5_usize,14215835843876966002_usize,7_usize];
_18 = [199826690_i32,1077071383_i32,17916526_i32];
_5 = [RET,RET,_13.0,RET,_13.0,_13.0,RET,_13.0];
_13.0 = RET;
RET = _13.0;
_6 = _4;
_7 = [_19,_19,_19,_19,_19];
_26 = -_24;
_21 = core::ptr::addr_of_mut!((*_21));
Goto(bb11)
}
bb11 = {
_5 = [_13.0,RET,_13.0,_13.0,_13.0,RET,_13.0,RET];
_27 = [3_usize];
_7 = [_19,_19,_19,_19,_19];
match _17.1 {
0 => bb1,
1 => bb8,
2 => bb12,
3 => bb13,
498459138 => bb15,
_ => bb14
}
}
bb12 = {
_7 = _3;
_2 = _3;
RET = '\u{5c49c}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = 802615927020803692_i64;
RET = '\u{1835f}';
_3 = [(-30421_i16),(-6395_i16),(-19392_i16),(-30741_i16),(-3940_i16)];
_1 = [5195198476088284239_u64,5833703278307171106_u64,7883114084801966925_u64,15499201268412128683_u64,10123496117258715421_u64,755899370251487501_u64,5201124007841457950_u64];
_9 = 0_usize as i64;
_4 = _6;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = !1874578670369497055_i64;
_2 = [(-27371_i16),7805_i16,16480_i16,(-3649_i16),(-23515_i16)];
_3 = [(-17640_i16),(-4561_i16),24802_i16,(-1540_i16),18210_i16];
_3 = [8734_i16,14090_i16,24381_i16,(-17647_i16),17715_i16];
RET = '\u{f9482}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [16941819514123905767_u64,17260153850119098670_u64,8950616080389913909_u64,17630680907626090375_u64,14440508285269156989_u64,17187119506639454871_u64,6233701724359206200_u64];
Goto(bb2)
}
bb13 = {
_7 = _3;
_2 = _3;
RET = '\u{5c49c}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = 802615927020803692_i64;
RET = '\u{1835f}';
_3 = [(-30421_i16),(-6395_i16),(-19392_i16),(-30741_i16),(-3940_i16)];
_1 = [5195198476088284239_u64,5833703278307171106_u64,7883114084801966925_u64,15499201268412128683_u64,10123496117258715421_u64,755899370251487501_u64,5201124007841457950_u64];
_9 = 0_usize as i64;
_4 = _6;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_9 = !1874578670369497055_i64;
_2 = [(-27371_i16),7805_i16,16480_i16,(-3649_i16),(-23515_i16)];
_3 = [(-17640_i16),(-4561_i16),24802_i16,(-1540_i16),18210_i16];
_3 = [8734_i16,14090_i16,24381_i16,(-17647_i16),17715_i16];
RET = '\u{f9482}';
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [16941819514123905767_u64,17260153850119098670_u64,8950616080389913909_u64,17630680907626090375_u64,14440508285269156989_u64,17187119506639454871_u64,6233701724359206200_u64];
Goto(bb2)
}
bb14 = {
_9 = -812964753675626542_i64;
RET = '\u{ddc2e}';
_2 = _3;
_9 = 1068961922116618064_i64;
_2 = _7;
_1 = [2278092974320643581_u64,9395488695712593214_u64,2463310611010146419_u64,836623264770448710_u64,5380021277768804233_u64,8723830160738450828_u64,1160085681929680269_u64];
_9 = 3529115869453107038_i64 & 6259432257160356314_i64;
_2 = [(-5393_i16),2685_i16,4654_i16,(-3196_i16),16352_i16];
_9 = (-7031801812319897020_i64) - (-2788337330263633245_i64);
_3 = [23952_i16,19257_i16,11346_i16,(-8539_i16),24289_i16];
_3 = _7;
_1 = [8184244366993551704_u64,16579964359537814260_u64,12363758099137140354_u64,12622785634781274907_u64,454234983739744388_u64,13012997351427452841_u64,11773923941500511363_u64];
_12 = 18_isize;
RET = '\u{f6e9d}';
RET = '\u{42761}';
_3 = _2;
_7 = [(-6586_i16),(-10915_i16),(-24534_i16),25040_i16,(-18454_i16)];
_1 = [3836789486042161407_u64,4637195007420761267_u64,14800081743802543870_u64,17355412463468030323_u64,2749968037383922131_u64,18435291383324065553_u64,1153802875233954225_u64];
_7 = [4183_i16,(-15013_i16),(-23208_i16),(-28950_i16),18567_i16];
_8 = true;
_6 = _5;
_2 = [(-21323_i16),17587_i16,(-9849_i16),11775_i16,(-2740_i16)];
RET = '\u{cadf3}';
RET = '\u{93b82}';
_8 = false;
Call(_3 = fn19(RET, _1, _12, _2, _7, _9, _7, _1, _2, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_13.1 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_27 = [2503655078867619610_usize];
(*_21) = 98_i8 | (-42_i8);
_5 = [RET,_13.0,RET,_13.0,_13.0,RET,_13.0,RET];
_15 = _1;
_27 = [3_usize];
_21 = core::ptr::addr_of_mut!((*_21));
_13.0 = RET;
_25 = [1_usize,3_usize,16779337667321378384_usize,3_usize,4506840009985249553_usize,1_usize,5_usize];
_9 = _24 as i64;
_26 = -_17.0;
_12 = _19 as isize;
_26 = _24 * _17.0;
_15 = [4507342685423812319_u64,16831108121326742644_u64,7214202578467431982_u64,6836097115969988032_u64,7624933355862608373_u64,1157541380369026217_u64,14948330699733142373_u64];
_29 = &_16;
_19 = !(-31110_i16);
_17.2 = _26;
_13.1 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_13.0 = RET;
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(18_usize, 25_usize, Move(_25), 23_usize, Move(_23), 5_usize, Move(_5), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(18_usize, 2_usize, Move(_2), 8_usize, Move(_8), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(18_usize, 9_usize, Move(_9), 27_usize, Move(_27), 31_usize, _31, 31_usize, _31), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: char,mut _2: [u64; 7],mut _3: isize,mut _4: [i16; 5],mut _5: [i16; 5],mut _6: i64,mut _7: [i16; 5],mut _8: [u64; 7],mut _9: [i16; 5],mut _10: [u64; 7]) -> [i16; 5] {
mir! {
type RET = [i16; 5];
let _11: f32;
let _12: *mut (*const u8, *const u8, (u128, char, bool, u8));
let _13: *mut u8;
let _14: u32;
let _15: *const &'static f32;
let _16: (f32, u32, f32, [i16; 5]);
let _17: u8;
let _18: ();
let _19: ();
{
_2 = [5494188152282944205_u64,4755920331031851286_u64,3428367738536436218_u64,16557921658172175074_u64,14927292985827223347_u64,14771097376968921478_u64,1156925801896739204_u64];
_9 = [29601_i16,(-8100_i16),(-31290_i16),(-20987_i16),25399_i16];
RET = [(-9353_i16),(-2728_i16),10688_i16,1489_i16,27586_i16];
match _3 {
0 => bb1,
1 => bb2,
18 => bb4,
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
_7 = _5;
_2 = _8;
_6 = -1284096822531334730_i64;
_10 = [17861391235481144331_u64,13212372909558995720_u64,14673109354446334308_u64,2985217631532293346_u64,10820608328752241707_u64,3921747635485500997_u64,18169818199533870781_u64];
_8 = [17374300729399371636_u64,12211507827424993594_u64,8374587711399937268_u64,6654625898986951936_u64,9443803304182459000_u64,3165354215966345826_u64,5671524513232473520_u64];
_8 = [10226678736570939974_u64,17569918434065073695_u64,7867747531207488654_u64,2342034230937024576_u64,8825982673178943507_u64,11880758971078425410_u64,17186928310797454909_u64];
_8 = [6140712949509635426_u64,13523366592121037260_u64,17654318069348611112_u64,2242675097833064774_u64,9809121732029824927_u64,16356165912909817857_u64,4360197345722190481_u64];
_10 = _2;
_5 = _7;
_11 = 9461215425705469490_u64 as f32;
_6 = (-6466458657201523665_i64);
_4 = [(-7296_i16),12248_i16,(-3845_i16),(-1869_i16),1217_i16];
_7 = _9;
_4 = [7022_i16,(-11368_i16),(-26334_i16),(-16321_i16),(-24722_i16)];
_5 = [31088_i16,9079_i16,5971_i16,19029_i16,(-12116_i16)];
_7 = [3938_i16,32657_i16,20772_i16,24770_i16,12931_i16];
RET = _7;
_3 = -9223372036854775807_isize;
_7 = _4;
match _6 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463456908148774566687791 => bb13,
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
Return()
}
bb13 = {
_5 = _9;
_9 = [(-25137_i16),14636_i16,10445_i16,29264_i16,(-10007_i16)];
RET = [19447_i16,(-16906_i16),23399_i16,(-3498_i16),(-13554_i16)];
_8 = [4719738322431848549_u64,7350037496045426337_u64,14389561955335419094_u64,16397054962940750495_u64,17397087236127770679_u64,5453497498693319854_u64,4338697089409706193_u64];
_3 = !9223372036854775807_isize;
RET = [(-20359_i16),222_i16,28088_i16,(-29371_i16),(-14090_i16)];
_2 = [16060758164859080233_u64,9229329415577467919_u64,15778283720356671721_u64,13138073769124228159_u64,15370579533902434460_u64,852884163967828867_u64,9871776231550427049_u64];
RET = _5;
_16 = (_11, 432299537_u32, _11, _9);
_14 = 16006658155500898226_usize as u32;
_10 = _8;
_1 = '\u{10c6bb}';
_13 = core::ptr::addr_of_mut!(_17);
match _16.1 {
432299537 => bb14,
_ => bb5
}
}
bb14 = {
(*_13) = 135319819615347023877654222836738027043_i128 as u8;
RET = [(-7312_i16),25937_i16,(-23731_i16),(-10378_i16),6228_i16];
_3 = !9223372036854775807_isize;
_7 = _16.3;
_16.0 = _11 - _11;
_6 = (-5903868684545717152_i64) << _3;
(*_13) = 17_u8;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(19_usize, 17_usize, Move(_17), 9_usize, Move(_9), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(19_usize, 3_usize, Move(_3), 8_usize, Move(_8), 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(8885614478374951187_u64), std::hint::black_box('\u{d7c4f}'), std::hint::black_box(18094357148879393773046400689021537439_u128), std::hint::black_box(409191553_u32), std::hint::black_box((-29348_i16)), std::hint::black_box((-859151700_i32)), std::hint::black_box((-4510587923855724342_i64)), std::hint::black_box(11595626039144647813110166882119004881_i128), std::hint::black_box(16691622658737970894_usize), std::hint::black_box(103_u8), std::hint::black_box(33468_u16));
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt21{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt21 {
fld0: u8,
fld1: [i16; 1],
fld2: i128,
fld3: i8,
}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){unsafe{printf("Adt29::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt29 {
Variant0{
fld0: bool,
fld1: u128,
fld2: u64,
fld3: Adt21,
fld4: i16,
fld5: *mut u8,
fld6: i64,

},
Variant1{
fld0: f32,
fld1: *mut u8,
fld2: i128,
fld3: *mut *mut u8,

},
Variant2{
fld0: u8,

},
Variant3{
fld0: *mut *mut *mut u8,
fld1: *mut *mut u8,
fld2: *const u8,
fld3: i32,
fld4: [i16; 1],

}}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: f32,
fld1: char,
fld2: usize,

},
Variant1{
fld0: bool,
fld1: i32,
fld2: u128,
fld3: *const u8,
fld4: u16,

},
Variant2{
fld0: u64,

},
Variant3{
fld0: [i16; 1],
fld1: u64,
fld2: f64,
fld3: i8,
fld4: *mut u8,
fld5: i32,
fld6: *mut *mut *mut u8,
fld7: i128,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt33{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt33 {
fld0: bool,
fld1: *mut *mut u8,
fld2: f64,
fld3: [i128; 6],
fld4: u16,
fld5: [u64; 7],
}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: i64,
fld1: [i128; 6],
fld2: (char, *mut u8),

},
Variant1{
fld0: f32,
fld1: [bool; 8],
fld2: isize,
fld3: Adt33,
fld4: i16,
fld5: Adt32,
fld6: u32,

},
Variant2{
fld0: bool,
fld1: *mut isize,
fld2: (isize,),
fld3: Adt33,
fld4: u128,
fld5: usize,
fld6: (u128, char, bool, u8),
fld7: Adt21,

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf("Adt61::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: usize,

},
Variant1{
fld0: (*const u8, *const u8, (u128, char, bool, u8)),
fld1: Adt32,

}}
impl PrintFDebug for Adt83{
	unsafe fn printf_debug(&self){unsafe{printf("Adt83::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt83 {
Variant0{
fld0: [i32; 3],
fld1: [char; 8],
fld2: (*const u8, [i16; 1]),
fld3: Adt32,
fld4: *const u8,

},
Variant1{
fld0: *mut u8,
fld1: [usize; 7],
fld2: [i8; 5],
fld3: [i16; 5],
fld4: *mut *mut *mut u8,
fld5: u128,
fld6: *mut isize,
fld7: i128,

},
Variant2{
fld0: [i16; 5],
fld1: [u8; 5],
fld2: isize,
fld3: u64,
fld4: [u64; 7],
fld5: [u32; 8],
fld6: usize,
fld7: u16,

},
Variant3{
fld0: *const Adt29,
fld1: [i16; 1],
fld2: u64,
fld3: (char, [u32; 8]),
fld4: [char; 8],
fld5: [u64; 3],
fld6: u128,

}}
impl PrintFDebug for Adt84{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt84{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt84 {
fld0: [i8; 5],
fld1: *mut u8,
}

