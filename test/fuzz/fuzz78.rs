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
pub fn fn0(mut _1: u16,mut _2: char,mut _3: u128,mut _4: i8,mut _5: u32,mut _6: u64,mut _7: i64,mut _8: i128,mut _9: usize) -> i8 {
mir! {
type RET = i8;
let _10: bool;
let _11: bool;
let _12: [u128; 2];
let _13: [u128; 2];
let _14: *mut f32;
let _15: [bool; 2];
let _16: (char, &'static i64);
let _17: [i8; 8];
let _18: u16;
let _19: isize;
let _20: u128;
let _21: isize;
let _22: isize;
let _23: i8;
let _24: ();
let _25: ();
{
RET = false as i8;
_3 = !315168140763499724328645881223739927218_u128;
_2 = '\u{b2160}';
Goto(bb1)
}
bb1 = {
_8 = -2538225423366985457324821709487750981_i128;
_1 = false as u16;
_5 = 639636938_u32 - 1292008275_u32;
_4 = -RET;
_12 = [_3,_3];
_9 = !5_usize;
_13 = _12;
_9 = !2_usize;
_4 = 529888993_i32 as i8;
_3 = 192316557780426201410668798448237012034_u128;
_2 = '\u{a878e}';
_10 = !false;
RET = _4;
_10 = !true;
_7 = (-3647207306276003393_i64);
_11 = _4 > RET;
RET = _4;
_7 = 610992518989411450_i64;
_3 = 334633788493376944432188623428437337718_u128 >> _5;
_6 = 7666430413585105533_u64 & 959910130944434397_u64;
RET = _4;
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
610992518989411450 => bb8,
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
_5 = 2782781433_u32 << _7;
_7 = _5 as i64;
_11 = _10;
_7 = (-5895537137849139394_i64) & 2473014873842849978_i64;
Call(_7 = core::intrinsics::transmute(_6), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = _6 != _6;
_5 = 917281400_u32 + 1000911197_u32;
_11 = !_10;
_6 = _3 as u64;
_5 = 307758189_u32;
_9 = 2_usize;
_8 = _10 as i128;
_9 = 7_usize;
_15 = [_10,_10];
_5 = !642899006_u32;
RET = -_4;
RET = 223_u8 as i8;
_10 = !_11;
_9 = _2 as usize;
_3 = _2 as u128;
RET = -_4;
Call(_15 = fn1(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8 = _4 as i128;
Call(_13 = fn16(_11, _2, _2, _1, _11, _10, _15, RET, _4, _6, _10, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = !_4;
_11 = !_10;
_8 = (-34655018690936612518056557828824328489_i128);
RET = _6 as i8;
_16.0 = _2;
_4 = RET;
_12 = _13;
RET = -_4;
_13 = _12;
_2 = _16.0;
_8 = _5 as i128;
_16.1 = &_7;
_12 = _13;
_5 = 4061959229_u32 ^ 2750802304_u32;
_13 = _12;
_4 = RET;
_11 = _10;
_8 = 21_isize as i128;
_8 = 80300107960537449374058611865917232352_i128 | (-45436219865481878473538306798631462269_i128);
_7 = -685064480518365076_i64;
Goto(bb12)
}
bb12 = {
_2 = _16.0;
_18 = _1;
_17 = [_4,_4,RET,_4,RET,RET,_4,_4];
_16.0 = _2;
RET = !_4;
_4 = -RET;
Goto(bb13)
}
bb13 = {
_18 = _4 as u16;
Call(_4 = core::intrinsics::bswap(RET), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_13 = [_3,_3];
_6 = 7310426757836624979_u64;
_16.0 = _2;
_3 = 9223372036854775807_isize as u128;
_13 = _12;
_17 = [_4,_4,_4,RET,RET,RET,RET,_4];
_10 = _11;
_21 = 9223372036854775807_isize + 9223372036854775807_isize;
_6 = !15583252142671669986_u64;
_20 = _18 as u128;
_12 = [_3,_20];
_22 = _21 + _21;
_16.1 = &_7;
_16.1 = &_7;
_8 = _21 as i128;
_23 = 16092_i16 as i8;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(0_usize, 20_usize, Move(_20), 6_usize, Move(_6), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(0_usize, 2_usize, Move(_2), 21_usize, Move(_21), 9_usize, Move(_9), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(0_usize, 23_usize, Move(_23), 4_usize, Move(_4), 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8) -> [bool; 2] {
mir! {
type RET = [bool; 2];
let _2: u128;
let _3: u64;
let _4: f64;
let _5: f64;
let _6: isize;
let _7: *const *const char;
let _8: &'static *mut [i64; 3];
let _9: (i32,);
let _10: f32;
let _11: [i32; 3];
let _12: [u16; 7];
let _13: Adt38;
let _14: [i8; 8];
let _15: i128;
let _16: char;
let _17: u128;
let _18: char;
let _19: bool;
let _20: &'static [i32; 3];
let _21: u8;
let _22: *mut u128;
let _23: i8;
let _24: bool;
let _25: Adt51;
let _26: isize;
let _27: bool;
let _28: &'static *mut Adt51;
let _29: ();
let _30: ();
{
RET = [false,true];
RET = [false,false];
_1 = 95_u8 as i8;
RET = [false,false];
RET = [true,true];
Goto(bb1)
}
bb1 = {
_2 = 233003642789824155466362004874250253548_u128;
_2 = !37138974913613404907253806846232390530_u128;
_1 = !(-95_i8);
_3 = !3670791087230911744_u64;
RET = [true,false];
_1 = -104_i8;
RET = [false,true];
_1 = 40391784720861646195218195725205587426_i128 as i8;
RET = [false,false];
_1 = (-81_i8);
RET = [true,false];
RET = [true,false];
_3 = !16909895699279907343_u64;
_2 = 158965229048746385206384825406443102145_u128;
RET = [true,false];
_1 = -53_i8;
RET = [false,true];
_1 = 122_i8 ^ (-69_i8);
_2 = 60813226129980320948870615356925234884_u128;
_1 = 57_isize as i8;
_1 = (-5_i8) ^ (-21_i8);
_3 = 13963378570057792389_u64 | 8354151214026299707_u64;
_4 = 12_u8 as f64;
_1 = 204_u8 as i8;
RET = [false,true];
_1 = 123_u8 as i8;
Call(_4 = fn2(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [true,true];
_3 = !413445409584877918_u64;
RET = [true,false];
RET = [false,true];
_1 = 46_i8;
RET = [true,true];
_1 = 68_i8 - (-65_i8);
_2 = _4 as u128;
RET = [true,true];
_2 = 1731278291_u32 as u128;
_5 = _2 as f64;
_1 = 9922837173488048090_usize as i8;
_2 = 193753463945233677951873508601497045497_u128;
RET = [true,true];
RET = [false,false];
_6 = (-9223372036854775808_isize) >> _1;
_3 = 10806229418231492603_u64;
_6 = 137_u8 as isize;
_9.0 = !(-292310596_i32);
_1 = 7889_i16 as i8;
_4 = 61657_u16 as f64;
RET = [false,true];
_5 = -_4;
Call(_6 = fn3(_1, _9, _2, _2, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [true,true];
_4 = _5 + _5;
_1 = -120_i8;
_9.0 = 874483586_i32 - 809022244_i32;
_5 = 41238_u16 as f64;
RET = [true,false];
_10 = 39147784204595911465820571154159265459_i128 as f32;
RET = [true,false];
_3 = 8391902419940579661_u64 * 15703506536439514593_u64;
_9.0 = !(-1227792200_i32);
_9.0 = (-697557050_i32) - (-1128105120_i32);
RET = [false,true];
_2 = _1 as u128;
_9.0 = -(-70562930_i32);
_3 = _6 as u64;
_11 = [_9.0,_9.0,_9.0];
_11 = [_9.0,_9.0,_9.0];
_12 = [43939_u16,9554_u16,58048_u16,43576_u16,60108_u16,44970_u16,30229_u16];
Goto(bb4)
}
bb4 = {
_1 = 61_i8;
_13.fld0 = core::ptr::addr_of_mut!(_10);
_12 = [37739_u16,43297_u16,59816_u16,51684_u16,38008_u16,3002_u16,44796_u16];
_9 = ((-703780632_i32),);
_13.fld1.0 = 28825_i16 as i8;
_13.fld1.0 = _1;
_9.0 = true as i32;
_5 = _4;
RET = [false,true];
_15 = _3 as i128;
RET = [false,false];
Goto(bb5)
}
bb5 = {
_5 = _4 + _4;
_12 = [59707_u16,24111_u16,41467_u16,57119_u16,48669_u16,7758_u16,42923_u16];
_13.fld0 = core::ptr::addr_of_mut!(_10);
_6 = 2382387304_u32 as isize;
_16 = '\u{4c17a}';
_4 = _9.0 as f64;
_5 = -_4;
_13.fld0 = core::ptr::addr_of_mut!(_10);
_12 = [31928_u16,37292_u16,16899_u16,21380_u16,17153_u16,1540_u16,3302_u16];
_9 = (1749262253_i32,);
_10 = 204_u8 as f32;
_14 = [_1,_13.fld1.0,_1,_13.fld1.0,_13.fld1.0,_13.fld1.0,_13.fld1.0,_13.fld1.0];
_15 = !(-76775938245675416252158730897543553274_i128);
_13.fld1 = (_1,);
_13.fld1.0 = _1;
match _13.fld1.0 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
61 => bb10,
_ => bb9
}
}
bb6 = {
_1 = 61_i8;
_13.fld0 = core::ptr::addr_of_mut!(_10);
_12 = [37739_u16,43297_u16,59816_u16,51684_u16,38008_u16,3002_u16,44796_u16];
_9 = ((-703780632_i32),);
_13.fld1.0 = 28825_i16 as i8;
_13.fld1.0 = _1;
_9.0 = true as i32;
_5 = _4;
RET = [false,true];
_15 = _3 as i128;
RET = [false,false];
Goto(bb5)
}
bb7 = {
RET = [true,true];
_4 = _5 + _5;
_1 = -120_i8;
_9.0 = 874483586_i32 - 809022244_i32;
_5 = 41238_u16 as f64;
RET = [true,false];
_10 = 39147784204595911465820571154159265459_i128 as f32;
RET = [true,false];
_3 = 8391902419940579661_u64 * 15703506536439514593_u64;
_9.0 = !(-1227792200_i32);
_9.0 = (-697557050_i32) - (-1128105120_i32);
RET = [false,true];
_2 = _1 as u128;
_9.0 = -(-70562930_i32);
_3 = _6 as u64;
_11 = [_9.0,_9.0,_9.0];
_11 = [_9.0,_9.0,_9.0];
_12 = [43939_u16,9554_u16,58048_u16,43576_u16,60108_u16,44970_u16,30229_u16];
Goto(bb4)
}
bb8 = {
RET = [true,true];
_3 = !413445409584877918_u64;
RET = [true,false];
RET = [false,true];
_1 = 46_i8;
RET = [true,true];
_1 = 68_i8 - (-65_i8);
_2 = _4 as u128;
RET = [true,true];
_2 = 1731278291_u32 as u128;
_5 = _2 as f64;
_1 = 9922837173488048090_usize as i8;
_2 = 193753463945233677951873508601497045497_u128;
RET = [true,true];
RET = [false,false];
_6 = (-9223372036854775808_isize) >> _1;
_3 = 10806229418231492603_u64;
_6 = 137_u8 as isize;
_9.0 = !(-292310596_i32);
_1 = 7889_i16 as i8;
_4 = 61657_u16 as f64;
RET = [false,true];
_5 = -_4;
Call(_6 = fn3(_1, _9, _2, _2, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_2 = 233003642789824155466362004874250253548_u128;
_2 = !37138974913613404907253806846232390530_u128;
_1 = !(-95_i8);
_3 = !3670791087230911744_u64;
RET = [true,false];
_1 = -104_i8;
RET = [false,true];
_1 = 40391784720861646195218195725205587426_i128 as i8;
RET = [false,false];
_1 = (-81_i8);
RET = [true,false];
RET = [true,false];
_3 = !16909895699279907343_u64;
_2 = 158965229048746385206384825406443102145_u128;
RET = [true,false];
_1 = -53_i8;
RET = [false,true];
_1 = 122_i8 ^ (-69_i8);
_2 = 60813226129980320948870615356925234884_u128;
_1 = 57_isize as i8;
_1 = (-5_i8) ^ (-21_i8);
_3 = 13963378570057792389_u64 | 8354151214026299707_u64;
_4 = 12_u8 as f64;
_1 = 204_u8 as i8;
RET = [false,true];
_1 = 123_u8 as i8;
Call(_4 = fn2(), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
RET = [false,true];
_1 = _13.fld1.0;
Goto(bb11)
}
bb11 = {
_13.fld1 = (_1,);
_17 = !_2;
_13.fld0 = core::ptr::addr_of_mut!(_10);
_6 = (-9223372036854775808_isize);
_2 = _17;
_13.fld1 = (_1,);
_6 = -127_isize;
_12 = [30463_u16,55420_u16,19674_u16,476_u16,23776_u16,25183_u16,20480_u16];
_4 = _5;
_15 = 116635185243043184061177546906811509015_i128 + (-6483808724568965747943343541715625670_i128);
_3 = 7525139528350344223_u64;
_5 = -_4;
Call(_9.0 = fn12(_17, _5, Move(_13), _12, _10, _11, _6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_23 = 31849_i16 as i8;
_13.fld1 = (_1,);
_6 = 3427604592_u32 as isize;
_18 = _16;
match _13.fld1.0 {
0 => bb6,
1 => bb10,
2 => bb8,
3 => bb4,
4 => bb11,
61 => bb14,
_ => bb13
}
}
bb13 = {
_5 = _4 + _4;
_12 = [59707_u16,24111_u16,41467_u16,57119_u16,48669_u16,7758_u16,42923_u16];
_13.fld0 = core::ptr::addr_of_mut!(_10);
_6 = 2382387304_u32 as isize;
_16 = '\u{4c17a}';
_4 = _9.0 as f64;
_5 = -_4;
_13.fld0 = core::ptr::addr_of_mut!(_10);
_12 = [31928_u16,37292_u16,16899_u16,21380_u16,17153_u16,1540_u16,3302_u16];
_9 = (1749262253_i32,);
_10 = 204_u8 as f32;
_14 = [_1,_13.fld1.0,_1,_13.fld1.0,_13.fld1.0,_13.fld1.0,_13.fld1.0,_13.fld1.0];
_15 = !(-76775938245675416252158730897543553274_i128);
_13.fld1 = (_1,);
_13.fld1.0 = _1;
match _13.fld1.0 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
61 => bb10,
_ => bb9
}
}
bb14 = {
_25.fld2 = [(-6709473030647446301_i64)];
_5 = -_4;
_22 = core::ptr::addr_of_mut!(_17);
_19 = !false;
_15 = (-27989650617149079733276363706054246455_i128) * 85789959140103813051640620940250561557_i128;
_2 = (*_22) & (*_22);
_12 = [58248_u16,26220_u16,62877_u16,40775_u16,40522_u16,13051_u16,53133_u16];
(*_22) = _2 - _2;
_25.fld1 = core::ptr::addr_of!(_3);
_25.fld4 = 4467140789280720319_i64;
(*_22) = 192_u8 as u128;
_4 = -_5;
_20 = &_11;
_13.fld0 = core::ptr::addr_of_mut!(_25.fld3);
RET = [_19,_19];
_19 = true;
_16 = _18;
_13.fld1.0 = _23;
_9.0 = !658271102_i32;
_27 = !_19;
_24 = !_19;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(1_usize, 18_usize, Move(_18), 12_usize, Move(_12), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(1_usize, 16_usize, Move(_16), 3_usize, Move(_3), 24_usize, Move(_24), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2() -> f64 {
mir! {
type RET = f64;
let _1: [u16; 7];
let _2: [u128; 2];
let _3: u8;
let _4: (i64,);
let _5: *const u64;
let _6: ([i64; 1],);
let _7: [bool; 2];
let _8: char;
let _9: i64;
let _10: &'static [u8; 7];
let _11: f64;
let _12: ((i8,), Adt51, *mut u128, char);
let _13: i16;
let _14: Adt66;
let _15: f64;
let _16: (*mut Adt66,);
let _17: isize;
let _18: [i8; 8];
let _19: [i8; 8];
let _20: ();
let _21: ();
{
RET = (-9223372036854775808_isize) as f64;
RET = 1010733258163516214_u64 as f64;
RET = (-42_isize) as f64;
RET = 2322037416_u32 as f64;
RET = 324634790708091980883533419078444445501_u128 as f64;
RET = 3_usize as f64;
RET = 147620286144812154710468565109794009421_u128 as f64;
RET = 2645439585_u32 as f64;
RET = 174_u8 as f64;
_1 = [59748_u16,40267_u16,40107_u16,54048_u16,30689_u16,62501_u16,32149_u16];
RET = (-681392797_i32) as f64;
RET = (-2344_i16) as f64;
RET = 42_i8 as f64;
Goto(bb1)
}
bb1 = {
RET = 11406722209474065078_usize as f64;
_1 = [20188_u16,4369_u16,8087_u16,7571_u16,2223_u16,61293_u16,11658_u16];
RET = (-161401402889137845934588225643874709987_i128) as f64;
RET = 8600965706793845892_i64 as f64;
_1 = [64845_u16,48765_u16,63262_u16,9285_u16,668_u16,44041_u16,55406_u16];
RET = 514833711_u32 as f64;
RET = 35277419287413989507706180370214460816_i128 as f64;
_2 = [13840809175620679062704796416739142163_u128,228683388678373422867766315999796796213_u128];
_2 = [91841610163076469736655081967057422905_u128,212822361426815616526546664113330989505_u128];
RET = (-8451493230845459798_i64) as f64;
_2 = [108615748309026090759310884527954494593_u128,2256125164972903008366749078077766702_u128];
_2 = [185587933514622996832065780554978497436_u128,321606818094618952474144324873627592068_u128];
RET = 5537236603057862069415207110100422796_u128 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 4583681784142544641_u64 as f64;
_3 = 186_u8;
_2 = [246175521923690587885623993968533754558_u128,201805120293332565532418806722157820222_u128];
_1 = [6062_u16,15221_u16,5885_u16,31136_u16,46748_u16,39274_u16,6136_u16];
Goto(bb2)
}
bb2 = {
_3 = 131_u8;
_4 = ((-7844499867642086660_i64),);
RET = 3585865567_u32 as f64;
_3 = 47_u8;
_2 = [152815230573381127827328030451755883457_u128,197777476217507911862377729884160243802_u128];
RET = 295196606042499897809116864835535715711_u128 as f64;
_7 = [true,true];
Goto(bb3)
}
bb3 = {
_4.0 = -(-4811103507531041590_i64);
_7 = [false,false];
RET = 88_i8 as f64;
Goto(bb4)
}
bb4 = {
_4 = (6993262293007023294_i64,);
_7 = [false,false];
_4 = ((-4810230167560013508_i64),);
_8 = '\u{915e1}';
_6.0 = [_4.0];
_7 = [true,false];
_11 = RET;
_11 = -RET;
_12.1.fld5 = 31264_u16 as i32;
_12.0 = ((-90_i8),);
_12.1.fld4 = _3 as i64;
_12.1.fld4 = 22045_i16 as i64;
_12.0 = ((-14_i8),);
_12.3 = _8;
_13 = 10345_i16;
_13 = 9532825057645735086_u64 as i16;
_9 = _12.1.fld4;
_12.0.0 = !(-65_i8);
_12.1.fld5 = 1028696523_i32;
_12.1.fld2 = _6.0;
_12.1.fld3 = RET as f32;
_2 = [78848604666051690582756231531817177525_u128,225970887323972096278976486773550242908_u128];
_12.0 = (51_i8,);
_12.0.0 = -3_i8;
Goto(bb5)
}
bb5 = {
_4.0 = _12.1.fld4;
_11 = RET;
_12.0.0 = 89_i8 >> _3;
_16.0 = core::ptr::addr_of_mut!(_14);
match _12.1.fld5 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
1028696523 => bb11,
_ => bb10
}
}
bb6 = {
_4 = (6993262293007023294_i64,);
_7 = [false,false];
_4 = ((-4810230167560013508_i64),);
_8 = '\u{915e1}';
_6.0 = [_4.0];
_7 = [true,false];
_11 = RET;
_11 = -RET;
_12.1.fld5 = 31264_u16 as i32;
_12.0 = ((-90_i8),);
_12.1.fld4 = _3 as i64;
_12.1.fld4 = 22045_i16 as i64;
_12.0 = ((-14_i8),);
_12.3 = _8;
_13 = 10345_i16;
_13 = 9532825057645735086_u64 as i16;
_9 = _12.1.fld4;
_12.0.0 = !(-65_i8);
_12.1.fld5 = 1028696523_i32;
_12.1.fld2 = _6.0;
_12.1.fld3 = RET as f32;
_2 = [78848604666051690582756231531817177525_u128,225970887323972096278976486773550242908_u128];
_12.0 = (51_i8,);
_12.0.0 = -3_i8;
Goto(bb5)
}
bb7 = {
_4.0 = -(-4811103507531041590_i64);
_7 = [false,false];
RET = 88_i8 as f64;
Goto(bb4)
}
bb8 = {
_3 = 131_u8;
_4 = ((-7844499867642086660_i64),);
RET = 3585865567_u32 as f64;
_3 = 47_u8;
_2 = [152815230573381127827328030451755883457_u128,197777476217507911862377729884160243802_u128];
RET = 295196606042499897809116864835535715711_u128 as f64;
_7 = [true,true];
Goto(bb3)
}
bb9 = {
RET = 11406722209474065078_usize as f64;
_1 = [20188_u16,4369_u16,8087_u16,7571_u16,2223_u16,61293_u16,11658_u16];
RET = (-161401402889137845934588225643874709987_i128) as f64;
RET = 8600965706793845892_i64 as f64;
_1 = [64845_u16,48765_u16,63262_u16,9285_u16,668_u16,44041_u16,55406_u16];
RET = 514833711_u32 as f64;
RET = 35277419287413989507706180370214460816_i128 as f64;
_2 = [13840809175620679062704796416739142163_u128,228683388678373422867766315999796796213_u128];
_2 = [91841610163076469736655081967057422905_u128,212822361426815616526546664113330989505_u128];
RET = (-8451493230845459798_i64) as f64;
_2 = [108615748309026090759310884527954494593_u128,2256125164972903008366749078077766702_u128];
_2 = [185587933514622996832065780554978497436_u128,321606818094618952474144324873627592068_u128];
RET = 5537236603057862069415207110100422796_u128 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 4583681784142544641_u64 as f64;
_3 = 186_u8;
_2 = [246175521923690587885623993968533754558_u128,201805120293332565532418806722157820222_u128];
_1 = [6062_u16,15221_u16,5885_u16,31136_u16,46748_u16,39274_u16,6136_u16];
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_15 = _11;
_6 = (_12.1.fld2,);
_9 = _4.0;
_8 = _12.3;
_9 = !_12.1.fld4;
_12.0 = (79_i8,);
_2 = [216705772282614852900863647703922578299_u128,291723474606126437016226124506418010601_u128];
_17 = (-9223372036854775808_isize);
_16.0 = core::ptr::addr_of_mut!(_14);
_12.0.0 = _13 as i8;
_9 = (-151592075571189783006547885828899423950_i128) as i64;
_12.0.0 = -49_i8;
_3 = 7583822876273218145_usize as u8;
_12.3 = _8;
RET = _11;
_2 = [305039624789393140603211634535823921816_u128,104314143944647638203550422953030282723_u128];
_18 = [_12.0.0,_12.0.0,_12.0.0,_12.0.0,_12.0.0,_12.0.0,_12.0.0,_12.0.0];
_12.1.fld3 = 8742995930984979821_u64 as f32;
_7 = [false,true];
_1 = [64916_u16,51194_u16,4743_u16,48607_u16,39889_u16,27330_u16,2566_u16];
_12.1.fld2 = [_4.0];
_12.1.fld2 = _6.0;
_6 = (_12.1.fld2,);
_15 = _11 * RET;
_1 = [721_u16,32946_u16,17685_u16,44233_u16,42667_u16,11966_u16,35880_u16];
_6.0 = [_12.1.fld4];
_9 = 28631568260141368855152229082297396432_u128 as i64;
_4 = (_12.1.fld4,);
match _17 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb12 = {
_3 = 131_u8;
_4 = ((-7844499867642086660_i64),);
RET = 3585865567_u32 as f64;
_3 = 47_u8;
_2 = [152815230573381127827328030451755883457_u128,197777476217507911862377729884160243802_u128];
RET = 295196606042499897809116864835535715711_u128 as f64;
_7 = [true,true];
Goto(bb3)
}
bb13 = {
RET = 11406722209474065078_usize as f64;
_1 = [20188_u16,4369_u16,8087_u16,7571_u16,2223_u16,61293_u16,11658_u16];
RET = (-161401402889137845934588225643874709987_i128) as f64;
RET = 8600965706793845892_i64 as f64;
_1 = [64845_u16,48765_u16,63262_u16,9285_u16,668_u16,44041_u16,55406_u16];
RET = 514833711_u32 as f64;
RET = 35277419287413989507706180370214460816_i128 as f64;
_2 = [13840809175620679062704796416739142163_u128,228683388678373422867766315999796796213_u128];
_2 = [91841610163076469736655081967057422905_u128,212822361426815616526546664113330989505_u128];
RET = (-8451493230845459798_i64) as f64;
_2 = [108615748309026090759310884527954494593_u128,2256125164972903008366749078077766702_u128];
_2 = [185587933514622996832065780554978497436_u128,321606818094618952474144324873627592068_u128];
RET = 5537236603057862069415207110100422796_u128 as f64;
RET = (-9223372036854775808_isize) as f64;
RET = 4583681784142544641_u64 as f64;
_3 = 186_u8;
_2 = [246175521923690587885623993968533754558_u128,201805120293332565532418806722157820222_u128];
_1 = [6062_u16,15221_u16,5885_u16,31136_u16,46748_u16,39274_u16,6136_u16];
Goto(bb2)
}
bb14 = {
_3 = 131_u8;
_4 = ((-7844499867642086660_i64),);
RET = 3585865567_u32 as f64;
_3 = 47_u8;
_2 = [152815230573381127827328030451755883457_u128,197777476217507911862377729884160243802_u128];
RET = 295196606042499897809116864835535715711_u128 as f64;
_7 = [true,true];
Goto(bb3)
}
bb15 = {
_4.0 = -(-4811103507531041590_i64);
_7 = [false,false];
RET = 88_i8 as f64;
Goto(bb4)
}
bb16 = {
_4 = (6993262293007023294_i64,);
_7 = [false,false];
_4 = ((-4810230167560013508_i64),);
_8 = '\u{915e1}';
_6.0 = [_4.0];
_7 = [true,false];
_11 = RET;
_11 = -RET;
_12.1.fld5 = 31264_u16 as i32;
_12.0 = ((-90_i8),);
_12.1.fld4 = _3 as i64;
_12.1.fld4 = 22045_i16 as i64;
_12.0 = ((-14_i8),);
_12.3 = _8;
_13 = 10345_i16;
_13 = 9532825057645735086_u64 as i16;
_9 = _12.1.fld4;
_12.0.0 = !(-65_i8);
_12.1.fld5 = 1028696523_i32;
_12.1.fld2 = _6.0;
_12.1.fld3 = RET as f32;
_2 = [78848604666051690582756231531817177525_u128,225970887323972096278976486773550242908_u128];
_12.0 = (51_i8,);
_12.0.0 = -3_i8;
Goto(bb5)
}
bb17 = {
_6.0 = _12.1.fld2;
_6 = (_12.1.fld2,);
_12.1.fld0 = Adt36::Variant1 { fld0: _3 };
_12.1.fld4 = !_4.0;
_15 = 9156_u16 as f64;
_4 = (_12.1.fld4,);
RET = 63345_u16 as f64;
_12.1.fld3 = 162686413107109674679204970368164915472_i128 as f32;
Goto(bb18)
}
bb18 = {
Call(_20 = dump_var(2_usize, 3_usize, Move(_3), 7_usize, Move(_7), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_20 = dump_var(2_usize, 18_usize, Move(_18), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i8,mut _2: (i32,),mut _3: u128,mut _4: u128,mut _5: u128) -> isize {
mir! {
type RET = isize;
let _6: [i64; 1];
let _7: *const &'static *const char;
let _8: &'static [u8; 7];
let _9: *const *mut &'static i32;
let _10: f32;
let _11: isize;
let _12: bool;
let _13: i32;
let _14: *const *mut &'static i32;
let _15: ();
let _16: ();
{
_4 = '\u{b63e3}' as u128;
_2.0 = !147207600_i32;
_6 = [6091758680286593399_i64];
_2.0 = (-1274108331_i32) - (-43980546_i32);
RET = (-117_isize);
_1 = (-52_i8);
_2 = (871170906_i32,);
Call(_3 = fn4(_1, _2.0, _2, _2.0, _4, _2, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = (-7_i8) << _4;
_2 = ((-1015082511_i32),);
_2.0 = (-1946382198_i32) & (-1028720938_i32);
_2.0 = !(-1023895055_i32);
RET = (-109_isize) - (-82_isize);
_6 = [1329072587220148554_i64];
RET = 9223372036854775807_isize;
RET = 103_isize << _5;
RET = 9223372036854775807_isize | 9223372036854775807_isize;
_2 = ((-404843724_i32),);
RET = -(-9223372036854775808_isize);
_5 = RET as u128;
Call(_1 = core::intrinsics::bswap(49_i8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !(-9223372036854775808_isize);
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize) & 83_isize;
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431363367732 => bb10,
_ => bb9
}
}
bb3 = {
_1 = (-7_i8) << _4;
_2 = ((-1015082511_i32),);
_2.0 = (-1946382198_i32) & (-1028720938_i32);
_2.0 = !(-1023895055_i32);
RET = (-109_isize) - (-82_isize);
_6 = [1329072587220148554_i64];
RET = 9223372036854775807_isize;
RET = 103_isize << _5;
RET = 9223372036854775807_isize | 9223372036854775807_isize;
_2 = ((-404843724_i32),);
RET = -(-9223372036854775808_isize);
_5 = RET as u128;
Call(_1 = core::intrinsics::bswap(49_i8), ReturnTo(bb2), UnwindUnreachable())
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
_3 = _4 >> _1;
_6 = [4116562762575171088_i64];
_1 = -(-62_i8);
_4 = !_5;
_2 = (1709286626_i32,);
RET = -(-9223372036854775808_isize);
RET = 51001_u16 as isize;
_6 = [6752765433922927166_i64];
_4 = _3;
RET = (-9223372036854775808_isize);
_2.0 = (-183469754_i32);
_5 = 3580598987_u32 as u128;
_2 = (878083534_i32,);
_4 = _3 - _3;
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
RET = 9223372036854775807_isize;
_3 = _4;
_3 = _4 * _4;
RET = 9223372036854775807_isize >> _4;
_6 = [7574309930557892585_i64];
_11 = 15908261062512831693_usize as isize;
_1 = (-59_i8) | 33_i8;
_2 = ((-1983311642_i32),);
_10 = RET as f32;
_6 = [6379072460224760491_i64];
_2.0 = 1902126808_i32;
_4 = 2434639532544123365_u64 as u128;
_11 = !RET;
_4 = _3 ^ _3;
Call(_6 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12 = !true;
match _2.0 {
0 => bb8,
1 => bb12,
2 => bb13,
3 => bb14,
1902126808 => bb16,
_ => bb15
}
}
bb12 = {
_3 = _4 >> _1;
_6 = [4116562762575171088_i64];
_1 = -(-62_i8);
_4 = !_5;
_2 = (1709286626_i32,);
RET = -(-9223372036854775808_isize);
RET = 51001_u16 as isize;
_6 = [6752765433922927166_i64];
_4 = _3;
RET = (-9223372036854775808_isize);
_2.0 = (-183469754_i32);
_5 = 3580598987_u32 as u128;
_2 = (878083534_i32,);
_4 = _3 - _3;
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
RET = 9223372036854775807_isize;
_3 = _4;
_3 = _4 * _4;
RET = 9223372036854775807_isize >> _4;
_6 = [7574309930557892585_i64];
_11 = 15908261062512831693_usize as isize;
_1 = (-59_i8) | 33_i8;
_2 = ((-1983311642_i32),);
_10 = RET as f32;
_6 = [6379072460224760491_i64];
_2.0 = 1902126808_i32;
_4 = 2434639532544123365_u64 as u128;
_11 = !RET;
_4 = _3 ^ _3;
Call(_6 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
RET = !(-9223372036854775808_isize);
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize) & 83_isize;
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431363367732 => bb10,
_ => bb9
}
}
bb14 = {
_1 = (-7_i8) << _4;
_2 = ((-1015082511_i32),);
_2.0 = (-1946382198_i32) & (-1028720938_i32);
_2.0 = !(-1023895055_i32);
RET = (-109_isize) - (-82_isize);
_6 = [1329072587220148554_i64];
RET = 9223372036854775807_isize;
RET = 103_isize << _5;
RET = 9223372036854775807_isize | 9223372036854775807_isize;
_2 = ((-404843724_i32),);
RET = -(-9223372036854775808_isize);
_5 = RET as u128;
Call(_1 = core::intrinsics::bswap(49_i8), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_12 = _11 > RET;
_13 = _4 as i32;
_3 = 10788224421603718132556636968984189576_i128 as u128;
RET = -_11;
RET = _11;
RET = _4 as isize;
_2 = (_13,);
RET = _11 & _11;
_2 = (_13,);
_11 = RET;
_13 = _2.0 + _2.0;
_5 = _13 as u128;
_4 = _5;
_5 = _1 as u128;
_3 = _4 ^ _4;
_3 = 81_u8 as u128;
_2.0 = _13 >> _4;
Goto(bb17)
}
bb17 = {
Call(_15 = dump_var(3_usize, 1_usize, Move(_1), 5_usize, Move(_5), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i8,mut _2: i32,mut _3: (i32,),mut _4: i32,mut _5: u128,mut _6: (i32,),mut _7: isize) -> u128 {
mir! {
type RET = u128;
let _8: &'static Adt38;
let _9: char;
let _10: bool;
let _11: [i64; 1];
let _12: (f32, u8, [i8; 5]);
let _13: u128;
let _14: i64;
let _15: bool;
let _16: [i64; 1];
let _17: &'static i64;
let _18: f64;
let _19: (*mut &'static i128, Adt38);
let _20: *const u64;
let _21: *mut &'static i32;
let _22: (f32, u8, [i8; 5]);
let _23: bool;
let _24: (i32,);
let _25: Adt77;
let _26: bool;
let _27: f32;
let _28: isize;
let _29: (char, &'static i64);
let _30: ();
let _31: ();
{
_4 = !_6.0;
RET = 6_usize as u128;
Call(_4 = fn5(_3, _3, _5, _6.0, _6.0, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _2 * _3.0;
_7 = !9223372036854775807_isize;
_7 = _1 as isize;
_6.0 = _4;
_3.0 = 533077843_u32 as i32;
_6.0 = _4;
_9 = '\u{52ee7}';
_6.0 = -_4;
_3 = (_2,);
_6.0 = _4 + _3.0;
_3.0 = _6.0 - _6.0;
_6.0 = -_4;
_6 = (_3.0,);
_1 = (-18_i8);
_6.0 = _9 as i32;
_6 = _3;
_10 = true & true;
_9 = '\u{fe3c8}';
_5 = _9 as u128;
RET = 2948353000_u32 as u128;
_12.1 = !9_u8;
_13 = RET ^ _5;
Goto(bb2)
}
bb2 = {
_9 = '\u{423dc}';
_14 = _1 as i64;
_12.2 = [_1,_1,_1,_1,_1];
RET = _5;
_12.2 = [_1,_1,_1,_1,_1];
_13 = _5 & RET;
_6.0 = _3.0 ^ _3.0;
RET = !_13;
_5 = _14 as u128;
RET = _13 ^ _13;
_14 = !(-4733732380267297884_i64);
_14 = (-3539353201955373728_i64) ^ 3415171725900657169_i64;
_15 = _10;
_1 = !51_i8;
_14 = -1110982913203796819_i64;
_2 = !_6.0;
_3 = _6;
_12.0 = _12.1 as f32;
_5 = RET;
_5 = !_13;
_4 = _3.0;
_10 = !_15;
_12.1 = _14 as u8;
_5 = _12.0 as u128;
_12.1 = 152_u8;
_14 = 5687684063301728576_i64 << _4;
_16 = [_14];
Call(_1 = core::intrinsics::bswap(92_i8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.0 = 4129_u16 as f32;
RET = _5 & _13;
_14 = (-8313602716693032587_i64);
Call(_13 = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12.1 = !149_u8;
match _14 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463455061004715075178869 => bb9,
_ => bb8
}
}
bb5 = {
_12.0 = 4129_u16 as f32;
RET = _5 & _13;
_14 = (-8313602716693032587_i64);
Call(_13 = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_9 = '\u{423dc}';
_14 = _1 as i64;
_12.2 = [_1,_1,_1,_1,_1];
RET = _5;
_12.2 = [_1,_1,_1,_1,_1];
_13 = _5 & RET;
_6.0 = _3.0 ^ _3.0;
RET = !_13;
_5 = _14 as u128;
RET = _13 ^ _13;
_14 = !(-4733732380267297884_i64);
_14 = (-3539353201955373728_i64) ^ 3415171725900657169_i64;
_15 = _10;
_1 = !51_i8;
_14 = -1110982913203796819_i64;
_2 = !_6.0;
_3 = _6;
_12.0 = _12.1 as f32;
_5 = RET;
_5 = !_13;
_4 = _3.0;
_10 = !_15;
_12.1 = _14 as u8;
_5 = _12.0 as u128;
_12.1 = 152_u8;
_14 = 5687684063301728576_i64 << _4;
_16 = [_14];
Call(_1 = core::intrinsics::bswap(92_i8), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4 = _2 * _3.0;
_7 = !9223372036854775807_isize;
_7 = _1 as isize;
_6.0 = _4;
_3.0 = 533077843_u32 as i32;
_6.0 = _4;
_9 = '\u{52ee7}';
_6.0 = -_4;
_3 = (_2,);
_6.0 = _4 + _3.0;
_3.0 = _6.0 - _6.0;
_6.0 = -_4;
_6 = (_3.0,);
_1 = (-18_i8);
_6.0 = _9 as i32;
_6 = _3;
_10 = true & true;
_9 = '\u{fe3c8}';
_5 = _9 as u128;
RET = 2948353000_u32 as u128;
_12.1 = !9_u8;
_13 = RET ^ _5;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_9 = '\u{cc44d}';
_12.2 = [_1,_1,_1,_1,_1];
_15 = _10;
_3 = (_2,);
_5 = _12.0 as u128;
_16 = [_14];
RET = 2911725173_u32 as u128;
_10 = _6.0 != _3.0;
_12.1 = 112_u8 >> _3.0;
_12.1 = !145_u8;
_5 = _13;
_7 = !9223372036854775807_isize;
_3 = _6;
_11 = [_14];
_3.0 = _4 >> _6.0;
_17 = &_14;
_19.1.fld1.0 = _1 >> _4;
_11 = [(*_17)];
_8 = &_19.1;
_19.1.fld0 = core::ptr::addr_of_mut!(_22.0);
_11 = _16;
_18 = _7 as f64;
_17 = &_14;
Call(_9 = fn7(Move(_8), _3, _3, _10, _7, (*_8).fld1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = !_10;
match (*_17) {
0 => bb1,
1 => bb6,
2 => bb5,
3 => bb4,
4 => bb11,
340282366920938463455061004715075178869 => bb13,
_ => bb12
}
}
bb11 = {
_4 = _2 * _3.0;
_7 = !9223372036854775807_isize;
_7 = _1 as isize;
_6.0 = _4;
_3.0 = 533077843_u32 as i32;
_6.0 = _4;
_9 = '\u{52ee7}';
_6.0 = -_4;
_3 = (_2,);
_6.0 = _4 + _3.0;
_3.0 = _6.0 - _6.0;
_6.0 = -_4;
_6 = (_3.0,);
_1 = (-18_i8);
_6.0 = _9 as i32;
_6 = _3;
_10 = true & true;
_9 = '\u{fe3c8}';
_5 = _9 as u128;
RET = 2948353000_u32 as u128;
_12.1 = !9_u8;
_13 = RET ^ _5;
Goto(bb2)
}
bb12 = {
_9 = '\u{423dc}';
_14 = _1 as i64;
_12.2 = [_1,_1,_1,_1,_1];
RET = _5;
_12.2 = [_1,_1,_1,_1,_1];
_13 = _5 & RET;
_6.0 = _3.0 ^ _3.0;
RET = !_13;
_5 = _14 as u128;
RET = _13 ^ _13;
_14 = !(-4733732380267297884_i64);
_14 = (-3539353201955373728_i64) ^ 3415171725900657169_i64;
_15 = _10;
_1 = !51_i8;
_14 = -1110982913203796819_i64;
_2 = !_6.0;
_3 = _6;
_12.0 = _12.1 as f32;
_5 = RET;
_5 = !_13;
_4 = _3.0;
_10 = !_15;
_12.1 = _14 as u8;
_5 = _12.0 as u128;
_12.1 = 152_u8;
_14 = 5687684063301728576_i64 << _4;
_16 = [_14];
Call(_1 = core::intrinsics::bswap(92_i8), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_22 = (_12.0, _12.1, _12.2);
RET = 4_usize as u128;
_23 = !_15;
_26 = _23;
_22.0 = 875242483_u32 as f32;
_2 = _6.0;
_22 = _12;
_24.0 = _6.0 << _3.0;
_9 = '\u{d2907}';
_3 = (_24.0,);
_19.1.fld1.0 = _1 << _24.0;
Goto(bb14)
}
bb14 = {
_29.1 = &_14;
RET = 5888215604496068436_u64 as u128;
_15 = _10;
_14 = 7750009981504740517_i64 & 8364007628784225877_i64;
_2 = !_6.0;
_12.1 = _22.1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(4_usize, 15_usize, Move(_15), 24_usize, Move(_24), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(4_usize, 5_usize, Move(_5), 1_usize, Move(_1), 26_usize, Move(_26), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (i32,),mut _2: (i32,),mut _3: u128,mut _4: i32,mut _5: i32,mut _6: i32) -> i32 {
mir! {
type RET = i32;
let _7: usize;
let _8: *const u64;
let _9: (*mut Adt66,);
let _10: usize;
let _11: i32;
let _12: u8;
let _13: [i8; 8];
let _14: ((i8,), Adt51, *mut u128, char);
let _15: (i32,);
let _16: f64;
let _17: ((i8,), Adt51, *mut u128, char);
let _18: char;
let _19: u8;
let _20: &'static *const char;
let _21: isize;
let _22: isize;
let _23: *mut f64;
let _24: [i64; 1];
let _25: *const (f64, *mut f64, [i64; 3]);
let _26: *const *mut &'static i128;
let _27: u32;
let _28: *const u64;
let _29: [i64; 3];
let _30: isize;
let _31: f64;
let _32: char;
let _33: [i64; 1];
let _34: i16;
let _35: char;
let _36: i16;
let _37: [u8; 7];
let _38: (f64, *mut f64, [i64; 3]);
let _39: *const *mut &'static i32;
let _40: f64;
let _41: *mut [i64; 3];
let _42: Adt66;
let _43: ();
let _44: ();
{
_2 = _1;
_4 = 79630028191902291321985313853614221737_i128 as i32;
RET = (-5809896630329488516_i64) as i32;
RET = !_2.0;
_7 = !14044583104287647984_usize;
_1.0 = false as i32;
_3 = 131536978886939082734276288384345495076_u128;
_1 = _2;
_1 = (RET,);
RET = 46428_u16 as i32;
RET = _5 & _2.0;
_10 = false as usize;
_4 = !_6;
_1 = _2;
_4 = -RET;
RET = _7 as i32;
RET = -_5;
_10 = !_7;
_4 = RET + _5;
_12 = 96_u8;
_13 = [47_i8,51_i8,(-97_i8),22_i8,114_i8,(-115_i8),36_i8,(-83_i8)];
Goto(bb1)
}
bb1 = {
_2.0 = _4;
_13 = [101_i8,119_i8,(-98_i8),41_i8,(-58_i8),3_i8,(-18_i8),34_i8];
RET = _5;
_12 = !119_u8;
RET = -_2.0;
_1 = _2;
_14.1.fld5 = -_4;
_14.3 = '\u{ec654}';
_14.2 = core::ptr::addr_of_mut!(_3);
_14.2 = core::ptr::addr_of_mut!(_3);
_14.1.fld3 = 9223372036854775807_isize as f32;
_14.2 = core::ptr::addr_of_mut!(_3);
_14.0.0 = (-33_i8) & (-23_i8);
_11 = (-144376510328175710534344853674055369972_i128) as i32;
_2.0 = _14.1.fld5;
RET = _2.0 * _1.0;
_10 = _7;
_14.1.fld3 = 15665_i16 as f32;
_15 = (RET,);
_12 = 52_u8 | 222_u8;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
131536978886939082734276288384345495076 => bb6,
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
_14.0.0 = (-40_i8) + (-2_i8);
_14.1.fld4 = -(-730161897706587310_i64);
_14.1.fld5 = _4;
_3 = 2941027629_u32 as u128;
_14.3 = '\u{34b5}';
_14.1.fld4 = _14.3 as i64;
_14.1.fld2 = [_14.1.fld4];
RET = false as i32;
_14.0 = ((-24_i8),);
_14.0.0 = !(-8_i8);
_12 = (-25_isize) as u8;
_15.0 = true as i32;
_17.1.fld4 = true as i64;
_7 = 133570604176360849462414898726345153727_i128 as usize;
_14.1.fld2 = [_14.1.fld4];
_16 = _12 as f64;
_14.1.fld5 = 1914904439_u32 as i32;
_17.2 = core::ptr::addr_of_mut!(_3);
_4 = _14.3 as i32;
_17.0.0 = 48007_u16 as i8;
_17.1.fld3 = _3 as f32;
_12 = 156_u8 ^ 156_u8;
_15.0 = _14.1.fld5;
_1 = _2;
_6 = (-3629_i16) as i32;
Call(_14.1.fld3 = fn6(_13, _15.0, _14.3, _17.1.fld4, _14.0, _16, _13, _2.0, _7, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_22 = !(-32_isize);
_12 = !57_u8;
_19 = _12 >> _11;
Goto(bb8)
}
bb8 = {
_1.0 = !_5;
_14.1.fld4 = _17.1.fld4;
Goto(bb9)
}
bb9 = {
_17.1.fld5 = 57644_u16 as i32;
_17.3 = _14.3;
_21 = _22;
_23 = core::ptr::addr_of_mut!(_16);
_17.1.fld2 = [_17.1.fld4];
Call(_15.0 = core::intrinsics::transmute(_5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2.0 = 15088960654875479462_u64 as i32;
_2 = (RET,);
_6 = _10 as i32;
_17.3 = _14.3;
_2.0 = RET;
_17.1.fld3 = _21 as f32;
_2.0 = _4;
_17.1.fld5 = _1.0 >> _11;
_4 = RET * _6;
_18 = _14.3;
_18 = _17.3;
_2 = _1;
_19 = _12 - _12;
_5 = _14.1.fld5;
Goto(bb11)
}
bb11 = {
_13 = [_17.0.0,_17.0.0,_17.0.0,_14.0.0,_17.0.0,_17.0.0,_14.0.0,_14.0.0];
_16 = _17.1.fld4 as f64;
RET = !_4;
_2.0 = _1.0 << _1.0;
_14.1.fld2 = [_17.1.fld4];
_17.0.0 = -_14.0.0;
_14.0 = (_17.0.0,);
RET = _5 | _6;
_22 = _21;
_14.1.fld3 = _17.1.fld3 + _17.1.fld3;
(*_23) = _3 as f64;
_17.1.fld0 = Adt36::Variant1 { fld0: _19 };
_17.3 = _18;
_11 = _5;
_14.1.fld5 = -_4;
_22 = 17829113189562891723_u64 as isize;
RET = _15.0;
_18 = _17.3;
Goto(bb12)
}
bb12 = {
_2.0 = _1.0;
_18 = _17.3;
_2.0 = -_4;
Goto(bb13)
}
bb13 = {
_14.1.fld2 = _17.1.fld2;
RET = _14.1.fld5 - _17.1.fld5;
_29 = [_17.1.fld4,_17.1.fld4,_17.1.fld4];
_21 = _22;
_17.1.fld3 = _14.1.fld3;
_3 = _14.1.fld3 as u128;
_31 = _10 as f64;
_10 = _7;
_22 = _21;
_12 = Field::<u8>(Variant(_17.1.fld0, 1), 0);
_17.0.0 = _14.0.0 * _14.0.0;
_14.1.fld0 = Move(_17.1.fld0);
_27 = !2330582356_u32;
_35 = _18;
SetDiscriminant(_14.1.fld0, 0);
_18 = _17.3;
_12 = _19 ^ _19;
_31 = (*_23) - (*_23);
Goto(bb14)
}
bb14 = {
_37 = [_19,_12,_19,_12,_12,_12,_19];
_38.2 = [_14.1.fld4,_17.1.fld4,_14.1.fld4];
_14.1.fld5 = RET | _4;
_38 = ((*_23), Move(_23), _29);
_20 = &place!(Field::<(*const char, i8)>(Variant(_14.1.fld0, 0), 0)).0;
_15 = _2;
_17.3 = _14.3;
_14.0.0 = _17.0.0;
_17.1.fld3 = -_14.1.fld3;
_33 = [_14.1.fld4];
_10 = _14.0.0 as usize;
_18 = _17.3;
_34 = 11800_i16;
_17.0.0 = _21 as i8;
_4 = _5;
_14.1.fld3 = _17.1.fld3 - _17.1.fld3;
_17.1.fld2 = _14.1.fld2;
_25 = core::ptr::addr_of!(_38);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(5_usize, 5_usize, Move(_5), 19_usize, Move(_19), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(5_usize, 6_usize, Move(_6), 10_usize, Move(_10), 11_usize, Move(_11), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(5_usize, 37_usize, Move(_37), 7_usize, Move(_7), 13_usize, Move(_13), 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i8; 8],mut _2: i32,mut _3: char,mut _4: i64,mut _5: (i8,),mut _6: f64,mut _7: [i8; 8],mut _8: i32,mut _9: usize,mut _10: (i32,)) -> f32 {
mir! {
type RET = f32;
let _11: (f64, *mut f64, [i64; 3]);
let _12: bool;
let _13: *const *mut &'static i32;
let _14: *mut *const *mut &'static i128;
let _15: Adt79;
let _16: isize;
let _17: f32;
let _18: *mut *const *mut &'static i128;
let _19: isize;
let _20: Adt24;
let _21: bool;
let _22: i32;
let _23: (i32,);
let _24: *const *mut &'static i32;
let _25: *const bool;
let _26: [u16; 7];
let _27: (&'static i32,);
let _28: isize;
let _29: isize;
let _30: f64;
let _31: u16;
let _32: Adt77;
let _33: bool;
let _34: ();
let _35: ();
{
_10 = (_2,);
_10.0 = !_2;
RET = 3242004712_u32 as f32;
_11.2 = [_4,_4,_4];
_9 = !13899517142767176943_usize;
_11.1 = core::ptr::addr_of_mut!(_11.0);
_1 = _7;
_9 = 3_usize;
_4 = (-8817854566800645828_i64);
_1[_9] = _7[_9];
_1 = [_7[_9],_7[_9],_7[_9],_7[_9],_7[_9],_7[_9],_7[_9],_7[_9]];
_9 = !6_usize;
_10.0 = 259030548449791849386673498815268916799_u128 as i32;
Goto(bb1)
}
bb1 = {
_10.0 = _8 & _8;
_11.0 = (-9223372036854775808_isize) as f64;
_11.2 = [_4,_4,_4];
_5.0 = _3 as i8;
_6 = _11.0 + _11.0;
_11.0 = 85721265362077507607533207373213462772_u128 as f64;
_8 = _10.0 ^ _10.0;
_10.0 = _8 ^ _8;
_6 = _11.0 * _11.0;
_10.0 = !_8;
_12 = true;
_3 = '\u{621b3}';
_9 = 7_usize;
_10 = (_8,);
_1[_9] = _7[_9];
_7[_9] = _3 as i8;
_11.1 = core::ptr::addr_of_mut!(_6);
_7[_9] = !_1[_9];
_1[_9] = _7[_9] | _7[_9];
_2 = 5368092629808571588_u64 as i32;
_5 = (_1[_9],);
_5 = (_7[_9],);
_1 = _7;
RET = 9223372036854775807_isize as f32;
_5 = (_7[_9],);
Goto(bb2)
}
bb2 = {
_9 = 15845146045263484655_usize >> _8;
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.0 = 12_i8;
_9 = !1_usize;
RET = 9223372036854775807_isize as f32;
_12 = true | true;
_6 = _11.0;
_1 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_4 = 2272253891396249971_i64 >> _8;
RET = 7345155793517414490_u64 as f32;
_11.2 = [_4,_4,_4];
_4 = -(-7423166578850200698_i64);
match _5.0 {
0 => bb1,
1 => bb3,
2 => bb4,
12 => bb6,
_ => bb5
}
}
bb3 = {
_10.0 = _8 & _8;
_11.0 = (-9223372036854775808_isize) as f64;
_11.2 = [_4,_4,_4];
_5.0 = _3 as i8;
_6 = _11.0 + _11.0;
_11.0 = 85721265362077507607533207373213462772_u128 as f64;
_8 = _10.0 ^ _10.0;
_10.0 = _8 ^ _8;
_6 = _11.0 * _11.0;
_10.0 = !_8;
_12 = true;
_3 = '\u{621b3}';
_9 = 7_usize;
_10 = (_8,);
_1[_9] = _7[_9];
_7[_9] = _3 as i8;
_11.1 = core::ptr::addr_of_mut!(_6);
_7[_9] = !_1[_9];
_1[_9] = _7[_9] | _7[_9];
_2 = 5368092629808571588_u64 as i32;
_5 = (_1[_9],);
_5 = (_7[_9],);
_1 = _7;
RET = 9223372036854775807_isize as f32;
_5 = (_7[_9],);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_7 = _1;
_11.2 = [_4,_4,_4];
_5.0 = (-61_i8);
_8 = _10.0 + _10.0;
_17 = RET;
_16 = -(-9223372036854775808_isize);
_2 = _8 << _10.0;
RET = _17 - _17;
_17 = RET;
_9 = 4_usize;
_7[_9] = !_5.0;
_2 = 53707_u16 as i32;
_20 = Adt24::Variant0 { fld0: _5,fld1: _3,fld2: 12168843839654246907_u64,fld3: 294385788122109929531392985960305858195_u128,fld4: Move(_11.1),fld5: _8,fld6: 4147998961_u32,fld7: _9 };
Goto(bb7)
}
bb7 = {
place!(Field::<u64>(Variant(_20, 0), 2)) = 6329205169368180725_u64;
place!(Field::<usize>(Variant(_20, 0), 7)) = Field::<i32>(Variant(_20, 0), 5) as usize;
_16 = 9223372036854775807_isize & (-9223372036854775808_isize);
_23.0 = Field::<i32>(Variant(_20, 0), 5);
_16 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
place!(Field::<u32>(Variant(_20, 0), 6)) = _23.0 as u32;
_12 = true;
_12 = false | false;
_7[_9] = _1[_9];
_23.0 = _8 & Field::<i32>(Variant(_20, 0), 5);
_11.1 = core::ptr::addr_of_mut!(_6);
_11.0 = -_6;
place!(Field::<(i8,)>(Variant(_20, 0), 0)) = _5;
_27.0 = &_8;
_11.0 = _6 * _6;
_16 = _11.0 as isize;
match Field::<u64>(Variant(_20, 0), 2) {
0 => bb8,
6329205169368180725 => bb10,
_ => bb9
}
}
bb8 = {
_7 = _1;
_11.2 = [_4,_4,_4];
_5.0 = (-61_i8);
_8 = _10.0 + _10.0;
_17 = RET;
_16 = -(-9223372036854775808_isize);
_2 = _8 << _10.0;
RET = _17 - _17;
_17 = RET;
_9 = 4_usize;
_7[_9] = !_5.0;
_2 = 53707_u16 as i32;
_20 = Adt24::Variant0 { fld0: _5,fld1: _3,fld2: 12168843839654246907_u64,fld3: 294385788122109929531392985960305858195_u128,fld4: Move(_11.1),fld5: _8,fld6: 4147998961_u32,fld7: _9 };
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
_3 = Field::<char>(Variant(_20, 0), 1);
_26[_9] = 65009_u16;
place!(Field::<usize>(Variant(_20, 0), 7)) = _9 >> _23.0;
_19 = (-138084062433331990309884056801849484416_i128) as isize;
_28 = _26[_9] as isize;
_26 = [13231_u16,14823_u16,63561_u16,12915_u16,56120_u16,16495_u16,59883_u16];
place!(Field::<u128>(Variant(_20, 0), 3)) = _12 as u128;
_1[_9] = _12 as i8;
SetDiscriminant(_20, 1);
_22 = -_10.0;
_1 = [_7[_9],_7[_9],_5.0,_7[_9],_7[_9],_7[_9],_5.0,_7[_9]];
_20 = Adt24::Variant1 { fld0: 15133213547618577478_u64 };
_2 = !_23.0;
_12 = !false;
_17 = RET;
_20 = Adt24::Variant1 { fld0: 4185839292224126865_u64 };
_25 = core::ptr::addr_of!(_12);
_12 = true;
_20 = Adt24::Variant2 { fld0: (-16356_i16),fld1: _5 };
RET = _17;
_29 = _19;
_16 = -_28;
_8 = _23.0;
_7[_9] = _17 as i8;
match _26[_9] {
0 => bb1,
56120 => bb11,
_ => bb5
}
}
bb11 = {
place!(Field::<(i8,)>(Variant(_20, 2), 1)) = _5;
_5 = Field::<(i8,)>(Variant(_20, 2), 1);
_22 = _9 as i32;
_21 = (*_25);
_27.0 = &_2;
_3 = '\u{3eff6}';
_31 = Field::<(i8,)>(Variant(_20, 2), 1).0 as u16;
Call(place!(Field::<i16>(Variant(_20, 2), 0)) = core::intrinsics::bswap(14915_i16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_8 = 2585952910_u32 as i32;
_21 = !(*_25);
_26 = [_31,_31,_31,_31,_31,_31,_31];
_10 = (_2,);
place!(Field::<(i8,)>(Variant(_20, 2), 1)) = (_5.0,);
_31 = _26[_9];
_28 = -_29;
_21 = !(*_25);
(*_25) = _17 == _17;
place!(Field::<(i8,)>(Variant(_20, 2), 1)).0 = _1[_9] | _7[_9];
_2 = _9 as i32;
_5.0 = _7[_9] | _7[_9];
match _1[_9] {
0 => bb1,
1 => bb9,
2 => bb10,
3 => bb4,
4 => bb13,
5 => bb14,
6 => bb15,
12 => bb17,
_ => bb16
}
}
bb13 = {
_9 = 15845146045263484655_usize >> _8;
_7 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.0 = 12_i8;
_9 = !1_usize;
RET = 9223372036854775807_isize as f32;
_12 = true | true;
_6 = _11.0;
_1 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_4 = 2272253891396249971_i64 >> _8;
RET = 7345155793517414490_u64 as f32;
_11.2 = [_4,_4,_4];
_4 = -(-7423166578850200698_i64);
match _5.0 {
0 => bb1,
1 => bb3,
2 => bb4,
12 => bb6,
_ => bb5
}
}
bb14 = {
place!(Field::<u64>(Variant(_20, 0), 2)) = 6329205169368180725_u64;
place!(Field::<usize>(Variant(_20, 0), 7)) = Field::<i32>(Variant(_20, 0), 5) as usize;
_16 = 9223372036854775807_isize & (-9223372036854775808_isize);
_23.0 = Field::<i32>(Variant(_20, 0), 5);
_16 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
place!(Field::<u32>(Variant(_20, 0), 6)) = _23.0 as u32;
_12 = true;
_12 = false | false;
_7[_9] = _1[_9];
_23.0 = _8 & Field::<i32>(Variant(_20, 0), 5);
_11.1 = core::ptr::addr_of_mut!(_6);
_11.0 = -_6;
place!(Field::<(i8,)>(Variant(_20, 0), 0)) = _5;
_27.0 = &_8;
_11.0 = _6 * _6;
_16 = _11.0 as isize;
match Field::<u64>(Variant(_20, 0), 2) {
0 => bb8,
6329205169368180725 => bb10,
_ => bb9
}
}
bb15 = {
Return()
}
bb16 = {
_7 = _1;
_11.2 = [_4,_4,_4];
_5.0 = (-61_i8);
_8 = _10.0 + _10.0;
_17 = RET;
_16 = -(-9223372036854775808_isize);
_2 = _8 << _10.0;
RET = _17 - _17;
_17 = RET;
_9 = 4_usize;
_7[_9] = !_5.0;
_2 = 53707_u16 as i32;
_20 = Adt24::Variant0 { fld0: _5,fld1: _3,fld2: 12168843839654246907_u64,fld3: 294385788122109929531392985960305858195_u128,fld4: Move(_11.1),fld5: _8,fld6: 4147998961_u32,fld7: _9 };
Goto(bb7)
}
bb17 = {
_6 = -_11.0;
_11.2 = [_4,_4,_4];
_11.0 = _6;
_30 = -_11.0;
SetDiscriminant(_20, 1);
_10.0 = _23.0;
_1 = _7;
place!(Field::<u64>(Variant(_20, 1), 0)) = 9315381890534954162_u64 - 13209741173725213338_u64;
_5 = (_1[_9],);
_3 = '\u{8c060}';
_26[_9] = _31;
_28 = _3 as isize;
_1[_9] = -_7[_9];
_31 = !_26[_9];
SetDiscriminant(_20, 3);
_6 = _11.0;
_1 = [_7[_9],_5.0,_5.0,_5.0,_7[_9],_5.0,_5.0,_5.0];
_23.0 = _10.0;
_26[_9] = _31;
_29 = !_19;
RET = _17;
_2 = _23.0;
_33 = !_12;
_19 = _28;
_11.0 = _31 as f64;
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(6_usize, 2_usize, Move(_2), 16_usize, Move(_16), 19_usize, Move(_19), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(6_usize, 9_usize, Move(_9), 31_usize, Move(_31), 26_usize, Move(_26), 33_usize, Move(_33)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(6_usize, 7_usize, Move(_7), 5_usize, Move(_5), 35_usize, _35, 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: &'static Adt38,mut _2: (i32,),mut _3: (i32,),mut _4: bool,mut _5: isize,mut _6: (i8,)) -> char {
mir! {
type RET = char;
let _7: (i8,);
let _8: i8;
let _9: &'static (*const char, i8);
let _10: &'static (f32, u8, [i8; 5]);
let _11: (i32,);
let _12: (i64,);
let _13: f32;
let _14: f32;
let _15: isize;
let _16: isize;
let _17: f32;
let _18: ([i64; 1],);
let _19: (*mut &'static i128, Adt38);
let _20: &'static i32;
let _21: [i8; 8];
let _22: Adt36;
let _23: i64;
let _24: Adt51;
let _25: isize;
let _26: [i32; 3];
let _27: bool;
let _28: i16;
let _29: isize;
let _30: &'static (f32, u8, [i8; 5]);
let _31: *const &'static *const char;
let _32: &'static Adt38;
let _33: [u128; 2];
let _34: &'static i128;
let _35: &'static (f32, u8, [i8; 5]);
let _36: ();
let _37: ();
{
_6 = (90_i8,);
RET = '\u{f6388}';
_7.0 = _6.0 << _3.0;
_2.0 = RET as i32;
_6 = (_7.0,);
_4 = true;
_5 = 9223372036854775807_isize << _3.0;
_7.0 = _6.0 + _6.0;
_2 = _3;
_6.0 = 33_u8 as i8;
_3 = (_2.0,);
_2 = _3;
_3.0 = (-7318423815695454000_i64) as i32;
_8 = -_7.0;
_11.0 = 6501623649092638059_i64 as i32;
RET = '\u{357fd}';
_11 = (_2.0,);
Goto(bb1)
}
bb1 = {
_8 = _5 as i8;
_3.0 = 643437459_u32 as i32;
_4 = !true;
_12 = (9070926727186334853_i64,);
_7 = (_8,);
_8 = _7.0;
_6.0 = -_7.0;
_11.0 = 12732_i16 as i32;
_14 = 4533_i16 as f32;
_6 = _7;
_5 = -(-9223372036854775808_isize);
_16 = _2.0 as isize;
_11 = (_2.0,);
_7 = (_8,);
_11.0 = 16_u8 as i32;
_12 = (5564926562638460644_i64,);
RET = '\u{817ed}';
_11 = _2;
_2 = _11;
_11.0 = _3.0 - _2.0;
Call(_3 = fn8(_2.0, _16, _2.0, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = '\u{cd312}';
_15 = _16 ^ _16;
_14 = 180799139514230687637258872712144414704_u128 as f32;
_5 = !_16;
_13 = _14;
_7.0 = _8 >> _15;
_7 = (_8,);
RET = '\u{a3650}';
_7 = (_8,);
_4 = true;
_7.0 = 628539535_u32 as i8;
_8 = -_6.0;
_11 = _2;
_7.0 = _6.0;
_15 = !_16;
_3 = (_2.0,);
_4 = false ^ true;
_7 = _6;
Goto(bb3)
}
bb3 = {
_3 = (_2.0,);
_2 = (_3.0,);
RET = '\u{bbea1}';
_2 = _11;
_5 = _14 as isize;
_17 = -_14;
_1 = &_19.1;
RET = '\u{975bd}';
_12 = ((-6261469336306963940_i64),);
_8 = !_7.0;
_12.0 = (-1254110190371897239_i64);
match _12.0 {
340282366920938463462120497241396314217 => bb5,
_ => bb4
}
}
bb4 = {
RET = '\u{cd312}';
_15 = _16 ^ _16;
_14 = 180799139514230687637258872712144414704_u128 as f32;
_5 = !_16;
_13 = _14;
_7.0 = _8 >> _15;
_7 = (_8,);
RET = '\u{a3650}';
_7 = (_8,);
_4 = true;
_7.0 = 628539535_u32 as i8;
_8 = -_6.0;
_11 = _2;
_7.0 = _6.0;
_15 = !_16;
_3 = (_2.0,);
_4 = false ^ true;
_7 = _6;
Goto(bb3)
}
bb5 = {
_17 = 81173971803808113029993536739478571634_i128 as f32;
_5 = _16 - _16;
_13 = _14;
RET = '\u{83fa0}';
_20 = &_3.0;
_19.1.fld0 = core::ptr::addr_of_mut!(_13);
_1 = &_19.1;
_17 = _2.0 as f32;
_7 = (_6.0,);
_1 = &_19.1;
Call(_3.0 = core::intrinsics::transmute(_2.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = _6;
_16 = -_5;
Goto(bb7)
}
bb7 = {
_2.0 = 5_usize as i32;
_21 = [_7.0,_6.0,_8,_6.0,_7.0,_6.0,_7.0,_8];
_1 = &(*_1);
_5 = -_16;
_19.1.fld1 = _6;
_21 = [_7.0,_7.0,_6.0,_7.0,_7.0,_7.0,_8,_7.0];
_2 = _3;
_2 = _11;
_12.0 = 90999350_u32 as i64;
_3.0 = _2.0 * _2.0;
Goto(bb8)
}
bb8 = {
_4 = false;
_4 = _16 >= _5;
_7.0 = -_19.1.fld1.0;
_22 = Adt36::Variant1 { fld0: 116_u8 };
_23 = _12.0 >> _19.1.fld1.0;
RET = '\u{caa26}';
_14 = 97436748614366755937930209040908407047_i128 as f32;
_24.fld3 = -_17;
_24.fld3 = _17;
_2 = (_3.0,);
_20 = &_3.0;
_3.0 = _2.0 * _2.0;
_25 = -_16;
_1 = &_19.1;
_19.1.fld1.0 = RET as i8;
Goto(bb9)
}
bb9 = {
_6.0 = _8 >> _15;
RET = '\u{41215}';
_1 = &_19.1;
RET = '\u{9b79d}';
_5 = RET as isize;
_16 = !_25;
place!(Field::<u8>(Variant(_22, 1), 0)) = !64_u8;
_13 = _17;
_21 = [_7.0,_8,_8,_8,_6.0,_8,_7.0,_6.0];
RET = '\u{c1255}';
_7 = (_8,);
place!(Field::<u8>(Variant(_22, 1), 0)) = 147_u8 - 157_u8;
_24.fld0 = Move(_22);
Goto(bb10)
}
bb10 = {
_1 = &_19.1;
_27 = !_4;
_18.0 = [_23];
_13 = -_24.fld3;
_2.0 = _3.0;
_11 = (_3.0,);
_24.fld2 = [_23];
_19.1.fld0 = core::ptr::addr_of_mut!(_24.fld3);
Goto(bb11)
}
bb11 = {
_27 = !_4;
_19.1.fld1.0 = 15753_u16 as i8;
_20 = &_3.0;
_6 = (_7.0,);
_6 = (_8,);
_19.1.fld0 = core::ptr::addr_of_mut!(_24.fld3);
_24.fld3 = _13;
_23 = _12.0;
_2.0 = _3.0;
SetDiscriminant(_24.fld0, 0);
place!(Field::<(*const char, i8)>(Variant(_24.fld0, 0), 0)).1 = _6.0;
_19.1.fld0 = core::ptr::addr_of_mut!(_24.fld3);
_13 = -_24.fld3;
_2 = ((*_20),);
_14 = -_13;
RET = '\u{df770}';
place!(Field::<(*const char, i8)>(Variant(_24.fld0, 0), 0)).1 = _8 << _2.0;
_19.1.fld0 = core::ptr::addr_of_mut!(_17);
_24.fld3 = _17 * _17;
place!(Field::<(*const char, i8)>(Variant(_24.fld0, 0), 0)).1 = !_8;
_7.0 = _8 * _8;
_13 = _24.fld3;
_24.fld4 = _23;
_18 = (_24.fld2,);
place!(Field::<(*const char, i8)>(Variant(_24.fld0, 0), 0)).0 = core::ptr::addr_of!(RET);
Goto(bb12)
}
bb12 = {
_12 = (_24.fld4,);
_11 = _2;
place!(Field::<(*const char, i8)>(Variant(_24.fld0, 0), 0)).1 = _6.0;
_3 = (_11.0,);
_7 = _19.1.fld1;
_2 = (_3.0,);
_24.fld3 = 3837960282_u32 as f32;
_12.0 = 354909457_u32 as i64;
_18 = (_24.fld2,);
RET = '\u{b7e05}';
_7 = (Field::<(*const char, i8)>(Variant(_24.fld0, 0), 0).1,);
_12.0 = _24.fld4;
_1 = &_19.1;
_17 = _13 + _14;
_24.fld0 = Adt36::Variant1 { fld0: 177_u8 };
_24.fld2 = _18.0;
_12 = (_24.fld4,);
_28 = !2448_i16;
_24.fld5 = _3.0;
_24.fld2 = _18.0;
_12 = (_23,);
Goto(bb13)
}
bb13 = {
_26 = [_24.fld5,_11.0,_3.0];
Call(_19.1 = fn9(_11, _11, _4, _16), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_27 = !_4;
_12.0 = _24.fld4;
_11 = (_2.0,);
_18.0 = [_24.fld4];
_19.1.fld1 = _6;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(7_usize, 2_usize, Move(_2), 21_usize, Move(_21), 23_usize, Move(_23), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(7_usize, 27_usize, Move(_27), 3_usize, Move(_3), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(7_usize, 11_usize, Move(_11), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i32,mut _2: isize,mut _3: i32,mut _4: (i8,)) -> (i32,) {
mir! {
type RET = (i32,);
let _5: [i64; 3];
let _6: isize;
let _7: ((i8,), Adt51, *mut u128, char);
let _8: f64;
let _9: isize;
let _10: ();
let _11: ();
{
RET = (_1,);
_4 = (83_i8,);
_3 = RET.0;
RET = (_1,);
RET.0 = _1 * _1;
RET = (_3,);
RET = (_1,);
_1 = -_3;
_3 = -RET.0;
RET.0 = false as i32;
RET.0 = -_1;
RET = (_3,);
_5 = [(-4932748329111360322_i64),460548346880604872_i64,3754798453052496204_i64];
_4.0 = !69_i8;
RET.0 = _3 | _1;
RET.0 = _2 as i32;
RET.0 = _3;
_7.0 = (_4.0,);
_7.1.fld3 = 304124303743941198737024403088043360656_u128 as f32;
RET.0 = _3 | _3;
_1 = -_3;
_5 = [100668769973390905_i64,8206869949776213867_i64,2803435161251051033_i64];
_7.3 = '\u{c5115}';
_4.0 = _7.0.0 >> _1;
RET = (_3,);
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(8_usize, 2_usize, Move(_2), 4_usize, Move(_4), 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: (i32,),mut _2: (i32,),mut _3: bool,mut _4: isize) -> Adt38 {
mir! {
type RET = Adt38;
let _5: [u64; 7];
let _6: u32;
let _7: &'static [u8; 7];
let _8: (char, &'static i64);
let _9: u8;
let _10: char;
let _11: u64;
let _12: u32;
let _13: char;
let _14: Adt51;
let _15: &'static (f32, u8, [i8; 5]);
let _16: &'static i32;
let _17: isize;
let _18: isize;
let _19: u32;
let _20: isize;
let _21: isize;
let _22: &'static *const char;
let _23: f64;
let _24: [u128; 2];
let _25: ();
let _26: ();
{
RET.fld1 = ((-108_i8),);
_1.0 = 2758044702_u32 as i32;
_3 = true;
RET.fld1.0 = (-39_i8);
RET.fld1 = ((-35_i8),);
RET.fld1.0 = 15_i8;
_4 = 9223372036854775807_isize >> _2.0;
_2.0 = _1.0;
_2 = _1;
_4 = 9223372036854775807_isize * (-9223372036854775808_isize);
_3 = false;
_3 = _2.0 != _1.0;
_3 = _4 <= _4;
_6 = !3791801748_u32;
_2.0 = -_1.0;
_2 = (_1.0,);
_5 = [16722656118784069496_u64,18037093017563698131_u64,8040470741426244285_u64,5470966183777820540_u64,13849182664129836823_u64,6196245786147032162_u64,9932363961609949504_u64];
RET.fld1.0 = !(-78_i8);
_1 = _2;
_4 = (-9223372036854775808_isize) << RET.fld1.0;
_1.0 = -_2.0;
RET.fld1 = (37_i8,);
Goto(bb1)
}
bb1 = {
_6 = (-6026573973032942017_i64) as u32;
_3 = false;
_1 = _2;
RET.fld1.0 = 113_i8 & 8_i8;
_4 = (-9223372036854775808_isize);
_2.0 = _1.0 << RET.fld1.0;
_2 = (_1.0,);
_4 = (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_3 = false;
RET.fld1.0 = 33_i8;
_1 = _2;
_8.0 = '\u{a9f38}';
RET.fld1 = ((-42_i8),);
_12 = _6 | _6;
Goto(bb3)
}
bb3 = {
_12 = 0_usize as u32;
_3 = !false;
_9 = !223_u8;
_2 = _1;
_10 = _8.0;
RET.fld1 = (28_i8,);
_5 = [16541996184507589551_u64,17301438438416272812_u64,8053170330837765637_u64,9763794295105738569_u64,6649407346178324229_u64,8888516719001551615_u64,12861059628763863714_u64];
_11 = 14714650064040724822_usize as u64;
_11 = 14458589366098332589_u64;
RET.fld1.0 = !(-33_i8);
_3 = !false;
_6 = _12;
match _11 {
0 => bb1,
14458589366098332589 => bb4,
_ => bb2
}
}
bb4 = {
RET.fld1 = (35_i8,);
RET.fld1.0 = (-127_i8);
_13 = _10;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
_14.fld4 = 7023357972847329407_i64 << _9;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
_8.1 = &_14.fld4;
_3 = _11 <= _11;
_14.fld5 = _2.0;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
RET.fld1.0 = _4 as i8;
_5 = [_11,_11,_11,_11,_11,_11,_11];
_14.fld3 = 51439688426485470230513145202438637769_i128 as f32;
Call(_2 = fn10(Move(_8.1), _13, _4, _10, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2.0 = -_1.0;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
_14.fld2 = [_14.fld4];
_9 = _3 as u8;
_13 = _8.0;
RET.fld1.0 = -(-95_i8);
_14.fld0 = Adt36::Variant1 { fld0: _9 };
_14.fld0 = Adt36::Variant1 { fld0: _9 };
_14.fld5 = !_1.0;
RET.fld1.0 = 13484_i16 as i8;
_5 = [_11,_11,_11,_11,_11,_11,_11];
_8.0 = _13;
_16 = &_1.0;
_10 = _13;
_1 = _2;
_16 = &_2.0;
_13 = _8.0;
_14.fld1 = core::ptr::addr_of!(_11);
Goto(bb6)
}
bb6 = {
RET.fld1 = (41_i8,);
SetDiscriminant(_14.fld0, 2);
_9 = _11 as u8;
_8.0 = _10;
match _4 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb7 = {
_2.0 = -_1.0;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
_14.fld2 = [_14.fld4];
_9 = _3 as u8;
_13 = _8.0;
RET.fld1.0 = -(-95_i8);
_14.fld0 = Adt36::Variant1 { fld0: _9 };
_14.fld0 = Adt36::Variant1 { fld0: _9 };
_14.fld5 = !_1.0;
RET.fld1.0 = 13484_i16 as i8;
_5 = [_11,_11,_11,_11,_11,_11,_11];
_8.0 = _13;
_16 = &_1.0;
_10 = _13;
_1 = _2;
_16 = &_2.0;
_13 = _8.0;
_14.fld1 = core::ptr::addr_of!(_11);
Goto(bb6)
}
bb8 = {
RET.fld1 = (35_i8,);
RET.fld1.0 = (-127_i8);
_13 = _10;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
_14.fld4 = 7023357972847329407_i64 << _9;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
_8.1 = &_14.fld4;
_3 = _11 <= _11;
_14.fld5 = _2.0;
RET.fld0 = core::ptr::addr_of_mut!(_14.fld3);
RET.fld1.0 = _4 as i8;
_5 = [_11,_11,_11,_11,_11,_11,_11];
_14.fld3 = 51439688426485470230513145202438637769_i128 as f32;
Call(_2 = fn10(Move(_8.1), _13, _4, _10, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_12 = 0_usize as u32;
_3 = !false;
_9 = !223_u8;
_2 = _1;
_10 = _8.0;
RET.fld1 = (28_i8,);
_5 = [16541996184507589551_u64,17301438438416272812_u64,8053170330837765637_u64,9763794295105738569_u64,6649407346178324229_u64,8888516719001551615_u64,12861059628763863714_u64];
_11 = 14714650064040724822_usize as u64;
_11 = 14458589366098332589_u64;
RET.fld1.0 = !(-33_i8);
_3 = !false;
_6 = _12;
match _11 {
0 => bb1,
14458589366098332589 => bb4,
_ => bb2
}
}
bb10 = {
_3 = false;
RET.fld1.0 = 33_i8;
_1 = _2;
_8.0 = '\u{a9f38}';
RET.fld1 = ((-42_i8),);
_12 = _6 | _6;
Goto(bb3)
}
bb11 = {
_6 = (-6026573973032942017_i64) as u32;
_3 = false;
_1 = _2;
RET.fld1.0 = 113_i8 & 8_i8;
_4 = (-9223372036854775808_isize);
_2.0 = _1.0 << RET.fld1.0;
_2 = (_1.0,);
_4 = (-9223372036854775808_isize);
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_14.fld3 = 60991537999303607342618968509272726910_u128 as f32;
_19 = !_12;
_14.fld2 = [_14.fld4];
_14.fld3 = _14.fld4 as f32;
_2.0 = !_14.fld5;
_3 = true;
_19 = _3 as u32;
_13 = _8.0;
_20 = _4;
_1 = _2;
RET.fld1 = (41_i8,);
_6 = (-74684055830608506684068727634248193285_i128) as u32;
_21 = !_4;
place!(Field::<u64>(Variant(_14.fld0, 2), 1)) = _11;
Goto(bb14)
}
bb14 = {
_18 = 128444649683112579214678730256088173021_u128 as isize;
RET.fld1.0 = _19 as i8;
_10 = _8.0;
place!(Field::<(*const char, i8)>(Variant(_14.fld0, 2), 2)).1 = RET.fld1.0;
_5 = [_11,_11,_11,_11,_11,_11,_11];
_8.1 = &_14.fld4;
place!(Field::<bool>(Variant(_14.fld0, 2), 0)) = _6 > _6;
RET.fld1 = (Field::<(*const char, i8)>(Variant(_14.fld0, 2), 2).1,);
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(9_usize, 5_usize, Move(_5), 2_usize, Move(_2), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(9_usize, 19_usize, Move(_19), 1_usize, Move(_1), 11_usize, Move(_11), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: &'static i64,mut _2: char,mut _3: isize,mut _4: char,mut _5: char) -> (i32,) {
mir! {
type RET = (i32,);
let _6: isize;
let _7: [i32; 3];
let _8: f64;
let _9: u8;
let _10: f64;
let _11: (i32,);
let _12: i32;
let _13: i8;
let _14: u8;
let _15: i8;
let _16: bool;
let _17: f32;
let _18: [u8; 7];
let _19: i128;
let _20: [u16; 7];
let _21: (i64,);
let _22: bool;
let _23: *mut f64;
let _24: ();
let _25: ();
{
RET = ((-778670816_i32),);
_2 = _5;
RET = (376262089_i32,);
_6 = _3 + _3;
RET = (1400914085_i32,);
_4 = _5;
match RET.0 {
1400914085 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_4 = _5;
RET.0 = true as i32;
_3 = (-72_i8) as isize;
_6 = 314683033428425913849962670382194950948_u128 as isize;
_7 = [RET.0,RET.0,RET.0];
RET.0 = (-271659639_i32);
_8 = 4330865300881353990_usize as f64;
_7 = [RET.0,RET.0,RET.0];
_5 = _4;
_3 = !_6;
_3 = 8027049256734443111_i64 as isize;
_2 = _5;
_3 = (-29547_i16) as isize;
_8 = 15531981934367492053_u64 as f64;
_4 = _2;
_2 = _5;
_2 = _4;
match RET.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431496551817 => bb10,
_ => bb9
}
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
RET = ((-1806961765_i32),);
RET = ((-1298381772_i32),);
_5 = _4;
RET.0 = !527780024_i32;
_10 = _8;
_5 = _2;
RET.0 = 6511_u16 as i32;
RET = (1027720445_i32,);
_4 = _2;
_3 = !_6;
_2 = _4;
RET = (317365934_i32,);
_2 = _4;
Call(_9 = fn11(_2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = !_3;
_4 = _5;
_8 = _10;
RET = (1937916467_i32,);
_6 = (-20216310217844442243037330615220926418_i128) as isize;
_10 = -_8;
_7 = [RET.0,RET.0,RET.0];
_7 = [RET.0,RET.0,RET.0];
RET.0 = (-1948518702_i32) >> _9;
_7 = [RET.0,RET.0,RET.0];
_3 = !_6;
Goto(bb12)
}
bb12 = {
_5 = _2;
_2 = _5;
RET = ((-484964818_i32),);
RET.0 = (-246710736_i32) ^ (-141635547_i32);
RET.0 = 296193672_i32;
_6 = _3;
RET = ((-466739052_i32),);
_7 = [RET.0,RET.0,RET.0];
_7 = [RET.0,RET.0,RET.0];
RET.0 = 1402622906_i32;
RET.0 = 787160152_i32 >> _9;
_5 = _2;
RET.0 = (-2117087117_i32) | 1976869434_i32;
_10 = (-31692_i16) as f64;
_7 = [RET.0,RET.0,RET.0];
RET.0 = (-1503_i16) as i32;
RET.0 = !(-493797720_i32);
_3 = _6 ^ _6;
_4 = _2;
RET.0 = _10 as i32;
_12 = -RET.0;
Goto(bb13)
}
bb13 = {
_2 = _5;
_12 = RET.0 | RET.0;
_2 = _5;
_7 = [_12,RET.0,RET.0];
_18 = [_9,_9,_9,_9,_9,_9,_9];
_18 = [_9,_9,_9,_9,_9,_9,_9];
_14 = _9;
_2 = _4;
_18 = [_9,_9,_9,_14,_9,_9,_14];
_13 = !(-2_i8);
RET.0 = 33006_u16 as i32;
_3 = -_6;
_11 = RET;
RET = (_12,);
Call(_15 = core::intrinsics::transmute(_14), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = (_12,);
_6 = !_3;
_11.0 = 41091_u16 as i32;
_11.0 = _14 as i32;
_18 = [_14,_14,_14,_9,_9,_14,_9];
_21 = (2677197416197023038_i64,);
_5 = _2;
RET.0 = _11.0;
_2 = _5;
_16 = _12 == RET.0;
_5 = _2;
_18 = [_14,_14,_9,_9,_9,_9,_14];
_14 = _9 ^ _9;
_12 = _11.0;
RET = _11;
_11 = (RET.0,);
RET.0 = _11.0 << _14;
_18 = [_14,_14,_9,_14,_14,_9,_9];
_11.0 = !RET.0;
_22 = _11.0 != RET.0;
_10 = _8 - _8;
_2 = _4;
_11 = RET;
_6 = _3;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(10_usize, 7_usize, Move(_7), 18_usize, Move(_18), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(10_usize, 13_usize, Move(_13), 6_usize, Move(_6), 21_usize, Move(_21), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: char) -> u8 {
mir! {
type RET = u8;
let _2: (u8, i8);
let _3: Adt38;
let _4: *const [u64; 7];
let _5: *mut *const *mut &'static i128;
let _6: u16;
let _7: [u16; 7];
let _8: (*const char, i8);
let _9: &'static *mut [i64; 3];
let _10: isize;
let _11: (&'static i32,);
let _12: (f64, *mut f64, [i64; 3]);
let _13: [i8; 5];
let _14: [u128; 2];
let _15: *const (f64, *mut f64, [i64; 3]);
let _16: *const (f64, *mut f64, [i64; 3]);
let _17: [i64; 1];
let _18: isize;
let _19: (i64,);
let _20: (*mut &'static i32, *mut u128);
let _21: u32;
let _22: ();
let _23: ();
{
_1 = '\u{c086}';
_1 = '\u{563a0}';
RET = 33_u8 ^ 102_u8;
RET = 146_u8 - 28_u8;
RET = 5_u8;
_1 = '\u{b6bdd}';
RET = 64_u8 | 252_u8;
RET = 17_u8;
RET = 21_u8 & 183_u8;
_2 = (RET, (-24_i8));
_2.1 = 8482895583378821664_usize as i8;
_2 = (RET, 87_i8);
_2.0 = RET + RET;
_1 = '\u{7b3e}';
_2.1 = 31457_i16 as i8;
_2.1 = (-108_i8) << _2.0;
_1 = '\u{f6a19}';
_2.0 = RET - RET;
_2.0 = 4846692158126620046_u64 as u8;
RET = !_2.0;
_2 = (RET, 65_i8);
RET = !_2.0;
_2 = (RET, 36_i8);
_3.fld1.0 = 167832794234735398273770319427446175671_u128 as i8;
_1 = '\u{d8343}';
_2.0 = !RET;
match _2.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
36 => bb9,
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
_3.fld1.0 = true as i8;
_3.fld1.0 = 9223372036854775807_isize as i8;
Goto(bb10)
}
bb10 = {
_2.1 = _3.fld1.0 << _3.fld1.0;
RET = !_2.0;
_3.fld1 = (_2.1,);
_2 = (RET, _3.fld1.0);
_3.fld1.0 = -_2.1;
_3.fld1.0 = _2.1 * _2.1;
_2 = (RET, _3.fld1.0);
RET = _2.0;
_1 = '\u{cd2d6}';
_3.fld1 = (_2.1,);
_2 = (RET, _3.fld1.0);
RET = _2.0;
_2.0 = RET;
_6 = !48927_u16;
_8.1 = 27_isize as i8;
_3.fld1.0 = 181937651075681800480976934011478714947_u128 as i8;
_6 = 20086_u16;
_2.1 = _3.fld1.0 & _8.1;
_8.0 = core::ptr::addr_of!(_1);
_1 = '\u{f111}';
RET = !_2.0;
_2 = (RET, _8.1);
RET = _2.0 & _2.0;
_8.1 = _2.1;
_7 = [_6,_6,_6,_6,_6,_6,_6];
_2.1 = (-14733_i16) as i8;
match _6 {
0 => bb4,
1 => bb2,
20086 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_3.fld1 = (_8.1,);
_10 = (-2_isize) | 103_isize;
_2.1 = _3.fld1.0;
_8.0 = core::ptr::addr_of!(_1);
_2 = (RET, _3.fld1.0);
_2.1 = !_3.fld1.0;
_2 = (RET, _8.1);
_8.0 = core::ptr::addr_of!(_1);
_8.0 = core::ptr::addr_of!(_1);
_12.0 = 1_usize as f64;
Goto(bb13)
}
bb13 = {
_3.fld1.0 = 14305946768413602488_usize as i8;
_8.0 = core::ptr::addr_of!(_1);
_7 = [_6,_6,_6,_6,_6,_6,_6];
_8.0 = core::ptr::addr_of!(_1);
RET = _2.0 + _2.0;
_14 = [162225297740707233609049701532587682892_u128,89411868698946202817741499188562340203_u128];
_14 = [4579837242045627540749662975787966913_u128,175649579722836059981395471327330601632_u128];
_2 = (RET, _8.1);
_12.2 = [(-6944585267322419269_i64),1454826857425116342_i64,(-7174560030343703589_i64)];
_2.1 = (-3854868552202224386_i64) as i8;
_13 = [_3.fld1.0,_8.1,_8.1,_3.fld1.0,_3.fld1.0];
_7 = [_6,_6,_6,_6,_6,_6,_6];
_3.fld1.0 = _8.1;
_2.1 = 5801391363662691996_u64 as i8;
_2.1 = _8.1;
_6 = _8.1 as u16;
Goto(bb14)
}
bb14 = {
_8.0 = core::ptr::addr_of!(_1);
_12.1 = core::ptr::addr_of_mut!(_12.0);
_2 = (RET, _3.fld1.0);
_15 = core::ptr::addr_of!(_12);
_16 = Move(_15);
_12.1 = core::ptr::addr_of_mut!(_12.0);
_1 = '\u{407eb}';
_14 = [12533903886250134297306329764054945311_u128,108989211625778152448394778658114753568_u128];
RET = _2.0;
_6 = 2482_u16 - 25435_u16;
_16 = core::ptr::addr_of!(_12);
_2.0 = !RET;
(*_16).1 = core::ptr::addr_of_mut!((*_16).0);
_7 = [_6,_6,_6,_6,_6,_6,_6];
_15 = core::ptr::addr_of!((*_16));
_8.0 = core::ptr::addr_of!(_1);
_1 = '\u{57817}';
_7 = [_6,_6,_6,_6,_6,_6,_6];
_2 = (RET, _8.1);
_20.0 = core::ptr::addr_of_mut!(_11.0);
(*_15).1 = core::ptr::addr_of_mut!((*_15).0);
(*_16).0 = (-115952492650666175562254220900471976731_i128) as f64;
_3.fld1 = (_8.1,);
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(11_usize, 1_usize, Move(_1), 14_usize, Move(_14), 2_usize, Move(_2), 23_usize, _23), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u128,mut _2: f64,mut _3: Adt38,mut _4: [u16; 7],mut _5: f32,mut _6: [i32; 3],mut _7: isize) -> i32 {
mir! {
type RET = i32;
let _8: *mut f32;
let _9: char;
let _10: &'static (*const char, i8);
let _11: (u8, i8);
let _12: (*mut &'static i32, *mut u128);
let _13: *const (f64, *mut f64, [i64; 3]);
let _14: f64;
let _15: i128;
let _16: *mut [i64; 3];
let _17: (*mut u128, &'static i64, (*mut &'static i32, *mut u128));
let _18: [u64; 7];
let _19: [i64; 3];
let _20: isize;
let _21: bool;
let _22: [u128; 2];
let _23: &'static Adt38;
let _24: [i8; 5];
let _25: i128;
let _26: i128;
let _27: i128;
let _28: (char, &'static i64);
let _29: &'static (*const char, i8);
let _30: (i64,);
let _31: (&'static i32,);
let _32: isize;
let _33: &'static Adt38;
let _34: &'static Adt38;
let _35: bool;
let _36: isize;
let _37: Adt38;
let _38: *mut f64;
let _39: [u64; 7];
let _40: *mut f64;
let _41: f32;
let _42: ();
let _43: ();
{
_8 = Move(_3.fld0);
RET = !1823854802_i32;
RET = -606624622_i32;
_3.fld0 = Move(_8);
_3.fld1.0 = '\u{4c9eb}' as i8;
_1 = !65047276652357427414567521312272115735_u128;
_8 = core::ptr::addr_of_mut!(_5);
_3.fld0 = Move(_8);
Goto(bb1)
}
bb1 = {
_7 = !9223372036854775807_isize;
_3.fld1 = ((-91_i8),);
RET = (-898223186_i32) ^ 2115618644_i32;
_4 = [32881_u16,61691_u16,13865_u16,26285_u16,65345_u16,14968_u16,567_u16];
RET = (-19186_i16) as i32;
_3.fld1 = (97_i8,);
match _3.fld1.0 {
97 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET = 1253_i16 as i32;
RET = _2 as i32;
RET = -(-1452775494_i32);
_7 = (-9223372036854775808_isize);
_6 = [RET,RET,RET];
_8 = core::ptr::addr_of_mut!(_5);
_7 = (-9223372036854775808_isize) * 49_isize;
_7 = (*_8) as isize;
_3.fld1 = ((-93_i8),);
_4 = [42250_u16,55244_u16,37865_u16,36318_u16,29402_u16,60055_u16,44850_u16];
RET = -1168720765_i32;
_5 = _7 as f32;
_6 = [RET,RET,RET];
_1 = 103084193068665939475240310743820658532_u128 | 238171134414650561152711490741400648656_u128;
_3.fld1.0 = 52_i8;
_9 = '\u{586c8}';
_7 = 9223372036854775807_isize;
RET = !1857850012_i32;
_11 = (28_u8, _3.fld1.0);
_11 = (12_u8, _3.fld1.0);
RET = _11.1 as i32;
Goto(bb4)
}
bb4 = {
_3.fld0 = core::ptr::addr_of_mut!((*_8));
RET = !1014379669_i32;
_8 = core::ptr::addr_of_mut!((*_8));
Goto(bb5)
}
bb5 = {
_11 = (5_u8, _3.fld1.0);
RET = 1071556884_i32;
Goto(bb6)
}
bb6 = {
_9 = '\u{a9c40}';
_7 = (-37_isize) >> _3.fld1.0;
_4 = [13015_u16,55863_u16,23598_u16,52082_u16,57820_u16,15713_u16,25615_u16];
RET = (-1869781402_i32);
_11.1 = (-11698_i16) as i8;
_14 = -_2;
RET = 907612047_i32 ^ 1165043639_i32;
_8 = Move(_3.fld0);
_11.0 = (-158943936553925571383554913356838621551_i128) as u8;
_9 = '\u{e2812}';
_17.0 = core::ptr::addr_of_mut!(_1);
_6 = [RET,RET,RET];
_18 = [3016959460753320939_u64,1520174864224113520_u64,14065120603206042843_u64,5070916250969308984_u64,14629405070595627169_u64,4714075757577935956_u64,16488197503705258171_u64];
_9 = '\u{108a97}';
_3.fld0 = core::ptr::addr_of_mut!(_5);
_18 = [18088045070692395666_u64,9994038844695075376_u64,8511930472834479836_u64,17287244853673080160_u64,6232720479963225168_u64,4948443181052272911_u64,14031405110883056888_u64];
RET = (-3327807990403150311_i64) as i32;
_1 = RET as u128;
_3.fld1.0 = _11.1 * _11.1;
_12.1 = core::ptr::addr_of_mut!(_1);
_18 = [12971074748748055932_u64,10062138778327939271_u64,17252646530921571405_u64,18102305539330209586_u64,12170186173836981332_u64,13418007488674896303_u64,15512191442484467636_u64];
RET = (-445023596_i32) * 222446614_i32;
_16 = core::ptr::addr_of_mut!(_19);
Goto(bb7)
}
bb7 = {
RET = !822869950_i32;
_5 = RET as f32;
_2 = _14;
(*_16) = [(-2070220662682783929_i64),1559283596403787405_i64,3533570000258955617_i64];
_21 = !true;
RET = !1211274466_i32;
_12.1 = core::ptr::addr_of_mut!(_1);
_2 = _14;
_16 = core::ptr::addr_of_mut!(_19);
_11.1 = !_3.fld1.0;
_17.2.1 = Move(_12.1);
_15 = !(-112624913140343794304080108176330852363_i128);
_23 = &_3;
_11.0 = 69_u8 ^ 126_u8;
(*_16) = [3429120102456623519_i64,(-9039215114818059484_i64),621895646982843189_i64];
_14 = _2;
_4 = [41067_u16,54368_u16,48937_u16,28654_u16,9091_u16,53164_u16,5275_u16];
_11.0 = _7 as u8;
RET = -(-1513008807_i32);
Goto(bb8)
}
bb8 = {
_24 = [(*_23).fld1.0,(*_23).fld1.0,_11.1,_11.1,(*_23).fld1.0];
_5 = _1 as f32;
_7 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = _21 as i32;
_5 = _14 as f32;
_19 = [(-2599665853247152090_i64),226807418175619795_i64,(-2283111610738515464_i64)];
_11.0 = _21 as u8;
_22 = [_1,_1];
_20 = _7 & _7;
_22 = [_1,_1];
_24 = [(*_23).fld1.0,(*_23).fld1.0,_11.1,_11.1,_11.1];
_31.0 = &RET;
_20 = 3_usize as isize;
_12.1 = core::ptr::addr_of_mut!(_1);
_1 = _21 as u128;
_3.fld0 = Move(_8);
_32 = -_20;
_34 = Move(_23);
_21 = !false;
_17.2.1 = core::ptr::addr_of_mut!(_1);
Goto(bb9)
}
bb9 = {
_8 = core::ptr::addr_of_mut!(_5);
_35 = !_21;
_28.0 = _9;
Call(_17.2.1 = fn13(Move(_31), (*_16), (*_16), Move(_3), _6, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_24 = [_11.1,_11.1,_11.1,_11.1,_11.1];
_23 = &_3;
_17.2.0 = core::ptr::addr_of_mut!(_31.0);
_28.1 = &_30.0;
_21 = _35;
_12.0 = Move(_17.2.0);
_26 = _15;
_28.0 = _9;
_33 = &_3;
_17.2.0 = core::ptr::addr_of_mut!(_31.0);
_17.2.0 = Move(_12.0);
_3.fld1 = (_11.1,);
_30 = ((-5831124935857059172_i64),);
_23 = Move(_33);
_31.0 = &RET;
(*_8) = 1232830048_u32 as f32;
_37.fld1.0 = _11.1;
_2 = _14 + _14;
_7 = -_32;
(*_8) = _26 as f32;
_25 = !_26;
_23 = &_3;
_34 = Move(_23);
_12 = Move(_17.2);
RET = _2 as i32;
_28.1 = &_30.0;
match _30.0 {
0 => bb3,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463457543482495911152284 => bb16,
_ => bb15
}
}
bb11 = {
_8 = core::ptr::addr_of_mut!(_5);
_35 = !_21;
_28.0 = _9;
Call(_17.2.1 = fn13(Move(_31), (*_16), (*_16), Move(_3), _6, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_24 = [(*_23).fld1.0,(*_23).fld1.0,_11.1,_11.1,(*_23).fld1.0];
_5 = _1 as f32;
_7 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = _21 as i32;
_5 = _14 as f32;
_19 = [(-2599665853247152090_i64),226807418175619795_i64,(-2283111610738515464_i64)];
_11.0 = _21 as u8;
_22 = [_1,_1];
_20 = _7 & _7;
_22 = [_1,_1];
_24 = [(*_23).fld1.0,(*_23).fld1.0,_11.1,_11.1,_11.1];
_31.0 = &RET;
_20 = 3_usize as isize;
_12.1 = core::ptr::addr_of_mut!(_1);
_1 = _21 as u128;
_3.fld0 = Move(_8);
_32 = -_20;
_34 = Move(_23);
_21 = !false;
_17.2.1 = core::ptr::addr_of_mut!(_1);
Goto(bb9)
}
bb13 = {
RET = !822869950_i32;
_5 = RET as f32;
_2 = _14;
(*_16) = [(-2070220662682783929_i64),1559283596403787405_i64,3533570000258955617_i64];
_21 = !true;
RET = !1211274466_i32;
_12.1 = core::ptr::addr_of_mut!(_1);
_2 = _14;
_16 = core::ptr::addr_of_mut!(_19);
_11.1 = !_3.fld1.0;
_17.2.1 = Move(_12.1);
_15 = !(-112624913140343794304080108176330852363_i128);
_23 = &_3;
_11.0 = 69_u8 ^ 126_u8;
(*_16) = [3429120102456623519_i64,(-9039215114818059484_i64),621895646982843189_i64];
_14 = _2;
_4 = [41067_u16,54368_u16,48937_u16,28654_u16,9091_u16,53164_u16,5275_u16];
_11.0 = _7 as u8;
RET = -(-1513008807_i32);
Goto(bb8)
}
bb14 = {
_9 = '\u{a9c40}';
_7 = (-37_isize) >> _3.fld1.0;
_4 = [13015_u16,55863_u16,23598_u16,52082_u16,57820_u16,15713_u16,25615_u16];
RET = (-1869781402_i32);
_11.1 = (-11698_i16) as i8;
_14 = -_2;
RET = 907612047_i32 ^ 1165043639_i32;
_8 = Move(_3.fld0);
_11.0 = (-158943936553925571383554913356838621551_i128) as u8;
_9 = '\u{e2812}';
_17.0 = core::ptr::addr_of_mut!(_1);
_6 = [RET,RET,RET];
_18 = [3016959460753320939_u64,1520174864224113520_u64,14065120603206042843_u64,5070916250969308984_u64,14629405070595627169_u64,4714075757577935956_u64,16488197503705258171_u64];
_9 = '\u{108a97}';
_3.fld0 = core::ptr::addr_of_mut!(_5);
_18 = [18088045070692395666_u64,9994038844695075376_u64,8511930472834479836_u64,17287244853673080160_u64,6232720479963225168_u64,4948443181052272911_u64,14031405110883056888_u64];
RET = (-3327807990403150311_i64) as i32;
_1 = RET as u128;
_3.fld1.0 = _11.1 * _11.1;
_12.1 = core::ptr::addr_of_mut!(_1);
_18 = [12971074748748055932_u64,10062138778327939271_u64,17252646530921571405_u64,18102305539330209586_u64,12170186173836981332_u64,13418007488674896303_u64,15512191442484467636_u64];
RET = (-445023596_i32) * 222446614_i32;
_16 = core::ptr::addr_of_mut!(_19);
Goto(bb7)
}
bb15 = {
_7 = !9223372036854775807_isize;
_3.fld1 = ((-91_i8),);
RET = (-898223186_i32) ^ 2115618644_i32;
_4 = [32881_u16,61691_u16,13865_u16,26285_u16,65345_u16,14968_u16,567_u16];
RET = (-19186_i16) as i32;
_3.fld1 = (97_i8,);
match _3.fld1.0 {
97 => bb3,
_ => bb2
}
}
bb16 = {
_38 = core::ptr::addr_of_mut!(_2);
_37 = Adt38 { fld0: Move(_8),fld1: _3.fld1 };
_37.fld1.0 = _32 as i8;
_17.2 = (Move(_12.0), Move(_12.1));
_12.1 = Move(_17.2.1);
_16 = core::ptr::addr_of_mut!(_19);
(*_38) = _5 as f64;
_26 = _25 & _25;
_14 = (*_38) - _2;
_41 = _5;
_2 = -_14;
_17.1 = Move(_28.1);
_25 = -_26;
_3.fld1 = (_11.1,);
_28.0 = _9;
_30.0 = 6518_u16 as i64;
_3 = Move(_37);
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(12_usize, 30_usize, Move(_30), 15_usize, Move(_15), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(12_usize, 18_usize, Move(_18), 19_usize, Move(_19), 32_usize, Move(_32), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(12_usize, 24_usize, Move(_24), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: (&'static i32,),mut _2: [i64; 3],mut _3: [i64; 3],mut _4: Adt38,mut _5: [i32; 3],mut _6: char) -> *mut u128 {
mir! {
type RET = *mut u128;
let _7: [i64; 3];
let _8: &'static i32;
let _9: (u8, i8);
let _10: Adt51;
let _11: u32;
let _12: bool;
let _13: bool;
let _14: &'static i32;
let _15: &'static char;
let _16: &'static Adt38;
let _17: usize;
let _18: *mut f64;
let _19: f32;
let _20: [bool; 2];
let _21: u64;
let _22: u16;
let _23: f32;
let _24: ([i64; 1],);
let _25: char;
let _26: [i64; 1];
let _27: u32;
let _28: *mut Adt66;
let _29: bool;
let _30: i32;
let _31: *mut f64;
let _32: isize;
let _33: ([i64; 1],);
let _34: bool;
let _35: isize;
let _36: isize;
let _37: u32;
let _38: [u16; 7];
let _39: [i8; 8];
let _40: (*mut &'static i128, Adt38);
let _41: Adt46;
let _42: (i32,);
let _43: u128;
let _44: usize;
let _45: f64;
let _46: (f32, u8, [i8; 5]);
let _47: u32;
let _48: u8;
let _49: ();
let _50: ();
{
_3 = _2;
_3 = [2707833025806182481_i64,4987383325713529902_i64,1520832345424688108_i64];
_4.fld1 = (71_i8,);
_6 = '\u{8d871}';
_6 = '\u{1c87c}';
_3 = [2619388578618626516_i64,3593424994584843613_i64,(-2882703335536324536_i64)];
_6 = '\u{3d1e2}';
_2 = [(-7910219058293602146_i64),1127655445962773197_i64,(-2782814702278576558_i64)];
_6 = '\u{a65de}';
_5 = [(-429858824_i32),162209366_i32,969810965_i32];
_3 = _2;
_7 = [(-1877129794717365138_i64),(-2042661579522477353_i64),(-9086429023478169068_i64)];
_3 = [2925947949456765791_i64,(-6101099683989273530_i64),2648783179583240818_i64];
_7 = _3;
_4.fld1 = ((-30_i8),);
Goto(bb1)
}
bb1 = {
_4.fld1.0 = 9_i8 + 61_i8;
_6 = '\u{a9542}';
_5 = [(-59214120_i32),1069752780_i32,1342759922_i32];
Goto(bb2)
}
bb2 = {
_5 = [733861552_i32,54318381_i32,(-890308839_i32)];
_4.fld1 = ((-120_i8),);
_3 = [(-6382029869345298402_i64),7994084670732644277_i64,5034497344848072394_i64];
_7 = [(-6830496933864170625_i64),6648897850554749962_i64,5830295054561364007_i64];
_6 = '\u{4db4a}';
_7 = [(-7929180223504919246_i64),788301293587550986_i64,3786382069889740919_i64];
_4.fld1.0 = (-90_i8);
_4.fld1 = ((-121_i8),);
_4.fld1.0 = 105_i8 + 36_i8;
_2 = [8102410228365779951_i64,(-3463637893969976019_i64),924441847797995247_i64];
_1.0 = &_10.fld5;
_8 = &_10.fld5;
_3 = [(-409727915593593239_i64),1078869276464242134_i64,6028666311481453047_i64];
_4.fld1 = (91_i8,);
_4.fld0 = core::ptr::addr_of_mut!(_10.fld3);
_12 = _4.fld1.0 > _4.fld1.0;
_11 = 63310_u16 as u32;
_7 = [1464156467770579506_i64,297814311553555658_i64,4016100123769867100_i64];
_11 = 9223372036854775807_isize as u32;
_10.fld3 = 1697950149684461282_u64 as f32;
_9.1 = 5023210444812251785645085271170188652_u128 as i8;
_4.fld0 = core::ptr::addr_of_mut!(_10.fld3);
_10.fld3 = _4.fld1.0 as f32;
_6 = '\u{82be4}';
_5 = [(-1595710324_i32),1216326978_i32,(-1919700163_i32)];
Goto(bb3)
}
bb3 = {
_10.fld2 = [(-1023520431913672825_i64)];
_2 = [(-5783426760267796000_i64),4495239390265215740_i64,(-48873143432373893_i64)];
_4.fld1 = (_9.1,);
_13 = _12;
_1.0 = &(*_8);
_14 = &(*_8);
_11 = 2596374114_u32 | 1500373337_u32;
_10.fld0 = Adt36::Variant1 { fld0: 194_u8 };
_9 = (198_u8, _4.fld1.0);
_19 = 7466112873977776987_i64 as f32;
_10.fld4 = 1645835656646220571_i64;
_10.fld5 = -(-935603791_i32);
_14 = &_10.fld5;
_20 = [_12,_13];
_17 = 6747682347823440171_usize + 7926253943265873574_usize;
_9 = (207_u8, _4.fld1.0);
_4.fld0 = core::ptr::addr_of_mut!(_10.fld3);
_17 = _9.1 as usize;
_16 = &_4;
_7 = [_10.fld4,_10.fld4,_10.fld4];
_4.fld1.0 = -_9.1;
_9 = (125_u8, _4.fld1.0);
_13 = _12 | _12;
Goto(bb4)
}
bb4 = {
_21 = 10346532326764012152_u64 ^ 7183273908658580948_u64;
_4.fld1.0 = _9.1 ^ _9.1;
_2 = [_10.fld4,_10.fld4,_10.fld4];
_4.fld0 = core::ptr::addr_of_mut!(_10.fld3);
_16 = &_4;
_14 = &_10.fld5;
_4.fld0 = core::ptr::addr_of_mut!(_19);
Goto(bb5)
}
bb5 = {
_14 = &(*_14);
_23 = (*_16).fld1.0 as f32;
_1.0 = &(*_14);
Call(_24 = fn14(Move(_16), Move(_1), _20, _9.0, _9.0, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10.fld3 = 18561_i16 as f32;
_10.fld4 = -4565416515882947030_i64;
_2 = _3;
_10.fld2 = [_10.fld4];
_14 = &_10.fld5;
_15 = &_25;
_22 = 36964_u16 << (*_14);
_4.fld0 = core::ptr::addr_of_mut!(_19);
_26 = [_10.fld4];
_6 = '\u{39e06}';
_10.fld1 = core::ptr::addr_of!(_21);
_9.0 = !135_u8;
_4.fld1.0 = -_9.1;
_8 = &_10.fld5;
_6 = '\u{f6490}';
_1 = (Move(_8),);
_19 = -_23;
_14 = &(*_14);
_10.fld1 = core::ptr::addr_of!(_21);
_19 = _10.fld3 - _10.fld3;
_19 = _17 as f32;
_14 = &(*_14);
_14 = &_10.fld5;
_9.1 = _10.fld4 as i8;
_10.fld4 = (-6963427485428065_i64);
_2 = [_10.fld4,_10.fld4,_10.fld4];
_8 = &_10.fld5;
_7 = _3;
Call(_2 = fn15(Move(_8), _9, Move(_10.fld1), _11, _4.fld1, Move(_4.fld0), _7, _9.0, _12, _21, _26), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = 3998442914_u32 + 1121551296_u32;
_16 = &_4;
_4.fld1.0 = _9.1;
_29 = _13 | _13;
_14 = &_10.fld5;
_26 = [_10.fld4];
_14 = &(*_14);
_8 = &(*_14);
_26 = [_10.fld4];
_1.0 = &(*_8);
_10.fld1 = core::ptr::addr_of!(_21);
_27 = _11;
_30 = (-30976_i16) as i32;
_4.fld1.0 = !_9.1;
place!(Field::<u8>(Variant(_10.fld0, 1), 0)) = _9.0 + _9.0;
_25 = _6;
_4.fld1 = (_9.1,);
_1.0 = &(*_14);
_19 = 9223372036854775807_isize as f32;
_25 = _6;
match _10.fld4 {
0 => bb5,
1 => bb3,
2 => bb8,
340282366920938463463367644004282783391 => bb10,
_ => bb9
}
}
bb8 = {
_10.fld3 = 18561_i16 as f32;
_10.fld4 = -4565416515882947030_i64;
_2 = _3;
_10.fld2 = [_10.fld4];
_14 = &_10.fld5;
_15 = &_25;
_22 = 36964_u16 << (*_14);
_4.fld0 = core::ptr::addr_of_mut!(_19);
_26 = [_10.fld4];
_6 = '\u{39e06}';
_10.fld1 = core::ptr::addr_of!(_21);
_9.0 = !135_u8;
_4.fld1.0 = -_9.1;
_8 = &_10.fld5;
_6 = '\u{f6490}';
_1 = (Move(_8),);
_19 = -_23;
_14 = &(*_14);
_10.fld1 = core::ptr::addr_of!(_21);
_19 = _10.fld3 - _10.fld3;
_19 = _17 as f32;
_14 = &(*_14);
_14 = &_10.fld5;
_9.1 = _10.fld4 as i8;
_10.fld4 = (-6963427485428065_i64);
_2 = [_10.fld4,_10.fld4,_10.fld4];
_8 = &_10.fld5;
_7 = _3;
Call(_2 = fn15(Move(_8), _9, Move(_10.fld1), _11, _4.fld1, Move(_4.fld0), _7, _9.0, _12, _21, _26), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_14 = &(*_14);
_23 = (*_16).fld1.0 as f32;
_1.0 = &(*_14);
Call(_24 = fn14(Move(_16), Move(_1), _20, _9.0, _9.0, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_14 = &_10.fld5;
match _10.fld4 {
340282366920938463463367644004282783391 => bb12,
_ => bb11
}
}
bb11 = {
_4.fld1.0 = 9_i8 + 61_i8;
_6 = '\u{a9542}';
_5 = [(-59214120_i32),1069752780_i32,1342759922_i32];
Goto(bb2)
}
bb12 = {
_5 = [(*_8),(*_14),(*_8)];
_15 = &_6;
match _10.fld4 {
0 => bb6,
1 => bb2,
2 => bb3,
340282366920938463463367644004282783391 => bb13,
_ => bb5
}
}
bb13 = {
_9 = (Field::<u8>(Variant(_10.fld0, 1), 0), _4.fld1.0);
_20 = [_29,_12];
_10.fld3 = (*_8) as f32;
_24 = (_26,);
_2 = [_10.fld4,_10.fld4,_10.fld4];
_34 = _29;
_1 = (Move(_8),);
_14 = &(*_14);
_10.fld5 = -_30;
_10.fld1 = core::ptr::addr_of!(_21);
_35 = -(-9223372036854775808_isize);
_14 = &_30;
_4.fld0 = core::ptr::addr_of_mut!(_19);
_14 = &(*_14);
_36 = _35 << _21;
_15 = &(*_15);
_9.0 = Field::<u8>(Variant(_10.fld0, 1), 0) * Field::<u8>(Variant(_10.fld0, 1), 0);
_5 = [(*_14),_10.fld5,_10.fld5];
_10.fld3 = _23 * _23;
_5 = [(*_14),(*_14),_30];
_30 = _10.fld5 - _10.fld5;
_10.fld1 = core::ptr::addr_of!(_21);
_33.0 = _24.0;
SetDiscriminant(_10.fld0, 2);
_24.0 = [_10.fld4];
match _10.fld4 {
0 => bb5,
1 => bb3,
2 => bb14,
3 => bb15,
340282366920938463463367644004282783391 => bb17,
_ => bb16
}
}
bb14 = {
_11 = 3998442914_u32 + 1121551296_u32;
_16 = &_4;
_4.fld1.0 = _9.1;
_29 = _13 | _13;
_14 = &_10.fld5;
_26 = [_10.fld4];
_14 = &(*_14);
_8 = &(*_14);
_26 = [_10.fld4];
_1.0 = &(*_8);
_10.fld1 = core::ptr::addr_of!(_21);
_27 = _11;
_30 = (-30976_i16) as i32;
_4.fld1.0 = !_9.1;
place!(Field::<u8>(Variant(_10.fld0, 1), 0)) = _9.0 + _9.0;
_25 = _6;
_4.fld1 = (_9.1,);
_1.0 = &(*_14);
_19 = 9223372036854775807_isize as f32;
_25 = _6;
match _10.fld4 {
0 => bb5,
1 => bb3,
2 => bb8,
340282366920938463463367644004282783391 => bb10,
_ => bb9
}
}
bb15 = {
_10.fld3 = 18561_i16 as f32;
_10.fld4 = -4565416515882947030_i64;
_2 = _3;
_10.fld2 = [_10.fld4];
_14 = &_10.fld5;
_15 = &_25;
_22 = 36964_u16 << (*_14);
_4.fld0 = core::ptr::addr_of_mut!(_19);
_26 = [_10.fld4];
_6 = '\u{39e06}';
_10.fld1 = core::ptr::addr_of!(_21);
_9.0 = !135_u8;
_4.fld1.0 = -_9.1;
_8 = &_10.fld5;
_6 = '\u{f6490}';
_1 = (Move(_8),);
_19 = -_23;
_14 = &(*_14);
_10.fld1 = core::ptr::addr_of!(_21);
_19 = _10.fld3 - _10.fld3;
_19 = _17 as f32;
_14 = &(*_14);
_14 = &_10.fld5;
_9.1 = _10.fld4 as i8;
_10.fld4 = (-6963427485428065_i64);
_2 = [_10.fld4,_10.fld4,_10.fld4];
_8 = &_10.fld5;
_7 = _3;
Call(_2 = fn15(Move(_8), _9, Move(_10.fld1), _11, _4.fld1, Move(_4.fld0), _7, _9.0, _12, _21, _26), ReturnTo(bb7), UnwindUnreachable())
}
bb16 = {
_10.fld3 = 18561_i16 as f32;
_10.fld4 = -4565416515882947030_i64;
_2 = _3;
_10.fld2 = [_10.fld4];
_14 = &_10.fld5;
_15 = &_25;
_22 = 36964_u16 << (*_14);
_4.fld0 = core::ptr::addr_of_mut!(_19);
_26 = [_10.fld4];
_6 = '\u{39e06}';
_10.fld1 = core::ptr::addr_of!(_21);
_9.0 = !135_u8;
_4.fld1.0 = -_9.1;
_8 = &_10.fld5;
_6 = '\u{f6490}';
_1 = (Move(_8),);
_19 = -_23;
_14 = &(*_14);
_10.fld1 = core::ptr::addr_of!(_21);
_19 = _10.fld3 - _10.fld3;
_19 = _17 as f32;
_14 = &(*_14);
_14 = &_10.fld5;
_9.1 = _10.fld4 as i8;
_10.fld4 = (-6963427485428065_i64);
_2 = [_10.fld4,_10.fld4,_10.fld4];
_8 = &_10.fld5;
_7 = _3;
Call(_2 = fn15(Move(_8), _9, Move(_10.fld1), _11, _4.fld1, Move(_4.fld0), _7, _9.0, _12, _21, _26), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
_13 = !_29;
_27 = _17 as u32;
_10.fld2 = [_10.fld4];
_26 = [_10.fld4];
_4.fld1 = (_9.1,);
_17 = _4.fld1.0 as usize;
_35 = 46687620973363815553182449013645366123_i128 as isize;
_37 = _11 & _11;
_38 = [_22,_22,_22,_22,_22,_22,_22];
place!(Field::<(*const char, i8)>(Variant(_10.fld0, 2), 2)).1 = _21 as i8;
place!(Field::<u64>(Variant(_10.fld0, 2), 1)) = !_21;
_26 = _24.0;
place!(Field::<u64>(Variant(_10.fld0, 2), 1)) = !_21;
_32 = _10.fld4 as isize;
_40.1.fld0 = Move(_4.fld0);
_30 = _10.fld4 as i32;
_40.1.fld1 = (_4.fld1.0,);
_24.0 = [_10.fld4];
_8 = &_30;
_10.fld2 = [_10.fld4];
_1 = (Move(_8),);
_40.1.fld1.0 = Field::<(*const char, i8)>(Variant(_10.fld0, 2), 2).1;
Call(_9.0 = core::intrinsics::bswap(16_u8), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
place!(Field::<(*const char, i8)>(Variant(_10.fld0, 2), 2)).1 = !_9.1;
place!(Field::<(*const char, i8)>(Variant(_10.fld0, 2), 2)).0 = core::ptr::addr_of!(_25);
_39 = [_9.1,_40.1.fld1.0,Field::<(*const char, i8)>(Variant(_10.fld0, 2), 2).1,_40.1.fld1.0,_4.fld1.0,_40.1.fld1.0,_4.fld1.0,_4.fld1.0];
_20 = [_34,_34];
_10.fld2 = [_10.fld4];
_41 = Adt46 { fld0: _10.fld3,fld1: Move(_40.1.fld0),fld2: _33,fld3: _5 };
_9.0 = 240_u8;
_33 = _24;
_3 = [_10.fld4,_10.fld4,_10.fld4];
_19 = -_23;
_4.fld0 = Move(_41.fld1);
place!(Field::<bool>(Variant(_10.fld0, 2), 0)) = _36 != _36;
_41.fld2 = _24;
_2 = [_10.fld4,_10.fld4,_10.fld4];
place!(Field::<u64>(Variant(_10.fld0, 2), 1)) = _30 as u64;
_12 = !_13;
_9.1 = _21 as i8;
_40.1.fld1.0 = _9.1 - _9.1;
_22 = 18637_u16;
_42 = (_10.fld5,);
place!(Field::<bool>(Variant(_10.fld0, 2), 0)) = _29;
_5 = [_30,_30,_42.0];
_16 = &_40.1;
place!(Field::<bool>(Variant(_10.fld0, 2), 0)) = !_34;
_40.1.fld0 = Move(_4.fld0);
_10.fld3 = _41.fld0;
Goto(bb19)
}
bb19 = {
_10.fld2 = [_10.fld4];
_31 = core::ptr::addr_of_mut!(_45);
_18 = core::ptr::addr_of_mut!((*_31));
_3 = [_10.fld4,_10.fld4,_10.fld4];
_27 = _11 + _37;
_43 = !151492433682484552936176675234139204830_u128;
match _22 {
0 => bb12,
1 => bb13,
2 => bb3,
3 => bb10,
4 => bb20,
18637 => bb22,
_ => bb21
}
}
bb20 = {
_14 = &(*_14);
_23 = (*_16).fld1.0 as f32;
_1.0 = &(*_14);
Call(_24 = fn14(Move(_16), Move(_1), _20, _9.0, _9.0, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb21 = {
_9 = (Field::<u8>(Variant(_10.fld0, 1), 0), _4.fld1.0);
_20 = [_29,_12];
_10.fld3 = (*_8) as f32;
_24 = (_26,);
_2 = [_10.fld4,_10.fld4,_10.fld4];
_34 = _29;
_1 = (Move(_8),);
_14 = &(*_14);
_10.fld5 = -_30;
_10.fld1 = core::ptr::addr_of!(_21);
_35 = -(-9223372036854775808_isize);
_14 = &_30;
_4.fld0 = core::ptr::addr_of_mut!(_19);
_14 = &(*_14);
_36 = _35 << _21;
_15 = &(*_15);
_9.0 = Field::<u8>(Variant(_10.fld0, 1), 0) * Field::<u8>(Variant(_10.fld0, 1), 0);
_5 = [(*_14),_10.fld5,_10.fld5];
_10.fld3 = _23 * _23;
_5 = [(*_14),(*_14),_30];
_30 = _10.fld5 - _10.fld5;
_10.fld1 = core::ptr::addr_of!(_21);
_33.0 = _24.0;
SetDiscriminant(_10.fld0, 2);
_24.0 = [_10.fld4];
match _10.fld4 {
0 => bb5,
1 => bb3,
2 => bb14,
3 => bb15,
340282366920938463463367644004282783391 => bb17,
_ => bb16
}
}
bb22 = {
RET = core::ptr::addr_of_mut!(_43);
_25 = (*_15);
_41.fld0 = _19;
place!(Field::<u16>(Variant(_10.fld0, 2), 3)) = _22;
_10.fld5 = _42.0;
_46.2 = [Field::<(*const char, i8)>(Variant(_10.fld0, 2), 2).1,_9.1,(*_16).fld1.0,(*_16).fld1.0,_40.1.fld1.0];
SetDiscriminant(_10.fld0, 0);
_26 = [_10.fld4];
_41.fld0 = -_19;
_41.fld2.0 = [_10.fld4];
place!(Field::<(*const char, i8)>(Variant(_10.fld0, 0), 0)).1 = _17 as i8;
(*_18) = _21 as f64;
_8 = &_10.fld5;
_40.1.fld1 = _4.fld1;
_41.fld1 = Move(_40.1.fld0);
_45 = _22 as f64;
_10.fld4 = _22 as i64;
_40.1 = Adt38 { fld0: Move(_41.fld1),fld1: _4.fld1 };
_4.fld0 = Move(_40.1.fld0);
Goto(bb23)
}
bb23 = {
Call(_49 = dump_var(13_usize, 13_usize, Move(_13), 33_usize, Move(_33), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_49 = dump_var(13_usize, 38_usize, Move(_38), 21_usize, Move(_21), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_49 = dump_var(13_usize, 35_usize, Move(_35), 20_usize, Move(_20), 36_usize, Move(_36), 22_usize, Move(_22)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_49 = dump_var(13_usize, 42_usize, Move(_42), 32_usize, Move(_32), 50_usize, _50, 50_usize, _50), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static Adt38,mut _2: (&'static i32,),mut _3: [bool; 2],mut _4: u8,mut _5: u8,mut _6: char) -> ([i64; 1],) {
mir! {
type RET = ([i64; 1],);
let _7: isize;
let _8: *mut u128;
let _9: Adt79;
let _10: [i8; 5];
let _11: f32;
let _12: [u64; 7];
let _13: bool;
let _14: [i8; 8];
let _15: [i64; 1];
let _16: u32;
let _17: f32;
let _18: [i8; 5];
let _19: isize;
let _20: isize;
let _21: f32;
let _22: f32;
let _23: (*const char, i8);
let _24: *mut *const *mut &'static i128;
let _25: u32;
let _26: (i32,);
let _27: i32;
let _28: &'static Adt38;
let _29: &'static (*mut u128, &'static i64, (*mut &'static i32, *mut u128));
let _30: ([i64; 1],);
let _31: &'static (*mut u128, &'static i64, (*mut &'static i32, *mut u128));
let _32: *const u64;
let _33: i16;
let _34: &'static *mut u128;
let _35: char;
let _36: ((i8,), Adt51, *mut u128, char);
let _37: [u16; 7];
let _38: ();
let _39: ();
{
RET.0 = [(-8884528900131412162_i64)];
_3 = [false,false];
_6 = '\u{373d3}';
_6 = '\u{109237}';
_6 = '\u{72e0}';
_5 = _4 % _4;
_6 = '\u{830b7}';
RET.0 = [(-6032597604740319826_i64)];
_3 = [false,true];
_5 = _4 % _4;
RET.0 = [(-6728344971384797485_i64)];
_6 = '\u{3f7b4}';
_3 = [false,true];
RET.0 = [(-2794189677812315710_i64)];
_6 = '\u{9dfcf}';
RET.0 = [(-8189775133927221719_i64)];
_6 = '\u{1946}';
RET.0 = [2097607141718329240_i64];
RET.0 = [6768622961084364872_i64];
RET.0 = [(-3474475582025126879_i64)];
Goto(bb1)
}
bb1 = {
_7 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_3 = [true,true];
_4 = _7 as u8;
_7 = 22357_i16 as isize;
_5 = _4 << _4;
_3 = [true,true];
_10 = [(-68_i8),24_i8,(-21_i8),32_i8,(-55_i8)];
RET.0 = [(-277161093655471175_i64)];
_4 = _5 ^ _5;
_10 = [45_i8,12_i8,26_i8,96_i8,69_i8];
_5 = _4 + _4;
_11 = 339450811509085186271056180898759381358_u128 as f32;
_3 = [true,true];
_13 = !false;
_12 = [8400681536462996363_u64,5165057817668477038_u64,17861790945932141897_u64,7441529576843582643_u64,14136324379876201373_u64,13948690696379476574_u64,1367072745064404630_u64];
RET.0 = [(-3206064496927694509_i64)];
Goto(bb2)
}
bb2 = {
_16 = (-8107_i16) as u32;
_15 = RET.0;
_3 = [_13,_13];
_5 = !_4;
_13 = _4 == _5;
_11 = _4 as f32;
_17 = -_11;
_13 = false;
_7 = 9223372036854775807_isize;
_14 = [(-46_i8),103_i8,61_i8,73_i8,(-68_i8),84_i8,(-14_i8),(-18_i8)];
_16 = 393990654_u32;
_15 = RET.0;
_15 = [6963949788145180114_i64];
_18 = [108_i8,42_i8,96_i8,8_i8,95_i8];
match _16 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
393990654 => bb7,
_ => bb6
}
}
bb3 = {
_7 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_3 = [true,true];
_4 = _7 as u8;
_7 = 22357_i16 as isize;
_5 = _4 << _4;
_3 = [true,true];
_10 = [(-68_i8),24_i8,(-21_i8),32_i8,(-55_i8)];
RET.0 = [(-277161093655471175_i64)];
_4 = _5 ^ _5;
_10 = [45_i8,12_i8,26_i8,96_i8,69_i8];
_5 = _4 + _4;
_11 = 339450811509085186271056180898759381358_u128 as f32;
_3 = [true,true];
_13 = !false;
_12 = [8400681536462996363_u64,5165057817668477038_u64,17861790945932141897_u64,7441529576843582643_u64,14136324379876201373_u64,13948690696379476574_u64,1367072745064404630_u64];
RET.0 = [(-3206064496927694509_i64)];
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
_5 = _13 as u8;
_4 = _5 ^ _5;
_12 = [1836901142461235460_u64,5782887490973236116_u64,9870873473840919686_u64,14594390361545790608_u64,7510833414940831109_u64,8951754101697668953_u64,3857989605624274589_u64];
_18 = _10;
_11 = 78_i8 as f32;
_15 = [(-5350892551688172988_i64)];
_12 = [16487700059604133226_u64,10707476391697575255_u64,5854076543774037549_u64,8086332666963892714_u64,455212810491651612_u64,1268748873848856386_u64,9214605426893878217_u64];
_16 = !3710708297_u32;
_17 = _11;
_13 = true;
RET.0 = _15;
_5 = _4;
_12 = [17775109914470549917_u64,2411017462155426873_u64,2135632713225728551_u64,3592920120129321406_u64,3369987540957932074_u64,14947688366521760599_u64,14598860866531060248_u64];
_20 = _7 & _7;
_7 = _20 >> _5;
_3 = [_13,_13];
RET.0 = [5033345966926299131_i64];
_4 = !_5;
Goto(bb8)
}
bb8 = {
_11 = -_17;
RET.0 = [(-3248011425008835131_i64)];
_20 = -_7;
_12 = [4779670621002100321_u64,4510698406977136233_u64,2137600250618414254_u64,18016394658007810781_u64,9791511388753654850_u64,6158338892547408964_u64,7421541474333104299_u64];
_21 = _5 as f32;
_15 = [8865556734284523841_i64];
_23.1 = -(-55_i8);
_10 = _18;
_10 = [_23.1,_23.1,_23.1,_23.1,_23.1];
_10 = [_23.1,_23.1,_23.1,_23.1,_23.1];
_6 = '\u{4b7a5}';
_23.0 = core::ptr::addr_of!(_6);
_14 = [_23.1,_23.1,_23.1,_23.1,_23.1,_23.1,_23.1,_23.1];
_15 = [6007322196955068183_i64];
_19 = 268866145582388295238009843738627628505_u128 as isize;
_22 = -_11;
_17 = _21;
_16 = 510406434_u32 * 3180586643_u32;
_3 = [_13,_13];
_17 = _11 - _11;
_20 = -_7;
RET.0 = [(-7863973213854624609_i64)];
RET.0 = [546594111126630533_i64];
_15 = [(-2043846229345475333_i64)];
_17 = _22;
_5 = _21 as u8;
Goto(bb9)
}
bb9 = {
_18 = [_23.1,_23.1,_23.1,_23.1,_23.1];
RET.0 = _15;
_4 = _5;
_16 = _17 as u32;
_12 = [6376134555037954575_u64,1526876988945400143_u64,16765969909006149082_u64,16059939188331574360_u64,5260686890740521501_u64,9013257654521295097_u64,8307736961608743072_u64];
RET.0 = [109390324483300676_i64];
_13 = !false;
_17 = -_21;
_19 = _20;
_13 = !true;
_23.1 = _4 as i8;
RET.0 = _15;
RET.0 = [1399433665250682863_i64];
_11 = -_21;
_14 = [_23.1,_23.1,_23.1,_23.1,_23.1,_23.1,_23.1,_23.1];
_23.1 = 5093_u16 as i8;
_5 = (-432084905022045988_i64) as u8;
_11 = _21;
RET.0 = _15;
_3 = [_13,_13];
_3 = [_13,_13];
RET = (_15,);
_14 = [_23.1,_23.1,_23.1,_23.1,_23.1,_23.1,_23.1,_23.1];
Goto(bb10)
}
bb10 = {
_6 = '\u{f7576}';
_26 = (2090646989_i32,);
_12 = [14196416912159906540_u64,15912110224378204136_u64,11926904736283915727_u64,11111009377464121870_u64,104786363866073312_u64,14516796812273431807_u64,7435090024713246495_u64];
_4 = _6 as u8;
_3 = [_13,_13];
_17 = 7770178209234853498_usize as f32;
_2.0 = &_26.0;
_22 = _11;
_2.0 = &_26.0;
_25 = _16 + _16;
_7 = -_19;
_16 = _25 << _7;
_4 = _5;
_7 = _20 ^ _20;
_5 = 35984_u16 as u8;
_13 = !false;
Goto(bb11)
}
bb11 = {
RET.0 = [4579992230931206668_i64];
RET = (_15,);
_23.1 = 12_i8 + (-65_i8);
_11 = _21 - _22;
_11 = _21 + _17;
_26.0 = (-1961360003_i32);
_12 = [1437582803952035552_u64,3531774267600485115_u64,16594350892603087352_u64,4573217075010876321_u64,605974465428958730_u64,5599357661164621988_u64,13421053833603206527_u64];
_15 = [(-4538837829062036043_i64)];
_15 = [5259340908840281843_i64];
_17 = 13083028131243208795_usize as f32;
_10 = _18;
_33 = !(-17646_i16);
_22 = -_21;
RET.0 = [8829002007329078278_i64];
RET = (_15,);
_30.0 = [(-7890221275736134080_i64)];
_13 = _17 >= _21;
_19 = _33 as isize;
RET = _30;
_12 = [15173012432065289889_u64,12071791311097097962_u64,3687708928318014131_u64,8561745396357644814_u64,8757908302012930124_u64,10028709047210362351_u64,986112492774437164_u64];
_30 = RET;
match _26.0 {
0 => bb1,
1 => bb5,
2 => bb12,
340282366920938463463374607429806851453 => bb14,
_ => bb13
}
}
bb12 = {
_16 = (-8107_i16) as u32;
_15 = RET.0;
_3 = [_13,_13];
_5 = !_4;
_13 = _4 == _5;
_11 = _4 as f32;
_17 = -_11;
_13 = false;
_7 = 9223372036854775807_isize;
_14 = [(-46_i8),103_i8,61_i8,73_i8,(-68_i8),84_i8,(-14_i8),(-18_i8)];
_16 = 393990654_u32;
_15 = RET.0;
_15 = [6963949788145180114_i64];
_18 = [108_i8,42_i8,96_i8,8_i8,95_i8];
match _16 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
393990654 => bb7,
_ => bb6
}
}
bb13 = {
_5 = _13 as u8;
_4 = _5 ^ _5;
_12 = [1836901142461235460_u64,5782887490973236116_u64,9870873473840919686_u64,14594390361545790608_u64,7510833414940831109_u64,8951754101697668953_u64,3857989605624274589_u64];
_18 = _10;
_11 = 78_i8 as f32;
_15 = [(-5350892551688172988_i64)];
_12 = [16487700059604133226_u64,10707476391697575255_u64,5854076543774037549_u64,8086332666963892714_u64,455212810491651612_u64,1268748873848856386_u64,9214605426893878217_u64];
_16 = !3710708297_u32;
_17 = _11;
_13 = true;
RET.0 = _15;
_5 = _4;
_12 = [17775109914470549917_u64,2411017462155426873_u64,2135632713225728551_u64,3592920120129321406_u64,3369987540957932074_u64,14947688366521760599_u64,14598860866531060248_u64];
_20 = _7 & _7;
_7 = _20 >> _5;
_3 = [_13,_13];
RET.0 = [5033345966926299131_i64];
_4 = !_5;
Goto(bb8)
}
bb14 = {
_12 = [7237572348390351594_u64,27004482103471495_u64,2464127914404520970_u64,3837498670447395740_u64,15519784670015003299_u64,1389320782555984992_u64,13626638721054077360_u64];
_6 = '\u{10d6ee}';
_7 = _6 as isize;
_27 = -_26.0;
_16 = _25;
_3 = [_13,_13];
_16 = _25;
_13 = !false;
_19 = _7 + _20;
_34 = &_8;
_35 = _6;
_25 = _16;
_7 = _20 << _23.1;
_17 = _22 + _21;
_13 = false;
_26.0 = 15918832685505352000_u64 as i32;
_36.1.fld4 = (-643765754381955300_i64) & 713163745340803648_i64;
_36.0.0 = _35 as i8;
RET.0 = [_36.1.fld4];
_36.1.fld2 = RET.0;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(14_usize, 15_usize, Move(_15), 6_usize, Move(_6), 30_usize, Move(_30), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(14_usize, 35_usize, Move(_35), 13_usize, Move(_13), 20_usize, Move(_20), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(14_usize, 33_usize, Move(_33), 18_usize, Move(_18), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: &'static i32,mut _2: (u8, i8),mut _3: *const u64,mut _4: u32,mut _5: (i8,),mut _6: *mut f32,mut _7: [i64; 3],mut _8: u8,mut _9: bool,mut _10: u64,mut _11: [i64; 1]) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _12: (u8, i8);
let _13: [i64; 3];
let _14: i32;
let _15: usize;
let _16: [bool; 2];
let _17: ([i64; 1],);
let _18: u32;
let _19: isize;
let _20: ((i8,), Adt51, *mut u128, char);
let _21: isize;
let _22: [i64; 1];
let _23: u128;
let _24: *mut Adt51;
let _25: u64;
let _26: i32;
let _27: isize;
let _28: isize;
let _29: f64;
let _30: u8;
let _31: *const u64;
let _32: i32;
let _33: isize;
let _34: &'static char;
let _35: bool;
let _36: Adt36;
let _37: ();
let _38: ();
{
RET = [281634581281750773_i64,4475707665529616912_i64,(-5758326905559943000_i64)];
_5.0 = -_2.1;
_5.0 = -_2.1;
RET = _7;
_9 = !false;
_5 = (_2.1,);
_12.0 = (-9223372036854775808_isize) as u8;
Goto(bb1)
}
bb1 = {
_12.0 = _2.0;
_12.0 = 46242225285736657485054390353339936924_u128 as u8;
_7 = RET;
_3 = core::ptr::addr_of!(_10);
_12.1 = !_2.1;
_12.0 = 13458061267320398777349881852656733327_i128 as u8;
_12.0 = _8;
_4 = 5516_u16 as u32;
_2.0 = !_12.0;
_5 = (_12.1,);
_7 = RET;
Goto(bb2)
}
bb2 = {
_10 = 7550354281498482644_u64;
_4 = 2839582556_u32;
_4 = 2673347827_u32 * 965537431_u32;
_10 = 15338197558439297152_u64 >> _5.0;
_11 = [(-8379270102872386652_i64)];
_12.1 = _2.1;
_7 = RET;
RET = _7;
_4 = 3883638304_u32;
_14 = 2013537065_i32;
_3 = core::ptr::addr_of!(_10);
_5.0 = _2.1 & _12.1;
_8 = _12.0 << _4;
_12.0 = _8 >> _12.1;
(*_3) = 17354831099251749239_u64 >> _2.0;
_15 = 2412971705067430708_usize;
match _4 {
0 => bb1,
3883638304 => bb4,
_ => bb3
}
}
bb3 = {
_12.0 = _2.0;
_12.0 = 46242225285736657485054390353339936924_u128 as u8;
_7 = RET;
_3 = core::ptr::addr_of!(_10);
_12.1 = !_2.1;
_12.0 = 13458061267320398777349881852656733327_i128 as u8;
_12.0 = _8;
_4 = 5516_u16 as u32;
_2.0 = !_12.0;
_5 = (_12.1,);
_7 = RET;
Goto(bb2)
}
bb4 = {
_10 = !11056722453500219387_u64;
_12 = (_8, _5.0);
_13 = [(-5722115809789290171_i64),1823466129922116292_i64,(-715106412257690115_i64)];
_14 = (-1084467005_i32) * 2061583931_i32;
_8 = _12.0;
_17 = (_11,);
_4 = !3060237798_u32;
_5 = (_12.1,);
_18 = _4;
_18 = _4;
_12 = (_2.0, _5.0);
_11 = _17.0;
_19 = !9223372036854775807_isize;
(*_3) = 2596975389149162826_u64 << _8;
_16 = [_9,_9];
_2 = _12;
_15 = !2_usize;
Goto(bb5)
}
bb5 = {
_13 = RET;
Call(_20.0.0 = core::intrinsics::transmute(_5.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_9 = !true;
_8 = !_12.0;
_6 = core::ptr::addr_of_mut!(_20.1.fld3);
_20.1.fld0 = Adt36::Variant1 { fld0: _8 };
(*_6) = (*_3) as f32;
(*_3) = 13954635098014441860_u64 & 12965449742613933489_u64;
_20.1.fld3 = _18 as f32;
(*_6) = _15 as f32;
Goto(bb7)
}
bb7 = {
_20.1.fld2 = [(-270596061642920685_i64)];
_13 = [1134233808629437209_i64,4943854309907084878_i64,5103502913395920083_i64];
(*_6) = (-9449_i16) as f32;
_17.0 = _20.1.fld2;
_13 = _7;
_20.0.0 = _4 as i8;
_5 = (_12.1,);
RET = _13;
_20.1.fld2 = [4361665918543634592_i64];
_22 = [(-4068196750446325161_i64)];
_20.3 = '\u{5f640}';
_23 = 26646480224277062545876947622154662307_u128;
_7 = RET;
_21 = _8 as isize;
_20.1.fld5 = _14;
_12.0 = !Field::<u8>(Variant(_20.1.fld0, 1), 0);
_20.2 = core::ptr::addr_of_mut!(_23);
_22 = [(-5536434274790857837_i64)];
_20.1.fld2 = _22;
_20.0 = (_12.1,);
_20.1.fld4 = !3402582261417284975_i64;
_9 = !true;
_3 = core::ptr::addr_of!((*_3));
Goto(bb8)
}
bb8 = {
_4 = !_18;
_9 = false | false;
_24 = core::ptr::addr_of_mut!(_20.1);
SetDiscriminant((*_24).fld0, 0);
_20.2 = core::ptr::addr_of_mut!(_23);
_20.0 = (_5.0,);
_28 = _19;
_2.1 = _5.0 & _20.0.0;
place!(Field::<(*const char, i8)>(Variant((*_24).fld0, 0), 0)).1 = _5.0 ^ _2.1;
_25 = (*_24).fld4 as u64;
place!(Field::<(*const char, i8)>(Variant(_20.1.fld0, 0), 0)).1 = -_2.1;
_17 = ((*_24).fld2,);
_20.3 = '\u{be7b3}';
_21 = (*_24).fld3 as isize;
_29 = _5.0 as f64;
place!(Field::<(*const char, i8)>(Variant(_20.1.fld0, 0), 0)).0 = core::ptr::addr_of!(_20.3);
RET = [(*_24).fld4,(*_24).fld4,(*_24).fld4];
(*_24).fld3 = _15 as f32;
_24 = core::ptr::addr_of_mut!((*_24));
Goto(bb9)
}
bb9 = {
_25 = _10 ^ (*_3);
_28 = _19;
_20.0.0 = Field::<(*const char, i8)>(Variant(_20.1.fld0, 0), 0).1;
(*_24).fld1 = core::ptr::addr_of!((*_3));
_22 = [_20.1.fld4];
_32 = (*_24).fld5 << (*_24).fld4;
_26 = _14;
(*_24).fld1 = Move(_3);
(*_24).fld3 = (*_24).fld4 as f32;
_32 = !_20.1.fld5;
_27 = -_21;
_2 = (_12.0, _20.0.0);
_29 = (*_24).fld5 as f64;
_22 = [(*_24).fld4];
_30 = _2.0;
(*_24).fld2 = _22;
_14 = -(*_24).fld5;
_34 = &_20.3;
_7 = _13;
place!(Field::<(*const char, i8)>(Variant((*_24).fld0, 0), 0)).1 = -_2.1;
_12.0 = !_2.0;
(*_24).fld1 = core::ptr::addr_of!(_25);
match _23 {
0 => bb6,
1 => bb10,
2 => bb11,
3 => bb12,
26646480224277062545876947622154662307 => bb14,
_ => bb13
}
}
bb10 = {
_12.0 = _2.0;
_12.0 = 46242225285736657485054390353339936924_u128 as u8;
_7 = RET;
_3 = core::ptr::addr_of!(_10);
_12.1 = !_2.1;
_12.0 = 13458061267320398777349881852656733327_i128 as u8;
_12.0 = _8;
_4 = 5516_u16 as u32;
_2.0 = !_12.0;
_5 = (_12.1,);
_7 = RET;
Goto(bb2)
}
bb11 = {
_20.1.fld2 = [(-270596061642920685_i64)];
_13 = [1134233808629437209_i64,4943854309907084878_i64,5103502913395920083_i64];
(*_6) = (-9449_i16) as f32;
_17.0 = _20.1.fld2;
_13 = _7;
_20.0.0 = _4 as i8;
_5 = (_12.1,);
RET = _13;
_20.1.fld2 = [4361665918543634592_i64];
_22 = [(-4068196750446325161_i64)];
_20.3 = '\u{5f640}';
_23 = 26646480224277062545876947622154662307_u128;
_7 = RET;
_21 = _8 as isize;
_20.1.fld5 = _14;
_12.0 = !Field::<u8>(Variant(_20.1.fld0, 1), 0);
_20.2 = core::ptr::addr_of_mut!(_23);
_22 = [(-5536434274790857837_i64)];
_20.1.fld2 = _22;
_20.0 = (_12.1,);
_20.1.fld4 = !3402582261417284975_i64;
_9 = !true;
_3 = core::ptr::addr_of!((*_3));
Goto(bb8)
}
bb12 = {
_10 = !11056722453500219387_u64;
_12 = (_8, _5.0);
_13 = [(-5722115809789290171_i64),1823466129922116292_i64,(-715106412257690115_i64)];
_14 = (-1084467005_i32) * 2061583931_i32;
_8 = _12.0;
_17 = (_11,);
_4 = !3060237798_u32;
_5 = (_12.1,);
_18 = _4;
_18 = _4;
_12 = (_2.0, _5.0);
_11 = _17.0;
_19 = !9223372036854775807_isize;
(*_3) = 2596975389149162826_u64 << _8;
_16 = [_9,_9];
_2 = _12;
_15 = !2_usize;
Goto(bb5)
}
bb13 = {
_12.0 = _2.0;
_12.0 = 46242225285736657485054390353339936924_u128 as u8;
_7 = RET;
_3 = core::ptr::addr_of!(_10);
_12.1 = !_2.1;
_12.0 = 13458061267320398777349881852656733327_i128 as u8;
_12.0 = _8;
_4 = 5516_u16 as u32;
_2.0 = !_12.0;
_5 = (_12.1,);
_7 = RET;
Goto(bb2)
}
bb14 = {
_17.0 = _20.1.fld2;
_12.1 = Field::<(*const char, i8)>(Variant((*_24).fld0, 0), 0).1;
_36 = Move((*_24).fld0);
_20.1.fld3 = _23 as f32;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(15_usize, 16_usize, Move(_16), 12_usize, Move(_12), 30_usize, Move(_30), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(15_usize, 13_usize, Move(_13), 11_usize, Move(_11), 27_usize, Move(_27), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(15_usize, 2_usize, Move(_2), 18_usize, Move(_18), 19_usize, Move(_19), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: char,mut _3: char,mut _4: u16,mut _5: bool,mut _6: bool,mut _7: [bool; 2],mut _8: i8,mut _9: i8,mut _10: u64,mut _11: bool,mut _12: u64) -> [u128; 2] {
mir! {
type RET = [u128; 2];
let _13: char;
let _14: isize;
let _15: i128;
let _16: usize;
let _17: [i32; 3];
let _18: bool;
let _19: i32;
let _20: (u8, i8);
let _21: f64;
let _22: i128;
let _23: &'static *mut u128;
let _24: (i64,);
let _25: (char, &'static i64);
let _26: &'static *mut u128;
let _27: &'static i32;
let _28: (f64, *mut f64, [i64; 3]);
let _29: isize;
let _30: char;
let _31: &'static (f32, u8, [i8; 5]);
let _32: i16;
let _33: [u128; 2];
let _34: usize;
let _35: i64;
let _36: &'static [i32; 3];
let _37: i128;
let _38: f32;
let _39: f64;
let _40: *mut f64;
let _41: isize;
let _42: ();
let _43: ();
{
_12 = !_10;
_6 = !_11;
_8 = _9;
_13 = _3;
_4 = _2 as u16;
RET = [324771829259384221124821197606234209083_u128,157626734216797197372416519466952201340_u128];
_4 = 30681_u16 | 46943_u16;
_4 = 5881_u16 + 63589_u16;
_2 = _13;
RET = [147527954430062420855203850819314140451_u128,166841234455225501633924561292327667610_u128];
_14 = (-9223372036854775808_isize);
_9 = _14 as i8;
_10 = _12;
_4 = 40116_u16 ^ 38233_u16;
_8 = _9;
_1 = _5;
_6 = _5 < _1;
_1 = _6 != _6;
RET = [104793944437841638953775857625448200218_u128,212333171257031428864401690970889906460_u128];
Goto(bb1)
}
bb1 = {
RET = [206220236696485524409221467401289514283_u128,285358776876256964143658613438141878423_u128];
_7 = [_11,_1];
_13 = _3;
_15 = (-20757_i16) as i128;
Goto(bb2)
}
bb2 = {
_8 = _12 as i8;
_9 = _8;
_5 = !_1;
_16 = _1 as usize;
_18 = !_1;
_15 = (-132483273806845792952561880967082390415_i128);
_8 = -_9;
_7 = [_6,_1];
_14 = !103_isize;
_1 = _5;
_10 = !_12;
_6 = !_18;
_8 = -_9;
_8 = _9 + _9;
_14 = 9223372036854775807_isize;
_17 = [344591185_i32,(-2067833681_i32),593532880_i32];
_19 = (-427639755_i32);
Goto(bb3)
}
bb3 = {
_15 = -(-150902127013888681411140890447284867393_i128);
_15 = 124294911203737226968120449365973408398_i128 >> _16;
_18 = _1;
_1 = !_6;
_20.0 = 75_u8;
_20.0 = 50_u8 >> _4;
_18 = _8 < _8;
_20.1 = -_9;
_22 = _15 - _15;
_14 = _10 as isize;
_5 = !_18;
_7 = [_1,_18];
RET = [207998645433191739590803426632174985343_u128,70923157040350734816810502781872658477_u128];
_2 = _13;
_11 = _1;
match _19 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431340571701 => bb9,
_ => bb8
}
}
bb4 = {
_8 = _12 as i8;
_9 = _8;
_5 = !_1;
_16 = _1 as usize;
_18 = !_1;
_15 = (-132483273806845792952561880967082390415_i128);
_8 = -_9;
_7 = [_6,_1];
_14 = !103_isize;
_1 = _5;
_10 = !_12;
_6 = !_18;
_8 = -_9;
_8 = _9 + _9;
_14 = 9223372036854775807_isize;
_17 = [344591185_i32,(-2067833681_i32),593532880_i32];
_19 = (-427639755_i32);
Goto(bb3)
}
bb5 = {
RET = [206220236696485524409221467401289514283_u128,285358776876256964143658613438141878423_u128];
_7 = [_11,_1];
_13 = _3;
_15 = (-20757_i16) as i128;
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
_21 = _16 as f64;
Goto(bb10)
}
bb10 = {
RET = [7433612405043110474073135343971183575_u128,242740126231431558818475831025693324128_u128];
_20.1 = _16 as i8;
_20.0 = 132_u8 * 44_u8;
_3 = _13;
_10 = !_12;
_20.1 = _9 >> _15;
_25.1 = &_24.0;
match _19 {
0 => bb9,
1 => bb2,
2 => bb6,
340282366920938463463374607431340571701 => bb11,
_ => bb8
}
}
bb11 = {
_2 = _3;
_11 = _20.1 != _9;
_24.0 = 1348532666912119665_i64 * 488558703534657937_i64;
_21 = _20.0 as f64;
_5 = _6;
_3 = _13;
_25.0 = _13;
_22 = _15;
_27 = &_19;
RET = [180025456454481666957816670798902933645_u128,246294143808423920344548445408058327892_u128];
_10 = _12;
_7 = [_1,_5];
_24.0 = (-6721846045082313502_i64);
match _19 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463463374607431340571701 => bb12,
_ => bb8
}
}
bb12 = {
_6 = _1;
_3 = _25.0;
_16 = !4_usize;
_29 = -_14;
_28.2 = [_24.0,_24.0,_24.0];
_25.1 = &_24.0;
_20 = (84_u8, _8);
_28.0 = _21 - _21;
_21 = _28.0 - _28.0;
_3 = _25.0;
_3 = _25.0;
_4 = 38375_u16;
_11 = _5 > _6;
_16 = !6_usize;
_10 = _12 ^ _12;
_6 = !_11;
_24 = (9005998969797815332_i64,);
_25.1 = &_24.0;
_29 = (-7643_i16) as isize;
Goto(bb13)
}
bb13 = {
_20.0 = !38_u8;
_24 = ((-1638172833508491867_i64),);
_27 = &(*_27);
_8 = _4 as i8;
_20.1 = !_9;
_27 = &(*_27);
RET = [155619103029639897654734730914076592406_u128,128511919311677258349418360944060749519_u128];
_25.0 = _13;
_24.0 = _16 as i64;
_13 = _2;
_27 = &_19;
_28.1 = core::ptr::addr_of_mut!(_21);
_32 = (-13389_i16) & (-11354_i16);
_19 = _16 as i32;
_8 = 687630147_u32 as i8;
_33 = [252958919259856107023426253178968914786_u128,68683765417469063367233217630759316365_u128];
_28.0 = _21;
_35 = _16 as i64;
_5 = _18;
_14 = _9 as isize;
_11 = _18 <= _6;
_8 = -_9;
_20.0 = 152_u8;
_4 = 14053_u16;
_28.2 = [_35,_35,_35];
match _20.0 {
0 => bb1,
1 => bb9,
2 => bb7,
3 => bb8,
4 => bb5,
152 => bb15,
_ => bb14
}
}
bb14 = {
_2 = _3;
_11 = _20.1 != _9;
_24.0 = 1348532666912119665_i64 * 488558703534657937_i64;
_21 = _20.0 as f64;
_5 = _6;
_3 = _13;
_25.0 = _13;
_22 = _15;
_27 = &_19;
RET = [180025456454481666957816670798902933645_u128,246294143808423920344548445408058327892_u128];
_10 = _12;
_7 = [_1,_5];
_24.0 = (-6721846045082313502_i64);
match _19 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463463374607431340571701 => bb12,
_ => bb8
}
}
bb15 = {
_4 = !4532_u16;
_11 = !_6;
_37 = _15 - _22;
_36 = &_17;
_35 = -_24.0;
_7 = [_11,_5];
_30 = _2;
_35 = -_24.0;
_8 = _20.1;
_15 = !_37;
_3 = _25.0;
_7 = [_6,_6];
_17 = [_19,_19,_19];
_29 = _14;
_4 = !23627_u16;
_30 = _3;
_29 = -_14;
_12 = _10 >> _10;
_8 = _9 + _20.1;
_15 = _37 - _37;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(16_usize, 14_usize, Move(_14), 4_usize, Move(_4), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(16_usize, 6_usize, Move(_6), 29_usize, Move(_29), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(16_usize, 2_usize, Move(_2), 20_usize, Move(_20), 12_usize, Move(_12), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(16_usize, 8_usize, Move(_8), 16_usize, Move(_16), 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(43759_u16), std::hint::black_box('\u{f1a9d}'), std::hint::black_box(233478400627009121936473827686723598277_u128), std::hint::black_box(3_i8), std::hint::black_box(2693372045_u32), std::hint::black_box(9695585426288417294_u64), std::hint::black_box((-8562781350383562206_i64)), std::hint::black_box((-35659433356280036842495702188052584584_i128)), std::hint::black_box(5_usize));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: (i8,),
fld1: char,
fld2: u64,
fld3: u128,
fld4: *mut f64,
fld5: i32,
fld6: u32,
fld7: usize,

},
Variant1{
fld0: u64,

},
Variant2{
fld0: i16,
fld1: (i8,),

},
Variant3{
fld0: isize,
fld1: u64,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: (*const char, i8),

},
Variant1{
fld0: u8,

},
Variant2{
fld0: bool,
fld1: u64,
fld2: (*const char, i8),
fld3: u16,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: *mut f32,
fld1: (i8,),
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: f32,
fld1: *mut f32,
fld2: ([i64; 1],),
fld3: [i32; 3],
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: Adt36,
fld1: *const u64,
fld2: [i64; 1],
fld3: f32,
fld4: i64,
fld5: i32,
}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){unsafe{printf("Adt66::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt66 {
Variant0{
fld0: u64,
fld1: *mut [i64; 3],

},
Variant1{
fld0: *const u64,

},
Variant2{
fld0: (f64, *mut f64, [i64; 3]),
fld1: Adt36,
fld2: *const i16,
fld3: Adt51,
fld4: *const char,
fld5: ((i8,), Adt51, *mut u128, char),

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: Adt36,
fld1: Adt51,
fld2: *const char,
fld3: *mut [i64; 3],
fld4: (*const char, i8),

},
Variant1{
fld0: *mut [i64; 3],
fld1: Adt38,
fld2: isize,
fld3: i8,
fld4: f64,
fld5: (f64, *mut f64, [i64; 3]),
fld6: [u128; 2],

},
Variant2{
fld0: *const bool,
fld1: ((i8,), Adt51, *mut u128, char),
fld2: [i8; 8],
fld3: *const (f64, *mut f64, [i64; 3]),

}}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){unsafe{printf("Adt79::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: *mut f64,
fld1: *mut f32,
fld2: [u16; 7],

},
Variant1{
fld0: ([i64; 1],),
fld1: ((i8,), Adt51, *mut u128, char),
fld2: *const (f64, *mut f64, [i64; 3]),
fld3: (i8,),
fld4: (u8, i8),
fld5: (*const char, i8),
fld6: *mut u128,
fld7: *mut f64,

},
Variant2{
fld0: *mut [i64; 3],
fld1: Adt46,
fld2: [u16; 7],

},
Variant3{
fld0: *const (f64, *mut f64, [i64; 3]),
fld1: ((i8,), Adt51, *mut u128, char),
fld2: *const char,

}}

