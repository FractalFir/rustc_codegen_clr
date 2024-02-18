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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u64,mut _7: i64,mut _8: i128,mut _9: u16) -> Adt36 {
mir! {
type RET = Adt36;
let _10: Adt44;
let _11: f32;
let _12: char;
let _13: Adt42;
let _14: bool;
let _15: (u16, [i64; 7], u128);
let _16: &'static [char; 2];
let _17: [u8; 5];
let _18: i128;
let _19: i64;
let _20: f32;
let _21: f32;
let _22: u16;
let _23: isize;
let _24: Adt36;
let _25: &'static &'static (i8,);
let _26: *mut Adt42;
let _27: (*mut [u8; 7],);
let _28: [u8; 2];
let _29: [u64; 7];
let _30: usize;
let _31: u64;
let _32: ();
let _33: ();
{
RET.fld0 = 169059742455039949525754986765158573404_i128 + 166566417200255675307489557553445033068_i128;
_9 = !33742_u16;
_2 = '\u{abb8d}';
RET.fld1 = [_9,_9,_9];
_6 = 446165173629677338_u64;
RET.fld2 = 20367_i16 * 13415_i16;
_8 = RET.fld0;
_7 = -(-8763989514519332131_i64);
_8 = RET.fld0;
_4 = -(-29_i8);
_12 = _2;
_5 = RET.fld2 * RET.fld2;
_11 = _9 as f32;
RET.fld0 = 2_usize as i128;
_7 = -(-2831546600530182018_i64);
_8 = RET.fld0 * RET.fld0;
_5 = RET.fld2 << _7;
_2 = _12;
RET.fld1 = [_9,_9,_9];
_12 = _2;
_2 = _12;
_7 = true as i64;
RET.fld0 = _8 * _8;
RET.fld1 = [_9,_9,_9];
Call(_8 = core::intrinsics::bswap(RET.fld0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = RET.fld0 << _4;
_13 = Adt42::Variant1 { fld0: true,fld1: 1950137784_u32,fld2: _5,fld3: 1902683201_i32 };
_15.2 = 229_u8 as u128;
_14 = !true;
place!(Field::<bool>(Variant(_13, 1), 0)) = _14 & _14;
_12 = _2;
Goto(bb2)
}
bb2 = {
_2 = _12;
place!(Field::<bool>(Variant(_13, 1), 0)) = _14 | _14;
place!(Field::<bool>(Variant(_13, 1), 0)) = _14;
_1 = Field::<i16>(Variant(_13, 1), 2) < _5;
place!(Field::<i32>(Variant(_13, 1), 3)) = (-1078831179_i32) ^ 188917972_i32;
RET.fld0 = -_8;
RET.fld0 = _8 * _8;
_9 = _2 as u16;
_17 = [80_u8,232_u8,148_u8,199_u8,197_u8];
_14 = !_1;
_2 = _12;
_5 = RET.fld0 as i16;
RET.fld1 = [_9,_9,_9];
_15.1 = [_7,_7,_7,_7,_7,_7,_7];
_6 = 3765653495441757835_u64 * 17793136392880737760_u64;
_19 = !_7;
RET.fld2 = !_5;
RET.fld2 = Field::<i16>(Variant(_13, 1), 2);
_15.2 = _12 as u128;
_20 = _11 - _11;
_11 = _20;
_15.0 = !_9;
Goto(bb3)
}
bb3 = {
RET.fld1 = [_15.0,_9,_9];
_1 = _14;
_19 = _7 ^ _7;
_20 = -_11;
Call(_12 = fn1(_1, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24.fld0 = RET.fld0 * RET.fld0;
_9 = _15.0;
_21 = (-9223372036854775808_isize) as f32;
place!(Field::<i32>(Variant(_13, 1), 3)) = -(-440143362_i32);
_18 = !_24.fld0;
place!(Field::<u32>(Variant(_13, 1), 1)) = _12 as u32;
_5 = !Field::<i16>(Variant(_13, 1), 2);
_17 = [10_u8,12_u8,166_u8,211_u8,167_u8];
_2 = _12;
_18 = !_24.fld0;
_2 = _12;
_24.fld1 = [_9,_15.0,_15.0];
_26 = core::ptr::addr_of_mut!(_13);
RET.fld2 = Field::<i16>(Variant(_13, 1), 2) - _5;
place!(Field::<i16>(Variant(_13, 1), 2)) = !RET.fld2;
_14 = _1;
_4 = !5_i8;
place!(Field::<i32>(Variant(_13, 1), 3)) = !(-1770675066_i32);
_24.fld0 = _18 & RET.fld0;
place!(Field::<i32>(Variant((*_26), 1), 3)) = RET.fld0 as i32;
_18 = RET.fld0;
(*_26) = Adt42::Variant1 { fld0: _1,fld1: 2974883299_u32,fld2: RET.fld2,fld3: 1781918451_i32 };
_19 = _6 as i64;
_8 = 1293630661_i32 as i128;
_15.2 = 267723804425041177886802949530121755695_u128 * 123728882776140804314293417553756118978_u128;
_8 = RET.fld0 & _24.fld0;
Goto(bb5)
}
bb5 = {
_23 = 9223372036854775807_isize;
RET.fld0 = -_24.fld0;
_28 = [33_u8,12_u8];
_22 = !_9;
_15.0 = !_9;
_24 = Move(RET);
RET.fld1 = [_15.0,_15.0,_15.0];
match _23 {
0 => bb6,
1 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb6 = {
_24.fld0 = RET.fld0 * RET.fld0;
_9 = _15.0;
_21 = (-9223372036854775808_isize) as f32;
place!(Field::<i32>(Variant(_13, 1), 3)) = -(-440143362_i32);
_18 = !_24.fld0;
place!(Field::<u32>(Variant(_13, 1), 1)) = _12 as u32;
_5 = !Field::<i16>(Variant(_13, 1), 2);
_17 = [10_u8,12_u8,166_u8,211_u8,167_u8];
_2 = _12;
_18 = !_24.fld0;
_2 = _12;
_24.fld1 = [_9,_15.0,_15.0];
_26 = core::ptr::addr_of_mut!(_13);
RET.fld2 = Field::<i16>(Variant(_13, 1), 2) - _5;
place!(Field::<i16>(Variant(_13, 1), 2)) = !RET.fld2;
_14 = _1;
_4 = !5_i8;
place!(Field::<i32>(Variant(_13, 1), 3)) = !(-1770675066_i32);
_24.fld0 = _18 & RET.fld0;
place!(Field::<i32>(Variant((*_26), 1), 3)) = RET.fld0 as i32;
_18 = RET.fld0;
(*_26) = Adt42::Variant1 { fld0: _1,fld1: 2974883299_u32,fld2: RET.fld2,fld3: 1781918451_i32 };
_19 = _6 as i64;
_8 = 1293630661_i32 as i128;
_15.2 = 267723804425041177886802949530121755695_u128 * 123728882776140804314293417553756118978_u128;
_8 = RET.fld0 & _24.fld0;
Goto(bb5)
}
bb7 = {
RET.fld1 = [_15.0,_9,_9];
_1 = _14;
_19 = _7 ^ _7;
_20 = -_11;
Call(_12 = fn1(_1, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_2 = _12;
place!(Field::<bool>(Variant(_13, 1), 0)) = _14 | _14;
place!(Field::<bool>(Variant(_13, 1), 0)) = _14;
_1 = Field::<i16>(Variant(_13, 1), 2) < _5;
place!(Field::<i32>(Variant(_13, 1), 3)) = (-1078831179_i32) ^ 188917972_i32;
RET.fld0 = -_8;
RET.fld0 = _8 * _8;
_9 = _2 as u16;
_17 = [80_u8,232_u8,148_u8,199_u8,197_u8];
_14 = !_1;
_2 = _12;
_5 = RET.fld0 as i16;
RET.fld1 = [_9,_9,_9];
_15.1 = [_7,_7,_7,_7,_7,_7,_7];
_6 = 3765653495441757835_u64 * 17793136392880737760_u64;
_19 = !_7;
RET.fld2 = !_5;
RET.fld2 = Field::<i16>(Variant(_13, 1), 2);
_15.2 = _12 as u128;
_20 = _11 - _11;
_11 = _20;
_15.0 = !_9;
Goto(bb3)
}
bb9 = {
_4 = _9 as i8;
place!(Field::<i16>(Variant(_13, 1), 2)) = _24.fld2;
RET.fld2 = _23 as i16;
place!(Field::<i32>(Variant((*_26), 1), 3)) = (-538968142_i32) ^ (-1704319010_i32);
_24.fld2 = !Field::<i16>(Variant(_13, 1), 2);
_29 = [_6,_6,_6,_6,_6,_6,_6];
_3 = !_23;
place!(Field::<u32>(Variant(_13, 1), 1)) = 1129213261_u32;
_29 = [_6,_6,_6,_6,_6,_6,_6];
_8 = -_24.fld0;
_2 = _12;
_26 = core::ptr::addr_of_mut!(_13);
_1 = Field::<bool>(Variant(_13, 1), 0) > Field::<bool>(Variant(_13, 1), 0);
_29 = [_6,_6,_6,_6,_6,_6,_6];
RET.fld0 = _24.fld0;
_18 = 6_usize as i128;
place!(Field::<i32>(Variant((*_26), 1), 3)) = 477890344_i32;
_2 = _12;
_5 = _6 as i16;
_20 = _15.2 as f32;
_6 = !15798662449515780539_u64;
_30 = 13138739233374724262_usize & 2260814157779463413_usize;
RET.fld1 = [_22,_15.0,_15.0];
SetDiscriminant((*_26), 3);
place!(Field::<*const f32>(Variant(_13, 3), 1)) = core::ptr::addr_of!(_21);
place!(Field::<[u8; 7]>(Variant((*_26), 3), 2)) = [22_u8,96_u8,87_u8,99_u8,101_u8,180_u8,52_u8];
place!(Field::<[i8; 5]>(Variant((*_26), 3), 4)) = [_4,_4,_4,_4,_4];
place!(Field::<[i8; 5]>(Variant(_13, 3), 4)) = [_4,_4,_4,_4,_4];
place!(Field::<[isize; 2]>(Variant((*_26), 3), 0)) = [_23,_23];
place!(Field::<*mut [u8; 7]>(Variant(_13, 3), 3)) = core::ptr::addr_of_mut!(place!(Field::<[u8; 7]>(Variant(_13, 3), 2)));
Goto(bb10)
}
bb10 = {
Call(_32 = dump_var(0_usize, 2_usize, Move(_2), 6_usize, Move(_6), 29_usize, Move(_29), 14_usize, Move(_14)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_32 = dump_var(0_usize, 9_usize, Move(_9), 23_usize, Move(_23), 28_usize, Move(_28), 12_usize, Move(_12)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_32 = dump_var(0_usize, 5_usize, Move(_5), 7_usize, Move(_7), 33_usize, _33, 33_usize, _33), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: i64) -> char {
mir! {
type RET = char;
let _3: (*mut [u8; 7],);
let _4: i16;
let _5: bool;
let _6: bool;
let _7: [u16; 7];
let _8: (u16, [i64; 7], u128);
let _9: f64;
let _10: Adt30;
let _11: f64;
let _12: *const [u16; 3];
let _13: f32;
let _14: [u8; 7];
let _15: Adt21;
let _16: isize;
let _17: isize;
let _18: i16;
let _19: u128;
let _20: &'static (i8,);
let _21: Adt51;
let _22: &'static u8;
let _23: [u8; 2];
let _24: (i8,);
let _25: char;
let _26: (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42);
let _27: [u16; 7];
let _28: isize;
let _29: &'static (i8,);
let _30: Adt42;
let _31: (u16, [i64; 7], u128);
let _32: f32;
let _33: i128;
let _34: ();
let _35: ();
{
RET = '\u{1c484}';
RET = '\u{a3df0}';
RET = '\u{49dcf}';
_2 = (-4975645460502440471_i64);
_1 = _2 < _2;
RET = '\u{10b3fb}';
RET = '\u{100fb2}';
_2 = RET as i64;
RET = '\u{30a10}';
_2 = -(-7709243996341971944_i64);
_1 = !false;
_2 = 8675716414437828898_i64;
RET = '\u{91db7}';
_1 = true & false;
RET = '\u{21728}';
_1 = !true;
RET = '\u{b38f2}';
_5 = RET > RET;
Goto(bb1)
}
bb1 = {
_1 = !_5;
RET = '\u{9505d}';
_6 = !_5;
_4 = !13417_i16;
_2 = (-8161567350542161168_i64);
_1 = _4 != _4;
_4 = (-23619_i16) & 26642_i16;
RET = '\u{e5941}';
_2 = -2540886967785184014_i64;
_7 = [57970_u16,14308_u16,27931_u16,42875_u16,50289_u16,6182_u16,21632_u16];
_5 = !_6;
_7 = [60481_u16,8763_u16,17005_u16,21426_u16,9014_u16,60127_u16,51898_u16];
_6 = !_1;
RET = '\u{ef0ac}';
_6 = _5;
_4 = 817998934_i32 as i16;
_4 = (-17359_i16) & 8220_i16;
_4 = 244481816414870211311818860944643899312_u128 as i16;
_6 = !_1;
Goto(bb2)
}
bb2 = {
RET = '\u{491a1}';
_6 = _4 == _4;
RET = '\u{40f81}';
RET = '\u{33003}';
RET = '\u{4b6e4}';
_7 = [26721_u16,13181_u16,35858_u16,8687_u16,52914_u16,32875_u16,35075_u16];
_4 = 19539_i16;
_8.1 = [_2,_2,_2,_2,_2,_2,_2];
_6 = _1 ^ _5;
_8.0 = 40210_u16 - 53728_u16;
_7 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_8.2 = !82309871981031591187249221325387176818_u128;
RET = '\u{6fd3a}';
_2 = (-8517817644767771789_i64);
_2 = 50_u8 as i64;
_6 = _5 | _5;
_9 = _4 as f64;
Call(_3 = fn2(_7, _7, _1, _6, RET, _6, _1, _8.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = _6 != _1;
_9 = 120_i8 as f64;
RET = '\u{c889a}';
_4 = 6785925152370525867_u64 as i16;
_8.0 = 30970_u16;
_9 = _2 as f64;
_10 = Adt30::Variant1 { fld0: 218_u8 };
RET = '\u{9775e}';
place!(Field::<u8>(Variant(_10, 1), 0)) = 174_u8 ^ 58_u8;
_1 = !_6;
_5 = _6 & _6;
_5 = _6;
_11 = -_9;
RET = '\u{69331}';
RET = '\u{87554}';
_5 = !_1;
_13 = (-105_i8) as f32;
_4 = (-20178_i16);
_15 = Adt21::Variant1 { fld0: 5549350199219644337732845442730285793_i128,fld1: _4,fld2: 9223372036854775807_isize,fld3: 1586343674_u32 };
place!(Field::<isize>(Variant(_15, 1), 2)) = (-3_isize) * 9223372036854775807_isize;
_4 = Field::<i16>(Variant(_15, 1), 1);
Goto(bb4)
}
bb4 = {
place!(Field::<i128>(Variant(_15, 1), 0)) = !38031495953708646153806015874496047349_i128;
_3.0 = core::ptr::addr_of_mut!(_14);
_10 = Adt30::Variant1 { fld0: 231_u8 };
_18 = RET as i16;
place!(Field::<u32>(Variant(_15, 1), 3)) = _8.2 as u32;
_13 = 761388561093736090_u64 as f32;
place!(Field::<u8>(Variant(_10, 1), 0)) = 241_u8 + 83_u8;
RET = '\u{f7cb8}';
_5 = !_6;
_7 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_8.0 = 52727_u16 | 57210_u16;
place!(Field::<u32>(Variant(_15, 1), 3)) = !2821801587_u32;
Call(place!(Field::<i16>(Variant(_15, 1), 1)) = core::intrinsics::bswap(_18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = _18;
place!(Field::<u32>(Variant(_15, 1), 3)) = 924537598_u32 * 3848839856_u32;
_9 = _11 + _11;
_14 = [Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0)];
_21.fld0 = (Move(_3.0),);
Goto(bb6)
}
bb6 = {
place!(Field::<i128>(Variant(_15, 1), 0)) = _2 as i128;
_12 = core::ptr::addr_of!(_21.fld6.fld1);
_21.fld6.fld0 = Field::<i128>(Variant(_15, 1), 0) << Field::<i128>(Variant(_15, 1), 0);
SetDiscriminant(_15, 1);
_3.0 = Move(_21.fld0.0);
_13 = _21.fld6.fld0 as f32;
_3.0 = core::ptr::addr_of_mut!(_14);
_12 = core::ptr::addr_of!(_21.fld6.fld1);
place!(Field::<isize>(Variant(_15, 1), 2)) = -1_isize;
_23 = [Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0)];
_21.fld6.fld2 = _9 as i16;
_21.fld6.fld1 = [_8.0,_8.0,_8.0];
_26.1.1 = RET;
_21.fld0 = (Move(_3.0),);
_16 = Field::<isize>(Variant(_15, 1), 2) * Field::<isize>(Variant(_15, 1), 2);
place!(Field::<i128>(Variant(_15, 1), 0)) = !_21.fld6.fld0;
Goto(bb7)
}
bb7 = {
SetDiscriminant(_10, 2);
_24.0 = _2 as i8;
_13 = _21.fld6.fld2 as f32;
place!(Field::<[u8; 7]>(Variant(_10, 2), 3)) = _14;
_2 = -(-5625875516433198644_i64);
(*_12) = [_8.0,_8.0,_8.0];
_12 = core::ptr::addr_of!(_21.fld6.fld1);
_21.fld2.0 = _24.0 >> _16;
_26.2 = (_8.0, _8.1, _8.2);
place!(Field::<u32>(Variant(_10, 2), 0)) = 89083774_u32;
_17 = _26.2.0 as isize;
_28 = !Field::<isize>(Variant(_15, 1), 2);
_7 = [_8.0,_8.0,_26.2.0,_26.2.0,_8.0,_26.2.0,_8.0];
_25 = _26.1.1;
RET = _26.1.1;
_10 = Adt30::Variant3 { fld0: 186_u8,fld1: _23 };
Goto(bb8)
}
bb8 = {
_15 = Adt21::Variant1 { fld0: _21.fld6.fld0,fld1: _18,fld2: _17,fld3: 2995271043_u32 };
_20 = &_24;
place!(Field::<isize>(Variant(_15, 1), 2)) = _5 as isize;
_20 = &(*_20);
_21.fld6.fld0 = Field::<i128>(Variant(_15, 1), 0) << _4;
_21.fld0.0 = core::ptr::addr_of_mut!(_14);
_20 = &(*_20);
_21.fld1 = [_2,_2,_2,_2,_2,_2,_2];
_17 = !Field::<isize>(Variant(_15, 1), 2);
_10 = Adt30::Variant3 { fld0: 31_u8,fld1: _23 };
place!(Field::<u32>(Variant(_15, 1), 3)) = 420166642_u32;
_26.2 = (_8.0, _8.1, _8.2);
_29 = &(*_20);
_11 = -_9;
_10 = Adt30::Variant2 { fld0: Field::<u32>(Variant(_15, 1), 3),fld1: _26.1.1,fld2: _16,fld3: _14 };
_12 = core::ptr::addr_of!((*_12));
place!(Field::<u32>(Variant(_10, 2), 0)) = !Field::<u32>(Variant(_15, 1), 3);
match Field::<u32>(Variant(_15, 1), 3) {
0 => bb5,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
420166642 => bb15,
_ => bb14
}
}
bb9 = {
SetDiscriminant(_10, 2);
_24.0 = _2 as i8;
_13 = _21.fld6.fld2 as f32;
place!(Field::<[u8; 7]>(Variant(_10, 2), 3)) = _14;
_2 = -(-5625875516433198644_i64);
(*_12) = [_8.0,_8.0,_8.0];
_12 = core::ptr::addr_of!(_21.fld6.fld1);
_21.fld2.0 = _24.0 >> _16;
_26.2 = (_8.0, _8.1, _8.2);
place!(Field::<u32>(Variant(_10, 2), 0)) = 89083774_u32;
_17 = _26.2.0 as isize;
_28 = !Field::<isize>(Variant(_15, 1), 2);
_7 = [_8.0,_8.0,_26.2.0,_26.2.0,_8.0,_26.2.0,_8.0];
_25 = _26.1.1;
RET = _26.1.1;
_10 = Adt30::Variant3 { fld0: 186_u8,fld1: _23 };
Goto(bb8)
}
bb10 = {
place!(Field::<i128>(Variant(_15, 1), 0)) = _2 as i128;
_12 = core::ptr::addr_of!(_21.fld6.fld1);
_21.fld6.fld0 = Field::<i128>(Variant(_15, 1), 0) << Field::<i128>(Variant(_15, 1), 0);
SetDiscriminant(_15, 1);
_3.0 = Move(_21.fld0.0);
_13 = _21.fld6.fld0 as f32;
_3.0 = core::ptr::addr_of_mut!(_14);
_12 = core::ptr::addr_of!(_21.fld6.fld1);
place!(Field::<isize>(Variant(_15, 1), 2)) = -1_isize;
_23 = [Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0)];
_21.fld6.fld2 = _9 as i16;
_21.fld6.fld1 = [_8.0,_8.0,_8.0];
_26.1.1 = RET;
_21.fld0 = (Move(_3.0),);
_16 = Field::<isize>(Variant(_15, 1), 2) * Field::<isize>(Variant(_15, 1), 2);
place!(Field::<i128>(Variant(_15, 1), 0)) = !_21.fld6.fld0;
Goto(bb7)
}
bb11 = {
_4 = _18;
place!(Field::<u32>(Variant(_15, 1), 3)) = 924537598_u32 * 3848839856_u32;
_9 = _11 + _11;
_14 = [Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0),Field::<u8>(Variant(_10, 1), 0)];
_21.fld0 = (Move(_3.0),);
Goto(bb6)
}
bb12 = {
_1 = !_5;
RET = '\u{9505d}';
_6 = !_5;
_4 = !13417_i16;
_2 = (-8161567350542161168_i64);
_1 = _4 != _4;
_4 = (-23619_i16) & 26642_i16;
RET = '\u{e5941}';
_2 = -2540886967785184014_i64;
_7 = [57970_u16,14308_u16,27931_u16,42875_u16,50289_u16,6182_u16,21632_u16];
_5 = !_6;
_7 = [60481_u16,8763_u16,17005_u16,21426_u16,9014_u16,60127_u16,51898_u16];
_6 = !_1;
RET = '\u{ef0ac}';
_6 = _5;
_4 = 817998934_i32 as i16;
_4 = (-17359_i16) & 8220_i16;
_4 = 244481816414870211311818860944643899312_u128 as i16;
_6 = !_1;
Goto(bb2)
}
bb13 = {
_5 = _6 != _1;
_9 = 120_i8 as f64;
RET = '\u{c889a}';
_4 = 6785925152370525867_u64 as i16;
_8.0 = 30970_u16;
_9 = _2 as f64;
_10 = Adt30::Variant1 { fld0: 218_u8 };
RET = '\u{9775e}';
place!(Field::<u8>(Variant(_10, 1), 0)) = 174_u8 ^ 58_u8;
_1 = !_6;
_5 = _6 & _6;
_5 = _6;
_11 = -_9;
RET = '\u{69331}';
RET = '\u{87554}';
_5 = !_1;
_13 = (-105_i8) as f32;
_4 = (-20178_i16);
_15 = Adt21::Variant1 { fld0: 5549350199219644337732845442730285793_i128,fld1: _4,fld2: 9223372036854775807_isize,fld3: 1586343674_u32 };
place!(Field::<isize>(Variant(_15, 1), 2)) = (-3_isize) * 9223372036854775807_isize;
_4 = Field::<i16>(Variant(_15, 1), 1);
Goto(bb4)
}
bb14 = {
RET = '\u{491a1}';
_6 = _4 == _4;
RET = '\u{40f81}';
RET = '\u{33003}';
RET = '\u{4b6e4}';
_7 = [26721_u16,13181_u16,35858_u16,8687_u16,52914_u16,32875_u16,35075_u16];
_4 = 19539_i16;
_8.1 = [_2,_2,_2,_2,_2,_2,_2];
_6 = _1 ^ _5;
_8.0 = 40210_u16 - 53728_u16;
_7 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_8.2 = !82309871981031591187249221325387176818_u128;
RET = '\u{6fd3a}';
_2 = (-8517817644767771789_i64);
_2 = 50_u8 as i64;
_6 = _5 | _5;
_9 = _4 as f64;
Call(_3 = fn2(_7, _7, _1, _6, RET, _6, _1, _8.2), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_4 = Field::<i16>(Variant(_15, 1), 1);
_11 = _9 - _9;
_3 = Move(_21.fld0);
_8.1 = [_2,_2,_2,_2,_2,_2,_2];
_21.fld2.0 = _24.0 & _24.0;
place!(Field::<char>(Variant(_10, 2), 1)) = _25;
_9 = _11 + _11;
_26.2.1 = _8.1;
place!(Field::<i128>(Variant(_15, 1), 0)) = !_21.fld6.fld0;
_8 = (_26.2.0, _26.2.1, _26.2.2);
_17 = _28 * Field::<isize>(Variant(_15, 1), 2);
_31.0 = _26.2.0 & _8.0;
_27 = [_26.2.0,_31.0,_31.0,_31.0,_31.0,_31.0,_31.0];
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(1_usize, 28_usize, Move(_28), 8_usize, Move(_8), 25_usize, Move(_25), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(1_usize, 1_usize, Move(_1), 6_usize, Move(_6), 18_usize, Move(_18), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [u16; 7],mut _2: [u16; 7],mut _3: bool,mut _4: bool,mut _5: char,mut _6: bool,mut _7: bool,mut _8: u128) -> (*mut [u8; 7],) {
mir! {
type RET = (*mut [u8; 7],);
let _9: Adt21;
let _10: bool;
let _11: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _12: (*const i64, Adt36);
let _13: isize;
let _14: Adt51;
let _15: i16;
let _16: i32;
let _17: &'static u8;
let _18: u128;
let _19: [i8; 5];
let _20: usize;
let _21: [u8; 7];
let _22: u16;
let _23: f32;
let _24: i128;
let _25: ();
let _26: ();
{
_8 = 172787336319363576963602140535490747590_u128;
_9 = Adt21::Variant1 { fld0: 107925812711296135924109098840323101932_i128,fld1: 20399_i16,fld2: 9223372036854775807_isize,fld3: 99438737_u32 };
place!(Field::<i128>(Variant(_9, 1), 0)) = 62515818505490277848244738297346611874_i128 | 48408999560388757283036313863824364645_i128;
_8 = 40960208951847840882841069746265912778_u128;
_6 = _3 | _7;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
40960208951847840882841069746265912778 => bb6,
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
_11.3.2.2 = !_8;
_11.0.0 = 53228_u16;
_11.3.2.1 = [(-8658407651134569116_i64),(-8947514897247983672_i64),(-6170157434487326492_i64),(-2174838817404008687_i64),5767179106042535939_i64,4016032728513050312_i64,6018807237564272649_i64];
_2 = [_11.0.0,_11.0.0,_11.0.0,_11.0.0,_11.0.0,_11.0.0,_11.0.0];
_11.0.1 = [7390210449889938493_i64,(-8827018636862078556_i64),(-121862161060212656_i64),1411162896924222896_i64,(-5142980615336155903_i64),8545052356193470025_i64,(-3998129339119262463_i64)];
place!(Field::<i16>(Variant(_9, 1), 1)) = !(-26789_i16);
_11.3.3 = Adt42::Variant1 { fld0: _6,fld1: 1736649727_u32,fld2: Field::<i16>(Variant(_9, 1), 1),fld3: 9800228_i32 };
_11.3.2.0 = !_11.0.0;
_10 = !_6;
_11.3.2.0 = _5 as u16;
_5 = '\u{42363}';
_11.3.1.1 = _5;
_11.3.0 = Adt21::Variant0 { fld0: _6,fld1: _2 };
_11.0 = (_11.3.2.0, _11.3.2.1, _11.3.2.2);
_12.1.fld1 = [_11.0.0,_11.0.0,_11.0.0];
place!(Field::<i16>(Variant(_9, 1), 1)) = !Field::<i16>(Variant(_11.3.3, 1), 2);
place!(Field::<u32>(Variant(_9, 1), 3)) = 3262593265_u32 + 4283834151_u32;
_13 = -(-83_isize);
place!(Field::<u32>(Variant(_11.3.3, 1), 1)) = !Field::<u32>(Variant(_9, 1), 3);
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = Field::<i16>(Variant(_9, 1), 1) >> Field::<i128>(Variant(_9, 1), 0);
place!(Field::<[u16; 7]>(Variant(_11.3.0, 0), 1)) = _1;
_1 = [_11.0.0,_11.0.0,_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.0.0];
_14.fld6 = Adt36 { fld0: Field::<i128>(Variant(_9, 1), 0),fld1: _12.1.fld1,fld2: Field::<i16>(Variant(_11.3.3, 1), 2) };
_11.1 = [1321895462_i32,1480268369_i32,(-2021686904_i32),1915848073_i32];
place!(Field::<i16>(Variant(_9, 1), 1)) = !_14.fld6.fld2;
_5 = _11.3.1.1;
_14.fld2.0 = 12_i8 << _13;
Goto(bb7)
}
bb7 = {
SetDiscriminant(_11.3.0, 1);
_11.3.2.0 = !_11.0.0;
place!(Field::<i128>(Variant(_11.3.0, 1), 0)) = Field::<i128>(Variant(_9, 1), 0);
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = _11.0.0 as i16;
_12.1 = Move(_14.fld6);
_14.fld6.fld1 = [_11.3.2.0,_11.3.2.0,_11.3.2.0];
_14.fld2.0 = !2_i8;
_12.1.fld0 = Field::<i128>(Variant(_9, 1), 0);
_11.1 = [(-1324248622_i32),1957956298_i32,(-1113180735_i32),(-1998145257_i32)];
place!(Field::<i16>(Variant(_9, 1), 1)) = _12.1.fld2;
_13 = Field::<i16>(Variant(_9, 1), 1) as isize;
_14.fld4 = [_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0];
Call(_14.fld1 = fn3(Field::<bool>(Variant(_11.3.3, 1), 0), Field::<i128>(Variant(_11.3.0, 1), 0), Field::<i128>(Variant(_11.3.0, 1), 0), _6, _11.3.2.1, _11.0.1, Move(_12.1), Field::<i16>(Variant(_9, 1), 1), _11.3.2.1, _11.0.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<u32>(Variant(_11.3.0, 1), 3)) = Field::<u32>(Variant(_9, 1), 3) | Field::<u32>(Variant(_9, 1), 3);
_9 = Adt21::Variant1 { fld0: Field::<i128>(Variant(_11.3.0, 1), 0),fld1: Field::<i16>(Variant(_11.3.3, 1), 2),fld2: _13,fld3: Field::<u32>(Variant(_11.3.0, 1), 3) };
place!(Field::<bool>(Variant(_11.3.3, 1), 0)) = !_4;
_3 = _6;
_11.0.0 = _14.fld2.0 as u16;
place!(Field::<i128>(Variant(_11.3.0, 1), 0)) = _7 as i128;
_11.3.0 = Adt21::Variant0 { fld0: _10,fld1: _2 };
_14.fld4 = [_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0];
_10 = _6;
_12.1.fld0 = _11.3.2.0 as i128;
_11.2 = Field::<bool>(Variant(_11.3.3, 1), 0) as u128;
_11.3.1.0 = &_20;
place!(Field::<u32>(Variant(_9, 1), 3)) = Field::<u32>(Variant(_11.3.3, 1), 1);
_12.1.fld1 = [_11.3.2.0,_11.0.0,_11.0.0];
RET.0 = core::ptr::addr_of_mut!(_21);
place!(Field::<bool>(Variant(_11.3.3, 1), 0)) = Field::<bool>(Variant(_11.3.0, 0), 0) < _6;
_14.fld6.fld1 = _12.1.fld1;
_11.0.1 = _14.fld1;
Call(RET = fn6(Move(_11.3.1), _9, Field::<isize>(Variant(_9, 1), 2), _11.0.1, _11.3.2, _11.0, _11.0, _11.0, _14.fld1, _14.fld1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14.fld3 = core::ptr::addr_of!(_20);
place!(Field::<i32>(Variant(_11.3.3, 1), 3)) = (-1993937374_i32) >> _11.0.2;
SetDiscriminant(_11.3.3, 1);
_11.3.1.0 = &_20;
match _8 {
0 => bb8,
1 => bb10,
2 => bb11,
3 => bb12,
40960208951847840882841069746265912778 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
SetDiscriminant(_11.3.0, 1);
_11.3.2.0 = !_11.0.0;
place!(Field::<i128>(Variant(_11.3.0, 1), 0)) = Field::<i128>(Variant(_9, 1), 0);
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = _11.0.0 as i16;
_12.1 = Move(_14.fld6);
_14.fld6.fld1 = [_11.3.2.0,_11.3.2.0,_11.3.2.0];
_14.fld2.0 = !2_i8;
_12.1.fld0 = Field::<i128>(Variant(_9, 1), 0);
_11.1 = [(-1324248622_i32),1957956298_i32,(-1113180735_i32),(-1998145257_i32)];
place!(Field::<i16>(Variant(_9, 1), 1)) = _12.1.fld2;
_13 = Field::<i16>(Variant(_9, 1), 1) as isize;
_14.fld4 = [_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0];
Call(_14.fld1 = fn3(Field::<bool>(Variant(_11.3.3, 1), 0), Field::<i128>(Variant(_11.3.0, 1), 0), Field::<i128>(Variant(_11.3.0, 1), 0), _6, _11.3.2.1, _11.0.1, Move(_12.1), Field::<i16>(Variant(_9, 1), 1), _11.3.2.1, _11.0.1), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
_11.3.2.2 = !_8;
_11.0.0 = 53228_u16;
_11.3.2.1 = [(-8658407651134569116_i64),(-8947514897247983672_i64),(-6170157434487326492_i64),(-2174838817404008687_i64),5767179106042535939_i64,4016032728513050312_i64,6018807237564272649_i64];
_2 = [_11.0.0,_11.0.0,_11.0.0,_11.0.0,_11.0.0,_11.0.0,_11.0.0];
_11.0.1 = [7390210449889938493_i64,(-8827018636862078556_i64),(-121862161060212656_i64),1411162896924222896_i64,(-5142980615336155903_i64),8545052356193470025_i64,(-3998129339119262463_i64)];
place!(Field::<i16>(Variant(_9, 1), 1)) = !(-26789_i16);
_11.3.3 = Adt42::Variant1 { fld0: _6,fld1: 1736649727_u32,fld2: Field::<i16>(Variant(_9, 1), 1),fld3: 9800228_i32 };
_11.3.2.0 = !_11.0.0;
_10 = !_6;
_11.3.2.0 = _5 as u16;
_5 = '\u{42363}';
_11.3.1.1 = _5;
_11.3.0 = Adt21::Variant0 { fld0: _6,fld1: _2 };
_11.0 = (_11.3.2.0, _11.3.2.1, _11.3.2.2);
_12.1.fld1 = [_11.0.0,_11.0.0,_11.0.0];
place!(Field::<i16>(Variant(_9, 1), 1)) = !Field::<i16>(Variant(_11.3.3, 1), 2);
place!(Field::<u32>(Variant(_9, 1), 3)) = 3262593265_u32 + 4283834151_u32;
_13 = -(-83_isize);
place!(Field::<u32>(Variant(_11.3.3, 1), 1)) = !Field::<u32>(Variant(_9, 1), 3);
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = Field::<i16>(Variant(_9, 1), 1) >> Field::<i128>(Variant(_9, 1), 0);
place!(Field::<[u16; 7]>(Variant(_11.3.0, 0), 1)) = _1;
_1 = [_11.0.0,_11.0.0,_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.0.0];
_14.fld6 = Adt36 { fld0: Field::<i128>(Variant(_9, 1), 0),fld1: _12.1.fld1,fld2: Field::<i16>(Variant(_11.3.3, 1), 2) };
_11.1 = [1321895462_i32,1480268369_i32,(-2021686904_i32),1915848073_i32];
place!(Field::<i16>(Variant(_9, 1), 1)) = !_14.fld6.fld2;
_5 = _11.3.1.1;
_14.fld2.0 = 12_i8 << _13;
Goto(bb7)
}
bb13 = {
Return()
}
bb14 = {
SetDiscriminant(_11.3.0, 0);
place!(Field::<bool>(Variant(_11.3.3, 1), 0)) = _4;
place!(Field::<u32>(Variant(_11.3.3, 1), 1)) = !Field::<u32>(Variant(_9, 1), 3);
_11.3.2 = (_11.0.0, _11.0.1, _11.2);
_11.3.2.0 = _11.0.0 >> Field::<u32>(Variant(_11.3.3, 1), 1);
SetDiscriminant(_9, 1);
_14.fld2.0 = 1049300610_i32 as i8;
place!(Field::<u32>(Variant(_9, 1), 3)) = Field::<u32>(Variant(_11.3.3, 1), 1) << _11.3.2.2;
_19 = [_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0];
_9 = Adt21::Variant1 { fld0: _12.1.fld0,fld1: (-32581_i16),fld2: _13,fld3: Field::<u32>(Variant(_11.3.3, 1), 1) };
_7 = _6;
_18 = _8 * _11.2;
_14.fld6 = Adt36 { fld0: Field::<i128>(Variant(_9, 1), 0),fld1: _12.1.fld1,fld2: (-4633_i16) };
_14.fld2.0 = (-5_i8);
_1 = [_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.3.2.0,_11.0.0,_11.3.2.0];
_19 = [_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0,_14.fld2.0];
_14.fld6.fld2 = 5_usize as i16;
_15 = _14.fld6.fld2 | _14.fld6.fld2;
_20 = !18433335131237015303_usize;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(2_usize, 19_usize, Move(_19), 2_usize, Move(_2), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(2_usize, 5_usize, Move(_5), 13_usize, Move(_13), 4_usize, Move(_4), 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: bool,mut _2: i128,mut _3: i128,mut _4: bool,mut _5: [i64; 7],mut _6: [i64; 7],mut _7: Adt36,mut _8: i16,mut _9: [i64; 7],mut _10: [i64; 7]) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _11: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _12: f64;
let _13: isize;
let _14: bool;
let _15: i64;
let _16: i64;
let _17: i32;
let _18: f64;
let _19: usize;
let _20: [usize; 8];
let _21: u128;
let _22: char;
let _23: char;
let _24: isize;
let _25: &'static usize;
let _26: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _27: (&'static usize, char);
let _28: isize;
let _29: ();
let _30: ();
{
_7.fld2 = _8;
_4 = _1;
_10 = [(-4841673816091158423_i64),(-9032519270343746734_i64),(-4814465990391037374_i64),741532094647319638_i64,(-2341129070131124267_i64),(-5166216883659361120_i64),1061721915806215347_i64];
RET = [(-1873776429740181812_i64),3875714025109755695_i64,4073828203883006460_i64,(-2034456808477213899_i64),2441812601448127196_i64,78048118406262104_i64,(-7227397778200623713_i64)];
_1 = _2 < _3;
_8 = _7.fld2 ^ _7.fld2;
_3 = !_2;
_8 = _7.fld2 | _7.fld2;
_4 = !_1;
_8 = !_7.fld2;
_5 = [10586095507619860_i64,(-4227510567123168537_i64),1489941456899463738_i64,7819781622808327540_i64,7015133453008899464_i64,(-4022471548201562959_i64),(-1639354889828938764_i64)];
_7.fld2 = _8 | _8;
_6 = [(-5248598262687926147_i64),(-3940254199202247063_i64),(-3396306574742151300_i64),(-2698045328992327937_i64),(-5331218316134926193_i64),(-9124796722042583563_i64),(-4781888127714336717_i64)];
_8 = -_7.fld2;
_7.fld1 = [39445_u16,56359_u16,54105_u16];
_3 = _7.fld0;
_11.0 = (56320_u16, RET, 287294491083944623977881924036261561878_u128);
_11.3.2.2 = _11.0.2 & _11.0.2;
_11.3.1.1 = '\u{105644}';
Call(_3 = core::intrinsics::transmute(_11.3.2.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _11.0.1;
_11.2 = !_11.3.2.2;
_11.0 = (65032_u16, RET, _11.2);
_11.3.2.1 = RET;
_7.fld0 = !_3;
_11.0 = (19791_u16, RET, _11.3.2.2);
_7.fld1 = [_11.0.0,_11.0.0,_11.0.0];
_11.1 = [394586518_i32,(-237849511_i32),96579716_i32,2006860754_i32];
_11.0.0 = 45357_u16;
_12 = 112_i8 as f64;
_11.3.1.1 = '\u{b7394}';
_2 = _3;
_2 = _7.fld0 << _7.fld0;
Call(_11.3.1.1 = fn4(_11.0.2, _1, _5, _12, _11.0.2, _11.3.2.2, _11.0.1, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = _11.3.2.1;
_12 = 1029231976_u32 as f64;
_11.3.2 = (_11.0.0, _6, _11.2);
_11.2 = _11.3.2.2 & _11.3.2.2;
_11.3.2.1 = [(-5129422919598927540_i64),(-1442533571614998503_i64),5290653186912881230_i64,8615801643465368416_i64,(-1730882344711848890_i64),(-3771752409428957331_i64),1503271700458210862_i64];
_8 = _11.2 as i16;
_11.3.2.2 = _11.0.2 << _7.fld0;
_7.fld0 = _3 ^ _3;
RET = [2300038632479992052_i64,(-8211681105113786763_i64),(-3119033586936251708_i64),845245942915814011_i64,4220887846868975167_i64,(-5147527672411093074_i64),558422964605464869_i64];
_11.0.2 = _11.3.2.2 ^ _11.2;
_11.3.2.2 = !_11.0.2;
_10 = [5146753866670599111_i64,(-2351704333417013891_i64),2930685407947993388_i64,(-1442816185015385302_i64),8058672936624337036_i64,2545808704605310943_i64,7640157210547918662_i64];
_7.fld0 = -_2;
RET = _10;
_11.3.3 = Adt42::Variant1 { fld0: _4,fld1: 1048743829_u32,fld2: _8,fld3: 1031141599_i32 };
_4 = _1;
_11.3.1.1 = '\u{fcb4b}';
place!(Field::<u32>(Variant(_11.3.3, 1), 1)) = 537943663_u32 >> _8;
match _11.0.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
45357 => bb9,
_ => bb8
}
}
bb3 = {
_9 = _11.0.1;
_11.2 = !_11.3.2.2;
_11.0 = (65032_u16, RET, _11.2);
_11.3.2.1 = RET;
_7.fld0 = !_3;
_11.0 = (19791_u16, RET, _11.3.2.2);
_7.fld1 = [_11.0.0,_11.0.0,_11.0.0];
_11.1 = [394586518_i32,(-237849511_i32),96579716_i32,2006860754_i32];
_11.0.0 = 45357_u16;
_12 = 112_i8 as f64;
_11.3.1.1 = '\u{b7394}';
_2 = _3;
_2 = _7.fld0 << _7.fld0;
Call(_11.3.1.1 = fn4(_11.0.2, _1, _5, _12, _11.0.2, _11.3.2.2, _11.0.1, _9), ReturnTo(bb2), UnwindUnreachable())
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
_11.0.1 = _9;
_11.3.2.0 = _2 as u16;
_11.0 = (_11.3.2.0, RET, _11.2);
_4 = _8 > Field::<i16>(Variant(_11.3.3, 1), 2);
_12 = (-4823553860528304850_i64) as f64;
_17 = !813899729_i32;
_11.3.1.1 = '\u{321bd}';
place!(Field::<i32>(Variant(_11.3.3, 1), 3)) = -_17;
_17 = Field::<i32>(Variant(_11.3.3, 1), 3) << _7.fld0;
_7.fld2 = !_8;
_9 = RET;
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = _8 ^ _8;
place!(Field::<bool>(Variant(_11.3.3, 1), 0)) = _2 <= _7.fld0;
_16 = (-2870984471066745511_i64);
_3 = (-9223372036854775808_isize) as i128;
Goto(bb10)
}
bb10 = {
_1 = Field::<bool>(Variant(_11.3.3, 1), 0) ^ Field::<bool>(Variant(_11.3.3, 1), 0);
_14 = !_4;
_11.3.0 = Adt21::Variant1 { fld0: _7.fld0,fld1: Field::<i16>(Variant(_11.3.3, 1), 2),fld2: (-9223372036854775808_isize),fld3: Field::<u32>(Variant(_11.3.3, 1), 1) };
_17 = Field::<i32>(Variant(_11.3.3, 1), 3);
_16 = (-8806300905111051488_i64);
_5 = [_16,_16,_16,_16,_16,_16,_16];
_15 = !_16;
match _16 {
0 => bb4,
1 => bb5,
2 => bb8,
340282366920938463454568306526657159968 => bb12,
_ => bb11
}
}
bb11 = {
_11.0.1 = _9;
_11.3.2.0 = _2 as u16;
_11.0 = (_11.3.2.0, RET, _11.2);
_4 = _8 > Field::<i16>(Variant(_11.3.3, 1), 2);
_12 = (-4823553860528304850_i64) as f64;
_17 = !813899729_i32;
_11.3.1.1 = '\u{321bd}';
place!(Field::<i32>(Variant(_11.3.3, 1), 3)) = -_17;
_17 = Field::<i32>(Variant(_11.3.3, 1), 3) << _7.fld0;
_7.fld2 = !_8;
_9 = RET;
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = _8 ^ _8;
place!(Field::<bool>(Variant(_11.3.3, 1), 0)) = _2 <= _7.fld0;
_16 = (-2870984471066745511_i64);
_3 = (-9223372036854775808_isize) as i128;
Goto(bb10)
}
bb12 = {
_15 = _11.3.2.2 as i64;
_11.3.2.0 = 85_i8 as u16;
_11.0.1 = [_15,_15,_15,_15,_15,_15,_15];
_2 = (-88_i8) as i128;
_19 = !10713062364971000470_usize;
place!(Field::<i16>(Variant(_11.3.3, 1), 2)) = Field::<i128>(Variant(_11.3.0, 1), 0) as i16;
_16 = Field::<u32>(Variant(_11.3.0, 1), 3) as i64;
_11.3.1.1 = '\u{e9049}';
_10 = [_15,_16,_15,_15,_16,_15,_15];
_3 = _11.0.2 as i128;
_11.3.2 = (_11.0.0, _10, _11.2);
_11.0.1 = _10;
_6 = [_15,_15,_15,_15,_15,_16,_16];
place!(Field::<i128>(Variant(_11.3.0, 1), 0)) = _7.fld0;
place!(Field::<i16>(Variant(_11.3.0, 1), 1)) = _8;
_5 = [_15,_16,_16,_16,_15,_15,_15];
_6 = _10;
_15 = -_16;
_5 = [_16,_15,_15,_16,_16,_16,_15];
place!(Field::<bool>(Variant(_11.3.3, 1), 0)) = !_14;
_15 = Field::<i128>(Variant(_11.3.0, 1), 0) as i64;
_11.3.0 = Adt21::Variant1 { fld0: _3,fld1: _7.fld2,fld2: (-9223372036854775808_isize),fld3: Field::<u32>(Variant(_11.3.3, 1), 1) };
_16 = !_15;
_11.3.1.0 = &_19;
_17 = _7.fld0 as i32;
_5 = _11.3.2.1;
_19 = 17884981999492256646_usize;
_4 = !_1;
Call(RET = core::intrinsics::transmute(_10), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18 = _12;
RET = _11.3.2.1;
_14 = _1;
_11.3.1.0 = &_19;
_2 = !_7.fld0;
_21 = Field::<u32>(Variant(_11.3.3, 1), 1) as u128;
place!(Field::<u32>(Variant(_11.3.3, 1), 1)) = Field::<u32>(Variant(_11.3.0, 1), 3) | Field::<u32>(Variant(_11.3.0, 1), 3);
_13 = -(-114_isize);
place!(Field::<isize>(Variant(_11.3.0, 1), 2)) = !_13;
_11.0.1 = [_16,_16,_15,_16,_16,_15,_16];
_26.3.2.0 = _11.3.2.0 << _21;
_26.3.1.0 = &_19;
_26.3.2 = (_11.3.2.0, RET, _21);
_1 = _14 ^ _14;
_26.0.2 = _11.0.2 ^ _11.2;
_26.0.2 = !_11.0.2;
_10 = _26.3.2.1;
_11.3.0 = Adt21::Variant1 { fld0: _2,fld1: _7.fld2,fld2: _13,fld3: Field::<u32>(Variant(_11.3.3, 1), 1) };
_11.3.1.1 = '\u{92b5d}';
_20 = [_19,_19,_19,_19,_19,_19,_19,_19];
_23 = _11.3.1.1;
_25 = &_19;
place!(Field::<isize>(Variant(_11.3.0, 1), 2)) = _13;
Goto(bb14)
}
bb14 = {
Call(_29 = dump_var(3_usize, 4_usize, Move(_4), 21_usize, Move(_21), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_29 = dump_var(3_usize, 15_usize, Move(_15), 20_usize, Move(_20), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(3_usize, 10_usize, Move(_10), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u128,mut _2: bool,mut _3: [i64; 7],mut _4: f64,mut _5: u128,mut _6: u128,mut _7: [i64; 7],mut _8: [i64; 7]) -> char {
mir! {
type RET = char;
let _9: i32;
let _10: f32;
let _11: Adt55;
let _12: (*const i64, Adt36);
let _13: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _14: isize;
let _15: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _16: [u16; 3];
let _17: isize;
let _18: isize;
let _19: &'static u8;
let _20: *const i64;
let _21: f64;
let _22: Adt55;
let _23: Adt44;
let _24: [i32; 4];
let _25: bool;
let _26: u16;
let _27: isize;
let _28: Adt36;
let _29: f64;
let _30: u64;
let _31: &'static [u8; 8];
let _32: ((u16, [i64; 7], u128),);
let _33: isize;
let _34: i32;
let _35: f32;
let _36: char;
let _37: isize;
let _38: &'static [char; 2];
let _39: isize;
let _40: i16;
let _41: (f32, u64, [i64; 7]);
let _42: (*const i64, Adt36);
let _43: f64;
let _44: f64;
let _45: ();
let _46: ();
{
_6 = _1 >> _1;
_6 = _1;
_1 = _6 ^ _6;
_8 = _3;
_7 = _3;
_9 = !(-219477107_i32);
_3 = _7;
RET = '\u{df97b}';
_4 = 4_usize as f64;
_8 = _7;
_5 = !_6;
_8 = [(-7924644889073717772_i64),(-5974645584830096775_i64),(-1231579173069590675_i64),(-10600464525999203_i64),(-5970997464500471077_i64),1080816027052742292_i64,(-503567445407945858_i64)];
_3 = [(-8292426499547547559_i64),(-1002402726727183922_i64),(-6071156348420379066_i64),7891796864097594676_i64,(-6779470548560716228_i64),1168847820960249340_i64,5013601892608686844_i64];
_9 = 112313663_i32 + (-1389655684_i32);
_2 = true;
_8 = _3;
_6 = !_5;
Goto(bb1)
}
bb1 = {
_10 = 4881639491338195253_i64 as f32;
_10 = 6597_i16 as f32;
_2 = !false;
Goto(bb2)
}
bb2 = {
_6 = _1;
_1 = _6;
_2 = false;
_10 = 220_u8 as f32;
_8 = [4659079779062769690_i64,1136722221844112123_i64,(-5603602935251882102_i64),3465393987331322152_i64,7935355863788590471_i64,6802976602647677761_i64,6392520007142008523_i64];
_4 = (-9223372036854775808_isize) as f64;
_1 = _6;
_1 = _6;
Goto(bb3)
}
bb3 = {
_4 = 16769236187475588599_u64 as f64;
_1 = _6 * _6;
_8 = _7;
_3 = _7;
_5 = _1;
RET = '\u{c1922}';
_13.2 = _9 as u128;
_13.2 = _1 & _5;
_12.1.fld0 = !(-55578421702539551461841324422532709094_i128);
_13.3.2.1 = [2679522772697587781_i64,(-1879720024368495084_i64),728945434393011848_i64,1409230637795790103_i64,6917572141455352622_i64,(-9153293343720515357_i64),(-6641956458924738313_i64)];
_13.3.2.2 = _1 | _13.2;
_13.3.2 = (25342_u16, _7, _5);
RET = '\u{554a1}';
_13.0 = (_13.3.2.0, _8, _1);
_13.3.2.0 = _13.0.0 >> _13.0.0;
_15.3.2 = (_13.3.2.0, _13.3.2.1, _6);
_13.1 = [_9,_9,_9,_9];
_4 = _9 as f64;
_15.0.1 = [(-3606112979158511518_i64),3784852380659053790_i64,2696771666232068183_i64,4348385933909224948_i64,8518721407721859231_i64,4943945642587531960_i64,3654031405916281481_i64];
_12.1.fld1 = [_15.3.2.0,_13.3.2.0,_15.3.2.0];
_12.1.fld0 = 142232132797054284731975955089039778049_i128 & 123758410403443523246642524862364670877_i128;
_13.3.2.2 = _15.3.2.2;
Call(_15.0.2 = core::intrinsics::transmute(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10 = 8084270672814008919_usize as f32;
_15.3.2 = (_13.3.2.0, _3, _1);
_15.3.3 = Adt42::Variant1 { fld0: _2,fld1: 2769913954_u32,fld2: 9264_i16,fld3: _9 };
_15.2 = _5;
_13.3.2 = (_13.0.0, _15.3.2.1, _1);
_15.0.0 = 467785816309092480_u64 as u16;
_12.1.fld1 = [_13.3.2.0,_13.0.0,_15.3.2.0];
_9 = Field::<i32>(Variant(_15.3.3, 1), 3);
place!(Field::<u32>(Variant(_15.3.3, 1), 1)) = 1458347602_u32 << _13.0.2;
place!(Field::<i32>(Variant(_15.3.3, 1), 3)) = _9;
_12.1.fld1 = [_13.0.0,_13.0.0,_13.0.0];
_15.1 = [_9,_9,_9,_9];
place!(Field::<i32>(Variant(_15.3.3, 1), 3)) = _9 + _9;
_9 = !Field::<i32>(Variant(_15.3.3, 1), 3);
_15.2 = 9223372036854775807_isize as u128;
_4 = (-32_i8) as f64;
_13.0.2 = 22_u8 as u128;
_12.1.fld1 = [_13.0.0,_15.3.2.0,_13.3.2.0];
_21 = _4;
_12.1.fld2 = 29889_i16;
_15.1 = [_9,Field::<i32>(Variant(_15.3.3, 1), 3),Field::<i32>(Variant(_15.3.3, 1), 3),Field::<i32>(Variant(_15.3.3, 1), 3)];
_15.3.1.1 = RET;
_5 = _13.3.2.2;
_18 = 5087639375048777210_u64 as isize;
Call(place!(Field::<i32>(Variant(_15.3.3, 1), 3)) = fn5(_13.3.2, _15.0.2, _13.3.2.0, _13.3.2.2, Field::<u32>(Variant(_15.3.3, 1), 1), _12.1.fld1, _8, _13.3.2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = !Field::<i32>(Variant(_15.3.3, 1), 3);
_15.3.1.1 = RET;
_13.3.2.1 = [(-5608079815600508390_i64),3336611839581925830_i64,3833075834823178984_i64,(-5536737037801654845_i64),(-229881778094790868_i64),5622417700200247285_i64,(-1059442178393904625_i64)];
_10 = _12.1.fld0 as f32;
_17 = _18;
place!(Field::<i16>(Variant(_15.3.3, 1), 2)) = -_12.1.fld2;
_15.3.2.0 = !_13.3.2.0;
_15.0 = (_13.0.0, _8, _13.2);
_16 = _12.1.fld1;
_13.3.3 = Move(_15.3.3);
_13.3.2.0 = _15.0.0;
_12.1.fld0 = !125040941128755686608863665724890416785_i128;
_15.0 = _13.3.2;
match _13.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
25342 => bb8,
_ => bb7
}
}
bb6 = {
_6 = _1;
_1 = _6;
_2 = false;
_10 = 220_u8 as f32;
_8 = [4659079779062769690_i64,1136722221844112123_i64,(-5603602935251882102_i64),3465393987331322152_i64,7935355863788590471_i64,6802976602647677761_i64,6392520007142008523_i64];
_4 = (-9223372036854775808_isize) as f64;
_1 = _6;
_1 = _6;
Goto(bb3)
}
bb7 = {
_4 = 16769236187475588599_u64 as f64;
_1 = _6 * _6;
_8 = _7;
_3 = _7;
_5 = _1;
RET = '\u{c1922}';
_13.2 = _9 as u128;
_13.2 = _1 & _5;
_12.1.fld0 = !(-55578421702539551461841324422532709094_i128);
_13.3.2.1 = [2679522772697587781_i64,(-1879720024368495084_i64),728945434393011848_i64,1409230637795790103_i64,6917572141455352622_i64,(-9153293343720515357_i64),(-6641956458924738313_i64)];
_13.3.2.2 = _1 | _13.2;
_13.3.2 = (25342_u16, _7, _5);
RET = '\u{554a1}';
_13.0 = (_13.3.2.0, _8, _1);
_13.3.2.0 = _13.0.0 >> _13.0.0;
_15.3.2 = (_13.3.2.0, _13.3.2.1, _6);
_13.1 = [_9,_9,_9,_9];
_4 = _9 as f64;
_15.0.1 = [(-3606112979158511518_i64),3784852380659053790_i64,2696771666232068183_i64,4348385933909224948_i64,8518721407721859231_i64,4943945642587531960_i64,3654031405916281481_i64];
_12.1.fld1 = [_15.3.2.0,_13.3.2.0,_15.3.2.0];
_12.1.fld0 = 142232132797054284731975955089039778049_i128 & 123758410403443523246642524862364670877_i128;
_13.3.2.2 = _15.3.2.2;
Call(_15.0.2 = core::intrinsics::transmute(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_15.0 = _15.3.2;
place!(Field::<i32>(Variant(_13.3.3, 1), 3)) = _4 as i32;
_15.1 = _13.1;
_28 = Adt36 { fld0: _12.1.fld0,fld1: _12.1.fld1,fld2: Field::<i16>(Variant(_13.3.3, 1), 2) };
_15.0.2 = _1 * _13.2;
_26 = _15.0.0;
SetDiscriminant(_13.3.3, 3);
place!(Field::<*mut [u8; 7]>(Variant(_13.3.3, 3), 3)) = core::ptr::addr_of_mut!(place!(Field::<[u8; 7]>(Variant(_13.3.3, 3), 2)));
_15.0 = (_15.3.2.0, _13.3.2.1, _6);
_15.2 = !_1;
_29 = _28.fld2 as f64;
_3 = _13.3.2.1;
_15.3.2.2 = _29 as u128;
place!(Field::<[i8; 5]>(Variant(_13.3.3, 3), 4)) = [(-7_i8),55_i8,0_i8,(-15_i8),(-63_i8)];
_23 = Adt44::Variant0 { fld0: _3 };
_3 = _15.3.2.1;
place!(Field::<[u8; 7]>(Variant(_13.3.3, 3), 2)) = [91_u8,4_u8,121_u8,87_u8,4_u8,138_u8,62_u8];
_6 = _13.3.2.2;
_15.3.0 = Adt21::Variant1 { fld0: _28.fld0,fld1: _12.1.fld2,fld2: _17,fld3: 3733020954_u32 };
_32.0.0 = _13.0.0 / _13.0.0;
place!(Field::<i32>(Variant(_13.3.3, 3), 5)) = -_9;
place!(Field::<[isize; 2]>(Variant(_13.3.3, 3), 0)) = [_18,Field::<isize>(Variant(_15.3.0, 1), 2)];
match _13.3.2.0 {
0 => bb1,
1 => bb6,
25342 => bb9,
_ => bb4
}
}
bb9 = {
_15.0 = (_13.3.2.0, _15.3.2.1, _13.2);
RET = _15.3.1.1;
_12.1.fld2 = _28.fld2 * Field::<i16>(Variant(_15.3.0, 1), 1);
_25 = _2;
_15.3.0 = Adt21::Variant1 { fld0: _28.fld0,fld1: _28.fld2,fld2: _18,fld3: 3068684449_u32 };
place!(Field::<i16>(Variant(_15.3.0, 1), 1)) = !_28.fld2;
_28.fld2 = Field::<i16>(Variant(_15.3.0, 1), 1);
_13.1 = [_9,Field::<i32>(Variant(_13.3.3, 3), 5),_9,_9];
_24 = _13.1;
_14 = Field::<isize>(Variant(_15.3.0, 1), 2) - _18;
_8 = [766867405406934821_i64,(-9075042791340888383_i64),5799191803499616142_i64,(-1437319035409905694_i64),473233118589291994_i64,(-6062475568007930219_i64),6818253918804208102_i64];
SetDiscriminant(_23, 0);
_28.fld0 = _12.1.fld0;
place!(Field::<i32>(Variant(_13.3.3, 3), 5)) = _21 as i32;
_32 = (_15.0,);
_24 = [_9,_9,_9,_9];
Goto(bb10)
}
bb10 = {
_12.1.fld0 = _9 as i128;
match _13.3.2.0 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb7,
25342 => bb12,
_ => bb11
}
}
bb11 = {
_6 = _1;
_1 = _6;
_2 = false;
_10 = 220_u8 as f32;
_8 = [4659079779062769690_i64,1136722221844112123_i64,(-5603602935251882102_i64),3465393987331322152_i64,7935355863788590471_i64,6802976602647677761_i64,6392520007142008523_i64];
_4 = (-9223372036854775808_isize) as f64;
_1 = _6;
_1 = _6;
Goto(bb3)
}
bb12 = {
_13.2 = _15.2 << _6;
_25 = !_2;
_30 = _14 as u64;
_12.1.fld1 = [_15.0.0,_26,_15.0.0];
_32 = (_13.3.2,);
_15.1 = [_9,_9,_9,_9];
_13.3.2.2 = _6;
place!(Field::<[i64; 7]>(Variant(_23, 0), 0)) = [(-1262831490822312663_i64),8756606637389330597_i64,(-4926082728397363447_i64),6673646152943366150_i64,3704502388915372668_i64,7900189689193123663_i64,2918239589712126383_i64];
_30 = 7601848366830435725_u64 & 8315717964898140946_u64;
_12.1.fld0 = Field::<i128>(Variant(_15.3.0, 1), 0) * Field::<i128>(Variant(_15.3.0, 1), 0);
place!(Field::<i16>(Variant(_15.3.0, 1), 1)) = _12.1.fld2 ^ _28.fld2;
_39 = -_17;
_15.0.1 = [(-8943535311130907505_i64),(-6315562191461787062_i64),4129843264606610119_i64,(-2599495689487977281_i64),7102492270556719039_i64,(-668597991378926072_i64),8078380237924924904_i64];
_12.1 = Adt36 { fld0: Field::<i128>(Variant(_15.3.0, 1), 0),fld1: _28.fld1,fld2: Field::<i16>(Variant(_15.3.0, 1), 1) };
_13.2 = !_32.0.2;
_15.3.2.2 = _13.2 - _5;
_3 = _15.0.1;
_33 = _18;
_27 = _33 & _14;
_2 = _25;
_13.3.2.2 = RET as u128;
_34 = _9 >> _26;
_13.3.0 = Adt21::Variant1 { fld0: Field::<i128>(Variant(_15.3.0, 1), 0),fld1: _12.1.fld2,fld2: _27,fld3: 3878991569_u32 };
_6 = RET as u128;
place!(Field::<u32>(Variant(_15.3.0, 1), 3)) = _29 as u32;
Goto(bb13)
}
bb13 = {
_28.fld2 = Field::<i16>(Variant(_13.3.0, 1), 1) - _12.1.fld2;
_13.3.1.1 = RET;
_34 = _9 * _9;
_13.3.2 = (_26, _7, _32.0.2);
_32.0.1 = [4060887117177579473_i64,9102415208335328327_i64,(-6714541633612967166_i64),5737392785884691754_i64,(-3070476513602775782_i64),8663911963616730368_i64,(-4252702733464417878_i64)];
_13.0 = (_13.3.2.0, _32.0.1, _1);
_13.3.2.1 = _7;
place!(Field::<i16>(Variant(_15.3.0, 1), 1)) = _28.fld2 ^ _28.fld2;
RET = _15.3.1.1;
_41 = (_10, _30, _3);
_5 = _15.3.2.0 as u128;
place!(Field::<u32>(Variant(_13.3.0, 1), 3)) = Field::<u32>(Variant(_15.3.0, 1), 3);
_36 = RET;
_28.fld0 = -_12.1.fld0;
Goto(bb14)
}
bb14 = {
_26 = _13.3.2.0;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(4_usize, 18_usize, Move(_18), 39_usize, Move(_39), 30_usize, Move(_30), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(4_usize, 7_usize, Move(_7), 9_usize, Move(_9), 34_usize, Move(_34), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(4_usize, 17_usize, Move(_17), 27_usize, Move(_27), 32_usize, Move(_32), 46_usize, _46), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (u16, [i64; 7], u128),mut _2: u128,mut _3: u16,mut _4: u128,mut _5: u32,mut _6: [u16; 3],mut _7: [i64; 7],mut _8: u16) -> i32 {
mir! {
type RET = i32;
let _9: Adt30;
let _10: isize;
let _11: &'static &'static [char; 2];
let _12: bool;
let _13: &'static (i8,);
let _14: u16;
let _15: &'static &'static i32;
let _16: f32;
let _17: u64;
let _18: ();
let _19: ();
{
_4 = !_1.2;
_5 = (-12545977765934771999018429216785515826_i128) as u32;
RET = 766843562_i32;
_1.0 = 4958866242352538116_u64 as u16;
_1 = (_3, _7, _2);
_1.1 = [(-1559927745233581356_i64),(-4762714527465586697_i64),2171400857596317740_i64,(-1771103379228647029_i64),8362357210868508301_i64,(-790456049689325264_i64),(-6856862697632253945_i64)];
_2 = 6_usize as u128;
_1.2 = _4 | _4;
_6 = [_8,_1.0,_3];
RET = -(-971508400_i32);
_4 = _1.2 ^ _1.2;
_5 = 2711271772_u32 & 1738037642_u32;
_10 = (-112_isize) | 9223372036854775807_isize;
_3 = _8;
RET = (-1526076747_i32) ^ 575539486_i32;
_7 = _1.1;
_4 = !_1.2;
_14 = !_8;
_4 = _1.2;
Call(_1.2 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.2 = _4 >> _1.0;
_8 = !_14;
RET = 1699051910_i32 << _14;
_1.2 = !_4;
_5 = 3273280780_u32 + 3164801388_u32;
_12 = true;
_1 = (_8, _7, _4);
_5 = 0_usize as u32;
RET = (-1333501134_i32);
_14 = _10 as u16;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
25342 => bb7,
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
_12 = !false;
_16 = _8 as f32;
_8 = _1.0;
_16 = (-104_i8) as f32;
_14 = _3;
_16 = (-34460181499740012369593783700182130856_i128) as f32;
_7 = [(-711300600492274620_i64),9159052671093843085_i64,796930480750963950_i64,(-3865067849908788286_i64),3077295149016775792_i64,(-8317160342948185248_i64),(-8403602580576654669_i64)];
_6 = [_3,_8,_3];
RET = (-1846963200_i32) << _1.2;
_1 = (_8, _7, _4);
_7 = _1.1;
_2 = _4;
_1.2 = _16 as u128;
_3 = RET as u16;
_2 = _4;
Goto(bb8)
}
bb8 = {
Call(_18 = dump_var(5_usize, 5_usize, Move(_5), 12_usize, Move(_12), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_18 = dump_var(5_usize, 14_usize, Move(_14), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: (&'static usize, char),mut _2: Adt21,mut _3: isize,mut _4: [i64; 7],mut _5: (u16, [i64; 7], u128),mut _6: (u16, [i64; 7], u128),mut _7: (u16, [i64; 7], u128),mut _8: (u16, [i64; 7], u128),mut _9: [i64; 7],mut _10: [i64; 7]) -> (*mut [u8; 7],) {
mir! {
type RET = (*mut [u8; 7],);
let _11: &'static usize;
let _12: i64;
let _13: Adt51;
let _14: u32;
let _15: *mut [u8; 7];
let _16: &'static &'static usize;
let _17: *const *const f32;
let _18: isize;
let _19: isize;
let _20: char;
let _21: *const i128;
let _22: isize;
let _23: Adt30;
let _24: i64;
let _25: &'static (i8,);
let _26: f64;
let _27: isize;
let _28: (*const i64, Adt36);
let _29: &'static u8;
let _30: (i64, i16, i8, [u16; 7]);
let _31: Adt51;
let _32: ();
let _33: ();
{
_7 = _6;
_5.2 = _7.2;
_9 = [5376267302942013486_i64,(-1417507395519954342_i64),7507777523532575247_i64,(-3125615473399034053_i64),2474231867787851790_i64,(-5229677911900821825_i64),(-2753789194911807795_i64)];
_6.0 = _5.0;
_6.0 = _5.0 & _7.0;
_6.1 = [(-1765370197415893291_i64),164370361777335562_i64,(-4065083198839980672_i64),2935343966751039229_i64,1921809349568339657_i64,7325224231699518076_i64,5291332844117650362_i64];
_8.2 = _6.2;
_6.1 = [3733478340707226868_i64,2672632001345739029_i64,(-7054118451817923429_i64),(-2285315031161791962_i64),(-6456546746368232872_i64),7204797758510966331_i64,(-2285407327444677116_i64)];
place!(Field::<u32>(Variant(_2, 1), 3)) = Field::<i16>(Variant(_2, 1), 1) as u32;
_9 = [(-5596985211612481337_i64),4945930420963757467_i64,(-1051725691898259457_i64),4350938112979973222_i64,(-3916363270939496167_i64),(-3471287180988750313_i64),(-35558935414558962_i64)];
place!(Field::<i128>(Variant(_2, 1), 0)) = 85833456136034902519764326773357077864_i128;
_5.0 = Field::<u32>(Variant(_2, 1), 3) as u16;
place!(Field::<i128>(Variant(_2, 1), 0)) = 130078365366576464338941274583543206969_i128;
_7.1 = [(-6885608937247186503_i64),(-7886056491167118600_i64),6015093911061785858_i64,7543880632449051798_i64,1287492588056415721_i64,(-1270621270415877989_i64),1546407367488729557_i64];
place!(Field::<i16>(Variant(_2, 1), 1)) = (-108765999_i32) as i16;
_4 = _7.1;
_9 = [3406021628262931353_i64,(-8060928579427817805_i64),(-6682933095852573552_i64),(-3540396927196936300_i64),805337580612410060_i64,(-5456901302845944389_i64),8249310827819186933_i64];
Call(_8.0 = fn7(_10, _6.1, _8.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.fld6.fld1 = [_8.0,_8.0,_8.0];
_9 = [7338816462664084630_i64,(-6200061489738963498_i64),7076967969663880154_i64,(-1666116677786221630_i64),6793954967335011611_i64,(-611031903169734723_i64),(-8912549271432269944_i64)];
_13.fld1 = [(-5323749574220955819_i64),5014557362148469360_i64,7045707985362167481_i64,6122634596254027374_i64,2919173393700689808_i64,2951478110054823601_i64,(-1961060691632381275_i64)];
_13.fld5 = [_1.1,_1.1];
_14 = Field::<u32>(Variant(_2, 1), 3) + Field::<u32>(Variant(_2, 1), 3);
SetDiscriminant(_2, 0);
_14 = 7404851029918881183_i64 as u32;
_7.2 = false as u128;
_2 = Adt21::Variant1 { fld0: (-19666771496039403051126249858929271805_i128),fld1: 2991_i16,fld2: _3,fld3: _14 };
_2 = Adt21::Variant1 { fld0: (-95624935095527628471112091186349369307_i128),fld1: (-23306_i16),fld2: _3,fld3: _14 };
_8.2 = !_6.2;
_13.fld6.fld1 = [_8.0,_8.0,_8.0];
_8.0 = false as u16;
place!(Field::<i16>(Variant(_2, 1), 1)) = -(-22267_i16);
_12 = -6713541642525719098_i64;
_8.0 = _6.0;
_3 = Field::<isize>(Variant(_2, 1), 2) >> _5.0;
place!(Field::<i128>(Variant(_2, 1), 0)) = _12 as i128;
_13.fld6.fld0 = 0_i8 as i128;
_8 = _6;
_7.1 = [_12,_12,_12,_12,_12,_12,_12];
_14 = !Field::<u32>(Variant(_2, 1), 3);
place!(Field::<i16>(Variant(_2, 1), 1)) = false as i16;
_16 = &_1.0;
Goto(bb2)
}
bb2 = {
_13.fld6.fld2 = !Field::<i16>(Variant(_2, 1), 1);
_13.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_13.fld5 = [_1.1,_1.1];
_8.0 = (-338422679_i32) as u16;
_6.2 = !_5.2;
_7.1 = [_12,_12,_12,_12,_12,_12,_12];
_5.0 = _12 as u16;
_13.fld4 = [126_i8,(-54_i8),(-39_i8),(-92_i8),(-37_i8)];
place!(Field::<u32>(Variant(_2, 1), 3)) = Field::<i16>(Variant(_2, 1), 1) as u32;
_6.1 = [_12,_12,_12,_12,_12,_12,_12];
_13.fld2.0 = _5.2 as i8;
_5 = _8;
place!(Field::<isize>(Variant(_2, 1), 2)) = _3 - _3;
_6.2 = _7.2 - _5.2;
_5.1 = [_12,_12,_12,_12,_12,_12,_12];
_6 = _5;
_6 = (_7.0, _10, _7.2);
_8.1 = [_12,_12,_12,_12,_12,_12,_12];
_8.2 = _12 as u128;
_5.2 = _8.2 + _7.2;
SetDiscriminant(_2, 1);
_6.0 = _8.0 ^ _7.0;
place!(Field::<u32>(Variant(_2, 1), 3)) = !_14;
_3 = !9223372036854775807_isize;
Call(_7.0 = core::intrinsics::bswap(_5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13.fld4 = [_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0];
_13.fld6.fld0 = 8119152843992777840_u64 as i128;
_8.1 = [_12,_12,_12,_12,_12,_12,_12];
_9 = _6.1;
_9 = _10;
_13.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_4 = _9;
_6.1 = _10;
_12 = _13.fld6.fld0 as i64;
_5.1 = [_12,_12,_12,_12,_12,_12,_12];
_6.1 = _10;
_8.2 = !_6.2;
_5.2 = 7_usize as u128;
_13.fld4 = [_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0];
_13.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_7 = (_6.0, _6.1, _8.2);
_13.fld2 = (102_i8,);
_10 = [_12,_12,_12,_12,_12,_12,_12];
_5.2 = !_7.2;
_13.fld6.fld0 = _12 as i128;
_5.0 = _7.0 ^ _7.0;
Call(_9 = fn9(_6.1, _7.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5.2 = 15_u8 as u128;
_13.fld6.fld2 = -(-15023_i16);
_6.0 = _5.0 & _5.0;
place!(Field::<i16>(Variant(_2, 1), 1)) = -_13.fld6.fld2;
_18 = _3 << _6.0;
_5.0 = !_6.0;
Goto(bb5)
}
bb5 = {
_6.0 = _5.0 + _8.0;
_2 = Adt21::Variant1 { fld0: _13.fld6.fld0,fld1: _13.fld6.fld2,fld2: _3,fld3: _14 };
_5.0 = _6.0;
_5.0 = _6.0 * _7.0;
_6.2 = _8.2 | _8.2;
_6.2 = !_7.2;
_20 = _1.1;
_13.fld2.0 = _18 as i8;
_8 = _6;
place!(Field::<i128>(Variant(_2, 1), 0)) = Field::<i16>(Variant(_2, 1), 1) as i128;
_13.fld5 = [_20,_20];
Goto(bb6)
}
bb6 = {
_13.fld6.fld1 = [_6.0,_5.0,_6.0];
_13.fld4 = [_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0];
_3 = _13.fld6.fld0 as isize;
_7.0 = (-2077751806_i32) as u16;
_13.fld6.fld0 = Field::<i128>(Variant(_2, 1), 0) >> _8.0;
_3 = _18;
_19 = !_18;
_1.1 = _20;
_13.fld2 = (55_i8,);
_6.1 = _7.1;
_13.fld6.fld1 = [_6.0,_5.0,_6.0];
_24 = 7_usize as i64;
_21 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_2, 1), 0)));
_7.2 = _5.0 as u128;
_14 = Field::<u32>(Variant(_2, 1), 3) - Field::<u32>(Variant(_2, 1), 3);
place!(Field::<u32>(Variant(_2, 1), 3)) = _7.2 as u32;
_13.fld6.fld2 = _18 as i16;
_8.2 = !_7.2;
_14 = !Field::<u32>(Variant(_2, 1), 3);
_8 = (_5.0, _6.1, _7.2);
_21 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_2, 1), 0)));
_5.2 = _8.2 | _8.2;
_7.2 = Field::<i16>(Variant(_2, 1), 1) as u128;
match _13.fld2.0 {
0 => bb4,
1 => bb3,
55 => bb8,
_ => bb7
}
}
bb7 = {
_6.0 = _5.0 + _8.0;
_2 = Adt21::Variant1 { fld0: _13.fld6.fld0,fld1: _13.fld6.fld2,fld2: _3,fld3: _14 };
_5.0 = _6.0;
_5.0 = _6.0 * _7.0;
_6.2 = _8.2 | _8.2;
_6.2 = !_7.2;
_20 = _1.1;
_13.fld2.0 = _18 as i8;
_8 = _6;
place!(Field::<i128>(Variant(_2, 1), 0)) = Field::<i16>(Variant(_2, 1), 1) as i128;
_13.fld5 = [_20,_20];
Goto(bb6)
}
bb8 = {
_13.fld2 = ((-18_i8),);
_13.fld1 = _8.1;
_19 = _3;
_25 = &_13.fld2;
place!(Field::<u32>(Variant(_2, 1), 3)) = _14 + _14;
(*_21) = _13.fld6.fld0 & _13.fld6.fld0;
Call((*_21) = core::intrinsics::transmute(_8.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_26 = (*_25).0 as f64;
_24 = -_12;
_13.fld6.fld1 = [_6.0,_5.0,_5.0];
_8.2 = !_5.2;
_21 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_2, 1), 0)));
_19 = _8.0 as isize;
_28.1.fld2 = _13.fld2.0 as i16;
_5.2 = !_8.2;
match (*_25).0 {
0 => bb8,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb10,
6 => bb11,
340282366920938463463374607431768211438 => bb13,
_ => bb12
}
}
bb10 = {
_13.fld4 = [_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0];
_13.fld6.fld0 = 8119152843992777840_u64 as i128;
_8.1 = [_12,_12,_12,_12,_12,_12,_12];
_9 = _6.1;
_9 = _10;
_13.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_4 = _9;
_6.1 = _10;
_12 = _13.fld6.fld0 as i64;
_5.1 = [_12,_12,_12,_12,_12,_12,_12];
_6.1 = _10;
_8.2 = !_6.2;
_5.2 = 7_usize as u128;
_13.fld4 = [_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0];
_13.fld1 = [_12,_12,_12,_12,_12,_12,_12];
_7 = (_6.0, _6.1, _8.2);
_13.fld2 = (102_i8,);
_10 = [_12,_12,_12,_12,_12,_12,_12];
_5.2 = !_7.2;
_13.fld6.fld0 = _12 as i128;
_5.0 = _7.0 ^ _7.0;
Call(_9 = fn9(_6.1, _7.1), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_6.0 = _5.0 + _8.0;
_2 = Adt21::Variant1 { fld0: _13.fld6.fld0,fld1: _13.fld6.fld2,fld2: _3,fld3: _14 };
_5.0 = _6.0;
_5.0 = _6.0 * _7.0;
_6.2 = _8.2 | _8.2;
_6.2 = !_7.2;
_20 = _1.1;
_13.fld2.0 = _18 as i8;
_8 = _6;
place!(Field::<i128>(Variant(_2, 1), 0)) = Field::<i16>(Variant(_2, 1), 1) as i128;
_13.fld5 = [_20,_20];
Goto(bb6)
}
bb12 = {
_13.fld6.fld1 = [_6.0,_5.0,_6.0];
_13.fld4 = [_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0,_13.fld2.0];
_3 = _13.fld6.fld0 as isize;
_7.0 = (-2077751806_i32) as u16;
_13.fld6.fld0 = Field::<i128>(Variant(_2, 1), 0) >> _8.0;
_3 = _18;
_19 = !_18;
_1.1 = _20;
_13.fld2 = (55_i8,);
_6.1 = _7.1;
_13.fld6.fld1 = [_6.0,_5.0,_6.0];
_24 = 7_usize as i64;
_21 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_2, 1), 0)));
_7.2 = _5.0 as u128;
_14 = Field::<u32>(Variant(_2, 1), 3) - Field::<u32>(Variant(_2, 1), 3);
place!(Field::<u32>(Variant(_2, 1), 3)) = _7.2 as u32;
_13.fld6.fld2 = _18 as i16;
_8.2 = !_7.2;
_14 = !Field::<u32>(Variant(_2, 1), 3);
_8 = (_5.0, _6.1, _7.2);
_21 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_2, 1), 0)));
_5.2 = _8.2 | _8.2;
_7.2 = Field::<i16>(Variant(_2, 1), 1) as u128;
match _13.fld2.0 {
0 => bb4,
1 => bb3,
55 => bb8,
_ => bb7
}
}
bb13 = {
_7 = (_6.0, _13.fld1, _8.2);
SetDiscriminant(_2, 0);
_5.2 = _8.2 >> _8.0;
place!(Field::<bool>(Variant(_2, 0), 0)) = false;
_20 = _1.1;
_28.0 = core::ptr::addr_of!(_24);
_28.1.fld2 = _13.fld6.fld2;
_13.fld2.0 = 42_i8;
_28.1.fld2 = _13.fld6.fld2;
_7 = (_8.0, _4, _8.2);
_30.1 = _13.fld6.fld2 ^ _13.fld6.fld2;
Call(RET = fn10(_7.2, _13.fld1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_8.1 = [_12,_24,_24,_24,_24,_24,_12];
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(6_usize, 3_usize, Move(_3), 12_usize, Move(_12), 18_usize, Move(_18), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(6_usize, 14_usize, Move(_14), 24_usize, Move(_24), 19_usize, Move(_19), 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [i64; 7],mut _2: [i64; 7],mut _3: [i64; 7]) -> u16 {
mir! {
type RET = u16;
let _4: (u16, [i64; 7], u128);
let _5: &'static &'static [char; 2];
let _6: ((u16, [i64; 7], u128),);
let _7: [char; 2];
let _8: &'static i32;
let _9: f32;
let _10: ((Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42), f32);
let _11: bool;
let _12: isize;
let _13: [usize; 8];
let _14: u16;
let _15: usize;
let _16: [u16; 3];
let _17: [i64; 7];
let _18: i128;
let _19: Adt44;
let _20: i16;
let _21: Adt42;
let _22: ((u16, [i64; 7], u128),);
let _23: &'static char;
let _24: &'static [char; 2];
let _25: i128;
let _26: f64;
let _27: ((i8,), (*const i64, Adt36));
let _28: ();
let _29: ();
{
RET = 25238_u16;
_4.0 = RET & RET;
_2 = _3;
_6.0.0 = !_4.0;
_4.2 = !165719221563692003549627622236031107379_u128;
RET = _6.0.0;
_3 = [(-8835015288369515621_i64),(-3661990920982648755_i64),(-3527416522119868267_i64),2809242409204227315_i64,5765083735427126303_i64,(-6470844512390689718_i64),(-2799055801015996930_i64)];
_1 = [8462559431492582290_i64,(-6848400744048930267_i64),8823676850285926434_i64,8468395417371695966_i64,8052091937758413115_i64,(-8961209998054802613_i64),(-9081429288842565850_i64)];
_4.2 = (-2710609430345810658_i64) as u128;
_4.2 = 2823283862031984381238247791369075205_u128 - 81453398951279740027911862730076133590_u128;
_1 = [(-33260106554207702_i64),(-3961802566084122818_i64),(-5313093793331343333_i64),(-4668292468029745124_i64),(-8666410454653013479_i64),(-5572090256541577166_i64),6806268037863273592_i64];
_4.1 = [8619746719389736078_i64,(-4456833138149437224_i64),(-1961443121522580046_i64),(-8954644028342830756_i64),(-8645741193778575480_i64),(-2265590730573965579_i64),8013076070855565944_i64];
_6.0 = (RET, _2, _4.2);
_6.0.1 = [5654215166947191607_i64,(-129205817306611299_i64),4687934513292293903_i64,(-1016773707538600204_i64),(-3210199570983631689_i64),1801609068781155355_i64,(-7547603294558460145_i64)];
_4.1 = [(-4374984765608766190_i64),4541709356350866310_i64,(-1775745645681110975_i64),660966194955305217_i64,(-1194358475745858912_i64),(-3108428523516322443_i64),4734190635565274148_i64];
_6.0.2 = _4.2 << _6.0.0;
_1 = _2;
_4 = _6.0;
_4 = _6.0;
_7 = ['\u{547d3}','\u{ab65a}'];
_7 = ['\u{dea0b}','\u{923e9}'];
Goto(bb1)
}
bb1 = {
_4.2 = !_6.0.2;
_6 = (_4,);
RET = (-1528_i16) as u16;
_6.0.1 = [(-7603346029847111395_i64),(-5692175016674030157_i64),(-4668383968186231475_i64),(-8357434598222317816_i64),(-1794172209280720486_i64),5654682498239072586_i64,3621858437680900449_i64];
_6.0.0 = (-108935334543461944869262408848759138100_i128) as u16;
_4.2 = _6.0.2 << _6.0.2;
RET = _4.0 << _4.2;
_10.0.2.2 = _6.0.2 | _6.0.2;
RET = (-1394048401_i32) as u16;
_10.0.3 = Adt42::Variant1 { fld0: true,fld1: 4224297435_u32,fld2: (-32504_i16),fld3: (-823761503_i32) };
_10.0.3 = Adt42::Variant1 { fld0: true,fld1: 2004841500_u32,fld2: (-5723_i16),fld3: 1819552629_i32 };
place!(Field::<i16>(Variant(_10.0.3, 1), 2)) = 86_isize as i16;
_10.0.2.0 = _4.0 << _4.2;
_11 = !false;
_10.0.0 = Adt21::Variant1 { fld0: (-88180130234777753484015952942602087152_i128),fld1: Field::<i16>(Variant(_10.0.3, 1), 2),fld2: (-111_isize),fld3: 3190286375_u32 };
_10.0.2.1 = [897694093751009394_i64,4640852267977169821_i64,8909110917484182080_i64,8569363380787671884_i64,(-6070470386774672894_i64),4686137413234520727_i64,6153488127329683836_i64];
RET = _4.0;
_7 = ['\u{dd54b}','\u{c195d}'];
place!(Field::<i16>(Variant(_10.0.0, 1), 1)) = _11 as i16;
_9 = (-8051219323033204016_i64) as f32;
place!(Field::<i16>(Variant(_10.0.0, 1), 1)) = Field::<i16>(Variant(_10.0.3, 1), 2);
Goto(bb2)
}
bb2 = {
_6 = (_10.0.2,);
place!(Field::<i32>(Variant(_10.0.3, 1), 3)) = !1607647011_i32;
place!(Field::<i16>(Variant(_10.0.3, 1), 2)) = Field::<i16>(Variant(_10.0.0, 1), 1) << RET;
_3 = [(-8576365655551481700_i64),4212080157334625767_i64,(-4477760155376071201_i64),7203347290282600498_i64,(-8224058227085336509_i64),(-3187951645420743592_i64),(-8008987567556296807_i64)];
place!(Field::<bool>(Variant(_10.0.3, 1), 0)) = !_11;
place!(Field::<i128>(Variant(_10.0.0, 1), 0)) = !33018723292721466381508637482580650235_i128;
_10.0.2 = (_6.0.0, _2, _4.2);
_3 = [5049160016410925769_i64,(-9110192636436959664_i64),7996999044361341281_i64,(-8336535218174381048_i64),3903730837861512778_i64,(-2747437886038279103_i64),(-2196429859658125124_i64)];
_10.0.0 = Adt21::Variant1 { fld0: (-78978828185316674978724643628445693003_i128),fld1: Field::<i16>(Variant(_10.0.3, 1), 2),fld2: (-58_isize),fld3: 2944280957_u32 };
_10.1 = Field::<i32>(Variant(_10.0.3, 1), 3) as f32;
Call(_9 = core::intrinsics::transmute(Field::<i32>(Variant(_10.0.3, 1), 3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _10.0.2;
_6 = (_4,);
_10.0.2 = (_6.0.0, _6.0.1, _4.2);
place!(Field::<isize>(Variant(_10.0.0, 1), 2)) = -94_isize;
place!(Field::<i32>(Variant(_10.0.3, 1), 3)) = (-66082394982989277262166101036162902855_i128) as i32;
_9 = _10.1 + _10.1;
_4 = (_6.0.0, _1, _6.0.2);
_1 = _3;
_8 = &place!(Field::<i32>(Variant(_10.0.3, 1), 3));
_4.0 = _10.0.2.0;
_10.0.2 = (_4.0, _6.0.1, _4.2);
_10.0.2.1 = _2;
_6.0.0 = _4.0;
_15 = _9 as usize;
Goto(bb4)
}
bb4 = {
_9 = 14542320668165260330_u64 as f32;
_10.1 = Field::<i16>(Variant(_10.0.3, 1), 2) as f32;
_6.0.0 = _4.0;
_15 = (-83_i8) as usize;
_9 = -_10.1;
Goto(bb5)
}
bb5 = {
_2 = [(-8376281685159485842_i64),6180272946106214024_i64,(-3789543086917617871_i64),4807037412356531982_i64,(-6663293601982609446_i64),361733601879709746_i64,3527547439688764196_i64];
_1 = _4.1;
place!(Field::<bool>(Variant(_10.0.3, 1), 0)) = _11;
place!(Field::<i16>(Variant(_10.0.3, 1), 2)) = Field::<i16>(Variant(_10.0.0, 1), 1) | Field::<i16>(Variant(_10.0.0, 1), 1);
_6.0.0 = 284879605_u32 as u16;
_4.0 = _10.0.2.0;
_17 = [(-1065921022042164296_i64),(-2973315184376101624_i64),(-3553720377313963284_i64),922459516406067220_i64,(-1100706188857915_i64),3326892612534351099_i64,(-7410911494728569571_i64)];
_14 = _10.0.2.0 * _10.0.2.0;
place!(Field::<u32>(Variant(_10.0.3, 1), 1)) = !1696006249_u32;
place!(Field::<u32>(Variant(_10.0.3, 1), 1)) = 5993261715664048741_i64 as u32;
SetDiscriminant(_10.0.3, 1);
_18 = -(-35806368827401118020074219482044807918_i128);
_10.0.2.0 = _14 << _10.0.2.2;
place!(Field::<i32>(Variant(_10.0.3, 1), 3)) = 2050756140_i32;
_14 = _18 as u16;
_15 = 9836511805217921212_usize & 15944068141862944084_usize;
place!(Field::<i128>(Variant(_10.0.0, 1), 0)) = _18 | _18;
place!(Field::<u32>(Variant(_10.0.3, 1), 1)) = 282694374_u32 | 1696769875_u32;
_8 = &place!(Field::<i32>(Variant(_10.0.3, 1), 3));
_13 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10.0.1.1 = '\u{5ffd4}';
Goto(bb6)
}
bb6 = {
_10.0.1.1 = '\u{d31c9}';
_11 = !false;
_8 = &(*_8);
_6.0 = (_10.0.2.0, _17, _4.2);
_22.0.1 = [(-4675633941275917032_i64),2458525568429910744_i64,(-559655217987099240_i64),2301381336822360409_i64,(-545087143744776641_i64),8466714094820971243_i64,1254541933209195425_i64];
place!(Field::<i16>(Variant(_10.0.3, 1), 2)) = 110_i8 as i16;
_16 = [_6.0.0,_10.0.2.0,_6.0.0];
_8 = &(*_8);
_7 = [_10.0.1.1,_10.0.1.1];
_10.0.3 = Adt42::Variant1 { fld0: _11,fld1: 4110314666_u32,fld2: Field::<i16>(Variant(_10.0.0, 1), 1),fld3: 1076855817_i32 };
_1 = _4.1;
_10.0.2.2 = !_4.2;
_24 = &_7;
Goto(bb7)
}
bb7 = {
_14 = _10.0.2.0 << Field::<i16>(Variant(_10.0.3, 1), 2);
place!(Field::<u32>(Variant(_10.0.0, 1), 3)) = 1211661335_u32;
SetDiscriminant(_10.0.0, 0);
_1 = [(-7930319203412809539_i64),3060662269841124898_i64,(-925682921864147920_i64),(-3593377716400536073_i64),9002787620642276955_i64,(-7158729946083193522_i64),(-5758167917315750619_i64)];
_6.0.2 = (-574722513_i32) as u128;
_22.0 = _10.0.2;
_3 = [1950586193213616078_i64,6678545661779543198_i64,(-5470669180581964520_i64),5889076379849141516_i64,(-4185210437948331128_i64),(-1102621786657493016_i64),233169920162505941_i64];
place!(Field::<u32>(Variant(_10.0.3, 1), 1)) = _18 as u32;
_10.0.2 = (_6.0.0, _2, _22.0.2);
_10.0.2.1 = _4.1;
place!(Field::<u32>(Variant(_10.0.3, 1), 1)) = _15 as u32;
_10.0.1.0 = &_15;
_24 = &(*_24);
_16 = [_22.0.0,_22.0.0,_14];
_20 = Field::<i16>(Variant(_10.0.3, 1), 2);
_23 = &_10.0.1.1;
_10.0.2.0 = _22.0.0;
_22.0.0 = _6.0.0;
_18 = -131572682289971873223369425719441869071_i128;
_13 = [_15,_15,_15,_15,_15,_15,_15,_15];
Call(_19 = fn8(Move(_10.0.1.0), Move(_23), _10.0.2.1, _22.0, _14), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = &_24;
place!(Field::<[u16; 7]>(Variant(_10.0.0, 0), 1)) = [_6.0.0,_4.0,_10.0.2.0,_14,_14,_10.0.2.0,_10.0.2.0];
_22 = _6;
_2 = [44504717444833085_i64,(-5013341320875223634_i64),(-4707941848965148661_i64),7521813066280401396_i64,2976969287967455129_i64,3772060647114339155_i64,1755777995819785811_i64];
place!(Field::<i32>(Variant(_10.0.3, 1), 3)) = _10.0.1.1 as i32;
_21 = Move(_10.0.3);
_19 = Adt44::Variant3 { fld0: Field::<[u16; 7]>(Variant(_10.0.0, 0), 1) };
_2 = _10.0.2.1;
place!(Field::<u32>(Variant(_21, 1), 1)) = 351462387_u32 ^ 411165207_u32;
_11 = !Field::<bool>(Variant(_21, 1), 0);
_4.2 = _9 as u128;
place!(Field::<i32>(Variant(_21, 1), 3)) = _10.0.1.1 as i32;
RET = _10.0.2.0;
place!(Field::<u32>(Variant(_21, 1), 1)) = !1415100457_u32;
_6.0.1 = [8866233330644192996_i64,(-5959912841193016300_i64),4983120177193628697_i64,(-904197406876536576_i64),(-5202153732917192866_i64),1380827098265641542_i64,9097931266106823475_i64];
_6.0.2 = !_10.0.2.2;
_27.1.1.fld1 = [_6.0.0,_6.0.0,RET];
_10.0.1.1 = '\u{58a5a}';
_2 = [(-4884932914405356654_i64),(-4127233717437528754_i64),(-1165943395204637063_i64),2138880262529546976_i64,(-4799474721274169493_i64),1931618002667450122_i64,(-8535397228553809082_i64)];
_3 = [(-3708179572541614978_i64),1738903480426751785_i64,(-5451366817093847449_i64),(-2271181942205751541_i64),(-638135082304248415_i64),(-1839548558967332427_i64),(-3556005392514460011_i64)];
_5 = &(*_5);
place!(Field::<[u16; 7]>(Variant(_19, 3), 0)) = Field::<[u16; 7]>(Variant(_10.0.0, 0), 1);
_20 = Field::<i16>(Variant(_21, 1), 2) >> _22.0.0;
Goto(bb9)
}
bb9 = {
Call(_28 = dump_var(7_usize, 2_usize, Move(_2), 20_usize, Move(_20), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_28 = dump_var(7_usize, 15_usize, Move(_15), 7_usize, Move(_7), 22_usize, Move(_22), 29_usize, _29), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: &'static usize,mut _2: &'static char,mut _3: [i64; 7],mut _4: (u16, [i64; 7], u128),mut _5: u16) -> Adt44 {
mir! {
type RET = Adt44;
let _6: isize;
let _7: &'static &'static usize;
let _8: ((u16, [i64; 7], u128),);
let _9: Adt30;
let _10: [char; 2];
let _11: ();
let _12: ();
{
_4.0 = 2161715849_u32 as u16;
_3 = [3824136538904305674_i64,7592135436201943319_i64,4787850969907594000_i64,(-8061732917054992467_i64),1383313027022252291_i64,4350600575214533825_i64,4475419138275228262_i64];
_4.0 = (-766029848_i32) as u16;
Goto(bb1)
}
bb1 = {
RET = Adt44::Variant0 { fld0: _4.1 };
_3 = Field::<[i64; 7]>(Variant(RET, 0), 0);
_5 = !_4.0;
_3 = [(-8749374594705535326_i64),3200942742036908345_i64,(-602378020773974053_i64),(-618825029716244603_i64),(-1269794771942897986_i64),(-5541916833849794305_i64),7826801856563849784_i64];
_6 = !65_isize;
_4.1 = [(-9090187943295551112_i64),3710396272947021430_i64,7694900888702405460_i64,(-5866092134502780440_i64),(-2116368158061515020_i64),(-1577172290006382100_i64),7126097957818975871_i64];
_5 = _4.0;
SetDiscriminant(RET, 0);
_7 = &_1;
_5 = 15189476289048830075718527105341693713_i128 as u16;
_8 = (_4,);
_4.1 = [7987753739689968217_i64,8180514563392893709_i64,(-509888315055641465_i64),(-1285282165119734116_i64),(-4331670900507766679_i64),(-9078086368615417996_i64),4012504465982059102_i64];
_9 = Adt30::Variant1 { fld0: 48_u8 };
_4.1 = _3;
place!(Field::<[i64; 7]>(Variant(RET, 0), 0)) = [(-7953067733963838759_i64),(-7982183930711513305_i64),(-9116243404399494077_i64),(-4686264792110773209_i64),(-5228033404076243422_i64),(-6640051019360304903_i64),(-7039676027958626133_i64)];
place!(Field::<u8>(Variant(_9, 1), 0)) = 28_u8;
place!(Field::<[i64; 7]>(Variant(RET, 0), 0)) = [(-1466696823230021511_i64),7246871210723662002_i64,4734671975948898664_i64,6540594355920181302_i64,(-665812434748754902_i64),4491642315563627967_i64,(-1369164533201734517_i64)];
place!(Field::<[i64; 7]>(Variant(RET, 0), 0)) = [(-8527576811797467859_i64),(-7600280943290730209_i64),5170263165710906264_i64,(-1864814288390104422_i64),(-793023620045532210_i64),1811929865862293598_i64,1426783928972186398_i64];
_4.2 = _8.0.2;
Goto(bb2)
}
bb2 = {
Call(_11 = dump_var(8_usize, 6_usize, Move(_6), 8_usize, Move(_8), 12_usize, _12, 12_usize, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i64; 7],mut _2: [i64; 7]) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _3: f32;
let _4: (u16, [i64; 7], u128);
let _5: f64;
let _6: Adt50;
let _7: f64;
let _8: [u16; 3];
let _9: char;
let _10: ((i8,), (*const i64, Adt36));
let _11: &'static *const i128;
let _12: u64;
let _13: &'static &'static (i8,);
let _14: &'static *const i128;
let _15: ([usize; 8], &'static i32, *mut [u8; 7], usize);
let _16: u16;
let _17: f64;
let _18: Adt50;
let _19: f64;
let _20: (&'static usize, char);
let _21: Adt44;
let _22: &'static [u8; 8];
let _23: ((u16, [i64; 7], u128),);
let _24: Adt42;
let _25: i8;
let _26: &'static *const i128;
let _27: ();
let _28: ();
{
RET = [6310788442716468779_i64,3915508818737151337_i64,(-8673222654520962889_i64),6430881730880973361_i64,5001116302048144449_i64,8278330650262834145_i64,1812329457290872212_i64];
_2 = [(-5215211385404613479_i64),5064740852640176253_i64,622597087233222905_i64,5219369735046442296_i64,2027860527661071026_i64,(-5147446805232259639_i64),86052352055689099_i64];
_2 = _1;
_3 = 2399640861_u32 as f32;
RET = [(-1353526439223743352_i64),8087930344587768408_i64,5731399482540290093_i64,(-7298321561938015364_i64),(-8133438262263407221_i64),(-515999229868142610_i64),2175609369604073567_i64];
RET = _2;
_1 = _2;
RET = [6995763265961446507_i64,7278527630387809103_i64,7511326910581154999_i64,6466691762422918738_i64,(-2559555331840995941_i64),5682643370733676203_i64,(-3966244808297421727_i64)];
_4 = (31637_u16, _1, 183308640398759319616238498690563314027_u128);
_4.2 = (-1424941389_i32) as u128;
_5 = 12862131280288408067_usize as f64;
_3 = 28_i8 as f32;
_2 = RET;
_4.2 = 212769259852030819406296666013609863451_u128;
RET = [9088959718179373849_i64,(-6492882734148294738_i64),(-6306653203016108677_i64),(-6982431386966477308_i64),(-6910438060596193387_i64),(-8692735122092148902_i64),621743087964177525_i64];
_7 = -_5;
_1 = [5142545191830897607_i64,(-6454968420115700632_i64),6831486469340002074_i64,4070673828294444928_i64,6706990994374503343_i64,7699122439519739028_i64,3567831499323745533_i64];
_2 = [(-4687699674197830512_i64),5820086577794991886_i64,7260702096703415930_i64,(-708289320583506373_i64),(-6546625371431227066_i64),1694745768261999867_i64,4963020668112657790_i64];
Goto(bb1)
}
bb1 = {
RET = [(-7318253435273695214_i64),(-2148412466593685587_i64),(-243232954041848414_i64),2854410572494077342_i64,(-6519782876150208196_i64),(-6301573718153872883_i64),(-6129752508372619204_i64)];
Goto(bb2)
}
bb2 = {
_4.0 = 39378_u16 - 54232_u16;
_4.0 = 2405856495_u32 as u16;
_3 = (-1831951937_i32) as f32;
_2 = _4.1;
_1 = [(-5406120035452706303_i64),(-7447195363576480065_i64),(-8879683112939110579_i64),5494950436551921665_i64,5584129072549713684_i64,(-2848641346001308145_i64),(-2200370446993957175_i64)];
_3 = (-16808_i16) as f32;
RET = [7936953688812040430_i64,941759594751530355_i64,(-8605177651622129268_i64),(-5700302759213933789_i64),8614855467241059885_i64,273157392032179982_i64,(-8992192829565889543_i64)];
_1 = [(-1464130552678783644_i64),885553575038042885_i64,(-4472239362248079219_i64),109718707933629492_i64,(-4768506226875773841_i64),(-4559301642392808961_i64),(-1218767498441591583_i64)];
_9 = '\u{1b5ff}';
_6 = Adt50::Variant1 { fld0: _5 };
_4.1 = [2726299642136433739_i64,2311975186198438414_i64,(-4651856245058584591_i64),(-4849070398180713814_i64),(-4424124361945737958_i64),5832684211079371572_i64,(-3374211947452112449_i64)];
place!(Field::<f64>(Variant(_6, 1), 0)) = -_7;
place!(Field::<f64>(Variant(_6, 1), 0)) = _7 * _7;
match _4.2 {
212769259852030819406296666013609863451 => bb4,
_ => bb3
}
}
bb3 = {
RET = [(-7318253435273695214_i64),(-2148412466593685587_i64),(-243232954041848414_i64),2854410572494077342_i64,(-6519782876150208196_i64),(-6301573718153872883_i64),(-6129752508372619204_i64)];
Goto(bb2)
}
bb4 = {
_4.1 = [2171643749789833615_i64,257423660363897388_i64,5908626304286977597_i64,(-9036291027682619916_i64),7980029938338889756_i64,(-6080510478334022203_i64),299038171711746136_i64];
_10.1.1.fld1 = [_4.0,_4.0,_4.0];
_10.1.1.fld2 = (-10857_i16);
_10.1.1.fld0 = (-118330613022863184684682282811752268392_i128);
_5 = Field::<f64>(Variant(_6, 1), 0) - Field::<f64>(Variant(_6, 1), 0);
_3 = 134_u8 as f32;
_1 = [8188607282660000292_i64,(-1926849522209562111_i64),5677250691823038005_i64,25079045487112299_i64,5282591768489474704_i64,8322898319143204806_i64,(-1158732518193796293_i64)];
_4.0 = (-52_i8) as u16;
_4 = (43190_u16, _1, 282822188067206153508720079224302171702_u128);
_4.2 = _4.0 as u128;
_12 = !14128155667239605750_u64;
_10.1.1.fld1 = [_4.0,_4.0,_4.0];
_10.0.0 = (-5_i8) + 10_i8;
_2 = [2737609803528021167_i64,(-8323948158220742935_i64),2708427430122296757_i64,(-1612601040783453954_i64),(-6553128246586249029_i64),3160837417260428791_i64,860002998349949789_i64];
_9 = '\u{104e03}';
_2 = [8478271137440801520_i64,(-7002697838014176293_i64),(-8747649025949087349_i64),2309432059789432071_i64,6656753465045534344_i64,(-6558944628172446648_i64),(-7326289435671316974_i64)];
Call(_4.1 = core::intrinsics::transmute(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10.0 = ((-85_i8),);
_4.1 = _2;
_5 = 8949142256027431192_usize as f64;
_4.2 = 271964761021473294055049562163623959177_u128 >> _12;
_1 = [(-1669070853837157120_i64),1627805369841558483_i64,7344645549016064963_i64,5024084567360754513_i64,(-7622724382146583502_i64),6023350716962763199_i64,3798182985325092556_i64];
_7 = _5 * Field::<f64>(Variant(_6, 1), 0);
_12 = true as u64;
_4.2 = !308525522285506316935346060510255664553_u128;
_7 = 2405896322_u32 as f64;
_8 = _10.1.1.fld1;
_7 = 226_u8 as f64;
_9 = '\u{4ad68}';
_4.0 = _10.1.1.fld2 as u16;
_7 = Field::<f64>(Variant(_6, 1), 0) + Field::<f64>(Variant(_6, 1), 0);
_12 = 5725795634253810283_u64;
_8 = [_4.0,_4.0,_4.0];
_10.1.1.fld1 = _8;
match _10.1.1.fld0 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
221951753898075278778692324620015943064 => bb10,
_ => bb9
}
}
bb6 = {
_4.1 = [2171643749789833615_i64,257423660363897388_i64,5908626304286977597_i64,(-9036291027682619916_i64),7980029938338889756_i64,(-6080510478334022203_i64),299038171711746136_i64];
_10.1.1.fld1 = [_4.0,_4.0,_4.0];
_10.1.1.fld2 = (-10857_i16);
_10.1.1.fld0 = (-118330613022863184684682282811752268392_i128);
_5 = Field::<f64>(Variant(_6, 1), 0) - Field::<f64>(Variant(_6, 1), 0);
_3 = 134_u8 as f32;
_1 = [8188607282660000292_i64,(-1926849522209562111_i64),5677250691823038005_i64,25079045487112299_i64,5282591768489474704_i64,8322898319143204806_i64,(-1158732518193796293_i64)];
_4.0 = (-52_i8) as u16;
_4 = (43190_u16, _1, 282822188067206153508720079224302171702_u128);
_4.2 = _4.0 as u128;
_12 = !14128155667239605750_u64;
_10.1.1.fld1 = [_4.0,_4.0,_4.0];
_10.0.0 = (-5_i8) + 10_i8;
_2 = [2737609803528021167_i64,(-8323948158220742935_i64),2708427430122296757_i64,(-1612601040783453954_i64),(-6553128246586249029_i64),3160837417260428791_i64,860002998349949789_i64];
_9 = '\u{104e03}';
_2 = [8478271137440801520_i64,(-7002697838014176293_i64),(-8747649025949087349_i64),2309432059789432071_i64,6656753465045534344_i64,(-6558944628172446648_i64),(-7326289435671316974_i64)];
Call(_4.1 = core::intrinsics::transmute(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
RET = [(-7318253435273695214_i64),(-2148412466593685587_i64),(-243232954041848414_i64),2854410572494077342_i64,(-6519782876150208196_i64),(-6301573718153872883_i64),(-6129752508372619204_i64)];
Goto(bb2)
}
bb8 = {
_4.0 = 39378_u16 - 54232_u16;
_4.0 = 2405856495_u32 as u16;
_3 = (-1831951937_i32) as f32;
_2 = _4.1;
_1 = [(-5406120035452706303_i64),(-7447195363576480065_i64),(-8879683112939110579_i64),5494950436551921665_i64,5584129072549713684_i64,(-2848641346001308145_i64),(-2200370446993957175_i64)];
_3 = (-16808_i16) as f32;
RET = [7936953688812040430_i64,941759594751530355_i64,(-8605177651622129268_i64),(-5700302759213933789_i64),8614855467241059885_i64,273157392032179982_i64,(-8992192829565889543_i64)];
_1 = [(-1464130552678783644_i64),885553575038042885_i64,(-4472239362248079219_i64),109718707933629492_i64,(-4768506226875773841_i64),(-4559301642392808961_i64),(-1218767498441591583_i64)];
_9 = '\u{1b5ff}';
_6 = Adt50::Variant1 { fld0: _5 };
_4.1 = [2726299642136433739_i64,2311975186198438414_i64,(-4651856245058584591_i64),(-4849070398180713814_i64),(-4424124361945737958_i64),5832684211079371572_i64,(-3374211947452112449_i64)];
place!(Field::<f64>(Variant(_6, 1), 0)) = -_7;
place!(Field::<f64>(Variant(_6, 1), 0)) = _7 * _7;
match _4.2 {
212769259852030819406296666013609863451 => bb4,
_ => bb3
}
}
bb9 = {
RET = [(-7318253435273695214_i64),(-2148412466593685587_i64),(-243232954041848414_i64),2854410572494077342_i64,(-6519782876150208196_i64),(-6301573718153872883_i64),(-6129752508372619204_i64)];
Goto(bb2)
}
bb10 = {
_10.1.1.fld2 = -(-10239_i16);
_10.1.1.fld2 = 3115_i16;
_8 = [_4.0,_4.0,_4.0];
_5 = -_7;
_6 = Adt50::Variant1 { fld0: _5 };
_18 = Move(_6);
_1 = [(-4263217482587309122_i64),(-5197657034544286058_i64),7265830514517318873_i64,2880035029553599133_i64,(-322114528574342105_i64),4268529775424014313_i64,(-4716651910107136692_i64)];
_19 = _7 + _7;
_4.2 = 304059766619719385385081982607189234622_u128;
_10.1.1.fld2 = -(-26645_i16);
SetDiscriminant(_18, 1);
_4.2 = 172775001654637419717854188893896038245_u128;
place!(Field::<f64>(Variant(_18, 1), 0)) = -_7;
_10.0 = (64_i8,);
_17 = _19 * _19;
_10.1.1.fld1 = [_4.0,_4.0,_4.0];
Goto(bb11)
}
bb11 = {
_7 = _17;
_6 = Move(_18);
_3 = _10.0.0 as f32;
_15.3 = !6_usize;
_18 = Adt50::Variant1 { fld0: _17 };
_10.0 = (101_i8,);
_12 = 13873136611840697348_u64;
_2 = [(-7089419424657733748_i64),(-5247704027975988912_i64),3290166750803834895_i64,3011216235938957330_i64,(-8530705960954322451_i64),4023649537379347018_i64,(-3761034620678499435_i64)];
_4 = (62934_u16, RET, 238786742077888052332361914118529020491_u128);
RET = _2;
_23.0.1 = [1000590021599760355_i64,4961435554147632171_i64,(-3112563486439551001_i64),6445164654626674861_i64,(-7570260014223429410_i64),(-4184495043448283879_i64),(-5265109100356505109_i64)];
_10.1.1.fld2 = (-519_i16);
_20.1 = _9;
_10.1.1.fld2 = _4.0 as i16;
_4.2 = !258709018219865089710301422283500102352_u128;
_7 = -Field::<f64>(Variant(_18, 1), 0);
_20.0 = &_15.3;
_19 = _10.1.1.fld0 as f64;
match _4.0 {
0 => bb12,
62934 => bb14,
_ => bb13
}
}
bb12 = {
_10.0 = ((-85_i8),);
_4.1 = _2;
_5 = 8949142256027431192_usize as f64;
_4.2 = 271964761021473294055049562163623959177_u128 >> _12;
_1 = [(-1669070853837157120_i64),1627805369841558483_i64,7344645549016064963_i64,5024084567360754513_i64,(-7622724382146583502_i64),6023350716962763199_i64,3798182985325092556_i64];
_7 = _5 * Field::<f64>(Variant(_6, 1), 0);
_12 = true as u64;
_4.2 = !308525522285506316935346060510255664553_u128;
_7 = 2405896322_u32 as f64;
_8 = _10.1.1.fld1;
_7 = 226_u8 as f64;
_9 = '\u{4ad68}';
_4.0 = _10.1.1.fld2 as u16;
_7 = Field::<f64>(Variant(_6, 1), 0) + Field::<f64>(Variant(_6, 1), 0);
_12 = 5725795634253810283_u64;
_8 = [_4.0,_4.0,_4.0];
_10.1.1.fld1 = _8;
match _10.1.1.fld0 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
221951753898075278778692324620015943064 => bb10,
_ => bb9
}
}
bb13 = {
RET = [(-7318253435273695214_i64),(-2148412466593685587_i64),(-243232954041848414_i64),2854410572494077342_i64,(-6519782876150208196_i64),(-6301573718153872883_i64),(-6129752508372619204_i64)];
Goto(bb2)
}
bb14 = {
_9 = _20.1;
_10.1.1.fld1 = [_4.0,_4.0,_4.0];
_23.0.0 = _4.2 as u16;
_15.3 = 3_usize;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(9_usize, 9_usize, Move(_9), 1_usize, Move(_1), 12_usize, Move(_12), 28_usize, _28), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u128,mut _2: [i64; 7]) -> (*mut [u8; 7],) {
mir! {
type RET = (*mut [u8; 7],);
let _3: bool;
let _4: &'static u16;
let _5: (f32, u64, [i64; 7]);
let _6: u32;
let _7: usize;
let _8: ((i8,), (*const i64, Adt36));
let _9: (i64, i16, i8, [u16; 7]);
let _10: char;
let _11: f32;
let _12: i64;
let _13: f32;
let _14: char;
let _15: char;
let _16: isize;
let _17: usize;
let _18: u32;
let _19: Adt42;
let _20: [isize; 2];
let _21: i8;
let _22: (&'static usize, char);
let _23: ((Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42), f32);
let _24: i64;
let _25: u32;
let _26: [i64; 7];
let _27: &'static [char; 2];
let _28: [u8; 8];
let _29: [usize; 8];
let _30: isize;
let _31: i32;
let _32: f64;
let _33: isize;
let _34: (i8,);
let _35: [usize; 5];
let _36: u128;
let _37: ();
let _38: ();
{
_1 = 425833208_i32 as u128;
_1 = 19079661805466664420636728537881751550_u128;
_1 = 6242617375232732813_i64 as u128;
_2 = [(-8371368070782927563_i64),(-7049694641245808512_i64),2108454751498892326_i64,(-3602850494510479205_i64),(-1048812889513060132_i64),1709058095583284304_i64,(-751317740722946379_i64)];
_1 = 9223372036854775807_isize as u128;
_2 = [(-6688090067088440575_i64),(-2421959322587669487_i64),1538630085513888537_i64,2682668528294425689_i64,(-8546771349888269322_i64),(-2872455880745345706_i64),8638961688880201798_i64];
_2 = [(-1709714174035616956_i64),970393855450169953_i64,1675401373601729627_i64,(-4215427233066009002_i64),(-4235369520587970386_i64),(-1492312987284244062_i64),7602358996256300236_i64];
_2 = [2322257199910469323_i64,(-5533626766378413109_i64),5097389967907617583_i64,(-5530328266717388940_i64),(-5013762675355644617_i64),(-3664181862707509254_i64),(-5217805744115109729_i64)];
_2 = [9222238238213226757_i64,(-5059372123895148506_i64),1482800622771485467_i64,(-9087423149209057947_i64),(-1982685890233017027_i64),(-3378488058732074255_i64),(-4311347804306181497_i64)];
_2 = [(-2499408502816069636_i64),9138993961476126773_i64,1430535242727781091_i64,7609426062916837095_i64,(-1694452390533574552_i64),(-4407451343841912216_i64),3014453996565119512_i64];
_2 = [6513861839331163670_i64,(-3337943656198881822_i64),(-7900036592705246258_i64),6032508796182112550_i64,8107691686669613189_i64,(-4341709913674362783_i64),2812186799964349737_i64];
_2 = [5849275853551949297_i64,(-3022429047092886134_i64),4030032199841862399_i64,5721131043882529901_i64,5965720809987796690_i64,2580181650781873095_i64,(-4998091695233126900_i64)];
Goto(bb1)
}
bb1 = {
_1 = 70958720309217224145647370078532064176_u128 & 134566679741220029587632384662088784335_u128;
_3 = false;
_3 = _1 == _1;
_1 = 200614475808306832977648678572741078570_u128;
_1 = 100034222693657560110613204689997776445_u128 * 155169195201129873473700422882866902558_u128;
_1 = 4_usize as u128;
_2 = [(-6093934763122660327_i64),2692862332269787619_i64,(-6163301244920406817_i64),9111301287372593118_i64,(-2823573993063893526_i64),(-8657324908583629161_i64),(-5174966263245141712_i64)];
_1 = 88733250654620717911174326234915482883_u128;
_1 = !117370513699775675005116797898901266561_u128;
_3 = false;
_3 = false;
_3 = false;
_2 = [8969321816466820319_i64,5852976297028470796_i64,(-3363566532954717620_i64),(-3141839179221938433_i64),(-3812910191136080982_i64),(-5005818597345268890_i64),5876830676963639653_i64];
_3 = !true;
_2 = [(-4005834862736798222_i64),(-4398212932519781660_i64),8193519550007352654_i64,(-8449721950988624254_i64),(-908120270283970776_i64),(-1441273345950180293_i64),(-534701596048156295_i64)];
_3 = false & true;
_2 = [(-9038596810513114005_i64),(-1965952664215773484_i64),2127477982929450366_i64,5419624042583363264_i64,9011447602509365125_i64,4433188095832773349_i64,(-4227663377088804479_i64)];
_2 = [6633495208273482299_i64,(-125400600599511404_i64),2320923785607463537_i64,(-1193831625095488201_i64),9043466149669045677_i64,(-1134281775074358990_i64),(-6903504268526609733_i64)];
_2 = [(-3859794004198296147_i64),(-2368187506547505640_i64),1129410053186193563_i64,7020429410693709483_i64,2653293697809401557_i64,(-8891171542529623561_i64),4864756995123547874_i64];
Call(RET.0 = fn11(_2, _2, _2, _2, _1, _2, _2, _2, _2, _2, _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = true;
_3 = true;
_1 = (-3322_i16) as u128;
_2 = [9098960960187142936_i64,(-5994987882527474425_i64),5038322810686740449_i64,3367782130406017207_i64,1866982265276706072_i64,(-2767781707322204097_i64),953054612680938869_i64];
_3 = !true;
_5.1 = 16683455337040854543_u64 << _1;
_1 = 146057735628580522200000198666854775658_u128;
_8.1.1.fld1 = [21500_u16,48410_u16,2276_u16];
_8.1.1.fld2 = '\u{bea}' as i16;
_8.1.1.fld2 = 23301_i16;
_8.1.1.fld1 = [23306_u16,52950_u16,63746_u16];
_8.1.1.fld1 = [47981_u16,59567_u16,60532_u16];
Goto(bb3)
}
bb3 = {
_5.1 = 24644_u16 as u64;
_9.2 = -(-68_i8);
_5.1 = 10188291421993089471_u64 << _1;
Call(_9.0 = fn18(_8.1.1.fld1, _5.1, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = _9.0 as u32;
_8.1.0 = core::ptr::addr_of!(_9.0);
_8.1.1.fld0 = 36451010360571325696793680223439086093_i128;
_7 = 63906_u16 as usize;
_9.3 = [17320_u16,2853_u16,46932_u16,38528_u16,47990_u16,13905_u16,6235_u16];
_10 = '\u{cf6d7}';
_5.0 = _5.1 as f32;
_8.0.0 = _9.2;
_8.1.1.fld2 = 13458_i16;
_6 = 2099054366_u32 >> _8.1.1.fld2;
_11 = _6 as f32;
_2 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_11 = _5.0 + _5.0;
_1 = 176074504245800099719396941309141704257_u128 - 161194997076766746093271212932125176111_u128;
_9.1 = _1 as i16;
_10 = '\u{27c15}';
_13 = _11 * _5.0;
match _8.1.1.fld0 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
36451010360571325696793680223439086093 => bb9,
_ => bb8
}
}
bb5 = {
_5.1 = 24644_u16 as u64;
_9.2 = -(-68_i8);
_5.1 = 10188291421993089471_u64 << _1;
Call(_9.0 = fn18(_8.1.1.fld1, _5.1, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_3 = true;
_3 = true;
_1 = (-3322_i16) as u128;
_2 = [9098960960187142936_i64,(-5994987882527474425_i64),5038322810686740449_i64,3367782130406017207_i64,1866982265276706072_i64,(-2767781707322204097_i64),953054612680938869_i64];
_3 = !true;
_5.1 = 16683455337040854543_u64 << _1;
_1 = 146057735628580522200000198666854775658_u128;
_8.1.1.fld1 = [21500_u16,48410_u16,2276_u16];
_8.1.1.fld2 = '\u{bea}' as i16;
_8.1.1.fld2 = 23301_i16;
_8.1.1.fld1 = [23306_u16,52950_u16,63746_u16];
_8.1.1.fld1 = [47981_u16,59567_u16,60532_u16];
Goto(bb3)
}
bb7 = {
_1 = 70958720309217224145647370078532064176_u128 & 134566679741220029587632384662088784335_u128;
_3 = false;
_3 = _1 == _1;
_1 = 200614475808306832977648678572741078570_u128;
_1 = 100034222693657560110613204689997776445_u128 * 155169195201129873473700422882866902558_u128;
_1 = 4_usize as u128;
_2 = [(-6093934763122660327_i64),2692862332269787619_i64,(-6163301244920406817_i64),9111301287372593118_i64,(-2823573993063893526_i64),(-8657324908583629161_i64),(-5174966263245141712_i64)];
_1 = 88733250654620717911174326234915482883_u128;
_1 = !117370513699775675005116797898901266561_u128;
_3 = false;
_3 = false;
_3 = false;
_2 = [8969321816466820319_i64,5852976297028470796_i64,(-3363566532954717620_i64),(-3141839179221938433_i64),(-3812910191136080982_i64),(-5005818597345268890_i64),5876830676963639653_i64];
_3 = !true;
_2 = [(-4005834862736798222_i64),(-4398212932519781660_i64),8193519550007352654_i64,(-8449721950988624254_i64),(-908120270283970776_i64),(-1441273345950180293_i64),(-534701596048156295_i64)];
_3 = false & true;
_2 = [(-9038596810513114005_i64),(-1965952664215773484_i64),2127477982929450366_i64,5419624042583363264_i64,9011447602509365125_i64,4433188095832773349_i64,(-4227663377088804479_i64)];
_2 = [6633495208273482299_i64,(-125400600599511404_i64),2320923785607463537_i64,(-1193831625095488201_i64),9043466149669045677_i64,(-1134281775074358990_i64),(-6903504268526609733_i64)];
_2 = [(-3859794004198296147_i64),(-2368187506547505640_i64),1129410053186193563_i64,7020429410693709483_i64,2653293697809401557_i64,(-8891171542529623561_i64),4864756995123547874_i64];
Call(RET.0 = fn11(_2, _2, _2, _2, _1, _2, _2, _2, _2, _2, _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_17 = _8.1.1.fld0 as usize;
_5.2 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_17 = _7;
_3 = !false;
_18 = !_6;
_8.1.1.fld1 = [59894_u16,3680_u16,59635_u16];
_5.2 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_5.2 = _2;
_12 = _8.1.1.fld0 as i64;
_3 = false;
_5.1 = 2264877142122203735_u64;
_14 = _10;
_1 = 222333274815105890361592548211025954837_u128;
_9.1 = 128_u8 as i16;
_20 = [(-34_isize),9223372036854775807_isize];
_8.1.1.fld1 = [47116_u16,40665_u16,31750_u16];
_1 = 231300864563530134187344245929163091016_u128 & 51192380965283575950053260364821899345_u128;
_7 = _17;
_9.3 = [28836_u16,32004_u16,51081_u16,4127_u16,11885_u16,39298_u16,49288_u16];
_9.3 = [31311_u16,29778_u16,48261_u16,49226_u16,20553_u16,30961_u16,52032_u16];
_22.1 = _14;
_22.0 = &_17;
_23.0.1.0 = Move(_22.0);
_8.1.1.fld1 = [39059_u16,39248_u16,26585_u16];
match _5.1 {
0 => bb6,
1 => bb2,
2264877142122203735 => bb10,
_ => bb7
}
}
bb10 = {
_8.1.0 = core::ptr::addr_of!(_9.0);
_5 = (_13, 8508950696341942527_u64, _2);
Goto(bb11)
}
bb11 = {
_9.2 = 37429_u16 as i8;
_23.0.1.0 = &_17;
_23.0.2.1 = [_12,_9.0,_12,_12,_12,_12,_12];
_8.1.1.fld1 = [19089_u16,15649_u16,15413_u16];
Goto(bb12)
}
bb12 = {
_8.1.1.fld0 = (-102268884934074167918541750733894451602_i128);
_23.0.2.0 = 60_u16;
_5.0 = _13 - _13;
_9.2 = _8.0.0;
_9.0 = _12 << _1;
_2 = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_22.0 = &_7;
_23.0.2.0 = !29418_u16;
_5.0 = -_13;
_23.0.1 = (Move(_22.0), _10);
_5.2 = [_12,_9.0,_12,_9.0,_9.0,_12,_12];
_23.0.2.2 = _17 as u128;
_23.1 = _13;
_22.0 = &_7;
_24 = -_9.0;
_16 = !9223372036854775807_isize;
_16 = 77_isize;
_9.1 = !_8.1.1.fld2;
_7 = _17;
_30 = _6 as isize;
Call(_1 = core::intrinsics::bswap(_23.0.2.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_21 = !_8.0.0;
_5.0 = -_11;
_5 = (_11, 10805200641526927638_u64, _2);
_29 = [_7,_7,_17,_17,_17,_17,_17,_17];
_17 = _7 | _7;
_9.0 = _24 & _24;
_16 = _30;
_2 = _5.2;
_8.0.0 = _11 as i8;
_13 = _16 as f32;
_6 = !_18;
_25 = _18 | _6;
_23.0.3 = Adt42::Variant1 { fld0: _3,fld1: _25,fld2: _8.1.1.fld2,fld3: (-1954695802_i32) };
_32 = _12 as f64;
match Field::<i16>(Variant(_23.0.3, 1), 2) {
0 => bb12,
1 => bb11,
2 => bb8,
13458 => bb14,
_ => bb6
}
}
bb14 = {
_4 = &_23.0.2.0;
_26 = _2;
_25 = Field::<u32>(Variant(_23.0.3, 1), 1);
_15 = _14;
_32 = _23.0.2.0 as f64;
_33 = _8.1.1.fld0 as isize;
_34.0 = (-2037254751_i32) as i8;
_22.1 = _23.0.1.1;
_5.1 = 14662030163230161275_u64;
_23.1 = _5.0;
_22.0 = &_17;
_28 = [155_u8,12_u8,18_u8,217_u8,226_u8,209_u8,197_u8,129_u8];
_25 = !_6;
_13 = _5.0 - _11;
_6 = Field::<u32>(Variant(_23.0.3, 1), 1);
_16 = _7 as isize;
_20 = [_30,_33];
_34 = (_8.0.0,);
_5.0 = _13;
_22.0 = &_7;
_30 = _16 + _33;
_13 = -_5.0;
_8.1.0 = core::ptr::addr_of!(_12);
place!(Field::<i16>(Variant(_23.0.3, 1), 2)) = -_8.1.1.fld2;
_3 = !Field::<bool>(Variant(_23.0.3, 1), 0);
_32 = _5.0 as f64;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(10_usize, 10_usize, Move(_10), 14_usize, Move(_14), 16_usize, Move(_16), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(10_usize, 26_usize, Move(_26), 21_usize, Move(_21), 34_usize, Move(_34), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(10_usize, 12_usize, Move(_12), 1_usize, Move(_1), 28_usize, Move(_28), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [i64; 7],mut _2: [i64; 7],mut _3: [i64; 7],mut _4: [i64; 7],mut _5: u128,mut _6: [i64; 7],mut _7: [i64; 7],mut _8: [i64; 7],mut _9: [i64; 7],mut _10: [i64; 7],mut _11: [i64; 7],mut _12: bool) -> *mut [u8; 7] {
mir! {
type RET = *mut [u8; 7];
let _13: u16;
let _14: &'static i32;
let _15: f64;
let _16: &'static char;
let _17: u8;
let _18: isize;
let _19: bool;
let _20: char;
let _21: *const i64;
let _22: [u8; 2];
let _23: f64;
let _24: Adt51;
let _25: &'static [u8; 8];
let _26: &'static &'static u16;
let _27: *const i64;
let _28: &'static &'static [char; 2];
let _29: *const [u16; 3];
let _30: [u8; 5];
let _31: [u8; 7];
let _32: (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42);
let _33: bool;
let _34: (&'static usize, char);
let _35: [usize; 8];
let _36: (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42);
let _37: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _38: ();
let _39: ();
{
_10 = _2;
_1 = [915276332760316779_i64,(-7408382984351273640_i64),3037739382436859929_i64,233909480697794168_i64,1430043683074630820_i64,7569330746727635460_i64,(-4358145915206158269_i64)];
_8 = [5745758760449783718_i64,(-8935180417801918886_i64),5470845246920965494_i64,(-7118796330200707299_i64),(-4158340478644980515_i64),3468036785546495864_i64,4714711492103116119_i64];
_3 = [(-5264381517614215115_i64),(-145442015377817404_i64),3918337838515007776_i64,(-7419672213560541397_i64),6844955283885619038_i64,209615298007646457_i64,3291710721445720583_i64];
_4 = [2882959488404973688_i64,5677499815535031811_i64,(-3921609879391249173_i64),8801054024692050352_i64,(-4332642285062049630_i64),(-5795977978374698800_i64),(-4011903355677502947_i64)];
_8 = [(-2141015090229711844_i64),(-9203983063732280265_i64),(-3161971052331315765_i64),1866568428196873288_i64,(-1465837427671328491_i64),(-3027185964708012471_i64),(-2427292210442785264_i64)];
_9 = _6;
_9 = _10;
_2 = _8;
_10 = _7;
_5 = !309960959846671554282088777979510998397_u128;
_10 = [(-1925380665416486421_i64),947604142305273189_i64,709892367179915703_i64,(-2921069798319505934_i64),3529609147315430078_i64,(-3401454324160563484_i64),(-2134014241805036433_i64)];
_8 = _11;
_9 = [7238956713281514663_i64,(-1358105487245668281_i64),(-1197382659061045245_i64),(-2501169016397825735_i64),5404055010402223226_i64,(-8203288679703584942_i64),(-4962008596194232240_i64)];
_7 = [(-6705398254809967909_i64),(-6343351708873273635_i64),4665155583683721604_i64,(-965469077705205009_i64),2263261501456308114_i64,(-4308468784767297570_i64),2250096209505814276_i64];
_13 = _12 as u16;
_3 = [662010762276074557_i64,8790602920057975666_i64,9052224844425087858_i64,(-5320550973423507060_i64),(-226486210844701351_i64),(-2884624194447163669_i64),(-8541832293153174373_i64)];
_2 = _4;
_6 = [(-5466271551044334497_i64),(-441013867667948194_i64),(-304590591720641162_i64),4389414059787009048_i64,(-3105936392246171663_i64),(-1947840303385792700_i64),2922902283834782263_i64];
_7 = _10;
_2 = _4;
_12 = !true;
_6 = _3;
_6 = [(-6765161999138308715_i64),(-8877488770741934612_i64),(-8780162790829469886_i64),(-6242568224952691864_i64),6362795615993845309_i64,2499299607799719262_i64,(-2312704132702709925_i64)];
_5 = 10121389456914444869_u64 as u128;
_9 = [(-4574137513571674384_i64),9134365740109753697_i64,1901282530848945109_i64,7310872719633792234_i64,3536590514299771474_i64,253242401391057007_i64,(-2475373123769888932_i64)];
_13 = 38967_u16 - 60639_u16;
_9 = [(-147320424535996700_i64),2176203342764945801_i64,(-7881200866015870424_i64),(-8179210528349514077_i64),(-7707901389741424349_i64),(-185490385802931943_i64),(-5195734540734564131_i64)];
_1 = _6;
Goto(bb1)
}
bb1 = {
_13 = _12 as u16;
_11 = [6898794572174204916_i64,2666319483726583641_i64,(-8672757053998517357_i64),7032893496777764258_i64,686869967615204788_i64,(-7847631308523011794_i64),3939632330560380721_i64];
_8 = [6276903259101010639_i64,(-3731794624262874030_i64),(-1888459169028833547_i64),4629988810414848816_i64,5693258665134919456_i64,5197830801159689013_i64,(-4240372799534658381_i64)];
_2 = _1;
_9 = [(-5929249878036043577_i64),7840500503386233780_i64,(-5001152578121729383_i64),4764224434590927197_i64,5175910920573746448_i64,4447995473677647713_i64,(-6586103225283382730_i64)];
_4 = [(-3690187650115339648_i64),(-7059999844133108415_i64),(-1421985393781079694_i64),8419281203596844161_i64,(-7708685721061415480_i64),1940500322156872243_i64,2555190765095260600_i64];
_10 = [6438445799190516425_i64,3288705041723435134_i64,8079588454830104007_i64,2929648616567833309_i64,(-2744223675137124446_i64),(-4565135014815420341_i64),1240527008661165744_i64];
_5 = 148378650563131807843591826920493621859_u128;
_12 = false & false;
_13 = 19709_u16;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
148378650563131807843591826920493621859 => bb10,
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
_7 = [(-2815780213516546381_i64),(-9056090701508187579_i64),(-1091809856079610157_i64),(-3900427409476663219_i64),(-8231132744213042239_i64),(-2430369233177446698_i64),(-8296815525880938061_i64)];
_12 = false;
_3 = [(-1327121905311530492_i64),2081517573081199337_i64,(-2483004696703168963_i64),5833076757540760693_i64,(-7682254009743869069_i64),7153795399082490838_i64,5829210405314344226_i64];
_2 = [(-8284840922618835729_i64),4832585622226256322_i64,(-4515740077893346299_i64),583852641040846164_i64,(-924590409396367856_i64),(-1193040693902729974_i64),2198549035119583382_i64];
_15 = (-1287010079_i32) as f64;
_5 = !298417940023141043725153639621782835361_u128;
_12 = false;
_4 = [(-1137728808108292869_i64),6003711432189366836_i64,2972219003744362530_i64,2513568704807452300_i64,8412285655712324190_i64,1075934165176216330_i64,(-6827020593486350498_i64)];
_9 = _1;
_11 = [(-9117006414309370656_i64),(-8324930702667286344_i64),2913582027328961897_i64,7105837547774435064_i64,7390726661718951823_i64,(-3178709856523989704_i64),(-3701007129259991429_i64)];
_6 = [6446145932359907078_i64,1926218759645998357_i64,(-1775110587391677791_i64),8023211014576318285_i64,2603502107589148601_i64,7187560624889570212_i64,(-861439010723298522_i64)];
_7 = [5481731236652477582_i64,2993430829729368199_i64,83890805364520137_i64,7489436684303920168_i64,(-9069406842004413183_i64),7883733003790870208_i64,6139851890991149946_i64];
_9 = [(-6733572225395110355_i64),5268173358986785638_i64,29549305726206823_i64,(-5425689965773695697_i64),(-9063953955488558512_i64),1393796127673322695_i64,6066762694324836152_i64];
_19 = _12;
_4 = _10;
_3 = [7602043727295486362_i64,5282334116228615617_i64,(-9004433089692316480_i64),4214087621367273115_i64,(-5561294829244360718_i64),4283428490112129160_i64,(-3472100417354988926_i64)];
_6 = _10;
_5 = 38950172768275539707354800140424308448_u128 << _13;
_10 = [(-9059685567765615694_i64),3596427170027236600_i64,(-7799340029494671430_i64),1546374467475882015_i64,4216520374519887056_i64,(-6045408494692103041_i64),(-566254306433146228_i64)];
_3 = _9;
_2 = [7347383648647860829_i64,8931876016970564859_i64,(-5967186425175933802_i64),4565991181865971960_i64,(-8777118576581026442_i64),(-4147963020975830456_i64),(-107480325760838912_i64)];
_4 = _3;
_17 = 71_u8;
_6 = _9;
match _13 {
0 => bb7,
1 => bb2,
2 => bb11,
19709 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_6 = [(-2642778025946215197_i64),6178074903317006135_i64,(-8659924959131317204_i64),(-6575253744881973723_i64),(-2614101780404251035_i64),4524684976911056983_i64,(-5989688428683847868_i64)];
_12 = !_19;
_8 = [6786399621731655364_i64,3257422850210085641_i64,4322261213722055076_i64,(-5493573154026376618_i64),(-3668282046781655181_i64),8094241996010571548_i64,8977992599904706764_i64];
_6 = [(-4293972529957771347_i64),(-6519923555900198650_i64),(-8911997927434419418_i64),8333705087272863143_i64,4777243588654334033_i64,2959368549115430132_i64,(-8865348540809791206_i64)];
_10 = _3;
_18 = 9223372036854775807_isize * 9223372036854775807_isize;
_12 = !_19;
_11 = _9;
Goto(bb14)
}
bb14 = {
_11 = _7;
_18 = 9223372036854775807_isize - (-107_isize);
_18 = (-120101397795222917675464316176614550004_i128) as isize;
_13 = '\u{6dc73}' as u16;
_6 = _1;
_13 = 53267_u16 >> _17;
_19 = _12 | _12;
_1 = _2;
_6 = _7;
_4 = [(-6577946937938077558_i64),(-340089457846059851_i64),1997238763097698266_i64,(-4116101802106961021_i64),(-829923545743848404_i64),5081477100655438383_i64,(-5954052945539620912_i64)];
_2 = [4683355230183113466_i64,(-8969742081969037865_i64),4670733111512102709_i64,(-1703469116575288872_i64),690523126993775327_i64,6171690280339951512_i64,(-2449444638932203064_i64)];
Call(_18 = fn12(_10, _9, _13, _7, _3, _17, _8, _1, _10, _10), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_16 = &_20;
match _17 {
0 => bb16,
1 => bb17,
2 => bb18,
3 => bb19,
4 => bb20,
5 => bb21,
6 => bb22,
71 => bb24,
_ => bb23
}
}
bb16 = {
_11 = _7;
_18 = 9223372036854775807_isize - (-107_isize);
_18 = (-120101397795222917675464316176614550004_i128) as isize;
_13 = '\u{6dc73}' as u16;
_6 = _1;
_13 = 53267_u16 >> _17;
_19 = _12 | _12;
_1 = _2;
_6 = _7;
_4 = [(-6577946937938077558_i64),(-340089457846059851_i64),1997238763097698266_i64,(-4116101802106961021_i64),(-829923545743848404_i64),5081477100655438383_i64,(-5954052945539620912_i64)];
_2 = [4683355230183113466_i64,(-8969742081969037865_i64),4670733111512102709_i64,(-1703469116575288872_i64),690523126993775327_i64,6171690280339951512_i64,(-2449444638932203064_i64)];
Call(_18 = fn12(_10, _9, _13, _7, _3, _17, _8, _1, _10, _10), ReturnTo(bb15), UnwindUnreachable())
}
bb17 = {
_6 = [(-2642778025946215197_i64),6178074903317006135_i64,(-8659924959131317204_i64),(-6575253744881973723_i64),(-2614101780404251035_i64),4524684976911056983_i64,(-5989688428683847868_i64)];
_12 = !_19;
_8 = [6786399621731655364_i64,3257422850210085641_i64,4322261213722055076_i64,(-5493573154026376618_i64),(-3668282046781655181_i64),8094241996010571548_i64,8977992599904706764_i64];
_6 = [(-4293972529957771347_i64),(-6519923555900198650_i64),(-8911997927434419418_i64),8333705087272863143_i64,4777243588654334033_i64,2959368549115430132_i64,(-8865348540809791206_i64)];
_10 = _3;
_18 = 9223372036854775807_isize * 9223372036854775807_isize;
_12 = !_19;
_11 = _9;
Goto(bb14)
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_7 = [(-2815780213516546381_i64),(-9056090701508187579_i64),(-1091809856079610157_i64),(-3900427409476663219_i64),(-8231132744213042239_i64),(-2430369233177446698_i64),(-8296815525880938061_i64)];
_12 = false;
_3 = [(-1327121905311530492_i64),2081517573081199337_i64,(-2483004696703168963_i64),5833076757540760693_i64,(-7682254009743869069_i64),7153795399082490838_i64,5829210405314344226_i64];
_2 = [(-8284840922618835729_i64),4832585622226256322_i64,(-4515740077893346299_i64),583852641040846164_i64,(-924590409396367856_i64),(-1193040693902729974_i64),2198549035119583382_i64];
_15 = (-1287010079_i32) as f64;
_5 = !298417940023141043725153639621782835361_u128;
_12 = false;
_4 = [(-1137728808108292869_i64),6003711432189366836_i64,2972219003744362530_i64,2513568704807452300_i64,8412285655712324190_i64,1075934165176216330_i64,(-6827020593486350498_i64)];
_9 = _1;
_11 = [(-9117006414309370656_i64),(-8324930702667286344_i64),2913582027328961897_i64,7105837547774435064_i64,7390726661718951823_i64,(-3178709856523989704_i64),(-3701007129259991429_i64)];
_6 = [6446145932359907078_i64,1926218759645998357_i64,(-1775110587391677791_i64),8023211014576318285_i64,2603502107589148601_i64,7187560624889570212_i64,(-861439010723298522_i64)];
_7 = [5481731236652477582_i64,2993430829729368199_i64,83890805364520137_i64,7489436684303920168_i64,(-9069406842004413183_i64),7883733003790870208_i64,6139851890991149946_i64];
_9 = [(-6733572225395110355_i64),5268173358986785638_i64,29549305726206823_i64,(-5425689965773695697_i64),(-9063953955488558512_i64),1393796127673322695_i64,6066762694324836152_i64];
_19 = _12;
_4 = _10;
_3 = [7602043727295486362_i64,5282334116228615617_i64,(-9004433089692316480_i64),4214087621367273115_i64,(-5561294829244360718_i64),4283428490112129160_i64,(-3472100417354988926_i64)];
_6 = _10;
_5 = 38950172768275539707354800140424308448_u128 << _13;
_10 = [(-9059685567765615694_i64),3596427170027236600_i64,(-7799340029494671430_i64),1546374467475882015_i64,4216520374519887056_i64,(-6045408494692103041_i64),(-566254306433146228_i64)];
_3 = _9;
_2 = [7347383648647860829_i64,8931876016970564859_i64,(-5967186425175933802_i64),4565991181865971960_i64,(-8777118576581026442_i64),(-4147963020975830456_i64),(-107480325760838912_i64)];
_4 = _3;
_17 = 71_u8;
_6 = _9;
match _13 {
0 => bb7,
1 => bb2,
2 => bb11,
19709 => bb13,
_ => bb12
}
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
Return()
}
bb24 = {
_6 = [1066114798365944980_i64,7572216983057195504_i64,3395070712636067336_i64,723172755335749212_i64,(-6455909767864461264_i64),7037486357593560066_i64,4243036204421919224_i64];
_4 = [(-593974491400199875_i64),5725699978266528004_i64,1500400948940106798_i64,6607020768974123055_i64,4028838350793591916_i64,(-2148745482066673443_i64),(-4082703345303833828_i64)];
_16 = &(*_16);
_17 = 52_u8 & 180_u8;
_15 = _13 as f64;
_16 = &(*_16);
_22 = [_17,_17];
_16 = &(*_16);
_3 = [(-217113338461196482_i64),(-1264180626629471672_i64),(-5553333383009406807_i64),2217143369426666327_i64,7050095690492455216_i64,1802373649405702538_i64,(-3992027503507429522_i64)];
_16 = &(*_16);
_16 = &(*_16);
_19 = _12;
_3 = [(-7967847799712750096_i64),1368633250149781316_i64,(-713085535804545433_i64),(-7598677518909426292_i64),(-6390846654244496000_i64),(-6044425663705204620_i64),(-4567950695938831335_i64)];
_16 = &_20;
_5 = !99905245428079243005328186417409502622_u128;
_24.fld1 = [(-5655079081711357890_i64),5540832692957071905_i64,2060348850892704871_i64,(-1912271989290669153_i64),(-4726303261418086786_i64),1447739678076203442_i64,(-4679168471527323686_i64)];
_18 = (-104_isize) & (-9223372036854775808_isize);
_24.fld6.fld2 = 17632_i16;
Goto(bb25)
}
bb25 = {
_16 = &(*_16);
_17 = 176_u8 + 83_u8;
_16 = &_20;
_13 = !12341_u16;
_24.fld5 = ['\u{1c905}','\u{a7af8}'];
_7 = _11;
_24.fld2.0 = 83_i8 << _18;
Call(_5 = fn13(_2, _9, _19, _24.fld2, _19, _7, _24.fld5, _18, _11, _11), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_8 = [(-3248893135751212028_i64),(-8634175884560808086_i64),4166223521060332851_i64,3512447822153671152_i64,1969972226159123487_i64,(-9045346941199617321_i64),5014467121327211590_i64];
_24.fld6.fld0 = 122993572183885085829292958689962360579_i128 >> _24.fld2.0;
_8 = [8630349249879379128_i64,(-1616837515615829519_i64),(-2782792589518915826_i64),7740366487564034904_i64,(-8993871044786216942_i64),(-6883549308469704204_i64),(-3204790823963973735_i64)];
_24.fld6.fld1 = [_13,_13,_13];
_8 = [5451694613502652790_i64,(-3791545901490290468_i64),(-6463576418946090539_i64),5468658319270369676_i64,(-5257264993235767545_i64),(-6493951193717466602_i64),8716668395283751051_i64];
_24.fld5 = ['\u{100ea4}','\u{ff6bf}'];
_12 = _19;
Goto(bb27)
}
bb27 = {
_24.fld6.fld1 = [_13,_13,_13];
_18 = (-9223372036854775808_isize);
_24.fld2 = (89_i8,);
match _18 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
340282366920938463454151235394913435648 => bb33,
_ => bb32
}
}
bb28 = {
Return()
}
bb29 = {
Return()
}
bb30 = {
_7 = [(-2815780213516546381_i64),(-9056090701508187579_i64),(-1091809856079610157_i64),(-3900427409476663219_i64),(-8231132744213042239_i64),(-2430369233177446698_i64),(-8296815525880938061_i64)];
_12 = false;
_3 = [(-1327121905311530492_i64),2081517573081199337_i64,(-2483004696703168963_i64),5833076757540760693_i64,(-7682254009743869069_i64),7153795399082490838_i64,5829210405314344226_i64];
_2 = [(-8284840922618835729_i64),4832585622226256322_i64,(-4515740077893346299_i64),583852641040846164_i64,(-924590409396367856_i64),(-1193040693902729974_i64),2198549035119583382_i64];
_15 = (-1287010079_i32) as f64;
_5 = !298417940023141043725153639621782835361_u128;
_12 = false;
_4 = [(-1137728808108292869_i64),6003711432189366836_i64,2972219003744362530_i64,2513568704807452300_i64,8412285655712324190_i64,1075934165176216330_i64,(-6827020593486350498_i64)];
_9 = _1;
_11 = [(-9117006414309370656_i64),(-8324930702667286344_i64),2913582027328961897_i64,7105837547774435064_i64,7390726661718951823_i64,(-3178709856523989704_i64),(-3701007129259991429_i64)];
_6 = [6446145932359907078_i64,1926218759645998357_i64,(-1775110587391677791_i64),8023211014576318285_i64,2603502107589148601_i64,7187560624889570212_i64,(-861439010723298522_i64)];
_7 = [5481731236652477582_i64,2993430829729368199_i64,83890805364520137_i64,7489436684303920168_i64,(-9069406842004413183_i64),7883733003790870208_i64,6139851890991149946_i64];
_9 = [(-6733572225395110355_i64),5268173358986785638_i64,29549305726206823_i64,(-5425689965773695697_i64),(-9063953955488558512_i64),1393796127673322695_i64,6066762694324836152_i64];
_19 = _12;
_4 = _10;
_3 = [7602043727295486362_i64,5282334116228615617_i64,(-9004433089692316480_i64),4214087621367273115_i64,(-5561294829244360718_i64),4283428490112129160_i64,(-3472100417354988926_i64)];
_6 = _10;
_5 = 38950172768275539707354800140424308448_u128 << _13;
_10 = [(-9059685567765615694_i64),3596427170027236600_i64,(-7799340029494671430_i64),1546374467475882015_i64,4216520374519887056_i64,(-6045408494692103041_i64),(-566254306433146228_i64)];
_3 = _9;
_2 = [7347383648647860829_i64,8931876016970564859_i64,(-5967186425175933802_i64),4565991181865971960_i64,(-8777118576581026442_i64),(-4147963020975830456_i64),(-107480325760838912_i64)];
_4 = _3;
_17 = 71_u8;
_6 = _9;
match _13 {
0 => bb7,
1 => bb2,
2 => bb11,
19709 => bb13,
_ => bb12
}
}
bb31 = {
Return()
}
bb32 = {
Return()
}
bb33 = {
_13 = !46352_u16;
_1 = [5388573248575679976_i64,7157698477075118730_i64,(-4863640626754125209_i64),(-493503802614658993_i64),(-6879145427751614594_i64),2634558059887808633_i64,7389698610440381006_i64];
_24.fld6.fld1 = [_13,_13,_13];
_13 = 57484_u16 << _24.fld2.0;
_18 = 9223372036854775807_isize << _24.fld6.fld2;
_18 = (-9223372036854775808_isize);
_24.fld6.fld1 = [_13,_13,_13];
_20 = '\u{bc107}';
Goto(bb34)
}
bb34 = {
_24.fld5 = [_20,_20];
_9 = _3;
_20 = '\u{30017}';
_23 = _15 + _15;
_24.fld6.fld2 = _20 as i16;
_16 = &_20;
_5 = 153887355527438598291200855764315421651_u128 | 26599926413927432874973796185073773384_u128;
_16 = &_20;
_12 = _24.fld2.0 >= _24.fld2.0;
_24.fld6.fld0 = (-138657651180563856007499021996808883325_i128) | (-132077390089699079571073308538639195983_i128);
_17 = 238_u8;
_7 = _9;
_22 = [_17,_17];
_7 = _6;
_24.fld6.fld0 = 159373041173064401153941772883286175395_i128 ^ (-141651839087925667524822956721633976562_i128);
_11 = [1079871670472596064_i64,5046457260128229104_i64,(-3189073763757505552_i64),1133457260121189791_i64,4570794762549256403_i64,2128955411530752768_i64,(-1902097824218472045_i64)];
_8 = _24.fld1;
_7 = [5316325025597732320_i64,1608978419437329635_i64,(-2353456154579799310_i64),(-6376273904037732486_i64),4339428762174633163_i64,8807865880278819875_i64,(-4323091508108096707_i64)];
_24.fld6.fld2 = 1571268411075205702_i64 as i16;
_22 = [_17,_17];
_32.2.2 = !_5;
_24.fld1 = _8;
match _24.fld2.0 {
89 => bb36,
_ => bb35
}
}
bb35 = {
_6 = [1066114798365944980_i64,7572216983057195504_i64,3395070712636067336_i64,723172755335749212_i64,(-6455909767864461264_i64),7037486357593560066_i64,4243036204421919224_i64];
_4 = [(-593974491400199875_i64),5725699978266528004_i64,1500400948940106798_i64,6607020768974123055_i64,4028838350793591916_i64,(-2148745482066673443_i64),(-4082703345303833828_i64)];
_16 = &(*_16);
_17 = 52_u8 & 180_u8;
_15 = _13 as f64;
_16 = &(*_16);
_22 = [_17,_17];
_16 = &(*_16);
_3 = [(-217113338461196482_i64),(-1264180626629471672_i64),(-5553333383009406807_i64),2217143369426666327_i64,7050095690492455216_i64,1802373649405702538_i64,(-3992027503507429522_i64)];
_16 = &(*_16);
_16 = &(*_16);
_19 = _12;
_3 = [(-7967847799712750096_i64),1368633250149781316_i64,(-713085535804545433_i64),(-7598677518909426292_i64),(-6390846654244496000_i64),(-6044425663705204620_i64),(-4567950695938831335_i64)];
_16 = &_20;
_5 = !99905245428079243005328186417409502622_u128;
_24.fld1 = [(-5655079081711357890_i64),5540832692957071905_i64,2060348850892704871_i64,(-1912271989290669153_i64),(-4726303261418086786_i64),1447739678076203442_i64,(-4679168471527323686_i64)];
_18 = (-104_isize) & (-9223372036854775808_isize);
_24.fld6.fld2 = 17632_i16;
Goto(bb25)
}
bb36 = {
_32.1.1 = (*_16);
_16 = &_20;
_23 = _15 + _15;
_3 = _24.fld1;
_32.2.1 = [2081854677290003405_i64,(-1891094941019638220_i64),(-7357605329604382740_i64),2746857811300752699_i64,273167065184842567_i64,5259781523682660067_i64,1221281222407107382_i64];
_24.fld6.fld0 = (-58356464696552274823044512693844529819_i128) + (-47172276891754345921167484256822318496_i128);
_34.1 = _32.1.1;
_32.2 = (_13, _3, _5);
match _24.fld2.0 {
0 => bb9,
1 => bb37,
2 => bb38,
89 => bb40,
_ => bb39
}
}
bb37 = {
_6 = [(-2642778025946215197_i64),6178074903317006135_i64,(-8659924959131317204_i64),(-6575253744881973723_i64),(-2614101780404251035_i64),4524684976911056983_i64,(-5989688428683847868_i64)];
_12 = !_19;
_8 = [6786399621731655364_i64,3257422850210085641_i64,4322261213722055076_i64,(-5493573154026376618_i64),(-3668282046781655181_i64),8094241996010571548_i64,8977992599904706764_i64];
_6 = [(-4293972529957771347_i64),(-6519923555900198650_i64),(-8911997927434419418_i64),8333705087272863143_i64,4777243588654334033_i64,2959368549115430132_i64,(-8865348540809791206_i64)];
_10 = _3;
_18 = 9223372036854775807_isize * 9223372036854775807_isize;
_12 = !_19;
_11 = _9;
Goto(bb14)
}
bb38 = {
_24.fld6.fld1 = [_13,_13,_13];
_18 = (-9223372036854775808_isize);
_24.fld2 = (89_i8,);
match _18 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
340282366920938463454151235394913435648 => bb33,
_ => bb32
}
}
bb39 = {
_8 = [(-3248893135751212028_i64),(-8634175884560808086_i64),4166223521060332851_i64,3512447822153671152_i64,1969972226159123487_i64,(-9045346941199617321_i64),5014467121327211590_i64];
_24.fld6.fld0 = 122993572183885085829292958689962360579_i128 >> _24.fld2.0;
_8 = [8630349249879379128_i64,(-1616837515615829519_i64),(-2782792589518915826_i64),7740366487564034904_i64,(-8993871044786216942_i64),(-6883549308469704204_i64),(-3204790823963973735_i64)];
_24.fld6.fld1 = [_13,_13,_13];
_8 = [5451694613502652790_i64,(-3791545901490290468_i64),(-6463576418946090539_i64),5468658319270369676_i64,(-5257264993235767545_i64),(-6493951193717466602_i64),8716668395283751051_i64];
_24.fld5 = ['\u{100ea4}','\u{ff6bf}'];
_12 = _19;
Goto(bb27)
}
bb40 = {
RET = core::ptr::addr_of_mut!(_31);
(*RET) = [_17,_17,_17,_17,_17,_17,_17];
_37.0.2 = !_32.2.2;
_36.1.1 = (*_16);
_37.0 = _32.2;
Goto(bb41)
}
bb41 = {
Call(_38 = dump_var(11_usize, 10_usize, Move(_10), 2_usize, Move(_2), 11_usize, Move(_11), 31_usize, Move(_31)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_38 = dump_var(11_usize, 8_usize, Move(_8), 19_usize, Move(_19), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_38 = dump_var(11_usize, 1_usize, Move(_1), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [i64; 7],mut _2: [i64; 7],mut _3: u16,mut _4: [i64; 7],mut _5: [i64; 7],mut _6: u8,mut _7: [i64; 7],mut _8: [i64; 7],mut _9: [i64; 7],mut _10: [i64; 7]) -> isize {
mir! {
type RET = isize;
let _11: i8;
let _12: i16;
let _13: char;
let _14: f64;
let _15: &'static &'static (i8,);
let _16: f32;
let _17: i32;
let _18: u128;
let _19: [i8; 5];
let _20: [u16; 7];
let _21: [char; 2];
let _22: u32;
let _23: isize;
let _24: [usize; 8];
let _25: isize;
let _26: [u64; 7];
let _27: [i64; 7];
let _28: &'static usize;
let _29: (u16, [i64; 7], u128);
let _30: ();
let _31: ();
{
_4 = [1725082923123869769_i64,1291650994597836036_i64,(-1633759274282348451_i64),(-4995174036812624414_i64),(-9044112618697207848_i64),(-1217220058622159096_i64),(-846238370232100692_i64)];
RET = -(-9223372036854775808_isize);
_5 = _10;
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_9 = [7880544888843847401_i64,2940258153204313666_i64,1642181054211426278_i64,7501578726210202745_i64,9121430364764575988_i64,(-6610924616497922704_i64),3590010240846440374_i64];
_3 = 3_usize as u16;
_5 = [8127121637401433503_i64,6220562552607362667_i64,3569835731385560574_i64,(-2841005205192444325_i64),3466237323132589481_i64,24463808053822892_i64,6727235251553287433_i64];
RET = 12_isize;
_6 = 20891_i16 as u8;
_10 = _8;
_1 = [924131913549509116_i64,1358656793572231866_i64,(-6050161984011403077_i64),(-1473906364264779660_i64),(-8203002075283583963_i64),(-7774496261199709479_i64),(-613258579787985348_i64)];
_5 = [(-3981881664513216604_i64),213643240185435831_i64,361698524724709885_i64,2648777841627875341_i64,(-136989614372941796_i64),8414751295571608214_i64,2410906930308321430_i64];
_9 = [4619938264266446647_i64,(-4136541575903546291_i64),1467286965424360179_i64,4947778291675120542_i64,5678889682251092653_i64,(-3589108652696585835_i64),(-4445538071883780125_i64)];
_5 = [8530576508957875726_i64,(-1587369405950762786_i64),(-1906727190340229630_i64),(-5269099535109441839_i64),(-3618315660269187659_i64),(-6776101505919649619_i64),5150841018160217578_i64];
_6 = !23_u8;
_4 = [(-7980977225322990805_i64),(-6299012484159487086_i64),(-7210966982174684511_i64),6482346194412776730_i64,124661368982638060_i64,(-8537915403353637924_i64),(-389882173306644073_i64)];
RET = -(-9223372036854775808_isize);
Call(_11 = core::intrinsics::bswap(104_i8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [173692612209765527_i64,(-2274303387340063376_i64),7335243869968657682_i64,(-3563211902201894728_i64),(-439642315780883136_i64),3552708656096992701_i64,1741013607307242708_i64];
_1 = _2;
_9 = [(-2144350985367243107_i64),7055725912188137269_i64,(-3326810308080331911_i64),(-8488940774810215046_i64),(-5621974886607679373_i64),(-808708171455947274_i64),6304545350755223743_i64];
_2 = [1265725302178087902_i64,6003463239469145255_i64,1679665417030836035_i64,(-7308567900328402401_i64),(-6382458159629820769_i64),(-2210839032372840206_i64),6062464588690285214_i64];
_1 = [8555295164512765418_i64,4348469156581004113_i64,(-223413887035907781_i64),(-3227417446167856533_i64),3401455860607889648_i64,6189516016350041347_i64,(-4333536248040698926_i64)];
_14 = (-1329263006_i32) as f64;
_9 = _10;
_6 = 149_u8 * 70_u8;
_12 = (-106_i8) as i16;
_13 = '\u{467cc}';
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_10 = [(-1106272707359572568_i64),1873298645418611151_i64,(-367944416932131421_i64),4881572344976971209_i64,(-3785711705491902509_i64),5936440160000835504_i64,(-3761383617755690022_i64)];
_5 = _9;
_11 = (-9_i8);
RET = (-33_isize);
_12 = (-8241_i16);
_18 = 198086009302795630394957284610667577128_u128 ^ 249631130459332632066375402322250089196_u128;
_7 = [3146093367894315568_i64,(-4044976205456578382_i64),(-3909444053111033458_i64),(-453461784160877046_i64),(-1479241387846788939_i64),(-666728621005676770_i64),(-7776881789394798964_i64)];
_9 = [1023930621446359081_i64,(-5905050078119225520_i64),7688877326227979666_i64,(-4233526333575943333_i64),6576609707956713315_i64,5030961206226167619_i64,(-8230705093948226142_i64)];
_17 = -(-448898036_i32);
_18 = 2491657250_u32 as u128;
_16 = 17821908488944635242_u64 as f32;
_11 = 111_i8 ^ 7_i8;
_2 = _10;
_7 = [1619596680693790316_i64,8427217456319824500_i64,(-2941908358486069144_i64),6495199315284594626_i64,143982362090382154_i64,(-8194336535372116587_i64),2142924156905717350_i64];
_17 = _11 as i32;
_4 = [(-313391007277325436_i64),(-5472001621436592996_i64),(-6055346732784920620_i64),5591546552891070899_i64,(-1828519801810784896_i64),(-5145962543260713173_i64),4381294283493169622_i64];
_12 = 7624_i16 * (-14557_i16);
_10 = [(-5672239847569367230_i64),(-5782490781702159794_i64),434175440452418213_i64,(-6703138031524124215_i64),(-3563284429547791553_i64),(-8505635541879477590_i64),(-6620842299300012306_i64)];
_1 = [2168264167074183904_i64,(-4343158812326244329_i64),(-6720518437455977544_i64),8825459583097173423_i64,(-57714402922941480_i64),289448166938008780_i64,(-4850012638561499215_i64)];
_5 = _1;
_19 = [_11,_11,_11,_11,_11];
_13 = '\u{ef81f}';
_5 = [7704823995888458097_i64,(-8559884558099363983_i64),2506410883957483421_i64,3503104799043704521_i64,(-3949720828760742998_i64),4833235483712496674_i64,(-8141593757217808193_i64)];
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211423 => bb6,
_ => bb5
}
}
bb3 = {
_5 = [173692612209765527_i64,(-2274303387340063376_i64),7335243869968657682_i64,(-3563211902201894728_i64),(-439642315780883136_i64),3552708656096992701_i64,1741013607307242708_i64];
_1 = _2;
_9 = [(-2144350985367243107_i64),7055725912188137269_i64,(-3326810308080331911_i64),(-8488940774810215046_i64),(-5621974886607679373_i64),(-808708171455947274_i64),6304545350755223743_i64];
_2 = [1265725302178087902_i64,6003463239469145255_i64,1679665417030836035_i64,(-7308567900328402401_i64),(-6382458159629820769_i64),(-2210839032372840206_i64),6062464588690285214_i64];
_1 = [8555295164512765418_i64,4348469156581004113_i64,(-223413887035907781_i64),(-3227417446167856533_i64),3401455860607889648_i64,6189516016350041347_i64,(-4333536248040698926_i64)];
_14 = (-1329263006_i32) as f64;
_9 = _10;
_6 = 149_u8 * 70_u8;
_12 = (-106_i8) as i16;
_13 = '\u{467cc}';
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_1 = [6400704517674907014_i64,(-9047476682005661488_i64),(-3290358675923820954_i64),(-3700483757864146764_i64),(-2744710287907577509_i64),(-5739279237463207766_i64),606484870540814488_i64];
_6 = 228_u8;
_16 = 3925259547463071656_i64 as f32;
RET = 29_isize;
match _6 {
0 => bb7,
228 => bb9,
_ => bb8
}
}
bb7 = {
_5 = [173692612209765527_i64,(-2274303387340063376_i64),7335243869968657682_i64,(-3563211902201894728_i64),(-439642315780883136_i64),3552708656096992701_i64,1741013607307242708_i64];
_1 = _2;
_9 = [(-2144350985367243107_i64),7055725912188137269_i64,(-3326810308080331911_i64),(-8488940774810215046_i64),(-5621974886607679373_i64),(-808708171455947274_i64),6304545350755223743_i64];
_2 = [1265725302178087902_i64,6003463239469145255_i64,1679665417030836035_i64,(-7308567900328402401_i64),(-6382458159629820769_i64),(-2210839032372840206_i64),6062464588690285214_i64];
_1 = [8555295164512765418_i64,4348469156581004113_i64,(-223413887035907781_i64),(-3227417446167856533_i64),3401455860607889648_i64,6189516016350041347_i64,(-4333536248040698926_i64)];
_14 = (-1329263006_i32) as f64;
_9 = _10;
_6 = 149_u8 * 70_u8;
_12 = (-106_i8) as i16;
_13 = '\u{467cc}';
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_3 = _16 as u16;
_7 = _9;
_18 = 275922775785836939411232448740461059388_u128 | 265784058659150454165647423723624700302_u128;
_17 = -1263430122_i32;
_9 = _7;
match _6 {
228 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_8 = [8284169564792770785_i64,3955980013921657983_i64,4521708832777269466_i64,(-2065502509729984418_i64),861658813818948756_i64,(-1873728735983649405_i64),(-7423687023969099780_i64)];
match RET {
0 => bb1,
29 => bb12,
_ => bb4
}
}
bb12 = {
_10 = [(-3833900706390301939_i64),(-8547749264662182366_i64),(-3096115414629421218_i64),(-9008649330607582837_i64),(-4260793530152579599_i64),(-6893388115407972375_i64),8684239277446381546_i64];
_19 = [_11,_11,_11,_11,_11];
_6 = RET as u8;
_19 = [_11,_11,_11,_11,_11];
_13 = '\u{13db3}';
RET = !9223372036854775807_isize;
_13 = '\u{96312}';
_9 = _2;
_10 = [8808271114274827115_i64,677649658815587979_i64,(-8934642786081675021_i64),6001331741213388635_i64,2675283322221556170_i64,5672287592045655504_i64,495576207127088107_i64];
Goto(bb13)
}
bb13 = {
_14 = RET as f64;
_20 = [_3,_3,_3,_3,_3,_3,_3];
_8 = [(-5076284288350394427_i64),7379009371396802480_i64,(-1100811330804096170_i64),(-3397907297179499860_i64),(-6435855042511428097_i64),5355491409188577245_i64,8139089955990196862_i64];
_10 = _5;
_25 = !RET;
_16 = 2683943972_u32 as f32;
_16 = 6_usize as f32;
_4 = [6942000939501777995_i64,(-6304741233799481702_i64),(-6762680807819448406_i64),3029340938828943588_i64,4208174333160823975_i64,5718063067046530156_i64,116687251284474561_i64];
_22 = 2865498212_u32 | 3717371839_u32;
_26 = [4960173867429765072_u64,8569627909161959898_u64,13677393338524275548_u64,11470709046885769243_u64,4220757840390345749_u64,7903079280600930776_u64,18443844259082136607_u64];
_18 = _3 as u128;
_21 = [_13,_13];
_23 = RET & _25;
Goto(bb14)
}
bb14 = {
_27 = [6536381317031012010_i64,5551815135874828113_i64,6291895363886586470_i64,6681952933007975650_i64,3611892353016768704_i64,(-7689255975947234150_i64),(-5822547243217050209_i64)];
_24 = [9237950012602032640_usize,5522854946941013580_usize,16566937374588426964_usize,16799004178221888759_usize,1945233764016920689_usize,15439005199878433575_usize,18112453611761943253_usize,5_usize];
_1 = [(-3048721732312974778_i64),3289770360928340496_i64,1710924245704271934_i64,(-1775915957413509231_i64),(-4387661385543509036_i64),8501766220553734127_i64,(-4216535584218146257_i64)];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(12_usize, 25_usize, Move(_25), 5_usize, Move(_5), 22_usize, Move(_22), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(12_usize, 8_usize, Move(_8), 7_usize, Move(_7), 20_usize, Move(_20), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(12_usize, 24_usize, Move(_24), 13_usize, Move(_13), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i64; 7],mut _2: [i64; 7],mut _3: bool,mut _4: (i8,),mut _5: bool,mut _6: [i64; 7],mut _7: [char; 2],mut _8: isize,mut _9: [i64; 7],mut _10: [i64; 7]) -> u128 {
mir! {
type RET = u128;
let _11: f64;
let _12: f32;
let _13: isize;
let _14: u64;
let _15: f64;
let _16: bool;
let _17: u32;
let _18: &'static &'static [char; 2];
let _19: isize;
let _20: f32;
let _21: char;
let _22: [i32; 4];
let _23: usize;
let _24: &'static u8;
let _25: i64;
let _26: *const *const f32;
let _27: &'static &'static (i8,);
let _28: bool;
let _29: [i64; 7];
let _30: char;
let _31: (i64, i16, i8, [u16; 7]);
let _32: &'static i64;
let _33: u8;
let _34: usize;
let _35: [i8; 5];
let _36: i128;
let _37: char;
let _38: (&'static usize, char);
let _39: u64;
let _40: &'static &'static [char; 2];
let _41: i16;
let _42: char;
let _43: ((u16, [i64; 7], u128), [i32; 4], u128, (Adt21, (&'static usize, char), (u16, [i64; 7], u128), Adt42));
let _44: &'static &'static usize;
let _45: i32;
let _46: Adt42;
let _47: u32;
let _48: bool;
let _49: Adt36;
let _50: (u128, *const i64);
let _51: *mut Adt42;
let _52: i64;
let _53: *const f32;
let _54: ();
let _55: ();
{
_2 = [(-9031639362371945405_i64),8215807830051465968_i64,8595097327693627078_i64,9161051227691043324_i64,185563210814540623_i64,535213054609477461_i64,3836840898167002149_i64];
_8 = (-86227513021441307830250400776245164761_i128) as isize;
_4 = (18_i8,);
_1 = _6;
RET = 331350377011015081853030314486414102115_u128 | 88590981118872180030610633039400059345_u128;
_6 = _2;
_3 = !_5;
_5 = _3;
_9 = [(-5175481794264780003_i64),2296201012098876485_i64,8839906005147474362_i64,6503292353351391710_i64,(-3385934179984797133_i64),(-4275538245547144375_i64),5670902256476517702_i64];
_2 = [(-5567075494270794467_i64),6920213713403762597_i64,(-4587222852561005337_i64),6190640194332304179_i64,(-4539930625400153832_i64),7145395343446901394_i64,(-2177230989316788519_i64)];
_1 = _6;
_6 = [7620814033200263728_i64,(-8713415226694050140_i64),5778162873504999178_i64,(-3012374129721660621_i64),(-8935148011638485792_i64),2421526489423608054_i64,5971161107069339488_i64];
Goto(bb1)
}
bb1 = {
_7 = ['\u{3d02d}','\u{1707a}'];
RET = 314990452049873372459815076025225189304_u128;
_10 = [2879096098704018529_i64,7687274521155200777_i64,8861752256974965435_i64,853819974162141854_i64,(-6146596863097690993_i64),(-5080894504691899934_i64),3236343802023969979_i64];
_10 = _9;
_10 = _9;
_1 = [7240408366117922187_i64,(-2505743902493395767_i64),4649179483554335742_i64,(-5666888040845014453_i64),(-1611400470911764033_i64),(-880089933423007840_i64),7424655720116252777_i64];
_2 = _10;
Goto(bb2)
}
bb2 = {
_6 = [(-6995032819606779398_i64),1678453562841895225_i64,(-810485596058086997_i64),2174010283509903694_i64,9213995254966016951_i64,8869785064195953022_i64,(-3589375857570741020_i64)];
_11 = (-744520380_i32) as f64;
_10 = [(-1057347914370054036_i64),(-8700968417625806038_i64),1253030107378293212_i64,4914921099187086850_i64,423370160393667166_i64,(-8894347659747822388_i64),3186833819497109555_i64];
_12 = (-3262_i16) as f32;
_10 = [(-9103077577379515956_i64),(-7559434345960465265_i64),1946150326841310292_i64,(-4662016853469176301_i64),(-5144198963774620056_i64),(-3468747416566999076_i64),(-4734700138248231139_i64)];
_1 = [(-4181367303273732968_i64),(-1653054543174264220_i64),(-2911590207380852337_i64),(-6952370792384556824_i64),(-4698052571070480396_i64),3617391686391704942_i64,(-6384833015640414755_i64)];
_4 = (86_i8,);
_13 = _8;
_2 = [2340209245784487257_i64,1687687231335040180_i64,5184974301354300634_i64,(-3625159953744357275_i64),7076386823080980741_i64,(-6411456755415624913_i64),1844425483306598603_i64];
_3 = _8 != _13;
_8 = _13 - _13;
_1 = [8000421879240698873_i64,(-7510242852603195714_i64),8352724756141670003_i64,8534030497456907758_i64,(-3863268987023456347_i64),5922706971697535473_i64,4555729061600594088_i64];
_9 = [2809803507514261039_i64,(-760397852385980851_i64),5029129654985871314_i64,(-7217141128457643212_i64),4556964202213470016_i64,(-4179454835815632068_i64),4811306808448351384_i64];
Goto(bb3)
}
bb3 = {
_1 = _2;
_4.0 = (-84_i8) & 97_i8;
_5 = _3;
_4.0 = !(-26_i8);
_9 = _2;
RET = 3045948605_u32 as u128;
_13 = _8;
_17 = _12 as u32;
_4 = ((-16_i8),);
_5 = _3;
_16 = !_3;
_15 = (-17425_i16) as f64;
_6 = _1;
_11 = _12 as f64;
_16 = !_3;
_4 = ((-62_i8),);
_20 = -_12;
_8 = _13;
_14 = !7339348410976098039_u64;
_12 = -_20;
RET = !311371546548227751387456420833386230636_u128;
_19 = _8 >> _14;
RET = !216443656834275663380935107267276846233_u128;
_22 = [868184474_i32,526672925_i32,1755000157_i32,(-302603403_i32)];
_8 = !_13;
_20 = 24_i16 as f32;
Goto(bb4)
}
bb4 = {
_4 = ((-68_i8),);
_12 = _20;
_2 = [6104729684606861563_i64,5260783296670946291_i64,626074775092275804_i64,(-4521030032763724054_i64),(-6946394928786066094_i64),(-7756947436396763640_i64),3503850202153082110_i64];
_2 = _9;
_12 = _20;
_3 = !_5;
_25 = !8678870706807976567_i64;
_3 = !_16;
_5 = _3 & _3;
RET = 265631689231019637876959212153801110905_u128 - 34197523682288139007853769465402440758_u128;
_17 = !336479882_u32;
Goto(bb5)
}
bb5 = {
_22 = [(-619916544_i32),(-590492133_i32),1760358329_i32,(-209519638_i32)];
_13 = _8;
_6 = _9;
_11 = _15;
_3 = _16;
_5 = _3;
_9 = _6;
_4.0 = -15_i8;
_16 = _12 < _20;
_17 = 1286854691_u32 - 2585778463_u32;
_11 = -_15;
RET = !281611663735056874386258327187346890454_u128;
_23 = 14476022740686543522_usize;
_25 = -(-2577249550718099980_i64);
match _23 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
14476022740686543522 => bb7,
_ => bb6
}
}
bb6 = {
_1 = _2;
_4.0 = (-84_i8) & 97_i8;
_5 = _3;
_4.0 = !(-26_i8);
_9 = _2;
RET = 3045948605_u32 as u128;
_13 = _8;
_17 = _12 as u32;
_4 = ((-16_i8),);
_5 = _3;
_16 = !_3;
_15 = (-17425_i16) as f64;
_6 = _1;
_11 = _12 as f64;
_16 = !_3;
_4 = ((-62_i8),);
_20 = -_12;
_8 = _13;
_14 = !7339348410976098039_u64;
_12 = -_20;
RET = !311371546548227751387456420833386230636_u128;
_19 = _8 >> _14;
RET = !216443656834275663380935107267276846233_u128;
_22 = [868184474_i32,526672925_i32,1755000157_i32,(-302603403_i32)];
_8 = !_13;
_20 = 24_i16 as f32;
Goto(bb4)
}
bb7 = {
_17 = 918116804_u32 & 1256659055_u32;
_15 = -_11;
_9 = _6;
_12 = _20;
_9 = [_25,_25,_25,_25,_25,_25,_25];
_4.0 = _14 as i8;
_9 = [_25,_25,_25,_25,_25,_25,_25];
Goto(bb8)
}
bb8 = {
_13 = 69_u8 as isize;
_6 = [_25,_25,_25,_25,_25,_25,_25];
_28 = !_3;
_21 = '\u{dcbf8}';
_3 = !_5;
_28 = !_3;
_12 = _20 - _20;
_8 = _19 | _19;
_5 = _3;
_8 = !_19;
_13 = _8 >> _19;
_11 = _15;
RET = !38056868312783673794563288490847287103_u128;
RET = 294440986368468040156791292839941205905_u128 - 297742132876837653703039926213982388266_u128;
_31.2 = _4.0 << _13;
_14 = 2026046425773099165_u64;
_4 = (_31.2,);
_31.0 = _25 >> _13;
_2 = [_31.0,_31.0,_25,_31.0,_31.0,_25,_31.0];
_4.0 = _31.2;
_15 = -_11;
_7 = [_21,_21];
RET = 303637856468506064214919443031307835771_u128;
_14 = 12176653084446205432_u64 | 1660196430575230920_u64;
_10 = _9;
Call(_13 = fn14(_31.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13 = !_8;
_14 = 9143113803131348945_u64 + 12909172207963777984_u64;
_31.3 = [24358_u16,1977_u16,37300_u16,27107_u16,30135_u16,52624_u16,57492_u16];
_19 = _8;
_19 = -_13;
_29 = _1;
_30 = _21;
_33 = (-1928050879_i32) as u8;
Goto(bb10)
}
bb10 = {
_14 = !17568955428942665488_u64;
_29 = _10;
_11 = _15;
_19 = _13 << _4.0;
_32 = &_31.0;
_31.1 = !(-23839_i16);
_21 = _30;
_15 = _11 - _11;
_21 = _30;
_25 = -(*_32);
_14 = 5082411476550430873_u64 & 698612999556803861_u64;
_7 = [_30,_21];
_24 = &_33;
_29 = [(*_32),(*_32),(*_32),_31.0,_25,(*_32),(*_32)];
_20 = -_12;
_3 = _5;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
303637856468506064214919443031307835771 => bb11,
_ => bb7
}
}
bb11 = {
_14 = !13979350999658178091_u64;
_25 = -_31.0;
_16 = _5 & _5;
_34 = _23 | _23;
_19 = _8 * _13;
_25 = -(*_32);
_31.0 = _11 as i64;
_32 = &_31.0;
_7 = [_30,_21];
_7 = [_30,_21];
_30 = _21;
_31.3 = [18148_u16,37939_u16,32552_u16,19080_u16,12251_u16,24176_u16,36595_u16];
RET = (-758547071_i32) as u128;
_38.0 = &_34;
_15 = _11 - _11;
_38.1 = _21;
match _23 {
0 => bb8,
1 => bb5,
14476022740686543522 => bb12,
_ => bb3
}
}
bb12 = {
_10 = [_25,_25,_25,_25,_25,(*_32),_25];
_6 = _10;
_11 = _15 - _15;
_1 = [_25,_25,_25,_25,_25,_25,_31.0];
_24 = &_33;
_3 = !_28;
_38.0 = &_34;
_24 = &_33;
_17 = 762518860_u32 ^ 1904670442_u32;
_11 = _15 + _15;
_4 = (_31.2,);
_38.1 = _30;
_21 = _30;
_39 = _14;
_35 = [_4.0,_31.2,_4.0,_4.0,_31.2];
_20 = _12;
_17 = 151074945064009839398860756936365782261_i128 as u32;
_31.0 = _33 as i64;
_23 = _17 as usize;
_8 = !_13;
_37 = _30;
_24 = &(*_24);
_20 = 52686_u16 as f32;
_1 = [_25,_25,_25,_25,_25,_25,_25];
_4 = (_31.2,);
_7 = [_38.1,_21];
_31.2 = _4.0 + _4.0;
Goto(bb13)
}
bb13 = {
_38.0 = &_34;
_31.1 = (-28072_i16);
_5 = _25 == _25;
_36 = _14 as i128;
_5 = (*_24) <= (*_24);
_39 = _14 + _14;
_4 = (_31.2,);
_16 = !_3;
_15 = -_11;
_43.3.1.1 = _30;
_32 = &_25;
_38.0 = &_23;
_43.3.2.1 = [_25,_25,_25,(*_32),(*_32),(*_32),_25];
_4 = (_31.2,);
_43.3.2.0 = 1929_u16 | 6097_u16;
_3 = _5;
_47 = _17;
_43.0.0 = _43.3.2.0 ^ _43.3.2.0;
_21 = _30;
_42 = _38.1;
_20 = -_12;
_23 = _3 as usize;
_49.fld1 = [_43.0.0,_43.0.0,_43.3.2.0];
match _31.1 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
340282366920938463463374607431768183384 => bb22,
_ => bb21
}
}
bb14 = {
_10 = [_25,_25,_25,_25,_25,(*_32),_25];
_6 = _10;
_11 = _15 - _15;
_1 = [_25,_25,_25,_25,_25,_25,_31.0];
_24 = &_33;
_3 = !_28;
_38.0 = &_34;
_24 = &_33;
_17 = 762518860_u32 ^ 1904670442_u32;
_11 = _15 + _15;
_4 = (_31.2,);
_38.1 = _30;
_21 = _30;
_39 = _14;
_35 = [_4.0,_31.2,_4.0,_4.0,_31.2];
_20 = _12;
_17 = 151074945064009839398860756936365782261_i128 as u32;
_31.0 = _33 as i64;
_23 = _17 as usize;
_8 = !_13;
_37 = _30;
_24 = &(*_24);
_20 = 52686_u16 as f32;
_1 = [_25,_25,_25,_25,_25,_25,_25];
_4 = (_31.2,);
_7 = [_38.1,_21];
_31.2 = _4.0 + _4.0;
Goto(bb13)
}
bb15 = {
_14 = !13979350999658178091_u64;
_25 = -_31.0;
_16 = _5 & _5;
_34 = _23 | _23;
_19 = _8 * _13;
_25 = -(*_32);
_31.0 = _11 as i64;
_32 = &_31.0;
_7 = [_30,_21];
_7 = [_30,_21];
_30 = _21;
_31.3 = [18148_u16,37939_u16,32552_u16,19080_u16,12251_u16,24176_u16,36595_u16];
RET = (-758547071_i32) as u128;
_38.0 = &_34;
_15 = _11 - _11;
_38.1 = _21;
match _23 {
0 => bb8,
1 => bb5,
14476022740686543522 => bb12,
_ => bb3
}
}
bb16 = {
_1 = _2;
_4.0 = (-84_i8) & 97_i8;
_5 = _3;
_4.0 = !(-26_i8);
_9 = _2;
RET = 3045948605_u32 as u128;
_13 = _8;
_17 = _12 as u32;
_4 = ((-16_i8),);
_5 = _3;
_16 = !_3;
_15 = (-17425_i16) as f64;
_6 = _1;
_11 = _12 as f64;
_16 = !_3;
_4 = ((-62_i8),);
_20 = -_12;
_8 = _13;
_14 = !7339348410976098039_u64;
_12 = -_20;
RET = !311371546548227751387456420833386230636_u128;
_19 = _8 >> _14;
RET = !216443656834275663380935107267276846233_u128;
_22 = [868184474_i32,526672925_i32,1755000157_i32,(-302603403_i32)];
_8 = !_13;
_20 = 24_i16 as f32;
Goto(bb4)
}
bb17 = {
_13 = !_8;
_14 = 9143113803131348945_u64 + 12909172207963777984_u64;
_31.3 = [24358_u16,1977_u16,37300_u16,27107_u16,30135_u16,52624_u16,57492_u16];
_19 = _8;
_19 = -_13;
_29 = _1;
_30 = _21;
_33 = (-1928050879_i32) as u8;
Goto(bb10)
}
bb18 = {
_13 = 69_u8 as isize;
_6 = [_25,_25,_25,_25,_25,_25,_25];
_28 = !_3;
_21 = '\u{dcbf8}';
_3 = !_5;
_28 = !_3;
_12 = _20 - _20;
_8 = _19 | _19;
_5 = _3;
_8 = !_19;
_13 = _8 >> _19;
_11 = _15;
RET = !38056868312783673794563288490847287103_u128;
RET = 294440986368468040156791292839941205905_u128 - 297742132876837653703039926213982388266_u128;
_31.2 = _4.0 << _13;
_14 = 2026046425773099165_u64;
_4 = (_31.2,);
_31.0 = _25 >> _13;
_2 = [_31.0,_31.0,_25,_31.0,_31.0,_25,_31.0];
_4.0 = _31.2;
_15 = -_11;
_7 = [_21,_21];
RET = 303637856468506064214919443031307835771_u128;
_14 = 12176653084446205432_u64 | 1660196430575230920_u64;
_10 = _9;
Call(_13 = fn14(_31.0), ReturnTo(bb9), UnwindUnreachable())
}
bb19 = {
_4 = ((-68_i8),);
_12 = _20;
_2 = [6104729684606861563_i64,5260783296670946291_i64,626074775092275804_i64,(-4521030032763724054_i64),(-6946394928786066094_i64),(-7756947436396763640_i64),3503850202153082110_i64];
_2 = _9;
_12 = _20;
_3 = !_5;
_25 = !8678870706807976567_i64;
_3 = !_16;
_5 = _3 & _3;
RET = 265631689231019637876959212153801110905_u128 - 34197523682288139007853769465402440758_u128;
_17 = !336479882_u32;
Goto(bb5)
}
bb20 = {
_1 = _2;
_4.0 = (-84_i8) & 97_i8;
_5 = _3;
_4.0 = !(-26_i8);
_9 = _2;
RET = 3045948605_u32 as u128;
_13 = _8;
_17 = _12 as u32;
_4 = ((-16_i8),);
_5 = _3;
_16 = !_3;
_15 = (-17425_i16) as f64;
_6 = _1;
_11 = _12 as f64;
_16 = !_3;
_4 = ((-62_i8),);
_20 = -_12;
_8 = _13;
_14 = !7339348410976098039_u64;
_12 = -_20;
RET = !311371546548227751387456420833386230636_u128;
_19 = _8 >> _14;
RET = !216443656834275663380935107267276846233_u128;
_22 = [868184474_i32,526672925_i32,1755000157_i32,(-302603403_i32)];
_8 = !_13;
_20 = 24_i16 as f32;
Goto(bb4)
}
bb21 = {
_22 = [(-619916544_i32),(-590492133_i32),1760358329_i32,(-209519638_i32)];
_13 = _8;
_6 = _9;
_11 = _15;
_3 = _16;
_5 = _3;
_9 = _6;
_4.0 = -15_i8;
_16 = _12 < _20;
_17 = 1286854691_u32 - 2585778463_u32;
_11 = -_15;
RET = !281611663735056874386258327187346890454_u128;
_23 = 14476022740686543522_usize;
_25 = -(-2577249550718099980_i64);
match _23 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
14476022740686543522 => bb7,
_ => bb6
}
}
bb22 = {
_31.1 = -(-10038_i16);
_5 = !_3;
_6 = [_25,(*_32),(*_32),_25,(*_32),(*_32),_25];
_4 = (_31.2,);
_43.0.0 = _43.3.2.0 + _43.3.2.0;
_3 = _31.1 != _31.1;
_29 = _9;
_48 = !_16;
_43.3.2.2 = !RET;
_43.0 = (_43.3.2.0, _2, RET);
_49.fld0 = _20 as i128;
_39 = _14 >> _43.0.0;
_43.3.1.0 = &_23;
_28 = _16;
_12 = _20 * _20;
_24 = &(*_24);
_31.3 = [_43.0.0,_43.0.0,_43.3.2.0,_43.3.2.0,_43.0.0,_43.3.2.0,_43.0.0];
_52 = _20 as i64;
RET = !_43.3.2.2;
_15 = -_11;
_49.fld1 = [_43.0.0,_43.0.0,_43.3.2.0];
_34 = !_23;
_49.fld0 = _36;
_26 = core::ptr::addr_of!(_53);
Goto(bb23)
}
bb23 = {
Call(_54 = dump_var(13_usize, 10_usize, Move(_10), 9_usize, Move(_9), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_54 = dump_var(13_usize, 42_usize, Move(_42), 14_usize, Move(_14), 6_usize, Move(_6), 17_usize, Move(_17)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_54 = dump_var(13_usize, 33_usize, Move(_33), 30_usize, Move(_30), 2_usize, Move(_2), 39_usize, Move(_39)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_54 = dump_var(13_usize, 48_usize, Move(_48), 47_usize, Move(_47), 22_usize, Move(_22), 29_usize, Move(_29)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i64) -> isize {
mir! {
type RET = isize;
let _2: &'static &'static [char; 2];
let _3: &'static [u8; 8];
let _4: f64;
let _5: ((u16, [i64; 7], u128),);
let _6: char;
let _7: *const usize;
let _8: &'static (*mut [u8; 7],);
let _9: bool;
let _10: bool;
let _11: usize;
let _12: f32;
let _13: i128;
let _14: Adt21;
let _15: Adt30;
let _16: [u8; 5];
let _17: ();
let _18: ();
{
_1 = 7305484778464039758_i64 * 2189738871628554387_i64;
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize);
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 3333757669440074696_i64 * 2561434683188964131_i64;
RET = 95_isize;
_4 = _1 as f64;
_4 = 79356163779538732791735368575228089348_i128 as f64;
_1 = (-5144349333333000666_i64);
RET = (-85_isize);
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_5.0.2 = 185401069304612779360251923306915787710_u128 + 244476127966989980320701316092274692922_u128;
_5.0.0 = 7293940_i32 as u16;
_5.0.2 = 16964199070114985957620498725791578972_u128 & 133590399620813183899746198465974618179_u128;
_5.0.0 = 32524_u16 * 5468_u16;
RET = 26_isize + (-9223372036854775808_isize);
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_1 = -2398242321826653079_i64;
_5.0.2 = 93392901099360795716859643934102112618_u128;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_5.0.0 = 60195_u16;
_1 = 4440424576653312143_i64 - (-5633128728144355453_i64);
Call(_5.0.2 = fn15(RET, _5.0.0, _1, RET, RET, RET, RET, RET, _4, _1, RET, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5.0.2 = 338932077684334922990647941335233711751_u128 >> _1;
RET = 9223372036854775807_isize;
_5.0.0 = 36241_u16 - 49217_u16;
_6 = '\u{ef2b9}';
_5.0.0 = !52559_u16;
_1 = -3738121986730418777_i64;
_5.0.0 = !8213_u16;
_5.0.2 = _4 as u128;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
RET = 9223372036854775807_isize;
_5.0.2 = !250838698421422761822402996591109855377_u128;
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_10 = !false;
_4 = 10379578037054965826_u64 as f64;
_5.0.2 = 12136060230128695254_u64 as u128;
_9 = _10 & _10;
_4 = _5.0.2 as f64;
RET = 9223372036854775807_isize;
_5.0.0 = (-776345206_i32) as u16;
Call(_5.0.1 = fn16(RET, _4, _10, _4, _5.0.2, RET, RET, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = _10;
_1 = (-7474666337647004399_i64);
_11 = 5_usize - 14343618752366066314_usize;
RET = (-9223372036854775808_isize) - (-10_isize);
RET = -9223372036854775807_isize;
_6 = '\u{769af}';
_12 = _4 as f32;
RET = _6 as isize;
RET = 125_isize + 9223372036854775807_isize;
_11 = 2_usize | 3_usize;
_5.0.2 = !116054833032022456541366934219922015065_u128;
RET = (-9223372036854775808_isize);
_9 = !_10;
RET = _4 as isize;
_5.0.2 = !40932140041216442772672520330476668896_u128;
match _1 {
0 => bb1,
1 => bb5,
340282366920938463455899941094121207057 => bb7,
_ => bb6
}
}
bb5 = {
_1 = 3333757669440074696_i64 * 2561434683188964131_i64;
RET = 95_isize;
_4 = _1 as f64;
_4 = 79356163779538732791735368575228089348_i128 as f64;
_1 = (-5144349333333000666_i64);
RET = (-85_isize);
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb6 = {
_5.0.2 = 185401069304612779360251923306915787710_u128 + 244476127966989980320701316092274692922_u128;
_5.0.0 = 7293940_i32 as u16;
_5.0.2 = 16964199070114985957620498725791578972_u128 & 133590399620813183899746198465974618179_u128;
_5.0.0 = 32524_u16 * 5468_u16;
RET = 26_isize + (-9223372036854775808_isize);
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_1 = -2398242321826653079_i64;
_5.0.2 = 93392901099360795716859643934102112618_u128;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_5.0.0 = 60195_u16;
_1 = 4440424576653312143_i64 - (-5633128728144355453_i64);
Call(_5.0.2 = fn15(RET, _5.0.0, _1, RET, RET, RET, RET, RET, _4, _1, RET, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_10 = _9 & _9;
_9 = RET > RET;
_5.0.2 = !339683459776951118943524474257909392534_u128;
Goto(bb8)
}
bb8 = {
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_11 = 5_usize;
_5.0.0 = !39770_u16;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
RET = 1551745577_u32 as isize;
match _1 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
340282366920938463455899941094121207057 => bb17,
_ => bb16
}
}
bb9 = {
_10 = _9 & _9;
_9 = RET > RET;
_5.0.2 = !339683459776951118943524474257909392534_u128;
Goto(bb8)
}
bb10 = {
_5.0.2 = 185401069304612779360251923306915787710_u128 + 244476127966989980320701316092274692922_u128;
_5.0.0 = 7293940_i32 as u16;
_5.0.2 = 16964199070114985957620498725791578972_u128 & 133590399620813183899746198465974618179_u128;
_5.0.0 = 32524_u16 * 5468_u16;
RET = 26_isize + (-9223372036854775808_isize);
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_1 = -2398242321826653079_i64;
_5.0.2 = 93392901099360795716859643934102112618_u128;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_5.0.0 = 60195_u16;
_1 = 4440424576653312143_i64 - (-5633128728144355453_i64);
Call(_5.0.2 = fn15(RET, _5.0.0, _1, RET, RET, RET, RET, RET, _4, _1, RET, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_1 = 3333757669440074696_i64 * 2561434683188964131_i64;
RET = 95_isize;
_4 = _1 as f64;
_4 = 79356163779538732791735368575228089348_i128 as f64;
_1 = (-5144349333333000666_i64);
RET = (-85_isize);
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb12 = {
_9 = _10;
_1 = (-7474666337647004399_i64);
_11 = 5_usize - 14343618752366066314_usize;
RET = (-9223372036854775808_isize) - (-10_isize);
RET = -9223372036854775807_isize;
_6 = '\u{769af}';
_12 = _4 as f32;
RET = _6 as isize;
RET = 125_isize + 9223372036854775807_isize;
_11 = 2_usize | 3_usize;
_5.0.2 = !116054833032022456541366934219922015065_u128;
RET = (-9223372036854775808_isize);
_9 = !_10;
RET = _4 as isize;
_5.0.2 = !40932140041216442772672520330476668896_u128;
match _1 {
0 => bb1,
1 => bb5,
340282366920938463455899941094121207057 => bb7,
_ => bb6
}
}
bb13 = {
_5.0.2 = 338932077684334922990647941335233711751_u128 >> _1;
RET = 9223372036854775807_isize;
_5.0.0 = 36241_u16 - 49217_u16;
_6 = '\u{ef2b9}';
_5.0.0 = !52559_u16;
_1 = -3738121986730418777_i64;
_5.0.0 = !8213_u16;
_5.0.2 = _4 as u128;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
RET = 9223372036854775807_isize;
_5.0.2 = !250838698421422761822402996591109855377_u128;
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
_10 = !false;
_4 = 10379578037054965826_u64 as f64;
_5.0.2 = 12136060230128695254_u64 as u128;
_9 = _10 & _10;
_4 = _5.0.2 as f64;
RET = 9223372036854775807_isize;
_5.0.0 = (-776345206_i32) as u16;
Call(_5.0.1 = fn16(RET, _4, _10, _4, _5.0.2, RET, RET, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_5.0.2 = 185401069304612779360251923306915787710_u128 + 244476127966989980320701316092274692922_u128;
_5.0.0 = 7293940_i32 as u16;
_5.0.2 = 16964199070114985957620498725791578972_u128 & 133590399620813183899746198465974618179_u128;
_5.0.0 = 32524_u16 * 5468_u16;
RET = 26_isize + (-9223372036854775808_isize);
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_1 = -2398242321826653079_i64;
_5.0.2 = 93392901099360795716859643934102112618_u128;
_5.0.1 = [_1,_1,_1,_1,_1,_1,_1];
_5.0.0 = 60195_u16;
_1 = 4440424576653312143_i64 - (-5633128728144355453_i64);
Call(_5.0.2 = fn15(RET, _5.0.0, _1, RET, RET, RET, RET, RET, _4, _1, RET, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_1 = 3333757669440074696_i64 * 2561434683188964131_i64;
RET = 95_isize;
_4 = _1 as f64;
_4 = 79356163779538732791735368575228089348_i128 as f64;
_1 = (-5144349333333000666_i64);
RET = (-85_isize);
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) - 9223372036854775807_isize;
RET = 9223372036854775807_isize;
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_11 = 1_usize;
_7 = core::ptr::addr_of!(_11);
_1 = _5.0.1[_11] | _5.0.1[_11];
_5.0.2 = !206384326677887294859302315042389484495_u128;
_4 = 129018232841710440767012429859041201830_i128 as f64;
_12 = _1 as f32;
_13 = _4 as i128;
_5.0.1[_11] = _6 as i64;
_5.0.1[_11] = _1 + _1;
_10 = !_9;
_1 = -_5.0.1[_11];
_15 = Adt30::Variant1 { fld0: 121_u8 };
(*_7) = 14049513489399864145_usize;
Goto(bb18)
}
bb18 = {
Call(_17 = dump_var(14_usize, 5_usize, Move(_5), 11_usize, Move(_11), 6_usize, Move(_6), 18_usize, _18), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: u16,mut _3: i64,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: f64,mut _10: i64,mut _11: isize,mut _12: i64) -> u128 {
mir! {
type RET = u128;
let _13: isize;
let _14: i8;
let _15: isize;
let _16: i8;
let _17: &'static i64;
let _18: i128;
let _19: (*const i64, Adt36);
let _20: *const i64;
let _21: f64;
let _22: u32;
let _23: (*mut [u8; 7],);
let _24: &'static char;
let _25: [i8; 5];
let _26: (*const i64, Adt36);
let _27: i128;
let _28: Adt51;
let _29: i128;
let _30: [usize; 5];
let _31: *mut [u64; 7];
let _32: [u16; 7];
let _33: isize;
let _34: bool;
let _35: Adt44;
let _36: &'static &'static usize;
let _37: [i64; 7];
let _38: ();
let _39: ();
{
_5 = _8;
_14 = -71_i8;
_6 = !_11;
RET = _3 as u128;
_13 = _9 as isize;
_7 = _1;
match _2 {
0 => bb1,
60195 => bb3,
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
RET = !16319256252972417793778134587795488573_u128;
RET = 69303424163606366334531864717042230393_u128 & 321801060114268563994017348503723831108_u128;
_15 = _1 & _4;
_15 = (-1965586899_i32) as isize;
_11 = !_1;
_15 = _11;
_6 = _7 ^ _11;
_13 = _15 * _1;
RET = !258461481430755087430833777737499390167_u128;
_1 = _7 * _11;
_1 = 13391494422468998291_u64 as isize;
_15 = -_11;
_11 = _6 >> _10;
_1 = _5;
_2 = 58206_u16;
_8 = _13;
_7 = _8 << _11;
_16 = _14 << _3;
_15 = !_6;
_14 = _16 * _16;
_5 = 65885617_u32 as isize;
_1 = -_7;
_1 = -_7;
_14 = _16;
RET = _15 as u128;
_4 = _2 as isize;
_5 = !_15;
_16 = !_14;
match _2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
58206 => bb9,
_ => bb8
}
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
_14 = -_16;
_19.0 = core::ptr::addr_of!(_3);
_19.1.fld0 = 4_usize as i128;
_6 = -_11;
_13 = -_1;
_3 = 10144154309021695534_u64 as i64;
RET = 256439902542506072346749181221487888057_u128;
_19.1.fld1 = [_2,_2,_2];
_18 = _19.1.fld0 << _1;
RET = 132838195496211632263689902099953138213_u128;
_8 = _13;
_5 = '\u{b57ea}' as isize;
_12 = _3;
_14 = -_16;
_18 = 7750215407580037619_usize as i128;
_20 = core::ptr::addr_of!(_12);
_16 = !_14;
_9 = 6910704031941125849_usize as f64;
_17 = &_12;
_19.0 = Move(_20);
_18 = 4391484552777852843_usize as i128;
_21 = -_9;
_2 = 63078_u16;
RET = _18 as u128;
_13 = _7;
RET = 186792162216667349306988908064608707064_u128 + 80567507409588787508528756194752756485_u128;
_17 = &_12;
_12 = _3;
Goto(bb10)
}
bb10 = {
_20 = core::ptr::addr_of!(_3);
_9 = -_21;
_10 = _18 as i64;
_14 = _16 & _16;
_19.0 = core::ptr::addr_of!((*_20));
_21 = 4_usize as f64;
_22 = 1131089815_u32;
_14 = !_16;
_17 = &_10;
_7 = 29128_i16 as isize;
_20 = core::ptr::addr_of!((*_17));
_2 = 15499_u16;
RET = !229589420637059226687384575276134218863_u128;
_22 = !3922019317_u32;
_8 = _22 as isize;
_11 = _6;
Goto(bb11)
}
bb11 = {
_11 = -_13;
_10 = _3;
_10 = _12;
_26.1.fld1 = [_2,_2,_2];
_26.1.fld2 = !228_i16;
_12 = _10 - _3;
_18 = -_19.1.fld0;
_26.1.fld0 = _18 - _19.1.fld0;
_19.1.fld0 = !_26.1.fld0;
_17 = &_12;
_7 = (*_17) as isize;
_19.1.fld2 = 9649270365830756496_u64 as i16;
_15 = _13;
_26.0 = core::ptr::addr_of!((*_17));
_10 = !_3;
_26.1.fld1 = _19.1.fld1;
_15 = _8;
_28.fld6.fld1 = _26.1.fld1;
_26.0 = core::ptr::addr_of!((*_17));
_25 = [_14,_16,_14,_14,_16];
_28.fld6 = Adt36 { fld0: _26.1.fld0,fld1: _19.1.fld1,fld2: _19.1.fld2 };
_27 = _14 as i128;
_28.fld5 = ['\u{5c2e7}','\u{1101f}'];
_26.1.fld1 = _19.1.fld1;
Goto(bb12)
}
bb12 = {
_28.fld1 = [(*_17),(*_17),(*_17),_10,_10,(*_17),(*_17)];
_26.1.fld1 = [_2,_2,_2];
_19.1.fld2 = _28.fld6.fld2 - _28.fld6.fld2;
_28.fld4 = _25;
_28.fld2.0 = !_14;
_26.1.fld1 = [_2,_2,_2];
RET = false as u128;
_14 = _16;
_1 = _22 as isize;
_19.1 = Move(_26.1);
_26.1.fld0 = _19.1.fld0;
match _2 {
0 => bb13,
15499 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_20 = core::ptr::addr_of!(_3);
_9 = -_21;
_10 = _18 as i64;
_14 = _16 & _16;
_19.0 = core::ptr::addr_of!((*_20));
_21 = 4_usize as f64;
_22 = 1131089815_u32;
_14 = !_16;
_17 = &_10;
_7 = 29128_i16 as isize;
_20 = core::ptr::addr_of!((*_17));
_2 = 15499_u16;
RET = !229589420637059226687384575276134218863_u128;
_22 = !3922019317_u32;
_8 = _22 as isize;
_11 = _6;
Goto(bb11)
}
bb15 = {
_18 = _27 * _26.1.fld0;
_7 = _8;
_3 = _12;
_2 = 31391_u16;
_13 = _6;
_32 = [_2,_2,_2,_2,_2,_2,_2];
_26.1 = Adt36 { fld0: _27,fld1: _19.1.fld1,fld2: _19.1.fld2 };
_19.1 = Move(_26.1);
_35 = Adt44::Variant3 { fld0: _32 };
_26.1.fld1 = [_2,_2,_2];
_22 = 2528322990_u32 - 3818510221_u32;
_32 = [_2,_2,_2,_2,_2,_2,_2];
_26.0 = core::ptr::addr_of!((*_17));
_26.1.fld2 = 2024839411_i32 as i16;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(15_usize, 11_usize, Move(_11), 14_usize, Move(_14), 6_usize, Move(_6), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(15_usize, 16_usize, Move(_16), 5_usize, Move(_5), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(15_usize, 18_usize, Move(_18), 3_usize, Move(_3), 39_usize, _39, 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: f64,mut _3: bool,mut _4: f64,mut _5: u128,mut _6: isize,mut _7: isize,mut _8: bool) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _9: (f32, u64, [i64; 7]);
let _10: u128;
let _11: [i32; 4];
let _12: (f32, u64, [i64; 7]);
let _13: (u16, [i64; 7], u128);
let _14: Adt44;
let _15: *const [u16; 3];
let _16: f32;
let _17: i128;
let _18: [usize; 8];
let _19: char;
let _20: [i64; 7];
let _21: ((u16, [i64; 7], u128),);
let _22: ();
let _23: ();
{
RET = [907337804585518391_i64,(-7258652342710495508_i64),5380768908128596305_i64,1939100901883975585_i64,7700039226543843724_i64,(-6150590316158549118_i64),(-2814066235928824563_i64)];
_2 = -_4;
_9.1 = 15665534042787419169_u64 * 9599957815974516810_u64;
_4 = 172_u8 as f64;
_10 = _5 ^ _5;
_1 = 7723161879213086404_i64 as isize;
_10 = !_5;
_9.1 = _7 as u64;
_9.2 = RET;
_1 = _7;
_6 = _10 as isize;
Goto(bb1)
}
bb1 = {
_9.0 = (-4799_i16) as f32;
_11 = [1897881769_i32,(-1321853676_i32),(-138975382_i32),1225293381_i32];
_11 = [1731814490_i32,(-1847359530_i32),1487742799_i32,1202253276_i32];
_12.0 = _9.0;
_13.0 = _5 as u16;
_13.1 = [1546302409996781365_i64,(-3113490050467048816_i64),(-821250504359493835_i64),(-4010407577528731321_i64),3002425591960906823_i64,(-1322513232910928196_i64),(-4171748156007215045_i64)];
_9.2 = _13.1;
_12 = _9;
_5 = _10;
_1 = -_7;
_12.0 = -_9.0;
_2 = _4;
_3 = _8 | _8;
Goto(bb2)
}
bb2 = {
_2 = _4;
_12.0 = -_9.0;
match _7 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb3 = {
_9.0 = (-4799_i16) as f32;
_11 = [1897881769_i32,(-1321853676_i32),(-138975382_i32),1225293381_i32];
_11 = [1731814490_i32,(-1847359530_i32),1487742799_i32,1202253276_i32];
_12.0 = _9.0;
_13.0 = _5 as u16;
_13.1 = [1546302409996781365_i64,(-3113490050467048816_i64),(-821250504359493835_i64),(-4010407577528731321_i64),3002425591960906823_i64,(-1322513232910928196_i64),(-4171748156007215045_i64)];
_9.2 = _13.1;
_12 = _9;
_5 = _10;
_1 = -_7;
_12.0 = -_9.0;
_2 = _4;
_3 = _8 | _8;
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
_9 = _12;
_13.0 = !38695_u16;
_4 = _2;
_12.1 = _9.1;
_12.2 = [1104736439070190865_i64,(-1564285253023344832_i64),6045501176746065342_i64,5569822685829034623_i64,(-2863724553204340055_i64),2371140150936595184_i64,(-7179473338637193572_i64)];
_12 = (_9.0, _9.1, _9.2);
_13 = (16569_u16, _9.2, _5);
_10 = !_13.2;
_13.2 = _10;
_9.1 = _12.1 >> _13.0;
_13 = (18808_u16, _9.2, _5);
_5 = !_13.2;
RET = [(-2281830448927035478_i64),7592508619178153765_i64,2923848406396479808_i64,1189528110260890695_i64,(-7006986746341090078_i64),(-3460172642016973782_i64),1629102503363512313_i64];
_10 = 2664954057769632151_usize as u128;
_2 = _4 + _4;
_12.0 = _9.0 * _9.0;
_1 = _7;
_2 = _4;
_8 = _3 & _3;
_6 = _1 >> _13.0;
_13 = (38609_u16, _9.2, _5);
Call(RET = fn17(_12, _9.1, _9, _9, _12, _6, _12, _8, _9, _13.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9.1 = !_12.1;
RET = _13.1;
_12.2 = [(-4319561292089844355_i64),(-4931373579758262292_i64),4585657740297586468_i64,8161749516599567724_i64,307345155770538620_i64,(-6684074061951173352_i64),(-2540218955611785987_i64)];
_2 = _4;
_5 = _10 | _13.2;
_4 = _2;
_5 = 219_u8 as u128;
_12.2 = [(-5962809974445918746_i64),(-444963557606226747_i64),(-987639667817874492_i64),3286251493294579283_i64,6952458375124561143_i64,8486853074264593396_i64,(-7031202512206473022_i64)];
_12 = (_9.0, _9.1, _9.2);
_13.2 = _5 + _10;
_13.1 = [(-3460394789068061732_i64),(-8150267622118140763_i64),(-2823717521064521143_i64),(-9063196552098630279_i64),861346514910431787_i64,2550714799622229331_i64,(-3502966396289601503_i64)];
_13 = (19357_u16, _9.2, _10);
_13 = (39034_u16, _12.2, _10);
_10 = _5;
_8 = _6 <= _6;
_12 = _9;
_9 = (_12.0, _12.1, _13.1);
_2 = -_4;
_1 = 128_u8 as isize;
_18 = [4725211908259989555_usize,5_usize,5858357475288746772_usize,14678672360274763341_usize,13732477436823108070_usize,1424611798278611368_usize,6_usize,2120914110192967384_usize];
_16 = _12.0 - _9.0;
RET = [2073323815000073890_i64,(-550244158940608559_i64),4175616400471224387_i64,5197960152554288117_i64,(-5598754555489937917_i64),6364219551172717778_i64,8025387112806063659_i64];
match _13.0 {
0 => bb9,
1 => bb12,
2 => bb13,
3 => bb14,
39034 => bb16,
_ => bb15
}
}
bb12 = {
_9 = _12;
_13.0 = !38695_u16;
_4 = _2;
_12.1 = _9.1;
_12.2 = [1104736439070190865_i64,(-1564285253023344832_i64),6045501176746065342_i64,5569822685829034623_i64,(-2863724553204340055_i64),2371140150936595184_i64,(-7179473338637193572_i64)];
_12 = (_9.0, _9.1, _9.2);
_13 = (16569_u16, _9.2, _5);
_10 = !_13.2;
_13.2 = _10;
_9.1 = _12.1 >> _13.0;
_13 = (18808_u16, _9.2, _5);
_5 = !_13.2;
RET = [(-2281830448927035478_i64),7592508619178153765_i64,2923848406396479808_i64,1189528110260890695_i64,(-7006986746341090078_i64),(-3460172642016973782_i64),1629102503363512313_i64];
_10 = 2664954057769632151_usize as u128;
_2 = _4 + _4;
_12.0 = _9.0 * _9.0;
_1 = _7;
_2 = _4;
_8 = _3 & _3;
_6 = _1 >> _13.0;
_13 = (38609_u16, _9.2, _5);
Call(RET = fn17(_12, _9.1, _9, _9, _12, _6, _12, _8, _9, _13.0), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_2 = _4;
_12.0 = -_9.0;
match _7 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_2 = (-2149179694939708953_i64) as f64;
_3 = _7 > _1;
_6 = _1 * _7;
_4 = 10317989337401232909_usize as f64;
_2 = -_4;
_19 = '\u{1de53}';
_13.0 = !13166_u16;
_7 = _6;
_8 = _3;
_13 = (35284_u16, _12.2, _10);
_17 = !(-160937057363354655078891304604667121333_i128);
_9 = (_16, _12.1, RET);
_12.2 = [(-2225914183484479875_i64),(-9127404618580512143_i64),5861067044444145554_i64,(-4205480210758115435_i64),4844615242418923573_i64,(-7598719677517590220_i64),(-1258236352079022660_i64)];
_13 = (8917_u16, _12.2, _10);
RET = [(-8379601607714316283_i64),2806491934518420938_i64,(-3939397182431892498_i64),(-1471727991031400491_i64),(-172004672717122701_i64),(-725007956727955794_i64),(-8198715050164840785_i64)];
_17 = _13.2 as i128;
_12.0 = _5 as f32;
_9 = (_16, _12.1, _13.1);
_9.1 = _12.1 + _12.1;
RET = _9.2;
_9 = (_16, _12.1, RET);
Goto(bb17)
}
bb17 = {
Call(_22 = dump_var(16_usize, 8_usize, Move(_8), 19_usize, Move(_19), 5_usize, Move(_5), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_22 = dump_var(16_usize, 6_usize, Move(_6), 13_usize, Move(_13), 23_usize, _23, 23_usize, _23), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: (f32, u64, [i64; 7]),mut _2: u64,mut _3: (f32, u64, [i64; 7]),mut _4: (f32, u64, [i64; 7]),mut _5: (f32, u64, [i64; 7]),mut _6: isize,mut _7: (f32, u64, [i64; 7]),mut _8: bool,mut _9: (f32, u64, [i64; 7]),mut _10: u16) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _11: char;
let _12: bool;
let _13: char;
let _14: [u16; 3];
let _15: f32;
let _16: u32;
let _17: Adt36;
let _18: (i64, i16, i8, [u16; 7]);
let _19: [i64; 7];
let _20: [u16; 7];
let _21: f64;
let _22: i8;
let _23: isize;
let _24: ();
let _25: ();
{
_7 = (_4.0, _2, _1.2);
_3.2 = [(-1216135240388819935_i64),(-536155168896368539_i64),8577285494076501665_i64,(-8771058323519152658_i64),(-8239947133615711235_i64),1691009692605522389_i64,(-1762859260552658454_i64)];
_5 = (_4.0, _7.1, _7.2);
_9 = _1;
_11 = '\u{8f759}';
_12 = _8;
_5 = _3;
_1.1 = _4.1 << _2;
RET = _5.2;
_12 = _8 ^ _8;
_13 = _11;
_8 = _12;
_13 = _11;
_1.2 = [7542870969427601188_i64,(-2904935755068774566_i64),(-9009643380555498615_i64),6318398783966845939_i64,1330484361421312503_i64,5616980809315054570_i64,(-2441839353730043930_i64)];
_5.1 = _6 as u64;
_9.1 = (-5817212904962231050_i64) as u64;
_9.1 = _2 * _5.1;
_1.2 = _4.2;
_4.0 = _1.0;
_9.1 = 924469801_u32 as u64;
Goto(bb1)
}
bb1 = {
_9.0 = -_4.0;
_5 = (_7.0, _1.1, _1.2);
_4 = (_5.0, _3.1, RET);
_10 = 11196_u16;
_1.0 = _4.0;
_8 = _12;
RET = [3103325422160608504_i64,(-7510935811216585192_i64),8693552079253126529_i64,5165371329412656596_i64,(-4790719460040875811_i64),6506367699359939609_i64,7151823839426179084_i64];
_4.0 = (-149075075336022989849978890418163554602_i128) as f32;
_5 = (_9.0, _4.1, _1.2);
_5.0 = _4.0 * _9.0;
_7.2 = [1786826205253879792_i64,2467023591600135630_i64,(-194605073822059657_i64),3029472980969621348_i64,2954704028297711260_i64,(-3467785482226556736_i64),(-5637350503258917608_i64)];
_3.0 = _9.0;
_4.0 = _10 as f32;
_4.2 = [(-4892863246201976070_i64),(-7369769025442743861_i64),351577624239709321_i64,4224428655119735376_i64,(-8725443813504246432_i64),(-712960801364797127_i64),4665809556671121560_i64];
_1.1 = !_7.1;
_9.2 = [(-5152734766991260067_i64),6166236090599801698_i64,(-2130564184632771532_i64),4783540578875151358_i64,4499808419438641715_i64,(-3935196106970587836_i64),(-9141807070546599294_i64)];
_3.1 = !_4.1;
_1.0 = (-8373703713621758932_i64) as f32;
_13 = _11;
_7.2 = _4.2;
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
11196 => bb7,
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
_3.1 = _6 as u64;
_4.1 = _5.1 << _5.1;
_7.0 = _5.0;
_4.0 = 109_u8 as f32;
_1.2 = _3.2;
_18.0 = (-8239585013209005028_i64) ^ (-6586854240343427405_i64);
_7.0 = -_3.0;
_16 = 4164685094_u32 + 1643123614_u32;
_4.1 = !_7.1;
_3 = _5;
RET = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_1 = (_7.0, _3.1, RET);
_17.fld1 = [_10,_10,_10];
_7.2 = _4.2;
_18.1 = -2814_i16;
RET = _3.2;
_18.3 = [_10,_10,_10,_10,_10,_10,_10];
_10 = !34801_u16;
_12 = _8 | _8;
_4.1 = _2;
_4.1 = _7.1;
_7.0 = _5.0 * _3.0;
_4.2 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_17.fld2 = _18.1 >> _1.1;
_1.0 = _17.fld2 as f32;
RET = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_3.1 = 176_u8 as u64;
_1.0 = _18.0 as f32;
_18.2 = 119_i8;
match _18.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
119 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
Goto(bb11)
}
bb11 = {
_21 = 35_u8 as f64;
_15 = _1.0 * _5.0;
_10 = 32477_u16 >> _16;
_5 = (_3.0, _1.1, _7.2);
_4.0 = 189790986879315801248634790022363998908_u128 as f32;
match _18.2 {
0 => bb10,
1 => bb12,
119 => bb14,
_ => bb13
}
}
bb12 = {
Goto(bb11)
}
bb13 = {
_3.1 = _6 as u64;
_4.1 = _5.1 << _5.1;
_7.0 = _5.0;
_4.0 = 109_u8 as f32;
_1.2 = _3.2;
_18.0 = (-8239585013209005028_i64) ^ (-6586854240343427405_i64);
_7.0 = -_3.0;
_16 = 4164685094_u32 + 1643123614_u32;
_4.1 = !_7.1;
_3 = _5;
RET = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_1 = (_7.0, _3.1, RET);
_17.fld1 = [_10,_10,_10];
_7.2 = _4.2;
_18.1 = -2814_i16;
RET = _3.2;
_18.3 = [_10,_10,_10,_10,_10,_10,_10];
_10 = !34801_u16;
_12 = _8 | _8;
_4.1 = _2;
_4.1 = _7.1;
_7.0 = _5.0 * _3.0;
_4.2 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_17.fld2 = _18.1 >> _1.1;
_1.0 = _17.fld2 as f32;
RET = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_3.1 = 176_u8 as u64;
_1.0 = _18.0 as f32;
_18.2 = 119_i8;
match _18.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
119 => bb10,
_ => bb9
}
}
bb14 = {
_3.0 = 4109920807957765524990866757146190495_i128 as f32;
_7.1 = 209_u8 as u64;
_12 = !_8;
_3 = _1;
_16 = 2323338410_u32 << _5.1;
_14 = [_10,_10,_10];
_9.1 = _1.1 & _1.1;
_1.1 = _17.fld2 as u64;
_19 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_4.1 = _11 as u64;
_15 = _18.2 as f32;
_4.0 = _15;
_12 = _8;
_17.fld2 = _18.1;
_18.1 = 1990792930_i32 as i16;
_5.0 = _7.0;
RET = _5.2;
_3.0 = _15 + _5.0;
_3.2 = _7.2;
_14 = [_10,_10,_10];
_19 = _7.2;
_7 = _5;
_1.1 = _7.1 * _3.1;
_17.fld0 = 23719542581946650993327034376837499410_i128;
_17 = Adt36 { fld0: 80403512268900144303529574120443245659_i128,fld1: _14,fld2: _18.1 };
_17 = Adt36 { fld0: (-72260460208159825399098424617433057808_i128),fld1: _14,fld2: _18.1 };
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(17_usize, 11_usize, Move(_11), 14_usize, Move(_14), 13_usize, Move(_13), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(17_usize, 12_usize, Move(_12), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [u16; 3],mut _2: u64,mut _3: [i64; 7]) -> i64 {
mir! {
type RET = i64;
let _4: bool;
let _5: Adt30;
let _6: i16;
let _7: u16;
let _8: &'static &'static (i8,);
let _9: isize;
let _10: isize;
let _11: &'static &'static i32;
let _12: [i32; 4];
let _13: &'static (*mut [u8; 7],);
let _14: (f32, u64, [i64; 7]);
let _15: bool;
let _16: *mut [u64; 7];
let _17: *const &'static &'static i32;
let _18: *const usize;
let _19: &'static &'static u16;
let _20: &'static (i8,);
let _21: ((i8,), (*const i64, Adt36));
let _22: *const &'static &'static i32;
let _23: bool;
let _24: [i8; 5];
let _25: Adt30;
let _26: ();
let _27: ();
{
_4 = !true;
_1 = [33884_u16,15726_u16,11101_u16];
_2 = 18140243667966843316_u64 ^ 10488919265679032630_u64;
RET = -(-4966321149318950597_i64);
RET = (-1978786915304602304_i64);
_4 = _2 == _2;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_4 = true;
RET = 536561818340711874_i64 << _2;
_4 = !true;
_2 = !7335120459973411760_u64;
RET = !7044048809008412824_i64;
RET = 45132_u16 as i64;
RET = (-8155572796331748803_i64);
RET = !1106082538070297880_i64;
_4 = _2 > _2;
RET = (-88719785444613651396348730882450398724_i128) as i64;
RET = !(-4681863646644930962_i64);
_3 = [RET,RET,RET,RET,RET,RET,RET];
_2 = 737200781344655651_u64;
_4 = !true;
RET = (-6485837113541594502_i64) & 5296394588688715058_i64;
_2 = !15102970154417953385_u64;
Goto(bb1)
}
bb1 = {
RET = 55385236051253295067714092706829243023_u128 as i64;
_6 = !(-28436_i16);
_3 = [RET,RET,RET,RET,RET,RET,RET];
_3 = [RET,RET,RET,RET,RET,RET,RET];
_4 = !true;
_7 = '\u{8242e}' as u16;
_7 = !25912_u16;
_5 = Adt30::Variant1 { fld0: 121_u8 };
_9 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
place!(Field::<u8>(Variant(_5, 1), 0)) = RET as u8;
_7 = 20805_u16;
_6 = (-11069_i16) << _2;
_6 = 18510_i16;
place!(Field::<u8>(Variant(_5, 1), 0)) = 88_u8 & 87_u8;
_2 = !1285101017426198382_u64;
Goto(bb2)
}
bb2 = {
place!(Field::<u8>(Variant(_5, 1), 0)) = 128_u8;
RET = (-597131287203694995_i64);
RET = 2475445990333599078_usize as i64;
_2 = 5445927434575078794_u64 + 8373429960301543759_u64;
_6 = (-22253_i16) | 20133_i16;
_1 = [_7,_7,_7];
place!(Field::<u8>(Variant(_5, 1), 0)) = 49_u8 + 235_u8;
_10 = _7 as isize;
_2 = 17562152553851792677_u64;
SetDiscriminant(_5, 3);
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [101_u8,92_u8];
_6 = !9797_i16;
_10 = 132912989851668056377357373010910303547_u128 as isize;
_2 = RET as u64;
RET = (-4116281101723232033_i64) - (-5921152364226492362_i64);
_3 = [RET,RET,RET,RET,RET,RET,RET];
place!(Field::<u8>(Variant(_5, 3), 0)) = 33_u8;
_12 = [(-1034249997_i32),(-664414396_i32),692407546_i32,(-313690570_i32)];
place!(Field::<u8>(Variant(_5, 3), 0)) = !120_u8;
RET = !5707752381495055177_i64;
RET = !6105990858850902963_i64;
Goto(bb3)
}
bb3 = {
_2 = 125_i8 as u64;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_6 = 424787744_i32 as i16;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_14.1 = 1901065025_i32 as u64;
_7 = 40208_u16 | 56309_u16;
_12 = [1258886435_i32,(-1228727860_i32),135357434_i32,(-889274745_i32)];
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_15 = !_4;
_7 = 46391_u16;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_14.0 = Field::<u8>(Variant(_5, 3), 0) as f32;
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
_6 = RET as i16;
_9 = _6 as isize;
Goto(bb4)
}
bb4 = {
_6 = -26357_i16;
SetDiscriminant(_5, 3);
_4 = _15 ^ _15;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [143_u8,252_u8];
_17 = core::ptr::addr_of!(_11);
_4 = !_15;
_6 = 13957_i16 | 16355_i16;
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
_4 = RET <= RET;
_14.0 = (-76_i8) as f32;
_12 = [640939264_i32,(-625922260_i32),1793726078_i32,(-1667751993_i32)];
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [1_u8,226_u8];
_14.1 = !_2;
match _7 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
46391 => bb8,
_ => bb7
}
}
bb5 = {
_2 = 125_i8 as u64;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_6 = 424787744_i32 as i16;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_14.1 = 1901065025_i32 as u64;
_7 = 40208_u16 | 56309_u16;
_12 = [1258886435_i32,(-1228727860_i32),135357434_i32,(-889274745_i32)];
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_15 = !_4;
_7 = 46391_u16;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_14.0 = Field::<u8>(Variant(_5, 3), 0) as f32;
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
_6 = RET as i16;
_9 = _6 as isize;
Goto(bb4)
}
bb6 = {
place!(Field::<u8>(Variant(_5, 1), 0)) = 128_u8;
RET = (-597131287203694995_i64);
RET = 2475445990333599078_usize as i64;
_2 = 5445927434575078794_u64 + 8373429960301543759_u64;
_6 = (-22253_i16) | 20133_i16;
_1 = [_7,_7,_7];
place!(Field::<u8>(Variant(_5, 1), 0)) = 49_u8 + 235_u8;
_10 = _7 as isize;
_2 = 17562152553851792677_u64;
SetDiscriminant(_5, 3);
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [101_u8,92_u8];
_6 = !9797_i16;
_10 = 132912989851668056377357373010910303547_u128 as isize;
_2 = RET as u64;
RET = (-4116281101723232033_i64) - (-5921152364226492362_i64);
_3 = [RET,RET,RET,RET,RET,RET,RET];
place!(Field::<u8>(Variant(_5, 3), 0)) = 33_u8;
_12 = [(-1034249997_i32),(-664414396_i32),692407546_i32,(-313690570_i32)];
place!(Field::<u8>(Variant(_5, 3), 0)) = !120_u8;
RET = !5707752381495055177_i64;
RET = !6105990858850902963_i64;
Goto(bb3)
}
bb7 = {
RET = 55385236051253295067714092706829243023_u128 as i64;
_6 = !(-28436_i16);
_3 = [RET,RET,RET,RET,RET,RET,RET];
_3 = [RET,RET,RET,RET,RET,RET,RET];
_4 = !true;
_7 = '\u{8242e}' as u16;
_7 = !25912_u16;
_5 = Adt30::Variant1 { fld0: 121_u8 };
_9 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
place!(Field::<u8>(Variant(_5, 1), 0)) = RET as u8;
_7 = 20805_u16;
_6 = (-11069_i16) << _2;
_6 = 18510_i16;
place!(Field::<u8>(Variant(_5, 1), 0)) = 88_u8 & 87_u8;
_2 = !1285101017426198382_u64;
Goto(bb2)
}
bb8 = {
_17 = core::ptr::addr_of!((*_17));
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
_12 = [99656352_i32,490379675_i32,1935434138_i32,(-1742120727_i32)];
_17 = core::ptr::addr_of!((*_17));
_17 = core::ptr::addr_of!((*_17));
_4 = _15;
_8 = &_20;
_7 = 21_u8 as u16;
_1 = [_7,_7,_7];
_2 = _14.1 | _14.1;
_5 = Adt30::Variant1 { fld0: 204_u8 };
_10 = -_9;
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb9)
}
bb9 = {
_8 = &(*_8);
_1 = [_7,_7,_7];
_10 = 3869925945_u32 as isize;
_21.1.0 = core::ptr::addr_of!(RET);
Call(_4 = fn19(_2, _3, _12, Move(_21.1.0), _14, _10), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_20 = &_21.0;
place!(Field::<u8>(Variant(_5, 1), 0)) = 152_u8 * 175_u8;
_21.1.1 = Adt36 { fld0: (-43568884016228883587910349111260484908_i128),fld1: _1,fld2: _6 };
_10 = _9;
match _21.1.1.fld0 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb11,
5 => bb12,
6 => bb13,
296713482904709579875464258320507726548 => bb15,
_ => bb14
}
}
bb11 = {
place!(Field::<u8>(Variant(_5, 1), 0)) = 128_u8;
RET = (-597131287203694995_i64);
RET = 2475445990333599078_usize as i64;
_2 = 5445927434575078794_u64 + 8373429960301543759_u64;
_6 = (-22253_i16) | 20133_i16;
_1 = [_7,_7,_7];
place!(Field::<u8>(Variant(_5, 1), 0)) = 49_u8 + 235_u8;
_10 = _7 as isize;
_2 = 17562152553851792677_u64;
SetDiscriminant(_5, 3);
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [101_u8,92_u8];
_6 = !9797_i16;
_10 = 132912989851668056377357373010910303547_u128 as isize;
_2 = RET as u64;
RET = (-4116281101723232033_i64) - (-5921152364226492362_i64);
_3 = [RET,RET,RET,RET,RET,RET,RET];
place!(Field::<u8>(Variant(_5, 3), 0)) = 33_u8;
_12 = [(-1034249997_i32),(-664414396_i32),692407546_i32,(-313690570_i32)];
place!(Field::<u8>(Variant(_5, 3), 0)) = !120_u8;
RET = !5707752381495055177_i64;
RET = !6105990858850902963_i64;
Goto(bb3)
}
bb12 = {
_17 = core::ptr::addr_of!((*_17));
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
_12 = [99656352_i32,490379675_i32,1935434138_i32,(-1742120727_i32)];
_17 = core::ptr::addr_of!((*_17));
_17 = core::ptr::addr_of!((*_17));
_4 = _15;
_8 = &_20;
_7 = 21_u8 as u16;
_1 = [_7,_7,_7];
_2 = _14.1 | _14.1;
_5 = Adt30::Variant1 { fld0: 204_u8 };
_10 = -_9;
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb9)
}
bb13 = {
RET = 55385236051253295067714092706829243023_u128 as i64;
_6 = !(-28436_i16);
_3 = [RET,RET,RET,RET,RET,RET,RET];
_3 = [RET,RET,RET,RET,RET,RET,RET];
_4 = !true;
_7 = '\u{8242e}' as u16;
_7 = !25912_u16;
_5 = Adt30::Variant1 { fld0: 121_u8 };
_9 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
place!(Field::<u8>(Variant(_5, 1), 0)) = RET as u8;
_7 = 20805_u16;
_6 = (-11069_i16) << _2;
_6 = 18510_i16;
place!(Field::<u8>(Variant(_5, 1), 0)) = 88_u8 & 87_u8;
_2 = !1285101017426198382_u64;
Goto(bb2)
}
bb14 = {
_2 = 125_i8 as u64;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_6 = 424787744_i32 as i16;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_14.1 = 1901065025_i32 as u64;
_7 = 40208_u16 | 56309_u16;
_12 = [1258886435_i32,(-1228727860_i32),135357434_i32,(-889274745_i32)];
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_15 = !_4;
_7 = 46391_u16;
place!(Field::<[u8; 2]>(Variant(_5, 3), 1)) = [Field::<u8>(Variant(_5, 3), 0),Field::<u8>(Variant(_5, 3), 0)];
_14.0 = Field::<u8>(Variant(_5, 3), 0) as f32;
_14.2 = [RET,RET,RET,RET,RET,RET,RET];
_6 = RET as i16;
_9 = _6 as isize;
Goto(bb4)
}
bb15 = {
_10 = _9;
_21.1.0 = core::ptr::addr_of!(RET);
_20 = &(*_20);
_20 = &_21.0;
_23 = _4 < _4;
_8 = &_20;
_21.1.1.fld0 = (-55646194941770152166612693133155752042_i128);
_1 = [_7,_7,_7];
_21.0 = (9_i8,);
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(18_usize, 12_usize, Move(_12), 2_usize, Move(_2), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(18_usize, 4_usize, Move(_4), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: u64,mut _2: [i64; 7],mut _3: [i32; 4],mut _4: *const i64,mut _5: (f32, u64, [i64; 7]),mut _6: isize) -> bool {
mir! {
type RET = bool;
let _7: isize;
let _8: bool;
let _9: isize;
let _10: bool;
let _11: f64;
let _12: *const f32;
let _13: &'static u8;
let _14: f64;
let _15: [usize; 5];
let _16: char;
let _17: (i64, i16, i8, [u16; 7]);
let _18: isize;
let _19: bool;
let _20: [u8; 8];
let _21: [u16; 3];
let _22: [u8; 2];
let _23: [usize; 5];
let _24: bool;
let _25: usize;
let _26: u16;
let _27: [u8; 5];
let _28: [u8; 5];
let _29: [i64; 7];
let _30: Adt21;
let _31: [i64; 7];
let _32: f32;
let _33: isize;
let _34: ();
let _35: ();
{
RET = true ^ true;
RET = _6 >= _6;
_7 = _6;
_2 = [787598796665489534_i64,1136371151356926851_i64,559665291918540813_i64,(-2758975542641082957_i64),1832094386492499464_i64,(-8927401966245496976_i64),(-1910924132690975182_i64)];
_1 = _5.1 ^ _5.1;
_5.1 = !_1;
RET = false;
_3 = [1956067563_i32,460866780_i32,837225396_i32,771045090_i32];
_3 = [499221743_i32,1331008949_i32,(-1306884651_i32),1710970700_i32];
_6 = !_7;
_5.2 = _2;
_5.2 = [2656801962211227099_i64,(-4008192960810555867_i64),(-3559289317348544043_i64),(-4689381443264860911_i64),(-4196067499615715568_i64),(-6041615555012862665_i64),(-8740014500914322298_i64)];
_5.2 = _2;
_2 = [7088648556969958527_i64,(-4899100780781368364_i64),8739099780706244497_i64,(-5550119175379774711_i64),2783557571848945621_i64,(-6020266254921842097_i64),3681413642398984477_i64];
_8 = !RET;
_1 = _6 as u64;
Goto(bb1)
}
bb1 = {
_1 = !_5.1;
RET = _8 ^ _8;
_6 = _7;
_5.2 = [(-3959669146894781691_i64),865504818044971373_i64,6640150188691151286_i64,5352527219568619077_i64,(-3258939687937783112_i64),3299405374161032704_i64,(-1698972499790826392_i64)];
_6 = !_7;
_6 = -_7;
_7 = 2279695334_u32 as isize;
RET = _8;
_5.0 = _1 as f32;
RET = !_8;
_2 = _5.2;
RET = !_8;
RET = _5.0 > _5.0;
_1 = !_5.1;
_5.0 = _5.1 as f32;
RET = _8;
_5.0 = 20940_u16 as f32;
_5.0 = 285426670727690340311025703860403129362_u128 as f32;
_5.0 = _7 as f32;
_2 = [1035975969865275413_i64,(-9067895165297914397_i64),(-6267519990942000479_i64),(-9184262671728707126_i64),2294220431300026864_i64,1902531599286900688_i64,2974679904103430888_i64];
_7 = -_6;
RET = _8;
_6 = _7 << _5.1;
Goto(bb2)
}
bb2 = {
_10 = RET;
_11 = 49259_u16 as f64;
_6 = _7;
_3 = [1859709667_i32,1859000404_i32,1979127553_i32,(-208937239_i32)];
_9 = _6;
_12 = core::ptr::addr_of!(_5.0);
RET = !_8;
_9 = _7 ^ _6;
(*_12) = 142403658419821286745323821911223588799_u128 as f32;
(*_12) = 2135405974_u32 as f32;
_12 = core::ptr::addr_of!(_5.0);
_14 = -_11;
Goto(bb3)
}
bb3 = {
_17.0 = -8444064984163251473_i64;
_5.1 = _1;
_12 = core::ptr::addr_of!((*_12));
_14 = 291150728741831133271815523950051888401_u128 as f64;
_5.1 = _1 - _1;
_15 = [4370052781126410397_usize,11502714062404780012_usize,4_usize,6_usize,11771863763442840487_usize];
_17.0 = 115590879315550025379379942658925742813_i128 as i64;
_17.3 = [44042_u16,33904_u16,15381_u16,24971_u16,17316_u16,15048_u16,19911_u16];
_17.1 = 21642_i16;
_16 = '\u{a0745}';
_5.2 = [_17.0,_17.0,_17.0,_17.0,_17.0,_17.0,_17.0];
_17.0 = _6 as i64;
_17.1 = -(-30285_i16);
_17.2 = (-38_i8);
_17.0 = (-5686104650936118875_i64) * 6758395515374859968_i64;
_18 = !_9;
_5.2 = _2;
_1 = 80737359848623098360330104341640738436_i128 as u64;
_1 = _5.1 ^ _5.1;
_12 = core::ptr::addr_of!(_5.0);
_10 = (*_12) >= (*_12);
_4 = core::ptr::addr_of!(_17.0);
_18 = _9 << _9;
_5.2 = [(*_4),(*_4),_17.0,_17.0,(*_4),_17.0,(*_4)];
RET = _8 | _8;
match _17.2 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211418 => bb6,
_ => bb5
}
}
bb4 = {
_10 = RET;
_11 = 49259_u16 as f64;
_6 = _7;
_3 = [1859709667_i32,1859000404_i32,1979127553_i32,(-208937239_i32)];
_9 = _6;
_12 = core::ptr::addr_of!(_5.0);
RET = !_8;
_9 = _7 ^ _6;
(*_12) = 142403658419821286745323821911223588799_u128 as f32;
(*_12) = 2135405974_u32 as f32;
_12 = core::ptr::addr_of!(_5.0);
_14 = -_11;
Goto(bb3)
}
bb5 = {
_1 = !_5.1;
RET = _8 ^ _8;
_6 = _7;
_5.2 = [(-3959669146894781691_i64),865504818044971373_i64,6640150188691151286_i64,5352527219568619077_i64,(-3258939687937783112_i64),3299405374161032704_i64,(-1698972499790826392_i64)];
_6 = !_7;
_6 = -_7;
_7 = 2279695334_u32 as isize;
RET = _8;
_5.0 = _1 as f32;
RET = !_8;
_2 = _5.2;
RET = !_8;
RET = _5.0 > _5.0;
_1 = !_5.1;
_5.0 = _5.1 as f32;
RET = _8;
_5.0 = 20940_u16 as f32;
_5.0 = 285426670727690340311025703860403129362_u128 as f32;
_5.0 = _7 as f32;
_2 = [1035975969865275413_i64,(-9067895165297914397_i64),(-6267519990942000479_i64),(-9184262671728707126_i64),2294220431300026864_i64,1902531599286900688_i64,2974679904103430888_i64];
_7 = -_6;
RET = _8;
_6 = _7 << _5.1;
Goto(bb2)
}
bb6 = {
_17.1 = 27946_i16 | 2981_i16;
(*_4) = !(-8955034025481492210_i64);
_15 = [15260904075110801972_usize,4656753011666439013_usize,16893168870747919595_usize,4_usize,7_usize];
_16 = '\u{53ecb}';
_17.0 = 6626381046556429195_i64;
match (*_4) {
0 => bb5,
6626381046556429195 => bb7,
_ => bb2
}
}
bb7 = {
(*_12) = _17.2 as f32;
(*_12) = (*_4) as f32;
_20 = [216_u8,191_u8,211_u8,60_u8,148_u8,58_u8,123_u8,196_u8];
_15 = [2_usize,69765797995210616_usize,1_usize,0_usize,6205735468003705789_usize];
_19 = _10;
RET = _10;
_3 = [9636513_i32,778349313_i32,1236886844_i32,1003431520_i32];
_12 = core::ptr::addr_of!((*_12));
_17.3 = [56838_u16,21032_u16,33085_u16,632_u16,11892_u16,26834_u16,54087_u16];
(*_4) = 8836184119242856122_i64;
_1 = _5.1 << (*_4);
_7 = -_18;
_20 = [125_u8,147_u8,221_u8,58_u8,92_u8,132_u8,104_u8,59_u8];
RET = !_19;
(*_4) = -(-574674455620897381_i64);
_9 = !_7;
_3 = [579028911_i32,1717257907_i32,200584840_i32,(-2105291837_i32)];
_17.1 = (-10981_i16);
RET = _10;
_8 = RET;
_22 = [85_u8,76_u8];
_18 = _17.2 as isize;
_17.3 = [40067_u16,49445_u16,56320_u16,61980_u16,13461_u16,39449_u16,37961_u16];
Call(_5.1 = core::intrinsics::transmute(_1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_4) = _17.2 as i64;
_17.1 = 4364_i16;
_7 = _6;
RET = !_19;
_4 = core::ptr::addr_of!(_17.0);
_23 = _15;
Goto(bb9)
}
bb9 = {
(*_12) = _17.1 as f32;
_2 = _5.2;
_2 = [(*_4),(*_4),_17.0,(*_4),(*_4),(*_4),(*_4)];
_17.3 = [16047_u16,43234_u16,41556_u16,25659_u16,6351_u16,45647_u16,59394_u16];
_21 = [12517_u16,41167_u16,41795_u16];
_5.0 = _5.1 as f32;
(*_12) = _14 as f32;
_16 = '\u{31d85}';
_15 = _23;
match _17.1 {
0 => bb1,
1 => bb3,
4364 => bb11,
_ => bb10
}
}
bb10 = {
_1 = !_5.1;
RET = _8 ^ _8;
_6 = _7;
_5.2 = [(-3959669146894781691_i64),865504818044971373_i64,6640150188691151286_i64,5352527219568619077_i64,(-3258939687937783112_i64),3299405374161032704_i64,(-1698972499790826392_i64)];
_6 = !_7;
_6 = -_7;
_7 = 2279695334_u32 as isize;
RET = _8;
_5.0 = _1 as f32;
RET = !_8;
_2 = _5.2;
RET = !_8;
RET = _5.0 > _5.0;
_1 = !_5.1;
_5.0 = _5.1 as f32;
RET = _8;
_5.0 = 20940_u16 as f32;
_5.0 = 285426670727690340311025703860403129362_u128 as f32;
_5.0 = _7 as f32;
_2 = [1035975969865275413_i64,(-9067895165297914397_i64),(-6267519990942000479_i64),(-9184262671728707126_i64),2294220431300026864_i64,1902531599286900688_i64,2974679904103430888_i64];
_7 = -_6;
RET = _8;
_6 = _7 << _5.1;
Goto(bb2)
}
bb11 = {
_5.0 = 3735486906_u32 as f32;
_14 = _11 - _11;
(*_12) = 379_u16 as f32;
_14 = -_11;
_7 = _6;
_4 = core::ptr::addr_of!((*_4));
_1 = _8 as u64;
RET = _9 == _9;
_25 = 5149232668015205901872101509806129051_i128 as usize;
_25 = 4_usize;
_13 = &_20[_25];
_5.0 = 12468792159215645881606990085901424747_u128 as f32;
_17.3[_25] = _5.0 as u16;
_1 = 752138782_u32 as u64;
_24 = !RET;
match _25 {
0 => bb8,
1 => bb12,
2 => bb13,
3 => bb14,
5 => bb16,
6 => bb17,
4 => bb19,
_ => bb18
}
}
bb12 = {
_17.0 = -8444064984163251473_i64;
_5.1 = _1;
_12 = core::ptr::addr_of!((*_12));
_14 = 291150728741831133271815523950051888401_u128 as f64;
_5.1 = _1 - _1;
_15 = [4370052781126410397_usize,11502714062404780012_usize,4_usize,6_usize,11771863763442840487_usize];
_17.0 = 115590879315550025379379942658925742813_i128 as i64;
_17.3 = [44042_u16,33904_u16,15381_u16,24971_u16,17316_u16,15048_u16,19911_u16];
_17.1 = 21642_i16;
_16 = '\u{a0745}';
_5.2 = [_17.0,_17.0,_17.0,_17.0,_17.0,_17.0,_17.0];
_17.0 = _6 as i64;
_17.1 = -(-30285_i16);
_17.2 = (-38_i8);
_17.0 = (-5686104650936118875_i64) * 6758395515374859968_i64;
_18 = !_9;
_5.2 = _2;
_1 = 80737359848623098360330104341640738436_i128 as u64;
_1 = _5.1 ^ _5.1;
_12 = core::ptr::addr_of!(_5.0);
_10 = (*_12) >= (*_12);
_4 = core::ptr::addr_of!(_17.0);
_18 = _9 << _9;
_5.2 = [(*_4),(*_4),_17.0,_17.0,(*_4),_17.0,(*_4)];
RET = _8 | _8;
match _17.2 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211418 => bb6,
_ => bb5
}
}
bb13 = {
_10 = RET;
_11 = 49259_u16 as f64;
_6 = _7;
_3 = [1859709667_i32,1859000404_i32,1979127553_i32,(-208937239_i32)];
_9 = _6;
_12 = core::ptr::addr_of!(_5.0);
RET = !_8;
_9 = _7 ^ _6;
(*_12) = 142403658419821286745323821911223588799_u128 as f32;
(*_12) = 2135405974_u32 as f32;
_12 = core::ptr::addr_of!(_5.0);
_14 = -_11;
Goto(bb3)
}
bb14 = {
_1 = !_5.1;
RET = _8 ^ _8;
_6 = _7;
_5.2 = [(-3959669146894781691_i64),865504818044971373_i64,6640150188691151286_i64,5352527219568619077_i64,(-3258939687937783112_i64),3299405374161032704_i64,(-1698972499790826392_i64)];
_6 = !_7;
_6 = -_7;
_7 = 2279695334_u32 as isize;
RET = _8;
_5.0 = _1 as f32;
RET = !_8;
_2 = _5.2;
RET = !_8;
RET = _5.0 > _5.0;
_1 = !_5.1;
_5.0 = _5.1 as f32;
RET = _8;
_5.0 = 20940_u16 as f32;
_5.0 = 285426670727690340311025703860403129362_u128 as f32;
_5.0 = _7 as f32;
_2 = [1035975969865275413_i64,(-9067895165297914397_i64),(-6267519990942000479_i64),(-9184262671728707126_i64),2294220431300026864_i64,1902531599286900688_i64,2974679904103430888_i64];
_7 = -_6;
RET = _8;
_6 = _7 << _5.1;
Goto(bb2)
}
bb15 = {
(*_12) = _17.2 as f32;
(*_12) = (*_4) as f32;
_20 = [216_u8,191_u8,211_u8,60_u8,148_u8,58_u8,123_u8,196_u8];
_15 = [2_usize,69765797995210616_usize,1_usize,0_usize,6205735468003705789_usize];
_19 = _10;
RET = _10;
_3 = [9636513_i32,778349313_i32,1236886844_i32,1003431520_i32];
_12 = core::ptr::addr_of!((*_12));
_17.3 = [56838_u16,21032_u16,33085_u16,632_u16,11892_u16,26834_u16,54087_u16];
(*_4) = 8836184119242856122_i64;
_1 = _5.1 << (*_4);
_7 = -_18;
_20 = [125_u8,147_u8,221_u8,58_u8,92_u8,132_u8,104_u8,59_u8];
RET = !_19;
(*_4) = -(-574674455620897381_i64);
_9 = !_7;
_3 = [579028911_i32,1717257907_i32,200584840_i32,(-2105291837_i32)];
_17.1 = (-10981_i16);
RET = _10;
_8 = RET;
_22 = [85_u8,76_u8];
_18 = _17.2 as isize;
_17.3 = [40067_u16,49445_u16,56320_u16,61980_u16,13461_u16,39449_u16,37961_u16];
Call(_5.1 = core::intrinsics::transmute(_1), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
_17.1 = 27946_i16 | 2981_i16;
(*_4) = !(-8955034025481492210_i64);
_15 = [15260904075110801972_usize,4656753011666439013_usize,16893168870747919595_usize,4_usize,7_usize];
_16 = '\u{53ecb}';
_17.0 = 6626381046556429195_i64;
match (*_4) {
0 => bb5,
6626381046556429195 => bb7,
_ => bb2
}
}
bb17 = {
_1 = !_5.1;
RET = _8 ^ _8;
_6 = _7;
_5.2 = [(-3959669146894781691_i64),865504818044971373_i64,6640150188691151286_i64,5352527219568619077_i64,(-3258939687937783112_i64),3299405374161032704_i64,(-1698972499790826392_i64)];
_6 = !_7;
_6 = -_7;
_7 = 2279695334_u32 as isize;
RET = _8;
_5.0 = _1 as f32;
RET = !_8;
_2 = _5.2;
RET = !_8;
RET = _5.0 > _5.0;
_1 = !_5.1;
_5.0 = _5.1 as f32;
RET = _8;
_5.0 = 20940_u16 as f32;
_5.0 = 285426670727690340311025703860403129362_u128 as f32;
_5.0 = _7 as f32;
_2 = [1035975969865275413_i64,(-9067895165297914397_i64),(-6267519990942000479_i64),(-9184262671728707126_i64),2294220431300026864_i64,1902531599286900688_i64,2974679904103430888_i64];
_7 = -_6;
RET = _8;
_6 = _7 << _5.1;
Goto(bb2)
}
bb18 = {
_10 = RET;
_11 = 49259_u16 as f64;
_6 = _7;
_3 = [1859709667_i32,1859000404_i32,1979127553_i32,(-208937239_i32)];
_9 = _6;
_12 = core::ptr::addr_of!(_5.0);
RET = !_8;
_9 = _7 ^ _6;
(*_12) = 142403658419821286745323821911223588799_u128 as f32;
(*_12) = 2135405974_u32 as f32;
_12 = core::ptr::addr_of!(_5.0);
_14 = -_11;
Goto(bb3)
}
bb19 = {
_18 = _24 as isize;
_28[_25] = !(*_13);
_25 = _23[_25];
_5.2 = [_17.0,(*_4),(*_4),_17.0,(*_4),_17.0,(*_4)];
_17.2 = (-114_i8) ^ 93_i8;
(*_12) = 109380298035509535869058353637384682599_i128 as f32;
_16 = '\u{94a79}';
_28 = [(*_13),(*_13),(*_13),(*_13),(*_13)];
(*_12) = _17.2 as f32;
RET = _18 == _9;
_27 = [(*_13),(*_13),(*_13),(*_13),(*_13)];
_17.3 = [32591_u16,2801_u16,64490_u16,56922_u16,37556_u16,60785_u16,3861_u16];
(*_12) = 40952_u16 as f32;
_22 = [(*_13),(*_13)];
(*_12) = _17.1 as f32;
_17.0 = 3356243178_u32 as i64;
_5.2 = [(*_4),_17.0,_17.0,_17.0,_17.0,(*_4),_17.0];
_21 = [9280_u16,3384_u16,18371_u16];
_17.1 = (-23254_i16) - 12013_i16;
_15 = _23;
_24 = !RET;
Goto(bb20)
}
bb20 = {
Call(_34 = dump_var(19_usize, 7_usize, Move(_7), 25_usize, Move(_25), 16_usize, Move(_16), 27_usize, Move(_27)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_34 = dump_var(19_usize, 8_usize, Move(_8), 28_usize, Move(_28), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_34 = dump_var(19_usize, 24_usize, Move(_24), 15_usize, Move(_15), 35_usize, _35, 35_usize, _35), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{bc84b}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-64_i8)), std::hint::black_box((-32534_i16)), std::hint::black_box(2138107584935339784_u64), std::hint::black_box((-9098034393563999525_i64)), std::hint::black_box((-140174088382296854957832072793067775599_i128)), std::hint::black_box(60581_u16));
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){unsafe{printf("Adt21::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt21 {
Variant0{
fld0: bool,
fld1: [u16; 7],

},
Variant1{
fld0: i128,
fld1: i16,
fld2: isize,
fld3: u32,

}}
impl PrintFDebug for Adt30{
	unsafe fn printf_debug(&self){unsafe{printf("Adt30::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt30 {
Variant0{
fld0: (i8,),
fld1: [u16; 7],
fld2: *const i64,
fld3: usize,
fld4: i16,
fld5: [i64; 7],
fld6: i64,

},
Variant1{
fld0: u8,

},
Variant2{
fld0: u32,
fld1: char,
fld2: isize,
fld3: [u8; 7],

},
Variant3{
fld0: u8,
fld1: [u8; 2],

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt36{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt36 {
fld0: i128,
fld1: [u16; 3],
fld2: i16,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *const f32,
fld1: [char; 2],
fld2: u8,
fld3: u32,
fld4: [u8; 7],

},
Variant1{
fld0: bool,
fld1: u32,
fld2: i16,
fld3: i32,

},
Variant2{
fld0: [char; 2],
fld1: char,
fld2: [u8; 7],

},
Variant3{
fld0: [isize; 2],
fld1: *const f32,
fld2: [u8; 7],
fld3: *mut [u8; 7],
fld4: [i8; 5],
fld5: i32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i64; 7],

},
Variant1{
fld0: u8,
fld1: char,
fld2: *mut Adt42,
fld3: [i8; 5],

},
Variant2{
fld0: u8,
fld1: *const f32,
fld2: isize,
fld3: usize,
fld4: [i8; 5],

},
Variant3{
fld0: [u16; 7],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [u16; 3],
fld1: (u16, [i64; 7], u128),
fld2: i32,

},
Variant1{
fld0: f64,

},
Variant2{
fld0: bool,
fld1: [u16; 7],
fld2: *const f32,
fld3: i8,
fld4: i16,
fld5: *const usize,
fld6: f32,
fld7: u32,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: (*mut [u8; 7],),
fld1: [i64; 7],
fld2: (i8,),
fld3: *const usize,
fld4: [i8; 5],
fld5: [char; 2],
fld6: Adt36,
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [i32; 4],
fld1: f32,
fld2: [u16; 3],
fld3: *const i64,

},
Variant1{
fld0: u128,
fld1: Adt21,
fld2: f64,
fld3: i8,
fld4: usize,
fld5: *const i64,
fld6: [i64; 7],
fld7: [u8; 8],

}}

