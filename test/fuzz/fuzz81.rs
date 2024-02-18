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
pub fn fn0(mut _1: bool,mut _2: usize,mut _3: i64,mut _4: u8,mut _5: i16) -> f32 {
mir! {
type RET = f32;
let _6: ([char; 3],);
let _7: *mut Adt47;
let _8: [bool; 8];
let _9: u16;
let _10: [u64; 1];
let _11: *mut f64;
let _12: isize;
let _13: bool;
let _14: isize;
let _15: Adt18;
let _16: &'static f32;
let _17: ([u64; 1], Adt47, char);
let _18: (u128, i8, ([i64; 3], u16, usize, u32));
let _19: i64;
let _20: [u32; 3];
let _21: (f64,);
let _22: u8;
let _23: isize;
let _24: f32;
let _25: *const isize;
let _26: isize;
let _27: *const i16;
let _28: Adt65;
let _29: *mut i32;
let _30: u64;
let _31: usize;
let _32: i128;
let _33: ();
let _34: ();
{
RET = 728162413794109223_i64 as f32;
_1 = true;
_5 = 1615684154_u32 as i16;
RET = 538901934_i32 as f32;
Goto(bb1)
}
bb1 = {
RET = 214_u8 as f32;
_1 = !false;
_1 = true;
_2 = 2_usize ^ 7_usize;
_5 = 8183_i16 >> _2;
_3 = (-3393710124043555071_i64);
_4 = 215_u8;
RET = _2 as f32;
_5 = (-6850_i16);
_6.0 = ['\u{321e0}','\u{71b3c}','\u{25776}'];
_2 = 2114637825_i32 as usize;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = 198_u8;
_5 = (-10150_i16);
_3 = (-110_i8) as i64;
RET = _4 as f32;
_2 = 17199072122625531796_usize - 1946358334262609034_usize;
RET = (-181798417_i32) as f32;
_2 = 1_usize + 10724155620077776678_usize;
_1 = true;
_3 = (-2189257504174673226_i64);
_6.0 = ['\u{e64a8}','\u{fd51c}','\u{5e591}'];
_4 = !71_u8;
RET = 222852866_u32 as f32;
_3 = 151618902970849877194935418504888075042_u128 as i64;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768201306 => bb7,
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
_10 = [17927997739662059672_u64];
_1 = !false;
_3 = !7716141096197305959_i64;
match _5 {
340282366920938463463374607431768201306 => bb8,
_ => bb6
}
}
bb8 = {
_12 = -(-103_isize);
_10 = [5530214759423595575_u64];
_10 = [10523018348865978107_u64];
_12 = 9223372036854775807_isize;
_10 = [1683463451201882548_u64];
_4 = 234_u8 | 48_u8;
_4 = 23_u8;
_13 = !_1;
_2 = 72_i8 as usize;
RET = 1094917305_i32 as f32;
_9 = 15428_u16 - 33840_u16;
_8 = [_1,_13,_1,_1,_13,_13,_13,_1];
match _4 {
0 => bb3,
1 => bb7,
23 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_14 = _12 * _12;
_10 = [9722805850933875777_u64];
RET = _3 as f32;
RET = 119_i8 as f32;
_5 = (-12448_i16) << _14;
RET = _9 as f32;
_15 = Adt18::Variant0 { fld0: 171891917210543199316397713728495039637_u128,fld1: 3275897292_u32,fld2: _12,fld3: _2,fld4: _5,fld5: 2467041127014015630_u64,fld6: (-100272250829616883685973463547429642178_i128) };
_6.0 = ['\u{fd74b}','\u{5e5d5}','\u{f48c9}'];
_17.0 = [16389880903936155189_u64];
_5 = Field::<i16>(Variant(_15, 0), 4);
_18.2.0 = [_3,_3,_3];
_15 = Adt18::Variant0 { fld0: 222984060534048498889198899300527399147_u128,fld1: 615939419_u32,fld2: _14,fld3: _2,fld4: _5,fld5: 3370864924271593316_u64,fld6: (-96630724695616482099287038226782511618_i128) };
Call(_18.0 = fn1(_3, _6.0, _4, _14, _14, Field::<usize>(Variant(_15, 0), 3)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<usize>(Variant(_15, 0), 3)) = (-83_i8) as usize;
_17.1.fld0 = core::ptr::addr_of_mut!(_18.2.1);
RET = _3 as f32;
place!(Field::<usize>(Variant(_15, 0), 3)) = _2;
_15 = Adt18::Variant0 { fld0: _18.0,fld1: 1437776181_u32,fld2: _12,fld3: _2,fld4: _5,fld5: 4367546343362402818_u64,fld6: (-6729913539535440408336394793296501611_i128) };
_16 = &RET;
_14 = Field::<isize>(Variant(_15, 0), 2) + _12;
RET = _3 as f32;
_19 = _3;
_3 = _19;
place!(Field::<i128>(Variant(_15, 0), 6)) = Field::<i16>(Variant(_15, 0), 4) as i128;
_14 = !Field::<isize>(Variant(_15, 0), 2);
_19 = !_3;
place!(Field::<i16>(Variant(_15, 0), 4)) = _5;
Call(_17.1.fld0 = fn2(Field::<i16>(Variant(_15, 0), 4), Field::<u128>(Variant(_15, 0), 0), Field::<isize>(Variant(_15, 0), 2), Field::<i16>(Variant(_15, 0), 4), Field::<i128>(Variant(_15, 0), 6), Field::<isize>(Variant(_15, 0), 2), Field::<isize>(Variant(_15, 0), 2)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = Field::<u128>(Variant(_15, 0), 0) as u16;
_21.0 = (-117_i8) as f64;
_17.2 = '\u{96c8b}';
_18.2.3 = 1136280625_u32 << _19;
_23 = RET as isize;
place!(Field::<u64>(Variant(_15, 0), 5)) = _18.0 as u64;
_11 = core::ptr::addr_of_mut!(_21.0);
_20 = [_18.2.3,_18.2.3,_18.2.3];
_18.2.0 = [_19,_19,_3];
_3 = _19;
_18.1 = _17.2 as i8;
_20 = [_18.2.3,_18.2.3,_18.2.3];
place!(Field::<i128>(Variant(_15, 0), 6)) = -(-77948998531469953837666295702303843406_i128);
_13 = !_1;
_17.0 = _10;
match _4 {
0 => bb1,
1 => bb11,
2 => bb13,
23 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_12 = -(-103_isize);
_10 = [5530214759423595575_u64];
_10 = [10523018348865978107_u64];
_12 = 9223372036854775807_isize;
_10 = [1683463451201882548_u64];
_4 = 234_u8 | 48_u8;
_4 = 23_u8;
_13 = !_1;
_2 = 72_i8 as usize;
RET = 1094917305_i32 as f32;
_9 = 15428_u16 - 33840_u16;
_8 = [_1,_13,_1,_1,_13,_13,_13,_1];
match _4 {
0 => bb3,
1 => bb7,
23 => bb10,
_ => bb9
}
}
bb15 = {
place!(Field::<usize>(Variant(_15, 0), 3)) = _2;
_11 = core::ptr::addr_of_mut!((*_11));
_24 = RET + RET;
place!(Field::<u32>(Variant(_15, 0), 1)) = !_18.2.3;
_12 = _19 as isize;
_24 = _4 as f32;
_6.0 = [_17.2,_17.2,_17.2];
_12 = !_14;
_18.1 = (-4_i8);
_13 = _1;
_18.2.2 = !_2;
_18.0 = _23 as u128;
_18.0 = Field::<u128>(Variant(_15, 0), 0);
_25 = core::ptr::addr_of!(_14);
RET = (*_11) as f32;
place!(Field::<i128>(Variant(_15, 0), 6)) = -102942301735761212099350475423109648506_i128;
place!(Field::<i128>(Variant(_15, 0), 6)) = !11769733848978407233216175690238194020_i128;
_18.2.1 = _9 & _9;
_17.2 = '\u{9ef07}';
_22 = _17.2 as u8;
_6.0 = [_17.2,_17.2,_17.2];
_16 = &_24;
_17.0 = _10;
_27 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_15, 0), 4)));
_10 = [Field::<u64>(Variant(_15, 0), 5)];
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(0_usize, 13_usize, Move(_13), 20_usize, Move(_20), 19_usize, Move(_19), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 14_usize, Move(_14), 18_usize, Move(_18), 10_usize, Move(_10), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i64,mut _2: [char; 3],mut _3: u8,mut _4: isize,mut _5: isize,mut _6: usize) -> u128 {
mir! {
type RET = u128;
let _7: &'static Adt32;
let _8: *mut Adt47;
let _9: [i64; 3];
let _10: Adt47;
let _11: &'static f32;
let _12: bool;
let _13: ([i64; 3], u16, usize, u32);
let _14: u32;
let _15: [u64; 4];
let _16: u128;
let _17: isize;
let _18: usize;
let _19: u8;
let _20: Adt65;
let _21: usize;
let _22: i8;
let _23: ([bool; 6], (u128, i8, ([i64; 3], u16, usize, u32)));
let _24: f32;
let _25: f32;
let _26: Adt47;
let _27: ();
let _28: ();
{
_4 = _5 >> _3;
_3 = 61_u8;
RET = 315839905717101820742172354073195226988_u128;
_2 = ['\u{106bff}','\u{9bde2}','\u{8fc82}'];
_1 = -(-1071661393720569708_i64);
match RET {
315839905717101820742172354073195226988 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_1 = (-3616742580449415548_i64);
_6 = 11585372324553693264_usize;
_3 = !71_u8;
_1 = !(-5992853484184766303_i64);
_5 = -_4;
_8 = core::ptr::addr_of_mut!(_10);
_6 = 26781_i16 as usize;
_4 = !_5;
_3 = 187_u8 | 159_u8;
_9 = [_1,_1,_1];
_1 = !4222784930470535982_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = 5_usize | 0_usize;
_5 = 13_i8 as isize;
_4 = !_5;
Goto(bb3)
}
bb3 = {
_9 = [_1,_1,_1];
_1 = -1923659808793665317_i64;
_5 = !_4;
_6 = 3_usize | 3_usize;
_6 = 14081006948179641688_usize;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_8 = core::ptr::addr_of_mut!((*_8));
_13 = (_9, 25310_u16, _6, 1882754439_u32);
_9 = [_1,_1,_1];
_13.3 = 420447425_u32;
_13.0 = [_1,_1,_1];
_13 = (_9, 4330_u16, _6, 3998228399_u32);
_13.2 = !_6;
_12 = false;
RET = 75271730486514204465457893653246897982_u128;
_2 = ['\u{d3a94}','\u{9d5c1}','\u{929f}'];
_13.1 = !42517_u16;
_9 = [_1,_1,_1];
_13.0 = [_1,_1,_1];
_13.3 = 1999527655_u32;
(*_8).fld0 = core::ptr::addr_of_mut!(_13.1);
_1 = 1366691997850185273_i64 * (-3201277531035933826_i64);
_8 = core::ptr::addr_of_mut!((*_8));
_8 = core::ptr::addr_of_mut!(_10);
_13.2 = !_6;
match _13.3 {
0 => bb1,
1999527655 => bb4,
_ => bb2
}
}
bb4 = {
_13.3 = 1879226114_u32;
(*_8).fld0 = core::ptr::addr_of_mut!(_13.1);
_13.1 = !15845_u16;
_3 = 105_u8;
_1 = _13.3 as i64;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_9 = [_1,_1,_1];
_12 = !true;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_6 = _13.2 + _13.2;
_12 = true & true;
RET = 318170539849018031845896620057835747818_u128;
_9 = _13.0;
RET = 234904683272932552800902880591078677439_u128 - 320872893208873158936856843434770744920_u128;
RET = 523518766_i32 as u128;
_3 = _4 as u8;
_5 = !_4;
_4 = 387337269_i32 as isize;
_13.1 = 31082_u16;
_1 = '\u{1423d}' as i64;
_8 = core::ptr::addr_of_mut!(_10);
_5 = _4;
_4 = -_5;
RET = _3 as u128;
Goto(bb5)
}
bb5 = {
_13.3 = 2207245219_u32 ^ 3921385609_u32;
_14 = 9243410273282844300_u64 as u32;
_2 = ['\u{8262e}','\u{b5d93}','\u{53256}'];
_6 = _13.2;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_13.1 = _14 as u16;
_9 = [_1,_1,_1];
_8 = core::ptr::addr_of_mut!((*_8));
RET = 128096507447750486423947990578066801208_u128;
RET = _1 as u128;
_1 = 4716966192853919699_i64;
(*_8).fld0 = core::ptr::addr_of_mut!(_13.1);
_15 = [9709359952738254054_u64,8741216228990268312_u64,14779906626342227525_u64,520233056818420924_u64];
RET = _3 as u128;
_14 = !_13.3;
_16 = !RET;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_12 = true;
_8 = core::ptr::addr_of_mut!((*_8));
_15 = [16519954019036254634_u64,9024695280734064627_u64,10758191696939100377_u64,8923111162475355098_u64];
(*_8).fld0 = core::ptr::addr_of_mut!(_13.1);
match _1 {
0 => bb1,
1 => bb3,
4716966192853919699 => bb7,
_ => bb6
}
}
bb6 = {
_1 = (-3616742580449415548_i64);
_6 = 11585372324553693264_usize;
_3 = !71_u8;
_1 = !(-5992853484184766303_i64);
_5 = -_4;
_8 = core::ptr::addr_of_mut!(_10);
_6 = 26781_i16 as usize;
_4 = !_5;
_3 = 187_u8 | 159_u8;
_9 = [_1,_1,_1];
_1 = !4222784930470535982_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = 5_usize | 0_usize;
_5 = 13_i8 as isize;
_4 = !_5;
Goto(bb3)
}
bb7 = {
_16 = RET;
_14 = _13.3;
_12 = _16 == _16;
_6 = (-73215639172633226116109466003369378572_i128) as usize;
_14 = _13.3;
_9 = _13.0;
_13.2 = _6 >> _13.1;
_13.3 = !_14;
_1 = 8796265632221979670_i64 ^ (-237252495397951445_i64);
_18 = !_6;
_13.2 = _18;
_13.3 = _14;
_2 = ['\u{f350d}','\u{d8fdc}','\u{6cbb1}'];
_13.2 = !_18;
_9 = [_1,_1,_1];
_12 = _5 != _4;
_13.3 = !_14;
_16 = RET ^ RET;
_8 = core::ptr::addr_of_mut!((*_8));
_13.3 = _14;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_4 = _5;
_9 = [_1,_1,_1];
_16 = _3 as u128;
_4 = _5;
_13 = (_9, 54676_u16, _6, _14);
RET = _13.3 as u128;
_3 = 164_u8 + 74_u8;
match _13.1 {
0 => bb1,
1 => bb6,
54676 => bb9,
_ => bb8
}
}
bb8 = {
_1 = (-3616742580449415548_i64);
_6 = 11585372324553693264_usize;
_3 = !71_u8;
_1 = !(-5992853484184766303_i64);
_5 = -_4;
_8 = core::ptr::addr_of_mut!(_10);
_6 = 26781_i16 as usize;
_4 = !_5;
_3 = 187_u8 | 159_u8;
_9 = [_1,_1,_1];
_1 = !4222784930470535982_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = 5_usize | 0_usize;
_5 = 13_i8 as isize;
_4 = !_5;
Goto(bb3)
}
bb9 = {
_12 = !true;
_23.1.2 = (_9, _13.1, _6, _14);
_23.1.1 = 47_i8 - 117_i8;
_23.1.0 = _16 * RET;
_21 = _13.2 * _18;
RET = _23.1.0;
_23.1.2 = (_9, _13.1, _21, _14);
_12 = _23.1.2.2 >= _23.1.2.2;
match _23.1.2.1 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
54676 => bb15,
_ => bb14
}
}
bb10 = {
_1 = (-3616742580449415548_i64);
_6 = 11585372324553693264_usize;
_3 = !71_u8;
_1 = !(-5992853484184766303_i64);
_5 = -_4;
_8 = core::ptr::addr_of_mut!(_10);
_6 = 26781_i16 as usize;
_4 = !_5;
_3 = 187_u8 | 159_u8;
_9 = [_1,_1,_1];
_1 = !4222784930470535982_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = 5_usize | 0_usize;
_5 = 13_i8 as isize;
_4 = !_5;
Goto(bb3)
}
bb11 = {
_16 = RET;
_14 = _13.3;
_12 = _16 == _16;
_6 = (-73215639172633226116109466003369378572_i128) as usize;
_14 = _13.3;
_9 = _13.0;
_13.2 = _6 >> _13.1;
_13.3 = !_14;
_1 = 8796265632221979670_i64 ^ (-237252495397951445_i64);
_18 = !_6;
_13.2 = _18;
_13.3 = _14;
_2 = ['\u{f350d}','\u{d8fdc}','\u{6cbb1}'];
_13.2 = !_18;
_9 = [_1,_1,_1];
_12 = _5 != _4;
_13.3 = !_14;
_16 = RET ^ RET;
_8 = core::ptr::addr_of_mut!((*_8));
_13.3 = _14;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_4 = _5;
_9 = [_1,_1,_1];
_16 = _3 as u128;
_4 = _5;
_13 = (_9, 54676_u16, _6, _14);
RET = _13.3 as u128;
_3 = 164_u8 + 74_u8;
match _13.1 {
0 => bb1,
1 => bb6,
54676 => bb9,
_ => bb8
}
}
bb12 = {
_1 = (-3616742580449415548_i64);
_6 = 11585372324553693264_usize;
_3 = !71_u8;
_1 = !(-5992853484184766303_i64);
_5 = -_4;
_8 = core::ptr::addr_of_mut!(_10);
_6 = 26781_i16 as usize;
_4 = !_5;
_3 = 187_u8 | 159_u8;
_9 = [_1,_1,_1];
_1 = !4222784930470535982_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = 5_usize | 0_usize;
_5 = 13_i8 as isize;
_4 = !_5;
Goto(bb3)
}
bb13 = {
_1 = (-3616742580449415548_i64);
_6 = 11585372324553693264_usize;
_3 = !71_u8;
_1 = !(-5992853484184766303_i64);
_5 = -_4;
_8 = core::ptr::addr_of_mut!(_10);
_6 = 26781_i16 as usize;
_4 = !_5;
_3 = 187_u8 | 159_u8;
_9 = [_1,_1,_1];
_1 = !4222784930470535982_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_6 = 5_usize | 0_usize;
_5 = 13_i8 as isize;
_4 = !_5;
Goto(bb3)
}
bb14 = {
_13.3 = 1879226114_u32;
(*_8).fld0 = core::ptr::addr_of_mut!(_13.1);
_13.1 = !15845_u16;
_3 = 105_u8;
_1 = _13.3 as i64;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_9 = [_1,_1,_1];
_12 = !true;
_10.fld0 = core::ptr::addr_of_mut!(_13.1);
_6 = _13.2 + _13.2;
_12 = true & true;
RET = 318170539849018031845896620057835747818_u128;
_9 = _13.0;
RET = 234904683272932552800902880591078677439_u128 - 320872893208873158936856843434770744920_u128;
RET = 523518766_i32 as u128;
_3 = _4 as u8;
_5 = !_4;
_4 = 387337269_i32 as isize;
_13.1 = 31082_u16;
_1 = '\u{1423d}' as i64;
_8 = core::ptr::addr_of_mut!(_10);
_5 = _4;
_4 = -_5;
RET = _3 as u128;
Goto(bb5)
}
bb15 = {
_5 = _4;
_23.1.2.3 = _14;
_23.1.2.0 = [_1,_1,_1];
_26.fld0 = Move((*_8).fld0);
_14 = _23.1.1 as u32;
_23.0 = [_12,_12,_12,_12,_12,_12];
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(1_usize, 2_usize, Move(_2), 4_usize, Move(_4), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(1_usize, 16_usize, Move(_16), 15_usize, Move(_15), 21_usize, Move(_21), 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i16,mut _2: u128,mut _3: isize,mut _4: i16,mut _5: i128,mut _6: isize,mut _7: isize) -> *mut u16 {
mir! {
type RET = *mut u16;
let _8: f64;
let _9: Adt65;
let _10: bool;
let _11: [bool; 8];
let _12: [bool; 8];
let _13: (u128, i8, ([i64; 3], u16, usize, u32));
let _14: *mut *const isize;
let _15: ([u64; 1], Adt47, char);
let _16: [u64; 4];
let _17: (isize, [u64; 1], u64, [i64; 3]);
let _18: isize;
let _19: &'static &'static Adt32;
let _20: ();
let _21: ();
{
_4 = 214_u8 as i16;
_6 = _4 as isize;
_4 = _1 >> _1;
_5 = _4 as i128;
_5 = 4207835840_u32 as i128;
_3 = _7;
Call(_2 = fn3(_7, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = -_4;
_8 = 42_i8 as f64;
_1 = _4;
_6 = _8 as isize;
Goto(bb2)
}
bb2 = {
_7 = _5 as isize;
match _3 {
0 => bb3,
1 => bb4,
2 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb3 = {
_1 = -_4;
_8 = 42_i8 as f64;
_1 = _4;
_6 = _8 as isize;
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
_5 = 163672987261380686708600368044876000935_i128 * 140170588644675873262314073639077090544_i128;
_10 = false ^ true;
_1 = _5 as i16;
_8 = (-269928406_i32) as f64;
_6 = _3 & _3;
_10 = true;
_6 = _3 ^ _7;
_1 = _4;
_4 = _1 | _1;
_6 = _7 ^ _7;
_6 = _3 << _4;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_5 = !138326555039004111984660976485497371167_i128;
_3 = _6;
_5 = -(-78858976632908475318233217417774149504_i128);
_11 = _12;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_1 = !_4;
_5 = !(-43655781109174755290766019780763587167_i128);
_13.2.0 = [153737898770032396_i64,4064866507418687923_i64,(-2661353139486082471_i64)];
_3 = 3609412158_u32 as isize;
_13.2.3 = !3982040158_u32;
_13.1 = 78_i8 ^ (-72_i8);
_13.2.2 = 5_usize;
_15.2 = '\u{3ad53}';
_13.0 = !_2;
_10 = !false;
match _13.2.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb8,
4 => bb9,
5 => bb11,
_ => bb10
}
}
bb8 = {
_1 = -_4;
_8 = 42_i8 as f64;
_1 = _4;
_6 = _8 as isize;
Goto(bb2)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
RET = core::ptr::addr_of_mut!(_13.2.1);
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_16 = [9379245185937232700_u64,2259323624762840120_u64,17755614128472851593_u64,1462698226225526300_u64];
_15.2 = '\u{a02af}';
(*RET) = 5223231424375225178_i64 as u16;
_13.2.1 = 14522_u16;
_3 = _6;
_13.1 = !(-11_i8);
_13.2.2 = 9953907389229165341_usize + 3_usize;
_2 = _13.0;
_5 = (-48472304834819480580083208763054725241_i128);
(*RET) = 57530_u16;
RET = core::ptr::addr_of_mut!((*RET));
_13.1 = (-36_i8);
_13.1 = (-6362589844064272375_i64) as i8;
_1 = _4;
_1 = _4;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_17.0 = 28_u8 as isize;
_18 = _6 * _6;
_5 = -54403981929203336888097363880791959019_i128;
Goto(bb12)
}
bb12 = {
_17.1 = [7267172280926847034_u64];
_8 = _6 as f64;
_17.1 = [313492123885202972_u64];
_13.2.3 = !1841471262_u32;
_16 = [1225267139889846677_u64,14156231096132611660_u64,5300208184433653410_u64,16479501621884636658_u64];
_2 = _13.1 as u128;
_15.2 = '\u{58886}';
_17.2 = 10823362697603953747_u64 << _18;
_17.0 = _10 as isize;
_18 = _3;
_6 = -_3;
_13.2.2 = 8684173499510923829_usize;
_7 = 100_u8 as isize;
_13.2.3 = _5 as u32;
_5 = 71167969277181671857625127611300333430_i128;
_13.2.1 = 37344_u16;
_17.3 = _13.2.0;
_15.0 = _17.1;
match (*RET) {
0 => bb1,
1 => bb10,
2 => bb6,
3 => bb4,
4 => bb13,
37344 => bb15,
_ => bb14
}
}
bb13 = {
_1 = -_4;
_8 = 42_i8 as f64;
_1 = _4;
_6 = _8 as isize;
Goto(bb2)
}
bb14 = {
_1 = -_4;
_8 = 42_i8 as f64;
_1 = _4;
_6 = _8 as isize;
Goto(bb2)
}
bb15 = {
_10 = true;
(*RET) = 64139_u16 - 38145_u16;
_15.1.fld0 = Move(RET);
_7 = !_18;
_10 = false;
_5 = _15.2 as i128;
_11 = [_10,_10,_10,_10,_10,_10,_10,_10];
_18 = _7 << _13.2.2;
_13.2.0 = _17.3;
RET = core::ptr::addr_of_mut!(_13.2.1);
_13.2.1 = _4 as u16;
_7 = _3;
_13.2.0 = [4435541149955251340_i64,5899167076217675574_i64,1725641821359781101_i64];
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(2_usize, 10_usize, Move(_10), 6_usize, Move(_6), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(2_usize, 16_usize, Move(_16), 1_usize, Move(_1), 18_usize, Move(_18), 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: i16) -> u128 {
mir! {
type RET = u128;
let _3: f32;
let _4: Adt60;
let _5: i32;
let _6: u64;
let _7: &'static &'static Adt32;
let _8: &'static i64;
let _9: f32;
let _10: *mut Adt18;
let _11: *const *mut &'static i64;
let _12: f64;
let _13: [i64; 6];
let _14: (u128, i8, ([i64; 3], u16, usize, u32));
let _15: [char; 4];
let _16: i8;
let _17: Adt60;
let _18: &'static Adt32;
let _19: isize;
let _20: Adt66;
let _21: *mut *const isize;
let _22: i32;
let _23: [u32; 1];
let _24: *mut [i8; 8];
let _25: i16;
let _26: [bool; 8];
let _27: bool;
let _28: ();
let _29: ();
{
_1 = (-9223372036854775808_isize);
_2 = 2319_i16 | (-16354_i16);
RET = !271526085110202233328462065363451464119_u128;
_3 = 447910944_i32 as f32;
RET = 168569307555255265587136583270596740052_u128 << _2;
RET = 96638070233291203219041998864015552788_u128 | 239778230370659456876174976291094344344_u128;
_2 = (-2307_i16) & 28707_i16;
_3 = RET as f32;
_2 = 17998717731614022882_usize as i16;
_6 = 61688910_i32 as u64;
_2 = 18634_i16;
_1 = (-9223372036854775808_isize);
_2 = -(-16761_i16);
RET = 50879609426271125363095533096365739633_u128 - 293355038380965687968970349775816120157_u128;
RET = '\u{d521e}' as u128;
_1 = !(-95_isize);
_5 = _2 as i32;
_5 = 835349599_i32;
_5 = 3643_u16 as i32;
_1 = 9223372036854775807_isize;
_5 = (-1213978166_i32);
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607430554233290 => bb6,
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
RET = 182676364837301768906674548352284685379_u128 - 34593470629326512668670651718664557989_u128;
RET = !339440603442234226583214243666685440614_u128;
Goto(bb7)
}
bb7 = {
_2 = '\u{b5a73}' as i16;
_5 = -928386241_i32;
_1 = '\u{31f77}' as isize;
_6 = 4354828238895827242_usize as u64;
_1 = !(-9223372036854775808_isize);
_2 = '\u{bcb45}' as i16;
_2 = !(-2139_i16);
_6 = _1 as u64;
_9 = _6 as f32;
RET = 116416447055063611030504725623337112812_u128;
_1 = 5000_u16 as isize;
_1 = 53504_u16 as isize;
_1 = (-85_isize);
RET = 95136061864373473144938501997064728337_u128 | 230714809569617322114865210361609821288_u128;
_13 = [3648938150116347595_i64,(-8497563300309746617_i64),5899070942308164703_i64,(-7467851741112261892_i64),2598754893842758721_i64,(-3435193010005180318_i64)];
_14.0 = 20143_u16 as u128;
_14.1 = 4279856582_u32 as i8;
RET = _14.0;
_5 = -(-897640330_i32);
_2 = !(-11085_i16);
_14.1 = 84_i8;
_14.2.0 = [2805531024399126219_i64,(-2163974584966303328_i64),(-4947132114746650098_i64)];
_14.2.1 = !64359_u16;
_13 = [1455193307369602370_i64,(-7080035046155870618_i64),8759331193303274075_i64,6589429665670654285_i64,(-5879651829039348736_i64),8898149294873471637_i64];
match _14.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
84 => bb8,
_ => bb6
}
}
bb8 = {
_15 = ['\u{be961}','\u{7cd3d}','\u{7c808}','\u{f82d8}'];
_14.2.3 = 728416057_u32 ^ 2686924346_u32;
_14.2.1 = !7092_u16;
_14.0 = !RET;
_16 = !_14.1;
_14.0 = _3 as u128;
_12 = 5_usize as f64;
_14.2.3 = 3882214810_u32;
_3 = _1 as f32;
_9 = RET as f32;
_14.0 = !RET;
_13 = [6202467734850790345_i64,(-469833453480536678_i64),(-2214763678597579290_i64),(-759392070113880257_i64),1201189290859439838_i64,(-8298106715097321523_i64)];
RET = _14.0;
_5 = -1908925006_i32;
RET = _14.0;
match _14.1 {
0 => bb6,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb9,
84 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_16 = !_14.1;
_14.2.0 = [(-8076036100365031507_i64),4634517556719909159_i64,(-5488792323318757767_i64)];
_15 = ['\u{59da7}','\u{4b939}','\u{94ef5}','\u{8665c}'];
_14.1 = _16 * _16;
_14.0 = RET | RET;
_1 = _14.2.1 as isize;
_14.0 = !RET;
_14.1 = _16 - _16;
_14.2.2 = 1_usize << _5;
_13 = [(-4860635290059387444_i64),(-3706712506105111333_i64),848850306037710532_i64,2482328045103675479_i64,(-4414153134978599788_i64),5620845546949055590_i64];
_3 = -_9;
_19 = _1 ^ _1;
_16 = _14.1 + _14.1;
RET = _16 as u128;
_14.2.0 = [(-8395875707026359609_i64),(-4239841802189222622_i64),(-75593917711144764_i64)];
_2 = _5 as i16;
_14.2.1 = 26627_u16 >> _16;
_14.0 = RET >> RET;
_14.2.3 = _6 as u32;
_7 = &_18;
Goto(bb12)
}
bb12 = {
_3 = _9 + _9;
RET = _14.0;
_15 = ['\u{b96d8}','\u{5bf31}','\u{46300}','\u{aa79d}'];
_14.2.3 = 802549578_u32 >> RET;
RET = !_14.0;
_7 = &(*_7);
_15 = ['\u{de853}','\u{80ebb}','\u{53f3d}','\u{eb4a}'];
_5 = (-1085287317_i32);
match _5 {
0 => bb11,
340282366920938463463374607430682924139 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_14.2.2 = !2_usize;
_1 = -_19;
_16 = _14.1 + _14.1;
_1 = 71205915169987219426859444770853321435_i128 as isize;
_3 = _9 + _9;
_6 = 11353979522095451539_u64 ^ 14746213186136923772_u64;
_7 = &(*_7);
_7 = &(*_7);
_5 = RET as i32;
_14.2.0 = [9005859461257959284_i64,643452255873411785_i64,7517173752369335854_i64];
_14.0 = _5 as u128;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(3_usize, 1_usize, Move(_1), 14_usize, Move(_14), 15_usize, Move(_15), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(7280992753865607460_usize), std::hint::black_box((-5241932353817519990_i64)), std::hint::black_box(32_u8), std::hint::black_box(20123_i16));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: u128,
fld1: u32,
fld2: isize,
fld3: usize,
fld4: i16,
fld5: u64,
fld6: i128,

},
Variant1{
fld0: bool,
fld1: f32,
fld2: u16,
fld3: f64,
fld4: u64,
fld5: i32,
fld6: u128,
fld7: u32,

}}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
fld0: u128,
fld1: *const isize,
fld2: isize,
fld3: i8,
fld4: Adt18,
fld5: i32,
fld6: i64,
fld7: ([i64; 3], u16, usize, u32),

},
Variant1{
fld0: u8,
fld1: *mut f64,
fld2: isize,
fld3: *const isize,
fld4: i16,
fld5: i32,
fld6: f32,

},
Variant2{
fld0: f64,
fld1: u8,
fld2: u32,
fld3: i8,
fld4: (isize, [u64; 1], u64, [i64; 3]),
fld5: i128,
fld6: [i8; 8],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: *mut u16,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: bool,
fld1: ([u64; 1], Adt47, char),
fld2: f32,
fld3: u16,
fld4: (isize, [u64; 1], u64, [i64; 3]),
fld5: u64,
fld6: u128,
fld7: usize,

},
Variant1{
fld0: *mut *const isize,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: f32,
fld1: *mut u16,
fld2: *mut *const isize,
fld3: [u32; 3],
fld4: *mut i32,

},
Variant1{
fld0: *mut [i8; 8],
fld1: u32,
fld2: usize,
fld3: (u128, i8, ([i64; 3], u16, usize, u32)),
fld4: [u64; 1],
fld5: ([u64; 1], Adt47, char),
fld6: i64,

},
Variant2{
fld0: ([char; 3],),
fld1: *mut u16,

},
Variant3{
fld0: *const Adt18,
fld1: char,
fld2: [i64; 3],
fld3: *mut i32,
fld4: [bool; 6],

}}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){unsafe{printf("Adt66::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt66 {
Variant0{
fld0: bool,
fld1: usize,
fld2: *mut f64,
fld3: (*mut f64, (u128, i8, ([i64; 3], u16, usize, u32)), (char,)),
fld4: [u64; 7],
fld5: *const isize,
fld6: [i8; 8],

},
Variant1{
fld0: f32,
fld1: f64,
fld2: *const isize,

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){unsafe{printf("Adt74::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt74 {
Variant0{
fld0: *mut Adt47,

},
Variant1{
fld0: [i64; 3],
fld1: *mut i32,

},
Variant2{
fld0: u32,
fld1: *const isize,
fld2: [u16; 1],
fld3: Adt66,
fld4: i16,

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
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: ([u64; 1], Adt47, char),
fld1: u64,
fld2: u32,

},
Variant1{
fld0: f32,
fld1: [i32; 5],
fld2: Adt47,
fld3: [u64; 7],

},
Variant2{
fld0: *const isize,
fld1: ([bool; 6], (u128, i8, ([i64; 3], u16, usize, u32))),
fld2: [i64; 3],
fld3: ([i64; 3], u16, usize, u32),
fld4: [i8; 8],

},
Variant3{
fld0: Adt32,
fld1: *mut [i8; 8],
fld2: [u64; 7],
fld3: Adt18,

}}

