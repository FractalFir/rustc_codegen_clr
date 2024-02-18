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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: isize,mut _4: i8,mut _5: u16,mut _6: u128,mut _7: i64,mut _8: i128,mut _9: u64) -> isize {
mir! {
type RET = isize;
let _10: [i16; 2];
let _11: i64;
let _12: (*mut i128, [bool; 2]);
let _13: isize;
let _14: (u128, Adt32, *mut [i32; 7], (f64, [char; 7]));
let _15: isize;
let _16: &'static [bool; 2];
let _17: Adt32;
let _18: isize;
let _19: Adt73;
let _20: &'static (u32, i128, isize, *mut i128);
let _21: isize;
let _22: u128;
let _23: Adt73;
let _24: f32;
let _25: ();
let _26: ();
{
_6 = 204_u8 as u128;
_4 = !79_i8;
_3 = !106_isize;
_9 = '\u{c2323}' as u64;
_8 = !(-70965003255362863889190106889627308941_i128);
_1 = !false;
_5 = 18974_i16 as u16;
Call(_4 = fn1(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14.3.1 = ['\u{4ccbf}','\u{4fb12}','\u{506de}','\u{38617}','\u{fdf60}','\u{68d29}','\u{4096d}'];
_14.3.0 = _9 as f64;
_2 = !239_u8;
_16 = &_12.1;
_3 = (-9223372036854775808_isize) * 52_isize;
_1 = _3 == _3;
_6 = !219498408436830357013325347511604804491_u128;
_11 = 1083404587_u32 as i64;
RET = !_3;
_14.0 = _6;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
35 => bb7,
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
RET = _3;
_11 = (-7141403255788788471_i64) * 6718206519633537231_i64;
Goto(bb8)
}
bb8 = {
_2 = !98_u8;
_12.0 = core::ptr::addr_of_mut!(_8);
_14.3.0 = 3371013770_u32 as f64;
_8 = !32317135073546610886490909599768920394_i128;
_12.1 = [_1,_1];
_6 = _14.0;
_10 = [18605_i16,(-19399_i16)];
_16 = &_12.1;
_16 = &(*_16);
_3 = RET;
_16 = &(*_16);
_14.0 = _6 + _6;
Call(RET = core::intrinsics::bswap(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = _8 as i64;
_12.1 = [_1,_1];
Goto(bb10)
}
bb10 = {
_4 = 120_i8 & (-100_i8);
_13 = RET + _3;
_19.fld4 = [1147071513_i32,1666118569_i32,1841216841_i32,1723118570_i32,1664625211_i32,(-1790464438_i32),869690044_i32];
_14.3.1 = ['\u{88e2}','\u{f27b1}','\u{d43eb}','\u{48864}','\u{f13bd}','\u{d03c2}','\u{c0a0e}'];
_19.fld1 = (_6,);
_15 = -_13;
_11 = -_7;
_14.2 = core::ptr::addr_of_mut!(_19.fld4);
_8 = '\u{e85f0}' as i128;
_12.1 = [_1,_1];
_16 = &_12.1;
_10 = [(-1094_i16),21245_i16];
_8 = 108966965269585188386329297340349188197_i128;
_19.fld6 = _8 as i64;
RET = _3 ^ _15;
match _8 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
108966965269585188386329297340349188197 => bb19,
_ => bb18
}
}
bb11 = {
_7 = _8 as i64;
_12.1 = [_1,_1];
Goto(bb10)
}
bb12 = {
_2 = !98_u8;
_12.0 = core::ptr::addr_of_mut!(_8);
_14.3.0 = 3371013770_u32 as f64;
_8 = !32317135073546610886490909599768920394_i128;
_12.1 = [_1,_1];
_6 = _14.0;
_10 = [18605_i16,(-19399_i16)];
_16 = &_12.1;
_16 = &(*_16);
_3 = RET;
_16 = &(*_16);
_14.0 = _6 + _6;
Call(RET = core::intrinsics::bswap(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
RET = _3;
_11 = (-7141403255788788471_i64) * 6718206519633537231_i64;
Goto(bb8)
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
Return()
}
bb18 = {
_14.3.1 = ['\u{4ccbf}','\u{4fb12}','\u{506de}','\u{38617}','\u{fdf60}','\u{68d29}','\u{4096d}'];
_14.3.0 = _9 as f64;
_2 = !239_u8;
_16 = &_12.1;
_3 = (-9223372036854775808_isize) * 52_isize;
_1 = _3 == _3;
_6 = !219498408436830357013325347511604804491_u128;
_11 = 1083404587_u32 as i64;
RET = !_3;
_14.0 = _6;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
35 => bb7,
_ => bb6
}
}
bb19 = {
_19.fld3 = _9 - _9;
_10 = [(-1353_i16),13960_i16];
_14.0 = _6 << _15;
_18 = _4 as isize;
_23.fld1.0 = _14.0;
_5 = (-19637_i16) as u16;
_19.fld5 = [_5,_5,_5,_5,_5,_5];
_14.0 = _5 as u128;
_23.fld2 = _18 | RET;
_8 = '\u{bb36}' as i128;
_19.fld6 = 18170_i16 as i64;
_12.0 = core::ptr::addr_of_mut!(_8);
_19.fld6 = _3 as i64;
_19.fld0 = core::ptr::addr_of!(_8);
_14.2 = core::ptr::addr_of_mut!(_23.fld4);
RET = _13;
_15 = RET >> RET;
_19.fld2 = _23.fld1.0 as isize;
_14.3.1 = ['\u{9abb6}','\u{f2779}','\u{f91ed}','\u{fc8ff}','\u{4880c}','\u{73584}','\u{107749}'];
_8 = 97805693839028469165420607928117549563_i128;
Goto(bb20)
}
bb20 = {
Call(_25 = dump_var(0_usize, 18_usize, Move(_18), 4_usize, Move(_4), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_25 = dump_var(0_usize, 8_usize, Move(_8), 2_usize, Move(_2), 5_usize, Move(_5), 26_usize, _26), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize) -> i8 {
mir! {
type RET = i8;
let _2: (Adt32, &'static bool, [u8; 6], [bool; 2]);
let _3: [i32; 7];
let _4: char;
let _5: &'static (u32, i128, isize, *mut i128);
let _6: &'static [bool; 2];
let _7: isize;
let _8: Adt56;
let _9: isize;
let _10: i64;
let _11: isize;
let _12: [bool; 5];
let _13: (f64, [char; 7]);
let _14: u32;
let _15: [bool; 3];
let _16: f32;
let _17: Adt32;
let _18: isize;
let _19: isize;
let _20: u8;
let _21: [u16; 6];
let _22: u16;
let _23: *mut (f32,);
let _24: [i8; 5];
let _25: f64;
let _26: (f64, [char; 7]);
let _27: f32;
let _28: i32;
let _29: char;
let _30: *mut [i32; 7];
let _31: ();
let _32: ();
{
_2.2 = [67_u8,77_u8,120_u8,8_u8,212_u8,248_u8];
Goto(bb1)
}
bb1 = {
_3 = [2032069010_i32,1004960320_i32,(-1492350074_i32),1750120366_i32,(-1871228552_i32),545567542_i32,(-131006250_i32)];
_2.3 = [true,true];
RET = (-22_i8);
_1 = !26_isize;
_2.3 = [true,false];
_1 = (-58_isize);
_4 = '\u{97815}';
_2.2 = [67_u8,8_u8,110_u8,234_u8,210_u8,134_u8];
RET = (-128_i8) >> _1;
_2.3 = [true,false];
RET = 29_i8;
_3 = [235880943_i32,(-2122111354_i32),1101378932_i32,1811966880_i32,2003015175_i32,1719375237_i32,907273121_i32];
_3 = [970560609_i32,1461202955_i32,(-614081783_i32),(-571193292_i32),(-331963629_i32),1254962671_i32,798425213_i32];
RET = 681156851707937388_i64 as i8;
_1 = 17104_u16 as isize;
_2.2 = [39_u8,5_u8,168_u8,139_u8,13_u8,83_u8];
_7 = 5303458392667031170_usize as isize;
_6 = &_2.3;
_2.2 = [21_u8,187_u8,69_u8,167_u8,170_u8,240_u8];
RET = 103_i8;
_2.2 = [63_u8,180_u8,222_u8,167_u8,62_u8,201_u8];
Goto(bb2)
}
bb2 = {
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_8.fld1.0 = 1193729552_u32 as f64;
_2.3 = [false,true];
RET = (-19_i8) ^ 19_i8;
_3 = [921718250_i32,1987552647_i32,759789057_i32,1196396400_i32,832396272_i32,(-571602776_i32),(-1408787536_i32)];
_8.fld3.0 = _8.fld1.0 as f32;
_2.3 = [false,true];
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_3 = [(-858652919_i32),1397553545_i32,(-661678354_i32),(-1969248530_i32),220993773_i32,(-2123851267_i32),(-320089500_i32)];
_8.fld2 = core::ptr::addr_of!(_9);
_8.fld3.0 = (-4173283511846592161_i64) as f32;
RET = (-45_i8) ^ 33_i8;
_8.fld2 = core::ptr::addr_of!(_9);
RET = (-7_i8) << _1;
_6 = &_2.3;
RET = (-108120460_i32) as i8;
Call(_8.fld3.0 = fn2(Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = &_2.3;
_8.fld0 = 907544814197087778_u64 * 11695137746540446722_u64;
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_1 = _7;
_2.0 = Adt32::Variant1 { fld0: _8.fld3,fld1: _1 };
_11 = !_1;
_8.fld2 = core::ptr::addr_of!(_7);
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_1 = 25_u8 as isize;
RET = (-34_i8) & 85_i8;
place!(Field::<isize>(Variant(_2.0, 1), 1)) = _1 << _11;
_8.fld3.0 = Field::<(f32,)>(Variant(_2.0, 1), 0).0 + Field::<(f32,)>(Variant(_2.0, 1), 0).0;
RET = (-58_i8) & (-38_i8);
_9 = (-1608387787_i32) as isize;
_9 = Field::<isize>(Variant(_2.0, 1), 1) & _1;
RET = (-5_i8) << _1;
_8.fld2 = core::ptr::addr_of!(_7);
_10 = -1381824533564299453_i64;
_7 = _4 as isize;
_10 = 511457370137427121_i64;
_11 = Field::<isize>(Variant(_2.0, 1), 1);
_2.0 = Adt32::Variant1 { fld0: _8.fld3,fld1: _1 };
_8.fld0 = 12247533190809055628_u64 | 12978880902906144323_u64;
_1 = 205_u8 as isize;
place!(Field::<(f32,)>(Variant(_2.0, 1), 0)) = (_8.fld3.0,);
_6 = &_2.3;
_4 = '\u{59072}';
_9 = _8.fld3.0 as isize;
Goto(bb4)
}
bb4 = {
_2.3 = [true,false];
_2.3 = [true,false];
_8.fld3.0 = Field::<(f32,)>(Variant(_2.0, 1), 0).0;
place!(Field::<(f32,)>(Variant(_2.0, 1), 0)) = (_8.fld3.0,);
place!(Field::<isize>(Variant(_2.0, 1), 1)) = 190837435381073939112548410776352817186_u128 as isize;
RET = 121_i8 & (-30_i8);
SetDiscriminant(_2.0, 1);
_7 = _8.fld3.0 as isize;
_1 = -_11;
_13.1 = _8.fld1.1;
_14 = 2444185461_u32;
_8.fld3.0 = _8.fld0 as f32;
_10 = !8743549695215652330_i64;
_12 = [true,false,false,false,false];
_8.fld1.1 = _13.1;
_6 = &_2.3;
_13.1 = [_4,_4,_4,_4,_4,_4,_4];
_16 = _8.fld3.0;
RET = 33_i8 * (-63_i8);
_1 = !_11;
_12 = [true,true,false,false,true];
_2.3 = [true,false];
_15 = [true,false,true];
_8.fld3.0 = _16 - _16;
_8.fld3 = (_16,);
_16 = _8.fld3.0;
_8.fld0 = 4892_u16 as u64;
Call(place!(Field::<isize>(Variant(_2.0, 1), 1)) = core::intrinsics::bswap(_9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_10 = -(-6354868077372911396_i64);
_8.fld3.0 = _16 * _16;
_1 = _14 as isize;
RET = 83_i8 + 84_i8;
_15 = [true,true,false];
_10 = 2531448067079070652_i64 ^ (-5781440884382443246_i64);
place!(Field::<(f32,)>(Variant(_2.0, 1), 0)).0 = -_8.fld3.0;
_8.fld2 = core::ptr::addr_of!(_7);
_3 = [(-1006045168_i32),(-1909116683_i32),(-1531922399_i32),1720347865_i32,1493559591_i32,(-862539059_i32),(-316867096_i32)];
RET = -(-119_i8);
_1 = _8.fld1.0 as isize;
_6 = &_2.3;
_16 = -Field::<(f32,)>(Variant(_2.0, 1), 0).0;
_2.2 = [193_u8,252_u8,98_u8,42_u8,52_u8,143_u8];
_8.fld0 = _8.fld1.0 as u64;
place!(Field::<isize>(Variant(_2.0, 1), 1)) = _14 as isize;
_12 = [false,true,true,false,false];
_8.fld1.0 = 967946228_i32 as f64;
_11 = 20659_u16 as isize;
place!(Field::<(f32,)>(Variant(_2.0, 1), 0)).0 = _16 * _16;
_1 = _11;
_8.fld2 = core::ptr::addr_of!(_9);
_13.0 = _8.fld1.0;
match _14 {
0 => bb1,
1 => bb4,
2 => bb6,
2444185461 => bb8,
_ => bb7
}
}
bb6 = {
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_8.fld1.0 = 1193729552_u32 as f64;
_2.3 = [false,true];
RET = (-19_i8) ^ 19_i8;
_3 = [921718250_i32,1987552647_i32,759789057_i32,1196396400_i32,832396272_i32,(-571602776_i32),(-1408787536_i32)];
_8.fld3.0 = _8.fld1.0 as f32;
_2.3 = [false,true];
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_3 = [(-858652919_i32),1397553545_i32,(-661678354_i32),(-1969248530_i32),220993773_i32,(-2123851267_i32),(-320089500_i32)];
_8.fld2 = core::ptr::addr_of!(_9);
_8.fld3.0 = (-4173283511846592161_i64) as f32;
RET = (-45_i8) ^ 33_i8;
_8.fld2 = core::ptr::addr_of!(_9);
RET = (-7_i8) << _1;
_6 = &_2.3;
RET = (-108120460_i32) as i8;
Call(_8.fld3.0 = fn2(Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = [2032069010_i32,1004960320_i32,(-1492350074_i32),1750120366_i32,(-1871228552_i32),545567542_i32,(-131006250_i32)];
_2.3 = [true,true];
RET = (-22_i8);
_1 = !26_isize;
_2.3 = [true,false];
_1 = (-58_isize);
_4 = '\u{97815}';
_2.2 = [67_u8,8_u8,110_u8,234_u8,210_u8,134_u8];
RET = (-128_i8) >> _1;
_2.3 = [true,false];
RET = 29_i8;
_3 = [235880943_i32,(-2122111354_i32),1101378932_i32,1811966880_i32,2003015175_i32,1719375237_i32,907273121_i32];
_3 = [970560609_i32,1461202955_i32,(-614081783_i32),(-571193292_i32),(-331963629_i32),1254962671_i32,798425213_i32];
RET = 681156851707937388_i64 as i8;
_1 = 17104_u16 as isize;
_2.2 = [39_u8,5_u8,168_u8,139_u8,13_u8,83_u8];
_7 = 5303458392667031170_usize as isize;
_6 = &_2.3;
_2.2 = [21_u8,187_u8,69_u8,167_u8,170_u8,240_u8];
RET = 103_i8;
_2.2 = [63_u8,180_u8,222_u8,167_u8,62_u8,201_u8];
Goto(bb2)
}
bb8 = {
RET = !(-60_i8);
_10 = -(-8598043539148534006_i64);
_13.0 = Field::<(f32,)>(Variant(_2.0, 1), 0).0 as f64;
_2.0 = Adt32::Variant1 { fld0: _8.fld3,fld1: _9 };
_7 = _1;
_4 = '\u{a9029}';
_6 = &(*_6);
_17 = Move(_2.0);
_16 = _8.fld3.0 * Field::<(f32,)>(Variant(_17, 1), 0).0;
_14 = !4212285397_u32;
place!(Field::<isize>(Variant(_17, 1), 1)) = RET as isize;
_2.0 = Move(_17);
RET = 127_i8;
_4 = '\u{26709}';
_18 = _9 - _1;
_8.fld3 = (_16,);
RET = (-100_i8) * (-119_i8);
SetDiscriminant(_2.0, 0);
Call(_8.fld1.1 = fn18(RET, _18, (*_6), _13, _8.fld1.0, _2.2, _8.fld3, _13.0, _13), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<(char, [u64; 4], u16)>(Variant(_2.0, 0), 2)).2 = 63976_u16 >> _8.fld0;
_8.fld1 = (_13.0, _13.1);
RET = 35_i8;
_4 = '\u{5f32e}';
Goto(bb10)
}
bb10 = {
place!(Field::<i8>(Variant(_2.0, 0), 3)) = !RET;
place!(Field::<(usize,)>(Variant(_2.0, 0), 0)) = (8501851625780811713_usize,);
place!(Field::<(f32,)>(Variant(_2.0, 0), 4)) = (_8.fld3.0,);
_19 = !_1;
_19 = _11;
_10 = (-1220306334487193120_i64);
_2.3 = [true,false];
_4 = '\u{6d7f6}';
_8.fld1.0 = _14 as f64;
match RET {
0 => bb4,
1 => bb7,
2 => bb11,
3 => bb12,
4 => bb13,
35 => bb15,
_ => bb14
}
}
bb11 = {
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_8.fld1.0 = 1193729552_u32 as f64;
_2.3 = [false,true];
RET = (-19_i8) ^ 19_i8;
_3 = [921718250_i32,1987552647_i32,759789057_i32,1196396400_i32,832396272_i32,(-571602776_i32),(-1408787536_i32)];
_8.fld3.0 = _8.fld1.0 as f32;
_2.3 = [false,true];
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_3 = [(-858652919_i32),1397553545_i32,(-661678354_i32),(-1969248530_i32),220993773_i32,(-2123851267_i32),(-320089500_i32)];
_8.fld2 = core::ptr::addr_of!(_9);
_8.fld3.0 = (-4173283511846592161_i64) as f32;
RET = (-45_i8) ^ 33_i8;
_8.fld2 = core::ptr::addr_of!(_9);
RET = (-7_i8) << _1;
_6 = &_2.3;
RET = (-108120460_i32) as i8;
Call(_8.fld3.0 = fn2(Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
RET = !(-60_i8);
_10 = -(-8598043539148534006_i64);
_13.0 = Field::<(f32,)>(Variant(_2.0, 1), 0).0 as f64;
_2.0 = Adt32::Variant1 { fld0: _8.fld3,fld1: _9 };
_7 = _1;
_4 = '\u{a9029}';
_6 = &(*_6);
_17 = Move(_2.0);
_16 = _8.fld3.0 * Field::<(f32,)>(Variant(_17, 1), 0).0;
_14 = !4212285397_u32;
place!(Field::<isize>(Variant(_17, 1), 1)) = RET as isize;
_2.0 = Move(_17);
RET = 127_i8;
_4 = '\u{26709}';
_18 = _9 - _1;
_8.fld3 = (_16,);
RET = (-100_i8) * (-119_i8);
SetDiscriminant(_2.0, 0);
Call(_8.fld1.1 = fn18(RET, _18, (*_6), _13, _8.fld1.0, _2.2, _8.fld3, _13.0, _13), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
_3 = [2032069010_i32,1004960320_i32,(-1492350074_i32),1750120366_i32,(-1871228552_i32),545567542_i32,(-131006250_i32)];
_2.3 = [true,true];
RET = (-22_i8);
_1 = !26_isize;
_2.3 = [true,false];
_1 = (-58_isize);
_4 = '\u{97815}';
_2.2 = [67_u8,8_u8,110_u8,234_u8,210_u8,134_u8];
RET = (-128_i8) >> _1;
_2.3 = [true,false];
RET = 29_i8;
_3 = [235880943_i32,(-2122111354_i32),1101378932_i32,1811966880_i32,2003015175_i32,1719375237_i32,907273121_i32];
_3 = [970560609_i32,1461202955_i32,(-614081783_i32),(-571193292_i32),(-331963629_i32),1254962671_i32,798425213_i32];
RET = 681156851707937388_i64 as i8;
_1 = 17104_u16 as isize;
_2.2 = [39_u8,5_u8,168_u8,139_u8,13_u8,83_u8];
_7 = 5303458392667031170_usize as isize;
_6 = &_2.3;
_2.2 = [21_u8,187_u8,69_u8,167_u8,170_u8,240_u8];
RET = 103_i8;
_2.2 = [63_u8,180_u8,222_u8,167_u8,62_u8,201_u8];
Goto(bb2)
}
bb14 = {
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_8.fld1.0 = 1193729552_u32 as f64;
_2.3 = [false,true];
RET = (-19_i8) ^ 19_i8;
_3 = [921718250_i32,1987552647_i32,759789057_i32,1196396400_i32,832396272_i32,(-571602776_i32),(-1408787536_i32)];
_8.fld3.0 = _8.fld1.0 as f32;
_2.3 = [false,true];
_8.fld1.1 = [_4,_4,_4,_4,_4,_4,_4];
_3 = [(-858652919_i32),1397553545_i32,(-661678354_i32),(-1969248530_i32),220993773_i32,(-2123851267_i32),(-320089500_i32)];
_8.fld2 = core::ptr::addr_of!(_9);
_8.fld3.0 = (-4173283511846592161_i64) as f32;
RET = (-45_i8) ^ 33_i8;
_8.fld2 = core::ptr::addr_of!(_9);
RET = (-7_i8) << _1;
_6 = &_2.3;
RET = (-108120460_i32) as i8;
Call(_8.fld3.0 = fn2(Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
place!(Field::<u16>(Variant(_2.0, 0), 1)) = Field::<(char, [u64; 4], u16)>(Variant(_2.0, 0), 2).2;
place!(Field::<(usize,)>(Variant(_2.0, 0), 0)).0 = !538874459981252294_usize;
place!(Field::<(char, [u64; 4], u16)>(Variant(_2.0, 0), 2)).1 = [_8.fld0,_8.fld0,_8.fld0,_8.fld0];
_2.2 = [94_u8,167_u8,105_u8,180_u8,163_u8,163_u8];
_12 = [true,false,false,true,false];
_2.2 = [135_u8,232_u8,68_u8,188_u8,91_u8,173_u8];
_22 = Field::<u16>(Variant(_2.0, 0), 1) - Field::<u16>(Variant(_2.0, 0), 1);
_26.1 = _8.fld1.1;
_2.2 = [146_u8,201_u8,128_u8,244_u8,250_u8,147_u8];
_22 = _10 as u16;
_16 = _7 as f32;
_4 = '\u{855fb}';
_23 = core::ptr::addr_of_mut!(place!(Field::<(f32,)>(Variant(_2.0, 0), 4)));
place!(Field::<(usize,)>(Variant(_2.0, 0), 0)) = (6_usize,);
_8.fld1.0 = _13.0 + _13.0;
_18 = _9;
_25 = _13.0 * _8.fld1.0;
_4 = '\u{20335}';
_23 = core::ptr::addr_of_mut!(place!(Field::<(f32,)>(Variant(_2.0, 0), 4)));
_18 = _7 >> Field::<(char, [u64; 4], u16)>(Variant(_2.0, 0), 2).2;
_6 = &_2.3;
_28 = !455374185_i32;
_14 = !4102326110_u32;
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(1_usize, 4_usize, Move(_4), 19_usize, Move(_19), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(1_usize, 11_usize, Move(_11), 14_usize, Move(_14), 18_usize, Move(_18), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: &'static [bool; 2]) -> f32 {
mir! {
type RET = f32;
let _2: u64;
let _3: [u128; 3];
let _4: char;
let _5: (Adt32, &'static bool, [u8; 6], [bool; 2]);
let _6: u16;
let _7: f64;
let _8: *const isize;
let _9: Adt24;
let _10: [u16; 4];
let _11: &'static f64;
let _12: [u8; 1];
let _13: ((usize,),);
let _14: &'static [bool; 2];
let _15: [i16; 5];
let _16: Adt73;
let _17: isize;
let _18: [bool; 2];
let _19: char;
let _20: ((u128, Adt32, *mut [i32; 7], (f64, [char; 7])), Adt67, bool, u8);
let _21: f32;
let _22: char;
let _23: i32;
let _24: Adt24;
let _25: bool;
let _26: i64;
let _27: u32;
let _28: ([char; 3],);
let _29: char;
let _30: [char; 7];
let _31: char;
let _32: u16;
let _33: f64;
let _34: (*mut i128, [bool; 2]);
let _35: i16;
let _36: isize;
let _37: *const i16;
let _38: Adt56;
let _39: ();
let _40: ();
{
RET = (-7457020654165747944_i64) as f32;
RET = 79348727743542613026414328180693503065_u128 as f32;
RET = (-47856302648854296352322055025360210666_i128) as f32;
RET = 26460208_u32 as f32;
RET = 188_u8 as f32;
_2 = !9490319490093389009_u64;
RET = _2 as f32;
RET = 13199142622532845988_usize as f32;
RET = 9_isize as f32;
RET = (-161169185824918646478976666044359300742_i128) as f32;
_2 = 9223372036854775807_isize as u64;
RET = 955001237732248343_usize as f32;
RET = _2 as f32;
RET = 2791309170_u32 as f32;
RET = (-7565599341871929724_i64) as f32;
_4 = '\u{3ec72}';
_2 = !9604741620016638463_u64;
RET = 1201030799_u32 as f32;
_3 = [3299335275222502948656878309086152292_u128,241324719689915085878971062806129434914_u128,272387532921984040934273352333641077293_u128];
_1 = &_5.3;
RET = 2731800741_u32 as f32;
Goto(bb1)
}
bb1 = {
_5.3 = [true,true];
_1 = &_5.3;
RET = 313961024076654971292375346740641609297_u128 as f32;
Call(RET = fn3(Move(_1), _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = &_5.3;
_6 = 28501_u16 * 31200_u16;
Goto(bb3)
}
bb3 = {
RET = 4210_i16 as f32;
_1 = &(*_1);
_2 = !14123517995277897622_u64;
RET = 21_isize as f32;
RET = (-3987825325393507360_i64) as f32;
_5.2 = [193_u8,249_u8,251_u8,18_u8,31_u8,220_u8];
_3 = [329865748059858644219656322301324987688_u128,40233896862225599402230990700309057992_u128,66291688146486813189624494863059648444_u128];
_1 = &(*_1);
RET = (-21459_i16) as f32;
_6 = 40605_u16;
_6 = 5573_u16;
_3 = [147445866656315714139726850276044972641_u128,110268711845117420701280135707262777454_u128,205923436821365264534376707270391660576_u128];
_6 = 45072_u16 + 59943_u16;
RET = 2931154157_u32 as f32;
Goto(bb4)
}
bb4 = {
RET = 0_usize as f32;
_6 = 11350_u16 - 51001_u16;
_7 = 2506205669016245520_i64 as f64;
_1 = &(*_1);
_1 = &(*_1);
_3 = [269745043848735967449290165407100672910_u128,306349102739020270334434722624550903835_u128,105132694484787902151781999767632565359_u128];
_6 = !59232_u16;
_1 = &(*_1);
_5.2 = [16_u8,21_u8,177_u8,98_u8,191_u8,20_u8];
RET = 2064147526_i32 as f32;
RET = _7 as f32;
_5.3 = [false,false];
_2 = !15169582064995404317_u64;
_7 = 4018576257_u32 as f64;
_10 = [_6,_6,_6,_6];
_6 = 40586_u16 & 56391_u16;
_6 = !9839_u16;
_4 = '\u{72906}';
RET = _2 as f32;
_11 = &_7;
_5.3 = [false,false];
_3 = [3393560680751355254263273414187394924_u128,283961682767992547351897682067364484661_u128,105269620484034370651883739425783075267_u128];
_5.2 = [99_u8,140_u8,236_u8,175_u8,253_u8,26_u8];
_1 = &_5.3;
_11 = &(*_11);
_11 = &_7;
RET = 1623764202_u32 as f32;
_6 = 65402_u16 + 6205_u16;
Goto(bb5)
}
bb5 = {
RET = 14635432804518303956_usize as f32;
RET = (-9223372036854775808_isize) as f32;
_11 = &_7;
_11 = &_7;
_6 = 39677_u16 + 10746_u16;
_12 = [171_u8];
_2 = 9219243703648799014_u64;
_13.0.0 = 2_usize >> _6;
RET = 9223372036854775807_isize as f32;
RET = 2335300352_u32 as f32;
_9 = Adt24::Variant2 { fld0: 165001316821811879827485101781780909065_i128,fld1: _4,fld2: 122_isize,fld3: (-92_i8),fld4: _13.0.0,fld5: (-717745526_i32) };
Goto(bb6)
}
bb6 = {
_5.2 = [172_u8,161_u8,201_u8,80_u8,134_u8,16_u8];
place!(Field::<isize>(Variant(_9, 2), 2)) = 18_isize << _6;
place!(Field::<i8>(Variant(_9, 2), 3)) = (-92_i8) | 21_i8;
_8 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_9, 2), 2)));
place!(Field::<char>(Variant(_9, 2), 1)) = _4;
RET = 54643426899836093678969767553543234691_i128 as f32;
_16.fld4 = [(-2028505221_i32),(-1431114049_i32),1530701435_i32,68189269_i32,(-1856909970_i32),(-1453653593_i32),(-895936208_i32)];
place!(Field::<i128>(Variant(_9, 2), 0)) = -(-162050987692361833681790227907237468697_i128);
_20.0.3.1 = [_4,_4,Field::<char>(Variant(_9, 2), 1),_4,_4,Field::<char>(Variant(_9, 2), 1),Field::<char>(Variant(_9, 2), 1)];
_20.1.fld5 = (-265285489_i32);
_7 = _13.0.0 as f64;
_17 = 29520_i16 as isize;
place!(Field::<isize>(Variant(_9, 2), 2)) = 251371192069780888594482648585362691481_u128 as isize;
_18 = [false,true];
(*_8) = _17 - _17;
_20.1.fld0 = _16.fld4;
_3 = [146875036743432417927251336714262872009_u128,60991161325204360540243912644151728161_u128,123713068505763075990145677569844220077_u128];
_14 = Move(_1);
Goto(bb7)
}
bb7 = {
_5.3 = _18;
_20.1.fld3 = -_7;
Goto(bb8)
}
bb8 = {
_13.0 = (Field::<usize>(Variant(_9, 2), 4),);
_16.fld2 = _20.1.fld5 as isize;
_16.fld1.0 = 64009462231219255356601757703445978723_u128;
_1 = &_5.3;
place!(Field::<i32>(Variant(_9, 2), 5)) = (*_8) as i32;
_18 = [true,false];
_20.0.2 = core::ptr::addr_of_mut!(_20.1.fld0);
_5.3 = _18;
_20.1.fld7 = core::ptr::addr_of!((*_8));
_19 = _4;
place!(Field::<i32>(Variant(_9, 2), 5)) = _20.1.fld5 | _20.1.fld5;
_16.fld3 = !_2;
_16.fld7 = 4161474470_u32;
_21 = Field::<i128>(Variant(_9, 2), 0) as f32;
match _16.fld1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
64009462231219255356601757703445978723 => bb9,
_ => bb7
}
}
bb9 = {
_20.1.fld0 = [Field::<i32>(Variant(_9, 2), 5),_20.1.fld5,Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5)];
_10 = [_6,_6,_6,_6];
_16.fld6 = 3926867740953689263_i64;
_20.1.fld5 = Field::<i32>(Variant(_9, 2), 5) & Field::<i32>(Variant(_9, 2), 5);
_16.fld5 = [_6,_6,_6,_6,_6,_6];
_5.1 = &_20.2;
_18 = [true,true];
SetDiscriminant(_9, 1);
_20.1.fld7 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_9, 1), 1)));
_20.1.fld4 = -_21;
_20.1.fld6 = core::ptr::addr_of!(_20.0.3);
_2 = _16.fld3 * _16.fld3;
_5.2 = [164_u8,184_u8,132_u8,22_u8,216_u8,152_u8];
place!(Field::<isize>(Variant(_9, 1), 1)) = _16.fld2;
_20.1.fld3 = Field::<isize>(Variant(_9, 1), 1) as f64;
_11 = &_20.0.3.0;
_20.1.fld3 = _7;
_16.fld5 = [_6,_6,_6,_6,_6,_6];
_20.1.fld6 = core::ptr::addr_of!(_20.0.3);
_20.1.fld4 = _21;
_20.1.fld7 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_9, 1), 1)));
_3 = [_16.fld1.0,_16.fld1.0,_16.fld1.0];
_23 = _20.1.fld5 >> _20.1.fld5;
_20.1.fld2 = _16.fld2 as u8;
_24 = Adt24::Variant2 { fld0: 22848551514829090174991291934466988169_i128,fld1: _4,fld2: _16.fld2,fld3: 88_i8,fld4: _13.0.0,fld5: _20.1.fld5 };
Goto(bb10)
}
bb10 = {
_20.1.fld7 = Move(_8);
_18 = [false,true];
_20.1.fld5 = _16.fld6 as i32;
_20.0.0 = _16.fld1.0 << Field::<usize>(Variant(_24, 2), 4);
place!(Field::<usize>(Variant(_24, 2), 4)) = _13.0.0 ^ _13.0.0;
_20.1.fld0 = [_23,_20.1.fld5,_23,_23,_23,Field::<i32>(Variant(_24, 2), 5),_23];
match _16.fld1.0 {
64009462231219255356601757703445978723 => bb12,
_ => bb11
}
}
bb11 = {
_20.1.fld0 = [Field::<i32>(Variant(_9, 2), 5),_20.1.fld5,Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5),Field::<i32>(Variant(_9, 2), 5)];
_10 = [_6,_6,_6,_6];
_16.fld6 = 3926867740953689263_i64;
_20.1.fld5 = Field::<i32>(Variant(_9, 2), 5) & Field::<i32>(Variant(_9, 2), 5);
_16.fld5 = [_6,_6,_6,_6,_6,_6];
_5.1 = &_20.2;
_18 = [true,true];
SetDiscriminant(_9, 1);
_20.1.fld7 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_9, 1), 1)));
_20.1.fld4 = -_21;
_20.1.fld6 = core::ptr::addr_of!(_20.0.3);
_2 = _16.fld3 * _16.fld3;
_5.2 = [164_u8,184_u8,132_u8,22_u8,216_u8,152_u8];
place!(Field::<isize>(Variant(_9, 1), 1)) = _16.fld2;
_20.1.fld3 = Field::<isize>(Variant(_9, 1), 1) as f64;
_11 = &_20.0.3.0;
_20.1.fld3 = _7;
_16.fld5 = [_6,_6,_6,_6,_6,_6];
_20.1.fld6 = core::ptr::addr_of!(_20.0.3);
_20.1.fld4 = _21;
_20.1.fld7 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_9, 1), 1)));
_3 = [_16.fld1.0,_16.fld1.0,_16.fld1.0];
_23 = _20.1.fld5 >> _20.1.fld5;
_20.1.fld2 = _16.fld2 as u8;
_24 = Adt24::Variant2 { fld0: 22848551514829090174991291934466988169_i128,fld1: _4,fld2: _16.fld2,fld3: 88_i8,fld4: _13.0.0,fld5: _20.1.fld5 };
Goto(bb10)
}
bb12 = {
place!(Field::<i8>(Variant(_24, 2), 3)) = (-4_i8) ^ 6_i8;
place!(Field::<isize>(Variant(_9, 1), 1)) = Field::<isize>(Variant(_24, 2), 2) << _20.0.0;
_20.3 = !_20.1.fld2;
_3 = [_20.0.0,_20.0.0,_16.fld1.0];
_16.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_24, 2), 0)));
_25 = !false;
_7 = _20.1.fld3 - _20.1.fld3;
_26 = _16.fld6 ^ _16.fld6;
place!(Field::<isize>(Variant(_9, 1), 1)) = RET as isize;
_20.1.fld5 = (-23001382250107594946492440063623769601_i128) as i32;
place!(Field::<usize>(Variant(_24, 2), 4)) = _16.fld7 as usize;
place!(Field::<i8>(Variant(_24, 2), 3)) = (-11_i8);
_16.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_24, 2), 0)));
Goto(bb13)
}
bb13 = {
place!(Field::<usize>(Variant(_24, 2), 4)) = _13.0.0;
_11 = &_20.1.fld3;
_27 = _16.fld7;
_8 = Move(_20.1.fld7);
_16.fld1 = (_20.0.0,);
_9 = Adt24::Variant2 { fld0: 31819582213840377424052854676496608488_i128,fld1: _4,fld2: Field::<isize>(Variant(_24, 2), 2),fld3: Field::<i8>(Variant(_24, 2), 3),fld4: _13.0.0,fld5: Field::<i32>(Variant(_24, 2), 5) };
place!(Field::<i128>(Variant(_24, 2), 0)) = _25 as i128;
_11 = &(*_11);
_16.fld1 = (_20.0.0,);
_16.fld6 = Field::<usize>(Variant(_9, 2), 4) as i64;
_20.0.3.0 = _20.1.fld3;
_20.0.3.1 = [Field::<char>(Variant(_9, 2), 1),_4,_19,Field::<char>(Variant(_24, 2), 1),_19,_19,_19];
_20.1.fld5 = !Field::<i32>(Variant(_9, 2), 5);
_32 = _23 as u16;
_11 = &_20.1.fld3;
match _27 {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb9,
4161474470 => bb15,
_ => bb14
}
}
bb14 = {
_1 = &_5.3;
_6 = 28501_u16 * 31200_u16;
Goto(bb3)
}
bb15 = {
_29 = _4;
_16.fld1.0 = _26 as u128;
_1 = &_18;
_1 = &(*_1);
_33 = _7 * (*_11);
_35 = !2909_i16;
_20.1.fld7 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_24, 2), 2)));
place!(Field::<isize>(Variant(_24, 2), 2)) = Field::<i128>(Variant(_24, 2), 0) as isize;
_19 = Field::<char>(Variant(_24, 2), 1);
_31 = Field::<char>(Variant(_24, 2), 1);
_30 = [_29,_31,_29,Field::<char>(Variant(_24, 2), 1),_19,_29,_19];
_30 = [_4,_4,_4,_29,Field::<char>(Variant(_9, 2), 1),Field::<char>(Variant(_9, 2), 1),_19];
_20.1.fld3 = -_20.0.3.0;
_4 = _19;
_15 = [_35,_35,_35,_35,_35];
_30 = [Field::<char>(Variant(_24, 2), 1),Field::<char>(Variant(_9, 2), 1),_19,Field::<char>(Variant(_24, 2), 1),_29,Field::<char>(Variant(_24, 2), 1),Field::<char>(Variant(_24, 2), 1)];
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(2_usize, 25_usize, Move(_25), 32_usize, Move(_32), 26_usize, Move(_26), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(2_usize, 30_usize, Move(_30), 2_usize, Move(_2), 23_usize, Move(_23), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(2_usize, 19_usize, Move(_19), 3_usize, Move(_3), 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: &'static [bool; 2],mut _2: u64) -> f32 {
mir! {
type RET = f32;
let _3: u8;
let _4: *mut isize;
let _5: (*mut i128, *const (f64, [char; 7]), u32, (u128, Adt32, *mut [i32; 7], (f64, [char; 7])));
let _6: [u64; 4];
let _7: i32;
let _8: (char, [u64; 4], u16);
let _9: i8;
let _10: (f32,);
let _11: *const i128;
let _12: bool;
let _13: *const [bool; 2];
let _14: ();
let _15: ();
{
RET = 23532_i16 as f32;
RET = 65_i8 as f32;
RET = 130_u8 as f32;
RET = 19044_i16 as f32;
_2 = 10061591784947573541_u64 * 16635959371604303195_u64;
_2 = 3690335325031866848_u64;
_3 = !211_u8;
RET = _3 as f32;
_2 = !426989355298836628_u64;
_3 = !235_u8;
_2 = _3 as u64;
Call(_2 = fn4(RET, RET, _3, _3, RET, RET, RET, _3, _3, RET, RET, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 279560173364528514963708407752250934284_u128 as f32;
RET = (-948389277_i32) as f32;
_2 = 5104424387554999117_u64;
match _2 {
0 => bb2,
5104424387554999117 => bb4,
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
_3 = 60_u8 + 187_u8;
RET = _3 as f32;
_3 = 145_u8 - 83_u8;
_5.2 = !2071494053_u32;
_5.3.3.1 = ['\u{a48fc}','\u{ed180}','\u{5d932}','\u{41147}','\u{60c1a}','\u{bd343}','\u{7b5f}'];
_3 = 223_u8;
Goto(bb5)
}
bb5 = {
_5.3.3.0 = 10065_u16 as f64;
_5.1 = core::ptr::addr_of!(_5.3.3);
_5.3.3.1 = ['\u{e52f6}','\u{5e28b}','\u{8b318}','\u{7bdfe}','\u{3d559}','\u{70b3d}','\u{ef0ec}'];
RET = 265360616074366694630532990241565793931_u128 as f32;
_5.3.3.0 = 1309871692_i32 as f64;
_5.2 = 4115173700_u32;
_5.3.3.1 = ['\u{106a0e}','\u{eaca7}','\u{d0d6f}','\u{5f8d8}','\u{2c2d9}','\u{27218}','\u{4da2c}'];
_5.2 = !813197087_u32;
_5.3.0 = 101977344412239500660134639598072895707_u128;
_5.1 = core::ptr::addr_of!(_5.3.3);
_5.3.3.1 = ['\u{bf953}','\u{e8fd7}','\u{7767f}','\u{73288}','\u{fe40d}','\u{f3b28}','\u{291ce}'];
_3 = 107_u8;
_5.3.0 = 255324815919833620859868704470713038593_u128 | 115080554878765066072228494945566853259_u128;
RET = _5.2 as f32;
RET = _3 as f32;
_6 = [_2,_2,_2,_2];
_5.3.0 = 173056984696109899064283322197614997735_u128 * 222173968432842172190388983400971370459_u128;
_7 = 2116677321_i32 - (-1259893853_i32);
_5.3.3.0 = 40037_u16 as f64;
RET = _5.3.0 as f32;
_7 = !1670846367_i32;
RET = _2 as f32;
Goto(bb6)
}
bb6 = {
_8.2 = 56416_u16 << _5.3.0;
RET = _2 as f32;
_8.2 = 1991_u16;
_9 = _5.3.3.0 as i8;
_3 = !171_u8;
_2 = 16880719883779643438_u64 ^ 10659703239900903176_u64;
_2 = 10989703935990512299_u64;
_5.3.3.0 = (-12831_i16) as f64;
match _8.2 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
1991 => bb14,
_ => bb13
}
}
bb7 = {
_5.3.3.0 = 10065_u16 as f64;
_5.1 = core::ptr::addr_of!(_5.3.3);
_5.3.3.1 = ['\u{e52f6}','\u{5e28b}','\u{8b318}','\u{7bdfe}','\u{3d559}','\u{70b3d}','\u{ef0ec}'];
RET = 265360616074366694630532990241565793931_u128 as f32;
_5.3.3.0 = 1309871692_i32 as f64;
_5.2 = 4115173700_u32;
_5.3.3.1 = ['\u{106a0e}','\u{eaca7}','\u{d0d6f}','\u{5f8d8}','\u{2c2d9}','\u{27218}','\u{4da2c}'];
_5.2 = !813197087_u32;
_5.3.0 = 101977344412239500660134639598072895707_u128;
_5.1 = core::ptr::addr_of!(_5.3.3);
_5.3.3.1 = ['\u{bf953}','\u{e8fd7}','\u{7767f}','\u{73288}','\u{fe40d}','\u{f3b28}','\u{291ce}'];
_3 = 107_u8;
_5.3.0 = 255324815919833620859868704470713038593_u128 | 115080554878765066072228494945566853259_u128;
RET = _5.2 as f32;
RET = _3 as f32;
_6 = [_2,_2,_2,_2];
_5.3.0 = 173056984696109899064283322197614997735_u128 * 222173968432842172190388983400971370459_u128;
_7 = 2116677321_i32 - (-1259893853_i32);
_5.3.3.0 = 40037_u16 as f64;
RET = _5.3.0 as f32;
_7 = !1670846367_i32;
RET = _2 as f32;
Goto(bb6)
}
bb8 = {
_3 = 60_u8 + 187_u8;
RET = _3 as f32;
_3 = 145_u8 - 83_u8;
_5.2 = !2071494053_u32;
_5.3.3.1 = ['\u{a48fc}','\u{ed180}','\u{5d932}','\u{41147}','\u{60c1a}','\u{bd343}','\u{7b5f}'];
_3 = 223_u8;
Goto(bb5)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
RET = 279560173364528514963708407752250934284_u128 as f32;
RET = (-948389277_i32) as f32;
_2 = 5104424387554999117_u64;
match _2 {
0 => bb2,
5104424387554999117 => bb4,
_ => bb3
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_8.0 = '\u{993c2}';
_5.3.3.1 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_6 = [_2,_2,_2,_2];
_6 = [_2,_2,_2,_2];
_8.1 = _6;
_8.1 = _6;
_10.0 = RET + RET;
_5.1 = core::ptr::addr_of!(_5.3.3);
_10 = (RET,);
_10 = (RET,);
_5.2 = !1497566893_u32;
_5.3.3.1 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_5.3.0 = false as u128;
_3 = 42_u8 | 179_u8;
_5.1 = core::ptr::addr_of!(_5.3.3);
_5.1 = core::ptr::addr_of!(_5.3.3);
_8.0 = '\u{ef27}';
_5.2 = 383017554_u32 ^ 1195202799_u32;
_5.1 = core::ptr::addr_of!(_5.3.3);
_9 = !71_i8;
_8.1 = [_2,_2,_2,_2];
_5.3.3.1 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_7 = (-346480359_i32) - 1484171915_i32;
_5.3.0 = 203931503920492184673954322741592588085_u128 ^ 113976846668679750066785677730404559668_u128;
_8.0 = '\u{a7d81}';
_12 = true;
_5.3.3.0 = 121070746061027326899741133473190476991_i128 as f64;
_5.3.3.0 = _2 as f64;
Goto(bb15)
}
bb15 = {
Call(_14 = dump_var(3_usize, 6_usize, Move(_6), 3_usize, Move(_3), 8_usize, Move(_8), 15_usize, _15), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: f32,mut _2: f32,mut _3: u8,mut _4: u8,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: u8,mut _9: u8,mut _10: f32,mut _11: f32,mut _12: u8) -> u64 {
mir! {
type RET = u64;
let _13: char;
let _14: i128;
let _15: usize;
let _16: Adt68;
let _17: bool;
let _18: Adt68;
let _19: i64;
let _20: i128;
let _21: u32;
let _22: [u8; 6];
let _23: *const i16;
let _24: (u32, i128, isize, *mut i128);
let _25: (f32,);
let _26: *const i128;
let _27: (*mut i128, *const (f64, [char; 7]), u32, (u128, Adt32, *mut [i32; 7], (f64, [char; 7])));
let _28: &'static (u32, i128, isize, *mut i128);
let _29: *mut i128;
let _30: (usize,);
let _31: (usize,);
let _32: u32;
let _33: bool;
let _34: [u16; 4];
let _35: f64;
let _36: (usize,);
let _37: ();
let _38: ();
{
_14 = -89436418256873231768247898620806650587_i128;
_9 = 9223372036854775807_isize as u8;
_11 = _7;
_6 = (-7700105435505324893_i64) as f32;
RET = 41_i8 as u64;
_9 = 9223372036854775807_isize as u8;
_10 = _5;
_1 = _5 + _10;
_13 = '\u{c6170}';
_1 = -_5;
_3 = _9 + _8;
_5 = -_1;
_5 = 6698000740081061916_usize as f32;
RET = 354262801_i32 as u64;
Goto(bb1)
}
bb1 = {
_9 = _3;
_3 = _12 | _9;
_6 = -_11;
RET = !8604335706635916171_u64;
_10 = _2;
_13 = '\u{d029}';
_6 = 1794762107_i32 as f32;
_15 = 4_usize - 7_usize;
RET = 17201401000302622829_u64;
_14 = RET as i128;
_11 = (-3682976634083901606_i64) as f32;
_2 = _7 + _1;
_16.fld1 = core::ptr::addr_of!(_14);
_16.fld5.fld1.0 = (-6439628249843315952_i64) as f64;
_4 = _3;
_16.fld5.fld3 = (_5,);
_2 = RET as f32;
Goto(bb2)
}
bb2 = {
_18.fld5.fld3 = (_2,);
_14 = (-87876792531543934714459744362443491572_i128);
_18.fld1 = Move(_16.fld1);
_3 = _9 | _8;
_18.fld2 = (-120_isize) - 9223372036854775807_isize;
_5 = -_2;
_16.fld5.fld2 = core::ptr::addr_of!(_16.fld2);
_18.fld6 = [_13,_13,_13];
_14 = !(-107329046293180086031895104744347962390_i128);
_18.fld5.fld0 = RET;
_8 = 1972237308_u32 as u8;
_10 = _7;
_16.fld1 = core::ptr::addr_of!(_14);
_19 = 8805245996975402790_i64;
_18.fld5.fld0 = RET | RET;
_5 = _1 + _1;
_16.fld4 = core::ptr::addr_of_mut!(_20);
RET = _9 as u64;
Call(_17 = fn5(_18.fld6, Move(_18.fld1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = !_8;
_9 = _3 * _3;
Call(_16.fld5.fld1.0 = core::intrinsics::transmute(_18.fld5.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _19 as u8;
_18.fld0 = !_17;
_16.fld2 = 20477_i16 as isize;
_16.fld5.fld2 = core::ptr::addr_of!(_16.fld2);
_16.fld5.fld3 = _18.fld5.fld3;
_18.fld3 = [(-2123146665_i32),(-514655950_i32),(-2091726275_i32),(-1716708958_i32),(-838116704_i32),(-1513287771_i32),1905714186_i32];
_6 = _1 - _1;
_14 = (-36966954508030747299564244251866051776_i128) ^ 68839945550295916346460793491311048139_i128;
match _19 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
8805245996975402790 => bb11,
_ => bb10
}
}
bb5 = {
_4 = !_8;
_9 = _3 * _3;
Call(_16.fld5.fld1.0 = core::intrinsics::transmute(_18.fld5.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_18.fld5.fld3 = (_2,);
_14 = (-87876792531543934714459744362443491572_i128);
_18.fld1 = Move(_16.fld1);
_3 = _9 | _8;
_18.fld2 = (-120_isize) - 9223372036854775807_isize;
_5 = -_2;
_16.fld5.fld2 = core::ptr::addr_of!(_16.fld2);
_18.fld6 = [_13,_13,_13];
_14 = !(-107329046293180086031895104744347962390_i128);
_18.fld5.fld0 = RET;
_8 = 1972237308_u32 as u8;
_10 = _7;
_16.fld1 = core::ptr::addr_of!(_14);
_19 = 8805245996975402790_i64;
_18.fld5.fld0 = RET | RET;
_5 = _1 + _1;
_16.fld4 = core::ptr::addr_of_mut!(_20);
RET = _9 as u64;
Call(_17 = fn5(_18.fld6, Move(_18.fld1)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_9 = _3;
_3 = _12 | _9;
_6 = -_11;
RET = !8604335706635916171_u64;
_10 = _2;
_13 = '\u{d029}';
_6 = 1794762107_i32 as f32;
_15 = 4_usize - 7_usize;
RET = 17201401000302622829_u64;
_14 = RET as i128;
_11 = (-3682976634083901606_i64) as f32;
_2 = _7 + _1;
_16.fld1 = core::ptr::addr_of!(_14);
_16.fld5.fld1.0 = (-6439628249843315952_i64) as f64;
_4 = _3;
_16.fld5.fld3 = (_5,);
_2 = RET as f32;
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
_25 = (_6,);
_4 = _9 ^ _3;
_18.fld5.fld1.1 = [_13,_13,_13,_13,_13,_13,_13];
_14 = 108848663365906839409133823613364649450_i128 ^ (-102956094426553853655723310527656009860_i128);
_13 = '\u{f8567}';
_16.fld5.fld3 = (_10,);
_11 = (-11531_i16) as f32;
_18.fld1 = Move(_16.fld1);
_12 = _17 as u8;
_20 = !_14;
_16.fld7 = core::ptr::addr_of_mut!(_16.fld3);
_19 = _16.fld5.fld1.0 as i64;
_2 = _5;
_18.fld7 = core::ptr::addr_of_mut!(_18.fld3);
_16.fld4 = core::ptr::addr_of_mut!(_20);
_16.fld5.fld0 = 1852443161_u32 as u64;
_18.fld3 = [(-157160318_i32),(-1969426471_i32),(-757601603_i32),(-796614863_i32),(-1752121514_i32),1957876788_i32,1796959760_i32];
_24.2 = _18.fld2 << _4;
_24.3 = core::ptr::addr_of_mut!(_14);
_24 = (1273608054_u32, _20, _16.fld2, Move(_16.fld4));
_27.2 = _24.0;
_16.fld2 = _24.2 - _18.fld2;
Goto(bb12)
}
bb12 = {
_2 = 872767471_i32 as f32;
match _27.2 {
0 => bb8,
1 => bb13,
2 => bb14,
1273608054 => bb16,
_ => bb15
}
}
bb13 = {
_25 = (_6,);
_4 = _9 ^ _3;
_18.fld5.fld1.1 = [_13,_13,_13,_13,_13,_13,_13];
_14 = 108848663365906839409133823613364649450_i128 ^ (-102956094426553853655723310527656009860_i128);
_13 = '\u{f8567}';
_16.fld5.fld3 = (_10,);
_11 = (-11531_i16) as f32;
_18.fld1 = Move(_16.fld1);
_12 = _17 as u8;
_20 = !_14;
_16.fld7 = core::ptr::addr_of_mut!(_16.fld3);
_19 = _16.fld5.fld1.0 as i64;
_2 = _5;
_18.fld7 = core::ptr::addr_of_mut!(_18.fld3);
_16.fld4 = core::ptr::addr_of_mut!(_20);
_16.fld5.fld0 = 1852443161_u32 as u64;
_18.fld3 = [(-157160318_i32),(-1969426471_i32),(-757601603_i32),(-796614863_i32),(-1752121514_i32),1957876788_i32,1796959760_i32];
_24.2 = _18.fld2 << _4;
_24.3 = core::ptr::addr_of_mut!(_14);
_24 = (1273608054_u32, _20, _16.fld2, Move(_16.fld4));
_27.2 = _24.0;
_16.fld2 = _24.2 - _18.fld2;
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_10 = -_25.0;
_27.3.0 = !198304329639207173273050415583199532274_u128;
_30 = (_15,);
_27.3.2 = Move(_18.fld7);
_18.fld5.fld1.1 = [_13,_13,_13,_13,_13,_13,_13];
_8 = _19 as u8;
_27.3.2 = core::ptr::addr_of_mut!(_16.fld3);
_29 = Move(_24.3);
_16.fld5.fld0 = RET;
_16.fld6 = _18.fld6;
_22 = [_4,_3,_9,_3,_4,_4];
_18.fld4 = Move(_29);
_18.fld3 = [(-774366442_i32),546862869_i32,974130455_i32,(-753387660_i32),1971235827_i32,1266681431_i32,(-1631158322_i32)];
_31.0 = _15;
_18.fld6 = [_13,_13,_13];
_18.fld5.fld1.0 = _18.fld2 as f64;
_27.3.1 = Adt32::Variant1 { fld0: _18.fld5.fld3,fld1: _24.2 };
_16.fld1 = core::ptr::addr_of!(_20);
_24.3 = core::ptr::addr_of_mut!(_24.1);
_19 = -(-849242516645653190_i64);
_25.0 = _7;
_27.1 = core::ptr::addr_of!(_27.3.3);
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(4_usize, 20_usize, Move(_20), 13_usize, Move(_13), 19_usize, Move(_19), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(4_usize, 12_usize, Move(_12), 30_usize, Move(_30), 22_usize, Move(_22), 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [char; 3],mut _2: *const i128) -> bool {
mir! {
type RET = bool;
let _3: char;
let _4: i8;
let _5: u16;
let _6: (u32, i128, isize, *mut i128);
let _7: Adt67;
let _8: [char; 7];
let _9: bool;
let _10: i128;
let _11: [u128; 8];
let _12: i64;
let _13: f64;
let _14: *mut usize;
let _15: [isize; 6];
let _16: ((usize,),);
let _17: isize;
let _18: ([char; 3],);
let _19: char;
let _20: ();
let _21: ();
{
RET = 5_usize != 6482808698507839469_usize;
_3 = '\u{d8d0d}';
RET = false;
_1 = [_3,_3,_3];
_1 = [_3,_3,_3];
Goto(bb1)
}
bb1 = {
RET = !true;
_1 = [_3,_3,_3];
RET = !false;
_5 = !12409_u16;
_4 = !(-4_i8);
Goto(bb2)
}
bb2 = {
RET = true;
_3 = '\u{4a262}';
RET = true;
_3 = '\u{93160}';
_1 = [_3,_3,_3];
_3 = '\u{a5799}';
_3 = '\u{9ab1e}';
RET = true;
_5 = 30769_u16;
_5 = !63298_u16;
_4 = -79_i8;
_3 = '\u{30a09}';
RET = _5 <= _5;
_2 = core::ptr::addr_of!(_6.1);
(*_2) = (-6640550583716028732095199308813007781_i128);
_1 = [_3,_3,_3];
_6.0 = !6000346_u32;
_6.3 = core::ptr::addr_of_mut!(_6.1);
(*_2) = !(-6970324252673504117639233559339115255_i128);
_3 = '\u{1977b}';
Goto(bb3)
}
bb3 = {
_6.2 = 9223372036854775807_isize;
RET = _6.0 != _6.0;
_7.fld2 = 225_u8;
(*_2) = !(-67040661765142288112103231534861022017_i128);
_6.1 = !(-56667468587242026593819670620715002616_i128);
_7.fld7 = core::ptr::addr_of!(_6.2);
_1 = [_3,_3,_3];
(*_2) = _4 as i128;
_7.fld0 = [1228837085_i32,735393492_i32,1655217169_i32,1661765931_i32,(-841852108_i32),(-960683858_i32),1004220719_i32];
_5 = 49008_u16;
_7.fld5 = (-36129308_i32);
_1 = [_3,_3,_3];
_6.3 = core::ptr::addr_of_mut!((*_2));
_5 = 63246_u16;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_7.fld2 = _6.0 as u8;
_8 = [_3,_3,_3,_3,_3,_3,_3];
RET = false;
_7.fld3 = _7.fld5 as f64;
_6.2 = (-22_isize);
_7.fld4 = _6.0 as f32;
match _6.2 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463463374607431768211434 => bb8,
_ => bb7
}
}
bb4 = {
RET = true;
_3 = '\u{4a262}';
RET = true;
_3 = '\u{93160}';
_1 = [_3,_3,_3];
_3 = '\u{a5799}';
_3 = '\u{9ab1e}';
RET = true;
_5 = 30769_u16;
_5 = !63298_u16;
_4 = -79_i8;
_3 = '\u{30a09}';
RET = _5 <= _5;
_2 = core::ptr::addr_of!(_6.1);
(*_2) = (-6640550583716028732095199308813007781_i128);
_1 = [_3,_3,_3];
_6.0 = !6000346_u32;
_6.3 = core::ptr::addr_of_mut!(_6.1);
(*_2) = !(-6970324252673504117639233559339115255_i128);
_3 = '\u{1977b}';
Goto(bb3)
}
bb5 = {
RET = !true;
_1 = [_3,_3,_3];
RET = !false;
_5 = !12409_u16;
_4 = !(-4_i8);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_7.fld2 = !6_u8;
_5 = 5_usize as u16;
_7.fld2 = 84_u8;
_7.fld3 = 3_usize as f64;
_6.3 = core::ptr::addr_of_mut!(_10);
_7.fld7 = core::ptr::addr_of!(_6.2);
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_4 = (-112_i8) - (-76_i8);
RET = !true;
_3 = '\u{fa1bf}';
_8 = [_3,_3,_3,_3,_3,_3,_3];
_7.fld5 = 2_usize as i32;
_5 = !49659_u16;
RET = false;
_7.fld4 = (-8960275797590172171_i64) as f32;
(*_2) = 24207_i16 as i128;
_9 = _7.fld2 < _7.fld2;
_9 = RET ^ RET;
_5 = _7.fld3 as u16;
Goto(bb9)
}
bb9 = {
_6.1 = _6.2 as i128;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
match _6.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
340282366920938463463374607431768211434 => bb10,
_ => bb7
}
}
bb10 = {
_7.fld5 = _7.fld3 as i32;
_7.fld5 = (-2052304634_i32) >> (*_2);
_10 = (*_2) - (*_2);
_1 = [_3,_3,_3];
_6.2 = !9223372036854775807_isize;
(*_2) = _10;
_6.0 = 1212543484_u32;
RET = _7.fld4 > _7.fld4;
_6.0 = 2728890587_u32;
_2 = core::ptr::addr_of!((*_2));
RET = !_9;
(*_2) = -_10;
_7.fld2 = 211_u8;
RET = !_9;
_11 = [25397618349928595500683111904611654216_u128,157900478910741787004962573686952041374_u128,29670180854333150524401103455365939558_u128,300783798612352113497594334565955421979_u128,133996556876619917625606044813514263180_u128,171158829856339262023901953055324273073_u128,254951375378938731565938481759792006773_u128,282563624416638146581921309026640392473_u128];
_7.fld4 = (-9551_i16) as f32;
_8 = [_3,_3,_3,_3,_3,_3,_3];
RET = !_9;
_7.fld3 = 16023030935411992901_usize as f64;
_13 = _7.fld3 + _7.fld3;
_12 = (-5024365380092846415_i64);
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_7.fld3 = -_13;
_6.1 = _12 as i128;
_12 = -9173291961792544470_i64;
Call(_7.fld4 = fn6(_4, Move(_7.fld7), _13, _4, _6.2, Move(_6)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6.0 = (-9223372036854775808_isize) as u32;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_9 = _5 < _5;
_6.1 = _10;
_8 = [_3,_3,_3,_3,_3,_3,_3];
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_7.fld3 = _13;
_6.2 = 9223372036854775807_isize | 9223372036854775807_isize;
(*_2) = _10;
_6.2 = (*_2) as isize;
_9 = _6.2 == _6.2;
_7.fld5 = -(-487668832_i32);
_6.0 = 4276423811_u32 + 1399205703_u32;
(*_2) = _10;
_6.3 = core::ptr::addr_of_mut!((*_2));
(*_2) = -_10;
_9 = RET;
_7.fld5 = 662975323_i32;
_9 = RET;
_11 = [5256882976373636833053159999591261189_u128,242429738465794059183071362913842316094_u128,338936670271467266705574898048259033948_u128,232536827761814543199337319240438348878_u128,115440700983542251033497744141528363782_u128,57209059077855000688788872839214262480_u128,323056165519579993661571775778155298811_u128,100140754599698011710127426739838435956_u128];
_7.fld2 = 187_u8;
_2 = core::ptr::addr_of!(_10);
_13 = _7.fld3;
_7.fld4 = _7.fld5 as f32;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
Goto(bb12)
}
bb12 = {
RET = _9 & _9;
match _7.fld2 {
0 => bb1,
1 => bb9,
2 => bb13,
3 => bb14,
4 => bb15,
187 => bb17,
_ => bb16
}
}
bb13 = {
_6.0 = (-9223372036854775808_isize) as u32;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_9 = _5 < _5;
_6.1 = _10;
_8 = [_3,_3,_3,_3,_3,_3,_3];
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_7.fld3 = _13;
_6.2 = 9223372036854775807_isize | 9223372036854775807_isize;
(*_2) = _10;
_6.2 = (*_2) as isize;
_9 = _6.2 == _6.2;
_7.fld5 = -(-487668832_i32);
_6.0 = 4276423811_u32 + 1399205703_u32;
(*_2) = _10;
_6.3 = core::ptr::addr_of_mut!((*_2));
(*_2) = -_10;
_9 = RET;
_7.fld5 = 662975323_i32;
_9 = RET;
_11 = [5256882976373636833053159999591261189_u128,242429738465794059183071362913842316094_u128,338936670271467266705574898048259033948_u128,232536827761814543199337319240438348878_u128,115440700983542251033497744141528363782_u128,57209059077855000688788872839214262480_u128,323056165519579993661571775778155298811_u128,100140754599698011710127426739838435956_u128];
_7.fld2 = 187_u8;
_2 = core::ptr::addr_of!(_10);
_13 = _7.fld3;
_7.fld4 = _7.fld5 as f32;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
Goto(bb12)
}
bb14 = {
RET = !true;
_1 = [_3,_3,_3];
RET = !false;
_5 = !12409_u16;
_4 = !(-4_i8);
Goto(bb2)
}
bb15 = {
_6.1 = _6.2 as i128;
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
match _6.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
340282366920938463463374607431768211434 => bb10,
_ => bb7
}
}
bb16 = {
_7.fld2 = !6_u8;
_5 = 5_usize as u16;
_7.fld2 = 84_u8;
_7.fld3 = 3_usize as f64;
_6.3 = core::ptr::addr_of_mut!(_10);
_7.fld7 = core::ptr::addr_of!(_6.2);
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_4 = (-112_i8) - (-76_i8);
RET = !true;
_3 = '\u{fa1bf}';
_8 = [_3,_3,_3,_3,_3,_3,_3];
_7.fld5 = 2_usize as i32;
_5 = !49659_u16;
RET = false;
_7.fld4 = (-8960275797590172171_i64) as f32;
(*_2) = 24207_i16 as i128;
_9 = _7.fld2 < _7.fld2;
_9 = RET ^ RET;
_5 = _7.fld3 as u16;
Goto(bb9)
}
bb17 = {
RET = _5 < _5;
(*_2) = !_6.1;
_6.2 = 9223372036854775807_isize;
_7.fld7 = core::ptr::addr_of!(_17);
_11 = [211206091286301320772457592920875854831_u128,167207308352981839820650678132054811260_u128,213445536864540278616826094791460465061_u128,280455197458110532067410311009703684827_u128,109875438668429987022883684998223272722_u128,7047582222238718060763122689546741958_u128,237433220066846205446890939172699678211_u128,321211531895926125427985689291268828483_u128];
_2 = core::ptr::addr_of!(_10);
_7.fld0 = [_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5,_7.fld5];
_10 = !_6.1;
_6.0 = !3533642963_u32;
_17 = !_6.2;
_14 = core::ptr::addr_of_mut!(_16.0.0);
(*_14) = 288539683584880744_usize - 4_usize;
_7.fld4 = (-7303_i16) as f32;
(*_2) = _6.1 >> _6.1;
_7.fld3 = _7.fld2 as f64;
_6.0 = 1281301179_u32;
_7.fld3 = _13;
_18 = (_1,);
_3 = '\u{82c38}';
_7.fld5 = (-981680773_i32);
_7.fld3 = -_13;
_11 = [293881695927842254349892471668888142464_u128,137504070582600353983676688029547704165_u128,243476284572892876809635171086100014380_u128,308141369203944003527310586504888088266_u128,175170677034778383779512644868125782463_u128,178652133538746294725816855398640564321_u128,51046829120447731516261200105535395273_u128,260658430573046247966080260410014567201_u128];
_19 = _3;
_4 = _7.fld5 as i8;
_1 = _18.0;
Goto(bb18)
}
bb18 = {
Call(_20 = dump_var(5_usize, 11_usize, Move(_11), 8_usize, Move(_8), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_20 = dump_var(5_usize, 9_usize, Move(_9), 10_usize, Move(_10), 21_usize, _21, 21_usize, _21), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i8,mut _2: *const isize,mut _3: f64,mut _4: i8,mut _5: isize,mut _6: (u32, i128, isize, *mut i128)) -> f32 {
mir! {
type RET = f32;
let _7: [u8; 7];
let _8: *mut i128;
let _9: ((f32,), (f32,), [bool; 2]);
let _10: u16;
let _11: *const usize;
let _12: ((u128, Adt32, *mut [i32; 7], (f64, [char; 7])), Adt67, bool, u8);
let _13: [u128; 3];
let _14: i128;
let _15: (u128, Adt32, *mut [i32; 7], (f64, [char; 7]));
let _16: Adt59;
let _17: [i8; 5];
let _18: bool;
let _19: isize;
let _20: (usize,);
let _21: f32;
let _22: char;
let _23: f64;
let _24: [u16; 6];
let _25: &'static Adt56;
let _26: [u8; 7];
let _27: &'static [i32; 7];
let _28: [u8; 6];
let _29: u8;
let _30: [u8; 7];
let _31: u8;
let _32: &'static (u32, i128, isize, *mut i128);
let _33: Adt68;
let _34: isize;
let _35: *mut [i32; 7];
let _36: f64;
let _37: [u8; 6];
let _38: (Adt32, &'static bool, [u8; 6], [bool; 2]);
let _39: ();
let _40: ();
{
_6.1 = (-109387365365491507349753542638494138250_i128);
_6.3 = core::ptr::addr_of_mut!(_6.1);
_3 = (-1433916180_i32) as f64;
_7 = [90_u8,211_u8,158_u8,56_u8,66_u8,72_u8,70_u8];
_8 = Move(_6.3);
_9.0.0 = 1970_u16 as f32;
_6.1 = _3 as i128;
_9.1.0 = -_9.0.0;
_9.0 = (_9.1.0,);
RET = -_9.1.0;
_6.3 = core::ptr::addr_of_mut!(_6.1);
_6.3 = core::ptr::addr_of_mut!(_6.1);
_12.0.3.1 = ['\u{98b1d}','\u{b7e1c}','\u{edc8f}','\u{ad5f0}','\u{5733d}','\u{3396c}','\u{1c11e}'];
_6.1 = 40652132661354294735645073228270957147_i128 - (-91105261936180711796926897379015108751_i128);
_12.1.fld6 = core::ptr::addr_of!(_12.0.3);
_12.0.3.1 = ['\u{3897e}','\u{e38bf}','\u{3254b}','\u{4cf17}','\u{2b1c8}','\u{15dab}','\u{dec8c}'];
_12.1.fld5 = !1643872502_i32;
_7 = [222_u8,190_u8,188_u8,166_u8,48_u8,247_u8,251_u8];
_12.1.fld4 = RET + RET;
_6 = (3103282491_u32, 22122757514260869956811333025419123364_i128, _5, Move(_8));
_8 = Move(_6.3);
_12.0.1 = Adt32::Variant1 { fld0: _9.1,fld1: _5 };
Goto(bb1)
}
bb1 = {
SetDiscriminant(_12.0.1, 1);
Call(_12.0.3.0 = fn7(Move(_12.1.fld6), _6.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)).0 = _9.0.0;
_15.0 = 12024389234842585716_u64 as u128;
_3 = _12.0.3.0;
_12.0.3.1 = ['\u{1dfbd}','\u{ab9dd}','\u{10d6be}','\u{7aa81}','\u{e3f5b}','\u{5c13}','\u{10bce1}'];
_12.0.3.1 = ['\u{b04e}','\u{b68}','\u{cb889}','\u{64934}','\u{1045d1}','\u{fcb8f}','\u{2a7db}'];
_12.3 = 135_u8 | 95_u8;
_12.1.fld0 = [_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5];
RET = -_12.1.fld4;
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)) = _9.1;
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_12.0.2 = core::ptr::addr_of_mut!(_12.1.fld0);
_15.3.1 = _12.0.3.1;
_15.3 = (_3, _12.0.3.1);
_12.1.fld1 = core::ptr::addr_of_mut!(_9.1);
_6.0 = 2339716276_u32 | 3531362300_u32;
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_8 = core::ptr::addr_of_mut!(_14);
_12.1.fld5 = -2054649282_i32;
Goto(bb3)
}
bb3 = {
_19 = _6.2;
_3 = _4 as f64;
_22 = '\u{a9b60}';
RET = _1 as f32;
_12.3 = 190_u8;
_12.2 = false;
_6.2 = _19 - _19;
_11 = core::ptr::addr_of!(_20.0);
_11 = core::ptr::addr_of!((*_11));
_15.2 = core::ptr::addr_of_mut!(_12.1.fld0);
(*_8) = -_6.1;
_1 = _4 + _4;
RET = -_12.1.fld4;
_6.3 = Move(_8);
_9.2 = [_12.2,_12.2];
_12.1.fld4 = -_9.0.0;
_6.1 = _14;
_11 = core::ptr::addr_of!((*_11));
RET = _12.1.fld4 - _12.1.fld4;
_6.0 = _15.0 as u32;
_15.3.1 = [_22,_22,_22,_22,_22,_22,_22];
_12.1.fld3 = _3;
_12.1.fld7 = Move(_2);
Goto(bb4)
}
bb4 = {
_5 = _6.2;
_15.0 = !116916275699697028352895460726804961247_u128;
Goto(bb5)
}
bb5 = {
_12.1.fld3 = _12.0.3.0;
_20.0 = !2_usize;
_15.3 = (_12.0.3.0, _12.0.3.1);
RET = _12.1.fld4 + _9.1.0;
_21 = -_12.1.fld4;
_12.1.fld1 = core::ptr::addr_of_mut!(_9.0);
_6.0 = _14 as u32;
_12.1.fld6 = core::ptr::addr_of!(_15.3);
_12.1.fld6 = core::ptr::addr_of!(_12.0.3);
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)) = _9.0;
_27 = &_12.1.fld0;
place!(Field::<isize>(Variant(_12.0.1, 1), 1)) = !_5;
_15 = (235025497878763393986217581471641747477_u128, Move(_12.0.1), Move(_12.0.2), _12.0.3);
match _15.0 {
0 => bb3,
1 => bb2,
2 => bb6,
235025497878763393986217581471641747477 => bb8,
_ => bb7
}
}
bb6 = {
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)).0 = _9.0.0;
_15.0 = 12024389234842585716_u64 as u128;
_3 = _12.0.3.0;
_12.0.3.1 = ['\u{1dfbd}','\u{ab9dd}','\u{10d6be}','\u{7aa81}','\u{e3f5b}','\u{5c13}','\u{10bce1}'];
_12.0.3.1 = ['\u{b04e}','\u{b68}','\u{cb889}','\u{64934}','\u{1045d1}','\u{fcb8f}','\u{2a7db}'];
_12.3 = 135_u8 | 95_u8;
_12.1.fld0 = [_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5];
RET = -_12.1.fld4;
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)) = _9.1;
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_12.0.2 = core::ptr::addr_of_mut!(_12.1.fld0);
_15.3.1 = _12.0.3.1;
_15.3 = (_3, _12.0.3.1);
_12.1.fld1 = core::ptr::addr_of_mut!(_9.1);
_6.0 = 2339716276_u32 | 3531362300_u32;
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_8 = core::ptr::addr_of_mut!(_14);
_12.1.fld5 = -2054649282_i32;
Goto(bb3)
}
bb7 = {
SetDiscriminant(_12.0.1, 1);
Call(_12.0.3.0 = fn7(Move(_12.1.fld6), _6.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_12.1.fld0 = [_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5];
Goto(bb9)
}
bb9 = {
_9.1 = (RET,);
Goto(bb10)
}
bb10 = {
_12.0.0 = _15.0;
(*_11) = 5_usize >> _19;
_27 = &_12.1.fld0;
(*_11) = 1338153642660960213_usize;
_15.0 = (*_11) as u128;
RET = _9.1.0 + _21;
_12.0.0 = _15.0;
_12.0.3 = _15.3;
_15.2 = core::ptr::addr_of_mut!((*_27));
_12.0.0 = _15.0 ^ _15.0;
_22 = '\u{724d2}';
_24 = [41605_u16,15068_u16,4198_u16,60069_u16,25742_u16,55255_u16];
_12.0.2 = Move(_15.2);
_15.2 = Move(_12.0.2);
_26 = _7;
_18 = _12.2;
_8 = Move(_6.3);
_6 = (606824928_u32, _14, Field::<isize>(Variant(_15.1, 1), 1), Move(_8));
_28 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_26 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_2 = Move(_12.1.fld7);
_12.1.fld3 = -_12.0.3.0;
_17 = [_4,_1,_4,_1,_1];
Goto(bb11)
}
bb11 = {
_12.0 = (_15.0, Move(_15.1), Move(_15.2), _15.3);
_5 = Field::<isize>(Variant(_12.0.1, 1), 1);
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)) = (RET,);
_12.1.fld2 = _12.3 & _12.3;
_12.3 = 19244_u16 as u8;
_12.1.fld4 = -Field::<(f32,)>(Variant(_12.0.1, 1), 0).0;
_25 = &_33.fld5;
RET = _12.1.fld4;
_30 = [_12.1.fld2,_12.1.fld2,_12.3,_12.3,_12.1.fld2,_12.1.fld2,_12.1.fld2];
_33.fld5.fld0 = !10962792122469282962_u64;
Goto(bb12)
}
bb12 = {
_15.3 = (_12.1.fld3, _12.0.3.1);
_2 = core::ptr::addr_of!(_19);
_33.fld0 = _12.2;
_9.0.0 = _3 as f32;
_15.2 = Move(_12.0.2);
match _6.0 {
0 => bb13,
606824928 => bb15,
_ => bb14
}
}
bb13 = {
_12.1.fld0 = [_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5];
Goto(bb9)
}
bb14 = {
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)).0 = _9.0.0;
_15.0 = 12024389234842585716_u64 as u128;
_3 = _12.0.3.0;
_12.0.3.1 = ['\u{1dfbd}','\u{ab9dd}','\u{10d6be}','\u{7aa81}','\u{e3f5b}','\u{5c13}','\u{10bce1}'];
_12.0.3.1 = ['\u{b04e}','\u{b68}','\u{cb889}','\u{64934}','\u{1045d1}','\u{fcb8f}','\u{2a7db}'];
_12.3 = 135_u8 | 95_u8;
_12.1.fld0 = [_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5,_12.1.fld5];
RET = -_12.1.fld4;
place!(Field::<(f32,)>(Variant(_12.0.1, 1), 0)) = _9.1;
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_12.0.2 = core::ptr::addr_of_mut!(_12.1.fld0);
_15.3.1 = _12.0.3.1;
_15.3 = (_3, _12.0.3.1);
_12.1.fld1 = core::ptr::addr_of_mut!(_9.1);
_6.0 = 2339716276_u32 | 3531362300_u32;
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_7 = [_12.3,_12.3,_12.3,_12.3,_12.3,_12.3,_12.3];
_8 = core::ptr::addr_of_mut!(_14);
_12.1.fld5 = -2054649282_i32;
Goto(bb3)
}
bb15 = {
(*_11) = 5413469616355038923_usize;
(*_2) = -_6.2;
_33.fld7 = Move(_15.2);
_15.1 = Move(_12.0.1);
_13 = [_12.0.0,_12.0.0,_15.0];
_33.fld5.fld3 = (Field::<(f32,)>(Variant(_15.1, 1), 0).0,);
_34 = _6.2 - _5;
_12.1.fld6 = core::ptr::addr_of!((*_25).fld1);
_33.fld7 = core::ptr::addr_of_mut!((*_27));
SetDiscriminant(_15.1, 2);
_9.0.0 = -RET;
_33.fld5.fld1 = (_15.3.0, _15.3.1);
_15.3.1 = [_22,_22,_22,_22,_22,_22,_22];
place!(Field::<i32>(Variant(_15.1, 2), 5)) = -_12.1.fld5;
(*_11) = 7073511398420642703_usize;
place!(Field::<[i32; 7]>(Variant(_15.1, 2), 4)) = [Field::<i32>(Variant(_15.1, 2), 5),Field::<i32>(Variant(_15.1, 2), 5),_12.1.fld5,Field::<i32>(Variant(_15.1, 2), 5),_12.1.fld5,Field::<i32>(Variant(_15.1, 2), 5),_12.1.fld5];
_33.fld4 = Move(_6.3);
place!(Field::<(char, [u64; 4], u16)>(Variant(_15.1, 2), 2)).2 = !31878_u16;
place!(Field::<Adt24>(Variant(_15.1, 2), 3)) = Adt24::Variant1 { fld0: _30,fld1: (*_2) };
_4 = _1;
_11 = core::ptr::addr_of!(_20.0);
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(6_usize, 4_usize, Move(_4), 18_usize, Move(_18), 22_usize, Move(_22), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(6_usize, 34_usize, Move(_34), 17_usize, Move(_17), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const (f64, [char; 7]),mut _2: u32) -> f64 {
mir! {
type RET = f64;
let _3: u64;
let _4: (Adt32, &'static bool, [u8; 6], [bool; 2]);
let _5: isize;
let _6: u64;
let _7: [bool; 5];
let _8: u8;
let _9: i8;
let _10: *const isize;
let _11: Adt67;
let _12: f64;
let _13: f32;
let _14: &'static (u32, i128, isize, *mut i128);
let _15: u64;
let _16: isize;
let _17: u64;
let _18: isize;
let _19: bool;
let _20: ();
let _21: ();
{
RET = (-43127891831591272826663690862886536289_i128) as f64;
RET = 32872_u16 as f64;
_4.2 = [94_u8,106_u8,48_u8,233_u8,83_u8,174_u8];
_4.3 = [true,true];
_4.2 = [74_u8,211_u8,166_u8,249_u8,2_u8,123_u8];
_3 = !1810557630765404772_u64;
_4.2 = [56_u8,234_u8,247_u8,125_u8,41_u8,216_u8];
_3 = 52371882308581479883223937938141391238_u128 as u64;
_3 = 6994911789935477144_u64 & 9873784163763882292_u64;
_4.3 = [false,false];
_3 = (-8709122164005418661_i64) as u64;
_3 = 6443329213832641771_u64;
_4.3 = [true,false];
_3 = false as u64;
Call(_2 = fn8(Move(_1), _4.2, _4.2, RET, _4.2, RET, _3, _4.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 252972891385791087978366415589579929811_u128 as f64;
_3 = 15231434600998235386_u64 << _2;
_3 = 14956625791974664564_u64 + 12482154598330637993_u64;
_5 = 175752597982213640174426603287001379091_u128 as isize;
RET = (-126_i8) as f64;
_3 = 7500506395896392457_u64;
_6 = !_3;
_6 = '\u{75113}' as u64;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
7500506395896392457 => bb6,
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
RET = 18310473047842040382_usize as f64;
_3 = _6;
RET = _6 as f64;
_4.3 = [true,true];
_3 = !_6;
_2 = '\u{9ad74}' as u32;
RET = _3 as f64;
_4.2 = [2_u8,34_u8,217_u8,214_u8,66_u8,192_u8];
_3 = _6;
Goto(bb7)
}
bb7 = {
RET = 12922261506031633789_usize as f64;
_7 = [true,false,false,true,false];
_2 = !3354227961_u32;
RET = _5 as f64;
_3 = !_6;
Goto(bb8)
}
bb8 = {
RET = (-6957350097921748417_i64) as f64;
_5 = !9223372036854775807_isize;
_4.3 = [false,false];
_7 = [true,true,false,false,false];
_7 = [false,false,true,true,true];
_8 = 149_u8;
_3 = _6 + _6;
_2 = 333245573_u32;
_3 = 1776259805_i32 as u64;
_7 = [true,true,true,true,false];
_4.2 = [_8,_8,_8,_8,_8,_8];
_5 = !9223372036854775807_isize;
RET = 1596618783_i32 as f64;
_8 = RET as u8;
Goto(bb9)
}
bb9 = {
_8 = 65_u8 - 7_u8;
_4.3 = [true,false];
_3 = !_6;
_11.fld5 = 8076137413289992927_usize as i32;
_11.fld7 = core::ptr::addr_of!(_5);
_2 = 2421502593_u32 * 3010158474_u32;
_11.fld4 = 4873566826475664041_i64 as f32;
_5 = (-23_isize);
RET = (-1508750134237769103_i64) as f64;
_4.3 = [true,false];
_11.fld2 = !_8;
_2 = _11.fld5 as u32;
_11.fld4 = 64745_u16 as f32;
_5 = !(-41_isize);
_11.fld3 = RET * RET;
_9 = RET as i8;
_11.fld0 = [_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5];
_11.fld4 = 21715_u16 as f32;
_8 = _11.fld2;
_13 = -_11.fld4;
_4.3 = [false,true];
_11.fld0 = [_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5];
_11.fld3 = 6328920559247066124_usize as f64;
_4.3 = [true,false];
Goto(bb10)
}
bb10 = {
_15 = 101741213511525704466735426243088436264_u128 as u64;
Call(_11 = fn9(RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11.fld7 = core::ptr::addr_of!(_16);
_6 = !_15;
RET = _11.fld3;
_4.3 = [true,true];
Call(_11.fld6 = fn11(Move(_11.fld1), _11.fld4, _11.fld4, _11.fld4, _11.fld4, _11.fld4, _15, _11.fld4, _7), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10 = core::ptr::addr_of!(_5);
_11.fld0 = [_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5];
Call(_11.fld7 = fn15(_4.2, _11.fld0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_15 = _6 ^ _6;
_11.fld0 = [_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5,_11.fld5];
_11.fld5 = _9 as i32;
_4.2 = [_8,_11.fld2,_11.fld2,_8,_8,_11.fld2];
_2 = 1904309914_u32 - 1297867430_u32;
_17 = _6;
_10 = core::ptr::addr_of!((*_10));
Call(RET = core::intrinsics::transmute(_6), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_11.fld4 = _13;
_10 = Move(_11.fld7);
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(7_usize, 8_usize, Move(_8), 7_usize, Move(_7), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: *const (f64, [char; 7]),mut _2: [u8; 6],mut _3: [u8; 6],mut _4: f64,mut _5: [u8; 6],mut _6: f64,mut _7: u64,mut _8: [u8; 6]) -> u32 {
mir! {
type RET = u32;
let _9: f64;
let _10: &'static bool;
let _11: Adt59;
let _12: char;
let _13: i32;
let _14: bool;
let _15: (usize,);
let _16: Adt24;
let _17: isize;
let _18: f64;
let _19: isize;
let _20: *const isize;
let _21: Adt56;
let _22: Adt82;
let _23: [u64; 7];
let _24: &'static bool;
let _25: *mut [i32; 7];
let _26: ();
let _27: ();
{
RET = (-14_i8) as u32;
RET = !2796559315_u32;
_4 = _6;
Call(_6 = core::intrinsics::fmaf64(_4, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = !13005434036760366160_u64;
_7 = 11104_i16 as u64;
_12 = '\u{86c7b}';
_9 = _4;
_15.0 = 11910429503866958891_usize & 0_usize;
_15.0 = false as usize;
_14 = _6 < _4;
_10 = &_14;
_13 = _12 as i32;
_4 = -_6;
RET = (-78_i8) as u32;
_15 = (6816295442174463453_usize,);
Goto(bb2)
}
bb2 = {
_5 = _3;
_7 = 5731441101903866587_u64 + 14463275616427709488_u64;
_14 = !false;
_3 = _5;
_12 = '\u{81579}';
_10 = &_14;
RET = !466521465_u32;
_5 = [27_u8,213_u8,164_u8,78_u8,210_u8,128_u8];
_14 = false;
_2 = [235_u8,204_u8,17_u8,33_u8,250_u8,157_u8];
_3 = [6_u8,228_u8,237_u8,187_u8,71_u8,21_u8];
RET = (-6520470708610793424_i64) as u32;
_18 = _4 - _9;
_8 = [153_u8,32_u8,206_u8,178_u8,62_u8,169_u8];
_19 = !(-9223372036854775808_isize);
_6 = -_18;
_6 = _19 as f64;
_8 = _2;
_12 = '\u{e5691}';
_13 = RET as i32;
_4 = _9 * _6;
_20 = core::ptr::addr_of!(_19);
_15.0 = !0_usize;
_15 = (12282731745716294436_usize,);
RET = _15.0 as u32;
_2 = [215_u8,213_u8,231_u8,161_u8,106_u8,69_u8];
(*_20) = (-9223372036854775808_isize) + 9223372036854775807_isize;
match _15.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
12282731745716294436 => bb9,
_ => bb8
}
}
bb3 = {
_7 = !13005434036760366160_u64;
_7 = 11104_i16 as u64;
_12 = '\u{86c7b}';
_9 = _4;
_15.0 = 11910429503866958891_usize & 0_usize;
_15.0 = false as usize;
_14 = _6 < _4;
_10 = &_14;
_13 = _12 as i32;
_4 = -_6;
RET = (-78_i8) as u32;
_15 = (6816295442174463453_usize,);
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
_18 = -_4;
_13 = -421731426_i32;
_7 = !16407568680740523822_u64;
(*_20) = 56588707195902293297696082367590298610_i128 as isize;
_21.fld0 = !_7;
_16 = Adt24::Variant2 { fld0: 80479914465932208660239213611968035413_i128,fld1: _12,fld2: (*_20),fld3: (-117_i8),fld4: _15.0,fld5: _13 };
RET = !4129012366_u32;
place!(Field::<char>(Variant(_16, 2), 1)) = _12;
_10 = &_14;
_21.fld3.0 = 6621715633301932887_i64 as f32;
_21.fld2 = core::ptr::addr_of!(_17);
place!(Field::<char>(Variant(_16, 2), 1)) = _12;
_1 = core::ptr::addr_of!(_21.fld1);
_1 = core::ptr::addr_of!((*_1));
(*_1).1 = [Field::<char>(Variant(_16, 2), 1),_12,Field::<char>(Variant(_16, 2), 1),_12,_12,_12,Field::<char>(Variant(_16, 2), 1)];
(*_20) = !Field::<isize>(Variant(_16, 2), 2);
_12 = Field::<char>(Variant(_16, 2), 1);
_4 = _18 - _6;
_7 = !_21.fld0;
_6 = _4 + _9;
_19 = Field::<isize>(Variant(_16, 2), 2);
place!(Field::<i128>(Variant(_16, 2), 0)) = 125_u8 as i128;
match Field::<usize>(Variant(_16, 2), 4) {
0 => bb1,
1 => bb8,
2 => bb10,
3 => bb11,
4 => bb12,
12282731745716294436 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_7 = !13005434036760366160_u64;
_7 = 11104_i16 as u64;
_12 = '\u{86c7b}';
_9 = _4;
_15.0 = 11910429503866958891_usize & 0_usize;
_15.0 = false as usize;
_14 = _6 < _4;
_10 = &_14;
_13 = _12 as i32;
_4 = -_6;
RET = (-78_i8) as u32;
_15 = (6816295442174463453_usize,);
Goto(bb2)
}
bb13 = {
_5 = _3;
_7 = 5731441101903866587_u64 + 14463275616427709488_u64;
_14 = !false;
_3 = _5;
_12 = '\u{81579}';
_10 = &_14;
RET = !466521465_u32;
_5 = [27_u8,213_u8,164_u8,78_u8,210_u8,128_u8];
_14 = false;
_2 = [235_u8,204_u8,17_u8,33_u8,250_u8,157_u8];
_3 = [6_u8,228_u8,237_u8,187_u8,71_u8,21_u8];
RET = (-6520470708610793424_i64) as u32;
_18 = _4 - _9;
_8 = [153_u8,32_u8,206_u8,178_u8,62_u8,169_u8];
_19 = !(-9223372036854775808_isize);
_6 = -_18;
_6 = _19 as f64;
_8 = _2;
_12 = '\u{e5691}';
_13 = RET as i32;
_4 = _9 * _6;
_20 = core::ptr::addr_of!(_19);
_15.0 = !0_usize;
_15 = (12282731745716294436_usize,);
RET = _15.0 as u32;
_2 = [215_u8,213_u8,231_u8,161_u8,106_u8,69_u8];
(*_20) = (-9223372036854775808_isize) + 9223372036854775807_isize;
match _15.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
12282731745716294436 => bb9,
_ => bb8
}
}
bb14 = {
_6 = _4 * _4;
place!(Field::<i8>(Variant(_16, 2), 3)) = 99_i8 << _7;
_17 = 38651_u16 as isize;
place!(Field::<i8>(Variant(_16, 2), 3)) = 207_u8 as i8;
_23 = [_7,_21.fld0,_7,_21.fld0,_7,_7,_21.fld0];
place!(Field::<usize>(Variant(_16, 2), 4)) = _15.0;
place!(Field::<i128>(Variant(_16, 2), 0)) = 21092_i16 as i128;
(*_20) = -Field::<isize>(Variant(_16, 2), 2);
(*_1).0 = _18 - _6;
place!(Field::<i32>(Variant(_16, 2), 5)) = _13;
_21.fld3.0 = _15.0 as f32;
_12 = Field::<char>(Variant(_16, 2), 1);
_21.fld1.0 = _18;
_16 = Adt24::Variant2 { fld0: 86715659615438848390803821594458123963_i128,fld1: _12,fld2: (*_20),fld3: (-28_i8),fld4: _15.0,fld5: _13 };
_16 = Adt24::Variant0 { fld0: 2402_u16,fld1: _21.fld0,fld2: _21.fld3.0,fld3: _21.fld3 };
(*_20) = !_17;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(8_usize, 3_usize, Move(_3), 5_usize, Move(_5), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(8_usize, 12_usize, Move(_12), 7_usize, Move(_7), 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: f64) -> Adt67 {
mir! {
type RET = Adt67;
let _2: f32;
let _3: u16;
let _4: i32;
let _5: char;
let _6: u16;
let _7: (((f32,), (f32,), [bool; 2]), (u128,), *mut [char; 7]);
let _8: u8;
let _9: f64;
let _10: *const usize;
let _11: *const i128;
let _12: i128;
let _13: u64;
let _14: [bool; 2];
let _15: bool;
let _16: f64;
let _17: Adt68;
let _18: usize;
let _19: (usize,);
let _20: &'static [i32; 7];
let _21: isize;
let _22: *mut usize;
let _23: (u128,);
let _24: *const usize;
let _25: (*mut i128, *const (f64, [char; 7]), u32, (u128, Adt32, *mut [i32; 7], (f64, [char; 7])));
let _26: ();
let _27: ();
{
RET.fld3 = -_1;
_2 = 2524693771822962105_usize as f32;
RET.fld0 = [928212566_i32,(-1493600187_i32),712646849_i32,1470156960_i32,1521164091_i32,(-382049870_i32),(-1891205653_i32)];
RET.fld4 = _2 * _2;
RET.fld3 = 2_usize as f64;
RET.fld0 = [1022795826_i32,(-1973525758_i32),(-1894577974_i32),985381213_i32,(-288286036_i32),1425733715_i32,(-1011389329_i32)];
_2 = RET.fld4 + RET.fld4;
RET.fld4 = -_2;
RET.fld2 = 203_u8;
RET.fld3 = _1;
RET.fld0 = [2084786728_i32,(-287763396_i32),(-1435174183_i32),(-477653622_i32),(-1818201122_i32),1085828783_i32,628548267_i32];
RET.fld2 = 198_u8;
match RET.fld2 {
0 => bb1,
198 => bb3,
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
RET.fld4 = _2;
RET.fld0 = [(-184276018_i32),(-290439292_i32),(-2124713919_i32),(-1716191094_i32),(-1961640303_i32),(-1011010958_i32),(-1950678543_i32)];
RET.fld4 = (-1402769747_i32) as f32;
RET.fld3 = _1 + _1;
RET.fld5 = 1100955521_i32;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
_2 = -RET.fld4;
_3 = 2279_u16;
RET.fld5 = 721940597_i32 | (-1477577142_i32);
_1 = RET.fld3;
RET.fld3 = _1 * _1;
RET.fld4 = _2 - _2;
_2 = -RET.fld4;
RET.fld2 = 95_u8 << RET.fld5;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld2 = (-970250518423986148_i64) as u8;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld5 = _2 as i32;
RET.fld2 = 81_u8;
_1 = RET.fld3 + RET.fld3;
_4 = !RET.fld5;
_6 = RET.fld2 as u16;
_2 = RET.fld4;
_7.1 = (233888683999092120602671286970471904026_u128,);
_7.0.0 = (RET.fld4,);
RET.fld0 = [RET.fld5,_4,_4,RET.fld5,RET.fld5,_4,RET.fld5];
Goto(bb4)
}
bb4 = {
_5 = '\u{7e8f4}';
_7.1.0 = (-30596_i16) as u128;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
RET.fld4 = _7.0.0.0 * _7.0.0.0;
RET.fld5 = _4 << RET.fld2;
RET.fld5 = _4 - _4;
_7.0.0.0 = RET.fld4 + RET.fld4;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
RET.fld0 = [RET.fld5,_4,RET.fld5,RET.fld5,_4,RET.fld5,RET.fld5];
RET.fld3 = _1;
_6 = _3;
_3 = _6;
_5 = '\u{1211a}';
_7.0.1 = (RET.fld4,);
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,_4,_4];
Goto(bb5)
}
bb5 = {
_7.1 = (64983958192042078205959038181775386299_u128,);
RET.fld5 = 2_i8 as i32;
RET.fld2 = 114_u8 >> _7.1.0;
RET.fld5 = 33_isize as i32;
_4 = -RET.fld5;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
_2 = 41_i8 as f32;
_6 = _3;
RET.fld2 = !210_u8;
_2 = _7.0.0.0 - _7.0.0.0;
RET.fld3 = _1 + _1;
RET.fld0 = [_4,RET.fld5,RET.fld5,_4,_4,_4,RET.fld5];
_9 = RET.fld3;
RET.fld3 = _1 - _1;
_9 = 28197_i16 as f64;
RET.fld3 = _4 as f64;
RET.fld3 = -_1;
RET.fld0 = [RET.fld5,RET.fld5,_4,_4,_4,_4,_4];
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld2 = 196_u8 - 76_u8;
_8 = RET.fld2 >> RET.fld2;
Goto(bb6)
}
bb6 = {
RET.fld3 = -_1;
match _7.1.0 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
6 => bb9,
64983958192042078205959038181775386299 => bb11,
_ => bb10
}
}
bb7 = {
_7.1 = (64983958192042078205959038181775386299_u128,);
RET.fld5 = 2_i8 as i32;
RET.fld2 = 114_u8 >> _7.1.0;
RET.fld5 = 33_isize as i32;
_4 = -RET.fld5;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
_2 = 41_i8 as f32;
_6 = _3;
RET.fld2 = !210_u8;
_2 = _7.0.0.0 - _7.0.0.0;
RET.fld3 = _1 + _1;
RET.fld0 = [_4,RET.fld5,RET.fld5,_4,_4,_4,RET.fld5];
_9 = RET.fld3;
RET.fld3 = _1 - _1;
_9 = 28197_i16 as f64;
RET.fld3 = _4 as f64;
RET.fld3 = -_1;
RET.fld0 = [RET.fld5,RET.fld5,_4,_4,_4,_4,_4];
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld2 = 196_u8 - 76_u8;
_8 = RET.fld2 >> RET.fld2;
Goto(bb6)
}
bb8 = {
_5 = '\u{7e8f4}';
_7.1.0 = (-30596_i16) as u128;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
RET.fld4 = _7.0.0.0 * _7.0.0.0;
RET.fld5 = _4 << RET.fld2;
RET.fld5 = _4 - _4;
_7.0.0.0 = RET.fld4 + RET.fld4;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
RET.fld0 = [RET.fld5,_4,RET.fld5,RET.fld5,_4,RET.fld5,RET.fld5];
RET.fld3 = _1;
_6 = _3;
_3 = _6;
_5 = '\u{1211a}';
_7.0.1 = (RET.fld4,);
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,_4,_4];
Goto(bb5)
}
bb9 = {
RET.fld4 = _2;
RET.fld0 = [(-184276018_i32),(-290439292_i32),(-2124713919_i32),(-1716191094_i32),(-1961640303_i32),(-1011010958_i32),(-1950678543_i32)];
RET.fld4 = (-1402769747_i32) as f32;
RET.fld3 = _1 + _1;
RET.fld5 = 1100955521_i32;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
_2 = -RET.fld4;
_3 = 2279_u16;
RET.fld5 = 721940597_i32 | (-1477577142_i32);
_1 = RET.fld3;
RET.fld3 = _1 * _1;
RET.fld4 = _2 - _2;
_2 = -RET.fld4;
RET.fld2 = 95_u8 << RET.fld5;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld2 = (-970250518423986148_i64) as u8;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld5 = _2 as i32;
RET.fld2 = 81_u8;
_1 = RET.fld3 + RET.fld3;
_4 = !RET.fld5;
_6 = RET.fld2 as u16;
_2 = RET.fld4;
_7.1 = (233888683999092120602671286970471904026_u128,);
_7.0.0 = (RET.fld4,);
RET.fld0 = [RET.fld5,_4,_4,RET.fld5,RET.fld5,_4,RET.fld5];
Goto(bb4)
}
bb10 = {
Return()
}
bb11 = {
_6 = _3 % _3;
_4 = RET.fld5 << _6;
_7.1.0 = !230729361830952777657317338856460603623_u128;
RET.fld2 = _7.1.0 as u8;
RET.fld5 = _4;
_7.0.0 = (_2,);
_9 = RET.fld3;
_7.0.2 = [true,true];
_2 = RET.fld4 + _7.0.0.0;
_4 = true as i32;
_4 = !RET.fld5;
Goto(bb12)
}
bb12 = {
_2 = -_7.0.0.0;
_4 = RET.fld5;
_1 = RET.fld3;
_5 = '\u{ef2d9}';
match _3 {
0 => bb2,
1 => bb13,
2 => bb14,
3 => bb15,
2279 => bb17,
_ => bb16
}
}
bb13 = {
_7.1 = (64983958192042078205959038181775386299_u128,);
RET.fld5 = 2_i8 as i32;
RET.fld2 = 114_u8 >> _7.1.0;
RET.fld5 = 33_isize as i32;
_4 = -RET.fld5;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
_2 = 41_i8 as f32;
_6 = _3;
RET.fld2 = !210_u8;
_2 = _7.0.0.0 - _7.0.0.0;
RET.fld3 = _1 + _1;
RET.fld0 = [_4,RET.fld5,RET.fld5,_4,_4,_4,RET.fld5];
_9 = RET.fld3;
RET.fld3 = _1 - _1;
_9 = 28197_i16 as f64;
RET.fld3 = _4 as f64;
RET.fld3 = -_1;
RET.fld0 = [RET.fld5,RET.fld5,_4,_4,_4,_4,_4];
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld2 = 196_u8 - 76_u8;
_8 = RET.fld2 >> RET.fld2;
Goto(bb6)
}
bb14 = {
Return()
}
bb15 = {
RET.fld4 = _2;
RET.fld0 = [(-184276018_i32),(-290439292_i32),(-2124713919_i32),(-1716191094_i32),(-1961640303_i32),(-1011010958_i32),(-1950678543_i32)];
RET.fld4 = (-1402769747_i32) as f32;
RET.fld3 = _1 + _1;
RET.fld5 = 1100955521_i32;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
_2 = -RET.fld4;
_3 = 2279_u16;
RET.fld5 = 721940597_i32 | (-1477577142_i32);
_1 = RET.fld3;
RET.fld3 = _1 * _1;
RET.fld4 = _2 - _2;
_2 = -RET.fld4;
RET.fld2 = 95_u8 << RET.fld5;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld2 = (-970250518423986148_i64) as u8;
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5];
RET.fld5 = _2 as i32;
RET.fld2 = 81_u8;
_1 = RET.fld3 + RET.fld3;
_4 = !RET.fld5;
_6 = RET.fld2 as u16;
_2 = RET.fld4;
_7.1 = (233888683999092120602671286970471904026_u128,);
_7.0.0 = (RET.fld4,);
RET.fld0 = [RET.fld5,_4,_4,RET.fld5,RET.fld5,_4,RET.fld5];
Goto(bb4)
}
bb16 = {
_5 = '\u{7e8f4}';
_7.1.0 = (-30596_i16) as u128;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
RET.fld4 = _7.0.0.0 * _7.0.0.0;
RET.fld5 = _4 << RET.fld2;
RET.fld5 = _4 - _4;
_7.0.0.0 = RET.fld4 + RET.fld4;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.1);
RET.fld0 = [RET.fld5,_4,RET.fld5,RET.fld5,_4,RET.fld5,RET.fld5];
RET.fld3 = _1;
_6 = _3;
_3 = _6;
_5 = '\u{1211a}';
_7.0.1 = (RET.fld4,);
RET.fld0 = [RET.fld5,RET.fld5,RET.fld5,RET.fld5,RET.fld5,_4,_4];
Goto(bb5)
}
bb17 = {
_8 = !RET.fld2;
_9 = (-121894893804011097658492392491561161671_i128) as f64;
_9 = _1;
_7.0.2 = [true,false];
_12 = false as i128;
RET.fld0 = [RET.fld5,RET.fld5,_4,_4,RET.fld5,_4,_4];
_5 = '\u{d57ed}';
_6 = _3;
_7.0.2 = [true,false];
RET.fld4 = _12 as f32;
Goto(bb18)
}
bb18 = {
_8 = (-114_isize) as u8;
_7.0.1.0 = _7.0.0.0 + _7.0.0.0;
_7.0.0.0 = _2 + _2;
_12 = (-91112106402194385103410733821647195031_i128);
RET.fld3 = 1563_i16 as f64;
RET.fld3 = -_9;
RET.fld1 = core::ptr::addr_of_mut!(_7.0.0);
RET.fld5 = !_4;
Goto(bb19)
}
bb19 = {
RET.fld1 = core::ptr::addr_of_mut!(_7.0.0);
_1 = -RET.fld3;
_7.0.1 = _7.0.0;
RET.fld1 = core::ptr::addr_of_mut!(_17.fld5.fld3);
_13 = 1124295683609397864_u64 | 11811486275205411421_u64;
_17.fld5.fld3 = _7.0.1;
_13 = !8953113781448933674_u64;
RET.fld7 = core::ptr::addr_of!(_17.fld2);
RET.fld3 = (-9223372036854775808_isize) as f64;
_7.0.2 = [true,true];
_17.fld1 = core::ptr::addr_of!(_12);
_15 = !true;
_3 = _6 ^ _6;
_8 = RET.fld2 - RET.fld2;
_17.fld6 = [_5,_5,_5];
_20 = &_17.fld3;
_11 = Move(_17.fld1);
_17.fld3 = RET.fld0;
RET.fld7 = core::ptr::addr_of!(_21);
_7.0.1 = _7.0.0;
_8 = RET.fld2 - RET.fld2;
_17.fld7 = core::ptr::addr_of_mut!(_17.fld3);
_17.fld5.fld3.0 = -_7.0.1.0;
RET.fld4 = _17.fld5.fld3.0;
Call(_7.2 = fn10(_17.fld3, _17.fld5.fld3), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_1 = -_9;
_17.fld1 = core::ptr::addr_of!(_12);
_17.fld6 = [_5,_5,_5];
_6 = _7.0.1.0 as u16;
_21 = (-126_isize);
RET.fld1 = core::ptr::addr_of_mut!(_17.fld5.fld3);
_17.fld5.fld3 = (_7.0.1.0,);
_17.fld0 = !_15;
_12 = 0_usize as i128;
_7.0.0.0 = _6 as f32;
_7.0.0 = (RET.fld4,);
RET.fld6 = core::ptr::addr_of!(_17.fld5.fld1);
_24 = core::ptr::addr_of!(_18);
_17.fld5.fld0 = _13;
RET.fld3 = -_1;
_16 = -_1;
(*_24) = 7_usize;
_7.0.0.0 = _17.fld5.fld3.0;
Goto(bb21)
}
bb21 = {
Call(_26 = dump_var(9_usize, 15_usize, Move(_15), 13_usize, Move(_13), 12_usize, Move(_12), 21_usize, Move(_21)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_26 = dump_var(9_usize, 3_usize, Move(_3), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i32; 7],mut _2: (f32,)) -> *mut [char; 7] {
mir! {
type RET = *mut [char; 7];
let _3: i8;
let _4: f32;
let _5: f32;
let _6: [u8; 1];
let _7: Adt32;
let _8: isize;
let _9: isize;
let _10: char;
let _11: u8;
let _12: *mut [i32; 7];
let _13: i8;
let _14: u16;
let _15: [u8; 1];
let _16: (*mut i128, *const (f64, [char; 7]), u32, (u128, Adt32, *mut [i32; 7], (f64, [char; 7])));
let _17: (usize,);
let _18: [u8; 7];
let _19: [u64; 4];
let _20: i16;
let _21: i32;
let _22: char;
let _23: u8;
let _24: f64;
let _25: &'static (u32, i128, isize, *mut i128);
let _26: &'static bool;
let _27: [u16; 4];
let _28: [i16; 2];
let _29: i8;
let _30: [u16; 6];
let _31: Adt24;
let _32: (u128,);
let _33: ();
let _34: ();
{
_1 = [1248185701_i32,1337289488_i32,1974318800_i32,2040528768_i32,(-1247159738_i32),870253319_i32,(-1914213450_i32)];
_1 = [(-1446824007_i32),1931679115_i32,1064568131_i32,(-1982514255_i32),1450713386_i32,(-1633954327_i32),433028046_i32];
_1 = [(-1438218119_i32),(-974543637_i32),597549853_i32,1097091228_i32,(-1685375058_i32),811640758_i32,(-1663881470_i32)];
_1 = [651612346_i32,1001186473_i32,1187966459_i32,1443235899_i32,1930130549_i32,(-246884138_i32),840203343_i32];
_1 = [1463582798_i32,2132180408_i32,(-265544227_i32),(-502325885_i32),(-1487526091_i32),(-890070056_i32),(-1087079796_i32)];
_1 = [(-929544909_i32),96712934_i32,1059727702_i32,(-1187924912_i32),2090924664_i32,(-1889728971_i32),1598805134_i32];
Goto(bb1)
}
bb1 = {
_1 = [997302674_i32,1163527477_i32,(-1462443038_i32),(-613399944_i32),1203083502_i32,(-353733513_i32),2062982427_i32];
_1 = [(-263674646_i32),(-599892918_i32),(-2101273137_i32),1799730023_i32,(-881943059_i32),818434812_i32,1480135329_i32];
_2.0 = 92_u8 as f32;
_2.0 = 4127_u16 as f32;
_1 = [(-1691949543_i32),394686661_i32,(-1633546187_i32),(-666317387_i32),689288100_i32,(-1838457870_i32),(-1992052186_i32)];
_2.0 = (-33_i8) as f32;
_1 = [(-2106725247_i32),(-1873329277_i32),(-1142724391_i32),1901053634_i32,(-1748217494_i32),913918103_i32,1205405321_i32];
_1 = [1001330350_i32,(-449839868_i32),(-1643172366_i32),1053642175_i32,1494276327_i32,(-1857910143_i32),599186401_i32];
_3 = (-30_i8) & (-115_i8);
_1 = [(-611379132_i32),(-1940686674_i32),(-1489782234_i32),2080591351_i32,(-929402211_i32),(-2140872646_i32),1019800977_i32];
_1 = [(-1795913581_i32),391139654_i32,193966752_i32,520280603_i32,1629462147_i32,1537339199_i32,(-1025877595_i32)];
_4 = _2.0 - _2.0;
_1 = [(-379117423_i32),(-1560650390_i32),(-1612124309_i32),(-1374655323_i32),800117438_i32,(-1170912244_i32),(-1671621287_i32)];
_4 = -_2.0;
_2 = (_4,);
_2.0 = -_4;
_4 = _2.0;
_3 = !(-80_i8);
_2.0 = _4 * _4;
_4 = _3 as f32;
_3 = (-94_i8) + 70_i8;
_2.0 = _4;
_1 = [(-2025381504_i32),(-818019975_i32),(-1166370356_i32),835308423_i32,(-396205974_i32),1351453963_i32,(-403475405_i32)];
_3 = (-108_i8) << (-79305833267367543829152372729502402570_i128);
_4 = _2.0 + _2.0;
_4 = -_2.0;
Call(_3 = core::intrinsics::bswap(78_i8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = (_4,);
_5 = 1830409775_u32 as f32;
_2.0 = 2323422596_u32 as f32;
_5 = -_2.0;
_6 = [243_u8];
_5 = 4041322294_u32 as f32;
_6 = [153_u8];
_7 = Adt32::Variant1 { fld0: _2,fld1: 9223372036854775807_isize };
_9 = -(-9223372036854775808_isize);
_8 = 48_u8 as isize;
Goto(bb3)
}
bb3 = {
_3 = 39_i8 * 119_i8;
place!(Field::<isize>(Variant(_7, 1), 1)) = -_9;
_8 = !Field::<isize>(Variant(_7, 1), 1);
_4 = -_5;
_7 = Adt32::Variant1 { fld0: _2,fld1: _8 };
SetDiscriminant(_7, 1);
Goto(bb4)
}
bb4 = {
_4 = _5;
place!(Field::<(f32,)>(Variant(_7, 1), 0)).0 = _9 as f32;
_2.0 = -_5;
_11 = !6_u8;
_6 = [_11];
_10 = '\u{9ef}';
_12 = core::ptr::addr_of_mut!(_1);
_6 = [_11];
_6 = [_11];
_14 = 63140_u16;
place!(Field::<isize>(Variant(_7, 1), 1)) = _2.0 as isize;
_14 = 37864_u16;
_14 = 47351_u16 ^ 12304_u16;
SetDiscriminant(_7, 0);
place!(Field::<(usize,)>(Variant(_7, 0), 0)) = (0_usize,);
_11 = 18_u8 >> _8;
place!(Field::<i8>(Variant(_7, 0), 3)) = _3 & _3;
place!(Field::<(char, [u64; 4], u16)>(Variant(_7, 0), 2)).2 = !_14;
_1 = [1216599601_i32,17532246_i32,(-285663012_i32),1481341828_i32,632312673_i32,541168853_i32,1226936037_i32];
_2.0 = 28354_i16 as f32;
place!(Field::<(usize,)>(Variant(_7, 0), 0)).0 = 0_usize & 4756124165018023560_usize;
Call(place!(Field::<u16>(Variant(_7, 0), 1)) = core::intrinsics::bswap(_14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<u16>(Variant(_7, 0), 1)) = _14 + _14;
place!(Field::<i8>(Variant(_7, 0), 3)) = _3 ^ _3;
place!(Field::<i8>(Variant(_7, 0), 3)) = _3;
_16.3.2 = Move(_12);
_15 = [_11];
_17 = (Field::<(usize,)>(Variant(_7, 0), 0).0,);
_9 = -_8;
place!(Field::<(char, [u64; 4], u16)>(Variant(_7, 0), 2)).1 = [863772880303759897_u64,18235531682740853390_u64,14100926258716175382_u64,17230292191229145272_u64];
place!(Field::<(f32,)>(Variant(_7, 0), 4)) = _2;
_15 = [_11];
_2 = Field::<(f32,)>(Variant(_7, 0), 4);
_16.1 = core::ptr::addr_of!(_16.3.3);
_16.3.3.1 = [_10,_10,_10,_10,_10,_10,_10];
_20 = 19663_i16;
_5 = _4;
_21 = 194996915_i32;
place!(Field::<(f32,)>(Variant(_7, 0), 4)).0 = _11 as f32;
_9 = _8;
Goto(bb6)
}
bb6 = {
place!(Field::<u16>(Variant(_7, 0), 1)) = _14;
_17.0 = 92352109645506613255624027040868263143_i128 as usize;
_20 = -(-4369_i16);
place!(Field::<(usize,)>(Variant(_7, 0), 0)) = _17;
_13 = Field::<i8>(Variant(_7, 0), 3) + Field::<i8>(Variant(_7, 0), 3);
_13 = _11 as i8;
place!(Field::<(char, [u64; 4], u16)>(Variant(_7, 0), 2)).2 = Field::<u16>(Variant(_7, 0), 1) & _14;
RET = core::ptr::addr_of_mut!(_16.3.3.1);
Goto(bb7)
}
bb7 = {
_16.3.1 = Adt32::Variant1 { fld0: Field::<(f32,)>(Variant(_7, 0), 4),fld1: _8 };
place!(Field::<u16>(Variant(_7, 0), 1)) = 94530778920325330990506466507345798481_i128 as u16;
_5 = Field::<(f32,)>(Variant(_7, 0), 4).0;
place!(Field::<(f32,)>(Variant(_16.3.1, 1), 0)).0 = _2.0 - _4;
_16.3.3.0 = 167096770_u32 as f64;
_9 = Field::<isize>(Variant(_16.3.1, 1), 1) * _8;
_7 = Move(_16.3.1);
_19 = [8923802483688804489_u64,6960161508725991188_u64,16469522067652420527_u64,723493709547543705_u64];
RET = core::ptr::addr_of_mut!((*RET));
_24 = 9433617295798755667_u64 as f64;
(*RET) = [_10,_10,_10,_10,_10,_10,_10];
_30 = [_14,_14,_14,_14,_14,_14];
_16.2 = !4128314429_u32;
match _21 {
0 => bb1,
1 => bb4,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
194996915 => bb14,
_ => bb13
}
}
bb8 = {
place!(Field::<u16>(Variant(_7, 0), 1)) = _14;
_17.0 = 92352109645506613255624027040868263143_i128 as usize;
_20 = -(-4369_i16);
place!(Field::<(usize,)>(Variant(_7, 0), 0)) = _17;
_13 = Field::<i8>(Variant(_7, 0), 3) + Field::<i8>(Variant(_7, 0), 3);
_13 = _11 as i8;
place!(Field::<(char, [u64; 4], u16)>(Variant(_7, 0), 2)).2 = Field::<u16>(Variant(_7, 0), 1) & _14;
RET = core::ptr::addr_of_mut!(_16.3.3.1);
Goto(bb7)
}
bb9 = {
place!(Field::<u16>(Variant(_7, 0), 1)) = _14 + _14;
place!(Field::<i8>(Variant(_7, 0), 3)) = _3 ^ _3;
place!(Field::<i8>(Variant(_7, 0), 3)) = _3;
_16.3.2 = Move(_12);
_15 = [_11];
_17 = (Field::<(usize,)>(Variant(_7, 0), 0).0,);
_9 = -_8;
place!(Field::<(char, [u64; 4], u16)>(Variant(_7, 0), 2)).1 = [863772880303759897_u64,18235531682740853390_u64,14100926258716175382_u64,17230292191229145272_u64];
place!(Field::<(f32,)>(Variant(_7, 0), 4)) = _2;
_15 = [_11];
_2 = Field::<(f32,)>(Variant(_7, 0), 4);
_16.1 = core::ptr::addr_of!(_16.3.3);
_16.3.3.1 = [_10,_10,_10,_10,_10,_10,_10];
_20 = 19663_i16;
_5 = _4;
_21 = 194996915_i32;
place!(Field::<(f32,)>(Variant(_7, 0), 4)).0 = _11 as f32;
_9 = _8;
Goto(bb6)
}
bb10 = {
_4 = _5;
place!(Field::<(f32,)>(Variant(_7, 1), 0)).0 = _9 as f32;
_2.0 = -_5;
_11 = !6_u8;
_6 = [_11];
_10 = '\u{9ef}';
_12 = core::ptr::addr_of_mut!(_1);
_6 = [_11];
_6 = [_11];
_14 = 63140_u16;
place!(Field::<isize>(Variant(_7, 1), 1)) = _2.0 as isize;
_14 = 37864_u16;
_14 = 47351_u16 ^ 12304_u16;
SetDiscriminant(_7, 0);
place!(Field::<(usize,)>(Variant(_7, 0), 0)) = (0_usize,);
_11 = 18_u8 >> _8;
place!(Field::<i8>(Variant(_7, 0), 3)) = _3 & _3;
place!(Field::<(char, [u64; 4], u16)>(Variant(_7, 0), 2)).2 = !_14;
_1 = [1216599601_i32,17532246_i32,(-285663012_i32),1481341828_i32,632312673_i32,541168853_i32,1226936037_i32];
_2.0 = 28354_i16 as f32;
place!(Field::<(usize,)>(Variant(_7, 0), 0)).0 = 0_usize & 4756124165018023560_usize;
Call(place!(Field::<u16>(Variant(_7, 0), 1)) = core::intrinsics::bswap(_14), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_3 = 39_i8 * 119_i8;
place!(Field::<isize>(Variant(_7, 1), 1)) = -_9;
_8 = !Field::<isize>(Variant(_7, 1), 1);
_4 = -_5;
_7 = Adt32::Variant1 { fld0: _2,fld1: _8 };
SetDiscriminant(_7, 1);
Goto(bb4)
}
bb12 = {
_2 = (_4,);
_5 = 1830409775_u32 as f32;
_2.0 = 2323422596_u32 as f32;
_5 = -_2.0;
_6 = [243_u8];
_5 = 4041322294_u32 as f32;
_6 = [153_u8];
_7 = Adt32::Variant1 { fld0: _2,fld1: 9223372036854775807_isize };
_9 = -(-9223372036854775808_isize);
_8 = 48_u8 as isize;
Goto(bb3)
}
bb13 = {
_1 = [997302674_i32,1163527477_i32,(-1462443038_i32),(-613399944_i32),1203083502_i32,(-353733513_i32),2062982427_i32];
_1 = [(-263674646_i32),(-599892918_i32),(-2101273137_i32),1799730023_i32,(-881943059_i32),818434812_i32,1480135329_i32];
_2.0 = 92_u8 as f32;
_2.0 = 4127_u16 as f32;
_1 = [(-1691949543_i32),394686661_i32,(-1633546187_i32),(-666317387_i32),689288100_i32,(-1838457870_i32),(-1992052186_i32)];
_2.0 = (-33_i8) as f32;
_1 = [(-2106725247_i32),(-1873329277_i32),(-1142724391_i32),1901053634_i32,(-1748217494_i32),913918103_i32,1205405321_i32];
_1 = [1001330350_i32,(-449839868_i32),(-1643172366_i32),1053642175_i32,1494276327_i32,(-1857910143_i32),599186401_i32];
_3 = (-30_i8) & (-115_i8);
_1 = [(-611379132_i32),(-1940686674_i32),(-1489782234_i32),2080591351_i32,(-929402211_i32),(-2140872646_i32),1019800977_i32];
_1 = [(-1795913581_i32),391139654_i32,193966752_i32,520280603_i32,1629462147_i32,1537339199_i32,(-1025877595_i32)];
_4 = _2.0 - _2.0;
_1 = [(-379117423_i32),(-1560650390_i32),(-1612124309_i32),(-1374655323_i32),800117438_i32,(-1170912244_i32),(-1671621287_i32)];
_4 = -_2.0;
_2 = (_4,);
_2.0 = -_4;
_4 = _2.0;
_3 = !(-80_i8);
_2.0 = _4 * _4;
_4 = _3 as f32;
_3 = (-94_i8) + 70_i8;
_2.0 = _4;
_1 = [(-2025381504_i32),(-818019975_i32),(-1166370356_i32),835308423_i32,(-396205974_i32),1351453963_i32,(-403475405_i32)];
_3 = (-108_i8) << (-79305833267367543829152372729502402570_i128);
_4 = _2.0 + _2.0;
_4 = -_2.0;
Call(_3 = core::intrinsics::bswap(78_i8), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_30 = [_14,_14,_14,_14,_14,_14];
_21 = (-2030761181_i32);
_16.2 = 330026945_u32 - 1462020808_u32;
_15 = [_11];
_2 = (_5,);
_16.3.2 = core::ptr::addr_of_mut!(_1);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(10_usize, 8_usize, Move(_8), 21_usize, Move(_21), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(10_usize, 11_usize, Move(_11), 1_usize, Move(_1), 20_usize, Move(_20), 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut (f32,),mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: u64,mut _8: f32,mut _9: [bool; 5]) -> *const (f64, [char; 7]) {
mir! {
type RET = *const (f64, [char; 7]);
let _10: u32;
let _11: &'static [u128; 8];
let _12: Adt67;
let _13: Adt68;
let _14: [u128; 8];
let _15: f32;
let _16: i128;
let _17: &'static (f64, [char; 7]);
let _18: u64;
let _19: *mut usize;
let _20: [u64; 7];
let _21: [i16; 5];
let _22: &'static [i32; 7];
let _23: ((u128, Adt32, *mut [i32; 7], (f64, [char; 7])), Adt67, bool, u8);
let _24: ();
let _25: ();
{
_9 = [false,true,false,true,true];
_8 = _5;
_5 = _8;
_6 = -_8;
_7 = !8575305388805369655_u64;
_8 = _2;
_9 = [true,true,true,true,true];
_9 = [false,true,true,true,false];
_6 = 64427556153845383493861203122769037866_u128 as f32;
Goto(bb1)
}
bb1 = {
_8 = _3 - _3;
_10 = 19610180_u32 - 809049837_u32;
_8 = -_2;
_6 = (-47_isize) as f32;
_2 = _3 * _4;
_6 = (-291859165_i32) as f32;
_10 = 57_u8 as u32;
_7 = 6003630717086538304_usize as u64;
_4 = 178756556187057397378872986988110122757_u128 as f32;
_12.fld3 = _3 as f64;
_12.fld5 = (-119669030_i32);
Call(_13.fld2 = fn12(_2, _5, _8, _12.fld3, _2, _8, _12.fld3, _8, Move(_1), _3, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.fld0 = [_12.fld5,_12.fld5,_12.fld5,_12.fld5,_12.fld5,_12.fld5,_12.fld5];
_16 = (-54279852228935676599460910518900050071_i128) & (-167181389090027720318065904057794055387_i128);
_14 = [78808323808086880419405980879024764209_u128,311569560046381385003647854394593881672_u128,138634744457510403435231900427351912123_u128,213391906168627061837636353413523606448_u128,72055757970303872945797070413567709192_u128,254875746856977847328074758418508495558_u128,260403656635043697018868824288310885581_u128,196584192245142493795728686503467376559_u128];
_13.fld3 = _12.fld0;
_4 = -_3;
_12.fld1 = core::ptr::addr_of_mut!(_13.fld5.fld3);
match _12.fld5 {
0 => bb3,
1 => bb4,
340282366920938463463374607431648542426 => bb6,
_ => bb5
}
}
bb3 = {
_8 = _3 - _3;
_10 = 19610180_u32 - 809049837_u32;
_8 = -_2;
_6 = (-47_isize) as f32;
_2 = _3 * _4;
_6 = (-291859165_i32) as f32;
_10 = 57_u8 as u32;
_7 = 6003630717086538304_usize as u64;
_4 = 178756556187057397378872986988110122757_u128 as f32;
_12.fld3 = _3 as f64;
_12.fld5 = (-119669030_i32);
Call(_13.fld2 = fn12(_2, _5, _8, _12.fld3, _2, _8, _12.fld3, _8, Move(_1), _3, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_15 = _8;
match _12.fld5 {
0 => bb4,
340282366920938463463374607431648542426 => bb7,
_ => bb2
}
}
bb7 = {
_17 = &_13.fld5.fld1;
_1 = core::ptr::addr_of_mut!(_13.fld5.fld3);
_12.fld5 = 808970683_i32;
_13.fld0 = true;
_18 = !_7;
_3 = 15812_u16 as f32;
_11 = &_14;
_3 = 88_i8 as f32;
_13.fld5.fld3.0 = _8;
_10 = 2415817876_u32;
_2 = _12.fld5 as f32;
_3 = _13.fld2 as f32;
_12.fld6 = core::ptr::addr_of!((*_17));
match _10 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
2415817876 => bb11,
_ => bb10
}
}
bb8 = {
_8 = _3 - _3;
_10 = 19610180_u32 - 809049837_u32;
_8 = -_2;
_6 = (-47_isize) as f32;
_2 = _3 * _4;
_6 = (-291859165_i32) as f32;
_10 = 57_u8 as u32;
_7 = 6003630717086538304_usize as u64;
_4 = 178756556187057397378872986988110122757_u128 as f32;
_12.fld3 = _3 as f64;
_12.fld5 = (-119669030_i32);
Call(_13.fld2 = fn12(_2, _5, _8, _12.fld3, _2, _8, _12.fld3, _8, Move(_1), _3, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_12.fld0 = [_12.fld5,_12.fld5,_12.fld5,_12.fld5,_12.fld5,_12.fld5,_12.fld5];
_16 = (-54279852228935676599460910518900050071_i128) & (-167181389090027720318065904057794055387_i128);
_14 = [78808323808086880419405980879024764209_u128,311569560046381385003647854394593881672_u128,138634744457510403435231900427351912123_u128,213391906168627061837636353413523606448_u128,72055757970303872945797070413567709192_u128,254875746856977847328074758418508495558_u128,260403656635043697018868824288310885581_u128,196584192245142493795728686503467376559_u128];
_13.fld3 = _12.fld0;
_4 = -_3;
_12.fld1 = core::ptr::addr_of_mut!(_13.fld5.fld3);
match _12.fld5 {
0 => bb3,
1 => bb4,
340282366920938463463374607431648542426 => bb6,
_ => bb5
}
}
bb10 = {
Return()
}
bb11 = {
_12.fld2 = !81_u8;
_13.fld7 = core::ptr::addr_of_mut!(_12.fld0);
_1 = Move(_12.fld1);
_20 = [_7,_7,_7,_7,_18,_18,_18];
match _12.fld5 {
0 => bb12,
808970683 => bb14,
_ => bb13
}
}
bb12 = {
_8 = _3 - _3;
_10 = 19610180_u32 - 809049837_u32;
_8 = -_2;
_6 = (-47_isize) as f32;
_2 = _3 * _4;
_6 = (-291859165_i32) as f32;
_10 = 57_u8 as u32;
_7 = 6003630717086538304_usize as u64;
_4 = 178756556187057397378872986988110122757_u128 as f32;
_12.fld3 = _3 as f64;
_12.fld5 = (-119669030_i32);
Call(_13.fld2 = fn12(_2, _5, _8, _12.fld3, _2, _8, _12.fld3, _8, Move(_1), _3, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_17 = &_13.fld5.fld1;
_1 = core::ptr::addr_of_mut!(_13.fld5.fld3);
_12.fld5 = 808970683_i32;
_13.fld0 = true;
_18 = !_7;
_3 = 15812_u16 as f32;
_11 = &_14;
_3 = 88_i8 as f32;
_13.fld5.fld3.0 = _8;
_10 = 2415817876_u32;
_2 = _12.fld5 as f32;
_3 = _13.fld2 as f32;
_12.fld6 = core::ptr::addr_of!((*_17));
match _10 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
2415817876 => bb11,
_ => bb10
}
}
bb14 = {
_11 = &(*_11);
RET = core::ptr::addr_of!((*_17));
_12.fld1 = core::ptr::addr_of_mut!(_13.fld5.fld3);
_12.fld1 = core::ptr::addr_of_mut!(_13.fld5.fld3);
_20 = [_18,_7,_7,_7,_7,_7,_7];
_13.fld4 = core::ptr::addr_of_mut!(_16);
_4 = _7 as f32;
RET = core::ptr::addr_of!((*_17));
_10 = 1608733540_u32 - 1569674083_u32;
_23.1.fld3 = -_12.fld3;
_23.1.fld2 = _12.fld2 * _12.fld2;
_14 = [166572344017535271505915178919480671613_u128,248090193603834255867534457751012000756_u128,260629318459686922380381906392030863184_u128,2746893668765101633535782192798096937_u128,66805191001601084927463951660498471523_u128,121978673916763900050140220618954570950_u128,121213082165545600302636064656177108158_u128,226737520577522447882965189067228841387_u128];
_13.fld4 = core::ptr::addr_of_mut!(_16);
_23.1.fld4 = -_8;
_22 = &_12.fld0;
_23.1.fld5 = _12.fld5 * _12.fld5;
_12.fld6 = core::ptr::addr_of!((*_17));
_23.1.fld4 = _5 - _15;
_23.1.fld2 = 25251_i16 as u8;
_23.0.0 = _13.fld2 as u128;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(11_usize, 20_usize, Move(_20), 9_usize, Move(_9), 16_usize, Move(_16), 25_usize, _25), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: f64,mut _5: f32,mut _6: f32,mut _7: f64,mut _8: f32,mut _9: *mut (f32,),mut _10: f32,mut _11: f32) -> isize {
mir! {
type RET = isize;
let _12: isize;
let _13: f64;
let _14: [u128; 3];
let _15: f32;
let _16: bool;
let _17: f32;
let _18: isize;
let _19: i32;
let _20: [u16; 6];
let _21: Adt32;
let _22: [bool; 3];
let _23: &'static [u128; 8];
let _24: i32;
let _25: (char, [u64; 4], u16);
let _26: isize;
let _27: i16;
let _28: f64;
let _29: ();
let _30: ();
{
_4 = _7;
_11 = 2_usize as f32;
RET = 9223372036854775807_isize;
_7 = -_4;
_6 = (-32697_i16) as f32;
_12 = RET;
_7 = _4 + _4;
_8 = _5;
_12 = !RET;
_12 = !RET;
_8 = _1 + _1;
_7 = _4 + _4;
_13 = 140477409558795266590833880214975552351_u128 as f64;
_12 = RET & RET;
_5 = _8;
_1 = _8;
_4 = -_7;
Call(_1 = fn13(_5, _4, _8, _8, Move(_9), _4, _13, _7, _2, _8, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = _4;
_11 = _3;
match RET {
9223372036854775807 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_8 = RET as f32;
Goto(bb4)
}
bb4 = {
_15 = _2;
_4 = _7 - _7;
_8 = -_3;
Call(_4 = fn14(_13, _13, _5, _11, _1, _1, _3, _13, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = 94545779670827807702016139280159667572_u128 as isize;
_16 = _13 > _13;
_4 = _7;
_8 = _10;
_6 = _15;
_2 = _5;
_8 = _2 - _1;
_3 = _6 - _8;
Call(_12 = core::intrinsics::transmute(RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = _12 * _12;
_13 = _7;
_6 = 17408_u16 as f32;
RET = _12;
_1 = -_5;
_18 = _12;
RET = _18 ^ _18;
RET = !_12;
_18 = 45620_u16 as isize;
_11 = _8 * _2;
_5 = 23205080963026755336317869654452076570_i128 as f32;
RET = 54494477300681112348849028495569892260_u128 as isize;
_18 = _12 ^ _12;
RET = (-16465_i16) as isize;
_15 = _1;
_15 = _11;
Goto(bb7)
}
bb7 = {
_6 = _13 as f32;
_14 = [99869594009896794434962291996567269812_u128,64219865414708395643065996214581570884_u128,186831675303465117407698988406041604166_u128];
_20 = [58916_u16,44584_u16,44480_u16,22069_u16,26419_u16,34679_u16];
_16 = true;
_3 = -_2;
_4 = _11 as f64;
_4 = _13 - _7;
_19 = (-222484230_i32);
_5 = _2 + _15;
_2 = -_11;
_12 = -_18;
_13 = -_4;
_13 = -_4;
_14 = [119403234694157530262197116483338786486_u128,191809208908621060734509012026460638842_u128,150061271652830319230369550615188472571_u128];
_4 = _7;
_7 = _13;
_11 = _1;
_17 = _4 as f32;
_19 = 832871633_i32 + 871401923_i32;
RET = _18 + _12;
_3 = _13 as f32;
_20 = [19709_u16,62611_u16,2601_u16,11202_u16,40676_u16,2791_u16];
Goto(bb8)
}
bb8 = {
_12 = -_18;
_10 = _11 * _2;
_4 = _13 * _7;
RET = _18;
_19 = -56795127_i32;
_5 = 4_usize as f32;
RET = -_18;
_13 = _4 + _4;
_12 = RET >> RET;
_12 = RET + RET;
_22 = [_16,_16,_16];
_2 = _6 + _8;
RET = _12 - _12;
_24 = !_19;
_25.2 = 12707_u16 * 369_u16;
_25.0 = '\u{597f}';
_25.1 = [5852180514437081339_u64,7681481823142719565_u64,8220140260446821481_u64,11535325159899849364_u64];
RET = !_12;
_10 = _15 * _3;
_13 = _7 + _4;
_25.1 = [14729311606941209874_u64,13771335368371820103_u64,14836509785025094429_u64,12231801966183108075_u64];
_11 = 102_u8 as f32;
_5 = _1 - _2;
Goto(bb9)
}
bb9 = {
_10 = _8;
_25.2 = 5516_u16 & 44637_u16;
_17 = _3 * _10;
_8 = _2;
_20 = [_25.2,_25.2,_25.2,_25.2,_25.2,_25.2];
_26 = _13 as isize;
_3 = (-8759085714387540959_i64) as f32;
_6 = _15;
_7 = _25.2 as f64;
RET = _26;
_12 = _18;
_6 = -_10;
_14 = [233002526727590796381022335613199651597_u128,202563391642049698613037712557546019234_u128,279431526404434495677606927709113122877_u128];
_24 = _19 - _19;
Goto(bb10)
}
bb10 = {
Call(_29 = dump_var(12_usize, 26_usize, Move(_26), 22_usize, Move(_22), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_29 = dump_var(12_usize, 18_usize, Move(_18), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: f32,mut _2: f64,mut _3: f32,mut _4: f32,mut _5: *mut (f32,),mut _6: f64,mut _7: f64,mut _8: f64,mut _9: f32,mut _10: f32,mut _11: f64) -> f32 {
mir! {
type RET = f32;
let _12: [u16; 4];
let _13: u16;
let _14: &'static [i32; 7];
let _15: [u64; 7];
let _16: ();
let _17: ();
{
_11 = _2 - _2;
_1 = _9;
RET = _4 - _1;
_10 = _1 + _4;
_1 = RET * _9;
_7 = _11;
_4 = -RET;
_3 = _10;
_12 = [2964_u16,24616_u16,39533_u16,22915_u16];
_8 = _7 - _2;
_12 = [35844_u16,631_u16,47834_u16,7107_u16];
_13 = '\u{608e1}' as u16;
_1 = _3 * _10;
_6 = -_2;
_3 = 830535857962514005_u64 as f32;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(13_usize, 13_usize, Move(_13), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: f64,mut _2: f64,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f64,mut _9: f64) -> f64 {
mir! {
type RET = f64;
let _10: ();
let _11: ();
{
RET = -_1;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [u8; 6],mut _2: [i32; 7]) -> *const isize {
mir! {
type RET = *const isize;
let _3: f32;
let _4: [u16; 4];
let _5: [i32; 7];
let _6: [bool; 2];
let _7: *const (f64, [char; 7]);
let _8: *mut *const isize;
let _9: bool;
let _10: *mut [char; 7];
let _11: i32;
let _12: [u8; 6];
let _13: *const i16;
let _14: f32;
let _15: isize;
let _16: [u64; 7];
let _17: [u8; 7];
let _18: f64;
let _19: i128;
let _20: &'static [u128; 8];
let _21: (*mut i128, [bool; 2]);
let _22: *mut isize;
let _23: &'static [bool; 2];
let _24: bool;
let _25: *const isize;
let _26: &'static bool;
let _27: Adt32;
let _28: usize;
let _29: isize;
let _30: [i32; 7];
let _31: [i16; 2];
let _32: Adt68;
let _33: [u8; 6];
let _34: (*mut u64, i16, usize);
let _35: Adt67;
let _36: *mut *const isize;
let _37: [u128; 8];
let _38: (f32,);
let _39: *mut u64;
let _40: char;
let _41: *mut usize;
let _42: *const i128;
let _43: f64;
let _44: ([u128; 8], [u64; 7], *mut (f32,));
let _45: ();
let _46: ();
{
_2 = [1518083710_i32,1204457842_i32,(-800364941_i32),697777472_i32,2094440168_i32,(-15183572_i32),1416212941_i32];
_2 = [(-1045820422_i32),(-1547824960_i32),(-471364835_i32),821366010_i32,1577562695_i32,1453058292_i32,56320424_i32];
_2 = [(-240866357_i32),(-2142193272_i32),964568209_i32,(-428381947_i32),817928824_i32,803350108_i32,(-1028193958_i32)];
_2 = [(-378645598_i32),(-896249448_i32),(-553121122_i32),2031136984_i32,(-1220771175_i32),1807068004_i32,(-1431722546_i32)];
Goto(bb1)
}
bb1 = {
_1 = [132_u8,191_u8,85_u8,240_u8,90_u8,103_u8];
_1 = [70_u8,136_u8,135_u8,132_u8,108_u8,24_u8];
_1 = [243_u8,72_u8,173_u8,22_u8,43_u8,11_u8];
_2 = [(-1671632869_i32),287715213_i32,(-662622067_i32),(-567863698_i32),(-1053351964_i32),(-131764161_i32),1415279271_i32];
_1 = [179_u8,59_u8,119_u8,179_u8,92_u8,140_u8];
_2 = [1751787703_i32,(-2033245576_i32),967988573_i32,(-197947463_i32),334788598_i32,(-2108738796_i32),(-869100663_i32)];
_2 = [(-1065627437_i32),2053625408_i32,608966675_i32,2118855019_i32,(-1396906289_i32),(-1508359484_i32),1909241449_i32];
_1 = [193_u8,136_u8,31_u8,233_u8,179_u8,97_u8];
_1 = [87_u8,132_u8,64_u8,16_u8,88_u8,239_u8];
_1 = [40_u8,181_u8,91_u8,207_u8,175_u8,210_u8];
_1 = [9_u8,255_u8,77_u8,18_u8,190_u8,142_u8];
Goto(bb2)
}
bb2 = {
_3 = 106_isize as f32;
_1 = [73_u8,255_u8,106_u8,121_u8,83_u8,229_u8];
_2 = [(-750581502_i32),(-1806091208_i32),1261705922_i32,(-488215170_i32),2090021694_i32,(-1740826543_i32),1491637995_i32];
_2 = [1387241810_i32,932914930_i32,1490726009_i32,51968426_i32,794854896_i32,(-205101189_i32),(-2147371673_i32)];
_3 = 9223372036854775807_isize as f32;
_4 = [19197_u16,50206_u16,3809_u16,59347_u16];
_5 = [(-2046241402_i32),1133209498_i32,861292740_i32,508570570_i32,1914461453_i32,1038716599_i32,(-707503790_i32)];
_3 = 55104_u16 as f32;
_2 = [(-1938451572_i32),500675266_i32,(-85488229_i32),949361686_i32,(-123758797_i32),(-1925632646_i32),(-825200213_i32)];
_1 = [59_u8,201_u8,154_u8,224_u8,100_u8,128_u8];
_1 = [145_u8,236_u8,224_u8,67_u8,19_u8,161_u8];
_5 = _2;
_1 = [14_u8,192_u8,191_u8,161_u8,42_u8,232_u8];
_3 = 1158339794_i32 as f32;
_1 = [173_u8,6_u8,58_u8,90_u8,213_u8,202_u8];
_4 = [38744_u16,25530_u16,2813_u16,24180_u16];
_3 = 1454403529755970494_i64 as f32;
_6 = [false,false];
_3 = 5_usize as f32;
_6 = [false,true];
_3 = 8799685354279467331_i64 as f32;
Goto(bb3)
}
bb3 = {
_6 = [false,false];
_3 = (-56383339969223668491232917856641467736_i128) as f32;
_4 = [13757_u16,50124_u16,17088_u16,31648_u16];
_4 = [11374_u16,55255_u16,16741_u16,20586_u16];
_2 = _5;
_9 = true;
_2 = [(-945204916_i32),(-12505068_i32),(-1015384179_i32),1723812379_i32,2019574053_i32,(-1019852271_i32),(-382939156_i32)];
_4 = [4739_u16,58894_u16,23905_u16,58141_u16];
_3 = 3_usize as f32;
_6 = [_9,_9];
Goto(bb4)
}
bb4 = {
_8 = core::ptr::addr_of_mut!(RET);
_6 = [_9,_9];
_1 = [67_u8,176_u8,69_u8,113_u8,175_u8,149_u8];
_11 = !(-1910873577_i32);
Goto(bb5)
}
bb5 = {
_12 = [47_u8,121_u8,13_u8,125_u8,236_u8,84_u8];
_12 = [249_u8,12_u8,59_u8,12_u8,21_u8,54_u8];
_1 = _12;
_14 = -_3;
_14 = -_3;
_6 = [_9,_9];
_11 = (-1596754709_i32) * (-38886573_i32);
_2 = _5;
RET = core::ptr::addr_of!(_15);
_14 = 98_u8 as f32;
Goto(bb6)
}
bb6 = {
_15 = 9223372036854775807_isize;
(*RET) = -42_isize;
(*RET) = -9_isize;
_3 = _14;
(*_8) = core::ptr::addr_of!((*RET));
_6 = [_9,_9];
(*_8) = core::ptr::addr_of!(_15);
_11 = _9 as i32;
(*RET) = (-9223372036854775808_isize);
RET = core::ptr::addr_of!(_15);
RET = core::ptr::addr_of!((*RET));
_3 = -_14;
(*RET) = -9223372036854775807_isize;
_12 = [55_u8,232_u8,109_u8,34_u8,146_u8,55_u8];
RET = core::ptr::addr_of!((*RET));
_15 = _9 as isize;
(*RET) = (-9223372036854775808_isize);
_14 = _11 as f32;
_12 = [58_u8,86_u8,100_u8,86_u8,255_u8,193_u8];
(*RET) = -9223372036854775807_isize;
_11 = 522008798_i32 & 950488261_i32;
_11 = !887876429_i32;
(*_8) = core::ptr::addr_of!((*RET));
_4 = [64042_u16,60071_u16,65322_u16,27111_u16];
Call(_5 = fn16(_4, _9, (*RET), (*RET), _4, _9, (*RET), _2, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = core::ptr::addr_of!(_15);
Goto(bb8)
}
bb8 = {
RET = core::ptr::addr_of!(_15);
_18 = 111_i8 as f64;
(*RET) = _14 as isize;
_5 = _2;
(*RET) = (-118_isize) ^ (-9223372036854775808_isize);
_2 = _5;
_17 = [180_u8,166_u8,107_u8,68_u8,155_u8,67_u8,190_u8];
_19 = (-139735759637000260639707455603914652435_i128);
_1 = [1_u8,119_u8,136_u8,133_u8,241_u8,161_u8];
_21.0 = core::ptr::addr_of_mut!(_19);
RET = core::ptr::addr_of!((*RET));
_4 = [64714_u16,59757_u16,38051_u16,14229_u16];
_1 = [189_u8,227_u8,246_u8,20_u8,219_u8,37_u8];
_21.0 = core::ptr::addr_of_mut!(_19);
(*RET) = (-9223372036854775808_isize);
_18 = 13792576723046276026_usize as f64;
_1 = [148_u8,60_u8,23_u8,191_u8,47_u8,34_u8];
(*_8) = core::ptr::addr_of!((*RET));
_21.1 = [_9,_9];
_23 = &_21.1;
_24 = _9 | _9;
_23 = &(*_23);
RET = core::ptr::addr_of!((*RET));
_2 = [_11,_11,_11,_11,_11,_11,_11];
_23 = &(*_23);
(*RET) = (-9223372036854775808_isize) << _11;
Goto(bb9)
}
bb9 = {
RET = core::ptr::addr_of!((*RET));
_14 = -_3;
_21.1 = [_24,_24];
_21.1 = _6;
_16 = [15822304509363730361_u64,15915940509263723870_u64,11953518578790307969_u64,7253623982904806944_u64,18251226011150668681_u64,12967160970190077247_u64,9717500858605107417_u64];
_2 = _5;
_22 = core::ptr::addr_of_mut!((*RET));
_23 = &_21.1;
RET = core::ptr::addr_of!((*_22));
_26 = &_24;
_1 = [177_u8,127_u8,47_u8,69_u8,232_u8,41_u8];
_4 = [31400_u16,30486_u16,37714_u16,13671_u16];
(*_22) = 116_isize;
_15 = (-9223372036854775808_isize);
(*RET) = -9223372036854775807_isize;
(*RET) = 9223372036854775807_isize;
(*RET) = -53_isize;
(*RET) = 9223372036854775807_isize | 9223372036854775807_isize;
_17 = [200_u8,110_u8,63_u8,77_u8,146_u8,28_u8,134_u8];
_18 = 7755984254396303572_usize as f64;
_28 = 265242640869193341137932227643601289520_u128 as usize;
_2 = [_11,_11,_11,_11,_11,_11,_11];
_9 = _24;
(*RET) = (-16_isize) | (-100_isize);
_21.1 = [_24,_24];
_4 = [42190_u16,46126_u16,51749_u16,61078_u16];
Goto(bb10)
}
bb10 = {
_25 = core::ptr::addr_of!((*_22));
_5 = _2;
_26 = &_24;
_18 = 10601_u16 as f64;
_2 = [_11,_11,_11,_11,_11,_11,_11];
_18 = (-29727_i16) as f64;
_25 = core::ptr::addr_of!((*_25));
_24 = _9;
_32.fld5.fld3.0 = _3;
_21.1 = [_9,_24];
(*RET) = _18 as isize;
_33 = _12;
_27 = Adt32::Variant1 { fld0: _32.fld5.fld3,fld1: (*RET) };
(*_8) = Move(_25);
_32.fld6 = ['\u{1763b}','\u{980c3}','\u{8bb09}'];
Goto(bb11)
}
bb11 = {
_26 = &_24;
_32.fld5.fld3.0 = 3388583824_u32 as f32;
_32.fld5.fld1.0 = _18 + _18;
match _19 {
200546607283938202823667151827853559021 => bb13,
_ => bb12
}
}
bb12 = {
_1 = [132_u8,191_u8,85_u8,240_u8,90_u8,103_u8];
_1 = [70_u8,136_u8,135_u8,132_u8,108_u8,24_u8];
_1 = [243_u8,72_u8,173_u8,22_u8,43_u8,11_u8];
_2 = [(-1671632869_i32),287715213_i32,(-662622067_i32),(-567863698_i32),(-1053351964_i32),(-131764161_i32),1415279271_i32];
_1 = [179_u8,59_u8,119_u8,179_u8,92_u8,140_u8];
_2 = [1751787703_i32,(-2033245576_i32),967988573_i32,(-197947463_i32),334788598_i32,(-2108738796_i32),(-869100663_i32)];
_2 = [(-1065627437_i32),2053625408_i32,608966675_i32,2118855019_i32,(-1396906289_i32),(-1508359484_i32),1909241449_i32];
_1 = [193_u8,136_u8,31_u8,233_u8,179_u8,97_u8];
_1 = [87_u8,132_u8,64_u8,16_u8,88_u8,239_u8];
_1 = [40_u8,181_u8,91_u8,207_u8,175_u8,210_u8];
_1 = [9_u8,255_u8,77_u8,18_u8,190_u8,142_u8];
Goto(bb2)
}
bb13 = {
_23 = &_6;
_33 = _1;
place!(Field::<(f32,)>(Variant(_27, 1), 0)) = (_3,);
_8 = core::ptr::addr_of_mut!((*_8));
_32.fld4 = core::ptr::addr_of_mut!(_19);
_19 = _28 as i128;
place!(Field::<isize>(Variant(_27, 1), 1)) = (*_22);
_1 = _33;
_8 = core::ptr::addr_of_mut!(RET);
_32.fld1 = core::ptr::addr_of!(_19);
_2 = _5;
_32.fld3 = [_11,_11,_11,_11,_11,_11,_11];
_14 = -_32.fld5.fld3.0;
_21 = (Move(_32.fld4), _6);
_32.fld2 = (*_22);
_29 = 145_u8 as isize;
_34.1 = -(-16180_i16);
_15 = _19 as isize;
_21.0 = core::ptr::addr_of_mut!(_19);
_33 = [29_u8,127_u8,189_u8,236_u8,5_u8,236_u8];
_18 = _32.fld5.fld1.0;
_32.fld5.fld2 = Move((*_8));
(*_22) = !Field::<isize>(Variant(_27, 1), 1);
_30 = _5;
SetDiscriminant(_27, 1);
Goto(bb14)
}
bb14 = {
_32.fld0 = !_24;
_36 = Move(_8);
_35.fld6 = core::ptr::addr_of!(_32.fld5.fld1);
_5 = _32.fld3;
_32.fld5.fld3.0 = (-91_i8) as f32;
_14 = _32.fld5.fld3.0;
_38 = _32.fld5.fld3;
_33 = [158_u8,5_u8,190_u8,189_u8,17_u8,43_u8];
_31 = [_34.1,_34.1];
_8 = Move(_36);
_12 = _1;
_14 = _32.fld5.fld3.0;
Call(place!(Field::<isize>(Variant(_27, 1), 1)) = fn17(Move(_23), Move(_26), Move(_22), _15), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_32.fld6 = ['\u{915d5}','\u{4c5b5}','\u{10e239}'];
_7 = Move(_35.fld6);
_35.fld2 = !40_u8;
_11 = 1801544656_i32 | (-1337139337_i32);
_38 = (_3,);
_10 = core::ptr::addr_of_mut!(_32.fld5.fld1.1);
_26 = &_32.fld0;
place!(Field::<(f32,)>(Variant(_27, 1), 0)).0 = _38.0;
_20 = &_37;
RET = core::ptr::addr_of!(_32.fld2);
_9 = (*_26) == (*_26);
(*RET) = _15;
_35.fld3 = -_32.fld5.fld1.0;
_21.1 = [_24,_9];
(*RET) = Field::<isize>(Variant(_27, 1), 1) << _35.fld2;
_34.1 = !(-10779_i16);
_35.fld6 = Move(_7);
_32.fld7 = core::ptr::addr_of_mut!(_2);
Goto(bb16)
}
bb16 = {
Call(_45 = dump_var(15_usize, 28_usize, Move(_28), 17_usize, Move(_17), 12_usize, Move(_12), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(15_usize, 1_usize, Move(_1), 30_usize, Move(_30), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(15_usize, 29_usize, Move(_29), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [u16; 4],mut _2: bool,mut _3: isize,mut _4: isize,mut _5: [u16; 4],mut _6: bool,mut _7: isize,mut _8: [i32; 7],mut _9: f32) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _10: *const i16;
let _11: char;
let _12: &'static f64;
let _13: f32;
let _14: [u8; 3];
let _15: ((u128, Adt32, *mut [i32; 7], (f64, [char; 7])), Adt67, bool, u8);
let _16: *mut (f32,);
let _17: [i8; 5];
let _18: ((f32,), (f32,), [bool; 2]);
let _19: i32;
let _20: usize;
let _21: [bool; 5];
let _22: char;
let _23: [u64; 4];
let _24: &'static [u128; 8];
let _25: (*mut i128, *const (f64, [char; 7]), u32, (u128, Adt32, *mut [i32; 7], (f64, [char; 7])));
let _26: i128;
let _27: u32;
let _28: i8;
let _29: ();
let _30: ();
{
RET = _8;
RET = _8;
_9 = _7 as f32;
RET = [(-2113247339_i32),1475471461_i32,1751707893_i32,(-932561929_i32),1598660406_i32,536132545_i32,(-864749897_i32)];
_3 = _7;
_8 = [(-1327444589_i32),382433503_i32,(-27324386_i32),(-1717838620_i32),(-459998629_i32),(-2082639894_i32),341301775_i32];
_2 = _6 & _6;
_7 = (-29388_i16) as isize;
_6 = _2 | _2;
_1 = _5;
_2 = _6;
RET = [(-296826811_i32),(-176717304_i32),1146586169_i32,(-463119157_i32),(-1605442987_i32),(-1894189410_i32),(-1963099795_i32)];
_11 = '\u{d9da6}';
_2 = !_6;
_2 = _3 < _3;
_1 = [60496_u16,32585_u16,41106_u16,28789_u16];
_1 = [27801_u16,23416_u16,53268_u16,63058_u16];
_5 = [26465_u16,6120_u16,54531_u16,3637_u16];
_9 = 7561041989350379785_u64 as f32;
_7 = 40526_u16 as isize;
_8 = RET;
Call(_3 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _5;
RET = _8;
_7 = !_3;
_3 = 101890523_i32 as isize;
_13 = (-25_i8) as f32;
_7 = _2 as isize;
_6 = !_2;
_5 = [33392_u16,33539_u16,55582_u16,40946_u16];
_8 = RET;
RET = [(-1269906540_i32),(-2112706371_i32),(-1958454834_i32),(-67543786_i32),(-1118762946_i32),631484960_i32,(-1535093641_i32)];
_3 = _7;
_11 = '\u{10315a}';
_8 = RET;
_14 = [239_u8,154_u8,197_u8];
_2 = !_6;
_2 = _6;
_13 = -_9;
_8 = RET;
Call(_15.1.fld4 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15.1.fld7 = core::ptr::addr_of!(_4);
_5 = [34842_u16,42913_u16,63511_u16,33843_u16];
_12 = &_15.0.3.0;
_15.1.fld3 = 220959306071648132964012676367996099852_u128 as f64;
_11 = '\u{e3585}';
_15.1.fld5 = 1484495555_i32;
_15.0.2 = core::ptr::addr_of_mut!(_8);
match _15.1.fld5 {
0 => bb1,
1484495555 => bb4,
_ => bb3
}
}
bb3 = {
_1 = _5;
RET = _8;
_7 = !_3;
_3 = 101890523_i32 as isize;
_13 = (-25_i8) as f32;
_7 = _2 as isize;
_6 = !_2;
_5 = [33392_u16,33539_u16,55582_u16,40946_u16];
_8 = RET;
RET = [(-1269906540_i32),(-2112706371_i32),(-1958454834_i32),(-67543786_i32),(-1118762946_i32),631484960_i32,(-1535093641_i32)];
_3 = _7;
_11 = '\u{10315a}';
_8 = RET;
_14 = [239_u8,154_u8,197_u8];
_2 = !_6;
_2 = _6;
_13 = -_9;
_8 = RET;
Call(_15.1.fld4 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_15.0.3.0 = 11660850758724451972_u64 as f64;
Goto(bb5)
}
bb5 = {
_4 = _7 | _3;
_12 = &_15.1.fld3;
_6 = _2 != _2;
_18.2 = [_6,_2];
_15.1.fld0 = _8;
_15.3 = 74_u8 - 25_u8;
_15.0.3.0 = 1963087252_u32 as f64;
_18.1 = (_15.1.fld4,);
_17 = [(-52_i8),(-10_i8),28_i8,54_i8,(-35_i8)];
_18.0.0 = -_18.1.0;
_11 = '\u{3e7b4}';
_15.1.fld7 = core::ptr::addr_of!(_4);
_16 = core::ptr::addr_of_mut!(_18.0);
match _15.1.fld5 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
1484495555 => bb11,
_ => bb10
}
}
bb6 = {
_15.0.3.0 = 11660850758724451972_u64 as f64;
Goto(bb5)
}
bb7 = {
_1 = _5;
RET = _8;
_7 = !_3;
_3 = 101890523_i32 as isize;
_13 = (-25_i8) as f32;
_7 = _2 as isize;
_6 = !_2;
_5 = [33392_u16,33539_u16,55582_u16,40946_u16];
_8 = RET;
RET = [(-1269906540_i32),(-2112706371_i32),(-1958454834_i32),(-67543786_i32),(-1118762946_i32),631484960_i32,(-1535093641_i32)];
_3 = _7;
_11 = '\u{10315a}';
_8 = RET;
_14 = [239_u8,154_u8,197_u8];
_2 = !_6;
_2 = _6;
_13 = -_9;
_8 = RET;
Call(_15.1.fld4 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_15.1.fld7 = core::ptr::addr_of!(_4);
_5 = [34842_u16,42913_u16,63511_u16,33843_u16];
_12 = &_15.0.3.0;
_15.1.fld3 = 220959306071648132964012676367996099852_u128 as f64;
_11 = '\u{e3585}';
_15.1.fld5 = 1484495555_i32;
_15.0.2 = core::ptr::addr_of_mut!(_8);
match _15.1.fld5 {
0 => bb1,
1484495555 => bb4,
_ => bb3
}
}
bb9 = {
_1 = _5;
RET = _8;
_7 = !_3;
_3 = 101890523_i32 as isize;
_13 = (-25_i8) as f32;
_7 = _2 as isize;
_6 = !_2;
_5 = [33392_u16,33539_u16,55582_u16,40946_u16];
_8 = RET;
RET = [(-1269906540_i32),(-2112706371_i32),(-1958454834_i32),(-67543786_i32),(-1118762946_i32),631484960_i32,(-1535093641_i32)];
_3 = _7;
_11 = '\u{10315a}';
_8 = RET;
_14 = [239_u8,154_u8,197_u8];
_2 = !_6;
_2 = _6;
_13 = -_9;
_8 = RET;
Call(_15.1.fld4 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_7 = !_4;
_19 = _15.1.fld5;
_15.0.3.1 = [_11,_11,_11,_11,_11,_11,_11];
_16 = core::ptr::addr_of_mut!(_18.1);
_15.1.fld6 = core::ptr::addr_of!(_15.0.3);
_15.1.fld1 = core::ptr::addr_of_mut!((*_16));
_15.0.3.1 = [_11,_11,_11,_11,_11,_11,_11];
_5 = [14762_u16,48807_u16,6288_u16,5861_u16];
_4 = -_3;
_21 = [_6,_6,_2,_6,_6];
_13 = (*_16).0 * _18.0.0;
_8 = RET;
(*_16) = (_9,);
_15.1.fld2 = _4 as u8;
_15.1.fld6 = core::ptr::addr_of!(_15.0.3);
RET = _8;
_17 = [52_i8,39_i8,45_i8,(-79_i8),66_i8];
_14 = [_15.3,_15.3,_15.1.fld2];
_15.1.fld4 = (*_16).0 + _18.1.0;
_22 = _11;
_15.0.2 = core::ptr::addr_of_mut!(RET);
_21 = [_2,_6,_2,_6,_2];
_19 = _15.1.fld5 - _15.1.fld5;
RET = [_19,_15.1.fld5,_19,_15.1.fld5,_19,_19,_19];
match _15.1.fld5 {
0 => bb12,
1484495555 => bb14,
_ => bb13
}
}
bb12 = {
_15.0.3.0 = 11660850758724451972_u64 as f64;
Goto(bb5)
}
bb13 = {
_1 = _5;
RET = _8;
_7 = !_3;
_3 = 101890523_i32 as isize;
_13 = (-25_i8) as f32;
_7 = _2 as isize;
_6 = !_2;
_5 = [33392_u16,33539_u16,55582_u16,40946_u16];
_8 = RET;
RET = [(-1269906540_i32),(-2112706371_i32),(-1958454834_i32),(-67543786_i32),(-1118762946_i32),631484960_i32,(-1535093641_i32)];
_3 = _7;
_11 = '\u{10315a}';
_8 = RET;
_14 = [239_u8,154_u8,197_u8];
_2 = !_6;
_2 = _6;
_13 = -_9;
_8 = RET;
Call(_15.1.fld4 = core::intrinsics::transmute(_11), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_22 = _11;
_20 = 2902645327341330285_usize;
_25.2 = !236031859_u32;
_8 = _15.1.fld0;
_18.0.0 = _15.3 as f32;
_25.3.3.1 = _15.0.3.1;
_25.3.1 = Adt32::Variant1 { fld0: (*_16),fld1: _7 };
_15.0.3.1 = _25.3.3.1;
_25.1 = core::ptr::addr_of!(_15.0.3);
(*_16).0 = _25.2 as f32;
_15.1.fld2 = _15.3 * _15.3;
_4 = -Field::<isize>(Variant(_25.3.1, 1), 1);
_25.3.3.0 = -_15.1.fld3;
_15.0.0 = 239726550039793745550937322712078549792_u128 | 224191615654110392430826087801349381781_u128;
_11 = _22;
_14 = [_15.1.fld2,_15.1.fld2,_15.3];
RET = [_19,_19,_19,_15.1.fld5,_19,_19,_19];
_15.0.3.1 = _25.3.3.1;
_18.2 = [_6,_6];
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(16_usize, 5_usize, Move(_5), 21_usize, Move(_21), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(16_usize, 2_usize, Move(_2), 3_usize, Move(_3), 1_usize, Move(_1), 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: &'static [bool; 2],mut _2: &'static bool,mut _3: *mut isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: f64;
let _6: &'static Adt56;
let _7: i128;
let _8: f64;
let _9: [i8; 5];
let _10: isize;
let _11: f32;
let _12: ();
let _13: ();
{
_5 = 373859381_u32 as f64;
RET = _4;
_4 = RET * RET;
RET = _4 & _4;
RET = false as isize;
RET = !_4;
RET = _4;
Goto(bb1)
}
bb1 = {
_4 = RET;
RET = _4;
_7 = 897_i16 as i128;
RET = 15794922916635709798_usize as isize;
_3 = core::ptr::addr_of_mut!(_4);
RET = (*_3) >> (*_3);
_7 = 435333771852029869_i64 as i128;
RET = (*_3);
_4 = RET;
_7 = (-104434222046397961031943936585029251487_i128);
(*_3) = -RET;
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
235848144874540502431430670846738959969 => bb9,
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
_7 = (-50637293892390467243487245300553628019_i128);
RET = !_4;
_7 = (-89403172060675718005070666838241025314_i128);
_7 = (-49652395517044804689484567369806122727_i128);
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb10,
290629971403893658773890040061962088729 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_5 = 25_u8 as f64;
_5 = 3_i8 as f64;
_7 = (-63021998621562577215638704093535422264_i128);
(*_3) = RET + RET;
(*_3) = -RET;
RET = (*_3);
_5 = 1857236647_i32 as f64;
_8 = _5 + _5;
match _7 {
0 => bb1,
1 => bb6,
2 => bb3,
277260368299375886247735903338232789192 => bb13,
_ => bb8
}
}
bb13 = {
(*_3) = RET << _7;
(*_3) = RET ^ RET;
_5 = _8 * _8;
RET = !(*_3);
_9 = [85_i8,(-49_i8),38_i8,80_i8,113_i8];
_8 = 3270525021_u32 as f64;
_8 = -_5;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !RET;
_3 = core::ptr::addr_of_mut!((*_3));
RET = !(*_3);
_3 = core::ptr::addr_of_mut!(_4);
_4 = RET;
_9 = [72_i8,(-42_i8),(-81_i8),24_i8,34_i8];
(*_3) = RET;
_3 = core::ptr::addr_of_mut!(_4);
match _7 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb9,
277260368299375886247735903338232789192 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_8 = _5 - _5;
RET = (*_3);
_5 = -_8;
(*_3) = !RET;
(*_3) = 8094875912530909482_i64 as isize;
_3 = core::ptr::addr_of_mut!(_10);
RET = _4 >> _4;
_8 = _5;
_5 = 13882421252420899683_u64 as f64;
_10 = (-2595846459821235526_i64) as isize;
_4 = RET;
_11 = _8 as f32;
RET = (*_3);
RET = 8890_i16 as isize;
Goto(bb16)
}
bb16 = {
Call(_12 = dump_var(17_usize, 10_usize, Move(_10), 4_usize, Move(_4), 13_usize, _13, 13_usize, _13), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i8,mut _2: isize,mut _3: [bool; 2],mut _4: (f64, [char; 7]),mut _5: f64,mut _6: [u8; 6],mut _7: (f32,),mut _8: f64,mut _9: (f64, [char; 7])) -> [char; 7] {
mir! {
type RET = [char; 7];
let _10: f64;
let _11: (*mut (f32,), i64);
let _12: u16;
let _13: [isize; 6];
let _14: (*mut (f32,), i64);
let _15: i32;
let _16: f32;
let _17: bool;
let _18: u8;
let _19: bool;
let _20: u64;
let _21: isize;
let _22: (u128,);
let _23: &'static (u32, i128, isize, *mut i128);
let _24: u32;
let _25: &'static [bool; 2];
let _26: &'static [i32; 7];
let _27: isize;
let _28: ();
let _29: ();
{
_3 = [true,false];
_4.0 = (-13289_i16) as f64;
_8 = _5;
_4.0 = _8;
_9.0 = _2 as f64;
_9.0 = -_8;
_9.0 = _5;
_10 = _9.0;
_9 = _4;
_9 = _4;
RET = ['\u{93c30}','\u{10bf7f}','\u{b0994}','\u{b47af}','\u{3fa99}','\u{3112c}','\u{1b0e5}'];
_4.0 = 1_usize as f64;
RET = ['\u{a7d98}','\u{c408b}','\u{69f2f}','\u{4b462}','\u{10e7d4}','\u{b284f}','\u{609ae}'];
_10 = (-3722_i16) as f64;
RET = ['\u{56436}','\u{8170a}','\u{1a084}','\u{6f9a2}','\u{a631b}','\u{5e9fb}','\u{e9cdd}'];
_5 = -_8;
_10 = 4171538358_u32 as f64;
_8 = -_4.0;
_1 = (-50_i8) ^ (-119_i8);
_3 = [false,true];
_7.0 = 96035313804124463111239858519948855100_i128 as f32;
_10 = _8;
Call(_9.0 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = ['\u{9b20c}','\u{58ab9}','\u{d0b0a}','\u{6a7c5}','\u{a360a}','\u{eeacc}','\u{b3d84}'];
_7.0 = 17911342725793191234_u64 as f32;
_11.0 = core::ptr::addr_of_mut!(_7);
_6 = [108_u8,180_u8,193_u8,126_u8,23_u8,30_u8];
_8 = _9.0 + _9.0;
RET = ['\u{106736}','\u{e7c10}','\u{e127f}','\u{624ef}','\u{38237}','\u{166b7}','\u{8a427}'];
Goto(bb2)
}
bb2 = {
RET = _9.1;
_10 = _8 + _8;
_9.1 = ['\u{81f19}','\u{41b27}','\u{b62f1}','\u{da3be}','\u{78786}','\u{94743}','\u{d4881}'];
_11.1 = (-5912079907061881989_i64);
Goto(bb3)
}
bb3 = {
_3 = [true,false];
Call(_1 = core::intrinsics::bswap(46_i8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8 = _2 as f64;
_5 = (-336735141_i32) as f64;
_1 = '\u{b5b54}' as i8;
Goto(bb5)
}
bb5 = {
_10 = -_9.0;
_11.1 = '\u{e97c7}' as i64;
_4.1 = ['\u{c55e2}','\u{d76eb}','\u{d153e}','\u{54318}','\u{1056dd}','\u{8517e}','\u{c519f}'];
_13 = [_2,_2,_2,_2,_2,_2];
_4 = _9;
RET = ['\u{c029c}','\u{c3f54}','\u{3145}','\u{82d40}','\u{fecc9}','\u{93115}','\u{af8c4}'];
_4.1 = ['\u{27c99}','\u{8115b}','\u{43311}','\u{545a3}','\u{990e6}','\u{35339}','\u{95159}'];
_2 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_4.0 = _10 + _10;
_14 = Move(_11);
_10 = 57314275503581174314739154898271856167_u128 as f64;
_8 = _5 + _9.0;
Goto(bb6)
}
bb6 = {
_7.0 = 84_u8 as f32;
_11.0 = Move(_14.0);
_7.0 = 7272342672213497874_u64 as f32;
_9.0 = _4.0 + _4.0;
_11.1 = _14.1 | _14.1;
_15 = (-194413427_i32);
_17 = !true;
_16 = -_7.0;
_1 = (-26_i8);
_5 = _4.0;
_12 = 49035_u16 - 4716_u16;
_1 = 72_i8 | (-47_i8);
_4 = (_9.0, _9.1);
_14.1 = _11.1 + _11.1;
_6 = [199_u8,100_u8,155_u8,40_u8,169_u8,57_u8];
_9.0 = _4.0;
_4.1 = ['\u{e384f}','\u{c409e}','\u{61b2f}','\u{2bea9}','\u{108ca5}','\u{5ac50}','\u{ed5cf}'];
_19 = _17 ^ _17;
_14.1 = _11.1;
_21 = (-113527233768074292368657018499983694531_i128) as isize;
match _15 {
0 => bb4,
340282366920938463463374607431573798029 => bb7,
_ => bb2
}
}
bb7 = {
_2 = _21;
_22 = (10494146579519533283922991825165377434_u128,);
_4.0 = _12 as f64;
_14.0 = core::ptr::addr_of_mut!(_7);
RET = ['\u{58f0d}','\u{6c213}','\u{9e8cc}','\u{483aa}','\u{647b6}','\u{53908}','\u{f43b1}'];
_16 = _7.0 + _7.0;
_11 = (Move(_14.0), _14.1);
_9 = (_5, _4.1);
_14 = Move(_11);
RET = ['\u{86092}','\u{e4818}','\u{4d325}','\u{26dbd}','\u{99735}','\u{e97a8}','\u{5a42e}'];
match _22.0 {
0 => bb8,
1 => bb9,
10494146579519533283922991825165377434 => bb11,
_ => bb10
}
}
bb8 = {
RET = _9.1;
_10 = _8 + _8;
_9.1 = ['\u{81f19}','\u{41b27}','\u{b62f1}','\u{da3be}','\u{78786}','\u{94743}','\u{d4881}'];
_11.1 = (-5912079907061881989_i64);
Goto(bb3)
}
bb9 = {
_10 = -_9.0;
_11.1 = '\u{e97c7}' as i64;
_4.1 = ['\u{c55e2}','\u{d76eb}','\u{d153e}','\u{54318}','\u{1056dd}','\u{8517e}','\u{c519f}'];
_13 = [_2,_2,_2,_2,_2,_2];
_4 = _9;
RET = ['\u{c029c}','\u{c3f54}','\u{3145}','\u{82d40}','\u{fecc9}','\u{93115}','\u{af8c4}'];
_4.1 = ['\u{27c99}','\u{8115b}','\u{43311}','\u{545a3}','\u{990e6}','\u{35339}','\u{95159}'];
_2 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_4.0 = _10 + _10;
_14 = Move(_11);
_10 = 57314275503581174314739154898271856167_u128 as f64;
_8 = _5 + _9.0;
Goto(bb6)
}
bb10 = {
_3 = [true,false];
Call(_1 = core::intrinsics::bswap(46_i8), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_19 = _8 <= _5;
_1 = (-13_i8);
_11.1 = _14.1;
_6 = [79_u8,165_u8,156_u8,143_u8,220_u8,190_u8];
_1 = !51_i8;
_10 = -_8;
_22.0 = _1 as u128;
RET = ['\u{37b8f}','\u{d5ab}','\u{89cc1}','\u{b83bc}','\u{fde3c}','\u{44c15}','\u{8a3b8}'];
_13 = [_2,_2,_2,_2,_21,_21];
_18 = !14_u8;
_18 = 67_u8 | 30_u8;
_16 = _7.0;
_9.1 = ['\u{940c6}','\u{913fe}','\u{47169}','\u{14c2c}','\u{a6a49}','\u{778f3}','\u{e53d0}'];
_11.0 = Move(_14.0);
RET = ['\u{9bc}','\u{5f4b7}','\u{9aeb8}','\u{5a5fa}','\u{10e509}','\u{1fb4c}','\u{9e7bc}'];
_9.0 = _1 as f64;
match _15 {
0 => bb4,
1 => bb3,
2 => bb12,
3 => bb13,
340282366920938463463374607431573798029 => bb15,
_ => bb14
}
}
bb12 = {
_8 = _2 as f64;
_5 = (-336735141_i32) as f64;
_1 = '\u{b5b54}' as i8;
Goto(bb5)
}
bb13 = {
_10 = -_9.0;
_11.1 = '\u{e97c7}' as i64;
_4.1 = ['\u{c55e2}','\u{d76eb}','\u{d153e}','\u{54318}','\u{1056dd}','\u{8517e}','\u{c519f}'];
_13 = [_2,_2,_2,_2,_2,_2];
_4 = _9;
RET = ['\u{c029c}','\u{c3f54}','\u{3145}','\u{82d40}','\u{fecc9}','\u{93115}','\u{af8c4}'];
_4.1 = ['\u{27c99}','\u{8115b}','\u{43311}','\u{545a3}','\u{990e6}','\u{35339}','\u{95159}'];
_2 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_4.0 = _10 + _10;
_14 = Move(_11);
_10 = 57314275503581174314739154898271856167_u128 as f64;
_8 = _5 + _9.0;
Goto(bb6)
}
bb14 = {
_7.0 = 84_u8 as f32;
_11.0 = Move(_14.0);
_7.0 = 7272342672213497874_u64 as f32;
_9.0 = _4.0 + _4.0;
_11.1 = _14.1 | _14.1;
_15 = (-194413427_i32);
_17 = !true;
_16 = -_7.0;
_1 = (-26_i8);
_5 = _4.0;
_12 = 49035_u16 - 4716_u16;
_1 = 72_i8 | (-47_i8);
_4 = (_9.0, _9.1);
_14.1 = _11.1 + _11.1;
_6 = [199_u8,100_u8,155_u8,40_u8,169_u8,57_u8];
_9.0 = _4.0;
_4.1 = ['\u{e384f}','\u{c409e}','\u{61b2f}','\u{2bea9}','\u{108ca5}','\u{5ac50}','\u{ed5cf}'];
_19 = _17 ^ _17;
_14.1 = _11.1;
_21 = (-113527233768074292368657018499983694531_i128) as isize;
match _15 {
0 => bb4,
340282366920938463463374607431573798029 => bb7,
_ => bb2
}
}
bb15 = {
_16 = 36490854324411457873197801711332082574_i128 as f32;
_19 = !_17;
_14 = (Move(_11.0), _11.1);
_10 = 44712622480308083673269377687169654585_i128 as f64;
_7 = (_16,);
_6 = [_18,_18,_18,_18,_18,_18];
_11.0 = core::ptr::addr_of_mut!(_7);
_7 = (_16,);
_11 = (Move(_14.0), _14.1);
_7 = (_16,);
_25 = &_3;
_10 = _5;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(18_usize, 15_usize, Move(_15), 13_usize, Move(_13), 19_usize, Move(_19), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(18_usize, 6_usize, Move(_6), 12_usize, Move(_12), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(118_u8), std::hint::black_box((-100_isize)), std::hint::black_box(51_i8), std::hint::black_box(13433_u16), std::hint::black_box(151281739907967475886251874055877531891_u128), std::hint::black_box(6363241902351870428_i64), std::hint::black_box(76250655530517381841044721491133766944_i128), std::hint::black_box(12751774507199200647_u64));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: u16,
fld1: u64,
fld2: f32,
fld3: (f32,),

},
Variant1{
fld0: [u8; 7],
fld1: isize,

},
Variant2{
fld0: i128,
fld1: char,
fld2: isize,
fld3: i8,
fld4: usize,
fld5: i32,

}}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: (usize,),
fld1: u16,
fld2: (char, [u64; 4], u16),
fld3: i8,
fld4: (f32,),

},
Variant1{
fld0: (f32,),
fld1: isize,

},
Variant2{
fld0: bool,
fld1: *const i128,
fld2: (char, [u64; 4], u16),
fld3: Adt24,
fld4: [i32; 7],
fld5: i32,
fld6: [char; 3],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: u64,
fld1: (f64, [char; 7]),
fld2: *const isize,
fld3: (f32,),
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: [u8; 3],
fld1: ([char; 3],),
fld2: [u128; 8],
fld3: *const [bool; 2],
fld4: (((f32,), (f32,), [bool; 2]), (u128,), *mut [char; 7]),
fld5: i32,
fld6: *mut usize,
fld7: f32,

},
Variant1{
fld0: bool,
fld1: (u32, i128, isize, *mut i128),
fld2: *mut [i32; 7],
fld3: Adt32,
fld4: [u128; 8],
fld5: *const isize,
fld6: ([char; 3],),
fld7: u64,

}}
impl PrintFDebug for Adt67{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt67{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt67 {
fld0: [i32; 7],
fld1: *mut (f32,),
fld2: u8,
fld3: f64,
fld4: f32,
fld5: i32,
fld6: *const (f64, [char; 7]),
fld7: *const isize,
}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt68{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt68 {
fld0: bool,
fld1: *const i128,
fld2: isize,
fld3: [i32; 7],
fld4: *mut i128,
fld5: Adt56,
fld6: [char; 3],
fld7: *mut [i32; 7],
}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt73{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt73 {
fld0: *const i128,
fld1: (u128,),
fld2: isize,
fld3: u64,
fld4: [i32; 7],
fld5: [u16; 6],
fld6: i64,
fld7: u32,
}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){unsafe{printf("Adt82::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt82 {
Variant0{
fld0: (*mut i128, [bool; 2]),
fld1: usize,
fld2: u8,
fld3: (u128,),
fld4: ((u128, Adt32, *mut [i32; 7], (f64, [char; 7])), Adt67, bool, u8),
fld5: *const (f64, [char; 7]),

},
Variant1{
fld0: ([char; 3],),
fld1: [u16; 4],

},
Variant2{
fld0: u32,
fld1: [u128; 3],
fld2: [char; 7],
fld3: Adt32,
fld4: *mut u64,
fld5: ((u128, Adt32, *mut [i32; 7], (f64, [char; 7])), Adt67, bool, u8),
fld6: (*mut u64, i16, usize),
fld7: *const (f64, [char; 7]),

}}

