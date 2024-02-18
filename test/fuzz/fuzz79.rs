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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u128) -> isize {
mir! {
type RET = isize;
let _13: bool;
let _14: Adt70;
let _15: *const char;
let _16: &'static *const i32;
let _17: *mut u32;
let _18: &'static ([i128; 2], (Adt24, u16));
let _19: [i128; 5];
let _20: (*mut u32, (((i32, i128), *const i32), i16, *mut u32, (i32, i128)));
let _21: isize;
let _22: i32;
let _23: u16;
let _24: &'static &'static [i64; 1];
let _25: u64;
let _26: [i128; 2];
let _27: u8;
let _28: (u8, [u128; 7]);
let _29: ();
let _30: ();
{
_2 = '\u{26e83}';
RET = (-27_isize);
_5 = !(-21768_i16);
_12 = !131740906789550888443105138833712995875_u128;
match RET {
0 => bb1,
340282366920938463463374607431768211429 => bb3,
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
_11 = !12504_u16;
_12 = _2 as u128;
RET = 9223372036854775807_isize;
_12 = 127103316052737610536044069558774892051_u128 << _5;
RET = 9223372036854775807_isize * 48_isize;
_9 = _5 as usize;
_2 = '\u{d1c4}';
RET = !(-9223372036854775808_isize);
RET = (-9223372036854775808_isize);
_8 = (-34406281391460300071448277880381554946_i128) + (-49872550592130224901654286697846573941_i128);
_6 = (-1047758498_i32) ^ 1763561897_i32;
_15 = core::ptr::addr_of!(_2);
_9 = !4_usize;
RET = -9223372036854775807_isize;
_9 = 10732652265718768635_usize + 1_usize;
_2 = '\u{b9ceb}';
(*_15) = '\u{10ebfd}';
(*_15) = '\u{aecf6}';
_3 = 3361598002_u32 as isize;
_7 = !5857555718506139781_i64;
RET = !_3;
_13 = false;
_12 = 219085858827571685254146076004001669146_u128 ^ 53141209106676805330304855190948809068_u128;
_10 = 70_u8;
_11 = !51764_u16;
_1 = _13;
Goto(bb4)
}
bb4 = {
_3 = RET << _11;
_6 = -1205864389_i32;
_4 = -98_i8;
_6 = -(-1974618540_i32);
_20.1.3.0 = _3 as i32;
_20.1.3.0 = _2 as i32;
_20.1.3.0 = -_6;
_3 = !RET;
_12 = 118732043065721702155141026483221965710_u128 + 194102110030788238478530134563230265793_u128;
_20.1.1 = !_5;
_20.1.3.1 = _8 ^ _8;
_8 = _20.1.3.1;
_20.1.0.0.0 = -_6;
_20.1.3.1 = !_8;
_20.1.3.0 = _6;
_20.1.0.1 = core::ptr::addr_of!(_20.1.3.0);
Call(_20.1 = fn1(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = !3778656527125725180_usize;
_13 = _1 == _1;
_20.1.3 = (_20.1.0.0.0, _8);
_16 = &_20.1.0.1;
_22 = _20.1.3.0 + _6;
RET = _5 as isize;
Goto(bb6)
}
bb6 = {
_4 = _12 as i8;
_20.1.0.0 = (_20.1.3.0, _8);
_11 = 4016_u16;
(*_15) = '\u{fec44}';
match _10 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
70 => bb13,
_ => bb12
}
}
bb7 = {
_9 = !3778656527125725180_usize;
_13 = _1 == _1;
_20.1.3 = (_20.1.0.0.0, _8);
_16 = &_20.1.0.1;
_22 = _20.1.3.0 + _6;
RET = _5 as isize;
Goto(bb6)
}
bb8 = {
_3 = RET << _11;
_6 = -1205864389_i32;
_4 = -98_i8;
_6 = -(-1974618540_i32);
_20.1.3.0 = _3 as i32;
_20.1.3.0 = _2 as i32;
_20.1.3.0 = -_6;
_3 = !RET;
_12 = 118732043065721702155141026483221965710_u128 + 194102110030788238478530134563230265793_u128;
_20.1.1 = !_5;
_20.1.3.1 = _8 ^ _8;
_8 = _20.1.3.1;
_20.1.0.0.0 = -_6;
_20.1.3.1 = !_8;
_20.1.3.0 = _6;
_20.1.0.1 = core::ptr::addr_of!(_20.1.3.0);
Call(_20.1 = fn1(), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_11 = !12504_u16;
_12 = _2 as u128;
RET = 9223372036854775807_isize;
_12 = 127103316052737610536044069558774892051_u128 << _5;
RET = 9223372036854775807_isize * 48_isize;
_9 = _5 as usize;
_2 = '\u{d1c4}';
RET = !(-9223372036854775808_isize);
RET = (-9223372036854775808_isize);
_8 = (-34406281391460300071448277880381554946_i128) + (-49872550592130224901654286697846573941_i128);
_6 = (-1047758498_i32) ^ 1763561897_i32;
_15 = core::ptr::addr_of!(_2);
_9 = !4_usize;
RET = -9223372036854775807_isize;
_9 = 10732652265718768635_usize + 1_usize;
_2 = '\u{b9ceb}';
(*_15) = '\u{10ebfd}';
(*_15) = '\u{aecf6}';
_3 = 3361598002_u32 as isize;
_7 = !5857555718506139781_i64;
RET = !_3;
_13 = false;
_12 = 219085858827571685254146076004001669146_u128 ^ 53141209106676805330304855190948809068_u128;
_10 = 70_u8;
_11 = !51764_u16;
_1 = _13;
Goto(bb4)
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
_19 = [_8,_8,_8,_8,_20.1.0.0.1];
_25 = 13652215839875329954_u64 + 7582405528107930216_u64;
_20.1.0.0.1 = _8 | _8;
_20.1.3.1 = (*_15) as i128;
_1 = _13;
_20.1.1 = _5;
_20.1.3 = (_20.1.0.0.0, _8);
_12 = _11 as u128;
_26 = [_8,_20.1.0.0.1];
_16 = &(*_16);
RET = -_3;
_6 = _12 as i32;
RET = _25 as isize;
_20.1.0.1 = core::ptr::addr_of!(_6);
_16 = &_20.1.0.1;
_21 = _3;
match _10 {
0 => bb7,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb14,
70 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_9 = !3778656527125725180_usize;
_13 = _1 == _1;
_20.1.3 = (_20.1.0.0.0, _8);
_16 = &_20.1.0.1;
_22 = _20.1.3.0 + _6;
RET = _5 as isize;
Goto(bb6)
}
bb16 = {
_20.1.0.1 = core::ptr::addr_of!(_6);
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(0_usize, 13_usize, Move(_13), 7_usize, Move(_7), 22_usize, Move(_22), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(0_usize, 21_usize, Move(_21), 8_usize, Move(_8), 25_usize, Move(_25), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(0_usize, 5_usize, Move(_5), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1() -> (((i32, i128), *const i32), i16, *mut u32, (i32, i128)) {
mir! {
type RET = (((i32, i128), *const i32), i16, *mut u32, (i32, i128));
let _1: Adt82;
let _2: isize;
let _3: u16;
let _4: &'static [i64; 1];
let _5: f32;
let _6: Adt24;
let _7: f32;
let _8: *mut *mut u32;
let _9: ([i128; 2], &'static isize, [i128; 7], (((u8, [u128; 7]),), [i128; 5], &'static &'static f32));
let _10: f64;
let _11: u32;
let _12: i64;
let _13: usize;
let _14: *mut u32;
let _15: [u128; 7];
let _16: isize;
let _17: [i16; 7];
let _18: u32;
let _19: u64;
let _20: ();
let _21: ();
{
RET.0.0.1 = !14482480720916774243691242196007880220_i128;
RET.0.0.0 = !1113412627_i32;
RET.0.1 = core::ptr::addr_of!(RET.0.0.0);
RET.0.0 = ((-1793453160_i32), (-170044729882469532337775500326837736143_i128));
RET.3.0 = -RET.0.0.0;
RET.0.0.1 = 128460110110687341074059830571635744859_i128;
RET.0.0.0 = -RET.3.0;
RET.0.1 = core::ptr::addr_of!(RET.3.0);
RET.3.1 = RET.0.0.1 >> RET.0.0.0;
RET.1 = RET.3.0 as i16;
RET.0.0.0 = 2083462881_u32 as i32;
RET.0.0.0 = 37_u8 as i32;
_2 = (-115_isize);
RET.0.0 = RET.3;
_3 = 16098_u16 & 29623_u16;
RET.0.0.1 = RET.3.1;
RET.3.0 = _3 as i32;
RET.3.0 = 800927278_u32 as i32;
RET.0.0 = RET.3;
Goto(bb1)
}
bb1 = {
RET.3.0 = -RET.0.0.0;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0 = RET.3;
RET.0.0 = RET.3;
RET.3.1 = !RET.0.0.1;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0.1 = _2 as i128;
RET.0.0.1 = false as i128;
RET.3 = RET.0.0;
RET.1 = (-4507_i16) - (-27255_i16);
_8 = core::ptr::addr_of_mut!(RET.2);
RET.0.0 = (RET.3.0, RET.3.1);
_3 = (-7301163697860532468_i64) as u16;
_7 = 4234898397_u32 as f32;
_9.3.0.0.1 = [149234194408485491922659838622576215101_u128,135980291303909670770014945409490141460_u128,98019403620114810836603559761414781922_u128,12147367707688601251769365108726015933_u128,139603794558107088975080602912134174282_u128,135422945060316974789900637288967173968_u128,95307854971045746187908719742153759364_u128];
_9.3.1 = [RET.0.0.1,RET.3.1,RET.0.0.1,RET.3.1,RET.0.0.1];
RET.0.0.0 = !RET.3.0;
RET.0.0.0 = 14226478938702278276_u64 as i32;
_9.3.0.0.0 = _7 as u8;
_9.3.0.0.0 = !122_u8;
_9.2 = [RET.3.1,RET.0.0.1,RET.0.0.1,RET.3.1,RET.0.0.1,RET.0.0.1,RET.0.0.1];
RET.0.0.0 = RET.3.0 - RET.3.0;
_6 = Adt24::Variant1 { fld0: RET.1 };
RET.0.0 = (RET.3.0, RET.3.1);
place!(Field::<i16>(Variant(_6, 1), 0)) = RET.1 & RET.1;
_9.3.0.0.1 = [250908494352378337712761557347188252297_u128,293536433860575306740752199449062385143_u128,220418882327104353639996329081382884148_u128,126263953117974813762143627232509272025_u128,273122541434596872481429252130208186100_u128,84260091854793933949057658858722272268_u128,188382343504777593226644196274705801849_u128];
Call(place!(Field::<i16>(Variant(_6, 1), 0)) = fn2(_9.3.0.0.1, _9.3.0.0.1, _9.3.0, Move(RET.0), _9.3.0, _9.3.0, _9.3.0.0.1, RET.3.0, _9.3.0, _7, _9.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0.0.0 = RET.3.0;
RET.0.0.0 = -RET.3.0;
_10 = _3 as f64;
RET.0.1 = core::ptr::addr_of!(RET.3.0);
RET.0.1 = core::ptr::addr_of!(RET.0.0.0);
(*_8) = core::ptr::addr_of_mut!(_11);
RET.3 = (RET.0.0.0, 28626125862664115507251576630560109734_i128);
RET.0.0.1 = RET.3.1;
Goto(bb3)
}
bb3 = {
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.3 = RET.0.0;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0.0 = -RET.3.0;
_11 = 3271774429_u32;
_9.0 = [RET.3.1,RET.0.0.1];
RET.3.1 = _7 as i128;
RET.0.0 = (RET.3.0, RET.3.1);
_2 = 5_usize as isize;
match _11 {
0 => bb4,
3271774429 => bb6,
_ => bb5
}
}
bb4 = {
RET.0.0.0 = RET.3.0;
RET.0.0.0 = -RET.3.0;
_10 = _3 as f64;
RET.0.1 = core::ptr::addr_of!(RET.3.0);
RET.0.1 = core::ptr::addr_of!(RET.0.0.0);
(*_8) = core::ptr::addr_of_mut!(_11);
RET.3 = (RET.0.0.0, 28626125862664115507251576630560109734_i128);
RET.0.0.1 = RET.3.1;
Goto(bb3)
}
bb5 = {
RET.3.0 = -RET.0.0.0;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0 = RET.3;
RET.0.0 = RET.3;
RET.3.1 = !RET.0.0.1;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0.1 = _2 as i128;
RET.0.0.1 = false as i128;
RET.3 = RET.0.0;
RET.1 = (-4507_i16) - (-27255_i16);
_8 = core::ptr::addr_of_mut!(RET.2);
RET.0.0 = (RET.3.0, RET.3.1);
_3 = (-7301163697860532468_i64) as u16;
_7 = 4234898397_u32 as f32;
_9.3.0.0.1 = [149234194408485491922659838622576215101_u128,135980291303909670770014945409490141460_u128,98019403620114810836603559761414781922_u128,12147367707688601251769365108726015933_u128,139603794558107088975080602912134174282_u128,135422945060316974789900637288967173968_u128,95307854971045746187908719742153759364_u128];
_9.3.1 = [RET.0.0.1,RET.3.1,RET.0.0.1,RET.3.1,RET.0.0.1];
RET.0.0.0 = !RET.3.0;
RET.0.0.0 = 14226478938702278276_u64 as i32;
_9.3.0.0.0 = _7 as u8;
_9.3.0.0.0 = !122_u8;
_9.2 = [RET.3.1,RET.0.0.1,RET.0.0.1,RET.3.1,RET.0.0.1,RET.0.0.1,RET.0.0.1];
RET.0.0.0 = RET.3.0 - RET.3.0;
_6 = Adt24::Variant1 { fld0: RET.1 };
RET.0.0 = (RET.3.0, RET.3.1);
place!(Field::<i16>(Variant(_6, 1), 0)) = RET.1 & RET.1;
_9.3.0.0.1 = [250908494352378337712761557347188252297_u128,293536433860575306740752199449062385143_u128,220418882327104353639996329081382884148_u128,126263953117974813762143627232509272025_u128,273122541434596872481429252130208186100_u128,84260091854793933949057658858722272268_u128,188382343504777593226644196274705801849_u128];
Call(place!(Field::<i16>(Variant(_6, 1), 0)) = fn2(_9.3.0.0.1, _9.3.0.0.1, _9.3.0, Move(RET.0), _9.3.0, _9.3.0, _9.3.0.0.1, RET.3.0, _9.3.0, _7, _9.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_9.3.1 = [RET.0.0.1,RET.3.1,RET.0.0.1,RET.3.1,RET.0.0.1];
_9.3.0.0.1 = [258153632026796542482063354305795441915_u128,283827433887225547120137000412392245202_u128,93687136698016930369028120471492873945_u128,251801754200346186031984611892822571099_u128,229344855998355951775532998969995891940_u128,230825344867759473859183806094068670058_u128,150075999890818754383313321380076994073_u128];
RET.0.0.1 = RET.3.1;
RET.1 = !Field::<i16>(Variant(_6, 1), 0);
RET.0.0.1 = RET.3.1 * RET.3.1;
_2 = RET.1 as isize;
SetDiscriminant(_6, 0);
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld3 = true as i8;
RET.3.0 = -RET.0.0.0;
Call(RET.3.1 = core::intrinsics::transmute(RET.0.0.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET.2 = core::ptr::addr_of_mut!(_11);
_12 = 12800836352139616285940280044818024839_u128 as i64;
RET.0.0 = (RET.3.0, RET.3.1);
RET.2 = core::ptr::addr_of_mut!(_11);
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld2 = RET.1 as u32;
place!(Field::<isize>(Variant(_6, 0), 1)) = '\u{b68a2}' as isize;
RET.3.1 = '\u{b182c}' as i128;
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld0 = -RET.0.0.0;
_9.3.0.0.0 = 184_u8 | 233_u8;
_14 = core::ptr::addr_of_mut!(place!(Field::<Adt18>(Variant(_6, 0), 0)).fld2);
_5 = _7 * _7;
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld2 = false as u32;
_11 = 210706691239063240705396776977154071428_u128 as u32;
RET.0.1 = core::ptr::addr_of!(place!(Field::<Adt18>(Variant(_6, 0), 0)).fld0);
_9.1 = &_16;
RET.3 = RET.0.0;
_9.2 = [RET.0.0.1,RET.3.1,RET.3.1,RET.3.1,RET.3.1,RET.0.0.1,RET.0.0.1];
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld3 = -(-123_i8);
_9.1 = &_16;
_7 = -_5;
_14 = Move(RET.2);
_9.3.0.0.0 = 144_u8 >> RET.0.0.1;
Goto(bb8)
}
bb8 = {
_18 = !Field::<Adt18>(Variant(_6, 0), 0).fld2;
RET.0.0 = (RET.3.0, RET.3.1);
RET.0.1 = core::ptr::addr_of!(RET.0.0.0);
_9.3.0.0.1 = [114426571070842284118184060720148708193_u128,219991316202442825900947243787812179554_u128,156456166104446581678108732765782064117_u128,251476583417256671440359183011622206899_u128,139265720673468866334974809180502721483_u128,261545850670588649300475780039369066067_u128,104018916364512668576864585709268717805_u128];
_11 = Field::<Adt18>(Variant(_6, 0), 0).fld2 ^ _18;
RET.0.0.0 = _10 as i32;
RET.0.0.0 = Field::<Adt18>(Variant(_6, 0), 0).fld0;
_12 = !(-9105111420619402057_i64);
_9.3.0.0.1 = [36686407809855138321287869716990246435_u128,270970729575224266518825711761385042181_u128,258425338793054126365636003982638778269_u128,128759111630886673791825431130355724254_u128,121509920415340241755946692534136968514_u128,232231500064632089683172908086933242011_u128,111813891727627663965349073222602233202_u128];
RET.3 = (Field::<Adt18>(Variant(_6, 0), 0).fld0, RET.0.0.1);
_16 = _2;
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld1 = '\u{79269}';
_18 = !_11;
(*_8) = core::ptr::addr_of_mut!(place!(Field::<Adt18>(Variant(_6, 0), 0)).fld2);
Call(_15 = core::intrinsics::transmute(_9.3.0.0.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9.0 = [RET.0.0.1,RET.0.0.1];
place!(Field::<Adt18>(Variant(_6, 0), 0)) = Adt18 { fld0: RET.0.0.0,fld1: '\u{5fe10}',fld2: _18,fld3: (-109_i8) };
RET.3.0 = !Field::<Adt18>(Variant(_6, 0), 0).fld0;
match Field::<Adt18>(Variant(_6, 0), 0).fld3 {
0 => bb7,
1 => bb3,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463463374607431768211347 => bb14,
_ => bb13
}
}
bb10 = {
_18 = !Field::<Adt18>(Variant(_6, 0), 0).fld2;
RET.0.0 = (RET.3.0, RET.3.1);
RET.0.1 = core::ptr::addr_of!(RET.0.0.0);
_9.3.0.0.1 = [114426571070842284118184060720148708193_u128,219991316202442825900947243787812179554_u128,156456166104446581678108732765782064117_u128,251476583417256671440359183011622206899_u128,139265720673468866334974809180502721483_u128,261545850670588649300475780039369066067_u128,104018916364512668576864585709268717805_u128];
_11 = Field::<Adt18>(Variant(_6, 0), 0).fld2 ^ _18;
RET.0.0.0 = _10 as i32;
RET.0.0.0 = Field::<Adt18>(Variant(_6, 0), 0).fld0;
_12 = !(-9105111420619402057_i64);
_9.3.0.0.1 = [36686407809855138321287869716990246435_u128,270970729575224266518825711761385042181_u128,258425338793054126365636003982638778269_u128,128759111630886673791825431130355724254_u128,121509920415340241755946692534136968514_u128,232231500064632089683172908086933242011_u128,111813891727627663965349073222602233202_u128];
RET.3 = (Field::<Adt18>(Variant(_6, 0), 0).fld0, RET.0.0.1);
_16 = _2;
place!(Field::<Adt18>(Variant(_6, 0), 0)).fld1 = '\u{79269}';
_18 = !_11;
(*_8) = core::ptr::addr_of_mut!(place!(Field::<Adt18>(Variant(_6, 0), 0)).fld2);
Call(_15 = core::intrinsics::transmute(_9.3.0.0.1), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.3 = RET.0.0;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0.0 = -RET.3.0;
_11 = 3271774429_u32;
_9.0 = [RET.3.1,RET.0.0.1];
RET.3.1 = _7 as i128;
RET.0.0 = (RET.3.0, RET.3.1);
_2 = 5_usize as isize;
match _11 {
0 => bb4,
3271774429 => bb6,
_ => bb5
}
}
bb12 = {
RET.3.0 = -RET.0.0.0;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0 = RET.3;
RET.0.0 = RET.3;
RET.3.1 = !RET.0.0.1;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0.1 = _2 as i128;
RET.0.0.1 = false as i128;
RET.3 = RET.0.0;
RET.1 = (-4507_i16) - (-27255_i16);
_8 = core::ptr::addr_of_mut!(RET.2);
RET.0.0 = (RET.3.0, RET.3.1);
_3 = (-7301163697860532468_i64) as u16;
_7 = 4234898397_u32 as f32;
_9.3.0.0.1 = [149234194408485491922659838622576215101_u128,135980291303909670770014945409490141460_u128,98019403620114810836603559761414781922_u128,12147367707688601251769365108726015933_u128,139603794558107088975080602912134174282_u128,135422945060316974789900637288967173968_u128,95307854971045746187908719742153759364_u128];
_9.3.1 = [RET.0.0.1,RET.3.1,RET.0.0.1,RET.3.1,RET.0.0.1];
RET.0.0.0 = !RET.3.0;
RET.0.0.0 = 14226478938702278276_u64 as i32;
_9.3.0.0.0 = _7 as u8;
_9.3.0.0.0 = !122_u8;
_9.2 = [RET.3.1,RET.0.0.1,RET.0.0.1,RET.3.1,RET.0.0.1,RET.0.0.1,RET.0.0.1];
RET.0.0.0 = RET.3.0 - RET.3.0;
_6 = Adt24::Variant1 { fld0: RET.1 };
RET.0.0 = (RET.3.0, RET.3.1);
place!(Field::<i16>(Variant(_6, 1), 0)) = RET.1 & RET.1;
_9.3.0.0.1 = [250908494352378337712761557347188252297_u128,293536433860575306740752199449062385143_u128,220418882327104353639996329081382884148_u128,126263953117974813762143627232509272025_u128,273122541434596872481429252130208186100_u128,84260091854793933949057658858722272268_u128,188382343504777593226644196274705801849_u128];
Call(place!(Field::<i16>(Variant(_6, 1), 0)) = fn2(_9.3.0.0.1, _9.3.0.0.1, _9.3.0, Move(RET.0), _9.3.0, _9.3.0, _9.3.0.0.1, RET.3.0, _9.3.0, _7, _9.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
RET.3.0 = -RET.0.0.0;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0 = RET.3;
RET.0.0 = RET.3;
RET.3.1 = !RET.0.0.1;
RET.3 = (RET.0.0.0, RET.0.0.1);
RET.0.0.1 = _2 as i128;
RET.0.0.1 = false as i128;
RET.3 = RET.0.0;
RET.1 = (-4507_i16) - (-27255_i16);
_8 = core::ptr::addr_of_mut!(RET.2);
RET.0.0 = (RET.3.0, RET.3.1);
_3 = (-7301163697860532468_i64) as u16;
_7 = 4234898397_u32 as f32;
_9.3.0.0.1 = [149234194408485491922659838622576215101_u128,135980291303909670770014945409490141460_u128,98019403620114810836603559761414781922_u128,12147367707688601251769365108726015933_u128,139603794558107088975080602912134174282_u128,135422945060316974789900637288967173968_u128,95307854971045746187908719742153759364_u128];
_9.3.1 = [RET.0.0.1,RET.3.1,RET.0.0.1,RET.3.1,RET.0.0.1];
RET.0.0.0 = !RET.3.0;
RET.0.0.0 = 14226478938702278276_u64 as i32;
_9.3.0.0.0 = _7 as u8;
_9.3.0.0.0 = !122_u8;
_9.2 = [RET.3.1,RET.0.0.1,RET.0.0.1,RET.3.1,RET.0.0.1,RET.0.0.1,RET.0.0.1];
RET.0.0.0 = RET.3.0 - RET.3.0;
_6 = Adt24::Variant1 { fld0: RET.1 };
RET.0.0 = (RET.3.0, RET.3.1);
place!(Field::<i16>(Variant(_6, 1), 0)) = RET.1 & RET.1;
_9.3.0.0.1 = [250908494352378337712761557347188252297_u128,293536433860575306740752199449062385143_u128,220418882327104353639996329081382884148_u128,126263953117974813762143627232509272025_u128,273122541434596872481429252130208186100_u128,84260091854793933949057658858722272268_u128,188382343504777593226644196274705801849_u128];
Call(place!(Field::<i16>(Variant(_6, 1), 0)) = fn2(_9.3.0.0.1, _9.3.0.0.1, _9.3.0, Move(RET.0), _9.3.0, _9.3.0, _9.3.0.0.1, RET.3.0, _9.3.0, _7, _9.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
RET.0.1 = core::ptr::addr_of!(RET.0.0.0);
_19 = 9697224873157998075_u64 >> Field::<Adt18>(Variant(_6, 0), 0).fld0;
SetDiscriminant(_6, 3);
(*_8) = core::ptr::addr_of_mut!(_11);
place!(Field::<bool>(Variant(_6, 3), 0)) = !false;
place!(Field::<(u8, [u128; 7])>(Variant(_6, 3), 1)).0 = !_9.3.0.0.0;
RET.0.0.0 = RET.3.0;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(1_usize, 3_usize, Move(_3), 19_usize, Move(_19), 18_usize, Move(_18), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u128; 7],mut _2: [u128; 7],mut _3: ((u8, [u128; 7]),),mut _4: ((i32, i128), *const i32),mut _5: ((u8, [u128; 7]),),mut _6: ((u8, [u128; 7]),),mut _7: [u128; 7],mut _8: i32,mut _9: ((u8, [u128; 7]),),mut _10: f32,mut _11: ((u8, [u128; 7]),)) -> i16 {
mir! {
type RET = i16;
let _12: [isize; 3];
let _13: ([i32; 5], [u16; 8]);
let _14: &'static i32;
let _15: *const char;
let _16: &'static usize;
let _17: [i32; 5];
let _18: isize;
let _19: &'static f32;
let _20: char;
let _21: [i64; 1];
let _22: &'static &'static f32;
let _23: ();
let _24: ();
{
_6 = (_3.0,);
_6 = (_3.0,);
_3.0.1 = [244482751819514754513645900373119129501_u128,98993847325992790425327509572726959036_u128,158203849717128685428060699120653534893_u128,203023053160844413555022831197607145230_u128,302871342027391237606866924989129808127_u128,129978718120793570440767994659820185440_u128,274715612346019284606541389299998897839_u128];
_13.0 = [_8,_4.0.0,_8,_4.0.0,_4.0.0];
_13.1 = [47056_u16,29723_u16,26014_u16,13729_u16,52934_u16,27188_u16,48777_u16,59732_u16];
_5.0.0 = !_6.0.0;
_4.0 = (_8, 157481708006918723883158608279636981587_i128);
_8 = 4054025954_u32 as i32;
_11.0 = (_3.0.0, _9.0.1);
_13.0 = [_8,_4.0.0,_4.0.0,_8,_8];
_2 = [23291253803125598759645853821780984567_u128,56854100525846188881634936084492191975_u128,164107484982665705878665590577045798757_u128,299114315320659459388650468957496473892_u128,199304286769929496478420535376787805059_u128,23769554684254619685589856963974012309_u128,287941103985961180976386959538382447790_u128];
RET = 22497_i16;
_2 = _11.0.1;
_11.0.1 = _5.0.1;
_11.0.0 = _3.0.0;
_9.0.0 = !_6.0.0;
_14 = &_8;
_11.0 = (_9.0.0, _3.0.1);
_6.0.0 = _9.0.0 >> _3.0.0;
_6.0.1 = [172308190598443452566002041116907944348_u128,232231329913026285753827997751335109588_u128,207774562776570164408686617402636743973_u128,62197083906286312081558643773506656863_u128,20290331093100782447049845424749993034_u128,47467196964739881754398108502568504813_u128,108382776902991239242160174765865601717_u128];
_12 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_13.1 = [63938_u16,55189_u16,15015_u16,26341_u16,28865_u16,64532_u16,33492_u16,7611_u16];
_11.0 = _5.0;
_6.0.0 = 15387248078908142882_u64 as u8;
_3.0 = (_6.0.0, _9.0.1);
_5.0.1 = [275773380678075376424640132553755364285_u128,318651187920487595783309618974048164865_u128,295700642125354636379955499015721506338_u128,214970620655780517123758850966895454258_u128,68210979548227832520420202349861770791_u128,229669648629800737488010589635978863090_u128,25204325976109181181183512533208430018_u128];
Call(_3.0.1 = fn3(Move(_14), _13.0, _5, _9.0, _9.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _6;
_3.0.1 = [193664504652055513386780494881014892738_u128,68299465349923978817159094097280861587_u128,94335394075363186833336380446931049129_u128,180314469863405944631248977729565549731_u128,127353838285134116656702813702070917835_u128,255220733388476480636419262377538060462_u128,22916756555859389168219816314571685367_u128];
_1 = _5.0.1;
_9 = (_5.0,);
_9.0.0 = !_11.0.0;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
22497 => bb6,
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
_8 = _4.0.0 | _4.0.0;
_11.0 = (_9.0.0, _1);
_6.0.0 = _5.0.0;
_3.0 = _9.0;
_9 = _11;
_3 = (_6.0,);
_5.0 = _6.0;
_8 = _4.0.0;
_5.0.0 = _11.0.0;
_4.0 = (_8, (-72583713236426199268093586749810582634_i128));
_13.0 = [_4.0.0,_8,_8,_4.0.0,_8];
_11.0 = _9.0;
_11.0.0 = _5.0.0;
_3 = _6;
_6.0.1 = [251964588737357467941144845090613566026_u128,100511849082718423295507431297920715537_u128,272561258387574081980445368139074538206_u128,58173453866014577079991153693748742190_u128,35110407647465755678786081212382581787_u128,118651025756232709534250268195827118871_u128,49371714919420432520638753575799272474_u128];
_4.0.0 = _8;
_2 = _7;
_6 = (_9.0,);
_1 = [25869434408918780263085254346022923970_u128,215716350671797459087463730572680113534_u128,243272990500136115256921149499708905645_u128,233381736747825473378772928194103355011_u128,16038122362505582262287253206555838574_u128,69230747390872961905071704387903563017_u128,190446727699543952304965754952451153863_u128];
_11 = (_9.0,);
_9.0 = _11.0;
_17 = [_4.0.0,_8,_8,_8,_8];
_19 = &_10;
_3.0.0 = _5.0.0;
Goto(bb7)
}
bb7 = {
_11 = _3;
_15 = core::ptr::addr_of!(_20);
_3 = _11;
_5.0.1 = [77123658406019874768724713885377211783_u128,20686550449528113759942076939625133307_u128,252171375525987224960591440605102316045_u128,83013602832942473897073335190120595836_u128,119453371877426141029527091230051640160_u128,130914329279514825999373984176208037300_u128,37345549001540750351921606011953705098_u128];
_2 = [325331696480334376088503836302704040227_u128,138949011326210001423987957525969131808_u128,21249370814383179551824864662479574111_u128,314769876635069097193617215845978355566_u128,247921765934392703632642696080931958193_u128,253612955836392079843474764612466268611_u128,307649658116756151360978551656927912206_u128];
Goto(bb8)
}
bb8 = {
_4.1 = core::ptr::addr_of!(_8);
_11.0.0 = _9.0.0;
_15 = core::ptr::addr_of!((*_15));
_3.0 = _9.0;
_15 = core::ptr::addr_of!(_20);
_3 = (_9.0,);
_6.0.0 = !_9.0.0;
RET = (-7469_i16);
match _4.0.1 {
0 => bb4,
1 => bb2,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
267698653684512264195281020681957628822 => bb15,
_ => bb14
}
}
bb9 = {
_11 = _3;
_15 = core::ptr::addr_of!(_20);
_3 = _11;
_5.0.1 = [77123658406019874768724713885377211783_u128,20686550449528113759942076939625133307_u128,252171375525987224960591440605102316045_u128,83013602832942473897073335190120595836_u128,119453371877426141029527091230051640160_u128,130914329279514825999373984176208037300_u128,37345549001540750351921606011953705098_u128];
_2 = [325331696480334376088503836302704040227_u128,138949011326210001423987957525969131808_u128,21249370814383179551824864662479574111_u128,314769876635069097193617215845978355566_u128,247921765934392703632642696080931958193_u128,253612955836392079843474764612466268611_u128,307649658116756151360978551656927912206_u128];
Goto(bb8)
}
bb10 = {
_8 = _4.0.0 | _4.0.0;
_11.0 = (_9.0.0, _1);
_6.0.0 = _5.0.0;
_3.0 = _9.0;
_9 = _11;
_3 = (_6.0,);
_5.0 = _6.0;
_8 = _4.0.0;
_5.0.0 = _11.0.0;
_4.0 = (_8, (-72583713236426199268093586749810582634_i128));
_13.0 = [_4.0.0,_8,_8,_4.0.0,_8];
_11.0 = _9.0;
_11.0.0 = _5.0.0;
_3 = _6;
_6.0.1 = [251964588737357467941144845090613566026_u128,100511849082718423295507431297920715537_u128,272561258387574081980445368139074538206_u128,58173453866014577079991153693748742190_u128,35110407647465755678786081212382581787_u128,118651025756232709534250268195827118871_u128,49371714919420432520638753575799272474_u128];
_4.0.0 = _8;
_2 = _7;
_6 = (_9.0,);
_1 = [25869434408918780263085254346022923970_u128,215716350671797459087463730572680113534_u128,243272990500136115256921149499708905645_u128,233381736747825473378772928194103355011_u128,16038122362505582262287253206555838574_u128,69230747390872961905071704387903563017_u128,190446727699543952304965754952451153863_u128];
_11 = (_9.0,);
_9.0 = _11.0;
_17 = [_4.0.0,_8,_8,_8,_8];
_19 = &_10;
_3.0.0 = _5.0.0;
Goto(bb7)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_3 = _6;
_3.0.1 = [193664504652055513386780494881014892738_u128,68299465349923978817159094097280861587_u128,94335394075363186833336380446931049129_u128,180314469863405944631248977729565549731_u128,127353838285134116656702813702070917835_u128,255220733388476480636419262377538060462_u128,22916756555859389168219816314571685367_u128];
_1 = _5.0.1;
_9 = (_5.0,);
_9.0.0 = !_11.0.0;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
22497 => bb6,
_ => bb5
}
}
bb14 = {
Return()
}
bb15 = {
_3.0.1 = [22265247812271269273694334023049620735_u128,331791242268433677768310653721272931162_u128,177048930173093422907954591196682935399_u128,280833375026963765933488042486821743990_u128,80789149228364688977426598760901929829_u128,130180588075254986765055280986656944727_u128,122398222255132967233377054303425496804_u128];
_5.0.0 = !_6.0.0;
_11.0 = (_9.0.0, _1);
_12 = [127_isize,(-80_isize),(-9223372036854775808_isize)];
RET = 1903_i16;
_14 = &_4.0.0;
_6.0.1 = [182819422878305622876744759418418933969_u128,258880559595275958543062510951323514704_u128,106683717594951736961038485065121688825_u128,122101785794371381752578436290009002981_u128,133181337778001200863533428387091062766_u128,303433302304745888324769398276238492330_u128,103135882009825570792796122777366258240_u128];
_2 = [148057691015581570452206787353750550484_u128,250454374333516180868587269999520334632_u128,128857712819688235412717220433191734162_u128,212637298646958648454497803787301144364_u128,307424802578946476043130640765072146887_u128,40210327961044675853014921090428232991_u128,151023796145166313885418531144706295451_u128];
_1 = _2;
_3.0.1 = [77946828028989934533766172424188678796_u128,202612874367888997157972651042854896375_u128,196528134975509608009312638922257746858_u128,152868089016869283974274500116547514405_u128,208105782883667438261347909407611561847_u128,17791384078849960362273020760458125843_u128,242273324255795721407529967842235433413_u128];
_3.0.0 = _5.0.0 ^ _11.0.0;
_9 = (_11.0,);
_9.0.1 = [201825609616551475108490993489595495921_u128,145317666794295118334501230559428184171_u128,17161283504320854195184048084077873686_u128,71612179334354275180751358557684574911_u128,267232720578247906169249550900212717506_u128,238645103812675273418677686610328149684_u128,150293310083396086816287413501008435412_u128];
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(2_usize, 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(2_usize, 9_usize, Move(_9), 8_usize, Move(_8), 24_usize, _24, 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: &'static i32,mut _2: [i32; 5],mut _3: ((u8, [u128; 7]),),mut _4: (u8, [u128; 7]),mut _5: (u8, [u128; 7])) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _6: &'static *const i32;
let _7: bool;
let _8: ((i32, i128), *const i32);
let _9: Adt82;
let _10: &'static &'static f32;
let _11: char;
let _12: *const i32;
let _13: bool;
let _14: &'static &'static *mut u32;
let _15: *mut u16;
let _16: &'static i32;
let _17: isize;
let _18: ([i32; 5], [u16; 8]);
let _19: f32;
let _20: (u8, [u128; 7]);
let _21: isize;
let _22: Adt70;
let _23: (&'static &'static [i64; 1], (Adt24, u16));
let _24: u64;
let _25: u128;
let _26: &'static &'static [i64; 1];
let _27: i128;
let _28: (u8, [u128; 7]);
let _29: f32;
let _30: (i32, i128);
let _31: u16;
let _32: i16;
let _33: i32;
let _34: &'static &'static &'static [i64; 1];
let _35: [isize; 3];
let _36: (&'static (*mut u32, (((i32, i128), *const i32), i16, *mut u32, (i32, i128))), *mut u32, [isize; 7], (Adt24, u16));
let _37: Adt51;
let _38: bool;
let _39: ();
let _40: ();
{
_5.1 = [336703998071457143474925975054243318412_u128,220948077634449004418983713578745903893_u128,118419662375373156090064104812500716885_u128,247329145290926280250256358928533698341_u128,178211331117938824191194118293222881529_u128,218994177205098384874976687289334592279_u128,274881970530842663897970858952484307442_u128];
_3 = (_5,);
RET = [87037733473084512483540764923106504985_u128,195937286441332739681043129778740973443_u128,182449356773791893987282527808466350027_u128,93745395632916988840312332463296620224_u128,311155833078592999127106996524776117925_u128,272434603594362595588285890675602913198_u128,162843316192504511226071513272525371256_u128];
_3.0.0 = _4.0;
_4.0 = _5.0 << _5.0;
_7 = _3.0.0 == _5.0;
_5.1 = [223544124403037035885193934751167271165_u128,323849961318930181508560490547022581317_u128,261074115445373213298793294021244298900_u128,198861390216530625703252284541517721791_u128,93068440140187928052201473809793253706_u128,130689534272443641016282741295674461745_u128,83421453452478612603450786257789129609_u128];
_7 = !false;
RET = _3.0.1;
_4.1 = [80927725348720574154077990246470138632_u128,8613863538586260381842683804190188097_u128,176045962174642426729079716567552103249_u128,33090374722188782482577583430322424835_u128,157304921466770789778648093939512244225_u128,139293789789497627411064448450895134744_u128,9439331507259539478912820492374592214_u128];
_3 = (_5,);
_5.1 = RET;
_3.0.0 = 19707_u16 as u8;
_4 = _5;
_6 = &_8.1;
Call(_4.0 = core::intrinsics::bswap(_3.0.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.1 = core::ptr::addr_of!(_8.0.0);
Call(RET = fn4(_3.0, _3.0, _4, _3.0, _3, _3, _5.1, _3, _5.1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.0 = _7 as u8;
_1 = &_8.0.0;
_1 = &(*_1);
_8.0.1 = 15779783239424139749352043098429675946_i128;
_5.0 = !_3.0.0;
_11 = '\u{935ff}';
_3.0.1 = [226037695221044025326433152488296460656_u128,70035671680926836545780364505668776653_u128,339195735376443646505546520672674595428_u128,145237251388825438552851792766989232489_u128,299859224701307670885149369315604115078_u128,286877234666427750889284719552328226657_u128,45450824867167205381286484595898485448_u128];
_8.0.1 = -(-7148376516333676577773891277337014388_i128);
_3.0.1 = [245850895489792290017627907681842088393_u128,204794609850261225250749091448992897261_u128,330075557991208152927154427536412970701_u128,246901977936758569601287457565441879627_u128,292961888302282221006510823989830795745_u128,126921811538714505859239408625500695245_u128,149731889181328926339110138592274029868_u128];
_6 = &_12;
_7 = !false;
Call(_3 = fn14(_2, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4.1 = [302292544245272328383654267448913860707_u128,266658335451594118059107124031311309944_u128,103865740506149791850385724095425024874_u128,46018921527786032945359243101453083072_u128,32597238915644896073294156951179564487_u128,331630138790414777622653985808324487158_u128,140278633493363136533606968275268645638_u128];
_8.0.1 = -(-164882256918219993483739501234971386958_i128);
_5.1 = [185936361056536979346778146323414405315_u128,328690959473581979891678525458629292464_u128,40893134249854708223675627497020457883_u128,81752211041308714887201274882616127998_u128,187755139773877419983508912758560518146_u128,291839787869331454676675242963008298308_u128,69810504614316947587377210996729247541_u128];
RET = _3.0.1;
_8.0 = ((-1423430258_i32), (-102400585352886139769632781845791772213_i128));
_3.0 = (_4.0, RET);
_5 = (_3.0.0, _3.0.1);
match _8.0.0 {
0 => bb2,
1 => bb4,
340282366920938463463374607430344781198 => bb6,
_ => bb5
}
}
bb4 = {
_4.0 = _7 as u8;
_1 = &_8.0.0;
_1 = &(*_1);
_8.0.1 = 15779783239424139749352043098429675946_i128;
_5.0 = !_3.0.0;
_11 = '\u{935ff}';
_3.0.1 = [226037695221044025326433152488296460656_u128,70035671680926836545780364505668776653_u128,339195735376443646505546520672674595428_u128,145237251388825438552851792766989232489_u128,299859224701307670885149369315604115078_u128,286877234666427750889284719552328226657_u128,45450824867167205381286484595898485448_u128];
_8.0.1 = -(-7148376516333676577773891277337014388_i128);
_3.0.1 = [245850895489792290017627907681842088393_u128,204794609850261225250749091448992897261_u128,330075557991208152927154427536412970701_u128,246901977936758569601287457565441879627_u128,292961888302282221006510823989830795745_u128,126921811538714505859239408625500695245_u128,149731889181328926339110138592274029868_u128];
_6 = &_12;
_7 = !false;
Call(_3 = fn14(_2, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_8.1 = core::ptr::addr_of!(_8.0.0);
Call(RET = fn4(_3.0, _3.0, _4, _3.0, _3, _3, _5.1, _3, _5.1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_8.1 = core::ptr::addr_of!(_8.0.0);
_3.0.1 = [122344749508447151126608138630680357267_u128,25157182221959067321180588236639599490_u128,86410491898029760765032054792234378236_u128,160061638150582264295260065256844705834_u128,5113165274435042673967370814089187678_u128,133339889310507387935736024136102317371_u128,277659519937230953483130354086431562260_u128];
_7 = true;
_8.0.0 = (-1_i8) as i32;
_7 = true;
_2 = [_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0];
_12 = Move(_8.1);
_16 = &_8.0.0;
_3.0.1 = [1757979454117417312032795109513387717_u128,282764473202107048840599403379882694702_u128,276119767987602433007927989694698570908_u128,19160656030819406038770136964361522056_u128,222910605845883252018143989667484948001_u128,213868309667276735924794812006656094939_u128,186804040692542558849533351814982241098_u128];
_11 = '\u{fdb8}';
_16 = &_8.0.0;
_16 = &(*_16);
_7 = !false;
_3 = (_4,);
_4.1 = [64320984964932101768032490230750692686_u128,146777204430098021474841043327582983303_u128,317498091890897610744093783258843554134_u128,156502917642183398561346675848739936626_u128,29077102533928634204306859950540701662_u128,72346426058341951940598573684563504608_u128,224269311167245601779463389830583361160_u128];
Goto(bb7)
}
bb7 = {
_5 = (_3.0.0, RET);
RET = [244946323096198098454379004921100261413_u128,167291568876039578441202722902369654904_u128,239781147211981543113851249358970692115_u128,125562625025112020400538840418474671134_u128,299875847953778369103093736744423317557_u128,173818944108258686456421275540273660322_u128,230831969094322005939386626773601007236_u128];
_13 = _7;
_17 = 68_isize;
_2 = [_8.0.0,(*_16),(*_16),(*_16),(*_16)];
_16 = &(*_16);
_16 = &_8.0.0;
_18.1 = [59989_u16,59596_u16,10544_u16,17685_u16,28785_u16,13570_u16,26774_u16,16870_u16];
_7 = _13 & _13;
_8.1 = core::ptr::addr_of!(_8.0.0);
_1 = &(*_16);
_18.0 = _2;
_17 = (-9223372036854775808_isize);
_16 = &(*_16);
_21 = _17 ^ _17;
_4.1 = [233238554172932105237213990524045744633_u128,338289854742872270704912687806987467701_u128,235685730653704802028885025820378920675_u128,111154826658212927437541138243020457516_u128,12121002125796962682444786319271467694_u128,321079688485383846447821763163530691358_u128,283191367075722190352131982748257351716_u128];
_18.0 = [(*_1),(*_1),(*_16),_8.0.0,(*_16)];
_7 = _13;
RET = [30686324074987272169434976400700009243_u128,175724457936312663759726202749297833693_u128,310414517551649560031986110185996423377_u128,47561716124096505212379997143092961425_u128,191504178644905644194547015456477597910_u128,249523137952217678453247153516572835562_u128,309402779923981592486317232957282275473_u128];
_20.0 = _5.0 ^ _3.0.0;
_4.1 = _5.1;
_4 = (_20.0, _3.0.1);
_4.0 = _11 as u8;
_6 = &_12;
_20 = (_3.0.0, _5.1);
_16 = &(*_1);
match _8.0.1 {
237881781568052323693741825585976439243 => bb8,
_ => bb3
}
}
bb8 = {
_20.1 = [249508229580377004232822032275607027161_u128,145698882398815414112439465882603965930_u128,146968680387877945081466377935505383580_u128,148671413477557262013784928367453160953_u128,177247500952297559479725069277389259527_u128,239687025235457919540476481853239141192_u128,113149842188386181442710981442509169017_u128];
Goto(bb9)
}
bb9 = {
_8.0.1 = -(-118598570642488272908034603089536281138_i128);
_8.1 = core::ptr::addr_of!((*_16));
_4.1 = [196387097610743588213312969817628896575_u128,185385267073247892279434800356236274400_u128,67524455789393182070768725786229893737_u128,67879051241214802123948300771715723980_u128,171422396584617918720682192145315900927_u128,88287204516606940533473595811835339778_u128,206283184994233915480690157060706991027_u128];
_21 = 3726636524788631471_i64 as isize;
_24 = 1567680339742313256_u64 + 8388914828040227366_u64;
_20 = (_3.0.0, _4.1);
_6 = &(*_6);
_23.1.0 = Adt24::Variant3 { fld0: _7,fld1: _4 };
_3.0.0 = Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1).0;
_23.1.1 = !32921_u16;
_3.0.0 = Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1).0 << _24;
_28 = _5;
_25 = 127502792362906713060694198543018732327_u128;
RET = [_25,_25,_25,_25,_25,_25,_25];
_24 = 463435203092033051_u64 & 6095780068462000761_u64;
SetDiscriminant(_23.1.0, 3);
place!(Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1)).1 = [_25,_25,_25,_25,_25,_25,_25];
_28 = (_3.0.0, _3.0.1);
_8.0.1 = !108949257310533538385298285579268717258_i128;
_20.1 = [_25,_25,_25,_25,_25,_25,_25];
_20 = (_3.0.0, _5.1);
Goto(bb10)
}
bb10 = {
_7 = _13;
RET = [_25,_25,_25,_25,_25,_25,_25];
_8.0.0 = 740481742_i32 + 1978688306_i32;
_4.0 = _28.0 & _28.0;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
340282366920938463454151235394913435648 => bb11,
_ => bb8
}
}
bb11 = {
_15 = core::ptr::addr_of_mut!(_23.1.1);
_3.0.0 = (-8972252609984998121_i64) as u8;
_19 = _8.0.0 as f32;
_29 = _19 * _19;
_8.0.0 = 950869989_i32;
place!(Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1)).0 = _25 as u8;
_11 = '\u{c4de2}';
place!(Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1)).1 = [_25,_25,_25,_25,_25,_25,_25];
_12 = Move(_8.1);
place!(Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1)) = (_4.0, _5.1);
_3.0.1 = Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1).1;
place!(Field::<(u8, [u128; 7])>(Variant(_23.1.0, 3), 1)).0 = _20.0;
_20 = _5;
_8.1 = core::ptr::addr_of!(_33);
_16 = &_30.0;
_23.1.1 = _8.0.1 as u16;
_7 = !_13;
_23.1.1 = 64602_u16 + 27275_u16;
_27 = _3.0.0 as i128;
match _25 {
0 => bb9,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb12,
6 => bb13,
127502792362906713060694198543018732327 => bb15,
_ => bb14
}
}
bb12 = {
_4.0 = _7 as u8;
_1 = &_8.0.0;
_1 = &(*_1);
_8.0.1 = 15779783239424139749352043098429675946_i128;
_5.0 = !_3.0.0;
_11 = '\u{935ff}';
_3.0.1 = [226037695221044025326433152488296460656_u128,70035671680926836545780364505668776653_u128,339195735376443646505546520672674595428_u128,145237251388825438552851792766989232489_u128,299859224701307670885149369315604115078_u128,286877234666427750889284719552328226657_u128,45450824867167205381286484595898485448_u128];
_8.0.1 = -(-7148376516333676577773891277337014388_i128);
_3.0.1 = [245850895489792290017627907681842088393_u128,204794609850261225250749091448992897261_u128,330075557991208152927154427536412970701_u128,246901977936758569601287457565441879627_u128,292961888302282221006510823989830795745_u128,126921811538714505859239408625500695245_u128,149731889181328926339110138592274029868_u128];
_6 = &_12;
_7 = !false;
Call(_3 = fn14(_2, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_4.0 = _7 as u8;
_1 = &_8.0.0;
_1 = &(*_1);
_8.0.1 = 15779783239424139749352043098429675946_i128;
_5.0 = !_3.0.0;
_11 = '\u{935ff}';
_3.0.1 = [226037695221044025326433152488296460656_u128,70035671680926836545780364505668776653_u128,339195735376443646505546520672674595428_u128,145237251388825438552851792766989232489_u128,299859224701307670885149369315604115078_u128,286877234666427750889284719552328226657_u128,45450824867167205381286484595898485448_u128];
_8.0.1 = -(-7148376516333676577773891277337014388_i128);
_3.0.1 = [245850895489792290017627907681842088393_u128,204794609850261225250749091448992897261_u128,330075557991208152927154427536412970701_u128,246901977936758569601287457565441879627_u128,292961888302282221006510823989830795745_u128,126921811538714505859239408625500695245_u128,149731889181328926339110138592274029868_u128];
_6 = &_12;
_7 = !false;
Call(_3 = fn14(_2, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_8.1 = core::ptr::addr_of!(_8.0.0);
_3.0.1 = [122344749508447151126608138630680357267_u128,25157182221959067321180588236639599490_u128,86410491898029760765032054792234378236_u128,160061638150582264295260065256844705834_u128,5113165274435042673967370814089187678_u128,133339889310507387935736024136102317371_u128,277659519937230953483130354086431562260_u128];
_7 = true;
_8.0.0 = (-1_i8) as i32;
_7 = true;
_2 = [_8.0.0,_8.0.0,_8.0.0,_8.0.0,_8.0.0];
_12 = Move(_8.1);
_16 = &_8.0.0;
_3.0.1 = [1757979454117417312032795109513387717_u128,282764473202107048840599403379882694702_u128,276119767987602433007927989694698570908_u128,19160656030819406038770136964361522056_u128,222910605845883252018143989667484948001_u128,213868309667276735924794812006656094939_u128,186804040692542558849533351814982241098_u128];
_11 = '\u{fdb8}';
_16 = &_8.0.0;
_16 = &(*_16);
_7 = !false;
_3 = (_4,);
_4.1 = [64320984964932101768032490230750692686_u128,146777204430098021474841043327582983303_u128,317498091890897610744093783258843554134_u128,156502917642183398561346675848739936626_u128,29077102533928634204306859950540701662_u128,72346426058341951940598573684563504608_u128,224269311167245601779463389830583361160_u128];
Goto(bb7)
}
bb15 = {
_20.1 = _4.1;
_34 = &_23.0;
_5.1 = [_25,_25,_25,_25,_25,_25,_25];
_36.3.0 = Adt24::Variant3 { fld0: _13,fld1: _3.0 };
_5 = _20;
_36.2 = [_21,_17,_21,_21,_17,_17,_21];
_8.0 = (1063459590_i32, _27);
_8.0.1 = !_27;
_6 = &_12;
_36.2 = [_17,_21,_21,_17,_17,_21,_21];
_23.1 = (Move(_36.3.0), 47536_u16);
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(3_usize, 28_usize, Move(_28), 7_usize, Move(_7), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(3_usize, 13_usize, Move(_13), 21_usize, Move(_21), 18_usize, Move(_18), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (u8, [u128; 7]),mut _2: (u8, [u128; 7]),mut _3: (u8, [u128; 7]),mut _4: (u8, [u128; 7]),mut _5: ((u8, [u128; 7]),),mut _6: ((u8, [u128; 7]),),mut _7: [u128; 7],mut _8: ((u8, [u128; 7]),),mut _9: [u128; 7],mut _10: ((u8, [u128; 7]),)) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _11: isize;
let _12: f32;
let _13: Adt51;
let _14: *const i32;
let _15: &'static u64;
let _16: f32;
let _17: char;
let _18: &'static &'static [i64; 1];
let _19: f64;
let _20: (Adt24, u16);
let _21: Adt82;
let _22: isize;
let _23: (&'static (*mut u32, (((i32, i128), *const i32), i16, *mut u32, (i32, i128))), *mut u32, [isize; 7], (Adt24, u16));
let _24: [i64; 1];
let _25: u64;
let _26: i64;
let _27: u128;
let _28: [i128; 7];
let _29: u8;
let _30: i128;
let _31: f32;
let _32: i8;
let _33: [u128; 7];
let _34: &'static u8;
let _35: char;
let _36: i64;
let _37: *const *const char;
let _38: Adt18;
let _39: f32;
let _40: f64;
let _41: u128;
let _42: ([i128; 2], &'static isize, [i128; 7], (((u8, [u128; 7]),), [i128; 5], &'static &'static f32));
let _43: (&'static &'static [i64; 1], (Adt24, u16));
let _44: ();
let _45: ();
{
_8.0.0 = _10.0.0 * _3.0;
_4.0 = _5.0.0;
_8.0 = _4;
RET = [10964937095856501247338979813992655233_u128,29348106078944469781796590862546806681_u128,70190204473277569213569776270574212470_u128,202521282938338630444694313982947044587_u128,16214669798182520149120464292891722766_u128,127252655383326769631056279289918289115_u128,208147373069503687797161771000063512305_u128];
_5.0.1 = _1.1;
_10.0.1 = [293377922268599342039833053351017713334_u128,285256255411368381261838289670978225212_u128,68255880238472180435371315903990432315_u128,314766144832476042536112272645002867467_u128,121834849900207715181278892254735803779_u128,67732571341658622673402676645982189097_u128,337165576971423515158092443074287684740_u128];
_4 = _8.0;
_2 = (_6.0.0, _7);
_3.1 = [60219803713758469969883969651002801454_u128,165089147000085935701274565772502761674_u128,188995551729051147017772137288636807371_u128,313624502009380959218187534275854909334_u128,213283479896758130527006274189045817809_u128,255253807464187576559625958642779033259_u128,175855434487202796664754779110024460924_u128];
_10.0 = (_8.0.0, _7);
_5.0 = (_10.0.0, _2.1);
_5 = (_8.0,);
_10.0.0 = 14262863453228139045_u64 as u8;
_1 = (_10.0.0, _7);
Goto(bb1)
}
bb1 = {
_5 = (_3,);
_10.0.0 = '\u{7a0ba}' as u8;
_4 = _5.0;
Goto(bb2)
}
bb2 = {
_2.0 = !_6.0.0;
_11 = (-55_isize);
_3.0 = 14626279066932630501_u64 as u8;
_12 = 4223506829726587576_u64 as f32;
_5.0.1 = _3.1;
_9 = [338998162879817917730343638290253921784_u128,230590737792560541706872315944957967318_u128,16614405590011281955734398812584373553_u128,220349184062442701839546614056677765282_u128,41000085836286420899081663907402948392_u128,309850223424268158149480341019997337637_u128,255201698878624936093708198144250304882_u128];
_5.0 = (_2.0, _2.1);
_2 = (_8.0.0, _1.1);
_12 = 672865513857276151_usize as f32;
_4.1 = _6.0.1;
_10.0.1 = [333708224000933974507372586857825354130_u128,333626194631914924864691601020911988894_u128,309813600146981970604705599804792491573_u128,216787432958124633881060137595340565719_u128,106455766963529272528488342214843518760_u128,193080247394427056636222332749609520011_u128,303684207943628511715420186871735491811_u128];
_9 = [81957698485181293968496504449787213632_u128,142043926665360193987045494064302264575_u128,128394764326870060108908520520597561167_u128,7044697851958813376553276744336456886_u128,279112691057711208072908018165475381570_u128,134486718731265438456305541489366441756_u128,158413958481221975897359501267508546935_u128];
RET = [195631159163832071711545081260444541349_u128,28740877092811555385015607455701357493_u128,243391941911924060102376614109143012492_u128,127687834296573024845289579230632495606_u128,338576612046636012438769911453898098578_u128,223411647450123776681743438760290260352_u128,210691962074896424349208453733681603999_u128];
_10.0 = (_6.0.0, _6.0.1);
_10.0.0 = _6.0.0 + _5.0.0;
_7 = [335992476175721274830426895955129737455_u128,76836636589796162029292784379916319267_u128,176561449561732128188840429470982671605_u128,325106046313783066816343210736069757734_u128,37691510704857480969356053508653248872_u128,142727262014664114313366912257027546676_u128,104958724073025257349073466861605029646_u128];
_8 = (_2,);
_1.1 = _8.0.1;
_4.1 = [183171093505016908738120565392614900858_u128,122625348958691117855754570837885387025_u128,2400105940763870413479090291689249864_u128,254564593191626937935227866914962647302_u128,189580720737692952071114389439178436954_u128,320713245176273302886963877944458799164_u128,13595969823505913523883266466379184818_u128];
_1.0 = _12 as u8;
_2 = (_10.0.0, RET);
_4.1 = _9;
match _11 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211401 => bb7,
_ => bb6
}
}
bb3 = {
_5 = (_3,);
_10.0.0 = '\u{7a0ba}' as u8;
_4 = _5.0;
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
_3.1 = _4.1;
_10.0.1 = [195292049700574443846205910672053279903_u128,265883247200009754306999905230165539626_u128,108044908427279478901644663679895604312_u128,11176708254760688900627132755893537410_u128,179280776483612181467576254298492891710_u128,293003154585893303207000181568889375584_u128,264215282038616438964002716747431064575_u128];
_10.0 = _1;
_10 = (_4,);
_4.0 = _6.0.0 * _10.0.0;
_10.0 = _2;
_8 = _10;
_8.0 = _1;
_5.0 = (_10.0.0, _4.1);
_5.0.0 = 16223773059444042492_u64 as u8;
_5.0.1 = [42275286476234071998055679474392276877_u128,1947316320357067450016265376052315386_u128,37461316393333054404090877317241230962_u128,305660770351616962959974532668179455150_u128,308317712509603158409058160742255823550_u128,64786547359169025410261854209532269309_u128,66659175896317450092795479733486744661_u128];
_1.1 = [174719207466823995211753456997501367620_u128,243635282505812036345606700284383155916_u128,166777454034129055167678209186652712026_u128,153523961379844627188724904082970837673_u128,169146834512887337273404563523468035558_u128,272950457672552346336585027831453197474_u128,124549485386582329902365308253145532780_u128];
_3.1 = _1.1;
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
340282366920938463463374607431768211401 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_6 = (_1,);
_5.0.1 = [207877304968790023770353716832216935831_u128,60418004450252647582698560201485544264_u128,16763245579021238937176673811412926117_u128,304080928065898316850282138460171470345_u128,10686402610051499192225853629635425028_u128,175388270300151408140573239110530346004_u128,213242039165116174793069971917782496021_u128];
RET = [108276464122901813190909940712972718445_u128,193484053434917613790859791625403157732_u128,337649325750777414192160466822266136585_u128,238919274725434656988192977898430102421_u128,162990517956522957475236366843117583513_u128,122151700730749644055966969700555276081_u128,97401905095523141042793048074768132720_u128];
_3 = (_6.0.0, _10.0.1);
_5.0.0 = _4.0 & _10.0.0;
_8.0.1 = [147153396675314676075446443854123478514_u128,125228516737503400281248114403545001820_u128,194225955436373518309406868507863421978_u128,107354352898997310077464042624999526855_u128,227964242655378781027200444225726961004_u128,164653288117535287343952822230925266657_u128,156017649663242397652941135707963199084_u128];
_19 = 103_i8 as f64;
_12 = 121502152412695689225893826555502707921_i128 as f32;
_11 = 85_isize | (-9223372036854775808_isize);
_5.0 = (_2.0, _3.1);
_5 = (_3,);
_4 = _3;
Goto(bb10)
}
bb10 = {
_5.0.1 = [79443275036816903919347683498065943983_u128,110010561738532763950229723481276084392_u128,272036466238408244164268498561521293598_u128,177128422288966774692359630408056559497_u128,142148207134080999748621780850086256614_u128,10058386835914288543825419589173909448_u128,203126034112652774411460027154690530173_u128];
_12 = 28_i8 as f32;
_3 = (_1.0, _4.1);
_2 = (_10.0.0, _9);
_5 = _6;
_6.0 = (_3.0, _7);
_1 = (_2.0, _5.0.1);
_12 = _19 as f32;
_22 = _11;
_1 = (_8.0.0, _7);
_1.1 = _3.1;
_2.0 = !_10.0.0;
Call(_2.1 = fn5(_5, _4.1, _8.0, _4, _6.0.1, _4.1, _4, _9, _1.1, _10.0.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_1.0 = 96681992144618199312519164303650236876_i128 as u8;
_19 = (-578991948_i32) as f64;
_23.3.0 = Adt24::Variant1 { fld0: (-1021_i16) };
_5.0.0 = _1.0 ^ _6.0.0;
_2.1 = [171498067989632132844255220397930337942_u128,59029591085623402851970553470522890794_u128,19271940969920619543576677426168000961_u128,254084170958719928837153781290212271418_u128,223113361849201469185792245770170929440_u128,271883989376180397561703010060686293121_u128,2559866002542750345027161984468378592_u128];
_10.0.0 = _3.0 | _1.0;
_23.2 = [_22,_22,_11,_11,_22,_22,_11];
_8 = _6;
_4 = _2;
Goto(bb12)
}
bb12 = {
_12 = _11 as f32;
_15 = &_25;
_20.0 = Adt24::Variant3 { fld0: true,fld1: _5.0 };
_8.0.1 = [287103209759942673208972392010365879234_u128,75115317568840022915343079212971342337_u128,68948458435588920345967287982387104770_u128,201177244501381641443731719698920732518_u128,130264830791907611802629916557025487509_u128,105048035294983218489887725132790300719_u128,147204365557256350027948407598982792907_u128];
_3.1 = _2.1;
_4.1 = _7;
_24 = [(-2347207126946252513_i64)];
Call(_4 = fn12(_2.1, _6.0.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = [324519864536365601825857881818304298065_u128,89271117743607390296545953248972351611_u128,153825689270987697505091054841929671801_u128,276774888493985885532378176238376452866_u128,288300029998732013201895728654312023314_u128,130168645464790491572630203753555329729_u128,158056572887357782636450615243436819353_u128];
_10.0 = (_4.0, _8.0.1);
_2 = (_4.0, RET);
_1 = _4;
_10.0 = (_4.0, Field::<(u8, [u128; 7])>(Variant(_20.0, 3), 1).1);
_26 = 5009109088971465901_i64;
_25 = 1668202721206869981_u64;
_8.0 = (_4.0, RET);
_35 = '\u{cb0e4}';
place!(Field::<(u8, [u128; 7])>(Variant(_20.0, 3), 1)).1 = [141815730567126380664962238623358151619_u128,257403039594836628583103557682345611664_u128,128983368594021646690870084799515167868_u128,117643795367370771956464728948231279635_u128,14492025470422318409430490379444319521_u128,115677212044379592365576484872145537902_u128,56369392039823384514861771834007578536_u128];
_38.fld3 = 83_i8;
_33 = [334861906306709379301450290900488508667_u128,120336564718029713574770939378333255858_u128,332655556012247401972691746665295455470_u128,29494556675638794782750907274485643815_u128,244215956987620272615528025825091707244_u128,80795228806584705997364299024244293581_u128,72207395890846713416603006726101548091_u128];
_6.0 = (_10.0.0, _33);
_24 = [_26];
place!(Field::<(u8, [u128; 7])>(Variant(_20.0, 3), 1)) = _10.0;
_2.1 = [35934591626599961227555851249885868728_u128,149584969910477695646477093753873831763_u128,160734729776259095210991399064964479346_u128,13238006658212260500113367789342418148_u128,149654159462904189553718323891842157903_u128,274431143557902453209596483771827068182_u128,111567689464532199240409820089156433381_u128];
Goto(bb14)
}
bb14 = {
_5 = (_6.0,);
_11 = !_22;
_42.3.1 = [90248751928648464060403259897654960863_i128,103318473161678995631165100797709648178_i128,23578185389085644952272447390296980564_i128,(-134716381426426683333379757798183460641_i128),141134757074490173345226184862257007834_i128];
_42.3.0.0.0 = _5.0.0 << _6.0.0;
_3.0 = !_1.0;
_42.3.0.0.1 = [282166465769170480764574398674509638334_u128,230608378783992682666207081685169788134_u128,240372624689835079667238855682663632175_u128,95621620477530811848285266350279122404_u128,82535703381456915207534831764877930068_u128,267709123787911770833657615434162385251_u128,10498298993646630352616217963242268621_u128];
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(4_usize, 24_usize, Move(_24), 9_usize, Move(_9), 22_usize, Move(_22), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(4_usize, 33_usize, Move(_33), 8_usize, Move(_8), 35_usize, Move(_35), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: ((u8, [u128; 7]),),mut _2: [u128; 7],mut _3: (u8, [u128; 7]),mut _4: (u8, [u128; 7]),mut _5: [u128; 7],mut _6: [u128; 7],mut _7: (u8, [u128; 7]),mut _8: [u128; 7],mut _9: [u128; 7],mut _10: [u128; 7]) -> [u128; 7] {
mir! {
type RET = [u128; 7];
let _11: [i128; 2];
let _12: isize;
let _13: (u8, [u128; 7]);
let _14: &'static &'static f32;
let _15: *mut u32;
let _16: ((u8, [u128; 7]), Adt24, [u128; 7]);
let _17: [isize; 3];
let _18: *mut u32;
let _19: f32;
let _20: [isize; 4];
let _21: (i32, i128);
let _22: char;
let _23: i64;
let _24: f64;
let _25: isize;
let _26: &'static [bool; 2];
let _27: bool;
let _28: ((i32, i128), &'static *mut u32, char, &'static &'static f32);
let _29: Adt51;
let _30: f64;
let _31: [i32; 7];
let _32: (&'static &'static [i64; 1], (Adt24, u16));
let _33: f64;
let _34: isize;
let _35: char;
let _36: f64;
let _37: *mut u8;
let _38: [bool; 2];
let _39: isize;
let _40: [i128; 7];
let _41: bool;
let _42: ();
let _43: ();
{
_9 = [260838061932390431222591334736106166794_u128,296682801897372911048447058571132624171_u128,16737176437217911045309462490055726048_u128,21758337373421927519628399769054863445_u128,69813685077682620458879242538939381911_u128,164636716179237466568210976770030477818_u128,167576362202709594319696168119217994647_u128];
_8 = _9;
RET = _6;
_1.0 = (_4.0, _9);
_9 = [150640148972334684042061170246938326307_u128,64037199271847898144713585212947495718_u128,11927122263975789399304397929039572658_u128,146483162416097933926376351488712002329_u128,172367200907706572200324015613695653885_u128,324376704393454982070598254930025991876_u128,189113146351872103933910686223552195131_u128];
_10 = [19148537502161563546721342493111309455_u128,145549238458005311072200915302989211969_u128,89136788686357581462321372020568537622_u128,18664145810816439710171320611333550485_u128,131642860638007680163704039383419406881_u128,143972981218950006152437920555310827863_u128,172700292823921891228809521318744130156_u128];
_2 = [4652535755816236768332862908250693013_u128,109326519055359232462401021077362215443_u128,166889115969113031034150657497838973204_u128,48614135344510494055828895631486497269_u128,259899344495917197432912454037437438131_u128,139990079340068312336362458496683615556_u128,619189405700361162328634583965499682_u128];
_7.1 = [203647405228757041351430042976700900491_u128,190452579843650803801808479002188238941_u128,175796702741146996912832096871006899248_u128,212033592808922841086291405140018958187_u128,331245178314606057648012834731994791672_u128,308558873953469977839069326085630953346_u128,231365319540110350280224418107504335386_u128];
_3.1 = [55829355427346874757731128589726353123_u128,224886303112439373631475403772889897419_u128,275746727337602688109307409651982500246_u128,213814336908922026426369643927488208089_u128,82800840206825291327521545313927991117_u128,162598503538147266627112324953519127848_u128,135756390587299600148641718488536717756_u128];
_9 = _6;
_1 = (_7,);
_13.0 = _3.0 * _3.0;
_12 = !9223372036854775807_isize;
_12 = false as isize;
_8 = _1.0.1;
_13.0 = _7.0;
_4.1 = [302523056043362864080115279435676660305_u128,71714121288375130341804296368073291462_u128,317548074928953999676746965752940970850_u128,290910863607483398223946683320831696145_u128,292825228177211706197233477803911378487_u128,26626110010664046772498266660165389438_u128,210451150030470527730808081579315760602_u128];
_7 = (_1.0.0, _9);
RET = [31586799775302444873042441300431640165_u128,55058313829195025934887854921094928619_u128,317714429337949229336452162647711325722_u128,214483870851452027903603633125283255542_u128,252624174243764499490584051231196954403_u128,264085125614136369930364862056760504787_u128,125274444935054649754448865388696666336_u128];
Goto(bb1)
}
bb1 = {
RET = _3.1;
_1.0.1 = [262920380947798800773878670863650769407_u128,334798897326342683102185279917676145483_u128,272995454604983116303618671688866185396_u128,104498864682474934837315530676698020736_u128,123374160785926227381150054762429676816_u128,104636349164125866596041461192431841156_u128,97262500630548022975816790393703796034_u128];
_13 = (_7.0, _5);
_9 = [13545566020315856926801956744560210123_u128,289116626275832453291021183036452425934_u128,2091801485738894444437709518375897657_u128,54564396174895833107507268765313508829_u128,320616046943551440821460197812693824606_u128,8018574171785585555976235691183889096_u128,100526597338141525219572896831023807476_u128];
_2 = [95890051324719139002755717139560299977_u128,240675509243282428907901098476653243272_u128,238568222604579819129519264372887113983_u128,113822164737471994979136745372812827637_u128,267152023798863899509065539481159744926_u128,195030171243574037644077191111917354482_u128,91718114736274862937580943064548851303_u128];
_8 = _7.1;
_13.0 = 4518351385077018833_i64 as u8;
_3.0 = _1.0.0;
_1.0.1 = [280120322344009565186553903838223146025_u128,339551911579697563163289105767791765664_u128,194956590827585914439467378710969504417_u128,187059497235640787568771095500471713406_u128,280792878380417910416755375008562011780_u128,5434274318629156297560569016210444169_u128,121932337171375870046836612156794160444_u128];
_4.1 = _2;
_11 = [110204028436490394979974501007344138785_i128,38273082251538006953356792287263148890_i128];
_16.0 = (_13.0, _9);
_13.0 = !_7.0;
_13.0 = _1.0.0 | _3.0;
_6 = [122787537594530174853602159736866003219_u128,257675828063216449311967145388168195231_u128,243212673012958155838881076966348888478_u128,44528320390023631358616162180192537341_u128,152132808562326708888807743034264060858_u128,43593137650527781301729846513867130642_u128,188494309487945817809047261899155743943_u128];
_7 = (_3.0, _2);
_1.0.1 = _7.1;
_3 = _1.0;
_1.0.0 = !_4.0;
_16.0.0 = _7.0 ^ _4.0;
_8 = _16.0.1;
_2 = [283412578270624253497618082160771173015_u128,324453016830952129092912844286316424222_u128,199775178931579764499689386029146549915_u128,235114018230613228445219390841005979682_u128,222646166261558666323033830467341590254_u128,320558167922175732794270222891804501627_u128,9138313008902533889504196861932987955_u128];
RET = _13.1;
_16.0.1 = _3.1;
_10 = _13.1;
_3.1 = [326121914080630831998572551894918570117_u128,315218586664277077987262619314485526807_u128,34113084056384734274058176784696754290_u128,114443226214121842534013691308051646864_u128,335881026765641649201128885987901402438_u128,79663327370889395246189798571097575709_u128,182143050368303277587218549335939999917_u128];
RET = [90993825241082970465706856909765034780_u128,257401238372298396490126063186289778269_u128,186947349244312750527652586094315237536_u128,322622506056829515299721546631070111186_u128,182409694124544569647403845385178676336_u128,66083464841833489964423637557920599822_u128,209034514905924215228649098939817740369_u128];
_7.0 = !_16.0.0;
Goto(bb2)
}
bb2 = {
_3 = (_1.0.0, _6);
_9 = [75114686217908069775278133015051109045_u128,176738359828822974257072087557370367350_u128,21560723595487303658880662425059353392_u128,45504246681493123628169844537308695378_u128,185916875778672888261202392072439582720_u128,324756394636446505141951564763524357058_u128,339481178926787329433635464993214025697_u128];
RET = _7.1;
_17 = [_12,_12,_12];
_16.0.1 = [338626032905250672441129012088393040226_u128,142047486141198596278841185182860585151_u128,33583483188743757765349992632082714454_u128,273679642996304618761708592853120343122_u128,166776078977765557953536603363360862880_u128,33237758897887775826346773842216233928_u128,340273034157987932351959032658418872088_u128];
_13 = (_3.0, _7.1);
_12 = _16.0.0 as isize;
_7.1 = [317896071646138982536061301951051194466_u128,58343033167509605137509384674198570194_u128,256247214479141893543958966865716070221_u128,183160505696717772825006106243674719742_u128,58930107805750963958819650162795061190_u128,157130302737250494851961133180004899645_u128,161663232296179508948266281449170100583_u128];
_13 = (_1.0.0, _9);
RET = [114052207211979052019774074943965057932_u128,141419781548316416645119164137993557509_u128,6809756954152284672180421204884659088_u128,139575620684005599962037182182182274405_u128,117516684809099667527280073002042194531_u128,267126599962593883532893150108548080855_u128,177860320595841541363364132014046523124_u128];
Goto(bb3)
}
bb3 = {
_16.1 = Adt24::Variant3 { fld0: true,fld1: _1.0 };
_13.1 = _7.1;
_12 = !9223372036854775807_isize;
place!(Field::<bool>(Variant(_16.1, 3), 0)) = !true;
_2 = [285501637564728265937352908231611808832_u128,95245323595173381800761591974478730054_u128,291623967117847048033225266084009425121_u128,281478745835600721781801564009755702125_u128,30065525143842793059530527502113781315_u128,81258817793437573556666541657402266950_u128,252120764838679261861759851213859199426_u128];
_3.1 = [337050672471600277578052456610210848646_u128,321231431731504540166706126379366111965_u128,148317535509435406654600053509175132112_u128,162633107619009183678221168601693584158_u128,70870891363626285800005679125497214895_u128,163672085306592610968521869447324946343_u128,210698874082835232889684064752887487470_u128];
_7.0 = _16.0.0;
_1.0.0 = (-27_i8) as u8;
_1 = (_3,);
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)) = _7;
_3 = (_16.0.0, _16.0.1);
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)) = (_13.0, _8);
_21 = ((-1751739091_i32), 122207108197066285837954035110840385218_i128);
SetDiscriminant(_16.1, 3);
_13.0 = !_3.0;
_11 = [_21.1,_21.1];
_23 = _12 as i64;
_21.0 = (-1701638968_i32);
_13 = (_3.0, _1.0.1);
_16.0 = _13;
_4.1 = [52455561231690088929876389266553515058_u128,339082774181964822455464245136609322715_u128,291099720960659973282298750608963872292_u128,223945945728511927047099682380734875097_u128,305656429431148645039509956142880268228_u128,175777034967449032438262437277424268692_u128,206718598999988247190311905566314791334_u128];
Goto(bb4)
}
bb4 = {
_16.2 = [313521663507985380748240352224726160131_u128,87139360164582018601138828826557781354_u128,323677920866035841826311708146166787998_u128,45101958715295560922593724317196613648_u128,15804801587827806668830838560331152016_u128,3559447897873389512737757211581075145_u128,258807158370763621585333396203553518351_u128];
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)).1 = [187651945439367659102428811129582118381_u128,183092131009070433018640844702177537519_u128,185492074358964261739901378124530533726_u128,229837713241164474754186511651837730126_u128,284012545597660072760347813073618150032_u128,314414947102043080039665804632897709220_u128,96778663059133026158962356608848738543_u128];
_21.0 = !162966470_i32;
_4.0 = 101652843_u32 as u8;
_20 = [_12,_12,_12,_12];
_22 = '\u{10071a}';
_22 = '\u{e7a60}';
_21.1 = (-59297251817663600203574864186459628579_i128);
_13 = (_3.0, _10);
RET = [45053085808136112463781117794837151220_u128,157886343927292982390901650334865334214_u128,273963212254038404662454163161609387910_u128,236635590783383191897313281429419865355_u128,116793482347803783814801532647839512346_u128,144922034703222826420510604684614723799_u128,238819713423038287709199196937728759167_u128];
RET = [99211968179913067429034428275923141327_u128,11435130991256787410930416535188273562_u128,337114677346475937311829297828849692178_u128,279824645578844458816762902645127198841_u128,214495385630478349596091014511706442336_u128,244191042900070423118773292900897617937_u128,59041886953132090760225500908149106718_u128];
_25 = _21.1 as isize;
_23 = !(-2746366008200472730_i64);
_16.2 = [119655936245605504913283795147580564850_u128,77116022861471443204197701370871148064_u128,226523969217185530691839999869962889872_u128,178383130898503751233012459790795809346_u128,3114404136623739274153864997207150382_u128,55824802021397564659591745818198880858_u128,274264284161993626681496723667122348122_u128];
_23 = (-5046675739312997693_i64);
_19 = _21.0 as f32;
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)).0 = _16.0.0;
_2 = [224143018217525892916495733377564162892_u128,224395374846511132782455610807774485627_u128,112368441419104827953367681250177318033_u128,202419880532581458722958018183277696213_u128,45139177251165734818797068479492830252_u128,58856180879641064321825664509484433935_u128,213487386048972632924521789507597025866_u128];
_4 = _1.0;
Call(_1 = fn6(_2, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3.0 = false as u8;
_1.0 = (_16.0.0, _6);
_19 = _25 as f32;
_13.0 = _1.0.0;
_23 = 4086268326423610099_i64;
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)).1 = [303303313887845660361609914601708266091_u128,11496142983960206771416093409807063008_u128,198297699024951242045829717391110500080_u128,5342787412786753356435418297622003273_u128,129680983153677346113397206001165011333_u128,107102156017684136561268076443108364373_u128,126425928838861224082456958120502857376_u128];
_13.1 = [117009279748683632830487700244737877686_u128,153775671685860488647982202042094477905_u128,207859288895312589734580731520743409176_u128,210437914557149488602836055776357296301_u128,124815803572976893049767372026476694662_u128,105207679602378494002655772767804046892_u128,167761477215298205638626504167341893439_u128];
_27 = Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1).0 != _13.0;
_21 = (895039556_i32, (-142886883842337858056250242425252158441_i128));
place!(Field::<bool>(Variant(_16.1, 3), 0)) = !_27;
_22 = '\u{58792}';
_11 = [_21.1,_21.1];
_13.0 = !_16.0.0;
SetDiscriminant(_16.1, 0);
_16.0.1 = [232611182432300214514534632267207588083_u128,43581992875341440990938521772732069909_u128,173997612103903198127420348019366140176_u128,160160101137427566797690805744252877705_u128,103629797763986547192578110085251111803_u128,49019375801403756977271120182322138711_u128,215472488990447857686231331041165299308_u128];
_28.1 = &_15;
_28.2 = _22;
_13.1 = [253173340916788810936598779551205454480_u128,275450987885852818399051180236475559716_u128,115129265211768902773207002215631266571_u128,94342392058672774174774744795115043521_u128,280151320444163848876762503693150917007_u128,320123444420087894425358436992124159394_u128,215538535194191390085153141648768921191_u128];
_28.2 = _22;
_12 = _25;
_16.1 = Adt24::Variant1 { fld0: 28829_i16 };
_28.0.0 = !_21.0;
_16.0 = (_13.0, _5);
_22 = _28.2;
match _21.1 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
197395483078600605407124365006516053015 => bb11,
_ => bb10
}
}
bb6 = {
_16.2 = [313521663507985380748240352224726160131_u128,87139360164582018601138828826557781354_u128,323677920866035841826311708146166787998_u128,45101958715295560922593724317196613648_u128,15804801587827806668830838560331152016_u128,3559447897873389512737757211581075145_u128,258807158370763621585333396203553518351_u128];
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)).1 = [187651945439367659102428811129582118381_u128,183092131009070433018640844702177537519_u128,185492074358964261739901378124530533726_u128,229837713241164474754186511651837730126_u128,284012545597660072760347813073618150032_u128,314414947102043080039665804632897709220_u128,96778663059133026158962356608848738543_u128];
_21.0 = !162966470_i32;
_4.0 = 101652843_u32 as u8;
_20 = [_12,_12,_12,_12];
_22 = '\u{10071a}';
_22 = '\u{e7a60}';
_21.1 = (-59297251817663600203574864186459628579_i128);
_13 = (_3.0, _10);
RET = [45053085808136112463781117794837151220_u128,157886343927292982390901650334865334214_u128,273963212254038404662454163161609387910_u128,236635590783383191897313281429419865355_u128,116793482347803783814801532647839512346_u128,144922034703222826420510604684614723799_u128,238819713423038287709199196937728759167_u128];
RET = [99211968179913067429034428275923141327_u128,11435130991256787410930416535188273562_u128,337114677346475937311829297828849692178_u128,279824645578844458816762902645127198841_u128,214495385630478349596091014511706442336_u128,244191042900070423118773292900897617937_u128,59041886953132090760225500908149106718_u128];
_25 = _21.1 as isize;
_23 = !(-2746366008200472730_i64);
_16.2 = [119655936245605504913283795147580564850_u128,77116022861471443204197701370871148064_u128,226523969217185530691839999869962889872_u128,178383130898503751233012459790795809346_u128,3114404136623739274153864997207150382_u128,55824802021397564659591745818198880858_u128,274264284161993626681496723667122348122_u128];
_23 = (-5046675739312997693_i64);
_19 = _21.0 as f32;
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)).0 = _16.0.0;
_2 = [224143018217525892916495733377564162892_u128,224395374846511132782455610807774485627_u128,112368441419104827953367681250177318033_u128,202419880532581458722958018183277696213_u128,45139177251165734818797068479492830252_u128,58856180879641064321825664509484433935_u128,213487386048972632924521789507597025866_u128];
_4 = _1.0;
Call(_1 = fn6(_2, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_16.1 = Adt24::Variant3 { fld0: true,fld1: _1.0 };
_13.1 = _7.1;
_12 = !9223372036854775807_isize;
place!(Field::<bool>(Variant(_16.1, 3), 0)) = !true;
_2 = [285501637564728265937352908231611808832_u128,95245323595173381800761591974478730054_u128,291623967117847048033225266084009425121_u128,281478745835600721781801564009755702125_u128,30065525143842793059530527502113781315_u128,81258817793437573556666541657402266950_u128,252120764838679261861759851213859199426_u128];
_3.1 = [337050672471600277578052456610210848646_u128,321231431731504540166706126379366111965_u128,148317535509435406654600053509175132112_u128,162633107619009183678221168601693584158_u128,70870891363626285800005679125497214895_u128,163672085306592610968521869447324946343_u128,210698874082835232889684064752887487470_u128];
_7.0 = _16.0.0;
_1.0.0 = (-27_i8) as u8;
_1 = (_3,);
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)) = _7;
_3 = (_16.0.0, _16.0.1);
place!(Field::<(u8, [u128; 7])>(Variant(_16.1, 3), 1)) = (_13.0, _8);
_21 = ((-1751739091_i32), 122207108197066285837954035110840385218_i128);
SetDiscriminant(_16.1, 3);
_13.0 = !_3.0;
_11 = [_21.1,_21.1];
_23 = _12 as i64;
_21.0 = (-1701638968_i32);
_13 = (_3.0, _1.0.1);
_16.0 = _13;
_4.1 = [52455561231690088929876389266553515058_u128,339082774181964822455464245136609322715_u128,291099720960659973282298750608963872292_u128,223945945728511927047099682380734875097_u128,305656429431148645039509956142880268228_u128,175777034967449032438262437277424268692_u128,206718598999988247190311905566314791334_u128];
Goto(bb4)
}
bb8 = {
_3 = (_1.0.0, _6);
_9 = [75114686217908069775278133015051109045_u128,176738359828822974257072087557370367350_u128,21560723595487303658880662425059353392_u128,45504246681493123628169844537308695378_u128,185916875778672888261202392072439582720_u128,324756394636446505141951564763524357058_u128,339481178926787329433635464993214025697_u128];
RET = _7.1;
_17 = [_12,_12,_12];
_16.0.1 = [338626032905250672441129012088393040226_u128,142047486141198596278841185182860585151_u128,33583483188743757765349992632082714454_u128,273679642996304618761708592853120343122_u128,166776078977765557953536603363360862880_u128,33237758897887775826346773842216233928_u128,340273034157987932351959032658418872088_u128];
_13 = (_3.0, _7.1);
_12 = _16.0.0 as isize;
_7.1 = [317896071646138982536061301951051194466_u128,58343033167509605137509384674198570194_u128,256247214479141893543958966865716070221_u128,183160505696717772825006106243674719742_u128,58930107805750963958819650162795061190_u128,157130302737250494851961133180004899645_u128,161663232296179508948266281449170100583_u128];
_13 = (_1.0.0, _9);
RET = [114052207211979052019774074943965057932_u128,141419781548316416645119164137993557509_u128,6809756954152284672180421204884659088_u128,139575620684005599962037182182182274405_u128,117516684809099667527280073002042194531_u128,267126599962593883532893150108548080855_u128,177860320595841541363364132014046523124_u128];
Goto(bb3)
}
bb9 = {
RET = _3.1;
_1.0.1 = [262920380947798800773878670863650769407_u128,334798897326342683102185279917676145483_u128,272995454604983116303618671688866185396_u128,104498864682474934837315530676698020736_u128,123374160785926227381150054762429676816_u128,104636349164125866596041461192431841156_u128,97262500630548022975816790393703796034_u128];
_13 = (_7.0, _5);
_9 = [13545566020315856926801956744560210123_u128,289116626275832453291021183036452425934_u128,2091801485738894444437709518375897657_u128,54564396174895833107507268765313508829_u128,320616046943551440821460197812693824606_u128,8018574171785585555976235691183889096_u128,100526597338141525219572896831023807476_u128];
_2 = [95890051324719139002755717139560299977_u128,240675509243282428907901098476653243272_u128,238568222604579819129519264372887113983_u128,113822164737471994979136745372812827637_u128,267152023798863899509065539481159744926_u128,195030171243574037644077191111917354482_u128,91718114736274862937580943064548851303_u128];
_8 = _7.1;
_13.0 = 4518351385077018833_i64 as u8;
_3.0 = _1.0.0;
_1.0.1 = [280120322344009565186553903838223146025_u128,339551911579697563163289105767791765664_u128,194956590827585914439467378710969504417_u128,187059497235640787568771095500471713406_u128,280792878380417910416755375008562011780_u128,5434274318629156297560569016210444169_u128,121932337171375870046836612156794160444_u128];
_4.1 = _2;
_11 = [110204028436490394979974501007344138785_i128,38273082251538006953356792287263148890_i128];
_16.0 = (_13.0, _9);
_13.0 = !_7.0;
_13.0 = _1.0.0 | _3.0;
_6 = [122787537594530174853602159736866003219_u128,257675828063216449311967145388168195231_u128,243212673012958155838881076966348888478_u128,44528320390023631358616162180192537341_u128,152132808562326708888807743034264060858_u128,43593137650527781301729846513867130642_u128,188494309487945817809047261899155743943_u128];
_7 = (_3.0, _2);
_1.0.1 = _7.1;
_3 = _1.0;
_1.0.0 = !_4.0;
_16.0.0 = _7.0 ^ _4.0;
_8 = _16.0.1;
_2 = [283412578270624253497618082160771173015_u128,324453016830952129092912844286316424222_u128,199775178931579764499689386029146549915_u128,235114018230613228445219390841005979682_u128,222646166261558666323033830467341590254_u128,320558167922175732794270222891804501627_u128,9138313008902533889504196861932987955_u128];
RET = _13.1;
_16.0.1 = _3.1;
_10 = _13.1;
_3.1 = [326121914080630831998572551894918570117_u128,315218586664277077987262619314485526807_u128,34113084056384734274058176784696754290_u128,114443226214121842534013691308051646864_u128,335881026765641649201128885987901402438_u128,79663327370889395246189798571097575709_u128,182143050368303277587218549335939999917_u128];
RET = [90993825241082970465706856909765034780_u128,257401238372298396490126063186289778269_u128,186947349244312750527652586094315237536_u128,322622506056829515299721546631070111186_u128,182409694124544569647403845385178676336_u128,66083464841833489964423637557920599822_u128,209034514905924215228649098939817740369_u128];
_7.0 = !_16.0.0;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_11 = [_21.1,_21.1];
_21 = (_28.0.0, (-26948308013573066710265626580243357881_i128));
_28.1 = &_15;
_1.0 = _16.0;
Goto(bb12)
}
bb12 = {
_2 = _6;
_21.1 = (-2376505706321647377344513031492818298_i128);
_4.0 = _16.0.0;
_32.1.1 = _25 as u16;
place!(Field::<i16>(Variant(_16.1, 1), 0)) = !(-6328_i16);
_16.0 = (_13.0, _7.1);
_16.0.0 = _13.0 * _13.0;
_8 = [206935279924449077761164203796345794272_u128,242055886754067900741414832032324279954_u128,39589063913732681389013791222684673401_u128,170949203449391195103538855533903158477_u128,142401325975605532220243143932780927998_u128,255514375226782467505213805660442986963_u128,240833258423743399557640625766225255906_u128];
_35 = _22;
match _23 {
0 => bb8,
1 => bb6,
4086268326423610099 => bb13,
_ => bb5
}
}
bb13 = {
_28.2 = _35;
_4.0 = _16.0.0;
_13.0 = !_1.0.0;
_1.0 = _7;
_28.0.0 = _13.0 as i32;
_38 = [_27,_27];
_16.0 = (_3.0, _2);
_16.2 = [98024654517037193633856437756950201659_u128,264220858428160035445747826107188900140_u128,11778979132448013575490912028791792173_u128,15168481413841613932679475099747390806_u128,284074507225217180574687490989599036752_u128,160180302073154210201314115528359053723_u128,202827258086963253904883196446834927657_u128];
_28.0.1 = !_21.1;
_28.0 = (_21.0, _21.1);
_38 = [_27,_27];
_30 = 147725829839737659688116925513187780580_u128 as f64;
match _28.0.1 {
0 => bb1,
1 => bb2,
2 => bb6,
337905861214616816086030094400275393158 => bb15,
_ => bb14
}
}
bb14 = {
_11 = [_21.1,_21.1];
_21 = (_28.0.0, (-26948308013573066710265626580243357881_i128));
_28.1 = &_15;
_1.0 = _16.0;
Goto(bb12)
}
bb15 = {
_38 = [_27,_27];
_13 = _3;
_28.0.0 = -_21.0;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(5_usize, 12_usize, Move(_12), 17_usize, Move(_17), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(5_usize, 35_usize, Move(_35), 22_usize, Move(_22), 38_usize, Move(_38), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(5_usize, 23_usize, Move(_23), 20_usize, Move(_20), 9_usize, Move(_9), 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [u128; 7],mut _2: (u8, [u128; 7])) -> ((u8, [u128; 7]),) {
mir! {
type RET = ((u8, [u128; 7]),);
let _3: f32;
let _4: [i128; 7];
let _5: char;
let _6: &'static [bool; 2];
let _7: ((u8, [u128; 7]), Adt24, [u128; 7]);
let _8: Adt82;
let _9: usize;
let _10: &'static &'static [i64; 1];
let _11: *mut u32;
let _12: bool;
let _13: &'static &'static [i64; 1];
let _14: isize;
let _15: i128;
let _16: isize;
let _17: ((u8, [u128; 7]), Adt24, [u128; 7]);
let _18: char;
let _19: &'static u8;
let _20: [u32; 4];
let _21: (&'static &'static [i64; 1], (Adt24, u16));
let _22: u32;
let _23: [i32; 5];
let _24: f64;
let _25: [u32; 6];
let _26: f32;
let _27: u16;
let _28: i128;
let _29: &'static *mut u32;
let _30: [u8; 8];
let _31: *const char;
let _32: [i128; 5];
let _33: &'static [i64; 1];
let _34: (u64,);
let _35: u32;
let _36: Adt18;
let _37: [u8; 8];
let _38: u16;
let _39: isize;
let _40: [i128; 7];
let _41: isize;
let _42: (&'static &'static [i64; 1], (Adt24, u16));
let _43: f32;
let _44: ();
let _45: ();
{
RET.0 = _2;
_3 = 44541_u16 as f32;
_2.1 = _1;
RET = (_2,);
_3 = 13604_u16 as f32;
_4 = [134792163306803034709643026579751007261_i128,(-47806038811691008544025121900800203958_i128),48980314597283898193679826119076403158_i128,45055133075601402649801598916607749749_i128,115086418175407647732822994524377055983_i128,(-20265633568103046980803349919001137155_i128),124148293305802641412829809040167738926_i128];
RET.0.0 = !_2.0;
RET.0 = _2;
_2.1 = RET.0.1;
_2.0 = RET.0.0;
RET = (_2,);
_5 = '\u{f10f6}';
RET.0.1 = [48672473776859439546405539470906186531_u128,13741410442856820183700077686222351974_u128,247464742186331828069038306874599196303_u128,65488640419011807861812194705531214891_u128,316102249400779054865565647519065299074_u128,60818935457650524729589392628695034772_u128,130043291996058137231590400717534480126_u128];
RET.0 = _2;
_2.1 = [124858839895818597691349891382854580406_u128,96716252200000306971595729277370514398_u128,116994217223192073542914004137557583405_u128,232722425154875512055788391267377339621_u128,55625329461486921274721726808122484213_u128,157832664349203964515290299087311705757_u128,262503035606364147450863572793475186996_u128];
_3 = RET.0.0 as f32;
RET.0 = (_2.0, _1);
RET.0.0 = !_2.0;
RET.0.1 = _2.1;
Goto(bb1)
}
bb1 = {
_2.0 = !RET.0.0;
_2.0 = _3 as u8;
_4 = [79312181516241458255091553776828362658_i128,12186286329949584989902199217032874071_i128,(-130356098327463232506992550533776874576_i128),77825973581556946829953308193761118768_i128,(-53552483447896771933337941401159982868_i128),(-119476625570929224543396560750264298981_i128),(-120347062401078447287339818917070909416_i128)];
_3 = 63275_u16 as f32;
Goto(bb2)
}
bb2 = {
_1 = [232524044695683448082985542959394995415_u128,185538196393650676355843807049554787911_u128,156129961774755581276251144144246534755_u128,69489974618751153166799167030836166528_u128,159804961734071685064491647392021726668_u128,234135435413893918109678731290296511455_u128,94361799031363332354700305804702809773_u128];
_2 = (RET.0.0, _1);
RET = (_2,);
_7.2 = RET.0.1;
RET.0.1 = _7.2;
_7.0.0 = _2.0;
_7.1 = Adt24::Variant3 { fld0: true,fld1: _2 };
_2.0 = 59349_u16 as u8;
_7.0.1 = [126538222127998628519971668517240636907_u128,294641109392026074299468880323671272103_u128,237238948722433561760553988321208136705_u128,65958399200247550020032937760696245422_u128,229922101568558794991105534933864357806_u128,238232747075936063996830653117920523444_u128,161028764724281955500932684120663494904_u128];
RET.0 = (_2.0, _1);
_7.0.1 = Field::<(u8, [u128; 7])>(Variant(_7.1, 3), 1).1;
RET = (_7.0,);
RET.0.0 = _7.0.0;
place!(Field::<bool>(Variant(_7.1, 3), 0)) = true | false;
place!(Field::<(u8, [u128; 7])>(Variant(_7.1, 3), 1)).1 = [48754691571787634097546676324390095811_u128,133850837287454211680992349817510579901_u128,126135165368573987926746903792299285233_u128,145990233595657196849012848007586028512_u128,160231587653895288756280633408664794290_u128,325664568171029825821837662415545302093_u128,92815265874222522916257412086857354580_u128];
SetDiscriminant(_7.1, 0);
_14 = (-9223372036854775808_isize);
_11 = core::ptr::addr_of_mut!(place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld2);
place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld0 = 1145524611_u32 as i32;
Goto(bb3)
}
bb3 = {
_4 = [(-113970626843153693177630411498562507721_i128),(-169268322666738532441090646457200098874_i128),16176063900510241159055945029347134846_i128,(-117614913137608652631938024648285299148_i128),(-29044812153938596461217506268596873024_i128),(-147418382664930085291932908714696539980_i128),(-161143337547995632320612587048246740534_i128)];
place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld3 = !94_i8;
_7.2 = _1;
_12 = false;
(*_11) = 766215146_u32 - 969160768_u32;
_2.1 = _7.0.1;
_7.0.0 = Field::<Adt18>(Variant(_7.1, 0), 0).fld0 as u8;
_3 = Field::<Adt18>(Variant(_7.1, 0), 0).fld2 as f32;
_3 = _7.0.0 as f32;
_11 = core::ptr::addr_of_mut!((*_11));
_3 = 4_usize as f32;
place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld0 = !17321436_i32;
place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld2 = 2675085958_u32;
_5 = '\u{45a80}';
_1 = [305740960823516353573860866682991374322_u128,189026725568477557233353967630390975031_u128,45277522383746228239078395804774173517_u128,275554515363335068188237122582325056625_u128,314922397350879479312409105434491993654_u128,213076060544689401263848087131254184946_u128,184650128802849400610310878229865117455_u128];
place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld1 = _5;
_4 = [68970140187292519839859244653607790853_i128,137000571850284633667515449644767996282_i128,(-40101944306512601337444553702522649475_i128),(-164219138479995490682148797731711462773_i128),45539859379296594231528269261264920834_i128,(-125069424057256974648503833576890543836_i128),4664989199209163962854438216124111712_i128];
_11 = core::ptr::addr_of_mut!((*_11));
place!(Field::<isize>(Variant(_7.1, 0), 1)) = !_14;
Goto(bb4)
}
bb4 = {
_11 = core::ptr::addr_of_mut!((*_11));
RET.0.0 = _3 as u8;
_9 = 6958020901233852484_i64 as usize;
_15 = _9 as i128;
_7.2 = [206160255288352135615565859002028280633_u128,9643135558286986075838565227782024535_u128,309579463681322144733568934457859925422_u128,18628392265529814233083723367313836840_u128,316856532557912791362173354496585569527_u128,276504253583955721061831735132182578482_u128,155041614104188316351173007825255264868_u128];
SetDiscriminant(_7.1, 1);
RET.0 = _7.0;
_17.0.1 = [18123564445659047883712445237502110378_u128,202659503901017821446899109807102841022_u128,330167541002842688915490510004982005359_u128,38773643863249633514678357816002006196_u128,296775520829378906553078827797589890335_u128,216192764137412120957956241371790850822_u128,200094680486653783570877904459693407431_u128];
_20 = [2556288600_u32,2778451619_u32,2724641095_u32,2508557372_u32];
_17.0 = _7.0;
_17.2 = [88849689259378465358601627195227408504_u128,118240145875979073673272155207824700733_u128,16219742303522647920139807225141513477_u128,317440573388516105464770879376036870334_u128,34758597751821938932803872295533211145_u128,186543449924890661007628529297857517928_u128,29327788188660457066666415859033279407_u128];
_7.2 = [42083960030649791825587920254854967335_u128,264771826482312116816550660778074679529_u128,124756998523816084267898895816470754479_u128,8246167850535243841106512913754057353_u128,19521614565873333530777569264048960704_u128,138823676851933602117995515238193906956_u128,140320353053685838645351710006599002291_u128];
_4 = [_15,_15,_15,_15,_15,_15,_15];
RET.0 = (_17.0.0, _7.0.1);
_4 = [_15,_15,_15,_15,_15,_15,_15];
Goto(bb5)
}
bb5 = {
_17.2 = RET.0.1;
_17.0 = (_2.0, RET.0.1);
_14 = _3 as isize;
RET.0 = (_2.0, _1);
_15 = (-58280327766953343913508830820526522153_i128) ^ 168633960579205484326578093773306204521_i128;
_19 = &_17.0.0;
place!(Field::<i16>(Variant(_7.1, 1), 0)) = !2269_i16;
_16 = _14 * _14;
_21.1.1 = 30140_u16;
_21.1.0 = Move(_7.1);
SetDiscriminant(_21.1.0, 2);
_17.1 = Adt24::Variant1 { fld0: (-22043_i16) };
_17.2 = [20425099723176924716771801010090456157_u128,194785202229126334223374982251309846432_u128,88695389639414516881423966037883414581_u128,29698088401395834170006488343471280951_u128,56127128429371969136144788329918334610_u128,269770383047300103706040085619461735609_u128,255347933123397624945500728304324768392_u128];
RET.0.1 = _2.1;
_4 = [_15,_15,_15,_15,_15,_15,_15];
RET.0.0 = 14245292_u32 as u8;
_2.1 = [293929807229152925077587186212316859354_u128,262784481257390643541373547382214148210_u128,273906389768185966517535891173893031486_u128,310325807462314334828446687710139727012_u128,243516305872016205371735421521932357762_u128,31047722155596717868933759117707632505_u128,8813919048920388396289432974879092428_u128];
RET = (_2,);
place!(Field::<char>(Variant(_21.1.0, 2), 1)) = _5;
Call(_9 = core::intrinsics::transmute(_16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_18 = Field::<char>(Variant(_21.1.0, 2), 1);
place!(Field::<u64>(Variant(_21.1.0, 2), 3)) = 14793392871583969429_u64 >> _2.0;
Call(_20 = fn7(), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = (_7.0.0, _17.0.1);
_9 = 4_usize | 4_usize;
RET.0.0 = !(*_19);
place!(Field::<bool>(Variant(_21.1.0, 2), 0)) = _12;
_7.2 = [81784194753722197360408696219831249284_u128,318559504762374390605416329548247374600_u128,107342303557282491435353776665690203771_u128,42658961855724798135986260087460525204_u128,61887174741072677637728115398471163518_u128,1154700978888162652305914817931293806_u128,99595221354079451343561686869673163617_u128];
_12 = _14 <= _14;
_24 = _3 as f64;
place!(Field::<usize>(Variant(_21.1.0, 2), 6)) = _16 as usize;
place!(Field::<u8>(Variant(_21.1.0, 2), 2)) = !_17.0.0;
place!(Field::<i16>(Variant(_21.1.0, 2), 4)) = !(-32166_i16);
_17.2 = [33873034263320415504342964263146538808_u128,103889230224944910107335165371699761486_u128,239637099301423501650064965009483931779_u128,214413032149169858622555627121423838683_u128,80431981917686003600054276232713627336_u128,275021669130431116543891020931354831974_u128,193336516273749633033530332884014652982_u128];
_29 = &_11;
_28 = _15;
RET.0 = (Field::<u8>(Variant(_21.1.0, 2), 2), _7.0.1);
_28 = _15;
_22 = 1694163604_u32;
Call(RET.0.1 = core::intrinsics::transmute(_2.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_25 = [_22,_22,_22,_22,_22,_22];
_17.1 = Adt24::Variant1 { fld0: Field::<i16>(Variant(_21.1.0, 2), 4) };
_2 = (_17.0.0, _7.2);
_23 = [431120554_i32,609073793_i32,(-272449488_i32),966171412_i32,(-1638707240_i32)];
_7 = (_2, Move(_17.1), _2.1);
place!(Field::<f64>(Variant(_21.1.0, 2), 5)) = Field::<u64>(Variant(_21.1.0, 2), 3) as f64;
_17 = Move(_7);
_17.0 = (_2.0, _2.1);
place!(Field::<i16>(Variant(_21.1.0, 2), 4)) = !Field::<i16>(Variant(_17.1, 1), 0);
_25 = [_22,_22,_22,_22,_22,_22];
_18 = Field::<char>(Variant(_21.1.0, 2), 1);
RET.0.1 = [91717238287696977956988278989382058018_u128,137829848401972485734729881557890233652_u128,128309327097509662152941961754622235282_u128,58873244254462596558376803407817457203_u128,137704990938841381081002905234588433752_u128,286406598258008864433160376555619083329_u128,18361869092836655394050293345338688593_u128];
_27 = !_21.1.1;
RET.0 = (Field::<u8>(Variant(_21.1.0, 2), 2), _17.2);
RET.0.0 = Field::<u8>(Variant(_21.1.0, 2), 2);
place!(Field::<i16>(Variant(_21.1.0, 2), 4)) = 1132234127206983325303174222524134430_u128 as i16;
_5 = _18;
RET.0.0 = _22 as u8;
_1 = [332335602890432035731949968177675722461_u128,91966700433296617133892102750918043153_u128,74399628560318261836893700808799073747_u128,330456530646102598067349199256203468106_u128,307514400986064181375802234596049372360_u128,167919841993642849846336305956931307073_u128,139107916624348335875389652224970352097_u128];
_16 = _14 | _14;
RET.0.1 = _1;
_2.0 = !_17.0.0;
_7 = Move(_17);
RET.0.0 = _2.0 ^ _2.0;
RET.0.0 = !_7.0.0;
_19 = &RET.0.0;
_21.1.1 = (-632391345_i32) as u16;
Goto(bb9)
}
bb9 = {
RET = (_2,);
place!(Field::<char>(Variant(_21.1.0, 2), 1)) = _5;
_17.0 = _7.0;
_22 = !2197774467_u32;
_11 = core::ptr::addr_of_mut!(_22);
_20 = [(*_11),(*_11),(*_11),_22];
_22 = 2475549943_u32;
SetDiscriminant(_7.1, 0);
_22 = 3830821536_u32 << _28;
_16 = _17.0.0 as isize;
_15 = (*_11) as i128;
RET.0.1 = [96778366702079923393943221463281631853_u128,153403054899895200464449238485292296115_u128,150221527249233499980875821786756997491_u128,37722346397946084776183841370612568968_u128,206538480541122792919234155075201869260_u128,272576439382343361502169415997118656709_u128,256173028053111411441484246633760604379_u128];
Goto(bb10)
}
bb10 = {
place!(Field::<Adt18>(Variant(_7.1, 0), 0)).fld3 = 3279589445272115037_i64 as i8;
(*_11) = 788222129_u32 ^ 3879691467_u32;
_32 = [_28,_15,_28,_28,_15];
_26 = _3;
_30 = [_2.0,Field::<u8>(Variant(_21.1.0, 2), 2),Field::<u8>(Variant(_21.1.0, 2), 2),Field::<u8>(Variant(_21.1.0, 2), 2),_7.0.0,_7.0.0,_2.0,Field::<u8>(Variant(_21.1.0, 2), 2)];
_2.0 = Field::<u8>(Variant(_21.1.0, 2), 2) + Field::<u8>(Variant(_21.1.0, 2), 2);
(*_11) = _28 as u32;
_7.1 = Adt24::Variant2 { fld0: Field::<bool>(Variant(_21.1.0, 2), 0),fld1: _5,fld2: _7.0.0,fld3: Field::<u64>(Variant(_21.1.0, 2), 3),fld4: Field::<i16>(Variant(_21.1.0, 2), 4),fld5: Field::<f64>(Variant(_21.1.0, 2), 5),fld6: _9 };
Call(_30 = core::intrinsics::transmute(_14), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_17.0.0 = !Field::<u8>(Variant(_21.1.0, 2), 2);
RET.0 = _17.0;
SetDiscriminant(_21.1.0, 0);
SetDiscriminant(_7.1, 3);
_35 = !(*_11);
_31 = core::ptr::addr_of!(_5);
_19 = &RET.0.0;
RET = (_7.0,);
_35 = (*_11);
(*_31) = _18;
(*_31) = _18;
Goto(bb12)
}
bb12 = {
_17.1 = Adt24::Variant3 { fld0: _12,fld1: _2 };
_21.1.0 = Move(_17.1);
_7 = (Field::<(u8, [u128; 7])>(Variant(_21.1.0, 3), 1), Move(_21.1.0), _1);
SetDiscriminant(_7.1, 2);
place!(Field::<u64>(Variant(_7.1, 2), 3)) = 7413873137426087830_u64 | 13687440528525913458_u64;
_29 = &_11;
Goto(bb13)
}
bb13 = {
_40 = [_15,_28,_15,_28,_15,_15,_15];
_34 = (Field::<u64>(Variant(_7.1, 2), 3),);
_16 = _14 | _14;
place!(Field::<bool>(Variant(_7.1, 2), 0)) = !_12;
(*_31) = _18;
_39 = _16 ^ _16;
_9 = 3_usize;
_32 = [_4[_9],_15,_28,_4[_9],_15];
_14 = _23[_9] as isize;
_42.0 = &_33;
_36.fld2 = (*_11) >> _14;
RET.0 = _2;
place!(Field::<f64>(Variant(_7.1, 2), 5)) = _24;
_11 = core::ptr::addr_of_mut!(_36.fld2);
_42.1.1 = _21.1.1;
_32 = [_28,_40[_9],_15,_4[_9],_4[_9]];
RET.0 = (_2.0, _17.0.1);
_21.0 = &_33;
Goto(bb14)
}
bb14 = {
_7.0.0 = _30[_9] & RET.0.0;
_13 = &_33;
_4[_9] = _28;
_32[_9] = (*_31) as i128;
_21.0 = &(*_13);
_14 = (-122_i8) as isize;
_1 = [_17.0.1[_9],_17.0.1[_9],_2.1[_9],_17.0.1[_9],_7.2[_9],RET.0.1[_9],_17.0.1[_9]];
_36 = Adt18 { fld0: _23[_9],fld1: _18,fld2: _22,fld3: (-77_i8) };
_13 = &_33;
_38 = _26 as u16;
_32[_9] = _40[_9] | _28;
_17.2[_9] = _9 as u128;
place!(Field::<usize>(Variant(_7.1, 2), 6)) = _12 as usize;
place!(Field::<usize>(Variant(_7.1, 2), 6)) = !_9;
_21.1.1 = _42.1.1;
place!(Field::<bool>(Variant(_7.1, 2), 0)) = _18 > (*_31);
_42.1.0 = Adt24::Variant2 { fld0: Field::<bool>(Variant(_7.1, 2), 0),fld1: _5,fld2: _17.0.0,fld3: Field::<u64>(Variant(_7.1, 2), 3),fld4: 30106_i16,fld5: Field::<f64>(Variant(_7.1, 2), 5),fld6: _9 };
_40 = [_15,_32[_9],_15,_15,_32[_9],_15,_32[_9]];
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(6_usize, 4_usize, Move(_4), 32_usize, Move(_32), 16_usize, Move(_16), 38_usize, Move(_38)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(6_usize, 9_usize, Move(_9), 22_usize, Move(_22), 34_usize, Move(_34), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(6_usize, 1_usize, Move(_1), 20_usize, Move(_20), 15_usize, Move(_15), 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7() -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _1: &'static &'static u8;
let _2: u64;
let _3: [i8; 6];
let _4: Adt24;
let _5: char;
let _6: u16;
let _7: Adt18;
let _8: bool;
let _9: [u32; 6];
let _10: u32;
let _11: (&'static &'static [i64; 1], (Adt24, u16));
let _12: i16;
let _13: [i32; 5];
let _14: i32;
let _15: f64;
let _16: f64;
let _17: i64;
let _18: &'static *mut u32;
let _19: &'static &'static *mut u32;
let _20: u64;
let _21: u32;
let _22: Adt18;
let _23: isize;
let _24: f64;
let _25: u128;
let _26: [i32; 5];
let _27: [u8; 6];
let _28: isize;
let _29: isize;
let _30: f32;
let _31: u128;
let _32: &'static &'static *mut u32;
let _33: ();
let _34: ();
{
RET = [3787103993_u32,169369494_u32,3317645788_u32,3857777661_u32];
RET = [1077862823_u32,149689612_u32,1200187579_u32,2341763317_u32];
RET = [3413519130_u32,3997864378_u32,7289905_u32,543064783_u32];
RET = [4177874461_u32,3901729336_u32,3814135567_u32,1601945185_u32];
RET = [1857495136_u32,3119588985_u32,2106972706_u32,3966900676_u32];
RET = [3624992138_u32,231282265_u32,2931811939_u32,1049151473_u32];
RET = [1221611769_u32,3330532879_u32,3185606137_u32,75000226_u32];
RET = [1303532949_u32,3375223577_u32,1214613074_u32,850109516_u32];
RET = [4011131308_u32,1975575576_u32,285718430_u32,1205379029_u32];
RET = [2409493871_u32,3845757947_u32,2655023416_u32,3261242046_u32];
RET = [4278217172_u32,2242042104_u32,814634849_u32,1453938866_u32];
RET = [2423036649_u32,1375442579_u32,1327455108_u32,1902557092_u32];
RET = [2406015903_u32,2107122219_u32,1338879444_u32,74227146_u32];
Goto(bb1)
}
bb1 = {
RET = [1081075600_u32,385143006_u32,3743447360_u32,2734102049_u32];
Goto(bb2)
}
bb2 = {
RET = [739054146_u32,983708905_u32,1708035783_u32,1698160833_u32];
RET = [2725091566_u32,166993714_u32,2394650260_u32,898405920_u32];
RET = [1715021700_u32,1429024802_u32,3298171014_u32,594022460_u32];
_3 = [(-27_i8),(-124_i8),(-62_i8),96_i8,(-59_i8),54_i8];
_6 = 33690_u16 - 62564_u16;
_7.fld2 = 721082078_u32 & 1363305534_u32;
_6 = 164313639941568375198182343191161034839_u128 as u16;
_2 = 5183426211305792391_u64;
_7.fld3 = !2_i8;
_7.fld3 = 51_i8;
_7.fld1 = '\u{21165}';
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_3 = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_5 = _7.fld1;
Goto(bb3)
}
bb3 = {
_4 = Adt24::Variant1 { fld0: (-8270_i16) };
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_10 = 71_u8 as u32;
_7.fld0 = -196589948_i32;
_5 = _7.fld1;
_2 = 137758174824448841277439948902325058106_i128 as u64;
_8 = !false;
Call(_3 = fn8(_7.fld2, _5, _10, _5, _5, _7.fld1, Move(_7), _6, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7.fld3 = -(-118_i8);
place!(Field::<i16>(Variant(_4, 1), 0)) = 28174_i16 ^ (-17838_i16);
_9 = [_10,_10,_10,_10,_10,_10];
_3 = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
place!(Field::<i16>(Variant(_4, 1), 0)) = (-22193_i16);
_7.fld3 = -(-110_i8);
SetDiscriminant(_4, 2);
_7 = Adt18 { fld0: 988614916_i32,fld1: _5,fld2: _10,fld3: 32_i8 };
_7.fld0 = (-581375737_i32);
place!(Field::<u8>(Variant(_4, 2), 2)) = 3563331949466971860_i64 as u8;
_11.1.1 = _6;
place!(Field::<f64>(Variant(_4, 2), 5)) = 2576801024464212773_usize as f64;
place!(Field::<usize>(Variant(_4, 2), 6)) = (-3566859947371024945_i64) as usize;
_6 = _11.1.1 * _11.1.1;
match _7.fld3 {
0 => bb1,
1 => bb5,
32 => bb7,
_ => bb6
}
}
bb5 = {
_4 = Adt24::Variant1 { fld0: (-8270_i16) };
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_10 = 71_u8 as u32;
_7.fld0 = -196589948_i32;
_5 = _7.fld1;
_2 = 137758174824448841277439948902325058106_i128 as u64;
_8 = !false;
Call(_3 = fn8(_7.fld2, _5, _10, _5, _5, _7.fld1, Move(_7), _6, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
RET = [739054146_u32,983708905_u32,1708035783_u32,1698160833_u32];
RET = [2725091566_u32,166993714_u32,2394650260_u32,898405920_u32];
RET = [1715021700_u32,1429024802_u32,3298171014_u32,594022460_u32];
_3 = [(-27_i8),(-124_i8),(-62_i8),96_i8,(-59_i8),54_i8];
_6 = 33690_u16 - 62564_u16;
_7.fld2 = 721082078_u32 & 1363305534_u32;
_6 = 164313639941568375198182343191161034839_u128 as u16;
_2 = 5183426211305792391_u64;
_7.fld3 = !2_i8;
_7.fld3 = 51_i8;
_7.fld1 = '\u{21165}';
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_3 = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_5 = _7.fld1;
Goto(bb3)
}
bb7 = {
place!(Field::<u8>(Variant(_4, 2), 2)) = (-4080_i16) as u8;
place!(Field::<char>(Variant(_4, 2), 1)) = _7.fld1;
_11.1.0 = Adt24::Variant1 { fld0: (-9527_i16) };
_3 = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_11.1.1 = _6;
place!(Field::<f64>(Variant(_4, 2), 5)) = _7.fld3 as f64;
_5 = Field::<char>(Variant(_4, 2), 1);
_7.fld2 = !_10;
_11.1.0 = Adt24::Variant0 { fld0: Move(_7),fld1: (-44_isize) };
_7.fld0 = Field::<Adt18>(Variant(_11.1.0, 0), 0).fld0;
match Field::<Adt18>(Variant(_11.1.0, 0), 0).fld3 {
32 => bb8,
_ => bb2
}
}
bb8 = {
place!(Field::<Adt18>(Variant(_11.1.0, 0), 0)).fld3 = (-122_i8) >> _11.1.1;
place!(Field::<u64>(Variant(_4, 2), 3)) = _2 - _2;
_6 = !_11.1.1;
_16 = Field::<f64>(Variant(_4, 2), 5) + Field::<f64>(Variant(_4, 2), 5);
_9 = [Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,_10];
place!(Field::<Adt18>(Variant(_11.1.0, 0), 0)).fld3 = (-12178_i16) as i8;
_16 = -Field::<f64>(Variant(_4, 2), 5);
place!(Field::<i16>(Variant(_4, 2), 4)) = (-14319_i16) >> _7.fld0;
_15 = -_16;
_7.fld3 = -Field::<Adt18>(Variant(_11.1.0, 0), 0).fld3;
place!(Field::<isize>(Variant(_11.1.0, 0), 1)) = (-97_isize) >> Field::<Adt18>(Variant(_11.1.0, 0), 0).fld0;
_7.fld1 = Field::<char>(Variant(_4, 2), 1);
place!(Field::<Adt18>(Variant(_11.1.0, 0), 0)) = Adt18 { fld0: _7.fld0,fld1: _7.fld1,fld2: _10,fld3: _7.fld3 };
_17 = 4359773800046276335_i64;
place!(Field::<Adt18>(Variant(_11.1.0, 0), 0)).fld2 = !_10;
_9 = [Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,_10,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,_10,_10,_10];
_4 = Adt24::Variant1 { fld0: 32091_i16 };
_14 = Field::<Adt18>(Variant(_11.1.0, 0), 0).fld0;
_7.fld2 = _7.fld1 as u32;
_13 = [_7.fld0,_14,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld0,_14,_14];
Goto(bb9)
}
bb9 = {
place!(Field::<i16>(Variant(_4, 1), 0)) = _8 as i16;
place!(Field::<Adt18>(Variant(_11.1.0, 0), 0)) = Adt18 { fld0: _14,fld1: _5,fld2: _10,fld3: _7.fld3 };
_11.1.1 = _6;
RET = [Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,_7.fld2,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2,Field::<Adt18>(Variant(_11.1.0, 0), 0).fld2];
_10 = _7.fld2;
_11.1 = (Move(_4), _6);
_12 = Field::<i16>(Variant(_11.1.0, 1), 0);
Goto(bb10)
}
bb10 = {
_7.fld0 = Field::<i16>(Variant(_11.1.0, 1), 0) as i32;
_4 = Move(_11.1.0);
_6 = _16 as u16;
_12 = Field::<i16>(Variant(_4, 1), 0);
_19 = &_18;
_4 = Adt24::Variant2 { fld0: _8,fld1: _5,fld2: 197_u8,fld3: _2,fld4: _12,fld5: _15,fld6: 5_usize };
_7.fld2 = !_10;
place!(Field::<f64>(Variant(_4, 2), 5)) = _16 - _15;
_5 = Field::<char>(Variant(_4, 2), 1);
_4 = Adt24::Variant1 { fld0: _12 };
match _14 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431186835719 => bb11,
_ => bb8
}
}
bb11 = {
SetDiscriminant(_4, 3);
place!(Field::<(u8, [u128; 7])>(Variant(_4, 3), 1)).1 = [155882038023918854729652083401796850693_u128,73751000954804554845269542836668458980_u128,318060922566745988561587456968137291924_u128,191550226730449971969289613742067995489_u128,161644258939077177577254161370002180994_u128,29196248873621627826555042299506596761_u128,4960343189025590143019265725916438826_u128];
_4 = Adt24::Variant2 { fld0: _8,fld1: _7.fld1,fld2: 157_u8,fld3: _2,fld4: _12,fld5: _15,fld6: 17798414467004141927_usize };
place!(Field::<u8>(Variant(_4, 2), 2)) = 17153223820525598000_usize as u8;
_16 = _15 - Field::<f64>(Variant(_4, 2), 5);
_7 = Adt18 { fld0: _14,fld1: _5,fld2: _10,fld3: (-10_i8) };
_12 = (-82747847271326878270635496435083763473_i128) as i16;
RET = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
place!(Field::<bool>(Variant(_4, 2), 0)) = Field::<u64>(Variant(_4, 2), 3) <= _2;
_5 = _7.fld1;
place!(Field::<bool>(Variant(_4, 2), 0)) = _8;
place!(Field::<usize>(Variant(_4, 2), 6)) = _7.fld0 as usize;
_4 = Adt24::Variant0 { fld0: Move(_7),fld1: 9223372036854775807_isize };
_14 = Field::<Adt18>(Variant(_4, 0), 0).fld0 * Field::<Adt18>(Variant(_4, 0), 0).fld0;
_22 = Adt18 { fld0: _14,fld1: _5,fld2: Field::<Adt18>(Variant(_4, 0), 0).fld2,fld3: Field::<Adt18>(Variant(_4, 0), 0).fld3 };
_20 = _2;
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld1 = _5;
_7.fld3 = !_22.fld3;
_7.fld2 = _8 as u32;
_7.fld0 = _22.fld0 * _14;
_22.fld2 = _10 | Field::<Adt18>(Variant(_4, 0), 0).fld2;
place!(Field::<Adt18>(Variant(_4, 0), 0)) = Adt18 { fld0: _7.fld0,fld1: _5,fld2: _22.fld2,fld3: _22.fld3 };
_22.fld1 = Field::<Adt18>(Variant(_4, 0), 0).fld1;
_9 = [_10,_22.fld2,_22.fld2,_10,_7.fld2,_10];
_9 = [Field::<Adt18>(Variant(_4, 0), 0).fld2,_22.fld2,_10,_22.fld2,_22.fld2,Field::<Adt18>(Variant(_4, 0), 0).fld2];
_22 = Move(Field::<Adt18>(Variant(_4, 0), 0));
_15 = _16 + _16;
Call(place!(Field::<isize>(Variant(_4, 0), 1)) = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6 = _11.1.1;
_10 = _22.fld2 & _7.fld2;
_9 = [_22.fld2,_10,_7.fld2,_22.fld2,_10,_10];
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld2 = _17 as u32;
_7.fld0 = _22.fld0 * _22.fld0;
_7.fld0 = _8 as i32;
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld1 = _22.fld1;
_4 = Adt24::Variant0 { fld0: Move(_22),fld1: (-9223372036854775808_isize) };
_6 = 2_usize as u16;
_7.fld1 = _5;
_6 = _20 as u16;
_22.fld0 = Field::<Adt18>(Variant(_4, 0), 0).fld0 * _14;
place!(Field::<Adt18>(Variant(_4, 0), 0)) = Adt18 { fld0: _22.fld0,fld1: _7.fld1,fld2: _10,fld3: _7.fld3 };
_9 = [Field::<Adt18>(Variant(_4, 0), 0).fld2,_7.fld2,_7.fld2,Field::<Adt18>(Variant(_4, 0), 0).fld2,_7.fld2,_10];
_22.fld1 = Field::<Adt18>(Variant(_4, 0), 0).fld1;
_26 = [_7.fld0,Field::<Adt18>(Variant(_4, 0), 0).fld0,_14,Field::<Adt18>(Variant(_4, 0), 0).fld0,_14];
_22.fld0 = _7.fld0;
_3 = [Field::<Adt18>(Variant(_4, 0), 0).fld3,Field::<Adt18>(Variant(_4, 0), 0).fld3,Field::<Adt18>(Variant(_4, 0), 0).fld3,Field::<Adt18>(Variant(_4, 0), 0).fld3,_7.fld3,_7.fld3];
RET = [Field::<Adt18>(Variant(_4, 0), 0).fld2,_10,_10,_10];
_15 = _16 - _16;
_8 = !false;
Goto(bb13)
}
bb13 = {
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld1 = _5;
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld3 = _7.fld3 - _7.fld3;
_2 = _8 as u64;
_6 = _11.1.1 >> _11.1.1;
_5 = _7.fld1;
_7.fld2 = !_10;
_28 = 9223372036854775807_isize;
_2 = _20 - _20;
_15 = -_16;
_24 = _15 - _16;
_21 = Field::<Adt18>(Variant(_4, 0), 0).fld2 >> _22.fld0;
_5 = _22.fld1;
_29 = _28 ^ _28;
_7.fld3 = Field::<Adt18>(Variant(_4, 0), 0).fld3;
_28 = _29 >> Field::<Adt18>(Variant(_4, 0), 0).fld3;
_24 = _15;
Call(_8 = fn10(_16, _3, _16, RET, Move(_7), _28, _24, Move(Field::<Adt18>(Variant(_4, 0), 0))), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_30 = _24 as f32;
_7 = Adt18 { fld0: _14,fld1: _5,fld2: _21,fld3: (-24_i8) };
_25 = _20 as u128;
_13 = [_7.fld0,_14,_14,_7.fld0,_14];
_22.fld2 = _2 as u32;
_6 = _11.1.1 + _11.1.1;
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld1 = _7.fld1;
place!(Field::<Adt18>(Variant(_4, 0), 0)).fld0 = _30 as i32;
_14 = _7.fld3 as i32;
_23 = _28 + _28;
_7.fld1 = Field::<Adt18>(Variant(_4, 0), 0).fld1;
_23 = _28 ^ _28;
_28 = _23 - _29;
_5 = _7.fld1;
_32 = &(*_19);
_7.fld2 = _10 * _21;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(7_usize, 28_usize, Move(_28), 9_usize, Move(_9), 25_usize, Move(_25), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(7_usize, 17_usize, Move(_17), 3_usize, Move(_3), 29_usize, Move(_29), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(7_usize, 23_usize, Move(_23), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u32,mut _2: char,mut _3: u32,mut _4: char,mut _5: char,mut _6: char,mut _7: Adt18,mut _8: u16,mut _9: u32) -> [i8; 6] {
mir! {
type RET = [i8; 6];
let _10: &'static usize;
let _11: bool;
let _12: &'static Adt47;
let _13: u64;
let _14: u128;
let _15: [isize; 3];
let _16: isize;
let _17: &'static (u8, [u128; 7]);
let _18: u64;
let _19: &'static Adt51;
let _20: *const i32;
let _21: *const char;
let _22: char;
let _23: u16;
let _24: (&'static &'static [i64; 1], (Adt24, u16));
let _25: ((i32, i128), &'static *mut u32, char, &'static &'static f32);
let _26: [isize; 4];
let _27: ();
let _28: ();
{
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_7 = Adt18 { fld0: (-371436371_i32),fld1: _5,fld2: _1,fld3: (-113_i8) };
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_7.fld3 = -82_i8;
_1 = !_7.fld2;
_7.fld3 = _7.fld0 as i8;
_5 = _6;
_7.fld3 = 32742_i16 as i8;
_1 = 1287336326502056901_i64 as u32;
Goto(bb1)
}
bb1 = {
_5 = _4;
_9 = !_3;
_2 = _4;
_4 = _5;
_9 = _7.fld2;
_6 = _2;
_8 = 24015_u16 >> _9;
_7.fld2 = _9;
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_8 = 62632332010717047825679283797762741928_i128 as u16;
_7 = Adt18 { fld0: (-1172653584_i32),fld1: _5,fld2: _9,fld3: 76_i8 };
match _7.fld3 {
0 => bb2,
1 => bb3,
76 => bb5,
_ => bb4
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
_4 = _2;
Goto(bb6)
}
bb6 = {
_4 = _2;
_5 = _4;
_7.fld2 = false as u32;
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_4 = _2;
_5 = _6;
_11 = !false;
_13 = !3410856182179241714_u64;
_7.fld1 = _4;
_8 = _13 as u16;
_9 = !_1;
_14 = _13 as u128;
_7.fld2 = _1 - _1;
_1 = _7.fld2;
_14 = !168812852668896775854043183092140351403_u128;
_3 = _7.fld2 & _9;
_6 = _7.fld1;
_7.fld2 = !_1;
_3 = _7.fld2 & _1;
Goto(bb7)
}
bb7 = {
_7.fld0 = !1158869895_i32;
_3 = _7.fld2 & _7.fld2;
_8 = 6901027372205259212_i64 as u16;
_7 = Adt18 { fld0: (-1067541345_i32),fld1: _6,fld2: _3,fld3: 94_i8 };
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_4 = _2;
_7 = Adt18 { fld0: (-1681463825_i32),fld1: _6,fld2: _9,fld3: 120_i8 };
_5 = _6;
_7.fld2 = _11 as u32;
_7.fld3 = (-84_i8);
_7.fld0 = !266597199_i32;
_15 = [(-9223372036854775808_isize),9223372036854775807_isize,(-83_isize)];
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_7.fld0 = -(-2059021454_i32);
_14 = !88296847501704237453261657405008197801_u128;
_2 = _7.fld1;
_7 = Adt18 { fld0: 1656662139_i32,fld1: _5,fld2: _9,fld3: 54_i8 };
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
Call(_4 = fn9(_15, _15, _7.fld0, _7.fld3, _7.fld3, _7.fld0, _11, _7.fld0, _8, _7.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3 = !_7.fld2;
_7.fld2 = _1;
_9 = !_7.fld2;
_2 = _4;
_16 = (-47_isize);
_18 = _13;
_7.fld1 = _5;
_9 = (-26916083628503229406764548109897993623_i128) as u32;
_7.fld1 = _5;
_20 = core::ptr::addr_of!(_7.fld0);
_7.fld3 = 44_i8 & 17_i8;
_14 = 240412429955009081604078872530273241092_u128 + 296172307373550878922680240135084701019_u128;
_20 = core::ptr::addr_of!((*_20));
_2 = _6;
(*_20) = (-1295845298_i32);
_5 = _6;
_7.fld3 = 97_i8 - 114_i8;
_16 = 107305776985147693686788018866763388900_i128 as isize;
_18 = (-1099697990208060910_i64) as u64;
_9 = _1;
_3 = !_7.fld2;
match (*_20) {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb5,
5 => bb9,
6 => bb10,
340282366920938463463374607430472366158 => bb12,
_ => bb11
}
}
bb9 = {
_7.fld0 = !1158869895_i32;
_3 = _7.fld2 & _7.fld2;
_8 = 6901027372205259212_i64 as u16;
_7 = Adt18 { fld0: (-1067541345_i32),fld1: _6,fld2: _3,fld3: 94_i8 };
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_4 = _2;
_7 = Adt18 { fld0: (-1681463825_i32),fld1: _6,fld2: _9,fld3: 120_i8 };
_5 = _6;
_7.fld2 = _11 as u32;
_7.fld3 = (-84_i8);
_7.fld0 = !266597199_i32;
_15 = [(-9223372036854775808_isize),9223372036854775807_isize,(-83_isize)];
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_7.fld0 = -(-2059021454_i32);
_14 = !88296847501704237453261657405008197801_u128;
_2 = _7.fld1;
_7 = Adt18 { fld0: 1656662139_i32,fld1: _5,fld2: _9,fld3: 54_i8 };
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
Call(_4 = fn9(_15, _15, _7.fld0, _7.fld3, _7.fld3, _7.fld0, _11, _7.fld0, _8, _7.fld0), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_3 = (-2275291797340853293_i64) as u32;
_7.fld2 = !_1;
_21 = core::ptr::addr_of!(_2);
(*_20) = !986090164_i32;
_14 = 75870056618619324953895163635301549792_u128 & 294351159964255682968876274516576287201_u128;
_23 = _8 - _8;
_1 = _9 | _7.fld2;
_23 = !_8;
_4 = (*_21);
_9 = _1 ^ _7.fld2;
(*_20) = _6 as i32;
_7.fld3 = (-20_i8) | (-11_i8);
_7.fld2 = _9;
_7 = Adt18 { fld0: (-1874695549_i32),fld1: _6,fld2: _1,fld3: 121_i8 };
_14 = 261487270178324802484448994296987801237_u128;
_7.fld3 = 89_i8 - 14_i8;
_18 = _16 as u64;
_2 = _5;
_25.0 = ((*_20), (-3161777338697316481227627403415075650_i128));
_2 = _5;
_24.1.1 = _8 | _8;
_7.fld0 = _25.0.0 ^ _25.0.0;
_25.0.0 = -_7.fld0;
_25.0 = (_7.fld0, (-153238314501850862356416436311603243780_i128));
match _25.0.1 {
0 => bb7,
187044052419087601106958171120164967676 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_22 = _4;
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
RET = [_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3,_7.fld3];
_7 = Adt18 { fld0: _25.0.0,fld1: _5,fld2: _1,fld3: 84_i8 };
_21 = core::ptr::addr_of!((*_21));
_4 = (*_21);
_9 = _1;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(8_usize, 4_usize, Move(_4), 14_usize, Move(_14), 3_usize, Move(_3), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(8_usize, 16_usize, Move(_16), 13_usize, Move(_13), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [isize; 3],mut _2: [isize; 3],mut _3: i32,mut _4: i8,mut _5: i8,mut _6: i32,mut _7: bool,mut _8: i32,mut _9: u16,mut _10: i32) -> char {
mir! {
type RET = char;
let _11: ((u8, [u128; 7]), Adt24, [u128; 7]);
let _12: (&'static &'static [i64; 1], (Adt24, u16));
let _13: i8;
let _14: (((u8, [u128; 7]),), [i128; 5], &'static &'static f32);
let _15: &'static *mut u32;
let _16: isize;
let _17: u8;
let _18: isize;
let _19: &'static u8;
let _20: f32;
let _21: (((u8, [u128; 7]),), [i128; 5], &'static &'static f32);
let _22: f32;
let _23: [u32; 4];
let _24: Adt41;
let _25: &'static [i64; 1];
let _26: [i64; 5];
let _27: char;
let _28: i16;
let _29: Adt24;
let _30: [u8; 8];
let _31: ([i32; 5], [u16; 8]);
let _32: [u32; 6];
let _33: [i8; 6];
let _34: ();
let _35: ();
{
RET = '\u{105713}';
_7 = false;
_5 = 117640966_u32 as i8;
_11.0.1 = [87500858732782706004398733451696773458_u128,133397209911798213402661723557093070089_u128,192269178355905610049536884092033745757_u128,337008719962305133811853705240041819228_u128,218051313045953788127521809863663204310_u128,193285667619813499111277819842755704685_u128,155704897669561838939391869433083013402_u128];
_11.2 = [171546719505320003046961979042342676100_u128,12531345340787111226949763019648175707_u128,145937998197991195192337580342386724587_u128,108133229646148150334027688185953087805_u128,4101004053173434997185196590142100062_u128,166507390136156866263083826412185253372_u128,261445628218867292749429467886668247043_u128];
RET = '\u{835b7}';
_11.0.0 = !239_u8;
RET = '\u{5116a}';
RET = '\u{52c72}';
_14.0.0 = (_11.0.0, _11.2);
_12.1.1 = !_9;
_14.0.0 = _11.0;
_14.1 = [(-18667297317937779653107009777925223834_i128),84073072765423393466805997837909195020_i128,167238230617040789475251724931325605327_i128,157948621102677281541574434708277839132_i128,(-10121792034718225304513549673553345417_i128)];
_8 = _3;
_11.0 = _14.0.0;
_10 = -_3;
_3 = -_8;
_2 = [(-9223372036854775808_isize),(-61_isize),9223372036854775807_isize];
_2 = [17_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_14.0.0.0 = !_11.0.0;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1656662139 => bb9,
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
_6 = _8;
_2 = [(-4_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_11.0.1 = _11.2;
_3 = -_8;
_12.1.0 = Adt24::Variant3 { fld0: _7,fld1: _14.0.0 };
_11 = (_14.0.0, Move(_12.1.0), _14.0.0.1);
place!(Field::<(u8, [u128; 7])>(Variant(_11.1, 3), 1)).0 = 0_usize as u8;
_11.0.1 = [189248874059794600216378536390965375559_u128,242013073677574938183412138517703318124_u128,147527375151038292913160907871001911016_u128,177305694603992262605819230080579663439_u128,147942222866063980765508770380103280467_u128,293773701396310618681575173959243536482_u128,297396483873110909133849945707563891530_u128];
_10 = _4 as i32;
RET = '\u{e176}';
_18 = -9223372036854775807_isize;
RET = '\u{61cf}';
_6 = _8 << _9;
place!(Field::<bool>(Variant(_11.1, 3), 0)) = _7 ^ _7;
Goto(bb10)
}
bb10 = {
_11.0.1 = [77539926813509301025518539368966331159_u128,143920592807139712272819471207975920502_u128,237281180923290267286160512950522868756_u128,250673274104598002804406326500141118375_u128,189610984396311063102804490679172439025_u128,21454859391790594941710021227171455698_u128,96192744760293170597307314538858626796_u128];
_14.0.0 = (_11.0.0, Field::<(u8, [u128; 7])>(Variant(_11.1, 3), 1).1);
_12.1.0 = Move(_11.1);
_21.0.0 = _11.0;
match _8 {
0 => bb1,
1656662139 => bb11,
_ => bb2
}
}
bb11 = {
_11.0 = _14.0.0;
_10 = _3 & _3;
_14.0 = (Field::<(u8, [u128; 7])>(Variant(_12.1.0, 3), 1),);
_7 = Field::<bool>(Variant(_12.1.0, 3), 0);
_12.1.1 = _9;
_21.1 = _14.1;
place!(Field::<(u8, [u128; 7])>(Variant(_12.1.0, 3), 1)) = (_14.0.0.0, _14.0.0.1);
_9 = !_12.1.1;
_14.1 = _21.1;
_11.0 = (_14.0.0.0, _14.0.0.1);
_13 = -_5;
RET = '\u{89173}';
_24.fld7 = Field::<bool>(Variant(_12.1.0, 3), 0) as u128;
_14.0 = (_11.0,);
_24.fld1 = [_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7];
_14.0.0 = (_11.0.0, _11.0.1);
place!(Field::<(u8, [u128; 7])>(Variant(_12.1.0, 3), 1)).0 = !_11.0.0;
_14.0.0.1 = [_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7];
_23 = [2495384388_u32,1731363115_u32,14385250_u32,832475996_u32];
SetDiscriminant(_12.1.0, 1);
match _8 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb7,
5 => bb6,
1656662139 => bb13,
_ => bb12
}
}
bb12 = {
_11.0.1 = [77539926813509301025518539368966331159_u128,143920592807139712272819471207975920502_u128,237281180923290267286160512950522868756_u128,250673274104598002804406326500141118375_u128,189610984396311063102804490679172439025_u128,21454859391790594941710021227171455698_u128,96192744760293170597307314538858626796_u128];
_14.0.0 = (_11.0.0, Field::<(u8, [u128; 7])>(Variant(_11.1, 3), 1).1);
_12.1.0 = Move(_11.1);
_21.0.0 = _11.0;
match _8 {
0 => bb1,
1656662139 => bb11,
_ => bb2
}
}
bb13 = {
_19 = &_14.0.0.0;
_24.fld4.0 = _11.0;
_22 = 4_usize as f32;
_24.fld3 = _24.fld7 as i8;
_27 = RET;
_12.1.1 = !_9;
_21.0.0.1 = [_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7];
_1 = _2;
_12.1.0 = Adt24::Variant1 { fld0: 12501_i16 };
_13 = 70694636380886796442876226008118258457_i128 as i8;
_16 = _18 | _18;
_20 = 2411390292_u32 as f32;
_3 = _9 as i32;
_8 = -_6;
_14.0 = _24.fld4;
_7 = !false;
Goto(bb14)
}
bb14 = {
_20 = 8283589890464109711_i64 as f32;
_19 = &_17;
_5 = _12.1.1 as i8;
_6 = _8 << _14.0.0.0;
_9 = _12.1.1;
_31.1 = [_12.1.1,_9,_12.1.1,_12.1.1,_12.1.1,_9,_9,_9];
_9 = !_12.1.1;
_11.2 = [_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7,_24.fld7];
_11.0 = (_21.0.0.0, _14.0.0.1);
_22 = -_20;
_8 = _10 & _6;
_29 = Adt24::Variant1 { fld0: (-11473_i16) };
_11.0 = (_21.0.0.0, _14.0.0.1);
_5 = _4 >> _24.fld3;
_7 = !true;
_12.0 = &_25;
_7 = true;
_24.fld3 = _5;
_11.0.1 = _24.fld4.0.1;
_14.1 = _21.1;
_26 = [(-1554369094809989687_i64),6685324262111641971_i64,6329105843461193429_i64,(-3384860633015082351_i64),9163435730438188598_i64];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(9_usize, 6_usize, Move(_6), 10_usize, Move(_10), 27_usize, Move(_27), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(9_usize, 4_usize, Move(_4), 8_usize, Move(_8), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: f64,mut _2: [i8; 6],mut _3: f64,mut _4: [u32; 4],mut _5: Adt18,mut _6: isize,mut _7: f64,mut _8: Adt18) -> bool {
mir! {
type RET = bool;
let _9: isize;
let _10: ((u8, [u128; 7]),);
let _11: f32;
let _12: char;
let _13: Adt70;
let _14: ();
let _15: ();
{
_2 = [_5.fld3,_8.fld3,_5.fld3,_8.fld3,_5.fld3,_8.fld3];
RET = true;
_1 = -_7;
_5 = Adt18 { fld0: _8.fld0,fld1: _8.fld1,fld2: _8.fld2,fld3: _8.fld3 };
RET = !false;
Call(_5.fld2 = fn11(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.fld2 = _5.fld2 ^ _5.fld2;
_8.fld0 = _5.fld0;
RET = !false;
RET = _8.fld2 == _8.fld2;
_3 = _7;
_5 = Move(_8);
_10.0.1 = [200180708189686518591750449214698792299_u128,49447778642094079232419724495522660224_u128,5293477655347122702622492567126784004_u128,62362003087620149487416029568658590837_u128,5664885178798750142979075288615170449_u128,147000336776614627866368892518387258391_u128,288393495684396485598705430475562339922_u128];
_5.fld3 = !(-18_i8);
_11 = _5.fld0 as f32;
_8 = Adt18 { fld0: _5.fld0,fld1: _5.fld1,fld2: _5.fld2,fld3: _5.fld3 };
_5.fld0 = _8.fld0 >> _8.fld0;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(10_usize, 6_usize, Move(_6), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11() -> u32 {
mir! {
type RET = u32;
let _1: (u64,);
let _2: &'static ([i128; 2], (Adt24, u16));
let _3: bool;
let _4: Adt41;
let _5: Adt41;
let _6: &'static isize;
let _7: bool;
let _8: char;
let _9: u32;
let _10: Adt53;
let _11: [isize; 3];
let _12: &'static (*mut u32, (((i32, i128), *const i32), i16, *mut u32, (i32, i128)));
let _13: &'static [i64; 1];
let _14: *const i32;
let _15: char;
let _16: &'static u8;
let _17: i8;
let _18: ((u8, [u128; 7]), Adt24, [u128; 7]);
let _19: &'static usize;
let _20: [u128; 7];
let _21: (&'static &'static [i64; 1], (Adt24, u16));
let _22: ((i32, i128), &'static *mut u32, char, &'static &'static f32);
let _23: ();
let _24: ();
{
RET = 60595753_u32 * 2066870104_u32;
RET = 3460570210_u32 & 3307279594_u32;
RET = 2765039847_u32;
RET = 2200935394_u32;
RET = 3621805545_u32 + 2166324562_u32;
Goto(bb1)
}
bb1 = {
RET = (-16_isize) as u32;
_1 = (18408355830112844322_u64,);
_1.0 = (-1135771711_i32) as u64;
_1.0 = 8223656593372852118_u64;
_3 = !true;
_3 = true;
_1.0 = 15397130012371761373_u64 & 6338480295169390015_u64;
RET = !4259534469_u32;
_3 = false;
_1 = (6468369307447036162_u64,);
RET = 497246034_u32;
RET = 414041303_u32 | 1704473386_u32;
RET = 2362703881_u32 & 156034124_u32;
_4.fld7 = 243936220295627475906247970181247581854_u128 << _1.0;
_5.fld4.0.0 = !29_u8;
RET = !999663544_u32;
_4.fld5 = !(-1245902303_i32);
_5.fld0 = RET as f32;
_3 = true;
_4.fld5 = (-750373059_i32);
_5.fld5 = _4.fld5;
match _5.fld5 {
0 => bb2,
340282366920938463463374607431017838397 => bb4,
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
_4.fld4.0.0 = _5.fld4.0.0;
_5.fld3 = (-45_i8);
_8 = '\u{20dfe}';
RET = 895_u16 as u32;
_5.fld4.0.1 = [_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7];
_8 = '\u{86042}';
_7 = _3;
_4.fld0 = -_5.fld0;
_4.fld0 = -_5.fld0;
_5.fld2 = [RET,RET,RET,RET,RET,RET];
_9 = !RET;
RET = (-3508_i16) as u32;
RET = _9;
_11 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_4.fld4.0.1 = _5.fld4.0.1;
RET = _9 | _9;
_4.fld1 = [_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7];
_5.fld3 = (-20_i8) >> _5.fld5;
_5.fld3 = (-17_i8);
_4.fld3 = _5.fld3 << _4.fld7;
_4.fld4 = (_5.fld4.0,);
match _4.fld5 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431017838397 => bb9,
_ => bb8
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
RET = (-16_isize) as u32;
_1 = (18408355830112844322_u64,);
_1.0 = (-1135771711_i32) as u64;
_1.0 = 8223656593372852118_u64;
_3 = !true;
_3 = true;
_1.0 = 15397130012371761373_u64 & 6338480295169390015_u64;
RET = !4259534469_u32;
_3 = false;
_1 = (6468369307447036162_u64,);
RET = 497246034_u32;
RET = 414041303_u32 | 1704473386_u32;
RET = 2362703881_u32 & 156034124_u32;
_4.fld7 = 243936220295627475906247970181247581854_u128 << _1.0;
_5.fld4.0.0 = !29_u8;
RET = !999663544_u32;
_4.fld5 = !(-1245902303_i32);
_5.fld0 = RET as f32;
_3 = true;
_4.fld5 = (-750373059_i32);
_5.fld5 = _4.fld5;
match _5.fld5 {
0 => bb2,
340282366920938463463374607431017838397 => bb4,
_ => bb3
}
}
bb8 = {
Return()
}
bb9 = {
_1 = (7148483983694184703_u64,);
_4.fld0 = 5274_u16 as f32;
_11 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-78_isize)];
_4.fld6 = [56303636212680998169551738875600719310_i128,23793400959683911653474686410219914789_i128,153745180080186691223083495797212627227_i128,(-147304687627241697897128349862009030200_i128),(-48004469320077303066802161082411103783_i128)];
_5.fld3 = _4.fld3 ^ _4.fld3;
_4.fld2 = [RET,RET,RET,RET,_9,RET];
_4.fld1 = _5.fld4.0.1;
_15 = _8;
_4.fld4.0.1 = [_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7];
_5.fld7 = !_4.fld7;
_16 = &_4.fld4.0.0;
_8 = _15;
_9 = RET - RET;
_4.fld3 = 25542_u16 as i8;
_4.fld3 = -_5.fld3;
Goto(bb10)
}
bb10 = {
_4.fld3 = -_5.fld3;
_4.fld7 = _5.fld7 + _5.fld7;
_3 = _4.fld3 != _4.fld3;
_1.0 = !6676387783925298009_u64;
_5.fld5 = _4.fld5 * _4.fld5;
_14 = core::ptr::addr_of!(_5.fld5);
_18.0.0 = (*_16) | _5.fld4.0.0;
_4.fld0 = -_5.fld0;
_8 = _15;
_5.fld4.0.0 = _4.fld4.0.0 - (*_16);
_5.fld4.0.1 = [_5.fld7,_4.fld7,_5.fld7,_4.fld7,_5.fld7,_4.fld7,_4.fld7];
Goto(bb11)
}
bb11 = {
_3 = _7 | _7;
_1 = (7075057351682032340_u64,);
_18.0 = _4.fld4.0;
_18.0.0 = _1.0 as u8;
_5.fld3 = _4.fld3;
(*_14) = _4.fld5 * _4.fld5;
_5.fld6 = [68246728814718892447070920818349455289_i128,87082405396076749730049644189271364659_i128,128378853647811122399177013632203015030_i128,94847876451441216276491420129596211963_i128,(-162808118769671801312381427215049459450_i128)];
_1.0 = 283029706007855382_usize as u64;
_3 = _5.fld3 != _5.fld3;
RET = !_9;
_18.2 = [_4.fld7,_4.fld7,_4.fld7,_4.fld7,_5.fld7,_4.fld7,_4.fld7];
_18.1 = Adt24::Variant3 { fld0: _3,fld1: _5.fld4.0 };
_18.2 = _18.0.1;
_5.fld1 = [_5.fld7,_5.fld7,_5.fld7,_4.fld7,_4.fld7,_5.fld7,_4.fld7];
_5.fld4.0.0 = 212560312651679601_usize as u8;
_5 = Adt41 { fld0: _4.fld0,fld1: Field::<(u8, [u128; 7])>(Variant(_18.1, 3), 1).1,fld2: _4.fld2,fld3: _4.fld3,fld4: _4.fld4,fld5: _4.fld5,fld6: _4.fld6,fld7: _4.fld7 };
_5.fld4.0 = ((*_16), _4.fld1);
place!(Field::<(u8, [u128; 7])>(Variant(_18.1, 3), 1)) = ((*_16), _18.2);
_15 = _8;
place!(Field::<(u8, [u128; 7])>(Variant(_18.1, 3), 1)).0 = (-90906310975556300387883510984642419745_i128) as u8;
_4.fld1 = _5.fld1;
RET = _9 + _9;
_5.fld2 = [_9,_9,RET,RET,RET,_9];
match _5.fld5 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
340282366920938463463374607431017838397 => bb18,
_ => bb17
}
}
bb12 = {
_4.fld3 = -_5.fld3;
_4.fld7 = _5.fld7 + _5.fld7;
_3 = _4.fld3 != _4.fld3;
_1.0 = !6676387783925298009_u64;
_5.fld5 = _4.fld5 * _4.fld5;
_14 = core::ptr::addr_of!(_5.fld5);
_18.0.0 = (*_16) | _5.fld4.0.0;
_4.fld0 = -_5.fld0;
_8 = _15;
_5.fld4.0.0 = _4.fld4.0.0 - (*_16);
_5.fld4.0.1 = [_5.fld7,_4.fld7,_5.fld7,_4.fld7,_5.fld7,_4.fld7,_4.fld7];
Goto(bb11)
}
bb13 = {
_1 = (7148483983694184703_u64,);
_4.fld0 = 5274_u16 as f32;
_11 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-78_isize)];
_4.fld6 = [56303636212680998169551738875600719310_i128,23793400959683911653474686410219914789_i128,153745180080186691223083495797212627227_i128,(-147304687627241697897128349862009030200_i128),(-48004469320077303066802161082411103783_i128)];
_5.fld3 = _4.fld3 ^ _4.fld3;
_4.fld2 = [RET,RET,RET,RET,_9,RET];
_4.fld1 = _5.fld4.0.1;
_15 = _8;
_4.fld4.0.1 = [_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7];
_5.fld7 = !_4.fld7;
_16 = &_4.fld4.0.0;
_8 = _15;
_9 = RET - RET;
_4.fld3 = 25542_u16 as i8;
_4.fld3 = -_5.fld3;
Goto(bb10)
}
bb14 = {
Return()
}
bb15 = {
RET = (-16_isize) as u32;
_1 = (18408355830112844322_u64,);
_1.0 = (-1135771711_i32) as u64;
_1.0 = 8223656593372852118_u64;
_3 = !true;
_3 = true;
_1.0 = 15397130012371761373_u64 & 6338480295169390015_u64;
RET = !4259534469_u32;
_3 = false;
_1 = (6468369307447036162_u64,);
RET = 497246034_u32;
RET = 414041303_u32 | 1704473386_u32;
RET = 2362703881_u32 & 156034124_u32;
_4.fld7 = 243936220295627475906247970181247581854_u128 << _1.0;
_5.fld4.0.0 = !29_u8;
RET = !999663544_u32;
_4.fld5 = !(-1245902303_i32);
_5.fld0 = RET as f32;
_3 = true;
_4.fld5 = (-750373059_i32);
_5.fld5 = _4.fld5;
match _5.fld5 {
0 => bb2,
340282366920938463463374607431017838397 => bb4,
_ => bb3
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_1.0 = !2427882556037727694_u64;
_5.fld0 = (-2293_i16) as f32;
_17 = _5.fld3;
_4.fld0 = _5.fld0;
_21.1.1 = !15268_u16;
_5 = Adt41 { fld0: _4.fld0,fld1: Field::<(u8, [u128; 7])>(Variant(_18.1, 3), 1).1,fld2: _4.fld2,fld3: _4.fld3,fld4: _4.fld4,fld5: _4.fld5,fld6: _4.fld6,fld7: _4.fld7 };
_8 = _15;
_4.fld6 = _5.fld6;
place!(Field::<bool>(Variant(_18.1, 3), 0)) = _3;
_5.fld2 = [RET,_9,RET,_9,RET,RET];
_4.fld4 = (_5.fld4.0,);
_21.0 = &_13;
_4.fld4.0.1 = [_4.fld7,_5.fld7,_4.fld7,_4.fld7,_4.fld7,_4.fld7,_5.fld7];
_21.0 = &_13;
_21.1.0 = Move(_18.1);
RET = !_9;
Goto(bb19)
}
bb19 = {
Call(_23 = dump_var(11_usize, 7_usize, Move(_7), 11_usize, Move(_11), 17_usize, Move(_17), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [u128; 7],mut _2: [u128; 7]) -> (u8, [u128; 7]) {
mir! {
type RET = (u8, [u128; 7]);
let _3: [u8; 8];
let _4: (((i32, i128), *const i32), i16, *mut u32, (i32, i128));
let _5: Adt47;
let _6: [i8; 6];
let _7: *const i32;
let _8: &'static &'static u8;
let _9: Adt53;
let _10: &'static &'static u8;
let _11: *const i32;
let _12: &'static *mut u32;
let _13: ([i128; 2], &'static isize, [i128; 7], (((u8, [u128; 7]),), [i128; 5], &'static &'static f32));
let _14: Adt53;
let _15: f32;
let _16: isize;
let _17: i64;
let _18: &'static *mut u32;
let _19: [u8; 6];
let _20: (u64,);
let _21: i128;
let _22: ([i128; 2], (Adt24, u16));
let _23: &'static &'static &'static [i64; 1];
let _24: f32;
let _25: char;
let _26: isize;
let _27: char;
let _28: usize;
let _29: &'static &'static *mut u32;
let _30: bool;
let _31: ();
let _32: ();
{
RET.1 = [192301037895935977493742028919951564669_u128,121323936639308528355723018942185868493_u128,110884327190395691087145257134431831586_u128,44289011511637659338832092357294320366_u128,18595211791277292443700120364290531279_u128,320825012691052068448356712151336077774_u128,97222702644037457066157536079907261401_u128];
RET = (71_u8, _1);
_1 = [65790935537109765813185451530168720904_u128,172026136372461991108494653137249739882_u128,215512567467328632536571900431038359391_u128,318660791360212557870711135040157081603_u128,146379774584711292470903950708141694406_u128,209844926052653335918694015979407586013_u128,29955176262017259733170443138313767047_u128];
RET = (190_u8, _2);
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.0 = 115_u8 + 87_u8;
RET = (114_u8, _1);
RET.1 = [249954649219059958392114576468364745430_u128,242354038347431272912963008140166818291_u128,18107844665859795279843803931048212417_u128,198356299862221127224456721702204693992_u128,264183589597337804964895415379406925988_u128,67868623622537674040830300530083023809_u128,302857995003891097151455925504904494544_u128];
RET = (166_u8, _1);
RET.0 = 245_u8 << (-1483759130_i32);
RET.0 = !233_u8;
RET.0 = 153807049019787867996614570373581488262_u128 as u8;
RET = (195_u8, _1);
RET.0 = 104_u8 | 122_u8;
RET.1 = [81587872813408443155346623044371122652_u128,167045622125232117848996371224043107398_u128,59475198627061389953716901401688164943_u128,17397706095624424011480972668916661438_u128,86860943415505634662114428873850501843_u128,11459254764251133269774768703091192668_u128,317037856622516356796479051467044803336_u128];
RET.0 = !167_u8;
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
RET.1 = [255776511021479160103777660590016339776_u128,319967540939812837006273191102753135852_u128,62858562340958757022169515466449746081_u128,38094843259180057042983991095054128176_u128,170337151063929522775660356220016441654_u128,18963339026316844799483103553263460357_u128,306437444436214025954939510742596579193_u128];
RET.0 = 196_u8 << 20011_i16;
RET.0 = 46_u8 + 232_u8;
_1 = _2;
RET = (35_u8, _2);
_4.0.0.0 = !(-671556928_i32);
RET.0 = 90_u8 >> _4.0.0.0;
_4.3.0 = -_4.0.0.0;
Goto(bb1)
}
bb1 = {
_4.1 = (-11031_i16);
_4.0.0.1 = -(-59163572096044188479048543810253953685_i128);
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_4.1 = (-27295_i16) ^ 29320_i16;
_4.1 = (-15743_i16) | 16152_i16;
RET.1 = [331475901501615158811564165909946177551_u128,124929833940651017846957589660410984173_u128,115505968415843853243669602779034986441_u128,170308057789679558626394998948153576344_u128,98263969906273251296581566231312702992_u128,278786006671746821700720545585263447745_u128,162666930719534563375245179616520559103_u128];
_4.0.0 = (_4.3.0, (-39755704545159054068169201779892516986_i128));
_4.0.0.0 = _4.0.0.1 as i32;
_4.0.0.1 = 9223372036854775807_isize as i128;
RET = (116_u8, _2);
_4.0.1 = core::ptr::addr_of!(_4.0.0.0);
_4.0.0 = (_4.3.0, 146904894921718203392678044254028908072_i128);
_7 = Move(_4.0.1);
_6 = [(-1_i8),90_i8,(-29_i8),(-68_i8),73_i8,50_i8];
_4.0.1 = Move(_7);
_4.3.1 = _4.0.0.1;
_4.3 = _4.0.0;
_7 = core::ptr::addr_of!(_4.0.0.0);
_4.1 = !(-6917_i16);
_4.3.1 = _4.0.0.1;
_4.0 = (_4.3, Move(_7));
RET.1 = [318141434734551403023212997313787060267_u128,76954513838564871331694144125404159638_u128,185420913233207514833142366357169949740_u128,23641032790247047951349004471645910949_u128,146216622846365223450329251010694627165_u128,180938612958057782112360154313997278646_u128,282642639073389101638356087074978051252_u128];
RET.0 = 86_u8 * 229_u8;
Goto(bb2)
}
bb2 = {
_4.1 = 525713491358128722_u64 as i16;
_4.3.1 = !_4.0.0.1;
_4.0.0 = (_4.3.0, _4.3.1);
_4.0.0 = _4.3;
_4.0.0.1 = _4.3.1 - _4.3.1;
_4.1 = (-26537_i16) + 12002_i16;
_2 = [7646508970762724902213155496332056611_u128,69791185526103508462189050140561302247_u128,107438947115972369934425453449017057047_u128,100174690587303073262526392141247938275_u128,194293731479165186101210830522863294366_u128,130941598934677934696268068430912120626_u128,175384794446900565123254283113053142718_u128];
RET.0 = 196_u8;
_4.0.1 = core::ptr::addr_of!(_4.3.0);
_4.1 = 43917_u16 as i16;
RET.1 = _1;
_2 = _1;
_4.1 = 7122_i16 + 9661_i16;
_4.0.0.0 = !_4.3.0;
RET.1 = [68982894874436509693428400683068749795_u128,17793717319257331966989754159150290198_u128,56213541840904905839971842496081282296_u128,102633678203458319969893586294532741103_u128,338978996815933312033373873391824923941_u128,292602830416748986655341738595909362229_u128,3736896324116441255796373598721844121_u128];
_4.1 = -23960_i16;
_4.3 = (_4.0.0.0, _4.0.0.1);
_4.0.0.1 = _4.3.1;
RET.1 = [13746534979404234555228899418309480944_u128,172334997244402808196627410332860953926_u128,129935502243686008885819161330455018689_u128,125295002943797248912777622176859948493_u128,197611190118188261144921607004588371052_u128,107701098834532696590103804365380297163_u128,33456244145101565682844577181488711032_u128];
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2 = RET.1;
_4.3.1 = _4.0.0.1;
_1 = RET.1;
_7 = Move(_4.0.1);
_4.0.1 = Move(_7);
RET.1 = _2;
RET = (107_u8, _1);
Goto(bb3)
}
bb3 = {
_4.0.1 = core::ptr::addr_of!(_4.3.0);
_1 = [181054513251305039275166385585661855453_u128,248623208779067767164026136906174324899_u128,278239013391573482698298666531446032924_u128,85636768014485189160508452197833683203_u128,150780928501390953388939775567827910801_u128,339702736948176610084992045608161931028_u128,173803048529565060604185524612100148044_u128];
Goto(bb4)
}
bb4 = {
_2 = [22285324716350887989255063810272500984_u128,33513510566431655378291773755640836021_u128,184968685272264490794691270696987917940_u128,222561232314789778111357305136526569800_u128,116315532566660903214802796572275477782_u128,87238681843157734233556772510686469395_u128,38439184991660609176657340140522594182_u128];
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
107 => bb9,
_ => bb8
}
}
bb5 = {
_4.0.1 = core::ptr::addr_of!(_4.3.0);
_1 = [181054513251305039275166385585661855453_u128,248623208779067767164026136906174324899_u128,278239013391573482698298666531446032924_u128,85636768014485189160508452197833683203_u128,150780928501390953388939775567827910801_u128,339702736948176610084992045608161931028_u128,173803048529565060604185524612100148044_u128];
Goto(bb4)
}
bb6 = {
_4.1 = 525713491358128722_u64 as i16;
_4.3.1 = !_4.0.0.1;
_4.0.0 = (_4.3.0, _4.3.1);
_4.0.0 = _4.3;
_4.0.0.1 = _4.3.1 - _4.3.1;
_4.1 = (-26537_i16) + 12002_i16;
_2 = [7646508970762724902213155496332056611_u128,69791185526103508462189050140561302247_u128,107438947115972369934425453449017057047_u128,100174690587303073262526392141247938275_u128,194293731479165186101210830522863294366_u128,130941598934677934696268068430912120626_u128,175384794446900565123254283113053142718_u128];
RET.0 = 196_u8;
_4.0.1 = core::ptr::addr_of!(_4.3.0);
_4.1 = 43917_u16 as i16;
RET.1 = _1;
_2 = _1;
_4.1 = 7122_i16 + 9661_i16;
_4.0.0.0 = !_4.3.0;
RET.1 = [68982894874436509693428400683068749795_u128,17793717319257331966989754159150290198_u128,56213541840904905839971842496081282296_u128,102633678203458319969893586294532741103_u128,338978996815933312033373873391824923941_u128,292602830416748986655341738595909362229_u128,3736896324116441255796373598721844121_u128];
_4.1 = -23960_i16;
_4.3 = (_4.0.0.0, _4.0.0.1);
_4.0.0.1 = _4.3.1;
RET.1 = [13746534979404234555228899418309480944_u128,172334997244402808196627410332860953926_u128,129935502243686008885819161330455018689_u128,125295002943797248912777622176859948493_u128,197611190118188261144921607004588371052_u128,107701098834532696590103804365380297163_u128,33456244145101565682844577181488711032_u128];
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2 = RET.1;
_4.3.1 = _4.0.0.1;
_1 = RET.1;
_7 = Move(_4.0.1);
_4.0.1 = Move(_7);
RET.1 = _2;
RET = (107_u8, _1);
Goto(bb3)
}
bb7 = {
_4.1 = (-11031_i16);
_4.0.0.1 = -(-59163572096044188479048543810253953685_i128);
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_4.1 = (-27295_i16) ^ 29320_i16;
_4.1 = (-15743_i16) | 16152_i16;
RET.1 = [331475901501615158811564165909946177551_u128,124929833940651017846957589660410984173_u128,115505968415843853243669602779034986441_u128,170308057789679558626394998948153576344_u128,98263969906273251296581566231312702992_u128,278786006671746821700720545585263447745_u128,162666930719534563375245179616520559103_u128];
_4.0.0 = (_4.3.0, (-39755704545159054068169201779892516986_i128));
_4.0.0.0 = _4.0.0.1 as i32;
_4.0.0.1 = 9223372036854775807_isize as i128;
RET = (116_u8, _2);
_4.0.1 = core::ptr::addr_of!(_4.0.0.0);
_4.0.0 = (_4.3.0, 146904894921718203392678044254028908072_i128);
_7 = Move(_4.0.1);
_6 = [(-1_i8),90_i8,(-29_i8),(-68_i8),73_i8,50_i8];
_4.0.1 = Move(_7);
_4.3.1 = _4.0.0.1;
_4.3 = _4.0.0;
_7 = core::ptr::addr_of!(_4.0.0.0);
_4.1 = !(-6917_i16);
_4.3.1 = _4.0.0.1;
_4.0 = (_4.3, Move(_7));
RET.1 = [318141434734551403023212997313787060267_u128,76954513838564871331694144125404159638_u128,185420913233207514833142366357169949740_u128,23641032790247047951349004471645910949_u128,146216622846365223450329251010694627165_u128,180938612958057782112360154313997278646_u128,282642639073389101638356087074978051252_u128];
RET.0 = 86_u8 * 229_u8;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_4.0.0.1 = _4.3.1;
RET = (47_u8, _1);
RET.1 = _2;
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2 = _1;
_4.1 = (-7909_i16);
_4.0.0.0 = !_4.3.0;
_4.0.0.1 = -_4.3.1;
RET.1 = _1;
_1 = [282390057933962254358857395371744245783_u128,9432183731120161861426399853775051537_u128,214539934768690998780029460664369712683_u128,68518218889365838159112235891078944372_u128,271284318907126558579774329459819187186_u128,35379860438891559227487475074941176841_u128,331878804001878243349126703033872608638_u128];
_4.3 = _4.0.0;
_4.3.1 = 54950155966002753738482303541299936934_u128 as i128;
_4.0.0 = (_4.3.0, _4.3.1);
_7 = core::ptr::addr_of!(_4.0.0.0);
_4.0.0 = (_4.3.0, _4.3.1);
RET = (3_u8, _1);
_1 = _2;
_13.3.0.0 = RET;
_4.0 = (_4.3, Move(_7));
Call(_4.0.1 = fn13(RET.1, _1, _13.3.0.0.0, RET, _1, _3, _13.3.0.0, _13.3.0.0.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = 2690876080918767474_u64 as f32;
_4.0.0.0 = 11376_u16 as i32;
_2 = [225416832430455020570167606127657158851_u128,168686280132803249860845764039789158485_u128,20337566754185202342143864801399311229_u128,128784380073769473896453561931063548135_u128,186194429313351329234648650703568991107_u128,39863159604579142600888733461352374539_u128,220170326276901504107186499872815956569_u128];
_4.1 = !(-1017_i16);
_13.0 = [_4.3.1,_4.3.1];
match _13.3.0.0.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb12,
_ => bb11
}
}
bb11 = {
_2 = [22285324716350887989255063810272500984_u128,33513510566431655378291773755640836021_u128,184968685272264490794691270696987917940_u128,222561232314789778111357305136526569800_u128,116315532566660903214802796572275477782_u128,87238681843157734233556772510686469395_u128,38439184991660609176657340140522594182_u128];
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
107 => bb9,
_ => bb8
}
}
bb12 = {
_13.1 = &_16;
_2 = RET.1;
_11 = core::ptr::addr_of!(_4.0.0.0);
_13.1 = &_16;
_14 = Adt53::Variant1 { fld0: 65982313417140026558868597188659943583_u128 };
_13.2 = [_4.3.1,_4.0.0.1,_4.3.1,_4.0.0.1,_4.0.0.1,_4.3.1,_4.0.0.1];
_13.3.0.0.0 = RET.0 + RET.0;
place!(Field::<u128>(Variant(_14, 1), 0)) = 32748783772173066167774010933346423726_u128;
_16 = (-9223372036854775808_isize);
_19 = [RET.0,_13.3.0.0.0,RET.0,RET.0,_13.3.0.0.0,_13.3.0.0.0];
place!(Field::<u128>(Variant(_14, 1), 0)) = 258755136974131883437155734846572721892_u128 - 189271061843003521902945030062285436294_u128;
_12 = &_4.2;
_4.0.0.0 = 384853946827838079_u64 as i32;
_13.2 = [_4.0.0.1,_4.3.1,_4.3.1,_4.3.1,_4.3.1,_4.3.1,_4.0.0.1];
_21 = _4.3.1 >> _13.3.0.0.0;
_13.1 = &_16;
_4.0.0 = (_4.3.0, _21);
_20 = (14873182040820899492_u64,);
_4.3.0 = (*_11) | _4.0.0.0;
_9 = Adt53::Variant1 { fld0: Field::<u128>(Variant(_14, 1), 0) };
_4.3.0 = _4.0.0.0;
_24 = _16 as f32;
_4.3 = _4.0.0;
match RET.0 {
0 => bb6,
1 => bb9,
2 => bb5,
4 => bb13,
5 => bb14,
6 => bb15,
3 => bb17,
_ => bb16
}
}
bb13 = {
_4.1 = 525713491358128722_u64 as i16;
_4.3.1 = !_4.0.0.1;
_4.0.0 = (_4.3.0, _4.3.1);
_4.0.0 = _4.3;
_4.0.0.1 = _4.3.1 - _4.3.1;
_4.1 = (-26537_i16) + 12002_i16;
_2 = [7646508970762724902213155496332056611_u128,69791185526103508462189050140561302247_u128,107438947115972369934425453449017057047_u128,100174690587303073262526392141247938275_u128,194293731479165186101210830522863294366_u128,130941598934677934696268068430912120626_u128,175384794446900565123254283113053142718_u128];
RET.0 = 196_u8;
_4.0.1 = core::ptr::addr_of!(_4.3.0);
_4.1 = 43917_u16 as i16;
RET.1 = _1;
_2 = _1;
_4.1 = 7122_i16 + 9661_i16;
_4.0.0.0 = !_4.3.0;
RET.1 = [68982894874436509693428400683068749795_u128,17793717319257331966989754159150290198_u128,56213541840904905839971842496081282296_u128,102633678203458319969893586294532741103_u128,338978996815933312033373873391824923941_u128,292602830416748986655341738595909362229_u128,3736896324116441255796373598721844121_u128];
_4.1 = -23960_i16;
_4.3 = (_4.0.0.0, _4.0.0.1);
_4.0.0.1 = _4.3.1;
RET.1 = [13746534979404234555228899418309480944_u128,172334997244402808196627410332860953926_u128,129935502243686008885819161330455018689_u128,125295002943797248912777622176859948493_u128,197611190118188261144921607004588371052_u128,107701098834532696590103804365380297163_u128,33456244145101565682844577181488711032_u128];
_3 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_2 = RET.1;
_4.3.1 = _4.0.0.1;
_1 = RET.1;
_7 = Move(_4.0.1);
_4.0.1 = Move(_7);
RET.1 = _2;
RET = (107_u8, _1);
Goto(bb3)
}
bb14 = {
_15 = 2690876080918767474_u64 as f32;
_4.0.0.0 = 11376_u16 as i32;
_2 = [225416832430455020570167606127657158851_u128,168686280132803249860845764039789158485_u128,20337566754185202342143864801399311229_u128,128784380073769473896453561931063548135_u128,186194429313351329234648650703568991107_u128,39863159604579142600888733461352374539_u128,220170326276901504107186499872815956569_u128];
_4.1 = !(-1017_i16);
_13.0 = [_4.3.1,_4.3.1];
match _13.3.0.0.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb12,
_ => bb11
}
}
bb15 = {
_4.0.1 = core::ptr::addr_of!(_4.3.0);
_1 = [181054513251305039275166385585661855453_u128,248623208779067767164026136906174324899_u128,278239013391573482698298666531446032924_u128,85636768014485189160508452197833683203_u128,150780928501390953388939775567827910801_u128,339702736948176610084992045608161931028_u128,173803048529565060604185524612100148044_u128];
Goto(bb4)
}
bb16 = {
Return()
}
bb17 = {
_22.0 = _13.0;
_22.0 = _13.0;
_11 = Move(_4.0.1);
_24 = -_15;
_4.1 = _15 as i16;
_25 = '\u{47f8f}';
_4.0 = (_4.3, Move(_11));
SetDiscriminant(_9, 1);
RET.0 = !_13.3.0.0.0;
SetDiscriminant(_14, 0);
_13.3.0 = (RET,);
_4.0.0.1 = _21 ^ _21;
_13.3.0 = (RET,);
_7 = Move(_4.0.1);
_20.0 = 12487118605039106402_u64;
_4.0 = (_4.3, Move(_7));
_22.1.1 = 7_usize as u16;
_13.3.1 = [_4.0.0.1,_21,_4.0.0.1,_4.3.1,_4.3.1];
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(12_usize, 6_usize, Move(_6), 21_usize, Move(_21), 25_usize, Move(_25), 20_usize, Move(_20)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [u128; 7],mut _2: [u128; 7],mut _3: u8,mut _4: (u8, [u128; 7]),mut _5: [u128; 7],mut _6: [u8; 8],mut _7: (u8, [u128; 7]),mut _8: u8) -> *const i32 {
mir! {
type RET = *const i32;
let _9: ((u8, [u128; 7]),);
let _10: &'static &'static &'static [i64; 1];
let _11: Adt47;
let _12: isize;
let _13: Adt82;
let _14: [u128; 7];
let _15: i32;
let _16: u32;
let _17: ((i32, i128), &'static *mut u32, char, &'static &'static f32);
let _18: char;
let _19: ();
let _20: ();
{
_7.1 = [177717044473554835855037430862164795552_u128,191312514587023960936489032998989703676_u128,320319105974419744802438552987138774373_u128,40442599141304001234101143572762583534_u128,89919791654878157772752010394579665496_u128,229537481566998326029688245027090475323_u128,332238978228681412414689117106356907092_u128];
_3 = !_8;
_4.0 = _7.0 & _8;
_4.0 = _8 / _8;
_8 = _7.0;
Goto(bb1)
}
bb1 = {
_9.0 = (_3, _5);
_8 = _4.0 | _9.0.0;
_5 = [260139171266854704699667366220232478951_u128,308385425353278479237737736361546740046_u128,313889113528740596873768464832772604351_u128,98806558140534579985722264477284041948_u128,137800683949735309015514844708868108805_u128,168779243561220780594309363836033187714_u128,179160158840858018182304181178910248132_u128];
_1 = [74648377336567532070948893412536083931_u128,74086779372557757325056463774275719881_u128,215255804801354572898534506225061942603_u128,258099767570465215460787452946520841842_u128,253421965905846074085478770817422150810_u128,109524540302246674593014205173502775642_u128,77623886247620291592833530343128322018_u128];
_1 = _7.1;
_2 = [106690627078295284243864627934733012404_u128,63547161690485479943237576568749061422_u128,2119008281877662278024382041877346362_u128,85603275627554949154267088285707540017_u128,97574132256401177007568761657288512431_u128,243383590390135347561986842479482309457_u128,132969970295634714956529116221318282581_u128];
_4.0 = (-853538569263280553_i64) as u8;
_9.0.0 = !_3;
_4.1 = [120507487739927489315349964226791380042_u128,195749042092116304478591048168003710965_u128,335727316264175215085291511525829990009_u128,107688859325491105925570406141547202824_u128,191374241508189161964910606526700956169_u128,82515163664999613466448476554899545863_u128,78641800354054793557522413742659429846_u128];
_4.0 = _9.0.0;
_4.0 = _9.0.0;
_4.0 = _8 / _7.0;
_9.0 = (_8, _4.1);
_7.0 = !_9.0.0;
_7.1 = _9.0.1;
_2 = [38539200479839589202439680594712002733_u128,328395558154414117068128647139307350608_u128,201443215385696468776049075265762163652_u128,145964395115394328033709904794241217641_u128,61018470774876173326314974537029579934_u128,47148646880720359263175775110790495710_u128,179369148373605935381367409377671727186_u128];
_9.0 = _7;
_9 = (_4,);
_4.1 = [61438640068501767547415450932630647000_u128,95834146413163456851023853593131679435_u128,334872996764051323291522315156339708915_u128,35874966033885150771653673886868144668_u128,95168822944165059783332130635427101665_u128,74052513388589507339156999804757752465_u128,196537721429817893358840435059027924708_u128];
_9.0 = (_4.0, _7.1);
_4.1 = [305060664238870047160319603307445898670_u128,335654932076435928974048099174382355212_u128,247070877375137268767778206763295235926_u128,34239424416997069016999467236051718196_u128,70873529140411077428982417183220968781_u128,93911022235075328024756752742450129749_u128,11883483956550468817711087298469208286_u128];
_1 = [3450985776393806640912012602417164635_u128,320460375679577003071113263360502196722_u128,119759093584609901504568740981615211732_u128,253248322257701176428135491823368619371_u128,124094274092671393558897373866432127469_u128,315695081794058286712041120565975283934_u128,191068881680030020006609247477569271936_u128];
_2 = [322365771419322089889265025558951348957_u128,26107092551270173977168554368818249530_u128,249277240052668627895770368943749528915_u128,159119336231116950931787619385082901323_u128,181323303347423820056725267336163766154_u128,320223251057676377056318418671225942372_u128,105830989526876181722292547295785459015_u128];
Call(_4.0 = core::intrinsics::bswap(_8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _4.0;
_7.1 = _9.0.1;
_7.1 = _1;
_6 = [_9.0.0,_9.0.0,_8,_4.0,_9.0.0,_4.0,_4.0,_4.0];
_9 = (_7,);
_9 = (_4,);
_7.1 = _2;
_4.0 = '\u{94f72}' as u8;
_9 = (_7,);
_3 = _7.0 >> _9.0.0;
_12 = 9223372036854775807_isize;
_9.0.0 = !_3;
_4 = _9.0;
_7 = (_3, _1);
_12 = (-9223372036854775808_isize);
_8 = 60598_u16 as u8;
_7.0 = _3;
_4.1 = [38062638515478113319521930323508129447_u128,327366779300362992578889864615471287760_u128,338299827251240008880243512181250598691_u128,259399027868165188624225734143294431105_u128,48121007546297678236744908156400953196_u128,306469180426132871929785301251762362237_u128,50937189146218156807458147897450312893_u128];
_9.0.1 = [168312134544290333156327134446973738134_u128,87372191832518801562860202637524354127_u128,169885960602711772050069102133994615145_u128,235678697494366890466759977521875458813_u128,157586178892441916726124016202500158216_u128,107258117614259339276563739500966496363_u128,287931061184221337722340277918796214349_u128];
_9 = (_4,);
_6 = [_9.0.0,_9.0.0,_4.0,_3,_7.0,_4.0,_4.0,_7.0];
_5 = _7.1;
_4.1 = _5;
_4.1 = [298014722200635970387999486541184980419_u128,323347594593756749123704173296345933656_u128,212216635955379260146784420260683163240_u128,234528021761091923524584196193461447597_u128,207761593978806928626267826803901034464_u128,43873034407850455094725610263562890327_u128,322459488955714240852403829038518971867_u128];
_5 = [42404585695360811433497091111359268621_u128,197972044798789241591169237336849281327_u128,331434604950979833306016908478894668157_u128,121580058820725764058106345459397321584_u128,61616570629566707627729676136342463828_u128,18254813212583158394748223528152306814_u128,213980904179138874417973695053123412078_u128];
_6 = [_7.0,_7.0,_4.0,_4.0,_4.0,_7.0,_7.0,_3];
_6 = [_3,_9.0.0,_7.0,_4.0,_7.0,_3,_4.0,_3];
_2 = _1;
_4.0 = (-57042945572802573246025516726195811900_i128) as u8;
match _12 {
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
_9.0 = (_3, _5);
_8 = _4.0 | _9.0.0;
_5 = [260139171266854704699667366220232478951_u128,308385425353278479237737736361546740046_u128,313889113528740596873768464832772604351_u128,98806558140534579985722264477284041948_u128,137800683949735309015514844708868108805_u128,168779243561220780594309363836033187714_u128,179160158840858018182304181178910248132_u128];
_1 = [74648377336567532070948893412536083931_u128,74086779372557757325056463774275719881_u128,215255804801354572898534506225061942603_u128,258099767570465215460787452946520841842_u128,253421965905846074085478770817422150810_u128,109524540302246674593014205173502775642_u128,77623886247620291592833530343128322018_u128];
_1 = _7.1;
_2 = [106690627078295284243864627934733012404_u128,63547161690485479943237576568749061422_u128,2119008281877662278024382041877346362_u128,85603275627554949154267088285707540017_u128,97574132256401177007568761657288512431_u128,243383590390135347561986842479482309457_u128,132969970295634714956529116221318282581_u128];
_4.0 = (-853538569263280553_i64) as u8;
_9.0.0 = !_3;
_4.1 = [120507487739927489315349964226791380042_u128,195749042092116304478591048168003710965_u128,335727316264175215085291511525829990009_u128,107688859325491105925570406141547202824_u128,191374241508189161964910606526700956169_u128,82515163664999613466448476554899545863_u128,78641800354054793557522413742659429846_u128];
_4.0 = _9.0.0;
_4.0 = _9.0.0;
_4.0 = _8 / _7.0;
_9.0 = (_8, _4.1);
_7.0 = !_9.0.0;
_7.1 = _9.0.1;
_2 = [38539200479839589202439680594712002733_u128,328395558154414117068128647139307350608_u128,201443215385696468776049075265762163652_u128,145964395115394328033709904794241217641_u128,61018470774876173326314974537029579934_u128,47148646880720359263175775110790495710_u128,179369148373605935381367409377671727186_u128];
_9.0 = _7;
_9 = (_4,);
_4.1 = [61438640068501767547415450932630647000_u128,95834146413163456851023853593131679435_u128,334872996764051323291522315156339708915_u128,35874966033885150771653673886868144668_u128,95168822944165059783332130635427101665_u128,74052513388589507339156999804757752465_u128,196537721429817893358840435059027924708_u128];
_9.0 = (_4.0, _7.1);
_4.1 = [305060664238870047160319603307445898670_u128,335654932076435928974048099174382355212_u128,247070877375137268767778206763295235926_u128,34239424416997069016999467236051718196_u128,70873529140411077428982417183220968781_u128,93911022235075328024756752742450129749_u128,11883483956550468817711087298469208286_u128];
_1 = [3450985776393806640912012602417164635_u128,320460375679577003071113263360502196722_u128,119759093584609901504568740981615211732_u128,253248322257701176428135491823368619371_u128,124094274092671393558897373866432127469_u128,315695081794058286712041120565975283934_u128,191068881680030020006609247477569271936_u128];
_2 = [322365771419322089889265025558951348957_u128,26107092551270173977168554368818249530_u128,249277240052668627895770368943749528915_u128,159119336231116950931787619385082901323_u128,181323303347423820056725267336163766154_u128,320223251057676377056318418671225942372_u128,105830989526876181722292547295785459015_u128];
Call(_4.0 = core::intrinsics::bswap(_8), ReturnTo(bb2), UnwindUnreachable())
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
_7.1 = [142188477424805060013474127784466699884_u128,291800708422343212656359961672069556680_u128,166440549780313779849871645626135000570_u128,41789623304338105784417554366452007757_u128,270287774623561399449451528282247887415_u128,193536160061396395197038772960828799154_u128,135356510020230810498738279379309057536_u128];
_4.0 = _9.0.0 - _3;
_7.0 = 5307690108985541628_i64 as u8;
_7.0 = 8126808612207393175_usize as u8;
_2 = [32440218420945002020508971466001406692_u128,260710724446448531821291115811942344823_u128,23382424787583798415865026306177676078_u128,140751998127575972778436522353784352907_u128,299018765243653468712945707826613863038_u128,173464436474175363906664944975297679463_u128,299565541478457726442857671396257131547_u128];
_5 = [202976086185392736853837285134623986933_u128,309648100253470981655320744275424707295_u128,235836854680822562566386394813507029284_u128,202968390964206034805397662319841339890_u128,44292424209633594792357312620856676987_u128,111203120577573333350756816682668152996_u128,277651927626303609043568164300859586698_u128];
_9.0.1 = _1;
_6 = [_9.0.0,_4.0,_4.0,_4.0,_3,_3,_3,_4.0];
_5 = [226724072840323965463151782328035819263_u128,193933294821182878429462493370752342865_u128,144851933511003750840078351081957316018_u128,117140478653523321861885799073992796971_u128,82525692078334262607292834381941577611_u128,289756122746807071170129919116505836422_u128,269250195526730917910848014758428591940_u128];
_8 = _3 + _3;
_14 = _9.0.1;
_4.0 = (-25980219445458960900385681365667260815_i128) as u8;
_6 = [_9.0.0,_8,_8,_8,_8,_3,_9.0.0,_9.0.0];
_15 = 454986520_i32;
_4.0 = 6355032715483603730_i64 as u8;
_7 = (_8, _2);
_7.0 = _8 | _8;
_9.0.0 = _7.0 >> _7.0;
RET = core::ptr::addr_of!(_15);
_2 = [8680469626264227982143720927924535386_u128,276029771700626079607210616239673138218_u128,82872955684059431370925891065232627689_u128,140462607320712289897781993484180184448_u128,45823426227298246267228731101151478716_u128,145643894142614288038992590658742424338_u128,151502993060553374715143917557247707745_u128];
_5 = [249509927542162728911320837646426444592_u128,145090607708213563534708992909313238316_u128,324958517690021306996957934112046628741_u128,164548423764730746059403629945317311619_u128,85119014142104171950362678920115820087_u128,62695214262418773704980756687165219127_u128,129942597731618109580764061712658802185_u128];
_7 = (_8, _4.1);
_1 = [289176351046116008721323348836101686865_u128,95652666477798972830437000111347241482_u128,45568467084691550113167158268751127952_u128,311881169993244249680236274963294974847_u128,252098337590490650773978438016015239184_u128,34871369215537549029475351858237909437_u128,229898828456298271726902032046255412223_u128];
match _15 {
0 => bb11,
1 => bb12,
454986520 => bb14,
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
_1 = [208012762902045538039774991206952014350_u128,41645618964399128016689220451635614559_u128,27359336805613784471065763994272171648_u128,291819223235821190322405998447929243479_u128,321902276609515084765961642696126448056_u128,286414272441014626910833748306779863722_u128,237193262602549790735997458968844156997_u128];
_4.0 = !_8;
_5 = [213805986783340440505459541214549535461_u128,83554413360905994234610111429454114711_u128,15774045106808247741029078809547042437_u128,276485395166758513483743073436084828764_u128,206653850842174147218449957823088544389_u128,208654389897737817274275515374744013390_u128,331399784746493639938544633224988544181_u128];
_17.0 = ((*RET), (-5354892667110997560712482834871138047_i128));
_7 = _4;
_18 = '\u{15cc2}';
_17.0.0 = -(*RET);
_2 = _14;
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(13_usize, 4_usize, Move(_4), 5_usize, Move(_5), 12_usize, Move(_12), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_19 = dump_var(13_usize, 14_usize, Move(_14), 18_usize, Move(_18), 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [i32; 5],mut _2: (u8, [u128; 7])) -> ((u8, [u128; 7]),) {
mir! {
type RET = ((u8, [u128; 7]),);
let _3: ((u8, [u128; 7]),);
let _4: i8;
let _5: [i128; 2];
let _6: [i128; 7];
let _7: &'static &'static &'static [i64; 1];
let _8: f32;
let _9: [i8; 6];
let _10: (u64,);
let _11: char;
let _12: i16;
let _13: *mut u8;
let _14: f32;
let _15: i64;
let _16: ([i32; 5], [u16; 8]);
let _17: isize;
let _18: &'static &'static *mut u32;
let _19: f32;
let _20: char;
let _21: [u16; 8];
let _22: &'static &'static u8;
let _23: f64;
let _24: u16;
let _25: (u8, [u128; 7]);
let _26: ();
let _27: ();
{
RET.0.0 = _2.0 >> _2.0;
RET = (_2,);
_2.1 = [138033842553730189640636749194909269700_u128,57138795451020865955187738054275637971_u128,28554148661569063352416109166447868037_u128,166559541359379744096832703137486419273_u128,220628804248465051589246921460873939064_u128,19824926390030639198102745206302916120_u128,100213260773369512228032597491664689520_u128];
RET = (_2,);
_3.0 = (RET.0.0, RET.0.1);
_3.0 = RET.0;
Call(_2.0 = core::intrinsics::bswap(RET.0.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.1 = [69235217813572319345045041523703866607_u128,335997194921843379412628752357764422720_u128,89593771868686811627496115189438168435_u128,242583646442906577829168297829892922827_u128,26231954667248379932026878216864627364_u128,300335319035222510238784272615278538332_u128,189629948053110105548181720665281576419_u128];
RET = (_3.0,);
RET = _3;
_2.1 = [338001589202601807481278822955200329066_u128,304474340959083122452045949071849006995_u128,308158913901120028939519929150421835673_u128,66424933692088853900051186609824151980_u128,297756163860591417207348023636641974392_u128,152668224904042516824454422299746829430_u128,261107889202821765530191920112984167284_u128];
_4 = -(-117_i8);
RET.0.1 = [233766245570881803418223969920099501267_u128,145303929497842679824083399260315205733_u128,308827197826154161231320406079353838946_u128,237548410716871447473554646611625528430_u128,314316858902728449096707440180013351605_u128,131449896994618868610337671968670063364_u128,295832661125640044344822030960297920107_u128];
_6 = [(-114964043641970054157278608135979240631_i128),157270018293735228705690851061066560292_i128,135132374184622418480924939988981482680_i128,161941558344580096199693574656083958148_i128,(-82908889861085119786709866868126021954_i128),(-119005132707707533758848005586692962659_i128),122537240184078469694313666424818814753_i128];
RET = (_2,);
_1 = [1946935628_i32,1960587858_i32,1533402668_i32,560380968_i32,282013479_i32];
RET = _3;
RET.0.0 = _3.0.0 | _3.0.0;
_6 = [(-16243244104322736283011522633279073971_i128),75671221929182187124299412539450231014_i128,(-4558758205657547782324186753443632189_i128),167177976294655974135706855519486661110_i128,(-105810205159392137381565272718765679645_i128),(-156659471053344910019622661242394234009_i128),(-139448439593383356129426374319257642513_i128)];
Goto(bb2)
}
bb2 = {
_2.1 = [281176982831155654325056552202041646477_u128,196770508827846579607803181874315179283_u128,268190615022198993150615547336246778170_u128,124483874555853934690661588268744113560_u128,79971814362161908720421476950792435829_u128,44541735884428891907498947277355581473_u128,326879739750477074874468084615060868069_u128];
RET.0 = (_2.0, _2.1);
_11 = '\u{99af5}';
Goto(bb3)
}
bb3 = {
RET.0.0 = 20128_i16 as u8;
RET.0 = (_3.0.0, _3.0.1);
RET = _3;
_5 = [90945168923149199115405080790371166833_i128,(-65762468565238903956305462926814406386_i128)];
_3.0.0 = RET.0.0 ^ RET.0.0;
_3.0.0 = _2.0 | RET.0.0;
RET.0.1 = [93864403986410590733238457940149607568_u128,13411578844779911259622129905857929054_u128,64489246473659165638308324900604478129_u128,174427025548665722094011459844326201559_u128,125571380230088716786965830534168699973_u128,264563803595329336037066024534999676882_u128,85295050591128553746922438770134264332_u128];
_9 = [_4,_4,_4,_4,_4,_4];
_9 = [_4,_4,_4,_4,_4,_4];
RET.0.0 = _3.0.0 - _3.0.0;
_2.1 = [72925456078711711592425835683364231438_u128,178276820047126379252101022994059581773_u128,240511842594589600834651695969493911018_u128,292744550392780322449944577333327312622_u128,172875099506096369160296661509892682050_u128,146014287571093804593138718434112459929_u128,216109109140543411578804201208810355906_u128];
_10.0 = !6805353516517617628_u64;
_6 = [127197916530452518598343805216075600213_i128,106800990043417682403187970182335491525_i128,(-23033201068341494186036432293910018985_i128),(-125213289098390594502643903958116633344_i128),53171082523000198254566709889081787273_i128,(-36529929401373157212484856352299113216_i128),(-129542536129716631732769794602661416856_i128)];
RET.0 = (_2.0, _3.0.1);
_9 = [_4,_4,_4,_4,_4,_4];
RET.0 = (_3.0.0, _3.0.1);
_8 = 44320_u16 as f32;
Goto(bb4)
}
bb4 = {
_8 = (-31649_i16) as f32;
RET = (_3.0,);
_2.1 = [9832372400252351498148933997723953363_u128,143958932159883759486607636838429321577_u128,174007886017291745053110692573807379636_u128,201975444223099214150819682997360716783_u128,241403860462670510496593917466706043850_u128,235177718589513315198896226189348818470_u128,58115400891621637486306977772113317782_u128];
_2.1 = [41989574985779915427491970507319919506_u128,202581827283273544898701008445418680564_u128,38769132334866900184534939890944202859_u128,2617755382997268620450551497988538228_u128,72597612346807210962364365558000998058_u128,313611511349478131277451314893224475355_u128,289374765995609632201910528182312278357_u128];
_13 = core::ptr::addr_of_mut!(_2.0);
_8 = 8336269202612444625_usize as f32;
_5 = [16848268876220410173467526652027579738_i128,78668102211149727160666519237852046362_i128];
_10 = (10299224787332740716_u64,);
_4 = 75_i8 << (*_13);
_9 = [_4,_4,_4,_4,_4,_4];
_16.0 = _1;
Goto(bb5)
}
bb5 = {
_5 = [12067089306478739707757618556216094939_i128,(-167688899447454810126808349240801797505_i128)];
_11 = '\u{8209e}';
match _10.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
10299224787332740716 => bb13,
_ => bb12
}
}
bb6 = {
_8 = (-31649_i16) as f32;
RET = (_3.0,);
_2.1 = [9832372400252351498148933997723953363_u128,143958932159883759486607636838429321577_u128,174007886017291745053110692573807379636_u128,201975444223099214150819682997360716783_u128,241403860462670510496593917466706043850_u128,235177718589513315198896226189348818470_u128,58115400891621637486306977772113317782_u128];
_2.1 = [41989574985779915427491970507319919506_u128,202581827283273544898701008445418680564_u128,38769132334866900184534939890944202859_u128,2617755382997268620450551497988538228_u128,72597612346807210962364365558000998058_u128,313611511349478131277451314893224475355_u128,289374765995609632201910528182312278357_u128];
_13 = core::ptr::addr_of_mut!(_2.0);
_8 = 8336269202612444625_usize as f32;
_5 = [16848268876220410173467526652027579738_i128,78668102211149727160666519237852046362_i128];
_10 = (10299224787332740716_u64,);
_4 = 75_i8 << (*_13);
_9 = [_4,_4,_4,_4,_4,_4];
_16.0 = _1;
Goto(bb5)
}
bb7 = {
RET.0.0 = 20128_i16 as u8;
RET.0 = (_3.0.0, _3.0.1);
RET = _3;
_5 = [90945168923149199115405080790371166833_i128,(-65762468565238903956305462926814406386_i128)];
_3.0.0 = RET.0.0 ^ RET.0.0;
_3.0.0 = _2.0 | RET.0.0;
RET.0.1 = [93864403986410590733238457940149607568_u128,13411578844779911259622129905857929054_u128,64489246473659165638308324900604478129_u128,174427025548665722094011459844326201559_u128,125571380230088716786965830534168699973_u128,264563803595329336037066024534999676882_u128,85295050591128553746922438770134264332_u128];
_9 = [_4,_4,_4,_4,_4,_4];
_9 = [_4,_4,_4,_4,_4,_4];
RET.0.0 = _3.0.0 - _3.0.0;
_2.1 = [72925456078711711592425835683364231438_u128,178276820047126379252101022994059581773_u128,240511842594589600834651695969493911018_u128,292744550392780322449944577333327312622_u128,172875099506096369160296661509892682050_u128,146014287571093804593138718434112459929_u128,216109109140543411578804201208810355906_u128];
_10.0 = !6805353516517617628_u64;
_6 = [127197916530452518598343805216075600213_i128,106800990043417682403187970182335491525_i128,(-23033201068341494186036432293910018985_i128),(-125213289098390594502643903958116633344_i128),53171082523000198254566709889081787273_i128,(-36529929401373157212484856352299113216_i128),(-129542536129716631732769794602661416856_i128)];
RET.0 = (_2.0, _3.0.1);
_9 = [_4,_4,_4,_4,_4,_4];
RET.0 = (_3.0.0, _3.0.1);
_8 = 44320_u16 as f32;
Goto(bb4)
}
bb8 = {
_2.1 = [281176982831155654325056552202041646477_u128,196770508827846579607803181874315179283_u128,268190615022198993150615547336246778170_u128,124483874555853934690661588268744113560_u128,79971814362161908720421476950792435829_u128,44541735884428891907498947277355581473_u128,326879739750477074874468084615060868069_u128];
RET.0 = (_2.0, _2.1);
_11 = '\u{99af5}';
Goto(bb3)
}
bb9 = {
_2.1 = [69235217813572319345045041523703866607_u128,335997194921843379412628752357764422720_u128,89593771868686811627496115189438168435_u128,242583646442906577829168297829892922827_u128,26231954667248379932026878216864627364_u128,300335319035222510238784272615278538332_u128,189629948053110105548181720665281576419_u128];
RET = (_3.0,);
RET = _3;
_2.1 = [338001589202601807481278822955200329066_u128,304474340959083122452045949071849006995_u128,308158913901120028939519929150421835673_u128,66424933692088853900051186609824151980_u128,297756163860591417207348023636641974392_u128,152668224904042516824454422299746829430_u128,261107889202821765530191920112984167284_u128];
_4 = -(-117_i8);
RET.0.1 = [233766245570881803418223969920099501267_u128,145303929497842679824083399260315205733_u128,308827197826154161231320406079353838946_u128,237548410716871447473554646611625528430_u128,314316858902728449096707440180013351605_u128,131449896994618868610337671968670063364_u128,295832661125640044344822030960297920107_u128];
_6 = [(-114964043641970054157278608135979240631_i128),157270018293735228705690851061066560292_i128,135132374184622418480924939988981482680_i128,161941558344580096199693574656083958148_i128,(-82908889861085119786709866868126021954_i128),(-119005132707707533758848005586692962659_i128),122537240184078469694313666424818814753_i128];
RET = (_2,);
_1 = [1946935628_i32,1960587858_i32,1533402668_i32,560380968_i32,282013479_i32];
RET = _3;
RET.0.0 = _3.0.0 | _3.0.0;
_6 = [(-16243244104322736283011522633279073971_i128),75671221929182187124299412539450231014_i128,(-4558758205657547782324186753443632189_i128),167177976294655974135706855519486661110_i128,(-105810205159392137381565272718765679645_i128),(-156659471053344910019622661242394234009_i128),(-139448439593383356129426374319257642513_i128)];
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
_6 = [(-19053771641259602295557100632588852716_i128),70804891345789194006885860521708331763_i128,(-2080340588910920145945116237746923301_i128),49147251047015659656812006369820870943_i128,(-100709986542980824093888702375260456109_i128),(-50621936876866771043134963844103103677_i128),(-57777308368313384630823722717170559802_i128)];
(*_13) = !RET.0.0;
RET.0.0 = (*_13);
(*_13) = _3.0.0;
_15 = (-6429237046387076228_i64);
_16.1 = [64186_u16,8258_u16,44749_u16,40519_u16,25918_u16,14182_u16,38939_u16,43617_u16];
RET = _3;
_12 = !15997_i16;
RET.0.1 = _3.0.1;
_3 = RET;
_6 = [(-33668435240141258383521327337035052321_i128),56961241224000758806824431793583922486_i128,(-53905703023311487030759404461599608290_i128),42166249271085402697282243010878834417_i128,(-11671556747732597054882124348492659497_i128),59024244461950570582550534597148476967_i128,82483096429538752807135678651796636774_i128];
_4 = (-1667884412_i32) as i8;
_3 = RET;
_2 = (_3.0.0, _3.0.1);
_20 = _11;
_16.1 = [993_u16,20297_u16,38424_u16,33390_u16,17036_u16,25623_u16,13450_u16,16042_u16];
_3.0.0 = 161323380674246900840713857627525268414_u128 as u8;
_16.1 = [8236_u16,2861_u16,28910_u16,51439_u16,57169_u16,8361_u16,34394_u16,44188_u16];
_21 = [58935_u16,37965_u16,20382_u16,64602_u16,49911_u16,38927_u16,20974_u16,55384_u16];
RET.0.1 = [191562894164167365765326069570088919434_u128,273089402710050514423980829584875693593_u128,290241454202776762343793305660162189793_u128,316880071045364970573347184477172714797_u128,156707729850037556462717011618777399479_u128,135942829602152901201216629767167374053_u128,77126701157956670563877763137930690323_u128];
_4 = -(-64_i8);
RET.0.0 = _4 as u8;
_20 = _11;
Goto(bb14)
}
bb14 = {
_21 = [38823_u16,9800_u16,41328_u16,848_u16,34246_u16,19417_u16,35625_u16,30151_u16];
_8 = _10.0 as f32;
_13 = core::ptr::addr_of_mut!(_3.0.0);
_11 = _20;
_1 = [33985097_i32,(-745173167_i32),(-2011363734_i32),(-852599841_i32),(-510648866_i32)];
_3.0.1 = RET.0.1;
_10.0 = 14410855759311157275_u64 & 13759020873627994316_u64;
_16 = (_1, _21);
_1 = [970045611_i32,(-963291972_i32),1021924808_i32,(-641026642_i32),551357425_i32];
RET.0.0 = false as u8;
RET.0.1 = _2.1;
_2.1 = [241451100475451694705122886189374638684_u128,249268307813303067336302592799618782826_u128,156136996639259999869260477136664293830_u128,254061583242404458165614211554816969290_u128,218798065387606742226352597333361965355_u128,166155768199119817110972184317001922630_u128,81429151233929356244044292559692719450_u128];
_13 = core::ptr::addr_of_mut!(_2.0);
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(14_usize, 16_usize, Move(_16), 6_usize, Move(_6), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(14_usize, 5_usize, Move(_5), 15_usize, Move(_15), 20_usize, Move(_20), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{97f8}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-4_i8)), std::hint::black_box(15910_i16), std::hint::black_box(2046758136_i32), std::hint::black_box(3050752393090599092_i64), std::hint::black_box((-52672487486631829430627588028138701431_i128)), std::hint::black_box(665218743605201490_usize), std::hint::black_box(251_u8), std::hint::black_box(25155_u16), std::hint::black_box(74844038420917026153909236758398866339_u128));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt18{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt18 {
fld0: i32,
fld1: char,
fld2: u32,
fld3: i8,
}
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
fld0: Adt18,
fld1: isize,

},
Variant1{
fld0: i16,

},
Variant2{
fld0: bool,
fld1: char,
fld2: u8,
fld3: u64,
fld4: i16,
fld5: f64,
fld6: usize,

},
Variant3{
fld0: bool,
fld1: (u8, [u128; 7]),

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: f32,
fld1: [u128; 7],
fld2: [u32; 6],
fld3: i8,
fld4: ((u8, [u128; 7]),),
fld5: i32,
fld6: [i128; 5],
fld7: u128,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: f64,
fld1: *mut u8,
fld2: i64,
fld3: *mut *mut u32,
fld4: (u8, [u128; 7]),
fld5: *mut u32,

},
Variant1{
fld0: u8,
fld1: usize,
fld2: (((i32, i128), *const i32), i16, *mut u32, (i32, i128)),
fld3: i8,
fld4: u16,
fld5: f32,
fld6: [u128; 7],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *mut *mut u32,
fld1: i64,
fld2: (u64,),
fld3: (i32, i128),
fld4: [i32; 5],
fld5: Adt47,

},
Variant1{
fld0: ((u8, [u128; 7]), Adt24, [u128; 7]),
fld1: (*mut u32, (((i32, i128), *const i32), i16, *mut u32, (i32, i128))),
fld2: (Adt24, u16),
fld3: *mut u32,

},
Variant2{
fld0: i8,
fld1: usize,

},
Variant3{
fld0: f32,
fld1: ((i32, i128), *const i32),
fld2: (i32, i128),
fld3: i8,
fld4: u64,
fld5: i32,
fld6: i64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u32; 6],

},
Variant1{
fld0: u128,

},
Variant2{
fld0: u128,
fld1: u64,
fld2: [i32; 5],
fld3: i8,
fld4: *const i32,
fld5: usize,
fld6: [u128; 7],

},
Variant3{
fld0: Adt24,
fld1: (u8, [u128; 7]),
fld2: isize,
fld3: [u32; 6],

}}
impl PrintFDebug for Adt70{
	unsafe fn printf_debug(&self){unsafe{printf("Adt70::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt70 {
Variant0{
fld0: ((u8, [u128; 7]), Adt24, [u128; 7]),
fld1: char,
fld2: Adt24,
fld3: [i16; 7],
fld4: [u32; 4],
fld5: [u32; 6],
fld6: ([i128; 2], (Adt24, u16)),
fld7: *mut u32,

},
Variant1{
fld0: u16,
fld1: (u64,),
fld2: *mut *mut u32,

}}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){unsafe{printf("Adt82::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt82 {
Variant0{
fld0: (Adt24, u16),
fld1: char,
fld2: [i128; 7],
fld3: [i64; 5],
fld4: usize,
fld5: (*mut u32, (((i32, i128), *const i32), i16, *mut u32, (i32, i128))),
fld6: (i32, i128),

},
Variant1{
fld0: u64,
fld1: *mut *mut u32,
fld2: ((i32, i128), *const i32),
fld3: (u64,),
fld4: i32,

},
Variant2{
fld0: (i32, i128),
fld1: *const i32,
fld2: u32,
fld3: i128,
fld4: *mut u8,

}}

