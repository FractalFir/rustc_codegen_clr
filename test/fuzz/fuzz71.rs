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
pub fn fn0(mut _1: bool,mut _2: u32,mut _3: usize,mut _4: u16) -> bool {
mir! {
type RET = bool;
let _5: (char,);
let _6: [u16; 6];
let _7: isize;
let _8: isize;
let _9: f64;
let _10: [u64; 5];
let _11: bool;
let _12: char;
let _13: Adt46;
let _14: isize;
let _15: isize;
let _16: ();
let _17: ();
{
_3 = 3442681840595345312_usize;
_2 = !4151419750_u32;
RET = !true;
_1 = _2 <= _2;
_2 = 3581985255_u32 ^ 326428860_u32;
_4 = 36331_u16 | 6602_u16;
_7 = 9223372036854775807_isize;
_7 = 67_isize;
Call(_3 = core::intrinsics::bswap(13046489804032503170_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = ('\u{2b8c6}',);
RET = _1;
Goto(bb2)
}
bb2 = {
_3 = (-77_i8) as usize;
_2 = 1174033392_u32 ^ 1512100750_u32;
_4 = (-98_i8) as u16;
RET = _1;
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_1 = _2 != _2;
_5.0 = '\u{712e2}';
RET = !_1;
_3 = _4 as usize;
_7 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_7 = 6385068448670585895_u64 as isize;
_9 = 213_u8 as f64;
_1 = _4 > _4;
_1 = _5.0 <= _5.0;
_3 = 353515120_i32 as usize;
RET = _1 | _1;
_8 = _7 >> _2;
_5.0 = '\u{55e2d}';
_5 = ('\u{5c0c2}',);
_9 = _4 as f64;
_5.0 = '\u{2a206}';
_6 = [_4,_4,_4,_4,_4,_4];
_2 = !1997845233_u32;
_2 = !3021208946_u32;
RET = _1 & _1;
Call(_8 = fn1(_2, RET, _7, _4, _2, _2, _5.0, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = [_4,_4,_4,_4,_4,_4];
_8 = _7 + _7;
_10 = [17026900676403272300_u64,17230095105737492715_u64,15630264307615726367_u64,13888838764884755329_u64,16731168019535822099_u64];
_1 = _3 <= _3;
_2 = RET as u32;
_9 = 13232747677758820559513106528870057072_u128 as f64;
RET = _1 <= _1;
_1 = !RET;
_6 = [_4,_4,_4,_4,_4,_4];
_10 = [5829747287996327743_u64,8879192098801572254_u64,15809211381158254915_u64,10991092739548469825_u64,6002283308448086633_u64];
_12 = _5.0;
Call(RET = fn7(_8, _8, _4, _9, _1, _10, _2, _10, _8, _4, _8, _7, _3, _12, _1, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13.fld1 = 8689357453246777396_i64;
_12 = _5.0;
_5.0 = _12;
_2 = 3556448795_u32 | 4281647359_u32;
_12 = _5.0;
Goto(bb5)
}
bb5 = {
Call(_16 = dump_var(0_usize, 1_usize, Move(_1), 8_usize, Move(_8), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_16 = dump_var(0_usize, 2_usize, Move(_2), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u32,mut _2: bool,mut _3: isize,mut _4: u16,mut _5: u32,mut _6: u32,mut _7: char,mut _8: bool) -> isize {
mir! {
type RET = isize;
let _9: *const u64;
let _10: [u64; 1];
let _11: (isize, u8);
let _12: usize;
let _13: ();
let _14: ();
{
_6 = 3011525631917758051_i64 as u32;
_10 = [5309528119683720533_u64];
_2 = _6 != _5;
_2 = _8 != _8;
_6 = _5;
Call(RET = fn2(_2, _4, _7, _4, _2, _5, _5, _2, _2, _2, _8, _8, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = -RET;
_3 = !RET;
_11 = (_3, 116_u8);
_6 = _5;
_3 = RET * _11.0;
_5 = _6;
_6 = !_1;
_7 = '\u{8159b}';
_7 = '\u{5cb93}';
_1 = !_5;
_5 = (-1847834488_i32) as u32;
_11 = (_3, 8_u8);
RET = _3;
_3 = _11.0 * _11.0;
RET = _11.0 ^ _3;
_11.1 = _7 as u8;
_2 = _8;
_6 = !_5;
_8 = !_2;
_1 = _11.1 as u32;
_3 = _11.0;
_8 = _2;
_4 = 35955_u16 << _11.0;
_2 = _8;
_11.1 = 90_u8 | 22_u8;
_11 = (RET, 50_u8);
_1 = 2263400667642024765_u64 as u32;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(1_usize, 3_usize, Move(_3), 6_usize, Move(_6), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(1_usize, 8_usize, Move(_8), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: u16,mut _3: char,mut _4: u16,mut _5: bool,mut _6: u32,mut _7: u32,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: u16) -> isize {
mir! {
type RET = isize;
let _14: [i16; 8];
let _15: [i32; 4];
let _16: [i32; 4];
let _17: [u64; 1];
let _18: bool;
let _19: i128;
let _20: (char,);
let _21: [i32; 4];
let _22: i128;
let _23: u32;
let _24: (isize, bool, isize);
let _25: bool;
let _26: Adt43;
let _27: (*const f32, (char, u64, *const isize), [u64; 1], i32);
let _28: [u16; 6];
let _29: char;
let _30: [u64; 1];
let _31: isize;
let _32: [u64; 1];
let _33: isize;
let _34: *mut [i32; 4];
let _35: i32;
let _36: [i16; 8];
let _37: [i16; 8];
let _38: i32;
let _39: (isize, bool, isize);
let _40: ();
let _41: ();
{
_9 = _5 > _10;
RET = -9223372036854775807_isize;
_11 = _9 == _5;
_5 = _10 >= _1;
_12 = _8;
_9 = _5;
RET = (-80_isize);
RET = !9223372036854775807_isize;
_14 = [7464_i16,(-5927_i16),(-24787_i16),30563_i16,23467_i16,(-10948_i16),(-7047_i16),(-29068_i16)];
_4 = _13;
_5 = _9 > _10;
_4 = !_2;
_13 = !_2;
_5 = _11;
_11 = _5 | _8;
_3 = '\u{cec23}';
_9 = _1 >= _5;
_16 = [727511310_i32,1573597748_i32,(-1162637813_i32),(-1442255553_i32)];
_14 = [(-12034_i16),(-14113_i16),(-5355_i16),303_i16,(-5343_i16),4233_i16,(-29538_i16),(-31835_i16)];
_18 = _5;
RET = (-9223372036854775808_isize);
_13 = _4 - _4;
_16 = [1667824567_i32,(-1288920911_i32),(-1920924995_i32),914608578_i32];
Call(_19 = fn3(_10, _5, _9, _12, _11, _9, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_20 = (_3,);
_16 = [1925116433_i32,2135512014_i32,1473747942_i32,(-1564698242_i32)];
_15 = [(-2014563203_i32),321037557_i32,(-1512631938_i32),506834995_i32];
_10 = _5 <= _5;
_7 = !_6;
_12 = !_10;
_17 = [2215482226766218281_u64];
_13 = _4;
_7 = !_6;
_4 = !_13;
_17 = [3783749957280290693_u64];
_6 = !_7;
_3 = _20.0;
_17 = [13940070955447183102_u64];
_21 = [741325908_i32,241403099_i32,(-1026239831_i32),(-235070681_i32)];
_17 = [2770828608654198680_u64];
_5 = _10;
_20 = (_3,);
_7 = _6;
_5 = !_10;
Call(_9 = fn5(_5, _1, _12, _11, _11, _1, _5, _18, _10, _5, _5, _12, _12, _5, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _6 - _6;
_4 = _2 ^ _13;
_14 = [15003_i16,(-27684_i16),30849_i16,4054_i16,(-16223_i16),18504_i16,26025_i16,17997_i16];
_1 = _10;
_22 = _19 * _19;
_2 = !_4;
_2 = _13 >> RET;
_12 = _5 > _9;
_23 = 22_u8 as u32;
_16 = [(-1481164553_i32),269613776_i32,1726257828_i32,1487731475_i32];
RET = (-9223372036854775808_isize) + (-127_isize);
_6 = RET as u32;
_22 = _1 as i128;
_3 = _20.0;
_16 = [485698061_i32,301915876_i32,(-1294138442_i32),2131402192_i32];
_15 = [(-1342362922_i32),(-1461058365_i32),1163270380_i32,(-2047333803_i32)];
_24 = (RET, _11, RET);
_24.0 = 124_u8 as isize;
_10 = _11 > _12;
_23 = 16_i8 as u32;
_6 = (-23582_i16) as u32;
_6 = _7 - _7;
Goto(bb3)
}
bb3 = {
_21 = _16;
_6 = !_7;
_23 = _7 << _22;
_24.2 = !RET;
RET = -_24.2;
_20 = (_3,);
_26 = Adt43::Variant1 { fld0: _23 };
_27.1.0 = _20.0;
_15 = [(-1678183360_i32),(-242957573_i32),(-443713008_i32),(-1476504832_i32)];
_4 = _13 + _2;
_26 = Adt43::Variant1 { fld0: _23 };
RET = _24.2 * _24.2;
_29 = _27.1.0;
_28 = [_4,_2,_13,_2,_4,_4];
_27.1.1 = !13055406703434651173_u64;
_24.1 = _1 != _1;
_20.0 = _29;
_6 = !Field::<u32>(Variant(_26, 1), 0);
_26 = Adt43::Variant1 { fld0: _23 };
_23 = 3_usize as u32;
_6 = 4856539520240514070_usize as u32;
_28 = [_4,_13,_4,_2,_13,_2];
_23 = Field::<u32>(Variant(_26, 1), 0) << Field::<u32>(Variant(_26, 1), 0);
_30 = _17;
_27.2 = [_27.1.1];
Call(_25 = fn6(_12, _11, Move(_26), _1, _24, _1, _23, _9, _22, _24), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24 = (RET, _25, RET);
_27.1.2 = core::ptr::addr_of!(_24.0);
_31 = _24.0 + RET;
_6 = !_7;
_4 = (-16284_i16) as u16;
_32 = [_27.1.1];
_27.1.0 = _29;
_19 = _22 ^ _22;
_13 = 11133_i16 as u16;
_27.1.1 = !2658297157116264129_u64;
_27.1.0 = _20.0;
_9 = !_10;
_23 = _6 >> _31;
RET = -_24.2;
_22 = _19;
_18 = _10 > _10;
_27.3 = (-1578326964_i32) - (-1561050941_i32);
_27.1.1 = 982178309604628533_u64 + 2700060519033748560_u64;
_33 = -_31;
Goto(bb5)
}
bb5 = {
_27.3 = (-1633571415_i32) ^ (-1577204002_i32);
_32 = _27.2;
_13 = 286692002474315465750700536821983729045_u128 as u16;
_4 = _2 | _2;
_8 = !_24.1;
_27.1.1 = !16608793896002994052_u64;
_9 = _22 != _22;
_35 = _27.3;
_24 = (_31, _10, _33);
_36 = [(-23392_i16),30997_i16,21535_i16,15858_i16,(-13921_i16),(-29899_i16),26288_i16,11765_i16];
_32 = [_27.1.1];
_20 = (_29,);
_37 = [(-5440_i16),20508_i16,24272_i16,9049_i16,(-942_i16),22438_i16,12969_i16,(-28643_i16)];
_28 = [_4,_13,_4,_4,_4,_2];
RET = _1 as isize;
_4 = _2 - _13;
_13 = _2 - _4;
_27.3 = RET as i32;
Goto(bb6)
}
bb6 = {
Call(_40 = dump_var(2_usize, 30_usize, Move(_30), 16_usize, Move(_16), 12_usize, Move(_12), 10_usize, Move(_10)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_40 = dump_var(2_usize, 21_usize, Move(_21), 32_usize, Move(_32), 20_usize, Move(_20), 1_usize, Move(_1)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_40 = dump_var(2_usize, 7_usize, Move(_7), 9_usize, Move(_9), 17_usize, Move(_17), 18_usize, Move(_18)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_40 = dump_var(2_usize, 8_usize, Move(_8), 24_usize, Move(_24), 36_usize, Move(_36), 15_usize, Move(_15)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_40 = dump_var(2_usize, 6_usize, Move(_6), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> i128 {
mir! {
type RET = i128;
let _8: [u64; 5];
let _9: Adt55;
let _10: Adt41;
let _11: i32;
let _12: char;
let _13: [u64; 1];
let _14: isize;
let _15: f64;
let _16: [u64; 5];
let _17: [u64; 1];
let _18: Adt46;
let _19: [u64; 1];
let _20: i32;
let _21: char;
let _22: (isize, bool, isize);
let _23: [i32; 4];
let _24: f32;
let _25: char;
let _26: [u64; 5];
let _27: ();
let _28: ();
{
RET = -(-57888742733658382776654635351144910778_i128);
_3 = !_5;
_4 = !_5;
RET = 3628243523812729373317780695784418532_i128;
_2 = !_6;
RET = (-85302870034039037836879913779933385307_i128);
RET = 8988100791247431487_u64 as i128;
_4 = !_2;
_4 = _3;
_7 = _2;
_4 = !_2;
_4 = _7 | _3;
RET = (-144610736817375349047915249234819994492_i128);
_4 = !_7;
_6 = !_4;
_1 = _4 & _5;
_8 = [14222790278649877587_u64,4412236181883215389_u64,8431638395164247823_u64,5467948115088999499_u64,8681461797763521966_u64];
_8 = [2177935920933232341_u64,11237645221748495204_u64,1808203488117473887_u64,12284964205728645479_u64,4012439673854225908_u64];
_7 = _4;
RET = 4427498917011271918488456575849740939_i128;
_7 = _2 | _6;
_1 = !_5;
Call(_2 = fn4(_1, _4, _3, _5, _1, _4, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !_6;
_8 = [17119366766917007582_u64,16024785253863671050_u64,13403715041531364505_u64,5424753066979484037_u64,11082694823857999039_u64];
_3 = _5;
Goto(bb2)
}
bb2 = {
_1 = !_7;
_6 = !_7;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
4427498917011271918488456575849740939 => bb9,
_ => bb8
}
}
bb3 = {
_2 = !_6;
_8 = [17119366766917007582_u64,16024785253863671050_u64,13403715041531364505_u64,5424753066979484037_u64,11082694823857999039_u64];
_3 = _5;
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
_8 = [13894600085279211544_u64,10637674322115140663_u64,13850408189762134519_u64,11914680227597254014_u64,3372629890957429411_u64];
_7 = _1;
_7 = !_6;
_6 = _1 <= _7;
_7 = _6 ^ _6;
_7 = !_5;
_6 = _3;
_9.fld1 = _8;
_10.fld1.2 = -(-1816264703_i32);
_8 = [4290949449896055738_u64,10557480011177608970_u64,13485319766763302769_u64,9301148786719849347_u64,6201870302513875905_u64];
_6 = _3;
_10.fld1.1 = _10.fld1.2 * _10.fld1.2;
_9.fld0 = [169_u8,164_u8,232_u8,232_u8,233_u8,106_u8];
_9.fld4 = [(-76_i8),45_i8,102_i8,45_i8];
_10.fld2 = _6 as u128;
_8 = [4692796894938441992_u64,11944205134806187065_u64,1128258267801515496_u64,14728243041761040300_u64,5349696187255745917_u64];
_10.fld0.0 = '\u{e6fd9}';
_10.fld2 = 339453869572577999753624514910952645052_u128;
_9.fld3 = core::ptr::addr_of_mut!(_9.fld0);
_11 = 780091829_u32 as i32;
_10.fld1.2 = _11 * _10.fld1.1;
_10.fld1.2 = -_10.fld1.1;
_9.fld1 = _8;
_10.fld2 = !334693104759400772056381648840600100718_u128;
Goto(bb10)
}
bb10 = {
RET = (-164826236046354042360082345218695935597_i128);
_9.fld0 = [185_u8,171_u8,189_u8,245_u8,149_u8,0_u8];
_10.fld1.1 = -_10.fld1.2;
_9.fld3 = core::ptr::addr_of_mut!(_9.fld0);
_4 = !_5;
_2 = _6;
_9.fld5 = _10.fld2 as f64;
_6 = _2;
_12 = _10.fld0.0;
_9.fld3 = core::ptr::addr_of_mut!(_9.fld0);
_6 = _4 ^ _3;
_7 = _5 == _1;
_15 = -_9.fld5;
_16 = [107464814703933646_u64,11710222552688703907_u64,7224512246888024481_u64,6152396219611489408_u64,2433107059996138967_u64];
RET = (-90137231101596893965806399346605023559_i128) * 7923231315918837084713998887150687359_i128;
_10.fld0 = (_12,);
_1 = _3;
Call(_10.fld2 = core::intrinsics::transmute(RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7 = !_6;
_13 = [11186907716523732830_u64];
_11 = _10.fld1.1;
_6 = _7 | _5;
_6 = _5;
_18.fld1 = (-435519440171866414_i64);
_16 = _8;
_9.fld5 = -_15;
match _18.fld1 {
340282366920938463462939087991596345042 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_4 = !_3;
_14 = (-9223372036854775808_isize) & (-78_isize);
_9.fld1 = [16343934743573889972_u64,8274403090267161467_u64,7020009677148340813_u64,10865184071742193760_u64,17150196707451212386_u64];
_6 = _5 <= _4;
_9.fld4 = [(-122_i8),(-93_i8),81_i8,(-46_i8)];
_10.fld2 = !219930669149264875251088178753858647338_u128;
_10.fld0 = (_12,);
_14 = -9223372036854775807_isize;
_15 = _9.fld5 - _9.fld5;
_10.fld1.1 = -_11;
_10.fld1.1 = _11;
match _18.fld1 {
340282366920938463462939087991596345042 => bb14,
_ => bb10
}
}
bb14 = {
_11 = _10.fld1.2 << _10.fld1.2;
_8 = [516091845452043941_u64,6674453305052212605_u64,8254057010793473895_u64,16104381126205211346_u64,4615650757181239111_u64];
_6 = !_7;
_21 = _12;
_17 = [12951565718719681825_u64];
_20 = _10.fld2 as i32;
_22 = (_14, _6, _14);
_18.fld1 = 4348908636377034192_i64;
_11 = -_10.fld1.2;
_22 = (_14, _6, _14);
_8 = [5163857685489383979_u64,16838872345144589127_u64,14965021218494094190_u64,1609700889565624521_u64,4793943283141082267_u64];
_10.fld2 = 80675861984932580465909027115422233088_u128 ^ 186520760425726513498541571035807342164_u128;
_12 = _10.fld0.0;
_11 = _10.fld1.1;
_6 = !_7;
_10.fld0 = (_12,);
_11 = -_10.fld1.1;
_9.fld0 = [202_u8,9_u8,43_u8,184_u8,66_u8,89_u8];
_10.fld1.2 = _20;
_16 = [5846639804815443849_u64,13245882246305186203_u64,2939452671168924868_u64,7874700508470334415_u64,2052371878778267196_u64];
_21 = _10.fld0.0;
_22.2 = _22.0 - _22.0;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(3_usize, 8_usize, Move(_8), 11_usize, Move(_11), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(3_usize, 5_usize, Move(_5), 14_usize, Move(_14), 22_usize, Move(_22), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: f64;
let _9: ();
let _10: ();
{
RET = _6 | _2;
_7 = _6;
_4 = RET & _1;
RET = _2;
RET = _4;
_3 = _5;
_4 = RET;
_2 = _4;
_5 = !_4;
_2 = !_4;
_5 = _2;
_7 = !_4;
RET = _3;
_5 = _4;
RET = _2 != _7;
_1 = _4;
_3 = _4;
_7 = RET ^ _2;
_7 = _2;
RET = _5 | _2;
_4 = _5;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(4_usize, 2_usize, Move(_2), 7_usize, Move(_7), 6_usize, Move(_6), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool) -> bool {
mir! {
type RET = bool;
let _16: i64;
let _17: Adt42;
let _18: ();
let _19: ();
{
RET = _1;
_6 = _14 < _5;
_6 = RET > _1;
_4 = _3;
_14 = _6 == _8;
_2 = !_13;
_10 = _11;
_7 = !_15;
RET = _12;
_10 = RET;
_16 = -9200983586988338836_i64;
_10 = _5 & _8;
_8 = !_14;
_16 = !(-7819830322643644564_i64);
_1 = _12 != _12;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(5_usize, 9_usize, Move(_9), 14_usize, Move(_14), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(5_usize, 6_usize, Move(_6), 3_usize, Move(_3), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: bool,mut _3: Adt43,mut _4: bool,mut _5: (isize, bool, isize),mut _6: bool,mut _7: u32,mut _8: bool,mut _9: i128,mut _10: (isize, bool, isize)) -> bool {
mir! {
type RET = bool;
let _11: isize;
let _12: Adt47;
let _13: [u64; 1];
let _14: isize;
let _15: u32;
let _16: bool;
let _17: [i8; 4];
let _18: isize;
let _19: ();
let _20: ();
{
_8 = _1 != _5.1;
_5 = _10;
_10.0 = _10.2;
SetDiscriminant(_3, 1);
_10.1 = !_5.1;
_5.0 = -_5.2;
RET = _6;
_4 = _10.1;
_2 = _6;
_5.1 = RET;
RET = !_8;
_2 = _10.1;
_2 = !_1;
_6 = _2 & _10.1;
place!(Field::<u32>(Variant(_3, 1), 0)) = _7;
_12.fld1 = !6385421229542629216_i64;
_2 = _6 < _8;
_3 = Adt43::Variant1 { fld0: _7 };
_13 = [12729851470257982016_u64];
_2 = !_4;
_16 = _10.1 >= _6;
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(6_usize, 9_usize, Move(_9), 4_usize, Move(_4), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(6_usize, 2_usize, Move(_2), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize,mut _3: u16,mut _4: f64,mut _5: bool,mut _6: [u64; 5],mut _7: u32,mut _8: [u64; 5],mut _9: isize,mut _10: u16,mut _11: isize,mut _12: isize,mut _13: usize,mut _14: char,mut _15: bool,mut _16: [u64; 5]) -> bool {
mir! {
type RET = bool;
let _17: (isize, bool, isize);
let _18: bool;
let _19: char;
let _20: [u64; 1];
let _21: (isize, u8);
let _22: *const isize;
let _23: Adt51;
let _24: i16;
let _25: Adt47;
let _26: [u64; 1];
let _27: ();
let _28: ();
{
_6 = [12014736690782131689_u64,2154214360880827215_u64,3163889790677285388_u64,17832228521837016124_u64,14248811886673343919_u64];
RET = _15;
_13 = !1_usize;
_2 = _12;
Call(_8 = fn8(_2, _15, _1, _4, _11, _16, _16, _16), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _15 <= _15;
_4 = 12126829946557809126_u64 as f64;
_7 = 1618923729566438903_i64 as u32;
_2 = 97549326217023881873818061469815248695_i128 as isize;
_13 = _11 as usize;
_2 = _9 & _9;
_12 = 116_u8 as isize;
_19 = _14;
RET = _4 < _4;
_17 = (_2, _5, _2);
_20 = [5608713027888778397_u64];
_8 = _6;
Goto(bb2)
}
bb2 = {
_2 = _1 << _1;
_16 = [15625194005181527046_u64,10512978919330173571_u64,15332997626965823265_u64,1314201024593888192_u64,12210778469273741170_u64];
_11 = !_2;
_17.1 = _5;
_22 = core::ptr::addr_of!(_17.0);
_14 = _19;
_17 = (_2, _5, _1);
_18 = !_17.1;
_17.0 = _1 & _9;
_13 = 86140368160060995420237385216408636062_u128 as usize;
_24 = (-23103_i16);
_17.1 = _5;
_21.1 = !203_u8;
_10 = 1287763606_i32 as u16;
_6 = _8;
_24 = _13 as i16;
_24 = !(-1463_i16);
_1 = -(*_22);
Call(_25.fld1 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _19 as u32;
_8 = [12516773683026825675_u64,11654268215207936193_u64,16074678940863136401_u64,718790495070561547_u64,7238664800512741334_u64];
_21.1 = 199_u8;
_21 = (_2, 179_u8);
RET = _17.1 >= _17.1;
_17 = (_21.0, _5, _21.0);
_6 = _16;
_21 = (_17.2, 153_u8);
_25.fld1 = (-3366354342825953295_i64) + (-7437394605330224396_i64);
_4 = (-69528990817571634096252236342800333798_i128) as f64;
_21 = ((*_22), 56_u8);
_25.fld2 = !57386463260888716018224057826744945934_u128;
_25.fld1 = 8603795817466609816_i64 << _2;
Goto(bb4)
}
bb4 = {
Call(_27 = dump_var(7_usize, 10_usize, Move(_10), 13_usize, Move(_13), 19_usize, Move(_19), 16_usize, Move(_16)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_27 = dump_var(7_usize, 15_usize, Move(_15), 11_usize, Move(_11), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_27 = dump_var(7_usize, 18_usize, Move(_18), 3_usize, Move(_3), 28_usize, _28, 28_usize, _28), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: bool,mut _3: isize,mut _4: f64,mut _5: isize,mut _6: [u64; 5],mut _7: [u64; 5],mut _8: [u64; 5]) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _9: i32;
let _10: f64;
let _11: (*const u64, i32, i32);
let _12: isize;
let _13: *const u64;
let _14: Adt49;
let _15: u128;
let _16: char;
let _17: isize;
let _18: Adt45;
let _19: Adt51;
let _20: [u64; 5];
let _21: (&'static f32,);
let _22: [i8; 4];
let _23: isize;
let _24: Adt47;
let _25: [u64; 1];
let _26: char;
let _27: ();
let _28: ();
{
_1 = '\u{20d98}' as isize;
RET = _8;
_9 = 1972041167_i32 | 1464602643_i32;
_9 = 1937458384_i32;
RET = [12138853568335193028_u64,17022539826017880417_u64,12562107248435476042_u64,16450912000461854413_u64,6738084127766868115_u64];
_2 = _5 == _1;
_5 = _3 >> _1;
_7 = _8;
_6 = _8;
_1 = _5 * _3;
_6 = [9212943954163785645_u64,3237556832511025903_u64,5810138923060108480_u64,2926709071495016446_u64,10403558788381747113_u64];
_6 = [2352531677650687369_u64,8811766004690181458_u64,11434550632544881699_u64,13838698402356767232_u64,12430860797061588358_u64];
Call(_3 = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !false;
_1 = _2 as isize;
_9 = (-166314093557377578109798321799994530313_i128) as i32;
_10 = _4;
_7 = [14521805030746034809_u64,2160819441702227823_u64,15851290045826548910_u64,11910216318245679695_u64,15829391981980425066_u64];
_6 = [17230372514723698584_u64,12336054482224339134_u64,15889317178245296147_u64,3315677848693149100_u64,10872099782880219496_u64];
Call(RET = fn9(_8, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [17939209576519258870_u64,8568571179317596016_u64,2670992882443355332_u64,14901810068805251898_u64,8822105400990122085_u64];
_9 = 459997451_i32;
_3 = _5;
_6 = [11843352273219491073_u64,12149676495702441478_u64,6741707302066550477_u64,794959886316884283_u64,54844817284236204_u64];
match _9 {
0 => bb3,
1 => bb4,
459997451 => bb6,
_ => bb5
}
}
bb3 = {
_2 = !false;
_1 = _2 as isize;
_9 = (-166314093557377578109798321799994530313_i128) as i32;
_10 = _4;
_7 = [14521805030746034809_u64,2160819441702227823_u64,15851290045826548910_u64,11910216318245679695_u64,15829391981980425066_u64];
_6 = [17230372514723698584_u64,12336054482224339134_u64,15889317178245296147_u64,3315677848693149100_u64,10872099782880219496_u64];
Call(RET = fn9(_8, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_11.1 = _9 & _9;
_4 = _1 as f64;
match _9 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
459997451 => bb12,
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
_2 = !false;
_1 = _2 as isize;
_9 = (-166314093557377578109798321799994530313_i128) as i32;
_10 = _4;
_7 = [14521805030746034809_u64,2160819441702227823_u64,15851290045826548910_u64,11910216318245679695_u64,15829391981980425066_u64];
_6 = [17230372514723698584_u64,12336054482224339134_u64,15889317178245296147_u64,3315677848693149100_u64,10872099782880219496_u64];
Call(RET = fn9(_8, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_8 = [17939209576519258870_u64,8568571179317596016_u64,2670992882443355332_u64,14901810068805251898_u64,8822105400990122085_u64];
_9 = 459997451_i32;
_3 = _5;
_6 = [11843352273219491073_u64,12149676495702441478_u64,6741707302066550477_u64,794959886316884283_u64,54844817284236204_u64];
match _9 {
0 => bb3,
1 => bb4,
459997451 => bb6,
_ => bb5
}
}
bb11 = {
_2 = !false;
_1 = _2 as isize;
_9 = (-166314093557377578109798321799994530313_i128) as i32;
_10 = _4;
_7 = [14521805030746034809_u64,2160819441702227823_u64,15851290045826548910_u64,11910216318245679695_u64,15829391981980425066_u64];
_6 = [17230372514723698584_u64,12336054482224339134_u64,15889317178245296147_u64,3315677848693149100_u64,10872099782880219496_u64];
Call(RET = fn9(_8, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_3 = _5;
_6 = _8;
_12 = 5_usize as isize;
_8 = [13381216447386366453_u64,4969369032605995889_u64,2534090687458960653_u64,3156337696921310588_u64,7643161222480838936_u64];
_3 = 130484589515398479805991693262932074255_i128 as isize;
_4 = -_10;
match _9 {
459997451 => bb14,
_ => bb13
}
}
bb13 = {
_2 = !false;
_1 = _2 as isize;
_9 = (-166314093557377578109798321799994530313_i128) as i32;
_10 = _4;
_7 = [14521805030746034809_u64,2160819441702227823_u64,15851290045826548910_u64,11910216318245679695_u64,15829391981980425066_u64];
_6 = [17230372514723698584_u64,12336054482224339134_u64,15889317178245296147_u64,3315677848693149100_u64,10872099782880219496_u64];
Call(RET = fn9(_8, _3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_10 = -_4;
_11.2 = _11.1;
_15 = !176223583655841056883947499195911351035_u128;
_17 = _5 >> _12;
_9 = 32279_i16 as i32;
_9 = 84_u8 as i32;
RET = [2760073255871628642_u64,4168397410737644865_u64,11405045799738524520_u64,10537386838710212464_u64,7499078868706189909_u64];
_17 = _1;
_7 = _8;
_24.fld2 = _15;
_2 = !false;
_15 = !_24.fld2;
_24.fld2 = _15 >> _15;
RET = [12740085173568815312_u64,9860456337948195410_u64,1861078334620998911_u64,7436024723667691535_u64,8195487334595337181_u64];
_4 = -_10;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(8_usize, 3_usize, Move(_3), 6_usize, Move(_6), 2_usize, Move(_2), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(8_usize, 1_usize, Move(_1), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [u64; 5],mut _2: isize,mut _3: isize) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _4: u128;
let _5: Adt43;
let _6: [u8; 6];
let _7: [u8; 6];
let _8: f64;
let _9: [u8; 6];
let _10: [i16; 8];
let _11: [i32; 4];
let _12: bool;
let _13: [u16; 6];
let _14: i8;
let _15: bool;
let _16: isize;
let _17: (isize, bool, isize);
let _18: char;
let _19: i32;
let _20: [u64; 1];
let _21: [i16; 8];
let _22: i8;
let _23: (&'static f32,);
let _24: (char,);
let _25: [u8; 6];
let _26: ();
let _27: ();
{
_2 = -_3;
_3 = _2;
RET = [10701621751790114816_u64,15033944202840241742_u64,5125663525336412368_u64,8075483476419571797_u64,15364616428357125516_u64];
RET = [17347646195811423683_u64,1431811016829026379_u64,3222436200087906510_u64,8170818931922881530_u64,714442652990678493_u64];
RET = _1;
_3 = _2;
_1 = RET;
RET = _1;
_4 = 191981484928089290894549180737757986178_u128;
_2 = !_3;
_3 = _2 ^ _2;
RET = [15842735579689734669_u64,1058093449167462595_u64,9569386729487981780_u64,8302289754750902955_u64,2832017169185240495_u64];
_3 = 45412_u16 as isize;
match _4 {
0 => bb1,
1 => bb2,
191981484928089290894549180737757986178 => bb4,
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
_3 = '\u{2b95f}' as isize;
_3 = 52_i8 as isize;
RET = [1583995644307769134_u64,17475018538542573109_u64,5824213988466268794_u64,11805158810140308971_u64,13022104346643927842_u64];
RET = [14217662114683319999_u64,16554182141515734895_u64,4190611235176336095_u64,6040723553732596715_u64,815235295503409470_u64];
_6 = [114_u8,130_u8,195_u8,146_u8,153_u8,148_u8];
RET = [12506756606734743016_u64,1324245023347603140_u64,139193487181408880_u64,14299119863988064402_u64,3570931743143732397_u64];
_1 = [11959502902048817234_u64,7349869217946785779_u64,7096138751549628983_u64,13579428922518921997_u64,5719515708884101257_u64];
_1 = RET;
_5 = Adt43::Variant1 { fld0: 2654692151_u32 };
_7 = _6;
_7 = [108_u8,122_u8,101_u8,124_u8,213_u8,89_u8];
_7 = [123_u8,88_u8,211_u8,70_u8,160_u8,254_u8];
_3 = (-6558522777407814095_i64) as isize;
_6 = [75_u8,8_u8,148_u8,63_u8,234_u8,167_u8];
_6 = _7;
_4 = 102434993397783751955106898209230345037_u128 ^ 3426298932494705655204924013666946193_u128;
Call(_8 = fn10(_1, _7, _7, _7, _2, _7, _7, _6, _6, _3, _7, _2, _6, _6, RET, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = [17567382886660103794_u64,3341734584814075229_u64,13633628169515330025_u64,17752952554726209560_u64,9014411936109366191_u64];
_8 = 7415586315567306804_i64 as f64;
_9 = [25_u8,143_u8,58_u8,130_u8,40_u8,86_u8];
_9 = [220_u8,45_u8,216_u8,31_u8,53_u8,230_u8];
_7 = [148_u8,202_u8,219_u8,23_u8,129_u8,11_u8];
_5 = Adt43::Variant1 { fld0: 87633441_u32 };
_1 = [13735654482313093236_u64,5944865572293377286_u64,7092651695839693840_u64,6509845603985319070_u64,10803545588491286437_u64];
_6 = _9;
_1 = RET;
_9 = [57_u8,168_u8,17_u8,82_u8,40_u8,6_u8];
_3 = _2;
_8 = (-6751701926046898482_i64) as f64;
_3 = 169_u8 as isize;
place!(Field::<u32>(Variant(_5, 1), 0)) = 3804929094_u32 - 1607295509_u32;
_7 = [43_u8,216_u8,208_u8,233_u8,93_u8,62_u8];
_7 = [81_u8,20_u8,230_u8,57_u8,214_u8,201_u8];
Goto(bb6)
}
bb6 = {
_10 = [(-27843_i16),24299_i16,14497_i16,5414_i16,26323_i16,(-6691_i16),(-1948_i16),(-12402_i16)];
_8 = 37_i8 as f64;
_6 = _9;
_9 = _6;
_9 = [122_u8,246_u8,9_u8,64_u8,46_u8,32_u8];
_9 = _6;
place!(Field::<u32>(Variant(_5, 1), 0)) = !3836156203_u32;
_7 = [75_u8,22_u8,179_u8,65_u8,179_u8,25_u8];
_8 = 15586566636687125080_usize as f64;
Call(_8 = core::intrinsics::transmute(_3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = !_2;
_2 = _8 as isize;
_7 = [165_u8,154_u8,21_u8,241_u8,144_u8,57_u8];
RET = [8433885452688821024_u64,15872582454602955495_u64,14668059012136704998_u64,1934593312185214553_u64,5135908814121602187_u64];
_13 = [13832_u16,28434_u16,61680_u16,34508_u16,13550_u16,58979_u16];
_11 = [(-1972176945_i32),(-1204506155_i32),(-2049019781_i32),(-393584492_i32)];
RET = [4965230086981319169_u64,750418368141852161_u64,6739449847241946216_u64,3988927991759387232_u64,15687211363719949667_u64];
SetDiscriminant(_5, 0);
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld1.2 = _8 as i32;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.2 = core::ptr::addr_of!(_2);
place!(Field::<i16>(Variant(_5, 0), 4)) = 14929_i16 & (-8566_i16);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.0 = '\u{392e6}';
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.0 = '\u{3040b}';
_6 = _9;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).3 = Field::<Adt41>(Variant(_5, 0), 0).fld1.2 - Field::<Adt41>(Variant(_5, 0), 0).fld1.2;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.2 = core::ptr::addr_of!(_3);
_16 = _2;
_13 = [40392_u16,38852_u16,55633_u16,16699_u16,19148_u16,26707_u16];
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.1 = 13096197855861760479_u64;
place!(Field::<u64>(Variant(_5, 0), 3)) = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1;
_7 = [219_u8,103_u8,123_u8,162_u8,55_u8,122_u8];
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).3 = 67_i8 as i32;
_17.0 = _2 + _16;
place!(Field::<u64>(Variant(_5, 0), 3)) = !Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.1 = Field::<u64>(Variant(_5, 0), 3);
Goto(bb8)
}
bb8 = {
_17.0 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).3 as isize;
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld0 = (Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.0,);
_2 = (-127_i8) as isize;
_10 = [Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4)];
_9 = [197_u8,193_u8,60_u8,169_u8,106_u8,244_u8];
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld1.0 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_5, 0), 3)));
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).3 = Field::<Adt41>(Variant(_5, 0), 0).fld1.2;
_18 = Field::<Adt41>(Variant(_5, 0), 0).fld0.0;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.2 = core::ptr::addr_of!(_2);
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld0.0 = _18;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).2 = [Field::<u64>(Variant(_5, 0), 3)];
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld0.0 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.0;
_12 = !false;
_19 = !Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).3;
Goto(bb9)
}
bb9 = {
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld1.0 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_5, 0), 3)));
_18 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.0;
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld2 = 183_u8 as u128;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.2 = core::ptr::addr_of!(_16);
_13 = [54664_u16,42227_u16,12029_u16,41815_u16,44058_u16,65012_u16];
place!(Field::<u64>(Variant(_5, 0), 3)) = Field::<i16>(Variant(_5, 0), 4) as u64;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).3 = _19 | _19;
RET = [Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<u64>(Variant(_5, 0), 3),Field::<u64>(Variant(_5, 0), 3),Field::<u64>(Variant(_5, 0), 3),Field::<u64>(Variant(_5, 0), 3)];
_1 = [Field::<u64>(Variant(_5, 0), 3),Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<u64>(Variant(_5, 0), 3),Field::<u64>(Variant(_5, 0), 3),Field::<u64>(Variant(_5, 0), 3)];
_2 = _3;
_15 = !_12;
RET = _1;
_19 = !Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).3;
place!(Field::<u64>(Variant(_5, 0), 3)) = !Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1;
_4 = Field::<Adt41>(Variant(_5, 0), 0).fld2 | Field::<Adt41>(Variant(_5, 0), 0).fld2;
_15 = !_12;
_14 = (-52_i8) << _16;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.2 = core::ptr::addr_of!(_17.0);
_11 = [Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).3,Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).3,Field::<Adt41>(Variant(_5, 0), 0).fld1.2,_19];
_13 = [25338_u16,16517_u16,15617_u16,17260_u16,4946_u16,65500_u16];
_13 = [54005_u16,32942_u16,30221_u16,25719_u16,50876_u16,14926_u16];
place!(Field::<u8>(Variant(_5, 0), 5)) = !72_u8;
RET = _1;
_1 = [Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<u64>(Variant(_5, 0), 3),Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1];
_1 = RET;
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld2 = _4 >> Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1;
Goto(bb10)
}
bb10 = {
_17.2 = -_16;
_12 = _15;
_18 = Field::<Adt41>(Variant(_5, 0), 0).fld0.0;
_1 = [Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<u64>(Variant(_5, 0), 3),Field::<u64>(Variant(_5, 0), 3),Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1,Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1];
_20 = [Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1];
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).2 = [Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.1];
place!(Field::<i16>(Variant(_5, 0), 4)) = 4612_i16;
match Field::<i16>(Variant(_5, 0), 4) {
0 => bb1,
1 => bb7,
2 => bb6,
4612 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_4 = 135461962371370841704634191165770290099_i128 as u128;
_21 = [Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4)];
_12 = _15;
_21 = [Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4),Field::<i16>(Variant(_5, 0), 4)];
Goto(bb13)
}
bb13 = {
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld0.0 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).1.0;
place!(Field::<u8>(Variant(_5, 0), 5)) = !211_u8;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.1 = !Field::<u64>(Variant(_5, 0), 3);
_17.2 = _3 | _2;
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld0 = (_18,);
_22 = _14 ^ _14;
_17.1 = !_12;
_4 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2).3 as u128;
_6 = [Field::<u8>(Variant(_5, 0), 5),Field::<u8>(Variant(_5, 0), 5),Field::<u8>(Variant(_5, 0), 5),Field::<u8>(Variant(_5, 0), 5),Field::<u8>(Variant(_5, 0), 5),Field::<u8>(Variant(_5, 0), 5)];
Goto(bb14)
}
bb14 = {
RET = _1;
place!(Field::<Adt41>(Variant(_5, 0), 0)).fld1.1 = -_19;
_8 = 274026493807983132_i64 as f64;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.0 = _18;
_13 = [17711_u16,61848_u16,15953_u16,58180_u16,15188_u16,53241_u16];
_15 = _17.1;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.1 = Field::<u64>(Variant(_5, 0), 3) | Field::<u64>(Variant(_5, 0), 3);
_21 = _10;
_7 = _9;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_5, 0), 2)).1.0 = _18;
_5 = Adt43::Variant1 { fld0: 1454825663_u32 };
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(9_usize, 14_usize, Move(_14), 15_usize, Move(_15), 18_usize, Move(_18), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(9_usize, 17_usize, Move(_17), 3_usize, Move(_3), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(9_usize, 2_usize, Move(_2), 11_usize, Move(_11), 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [u64; 5],mut _2: [u8; 6],mut _3: [u8; 6],mut _4: [u8; 6],mut _5: isize,mut _6: [u8; 6],mut _7: [u8; 6],mut _8: [u8; 6],mut _9: [u8; 6],mut _10: isize,mut _11: [u8; 6],mut _12: isize,mut _13: [u8; 6],mut _14: [u8; 6],mut _15: [u64; 5],mut _16: [u8; 6]) -> f64 {
mir! {
type RET = f64;
let _17: [i16; 8];
let _18: [u16; 6];
let _19: [i32; 4];
let _20: Adt51;
let _21: [i8; 4];
let _22: char;
let _23: [u64; 1];
let _24: (char,);
let _25: f64;
let _26: (char,);
let _27: u64;
let _28: Adt48;
let _29: u32;
let _30: (isize, bool, isize);
let _31: usize;
let _32: i128;
let _33: [u8; 6];
let _34: Adt46;
let _35: u64;
let _36: bool;
let _37: [u8; 6];
let _38: u128;
let _39: *const f32;
let _40: [u64; 5];
let _41: i64;
let _42: [i8; 4];
let _43: [u64; 1];
let _44: ();
let _45: ();
{
RET = 1362575796681259851_i64 as f64;
RET = 2061877340_u32 as f64;
_16 = [215_u8,252_u8,193_u8,86_u8,88_u8,76_u8];
_11 = [175_u8,78_u8,36_u8,39_u8,79_u8,117_u8];
_13 = _2;
_14 = [213_u8,132_u8,146_u8,174_u8,230_u8,205_u8];
_17 = [(-4801_i16),22075_i16,(-6112_i16),6609_i16,1122_i16,(-7051_i16),(-22932_i16),(-608_i16)];
_12 = -_5;
_14 = [54_u8,228_u8,155_u8,70_u8,218_u8,156_u8];
Goto(bb1)
}
bb1 = {
RET = _5 as f64;
_17 = [(-12608_i16),17595_i16,(-25233_i16),(-23860_i16),(-15271_i16),7994_i16,(-10834_i16),14191_i16];
RET = 247_u8 as f64;
_16 = [67_u8,98_u8,204_u8,204_u8,20_u8,1_u8];
_6 = [227_u8,220_u8,54_u8,59_u8,129_u8,25_u8];
_13 = _4;
_8 = [144_u8,116_u8,9_u8,199_u8,60_u8,180_u8];
_15 = _1;
_4 = [11_u8,230_u8,144_u8,11_u8,152_u8,97_u8];
_13 = [244_u8,15_u8,44_u8,56_u8,39_u8,254_u8];
_12 = _5;
_17 = [(-9506_i16),(-6918_i16),29694_i16,(-15881_i16),(-30647_i16),32003_i16,32582_i16,(-199_i16)];
_10 = !_5;
_11 = [38_u8,156_u8,252_u8,15_u8,177_u8,22_u8];
_15 = [5372280354250131565_u64,6630438066170215537_u64,506919951479122792_u64,9055051416503950051_u64,2853443290675388438_u64];
_7 = _8;
RET = 34_u8 as f64;
_3 = [125_u8,159_u8,12_u8,22_u8,221_u8,171_u8];
_10 = 3155916289_u32 as isize;
_5 = _12;
_8 = [204_u8,235_u8,153_u8,179_u8,119_u8,241_u8];
Call(_7 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = [233_u8,23_u8,78_u8,143_u8,39_u8,46_u8];
_7 = [188_u8,152_u8,43_u8,248_u8,103_u8,208_u8];
_11 = [235_u8,141_u8,77_u8,162_u8,30_u8,71_u8];
_19 = [1739221156_i32,(-832981718_i32),2031927549_i32,1304888576_i32];
_11 = [120_u8,132_u8,43_u8,223_u8,146_u8,95_u8];
_8 = [208_u8,75_u8,41_u8,7_u8,7_u8,240_u8];
_14 = _11;
RET = 2427587867197342974_u64 as f64;
_4 = _16;
_11 = [229_u8,170_u8,137_u8,192_u8,235_u8,210_u8];
_10 = _5;
Goto(bb3)
}
bb3 = {
_17 = [4739_i16,(-13641_i16),13050_i16,10906_i16,29408_i16,(-14922_i16),(-23885_i16),20374_i16];
_16 = _11;
_11 = [106_u8,27_u8,73_u8,10_u8,212_u8,15_u8];
_8 = [209_u8,5_u8,81_u8,97_u8,142_u8,79_u8];
_18 = [22108_u16,20146_u16,25772_u16,18759_u16,16151_u16,61496_u16];
_15 = _1;
_14 = _6;
_13 = [244_u8,223_u8,251_u8,154_u8,174_u8,175_u8];
_8 = [90_u8,226_u8,63_u8,183_u8,126_u8,1_u8];
_3 = _8;
_21 = [(-71_i8),(-24_i8),(-1_i8),71_i8];
_8 = [120_u8,162_u8,168_u8,82_u8,61_u8,94_u8];
_23 = [16694826302760582339_u64];
_8 = [207_u8,212_u8,111_u8,195_u8,15_u8,45_u8];
_14 = [172_u8,162_u8,212_u8,220_u8,131_u8,1_u8];
_10 = -_5;
_18 = [4138_u16,57211_u16,17914_u16,15000_u16,7710_u16,59857_u16];
_18 = [27940_u16,24628_u16,40935_u16,18876_u16,44200_u16,57066_u16];
_10 = _12;
_3 = [224_u8,174_u8,196_u8,75_u8,247_u8,217_u8];
Goto(bb4)
}
bb4 = {
RET = (-29_i8) as f64;
_16 = [199_u8,135_u8,232_u8,5_u8,23_u8,18_u8];
_24.0 = '\u{3b572}';
_27 = !3772986688088422040_u64;
_9 = _14;
_3 = _6;
_14 = _13;
RET = 87976633025526327636354922281902200721_u128 as f64;
_16 = [97_u8,183_u8,13_u8,129_u8,173_u8,140_u8];
_9 = _3;
_26 = (_24.0,);
_7 = [84_u8,175_u8,58_u8,7_u8,24_u8,161_u8];
_15 = _1;
_17 = [(-30814_i16),(-15211_i16),(-1196_i16),32209_i16,24279_i16,(-15364_i16),(-16676_i16),5441_i16];
_7 = [118_u8,148_u8,45_u8,49_u8,224_u8,156_u8];
_1 = [_27,_27,_27,_27,_27];
_24 = (_26.0,);
_9 = [97_u8,26_u8,181_u8,228_u8,183_u8,219_u8];
_5 = 191328066_i32 as isize;
RET = (-8104522842205681224948673276558604858_i128) as f64;
Goto(bb5)
}
bb5 = {
_30.1 = true;
RET = (-27_i8) as f64;
_8 = [212_u8,110_u8,100_u8,147_u8,241_u8,53_u8];
_26 = _24;
_12 = _5 | _10;
_3 = _9;
_15 = _1;
_25 = RET;
_23 = [_27];
_22 = _26.0;
_7 = [37_u8,211_u8,56_u8,159_u8,229_u8,205_u8];
_27 = !7619125617002325132_u64;
_27 = 5285415310744463499_u64;
_8 = [77_u8,236_u8,69_u8,94_u8,34_u8,20_u8];
RET = -_25;
_19 = [(-101307173_i32),63476227_i32,(-1871784569_i32),(-603223281_i32)];
RET = _25 - _25;
_11 = _13;
_17 = [23302_i16,(-18327_i16),3744_i16,27927_i16,20810_i16,24137_i16,(-12193_i16),(-24497_i16)];
_21 = [123_i8,117_i8,(-86_i8),(-99_i8)];
match _27 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
5285415310744463499 => bb13,
_ => bb12
}
}
bb6 = {
RET = (-29_i8) as f64;
_16 = [199_u8,135_u8,232_u8,5_u8,23_u8,18_u8];
_24.0 = '\u{3b572}';
_27 = !3772986688088422040_u64;
_9 = _14;
_3 = _6;
_14 = _13;
RET = 87976633025526327636354922281902200721_u128 as f64;
_16 = [97_u8,183_u8,13_u8,129_u8,173_u8,140_u8];
_9 = _3;
_26 = (_24.0,);
_7 = [84_u8,175_u8,58_u8,7_u8,24_u8,161_u8];
_15 = _1;
_17 = [(-30814_i16),(-15211_i16),(-1196_i16),32209_i16,24279_i16,(-15364_i16),(-16676_i16),5441_i16];
_7 = [118_u8,148_u8,45_u8,49_u8,224_u8,156_u8];
_1 = [_27,_27,_27,_27,_27];
_24 = (_26.0,);
_9 = [97_u8,26_u8,181_u8,228_u8,183_u8,219_u8];
_5 = 191328066_i32 as isize;
RET = (-8104522842205681224948673276558604858_i128) as f64;
Goto(bb5)
}
bb7 = {
_17 = [4739_i16,(-13641_i16),13050_i16,10906_i16,29408_i16,(-14922_i16),(-23885_i16),20374_i16];
_16 = _11;
_11 = [106_u8,27_u8,73_u8,10_u8,212_u8,15_u8];
_8 = [209_u8,5_u8,81_u8,97_u8,142_u8,79_u8];
_18 = [22108_u16,20146_u16,25772_u16,18759_u16,16151_u16,61496_u16];
_15 = _1;
_14 = _6;
_13 = [244_u8,223_u8,251_u8,154_u8,174_u8,175_u8];
_8 = [90_u8,226_u8,63_u8,183_u8,126_u8,1_u8];
_3 = _8;
_21 = [(-71_i8),(-24_i8),(-1_i8),71_i8];
_8 = [120_u8,162_u8,168_u8,82_u8,61_u8,94_u8];
_23 = [16694826302760582339_u64];
_8 = [207_u8,212_u8,111_u8,195_u8,15_u8,45_u8];
_14 = [172_u8,162_u8,212_u8,220_u8,131_u8,1_u8];
_10 = -_5;
_18 = [4138_u16,57211_u16,17914_u16,15000_u16,7710_u16,59857_u16];
_18 = [27940_u16,24628_u16,40935_u16,18876_u16,44200_u16,57066_u16];
_10 = _12;
_3 = [224_u8,174_u8,196_u8,75_u8,247_u8,217_u8];
Goto(bb4)
}
bb8 = {
_16 = [233_u8,23_u8,78_u8,143_u8,39_u8,46_u8];
_7 = [188_u8,152_u8,43_u8,248_u8,103_u8,208_u8];
_11 = [235_u8,141_u8,77_u8,162_u8,30_u8,71_u8];
_19 = [1739221156_i32,(-832981718_i32),2031927549_i32,1304888576_i32];
_11 = [120_u8,132_u8,43_u8,223_u8,146_u8,95_u8];
_8 = [208_u8,75_u8,41_u8,7_u8,7_u8,240_u8];
_14 = _11;
RET = 2427587867197342974_u64 as f64;
_4 = _16;
_11 = [229_u8,170_u8,137_u8,192_u8,235_u8,210_u8];
_10 = _5;
Goto(bb3)
}
bb9 = {
RET = _5 as f64;
_17 = [(-12608_i16),17595_i16,(-25233_i16),(-23860_i16),(-15271_i16),7994_i16,(-10834_i16),14191_i16];
RET = 247_u8 as f64;
_16 = [67_u8,98_u8,204_u8,204_u8,20_u8,1_u8];
_6 = [227_u8,220_u8,54_u8,59_u8,129_u8,25_u8];
_13 = _4;
_8 = [144_u8,116_u8,9_u8,199_u8,60_u8,180_u8];
_15 = _1;
_4 = [11_u8,230_u8,144_u8,11_u8,152_u8,97_u8];
_13 = [244_u8,15_u8,44_u8,56_u8,39_u8,254_u8];
_12 = _5;
_17 = [(-9506_i16),(-6918_i16),29694_i16,(-15881_i16),(-30647_i16),32003_i16,32582_i16,(-199_i16)];
_10 = !_5;
_11 = [38_u8,156_u8,252_u8,15_u8,177_u8,22_u8];
_15 = [5372280354250131565_u64,6630438066170215537_u64,506919951479122792_u64,9055051416503950051_u64,2853443290675388438_u64];
_7 = _8;
RET = 34_u8 as f64;
_3 = [125_u8,159_u8,12_u8,22_u8,221_u8,171_u8];
_10 = 3155916289_u32 as isize;
_5 = _12;
_8 = [204_u8,235_u8,153_u8,179_u8,119_u8,241_u8];
Call(_7 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
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
_17 = [14672_i16,19291_i16,27696_i16,(-3830_i16),2690_i16,19192_i16,(-9029_i16),(-18376_i16)];
_31 = 6_usize + 535037570295358987_usize;
_35 = _27;
_30.0 = _10 - _10;
_3 = [96_u8,132_u8,68_u8,1_u8,80_u8,210_u8];
_32 = (-7393460850935394134086844198953853289_i128) ^ 145686045091512249002978493556183786228_i128;
_31 = 8319999187132001850_usize << _10;
_30.2 = !_30.0;
_35 = 42082_u16 as u64;
_4 = _11;
_29 = _32 as u32;
_22 = _24.0;
_38 = !16333058231825749980017760812023983065_u128;
_12 = !_30.0;
_19 = [768056038_i32,(-1903212188_i32),(-1960222539_i32),(-1118431757_i32)];
_37 = [236_u8,122_u8,83_u8,206_u8,195_u8,240_u8];
Call(_28 = fn11(_9, _30, _16, _9, _11, _3, _12, _38, _21), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_33 = [Field::<(isize, u8)>(Variant(_28, 1), 1).1,Field::<(isize, u8)>(Variant(_28, 1), 1).1,Field::<(isize, u8)>(Variant(_28, 1), 1).1,Field::<(isize, u8)>(Variant(_28, 1), 1).1,Field::<(isize, u8)>(Variant(_28, 1), 1).1,Field::<(isize, u8)>(Variant(_28, 1), 1).1];
_24 = (_26.0,);
SetDiscriminant(_28, 1);
place!(Field::<(char,)>(Variant(_28, 1), 3)) = (_26.0,);
place!(Field::<(char,)>(Variant(_28, 1), 3)).0 = _24.0;
_27 = 3907_u16 as u64;
_17 = [(-26740_i16),(-8413_i16),(-30063_i16),(-10218_i16),(-16465_i16),24348_i16,2857_i16,(-22099_i16)];
_29 = !3967260152_u32;
_29 = 2877283604_u32;
_6 = [74_u8,17_u8,250_u8,97_u8,0_u8,170_u8];
place!(Field::<u128>(Variant(_28, 1), 7)) = 8143_u16 as u128;
place!(Field::<isize>(Variant(_28, 1), 2)) = _30.0 & _30.2;
RET = -_25;
place!(Field::<[u16; 6]>(Variant(_28, 1), 5)) = [1666_u16,8736_u16,38305_u16,61413_u16,59964_u16,62116_u16];
_5 = _10 >> _30.0;
place!(Field::<u32>(Variant(_28, 1), 4)) = _29 << _5;
place!(Field::<usize>(Variant(_28, 1), 6)) = _31 >> _30.0;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(10_usize, 3_usize, Move(_3), 13_usize, Move(_13), 16_usize, Move(_16), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(10_usize, 4_usize, Move(_4), 19_usize, Move(_19), 14_usize, Move(_14), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(10_usize, 5_usize, Move(_5), 7_usize, Move(_7), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(10_usize, 18_usize, Move(_18), 32_usize, Move(_32), 30_usize, Move(_30), 23_usize, Move(_23)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [u8; 6],mut _2: (isize, bool, isize),mut _3: [u8; 6],mut _4: [u8; 6],mut _5: [u8; 6],mut _6: [u8; 6],mut _7: isize,mut _8: u128,mut _9: [i8; 4]) -> Adt48 {
mir! {
type RET = Adt48;
let _10: [u64; 5];
let _11: u16;
let _12: [u16; 6];
let _13: [i16; 8];
let _14: Adt42;
let _15: u128;
let _16: [i8; 4];
let _17: f64;
let _18: (&'static f32,);
let _19: i16;
let _20: Adt52;
let _21: [u64; 5];
let _22: bool;
let _23: [u64; 1];
let _24: Adt50;
let _25: u64;
let _26: *const isize;
let _27: isize;
let _28: [i32; 4];
let _29: [i16; 8];
let _30: u8;
let _31: [u64; 1];
let _32: *const isize;
let _33: u32;
let _34: (isize, u8);
let _35: [i8; 4];
let _36: Adt43;
let _37: Adt56;
let _38: Adt54;
let _39: (isize, bool, isize);
let _40: f32;
let _41: Adt46;
let _42: f32;
let _43: [u64; 5];
let _44: (&'static f32,);
let _45: f64;
let _46: Adt54;
let _47: f64;
let _48: [u16; 6];
let _49: (char,);
let _50: i128;
let _51: u64;
let _52: [u16; 6];
let _53: [i16; 8];
let _54: (char,);
let _55: [u8; 6];
let _56: [i16; 8];
let _57: char;
let _58: (isize, u8);
let _59: [u64; 5];
let _60: [i16; 8];
let _61: [u8; 6];
let _62: usize;
let _63: [u8; 6];
let _64: isize;
let _65: Adt47;
let _66: f32;
let _67: Adt44;
let _68: bool;
let _69: *const u16;
let _70: f32;
let _71: (char,);
let _72: isize;
let _73: [i16; 8];
let _74: [i32; 4];
let _75: ([u64; 1], &'static f32);
let _76: (isize, bool, isize);
let _77: [u64; 5];
let _78: [i16; 8];
let _79: [i16; 8];
let _80: i32;
let _81: [i32; 4];
let _82: (char,);
let _83: u8;
let _84: bool;
let _85: Adt44;
let _86: (char,);
let _87: (isize, u8);
let _88: (char,);
let _89: u16;
let _90: [u64; 1];
let _91: [u16; 6];
let _92: char;
let _93: f32;
let _94: ();
let _95: ();
{
_2.2 = (-60_i8) as isize;
_5 = _1;
_2.1 = _2.0 > _2.0;
_2.0 = _7 ^ _7;
_2.2 = _2.0;
_4 = _1;
_5 = _3;
_8 = 78464999550059779764240667700240202621_u128;
_2.1 = false;
_10 = [7595204261954018904_u64,16357350993121596493_u64,9809891307357683418_u64,8495490644646097096_u64,6876769326919912299_u64];
_5 = [30_u8,212_u8,21_u8,85_u8,74_u8,114_u8];
_6 = _1;
_4 = [112_u8,215_u8,124_u8,114_u8,200_u8,143_u8];
_5 = [144_u8,197_u8,38_u8,201_u8,139_u8,64_u8];
_2 = (_7, false, _7);
_3 = [83_u8,190_u8,41_u8,205_u8,252_u8,75_u8];
_3 = _1;
_1 = _5;
_4 = [108_u8,183_u8,94_u8,29_u8,71_u8,90_u8];
_3 = [44_u8,32_u8,230_u8,42_u8,216_u8,100_u8];
_5 = [0_u8,215_u8,182_u8,191_u8,173_u8,248_u8];
_8 = 25_u8 as u128;
_7 = _2.1 as isize;
_11 = 11892_u16 | 22563_u16;
_12 = [_11,_11,_11,_11,_11,_11];
Call(_4 = fn12(_2.1, _7, _7, _7, _2, _2.2, _3, _10, _7, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = 221124076762339876313929009702744034972_u128;
_10 = [6550762054571878266_u64,2964782471309409586_u64,14230682004824315515_u64,3030166524012958228_u64,5164191331063341873_u64];
_13 = [906_i16,23516_i16,23159_i16,25463_i16,(-32595_i16),(-28043_i16),(-23506_i16),(-7752_i16)];
_2 = (_7, false, _7);
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
221124076762339876313929009702744034972 => bb9,
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
_3 = [69_u8,152_u8,212_u8,50_u8,145_u8,165_u8];
_8 = 289949244703338266313277141400588010921_u128 - 185506368314469511051257695954262760499_u128;
Goto(bb10)
}
bb10 = {
_3 = _1;
_16 = [(-115_i8),(-33_i8),49_i8,102_i8];
_1 = [17_u8,207_u8,80_u8,96_u8,155_u8,116_u8];
_4 = _5;
_15 = _8 | _8;
_15 = _8;
_13 = [17118_i16,556_i16,(-1805_i16),5833_i16,(-26977_i16),15663_i16,(-3684_i16),(-7516_i16)];
_17 = 9743312139914145919_u64 as f64;
_7 = (-448708499_i32) as isize;
_4 = [19_u8,227_u8,197_u8,135_u8,219_u8,97_u8];
_8 = _15;
_2.2 = 25660_i16 as isize;
_20.fld5.fld1 = !(-499008502558480171_i64);
_2 = (_7, true, _7);
_8 = 1_u8 as u128;
_11 = !59568_u16;
_20.fld6 = [1395809792_i32,1239432026_i32,(-288360898_i32),2072030810_i32];
_1 = [137_u8,250_u8,179_u8,35_u8,149_u8,143_u8];
_20.fld3.0 = '\u{17082}';
_20.fld2.1 = 642825728511813089_u64;
Goto(bb11)
}
bb11 = {
_20.fld5.fld3 = _16;
Goto(bb12)
}
bb12 = {
_12 = [_11,_11,_11,_11,_11,_11];
_20.fld2.2 = core::ptr::addr_of!(_2.2);
_10 = [_20.fld2.1,_20.fld2.1,_20.fld2.1,_20.fld2.1,_20.fld2.1];
_22 = _15 < _15;
_8 = _15 ^ _15;
_15 = !_8;
Goto(bb13)
}
bb13 = {
_1 = [239_u8,78_u8,186_u8,15_u8,124_u8,139_u8];
_17 = 7_usize as f64;
_20.fld1 = _20.fld3.0;
_17 = _11 as f64;
_2 = (_7, _22, _7);
_20.fld2.0 = _20.fld1;
_20.fld5.fld2 = _8 | _15;
_6 = _1;
_11 = 62873_u16;
_20.fld3 = (_20.fld1,);
_20.fld1 = _20.fld2.0;
_25 = (-18344_i16) as u64;
_10 = [_25,_25,_25,_20.fld2.1,_20.fld2.1];
_23 = [_20.fld2.1];
_19 = !13990_i16;
_26 = core::ptr::addr_of!(_7);
Goto(bb14)
}
bb14 = {
_5 = [195_u8,142_u8,252_u8,92_u8,246_u8,191_u8];
_24 = Adt50::Variant3 { fld0: _20.fld6 };
_27 = (*_26) >> _20.fld5.fld2;
_20.fld6 = [982308718_i32,1232309062_i32,(-1534260320_i32),(-823278549_i32)];
_2.2 = _27 >> _20.fld5.fld2;
_12 = [_11,_11,_11,_11,_11,_11];
_19 = 14058_i16;
_2.0 = _2.2;
_29 = _13;
_20.fld3.0 = _20.fld1;
_28 = Field::<[i32; 4]>(Variant(_24, 3), 0);
_30 = !127_u8;
_2.2 = -_2.0;
SetDiscriminant(_24, 3);
_24 = Adt50::Variant3 { fld0: _28 };
_21 = [_20.fld2.1,_25,_25,_25,_20.fld2.1];
_20.fld2 = (_20.fld1, _25, _26);
(*_26) = !_2.2;
_23 = [_25];
_2.1 = _2.2 <= _2.2;
_20.fld0 = _12;
_28 = [335507002_i32,(-768181475_i32),(-1241986224_i32),(-1499038090_i32)];
_32 = _26;
Goto(bb15)
}
bb15 = {
_20.fld5.fld1 = (-4966456660799729680_i64) | 265771509816516355_i64;
_21 = [_25,_25,_20.fld2.1,_25,_25];
(*_26) = _2.2 << _15;
_20.fld5.fld2 = _20.fld2.0 as u128;
(*_32) = -_2.2;
_20.fld2 = (_20.fld3.0, _25, _32);
_2.2 = 1437162352_u32 as isize;
(*_32) = _30 as isize;
match _11 {
0 => bb10,
1 => bb14,
2 => bb5,
3 => bb4,
4 => bb16,
5 => bb17,
62873 => bb19,
_ => bb18
}
}
bb16 = {
_5 = [195_u8,142_u8,252_u8,92_u8,246_u8,191_u8];
_24 = Adt50::Variant3 { fld0: _20.fld6 };
_27 = (*_26) >> _20.fld5.fld2;
_20.fld6 = [982308718_i32,1232309062_i32,(-1534260320_i32),(-823278549_i32)];
_2.2 = _27 >> _20.fld5.fld2;
_12 = [_11,_11,_11,_11,_11,_11];
_19 = 14058_i16;
_2.0 = _2.2;
_29 = _13;
_20.fld3.0 = _20.fld1;
_28 = Field::<[i32; 4]>(Variant(_24, 3), 0);
_30 = !127_u8;
_2.2 = -_2.0;
SetDiscriminant(_24, 3);
_24 = Adt50::Variant3 { fld0: _28 };
_21 = [_20.fld2.1,_25,_25,_25,_20.fld2.1];
_20.fld2 = (_20.fld1, _25, _26);
(*_26) = !_2.2;
_23 = [_25];
_2.1 = _2.2 <= _2.2;
_20.fld0 = _12;
_28 = [335507002_i32,(-768181475_i32),(-1241986224_i32),(-1499038090_i32)];
_32 = _26;
Goto(bb15)
}
bb17 = {
_8 = 221124076762339876313929009702744034972_u128;
_10 = [6550762054571878266_u64,2964782471309409586_u64,14230682004824315515_u64,3030166524012958228_u64,5164191331063341873_u64];
_13 = [906_i16,23516_i16,23159_i16,25463_i16,(-32595_i16),(-28043_i16),(-23506_i16),(-7752_i16)];
_2 = (_7, false, _7);
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
221124076762339876313929009702744034972 => bb9,
_ => bb8
}
}
bb18 = {
_12 = [_11,_11,_11,_11,_11,_11];
_20.fld2.2 = core::ptr::addr_of!(_2.2);
_10 = [_20.fld2.1,_20.fld2.1,_20.fld2.1,_20.fld2.1,_20.fld2.1];
_22 = _15 < _15;
_8 = _15 ^ _15;
_15 = !_8;
Goto(bb13)
}
bb19 = {
_20.fld2.2 = _32;
_19 = !(-9056_i16);
_23 = [_25];
_17 = _2.0 as f64;
_11 = !25225_u16;
_16 = [75_i8,84_i8,(-96_i8),(-87_i8)];
SetDiscriminant(_24, 0);
_34 = (_27, _30);
place!(Field::<(char,)>(Variant(_24, 0), 1)) = (_20.fld2.0,);
(*_32) = _2.0 + _2.0;
_19 = 25617_i16 << _15;
_35 = _9;
_23 = [_25];
_39 = _2;
place!(Field::<u64>(Variant(_24, 0), 0)) = _25;
_8 = _15;
_15 = _8 * _8;
_20.fld0 = [_11,_11,_11,_11,_11,_11];
_33 = !3389946385_u32;
Goto(bb20)
}
bb20 = {
_20.fld1 = _20.fld3.0;
_41.fld1 = -_20.fld5.fld1;
_32 = core::ptr::addr_of!(_2.2);
_15 = _8;
_20.fld5.fld2 = (-1688607428_i32) as u128;
_32 = _26;
_31 = [Field::<u64>(Variant(_24, 0), 0)];
_16 = [(-76_i8),120_i8,8_i8,(-97_i8)];
_20.fld3 = Field::<(char,)>(Variant(_24, 0), 1);
_9 = _16;
_39.0 = !_2.0;
_29 = [_19,_19,_19,_19,_19,_19,_19,_19];
_39 = _2;
_20.fld2.2 = core::ptr::addr_of!(_2.2);
_26 = _32;
_35 = [(-8_i8),(-90_i8),97_i8,(-73_i8)];
Call(_40 = fn18((*_26), (*_26), _39, _2.1, _26, _32, _26), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_42 = -_40;
_27 = _39.0 * _2.0;
_26 = _32;
_28 = [176923078_i32,1061126911_i32,1880275168_i32,692353857_i32];
_20.fld5.fld1 = _41.fld1 | _41.fld1;
_36 = Adt43::Variant1 { fld0: _33 };
Goto(bb22)
}
bb22 = {
_30 = _34.1 * _34.1;
_33 = Field::<u32>(Variant(_36, 1), 0) >> _27;
_7 = _27 ^ _2.0;
_23 = [Field::<u64>(Variant(_24, 0), 0)];
_25 = _11 as u64;
(*_32) = _34.0 << _33;
_28 = [(-2027323042_i32),470060408_i32,1542425309_i32,(-195041686_i32)];
_24 = Adt50::Variant0 { fld0: _25,fld1: _20.fld3 };
_2.2 = _33 as isize;
_9 = [(-40_i8),(-116_i8),26_i8,(-41_i8)];
_20.fld2.2 = core::ptr::addr_of!(_2.2);
_24 = Adt50::Variant1 { fld0: 5_usize };
_21 = _10;
Goto(bb23)
}
bb23 = {
_18.0 = &_40;
_20.fld0 = _12;
_20.fld5.fld1 = !_41.fld1;
_20.fld2.2 = _32;
_34.1 = (*_26) as u8;
_20.fld5.fld3 = [44_i8,99_i8,12_i8,78_i8];
Goto(bb24)
}
bb24 = {
_20.fld5.fld1 = _41.fld1 | _41.fld1;
_45 = -_17;
_39.0 = -_27;
_9 = [(-41_i8),33_i8,(-93_i8),(-70_i8)];
_47 = -_17;
_30 = !_34.1;
_21 = _10;
_20.fld2 = (_20.fld1, _25, _26);
_20.fld5.fld1 = _19 as i64;
_22 = !_39.1;
_39.2 = _2.2 >> _30;
_13 = _29;
_35 = _16;
_2.2 = _39.2 + (*_32);
_24 = Adt50::Variant0 { fld0: _25,fld1: _20.fld3 };
_20.fld5.fld0 = core::ptr::addr_of_mut!(_28);
_11 = _25 as u16;
_10 = _21;
_49 = Field::<(char,)>(Variant(_24, 0), 1);
(*_32) = -_34.0;
Call((*_26) = core::intrinsics::bswap(_27), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_44.0 = &_40;
_10 = _21;
_4 = _5;
Goto(bb26)
}
bb26 = {
_50 = 133755123565992317270484016987735859229_i128 & (-93231737784070928366791860404325707758_i128);
_51 = !Field::<u64>(Variant(_24, 0), 0);
_18.0 = Move(_44.0);
_21 = [_25,_25,_20.fld2.1,_25,_51];
_31 = [_25];
_2 = ((*_32), _39.1, (*_26));
_41.fld2 = core::ptr::addr_of_mut!(_6);
_16 = [22_i8,(-72_i8),(-76_i8),9_i8];
_2.1 = _22;
_49.0 = _20.fld2.0;
_41.fld2 = core::ptr::addr_of_mut!(_1);
SetDiscriminant(_24, 0);
_16 = _35;
_34.1 = !_30;
_39.2 = _2.0;
_20.fld5.fld1 = _41.fld1;
_9 = _16;
_26 = _20.fld2.2;
_39 = _2;
Goto(bb27)
}
bb27 = {
SetDiscriminant(_36, 0);
place!(Field::<Adt41>(Variant(_36, 0), 0)).fld2 = _8;
place!(Field::<Adt41>(Variant(_36, 0), 0)).fld0 = (_20.fld2.0,);
_20.fld3.0 = _20.fld2.0;
_36 = Adt43::Variant1 { fld0: _33 };
SetDiscriminant(_36, 0);
_35 = [(-32_i8),58_i8,5_i8,(-69_i8)];
_39.2 = (*_26) | _27;
_12 = [_11,_11,_11,_11,_11,_11];
place!(Field::<Adt41>(Variant(_36, 0), 0)).fld0.0 = _49.0;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_36, 0), 2)).1 = (_49.0, _25, _32);
_1 = _5;
_35 = _16;
_16 = [52_i8,(-98_i8),(-38_i8),(-22_i8)];
_5 = _4;
_51 = _39.1 as u64;
_41.fld3 = core::ptr::addr_of!(_42);
_20.fld2.0 = _49.0;
place!(Field::<u64>(Variant(_24, 0), 0)) = !_51;
Goto(bb28)
}
bb28 = {
_39.0 = _39.2 ^ _7;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_36, 0), 2)).1.2 = core::ptr::addr_of!(_7);
_7 = _39.0;
_20.fld3 = (_49.0,);
_40 = _42 * _42;
Goto(bb29)
}
bb29 = {
place!(Field::<Adt41>(Variant(_36, 0), 0)).fld2 = !_15;
_20.fld5.fld0 = core::ptr::addr_of_mut!(_28);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_36, 0), 2)).3 = !780694491_i32;
_49 = (_20.fld1,);
_58.1 = _30 * _30;
Goto(bb30)
}
bb30 = {
_55 = [_58.1,_34.1,_30,_34.1,_58.1,_58.1];
_60 = [_19,_19,_19,_19,_19,_19,_19,_19];
_20.fld2.2 = _26;
_20.fld2.2 = core::ptr::addr_of!(_39.0);
_36 = Adt43::Variant1 { fld0: _33 };
_34 = ((*_32), _58.1);
_49.0 = _20.fld2.0;
_1 = _55;
_20.fld0 = _12;
Goto(bb31)
}
bb31 = {
_39 = (_27, _2.1, _34.0);
_61 = [_58.1,_58.1,_34.1,_58.1,_30,_34.1];
_57 = _49.0;
_24 = Adt50::Variant0 { fld0: _51,fld1: _20.fld3 };
Goto(bb32)
}
bb32 = {
_50 = !(-104705558430238558495725244639331150566_i128);
_54 = (_20.fld3.0,);
_26 = core::ptr::addr_of!(_58.0);
_13 = _60;
_20.fld5.fld1 = -_41.fld1;
_42 = -_40;
_60 = [_19,_19,_19,_19,_19,_19,_19,_19];
_3 = [_34.1,_58.1,_30,_30,_34.1,_34.1];
_15 = _8;
_2.2 = -(*_32);
_20.fld2.1 = !_51;
_46 = Adt54::Variant1 { fld0: _58.1 };
_25 = _51 & _51;
(*_26) = _41.fld1 as isize;
Goto(bb33)
}
bb33 = {
_20.fld0 = [_11,_11,_11,_11,_11,_11];
_7 = -(*_26);
_48 = [_11,_11,_11,_11,_11,_11];
_19 = 16125_i16 + (-18139_i16);
_66 = _40 * _40;
(*_26) = _17 as isize;
place!(Field::<u64>(Variant(_24, 0), 0)) = _25;
SetDiscriminant(_46, 2);
place!(Field::<(char,)>(Variant(_24, 0), 1)).0 = _54.0;
_20.fld5.fld0 = core::ptr::addr_of_mut!(_28);
SetDiscriminant(_36, 1);
place!(Field::<*mut [u8; 6]>(Variant(_46, 2), 4)) = _41.fld2;
_65 = Adt47 { fld0: _20.fld5.fld0,fld1: _41.fld1,fld2: _15,fld3: _35 };
_3 = _61;
(*_26) = _65.fld1 as isize;
_2.2 = _33 as isize;
_68 = _27 > (*_26);
place!(Field::<[u64; 5]>(Variant(_46, 2), 2)) = [_51,_25,_51,_20.fld2.1,_51];
_66 = _40;
_39.1 = _2.1;
_69 = core::ptr::addr_of!(_11);
_2 = (_39.2, _39.1, (*_26));
place!(Field::<*const u16>(Variant(_46, 2), 5)) = core::ptr::addr_of!((*_69));
_53 = [_19,_19,_19,_19,_19,_19,_19,_19];
_9 = [(-61_i8),(-24_i8),52_i8,90_i8];
_4 = [_30,_30,_34.1,_58.1,_34.1,_34.1];
Goto(bb34)
}
bb34 = {
place!(Field::<[i32; 4]>(Variant(_46, 2), 6)) = [(-550424488_i32),171498655_i32,(-103804211_i32),672822016_i32];
place!(Field::<(isize, bool, isize)>(Variant(_46, 2), 0)).1 = _39.1;
_52 = _12;
_65 = Adt47 { fld0: _20.fld5.fld0,fld1: _41.fld1,fld2: _8,fld3: _20.fld5.fld3 };
_11 = 1804_u16;
_65.fld3 = [31_i8,48_i8,78_i8,105_i8];
_65.fld0 = core::ptr::addr_of_mut!(_20.fld6);
Goto(bb35)
}
bb35 = {
_39.2 = _34.0;
_20.fld5 = Adt47 { fld0: _65.fld0,fld1: _65.fld1,fld2: _65.fld2,fld3: _9 };
place!(Field::<*const isize>(Variant(_46, 2), 3)) = _20.fld2.2;
_71.0 = Field::<(char,)>(Variant(_24, 0), 1).0;
_71.0 = _20.fld1;
_56 = [_19,_19,_19,_19,_19,_19,_19,_19];
_63 = [_34.1,_30,_34.1,_58.1,_34.1,_58.1];
_55 = _1;
_74 = [(-302348302_i32),675422562_i32,385249582_i32,696311350_i32];
SetDiscriminant(_24, 0);
RET = Adt48::Variant1 { fld0: Field::<*const u16>(Variant(_46, 2), 5),fld1: _34,fld2: _34.0,fld3: _54,fld4: _33,fld5: _12,fld6: 13464046934984961231_usize,fld7: _65.fld2 };
_35 = [(-63_i8),(-40_i8),68_i8,59_i8];
place!(Field::<(char,)>(Variant(_24, 0), 1)).0 = _49.0;
_30 = _17 as u8;
_15 = Field::<u128>(Variant(RET, 1), 7);
_16 = _20.fld5.fld3;
place!(Field::<u64>(Variant(_24, 0), 0)) = _17 as u64;
Goto(bb36)
}
bb36 = {
_60 = [_19,_19,_19,_19,_19,_19,_19,_19];
_58.1 = !_34.1;
_26 = _20.fld2.2;
_49.0 = Field::<(char,)>(Variant(_24, 0), 1).0;
_34 = (Field::<isize>(Variant(RET, 1), 2), Field::<(isize, u8)>(Variant(RET, 1), 1).1);
place!(Field::<u32>(Variant(RET, 1), 4)) = _33;
_65.fld3 = _35;
Goto(bb37)
}
bb37 = {
_69 = core::ptr::addr_of!((*_69));
_64 = _34.0 - Field::<(isize, u8)>(Variant(RET, 1), 1).0;
_19 = _20.fld3.0 as i16;
_76 = (Field::<isize>(Variant(RET, 1), 2), _39.1, _64);
_76.0 = -_34.0;
place!(Field::<u32>(Variant(_36, 1), 0)) = !_33;
_73 = [_19,_19,_19,_19,_19,_19,_19,_19];
_15 = !_65.fld2;
place!(Field::<(isize, u8)>(Variant(RET, 1), 1)).0 = (*_69) as isize;
place!(Field::<*mut [u8; 6]>(Variant(_46, 2), 4)) = core::ptr::addr_of_mut!(_6);
_43 = Field::<[u64; 5]>(Variant(_46, 2), 2);
place!(Field::<(char,)>(Variant(RET, 1), 3)) = (_20.fld3.0,);
place!(Field::<[i32; 4]>(Variant(_46, 2), 6)) = [(-1772662655_i32),1653817847_i32,840957588_i32,477469818_i32];
_20.fld3.0 = _57;
_3 = [_58.1,_30,Field::<(isize, u8)>(Variant(RET, 1), 1).1,_34.1,_34.1,_58.1];
_2.2 = _17 as isize;
(*_26) = _64 >> _20.fld2.1;
_62 = 4_usize;
SetDiscriminant(_24, 2);
place!(Field::<Adt41>(Variant(_24, 2), 3)).fld1.2 = 1887036958_i32 * (-1704578309_i32);
place!(Field::<Adt41>(Variant(_24, 2), 3)).fld0.0 = _49.0;
Goto(bb38)
}
bb38 = {
_41.fld1 = _20.fld5.fld1 & _20.fld5.fld1;
_4 = [_63[_62],_55[_62],_3[_62],_58.1,_1[_62],_55[_62]];
_26 = Field::<*const isize>(Variant(_46, 2), 3);
_68 = Field::<u32>(Variant(_36, 1), 0) != Field::<u32>(Variant(_36, 1), 0);
place!(Field::<(isize, bool, isize)>(Variant(_24, 2), 4)).2 = (*_26);
_29 = _13;
match _6[_62] {
0 => bb27,
1 => bb23,
2 => bb22,
3 => bb26,
124 => bb39,
_ => bb19
}
}
bb39 = {
place!(Field::<Adt41>(Variant(_24, 2), 3)).fld1.1 = -Field::<Adt41>(Variant(_24, 2), 3).fld1.2;
_59 = Field::<[u64; 5]>(Variant(_46, 2), 2);
_43 = [_51,Field::<[u64; 5]>(Variant(_46, 2), 2)[_62],_59[_62],_25,Field::<[u64; 5]>(Variant(_46, 2), 2)[_62]];
place!(Field::<(char,)>(Variant(RET, 1), 3)).0 = _20.fld1;
place!(Field::<(isize, bool, isize)>(Variant(_24, 2), 4)).0 = _2.2;
_78 = [_29[_62],_13[_62],_29[_62],_53[_62],_19,_29[_62],_53[_62],_29[_62]];
_42 = _50 as f32;
place!(Field::<(isize, u8)>(Variant(_24, 2), 1)).0 = 91_i8 as isize;
place!(Field::<(isize, bool, isize)>(Variant(_46, 2), 0)).0 = -_2.0;
_73[_62] = _29[_62];
_23 = [_25];
place!(Field::<[u64; 5]>(Variant(_46, 2), 2)) = _43;
_44.0 = &_66;
_20.fld2 = (_71.0, _25, Field::<*const isize>(Variant(_46, 2), 3));
_60[_62] = _1[_62] as i16;
_39.2 = (*_26) ^ Field::<(isize, bool, isize)>(Variant(_46, 2), 0).0;
_19 = !_60[_62];
place!(Field::<(isize, bool, isize)>(Variant(_24, 2), 4)).2 = _39.2 >> Field::<isize>(Variant(RET, 1), 2);
SetDiscriminant(_36, 1);
place!(Field::<[i32; 4]>(Variant(_46, 2), 6)) = _74;
_80 = Field::<Adt41>(Variant(_24, 2), 3).fld1.1;
place!(Field::<[u16; 6]>(Variant(RET, 1), 5))[_62] = _12[_62];
_72 = _22 as isize;
place!(Field::<[u64; 5]>(Variant(_24, 2), 5)) = [_25,Field::<[u64; 5]>(Variant(_46, 2), 2)[_62],_59[_62],_20.fld2.1,_25];
place!(Field::<(isize, bool, isize)>(Variant(_46, 2), 0)).0 = (*_26) << Field::<(isize, u8)>(Variant(RET, 1), 1).1;
place!(Field::<(char,)>(Variant(RET, 1), 3)) = (_49.0,);
match _6[_62] {
124 => bb40,
_ => bb6
}
}
bb40 = {
place!(Field::<*const isize>(Variant(_46, 2), 3)) = _26;
match (*_69) {
0 => bb41,
1 => bb42,
2 => bb43,
3 => bb44,
1804 => bb46,
_ => bb45
}
}
bb41 = {
place!(Field::<Adt41>(Variant(_24, 2), 3)).fld1.1 = -Field::<Adt41>(Variant(_24, 2), 3).fld1.2;
_59 = Field::<[u64; 5]>(Variant(_46, 2), 2);
_43 = [_51,Field::<[u64; 5]>(Variant(_46, 2), 2)[_62],_59[_62],_25,Field::<[u64; 5]>(Variant(_46, 2), 2)[_62]];
place!(Field::<(char,)>(Variant(RET, 1), 3)).0 = _20.fld1;
place!(Field::<(isize, bool, isize)>(Variant(_24, 2), 4)).0 = _2.2;
_78 = [_29[_62],_13[_62],_29[_62],_53[_62],_19,_29[_62],_53[_62],_29[_62]];
_42 = _50 as f32;
place!(Field::<(isize, u8)>(Variant(_24, 2), 1)).0 = 91_i8 as isize;
place!(Field::<(isize, bool, isize)>(Variant(_46, 2), 0)).0 = -_2.0;
_73[_62] = _29[_62];
_23 = [_25];
place!(Field::<[u64; 5]>(Variant(_46, 2), 2)) = _43;
_44.0 = &_66;
_20.fld2 = (_71.0, _25, Field::<*const isize>(Variant(_46, 2), 3));
_60[_62] = _1[_62] as i16;
_39.2 = (*_26) ^ Field::<(isize, bool, isize)>(Variant(_46, 2), 0).0;
_19 = !_60[_62];
place!(Field::<(isize, bool, isize)>(Variant(_24, 2), 4)).2 = _39.2 >> Field::<isize>(Variant(RET, 1), 2);
SetDiscriminant(_36, 1);
place!(Field::<[i32; 4]>(Variant(_46, 2), 6)) = _74;
_80 = Field::<Adt41>(Variant(_24, 2), 3).fld1.1;
place!(Field::<[u16; 6]>(Variant(RET, 1), 5))[_62] = _12[_62];
_72 = _22 as isize;
place!(Field::<[u64; 5]>(Variant(_24, 2), 5)) = [_25,Field::<[u64; 5]>(Variant(_46, 2), 2)[_62],_59[_62],_20.fld2.1,_25];
place!(Field::<(isize, bool, isize)>(Variant(_46, 2), 0)).0 = (*_26) << Field::<(isize, u8)>(Variant(RET, 1), 1).1;
place!(Field::<(char,)>(Variant(RET, 1), 3)) = (_49.0,);
match _6[_62] {
124 => bb40,
_ => bb6
}
}
bb42 = {
place!(Field::<Adt41>(Variant(_36, 0), 0)).fld2 = !_15;
_20.fld5.fld0 = core::ptr::addr_of_mut!(_28);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_36, 0), 2)).3 = !780694491_i32;
_49 = (_20.fld1,);
_58.1 = _30 * _30;
Goto(bb30)
}
bb43 = {
_8 = 221124076762339876313929009702744034972_u128;
_10 = [6550762054571878266_u64,2964782471309409586_u64,14230682004824315515_u64,3030166524012958228_u64,5164191331063341873_u64];
_13 = [906_i16,23516_i16,23159_i16,25463_i16,(-32595_i16),(-28043_i16),(-23506_i16),(-7752_i16)];
_2 = (_7, false, _7);
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
221124076762339876313929009702744034972 => bb9,
_ => bb8
}
}
bb44 = {
_20.fld5.fld1 = (-4966456660799729680_i64) | 265771509816516355_i64;
_21 = [_25,_25,_20.fld2.1,_25,_25];
(*_26) = _2.2 << _15;
_20.fld5.fld2 = _20.fld2.0 as u128;
(*_32) = -_2.2;
_20.fld2 = (_20.fld3.0, _25, _32);
_2.2 = 1437162352_u32 as isize;
(*_32) = _30 as isize;
match _11 {
0 => bb10,
1 => bb14,
2 => bb5,
3 => bb4,
4 => bb16,
5 => bb17,
62873 => bb19,
_ => bb18
}
}
bb45 = {
Return()
}
bb46 = {
_12[_62] = _52[_62];
_71 = (_49.0,);
place!(Field::<(isize, bool, isize)>(Variant(_46, 2), 0)).2 = -_2.0;
place!(Field::<u32>(Variant(RET, 1), 4)) = !_33;
_80 = !Field::<Adt41>(Variant(_24, 2), 3).fld1.2;
_2 = ((*_26), _68, _39.2);
_65 = _20.fld5;
_10[_62] = _25;
_35 = _16;
place!(Field::<(isize, u8)>(Variant(_24, 2), 1)) = (_2.0, _55[_62]);
_70 = -_66;
_2.0 = _72;
_80 = Field::<Adt41>(Variant(_24, 2), 3).fld1.2;
_61 = [_1[_62],_1[_62],_4[_62],_58.1,_4[_62],_34.1];
place!(Field::<Adt41>(Variant(_24, 2), 3)).fld0 = (Field::<(char,)>(Variant(RET, 1), 3).0,);
_24 = Adt50::Variant0 { fld0: _51,fld1: _54 };
SetDiscriminant(_24, 1);
match _6[_62] {
0 => bb47,
1 => bb48,
2 => bb49,
124 => bb51,
_ => bb50
}
}
bb47 = {
_20.fld5.fld1 = _41.fld1 | _41.fld1;
_45 = -_17;
_39.0 = -_27;
_9 = [(-41_i8),33_i8,(-93_i8),(-70_i8)];
_47 = -_17;
_30 = !_34.1;
_21 = _10;
_20.fld2 = (_20.fld1, _25, _26);
_20.fld5.fld1 = _19 as i64;
_22 = !_39.1;
_39.2 = _2.2 >> _30;
_13 = _29;
_35 = _16;
_2.2 = _39.2 + (*_32);
_24 = Adt50::Variant0 { fld0: _25,fld1: _20.fld3 };
_20.fld5.fld0 = core::ptr::addr_of_mut!(_28);
_11 = _25 as u16;
_10 = _21;
_49 = Field::<(char,)>(Variant(_24, 0), 1);
(*_32) = -_34.0;
Call((*_26) = core::intrinsics::bswap(_27), ReturnTo(bb25), UnwindUnreachable())
}
bb48 = {
_39 = (_27, _2.1, _34.0);
_61 = [_58.1,_58.1,_34.1,_58.1,_30,_34.1];
_57 = _49.0;
_24 = Adt50::Variant0 { fld0: _51,fld1: _20.fld3 };
Goto(bb32)
}
bb49 = {
_8 = 221124076762339876313929009702744034972_u128;
_10 = [6550762054571878266_u64,2964782471309409586_u64,14230682004824315515_u64,3030166524012958228_u64,5164191331063341873_u64];
_13 = [906_i16,23516_i16,23159_i16,25463_i16,(-32595_i16),(-28043_i16),(-23506_i16),(-7752_i16)];
_2 = (_7, false, _7);
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
221124076762339876313929009702744034972 => bb9,
_ => bb8
}
}
bb50 = {
_30 = _34.1 * _34.1;
_33 = Field::<u32>(Variant(_36, 1), 0) >> _27;
_7 = _27 ^ _2.0;
_23 = [Field::<u64>(Variant(_24, 0), 0)];
_25 = _11 as u64;
(*_32) = _34.0 << _33;
_28 = [(-2027323042_i32),470060408_i32,1542425309_i32,(-195041686_i32)];
_24 = Adt50::Variant0 { fld0: _25,fld1: _20.fld3 };
_2.2 = _33 as isize;
_9 = [(-40_i8),(-116_i8),26_i8,(-41_i8)];
_20.fld2.2 = core::ptr::addr_of!(_2.2);
_24 = Adt50::Variant1 { fld0: 5_usize };
_21 = _10;
Goto(bb23)
}
bb51 = {
_65.fld1 = Field::<(char,)>(Variant(RET, 1), 3).0 as i64;
place!(Field::<[u16; 6]>(Variant(RET, 1), 5))[_62] = _11 | _20.fld0[_62];
place!(Field::<*const u16>(Variant(RET, 1), 0)) = core::ptr::addr_of!(_48[_62]);
_4 = _61;
_82.0 = _71.0;
_58.1 = _61[_62] & _55[_62];
_30 = _34.1;
_6[_62] = !_3[_62];
Goto(bb52)
}
bb52 = {
_72 = _33 as isize;
(*_32) = Field::<(isize, bool, isize)>(Variant(_46, 2), 0).2 | _39.0;
_26 = _32;
_65.fld1 = _20.fld5.fld1;
_9 = [(-26_i8),60_i8,91_i8,(-17_i8)];
_20.fld2.2 = _26;
_5 = [_4[_62],_3[_62],_63[_62],_6[_62],_34.1,_6[_62]];
place!(Field::<(isize, u8)>(Variant(RET, 1), 1)).1 = _34.1;
_27 = _80 as isize;
_2.1 = _68;
_81 = _74;
_39.1 = !_22;
_48 = Field::<[u16; 6]>(Variant(RET, 1), 5);
_8 = _51 as u128;
match _11 {
0 => bb33,
1 => bb45,
2 => bb37,
1804 => bb53,
_ => bb28
}
}
bb53 = {
_20.fld5.fld0 = core::ptr::addr_of_mut!(_28);
place!(Field::<usize>(Variant(RET, 1), 6)) = !_62;
_45 = -_17;
_77[_62] = _20.fld2.1;
_65.fld2 = _15;
_56[_62] = _73[_62];
_61 = [_58.1,_55[_62],Field::<(isize, u8)>(Variant(RET, 1), 1).1,_63[_62],_6[_62],_5[_62]];
_75.0 = [_43[_62]];
_58 = Field::<(isize, u8)>(Variant(RET, 1), 1);
Goto(bb54)
}
bb54 = {
Call(_94 = dump_var(11_usize, 74_usize, Move(_74), 57_usize, Move(_57), 63_usize, Move(_63), 73_usize, Move(_73)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_94 = dump_var(11_usize, 68_usize, Move(_68), 51_usize, Move(_51), 64_usize, Move(_64), 30_usize, Move(_30)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_94 = dump_var(11_usize, 71_usize, Move(_71), 59_usize, Move(_59), 43_usize, Move(_43), 78_usize, Move(_78)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_94 = dump_var(11_usize, 11_usize, Move(_11), 2_usize, Move(_2), 53_usize, Move(_53), 27_usize, Move(_27)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_94 = dump_var(11_usize, 8_usize, Move(_8), 21_usize, Move(_21), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_94 = dump_var(11_usize, 80_usize, Move(_80), 50_usize, Move(_50), 12_usize, Move(_12), 28_usize, Move(_28)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_94 = dump_var(11_usize, 82_usize, Move(_82), 5_usize, Move(_5), 9_usize, Move(_9), 31_usize, Move(_31)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: (isize, bool, isize),mut _6: isize,mut _7: [u8; 6],mut _8: [u64; 5],mut _9: isize,mut _10: (isize, bool, isize)) -> [u8; 6] {
mir! {
type RET = [u8; 6];
let _11: f32;
let _12: [i32; 4];
let _13: isize;
let _14: Adt46;
let _15: char;
let _16: char;
let _17: bool;
let _18: u128;
let _19: [i32; 4];
let _20: Adt51;
let _21: f64;
let _22: f64;
let _23: u8;
let _24: f64;
let _25: isize;
let _26: i128;
let _27: f64;
let _28: *const f32;
let _29: usize;
let _30: [i8; 4];
let _31: [u64; 1];
let _32: Adt44;
let _33: Adt50;
let _34: f32;
let _35: isize;
let _36: ();
let _37: ();
{
RET = _7;
_5.2 = -_9;
_5.0 = 80_i8 as isize;
_6 = _5.2;
_7 = [34_u8,230_u8,51_u8,181_u8,124_u8,113_u8];
_14.fld2 = core::ptr::addr_of_mut!(_7);
_10.0 = -_9;
_14.fld3 = core::ptr::addr_of!(_11);
_5.2 = _5.1 as isize;
_13 = _10.2;
_3 = _10.2;
_12 = [(-2076043930_i32),(-1966834055_i32),835650660_i32,1333558877_i32];
_18 = !230846886770565514002212800017234681734_u128;
_15 = '\u{3013b}';
_17 = !_10.1;
RET = [71_u8,205_u8,64_u8,200_u8,156_u8,196_u8];
_15 = '\u{e8f0a}';
RET = [177_u8,155_u8,110_u8,102_u8,69_u8,251_u8];
_15 = '\u{1075d1}';
_4 = _5.2 - _9;
_5.2 = 49211_u16 as isize;
_4 = 1576933757_u32 as isize;
Goto(bb1)
}
bb1 = {
_11 = 2076104296_i32 as f32;
_10 = _5;
_21 = _18 as f64;
_17 = !_1;
_9 = _5.2;
_6 = !_10.0;
_5.0 = -_3;
_13 = -_10.2;
_8 = [12458834031235053783_u64,6969742393136374852_u64,8896606905108502871_u64,3327543674685693640_u64,11283808639700716929_u64];
_5.0 = 407050608259760479_u64 as isize;
Goto(bb2)
}
bb2 = {
_11 = 16752_i16 as f32;
_10 = (_2, _1, _2);
_14.fld3 = core::ptr::addr_of!(_11);
RET = [89_u8,148_u8,14_u8,19_u8,165_u8,56_u8];
_22 = -_21;
_16 = _15;
_10.2 = _3 & _5.2;
_13 = !_2;
Call(_14.fld2 = fn13(_10.1, _3, _2, _1, _10.1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = _16;
_16 = _15;
_10.0 = _13 | _3;
_2 = !_3;
_8 = [5948435538119661624_u64,15419100451847448964_u64,14824351233313157882_u64,11852612198266709899_u64,3679775100674865068_u64];
_11 = (-99_i8) as f32;
_14.fld2 = core::ptr::addr_of_mut!(RET);
_15 = _16;
_15 = _16;
_18 = _10.0 as u128;
_10 = _5;
_15 = _16;
_5.1 = !_17;
_19 = _12;
_9 = _2;
_2 = _13 | _10.2;
_4 = _2 * _13;
_15 = _16;
_13 = !_4;
_10 = _5;
_10.1 = _1;
_18 = !315703942550399533257062365152305554077_u128;
Call(_5.1 = fn17(_6, _13, _9, _10.2, _4, _13, _10.1, _1, _4, _1, _14.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = _16 as isize;
_5.2 = _2;
_24 = _22 - _21;
_7 = [221_u8,253_u8,174_u8,94_u8,109_u8,8_u8];
_10 = (_5.2, _5.1, _2);
_5.2 = (-17052_i16) as isize;
Goto(bb5)
}
bb5 = {
_14.fld2 = core::ptr::addr_of_mut!(_7);
_23 = 17059211998188858033_usize as u8;
_6 = _13 - _2;
_10.2 = _6;
_14.fld2 = core::ptr::addr_of_mut!(RET);
_18 = !337432462063954405609834423751761186464_u128;
Goto(bb6)
}
bb6 = {
_1 = _5.1;
_17 = _5.1 ^ _1;
_4 = 1_usize as isize;
_6 = _10.0 ^ _10.2;
_21 = _23 as f64;
_1 = !_17;
_26 = (-150864478616680381987954360762470272495_i128);
_6 = _2 << _10.0;
match _26 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
189417888304258081475420246669297938961 => bb13,
_ => bb12
}
}
bb7 = {
_14.fld2 = core::ptr::addr_of_mut!(_7);
_23 = 17059211998188858033_usize as u8;
_6 = _13 - _2;
_10.2 = _6;
_14.fld2 = core::ptr::addr_of_mut!(RET);
_18 = !337432462063954405609834423751761186464_u128;
Goto(bb6)
}
bb8 = {
_9 = _16 as isize;
_5.2 = _2;
_24 = _22 - _21;
_7 = [221_u8,253_u8,174_u8,94_u8,109_u8,8_u8];
_10 = (_5.2, _5.1, _2);
_5.2 = (-17052_i16) as isize;
Goto(bb5)
}
bb9 = {
_15 = _16;
_16 = _15;
_10.0 = _13 | _3;
_2 = !_3;
_8 = [5948435538119661624_u64,15419100451847448964_u64,14824351233313157882_u64,11852612198266709899_u64,3679775100674865068_u64];
_11 = (-99_i8) as f32;
_14.fld2 = core::ptr::addr_of_mut!(RET);
_15 = _16;
_15 = _16;
_18 = _10.0 as u128;
_10 = _5;
_15 = _16;
_5.1 = !_17;
_19 = _12;
_9 = _2;
_2 = _13 | _10.2;
_4 = _2 * _13;
_15 = _16;
_13 = !_4;
_10 = _5;
_10.1 = _1;
_18 = !315703942550399533257062365152305554077_u128;
Call(_5.1 = fn17(_6, _13, _9, _10.2, _4, _13, _10.1, _1, _4, _1, _14.fld2), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_11 = 16752_i16 as f32;
_10 = (_2, _1, _2);
_14.fld3 = core::ptr::addr_of!(_11);
RET = [89_u8,148_u8,14_u8,19_u8,165_u8,56_u8];
_22 = -_21;
_16 = _15;
_10.2 = _3 & _5.2;
_13 = !_2;
Call(_14.fld2 = fn13(_10.1, _3, _2, _1, _10.1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_11 = 2076104296_i32 as f32;
_10 = _5;
_21 = _18 as f64;
_17 = !_1;
_9 = _5.2;
_6 = !_10.0;
_5.0 = -_3;
_13 = -_10.2;
_8 = [12458834031235053783_u64,6969742393136374852_u64,8896606905108502871_u64,3327543674685693640_u64,11283808639700716929_u64];
_5.0 = 407050608259760479_u64 as isize;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_12 = _19;
_1 = _10.1 != _5.1;
_5 = (_13, _1, _2);
_22 = -_21;
_10.1 = _1 == _17;
_19 = [983574079_i32,1418963397_i32,(-1343300352_i32),788970257_i32];
RET = [_23,_23,_23,_23,_23,_23];
_6 = _5.0 & _10.2;
_21 = _18 as f64;
_29 = (-63_i8) as usize;
_1 = _5.1 >= _17;
_10.1 = _17;
_7 = [_23,_23,_23,_23,_23,_23];
_17 = !_5.1;
_26 = 85066207753503622990675107925789391935_i128;
_28 = _14.fld3;
_5.1 = !_10.1;
_25 = !_10.2;
_15 = _16;
_27 = _24;
_2 = _13;
_9 = _6 & _13;
_8 = [7542883707381548601_u64,9561032433846355746_u64,3497577597915419009_u64,14996997446239068552_u64,17131327104109957843_u64];
_5.0 = 24085_u16 as isize;
_14.fld2 = core::ptr::addr_of_mut!(_7);
Goto(bb14)
}
bb14 = {
_14.fld2 = core::ptr::addr_of_mut!(RET);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(12_usize, 25_usize, Move(_25), 13_usize, Move(_13), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(12_usize, 6_usize, Move(_6), 15_usize, Move(_15), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(12_usize, 7_usize, Move(_7), 17_usize, Move(_17), 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: bool,mut _5: bool,mut _6: isize) -> *mut [u8; 6] {
mir! {
type RET = *mut [u8; 6];
let _7: Adt48;
let _8: isize;
let _9: f32;
let _10: Adt56;
let _11: (isize, u8);
let _12: (isize, u8);
let _13: i16;
let _14: *const u64;
let _15: char;
let _16: isize;
let _17: Adt53;
let _18: [u64; 1];
let _19: f32;
let _20: *const u64;
let _21: isize;
let _22: [i8; 4];
let _23: [u8; 6];
let _24: Adt45;
let _25: i64;
let _26: Adt47;
let _27: [u64; 5];
let _28: f32;
let _29: [u8; 6];
let _30: (char, u64, *const isize);
let _31: char;
let _32: isize;
let _33: isize;
let _34: i64;
let _35: (char,);
let _36: [u64; 5];
let _37: [u64; 1];
let _38: (char,);
let _39: u16;
let _40: f64;
let _41: ((*const f32, (char, u64, *const isize), [u64; 1], i32), *const u64, isize);
let _42: [u64; 1];
let _43: ();
let _44: ();
{
_5 = _1;
_1 = _6 >= _3;
_2 = _3 & _3;
_3 = _2 * _6;
_3 = _2;
_6 = !_2;
_1 = !_5;
_2 = _3;
_1 = _2 >= _2;
_6 = -_2;
_9 = 7363_u16 as f32;
_5 = _1;
_1 = _6 < _2;
_8 = _2;
_11 = (_8, 126_u8);
_6 = !_2;
_9 = 50_i8 as f32;
_3 = -_11.0;
_9 = (-103120173144302709244084968472179559740_i128) as f32;
_12.0 = 8719198348299140744_u64 as isize;
_11.1 = 172_u8 << _3;
Goto(bb1)
}
bb1 = {
_11 = (_2, 225_u8);
_13 = 8288857458259661377_u64 as i16;
_6 = (-109862606173112978772136214907528522869_i128) as isize;
_12.1 = 26687_u16 as u8;
_12.1 = !_11.1;
_15 = '\u{d22b1}';
_12 = (_11.0, _11.1);
_15 = '\u{9f1a9}';
_12.0 = _8 + _3;
_1 = !_5;
_2 = _11.0 >> _12.0;
_11.1 = !_12.1;
_8 = (-155559120_i32) as isize;
_12.1 = !_11.1;
_11 = (_2, _12.1);
_15 = '\u{49fb1}';
_3 = 59066969892689242908527001839599795536_u128 as isize;
_12.1 = _11.1 | _11.1;
_11 = (_12.0, _12.1);
_12.0 = _2;
_6 = _12.0 * _11.0;
Goto(bb2)
}
bb2 = {
_3 = 4844252920755493372545681672635866131_i128 as isize;
_11.0 = (-48_i8) as isize;
_19 = _13 as f32;
_3 = 5635484174381295679_i64 as isize;
Goto(bb3)
}
bb3 = {
_2 = _12.0;
_16 = _2 | _6;
_23 = [_11.1,_12.1,_11.1,_12.1,_11.1,_12.1];
_13 = (-2439_i16);
_15 = '\u{39fe5}';
_2 = -_16;
_5 = !_1;
_18 = [8241412575781163190_u64];
_28 = _9;
_25 = (-5914746468225491578_i64) + 5034732121665987001_i64;
_27 = [9173444106235522842_u64,17372884870417219767_u64,13907490358915622688_u64,3396758145576967349_u64,14849007427319712442_u64];
_9 = _28;
_26.fld3 = [116_i8,(-98_i8),37_i8,43_i8];
_27 = [16266346132630814990_u64,10893506584599407397_u64,1332959917306340462_u64,7481520649688837715_u64,454507738126501091_u64];
Goto(bb4)
}
bb4 = {
RET = core::ptr::addr_of_mut!(_29);
_11.1 = _12.1 + _12.1;
_20 = core::ptr::addr_of!(_30.1);
_13 = (-8863_i16);
_25 = (-1919426447104966824_i64);
_4 = _5;
RET = core::ptr::addr_of_mut!((*RET));
_3 = _6;
_19 = _28;
(*RET) = _23;
_13 = (-25721_i16);
_29 = _23;
_35 = (_15,);
(*RET) = _23;
_22 = [58_i8,(-68_i8),(-11_i8),(-46_i8)];
(*_20) = 7272737439894212101_u64;
_33 = -_16;
(*RET) = [_12.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_11 = (_6, _12.1);
_15 = _35.0;
_32 = _33;
(*_20) = (-41097755201439324595613962458604169553_i128) as u64;
_11.1 = 230130829160705751801826074098645145792_u128 as u8;
_4 = _32 >= _3;
_35.0 = _15;
Goto(bb5)
}
bb5 = {
_19 = -_28;
_1 = !_4;
_30.2 = core::ptr::addr_of!(_6);
Call(_21 = core::intrinsics::transmute(_6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12.1 = !_11.1;
match _13 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
340282366920938463463374607431768185735 => bb10,
_ => bb9
}
}
bb7 = {
_3 = 4844252920755493372545681672635866131_i128 as isize;
_11.0 = (-48_i8) as isize;
_19 = _13 as f32;
_3 = 5635484174381295679_i64 as isize;
Goto(bb3)
}
bb8 = {
RET = core::ptr::addr_of_mut!(_29);
_11.1 = _12.1 + _12.1;
_20 = core::ptr::addr_of!(_30.1);
_13 = (-8863_i16);
_25 = (-1919426447104966824_i64);
_4 = _5;
RET = core::ptr::addr_of_mut!((*RET));
_3 = _6;
_19 = _28;
(*RET) = _23;
_13 = (-25721_i16);
_29 = _23;
_35 = (_15,);
(*RET) = _23;
_22 = [58_i8,(-68_i8),(-11_i8),(-46_i8)];
(*_20) = 7272737439894212101_u64;
_33 = -_16;
(*RET) = [_12.1,_11.1,_11.1,_11.1,_11.1,_11.1];
_11 = (_6, _12.1);
_15 = _35.0;
_32 = _33;
(*_20) = (-41097755201439324595613962458604169553_i128) as u64;
_11.1 = 230130829160705751801826074098645145792_u128 as u8;
_4 = _32 >= _3;
_35.0 = _15;
Goto(bb5)
}
bb9 = {
_2 = _12.0;
_16 = _2 | _6;
_23 = [_11.1,_12.1,_11.1,_12.1,_11.1,_12.1];
_13 = (-2439_i16);
_15 = '\u{39fe5}';
_2 = -_16;
_5 = !_1;
_18 = [8241412575781163190_u64];
_28 = _9;
_25 = (-5914746468225491578_i64) + 5034732121665987001_i64;
_27 = [9173444106235522842_u64,17372884870417219767_u64,13907490358915622688_u64,3396758145576967349_u64,14849007427319712442_u64];
_9 = _28;
_26.fld3 = [116_i8,(-98_i8),37_i8,43_i8];
_27 = [16266346132630814990_u64,10893506584599407397_u64,1332959917306340462_u64,7481520649688837715_u64,454507738126501091_u64];
Goto(bb4)
}
bb10 = {
_26.fld1 = !_25;
_33 = _32 >> _32;
_8 = -_16;
_22 = [14_i8,83_i8,49_i8,(-64_i8)];
_22 = _26.fld3;
_35.0 = _15;
_37 = _18;
_34 = _26.fld1;
_1 = !_5;
_38 = (_15,);
_30.1 = _19 as u64;
_18 = [(*_20)];
_16 = -_3;
(*RET) = _23;
_41.0.1.0 = _38.0;
Call(_35 = fn14(_8, _30.2, _20, _16, _33, _12, _12.0, _30.2, _12, (*RET), _11.0, _33, _11.0, _16), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12.0 = _8 * _6;
_18 = [_30.1];
_16 = _34 as isize;
_30.0 = _41.0.1.0;
_16 = _11.0;
_41.0.1 = _30;
_3 = 60181_u16 as isize;
_41.0.2 = [_41.0.1.1];
_38.0 = _35.0;
_41.0.1.1 = !(*_20);
_41.2 = _8;
_14 = _20;
_41.0.3 = _6 as i32;
_20 = core::ptr::addr_of!((*_20));
_26.fld2 = 164184296963535725013656294499834964027_u128;
_42 = [(*_20)];
Goto(bb12)
}
bb12 = {
_32 = 7522_u16 as isize;
(*RET) = _23;
_35 = (_41.0.1.0,);
_30.2 = _41.0.1.2;
_11.1 = !_12.1;
_34 = !_25;
_29 = _23;
_37 = _41.0.2;
_28 = _9 * _9;
_41.0.0 = core::ptr::addr_of!(_9);
RET = core::ptr::addr_of_mut!(_29);
_18 = [_30.1];
_41.0.1 = _30;
_13 = (-2040_i16);
Goto(bb13)
}
bb13 = {
Call(_43 = dump_var(13_usize, 16_usize, Move(_16), 22_usize, Move(_22), 1_usize, Move(_1), 42_usize, Move(_42)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_43 = dump_var(13_usize, 27_usize, Move(_27), 21_usize, Move(_21), 38_usize, Move(_38), 3_usize, Move(_3)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_43 = dump_var(13_usize, 29_usize, Move(_29), 25_usize, Move(_25), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(13_usize, 32_usize, Move(_32), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: *const isize,mut _3: *const u64,mut _4: isize,mut _5: isize,mut _6: (isize, u8),mut _7: isize,mut _8: *const isize,mut _9: (isize, u8),mut _10: [u8; 6],mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> (char,) {
mir! {
type RET = (char,);
let _15: [u16; 6];
let _16: (char,);
let _17: i128;
let _18: isize;
let _19: Adt46;
let _20: Adt43;
let _21: [i32; 4];
let _22: f32;
let _23: bool;
let _24: f64;
let _25: usize;
let _26: (&'static f32,);
let _27: bool;
let _28: [u8; 6];
let _29: [i16; 8];
let _30: [i32; 4];
let _31: [i32; 4];
let _32: f32;
let _33: Adt41;
let _34: char;
let _35: isize;
let _36: u128;
let _37: f64;
let _38: (isize, u8);
let _39: bool;
let _40: isize;
let _41: u16;
let _42: Adt55;
let _43: (isize, u8);
let _44: f64;
let _45: [i8; 4];
let _46: Adt53;
let _47: [u64; 1];
let _48: Adt44;
let _49: u8;
let _50: i64;
let _51: f32;
let _52: [i8; 4];
let _53: i128;
let _54: ();
let _55: ();
{
(*_3) = !686686602105335888_u64;
_4 = (*_8) << _14;
_11 = (-81_i8) as isize;
_14 = (*_2) | (*_8);
_15 = [54849_u16,46598_u16,19131_u16,52231_u16,46645_u16,24751_u16];
Goto(bb1)
}
bb1 = {
_16 = ('\u{41a71}',);
Goto(bb2)
}
bb2 = {
_15 = [31723_u16,41154_u16,40585_u16,2955_u16,43634_u16,46055_u16];
_18 = _5 & (*_8);
(*_8) = _5 ^ _18;
_12 = 5_usize as isize;
_13 = !_9.0;
(*_2) = _9.0 | _6.0;
(*_2) = 11879859804964379430_usize as isize;
_15 = [38010_u16,60193_u16,27998_u16,59048_u16,63777_u16,48358_u16];
_20 = Adt43::Variant1 { fld0: 1610426647_u32 };
_9.1 = 1233352205_i32 as u8;
_19.fld1 = _16.0 as i64;
(*_2) = -_7;
_19.fld1 = (-8460607707801535958_i64);
Call(_21 = fn15(_8, _18, (*_2), _5, _5, (*_2), (*_2), _7, (*_2), (*_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = [1877_u16,9296_u16,28013_u16,1540_u16,15233_u16,28584_u16];
_20 = Adt43::Variant1 { fld0: 1897180529_u32 };
RET.0 = _16.0;
_4 = _6.0 ^ _18;
_7 = (*_2);
_7 = -(*_2);
Goto(bb4)
}
bb4 = {
_4 = -(*_8);
match _19.fld1 {
340282366920938463454913999723966675498 => bb5,
_ => bb3
}
}
bb5 = {
_24 = 16694_u16 as f64;
_11 = !_18;
(*_8) = _11 ^ _1;
place!(Field::<u32>(Variant(_20, 1), 0)) = 88738138560022132977780776285112769684_i128 as u32;
_11 = !_6.0;
(*_8) = _1;
_6.1 = _9.1 ^ _9.1;
_33.fld1.0 = _3;
_13 = _11;
_9.0 = 189589914315051570039553032505092033215_u128 as isize;
_9 = _6;
_28 = [_9.1,_9.1,_6.1,_6.1,_6.1,_9.1];
RET.0 = _16.0;
Call(_6.1 = core::intrinsics::bswap(_9.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_33.fld1.0 = _3;
(*_8) = _18 | _7;
_34 = _16.0;
_33.fld1 = (_3, (-1540441562_i32), (-1824757629_i32));
_24 = (-3028058293869708665731711412922756443_i128) as f64;
_34 = RET.0;
_31 = _21;
(*_3) = 1106636491752268057_u64 & 14130651605704652939_u64;
_33.fld0.0 = _16.0;
_24 = _19.fld1 as f64;
(*_3) = 6547476521434318290_u64;
_1 = !_18;
_30 = _31;
Goto(bb7)
}
bb7 = {
_30 = _21;
SetDiscriminant(_20, 0);
_33.fld1.1 = -_33.fld1.2;
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld2 = 241558405691778447803140940469187887715_u128 ^ 144884359845207225335557449189160552097_u128;
place!(Field::<Adt41>(Variant(_20, 0), 0)) = Adt41 { fld0: RET,fld1: _33.fld1,fld2: 21487113739896807970972393618912869639_u128 };
place!(Field::<u8>(Variant(_20, 0), 5)) = !_9.1;
place!(Field::<u8>(Variant(_20, 0), 5)) = _9.1;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).0 = core::ptr::addr_of!(_22);
_33.fld1.1 = Field::<Adt41>(Variant(_20, 0), 0).fld1.1 >> _9.0;
Goto(bb8)
}
bb8 = {
_19.fld2 = core::ptr::addr_of_mut!(_28);
_6.1 = true as u8;
_33.fld2 = Field::<Adt41>(Variant(_20, 0), 0).fld2 + Field::<Adt41>(Variant(_20, 0), 0).fld2;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).1.2 = core::ptr::addr_of!(_12);
_24 = _33.fld1.1 as f64;
RET = (Field::<Adt41>(Variant(_20, 0), 0).fld0.0,);
_25 = 12460243564237065689_usize >> _11;
_34 = Field::<Adt41>(Variant(_20, 0), 0).fld0.0;
(*_3) = 8817576276438957055_u64;
_17 = (-26057045781813601582315018815619209755_i128);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).2 = [(*_3)];
_2 = _8;
_23 = true;
_17 = 155171388161523467465562908793795912017_i128 << _5;
_19.fld1 = 8475540263311912097_i64;
RET.0 = Field::<Adt41>(Variant(_20, 0), 0).fld0.0;
_28 = [Field::<u8>(Variant(_20, 0), 5),_6.1,Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5)];
_1 = (*_8);
_4 = -_7;
_33 = Adt41 { fld0: _16,fld1: Field::<Adt41>(Variant(_20, 0), 0).fld1,fld2: Field::<Adt41>(Variant(_20, 0), 0).fld2 };
_28 = [Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),_6.1,Field::<u8>(Variant(_20, 0), 5),_9.1];
Goto(bb9)
}
bb9 = {
_16.0 = Field::<Adt41>(Variant(_20, 0), 0).fld0.0;
_15 = [13891_u16,57647_u16,42445_u16,27484_u16,23980_u16,6685_u16];
_37 = -_24;
_32 = _33.fld2 as f32;
_30 = _21;
_33 = Adt41 { fld0: RET,fld1: Field::<Adt41>(Variant(_20, 0), 0).fld1,fld2: Field::<Adt41>(Variant(_20, 0), 0).fld2 };
_25 = !1_usize;
_27 = _23;
(*_2) = _1 - _13;
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld1 = (_33.fld1.0, _33.fld1.2, _33.fld1.1);
RET.0 = _33.fld0.0;
_26.0 = &_22;
place!(Field::<u8>(Variant(_20, 0), 5)) = _9.1 - _9.1;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).1 = (_33.fld0.0, (*_3), _8);
_38.0 = -_18;
_38 = _6;
_31 = _30;
_19.fld1 = (-4997898021013601355_i64) + (-5625807659637086045_i64);
_38 = _6;
_34 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2).1.0;
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld1 = _33.fld1;
_16 = (RET.0,);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).3 = Field::<Adt41>(Variant(_20, 0), 0).fld1.1 << _11;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).3 = _33.fld1.2;
Goto(bb10)
}
bb10 = {
_3 = core::ptr::addr_of!(place!(Field::<u64>(Variant(_20, 0), 3)));
_21 = _30;
_38.0 = 127_i8 as isize;
_9 = (_4, Field::<u8>(Variant(_20, 0), 5));
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld1 = (_33.fld1.0, _33.fld1.2, _33.fld1.2);
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld0.0 = _34;
_2 = core::ptr::addr_of!(_5);
_36 = _33.fld2;
_42.fld3 = core::ptr::addr_of_mut!(_42.fld0);
_27 = _7 < _4;
_38.0 = _19.fld1 as isize;
_38.1 = _25 as u8;
Goto(bb11)
}
bb11 = {
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).1 = (Field::<Adt41>(Variant(_20, 0), 0).fld0.0, 15548296528718191478_u64, _2);
_23 = !_27;
(*_3) = !Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2).1.1;
_7 = _6.0;
_42.fld5 = -_37;
_45 = [(-2_i8),(-76_i8),34_i8,54_i8];
_20 = Adt43::Variant1 { fld0: 662513504_u32 };
_29 = [(-30201_i16),(-27378_i16),(-19509_i16),(-16543_i16),(-19333_i16),28146_i16,19755_i16,(-31833_i16)];
_19.fld3 = core::ptr::addr_of!(_22);
_43.1 = !_38.1;
_14 = -(*_8);
_22 = _32;
_43 = (_14, _9.1);
_33.fld0 = RET;
_44 = _24 + _37;
_17 = (-63749650960379139243236537604143245736_i128) * 118699411816996555488837094337260509840_i128;
_13 = _11;
_34 = _33.fld0.0;
_39 = _4 != _1;
RET = (_16.0,);
_9 = _6;
_19.fld2 = core::ptr::addr_of_mut!(_42.fld0);
_19.fld2 = core::ptr::addr_of_mut!(_10);
_42.fld4 = _45;
_12 = (-1067_i16) as isize;
_27 = !_23;
_49 = !_9.1;
Goto(bb12)
}
bb12 = {
_11 = _1;
RET = (_34,);
_26.0 = &_32;
RET = (_16.0,);
(*_8) = _43.0;
_49 = _9.1;
match _36 {
0 => bb1,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb13,
5 => bb14,
6 => bb15,
21487113739896807970972393618912869639 => bb17,
_ => bb16
}
}
bb13 = {
_24 = 16694_u16 as f64;
_11 = !_18;
(*_8) = _11 ^ _1;
place!(Field::<u32>(Variant(_20, 1), 0)) = 88738138560022132977780776285112769684_i128 as u32;
_11 = !_6.0;
(*_8) = _1;
_6.1 = _9.1 ^ _9.1;
_33.fld1.0 = _3;
_13 = _11;
_9.0 = 189589914315051570039553032505092033215_u128 as isize;
_9 = _6;
_28 = [_9.1,_9.1,_6.1,_6.1,_6.1,_9.1];
RET.0 = _16.0;
Call(_6.1 = core::intrinsics::bswap(_9.1), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_16 = ('\u{41a71}',);
Goto(bb2)
}
bb15 = {
_16.0 = Field::<Adt41>(Variant(_20, 0), 0).fld0.0;
_15 = [13891_u16,57647_u16,42445_u16,27484_u16,23980_u16,6685_u16];
_37 = -_24;
_32 = _33.fld2 as f32;
_30 = _21;
_33 = Adt41 { fld0: RET,fld1: Field::<Adt41>(Variant(_20, 0), 0).fld1,fld2: Field::<Adt41>(Variant(_20, 0), 0).fld2 };
_25 = !1_usize;
_27 = _23;
(*_2) = _1 - _13;
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld1 = (_33.fld1.0, _33.fld1.2, _33.fld1.1);
RET.0 = _33.fld0.0;
_26.0 = &_22;
place!(Field::<u8>(Variant(_20, 0), 5)) = _9.1 - _9.1;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).1 = (_33.fld0.0, (*_3), _8);
_38.0 = -_18;
_38 = _6;
_31 = _30;
_19.fld1 = (-4997898021013601355_i64) + (-5625807659637086045_i64);
_38 = _6;
_34 = Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2).1.0;
place!(Field::<Adt41>(Variant(_20, 0), 0)).fld1 = _33.fld1;
_16 = (RET.0,);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).3 = Field::<Adt41>(Variant(_20, 0), 0).fld1.1 << _11;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).3 = _33.fld1.2;
Goto(bb10)
}
bb16 = {
_19.fld2 = core::ptr::addr_of_mut!(_28);
_6.1 = true as u8;
_33.fld2 = Field::<Adt41>(Variant(_20, 0), 0).fld2 + Field::<Adt41>(Variant(_20, 0), 0).fld2;
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).1.2 = core::ptr::addr_of!(_12);
_24 = _33.fld1.1 as f64;
RET = (Field::<Adt41>(Variant(_20, 0), 0).fld0.0,);
_25 = 12460243564237065689_usize >> _11;
_34 = Field::<Adt41>(Variant(_20, 0), 0).fld0.0;
(*_3) = 8817576276438957055_u64;
_17 = (-26057045781813601582315018815619209755_i128);
place!(Field::<(*const f32, (char, u64, *const isize), [u64; 1], i32)>(Variant(_20, 0), 2)).2 = [(*_3)];
_2 = _8;
_23 = true;
_17 = 155171388161523467465562908793795912017_i128 << _5;
_19.fld1 = 8475540263311912097_i64;
RET.0 = Field::<Adt41>(Variant(_20, 0), 0).fld0.0;
_28 = [Field::<u8>(Variant(_20, 0), 5),_6.1,Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5)];
_1 = (*_8);
_4 = -_7;
_33 = Adt41 { fld0: _16,fld1: Field::<Adt41>(Variant(_20, 0), 0).fld1,fld2: Field::<Adt41>(Variant(_20, 0), 0).fld2 };
_28 = [Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),Field::<u8>(Variant(_20, 0), 5),_6.1,Field::<u8>(Variant(_20, 0), 5),_9.1];
Goto(bb9)
}
bb17 = {
(*_2) = _11 << _43.0;
_33.fld2 = _36 << _5;
_43 = _6;
_43 = ((*_8), _9.1);
_42.fld1 = [10313725430902482434_u64,15257889312049820118_u64,4240024489856372727_u64,8752428776203935201_u64,8117246659703589454_u64];
Goto(bb18)
}
bb18 = {
Call(_54 = dump_var(14_usize, 25_usize, Move(_25), 23_usize, Move(_23), 49_usize, Move(_49), 36_usize, Move(_36)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_54 = dump_var(14_usize, 27_usize, Move(_27), 12_usize, Move(_12), 7_usize, Move(_7), 39_usize, Move(_39)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_54 = dump_var(14_usize, 15_usize, Move(_15), 14_usize, Move(_14), 17_usize, Move(_17), 13_usize, Move(_13)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_54 = dump_var(14_usize, 4_usize, Move(_4), 18_usize, Move(_18), 34_usize, Move(_34), 55_usize, _55), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *const isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> [i32; 4] {
mir! {
type RET = [i32; 4];
let _11: char;
let _12: [u64; 5];
let _13: Adt48;
let _14: i8;
let _15: [u8; 6];
let _16: [i32; 4];
let _17: Adt50;
let _18: [i16; 8];
let _19: i64;
let _20: isize;
let _21: f32;
let _22: [i32; 4];
let _23: [u8; 6];
let _24: [i32; 4];
let _25: i8;
let _26: [i32; 4];
let _27: &'static f32;
let _28: Adt49;
let _29: ();
let _30: ();
{
(*_1) = !_5;
_2 = _3;
RET = [889372685_i32,860745109_i32,982139024_i32,(-981066746_i32)];
RET = [2130554308_i32,(-635222284_i32),(-675513323_i32),1394103796_i32];
_6 = (*_1) ^ (*_1);
_11 = '\u{6c2ed}';
RET = [539766967_i32,(-1261997473_i32),1754102992_i32,2017986975_i32];
_10 = (*_1);
_6 = (*_1);
_4 = (-1409306769_i32) as isize;
_5 = (*_1);
_9 = -_3;
_12 = [9144761788731862961_u64,7188871118304944453_u64,18276951433522375465_u64,9560519852813481130_u64,8057020715265918493_u64];
_8 = _9;
_5 = _8;
_9 = !_2;
_10 = 5264049166164867474_u64 as isize;
_11 = '\u{c22a5}';
_6 = -_2;
_11 = '\u{1421}';
_3 = _2 - _5;
(*_1) = _5 * _3;
_12 = [12657594410774380691_u64,8953398435464854905_u64,12591172706096514107_u64,5820896710351235504_u64,4971125849326579559_u64];
(*_1) = -_9;
_6 = !_5;
_12 = [10046367322701470658_u64,18300392964022074369_u64,6415206068879219642_u64,14983132570981064504_u64,10108498215631874649_u64];
Call(_5 = fn16((*_1), _7, _6, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = '\u{7d774}';
_14 = -2_i8;
_12 = [4953459181062342444_u64,4640264165839927548_u64,17677955571070091310_u64,1110974838684800927_u64,255571657456051532_u64];
_5 = _2;
_1 = core::ptr::addr_of!(_10);
_9 = _2;
_7 = _5 << _5;
_11 = '\u{ec5ef}';
_5 = _8;
_8 = _5 ^ _7;
RET = [303764316_i32,1107251085_i32,(-1789933136_i32),(-160967754_i32)];
_8 = _2;
(*_1) = _5;
_3 = -(*_1);
(*_1) = _2;
_8 = -(*_1);
(*_1) = _8;
_3 = (-21889_i16) as isize;
_8 = -(*_1);
_14 = (-20457_i16) as i8;
RET = [1423289268_i32,(-874449921_i32),(-201468274_i32),(-2066624993_i32)];
_1 = core::ptr::addr_of!(_2);
_7 = _10;
(*_1) = -_5;
_8 = 3542727961773515217040915901733005348_u128 as isize;
_4 = 226521954270238854086207579769092953476_u128 as isize;
RET = [272840432_i32,1739883013_i32,1157714212_i32,1426361863_i32];
_7 = 85_u8 as isize;
Goto(bb2)
}
bb2 = {
_11 = '\u{ceb67}';
_10 = _6;
_5 = _9 & _2;
_16 = [1552993319_i32,(-1754294911_i32),(-171326752_i32),(-1593608792_i32)];
_10 = -(*_1);
_15 = [89_u8,11_u8,120_u8,102_u8,35_u8,189_u8];
_18 = [15411_i16,(-6129_i16),2340_i16,(-17860_i16),16220_i16,(-32764_i16),30553_i16,(-17466_i16)];
_11 = '\u{984b7}';
_17 = Adt50::Variant3 { fld0: _16 };
Goto(bb3)
}
bb3 = {
RET = [171060568_i32,(-1841254318_i32),(-1300906004_i32),709011192_i32];
_9 = 2868_i16 as isize;
_7 = !_2;
_6 = _2;
_6 = _14 as isize;
_14 = !115_i8;
_14 = 29_i8;
_15 = [12_u8,138_u8,179_u8,70_u8,104_u8,70_u8];
_23 = [103_u8,125_u8,96_u8,241_u8,72_u8,22_u8];
_2 = _10 * _5;
place!(Field::<[i32; 4]>(Variant(_17, 3), 0)) = _16;
_4 = (-136767969109922928562929634395259288807_i128) as isize;
(*_1) = _7;
(*_1) = !_5;
_16 = RET;
_3 = _10;
match _14 {
0 => bb4,
1 => bb5,
2 => bb6,
29 => bb8,
_ => bb7
}
}
bb4 = {
_11 = '\u{ceb67}';
_10 = _6;
_5 = _9 & _2;
_16 = [1552993319_i32,(-1754294911_i32),(-171326752_i32),(-1593608792_i32)];
_10 = -(*_1);
_15 = [89_u8,11_u8,120_u8,102_u8,35_u8,189_u8];
_18 = [15411_i16,(-6129_i16),2340_i16,(-17860_i16),16220_i16,(-32764_i16),30553_i16,(-17466_i16)];
_11 = '\u{984b7}';
_17 = Adt50::Variant3 { fld0: _16 };
Goto(bb3)
}
bb5 = {
_11 = '\u{7d774}';
_14 = -2_i8;
_12 = [4953459181062342444_u64,4640264165839927548_u64,17677955571070091310_u64,1110974838684800927_u64,255571657456051532_u64];
_5 = _2;
_1 = core::ptr::addr_of!(_10);
_9 = _2;
_7 = _5 << _5;
_11 = '\u{ec5ef}';
_5 = _8;
_8 = _5 ^ _7;
RET = [303764316_i32,1107251085_i32,(-1789933136_i32),(-160967754_i32)];
_8 = _2;
(*_1) = _5;
_3 = -(*_1);
(*_1) = _2;
_8 = -(*_1);
(*_1) = _8;
_3 = (-21889_i16) as isize;
_8 = -(*_1);
_14 = (-20457_i16) as i8;
RET = [1423289268_i32,(-874449921_i32),(-201468274_i32),(-2066624993_i32)];
_1 = core::ptr::addr_of!(_2);
_7 = _10;
(*_1) = -_5;
_8 = 3542727961773515217040915901733005348_u128 as isize;
_4 = 226521954270238854086207579769092953476_u128 as isize;
RET = [272840432_i32,1739883013_i32,1157714212_i32,1426361863_i32];
_7 = 85_u8 as isize;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_24 = [907292563_i32,220243983_i32,(-160615899_i32),482292660_i32];
_22 = [(-809383758_i32),(-1702830475_i32),(-122044048_i32),(-1622465630_i32)];
_10 = (*_1) << (*_1);
_19 = 3049679716010973883_i64 >> _2;
place!(Field::<[i32; 4]>(Variant(_17, 3), 0)) = [(-2104792293_i32),(-1401393712_i32),(-1912493876_i32),1232622546_i32];
_11 = '\u{7acc9}';
_20 = 15655_u16 as isize;
SetDiscriminant(_17, 0);
RET = _22;
match _14 {
0 => bb9,
1 => bb10,
2 => bb11,
29 => bb13,
_ => bb12
}
}
bb9 = {
_11 = '\u{7d774}';
_14 = -2_i8;
_12 = [4953459181062342444_u64,4640264165839927548_u64,17677955571070091310_u64,1110974838684800927_u64,255571657456051532_u64];
_5 = _2;
_1 = core::ptr::addr_of!(_10);
_9 = _2;
_7 = _5 << _5;
_11 = '\u{ec5ef}';
_5 = _8;
_8 = _5 ^ _7;
RET = [303764316_i32,1107251085_i32,(-1789933136_i32),(-160967754_i32)];
_8 = _2;
(*_1) = _5;
_3 = -(*_1);
(*_1) = _2;
_8 = -(*_1);
(*_1) = _8;
_3 = (-21889_i16) as isize;
_8 = -(*_1);
_14 = (-20457_i16) as i8;
RET = [1423289268_i32,(-874449921_i32),(-201468274_i32),(-2066624993_i32)];
_1 = core::ptr::addr_of!(_2);
_7 = _10;
(*_1) = -_5;
_8 = 3542727961773515217040915901733005348_u128 as isize;
_4 = 226521954270238854086207579769092953476_u128 as isize;
RET = [272840432_i32,1739883013_i32,1157714212_i32,1426361863_i32];
_7 = 85_u8 as isize;
Goto(bb2)
}
bb10 = {
RET = [171060568_i32,(-1841254318_i32),(-1300906004_i32),709011192_i32];
_9 = 2868_i16 as isize;
_7 = !_2;
_6 = _2;
_6 = _14 as isize;
_14 = !115_i8;
_14 = 29_i8;
_15 = [12_u8,138_u8,179_u8,70_u8,104_u8,70_u8];
_23 = [103_u8,125_u8,96_u8,241_u8,72_u8,22_u8];
_2 = _10 * _5;
place!(Field::<[i32; 4]>(Variant(_17, 3), 0)) = _16;
_4 = (-136767969109922928562929634395259288807_i128) as isize;
(*_1) = _7;
(*_1) = !_5;
_16 = RET;
_3 = _10;
match _14 {
0 => bb4,
1 => bb5,
2 => bb6,
29 => bb8,
_ => bb7
}
}
bb11 = {
_11 = '\u{7d774}';
_14 = -2_i8;
_12 = [4953459181062342444_u64,4640264165839927548_u64,17677955571070091310_u64,1110974838684800927_u64,255571657456051532_u64];
_5 = _2;
_1 = core::ptr::addr_of!(_10);
_9 = _2;
_7 = _5 << _5;
_11 = '\u{ec5ef}';
_5 = _8;
_8 = _5 ^ _7;
RET = [303764316_i32,1107251085_i32,(-1789933136_i32),(-160967754_i32)];
_8 = _2;
(*_1) = _5;
_3 = -(*_1);
(*_1) = _2;
_8 = -(*_1);
(*_1) = _8;
_3 = (-21889_i16) as isize;
_8 = -(*_1);
_14 = (-20457_i16) as i8;
RET = [1423289268_i32,(-874449921_i32),(-201468274_i32),(-2066624993_i32)];
_1 = core::ptr::addr_of!(_2);
_7 = _10;
(*_1) = -_5;
_8 = 3542727961773515217040915901733005348_u128 as isize;
_4 = 226521954270238854086207579769092953476_u128 as isize;
RET = [272840432_i32,1739883013_i32,1157714212_i32,1426361863_i32];
_7 = 85_u8 as isize;
Goto(bb2)
}
bb12 = {
_11 = '\u{ceb67}';
_10 = _6;
_5 = _9 & _2;
_16 = [1552993319_i32,(-1754294911_i32),(-171326752_i32),(-1593608792_i32)];
_10 = -(*_1);
_15 = [89_u8,11_u8,120_u8,102_u8,35_u8,189_u8];
_18 = [15411_i16,(-6129_i16),2340_i16,(-17860_i16),16220_i16,(-32764_i16),30553_i16,(-17466_i16)];
_11 = '\u{984b7}';
_17 = Adt50::Variant3 { fld0: _16 };
Goto(bb3)
}
bb13 = {
_21 = (-1917762025_i32) as f32;
_27 = &_21;
_15 = [47_u8,77_u8,60_u8,150_u8,130_u8,126_u8];
_20 = _3;
Goto(bb14)
}
bb14 = {
_15 = _23;
_1 = core::ptr::addr_of!((*_1));
_3 = _20;
_26 = [1804904367_i32,(-1485658517_i32),(-1014832730_i32),23144049_i32];
_25 = _14 - _14;
_24 = _26;
_20 = (*_1);
place!(Field::<(char,)>(Variant(_17, 0), 1)) = (_11,);
_23 = _15;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(15_usize, 4_usize, Move(_4), 9_usize, Move(_9), 11_usize, Move(_11), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(15_usize, 15_usize, Move(_15), 10_usize, Move(_10), 23_usize, Move(_23), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(15_usize, 6_usize, Move(_6), 7_usize, Move(_7), 5_usize, Move(_5), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: [u8; 6];
let _6: ();
let _7: ();
{
_1 = _3;
_1 = _4;
RET = _4;
RET = _2;
RET = -_1;
_4 = 11811889978251577088_u64 as isize;
_5 = [77_u8,180_u8,26_u8,18_u8,51_u8,96_u8];
_2 = RET;
_5 = [187_u8,109_u8,48_u8,227_u8,44_u8,253_u8];
_5 = [180_u8,88_u8,142_u8,192_u8,164_u8,178_u8];
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(16_usize, 4_usize, Move(_4), 2_usize, Move(_2), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: bool,mut _8: bool,mut _9: isize,mut _10: bool,mut _11: *mut [u8; 6]) -> bool {
mir! {
type RET = bool;
let _12: ();
let _13: ();
{
(*_11) = [179_u8,254_u8,15_u8,56_u8,214_u8,65_u8];
_10 = _7 | _8;
(*_11) = [124_u8,10_u8,102_u8,148_u8,114_u8,232_u8];
RET = _7 | _10;
_11 = core::ptr::addr_of_mut!((*_11));
_5 = _2 + _9;
_6 = 19684519144914734520740401219813568882_u128 as isize;
_9 = _5 * _5;
_11 = core::ptr::addr_of_mut!((*_11));
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(17_usize, 10_usize, Move(_10), 2_usize, Move(_2), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_12 = dump_var(17_usize, 8_usize, Move(_8), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: isize,mut _3: (isize, bool, isize),mut _4: bool,mut _5: *const isize,mut _6: *const isize,mut _7: *const isize) -> f32 {
mir! {
type RET = f32;
let _8: isize;
let _9: char;
let _10: bool;
let _11: isize;
let _12: *const isize;
let _13: i32;
let _14: isize;
let _15: *const f32;
let _16: char;
let _17: isize;
let _18: char;
let _19: [u64; 1];
let _20: Adt46;
let _21: bool;
let _22: [i32; 4];
let _23: bool;
let _24: usize;
let _25: ();
let _26: ();
{
(*_5) = _2 >> _1;
RET = 2_usize as f32;
_3.0 = 4242022393446718693_i64 as isize;
RET = 6_usize as f32;
Call((*_7) = core::intrinsics::bswap(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.2 = (*_5) << (*_6);
_3 = ((*_6), _4, (*_5));
_6 = core::ptr::addr_of!((*_6));
_3.1 = (*_6) < (*_7);
_3.2 = (*_7) | _1;
_8 = _3.2 + (*_7);
_3.0 = -(*_5);
RET = 17276291806610276455_usize as f32;
_5 = _6;
Goto(bb2)
}
bb2 = {
RET = 976978555_u32 as f32;
Goto(bb3)
}
bb3 = {
(*_7) = -_8;
_5 = core::ptr::addr_of!(_3.0);
(*_6) = !_8;
(*_6) = _3.0 | (*_5);
(*_7) = !_1;
Goto(bb4)
}
bb4 = {
(*_7) = _2;
(*_7) = _1;
_3.1 = !_4;
_3.0 = _3.2;
(*_5) = (*_6);
_3.1 = _4;
_6 = core::ptr::addr_of!(_3.2);
_3 = (_2, _4, _8);
(*_6) = _3.1 as isize;
_6 = core::ptr::addr_of!((*_7));
_3.1 = !_4;
_4 = !_3.1;
_4 = (*_5) <= _3.2;
RET = 174975251757927766583940841649872672140_u128 as f32;
_3.2 = (-1954023050360843017_i64) as isize;
(*_5) = RET as isize;
Goto(bb5)
}
bb5 = {
(*_7) = (*_5) + _8;
Goto(bb6)
}
bb6 = {
(*_5) = -(*_7);
_10 = !_4;
Goto(bb7)
}
bb7 = {
_3 = ((*_6), _10, (*_6));
(*_5) = -(*_7);
_5 = core::ptr::addr_of!(_3.2);
_9 = '\u{20dc5}';
_10 = _3.1;
(*_7) = (*_5);
(*_7) = 215_u8 as isize;
_1 = !(*_5);
_3.0 = _1 ^ (*_5);
RET = 224_u8 as f32;
_8 = _3.0;
_3.2 = _3.0;
Goto(bb8)
}
bb8 = {
_7 = _5;
_10 = !_3.1;
_6 = core::ptr::addr_of!(_11);
_14 = !_3.2;
_14 = -(*_5);
(*_6) = (*_5);
Goto(bb9)
}
bb9 = {
RET = 5198_u16 as f32;
RET = 15850_i16 as f32;
_18 = _9;
(*_6) = !(*_7);
_4 = _3.1 ^ _10;
(*_6) = 5358659064954047245_u64 as isize;
_10 = _4;
(*_6) = 33627285_i32 as isize;
(*_6) = -_3.2;
_3.2 = _11 * (*_6);
Goto(bb10)
}
bb10 = {
(*_5) = (*_6) + _11;
_6 = core::ptr::addr_of!(_8);
_8 = -_11;
_16 = _9;
_14 = _3.2 << (*_6);
RET = 63_u8 as f32;
_8 = !_3.2;
_3.0 = !_8;
_5 = _6;
_14 = 3655812881119198996_usize as isize;
_3 = (_8, _4, (*_6));
_4 = _10;
_1 = (*_7);
_12 = _5;
_17 = (*_6) + _3.2;
_22 = [(-204462213_i32),1797518126_i32,135128049_i32,1324392591_i32];
_3.0 = 15871_u16 as isize;
_14 = (*_6) >> (*_6);
_6 = core::ptr::addr_of!(_8);
(*_7) = _14 | _17;
RET = (*_7) as f32;
_24 = 1_usize << _11;
_20.fld1 = 5228632489098630356_i64 + (-9187617611313492767_i64);
RET = _24 as f32;
_1 = _11;
Goto(bb11)
}
bb11 = {
Call(_25 = dump_var(18_usize, 14_usize, Move(_14), 1_usize, Move(_1), 9_usize, Move(_9), 16_usize, Move(_16)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_25 = dump_var(18_usize, 3_usize, Move(_3), 4_usize, Move(_4), 2_usize, Move(_2), 26_usize, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(1492293354_u32), std::hint::black_box(0_usize), std::hint::black_box(47468_u16));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: (char,),
fld1: (*const u64, i32, i32),
fld2: u128,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u32,
fld1: (*const u64, i32, i32),
fld2: [u16; 6],
fld3: *mut [u8; 6],

},
Variant1{
fld0: u64,
fld1: *const u16,
fld2: i128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: Adt41,
fld1: *const u16,
fld2: (*const f32, (char, u64, *const isize), [u64; 1], i32),
fld3: u64,
fld4: i16,
fld5: u8,

},
Variant1{
fld0: u32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: (isize, u8),
fld1: (isize, bool, isize),
fld2: *const isize,
fld3: (*const u64, i32, i32),
fld4: u16,
fld5: [i8; 4],

},
Variant1{
fld0: Adt42,

},
Variant2{
fld0: bool,
fld1: (isize, u8),
fld2: *mut [i32; 4],
fld3: *const isize,
fld4: f64,
fld5: i32,
fld6: ((*const f32, (char, u64, *const isize), [u64; 1], i32), *const u64, isize),
fld7: [u64; 5],

},
Variant3{
fld0: [u64; 5],
fld1: i128,
fld2: u8,
fld3: [u64; 1],
fld4: (char, u64, *const isize),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: ((*const f32, (char, u64, *const isize), [u64; 1], i32), *const u64, isize),
fld1: Adt42,
fld2: *const u64,

},
Variant1{
fld0: bool,
fld1: (isize, bool, isize),
fld2: *const isize,
fld3: u32,
fld4: (char,),
fld5: Adt41,
fld6: f32,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: Adt42,
fld1: i64,
fld2: *mut [u8; 6],
fld3: *const f32,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: *mut [i32; 4],
fld1: i64,
fld2: u128,
fld3: [i8; 4],
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt41,

},
Variant1{
fld0: *const u16,
fld1: (isize, u8),
fld2: isize,
fld3: (char,),
fld4: u32,
fld5: [u16; 6],
fld6: usize,
fld7: u128,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt44,
fld1: u64,
fld2: (isize, u8),
fld3: Adt46,

},
Variant1{
fld0: (*const f32, (char, u64, *const isize), [u64; 1], i32),
fld1: [u64; 5],
fld2: (char, u64, *const isize),
fld3: *mut [i32; 4],
fld4: f64,
fld5: *const u16,
fld6: i64,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u64,
fld1: (char,),

},
Variant1{
fld0: usize,

},
Variant2{
fld0: Adt49,
fld1: (isize, u8),
fld2: u64,
fld3: Adt41,
fld4: (isize, bool, isize),
fld5: [u64; 5],

},
Variant3{
fld0: [i32; 4],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (isize, bool, isize),
fld1: i128,
fld2: *const u64,
fld3: Adt50,
fld4: [i16; 8],
fld5: [u64; 1],
fld6: Adt49,

},
Variant1{
fld0: (*const f32, (char, u64, *const isize), [u64; 1], i32),
fld1: [u64; 5],
fld2: Adt49,
fld3: u8,
fld4: (*const u64, i32, i32),
fld5: Adt46,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: [u16; 6],
fld1: char,
fld2: (char, u64, *const isize),
fld3: (char,),
fld4: Adt42,
fld5: Adt47,
fld6: [i32; 4],
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
fld0: bool,
fld1: u128,
fld2: Adt41,

},
Variant1{
fld0: ((*const f32, (char, u64, *const isize), [u64; 1], i32), *const u64, isize),
fld1: u32,

},
Variant2{
fld0: bool,
fld1: [u16; 6],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt44,

},
Variant1{
fld0: u8,

},
Variant2{
fld0: (isize, bool, isize),
fld1: char,
fld2: [u64; 5],
fld3: *const isize,
fld4: *mut [u8; 6],
fld5: *const u16,
fld6: [i32; 4],

},
Variant3{
fld0: bool,
fld1: Adt42,
fld2: Adt47,
fld3: Adt50,
fld4: i32,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [u8; 6],
fld1: [u64; 5],
fld2: *const u16,
fld3: *mut [u8; 6],
fld4: [i8; 4],
fld5: f64,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (*const f32, (char, u64, *const isize), [u64; 1], i32),
fld1: f32,
fld2: *mut [u8; 6],
fld3: i8,
fld4: Adt52,
fld5: *const u64,
fld6: *const f32,
fld7: Adt50,

},
Variant1{
fld0: Adt53,
fld1: Adt42,
fld2: Adt55,
fld3: Adt54,
fld4: *const f32,
fld5: Adt46,
fld6: i64,
fld7: u128,

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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: isize,
fld1: Adt54,

},
Variant1{
fld0: (*const u64, i32, i32),
fld1: [u64; 1],
fld2: Adt42,
fld3: (isize, u8),

}}

