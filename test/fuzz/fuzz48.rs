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
pub fn fn0(mut _1: u16,mut _2: u64,mut _3: isize,mut _4: u128) -> u16 {
mir! {
type RET = u16;
let _5: *mut Adt47;
let _6: (Adt18, *const i16);
let _7: bool;
let _8: (*const u128, usize, &'static i64, &'static *mut *const i128);
let _9: i64;
let _10: char;
let _11: i32;
let _12: isize;
let _13: char;
let _14: Adt81;
let _15: isize;
let _16: *const &'static Adt18;
let _17: f64;
let _18: (Adt17, i32, Adt17, usize);
let _19: Adt47;
let _20: f32;
let _21: Adt77;
let _22: *const &'static *mut *const i128;
let _23: bool;
let _24: bool;
let _25: (*const u128, usize, &'static i64, &'static *mut *const i128);
let _26: [usize; 5];
let _27: char;
let _28: *mut f32;
let _29: u32;
let _30: usize;
let _31: char;
let _32: isize;
let _33: ();
let _34: ();
{
_1 = !37496_u16;
_4 = !221495433111939228772407561846863935672_u128;
RET = _1 ^ _1;
_1 = RET;
_3 = (-29_isize);
_2 = 290295513457691510_u64 >> _1;
_2 = !17263068352730220284_u64;
_2 = !15328510007209986402_u64;
_3 = !(-9223372036854775808_isize);
RET = _1 | _1;
_2 = !13122705022159385177_u64;
RET = 4972069501623226522_i64 as u16;
_7 = false;
_2 = (-5486506194324715989_i64) as u64;
_1 = _2 as u16;
_1 = RET;
_8.1 = 4473796521610754695_usize;
_8.1 = 115203677536534357_usize - 3_usize;
RET = 145024055420426493905809958112714269311_i128 as u16;
_8.0 = core::ptr::addr_of!(_4);
_3 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = _7 as u16;
_7 = true;
_8.2 = &_9;
_7 = false | false;
Call(_6.1 = fn1(_8.1, Move(_8.0), _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 181403276658497684257640754592969034514_u128 & 305630084933055343698194467306654366446_u128;
_4 = 453321019881928707_i64 as u128;
RET = _1 & _1;
_8.0 = core::ptr::addr_of!(_4);
_9 = 8262634980353785749_i64;
RET = !_1;
_7 = !false;
_9 = 8959118995404336166_i64 * (-7960668239356494083_i64);
RET = _1 ^ _1;
RET = !_1;
_2 = _7 as u64;
_3 = 117_isize;
_10 = '\u{cf854}';
_8.2 = &_9;
_2 = _4 as u64;
_11 = 1096773215_i32 + 1279295626_i32;
_8.2 = &_9;
_8.2 = &_9;
_11 = (-799909071_i32) & (-955130338_i32);
_9 = 1886911354673435341_i64;
RET = !_1;
Call(_3 = fn10(Move(_6.1), _2, RET, _11, _11, RET, _11, _8.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.1 = !0_usize;
_8.2 = &_9;
_12 = -_3;
_13 = _10;
_8.0 = core::ptr::addr_of!(_4);
_4 = !80780715025292791505829885131937316070_u128;
_11 = -1167535569_i32;
_7 = false;
_3 = !_12;
_10 = _13;
RET = _1 << _11;
_11 = _13 as i32;
_10 = _13;
_10 = _13;
_9 = 7214588099269911611_i64 ^ (-3112986761964732677_i64);
_15 = _7 as isize;
_8.1 = 8793109473352176914_usize;
_2 = !2677578518407676102_u64;
_10 = _13;
_2 = 6107266101682722930_u64 >> _3;
_7 = false;
_13 = _10;
_8.0 = core::ptr::addr_of!(_4);
_3 = _12 * _15;
_10 = _13;
_7 = _3 <= _3;
_13 = _10;
Goto(bb3)
}
bb3 = {
RET = !_1;
_13 = _10;
_10 = _13;
_8.1 = 7_usize;
RET = !_1;
_3 = _15 & _12;
_8.1 = _3 as usize;
_10 = _13;
_1 = RET;
_10 = _13;
_18.2 = Adt17::Variant0 { fld0: 154_u8,fld1: _2 };
_18.1 = _11;
_18.0 = Adt17::Variant0 { fld0: 50_u8,fld1: _2 };
_18.0 = Adt17::Variant0 { fld0: 15_u8,fld1: Field::<u64>(Variant(_18.2, 0), 1) };
_17 = _4 as f64;
RET = _12 as u16;
_18.0 = Adt17::Variant1 { fld0: _9,fld1: 29_u8,fld2: (-55093801313808905041708050013473388249_i128),fld3: _17,fld4: (-29476_i16),fld5: _4 };
_11 = _18.1 - _18.1;
_8.2 = &place!(Field::<i64>(Variant(_18.0, 1), 0));
_18.0 = Adt17::Variant1 { fld0: _9,fld1: 27_u8,fld2: 39313116962442298516321929383665750467_i128,fld3: _17,fld4: 30279_i16,fld5: _4 };
place!(Field::<u64>(Variant(_18.2, 0), 1)) = !_2;
Goto(bb4)
}
bb4 = {
place!(Field::<u8>(Variant(_18.0, 1), 1)) = 220_u8 + 36_u8;
_15 = !_12;
_20 = _11 as f32;
_3 = _7 as isize;
RET = _1;
place!(Field::<u64>(Variant(_18.2, 0), 1)) = (-17873_i16) as u64;
_6.1 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_18.0, 1), 4)));
_8.1 = 11442576304228862330_usize;
place!(Field::<f64>(Variant(_18.0, 1), 3)) = _17 - _17;
place!(Field::<f64>(Variant(_18.0, 1), 3)) = Field::<i64>(Variant(_18.0, 1), 0) as f64;
match _8.1 {
0 => bb1,
1 => bb5,
2 => bb6,
11442576304228862330 => bb8,
_ => bb7
}
}
bb5 = {
RET = !_1;
_13 = _10;
_10 = _13;
_8.1 = 7_usize;
RET = !_1;
_3 = _15 & _12;
_8.1 = _3 as usize;
_10 = _13;
_1 = RET;
_10 = _13;
_18.2 = Adt17::Variant0 { fld0: 154_u8,fld1: _2 };
_18.1 = _11;
_18.0 = Adt17::Variant0 { fld0: 50_u8,fld1: _2 };
_18.0 = Adt17::Variant0 { fld0: 15_u8,fld1: Field::<u64>(Variant(_18.2, 0), 1) };
_17 = _4 as f64;
RET = _12 as u16;
_18.0 = Adt17::Variant1 { fld0: _9,fld1: 29_u8,fld2: (-55093801313808905041708050013473388249_i128),fld3: _17,fld4: (-29476_i16),fld5: _4 };
_11 = _18.1 - _18.1;
_8.2 = &place!(Field::<i64>(Variant(_18.0, 1), 0));
_18.0 = Adt17::Variant1 { fld0: _9,fld1: 27_u8,fld2: 39313116962442298516321929383665750467_i128,fld3: _17,fld4: 30279_i16,fld5: _4 };
place!(Field::<u64>(Variant(_18.2, 0), 1)) = !_2;
Goto(bb4)
}
bb6 = {
_8.1 = !0_usize;
_8.2 = &_9;
_12 = -_3;
_13 = _10;
_8.0 = core::ptr::addr_of!(_4);
_4 = !80780715025292791505829885131937316070_u128;
_11 = -1167535569_i32;
_7 = false;
_3 = !_12;
_10 = _13;
RET = _1 << _11;
_11 = _13 as i32;
_10 = _13;
_10 = _13;
_9 = 7214588099269911611_i64 ^ (-3112986761964732677_i64);
_15 = _7 as isize;
_8.1 = 8793109473352176914_usize;
_2 = !2677578518407676102_u64;
_10 = _13;
_2 = 6107266101682722930_u64 >> _3;
_7 = false;
_13 = _10;
_8.0 = core::ptr::addr_of!(_4);
_3 = _12 * _15;
_10 = _13;
_7 = _3 <= _3;
_13 = _10;
Goto(bb3)
}
bb7 = {
_4 = 181403276658497684257640754592969034514_u128 & 305630084933055343698194467306654366446_u128;
_4 = 453321019881928707_i64 as u128;
RET = _1 & _1;
_8.0 = core::ptr::addr_of!(_4);
_9 = 8262634980353785749_i64;
RET = !_1;
_7 = !false;
_9 = 8959118995404336166_i64 * (-7960668239356494083_i64);
RET = _1 ^ _1;
RET = !_1;
_2 = _7 as u64;
_3 = 117_isize;
_10 = '\u{cf854}';
_8.2 = &_9;
_2 = _4 as u64;
_11 = 1096773215_i32 + 1279295626_i32;
_8.2 = &_9;
_8.2 = &_9;
_11 = (-799909071_i32) & (-955130338_i32);
_9 = 1886911354673435341_i64;
RET = !_1;
Call(_3 = fn10(Move(_6.1), _2, RET, _11, _11, RET, _11, _8.1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_18.3 = 128264718779330809711433818426751907487_i128 as usize;
_8.2 = &_9;
Call(_4 = core::intrinsics::bswap(Field::<u128>(Variant(_18.0, 1), 5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_17 = Field::<f64>(Variant(_18.0, 1), 3) + Field::<f64>(Variant(_18.0, 1), 3);
_7 = _3 >= _3;
_25.2 = Move(_8.2);
_4 = Field::<u128>(Variant(_18.0, 1), 5) << _3;
_18.0 = Adt17::Variant0 { fld0: 44_u8,fld1: _2 };
_24 = _7;
_22 = core::ptr::addr_of!(_25.3);
_28 = core::ptr::addr_of_mut!(_20);
_5 = core::ptr::addr_of_mut!(_19);
_10 = _13;
RET = _1 | _1;
place!(Field::<u8>(Variant(_18.2, 0), 0)) = 148_u8;
_26 = [_18.3,_18.3,_8.1,_18.3,_8.1];
place!(Field::<u8>(Variant(_18.0, 0), 0)) = Field::<u8>(Variant(_18.2, 0), 0) - Field::<u8>(Variant(_18.2, 0), 0);
SetDiscriminant(_18.2, 1);
_8.2 = &_9;
_25.2 = &place!(Field::<i64>(Variant(_18.2, 1), 0));
_18.3 = _8.1;
Goto(bb10)
}
bb10 = {
_20 = _4 as f32;
_8.0 = core::ptr::addr_of!(_4);
_23 = !_7;
_25.2 = Move(_8.2);
_24 = _3 >= _3;
_23 = _24;
place!(Field::<f64>(Variant(_18.2, 1), 3)) = -_17;
_4 = 132420091636919107811107119510010194489_u128 + 167797369669921999503120321577377320561_u128;
_25.2 = &place!(Field::<i64>(Variant(_18.2, 1), 0));
_18.2 = Adt17::Variant1 { fld0: _9,fld1: Field::<u8>(Variant(_18.0, 0), 0),fld2: 88624934065569515602504641143417839447_i128,fld3: _17,fld4: 12682_i16,fld5: _4 };
_6.0 = Adt18::Variant0 { fld0: _20,fld1: _10,fld2: _12,fld3: 3720397690_u32,fld4: 19390_i16,fld5: _4,fld6: _8.1,fld7: Move(_18.0) };
_4 = !Field::<u128>(Variant(_6.0, 0), 5);
_19 = Adt47::Variant0 { fld0: Move(Field::<Adt17>(Variant(_6.0, 0), 7)) };
place!(Field::<f32>(Variant(_6.0, 0), 0)) = (-11_i8) as f32;
_18.2 = Adt17::Variant0 { fld0: Field::<u8>(Variant(Field::<Adt17>(Variant((*_5), 0), 0), 0), 0),fld1: _2 };
SetDiscriminant(Field::<Adt17>(Variant((*_5), 0), 0), 1);
_13 = Field::<char>(Variant(_6.0, 0), 1);
_8.2 = &place!(Field::<i64>(Variant(place!(Field::<Adt17>(Variant(_19, 0), 0)), 1), 0));
_15 = _12 << RET;
match _8.1 {
0 => bb11,
1 => bb12,
11442576304228862330 => bb14,
_ => bb13
}
}
bb11 = {
RET = !_1;
_13 = _10;
_10 = _13;
_8.1 = 7_usize;
RET = !_1;
_3 = _15 & _12;
_8.1 = _3 as usize;
_10 = _13;
_1 = RET;
_10 = _13;
_18.2 = Adt17::Variant0 { fld0: 154_u8,fld1: _2 };
_18.1 = _11;
_18.0 = Adt17::Variant0 { fld0: 50_u8,fld1: _2 };
_18.0 = Adt17::Variant0 { fld0: 15_u8,fld1: Field::<u64>(Variant(_18.2, 0), 1) };
_17 = _4 as f64;
RET = _12 as u16;
_18.0 = Adt17::Variant1 { fld0: _9,fld1: 29_u8,fld2: (-55093801313808905041708050013473388249_i128),fld3: _17,fld4: (-29476_i16),fld5: _4 };
_11 = _18.1 - _18.1;
_8.2 = &place!(Field::<i64>(Variant(_18.0, 1), 0));
_18.0 = Adt17::Variant1 { fld0: _9,fld1: 27_u8,fld2: 39313116962442298516321929383665750467_i128,fld3: _17,fld4: 30279_i16,fld5: _4 };
place!(Field::<u64>(Variant(_18.2, 0), 1)) = !_2;
Goto(bb4)
}
bb12 = {
place!(Field::<u8>(Variant(_18.0, 1), 1)) = 220_u8 + 36_u8;
_15 = !_12;
_20 = _11 as f32;
_3 = _7 as isize;
RET = _1;
place!(Field::<u64>(Variant(_18.2, 0), 1)) = (-17873_i16) as u64;
_6.1 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_18.0, 1), 4)));
_8.1 = 11442576304228862330_usize;
place!(Field::<f64>(Variant(_18.0, 1), 3)) = _17 - _17;
place!(Field::<f64>(Variant(_18.0, 1), 3)) = Field::<i64>(Variant(_18.0, 1), 0) as f64;
match _8.1 {
0 => bb1,
1 => bb5,
2 => bb6,
11442576304228862330 => bb8,
_ => bb7
}
}
bb13 = {
_8.1 = !0_usize;
_8.2 = &_9;
_12 = -_3;
_13 = _10;
_8.0 = core::ptr::addr_of!(_4);
_4 = !80780715025292791505829885131937316070_u128;
_11 = -1167535569_i32;
_7 = false;
_3 = !_12;
_10 = _13;
RET = _1 << _11;
_11 = _13 as i32;
_10 = _13;
_10 = _13;
_9 = 7214588099269911611_i64 ^ (-3112986761964732677_i64);
_15 = _7 as isize;
_8.1 = 8793109473352176914_usize;
_2 = !2677578518407676102_u64;
_10 = _13;
_2 = 6107266101682722930_u64 >> _3;
_7 = false;
_13 = _10;
_8.0 = core::ptr::addr_of!(_4);
_3 = _12 * _15;
_10 = _13;
_7 = _3 <= _3;
_13 = _10;
Goto(bb3)
}
bb14 = {
place!(Field::<i16>(Variant(_6.0, 0), 4)) = _7 as i16;
place!(Field::<char>(Variant(_6.0, 0), 1)) = _10;
place!(Field::<u8>(Variant(place!(Field::<Adt17>(Variant(_19, 0), 0)), 1), 1)) = !Field::<u8>(Variant(_18.2, 0), 0);
_29 = 16672080_u32 - 3815574197_u32;
SetDiscriminant(_18.2, 0);
place!(Field::<usize>(Variant(_6.0, 0), 6)) = _18.1 as usize;
_18.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(Field::<Adt17>(Variant((*_5), 0), 0), 1), 1),fld1: _2 };
_2 = Field::<u64>(Variant(_18.0, 0), 1);
_8.0 = core::ptr::addr_of!(_4);
place!(Field::<i128>(Variant(place!(Field::<Adt17>(Variant((*_5), 0), 0)), 1), 2)) = 141691479511713781718863662258529613323_i128 | (-153892293947782012720816601825616066033_i128);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 9_usize, Move(_9), 15_usize, Move(_15), 24_usize, Move(_24), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 2_usize, Move(_2), 26_usize, Move(_26), 23_usize, Move(_23), 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: usize,mut _2: *const u128,mut _3: bool) -> *const i16 {
mir! {
type RET = *const i16;
let _4: f64;
let _5: [u16; 1];
let _6: *mut f32;
let _7: bool;
let _8: f64;
let _9: [usize; 2];
let _10: isize;
let _11: f32;
let _12: char;
let _13: (i8, u64);
let _14: isize;
let _15: char;
let _16: isize;
let _17: u128;
let _18: i32;
let _19: i32;
let _20: Adt55;
let _21: f32;
let _22: ([u64; 2], [bool; 5], Adt18, [u64; 8]);
let _23: [char; 6];
let _24: char;
let _25: [i16; 2];
let _26: *const *const i16;
let _27: f64;
let _28: &'static i64;
let _29: isize;
let _30: *mut f32;
let _31: ();
let _32: ();
{
_3 = false;
_3 = !false;
Call(_1 = core::intrinsics::bswap(6221975199398061893_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 2_usize * 2_usize;
_1 = 2_usize;
_3 = !false;
_3 = false ^ true;
_4 = 16312679013317059002_u64 as f64;
_3 = false;
_3 = false;
_1 = !7_usize;
_1 = 191325433898611649732619873393920067880_u128 as usize;
_5 = [28244_u16];
_3 = true;
_1 = !9180535262634196771_usize;
_1 = 5768902980358840118_usize << (-85_i8);
_1 = 5_usize;
_5 = [54638_u16];
_4 = 1476320885_i32 as f64;
_7 = !_3;
_5 = [58818_u16];
_4 = 44192947579731847524150427431007898476_u128 as f64;
_1 = 235_u8 as usize;
_7 = _3;
_5 = [15604_u16];
_3 = _7 > _7;
Goto(bb2)
}
bb2 = {
_7 = !_3;
_3 = _4 >= _4;
_5 = [35347_u16];
_1 = 32_i8 as usize;
_7 = _3 < _3;
_4 = 7968548300172428588_i64 as f64;
_4 = 134_u8 as f64;
_3 = _4 > _4;
_7 = !_3;
_3 = _7;
_8 = _1 as f64;
_3 = _7;
_8 = _4;
_3 = !_7;
_7 = _1 != _1;
_9 = [_1,_1];
_7 = !_3;
_10 = _3 as isize;
_3 = !_7;
_1 = 29393_i16 as usize;
_10 = (-9223372036854775808_isize);
Goto(bb3)
}
bb3 = {
_5 = [32143_u16];
Call(RET = fn2(_10, _10, _10, _8, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _8;
_6 = core::ptr::addr_of_mut!(_11);
_5 = [55841_u16];
_1 = 0_usize;
_9[_1] = 16518_i16 as usize;
_8 = _4;
_13.0 = (-99_i8);
_3 = _10 < _10;
(*_6) = 4747160325197386610_u64 as f32;
_5[_1] = 258_u16;
_13.0 = (-78_i8) | (-11_i8);
_13.0 = -119_i8;
_13.1 = 7635707942592678012_u64 | 2559633895744214214_u64;
_12 = '\u{6b9c8}';
_5 = [51557_u16];
_3 = _7 ^ _7;
_5[_1] = _9[_1] as u16;
_5 = [24289_u16];
match _1 {
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
0 => bb12,
_ => bb11
}
}
bb5 = {
_5 = [32143_u16];
Call(RET = fn2(_10, _10, _10, _8, _5, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_7 = !_3;
_3 = _4 >= _4;
_5 = [35347_u16];
_1 = 32_i8 as usize;
_7 = _3 < _3;
_4 = 7968548300172428588_i64 as f64;
_4 = 134_u8 as f64;
_3 = _4 > _4;
_7 = !_3;
_3 = _7;
_8 = _1 as f64;
_3 = _7;
_8 = _4;
_3 = !_7;
_7 = _1 != _1;
_9 = [_1,_1];
_7 = !_3;
_10 = _3 as isize;
_3 = !_7;
_1 = 29393_i16 as usize;
_10 = (-9223372036854775808_isize);
Goto(bb3)
}
bb7 = {
_1 = 2_usize * 2_usize;
_1 = 2_usize;
_3 = !false;
_3 = false ^ true;
_4 = 16312679013317059002_u64 as f64;
_3 = false;
_3 = false;
_1 = !7_usize;
_1 = 191325433898611649732619873393920067880_u128 as usize;
_5 = [28244_u16];
_3 = true;
_1 = !9180535262634196771_usize;
_1 = 5768902980358840118_usize << (-85_i8);
_1 = 5_usize;
_5 = [54638_u16];
_4 = 1476320885_i32 as f64;
_7 = !_3;
_5 = [58818_u16];
_4 = 44192947579731847524150427431007898476_u128 as f64;
_1 = 235_u8 as usize;
_7 = _3;
_5 = [15604_u16];
_3 = _7 > _7;
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
_14 = _10;
_20.fld1 = core::ptr::addr_of_mut!(_1);
_20.fld0 = [_13.0,_13.0,_13.0,_13.0,_13.0,_13.0,_13.0,_13.0];
_20.fld2.0 = [_13.1,_13.1];
_20.fld2.0[_1] = !_13.1;
_9 = [_1,_1];
_1 = (-75408692221816175232667846804786600469_i128) as usize;
_15 = _12;
_10 = _14;
_8 = _4;
_20.fld6 = Adt17::Variant0 { fld0: 66_u8,fld1: _13.1 };
place!(Field::<u8>(Variant(_20.fld6, 0), 0)) = _13.0 as u8;
_19 = -1157135489_i32;
_21 = _11 - _11;
_12 = _15;
_20.fld1 = core::ptr::addr_of_mut!(_1);
_13.1 = !Field::<u64>(Variant(_20.fld6, 0), 1);
Call(_16 = fn3(_19, Move(RET), _7, _3, _3, _21, _20.fld2.0, _3, Move(_20.fld1), _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_15 = _12;
_10 = !_14;
_2 = core::ptr::addr_of!(_17);
_18 = _19;
_14 = _10 * _10;
_13.1 = Field::<u64>(Variant(_20.fld6, 0), 1) - Field::<u64>(Variant(_20.fld6, 0), 1);
_17 = 232630736760885851100952385716696305955_u128 - 38275820207459472550655838105448110888_u128;
_22.2 = Adt18::Variant1 { fld0: Move(_20.fld6),fld1: (*_2),fld2: _19 };
_8 = _14 as f64;
_22.1 = [_7,_7,_3,_3,_7];
_23 = [_12,_15,_15,_12,_12,_12];
Call(place!(Field::<i32>(Variant(_22.2, 1), 2)) = core::intrinsics::transmute(_19), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_20.fld6 = Move(Field::<Adt17>(Variant(_22.2, 1), 0));
_20.fld3 = Move(_6);
place!(Field::<i32>(Variant(_22.2, 1), 2)) = -_19;
(*_2) = Field::<u128>(Variant(_22.2, 1), 1) + Field::<u128>(Variant(_22.2, 1), 1);
place!(Field::<Adt17>(Variant(_22.2, 1), 0)) = Adt17::Variant1 { fld0: (-1680961213791581080_i64),fld1: Field::<u8>(Variant(_20.fld6, 0), 0),fld2: 54722611037067731479359313850890557212_i128,fld3: _8,fld4: 29521_i16,fld5: _17 };
_15 = _12;
SetDiscriminant(_20.fld6, 0);
_20.fld6 = Adt17::Variant1 { fld0: (-271623868708606129_i64),fld1: Field::<u8>(Variant(Field::<Adt17>(Variant(_22.2, 1), 0), 1), 1),fld2: 23693821967092093364119438070618844466_i128,fld3: Field::<f64>(Variant(Field::<Adt17>(Variant(_22.2, 1), 0), 1), 3),fld4: 31228_i16,fld5: Field::<u128>(Variant(Field::<Adt17>(Variant(_22.2, 1), 0), 1), 5) };
_20.fld5 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_20.fld6, 1), 4)));
Call(_4 = fn7(Move(_20.fld5), _13, (*_2), (*_2), Field::<u128>(Variant(_20.fld6, 1), 5), Move(_2), _3, (*_2)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
place!(Field::<i128>(Variant(place!(Field::<Adt17>(Variant(_22.2, 1), 0)), 1), 2)) = 61026933368943410556592729904051271481_i128;
_20.fld0 = [_13.0,_13.0,_13.0,_13.0,_13.0,_13.0,_13.0,_13.0];
_15 = _12;
place!(Field::<i16>(Variant(_20.fld6, 1), 4)) = -2333_i16;
place!(Field::<u128>(Variant(_22.2, 1), 1)) = !Field::<u128>(Variant(_20.fld6, 1), 5);
_25 = [Field::<i16>(Variant(_20.fld6, 1), 4),Field::<i16>(Variant(_20.fld6, 1), 4)];
_20.fld5 = core::ptr::addr_of!(place!(Field::<i16>(Variant(place!(Field::<Adt17>(Variant(_22.2, 1), 0)), 1), 4)));
RET = Move(_20.fld5);
_26 = core::ptr::addr_of!(_20.fld5);
_14 = 60900_u16 as isize;
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(1_usize, 25_usize, Move(_25), 16_usize, Move(_16), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(1_usize, 9_usize, Move(_9), 3_usize, Move(_3), 15_usize, Move(_15), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: f64,mut _5: [u16; 1],mut _6: bool) -> *const i16 {
mir! {
type RET = *const i16;
let _7: u128;
let _8: *const *const u128;
let _9: [usize; 5];
let _10: [u128; 4];
let _11: u8;
let _12: bool;
let _13: [i16; 2];
let _14: *mut ([u64; 2],);
let _15: char;
let _16: *mut f32;
let _17: u64;
let _18: i16;
let _19: &'static *mut *const i128;
let _20: &'static [bool; 5];
let _21: *const *const u128;
let _22: (*const u128, usize, &'static i64, &'static *mut *const i128);
let _23: char;
let _24: isize;
let _25: (&'static u32, &'static [isize; 7], &'static (*const [u64; 2], u8, [bool; 5]), i16);
let _26: *const u32;
let _27: i8;
let _28: (Adt17, i32, Adt17, usize);
let _29: f32;
let _30: (i8, u64);
let _31: [isize; 4];
let _32: ();
let _33: ();
{
_6 = !true;
_1 = _2 | _2;
_6 = true;
_1 = !_3;
_6 = true | true;
_5 = [42738_u16];
_5 = [7770_u16];
_1 = _2;
_5 = [33716_u16];
_6 = false;
_4 = 1717596713_i32 as f64;
_5 = [21637_u16];
_4 = 11128_i16 as f64;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
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
_3 = _1 | _2;
_5 = [19327_u16];
_2 = _3;
_7 = 612495348_u32 as u128;
_2 = _3;
_9 = [0_usize,7260417542755206556_usize,4428791974604181884_usize,14536457445128394118_usize,2_usize];
_1 = _2 >> _3;
_3 = _2;
_6 = !true;
_5 = [3721_u16];
_7 = 78936173973387598954531932644969805262_u128;
_5 = [6177_u16];
_7 = 268619935182937322663799384219181122558_u128 - 275878810565219735310174905850925915771_u128;
_4 = 26360_i16 as f64;
_2 = -_1;
_3 = _2;
_6 = _1 > _3;
_5 = [59596_u16];
_7 = !171380042986196671788339471971885311714_u128;
_1 = _3;
_1 = (-17477_i16) as isize;
_3 = _2;
_2 = _3;
_5 = [16739_u16];
_7 = 25656306929583885112781107138223772686_u128;
_10 = [_7,_7,_7,_7];
_6 = _3 == _1;
_9 = [0_usize,18388500885672262434_usize,7_usize,8615906078335375796_usize,6_usize];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb7,
25656306929583885112781107138223772686 => bb9,
_ => bb8
}
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_7 = 140166343275964496865458469630005934141_u128;
_4 = 9322775805008127259_usize as f64;
_7 = 22380868326735706994011992603816847648_u128 | 219516172607274984647137061966041091176_u128;
_10 = [_7,_7,_7,_7];
_6 = false;
_4 = _3 as f64;
_11 = 21_u8;
_2 = 35_i8 as isize;
_2 = _3 >> _3;
_11 = 126_u8 + 141_u8;
_6 = _2 < _3;
_10 = [_7,_7,_7,_7];
_12 = !_6;
_3 = !_2;
_6 = _12;
_5 = [3781_u16];
_11 = 106_u8;
_1 = !_2;
Goto(bb10)
}
bb10 = {
_1 = _2 ^ _2;
_2 = -_1;
_2 = '\u{104c32}' as isize;
_3 = _1;
match _11 {
0 => bb1,
106 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_12 = _2 != _1;
_7 = 124290329188955513357557096883163731013_u128 + 187449709918731507088252594479648974147_u128;
_2 = _1 + _3;
_13 = [13071_i16,14388_i16];
_2 = _11 as isize;
_15 = '\u{68bc0}';
_17 = 402176573645605221_usize as u64;
_7 = 236203696957751542736399702733878362126_u128 + 1637948315118194173084591941484186969_u128;
RET = core::ptr::addr_of!(_18);
_18 = _17 as i16;
_18 = (-20752_i16);
_11 = !55_u8;
_5 = [41172_u16];
_4 = 1970271880_u32 as f64;
_13 = [(*RET),(*RET)];
_2 = _11 as isize;
_4 = 2_usize as f64;
_22.0 = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!(_18);
_13 = [(*RET),(*RET)];
_22.1 = (-113494216850309229644065033011579635159_i128) as usize;
_12 = _6;
_9 = [_22.1,_22.1,_22.1,_22.1,_22.1];
Goto(bb13)
}
bb13 = {
_25.3 = (*RET) & (*RET);
(*RET) = _25.3;
RET = core::ptr::addr_of!(_25.3);
_18 = -(*RET);
_10 = [_7,_7,_7,_7];
_24 = _12 as isize;
_3 = _4 as isize;
_27 = 23_i8 & (-49_i8);
_23 = _15;
_28.3 = _7 as usize;
_10 = [_7,_7,_7,_7];
_18 = -(*RET);
_13 = [(*RET),_18];
(*RET) = _18 * _18;
_28.1 = 212213857_u32 as i32;
_15 = _23;
RET = core::ptr::addr_of!(_25.3);
_13 = [(*RET),_25.3];
_8 = core::ptr::addr_of!(_22.0);
_28.1 = !(-1983487516_i32);
_28.1 = !(-1401104172_i32);
(*_8) = core::ptr::addr_of!(_7);
_28.2 = Adt17::Variant1 { fld0: (-2927735982224256264_i64),fld1: _11,fld2: 46153698209415667754250630474562037209_i128,fld3: _4,fld4: (*RET),fld5: _7 };
place!(Field::<i128>(Variant(_28.2, 1), 2)) = -80155797137704397829298740912862976815_i128;
_28.3 = !_22.1;
Goto(bb14)
}
bb14 = {
_8 = core::ptr::addr_of!(_22.0);
place!(Field::<u8>(Variant(_28.2, 1), 1)) = _11 + _11;
_18 = Field::<i16>(Variant(_28.2, 1), 4);
_24 = _1;
_15 = _23;
_25.3 = Field::<i16>(Variant(_28.2, 1), 4) & Field::<i16>(Variant(_28.2, 1), 4);
_7 = Field::<u128>(Variant(_28.2, 1), 5) - Field::<u128>(Variant(_28.2, 1), 5);
_29 = Field::<f64>(Variant(_28.2, 1), 3) as f32;
_28.2 = Adt17::Variant0 { fld0: _11,fld1: _17 };
_27 = (-39_i8);
_28.2 = Adt17::Variant1 { fld0: 6559919162663262967_i64,fld1: _11,fld2: (-114500669610814112739133318984866151728_i128),fld3: _4,fld4: (*RET),fld5: _7 };
_9 = [_28.3,_22.1,_28.3,_22.1,_28.3];
(*RET) = Field::<i16>(Variant(_28.2, 1), 4);
_2 = _1 - _24;
_11 = _17 as u8;
(*RET) = _18;
_21 = core::ptr::addr_of!((*_8));
_27 = _28.1 as i8;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(2_usize, 27_usize, Move(_27), 7_usize, Move(_7), 11_usize, Move(_11), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(2_usize, 6_usize, Move(_6), 18_usize, Move(_18), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: i32,mut _2: *const i16,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: f32,mut _7: [u64; 2],mut _8: bool,mut _9: *mut usize,mut _10: usize) -> isize {
mir! {
type RET = isize;
let _11: [i16; 2];
let _12: f32;
let _13: [bool; 4];
let _14: (i8, u64);
let _15: *mut Adt47;
let _16: Adt47;
let _17: &'static (*const [u64; 2], u8, [bool; 5]);
let _18: (f64,);
let _19: Adt81;
let _20: (Adt18, (*const u128, u64, u64), u32);
let _21: [char; 6];
let _22: i64;
let _23: *const u32;
let _24: *const &'static *mut *const i128;
let _25: bool;
let _26: Adt77;
let _27: f32;
let _28: [bool; 4];
let _29: u64;
let _30: [usize; 5];
let _31: isize;
let _32: ();
let _33: ();
{
_6 = 4424310700010460652_u64 as f32;
_1 = (-1104247235_i32) + 1897849713_i32;
RET = (-9223372036854775808_isize);
_11 = [18086_i16,(-3480_i16)];
_9 = core::ptr::addr_of_mut!(_10);
_5 = !_8;
_9 = core::ptr::addr_of_mut!(_10);
Call(_12 = fn4(_5, _1, _8, (*_9), (*_9), _4, Move(_9), _8, Move(_2), _7, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = [_5,_4,_4,_8];
_1 = 2098102768_i32;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
_10 = !4_usize;
RET = 51830_u16 as isize;
_14.0 = (-82_i8);
_4 = !_5;
Goto(bb2)
}
bb2 = {
RET = !(-9223372036854775808_isize);
_13 = [_5,_8,_8,_4];
_7 = [17058007368171341056_u64,532842495157544907_u64];
_7 = [13225376410752532634_u64,4507006106898365287_u64];
RET = 29_isize & 41_isize;
_4 = _10 == _10;
_9 = core::ptr::addr_of_mut!(_10);
_7 = [9275614276085511292_u64,12920535535026212732_u64];
_4 = _5;
_14.1 = 2702734734_u32 as u64;
_12 = _6 - _6;
_1 = !(-1251116393_i32);
_6 = -_12;
_14 = ((-127_i8), 11432849875187981983_u64);
_6 = -_12;
_11 = [23216_i16,(-23879_i16)];
_5 = _8;
match _14.0 {
0 => bb3,
340282366920938463463374607431768211329 => bb5,
_ => bb4
}
}
bb3 = {
_13 = [_5,_4,_4,_8];
_1 = 2098102768_i32;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
_10 = !4_usize;
RET = 51830_u16 as isize;
_14.0 = (-82_i8);
_4 = !_5;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_4 = _5 ^ _3;
_3 = _8 & _8;
(*_9) = 4_usize - 7_usize;
_14.1 = !6142973941881299727_u64;
_8 = !_3;
Call(_12 = fn6(_6, _14, _11, (*_9), _5, _14.0, _8, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = !_3;
Goto(bb7)
}
bb7 = {
_12 = RET as f32;
_9 = core::ptr::addr_of_mut!((*_9));
_5 = _8;
_9 = core::ptr::addr_of_mut!(_10);
_1 = !2012938326_i32;
_4 = _3 & _3;
_1 = (-1634254999_i32);
_12 = _1 as f32;
_6 = _12 - _12;
_18.0 = RET as f64;
_13 = [_3,_4,_4,_4];
_3 = !_4;
_15 = core::ptr::addr_of_mut!(_16);
(*_9) = _6 as usize;
_7 = [_14.1,_14.1];
_13 = [_5,_5,_8,_4];
_10 = 13848577282741529682_usize;
_6 = (-127562294422271592024242510941447492538_i128) as f32;
_13 = [_4,_3,_4,_3];
_18.0 = (-5344069240944469443_i64) as f64;
_13 = [_4,_8,_3,_8];
RET = 46_isize;
_14.0 = -(-79_i8);
_14.0 = 26_i8 ^ 111_i8;
_1 = _6 as i32;
_12 = _6;
_5 = _3;
match _10 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
13848577282741529682 => bb11,
_ => bb10
}
}
bb8 = {
_4 = !_3;
Goto(bb7)
}
bb9 = {
_4 = _5 ^ _3;
_3 = _8 & _8;
(*_9) = 4_usize - 7_usize;
_14.1 = !6142973941881299727_u64;
_8 = !_3;
Call(_12 = fn6(_6, _14, _11, (*_9), _5, _14.0, _8, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_13 = [_5,_4,_4,_8];
_1 = 2098102768_i32;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
_10 = !4_usize;
RET = 51830_u16 as isize;
_14.0 = (-82_i8);
_4 = !_5;
Goto(bb2)
}
bb11 = {
_10 = 3_usize & 12712268134559468895_usize;
_20.2 = 3656102010_u32;
_14.0 = 50_i8 ^ (-79_i8);
_21 = ['\u{499cf}','\u{e0faf}','\u{69493}','\u{f29ec}','\u{9c8e2}','\u{4171c}'];
_18.0 = 223442871164981742254995492194818620252_u128 as f64;
(*_9) = 11447_i16 as usize;
RET = '\u{2bcf9}' as isize;
_14.0 = (-9_i8);
(*_9) = 8707347816846711843_i64 as usize;
(*_9) = 7_usize;
_11 = [11010_i16,16877_i16];
_15 = core::ptr::addr_of_mut!((*_15));
_11 = [2724_i16,27394_i16];
_8 = !_3;
_14 = ((-14_i8), 13773988972337841418_u64);
Goto(bb12)
}
bb12 = {
_18.0 = 213_u8 as f64;
_6 = _18.0 as f32;
_7 = [_14.1,_14.1];
_18.0 = 173_u8 as f64;
_6 = _18.0 as f32;
_20.2 = 272498601_u32 << _10;
_14.0 = 106_i8;
_14 = ((-101_i8), 9369956297559934965_u64);
_21 = ['\u{dad54}','\u{d11d0}','\u{c7031}','\u{ebcb9}','\u{10303e}','\u{107842}'];
_20.1.1 = !_14.1;
_20.1.1 = (*_9) as u64;
_25 = !_4;
RET = !67_isize;
_3 = _4;
_4 = !_5;
(*_9) = _14.0 as usize;
match _14.1 {
0 => bb9,
1 => bb2,
2 => bb11,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb13,
9369956297559934965 => bb15,
_ => bb14
}
}
bb13 = {
_4 = _5 ^ _3;
_3 = _8 & _8;
(*_9) = 4_usize - 7_usize;
_14.1 = !6142973941881299727_u64;
_8 = !_3;
Call(_12 = fn6(_6, _14, _11, (*_9), _5, _14.0, _8, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_4 = _5 ^ _3;
_3 = _8 & _8;
(*_9) = 4_usize - 7_usize;
_14.1 = !6142973941881299727_u64;
_8 = !_3;
Call(_12 = fn6(_6, _14, _11, (*_9), _5, _14.0, _8, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_8 = _25 <= _3;
(*_9) = 10484359441794351299_usize & 7_usize;
_21 = ['\u{eacf5}','\u{50431}','\u{18708}','\u{223ce}','\u{98611}','\u{69120}'];
_20.1.2 = !_14.1;
_6 = _12;
RET = -9223372036854775807_isize;
_11 = [(-24132_i16),5828_i16];
_9 = core::ptr::addr_of_mut!((*_9));
_23 = core::ptr::addr_of!(_20.2);
_18.0 = RET as f64;
_13 = [_8,_3,_25,_8];
(*_9) = 0_usize >> _14.0;
_9 = core::ptr::addr_of_mut!(_10);
_28 = [_4,_5,_8,_25];
_5 = _4;
_23 = core::ptr::addr_of!(_20.2);
_28 = _13;
(*_23) = 3184813253_u32;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(3_usize, 25_usize, Move(_25), 11_usize, Move(_11), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(3_usize, 7_usize, Move(_7), 21_usize, Move(_21), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: i32,mut _3: bool,mut _4: usize,mut _5: usize,mut _6: bool,mut _7: *mut usize,mut _8: bool,mut _9: *const i16,mut _10: [u64; 2],mut _11: bool) -> f32 {
mir! {
type RET = f32;
let _12: *mut f32;
let _13: u16;
let _14: [u128; 4];
let _15: i8;
let _16: isize;
let _17: (Adt17, i32, Adt17, usize);
let _18: &'static Adt18;
let _19: char;
let _20: Adt81;
let _21: (Adt18, (*const u128, u64, u64), u32);
let _22: (*const [u64; 2], u8, [bool; 5]);
let _23: i64;
let _24: *const *const i16;
let _25: i64;
let _26: u64;
let _27: i32;
let _28: f64;
let _29: (u16,);
let _30: *const &'static *mut *const i128;
let _31: isize;
let _32: *const i128;
let _33: Adt47;
let _34: u32;
let _35: f64;
let _36: *const i128;
let _37: ((u16,), [usize; 5], (i8, u64), [u128; 4]);
let _38: [i8; 3];
let _39: [bool; 4];
let _40: [char; 6];
let _41: ();
let _42: ();
{
_1 = _11 | _3;
_4 = _5 + _5;
RET = 1669761558232923857_i64 as f32;
_7 = core::ptr::addr_of_mut!(_4);
(*_7) = 9810617130401387210_u64 as usize;
_10 = [1462626112412024558_u64,5585291918551328132_u64];
(*_7) = _5 ^ _5;
_2 = 1068501669_i32;
_10 = [3492895459343991844_u64,4197222395486841023_u64];
_4 = _5 | _5;
_13 = !26574_u16;
_12 = core::ptr::addr_of_mut!(RET);
RET = _4 as f32;
RET = _2 as f32;
_8 = !_11;
_10 = [5509081616313464925_u64,3091997152888408935_u64];
_4 = !_5;
_12 = core::ptr::addr_of_mut!(RET);
(*_7) = RET as usize;
_2 = 1433628466_i32;
(*_7) = _2 as usize;
_3 = _1;
_6 = _8;
_2 = (-583653174_i32);
_14 = [64003177773643697929660195000337871999_u128,284939570648120159591875552455796871945_u128,226166132165120168961356625140664940124_u128,169411866976732601891092419303655530642_u128];
_13 = 32320_u16 + 2233_u16;
_5 = 221_u8 as usize;
Goto(bb1)
}
bb1 = {
_8 = _1;
(*_12) = 1771870718477327038_i64 as f32;
RET = 4182540692_u32 as f32;
(*_12) = (-3560935778267615686_i64) as f32;
(*_12) = (-9223372036854775808_isize) as f32;
RET = 72_i8 as f32;
_15 = 240088191182632876454799227351277324780_u128 as i8;
_11 = !_3;
_6 = _8;
_2 = 1360988063_i32;
RET = 256845428587639950243583604712294528575_u128 as f32;
_14 = [236026469394144817690436376360738463629_u128,161217147889408413703930347143504844747_u128,195364896408003991646701329221165150764_u128,216806472045647353548033634216223797037_u128];
_11 = !_6;
_2 = (-445421534_i32);
_16 = (-9223372036854775808_isize);
RET = _15 as f32;
_17.2 = Adt17::Variant0 { fld0: 3_u8,fld1: 4357839782793684970_u64 };
Goto(bb2)
}
bb2 = {
_3 = _8;
RET = _2 as f32;
_17.1 = 1776861078_u32 as i32;
_15 = (*_7) as i8;
place!(Field::<u8>(Variant(_17.2, 0), 0)) = 36_u8 << _13;
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = (-10345_i16) as f32;
_17.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_17.2, 0), 0),fld1: 8260998111423637407_u64 };
(*_12) = Field::<u8>(Variant(_17.0, 0), 0) as f32;
place!(Field::<u64>(Variant(_17.2, 0), 1)) = 17813373087909816565_u64;
_2 = _17.1;
_2 = _17.1;
RET = Field::<u64>(Variant(_17.2, 0), 1) as f32;
_19 = '\u{3b9da}';
place!(Field::<u64>(Variant(_17.2, 0), 1)) = !17811998310459264182_u64;
_8 = !_6;
_17.0 = Move(_17.2);
_14 = [324481888355408763509785597162868307341_u128,113463085452535357364519863971994075588_u128,60721412253359909221138449676423160645_u128,186793376474609146031214535518164365092_u128];
(*_7) = _5;
_17.3 = (*_7) ^ (*_7);
_21.0 = Adt18::Variant0 { fld0: (*_12),fld1: _19,fld2: _16,fld3: 2729750939_u32,fld4: 3094_i16,fld5: 204334810390070864998806775842390957490_u128,fld6: _4,fld7: Move(_17.0) };
_18 = &_21.0;
_8 = _3;
(*_12) = Field::<f32>(Variant((*_18), 0), 0);
match Field::<isize>(Variant((*_18), 0), 2) {
0 => bb3,
1 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb3 = {
_8 = _1;
(*_12) = 1771870718477327038_i64 as f32;
RET = 4182540692_u32 as f32;
(*_12) = (-3560935778267615686_i64) as f32;
(*_12) = (-9223372036854775808_isize) as f32;
RET = 72_i8 as f32;
_15 = 240088191182632876454799227351277324780_u128 as i8;
_11 = !_3;
_6 = _8;
_2 = 1360988063_i32;
RET = 256845428587639950243583604712294528575_u128 as f32;
_14 = [236026469394144817690436376360738463629_u128,161217147889408413703930347143504844747_u128,195364896408003991646701329221165150764_u128,216806472045647353548033634216223797037_u128];
_11 = !_6;
_2 = (-445421534_i32);
_16 = (-9223372036854775808_isize);
RET = _15 as f32;
_17.2 = Adt17::Variant0 { fld0: 3_u8,fld1: 4357839782793684970_u64 };
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
SetDiscriminant(Field::<Adt17>(Variant(_21.0, 0), 7), 0);
_17.0 = Adt17::Variant0 { fld0: 42_u8,fld1: 13406247891146100752_u64 };
_17.2 = Adt17::Variant0 { fld0: 75_u8,fld1: 8281260594760612285_u64 };
_5 = (*_7);
place!(Field::<f32>(Variant(_21.0, 0), 0)) = RET * (*_12);
Goto(bb7)
}
bb7 = {
place!(Field::<u64>(Variant(_17.2, 0), 1)) = (*_12) as u64;
_22.0 = core::ptr::addr_of!(_10);
place!(Field::<u64>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 0), 1)) = !Field::<u64>(Variant(_17.2, 0), 1);
place!(Field::<u128>(Variant(_21.0, 0), 5)) = 40315435967003632407131130315700888807_u128 + 35033798043826360609610715361010822305_u128;
_3 = _6;
place!(Field::<u8>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 0), 0)) = 8_u8;
(*_7) = Field::<usize>(Variant((*_18), 0), 6);
place!(Field::<u32>(Variant(_21.0, 0), 3)) = 1341552834_u32 - 2699618890_u32;
SetDiscriminant(Field::<Adt17>(Variant(_21.0, 0), 7), 1);
place!(Field::<u64>(Variant(_17.0, 0), 1)) = !Field::<u64>(Variant(_17.2, 0), 1);
place!(Field::<usize>(Variant(_21.0, 0), 6)) = _4;
place!(Field::<u8>(Variant(_17.2, 0), 0)) = !250_u8;
place!(Field::<i16>(Variant(_21.0, 0), 4)) = -251_i16;
_21.1.2 = Field::<u64>(Variant(_17.2, 0), 1) | Field::<u64>(Variant(_17.0, 0), 1);
place!(Field::<isize>(Variant(_21.0, 0), 2)) = -_16;
place!(Field::<char>(Variant(_21.0, 0), 1)) = _19;
_9 = core::ptr::addr_of!(place!(Field::<i16>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 4)));
_12 = core::ptr::addr_of_mut!(RET);
_24 = core::ptr::addr_of!(_9);
place!(Field::<i16>(Variant(_21.0, 0), 4)) = (-26534_i16) + 2207_i16;
Goto(bb8)
}
bb8 = {
_9 = core::ptr::addr_of!(place!(Field::<i16>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 4)));
place!(Field::<u64>(Variant(_17.2, 0), 1)) = !_21.1.2;
_12 = core::ptr::addr_of_mut!((*_12));
place!(Field::<u128>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 5)) = Field::<u32>(Variant(_21.0, 0), 3) as u128;
_21.1.2 = Field::<u128>(Variant(_21.0, 0), 5) as u64;
_21.1.2 = Field::<u64>(Variant(_17.2, 0), 1) << Field::<i16>(Variant(_21.0, 0), 4);
(*_9) = Field::<i16>(Variant(_21.0, 0), 4) ^ Field::<i16>(Variant(_21.0, 0), 4);
(*_9) = -Field::<i16>(Variant(_21.0, 0), 4);
_22.1 = Field::<u8>(Variant(_17.2, 0), 0) ^ Field::<u8>(Variant(_17.2, 0), 0);
place!(Field::<u8>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 1)) = _13 as u8;
_4 = Field::<isize>(Variant(_21.0, 0), 2) as usize;
_10 = [Field::<u64>(Variant(_17.2, 0), 1),_21.1.2];
(*_7) = _1 as usize;
_17.0 = Move(_17.2);
place!(Field::<i128>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 2)) = -(-102717321150394864520482643463463146346_i128);
_21.2 = _16 as u32;
_23 = _21.2 as i64;
Goto(bb9)
}
bb9 = {
_17.2 = Adt17::Variant0 { fld0: _22.1,fld1: _21.1.2 };
_21.1.1 = !_21.1.2;
place!(Field::<i16>(Variant(_21.0, 0), 4)) = (*_9) >> _21.2;
place!(Field::<char>(Variant(_21.0, 0), 1)) = _19;
_17.1 = _2 ^ _2;
place!(Field::<u8>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 1)) = !_22.1;
(*_9) = _13 as i16;
(*_9) = !Field::<i16>(Variant(_21.0, 0), 4);
_19 = Field::<char>(Variant(_21.0, 0), 1);
_15 = (*_12) as i8;
_26 = Field::<u64>(Variant(_17.2, 0), 1) - _21.1.1;
_5 = (*_7);
place!(Field::<usize>(Variant(_21.0, 0), 6)) = (*_7) - (*_7);
_19 = Field::<char>(Variant(_21.0, 0), 1);
_21.1.2 = _26;
place!(Field::<u64>(Variant(_17.0, 0), 1)) = !_26;
place!(Field::<f64>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 3)) = _21.1.2 as f64;
place!(Field::<u8>(Variant(_17.0, 0), 0)) = Field::<u8>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 1) - Field::<u8>(Variant(_17.2, 0), 0);
_29 = (_13,);
_17.2 = Move(_17.0);
_28 = Field::<f64>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 3) + Field::<f64>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 3);
place!(Field::<u8>(Variant(_17.2, 0), 0)) = _22.1;
match _16 {
0 => bb8,
1 => bb7,
2 => bb5,
3 => bb6,
4 => bb10,
5 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
_8 = _1;
(*_12) = 1771870718477327038_i64 as f32;
RET = 4182540692_u32 as f32;
(*_12) = (-3560935778267615686_i64) as f32;
(*_12) = (-9223372036854775808_isize) as f32;
RET = 72_i8 as f32;
_15 = 240088191182632876454799227351277324780_u128 as i8;
_11 = !_3;
_6 = _8;
_2 = 1360988063_i32;
RET = 256845428587639950243583604712294528575_u128 as f32;
_14 = [236026469394144817690436376360738463629_u128,161217147889408413703930347143504844747_u128,195364896408003991646701329221165150764_u128,216806472045647353548033634216223797037_u128];
_11 = !_6;
_2 = (-445421534_i32);
_16 = (-9223372036854775808_isize);
RET = _15 as f32;
_17.2 = Adt17::Variant0 { fld0: 3_u8,fld1: 4357839782793684970_u64 };
Goto(bb2)
}
bb12 = {
SetDiscriminant(Field::<Adt17>(Variant(_21.0, 0), 7), 0);
_17.0 = Adt17::Variant0 { fld0: 42_u8,fld1: 13406247891146100752_u64 };
_17.2 = Adt17::Variant0 { fld0: 75_u8,fld1: 8281260594760612285_u64 };
_5 = (*_7);
place!(Field::<f32>(Variant(_21.0, 0), 0)) = RET * (*_12);
Goto(bb7)
}
bb13 = {
_10 = [_26,_21.1.1];
_10 = [_26,_26];
place!(Field::<u8>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 1)) = _22.1;
_25 = _17.1 as i64;
_18 = &_21.0;
(*_7) = Field::<usize>(Variant((*_18), 0), 6) ^ _5;
place!(Field::<i128>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 2)) = (-124923221410823363722866087438696346767_i128);
_25 = _23;
SetDiscriminant(_17.2, 0);
_17.3 = Field::<usize>(Variant(_21.0, 0), 6);
_4 = _17.3;
_27 = -_17.1;
place!(Field::<u128>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 5)) = !Field::<u128>(Variant((*_18), 0), 5);
_22.1 = !Field::<u8>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 1);
place!(Field::<i16>(Variant(_21.0, 0), 4)) = Field::<i16>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 4) & (*_9);
place!(Field::<u64>(Variant(_17.2, 0), 1)) = !_26;
_21.1.0 = core::ptr::addr_of!(place!(Field::<u128>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 5)));
place!(Field::<u8>(Variant(_17.2, 0), 0)) = _16 as u8;
_2 = _27 | _27;
_33 = Adt47::Variant0 { fld0: Move(_17.2) };
_10 = [Field::<u64>(Variant(Field::<Adt17>(Variant(_33, 0), 0), 0), 1),_26];
_26 = _21.1.2 * Field::<u64>(Variant(Field::<Adt17>(Variant(_33, 0), 0), 0), 1);
Call(_17.3 = fn5(_4, Move(_22.0), Field::<u64>(Variant(Field::<Adt17>(Variant(_33, 0), 0), 0), 1), Field::<u128>(Variant((*_18), 0), 5), Move(_33), _26, (*_9), (*_7), Field::<f32>(Variant((*_18), 0), 0), _4), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_35 = Field::<f64>(Variant(Field::<Adt17>(Variant((*_18), 0), 7), 1), 3) * Field::<f64>(Variant(Field::<Adt17>(Variant((*_18), 0), 7), 1), 3);
place!(Field::<char>(Variant(_21.0, 0), 1)) = _19;
_22.2 = [_8,_8,_3,_3,_3];
place!(Field::<i16>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 4)) = Field::<char>(Variant(_21.0, 0), 1) as i16;
_25 = _23;
place!(Field::<f64>(Variant(place!(Field::<Adt17>(Variant(_21.0, 0), 7)), 1), 3)) = _16 as f64;
place!(Field::<u128>(Variant(_21.0, 0), 5)) = Field::<u128>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 5) | Field::<u128>(Variant(Field::<Adt17>(Variant(_21.0, 0), 7), 1), 5);
place!(Field::<Adt17>(Variant(_21.0, 0), 7)) = Adt17::Variant0 { fld0: _22.1,fld1: _26 };
_27 = _2;
_39 = [_8,_1,_3,_11];
_17.0 = Adt17::Variant0 { fld0: _22.1,fld1: _21.1.1 };
_2 = Field::<u32>(Variant(_21.0, 0), 3) as i32;
_21.2 = Field::<u32>(Variant((*_18), 0), 3) | Field::<u32>(Variant(_21.0, 0), 3);
_13 = !_29.0;
_21.2 = Field::<u32>(Variant((*_18), 0), 3) - Field::<u32>(Variant((*_18), 0), 3);
place!(Field::<f32>(Variant(_21.0, 0), 0)) = RET;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(4_usize, 39_usize, Move(_39), 26_usize, Move(_26), 10_usize, Move(_10), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(4_usize, 5_usize, Move(_5), 19_usize, Move(_19), 25_usize, Move(_25), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(4_usize, 3_usize, Move(_3), 4_usize, Move(_4), 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: usize,mut _2: *const [u64; 2],mut _3: u64,mut _4: u128,mut _5: Adt47,mut _6: u64,mut _7: i16,mut _8: usize,mut _9: f32,mut _10: usize) -> usize {
mir! {
type RET = usize;
let _11: i64;
let _12: [isize; 4];
let _13: ();
let _14: ();
{
SetDiscriminant(Field::<Adt17>(Variant(_5, 0), 0), 1);
place!(Field::<u128>(Variant(place!(Field::<Adt17>(Variant(_5, 0), 0)), 1), 5)) = _4;
place!(Field::<i64>(Variant(place!(Field::<Adt17>(Variant(_5, 0), 0)), 1), 0)) = (-6072619306983806933_i64);
_3 = 112_u8 as u64;
_7 = _9 as i16;
_11 = _7 as i64;
_11 = Field::<i64>(Variant(Field::<Adt17>(Variant(_5, 0), 0), 1), 0) + Field::<i64>(Variant(Field::<Adt17>(Variant(_5, 0), 0), 1), 0);
_10 = 210_u8 as usize;
RET = _1;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(5_usize, 7_usize, Move(_7), 6_usize, Move(_6), 3_usize, Move(_3), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: f32,mut _2: (i8, u64),mut _3: [i16; 2],mut _4: usize,mut _5: bool,mut _6: i8,mut _7: bool,mut _8: bool) -> f32 {
mir! {
type RET = f32;
let _9: *const *const i16;
let _10: [u128; 7];
let _11: Adt81;
let _12: [u128; 4];
let _13: (Adt17, i32, Adt17, usize);
let _14: usize;
let _15: *const *const i16;
let _16: Adt47;
let _17: (*const u128, u64, u64);
let _18: u16;
let _19: [i8; 8];
let _20: Adt44;
let _21: &'static *mut *const i128;
let _22: *mut Adt47;
let _23: (Adt18, *const i16);
let _24: ();
let _25: ();
{
RET = _1 * _1;
_3 = [(-26421_i16),(-11620_i16)];
RET = _1;
_6 = (-2031469439_i32) as i8;
RET = -_1;
_4 = 917330592855168735_usize;
RET = _4 as f32;
_2.0 = _6;
RET = _1 + _1;
_2.0 = (-9223372036854775808_isize) as i8;
_8 = _5 & _5;
RET = _1;
_2 = (_6, 8703401052355652848_u64);
_2 = (_6, 15319840296963938158_u64);
Goto(bb1)
}
bb1 = {
_5 = _8;
_3 = [(-1113_i16),6372_i16];
_10 = [267332007364568885386445249812627690878_u128,167209519270359032653628365976616436358_u128,257569607382568962559492002689907722688_u128,252865598824468829806303881634609339476_u128,299802202401640714351264614828046234244_u128,233238359132894574995825486889645667131_u128,132573665458626614988154509315106245776_u128];
_2.1 = 17942552074826330079_u64;
_2 = (_6, 1204558709480125504_u64);
_2.0 = _6;
_5 = _7;
_2.0 = 4780023823138112157_i64 as i8;
_13.1 = (-1317322157_i32) & (-1569902259_i32);
_12 = [68212091023319374952031978284737897625_u128,249796883718692338197863646736727441930_u128,273982294278224148085481218385446478656_u128,131740666464412604017126687536907393814_u128];
_8 = _7 <= _7;
Goto(bb2)
}
bb2 = {
_3 = [13316_i16,6654_i16];
_12 = [40296193887735008785410772949750014306_u128,145222758320661057471200623990356735466_u128,15071838682444181013939628418702572914_u128,143842496588129329824603858573934135494_u128];
_5 = _8;
_13.0 = Adt17::Variant0 { fld0: 79_u8,fld1: _2.1 };
_12 = [115227452772342941325088765327759104990_u128,8725097871881098315378729172370905295_u128,301860392357604106491792645168162424705_u128,312898319648052001714356727713739722724_u128];
RET = _1;
_2.1 = Field::<u64>(Variant(_13.0, 0), 1) + Field::<u64>(Variant(_13.0, 0), 1);
_13.3 = 68421047776113440596755954212010554725_i128 as usize;
_13.2 = Adt17::Variant0 { fld0: 81_u8,fld1: _2.1 };
_13.1 = '\u{7f1de}' as i32;
_2.1 = _13.1 as u64;
place!(Field::<u8>(Variant(_13.0, 0), 0)) = _13.1 as u8;
_8 = !_5;
_2.1 = (-88980070527029658753038207388947146612_i128) as u64;
_7 = _8 | _8;
_17.2 = Field::<u64>(Variant(_13.2, 0), 1) - Field::<u64>(Variant(_13.2, 0), 1);
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
917330592855168735 => bb9,
_ => bb8
}
}
bb3 = {
_5 = _8;
_3 = [(-1113_i16),6372_i16];
_10 = [267332007364568885386445249812627690878_u128,167209519270359032653628365976616436358_u128,257569607382568962559492002689907722688_u128,252865598824468829806303881634609339476_u128,299802202401640714351264614828046234244_u128,233238359132894574995825486889645667131_u128,132573665458626614988154509315106245776_u128];
_2.1 = 17942552074826330079_u64;
_2 = (_6, 1204558709480125504_u64);
_2.0 = _6;
_5 = _7;
_2.0 = 4780023823138112157_i64 as i8;
_13.1 = (-1317322157_i32) & (-1569902259_i32);
_12 = [68212091023319374952031978284737897625_u128,249796883718692338197863646736727441930_u128,273982294278224148085481218385446478656_u128,131740666464412604017126687536907393814_u128];
_8 = _7 <= _7;
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
_10 = [321854636927013619585087576106134496259_u128,26691609894418939048624247285290809371_u128,1389230534840104405539135334485304551_u128,193863486604301893660597822102870810109_u128,258283894186989475344213483591343059739_u128,129742062598443752555864108647356947508_u128,2218784808486096441813507712445256260_u128];
match Field::<u64>(Variant(_13.0, 0), 1) {
0 => bb7,
1 => bb10,
2 => bb11,
3 => bb12,
1204558709480125504 => bb14,
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
_3 = [13316_i16,6654_i16];
_12 = [40296193887735008785410772949750014306_u128,145222758320661057471200623990356735466_u128,15071838682444181013939628418702572914_u128,143842496588129329824603858573934135494_u128];
_5 = _8;
_13.0 = Adt17::Variant0 { fld0: 79_u8,fld1: _2.1 };
_12 = [115227452772342941325088765327759104990_u128,8725097871881098315378729172370905295_u128,301860392357604106491792645168162424705_u128,312898319648052001714356727713739722724_u128];
RET = _1;
_2.1 = Field::<u64>(Variant(_13.0, 0), 1) + Field::<u64>(Variant(_13.0, 0), 1);
_13.3 = 68421047776113440596755954212010554725_i128 as usize;
_13.2 = Adt17::Variant0 { fld0: 81_u8,fld1: _2.1 };
_13.1 = '\u{7f1de}' as i32;
_2.1 = _13.1 as u64;
place!(Field::<u8>(Variant(_13.0, 0), 0)) = _13.1 as u8;
_8 = !_5;
_2.1 = (-88980070527029658753038207388947146612_i128) as u64;
_7 = _8 | _8;
_17.2 = Field::<u64>(Variant(_13.2, 0), 1) - Field::<u64>(Variant(_13.2, 0), 1);
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
917330592855168735 => bb9,
_ => bb8
}
}
bb13 = {
_5 = _8;
_3 = [(-1113_i16),6372_i16];
_10 = [267332007364568885386445249812627690878_u128,167209519270359032653628365976616436358_u128,257569607382568962559492002689907722688_u128,252865598824468829806303881634609339476_u128,299802202401640714351264614828046234244_u128,233238359132894574995825486889645667131_u128,132573665458626614988154509315106245776_u128];
_2.1 = 17942552074826330079_u64;
_2 = (_6, 1204558709480125504_u64);
_2.0 = _6;
_5 = _7;
_2.0 = 4780023823138112157_i64 as i8;
_13.1 = (-1317322157_i32) & (-1569902259_i32);
_12 = [68212091023319374952031978284737897625_u128,249796883718692338197863646736727441930_u128,273982294278224148085481218385446478656_u128,131740666464412604017126687536907393814_u128];
_8 = _7 <= _7;
Goto(bb2)
}
bb14 = {
_8 = _5;
RET = -_1;
_7 = !_8;
_17.1 = (-58415072541963417581487573893411266323_i128) as u64;
place!(Field::<u8>(Variant(_13.2, 0), 0)) = !Field::<u8>(Variant(_13.0, 0), 0);
_2.0 = -_6;
_10 = [118105313439923519662168830986525487287_u128,41659540528538212817846323396546105106_u128,161541676893098798614927239543570821781_u128,52735893232181180825545495278032360531_u128,50420953759913643554091933179209050864_u128,235616909500421856475067174546318139829_u128,7342417231993193000205151464106636813_u128];
SetDiscriminant(_13.0, 1);
place!(Field::<u8>(Variant(_13.0, 1), 1)) = Field::<u8>(Variant(_13.2, 0), 0);
_2.1 = !_17.2;
_17.0 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_13.0, 1), 5)));
_13.3 = !_4;
_10 = [53409432370666156615566953119227143061_u128,172139312010994341035811507569204263919_u128,266256570662755493942809247311278705588_u128,259372457481876757752108276979073189317_u128,244802876122015662430173865382749965516_u128,48492424652730633030345245820210779367_u128,67694594720045272541038659987537030044_u128];
_10 = [44292553867187350514334363103919856467_u128,291759846410998341764225822395962811282_u128,208269734897200480949659400812526100911_u128,73446365795422009441993618726682392039_u128,51467059072995973602578391369907253494_u128,279542966008128920070997135877096268167_u128,287860768210479484896583893672488995720_u128];
place!(Field::<i16>(Variant(_13.0, 1), 4)) = -(-11882_i16);
_2 = (_6, Field::<u64>(Variant(_13.2, 0), 1));
SetDiscriminant(_13.2, 0);
place!(Field::<u128>(Variant(_13.0, 1), 5)) = !232844765662546381777431649453796126266_u128;
_15 = core::ptr::addr_of!(_20.fld2);
_13.1 = (-2073712404_i32) + 552520581_i32;
_20.fld3 = Field::<i16>(Variant(_13.0, 1), 4) as f32;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(6_usize, 6_usize, Move(_6), 5_usize, Move(_5), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *const i16,mut _2: (i8, u64),mut _3: u128,mut _4: u128,mut _5: u128,mut _6: *const u128,mut _7: bool,mut _8: u128) -> f64 {
mir! {
type RET = f64;
let _9: isize;
let _10: bool;
let _11: Adt17;
let _12: f64;
let _13: i16;
let _14: bool;
let _15: Adt81;
let _16: [u64; 2];
let _17: f32;
let _18: (f64,);
let _19: &'static Adt18;
let _20: (&'static (*const [u64; 2], u8, [bool; 5]),);
let _21: *const &'static *mut *const i128;
let _22: *const i16;
let _23: bool;
let _24: u32;
let _25: isize;
let _26: ();
let _27: ();
{
_2.0 = _5 as i8;
_8 = 4781026465467059722_i64 as u128;
RET = _2.0 as f64;
_2 = (123_i8, 1943503107688567808_u64);
_7 = !false;
_2.0 = -14_i8;
_5 = _4;
RET = _2.0 as f64;
_8 = _4;
_3 = (-81_isize) as u128;
RET = 44228_u16 as f64;
_2.1 = 2142322612_u32 as u64;
_3 = !_8;
_3 = !_5;
_3 = _8;
_2 = (19_i8, 3598366481142427216_u64);
_2 = ((-20_i8), 8288015549709734541_u64);
Call(_9 = fn8(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 1568299479_u32 as u128;
_4 = _3 + _3;
RET = 145_u8 as f64;
RET = _2.0 as f64;
_11 = Adt17::Variant1 { fld0: (-5351028459976185646_i64),fld1: 5_u8,fld2: (-81442796246576058718075854560471201989_i128),fld3: RET,fld4: (-1298_i16),fld5: _4 };
place!(Field::<i16>(Variant(_11, 1), 4)) = (-7593_i16) << _3;
_8 = Field::<u128>(Variant(_11, 1), 5);
_2.0 = (-84_i8) * (-72_i8);
_11 = Adt17::Variant0 { fld0: 164_u8,fld1: _2.1 };
place!(Field::<u8>(Variant(_11, 0), 0)) = RET as u8;
_12 = _4 as f64;
_13 = !22656_i16;
_1 = core::ptr::addr_of!(_13);
_9 = (-74_isize);
place!(Field::<u8>(Variant(_11, 0), 0)) = 79_u8 ^ 60_u8;
_3 = _4;
_2 = ((-68_i8), Field::<u64>(Variant(_11, 0), 1));
_7 = false;
place!(Field::<u8>(Variant(_11, 0), 0)) = 3977744567261478613_i64 as u8;
_4 = _8 - _5;
_5 = !_4;
place!(Field::<u8>(Variant(_11, 0), 0)) = 352096714_u32 as u8;
_11 = Adt17::Variant1 { fld0: 8342201336552604477_i64,fld1: 5_u8,fld2: (-168309950240920147476596256168586424518_i128),fld3: _12,fld4: _13,fld5: _3 };
match _9 {
340282366920938463463374607431768211382 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
RET = 87_u8 as f64;
_10 = !_7;
place!(Field::<u128>(Variant(_11, 1), 5)) = _4;
_3 = _8 * _5;
place!(Field::<i64>(Variant(_11, 1), 0)) = 4644659097103225872_i64;
_12 = Field::<f64>(Variant(_11, 1), 3) + Field::<f64>(Variant(_11, 1), 3);
(*_1) = Field::<i16>(Variant(_11, 1), 4) ^ Field::<i16>(Variant(_11, 1), 4);
place!(Field::<i128>(Variant(_11, 1), 2)) = !(-89496921243136788059083784592747698896_i128);
match _2.1 {
0 => bb1,
8288015549709734541 => bb4,
_ => bb2
}
}
bb4 = {
place!(Field::<u128>(Variant(_11, 1), 5)) = _12 as u128;
_11 = Adt17::Variant1 { fld0: 4623891312969459172_i64,fld1: 156_u8,fld2: (-1444436644643287351781192025565366368_i128),fld3: _12,fld4: (*_1),fld5: _8 };
_4 = _3 << (*_1);
_14 = _2.1 >= _2.1;
place!(Field::<u8>(Variant(_11, 1), 1)) = 97_u8;
place!(Field::<u8>(Variant(_11, 1), 1)) = '\u{8dfc7}' as u8;
(*_1) = Field::<i16>(Variant(_11, 1), 4) | Field::<i16>(Variant(_11, 1), 4);
_11 = Adt17::Variant0 { fld0: 249_u8,fld1: _2.1 };
_5 = _8 + _4;
_13 = (-10121_i16);
(*_1) = 174_u8 as i16;
_3 = _5 - _4;
place!(Field::<u64>(Variant(_11, 0), 1)) = _2.1;
_18 = (_12,);
_7 = _14;
_14 = !_10;
_9 = (-9223372036854775808_isize) << _3;
_11 = Adt17::Variant1 { fld0: 2195390983586635921_i64,fld1: 205_u8,fld2: 135411150422192196016395412503483443400_i128,fld3: _12,fld4: _13,fld5: _3 };
_16 = [_2.1,_2.1];
_10 = _12 < _12;
Call(_9 = core::intrinsics::transmute(_2.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_1) = !Field::<i16>(Variant(_11, 1), 4);
place!(Field::<i64>(Variant(_11, 1), 0)) = 54090_u16 as i64;
_11 = Adt17::Variant1 { fld0: (-924164100606326889_i64),fld1: 144_u8,fld2: (-115431753808567660727466434601864902256_i128),fld3: _12,fld4: _13,fld5: _3 };
_5 = Field::<u128>(Variant(_11, 1), 5) - Field::<u128>(Variant(_11, 1), 5);
place!(Field::<i128>(Variant(_11, 1), 2)) = _2.1 as i128;
(*_1) = Field::<i128>(Variant(_11, 1), 2) as i16;
place!(Field::<i64>(Variant(_11, 1), 0)) = 2132513279747130525_i64 * 5323003540900543059_i64;
place!(Field::<i64>(Variant(_11, 1), 0)) = !(-1128076804663890310_i64);
place!(Field::<u8>(Variant(_11, 1), 1)) = !142_u8;
_5 = !_3;
_5 = _3;
_6 = core::ptr::addr_of!(_8);
_10 = _3 > _3;
place!(Field::<i16>(Variant(_11, 1), 4)) = (*_1);
_4 = _3 - _3;
place!(Field::<i64>(Variant(_11, 1), 0)) = (-7741277716246035227_i64) << Field::<u128>(Variant(_11, 1), 5);
_17 = Field::<u8>(Variant(_11, 1), 1) as f32;
_18.0 = Field::<f64>(Variant(_11, 1), 3) + Field::<f64>(Variant(_11, 1), 3);
_10 = _7 | _7;
_2 = (35_i8, 2438616146703417942_u64);
_1 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_11, 1), 4)));
RET = -_18.0;
_25 = 2973250437_u32 as isize;
Goto(bb6)
}
bb6 = {
Call(_26 = dump_var(7_usize, 25_usize, Move(_25), 2_usize, Move(_2), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_26 = dump_var(7_usize, 13_usize, Move(_13), 5_usize, Move(_5), 27_usize, _27, 27_usize, _27), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8() -> isize {
mir! {
type RET = isize;
let _1: bool;
let _2: [char; 6];
let _3: *mut Adt18;
let _4: f32;
let _5: *const u128;
let _6: *mut *const i128;
let _7: *mut ([u64; 2],);
let _8: f64;
let _9: u8;
let _10: f32;
let _11: f32;
let _12: isize;
let _13: f32;
let _14: i16;
let _15: (Adt18, (*const u128, u64, u64), u32);
let _16: i16;
let _17: char;
let _18: [u128; 4];
let _19: (*const u128, usize, &'static i64, &'static *mut *const i128);
let _20: usize;
let _21: f32;
let _22: *mut *mut i16;
let _23: u32;
let _24: char;
let _25: f64;
let _26: i8;
let _27: (Adt18, (*const u128, u64, u64), u32);
let _28: [i16; 2];
let _29: [u64; 8];
let _30: &'static i64;
let _31: ();
let _32: ();
{
RET = 9223372036854775807_isize + (-9223372036854775808_isize);
RET = !(-34_isize);
_1 = RET > RET;
RET = !40_isize;
RET = (-28655_i16) as isize;
RET = !9223372036854775807_isize;
RET = (-121_isize);
RET = -(-1_isize);
_1 = false & false;
_1 = true;
Goto(bb1)
}
bb1 = {
RET = -(-9223372036854775808_isize);
_1 = true;
_1 = false;
RET = 9223372036854775807_isize | (-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_1 = RET >= RET;
RET = (-46_isize);
RET = 15_isize;
_2 = ['\u{6952e}','\u{d8cee}','\u{6356e}','\u{4c5d5}','\u{324f1}','\u{dbdd3}'];
_1 = true | true;
RET = (-81_isize);
_1 = RET < RET;
_1 = !true;
_2 = ['\u{15fb3}','\u{ac3a5}','\u{85a98}','\u{7f544}','\u{c1399}','\u{45c89}'];
RET = 16_isize;
_1 = false;
_1 = !true;
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
_2 = ['\u{95196}','\u{a7a8d}','\u{b3177}','\u{22a29}','\u{85dc5}','\u{a2772}'];
_1 = false | false;
RET = (-9223372036854775808_isize) & 9223372036854775807_isize;
_2 = ['\u{118d4}','\u{fd4a7}','\u{107d64}','\u{18ca2}','\u{b4297}','\u{77182}'];
Goto(bb2)
}
bb2 = {
RET = (-9223372036854775808_isize);
_4 = 21720_i16 as f32;
RET = 9223372036854775807_isize * 15_isize;
_4 = 1507220600_u32 as f32;
RET = !(-9223372036854775808_isize);
_4 = (-112_i8) as f32;
_1 = !true;
RET = (-9223372036854775808_isize);
RET = 45_isize;
_4 = 877959284_i32 as f32;
match RET {
0 => bb3,
1 => bb4,
45 => bb6,
_ => bb5
}
}
bb3 = {
RET = -(-9223372036854775808_isize);
_1 = true;
_1 = false;
RET = 9223372036854775807_isize | (-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_1 = RET >= RET;
RET = (-46_isize);
RET = 15_isize;
_2 = ['\u{6952e}','\u{d8cee}','\u{6356e}','\u{4c5d5}','\u{324f1}','\u{dbdd3}'];
_1 = true | true;
RET = (-81_isize);
_1 = RET < RET;
_1 = !true;
_2 = ['\u{15fb3}','\u{ac3a5}','\u{85a98}','\u{7f544}','\u{c1399}','\u{45c89}'];
RET = 16_isize;
_1 = false;
_1 = !true;
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
_2 = ['\u{95196}','\u{a7a8d}','\u{b3177}','\u{22a29}','\u{85dc5}','\u{a2772}'];
_1 = false | false;
RET = (-9223372036854775808_isize) & 9223372036854775807_isize;
_2 = ['\u{118d4}','\u{fd4a7}','\u{107d64}','\u{18ca2}','\u{b4297}','\u{77182}'];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_4 = (-102_i8) as f32;
RET = 9223372036854775807_isize << 28366_i16;
_1 = !true;
_1 = RET >= RET;
_2 = ['\u{61cbb}','\u{1e17b}','\u{d6bd3}','\u{a9b49}','\u{11097}','\u{7f0a8}'];
_1 = false;
_4 = 3_usize as f32;
_4 = 7324874850399104737_u64 as f32;
RET = -9223372036854775807_isize;
_2 = ['\u{59a}','\u{aa9eb}','\u{7f78d}','\u{2fe21}','\u{3cf93}','\u{10876e}'];
RET = 91_u8 as isize;
_1 = !false;
_2 = ['\u{f0203}','\u{32358}','\u{eab4e}','\u{7fdbc}','\u{10c88d}','\u{56324}'];
RET = 9223372036854775807_isize << 11_i8;
_2 = ['\u{99d89}','\u{6acbd}','\u{abba9}','\u{32766}','\u{45ad9}','\u{f4ddc}'];
RET = !6_isize;
RET = (-7786223183825514463_i64) as isize;
_2 = ['\u{bb5c0}','\u{92289}','\u{3fed0}','\u{df69a}','\u{2a952}','\u{109f37}'];
_8 = 6515719216125602437_i64 as f64;
_4 = (-1677927524_i32) as f32;
_8 = 60229718515501793336552297071185520990_i128 as f64;
RET = 9223372036854775807_isize;
_8 = (-3687804982765863352_i64) as f64;
_8 = (-7807803247904911501_i64) as f64;
Call(_10 = fn9(_2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = _4;
_10 = 116919346368046543465991093719600623732_i128 as f32;
_1 = _4 != _4;
RET = 74_isize;
_10 = _4;
_12 = -RET;
_1 = false;
_9 = 81_u8 - 29_u8;
_11 = _4 * _4;
_11 = _4 - _10;
match RET {
0 => bb3,
1 => bb4,
74 => bb9,
_ => bb8
}
}
bb8 = {
RET = (-9223372036854775808_isize);
_4 = 21720_i16 as f32;
RET = 9223372036854775807_isize * 15_isize;
_4 = 1507220600_u32 as f32;
RET = !(-9223372036854775808_isize);
_4 = (-112_i8) as f32;
_1 = !true;
RET = (-9223372036854775808_isize);
RET = 45_isize;
_4 = 877959284_i32 as f32;
match RET {
0 => bb3,
1 => bb4,
45 => bb6,
_ => bb5
}
}
bb9 = {
_9 = 4_u8 | 104_u8;
RET = -_12;
_11 = _10 - _4;
RET = _12 >> _9;
RET = _9 as isize;
_3 = core::ptr::addr_of_mut!(_15.0);
_9 = 243_u8;
_13 = 6664622180172065366_usize as f32;
_15.1.1 = !12386613750690886024_u64;
_8 = _15.1.1 as f64;
_3 = core::ptr::addr_of_mut!(_15.0);
_8 = 1453202658103810377_i64 as f64;
_15.2 = 638230400_u32;
_15.1.2 = !_15.1.1;
_4 = -_10;
_18 = [295153898518410276598777690411876324754_u128,246539016148417736606458001218901873921_u128,316248750384730361477847950506804571501_u128,217888854332540809283332726254807961819_u128];
_4 = _13 * _11;
_4 = _11 * _13;
_3 = core::ptr::addr_of_mut!((*_3));
_19.1 = 10431_u16 as usize;
RET = _12 - _12;
_1 = !false;
match _15.2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
638230400 => bb11,
_ => bb10
}
}
bb10 = {
_4 = (-102_i8) as f32;
RET = 9223372036854775807_isize << 28366_i16;
_1 = !true;
_1 = RET >= RET;
_2 = ['\u{61cbb}','\u{1e17b}','\u{d6bd3}','\u{a9b49}','\u{11097}','\u{7f0a8}'];
_1 = false;
_4 = 3_usize as f32;
_4 = 7324874850399104737_u64 as f32;
RET = -9223372036854775807_isize;
_2 = ['\u{59a}','\u{aa9eb}','\u{7f78d}','\u{2fe21}','\u{3cf93}','\u{10876e}'];
RET = 91_u8 as isize;
_1 = !false;
_2 = ['\u{f0203}','\u{32358}','\u{eab4e}','\u{7fdbc}','\u{10c88d}','\u{56324}'];
RET = 9223372036854775807_isize << 11_i8;
_2 = ['\u{99d89}','\u{6acbd}','\u{abba9}','\u{32766}','\u{45ad9}','\u{f4ddc}'];
RET = !6_isize;
RET = (-7786223183825514463_i64) as isize;
_2 = ['\u{bb5c0}','\u{92289}','\u{3fed0}','\u{df69a}','\u{2a952}','\u{109f37}'];
_8 = 6515719216125602437_i64 as f64;
_4 = (-1677927524_i32) as f32;
_8 = 60229718515501793336552297071185520990_i128 as f64;
RET = 9223372036854775807_isize;
_8 = (-3687804982765863352_i64) as f64;
_8 = (-7807803247904911501_i64) as f64;
Call(_10 = fn9(_2), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_8 = _9 as f64;
RET = _12;
_10 = 126834033742257730506844660948878187232_i128 as f32;
_4 = _15.1.2 as f32;
_10 = _12 as f32;
_16 = _19.1 as i16;
_18 = [276305226044056877893970528447126853611_u128,194535407585564063703526651323029640692_u128,13621388728416761039440242886146999632_u128,32620508990142146802146325843619676371_u128];
_15.2 = 201066610_u32 & 3650673470_u32;
_14 = _16;
_15.1.1 = _15.1.2 * _15.1.2;
_4 = 103497908423996918295091081372396807671_u128 as f32;
Goto(bb12)
}
bb12 = {
_19.3 = &_6;
_11 = _13;
_15.1.2 = _15.1.1;
_17 = '\u{10e72}';
_20 = 24_i8 as usize;
_14 = !_16;
_14 = !_16;
RET = -_12;
_20 = !_19.1;
_11 = _4 * _10;
_3 = core::ptr::addr_of_mut!((*_3));
match _9 {
0 => bb1,
1 => bb10,
243 => bb13,
_ => bb4
}
}
bb13 = {
_20 = !_19.1;
_15.2 = 31830294167652871369663273327121043802_i128 as u32;
_24 = _17;
_14 = 32376_u16 as i16;
_23 = _15.2;
_21 = _13 * _13;
_11 = _10 - _13;
_2 = [_17,_24,_17,_17,_24,_24];
_20 = _19.1;
_21 = _11 + _11;
_27.2 = 26_i8 as u32;
_19.3 = &_6;
_4 = _21;
RET = _16 as isize;
_27.1.1 = _15.1.2 * _15.1.1;
match _9 {
0 => bb7,
1 => bb4,
2 => bb14,
3 => bb15,
243 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
_10 = _4;
_10 = 116919346368046543465991093719600623732_i128 as f32;
_1 = _4 != _4;
RET = 74_isize;
_10 = _4;
_12 = -RET;
_1 = false;
_9 = 81_u8 - 29_u8;
_11 = _4 * _4;
_11 = _4 - _10;
match RET {
0 => bb3,
1 => bb4,
74 => bb9,
_ => bb8
}
}
bb16 = {
RET = -(-9223372036854775808_isize);
_1 = true;
_1 = false;
RET = 9223372036854775807_isize | (-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_1 = RET >= RET;
RET = (-46_isize);
RET = 15_isize;
_2 = ['\u{6952e}','\u{d8cee}','\u{6356e}','\u{4c5d5}','\u{324f1}','\u{dbdd3}'];
_1 = true | true;
RET = (-81_isize);
_1 = RET < RET;
_1 = !true;
_2 = ['\u{15fb3}','\u{ac3a5}','\u{85a98}','\u{7f544}','\u{c1399}','\u{45c89}'];
RET = 16_isize;
_1 = false;
_1 = !true;
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
_2 = ['\u{95196}','\u{a7a8d}','\u{b3177}','\u{22a29}','\u{85dc5}','\u{a2772}'];
_1 = false | false;
RET = (-9223372036854775808_isize) & 9223372036854775807_isize;
_2 = ['\u{118d4}','\u{fd4a7}','\u{107d64}','\u{18ca2}','\u{b4297}','\u{77182}'];
Goto(bb2)
}
bb17 = {
_27.1.2 = !_27.1.1;
_17 = _24;
_13 = 14294_u16 as f32;
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(8_usize, 24_usize, Move(_24), 20_usize, Move(_20), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(8_usize, 16_usize, Move(_16), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [char; 6]) -> f32 {
mir! {
type RET = f32;
let _2: i128;
let _3: char;
let _4: (i16, u64, i8, (Adt17, i32, Adt17, usize));
let _5: *const *const i16;
let _6: f32;
let _7: &'static u64;
let _8: &'static u32;
let _9: f64;
let _10: Adt47;
let _11: (Adt18, *const i16);
let _12: i64;
let _13: u64;
let _14: isize;
let _15: isize;
let _16: [isize; 7];
let _17: i32;
let _18: ();
let _19: ();
{
RET = 759_i16 as f32;
RET = (-144447302729572305154351413969388949476_i128) as f32;
RET = 22_i8 as f32;
_1 = ['\u{cde8c}','\u{e4cee}','\u{8e8f4}','\u{6d1fd}','\u{109a19}','\u{a4387}'];
_1 = ['\u{7c073}','\u{289df}','\u{2973}','\u{f0423}','\u{10b8f1}','\u{ba781}'];
_2 = 68_i8 as i128;
RET = 4468946928052538733_i64 as f32;
RET = 17411205209855399681_u64 as f32;
_2 = -52638909547985989198432919890449716643_i128;
_3 = '\u{51a36}';
_1 = [_3,_3,_3,_3,_3,_3];
_3 = '\u{fc932}';
_2 = -153490515828168041747117492050734891806_i128;
RET = 11267037057522290502_u64 as f32;
_4.3.2 = Adt17::Variant0 { fld0: 42_u8,fld1: 10429045846133157464_u64 };
_4.3.1 = (-1118416683_i32);
_4.1 = 6874513975195750863_u64;
_3 = '\u{1db6f}';
Goto(bb1)
}
bb1 = {
_4.3.3 = !5_usize;
RET = _4.3.3 as f32;
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = !48_u8;
place!(Field::<u64>(Variant(_4.3.2, 0), 1)) = _4.1;
_4.0 = _2 as i16;
_4.3.1 = !490841021_i32;
_1 = [_3,_3,_3,_3,_3,_3];
_4.3.0 = Move(_4.3.2);
_3 = '\u{a3818}';
_4.2 = !(-81_i8);
_4.0 = (-24621_i16);
_4.0 = (-4906_i16) ^ 18561_i16;
_4.1 = Field::<u64>(Variant(_4.3.0, 0), 1) + Field::<u64>(Variant(_4.3.0, 0), 1);
RET = _4.1 as f32;
_4.3.2 = Move(_4.3.0);
RET = (-35_isize) as f32;
_3 = '\u{b860c}';
RET = _4.1 as f32;
_4.3.0 = Move(_4.3.2);
RET = 7778392111922570303_i64 as f32;
Call(_4.2 = core::intrinsics::transmute(Field::<u8>(Variant(_4.3.0, 0), 0)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4.0 = true as i16;
_4.3.3 = _4.0 as usize;
_4.0 = (-25644_i16) >> _4.1;
_4.3.1 = !(-619077785_i32);
_4.3.3 = 1_usize;
_4.3.2 = Move(_4.3.0);
_4.3.0 = Move(_4.3.2);
match Field::<u64>(Variant(_4.3.0, 0), 1) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6874513975195750863 => bb9,
_ => bb8
}
}
bb3 = {
_4.3.3 = !5_usize;
RET = _4.3.3 as f32;
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = !48_u8;
place!(Field::<u64>(Variant(_4.3.2, 0), 1)) = _4.1;
_4.0 = _2 as i16;
_4.3.1 = !490841021_i32;
_1 = [_3,_3,_3,_3,_3,_3];
_4.3.0 = Move(_4.3.2);
_3 = '\u{a3818}';
_4.2 = !(-81_i8);
_4.0 = (-24621_i16);
_4.0 = (-4906_i16) ^ 18561_i16;
_4.1 = Field::<u64>(Variant(_4.3.0, 0), 1) + Field::<u64>(Variant(_4.3.0, 0), 1);
RET = _4.1 as f32;
_4.3.2 = Move(_4.3.0);
RET = (-35_isize) as f32;
_3 = '\u{b860c}';
RET = _4.1 as f32;
_4.3.0 = Move(_4.3.2);
RET = 7778392111922570303_i64 as f32;
Call(_4.2 = core::intrinsics::transmute(Field::<u8>(Variant(_4.3.0, 0), 0)), ReturnTo(bb2), UnwindUnreachable())
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
RET = _4.0 as f32;
RET = _4.2 as f32;
_4.3.2 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_4.3.0, 0), 0),fld1: _4.1 };
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = !Field::<u8>(Variant(_4.3.0, 0), 0);
_6 = RET;
RET = -_6;
SetDiscriminant(_4.3.2, 0);
RET = -_6;
RET = 243218800126388581050308512497873776440_u128 as f32;
RET = _6 + _6;
_4.3.0 = Adt17::Variant0 { fld0: 199_u8,fld1: _4.1 };
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = 181_u8 << _4.3.1;
_4.0 = _4.1 as i16;
_6 = RET * RET;
_3 = '\u{3889f}';
_7 = &place!(Field::<u64>(Variant(_4.3.2, 0), 1));
_4.2 = 109_i8;
place!(Field::<u64>(Variant(_4.3.0, 0), 1)) = _4.1 << _4.1;
_4.3.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_4.3.2, 0), 0),fld1: _4.1 };
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = Field::<u8>(Variant(_4.3.0, 0), 0) * Field::<u8>(Variant(_4.3.0, 0), 0);
_2 = -116841836770843640832131735191119889814_i128;
_14 = !93_isize;
_9 = 42859121116637819899544483928873758005_u128 as f64;
SetDiscriminant(_4.3.0, 0);
_4.3.1 = -197319815_i32;
_3 = '\u{1af5e}';
_11.1 = core::ptr::addr_of!(_4.0);
Goto(bb10)
}
bb10 = {
_5 = core::ptr::addr_of!(_11.1);
_4.1 = _3 as u64;
_13 = true as u64;
_4.3.2 = Adt17::Variant1 { fld0: 8340316288468338140_i64,fld1: 126_u8,fld2: _2,fld3: _9,fld4: _4.0,fld5: 19280635635006643044628165034727817643_u128 };
_4.1 = !_13;
_5 = core::ptr::addr_of!((*_5));
_7 = &_13;
place!(Field::<u128>(Variant(_4.3.2, 1), 5)) = !146966560660228694673223863399057085885_u128;
_4.1 = !_13;
RET = _6;
place!(Field::<i16>(Variant(_4.3.2, 1), 4)) = _4.0;
place!(Field::<i64>(Variant(_4.3.2, 1), 0)) = 3486862006163307120_i64;
_4.1 = (*_7) >> _4.0;
place!(Field::<i64>(Variant(_4.3.2, 1), 0)) = 65215_u16 as i64;
place!(Field::<u8>(Variant(_4.3.0, 0), 0)) = !245_u8;
_16 = [_14,_14,_14,_14,_14,_14,_14];
_7 = &(*_7);
_1 = [_3,_3,_3,_3,_3,_3];
place!(Field::<u128>(Variant(_4.3.2, 1), 5)) = 170449554538759088134405458575866365889_u128;
_2 = Field::<u8>(Variant(_4.3.0, 0), 0) as i128;
place!(Field::<f64>(Variant(_4.3.2, 1), 3)) = -_9;
_4.3.2 = Adt17::Variant1 { fld0: 142580885381513203_i64,fld1: Field::<u8>(Variant(_4.3.0, 0), 0),fld2: _2,fld3: _9,fld4: _4.0,fld5: 262192446457682286213689789742337187756_u128 };
_4.3.2 = Adt17::Variant1 { fld0: 1816749028014132044_i64,fld1: Field::<u8>(Variant(_4.3.0, 0), 0),fld2: _2,fld3: _9,fld4: _4.0,fld5: 103295224954649604022440167496705221419_u128 };
place!(Field::<i64>(Variant(_4.3.2, 1), 0)) = 1096175475_u32 as i64;
_17 = true as i32;
_9 = Field::<f64>(Variant(_4.3.2, 1), 3) * Field::<f64>(Variant(_4.3.2, 1), 3);
place!(Field::<u128>(Variant(_4.3.2, 1), 5)) = 302536885183498407712913532777166902924_u128;
match Field::<u128>(Variant(_4.3.2, 1), 5) {
0 => bb2,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
302536885183498407712913532777166902924 => bb18,
_ => bb17
}
}
bb11 = {
RET = _4.0 as f32;
RET = _4.2 as f32;
_4.3.2 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_4.3.0, 0), 0),fld1: _4.1 };
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = !Field::<u8>(Variant(_4.3.0, 0), 0);
_6 = RET;
RET = -_6;
SetDiscriminant(_4.3.2, 0);
RET = -_6;
RET = 243218800126388581050308512497873776440_u128 as f32;
RET = _6 + _6;
_4.3.0 = Adt17::Variant0 { fld0: 199_u8,fld1: _4.1 };
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = 181_u8 << _4.3.1;
_4.0 = _4.1 as i16;
_6 = RET * RET;
_3 = '\u{3889f}';
_7 = &place!(Field::<u64>(Variant(_4.3.2, 0), 1));
_4.2 = 109_i8;
place!(Field::<u64>(Variant(_4.3.0, 0), 1)) = _4.1 << _4.1;
_4.3.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_4.3.2, 0), 0),fld1: _4.1 };
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = Field::<u8>(Variant(_4.3.0, 0), 0) * Field::<u8>(Variant(_4.3.0, 0), 0);
_2 = -116841836770843640832131735191119889814_i128;
_14 = !93_isize;
_9 = 42859121116637819899544483928873758005_u128 as f64;
SetDiscriminant(_4.3.0, 0);
_4.3.1 = -197319815_i32;
_3 = '\u{1af5e}';
_11.1 = core::ptr::addr_of!(_4.0);
Goto(bb10)
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
_4.0 = true as i16;
_4.3.3 = _4.0 as usize;
_4.0 = (-25644_i16) >> _4.1;
_4.3.1 = !(-619077785_i32);
_4.3.3 = 1_usize;
_4.3.2 = Move(_4.3.0);
_4.3.0 = Move(_4.3.2);
match Field::<u64>(Variant(_4.3.0, 0), 1) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6874513975195750863 => bb9,
_ => bb8
}
}
bb16 = {
Return()
}
bb17 = {
_4.3.3 = !5_usize;
RET = _4.3.3 as f32;
place!(Field::<u8>(Variant(_4.3.2, 0), 0)) = !48_u8;
place!(Field::<u64>(Variant(_4.3.2, 0), 1)) = _4.1;
_4.0 = _2 as i16;
_4.3.1 = !490841021_i32;
_1 = [_3,_3,_3,_3,_3,_3];
_4.3.0 = Move(_4.3.2);
_3 = '\u{a3818}';
_4.2 = !(-81_i8);
_4.0 = (-24621_i16);
_4.0 = (-4906_i16) ^ 18561_i16;
_4.1 = Field::<u64>(Variant(_4.3.0, 0), 1) + Field::<u64>(Variant(_4.3.0, 0), 1);
RET = _4.1 as f32;
_4.3.2 = Move(_4.3.0);
RET = (-35_isize) as f32;
_3 = '\u{b860c}';
RET = _4.1 as f32;
_4.3.0 = Move(_4.3.2);
RET = 7778392111922570303_i64 as f32;
Call(_4.2 = core::intrinsics::transmute(Field::<u8>(Variant(_4.3.0, 0), 0)), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_4.2 = _4.1 as i8;
place!(Field::<u8>(Variant(_4.3.2, 1), 1)) = !Field::<u8>(Variant(_4.3.0, 0), 0);
Goto(bb19)
}
bb19 = {
Call(_18 = dump_var(9_usize, 14_usize, Move(_14), 2_usize, Move(_2), 16_usize, Move(_16), 19_usize, _19), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const i16,mut _2: u64,mut _3: u16,mut _4: i32,mut _5: i32,mut _6: u16,mut _7: i32,mut _8: usize) -> isize {
mir! {
type RET = isize;
let _9: &'static *mut *const i128;
let _10: Adt18;
let _11: isize;
let _12: u32;
let _13: (&'static (*const [u64; 2], u8, [bool; 5]),);
let _14: bool;
let _15: (u16,);
let _16: *mut *const i128;
let _17: i8;
let _18: [i8; 8];
let _19: (i8, u64);
let _20: isize;
let _21: u128;
let _22: Adt17;
let _23: [i16; 2];
let _24: f32;
let _25: Adt18;
let _26: f32;
let _27: f64;
let _28: f64;
let _29: isize;
let _30: (u16,);
let _31: Adt18;
let _32: *mut *const i128;
let _33: [isize; 4];
let _34: ((Adt18, (*const u128, u64, u64), u32), [isize; 4], (*const [u64; 2], u8, [bool; 5]), Adt55);
let _35: (i16, u64, i8, (Adt17, i32, Adt17, usize));
let _36: &'static [isize; 7];
let _37: u32;
let _38: u32;
let _39: *const [u64; 2];
let _40: [usize; 2];
let _41: Adt81;
let _42: (*const u128, usize, &'static i64, &'static *mut *const i128);
let _43: f64;
let _44: *const *const u128;
let _45: Adt18;
let _46: &'static Adt18;
let _47: Adt77;
let _48: i16;
let _49: *mut ([u64; 2],);
let _50: isize;
let _51: f32;
let _52: isize;
let _53: [isize; 4];
let _54: (u16,);
let _55: bool;
let _56: [i8; 8];
let _57: ();
let _58: ();
{
_5 = _4;
_8 = _5 as usize;
_5 = _2 as i32;
_4 = -_7;
RET = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_6 = RET as u16;
_7 = _5 - _5;
_3 = !_6;
_7 = _4 ^ _4;
RET = 10785249873435603458663085210304814782_u128 as isize;
_3 = _6 + _6;
RET = 9223372036854775807_isize << _4;
_5 = _7 ^ _4;
_7 = _4;
Call(RET = fn11(_5, _2, _5, Move(_1), _3, _2, _5, _8, _5, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !_6;
RET = (-9223372036854775808_isize);
match RET {
340282366920938463454151235394913435648 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_8 = 2_usize;
RET = (-9223372036854775808_isize) | 9223372036854775807_isize;
_5 = _4 - _4;
_11 = 177_u8 as isize;
_4 = _5;
Goto(bb4)
}
bb4 = {
_12 = !1760764723_u32;
_8 = 4_usize ^ 14066680974719877692_usize;
_6 = !_3;
_14 = _2 <= _2;
_11 = RET;
_8 = 125_i8 as usize;
_11 = -RET;
_15 = (_6,);
_6 = _3 << _11;
RET = _11;
RET = (-423357703675402821_i64) as isize;
_3 = _6;
_3 = _6;
_12 = 3954811333_u32 & 228167668_u32;
RET = _11 << _6;
_6 = (-43207391205371241269433368970424988488_i128) as u16;
_17 = !16_i8;
_8 = 151730485322913979876631606126025589942_i128 as usize;
RET = _11 ^ _11;
_14 = true;
_8 = 0_usize;
_17 = _5 as i8;
_18[_8] = _17 >> _6;
Goto(bb5)
}
bb5 = {
RET = _11;
_19.1 = _11 as u64;
_6 = _2 as u16;
_2 = _19.1;
_17 = !_18[_8];
_19 = (_18[_8], _2);
_19.1 = _2;
_11 = !RET;
_15 = (_3,);
_18[_8] = _17 ^ _19.0;
_11 = -RET;
Goto(bb6)
}
bb6 = {
_6 = _15.0 ^ _15.0;
_9 = &_16;
_15.0 = 27_u8 as u16;
_5 = _12 as i32;
_20 = 2493963945781442925612003210033755540_i128 as isize;
_24 = _17 as f32;
_1 = core::ptr::addr_of!(_23[_8]);
_21 = !125752172579733553949541708706283027218_u128;
_7 = -_4;
_21 = !253794637746187328088029922478993671795_u128;
_20 = _19.0 as isize;
_11 = -_20;
_9 = &(*_9);
_19 = (_18[_8], _2);
_12 = !2514255667_u32;
_22 = Adt17::Variant0 { fld0: 85_u8,fld1: _2 };
(*_1) = 16010_i16 << _19.1;
_24 = _23[_8] as f32;
_19 = (_18[_8], _2);
_23 = [(-21342_i16),(-20593_i16)];
_14 = true;
_9 = &(*_9);
_19 = (_17, Field::<u64>(Variant(_22, 0), 1));
_26 = -_24;
_19 = (_18[_8], _2);
(*_1) = 7259_i16;
Goto(bb7)
}
bb7 = {
_17 = _2 as i8;
_26 = -_24;
Goto(bb8)
}
bb8 = {
(*_1) = _12 as i16;
_19.1 = !Field::<u64>(Variant(_22, 0), 1);
_11 = !_20;
_22 = Adt17::Variant0 { fld0: 100_u8,fld1: _2 };
_27 = _17 as f64;
_6 = !_3;
_30.0 = '\u{d45d5}' as u16;
_26 = _24;
_11 = _20 - RET;
_9 = &_16;
_28 = _27;
_12 = 827056515_u32;
(*_1) = (-23161_i16);
Call(_19.0 = core::intrinsics::transmute(_18[_8]), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = core::ptr::addr_of!((*_1));
_18[_8] = _4 as i8;
RET = _12 as isize;
_3 = !_30.0;
_22 = Adt17::Variant1 { fld0: 9211567527890369904_i64,fld1: 191_u8,fld2: 163058223801139154875470947816409452689_i128,fld3: _27,fld4: (*_1),fld5: _21 };
_3 = _15.0 & _6;
_34.1 = [_20,_11,_20,_11];
_34.0.2 = _12;
_18[_8] = _19.0;
_34.2.2[_8] = !_14;
place!(Field::<u8>(Variant(_22, 1), 1)) = 22_u8;
Goto(bb10)
}
bb10 = {
_34.3.fld4 = core::ptr::addr_of_mut!(_34.2.0);
_29 = _27 as isize;
_35.2 = _12 as i8;
_34.3.fld3 = core::ptr::addr_of_mut!(_24);
_34.3.fld2.0 = [_2,_19.1];
RET = _34.0.2 as isize;
_34.0.1.2 = _3 as u64;
(*_1) = _3 as i16;
_34.1 = [_20,_20,_11,_20];
_11 = _20;
_34.0.2 = _12;
_35.2 = _18[_8];
_15 = (_3,);
_23[_8] = Field::<i16>(Variant(_22, 1), 4);
_34.0.1.0 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_22, 1), 5)));
_34.3.fld4 = core::ptr::addr_of_mut!(_39);
_18 = [_19.0,_35.2,_19.0,_35.2,_17,_19.0,_19.0,_19.0];
_35.3.1 = _7 + _7;
_21 = Field::<u8>(Variant(_22, 1), 1) as u128;
_19.0 = _18[_8];
_37 = !_34.0.2;
place!(Field::<i128>(Variant(_22, 1), 2)) = 84570174296660236574442023243748249286_i128;
match _8 {
0 => bb12,
_ => bb11
}
}
bb11 = {
_1 = core::ptr::addr_of!((*_1));
_18[_8] = _4 as i8;
RET = _12 as isize;
_3 = !_30.0;
_22 = Adt17::Variant1 { fld0: 9211567527890369904_i64,fld1: 191_u8,fld2: 163058223801139154875470947816409452689_i128,fld3: _27,fld4: (*_1),fld5: _21 };
_3 = _15.0 & _6;
_34.1 = [_20,_11,_20,_11];
_34.0.2 = _12;
_18[_8] = _19.0;
_34.2.2[_8] = !_14;
place!(Field::<u8>(Variant(_22, 1), 1)) = 22_u8;
Goto(bb10)
}
bb12 = {
_9 = &_16;
_22 = Adt17::Variant0 { fld0: 54_u8,fld1: _34.0.1.2 };
_33[_8] = _20;
_34.2.2 = [_14,_14,_14,_14,_14];
_34.3.fld2.0[_8] = !_34.0.1.2;
_34.0.1.1 = _2;
_44 = core::ptr::addr_of!(_34.0.1.0);
(*_1) = -20112_i16;
_18[_8] = _19.0;
RET = _34.1[_8];
_48 = -(*_1);
_37 = 5933787095562323685_i64 as u32;
_19 = (_18[_8], _34.3.fld2.0[_8]);
_34.0.2 = !_12;
_34.0.2 = _37 & _12;
_42.0 = core::ptr::addr_of!(_21);
_12 = _8 as u32;
_21 = !40763505307454762069397311186125077740_u128;
match _8 {
1 => bb10,
2 => bb13,
3 => bb14,
4 => bb15,
0 => bb17,
_ => bb16
}
}
bb13 = {
_1 = core::ptr::addr_of!((*_1));
_18[_8] = _4 as i8;
RET = _12 as isize;
_3 = !_30.0;
_22 = Adt17::Variant1 { fld0: 9211567527890369904_i64,fld1: 191_u8,fld2: 163058223801139154875470947816409452689_i128,fld3: _27,fld4: (*_1),fld5: _21 };
_3 = _15.0 & _6;
_34.1 = [_20,_11,_20,_11];
_34.0.2 = _12;
_18[_8] = _19.0;
_34.2.2[_8] = !_14;
place!(Field::<u8>(Variant(_22, 1), 1)) = 22_u8;
Goto(bb10)
}
bb14 = {
_8 = 2_usize;
RET = (-9223372036854775808_isize) | 9223372036854775807_isize;
_5 = _4 - _4;
_11 = 177_u8 as isize;
_4 = _5;
Goto(bb4)
}
bb15 = {
_1 = core::ptr::addr_of!((*_1));
_18[_8] = _4 as i8;
RET = _12 as isize;
_3 = !_30.0;
_22 = Adt17::Variant1 { fld0: 9211567527890369904_i64,fld1: 191_u8,fld2: 163058223801139154875470947816409452689_i128,fld3: _27,fld4: (*_1),fld5: _21 };
_3 = _15.0 & _6;
_34.1 = [_20,_11,_20,_11];
_34.0.2 = _12;
_18[_8] = _19.0;
_34.2.2[_8] = !_14;
place!(Field::<u8>(Variant(_22, 1), 1)) = 22_u8;
Goto(bb10)
}
bb16 = {
_12 = !1760764723_u32;
_8 = 4_usize ^ 14066680974719877692_usize;
_6 = !_3;
_14 = _2 <= _2;
_11 = RET;
_8 = 125_i8 as usize;
_11 = -RET;
_15 = (_6,);
_6 = _3 << _11;
RET = _11;
RET = (-423357703675402821_i64) as isize;
_3 = _6;
_3 = _6;
_12 = 3954811333_u32 & 228167668_u32;
RET = _11 << _6;
_6 = (-43207391205371241269433368970424988488_i128) as u16;
_17 = !16_i8;
_8 = 151730485322913979876631606126025589942_i128 as usize;
RET = _11 ^ _11;
_14 = true;
_8 = 0_usize;
_17 = _5 as i8;
_18[_8] = _17 >> _6;
Goto(bb5)
}
bb17 = {
(*_44) = core::ptr::addr_of!(_21);
_34.3.fld0[_8] = _35.3.1 as i8;
_34.3.fld2.0 = [Field::<u64>(Variant(_22, 0), 1),_2];
_2 = !_19.1;
_52 = RET & RET;
_35.1 = _2 >> _18[_8];
(*_1) = '\u{249d7}' as i16;
_6 = _3 << _19.1;
_15 = (_3,);
_19 = (_18[_8], _35.1);
_35.2 = !_19.0;
_38 = _21 as u32;
(*_44) = core::ptr::addr_of!(_21);
_18[_8] = -_17;
_30 = _15;
_19.1 = _34.0.1.1 | _34.0.1.2;
_42.3 = &_16;
Goto(bb18)
}
bb18 = {
Call(_57 = dump_var(10_usize, 52_usize, Move(_52), 14_usize, Move(_14), 3_usize, Move(_3), 38_usize, Move(_38)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_57 = dump_var(10_usize, 29_usize, Move(_29), 48_usize, Move(_48), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_57 = dump_var(10_usize, 18_usize, Move(_18), 7_usize, Move(_7), 23_usize, Move(_23), 58_usize, _58), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i32,mut _2: u64,mut _3: i32,mut _4: *const i16,mut _5: u16,mut _6: u64,mut _7: i32,mut _8: usize,mut _9: i32,mut _10: u16) -> isize {
mir! {
type RET = isize;
let _11: f64;
let _12: [usize; 5];
let _13: bool;
let _14: (Adt17, i32, Adt17, usize);
let _15: *const [u64; 2];
let _16: Adt81;
let _17: i32;
let _18: u8;
let _19: *mut Adt47;
let _20: Adt18;
let _21: isize;
let _22: ((u16,), [usize; 5], (i8, u64), [u128; 4]);
let _23: (f64,);
let _24: char;
let _25: char;
let _26: [i8; 3];
let _27: usize;
let _28: (Adt18, (*const u128, u64, u64), u32);
let _29: (&'static u32, &'static [isize; 7], &'static (*const [u64; 2], u8, [bool; 5]), i16);
let _30: (&'static (*const [u64; 2], u8, [bool; 5]),);
let _31: f32;
let _32: &'static *mut *const i128;
let _33: *const i128;
let _34: bool;
let _35: char;
let _36: isize;
let _37: [bool; 4];
let _38: u16;
let _39: [usize; 2];
let _40: i32;
let _41: isize;
let _42: [u16; 1];
let _43: *const i128;
let _44: bool;
let _45: f64;
let _46: ();
let _47: ();
{
RET = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_1 = (-86_i8) as i32;
RET = (-9223372036854775808_isize);
_6 = _2 << _7;
_13 = false;
_10 = RET as u16;
_12 = [_8,_8,_8,_8,_8];
_13 = !false;
_5 = _10;
RET = (-9223372036854775808_isize);
_5 = _10 | _10;
RET = (-9223372036854775808_isize) >> _7;
_5 = !_10;
_3 = _9;
RET = (-9_isize) >> _9;
_11 = (-112213567795520520355031250435416239862_i128) as f64;
_7 = _9 | _1;
_14.2 = Adt17::Variant0 { fld0: 238_u8,fld1: _6 };
_14.3 = !_8;
_5 = _10 | _10;
_11 = _7 as f64;
_2 = !_6;
_14.1 = _9 ^ _3;
_14.2 = Adt17::Variant1 { fld0: 8939597825509511657_i64,fld1: 69_u8,fld2: 14527138183875393660590937461106064230_i128,fld3: _11,fld4: (-4405_i16),fld5: 278001376044058733495610410329038488900_u128 };
place!(Field::<u128>(Variant(_14.2, 1), 5)) = !315261925778701473979136650359170440601_u128;
_14.2 = Adt17::Variant0 { fld0: 146_u8,fld1: _2 };
_13 = false & false;
Goto(bb1)
}
bb1 = {
place!(Field::<u8>(Variant(_14.2, 0), 0)) = _11 as u8;
_3 = Field::<u8>(Variant(_14.2, 0), 0) as i32;
_14.2 = Adt17::Variant0 { fld0: 193_u8,fld1: _6 };
_9 = !_7;
_11 = 7410135150136480900896316279857446466_i128 as f64;
_14.0 = Adt17::Variant0 { fld0: 193_u8,fld1: _2 };
Call(_14.1 = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<u8>(Variant(_14.2, 0), 0)) = 36_u8;
_11 = Field::<u64>(Variant(_14.0, 0), 1) as f64;
place!(Field::<u8>(Variant(_14.0, 0), 0)) = !Field::<u8>(Variant(_14.2, 0), 0);
_17 = _9 + _1;
_8 = _14.3 * _14.3;
_5 = _10 ^ _10;
_14.0 = Adt17::Variant1 { fld0: (-660074048719586800_i64),fld1: Field::<u8>(Variant(_14.2, 0), 0),fld2: 50609194321424716209753297650974299211_i128,fld3: _11,fld4: 18744_i16,fld5: 222279727278929245219635400591885167826_u128 };
Goto(bb3)
}
bb3 = {
place!(Field::<i128>(Variant(_14.0, 1), 2)) = 316418581016242232163437891544660076633_u128 as i128;
place!(Field::<u128>(Variant(_14.0, 1), 5)) = RET as u128;
place!(Field::<u8>(Variant(_14.0, 1), 1)) = Field::<i128>(Variant(_14.0, 1), 2) as u8;
place!(Field::<u64>(Variant(_14.2, 0), 1)) = _2 >> _10;
SetDiscriminant(_14.2, 0);
place!(Field::<i16>(Variant(_14.0, 1), 4)) = (-30240_i16) | 19304_i16;
_13 = true | true;
Call(_19 = fn12(_3, _7, Field::<i16>(Variant(_14.0, 1), 4), RET, _17, Move(_4), _3, RET, RET, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_17 = RET as i32;
_10 = !_5;
_14.3 = _14.1 as usize;
_4 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_14.0, 1), 4)));
_22.3 = [Field::<u128>(Variant(_14.0, 1), 5),Field::<u128>(Variant(_14.0, 1), 5),Field::<u128>(Variant(_14.0, 1), 5),Field::<u128>(Variant(_14.0, 1), 5)];
_22.2.1 = '\u{68b80}' as u64;
_7 = _9;
place!(Field::<u8>(Variant(_14.2, 0), 0)) = Field::<u8>(Variant(_14.0, 1), 1) & Field::<u8>(Variant(_14.0, 1), 1);
place!(Field::<i64>(Variant(_14.0, 1), 0)) = (-1596741579320809814_i64);
_14.2 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_14.0, 1), 1),fld1: _2 };
_3 = _2 as i32;
_23.0 = -Field::<f64>(Variant(_14.0, 1), 3);
_28.2 = !3447991460_u32;
Goto(bb5)
}
bb5 = {
_22.0.0 = Field::<u64>(Variant(_14.2, 0), 1) as u16;
(*_4) = 6859_i16;
_14.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_14.2, 0), 0),fld1: Field::<u64>(Variant(_14.2, 0), 1) };
_27 = _22.0.0 as usize;
_17 = _7;
_24 = '\u{5216a}';
_23 = (_11,);
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_25 = _24;
_28.1.2 = _2;
_9 = RET as i32;
Goto(bb6)
}
bb6 = {
_22.0 = (_5,);
_24 = _25;
_8 = _14.3 | _27;
_26 = [(-10_i8),(-97_i8),124_i8];
_14.0 = Move(_14.2);
_21 = -RET;
_14.3 = _28.2 as usize;
_14.1 = _28.2 as i32;
_22.0.0 = _10;
_29.0 = &_28.2;
_3 = _7;
_22.2 = (126_i8, _6);
_5 = !_10;
_23 = (_11,);
_29.3 = !(-374_i16);
RET = -_21;
_9 = _3;
_14.3 = _8;
RET = _22.2.0 as isize;
_23 = (_11,);
_14.3 = _8 - _8;
_28.2 = 1085172672_u32 * 1697097735_u32;
SetDiscriminant(_14.0, 0);
Goto(bb7)
}
bb7 = {
_14.3 = _24 as usize;
_24 = _25;
_29.0 = &_28.2;
_14.2 = Adt17::Variant1 { fld0: (-2280163274956220848_i64),fld1: 65_u8,fld2: 103682063824344451384822183529876007811_i128,fld3: _11,fld4: _29.3,fld5: 220666796427169381250275424871762254133_u128 };
_13 = true;
place!(Field::<i64>(Variant(_14.2, 1), 0)) = (-1589809021331161201_i64);
_26 = [_22.2.0,_22.2.0,_22.2.0];
_9 = -_3;
_22.2.0 = _29.3 as i8;
_29.0 = &_28.2;
_14.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(_14.2, 1), 0),fld1: 72_u8,fld2: 169403801706808065975124230866097678127_i128,fld3: _23.0,fld4: _29.3,fld5: 274629071599146883240426034362280930377_u128 };
_22.2 = (107_i8, _2);
_34 = !_13;
Goto(bb8)
}
bb8 = {
_14.2 = Adt17::Variant1 { fld0: Field::<i64>(Variant(_14.0, 1), 0),fld1: 196_u8,fld2: 102281673182906972642410512621619179757_i128,fld3: _11,fld4: _29.3,fld5: 69084284012994312441771218141764519866_u128 };
_26 = [_22.2.0,_22.2.0,_22.2.0];
_27 = _8;
_11 = 255848583890023329444577654165663282407_u128 as f64;
_35 = _24;
_7 = !_3;
_18 = _25 as u8;
_7 = _9 >> _17;
_4 = core::ptr::addr_of!(_29.3);
place!(Field::<i128>(Variant(_14.0, 1), 2)) = 107379237461840235262934904710614722096_i128 << _8;
_31 = Field::<i64>(Variant(_14.0, 1), 0) as f32;
place!(Field::<u128>(Variant(_14.0, 1), 5)) = 219345748464911541169781483070875607015_u128;
_10 = !_5;
place!(Field::<u8>(Variant(_14.2, 1), 1)) = _18;
place!(Field::<f64>(Variant(_14.0, 1), 3)) = Field::<f64>(Variant(_14.2, 1), 3);
place!(Field::<i64>(Variant(_14.0, 1), 0)) = Field::<i64>(Variant(_14.2, 1), 0) + Field::<i64>(Variant(_14.2, 1), 0);
_10 = _31 as u16;
place!(Field::<f64>(Variant(_14.2, 1), 3)) = -_23.0;
place!(Field::<i64>(Variant(_14.2, 1), 0)) = _28.2 as i64;
place!(Field::<f64>(Variant(_14.2, 1), 3)) = _27 as f64;
_37 = [_13,_13,_13,_13];
_1 = _3;
_3 = _9;
_28.1.2 = _22.2.1;
_7 = _1 - _9;
match _22.2.0 {
0 => bb6,
1 => bb5,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb10,
6 => bb11,
107 => bb13,
_ => bb12
}
}
bb9 = {
_14.3 = _24 as usize;
_24 = _25;
_29.0 = &_28.2;
_14.2 = Adt17::Variant1 { fld0: (-2280163274956220848_i64),fld1: 65_u8,fld2: 103682063824344451384822183529876007811_i128,fld3: _11,fld4: _29.3,fld5: 220666796427169381250275424871762254133_u128 };
_13 = true;
place!(Field::<i64>(Variant(_14.2, 1), 0)) = (-1589809021331161201_i64);
_26 = [_22.2.0,_22.2.0,_22.2.0];
_9 = -_3;
_22.2.0 = _29.3 as i8;
_29.0 = &_28.2;
_14.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(_14.2, 1), 0),fld1: 72_u8,fld2: 169403801706808065975124230866097678127_i128,fld3: _23.0,fld4: _29.3,fld5: 274629071599146883240426034362280930377_u128 };
_22.2 = (107_i8, _2);
_34 = !_13;
Goto(bb8)
}
bb10 = {
place!(Field::<i128>(Variant(_14.0, 1), 2)) = 316418581016242232163437891544660076633_u128 as i128;
place!(Field::<u128>(Variant(_14.0, 1), 5)) = RET as u128;
place!(Field::<u8>(Variant(_14.0, 1), 1)) = Field::<i128>(Variant(_14.0, 1), 2) as u8;
place!(Field::<u64>(Variant(_14.2, 0), 1)) = _2 >> _10;
SetDiscriminant(_14.2, 0);
place!(Field::<i16>(Variant(_14.0, 1), 4)) = (-30240_i16) | 19304_i16;
_13 = true | true;
Call(_19 = fn12(_3, _7, Field::<i16>(Variant(_14.0, 1), 4), RET, _17, Move(_4), _3, RET, RET, _8), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_22.0.0 = Field::<u64>(Variant(_14.2, 0), 1) as u16;
(*_4) = 6859_i16;
_14.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_14.2, 0), 0),fld1: Field::<u64>(Variant(_14.2, 0), 1) };
_27 = _22.0.0 as usize;
_17 = _7;
_24 = '\u{5216a}';
_23 = (_11,);
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_25 = _24;
_28.1.2 = _2;
_9 = RET as i32;
Goto(bb6)
}
bb12 = {
_17 = RET as i32;
_10 = !_5;
_14.3 = _14.1 as usize;
_4 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_14.0, 1), 4)));
_22.3 = [Field::<u128>(Variant(_14.0, 1), 5),Field::<u128>(Variant(_14.0, 1), 5),Field::<u128>(Variant(_14.0, 1), 5),Field::<u128>(Variant(_14.0, 1), 5)];
_22.2.1 = '\u{68b80}' as u64;
_7 = _9;
place!(Field::<u8>(Variant(_14.2, 0), 0)) = Field::<u8>(Variant(_14.0, 1), 1) & Field::<u8>(Variant(_14.0, 1), 1);
place!(Field::<i64>(Variant(_14.0, 1), 0)) = (-1596741579320809814_i64);
_14.2 = Adt17::Variant0 { fld0: Field::<u8>(Variant(_14.0, 1), 1),fld1: _2 };
_3 = _2 as i32;
_23.0 = -Field::<f64>(Variant(_14.0, 1), 3);
_28.2 = !3447991460_u32;
Goto(bb5)
}
bb13 = {
_12 = [_27,_8,_27,_14.3,_8];
_14.2 = Adt17::Variant0 { fld0: _18,fld1: _22.2.1 };
_14.0 = Adt17::Variant1 { fld0: (-1730743466069184474_i64),fld1: Field::<u8>(Variant(_14.2, 0), 0),fld2: 110413256244528846591153591197910928727_i128,fld3: _23.0,fld4: _29.3,fld5: 114809003730473108976880461093239258310_u128 };
_38 = _5;
Goto(bb14)
}
bb14 = {
_1 = !_3;
RET = _21;
_37 = [_13,_34,_34,_34];
_22.0 = (_38,);
_28.1.2 = !_6;
RET = _21;
_33 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_14.0, 1), 2)));
_26 = [_22.2.0,_22.2.0,_22.2.0];
place!(Field::<i64>(Variant(_14.0, 1), 0)) = _24 as i64;
_28.0 = Adt18::Variant0 { fld0: _31,fld1: _35,fld2: RET,fld3: _28.2,fld4: (*_4),fld5: 259948575166966592067822321331402237095_u128,fld6: _27,fld7: Move(_14.2) };
place!(Field::<Adt17>(Variant(_28.0, 0), 7)) = Adt17::Variant1 { fld0: Field::<i64>(Variant(_14.0, 1), 0),fld1: _18,fld2: 27258607610676612359048456614709920995_i128,fld3: _23.0,fld4: Field::<i16>(Variant(_28.0, 0), 4),fld5: 318836505690960710746846945398286746871_u128 };
_28.2 = Field::<u32>(Variant(_28.0, 0), 3);
_36 = _21 | _21;
_11 = -_23.0;
_41 = !_36;
_28.1.1 = _2 & _6;
_22.3 = [254026587100720383101709811093527502929_u128,145127031834842793957217299219338785691_u128,91616884745694399690601402629555698251_u128,81409969466468316292546557432076265079_u128];
_11 = -Field::<f64>(Variant(_14.0, 1), 3);
_44 = _13;
_42 = [_22.0.0];
_17 = _1;
place!(Field::<u128>(Variant(_28.0, 0), 5)) = 199539072037991777478569747359313709500_u128;
place!(Field::<i64>(Variant(_14.0, 1), 0)) = !Field::<i64>(Variant(Field::<Adt17>(Variant(_28.0, 0), 7), 1), 0);
_5 = _28.2 as u16;
_39 = [_27,Field::<usize>(Variant(_28.0, 0), 6)];
place!(Field::<usize>(Variant(_28.0, 0), 6)) = !_27;
RET = Field::<isize>(Variant(_28.0, 0), 2);
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(11_usize, 12_usize, Move(_12), 13_usize, Move(_13), 27_usize, Move(_27), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(11_usize, 1_usize, Move(_1), 2_usize, Move(_2), 42_usize, Move(_42), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(11_usize, 6_usize, Move(_6), 10_usize, Move(_10), 36_usize, Move(_36), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(11_usize, 26_usize, Move(_26), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i32,mut _2: i32,mut _3: i16,mut _4: isize,mut _5: i32,mut _6: *const i16,mut _7: i32,mut _8: isize,mut _9: isize,mut _10: usize) -> *mut Adt47 {
mir! {
type RET = *mut Adt47;
let _11: (&'static u32, &'static [isize; 7], &'static (*const [u64; 2], u8, [bool; 5]), i16);
let _12: Adt47;
let _13: *const *const i16;
let _14: usize;
let _15: [i32; 2];
let _16: (*const [u64; 2], u8, [bool; 5]);
let _17: isize;
let _18: isize;
let _19: char;
let _20: (f64,);
let _21: isize;
let _22: isize;
let _23: *mut ([u64; 2],);
let _24: u32;
let _25: isize;
let _26: isize;
let _27: ([u64; 2],);
let _28: f32;
let _29: *const &'static Adt18;
let _30: usize;
let _31: bool;
let _32: u64;
let _33: f64;
let _34: char;
let _35: *const &'static *mut *const i128;
let _36: f64;
let _37: &'static u64;
let _38: u128;
let _39: i64;
let _40: ();
let _41: ();
{
_1 = !_5;
_4 = 73_i8 as isize;
_1 = !_5;
_4 = -_9;
_11.3 = _3 | _3;
_3 = !_11.3;
_10 = !6_usize;
_5 = _7;
RET = core::ptr::addr_of_mut!(_12);
_4 = _9;
RET = core::ptr::addr_of_mut!((*RET));
_8 = _4 << _7;
_13 = core::ptr::addr_of!(_6);
(*_13) = core::ptr::addr_of!(_3);
(*_13) = core::ptr::addr_of!((*_6));
(*_6) = _11.3;
_4 = 130426406085467975943566633824491614199_u128 as isize;
_11.3 = (*_6) ^ (*_6);
Goto(bb1)
}
bb1 = {
_5 = _1 ^ _7;
_9 = _8;
_9 = -_4;
Goto(bb2)
}
bb2 = {
_14 = !_10;
Call(_2 = fn13(_8, _5, _8, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = core::ptr::addr_of!((*_6));
_14 = false as usize;
RET = core::ptr::addr_of_mut!(_12);
_7 = !_2;
Goto(bb4)
}
bb4 = {
_3 = -_11.3;
_10 = _14 & _14;
_8 = _4;
_16.2 = [false,true,false,false,false];
_16.1 = !130_u8;
_16.2 = [true,false,false,false,false];
_11.2 = &_16;
_19 = '\u{4c05d}';
_5 = 65_i8 as i32;
_17 = !_9;
_1 = _7 - _2;
_2 = _1;
_11.3 = _4 as i16;
_10 = _14 + _14;
_11.2 = &_16;
Goto(bb5)
}
bb5 = {
_4 = -_9;
_16.1 = !190_u8;
RET = core::ptr::addr_of_mut!(_12);
_20.0 = _16.1 as f64;
_8 = _4;
_27.0 = [198168555226966298_u64,16889655988314968814_u64];
_11.2 = &_16;
_16.2 = [true,true,true,true,true];
_30 = _20.0 as usize;
_11.0 = &_24;
_4 = _17;
(*_6) = _11.3 - _11.3;
_11.2 = &_16;
_31 = true;
_31 = _2 >= _1;
_16.1 = 5916_u16 as u8;
_16.0 = core::ptr::addr_of!(_27.0);
_3 = _17 as i16;
_31 = !true;
_13 = core::ptr::addr_of!(_6);
Goto(bb6)
}
bb6 = {
RET = core::ptr::addr_of_mut!((*RET));
_28 = (-82060219821264709992266844023150960281_i128) as f32;
_25 = -_17;
_19 = '\u{b085d}';
_11.2 = &_16;
_15 = [_2,_1];
Call(_16 = fn14(_2, _2, _7, _2, Move((*_13)), _15, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_21 = !_25;
_14 = !_10;
_6 = core::ptr::addr_of!(_11.3);
_24 = 728272005_u32;
_23 = core::ptr::addr_of_mut!(_27);
_16.0 = core::ptr::addr_of!(_27.0);
match _24 {
0 => bb4,
1 => bb3,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
728272005 => bb14,
_ => bb13
}
}
bb8 = {
RET = core::ptr::addr_of_mut!((*RET));
_28 = (-82060219821264709992266844023150960281_i128) as f32;
_25 = -_17;
_19 = '\u{b085d}';
_11.2 = &_16;
_15 = [_2,_1];
Call(_16 = fn14(_2, _2, _7, _2, Move((*_13)), _15, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_4 = -_9;
_16.1 = !190_u8;
RET = core::ptr::addr_of_mut!(_12);
_20.0 = _16.1 as f64;
_8 = _4;
_27.0 = [198168555226966298_u64,16889655988314968814_u64];
_11.2 = &_16;
_16.2 = [true,true,true,true,true];
_30 = _20.0 as usize;
_11.0 = &_24;
_4 = _17;
(*_6) = _11.3 - _11.3;
_11.2 = &_16;
_31 = true;
_31 = _2 >= _1;
_16.1 = 5916_u16 as u8;
_16.0 = core::ptr::addr_of!(_27.0);
_3 = _17 as i16;
_31 = !true;
_13 = core::ptr::addr_of!(_6);
Goto(bb6)
}
bb10 = {
_3 = -_11.3;
_10 = _14 & _14;
_8 = _4;
_16.2 = [false,true,false,false,false];
_16.1 = !130_u8;
_16.2 = [true,false,false,false,false];
_11.2 = &_16;
_19 = '\u{4c05d}';
_5 = 65_i8 as i32;
_17 = !_9;
_1 = _7 - _2;
_2 = _1;
_11.3 = _4 as i16;
_10 = _14 + _14;
_11.2 = &_16;
Goto(bb5)
}
bb11 = {
_6 = core::ptr::addr_of!((*_6));
_14 = false as usize;
RET = core::ptr::addr_of_mut!(_12);
_7 = !_2;
Goto(bb4)
}
bb12 = {
_14 = !_10;
Call(_2 = fn13(_8, _5, _8, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_5 = _1 ^ _7;
_9 = _8;
_9 = -_4;
Goto(bb2)
}
bb14 = {
_18 = -_8;
_20.0 = 9686_u16 as f64;
_16.2 = [_31,_31,_31,_31,_31];
_11.2 = &_16;
_24 = 60_u16 as u32;
_14 = 8725091789619540915_i64 as usize;
_23 = core::ptr::addr_of_mut!((*_23));
_9 = _18;
_22 = _19 as isize;
_1 = !_2;
_16.2 = [_31,_31,_31,_31,_31];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(12_usize, 8_usize, Move(_8), 25_usize, Move(_25), 15_usize, Move(_15), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(12_usize, 30_usize, Move(_30), 24_usize, Move(_24), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(12_usize, 18_usize, Move(_18), 19_usize, Move(_19), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: i32,mut _3: isize,mut _4: i32) -> i32 {
mir! {
type RET = i32;
let _5: &'static (*const [u64; 2], u8, [bool; 5]);
let _6: *const i16;
let _7: isize;
let _8: Adt44;
let _9: [i32; 2];
let _10: i32;
let _11: ();
let _12: ();
{
RET = _4 << _3;
RET = -_4;
_3 = 117_u8 as isize;
_3 = 201513475443333860934320628268292548908_u128 as isize;
RET = _2 >> _4;
_8.fld3 = 157324261079616712515657573976697080368_u128 as f32;
_7 = _3;
_4 = _2;
_3 = !_1;
_3 = _1 & _1;
_9 = [_2,RET];
_8.fld3 = 16878_i16 as f32;
_8.fld1 = core::ptr::addr_of!(_8.fld2);
_7 = !_3;
_9 = [RET,_4];
_4 = RET;
_7 = 65465_u16 as isize;
_8.fld1 = core::ptr::addr_of!(_8.fld2);
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(13_usize, 1_usize, Move(_1), 2_usize, Move(_2), 9_usize, Move(_9), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: i32,mut _5: *const i16,mut _6: [i32; 2],mut _7: i32) -> (*const [u64; 2], u8, [bool; 5]) {
mir! {
type RET = (*const [u64; 2], u8, [bool; 5]);
let _8: char;
let _9: &'static Adt18;
let _10: [isize; 7];
let _11: *const *const u128;
let _12: *const u128;
let _13: *mut *const i128;
let _14: bool;
let _15: Adt77;
let _16: u64;
let _17: (Adt18, *const i16);
let _18: isize;
let _19: (&'static (*const [u64; 2], u8, [bool; 5]),);
let _20: u16;
let _21: (Adt17, i32, Adt17, usize);
let _22: *mut usize;
let _23: u32;
let _24: ();
let _25: ();
{
RET.1 = 202_u8;
_7 = _4;
RET.2 = [false,false,true,true,false];
_8 = '\u{a481c}';
RET.1 = 187_u8;
_4 = -_7;
_2 = _4 & _1;
match RET.1 {
0 => bb1,
187 => bb3,
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
_3 = _4;
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb4,
187 => bb6,
_ => bb5
}
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_3 = _7 ^ _2;
_7 = (-89_i8) as i32;
_3 = _2;
RET.2 = [true,false,false,false,false];
_8 = '\u{ffdd7}';
_3 = -_1;
_4 = -_2;
_1 = _4 ^ _4;
_8 = '\u{1003a1}';
RET.1 = _8 as u8;
RET.1 = (-8177_i16) as u8;
_11 = core::ptr::addr_of!(_12);
RET.1 = 29_u8 << _4;
RET.1 = (-103062009503961060802347428188764496621_i128) as u8;
_4 = 2699484427_u32 as i32;
_6 = [_2,_2];
_8 = '\u{e16ae}';
_8 = '\u{d0704}';
_14 = !true;
_1 = _2 ^ _7;
_11 = core::ptr::addr_of!((*_11));
_14 = !false;
_10 = [(-28_isize),(-77_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _1 ^ _1;
_8 = '\u{7f61a}';
_7 = _3;
_10 = [(-9223372036854775808_isize),107_isize,(-9223372036854775808_isize),(-35_isize),(-11_isize),(-9223372036854775808_isize),9223372036854775807_isize];
Call(_4 = fn15(_6, _2, _3, _7, _1, _2, _1, _6, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = 1_usize as i32;
RET.1 = 40_u8;
_2 = _4 * _3;
_4 = _3;
_17.1 = Move(_5);
RET.1 = !177_u8;
_10 = [(-9223372036854775808_isize),(-101_isize),9223372036854775807_isize,(-63_isize),(-70_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = 3673022695_u32 as i32;
_5 = Move(_17.1);
_16 = 3587693652122438054_u64 & 16632407188821866681_u64;
_16 = 15147928744112920670_u64 | 16720146771142126553_u64;
_8 = '\u{e3213}';
_3 = RET.1 as i32;
_16 = (-21275994870294063081025424196768929397_i128) as u64;
_9 = &_17.0;
_11 = core::ptr::addr_of!((*_11));
_8 = '\u{56640}';
_18 = 9223372036854775807_isize;
_11 = core::ptr::addr_of!((*_11));
_10 = [_18,_18,_18,_18,_18,_18,_18];
_8 = '\u{ce3f2}';
RET.2 = [_14,_14,_14,_14,_14];
_17.1 = Move(_5);
RET.2 = [_14,_14,_14,_14,_14];
RET.2 = [_14,_14,_14,_14,_14];
Call(RET = fn16(_4, _7, _1, _6, _4, _6, _6, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_19.0 = &RET;
_6 = [_7,_4];
_11 = core::ptr::addr_of!((*_11));
_2 = -_7;
_10 = [_18,_18,_18,_18,_18,_18,_18];
RET.1 = 243_u8 | 254_u8;
_20 = 7182203439715219611_i64 as u16;
match _18 {
0 => bb8,
9223372036854775807 => bb11,
_ => bb10
}
}
bb10 = {
_3 = _4;
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb4,
187 => bb6,
_ => bb5
}
}
bb11 = {
Goto(bb12)
}
bb12 = {
_18 = -45_isize;
_21.2 = Adt17::Variant0 { fld0: RET.1,fld1: _16 };
RET.2 = [_14,_14,_14,_14,_14];
Call(_21 = fn17(Move(_19), _6, _4, _6, _7, _2, _1, _7, _1, _4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_21.3 = !0_usize;
place!(Field::<u128>(Variant(_21.0, 1), 5)) = !Field::<u128>(Variant(_21.2, 1), 5);
match Field::<u128>(Variant(_21.2, 1), 5) {
0 => bb1,
1 => bb11,
2 => bb9,
3 => bb12,
4 => bb5,
5 => bb6,
6 => bb7,
278397632895588749326602726687861054197 => bb14,
_ => bb8
}
}
bb14 = {
_1 = _8 as i32;
_7 = !_2;
(*_11) = core::ptr::addr_of!(place!(Field::<u128>(Variant(_21.2, 1), 5)));
_20 = _21.3 as u16;
_17.1 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_21.2, 1), 4)));
place!(Field::<f64>(Variant(_21.2, 1), 3)) = _7 as f64;
_23 = 2731001360_u32 & 6588310_u32;
_11 = core::ptr::addr_of!(_12);
place!(Field::<i16>(Variant(_21.0, 1), 4)) = _4 as i16;
_5 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_21.2, 1), 4)));
(*_5) = Field::<i16>(Variant(_21.0, 1), 4);
_4 = _21.1;
place!(Field::<u8>(Variant(_21.2, 1), 1)) = Field::<u8>(Variant(_21.0, 1), 1) | RET.1;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(14_usize, 6_usize, Move(_6), 23_usize, Move(_23), 18_usize, Move(_18), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(14_usize, 16_usize, Move(_16), 2_usize, Move(_2), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [i32; 2],mut _2: i32,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: i32,mut _8: [i32; 2],mut _9: i32) -> i32 {
mir! {
type RET = i32;
let _10: i32;
let _11: Adt55;
let _12: (&'static (*const [u64; 2], u8, [bool; 5]),);
let _13: usize;
let _14: ();
let _15: ();
{
_8 = [_9,_4];
RET = !_2;
_8 = [_2,RET];
_2 = 34_i8 as i32;
_8 = [_7,_9];
_6 = _4 << _7;
_10 = _7;
_11.fld2.0 = [6889432295255505436_u64,17169507185826362304_u64];
_1 = _8;
RET = _3 + _5;
_11.fld0 = [(-118_i8),16_i8,(-54_i8),(-16_i8),(-23_i8),11_i8,67_i8,(-28_i8)];
_10 = _9 + _6;
_11.fld2.0 = [1026378450656933513_u64,18255013022362763655_u64];
_3 = (-19135_i16) as i32;
_11.fld0 = [46_i8,(-5_i8),75_i8,(-107_i8),(-81_i8),(-70_i8),(-112_i8),121_i8];
_11.fld6 = Adt17::Variant0 { fld0: 99_u8,fld1: 11988061757571557563_u64 };
_1 = [_5,_7];
_11.fld0 = [88_i8,51_i8,(-10_i8),61_i8,125_i8,110_i8,(-65_i8),96_i8];
_2 = _6;
_1 = [_10,_9];
_4 = RET | _5;
_2 = _10 << _7;
_11.fld0 = [88_i8,(-5_i8),(-71_i8),54_i8,53_i8,48_i8,(-18_i8),124_i8];
_2 = 9264899229132066577_u64 as i32;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(15_usize, 2_usize, Move(_2), 5_usize, Move(_5), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(15_usize, 10_usize, Move(_10), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: [i32; 2],mut _5: i32,mut _6: [i32; 2],mut _7: [i32; 2],mut _8: i32) -> (*const [u64; 2], u8, [bool; 5]) {
mir! {
type RET = (*const [u64; 2], u8, [bool; 5]);
let _9: (&'static u32, &'static [isize; 7], &'static (*const [u64; 2], u8, [bool; 5]), i16);
let _10: u32;
let _11: i8;
let _12: u32;
let _13: [u64; 2];
let _14: i64;
let _15: *mut Adt47;
let _16: char;
let _17: u64;
let _18: Adt17;
let _19: f64;
let _20: ();
let _21: ();
{
RET.2 = [false,true,true,false,false];
_8 = (-118_i8) as i32;
_2 = (-29359886568765662890285751446801716028_i128) as i32;
_9.2 = &RET;
_6 = [_1,_5];
_2 = 16724_i16 as i32;
_4 = [_5,_1];
_2 = _3 & _5;
_1 = _2;
_4 = _6;
_2 = 3656408234_u32 as i32;
_10 = 170791278727789674418327421950910477106_u128 as u32;
_9.3 = '\u{cd5f4}' as i16;
Goto(bb1)
}
bb1 = {
_10 = '\u{bd582}' as u32;
_9.0 = &_10;
RET.1 = 141_u8;
_2 = -_5;
_2 = _1;
_4 = [_1,_1];
match RET.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
141 => bb10,
_ => bb9
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
Return()
}
bb10 = {
_10 = 9727863288474949609_u64 as u32;
_11 = (-50_i8);
_1 = !_2;
_2 = _1 + _5;
_8 = 52031_u16 as i32;
_8 = _2;
_7 = _4;
match RET.1 {
0 => bb9,
141 => bb11,
_ => bb6
}
}
bb11 = {
_5 = _8;
_12 = !_10;
_1 = !_2;
RET.2 = [false,false,true,false,false];
_12 = _10;
RET.2 = [false,true,true,false,false];
_9.0 = &_10;
_9.0 = &_10;
match _11 {
0 => bb7,
1 => bb10,
2 => bb9,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211406 => bb12,
_ => bb8
}
}
bb12 = {
_1 = _3;
_5 = !_1;
_3 = _8 ^ _1;
_9.0 = &_10;
_13 = [3653771716221834320_u64,3318737612016830524_u64];
RET.0 = core::ptr::addr_of!(_13);
RET.1 = 48706_u16 as u8;
_8 = true as i32;
_2 = -_5;
RET.0 = core::ptr::addr_of!(_13);
_9.2 = &RET;
_9.3 = 4539877081817967676_i64 as i16;
_8 = -_1;
RET.0 = core::ptr::addr_of!(_13);
_9.0 = &_10;
Call(_9.3 = core::intrinsics::bswap((-26056_i16)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET.0 = core::ptr::addr_of!(_13);
_13 = [14978090615204927245_u64,14889573555203486610_u64];
_10 = _12 | _12;
_17 = 13181710756894793938_u64 | 14541505731891710130_u64;
_9.3 = 12339_i16 & 23259_i16;
RET.2 = [false,false,false,true,true];
_6 = [_1,_2];
_4 = _6;
_5 = 34832_u16 as i32;
_13 = [_17,_17];
_6 = [_1,_1];
_19 = _1 as f64;
_9.0 = &_12;
_1 = -_3;
match _11 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
340282366920938463463374607431768211406 => bb20,
_ => bb19
}
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
Return()
}
bb19 = {
Return()
}
bb20 = {
RET.2 = [true,true,true,true,false];
_8 = _5;
_8 = _1 << _1;
_16 = '\u{498d1}';
_14 = !1796147944947225426_i64;
_8 = _9.3 as i32;
_1 = _3;
_7 = _4;
_7 = [_3,_2];
_1 = _3 ^ _3;
_9.3 = !(-27898_i16);
RET.2 = [false,true,false,true,false];
Goto(bb21)
}
bb21 = {
Call(_20 = dump_var(16_usize, 7_usize, Move(_7), 8_usize, Move(_8), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_20 = dump_var(16_usize, 1_usize, Move(_1), 4_usize, Move(_4), 6_usize, Move(_6), 21_usize, _21), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: (&'static (*const [u64; 2], u8, [bool; 5]),),mut _2: [i32; 2],mut _3: i32,mut _4: [i32; 2],mut _5: i32,mut _6: i32,mut _7: i32,mut _8: i32,mut _9: i32,mut _10: i32) -> (Adt17, i32, Adt17, usize) {
mir! {
type RET = (Adt17, i32, Adt17, usize);
let _11: *mut f32;
let _12: char;
let _13: i64;
let _14: (u16,);
let _15: [i16; 2];
let _16: isize;
let _17: f64;
let _18: u16;
let _19: [u64; 2];
let _20: *const u128;
let _21: i8;
let _22: bool;
let _23: [bool; 5];
let _24: *mut Adt47;
let _25: *mut *const i128;
let _26: &'static u64;
let _27: char;
let _28: *mut usize;
let _29: *const i16;
let _30: [bool; 4];
let _31: ((Adt18, (*const u128, u64, u64), u32), [isize; 4], (*const [u64; 2], u8, [bool; 5]), Adt55);
let _32: [usize; 2];
let _33: &'static u64;
let _34: (*const u128, u64, u64);
let _35: i32;
let _36: f32;
let _37: *mut *const [u64; 2];
let _38: Adt44;
let _39: [usize; 5];
let _40: ();
let _41: ();
{
_6 = !_5;
RET.1 = _6 ^ _8;
_7 = (-119_i8) as i32;
_10 = _3 | _8;
RET.2 = Adt17::Variant0 { fld0: 38_u8,fld1: 8206953520653248268_u64 };
_8 = _6;
_8 = _5;
RET.0 = Adt17::Variant0 { fld0: 116_u8,fld1: 13286171258204331725_u64 };
Call(RET.2 = fn18(_10, _4, _10, _6, RET.1, _4, _10, RET.1, _9, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET.2, 1), 0),fld1: Field::<u8>(Variant(RET.2, 1), 1),fld2: Field::<i128>(Variant(RET.2, 1), 2),fld3: Field::<f64>(Variant(RET.2, 1), 3),fld4: Field::<i16>(Variant(RET.2, 1), 4),fld5: Field::<u128>(Variant(RET.2, 1), 5) };
place!(Field::<u8>(Variant(RET.2, 1), 1)) = Field::<u8>(Variant(RET.0, 1), 1) - Field::<u8>(Variant(RET.0, 1), 1);
RET.1 = _5;
RET.3 = !123060454889466894_usize;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = !Field::<u8>(Variant(RET.2, 1), 1);
_9 = _5;
_5 = !RET.1;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = !Field::<i64>(Variant(RET.2, 1), 0);
RET.3 = Field::<f64>(Variant(RET.0, 1), 3) as usize;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = -Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<i64>(Variant(RET.2, 1), 0)) = Field::<i64>(Variant(RET.0, 1), 0) & Field::<i64>(Variant(RET.0, 1), 0);
RET.3 = 13691584432904149516_usize + 4571659742936565732_usize;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = Field::<i64>(Variant(RET.2, 1), 0) ^ Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<i128>(Variant(RET.2, 1), 2)) = Field::<i128>(Variant(RET.0, 1), 2);
place!(Field::<i64>(Variant(RET.0, 1), 0)) = Field::<f64>(Variant(RET.2, 1), 3) as i64;
place!(Field::<i128>(Variant(RET.0, 1), 2)) = (-84_isize) as i128;
_5 = !RET.1;
RET.2 = Move(RET.0);
_2 = [_9,_10];
_4 = [_9,_6];
place!(Field::<f64>(Variant(RET.2, 1), 3)) = (-9223372036854775808_isize) as f64;
Call(place!(Field::<i128>(Variant(RET.2, 1), 2)) = core::intrinsics::bswap((-150533368351261389321260957044572823854_i128)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.3 = !14071240421365589341_usize;
Goto(bb3)
}
bb3 = {
_13 = Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<f64>(Variant(RET.2, 1), 3)) = _3 as f64;
place!(Field::<u8>(Variant(RET.2, 1), 1)) = !51_u8;
_4 = [_6,RET.1];
RET.0 = Move(RET.2);
RET.3 = Field::<u128>(Variant(RET.0, 1), 5) as usize;
Goto(bb4)
}
bb4 = {
place!(Field::<u128>(Variant(RET.0, 1), 5)) = 43676541216575307669597666355139466197_u128 * 174540220708890256263599136078961935598_u128;
place!(Field::<i128>(Variant(RET.0, 1), 2)) = (-9223372036854775808_isize) as i128;
_4 = [_9,_3];
_14 = (57160_u16,);
place!(Field::<i16>(Variant(RET.0, 1), 4)) = 16409_i16 + 26660_i16;
_17 = 15569528938286205759_u64 as f64;
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 199_u8,fld2: (-128957014668450043483907277723525289089_i128),fld3: _17,fld4: (-31964_i16),fld5: 268849928865536833115239091717469424367_u128 };
place!(Field::<u128>(Variant(RET.0, 1), 5)) = 204850656076080051550278895605851068237_u128 * 4585124216754695953871958979261542893_u128;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = 39_u8 | 119_u8;
place!(Field::<i16>(Variant(RET.0, 1), 4)) = _8 as i16;
_15 = [Field::<i16>(Variant(RET.0, 1), 4),Field::<i16>(Variant(RET.0, 1), 4)];
place!(Field::<i128>(Variant(RET.0, 1), 2)) = Field::<u8>(Variant(RET.0, 1), 1) as i128;
SetDiscriminant(RET.0, 1);
_3 = (-117482420459194862136034877547817168434_i128) as i32;
RET.2 = Adt17::Variant1 { fld0: _13,fld1: 178_u8,fld2: 57980699069231971739087400575375722516_i128,fld3: _17,fld4: 12449_i16,fld5: 23932999155583544081272182911707024321_u128 };
_15 = [8785_i16,(-30294_i16)];
_2 = [RET.1,_9];
place!(Field::<u128>(Variant(RET.0, 1), 5)) = !305913880823432811191523183754324110736_u128;
match _14.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
57160 => bb8,
_ => bb7
}
}
bb5 = {
_13 = Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<f64>(Variant(RET.2, 1), 3)) = _3 as f64;
place!(Field::<u8>(Variant(RET.2, 1), 1)) = !51_u8;
_4 = [_6,RET.1];
RET.0 = Move(RET.2);
RET.3 = Field::<u128>(Variant(RET.0, 1), 5) as usize;
Goto(bb4)
}
bb6 = {
RET.3 = !14071240421365589341_usize;
Goto(bb3)
}
bb7 = {
RET.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET.2, 1), 0),fld1: Field::<u8>(Variant(RET.2, 1), 1),fld2: Field::<i128>(Variant(RET.2, 1), 2),fld3: Field::<f64>(Variant(RET.2, 1), 3),fld4: Field::<i16>(Variant(RET.2, 1), 4),fld5: Field::<u128>(Variant(RET.2, 1), 5) };
place!(Field::<u8>(Variant(RET.2, 1), 1)) = Field::<u8>(Variant(RET.0, 1), 1) - Field::<u8>(Variant(RET.0, 1), 1);
RET.1 = _5;
RET.3 = !123060454889466894_usize;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = !Field::<u8>(Variant(RET.2, 1), 1);
_9 = _5;
_5 = !RET.1;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = !Field::<i64>(Variant(RET.2, 1), 0);
RET.3 = Field::<f64>(Variant(RET.0, 1), 3) as usize;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = -Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<i64>(Variant(RET.2, 1), 0)) = Field::<i64>(Variant(RET.0, 1), 0) & Field::<i64>(Variant(RET.0, 1), 0);
RET.3 = 13691584432904149516_usize + 4571659742936565732_usize;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = Field::<i64>(Variant(RET.2, 1), 0) ^ Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<i128>(Variant(RET.2, 1), 2)) = Field::<i128>(Variant(RET.0, 1), 2);
place!(Field::<i64>(Variant(RET.0, 1), 0)) = Field::<f64>(Variant(RET.2, 1), 3) as i64;
place!(Field::<i128>(Variant(RET.0, 1), 2)) = (-84_isize) as i128;
_5 = !RET.1;
RET.2 = Move(RET.0);
_2 = [_9,_10];
_4 = [_9,_6];
place!(Field::<f64>(Variant(RET.2, 1), 3)) = (-9223372036854775808_isize) as f64;
Call(place!(Field::<i128>(Variant(RET.2, 1), 2)) = core::intrinsics::bswap((-150533368351261389321260957044572823854_i128)), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
place!(Field::<i128>(Variant(RET.2, 1), 2)) = !127007266862366839036278523784232157610_i128;
place!(Field::<u8>(Variant(RET.2, 1), 1)) = 37_u8 | 84_u8;
place!(Field::<i128>(Variant(RET.0, 1), 2)) = 1161199261_u32 as i128;
_16 = -9223372036854775807_isize;
place!(Field::<f64>(Variant(RET.2, 1), 3)) = _17 + _17;
place!(Field::<i64>(Variant(RET.0, 1), 0)) = Field::<i64>(Variant(RET.2, 1), 0);
place!(Field::<i64>(Variant(RET.2, 1), 0)) = _13 << _10;
place!(Field::<i128>(Variant(RET.0, 1), 2)) = Field::<i128>(Variant(RET.2, 1), 2) << _6;
place!(Field::<i16>(Variant(RET.2, 1), 4)) = -18576_i16;
RET.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(RET.2, 1), 1),fld1: 3118256930040068933_u64 };
_9 = Field::<i16>(Variant(RET.2, 1), 4) as i32;
_12 = '\u{a08cd}';
RET.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(RET.2, 1), 1),fld1: 4905644480306816703_u64 };
RET.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET.2, 1), 0),fld1: Field::<u8>(Variant(RET.2, 1), 1),fld2: Field::<i128>(Variant(RET.2, 1), 2),fld3: Field::<f64>(Variant(RET.2, 1), 3),fld4: Field::<i16>(Variant(RET.2, 1), 4),fld5: 325275485528753285258723903086713675222_u128 };
RET.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(RET.2, 1), 1),fld1: 13476489773854896361_u64 };
place!(Field::<i16>(Variant(RET.2, 1), 4)) = (-25857_i16);
_20 = core::ptr::addr_of!(place!(Field::<u128>(Variant(RET.2, 1), 5)));
RET.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET.2, 1), 0),fld1: Field::<u8>(Variant(RET.2, 1), 1),fld2: Field::<i128>(Variant(RET.2, 1), 2),fld3: _17,fld4: Field::<i16>(Variant(RET.2, 1), 4),fld5: 256196438684803762246818169135720672246_u128 };
Goto(bb9)
}
bb9 = {
_13 = _10 as i64;
place!(Field::<u128>(Variant(RET.2, 1), 5)) = 15212076894715706021618935314814035274_u128;
_20 = core::ptr::addr_of!((*_20));
_21 = 1341815346094804255_u64 as i8;
place!(Field::<f64>(Variant(RET.2, 1), 3)) = -Field::<f64>(Variant(RET.0, 1), 3);
_16 = 9223372036854775807_isize >> _13;
_22 = true ^ false;
RET.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(RET.2, 1), 1),fld1: 15855010897671674586_u64 };
match _14.0 {
0 => bb6,
57160 => bb10,
_ => bb2
}
}
bb10 = {
RET.0 = Move(RET.2);
_15 = [Field::<i16>(Variant(RET.0, 1), 4),Field::<i16>(Variant(RET.0, 1), 4)];
_22 = false;
_2 = [_6,_10];
_23 = [_22,_22,_22,_22,_22];
_8 = _6 >> _6;
_17 = Field::<f64>(Variant(RET.0, 1), 3);
_10 = !_5;
place!(Field::<i16>(Variant(RET.0, 1), 4)) = 12581_i16;
_7 = !RET.1;
_4 = [_7,_5];
_3 = _5 + _5;
Goto(bb11)
}
bb11 = {
_14.0 = !20229_u16;
(*_20) = !Field::<u128>(Variant(RET.0, 1), 5);
_14.0 = _17 as u16;
_2 = [_6,_6];
RET.2 = Move(RET.0);
RET.1 = _14.0 as i32;
place!(Field::<u8>(Variant(RET.2, 1), 1)) = _16 as u8;
_7 = _5;
place!(Field::<i64>(Variant(RET.2, 1), 0)) = 12859980340556367433_u64 as i64;
_18 = !_14.0;
_22 = true;
RET.0 = Adt17::Variant0 { fld0: Field::<u8>(Variant(RET.2, 1), 1),fld1: 17392047988364709360_u64 };
match Field::<u128>(Variant(RET.2, 1), 5) {
0 => bb9,
1 => bb6,
15212076894715706021618935314814035274 => bb12,
_ => bb7
}
}
bb12 = {
RET.0 = Move(RET.2);
_5 = _8;
RET.1 = _12 as i32;
_8 = _3 | _6;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = Field::<i16>(Variant(RET.0, 1), 4) as u8;
_3 = _10;
_19 = [4932341846256483703_u64,4467059667724884510_u64];
_14 = (_18,);
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 192_u8,fld2: 77269243941047200672250013121532795156_i128,fld3: _17,fld4: 28887_i16,fld5: 79448187989860158390661270074788683168_u128 };
place!(Field::<i16>(Variant(RET.0, 1), 4)) = (-27880_i16) * (-14285_i16);
_9 = -_7;
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 179_u8,fld2: 144081577240190590982258307674211455932_i128,fld3: _17,fld4: 2912_i16,fld5: 238617963096848015989001879593603376033_u128 };
place!(Field::<f64>(Variant(RET.0, 1), 3)) = _7 as f64;
_17 = -Field::<f64>(Variant(RET.0, 1), 3);
RET.2 = Adt17::Variant0 { fld0: 116_u8,fld1: 11065370141332007565_u64 };
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 172_u8,fld2: (-159782539713527792731328854942593993978_i128),fld3: _17,fld4: 5107_i16,fld5: 197228983758374972273828816688220176315_u128 };
_7 = 89236320763754363478472094962393216649_u128 as i32;
_17 = Field::<f64>(Variant(RET.0, 1), 3);
RET.3 = 3_usize & 1_usize;
_29 = core::ptr::addr_of!(place!(Field::<i16>(Variant(RET.0, 1), 4)));
_13 = !Field::<i64>(Variant(RET.0, 1), 0);
place!(Field::<u128>(Variant(RET.0, 1), 5)) = 192_u8 as u128;
_8 = 16027149355428008159_u64 as i32;
_30 = [_22,_22,_22,_22];
Call(place!(Field::<u128>(Variant(RET.0, 1), 5)) = core::intrinsics::transmute(_19), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET.1 = -_6;
RET.2 = Adt17::Variant0 { fld0: 72_u8,fld1: 3166099115174923164_u64 };
_28 = core::ptr::addr_of_mut!(RET.3);
RET.2 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET.0, 1), 0),fld1: 205_u8,fld2: (-84819501649709227777733138254500161279_i128),fld3: _17,fld4: 27006_i16,fld5: Field::<u128>(Variant(RET.0, 1), 5) };
(*_29) = !10663_i16;
_13 = Field::<i64>(Variant(RET.2, 1), 0);
RET.2 = Adt17::Variant0 { fld0: 238_u8,fld1: 1408373766287117976_u64 };
_31.0.2 = (*_29) as u32;
place!(Field::<u64>(Variant(RET.2, 0), 1)) = !12279134429258057148_u64;
place!(Field::<u64>(Variant(RET.2, 0), 1)) = 10020387817652530774_u64;
_1.0 = &_31.2;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = 5979761330253435972578780081823618801_i128 as u8;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = !205_u8;
_13 = Field::<i64>(Variant(RET.0, 1), 0) | Field::<i64>(Variant(RET.0, 1), 0);
Call(_18 = core::intrinsics::transmute(_14.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_31.0.1.2 = Field::<u64>(Variant(RET.2, 0), 1);
_14.0 = _18 << Field::<i64>(Variant(RET.0, 1), 0);
_20 = core::ptr::addr_of!(place!(Field::<u128>(Variant(RET.0, 1), 5)));
_1.0 = &_31.2;
_33 = &_34.1;
_31.3.fld2.0 = [_31.0.1.2,_31.0.1.2];
_31.0.1.1 = Field::<u64>(Variant(RET.2, 0), 1) & _31.0.1.2;
(*_20) = 278397632895588749326602726687861054197_u128;
_33 = &_31.0.1.2;
_31.0.2 = (-38062771003300081937369425759666527920_i128) as u32;
_16 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_31.0.1.2 = _31.0.1.1 + _31.0.1.1;
_18 = !_14.0;
_8 = (*_20) as i32;
RET.2 = Adt17::Variant1 { fld0: _13,fld1: Field::<u8>(Variant(RET.0, 1), 1),fld2: (-100951859766958399301688427918248243550_i128),fld3: Field::<f64>(Variant(RET.0, 1), 3),fld4: (*_29),fld5: (*_20) };
_30 = [_22,_22,_22,_22];
place!(Field::<i128>(Variant(RET.2, 1), 2)) = Field::<u8>(Variant(RET.2, 1), 1) as i128;
match Field::<u128>(Variant(RET.0, 1), 5) {
0 => bb12,
1 => bb2,
2 => bb13,
3 => bb6,
278397632895588749326602726687861054197 => bb16,
_ => bb15
}
}
bb15 = {
RET.0 = Move(RET.2);
_5 = _8;
RET.1 = _12 as i32;
_8 = _3 | _6;
place!(Field::<u8>(Variant(RET.0, 1), 1)) = Field::<i16>(Variant(RET.0, 1), 4) as u8;
_3 = _10;
_19 = [4932341846256483703_u64,4467059667724884510_u64];
_14 = (_18,);
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 192_u8,fld2: 77269243941047200672250013121532795156_i128,fld3: _17,fld4: 28887_i16,fld5: 79448187989860158390661270074788683168_u128 };
place!(Field::<i16>(Variant(RET.0, 1), 4)) = (-27880_i16) * (-14285_i16);
_9 = -_7;
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 179_u8,fld2: 144081577240190590982258307674211455932_i128,fld3: _17,fld4: 2912_i16,fld5: 238617963096848015989001879593603376033_u128 };
place!(Field::<f64>(Variant(RET.0, 1), 3)) = _7 as f64;
_17 = -Field::<f64>(Variant(RET.0, 1), 3);
RET.2 = Adt17::Variant0 { fld0: 116_u8,fld1: 11065370141332007565_u64 };
RET.0 = Adt17::Variant1 { fld0: _13,fld1: 172_u8,fld2: (-159782539713527792731328854942593993978_i128),fld3: _17,fld4: 5107_i16,fld5: 197228983758374972273828816688220176315_u128 };
_7 = 89236320763754363478472094962393216649_u128 as i32;
_17 = Field::<f64>(Variant(RET.0, 1), 3);
RET.3 = 3_usize & 1_usize;
_29 = core::ptr::addr_of!(place!(Field::<i16>(Variant(RET.0, 1), 4)));
_13 = !Field::<i64>(Variant(RET.0, 1), 0);
place!(Field::<u128>(Variant(RET.0, 1), 5)) = 192_u8 as u128;
_8 = 16027149355428008159_u64 as i32;
_30 = [_22,_22,_22,_22];
Call(place!(Field::<u128>(Variant(RET.0, 1), 5)) = core::intrinsics::transmute(_19), ReturnTo(bb13), UnwindUnreachable())
}
bb16 = {
RET.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET.2, 1), 0),fld1: Field::<u8>(Variant(RET.2, 1), 1),fld2: Field::<i128>(Variant(RET.2, 1), 2),fld3: Field::<f64>(Variant(RET.2, 1), 3),fld4: Field::<i16>(Variant(RET.2, 1), 4),fld5: Field::<u128>(Variant(RET.2, 1), 5) };
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(17_usize, 23_usize, Move(_23), 14_usize, Move(_14), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(17_usize, 19_usize, Move(_19), 12_usize, Move(_12), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(17_usize, 10_usize, Move(_10), 8_usize, Move(_8), 41_usize, _41, 41_usize, _41), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i32,mut _2: [i32; 2],mut _3: i32,mut _4: i32,mut _5: i32,mut _6: [i32; 2],mut _7: i32,mut _8: i32,mut _9: i32,mut _10: i32) -> Adt17 {
mir! {
type RET = Adt17;
let _11: f64;
let _12: (&'static u32, &'static [isize; 7], &'static (*const [u64; 2], u8, [bool; 5]), i16);
let _13: (Adt18, *const i16);
let _14: bool;
let _15: (i16, u64, i8, (Adt17, i32, Adt17, usize));
let _16: isize;
let _17: f64;
let _18: bool;
let _19: Adt17;
let _20: *const &'static *mut *const i128;
let _21: &'static i64;
let _22: &'static i64;
let _23: ();
let _24: ();
{
Goto(bb1)
}
bb1 = {
RET = Adt17::Variant0 { fld0: 215_u8,fld1: 4576697238105424100_u64 };
_10 = _3 & _9;
_3 = _1 | _1;
place!(Field::<u8>(Variant(RET, 0), 0)) = 25_u8 | 247_u8;
place!(Field::<u8>(Variant(RET, 0), 0)) = !110_u8;
RET = Adt17::Variant0 { fld0: 40_u8,fld1: 10661626956939424423_u64 };
_9 = (-80_i8) as i32;
_11 = (-722_i16) as f64;
_8 = -_7;
_2 = _6;
RET = Adt17::Variant1 { fld0: (-6047853033473257125_i64),fld1: 91_u8,fld2: (-164365077067825265460900829673817497343_i128),fld3: _11,fld4: 15053_i16,fld5: 275454065811913426074340583734474054570_u128 };
_7 = -_5;
_12.3 = (-7287_i16);
place!(Field::<i64>(Variant(RET, 1), 0)) = (-6036296898816771738_i64) >> _8;
_12.3 = -(-6836_i16);
Goto(bb2)
}
bb2 = {
_15.3.1 = !_1;
_9 = _4 ^ _1;
_15.2 = (-68_i8) | (-74_i8);
place!(Field::<u128>(Variant(RET, 1), 5)) = 40001136095056375681815581462784437710_u128 | 166851800423434811366680449985426254982_u128;
_14 = !false;
RET = Adt17::Variant1 { fld0: (-3303334404706392309_i64),fld1: 228_u8,fld2: (-129881519497738065898523008072006739846_i128),fld3: _11,fld4: _12.3,fld5: 203128685016217994272936872973208963585_u128 };
place!(Field::<u128>(Variant(RET, 1), 5)) = _14 as u128;
place!(Field::<i64>(Variant(RET, 1), 0)) = _15.2 as i64;
_15.3.3 = 2_usize >> _10;
place!(Field::<i16>(Variant(RET, 1), 4)) = Field::<i64>(Variant(RET, 1), 0) as i16;
Goto(bb3)
}
bb3 = {
_15.3.0 = Adt17::Variant0 { fld0: 73_u8,fld1: 6326659102664382496_u64 };
place!(Field::<i64>(Variant(RET, 1), 0)) = !(-6231661423582987985_i64);
_15.3.2 = Adt17::Variant1 { fld0: Field::<i64>(Variant(RET, 1), 0),fld1: 17_u8,fld2: (-6776762351691561947804704459405446486_i128),fld3: _11,fld4: Field::<i16>(Variant(RET, 1), 4),fld5: Field::<u128>(Variant(RET, 1), 5) };
_6 = [_1,_5];
_16 = 9223372036854775807_isize;
place!(Field::<i128>(Variant(RET, 1), 2)) = 149922174858385359628417592467476662412_i128 + 80820205643893871190780771104415685087_i128;
_15.1 = 12501043924950756752_u64;
place!(Field::<i16>(Variant(_15.3.2, 1), 4)) = -Field::<i16>(Variant(RET, 1), 4);
_2 = _6;
_4 = _15.3.1 << _5;
_17 = _15.1 as f64;
place!(Field::<i128>(Variant(_15.3.2, 1), 2)) = Field::<i64>(Variant(_15.3.2, 1), 0) as i128;
place!(Field::<u8>(Variant(_15.3.0, 0), 0)) = Field::<i128>(Variant(RET, 1), 2) as u8;
_15.3.0 = Adt17::Variant1 { fld0: Field::<i64>(Variant(_15.3.2, 1), 0),fld1: 167_u8,fld2: Field::<i128>(Variant(_15.3.2, 1), 2),fld3: _17,fld4: _12.3,fld5: Field::<u128>(Variant(RET, 1), 5) };
place!(Field::<u128>(Variant(_15.3.0, 1), 5)) = Field::<u128>(Variant(RET, 1), 5) >> _15.3.3;
place!(Field::<i64>(Variant(_15.3.0, 1), 0)) = Field::<u128>(Variant(_15.3.0, 1), 5) as i64;
place!(Field::<i64>(Variant(RET, 1), 0)) = Field::<i64>(Variant(_15.3.0, 1), 0);
place!(Field::<i128>(Variant(RET, 1), 2)) = Field::<i128>(Variant(_15.3.0, 1), 2);
place!(Field::<u8>(Variant(RET, 1), 1)) = !154_u8;
place!(Field::<u8>(Variant(_15.3.2, 1), 1)) = Field::<u8>(Variant(RET, 1), 1);
place!(Field::<f64>(Variant(_15.3.0, 1), 3)) = _16 as f64;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(18_usize, 8_usize, Move(_8), 3_usize, Move(_3), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(18_usize, 10_usize, Move(_10), 2_usize, Move(_2), 24_usize, _24, 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(33170_u16), std::hint::black_box(4751785460443354654_u64), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(87378697013174687582965281345633642286_u128));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){unsafe{printf("Adt17::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt17 {
Variant0{
fld0: u8,
fld1: u64,

},
Variant1{
fld0: i64,
fld1: u8,
fld2: i128,
fld3: f64,
fld4: i16,
fld5: u128,

}}
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: f32,
fld1: char,
fld2: isize,
fld3: u32,
fld4: i16,
fld5: u128,
fld6: usize,
fld7: Adt17,

},
Variant1{
fld0: Adt17,
fld1: u128,
fld2: i32,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt33{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt33 {
fld0: *mut i16,
fld1: Adt17,
fld2: ([u64; 2], [bool; 5], Adt18, [u64; 8]),
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: *mut i16,
fld1: *const *const i16,
fld2: *const i16,
fld3: f32,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt17,

},
Variant1{
fld0: [u64; 8],
fld1: *const u128,
fld2: [i16; 2],
fld3: u64,

},
Variant2{
fld0: bool,
fld1: char,
fld2: f64,
fld3: *mut *const [u64; 2],
fld4: (Adt17, i32, Adt17, usize),
fld5: f32,
fld6: [u64; 8],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: [i8; 8],
fld1: *mut usize,
fld2: ([u64; 2],),
fld3: *mut f32,
fld4: *mut *const [u64; 2],
fld5: *const i16,
fld6: Adt17,
}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: (u16,),
fld1: *mut *const [u64; 2],
fld2: u128,
fld3: [i32; 2],
fld4: i16,
fld5: (*const u128, u64, u64),

},
Variant1{
fld0: u128,
fld1: (f64,),
fld2: [u16; 1],
fld3: Adt18,
fld4: *mut usize,
fld5: Adt55,

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: [i8; 8],
fld1: *mut usize,
fld2: i64,
fld3: *const *const u128,

},
Variant1{
fld0: bool,
fld1: *mut f32,
fld2: [u128; 7],
fld3: ((u16,), [usize; 5], (i8, u64), [u128; 4]),

}}

