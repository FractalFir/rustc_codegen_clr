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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: isize,mut _4: u8,mut _5: u128,mut _6: i32,mut _7: i64,mut _8: u32) -> isize {
mir! {
type RET = isize;
let _9: usize;
let _10: isize;
let _11: isize;
let _12: Adt33;
let _13: i64;
let _14: f64;
let _15: bool;
let _16: u128;
let _17: f64;
let _18: isize;
let _19: [i8; 5];
let _20: *mut Adt22;
let _21: isize;
let _22: Adt73;
let _23: u128;
let _24: (*const f32,);
let _25: &'static &'static (&'static u64,);
let _26: ();
let _27: ();
{
RET = !9223372036854775807_isize;
_6 = !(-777054960_i32);
_9 = 15448134378518293057_u64 as usize;
_9 = 17330078274323400790_usize;
_3 = RET;
Call(_8 = fn1(RET, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 176_u8 as u64;
_3 = RET;
_5 = (-38789648882782530563577415787881736295_i128) as u128;
_4 = 47_u8;
_7 = true as i64;
RET = -_3;
_5 = 76404778942689606404332141299605473255_u128 & 136398411552932251785838264695344147985_u128;
RET = _3;
RET = _3 + _3;
_10 = _3 ^ RET;
_6 = -423883189_i32;
RET = _3;
_9 = 14770604334121210189_usize;
_3 = _5 as isize;
_1 = 13674512775669453971_u64;
Goto(bb2)
}
bb2 = {
_1 = (-92_i8) as u64;
RET = !_10;
_1 = !3871339127264795009_u64;
_11 = (-2807_i16) as isize;
_4 = 139_u8 + 40_u8;
RET = false as isize;
_3 = _10;
_3 = _8 as isize;
_8 = 2239494907_u32;
_4 = _8 as u8;
_6 = -1571491935_i32;
RET = _10 & _10;
match _9 {
0 => bb3,
14770604334121210189 => bb5,
_ => bb4
}
}
bb3 = {
_1 = 176_u8 as u64;
_3 = RET;
_5 = (-38789648882782530563577415787881736295_i128) as u128;
_4 = 47_u8;
_7 = true as i64;
RET = -_3;
_5 = 76404778942689606404332141299605473255_u128 & 136398411552932251785838264695344147985_u128;
RET = _3;
RET = _3 + _3;
_10 = _3 ^ RET;
_6 = -423883189_i32;
RET = _3;
_9 = 14770604334121210189_usize;
_3 = _5 as isize;
_1 = 13674512775669453971_u64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_13 = _7 << _3;
_10 = 9659_i16 as isize;
_2 = '\u{38c54}';
_14 = (-18855_i16) as f64;
RET = _11;
_1 = 13751318447281317506_u64 + 1425133398595954652_u64;
RET = _11 + _11;
_8 = _5 as u32;
_10 = !RET;
_3 = _8 as isize;
Goto(bb6)
}
bb6 = {
_9 = 10799849192990239295_usize ^ 6_usize;
_7 = !_13;
RET = _5 as isize;
_4 = _10 as u8;
_10 = !_3;
_4 = !120_u8;
_7 = -_13;
_12 = Adt33::Variant0 { fld0: _4,fld1: _2,fld2: 34640_u16,fld3: _8,fld4: _9 };
_9 = 12220_i16 as usize;
RET = !_3;
Goto(bb7)
}
bb7 = {
_13 = _7 << _9;
_6 = 1036401845_i32 >> _8;
_1 = (-128_i8) as u64;
place!(Field::<usize>(Variant(_12, 0), 4)) = !_9;
_11 = _10 * _3;
_10 = _13 as isize;
place!(Field::<u32>(Variant(_12, 0), 3)) = !_8;
_14 = RET as f64;
place!(Field::<usize>(Variant(_12, 0), 4)) = _9;
_9 = 81209019993562909746568016705080392664_i128 as usize;
_2 = Field::<char>(Variant(_12, 0), 1);
_15 = !true;
_16 = _5 - _5;
_10 = RET >> RET;
Goto(bb8)
}
bb8 = {
_4 = !Field::<u8>(Variant(_12, 0), 0);
place!(Field::<u32>(Variant(_12, 0), 3)) = _8 << _9;
_11 = RET;
Goto(bb9)
}
bb9 = {
_14 = Field::<u32>(Variant(_12, 0), 3) as f64;
_2 = Field::<char>(Variant(_12, 0), 1);
_16 = (-32424129292614059379144531577901078958_i128) as u128;
_16 = _5 - _5;
_6 = 7260515_i32 & (-1323799057_i32);
_17 = -_14;
place!(Field::<u8>(Variant(_12, 0), 0)) = _4;
place!(Field::<u16>(Variant(_12, 0), 2)) = _1 as u16;
_6 = !1892942916_i32;
_4 = !Field::<u8>(Variant(_12, 0), 0);
_4 = Field::<u8>(Variant(_12, 0), 0) - Field::<u8>(Variant(_12, 0), 0);
_1 = 8196566885529865720_u64;
place!(Field::<u16>(Variant(_12, 0), 2)) = 49229_u16;
_12 = Adt33::Variant0 { fld0: _4,fld1: _2,fld2: 18563_u16,fld3: _8,fld4: _9 };
place!(Field::<u8>(Variant(_12, 0), 0)) = _15 as u8;
RET = (-118_i8) as isize;
_11 = _10;
_15 = !true;
RET = !_11;
_19 = [4_i8,(-127_i8),13_i8,38_i8,40_i8];
_2 = Field::<char>(Variant(_12, 0), 1);
Call(place!(Field::<usize>(Variant(_12, 0), 4)) = core::intrinsics::transmute(_10), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<u16>(Variant(_12, 0), 2)) = 13059_u16;
match _1 {
8196566885529865720 => bb11,
_ => bb1
}
}
bb11 = {
place!(Field::<u32>(Variant(_12, 0), 3)) = 22981640666680225973137755945115433450_i128 as u32;
_1 = 11927481021656642820_u64;
place!(Field::<u16>(Variant(_12, 0), 2)) = 58660_u16 >> _16;
_19 = [127_i8,60_i8,(-13_i8),(-63_i8),(-57_i8)];
_10 = _11 >> _5;
_3 = !_11;
match _1 {
0 => bb2,
11927481021656642820 => bb13,
_ => bb12
}
}
bb12 = {
_1 = 176_u8 as u64;
_3 = RET;
_5 = (-38789648882782530563577415787881736295_i128) as u128;
_4 = 47_u8;
_7 = true as i64;
RET = -_3;
_5 = 76404778942689606404332141299605473255_u128 & 136398411552932251785838264695344147985_u128;
RET = _3;
RET = _3 + _3;
_10 = _3 ^ RET;
_6 = -423883189_i32;
RET = _3;
_9 = 14770604334121210189_usize;
_3 = _5 as isize;
_1 = 13674512775669453971_u64;
Goto(bb2)
}
bb13 = {
_1 = 4277716171277677426_u64;
place!(Field::<char>(Variant(_12, 0), 1)) = _2;
place!(Field::<u32>(Variant(_12, 0), 3)) = (-20182_i16) as u32;
place!(Field::<usize>(Variant(_12, 0), 4)) = _9 << _11;
_21 = -_11;
_11 = !_10;
_2 = Field::<char>(Variant(_12, 0), 1);
Goto(bb14)
}
bb14 = {
_8 = _15 as u32;
_17 = _14 - _14;
place!(Field::<u32>(Variant(_12, 0), 3)) = !_8;
_14 = _11 as f64;
place!(Field::<char>(Variant(_12, 0), 1)) = _2;
_15 = _13 == _7;
_13 = !_7;
_1 = 18059472034053128157_u64 + 368973542452092019_u64;
_16 = _5;
_7 = _13;
_9 = Field::<usize>(Variant(_12, 0), 4);
_2 = Field::<char>(Variant(_12, 0), 1);
_19 = [(-35_i8),106_i8,105_i8,99_i8,54_i8];
_3 = !_10;
place!(Field::<u32>(Variant(_12, 0), 3)) = _8;
place!(Field::<char>(Variant(_12, 0), 1)) = _2;
_2 = Field::<char>(Variant(_12, 0), 1);
_8 = Field::<u32>(Variant(_12, 0), 3);
RET = _10;
_18 = _16 as isize;
_11 = _21 ^ _21;
_15 = true;
_4 = Field::<u8>(Variant(_12, 0), 0) * Field::<u8>(Variant(_12, 0), 0);
_10 = Field::<char>(Variant(_12, 0), 1) as isize;
_2 = Field::<char>(Variant(_12, 0), 1);
_1 = _2 as u64;
_21 = _3 << _3;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(0_usize, 10_usize, Move(_10), 2_usize, Move(_2), 16_usize, Move(_16), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(0_usize, 8_usize, Move(_8), 19_usize, Move(_19), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: usize) -> u32 {
mir! {
type RET = u32;
let _3: f32;
let _4: &'static (*const u128, [u8; 6]);
let _5: isize;
let _6: u8;
let _7: *mut *mut [i8; 4];
let _8: u128;
let _9: isize;
let _10: i128;
let _11: [i8; 4];
let _12: bool;
let _13: bool;
let _14: i128;
let _15: *mut Adt22;
let _16: &'static &'static (&'static u64,);
let _17: &'static Adt25;
let _18: &'static (u16, char, i8);
let _19: *mut Adt22;
let _20: char;
let _21: i32;
let _22: Adt19;
let _23: u16;
let _24: i64;
let _25: &'static &'static (&'static u64,);
let _26: &'static i8;
let _27: [u64; 4];
let _28: *mut Adt22;
let _29: &'static &'static (&'static u64,);
let _30: &'static *const f32;
let _31: &'static Adt22;
let _32: ([u32; 7],);
let _33: (f64, Adt22);
let _34: &'static Adt22;
let _35: ();
let _36: ();
{
RET = !413748189_u32;
_2 = (-140558626502859286635655544966146452796_i128) as usize;
RET = 91_i8 as u32;
_2 = !6_usize;
_2 = !1_usize;
RET = 9228097064701464683541256743255329523_i128 as u32;
_2 = 7_usize >> RET;
_2 = 5_usize + 16667320170476861232_usize;
_2 = RET as usize;
RET = false as u32;
_2 = !2577201166271780207_usize;
_1 = (-6_isize);
Goto(bb1)
}
bb1 = {
_1 = (-28_isize);
_3 = 29532_u16 as f32;
_1 = (-9223372036854775808_isize);
RET = false as u32;
_1 = 9223372036854775807_isize;
_3 = (-56_i8) as f32;
RET = _3 as u32;
_2 = 1_usize;
_3 = 67_i8 as f32;
_1 = !(-52_isize);
_3 = 53_u8 as f32;
_3 = 4545819370268519896_i64 as f32;
_2 = 12072727671540393332_usize;
_2 = (-80_i8) as usize;
_1 = 9223372036854775807_isize | 43_isize;
_3 = 201354427400482462985848815107263224485_u128 as f32;
_2 = _1 as usize;
_1 = 9223372036854775807_isize >> RET;
_5 = !_1;
RET = 460263837_u32 + 1077194346_u32;
_5 = _1 * _1;
_1 = _5 << RET;
_3 = (-22019_i16) as f32;
_1 = (-31138_i16) as isize;
_3 = _5 as f32;
RET = 1084960225_u32;
Call(_2 = fn2(_1, _5, _3, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 3669001171_u32 - 2197932929_u32;
_9 = _5 + _5;
_8 = 13_u8 as u128;
_2 = 6_usize * 3_usize;
Goto(bb3)
}
bb3 = {
_8 = !181828560447295678322316109890685938634_u128;
_3 = _2 as f32;
_1 = !_9;
_3 = (-30_i8) as f32;
_6 = !85_u8;
RET = '\u{b9981}' as u32;
RET = _8 as u32;
_10 = (-25568_i16) as i128;
_2 = 1568480474433166915_usize;
RET = 1591504491_u32 << _9;
_1 = 1606436194362541147_u64 as isize;
RET = true as u32;
_6 = 51_u8 - 88_u8;
_5 = 27128_i16 as isize;
_2 = !16573255212618317306_usize;
_9 = (-13_i8) as isize;
_8 = (-22165_i16) as u128;
_3 = _9 as f32;
_1 = _5 << _5;
_5 = -_1;
_1 = -_5;
_8 = !55243771799286032968955157987852549300_u128;
_8 = 73158519614208126318722471173412527244_u128;
Goto(bb4)
}
bb4 = {
_8 = 173987366519273069040072609758228492287_u128 + 189768266150399433046909200012584026555_u128;
_11 = [(-83_i8),(-106_i8),(-61_i8),56_i8];
RET = !1799383159_u32;
_2 = 85_i8 as usize;
_12 = !true;
_8 = 30402433541348198074729359300003216590_u128;
_10 = !(-36442173795288666133607833031912828372_i128);
_3 = (-7294_i16) as f32;
_12 = _10 >= _10;
_13 = !_12;
RET = 6514669353492780727_u64 as u32;
_3 = (-13055_i16) as f32;
_14 = _10 & _10;
_5 = -_1;
_3 = 116_i8 as f32;
_5 = !_9;
_12 = RET <= RET;
_3 = RET as f32;
match _8 {
0 => bb5,
30402433541348198074729359300003216590 => bb7,
_ => bb6
}
}
bb5 = {
_8 = !181828560447295678322316109890685938634_u128;
_3 = _2 as f32;
_1 = !_9;
_3 = (-30_i8) as f32;
_6 = !85_u8;
RET = '\u{b9981}' as u32;
RET = _8 as u32;
_10 = (-25568_i16) as i128;
_2 = 1568480474433166915_usize;
RET = 1591504491_u32 << _9;
_1 = 1606436194362541147_u64 as isize;
RET = true as u32;
_6 = 51_u8 - 88_u8;
_5 = 27128_i16 as isize;
_2 = !16573255212618317306_usize;
_9 = (-13_i8) as isize;
_8 = (-22165_i16) as u128;
_3 = _9 as f32;
_1 = _5 << _5;
_5 = -_1;
_1 = -_5;
_8 = !55243771799286032968955157987852549300_u128;
_8 = 73158519614208126318722471173412527244_u128;
Goto(bb4)
}
bb6 = {
_1 = (-28_isize);
_3 = 29532_u16 as f32;
_1 = (-9223372036854775808_isize);
RET = false as u32;
_1 = 9223372036854775807_isize;
_3 = (-56_i8) as f32;
RET = _3 as u32;
_2 = 1_usize;
_3 = 67_i8 as f32;
_1 = !(-52_isize);
_3 = 53_u8 as f32;
_3 = 4545819370268519896_i64 as f32;
_2 = 12072727671540393332_usize;
_2 = (-80_i8) as usize;
_1 = 9223372036854775807_isize | 43_isize;
_3 = 201354427400482462985848815107263224485_u128 as f32;
_2 = _1 as usize;
_1 = 9223372036854775807_isize >> RET;
_5 = !_1;
RET = 460263837_u32 + 1077194346_u32;
_5 = _1 * _1;
_1 = _5 << RET;
_3 = (-22019_i16) as f32;
_1 = (-31138_i16) as isize;
_3 = _5 as f32;
RET = 1084960225_u32;
Call(_2 = fn2(_1, _5, _3, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
RET = !555915255_u32;
_5 = _9 | _1;
_11 = [103_i8,54_i8,57_i8,75_i8];
RET = 866044932_u32;
_8 = 270764569181016104272265061693060221035_u128 + 271918760024588899402914477682840431983_u128;
_12 = _3 == _3;
_5 = _1 ^ _1;
_14 = (-385858332_i32) as i128;
RET = 644443746_u32;
_11 = [109_i8,(-55_i8),(-104_i8),89_i8];
_8 = !333705035576371591244994941804179156555_u128;
_8 = 320008386546686429759790360772828491254_u128;
_21 = (-1867359034_i32) - 1113223231_i32;
_14 = 7_i8 as i128;
_22.fld3 = RET as i8;
_22 = Adt19 { fld0: _12,fld1: '\u{5fd05}',fld2: _1,fld3: 19_i8,fld4: _14 };
_22.fld1 = '\u{41c8}';
_3 = RET as f32;
RET = !3130305082_u32;
_23 = 61802_u16 * 25208_u16;
_20 = _22.fld1;
_3 = _21 as f32;
Goto(bb8)
}
bb8 = {
_13 = _22.fld0;
_9 = 5073576057374606713_i64 as isize;
_22.fld0 = !_12;
_21 = RET as i32;
match _22.fld3 {
0 => bb7,
19 => bb9,
_ => bb4
}
}
bb9 = {
_24 = -4090533098602056417_i64;
_22 = Adt19 { fld0: _13,fld1: _20,fld2: _5,fld3: (-112_i8),fld4: _14 };
match _8 {
0 => bb2,
320008386546686429759790360772828491254 => bb11,
_ => bb10
}
}
bb10 = {
_1 = (-28_isize);
_3 = 29532_u16 as f32;
_1 = (-9223372036854775808_isize);
RET = false as u32;
_1 = 9223372036854775807_isize;
_3 = (-56_i8) as f32;
RET = _3 as u32;
_2 = 1_usize;
_3 = 67_i8 as f32;
_1 = !(-52_isize);
_3 = 53_u8 as f32;
_3 = 4545819370268519896_i64 as f32;
_2 = 12072727671540393332_usize;
_2 = (-80_i8) as usize;
_1 = 9223372036854775807_isize | 43_isize;
_3 = 201354427400482462985848815107263224485_u128 as f32;
_2 = _1 as usize;
_1 = 9223372036854775807_isize >> RET;
_5 = !_1;
RET = 460263837_u32 + 1077194346_u32;
_5 = _1 * _1;
_1 = _5 << RET;
_3 = (-22019_i16) as f32;
_1 = (-31138_i16) as isize;
_3 = _5 as f32;
RET = 1084960225_u32;
Call(_2 = fn2(_1, _5, _3, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_6 = 42_u8 + 41_u8;
_27 = [14200061034835291368_u64,12228007621232775475_u64,4525552874256811255_u64,10912658983059027048_u64];
_5 = !_1;
RET = 2802153641_u32;
_22.fld3 = _8 as i8;
_27 = [8474496640541173607_u64,6721162019978788628_u64,17435999305065006125_u64,6774069486790389825_u64];
_2 = 5_usize | 3_usize;
_26 = &_22.fld3;
_11 = [(*_26),(*_26),(*_26),_22.fld3];
_1 = _22.fld2;
_22.fld1 = _20;
_9 = _2 as isize;
_10 = RET as i128;
match _8 {
0 => bb4,
1 => bb6,
2 => bb9,
3 => bb12,
4 => bb13,
5 => bb14,
320008386546686429759790360772828491254 => bb16,
_ => bb15
}
}
bb12 = {
_1 = (-28_isize);
_3 = 29532_u16 as f32;
_1 = (-9223372036854775808_isize);
RET = false as u32;
_1 = 9223372036854775807_isize;
_3 = (-56_i8) as f32;
RET = _3 as u32;
_2 = 1_usize;
_3 = 67_i8 as f32;
_1 = !(-52_isize);
_3 = 53_u8 as f32;
_3 = 4545819370268519896_i64 as f32;
_2 = 12072727671540393332_usize;
_2 = (-80_i8) as usize;
_1 = 9223372036854775807_isize | 43_isize;
_3 = 201354427400482462985848815107263224485_u128 as f32;
_2 = _1 as usize;
_1 = 9223372036854775807_isize >> RET;
_5 = !_1;
RET = 460263837_u32 + 1077194346_u32;
_5 = _1 * _1;
_1 = _5 << RET;
_3 = (-22019_i16) as f32;
_1 = (-31138_i16) as isize;
_3 = _5 as f32;
RET = 1084960225_u32;
Call(_2 = fn2(_1, _5, _3, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_24 = -4090533098602056417_i64;
_22 = Adt19 { fld0: _13,fld1: _20,fld2: _5,fld3: (-112_i8),fld4: _14 };
match _8 {
0 => bb2,
320008386546686429759790360772828491254 => bb11,
_ => bb10
}
}
bb14 = {
_8 = !181828560447295678322316109890685938634_u128;
_3 = _2 as f32;
_1 = !_9;
_3 = (-30_i8) as f32;
_6 = !85_u8;
RET = '\u{b9981}' as u32;
RET = _8 as u32;
_10 = (-25568_i16) as i128;
_2 = 1568480474433166915_usize;
RET = 1591504491_u32 << _9;
_1 = 1606436194362541147_u64 as isize;
RET = true as u32;
_6 = 51_u8 - 88_u8;
_5 = 27128_i16 as isize;
_2 = !16573255212618317306_usize;
_9 = (-13_i8) as isize;
_8 = (-22165_i16) as u128;
_3 = _9 as f32;
_1 = _5 << _5;
_5 = -_1;
_1 = -_5;
_8 = !55243771799286032968955157987852549300_u128;
_8 = 73158519614208126318722471173412527244_u128;
Goto(bb4)
}
bb15 = {
RET = !555915255_u32;
_5 = _9 | _1;
_11 = [103_i8,54_i8,57_i8,75_i8];
RET = 866044932_u32;
_8 = 270764569181016104272265061693060221035_u128 + 271918760024588899402914477682840431983_u128;
_12 = _3 == _3;
_5 = _1 ^ _1;
_14 = (-385858332_i32) as i128;
RET = 644443746_u32;
_11 = [109_i8,(-55_i8),(-104_i8),89_i8];
_8 = !333705035576371591244994941804179156555_u128;
_8 = 320008386546686429759790360772828491254_u128;
_21 = (-1867359034_i32) - 1113223231_i32;
_14 = 7_i8 as i128;
_22.fld3 = RET as i8;
_22 = Adt19 { fld0: _12,fld1: '\u{5fd05}',fld2: _1,fld3: 19_i8,fld4: _14 };
_22.fld1 = '\u{41c8}';
_3 = RET as f32;
RET = !3130305082_u32;
_23 = 61802_u16 * 25208_u16;
_20 = _22.fld1;
_3 = _21 as f32;
Goto(bb8)
}
bb16 = {
_5 = _22.fld2;
_14 = _22.fld4;
_6 = !149_u8;
_22.fld2 = _9 * _9;
_2 = _6 as usize;
_8 = 138884283368186737384337026689925925874_u128;
_22.fld4 = (*_26) as i128;
_22.fld4 = _10 * _10;
_15 = core::ptr::addr_of_mut!(_33.1);
_32.0 = [RET,RET,RET,RET,RET,RET,RET];
_33.0 = _23 as f64;
_8 = !313181748741607263065631081705355267228_u128;
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(1_usize, 11_usize, Move(_11), 13_usize, Move(_13), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(1_usize, 32_usize, Move(_32), 2_usize, Move(_2), 6_usize, Move(_6), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: f32,mut _4: f32,mut _5: u32) -> usize {
mir! {
type RET = usize;
let _6: bool;
let _7: ([u32; 7],);
let _8: i8;
let _9: [u32; 5];
let _10: isize;
let _11: i64;
let _12: isize;
let _13: f32;
let _14: bool;
let _15: bool;
let _16: char;
let _17: &'static &'static u128;
let _18: *mut u128;
let _19: Adt73;
let _20: u8;
let _21: *mut Adt22;
let _22: &'static &'static u128;
let _23: isize;
let _24: char;
let _25: f64;
let _26: u8;
let _27: &'static Adt22;
let _28: isize;
let _29: ();
let _30: ();
{
_1 = 144_u8 as isize;
_1 = -_2;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1084960225 => bb7,
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
_5 = !3490043436_u32;
_5 = 3601763704_u32 + 2914473553_u32;
_4 = _3 + _3;
Call(RET = fn3(_2, _2, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = !6_usize;
RET = 6_usize & 4097526177014242976_usize;
_4 = 12260735951430231034_u64 as f32;
_5 = 1007600173_u32;
_1 = !_2;
_5 = !2112540781_u32;
_1 = -_2;
_3 = _4;
_5 = 2829748019_u32 + 1276911407_u32;
_8 = !(-77_i8);
_4 = _2 as f32;
_2 = _1 | _1;
_2 = 1644449008_i32 as isize;
_6 = _5 < _5;
_10 = _1 ^ _1;
Goto(bb9)
}
bb9 = {
_7.0 = [_5,_5,_5,_5,_5,_5,_5];
RET = 10989006540322076688_usize >> _1;
_2 = _10;
_2 = _1 + _10;
RET = _4 as usize;
RET = !536264463549675820_usize;
_2 = _10;
RET = _10 as usize;
Call(_2 = core::intrinsics::bswap(_10), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6 = !false;
_11 = (-8595614154596343743_i64) ^ 1104252617161180847_i64;
_9 = [_5,_5,_5,_5,_5];
_7.0 = [_5,_5,_5,_5,_5,_5,_5];
_9 = [_5,_5,_5,_5,_5];
_13 = _4 + _3;
_3 = _8 as f32;
_5 = 3624293198_u32;
RET = _10 as usize;
_1 = _2;
_5 = !1766943593_u32;
_5 = 14128144405625954747_u64 as u32;
_7.0 = [_5,_5,_5,_5,_5,_5,_5];
_12 = 6490_u16 as isize;
Goto(bb11)
}
bb11 = {
_12 = !_10;
_10 = _1;
_7.0 = [_5,_5,_5,_5,_5,_5,_5];
_14 = _6;
_5 = 2389764371_u32 & 3924453487_u32;
_11 = -(-8607135757607411523_i64);
_1 = 639837581_i32 as isize;
_10 = 16533400843861862108_u64 as isize;
_1 = _12 - _2;
_10 = !_2;
_8 = (-86_i8);
_8 = '\u{74a16}' as i8;
_12 = _10 & _1;
_2 = !_1;
RET = 0_usize ^ 7_usize;
_16 = '\u{cc767}';
_15 = !_14;
_13 = _4;
_3 = _5 as f32;
RET = 0_usize;
_6 = _2 < _12;
_3 = _4 - _13;
Goto(bb12)
}
bb12 = {
_9[RET] = _7.0[RET] >> _5;
_13 = _3;
_8 = 72_i8 << _1;
RET = 8422572139625304902_usize | 2_usize;
RET = 3532194397923196780_usize & 217593299786547744_usize;
_5 = 1345366030_u32;
_2 = !_1;
_20 = _11 as u8;
_9 = [_5,_5,_5,_5,_5];
_7.0 = [_5,_5,_5,_5,_5,_5,_5];
RET = !11728415486558264409_usize;
_6 = _15;
_2 = 237858411364378801052783492397960547843_u128 as isize;
RET = _6 as usize;
_9 = [_5,_5,_5,_5,_5];
_1 = _11 as isize;
_6 = !_15;
_10 = _5 as isize;
_10 = _2 - _12;
Goto(bb13)
}
bb13 = {
_10 = _12;
_10 = !_12;
_12 = _10;
_4 = RET as f32;
_15 = _10 == _12;
_2 = _12 ^ _10;
RET = !7889228285932483137_usize;
_15 = !_6;
_14 = !_15;
_10 = _2;
RET = _14 as usize;
RET = 2_usize & 7106881377606029231_usize;
_10 = !_12;
_16 = '\u{2a565}';
_5 = 4082300406_u32;
_12 = _2;
_8 = (-90_i8) * (-93_i8);
_15 = !_14;
_23 = -_2;
_25 = _20 as f64;
_20 = !21_u8;
Goto(bb14)
}
bb14 = {
RET = _20 as usize;
_11 = -(-1396019982113181174_i64);
_12 = 250460388086869104667185628313993820587_u128 as isize;
_3 = 21507_i16 as f32;
_20 = !147_u8;
_23 = _10;
_16 = '\u{d5caf}';
_15 = _2 <= _10;
_6 = _15 | _15;
_26 = !_20;
_25 = RET as f64;
_3 = _13;
_24 = _16;
_2 = -_23;
_16 = _24;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 8_usize, Move(_8), 12_usize, Move(_12), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 16_usize, Move(_16), 7_usize, Move(_7), 20_usize, Move(_20), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: isize,mut _3: f32) -> usize {
mir! {
type RET = usize;
let _4: isize;
let _5: i128;
let _6: *mut [isize; 1];
let _7: &'static &'static i8;
let _8: u128;
let _9: bool;
let _10: *mut [i8; 4];
let _11: (*const f32,);
let _12: f32;
let _13: [isize; 1];
let _14: isize;
let _15: char;
let _16: i16;
let _17: isize;
let _18: i64;
let _19: i128;
let _20: *const u128;
let _21: [i128; 3];
let _22: Adt77;
let _23: [u32; 5];
let _24: u8;
let _25: &'static f64;
let _26: isize;
let _27: ();
let _28: ();
{
_2 = -_1;
_3 = 269186992395960858128682953389215502335_u128 as f32;
_4 = (-3510_i16) as isize;
_4 = !_2;
_2 = _1 * _1;
_5 = (-97084780598080034705222745094706661930_i128);
_4 = _2;
_4 = !_1;
RET = 4026949927400696794_usize - 16427609002207793782_usize;
_4 = -_2;
_2 = 198_u8 as isize;
RET = (-8013051786674980610_i64) as usize;
RET = 5921489516726786564_u64 as usize;
_3 = _4 as f32;
Goto(bb1)
}
bb1 = {
_2 = _4;
Call(RET = fn4(_1, _4, _4, _3, _2, _3, _2, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 6_usize;
_4 = -_1;
_2 = -_4;
RET = 0_usize;
RET = !5_usize;
_5 = 3093044948_u32 as i128;
RET = 243800090433065825574765955018583430246_u128 as usize;
_2 = _4 * _1;
_5 = (-25378786718670668241029238701351104661_i128) >> _2;
_5 = 70_i8 as i128;
_4 = 158_u8 as isize;
Goto(bb3)
}
bb3 = {
RET = 14689166725403754539_usize;
_2 = -_4;
_1 = _4 + _2;
RET = 0_usize >> _2;
Goto(bb4)
}
bb4 = {
_2 = _3 as isize;
_3 = (-1380918865_i32) as f32;
_1 = _2;
_8 = !305169003224448266279925212573127426136_u128;
_3 = _2 as f32;
_2 = _4 & _1;
_9 = _3 < _3;
_9 = false;
_2 = _1 & _1;
_12 = -_3;
_9 = true & true;
_11.0 = core::ptr::addr_of!(_12);
_3 = _5 as f32;
_6 = core::ptr::addr_of_mut!(_13);
_14 = 2086074133_u32 as isize;
Goto(bb5)
}
bb5 = {
RET = !6_usize;
_14 = _4 + _2;
_13 = [_1];
(*_6) = [_14];
_4 = -_1;
_1 = _2;
_2 = _1;
RET = !508359820560934354_usize;
_1 = _14 << _14;
_14 = _1;
(*_6) = [_14];
_2 = -_14;
_15 = '\u{5bfc3}';
_15 = '\u{f8abc}';
RET = !11198594005125242961_usize;
_4 = 1972372421_i32 as isize;
_1 = -_2;
_2 = _14;
_14 = _1;
_16 = -(-20577_i16);
Goto(bb6)
}
bb6 = {
RET = 16542854663948108433_usize >> _16;
_2 = _1;
_9 = !false;
_17 = _2 | _14;
RET = 6785035959247651258_usize & 3_usize;
_11.0 = core::ptr::addr_of!(_3);
Goto(bb7)
}
bb7 = {
(*_6) = [_17];
_9 = false;
_2 = _17;
(*_6) = [_17];
_9 = !false;
_5 = (-85465398572096911521855301327762147765_i128);
_14 = _16 as isize;
_17 = -_2;
_14 = _17 >> _17;
_15 = '\u{d42e7}';
_4 = -_14;
_12 = 3341427739322340905_i64 as f32;
_15 = '\u{f7652}';
_8 = 131714832424027427469988696989240974824_u128 | 23002096952707699052976047669254688225_u128;
_16 = 11250_i16 >> _14;
_14 = _4 - _2;
_6 = core::ptr::addr_of_mut!((*_6));
_18 = (-2597204962383394105_i64);
_1 = !_17;
_1 = _17 >> _4;
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb8)
}
bb8 = {
_8 = !170872729292682869455302680365985427386_u128;
_19 = !_5;
_21 = [_19,_5,_5];
_6 = core::ptr::addr_of_mut!(_13);
_23 = [951902271_u32,1241587256_u32,1610102038_u32,4232182043_u32,1596122401_u32];
_1 = _4;
RET = 2304419862_u32 as usize;
_9 = _2 == _1;
_5 = _19;
_20 = core::ptr::addr_of!(_8);
_23 = [2959213764_u32,3446823163_u32,3721185525_u32,439389193_u32,3246127188_u32];
_15 = '\u{da4c3}';
_1 = 248228221_i32 as isize;
_24 = 138_u8;
_11.0 = core::ptr::addr_of!(_12);
_19 = 11319340553496419406_u64 as i128;
Goto(bb9)
}
bb9 = {
_11.0 = core::ptr::addr_of!(_3);
_17 = RET as isize;
(*_6) = [_2];
_11.0 = core::ptr::addr_of!(_12);
_8 = 11_i8 as u128;
_21 = [_5,_19,_5];
_4 = _14 + _14;
_23 = [3171293200_u32,193264666_u32,1098672592_u32,292346741_u32,1804663640_u32];
_1 = _2 * _2;
_14 = _4 >> _1;
_23 = [1672908920_u32,4134829718_u32,2287674951_u32,1042413741_u32,2990079719_u32];
Goto(bb10)
}
bb10 = {
_6 = core::ptr::addr_of_mut!(_13);
_14 = -_1;
RET = 6047738410819275935_usize + 6_usize;
_26 = -_4;
match _24 {
138 => bb12,
_ => bb11
}
}
bb11 = {
_8 = !170872729292682869455302680365985427386_u128;
_19 = !_5;
_21 = [_19,_5,_5];
_6 = core::ptr::addr_of_mut!(_13);
_23 = [951902271_u32,1241587256_u32,1610102038_u32,4232182043_u32,1596122401_u32];
_1 = _4;
RET = 2304419862_u32 as usize;
_9 = _2 == _1;
_5 = _19;
_20 = core::ptr::addr_of!(_8);
_23 = [2959213764_u32,3446823163_u32,3721185525_u32,439389193_u32,3246127188_u32];
_15 = '\u{da4c3}';
_1 = 248228221_i32 as isize;
_24 = 138_u8;
_11.0 = core::ptr::addr_of!(_12);
_19 = 11319340553496419406_u64 as i128;
Goto(bb9)
}
bb12 = {
_13 = [_26];
_13 = [_26];
_14 = _9 as isize;
RET = _9 as usize;
_23 = [2113252207_u32,2402913310_u32,2887883878_u32,3722191397_u32,2087417939_u32];
_19 = _5;
_13 = [_26];
Goto(bb13)
}
bb13 = {
Call(_27 = dump_var(3_usize, 9_usize, Move(_9), 24_usize, Move(_24), 8_usize, Move(_8), 15_usize, Move(_15)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_27 = dump_var(3_usize, 21_usize, Move(_21), 23_usize, Move(_23), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: f32,mut _5: isize,mut _6: f32,mut _7: isize,mut _8: isize) -> usize {
mir! {
type RET = usize;
let _9: bool;
let _10: char;
let _11: *const [bool; 3];
let _12: isize;
let _13: Adt52;
let _14: Adt33;
let _15: (*mut u128, Adt19);
let _16: &'static (&'static u64,);
let _17: isize;
let _18: f64;
let _19: f32;
let _20: char;
let _21: u8;
let _22: i16;
let _23: i32;
let _24: *mut *mut [i8; 4];
let _25: (char, i64, Adt19, [u8; 6]);
let _26: bool;
let _27: u128;
let _28: u128;
let _29: i16;
let _30: [i8; 4];
let _31: *mut [i8; 4];
let _32: Adt73;
let _33: ();
let _34: ();
{
RET = !3_usize;
_6 = _4 * _4;
_1 = _2 ^ _3;
_10 = '\u{1bb24}';
_6 = -_4;
_9 = true;
_1 = _7 >> _7;
_8 = _1 + _3;
_2 = _8 + _3;
_13.fld0 = [(-3469917411538989043_i64),(-6895740110695768904_i64),5910121283403774696_i64,(-4413949019371005100_i64),1454111080646229265_i64,(-5234799654929208969_i64),9169767025396722611_i64,(-1479666874514461431_i64)];
_8 = _9 as isize;
_12 = !_2;
_3 = _10 as isize;
_10 = '\u{80baa}';
_12 = _2;
_13.fld0 = [(-8865181565880032787_i64),1961383622030503166_i64,(-4886862937334497014_i64),(-3925734347188110821_i64),8045405442525268416_i64,(-2432555584287041914_i64),(-3623899451282362288_i64),(-8007391259855404050_i64)];
_7 = _4 as isize;
_6 = (-24880_i16) as f32;
Call(_10 = fn5(_12, _2, _12, _12, _12, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 7_usize & 2_usize;
_2 = -_1;
_9 = !true;
_2 = _12;
_15.1.fld1 = _10;
_10 = _15.1.fld1;
_15.1.fld2 = (-12744_i16) as isize;
_1 = !_12;
_7 = _12;
_17 = _1;
_15.1.fld4 = 12192259019596780295_u64 as i128;
_18 = (-4473825611964130544_i64) as f64;
_6 = _4;
_15.1 = Adt19 { fld0: _9,fld1: _10,fld2: _7,fld3: (-33_i8),fld4: (-114415490355053375180069411136033228151_i128) };
_8 = 32614_i16 as isize;
_2 = _9 as isize;
RET = 17688521481520063012_usize;
_15.1.fld0 = _9 ^ _9;
RET = _10 as usize;
RET = 0_usize;
Goto(bb2)
}
bb2 = {
_20 = _15.1.fld1;
_8 = -_12;
_2 = _13.fld0[RET] as isize;
_15.1.fld2 = 37_u8 as isize;
_10 = _15.1.fld1;
_15.1.fld3 = -33_i8;
_13.fld0[RET] = 2167635763992999033_i64 - 3374907571719895137_i64;
_8 = -_7;
_15.1.fld2 = 38310254546111001019699016458093772448_u128 as isize;
_20 = _15.1.fld1;
_23 = 1202769560_i32;
_15.1.fld3 = (-61_i8) - 46_i8;
_25.2.fld4 = -_15.1.fld4;
_25.2.fld3 = -_15.1.fld3;
_25.1 = _13.fld0[RET];
_22 = 191_u8 as i16;
_25.3[RET] = _22 as u8;
_25.2 = Move(_15.1);
_17 = _7 ^ _8;
RET = 4226884243_u32 as usize;
_1 = _8;
_15.1 = Adt19 { fld0: _9,fld1: _10,fld2: _2,fld3: _25.2.fld3,fld4: _25.2.fld4 };
_25.3 = [242_u8,95_u8,102_u8,6_u8,145_u8,95_u8];
_25.0 = _25.2.fld1;
match _15.1.fld4 {
0 => bb1,
225866876565885088283305196295734983305 => bb4,
_ => bb3
}
}
bb3 = {
RET = 7_usize & 2_usize;
_2 = -_1;
_9 = !true;
_2 = _12;
_15.1.fld1 = _10;
_10 = _15.1.fld1;
_15.1.fld2 = (-12744_i16) as isize;
_1 = !_12;
_7 = _12;
_17 = _1;
_15.1.fld4 = 12192259019596780295_u64 as i128;
_18 = (-4473825611964130544_i64) as f64;
_6 = _4;
_15.1 = Adt19 { fld0: _9,fld1: _10,fld2: _7,fld3: (-33_i8),fld4: (-114415490355053375180069411136033228151_i128) };
_8 = 32614_i16 as isize;
_2 = _9 as isize;
RET = 17688521481520063012_usize;
_15.1.fld0 = _9 ^ _9;
RET = _10 as usize;
RET = 0_usize;
Goto(bb2)
}
bb4 = {
_25.2 = Adt19 { fld0: _15.1.fld0,fld1: _15.1.fld1,fld2: _8,fld3: _15.1.fld3,fld4: _15.1.fld4 };
match _23 {
0 => bb1,
1 => bb2,
1202769560 => bb5,
_ => bb3
}
}
bb5 = {
_15.1.fld3 = _18 as i8;
_21 = 207_u8 >> _15.1.fld4;
_26 = _7 == _8;
Call(_18 = fn19(Move(_15.1), _21, _8, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
match _25.2.fld4 {
225866876565885088283305196295734983305 => bb7,
_ => bb2
}
}
bb7 = {
_15.1.fld3 = -_25.2.fld3;
_13.fld0 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_1 = -_7;
_19 = _6;
_25.2.fld2 = _17;
_23 = (-1396777138_i32);
_27 = _17 as u128;
_15.1.fld4 = -_25.2.fld4;
_25.2.fld0 = !_26;
_1 = _25.2.fld3 as isize;
_28 = _22 as u128;
RET = 16571685613910976578_usize;
_17 = _7 & _7;
RET = 3_usize;
_29 = _22;
_15.1 = Adt19 { fld0: _26,fld1: _25.2.fld1,fld2: _12,fld3: _25.2.fld3,fld4: _25.2.fld4 };
_4 = -_6;
_25.2.fld3 = _15.1.fld3 - _15.1.fld3;
_9 = !_26;
_25.3[RET] = _21 + _21;
match _25.2.fld4 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
225866876565885088283305196295734983305 => bb14,
_ => bb13
}
}
bb8 = {
match _25.2.fld4 {
225866876565885088283305196295734983305 => bb7,
_ => bb2
}
}
bb9 = {
_15.1.fld3 = _18 as i8;
_21 = 207_u8 >> _15.1.fld4;
_26 = _7 == _8;
Call(_18 = fn19(Move(_15.1), _21, _8, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_25.2 = Adt19 { fld0: _15.1.fld0,fld1: _15.1.fld1,fld2: _8,fld3: _15.1.fld3,fld4: _15.1.fld4 };
match _23 {
0 => bb1,
1 => bb2,
1202769560 => bb5,
_ => bb3
}
}
bb11 = {
RET = 7_usize & 2_usize;
_2 = -_1;
_9 = !true;
_2 = _12;
_15.1.fld1 = _10;
_10 = _15.1.fld1;
_15.1.fld2 = (-12744_i16) as isize;
_1 = !_12;
_7 = _12;
_17 = _1;
_15.1.fld4 = 12192259019596780295_u64 as i128;
_18 = (-4473825611964130544_i64) as f64;
_6 = _4;
_15.1 = Adt19 { fld0: _9,fld1: _10,fld2: _7,fld3: (-33_i8),fld4: (-114415490355053375180069411136033228151_i128) };
_8 = 32614_i16 as isize;
_2 = _9 as isize;
RET = 17688521481520063012_usize;
_15.1.fld0 = _9 ^ _9;
RET = _10 as usize;
RET = 0_usize;
Goto(bb2)
}
bb12 = {
_20 = _15.1.fld1;
_8 = -_12;
_2 = _13.fld0[RET] as isize;
_15.1.fld2 = 37_u8 as isize;
_10 = _15.1.fld1;
_15.1.fld3 = -33_i8;
_13.fld0[RET] = 2167635763992999033_i64 - 3374907571719895137_i64;
_8 = -_7;
_15.1.fld2 = 38310254546111001019699016458093772448_u128 as isize;
_20 = _15.1.fld1;
_23 = 1202769560_i32;
_15.1.fld3 = (-61_i8) - 46_i8;
_25.2.fld4 = -_15.1.fld4;
_25.2.fld3 = -_15.1.fld3;
_25.1 = _13.fld0[RET];
_22 = 191_u8 as i16;
_25.3[RET] = _22 as u8;
_25.2 = Move(_15.1);
_17 = _7 ^ _8;
RET = 4226884243_u32 as usize;
_1 = _8;
_15.1 = Adt19 { fld0: _9,fld1: _10,fld2: _2,fld3: _25.2.fld3,fld4: _25.2.fld4 };
_25.3 = [242_u8,95_u8,102_u8,6_u8,145_u8,95_u8];
_25.0 = _25.2.fld1;
match _15.1.fld4 {
0 => bb1,
225866876565885088283305196295734983305 => bb4,
_ => bb3
}
}
bb13 = {
RET = 7_usize & 2_usize;
_2 = -_1;
_9 = !true;
_2 = _12;
_15.1.fld1 = _10;
_10 = _15.1.fld1;
_15.1.fld2 = (-12744_i16) as isize;
_1 = !_12;
_7 = _12;
_17 = _1;
_15.1.fld4 = 12192259019596780295_u64 as i128;
_18 = (-4473825611964130544_i64) as f64;
_6 = _4;
_15.1 = Adt19 { fld0: _9,fld1: _10,fld2: _7,fld3: (-33_i8),fld4: (-114415490355053375180069411136033228151_i128) };
_8 = 32614_i16 as isize;
_2 = _9 as isize;
RET = 17688521481520063012_usize;
_15.1.fld0 = _9 ^ _9;
RET = _10 as usize;
RET = 0_usize;
Goto(bb2)
}
bb14 = {
_15.1.fld4 = _25.2.fld4;
_30[RET] = !_25.2.fld3;
_27 = _25.2.fld4 as u128;
_24 = core::ptr::addr_of_mut!(_31);
_25.0 = _15.1.fld1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(4_usize, 9_usize, Move(_9), 2_usize, Move(_2), 20_usize, Move(_20), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(4_usize, 1_usize, Move(_1), 5_usize, Move(_5), 8_usize, Move(_8), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(4_usize, 23_usize, Move(_23), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> char {
mir! {
type RET = char;
let _7: isize;
let _8: char;
let _9: *const [bool; 3];
let _10: Adt33;
let _11: &'static &'static (&'static u64,);
let _12: [i16; 8];
let _13: isize;
let _14: i128;
let _15: ([i16; 8],);
let _16: bool;
let _17: i8;
let _18: (char, i64, Adt19, [u8; 6]);
let _19: *mut [i8; 4];
let _20: *mut [isize; 1];
let _21: [isize; 4];
let _22: bool;
let _23: &'static *const f32;
let _24: Adt73;
let _25: (&'static u64,);
let _26: u16;
let _27: u128;
let _28: [u32; 5];
let _29: ([i16; 8],);
let _30: u64;
let _31: *mut u128;
let _32: isize;
let _33: &'static *const f32;
let _34: *mut [isize; 1];
let _35: u64;
let _36: [u64; 4];
let _37: ();
let _38: ();
{
RET = '\u{4ba49}';
RET = '\u{87da8}';
_1 = -_2;
_1 = _2;
RET = '\u{95fa0}';
_4 = !_3;
_7 = _4;
RET = '\u{2e48a}';
_5 = RET as isize;
_6 = -_3;
_3 = _4 + _1;
_2 = (-1852628478139435476_i64) as isize;
_7 = -_1;
_6 = 3886423818168200146_u64 as isize;
_5 = 468581639_i32 as isize;
_8 = RET;
_5 = _3;
RET = _8;
_5 = -_3;
RET = _8;
RET = _8;
_5 = -_1;
_8 = RET;
RET = _8;
_1 = !_5;
Goto(bb1)
}
bb1 = {
_7 = true as isize;
_6 = _5 >> _1;
_1 = -_3;
_2 = (-16813_i16) as isize;
_2 = _1 + _1;
_12 = [10210_i16,(-23117_i16),7028_i16,(-6409_i16),14289_i16,(-28389_i16),(-32235_i16),271_i16];
_8 = RET;
_4 = _5;
_8 = RET;
_5 = 1435763873_i32 as isize;
_14 = !(-20816931046137888777696856802751491713_i128);
_16 = !true;
_13 = -_3;
Goto(bb2)
}
bb2 = {
RET = _8;
_18.2.fld3 = (-110_i8) >> _2;
_18.2.fld0 = _13 <= _3;
_18.2.fld0 = _16;
_18.2.fld2 = _1 << _13;
_15 = (_12,);
_18.2.fld1 = RET;
_18.2.fld2 = _2 - _1;
_14 = 238815993926952072132018988160289123258_u128 as i128;
_18.0 = _8;
_12 = _15.0;
_18.2.fld4 = RET as i128;
_15.0 = [10013_i16,(-12523_i16),16219_i16,2660_i16,(-21996_i16),7632_i16,25817_i16,18044_i16];
_18.2.fld1 = RET;
_17 = _18.2.fld3;
_18.2.fld4 = _14 * _14;
_18.1 = -(-5656235344657138577_i64);
_3 = 16937581543745871057_usize as isize;
_13 = _6;
_2 = -_18.2.fld2;
_12 = [31734_i16,11572_i16,1200_i16,14620_i16,2677_i16,(-10619_i16),(-29582_i16),30743_i16];
Goto(bb3)
}
bb3 = {
_18.3 = [136_u8,126_u8,157_u8,202_u8,132_u8,101_u8];
Goto(bb4)
}
bb4 = {
_12 = _15.0;
_4 = _18.2.fld2 | _6;
_18.3 = [11_u8,1_u8,166_u8,74_u8,253_u8,23_u8];
_10 = Adt33::Variant0 { fld0: 21_u8,fld1: _8,fld2: 24243_u16,fld3: 154818556_u32,fld4: 5_usize };
_18.2.fld4 = _14 >> _18.2.fld2;
place!(Field::<u32>(Variant(_10, 0), 3)) = 1442945722_u32 << _18.2.fld3;
place!(Field::<u32>(Variant(_10, 0), 3)) = 2838202061_u32;
_18.2 = Adt19 { fld0: _16,fld1: Field::<char>(Variant(_10, 0), 1),fld2: _6,fld3: _17,fld4: _14 };
_18.3 = [25_u8,103_u8,72_u8,8_u8,40_u8,101_u8];
place!(Field::<u8>(Variant(_10, 0), 0)) = !89_u8;
place!(Field::<u8>(Variant(_10, 0), 0)) = 102_u8;
_18.0 = _8;
place!(Field::<usize>(Variant(_10, 0), 4)) = _18.2.fld2 as usize;
_22 = _18.2.fld3 >= _18.2.fld3;
match Field::<u8>(Variant(_10, 0), 0) {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
102 => bb12,
_ => bb11
}
}
bb5 = {
_18.3 = [136_u8,126_u8,157_u8,202_u8,132_u8,101_u8];
Goto(bb4)
}
bb6 = {
RET = _8;
_18.2.fld3 = (-110_i8) >> _2;
_18.2.fld0 = _13 <= _3;
_18.2.fld0 = _16;
_18.2.fld2 = _1 << _13;
_15 = (_12,);
_18.2.fld1 = RET;
_18.2.fld2 = _2 - _1;
_14 = 238815993926952072132018988160289123258_u128 as i128;
_18.0 = _8;
_12 = _15.0;
_18.2.fld4 = RET as i128;
_15.0 = [10013_i16,(-12523_i16),16219_i16,2660_i16,(-21996_i16),7632_i16,25817_i16,18044_i16];
_18.2.fld1 = RET;
_17 = _18.2.fld3;
_18.2.fld4 = _14 * _14;
_18.1 = -(-5656235344657138577_i64);
_3 = 16937581543745871057_usize as isize;
_13 = _6;
_2 = -_18.2.fld2;
_12 = [31734_i16,11572_i16,1200_i16,14620_i16,2677_i16,(-10619_i16),(-29582_i16),30743_i16];
Goto(bb3)
}
bb7 = {
_7 = true as isize;
_6 = _5 >> _1;
_1 = -_3;
_2 = (-16813_i16) as isize;
_2 = _1 + _1;
_12 = [10210_i16,(-23117_i16),7028_i16,(-6409_i16),14289_i16,(-28389_i16),(-32235_i16),271_i16];
_8 = RET;
_4 = _5;
_8 = RET;
_5 = 1435763873_i32 as isize;
_14 = !(-20816931046137888777696856802751491713_i128);
_16 = !true;
_13 = -_3;
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
Return()
}
bb12 = {
_18.2.fld3 = -_17;
_15 = (_12,);
_5 = -_4;
_18.2 = Adt19 { fld0: _22,fld1: _8,fld2: _13,fld3: _17,fld4: _14 };
_3 = _1;
place!(Field::<char>(Variant(_10, 0), 1)) = _8;
_6 = -_2;
_3 = _18.1 as isize;
_7 = _1;
_18.2.fld4 = _14;
_6 = _5;
_22 = _17 < _17;
place!(Field::<char>(Variant(_10, 0), 1)) = _18.2.fld1;
_18.1 = _18.2.fld2 as i64;
_14 = 61951_u16 as i128;
_12 = [(-8356_i16),29105_i16,(-22759_i16),(-4211_i16),116_i16,23580_i16,(-10284_i16),5597_i16];
_27 = _22 as u128;
_4 = !_6;
_18.2 = Adt19 { fld0: _22,fld1: _8,fld2: _1,fld3: _17,fld4: _14 };
_29.0 = _12;
_5 = -_7;
_29 = _15;
Call(_26 = fn6(_27, _1, _22, _2, _17, _22, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18.2.fld0 = !_22;
_5 = _6 + _2;
_22 = !_18.2.fld0;
place!(Field::<usize>(Variant(_10, 0), 4)) = !1_usize;
_18.2.fld0 = !_22;
place!(Field::<u16>(Variant(_10, 0), 2)) = Field::<u32>(Variant(_10, 0), 3) as u16;
_31 = core::ptr::addr_of_mut!(_27);
_25.0 = &_30;
_2 = _4;
_29.0 = [22920_i16,11637_i16,(-3123_i16),(-31421_i16),(-13447_i16),(-25151_i16),(-9147_i16),11979_i16];
place!(Field::<u16>(Variant(_10, 0), 2)) = _18.2.fld1 as u16;
_18.0 = _18.2.fld1;
_2 = _7;
_18.2.fld0 = _22;
place!(Field::<u8>(Variant(_10, 0), 0)) = (-1037343716_i32) as u8;
Goto(bb14)
}
bb14 = {
_18.2.fld3 = -_17;
_22 = _6 == _2;
_18.2 = Adt19 { fld0: _22,fld1: _8,fld2: _4,fld3: _17,fld4: _14 };
_8 = RET;
_30 = 11882633352277027685_u64 | 12681407306325750222_u64;
_28 = [Field::<u32>(Variant(_10, 0), 3),Field::<u32>(Variant(_10, 0), 3),Field::<u32>(Variant(_10, 0), 3),Field::<u32>(Variant(_10, 0), 3),Field::<u32>(Variant(_10, 0), 3)];
_27 = 266616618623192842960581428978694201103_u128;
_27 = _1 as u128;
_26 = Field::<u16>(Variant(_10, 0), 2) >> _5;
place!(Field::<u16>(Variant(_10, 0), 2)) = _26;
_18.2 = Adt19 { fld0: _22,fld1: RET,fld2: _7,fld3: _17,fld4: _14 };
(*_31) = !20788293600893052269495140206101670885_u128;
_7 = (*_31) as isize;
_18.0 = RET;
_8 = _18.0;
_32 = _4;
_17 = _30 as i8;
_21 = [_2,_32,_5,_5];
_15.0 = _12;
_30 = !8419018933253916006_u64;
place!(Field::<u16>(Variant(_10, 0), 2)) = !_26;
_18.2.fld4 = _14 ^ _14;
_27 = !277958613197316619107822716463914751174_u128;
_18.2.fld4 = _14 << _4;
_18.2.fld0 = _22;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(5_usize, 2_usize, Move(_2), 16_usize, Move(_16), 22_usize, Move(_22), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(5_usize, 13_usize, Move(_13), 3_usize, Move(_3), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(5_usize, 26_usize, Move(_26), 6_usize, Move(_6), 17_usize, Move(_17), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u128,mut _2: isize,mut _3: bool,mut _4: isize,mut _5: i8,mut _6: bool,mut _7: isize) -> u16 {
mir! {
type RET = u16;
let _8: isize;
let _9: char;
let _10: &'static &'static (&'static u64,);
let _11: ([i16; 8],);
let _12: char;
let _13: &'static &'static i8;
let _14: i8;
let _15: i32;
let _16: char;
let _17: u64;
let _18: (u16, char, i8);
let _19: f64;
let _20: [i8; 4];
let _21: [u32; 7];
let _22: (*mut u128, Adt19);
let _23: f32;
let _24: isize;
let _25: u8;
let _26: &'static (&'static u64,);
let _27: [i8; 4];
let _28: bool;
let _29: &'static i32;
let _30: u16;
let _31: [u32; 5];
let _32: f64;
let _33: &'static Adt25;
let _34: usize;
let _35: (&'static u64,);
let _36: *mut Adt22;
let _37: i128;
let _38: &'static (*const u128, [u8; 6]);
let _39: (*mut u128, Adt19);
let _40: ();
let _41: ();
{
_6 = !_3;
_4 = _7;
RET = !36526_u16;
_2 = _7;
_9 = '\u{e284b}';
_6 = _3;
_8 = (-18475_i16) as isize;
RET = 63805_u16;
_11.0 = [(-28918_i16),(-26629_i16),(-30623_i16),21206_i16,(-6620_i16),(-29872_i16),3891_i16,22477_i16];
_8 = _4;
_15 = -1281706312_i32;
RET = _6 as u16;
RET = 321_u16;
_12 = _9;
_12 = _9;
_11.0 = [(-11181_i16),(-21011_i16),18251_i16,(-30107_i16),(-18851_i16),(-13654_i16),(-1866_i16),11936_i16];
_5 = 101_i8 << _1;
_16 = _12;
match RET {
321 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_14 = _5;
_15 = -1574073954_i32;
_12 = _16;
_17 = 1755379206420175983_u64 ^ 7679506573514464409_u64;
_11.0 = [8672_i16,8595_i16,(-2767_i16),(-3494_i16),5561_i16,(-20913_i16),(-10423_i16),8368_i16];
_7 = _2 + _2;
_2 = _7;
_2 = _12 as isize;
Call(_1 = fn7(_5, _11.0, _4, _7, _8, _11, _8, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11.0 = [(-6001_i16),(-29605_i16),31507_i16,6120_i16,(-32064_i16),(-31968_i16),23516_i16,(-21954_i16)];
_17 = 9603865781755445751_u64 >> _5;
_4 = _7 - _7;
_18.2 = _15 as i8;
_16 = _12;
_1 = 287542951542615377556625724846525416804_u128;
_6 = !_3;
_9 = _12;
_4 = (-5050235130564688327_i64) as isize;
_2 = _7 << _14;
_4 = _7;
_18.0 = !RET;
_2 = _4;
_1 = 228386899352858947674931082522501125458_u128;
_14 = _5 + _5;
match _1 {
0 => bb1,
1 => bb2,
228386899352858947674931082522501125458 => bb5,
_ => bb4
}
}
bb4 = {
_14 = _5;
_15 = -1574073954_i32;
_12 = _16;
_17 = 1755379206420175983_u64 ^ 7679506573514464409_u64;
_11.0 = [8672_i16,8595_i16,(-2767_i16),(-3494_i16),5561_i16,(-20913_i16),(-10423_i16),8368_i16];
_7 = _2 + _2;
_2 = _7;
_2 = _12 as isize;
Call(_1 = fn7(_5, _11.0, _4, _7, _8, _11, _8, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = _18.0 << _14;
RET = _18.0 & _18.0;
_1 = 318055796191899773312094110488354887668_u128;
_15 = -(-1805966888_i32);
_8 = _7;
_7 = _2 & _2;
_21 = [3983848029_u32,1727823767_u32,310696564_u32,3190351100_u32,3301278088_u32,468812600_u32,1337849871_u32];
_19 = 83_u8 as f64;
_20 = [_5,_14,_5,_14];
_6 = _3;
_9 = _12;
_18.2 = _15 as i8;
_21 = [2118828387_u32,3675724801_u32,1134528061_u32,3402974011_u32,217681698_u32,2102608514_u32,2132496912_u32];
_16 = _9;
_14 = _5 >> _17;
_22.1.fld4 = -(-74366113595013123980969514061141339126_i128);
_2 = !_8;
_22.0 = core::ptr::addr_of_mut!(_1);
_18.1 = _16;
_18 = (RET, _9, _5);
_22.1 = Adt19 { fld0: _3,fld1: _18.1,fld2: _8,fld3: _14,fld4: 160600243929699181416397242933182977243_i128 };
_25 = 183_u8;
Call(RET = core::intrinsics::transmute(_18.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_20 = [_22.1.fld3,_14,_22.1.fld3,_22.1.fld3];
_17 = !274671245263509105_u64;
_16 = _18.1;
_23 = _25 as f32;
_22.1 = Adt19 { fld0: _3,fld1: _18.1,fld2: _4,fld3: _5,fld4: (-25913470620747511090452747515675654221_i128) };
_25 = 140_u8;
_17 = !7801425364821873985_u64;
_22.1.fld3 = !_18.2;
_14 = _5;
_6 = _3;
_4 = _22.1.fld2 - _7;
_10 = &_26;
_19 = 26209_i16 as f64;
_16 = _18.1;
_24 = -_7;
_2 = _22.1.fld4 as isize;
Goto(bb7)
}
bb7 = {
_18.1 = _12;
RET = !_18.0;
_16 = _22.1.fld1;
_22.1.fld3 = _5 - _18.2;
_6 = _3 ^ _3;
match _22.1.fld4 {
0 => bb2,
1 => bb8,
314368896300190952372921859916092557235 => bb10,
_ => bb9
}
}
bb8 = {
_14 = _5;
_15 = -1574073954_i32;
_12 = _16;
_17 = 1755379206420175983_u64 ^ 7679506573514464409_u64;
_11.0 = [8672_i16,8595_i16,(-2767_i16),(-3494_i16),5561_i16,(-20913_i16),(-10423_i16),8368_i16];
_7 = _2 + _2;
_2 = _7;
_2 = _12 as isize;
Call(_1 = fn7(_5, _11.0, _4, _7, _8, _11, _8, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_11.0 = [(-6001_i16),(-29605_i16),31507_i16,6120_i16,(-32064_i16),(-31968_i16),23516_i16,(-21954_i16)];
_17 = 9603865781755445751_u64 >> _5;
_4 = _7 - _7;
_18.2 = _15 as i8;
_16 = _12;
_1 = 287542951542615377556625724846525416804_u128;
_6 = !_3;
_9 = _12;
_4 = (-5050235130564688327_i64) as isize;
_2 = _7 << _14;
_4 = _7;
_18.0 = !RET;
_2 = _4;
_1 = 228386899352858947674931082522501125458_u128;
_14 = _5 + _5;
match _1 {
0 => bb1,
1 => bb2,
228386899352858947674931082522501125458 => bb5,
_ => bb4
}
}
bb10 = {
_22.1.fld3 = 1669926991846307063_i64 as i8;
_10 = &_26;
_14 = _19 as i8;
RET = _18.0 | _18.0;
_22.1.fld1 = _18.1;
_6 = !_22.1.fld0;
_25 = !123_u8;
_22.1.fld1 = _12;
_1 = 297536403991794609423157750142145071156_u128 ^ 137758305257811148350366116653839978545_u128;
_19 = (-30070_i16) as f64;
_7 = _8 + _4;
_18.1 = _22.1.fld1;
_31 = [3385617160_u32,243215570_u32,3345511699_u32,2099949556_u32,4106364747_u32];
_3 = !_22.1.fld0;
_5 = 16865205354961343010_usize as i8;
_1 = !109889645108291384518878218564921776050_u128;
_5 = _18.2 ^ _18.2;
_30 = !_18.0;
_19 = _25 as f64;
_5 = !_18.2;
_22.1 = Adt19 { fld0: _6,fld1: _16,fld2: _7,fld3: _18.2,fld4: (-6627934253793225300360421686713075751_i128) };
_25 = !54_u8;
match _22.1.fld4 {
333654432667145238163014185745055135705 => bb11,
_ => bb2
}
}
bb11 = {
_25 = 7244_i16 as u8;
_11.0 = [22113_i16,(-28253_i16),(-26844_i16),(-20460_i16),17782_i16,3151_i16,10167_i16,7315_i16];
_22.1.fld2 = -_2;
_22.1.fld2 = !_7;
_22.1.fld0 = _3 & _3;
_9 = _12;
_27 = _20;
_10 = &_26;
_32 = _19 + _19;
_31 = [1697767503_u32,2303172251_u32,4061657530_u32,3860480297_u32,3180299744_u32];
_29 = &_15;
_23 = _25 as f32;
_18.1 = _16;
_18.0 = RET;
_16 = _12;
_22.1.fld3 = _18.2 & _5;
_4 = 5_usize as isize;
_29 = &(*_29);
_22.1.fld0 = _6;
_34 = !15157596759866135095_usize;
_6 = _22.1.fld0;
Call(_34 = core::intrinsics::bswap(5861404354766521235_usize), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22.0 = core::ptr::addr_of_mut!(_1);
_22.1 = Adt19 { fld0: _6,fld1: _16,fld2: _8,fld3: _5,fld4: (-90748782945935264850610798459547616899_i128) };
_18.1 = _9;
_26 = &_35;
_30 = _17 as u16;
_32 = _19;
_22.1 = Adt19 { fld0: _6,fld1: _18.1,fld2: _2,fld3: _18.2,fld4: (-54700305809459738585385065777332120180_i128) };
_24 = _2 >> _22.1.fld3;
_22.1.fld3 = !_18.2;
match _22.1.fld4 {
0 => bb7,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
285582061111478724877989541654436091276 => bb18,
_ => bb17
}
}
bb13 = {
RET = _18.0 << _14;
RET = _18.0 & _18.0;
_1 = 318055796191899773312094110488354887668_u128;
_15 = -(-1805966888_i32);
_8 = _7;
_7 = _2 & _2;
_21 = [3983848029_u32,1727823767_u32,310696564_u32,3190351100_u32,3301278088_u32,468812600_u32,1337849871_u32];
_19 = 83_u8 as f64;
_20 = [_5,_14,_5,_14];
_6 = _3;
_9 = _12;
_18.2 = _15 as i8;
_21 = [2118828387_u32,3675724801_u32,1134528061_u32,3402974011_u32,217681698_u32,2102608514_u32,2132496912_u32];
_16 = _9;
_14 = _5 >> _17;
_22.1.fld4 = -(-74366113595013123980969514061141339126_i128);
_2 = !_8;
_22.0 = core::ptr::addr_of_mut!(_1);
_18.1 = _16;
_18 = (RET, _9, _5);
_22.1 = Adt19 { fld0: _3,fld1: _18.1,fld2: _8,fld3: _14,fld4: 160600243929699181416397242933182977243_i128 };
_25 = 183_u8;
Call(RET = core::intrinsics::transmute(_18.0), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_22.1.fld3 = 1669926991846307063_i64 as i8;
_10 = &_26;
_14 = _19 as i8;
RET = _18.0 | _18.0;
_22.1.fld1 = _18.1;
_6 = !_22.1.fld0;
_25 = !123_u8;
_22.1.fld1 = _12;
_1 = 297536403991794609423157750142145071156_u128 ^ 137758305257811148350366116653839978545_u128;
_19 = (-30070_i16) as f64;
_7 = _8 + _4;
_18.1 = _22.1.fld1;
_31 = [3385617160_u32,243215570_u32,3345511699_u32,2099949556_u32,4106364747_u32];
_3 = !_22.1.fld0;
_5 = 16865205354961343010_usize as i8;
_1 = !109889645108291384518878218564921776050_u128;
_5 = _18.2 ^ _18.2;
_30 = !_18.0;
_19 = _25 as f64;
_5 = !_18.2;
_22.1 = Adt19 { fld0: _6,fld1: _16,fld2: _7,fld3: _18.2,fld4: (-6627934253793225300360421686713075751_i128) };
_25 = !54_u8;
match _22.1.fld4 {
333654432667145238163014185745055135705 => bb11,
_ => bb2
}
}
bb15 = {
_14 = _5;
_15 = -1574073954_i32;
_12 = _16;
_17 = 1755379206420175983_u64 ^ 7679506573514464409_u64;
_11.0 = [8672_i16,8595_i16,(-2767_i16),(-3494_i16),5561_i16,(-20913_i16),(-10423_i16),8368_i16];
_7 = _2 + _2;
_2 = _7;
_2 = _12 as isize;
Call(_1 = fn7(_5, _11.0, _4, _7, _8, _11, _8, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_14 = _5;
_15 = -1574073954_i32;
_12 = _16;
_17 = 1755379206420175983_u64 ^ 7679506573514464409_u64;
_11.0 = [8672_i16,8595_i16,(-2767_i16),(-3494_i16),5561_i16,(-20913_i16),(-10423_i16),8368_i16];
_7 = _2 + _2;
_2 = _7;
_2 = _12 as isize;
Call(_1 = fn7(_5, _11.0, _4, _7, _8, _11, _8, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_18.1 = _12;
RET = !_18.0;
_16 = _22.1.fld1;
_22.1.fld3 = _5 - _18.2;
_6 = _3 ^ _3;
match _22.1.fld4 {
0 => bb2,
1 => bb8,
314368896300190952372921859916092557235 => bb10,
_ => bb9
}
}
bb18 = {
_39.1 = Adt19 { fld0: _22.1.fld0,fld1: _12,fld2: _8,fld3: _22.1.fld3,fld4: _22.1.fld4 };
_22.1 = Move(_39.1);
Goto(bb19)
}
bb19 = {
Call(_40 = dump_var(6_usize, 30_usize, Move(_30), 4_usize, Move(_4), 27_usize, Move(_27), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(6_usize, 20_usize, Move(_20), 15_usize, Move(_15), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_40 = dump_var(6_usize, 12_usize, Move(_12), 5_usize, Move(_5), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i8,mut _2: [i16; 8],mut _3: isize,mut _4: isize,mut _5: isize,mut _6: ([i16; 8],),mut _7: isize,mut _8: i8) -> u128 {
mir! {
type RET = u128;
let _9: [char; 4];
let _10: bool;
let _11: *mut u128;
let _12: char;
let _13: (Adt56,);
let _14: [i8; 5];
let _15: Adt73;
let _16: u8;
let _17: &'static f64;
let _18: bool;
let _19: isize;
let _20: [bool; 3];
let _21: Adt56;
let _22: f64;
let _23: (f64, Adt22);
let _24: (f64, Adt22);
let _25: u64;
let _26: f64;
let _27: f32;
let _28: isize;
let _29: ();
let _30: ();
{
_6.0 = [(-6962_i16),26083_i16,2255_i16,13203_i16,(-12223_i16),(-21285_i16),9135_i16,11927_i16];
_4 = _5 >> _5;
_8 = !_1;
_6 = (_2,);
_7 = _5 ^ _4;
RET = !79019786179185749599899116488771083596_u128;
_8 = _1;
_1 = !_8;
_7 = _3 & _5;
_10 = false;
_10 = !false;
_11 = core::ptr::addr_of_mut!(RET);
_4 = _7 - _3;
_2 = _6.0;
_4 = _3 - _7;
_8 = _1 >> _1;
_6.0 = _2;
_11 = core::ptr::addr_of_mut!((*_11));
_12 = '\u{fd407}';
_5 = 1413960261_u32 as isize;
_5 = -_4;
_9 = [_12,_12,_12,_12];
_1 = !_8;
_2 = _6.0;
(*_11) = !150711860302139857300234946014191171622_u128;
Goto(bb1)
}
bb1 = {
(*_11) = 937712241_i32 as u128;
_2 = _6.0;
_4 = _5;
_6 = (_2,);
_2 = _6.0;
_5 = 8511817123021752693_i64 as isize;
_7 = _4 - _3;
_9 = [_12,_12,_12,_12];
_2 = [2391_i16,(-25251_i16),15628_i16,(-17378_i16),(-12696_i16),285_i16,(-14947_i16),31833_i16];
_9 = [_12,_12,_12,_12];
_9 = [_12,_12,_12,_12];
_8 = !_1;
Call(_13.0 = fn8(_7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _8;
_8 = _10 as i8;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).1 = _12;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)) = (42918_u16, _12, _1);
_6 = (_2,);
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)) = (30197_u16, _12, _1);
RET = 302219976164224863905653409743068701383_u128 >> _3;
_2 = _6.0;
_6.0 = _2;
_6.0 = [32155_i16,(-19067_i16),28267_i16,17288_i16,4403_i16,(-9613_i16),28262_i16,(-12165_i16)];
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).1 = _12;
place!(Field::<u32>(Variant(_13.0, 2), 2)) = !2012270907_u32;
_11 = core::ptr::addr_of_mut!((*_11));
place!(Field::<f64>(Variant(_13.0, 2), 1)) = 5_u8 as f64;
(*_11) = 140605418547053563795911856894382337217_u128;
place!(Field::<u32>(Variant(_13.0, 2), 2)) = 2735545104_u32 - 1659830697_u32;
RET = !19255042860189248205866357449400802662_u128;
Goto(bb3)
}
bb3 = {
_11 = core::ptr::addr_of_mut!(RET);
_3 = _7;
_5 = _3;
_14 = [Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).2,_1,_1,_1,_1];
RET = (-5206_i16) as u128;
_6.0 = _2;
_5 = _3 - _3;
place!(Field::<f64>(Variant(_13.0, 2), 1)) = 157491756886971455932353435570811769712_i128 as f64;
_7 = -_3;
_7 = _3 | _5;
_6.0 = [(-20602_i16),30273_i16,11830_i16,(-17407_i16),(-3480_i16),(-28687_i16),(-10675_i16),(-25408_i16)];
Goto(bb4)
}
bb4 = {
_17 = &place!(Field::<f64>(Variant(_13.0, 2), 1));
Call((*_11) = core::intrinsics::transmute(_9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16 = !99_u8;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).2 = _1;
_6.0 = _2;
SetDiscriminant(_13.0, 2);
_4 = _5;
_5 = _4 - _7;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)) = (46081_u16, _12, _1);
_12 = Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).1;
place!(Field::<*mut u128>(Variant(_13.0, 2), 3)) = Move(_11);
_17 = &place!(Field::<f64>(Variant(_13.0, 2), 1));
place!(Field::<f64>(Variant(_13.0, 2), 1)) = _3 as f64;
_18 = !_10;
_16 = 164_u8 & 64_u8;
match Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).0 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
46081 => bb11,
_ => bb10
}
}
bb6 = {
_17 = &place!(Field::<f64>(Variant(_13.0, 2), 1));
Call((*_11) = core::intrinsics::transmute(_9), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_11 = core::ptr::addr_of_mut!(RET);
_3 = _7;
_5 = _3;
_14 = [Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).2,_1,_1,_1,_1];
RET = (-5206_i16) as u128;
_6.0 = _2;
_5 = _3 - _3;
place!(Field::<f64>(Variant(_13.0, 2), 1)) = 157491756886971455932353435570811769712_i128 as f64;
_7 = -_3;
_7 = _3 | _5;
_6.0 = [(-20602_i16),30273_i16,11830_i16,(-17407_i16),(-3480_i16),(-28687_i16),(-10675_i16),(-25408_i16)];
Goto(bb4)
}
bb8 = {
_1 = _8;
_8 = _10 as i8;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).1 = _12;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)) = (42918_u16, _12, _1);
_6 = (_2,);
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)) = (30197_u16, _12, _1);
RET = 302219976164224863905653409743068701383_u128 >> _3;
_2 = _6.0;
_6.0 = _2;
_6.0 = [32155_i16,(-19067_i16),28267_i16,17288_i16,4403_i16,(-9613_i16),28262_i16,(-12165_i16)];
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).1 = _12;
place!(Field::<u32>(Variant(_13.0, 2), 2)) = !2012270907_u32;
_11 = core::ptr::addr_of_mut!((*_11));
place!(Field::<f64>(Variant(_13.0, 2), 1)) = 5_u8 as f64;
(*_11) = 140605418547053563795911856894382337217_u128;
place!(Field::<u32>(Variant(_13.0, 2), 2)) = 2735545104_u32 - 1659830697_u32;
RET = !19255042860189248205866357449400802662_u128;
Goto(bb3)
}
bb9 = {
(*_11) = 937712241_i32 as u128;
_2 = _6.0;
_4 = _5;
_6 = (_2,);
_2 = _6.0;
_5 = 8511817123021752693_i64 as isize;
_7 = _4 - _3;
_9 = [_12,_12,_12,_12];
_2 = [2391_i16,(-25251_i16),15628_i16,(-17378_i16),(-12696_i16),285_i16,(-14947_i16),31833_i16];
_9 = [_12,_12,_12,_12];
_9 = [_12,_12,_12,_12];
_8 = !_1;
Call(_13.0 = fn8(_7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_18 = _10 & _10;
_1 = Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).2;
_6.0 = [(-18417_i16),4668_i16,(-20929_i16),25826_i16,(-1690_i16),16023_i16,(-30006_i16),(-454_i16)];
_9 = [Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).1,_12,_12,Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).1];
_5 = -_3;
_1 = Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).2;
_5 = 113143673_u32 as isize;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).2 = _1 + _1;
_18 = !_10;
_2 = _6.0;
_4 = _7;
_16 = !110_u8;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).2 = 4014553557644361007_u64 as i8;
place!(Field::<(u16, char, i8)>(Variant(_13.0, 2), 0)).1 = _12;
_1 = !Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).2;
_2 = [(-7203_i16),28129_i16,(-9667_i16),19599_i16,2715_i16,(-26506_i16),17964_i16,2913_i16];
_6.0 = [24552_i16,8930_i16,15641_i16,(-22543_i16),(-25676_i16),1687_i16,(-16529_i16),28515_i16];
_19 = _3;
_11 = core::ptr::addr_of_mut!(RET);
(*_11) = 106259790977593974721355778783434967440_u128;
(*_11) = 252014144177883346986173411465817883408_u128;
place!(Field::<*mut u128>(Variant(_13.0, 2), 3)) = core::ptr::addr_of_mut!((*_11));
match Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).0 {
0 => bb7,
1 => bb10,
2 => bb8,
3 => bb4,
4 => bb5,
46081 => bb13,
_ => bb12
}
}
bb12 = {
(*_11) = 937712241_i32 as u128;
_2 = _6.0;
_4 = _5;
_6 = (_2,);
_2 = _6.0;
_5 = 8511817123021752693_i64 as isize;
_7 = _4 - _3;
_9 = [_12,_12,_12,_12];
_2 = [2391_i16,(-25251_i16),15628_i16,(-17378_i16),(-12696_i16),285_i16,(-14947_i16),31833_i16];
_9 = [_12,_12,_12,_12];
_9 = [_12,_12,_12,_12];
_8 = !_1;
Call(_13.0 = fn8(_7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_23.0 = Field::<f64>(Variant(_13.0, 2), 1);
_6.0 = _2;
_20 = [_18,_10,_10];
_20 = [_10,_18,_10];
place!(Field::<u32>(Variant(_13.0, 2), 2)) = 3061767331_u32 - 453786898_u32;
_24.0 = (-1706381875_i32) as f64;
_14 = [_8,Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).2,_8,_1,_1];
_17 = &_24.0;
RET = !273893700428727518280413873821731988310_u128;
_24.1 = Adt22::Variant2 { fld0: _16,fld1: (*_11) };
_12 = Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).1;
place!(Field::<*mut u128>(Variant(_13.0, 2), 3)) = core::ptr::addr_of_mut!((*_11));
_12 = Field::<(u16, char, i8)>(Variant(_13.0, 2), 0).1;
RET = Field::<u128>(Variant(_24.1, 2), 1) & Field::<u128>(Variant(_24.1, 2), 1);
_28 = -_3;
_21 = Move(_13.0);
RET = _19 as u128;
place!(Field::<f64>(Variant(_21, 2), 1)) = _23.0 + _23.0;
Goto(bb14)
}
bb14 = {
Call(_29 = dump_var(7_usize, 20_usize, Move(_20), 19_usize, Move(_19), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_29 = dump_var(7_usize, 7_usize, Move(_7), 18_usize, Move(_18), 14_usize, Move(_14), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: isize) -> Adt56 {
mir! {
type RET = Adt56;
let _3: isize;
let _4: Adt33;
let _5: isize;
let _6: f64;
let _7: u8;
let _8: i8;
let _9: f64;
let _10: Adt33;
let _11: &'static i8;
let _12: Adt73;
let _13: (bool, &'static u128);
let _14: Adt25;
let _15: f32;
let _16: Adt22;
let _17: *mut [isize; 1];
let _18: isize;
let _19: [i32; 1];
let _20: (Adt19, &'static i8, isize);
let _21: u128;
let _22: char;
let _23: i32;
let _24: &'static i8;
let _25: *const f32;
let _26: isize;
let _27: f32;
let _28: *mut [i8; 4];
let _29: bool;
let _30: bool;
let _31: u8;
let _32: bool;
let _33: *mut [i8; 4];
let _34: i64;
let _35: Adt33;
let _36: *mut Adt22;
let _37: usize;
let _38: &'static Adt25;
let _39: bool;
let _40: [u64; 4];
let _41: *mut f64;
let _42: i8;
let _43: (f64, Adt22);
let _44: ();
let _45: ();
{
_2 = -_1;
_1 = _2 | _2;
_1 = _2;
_2 = _1 - _1;
_2 = _1 & _1;
_2 = _1 << _1;
_3 = -_1;
_1 = !_3;
_3 = _2 & _2;
_2 = _3;
_2 = 1370135612_i32 as isize;
_1 = -_3;
_2 = _1;
_3 = _2;
_2 = (-4639903738230084987_i64) as isize;
_3 = '\u{2a53e}' as isize;
_1 = !_2;
_1 = _2 * _2;
_2 = _1 * _1;
_1 = (-145002982745672602300822180190381713089_i128) as isize;
_3 = _2;
_1 = _2 ^ _3;
_2 = 38963784114167037241076539780922196491_i128 as isize;
_2 = _3;
_2 = _1;
_3 = _2 << _1;
_2 = 118_i8 as isize;
_2 = !_3;
Goto(bb1)
}
bb1 = {
_2 = 3088365045405710633581560660259382271_i128 as isize;
_3 = _1 ^ _1;
_3 = -_2;
_3 = _1;
_6 = 24_u8 as f64;
_4 = Adt33::Variant0 { fld0: 225_u8,fld1: '\u{f926e}',fld2: 5977_u16,fld3: 587202864_u32,fld4: 1_usize };
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{9c8ba}';
place!(Field::<u16>(Variant(_4, 0), 2)) = 34179_u16;
place!(Field::<u16>(Variant(_4, 0), 2)) = (-83531061809739211315442788178299658701_i128) as u16;
_5 = _2 & _3;
place!(Field::<usize>(Variant(_4, 0), 4)) = Field::<u16>(Variant(_4, 0), 2) as usize;
_7 = (-97794086527069153332386910762350947957_i128) as u8;
place!(Field::<u8>(Variant(_4, 0), 0)) = 3759939687_u32 as u8;
_9 = _6 + _6;
_11 = &_8;
Call(_5 = fn9(_1, _3, _1, _1, _3, _6, Field::<usize>(Variant(_4, 0), 4), _1, _3, _3, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = Field::<char>(Variant(_4, 0), 1) as i8;
_6 = _9 + _9;
_3 = 11870013468811030103_u64 as isize;
_4 = Adt33::Variant0 { fld0: _7,fld1: '\u{71c44}',fld2: 19363_u16,fld3: 557518449_u32,fld4: 6600676872173886591_usize };
_4 = Adt33::Variant0 { fld0: _7,fld1: '\u{1672a}',fld2: 4386_u16,fld3: 4209723797_u32,fld4: 5313940806426120630_usize };
_2 = _5 | _5;
_8 = 50_i8 - 81_i8;
place!(Field::<u32>(Variant(_4, 0), 3)) = 1426474968794134163725942752400811531_i128 as u32;
place!(Field::<u16>(Variant(_4, 0), 2)) = 45707_u16;
_6 = _9 + _9;
place!(Field::<u16>(Variant(_4, 0), 2)) = 29202_u16;
place!(Field::<usize>(Variant(_4, 0), 4)) = !1806333505737474378_usize;
place!(Field::<usize>(Variant(_4, 0), 4)) = 7_usize;
_13.0 = !true;
_3 = _5 - _2;
place!(Field::<u8>(Variant(_4, 0), 0)) = _2 as u8;
_11 = &_8;
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{19201}';
place!(Field::<u16>(Variant(_4, 0), 2)) = Field::<u8>(Variant(_4, 0), 0) as u16;
_1 = !_5;
_10 = Move(_4);
_4 = Move(_10);
place!(Field::<u16>(Variant(_4, 0), 2)) = 6923_u16;
match Field::<usize>(Variant(_4, 0), 4) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
7 => bb11,
_ => bb10
}
}
bb3 = {
_2 = 3088365045405710633581560660259382271_i128 as isize;
_3 = _1 ^ _1;
_3 = -_2;
_3 = _1;
_6 = 24_u8 as f64;
_4 = Adt33::Variant0 { fld0: 225_u8,fld1: '\u{f926e}',fld2: 5977_u16,fld3: 587202864_u32,fld4: 1_usize };
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{9c8ba}';
place!(Field::<u16>(Variant(_4, 0), 2)) = 34179_u16;
place!(Field::<u16>(Variant(_4, 0), 2)) = (-83531061809739211315442788178299658701_i128) as u16;
_5 = _2 & _3;
place!(Field::<usize>(Variant(_4, 0), 4)) = Field::<u16>(Variant(_4, 0), 2) as usize;
_7 = (-97794086527069153332386910762350947957_i128) as u8;
place!(Field::<u8>(Variant(_4, 0), 0)) = 3759939687_u32 as u8;
_9 = _6 + _6;
_11 = &_8;
Call(_5 = fn9(_1, _3, _1, _1, _3, _6, Field::<usize>(Variant(_4, 0), 4), _1, _3, _3, _1), ReturnTo(bb2), UnwindUnreachable())
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
_6 = _9 + _9;
_8 = (-26_i8);
_11 = &_8;
_15 = 36589438579585842875502555255020016230_i128 as f32;
_8 = 64_i8;
_3 = 2571579908715043519_u64 as isize;
place!(Field::<u32>(Variant(_4, 0), 3)) = 1017297283_u32;
_11 = &_8;
_13.0 = _2 >= _1;
_7 = Field::<char>(Variant(_4, 0), 1) as u8;
_10 = Move(_4);
_20.0 = Adt19 { fld0: _13.0,fld1: Field::<char>(Variant(_10, 0), 1),fld2: _2,fld3: (*_11),fld4: 32460378216770312722889959734942023566_i128 };
_9 = Field::<u16>(Variant(_10, 0), 2) as f64;
_14 = Adt25::Variant0 { fld0: _1 };
match _20.0.fld4 {
0 => bb10,
1 => bb2,
2 => bb9,
3 => bb8,
4 => bb5,
32460378216770312722889959734942023566 => bb12,
_ => bb7
}
}
bb12 = {
_5 = _2;
_9 = _6;
_4 = Adt33::Variant0 { fld0: Field::<u8>(Variant(_10, 0), 0),fld1: _20.0.fld1,fld2: Field::<u16>(Variant(_10, 0), 2),fld3: Field::<u32>(Variant(_10, 0), 3),fld4: Field::<usize>(Variant(_10, 0), 4) };
_20.0.fld0 = _2 != _5;
_2 = _20.0.fld2 << _1;
_13.0 = _20.0.fld0;
_20.2 = !_20.0.fld2;
place!(Field::<u8>(Variant(_4, 0), 0)) = Field::<u8>(Variant(_10, 0), 0) & Field::<u8>(Variant(_10, 0), 0);
_4 = Move(_10);
_4 = Adt33::Variant0 { fld0: _7,fld1: _20.0.fld1,fld2: 55500_u16,fld3: 1461636649_u32,fld4: 6_usize };
_10 = Adt33::Variant0 { fld0: Field::<u8>(Variant(_4, 0), 0),fld1: Field::<char>(Variant(_4, 0), 1),fld2: 2710_u16,fld3: 4191822400_u32,fld4: 0_usize };
place!(Field::<u8>(Variant(_4, 0), 0)) = !_7;
place!(Field::<usize>(Variant(_4, 0), 4)) = 11317739669491665204_usize;
Call(place!(Field::<isize>(Variant(_14, 0), 0)) = core::intrinsics::bswap(_2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = -_9;
place!(Field::<u16>(Variant(_4, 0), 2)) = 8398_u16 >> Field::<isize>(Variant(_14, 0), 0);
_20.1 = &(*_11);
_13.1 = &_21;
_1 = _15 as isize;
place!(Field::<u32>(Variant(_4, 0), 3)) = 110086397_u32;
_3 = !_20.2;
_2 = _20.2 >> _20.0.fld2;
match _20.0.fld4 {
32460378216770312722889959734942023566 => bb14,
_ => bb5
}
}
bb14 = {
_9 = _15 as f64;
_19 = [(-663164037_i32)];
SetDiscriminant(_14, 0);
_7 = Field::<u8>(Variant(_4, 0), 0);
_20.0 = Adt19 { fld0: _13.0,fld1: Field::<char>(Variant(_4, 0), 1),fld2: _5,fld3: (*_11),fld4: 147765578997843808080540128286821515290_i128 };
place!(Field::<usize>(Variant(_10, 0), 4)) = _20.0.fld1 as usize;
_5 = -_20.0.fld2;
_6 = -_9;
_7 = Field::<u8>(Variant(_4, 0), 0);
place!(Field::<char>(Variant(_4, 0), 1)) = Field::<char>(Variant(_10, 0), 1);
place!(Field::<u8>(Variant(_4, 0), 0)) = _20.2 as u8;
_2 = _20.2;
_6 = Field::<usize>(Variant(_4, 0), 4) as f64;
place!(Field::<u32>(Variant(_10, 0), 3)) = !Field::<u32>(Variant(_4, 0), 3);
_22 = _20.0.fld1;
_6 = Field::<usize>(Variant(_10, 0), 4) as f64;
_20.0.fld4 = 125141321577003844972407445477245878932_i128 * (-990056732523051296913746574450364068_i128);
_2 = _5;
place!(Field::<isize>(Variant(_14, 0), 0)) = -_2;
_20.0 = Adt19 { fld0: _13.0,fld1: _22,fld2: Field::<isize>(Variant(_14, 0), 0),fld3: _8,fld4: 137789190987250829528413599264260619222_i128 };
_20.0.fld3 = (*_11) - _8;
Goto(bb15)
}
bb15 = {
_22 = _20.0.fld1;
_18 = _9 as isize;
match _20.0.fld4 {
0 => bb8,
137789190987250829528413599264260619222 => bb17,
_ => bb16
}
}
bb16 = {
Return()
}
bb17 = {
place!(Field::<char>(Variant(_4, 0), 1)) = _20.0.fld1;
_24 = Move(_11);
_20.0.fld4 = (-20604502054748587883949853369832136207_i128) - 62485392763300054112925341986426569060_i128;
_21 = 313945931637311165310148545919076894276_u128;
_11 = &_8;
match _8 {
0 => bb13,
1 => bb11,
2 => bb16,
3 => bb4,
4 => bb15,
5 => bb6,
64 => bb19,
_ => bb18
}
}
bb18 = {
Return()
}
bb19 = {
_11 = &_8;
_20.0.fld2 = !_20.2;
SetDiscriminant(_14, 2);
_4 = Adt33::Variant0 { fld0: Field::<u8>(Variant(_10, 0), 0),fld1: _20.0.fld1,fld2: 8443_u16,fld3: Field::<u32>(Variant(_10, 0), 3),fld4: Field::<usize>(Variant(_10, 0), 4) };
_1 = 3099688657848615739_u64 as isize;
_25 = core::ptr::addr_of!(_15);
place!(Field::<char>(Variant(_4, 0), 1)) = _20.0.fld1;
(*_25) = 11178718478013658048_u64 as f32;
place!(Field::<u32>(Variant(_10, 0), 3)) = Field::<u32>(Variant(_4, 0), 3);
place!(Field::<u128>(Variant(_14, 2), 0)) = _8 as u128;
_24 = &_20.0.fld3;
_22 = Field::<char>(Variant(_10, 0), 1);
(*_25) = Field::<usize>(Variant(_4, 0), 4) as f32;
_23 = 1224593544_i32 - (-1328355103_i32);
place!(Field::<usize>(Variant(_10, 0), 4)) = !Field::<usize>(Variant(_4, 0), 4);
_2 = _5 & _3;
Goto(bb20)
}
bb20 = {
place!(Field::<usize>(Variant(_10, 0), 4)) = Field::<usize>(Variant(_4, 0), 4);
_1 = _20.0.fld2;
_11 = &_20.0.fld3;
_14 = Adt25::Variant0 { fld0: _20.2 };
_20.0.fld3 = _20.0.fld4 as i8;
_15 = Field::<usize>(Variant(_10, 0), 4) as f32;
_20.0.fld3 = 12374_i16 as i8;
_13.0 = !_20.0.fld0;
_24 = &_8;
place!(Field::<usize>(Variant(_10, 0), 4)) = Field::<usize>(Variant(_4, 0), 4) >> _5;
_20.0.fld4 = 24353591099370530527209007543432109383_i128;
_20.0.fld1 = Field::<char>(Variant(_10, 0), 1);
_32 = _20.2 < _1;
_3 = !_20.2;
_25 = core::ptr::addr_of!((*_25));
_26 = -_1;
_16 = Adt22::Variant2 { fld0: Field::<u8>(Variant(_4, 0), 0),fld1: _21 };
_34 = (-7368508399465146920_i64) | (-3746445060991009430_i64);
place!(Field::<u32>(Variant(_4, 0), 3)) = Field::<u32>(Variant(_10, 0), 3) - Field::<u32>(Variant(_10, 0), 3);
_18 = _23 as isize;
place!(Field::<u8>(Variant(_4, 0), 0)) = Field::<u8>(Variant(_16, 2), 0);
place!(Field::<u16>(Variant(_10, 0), 2)) = 45647_u16;
match _21 {
0 => bb11,
1 => bb4,
2 => bb13,
3 => bb21,
313945931637311165310148545919076894276 => bb23,
_ => bb22
}
}
bb21 = {
_2 = 3088365045405710633581560660259382271_i128 as isize;
_3 = _1 ^ _1;
_3 = -_2;
_3 = _1;
_6 = 24_u8 as f64;
_4 = Adt33::Variant0 { fld0: 225_u8,fld1: '\u{f926e}',fld2: 5977_u16,fld3: 587202864_u32,fld4: 1_usize };
place!(Field::<char>(Variant(_4, 0), 1)) = '\u{9c8ba}';
place!(Field::<u16>(Variant(_4, 0), 2)) = 34179_u16;
place!(Field::<u16>(Variant(_4, 0), 2)) = (-83531061809739211315442788178299658701_i128) as u16;
_5 = _2 & _3;
place!(Field::<usize>(Variant(_4, 0), 4)) = Field::<u16>(Variant(_4, 0), 2) as usize;
_7 = (-97794086527069153332386910762350947957_i128) as u8;
place!(Field::<u8>(Variant(_4, 0), 0)) = 3759939687_u32 as u8;
_9 = _6 + _6;
_11 = &_8;
Call(_5 = fn9(_1, _3, _1, _1, _3, _6, Field::<usize>(Variant(_4, 0), 4), _1, _3, _3, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb22 = {
Return()
}
bb23 = {
place!(Field::<u8>(Variant(_16, 2), 0)) = Field::<u8>(Variant(_10, 0), 0) | _7;
_31 = _20.0.fld4 as u8;
_20.0.fld0 = _13.0;
_3 = _20.0.fld3 as isize;
_35 = Adt33::Variant1 { fld0: _15,fld1: Move(_16),fld2: Field::<usize>(Variant(_10, 0), 4) };
place!(Field::<f32>(Variant(_35, 1), 0)) = (*_25);
_11 = &(*_24);
(*_25) = Field::<u8>(Variant(Field::<Adt22>(Variant(_35, 1), 1), 2), 0) as f32;
_32 = !_13.0;
_20.0.fld2 = _5 ^ _2;
_29 = _13.0;
_27 = (*_25) + (*_25);
place!(Field::<usize>(Variant(_4, 0), 4)) = Field::<usize>(Variant(_35, 1), 2);
_10 = Move(_35);
_20.0.fld2 = !_26;
_20.0.fld4 = (-105819796911316963183179624939814723226_i128);
place!(Field::<u16>(Variant(_4, 0), 2)) = 58600_u16 | 58410_u16;
place!(Field::<isize>(Variant(_14, 0), 0)) = _5;
_20.0 = Adt19 { fld0: _32,fld1: Field::<char>(Variant(_4, 0), 1),fld2: _5,fld3: _8,fld4: (-20422690972636841184810015215994412752_i128) };
_30 = _29;
_2 = Field::<usize>(Variant(_10, 1), 2) as isize;
_37 = !Field::<usize>(Variant(_10, 1), 2);
_20.0 = Adt19 { fld0: _32,fld1: _22,fld2: _1,fld3: (*_24),fld4: 55703075892936460500921767760556385567_i128 };
_5 = _20.0.fld2;
SetDiscriminant(Field::<Adt22>(Variant(_10, 1), 1), 2);
_38 = &_14;
place!(Field::<u8>(Variant(_4, 0), 0)) = _7 - _31;
place!(Field::<usize>(Variant(_4, 0), 4)) = _37;
Call(RET = fn11(Move(_20), Move(_38), Move(_11), Move(_24), Move(_4), Field::<isize>(Variant(_14, 0), 0), _29, Field::<isize>(Variant((*_38), 0), 0), _5, _32, _13.0, _26), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_20.0.fld1 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).2 = _8;
_31 = _29 as u8;
_39 = !_32;
_20.2 = _23 as isize;
_20.0.fld4 = Field::<u32>(Variant(RET, 2), 2) as i128;
_19 = [_23];
_37 = Field::<u32>(Variant(RET, 2), 2) as usize;
_4 = Adt33::Variant0 { fld0: _31,fld1: _20.0.fld1,fld2: Field::<(u16, char, i8)>(Variant(RET, 2), 0).0,fld3: Field::<u32>(Variant(RET, 2), 2),fld4: _37 };
place!(Field::<f32>(Variant(_10, 1), 0)) = _27 * _27;
_41 = core::ptr::addr_of_mut!(_6);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)) = (Field::<u16>(Variant(_4, 0), 2), _20.0.fld1, _8);
place!(Field::<f64>(Variant(RET, 2), 1)) = -(*_41);
_20.0 = Adt19 { fld0: _29,fld1: Field::<char>(Variant(_4, 0), 1),fld2: _26,fld3: Field::<(u16, char, i8)>(Variant(RET, 2), 0).2,fld4: (-160978684564441669760967124000741286048_i128) };
_15 = Field::<f32>(Variant(_10, 1), 0) * Field::<f32>(Variant(_10, 1), 0);
_20.0.fld1 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
Goto(bb25)
}
bb25 = {
Call(_44 = dump_var(8_usize, 5_usize, Move(_5), 22_usize, Move(_22), 1_usize, Move(_1), 32_usize, Move(_32)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_44 = dump_var(8_usize, 39_usize, Move(_39), 2_usize, Move(_2), 21_usize, Move(_21), 31_usize, Move(_31)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_44 = dump_var(8_usize, 7_usize, Move(_7), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: f64,mut _7: usize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> isize {
mir! {
type RET = isize;
let _12: char;
let _13: u64;
let _14: *mut u8;
let _15: char;
let _16: isize;
let _17: f32;
let _18: ();
let _19: ();
{
_3 = 157644870956119495997775076078372994809_u128 as isize;
_11 = '\u{15841}' as isize;
_6 = (-5234_i16) as f64;
_4 = '\u{caafd}' as isize;
_5 = 16576_u16 as isize;
_8 = _1;
RET = _2 - _10;
_1 = !_8;
_9 = !_10;
_7 = true as usize;
_3 = _8 ^ _2;
_4 = (-298097638_i32) as isize;
_3 = _9 * RET;
RET = (-34_i8) as isize;
_1 = _3;
Call(_8 = fn10(_3, _3, _1, _10, _2, _2, _1, _10, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _3 >> _5;
_2 = !_3;
_1 = _2 - _8;
Goto(bb2)
}
bb2 = {
_1 = 198_u8 as isize;
_7 = 2_usize * 0_usize;
_11 = _2;
_1 = _9 | _11;
_10 = _2 | _3;
_12 = '\u{3e3}';
_5 = _7 as isize;
_9 = _7 as isize;
_2 = _8 << _11;
_1 = _10 * _11;
_7 = 109984224538660967_usize * 15825323809459915893_usize;
_15 = _12;
_5 = _3;
RET = _10;
_13 = 5516450049669928609_u64;
_3 = _10;
_8 = _3;
_7 = 3_usize;
_6 = (-12735_i16) as f64;
_2 = _1;
_16 = _10;
_7 = _12 as usize;
_1 = 20123226_i32 as isize;
_7 = !1145710679803640577_usize;
_8 = _4 ^ _5;
_2 = _13 as isize;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(9_usize, 9_usize, Move(_9), 10_usize, Move(_10), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(9_usize, 16_usize, Move(_16), 3_usize, Move(_3), 8_usize, Move(_8), 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: f32;
let _11: [i64; 1];
let _12: bool;
let _13: ();
let _14: ();
{
RET = _3 & _7;
_5 = _3 >> RET;
_3 = _5 + RET;
_11 = [(-4333874304891357076_i64)];
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(10_usize, 1_usize, Move(_1), 2_usize, Move(_2), 11_usize, Move(_11), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(10_usize, 6_usize, Move(_6), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: (Adt19, &'static i8, isize),mut _2: &'static Adt25,mut _3: &'static i8,mut _4: &'static i8,mut _5: Adt33,mut _6: isize,mut _7: bool,mut _8: isize,mut _9: isize,mut _10: bool,mut _11: bool,mut _12: isize) -> Adt56 {
mir! {
type RET = Adt56;
let _13: [i32; 1];
let _14: i64;
let _15: Adt73;
let _16: isize;
let _17: *mut Adt22;
let _18: &'static u64;
let _19: char;
let _20: &'static (u16, char, i8);
let _21: Adt19;
let _22: f32;
let _23: ([u32; 7],);
let _24: isize;
let _25: (*mut u128, Adt19);
let _26: *mut *mut [i8; 4];
let _27: *mut u8;
let _28: u16;
let _29: (&'static u64,);
let _30: isize;
let _31: bool;
let _32: f32;
let _33: (u16, char, i8);
let _34: Adt19;
let _35: &'static u64;
let _36: &'static u64;
let _37: &'static i8;
let _38: isize;
let _39: i32;
let _40: i32;
let _41: ([u32; 7],);
let _42: f64;
let _43: [u128; 6];
let _44: isize;
let _45: u128;
let _46: u16;
let _47: f32;
let _48: &'static &'static i8;
let _49: u64;
let _50: &'static &'static (&'static u64,);
let _51: bool;
let _52: isize;
let _53: [i32; 1];
let _54: (*mut u128, Adt19);
let _55: f32;
let _56: u8;
let _57: isize;
let _58: u32;
let _59: f32;
let _60: Adt77;
let _61: char;
let _62: &'static u128;
let _63: [bool; 3];
let _64: isize;
let _65: u64;
let _66: ([u32; 7],);
let _67: (*const u128, [u8; 6]);
let _68: i16;
let _69: u32;
let _70: u32;
let _71: [i64; 8];
let _72: i128;
let _73: Adt25;
let _74: ();
let _75: ();
{
_1.1 = &_1.0.fld3;
Call(_12 = core::intrinsics::bswap(_1.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<u32>(Variant(_5, 0), 3)) = 1591133706_u32 ^ 3576010540_u32;
_1.0.fld3 = (-71_i8) >> _1.0.fld4;
_9 = _6;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<u32>(Variant(_5, 0), 3)) = 2861952938_u32 << _12;
place!(Field::<usize>(Variant(_5, 0), 4)) = !13414888479744933802_usize;
Call(RET = fn12(Field::<u32>(Variant(_5, 0), 3), Move(_5), Move(_1.0), _10, _12, _8, _9, _12, _8, _12, _10, _1.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.0 = Adt19 { fld0: _7,fld1: '\u{7146c}',fld2: _1.2,fld3: 12_i8,fld4: 123235471507091351230788896785344896716_i128 };
_1.0 = Adt19 { fld0: _11,fld1: '\u{7f730}',fld2: _12,fld3: (-94_i8),fld4: (-64417132475884303217364187517369945584_i128) };
_1.0.fld2 = _9;
_1.1 = &_1.0.fld3;
_14 = (-7673832613698295214_i64);
_5 = Adt33::Variant0 { fld0: 178_u8,fld1: _1.0.fld1,fld2: 21630_u16,fld3: 4013426468_u32,fld4: 7_usize };
_1.0.fld3 = _1.0.fld4 as i8;
place!(Field::<u16>(Variant(_5, 0), 2)) = 45516_u16;
SetDiscriminant(RET, 2);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)) = (Field::<u16>(Variant(_5, 0), 2), Field::<char>(Variant(_5, 0), 1), _1.0.fld3);
_5 = Adt33::Variant0 { fld0: 91_u8,fld1: _1.0.fld1,fld2: Field::<(u16, char, i8)>(Variant(RET, 2), 0).0,fld3: 1731657928_u32,fld4: 1_usize };
_9 = -_12;
place!(Field::<f64>(Variant(RET, 2), 1)) = (-108139927_i32) as f64;
_7 = _10;
_1.0.fld4 = 19645057058562715167717797067181042734_i128 << Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_1.0.fld0 = _10;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
Goto(bb3)
}
bb3 = {
_1.2 = _8;
_1.2 = _14 as isize;
_4 = &_21.fld3;
_11 = !_1.0.fld0;
_1.0.fld0 = !_7;
_21.fld2 = 3679445560_u32 as isize;
_16 = _9 & _12;
_1.0.fld1 = Field::<char>(Variant(_5, 0), 1);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = Field::<u16>(Variant(_5, 0), 2) >> _1.0.fld2;
_19 = _1.0.fld1;
_7 = _11 == _10;
match _14 {
340282366920938463455700774818069916242 => bb5,
_ => bb4
}
}
bb4 = {
place!(Field::<u32>(Variant(_5, 0), 3)) = 1591133706_u32 ^ 3576010540_u32;
_1.0.fld3 = (-71_i8) >> _1.0.fld4;
_9 = _6;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<u32>(Variant(_5, 0), 3)) = 2861952938_u32 << _12;
place!(Field::<usize>(Variant(_5, 0), 4)) = !13414888479744933802_usize;
Call(RET = fn12(Field::<u32>(Variant(_5, 0), 3), Move(_5), Move(_1.0), _10, _12, _8, _9, _12, _8, _12, _10, _1.2), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
place!(Field::<f64>(Variant(RET, 2), 1)) = 64_u8 as f64;
_1.0.fld0 = _10;
_21.fld0 = _19 > Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
place!(Field::<f64>(Variant(RET, 2), 1)) = 1587133482559994103_usize as f64;
place!(Field::<u32>(Variant(_5, 0), 3)) = 3483926528_u32 >> Field::<(u16, char, i8)>(Variant(RET, 2), 0).0;
_8 = _19 as isize;
Goto(bb6)
}
bb6 = {
_1.0.fld4 = 12511524534338491895_usize as i128;
_21 = Adt19 { fld0: _7,fld1: Field::<(u16, char, i8)>(Variant(RET, 2), 0).1,fld2: _9,fld3: Field::<(u16, char, i8)>(Variant(RET, 2), 0).2,fld4: _1.0.fld4 };
_23.0 = [Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3)];
_22 = 3578592214213870436_u64 as f32;
_24 = _1.0.fld2;
_21.fld0 = !_10;
_4 = &_21.fld3;
Goto(bb7)
}
bb7 = {
_21.fld1 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
place!(Field::<char>(Variant(_5, 0), 1)) = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
_1.1 = Move(_4);
place!(Field::<char>(Variant(_5, 0), 1)) = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
_25.1.fld0 = _7;
_1.0.fld3 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).2 ^ Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_6 = !_16;
Goto(bb8)
}
bb8 = {
_20 = &place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0));
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = Field::<u16>(Variant(_5, 0), 2) | Field::<u16>(Variant(_5, 0), 2);
_1.0.fld3 = (*_20).2;
_1.0.fld0 = _10 ^ _25.1.fld0;
_28 = !Field::<(u16, char, i8)>(Variant(RET, 2), 0).0;
_1.2 = _11 as isize;
_4 = &_25.1.fld3;
place!(Field::<u32>(Variant(RET, 2), 2)) = Field::<u32>(Variant(_5, 0), 3);
_25.1 = Move(_21);
_12 = _16;
_25.1.fld1 = _1.0.fld1;
_1.0.fld1 = _25.1.fld1;
_6 = _16 | _16;
_7 = _1.0.fld0;
_8 = -_12;
_21 = Adt19 { fld0: _11,fld1: _1.0.fld1,fld2: _8,fld3: (*_20).2,fld4: _25.1.fld4 };
_10 = _1.0.fld0;
Goto(bb9)
}
bb9 = {
_27 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_5, 0), 0)));
_12 = -_9;
_8 = _6;
(*_27) = 64_u8 >> Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_25.1.fld0 = _1.0.fld0;
_10 = _1.0.fld0 | _7;
place!(Field::<usize>(Variant(_5, 0), 4)) = !13009226515214921008_usize;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = _28;
_6 = Field::<u32>(Variant(RET, 2), 2) as isize;
_1.1 = &_21.fld3;
_4 = &_25.1.fld3;
_39 = (-342607461_i32);
match _39 {
0 => bb6,
1 => bb10,
2 => bb11,
340282366920938463463374607431425603995 => bb13,
_ => bb12
}
}
bb10 = {
_20 = &place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0));
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = Field::<u16>(Variant(_5, 0), 2) | Field::<u16>(Variant(_5, 0), 2);
_1.0.fld3 = (*_20).2;
_1.0.fld0 = _10 ^ _25.1.fld0;
_28 = !Field::<(u16, char, i8)>(Variant(RET, 2), 0).0;
_1.2 = _11 as isize;
_4 = &_25.1.fld3;
place!(Field::<u32>(Variant(RET, 2), 2)) = Field::<u32>(Variant(_5, 0), 3);
_25.1 = Move(_21);
_12 = _16;
_25.1.fld1 = _1.0.fld1;
_1.0.fld1 = _25.1.fld1;
_6 = _16 | _16;
_7 = _1.0.fld0;
_8 = -_12;
_21 = Adt19 { fld0: _11,fld1: _1.0.fld1,fld2: _8,fld3: (*_20).2,fld4: _25.1.fld4 };
_10 = _1.0.fld0;
Goto(bb9)
}
bb11 = {
_1.0 = Adt19 { fld0: _7,fld1: '\u{7146c}',fld2: _1.2,fld3: 12_i8,fld4: 123235471507091351230788896785344896716_i128 };
_1.0 = Adt19 { fld0: _11,fld1: '\u{7f730}',fld2: _12,fld3: (-94_i8),fld4: (-64417132475884303217364187517369945584_i128) };
_1.0.fld2 = _9;
_1.1 = &_1.0.fld3;
_14 = (-7673832613698295214_i64);
_5 = Adt33::Variant0 { fld0: 178_u8,fld1: _1.0.fld1,fld2: 21630_u16,fld3: 4013426468_u32,fld4: 7_usize };
_1.0.fld3 = _1.0.fld4 as i8;
place!(Field::<u16>(Variant(_5, 0), 2)) = 45516_u16;
SetDiscriminant(RET, 2);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)) = (Field::<u16>(Variant(_5, 0), 2), Field::<char>(Variant(_5, 0), 1), _1.0.fld3);
_5 = Adt33::Variant0 { fld0: 91_u8,fld1: _1.0.fld1,fld2: Field::<(u16, char, i8)>(Variant(RET, 2), 0).0,fld3: 1731657928_u32,fld4: 1_usize };
_9 = -_12;
place!(Field::<f64>(Variant(RET, 2), 1)) = (-108139927_i32) as f64;
_7 = _10;
_1.0.fld4 = 19645057058562715167717797067181042734_i128 << Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_1.0.fld0 = _10;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
Goto(bb3)
}
bb12 = {
_1.0.fld4 = 12511524534338491895_usize as i128;
_21 = Adt19 { fld0: _7,fld1: Field::<(u16, char, i8)>(Variant(RET, 2), 0).1,fld2: _9,fld3: Field::<(u16, char, i8)>(Variant(RET, 2), 0).2,fld4: _1.0.fld4 };
_23.0 = [Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3)];
_22 = 3578592214213870436_u64 as f32;
_24 = _1.0.fld2;
_21.fld0 = !_10;
_4 = &_21.fld3;
Goto(bb7)
}
bb13 = {
_1.0.fld4 = _25.1.fld4 >> (*_4);
_34.fld1 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
_33 = Field::<(u16, char, i8)>(Variant(RET, 2), 0);
match _39 {
0 => bb11,
1 => bb14,
340282366920938463463374607431425603995 => bb16,
_ => bb15
}
}
bb14 = {
_1.0 = Adt19 { fld0: _7,fld1: '\u{7146c}',fld2: _1.2,fld3: 12_i8,fld4: 123235471507091351230788896785344896716_i128 };
_1.0 = Adt19 { fld0: _11,fld1: '\u{7f730}',fld2: _12,fld3: (-94_i8),fld4: (-64417132475884303217364187517369945584_i128) };
_1.0.fld2 = _9;
_1.1 = &_1.0.fld3;
_14 = (-7673832613698295214_i64);
_5 = Adt33::Variant0 { fld0: 178_u8,fld1: _1.0.fld1,fld2: 21630_u16,fld3: 4013426468_u32,fld4: 7_usize };
_1.0.fld3 = _1.0.fld4 as i8;
place!(Field::<u16>(Variant(_5, 0), 2)) = 45516_u16;
SetDiscriminant(RET, 2);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)) = (Field::<u16>(Variant(_5, 0), 2), Field::<char>(Variant(_5, 0), 1), _1.0.fld3);
_5 = Adt33::Variant0 { fld0: 91_u8,fld1: _1.0.fld1,fld2: Field::<(u16, char, i8)>(Variant(RET, 2), 0).0,fld3: 1731657928_u32,fld4: 1_usize };
_9 = -_12;
place!(Field::<f64>(Variant(RET, 2), 1)) = (-108139927_i32) as f64;
_7 = _10;
_1.0.fld4 = 19645057058562715167717797067181042734_i128 << Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_1.0.fld0 = _10;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
Goto(bb3)
}
bb15 = {
_20 = &place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0));
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = Field::<u16>(Variant(_5, 0), 2) | Field::<u16>(Variant(_5, 0), 2);
_1.0.fld3 = (*_20).2;
_1.0.fld0 = _10 ^ _25.1.fld0;
_28 = !Field::<(u16, char, i8)>(Variant(RET, 2), 0).0;
_1.2 = _11 as isize;
_4 = &_25.1.fld3;
place!(Field::<u32>(Variant(RET, 2), 2)) = Field::<u32>(Variant(_5, 0), 3);
_25.1 = Move(_21);
_12 = _16;
_25.1.fld1 = _1.0.fld1;
_1.0.fld1 = _25.1.fld1;
_6 = _16 | _16;
_7 = _1.0.fld0;
_8 = -_12;
_21 = Adt19 { fld0: _11,fld1: _1.0.fld1,fld2: _8,fld3: (*_20).2,fld4: _25.1.fld4 };
_10 = _1.0.fld0;
Goto(bb9)
}
bb16 = {
SetDiscriminant(_5, 2);
_1.2 = _9 ^ _21.fld2;
_40 = -_39;
_21.fld0 = !_10;
_9 = -_21.fld2;
_21 = Move(_1.0);
place!(Field::<[u32; 7]>(Variant(_5, 2), 1)) = [Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2)];
place!(Field::<f64>(Variant(RET, 2), 1)) = _14 as f64;
_1.0.fld0 = _11 != _21.fld0;
_11 = !_10;
place!(Field::<u32>(Variant(RET, 2), 2)) = !2426897345_u32;
_34.fld4 = _21.fld4;
_1.0 = Adt19 { fld0: _7,fld1: _21.fld1,fld2: _6,fld3: _33.2,fld4: _34.fld4 };
_1 = (Move(_25.1), Move(_4), _24);
_13 = [_39];
_7 = _9 >= _6;
_41 = (Field::<[u32; 7]>(Variant(_5, 2), 1),);
_37 = &place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).2;
_3 = &(*_37);
_25.1.fld1 = _34.fld1;
_21.fld3 = (*_3);
_31 = _11;
_33 = Field::<(u16, char, i8)>(Variant(RET, 2), 0);
place!(Field::<i16>(Variant(_5, 2), 0)) = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1 as i16;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = !_28;
_34 = Adt19 { fld0: _31,fld1: _19,fld2: _24,fld3: _1.0.fld3,fld4: _21.fld4 };
_20 = &place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0));
_21.fld1 = (*_20).1;
Goto(bb17)
}
bb17 = {
place!(Field::<i16>(Variant(_5, 2), 0)) = _9 as i16;
place!(Field::<f64>(Variant(RET, 2), 1)) = Field::<i16>(Variant(_5, 2), 0) as f64;
_6 = -_1.0.fld2;
_25.1.fld3 = _1.0.fld3;
_33.0 = Field::<f64>(Variant(RET, 2), 1) as u16;
place!(Field::<u32>(Variant(RET, 2), 2)) = Field::<i16>(Variant(_5, 2), 0) as u32;
match _14 {
0 => bb9,
340282366920938463455700774818069916242 => bb18,
_ => bb10
}
}
bb18 = {
_25.1.fld1 = (*_20).1;
_24 = _1.0.fld2;
_25.1.fld2 = _1.0.fld2;
_30 = 11930532995907451932_u64 as isize;
place!(Field::<i16>(Variant(_5, 2), 0)) = _14 as i16;
_1.0.fld2 = -_8;
_25.1 = Move(_34);
_33.2 = _21.fld4 as i8;
place!(Field::<i8>(Variant(_5, 2), 3)) = (*_3) * _21.fld3;
match _39 {
0 => bb17,
1 => bb12,
2 => bb13,
3 => bb10,
340282366920938463463374607431425603995 => bb19,
_ => bb11
}
}
bb19 = {
_22 = Field::<f64>(Variant(RET, 2), 1) as f32;
_34.fld1 = _25.1.fld1;
place!(Field::<i8>(Variant(_5, 2), 3)) = _33.2;
_18 = &_49;
_46 = _33.0;
_20 = &(*_20);
_54.1.fld1 = _21.fld1;
_54.1.fld3 = Field::<f64>(Variant(RET, 2), 1) as i8;
_52 = _9 + _9;
_1.1 = Move(_3);
_1.0.fld1 = _34.fld1;
_28 = _39 as u16;
_54.1.fld4 = _21.fld4 | _21.fld4;
_34.fld0 = _25.1.fld0;
_41.0 = [Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2)];
_54.1 = Adt19 { fld0: _25.1.fld0,fld1: _33.1,fld2: _8,fld3: _25.1.fld3,fld4: _25.1.fld4 };
_31 = _10;
_20 = &(*_20);
_34.fld1 = _33.1;
match _14 {
0 => bb17,
340282366920938463455700774818069916242 => bb21,
_ => bb20
}
}
bb20 = {
_1.0.fld4 = 12511524534338491895_usize as i128;
_21 = Adt19 { fld0: _7,fld1: Field::<(u16, char, i8)>(Variant(RET, 2), 0).1,fld2: _9,fld3: Field::<(u16, char, i8)>(Variant(RET, 2), 0).2,fld4: _1.0.fld4 };
_23.0 = [Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3),Field::<u32>(Variant(_5, 0), 3)];
_22 = 3578592214213870436_u64 as f32;
_24 = _1.0.fld2;
_21.fld0 = !_10;
_4 = &_21.fld3;
Goto(bb7)
}
bb21 = {
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _25.1.fld1;
_11 = !_1.0.fld0;
_29.0 = &_49;
_36 = &(*_18);
_25.1 = Adt19 { fld0: _11,fld1: _19,fld2: _24,fld3: _33.2,fld4: _21.fld4 };
place!(Field::<f64>(Variant(RET, 2), 1)) = _21.fld4 as f64;
_34.fld4 = _25.1.fld4 + _54.1.fld4;
_21.fld1 = _33.1;
_54.1.fld3 = _25.1.fld2 as i8;
_45 = !221215854131415012219664059231543567330_u128;
_40 = _39;
_1.0.fld0 = _21.fld0;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).2 = -_21.fld3;
_58 = !Field::<u32>(Variant(RET, 2), 2);
_37 = &_33.2;
_1.0.fld1 = _25.1.fld1;
_56 = !119_u8;
_9 = _58 as isize;
_51 = _54.1.fld0;
_1.2 = _52 + _9;
_54.1 = Adt19 { fld0: _31,fld1: Field::<(u16, char, i8)>(Variant(RET, 2), 0).1,fld2: _8,fld3: Field::<(u16, char, i8)>(Variant(RET, 2), 0).2,fld4: _21.fld4 };
_56 = 188_u8 | 127_u8;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _33.1;
place!(Field::<i8>(Variant(_5, 2), 3)) = -_25.1.fld3;
_20 = &_33;
match _39 {
340282366920938463463374607431425603995 => bb23,
_ => bb22
}
}
bb22 = {
_1.0 = Adt19 { fld0: _7,fld1: '\u{7146c}',fld2: _1.2,fld3: 12_i8,fld4: 123235471507091351230788896785344896716_i128 };
_1.0 = Adt19 { fld0: _11,fld1: '\u{7f730}',fld2: _12,fld3: (-94_i8),fld4: (-64417132475884303217364187517369945584_i128) };
_1.0.fld2 = _9;
_1.1 = &_1.0.fld3;
_14 = (-7673832613698295214_i64);
_5 = Adt33::Variant0 { fld0: 178_u8,fld1: _1.0.fld1,fld2: 21630_u16,fld3: 4013426468_u32,fld4: 7_usize };
_1.0.fld3 = _1.0.fld4 as i8;
place!(Field::<u16>(Variant(_5, 0), 2)) = 45516_u16;
SetDiscriminant(RET, 2);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)) = (Field::<u16>(Variant(_5, 0), 2), Field::<char>(Variant(_5, 0), 1), _1.0.fld3);
_5 = Adt33::Variant0 { fld0: 91_u8,fld1: _1.0.fld1,fld2: Field::<(u16, char, i8)>(Variant(RET, 2), 0).0,fld3: 1731657928_u32,fld4: 1_usize };
_9 = -_12;
place!(Field::<f64>(Variant(RET, 2), 1)) = (-108139927_i32) as f64;
_7 = _10;
_1.0.fld4 = 19645057058562715167717797067181042734_i128 << Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_1.0.fld0 = _10;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
Goto(bb3)
}
bb23 = {
_42 = _58 as f64;
_8 = -_12;
_48 = &_3;
_23 = (_41.0,);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).2 = _45 as i8;
_59 = _22;
_37 = &(*_37);
_67.1 = [_56,_56,_56,_56,_56,_56];
_25.1.fld1 = (*_20).1;
_23 = _41;
match _14 {
0 => bb24,
1 => bb25,
340282366920938463455700774818069916242 => bb27,
_ => bb26
}
}
bb24 = {
_1.0 = Adt19 { fld0: _7,fld1: '\u{7146c}',fld2: _1.2,fld3: 12_i8,fld4: 123235471507091351230788896785344896716_i128 };
_1.0 = Adt19 { fld0: _11,fld1: '\u{7f730}',fld2: _12,fld3: (-94_i8),fld4: (-64417132475884303217364187517369945584_i128) };
_1.0.fld2 = _9;
_1.1 = &_1.0.fld3;
_14 = (-7673832613698295214_i64);
_5 = Adt33::Variant0 { fld0: 178_u8,fld1: _1.0.fld1,fld2: 21630_u16,fld3: 4013426468_u32,fld4: 7_usize };
_1.0.fld3 = _1.0.fld4 as i8;
place!(Field::<u16>(Variant(_5, 0), 2)) = 45516_u16;
SetDiscriminant(RET, 2);
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)) = (Field::<u16>(Variant(_5, 0), 2), Field::<char>(Variant(_5, 0), 1), _1.0.fld3);
_5 = Adt33::Variant0 { fld0: 91_u8,fld1: _1.0.fld1,fld2: Field::<(u16, char, i8)>(Variant(RET, 2), 0).0,fld3: 1731657928_u32,fld4: 1_usize };
_9 = -_12;
place!(Field::<f64>(Variant(RET, 2), 1)) = (-108139927_i32) as f64;
_7 = _10;
_1.0.fld4 = 19645057058562715167717797067181042734_i128 << Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_1.0.fld0 = _10;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<char>(Variant(_5, 0), 1)) = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).1 = _1.0.fld1;
Goto(bb3)
}
bb25 = {
_21.fld1 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
place!(Field::<char>(Variant(_5, 0), 1)) = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
_1.1 = Move(_4);
place!(Field::<char>(Variant(_5, 0), 1)) = Field::<(u16, char, i8)>(Variant(RET, 2), 0).1;
_25.1.fld0 = _7;
_1.0.fld3 = Field::<(u16, char, i8)>(Variant(RET, 2), 0).2 ^ Field::<(u16, char, i8)>(Variant(RET, 2), 0).2;
_6 = !_16;
Goto(bb8)
}
bb26 = {
_20 = &place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0));
place!(Field::<(u16, char, i8)>(Variant(RET, 2), 0)).0 = Field::<u16>(Variant(_5, 0), 2) | Field::<u16>(Variant(_5, 0), 2);
_1.0.fld3 = (*_20).2;
_1.0.fld0 = _10 ^ _25.1.fld0;
_28 = !Field::<(u16, char, i8)>(Variant(RET, 2), 0).0;
_1.2 = _11 as isize;
_4 = &_25.1.fld3;
place!(Field::<u32>(Variant(RET, 2), 2)) = Field::<u32>(Variant(_5, 0), 3);
_25.1 = Move(_21);
_12 = _16;
_25.1.fld1 = _1.0.fld1;
_1.0.fld1 = _25.1.fld1;
_6 = _16 | _16;
_7 = _1.0.fld0;
_8 = -_12;
_21 = Adt19 { fld0: _11,fld1: _1.0.fld1,fld2: _8,fld3: (*_20).2,fld4: _25.1.fld4 };
_10 = _1.0.fld0;
Goto(bb9)
}
bb27 = {
_21.fld1 = _1.0.fld1;
_66.0 = [Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),_58,Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),_58,Field::<u32>(Variant(RET, 2), 2)];
_1.0.fld1 = _33.1;
place!(Field::<[u32; 7]>(Variant(_5, 2), 1)) = [_58,Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),Field::<u32>(Variant(RET, 2), 2),_58,_58,_58];
_43 = [_45,_45,_45,_45,_45,_45];
_59 = _22 + _22;
_14 = _56 as i64;
_37 = &(*_20).2;
_30 = _14 as isize;
_67.0 = core::ptr::addr_of!(_45);
_68 = Field::<u32>(Variant(RET, 2), 2) as i16;
_33.1 = _34.fld1;
place!(Field::<u32>(Variant(RET, 2), 2)) = _58;
_49 = 1542325842106307834_u64;
_33.0 = _46;
_69 = !Field::<u32>(Variant(RET, 2), 2);
place!(Field::<i16>(Variant(_5, 2), 0)) = _68;
_18 = &_49;
_40 = -_39;
_63 = [_11,_54.1.fld0,_7];
_33 = Field::<(u16, char, i8)>(Variant(RET, 2), 0);
_34.fld0 = !_1.0.fld0;
Goto(bb28)
}
bb28 = {
_54.0 = core::ptr::addr_of_mut!(_45);
_4 = &_33.2;
place!(Field::<*mut u128>(Variant(RET, 2), 3)) = Move(_54.0);
_73 = Adt25::Variant3 { fld0: _56,fld1: _45,fld2: _22,fld3: _41.0 };
_20 = &_33;
_1.0.fld2 = _54.1.fld2;
_34 = Move(_25.1);
Goto(bb29)
}
bb29 = {
Call(_74 = dump_var(11_usize, 49_usize, Move(_49), 41_usize, Move(_41), 43_usize, Move(_43), 39_usize, Move(_39)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_74 = dump_var(11_usize, 31_usize, Move(_31), 58_usize, Move(_58), 45_usize, Move(_45), 56_usize, Move(_56)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_74 = dump_var(11_usize, 9_usize, Move(_9), 10_usize, Move(_10), 24_usize, Move(_24), 33_usize, Move(_33)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_74 = dump_var(11_usize, 63_usize, Move(_63), 52_usize, Move(_52), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u32,mut _2: Adt33,mut _3: Adt19,mut _4: bool,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: bool,mut _12: isize) -> Adt56 {
mir! {
type RET = Adt56;
let _13: *const u128;
let _14: &'static i8;
let _15: &'static (u16, char, i8);
let _16: ([i16; 8],);
let _17: bool;
let _18: i32;
let _19: f64;
let _20: &'static &'static i8;
let _21: (*const u128, [u8; 6]);
let _22: (Adt56,);
let _23: i64;
let _24: usize;
let _25: &'static (&'static u64,);
let _26: [i32; 1];
let _27: Adt56;
let _28: i64;
let _29: Adt77;
let _30: f64;
let _31: *mut &'static (u16, char, i8);
let _32: &'static *const f32;
let _33: i32;
let _34: (*const f32,);
let _35: isize;
let _36: f64;
let _37: i128;
let _38: [i32; 1];
let _39: isize;
let _40: isize;
let _41: [i8; 8];
let _42: isize;
let _43: Adt19;
let _44: i128;
let _45: [i8; 4];
let _46: i8;
let _47: [u64; 4];
let _48: [i16; 8];
let _49: bool;
let _50: i8;
let _51: Adt77;
let _52: (bool, &'static u128);
let _53: isize;
let _54: &'static f64;
let _55: &'static f64;
let _56: [i64; 8];
let _57: f64;
let _58: bool;
let _59: f64;
let _60: i64;
let _61: (*const u128, [u8; 6]);
let _62: [u32; 7];
let _63: isize;
let _64: [char; 4];
let _65: Adt33;
let _66: [isize; 1];
let _67: *mut u128;
let _68: isize;
let _69: u32;
let _70: [i64; 1];
let _71: char;
let _72: Adt56;
let _73: &'static Adt25;
let _74: Adt19;
let _75: f32;
let _76: (bool, &'static u128);
let _77: Adt73;
let _78: (u16, char, i8);
let _79: isize;
let _80: ();
let _81: ();
{
place!(Field::<u32>(Variant(_2, 0), 3)) = _1 & _1;
match _3.fld4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
55703075892936460500921767760556385567 => bb7,
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
_10 = (-9235_i16) as isize;
place!(Field::<u32>(Variant(_2, 0), 3)) = !_1;
_7 = -_6;
place!(Field::<u32>(Variant(_2, 0), 3)) = 274853375554438673105567268727877581989_u128 as u32;
_7 = (-11610_i16) as isize;
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_9 = _3.fld2;
_9 = -_6;
_3.fld2 = !_8;
_8 = _5;
_7 = Field::<u8>(Variant(_2, 0), 0) as isize;
_3.fld0 = _4;
place!(Field::<u16>(Variant(_2, 0), 2)) = _1 as u16;
_8 = _6;
_1 = Field::<u32>(Variant(_2, 0), 3);
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_3.fld4 = (-22898282452216242484621072455179765242_i128) ^ 64879909161384682101635305141298766594_i128;
place!(Field::<usize>(Variant(_2, 0), 4)) = !1_usize;
place!(Field::<u16>(Variant(_2, 0), 2)) = _11 as u16;
_12 = _9;
_3.fld0 = !_11;
_7 = !_3.fld2;
Goto(bb8)
}
bb8 = {
place!(Field::<usize>(Variant(_2, 0), 4)) = !0_usize;
_3 = Adt19 { fld0: _4,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _8,fld3: 12_i8,fld4: (-101724111309545235533497991564293285660_i128) };
_4 = _7 > _6;
place!(Field::<u8>(Variant(_2, 0), 0)) = _3.fld3 as u8;
_12 = -_6;
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_3.fld0 = _4 ^ _11;
_12 = _3.fld2;
_3.fld2 = _9 + _9;
_8 = _9 & _12;
_3.fld4 = 95969483947742125410090687807601093056_i128;
_3.fld3 = 19_i8;
_3.fld4 = 70238196503266280227323333420658267376_i128 << _12;
_11 = Field::<u8>(Variant(_2, 0), 0) > Field::<u8>(Variant(_2, 0), 0);
_10 = _12;
_17 = _5 <= _8;
_14 = &_3.fld3;
match (*_14) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb6,
6 => bb9,
19 => bb11,
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
_1 = _17 as u32;
place!(Field::<u32>(Variant(_2, 0), 3)) = 24287_i16 as u32;
_7 = _5 & _3.fld2;
_19 = (*_14) as f64;
_16.0 = [(-21258_i16),7642_i16,571_i16,19279_i16,(-21572_i16),(-10612_i16),(-683_i16),25498_i16];
_3.fld3 = (-8_i8) << _5;
Goto(bb12)
}
bb12 = {
_18 = (-1194058280_i32);
_23 = _19 as i64;
_3 = Adt19 { fld0: _4,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _12,fld3: (-65_i8),fld4: 46571632302212224544660122028664663914_i128 };
_1 = Field::<u32>(Variant(_2, 0), 3) ^ Field::<u32>(Variant(_2, 0), 3);
_3.fld0 = _17;
Call(_13 = fn13(_3.fld2, _12, _3.fld2, _4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3.fld0 = _4;
place!(Field::<u32>(Variant(_2, 0), 3)) = _1 << Field::<u8>(Variant(_2, 0), 0);
_3.fld1 = Field::<char>(Variant(_2, 0), 1);
_21.1 = [Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0)];
_3.fld2 = Field::<u8>(Variant(_2, 0), 0) as isize;
_18 = 77231410_i32 * 509861496_i32;
_21.0 = Move(_13);
match _3.fld3 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463463374607431768211391 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_24 = _23 as usize;
_3.fld1 = Field::<char>(Variant(_2, 0), 1);
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_3.fld0 = _17 ^ _4;
_5 = Field::<u8>(Variant(_2, 0), 0) as isize;
_4 = !_11;
_20 = &_14;
_5 = _3.fld2 ^ _7;
_16.0 = [31948_i16,(-11447_i16),13040_i16,14474_i16,20915_i16,(-5070_i16),(-12837_i16),16265_i16];
_26 = [_18];
_3.fld4 = -889743178330278855227289384652090716_i128;
_6 = _7;
SetDiscriminant(_2, 0);
_17 = !_4;
place!(Field::<usize>(Variant(_2, 0), 4)) = !_24;
place!(Field::<u16>(Variant(_2, 0), 2)) = _19 as u16;
_3.fld3 = (-328_i16) as i8;
_1 = !752488363_u32;
Goto(bb16)
}
bb16 = {
_4 = !_17;
_13 = Move(_21.0);
_26 = [_18];
_24 = Field::<usize>(Variant(_2, 0), 4) ^ Field::<usize>(Variant(_2, 0), 4);
Call(_3.fld4 = fn14(Move(_20), _3.fld2, _10, _8, _10, _11), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_3.fld2 = _12 >> _7;
_24 = Field::<usize>(Variant(_2, 0), 4);
_28 = Field::<usize>(Variant(_2, 0), 4) as i64;
place!(Field::<u8>(Variant(_2, 0), 0)) = !132_u8;
place!(Field::<u16>(Variant(_2, 0), 2)) = 28902_u16 << _3.fld4;
_21.0 = Move(_13);
_14 = &_3.fld3;
_3.fld1 = Field::<char>(Variant(_2, 0), 1);
_13 = Move(_21.0);
_14 = &(*_14);
_3.fld3 = -82_i8;
place!(Field::<u16>(Variant(_2, 0), 2)) = !7002_u16;
_4 = _3.fld2 != _10;
_4 = !_17;
_21.0 = Move(_13);
place!(Field::<u8>(Variant(_2, 0), 0)) = _3.fld3 as u8;
place!(Field::<usize>(Variant(_2, 0), 4)) = !_24;
_3.fld3 = Field::<usize>(Variant(_2, 0), 4) as i8;
place!(Field::<u8>(Variant(_2, 0), 0)) = !40_u8;
_35 = _12;
_21.1 = [Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0),Field::<u8>(Variant(_2, 0), 0)];
Goto(bb18)
}
bb18 = {
place!(Field::<u8>(Variant(_2, 0), 0)) = 212_u8;
_24 = Field::<usize>(Variant(_2, 0), 4) & Field::<usize>(Variant(_2, 0), 4);
_11 = _17 ^ _3.fld0;
_14 = &_3.fld3;
_3 = Adt19 { fld0: _4,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _6,fld3: 6_i8,fld4: (-89222582598861271425619722820648086776_i128) };
_26 = [_18];
_26 = [_18];
match _3.fld3 {
0 => bb13,
1 => bb5,
2 => bb16,
6 => bb19,
_ => bb17
}
}
bb19 = {
_17 = _5 < _3.fld2;
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_26 = [_18];
_20 = &_14;
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_1 = 2949363789_u32;
Goto(bb20)
}
bb20 = {
Goto(bb21)
}
bb21 = {
_3.fld3 = Field::<u8>(Variant(_2, 0), 0) as i8;
_5 = _8 | _8;
_32 = &_34.0;
place!(Field::<u32>(Variant(_2, 0), 3)) = !_1;
match _3.fld4 {
0 => bb20,
1 => bb11,
2 => bb9,
3 => bb8,
4 => bb17,
5 => bb6,
6 => bb13,
251059784322077192037754884611120124680 => bb23,
_ => bb22
}
}
bb22 = {
Return()
}
bb23 = {
place!(Field::<u16>(Variant(_2, 0), 2)) = 42486_u16;
_37 = !_3.fld4;
match _3.fld4 {
0 => bb1,
1 => bb15,
2 => bb19,
251059784322077192037754884611120124680 => bb24,
_ => bb18
}
}
bb24 = {
_3.fld2 = -_10;
_41 = [_3.fld3,_3.fld3,_3.fld3,_3.fld3,_3.fld3,_3.fld3,_3.fld3,_3.fld3];
_3.fld2 = _9 & _8;
place!(Field::<usize>(Variant(_2, 0), 4)) = _24;
_40 = _12;
_32 = &(*_32);
_33 = !_18;
_13 = Move(_21.0);
SetDiscriminant(_2, 0);
_36 = _19;
_9 = _11 as isize;
_35 = 283721771202099634044814357580586967033_u128 as isize;
match _3.fld4 {
251059784322077192037754884611120124680 => bb26,
_ => bb25
}
}
bb25 = {
place!(Field::<u16>(Variant(_2, 0), 2)) = 42486_u16;
_37 = !_3.fld4;
match _3.fld4 {
0 => bb1,
1 => bb15,
2 => bb19,
251059784322077192037754884611120124680 => bb24,
_ => bb18
}
}
bb26 = {
_3.fld3 = 48044_u16 as i8;
_10 = _3.fld1 as isize;
_42 = _12;
place!(Field::<u32>(Variant(_2, 0), 3)) = _4 as u32;
_21.0 = Move(_13);
_43.fld4 = -_37;
place!(Field::<char>(Variant(_2, 0), 1)) = _3.fld1;
_43.fld0 = !_11;
_42 = _9 - _5;
_43.fld3 = _3.fld3;
_43.fld0 = _3.fld0;
_2 = Adt33::Variant0 { fld0: 244_u8,fld1: _3.fld1,fld2: 25558_u16,fld3: _1,fld4: _24 };
_43 = Adt19 { fld0: _4,fld1: _3.fld1,fld2: _42,fld3: _3.fld3,fld4: _37 };
_8 = _17 as isize;
_13 = Move(_21.0);
_50 = _36 as i8;
_43.fld1 = _3.fld1;
_5 = -_3.fld2;
_48 = [7690_i16,15215_i16,(-17557_i16),(-30838_i16),(-18849_i16),19415_i16,13555_i16,(-7380_i16)];
_3.fld2 = _5 << _40;
_43.fld0 = !_3.fld0;
_38 = [_18];
_30 = 185926519018696734764061343465967473634_u128 as f64;
_35 = -_5;
_31 = core::ptr::addr_of_mut!(_15);
_24 = !Field::<usize>(Variant(_2, 0), 4);
Goto(bb27)
}
bb27 = {
_43 = Adt19 { fld0: _17,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _40,fld3: _50,fld4: _37 };
_14 = &_43.fld3;
_8 = !_9;
_40 = _6 + _12;
place!(Field::<u16>(Variant(_2, 0), 2)) = 34626_u16;
_4 = !_3.fld0;
_17 = _3.fld0;
_33 = _28 as i32;
_24 = !Field::<usize>(Variant(_2, 0), 4);
_5 = _35 * _6;
place!(Field::<u16>(Variant(_2, 0), 2)) = 34763_u16 ^ 26162_u16;
place!(Field::<u16>(Variant(_2, 0), 2)) = 44410_u16 << _3.fld2;
_16.0 = [2673_i16,(-3796_i16),(-7505_i16),28585_i16,22840_i16,14193_i16,(-608_i16),(-7781_i16)];
_47 = [4544742969011640558_u64,3915309544782697563_u64,13474317346055294875_u64,5280545832582050427_u64];
_40 = 16480780124209666265_u64 as isize;
_23 = _28;
_3.fld1 = _43.fld1;
_46 = 100637046928761635458459127152409495019_u128 as i8;
_45 = [(*_14),_50,_50,_43.fld3];
_16 = (_48,);
_3.fld4 = _43.fld1 as i128;
_21.0 = Move(_13);
_13 = Move(_21.0);
place!(Field::<char>(Variant(_2, 0), 1)) = _43.fld1;
_29 = Adt77::Variant0 { fld0: _16.0,fld1: _43.fld4 };
_9 = _18 as isize;
_19 = _36;
_43.fld2 = _8 << _42;
match Field::<u32>(Variant(_2, 0), 3) {
0 => bb1,
1 => bb12,
2 => bb3,
3 => bb21,
4 => bb22,
2949363789 => bb29,
_ => bb28
}
}
bb28 = {
Return()
}
bb29 = {
_3.fld4 = !Field::<i128>(Variant(_29, 0), 1);
_18 = _30 as i32;
_3.fld4 = _28 as i128;
_48 = _16.0;
match _1 {
0 => bb13,
1 => bb22,
2 => bb26,
3 => bb30,
4 => bb31,
2949363789 => bb33,
_ => bb32
}
}
bb30 = {
place!(Field::<u8>(Variant(_2, 0), 0)) = 212_u8;
_24 = Field::<usize>(Variant(_2, 0), 4) & Field::<usize>(Variant(_2, 0), 4);
_11 = _17 ^ _3.fld0;
_14 = &_3.fld3;
_3 = Adt19 { fld0: _4,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _6,fld3: 6_i8,fld4: (-89222582598861271425619722820648086776_i128) };
_26 = [_18];
_26 = [_18];
match _3.fld3 {
0 => bb13,
1 => bb5,
2 => bb16,
6 => bb19,
_ => bb17
}
}
bb31 = {
_43 = Adt19 { fld0: _17,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _40,fld3: _50,fld4: _37 };
_14 = &_43.fld3;
_8 = !_9;
_40 = _6 + _12;
place!(Field::<u16>(Variant(_2, 0), 2)) = 34626_u16;
_4 = !_3.fld0;
_17 = _3.fld0;
_33 = _28 as i32;
_24 = !Field::<usize>(Variant(_2, 0), 4);
_5 = _35 * _6;
place!(Field::<u16>(Variant(_2, 0), 2)) = 34763_u16 ^ 26162_u16;
place!(Field::<u16>(Variant(_2, 0), 2)) = 44410_u16 << _3.fld2;
_16.0 = [2673_i16,(-3796_i16),(-7505_i16),28585_i16,22840_i16,14193_i16,(-608_i16),(-7781_i16)];
_47 = [4544742969011640558_u64,3915309544782697563_u64,13474317346055294875_u64,5280545832582050427_u64];
_40 = 16480780124209666265_u64 as isize;
_23 = _28;
_3.fld1 = _43.fld1;
_46 = 100637046928761635458459127152409495019_u128 as i8;
_45 = [(*_14),_50,_50,_43.fld3];
_16 = (_48,);
_3.fld4 = _43.fld1 as i128;
_21.0 = Move(_13);
_13 = Move(_21.0);
place!(Field::<char>(Variant(_2, 0), 1)) = _43.fld1;
_29 = Adt77::Variant0 { fld0: _16.0,fld1: _43.fld4 };
_9 = _18 as isize;
_19 = _36;
_43.fld2 = _8 << _42;
match Field::<u32>(Variant(_2, 0), 3) {
0 => bb1,
1 => bb12,
2 => bb3,
3 => bb21,
4 => bb22,
2949363789 => bb29,
_ => bb28
}
}
bb32 = {
Return()
}
bb33 = {
_4 = _3.fld0;
_41 = [_50,(*_14),_3.fld3,_46,_46,(*_14),(*_14),_43.fld3];
_3 = Adt19 { fld0: _11,fld1: _43.fld1,fld2: _6,fld3: _46,fld4: _37 };
_54 = &_30;
place!(Field::<usize>(Variant(_2, 0), 4)) = !_24;
_54 = &_36;
SetDiscriminant(_29, 3);
_26 = _38;
_43.fld1 = _3.fld1;
place!(Field::<u8>(Variant(_2, 0), 0)) = !112_u8;
_46 = !_3.fld3;
_55 = &(*_54);
_20 = &_14;
_49 = _3.fld0 > _4;
_3 = Move(_43);
place!(Field::<u16>(Variant(_2, 0), 2)) = 48514_u16 | 1495_u16;
_24 = !Field::<usize>(Variant(_2, 0), 4);
_43 = Adt19 { fld0: _49,fld1: Field::<char>(Variant(_2, 0), 1),fld2: _12,fld3: _50,fld4: _3.fld4 };
_36 = _30 - _30;
_36 = _24 as f64;
_33 = _18;
place!(Field::<u32>(Variant(_2, 0), 3)) = _30 as u32;
_32 = &(*_32);
_36 = -_30;
Goto(bb34)
}
bb34 = {
_42 = _35 ^ _35;
_40 = !_43.fld2;
place!(Field::<f64>(Variant(_29, 3), 4)) = _30;
place!(Field::<[u32; 5]>(Variant(_29, 3), 1)) = [_1,Field::<u32>(Variant(_2, 0), 3),_1,_1,_1];
_14 = &_46;
place!(Field::<*const u128>(Variant(_29, 3), 0)) = Move(_13);
_45 = [(*_14),(*_14),_46,_43.fld3];
_52.0 = _49 | _3.fld0;
_35 = _40 << _37;
_32 = &(*_32);
place!(Field::<u32>(Variant(_2, 0), 3)) = _1 ^ _1;
_20 = &_14;
_55 = &place!(Field::<f64>(Variant(_29, 3), 4));
place!(Field::<u16>(Variant(_2, 0), 2)) = _1 as u16;
_1 = Field::<u32>(Variant(_2, 0), 3);
_43.fld2 = _42 | _7;
_8 = _3.fld2 << _3.fld4;
_48 = [(-5093_i16),1913_i16,13217_i16,(-22946_i16),(-31628_i16),(-24350_i16),13844_i16,(-23723_i16)];
_30 = -(*_55);
Call(_3.fld3 = fn15(Move(_14), _52.0, _5, Move(_55), _11, _12, _49), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
_55 = &_19;
_3.fld1 = Field::<char>(Variant(_2, 0), 1);
_40 = -_35;
_51 = Adt77::Variant0 { fld0: _48,fld1: _37 };
SetDiscriminant(_2, 0);
_33 = _18 ^ _18;
_21.0 = Move(Field::<*const u128>(Variant(_29, 3), 0));
place!(Field::<char>(Variant(_2, 0), 1)) = _43.fld1;
_8 = -_43.fld2;
_37 = Field::<i128>(Variant(_51, 0), 1);
place!(Field::<i128>(Variant(_29, 3), 3)) = !_3.fld4;
_18 = _17 as i32;
_1 = _43.fld4 as u32;
_32 = &(*_32);
place!(Field::<u16>(Variant(_2, 0), 2)) = _18 as u16;
_45 = [_43.fld3,_3.fld3,_43.fld3,_43.fld3];
_36 = Field::<f64>(Variant(_29, 3), 4);
place!(Field::<[u32; 5]>(Variant(_29, 3), 1)) = [_1,_1,_1,_1,_1];
Goto(bb36)
}
bb36 = {
_31 = core::ptr::addr_of_mut!((*_31));
_52.0 = _43.fld2 <= _8;
_12 = _35 ^ _43.fld2;
_54 = Move(_55);
_41 = [_3.fld3,_46,_43.fld3,_3.fld3,_46,_3.fld3,_50,_43.fld3];
_46 = !_43.fld3;
_58 = _52.0;
_13 = Move(_21.0);
Goto(bb37)
}
bb37 = {
SetDiscriminant(_51, 2);
_52.1 = &place!(Field::<u128>(Variant(_51, 2), 7));
place!(Field::<u16>(Variant(_51, 2), 6)) = !Field::<u16>(Variant(_2, 0), 2);
_32 = &_34.0;
_19 = _36 * _30;
place!(Field::<u128>(Variant(_51, 2), 7)) = !126686736166155721667308705194732507724_u128;
_61.1 = [157_u8,155_u8,49_u8,204_u8,190_u8,109_u8];
Call(place!(Field::<bool>(Variant(_51, 2), 0)) = fn16(_35, _8, _43.fld2), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
_11 = !Field::<bool>(Variant(_51, 2), 0);
place!(Field::<(char, i64, Adt19, [u8; 6])>(Variant(_51, 2), 2)).2 = Move(_43);
_33 = _50 as i32;
_51 = Adt77::Variant0 { fld0: _16.0,fld1: Field::<i128>(Variant(_29, 3), 3) };
_32 = &(*_32);
_3.fld2 = Field::<u16>(Variant(_2, 0), 2) as isize;
SetDiscriminant(_51, 3);
_57 = Field::<f64>(Variant(_29, 3), 4);
_38 = _26;
_5 = Field::<f64>(Variant(_29, 3), 4) as isize;
place!(Field::<i128>(Variant(_51, 3), 3)) = _3.fld4 * Field::<i128>(Variant(_29, 3), 3);
_46 = !_50;
place!(Field::<*const u128>(Variant(_51, 3), 0)) = Move(_13);
_66 = [_35];
_36 = Field::<u16>(Variant(_2, 0), 2) as f64;
place!(Field::<*const u128>(Variant(_29, 3), 0)) = Move(Field::<*const u128>(Variant(_51, 3), 0));
_58 = !_4;
_13 = Move(Field::<*const u128>(Variant(_29, 3), 0));
_2 = Adt33::Variant0 { fld0: 246_u8,fld1: _3.fld1,fld2: 26967_u16,fld3: _1,fld4: _24 };
place!(Field::<[u32; 5]>(Variant(_29, 3), 1)) = [Field::<u32>(Variant(_2, 0), 3),_1,_1,Field::<u32>(Variant(_2, 0), 3),_1];
place!(Field::<[u32; 5]>(Variant(_29, 3), 1)) = [_1,_1,Field::<u32>(Variant(_2, 0), 3),_1,_1];
_16.0 = _48;
Call(_43.fld1 = fn17(), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
_69 = !Field::<u32>(Variant(_2, 0), 3);
_21 = (Move(_13), _61.1);
place!(Field::<*const u128>(Variant(_51, 3), 0)) = Move(_21.0);
place!(Field::<[bool; 3]>(Variant(_29, 3), 2)) = [_58,_49,_11];
_65 = Adt33::Variant0 { fld0: 36_u8,fld1: _43.fld1,fld2: 56292_u16,fld3: _69,fld4: Field::<usize>(Variant(_2, 0), 4) };
_43.fld3 = _3.fld3;
_36 = _30;
place!(Field::<f64>(Variant(_51, 3), 4)) = _19 - Field::<f64>(Variant(_29, 3), 4);
_21.1 = [240_u8,189_u8,229_u8,84_u8,79_u8,77_u8];
place!(Field::<u8>(Variant(_65, 0), 0)) = _57 as u8;
place!(Field::<[u32; 5]>(Variant(_51, 3), 1)) = Field::<[u32; 5]>(Variant(_29, 3), 1);
_70 = [_28];
_64 = [Field::<char>(Variant(_65, 0), 1),Field::<char>(Variant(_2, 0), 1),_43.fld1,Field::<char>(Variant(_65, 0), 1)];
_39 = _7 - _8;
_32 = &_34.0;
place!(Field::<f64>(Variant(_29, 3), 4)) = _36 * _36;
_3.fld3 = !_50;
_21 = (Move(Field::<*const u128>(Variant(_51, 3), 0)), _61.1);
_62 = [Field::<u32>(Variant(_2, 0), 3),_1,Field::<u32>(Variant(_2, 0), 3),_1,_1,_69,_69];
place!(Field::<f64>(Variant(_51, 3), 4)) = _36 - Field::<f64>(Variant(_29, 3), 4);
place!(Field::<f64>(Variant(_51, 3), 4)) = _57 - _36;
_19 = -_57;
_65 = Adt33::Variant0 { fld0: 244_u8,fld1: Field::<char>(Variant(_2, 0), 1),fld2: 48834_u16,fld3: _69,fld4: Field::<usize>(Variant(_2, 0), 4) };
_3.fld4 = 176443794080863047129577847821512367555_u128 as i128;
_16 = (_48,);
_51 = Adt77::Variant3 { fld0: Move(_21.0),fld1: Field::<[u32; 5]>(Variant(_29, 3), 1),fld2: Field::<[bool; 3]>(Variant(_29, 3), 2),fld3: _37,fld4: _57 };
_23 = -_28;
_26 = [_18];
_62 = [_69,_69,_1,Field::<u32>(Variant(_2, 0), 3),_1,_1,Field::<u32>(Variant(_2, 0), 3)];
_40 = _3.fld2 & _42;
Goto(bb40)
}
bb40 = {
_46 = -_3.fld3;
_26 = [_18];
SetDiscriminant(_51, 0);
_10 = _12 * _12;
_21.1 = [203_u8,20_u8,14_u8,191_u8,195_u8,213_u8];
place!(Field::<u32>(Variant(_65, 0), 3)) = Field::<u32>(Variant(_2, 0), 3);
_55 = &place!(Field::<f64>(Variant(_29, 3), 4));
place!(Field::<i128>(Variant(_29, 3), 3)) = _37 & _37;
_45 = [_46,_50,_43.fld3,_50];
_51 = Adt77::Variant0 { fld0: _16.0,fld1: Field::<i128>(Variant(_29, 3), 3) };
_23 = _28;
_65 = Adt33::Variant0 { fld0: 205_u8,fld1: Field::<char>(Variant(_2, 0), 1),fld2: 63763_u16,fld3: _1,fld4: _24 };
_51 = Adt77::Variant0 { fld0: _48,fld1: Field::<i128>(Variant(_29, 3), 3) };
_19 = _36;
_3.fld2 = _6;
_2 = Adt33::Variant0 { fld0: 55_u8,fld1: Field::<char>(Variant(_65, 0), 1),fld2: 46353_u16,fld3: Field::<u32>(Variant(_65, 0), 3),fld4: _24 };
_56 = [_28,_28,_28,_23,_28,_23,_28,_28];
_2 = Adt33::Variant0 { fld0: 29_u8,fld1: _3.fld1,fld2: 32691_u16,fld3: _1,fld4: Field::<usize>(Variant(_65, 0), 4) };
SetDiscriminant(_51, 1);
_19 = (-30472_i16) as f64;
_59 = _43.fld3 as f64;
_56 = [_23,_23,_28,_23,_28,_23,_28,_28];
Goto(bb41)
}
bb41 = {
_43.fld4 = -Field::<i128>(Variant(_29, 3), 3);
place!(Field::<u16>(Variant(_65, 0), 2)) = 52487_u16 * 32048_u16;
_64 = [_3.fld1,_3.fld1,Field::<char>(Variant(_2, 0), 1),_43.fld1];
_68 = !_39;
_59 = (*_55);
_54 = Move(_55);
_75 = _18 as f32;
Goto(bb42)
}
bb42 = {
_6 = !_40;
_27 = Adt56::Variant3 { fld0: _56,fld1: 1538_i16 };
_42 = _3.fld2;
_20 = &_14;
_42 = !_3.fld2;
place!(Field::<[bool; 3]>(Variant(_29, 3), 2)) = [_3.fld0,_3.fld0,_4];
_35 = _3.fld0 as isize;
place!(Field::<u16>(Variant(_2, 0), 2)) = _18 as u16;
_17 = !_58;
_71 = Field::<char>(Variant(_65, 0), 1);
Call(place!(Field::<[i64; 8]>(Variant(_27, 3), 0)) = core::intrinsics::transmute(_56), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
_78.0 = Field::<u16>(Variant(_2, 0), 2);
_74.fld4 = _3.fld4 << _8;
_4 = !_52.0;
_32 = &_34.0;
place!(Field::<i16>(Variant(_27, 3), 1)) = -1646_i16;
_72 = Move(_27);
RET = Adt56::Variant3 { fld0: Field::<[i64; 8]>(Variant(_72, 3), 0),fld1: Field::<i16>(Variant(_72, 3), 1) };
Goto(bb44)
}
bb44 = {
Call(_80 = dump_var(12_usize, 42_usize, Move(_42), 47_usize, Move(_47), 35_usize, Move(_35), 58_usize, Move(_58)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_80 = dump_var(12_usize, 50_usize, Move(_50), 10_usize, Move(_10), 39_usize, Move(_39), 12_usize, Move(_12)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_80 = dump_var(12_usize, 56_usize, Move(_56), 69_usize, Move(_69), 45_usize, Move(_45), 48_usize, Move(_48)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_80 = dump_var(12_usize, 23_usize, Move(_23), 49_usize, Move(_49), 11_usize, Move(_11), 28_usize, Move(_28)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_80 = dump_var(12_usize, 71_usize, Move(_71), 4_usize, Move(_4), 38_usize, Move(_38), 9_usize, Move(_9)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: bool) -> *const u128 {
mir! {
type RET = *const u128;
let _5: (*const f32,);
let _6: &'static &'static (&'static u64,);
let _7: isize;
let _8: i32;
let _9: ([i16; 8],);
let _10: *mut u8;
let _11: f32;
let _12: Adt33;
let _13: u128;
let _14: char;
let _15: f64;
let _16: &'static &'static u128;
let _17: &'static &'static (&'static u64,);
let _18: f32;
let _19: [u32; 7];
let _20: &'static (u16, char, i8);
let _21: *mut u128;
let _22: char;
let _23: [u32; 7];
let _24: [i32; 1];
let _25: ();
let _26: ();
{
_3 = _2;
_1 = _3;
_4 = !true;
_3 = _1;
_3 = !_2;
_2 = !_3;
_4 = _3 <= _1;
_4 = !true;
Goto(bb1)
}
bb1 = {
_2 = 6_usize as isize;
_2 = 3287486978211897185_i64 as isize;
_4 = false;
_4 = false | true;
_4 = _3 <= _1;
_2 = _3;
_2 = _3;
_4 = _2 > _2;
_3 = !_1;
_3 = _2 + _1;
_4 = !true;
_4 = true;
_4 = !true;
_4 = true;
_1 = _3 ^ _3;
_2 = _3 + _1;
_2 = _4 as isize;
Goto(bb2)
}
bb2 = {
_4 = _1 >= _3;
_1 = _3 >> _3;
_3 = '\u{78e79}' as isize;
_1 = -_2;
_3 = _1;
_2 = -_1;
_4 = !false;
_4 = true;
_1 = _3 << _2;
_8 = 428693001_i32;
_1 = -_2;
_8 = !(-1873000193_i32);
_4 = !true;
_3 = _1 | _2;
_4 = false;
_9.0 = [17188_i16,(-22823_i16),1843_i16,(-25871_i16),10894_i16,(-1374_i16),7797_i16,(-10430_i16)];
_7 = _3 | _1;
_3 = _7 * _7;
_4 = !false;
_3 = _8 as isize;
_3 = !_7;
_8 = 161_u8 as i32;
Goto(bb3)
}
bb3 = {
_8 = (-615987277_i32);
_2 = !_3;
_9.0 = [(-12257_i16),(-27135_i16),23536_i16,(-16921_i16),(-13638_i16),(-27087_i16),(-14552_i16),(-508_i16)];
_11 = 6601840655539945364_i64 as f32;
_8 = 352703547_i32;
_3 = _7 & _2;
_4 = true;
_12 = Adt33::Variant0 { fld0: 205_u8,fld1: '\u{5c152}',fld2: 60338_u16,fld3: 1498081037_u32,fld4: 1_usize };
_2 = _7;
_7 = !_3;
_13 = !145749593429988620352319844065944821675_u128;
place!(Field::<usize>(Variant(_12, 0), 4)) = 3_usize & 2_usize;
place!(Field::<char>(Variant(_12, 0), 1)) = '\u{3da7}';
_14 = Field::<char>(Variant(_12, 0), 1);
_10 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12, 0), 0)));
_5.0 = core::ptr::addr_of!(_11);
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
Goto(bb4)
}
bb4 = {
(*_10) = _13 as u8;
place!(Field::<u32>(Variant(_12, 0), 3)) = _8 as u32;
RET = core::ptr::addr_of!(_13);
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
RET = core::ptr::addr_of!(_13);
match _8 {
0 => bb5,
352703547 => bb7,
_ => bb6
}
}
bb5 = {
_8 = (-615987277_i32);
_2 = !_3;
_9.0 = [(-12257_i16),(-27135_i16),23536_i16,(-16921_i16),(-13638_i16),(-27087_i16),(-14552_i16),(-508_i16)];
_11 = 6601840655539945364_i64 as f32;
_8 = 352703547_i32;
_3 = _7 & _2;
_4 = true;
_12 = Adt33::Variant0 { fld0: 205_u8,fld1: '\u{5c152}',fld2: 60338_u16,fld3: 1498081037_u32,fld4: 1_usize };
_2 = _7;
_7 = !_3;
_13 = !145749593429988620352319844065944821675_u128;
place!(Field::<usize>(Variant(_12, 0), 4)) = 3_usize & 2_usize;
place!(Field::<char>(Variant(_12, 0), 1)) = '\u{3da7}';
_14 = Field::<char>(Variant(_12, 0), 1);
_10 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12, 0), 0)));
_5.0 = core::ptr::addr_of!(_11);
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
Goto(bb4)
}
bb6 = {
_4 = _1 >= _3;
_1 = _3 >> _3;
_3 = '\u{78e79}' as isize;
_1 = -_2;
_3 = _1;
_2 = -_1;
_4 = !false;
_4 = true;
_1 = _3 << _2;
_8 = 428693001_i32;
_1 = -_2;
_8 = !(-1873000193_i32);
_4 = !true;
_3 = _1 | _2;
_4 = false;
_9.0 = [17188_i16,(-22823_i16),1843_i16,(-25871_i16),10894_i16,(-1374_i16),7797_i16,(-10430_i16)];
_7 = _3 | _1;
_3 = _7 * _7;
_4 = !false;
_3 = _8 as isize;
_3 = !_7;
_8 = 161_u8 as i32;
Goto(bb3)
}
bb7 = {
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
_10 = core::ptr::addr_of_mut!((*_10));
place!(Field::<u16>(Variant(_12, 0), 2)) = (-7267133341595300789_i64) as u16;
_11 = 5136054776245417377_u64 as f32;
_8 = 1261783350_i32 << _2;
_11 = 49_i8 as f32;
_15 = _3 as f64;
place!(Field::<u32>(Variant(_12, 0), 3)) = _15 as u32;
_14 = Field::<char>(Variant(_12, 0), 1);
Goto(bb8)
}
bb8 = {
(*RET) = !165194774566246455108761296628925040565_u128;
(*_10) = 54_u8 - 170_u8;
place!(Field::<u16>(Variant(_12, 0), 2)) = 35460_u16;
_18 = _11 - _11;
_10 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12, 0), 0)));
_3 = _7 + _2;
place!(Field::<usize>(Variant(_12, 0), 4)) = !3_usize;
_15 = _13 as f64;
_7 = !_3;
(*RET) = 150329860670513939833626823701324055607_u128 & 188812141350104552404103580127651345706_u128;
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
RET = core::ptr::addr_of!((*RET));
_15 = Field::<u16>(Variant(_12, 0), 2) as f64;
(*_10) = !73_u8;
_8 = (-1703093042_i32);
_9.0 = [28393_i16,(-27313_i16),21206_i16,32071_i16,(-3453_i16),19167_i16,32389_i16,14446_i16];
(*_10) = 235_u8;
(*RET) = 2390031343547358522_i64 as u128;
RET = core::ptr::addr_of!(_13);
_5.0 = core::ptr::addr_of!(_18);
_8 = !(-535316526_i32);
_11 = -_18;
_3 = -_2;
RET = core::ptr::addr_of!(_13);
(*RET) = 137298032810195860337463532612788948065_u128;
place!(Field::<u16>(Variant(_12, 0), 2)) = !18487_u16;
Goto(bb9)
}
bb9 = {
(*RET) = !171551599417571804019568531927967051826_u128;
_5.0 = core::ptr::addr_of!(_18);
place!(Field::<u32>(Variant(_12, 0), 3)) = 1855252563_u32;
place!(Field::<usize>(Variant(_12, 0), 4)) = 5678887212206906862_usize & 16529386237988386887_usize;
_1 = _18 as isize;
_21 = core::ptr::addr_of_mut!((*RET));
_19 = [Field::<u32>(Variant(_12, 0), 3),Field::<u32>(Variant(_12, 0), 3),Field::<u32>(Variant(_12, 0), 3),Field::<u32>(Variant(_12, 0), 3),Field::<u32>(Variant(_12, 0), 3),Field::<u32>(Variant(_12, 0), 3),Field::<u32>(Variant(_12, 0), 3)];
_2 = _7 + _1;
place!(Field::<u32>(Variant(_12, 0), 3)) = !257440578_u32;
_22 = _14;
_4 = false;
match (*_10) {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
235 => bb18,
_ => bb17
}
}
bb10 = {
(*RET) = !165194774566246455108761296628925040565_u128;
(*_10) = 54_u8 - 170_u8;
place!(Field::<u16>(Variant(_12, 0), 2)) = 35460_u16;
_18 = _11 - _11;
_10 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12, 0), 0)));
_3 = _7 + _2;
place!(Field::<usize>(Variant(_12, 0), 4)) = !3_usize;
_15 = _13 as f64;
_7 = !_3;
(*RET) = 150329860670513939833626823701324055607_u128 & 188812141350104552404103580127651345706_u128;
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
RET = core::ptr::addr_of!((*RET));
_15 = Field::<u16>(Variant(_12, 0), 2) as f64;
(*_10) = !73_u8;
_8 = (-1703093042_i32);
_9.0 = [28393_i16,(-27313_i16),21206_i16,32071_i16,(-3453_i16),19167_i16,32389_i16,14446_i16];
(*_10) = 235_u8;
(*RET) = 2390031343547358522_i64 as u128;
RET = core::ptr::addr_of!(_13);
_5.0 = core::ptr::addr_of!(_18);
_8 = !(-535316526_i32);
_11 = -_18;
_3 = -_2;
RET = core::ptr::addr_of!(_13);
(*RET) = 137298032810195860337463532612788948065_u128;
place!(Field::<u16>(Variant(_12, 0), 2)) = !18487_u16;
Goto(bb9)
}
bb11 = {
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
_10 = core::ptr::addr_of_mut!((*_10));
place!(Field::<u16>(Variant(_12, 0), 2)) = (-7267133341595300789_i64) as u16;
_11 = 5136054776245417377_u64 as f32;
_8 = 1261783350_i32 << _2;
_11 = 49_i8 as f32;
_15 = _3 as f64;
place!(Field::<u32>(Variant(_12, 0), 3)) = _15 as u32;
_14 = Field::<char>(Variant(_12, 0), 1);
Goto(bb8)
}
bb12 = {
_4 = _1 >= _3;
_1 = _3 >> _3;
_3 = '\u{78e79}' as isize;
_1 = -_2;
_3 = _1;
_2 = -_1;
_4 = !false;
_4 = true;
_1 = _3 << _2;
_8 = 428693001_i32;
_1 = -_2;
_8 = !(-1873000193_i32);
_4 = !true;
_3 = _1 | _2;
_4 = false;
_9.0 = [17188_i16,(-22823_i16),1843_i16,(-25871_i16),10894_i16,(-1374_i16),7797_i16,(-10430_i16)];
_7 = _3 | _1;
_3 = _7 * _7;
_4 = !false;
_3 = _8 as isize;
_3 = !_7;
_8 = 161_u8 as i32;
Goto(bb3)
}
bb13 = {
_8 = (-615987277_i32);
_2 = !_3;
_9.0 = [(-12257_i16),(-27135_i16),23536_i16,(-16921_i16),(-13638_i16),(-27087_i16),(-14552_i16),(-508_i16)];
_11 = 6601840655539945364_i64 as f32;
_8 = 352703547_i32;
_3 = _7 & _2;
_4 = true;
_12 = Adt33::Variant0 { fld0: 205_u8,fld1: '\u{5c152}',fld2: 60338_u16,fld3: 1498081037_u32,fld4: 1_usize };
_2 = _7;
_7 = !_3;
_13 = !145749593429988620352319844065944821675_u128;
place!(Field::<usize>(Variant(_12, 0), 4)) = 3_usize & 2_usize;
place!(Field::<char>(Variant(_12, 0), 1)) = '\u{3da7}';
_14 = Field::<char>(Variant(_12, 0), 1);
_10 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12, 0), 0)));
_5.0 = core::ptr::addr_of!(_11);
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
Goto(bb4)
}
bb14 = {
(*_10) = _13 as u8;
place!(Field::<u32>(Variant(_12, 0), 3)) = _8 as u32;
RET = core::ptr::addr_of!(_13);
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
RET = core::ptr::addr_of!(_13);
match _8 {
0 => bb5,
352703547 => bb7,
_ => bb6
}
}
bb15 = {
_8 = (-615987277_i32);
_2 = !_3;
_9.0 = [(-12257_i16),(-27135_i16),23536_i16,(-16921_i16),(-13638_i16),(-27087_i16),(-14552_i16),(-508_i16)];
_11 = 6601840655539945364_i64 as f32;
_8 = 352703547_i32;
_3 = _7 & _2;
_4 = true;
_12 = Adt33::Variant0 { fld0: 205_u8,fld1: '\u{5c152}',fld2: 60338_u16,fld3: 1498081037_u32,fld4: 1_usize };
_2 = _7;
_7 = !_3;
_13 = !145749593429988620352319844065944821675_u128;
place!(Field::<usize>(Variant(_12, 0), 4)) = 3_usize & 2_usize;
place!(Field::<char>(Variant(_12, 0), 1)) = '\u{3da7}';
_14 = Field::<char>(Variant(_12, 0), 1);
_10 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_12, 0), 0)));
_5.0 = core::ptr::addr_of!(_11);
place!(Field::<char>(Variant(_12, 0), 1)) = _14;
Goto(bb4)
}
bb16 = {
_4 = _1 >= _3;
_1 = _3 >> _3;
_3 = '\u{78e79}' as isize;
_1 = -_2;
_3 = _1;
_2 = -_1;
_4 = !false;
_4 = true;
_1 = _3 << _2;
_8 = 428693001_i32;
_1 = -_2;
_8 = !(-1873000193_i32);
_4 = !true;
_3 = _1 | _2;
_4 = false;
_9.0 = [17188_i16,(-22823_i16),1843_i16,(-25871_i16),10894_i16,(-1374_i16),7797_i16,(-10430_i16)];
_7 = _3 | _1;
_3 = _7 * _7;
_4 = !false;
_3 = _8 as isize;
_3 = !_7;
_8 = 161_u8 as i32;
Goto(bb3)
}
bb17 = {
_2 = 6_usize as isize;
_2 = 3287486978211897185_i64 as isize;
_4 = false;
_4 = false | true;
_4 = _3 <= _1;
_2 = _3;
_2 = _3;
_4 = _2 > _2;
_3 = !_1;
_3 = _2 + _1;
_4 = !true;
_4 = true;
_4 = !true;
_4 = true;
_1 = _3 ^ _3;
_2 = _3 + _1;
_2 = _4 as isize;
Goto(bb2)
}
bb18 = {
_2 = 29970_i16 as isize;
_24 = [_8];
place!(Field::<char>(Variant(_12, 0), 1)) = _22;
_18 = _11 - _11;
RET = core::ptr::addr_of!(_13);
place!(Field::<char>(Variant(_12, 0), 1)) = _22;
SetDiscriminant(_12, 2);
place!(Field::<i16>(Variant(_12, 2), 0)) = !(-7407_i16);
(*_21) = (-84794752647856936372573135822984250458_i128) as u128;
_7 = _2;
_7 = _1;
_24 = [_8];
(*RET) = _4 as u128;
_5.0 = core::ptr::addr_of!(_18);
(*RET) = !299975824733206306727835608498591832825_u128;
_8 = 49_u8 as i32;
Goto(bb19)
}
bb19 = {
Call(_25 = dump_var(13_usize, 7_usize, Move(_7), 24_usize, Move(_24), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_25 = dump_var(13_usize, 4_usize, Move(_4), 1_usize, Move(_1), 26_usize, _26, 26_usize, _26), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static &'static i8,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: bool) -> i128 {
mir! {
type RET = i128;
let _7: isize;
let _8: ();
let _9: ();
{
_3 = _2;
RET = -58816194031197558773103557389228116302_i128;
_2 = -_5;
_5 = _4 ^ _2;
_6 = true ^ true;
_4 = _5 & _2;
RET = (-160271153722942428604930435744786945638_i128) - (-130667794709925702358002974588104548156_i128);
_6 = false;
_6 = true;
RET = 8539842421345535337_u64 as i128;
_4 = _3 - _3;
RET = 125979964120475574608644220544854018014_i128;
_5 = !_3;
RET = 60715676910635342290958101192985764981_i128 >> _5;
_2 = !_3;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(14_usize, 2_usize, Move(_2), 3_usize, Move(_3), 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: &'static i8,mut _2: bool,mut _3: isize,mut _4: &'static f64,mut _5: bool,mut _6: isize,mut _7: bool) -> i8 {
mir! {
type RET = i8;
let _8: isize;
let _9: i8;
let _10: isize;
let _11: char;
let _12: ([u32; 7],);
let _13: &'static *mut Adt22;
let _14: Adt56;
let _15: u32;
let _16: [isize; 1];
let _17: isize;
let _18: *mut *mut [i8; 4];
let _19: *mut u8;
let _20: i128;
let _21: (f64, Adt22);
let _22: *mut [isize; 1];
let _23: char;
let _24: f32;
let _25: bool;
let _26: bool;
let _27: (*const f32,);
let _28: &'static *const f32;
let _29: [i16; 8];
let _30: isize;
let _31: f32;
let _32: f32;
let _33: [isize; 1];
let _34: isize;
let _35: [i8; 5];
let _36: *mut u128;
let _37: [i32; 1];
let _38: isize;
let _39: isize;
let _40: ();
let _41: ();
{
RET = 124_i8;
_6 = 700253053_i32 as isize;
_1 = &RET;
_1 = &(*_1);
_3 = _6;
_5 = !_7;
_1 = &(*_1);
_7 = _5;
_3 = '\u{60488}' as isize;
RET = (-28_i8);
_6 = 5843792013181010472_u64 as isize;
match RET {
0 => bb1,
340282366920938463463374607431768211428 => bb3,
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
_9 = RET & RET;
_2 = _7;
Goto(bb4)
}
bb4 = {
RET = !_9;
RET = -_9;
RET = _9;
_11 = '\u{edd0a}';
_11 = '\u{f232b}';
_3 = _6 ^ _6;
_7 = !_5;
_1 = &RET;
_6 = _3;
_7 = !_5;
_5 = _2 != _7;
_10 = !_3;
RET = !_9;
_7 = !_5;
RET = _9;
_1 = &RET;
_1 = &RET;
_10 = 2857704117732255065_i64 as isize;
_1 = &(*_1);
_8 = 2309507416_u32 as isize;
_1 = &RET;
RET = 160130130664735643136494025847450077707_i128 as i8;
_1 = &_9;
_9 = !RET;
_1 = &RET;
Goto(bb5)
}
bb5 = {
_5 = !_7;
_7 = _2;
_9 = (*_1);
_3 = 8517916521357272763_i64 as isize;
_10 = !_6;
_3 = 14_u8 as isize;
_1 = &(*_1);
_2 = _7;
RET = _9;
_7 = !_5;
_11 = '\u{bb4af}';
_8 = _6;
_10 = _3 + _6;
_10 = !_6;
_11 = '\u{62060}';
_11 = '\u{5d1b4}';
_12.0 = [1830854608_u32,465484518_u32,2452665196_u32,3392014048_u32,1736178498_u32,1222382989_u32,176215539_u32];
_5 = _7;
_11 = '\u{9df18}';
_11 = '\u{d0a11}';
_3 = !_10;
_10 = (-4276544854843605869_i64) as isize;
Goto(bb6)
}
bb6 = {
_1 = &RET;
_12.0 = [2396939789_u32,2956977375_u32,3230553966_u32,2630763073_u32,1420398115_u32,2032571422_u32,3363865585_u32];
_11 = '\u{d1546}';
_8 = _3 ^ _6;
_1 = &RET;
RET = (-609031673_i32) as i8;
_10 = _6 & _3;
_12.0 = [4034602697_u32,4137154369_u32,2641625574_u32,1808125508_u32,2975621895_u32,413660315_u32,866776424_u32];
_9 = RET;
_10 = _6;
_5 = _2 > _7;
RET = !_9;
_12.0 = [713083832_u32,4095212181_u32,4016852727_u32,2154213427_u32,1198881272_u32,2499286535_u32,3631335078_u32];
Goto(bb7)
}
bb7 = {
_12.0 = [1815809370_u32,1196528604_u32,239582657_u32,569809804_u32,2355660042_u32,3319506070_u32,1631103063_u32];
_1 = &_9;
_9 = 49133_u16 as i8;
_1 = &_9;
_2 = !_5;
_3 = !_10;
_6 = _3 >> _8;
_6 = RET as isize;
_7 = _2;
_12.0 = [650691502_u32,3202776244_u32,3947410809_u32,1891052522_u32,3767899566_u32,2170293432_u32,1502233420_u32];
_2 = _5;
_3 = !_8;
RET = _9;
_15 = 2836275402_u32 & 2283786778_u32;
_9 = RET << _6;
_5 = !_7;
RET = -_9;
Goto(bb8)
}
bb8 = {
_6 = -_10;
RET = 2457_u16 as i8;
_1 = &RET;
RET = -_9;
_7 = _5;
_10 = _6 * _3;
_6 = RET as isize;
_17 = -_8;
_3 = (-288542807_i32) as isize;
RET = !_9;
_1 = &RET;
_10 = _17 << _8;
_11 = '\u{c4c47}';
_12.0 = [_15,_15,_15,_15,_15,_15,_15];
RET = (-8787498594237081737_i64) as i8;
_15 = _2 as u32;
RET = _9;
_16 = [_17];
_8 = _10;
_10 = 10653_i16 as isize;
_11 = '\u{5e1ec}';
RET = -_9;
RET = _9 * _9;
_15 = !3749287226_u32;
_10 = _17;
_10 = _17;
_17 = _8 ^ _10;
RET = _9 + _9;
_10 = _8;
Goto(bb9)
}
bb9 = {
_2 = _5 & _5;
_1 = &_9;
_3 = -_10;
RET = (-54144500329239608836263182587905895566_i128) as i8;
_3 = 1290744463842572769559282493242862003_i128 as isize;
_21.0 = 187445011905261834521411074134900993128_u128 as f64;
_15 = !3523565761_u32;
_10 = 4242894795319564361_i64 as isize;
_21.0 = 18038_u16 as f64;
_16 = [_17];
_2 = _7 ^ _5;
RET = _9 | _9;
_20 = -(-158138757254972555485392997780422674229_i128);
_2 = !_5;
_15 = 3505634567_u32 << _17;
_6 = 1_usize as isize;
_1 = &(*_1);
_6 = -_8;
_11 = '\u{d9a8d}';
_11 = '\u{f9a1}';
Goto(bb10)
}
bb10 = {
_4 = &_21.0;
_3 = _6;
_4 = &(*_4);
_20 = !45339605328559877923488713244253718178_i128;
_23 = _11;
_22 = core::ptr::addr_of_mut!(_16);
(*_22) = [_3];
_17 = _3;
(*_22) = [_6];
_9 = 16854667555183970243_usize as i8;
(*_22) = [_17];
RET = !_9;
RET = (-12564_i16) as i8;
_21.0 = RET as f64;
_21.0 = 8786846703642741646_i64 as f64;
_25 = _7 > _2;
_24 = RET as f32;
_22 = core::ptr::addr_of_mut!(_16);
_27.0 = core::ptr::addr_of!(_24);
_12.0 = [_15,_15,_15,_15,_15,_15,_15];
(*_22) = [_3];
_22 = core::ptr::addr_of_mut!(_16);
_21.0 = 8728752188339873123_u64 as f64;
_15 = !736361019_u32;
_16 = [_17];
Goto(bb11)
}
bb11 = {
_26 = !_7;
RET = !_9;
_26 = !_5;
RET = _9;
_11 = _23;
_4 = &_21.0;
_20 = !(-97347365048374472154486545064408175653_i128);
_24 = 71902853_i32 as f32;
_3 = _17 >> _8;
_27.0 = core::ptr::addr_of!(_24);
_1 = &RET;
_21.1 = Adt22::Variant1 { fld0: _2,fld1: _12.0,fld2: _24,fld3: (*_1),fld4: _15,fld5: 1330875184_i32 };
_21.1 = Adt22::Variant1 { fld0: _5,fld1: _12.0,fld2: _24,fld3: RET,fld4: _15,fld5: (-1284265282_i32) };
_12 = (Field::<[u32; 7]>(Variant(_21.1, 1), 1),);
place!(Field::<i32>(Variant(_21.1, 1), 5)) = 247073264_i32 - (-1847723662_i32);
_21.1 = Adt22::Variant2 { fld0: 119_u8,fld1: 335525704984372402781304846717807287276_u128 };
place!(Field::<u128>(Variant(_21.1, 2), 1)) = 53155279840449671849264246528266825239_u128 - 70331169186079373490917719322480429469_u128;
_25 = _5 < _5;
(*_22) = [_3];
RET = !_9;
_16 = [_3];
_12.0 = [_15,_15,_15,_15,_15,_15,_15];
Goto(bb12)
}
bb12 = {
_24 = 6_u8 as f32;
_26 = _2 & _2;
place!(Field::<u8>(Variant(_21.1, 2), 0)) = 29_u8 & 211_u8;
RET = !_9;
_29 = [(-12692_i16),3698_i16,(-29686_i16),(-4715_i16),3060_i16,8580_i16,(-29607_i16),(-32_i16)];
_27.0 = core::ptr::addr_of!(_24);
_22 = core::ptr::addr_of_mut!(_16);
SetDiscriminant(_21.1, 2);
_1 = &_9;
(*_22) = [_8];
Goto(bb13)
}
bb13 = {
_24 = 4653293535034549872_i64 as f32;
_3 = _24 as isize;
_21.0 = _20 as f64;
_7 = _25 != _5;
_24 = 105528306056038382558329016240745326086_u128 as f32;
_9 = !RET;
_28 = &_27.0;
_9 = RET;
_32 = _17 as f32;
_8 = _6;
_8 = _17;
_10 = _8 - _17;
_29 = [29386_i16,446_i16,10831_i16,(-7483_i16),29064_i16,24366_i16,(-21144_i16),11518_i16];
_16 = [_6];
place!(Field::<u8>(Variant(_21.1, 2), 0)) = 168_u8 ^ 138_u8;
_34 = _10;
_4 = &_21.0;
RET = _20 as i8;
_31 = -_32;
_21.0 = _31 as f64;
_29 = [387_i16,22220_i16,26233_i16,(-26069_i16),(-6221_i16),(-12162_i16),(-25626_i16),27412_i16];
_32 = _31 * _31;
_1 = &_9;
Goto(bb14)
}
bb14 = {
_1 = &RET;
_5 = _2 == _26;
_35 = [_9,(*_1),(*_1),_9,(*_1)];
_36 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_21.1, 2), 1)));
_9 = _6 as i8;
_23 = _11;
_34 = 4394947877228631246_u64 as isize;
place!(Field::<u8>(Variant(_21.1, 2), 0)) = 194_u8 >> _9;
_30 = !_8;
_1 = &RET;
_33 = (*_22);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(15_usize, 23_usize, Move(_23), 17_usize, Move(_17), 7_usize, Move(_7), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(15_usize, 8_usize, Move(_8), 29_usize, Move(_29), 35_usize, Move(_35), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(15_usize, 20_usize, Move(_20), 5_usize, Move(_5), 34_usize, Move(_34), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize) -> bool {
mir! {
type RET = bool;
let _4: Adt19;
let _5: *mut [i8; 4];
let _6: ();
let _7: ();
{
_3 = _1;
_4 = Adt19 { fld0: false,fld1: '\u{bb3cd}',fld2: _1,fld3: (-16_i8),fld4: 121614346736037535803759882887742593303_i128 };
_3 = _4.fld2 + _4.fld2;
_4 = Adt19 { fld0: false,fld1: '\u{278e7}',fld2: _1,fld3: (-57_i8),fld4: (-42985261455230282608052155797058042616_i128) };
RET = _4.fld0;
_3 = !_2;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(16_usize, 1_usize, Move(_1), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17() -> char {
mir! {
type RET = char;
let _1: char;
let _2: u64;
let _3: isize;
let _4: bool;
let _5: i32;
let _6: &'static *mut Adt22;
let _7: [i32; 1];
let _8: ();
let _9: ();
{
RET = '\u{6ccab}';
RET = '\u{f944a}';
_1 = RET;
_1 = RET;
RET = _1;
RET = _1;
_2 = !10599613445189640341_u64;
RET = _1;
_2 = 2282559252194730055_u64;
RET = _1;
RET = _1;
_3 = 9223372036854775807_isize & 9223372036854775807_isize;
_3 = (-28550_i16) as isize;
RET = _1;
_1 = RET;
_3 = !87_isize;
RET = _1;
_3 = 44_isize ^ 9223372036854775807_isize;
Call(RET = fn18(_1, _2, _3, _3, _3, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !10200300987494568661_u64;
_2 = 16096861453583907539_u64;
_3 = (-9223372036854775808_isize);
_1 = RET;
_1 = RET;
_2 = 1038631265919886996_u64 ^ 5515938720449208693_u64;
_5 = true as i32;
_5 = 355001053_i32 << _3;
_4 = _2 <= _2;
_2 = (-4177587994231310232_i64) as u64;
_2 = 3394280189504333979_u64 << _5;
_3 = 7_u8 as isize;
_4 = false & true;
_4 = false;
_5 = !1413941965_i32;
_2 = !14055896490621009891_u64;
RET = _1;
RET = _1;
RET = _1;
_3 = 9223372036854775807_isize;
_1 = RET;
_4 = false & true;
_4 = !false;
_5 = (-663583247_i32) | (-1395401513_i32);
_1 = RET;
RET = _1;
Goto(bb2)
}
bb2 = {
_4 = false;
_1 = RET;
RET = _1;
RET = _1;
RET = _1;
RET = _1;
_5 = 356564190_i32;
RET = _1;
_4 = !false;
_4 = false;
_1 = RET;
_5 = 939223926_i32;
RET = _1;
_1 = RET;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb3 = {
_2 = !10200300987494568661_u64;
_2 = 16096861453583907539_u64;
_3 = (-9223372036854775808_isize);
_1 = RET;
_1 = RET;
_2 = 1038631265919886996_u64 ^ 5515938720449208693_u64;
_5 = true as i32;
_5 = 355001053_i32 << _3;
_4 = _2 <= _2;
_2 = (-4177587994231310232_i64) as u64;
_2 = 3394280189504333979_u64 << _5;
_3 = 7_u8 as isize;
_4 = false & true;
_4 = false;
_5 = !1413941965_i32;
_2 = !14055896490621009891_u64;
RET = _1;
RET = _1;
RET = _1;
_3 = 9223372036854775807_isize;
_1 = RET;
_4 = false & true;
_4 = !false;
_5 = (-663583247_i32) | (-1395401513_i32);
_1 = RET;
RET = _1;
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
_1 = RET;
_3 = _5 as isize;
_1 = RET;
match _5 {
0 => bb5,
1 => bb7,
2 => bb9,
3 => bb10,
4 => bb11,
939223926 => bb13,
_ => bb12
}
}
bb9 = {
Return()
}
bb10 = {
_2 = !10200300987494568661_u64;
_2 = 16096861453583907539_u64;
_3 = (-9223372036854775808_isize);
_1 = RET;
_1 = RET;
_2 = 1038631265919886996_u64 ^ 5515938720449208693_u64;
_5 = true as i32;
_5 = 355001053_i32 << _3;
_4 = _2 <= _2;
_2 = (-4177587994231310232_i64) as u64;
_2 = 3394280189504333979_u64 << _5;
_3 = 7_u8 as isize;
_4 = false & true;
_4 = false;
_5 = !1413941965_i32;
_2 = !14055896490621009891_u64;
RET = _1;
RET = _1;
RET = _1;
_3 = 9223372036854775807_isize;
_1 = RET;
_4 = false & true;
_4 = !false;
_5 = (-663583247_i32) | (-1395401513_i32);
_1 = RET;
RET = _1;
Goto(bb2)
}
bb11 = {
_4 = false;
_1 = RET;
RET = _1;
RET = _1;
RET = _1;
RET = _1;
_5 = 356564190_i32;
RET = _1;
_4 = !false;
_4 = false;
_1 = RET;
_5 = 939223926_i32;
RET = _1;
_1 = RET;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb12 = {
_2 = !10200300987494568661_u64;
_2 = 16096861453583907539_u64;
_3 = (-9223372036854775808_isize);
_1 = RET;
_1 = RET;
_2 = 1038631265919886996_u64 ^ 5515938720449208693_u64;
_5 = true as i32;
_5 = 355001053_i32 << _3;
_4 = _2 <= _2;
_2 = (-4177587994231310232_i64) as u64;
_2 = 3394280189504333979_u64 << _5;
_3 = 7_u8 as isize;
_4 = false & true;
_4 = false;
_5 = !1413941965_i32;
_2 = !14055896490621009891_u64;
RET = _1;
RET = _1;
RET = _1;
_3 = 9223372036854775807_isize;
_1 = RET;
_4 = false & true;
_4 = !false;
_5 = (-663583247_i32) | (-1395401513_i32);
_1 = RET;
RET = _1;
Goto(bb2)
}
bb13 = {
_5 = !(-839527341_i32);
_7 = [_5];
RET = _1;
RET = _1;
RET = _1;
Goto(bb14)
}
bb14 = {
_2 = (-14376729336577280182854427946726726091_i128) as u64;
_3 = (-9223372036854775808_isize);
RET = _1;
_2 = 1213180966442025812_u64;
_2 = 8054012991641252911_u64;
_1 = RET;
_3 = !9223372036854775807_isize;
RET = _1;
RET = _1;
Goto(bb15)
}
bb15 = {
Call(_8 = dump_var(17_usize, 4_usize, Move(_4), 5_usize, Move(_5), 7_usize, Move(_7), 9_usize, _9), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: char,mut _2: u64,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u64,mut _7: u64) -> char {
mir! {
type RET = char;
let _8: (Adt56,);
let _9: &'static f64;
let _10: &'static (*const u128, [u8; 6]);
let _11: *mut [isize; 1];
let _12: u128;
let _13: isize;
let _14: isize;
let _15: isize;
let _16: (u16, char, i8);
let _17: [i8; 5];
let _18: (*const u128, [u8; 6]);
let _19: &'static i8;
let _20: &'static (&'static u64,);
let _21: *const [bool; 3];
let _22: *mut [i8; 4];
let _23: isize;
let _24: usize;
let _25: Adt73;
let _26: f32;
let _27: &'static f64;
let _28: isize;
let _29: ();
let _30: ();
{
_3 = _4;
Goto(bb1)
}
bb1 = {
_2 = _7 + _7;
_2 = _6 ^ _7;
_4 = _3;
RET = _1;
RET = _1;
RET = _1;
_4 = -_5;
_3 = _5 + _5;
_6 = !_7;
_5 = _3 & _4;
_6 = _2;
_6 = (-27276_i16) as u64;
_3 = _4 - _5;
_2 = _7;
match _2 {
2282559252194730055 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET = _1;
_6 = _7 % _2;
_13 = _5;
_4 = (-8454241523957853951_i64) as isize;
_12 = _13 as u128;
_4 = _3 ^ _5;
_1 = RET;
RET = _1;
_3 = _4;
RET = _1;
_1 = RET;
RET = _1;
_6 = _2;
Goto(bb4)
}
bb4 = {
_14 = _2 as isize;
RET = _1;
_16.0 = !43317_u16;
_2 = _16.0 as u64;
_18.1 = [189_u8,220_u8,250_u8,87_u8,161_u8,177_u8];
_17 = [(-10_i8),(-2_i8),100_i8,(-17_i8),13_i8];
_18.1 = [171_u8,246_u8,218_u8,190_u8,51_u8,39_u8];
_10 = &_18;
_16.2 = _6 as i8;
_18.0 = core::ptr::addr_of!(_12);
_4 = _5 << _12;
_16.1 = _1;
match _6 {
2282559252194730055 => bb6,
_ => bb5
}
}
bb5 = {
_2 = _7 + _7;
_2 = _6 ^ _7;
_4 = _3;
RET = _1;
RET = _1;
RET = _1;
_4 = -_5;
_3 = _5 + _5;
_6 = !_7;
_5 = _3 & _4;
_6 = _2;
_6 = (-27276_i16) as u64;
_3 = _4 - _5;
_2 = _7;
match _2 {
2282559252194730055 => bb3,
_ => bb2
}
}
bb6 = {
_15 = 1565084414204890136_i64 as isize;
_14 = _3;
_10 = &_18;
_18.0 = core::ptr::addr_of!(_12);
_4 = 3478233318_u32 as isize;
_14 = _7 as isize;
_2 = _7;
_5 = _3 << _12;
_17 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_16 = (35090_u16, RET, 107_i8);
_18.0 = core::ptr::addr_of!(_12);
_5 = _3;
match _16.2 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb7,
107 => bb9,
_ => bb8
}
}
bb7 = {
_2 = _7 + _7;
_2 = _6 ^ _7;
_4 = _3;
RET = _1;
RET = _1;
RET = _1;
_4 = -_5;
_3 = _5 + _5;
_6 = !_7;
_5 = _3 & _4;
_6 = _2;
_6 = (-27276_i16) as u64;
_3 = _4 - _5;
_2 = _7;
match _2 {
2282559252194730055 => bb3,
_ => bb2
}
}
bb8 = {
_14 = _2 as isize;
RET = _1;
_16.0 = !43317_u16;
_2 = _16.0 as u64;
_18.1 = [189_u8,220_u8,250_u8,87_u8,161_u8,177_u8];
_17 = [(-10_i8),(-2_i8),100_i8,(-17_i8),13_i8];
_18.1 = [171_u8,246_u8,218_u8,190_u8,51_u8,39_u8];
_10 = &_18;
_16.2 = _6 as i8;
_18.0 = core::ptr::addr_of!(_12);
_4 = _5 << _12;
_16.1 = _1;
match _6 {
2282559252194730055 => bb6,
_ => bb5
}
}
bb9 = {
_12 = !324012859007079148726846547868878710163_u128;
_6 = !_7;
_7 = 80_u8 as u64;
_5 = _3 + _13;
_10 = &_18;
_14 = !_5;
_16.0 = !6233_u16;
_16.2 = !38_i8;
_19 = &_16.2;
_16.1 = RET;
_16.2 = -(-89_i8);
_16 = (2170_u16, RET, (-82_i8));
_13 = (-1695294929777524533_i64) as isize;
_4 = !_5;
_3 = _4 | _14;
_16.1 = _1;
_16.0 = 8573_u16;
_18.1 = [46_u8,203_u8,197_u8,174_u8,126_u8,125_u8];
_19 = &_16.2;
_18.1 = [24_u8,79_u8,153_u8,138_u8,245_u8,52_u8];
_13 = _14;
_16 = (54777_u16, _1, 15_i8);
_16 = (60058_u16, RET, (-67_i8));
Goto(bb10)
}
bb10 = {
_5 = _2 as isize;
_2 = _12 as u64;
_16.1 = _1;
_16.1 = _1;
_16.1 = RET;
_6 = _7 ^ _7;
_1 = _16.1;
RET = _16.1;
_10 = &_18;
_2 = _6;
_18.0 = core::ptr::addr_of!(_12);
_15 = _4;
_2 = !_6;
_16.0 = 36076_u16;
_16.2 = (-87_i8);
_16.0 = !16654_u16;
_19 = &_16.2;
_4 = _13;
_16.0 = !31354_u16;
_19 = &(*_19);
_10 = &_18;
_13 = 3707109269_u32 as isize;
_4 = !_3;
_1 = _16.1;
_19 = &(*_19);
_1 = _16.1;
Goto(bb11)
}
bb11 = {
_13 = _4 ^ _4;
RET = _1;
_16.2 = !12_i8;
_18.1 = [110_u8,189_u8,28_u8,34_u8,231_u8,98_u8];
_17 = [_16.2,_16.2,_16.2,_16.2,_16.2];
_14 = (-458736724_i32) as isize;
_13 = _15 * _15;
RET = _1;
RET = _1;
_3 = _16.0 as isize;
_4 = _13 ^ _14;
_16.0 = !31258_u16;
Goto(bb12)
}
bb12 = {
_4 = _15 & _15;
_23 = _13;
RET = _16.1;
_18.0 = core::ptr::addr_of!(_12);
_13 = _15;
_23 = !_4;
_16.1 = _1;
_15 = (-22093_i16) as isize;
_13 = !_23;
_24 = !14050136170043530370_usize;
_13 = !_4;
_7 = _16.2 as u64;
_2 = _6 << _23;
_1 = RET;
RET = _1;
Goto(bb13)
}
bb13 = {
RET = _16.1;
_5 = _13;
_16.0 = !2395_u16;
_23 = _5;
_19 = &_16.2;
_15 = _5 >> _5;
_18.0 = core::ptr::addr_of!(_12);
_26 = _16.0 as f32;
_5 = 288930699_i32 as isize;
_16.0 = !13923_u16;
_18.0 = core::ptr::addr_of!(_12);
Goto(bb14)
}
bb14 = {
_6 = !_2;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(18_usize, 5_usize, Move(_5), 17_usize, Move(_17), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(18_usize, 16_usize, Move(_16), 1_usize, Move(_1), 12_usize, Move(_12), 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: Adt19,mut _2: u8,mut _3: isize,mut _4: isize) -> f64 {
mir! {
type RET = f64;
let _5: u128;
let _6: &'static u128;
let _7: (&'static u64,);
let _8: [char; 4];
let _9: &'static i32;
let _10: [i16; 8];
let _11: &'static i8;
let _12: ();
let _13: ();
{
_1.fld4 = 48779185898737147244216178798234378541_i128;
RET = 8027898993524117317_u64 as f64;
_1.fld1 = '\u{42237}';
_1.fld4 = _4 as i128;
_1.fld0 = _4 <= _3;
_1.fld0 = false;
_4 = _3;
_5 = !51078282662677041013205292082239832423_u128;
_4 = -_3;
RET = _2 as f64;
_1 = Adt19 { fld0: false,fld1: '\u{39b1b}',fld2: _4,fld3: 78_i8,fld4: 112536111951568484234578074504772300248_i128 };
_1.fld3 = 114_i8;
_2 = !97_u8;
_1.fld4 = (-121887374864155317505784303052288318587_i128);
_3 = _1.fld2 - _4;
RET = _2 as f64;
_5 = 283051182227626545157237054277193018623_u128;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
283051182227626545157237054277193018623 => bb7,
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
RET = _1.fld2 as f64;
_6 = &_5;
_1.fld3 = -(-100_i8);
RET = _1.fld4 as f64;
_1.fld1 = '\u{d1997}';
_1.fld0 = _3 < _3;
_1.fld0 = !true;
_6 = &(*_6);
_1.fld1 = '\u{c4129}';
RET = _2 as f64;
_6 = &(*_6);
_1 = Adt19 { fld0: true,fld1: '\u{da479}',fld2: _3,fld3: 116_i8,fld4: (-69090887198677734657076848160105552346_i128) };
_1.fld2 = 2969605576129758043_u64 as isize;
_1.fld0 = !false;
_5 = _1.fld0 as u128;
RET = 26450_i16 as f64;
_2 = 6_usize as u8;
_1 = Adt19 { fld0: true,fld1: '\u{c296f}',fld2: _4,fld3: (-125_i8),fld4: 18135199893685246677462455444043857122_i128 };
_1.fld0 = _1.fld1 >= _1.fld1;
RET = 6777141452071606886_usize as f64;
_1 = Adt19 { fld0: true,fld1: '\u{aec1a}',fld2: _4,fld3: 30_i8,fld4: (-101872318803519048813766737006696907233_i128) };
RET = (-4495697736836015716_i64) as f64;
match _1.fld4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
238410048117419414649607870425071304223 => bb8,
_ => bb6
}
}
bb8 = {
_8 = [_1.fld1,_1.fld1,_1.fld1,_1.fld1];
_1.fld3 = 8600288111308282198_u64 as i8;
_1.fld3 = (-38_i8);
Goto(bb9)
}
bb9 = {
_1.fld4 = (-138336833295088738637384650871001836217_i128) >> _3;
_5 = _1.fld1 as u128;
_6 = &_5;
_1.fld3 = (-66_i8);
RET = (-1894940210_i32) as f64;
_1.fld0 = _4 > _4;
_1.fld4 = !49541912100780779750790232587602449333_i128;
match _1.fld3 {
0 => bb1,
1 => bb5,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463463374607431768211390 => bb14,
_ => bb13
}
}
bb10 = {
_8 = [_1.fld1,_1.fld1,_1.fld1,_1.fld1];
_1.fld3 = 8600288111308282198_u64 as i8;
_1.fld3 = (-38_i8);
Goto(bb9)
}
bb11 = {
RET = _1.fld2 as f64;
_6 = &_5;
_1.fld3 = -(-100_i8);
RET = _1.fld4 as f64;
_1.fld1 = '\u{d1997}';
_1.fld0 = _3 < _3;
_1.fld0 = !true;
_6 = &(*_6);
_1.fld1 = '\u{c4129}';
RET = _2 as f64;
_6 = &(*_6);
_1 = Adt19 { fld0: true,fld1: '\u{da479}',fld2: _3,fld3: 116_i8,fld4: (-69090887198677734657076848160105552346_i128) };
_1.fld2 = 2969605576129758043_u64 as isize;
_1.fld0 = !false;
_5 = _1.fld0 as u128;
RET = 26450_i16 as f64;
_2 = 6_usize as u8;
_1 = Adt19 { fld0: true,fld1: '\u{c296f}',fld2: _4,fld3: (-125_i8),fld4: 18135199893685246677462455444043857122_i128 };
_1.fld0 = _1.fld1 >= _1.fld1;
RET = 6777141452071606886_usize as f64;
_1 = Adt19 { fld0: true,fld1: '\u{aec1a}',fld2: _4,fld3: 30_i8,fld4: (-101872318803519048813766737006696907233_i128) };
RET = (-4495697736836015716_i64) as f64;
match _1.fld4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
238410048117419414649607870425071304223 => bb8,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_5 = !240177027463905710148481481774300667791_u128;
_2 = (-2093014099_i32) as u8;
_6 = &_5;
_4 = _3 & _3;
_1.fld4 = _1.fld1 as i128;
_1.fld3 = 54_i8;
RET = 10766244581023267350_u64 as f64;
Goto(bb15)
}
bb15 = {
Call(_12 = dump_var(19_usize, 3_usize, Move(_3), 5_usize, Move(_5), 13_usize, _13, 13_usize, _13), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(5966245197408039508_u64), std::hint::black_box('\u{6d20d}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(117_u8), std::hint::black_box(137201554053682667733135813506710773273_u128), std::hint::black_box(128369953_i32), std::hint::black_box(6410483504431266007_i64), std::hint::black_box(3031144456_u32));
                
            }
impl PrintFDebug for Adt19{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt19{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt19 {
fld0: bool,
fld1: char,
fld2: isize,
fld3: i8,
fld4: i128,
}
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: bool,
fld1: Adt19,
fld2: u16,
fld3: i8,
fld4: u64,
fld5: [u32; 7],

},
Variant1{
fld0: bool,
fld1: [u32; 7],
fld2: f32,
fld3: i8,
fld4: u32,
fld5: i32,

},
Variant2{
fld0: u8,
fld1: u128,

}}
impl PrintFDebug for Adt25{
	unsafe fn printf_debug(&self){unsafe{printf("Adt25::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt25 {
Variant0{
fld0: isize,

},
Variant1{
fld0: u16,
fld1: u128,
fld2: f64,
fld3: i8,
fld4: usize,
fld5: Adt22,
fld6: *mut i32,

},
Variant2{
fld0: u128,
fld1: f64,
fld2: i16,
fld3: i64,

},
Variant3{
fld0: u8,
fld1: u128,
fld2: f32,
fld3: [u32; 7],

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){unsafe{printf("Adt33::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt33 {
Variant0{
fld0: u8,
fld1: char,
fld2: u16,
fld3: u32,
fld4: usize,

},
Variant1{
fld0: f32,
fld1: Adt22,
fld2: usize,

},
Variant2{
fld0: i16,
fld1: [u32; 7],
fld2: u8,
fld3: i8,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: [i64; 8],
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [u64; 4],
fld1: (char, i64, Adt19, [u8; 6]),

},
Variant1{
fld0: [bool; 3],
fld1: u32,
fld2: *mut f64,
fld3: [isize; 1],
fld4: Adt19,
fld5: usize,

},
Variant2{
fld0: (u16, char, i8),
fld1: f64,
fld2: u32,
fld3: *mut u128,

},
Variant3{
fld0: [i64; 8],
fld1: i16,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: u64,
fld1: i64,
fld2: *const f32,
fld3: u8,
fld4: f64,
fld5: [u32; 5],

},
Variant1{
fld0: [i32; 1],
fld1: *const f32,
fld2: *const [bool; 3],
fld3: u8,

},
Variant2{
fld0: *mut [i8; 4],

},
Variant3{
fld0: [isize; 4],

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: [i16; 8],
fld1: i128,

},
Variant1{
fld0: *mut u128,
fld1: f32,
fld2: (Adt56,),
fld3: [i64; 1],

},
Variant2{
fld0: bool,
fld1: char,
fld2: (char, i64, Adt19, [u8; 6]),
fld3: [i32; 1],
fld4: [u32; 5],
fld5: [i8; 5],
fld6: u16,
fld7: u128,

},
Variant3{
fld0: *const u128,
fld1: [u32; 5],
fld2: [bool; 3],
fld3: i128,
fld4: f64,

}}

