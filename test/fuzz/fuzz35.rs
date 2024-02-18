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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u64,mut _4: u32,mut _5: u8,mut _6: usize,mut _7: u128,mut _8: u16) -> char {
mir! {
type RET = char;
let _9: *mut [u8; 8];
let _10: i128;
let _11: *mut [i32; 7];
let _12: f64;
let _13: (*mut i128, u64, Adt33);
let _14: char;
let _15: usize;
let _16: isize;
let _17: i16;
let _18: &'static Adt17;
let _19: &'static *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _20: f64;
let _21: f32;
let _22: ();
let _23: ();
{
_3 = 37256_u16 as u64;
_4 = !3906298442_u32;
RET = '\u{107ff1}';
_2 = RET;
_1 = _4 >= _4;
_4 = 1244125853_u32 >> _3;
RET = _2;
_8 = 26251_u16;
_7 = !331614187279725288084849160399078445783_u128;
_2 = RET;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
26251 => bb8,
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
_5 = (-52_isize) as u8;
Call(_5 = fn1(_2, RET, _2, _1, _2, _1, RET, RET, _1, RET), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = (-47535966858340047_i64) as f64;
_6 = _4 as usize;
_7 = !109430677185215603978410568413042481000_u128;
_3 = 14058363732426831231_u64;
_4 = (-108_i8) as u32;
_8 = 37510_u16 << _5;
_10 = (-65468887000965382894661530369741632684_i128);
_3 = !9265073061487631954_u64;
_13.1 = !_3;
_7 = 270228411234960078347039016062362726776_u128 & 239535230484845055542594835791817326580_u128;
RET = _2;
RET = _2;
_15 = _6 ^ _6;
_14 = RET;
_13.0 = core::ptr::addr_of_mut!(_10);
_7 = 190418138994296178492034438024357173087_u128;
Goto(bb10)
}
bb10 = {
_14 = _2;
_7 = 42057821784199227407380922061369818091_u128;
_6 = _15 ^ _15;
_6 = !_15;
_2 = _14;
_15 = _8 as usize;
RET = _14;
RET = _2;
_14 = RET;
_13.1 = !_3;
_14 = _2;
_2 = _14;
_6 = _15;
_5 = 61_u8 + 220_u8;
RET = _2;
_7 = 65_isize as u128;
_16 = (-9223372036854775808_isize);
RET = _2;
_7 = 125614281267611053735860182193446465591_u128;
RET = _2;
_6 = !_15;
RET = _2;
RET = _2;
match _16 {
0 => bb5,
1 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb8
}
}
bb11 = {
_13.0 = core::ptr::addr_of_mut!(_10);
_13.0 = core::ptr::addr_of_mut!(_10);
_4 = 1831148897_u32 << _8;
_10 = -(-14786252503027034369994904761624744420_i128);
_4 = (-88_i8) as u32;
_2 = _14;
_13.1 = _2 as u64;
_17 = (-26861_i16) ^ (-19320_i16);
_3 = _13.1;
_17 = 22588_i16;
_14 = _2;
_7 = _3 as u128;
_13.0 = core::ptr::addr_of_mut!(_10);
Goto(bb12)
}
bb12 = {
match _16 {
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
}
bb13 = {
_13.0 = core::ptr::addr_of_mut!(_10);
_13.0 = core::ptr::addr_of_mut!(_10);
_4 = 1831148897_u32 << _8;
_10 = -(-14786252503027034369994904761624744420_i128);
_4 = (-88_i8) as u32;
_2 = _14;
_13.1 = _2 as u64;
_17 = (-26861_i16) ^ (-19320_i16);
_3 = _13.1;
_17 = 22588_i16;
_14 = _2;
_7 = _3 as u128;
_13.0 = core::ptr::addr_of_mut!(_10);
Goto(bb12)
}
bb14 = {
_6 = _15;
_6 = _8 as usize;
RET = _14;
_3 = _8 as u64;
_7 = 328178036951629227054645193568797192994_u128 & 186921705861352803078774298816054672321_u128;
_2 = RET;
_5 = _7 as u8;
_1 = true;
_3 = _13.1;
_17 = 10031_i16 * 29622_i16;
_14 = RET;
_7 = !272428014806481763717958002516203168356_u128;
_16 = -(-9223372036854775808_isize);
_8 = 41074_u16;
_1 = true;
_14 = _2;
_3 = _13.1;
_21 = (-75_i8) as f32;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(0_usize, 14_usize, Move(_14), 3_usize, Move(_3), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(0_usize, 6_usize, Move(_6), 4_usize, Move(_4), 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: char,mut _3: char,mut _4: bool,mut _5: char,mut _6: bool,mut _7: char,mut _8: char,mut _9: bool,mut _10: char) -> u8 {
mir! {
type RET = u8;
let _11: bool;
let _12: isize;
let _13: f32;
let _14: [i8; 3];
let _15: ([char; 7], [i8; 3], i32);
let _16: *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _17: ((i32,),);
let _18: [usize; 7];
let _19: char;
let _20: &'static [i32; 1];
let _21: (([i32; 1], [u8; 8]), *const u8, &'static i32, &'static [u128; 7]);
let _22: *mut ([u8; 8], i128, [i32; 1], bool);
let _23: isize;
let _24: [char; 8];
let _25: [usize; 5];
let _26: f64;
let _27: [u8; 8];
let _28: f32;
let _29: [i64; 5];
let _30: f64;
let _31: &'static Adt17;
let _32: *mut [i32; 7];
let _33: i64;
let _34: i8;
let _35: &'static [i32; 1];
let _36: isize;
let _37: [i8; 3];
let _38: f32;
let _39: u64;
let _40: &'static [i32; 1];
let _41: char;
let _42: ();
let _43: ();
{
_4 = _9;
_7 = _3;
_8 = _10;
_10 = _2;
Goto(bb1)
}
bb1 = {
_2 = _1;
_12 = -9223372036854775807_isize;
RET = _12 as u8;
_13 = 405216433_i32 as f32;
RET = 91_u8;
_8 = _1;
_8 = _5;
_9 = _6;
RET = (-1035672219_i32) as u8;
_5 = _1;
_2 = _7;
_9 = !_4;
_9 = !_6;
_13 = 46374_u16 as f32;
RET = 33_u8 + 219_u8;
_10 = _2;
_10 = _3;
_13 = 58_i8 as f32;
Call(RET = core::intrinsics::bswap(58_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15.2 = 1086122299_i32;
Goto(bb3)
}
bb3 = {
_9 = _4;
_5 = _7;
_10 = _2;
_7 = _10;
_11 = !_6;
_13 = 39817147620896061723120409921234759538_u128 as f32;
_17.0.0 = -_15.2;
_17.0.0 = _15.2 + _15.2;
RET = 50_u8 * 4_u8;
_15.1 = [(-54_i8),105_i8,(-15_i8)];
_11 = !_9;
_11 = !_6;
_13 = 2390808835324370246_i64 as f32;
RET = !254_u8;
_3 = _5;
_2 = _8;
_14 = [82_i8,14_i8,(-101_i8)];
_12 = (-9223372036854775808_isize);
_15.0 = [_1,_8,_5,_1,_10,_1,_3];
_3 = _10;
Call(_15.0 = fn2(_17, _3, _15.1, _12, _12, _7, _17.0.0, _2, _9, _7, _3, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = _4;
_14 = [(-20_i8),(-112_i8),25_i8];
_15.2 = (-9939_i16) as i32;
_18 = [2_usize,6266547924796853273_usize,7_usize,3_usize,6194720068521099905_usize,12531698547386491945_usize,1_usize];
_17.0 = (_15.2,);
_5 = _10;
_3 = _10;
_6 = _4;
RET = !138_u8;
RET = 69_u8 ^ 234_u8;
_4 = _6;
_2 = _1;
_2 = _1;
_5 = _2;
_3 = _10;
_13 = _12 as f32;
_15.1 = [121_i8,109_i8,(-87_i8)];
_14 = [90_i8,(-20_i8),(-13_i8)];
_5 = _10;
_4 = _6;
RET = 226_u8 * 105_u8;
_3 = _7;
_11 = _9 | _4;
Goto(bb5)
}
bb5 = {
_21.1 = core::ptr::addr_of!(RET);
_15.0 = [_5,_5,_1,_10,_3,_8,_7];
match _12 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb6 = {
_6 = _4;
_14 = [(-20_i8),(-112_i8),25_i8];
_15.2 = (-9939_i16) as i32;
_18 = [2_usize,6266547924796853273_usize,7_usize,3_usize,6194720068521099905_usize,12531698547386491945_usize,1_usize];
_17.0 = (_15.2,);
_5 = _10;
_3 = _10;
_6 = _4;
RET = !138_u8;
RET = 69_u8 ^ 234_u8;
_4 = _6;
_2 = _1;
_2 = _1;
_5 = _2;
_3 = _10;
_13 = _12 as f32;
_15.1 = [121_i8,109_i8,(-87_i8)];
_14 = [90_i8,(-20_i8),(-13_i8)];
_5 = _10;
_4 = _6;
RET = 226_u8 * 105_u8;
_3 = _7;
_11 = _9 | _4;
Goto(bb5)
}
bb7 = {
_9 = _4;
_5 = _7;
_10 = _2;
_7 = _10;
_11 = !_6;
_13 = 39817147620896061723120409921234759538_u128 as f32;
_17.0.0 = -_15.2;
_17.0.0 = _15.2 + _15.2;
RET = 50_u8 * 4_u8;
_15.1 = [(-54_i8),105_i8,(-15_i8)];
_11 = !_9;
_11 = !_6;
_13 = 2390808835324370246_i64 as f32;
RET = !254_u8;
_3 = _5;
_2 = _8;
_14 = [82_i8,14_i8,(-101_i8)];
_12 = (-9223372036854775808_isize);
_15.0 = [_1,_8,_5,_1,_10,_1,_3];
_3 = _10;
Call(_15.0 = fn2(_17, _3, _15.1, _12, _12, _7, _17.0.0, _2, _9, _7, _3, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_15.2 = 1086122299_i32;
Goto(bb3)
}
bb9 = {
_8 = _7;
_7 = _3;
_14 = [68_i8,(-105_i8),(-45_i8)];
_15.2 = 10460408549449536112_u64 as i32;
_15.0 = [_2,_8,_10,_7,_2,_5,_10];
_3 = _5;
_10 = _8;
_15.0 = [_2,_3,_10,_7,_5,_8,_3];
_15.1 = [101_i8,(-23_i8),(-27_i8)];
_11 = _17.0.0 < _15.2;
_13 = (-71_i8) as f32;
_21.2 = &_17.0.0;
_24 = [_5,_1,_10,_7,_5,_2,_10,_5];
_4 = !_9;
_19 = _1;
_19 = _10;
_4 = !_11;
_15.2 = -_17.0.0;
_9 = _1 != _5;
_21.0.0 = [_17.0.0];
_4 = _12 == _12;
_15.2 = -_17.0.0;
_24 = [_19,_2,_5,_2,_1,_10,_8,_2];
_7 = _2;
Goto(bb10)
}
bb10 = {
_21.1 = core::ptr::addr_of!(RET);
_2 = _1;
_5 = _8;
_26 = 6338369489330272873_usize as f64;
_9 = _6 | _11;
_8 = _2;
_19 = _3;
_3 = _1;
_20 = &_21.0.0;
_9 = !_6;
_18 = [2_usize,11984666612386548626_usize,14699851012813556231_usize,1_usize,13977821249605454449_usize,13101320428071607461_usize,8495062164364346896_usize];
_5 = _8;
_11 = _6;
_28 = _13;
_9 = !_4;
_29 = [7205307244375367862_i64,7529271453536555852_i64,6353140540518415374_i64,(-7421519177967393144_i64),(-3792049057348526869_i64)];
_23 = _15.2 as isize;
_25 = [6969692285602854702_usize,10458811884025505767_usize,4_usize,3227411567366494094_usize,0_usize];
_21.0.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_24 = [_19,_1,_8,_5,_19,_2,_3,_19];
_15.2 = _17.0.0;
_30 = _26 + _26;
_15.0 = [_2,_2,_19,_7,_10,_1,_1];
_27 = _21.0.1;
_15.2 = _17.0.0 << RET;
_15.0 = [_7,_5,_5,_7,_10,_8,_5];
_6 = _4;
Goto(bb11)
}
bb11 = {
_11 = _6;
_5 = _10;
match _12 {
0 => bb1,
340282366920938463454151235394913435648 => bb12,
_ => bb6
}
}
bb12 = {
_15.2 = 45857_u16 as i32;
_15.2 = _17.0.0 - _17.0.0;
_15.0 = [_1,_5,_3,_10,_5,_10,_8];
_3 = _19;
_10 = _8;
_15.2 = 162993994271822124957459617817243786663_u128 as i32;
_9 = _6 & _11;
_14 = [(-41_i8),95_i8,(-118_i8)];
_21.0.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_15.1 = [2_i8,29_i8,(-23_i8)];
_2 = _5;
_10 = _19;
_23 = _12 ^ _12;
_23 = 1602293665_u32 as isize;
_1 = _2;
_8 = _19;
_2 = _19;
_26 = _30;
_25 = [4_usize,12345677606457023874_usize,7_usize,8121110115359952795_usize,6_usize];
_18 = [2371091562624661469_usize,6_usize,3_usize,7_usize,11716200753968438076_usize,16881503132452809049_usize,17622123893811878030_usize];
Goto(bb13)
}
bb13 = {
_15.2 = -_17.0.0;
Call(_34 = core::intrinsics::bswap((-97_i8)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = !147_u8;
_21.2 = &_17.0.0;
_17.0 = (_15.2,);
_27 = [RET,RET,RET,RET,RET,RET,RET,RET];
_30 = _26 * _26;
_19 = _2;
_35 = &(*_20);
_28 = _13;
_9 = !_11;
_13 = _28 - _28;
_2 = _3;
_17.0 = (_15.2,);
_3 = _19;
_6 = !_11;
_36 = 7_i8 as isize;
_33 = (-295349366833650085_i64) & (-2438345749023837690_i64);
_15.2 = 5933510876119445416_u64 as i32;
_15.1 = _14;
_21.0.0 = [_15.2];
_21.2 = &_17.0.0;
_15.2 = -_17.0.0;
_21.2 = &_15.2;
_5 = _7;
_21.0.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(1_usize, 5_usize, Move(_5), 29_usize, Move(_29), 25_usize, Move(_25), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(1_usize, 6_usize, Move(_6), 27_usize, Move(_27), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(1_usize, 3_usize, Move(_3), 36_usize, Move(_36), 18_usize, Move(_18), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: ((i32,),),mut _2: char,mut _3: [i8; 3],mut _4: isize,mut _5: isize,mut _6: char,mut _7: i32,mut _8: char,mut _9: bool,mut _10: char,mut _11: char,mut _12: char) -> [char; 7] {
mir! {
type RET = [char; 7];
let _13: f64;
let _14: char;
let _15: i16;
let _16: ([u8; 8], i128, [i32; 1], bool);
let _17: i64;
let _18: [i32; 1];
let _19: Adt39;
let _20: char;
let _21: i8;
let _22: isize;
let _23: *const bool;
let _24: &'static u16;
let _25: Adt40;
let _26: usize;
let _27: &'static u16;
let _28: isize;
let _29: *mut &'static (i32,);
let _30: u128;
let _31: i16;
let _32: &'static i32;
let _33: f64;
let _34: &'static (i32,);
let _35: f64;
let _36: [i8; 3];
let _37: usize;
let _38: isize;
let _39: u64;
let _40: isize;
let _41: *mut &'static (i32,);
let _42: ((&'static u8,),);
let _43: u128;
let _44: ();
let _45: ();
{
_5 = !_4;
_12 = _2;
_3 = [5_i8,88_i8,(-127_i8)];
_13 = 93369224891203887185841924819833051438_u128 as f64;
_1.0.0 = _7;
_2 = _10;
_7 = 163682676588831426351040117334640610338_i128 as i32;
RET = [_6,_2,_12,_12,_11,_8,_10];
_8 = _10;
_12 = _10;
_8 = _10;
_10 = _6;
_4 = !_5;
_14 = _11;
_5 = 143693561739994248935869208028381202774_i128 as isize;
_2 = _6;
Call(_3 = fn3(_9, _2, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = 2_usize as f64;
_6 = _14;
_15 = 3857795562_u32 as i16;
_17 = 3081902449695479711_i64;
_8 = _2;
_15 = _9 as i16;
_15 = -(-177_i16);
_15 = (-25686_i16) << _1.0.0;
_16.0 = [118_u8,146_u8,118_u8,113_u8,110_u8,170_u8,26_u8,135_u8];
RET = [_10,_10,_8,_2,_12,_11,_14];
_5 = _4;
_16.0 = [54_u8,242_u8,188_u8,231_u8,255_u8,7_u8,44_u8,159_u8];
_16.1 = (-37922628790068918753066618026258949458_i128) >> _17;
_5 = _13 as isize;
RET = [_11,_2,_8,_8,_10,_2,_6];
RET = [_14,_2,_2,_2,_12,_2,_11];
_1.0 = (_7,);
_15 = 9337_i16 >> _16.1;
_14 = _8;
_1.0.0 = -_7;
_8 = _11;
_9 = false ^ true;
_14 = _2;
_1.0.0 = _7;
match _17 {
0 => bb2,
3081902449695479711 => bb4,
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
RET = [_14,_12,_14,_12,_12,_8,_10];
_16.3 = !_9;
_23 = core::ptr::addr_of!(_16.3);
_22 = -_5;
_14 = _8;
RET = [_6,_8,_12,_2,_6,_14,_12];
_3 = [(-46_i8),123_i8,95_i8];
_16.3 = !_9;
_16.1 = (-95716520361203231234732584790018955514_i128);
_20 = _6;
(*_23) = _9;
_5 = !_4;
_16.3 = _9 | _9;
_16.2 = [_7];
RET = [_20,_20,_6,_8,_12,_6,_14];
_21 = 34_i8 * 70_i8;
_16.3 = !_9;
_11 = _14;
RET = [_2,_8,_12,_20,_2,_11,_11];
_5 = _4 >> _15;
_22 = 2_usize as isize;
(*_23) = !_9;
_3 = [_21,_21,_21];
_16.1 = (-161250734012150062808898032622182145164_i128);
(*_23) = _9;
_4 = _5;
Goto(bb5)
}
bb5 = {
(*_23) = _7 < _1.0.0;
_9 = !(*_23);
_16.2 = [_1.0.0];
(*_23) = !_9;
_21 = -112_i8;
RET = [_8,_8,_2,_14,_10,_2,_14];
_9 = !(*_23);
_7 = !_1.0.0;
_12 = _8;
_25 = Adt40::Variant1 { fld0: 381783619_u32,fld1: 332633321676787769666555667855816582785_u128,fld2: _16.2,fld3: 4_usize,fld4: _15,fld5: 1358371159221246730_u64,fld6: _17,fld7: _16.1 };
_16.1 = Field::<i128>(Variant(_25, 1), 7) << _7;
_8 = _12;
_18 = [_7];
_26 = 5_usize;
RET[_26] = _20;
place!(Field::<u64>(Variant(_25, 1), 5)) = 3653386853_u32 as u64;
_6 = _8;
match _16.0[_26] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
7 => bb10,
_ => bb9
}
}
bb6 = {
RET = [_14,_12,_14,_12,_12,_8,_10];
_16.3 = !_9;
_23 = core::ptr::addr_of!(_16.3);
_22 = -_5;
_14 = _8;
RET = [_6,_8,_12,_2,_6,_14,_12];
_3 = [(-46_i8),123_i8,95_i8];
_16.3 = !_9;
_16.1 = (-95716520361203231234732584790018955514_i128);
_20 = _6;
(*_23) = _9;
_5 = !_4;
_16.3 = _9 | _9;
_16.2 = [_7];
RET = [_20,_20,_6,_8,_12,_6,_14];
_21 = 34_i8 * 70_i8;
_16.3 = !_9;
_11 = _14;
RET = [_2,_8,_12,_20,_2,_11,_11];
_5 = _4 >> _15;
_22 = 2_usize as isize;
(*_23) = !_9;
_3 = [_21,_21,_21];
_16.1 = (-161250734012150062808898032622182145164_i128);
(*_23) = _9;
_4 = _5;
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_13 = 2_usize as f64;
_6 = _14;
_15 = 3857795562_u32 as i16;
_17 = 3081902449695479711_i64;
_8 = _2;
_15 = _9 as i16;
_15 = -(-177_i16);
_15 = (-25686_i16) << _1.0.0;
_16.0 = [118_u8,146_u8,118_u8,113_u8,110_u8,170_u8,26_u8,135_u8];
RET = [_10,_10,_8,_2,_12,_11,_14];
_5 = _4;
_16.0 = [54_u8,242_u8,188_u8,231_u8,255_u8,7_u8,44_u8,159_u8];
_16.1 = (-37922628790068918753066618026258949458_i128) >> _17;
_5 = _13 as isize;
RET = [_11,_2,_8,_8,_10,_2,_6];
RET = [_14,_2,_2,_2,_12,_2,_11];
_1.0 = (_7,);
_15 = 9337_i16 >> _16.1;
_14 = _8;
_1.0.0 = -_7;
_8 = _11;
_9 = false ^ true;
_14 = _2;
_1.0.0 = _7;
match _17 {
0 => bb2,
3081902449695479711 => bb4,
_ => bb3
}
}
bb10 = {
_17 = Field::<i64>(Variant(_25, 1), 6);
_25 = Adt40::Variant1 { fld0: 2012496546_u32,fld1: 262910911108353712289538464288458908809_u128,fld2: _18,fld3: _26,fld4: _15,fld5: 1905737521982503190_u64,fld6: _17,fld7: _16.1 };
_2 = _12;
_11 = _20;
_16.0 = [48_u8,97_u8,126_u8,114_u8,153_u8,238_u8,151_u8,229_u8];
_1.0 = (_7,);
_16.3 = _5 < _4;
place!(Field::<u128>(Variant(_25, 1), 1)) = 37311473065755986884485815085306006675_u128 + 125565829711776239330936166564547801754_u128;
_12 = _14;
(*_23) = !_9;
_16.1 = _12 as i128;
RET[_26] = _10;
_16.2 = [_7];
_11 = _6;
_23 = core::ptr::addr_of!((*_23));
_1.0.0 = _7;
Goto(bb11)
}
bb11 = {
_9 = _5 == _4;
_16.0 = [65_u8,110_u8,219_u8,2_u8,172_u8,5_u8,77_u8,12_u8];
place!(Field::<u64>(Variant(_25, 1), 5)) = 12284426094492704708_u64;
_33 = _13 * _13;
_16.2 = [_7];
_26 = !Field::<usize>(Variant(_25, 1), 3);
_32 = &_7;
place!(Field::<i64>(Variant(_25, 1), 6)) = _17 & _17;
(*_23) = !_9;
_28 = _4;
(*_23) = !_9;
_3 = [_21,_21,_21];
_10 = _12;
place!(Field::<[i32; 1]>(Variant(_25, 1), 2)) = [_1.0.0];
_30 = Field::<u128>(Variant(_25, 1), 1);
_5 = Field::<u128>(Variant(_25, 1), 1) as isize;
_7 = _1.0.0;
_34 = &_1.0;
_12 = _10;
_8 = _10;
RET = [_6,_8,_2,_2,_12,_20,_2];
_31 = !_15;
_1.0 = (_7,);
(*_23) = _9 & _9;
_38 = -_4;
_1.0 = (_7,);
Goto(bb12)
}
bb12 = {
_15 = Field::<i16>(Variant(_25, 1), 4) << _1.0.0;
_29 = core::ptr::addr_of_mut!(_34);
_3 = [_21,_21,_21];
_34 = &_1.0;
place!(Field::<i128>(Variant(_25, 1), 7)) = _16.1;
_39 = _28 as u64;
place!(Field::<i64>(Variant(_25, 1), 6)) = -_17;
_37 = _28 as usize;
match Field::<usize>(Variant(_25, 1), 3) {
0 => bb1,
1 => bb8,
2 => bb13,
3 => bb14,
5 => bb16,
_ => bb15
}
}
bb13 = {
(*_23) = _7 < _1.0.0;
_9 = !(*_23);
_16.2 = [_1.0.0];
(*_23) = !_9;
_21 = -112_i8;
RET = [_8,_8,_2,_14,_10,_2,_14];
_9 = !(*_23);
_7 = !_1.0.0;
_12 = _8;
_25 = Adt40::Variant1 { fld0: 381783619_u32,fld1: 332633321676787769666555667855816582785_u128,fld2: _16.2,fld3: 4_usize,fld4: _15,fld5: 1358371159221246730_u64,fld6: _17,fld7: _16.1 };
_16.1 = Field::<i128>(Variant(_25, 1), 7) << _7;
_8 = _12;
_18 = [_7];
_26 = 5_usize;
RET[_26] = _20;
place!(Field::<u64>(Variant(_25, 1), 5)) = 3653386853_u32 as u64;
_6 = _8;
match _16.0[_26] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
7 => bb10,
_ => bb9
}
}
bb14 = {
_17 = Field::<i64>(Variant(_25, 1), 6);
_25 = Adt40::Variant1 { fld0: 2012496546_u32,fld1: 262910911108353712289538464288458908809_u128,fld2: _18,fld3: _26,fld4: _15,fld5: 1905737521982503190_u64,fld6: _17,fld7: _16.1 };
_2 = _12;
_11 = _20;
_16.0 = [48_u8,97_u8,126_u8,114_u8,153_u8,238_u8,151_u8,229_u8];
_1.0 = (_7,);
_16.3 = _5 < _4;
place!(Field::<u128>(Variant(_25, 1), 1)) = 37311473065755986884485815085306006675_u128 + 125565829711776239330936166564547801754_u128;
_12 = _14;
(*_23) = !_9;
_16.1 = _12 as i128;
RET[_26] = _10;
_16.2 = [_7];
_11 = _6;
_23 = core::ptr::addr_of!((*_23));
_1.0.0 = _7;
Goto(bb11)
}
bb15 = {
Return()
}
bb16 = {
_16.1 = _13 as i128;
_15 = !Field::<i16>(Variant(_25, 1), 4);
place!(Field::<usize>(Variant(_25, 1), 3)) = _30 as usize;
_1.0 = (_7,);
_10 = _11;
_34 = &_1.0;
_32 = &_1.0.0;
_25 = Adt40::Variant2 { fld0: (*_23),fld1: (*_34),fld2: _1,fld3: 206_u8,fld4: _30 };
_34 = &(*_34);
_13 = _33 * _33;
RET = [_8,_8,_8,_10,_11,_12,_14];
Goto(bb17)
}
bb17 = {
Call(_44 = dump_var(2_usize, 15_usize, Move(_15), 30_usize, Move(_30), 26_usize, Move(_26), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(2_usize, 4_usize, Move(_4), 16_usize, Move(_16), 22_usize, Move(_22), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(2_usize, 28_usize, Move(_28), 14_usize, Move(_14), 21_usize, Move(_21), 37_usize, Move(_37)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(2_usize, 1_usize, Move(_1), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: char,mut _3: char) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _4: *mut &'static (i32,);
let _5: i64;
let _6: (&'static u8,);
let _7: i8;
let _8: ([i32; 1], char);
let _9: *mut &'static Adt17;
let _10: i128;
let _11: u16;
let _12: f64;
let _13: char;
let _14: &'static Adt17;
let _15: f32;
let _16: *const *mut [i32; 7];
let _17: f32;
let _18: &'static &'static u16;
let _19: char;
let _20: ((i32,),);
let _21: [usize; 5];
let _22: &'static *mut i128;
let _23: u32;
let _24: char;
let _25: u64;
let _26: u32;
let _27: bool;
let _28: *mut ([u8; 8], i128, [i32; 1], bool);
let _29: char;
let _30: [isize; 6];
let _31: u16;
let _32: [isize; 6];
let _33: u16;
let _34: isize;
let _35: i128;
let _36: isize;
let _37: ();
let _38: ();
{
_2 = _3;
RET = [16_i8,(-13_i8),126_i8];
RET = [(-90_i8),(-92_i8),(-66_i8)];
_2 = _3;
_1 = false;
_2 = _3;
_3 = _2;
_3 = _2;
_1 = true;
_5 = -6354170514528450523_i64;
_3 = _2;
_1 = _3 <= _3;
RET = [55_i8,(-18_i8),(-123_i8)];
_2 = _3;
_5 = !(-950574217460524448_i64);
_5 = 1880095914_i32 as i64;
_5 = (-5346579208295839_i64);
_3 = _2;
_1 = !false;
_5 = (-156182328707745801255353951003436789250_i128) as i64;
Goto(bb1)
}
bb1 = {
_1 = true;
RET = [49_i8,64_i8,2_i8];
RET = [(-109_i8),83_i8,(-19_i8)];
_2 = _3;
_2 = _3;
_3 = _2;
_5 = 164_u8 as i64;
_3 = _2;
_5 = !3232474623434473631_i64;
Call(RET = fn4(_2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _2;
RET = [(-118_i8),86_i8,(-82_i8)];
_5 = _1 as i64;
_1 = false | true;
_1 = true;
_2 = _3;
RET = [(-53_i8),(-76_i8),(-98_i8)];
RET = [80_i8,46_i8,18_i8];
RET = [(-64_i8),(-50_i8),39_i8];
_2 = _3;
_2 = _3;
_1 = !false;
RET = [105_i8,31_i8,(-82_i8)];
_7 = -(-10_i8);
RET = [_7,_7,_7];
_8.0 = [(-22867499_i32)];
_3 = _2;
_3 = _2;
_5 = (-4452886934213107488_i64) + 2194887682249369616_i64;
_8.1 = _3;
Goto(bb3)
}
bb3 = {
_1 = false;
_5 = !4856360290588037901_i64;
_8.1 = _3;
_2 = _3;
_1 = _7 != _7;
_5 = (-4817849303485830895_i64);
_11 = _7 as u16;
_12 = _11 as f64;
_8.1 = _3;
_8.0 = [1994591929_i32];
_1 = true;
_3 = _2;
_12 = 107016179472572520147884220150490117363_i128 as f64;
_13 = _8.1;
_12 = 8090103204751646655_u64 as f64;
_9 = core::ptr::addr_of_mut!(_14);
_10 = (-59278811665685306601599416536868609684_i128);
_13 = _2;
match _10 {
0 => bb4,
1 => bb5,
281003555255253156861775190894899601772 => bb7,
_ => bb6
}
}
bb4 = {
_3 = _2;
RET = [(-118_i8),86_i8,(-82_i8)];
_5 = _1 as i64;
_1 = false | true;
_1 = true;
_2 = _3;
RET = [(-53_i8),(-76_i8),(-98_i8)];
RET = [80_i8,46_i8,18_i8];
RET = [(-64_i8),(-50_i8),39_i8];
_2 = _3;
_2 = _3;
_1 = !false;
RET = [105_i8,31_i8,(-82_i8)];
_7 = -(-10_i8);
RET = [_7,_7,_7];
_8.0 = [(-22867499_i32)];
_3 = _2;
_3 = _2;
_5 = (-4452886934213107488_i64) + 2194887682249369616_i64;
_8.1 = _3;
Goto(bb3)
}
bb5 = {
_1 = true;
RET = [49_i8,64_i8,2_i8];
RET = [(-109_i8),83_i8,(-19_i8)];
_2 = _3;
_2 = _3;
_3 = _2;
_5 = 164_u8 as i64;
_3 = _2;
_5 = !3232474623434473631_i64;
Call(RET = fn4(_2, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_9 = core::ptr::addr_of_mut!((*_9));
_11 = 61992_u16 * 27785_u16;
_3 = _8.1;
_2 = _8.1;
_3 = _8.1;
_8.0 = [(-1437136771_i32)];
_1 = !false;
RET = [_7,_7,_7];
_2 = _13;
_11 = !58244_u16;
_8.0 = [(-93095564_i32)];
RET = [_7,_7,_7];
RET = [_7,_7,_7];
_11 = 15968_u16 - 14317_u16;
_2 = _3;
_7 = (-121_i8) | (-58_i8);
_15 = 10879019725499726532_usize as f32;
_8.0 = [(-1744770434_i32)];
_13 = _8.1;
_15 = _7 as f32;
RET = [_7,_7,_7];
_11 = 15556_u16;
_10 = !(-94058401657034171477912674549559533963_i128);
_3 = _8.1;
RET = [_7,_7,_7];
RET = [_7,_7,_7];
_8.0 = [1045352158_i32];
_7 = 14_i8 >> _5;
Goto(bb8)
}
bb8 = {
_5 = 6914572353155764435_i64 & (-1890273697873206035_i64);
_8.0 = [(-694345666_i32)];
_13 = _8.1;
_17 = -_15;
_15 = _17 + _17;
_12 = 550527268_i32 as f64;
_3 = _2;
RET = [_7,_7,_7];
_1 = true ^ false;
_8.0 = [1527244277_i32];
_12 = 9223372036854775807_isize as f64;
RET = [_7,_7,_7];
_10 = 60528920269349185276075491448153903088_i128 >> _7;
_12 = 40691318825590196288474978666486001498_u128 as f64;
_20.0 = (479262924_i32,);
_1 = true;
Goto(bb9)
}
bb9 = {
_15 = _12 as f32;
_7 = 40_i8;
_20.0.0 = 4201_i16 as i32;
_12 = _10 as f64;
_3 = _13;
_20.0 = ((-1129669363_i32),);
_12 = _10 as f64;
_15 = _17 + _17;
_2 = _8.1;
_3 = _2;
_20.0.0 = _5 as i32;
_9 = core::ptr::addr_of_mut!((*_9));
_1 = false;
RET = [_7,_7,_7];
match _11 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
15556 => bb10,
_ => bb7
}
}
bb10 = {
RET = [_7,_7,_7];
_23 = 4185346615_u32 - 2516457911_u32;
_11 = !55042_u16;
_11 = !46710_u16;
_15 = _17 + _17;
_29 = _13;
_9 = core::ptr::addr_of_mut!(_14);
_30 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,52_isize,4_isize,(-9223372036854775808_isize)];
_21 = [2360582831943016859_usize,4_usize,0_usize,2_usize,8892352507088645340_usize];
_2 = _13;
_31 = _7 as u16;
_5 = 8901321976779053838_i64 | 2117339747030413324_i64;
_13 = _2;
_13 = _3;
_19 = _29;
_27 = _10 >= _10;
_33 = 73_u8 as u16;
Goto(bb11)
}
bb11 = {
_12 = _31 as f64;
_12 = _20.0.0 as f64;
_9 = core::ptr::addr_of_mut!((*_9));
Goto(bb12)
}
bb12 = {
_11 = _20.0.0 as u16;
_3 = _19;
_23 = 120760270409098752965406108676377120825_u128 as u32;
_32 = _30;
_31 = (-9223372036854775808_isize) as u16;
_25 = !14564679899189320774_u64;
_23 = 4119221729_u32 >> _5;
_32 = [9223372036854775807_isize,66_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-110_isize),(-9223372036854775808_isize)];
_8.1 = _3;
_24 = _2;
_20.0 = ((-1191696278_i32),);
_25 = !17465685766816732349_u64;
RET = [_7,_7,_7];
_34 = !(-9223372036854775808_isize);
_26 = _7 as u32;
RET = [_7,_7,_7];
_21 = [6_usize,12406092721082184807_usize,3133364476961454408_usize,15675683155034102776_usize,11726067838177189880_usize];
_20.0 = (1979101376_i32,);
_11 = _33 - _31;
_20.0 = (98198416_i32,);
_17 = -_15;
Call(_26 = core::intrinsics::bswap(_23), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3 = _2;
_32 = [_34,_34,_34,_34,_34,_34];
Goto(bb14)
}
bb14 = {
_9 = core::ptr::addr_of_mut!(_14);
_36 = -_34;
_9 = core::ptr::addr_of_mut!(_14);
_5 = (-5679895681403771761_i64);
_35 = -_10;
_3 = _19;
_10 = _17 as i128;
_3 = _19;
_17 = -_15;
_31 = !_11;
_1 = !_27;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(3_usize, 36_usize, Move(_36), 2_usize, Move(_2), 32_usize, Move(_32), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(3_usize, 5_usize, Move(_5), 13_usize, Move(_13), 34_usize, Move(_34), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(3_usize, 29_usize, Move(_29), 24_usize, Move(_24), 7_usize, Move(_7), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: char,mut _2: char,mut _3: char) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _4: &'static i32;
let _5: isize;
let _6: (([i32; 1], [u8; 8]), *const u8, &'static i32, &'static [u128; 7]);
let _7: i64;
let _8: u64;
let _9: (i16, u128);
let _10: ([i32; 1], &'static i16, i64, [char; 7]);
let _11: (*const Adt17, u64, *const bool, *mut &'static Adt17);
let _12: bool;
let _13: i16;
let _14: isize;
let _15: &'static u16;
let _16: i64;
let _17: *mut &'static Adt17;
let _18: f32;
let _19: &'static i32;
let _20: isize;
let _21: ([u128; 7], (&'static u8,), &'static u8);
let _22: f32;
let _23: *const Adt17;
let _24: isize;
let _25: f64;
let _26: &'static u16;
let _27: f64;
let _28: f32;
let _29: ();
let _30: ();
{
_3 = _1;
_3 = _1;
RET = [(-115_i8),(-28_i8),(-46_i8)];
_5 = (-20503_i16) as isize;
RET = [76_i8,(-99_i8),(-97_i8)];
_1 = _3;
RET = [(-8_i8),(-109_i8),(-111_i8)];
RET = [68_i8,(-7_i8),(-93_i8)];
_6.0.1 = [22_u8,131_u8,67_u8,209_u8,219_u8,227_u8,132_u8,87_u8];
_1 = _2;
_7 = true as i64;
RET = [69_i8,10_i8,(-78_i8)];
_3 = _2;
_1 = _3;
Goto(bb1)
}
bb1 = {
_1 = _3;
_3 = _1;
RET = [11_i8,44_i8,52_i8];
_6.0.0 = [1686653457_i32];
_6.0.1 = [193_u8,88_u8,206_u8,91_u8,169_u8,167_u8,234_u8,211_u8];
_1 = _2;
RET = [16_i8,81_i8,(-65_i8)];
_6.0.0 = [490207609_i32];
_7 = (-4993025942659934379_i64) >> _5;
_6.0.1 = [50_u8,90_u8,217_u8,189_u8,89_u8,68_u8,59_u8,202_u8];
_6.0.1 = [191_u8,137_u8,113_u8,25_u8,166_u8,101_u8,41_u8,74_u8];
_1 = _3;
_9.0 = 27584_i16;
_9.1 = 139124722431573408118931558228474840214_u128 - 339109346170299819436783923425141600403_u128;
_9.1 = 168058744686882505904814628794594029898_u128 >> _7;
_6.0.0 = [1549487847_i32];
_9.0 = 27097_i16;
_9 = ((-29360_i16), 179558015143251087720072938026508257222_u128);
_5 = 8_isize;
RET = [103_i8,13_i8,(-71_i8)];
_11.2 = core::ptr::addr_of!(_12);
_12 = true;
_11.1 = (-142196672_i32) as u64;
_3 = _2;
_3 = _1;
Call(_8 = fn5(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6.0.1 = [244_u8,189_u8,217_u8,121_u8,73_u8,210_u8,55_u8,201_u8];
_3 = _1;
RET = [64_i8,112_i8,(-46_i8)];
_7 = !(-1116221018653318756_i64);
_2 = _3;
_10.0 = [(-227343564_i32)];
_10.1 = &_13;
_10.2 = _7;
_10.3 = [_3,_2,_1,_3,_1,_3,_2];
_6.0.1 = [7_u8,212_u8,30_u8,31_u8,67_u8,66_u8,135_u8,35_u8];
_10.1 = &_9.0;
_11.2 = core::ptr::addr_of!(_12);
_1 = _3;
_5 = -(-30_isize);
Call(_11 = fn14(_12, _6.0.1, _6.0.1, _6.0.1, RET, RET, _12, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.0.0 = [1900873124_i32];
_9.1 = 214734169785683709676902828048598294380_u128 >> _8;
_3 = _1;
_14 = _5;
RET = [106_i8,(-83_i8),86_i8];
_8 = _11.1;
_1 = _2;
_1 = _2;
_10.1 = &_9.0;
_9.0 = (-23805_i16);
_13 = _1 as i16;
_9.0 = _13;
_10.3 = [_2,_3,_3,_2,_2,_1,_1];
_2 = _3;
Goto(bb4)
}
bb4 = {
_8 = _11.1;
_6.0.0 = _10.0;
_6.0.1 = [150_u8,9_u8,152_u8,18_u8,113_u8,175_u8,180_u8,89_u8];
_8 = _11.1 & _11.1;
_2 = _3;
_1 = _3;
_7 = !_10.2;
_9.0 = 40_u8 as i16;
RET = [(-97_i8),(-78_i8),(-41_i8)];
_9.0 = _13 ^ _13;
_10.0 = _6.0.0;
_11.1 = _8 >> _14;
_11.1 = !_8;
Goto(bb5)
}
bb5 = {
_10.3 = [_2,_3,_1,_2,_2,_2,_1];
_13 = _9.0 >> _14;
_2 = _3;
_3 = _2;
_10.1 = &_9.0;
_16 = _11.1 as i64;
_6.0.0 = [(-1701967579_i32)];
_8 = 158122748992976432686370968373773158121_i128 as u64;
_6.0.1 = [226_u8,212_u8,156_u8,188_u8,138_u8,10_u8,219_u8,107_u8];
_9 = (_13, 225051285858947070180818899636838154984_u128);
_20 = !_5;
_18 = 29001_u16 as f32;
_10.1 = &_9.0;
RET = [(-66_i8),40_i8,(-57_i8)];
match _9.1 {
0 => bb1,
1 => bb2,
2 => bb3,
225051285858947070180818899636838154984 => bb7,
_ => bb6
}
}
bb6 = {
_1 = _3;
_3 = _1;
RET = [11_i8,44_i8,52_i8];
_6.0.0 = [1686653457_i32];
_6.0.1 = [193_u8,88_u8,206_u8,91_u8,169_u8,167_u8,234_u8,211_u8];
_1 = _2;
RET = [16_i8,81_i8,(-65_i8)];
_6.0.0 = [490207609_i32];
_7 = (-4993025942659934379_i64) >> _5;
_6.0.1 = [50_u8,90_u8,217_u8,189_u8,89_u8,68_u8,59_u8,202_u8];
_6.0.1 = [191_u8,137_u8,113_u8,25_u8,166_u8,101_u8,41_u8,74_u8];
_1 = _3;
_9.0 = 27584_i16;
_9.1 = 139124722431573408118931558228474840214_u128 - 339109346170299819436783923425141600403_u128;
_9.1 = 168058744686882505904814628794594029898_u128 >> _7;
_6.0.0 = [1549487847_i32];
_9.0 = 27097_i16;
_9 = ((-29360_i16), 179558015143251087720072938026508257222_u128);
_5 = 8_isize;
RET = [103_i8,13_i8,(-71_i8)];
_11.2 = core::ptr::addr_of!(_12);
_12 = true;
_11.1 = (-142196672_i32) as u64;
_3 = _2;
_3 = _1;
Call(_8 = fn5(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_13 = _12 as i16;
_10.2 = _11.1 as i64;
_10.1 = &_9.0;
_6.3 = &_21.0;
_12 = _16 >= _7;
_22 = -_18;
_9 = (_13, 211801130806836626252831236747164988060_u128);
_7 = _10.2;
_10.1 = &_13;
_6.0.1 = [112_u8,8_u8,79_u8,163_u8,95_u8,56_u8,25_u8,189_u8];
_13 = 3467813591_u32 as i16;
match _9.1 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb8,
211801130806836626252831236747164988060 => bb10,
_ => bb9
}
}
bb8 = {
_6.0.1 = [244_u8,189_u8,217_u8,121_u8,73_u8,210_u8,55_u8,201_u8];
_3 = _1;
RET = [64_i8,112_i8,(-46_i8)];
_7 = !(-1116221018653318756_i64);
_2 = _3;
_10.0 = [(-227343564_i32)];
_10.1 = &_13;
_10.2 = _7;
_10.3 = [_3,_2,_1,_3,_1,_3,_2];
_6.0.1 = [7_u8,212_u8,30_u8,31_u8,67_u8,66_u8,135_u8,35_u8];
_10.1 = &_9.0;
_11.2 = core::ptr::addr_of!(_12);
_1 = _3;
_5 = -(-30_isize);
Call(_11 = fn14(_12, _6.0.1, _6.0.1, _6.0.1, RET, RET, _12, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_10.3 = [_2,_3,_1,_2,_2,_2,_1];
_13 = _9.0 >> _14;
_2 = _3;
_3 = _2;
_10.1 = &_9.0;
_16 = _11.1 as i64;
_6.0.0 = [(-1701967579_i32)];
_8 = 158122748992976432686370968373773158121_i128 as u64;
_6.0.1 = [226_u8,212_u8,156_u8,188_u8,138_u8,10_u8,219_u8,107_u8];
_9 = (_13, 225051285858947070180818899636838154984_u128);
_20 = !_5;
_18 = 29001_u16 as f32;
_10.1 = &_9.0;
RET = [(-66_i8),40_i8,(-57_i8)];
match _9.1 {
0 => bb1,
1 => bb2,
2 => bb3,
225051285858947070180818899636838154984 => bb7,
_ => bb6
}
}
bb10 = {
_9.0 = -_13;
Goto(bb11)
}
bb11 = {
_24 = 1_usize as isize;
_24 = _5;
_12 = !true;
_21.0 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_20 = !_24;
_11.2 = core::ptr::addr_of!(_12);
RET = [(-118_i8),(-14_i8),(-34_i8)];
_10.1 = &_13;
_6.0.0 = [331579847_i32];
_10.2 = _16;
_25 = 121372292872195799767748266843981928456_i128 as f64;
match _9.1 {
0 => bb8,
1 => bb2,
2 => bb12,
3 => bb13,
211801130806836626252831236747164988060 => bb15,
_ => bb14
}
}
bb12 = {
_9.0 = -_13;
Goto(bb11)
}
bb13 = {
_1 = _3;
_3 = _1;
RET = [11_i8,44_i8,52_i8];
_6.0.0 = [1686653457_i32];
_6.0.1 = [193_u8,88_u8,206_u8,91_u8,169_u8,167_u8,234_u8,211_u8];
_1 = _2;
RET = [16_i8,81_i8,(-65_i8)];
_6.0.0 = [490207609_i32];
_7 = (-4993025942659934379_i64) >> _5;
_6.0.1 = [50_u8,90_u8,217_u8,189_u8,89_u8,68_u8,59_u8,202_u8];
_6.0.1 = [191_u8,137_u8,113_u8,25_u8,166_u8,101_u8,41_u8,74_u8];
_1 = _3;
_9.0 = 27584_i16;
_9.1 = 139124722431573408118931558228474840214_u128 - 339109346170299819436783923425141600403_u128;
_9.1 = 168058744686882505904814628794594029898_u128 >> _7;
_6.0.0 = [1549487847_i32];
_9.0 = 27097_i16;
_9 = ((-29360_i16), 179558015143251087720072938026508257222_u128);
_5 = 8_isize;
RET = [103_i8,13_i8,(-71_i8)];
_11.2 = core::ptr::addr_of!(_12);
_12 = true;
_11.1 = (-142196672_i32) as u64;
_3 = _2;
_3 = _1;
Call(_8 = fn5(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_6.0.1 = [244_u8,189_u8,217_u8,121_u8,73_u8,210_u8,55_u8,201_u8];
_3 = _1;
RET = [64_i8,112_i8,(-46_i8)];
_7 = !(-1116221018653318756_i64);
_2 = _3;
_10.0 = [(-227343564_i32)];
_10.1 = &_13;
_10.2 = _7;
_10.3 = [_3,_2,_1,_3,_1,_3,_2];
_6.0.1 = [7_u8,212_u8,30_u8,31_u8,67_u8,66_u8,135_u8,35_u8];
_10.1 = &_9.0;
_11.2 = core::ptr::addr_of!(_12);
_1 = _3;
_5 = -(-30_isize);
Call(_11 = fn14(_12, _6.0.1, _6.0.1, _6.0.1, RET, RET, _12, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_11.2 = core::ptr::addr_of!(_12);
_11.2 = core::ptr::addr_of!(_12);
_6.0.1 = [202_u8,129_u8,86_u8,25_u8,0_u8,6_u8,101_u8,54_u8];
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(4_usize, 3_usize, Move(_3), 9_usize, Move(_9), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(4_usize, 13_usize, Move(_13), 8_usize, Move(_8), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: char) -> u64 {
mir! {
type RET = u64;
let _2: &'static [u128; 7];
let _3: i64;
let _4: [usize; 7];
let _5: Adt82;
let _6: i16;
let _7: *const [char; 7];
let _8: [char; 8];
let _9: i16;
let _10: isize;
let _11: isize;
let _12: *const u8;
let _13: isize;
let _14: &'static (i16, u128);
let _15: *mut &'static (i32,);
let _16: f64;
let _17: ([char; 7], [i8; 3], i32);
let _18: [i64; 5];
let _19: f32;
let _20: *mut &'static Adt17;
let _21: Adt39;
let _22: ([i8; 3], u32, ([u8; 8], i128, [i32; 1], bool), *const [char; 7]);
let _23: u64;
let _24: [isize; 6];
let _25: char;
let _26: &'static u16;
let _27: [i64; 5];
let _28: Adt82;
let _29: Adt33;
let _30: u32;
let _31: ();
let _32: ();
{
_1 = '\u{81e24}';
RET = 15933160686705527182_u64;
_3 = (-4047461824091874612_i64) * 6342283275575277346_i64;
_3 = 8167217547524852373_i64;
RET = 66609900051091530232030175458138361437_i128 as u64;
RET = 11016055247526293065_u64;
RET = 2274950176488700945_u64 - 12734085475758826390_u64;
RET = 8250908812416136558_u64 - 2087849635980041441_u64;
_1 = '\u{e8027}';
_1 = '\u{10d24d}';
_1 = '\u{ced4c}';
_4 = [5_usize,6_usize,2_usize,4_usize,1_usize,16294865842833800014_usize,7_usize];
_3 = 493532287343368036_i64;
RET = 12417496991317681110_u64 ^ 13652315750814325066_u64;
_1 = '\u{80a1a}';
Call(_3 = fn6(_4, _1, _4, _4, RET, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = -4434814810232051374_i64;
_4 = [7_usize,1_usize,17223070612051675208_usize,3_usize,2_usize,1_usize,5173461922059261370_usize];
_1 = '\u{e76b7}';
_3 = 5345745279585675042_i64 | (-7310662351940374575_i64);
_4 = [509157806652935220_usize,5_usize,14617270400833541745_usize,0_usize,11501998934787656537_usize,5477607296697911629_usize,7_usize];
RET = !6837831382309422923_u64;
_4 = [11403925850815511111_usize,5_usize,3_usize,10247858750882401130_usize,1_usize,0_usize,15029336780159631613_usize];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb2)
}
bb2 = {
_6 = _1 as i16;
_9 = -_6;
RET = 15317225554595494470_u64 | 1876691644808120222_u64;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = [11864032336931607490_usize,15326258638428140471_usize,14085871823744827485_usize,17055925532531474999_usize,14301953507506395654_usize,2_usize,4_usize];
_9 = _6 * _6;
_1 = '\u{8925b}';
RET = (-25_i8) as u64;
_3 = !(-3018176052438025343_i64);
_9 = 2_usize as i16;
_9 = 12288241841289846349_usize as i16;
_1 = '\u{1fcd6}';
_9 = -_6;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = !16028100416352462721_u64;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb3)
}
bb3 = {
_3 = 1986660931737063506_i64;
RET = !6534776905135656774_u64;
_4 = [7_usize,3_usize,1_usize,4_usize,5_usize,4_usize,0_usize];
RET = 10267643247860774535_u64;
_10 = 9223372036854775807_isize;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = [6_usize,6_usize,4_usize,2279137377690634320_usize,7683747460503113659_usize,5_usize,1_usize];
_9 = _6 << _3;
_1 = '\u{7dea0}';
_9 = _6 >> _10;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_11 = _3 as isize;
match _3 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
1986660931737063506 => bb11,
_ => bb10
}
}
bb4 = {
_6 = _1 as i16;
_9 = -_6;
RET = 15317225554595494470_u64 | 1876691644808120222_u64;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = [11864032336931607490_usize,15326258638428140471_usize,14085871823744827485_usize,17055925532531474999_usize,14301953507506395654_usize,2_usize,4_usize];
_9 = _6 * _6;
_1 = '\u{8925b}';
RET = (-25_i8) as u64;
_3 = !(-3018176052438025343_i64);
_9 = 2_usize as i16;
_9 = 12288241841289846349_usize as i16;
_1 = '\u{1fcd6}';
_9 = -_6;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = !16028100416352462721_u64;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb3)
}
bb5 = {
_3 = -4434814810232051374_i64;
_4 = [7_usize,1_usize,17223070612051675208_usize,3_usize,2_usize,1_usize,5173461922059261370_usize];
_1 = '\u{e76b7}';
_3 = 5345745279585675042_i64 | (-7310662351940374575_i64);
_4 = [509157806652935220_usize,5_usize,14617270400833541745_usize,0_usize,11501998934787656537_usize,5477607296697911629_usize,7_usize];
RET = !6837831382309422923_u64;
_4 = [11403925850815511111_usize,5_usize,3_usize,10247858750882401130_usize,1_usize,0_usize,15029336780159631613_usize];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
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
_10 = _11 | _11;
RET = 4156138255397820039_u64;
_10 = _3 as isize;
_4 = [12749846723188662640_usize,0_usize,7_usize,17661853083954822718_usize,16759770768594050463_usize,6152821939888260592_usize,5_usize];
_11 = !_10;
_6 = _9 ^ _9;
_3 = 6502437945481887210_i64 | (-8005505332684987611_i64);
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = [10941613535661370323_usize,16145915504886586827_usize,8176577378360778656_usize,13770841129186700158_usize,132053675957247060_usize,8592812925370328305_usize,1232944323758997786_usize];
RET = 14179308694875180317_u64;
_1 = '\u{b9f83}';
_4 = [7_usize,4_usize,7_usize,2740050660633133465_usize,6_usize,6_usize,5539049137395525162_usize];
RET = !17324438059492230441_u64;
_1 = '\u{e1925}';
_4 = [5_usize,4_usize,17781730259951957517_usize,9719061710200753598_usize,5_usize,3_usize,8850424086386144912_usize];
_9 = true as i16;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_6 = _9;
_1 = '\u{103a48}';
_13 = RET as isize;
_10 = -_11;
_13 = _11;
Goto(bb12)
}
bb12 = {
_10 = _13;
_1 = '\u{dbd0a}';
_11 = _9 as isize;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_9 = !_6;
RET = 16786839254614835685_u64;
_6 = _9 | _9;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = 10792368435469437514_u64;
_7 = core::ptr::addr_of!(_17.0);
(*_7) = [_1,_1,_1,_1,_1,_1,_1];
_19 = _3 as f32;
(*_7) = [_1,_1,_1,_1,_1,_1,_1];
_3 = 12598_u16 as i64;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
(*_7) = [_1,_1,_1,_1,_1,_1,_1];
_22.2.3 = true;
_17.1 = [4_i8,121_i8,(-16_i8)];
_16 = 2440052012_u32 as f64;
_22.2.2 = [599793394_i32];
match RET {
0 => bb1,
1 => bb6,
2 => bb11,
3 => bb5,
10792368435469437514 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_6 = -_9;
_22.2.1 = -(-108115784339729561373419770000226170673_i128);
_17.2 = 1074431225_i32 >> _13;
_22.2.2 = [_17.2];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_19 = 42037_u16 as f32;
_1 = '\u{5aa7}';
_22.2.0 = [251_u8,6_u8,8_u8,52_u8,236_u8,55_u8,60_u8,20_u8];
_22.1 = _22.2.3 as u32;
_24 = [_11,_11,_13,_13,_10,_11];
_17.1 = [(-38_i8),38_i8,(-114_i8)];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = [5_usize,3970356151402958189_usize,7289127552795390333_usize,16570138899630823947_usize,1_usize,8743453090468247646_usize,7_usize];
_13 = -_11;
_1 = '\u{c36cb}';
_1 = '\u{411f5}';
_6 = -_9;
_22.0 = [43_i8,(-99_i8),59_i8];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_18 = [_3,_3,_3,_3,_3];
_22.2.1 = !(-37769338937394157713229852075822720066_i128);
RET = !14597938326262753957_u64;
_18 = [_3,_3,_3,_3,_3];
_13 = !_11;
_22.0 = [103_i8,(-101_i8),(-93_i8)];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(5_usize, 9_usize, Move(_9), 6_usize, Move(_6), 3_usize, Move(_3), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(5_usize, 11_usize, Move(_11), 13_usize, Move(_13), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [usize; 7],mut _2: char,mut _3: [usize; 7],mut _4: [usize; 7],mut _5: u64,mut _6: [usize; 7]) -> i64 {
mir! {
type RET = i64;
let _7: isize;
let _8: Adt76;
let _9: isize;
let _10: u64;
let _11: f64;
let _12: [usize; 5];
let _13: &'static u8;
let _14: isize;
let _15: char;
let _16: u32;
let _17: bool;
let _18: ([char; 7], [i8; 3], i32);
let _19: &'static u8;
let _20: &'static u16;
let _21: ();
let _22: ();
{
_4 = [16335600140527228007_usize,0_usize,1_usize,6_usize,9790169398217231821_usize,4_usize,15230044416734805930_usize];
_6 = [7_usize,9455773886276838010_usize,4315369436298361981_usize,5998251319588998724_usize,2362571507314977815_usize,7602772752938783395_usize,0_usize];
_1 = [6_usize,3_usize,10553786939266783560_usize,4801173206219686091_usize,7_usize,17257750947468694763_usize,16198272364499407671_usize];
RET = !(-5501795702473156161_i64);
RET = 7318680049604465052_i64;
_2 = '\u{aef1d}';
_9 = !(-96_isize);
_5 = 4693393332887660971_u64 + 10756348192941458334_u64;
_4 = [11401122372221481394_usize,4428780464741937308_usize,11674271877068821165_usize,17899222076959315053_usize,4_usize,5_usize,7_usize];
_2 = '\u{29a6a}';
Goto(bb1)
}
bb1 = {
RET = 604186965859085674_i64 >> _9;
_10 = !_5;
_9 = 9223372036854775807_isize;
_8.fld0 = 13941_u16;
RET = _10 as i64;
RET = -(-8124125438492969973_i64);
Call(_3 = fn7(_2, _6, _1, _4, _5, _10, _10, _6, _1, _10, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [12511569860926608344_usize,3_usize,3_usize,0_usize,11085517928057723133_usize,12407478098397573341_usize,3001310886457644175_usize];
_8 = Adt76 { fld0: 12589_u16 };
_3 = [16844467075534221828_usize,12636815844526794282_usize,6_usize,4_usize,6_usize,2_usize,1_usize];
_1 = [296195251065263748_usize,6_usize,4_usize,4_usize,9259969182470954780_usize,3441759494830385979_usize,10512784505353799087_usize];
_11 = 4045218310269005472429614444496131032_i128 as f64;
RET = (-1999056746979547001_i64) + 9033876629279206902_i64;
_8.fld0 = 20852_u16;
_7 = false as isize;
_8 = Adt76 { fld0: 44322_u16 };
_1 = [7_usize,5360660039706705616_usize,18344317230073560399_usize,16954448697647039564_usize,4346350646807565191_usize,17236710315772994562_usize,0_usize];
_12 = [1_usize,15599523272345085540_usize,1982472978872383947_usize,0_usize,12191786536571191002_usize];
_7 = _9;
_6 = [7496751683110997031_usize,5_usize,2_usize,7_usize,4_usize,4_usize,9957160476710183100_usize];
_3 = [7_usize,1031921697653423143_usize,13956588377760713084_usize,6_usize,1_usize,9812221582018248600_usize,2244525418404128748_usize];
_1 = _3;
_10 = _5 | _5;
Goto(bb3)
}
bb3 = {
_3 = [0_usize,4700973011056479236_usize,4_usize,3_usize,3_usize,3479637204412944236_usize,4_usize];
_10 = !_5;
_8 = Adt76 { fld0: 65216_u16 };
_7 = !_9;
_4 = _1;
_4 = _3;
_14 = -_7;
_9 = _7;
RET = -2327337779412572557_i64;
_8 = Adt76 { fld0: 50313_u16 };
_15 = _2;
_6 = [5_usize,8802599424162513827_usize,1_usize,3_usize,2_usize,1_usize,10474309564970055333_usize];
_6 = _4;
_5 = _10;
_8 = Adt76 { fld0: 10378_u16 };
_8.fld0 = true as u16;
_5 = !_10;
_4 = [7_usize,6_usize,1_usize,1999273275431550527_usize,6_usize,1_usize,2_usize];
_6 = [1_usize,3_usize,3_usize,6_usize,3_usize,7_usize,4_usize];
_10 = !_5;
Goto(bb4)
}
bb4 = {
_9 = _5 as isize;
_4 = [5598544931105082893_usize,1_usize,9444685874180539947_usize,18030865561210704214_usize,2_usize,3323059209759404900_usize,14828890703973795534_usize];
Goto(bb5)
}
bb5 = {
_7 = -_9;
_6 = [1364698190955612549_usize,378865925986682856_usize,1_usize,11002188922997538507_usize,8913435512627841615_usize,2_usize,9889210702672569599_usize];
_8.fld0 = 1302_u16;
_8 = Adt76 { fld0: 51554_u16 };
match _8.fld0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
51554 => bb13,
_ => bb12
}
}
bb6 = {
_9 = _5 as isize;
_4 = [5598544931105082893_usize,1_usize,9444685874180539947_usize,18030865561210704214_usize,2_usize,3323059209759404900_usize,14828890703973795534_usize];
Goto(bb5)
}
bb7 = {
_3 = [0_usize,4700973011056479236_usize,4_usize,3_usize,3_usize,3479637204412944236_usize,4_usize];
_10 = !_5;
_8 = Adt76 { fld0: 65216_u16 };
_7 = !_9;
_4 = _1;
_4 = _3;
_14 = -_7;
_9 = _7;
RET = -2327337779412572557_i64;
_8 = Adt76 { fld0: 50313_u16 };
_15 = _2;
_6 = [5_usize,8802599424162513827_usize,1_usize,3_usize,2_usize,1_usize,10474309564970055333_usize];
_6 = _4;
_5 = _10;
_8 = Adt76 { fld0: 10378_u16 };
_8.fld0 = true as u16;
_5 = !_10;
_4 = [7_usize,6_usize,1_usize,1999273275431550527_usize,6_usize,1_usize,2_usize];
_6 = [1_usize,3_usize,3_usize,6_usize,3_usize,7_usize,4_usize];
_10 = !_5;
Goto(bb4)
}
bb8 = {
_4 = [12511569860926608344_usize,3_usize,3_usize,0_usize,11085517928057723133_usize,12407478098397573341_usize,3001310886457644175_usize];
_8 = Adt76 { fld0: 12589_u16 };
_3 = [16844467075534221828_usize,12636815844526794282_usize,6_usize,4_usize,6_usize,2_usize,1_usize];
_1 = [296195251065263748_usize,6_usize,4_usize,4_usize,9259969182470954780_usize,3441759494830385979_usize,10512784505353799087_usize];
_11 = 4045218310269005472429614444496131032_i128 as f64;
RET = (-1999056746979547001_i64) + 9033876629279206902_i64;
_8.fld0 = 20852_u16;
_7 = false as isize;
_8 = Adt76 { fld0: 44322_u16 };
_1 = [7_usize,5360660039706705616_usize,18344317230073560399_usize,16954448697647039564_usize,4346350646807565191_usize,17236710315772994562_usize,0_usize];
_12 = [1_usize,15599523272345085540_usize,1982472978872383947_usize,0_usize,12191786536571191002_usize];
_7 = _9;
_6 = [7496751683110997031_usize,5_usize,2_usize,7_usize,4_usize,4_usize,9957160476710183100_usize];
_3 = [7_usize,1031921697653423143_usize,13956588377760713084_usize,6_usize,1_usize,9812221582018248600_usize,2244525418404128748_usize];
_1 = _3;
_10 = _5 | _5;
Goto(bb3)
}
bb9 = {
RET = 604186965859085674_i64 >> _9;
_10 = !_5;
_9 = 9223372036854775807_isize;
_8.fld0 = 13941_u16;
RET = _10 as i64;
RET = -(-8124125438492969973_i64);
Call(_3 = fn7(_2, _6, _1, _4, _5, _10, _10, _6, _1, _10, _4, _4), ReturnTo(bb2), UnwindUnreachable())
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
_4 = [2_usize,7_usize,2_usize,2428705814601108673_usize,11464219025673733154_usize,11440478334799973945_usize,0_usize];
_11 = RET as f64;
_2 = _15;
_15 = _2;
_16 = 1663552419_u32 >> _9;
RET = _8.fld0 as i64;
_2 = _15;
_8 = Adt76 { fld0: 47117_u16 };
_18.2 = _16 as i32;
_10 = _5 + _5;
_18.0 = [_15,_2,_15,_15,_15,_2,_2];
_2 = _15;
_11 = (-7671_i16) as f64;
_16 = 2396688196_u32;
match _16 {
0 => bb1,
1 => bb8,
2 => bb7,
3 => bb14,
2396688196 => bb16,
_ => bb15
}
}
bb14 = {
_4 = [12511569860926608344_usize,3_usize,3_usize,0_usize,11085517928057723133_usize,12407478098397573341_usize,3001310886457644175_usize];
_8 = Adt76 { fld0: 12589_u16 };
_3 = [16844467075534221828_usize,12636815844526794282_usize,6_usize,4_usize,6_usize,2_usize,1_usize];
_1 = [296195251065263748_usize,6_usize,4_usize,4_usize,9259969182470954780_usize,3441759494830385979_usize,10512784505353799087_usize];
_11 = 4045218310269005472429614444496131032_i128 as f64;
RET = (-1999056746979547001_i64) + 9033876629279206902_i64;
_8.fld0 = 20852_u16;
_7 = false as isize;
_8 = Adt76 { fld0: 44322_u16 };
_1 = [7_usize,5360660039706705616_usize,18344317230073560399_usize,16954448697647039564_usize,4346350646807565191_usize,17236710315772994562_usize,0_usize];
_12 = [1_usize,15599523272345085540_usize,1982472978872383947_usize,0_usize,12191786536571191002_usize];
_7 = _9;
_6 = [7496751683110997031_usize,5_usize,2_usize,7_usize,4_usize,4_usize,9957160476710183100_usize];
_3 = [7_usize,1031921697653423143_usize,13956588377760713084_usize,6_usize,1_usize,9812221582018248600_usize,2244525418404128748_usize];
_1 = _3;
_10 = _5 | _5;
Goto(bb3)
}
bb15 = {
Return()
}
bb16 = {
_18.1 = [7_i8,(-110_i8),38_i8];
_10 = _5;
_16 = _8.fld0 as u32;
_18.2 = 897435207_i32;
_14 = _7;
Goto(bb17)
}
bb17 = {
Call(_21 = dump_var(6_usize, 16_usize, Move(_16), 9_usize, Move(_9), 15_usize, Move(_15), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_21 = dump_var(6_usize, 4_usize, Move(_4), 5_usize, Move(_5), 3_usize, Move(_3), 22_usize, _22), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: char,mut _2: [usize; 7],mut _3: [usize; 7],mut _4: [usize; 7],mut _5: u64,mut _6: u64,mut _7: u64,mut _8: [usize; 7],mut _9: [usize; 7],mut _10: u64,mut _11: [usize; 7],mut _12: [usize; 7]) -> [usize; 7] {
mir! {
type RET = [usize; 7];
let _13: f64;
let _14: bool;
let _15: [char; 8];
let _16: [i64; 5];
let _17: i8;
let _18: *const *mut [i32; 7];
let _19: [i32; 7];
let _20: u16;
let _21: [char; 4];
let _22: i64;
let _23: [usize; 5];
let _24: u128;
let _25: *const [char; 7];
let _26: &'static [i32; 1];
let _27: f32;
let _28: *const [char; 4];
let _29: &'static f64;
let _30: char;
let _31: &'static *mut i128;
let _32: (i16, u128);
let _33: &'static *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _34: *const *mut [i32; 7];
let _35: [isize; 6];
let _36: f64;
let _37: (([u8; 8], i128, [i32; 1], bool), u8);
let _38: *const [char; 4];
let _39: i32;
let _40: u16;
let _41: *mut [u8; 8];
let _42: ((&'static u8,),);
let _43: [i32; 7];
let _44: i32;
let _45: &'static [char; 7];
let _46: i16;
let _47: char;
let _48: &'static &'static i16;
let _49: u8;
let _50: [usize; 5];
let _51: u32;
let _52: [i128; 6];
let _53: ();
let _54: ();
{
_8 = [11693320981224210456_usize,17200692541852833635_usize,2705821297446018903_usize,1_usize,9419911360649962781_usize,17478282283812396779_usize,0_usize];
_3 = [5379102036563550930_usize,5_usize,16353371985908630351_usize,3535608823771154939_usize,2745971714638990728_usize,6050944319047028481_usize,5_usize];
_12 = [1_usize,4_usize,9367000435785079882_usize,6_usize,6_usize,3_usize,2_usize];
RET = [7567495699854943666_usize,4259946568678771178_usize,7_usize,6_usize,0_usize,9890201873499809818_usize,4_usize];
_13 = 49382099393275602918010210512456341927_u128 as f64;
_6 = _10;
_5 = 1742890422501332346_i64 as u64;
RET = _3;
_6 = _7;
_5 = (-27194_i16) as u64;
_8 = _9;
_12 = [15929751248267357396_usize,6766349519539374362_usize,5589957945711340493_usize,16782780170060627480_usize,321134302356332647_usize,9934788702705759954_usize,11119122284956780753_usize];
RET = [11737441136947444820_usize,11581489064121954969_usize,3_usize,15558002715315581193_usize,6_usize,6_usize,16875200809031609576_usize];
_1 = '\u{e627}';
_11 = [7_usize,5_usize,4_usize,4_usize,18296018180082652238_usize,8207098994067310571_usize,16724926733252042945_usize];
_8 = [3_usize,6292811060226467079_usize,2_usize,1_usize,0_usize,2_usize,10057026548068477340_usize];
_20 = 35361_u16 + 41179_u16;
Goto(bb1)
}
bb1 = {
_11 = _9;
_3 = _12;
_7 = _5 | _10;
_10 = _5 + _7;
_7 = _13 as u64;
_12 = [14621341423171177522_usize,12104585795206817881_usize,2_usize,3_usize,11552853562572283224_usize,1042909140132529532_usize,6_usize];
_22 = (-6494533898889700831_i64) * (-2015910832836199693_i64);
_12 = [0_usize,1_usize,0_usize,4260670778649420153_usize,13274009674333371754_usize,7180425293329767432_usize,13791349187209962482_usize];
_19 = [1016294038_i32,(-387364968_i32),(-283248611_i32),1621840159_i32,646018582_i32,(-1575712996_i32),931737013_i32];
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_7 = _10;
_8 = [10135333409531204355_usize,5942004277089373759_usize,7_usize,35129223316806337_usize,6865309359263720753_usize,3_usize,5_usize];
_10 = _7;
_20 = _10 as u16;
Goto(bb2)
}
bb2 = {
_16 = [_22,_22,_22,_22,_22];
_5 = !_6;
_5 = _7 ^ _7;
_22 = 1426546579762735687_i64 ^ (-2449001264807304928_i64);
_24 = 247234294733817007224617828688793977123_u128 << _7;
RET = _9;
_23 = [9510869339643926405_usize,0_usize,4_usize,3_usize,3_usize];
_17 = 34_i8;
_16 = [_22,_22,_22,_22,_22];
_23 = [9103603143996267882_usize,1184391265825881717_usize,5_usize,16866032798967684384_usize,5770126999015778671_usize];
_23 = [12927165720010938407_usize,5_usize,10730874324245380523_usize,7_usize,0_usize];
_24 = 175750665308570558494297648589924761515_u128;
_24 = !229608701962344619698165364393407887940_u128;
_4 = [13112695177559740766_usize,4_usize,13770091649102861065_usize,14231766257011219914_usize,1_usize,1_usize,13823834316611412472_usize];
_8 = _11;
_16 = [_22,_22,_22,_22,_22];
_21 = [_1,_1,_1,_1];
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_10 = _5;
RET = [2898917101412452270_usize,9230279701909341622_usize,9241790333087748729_usize,4267338721256014154_usize,1_usize,2399245256690632038_usize,10674626624432814868_usize];
_7 = _5 | _5;
_29 = &_13;
Goto(bb3)
}
bb3 = {
_16 = [_22,_22,_22,_22,_22];
_24 = _22 as u128;
_14 = !true;
_13 = (-101752822873881491412943002726178293592_i128) as f64;
_17 = (-78_i8);
_23 = [4_usize,2_usize,3080394837674619559_usize,6_usize,6642134680418599454_usize];
_20 = 73_u8 as u16;
RET = [7_usize,1_usize,1556435316127064308_usize,4639737166217800911_usize,10212323281030214483_usize,7662855627438631009_usize,10371257864066156561_usize];
Goto(bb4)
}
bb4 = {
_19 = [(-1900234875_i32),(-1074669279_i32),1820394941_i32,1857640768_i32,381292967_i32,(-1408692754_i32),(-1763594710_i32)];
_19 = [(-224741806_i32),(-1879061876_i32),38477088_i32,(-1940493503_i32),415971936_i32,1346681846_i32,815465200_i32];
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_24 = !34189065838400209182288787993117145071_u128;
_29 = &_13;
_24 = 165502002436331001048214605560819811264_u128 | 152372032716197779115116820839675680840_u128;
_30 = _1;
_2 = _8;
_28 = core::ptr::addr_of!(_21);
_20 = 52613_u16;
_6 = _20 as u64;
_10 = _7 | _7;
Call(_22 = fn8(Move(_29), (*_29)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6 = !_7;
_21 = [_30,_1,_1,_30];
_17 = _13 as i8;
_3 = [7077231807938696911_usize,13602072982319604452_usize,7_usize,13417442989560762241_usize,5_usize,17398644397267829721_usize,2_usize];
_21 = [_1,_30,_30,_1];
RET = [2731744724686817814_usize,17692071882601380499_usize,4_usize,16069204586428702657_usize,9733365598413985688_usize,4825096723194230530_usize,519264336443402067_usize];
_12 = [1419012909681887251_usize,4219024464655246323_usize,14532851234339848290_usize,0_usize,4_usize,0_usize,0_usize];
_13 = (-1346818056_i32) as f64;
RET = [6193672909049386030_usize,6_usize,3_usize,3840641577676832609_usize,4_usize,15374147956326645368_usize,54184491388179339_usize];
_2 = [11437968321381658392_usize,16988479493323078550_usize,488325873693705593_usize,1989935553671109576_usize,12642504493505020348_usize,5_usize,8072117457430534740_usize];
_3 = [6_usize,6806031053921375029_usize,15114838191267008590_usize,11063972955949144093_usize,10281372009683512662_usize,10505202567927521750_usize,3921915317696787668_usize];
_24 = 83199077687092665923792011774588235617_u128;
_6 = !_7;
_2 = [10843257410495018302_usize,7_usize,15884149184401207613_usize,11890570684387733866_usize,2498902325855370438_usize,7_usize,13134668866753014344_usize];
_17 = (-15_i8) | (-15_i8);
_22 = 68611802899494271271734258735670804830_i128 as i64;
_32.1 = _24 & _24;
_8 = [6_usize,1_usize,7_usize,506676706737935854_usize,14464724363517221386_usize,6625881538320377917_usize,1_usize];
_3 = _12;
_23 = [2_usize,9315648043486311477_usize,5_usize,3_usize,10498439371356528806_usize];
_8 = [0_usize,12766389369319595545_usize,14422761449760531741_usize,5_usize,3_usize,13401327170444030136_usize,4_usize];
Goto(bb6)
}
bb6 = {
_27 = _24 as f32;
_29 = &_13;
_19 = [(-825148622_i32),(-1718570893_i32),1323768844_i32,(-439951830_i32),(-897185377_i32),1737107998_i32,1101481244_i32];
_32.1 = _24 | _24;
_32.1 = !_24;
_20 = !9823_u16;
_32.1 = (-7255_i16) as u128;
_16 = [_22,_22,_22,_22,_22];
_36 = -(*_29);
_24 = !_32.1;
_16 = [_22,_22,_22,_22,_22];
_17 = !(-125_i8);
_30 = _1;
_28 = core::ptr::addr_of!((*_28));
_10 = _27 as u64;
_23 = [0_usize,2_usize,6375989162610453637_usize,1_usize,4_usize];
_38 = core::ptr::addr_of!(_21);
_6 = _5;
_23 = [10437495897138830517_usize,7295157670326019951_usize,1451870315320226767_usize,7_usize,4_usize];
_22 = (-2137761210909854877_i64) & 6220427406249559322_i64;
_8 = _9;
_37.0.0 = [57_u8,57_u8,136_u8,178_u8,45_u8,205_u8,24_u8,49_u8];
_11 = [1_usize,5_usize,2435865736716443119_usize,4_usize,5626138733733202136_usize,6_usize,8660661336797797142_usize];
_20 = !46837_u16;
_38 = core::ptr::addr_of!((*_28));
_30 = _1;
_30 = _1;
Goto(bb7)
}
bb7 = {
_37.0.3 = !_14;
_29 = &_36;
(*_28) = [_30,_30,_30,_1];
_15 = [_30,_30,_1,_30,_1,_30,_1,_1];
(*_28) = [_30,_1,_30,_1];
Goto(bb8)
}
bb8 = {
_32.0 = !30020_i16;
_37.0.2 = [268226428_i32];
_37.0.1 = 21233084689978095023135423551583213607_i128;
_35 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_21 = [_1,_1,_30,_30];
_8 = [4726940992463302195_usize,3_usize,16489668195708721835_usize,6_usize,2_usize,1900053878573761459_usize,0_usize];
Goto(bb9)
}
bb9 = {
_37.0.1 = _20 as i128;
_3 = _11;
_24 = !_32.1;
RET = [8081363405744108631_usize,6320013840512126567_usize,4_usize,4435742346058273644_usize,1626405956106478296_usize,7_usize,12968959231683976696_usize];
_24 = _32.1 & _32.1;
_37.0.0 = [139_u8,10_u8,7_u8,88_u8,167_u8,243_u8,91_u8,223_u8];
(*_28) = [_30,_1,_30,_1];
(*_38) = [_30,_30,_1,_1];
_37.1 = 6389070201059009080_usize as u8;
_26 = &_37.0.2;
_17 = 84_i8;
_21 = [_30,_30,_30,_1];
_32 = (30639_i16, _24);
_44 = !482079586_i32;
_20 = 26241_u16;
_37.0.2 = [_44];
_43 = [_44,_44,_44,_44,_44,_44,_44];
(*_38) = [_1,_30,_1,_1];
_24 = _32.1;
match _32.0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb10,
6 => bb11,
30639 => bb13,
_ => bb12
}
}
bb10 = {
_32.0 = !30020_i16;
_37.0.2 = [268226428_i32];
_37.0.1 = 21233084689978095023135423551583213607_i128;
_35 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_21 = [_1,_1,_30,_30];
_8 = [4726940992463302195_usize,3_usize,16489668195708721835_usize,6_usize,2_usize,1900053878573761459_usize,0_usize];
Goto(bb9)
}
bb11 = {
_19 = [(-1900234875_i32),(-1074669279_i32),1820394941_i32,1857640768_i32,381292967_i32,(-1408692754_i32),(-1763594710_i32)];
_19 = [(-224741806_i32),(-1879061876_i32),38477088_i32,(-1940493503_i32),415971936_i32,1346681846_i32,815465200_i32];
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_24 = !34189065838400209182288787993117145071_u128;
_29 = &_13;
_24 = 165502002436331001048214605560819811264_u128 | 152372032716197779115116820839675680840_u128;
_30 = _1;
_2 = _8;
_28 = core::ptr::addr_of!(_21);
_20 = 52613_u16;
_6 = _20 as u64;
_10 = _7 | _7;
Call(_22 = fn8(Move(_29), (*_29)), ReturnTo(bb5), UnwindUnreachable())
}
bb12 = {
_16 = [_22,_22,_22,_22,_22];
_5 = !_6;
_5 = _7 ^ _7;
_22 = 1426546579762735687_i64 ^ (-2449001264807304928_i64);
_24 = 247234294733817007224617828688793977123_u128 << _7;
RET = _9;
_23 = [9510869339643926405_usize,0_usize,4_usize,3_usize,3_usize];
_17 = 34_i8;
_16 = [_22,_22,_22,_22,_22];
_23 = [9103603143996267882_usize,1184391265825881717_usize,5_usize,16866032798967684384_usize,5770126999015778671_usize];
_23 = [12927165720010938407_usize,5_usize,10730874324245380523_usize,7_usize,0_usize];
_24 = 175750665308570558494297648589924761515_u128;
_24 = !229608701962344619698165364393407887940_u128;
_4 = [13112695177559740766_usize,4_usize,13770091649102861065_usize,14231766257011219914_usize,1_usize,1_usize,13823834316611412472_usize];
_8 = _11;
_16 = [_22,_22,_22,_22,_22];
_21 = [_1,_1,_1,_1];
_15 = [_1,_1,_1,_1,_1,_1,_1,_1];
_10 = _5;
RET = [2898917101412452270_usize,9230279701909341622_usize,9241790333087748729_usize,4267338721256014154_usize,1_usize,2399245256690632038_usize,10674626624432814868_usize];
_7 = _5 | _5;
_29 = &_13;
Goto(bb3)
}
bb13 = {
_43 = [_44,_44,_44,_44,_44,_44,_44];
_32.1 = _24;
_1 = _30;
_13 = _36;
RET = [3984839225456420091_usize,3_usize,3_usize,4_usize,0_usize,0_usize,3_usize];
_44 = _32.0 as i32;
_46 = _32.0 ^ _32.0;
(*_38) = [_1,_30,_1,_1];
(*_28) = [_30,_30,_30,_1];
_12 = RET;
_32.1 = _24;
_37.1 = _37.0.1 as u8;
_5 = (-9223372036854775808_isize) as u64;
_7 = _10;
_37.0.1 = (-10287555841911275604156504922880181172_i128);
_37.1 = 219_u8;
_23 = [10253855363133049426_usize,2_usize,1_usize,5_usize,7116645558806749733_usize];
_13 = (*_29) * _36;
_44 = (-792417017_i32) + 1269969201_i32;
_37.0.1 = _37.1 as i128;
Call(_25 = fn11(RET, _22, _19, _37.0.0, _30), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_29 = &_13;
(*_38) = [_1,_30,_1,_1];
_32.0 = !_46;
_5 = !_6;
_7 = _1 as u64;
_47 = _1;
_32.0 = _32.1 as i16;
_37.0.1 = !(-41943060106141594090340330157045245337_i128);
_32 = (_46, _24);
_50 = [13715033356661557997_usize,3_usize,3_usize,10195818629124723032_usize,6266990109111708942_usize];
_7 = _6;
_41 = core::ptr::addr_of_mut!(_37.0.0);
_39 = _44 & _44;
_13 = _36 * _36;
_42.0.0 = &_37.1;
_4 = _12;
_52 = [_37.0.1,_37.0.1,_37.0.1,_37.0.1,_37.0.1,_37.0.1];
_40 = _20 + _20;
_26 = &_37.0.2;
_17 = -(-111_i8);
_38 = core::ptr::addr_of!((*_28));
_46 = _32.0 & _32.0;
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(7_usize, 35_usize, Move(_35), 19_usize, Move(_19), 47_usize, Move(_47), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(7_usize, 7_usize, Move(_7), 23_usize, Move(_23), 46_usize, Move(_46), 52_usize, Move(_52)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(7_usize, 17_usize, Move(_17), 22_usize, Move(_22), 43_usize, Move(_43), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(7_usize, 1_usize, Move(_1), 40_usize, Move(_40), 11_usize, Move(_11), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(7_usize, 21_usize, Move(_21), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: &'static f64,mut _2: f64) -> i64 {
mir! {
type RET = i64;
let _3: [i8; 3];
let _4: *const *const Adt17;
let _5: &'static [i32; 1];
let _6: &'static i32;
let _7: (&'static u8,);
let _8: ([i32; 1], [u8; 8]);
let _9: (([u8; 8], i128, [i32; 1], bool), u8);
let _10: i32;
let _11: Adt52;
let _12: usize;
let _13: *const [char; 4];
let _14: char;
let _15: u32;
let _16: Adt52;
let _17: [char; 8];
let _18: isize;
let _19: &'static i16;
let _20: *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _21: Adt17;
let _22: *const [char; 4];
let _23: bool;
let _24: f32;
let _25: ();
let _26: ();
{
_1 = &_2;
_1 = &_2;
_2 = 9071465144180965834761442595438599269_u128 as f64;
_3 = [(-100_i8),71_i8,77_i8];
RET = !8324590789408483426_i64;
_3 = [(-87_i8),111_i8,(-61_i8)];
_1 = &_2;
RET = !6702889656260227472_i64;
RET = (-878296833520413036_i64) >> (-60232140579366198519222054983050955610_i128);
RET = 9223372036854775807_isize as i64;
_3 = [16_i8,(-128_i8),(-24_i8)];
RET = 217265929540051006_i64 - (-7865371211606475553_i64);
_2 = RET as f64;
_1 = &_2;
_2 = (-1931769427_i32) as f64;
RET = (-8372531868794668531_i64) + (-2599769436029917573_i64);
RET = !(-8641013873341592517_i64);
RET = 2060244504988794347_i64 | 1000146761206561483_i64;
RET = (-9048156307026260251_i64);
RET = -8948121184643122872_i64;
RET = 6014383690426527224_i64;
RET = !6259511557141118202_i64;
RET = -(-8514266792229077796_i64);
Goto(bb1)
}
bb1 = {
RET = (-812084370_i32) as i64;
_2 = 254171225099015310202133982013287195381_u128 as f64;
_2 = 47105_u16 as f64;
_2 = RET as f64;
_3 = [(-67_i8),(-56_i8),78_i8];
RET = !1865240034822976098_i64;
_2 = 226_u8 as f64;
RET = !6017836597148523460_i64;
_1 = &_2;
_2 = (-9223372036854775808_isize) as f64;
RET = 13361746754042480366_u64 as i64;
_3 = [(-16_i8),23_i8,8_i8];
RET = 5674594879659681809_i64 - 707026301468877467_i64;
_3 = [55_i8,75_i8,(-122_i8)];
_8.0 = [(-1798602638_i32)];
_7.0 = &_9.1;
_9.0.0 = [60_u8,209_u8,151_u8,231_u8,243_u8,78_u8,211_u8,15_u8];
_8.1 = [193_u8,96_u8,30_u8,205_u8,99_u8,134_u8,244_u8,202_u8];
_9.0 = (_8.1, (-123737480171407416346402462664700176990_i128), _8.0, false);
_1 = &_2;
_5 = &_8.0;
_1 = &_2;
_9.0 = (_8.1, (-101416162287827560218400415325728088759_i128), (*_5), false);
_9.1 = 68_u8 ^ 166_u8;
_7.0 = &_9.1;
_8 = (_9.0.2, _9.0.0);
_8.0 = [736186189_i32];
_8.0 = [1222074852_i32];
Call(RET = fn9(Move(_7), _8.1, _9.0, _9.0.3, _9.0.3, _9.0.1, _9.0, _9.0.1, _8.1, _8, _9.0.3, _9.0.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = [(-34_i8),84_i8,(-35_i8)];
_9.0.1 = 95320190963495498862792277560187164293_i128;
_5 = &_9.0.2;
RET = (-4366496058809363436_i64);
_8 = (_9.0.2, _9.0.0);
match _9.0.1 {
95320190963495498862792277560187164293 => bb4,
_ => bb3
}
}
bb3 = {
RET = (-812084370_i32) as i64;
_2 = 254171225099015310202133982013287195381_u128 as f64;
_2 = 47105_u16 as f64;
_2 = RET as f64;
_3 = [(-67_i8),(-56_i8),78_i8];
RET = !1865240034822976098_i64;
_2 = 226_u8 as f64;
RET = !6017836597148523460_i64;
_1 = &_2;
_2 = (-9223372036854775808_isize) as f64;
RET = 13361746754042480366_u64 as i64;
_3 = [(-16_i8),23_i8,8_i8];
RET = 5674594879659681809_i64 - 707026301468877467_i64;
_3 = [55_i8,75_i8,(-122_i8)];
_8.0 = [(-1798602638_i32)];
_7.0 = &_9.1;
_9.0.0 = [60_u8,209_u8,151_u8,231_u8,243_u8,78_u8,211_u8,15_u8];
_8.1 = [193_u8,96_u8,30_u8,205_u8,99_u8,134_u8,244_u8,202_u8];
_9.0 = (_8.1, (-123737480171407416346402462664700176990_i128), _8.0, false);
_1 = &_2;
_5 = &_8.0;
_1 = &_2;
_9.0 = (_8.1, (-101416162287827560218400415325728088759_i128), (*_5), false);
_9.1 = 68_u8 ^ 166_u8;
_7.0 = &_9.1;
_8 = (_9.0.2, _9.0.0);
_8.0 = [736186189_i32];
_8.0 = [1222074852_i32];
Call(RET = fn9(Move(_7), _8.1, _9.0, _9.0.3, _9.0.3, _9.0.1, _9.0, _9.0.1, _8.1, _8, _9.0.3, _9.0.1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_9.0.1 = !(-167233288083371930727753378837282810179_i128);
_9.0.0 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_11.fld0.1 = '\u{92a3a}';
_9.0.1 = 193824048980333695017363207876597922546_u128 as i128;
_9.0 = (_8.1, (-86926154599067027294779798351106066331_i128), _8.0, true);
_6 = &_10;
_11.fld1.0 = ((-1522488569_i32),);
_1 = &_2;
RET = (-4507565131915263479_i64);
_11.fld1.0 = ((-511870239_i32),);
_11.fld1.0.0 = -(-1927610743_i32);
_9.0.3 = !false;
_8.0 = _9.0.2;
_9.0.2 = _8.0;
_11.fld0 = (_9.0.2, '\u{980d1}');
_5 = &_8.0;
_9.0.3 = _9.0.1 > _9.0.1;
_1 = &(*_1);
_9.0.0 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_11.fld0.1 = '\u{54709}';
RET = (-6657470905908737633_i64);
_9.0 = (_8.1, 143212456471159200607070731406762752431_i128, (*_5), false);
match _9.0.1 {
143212456471159200607070731406762752431 => bb6,
_ => bb5
}
}
bb5 = {
_3 = [(-34_i8),84_i8,(-35_i8)];
_9.0.1 = 95320190963495498862792277560187164293_i128;
_5 = &_9.0.2;
RET = (-4366496058809363436_i64);
_8 = (_9.0.2, _9.0.0);
match _9.0.1 {
95320190963495498862792277560187164293 => bb4,
_ => bb3
}
}
bb6 = {
_9.0.1 = (-30356928958912926426317181839451420426_i128) << RET;
_3 = [(-89_i8),(-66_i8),42_i8];
_9.0.2 = [_11.fld1.0.0];
_8.1 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_12 = !0_usize;
_6 = &_11.fld1.0.0;
_1 = &_2;
_9.0 = (_8.1, (-55664875780960966091446997616603272080_i128), _8.0, false);
_11.fld1.0.0 = (-1988151702_i32) << _9.0.1;
_7.0 = &_9.1;
_3 = [32_i8,(-58_i8),(-73_i8)];
_11.fld0 = ((*_5), '\u{e2f45}');
_1 = &(*_1);
_5 = &(*_5);
_16.fld1.0.0 = -_11.fld1.0.0;
_5 = &(*_5);
_11.fld0.0 = [_16.fld1.0.0];
_6 = &_10;
Goto(bb7)
}
bb7 = {
_14 = _11.fld0.1;
_11.fld0.1 = _14;
_16 = Adt52 { fld0: _11.fld0,fld1: _11.fld1 };
_14 = _16.fld0.1;
Call(_16.fld0 = fn10(_14, _9.0.1, _16.fld1.0.0, _11.fld1.0, _11.fld0.1, Move(_11), _16.fld1.0.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11.fld0.0 = _16.fld0.0;
_15 = _12 as u32;
_16.fld0.0 = _11.fld0.0;
_11 = Move(_16);
_8 = (_11.fld0.0, _9.0.0);
_16 = Adt52 { fld0: _11.fld0,fld1: _11.fld1 };
_16.fld1.0.0 = _15 as i32;
_11.fld1.0.0 = _16.fld0.1 as i32;
_11.fld0.1 = _14;
_8.1 = _9.0.0;
_9.0.0 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_1 = &_2;
RET = 4000438209445444379_i64 ^ 7154736235955558650_i64;
RET = 39_i8 as i64;
_2 = 20351_i16 as f64;
_8 = (_16.fld0.0, _9.0.0);
_9.0 = (_8.1, (-151193186192497339326076053625550448052_i128), _8.0, true);
_8.0 = [_16.fld1.0.0];
_1 = &_2;
RET = (-5234649748500011134_i64);
_12 = !11140450789581274744_usize;
_9.0.2 = [_16.fld1.0.0];
_9.0.2 = [_16.fld1.0.0];
match _9.0.1 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
189089180728441124137298553806217763404 => bb15,
_ => bb14
}
}
bb9 = {
_14 = _11.fld0.1;
_11.fld0.1 = _14;
_16 = Adt52 { fld0: _11.fld0,fld1: _11.fld1 };
_14 = _16.fld0.1;
Call(_16.fld0 = fn10(_14, _9.0.1, _16.fld1.0.0, _11.fld1.0, _11.fld0.1, Move(_11), _16.fld1.0.0), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
RET = (-812084370_i32) as i64;
_2 = 254171225099015310202133982013287195381_u128 as f64;
_2 = 47105_u16 as f64;
_2 = RET as f64;
_3 = [(-67_i8),(-56_i8),78_i8];
RET = !1865240034822976098_i64;
_2 = 226_u8 as f64;
RET = !6017836597148523460_i64;
_1 = &_2;
_2 = (-9223372036854775808_isize) as f64;
RET = 13361746754042480366_u64 as i64;
_3 = [(-16_i8),23_i8,8_i8];
RET = 5674594879659681809_i64 - 707026301468877467_i64;
_3 = [55_i8,75_i8,(-122_i8)];
_8.0 = [(-1798602638_i32)];
_7.0 = &_9.1;
_9.0.0 = [60_u8,209_u8,151_u8,231_u8,243_u8,78_u8,211_u8,15_u8];
_8.1 = [193_u8,96_u8,30_u8,205_u8,99_u8,134_u8,244_u8,202_u8];
_9.0 = (_8.1, (-123737480171407416346402462664700176990_i128), _8.0, false);
_1 = &_2;
_5 = &_8.0;
_1 = &_2;
_9.0 = (_8.1, (-101416162287827560218400415325728088759_i128), (*_5), false);
_9.1 = 68_u8 ^ 166_u8;
_7.0 = &_9.1;
_8 = (_9.0.2, _9.0.0);
_8.0 = [736186189_i32];
_8.0 = [1222074852_i32];
Call(RET = fn9(Move(_7), _8.1, _9.0, _9.0.3, _9.0.3, _9.0.1, _9.0, _9.0.1, _8.1, _8, _9.0.3, _9.0.1), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_3 = [(-34_i8),84_i8,(-35_i8)];
_9.0.1 = 95320190963495498862792277560187164293_i128;
_5 = &_9.0.2;
RET = (-4366496058809363436_i64);
_8 = (_9.0.2, _9.0.0);
match _9.0.1 {
95320190963495498862792277560187164293 => bb4,
_ => bb3
}
}
bb12 = {
_9.0.1 = !(-167233288083371930727753378837282810179_i128);
_9.0.0 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_11.fld0.1 = '\u{92a3a}';
_9.0.1 = 193824048980333695017363207876597922546_u128 as i128;
_9.0 = (_8.1, (-86926154599067027294779798351106066331_i128), _8.0, true);
_6 = &_10;
_11.fld1.0 = ((-1522488569_i32),);
_1 = &_2;
RET = (-4507565131915263479_i64);
_11.fld1.0 = ((-511870239_i32),);
_11.fld1.0.0 = -(-1927610743_i32);
_9.0.3 = !false;
_8.0 = _9.0.2;
_9.0.2 = _8.0;
_11.fld0 = (_9.0.2, '\u{980d1}');
_5 = &_8.0;
_9.0.3 = _9.0.1 > _9.0.1;
_1 = &(*_1);
_9.0.0 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.1];
_11.fld0.1 = '\u{54709}';
RET = (-6657470905908737633_i64);
_9.0 = (_8.1, 143212456471159200607070731406762752431_i128, (*_5), false);
match _9.0.1 {
143212456471159200607070731406762752431 => bb6,
_ => bb5
}
}
bb13 = {
RET = (-812084370_i32) as i64;
_2 = 254171225099015310202133982013287195381_u128 as f64;
_2 = 47105_u16 as f64;
_2 = RET as f64;
_3 = [(-67_i8),(-56_i8),78_i8];
RET = !1865240034822976098_i64;
_2 = 226_u8 as f64;
RET = !6017836597148523460_i64;
_1 = &_2;
_2 = (-9223372036854775808_isize) as f64;
RET = 13361746754042480366_u64 as i64;
_3 = [(-16_i8),23_i8,8_i8];
RET = 5674594879659681809_i64 - 707026301468877467_i64;
_3 = [55_i8,75_i8,(-122_i8)];
_8.0 = [(-1798602638_i32)];
_7.0 = &_9.1;
_9.0.0 = [60_u8,209_u8,151_u8,231_u8,243_u8,78_u8,211_u8,15_u8];
_8.1 = [193_u8,96_u8,30_u8,205_u8,99_u8,134_u8,244_u8,202_u8];
_9.0 = (_8.1, (-123737480171407416346402462664700176990_i128), _8.0, false);
_1 = &_2;
_5 = &_8.0;
_1 = &_2;
_9.0 = (_8.1, (-101416162287827560218400415325728088759_i128), (*_5), false);
_9.1 = 68_u8 ^ 166_u8;
_7.0 = &_9.1;
_8 = (_9.0.2, _9.0.0);
_8.0 = [736186189_i32];
_8.0 = [1222074852_i32];
Call(RET = fn9(Move(_7), _8.1, _9.0, _9.0.3, _9.0.3, _9.0.1, _9.0, _9.0.1, _8.1, _8, _9.0.3, _9.0.1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_3 = [(-34_i8),84_i8,(-35_i8)];
_9.0.1 = 95320190963495498862792277560187164293_i128;
_5 = &_9.0.2;
RET = (-4366496058809363436_i64);
_8 = (_9.0.2, _9.0.0);
match _9.0.1 {
95320190963495498862792277560187164293 => bb4,
_ => bb3
}
}
bb15 = {
_11.fld0.1 = _14;
_11.fld1.0.0 = _16.fld1.0.0;
_11.fld1 = (_16.fld1.0,);
_9.1 = 115_u8 << _9.0.1;
_17 = [_16.fld0.1,_14,_14,_11.fld0.1,_16.fld0.1,_11.fld0.1,_11.fld0.1,_11.fld0.1];
_16.fld0.1 = _11.fld0.1;
_2 = (-114_i8) as f64;
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(8_usize, 12_usize, Move(_12), 17_usize, Move(_17), 9_usize, Move(_9), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: (&'static u8,),mut _2: [u8; 8],mut _3: ([u8; 8], i128, [i32; 1], bool),mut _4: bool,mut _5: bool,mut _6: i128,mut _7: ([u8; 8], i128, [i32; 1], bool),mut _8: i128,mut _9: [u8; 8],mut _10: ([i32; 1], [u8; 8]),mut _11: bool,mut _12: i128) -> i64 {
mir! {
type RET = i64;
let _13: [u8; 8];
let _14: u8;
let _15: *const bool;
let _16: u16;
let _17: Adt39;
let _18: ((i32,),);
let _19: [isize; 6];
let _20: Adt33;
let _21: [usize; 5];
let _22: i8;
let _23: bool;
let _24: ();
let _25: ();
{
_7.0 = [32_u8,251_u8,36_u8,4_u8,185_u8,103_u8,82_u8,6_u8];
_6 = 14159_i16 as i128;
Goto(bb1)
}
bb1 = {
RET = 493333138306028597_i64;
_10.1 = _3.0;
_7.3 = _3.3;
_3.1 = 13662_u16 as i128;
_5 = _7.1 > _12;
_7.1 = -_8;
_7.0 = [128_u8,234_u8,147_u8,247_u8,179_u8,77_u8,198_u8,225_u8];
_4 = _5 | _11;
_13 = _10.1;
_5 = _7.3;
_7.3 = _4;
_11 = !_4;
_15 = core::ptr::addr_of!(_5);
_16 = 50756_u16 - 25412_u16;
Call(_14 = core::intrinsics::bswap(15_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.0 = [92_u8,138_u8,97_u8,164_u8,96_u8,72_u8,241_u8,157_u8];
_3.0 = [28_u8,14_u8,140_u8,235_u8,195_u8,150_u8,14_u8,147_u8];
Goto(bb3)
}
bb3 = {
_2 = [126_u8,229_u8,181_u8,128_u8,68_u8,92_u8,175_u8,62_u8];
_3.1 = _7.1;
_3.2 = [(-239575902_i32)];
_6 = _12;
_8 = (-9223372036854775808_isize) as i128;
_13 = _7.0;
_2 = [248_u8,198_u8,32_u8,24_u8,22_u8,218_u8,44_u8,62_u8];
_3.0 = [99_u8,192_u8,220_u8,160_u8,164_u8,28_u8,202_u8,58_u8];
_6 = _12;
_3.3 = _11;
_7.3 = _4;
_7 = (_13, _3.1, _3.2, _3.3);
_8 = _12;
_4 = _5;
RET = !(-608300710277756309_i64);
_7.2 = [500041475_i32];
_7.0 = _13;
_7.2 = [1165384121_i32];
_15 = core::ptr::addr_of!(_11);
_8 = -_3.1;
match _12 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
238866204633110903244974192106040122697 => bb8,
_ => bb7
}
}
bb4 = {
_3.0 = [92_u8,138_u8,97_u8,164_u8,96_u8,72_u8,241_u8,157_u8];
_3.0 = [28_u8,14_u8,140_u8,235_u8,195_u8,150_u8,14_u8,147_u8];
Goto(bb3)
}
bb5 = {
RET = 493333138306028597_i64;
_10.1 = _3.0;
_7.3 = _3.3;
_3.1 = 13662_u16 as i128;
_5 = _7.1 > _12;
_7.1 = -_8;
_7.0 = [128_u8,234_u8,147_u8,247_u8,179_u8,77_u8,198_u8,225_u8];
_4 = _5 | _11;
_13 = _10.1;
_5 = _7.3;
_7.3 = _4;
_11 = !_4;
_15 = core::ptr::addr_of!(_5);
_16 = 50756_u16 - 25412_u16;
Call(_14 = core::intrinsics::bswap(15_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_3.2 = _7.2;
_12 = !_3.1;
RET = 11527_i16 as i64;
_18.0.0 = (-1076232701_i32);
_8 = _6;
RET = -(-460721012738246641_i64);
_19 = [9223372036854775807_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-29_isize),(-99_isize)];
_11 = !_3.3;
_3.1 = 279015489487502720574055759434634582005_u128 as i128;
_7.3 = !_4;
_2 = [129_u8,26_u8,182_u8,90_u8,127_u8,24_u8,233_u8,156_u8];
_15 = core::ptr::addr_of!((*_15));
_5 = !(*_15);
_3.2 = [_18.0.0];
_15 = core::ptr::addr_of!(_7.3);
_11 = _5 >= _3.3;
_19 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-10_isize)];
_16 = 25907_u16 << _7.1;
_1.0 = &_14;
_1.0 = &_14;
_18.0.0 = 1632576555_i32 & 1767106374_i32;
_21 = [4_usize,3920839267469852334_usize,3464440583927540371_usize,9984087255569883053_usize,3_usize];
RET = 4163217800367953442_i64;
match _8 {
0 => bb9,
1 => bb10,
2 => bb11,
238866204633110903244974192106040122697 => bb13,
_ => bb12
}
}
bb9 = {
Return()
}
bb10 = {
RET = 493333138306028597_i64;
_10.1 = _3.0;
_7.3 = _3.3;
_3.1 = 13662_u16 as i128;
_5 = _7.1 > _12;
_7.1 = -_8;
_7.0 = [128_u8,234_u8,147_u8,247_u8,179_u8,77_u8,198_u8,225_u8];
_4 = _5 | _11;
_13 = _10.1;
_5 = _7.3;
_7.3 = _4;
_11 = !_4;
_15 = core::ptr::addr_of!(_5);
_16 = 50756_u16 - 25412_u16;
Call(_14 = core::intrinsics::bswap(15_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
RET = 493333138306028597_i64;
_10.1 = _3.0;
_7.3 = _3.3;
_3.1 = 13662_u16 as i128;
_5 = _7.1 > _12;
_7.1 = -_8;
_7.0 = [128_u8,234_u8,147_u8,247_u8,179_u8,77_u8,198_u8,225_u8];
_4 = _5 | _11;
_13 = _10.1;
_5 = _7.3;
_7.3 = _4;
_11 = !_4;
_15 = core::ptr::addr_of!(_5);
_16 = 50756_u16 - 25412_u16;
Call(_14 = core::intrinsics::bswap(15_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_3.0 = [92_u8,138_u8,97_u8,164_u8,96_u8,72_u8,241_u8,157_u8];
_3.0 = [28_u8,14_u8,140_u8,235_u8,195_u8,150_u8,14_u8,147_u8];
Goto(bb3)
}
bb13 = {
_6 = _7.1;
Goto(bb14)
}
bb14 = {
_5 = (*_15);
_10.1 = _9;
_7.0 = _13;
_3.0 = _13;
_8 = !_12;
_21 = [13416205287733993879_usize,8957009170313849036_usize,3_usize,5529021872279652266_usize,3078126542150488275_usize];
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(9_usize, 7_usize, Move(_7), 4_usize, Move(_4), 8_usize, Move(_8), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(9_usize, 13_usize, Move(_13), 16_usize, Move(_16), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: char,mut _2: i128,mut _3: i32,mut _4: (i32,),mut _5: char,mut _6: Adt52,mut _7: i32) -> ([i32; 1], char) {
mir! {
type RET = ([i32; 1], char);
let _8: Adt82;
let _9: &'static *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _10: ();
let _11: ();
{
RET.0 = _6.fld0.0;
match _2 {
0 => bb1,
1 => bb2,
284617491139977497371927609815164939376 => bb4,
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
RET.0 = [_4.0];
_6.fld0 = (RET.0, _5);
RET = (_6.fld0.0, _1);
_7 = -_6.fld1.0.0;
_6.fld0.1 = _1;
_1 = _6.fld0.1;
RET.0 = _6.fld0.0;
RET = (_6.fld0.0, _6.fld0.1);
_7 = _3 ^ _3;
_6.fld0 = (RET.0, RET.1);
_2 = !51813220720121816450585928818939635264_i128;
_4.0 = false as i32;
_6.fld0.0 = [_7];
Goto(bb5)
}
bb5 = {
_4 = (_7,);
_1 = RET.1;
RET = _6.fld0;
RET.0 = _6.fld0.0;
_7 = _4.0;
_2 = (-29_i8) as i128;
RET = (_6.fld0.0, _1);
_6.fld1 = (_4,);
_2 = _6.fld1.0.0 as i128;
_1 = RET.1;
_4 = _6.fld1.0;
RET.1 = _5;
RET = _6.fld0;
_6.fld1.0.0 = (-5897062721227462524_i64) as i32;
_6.fld1.0 = (_4.0,);
_7 = !_4.0;
_6.fld0.1 = _1;
_6.fld0.1 = _5;
_6.fld0.1 = _1;
_6.fld0 = (RET.0, RET.1);
RET.1 = _1;
_4 = (_6.fld1.0.0,);
_6.fld1 = (_4,);
_6.fld1.0.0 = _4.0;
RET.0 = _6.fld0.0;
Goto(bb6)
}
bb6 = {
Call(_10 = dump_var(10_usize, 7_usize, Move(_7), 3_usize, Move(_3), 4_usize, Move(_4), 11_usize, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [usize; 7],mut _2: i64,mut _3: [i32; 7],mut _4: [u8; 8],mut _5: char) -> *const [char; 7] {
mir! {
type RET = *const [char; 7];
let _6: Adt82;
let _7: isize;
let _8: *const [char; 7];
let _9: ([u8; 8], i128, [i32; 1], bool);
let _10: isize;
let _11: f64;
let _12: i64;
let _13: char;
let _14: [char; 7];
let _15: isize;
let _16: ([i8; 3], u32, ([u8; 8], i128, [i32; 1], bool), *const [char; 7]);
let _17: i32;
let _18: &'static &'static u16;
let _19: f32;
let _20: Adt52;
let _21: &'static i32;
let _22: char;
let _23: (i32,);
let _24: &'static [i32; 1];
let _25: *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _26: f32;
let _27: isize;
let _28: bool;
let _29: &'static [char; 7];
let _30: *mut [i32; 7];
let _31: *mut [i32; 7];
let _32: i16;
let _33: isize;
let _34: isize;
let _35: bool;
let _36: isize;
let _37: f32;
let _38: ();
let _39: ();
{
_2 = (-2023698319387804148_i64) ^ (-7659179350571420175_i64);
_1 = [0_usize,3_usize,2168565332224603576_usize,1508439919099609268_usize,12649325964696786438_usize,4_usize,11932181680074454093_usize];
_5 = '\u{1e8cb}';
_3 = [1157031718_i32,1667207271_i32,(-2107098417_i32),(-2119413286_i32),(-1535762353_i32),(-2052290822_i32),801300530_i32];
_4 = [50_u8,225_u8,72_u8,121_u8,248_u8,62_u8,25_u8,134_u8];
_2 = -(-5574687616040527818_i64);
_2 = -(-8617465239783760214_i64);
_5 = '\u{cc54d}';
Goto(bb1)
}
bb1 = {
_5 = '\u{f38b8}';
_3 = [(-1163074772_i32),(-1671243138_i32),466812331_i32,896939707_i32,(-10562393_i32),2052521256_i32,(-1725738723_i32)];
_3 = [842137970_i32,2055188941_i32,7977947_i32,39579905_i32,153381797_i32,(-1156442614_i32),(-2117765622_i32)];
_7 = !(-9223372036854775808_isize);
_1 = [6_usize,3299191794397627186_usize,15182128919224607808_usize,681198470789509060_usize,3_usize,4_usize,7_usize];
_9.3 = !false;
_9.3 = _7 <= _7;
_9.0 = [203_u8,66_u8,208_u8,79_u8,227_u8,147_u8,63_u8,89_u8];
_9.2 = [1455785611_i32];
_2 = -(-7374081955360292168_i64);
_9.0 = _4;
_9.2 = [139187611_i32];
_3 = [(-2038157984_i32),1243899802_i32,(-433321836_i32),1769261060_i32,23478203_i32,(-952157606_i32),1153001199_i32];
_2 = 2_usize as i64;
_9.2 = [1777271443_i32];
_9.1 = 1082200302_i32 as i128;
_4 = [112_u8,181_u8,40_u8,33_u8,63_u8,237_u8,163_u8,16_u8];
Call(_9 = fn12(_3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.0 = [219_u8,139_u8,219_u8,116_u8,179_u8,56_u8,238_u8,224_u8];
_4 = [201_u8,110_u8,213_u8,183_u8,88_u8,193_u8,115_u8,208_u8];
_4 = [35_u8,93_u8,69_u8,174_u8,91_u8,70_u8,114_u8,89_u8];
_3 = [597680893_i32,(-806571746_i32),(-1085008392_i32),1274267449_i32,838248489_i32,(-1266894180_i32),(-1685306184_i32)];
_4 = [0_u8,43_u8,0_u8,182_u8,169_u8,201_u8,210_u8,214_u8];
_3 = [(-1941792204_i32),1892240095_i32,(-1370591210_i32),(-653780020_i32),1769062126_i32,773738343_i32,494911073_i32];
_4 = [237_u8,176_u8,218_u8,94_u8,16_u8,16_u8,157_u8,160_u8];
_10 = _7 << _7;
Goto(bb3)
}
bb3 = {
_9.0 = [176_u8,51_u8,46_u8,168_u8,231_u8,240_u8,68_u8,80_u8];
_5 = '\u{715b8}';
_7 = _10;
Goto(bb4)
}
bb4 = {
_3 = [1700147545_i32,(-641488620_i32),(-1212489722_i32),(-204588436_i32),(-754885172_i32),(-1893523888_i32),144805689_i32];
_9.2 = [(-1872590436_i32)];
_9.0 = [230_u8,245_u8,25_u8,30_u8,146_u8,184_u8,107_u8,236_u8];
_10 = _7 >> _9.1;
_13 = _5;
_9.2 = [(-2122592517_i32)];
_8 = core::ptr::addr_of!(_14);
_9.1 = 1934183703_i32 as i128;
_9.0 = _4;
(*_8) = [_5,_5,_5,_13,_13,_13,_5];
_5 = _13;
(*_8) = [_5,_5,_5,_5,_13,_13,_5];
_9.1 = -(-15954536238916796148670668930816424360_i128);
_11 = 61_i8 as f64;
_12 = !_2;
_8 = core::ptr::addr_of!((*_8));
_7 = _10;
RET = core::ptr::addr_of!((*_8));
(*_8) = [_13,_5,_5,_5,_5,_5,_13];
_9.2 = [(-1317598892_i32)];
_2 = _12;
_2 = (-8743_i16) as i64;
(*RET) = [_13,_5,_5,_5,_5,_13,_13];
_10 = _2 as isize;
Call(_15 = fn13(_9.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = _10;
_13 = _5;
_16.2 = (_9.0, _9.1, _9.2, _9.3);
RET = core::ptr::addr_of!((*RET));
_9 = _16.2;
_9.1 = 6224_i16 as i128;
RET = Move(_8);
_15 = _7;
_8 = core::ptr::addr_of!(_14);
_16.3 = core::ptr::addr_of!((*_8));
_3 = [408333310_i32,440483808_i32,(-319262253_i32),333404350_i32,528356610_i32,303848829_i32,(-41741308_i32)];
_9 = (_4, _16.2.1, _16.2.2, _16.2.3);
Goto(bb6)
}
bb6 = {
_9 = (_16.2.0, _16.2.1, _16.2.2, _16.2.3);
_3 = [1082223141_i32,384028305_i32,(-1024429193_i32),(-946279904_i32),(-1020903138_i32),(-733208363_i32),1343068656_i32];
_16.2 = (_9.0, _9.1, _9.2, _9.3);
_9.3 = !_16.2.3;
_16.2.2 = [1535633945_i32];
_20.fld0 = (_9.2, _5);
_16.0 = [(-85_i8),0_i8,(-30_i8)];
(*_8) = [_13,_20.fld0.1,_5,_5,_5,_5,_5];
_19 = 3131798163_u32 as f32;
_8 = Move(_16.3);
_20.fld1.0.0 = (-57275592_i32) + 294726997_i32;
_20.fld1.0 = ((-1512434843_i32),);
_12 = -_2;
_16.2.1 = -_9.1;
_16.2.1 = 98_u8 as i128;
Call(_20.fld0.0 = core::intrinsics::transmute(_9.2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16.3 = Move(RET);
_9 = (_16.2.0, _16.2.1, _20.fld0.0, _16.2.3);
_16.2.3 = !_9.3;
_20.fld1.0 = (1798706606_i32,);
_20.fld1.0 = ((-448785373_i32),);
_21 = &_20.fld1.0.0;
_9.1 = _9.3 as i128;
_8 = core::ptr::addr_of!(_14);
_9 = (_16.2.0, _16.2.1, _16.2.2, _16.2.3);
_10 = _15 ^ _7;
_26 = 4955482417238317946655551848707073056_u128 as f32;
_9.3 = _16.2.3;
_16.2.1 = -_9.1;
_21 = &_23.0;
_24 = &_16.2.2;
_7 = _10 ^ _10;
_16.0 = [16_i8,95_i8,34_i8];
match _20.fld1.0.0 {
340282366920938463463374607431319426083 => bb8,
_ => bb5
}
}
bb8 = {
_10 = _9.1 as isize;
_16.2 = (_9.0, _9.1, _20.fld0.0, _9.3);
_16.1 = 100941855_u32 * 3531727579_u32;
_5 = _13;
_9.1 = -_16.2.1;
_11 = _20.fld1.0.0 as f64;
match _20.fld1.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431319426083 => bb10,
_ => bb9
}
}
bb9 = {
_5 = '\u{f38b8}';
_3 = [(-1163074772_i32),(-1671243138_i32),466812331_i32,896939707_i32,(-10562393_i32),2052521256_i32,(-1725738723_i32)];
_3 = [842137970_i32,2055188941_i32,7977947_i32,39579905_i32,153381797_i32,(-1156442614_i32),(-2117765622_i32)];
_7 = !(-9223372036854775808_isize);
_1 = [6_usize,3299191794397627186_usize,15182128919224607808_usize,681198470789509060_usize,3_usize,4_usize,7_usize];
_9.3 = !false;
_9.3 = _7 <= _7;
_9.0 = [203_u8,66_u8,208_u8,79_u8,227_u8,147_u8,63_u8,89_u8];
_9.2 = [1455785611_i32];
_2 = -(-7374081955360292168_i64);
_9.0 = _4;
_9.2 = [139187611_i32];
_3 = [(-2038157984_i32),1243899802_i32,(-433321836_i32),1769261060_i32,23478203_i32,(-952157606_i32),1153001199_i32];
_2 = 2_usize as i64;
_9.2 = [1777271443_i32];
_9.1 = 1082200302_i32 as i128;
_4 = [112_u8,181_u8,40_u8,33_u8,63_u8,237_u8,163_u8,16_u8];
Call(_9 = fn12(_3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_16.2.2 = [_20.fld1.0.0];
_17 = _20.fld1.0.0 & _20.fld1.0.0;
_5 = _20.fld0.1;
_13 = _20.fld0.1;
_21 = &(*_21);
_3 = [_17,_17,_17,_20.fld1.0.0,_17,_17,_20.fld1.0.0];
_4 = [230_u8,137_u8,9_u8,47_u8,130_u8,46_u8,103_u8,194_u8];
(*_8) = [_5,_13,_13,_5,_13,_20.fld0.1,_13];
_15 = _7 >> _17;
_15 = 235139390825674541751487061820963071576_u128 as isize;
_26 = -_19;
_26 = _20.fld1.0.0 as f32;
RET = Move(_16.3);
_32 = -(-2594_i16);
_16.3 = Move(RET);
_26 = 11265838988181830956_u64 as f32;
_23 = _20.fld1.0;
_16.2 = _9;
_9 = (_16.2.0, _16.2.1, _20.fld0.0, _16.2.3);
_28 = _9.3;
_16.1 = !623371339_u32;
(*_8) = [_5,_5,_20.fld0.1,_13,_5,_13,_13];
_16.2.0 = [90_u8,105_u8,240_u8,26_u8,90_u8,72_u8,139_u8,248_u8];
_26 = _19;
RET = core::ptr::addr_of!((*_8));
match _20.fld1.0.0 {
0 => bb4,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463463374607431319426083 => bb17,
_ => bb16
}
}
bb11 = {
_5 = '\u{f38b8}';
_3 = [(-1163074772_i32),(-1671243138_i32),466812331_i32,896939707_i32,(-10562393_i32),2052521256_i32,(-1725738723_i32)];
_3 = [842137970_i32,2055188941_i32,7977947_i32,39579905_i32,153381797_i32,(-1156442614_i32),(-2117765622_i32)];
_7 = !(-9223372036854775808_isize);
_1 = [6_usize,3299191794397627186_usize,15182128919224607808_usize,681198470789509060_usize,3_usize,4_usize,7_usize];
_9.3 = !false;
_9.3 = _7 <= _7;
_9.0 = [203_u8,66_u8,208_u8,79_u8,227_u8,147_u8,63_u8,89_u8];
_9.2 = [1455785611_i32];
_2 = -(-7374081955360292168_i64);
_9.0 = _4;
_9.2 = [139187611_i32];
_3 = [(-2038157984_i32),1243899802_i32,(-433321836_i32),1769261060_i32,23478203_i32,(-952157606_i32),1153001199_i32];
_2 = 2_usize as i64;
_9.2 = [1777271443_i32];
_9.1 = 1082200302_i32 as i128;
_4 = [112_u8,181_u8,40_u8,33_u8,63_u8,237_u8,163_u8,16_u8];
Call(_9 = fn12(_3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_10 = _9.1 as isize;
_16.2 = (_9.0, _9.1, _20.fld0.0, _9.3);
_16.1 = 100941855_u32 * 3531727579_u32;
_5 = _13;
_9.1 = -_16.2.1;
_11 = _20.fld1.0.0 as f64;
match _20.fld1.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431319426083 => bb10,
_ => bb9
}
}
bb13 = {
_16.3 = Move(RET);
_9 = (_16.2.0, _16.2.1, _20.fld0.0, _16.2.3);
_16.2.3 = !_9.3;
_20.fld1.0 = (1798706606_i32,);
_20.fld1.0 = ((-448785373_i32),);
_21 = &_20.fld1.0.0;
_9.1 = _9.3 as i128;
_8 = core::ptr::addr_of!(_14);
_9 = (_16.2.0, _16.2.1, _16.2.2, _16.2.3);
_10 = _15 ^ _7;
_26 = 4955482417238317946655551848707073056_u128 as f32;
_9.3 = _16.2.3;
_16.2.1 = -_9.1;
_21 = &_23.0;
_24 = &_16.2.2;
_7 = _10 ^ _10;
_16.0 = [16_i8,95_i8,34_i8];
match _20.fld1.0.0 {
340282366920938463463374607431319426083 => bb8,
_ => bb5
}
}
bb14 = {
_9.0 = [176_u8,51_u8,46_u8,168_u8,231_u8,240_u8,68_u8,80_u8];
_5 = '\u{715b8}';
_7 = _10;
Goto(bb4)
}
bb15 = {
_9.0 = [219_u8,139_u8,219_u8,116_u8,179_u8,56_u8,238_u8,224_u8];
_4 = [201_u8,110_u8,213_u8,183_u8,88_u8,193_u8,115_u8,208_u8];
_4 = [35_u8,93_u8,69_u8,174_u8,91_u8,70_u8,114_u8,89_u8];
_3 = [597680893_i32,(-806571746_i32),(-1085008392_i32),1274267449_i32,838248489_i32,(-1266894180_i32),(-1685306184_i32)];
_4 = [0_u8,43_u8,0_u8,182_u8,169_u8,201_u8,210_u8,214_u8];
_3 = [(-1941792204_i32),1892240095_i32,(-1370591210_i32),(-653780020_i32),1769062126_i32,773738343_i32,494911073_i32];
_4 = [237_u8,176_u8,218_u8,94_u8,16_u8,16_u8,157_u8,160_u8];
_10 = _7 << _7;
Goto(bb3)
}
bb16 = {
_5 = '\u{f38b8}';
_3 = [(-1163074772_i32),(-1671243138_i32),466812331_i32,896939707_i32,(-10562393_i32),2052521256_i32,(-1725738723_i32)];
_3 = [842137970_i32,2055188941_i32,7977947_i32,39579905_i32,153381797_i32,(-1156442614_i32),(-2117765622_i32)];
_7 = !(-9223372036854775808_isize);
_1 = [6_usize,3299191794397627186_usize,15182128919224607808_usize,681198470789509060_usize,3_usize,4_usize,7_usize];
_9.3 = !false;
_9.3 = _7 <= _7;
_9.0 = [203_u8,66_u8,208_u8,79_u8,227_u8,147_u8,63_u8,89_u8];
_9.2 = [1455785611_i32];
_2 = -(-7374081955360292168_i64);
_9.0 = _4;
_9.2 = [139187611_i32];
_3 = [(-2038157984_i32),1243899802_i32,(-433321836_i32),1769261060_i32,23478203_i32,(-952157606_i32),1153001199_i32];
_2 = 2_usize as i64;
_9.2 = [1777271443_i32];
_9.1 = 1082200302_i32 as i128;
_4 = [112_u8,181_u8,40_u8,33_u8,63_u8,237_u8,163_u8,16_u8];
Call(_9 = fn12(_3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_11 = _16.2.1 as f64;
_22 = _20.fld0.1;
_20.fld1 = (_23,);
_10 = _7 * _7;
(*RET) = [_20.fld0.1,_13,_13,_13,_5,_5,_22];
_30 = core::ptr::addr_of_mut!(_3);
_15 = _7 << _12;
_4 = [165_u8,238_u8,205_u8,53_u8,113_u8,236_u8,3_u8,32_u8];
_31 = core::ptr::addr_of_mut!((*_30));
_33 = 8999839728354947748_u64 as isize;
_2 = _12;
_12 = _2;
_31 = core::ptr::addr_of_mut!((*_31));
(*_30) = [_17,_17,_23.0,_17,_17,_17,_23.0];
RET = core::ptr::addr_of!((*RET));
_20.fld0 = (_9.2, _5);
_35 = _15 >= _7;
_16.2 = (_4, _9.1, _9.2, _28);
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(11_usize, 32_usize, Move(_32), 17_usize, Move(_17), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(11_usize, 23_usize, Move(_23), 10_usize, Move(_10), 22_usize, Move(_22), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(11_usize, 4_usize, Move(_4), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [i32; 7],mut _2: [u8; 8]) -> ([u8; 8], i128, [i32; 1], bool) {
mir! {
type RET = ([u8; 8], i128, [i32; 1], bool);
let _3: &'static *mut i128;
let _4: u16;
let _5: [u128; 7];
let _6: f64;
let _7: isize;
let _8: &'static f64;
let _9: ([i32; 1], [u8; 8]);
let _10: [char; 4];
let _11: [i128; 6];
let _12: &'static [char; 7];
let _13: u16;
let _14: ();
let _15: ();
{
RET.0 = [168_u8,248_u8,186_u8,1_u8,211_u8,211_u8,170_u8,241_u8];
RET.3 = 172_u8 <= 101_u8;
_1 = [1880300671_i32,424351918_i32,362849334_i32,(-16901549_i32),(-302169434_i32),947961837_i32,40722781_i32];
_4 = 51481_u16;
_5 = [93799998219958917615571659298205096212_u128,160698872977552429999647845719636251385_u128,159791670135678753623086629014281661497_u128,19048942368681779881884584699053601274_u128,66551899208770572656331040006698377116_u128,59264559186508931581414766365990783551_u128,276691106751366993356106222582066812501_u128];
RET.3 = true ^ false;
_2 = [255_u8,222_u8,29_u8,134_u8,130_u8,203_u8,13_u8,139_u8];
RET.1 = 53103763790831049838329016039285286860_i128;
RET.2 = [365903389_i32];
RET.1 = 22900033838469972556851434623438738873_i128 | (-84300285226031247482189760665481843029_i128);
_6 = _4 as f64;
RET.0 = _2;
_4 = 24711_u16;
RET.2 = [1119396836_i32];
RET.2 = [441289915_i32];
RET.0 = _2;
_1 = [1089369849_i32,(-675534628_i32),(-2137365122_i32),(-268507038_i32),981501917_i32,(-1289337541_i32),716220087_i32];
_1 = [(-311101838_i32),78448892_i32,2043684395_i32,1076418306_i32,697952335_i32,366583454_i32,(-310985829_i32)];
_6 = (-29129_i16) as f64;
RET.0 = [34_u8,218_u8,185_u8,194_u8,103_u8,191_u8,186_u8,201_u8];
RET.1 = (-36068975396937048643288208199250170542_i128) & (-67681121209145013495812288754515524008_i128);
RET.0 = [131_u8,156_u8,151_u8,140_u8,206_u8,122_u8,96_u8,143_u8];
_7 = !(-71_isize);
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
24711 => bb6,
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
_2 = [233_u8,202_u8,241_u8,140_u8,231_u8,192_u8,128_u8,103_u8];
RET.0 = [173_u8,232_u8,45_u8,15_u8,7_u8,29_u8,21_u8,53_u8];
match _4 {
24711 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
RET.0 = [136_u8,50_u8,148_u8,159_u8,240_u8,87_u8,56_u8,92_u8];
RET.1 = 22731014057602778488657166841362826628_i128;
_7 = 14676195860955031169_u64 as isize;
_7 = 3689674125309749394_usize as isize;
_4 = (-18623_i16) as u16;
_2 = [207_u8,75_u8,62_u8,210_u8,38_u8,105_u8,18_u8,100_u8];
_9 = (RET.2, RET.0);
Goto(bb9)
}
bb9 = {
RET = (_2, 2520122505281931198519785300371201401_i128, _9.0, false);
RET.1 = -(-94496165677851143421759203589208860515_i128);
RET.2 = _9.0;
_8 = &_6;
RET.3 = (*_8) != _6;
RET.3 = true & true;
_6 = 233_u8 as f64;
RET.1 = 131013265273793692601850939976806200440_i128;
_7 = !(-9223372036854775808_isize);
_7 = (-9223372036854775808_isize);
RET.2 = [(-1913692767_i32)];
_4 = !54046_u16;
_9 = (RET.2, _2);
RET.0 = _2;
RET.3 = false | false;
_7 = 15_isize;
_7 = 310671190239117304576799796981932935611_u128 as isize;
_8 = &_6;
_6 = (-20943_i16) as f64;
RET.0 = _2;
RET.0 = _9.1;
RET.3 = _4 > _4;
RET.2 = [(-293898496_i32)];
_9.1 = _2;
_1 = [1626854533_i32,(-378110854_i32),(-676511388_i32),(-1687307213_i32),(-156694667_i32),(-46585746_i32),(-2068380183_i32)];
_5 = [155987473720103544452615316165894855994_u128,166458443755769911654514205546488351621_u128,328383215405226137191810867453545983300_u128,317627689510029621360595104511598347202_u128,177195226524053943537934773726101570938_u128,242587265340097598726376667613765047213_u128,29706134854830162587653719642796439601_u128];
match RET.1 {
0 => bb7,
1 => bb4,
2 => bb3,
3 => bb10,
4 => bb11,
131013265273793692601850939976806200440 => bb13,
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
_2 = [233_u8,202_u8,241_u8,140_u8,231_u8,192_u8,128_u8,103_u8];
RET.0 = [173_u8,232_u8,45_u8,15_u8,7_u8,29_u8,21_u8,53_u8];
match _4 {
24711 => bb8,
_ => bb7
}
}
bb13 = {
_8 = &_6;
RET.3 = !true;
RET = (_9.1, 37349587020123380641941983280034791179_i128, _9.0, true);
RET = (_2, (-3259482372870675167374673745955252330_i128), _9.0, false);
_11 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_9 = (RET.2, RET.0);
_5 = [117724786239733660692985248126158331609_u128,164993787353779011387724228370928536950_u128,94010185046292044955723410011986140226_u128,67153394076570930911213198782594293999_u128,333076582790341824394983064846473725031_u128,149011299083390867305469930641795245745_u128,261074953678007921923524371636507393637_u128];
RET.2 = [1876343770_i32];
Goto(bb14)
}
bb14 = {
Call(_14 = dump_var(12_usize, 11_usize, Move(_11), 7_usize, Move(_7), 2_usize, Move(_2), 15_usize, _15), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [u8; 8]) -> isize {
mir! {
type RET = isize;
let _2: *mut &'static (i32,);
let _3: Adt82;
let _4: ([u128; 7], (&'static u8,), &'static u8);
let _5: i16;
let _6: i128;
let _7: [char; 4];
let _8: f32;
let _9: *const [char; 4];
let _10: [i64; 5];
let _11: i16;
let _12: [i8; 3];
let _13: [i64; 5];
let _14: *mut [char; 7];
let _15: ((i32,),);
let _16: isize;
let _17: [char; 4];
let _18: &'static i32;
let _19: &'static *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _20: i64;
let _21: i16;
let _22: char;
let _23: ();
let _24: ();
{
RET = 56579066995951163689854641201199025788_i128 as isize;
RET = false as isize;
RET = -(-91_isize);
_1 = [75_u8,41_u8,220_u8,84_u8,73_u8,80_u8,199_u8,158_u8];
RET = (-70_isize) >> 93_i8;
RET = -9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
_1 = [87_u8,240_u8,49_u8,180_u8,129_u8,209_u8,143_u8,119_u8];
RET = 52_isize;
_1 = [21_u8,45_u8,69_u8,176_u8,147_u8,182_u8,145_u8,60_u8];
_1 = [46_u8,94_u8,86_u8,191_u8,76_u8,171_u8,100_u8,77_u8];
_5 = (-1951_i16);
RET = !83_isize;
_4.0 = [97748018768470921063398152103067933749_u128,212158543983556184085665415043263924378_u128,242599897125875194711211576684948119536_u128,84059245840817756524479664944642674833_u128,240600627058858664201507009621744918663_u128,277818501764498271265963165057427950200_u128,58121695366554159412281072364224169110_u128];
RET = (-16_isize) & 9223372036854775807_isize;
_4.0 = [114918576657991062201786425423496239120_u128,28400234376459877571473331605223561184_u128,137169135822159800178852255999283582625_u128,123753832586319259417414661676801824354_u128,306676540510080027722051420053430102381_u128,290037980184646862566916818250756041159_u128,164683300343354095362822548726210651651_u128];
_6 = 127390979851811801202109869903487684166_i128 + 87596056901316996056113955454511072033_i128;
_5 = !25218_i16;
_1 = [2_u8,246_u8,195_u8,25_u8,226_u8,156_u8,160_u8,134_u8];
_1 = [25_u8,147_u8,211_u8,221_u8,58_u8,86_u8,246_u8,81_u8];
RET = 37895_u16 as isize;
RET = 1747883380291968191_usize as isize;
RET = (-9223372036854775808_isize);
_4.0 = [131215984940469961389920463350562375307_u128,276979609767091758999816332406895347475_u128,49334824059222612455979423679057021534_u128,235178424679220962342291171129069300143_u128,67583629847487914703185219118399512730_u128,110873062587710845077553386960502743337_u128,232267775620395405735738092991843638647_u128];
_6 = 2649836033_u32 as i128;
Goto(bb2)
}
bb2 = {
_6 = (-21115344482038043338492475353084894500_i128) ^ 155406295793695330593525514359454246803_i128;
_5 = (-14141_i16) ^ (-13787_i16);
_7 = ['\u{64100}','\u{fcd05}','\u{eb9ce}','\u{c0ec4}'];
_8 = (-66_i8) as f32;
Goto(bb3)
}
bb3 = {
_4.0 = [243975304951859779163468234155404036502_u128,101081464493097439398737327395708462122_u128,145360445873842883631413700542778597104_u128,169521211268439951357241728608365843149_u128,261734251549521913662636318238033944526_u128,49460590899209196371103018156513696376_u128,45356261535754998909704036502145944979_u128];
_1 = [32_u8,131_u8,123_u8,111_u8,108_u8,123_u8,167_u8,15_u8];
_11 = _5 >> _5;
_6 = (-65489428098644647876579983806476766259_i128) ^ 1097500920706405281708251479032223075_i128;
_9 = core::ptr::addr_of!(_7);
_11 = _5;
_13 = [(-8576062164467418122_i64),3320760551308858114_i64,(-2146264055721356863_i64),(-5904406286592755226_i64),7662721669528984306_i64];
_7 = ['\u{7141e}','\u{ababa}','\u{2d6c5}','\u{beda9}'];
(*_9) = ['\u{104efb}','\u{96dfa}','\u{ffa63}','\u{54d2a}'];
_10 = [3284892830203578423_i64,(-7984016542535740268_i64),958514750795574222_i64,(-8835124194148226516_i64),1415854437536603477_i64];
_8 = _5 as f32;
_10 = [3882464742315872859_i64,2408788356270125697_i64,540913408512679716_i64,(-1680137239453918675_i64),6595545411619436013_i64];
(*_9) = ['\u{c715}','\u{241db}','\u{87f5c}','\u{b0a43}'];
RET = 9223372036854775807_isize;
_12 = [(-36_i8),(-63_i8),(-108_i8)];
_5 = _11;
_16 = -RET;
match RET {
0 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb4 = {
_6 = (-21115344482038043338492475353084894500_i128) ^ 155406295793695330593525514359454246803_i128;
_5 = (-14141_i16) ^ (-13787_i16);
_7 = ['\u{64100}','\u{fcd05}','\u{eb9ce}','\u{c0ec4}'];
_8 = (-66_i8) as f32;
Goto(bb3)
}
bb5 = {
_1 = [87_u8,240_u8,49_u8,180_u8,129_u8,209_u8,143_u8,119_u8];
RET = 52_isize;
_1 = [21_u8,45_u8,69_u8,176_u8,147_u8,182_u8,145_u8,60_u8];
_1 = [46_u8,94_u8,86_u8,191_u8,76_u8,171_u8,100_u8,77_u8];
_5 = (-1951_i16);
RET = !83_isize;
_4.0 = [97748018768470921063398152103067933749_u128,212158543983556184085665415043263924378_u128,242599897125875194711211576684948119536_u128,84059245840817756524479664944642674833_u128,240600627058858664201507009621744918663_u128,277818501764498271265963165057427950200_u128,58121695366554159412281072364224169110_u128];
RET = (-16_isize) & 9223372036854775807_isize;
_4.0 = [114918576657991062201786425423496239120_u128,28400234376459877571473331605223561184_u128,137169135822159800178852255999283582625_u128,123753832586319259417414661676801824354_u128,306676540510080027722051420053430102381_u128,290037980184646862566916818250756041159_u128,164683300343354095362822548726210651651_u128];
_6 = 127390979851811801202109869903487684166_i128 + 87596056901316996056113955454511072033_i128;
_5 = !25218_i16;
_1 = [2_u8,246_u8,195_u8,25_u8,226_u8,156_u8,160_u8,134_u8];
_1 = [25_u8,147_u8,211_u8,221_u8,58_u8,86_u8,246_u8,81_u8];
RET = 37895_u16 as isize;
RET = 1747883380291968191_usize as isize;
RET = (-9223372036854775808_isize);
_4.0 = [131215984940469961389920463350562375307_u128,276979609767091758999816332406895347475_u128,49334824059222612455979423679057021534_u128,235178424679220962342291171129069300143_u128,67583629847487914703185219118399512730_u128,110873062587710845077553386960502743337_u128,232267775620395405735738092991843638647_u128];
_6 = 2649836033_u32 as i128;
Goto(bb2)
}
bb6 = {
_15.0.0 = -1179746775_i32;
_15.0.0 = !10011159_i32;
_4.0 = [179389006006410105015251864896855972288_u128,99565039399991135059157794887333624890_u128,97741313290622233845189780305622772819_u128,38515002304089430887619227528558675323_u128,109149458789309542502697894234952363456_u128,82588216750041537334759091901331147300_u128,111112467035245007946972095976318516666_u128];
_16 = RET;
_13 = _10;
_16 = RET;
_10 = _13;
(*_9) = ['\u{bc540}','\u{108832}','\u{4503b}','\u{2e997}'];
_12 = [(-16_i8),(-111_i8),52_i8];
_6 = 57684487852609036726864359393001962387_i128 ^ 155176414628769415966611166148827781426_i128;
_11 = _5 & _5;
_9 = core::ptr::addr_of!(_7);
_11 = -_5;
_15.0 = ((-957603478_i32),);
_1 = [222_u8,66_u8,95_u8,124_u8,23_u8,29_u8,183_u8,173_u8];
_1 = [225_u8,137_u8,189_u8,10_u8,157_u8,23_u8,100_u8,172_u8];
_6 = !(-158217308126437323718774260031222995866_i128);
_13 = [5273094168061571859_i64,(-1090212939977619545_i64),(-782150443041604167_i64),(-394186792771941810_i64),(-1632972085103041281_i64)];
_12 = [60_i8,68_i8,14_i8];
_12 = [(-73_i8),(-3_i8),115_i8];
RET = !_16;
Goto(bb7)
}
bb7 = {
_10 = [4226483170503608826_i64,(-8813388447904516657_i64),(-3984641489701788177_i64),(-5539847277131128680_i64),(-6609687328315011998_i64)];
_16 = !RET;
RET = _16;
_6 = !(-72225624582493851938156854657715781701_i128);
_12 = [(-12_i8),(-15_i8),47_i8];
_7 = ['\u{feace}','\u{dd167}','\u{1e09}','\u{a230c}'];
_17 = (*_9);
_8 = 11233229562307390292_u64 as f32;
_4.0 = [29213843978251663548909915821460620663_u128,200000230661171932049421659954839725273_u128,182265556659304657343305012147815127893_u128,290825531407601927383290396452242992074_u128,27546158926724600928715474379547557064_u128,242193136354121457660506254549338981901_u128,298453971882266390706452284146808136189_u128];
_20 = -6896276334878020771_i64;
_18 = &_15.0.0;
_15.0 = ((-1177312424_i32),);
match _15.0.0 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607430590899032 => bb15,
_ => bb14
}
}
bb8 = {
_15.0.0 = -1179746775_i32;
_15.0.0 = !10011159_i32;
_4.0 = [179389006006410105015251864896855972288_u128,99565039399991135059157794887333624890_u128,97741313290622233845189780305622772819_u128,38515002304089430887619227528558675323_u128,109149458789309542502697894234952363456_u128,82588216750041537334759091901331147300_u128,111112467035245007946972095976318516666_u128];
_16 = RET;
_13 = _10;
_16 = RET;
_10 = _13;
(*_9) = ['\u{bc540}','\u{108832}','\u{4503b}','\u{2e997}'];
_12 = [(-16_i8),(-111_i8),52_i8];
_6 = 57684487852609036726864359393001962387_i128 ^ 155176414628769415966611166148827781426_i128;
_11 = _5 & _5;
_9 = core::ptr::addr_of!(_7);
_11 = -_5;
_15.0 = ((-957603478_i32),);
_1 = [222_u8,66_u8,95_u8,124_u8,23_u8,29_u8,183_u8,173_u8];
_1 = [225_u8,137_u8,189_u8,10_u8,157_u8,23_u8,100_u8,172_u8];
_6 = !(-158217308126437323718774260031222995866_i128);
_13 = [5273094168061571859_i64,(-1090212939977619545_i64),(-782150443041604167_i64),(-394186792771941810_i64),(-1632972085103041281_i64)];
_12 = [60_i8,68_i8,14_i8];
_12 = [(-73_i8),(-3_i8),115_i8];
RET = !_16;
Goto(bb7)
}
bb9 = {
_1 = [87_u8,240_u8,49_u8,180_u8,129_u8,209_u8,143_u8,119_u8];
RET = 52_isize;
_1 = [21_u8,45_u8,69_u8,176_u8,147_u8,182_u8,145_u8,60_u8];
_1 = [46_u8,94_u8,86_u8,191_u8,76_u8,171_u8,100_u8,77_u8];
_5 = (-1951_i16);
RET = !83_isize;
_4.0 = [97748018768470921063398152103067933749_u128,212158543983556184085665415043263924378_u128,242599897125875194711211576684948119536_u128,84059245840817756524479664944642674833_u128,240600627058858664201507009621744918663_u128,277818501764498271265963165057427950200_u128,58121695366554159412281072364224169110_u128];
RET = (-16_isize) & 9223372036854775807_isize;
_4.0 = [114918576657991062201786425423496239120_u128,28400234376459877571473331605223561184_u128,137169135822159800178852255999283582625_u128,123753832586319259417414661676801824354_u128,306676540510080027722051420053430102381_u128,290037980184646862566916818250756041159_u128,164683300343354095362822548726210651651_u128];
_6 = 127390979851811801202109869903487684166_i128 + 87596056901316996056113955454511072033_i128;
_5 = !25218_i16;
_1 = [2_u8,246_u8,195_u8,25_u8,226_u8,156_u8,160_u8,134_u8];
_1 = [25_u8,147_u8,211_u8,221_u8,58_u8,86_u8,246_u8,81_u8];
RET = 37895_u16 as isize;
RET = 1747883380291968191_usize as isize;
RET = (-9223372036854775808_isize);
_4.0 = [131215984940469961389920463350562375307_u128,276979609767091758999816332406895347475_u128,49334824059222612455979423679057021534_u128,235178424679220962342291171129069300143_u128,67583629847487914703185219118399512730_u128,110873062587710845077553386960502743337_u128,232267775620395405735738092991843638647_u128];
_6 = 2649836033_u32 as i128;
Goto(bb2)
}
bb10 = {
_6 = (-21115344482038043338492475353084894500_i128) ^ 155406295793695330593525514359454246803_i128;
_5 = (-14141_i16) ^ (-13787_i16);
_7 = ['\u{64100}','\u{fcd05}','\u{eb9ce}','\u{c0ec4}'];
_8 = (-66_i8) as f32;
Goto(bb3)
}
bb11 = {
_4.0 = [243975304951859779163468234155404036502_u128,101081464493097439398737327395708462122_u128,145360445873842883631413700542778597104_u128,169521211268439951357241728608365843149_u128,261734251549521913662636318238033944526_u128,49460590899209196371103018156513696376_u128,45356261535754998909704036502145944979_u128];
_1 = [32_u8,131_u8,123_u8,111_u8,108_u8,123_u8,167_u8,15_u8];
_11 = _5 >> _5;
_6 = (-65489428098644647876579983806476766259_i128) ^ 1097500920706405281708251479032223075_i128;
_9 = core::ptr::addr_of!(_7);
_11 = _5;
_13 = [(-8576062164467418122_i64),3320760551308858114_i64,(-2146264055721356863_i64),(-5904406286592755226_i64),7662721669528984306_i64];
_7 = ['\u{7141e}','\u{ababa}','\u{2d6c5}','\u{beda9}'];
(*_9) = ['\u{104efb}','\u{96dfa}','\u{ffa63}','\u{54d2a}'];
_10 = [3284892830203578423_i64,(-7984016542535740268_i64),958514750795574222_i64,(-8835124194148226516_i64),1415854437536603477_i64];
_8 = _5 as f32;
_10 = [3882464742315872859_i64,2408788356270125697_i64,540913408512679716_i64,(-1680137239453918675_i64),6595545411619436013_i64];
(*_9) = ['\u{c715}','\u{241db}','\u{87f5c}','\u{b0a43}'];
RET = 9223372036854775807_isize;
_12 = [(-36_i8),(-63_i8),(-108_i8)];
_5 = _11;
_16 = -RET;
match RET {
0 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb12 = {
_6 = (-21115344482038043338492475353084894500_i128) ^ 155406295793695330593525514359454246803_i128;
_5 = (-14141_i16) ^ (-13787_i16);
_7 = ['\u{64100}','\u{fcd05}','\u{eb9ce}','\u{c0ec4}'];
_8 = (-66_i8) as f32;
Goto(bb3)
}
bb13 = {
_1 = [87_u8,240_u8,49_u8,180_u8,129_u8,209_u8,143_u8,119_u8];
RET = 52_isize;
_1 = [21_u8,45_u8,69_u8,176_u8,147_u8,182_u8,145_u8,60_u8];
_1 = [46_u8,94_u8,86_u8,191_u8,76_u8,171_u8,100_u8,77_u8];
_5 = (-1951_i16);
RET = !83_isize;
_4.0 = [97748018768470921063398152103067933749_u128,212158543983556184085665415043263924378_u128,242599897125875194711211576684948119536_u128,84059245840817756524479664944642674833_u128,240600627058858664201507009621744918663_u128,277818501764498271265963165057427950200_u128,58121695366554159412281072364224169110_u128];
RET = (-16_isize) & 9223372036854775807_isize;
_4.0 = [114918576657991062201786425423496239120_u128,28400234376459877571473331605223561184_u128,137169135822159800178852255999283582625_u128,123753832586319259417414661676801824354_u128,306676540510080027722051420053430102381_u128,290037980184646862566916818250756041159_u128,164683300343354095362822548726210651651_u128];
_6 = 127390979851811801202109869903487684166_i128 + 87596056901316996056113955454511072033_i128;
_5 = !25218_i16;
_1 = [2_u8,246_u8,195_u8,25_u8,226_u8,156_u8,160_u8,134_u8];
_1 = [25_u8,147_u8,211_u8,221_u8,58_u8,86_u8,246_u8,81_u8];
RET = 37895_u16 as isize;
RET = 1747883380291968191_usize as isize;
RET = (-9223372036854775808_isize);
_4.0 = [131215984940469961389920463350562375307_u128,276979609767091758999816332406895347475_u128,49334824059222612455979423679057021534_u128,235178424679220962342291171129069300143_u128,67583629847487914703185219118399512730_u128,110873062587710845077553386960502743337_u128,232267775620395405735738092991843638647_u128];
_6 = 2649836033_u32 as i128;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_15.0.0 = (-100_i8) as i32;
_17 = ['\u{790f3}','\u{81201}','\u{f2560}','\u{cac2e}'];
_10 = [_20,_20,_20,_20,_20];
_21 = -_11;
_4.0 = [187794834406462676570741176795761349894_u128,321262395896269377129628712642059483481_u128,122936660204459482826676645800008496982_u128,103328433037929384625269446529667374451_u128,262908034968432240086789925220788950893_u128,477290725020193016053786466055958885_u128,83851615332722784848926557263721466434_u128];
_9 = core::ptr::addr_of!(_7);
(*_9) = ['\u{3ad0b}','\u{985bf}','\u{2c755}','\u{30b32}'];
_22 = '\u{10d6d5}';
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(13_usize, 17_usize, Move(_17), 7_usize, Move(_7), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(13_usize, 10_usize, Move(_10), 20_usize, Move(_20), 11_usize, Move(_11), 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: [u8; 8],mut _3: [u8; 8],mut _4: [u8; 8],mut _5: [i8; 3],mut _6: [i8; 3],mut _7: bool,mut _8: char) -> (*const Adt17, u64, *const bool, *mut &'static Adt17) {
mir! {
type RET = (*const Adt17, u64, *const bool, *mut &'static Adt17);
let _9: bool;
let _10: u16;
let _11: *mut ([u8; 8], i128, [i32; 1], bool);
let _12: i8;
let _13: char;
let _14: u32;
let _15: isize;
let _16: f32;
let _17: [i64; 5];
let _18: (([i32; 1], [u8; 8]), *const u8, &'static i32, &'static [u128; 7]);
let _19: [char; 4];
let _20: [u128; 7];
let _21: f64;
let _22: Adt17;
let _23: *const u8;
let _24: ([i32; 1], char);
let _25: u32;
let _26: *mut *mut &'static Adt17;
let _27: &'static i16;
let _28: &'static i16;
let _29: bool;
let _30: [usize; 7];
let _31: isize;
let _32: [u128; 7];
let _33: u8;
let _34: u64;
let _35: &'static [char; 7];
let _36: Adt72;
let _37: (([u8; 8], i128, [i32; 1], bool), u8);
let _38: isize;
let _39: u8;
let _40: isize;
let _41: i8;
let _42: &'static u16;
let _43: ([char; 7], [i8; 3], i32);
let _44: f32;
let _45: isize;
let _46: &'static (i32,);
let _47: char;
let _48: Adt33;
let _49: &'static &'static i16;
let _50: bool;
let _51: *const [char; 7];
let _52: &'static [char; 7];
let _53: &'static [i32; 1];
let _54: (i32,);
let _55: i64;
let _56: &'static Adt17;
let _57: Adt72;
let _58: Adt82;
let _59: isize;
let _60: *mut [char; 7];
let _61: *const [char; 4];
let _62: Adt40;
let _63: u128;
let _64: [usize; 5];
let _65: [i32; 7];
let _66: *mut [i32; 7];
let _67: Adt39;
let _68: i8;
let _69: ();
let _70: ();
{
_6 = _5;
_6 = [98_i8,(-33_i8),58_i8];
RET.1 = 4244998478652271254_u64;
_6 = [110_i8,(-74_i8),(-46_i8)];
RET.2 = core::ptr::addr_of!(_7);
RET.2 = core::ptr::addr_of!(_1);
_3 = _2;
Call(_6 = fn15(_8, _2, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = !_1;
_5 = _6;
_2 = [6_u8,149_u8,8_u8,142_u8,124_u8,13_u8,175_u8,171_u8];
_1 = _7 < _7;
_7 = _1;
_6 = [71_i8,(-24_i8),30_i8];
_5 = [66_i8,85_i8,(-112_i8)];
_3 = _2;
RET.2 = core::ptr::addr_of!(_1);
RET.2 = core::ptr::addr_of!(_7);
_1 = !_7;
_8 = '\u{4c159}';
_9 = RET.1 < RET.1;
_9 = _7;
match RET.1 {
0 => bb2,
4244998478652271254 => bb4,
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
_6 = [52_i8,26_i8,95_i8];
_6 = [55_i8,63_i8,41_i8];
match RET.1 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4244998478652271254 => bb8,
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
_7 = !_1;
_5 = _6;
_2 = [6_u8,149_u8,8_u8,142_u8,124_u8,13_u8,175_u8,171_u8];
_1 = _7 < _7;
_7 = _1;
_6 = [71_i8,(-24_i8),30_i8];
_5 = [66_i8,85_i8,(-112_i8)];
_3 = _2;
RET.2 = core::ptr::addr_of!(_1);
RET.2 = core::ptr::addr_of!(_7);
_1 = !_7;
_8 = '\u{4c159}';
_9 = RET.1 < RET.1;
_9 = _7;
match RET.1 {
0 => bb2,
4244998478652271254 => bb4,
_ => bb3
}
}
bb8 = {
_9 = _7 & _7;
_5 = [76_i8,29_i8,35_i8];
_5 = _6;
_3 = [208_u8,15_u8,153_u8,181_u8,10_u8,93_u8,123_u8,53_u8];
_1 = _9;
_2 = _4;
_10 = 51011_u16;
_9 = !_7;
_4 = [251_u8,127_u8,130_u8,135_u8,80_u8,167_u8,26_u8,223_u8];
RET.1 = 39918028_u32 as u64;
RET.2 = core::ptr::addr_of!(_1);
_7 = !_9;
RET.1 = !17823496770387161868_u64;
_3 = [214_u8,11_u8,31_u8,115_u8,217_u8,80_u8,203_u8,14_u8];
_2 = [251_u8,57_u8,19_u8,136_u8,8_u8,37_u8,187_u8,244_u8];
_8 = '\u{10298a}';
_12 = 81_i8 >> RET.1;
_7 = _9 == _9;
match _10 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
51011 => bb15,
_ => bb14
}
}
bb9 = {
_7 = !_1;
_5 = _6;
_2 = [6_u8,149_u8,8_u8,142_u8,124_u8,13_u8,175_u8,171_u8];
_1 = _7 < _7;
_7 = _1;
_6 = [71_i8,(-24_i8),30_i8];
_5 = [66_i8,85_i8,(-112_i8)];
_3 = _2;
RET.2 = core::ptr::addr_of!(_1);
RET.2 = core::ptr::addr_of!(_7);
_1 = !_7;
_8 = '\u{4c159}';
_9 = RET.1 < RET.1;
_9 = _7;
match RET.1 {
0 => bb2,
4244998478652271254 => bb4,
_ => bb3
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_7 = !_1;
_5 = _6;
_2 = [6_u8,149_u8,8_u8,142_u8,124_u8,13_u8,175_u8,171_u8];
_1 = _7 < _7;
_7 = _1;
_6 = [71_i8,(-24_i8),30_i8];
_5 = [66_i8,85_i8,(-112_i8)];
_3 = _2;
RET.2 = core::ptr::addr_of!(_1);
RET.2 = core::ptr::addr_of!(_7);
_1 = !_7;
_8 = '\u{4c159}';
_9 = RET.1 < RET.1;
_9 = _7;
match RET.1 {
0 => bb2,
4244998478652271254 => bb4,
_ => bb3
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_12 = -(-57_i8);
RET.1 = !17277710078637071441_u64;
_14 = _9 as u32;
_13 = _8;
_2 = [63_u8,124_u8,216_u8,75_u8,95_u8,14_u8,129_u8,251_u8];
_13 = _8;
_14 = 0_usize as u32;
RET.2 = core::ptr::addr_of!(_7);
_13 = _8;
_13 = _8;
_1 = _7;
_4 = [241_u8,43_u8,50_u8,193_u8,234_u8,124_u8,22_u8,231_u8];
_9 = !_7;
RET.1 = 2482_i16 as u64;
_10 = !59602_u16;
_13 = _8;
_16 = 22312963300247655171530040024380438273_i128 as f32;
_14 = 2170322473_u32 + 513774084_u32;
Goto(bb16)
}
bb16 = {
_8 = _13;
_15 = -(-9223372036854775808_isize);
_7 = _1;
_15 = _16 as isize;
_12 = _10 as i8;
_17 = [(-879564947196071044_i64),(-4739690192604708482_i64),(-5935516039280470504_i64),7119601110843093978_i64,3307152906214844017_i64];
_12 = _13 as i8;
_10 = !50584_u16;
RET.1 = 4898850002610715448_u64 >> _12;
_19 = [_8,_8,_13,_13];
_9 = !_1;
_17 = [6435778284503010793_i64,7969680106612404850_i64,(-1788015992244683665_i64),(-1300180595873281729_i64),3482107412804695498_i64];
_7 = _9;
_3 = [111_u8,57_u8,70_u8,187_u8,52_u8,217_u8,170_u8,37_u8];
RET.2 = core::ptr::addr_of!(_7);
_12 = -39_i8;
_22 = Adt17::Variant1 { fld0: _14,fld1: _8,fld2: _15,fld3: 7268625546954950387_usize,fld4: RET.1,fld5: _10,fld6: _16 };
place!(Field::<usize>(Variant(_22, 1), 3)) = !11955118934934025329_usize;
RET.1 = Field::<u64>(Variant(_22, 1), 4) ^ Field::<u64>(Variant(_22, 1), 4);
Goto(bb17)
}
bb17 = {
_18.0.1 = [80_u8,241_u8,52_u8,255_u8,152_u8,85_u8,216_u8,94_u8];
Goto(bb18)
}
bb18 = {
_14 = _1 as u32;
place!(Field::<usize>(Variant(_22, 1), 3)) = 249_u8 as usize;
_14 = Field::<u32>(Variant(_22, 1), 0);
_2 = [187_u8,166_u8,31_u8,82_u8,152_u8,6_u8,242_u8,228_u8];
Goto(bb19)
}
bb19 = {
_24.0 = [1251317389_i32];
place!(Field::<char>(Variant(_22, 1), 1)) = _13;
place!(Field::<isize>(Variant(_22, 1), 2)) = 240_u8 as isize;
RET.2 = core::ptr::addr_of!(_1);
_22 = Adt17::Variant2 { fld0: _10,fld1: _8,fld2: 190_u8 };
_16 = 331997355777057249912350987693463869000_u128 as f32;
_18.0 = (_24.0, _4);
RET.0 = core::ptr::addr_of!(_22);
_18.3 = &_20;
_1 = _9;
_2 = _3;
_18.0.0 = [(-844542837_i32)];
_25 = _14 << RET.1;
_32 = [124632348811924967146367610290983195085_u128,205330130064690491368043733123499852933_u128,167666402892254022854737163138751311378_u128,201568866008752851693823057941768944613_u128,45161458167844574954264665371476588796_u128,191514371561139630246410419086036703290_u128,87277830249285831527630644514567895808_u128];
_17 = [(-7337177903498594630_i64),(-7975897091612674417_i64),(-5072531688713004082_i64),(-8630985793781756550_i64),2253561013959802555_i64];
_8 = Field::<char>(Variant(_22, 2), 1);
_25 = !_14;
_6 = [_12,_12,_12];
_24 = (_18.0.0, Field::<char>(Variant(_22, 2), 1));
_23 = core::ptr::addr_of!(_33);
_13 = _8;
Goto(bb20)
}
bb20 = {
RET.0 = core::ptr::addr_of!(_22);
_12 = (-6_i8);
_17 = [7404766345537058197_i64,4909272422203230676_i64,488422313413539334_i64,(-5974302533836942144_i64),(-3166880853656773570_i64)];
_33 = !64_u8;
_18.0.1 = [(*_23),(*_23),(*_23),(*_23),(*_23),_33,_33,(*_23)];
_24.1 = Field::<char>(Variant(_22, 2), 1);
_17 = [(-4493743772545301263_i64),8278949539718252954_i64,(-3172614774853149506_i64),(-1749685290940465201_i64),(-621073106372223401_i64)];
_31 = (-1487518730259733629_i64) as isize;
_18.0.1 = [(*_23),(*_23),_33,(*_23),(*_23),(*_23),_33,(*_23)];
_22 = Adt17::Variant0 { fld0: (-4630322578063830447_i64),fld1: _10,fld2: _31,fld3: 321322260127075360579907507387002649348_u128,fld4: 124194913736106056428712136298627161641_i128 };
RET.0 = core::ptr::addr_of!(_22);
RET.0 = core::ptr::addr_of!(_22);
place!(Field::<u128>(Variant(_22, 0), 3)) = !15951618947799583149243173533964353951_u128;
place!(Field::<i128>(Variant(_22, 0), 4)) = 142041102532058961458008820654493897380_i128 >> _14;
Call(place!(Field::<i128>(Variant(_22, 0), 4)) = core::intrinsics::bswap((-151168038018972625112119906131076050313_i128)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_12 = 11_i8;
_25 = _33 as u32;
_18.0.1 = [(*_23),_33,(*_23),(*_23),(*_23),(*_23),_33,_33];
_28 = &_36.fld1.0;
_18.0.1 = _2;
place!(Field::<u128>(Variant(_22, 0), 3)) = _16 as u128;
_37.0 = (_3, Field::<i128>(Variant(_22, 0), 4), _24.0, _7);
_18.1 = core::ptr::addr_of!(_33);
_34 = RET.1;
_37.0 = (_2, Field::<i128>(Variant(_22, 0), 4), _24.0, _9);
_37.1 = (*_23);
_37.0 = (_2, Field::<i128>(Variant(_22, 0), 4), _24.0, _7);
_36.fld0 = [_24.1,_13,_24.1,_8,_13,_24.1,_13];
_20 = [Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3)];
_36.fld4 = core::ptr::addr_of_mut!(_37.0);
place!(Field::<i128>(Variant(_22, 0), 4)) = _37.0.1;
Goto(bb22)
}
bb22 = {
place!(Field::<i128>(Variant(_22, 0), 4)) = _13 as i128;
_36.fld0 = [_24.1,_8,_13,_24.1,_13,_24.1,_24.1];
_38 = _31 << Field::<i128>(Variant(_22, 0), 4);
_18.0.0 = [(-1797633989_i32)];
_29 = !_1;
_11 = Move(_36.fld4);
place!(Field::<u16>(Variant(_22, 0), 1)) = _10;
_24.0 = [(-721817905_i32)];
_18.1 = core::ptr::addr_of!(_36.fld2);
_20 = [Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3),Field::<u128>(Variant(_22, 0), 3)];
RET.2 = core::ptr::addr_of!(_37.0.3);
place!(Field::<u128>(Variant(_22, 0), 3)) = !79346763971372844293725259712509076859_u128;
_26 = core::ptr::addr_of_mut!(RET.3);
_18.3 = &_20;
_22 = Adt17::Variant2 { fld0: _10,fld1: _24.1,fld2: (*_23) };
RET.0 = core::ptr::addr_of!(_22);
_27 = &(*_28);
_2 = [(*_23),_33,(*_23),(*_23),Field::<u8>(Variant(_22, 2), 2),_37.1,_37.1,_33];
_36.fld1.0 = 24305_i16 - 16857_i16;
_11 = core::ptr::addr_of_mut!(_37.0);
_2 = [_33,_33,(*_23),(*_23),_33,(*_23),_37.1,(*_23)];
Goto(bb23)
}
bb23 = {
_26 = core::ptr::addr_of_mut!((*_26));
RET.0 = core::ptr::addr_of!(_22);
(*_11) = (_4, 83628575348554790138044860944556384919_i128, _24.0, _1);
_2 = _18.0.1;
(*_11).3 = _7;
_43.0 = [Field::<char>(Variant(_22, 2), 1),_24.1,_24.1,_8,_24.1,_24.1,Field::<char>(Variant(_22, 2), 1)];
SetDiscriminant(_22, 2);
Goto(bb24)
}
bb24 = {
place!(Field::<char>(Variant(_22, 2), 1)) = _13;
(*_11).1 = !100031897545814155686363339224123347948_i128;
_18.0.1 = [_33,(*_23),(*_23),(*_23),_33,(*_23),(*_23),_37.1];
_34 = !RET.1;
_36.fld2 = (*_23) | (*_23);
_37.0.1 = 57427898017098326050899828411530186699_i128 << _36.fld2;
_42 = &place!(Field::<u16>(Variant(_22, 2), 0));
match _12 {
0 => bb6,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
11 => bb32,
_ => bb31
}
}
bb25 = {
_9 = _7 & _7;
_5 = [76_i8,29_i8,35_i8];
_5 = _6;
_3 = [208_u8,15_u8,153_u8,181_u8,10_u8,93_u8,123_u8,53_u8];
_1 = _9;
_2 = _4;
_10 = 51011_u16;
_9 = !_7;
_4 = [251_u8,127_u8,130_u8,135_u8,80_u8,167_u8,26_u8,223_u8];
RET.1 = 39918028_u32 as u64;
RET.2 = core::ptr::addr_of!(_1);
_7 = !_9;
RET.1 = !17823496770387161868_u64;
_3 = [214_u8,11_u8,31_u8,115_u8,217_u8,80_u8,203_u8,14_u8];
_2 = [251_u8,57_u8,19_u8,136_u8,8_u8,37_u8,187_u8,244_u8];
_8 = '\u{10298a}';
_12 = 81_i8 >> RET.1;
_7 = _9 == _9;
match _10 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
51011 => bb15,
_ => bb14
}
}
bb26 = {
Return()
}
bb27 = {
_7 = !_1;
_5 = _6;
_2 = [6_u8,149_u8,8_u8,142_u8,124_u8,13_u8,175_u8,171_u8];
_1 = _7 < _7;
_7 = _1;
_6 = [71_i8,(-24_i8),30_i8];
_5 = [66_i8,85_i8,(-112_i8)];
_3 = _2;
RET.2 = core::ptr::addr_of!(_1);
RET.2 = core::ptr::addr_of!(_7);
_1 = !_7;
_8 = '\u{4c159}';
_9 = RET.1 < RET.1;
_9 = _7;
match RET.1 {
0 => bb2,
4244998478652271254 => bb4,
_ => bb3
}
}
bb28 = {
Return()
}
bb29 = {
Return()
}
bb30 = {
_7 = !_1;
_5 = _6;
_2 = [6_u8,149_u8,8_u8,142_u8,124_u8,13_u8,175_u8,171_u8];
_1 = _7 < _7;
_7 = _1;
_6 = [71_i8,(-24_i8),30_i8];
_5 = [66_i8,85_i8,(-112_i8)];
_3 = _2;
RET.2 = core::ptr::addr_of!(_1);
RET.2 = core::ptr::addr_of!(_7);
_1 = !_7;
_8 = '\u{4c159}';
_9 = RET.1 < RET.1;
_9 = _7;
match RET.1 {
0 => bb2,
4244998478652271254 => bb4,
_ => bb3
}
}
bb31 = {
_8 = _13;
_15 = -(-9223372036854775808_isize);
_7 = _1;
_15 = _16 as isize;
_12 = _10 as i8;
_17 = [(-879564947196071044_i64),(-4739690192604708482_i64),(-5935516039280470504_i64),7119601110843093978_i64,3307152906214844017_i64];
_12 = _13 as i8;
_10 = !50584_u16;
RET.1 = 4898850002610715448_u64 >> _12;
_19 = [_8,_8,_13,_13];
_9 = !_1;
_17 = [6435778284503010793_i64,7969680106612404850_i64,(-1788015992244683665_i64),(-1300180595873281729_i64),3482107412804695498_i64];
_7 = _9;
_3 = [111_u8,57_u8,70_u8,187_u8,52_u8,217_u8,170_u8,37_u8];
RET.2 = core::ptr::addr_of!(_7);
_12 = -39_i8;
_22 = Adt17::Variant1 { fld0: _14,fld1: _8,fld2: _15,fld3: 7268625546954950387_usize,fld4: RET.1,fld5: _10,fld6: _16 };
place!(Field::<usize>(Variant(_22, 1), 3)) = !11955118934934025329_usize;
RET.1 = Field::<u64>(Variant(_22, 1), 4) ^ Field::<u64>(Variant(_22, 1), 4);
Goto(bb17)
}
bb32 = {
_32 = [155659013782642696396401240042251819748_u128,146429487896660394164766773981590418717_u128,17729940523816407352100215537653674562_u128,305913069252251837792543712598333152270_u128,183798159109020443012870428848112557466_u128,130962722337900977291173387849667691351_u128,123898677223732538969733263032401634018_u128];
_43.1 = _6;
_17 = [(-6620367282652544800_i64),(-2337641780589006325_i64),(-3215977608297456026_i64),5644311890226204303_i64,3094521055361074368_i64];
_36.fld4 = Move(_11);
_37.0.1 = _36.fld1.0 as i128;
_10 = !37641_u16;
_10 = 26271_u16 ^ 39714_u16;
_27 = &_36.fld1.0;
Goto(bb33)
}
bb33 = {
_19 = [Field::<char>(Variant(_22, 2), 1),Field::<char>(Variant(_22, 2), 1),Field::<char>(Variant(_22, 2), 1),_24.1];
_13 = Field::<char>(Variant(_22, 2), 1);
_39 = _36.fld2 | _36.fld2;
RET.0 = core::ptr::addr_of!(_22);
_36.fld1.0 = (-14390_i16) << _34;
_44 = 1954951443_i32 as f32;
_29 = _1 | _37.0.3;
_35 = &_36.fld0;
_47 = Field::<char>(Variant(_22, 2), 1);
_18.0.1 = [_36.fld2,_33,_37.1,_39,_33,_39,_39,(*_23)];
match _12 {
0 => bb9,
1 => bb17,
2 => bb4,
3 => bb34,
4 => bb35,
5 => bb36,
11 => bb38,
_ => bb37
}
}
bb34 = {
Return()
}
bb35 = {
_12 = -(-57_i8);
RET.1 = !17277710078637071441_u64;
_14 = _9 as u32;
_13 = _8;
_2 = [63_u8,124_u8,216_u8,75_u8,95_u8,14_u8,129_u8,251_u8];
_13 = _8;
_14 = 0_usize as u32;
RET.2 = core::ptr::addr_of!(_7);
_13 = _8;
_13 = _8;
_1 = _7;
_4 = [241_u8,43_u8,50_u8,193_u8,234_u8,124_u8,22_u8,231_u8];
_9 = !_7;
RET.1 = 2482_i16 as u64;
_10 = !59602_u16;
_13 = _8;
_16 = 22312963300247655171530040024380438273_i128 as f32;
_14 = 2170322473_u32 + 513774084_u32;
Goto(bb16)
}
bb36 = {
Return()
}
bb37 = {
Return()
}
bb38 = {
_17 = [3444652270901475957_i64,(-5967966453534264620_i64),(-1250407011191780225_i64),1198744368308976996_i64,(-8552480876828279428_i64)];
_34 = 29659323124305935447953239773183250306_u128 as u64;
_13 = _24.1;
_11 = core::ptr::addr_of_mut!(_37.0);
_4 = _37.0.0;
_11 = Move(_36.fld4);
_23 = Move(_18.1);
_36.fld1.0 = (-451_i16);
_50 = !_1;
_36.fld1.1 = 131547443716687558125621176113844756698_u128;
_7 = _9;
_10 = _1 as u16;
_51 = core::ptr::addr_of!((*_35));
_43.1 = [_12,_12,_12];
_18.2 = &_43.2;
RET.2 = core::ptr::addr_of!(_29);
Goto(bb39)
}
bb39 = {
_31 = -_38;
_41 = _12;
_11 = core::ptr::addr_of_mut!(_37.0);
_22 = Adt17::Variant2 { fld0: _10,fld1: _24.1,fld2: _39 };
_17 = [(-5209860810791427667_i64),2646089957536962166_i64,2419396382962397674_i64,5870237096271926202_i64,8128449853096070699_i64];
_45 = !_31;
_31 = -_38;
_18.0.1 = _37.0.0;
_39 = Field::<u8>(Variant(_22, 2), 2) & _36.fld2;
_17 = [(-7445689653390708982_i64),6476494182048660436_i64,3077489123630029141_i64,7892251734905272758_i64,4008229087599328223_i64];
_29 = (*_11).3;
_43.0 = [_47,_13,Field::<char>(Variant(_22, 2), 1),Field::<char>(Variant(_22, 2), 1),_24.1,Field::<char>(Variant(_22, 2), 1),_47];
RET.1 = _34;
_46 = &_54;
_27 = &_36.fld1.0;
_5 = [_12,_41,_12];
_40 = !_31;
_9 = (*_11).3;
_24 = ((*_11).2, Field::<char>(Variant(_22, 2), 1));
_42 = &_10;
_39 = Field::<u8>(Variant(_22, 2), 2) << (*_42);
_21 = 157267120_i32 as f64;
SetDiscriminant(_22, 0);
place!(Field::<u128>(Variant(_22, 0), 3)) = !_36.fld1.1;
(*_11).3 = _7 & _50;
_32 = [Field::<u128>(Variant(_22, 0), 3),_36.fld1.1,Field::<u128>(Variant(_22, 0), 3),_36.fld1.1,_36.fld1.1,Field::<u128>(Variant(_22, 0), 3),_36.fld1.1];
Goto(bb40)
}
bb40 = {
_44 = -_16;
RET.0 = core::ptr::addr_of!(_22);
_18.0 = (_24.0, _4);
_36.fld1.0 = 22615_i16;
(*_11).1 = 3430310881456021195657562981507425907_i128;
_14 = !_25;
_9 = !_7;
_18.0.1 = [_36.fld2,_36.fld2,_39,_39,_37.1,_37.1,_39,_36.fld2];
_30 = [5029872901715205036_usize,10667449892346580243_usize,13282326083777762308_usize,7704774419553377412_usize,1_usize,4_usize,0_usize];
_26 = core::ptr::addr_of_mut!((*_26));
_49 = &_27;
_43.2 = (-1403061805_i32) * (-505296920_i32);
_21 = (*_11).1 as f64;
_57.fld3 = _25 >> _43.2;
_19 = [_24.1,_47,_8,_13];
Goto(bb41)
}
bb41 = {
_29 = _37.0.3;
_35 = &(*_35);
_37.0 = (_4, (-39019446889904184900042469498397653693_i128), _24.0, _50);
_54 = (_43.2,);
_22 = Adt17::Variant2 { fld0: (*_42),fld1: _47,fld2: _39 };
_26 = core::ptr::addr_of_mut!((*_26));
_57.fld0 = [_47,_8,_8,_47,_47,_24.1,_47];
(*_11).2 = _18.0.0;
_37.0 = (_18.0.1, 143375540513325814604262143713227078006_i128, _18.0.0, _50);
_43.0 = (*_35);
_48 = Adt33::Variant0 { fld0: (*_42),fld1: Move(_22),fld2: _57.fld0,fld3: _36.fld1.1 };
place!(Field::<[char; 7]>(Variant(_48, 0), 2)) = [_8,Field::<char>(Variant(Field::<Adt17>(Variant(_48, 0), 1), 2), 1),Field::<char>(Variant(Field::<Adt17>(Variant(_48, 0), 1), 2), 1),_8,_13,_8,_24.1];
place!(Field::<Adt17>(Variant(_48, 0), 1)) = Adt17::Variant3 { fld0: _29,fld1: 5_usize,fld2: RET.1,fld3: _12,fld4: _36.fld1.0,fld5: _43.2,fld6: 3424189421778930827_i64,fld7: _33 };
_36.fld3 = (-7783345361405661573_i64) as u32;
_13 = _8;
_57.fld4 = Move(_11);
_64 = [3_usize,16398753637577257753_usize,2524842898193853603_usize,0_usize,4992146381293075183_usize];
(*_26) = core::ptr::addr_of_mut!(_56);
_53 = &_37.0.2;
_54 = (_43.2,);
_57.fld1.0 = !_36.fld1.0;
Goto(bb42)
}
bb42 = {
Call(_69 = dump_var(14_usize, 31_usize, Move(_31), 14_usize, Move(_14), 30_usize, Move(_30), 32_usize, Move(_32)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_69 = dump_var(14_usize, 43_usize, Move(_43), 2_usize, Move(_2), 20_usize, Move(_20), 25_usize, Move(_25)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_69 = dump_var(14_usize, 64_usize, Move(_64), 33_usize, Move(_33), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_69 = dump_var(14_usize, 9_usize, Move(_9), 17_usize, Move(_17), 29_usize, Move(_29), 54_usize, Move(_54)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_69 = dump_var(14_usize, 39_usize, Move(_39), 34_usize, Move(_34), 70_usize, _70, 70_usize, _70), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: char,mut _2: [u8; 8],mut _3: [i8; 3]) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _4: f32;
let _5: char;
let _6: Adt39;
let _7: ([char; 7], [i8; 3], i32);
let _8: [i32; 1];
let _9: u16;
let _10: i16;
let _11: isize;
let _12: &'static i32;
let _13: Adt72;
let _14: &'static [u128; 7];
let _15: f64;
let _16: &'static Adt17;
let _17: *mut *mut &'static Adt17;
let _18: *const Adt17;
let _19: [u128; 7];
let _20: &'static i32;
let _21: ([char; 7], [i8; 3], i32);
let _22: *const [char; 4];
let _23: u8;
let _24: isize;
let _25: i16;
let _26: isize;
let _27: u128;
let _28: u16;
let _29: u64;
let _30: [u128; 7];
let _31: *mut [u8; 8];
let _32: char;
let _33: u16;
let _34: &'static u16;
let _35: isize;
let _36: f64;
let _37: u32;
let _38: char;
let _39: ();
let _40: ();
{
RET = [101_i8,29_i8,(-39_i8)];
_1 = '\u{f04b6}';
RET = _3;
Goto(bb1)
}
bb1 = {
_4 = (-30832_i16) as f32;
_3 = RET;
_3 = RET;
_4 = 12868647470946657617_u64 as f32;
_2 = [142_u8,10_u8,31_u8,238_u8,91_u8,156_u8,228_u8,63_u8];
_1 = '\u{bd863}';
_1 = '\u{e6fb7}';
_3 = [(-9_i8),(-33_i8),(-66_i8)];
Call(_3 = fn16(_1, _1, _4, _2, _2, _2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = '\u{3b697}';
_1 = '\u{c73df}';
RET = _3;
_7.1 = [(-14_i8),(-58_i8),115_i8];
_2 = [114_u8,13_u8,129_u8,216_u8,71_u8,220_u8,172_u8,54_u8];
_4 = (-9223372036854775808_isize) as f32;
_7.2 = 2022644111_i32 ^ (-1939627826_i32);
_8 = [_7.2];
_1 = '\u{b4a06}';
_5 = _1;
_4 = 7275625518243903401714228660035868744_i128 as f32;
_4 = 11789803307913299344_usize as f32;
_5 = _1;
RET = _3;
_7.2 = (-2113687503_i32);
_1 = _5;
_1 = _5;
RET = [54_i8,(-92_i8),24_i8];
_7.2 = 1611336973_i32 << (-3674_i16);
Goto(bb3)
}
bb3 = {
_10 = (-8058_i16);
RET = _7.1;
_9 = (-61_i8) as u16;
_2 = [104_u8,64_u8,7_u8,191_u8,29_u8,196_u8,93_u8,85_u8];
_7.0 = [_5,_5,_5,_5,_1,_5,_1];
_10 = !(-24305_i16);
_13.fld2 = 90_u8 ^ 39_u8;
_13.fld1 = (_10, 277028933823929090014383119859919871383_u128);
Call(_13 = fn17(RET, RET, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = [2_i8,1_i8,(-62_i8)];
_8 = [_7.2];
_9 = 48538_u16;
_2 = [_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2,_13.fld2];
_10 = _13.fld1.0 >> _13.fld1.1;
_7.0 = _13.fld0;
Call(_11 = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = (_13.fld0, RET, 74109942_i32);
_13.fld0 = [_5,_1,_5,_1,_1,_1,_5];
_5 = _1;
_9 = _1 as u16;
_4 = (-9223372036854775808_isize) as f32;
_12 = &_7.2;
_12 = &(*_12);
RET = [(-91_i8),33_i8,(-56_i8)];
_12 = &(*_12);
_9 = 5644_u16 | 24874_u16;
_8 = [(*_12)];
Call(_7.1 = core::intrinsics::transmute(RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = _7.1;
_1 = _5;
_12 = &(*_12);
_7.1 = [17_i8,73_i8,80_i8];
_4 = _13.fld3 as f32;
_15 = 5474076115347472265_i64 as f64;
_13.fld1 = (_10, 220371427046860153418711435178576897517_u128);
_13.fld1.0 = _10;
_13.fld1.1 = 193240948136269080739943706768738071824_u128 - 102446568616349553125308015699598801651_u128;
_10 = _13.fld1.0 ^ _13.fld1.0;
_12 = &(*_12);
_5 = _1;
_13.fld1.1 = !249977644813487570062059530936352073431_u128;
_13.fld1.0 = 5854807756917042124_u64 as i16;
_7 = (_13.fld0, RET, 1695672453_i32);
_7.1 = [(-44_i8),9_i8,(-61_i8)];
_4 = 1708519145817566758_usize as f32;
_10 = _13.fld1.0;
_4 = 13472844115427605179_u64 as f32;
RET = _3;
_8 = [_7.2];
_13.fld0 = [_1,_1,_1,_5,_1,_5,_5];
_20 = &_7.2;
RET = [51_i8,36_i8,103_i8];
_5 = _1;
_7.0 = [_1,_1,_1,_5,_5,_1,_5];
Goto(bb7)
}
bb7 = {
_7.1 = [(-107_i8),69_i8,(-56_i8)];
_23 = _13.fld2 + _13.fld2;
_10 = _13.fld1.0 ^ _13.fld1.0;
_10 = _13.fld1.0 + _13.fld1.0;
RET = [37_i8,(-104_i8),(-89_i8)];
_24 = 9223372036854775807_isize;
_3 = RET;
Goto(bb8)
}
bb8 = {
_21.0 = [_1,_5,_5,_1,_5,_5,_1];
_27 = !_13.fld1.1;
_11 = !_24;
_11 = _24 | _24;
_24 = _11;
_21 = (_13.fld0, RET, _7.2);
_10 = _13.fld1.0;
_24 = _9 as isize;
_13.fld2 = _23;
_13.fld1.0 = _10 ^ _10;
_24 = _13.fld1.1 as isize;
_27 = _13.fld1.1 & _13.fld1.1;
_8 = [(*_20)];
_7.2 = _21.2;
_13.fld1 = (_10, _27);
_2 = [_23,_13.fld2,_13.fld2,_13.fld2,_23,_23,_23,_23];
match _7.2 {
0 => bb1,
1 => bb6,
2 => bb5,
3 => bb9,
1695672453 => bb11,
_ => bb10
}
}
bb9 = {
_7 = (_13.fld0, RET, 74109942_i32);
_13.fld0 = [_5,_1,_5,_1,_1,_1,_5];
_5 = _1;
_9 = _1 as u16;
_4 = (-9223372036854775808_isize) as f32;
_12 = &_7.2;
_12 = &(*_12);
RET = [(-91_i8),33_i8,(-56_i8)];
_12 = &(*_12);
_9 = 5644_u16 | 24874_u16;
_8 = [(*_12)];
Call(_7.1 = core::intrinsics::transmute(RET), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_10 = (-8058_i16);
RET = _7.1;
_9 = (-61_i8) as u16;
_2 = [104_u8,64_u8,7_u8,191_u8,29_u8,196_u8,93_u8,85_u8];
_7.0 = [_5,_5,_5,_5,_1,_5,_1];
_10 = !(-24305_i16);
_13.fld2 = 90_u8 ^ 39_u8;
_13.fld1 = (_10, 277028933823929090014383119859919871383_u128);
Call(_13 = fn17(RET, RET, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_21.2 = -_7.2;
_21.0 = [_5,_1,_5,_5,_5,_5,_1];
_14 = &_19;
_28 = _5 as u16;
_25 = _10 & _10;
_13.fld2 = _10 as u8;
_7.1 = _3;
_24 = _11;
_4 = 12680944699752290311_u64 as f32;
_13.fld1 = (_25, _27);
_21.1 = [(-95_i8),120_i8,119_i8];
_7 = (_21.0, RET, _21.2);
_1 = _5;
_15 = _13.fld1.0 as f64;
_30 = [_13.fld1.1,_27,_27,_27,_13.fld1.1,_27,_13.fld1.1];
_29 = 10006911913858344493_u64;
_7.1 = [(-44_i8),19_i8,28_i8];
_2 = [_23,_23,_13.fld2,_23,_23,_13.fld2,_23,_23];
RET = [16_i8,(-125_i8),18_i8];
_26 = _13.fld3 as isize;
_13.fld1.1 = _27 - _27;
_24 = _5 as isize;
_4 = _11 as f32;
_21.0 = _7.0;
_7.2 = _21.2 & _21.2;
_7 = _21;
_1 = _5;
RET = [(-82_i8),(-47_i8),(-34_i8)];
RET = _7.1;
Goto(bb12)
}
bb12 = {
_19 = _30;
_7 = (_13.fld0, _21.1, _21.2);
_24 = -_11;
_13.fld1 = (_25, _27);
Goto(bb13)
}
bb13 = {
_21.0 = _7.0;
_32 = _1;
_31 = core::ptr::addr_of_mut!(_2);
_8 = [_21.2];
_14 = &_19;
_21.1 = [(-77_i8),79_i8,(-79_i8)];
_33 = !_9;
_8 = [_21.2];
_21.0 = [_1,_5,_1,_32,_1,_32,_5];
_35 = _11;
_11 = _35 | _24;
Goto(bb14)
}
bb14 = {
_21.2 = _35 as i32;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(15_usize, 3_usize, Move(_3), 5_usize, Move(_5), 35_usize, Move(_35), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(15_usize, 26_usize, Move(_26), 24_usize, Move(_24), 32_usize, Move(_32), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(15_usize, 9_usize, Move(_9), 33_usize, Move(_33), 25_usize, Move(_25), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: char,mut _2: char,mut _3: f32,mut _4: [u8; 8],mut _5: [u8; 8],mut _6: [u8; 8],mut _7: f32) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _8: (i32,);
let _9: *const *const Adt17;
let _10: ([i32; 1], [u8; 8]);
let _11: *const *mut [i32; 7];
let _12: u64;
let _13: i8;
let _14: (*mut i128, u64, Adt33);
let _15: Adt72;
let _16: Adt40;
let _17: i32;
let _18: f32;
let _19: isize;
let _20: ();
let _21: ();
{
_5 = [40_u8,161_u8,129_u8,213_u8,176_u8,90_u8,40_u8,244_u8];
_8.0 = 1207632872_i32 >> 47823_u16;
_8 = ((-1500581568_i32),);
_10.0 = [_8.0];
_12 = 4115923984155288450_u64 >> _8.0;
_5 = _6;
match _8.0 {
0 => bb1,
340282366920938463463374607430267629888 => bb3,
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
_10.0 = [_8.0];
RET = [(-79_i8),(-104_i8),(-126_i8)];
_6 = _4;
_12 = 4163296648183733873_u64;
_10.0 = [_8.0];
_14.1 = _12 + _12;
_4 = [103_u8,249_u8,130_u8,128_u8,12_u8,175_u8,88_u8,10_u8];
_13 = 25_u8 as i8;
_15.fld0 = [_2,_1,_2,_2,_2,_1,_2];
_4 = [167_u8,31_u8,176_u8,6_u8,242_u8,184_u8,171_u8,4_u8];
_15.fld2 = _2 as u8;
_15.fld1.0 = (-14983_i16) ^ 22540_i16;
_8.0 = !1648479004_i32;
_14.1 = 156025084576575054998071848818799510210_i128 as u64;
_12 = _14.1 & _14.1;
Goto(bb4)
}
bb4 = {
_5 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_15.fld3 = 3817054956_u32;
_15.fld1.1 = 46910819591983659303824094554198249482_u128 - 146217964606542926169793026502368415667_u128;
_14.1 = !_12;
_15.fld2 = !169_u8;
_8.0 = _3 as i32;
_15.fld1.1 = 304764660657066239410416201955110437308_u128 + 127826123641859338308641896824639517247_u128;
_10.1 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_15.fld1 = ((-14835_i16), 250367631505474226445418536868121640543_u128);
_2 = _1;
Goto(bb5)
}
bb5 = {
_15.fld0 = [_2,_2,_1,_2,_1,_2,_2];
_3 = _15.fld2 as f32;
_13 = (-124_i8);
_10.1 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
RET = [_13,_13,_13];
RET = [_13,_13,_13];
_3 = 34146_u16 as f32;
_10.0 = [_8.0];
_5 = _4;
_15.fld1 = ((-21180_i16), 98164162306736831254701138492869962754_u128);
_10.0 = [_8.0];
match _15.fld1.1 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
98164162306736831254701138492869962754 => bb12,
_ => bb11
}
}
bb6 = {
_5 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_15.fld3 = 3817054956_u32;
_15.fld1.1 = 46910819591983659303824094554198249482_u128 - 146217964606542926169793026502368415667_u128;
_14.1 = !_12;
_15.fld2 = !169_u8;
_8.0 = _3 as i32;
_15.fld1.1 = 304764660657066239410416201955110437308_u128 + 127826123641859338308641896824639517247_u128;
_10.1 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_15.fld1 = ((-14835_i16), 250367631505474226445418536868121640543_u128);
_2 = _1;
Goto(bb5)
}
bb7 = {
_10.0 = [_8.0];
RET = [(-79_i8),(-104_i8),(-126_i8)];
_6 = _4;
_12 = 4163296648183733873_u64;
_10.0 = [_8.0];
_14.1 = _12 + _12;
_4 = [103_u8,249_u8,130_u8,128_u8,12_u8,175_u8,88_u8,10_u8];
_13 = 25_u8 as i8;
_15.fld0 = [_2,_1,_2,_2,_2,_1,_2];
_4 = [167_u8,31_u8,176_u8,6_u8,242_u8,184_u8,171_u8,4_u8];
_15.fld2 = _2 as u8;
_15.fld1.0 = (-14983_i16) ^ 22540_i16;
_8.0 = !1648479004_i32;
_14.1 = 156025084576575054998071848818799510210_i128 as u64;
_12 = _14.1 & _14.1;
Goto(bb4)
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
_15.fld0 = [_1,_1,_2,_2,_2,_1,_1];
_8.0 = -666115234_i32;
_16 = Adt40::Variant0 { fld0: _15.fld1.0,fld1: _2,fld2: _7,fld3: 14074_u16 };
_15.fld1 = (Field::<i16>(Variant(_16, 0), 0), 77168549396023242525456008615596699724_u128);
RET = [_13,_13,_13];
_15.fld3 = true as u32;
_15.fld3 = !1179420782_u32;
_13 = _3 as i8;
place!(Field::<u16>(Variant(_16, 0), 3)) = !48451_u16;
_2 = Field::<char>(Variant(_16, 0), 1);
_5 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_3 = -Field::<f32>(Variant(_16, 0), 2);
_1 = _2;
_15.fld3 = false as u32;
_8.0 = 9223372036854775807_isize as i32;
_12 = _14.1 + _14.1;
_8.0 = (-2103361113_i32) | (-590910046_i32);
match Field::<i16>(Variant(_16, 0), 0) {
340282366920938463463374607431768190276 => bb14,
_ => bb13
}
}
bb13 = {
_5 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_15.fld3 = 3817054956_u32;
_15.fld1.1 = 46910819591983659303824094554198249482_u128 - 146217964606542926169793026502368415667_u128;
_14.1 = !_12;
_15.fld2 = !169_u8;
_8.0 = _3 as i32;
_15.fld1.1 = 304764660657066239410416201955110437308_u128 + 127826123641859338308641896824639517247_u128;
_10.1 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_15.fld1 = ((-14835_i16), 250367631505474226445418536868121640543_u128);
_2 = _1;
Goto(bb5)
}
bb14 = {
_17 = !_8.0;
_15.fld2 = _14.1 as u8;
_15.fld1 = (Field::<i16>(Variant(_16, 0), 0), 329432783953801218374298329541178845737_u128);
_10.1 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_8.0 = _17;
_18 = -_7;
SetDiscriminant(_16, 0);
_15.fld2 = !7_u8;
_16 = Adt40::Variant1 { fld0: _15.fld3,fld1: _15.fld1.1,fld2: _10.0,fld3: 1_usize,fld4: _15.fld1.0,fld5: _12,fld6: (-5884583625993287835_i64),fld7: (-116113341634092829095073830093769325793_i128) };
_12 = Field::<u64>(Variant(_16, 1), 5);
place!(Field::<[i32; 1]>(Variant(_16, 1), 2)) = [_17];
_14.0 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_16, 1), 7)));
_7 = _3 - _18;
_12 = !Field::<u64>(Variant(_16, 1), 5);
place!(Field::<[i32; 1]>(Variant(_16, 1), 2)) = [_17];
place!(Field::<i64>(Variant(_16, 1), 6)) = !3092559863361417389_i64;
_13 = (-2_i8) | 49_i8;
_19 = (-9223372036854775808_isize) * (-64_isize);
place!(Field::<u32>(Variant(_16, 1), 0)) = !_15.fld3;
_10.1 = _6;
_15.fld3 = Field::<u32>(Variant(_16, 1), 0) - Field::<u32>(Variant(_16, 1), 0);
_15.fld0 = [_1,_1,_1,_2,_2,_2,_1];
_17 = _8.0 ^ _8.0;
place!(Field::<[i32; 1]>(Variant(_16, 1), 2)) = [_17];
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(16_usize, 17_usize, Move(_17), 8_usize, Move(_8), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(16_usize, 4_usize, Move(_4), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [i8; 3],mut _2: [i8; 3],mut _3: char) -> Adt72 {
mir! {
type RET = Adt72;
let _4: isize;
let _5: i128;
let _6: *const Adt17;
let _7: (([i32; 1], [u8; 8]), *const u8, &'static i32, &'static [u128; 7]);
let _8: i32;
let _9: ([u128; 7], (&'static u8,), &'static u8);
let _10: u32;
let _11: bool;
let _12: u32;
let _13: bool;
let _14: f32;
let _15: &'static *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _16: f64;
let _17: i128;
let _18: char;
let _19: [u8; 8];
let _20: bool;
let _21: [u128; 7];
let _22: (u8, [i8; 3], &'static u8, u16);
let _23: u16;
let _24: &'static i32;
let _25: Adt33;
let _26: &'static (i32,);
let _27: &'static [i32; 1];
let _28: &'static i32;
let _29: &'static u8;
let _30: bool;
let _31: Adt40;
let _32: &'static [u128; 7];
let _33: char;
let _34: [usize; 5];
let _35: f64;
let _36: &'static *mut ([i32; 1], &'static i16, i64, [char; 7]);
let _37: [i32; 1];
let _38: u128;
let _39: *const [char; 4];
let _40: (u8, [i8; 3], &'static u8, u16);
let _41: ([i32; 1], char);
let _42: &'static u16;
let _43: bool;
let _44: f32;
let _45: u64;
let _46: [char; 4];
let _47: bool;
let _48: i128;
let _49: *const *const Adt17;
let _50: Adt72;
let _51: f64;
let _52: (([u8; 8], i128, [i32; 1], bool), u8);
let _53: Adt76;
let _54: char;
let _55: &'static i16;
let _56: [i32; 7];
let _57: char;
let _58: ();
let _59: ();
{
RET.fld2 = (-8010951034247396865_i64) as u8;
RET.fld2 = !229_u8;
_1 = [25_i8,67_i8,0_i8];
RET.fld1 = (31665_i16, 131353078384732493331866340827934931566_u128);
_1 = _2;
RET.fld1 = (7168_i16, 263081096920092001404250785957136158115_u128);
_4 = 64_isize;
RET.fld2 = !143_u8;
RET.fld2 = !188_u8;
RET.fld1.1 = 113199173034314432847817761673596177859_u128;
_3 = '\u{d3ad9}';
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
_1 = [(-53_i8),124_i8,111_i8];
RET.fld3 = 160678076_u32;
RET.fld3 = 1429762106508885760_u64 as u32;
RET.fld1 = (2797_i16, 126924549433962789661040674147767101308_u128);
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld3 = !1684609296_u32;
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld1 = ((-6291_i16), 157150015608757135842521836654630840981_u128);
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld1.0 = 27131_i16 | 24405_i16;
RET.fld2 = 73_u8 << RET.fld1.1;
RET.fld3 = 1212754236_i32 as u32;
RET.fld1.1 = (-4968045378120884410_i64) as u128;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
64 => bb7,
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
_5 = RET.fld2 as i128;
RET.fld1 = ((-11127_i16), 99434884809017979019998944858406912026_u128);
RET.fld1 = ((-18103_i16), 231442883107650046317051758120542020656_u128);
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
_7.0.0 = [1468325936_i32];
_4 = 57841_u16 as isize;
_2 = [(-63_i8),(-128_i8),(-71_i8)];
_8 = !(-1605905954_i32);
_7.2 = &_8;
_8 = 880530028_i32;
RET.fld1 = ((-10099_i16), 31891078082242712143098589982546885637_u128);
_1 = [(-42_i8),(-112_i8),(-64_i8)];
_7.1 = core::ptr::addr_of!(RET.fld2);
RET.fld1.1 = _5 as u128;
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb8)
}
bb8 = {
_8 = 890934128_i32;
RET.fld1 = (3750_i16, 237614191626469377064549839068475687068_u128);
RET.fld3 = !73795828_u32;
_7.0.0 = [_8];
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
_7.0.1 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
RET.fld1.0 = !(-13836_i16);
_7.0.0 = [_8];
_1 = _2;
_7.0.0 = [_8];
RET.fld1.1 = !323860243853987380717168032616870226725_u128;
_8 = (-503692957_i32) + (-218798781_i32);
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld2 = !84_u8;
RET.fld3 = !760748104_u32;
_7.2 = &_8;
_9.2 = &RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_4 = _8 as isize;
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
Call(RET.fld2 = fn18(Move(_7.2), _7.0, _7.0.1, _1, RET.fld3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7.1 = core::ptr::addr_of!(RET.fld2);
Call(RET.fld1.1 = core::intrinsics::bswap(227974810825900373144484627354051182450_u128), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = -110736154308601348037248436566436798035_i128;
Goto(bb11)
}
bb11 = {
_5 = 39670_u16 as i128;
RET.fld1.1 = _8 as u128;
RET.fld3 = 2809630554_u32;
_3 = '\u{967c8}';
RET.fld1.1 = 250221725432483060282153835261615327708_u128;
_9.2 = &RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld1.1 = 16078250835299515898_usize as u128;
_13 = !true;
_7.1 = core::ptr::addr_of!(RET.fld2);
match RET.fld3 {
2809630554 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_7.2 = &_8;
_16 = RET.fld3 as f64;
RET.fld1.0 = 29329_i16 >> _4;
_7.0.1 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld1 = ((-5600_i16), 263743140191730796574361853476301762612_u128);
_9.1 = (Move(_9.2),);
_10 = 115_i8 as u32;
_5 = 120_i8 as i128;
RET.fld1.1 = !337853415057928441894648184048917673227_u128;
_7.3 = &_9.0;
_11 = _13;
_3 = '\u{c3b27}';
_3 = '\u{d3a9a}';
_1 = _2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_2 = [35_i8,(-92_i8),4_i8];
_7.3 = &_21;
_5 = (-61666987609864337198417474369185422047_i128);
_9.1.0 = &RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld1.0 = RET.fld1.1 as i16;
Goto(bb14)
}
bb14 = {
_3 = '\u{ad2c7}';
_14 = 6_i8 as f32;
_9.1.0 = &RET.fld2;
_11 = _13 ^ _13;
RET.fld2 = RET.fld1.1 as u8;
RET.fld2 = 9_u8;
_22.3 = RET.fld3 as u16;
_19 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_5 = 105343899241111691631506021973436199211_i128 << RET.fld1.1;
_23 = _22.3 & _22.3;
_20 = _23 < _23;
_12 = !RET.fld3;
_22.2 = &_22.0;
_7.3 = &_21;
_18 = _3;
RET.fld2 = !9_u8;
_22.1 = _2;
match RET.fld3 {
0 => bb8,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
2809630554 => bb20,
_ => bb19
}
}
bb15 = {
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_7.2 = &_8;
_16 = RET.fld3 as f64;
RET.fld1.0 = 29329_i16 >> _4;
_7.0.1 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld1 = ((-5600_i16), 263743140191730796574361853476301762612_u128);
_9.1 = (Move(_9.2),);
_10 = 115_i8 as u32;
_5 = 120_i8 as i128;
RET.fld1.1 = !337853415057928441894648184048917673227_u128;
_7.3 = &_9.0;
_11 = _13;
_3 = '\u{c3b27}';
_3 = '\u{d3a9a}';
_1 = _2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_2 = [35_i8,(-92_i8),4_i8];
_7.3 = &_21;
_5 = (-61666987609864337198417474369185422047_i128);
_9.1.0 = &RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld1.0 = RET.fld1.1 as i16;
Goto(bb14)
}
bb16 = {
Return()
}
bb17 = {
_5 = 39670_u16 as i128;
RET.fld1.1 = _8 as u128;
RET.fld3 = 2809630554_u32;
_3 = '\u{967c8}';
RET.fld1.1 = 250221725432483060282153835261615327708_u128;
_9.2 = &RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld1.1 = 16078250835299515898_usize as u128;
_13 = !true;
_7.1 = core::ptr::addr_of!(RET.fld2);
match RET.fld3 {
2809630554 => bb13,
_ => bb12
}
}
bb18 = {
Return()
}
bb19 = {
_5 = RET.fld2 as i128;
RET.fld1 = ((-11127_i16), 99434884809017979019998944858406912026_u128);
RET.fld1 = ((-18103_i16), 231442883107650046317051758120542020656_u128);
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
_7.0.0 = [1468325936_i32];
_4 = 57841_u16 as isize;
_2 = [(-63_i8),(-128_i8),(-71_i8)];
_8 = !(-1605905954_i32);
_7.2 = &_8;
_8 = 880530028_i32;
RET.fld1 = ((-10099_i16), 31891078082242712143098589982546885637_u128);
_1 = [(-42_i8),(-112_i8),(-64_i8)];
_7.1 = core::ptr::addr_of!(RET.fld2);
RET.fld1.1 = _5 as u128;
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb8)
}
bb20 = {
_9.1.0 = &RET.fld2;
match RET.fld3 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb11,
2809630554 => bb21,
_ => bb12
}
}
bb21 = {
RET.fld1.1 = _16 as u128;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld0 = [_18,_18,_3,_18,_18,_3,_18];
_7.2 = &_8;
_28 = &_8;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_24 = Move(_28);
_7.2 = &_8;
RET.fld1.0 = -(-27698_i16);
_29 = &RET.fld2;
match RET.fld3 {
2809630554 => bb22,
_ => bb13
}
}
bb22 = {
RET.fld2 = 108_u8;
_17 = !_5;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld1.1 = 10681781424916608921_usize as u128;
_20 = !_11;
_22.2 = &_22.0;
_18 = _3;
_23 = _4 as u16;
_10 = !_12;
_7.3 = &_9.0;
_21 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_7.3 = &_21;
_32 = &_21;
RET.fld2 = 129_u8;
RET.fld2 = 252_u8;
_9.0 = (*_32);
_27 = &_7.0.0;
_20 = !_11;
_2 = _1;
_27 = &_7.0.0;
_22.3 = !_23;
Goto(bb23)
}
bb23 = {
_32 = &_21;
_9.1.0 = &_22.0;
_7.3 = &(*_32);
_18 = _3;
_32 = &(*_32);
_19 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_24 = &_8;
_30 = !_20;
_22.0 = !RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_8 = 865010537340300368_u64 as i32;
_9.1.0 = &RET.fld2;
RET.fld1.1 = 27439569977485840762863626486926122876_u128 - 248447463467222910331534358490792103733_u128;
_29 = &RET.fld2;
_16 = _22.0 as f64;
RET.fld2 = _22.0 << _23;
_8 = (-1411440094_i32) >> _23;
_7.0.1 = [RET.fld2,_22.0,_22.0,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_19 = [_22.0,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_31 = Adt40::Variant0 { fld0: RET.fld1.0,fld1: _18,fld2: _14,fld3: _22.3 };
_24 = &_8;
Goto(bb24)
}
bb24 = {
RET.fld2 = _22.0;
RET.fld1.0 = -Field::<i16>(Variant(_31, 0), 0);
_22.0 = RET.fld2 - RET.fld2;
_7.1 = core::ptr::addr_of!(RET.fld2);
_11 = _12 >= _12;
_21 = _9.0;
RET.fld1 = (Field::<i16>(Variant(_31, 0), 0), 214156285722563668545909579781029114408_u128);
_8 = Field::<char>(Variant(_31, 0), 1) as i32;
_31 = Adt40::Variant0 { fld0: RET.fld1.0,fld1: _3,fld2: _14,fld3: _23 };
_21 = _9.0;
_35 = _16;
_32 = &_9.0;
_7.2 = &_8;
_12 = _30 as u32;
_2 = [103_i8,18_i8,16_i8];
_22.2 = &RET.fld2;
place!(Field::<f32>(Variant(_31, 0), 2)) = _8 as f32;
_7.0.1 = [_22.0,_22.0,_22.0,RET.fld2,_22.0,RET.fld2,_22.0,_22.0];
_28 = &_8;
_29 = Move(_22.2);
_22.0 = !RET.fld2;
SetDiscriminant(_31, 0);
_13 = _11 ^ _11;
_28 = &(*_28);
_19 = _7.0.1;
_13 = _22.3 != _22.3;
_18 = _3;
_9.2 = &_40.0;
Goto(bb25)
}
bb25 = {
_24 = &(*_28);
_32 = &(*_32);
RET.fld2 = _22.0;
_22.2 = &RET.fld2;
RET.fld0 = [_18,_3,_18,_18,_3,_3,_18];
_41.1 = _18;
RET.fld1.0 = 8557_i16 & 10481_i16;
_7.3 = &_9.0;
_9.1 = (Move(_22.2),);
_40.1 = [0_i8,37_i8,(-31_i8)];
match RET.fld1.1 {
0 => bb26,
1 => bb27,
2 => bb28,
214156285722563668545909579781029114408 => bb30,
_ => bb29
}
}
bb26 = {
_5 = -110736154308601348037248436566436798035_i128;
Goto(bb11)
}
bb27 = {
_9.1.0 = &RET.fld2;
match RET.fld3 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb11,
2809630554 => bb21,
_ => bb12
}
}
bb28 = {
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_7.2 = &_8;
_16 = RET.fld3 as f64;
RET.fld1.0 = 29329_i16 >> _4;
_7.0.1 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
RET.fld1 = ((-5600_i16), 263743140191730796574361853476301762612_u128);
_9.1 = (Move(_9.2),);
_10 = 115_i8 as u32;
_5 = 120_i8 as i128;
RET.fld1.1 = !337853415057928441894648184048917673227_u128;
_7.3 = &_9.0;
_11 = _13;
_3 = '\u{c3b27}';
_3 = '\u{d3a9a}';
_1 = _2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_2 = [35_i8,(-92_i8),4_i8];
_7.3 = &_21;
_5 = (-61666987609864337198417474369185422047_i128);
_9.1.0 = &RET.fld2;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld1.0 = RET.fld1.1 as i16;
Goto(bb14)
}
bb29 = {
Return()
}
bb30 = {
place!(Field::<u16>(Variant(_31, 0), 3)) = _22.3 * _22.3;
_28 = &_8;
_21 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_35 = _14 as f64;
RET.fld1.0 = -19824_i16;
_33 = _18;
_40.1 = _22.1;
place!(Field::<i16>(Variant(_31, 0), 0)) = !RET.fld1.0;
_41 = ((*_27), _3);
_40.3 = 116_i8 as u16;
_2 = [26_i8,(-6_i8),(-22_i8)];
_9.1.0 = &_40.0;
_2 = _1;
_44 = 12725064261214949430_u64 as f32;
RET.fld2 = _22.0;
_38 = RET.fld1.1;
RET.fld1 = (Field::<i16>(Variant(_31, 0), 0), _38);
_22.2 = &_40.0;
RET.fld1.1 = 2367768560809652580_i64 as u128;
_16 = _35 + _35;
Goto(bb31)
}
bb31 = {
_37 = [_8];
_16 = _35;
_28 = &(*_28);
RET.fld1.1 = _38 + _38;
_43 = _30 | _30;
_23 = _10 as u16;
_1 = [(-37_i8),(-66_i8),127_i8];
_1 = [40_i8,(-104_i8),52_i8];
_3 = _33;
_7.1 = core::ptr::addr_of!(RET.fld2);
_27 = &(*_27);
_28 = &(*_28);
match _38 {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
5 => bb37,
6 => bb38,
214156285722563668545909579781029114408 => bb40,
_ => bb39
}
}
bb32 = {
RET.fld1.1 = _16 as u128;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
RET.fld0 = [_18,_18,_3,_18,_18,_3,_18];
_7.2 = &_8;
_28 = &_8;
_9.0 = [RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1,RET.fld1.1];
_24 = Move(_28);
_7.2 = &_8;
RET.fld1.0 = -(-27698_i16);
_29 = &RET.fld2;
match RET.fld3 {
2809630554 => bb22,
_ => bb13
}
}
bb33 = {
_5 = RET.fld2 as i128;
RET.fld1 = ((-11127_i16), 99434884809017979019998944858406912026_u128);
RET.fld1 = ((-18103_i16), 231442883107650046317051758120542020656_u128);
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
_7.0.0 = [1468325936_i32];
_4 = 57841_u16 as isize;
_2 = [(-63_i8),(-128_i8),(-71_i8)];
_8 = !(-1605905954_i32);
_7.2 = &_8;
_8 = 880530028_i32;
RET.fld1 = ((-10099_i16), 31891078082242712143098589982546885637_u128);
_1 = [(-42_i8),(-112_i8),(-64_i8)];
_7.1 = core::ptr::addr_of!(RET.fld2);
RET.fld1.1 = _5 as u128;
RET.fld0 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb8)
}
bb34 = {
Return()
}
bb35 = {
_7.1 = core::ptr::addr_of!(RET.fld2);
Call(RET.fld1.1 = core::intrinsics::bswap(227974810825900373144484627354051182450_u128), ReturnTo(bb10), UnwindUnreachable())
}
bb36 = {
_5 = -110736154308601348037248436566436798035_i128;
Goto(bb11)
}
bb37 = {
_24 = &(*_28);
_32 = &(*_32);
RET.fld2 = _22.0;
_22.2 = &RET.fld2;
RET.fld0 = [_18,_3,_18,_18,_3,_3,_18];
_41.1 = _18;
RET.fld1.0 = 8557_i16 & 10481_i16;
_7.3 = &_9.0;
_9.1 = (Move(_22.2),);
_40.1 = [0_i8,37_i8,(-31_i8)];
match RET.fld1.1 {
0 => bb26,
1 => bb27,
2 => bb28,
214156285722563668545909579781029114408 => bb30,
_ => bb29
}
}
bb38 = {
Return()
}
bb39 = {
_3 = '\u{ad2c7}';
_14 = 6_i8 as f32;
_9.1.0 = &RET.fld2;
_11 = _13 ^ _13;
RET.fld2 = RET.fld1.1 as u8;
RET.fld2 = 9_u8;
_22.3 = RET.fld3 as u16;
_19 = [RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2,RET.fld2];
_5 = 105343899241111691631506021973436199211_i128 << RET.fld1.1;
_23 = _22.3 & _22.3;
_20 = _23 < _23;
_12 = !RET.fld3;
_22.2 = &_22.0;
_7.3 = &_21;
_18 = _3;
RET.fld2 = !9_u8;
_22.1 = _2;
match RET.fld3 {
0 => bb8,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
2809630554 => bb20,
_ => bb19
}
}
bb40 = {
_17 = _5 * _5;
_17 = !_5;
_40 = Move(_22);
Goto(bb41)
}
bb41 = {
_34 = [6_usize,11347842180139769747_usize,6837479203973509326_usize,4803803329117156520_usize,5442378006017679942_usize];
_7.1 = core::ptr::addr_of!(_40.0);
_22.0 = RET.fld2 ^ _40.0;
RET.fld1.1 = _38;
_22.2 = &_40.0;
_7.1 = core::ptr::addr_of!(_22.0);
RET.fld1.1 = _38;
_40.3 = RET.fld3 as u16;
_38 = !RET.fld1.1;
_4 = (-9223372036854775808_isize);
_28 = Move(_7.2);
RET.fld1 = (Field::<i16>(Variant(_31, 0), 0), _38);
_9.1 = (Move(_22.2),);
Goto(bb42)
}
bb42 = {
_12 = _17 as u32;
_50.fld0 = RET.fld0;
_39 = core::ptr::addr_of!(_46);
_22.2 = &RET.fld2;
_14 = _44 * _44;
_34 = [10516279431010538138_usize,4_usize,3_usize,14213514739712945191_usize,2_usize];
_51 = _35;
_50.fld3 = !_12;
_22.1 = [17_i8,26_i8,(-53_i8)];
_8 = (-1086563537_i32);
_50.fld1.1 = !RET.fld1.1;
place!(Field::<f32>(Variant(_31, 0), 2)) = _14 + _14;
_52.1 = 3_usize as u8;
RET.fld1 = (Field::<i16>(Variant(_31, 0), 0), _38);
_50.fld1 = RET.fld1;
place!(Field::<f32>(Variant(_31, 0), 2)) = _44 + _14;
_44 = -Field::<f32>(Variant(_31, 0), 2);
_24 = &_8;
_40.1 = [(-109_i8),119_i8,(-30_i8)];
RET.fld4 = core::ptr::addr_of_mut!(_52.0);
_52.0.2 = [(*_24)];
RET.fld1.1 = _38;
_40.2 = Move(_22.2);
RET.fld4 = core::ptr::addr_of_mut!(_52.0);
_18 = _41.1;
Goto(bb43)
}
bb43 = {
Call(_58 = dump_var(17_usize, 12_usize, Move(_12), 30_usize, Move(_30), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_58 = dump_var(17_usize, 3_usize, Move(_3), 4_usize, Move(_4), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_58 = dump_var(17_usize, 8_usize, Move(_8), 18_usize, Move(_18), 5_usize, Move(_5), 59_usize, _59), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: &'static i32,mut _2: ([i32; 1], [u8; 8]),mut _3: [u8; 8],mut _4: [i8; 3],mut _5: u32) -> u8 {
mir! {
type RET = u8;
let _6: *mut i128;
let _7: f32;
let _8: *mut ([u8; 8], i128, [i32; 1], bool);
let _9: *mut i128;
let _10: ([i32; 1], char);
let _11: *const [char; 7];
let _12: &'static i16;
let _13: u128;
let _14: Adt52;
let _15: *const bool;
let _16: u16;
let _17: &'static f64;
let _18: usize;
let _19: bool;
let _20: i32;
let _21: *mut &'static Adt17;
let _22: ();
let _23: ();
{
_2.0 = [425780245_i32];
RET = !130_u8;
_2.0 = [(-2031041180_i32)];
RET = 168_u8;
_3 = _2.1;
RET = 135_u8 & 228_u8;
_4 = [(-3_i8),(-56_i8),(-62_i8)];
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [31_i8,(-83_i8),45_i8];
_4 = [(-29_i8),(-11_i8),118_i8];
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [(-13_i8),(-22_i8),(-112_i8)];
_7 = 9223372036854775807_isize as f32;
_2.0 = [(-1754666231_i32)];
_7 = RET as f32;
_10.1 = '\u{24125}';
_2.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = _2.1;
_2.0 = [(-650781943_i32)];
_10.0 = [(-1776448923_i32)];
Goto(bb1)
}
bb1 = {
_2 = (_10.0, _3);
_2 = (_10.0, _3);
_10.1 = '\u{bd4bf}';
_5 = 3740073440_u32 | 1625983810_u32;
RET = 16_u8 & 26_u8;
_2.0 = _10.0;
Goto(bb2)
}
bb2 = {
_5 = !2565892751_u32;
_10.1 = '\u{654ea}';
_10.1 = '\u{597fe}';
RET = !53_u8;
Call(RET = fn19(_2.0, _2.0, _10.0, _7, _4, _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = (_2.0, '\u{36f23}');
_7 = _5 as f32;
RET = 8737768112214723111_u64 as u8;
_7 = 10843311959345882933157380839629418735_u128 as f32;
_10.1 = '\u{4ce19}';
_7 = 1226823572566791015_i64 as f32;
_7 = (-39_i8) as f32;
_5 = 1254659785_u32;
_10 = (_2.0, '\u{c309c}');
RET = true as u8;
_14.fld1.0.0 = -690386684_i32;
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1254659785 => bb9,
_ => bb8
}
}
bb4 = {
_5 = !2565892751_u32;
_10.1 = '\u{654ea}';
_10.1 = '\u{597fe}';
RET = !53_u8;
Call(RET = fn19(_2.0, _2.0, _10.0, _7, _4, _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2 = (_10.0, _3);
_2 = (_10.0, _3);
_10.1 = '\u{bd4bf}';
_5 = 3740073440_u32 | 1625983810_u32;
RET = 16_u8 & 26_u8;
_2.0 = _10.0;
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
_10.1 = '\u{d43be}';
_3 = _2.1;
_14.fld1.0.0 = -2100276021_i32;
_7 = 61248_u16 as f32;
_14.fld0 = (_2.0, _10.1);
_14.fld0.0 = _2.0;
RET = _5 as u8;
_14.fld1.0 = ((-811419835_i32),);
_10.0 = [_14.fld1.0.0];
_10 = (_14.fld0.0, _14.fld0.1);
match _14.fld1.0.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb10,
340282366920938463463374607430956791621 => bb12,
_ => bb11
}
}
bb10 = {
_5 = !2565892751_u32;
_10.1 = '\u{654ea}';
_10.1 = '\u{597fe}';
RET = !53_u8;
Call(RET = fn19(_2.0, _2.0, _10.0, _7, _4, _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_10 = (_2.0, '\u{36f23}');
_7 = _5 as f32;
RET = 8737768112214723111_u64 as u8;
_7 = 10843311959345882933157380839629418735_u128 as f32;
_10.1 = '\u{4ce19}';
_7 = 1226823572566791015_i64 as f32;
_7 = (-39_i8) as f32;
_5 = 1254659785_u32;
_10 = (_2.0, '\u{c309c}');
RET = true as u8;
_14.fld1.0.0 = -690386684_i32;
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1254659785 => bb9,
_ => bb8
}
}
bb12 = {
_10 = (_14.fld0.0, _14.fld0.1);
RET = 222_u8 << _14.fld1.0.0;
_16 = 23663_u16 + 30907_u16;
_1 = &_14.fld1.0.0;
RET = !133_u8;
_14.fld0.1 = _10.1;
_10 = _14.fld0;
_2.1 = _3;
RET = 137_u8;
_14.fld0 = _10;
_18 = !392389162216875308_usize;
RET = !10_u8;
_10.0 = [(*_1)];
RET = (-3896343430745435719_i64) as u8;
_14.fld0.1 = _10.1;
_4 = [(-79_i8),79_i8,32_i8];
RET = 116_u8 - 186_u8;
_18 = !0_usize;
_14.fld0 = _10;
_1 = &(*_1);
_5 = 2953809320_u32 - 3535097213_u32;
_18 = !6_usize;
_4 = [(-30_i8),(-83_i8),76_i8];
_2.0 = [(*_1)];
Goto(bb13)
}
bb13 = {
_13 = !263737633805295819031157974340414153716_u128;
match (*_1) {
0 => bb9,
1 => bb12,
2 => bb8,
3 => bb10,
4 => bb14,
5 => bb15,
340282366920938463463374607430956791621 => bb17,
_ => bb16
}
}
bb14 = {
_10 = (_14.fld0.0, _14.fld0.1);
RET = 222_u8 << _14.fld1.0.0;
_16 = 23663_u16 + 30907_u16;
_1 = &_14.fld1.0.0;
RET = !133_u8;
_14.fld0.1 = _10.1;
_10 = _14.fld0;
_2.1 = _3;
RET = 137_u8;
_14.fld0 = _10;
_18 = !392389162216875308_usize;
RET = !10_u8;
_10.0 = [(*_1)];
RET = (-3896343430745435719_i64) as u8;
_14.fld0.1 = _10.1;
_4 = [(-79_i8),79_i8,32_i8];
RET = 116_u8 - 186_u8;
_18 = !0_usize;
_14.fld0 = _10;
_1 = &(*_1);
_5 = 2953809320_u32 - 3535097213_u32;
_18 = !6_usize;
_4 = [(-30_i8),(-83_i8),76_i8];
_2.0 = [(*_1)];
Goto(bb13)
}
bb15 = {
_5 = !2565892751_u32;
_10.1 = '\u{654ea}';
_10.1 = '\u{597fe}';
RET = !53_u8;
Call(RET = fn19(_2.0, _2.0, _10.0, _7, _4, _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_10 = (_2.0, '\u{36f23}');
_7 = _5 as f32;
RET = 8737768112214723111_u64 as u8;
_7 = 10843311959345882933157380839629418735_u128 as f32;
_10.1 = '\u{4ce19}';
_7 = 1226823572566791015_i64 as f32;
_7 = (-39_i8) as f32;
_5 = 1254659785_u32;
_10 = (_2.0, '\u{c309c}');
RET = true as u8;
_14.fld1.0.0 = -690386684_i32;
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1254659785 => bb9,
_ => bb8
}
}
bb17 = {
_14.fld0.0 = [(*_1)];
_14.fld1.0 = ((-81619503_i32),);
_10.0 = [_14.fld1.0.0];
_19 = true | false;
RET = 43_i8 as u8;
_20 = -_14.fld1.0.0;
_19 = false ^ false;
RET = !226_u8;
_14.fld0.1 = _10.1;
_10.1 = _14.fld0.1;
_4 = [14_i8,(-74_i8),(-64_i8)];
_2 = (_14.fld0.0, _3);
Goto(bb18)
}
bb18 = {
Call(_22 = dump_var(18_usize, 3_usize, Move(_3), 10_usize, Move(_10), 20_usize, Move(_20), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_22 = dump_var(18_usize, 2_usize, Move(_2), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: [i32; 1],mut _2: [i32; 1],mut _3: [i32; 1],mut _4: f32,mut _5: [i8; 3],mut _6: [i32; 1]) -> u8 {
mir! {
type RET = u8;
let _7: [usize; 7];
let _8: Adt82;
let _9: Adt17;
let _10: isize;
let _11: f64;
let _12: *const [char; 4];
let _13: [i128; 6];
let _14: bool;
let _15: bool;
let _16: bool;
let _17: &'static i32;
let _18: Adt40;
let _19: ([i32; 1], [u8; 8]);
let _20: &'static f64;
let _21: isize;
let _22: (([i32; 1], [u8; 8]), *const u8, &'static i32, &'static [u128; 7]);
let _23: f32;
let _24: *mut [i32; 7];
let _25: ();
let _26: ();
{
_6 = [(-1252533812_i32)];
RET = 1906324886667936549_u64 as u8;
_4 = 54565_u16 as f32;
_6 = [851820614_i32];
_6 = [803032034_i32];
RET = !114_u8;
_4 = 3875198931896529515_u64 as f32;
_1 = [(-1476959192_i32)];
_3 = _6;
_4 = 2854516152_u32 as f32;
_6 = [(-896184580_i32)];
_1 = [(-318169273_i32)];
RET = 119_u8 >> 2462817448_u32;
_5 = [87_i8,(-40_i8),58_i8];
_6 = _2;
_2 = _1;
_4 = 157779642372650053069835729067179097418_i128 as f32;
_2 = [(-365269122_i32)];
_7 = [7_usize,5760632870327736237_usize,18125978782749299593_usize,0_usize,2_usize,17987944243179765737_usize,10514483491535640414_usize];
RET = 203_u8;
_2 = _1;
_4 = (-10181_i16) as f32;
Goto(bb1)
}
bb1 = {
_2 = _6;
_9 = Adt17::Variant2 { fld0: 62128_u16,fld1: '\u{b344d}',fld2: RET };
place!(Field::<char>(Variant(_9, 2), 1)) = '\u{8d68f}';
_7 = [16634914975691972515_usize,15910138570693822_usize,10599849680232187561_usize,2991788321308847480_usize,1_usize,5349433988474316964_usize,15780876338253711017_usize];
place!(Field::<u8>(Variant(_9, 2), 2)) = RET / RET;
_1 = [(-974574332_i32)];
place!(Field::<u16>(Variant(_9, 2), 0)) = 60529_u16;
_5 = [95_i8,(-56_i8),(-106_i8)];
_9 = Adt17::Variant3 { fld0: true,fld1: 1731496207433606947_usize,fld2: 10774549745517291336_u64,fld3: 88_i8,fld4: (-19918_i16),fld5: (-264218847_i32),fld6: 2596631733254424652_i64,fld7: RET };
place!(Field::<i64>(Variant(_9, 3), 6)) = 3759508999_u32 as i64;
RET = !Field::<u8>(Variant(_9, 3), 7);
Goto(bb2)
}
bb2 = {
place!(Field::<bool>(Variant(_9, 3), 0)) = !true;
place!(Field::<i32>(Variant(_9, 3), 5)) = !634433691_i32;
match Field::<u8>(Variant(_9, 3), 7) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
203 => bb11,
_ => bb10
}
}
bb3 = {
_2 = _6;
_9 = Adt17::Variant2 { fld0: 62128_u16,fld1: '\u{b344d}',fld2: RET };
place!(Field::<char>(Variant(_9, 2), 1)) = '\u{8d68f}';
_7 = [16634914975691972515_usize,15910138570693822_usize,10599849680232187561_usize,2991788321308847480_usize,1_usize,5349433988474316964_usize,15780876338253711017_usize];
place!(Field::<u8>(Variant(_9, 2), 2)) = RET / RET;
_1 = [(-974574332_i32)];
place!(Field::<u16>(Variant(_9, 2), 0)) = 60529_u16;
_5 = [95_i8,(-56_i8),(-106_i8)];
_9 = Adt17::Variant3 { fld0: true,fld1: 1731496207433606947_usize,fld2: 10774549745517291336_u64,fld3: 88_i8,fld4: (-19918_i16),fld5: (-264218847_i32),fld6: 2596631733254424652_i64,fld7: RET };
place!(Field::<i64>(Variant(_9, 3), 6)) = 3759508999_u32 as i64;
RET = !Field::<u8>(Variant(_9, 3), 7);
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
Return()
}
bb11 = {
place!(Field::<i8>(Variant(_9, 3), 3)) = (-80_i8);
_13 = [45284420574238656512076178631892046170_i128,(-103732776828192035283085629633551494937_i128),71342274587006519913467252310517023029_i128,(-31033679177757074148622319337190717117_i128),(-70284361320009747182417061693653045158_i128),(-39608604448948538712856211698603591979_i128)];
_1 = _6;
place!(Field::<i8>(Variant(_9, 3), 3)) = 66087624840201675052231117014056947099_u128 as i8;
place!(Field::<i32>(Variant(_9, 3), 5)) = Field::<i8>(Variant(_9, 3), 3) as i32;
place!(Field::<usize>(Variant(_9, 3), 1)) = 3_usize & 3_usize;
_10 = Field::<i32>(Variant(_9, 3), 5) as isize;
_16 = Field::<bool>(Variant(_9, 3), 0) < Field::<bool>(Variant(_9, 3), 0);
_16 = _4 < _4;
_3 = [Field::<i32>(Variant(_9, 3), 5)];
_1 = [Field::<i32>(Variant(_9, 3), 5)];
_15 = !_16;
place!(Field::<bool>(Variant(_9, 3), 0)) = _15;
place!(Field::<u64>(Variant(_9, 3), 2)) = 7103_i16 as u64;
_11 = 244540863492955202050454266497461823250_u128 as f64;
_17 = &place!(Field::<i32>(Variant(_9, 3), 5));
_15 = Field::<bool>(Variant(_9, 3), 0);
place!(Field::<i32>(Variant(_9, 3), 5)) = (-10710_i16) as i32;
_17 = &place!(Field::<i32>(Variant(_9, 3), 5));
_16 = !_15;
Goto(bb12)
}
bb12 = {
_16 = _15 ^ _15;
place!(Field::<i32>(Variant(_9, 3), 5)) = (-1888014690_i32) << Field::<i64>(Variant(_9, 3), 6);
_1 = [Field::<i32>(Variant(_9, 3), 5)];
place!(Field::<u8>(Variant(_9, 3), 7)) = 40811_u16 as u8;
_2 = [Field::<i32>(Variant(_9, 3), 5)];
_4 = Field::<i8>(Variant(_9, 3), 3) as f32;
place!(Field::<i64>(Variant(_9, 3), 6)) = -2155004912809274204_i64;
_2 = _1;
place!(Field::<i16>(Variant(_9, 3), 4)) = !30404_i16;
_5 = [Field::<i8>(Variant(_9, 3), 3),Field::<i8>(Variant(_9, 3), 3),Field::<i8>(Variant(_9, 3), 3)];
_4 = Field::<i8>(Variant(_9, 3), 3) as f32;
_6 = [Field::<i32>(Variant(_9, 3), 5)];
place!(Field::<bool>(Variant(_9, 3), 0)) = _15;
_18 = Adt40::Variant0 { fld0: Field::<i16>(Variant(_9, 3), 4),fld1: '\u{86786}',fld2: _4,fld3: 46946_u16 };
place!(Field::<usize>(Variant(_9, 3), 1)) = !6_usize;
_9 = Adt17::Variant3 { fld0: _15,fld1: 7_usize,fld2: 15366117388808182318_u64,fld3: 96_i8,fld4: Field::<i16>(Variant(_18, 0), 0),fld5: 886335917_i32,fld6: 5990414762205026356_i64,fld7: RET };
_9 = Adt17::Variant0 { fld0: (-9178144932836543975_i64),fld1: 29974_u16,fld2: _10,fld3: 274506566944766647747890557208122150985_u128,fld4: 101535697873973987243617989664585284935_i128 };
place!(Field::<u128>(Variant(_9, 0), 3)) = 225686365906881967973041844892339865002_u128 >> Field::<isize>(Variant(_9, 0), 2);
_9 = Adt17::Variant1 { fld0: 2968907045_u32,fld1: '\u{8424c}',fld2: _10,fld3: 0_usize,fld4: 14678599833311819566_u64,fld5: 38745_u16,fld6: Field::<f32>(Variant(_18, 0), 2) };
_19.0 = _2;
place!(Field::<usize>(Variant(_9, 1), 3)) = 15063795129941583938_usize >> Field::<isize>(Variant(_9, 1), 2);
place!(Field::<u16>(Variant(_18, 0), 3)) = 5407_u16 * 48478_u16;
place!(Field::<u16>(Variant(_9, 1), 5)) = !Field::<u16>(Variant(_18, 0), 3);
_1 = [(-1473388745_i32)];
Goto(bb13)
}
bb13 = {
place!(Field::<usize>(Variant(_9, 1), 3)) = !16084251891416804055_usize;
_10 = Field::<isize>(Variant(_9, 1), 2);
Goto(bb14)
}
bb14 = {
_18 = Adt40::Variant1 { fld0: 4221873244_u32,fld1: 20910040156871057233357236170988544559_u128,fld2: _2,fld3: Field::<usize>(Variant(_9, 1), 3),fld4: 16866_i16,fld5: 13074270388356296906_u64,fld6: (-5473360412851116423_i64),fld7: (-45509860063894550598572954126635267361_i128) };
place!(Field::<u16>(Variant(_9, 1), 5)) = 20615_u16;
_18 = Adt40::Variant1 { fld0: 2476981069_u32,fld1: 134983162953962800472434974419360990172_u128,fld2: _2,fld3: Field::<usize>(Variant(_9, 1), 3),fld4: 24794_i16,fld5: 17122875693087510719_u64,fld6: (-2389265853766493522_i64),fld7: (-117962402734455209601378114179070823674_i128) };
_9 = Adt17::Variant1 { fld0: 3989488567_u32,fld1: '\u{31fe3}',fld2: _10,fld3: Field::<usize>(Variant(_18, 1), 3),fld4: 16013812459823617253_u64,fld5: 13609_u16,fld6: _4 };
_20 = &_11;
_21 = 117_i8 as isize;
_22.0.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
place!(Field::<u32>(Variant(_18, 1), 0)) = 2384709979_u32;
_22.1 = core::ptr::addr_of!(RET);
_22.0.0 = Field::<[i32; 1]>(Variant(_18, 1), 2);
_9 = Adt17::Variant1 { fld0: Field::<u32>(Variant(_18, 1), 0),fld1: '\u{d26d0}',fld2: _10,fld3: Field::<usize>(Variant(_18, 1), 3),fld4: 3812174122188093874_u64,fld5: 52742_u16,fld6: _4 };
_9 = Adt17::Variant0 { fld0: 2401443237823546505_i64,fld1: 33668_u16,fld2: _21,fld3: 231834298269236030710307971309434515553_u128,fld4: 140605353092044758981744694222918455485_i128 };
place!(Field::<u64>(Variant(_18, 1), 5)) = !11546064009232435728_u64;
_19 = _22.0;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(19_usize, 15_usize, Move(_15), 13_usize, Move(_13), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(19_usize, 16_usize, Move(_16), 3_usize, Move(_3), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{2b0eb}'), std::hint::black_box(6340367088546863763_u64), std::hint::black_box(1548214970_u32), std::hint::black_box(175_u8), std::hint::black_box(1_usize), std::hint::black_box(245356817663179397536666513418876657909_u128), std::hint::black_box(13621_u16));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){unsafe{printf("Adt17::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt17 {
Variant0{
fld0: i64,
fld1: u16,
fld2: isize,
fld3: u128,
fld4: i128,

},
Variant1{
fld0: u32,
fld1: char,
fld2: isize,
fld3: usize,
fld4: u64,
fld5: u16,
fld6: f32,

},
Variant2{
fld0: u16,
fld1: char,
fld2: u8,

},
Variant3{
fld0: bool,
fld1: usize,
fld2: u64,
fld3: i8,
fld4: i16,
fld5: i32,
fld6: i64,
fld7: u8,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){unsafe{printf("Adt33::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt33 {
Variant0{
fld0: u16,
fld1: Adt17,
fld2: [char; 7],
fld3: u128,

},
Variant1{
fld0: Adt17,
fld1: f64,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: ((i32,),),
fld1: f32,
fld2: [char; 7],

},
Variant1{
fld0: (i32,),
fld1: char,
fld2: [i8; 3],
fld3: f32,
fld4: ((i32,),),
fld5: u128,
fld6: u8,
fld7: [u8; 8],

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: i16,
fld1: char,
fld2: f32,
fld3: u16,

},
Variant1{
fld0: u32,
fld1: u128,
fld2: [i32; 1],
fld3: usize,
fld4: i16,
fld5: u64,
fld6: i64,
fld7: i128,

},
Variant2{
fld0: bool,
fld1: (i32,),
fld2: ((i32,),),
fld3: u8,
fld4: u128,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: ([i32; 1], char),
fld1: ((i32,),),
}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt72{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt72 {
fld0: [char; 7],
fld1: (i16, u128),
fld2: u8,
fld3: u32,
fld4: *mut ([u8; 8], i128, [i32; 1], bool),
}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt76{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt76 {
fld0: u16,
}
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
fld0: *mut ([u8; 8], i128, [i32; 1], bool),
fld1: [u128; 7],
fld2: [i128; 6],
fld3: ([char; 7], [i8; 3], i32),

},
Variant1{
fld0: *const [char; 4],
fld1: ([u8; 8], i128, [i32; 1], bool),
fld2: f64,
fld3: *mut [u8; 8],
fld4: [char; 4],
fld5: ([i8; 3], u32, ([u8; 8], i128, [i32; 1], bool), *const [char; 7]),

},
Variant2{
fld0: [i32; 1],
fld1: usize,
fld2: ([i32; 1], char),
fld3: i8,
fld4: (([u8; 8], i128, [i32; 1], bool), u8),
fld5: *const Adt17,

},
Variant3{
fld0: *mut [char; 7],
fld1: *mut ([u8; 8], i128, [i32; 1], bool),
fld2: [char; 7],
fld3: (*mut i128, u64, Adt33),
fld4: u128,
fld5: i32,

}}

