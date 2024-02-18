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
pub fn fn0(mut _1: u64,mut _2: i32) -> u8 {
mir! {
type RET = u8;
let _3: u64;
let _4: f64;
let _5: i128;
let _6: char;
let _7: f64;
let _8: i16;
let _9: u8;
let _10: ();
let _11: ();
{
RET = !135_u8;
_3 = !11471196813731938099_u64;
_3 = 6852004720580112860_u64;
_1 = _3 ^ _3;
_2 = (-1124429027_i32);
match _3 {
6852004720580112860 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = !220_u8;
RET = !149_u8;
_2 = 14806165936465330919_usize as i32;
_2 = !859391218_i32;
_1 = _3;
_3 = !_1;
Call(_1 = fn1(_2, _3, _3, _3, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = !_1;
_4 = 2076294492_u32 as f64;
_3 = !_1;
_2 = (-2046935503_i32) & 50191415_i32;
_4 = _1 as f64;
_4 = (-5911941098138932230_i64) as f64;
_4 = 167398691431700331518063789107052288945_u128 as f64;
RET = !128_u8;
_4 = 163801565905438004448327697130682369743_u128 as f64;
_1 = _3;
_2 = -274553467_i32;
_4 = 63_i8 as f64;
_3 = _1 - _1;
RET = 234_u8 - 233_u8;
_5 = !68046025556824391976362517455321309636_i128;
_1 = !_3;
_4 = 3648021999_u32 as f64;
_1 = !_3;
_5 = 18621_i16 as i128;
RET = !52_u8;
RET = 201_u8 * 148_u8;
RET = 5_u8 + 243_u8;
_5 = 36999963679493283014488661603947072969_i128 ^ (-29013269731274229145788537502514275083_i128);
RET = 76_u8;
_4 = (-111_i8) as f64;
RET = 131_u8;
Call(_1 = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = (-73265855725309447610235107218178397461_i128);
_1 = !_3;
RET = 9223372036854775807_isize as u8;
RET = 82_u8 << _1;
_1 = _3;
_7 = -_4;
Goto(bb5)
}
bb5 = {
Call(_10 = dump_var(0_usize, 3_usize, Move(_3), 1_usize, Move(_1), 11_usize, _11, 11_usize, _11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i32,mut _2: u64,mut _3: u64,mut _4: u64,mut _5: i32) -> u64 {
mir! {
type RET = u64;
let _6: f64;
let _7: &'static *mut i128;
let _8: (f64, *mut i128, i8, char);
let _9: char;
let _10: bool;
let _11: ((u128, u32, i32),);
let _12: [i64; 5];
let _13: f64;
let _14: i32;
let _15: [i8; 4];
let _16: [u16; 7];
let _17: bool;
let _18: f32;
let _19: Adt52;
let _20: i8;
let _21: ((u16, bool, Adt39, [u16; 8]), *const i32, (f32,), Adt43);
let _22: isize;
let _23: ((u128, u32, i32),);
let _24: (f32,);
let _25: u16;
let _26: isize;
let _27: [usize; 1];
let _28: &'static [u16; 8];
let _29: f64;
let _30: [usize; 2];
let _31: isize;
let _32: ((i64, (*const i32, [i16; 3], usize, bool), (bool,)),);
let _33: ((i64, (*const i32, [i16; 3], usize, bool), (bool,)),);
let _34: [u64; 7];
let _35: u32;
let _36: i16;
let _37: [u64; 7];
let _38: ();
let _39: ();
{
RET = _3 << _4;
_5 = _1 * _1;
_2 = RET;
RET = 106_u8 as u64;
_3 = RET;
RET = _4 << _5;
_6 = 106_u8 as f64;
_2 = RET >> RET;
RET = _2 ^ _4;
RET = 36_i8 as u64;
RET = _2 ^ _2;
_3 = _2 << RET;
_5 = '\u{df258}' as i32;
_7 = &_8.1;
_1 = _5 * _5;
_9 = '\u{d32c0}';
_8.2 = !106_i8;
_7 = &(*_7);
_8.0 = -_6;
_3 = _1 as u64;
RET = _3;
_4 = _9 as u64;
_7 = &(*_7);
RET = 210_u8 as u64;
Goto(bb1)
}
bb1 = {
_6 = (-2107106555281773334_i64) as f64;
_8.3 = _9;
_9 = _8.3;
_4 = !_2;
_10 = false;
RET = _2 >> _5;
_1 = !_5;
_2 = !_4;
_4 = !_3;
_9 = _8.3;
_3 = 193_u8 as u64;
_8.3 = _9;
_2 = _10 as u64;
_7 = &(*_7);
_1 = _5;
_12 = [8091122338554667107_i64,(-5607713943371081217_i64),6986877366931445497_i64,4487993454159583411_i64,(-5546394948474767844_i64)];
_12 = [5623274001266070676_i64,(-3123104688455657096_i64),(-5626688909810203160_i64),(-2599081003644857376_i64),2641606110947827056_i64];
_11.0.1 = 583_i16 as u32;
_7 = &_8.1;
Goto(bb2)
}
bb2 = {
_3 = _8.3 as u64;
_9 = _8.3;
_5 = 213_u8 as i32;
RET = _4;
_8.0 = _11.0.1 as f64;
_12 = [(-2010073702460746469_i64),4693484608107648836_i64,(-8370066134839505155_i64),(-3131617238409031519_i64),6557273125686035904_i64];
_12 = [(-5435294380999721512_i64),1164345476523201717_i64,901435810809906180_i64,(-6963395953808494792_i64),4751267698536884114_i64];
_14 = _5;
_6 = _8.0 - _8.0;
RET = _2 ^ _4;
_11.0.0 = (-9223372036854775808_isize) as u128;
_8.0 = _6;
_16 = [55699_u16,741_u16,42123_u16,23107_u16,55985_u16,581_u16,1650_u16];
_7 = &_8.1;
_8.0 = _6 + _6;
_11.0.2 = _14;
_9 = _8.3;
_10 = _9 > _8.3;
_13 = _8.0 * _8.0;
_3 = !_4;
_11.0.0 = 217046674105600609444492195893204282943_u128 * 66350969768721841227676761697946314618_u128;
_5 = _11.0.1 as i32;
RET = 4196896177556844565_usize as u64;
_11.0.0 = _4 as u128;
Call(_14 = fn2(_8.3, _13, _13, _12, _8.0, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = _8.0;
_2 = 91_isize as u64;
_16 = [15906_u16,1920_u16,56690_u16,33119_u16,8713_u16,49097_u16,49837_u16];
_2 = _9 as u64;
_7 = &(*_7);
_5 = _14;
_11.0 = (114716060648809246985334749933546896792_u128, 922680540_u32, _14);
_11.0.1 = 46628924_u32;
_4 = !_2;
_12 = [(-6056153360008038082_i64),7346199988290820594_i64,5610462350606719431_i64,9003611323486246004_i64,7371412837560140923_i64];
Goto(bb4)
}
bb4 = {
_11.0 = (219344633283397430183241838400170516002_u128, 715282049_u32, _5);
RET = _2 & _2;
_13 = 42808_u16 as f64;
_17 = !_10;
_11.0.1 = 3204920694_u32 >> _3;
_10 = _4 > _2;
_11.0.1 = 964747210_u32 << _5;
_1 = (-118_isize) as i32;
_11.0.1 = 4270200208_u32 - 375103936_u32;
_3 = !_4;
Call(_8.0 = core::intrinsics::fmaf64(_6, _13, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = &(*_7);
_5 = _14 & _11.0.2;
_11.0.2 = 219_u8 as i32;
_18 = 12017635544379016459_usize as f32;
_12 = [6307116952360829974_i64,(-3358721472441217768_i64),(-3780670828531073419_i64),(-3075802889161219937_i64),(-1151409379198117590_i64)];
RET = _4;
_13 = -_8.0;
_7 = &_8.1;
_13 = (-14_isize) as f64;
_4 = _8.0 as u64;
_12 = [2721671334721631492_i64,7426981144900824516_i64,(-1238631795416352610_i64),(-4597915536037600531_i64),6300165322736286093_i64];
_11.0.0 = !22321400927735412869656224524594489256_u128;
_11.0.0 = !290278703270998472635729167180160386198_u128;
_11.0 = (336288110542194662495027148002952240335_u128, 2742401927_u32, _1);
_9 = _8.3;
_8.0 = _6;
_6 = -_8.0;
_17 = _10;
_14 = _11.0.2;
_5 = _11.0.2 << _11.0.0;
_7 = &_8.1;
_22 = _8.3 as isize;
_21.3.fld4.3 = [364_u16,22795_u16,2301_u16,17885_u16,58351_u16,9614_u16,41346_u16,39487_u16];
_11.0.0 = 280721995911461638100389543575797137328_u128;
_11.0.1 = !1014634006_u32;
_21.3.fld3.0.0.0 = _8.0 + _6;
Goto(bb6)
}
bb6 = {
_21.0.1 = _8.2 >= _8.2;
_8.1 = core::ptr::addr_of_mut!(_21.3.fld2.1);
_16 = [51053_u16,42257_u16,9754_u16,18643_u16,48816_u16,15423_u16,56125_u16];
_21.3.fld2 = (_21.0.1, 48416916810731415734059584995477173804_i128, _10, _22);
_21.3.fld3.0.0.0 = _8.0;
_21.3.fld0 = _21.3.fld2.0 & _10;
_12 = [5716217332054174158_i64,(-2878982993452767581_i64),(-5618781992376022383_i64),(-7204756163443868620_i64),140316705775102655_i64];
_22 = _21.3.fld2.3;
_22 = _21.3.fld2.3;
_24 = (_18,);
_21.3.fld3.0.0.3 = _8.3;
_20 = _8.2;
_6 = 133_u8 as f64;
match _21.3.fld2.1 {
0 => bb1,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
48416916810731415734059584995477173804 => bb12,
_ => bb11
}
}
bb7 = {
_7 = &(*_7);
_5 = _14 & _11.0.2;
_11.0.2 = 219_u8 as i32;
_18 = 12017635544379016459_usize as f32;
_12 = [6307116952360829974_i64,(-3358721472441217768_i64),(-3780670828531073419_i64),(-3075802889161219937_i64),(-1151409379198117590_i64)];
RET = _4;
_13 = -_8.0;
_7 = &_8.1;
_13 = (-14_isize) as f64;
_4 = _8.0 as u64;
_12 = [2721671334721631492_i64,7426981144900824516_i64,(-1238631795416352610_i64),(-4597915536037600531_i64),6300165322736286093_i64];
_11.0.0 = !22321400927735412869656224524594489256_u128;
_11.0.0 = !290278703270998472635729167180160386198_u128;
_11.0 = (336288110542194662495027148002952240335_u128, 2742401927_u32, _1);
_9 = _8.3;
_8.0 = _6;
_6 = -_8.0;
_17 = _10;
_14 = _11.0.2;
_5 = _11.0.2 << _11.0.0;
_7 = &_8.1;
_22 = _8.3 as isize;
_21.3.fld4.3 = [364_u16,22795_u16,2301_u16,17885_u16,58351_u16,9614_u16,41346_u16,39487_u16];
_11.0.0 = 280721995911461638100389543575797137328_u128;
_11.0.1 = !1014634006_u32;
_21.3.fld3.0.0.0 = _8.0 + _6;
Goto(bb6)
}
bb8 = {
_11.0 = (219344633283397430183241838400170516002_u128, 715282049_u32, _5);
RET = _2 & _2;
_13 = 42808_u16 as f64;
_17 = !_10;
_11.0.1 = 3204920694_u32 >> _3;
_10 = _4 > _2;
_11.0.1 = 964747210_u32 << _5;
_1 = (-118_isize) as i32;
_11.0.1 = 4270200208_u32 - 375103936_u32;
_3 = !_4;
Call(_8.0 = core::intrinsics::fmaf64(_6, _13, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_13 = _8.0;
_2 = 91_isize as u64;
_16 = [15906_u16,1920_u16,56690_u16,33119_u16,8713_u16,49097_u16,49837_u16];
_2 = _9 as u64;
_7 = &(*_7);
_5 = _14;
_11.0 = (114716060648809246985334749933546896792_u128, 922680540_u32, _14);
_11.0.1 = 46628924_u32;
_4 = !_2;
_12 = [(-6056153360008038082_i64),7346199988290820594_i64,5610462350606719431_i64,9003611323486246004_i64,7371412837560140923_i64];
Goto(bb4)
}
bb10 = {
_3 = _8.3 as u64;
_9 = _8.3;
_5 = 213_u8 as i32;
RET = _4;
_8.0 = _11.0.1 as f64;
_12 = [(-2010073702460746469_i64),4693484608107648836_i64,(-8370066134839505155_i64),(-3131617238409031519_i64),6557273125686035904_i64];
_12 = [(-5435294380999721512_i64),1164345476523201717_i64,901435810809906180_i64,(-6963395953808494792_i64),4751267698536884114_i64];
_14 = _5;
_6 = _8.0 - _8.0;
RET = _2 ^ _4;
_11.0.0 = (-9223372036854775808_isize) as u128;
_8.0 = _6;
_16 = [55699_u16,741_u16,42123_u16,23107_u16,55985_u16,581_u16,1650_u16];
_7 = &_8.1;
_8.0 = _6 + _6;
_11.0.2 = _14;
_9 = _8.3;
_10 = _9 > _8.3;
_13 = _8.0 * _8.0;
_3 = !_4;
_11.0.0 = 217046674105600609444492195893204282943_u128 * 66350969768721841227676761697946314618_u128;
_5 = _11.0.1 as i32;
RET = 4196896177556844565_usize as u64;
_11.0.0 = _4 as u128;
Call(_14 = fn2(_8.3, _13, _13, _12, _8.0, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_6 = (-2107106555281773334_i64) as f64;
_8.3 = _9;
_9 = _8.3;
_4 = !_2;
_10 = false;
RET = _2 >> _5;
_1 = !_5;
_2 = !_4;
_4 = !_3;
_9 = _8.3;
_3 = 193_u8 as u64;
_8.3 = _9;
_2 = _10 as u64;
_7 = &(*_7);
_1 = _5;
_12 = [8091122338554667107_i64,(-5607713943371081217_i64),6986877366931445497_i64,4487993454159583411_i64,(-5546394948474767844_i64)];
_12 = [5623274001266070676_i64,(-3123104688455657096_i64),(-5626688909810203160_i64),(-2599081003644857376_i64),2641606110947827056_i64];
_11.0.1 = 583_i16 as u32;
_7 = &_8.1;
Goto(bb2)
}
bb12 = {
_3 = !_2;
_21.3.fld3.0.0.1 = Move(_8.1);
_21.3.fld2.2 = _21.3.fld0;
_21.3.fld2.1 = !(-44181442561523761330436439688104263944_i128);
_8 = (_21.3.fld3.0.0.0, Move(_21.3.fld3.0.0.1), _20, _9);
_18 = -_24.0;
_21.1 = core::ptr::addr_of!(_5);
_17 = _21.0.1 ^ _21.0.1;
_21.3.fld3.0.1 = !_21.3.fld2.0;
_21.0.3 = [19526_u16,61755_u16,26517_u16,61065_u16,64123_u16,1886_u16,19194_u16,31918_u16];
_26 = -_21.3.fld2.3;
_21.3.fld3.0.1 = _21.3.fld2.2 & _21.0.1;
_15 = [_20,_8.2,_8.2,_20];
_26 = _21.3.fld2.3;
_21.3.fld3.0.0.3 = _8.3;
_23.0.2 = _5;
_21.3.fld3.0 = (Move(_8), _17, _16, _11.0.0);
_21.3.fld2.2 = !_21.3.fld0;
_21.2 = (_24.0,);
_8.0 = _18 as f64;
_21.2 = (_18,);
Goto(bb13)
}
bb13 = {
_23.0.2 = _5;
_21.3.fld4.1 = !_10;
_8.2 = _20;
RET = _4;
match _11.0.0 {
0 => bb1,
1 => bb10,
2 => bb12,
3 => bb4,
4 => bb5,
280721995911461638100389543575797137328 => bb14,
_ => bb6
}
}
bb14 = {
_7 = &_8.1;
_33.0.1.1 = [(-5945_i16),(-977_i16),29181_i16];
_26 = _22;
_32.0.1.3 = _21.3.fld3.0.1;
_21.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_21.3.fld2.1);
_34 = [_4,RET,RET,RET,_4,RET,_3];
_21.3.fld4.0 = 4117_u16;
_21.3.fld1 = Adt28::Variant1 { fld0: Move(_21.1),fld1: _9,fld2: 42_u8,fld3: _24,fld4: Move(_21.3.fld3.0.0.1),fld5: _21.3.fld3.0.3,fld6: _21.3.fld4.0 };
_5 = _23.0.2 >> _26;
_21.3.fld3.0.0 = (_13, Move(Field::<*mut i128>(Variant(_21.3.fld1, 1), 4)), _20, _9);
_21.3.fld3.0.0.2 = _20;
_22 = _21.3.fld2.3 & _21.3.fld2.3;
RET = _4 << _11.0.1;
_33.0.2 = (_21.3.fld2.0,);
_1 = Field::<u128>(Variant(_21.3.fld1, 1), 5) as i32;
_32.0.1.0 = Move(Field::<*const i32>(Variant(_21.3.fld1, 1), 0));
_8 = (_6, Move(_21.3.fld3.0.0.1), _21.3.fld3.0.0.2, _21.3.fld3.0.0.3);
_33.0.1.0 = core::ptr::addr_of!(_11.0.2);
_6 = _21.3.fld3.0.0.0;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(1_usize, 14_usize, Move(_14), 10_usize, Move(_10), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(1_usize, 11_usize, Move(_11), 15_usize, Move(_15), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: char,mut _2: f64,mut _3: f64,mut _4: [i64; 5],mut _5: f64,mut _6: [u16; 7]) -> i32 {
mir! {
type RET = i32;
let _7: [u64; 1];
let _8: *const [u16; 7];
let _9: *const [i64; 5];
let _10: f64;
let _11: i64;
let _12: f64;
let _13: [u32; 6];
let _14: isize;
let _15: char;
let _16: ((f32,), *mut u16, i16, (*const i32, [i16; 3], usize, bool));
let _17: f64;
let _18: *mut u32;
let _19: (bool,);
let _20: [char; 5];
let _21: f32;
let _22: *const [i64; 5];
let _23: isize;
let _24: (u128, u32, i32);
let _25: f32;
let _26: f64;
let _27: u64;
let _28: [u16; 7];
let _29: [i16; 3];
let _30: (u128, u32, i32);
let _31: f32;
let _32: *const (*const i32, f64, (f32,), f32);
let _33: *const i32;
let _34: *mut u16;
let _35: ();
let _36: ();
{
_2 = _3;
_2 = 52630_u16 as f64;
RET = (-27735745795175692425392653067994447605_i128) as i32;
_6 = [17861_u16,45158_u16,7493_u16,55074_u16,6938_u16,1170_u16,47618_u16];
_3 = -_5;
RET = 1547767470_i32;
RET = 1775981715_i32 - (-780663978_i32);
_9 = core::ptr::addr_of!(_4);
_6 = [51343_u16,53215_u16,5211_u16,21170_u16,14602_u16,33388_u16,43941_u16];
_4 = [4470667356761181631_i64,(-1303532205370013424_i64),5651065877855421959_i64,(-3291242340220942799_i64),2071759044275527725_i64];
_11 = 14371126336289406589_u64 as i64;
_4 = [_11,_11,_11,_11,_11];
_10 = -_5;
_7 = [16446584829079423120_u64];
_7 = [7866199135420109346_u64];
_8 = core::ptr::addr_of!(_6);
_5 = 30701_i16 as f64;
RET = 151_u8 as i32;
_5 = _3 - _10;
_4 = [_11,_11,_11,_11,_11];
Goto(bb1)
}
bb1 = {
RET = 115281716_i32;
(*_9) = [_11,_11,_11,_11,_11];
_8 = core::ptr::addr_of!((*_8));
_9 = core::ptr::addr_of!((*_9));
_7 = [14242929717034921809_u64];
_8 = core::ptr::addr_of!(_6);
_7 = [4719926582184338377_u64];
_2 = _3;
_12 = -_5;
_13 = [2116537633_u32,3590241283_u32,457350938_u32,3818983005_u32,4177166149_u32,1191768489_u32];
_9 = core::ptr::addr_of!(_4);
_7 = [5581498674544238427_u64];
_7 = [4791936784439771050_u64];
(*_9) = [_11,_11,_11,_11,_11];
(*_9) = [_11,_11,_11,_11,_11];
RET = (-104704245_i32);
RET = -(-1351727486_i32);
_12 = -_10;
_14 = (-9223372036854775808_isize);
_1 = '\u{ad547}';
RET = (-1013435123_i32);
_5 = _3 * _12;
Call(_3 = fn3(_1, _5, _14, Move(_8), _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_9) = [_11,_11,_11,_11,_11];
_8 = core::ptr::addr_of!(_6);
_7 = [14581590406875977306_u64];
RET = !(-1271354590_i32);
_12 = -_3;
_7 = [3350974203565760663_u64];
_4 = [_11,_11,_11,_11,_11];
_3 = _12;
(*_8) = [9368_u16,57573_u16,57715_u16,15095_u16,58029_u16,39596_u16,53497_u16];
(*_8) = [1021_u16,47481_u16,38099_u16,893_u16,20277_u16,1516_u16,28020_u16];
_16.2 = 3082_i16;
_5 = _12 * _12;
Goto(bb3)
}
bb3 = {
_3 = _5 * _5;
_17 = _3 + _5;
_11 = -(-4585969061130913933_i64);
_9 = core::ptr::addr_of!((*_9));
_16.3.1 = [_16.2,_16.2,_16.2];
(*_8) = [19222_u16,44168_u16,63283_u16,19711_u16,6246_u16,32112_u16,9952_u16];
_9 = core::ptr::addr_of!((*_9));
RET = 157_u8 as i32;
_6 = [30319_u16,7089_u16,33860_u16,53743_u16,25440_u16,62192_u16,23313_u16];
_16.3.3 = !true;
_1 = '\u{bd450}';
(*_9) = [_11,_11,_11,_11,_11];
_8 = core::ptr::addr_of!((*_8));
_10 = _12;
_16.3.3 = false;
_9 = core::ptr::addr_of!((*_9));
_14 = RET as isize;
_7 = [6747428748059287336_u64];
_16.0.0 = 48611_u16 as f32;
_3 = _10;
(*_8) = [2264_u16,37306_u16,10962_u16,39385_u16,20425_u16,52441_u16,26034_u16];
_12 = _17 - _17;
_12 = _17 - _17;
_16.3.0 = core::ptr::addr_of!(RET);
_4 = [_11,_11,_11,_11,_11];
_13 = [3220707485_u32,2854713095_u32,1759958714_u32,156430865_u32,3403041973_u32,1563171535_u32];
_9 = core::ptr::addr_of!((*_9));
_1 = '\u{c34f7}';
match _16.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
3082 => bb9,
_ => bb8
}
}
bb4 = {
(*_9) = [_11,_11,_11,_11,_11];
_8 = core::ptr::addr_of!(_6);
_7 = [14581590406875977306_u64];
RET = !(-1271354590_i32);
_12 = -_3;
_7 = [3350974203565760663_u64];
_4 = [_11,_11,_11,_11,_11];
_3 = _12;
(*_8) = [9368_u16,57573_u16,57715_u16,15095_u16,58029_u16,39596_u16,53497_u16];
(*_8) = [1021_u16,47481_u16,38099_u16,893_u16,20277_u16,1516_u16,28020_u16];
_16.2 = 3082_i16;
_5 = _12 * _12;
Goto(bb3)
}
bb5 = {
RET = 115281716_i32;
(*_9) = [_11,_11,_11,_11,_11];
_8 = core::ptr::addr_of!((*_8));
_9 = core::ptr::addr_of!((*_9));
_7 = [14242929717034921809_u64];
_8 = core::ptr::addr_of!(_6);
_7 = [4719926582184338377_u64];
_2 = _3;
_12 = -_5;
_13 = [2116537633_u32,3590241283_u32,457350938_u32,3818983005_u32,4177166149_u32,1191768489_u32];
_9 = core::ptr::addr_of!(_4);
_7 = [5581498674544238427_u64];
_7 = [4791936784439771050_u64];
(*_9) = [_11,_11,_11,_11,_11];
(*_9) = [_11,_11,_11,_11,_11];
RET = (-104704245_i32);
RET = -(-1351727486_i32);
_12 = -_10;
_14 = (-9223372036854775808_isize);
_1 = '\u{ad547}';
RET = (-1013435123_i32);
_5 = _3 * _12;
Call(_3 = fn3(_1, _5, _14, Move(_8), _13), ReturnTo(bb2), UnwindUnreachable())
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
_16.0.0 = 111_u8 as f32;
_4 = [_11,_11,_11,_11,_11];
_11 = 4435515008294328200_usize as i64;
_16.2 = 26695894269241839236386074023290887432_u128 as i16;
_15 = _1;
_2 = -_17;
_16.3.3 = false;
_15 = _1;
_16.0.0 = _11 as f32;
_2 = 64695289419387254247352549651195370098_u128 as f64;
_16.3.2 = 7_usize;
_19 = (_16.3.3,);
_1 = _15;
_16.3.0 = core::ptr::addr_of!(RET);
_16.3.2 = 4_usize - 4403058020062535703_usize;
_11 = 319921140630506105_i64 + (-362522319902239369_i64);
_13 = [3190254894_u32,1299202963_u32,2874780343_u32,3104838817_u32,559691471_u32,873484601_u32];
Goto(bb10)
}
bb10 = {
_13 = [1220743073_u32,1562066880_u32,2430998342_u32,2355411833_u32,856332948_u32,4162005347_u32];
_3 = -_17;
_15 = _1;
_16.0.0 = 299721414972611984985374808123954573164_u128 as f32;
_6 = [43139_u16,53434_u16,40_u16,23825_u16,48689_u16,65059_u16,57622_u16];
_16.2 = -(-29635_i16);
_20 = [_15,_1,_15,_1,_1];
_16.3.2 = !4_usize;
Call(_6 = fn18(_3, _5, _5, Move(_8)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = 639704264_i32 | 2101790580_i32;
_9 = core::ptr::addr_of!((*_9));
(*_9) = [_11,_11,_11,_11,_11];
_12 = _5;
_16.0.0 = 10782232013679279148_u64 as f32;
Goto(bb12)
}
bb12 = {
_17 = -_3;
(*_9) = [_11,_11,_11,_11,_11];
_14 = -9223372036854775807_isize;
_14 = _16.0.0 as isize;
_22 = core::ptr::addr_of!(_4);
_6 = [39780_u16,65222_u16,32664_u16,13800_u16,17431_u16,7607_u16,55970_u16];
_16.3.0 = core::ptr::addr_of!(RET);
_16.3.2 = 61034_u16 as usize;
RET = (-1702493842_i32) - 1787519062_i32;
_16.3.0 = core::ptr::addr_of!(RET);
_23 = !_14;
_16.3.2 = !5_usize;
RET = (-1637376413_i32) * 890675061_i32;
_8 = core::ptr::addr_of!(_6);
_19 = (_16.3.3,);
_24.1 = 3412409402_u32 >> _11;
_12 = -_10;
_16.0.0 = 5682383440035362431_u64 as f32;
_6 = [21549_u16,23515_u16,20451_u16,56503_u16,12573_u16,62001_u16,12603_u16];
_1 = _15;
_11 = !3058395637922401647_i64;
_16.3.2 = (-55_i8) as usize;
_24.2 = -RET;
Goto(bb13)
}
bb13 = {
_13 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_13 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
(*_9) = [_11,_11,_11,_11,_11];
_5 = _24.2 as f64;
_17 = -_3;
_24.1 = 3876625103_u32;
(*_22) = [_11,_11,_11,_11,_11];
_24.1 = !3191615160_u32;
Goto(bb14)
}
bb14 = {
_13 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_16.2 = (-7648_i16);
(*_8) = [57325_u16,39221_u16,34042_u16,17738_u16,7513_u16,65431_u16,40757_u16];
_16.3.1 = [_16.2,_16.2,_16.2];
_14 = _23 | _23;
_25 = -_16.0.0;
_15 = _1;
_11 = 4549545531661070109_i64;
_16.0 = (_25,);
_24 = (222630091098877255918446829156634909588_u128, 1959862561_u32, RET);
_3 = _14 as f64;
_18 = core::ptr::addr_of_mut!(_24.1);
_1 = _15;
_26 = _17;
_27 = 6676341461451655008_u64;
_2 = -_26;
_21 = -_25;
(*_8) = [17205_u16,9196_u16,16125_u16,10988_u16,49345_u16,44414_u16,12840_u16];
_10 = _26;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(2_usize, 4_usize, Move(_4), 24_usize, Move(_24), 14_usize, Move(_14), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(2_usize, 7_usize, Move(_7), 11_usize, Move(_11), 36_usize, _36, 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: f64,mut _3: isize,mut _4: *const [u16; 7],mut _5: [u32; 6]) -> f64 {
mir! {
type RET = f64;
let _6: (i32, (f32,), *mut u16);
let _7: isize;
let _8: (i32, (f32,), *mut u16);
let _9: *mut bool;
let _10: [usize; 2];
let _11: [i64; 5];
let _12: &'static [u16; 8];
let _13: i64;
let _14: [char; 5];
let _15: &'static [u16; 8];
let _16: Adt48;
let _17: ();
let _18: ();
{
RET = 49713_u16 as f64;
RET = 20580_i16 as f64;
_5 = [2582553593_u32,4056907698_u32,882379657_u32,2177964581_u32,2838383173_u32,1058930014_u32];
RET = _2 - _2;
RET = _2;
_5 = [2178823991_u32,1023445747_u32,1251049994_u32,73973541_u32,2537773470_u32,3711157702_u32];
_6.0 = (-1173708056_i32) + 1446311504_i32;
Goto(bb1)
}
bb1 = {
_5 = [3377954130_u32,1782784647_u32,2703509912_u32,305918842_u32,1819453537_u32,192669073_u32];
_3 = -(-9223372036854775808_isize);
_2 = -RET;
Goto(bb2)
}
bb2 = {
_3 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_5 = [2323076875_u32,3358746899_u32,590158656_u32,164439286_u32,1324666687_u32,1294953992_u32];
_2 = RET;
_5 = [3132261441_u32,1058276566_u32,2201112052_u32,3491455964_u32,2092470382_u32,29609217_u32];
_8.0 = _6.0;
_5 = [3310401442_u32,64167045_u32,296562739_u32,2504546106_u32,2885326962_u32,1516065792_u32];
_8.0 = _6.0;
_8.1.0 = _3 as f32;
RET = -_2;
RET = _2 * _2;
_6.1 = (_8.1.0,);
_3 = !(-120_isize);
_8.1.0 = -_6.1.0;
_6.1 = _8.1;
RET = -_2;
_5 = [4006534506_u32,1615147159_u32,1279389823_u32,1611637956_u32,2892082998_u32,1162647576_u32];
RET = -_2;
_6.1 = (_8.1.0,);
_6.1 = _8.1;
_6.1 = (_8.1.0,);
_1 = '\u{cfb82}';
_6.0 = _8.0;
RET = _2;
_3 = -101_isize;
_6.1 = _8.1;
_6.1.0 = _8.1.0;
_6.1 = _8.1;
Goto(bb3)
}
bb3 = {
_8.1.0 = _6.1.0 * _6.1.0;
_8.0 = -_6.0;
_6.1 = _8.1;
_7 = !_3;
_8.1 = _6.1;
Goto(bb4)
}
bb4 = {
_11 = [(-2100917266925059722_i64),9163227708131033231_i64,184002048315269593_i64,(-7349700661815670087_i64),(-1130757508776488720_i64)];
_7 = !_3;
_6.0 = -_8.0;
_2 = RET;
_10 = [2_usize,1_usize];
Call(RET = fn4(_5, _1, _6.1.0, _7, Move(_4), _6.1.0, _8.1, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8.1.0 = -_6.1.0;
_5 = [946729553_u32,3963382914_u32,98232712_u32,969258980_u32,847666340_u32,815970641_u32];
Call(_4 = fn8(_5, _7, _1, RET, RET, _2, _2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8.0 = (-4336562731879221257_i64) as i32;
_13 = -684679127444075138_i64;
_2 = RET * RET;
_6.1 = (_8.1.0,);
_11 = [_13,_13,_13,_13,_13];
_1 = '\u{2bd93}';
RET = _6.1.0 as f64;
_8.1 = _6.1;
RET = -_2;
_13 = (-864287474652951166_i64);
_8.0 = !_6.0;
_6.1.0 = RET as f32;
_11 = [_13,_13,_13,_13,_13];
_5 = [2403644971_u32,541781161_u32,328661311_u32,2122150872_u32,2547925617_u32,2321011120_u32];
_5 = [3145267867_u32,2615467114_u32,222717970_u32,1751511381_u32,1858913956_u32,2849941615_u32];
_11 = [_13,_13,_13,_13,_13];
Goto(bb7)
}
bb7 = {
Call(_17 = dump_var(3_usize, 3_usize, Move(_3), 13_usize, Move(_13), 10_usize, Move(_10), 18_usize, _18), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [u32; 6],mut _2: char,mut _3: f32,mut _4: isize,mut _5: *const [u16; 7],mut _6: f32,mut _7: (f32,),mut _8: char) -> f64 {
mir! {
type RET = f64;
let _9: u16;
let _10: [i64; 5];
let _11: f64;
let _12: u32;
let _13: [u64; 1];
let _14: Adt52;
let _15: [u64; 1];
let _16: *mut u16;
let _17: char;
let _18: f64;
let _19: isize;
let _20: bool;
let _21: (u16,);
let _22: &'static [u16; 8];
let _23: isize;
let _24: Adt48;
let _25: [i16; 3];
let _26: i32;
let _27: *const [usize; 1];
let _28: [u64; 1];
let _29: [i64; 5];
let _30: Adt48;
let _31: (u16,);
let _32: ();
let _33: ();
{
_3 = 647924022_u32 as f32;
RET = 109136951925691417044186636330472004952_i128 as f64;
_3 = _6 * _6;
_3 = 51826_u16 as f32;
_3 = -_7.0;
_7.0 = _6;
_9 = 7283_u16;
_7.0 = -_3;
RET = 3937357833598790101_usize as f64;
_1 = [154688396_u32,454151516_u32,3660779060_u32,368642281_u32,3478497385_u32,1532163796_u32];
_10 = [9033455777642457747_i64,165865000733487421_i64,(-635165674336953111_i64),(-1875733256945233604_i64),1700677881656603185_i64];
_2 = _8;
_9 = 2569570682_u32 as u16;
_1 = [3122566718_u32,3197721089_u32,3707418247_u32,180873137_u32,389565271_u32,4021584706_u32];
_4 = !(-9223372036854775808_isize);
_9 = 246_u8 as u16;
_1 = [3274954663_u32,144833146_u32,1258079603_u32,147421848_u32,860492346_u32,342786786_u32];
_9 = 6155_u16;
_7.0 = _3 * _6;
_6 = (-128589471133682426021181073283052615995_i128) as f32;
_10 = [(-3046113678479448532_i64),(-6660911064992792303_i64),(-4048299187004524349_i64),(-8717099333735973744_i64),4782803800815453521_i64];
_2 = _8;
_9 = 15867_u16;
_11 = RET;
RET = -_11;
match _9 {
0 => bb1,
15867 => bb3,
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
_2 = _8;
_4 = 9223372036854775807_isize;
_3 = _6;
_8 = _2;
_6 = 31_i8 as f32;
_6 = 229_u8 as f32;
_12 = _4 as u32;
RET = -_11;
_9 = !20573_u16;
_10 = [637544702389634021_i64,5029723636594062946_i64,5894892201018671632_i64,(-4243913409058846905_i64),(-4916780336631159545_i64)];
_12 = _9 as u32;
_7.0 = _6;
_3 = RET as f32;
_13 = [10240001060610089954_u64];
RET = _11;
_12 = !1900608211_u32;
_7.0 = _6;
_8 = _2;
_11 = -RET;
_13 = [6239174371831805386_u64];
_1 = [_12,_12,_12,_12,_12,_12];
_13 = [14473069164122749966_u64];
_8 = _2;
_13 = [7315777601347258241_u64];
_4 = (-9223372036854775808_isize) + 44_isize;
RET = _11 + _11;
_2 = _8;
_4 = 4942703343857488362_u64 as isize;
Call(_12 = fn5(_10, Move(_5), _3, RET, _10, _10, _2, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = 805_i16 as f64;
_4 = (-40_isize) * (-9223372036854775808_isize);
_7 = (_3,);
_6 = _3 - _7.0;
_8 = _2;
_11 = 16531547982030826481_usize as f64;
_15 = [13982037603346627334_u64];
_4 = !9223372036854775807_isize;
_4 = -(-9223372036854775808_isize);
_7.0 = _3;
_13 = [16367278521565702896_u64];
RET = _4 as f64;
_9 = 62067_u16 << _12;
_8 = _2;
RET = _11;
_6 = _3;
_7 = (_3,);
_7 = (_6,);
_11 = 13966314132980206033_usize as f64;
_17 = _2;
_8 = _17;
Goto(bb5)
}
bb5 = {
_6 = _3 + _7.0;
_7 = (_6,);
RET = -_11;
_6 = _4 as f32;
_17 = _8;
_2 = _8;
_4 = 8784946904987133557_i64 as isize;
Goto(bb6)
}
bb6 = {
_18 = 74_u8 as f64;
_10 = [7420683315297988517_i64,8400795055149752952_i64,(-8848642591923038777_i64),4813532073130417781_i64,(-2079905256961171482_i64)];
RET = _11 - _11;
_17 = _2;
_17 = _2;
_3 = _7.0 - _7.0;
_3 = _12 as f32;
_18 = -_11;
_4 = 126_isize;
_11 = RET;
_6 = _9 as f32;
_11 = RET;
_10 = [6318253167901712296_i64,2332017907683355659_i64,169362797982892400_i64,(-7539225050821063503_i64),(-217063691288793854_i64)];
_4 = 496489967_i32 as isize;
_7.0 = -_3;
_16 = core::ptr::addr_of_mut!(_9);
_12 = 709561190_u32;
_2 = _17;
_13 = [10933292120982252661_u64];
_7.0 = -_6;
match _12 {
0 => bb7,
1 => bb8,
709561190 => bb10,
_ => bb9
}
}
bb7 = {
_6 = _3 + _7.0;
_7 = (_6,);
RET = -_11;
_6 = _4 as f32;
_17 = _8;
_2 = _8;
_4 = 8784946904987133557_i64 as isize;
Goto(bb6)
}
bb8 = {
Return()
}
bb9 = {
_2 = _8;
_4 = 9223372036854775807_isize;
_3 = _6;
_8 = _2;
_6 = 31_i8 as f32;
_6 = 229_u8 as f32;
_12 = _4 as u32;
RET = -_11;
_9 = !20573_u16;
_10 = [637544702389634021_i64,5029723636594062946_i64,5894892201018671632_i64,(-4243913409058846905_i64),(-4916780336631159545_i64)];
_12 = _9 as u32;
_7.0 = _6;
_3 = RET as f32;
_13 = [10240001060610089954_u64];
RET = _11;
_12 = !1900608211_u32;
_7.0 = _6;
_8 = _2;
_11 = -RET;
_13 = [6239174371831805386_u64];
_1 = [_12,_12,_12,_12,_12,_12];
_13 = [14473069164122749966_u64];
_8 = _2;
_13 = [7315777601347258241_u64];
_4 = (-9223372036854775808_isize) + 44_isize;
RET = _11 + _11;
_2 = _8;
_4 = 4942703343857488362_u64 as isize;
Call(_12 = fn5(_10, Move(_5), _3, RET, _10, _10, _2, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_7 = (_3,);
_16 = core::ptr::addr_of_mut!((*_16));
_19 = !_4;
RET = _18;
_20 = _3 == _3;
_20 = _7.0 > _6;
_8 = _17;
_8 = _2;
_18 = -_11;
_3 = _7.0;
_7.0 = _3 - _3;
_21 = ((*_16),);
_17 = _8;
(*_16) = _21.0 >> _21.0;
(*_16) = !_21.0;
_4 = !_19;
_12 = !739819758_u32;
_12 = 1433479607_u32 + 3928383399_u32;
_18 = -_11;
(*_16) = 13274_i16 as u16;
_25 = [(-2129_i16),(-5655_i16),(-30706_i16)];
_2 = _8;
(*_16) = _21.0;
_4 = _19 & _19;
_4 = 20_i8 as isize;
_20 = true;
_21 = ((*_16),);
Call(_10 = fn7(_7.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = _11;
_20 = !false;
_17 = _2;
_11 = _12 as f64;
_15 = _13;
_17 = _8;
(*_16) = !_21.0;
_12 = _19 as u32;
_10 = [(-4615108687139776119_i64),(-6303106403429134473_i64),(-4588687014921808954_i64),(-7789963664595430847_i64),(-2558358314013194877_i64)];
_23 = (-9182_i16) as isize;
_18 = _11 - RET;
Goto(bb12)
}
bb12 = {
_16 = core::ptr::addr_of_mut!((*_16));
_2 = _8;
RET = _18 - _18;
_21 = (_9,);
_18 = (-27842643302921989851785364695850914748_i128) as f64;
_15 = [616111451655123871_u64];
_21 = (_9,);
_7.0 = 243154837295349038233271424664724487118_u128 as f32;
_26 = 1722889722_i32 & 1907071758_i32;
_18 = _11 * RET;
_11 = -_18;
_1 = [_12,_12,_12,_12,_12,_12];
_15 = [1921739493410332859_u64];
_4 = _23 | _23;
_4 = 4_usize as isize;
_8 = _17;
_19 = _23;
_7 = (_3,);
RET = _18;
_1 = [_12,_12,_12,_12,_12,_12];
_28 = [3061954369902713950_u64];
_4 = _19;
_7.0 = _6 * _3;
_20 = !false;
_6 = -_7.0;
_26 = (-105772449_i32);
_2 = _17;
_21.0 = _9;
Goto(bb13)
}
bb13 = {
_15 = [2356604499053501348_u64];
_17 = _8;
_21.0 = (-3905288617688176967_i64) as u16;
_7.0 = _6 * _6;
_19 = _23;
(*_16) = _21.0;
_3 = _7.0 + _7.0;
(*_16) = !_21.0;
_29 = [6358826727940549691_i64,(-6951061696944729222_i64),(-2333779080597728494_i64),(-740652569117098183_i64),(-4604721245429255691_i64)];
_11 = _26 as f64;
_18 = _7.0 as f64;
_25 = [(-31522_i16),4003_i16,5118_i16];
_29 = [(-767623266953286732_i64),8475686146871735682_i64,(-2565939278628941665_i64),(-2169025698716504078_i64),8206667878472765664_i64];
_7.0 = -_3;
(*_16) = _21.0 | _21.0;
_26 = 2048608620_i32 - (-1957122119_i32);
_19 = (*_16) as isize;
_29 = [9126024585298372358_i64,(-5041046590015547207_i64),(-1735431111273297499_i64),220246082889695140_i64,(-7464381409234408827_i64)];
_16 = core::ptr::addr_of_mut!(_31.0);
_31.0 = !_21.0;
_7 = (_3,);
_19 = _23 * _4;
_1 = [_12,_12,_12,_12,_12,_12];
_13 = [6367055326772329952_u64];
_4 = (-43_i8) as isize;
_15 = [18150754797678602134_u64];
Goto(bb14)
}
bb14 = {
_6 = (-2204_i16) as f32;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(4_usize, 10_usize, Move(_10), 21_usize, Move(_21), 19_usize, Move(_19), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(4_usize, 25_usize, Move(_25), 17_usize, Move(_17), 1_usize, Move(_1), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(4_usize, 2_usize, Move(_2), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [i64; 5],mut _2: *const [u16; 7],mut _3: f32,mut _4: f64,mut _5: [i64; 5],mut _6: [i64; 5],mut _7: char,mut _8: isize) -> u32 {
mir! {
type RET = u32;
let _9: char;
let _10: char;
let _11: f32;
let _12: ((u128, u32, i32),);
let _13: *const [i64; 5];
let _14: ();
let _15: ();
{
RET = 3414043824_u32 << _8;
_5 = [(-681227516315922833_i64),(-6467521454341082613_i64),6913037820989322229_i64,1236319025234929227_i64,1311218053856724897_i64];
_3 = 17696557655365644520_usize as f32;
_6 = [1388410921289663210_i64,8070088538733933382_i64,(-4546763003780575418_i64),2262680523560409719_i64,(-6128116755044837101_i64)];
RET = 270836823977621366254228871402195641441_u128 as u32;
Goto(bb1)
}
bb1 = {
_9 = _7;
_7 = _9;
_8 = (-9223372036854775808_isize) + 89_isize;
_9 = _7;
_1 = _5;
_1 = [4557154680454846703_i64,1399091879434811009_i64,7033911538534414632_i64,(-7599502332896217311_i64),1646989357894792835_i64];
RET = !3151098166_u32;
_1 = _6;
_8 = 26387752645134983971629566330690880508_i128 as isize;
_8 = -(-9223372036854775808_isize);
_9 = _7;
_9 = _7;
_4 = 16312845376720961655_u64 as f64;
_10 = _9;
_6 = [(-2246189017425035152_i64),8432979227901235005_i64,(-8068270596985975026_i64),(-6402425142380793776_i64),(-7953773033752529460_i64)];
_7 = _10;
_3 = (-93_i8) as f32;
_4 = 9527_i16 as f64;
_4 = 81_u8 as f64;
_11 = _3;
_1 = [8564352027451447047_i64,(-6985337047194765875_i64),(-3542733247132299556_i64),(-8785791824067654117_i64),6043259079744009116_i64];
RET = 2927395691_u32;
_1 = _5;
_10 = _7;
Call(_12.0 = fn6(_6, _6, _5, _6, Move(_2), RET, RET, _9, _6, _11, _6, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !_12.0.1;
RET = _12.0.0 as u32;
Goto(bb3)
}
bb3 = {
Call(_14 = dump_var(5_usize, 9_usize, Move(_9), 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i64; 5],mut _2: [i64; 5],mut _3: [i64; 5],mut _4: [i64; 5],mut _5: *const [u16; 7],mut _6: u32,mut _7: u32,mut _8: char,mut _9: [i64; 5],mut _10: f32,mut _11: [i64; 5],mut _12: f32) -> (u128, u32, i32) {
mir! {
type RET = (u128, u32, i32);
let _13: f64;
let _14: Adt52;
let _15: u128;
let _16: (Adt43, char, Adt39, u16);
let _17: (&'static (*const i32, [i16; 3], usize, bool), [i8; 4], i64);
let _18: [u64; 7];
let _19: i32;
let _20: ((u128, u32, i32),);
let _21: f32;
let _22: Adt68;
let _23: Adt52;
let _24: f64;
let _25: ();
let _26: ();
{
RET = (331886135505462972870866867053057984815_u128, _6, 289820765_i32);
RET.0 = 136835839665019795245634459163441144837_u128 & 256844933310221981337890818229756545146_u128;
_7 = !_6;
_11 = _4;
RET.0 = 117741516734841078282761656492526433454_i128 as u128;
RET.1 = _6;
RET.2 = -(-585004617_i32);
RET = (246730510367953044675196197537162100547_u128, _7, 10956764_i32);
RET.0 = !96818544780279908549672715225450989378_u128;
RET.0 = 336512577437338540873195324561745283676_u128 | 132531221689942810801056861090691862510_u128;
_13 = _6 as f64;
_10 = _12 * _12;
_11 = _4;
_12 = -_10;
RET.1 = !_6;
RET = (174261737138266994303332151896639312146_u128, _6, 1853767451_i32);
Goto(bb1)
}
bb1 = {
_3 = _2;
_16.0.fld4.0 = 3342142811942598524_i64 as u16;
_16.0.fld2 = (false, (-104906601899996770422776247890257365314_i128), true, (-9223372036854775808_isize));
_16.0.fld3.0.0.1 = core::ptr::addr_of_mut!(_16.0.fld2.1);
_16.0.fld0 = _16.0.fld2.2 | _16.0.fld2.0;
_20.0.0 = RET.0;
_16.0.fld3.0.0.1 = core::ptr::addr_of_mut!(_16.0.fld2.1);
_16.0.fld4.1 = _16.0.fld0;
_8 = '\u{5f409}';
_22.fld1.3 = _20.0.0 | RET.0;
_16.0.fld3.0.0.2 = !(-50_i8);
_16.0.fld3.0.0.0 = _12 as f64;
_16.0.fld2.1 = !159343849759737153963608598463968343280_i128;
RET.0 = _22.fld1.3 + _22.fld1.3;
_16.1 = _8;
_22.fld1.0.2 = _16.1 as i8;
Goto(bb2)
}
bb2 = {
Call(_25 = dump_var(6_usize, 4_usize, Move(_4), 9_usize, Move(_9), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: f32) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _2: f32;
let _3: *mut bool;
let _4: f32;
let _5: (u16, bool, Adt39, [u16; 8]);
let _6: isize;
let _7: isize;
let _8: [usize; 2];
let _9: f64;
let _10: u128;
let _11: isize;
let _12: u64;
let _13: i32;
let _14: isize;
let _15: (*const i32, f64, (f32,), f32);
let _16: *mut i128;
let _17: f64;
let _18: [isize; 2];
let _19: bool;
let _20: isize;
let _21: (f64, *mut i128, i8, char);
let _22: &'static [u16; 8];
let _23: bool;
let _24: Adt31;
let _25: [i8; 4];
let _26: [usize; 1];
let _27: *const (*const i32, f64, (f32,), f32);
let _28: ((f64, *mut i128, i8, char), bool, [u16; 7], u128);
let _29: f32;
let _30: ();
let _31: ();
{
RET = [1936068300455934410_i64,4603233198684831768_i64,7490251068211254227_i64,1015528223537937802_i64,1001766275214295204_i64];
RET = [(-324100475706192485_i64),(-5301718454603772114_i64),8954204353622543877_i64,3182564987135289995_i64,(-8392682246973506785_i64)];
RET = [7043001456276819225_i64,6907251022128808556_i64,6779660623547126304_i64,8844958336043821457_i64,(-729425236473885104_i64)];
RET = [(-1473916867327505174_i64),735294436391043917_i64,286457246887531152_i64,1831148858867106955_i64,7670345306657670650_i64];
_2 = -_1;
RET = [(-7972191321166896966_i64),(-1041576414282051304_i64),(-7069595741399415840_i64),(-1301298068304790448_i64),(-8580136808730306698_i64)];
_2 = _1 + _1;
_2 = -_1;
Goto(bb1)
}
bb1 = {
_1 = -_2;
_2 = _1 - _1;
RET = [7250782445980732530_i64,(-7755835327475016344_i64),1971828129286198091_i64,8896444422177909210_i64,(-2674131770970414456_i64)];
_4 = _2 + _2;
_4 = _1 - _2;
RET = [4882162353194161678_i64,(-6469442032199674115_i64),5160761106323925715_i64,3421171754587191932_i64,(-4174583036507738768_i64)];
_5.0 = 5869_u16 >> 3467233059488770649_u64;
_6 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_1 = 0_usize as f32;
_1 = _2 * _2;
_6 = 123_i8 as isize;
_1 = _2 - _4;
RET = [6634540438320004632_i64,(-353954981137853170_i64),2882664572157778024_i64,3372455145644973221_i64,(-685931932750284832_i64)];
_2 = -_4;
RET = [1732203158610863302_i64,(-1728470286467743599_i64),(-3068263357679073372_i64),9197197748112843557_i64,2681640670357651821_i64];
_1 = _2;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
Goto(bb2)
}
bb2 = {
_5.1 = !false;
_3 = core::ptr::addr_of_mut!(_5.1);
RET = [8666136449267567007_i64,(-3490091342629739269_i64),1587242591791639314_i64,4219144089361541859_i64,(-3042898630771351928_i64)];
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb3)
}
bb3 = {
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_2 = _1 * _4;
_8 = [3_usize,9199557121967744988_usize];
_5.0 = 40284_u16;
_5.1 = false;
Goto(bb4)
}
bb4 = {
RET = [(-1505622609783129089_i64),(-74576354575885068_i64),(-3129486020076946840_i64),1464045234561366238_i64,(-8487122993113328815_i64)];
_7 = !_6;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_2 = _1 * _1;
_10 = 36228548050967441647472827292822505808_u128 - 47642952243433092125031380043691819271_u128;
_2 = -_1;
_10 = 3_usize as u128;
_10 = 292103086963472372266738933121992299659_u128;
_1 = 10001_i16 as f32;
match _10 {
0 => bb1,
292103086963472372266738933121992299659 => bb5,
_ => bb2
}
}
bb5 = {
_11 = !_7;
(*_3) = _2 == _2;
_1 = _11 as f32;
_6 = !_7;
_6 = _11 + _11;
_4 = _2 + _2;
RET = [(-2854548827867160094_i64),(-4301768169642662905_i64),2011178030294341744_i64,(-898231138956618953_i64),3212341464442380953_i64];
Goto(bb6)
}
bb6 = {
_15.0 = core::ptr::addr_of!(_13);
_6 = _7 * _7;
_5.0 = 61971_u16;
_5.1 = !true;
_15.2 = (_4,);
_1 = _11 as f32;
_15.1 = _5.0 as f64;
_4 = _15.2.0;
_15.0 = core::ptr::addr_of!(_13);
_13 = (-1699263832_i32);
_12 = 119582197571838383_u64;
RET = [(-4675503455104881666_i64),(-5021545415523355419_i64),(-5788735038754298062_i64),4439490168485747002_i64,(-4070884624943603385_i64)];
_15.3 = _5.0 as f32;
RET = [892787616702071354_i64,(-1710043105465702724_i64),(-1149928494815896768_i64),(-5870048278199693382_i64),6152178418238764247_i64];
(*_3) = _2 != _4;
_14 = -_11;
Goto(bb7)
}
bb7 = {
_15.1 = _13 as f64;
_7 = (-11881_i16) as isize;
_5.1 = true | true;
_5.0 = 30386_u16 >> _10;
_14 = _11 - _7;
_1 = _4 - _15.2.0;
(*_3) = _15.2.0 > _4;
_12 = !11899065242363291183_u64;
_15.0 = core::ptr::addr_of!(_13);
_4 = _15.2.0;
_21.3 = '\u{78b8b}';
_18 = [_14,_7];
_9 = _15.1 * _15.1;
_8 = [16257106664820168181_usize,7_usize];
_19 = !(*_3);
(*_3) = _2 == _1;
_9 = _15.1;
_20 = !_11;
_20 = _7;
_20 = (-81_i8) as isize;
_13 = 1900386007_i32;
_10 = 2248902320_u32 as u128;
_15.2 = (_4,);
match _13 {
0 => bb3,
1 => bb6,
2 => bb8,
1900386007 => bb10,
_ => bb9
}
}
bb8 = {
RET = [(-1505622609783129089_i64),(-74576354575885068_i64),(-3129486020076946840_i64),1464045234561366238_i64,(-8487122993113328815_i64)];
_7 = !_6;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_2 = _1 * _1;
_10 = 36228548050967441647472827292822505808_u128 - 47642952243433092125031380043691819271_u128;
_2 = -_1;
_10 = 3_usize as u128;
_10 = 292103086963472372266738933121992299659_u128;
_1 = 10001_i16 as f32;
match _10 {
0 => bb1,
292103086963472372266738933121992299659 => bb5,
_ => bb2
}
}
bb9 = {
_1 = -_2;
_2 = _1 - _1;
RET = [7250782445980732530_i64,(-7755835327475016344_i64),1971828129286198091_i64,8896444422177909210_i64,(-2674131770970414456_i64)];
_4 = _2 + _2;
_4 = _1 - _2;
RET = [4882162353194161678_i64,(-6469442032199674115_i64),5160761106323925715_i64,3421171754587191932_i64,(-4174583036507738768_i64)];
_5.0 = 5869_u16 >> 3467233059488770649_u64;
_6 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_1 = 0_usize as f32;
_1 = _2 * _2;
_6 = 123_i8 as isize;
_1 = _2 - _4;
RET = [6634540438320004632_i64,(-353954981137853170_i64),2882664572157778024_i64,3372455145644973221_i64,(-685931932750284832_i64)];
_2 = -_4;
RET = [1732203158610863302_i64,(-1728470286467743599_i64),(-3068263357679073372_i64),9197197748112843557_i64,2681640670357651821_i64];
_1 = _2;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
Goto(bb2)
}
bb10 = {
_23 = _5.1;
_17 = _15.1 - _9;
_8 = [6593339643707867830_usize,17515267696856230267_usize];
_6 = !_14;
_22 = &_5.3;
_6 = _14 + _14;
_14 = !_6;
_11 = _6 - _14;
RET = [(-6509877101623121849_i64),2671722875841776612_i64,4902906645598568144_i64,4019149074616090740_i64,(-2715685368330276655_i64)];
_7 = _5.0 as isize;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_21.2 = !(-65_i8);
_26 = [6_usize];
_3 = core::ptr::addr_of_mut!(_19);
_10 = _5.0 as u128;
_21.2 = (-102106478001426736536568330985195239725_i128) as i8;
_12 = _21.2 as u64;
_15.2 = (_1,);
_28.3 = 3902722867_u32 as u128;
match _13 {
1900386007 => bb12,
_ => bb11
}
}
bb11 = {
RET = [(-1505622609783129089_i64),(-74576354575885068_i64),(-3129486020076946840_i64),1464045234561366238_i64,(-8487122993113328815_i64)];
_7 = !_6;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_2 = _1 * _1;
_10 = 36228548050967441647472827292822505808_u128 - 47642952243433092125031380043691819271_u128;
_2 = -_1;
_10 = 3_usize as u128;
_10 = 292103086963472372266738933121992299659_u128;
_1 = 10001_i16 as f32;
match _10 {
0 => bb1,
292103086963472372266738933121992299659 => bb5,
_ => bb2
}
}
bb12 = {
RET = [3039051494510530365_i64,1778361287094505446_i64,872465583343595821_i64,3599873828411771560_i64,8902936129695043804_i64];
_9 = -_17;
_20 = !_6;
_28.2 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_15.0 = core::ptr::addr_of!(_13);
match _13 {
0 => bb1,
1 => bb7,
2 => bb13,
1900386007 => bb15,
_ => bb14
}
}
bb13 = {
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_2 = _1 * _4;
_8 = [3_usize,9199557121967744988_usize];
_5.0 = 40284_u16;
_5.1 = false;
Goto(bb4)
}
bb14 = {
_23 = _5.1;
_17 = _15.1 - _9;
_8 = [6593339643707867830_usize,17515267696856230267_usize];
_6 = !_14;
_22 = &_5.3;
_6 = _14 + _14;
_14 = !_6;
_11 = _6 - _14;
RET = [(-6509877101623121849_i64),2671722875841776612_i64,4902906645598568144_i64,4019149074616090740_i64,(-2715685368330276655_i64)];
_7 = _5.0 as isize;
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_21.2 = !(-65_i8);
_26 = [6_usize];
_3 = core::ptr::addr_of_mut!(_19);
_10 = _5.0 as u128;
_21.2 = (-102106478001426736536568330985195239725_i128) as i8;
_12 = _21.2 as u64;
_15.2 = (_1,);
_28.3 = 3902722867_u32 as u128;
match _13 {
1900386007 => bb12,
_ => bb11
}
}
bb15 = {
_15.0 = core::ptr::addr_of!(_13);
_28.1 = _23;
_9 = -_15.1;
_19 = !_23;
_21.3 = '\u{6c259}';
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_15.0 = core::ptr::addr_of!(_13);
_28.1 = !(*_3);
_2 = _15.2.0 + _4;
_27 = core::ptr::addr_of!(_15);
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(7_usize, 10_usize, Move(_10), 23_usize, Move(_23), 6_usize, Move(_6), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(7_usize, 11_usize, Move(_11), 18_usize, Move(_18), 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [u32; 6],mut _2: isize,mut _3: char,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64) -> *const [u16; 7] {
mir! {
type RET = *const [u16; 7];
let _8: *mut [u64; 7];
let _9: ((u128, u32, i32),);
let _10: isize;
let _11: [i16; 3];
let _12: ((i64, (*const i32, [i16; 3], usize, bool), (bool,)),);
let _13: f64;
let _14: (u128, u32, i32);
let _15: isize;
let _16: (i32, (f32,), *mut u16);
let _17: f32;
let _18: *mut u16;
let _19: f64;
let _20: (*const i32, f64, (f32,), f32);
let _21: f32;
let _22: isize;
let _23: *mut (bool,);
let _24: isize;
let _25: &'static (*const i32, [i16; 3], usize, bool);
let _26: (Adt43, char, Adt39, u16);
let _27: i8;
let _28: f64;
let _29: *const i32;
let _30: *mut u16;
let _31: f64;
let _32: &'static i64;
let _33: (i8, (i64, (*const i32, [i16; 3], usize, bool), (bool,)));
let _34: u8;
let _35: [char; 7];
let _36: ((u128, u32, i32),);
let _37: i32;
let _38: isize;
let _39: isize;
let _40: (u16,);
let _41: char;
let _42: (u128, u32, i32);
let _43: ();
let _44: ();
{
_1 = [596158518_u32,897679899_u32,4003430543_u32,103225306_u32,1666513674_u32,3795954094_u32];
_6 = _7 + _5;
_5 = -_6;
_3 = '\u{44204}';
_6 = _7;
_1 = [2229364182_u32,789890445_u32,2206807547_u32,2630040084_u32,4005356711_u32,1538300375_u32];
_6 = _5 + _5;
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !46_isize;
_2 = (-9223372036854775808_isize);
_5 = -_6;
_2 = (-81_isize) >> 34953_u16;
_9.0.0 = (-1435274952_i32) as u128;
_7 = _4;
_1 = [3299704533_u32,3264785913_u32,2210031786_u32,2762623847_u32,734865136_u32,3115313759_u32];
_6 = _4;
_9.0.2 = !751400145_i32;
_7 = _6 - _4;
_9.0 = (186414964388964295554776694409727126377_u128, 2276865697_u32, (-790209343_i32));
_1 = [_9.0.1,_9.0.1,_9.0.1,_9.0.1,_9.0.1,_9.0.1];
Goto(bb2)
}
bb2 = {
_3 = '\u{558ac}';
_7 = _6 + _6;
_9.0.2 = 1655791404_i32 & 566907197_i32;
_10 = _2;
_9.0.2 = -(-825994808_i32);
_3 = '\u{ac9ff}';
_6 = _9.0.1 as f64;
_9.0.0 = !153051439859367745778575137662506236856_u128;
_9.0.0 = 278210543425829389920491039032362118127_u128 + 260743681951648820853396660193593483010_u128;
_12.0.2.0 = !false;
_3 = '\u{756cd}';
_12.0.0 = -7620074167982507546_i64;
_12.0.2.0 = true;
_11 = [(-22630_i16),17376_i16,15195_i16];
Goto(bb3)
}
bb3 = {
_12.0.1.0 = core::ptr::addr_of!(_9.0.2);
_4 = -_7;
_6 = -_4;
_12.0.1.0 = core::ptr::addr_of!(_9.0.2);
_12.0.1.1 = [(-7182_i16),356_i16,(-19384_i16)];
_13 = -_6;
_1 = [_9.0.1,_9.0.1,_9.0.1,_9.0.1,_9.0.1,_9.0.1];
_12.0.0 = 74_i8 as i64;
_12.0.1.1 = _11;
_2 = !_10;
Call(_12.0.1 = fn9(_12.0.2, _7, _1, _4, _4, _7, _13, _6, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = '\u{d1af3}';
_7 = -_6;
_12.0.0 = 6261191427721908951_i64;
_2 = -_10;
_2 = (-121_i8) as isize;
_12.0.1.0 = core::ptr::addr_of!(_14.2);
_7 = 45180510821912774134867518723299834148_i128 as f64;
_5 = _6 + _13;
_14.0 = _10 as u128;
_14.2 = -_9.0.2;
Goto(bb5)
}
bb5 = {
_17 = _14.2 as f32;
_12.0.1.1 = [(-26105_i16),(-17993_i16),8644_i16];
_3 = '\u{8172b}';
_12.0.2 = (_12.0.1.3,);
Goto(bb6)
}
bb6 = {
_12.0.1.1 = _11;
_9.0.1 = 2459552963_u32 >> _14.0;
_4 = _6 - _13;
_7 = -_4;
_12.0.1.2 = _9.0.2 as usize;
_12.0.0 = !(-3441534184250458974_i64);
_10 = 79135071802175506711320186060478789666_i128 as isize;
_12.0.0 = _12.0.2.0 as i64;
Call(_18 = fn10(Move(_12.0.1), _6, _12.0.2.0, _4, _6, _12.0.2.0, _12.0.2.0, _13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16.1.0 = _17;
_12.0.1.2 = _9.0.1 as usize;
_14 = _9.0;
_10 = _2;
_19 = _5 + _4;
_15 = _12.0.2.0 as isize;
_16.2 = Move(_18);
_11 = [(-7297_i16),(-11179_i16),31718_i16];
_9.0.2 = _14.2;
_20.0 = core::ptr::addr_of!(_14.2);
_16.1.0 = 38963_u16 as f32;
_5 = -_4;
_12.0.1.3 = _12.0.2.0;
Call(_6 = fn17(_7, _12.0.2.0, Move(_16.2), _7, _19, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12.0.2 = (_12.0.1.3,);
_21 = _16.1.0;
_9.0.1 = _14.1;
_20.2 = (_16.1.0,);
_20.1 = _19;
_11 = [(-22170_i16),(-30423_i16),6322_i16];
_16.0 = _9.0.2;
Goto(bb9)
}
bb9 = {
_12.0.1 = (Move(_20.0), _11, 9165237408180480209_usize, _12.0.2.0);
_7 = _19;
_9 = (_14,);
_16.0 = _9.0.1 as i32;
_13 = -_20.1;
_9.0.0 = _14.0 + _14.0;
_9.0 = (_14.0, _14.1, _16.0);
_9.0.1 = _14.1;
_23 = core::ptr::addr_of_mut!(_12.0.2);
_15 = _10;
_9.0.0 = _16.1.0 as u128;
_12.0.1.0 = core::ptr::addr_of!(_9.0.2);
_23 = core::ptr::addr_of_mut!((*_23));
_24 = _2;
RET = core::ptr::addr_of!(_26.0.fld3.0.2);
(*RET) = [47016_u16,16261_u16,2695_u16,59020_u16,13927_u16,12020_u16,2509_u16];
_9.0.1 = _14.1 >> _16.0;
_26.1 = _3;
_26.0.fld4.1 = _12.0.2.0;
_26.0.fld3.0.0.0 = _13;
match _12.0.1.2 {
0 => bb5,
9165237408180480209 => bb11,
_ => bb10
}
}
bb10 = {
_12.0.1.1 = _11;
_9.0.1 = 2459552963_u32 >> _14.0;
_4 = _6 - _13;
_7 = -_4;
_12.0.1.2 = _9.0.2 as usize;
_12.0.0 = !(-3441534184250458974_i64);
_10 = 79135071802175506711320186060478789666_i128 as isize;
_12.0.0 = _12.0.2.0 as i64;
Call(_18 = fn10(Move(_12.0.1), _6, _12.0.2.0, _4, _6, _12.0.2.0, _12.0.2.0, _13), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_26.0.fld3.0.0.3 = _26.1;
_27 = _14.0 as i8;
_2 = 14060_u16 as isize;
(*_23).0 = !_26.0.fld4.1;
_26.0.fld2 = ((*_23).0, (-145693079877556545258338641189389851020_i128), (*_23).0, _24);
_12.0.1.2 = !11104848089602094532_usize;
_24 = _17 as isize;
(*_23) = (_26.0.fld2.2,);
_10 = _26.0.fld2.3 & _26.0.fld2.3;
_26.0.fld3.0.0.1 = core::ptr::addr_of_mut!(_26.0.fld2.1);
_12.0.1.2 = 13970692950025876937_usize;
(*_23).0 = _26.0.fld2.2;
_11 = [10159_i16,4504_i16,12813_i16];
_9.0.2 = _14.2 << _12.0.0;
(*RET) = [52230_u16,30813_u16,45773_u16,19250_u16,33221_u16,30458_u16,17286_u16];
_26.1 = _26.0.fld3.0.0.3;
_12.0.1.0 = core::ptr::addr_of!(_14.2);
_26.0.fld2.0 = _26.0.fld4.1;
_33.1.2.0 = !(*_23).0;
_25 = &_12.0.1;
_33.1.1 = Move((*_25));
_9.0.1 = _14.1;
_19 = 17634614452417266749_u64 as f64;
_26.0.fld2.3 = _24 ^ _15;
_26.0.fld3.0.0.2 = _14.0 as i8;
Goto(bb12)
}
bb12 = {
_9.0.1 = 36_u8 as u32;
_24 = _26.0.fld2.3;
_12.0.1.2 = _33.1.1.2;
_12.0.1.1 = [(-10575_i16),(-2804_i16),(-24962_i16)];
_19 = _26.0.fld2.1 as f64;
_5 = -_6;
_26.0.fld0 = (*_23).0 > _26.0.fld2.2;
_26.0.fld4.0 = _10 as u16;
(*RET) = [_26.0.fld4.0,_26.0.fld4.0,_26.0.fld4.0,_26.0.fld4.0,_26.0.fld4.0,_26.0.fld4.0,_26.0.fld4.0];
_9.0.1 = _33.1.1.2 as u32;
_33.1.2.0 = !_26.0.fld0;
_9.0.2 = -_16.0;
_7 = (-8558_i16) as f64;
_36.0.1 = _14.1;
_12.0 = (6996645330698785422_i64, Move(_33.1.1), _33.1.2);
_16.2 = core::ptr::addr_of_mut!(_26.3);
_12.0.2.0 = _26.0.fld2.2;
_16.2 = core::ptr::addr_of_mut!(_26.3);
_33.1.2.0 = !_12.0.2.0;
_32 = &_12.0.0;
_26.1 = _3;
_16.0 = _9.0.2 * _9.0.2;
Goto(bb13)
}
bb13 = {
_26.0.fld3.0.1 = _26.0.fld4.1;
_20.2 = (_17,);
_9.0.2 = _26.0.fld3.0.0.2 as i32;
Goto(bb14)
}
bb14 = {
_33.0 = _16.0 as i8;
_19 = _13;
_33.0 = _26.0.fld3.0.0.3 as i8;
_36.0.0 = 97_u8 as u128;
_26.3 = _26.0.fld4.0 & _26.0.fld4.0;
_33.1.2 = (_26.0.fld2.2,);
_12.0.1.0 = core::ptr::addr_of!(_36.0.2);
_29 = core::ptr::addr_of!(_14.2);
_33.1.1.0 = core::ptr::addr_of!(_14.2);
_26.0.fld1 = Adt28::Variant1 { fld0: Move(_29),fld1: _3,fld2: 231_u8,fld3: _16.1,fld4: Move(_26.0.fld3.0.0.1),fld5: _36.0.0,fld6: _26.3 };
_33.1.2 = (_26.0.fld2.2,);
_26.0.fld2.2 = (*_32) >= _12.0.0;
_26.0.fld4.2 = Adt39::Variant1 { fld0: Move(_12.0.1),fld1: _26.1,fld2: 88_u8,fld3: Move(Field::<*mut i128>(Variant(_26.0.fld1, 1), 4)),fld4: Field::<u16>(Variant(_26.0.fld1, 1), 6),fld5: _16.0,fld6: (*_23),fld7: _26.0.fld2.1 };
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(8_usize, 11_usize, Move(_11), 2_usize, Move(_2), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(8_usize, 1_usize, Move(_1), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: (bool,),mut _2: f64,mut _3: [u32; 6],mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64,mut _8: f64,mut _9: f64) -> (*const i32, [i16; 3], usize, bool) {
mir! {
type RET = (*const i32, [i16; 3], usize, bool);
let _10: i32;
let _11: [isize; 2];
let _12: ((u128, u32, i32),);
let _13: u32;
let _14: i64;
let _15: ();
let _16: ();
{
_1.0 = !false;
_3 = [1778614239_u32,1881131575_u32,3751889958_u32,665656970_u32,4207964215_u32,1345385914_u32];
RET.2 = (-1693043322140000482_i64) as usize;
_10 = (-752590875_i32) * (-1756908414_i32);
RET.1 = [21892_i16,11532_i16,(-19426_i16)];
RET.3 = !_1.0;
RET.1 = [(-14793_i16),(-22870_i16),(-29014_i16)];
_4 = -_6;
RET.2 = !7_usize;
RET.3 = _4 < _9;
_13 = !1578177441_u32;
_12.0.1 = _13;
RET.0 = core::ptr::addr_of!(_10);
_5 = _2 - _7;
_12.0.2 = -_10;
_12.0.2 = !_10;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(9_usize, 3_usize, Move(_3), 1_usize, Move(_1), 16_usize, _16, 16_usize, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (*const i32, [i16; 3], usize, bool),mut _2: f64,mut _3: bool,mut _4: f64,mut _5: f64,mut _6: bool,mut _7: bool,mut _8: f64) -> *mut u16 {
mir! {
type RET = *mut u16;
let _9: ((i32, (f32,), *mut u16), ((i64, (*const i32, [i16; 3], usize, bool), (bool,)),), ((f32,),), (i8, *mut bool, *mut (bool,), Adt43));
let _10: (u128, u32, i32);
let _11: isize;
let _12: (((f64, *mut i128, i8, char), bool, [u16; 7], u128),);
let _13: u128;
let _14: isize;
let _15: (u16, bool, Adt39, [u16; 8]);
let _16: f64;
let _17: [isize; 3];
let _18: bool;
let _19: (i8, (i64, (*const i32, [i16; 3], usize, bool), (bool,)));
let _20: i128;
let _21: (((f64, *mut i128, i8, char), bool, [u16; 7], u128),);
let _22: Adt31;
let _23: ((f32,),);
let _24: u8;
let _25: [isize; 3];
let _26: char;
let _27: [isize; 2];
let _28: Adt43;
let _29: [usize; 2];
let _30: f32;
let _31: &'static (*const i32, [i16; 3], usize, bool);
let _32: usize;
let _33: isize;
let _34: isize;
let _35: char;
let _36: bool;
let _37: [u64; 1];
let _38: isize;
let _39: isize;
let _40: isize;
let _41: [usize; 1];
let _42: i64;
let _43: &'static i64;
let _44: (i32, (f32,), *mut u16);
let _45: bool;
let _46: *mut (bool,);
let _47: ((f32,),);
let _48: isize;
let _49: *mut isize;
let _50: u128;
let _51: &'static i64;
let _52: [u32; 6];
let _53: [u64; 1];
let _54: *mut u32;
let _55: *mut u32;
let _56: ((f32,), *mut u16, i16, (*const i32, [i16; 3], usize, bool));
let _57: f64;
let _58: i32;
let _59: ();
let _60: ();
{
_3 = _7 > _7;
_7 = _3;
_9.3.3.fld3.0.3 = _1.3 as u128;
_9.3.2 = core::ptr::addr_of_mut!(_9.1.0.2);
_9.1.0.2.0 = !_1.3;
_9.3.3.fld3.0.1 = !_1.3;
_9.3.3.fld2.0 = _9.3.3.fld3.0.3 != _9.3.3.fld3.0.3;
_9.1.0.1.1 = [25072_i16,(-4326_i16),17199_i16];
_9.3.3.fld3.0.2 = [61375_u16,5857_u16,41902_u16,48306_u16,47921_u16,60457_u16,10199_u16];
Call(_9.3.3.fld3.0.3 = fn11(_7, _9.1.0.2.0, _9.3.3.fld3.0.1, _9.3.3.fld3.0.1, _1.3, _9.1.0.2.0, Move(_1), _4, _9.3.3.fld2.0, _5, Move(_9.3.2), _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.3.3.fld4.1 = _9.1.0.2.0 < _9.3.3.fld2.0;
_9.1.0.2 = (_6,);
_9.3.3.fld2.2 = _6 > _9.3.3.fld3.0.1;
_9.3.3.fld0 = _9.3.3.fld2.0 | _6;
_12.0.0.3 = '\u{12d9b}';
_1.3 = _7;
_9.1.0.0 = 3349931794703115650_i64;
_9.0.0 = _9.1.0.0 as i32;
match _9.1.0.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
3349931794703115650 => bb7,
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
_10 = (_9.3.3.fld3.0.3, 2887561609_u32, _9.0.0);
_9.3.2 = core::ptr::addr_of_mut!(_9.1.0.2);
_9.3.3.fld3.0.2 = [17456_u16,19451_u16,32389_u16,40335_u16,63943_u16,38873_u16,52579_u16];
RET = core::ptr::addr_of_mut!(_15.0);
_9.3.3.fld3.0.1 = _3;
_9.1.0.1.0 = core::ptr::addr_of!(_10.2);
_9.3.3.fld2.1 = (-119515470457087373917668799533116701926_i128);
_9.1.0.2.0 = _7 <= _9.3.3.fld2.2;
_5 = (-80_isize) as f64;
_9.2.0.0 = 3_usize as f32;
_11 = !(-9223372036854775808_isize);
_15.1 = _9.3.3.fld2.0;
_1.0 = Move(_9.1.0.1.0);
_9.1.0.2 = (_1.3,);
_9.3.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_9.3.3.fld2.1);
Goto(bb8)
}
bb8 = {
_19.1.1 = (Move(_1.0), _9.1.0.1.1, 12718494076403929373_usize, _15.1);
_9.3.3.fld2.3 = _11;
_10.2 = _9.0.0;
(*RET) = 111_i8 as u16;
_8 = _2;
_9.3.3.fld3.0.0.2 = 9_i8 - 41_i8;
_9.1.0.2.0 = _9.3.3.fld0 ^ _1.3;
_1.0 = core::ptr::addr_of!(_10.2);
_21.0.0 = (_8, Move(_9.3.3.fld3.0.0.1), _9.3.3.fld3.0.0.2, _12.0.0.3);
_9.3.3.fld2.2 = _1.3;
_9.3.3.fld4.1 = _1.3;
_12.0.0.2 = 225_u8 as i8;
_19.1.2 = (_9.3.3.fld0,);
_9.1.0.1.2 = !_19.1.1.2;
_9.1.0.0 = 5779515482974154304_i64 >> _19.1.1.2;
(*RET) = 63968_u16 << _9.3.3.fld3.0.3;
_1.3 = _9.3.3.fld2.2;
_1 = (Move(_19.1.1.0), _9.1.0.1.1, _19.1.1.2, _19.1.1.3);
_21.0.1 = _9.3.3.fld4.1 <= _9.3.3.fld4.1;
_9.0 = (_10.2, _9.2.0, Move(RET));
match _10.1 {
0 => bb9,
1 => bb10,
2 => bb11,
2887561609 => bb13,
_ => bb12
}
}
bb9 = {
_10 = (_9.3.3.fld3.0.3, 2887561609_u32, _9.0.0);
_9.3.2 = core::ptr::addr_of_mut!(_9.1.0.2);
_9.3.3.fld3.0.2 = [17456_u16,19451_u16,32389_u16,40335_u16,63943_u16,38873_u16,52579_u16];
RET = core::ptr::addr_of_mut!(_15.0);
_9.3.3.fld3.0.1 = _3;
_9.1.0.1.0 = core::ptr::addr_of!(_10.2);
_9.3.3.fld2.1 = (-119515470457087373917668799533116701926_i128);
_9.1.0.2.0 = _7 <= _9.3.3.fld2.2;
_5 = (-80_isize) as f64;
_9.2.0.0 = 3_usize as f32;
_11 = !(-9223372036854775808_isize);
_15.1 = _9.3.3.fld2.0;
_1.0 = Move(_9.1.0.1.0);
_9.1.0.2 = (_1.3,);
_9.3.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_9.3.3.fld2.1);
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
_28.fld4.2 = Adt39::Variant1 { fld0: Move(_1),fld1: _21.0.0.3,fld2: 184_u8,fld3: Move(_21.0.0.1),fld4: _15.0,fld5: _9.0.0,fld6: _9.1.0.2,fld7: _9.3.3.fld2.1 };
_24 = 16973046602471382671_u64 as u8;
_23.0 = (_9.2.0.0,);
_23.0 = _9.0.1;
_19.1.1.1 = Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0).1;
place!(Field::<(bool,)>(Variant(_28.fld4.2, 1), 6)).0 = _4 <= _4;
_17 = [_9.3.3.fld2.3,_11,_9.3.3.fld2.3];
_28.fld0 = _9.3.3.fld0 == _9.3.3.fld3.0.1;
_9.3.3.fld1 = Adt28::Variant0 { fld0: Field::<i128>(Variant(_28.fld4.2, 1), 7),fld1: _9.0.0,fld2: Move(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)),fld3: _9.0.1,fld4: Move(Field::<*mut i128>(Variant(_28.fld4.2, 1), 3)) };
SetDiscriminant(_9.3.3.fld1, 0);
match _10.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb12,
2887561609 => bb14,
_ => bb10
}
}
bb14 = {
_27 = [_9.3.3.fld2.3,_11];
_27 = [_9.3.3.fld2.3,_9.3.3.fld2.3];
_15.1 = !_9.3.3.fld2.0;
_28.fld2.0 = !_28.fld0;
_9.1.0.1.2 = _19.1.1.2;
_9.3.3.fld3.0.0.0 = _2;
_9.2.0 = (_9.0.1.0,);
_28.fld2.2 = !_9.1.0.2.0;
_1.1 = [29492_i16,24550_i16,8334_i16];
_2 = -_4;
_21.0.3 = !_9.3.3.fld3.0.3;
_40 = !_11;
_9.3.3.fld3.0.3 = _10.0 - _21.0.3;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)).2 = !_19.1.1.2;
match _19.1.1.2 {
0 => bb10,
1 => bb15,
12718494076403929373 => bb17,
_ => bb16
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
place!(Field::<(f32,)>(Variant(_9.3.3.fld1, 0), 3)).0 = _9.2.0.0 + _9.0.1.0;
_9.3.3.fld3.0.0.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_9.3.3.fld1, 0), 0)));
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_9.3.3.fld1, 0), 2)).2 = _23.0.0 as usize;
place!(Field::<i32>(Variant(_9.3.3.fld1, 0), 1)) = _9.0.0 << _10.1;
_9.3.3.fld2.1 = -Field::<i128>(Variant(_28.fld4.2, 1), 7);
_9.3.3.fld3.0.0.3 = Field::<char>(Variant(_28.fld4.2, 1), 1);
_28.fld3.0.0.0 = -_4;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)).3 = _7 & _28.fld2.2;
Goto(bb18)
}
bb18 = {
place!(Field::<i128>(Variant(_28.fld4.2, 1), 7)) = _2 as i128;
_9.2 = _23;
_41 = [_9.1.0.1.2];
_28.fld2.0 = !_28.fld0;
_9.3.3.fld2.2 = _28.fld2.2 & _9.1.0.2.0;
_28.fld4.3 = [Field::<u16>(Variant(_28.fld4.2, 1), 4),_15.0,Field::<u16>(Variant(_28.fld4.2, 1), 4),Field::<u16>(Variant(_28.fld4.2, 1), 4),_15.0,Field::<u16>(Variant(_28.fld4.2, 1), 4),Field::<u16>(Variant(_28.fld4.2, 1), 4),_15.0];
_1.2 = _19.1.1.2 % _9.1.0.1.2;
_9.2.0.0 = _21.0.3 as f32;
_21.0.3 = _9.3.3.fld3.0.3;
_12.0 = (Move(_9.3.3.fld3.0.0), _28.fld2.2, _9.3.3.fld3.0.2, _21.0.3);
_9.3.3.fld3.0 = (Move(_12.0.0), _9.3.3.fld2.0, _12.0.2, _10.0);
_19.1.1.2 = Field::<i128>(Variant(_28.fld4.2, 1), 7) as usize;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)).1 = [(-31821_i16),28065_i16,(-2030_i16)];
_28.fld4.0 = !_15.0;
place!(Field::<i128>(Variant(_9.3.3.fld1, 0), 0)) = Field::<i128>(Variant(_28.fld4.2, 1), 7);
_10.0 = _12.0.3 + _21.0.3;
_19.1.2.0 = _19.1.1.3;
_34 = Field::<u16>(Variant(_28.fld4.2, 1), 4) as isize;
_28.fld2.3 = _34 ^ _34;
_9.0.1.0 = _9.1.0.0 as f32;
_25 = _17;
_28.fld2.3 = _34 * _34;
_24 = _10.0 as u8;
_15.3 = _28.fld4.3;
_29 = [_1.2,_1.2];
place!(Field::<u8>(Variant(_28.fld4.2, 1), 2)) = _24 - _24;
_12.0.0 = Move(_9.3.3.fld3.0.0);
_21.0.0.1 = Move(_12.0.0.1);
_12.0.0 = Move(_21.0.0);
Goto(bb19)
}
bb19 = {
_23 = (_9.2.0,);
_35 = Field::<char>(Variant(_28.fld4.2, 1), 1);
_9.0.0 = !Field::<i32>(Variant(_9.3.3.fld1, 0), 1);
_1.3 = Field::<(bool,)>(Variant(_28.fld4.2, 1), 6).0;
_1.2 = Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0).2 - _19.1.1.2;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_9.3.3.fld1, 0), 2)).0 = core::ptr::addr_of!(_9.0.0);
_10.1 = !1920003214_u32;
_44.1.0 = _9.2.0.0;
place!(Field::<u8>(Variant(_28.fld4.2, 1), 2)) = _24 | _24;
_28.fld3 = Move(_12);
_9.3.3.fld3.0.1 = !_28.fld2.2;
_9.3.3.fld2 = (_9.3.3.fld0, Field::<i128>(Variant(_9.3.3.fld1, 0), 0), _1.3, _28.fld2.3);
_9.0.1.0 = -_23.0.0;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_9.3.3.fld1, 0), 2)).2 = _1.2 & Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0).2;
_19.1.1 = (Move(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_9.3.3.fld1, 0), 2).0), _9.1.0.1.1, _1.2, Field::<(bool,)>(Variant(_28.fld4.2, 1), 6).0);
place!(Field::<char>(Variant(_28.fld4.2, 1), 1)) = _28.fld3.0.0.3;
_47.0 = _23.0;
Goto(bb20)
}
bb20 = {
_9.3.3.fld3.0.0.1 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_9.3.3.fld1, 0), 0)));
place!(Field::<i32>(Variant(_9.3.3.fld1, 0), 1)) = _9.0.0;
_48 = _34;
_20 = _9.1.0.0 as i128;
_44.0 = !Field::<i32>(Variant(_9.3.3.fld1, 0), 1);
_9.1.0.1.0 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_9.3.3.fld1, 0), 1)));
place!(Field::<*mut i128>(Variant(_9.3.3.fld1, 0), 4)) = core::ptr::addr_of_mut!(_9.3.3.fld2.1);
_9.1.0.2 = Field::<(bool,)>(Variant(_28.fld4.2, 1), 6);
_42 = _10.1 as i64;
_24 = !Field::<u8>(Variant(_28.fld4.2, 1), 2);
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)).2 = _1.2 & _9.1.0.1.2;
_19.1.0 = _9.1.0.0;
_19.0 = _28.fld3.0.0.2;
_28.fld2 = _9.3.3.fld2;
Goto(bb21)
}
bb21 = {
_47.0 = (_9.2.0.0,);
_10 = (_21.0.3, 686492551_u32, _44.0);
_19.1.2 = (_28.fld2.0,);
_21.0.0.2 = -_28.fld3.0.0.2;
_54 = core::ptr::addr_of_mut!(_10.1);
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)).0 = Move(_9.1.0.1.0);
_13 = _21.0.3 * _21.0.3;
_40 = _48;
_1.2 = _19.1.1.3 as usize;
_52 = [_10.1,_10.1,(*_54),(*_54),(*_54),(*_54)];
_21.0 = Move(_28.fld3.0);
_21.0.0.3 = _35;
_9.3.3.fld1 = Adt28::Variant0 { fld0: _28.fld2.1,fld1: _44.0,fld2: Move(_19.1.1),fld3: _23.0,fld4: Move(_9.3.3.fld3.0.0.1) };
_9.1.0.1.2 = _1.2;
match (*_54) {
0 => bb17,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb19,
686492551 => bb23,
_ => bb22
}
}
bb22 = {
place!(Field::<i128>(Variant(_28.fld4.2, 1), 7)) = _2 as i128;
_9.2 = _23;
_41 = [_9.1.0.1.2];
_28.fld2.0 = !_28.fld0;
_9.3.3.fld2.2 = _28.fld2.2 & _9.1.0.2.0;
_28.fld4.3 = [Field::<u16>(Variant(_28.fld4.2, 1), 4),_15.0,Field::<u16>(Variant(_28.fld4.2, 1), 4),Field::<u16>(Variant(_28.fld4.2, 1), 4),_15.0,Field::<u16>(Variant(_28.fld4.2, 1), 4),Field::<u16>(Variant(_28.fld4.2, 1), 4),_15.0];
_1.2 = _19.1.1.2 % _9.1.0.1.2;
_9.2.0.0 = _21.0.3 as f32;
_21.0.3 = _9.3.3.fld3.0.3;
_12.0 = (Move(_9.3.3.fld3.0.0), _28.fld2.2, _9.3.3.fld3.0.2, _21.0.3);
_9.3.3.fld3.0 = (Move(_12.0.0), _9.3.3.fld2.0, _12.0.2, _10.0);
_19.1.1.2 = Field::<i128>(Variant(_28.fld4.2, 1), 7) as usize;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_28.fld4.2, 1), 0)).1 = [(-31821_i16),28065_i16,(-2030_i16)];
_28.fld4.0 = !_15.0;
place!(Field::<i128>(Variant(_9.3.3.fld1, 0), 0)) = Field::<i128>(Variant(_28.fld4.2, 1), 7);
_10.0 = _12.0.3 + _21.0.3;
_19.1.2.0 = _19.1.1.3;
_34 = Field::<u16>(Variant(_28.fld4.2, 1), 4) as isize;
_28.fld2.3 = _34 ^ _34;
_9.0.1.0 = _9.1.0.0 as f32;
_25 = _17;
_28.fld2.3 = _34 * _34;
_24 = _10.0 as u8;
_15.3 = _28.fld4.3;
_29 = [_1.2,_1.2];
place!(Field::<u8>(Variant(_28.fld4.2, 1), 2)) = _24 - _24;
_12.0.0 = Move(_9.3.3.fld3.0.0);
_21.0.0.1 = Move(_12.0.0.1);
_12.0.0 = Move(_21.0.0);
Goto(bb19)
}
bb23 = {
_1.3 = _28.fld2.2;
_15.0 = !_28.fld4.0;
_49 = core::ptr::addr_of_mut!(_14);
_45 = _9.3.3.fld4.1;
_9.3.3.fld2.2 = _9.1.0.2.0;
_28.fld4.2 = Adt39::Variant2 { fld0: Move(_9.3.3.fld1),fld1: _24,fld2: Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_9.3.3.fld1, 0), 2).1,fld3: _8,fld4: 24903_i16,fld5: _10.1 };
_19.1.2 = _9.1.0.2;
SetDiscriminant(Field::<Adt28>(Variant(_28.fld4.2, 2), 0), 1);
_19.1.1.1 = [19207_i16,157_i16,6908_i16];
_55 = Move(_54);
_28.fld3.0.0.0 = _44.1.0 as f64;
place!(Field::<[i16; 3]>(Variant(_28.fld4.2, 2), 2)) = [5792_i16,(-17619_i16),(-3539_i16)];
_44 = (_10.2, _9.2.0, Move(_9.0.2));
_12.0.3 = _28.fld4.0 as u128;
match _10.1 {
0 => bb1,
1 => bb4,
686492551 => bb24,
_ => bb6
}
}
bb24 = {
_53 = [15147858486788925286_u64];
_28.fld0 = _6;
_43 = &_9.1.0.0;
_1.1 = _9.1.0.1.1;
place!(Field::<char>(Variant(place!(Field::<Adt28>(Variant(_28.fld4.2, 2), 0)), 1), 1)) = _35;
_9.0.0 = -_10.2;
_19.1.1.0 = core::ptr::addr_of!(_44.0);
_9.1.0.1.0 = core::ptr::addr_of!(_10.2);
_12.0.1 = !_9.1.0.2.0;
RET = core::ptr::addr_of_mut!(_15.0);
_9.0.0 = _10.2;
_23.0 = (_9.0.1.0,);
_10.0 = _9.3.3.fld3.0.3;
_53 = [18054884398550135384_u64];
_56.3.2 = _9.1.0.1.2;
_10 = (_12.0.3, Field::<u32>(Variant(_28.fld4.2, 2), 5), _9.0.0);
_12.0.0.3 = Field::<char>(Variant(Field::<Adt28>(Variant(_28.fld4.2, 2), 0), 1), 1);
_28.fld2 = _9.3.3.fld2;
_7 = _10.0 == _13;
_53 = [7369468731257583368_u64];
_44.1.0 = _23.0.0;
_31 = &_9.1.0.1;
_9.3.3.fld4.3 = [_15.0,_15.0,_28.fld4.0,(*RET),(*RET),(*RET),_15.0,_28.fld4.0];
_19.1.1.3 = !_9.1.0.2.0;
_9.3.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_28.fld2.1);
Goto(bb25)
}
bb25 = {
Call(_59 = dump_var(10_usize, 27_usize, Move(_27), 6_usize, Move(_6), 45_usize, Move(_45), 24_usize, Move(_24)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_59 = dump_var(10_usize, 13_usize, Move(_13), 29_usize, Move(_29), 34_usize, Move(_34), 17_usize, Move(_17)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_59 = dump_var(10_usize, 10_usize, Move(_10), 20_usize, Move(_20), 60_usize, _60, 60_usize, _60), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: (*const i32, [i16; 3], usize, bool),mut _8: f64,mut _9: bool,mut _10: f64,mut _11: *mut (bool,),mut _12: f64) -> u128 {
mir! {
type RET = u128;
let _13: &'static isize;
let _14: f64;
let _15: [isize; 3];
let _16: char;
let _17: *mut bool;
let _18: Adt52;
let _19: ();
let _20: ();
{
_10 = -_8;
_8 = (-432208147836938213_i64) as f64;
RET = 281943085260790663289686200122132951762_u128 + 40556860844902016735630069986087891824_u128;
_2 = !_5;
_9 = !_1;
Goto(bb1)
}
bb1 = {
_5 = !_6;
_6 = !_3;
_8 = 1133294120150521237_i64 as f64;
_12 = 145308908917229902925433575029453780136_i128 as f64;
_10 = -_12;
_7.3 = !_3;
_5 = _7.3 | _2;
_3 = _2 < _1;
_7.1 = [19302_i16,(-9317_i16),22202_i16];
_6 = _7.3 | _9;
_15 = [40_isize,115_isize,(-9223372036854775808_isize)];
_4 = _2 ^ _7.3;
_14 = _10 + _8;
_6 = !_5;
_10 = 59316_u16 as f64;
_8 = -_14;
_12 = _8;
_14 = _12;
_4 = !_2;
_5 = _3;
Call(_8 = fn12(_9, _2, _4, _5, Move(_11), _5, _2, _4, _5, _3, _5, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.1 = [(-19876_i16),(-19965_i16),13713_i16];
_14 = _12;
_7.3 = !_4;
_2 = _9;
_5 = _3;
RET = !123096036996151403591494281384408041015_u128;
_14 = _12 * _8;
_2 = !_5;
RET = !133472932795794713276872496413661575786_u128;
_7.1 = [(-642_i16),(-11270_i16),14051_i16];
_6 = _1 == _5;
_7.1 = [28788_i16,28915_i16,(-22409_i16)];
_8 = _10;
_5 = _1;
_7.3 = _1 < _5;
RET = 197392484624143293538071625402179140675_u128;
_7.2 = 226_u8 as usize;
_1 = !_7.3;
_16 = '\u{649a5}';
_16 = '\u{5513e}';
_1 = !_4;
_7.1 = [24360_i16,(-22282_i16),(-14250_i16)];
_17 = core::ptr::addr_of_mut!(_2);
_14 = _7.2 as f64;
_17 = core::ptr::addr_of_mut!(_9);
RET = !308141764191421551106978882086695703551_u128;
RET = !245042794720619698974180226523514635657_u128;
RET = _6 as u128;
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(11_usize, 5_usize, Move(_5), 15_usize, Move(_15), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: *mut (bool,),mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool) -> f64 {
mir! {
type RET = f64;
let _13: char;
let _14: char;
let _15: ((u16, bool, Adt39, [u16; 8]), *const i32, (f32,), Adt43);
let _16: bool;
let _17: [isize; 3];
let _18: &'static &'static isize;
let _19: Adt43;
let _20: Adt31;
let _21: f32;
let _22: i128;
let _23: u8;
let _24: ();
let _25: ();
{
RET = 87840139551002851271185436608139494301_i128 as f64;
_14 = '\u{3d183}';
_3 = _9;
RET = (-3750709155611752421_i64) as f64;
_6 = _10 >= _4;
_12 = _10 ^ _3;
_11 = !_4;
_2 = _6;
_13 = _14;
_8 = _12;
_3 = !_8;
Goto(bb1)
}
bb1 = {
_3 = _11 <= _2;
_4 = _3;
_12 = !_4;
_13 = _14;
_3 = _6;
_15.3.fld2.2 = !_3;
_12 = _15.3.fld2.2 <= _9;
_15.0.1 = !_3;
_3 = !_8;
_15.3.fld2 = (_3, 70983067829176397396360981419157521547_i128, _11, 119_isize);
_15.3.fld0 = _15.3.fld2.2 == _12;
_10 = !_12;
_15.3.fld3.0.3 = 217535251791721780575907542761920512790_u128;
_16 = _15.3.fld0;
match _15.3.fld2.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
119 => bb9,
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
_19.fld3.0.3 = _15.3.fld3.0.3 / _15.3.fld3.0.3;
_15.3.fld2.3 = 42_isize;
_19.fld4.0 = 13424_u16 >> _15.3.fld2.1;
_19.fld4.1 = !_6;
_19.fld2 = (_10, _15.3.fld2.1, _12, _15.3.fld2.3);
_15.3.fld3.0.0.2 = (-123_i8);
_15.3.fld2.2 = !_15.0.1;
_19.fld2.2 = _8;
_19.fld3.0.0.1 = core::ptr::addr_of_mut!(_19.fld2.1);
_15.3.fld3.0.0.1 = Move(_19.fld3.0.0.1);
_12 = !_15.3.fld2.0;
_15.0.1 = !_8;
_19.fld3.0.0.3 = _13;
match _19.fld2.1 {
0 => bb10,
1 => bb11,
70983067829176397396360981419157521547 => bb13,
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
_3 = _11 <= _2;
_4 = _3;
_12 = !_4;
_13 = _14;
_3 = _6;
_15.3.fld2.2 = !_3;
_12 = _15.3.fld2.2 <= _9;
_15.0.1 = !_3;
_3 = !_8;
_15.3.fld2 = (_3, 70983067829176397396360981419157521547_i128, _11, 119_isize);
_15.3.fld0 = _15.3.fld2.2 == _12;
_10 = !_12;
_15.3.fld3.0.3 = 217535251791721780575907542761920512790_u128;
_16 = _15.3.fld0;
match _15.3.fld2.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
119 => bb9,
_ => bb8
}
}
bb13 = {
_15.2.0 = 711095038334543265_i64 as f32;
_19.fld3.0.0 = (RET, Move(_15.3.fld3.0.0.1), _15.3.fld3.0.0.2, _13);
_7 = !_19.fld4.1;
_12 = _7;
_19.fld3.0.2 = [_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0];
_19.fld2 = (_3, _15.3.fld2.1, _4, _15.3.fld2.3);
_19.fld3.0.0.1 = core::ptr::addr_of_mut!(_22);
_19.fld2.0 = !_7;
_15.3.fld4.1 = _7;
_19.fld3.0.1 = _6 ^ _10;
_14 = _13;
_15.0.3 = [_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0];
_6 = !_9;
_15.3.fld3.0.1 = !_6;
_15.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_15.3.fld2.1);
_15.3.fld3.0.0.0 = -_19.fld3.0.0.0;
_8 = !_15.0.1;
_15.3.fld4.3 = [_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0,_19.fld4.0];
_14 = _19.fld3.0.0.3;
_19.fld0 = _7;
_19.fld3.0.0 = (RET, Move(_15.3.fld3.0.0.1), _15.3.fld3.0.0.2, _14);
_15.3.fld3.0.0.3 = _13;
Call(_20 = fn13(_19.fld2.1, _15.3.fld3.0.1, _15.3.fld2.2, _19.fld2.2, _19.fld2, _15.0.1, _19.fld2.1, _19.fld4.1, _15.3.fld2.1, _15.0.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17 = [_19.fld2.3,_15.3.fld2.3,_19.fld2.3];
_19.fld3.0.3 = _19.fld4.0 as u128;
_15.3.fld4.0 = !_19.fld4.0;
_15.3.fld2.3 = -_19.fld2.3;
_22 = _15.3.fld2.3 as i128;
_15.3.fld0 = _15.3.fld2.2 <= _9;
_15.3.fld1 = Adt28::Variant2 { fld0: 3341965791_u32,fld1: _15.2,fld2: Move(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(_20, 1), 0)),fld3: Field::<i8>(Variant(_20, 1), 2),fld4: _15.3.fld3.0.0.0,fld5: Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(_20, 1), 0).1.2,fld6: 7080322599923293643_u64 };
_15.3.fld4.1 = !_12;
_19.fld4.0 = !_15.3.fld4.0;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(12_usize, 16_usize, Move(_16), 12_usize, Move(_12), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(12_usize, 8_usize, Move(_8), 7_usize, Move(_7), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i128,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: (bool, i128, bool, isize),mut _6: bool,mut _7: i128,mut _8: bool,mut _9: i128,mut _10: bool) -> Adt31 {
mir! {
type RET = Adt31;
let _11: [isize; 2];
let _12: [u64; 7];
let _13: char;
let _14: (bool, i128, bool, isize);
let _15: isize;
let _16: ();
let _17: ();
{
_6 = !_3;
_6 = _5.0;
_2 = _5.0;
_5.3 = 193_u8 as isize;
_5.2 = _8 ^ _8;
_6 = _8 ^ _3;
_1 = _9 << _9;
_1 = _5.1 & _9;
_8 = _4 & _2;
_1 = _7 * _9;
_5.2 = _10 | _10;
_3 = _5.0 != _5.2;
_10 = _5.0;
_2 = _3;
_8 = _5.2 < _5.0;
_8 = !_2;
_8 = _3 != _6;
_5.2 = _8 != _10;
_5.1 = _1 - _1;
_5 = (_2, _9, _3, 9223372036854775807_isize);
_7 = _6 as i128;
_7 = 2598568078_u32 as i128;
_1 = _9;
_5.3 = (-9223372036854775808_isize);
_11 = [_5.3,_5.3];
_2 = _6 != _5.2;
_3 = _6;
_11 = [_5.3,_5.3];
Goto(bb1)
}
bb1 = {
_2 = _10;
_9 = _1 | _5.1;
_3 = _5.0;
_5 = (_3, _1, _2, (-9223372036854775808_isize));
_10 = !_3;
_4 = !_6;
_2 = _10;
_5 = (_10, _1, _4, 9223372036854775807_isize);
_6 = _5.0;
_5.3 = (-9223372036854775808_isize);
_7 = !_1;
_5.1 = _7;
_11 = [_5.3,_5.3];
_5.3 = (-100_isize);
_6 = _7 > _9;
_5.3 = (-9223372036854775808_isize);
_3 = !_6;
_5.2 = !_8;
_9 = 206720329256130236051595751608958810319_u128 as i128;
_7 = 27262_u16 as i128;
_10 = _3;
_6 = !_5.0;
_7 = -_1;
_13 = '\u{a2f1c}';
_2 = _5.2;
Goto(bb2)
}
bb2 = {
_13 = '\u{e4608}';
_5.0 = !_8;
_3 = !_4;
_13 = '\u{f2db7}';
_4 = _3;
Call(_5.2 = fn14(_4, _5.0, _5.0, _5.1, _5.0, _2, _5.0, _1, _3, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = [_5.3,_5.3];
_2 = !_5.2;
_8 = _2 < _5.2;
Call(RET = fn15(_2, _6, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.3 = _4 as isize;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [(-8367_i16),(-29180_i16),(-19646_i16)];
place!(Field::<char>(Variant(RET, 1), 1)) = _13;
_14.1 = _5.1 + _5.1;
_7 = -_5.1;
Goto(bb5)
}
bb5 = {
Call(_16 = dump_var(13_usize, 11_usize, Move(_11), 5_usize, Move(_5), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_16 = dump_var(13_usize, 4_usize, Move(_4), 2_usize, Move(_2), 17_usize, _17, 17_usize, _17), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: i128,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: i128,mut _9: bool,mut _10: i128) -> bool {
mir! {
type RET = bool;
let _11: (bool, i128, bool, isize);
let _12: u64;
let _13: ();
let _14: ();
{
RET = _2 ^ _6;
_6 = _7 | _5;
_7 = !_5;
_2 = !_5;
_10 = 12786_u16 as i128;
_9 = !_3;
_11.2 = !_3;
_11 = (_1, _8, _9, (-9223372036854775808_isize));
_11.2 = _11.3 != _11.3;
_11.0 = _5;
_10 = 3975344804_u32 as i128;
_11.1 = _8 + _8;
_11.2 = !_2;
RET = _5 | _11.2;
_11 = (_5, _8, _6, (-9223372036854775808_isize));
_3 = !RET;
_11.0 = !RET;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(14_usize, 10_usize, Move(_10), 4_usize, Move(_4), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(14_usize, 9_usize, Move(_9), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: bool,mut _2: bool,mut _3: (bool, i128, bool, isize)) -> Adt31 {
mir! {
type RET = Adt31;
let _4: f32;
let _5: (u128, u32, i32);
let _6: u8;
let _7: *mut i128;
let _8: f32;
let _9: (i64, (*const i32, [i16; 3], usize, bool), (bool,));
let _10: f64;
let _11: (bool,);
let _12: f64;
let _13: bool;
let _14: char;
let _15: i32;
let _16: Adt39;
let _17: isize;
let _18: f32;
let _19: ();
let _20: ();
{
_3.3 = -(-85_isize);
_3.2 = !_3.0;
_3.0 = _2;
_4 = 263599429310444978594188057801662402342_u128 as f32;
_3.2 = !_2;
_1 = _3.0;
_1 = !_3.0;
_3 = (_1, (-12876757343776345944519922918939075872_i128), _1, (-71_isize));
_5 = (285709125442421721137168348265423261703_u128, 2921415689_u32, 95309222_i32);
_2 = _3.2 & _3.2;
_5.0 = 290120221426472030944079254009729320550_u128 & 254199390862670848912410690769688130148_u128;
_8 = _4 + _4;
_7 = core::ptr::addr_of_mut!(_3.1);
_3.2 = !_2;
_4 = _8 - _8;
match _3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211385 => bb8,
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
_1 = !_3.0;
_3 = (_1, 153667848071688022839383982462767367092_i128, _2, (-25_isize));
_3.0 = _3.2;
_6 = !161_u8;
(*_7) = (-166167963355822671384047981806801784364_i128);
(*_7) = _6 as i128;
_9.1.2 = !4730805966299898753_usize;
_9.0 = 6106225348908839554_i64 - (-3793507550042140867_i64);
_5.2 = 1134847017_i32 >> _3.3;
_9.0 = (-2474961521671920767_i64) + 978203980887427408_i64;
match _3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211431 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_2 = _3.2;
_2 = _3.0;
_9.1.1 = [(-22711_i16),1933_i16,19892_i16];
_4 = _8 + _8;
_1 = !_3.0;
_3 = (_1, (-165793835931364860864637904395636328885_i128), _1, (-9223372036854775808_isize));
_3 = (_1, 153885509360987872289954966606776830136_i128, _2, (-9223372036854775808_isize));
_2 = _3.2 | _3.2;
_9.2.0 = !_3.2;
_9.2 = (_3.0,);
_5.2 = 231957157_i32 - (-555042083_i32);
_3.2 = _2;
_8 = _4;
(*_7) = -(-26138907419644467613930941646085108061_i128);
_2 = _3.0;
_9.1.0 = core::ptr::addr_of!(_5.2);
Goto(bb11)
}
bb11 = {
_9.1.3 = _2 == _2;
_5.2 = 1791850029_i32 >> _3.3;
_3.3 = 9223372036854775807_isize >> _5.2;
_5.1 = 492103085_u32;
_1 = _5.2 != _5.2;
_10 = (*_7) as f64;
RET = Adt31::Variant1 { fld0: Move(_9),fld1: '\u{fe440}',fld2: 13_i8 };
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.0 = core::ptr::addr_of!(_5.2);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.3 = _3.0 != Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).2.0;
_4 = -_8;
_9.0 = Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1.2 as i64;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{1076b4}';
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [24711_i16,19079_i16,(-1685_i16)];
_9.1.0 = core::ptr::addr_of!(_5.2);
_5 = (86737130350722102607965758172562974610_u128, 2610538171_u32, 1214721808_i32);
_5.0 = 331361100735082921038632526833906257382_u128 - 19692727430953260082222217515799872156_u128;
place!(Field::<i8>(Variant(RET, 1), 2)) = 123_i8;
_4 = _8 * _8;
SetDiscriminant(RET, 1);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [(-21096_i16),31667_i16,23351_i16];
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).0 = _9.0;
_11.0 = _3.2 != _2;
Call(_9.1 = fn16(_1, _3, _2, _9.0, _11, _3, _11), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_3.1 = _5.2 as i128;
match _5.1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
2610538171 => bb18,
_ => bb17
}
}
bb13 = {
_9.1.3 = _2 == _2;
_5.2 = 1791850029_i32 >> _3.3;
_3.3 = 9223372036854775807_isize >> _5.2;
_5.1 = 492103085_u32;
_1 = _5.2 != _5.2;
_10 = (*_7) as f64;
RET = Adt31::Variant1 { fld0: Move(_9),fld1: '\u{fe440}',fld2: 13_i8 };
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.0 = core::ptr::addr_of!(_5.2);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.3 = _3.0 != Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).2.0;
_4 = -_8;
_9.0 = Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1.2 as i64;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{1076b4}';
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [24711_i16,19079_i16,(-1685_i16)];
_9.1.0 = core::ptr::addr_of!(_5.2);
_5 = (86737130350722102607965758172562974610_u128, 2610538171_u32, 1214721808_i32);
_5.0 = 331361100735082921038632526833906257382_u128 - 19692727430953260082222217515799872156_u128;
place!(Field::<i8>(Variant(RET, 1), 2)) = 123_i8;
_4 = _8 * _8;
SetDiscriminant(RET, 1);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [(-21096_i16),31667_i16,23351_i16];
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).0 = _9.0;
_11.0 = _3.2 != _2;
Call(_9.1 = fn16(_1, _3, _2, _9.0, _11, _3, _11), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_1 = !_3.0;
_3 = (_1, 153667848071688022839383982462767367092_i128, _2, (-25_isize));
_3.0 = _3.2;
_6 = !161_u8;
(*_7) = (-166167963355822671384047981806801784364_i128);
(*_7) = _6 as i128;
_9.1.2 = !4730805966299898753_usize;
_9.0 = 6106225348908839554_i64 - (-3793507550042140867_i64);
_5.2 = 1134847017_i32 >> _3.3;
_9.0 = (-2474961521671920767_i64) + 978203980887427408_i64;
match _3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211431 => bb10,
_ => bb9
}
}
bb17 = {
Return()
}
bb18 = {
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).2 = (_1,);
_5.0 = _5.1 as u128;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.3 = _1 != _3.2;
_12 = _10 * _10;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.0 = core::ptr::addr_of!(_5.2);
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{714ac}';
_9.1.1 = [(-8223_i16),(-4562_i16),(-28809_i16)];
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.3 = _9.1.3 ^ _1;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.2 = _9.1.2;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).0 = _5.0 as i64;
_3 = (Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).2.0, (-167249157203844488349534651982557099560_i128), Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).2.0, 9223372036854775807_isize);
match _3.3 {
0 => bb1,
1 => bb3,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
9223372036854775807 => bb25,
_ => bb24
}
}
bb19 = {
Return()
}
bb20 = {
_1 = !_3.0;
_3 = (_1, 153667848071688022839383982462767367092_i128, _2, (-25_isize));
_3.0 = _3.2;
_6 = !161_u8;
(*_7) = (-166167963355822671384047981806801784364_i128);
(*_7) = _6 as i128;
_9.1.2 = !4730805966299898753_usize;
_9.0 = 6106225348908839554_i64 - (-3793507550042140867_i64);
_5.2 = 1134847017_i32 >> _3.3;
_9.0 = (-2474961521671920767_i64) + 978203980887427408_i64;
match _3.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211431 => bb10,
_ => bb9
}
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
_9.1.3 = _2 == _2;
_5.2 = 1791850029_i32 >> _3.3;
_3.3 = 9223372036854775807_isize >> _5.2;
_5.1 = 492103085_u32;
_1 = _5.2 != _5.2;
_10 = (*_7) as f64;
RET = Adt31::Variant1 { fld0: Move(_9),fld1: '\u{fe440}',fld2: 13_i8 };
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.0 = core::ptr::addr_of!(_5.2);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.3 = _3.0 != Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).2.0;
_4 = -_8;
_9.0 = Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1.2 as i64;
place!(Field::<char>(Variant(RET, 1), 1)) = '\u{1076b4}';
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [24711_i16,19079_i16,(-1685_i16)];
_9.1.0 = core::ptr::addr_of!(_5.2);
_5 = (86737130350722102607965758172562974610_u128, 2610538171_u32, 1214721808_i32);
_5.0 = 331361100735082921038632526833906257382_u128 - 19692727430953260082222217515799872156_u128;
place!(Field::<i8>(Variant(RET, 1), 2)) = 123_i8;
_4 = _8 * _8;
SetDiscriminant(RET, 1);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = [(-21096_i16),31667_i16,23351_i16];
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).0 = _9.0;
_11.0 = _3.2 != _2;
Call(_9.1 = fn16(_1, _3, _2, _9.0, _11, _3, _11), ReturnTo(bb12), UnwindUnreachable())
}
bb24 = {
_3.1 = _5.2 as i128;
match _5.1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
2610538171 => bb18,
_ => bb17
}
}
bb25 = {
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = _9.1.1;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)) = (_9.0, Move(_9.1), _11);
_9.2 = (_11.0,);
_9 = (Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).0, Move(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1), _11);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.1 = _9.1.1;
_13 = !_9.1.3;
(*_7) = 123649209319257690114116498789027648778_i128;
_15 = _5.2 * _5.2;
_16 = Adt39::Variant1 { fld0: Move(_9.1),fld1: Field::<char>(Variant(RET, 1), 1),fld2: _6,fld3: Move(_7),fld4: 65229_u16,fld5: _5.2,fld6: _11,fld7: _3.1 };
_9 = (Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).0, Move(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_16, 1), 0)), _11);
place!(Field::<char>(Variant(RET, 1), 1)) = Field::<char>(Variant(_16, 1), 1);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)) = (_9.0, Move(_9.1), _9.2);
_9.1.0 = core::ptr::addr_of!(_15);
_14 = Field::<char>(Variant(RET, 1), 1);
place!(Field::<i8>(Variant(RET, 1), 2)) = _10 as i8;
_10 = _12 + _12;
place!(Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_16, 1), 0)).3 = !_1;
_9.0 = _5.0 as i64;
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.0 = Move(_9.1.0);
_9.1 = (Move(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1.0), Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1.1, Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0).1.2, Field::<(*const i32, [i16; 3], usize, bool)>(Variant(_16, 1), 0).3);
place!(Field::<(i64, (*const i32, [i16; 3], usize, bool), (bool,))>(Variant(RET, 1), 0)).1.0 = core::ptr::addr_of!(_15);
_5 = (225351479625521497158523505288378826668_u128, 2957171452_u32, _15);
_18 = -_4;
Goto(bb26)
}
bb26 = {
Call(_19 = dump_var(15_usize, 14_usize, Move(_14), 13_usize, Move(_13), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: (bool, i128, bool, isize),mut _3: bool,mut _4: i64,mut _5: (bool,),mut _6: (bool, i128, bool, isize),mut _7: (bool,)) -> (*const i32, [i16; 3], usize, bool) {
mir! {
type RET = (*const i32, [i16; 3], usize, bool);
let _8: ((u16, bool, Adt39, [u16; 8]), *const i32, (f32,), Adt43);
let _9: [u16; 7];
let _10: bool;
let _11: ((*const i32, [i16; 3], usize, bool), ((f32,),), ((f64, *mut i128, i8, char), bool, [u16; 7], u128), *mut (bool,));
let _12: [char; 7];
let _13: i32;
let _14: [isize; 2];
let _15: i32;
let _16: char;
let _17: *const [usize; 1];
let _18: usize;
let _19: &'static (*const i32, [i16; 3], usize, bool);
let _20: isize;
let _21: i32;
let _22: u64;
let _23: ();
let _24: ();
{
RET.1 = [32019_i16,270_i16,(-5140_i16)];
RET.2 = 7_usize;
RET.3 = _7.0;
RET.3 = _6.2 & _2.2;
_7.0 = _6.2 == _6.0;
RET.2 = 67380289_u32 as usize;
RET.1 = [(-2622_i16),(-5503_i16),(-23172_i16)];
RET.2 = 5763287416903666222_usize;
_6 = (_1, _2.1, _2.2, _2.3);
RET.1 = [(-21405_i16),11566_i16,(-8558_i16)];
_8.3.fld2.3 = _6.3 >> _6.3;
RET.2 = 5756683605222232020_usize;
RET.2 = '\u{acbdf}' as usize;
_8.3.fld4.1 = !_2.2;
_3 = _2.0;
RET.3 = _5.0 <= _1;
_8.3.fld2.1 = 8514488034085911173_u64 as i128;
_8.3.fld3.0.3 = !27993707096871832546053656217431728208_u128;
_5.0 = _3 & _1;
_8.3.fld4.3 = [62061_u16,18310_u16,45658_u16,50171_u16,16073_u16,28416_u16,23483_u16,35249_u16];
_8.0.1 = _2.0 <= _6.2;
Goto(bb1)
}
bb1 = {
_6.1 = (-11832_i16) as i128;
_1 = _5.0 == _3;
RET.2 = 497509745_u32 as usize;
_8.0.0 = !44107_u16;
_8.0.3 = [_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0];
_8.3.fld3.0.0.0 = RET.2 as f64;
_8.3.fld3.0.0.2 = (-105_i8) & 117_i8;
_2.3 = RET.2 as isize;
_2 = (_6.0, _6.1, _8.3.fld4.1, _6.3);
_9 = [_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0];
_8.3.fld3.0.0.3 = '\u{d2331}';
_8.2.0 = 2941494650_u32 as f32;
_12 = [_8.3.fld3.0.0.3,_8.3.fld3.0.0.3,_8.3.fld3.0.0.3,_8.3.fld3.0.0.3,_8.3.fld3.0.0.3,_8.3.fld3.0.0.3,_8.3.fld3.0.0.3];
_6.2 = _2.0;
_11.1.0.0 = _6.1 as f32;
_5 = (_2.0,);
Goto(bb2)
}
bb2 = {
_2.0 = _8.0.1;
_8.0.0 = !57073_u16;
_6.2 = _3 == _8.3.fld4.1;
_8.3.fld2.0 = _5.0;
_8.2 = _11.1.0;
_11.2.0.3 = _8.3.fld3.0.0.3;
_11.0.1 = RET.1;
_5 = _7;
_2.0 = !_8.3.fld2.0;
_8.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_2.1);
_11.2.0.3 = _8.3.fld3.0.0.3;
_8.3.fld3.0.1 = !_3;
_10 = _6.0;
_8.3.fld2.2 = !_6.0;
_8.1 = core::ptr::addr_of!(_13);
_8.3.fld3.0.0.2 = !19_i8;
_11.0.3 = !_8.3.fld4.1;
_7.0 = _2.2;
_11.0.0 = core::ptr::addr_of!(_15);
_15 = _11.1.0.0 as i32;
_7.0 = !_5.0;
_8.2.0 = RET.2 as f32;
_7 = (_8.0.1,);
_6.1 = _2.1 - _8.3.fld2.1;
Goto(bb3)
}
bb3 = {
_6.0 = !_8.3.fld2.0;
_8.3.fld3.0.0.1 = core::ptr::addr_of_mut!(_8.3.fld2.1);
_8.3.fld3.0.3 = !61968570986348414566319495007528369192_u128;
_13 = _15 ^ _15;
_6.1 = _8.3.fld2.1 >> _2.3;
_15 = (-9879_i16) as i32;
_8.3.fld3.0.0.3 = _11.2.0.3;
_5.0 = _2.0 >= _2.2;
_8.3.fld2.0 = !_2.0;
RET.1 = [(-381_i16),16672_i16,(-15106_i16)];
_14 = [_6.3,_6.3];
_11.1.0 = (_8.2.0,);
RET.3 = !_8.0.1;
_11.1 = (_8.2,);
RET.0 = core::ptr::addr_of!(_15);
_8.3.fld2.2 = !_3;
_3 = !_7.0;
_2.0 = _2.2 >= _6.2;
_20 = _6.3;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(16_usize, 12_usize, Move(_12), 14_usize, Move(_14), 20_usize, Move(_20), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(16_usize, 5_usize, Move(_5), 10_usize, Move(_10), 6_usize, Move(_6), 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: f64,mut _2: bool,mut _3: *mut u16,mut _4: f64,mut _5: f64,mut _6: f64) -> f64 {
mir! {
type RET = f64;
let _7: *const [i64; 5];
let _8: u128;
let _9: f64;
let _10: ();
let _11: ();
{
_1 = -_6;
_6 = (-8275950192996147180_i64) as f64;
_2 = false;
_1 = _5 + _4;
Goto(bb1)
}
bb1 = {
_6 = _1 * _1;
RET = (-31402_i16) as f64;
RET = _6;
_5 = 7224_i16 as f64;
_1 = 199634443234674906224785288301932378596_u128 as f64;
_5 = _6;
_8 = 7_usize as u128;
_1 = RET - _5;
_4 = -_6;
_8 = 314040080730353496822183068023140926438_u128;
_9 = 8845307497501357445_u64 as f64;
_5 = -_6;
RET = _4;
_8 = 50_u8 as u128;
RET = _4 + _5;
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(17_usize, 2_usize, Move(_2), 11_usize, _11, 11_usize, _11, 11_usize, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: *const [u16; 7]) -> [u16; 7] {
mir! {
type RET = [u16; 7];
let _5: (bool, i128, bool, isize);
let _6: i128;
let _7: isize;
let _8: ((f32,),);
let _9: u64;
let _10: isize;
let _11: [usize; 2];
let _12: isize;
let _13: [usize; 1];
let _14: (i8, (i64, (*const i32, [i16; 3], usize, bool), (bool,)));
let _15: f32;
let _16: [i64; 5];
let _17: ((f32,), *mut u16, i16, (*const i32, [i16; 3], usize, bool));
let _18: ();
let _19: ();
{
_2 = _1;
RET = [33224_u16,28751_u16,21231_u16,36851_u16,47744_u16,62136_u16,60386_u16];
_3 = 97515568_i32 as f64;
_1 = _2 * _2;
RET = [50163_u16,26786_u16,11962_u16,49427_u16,58953_u16,34170_u16,42762_u16];
RET = [25251_u16,27226_u16,8984_u16,25378_u16,45864_u16,51808_u16,16551_u16];
_1 = -_2;
_4 = core::ptr::addr_of!(RET);
_5.2 = !false;
(*_4) = [24115_u16,18382_u16,14305_u16,9381_u16,47799_u16,8015_u16,56826_u16];
_5 = (true, (-21118247391924573738468134467925109745_i128), false, (-89_isize));
_4 = core::ptr::addr_of!((*_4));
_5.3 = !9223372036854775807_isize;
_2 = 2468732821_u32 as f64;
_5.0 = _1 == _1;
_5.0 = !_5.2;
_5 = (true, (-162832874434625985094455650405939786929_i128), false, (-87_isize));
RET = [40007_u16,17951_u16,33267_u16,51521_u16,35139_u16,43258_u16,31472_u16];
_5.2 = _1 >= _1;
_2 = -_1;
_5.1 = _1 as i128;
_5 = (true, 109721367161445969314593080811522605376_i128, false, 9223372036854775807_isize);
Goto(bb1)
}
bb1 = {
_3 = _2 * _2;
_5.2 = _2 != _3;
RET = [54185_u16,37895_u16,2444_u16,63344_u16,65478_u16,41889_u16,23781_u16];
RET = [27608_u16,13417_u16,5505_u16,60701_u16,23897_u16,9757_u16,50619_u16];
_7 = _5.3;
RET = [33649_u16,12441_u16,59200_u16,22324_u16,61174_u16,37267_u16,60904_u16];
_4 = core::ptr::addr_of!((*_4));
(*_4) = [691_u16,60820_u16,59343_u16,55706_u16,46732_u16,2_u16,36965_u16];
_8.0.0 = (-1081695086_i32) as f32;
_5.2 = _1 < _2;
match _5.1 {
0 => bb2,
109721367161445969314593080811522605376 => bb4,
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
_7 = _5.3 >> _5.1;
_4 = core::ptr::addr_of!((*_4));
RET = [43649_u16,43505_u16,204_u16,35901_u16,64223_u16,54151_u16,39221_u16];
Goto(bb5)
}
bb5 = {
_6 = 49_u8 as i128;
_5.2 = !_5.0;
_5 = (false, _6, false, _7);
_8.0.0 = 14_u8 as f32;
(*_4) = [9333_u16,1260_u16,1112_u16,32582_u16,42438_u16,17692_u16,20991_u16];
_4 = core::ptr::addr_of!((*_4));
_6 = _5.1;
_1 = _2 - _2;
_6 = !_5.1;
_8.0.0 = 2043249435979364146_u64 as f32;
(*_4) = [13019_u16,27237_u16,30628_u16,28494_u16,21263_u16,23196_u16,50395_u16];
(*_4) = [6620_u16,58793_u16,43420_u16,19343_u16,62545_u16,22339_u16,52654_u16];
_5.1 = _6;
_2 = _3 + _1;
(*_4) = [36748_u16,53276_u16,57888_u16,55670_u16,33221_u16,64757_u16,31875_u16];
Goto(bb6)
}
bb6 = {
RET = [35401_u16,45688_u16,20260_u16,7870_u16,64348_u16,6801_u16,42876_u16];
RET = [52907_u16,18586_u16,15375_u16,9510_u16,29726_u16,38758_u16,45596_u16];
_3 = _2 * _2;
_8.0.0 = 2053202366_u32 as f32;
(*_4) = [1722_u16,35238_u16,2122_u16,14937_u16,1151_u16,48211_u16,18018_u16];
_6 = _5.1 << _5.3;
_3 = _1;
_5.2 = !_5.0;
_9 = 1668429837091114589_u64;
RET = [28261_u16,19010_u16,42035_u16,24165_u16,10582_u16,5645_u16,18056_u16];
_5.3 = _7 ^ _7;
_5.3 = _7;
RET = [62705_u16,40675_u16,62291_u16,4114_u16,57194_u16,23041_u16,50239_u16];
_5.2 = !_5.0;
Goto(bb7)
}
bb7 = {
_5 = (false, _6, true, _7);
_7 = _5.3;
_1 = _3;
_8.0.0 = 4251651575_u32 as f32;
_4 = core::ptr::addr_of!(RET);
_4 = core::ptr::addr_of!(RET);
_11 = [17861154264589187035_usize,4326733284502017695_usize];
_6 = _5.1 & _5.1;
_10 = _7 - _7;
RET = [7703_u16,55678_u16,40162_u16,3994_u16,24971_u16,34235_u16,35703_u16];
_9 = 11021467601052040281_u64;
match _9 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
11021467601052040281 => bb14,
_ => bb13
}
}
bb8 = {
RET = [35401_u16,45688_u16,20260_u16,7870_u16,64348_u16,6801_u16,42876_u16];
RET = [52907_u16,18586_u16,15375_u16,9510_u16,29726_u16,38758_u16,45596_u16];
_3 = _2 * _2;
_8.0.0 = 2053202366_u32 as f32;
(*_4) = [1722_u16,35238_u16,2122_u16,14937_u16,1151_u16,48211_u16,18018_u16];
_6 = _5.1 << _5.3;
_3 = _1;
_5.2 = !_5.0;
_9 = 1668429837091114589_u64;
RET = [28261_u16,19010_u16,42035_u16,24165_u16,10582_u16,5645_u16,18056_u16];
_5.3 = _7 ^ _7;
_5.3 = _7;
RET = [62705_u16,40675_u16,62291_u16,4114_u16,57194_u16,23041_u16,50239_u16];
_5.2 = !_5.0;
Goto(bb7)
}
bb9 = {
_6 = 49_u8 as i128;
_5.2 = !_5.0;
_5 = (false, _6, false, _7);
_8.0.0 = 14_u8 as f32;
(*_4) = [9333_u16,1260_u16,1112_u16,32582_u16,42438_u16,17692_u16,20991_u16];
_4 = core::ptr::addr_of!((*_4));
_6 = _5.1;
_1 = _2 - _2;
_6 = !_5.1;
_8.0.0 = 2043249435979364146_u64 as f32;
(*_4) = [13019_u16,27237_u16,30628_u16,28494_u16,21263_u16,23196_u16,50395_u16];
(*_4) = [6620_u16,58793_u16,43420_u16,19343_u16,62545_u16,22339_u16,52654_u16];
_5.1 = _6;
_2 = _3 + _1;
(*_4) = [36748_u16,53276_u16,57888_u16,55670_u16,33221_u16,64757_u16,31875_u16];
Goto(bb6)
}
bb10 = {
_7 = _5.3 >> _5.1;
_4 = core::ptr::addr_of!((*_4));
RET = [43649_u16,43505_u16,204_u16,35901_u16,64223_u16,54151_u16,39221_u16];
Goto(bb5)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_3 = _2 * _2;
_5.2 = _2 != _3;
RET = [54185_u16,37895_u16,2444_u16,63344_u16,65478_u16,41889_u16,23781_u16];
RET = [27608_u16,13417_u16,5505_u16,60701_u16,23897_u16,9757_u16,50619_u16];
_7 = _5.3;
RET = [33649_u16,12441_u16,59200_u16,22324_u16,61174_u16,37267_u16,60904_u16];
_4 = core::ptr::addr_of!((*_4));
(*_4) = [691_u16,60820_u16,59343_u16,55706_u16,46732_u16,2_u16,36965_u16];
_8.0.0 = (-1081695086_i32) as f32;
_5.2 = _1 < _2;
match _5.1 {
0 => bb2,
109721367161445969314593080811522605376 => bb4,
_ => bb3
}
}
bb14 = {
_5.2 = !_5.0;
_11 = [1_usize,0_usize];
_5.1 = _6;
(*_4) = [41568_u16,15443_u16,44246_u16,61747_u16,8346_u16,20715_u16,24332_u16];
_4 = core::ptr::addr_of!((*_4));
_12 = !_10;
_14.1.1.3 = _5.2 ^ _5.0;
_14.1.2 = (_5.2,);
_16 = [4935293204197503800_i64,8588143474208944062_i64,2252082315271777146_i64,(-5223078926764127695_i64),8867891643462767228_i64];
_5.2 = !_14.1.1.3;
_14.1.1.1 = [(-10303_i16),10745_i16,2738_i16];
_14.1.2.0 = _5.2 & _14.1.1.3;
_9 = !16459556455534147456_u64;
RET = [28447_u16,34669_u16,21145_u16,55817_u16,45870_u16,4366_u16,49098_u16];
_11 = [1_usize,0_usize];
_14.1.1.2 = !14093013454724756637_usize;
_17.3.2 = _6 as usize;
_5 = (_14.1.1.3, _6, _14.1.1.3, _10);
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(18_usize, 16_usize, Move(_16), 7_usize, Move(_7), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(17975531054569168916_u64), std::hint::black_box((-839759072_i32)));
                
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt28 {
Variant0{
fld0: i128,
fld1: i32,
fld2: (*const i32, [i16; 3], usize, bool),
fld3: (f32,),
fld4: *mut i128,

},
Variant1{
fld0: *const i32,
fld1: char,
fld2: u8,
fld3: (f32,),
fld4: *mut i128,
fld5: u128,
fld6: u16,

},
Variant2{
fld0: u32,
fld1: (f32,),
fld2: (i64, (*const i32, [i16; 3], usize, bool), (bool,)),
fld3: i8,
fld4: f64,
fld5: usize,
fld6: u64,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: Adt28,
fld1: char,
fld2: i16,
fld3: (f32,),

},
Variant1{
fld0: (i64, (*const i32, [i16; 3], usize, bool), (bool,)),
fld1: char,
fld2: i8,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: bool,
fld1: *mut isize,
fld2: u128,
fld3: i8,
fld4: [u16; 8],
fld5: Adt31,
fld6: [i64; 5],
fld7: ((f64, *mut i128, i8, char), bool, [u16; 7], u128),

},
Variant1{
fld0: (*const i32, [i16; 3], usize, bool),
fld1: char,
fld2: u8,
fld3: *mut i128,
fld4: u16,
fld5: i32,
fld6: (bool,),
fld7: i128,

},
Variant2{
fld0: Adt28,
fld1: u8,
fld2: [i16; 3],
fld3: f64,
fld4: i16,
fld5: u32,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: bool,
fld1: Adt28,
fld2: (bool, i128, bool, isize),
fld3: (((f64, *mut i128, i8, char), bool, [u16; 7], u128),),
fld4: (u16, bool, Adt39, [u16; 8]),
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut (bool,),
fld1: *mut u16,
fld2: ((f32,), *mut u16, i16, (*const i32, [i16; 3], usize, bool)),
fld3: u32,
fld4: *const i32,

},
Variant1{
fld0: u16,
fld1: (*const i32, f64, (f32,), f32),
fld2: isize,
fld3: *mut (bool,),
fld4: (bool, i128, bool, isize),
fld5: Adt39,
fld6: *mut i128,
fld7: i128,

},
Variant2{
fld0: (*const i32, f64, (f32,), f32),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt28,

},
Variant1{
fld0: ((f32,),),
fld1: f64,
fld2: [u16; 7],
fld3: [u16; 8],

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt68{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt68 {
fld0: [i64; 5],
fld1: ((f64, *mut i128, i8, char), bool, [u16; 7], u128),
fld2: *const [i64; 5],
fld3: *const [u16; 7],
fld4: f32,
fld5: i32,
fld6: i64,
fld7: *mut u32,
}
impl PrintFDebug for Adt70{
	unsafe fn printf_debug(&self){unsafe{printf("Adt70::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt70 {
Variant0{
fld0: [i64; 5],
fld1: char,
fld2: *mut i128,
fld3: u8,
fld4: (*const i32, f64, (f32,), f32),
fld5: i32,
fld6: Adt43,

},
Variant1{
fld0: f32,
fld1: *const [u16; 7],

},
Variant2{
fld0: (u16, bool, Adt39, [u16; 8]),
fld1: [usize; 2],
fld2: *const [u16; 7],
fld3: (u16,),
fld4: *mut isize,

},
Variant3{
fld0: f64,
fld1: u8,
fld2: Adt39,
fld3: (((f64, *mut i128, i8, char), bool, [u16; 7], u128),),

}}

