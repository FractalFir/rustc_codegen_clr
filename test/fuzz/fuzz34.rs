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
pub fn fn0() -> u16 {
mir! {
type RET = u16;
let _1: bool;
let _2: *const *mut (Adt25, *const *mut i64, u32, i16);
let _3: bool;
let _4: char;
let _5: (char, Adt56, u8);
let _6: f64;
let _7: [bool; 5];
let _8: Adt56;
let _9: &'static Adt25;
let _10: *mut &'static i8;
let _11: i16;
let _12: *const *mut (Adt25, *const *mut i64, u32, i16);
let _13: &'static (*const Adt18, bool);
let _14: *const Adt18;
let _15: bool;
let _16: [u8; 3];
let _17: (i16,);
let _18: *mut *const Adt25;
let _19: isize;
let _20: (i16,);
let _21: char;
let _22: *mut bool;
let _23: Adt51;
let _24: *const Adt18;
let _25: &'static i8;
let _26: [bool; 2];
let _27: bool;
let _28: (&'static *mut i64, &'static Adt25, (i16,), usize);
let _29: [u8; 3];
let _30: isize;
let _31: f64;
let _32: u8;
let _33: ();
let _34: ();
{
RET = false as u16;
RET = 61632_u16;
RET = 9010642274824205187_usize as u16;
RET = 11605_u16;
RET = 31741_u16;
RET = 20559_u16 << 429724489_u32;
RET = 57423_u16;
RET = (-29925_i16) as u16;
RET = !52572_u16;
RET = '\u{d85a9}' as u16;
RET = 64029_u16;
RET = 40509_u16 | 48329_u16;
_1 = true;
RET = 1926_u16;
RET = 50690_u16 & 42684_u16;
RET = 43044_u16 | 35789_u16;
RET = !11058_u16;
_3 = _1 <= _1;
_3 = RET <= RET;
RET = 54770_u16 - 7037_u16;
RET = 33063_u16 << 321805228136938833780971930568246173969_u128;
_3 = _1 & _1;
RET = !46461_u16;
RET = 17039_u16;
_3 = !_1;
_3 = !_1;
RET = 52809_u16;
Call(_4 = fn1(_1, _1, _3, RET, _1, RET, _3, RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 8148771694851700463_u64 as u16;
RET = 140_u8 as u16;
RET = 452628837_i32 as u16;
_3 = _1;
_5.1.fld0 = [214_u8,78_u8,33_u8];
_5.1.fld0 = [244_u8,37_u8,120_u8];
_1 = !_3;
_5.2 = 19_u8;
_5.1.fld0 = [_5.2,_5.2,_5.2];
_3 = !_1;
_5.0 = _4;
_5.2 = 0_u8 << RET;
Goto(bb2)
}
bb2 = {
RET = !15088_u16;
Goto(bb3)
}
bb3 = {
_5.2 = !179_u8;
_3 = !_1;
_3 = _1 & _1;
_1 = !_3;
RET = (-907834949_i32) as u16;
Goto(bb4)
}
bb4 = {
RET = _3 as u16;
_5.0 = _4;
_4 = _5.0;
_6 = 774897389_i32 as f64;
RET = 26796_u16 << _5.2;
_5.2 = !36_u8;
_7 = [_3,_3,_3,_3,_1];
_4 = _5.0;
RET = 31027_u16 + 60487_u16;
_1 = _5.2 > _5.2;
_7 = [_1,_3,_3,_1,_3];
RET = 9843_u16;
_5.0 = _4;
_4 = _5.0;
_7 = [_3,_3,_3,_3,_3];
_5.2 = (-425549379_i32) as u8;
_8.fld0 = [_5.2,_5.2,_5.2];
_5.0 = _4;
_5.0 = _4;
_8 = Adt56 { fld0: _5.1.fld0 };
_1 = _3;
_1 = _3;
_4 = _5.0;
_5.1.fld0 = [_5.2,_5.2,_5.2];
_8 = Adt56 { fld0: _5.1.fld0 };
_5.0 = _4;
Goto(bb5)
}
bb5 = {
_5 = (_4, Move(_8), 11_u8);
_5.2 = !195_u8;
_4 = _5.0;
RET = !23411_u16;
_6 = 14709008238477943041_usize as f64;
_8 = Adt56 { fld0: _5.1.fld0 };
RET = 2_usize as u16;
_1 = !_3;
_6 = 1_usize as f64;
_3 = _1 >= _1;
_1 = _3 == _3;
RET = !52933_u16;
_5.0 = _4;
_5.0 = _4;
_5.1 = Adt56 { fld0: _8.fld0 };
_6 = (-1274427825_i32) as f64;
_8.fld0 = [_5.2,_5.2,_5.2];
_5.2 = !147_u8;
_5 = (_4, Move(_8), 17_u8);
_4 = _5.0;
Goto(bb6)
}
bb6 = {
_1 = !_3;
_4 = _5.0;
_11 = (-31490_i16) - 15238_i16;
_5.2 = 1754337716568085051_i64 as u8;
_5.0 = _4;
_4 = _5.0;
_15 = RET > RET;
_5.2 = 15_u8;
_4 = _5.0;
_5.2 = 220_u8;
_16 = [_5.2,_5.2,_5.2];
_17.0 = _11 >> _11;
_3 = _1;
_19 = 10018510029760196956_u64 as isize;
_17.0 = 2984221940_u32 as i16;
_4 = _5.0;
_20.0 = _11;
_7 = [_1,_3,_15,_15,_1];
_8.fld0 = _16;
_22 = core::ptr::addr_of_mut!(_3);
_5.2 = (-89058737983528956556323223098079204247_i128) as u8;
Goto(bb7)
}
bb7 = {
_20 = (_17.0,);
_11 = _17.0;
_5 = (_4, Move(_8), 180_u8);
_21 = _5.0;
_1 = (*_22);
_15 = !(*_22);
_24 = core::ptr::addr_of!(_23.fld1);
_21 = _4;
_17.0 = _5.2 as i16;
_23.fld0.fld1 = !103341357719346256000411281021776052238_i128;
_5.1 = Adt56 { fld0: _16 };
(*_24) = Adt18::Variant0 { fld0: _6,fld1: (-234476594095803605_i64) };
_17.0 = _11;
RET = !37767_u16;
_23.fld1 = Adt18::Variant0 { fld0: _6,fld1: 37204160529645320_i64 };
RET = 20388_u16 - 21523_u16;
_11 = !_17.0;
_23.fld0.fld2 = !_19;
_23.fld2 = (-5737800911368354452_i64) << _23.fld0.fld1;
Goto(bb8)
}
bb8 = {
_1 = _15 ^ _15;
place!(Field::<f64>(Variant((*_24), 0), 0)) = _6 - _6;
place!(Field::<i64>(Variant((*_24), 0), 1)) = 3405738354_u32 as i64;
_23.fld0.fld4 = 9625106874166216490_u64;
_10 = core::ptr::addr_of_mut!(_25);
_16 = _5.1.fld0;
(*_22) = _15;
(*_24) = Adt18::Variant1 { fld0: 39162225559542022927107871634560636851_u128,fld1: _23.fld0.fld1,fld2: 2510501989_u32,fld3: _5.2 };
place!(Field::<u8>(Variant((*_24), 1), 3)) = _5.2;
_8.fld0 = [Field::<u8>(Variant(_23.fld1, 1), 3),_5.2,Field::<u8>(Variant((*_24), 1), 3)];
_14 = core::ptr::addr_of!((*_24));
_23.fld0 = Adt22 { fld0: (*_22),fld1: Field::<i128>(Variant((*_14), 1), 1),fld2: _19,fld3: RET,fld4: 1036635528564358067_u64 };
_29 = [Field::<u8>(Variant((*_14), 1), 3),Field::<u8>(Variant((*_24), 1), 3),Field::<u8>(Variant((*_14), 1), 3)];
(*_22) = _1 == _23.fld0.fld0;
_23.fld3 = [3649253167_u32,2214199333_u32,2545071537_u32];
_23.fld4 = [_1,(*_22),_3,_3,(*_22)];
_23.fld4 = [(*_22),(*_22),_1,(*_22),(*_22)];
match Field::<u8>(Variant(_23.fld1, 1), 3) {
0 => bb6,
1 => bb2,
2 => bb5,
180 => bb10,
_ => bb9
}
}
bb9 = {
_1 = !_3;
_4 = _5.0;
_11 = (-31490_i16) - 15238_i16;
_5.2 = 1754337716568085051_i64 as u8;
_5.0 = _4;
_4 = _5.0;
_15 = RET > RET;
_5.2 = 15_u8;
_4 = _5.0;
_5.2 = 220_u8;
_16 = [_5.2,_5.2,_5.2];
_17.0 = _11 >> _11;
_3 = _1;
_19 = 10018510029760196956_u64 as isize;
_17.0 = 2984221940_u32 as i16;
_4 = _5.0;
_20.0 = _11;
_7 = [_1,_3,_15,_15,_1];
_8.fld0 = _16;
_22 = core::ptr::addr_of_mut!(_3);
_5.2 = (-89058737983528956556323223098079204247_i128) as u8;
Goto(bb7)
}
bb10 = {
place!(Field::<u128>(Variant((*_24), 1), 0)) = 316025215240676609761079495939683094955_u128 >> Field::<u8>(Variant((*_14), 1), 3);
place!(Field::<i128>(Variant((*_24), 1), 1)) = !_23.fld0.fld1;
match _23.fld0.fld4 {
0 => bb4,
1 => bb6,
2 => bb11,
3 => bb12,
1036635528564358067 => bb14,
_ => bb13
}
}
bb11 = {
RET = !15088_u16;
Goto(bb3)
}
bb12 = {
_1 = !_3;
_4 = _5.0;
_11 = (-31490_i16) - 15238_i16;
_5.2 = 1754337716568085051_i64 as u8;
_5.0 = _4;
_4 = _5.0;
_15 = RET > RET;
_5.2 = 15_u8;
_4 = _5.0;
_5.2 = 220_u8;
_16 = [_5.2,_5.2,_5.2];
_17.0 = _11 >> _11;
_3 = _1;
_19 = 10018510029760196956_u64 as isize;
_17.0 = 2984221940_u32 as i16;
_4 = _5.0;
_20.0 = _11;
_7 = [_1,_3,_15,_15,_1];
_8.fld0 = _16;
_22 = core::ptr::addr_of_mut!(_3);
_5.2 = (-89058737983528956556323223098079204247_i128) as u8;
Goto(bb7)
}
bb13 = {
_5.2 = !179_u8;
_3 = !_1;
_3 = _1 & _1;
_1 = !_3;
RET = (-907834949_i32) as u16;
Goto(bb4)
}
bb14 = {
place!(Field::<i128>(Variant((*_14), 1), 1)) = _23.fld0.fld1 & _23.fld0.fld1;
place!(Field::<u128>(Variant((*_24), 1), 0)) = _23.fld0.fld3 as u128;
place!(Field::<u8>(Variant((*_24), 1), 3)) = !_5.2;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 17_usize, Move(_17), 4_usize, Move(_4), 7_usize, Move(_7), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 1_usize, Move(_1), 20_usize, Move(_20), 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: u16,mut _5: bool,mut _6: u16,mut _7: bool,mut _8: u16,mut _9: u16,mut _10: u16,mut _11: u16) -> char {
mir! {
type RET = char;
let _12: [i128; 3];
let _13: i8;
let _14: Adt25;
let _15: u64;
let _16: *mut *const Adt25;
let _17: (char, Adt56, u8);
let _18: Adt51;
let _19: isize;
let _20: ((Adt22, *const Adt25, [bool; 5], (&'static usize, &'static Adt18)), &'static *mut i64);
let _21: f64;
let _22: i8;
let _23: (((Adt22, *const Adt25, [bool; 5], (&'static usize, &'static Adt18)), &'static *mut i64),);
let _24: u16;
let _25: &'static u128;
let _26: [bool; 2];
let _27: [i16; 5];
let _28: bool;
let _29: [i8; 8];
let _30: i64;
let _31: bool;
let _32: i64;
let _33: *const Adt25;
let _34: *const (&'static usize, &'static Adt18);
let _35: &'static i8;
let _36: u128;
let _37: [i16; 5];
let _38: Adt76;
let _39: [bool; 2];
let _40: i16;
let _41: i16;
let _42: u16;
let _43: char;
let _44: bool;
let _45: ();
let _46: ();
{
_5 = !_7;
_2 = !_5;
Goto(bb1)
}
bb1 = {
_13 = (-34_i8);
_12 = [145027881447989439376555658692508231675_i128,162533602600907306781100596756449093189_i128,(-138770794933939845401176527520410379111_i128)];
_11 = 323215857_i32 as u16;
_5 = _2;
_8 = (-48990307157516204184738479389263772918_i128) as u16;
_8 = '\u{b6763}' as u16;
RET = '\u{a708}';
_9 = 372_i16 as u16;
RET = '\u{87314}';
_14.fld3.fld0 = RET < RET;
_14.fld3.fld3 = 10189662090672087549_u64 as u16;
_3 = _14.fld3.fld0;
_7 = !_14.fld3.fld0;
_14.fld3.fld4 = 8677354963553359835_usize as u64;
_14.fld3.fld1 = 15233578541195458503326968229471917154_i128;
_14.fld3.fld4 = 7651906299262769642_u64;
Goto(bb2)
}
bb2 = {
_7 = !_5;
_3 = _2;
Goto(bb3)
}
bb3 = {
_14.fld3.fld2 = _9 as isize;
_5 = _14.fld3.fld0;
_3 = _14.fld3.fld4 != _14.fld3.fld4;
_12 = [_14.fld3.fld1,_14.fld3.fld1,_14.fld3.fld1];
_17.1.fld0 = [178_u8,194_u8,63_u8];
RET = '\u{8e87e}';
_14.fld1 = RET;
_20.0.0.fld2 = _14.fld3.fld2;
_20.0.3.1 = &_18.fld1;
_18.fld4 = [_14.fld3.fld0,_1,_14.fld3.fld0,_5,_7];
_14.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_17.2 = 174_u8;
Goto(bb4)
}
bb4 = {
_17.0 = RET;
_1 = _17.2 == _17.2;
match _14.fld3.fld4 {
7651906299262769642 => bb6,
_ => bb5
}
}
bb5 = {
_13 = (-34_i8);
_12 = [145027881447989439376555658692508231675_i128,162533602600907306781100596756449093189_i128,(-138770794933939845401176527520410379111_i128)];
_11 = 323215857_i32 as u16;
_5 = _2;
_8 = (-48990307157516204184738479389263772918_i128) as u16;
_8 = '\u{b6763}' as u16;
RET = '\u{a708}';
_9 = 372_i16 as u16;
RET = '\u{87314}';
_14.fld3.fld0 = RET < RET;
_14.fld3.fld3 = 10189662090672087549_u64 as u16;
_3 = _14.fld3.fld0;
_7 = !_14.fld3.fld0;
_14.fld3.fld4 = 8677354963553359835_usize as u64;
_14.fld3.fld1 = 15233578541195458503326968229471917154_i128;
_14.fld3.fld4 = 7651906299262769642_u64;
Goto(bb2)
}
bb6 = {
_20.1 = &_14.fld0;
_14.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_20.0.0.fld4 = _14.fld3.fld4 + _14.fld3.fld4;
_23.0.0.0.fld0 = _14.fld3.fld0;
_20.0.0.fld1 = !_14.fld3.fld1;
_18.fld0.fld3 = _9 - _10;
_23.0.0.0.fld1 = -_20.0.0.fld1;
_19 = _14.fld3.fld2 - _20.0.0.fld2;
_18.fld3 = [316067853_u32,2638963916_u32,1090773871_u32];
_11 = _9 | _14.fld3.fld3;
_14.fld3.fld0 = _1;
_7 = _14.fld3.fld0;
match _14.fld3.fld4 {
0 => bb1,
1 => bb4,
7651906299262769642 => bb8,
_ => bb7
}
}
bb7 = {
_7 = !_5;
_3 = _2;
Goto(bb3)
}
bb8 = {
_23.0.0.2 = [_3,_7,_7,_5,_23.0.0.0.fld0];
_17.0 = _14.fld1;
_23.0.0.0 = _14.fld3;
match _23.0.0.0.fld1 {
0 => bb5,
15233578541195458503326968229471917154 => bb10,
_ => bb9
}
}
bb9 = {
_13 = (-34_i8);
_12 = [145027881447989439376555658692508231675_i128,162533602600907306781100596756449093189_i128,(-138770794933939845401176527520410379111_i128)];
_11 = 323215857_i32 as u16;
_5 = _2;
_8 = (-48990307157516204184738479389263772918_i128) as u16;
_8 = '\u{b6763}' as u16;
RET = '\u{a708}';
_9 = 372_i16 as u16;
RET = '\u{87314}';
_14.fld3.fld0 = RET < RET;
_14.fld3.fld3 = 10189662090672087549_u64 as u16;
_3 = _14.fld3.fld0;
_7 = !_14.fld3.fld0;
_14.fld3.fld4 = 8677354963553359835_usize as u64;
_14.fld3.fld1 = 15233578541195458503326968229471917154_i128;
_14.fld3.fld4 = 7651906299262769642_u64;
Goto(bb2)
}
bb10 = {
_14.fld0 = core::ptr::addr_of_mut!(_18.fld2);
_1 = !_3;
_18.fld3 = [3466271510_u32,2441493872_u32,609856386_u32];
_20.0.0.fld3 = !_4;
_20.0.2 = [_14.fld3.fld0,_14.fld3.fld0,_3,_3,_23.0.0.0.fld0];
_26 = [_7,_7];
_20.1 = &_14.fld0;
_20.0.1 = core::ptr::addr_of!(_14);
_28 = !_7;
_17.2 = !102_u8;
match _10 {
52809 => bb11,
_ => bb2
}
}
bb11 = {
RET = _14.fld1;
_15 = _20.0.0.fld4;
_17.2 = 10_u8;
Goto(bb12)
}
bb12 = {
_22 = _17.2 as i8;
_14.fld1 = RET;
_23.0.0.3.1 = &_18.fld1;
_12 = [_23.0.0.0.fld1,_23.0.0.0.fld1,_20.0.0.fld1];
_18.fld0.fld2 = RET as isize;
_30 = 3343241522767803273_i64 + (-6735574473516041749_i64);
_18.fld0.fld4 = _23.0.0.0.fld2 as u64;
_14.fld3 = Adt22 { fld0: _1,fld1: _23.0.0.0.fld1,fld2: _19,fld3: _6,fld4: _15 };
_33 = Move(_20.0.1);
_18.fld0.fld3 = _17.2 as u16;
_14.fld2 = _20.0.0.fld3 as i128;
_18.fld0.fld3 = _11 ^ _8;
_28 = _1;
_23.0.0.2 = [_1,_1,_2,_28,_1];
_32 = 5_usize as i64;
_20.0.0 = Adt22 { fld0: _3,fld1: _23.0.0.0.fld1,fld2: _23.0.0.0.fld2,fld3: _18.fld0.fld3,fld4: _18.fld0.fld4 };
_17.0 = RET;
_38.fld2 = core::ptr::addr_of!(_14);
Call(_38.fld5.2 = fn2(Move(_17), _18.fld0.fld3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_9 = _10;
_35 = &_22;
_23.0.0.3.1 = &_18.fld1;
_25 = &_36;
_18.fld2 = 656119159_u32 as i64;
_23.0.0.1 = Move(_38.fld2);
_33 = core::ptr::addr_of!(_14);
(*_33).fld2 = _23.0.0.0.fld1 * _14.fld3.fld1;
(*_33).fld3.fld3 = !_20.0.0.fld3;
_18.fld4 = [_5,(*_33).fld3.fld0,(*_33).fld3.fld0,_7,_7];
_16 = core::ptr::addr_of_mut!(_20.0.1);
_17.0 = RET;
_17.2 = !_38.fld5.2;
_14.fld3.fld3 = _20.0.0.fld3;
_21 = (*_33).fld2 as f64;
_20.0.0 = (*_33).fld3;
match (*_33).fld3.fld1 {
0 => bb14,
1 => bb15,
15233578541195458503326968229471917154 => bb17,
_ => bb16
}
}
bb14 = {
_13 = (-34_i8);
_12 = [145027881447989439376555658692508231675_i128,162533602600907306781100596756449093189_i128,(-138770794933939845401176527520410379111_i128)];
_11 = 323215857_i32 as u16;
_5 = _2;
_8 = (-48990307157516204184738479389263772918_i128) as u16;
_8 = '\u{b6763}' as u16;
RET = '\u{a708}';
_9 = 372_i16 as u16;
RET = '\u{87314}';
_14.fld3.fld0 = RET < RET;
_14.fld3.fld3 = 10189662090672087549_u64 as u16;
_3 = _14.fld3.fld0;
_7 = !_14.fld3.fld0;
_14.fld3.fld4 = 8677354963553359835_usize as u64;
_14.fld3.fld1 = 15233578541195458503326968229471917154_i128;
_14.fld3.fld4 = 7651906299262769642_u64;
Goto(bb2)
}
bb15 = {
_17.0 = RET;
_1 = _17.2 == _17.2;
match _14.fld3.fld4 {
7651906299262769642 => bb6,
_ => bb5
}
}
bb16 = {
_13 = (-34_i8);
_12 = [145027881447989439376555658692508231675_i128,162533602600907306781100596756449093189_i128,(-138770794933939845401176527520410379111_i128)];
_11 = 323215857_i32 as u16;
_5 = _2;
_8 = (-48990307157516204184738479389263772918_i128) as u16;
_8 = '\u{b6763}' as u16;
RET = '\u{a708}';
_9 = 372_i16 as u16;
RET = '\u{87314}';
_14.fld3.fld0 = RET < RET;
_14.fld3.fld3 = 10189662090672087549_u64 as u16;
_3 = _14.fld3.fld0;
_7 = !_14.fld3.fld0;
_14.fld3.fld4 = 8677354963553359835_usize as u64;
_14.fld3.fld1 = 15233578541195458503326968229471917154_i128;
_14.fld3.fld4 = 7651906299262769642_u64;
Goto(bb2)
}
bb17 = {
_20.0.0.fld4 = _13 as u64;
_23.0.0.0.fld0 = !_14.fld3.fld0;
_20.0.0.fld3 = _14.fld3.fld3 >> _17.2;
_14.fld3.fld3 = _20.0.0.fld3;
(*_33).fld3.fld1 = _23.0.0.0.fld1 >> _17.2;
_34 = core::ptr::addr_of!(_20.0.3);
_29 = [(*_35),_13,_13,(*_35),(*_35),_13,(*_35),(*_35)];
_20.0.0.fld4 = _17.2 as u64;
(*_33).fld4 = 10138_i16 << _20.0.0.fld3;
(*_33).fld3.fld3 = !_20.0.0.fld3;
_35 = &(*_35);
(*_33).fld2 = _14.fld3.fld1;
_20.0.0.fld0 = _14.fld3.fld0 | (*_33).fld3.fld0;
_10 = !_23.0.0.0.fld3;
_42 = (*_33).fld2 as u16;
(*_33).fld3.fld1 = -_14.fld2;
_23.0.0.3.1 = &_38.fld1;
_38.fld5.1.fld0 = [_38.fld5.2,_17.2,_17.2];
_38.fld1 = Adt18::Variant0 { fld0: _21,fld1: _32 };
_2 = !_5;
_18.fld3 = [778142125_u32,1030087998_u32,1389797698_u32];
_32 = _30;
(*_33).fld3.fld1 = -_14.fld2;
Goto(bb18)
}
bb18 = {
Call(_45 = dump_var(1_usize, 30_usize, Move(_30), 13_usize, Move(_13), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(1_usize, 9_usize, Move(_9), 6_usize, Move(_6), 5_usize, Move(_5), 42_usize, Move(_42)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(1_usize, 32_usize, Move(_32), 11_usize, Move(_11), 10_usize, Move(_10), 46_usize, _46), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: (char, Adt56, u8),mut _2: u16) -> u8 {
mir! {
type RET = u8;
let _3: u8;
let _4: f32;
let _5: isize;
let _6: [u8; 3];
let _7: i32;
let _8: isize;
let _9: i32;
let _10: (i16,);
let _11: [i8; 8];
let _12: u32;
let _13: u16;
let _14: Adt56;
let _15: i16;
let _16: &'static [u8; 3];
let _17: i32;
let _18: isize;
let _19: u128;
let _20: char;
let _21: [u32; 7];
let _22: &'static f64;
let _23: i32;
let _24: [i64; 1];
let _25: f64;
let _26: f64;
let _27: Adt51;
let _28: f32;
let _29: ();
let _30: ();
{
RET = _1.2;
RET = _1.2;
RET = !_1.2;
RET = _1.2 * _1.2;
_1.1.fld0 = [_1.2,_1.2,RET];
_3 = !RET;
_5 = 9223372036854775807_isize;
_1.2 = _3;
_2 = 116_i8 as u16;
_4 = 23555_i16 as f32;
_1.2 = !_3;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9223372036854775807 => bb6,
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
RET = _3;
RET = !_1.2;
_4 = (-1030345713_i32) as f32;
RET = _1.2;
_2 = 27306_u16;
RET = !_1.2;
RET = _3 - _1.2;
_1.2 = !RET;
_1.1.fld0 = [RET,RET,_1.2];
_1.1.fld0 = [RET,RET,_1.2];
_7 = (-1348749808_i32) - 1991292865_i32;
_6 = _1.1.fld0;
_4 = 1837699805081752405_u64 as f32;
_6 = [_3,_1.2,_1.2];
RET = 7_usize as u8;
RET = _5 as u8;
RET = _1.2 * _1.2;
_1.0 = '\u{10197d}';
_1.0 = '\u{9ed17}';
_1.0 = '\u{2e499}';
Goto(bb7)
}
bb7 = {
_3 = !RET;
_8 = _5;
_8 = !_5;
_2 = !23123_u16;
_6 = _1.1.fld0;
_4 = _8 as f32;
_1.0 = '\u{2e9da}';
_11 = [(-94_i8),104_i8,(-28_i8),(-89_i8),(-2_i8),(-96_i8),0_i8,(-14_i8)];
_6 = [_3,RET,RET];
_7 = !(-538520850_i32);
_1.1.fld0 = [_3,_3,_3];
_12 = 4213602539_u32 ^ 3361615138_u32;
_1.1.fld0 = [RET,_1.2,_3];
_10 = ((-30702_i16),);
_12 = 257005763_u32 << _8;
_9 = !_7;
_4 = RET as f32;
RET = _3;
Goto(bb8)
}
bb8 = {
_1.1.fld0 = [RET,_1.2,_1.2];
_9 = _3 as i32;
_1.1 = Adt56 { fld0: _6 };
_8 = _5 + _5;
_1.2 = !RET;
_1.1 = Adt56 { fld0: _6 };
_12 = 3931553618_u32 | 1101691924_u32;
_13 = (-6626726370158736552_i64) as u16;
_8 = _9 as isize;
RET = (-48397160254787052697188197158796607921_i128) as u8;
_1.1.fld0 = _6;
_14.fld0 = [_3,_1.2,_1.2];
_11 = [2_i8,(-82_i8),66_i8,37_i8,(-79_i8),25_i8,93_i8,76_i8];
_12 = !3773668271_u32;
_12 = 26126697_u32 ^ 609277213_u32;
_14 = Adt56 { fld0: _1.1.fld0 };
_6 = [_1.2,_3,_3];
_12 = 273016129_u32 & 3927241905_u32;
_15 = _8 as i16;
_10.0 = _15;
_1 = ('\u{b3ea6}', Move(_14), _3);
_4 = 96_i8 as f32;
_4 = 2522974199168613156_usize as f32;
RET = _3;
_1.1 = Adt56 { fld0: _6 };
_6 = _1.1.fld0;
_14 = Adt56 { fld0: _1.1.fld0 };
match _5 {
0 => bb1,
1 => bb5,
9223372036854775807 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_14 = Move(_1.1);
_12 = !1374987267_u32;
RET = 15_i8 as u8;
_5 = _8 & _8;
_9 = _7 & _7;
_15 = _10.0 - _10.0;
_2 = !_13;
_9 = _7;
_10.0 = RET as i16;
_10 = (_15,);
_1.0 = '\u{e6dcb}';
_1.1.fld0 = [_1.2,_3,_3];
_1.2 = _5 as u8;
_1.1.fld0 = [_3,_3,_3];
_14.fld0 = _6;
_9 = 1455166961037340099_i64 as i32;
RET = false as u8;
_10 = (_15,);
_15 = _10.0;
RET = 120924636212073031618390152922736564548_u128 as u8;
_14 = Adt56 { fld0: _1.1.fld0 };
Call(_18 = fn3(Move(_1), _14.fld0, _8, _15, _10.0, _10.0, _5, _11, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9 = _7;
_4 = 6168538726146775181_u64 as f32;
_16 = &_6;
_1.1 = Move(_14);
_18 = '\u{b1950}' as isize;
_14.fld0 = [_3,_3,RET];
_9 = _7;
_5 = _8 + _8;
_17 = _7;
RET = 11227694154974822904_u64 as u8;
_4 = _12 as f32;
_19 = !12736122657429828445299738515928813702_u128;
_14 = Adt56 { fld0: (*_16) };
Call(RET = fn4(Move(_16), _14.fld0, _10, _6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = Adt56 { fld0: _1.1.fld0 };
RET = true as u8;
_6 = [_3,_3,_3];
_1.0 = '\u{be06c}';
_3 = !RET;
_16 = &_14.fld0;
_7 = _17;
_4 = 8580530569246173857_i64 as f32;
_18 = _8 ^ _8;
_18 = _3 as isize;
_1 = ('\u{56261}', Move(_14), RET);
_7 = _13 as i32;
_4 = _12 as f32;
_9 = _7 ^ _7;
Call(RET = fn12(_1.0, _1.2, _6, _1.0, _11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_17 = _9;
_19 = !193717465176016499846751440069152633934_u128;
_19 = 243329536053107682526792287704812713472_u128;
_11 = [34_i8,37_i8,45_i8,(-3_i8),(-77_i8),71_i8,(-75_i8),102_i8];
_21 = [_12,_12,_12,_12,_12,_12,_12];
_14 = Adt56 { fld0: _1.1.fld0 };
_17 = _9 * _9;
_3 = RET;
_1.1 = Adt56 { fld0: _14.fld0 };
_20 = _1.0;
_5 = _8;
_1.1 = Adt56 { fld0: _14.fld0 };
_14.fld0 = [RET,RET,_3];
_19 = !131940753463741417643768077729149981757_u128;
_24 = [5374521431753918931_i64];
_1.1 = Adt56 { fld0: _14.fld0 };
_3 = RET & RET;
_19 = 262993077745515889539982882033061185076_u128 | 175560689034824652538427292487473007057_u128;
_7 = _17;
Goto(bb14)
}
bb14 = {
Call(_29 = dump_var(2_usize, 11_usize, Move(_11), 18_usize, Move(_18), 21_usize, Move(_21), 19_usize, Move(_19)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_29 = dump_var(2_usize, 8_usize, Move(_8), 10_usize, Move(_10), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 9_usize, Move(_9), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: (char, Adt56, u8),mut _2: [u8; 3],mut _3: isize,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: isize,mut _8: [i8; 8],mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: Adt51;
let _11: [i8; 8];
let _12: ();
let _13: ();
{
_1.0 = '\u{fd512}';
RET = _7;
RET = _9 ^ _9;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(3_usize, 7_usize, Move(_7), 2_usize, Move(_2), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: &'static [u8; 3],mut _2: [u8; 3],mut _3: (i16,),mut _4: [u8; 3]) -> u8 {
mir! {
type RET = u8;
let _5: [usize; 8];
let _6: [i16; 5];
let _7: &'static Adt25;
let _8: u8;
let _9: isize;
let _10: *const *mut (Adt25, *const *mut i64, u32, i16);
let _11: [i8; 8];
let _12: isize;
let _13: *const i32;
let _14: i16;
let _15: (((Adt22, *const Adt25, [bool; 5], (&'static usize, &'static Adt18)), &'static *mut i64),);
let _16: f32;
let _17: u32;
let _18: *mut f32;
let _19: i128;
let _20: Adt51;
let _21: (i128,);
let _22: usize;
let _23: [u32; 7];
let _24: [i8; 8];
let _25: [i16; 5];
let _26: i16;
let _27: i128;
let _28: [i64; 1];
let _29: isize;
let _30: f32;
let _31: i16;
let _32: [bool; 2];
let _33: Adt25;
let _34: isize;
let _35: isize;
let _36: ();
let _37: ();
{
_4 = [250_u8,64_u8,234_u8];
RET = !181_u8;
RET = 4_usize as u8;
_4 = [RET,RET,RET];
_1 = &_2;
RET = 100_u8;
RET = (-44_i8) as u8;
_4 = [RET,RET,RET];
_4 = [RET,RET,RET];
_5 = [5_usize,7_usize,9358140414377210528_usize,1_usize,3_usize,14988258792398242422_usize,0_usize,9966887694940079121_usize];
_5 = [4_usize,14205151908822134780_usize,6_usize,4137172120241064682_usize,8576319005062893625_usize,9124810606948419993_usize,15474646804431212762_usize,2459329293974698317_usize];
RET = (-34_i8) as u8;
_1 = &_4;
_3.0 = -26684_i16;
_3.0 = !(-14112_i16);
_3 = ((-9572_i16),);
Goto(bb1)
}
bb1 = {
_1 = &(*_1);
_3 = (14193_i16,);
_8 = 2798820818_u32 as u8;
_3.0 = (-7658_i16);
RET = _8 * _8;
_1 = &_4;
_6 = [_3.0,_3.0,_3.0,_3.0,_3.0];
_4 = _2;
_9 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_4 = [_8,_8,_8];
_1 = &_2;
_5 = [6_usize,16501844185704202467_usize,5_usize,1_usize,0_usize,4793738092588688138_usize,4585487143763937646_usize,0_usize];
RET = !_8;
_2 = [RET,_8,_8];
RET = 57029_u16 as u8;
_3.0 = (-4641_i16);
_1 = &_4;
Goto(bb2)
}
bb2 = {
_11 = [119_i8,51_i8,(-81_i8),(-100_i8),(-36_i8),60_i8,46_i8,33_i8];
_5 = [121590201485575211_usize,453624778077368044_usize,5_usize,4987964809379198987_usize,10706160527167722585_usize,2987182772120828486_usize,7259316352380930860_usize,4_usize];
_1 = &(*_1);
_9 = 68_i8 as isize;
_12 = true as isize;
RET = (-1115405745771451598_i64) as u8;
_2 = [_8,_8,_8];
_11 = [118_i8,61_i8,109_i8,80_i8,(-12_i8),(-24_i8),(-112_i8),(-73_i8)];
match _3.0 {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463463374607431768206815 => bb6,
_ => bb5
}
}
bb3 = {
_1 = &(*_1);
_3 = (14193_i16,);
_8 = 2798820818_u32 as u8;
_3.0 = (-7658_i16);
RET = _8 * _8;
_1 = &_4;
_6 = [_3.0,_3.0,_3.0,_3.0,_3.0];
_4 = _2;
_9 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_4 = [_8,_8,_8];
_1 = &_2;
_5 = [6_usize,16501844185704202467_usize,5_usize,1_usize,0_usize,4793738092588688138_usize,4585487143763937646_usize,0_usize];
RET = !_8;
_2 = [RET,_8,_8];
RET = 57029_u16 as u8;
_3.0 = (-4641_i16);
_1 = &_4;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_15.0.0.2 = [false,true,false,false,true];
match _3.0 {
0 => bb1,
1 => bb5,
340282366920938463463374607431768206815 => bb7,
_ => bb3
}
}
bb7 = {
_14 = '\u{86c57}' as i16;
_16 = _3.0 as f32;
_4 = [_8,_8,_8];
_15.0.0.0 = Adt22 { fld0: true,fld1: 95515346044194123194935945728052193711_i128,fld2: _9,fld3: 17088_u16,fld4: 10324577471827952947_u64 };
_15.0.0.0.fld4 = !9284495667353551955_u64;
_15.0.0.0.fld4 = 14955011235075686135_u64 | 9052620592796276290_u64;
_12 = -_9;
Call(_14 = fn5(_15.0.0.0.fld1, _15.0.0.0.fld1, _11, _5, _15.0.0.2, _15.0.0.0.fld1, _5, _11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = _9 as u32;
_15.0.0.0 = Adt22 { fld0: false,fld1: 43981940524367013174812785503755308660_i128,fld2: _12,fld3: 22634_u16,fld4: 14054799271824532421_u64 };
_3 = (_14,);
_9 = !_15.0.0.0.fld2;
_9 = _12 >> _14;
_2 = [_8,RET,_8];
_14 = _3.0;
RET = !_8;
_2 = [_8,RET,RET];
_4 = [_8,RET,_8];
RET = !_8;
_1 = &_4;
_11 = [(-26_i8),(-101_i8),106_i8,(-89_i8),(-77_i8),(-95_i8),29_i8,63_i8];
_11 = [38_i8,(-94_i8),44_i8,64_i8,(-82_i8),(-80_i8),64_i8,24_i8];
_18 = core::ptr::addr_of_mut!(_16);
_3 = (_14,);
_15.0.0.2 = [_15.0.0.0.fld0,_15.0.0.0.fld0,_15.0.0.0.fld0,_15.0.0.0.fld0,_15.0.0.0.fld0];
_15.0.0.0.fld1 = (-118950108244200205857400058899766079784_i128);
_15.0.0.0 = Adt22 { fld0: true,fld1: 142167762401148584942283463060927465104_i128,fld2: _12,fld3: 4737_u16,fld4: 5720510526445198382_u64 };
_14 = 1883168152_i32 as i16;
_15.0.0.0 = Adt22 { fld0: false,fld1: 89155280141528195618357054410150302621_i128,fld2: _9,fld3: 28581_u16,fld4: 12939268485690324759_u64 };
_6 = [_3.0,_14,_14,_14,_14];
_4 = _2;
_9 = -_12;
_20.fld0.fld4 = _15.0.0.0.fld4;
match _15.0.0.0.fld3 {
0 => bb6,
1 => bb4,
2 => bb3,
28581 => bb10,
_ => bb9
}
}
bb9 = {
_11 = [119_i8,51_i8,(-81_i8),(-100_i8),(-36_i8),60_i8,46_i8,33_i8];
_5 = [121590201485575211_usize,453624778077368044_usize,5_usize,4987964809379198987_usize,10706160527167722585_usize,2987182772120828486_usize,7259316352380930860_usize,4_usize];
_1 = &(*_1);
_9 = 68_i8 as isize;
_12 = true as isize;
RET = (-1115405745771451598_i64) as u8;
_2 = [_8,_8,_8];
_11 = [118_i8,61_i8,109_i8,80_i8,(-12_i8),(-24_i8),(-112_i8),(-73_i8)];
match _3.0 {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463463374607431768206815 => bb6,
_ => bb5
}
}
bb10 = {
_20.fld0.fld1 = !_15.0.0.0.fld1;
_20.fld0.fld2 = _14 as isize;
RET = 7742576743827221_i64 as u8;
_16 = 320520027096407462576383506268768576363_u128 as f32;
_15.0.0.0.fld4 = _20.fld0.fld4 - _20.fld0.fld4;
_20.fld2 = 7660397741086902573_i64;
_15.0.0.0.fld3 = 25343_u16 ^ 19312_u16;
_20.fld0.fld3 = _15.0.0.0.fld3 << _15.0.0.0.fld3;
_21 = (_20.fld0.fld1,);
_15.0.0.2 = [_15.0.0.0.fld0,_15.0.0.0.fld0,_15.0.0.0.fld0,_15.0.0.0.fld0,_15.0.0.0.fld0];
Goto(bb11)
}
bb11 = {
_24 = [29_i8,(-120_i8),93_i8,(-74_i8),122_i8,(-83_i8),(-45_i8),(-112_i8)];
_20.fld0.fld3 = _20.fld0.fld4 as u16;
Call(_8 = core::intrinsics::transmute(_15.0.0.0.fld0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_15.0.0.0 = Adt22 { fld0: true,fld1: _21.0,fld2: _9,fld3: _20.fld0.fld3,fld4: _20.fld0.fld4 };
_23 = [_17,_17,_17,_17,_17,_17,_17];
_15.0.0.0 = Adt22 { fld0: true,fld1: _20.fld0.fld1,fld2: _9,fld3: _20.fld0.fld3,fld4: _20.fld0.fld4 };
_20.fld0.fld3 = _15.0.0.0.fld3;
_15.0.0.3.0 = &_22;
_15.0.0.3.0 = &_22;
Call((*_18) = fn11(_15.0.0.0, _15.0.0.0, _15.0.0.0, _3.0, _15.0.0.0.fld0, _15.0.0.0.fld0, _20.fld0.fld4, _15.0.0.0.fld0, _15.0.0.0, _20.fld0.fld4, _15.0.0.0.fld1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20.fld2 = !(-6341409948202949580_i64);
_8 = RET;
_21.0 = _20.fld0.fld1;
_1 = &_4;
_16 = _3.0 as f32;
_20.fld0 = Adt22 { fld0: _15.0.0.0.fld0,fld1: _21.0,fld2: _12,fld3: _15.0.0.0.fld3,fld4: _15.0.0.0.fld4 };
(*_18) = _20.fld2 as f32;
_26 = _15.0.0.0.fld3 as i16;
(*_18) = _15.0.0.0.fld4 as f32;
_15.0.0.3.0 = &_22;
Goto(bb14)
}
bb14 = {
(*_18) = _20.fld2 as f32;
_25 = [_14,_26,_14,_26,_3.0];
_27 = _20.fld2 as i128;
_20.fld2 = 7366598123019204123_i64;
_3.0 = _15.0.0.0.fld1 as i16;
_3.0 = !_26;
_22 = _20.fld0.fld3 as usize;
_5 = [_22,_22,_22,_22,_22,_22,_22,_22];
RET = !_8;
_14 = _26 * _3.0;
_20.fld0.fld4 = !_15.0.0.0.fld4;
_34 = _15.0.0.0.fld2 - _20.fld0.fld2;
_21 = (_15.0.0.0.fld1,);
_11 = [(-56_i8),40_i8,15_i8,4_i8,88_i8,78_i8,(-89_i8),33_i8];
_11 = [0_i8,18_i8,(-53_i8),(-34_i8),(-122_i8),114_i8,(-53_i8),(-80_i8)];
_33.fld1 = '\u{6a698}';
_33.fld3.fld2 = _12 ^ _15.0.0.0.fld2;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(4_usize, 2_usize, Move(_2), 3_usize, Move(_3), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(4_usize, 23_usize, Move(_23), 34_usize, Move(_34), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(4_usize, 11_usize, Move(_11), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i128,mut _2: i128,mut _3: [i8; 8],mut _4: [usize; 8],mut _5: [bool; 5],mut _6: i128,mut _7: [usize; 8],mut _8: [i8; 8]) -> i16 {
mir! {
type RET = i16;
let _9: &'static Adt25;
let _10: [bool; 2];
let _11: f64;
let _12: Adt22;
let _13: i16;
let _14: *mut i16;
let _15: &'static u16;
let _16: bool;
let _17: Adt18;
let _18: bool;
let _19: *mut f32;
let _20: *const *mut (Adt25, *const *mut i64, u32, i16);
let _21: *mut &'static &'static f32;
let _22: (Adt22, *const Adt25, [bool; 5], (&'static usize, &'static Adt18));
let _23: *const *mut i64;
let _24: [i64; 1];
let _25: char;
let _26: u8;
let _27: [bool; 5];
let _28: i32;
let _29: usize;
let _30: usize;
let _31: (i128,);
let _32: isize;
let _33: *const *mut bool;
let _34: u8;
let _35: *const *mut &'static i8;
let _36: ();
let _37: ();
{
_7 = [3_usize,3553707865865508026_usize,7_usize,2_usize,8305854320184169758_usize,5_usize,0_usize,754860595115347491_usize];
_8 = _3;
Call(_7 = fn6(_6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 8865906206000820354_usize as i16;
_5 = [true,false,true,false,false];
RET = true as i16;
_12.fld0 = true & false;
_2 = (-34_i8) as i128;
_12.fld1 = _6;
_12.fld4 = 315911866041392911234166213974114775147_u128 as u64;
_4 = _7;
_4 = [9742580426503082815_usize,5_usize,2_usize,6_usize,3_usize,5673834748762423587_usize,17185828909751636046_usize,18018321730359742024_usize];
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
95515346044194123194935945728052193711 => bb7,
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
RET = 13974_i16 * (-30496_i16);
_4 = _7;
_10 = [_12.fld0,_12.fld0];
_12 = Adt22 { fld0: false,fld1: _6,fld2: 9223372036854775807_isize,fld3: 51020_u16,fld4: 2567990918803866713_u64 };
_12.fld1 = _6;
_8 = _3;
_14 = core::ptr::addr_of_mut!(RET);
_11 = _12.fld4 as f64;
_4 = [2563213908489870780_usize,12031504694601527234_usize,5_usize,1470870123593405313_usize,14832914100019879923_usize,9075790419966865398_usize,1_usize,4_usize];
_6 = (*_14) as i128;
_8 = [(-79_i8),(-127_i8),117_i8,(-88_i8),(-100_i8),82_i8,102_i8,57_i8];
_8 = [119_i8,70_i8,125_i8,52_i8,(-8_i8),(-101_i8),(-124_i8),(-34_i8)];
_4 = [2_usize,0_usize,5_usize,9376703587052318191_usize,17365388559394186761_usize,7445247110335375374_usize,5_usize,2_usize];
_2 = '\u{b5a3}' as i128;
_8 = [102_i8,66_i8,52_i8,73_i8,89_i8,(-16_i8),26_i8,104_i8];
RET = (-25723_i16);
_10 = [_12.fld0,_12.fld0];
_5 = [_12.fld0,_12.fld0,_12.fld0,_12.fld0,_12.fld0];
(*_14) = 334673481005372499871601190821850600377_u128 as i16;
_13 = !RET;
_7 = _4;
_12.fld4 = 10577738754203519056_u64;
Goto(bb8)
}
bb8 = {
_16 = _12.fld0;
RET = -_13;
_12.fld2 = (-22_isize) >> _12.fld3;
_8 = [16_i8,(-69_i8),(-14_i8),104_i8,19_i8,99_i8,70_i8,60_i8];
(*_14) = !_13;
_15 = &_12.fld3;
_5 = [_12.fld0,_12.fld0,_16,_16,_16];
_13 = _12.fld4 as i16;
_10 = [_12.fld0,_12.fld0];
_8 = _3;
_18 = !_16;
_12.fld2 = '\u{d4d4e}' as isize;
_13 = _12.fld3 as i16;
_13 = !(*_14);
_17 = Adt18::Variant2 { fld0: _12.fld4,fld1: _12.fld3,fld2: 65_u8,fld3: _11 };
_3 = [(-61_i8),(-115_i8),111_i8,(-101_i8),(-41_i8),61_i8,(-67_i8),76_i8];
_12.fld1 = -_1;
_16 = !_18;
_15 = &(*_15);
_3 = [(-66_i8),(-50_i8),104_i8,65_i8,(-87_i8),94_i8,(-47_i8),104_i8];
Call(_19 = fn9(), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12.fld3 = Field::<u16>(Variant(_17, 2), 1) % Field::<u16>(Variant(_17, 2), 1);
_12.fld2 = (-9223372036854775808_isize);
_15 = &_12.fld3;
_15 = &_12.fld3;
_5 = [_18,_12.fld0,_12.fld0,_12.fld0,_12.fld0];
_11 = Field::<f64>(Variant(_17, 2), 3);
place!(Field::<u8>(Variant(_17, 2), 2)) = 177_u8 ^ 43_u8;
_6 = _12.fld1 - _1;
_22.0.fld4 = _12.fld4 / _12.fld4;
_22.0 = Adt22 { fld0: _12.fld0,fld1: _1,fld2: _12.fld2,fld3: (*_15),fld4: _12.fld4 };
place!(Field::<u64>(Variant(_17, 2), 0)) = 148451305669226772_i64 as u64;
(*_14) = !_13;
_6 = _22.0.fld1 & _1;
_17 = Adt18::Variant0 { fld0: _11,fld1: 1783616418346678891_i64 };
_22.0.fld4 = '\u{3c6a1}' as u64;
_22.0.fld1 = (-583179161_i32) as i128;
_12.fld4 = _22.0.fld4 - _22.0.fld4;
_24 = [(-858944122681426785_i64)];
_12 = Adt22 { fld0: _18,fld1: _1,fld2: _22.0.fld2,fld3: _22.0.fld3,fld4: _22.0.fld4 };
_11 = Field::<f64>(Variant(_17, 0), 0) * Field::<f64>(Variant(_17, 0), 0);
place!(Field::<i64>(Variant(_17, 0), 1)) = (-7027577595992644474_i64);
_13 = RET;
place!(Field::<f64>(Variant(_17, 0), 0)) = _11;
Goto(bb10)
}
bb10 = {
_12.fld0 = !_22.0.fld0;
RET = _12.fld1 as i16;
_18 = _22.0.fld0 & _12.fld0;
_22.2 = [_16,_12.fld0,_18,_16,_12.fld0];
SetDiscriminant(_17, 1);
_6 = !_1;
_22.0.fld3 = _12.fld3 | _12.fld3;
RET = 181_u8 as i16;
place!(Field::<u8>(Variant(_17, 1), 3)) = _12.fld0 as u8;
_12.fld1 = _6;
_12.fld4 = _22.0.fld4 + _22.0.fld4;
(*_14) = _13 & _13;
place!(Field::<u32>(Variant(_17, 1), 2)) = !440571019_u32;
Goto(bb11)
}
bb11 = {
_12.fld3 = _6 as u16;
_12.fld1 = _1 ^ _6;
_12.fld3 = '\u{9dc7e}' as u16;
place!(Field::<u128>(Variant(_17, 1), 0)) = 243760687191081345037894873560327558059_u128 ^ 288230349118218705322213051070657138918_u128;
_12.fld3 = _22.0.fld3;
_7 = _4;
_14 = core::ptr::addr_of_mut!((*_14));
place!(Field::<u128>(Variant(_17, 1), 0)) = 334777568009532319684199531102527708239_u128 >> _22.0.fld3;
_18 = _12.fld0;
_12.fld4 = _22.0.fld4;
_12.fld4 = _22.0.fld4;
_22.3.1 = &_17;
_12 = Adt22 { fld0: _22.0.fld0,fld1: _6,fld2: _22.0.fld2,fld3: _22.0.fld3,fld4: _22.0.fld4 };
place!(Field::<i128>(Variant(_17, 1), 1)) = _6;
_12.fld2 = Field::<u32>(Variant(_17, 1), 2) as isize;
_2 = (-8206565922056452391_i64) as i128;
_29 = '\u{ff2a}' as usize;
_4 = _7;
_22.3.1 = &_17;
_26 = Field::<u8>(Variant(_17, 1), 3) ^ Field::<u8>(Variant(_17, 1), 3);
match _1 {
0 => bb6,
1 => bb9,
95515346044194123194935945728052193711 => bb12,
_ => bb4
}
}
bb12 = {
_24 = [8787052718268187707_i64];
_15 = &_12.fld3;
_22.0.fld1 = _1;
RET = -_13;
_15 = &_12.fld3;
_1 = _12.fld4 as i128;
_30 = _29 & _29;
_22.0.fld4 = _12.fld4;
place!(Field::<u32>(Variant(_17, 1), 2)) = 3271615779_u32;
_27 = [_12.fld0,_18,_22.0.fld0,_18,_22.0.fld0];
_27 = [_22.0.fld0,_22.0.fld0,_18,_16,_18];
_24 = [(-2617096040372112353_i64)];
match _22.0.fld1 {
0 => bb3,
1 => bb13,
2 => bb14,
95515346044194123194935945728052193711 => bb16,
_ => bb15
}
}
bb13 = {
_16 = _12.fld0;
RET = -_13;
_12.fld2 = (-22_isize) >> _12.fld3;
_8 = [16_i8,(-69_i8),(-14_i8),104_i8,19_i8,99_i8,70_i8,60_i8];
(*_14) = !_13;
_15 = &_12.fld3;
_5 = [_12.fld0,_12.fld0,_16,_16,_16];
_13 = _12.fld4 as i16;
_10 = [_12.fld0,_12.fld0];
_8 = _3;
_18 = !_16;
_12.fld2 = '\u{d4d4e}' as isize;
_13 = _12.fld3 as i16;
_13 = !(*_14);
_17 = Adt18::Variant2 { fld0: _12.fld4,fld1: _12.fld3,fld2: 65_u8,fld3: _11 };
_3 = [(-61_i8),(-115_i8),111_i8,(-101_i8),(-41_i8),61_i8,(-67_i8),76_i8];
_12.fld1 = -_1;
_16 = !_18;
_15 = &(*_15);
_3 = [(-66_i8),(-50_i8),104_i8,65_i8,(-87_i8),94_i8,(-47_i8),104_i8];
Call(_19 = fn9(), ReturnTo(bb9), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_16 = !_12.fld0;
_32 = _22.0.fld4 as isize;
_10 = [_16,_22.0.fld0];
_15 = &_12.fld3;
SetDiscriminant(_17, 0);
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(5_usize, 3_usize, Move(_3), 24_usize, Move(_24), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(5_usize, 2_usize, Move(_2), 26_usize, Move(_26), 16_usize, Move(_16), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(5_usize, 6_usize, Move(_6), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i128,mut _2: [i8; 8]) -> [usize; 8] {
mir! {
type RET = [usize; 8];
let _3: Adt56;
let _4: *const *const *mut i64;
let _5: &'static f64;
let _6: Adt76;
let _7: usize;
let _8: bool;
let _9: &'static u16;
let _10: [bool; 5];
let _11: *const *mut (Adt25, *const *mut i64, u32, i16);
let _12: *const Adt25;
let _13: ();
let _14: ();
{
_3.fld0 = [55_u8,137_u8,165_u8];
_3.fld0 = [165_u8,130_u8,204_u8];
_1 = (-121083454294760791129657624000886246103_i128);
_1 = 119405996797694743866935811530622803706_i128 - 131780985584292273938894697701245988355_i128;
RET = [17894999036803802131_usize,14266211333826083556_usize,4351685019454072206_usize,2_usize,12128326974612809200_usize,4910574357004950326_usize,2_usize,13437871236981502245_usize];
_2 = [(-84_i8),(-104_i8),99_i8,(-14_i8),0_i8,90_i8,(-84_i8),(-82_i8)];
RET = [1_usize,13538656409113670395_usize,3_usize,5_usize,14809989880793730162_usize,5_usize,1688705090018740911_usize,0_usize];
RET = [4_usize,4_usize,3456883460892440981_usize,3943491675555276521_usize,2_usize,13261046678224286608_usize,1_usize,3_usize];
RET = [1_usize,1_usize,4_usize,7754399391315708170_usize,7_usize,11341790808808415104_usize,0_usize,11247519629391722798_usize];
_2 = [(-12_i8),(-74_i8),(-61_i8),(-65_i8),106_i8,(-18_i8),(-16_i8),(-99_i8)];
RET = [332138039846623335_usize,2_usize,6_usize,4373820689215113619_usize,5968369089518270532_usize,1435730829967956371_usize,7_usize,6_usize];
RET = [94645539847004122_usize,16048779623867893483_usize,16526324800411894354_usize,17852752058241292623_usize,17078146292956225193_usize,1940339074633067316_usize,7_usize,2_usize];
RET = [1_usize,5_usize,12273897663335564212_usize,2816493416087470477_usize,6_usize,584082013030957621_usize,9774369308303940730_usize,6_usize];
RET = [9422243749784765223_usize,2601184908510738926_usize,7160172115099573420_usize,2_usize,7_usize,6_usize,3_usize,1_usize];
RET = [3_usize,1671550157115737051_usize,7715110994875283054_usize,8947313418719173856_usize,5673117454937661427_usize,6_usize,6_usize,6_usize];
RET = [9834948251516789118_usize,4_usize,128573592686775665_usize,1_usize,5406940584688619494_usize,5375006456434742090_usize,7_usize,14449723670500744992_usize];
_1 = (-62876575654626257984508348062306884065_i128);
RET = [3_usize,11280699687847250651_usize,2_usize,5_usize,9266161182129612843_usize,10490189865026369769_usize,3_usize,1_usize];
_2 = [(-85_i8),94_i8,(-26_i8),28_i8,3_i8,(-101_i8),108_i8,112_i8];
RET = [17548110201860360347_usize,12782425640457814857_usize,6095808265538612161_usize,3_usize,0_usize,11353794150339824020_usize,2_usize,4_usize];
_1 = !35484715459288175874726483732689999150_i128;
_3.fld0 = [218_u8,7_u8,213_u8];
Goto(bb1)
}
bb1 = {
RET = [6330479794821757248_usize,18239052333171690062_usize,1490015058072398974_usize,12412562407813039800_usize,904562254962370493_usize,3_usize,5911991596364882401_usize,3_usize];
_3.fld0 = [139_u8,127_u8,190_u8];
RET = [9520151746713653398_usize,6_usize,9741248326884373111_usize,7229722650320006716_usize,5_usize,6_usize,18217441093597867679_usize,2501583734526622503_usize];
_3.fld0 = [5_u8,193_u8,66_u8];
_1 = 92930630459781064054895612536910083064_i128 << 8841773711457401621_i64;
RET = [0_usize,8031460230573460802_usize,7693162728658749015_usize,15398726663891549636_usize,0_usize,2621567541249734971_usize,2_usize,3_usize];
_2 = [(-66_i8),100_i8,55_i8,118_i8,126_i8,80_i8,(-84_i8),(-77_i8)];
RET = [7344746065689052069_usize,1_usize,7519167576360331568_usize,8732053997840922459_usize,0_usize,9943309685576724826_usize,945729228589073344_usize,9077922149295564629_usize];
RET = [1_usize,1183987062169024468_usize,2_usize,6252085009902960903_usize,3998557497696190455_usize,15383910712617144519_usize,4_usize,11906457042200741296_usize];
Call(_1 = fn7(_3.fld0, _2, _2, Move(_3), RET, RET, RET, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [1_usize,15688089921225069401_usize,6_usize,11339860510135911018_usize,11211516174949108989_usize,6_usize,3_usize,4_usize];
_1 = 4028879098149916159_usize as i128;
_6.fld5.0 = '\u{10930a}';
_6.fld5.2 = _1 as u8;
RET = [5039196756824720701_usize,2671666125469636556_usize,13862199312162807041_usize,7341578482529620466_usize,1_usize,5_usize,6868308745258478279_usize,18060888911056893732_usize];
_3.fld0 = [_6.fld5.2,_6.fld5.2,_6.fld5.2];
_6.fld5.1 = Adt56 { fld0: _3.fld0 };
_6.fld5.1.fld0 = _3.fld0;
_7 = 16600404972921073307_usize | 14855082892324133178_usize;
_3.fld0 = _6.fld5.1.fld0;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld5 = ('\u{7aaa}', Move(_3), 145_u8);
_1 = -(-104236077681120815278367918637694886653_i128);
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_1 = (-375531763291073431316897742962775259_i128);
_6.fld5.2 = false as u8;
_6.fld5.2 = !232_u8;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld5 = ('\u{77236}', Move(_3), 134_u8);
Call(_1 = core::intrinsics::bswap((-70719199458337720636708572121722714939_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = (-927184676832956350_i64) as i128;
_7 = 7380419198949200696_u64 as usize;
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = 5094571433366212355_i64 as i128;
_6.fld1 = Adt18::Variant1 { fld0: 118455745727851106668343920896468697029_u128,fld1: _1,fld2: 187767012_u32,fld3: _6.fld5.2 };
match _6.fld5.2 {
0 => bb4,
1 => bb5,
134 => bb7,
_ => bb6
}
}
bb4 = {
RET = [1_usize,15688089921225069401_usize,6_usize,11339860510135911018_usize,11211516174949108989_usize,6_usize,3_usize,4_usize];
_1 = 4028879098149916159_usize as i128;
_6.fld5.0 = '\u{10930a}';
_6.fld5.2 = _1 as u8;
RET = [5039196756824720701_usize,2671666125469636556_usize,13862199312162807041_usize,7341578482529620466_usize,1_usize,5_usize,6868308745258478279_usize,18060888911056893732_usize];
_3.fld0 = [_6.fld5.2,_6.fld5.2,_6.fld5.2];
_6.fld5.1 = Adt56 { fld0: _3.fld0 };
_6.fld5.1.fld0 = _3.fld0;
_7 = 16600404972921073307_usize | 14855082892324133178_usize;
_3.fld0 = _6.fld5.1.fld0;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld5 = ('\u{7aaa}', Move(_3), 145_u8);
_1 = -(-104236077681120815278367918637694886653_i128);
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_1 = (-375531763291073431316897742962775259_i128);
_6.fld5.2 = false as u8;
_6.fld5.2 = !232_u8;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld5 = ('\u{77236}', Move(_3), 134_u8);
Call(_1 = core::intrinsics::bswap((-70719199458337720636708572121722714939_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = [6330479794821757248_usize,18239052333171690062_usize,1490015058072398974_usize,12412562407813039800_usize,904562254962370493_usize,3_usize,5911991596364882401_usize,3_usize];
_3.fld0 = [139_u8,127_u8,190_u8];
RET = [9520151746713653398_usize,6_usize,9741248326884373111_usize,7229722650320006716_usize,5_usize,6_usize,18217441093597867679_usize,2501583734526622503_usize];
_3.fld0 = [5_u8,193_u8,66_u8];
_1 = 92930630459781064054895612536910083064_i128 << 8841773711457401621_i64;
RET = [0_usize,8031460230573460802_usize,7693162728658749015_usize,15398726663891549636_usize,0_usize,2621567541249734971_usize,2_usize,3_usize];
_2 = [(-66_i8),100_i8,55_i8,118_i8,126_i8,80_i8,(-84_i8),(-77_i8)];
RET = [7344746065689052069_usize,1_usize,7519167576360331568_usize,8732053997840922459_usize,0_usize,9943309685576724826_usize,945729228589073344_usize,9077922149295564629_usize];
RET = [1_usize,1183987062169024468_usize,2_usize,6252085009902960903_usize,3998557497696190455_usize,15383910712617144519_usize,4_usize,11906457042200741296_usize];
Call(_1 = fn7(_3.fld0, _2, _2, Move(_3), RET, RET, RET, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_6.fld1 = Adt18::Variant1 { fld0: 137555934930617318845743680039836153581_u128,fld1: _1,fld2: 2886665863_u32,fld3: _6.fld5.2 };
_8 = !false;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = !186590852205542331963340953682252070335_u128;
place!(Field::<u32>(Variant(_6.fld1, 1), 2)) = 1464850300_u32;
_2 = [91_i8,(-61_i8),88_i8,(-75_i8),2_i8,(-10_i8),28_i8,(-34_i8)];
_6.fld3 = core::ptr::addr_of_mut!(_8);
_10 = [_8,_8,_8,_8,_8];
match Field::<u8>(Variant(_6.fld1, 1), 3) {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
134 => bb13,
_ => bb12
}
}
bb8 = {
Return()
}
bb9 = {
RET = [6330479794821757248_usize,18239052333171690062_usize,1490015058072398974_usize,12412562407813039800_usize,904562254962370493_usize,3_usize,5911991596364882401_usize,3_usize];
_3.fld0 = [139_u8,127_u8,190_u8];
RET = [9520151746713653398_usize,6_usize,9741248326884373111_usize,7229722650320006716_usize,5_usize,6_usize,18217441093597867679_usize,2501583734526622503_usize];
_3.fld0 = [5_u8,193_u8,66_u8];
_1 = 92930630459781064054895612536910083064_i128 << 8841773711457401621_i64;
RET = [0_usize,8031460230573460802_usize,7693162728658749015_usize,15398726663891549636_usize,0_usize,2621567541249734971_usize,2_usize,3_usize];
_2 = [(-66_i8),100_i8,55_i8,118_i8,126_i8,80_i8,(-84_i8),(-77_i8)];
RET = [7344746065689052069_usize,1_usize,7519167576360331568_usize,8732053997840922459_usize,0_usize,9943309685576724826_usize,945729228589073344_usize,9077922149295564629_usize];
RET = [1_usize,1183987062169024468_usize,2_usize,6252085009902960903_usize,3998557497696190455_usize,15383910712617144519_usize,4_usize,11906457042200741296_usize];
Call(_1 = fn7(_3.fld0, _2, _2, Move(_3), RET, RET, RET, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
RET = [6330479794821757248_usize,18239052333171690062_usize,1490015058072398974_usize,12412562407813039800_usize,904562254962370493_usize,3_usize,5911991596364882401_usize,3_usize];
_3.fld0 = [139_u8,127_u8,190_u8];
RET = [9520151746713653398_usize,6_usize,9741248326884373111_usize,7229722650320006716_usize,5_usize,6_usize,18217441093597867679_usize,2501583734526622503_usize];
_3.fld0 = [5_u8,193_u8,66_u8];
_1 = 92930630459781064054895612536910083064_i128 << 8841773711457401621_i64;
RET = [0_usize,8031460230573460802_usize,7693162728658749015_usize,15398726663891549636_usize,0_usize,2621567541249734971_usize,2_usize,3_usize];
_2 = [(-66_i8),100_i8,55_i8,118_i8,126_i8,80_i8,(-84_i8),(-77_i8)];
RET = [7344746065689052069_usize,1_usize,7519167576360331568_usize,8732053997840922459_usize,0_usize,9943309685576724826_usize,945729228589073344_usize,9077922149295564629_usize];
RET = [1_usize,1183987062169024468_usize,2_usize,6252085009902960903_usize,3998557497696190455_usize,15383910712617144519_usize,4_usize,11906457042200741296_usize];
Call(_1 = fn7(_3.fld0, _2, _2, Move(_3), RET, RET, RET, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_1 = (-927184676832956350_i64) as i128;
_7 = 7380419198949200696_u64 as usize;
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = 5094571433366212355_i64 as i128;
_6.fld1 = Adt18::Variant1 { fld0: 118455745727851106668343920896468697029_u128,fld1: _1,fld2: 187767012_u32,fld3: _6.fld5.2 };
match _6.fld5.2 {
0 => bb4,
1 => bb5,
134 => bb7,
_ => bb6
}
}
bb12 = {
RET = [1_usize,15688089921225069401_usize,6_usize,11339860510135911018_usize,11211516174949108989_usize,6_usize,3_usize,4_usize];
_1 = 4028879098149916159_usize as i128;
_6.fld5.0 = '\u{10930a}';
_6.fld5.2 = _1 as u8;
RET = [5039196756824720701_usize,2671666125469636556_usize,13862199312162807041_usize,7341578482529620466_usize,1_usize,5_usize,6868308745258478279_usize,18060888911056893732_usize];
_3.fld0 = [_6.fld5.2,_6.fld5.2,_6.fld5.2];
_6.fld5.1 = Adt56 { fld0: _3.fld0 };
_6.fld5.1.fld0 = _3.fld0;
_7 = 16600404972921073307_usize | 14855082892324133178_usize;
_3.fld0 = _6.fld5.1.fld0;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld5 = ('\u{7aaa}', Move(_3), 145_u8);
_1 = -(-104236077681120815278367918637694886653_i128);
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_1 = (-375531763291073431316897742962775259_i128);
_6.fld5.2 = false as u8;
_6.fld5.2 = !232_u8;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld5 = ('\u{77236}', Move(_3), 134_u8);
Call(_1 = core::intrinsics::bswap((-70719199458337720636708572121722714939_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = [_8,_8,_8,_8,_8];
SetDiscriminant(_6.fld1, 1);
place!(Field::<i128>(Variant(_6.fld1, 1), 1)) = 10817_i16 as i128;
_6.fld3 = core::ptr::addr_of_mut!(_8);
place!(Field::<u8>(Variant(_6.fld1, 1), 3)) = 47_isize as u8;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = 222863683347406199885671841642224776944_u128;
_8 = _7 == _7;
place!(Field::<u8>(Variant(_6.fld1, 1), 3)) = _6.fld5.2 - _6.fld5.2;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = 68773845226743766365707811132259352933_u128;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = !11649805769106886807840048610249910178_u128;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = 56559065230549459127213179216386147006_u128;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = 240970523381734046909470630265639814883_u128 | 76904990157480491866402920246543799666_u128;
_3.fld0 = _6.fld5.1.fld0;
place!(Field::<u32>(Variant(_6.fld1, 1), 2)) = !2928282386_u32;
_7 = 7_usize | 1469262290792199453_usize;
match _6.fld5.2 {
0 => bb1,
1 => bb11,
2 => bb14,
3 => bb15,
134 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
_1 = (-927184676832956350_i64) as i128;
_7 = 7380419198949200696_u64 as usize;
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = 5094571433366212355_i64 as i128;
_6.fld1 = Adt18::Variant1 { fld0: 118455745727851106668343920896468697029_u128,fld1: _1,fld2: 187767012_u32,fld3: _6.fld5.2 };
match _6.fld5.2 {
0 => bb4,
1 => bb5,
134 => bb7,
_ => bb6
}
}
bb16 = {
RET = [6330479794821757248_usize,18239052333171690062_usize,1490015058072398974_usize,12412562407813039800_usize,904562254962370493_usize,3_usize,5911991596364882401_usize,3_usize];
_3.fld0 = [139_u8,127_u8,190_u8];
RET = [9520151746713653398_usize,6_usize,9741248326884373111_usize,7229722650320006716_usize,5_usize,6_usize,18217441093597867679_usize,2501583734526622503_usize];
_3.fld0 = [5_u8,193_u8,66_u8];
_1 = 92930630459781064054895612536910083064_i128 << 8841773711457401621_i64;
RET = [0_usize,8031460230573460802_usize,7693162728658749015_usize,15398726663891549636_usize,0_usize,2621567541249734971_usize,2_usize,3_usize];
_2 = [(-66_i8),100_i8,55_i8,118_i8,126_i8,80_i8,(-84_i8),(-77_i8)];
RET = [7344746065689052069_usize,1_usize,7519167576360331568_usize,8732053997840922459_usize,0_usize,9943309685576724826_usize,945729228589073344_usize,9077922149295564629_usize];
RET = [1_usize,1183987062169024468_usize,2_usize,6252085009902960903_usize,3998557497696190455_usize,15383910712617144519_usize,4_usize,11906457042200741296_usize];
Call(_1 = fn7(_3.fld0, _2, _2, Move(_3), RET, RET, RET, RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_6.fld5.0 = '\u{d6cce}';
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = (-1014_i16) as i128;
_3.fld0 = [Field::<u8>(Variant(_6.fld1, 1), 3),Field::<u8>(Variant(_6.fld1, 1), 3),Field::<u8>(Variant(_6.fld1, 1), 3)];
_6.fld3 = core::ptr::addr_of_mut!(_8);
_2 = [(-30_i8),85_i8,21_i8,(-49_i8),(-107_i8),(-30_i8),(-82_i8),114_i8];
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_6.fld5 = ('\u{d4a23}', Move(_3), Field::<u8>(Variant(_6.fld1, 1), 3));
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = [_8,_8,_8,_8,_8];
_3.fld0 = [_6.fld5.2,Field::<u8>(Variant(_6.fld1, 1), 3),_6.fld5.2];
_8 = true & true;
RET = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = !Field::<i128>(Variant(_6.fld1, 1), 1);
_6.fld1 = Adt18::Variant1 { fld0: 300181255644053497283119581101849903106_u128,fld1: _1,fld2: 242772963_u32,fld3: _6.fld5.2 };
_6.fld1 = Adt18::Variant1 { fld0: 317250984128348056401760803485357153261_u128,fld1: _1,fld2: 2802989303_u32,fld3: _6.fld5.2 };
_10 = [_8,_8,_8,_8,_8];
_1 = Field::<i128>(Variant(_6.fld1, 1), 1);
_7 = (-18_i8) as usize;
_6.fld5.1 = Move(_3);
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = Field::<i128>(Variant(_6.fld1, 1), 1) as u128;
_3 = Adt56 { fld0: _6.fld5.1.fld0 };
_6.fld3 = core::ptr::addr_of_mut!(_8);
place!(Field::<u32>(Variant(_6.fld1, 1), 2)) = 1005794465_u32;
place!(Field::<u128>(Variant(_6.fld1, 1), 0)) = 25695541569438562018820696178861762845_u128 | 276443372658119418874468821164963756045_u128;
Goto(bb18)
}
bb18 = {
Call(_13 = dump_var(6_usize, 2_usize, Move(_2), 8_usize, Move(_8), 14_usize, _14, 14_usize, _14), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [u8; 3],mut _2: [i8; 8],mut _3: [i8; 8],mut _4: Adt56,mut _5: [usize; 8],mut _6: [usize; 8],mut _7: [usize; 8],mut _8: [usize; 8],mut _9: [i8; 8],mut _10: [i8; 8]) -> i128 {
mir! {
type RET = i128;
let _11: [i8; 8];
let _12: (char, Adt56, u8);
let _13: f32;
let _14: isize;
let _15: [i128; 3];
let _16: i32;
let _17: char;
let _18: [usize; 8];
let _19: u16;
let _20: [i64; 1];
let _21: Adt22;
let _22: isize;
let _23: ();
let _24: ();
{
RET = 47981697361699705837641116743259414268_i128 * 103474048273150891602497680379448825091_i128;
_10 = [5_i8,(-54_i8),(-89_i8),126_i8,32_i8,(-56_i8),12_i8,118_i8];
RET = (-123122697344591377361088932145310414647_i128) ^ (-35197652806807795702733212454721454198_i128);
_7 = [12484538610852137785_usize,16609383354866480669_usize,0_usize,7010337247313001886_usize,17370581826179351382_usize,1546825649172555254_usize,1_usize,1_usize];
_3 = _2;
_10 = _2;
_11 = [41_i8,(-9_i8),(-75_i8),(-69_i8),(-49_i8),(-7_i8),(-69_i8),100_i8];
_11 = [(-126_i8),124_i8,(-78_i8),30_i8,(-125_i8),118_i8,(-42_i8),(-46_i8)];
_12.0 = '\u{d5353}';
_12 = ('\u{6b45}', Move(_4), 127_u8);
_7 = [6_usize,5883876831347641629_usize,10174582670329653303_usize,3022659557641354562_usize,16810192993491143205_usize,7_usize,3_usize,320527920133024114_usize];
RET = 78578071130631733066598013445416494110_i128 - 93363463902066435997102401639974845986_i128;
_4.fld0 = [_12.2,_12.2,_12.2];
RET = 84373407229180359505979289025389611397_i128 << _12.2;
_14 = 108_isize;
_14 = (-9223372036854775808_isize);
_6 = _7;
_12.1 = Adt56 { fld0: _1 };
_10 = _2;
_12.1.fld0 = [_12.2,_12.2,_12.2];
_5 = _8;
_14 = (-9223372036854775808_isize);
_5 = _6;
_1 = _4.fld0;
match _12.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
127 => bb6,
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
_11 = _10;
RET = (-94558534087738399314779212792541012269_i128) * (-89828635117834903774682616867002042170_i128);
RET = 17552_i16 as i128;
_12 = ('\u{c2f5b}', Move(_4), 131_u8);
_12.1.fld0 = _1;
_3 = _2;
_12.0 = '\u{cafbf}';
_12.1 = Adt56 { fld0: _1 };
_12.2 = !202_u8;
_1 = [_12.2,_12.2,_12.2];
_12.1 = Adt56 { fld0: _1 };
_12.1.fld0 = [_12.2,_12.2,_12.2];
_3 = _2;
Goto(bb7)
}
bb7 = {
_16 = (-1204052603_i32);
_13 = 874710112_u32 as f32;
Call(_6 = fn8(_9, _8, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = Adt56 { fld0: _12.1.fld0 };
_2 = [(-23_i8),76_i8,(-111_i8),(-79_i8),(-99_i8),(-8_i8),83_i8,(-66_i8)];
_17 = _12.0;
_8 = [15083618025273219182_usize,17461688587275924569_usize,4_usize,7_usize,18338505546974208358_usize,2_usize,4851250126594283319_usize,1_usize];
_1 = _12.1.fld0;
RET = (-84862305492665485267581096815305166807_i128);
_12.1.fld0 = [_12.2,_12.2,_12.2];
_12 = (_17, Move(_4), 145_u8);
_12.0 = _17;
_10 = _3;
_4.fld0 = _1;
_16 = true as i32;
_18 = _6;
_14 = 9223372036854775807_isize | 9223372036854775807_isize;
_4 = Move(_12.1);
_15 = [RET,RET,RET];
_12.1 = Adt56 { fld0: _1 };
_15 = [RET,RET,RET];
_12 = (_17, Move(_4), 23_u8);
_3 = [(-114_i8),96_i8,(-101_i8),(-21_i8),(-77_i8),(-70_i8),127_i8,11_i8];
_8 = _5;
_19 = !11951_u16;
_12.0 = _17;
_6 = [8947555338639613052_usize,3_usize,11075193723294435334_usize,10628388732452552849_usize,4_usize,4_usize,0_usize,4_usize];
_4 = Adt56 { fld0: _1 };
_19 = 25919_u16 << _16;
Goto(bb9)
}
bb9 = {
_8 = _7;
_4.fld0 = _12.1.fld0;
_1 = _4.fld0;
RET = (-1944067523337270305870425781420040891_i128);
_16 = !297950497_i32;
_21.fld0 = !true;
_4.fld0 = _12.1.fld0;
_1 = [_12.2,_12.2,_12.2];
_5 = _7;
_20 = [712633792661970891_i64];
_8 = [3_usize,12243371346036084140_usize,5_usize,4740875231007259549_usize,4_usize,3989436460363905026_usize,1_usize,1_usize];
_5 = [5800555884861571473_usize,6_usize,7_usize,9101595084937012217_usize,3_usize,3_usize,10455772988546626863_usize,6_usize];
_12.1.fld0 = [_12.2,_12.2,_12.2];
_20 = [(-1089755607397974505_i64)];
_21.fld1 = RET ^ RET;
_12.1 = Adt56 { fld0: _1 };
RET = _21.fld1;
match _12.2 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
23 => bb15,
_ => bb14
}
}
bb10 = {
Return()
}
bb11 = {
_16 = (-1204052603_i32);
_13 = 874710112_u32 as f32;
Call(_6 = fn8(_9, _8, _9), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_5 = [1_usize,11981240239866689931_usize,1_usize,16882662868469279519_usize,13829854547445891972_usize,6_usize,6_usize,5_usize];
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(7_usize, 10_usize, Move(_10), 5_usize, Move(_5), 20_usize, Move(_20), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(7_usize, 8_usize, Move(_8), 17_usize, Move(_17), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [i8; 8],mut _2: [usize; 8],mut _3: [i8; 8]) -> [usize; 8] {
mir! {
type RET = [usize; 8];
let _4: &'static &'static f32;
let _5: usize;
let _6: &'static Adt25;
let _7: [u32; 7];
let _8: *mut i16;
let _9: f32;
let _10: u8;
let _11: i64;
let _12: f32;
let _13: [i16; 4];
let _14: [i128; 3];
let _15: &'static [i64; 1];
let _16: &'static [i64; 1];
let _17: isize;
let _18: &'static (*const Adt18, bool);
let _19: f64;
let _20: f64;
let _21: [bool; 2];
let _22: usize;
let _23: usize;
let _24: i64;
let _25: ();
let _26: ();
{
_3 = [(-94_i8),21_i8,63_i8,(-117_i8),(-3_i8),(-95_i8),(-10_i8),(-102_i8)];
_1 = _3;
RET = [14192732558393540800_usize,11524194881425446912_usize,7221038887759280468_usize,7_usize,0_usize,0_usize,7_usize,1205963336194139572_usize];
_3 = _1;
RET = _2;
_1 = [75_i8,122_i8,80_i8,(-73_i8),(-99_i8),28_i8,(-78_i8),108_i8];
RET = [12220291647520958099_usize,7_usize,2_usize,3_usize,18180536210191708925_usize,4_usize,5480287428765286942_usize,3525726462232090317_usize];
Goto(bb1)
}
bb1 = {
_3 = [116_i8,(-121_i8),(-28_i8),9_i8,(-62_i8),34_i8,22_i8,(-51_i8)];
RET = [17711754844055374136_usize,890613144758161115_usize,3_usize,18294988584272125979_usize,8911779349100029854_usize,2_usize,4_usize,1_usize];
_3 = [96_i8,(-28_i8),(-113_i8),112_i8,76_i8,93_i8,36_i8,(-33_i8)];
_2 = [2_usize,1_usize,3_usize,11992122611524822931_usize,5_usize,4_usize,0_usize,261585226530133598_usize];
RET = [2_usize,13501049888591823737_usize,7495718725649162555_usize,12828345470887935084_usize,1824003257066818404_usize,4_usize,6152306588181021428_usize,5_usize];
RET = [3_usize,1837634026453436195_usize,7_usize,2_usize,0_usize,1001599719600999539_usize,5229886799071757903_usize,12180011549728610531_usize];
_1 = [(-113_i8),74_i8,63_i8,75_i8,98_i8,(-35_i8),113_i8,112_i8];
RET = [2731338739696151096_usize,7_usize,6961204656481895240_usize,8857812314081689748_usize,2144141886053793076_usize,5_usize,2_usize,5_usize];
_5 = 17642024447467446225_usize;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = '\u{9d38b}' as usize;
RET = _2;
_2 = RET;
_7 = [2314633804_u32,2848122867_u32,190771100_u32,2810226599_u32,1976057990_u32,699387182_u32,4261681462_u32];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
Goto(bb2)
}
bb2 = {
_1 = [32_i8,93_i8,75_i8,82_i8,126_i8,34_i8,1_i8,44_i8];
_1 = [(-95_i8),(-102_i8),(-16_i8),(-109_i8),(-6_i8),67_i8,122_i8,(-32_i8)];
_3 = [(-8_i8),(-11_i8),(-32_i8),73_i8,59_i8,63_i8,57_i8,(-30_i8)];
_2 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = RET;
_3 = _1;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_3 = _1;
_5 = 4_usize << 202700900906437362314566489222141098009_u128;
_9 = 7728783546770771196_u64 as f32;
_2 = [_5,_5,_5,_5,_5,_5,_5,_5];
_12 = _9;
_11 = 4389386309645571106_i64 + (-3744786766323129452_i64);
_10 = !98_u8;
Goto(bb3)
}
bb3 = {
_5 = 8124121263574620515_usize & 7_usize;
_1 = [82_i8,(-119_i8),97_i8,29_i8,37_i8,33_i8,111_i8,(-92_i8)];
_13 = [(-17556_i16),16186_i16,(-16841_i16),(-16207_i16)];
Goto(bb4)
}
bb4 = {
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_11 = !(-8051294699628965366_i64);
Goto(bb5)
}
bb5 = {
_5 = (-588723217_i32) as usize;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = 40170_u16 as usize;
_11 = !(-4318714552493195282_i64);
_11 = 2290233864733602556_u64 as i64;
_14 = [50841177452422650869649223300953146647_i128,(-98284350491259148049363128144308110870_i128),132477780448632830830978392353177326795_i128];
RET = _2;
_12 = -_9;
_5 = 2_usize | 3484967556181736320_usize;
_11 = _10 as i64;
_12 = 44780791123903238695517904515596684646_u128 as f32;
_5 = 14584390938673162980_usize;
_11 = 2025106108676442913_i64;
_13 = [(-31527_i16),31634_i16,(-24820_i16),(-7677_i16)];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_3 = [(-97_i8),75_i8,(-49_i8),(-8_i8),71_i8,57_i8,(-35_i8),(-20_i8)];
_13 = [15928_i16,(-26410_i16),(-7305_i16),(-8008_i16)];
_5 = 2305124961438321667_usize;
_2 = RET;
_3 = [(-43_i8),119_i8,85_i8,16_i8,(-21_i8),69_i8,(-77_i8),88_i8];
Goto(bb6)
}
bb6 = {
RET = _2;
RET = _2;
RET = _2;
_5 = 15605655198806793392_usize;
_2 = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = 12040629578062147109_usize + 4_usize;
_3 = _1;
_7 = [3979944407_u32,3831411572_u32,2558369846_u32,2273703271_u32,3603370169_u32,2495515234_u32,2602059512_u32];
_3 = [27_i8,90_i8,(-127_i8),52_i8,122_i8,(-78_i8),(-42_i8),(-52_i8)];
_1 = _3;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
RET = _2;
_10 = !53_u8;
_9 = _5 as f32;
RET = _2;
_10 = !117_u8;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_10 = 83_i8 as u8;
_17 = !(-9223372036854775808_isize);
_1 = _3;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_1 = [(-11_i8),76_i8,71_i8,43_i8,(-112_i8),72_i8,118_i8,89_i8];
_9 = _17 as f32;
_1 = [54_i8,81_i8,82_i8,111_i8,82_i8,(-84_i8),41_i8,51_i8];
_19 = _5 as f64;
_17 = 9223372036854775807_isize;
_13 = [(-28095_i16),2866_i16,21444_i16,17573_i16];
_3 = [95_i8,62_i8,75_i8,96_i8,127_i8,31_i8,(-73_i8),(-113_i8)];
match _11 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
2025106108676442913 => bb15,
_ => bb14
}
}
bb7 = {
_5 = (-588723217_i32) as usize;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = 40170_u16 as usize;
_11 = !(-4318714552493195282_i64);
_11 = 2290233864733602556_u64 as i64;
_14 = [50841177452422650869649223300953146647_i128,(-98284350491259148049363128144308110870_i128),132477780448632830830978392353177326795_i128];
RET = _2;
_12 = -_9;
_5 = 2_usize | 3484967556181736320_usize;
_11 = _10 as i64;
_12 = 44780791123903238695517904515596684646_u128 as f32;
_5 = 14584390938673162980_usize;
_11 = 2025106108676442913_i64;
_13 = [(-31527_i16),31634_i16,(-24820_i16),(-7677_i16)];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_3 = [(-97_i8),75_i8,(-49_i8),(-8_i8),71_i8,57_i8,(-35_i8),(-20_i8)];
_13 = [15928_i16,(-26410_i16),(-7305_i16),(-8008_i16)];
_5 = 2305124961438321667_usize;
_2 = RET;
_3 = [(-43_i8),119_i8,85_i8,16_i8,(-21_i8),69_i8,(-77_i8),88_i8];
Goto(bb6)
}
bb8 = {
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_11 = !(-8051294699628965366_i64);
Goto(bb5)
}
bb9 = {
_5 = 8124121263574620515_usize & 7_usize;
_1 = [82_i8,(-119_i8),97_i8,29_i8,37_i8,33_i8,111_i8,(-92_i8)];
_13 = [(-17556_i16),16186_i16,(-16841_i16),(-16207_i16)];
Goto(bb4)
}
bb10 = {
_1 = [32_i8,93_i8,75_i8,82_i8,126_i8,34_i8,1_i8,44_i8];
_1 = [(-95_i8),(-102_i8),(-16_i8),(-109_i8),(-6_i8),67_i8,122_i8,(-32_i8)];
_3 = [(-8_i8),(-11_i8),(-32_i8),73_i8,59_i8,63_i8,57_i8,(-30_i8)];
_2 = [_5,_5,_5,_5,_5,_5,_5,_5];
_2 = RET;
_3 = _1;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_3 = _1;
_5 = 4_usize << 202700900906437362314566489222141098009_u128;
_9 = 7728783546770771196_u64 as f32;
_2 = [_5,_5,_5,_5,_5,_5,_5,_5];
_12 = _9;
_11 = 4389386309645571106_i64 + (-3744786766323129452_i64);
_10 = !98_u8;
Goto(bb3)
}
bb11 = {
_3 = [116_i8,(-121_i8),(-28_i8),9_i8,(-62_i8),34_i8,22_i8,(-51_i8)];
RET = [17711754844055374136_usize,890613144758161115_usize,3_usize,18294988584272125979_usize,8911779349100029854_usize,2_usize,4_usize,1_usize];
_3 = [96_i8,(-28_i8),(-113_i8),112_i8,76_i8,93_i8,36_i8,(-33_i8)];
_2 = [2_usize,1_usize,3_usize,11992122611524822931_usize,5_usize,4_usize,0_usize,261585226530133598_usize];
RET = [2_usize,13501049888591823737_usize,7495718725649162555_usize,12828345470887935084_usize,1824003257066818404_usize,4_usize,6152306588181021428_usize,5_usize];
RET = [3_usize,1837634026453436195_usize,7_usize,2_usize,0_usize,1001599719600999539_usize,5229886799071757903_usize,12180011549728610531_usize];
_1 = [(-113_i8),74_i8,63_i8,75_i8,98_i8,(-35_i8),113_i8,112_i8];
RET = [2731338739696151096_usize,7_usize,6961204656481895240_usize,8857812314081689748_usize,2144141886053793076_usize,5_usize,2_usize,5_usize];
_5 = 17642024447467446225_usize;
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = '\u{9d38b}' as usize;
RET = _2;
_2 = RET;
_7 = [2314633804_u32,2848122867_u32,190771100_u32,2810226599_u32,1976057990_u32,699387182_u32,4261681462_u32];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
RET = _2;
_13 = [16182_i16,(-7920_i16),2998_i16,(-30314_i16)];
_10 = 151_u8 + 111_u8;
_3 = [(-15_i8),44_i8,116_i8,(-59_i8),(-80_i8),(-105_i8),22_i8,57_i8];
_21 = [false,true];
_14 = [117835298973552286032252288691243347902_i128,(-115965480382531319234620000152962239255_i128),(-47740297807672665801033587306808632312_i128)];
_3 = [(-87_i8),112_i8,(-87_i8),13_i8,71_i8,(-106_i8),(-49_i8),2_i8];
RET = [_5,_5,_5,_5,_5,_5,_5,_5];
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(8_usize, 14_usize, Move(_14), 17_usize, Move(_17), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_25 = dump_var(8_usize, 13_usize, Move(_13), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9() -> *mut f32 {
mir! {
type RET = *mut f32;
let _1: u128;
let _2: isize;
let _3: char;
let _4: usize;
let _5: f32;
let _6: [i16; 5];
let _7: isize;
let _8: isize;
let _9: f64;
let _10: &'static Adt18;
let _11: bool;
let _12: bool;
let _13: [i128; 3];
let _14: i8;
let _15: [u32; 7];
let _16: u8;
let _17: *const *mut bool;
let _18: i128;
let _19: u32;
let _20: [bool; 5];
let _21: u16;
let _22: char;
let _23: i8;
let _24: bool;
let _25: [bool; 5];
let _26: [u32; 2];
let _27: isize;
let _28: (i16,);
let _29: Adt22;
let _30: *const *mut i64;
let _31: Adt56;
let _32: (Adt25, *const *mut i64, u32, i16);
let _33: f32;
let _34: ();
let _35: ();
{
_1 = 9991856715102107952_u64 as u128;
Goto(bb1)
}
bb1 = {
_1 = 133417258622701975212484124695258866150_u128 << 1929956924_i32;
_1 = 32566815097698067_i64 as u128;
_2 = !(-9223372036854775808_isize);
_1 = 131650524869880245459285005844557800631_u128 << _2;
_1 = 186299570470746255932545240023377038979_u128 << _2;
_2 = 121_isize;
_2 = 65_isize * 18_isize;
_1 = 16816434881963854432821990092806481269_u128 - 301113709504221095070480013918788432769_u128;
_2 = 1098113621552418685_usize as isize;
_2 = 7509_u16 as isize;
_2 = (-9223372036854775808_isize);
_3 = '\u{e9be4}';
_1 = _2 as u128;
_2 = -9223372036854775807_isize;
_1 = 208955311081699599466627811948104305298_u128 << _2;
_3 = '\u{314dd}';
_3 = '\u{4a771}';
_1 = 72246787412973217477041709143979972434_u128 << _2;
_2 = (-112_isize) + 9223372036854775807_isize;
_3 = '\u{75e01}';
_2 = 9223372036854775807_isize | (-9223372036854775808_isize);
_2 = 9223372036854775807_isize;
_3 = '\u{1448d}';
_3 = '\u{6a2c2}';
_2 = -9223372036854775807_isize;
_4 = !3160698475539331352_usize;
_2 = (-9223372036854775808_isize) >> _1;
_2 = 9223372036854775807_isize - 29_isize;
_4 = 4344093913499237735_usize;
Call(_4 = fn10(_3, _2, _2, _2, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = 128522735194944741313316622293701419475_i128 as isize;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = (-123665504613602254505525154297167984946_i128) as f32;
_3 = '\u{9bc05}';
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 45_u8 as f32;
_7 = -_2;
_1 = 143_u8 as u128;
_2 = _7 >> _7;
(*RET) = 1993002331_u32 as f32;
_6 = [(-14181_i16),15013_i16,16303_i16,(-21841_i16),10193_i16];
_5 = 6962783812225634773_u64 as f32;
(*RET) = (-1716963340_i32) as f32;
(*RET) = 35_u8 as f32;
_6 = [(-10065_i16),(-28401_i16),(-30587_i16),(-24232_i16),(-22994_i16)];
_5 = 6985_i16 as f32;
RET = core::ptr::addr_of_mut!(_5);
_1 = 5_u8 as u128;
_6 = [(-11302_i16),11071_i16,9439_i16,(-23467_i16),200_i16];
_7 = -_2;
Goto(bb3)
}
bb3 = {
(*RET) = _2 as f32;
_9 = 9630843085964718735_u64 as f64;
_1 = _3 as u128;
(*RET) = 51840_u16 as f32;
(*RET) = _4 as f32;
_7 = _2;
(*RET) = 36856042108188168307689560743206243285_i128 as f32;
_6 = [(-14289_i16),(-3582_i16),(-10961_i16),(-20868_i16),3358_i16];
_4 = 7847_u16 as usize;
_4 = (*RET) as usize;
_11 = !true;
Goto(bb4)
}
bb4 = {
_7 = 2550_i16 as isize;
_8 = 60219_u16 as isize;
_12 = _11 | _11;
_13 = [(-86269736806948308041125664577920217202_i128),(-15571807932190600073101792935766887321_i128),99018404180174801283246478665871091657_i128];
_14 = (-1908708667019529807743898330739685538_i128) as i8;
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of_mut!((*RET));
_14 = (-123_i8);
_6 = [14233_i16,(-1952_i16),(-25663_i16),(-23677_i16),28460_i16];
(*RET) = 592532723_u32 as f32;
_1 = 152795152174039827840351609387458030322_u128 >> _8;
_1 = !46037120534091814210297625409482530292_u128;
_4 = !2197066759957851662_usize;
_11 = _12;
_11 = !_12;
_13 = [(-1166646575955377055600421730074989413_i128),(-72374013295584686211465455864864902299_i128),(-165088028384373028010251999849382932840_i128)];
_13 = [(-34102003229667076242330607966577786841_i128),(-101926881223956627963951756299806219601_i128),152824817426772794587745699455370063785_i128];
_14 = !(-49_i8);
_13 = [98113911535257313258842249512153685245_i128,131055095611671923734261731252458157756_i128,31830852267868985318182203141048323611_i128];
_9 = 2456747183_u32 as f64;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = _14 as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 9_u8 as f32;
_6 = [(-28034_i16),(-12354_i16),(-3034_i16),(-18911_i16),(-20933_i16)];
_6 = [14442_i16,12440_i16,5416_i16,(-24040_i16),(-2970_i16)];
Goto(bb6)
}
bb6 = {
_13 = [(-8901999925731878359747163643666368539_i128),22979202720361845006606422243056814705_i128,167105088966799616838519688439180681799_i128];
Goto(bb7)
}
bb7 = {
_6 = [(-16769_i16),7733_i16,23141_i16,(-13959_i16),(-19262_i16)];
_1 = (-4053_i16) as u128;
RET = core::ptr::addr_of_mut!((*RET));
_3 = '\u{10e53f}';
_1 = 151074584565885184389014593429493909501_u128 ^ 9955157054520918037353216706461340310_u128;
_16 = 18_u8;
_11 = _12 | _12;
_14 = 7164116147209667936_i64 as i8;
(*RET) = (-7660891476256433245_i64) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 27857_i16 as f32;
RET = core::ptr::addr_of_mut!((*RET));
_3 = '\u{14a4f}';
_15 = [1925695326_u32,4171011451_u32,2592834906_u32,2938872359_u32,2331048440_u32,1352947616_u32,394482827_u32];
_9 = 5308029110850349221_i64 as f64;
(*RET) = 15638_u16 as f32;
_2 = _7 & _7;
_14 = 18786_u16 as i8;
_9 = (-6423590049880757291_i64) as f64;
(*RET) = _1 as f32;
_14 = 40_i8 - (-8_i8);
match _16 {
0 => bb4,
1 => bb6,
18 => bb9,
_ => bb8
}
}
bb8 = {
_13 = [(-8901999925731878359747163643666368539_i128),22979202720361845006606422243056814705_i128,167105088966799616838519688439180681799_i128];
Goto(bb7)
}
bb9 = {
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
_4 = 13839264465393615267_usize | 15579837539155167448_usize;
RET = core::ptr::addr_of_mut!(_5);
_15 = [840161579_u32,3890656425_u32,676648318_u32,222591681_u32,3268828312_u32,1473904017_u32,14030331_u32];
_19 = 4236_i16 as u32;
(*RET) = 23192_u16 as f32;
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
_1 = 240486299674502361381480980523861081206_u128 >> _8;
_20 = [_11,_11,_11,_12,_11];
_13 = [125054920909168366733229096365628573509_i128,(-1737852420460426339834843833305392908_i128),(-101160791666075033747082218744513258833_i128)];
_16 = 161_u8 ^ 153_u8;
(*RET) = 6392474931454282788_i64 as f32;
_2 = _7;
Goto(bb10)
}
bb10 = {
_8 = !_7;
_18 = _2 as i128;
_11 = _12;
_7 = -_8;
RET = core::ptr::addr_of_mut!((*RET));
_9 = _16 as f64;
_4 = 6641456503245502819_usize;
_13 = [_18,_18,_18];
_21 = 55761_u16 - 35967_u16;
_14 = 84_i8 >> _21;
_2 = _11 as isize;
_4 = 44747213245389089_usize - 9801751000851391956_usize;
Goto(bb11)
}
bb11 = {
_6 = [(-2233_i16),7459_i16,(-13508_i16),(-31857_i16),(-29532_i16)];
_18 = 16649672608413005780907550309372664367_i128 | 66932290882287190447675214277044527654_i128;
_11 = !_12;
_11 = _16 < _16;
_12 = _11;
_13 = [_18,_18,_18];
_3 = '\u{5ca13}';
_2 = _8;
_14 = 65_i8;
_9 = 9520_i16 as f64;
_19 = _4 as u32;
RET = core::ptr::addr_of_mut!(_5);
_1 = 23949918543430002481391709701726403968_u128 ^ 233406100648907083993875624334735443373_u128;
_23 = _14 * _14;
_22 = _3;
(*RET) = _16 as f32;
_6 = [(-27059_i16),6102_i16,(-16114_i16),25807_i16,16410_i16];
_8 = !_2;
_14 = _23 << _19;
Goto(bb12)
}
bb12 = {
_6 = [(-14267_i16),1142_i16,25992_i16,19554_i16,919_i16];
_20 = [_11,_12,_12,_11,_11];
_22 = _3;
(*RET) = _18 as f32;
_12 = _11 | _11;
(*RET) = _8 as f32;
_21 = 31643_u16;
_4 = 8522796193964579756_usize;
_24 = !_11;
_14 = _23;
_7 = _2;
_26 = [_19,_19];
_1 = 95380388338374837178363453960400758799_u128;
(*RET) = (-271305557257076255_i64) as f32;
_19 = !691860044_u32;
_20 = [_11,_11,_12,_24,_12];
_13 = [_18,_18,_18];
(*RET) = 18311414207483168383_u64 as f32;
_12 = !_24;
RET = core::ptr::addr_of_mut!((*RET));
_16 = 200_u8;
_14 = _23;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = _9 as f32;
_1 = 7375614509721978073167827192110479942_u128;
_20 = [_12,_24,_11,_12,_24];
_14 = _23 << _16;
_26 = [_19,_19];
Goto(bb13)
}
bb13 = {
_11 = !_24;
_5 = _18 as f32;
_28 = (24787_i16,);
_26 = [_19,_19];
_7 = 1348633407_i32 as isize;
_29.fld2 = _16 as isize;
_27 = _29.fld2;
_25 = [_24,_24,_24,_24,_11];
_14 = _23 * _23;
Goto(bb14)
}
bb14 = {
_8 = _1 as isize;
_15 = [_19,_19,_19,_19,_19,_19,_19];
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = _16 as f32;
(*RET) = _18 as f32;
_29.fld3 = _21 >> _18;
_32.0.fld4 = _28.0;
_22 = _3;
RET = core::ptr::addr_of_mut!(_33);
_32.0.fld3.fld3 = _29.fld2 as u16;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(9_usize, 13_usize, Move(_13), 6_usize, Move(_6), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(9_usize, 21_usize, Move(_21), 22_usize, Move(_22), 12_usize, Move(_12), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(9_usize, 24_usize, Move(_24), 14_usize, Move(_14), 26_usize, Move(_26), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: char,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: u128,mut _6: isize) -> usize {
mir! {
type RET = usize;
let _7: &'static *mut i64;
let _8: [i16; 5];
let _9: i16;
let _10: &'static i8;
let _11: (i16,);
let _12: (Adt22, *const Adt25, [bool; 5], (&'static usize, &'static Adt18));
let _13: char;
let _14: &'static [i64; 1];
let _15: [i128; 3];
let _16: *const u64;
let _17: &'static (*const Adt18, bool);
let _18: char;
let _19: f32;
let _20: i128;
let _21: isize;
let _22: u8;
let _23: [i128; 3];
let _24: Adt22;
let _25: isize;
let _26: *const *mut i64;
let _27: *mut &'static u128;
let _28: isize;
let _29: f32;
let _30: ();
let _31: ();
{
_3 = 144565625186769051669248586536338438363_i128 as isize;
RET = 3_usize | 5_usize;
_3 = _2 - _2;
_6 = !_2;
RET = 2014495542749408329_usize | 2_usize;
_6 = 8265759718301156998_u64 as isize;
_2 = _6 | _3;
Goto(bb1)
}
bb1 = {
RET = 4_usize;
_8 = [(-21063_i16),25719_i16,30374_i16,(-12781_i16),27917_i16];
RET = !7_usize;
_5 = 123869431651968216674203916033763388805_u128;
_8 = [(-29397_i16),20402_i16,10609_i16,27436_i16,24071_i16];
_9 = !16315_i16;
_9 = 21270_i16 >> _3;
_4 = -_2;
_9 = 21906_i16 ^ (-5118_i16);
_2 = _4 ^ _3;
_4 = _2 + _3;
_2 = -_4;
_6 = _2;
_12.0.fld3 = !47416_u16;
_11 = (_9,);
_6 = _4;
_12.2 = [false,false,false,false,true];
Goto(bb2)
}
bb2 = {
_12.3.0 = &RET;
_12.0.fld4 = _5 as u64;
_11.0 = 57875750115814665868647039494017878695_i128 as i16;
_11.0 = _9 - _9;
_12.3.0 = &RET;
_13 = _1;
_12.0.fld1 = _11.0 as i128;
_12.0.fld3 = 56853_u16;
_12.3.0 = &RET;
_12.3.0 = &RET;
_2 = _6 << _6;
_12.0.fld4 = _12.0.fld3 as u64;
_12.0.fld2 = _5 as isize;
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_6 = _13 as isize;
_12.0.fld3 = 29196_u16;
_11.0 = 1915630818_u32 as i16;
_12.0.fld2 = _2;
_2 = _12.0.fld2;
_18 = _1;
_3 = _18 as isize;
_16 = core::ptr::addr_of!(_12.0.fld4);
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_8 = [_11.0,_9,_9,_9,_9];
Call(_12.0.fld1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = _1;
_13 = _18;
(*_16) = 229_u8 as u64;
_3 = _4;
match _12.0.fld3 {
0 => bb2,
1 => bb4,
2 => bb5,
29196 => bb7,
_ => bb6
}
}
bb4 = {
_12.3.0 = &RET;
_12.0.fld4 = _5 as u64;
_11.0 = 57875750115814665868647039494017878695_i128 as i16;
_11.0 = _9 - _9;
_12.3.0 = &RET;
_13 = _1;
_12.0.fld1 = _11.0 as i128;
_12.0.fld3 = 56853_u16;
_12.3.0 = &RET;
_12.3.0 = &RET;
_2 = _6 << _6;
_12.0.fld4 = _12.0.fld3 as u64;
_12.0.fld2 = _5 as isize;
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_6 = _13 as isize;
_12.0.fld3 = 29196_u16;
_11.0 = 1915630818_u32 as i16;
_12.0.fld2 = _2;
_2 = _12.0.fld2;
_18 = _1;
_3 = _18 as isize;
_16 = core::ptr::addr_of!(_12.0.fld4);
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_8 = [_11.0,_9,_9,_9,_9];
Call(_12.0.fld1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = 4_usize;
_8 = [(-21063_i16),25719_i16,30374_i16,(-12781_i16),27917_i16];
RET = !7_usize;
_5 = 123869431651968216674203916033763388805_u128;
_8 = [(-29397_i16),20402_i16,10609_i16,27436_i16,24071_i16];
_9 = !16315_i16;
_9 = 21270_i16 >> _3;
_4 = -_2;
_9 = 21906_i16 ^ (-5118_i16);
_2 = _4 ^ _3;
_4 = _2 + _3;
_2 = -_4;
_6 = _2;
_12.0.fld3 = !47416_u16;
_11 = (_9,);
_6 = _4;
_12.2 = [false,false,false,false,true];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
RET = !6677502533635044685_usize;
(*_16) = !11287209654212915666_u64;
_4 = -_3;
_12.0.fld4 = 14398361787479493613_u64;
Goto(bb8)
}
bb8 = {
_12.0.fld4 = !8195286190692570123_u64;
_3 = _12.0.fld3 as isize;
_12.0.fld3 = (-9012277637938361765_i64) as u16;
_12.0.fld0 = !true;
_12.3.0 = &RET;
_1 = _18;
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
RET = 13867631497846154865_usize;
_9 = -_11.0;
_12.2 = [_12.0.fld0,_12.0.fld0,_12.0.fld0,_12.0.fld0,_12.0.fld0];
_12.3.0 = &RET;
_12.0.fld1 = 108406222505912228700630733540169145038_i128;
_21 = _9 as isize;
_4 = _12.0.fld2;
match RET {
0 => bb5,
1 => bb9,
2 => bb10,
3 => bb11,
13867631497846154865 => bb13,
_ => bb12
}
}
bb9 = {
_12.3.0 = &RET;
_12.0.fld4 = _5 as u64;
_11.0 = 57875750115814665868647039494017878695_i128 as i16;
_11.0 = _9 - _9;
_12.3.0 = &RET;
_13 = _1;
_12.0.fld1 = _11.0 as i128;
_12.0.fld3 = 56853_u16;
_12.3.0 = &RET;
_12.3.0 = &RET;
_2 = _6 << _6;
_12.0.fld4 = _12.0.fld3 as u64;
_12.0.fld2 = _5 as isize;
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_6 = _13 as isize;
_12.0.fld3 = 29196_u16;
_11.0 = 1915630818_u32 as i16;
_12.0.fld2 = _2;
_2 = _12.0.fld2;
_18 = _1;
_3 = _18 as isize;
_16 = core::ptr::addr_of!(_12.0.fld4);
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_8 = [_11.0,_9,_9,_9,_9];
Call(_12.0.fld1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
RET = 4_usize;
_8 = [(-21063_i16),25719_i16,30374_i16,(-12781_i16),27917_i16];
RET = !7_usize;
_5 = 123869431651968216674203916033763388805_u128;
_8 = [(-29397_i16),20402_i16,10609_i16,27436_i16,24071_i16];
_9 = !16315_i16;
_9 = 21270_i16 >> _3;
_4 = -_2;
_9 = 21906_i16 ^ (-5118_i16);
_2 = _4 ^ _3;
_4 = _2 + _3;
_2 = -_4;
_6 = _2;
_12.0.fld3 = !47416_u16;
_11 = (_9,);
_6 = _4;
_12.2 = [false,false,false,false,true];
Goto(bb2)
}
bb12 = {
_12.3.0 = &RET;
_12.0.fld4 = _5 as u64;
_11.0 = 57875750115814665868647039494017878695_i128 as i16;
_11.0 = _9 - _9;
_12.3.0 = &RET;
_13 = _1;
_12.0.fld1 = _11.0 as i128;
_12.0.fld3 = 56853_u16;
_12.3.0 = &RET;
_12.3.0 = &RET;
_2 = _6 << _6;
_12.0.fld4 = _12.0.fld3 as u64;
_12.0.fld2 = _5 as isize;
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_6 = _13 as isize;
_12.0.fld3 = 29196_u16;
_11.0 = 1915630818_u32 as i16;
_12.0.fld2 = _2;
_2 = _12.0.fld2;
_18 = _1;
_3 = _18 as isize;
_16 = core::ptr::addr_of!(_12.0.fld4);
_15 = [_12.0.fld1,_12.0.fld1,_12.0.fld1];
_8 = [_11.0,_9,_9,_9,_9];
Call(_12.0.fld1 = core::intrinsics::transmute(_5), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_19 = _12.0.fld2 as f32;
_8 = [_11.0,_9,_9,_9,_9];
_11.0 = _9;
_12.0.fld0 = !true;
_16 = core::ptr::addr_of!(_12.0.fld4);
_12.0.fld3 = 20733_u16;
_12.2 = [_12.0.fld0,_12.0.fld0,_12.0.fld0,_12.0.fld0,_12.0.fld0];
_24.fld0 = _12.0.fld0;
_21 = _12.0.fld2;
_24 = Adt22 { fld0: _12.0.fld0,fld1: _12.0.fld1,fld2: _6,fld3: _12.0.fld3,fld4: (*_16) };
_6 = _2;
_12.0 = _24;
_23 = [_12.0.fld1,_12.0.fld1,_24.fld1];
_9 = _11.0 ^ _11.0;
_24.fld0 = !_12.0.fld0;
_11 = (_9,);
_12.3.0 = &RET;
_19 = (-45_i8) as f32;
_12.2 = [_24.fld0,_12.0.fld0,_24.fld0,_24.fld0,_24.fld0];
Goto(bb14)
}
bb14 = {
_12.3.0 = &RET;
_21 = -_6;
_9 = _11.0 | _11.0;
_15 = [_24.fld1,_12.0.fld1,_12.0.fld1];
(*_16) = _24.fld4 & _24.fld4;
_9 = !_11.0;
_24.fld4 = !(*_16);
_9 = !_11.0;
_5 = 77_i8 as u128;
_24.fld4 = !_12.0.fld4;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(10_usize, 13_usize, Move(_13), 18_usize, Move(_18), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(10_usize, 8_usize, Move(_8), 9_usize, Move(_9), 21_usize, Move(_21), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: Adt22,mut _2: Adt22,mut _3: Adt22,mut _4: i16,mut _5: bool,mut _6: bool,mut _7: u64,mut _8: bool,mut _9: Adt22,mut _10: u64,mut _11: i128) -> f32 {
mir! {
type RET = f32;
let _12: char;
let _13: [i16; 5];
let _14: f64;
let _15: [i8; 8];
let _16: *mut &'static u128;
let _17: isize;
let _18: &'static [u8; 3];
let _19: [usize; 8];
let _20: isize;
let _21: (&'static *mut i64, &'static Adt25, (i16,), usize);
let _22: ();
let _23: ();
{
_1.fld1 = _3.fld3 as i128;
_1.fld0 = _5 ^ _5;
_1.fld4 = !_9.fld4;
_1.fld1 = _3.fld1 - _11;
_9.fld3 = _3.fld3;
_2.fld1 = 2057634140_u32 as i128;
_9.fld0 = _3.fld0;
_1.fld2 = _2.fld2;
_9 = Adt22 { fld0: _5,fld1: _3.fld1,fld2: _2.fld2,fld3: _1.fld3,fld4: _1.fld4 };
_9 = Adt22 { fld0: _8,fld1: _11,fld2: _1.fld2,fld3: _1.fld3,fld4: _1.fld4 };
_1.fld4 = _3.fld4 >> _9.fld3;
_2.fld3 = _9.fld3 << _10;
_1.fld3 = _3.fld3;
_3 = Adt22 { fld0: _6,fld1: _1.fld1,fld2: _9.fld2,fld3: _2.fld3,fld4: _10 };
_14 = 2464364832_u32 as f64;
_1.fld2 = _5 as isize;
_14 = 59813466147430262279520836435466290383_u128 as f64;
_1.fld4 = _2.fld4;
Call(_9.fld4 = core::intrinsics::transmute(_1.fld2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.fld1 = _1.fld2 as i128;
_9.fld1 = _1.fld1 << _1.fld2;
_1.fld1 = !_3.fld1;
_2.fld1 = -_3.fld1;
_1.fld1 = _9.fld1 >> _3.fld1;
_1.fld0 = _8;
_14 = _7 as f64;
_15 = [(-88_i8),(-52_i8),122_i8,43_i8,(-16_i8),110_i8,98_i8,(-100_i8)];
_1 = _2;
_14 = _3.fld1 as f64;
_12 = '\u{1551c}';
_4 = (-13107_i16) * 27216_i16;
_3.fld1 = _4 as i128;
_1.fld0 = _9.fld0 > _6;
_2.fld3 = _9.fld3;
_15 = [68_i8,125_i8,(-58_i8),(-127_i8),(-128_i8),80_i8,33_i8,127_i8];
_14 = 3656120155_u32 as f64;
_3.fld2 = _2.fld2 << _3.fld3;
_9.fld2 = -_3.fld2;
Goto(bb2)
}
bb2 = {
RET = _4 as f32;
_10 = !_2.fld4;
_2.fld3 = _3.fld3;
_9.fld0 = _2.fld0;
_3.fld3 = _2.fld3 >> _9.fld4;
_3.fld0 = !_2.fld0;
_3 = Adt22 { fld0: _6,fld1: _2.fld1,fld2: _9.fld2,fld3: _2.fld3,fld4: _1.fld4 };
_12 = '\u{ae136}';
_3.fld0 = _2.fld0;
_5 = _10 >= _9.fld4;
_19 = [10006043276179599749_usize,4675383567278333700_usize,17628979880978669514_usize,1_usize,2_usize,0_usize,0_usize,2_usize];
_1.fld3 = !_3.fld3;
_1.fld3 = _3.fld3 >> _3.fld4;
_17 = _3.fld2;
_12 = '\u{d9539}';
_9.fld3 = !_1.fld3;
_3.fld4 = (-7949016938654444174_i64) as u64;
_9 = Adt22 { fld0: _6,fld1: _1.fld1,fld2: _17,fld3: _2.fld3,fld4: _10 };
_2 = Adt22 { fld0: _3.fld0,fld1: _11,fld2: _3.fld2,fld3: _3.fld3,fld4: _1.fld4 };
_2 = Adt22 { fld0: _9.fld0,fld1: _3.fld1,fld2: _17,fld3: _1.fld3,fld4: _9.fld4 };
_2.fld3 = !_9.fld3;
Goto(bb3)
}
bb3 = {
_17 = -_9.fld2;
_9 = Adt22 { fld0: _2.fld0,fld1: _3.fld1,fld2: _17,fld3: _1.fld3,fld4: _7 };
_10 = _1.fld4 - _1.fld4;
_14 = (-1455757528_i32) as f64;
_1.fld4 = !_9.fld4;
_1.fld3 = _5 as u16;
_4 = (-5118649276928363062_i64) as i16;
_1.fld0 = _6;
_19 = [7673658544636001575_usize,11623422984066239181_usize,1_usize,6_usize,7_usize,1_usize,5_usize,7_usize];
_4 = 10354_i16 * (-27249_i16);
RET = _9.fld2 as f32;
_9 = Adt22 { fld0: _8,fld1: _1.fld1,fld2: _3.fld2,fld3: _2.fld3,fld4: _10 };
_2.fld4 = _10;
_13 = [_4,_4,_4,_4,_4];
_13 = [_4,_4,_4,_4,_4];
_9.fld1 = _2.fld1;
_9.fld2 = -_17;
_20 = _2.fld2;
_9.fld4 = _1.fld4 / _7;
_5 = _9.fld0;
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(11_usize, 4_usize, Move(_4), 12_usize, Move(_12), 11_usize, Move(_11), 15_usize, Move(_15)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_22 = dump_var(11_usize, 8_usize, Move(_8), 20_usize, Move(_20), 23_usize, _23, 23_usize, _23), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: char,mut _2: u8,mut _3: [u8; 3],mut _4: char,mut _5: [i8; 8]) -> u8 {
mir! {
type RET = u8;
let _6: char;
let _7: u32;
let _8: *mut *const Adt25;
let _9: isize;
let _10: u128;
let _11: &'static [u8; 3];
let _12: bool;
let _13: i32;
let _14: *const *mut &'static i8;
let _15: Adt51;
let _16: i32;
let _17: (&'static usize, &'static Adt18);
let _18: ();
let _19: ();
{
_4 = _1;
_1 = _4;
RET = _2;
_1 = _4;
_1 = _4;
_6 = _1;
_1 = _4;
RET = _2;
_4 = _6;
_1 = _4;
RET = false as u8;
_2 = RET;
_2 = !RET;
Call(_2 = fn13(_5, _4, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _6;
RET = (-9133_i16) as u8;
_7 = 2549330394_u32;
_4 = _1;
_11 = &_3;
_1 = _6;
_6 = _4;
_11 = &(*_11);
_13 = (-821283816_i32) + (-873669319_i32);
_12 = !false;
_12 = !true;
_11 = &(*_11);
_13 = (-116454502_i32) + (-1536917146_i32);
_13 = 465773670_i32;
_10 = 172304943364011110736737562366860816260_u128;
_12 = _2 < _2;
_11 = &_3;
Goto(bb2)
}
bb2 = {
_7 = 3645433213_u32 << _2;
_9 = 31745_u16 as isize;
_4 = _1;
_9 = -9223372036854775807_isize;
_10 = 72502912519815863400105494463754127808_u128;
_11 = &(*_11);
_3 = [_2,_2,_2];
RET = _2;
_5 = [54_i8,(-45_i8),(-86_i8),107_i8,(-86_i8),(-32_i8),(-21_i8),71_i8];
_5 = [(-81_i8),84_i8,35_i8,(-22_i8),63_i8,89_i8,(-65_i8),33_i8];
RET = _2;
_7 = _6 as u32;
_15.fld0.fld0 = _12;
_13 = 2025114047_i32;
_15.fld0.fld3 = !3458_u16;
_15.fld0.fld4 = _15.fld0.fld3 as u64;
_11 = &_3;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(12_usize, 2_usize, Move(_2), 7_usize, Move(_7), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(12_usize, 5_usize, Move(_5), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i8; 8],mut _2: char,mut _3: char) -> u8 {
mir! {
type RET = u8;
let _4: isize;
let _5: [i128; 3];
let _6: bool;
let _7: f32;
let _8: &'static [u8; 3];
let _9: Adt22;
let _10: isize;
let _11: *mut &'static &'static f32;
let _12: bool;
let _13: isize;
let _14: f32;
let _15: &'static &'static f32;
let _16: &'static *mut i64;
let _17: Adt18;
let _18: Adt76;
let _19: *const *const *mut i64;
let _20: f32;
let _21: &'static usize;
let _22: f64;
let _23: u16;
let _24: ();
let _25: ();
{
_4 = -(-9223372036854775808_isize);
RET = 79_u8 * 116_u8;
_2 = _3;
_5 = [(-88124848754395346823287181450440478382_i128),138224945003801237306793703633070492035_i128,9037037726425775195275685361920843176_i128];
Call(RET = core::intrinsics::bswap(164_u8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !136_u8;
RET = _4 as u8;
_3 = _2;
_1 = [30_i8,(-94_i8),(-103_i8),(-55_i8),12_i8,124_i8,(-46_i8),(-6_i8)];
_2 = _3;
_3 = _2;
_5 = [(-114535584887464069643826853641681534868_i128),(-124225175834394052529885991900232467915_i128),44637431419557198589393318817976353800_i128];
_2 = _3;
_3 = _2;
_4 = (-61451384073523858294064461769207627018_i128) as isize;
_5 = [127647560613677004674661139553295933171_i128,(-44189048564709770148101880308088060113_i128),(-162090670478786228074664243169933515647_i128)];
_4 = (-9223372036854775808_isize);
_5 = [16445276434434300465565967771060653021_i128,(-85480205900456078478560120124224183968_i128),125020397207759325169676557950951617257_i128];
RET = !249_u8;
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = [(-36064798392866651262995514559162589520_i128),(-99519994048133674984684242408690509197_i128),56074897754305883134162963701624607236_i128];
_6 = false;
RET = _4 as u8;
_3 = _2;
RET = 69_u8;
_3 = _2;
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _6 as isize;
_5 = [100230386352655749155149006134094546486_i128,111192918053948109783575407608217374302_i128,(-103175763704264063678751440953016472130_i128)];
_5 = [127013404636294380436646287776855419109_i128,52752384690543779310512594042827398948_i128,(-109442250183043384667347266820505443522_i128)];
_1 = [122_i8,(-44_i8),49_i8,(-41_i8),29_i8,77_i8,(-64_i8),(-46_i8)];
_4 = 48_isize + 9223372036854775807_isize;
RET = 152848025791424425954693477027975064899_i128 as u8;
_2 = _3;
_2 = _3;
_3 = _2;
_9.fld2 = !_4;
_9 = Adt22 { fld0: _6,fld1: 30785674149693635602535632422823047485_i128,fld2: _4,fld3: 64132_u16,fld4: 11376947898374340521_u64 };
_9 = Adt22 { fld0: _6,fld1: 5274114244270388906595911803588151948_i128,fld2: _4,fld3: 7483_u16,fld4: 5590153222463072878_u64 };
_9.fld0 = _9.fld1 == _9.fld1;
_4 = _9.fld2 >> _9.fld4;
_7 = (-737103158603412866_i64) as f32;
_9.fld2 = _4 | _4;
_9.fld2 = _7 as isize;
Goto(bb4)
}
bb4 = {
_1 = [107_i8,(-60_i8),19_i8,(-109_i8),24_i8,27_i8,(-43_i8),(-123_i8)];
_5 = [_9.fld1,_9.fld1,_9.fld1];
_9 = Adt22 { fld0: _6,fld1: 131054991848497726701665155885122798757_i128,fld2: _4,fld3: 14713_u16,fld4: 18272395799273059215_u64 };
_3 = _2;
_7 = 2177601780030893124_i64 as f32;
_9.fld1 = !(-147672883380370300274962729659064191146_i128);
_9.fld3 = 19231_u16 ^ 11459_u16;
_4 = _9.fld2 + _9.fld2;
_6 = _7 < _7;
_1 = [(-99_i8),67_i8,(-124_i8),(-39_i8),(-63_i8),54_i8,8_i8,(-86_i8)];
_9 = Adt22 { fld0: _6,fld1: 48719779283428296297347926450895104144_i128,fld2: _4,fld3: 8697_u16,fld4: 6164293218766971809_u64 };
_9.fld2 = _4;
RET = !124_u8;
_9 = Adt22 { fld0: _6,fld1: (-69944830537610165277206473801253268911_i128),fld2: _4,fld3: 63832_u16,fld4: 6938983952852393969_u64 };
_6 = _9.fld0;
RET = 243_u8;
_6 = _9.fld3 >= _9.fld3;
_9.fld0 = _6 | _6;
_6 = !_9.fld0;
RET = !69_u8;
_5 = [_9.fld1,_9.fld1,_9.fld1];
_1 = [(-124_i8),46_i8,(-118_i8),105_i8,(-43_i8),45_i8,(-102_i8),(-69_i8)];
_9.fld3 = !57301_u16;
RET = 247_u8;
Goto(bb5)
}
bb5 = {
_9.fld0 = !_6;
_10 = !_4;
_12 = _9.fld0;
_10 = (-30930_i16) as isize;
_9.fld4 = 18140399543079761855_u64 << _9.fld1;
RET = 152_u8;
_4 = 33847921264196096308464111712920634796_u128 as isize;
_11 = core::ptr::addr_of_mut!(_15);
_3 = _2;
RET = !78_u8;
_10 = _9.fld2;
_13 = !_10;
_1 = [0_i8,(-62_i8),(-58_i8),65_i8,13_i8,10_i8,95_i8,16_i8];
_2 = _3;
match _9.fld1 {
0 => bb6,
270337536383328298186168133630514942545 => bb8,
_ => bb7
}
}
bb6 = {
_1 = [107_i8,(-60_i8),19_i8,(-109_i8),24_i8,27_i8,(-43_i8),(-123_i8)];
_5 = [_9.fld1,_9.fld1,_9.fld1];
_9 = Adt22 { fld0: _6,fld1: 131054991848497726701665155885122798757_i128,fld2: _4,fld3: 14713_u16,fld4: 18272395799273059215_u64 };
_3 = _2;
_7 = 2177601780030893124_i64 as f32;
_9.fld1 = !(-147672883380370300274962729659064191146_i128);
_9.fld3 = 19231_u16 ^ 11459_u16;
_4 = _9.fld2 + _9.fld2;
_6 = _7 < _7;
_1 = [(-99_i8),67_i8,(-124_i8),(-39_i8),(-63_i8),54_i8,8_i8,(-86_i8)];
_9 = Adt22 { fld0: _6,fld1: 48719779283428296297347926450895104144_i128,fld2: _4,fld3: 8697_u16,fld4: 6164293218766971809_u64 };
_9.fld2 = _4;
RET = !124_u8;
_9 = Adt22 { fld0: _6,fld1: (-69944830537610165277206473801253268911_i128),fld2: _4,fld3: 63832_u16,fld4: 6938983952852393969_u64 };
_6 = _9.fld0;
RET = 243_u8;
_6 = _9.fld3 >= _9.fld3;
_9.fld0 = _6 | _6;
_6 = !_9.fld0;
RET = !69_u8;
_5 = [_9.fld1,_9.fld1,_9.fld1];
_1 = [(-124_i8),46_i8,(-118_i8),105_i8,(-43_i8),45_i8,(-102_i8),(-69_i8)];
_9.fld3 = !57301_u16;
RET = 247_u8;
Goto(bb5)
}
bb7 = {
RET = !136_u8;
RET = _4 as u8;
_3 = _2;
_1 = [30_i8,(-94_i8),(-103_i8),(-55_i8),12_i8,124_i8,(-46_i8),(-6_i8)];
_2 = _3;
_3 = _2;
_5 = [(-114535584887464069643826853641681534868_i128),(-124225175834394052529885991900232467915_i128),44637431419557198589393318817976353800_i128];
_2 = _3;
_3 = _2;
_4 = (-61451384073523858294064461769207627018_i128) as isize;
_5 = [127647560613677004674661139553295933171_i128,(-44189048564709770148101880308088060113_i128),(-162090670478786228074664243169933515647_i128)];
_4 = (-9223372036854775808_isize);
_5 = [16445276434434300465565967771060653021_i128,(-85480205900456078478560120124224183968_i128),125020397207759325169676557950951617257_i128];
RET = !249_u8;
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_9.fld0 = _6;
_13 = !_10;
_9.fld1 = (-103150357332471577660160357875090953033_i128);
RET = !216_u8;
_9.fld0 = _12 < _6;
_14 = 7185888150980955458_i64 as f32;
_9 = Adt22 { fld0: _6,fld1: 12627337147777959765120604813358253045_i128,fld2: _13,fld3: 27324_u16,fld4: 5810949550717721546_u64 };
_18.fld5.1.fld0 = [RET,RET,RET];
_9.fld2 = -_10;
_14 = 21518_i16 as f32;
_20 = _13 as f32;
RET = _9.fld1 as u8;
_18.fld5.2 = _9.fld3 as u8;
_11 = core::ptr::addr_of_mut!(_15);
_9.fld0 = _12 ^ _12;
_18.fld5.1.fld0 = [RET,_18.fld5.2,RET];
_14 = _20;
Goto(bb9)
}
bb9 = {
Call(_24 = dump_var(13_usize, 1_usize, Move(_1), 2_usize, Move(_2), 13_usize, Move(_13), 10_usize, Move(_10)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0();
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: f64,
fld1: i64,

},
Variant1{
fld0: u128,
fld1: i128,
fld2: u32,
fld3: u8,

},
Variant2{
fld0: u64,
fld1: u16,
fld2: u8,
fld3: f64,

},
Variant3{
fld0: u16,
fld1: i8,
fld2: i64,

}}
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt22{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt22 {
fld0: bool,
fld1: i128,
fld2: isize,
fld3: u16,
fld4: u64,
}
impl PrintFDebug for Adt25{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt25{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt25 {
fld0: *mut i64,
fld1: char,
fld2: i128,
fld3: Adt22,
fld4: i16,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: Adt22,
fld1: Adt18,
fld2: i64,
fld3: [u32; 3],
fld4: [bool; 5],
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: [u8; 3],
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: char,

},
Variant1{
fld0: bool,
fld1: *mut f32,
fld2: *const Adt25,
fld3: [u8; 3],
fld4: *mut (Adt25, *const *mut i64, u32, i16),
fld5: [bool; 2],
fld6: *mut i64,
fld7: u32,

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: [u32; 3],
fld1: char,
fld2: *const Adt25,
fld3: u16,
fld4: [i64; 1],
fld5: i32,
fld6: u32,

},
Variant1{
fld0: *const *mut i64,
fld1: char,
fld2: *mut i64,
fld3: f32,
fld4: u8,
fld5: *const *const *mut i64,
fld6: Adt51,
fld7: i128,

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt76{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt76 {
fld0: *const i32,
fld1: Adt18,
fld2: *const Adt25,
fld3: *mut bool,
fld4: *mut f32,
fld5: (char, Adt56, u8),
}

