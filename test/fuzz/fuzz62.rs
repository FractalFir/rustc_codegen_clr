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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u128,mut _9: usize,mut _10: u64,mut _11: u16,mut _12: u32) -> [i32; 5] {
mir! {
type RET = [i32; 5];
let _13: (i128, *const usize, u8);
let _14: isize;
let _15: u16;
let _16: f32;
let _17: [u16; 8];
let _18: i8;
let _19: i32;
let _20: isize;
let _21: i16;
let _22: &'static Adt63;
let _23: (Adt47, &'static u32, &'static u128, u64);
let _24: &'static i8;
let _25: isize;
let _26: isize;
let _27: [usize; 2];
let _28: &'static Adt47;
let _29: &'static (usize, i16);
let _30: i32;
let _31: u16;
let _32: &'static (i64, u8, i64);
let _33: *const u32;
let _34: f32;
let _35: usize;
let _36: *mut (i8, (i64, u8, i64), char, i128);
let _37: f64;
let _38: i32;
let _39: (usize, u128);
let _40: Adt58;
let _41: Adt30;
let _42: (Adt47, &'static u32, &'static u128, u64);
let _43: u16;
let _44: f32;
let _45: [i16; 8];
let _46: (i128, &'static (bool, (bool, u32, i32, bool), u128), u128, &'static &'static u128);
let _47: &'static u32;
let _48: [i16; 8];
let _49: &'static Adt63;
let _50: isize;
let _51: (i64, u8, i64);
let _52: &'static &'static u128;
let _53: ();
let _54: ();
{
RET = [1933424937_i32,(-1109452053_i32),(-776400280_i32),(-1229267783_i32),(-1404863064_i32)];
_12 = !210790843_u32;
_13.1 = core::ptr::addr_of!(_9);
_3 = !9223372036854775807_isize;
_11 = 46068_u16 ^ 13450_u16;
_13.1 = core::ptr::addr_of!(_9);
_6 = 1595865665923301516_i64 as i32;
_9 = 6916809884811375130_usize & 13478534066728724155_usize;
_5 = _12 as i16;
_10 = 7816359309300196409_u64;
_13.1 = core::ptr::addr_of!(_9);
_16 = _5 as f32;
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
7816359309300196409 => bb7,
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
_9 = 15204712174700911070_usize;
_10 = 236325194763475408_u64 & 6234530197796514126_u64;
_12 = !2155242377_u32;
_8 = !250686945660098655735500122878311694944_u128;
_5 = (-2403_i16);
_4 = -(-37_i8);
_12 = _16 as u32;
_7 = (-3612693256938436138_i64) ^ (-5796430482440823066_i64);
_15 = true as u16;
_2 = '\u{5360d}';
_7 = -(-1099520246720494388_i64);
_2 = '\u{66af4}';
_18 = -_4;
_14 = _9 as isize;
_13.2 = !80_u8;
_1 = _13.2 > _13.2;
_7 = 6474228452715225081_i64 * 1507918606045308712_i64;
_3 = -_14;
_6 = 1581235735_i32 >> _5;
_13.0 = -46937678894502854954234736753798743046_i128;
_17 = [_15,_15,_15,_15,_11,_15,_11,_11];
_4 = _18;
_16 = _13.0 as f32;
_11 = _15;
Goto(bb8)
}
bb8 = {
_14 = _1 as isize;
_15 = _11;
_7 = _15 as i64;
_1 = true;
_13.0 = _14 as i128;
_16 = _8 as f32;
_7 = (-1999641167658023590_i64) - 2603010612851613766_i64;
_12 = !775433188_u32;
_2 = '\u{18dad}';
_6 = 412323231_i32 * (-773776951_i32);
_1 = !false;
_16 = _14 as f32;
_18 = _4 + _4;
_9 = _13.2 as usize;
_10 = 2748594751205229465_u64 | 17181874615303847373_u64;
_21 = _5 & _5;
_20 = _14 * _3;
_4 = -_18;
_4 = !_18;
RET = [_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6];
_10 = !11662257841886648687_u64;
_6 = !(-56441647_i32);
_10 = 12881062821541225490_u64;
_23.2 = &_8;
Call(_14 = fn1(Move(_23.2)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_23.1 = &_12;
_13.1 = core::ptr::addr_of!(_9);
_23.3 = _10 << _5;
_10 = !_23.3;
_20 = _8 as isize;
_25 = _9 as isize;
_4 = !_18;
RET = [_6,_6,_6,_6,_6];
_12 = 2071762235_u32 | 481172604_u32;
_3 = _14 | _14;
_13.1 = core::ptr::addr_of!(_9);
_15 = _13.2 as u16;
_23.2 = &_8;
RET = [_6,_6,_6,_6,_6];
_26 = _3 << _15;
_26 = _3;
_24 = &_18;
_2 = '\u{103c14}';
Call(_6 = core::intrinsics::bswap((-1925538118_i32)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_21 = !_5;
RET = [_6,_6,_6,_6,_6];
_23.1 = &_12;
_11 = _15;
_2 = '\u{1d894}';
_3 = _26;
_24 = &(*_24);
_34 = _16;
_24 = &_18;
_28 = &_23.0;
_33 = core::ptr::addr_of!(_12);
_5 = _8 as i16;
_20 = !_3;
_19 = !_6;
_18 = _4;
_6 = _19;
_33 = core::ptr::addr_of!(_12);
_2 = '\u{79cf1}';
_16 = _34;
_33 = core::ptr::addr_of!((*_33));
_5 = -_21;
_34 = -_16;
Goto(bb11)
}
bb11 = {
_21 = _5 >> (*_33);
_39.0 = _9;
_14 = _20 << _18;
_28 = &(*_28);
_16 = _10 as f32;
_13.0 = (-165347858483794052328136864154362904621_i128) - 11119809917726306906647182361206585425_i128;
_35 = _9 | _39.0;
_15 = !_11;
_24 = &_4;
_13.0 = (-89076127183900750285474583356601355695_i128) ^ (-157759410451823596818756529373421063575_i128);
_23.1 = &(*_33);
_33 = core::ptr::addr_of!((*_33));
_30 = _6;
_30 = _6 | _19;
_39 = (_35, _8);
_7 = -2795184976483355156_i64;
_3 = _13.0 as isize;
_27 = [_39.0,_9];
_38 = _21 as i32;
_17 = [_15,_11,_11,_11,_15,_15,_15,_15];
_18 = (*_24) ^ _4;
_13.0 = _10 as i128;
_16 = _21 as f32;
_17 = [_15,_15,_11,_15,_11,_15,_15,_11];
_33 = core::ptr::addr_of!(_12);
Goto(bb12)
}
bb12 = {
RET = [_30,_38,_38,_38,_30];
_10 = _7 as u64;
_43 = _15 >> _14;
_37 = _23.3 as f64;
_31 = !_43;
_23.2 = &_39.1;
_25 = !_26;
_45 = [_5,_21,_21,_21,_21,_21,_21,_5];
_17 = [_43,_31,_43,_43,_11,_43,_43,_43];
_12 = _34 as u32;
_16 = _34 + _34;
_19 = _6 & _30;
Goto(bb13)
}
bb13 = {
_26 = _16 as isize;
_43 = _31 - _15;
_46.0 = _13.0 ^ _13.0;
_42.2 = &_8;
_13.1 = core::ptr::addr_of!(_35);
_16 = _37 as f32;
_42.3 = _23.3 & _23.3;
_9 = _1 as usize;
Goto(bb14)
}
bb14 = {
_37 = (*_24) as f64;
_20 = _14;
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(0_usize, 26_usize, Move(_26), 9_usize, Move(_9), 4_usize, Move(_4), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(0_usize, 35_usize, Move(_35), 1_usize, Move(_1), 31_usize, Move(_31), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(0_usize, 6_usize, Move(_6), 25_usize, Move(_25), 10_usize, Move(_10), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(0_usize, 17_usize, Move(_17), 3_usize, Move(_3), 54_usize, _54, 54_usize, _54), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: &'static u128) -> isize {
mir! {
type RET = isize;
let _2: &'static [u16; 4];
let _3: i16;
let _4: &'static (i64, u8, i64);
let _5: Adt47;
let _6: (((u16,),), Adt47, (bool, u32, i32, bool));
let _7: u128;
let _8: (usize, i16);
let _9: f64;
let _10: [u16; 4];
let _11: u16;
let _12: (bool, u32, i32, bool);
let _13: *const ([u16; 8],);
let _14: char;
let _15: u16;
let _16: isize;
let _17: &'static &'static u32;
let _18: u32;
let _19: isize;
let _20: ();
let _21: ();
{
RET = 152_u8 as isize;
RET = 9223372036854775807_isize + 9223372036854775807_isize;
RET = '\u{adf49}' as isize;
RET = 881764380_u32 as isize;
RET = (-40_isize) * (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
match RET {
0 => bb1,
340282366920938463454151235394913435648 => bb3,
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
_3 = 21049_i16;
RET = (-50_isize);
_3 = (-2029_i16) >> RET;
RET = (-9223372036854775808_isize);
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
RET = 107_isize << _3;
RET = 9223372036854775807_isize;
RET = 3606415420_u32 as isize;
RET = !9223372036854775807_isize;
_3 = !(-2271_i16);
_3 = (-9324_i16) * (-1826_i16);
RET = 3390496755897443918_usize as isize;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize >> _3;
_3 = !11985_i16;
RET = !9223372036854775807_isize;
_6.2 = (true, 2375376416_u32, 1532550773_i32, true);
_6.2 = (false, 1493921265_u32, (-704787670_i32), false);
_6.0.0.0 = !36537_u16;
match _6.2.1 {
0 => bb4,
1 => bb5,
1493921265 => bb7,
_ => bb6
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
RET = 257378058902295268_i64 as isize;
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_6.2 = (true, 2786558294_u32, (-2018266612_i32), false);
_6.2.0 = !_6.2.3;
RET = 102_isize * 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_6.2 = (true, 2428045543_u32, 527615261_i32, true);
RET = 9223372036854775807_isize;
_6.2.1 = !540441383_u32;
_6.2.0 = _6.2.3 != _6.2.3;
_6.0.0.0 = 37693_u16;
RET = (-95_isize) * 9223372036854775807_isize;
_6.0.0.0 = '\u{5fd1d}' as u16;
RET = -(-9223372036854775808_isize);
_6.2.3 = _6.2.2 < _6.2.2;
_6.2.2 = (-108369167_i32);
_6.0.0 = (44823_u16,);
_6.0.0 = (34345_u16,);
_6.2.2 = _6.2.3 as i32;
RET = 9010421101344127379_u64 as isize;
_1 = &_7;
_6.2.1 = 2703947984_u32 << _6.2.2;
Call(RET = fn2(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_6.0.0 = (41590_u16,);
RET = !(-46_isize);
_7 = 16763381170545207732007681273514599129_u128 - 263829929413918138798120779736756923192_u128;
_6.2 = (false, 2713133997_u32, 2005864856_i32, false);
_6.2 = (true, 709843871_u32, (-337494168_i32), false);
_6.2.1 = 925517027_u32;
_8.0 = !12271784510963223558_usize;
_6.0.0.0 = 46368_u16 << RET;
_6.2.3 = _6.2.1 != _6.2.1;
_8.1 = _3 | _3;
_3 = 8781595497575056640_i64 as i16;
_8.1 = !_3;
_6.0.0.0 = 37857_u16 + 19896_u16;
_1 = &_7;
match _6.2.2 {
0 => bb1,
1 => bb7,
2 => bb5,
3 => bb9,
4 => bb10,
5 => bb11,
340282366920938463463374607431430717288 => bb13,
_ => bb12
}
}
bb9 = {
RET = 257378058902295268_i64 as isize;
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_6.2 = (true, 2786558294_u32, (-2018266612_i32), false);
_6.2.0 = !_6.2.3;
RET = 102_isize * 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_6.2 = (true, 2428045543_u32, 527615261_i32, true);
RET = 9223372036854775807_isize;
_6.2.1 = !540441383_u32;
_6.2.0 = _6.2.3 != _6.2.3;
_6.0.0.0 = 37693_u16;
RET = (-95_isize) * 9223372036854775807_isize;
_6.0.0.0 = '\u{5fd1d}' as u16;
RET = -(-9223372036854775808_isize);
_6.2.3 = _6.2.2 < _6.2.2;
_6.2.2 = (-108369167_i32);
_6.0.0 = (44823_u16,);
_6.0.0 = (34345_u16,);
_6.2.2 = _6.2.3 as i32;
RET = 9010421101344127379_u64 as isize;
_1 = &_7;
_6.2.1 = 2703947984_u32 << _6.2.2;
Call(RET = fn2(), ReturnTo(bb8), UnwindUnreachable())
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
_6.2.2 = (-265108059_i32) | 1955099087_i32;
_6.2.0 = (*_1) == (*_1);
_11 = _6.0.0.0;
_12.0 = _6.2.0;
_14 = '\u{89199}';
match _6.2.1 {
0 => bb1,
1 => bb2,
2 => bb8,
925517027 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_3 = _14 as i16;
_8.0 = 4_usize & 17475347096235907970_usize;
_15 = _6.2.2 as u16;
_6.2.1 = 3757259915_u32;
_10 = [_15,_6.0.0.0,_6.0.0.0,_6.0.0.0];
RET = -46_isize;
_16 = 21_u8 as isize;
_12.1 = _6.2.1 % _6.2.1;
_9 = 83_u8 as f64;
RET = _16;
_10 = [_11,_11,_6.0.0.0,_6.0.0.0];
_14 = '\u{a70b7}';
_1 = &(*_1);
_1 = &(*_1);
_12.2 = _6.2.2;
_18 = _16 as u32;
_6.2.1 = _6.2.3 as u32;
_12.3 = _6.2.0;
_10 = [_11,_11,_15,_15];
_10 = [_11,_15,_11,_11];
_18 = 41_i8 as u32;
_6.2.2 = -_12.2;
_6.0.0 = (_11,);
_8.1 = (-2612044591706209783_i64) as i16;
_18 = _12.1;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(1_usize, 11_usize, Move(_11), 18_usize, Move(_18), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(1_usize, 3_usize, Move(_3), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2() -> isize {
mir! {
type RET = isize;
let _1: f64;
let _2: i8;
let _3: &'static &'static u32;
let _4: *mut (i8, (i64, u8, i64), char, i128);
let _5: &'static u64;
let _6: isize;
let _7: isize;
let _8: Adt47;
let _9: &'static &'static u128;
let _10: Adt47;
let _11: [isize; 6];
let _12: *const ([u16; 8],);
let _13: (bool, u32, i32, bool);
let _14: &'static Adt47;
let _15: char;
let _16: (i128, &'static (bool, (bool, u32, i32, bool), u128), u128, &'static &'static u128);
let _17: (bool, u32, i32, bool);
let _18: f32;
let _19: *const usize;
let _20: &'static (((u16,),), Adt47, (bool, u32, i32, bool));
let _21: (Adt47, &'static u32, &'static u128, u64);
let _22: &'static (((u16,),), Adt47, (bool, u32, i32, bool));
let _23: isize;
let _24: *mut i8;
let _25: (i8, (i64, u8, i64), char, i128);
let _26: (((u16,),), Adt47, (bool, u32, i32, bool));
let _27: Adt42;
let _28: isize;
let _29: isize;
let _30: ();
let _31: ();
{
RET = 9223372036854775807_isize >> 53124436080661613190348875505605635045_u128;
RET = 45640954529335048444826241167691038698_i128 as isize;
_1 = RET as f64;
RET = 3777_u16 as isize;
RET = !(-9223372036854775808_isize);
_2 = 39_i8;
_1 = (-73124486315031615407686102659777258075_i128) as f64;
_2 = 72_i8;
RET = 69_isize ^ (-9223372036854775808_isize);
RET = (-9223372036854775808_isize);
_1 = 1454551418_u32 as f64;
_1 = 1332021810_u32 as f64;
Goto(bb1)
}
bb1 = {
_1 = (-1520759166_i32) as f64;
_2 = RET as i8;
_1 = 31713_u16 as f64;
RET = false as isize;
RET = (-78_isize);
Call(RET = fn3(_2, _2, _1, _1, _1, _1, _2, _2, _2, _1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = 17_i8;
RET = !(-9223372036854775808_isize);
_2 = 4_i8;
RET = '\u{9b76f}' as isize;
RET = !(-9223372036854775808_isize);
_1 = 5_usize as f64;
RET = -(-9223372036854775808_isize);
_1 = 3_usize as f64;
_2 = -(-49_i8);
_1 = 51921_u16 as f64;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
Goto(bb3)
}
bb3 = {
RET = 3412746670191907534_usize as isize;
Goto(bb4)
}
bb4 = {
RET = 9223372036854775807_isize;
_1 = RET as f64;
_6 = !RET;
RET = _6;
Goto(bb5)
}
bb5 = {
_1 = 109604494844967809344285679514234517367_u128 as f64;
_2 = (-122_i8);
_7 = _6;
_1 = 30599_u16 as f64;
_2 = (-9_i8) | 2_i8;
_2 = 3084514982959290002_u64 as i8;
_1 = 18147756690688048414_usize as f64;
RET = 210_u8 as isize;
RET = _7;
_2 = !(-24_i8);
_1 = 33992_u16 as f64;
_6 = -RET;
_6 = RET;
_6 = -RET;
RET = -_7;
RET = _7;
_7 = 225607585376073428864034595161207540086_u128 as isize;
RET = -_7;
_7 = _6 * RET;
_6 = _7 << RET;
_1 = _2 as f64;
_7 = (-146279549593267509446006788308533876684_i128) as isize;
_2 = -(-75_i8);
_1 = 3_usize as f64;
Goto(bb6)
}
bb6 = {
RET = 3300610725_u32 as isize;
RET = _6 - _6;
_6 = 16100942502979859440_usize as isize;
Goto(bb7)
}
bb7 = {
RET = !_6;
_6 = 23636_u16 as isize;
_11 = [RET,_7,_6,_7,RET,_7];
_2 = -(-89_i8);
_13 = (true, 3918655054_u32, 1557268526_i32, false);
_11 = [_7,_7,_6,_7,RET,_6];
_13.2 = (-108118342_i32) ^ 143959472_i32;
_6 = RET + RET;
RET = _6;
_13.1 = _13.0 as u32;
_13.1 = 18279_i16 as u32;
_11 = [_6,_6,RET,_6,RET,RET];
_1 = 4223734666149970544_u64 as f64;
_6 = _7;
_14 = &_8;
RET = -_6;
RET = -_7;
_14 = &_10;
_13.2 = (-1828256262_i32);
_1 = 5014292687858897497_u64 as f64;
RET = -_6;
RET = _1 as isize;
_16.0 = _1 as i128;
RET = _6 & _6;
_7 = -RET;
_14 = &(*_14);
Call(_13.2 = core::intrinsics::bswap((-359506466_i32)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = _2 as f64;
_17.3 = !_13.0;
_15 = '\u{100879}';
_17.0 = _17.3;
RET = _7;
_6 = 5_usize as isize;
_11 = [_7,RET,RET,RET,_7,RET];
_15 = '\u{3acd5}';
_1 = 150_u8 as f64;
_13.2 = 853488127_i32 - 304673147_i32;
_16.2 = 48968905004903650485583917029043764567_u128 * 67790041324550310926545204490412393022_u128;
_17.0 = _17.3 & _13.3;
_13.1 = 4274958534_u32;
_15 = '\u{6c733}';
_6 = 142_u16 as isize;
_18 = 968736859291771982_u64 as f32;
Goto(bb9)
}
bb9 = {
_13.3 = !_17.3;
_21.1 = &_17.1;
_17 = _13;
_17.1 = _13.1 + _13.1;
_11 = [RET,_7,_6,RET,_6,RET];
_21.2 = &_16.2;
_16.3 = &_21.2;
_16.0 = !48229566068976873217126865020135545357_i128;
_14 = &_8;
_2 = 20_i8 - (-22_i8);
RET = _6;
_9 = Move(_16.3);
_2 = 14339704899352307456_usize as i8;
_16.2 = 28703477360983780498610102366759144977_u128;
_3 = &_21.1;
RET = (-8944_i16) as isize;
RET = _7;
_21.2 = &_16.2;
_17.0 = _17.1 == _13.1;
Goto(bb10)
}
bb10 = {
_13.1 = !_17.1;
RET = -_6;
_23 = _2 as isize;
_13.0 = !_13.3;
Goto(bb11)
}
bb11 = {
_9 = &_21.2;
_21.1 = &_17.1;
_25.0 = -_2;
_5 = &_21.3;
_25.1 = ((-6772370592484842037_i64), 26_u8, (-5023156921294428919_i64));
_21.2 = &_16.2;
_6 = _7 ^ _7;
_9 = &_21.2;
_3 = &_21.1;
_3 = &(*_3);
_14 = &(*_14);
_21.3 = !1985073839117622224_u64;
_13 = (_17.0, _17.1, _17.2, _17.3);
_4 = core::ptr::addr_of_mut!(_25);
_4 = core::ptr::addr_of_mut!(_25);
(*_4).3 = _16.0;
match (*_4).1.0 {
0 => bb4,
1 => bb9,
2 => bb5,
3 => bb12,
4 => bb13,
340282366920938463456602236839283369419 => bb15,
_ => bb14
}
}
bb12 = {
_1 = (-1520759166_i32) as f64;
_2 = RET as i8;
_1 = 31713_u16 as f64;
RET = false as isize;
RET = (-78_isize);
Call(RET = fn3(_2, _2, _1, _1, _1, _1, _2, _2, _2, _1, _1, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_2 = 17_i8;
RET = !(-9223372036854775808_isize);
_2 = 4_i8;
RET = '\u{9b76f}' as isize;
RET = !(-9223372036854775808_isize);
_1 = 5_usize as f64;
RET = -(-9223372036854775808_isize);
_1 = 3_usize as f64;
_2 = -(-49_i8);
_1 = 51921_u16 as f64;
RET = 9223372036854775807_isize * 9223372036854775807_isize;
Goto(bb3)
}
bb14 = {
_1 = 109604494844967809344285679514234517367_u128 as f64;
_2 = (-122_i8);
_7 = _6;
_1 = 30599_u16 as f64;
_2 = (-9_i8) | 2_i8;
_2 = 3084514982959290002_u64 as i8;
_1 = 18147756690688048414_usize as f64;
RET = 210_u8 as isize;
RET = _7;
_2 = !(-24_i8);
_1 = 33992_u16 as f64;
_6 = -RET;
_6 = RET;
_6 = -RET;
RET = -_7;
RET = _7;
_7 = 225607585376073428864034595161207540086_u128 as isize;
RET = -_7;
_7 = _6 * RET;
_6 = _7 << RET;
_1 = _2 as f64;
_7 = (-146279549593267509446006788308533876684_i128) as isize;
_2 = -(-75_i8);
_1 = 3_usize as f64;
Goto(bb6)
}
bb15 = {
(*_4).0 = (*_4).3 as i8;
(*_4).1.1 = 204_u8;
_26.0.0.0 = 53926_u16 & 57379_u16;
_26.2.1 = _13.1 << (*_4).0;
_21.3 = !9215359836096631893_u64;
(*_4).2 = _15;
_17.1 = _13.1;
(*_4).1.2 = (*_4).1.0;
_11 = [_23,_6,_23,_6,_7,_6];
_16.3 = &(*_9);
(*_4).0 = _2;
_7 = _6;
_17.3 = _13.3 ^ _13.0;
_26.2.3 = _17.3 & _17.3;
_13.3 = !_26.2.3;
_16.2 = 227121978620310900333633891747907249847_u128;
_24 = core::ptr::addr_of_mut!(_25.0);
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(2_usize, 2_usize, Move(_2), 25_usize, Move(_25), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i8,mut _2: i8,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: i8,mut _8: i8,mut _9: i8,mut _10: f64,mut _11: f64,mut _12: i8) -> isize {
mir! {
type RET = isize;
let _13: Adt42;
let _14: usize;
let _15: char;
let _16: u8;
let _17: i64;
let _18: (*const usize, [u8; 4], u16, ([u16; 8],));
let _19: f32;
let _20: &'static (((u16,),), Adt47, (bool, u32, i32, bool));
let _21: (i64, u8, i64);
let _22: [i64; 6];
let _23: i128;
let _24: &'static [i32; 7];
let _25: f32;
let _26: f64;
let _27: (i8, (i64, u8, i64), char, i128);
let _28: f64;
let _29: f64;
let _30: *mut ((u16,),);
let _31: [i8; 4];
let _32: [i16; 8];
let _33: ();
let _34: ();
{
_8 = _5 as i8;
RET = 11822589677822108298790930398865333022_i128 as isize;
_10 = _4 + _6;
_11 = RET as f64;
_5 = _10 * _10;
_5 = _4;
_12 = -_9;
_12 = _4 as i8;
Goto(bb1)
}
bb1 = {
_2 = _8;
_4 = -_3;
_2 = !_1;
_5 = _10;
Goto(bb2)
}
bb2 = {
_12 = 28034_u16 as i8;
_1 = _7 * _2;
_9 = !_1;
_2 = _1;
_14 = !10576048591692375928_usize;
_15 = '\u{1325d}';
_15 = '\u{297af}';
_5 = _4 - _10;
_8 = _9;
_10 = -_5;
_9 = _2 & _1;
_3 = 719325007_u32 as f64;
_9 = 149760719290215767997527924578732230299_u128 as i8;
_12 = _1 >> _14;
_9 = _12;
Goto(bb3)
}
bb3 = {
RET = -7_isize;
_8 = _1 ^ _1;
_8 = -_1;
_18.1 = [175_u8,243_u8,196_u8,112_u8];
RET = 9223372036854775807_isize;
_12 = _8;
_14 = 6_usize;
_18.3.0 = [24161_u16,38098_u16,57292_u16,42458_u16,23836_u16,64729_u16,22467_u16,8929_u16];
_3 = -_10;
_5 = _10;
_15 = '\u{b64de}';
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_14 = 14803727287622809085_usize * 11437086258090987021_usize;
_18.0 = core::ptr::addr_of!(_14);
_11 = -_10;
_17 = (-844994067352495564_i64);
_7 = _9 | _1;
_4 = _10;
_18.1 = [24_u8,158_u8,255_u8,61_u8];
_18.2 = 63882_u16;
_5 = RET as f64;
_17 = 3687843530381226795_i64;
_19 = 555460620_u32 as f32;
_21 = (_17, 19_u8, _17);
_18.1 = [_21.1,_21.1,_21.1,_21.1];
match _17 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
3687843530381226795 => bb7,
_ => bb6
}
}
bb4 = {
_12 = 28034_u16 as i8;
_1 = _7 * _2;
_9 = !_1;
_2 = _1;
_14 = !10576048591692375928_usize;
_15 = '\u{1325d}';
_15 = '\u{297af}';
_5 = _4 - _10;
_8 = _9;
_10 = -_5;
_9 = _2 & _1;
_3 = 719325007_u32 as f64;
_9 = 149760719290215767997527924578732230299_u128 as i8;
_12 = _1 >> _14;
_9 = _12;
Goto(bb3)
}
bb5 = {
_2 = _8;
_4 = -_3;
_2 = !_1;
_5 = _10;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_2 = _9 >> _12;
_18.2 = !10760_u16;
_21.1 = 142_u8 + 246_u8;
_4 = _10 - _6;
_1 = _2 + _2;
_21 = (_17, 197_u8, _17);
_19 = RET as f32;
match _21.2 {
0 => bb5,
1 => bb4,
2 => bb6,
3 => bb8,
4 => bb9,
3687843530381226795 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
_2 = _8;
_4 = -_3;
_2 = !_1;
_5 = _10;
Goto(bb2)
}
bb10 = {
_12 = 28034_u16 as i8;
_1 = _7 * _2;
_9 = !_1;
_2 = _1;
_14 = !10576048591692375928_usize;
_15 = '\u{1325d}';
_15 = '\u{297af}';
_5 = _4 - _10;
_8 = _9;
_10 = -_5;
_9 = _2 & _1;
_3 = 719325007_u32 as f64;
_9 = 149760719290215767997527924578732230299_u128 as i8;
_12 = _1 >> _14;
_9 = _12;
Goto(bb3)
}
bb11 = {
_26 = -_10;
_25 = 288935964444804581863460448395839785028_u128 as f32;
_16 = _21.1 >> _12;
_9 = !_1;
_18.2 = 18961_u16 << RET;
_22 = [_21.2,_21.0,_17,_17,_21.2,_21.2];
_18.0 = core::ptr::addr_of!(_14);
_4 = _26 * _11;
match _21.2 {
3687843530381226795 => bb12,
_ => bb6
}
}
bb12 = {
_2 = !_1;
_18.0 = core::ptr::addr_of!(_14);
_25 = -_19;
_21.2 = _21.0 << _9;
Goto(bb13)
}
bb13 = {
_27.2 = _15;
_14 = _18.2 as usize;
_23 = 164645682931434249606255181773395097672_i128 >> _2;
RET = 9223372036854775807_isize;
_28 = _11 - _26;
RET = 87_isize & 9223372036854775807_isize;
_27.1.1 = !_16;
_11 = _21.2 as f64;
_19 = -_25;
_29 = _27.1.1 as f64;
_21 = (_17, _16, _17);
_18.3.0 = [_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2];
_28 = _10;
_27.1.2 = _18.2 as i64;
_27.1.0 = 18093186028628433240_u64 as i64;
_21 = (_27.1.2, _27.1.1, _27.1.2);
_21 = (_17, _27.1.1, _27.1.2);
_3 = _25 as f64;
_18.0 = core::ptr::addr_of!(_14);
_10 = _29;
_27.1.0 = RET as i64;
_27.1.1 = _21.1;
_27.1 = _21;
_27.1 = (_21.2, _21.1, _21.2);
_4 = _10 * _11;
_27.1 = (_17, _16, _21.2);
Call(_18.0 = fn4(_7, _15), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_27.3 = _16 as i128;
_23 = _27.3 << RET;
_27.3 = !_23;
_27.3 = _27.1.2 as i128;
_18.0 = core::ptr::addr_of!(_14);
_27.2 = _15;
_10 = -_4;
_7 = _18.2 as i8;
_27.1 = _21;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(3_usize, 15_usize, Move(_15), 14_usize, Move(_14), 23_usize, Move(_23), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(3_usize, 17_usize, Move(_17), 16_usize, Move(_16), 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i8,mut _2: char) -> *const usize {
mir! {
type RET = *const usize;
let _3: *mut ((u16,),);
let _4: bool;
let _5: bool;
let _6: f32;
let _7: bool;
let _8: [char; 2];
let _9: f64;
let _10: [i8; 4];
let _11: (usize, u128);
let _12: isize;
let _13: isize;
let _14: *mut u16;
let _15: ([u16; 8],);
let _16: f64;
let _17: [i8; 8];
let _18: [i64; 6];
let _19: *const u32;
let _20: Adt24;
let _21: &'static (bool, (bool, u32, i32, bool), u128);
let _22: isize;
let _23: &'static i8;
let _24: &'static ((u16,),);
let _25: u128;
let _26: (i128, &'static (bool, (bool, u32, i32, bool), u128), u128, &'static &'static u128);
let _27: bool;
let _28: u8;
let _29: u64;
let _30: [u16; 4];
let _31: *mut u16;
let _32: isize;
let _33: &'static (i64, u8, i64);
let _34: *mut i8;
let _35: bool;
let _36: *mut [char; 2];
let _37: isize;
let _38: &'static [i32; 7];
let _39: ();
let _40: ();
{
_2 = '\u{6ed7b}';
_2 = '\u{4bc1d}';
_2 = '\u{911e7}';
_2 = '\u{69ba8}';
_2 = '\u{7cc9c}';
_4 = false & false;
_1 = 1_i8;
_4 = true ^ true;
_4 = false;
_2 = '\u{ab0d2}';
_1 = 45_i8 & 39_i8;
_2 = '\u{92625}';
_1 = 52_i8 | (-27_i8);
Call(_2 = fn5(_4, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = false;
_1 = !34_i8;
_2 = '\u{6e55f}';
_1 = !49_i8;
_4 = !true;
_2 = '\u{b068f}';
_4 = !false;
_2 = '\u{7b8dd}';
_2 = '\u{c3eb5}';
_1 = !(-123_i8);
_5 = _4;
_5 = !_4;
_5 = !_4;
_4 = !_5;
_2 = '\u{9b635}';
_5 = _4;
_2 = '\u{1040fa}';
_2 = '\u{e2659}';
_5 = _4;
_5 = _2 >= _2;
_2 = '\u{cb615}';
_2 = '\u{63ec6}';
_2 = '\u{1338b}';
Goto(bb2)
}
bb2 = {
_5 = !_4;
_4 = _5 ^ _5;
_1 = (-35_i8);
_1 = (-2698195990399025860_i64) as i8;
_2 = '\u{69067}';
_6 = _1 as f32;
_7 = _4;
_8 = [_2,_2];
_8 = [_2,_2];
_8 = [_2,_2];
_7 = !_4;
_5 = !_4;
_2 = '\u{6ebbb}';
_2 = '\u{cfeeb}';
_4 = _5;
_5 = !_7;
_5 = _7;
_5 = !_4;
_5 = _4 >= _7;
_5 = !_7;
_9 = 69970722161784245347129245228311456261_i128 as f64;
_5 = _7;
_4 = !_5;
_4 = _7;
_2 = '\u{aec28}';
_8 = [_2,_2];
Goto(bb3)
}
bb3 = {
_10 = [_1,_1,_1,_1];
_8 = [_2,_2];
_7 = _5;
_4 = !_5;
_11.0 = 1116970598378081665_usize;
_5 = !_4;
RET = core::ptr::addr_of!(_11.0);
(*RET) = 7_usize;
_12 = (-27235_i16) as isize;
RET = core::ptr::addr_of!((*RET));
_13 = _12;
_9 = _6 as f64;
_10 = [_1,_1,_1,_1];
_11 = (1_usize, 119512807018748386984248648102175161794_u128);
RET = core::ptr::addr_of!(_11.0);
_2 = '\u{d90ba}';
_15.0 = [58227_u16,21638_u16,5021_u16,49546_u16,45680_u16,9497_u16,8810_u16,50652_u16];
_7 = !_4;
RET = core::ptr::addr_of!(_11.0);
RET = core::ptr::addr_of!((*RET));
_9 = (-90443749907790140105350121662263440767_i128) as f64;
_16 = _9 - _9;
_11.0 = 7_usize;
Goto(bb4)
}
bb4 = {
RET = core::ptr::addr_of!((*RET));
(*RET) = 10308978381459380800_usize & 17409792519707018515_usize;
_11.1 = 268974308967713773662355529998649103100_u128 & 24114530045289213570086818411529144739_u128;
_7 = _5 & _5;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_11 = (8532774280930753048_usize, 147854342770100902200164655873104032426_u128);
(*RET) = !0_usize;
_1 = (-82_i8) ^ 87_i8;
(*RET) = 4760475197484410429_usize;
_5 = _7;
_18 = [840803197216935281_i64,(-9271036296526951_i64),2816983144889358098_i64,6236880539569744203_i64,4975956524777959788_i64,(-3817743472820830199_i64)];
RET = core::ptr::addr_of!(_11.0);
_7 = _4;
_8 = [_2,_2];
(*RET) = 1392663991377396340_usize;
(*RET) = 9109733391923004872_usize * 0_usize;
_18 = [9198484640634902201_i64,(-6688409645122147152_i64),9082787865932763186_i64,3856148657649877704_i64,(-8316896988103210380_i64),5443763077402183344_i64];
_11 = (4_usize, 242192143917058207710216707107491274443_u128);
_2 = '\u{ba0c9}';
_12 = -_13;
RET = core::ptr::addr_of!(_11.0);
_9 = _16 * _16;
_17 = [_1,_1,_1,_1,_1,_1,_1,_1];
_16 = -_9;
Call((*RET) = core::intrinsics::transmute(_12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = core::ptr::addr_of!((*RET));
(*RET) = _9 as usize;
RET = core::ptr::addr_of!(_11.0);
_6 = 1778892648_u32 as f32;
_11.1 = 39669763489196838026423766224781205511_u128;
_20.fld2 = -_12;
(*RET) = 0_usize;
(*RET) = 6862952190819262336_usize;
_5 = _7 | _7;
_15.0 = [14436_u16,40357_u16,234_u16,5474_u16,35165_u16,28657_u16,12534_u16,31170_u16];
_20.fld1 = _11.0 & (*RET);
_9 = _16;
_11.1 = 42226958708476138255281428421273142383_u128 + 159501133750195995205745981596461252945_u128;
_11 = (_20.fld1, 82729853783449313196132072717442551913_u128);
_20.fld0 = [143_u8,251_u8,130_u8,161_u8];
_11.0 = 6352320294253282889_i64 as usize;
_13 = -_12;
_11 = (_20.fld1, 106409318002939069651547850021018361391_u128);
match _11.1 {
0 => bb4,
1 => bb3,
2 => bb6,
106409318002939069651547850021018361391 => bb8,
_ => bb7
}
}
bb6 = {
_5 = !_4;
_4 = _5 ^ _5;
_1 = (-35_i8);
_1 = (-2698195990399025860_i64) as i8;
_2 = '\u{69067}';
_6 = _1 as f32;
_7 = _4;
_8 = [_2,_2];
_8 = [_2,_2];
_8 = [_2,_2];
_7 = !_4;
_5 = !_4;
_2 = '\u{6ebbb}';
_2 = '\u{cfeeb}';
_4 = _5;
_5 = !_7;
_5 = _7;
_5 = !_4;
_5 = _4 >= _7;
_5 = !_7;
_9 = 69970722161784245347129245228311456261_i128 as f64;
_5 = _7;
_4 = !_5;
_4 = _7;
_2 = '\u{aec28}';
_8 = [_2,_2];
Goto(bb3)
}
bb7 = {
_10 = [_1,_1,_1,_1];
_8 = [_2,_2];
_7 = _5;
_4 = !_5;
_11.0 = 1116970598378081665_usize;
_5 = !_4;
RET = core::ptr::addr_of!(_11.0);
(*RET) = 7_usize;
_12 = (-27235_i16) as isize;
RET = core::ptr::addr_of!((*RET));
_13 = _12;
_9 = _6 as f64;
_10 = [_1,_1,_1,_1];
_11 = (1_usize, 119512807018748386984248648102175161794_u128);
RET = core::ptr::addr_of!(_11.0);
_2 = '\u{d90ba}';
_15.0 = [58227_u16,21638_u16,5021_u16,49546_u16,45680_u16,9497_u16,8810_u16,50652_u16];
_7 = !_4;
RET = core::ptr::addr_of!(_11.0);
RET = core::ptr::addr_of!((*RET));
_9 = (-90443749907790140105350121662263440767_i128) as f64;
_16 = _9 - _9;
_11.0 = 7_usize;
Goto(bb4)
}
bb8 = {
_1 = -125_i8;
_2 = '\u{8a688}';
Goto(bb9)
}
bb9 = {
_22 = _12 ^ _12;
_7 = !_5;
_15.0 = [57911_u16,39779_u16,35075_u16,58818_u16,53892_u16,61150_u16,52875_u16,463_u16];
(*RET) = 4034_u16 as usize;
_20.fld1 = !(*RET);
(*RET) = !_20.fld1;
(*RET) = _20.fld1;
_18 = [2055175630351132193_i64,(-1843854255973635462_i64),(-669174833998596242_i64),4928238402370747386_i64,5974804681130287397_i64,1996028178911408356_i64];
_20.fld3 = !_1;
_18 = [7654215856009371243_i64,(-2052322273720223994_i64),1272944811766209871_i64,(-6772048265343665824_i64),(-8749715383953863101_i64),(-647688962701161972_i64)];
_5 = _7;
_20.fld3 = _1;
match _11.1 {
106409318002939069651547850021018361391 => bb10,
_ => bb6
}
}
bb10 = {
_25 = _11.1;
_20.fld2 = _13 << _11.0;
_7 = _4;
_23 = &_20.fld3;
_16 = -_9;
_13 = _20.fld2;
_29 = 16630750875216944494_u64;
_18 = [5315521669161792471_i64,8589178946887203485_i64,6821818965033962159_i64,326407450327274091_i64,4451079296475169365_i64,(-3484025131092183010_i64)];
_16 = _9;
_2 = '\u{97144}';
_26.2 = !_25;
_23 = &(*_23);
_23 = &_20.fld3;
Goto(bb11)
}
bb11 = {
_22 = (-2670471875517636115_i64) as isize;
_17 = [_20.fld3,(*_23),(*_23),_20.fld3,_1,(*_23),(*_23),_1];
_27 = _29 <= _29;
RET = core::ptr::addr_of!((*RET));
_7 = _11.1 != _11.1;
_26.0 = 123297046435577762089690871389248041919_i128;
_10 = [(*_23),(*_23),(*_23),_1];
_11 = (_20.fld1, _26.2);
RET = core::ptr::addr_of!(_11.0);
_2 = '\u{a8ca0}';
_27 = !_5;
_30 = [37514_u16,16420_u16,52423_u16,56181_u16];
(*RET) = _20.fld1;
RET = core::ptr::addr_of!((*RET));
_32 = _13 ^ _22;
_1 = (*_23);
_18 = [1797036364872942245_i64,(-8896710653999938542_i64),1912952629130939607_i64,9203560768413356590_i64,7730068413944265923_i64,(-6831831730479341011_i64)];
_27 = !_7;
_4 = _16 <= _9;
_10 = [(*_23),_1,(*_23),_20.fld3];
_34 = core::ptr::addr_of_mut!((*_23));
match _29 {
0 => bb7,
1 => bb9,
2 => bb12,
3 => bb13,
16630750875216944494 => bb15,
_ => bb14
}
}
bb12 = {
_10 = [_1,_1,_1,_1];
_8 = [_2,_2];
_7 = _5;
_4 = !_5;
_11.0 = 1116970598378081665_usize;
_5 = !_4;
RET = core::ptr::addr_of!(_11.0);
(*RET) = 7_usize;
_12 = (-27235_i16) as isize;
RET = core::ptr::addr_of!((*RET));
_13 = _12;
_9 = _6 as f64;
_10 = [_1,_1,_1,_1];
_11 = (1_usize, 119512807018748386984248648102175161794_u128);
RET = core::ptr::addr_of!(_11.0);
_2 = '\u{d90ba}';
_15.0 = [58227_u16,21638_u16,5021_u16,49546_u16,45680_u16,9497_u16,8810_u16,50652_u16];
_7 = !_4;
RET = core::ptr::addr_of!(_11.0);
RET = core::ptr::addr_of!((*RET));
_9 = (-90443749907790140105350121662263440767_i128) as f64;
_16 = _9 - _9;
_11.0 = 7_usize;
Goto(bb4)
}
bb13 = {
_10 = [_1,_1,_1,_1];
_8 = [_2,_2];
_7 = _5;
_4 = !_5;
_11.0 = 1116970598378081665_usize;
_5 = !_4;
RET = core::ptr::addr_of!(_11.0);
(*RET) = 7_usize;
_12 = (-27235_i16) as isize;
RET = core::ptr::addr_of!((*RET));
_13 = _12;
_9 = _6 as f64;
_10 = [_1,_1,_1,_1];
_11 = (1_usize, 119512807018748386984248648102175161794_u128);
RET = core::ptr::addr_of!(_11.0);
_2 = '\u{d90ba}';
_15.0 = [58227_u16,21638_u16,5021_u16,49546_u16,45680_u16,9497_u16,8810_u16,50652_u16];
_7 = !_4;
RET = core::ptr::addr_of!(_11.0);
RET = core::ptr::addr_of!((*RET));
_9 = (-90443749907790140105350121662263440767_i128) as f64;
_16 = _9 - _9;
_11.0 = 7_usize;
Goto(bb4)
}
bb14 = {
_1 = -125_i8;
_2 = '\u{8a688}';
Goto(bb9)
}
bb15 = {
_17 = [(*_34),(*_34),(*_23),(*_34),(*_34),(*_23),(*_23),(*_23)];
RET = core::ptr::addr_of!((*RET));
_15.0 = [44986_u16,12545_u16,4855_u16,65076_u16,61777_u16,11718_u16,8066_u16,9235_u16];
_29 = !3159326474093915834_u64;
_20.fld4 = (*_34) as i16;
_34 = core::ptr::addr_of_mut!((*_23));
_30 = [7707_u16,48417_u16,61147_u16,42616_u16];
_12 = _27 as isize;
_23 = &_1;
_12 = _13;
_20.fld3 = _9 as i8;
_29 = !8923437240526195154_u64;
_20.fld3 = -_1;
(*RET) = _20.fld1 >> _26.0;
_23 = &_20.fld3;
_20.fld3 = _1 | _1;
_23 = &_20.fld3;
(*RET) = _20.fld1;
(*RET) = _20.fld1 | _20.fld1;
_30 = [32198_u16,56863_u16,14892_u16,37039_u16];
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(4_usize, 32_usize, Move(_32), 15_usize, Move(_15), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(4_usize, 4_usize, Move(_4), 13_usize, Move(_13), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(4_usize, 22_usize, Move(_22), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: bool,mut _2: i8,mut _3: bool) -> char {
mir! {
type RET = char;
let _4: i64;
let _5: [i32; 7];
let _6: char;
let _7: [i8; 4];
let _8: (i8, (i64, u8, i64), char, i128);
let _9: *mut Adt24;
let _10: Adt66;
let _11: i64;
let _12: ((u16,),);
let _13: Adt47;
let _14: *mut u16;
let _15: f64;
let _16: i16;
let _17: i128;
let _18: f32;
let _19: ();
let _20: ();
{
RET = '\u{c11db}';
_3 = !_1;
_2 = (-32_i8) & (-81_i8);
_4 = 2895688084400262416_i64 * 405564930991497910_i64;
RET = '\u{fb55e}';
_2 = 4684579930045544295_usize as i8;
_1 = !_3;
_2 = 19_i8 * 5_i8;
_1 = _4 >= _4;
_1 = !_3;
_3 = !_1;
RET = '\u{5b99b}';
_6 = RET;
_2 = 67_i8;
_5 = [1606756628_i32,(-150966675_i32),(-1814107243_i32),1963878263_i32,(-606308651_i32),1165168471_i32,(-284519895_i32)];
_2 = 67_i8 & (-106_i8);
_2 = !(-99_i8);
_6 = RET;
_5 = [(-309124981_i32),(-130453232_i32),108598234_i32,625297963_i32,312729338_i32,(-454659753_i32),(-536586573_i32)];
_6 = RET;
_1 = !_3;
Goto(bb1)
}
bb1 = {
_8.1.1 = !163_u8;
_8.3 = 1_usize as i128;
_8.0 = -_2;
_7 = [_8.0,_2,_8.0,_8.0];
_8.1.0 = !_4;
_8.3 = 91211805405590866799474274687477256125_i128 << _4;
_1 = !_3;
_2 = _8.0;
_8.2 = RET;
_8.3 = _8.1.1 as i128;
_5 = [(-2086303963_i32),(-1075480843_i32),(-544301896_i32),(-532337216_i32),68509924_i32,889240769_i32,(-1655495625_i32)];
Call(_9 = fn6(RET, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.1.2 = _4;
Goto(bb3)
}
bb3 = {
_8.1 = (_4, 239_u8, _4);
RET = _8.2;
_2 = !_8.0;
_8.3 = (-73024657114737529837531406214885144616_i128) | 144032385209448275286441102443982280569_i128;
_5 = [(-1542741986_i32),(-464257200_i32),5449953_i32,(-1837410746_i32),817307257_i32,2138750319_i32,2028268236_i32];
_8.0 = _2 - _2;
_8.1.1 = !58_u8;
RET = _8.2;
_8.1.1 = 232_u8 >> _8.3;
_8.0 = _8.3 as i8;
_3 = _8.2 != _8.2;
_8.1.1 = 2751354629_u32 as u8;
_1 = !_3;
_7 = [_8.0,_8.0,_2,_8.0];
_1 = _3;
_5 = [598107714_i32,1061254691_i32,(-1640063774_i32),(-1693051100_i32),(-2122380022_i32),521601506_i32,(-1725888303_i32)];
_11 = !_8.1.0;
_3 = _1 <= _1;
_8.1.0 = _8.1.2 << _11;
_2 = !_8.0;
_4 = _8.1.2;
_4 = (-12297_i16) as i64;
_8.2 = RET;
_8.1.1 = 199_u8;
_8.1.0 = _8.2 as i64;
RET = _6;
_8.1.1 = !2_u8;
_5 = [1365399578_i32,142395218_i32,(-1609073469_i32),1010880594_i32,(-1388439683_i32),1249195553_i32,(-188287912_i32)];
_5 = [1209248237_i32,1022766594_i32,279423160_i32,(-1711754695_i32),1625823853_i32,(-1230693594_i32),(-1670854402_i32)];
_5 = [(-1680750759_i32),1149752028_i32,1445977619_i32,1917862700_i32,1443844921_i32,(-914340630_i32),(-1127133909_i32)];
Goto(bb4)
}
bb4 = {
_8.0 = !_2;
_2 = (-20532_i16) as i8;
_5 = [(-1006946110_i32),875068729_i32,697621922_i32,(-123864520_i32),630247307_i32,(-1696681315_i32),277055950_i32];
_11 = _4 | _8.1.0;
_8.0 = _2;
RET = _6;
_8.2 = _6;
_5 = [1456006368_i32,404711502_i32,1638081341_i32,1573267490_i32,(-1726412937_i32),(-1958044610_i32),1959531972_i32];
_13 = Adt47::Variant0 { fld0: _8.1.2,fld1: _8.3,fld2: _8.1.1,fld3: _2,fld4: _7,fld5: 54282_u16 };
_6 = _8.2;
_12.0 = (36678_u16,);
_12.0.0 = 36655_u16 + 50660_u16;
_12.0.0 = _3 as u16;
place!(Field::<[i8; 4]>(Variant(_13, 0), 4)) = [Field::<i8>(Variant(_13, 0), 3),_8.0,_2,_2];
_4 = !_8.1.2;
RET = _8.2;
_7 = Field::<[i8; 4]>(Variant(_13, 0), 4);
RET = _6;
_13 = Adt47::Variant0 { fld0: _8.1.2,fld1: _8.3,fld2: _8.1.1,fld3: _8.0,fld4: _7,fld5: _12.0.0 };
SetDiscriminant(_13, 1);
Goto(bb5)
}
bb5 = {
_8.2 = RET;
place!(Field::<(i64, u8, i64)>(Variant(_13, 1), 4)) = (_4, _8.1.1, _8.1.2);
place!(Field::<f32>(Variant(_13, 1), 2)) = 17914996982884951452_u64 as f32;
_4 = _8.1.2 << _12.0.0;
_5 = [338220083_i32,259814940_i32,1053923946_i32,(-1526971480_i32),155182755_i32,(-589010375_i32),1161232578_i32];
_8.0 = _2;
place!(Field::<i128>(Variant(_13, 1), 1)) = -_8.3;
place!(Field::<(i64, u8, i64)>(Variant(_13, 1), 4)).0 = _8.1.2 - _4;
_6 = _8.2;
_12.0 = (36984_u16,);
match _12.0.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
36984 => bb14,
_ => bb13
}
}
bb6 = {
_8.0 = !_2;
_2 = (-20532_i16) as i8;
_5 = [(-1006946110_i32),875068729_i32,697621922_i32,(-123864520_i32),630247307_i32,(-1696681315_i32),277055950_i32];
_11 = _4 | _8.1.0;
_8.0 = _2;
RET = _6;
_8.2 = _6;
_5 = [1456006368_i32,404711502_i32,1638081341_i32,1573267490_i32,(-1726412937_i32),(-1958044610_i32),1959531972_i32];
_13 = Adt47::Variant0 { fld0: _8.1.2,fld1: _8.3,fld2: _8.1.1,fld3: _2,fld4: _7,fld5: 54282_u16 };
_6 = _8.2;
_12.0 = (36678_u16,);
_12.0.0 = 36655_u16 + 50660_u16;
_12.0.0 = _3 as u16;
place!(Field::<[i8; 4]>(Variant(_13, 0), 4)) = [Field::<i8>(Variant(_13, 0), 3),_8.0,_2,_2];
_4 = !_8.1.2;
RET = _8.2;
_7 = Field::<[i8; 4]>(Variant(_13, 0), 4);
RET = _6;
_13 = Adt47::Variant0 { fld0: _8.1.2,fld1: _8.3,fld2: _8.1.1,fld3: _8.0,fld4: _7,fld5: _12.0.0 };
SetDiscriminant(_13, 1);
Goto(bb5)
}
bb7 = {
_8.1 = (_4, 239_u8, _4);
RET = _8.2;
_2 = !_8.0;
_8.3 = (-73024657114737529837531406214885144616_i128) | 144032385209448275286441102443982280569_i128;
_5 = [(-1542741986_i32),(-464257200_i32),5449953_i32,(-1837410746_i32),817307257_i32,2138750319_i32,2028268236_i32];
_8.0 = _2 - _2;
_8.1.1 = !58_u8;
RET = _8.2;
_8.1.1 = 232_u8 >> _8.3;
_8.0 = _8.3 as i8;
_3 = _8.2 != _8.2;
_8.1.1 = 2751354629_u32 as u8;
_1 = !_3;
_7 = [_8.0,_8.0,_2,_8.0];
_1 = _3;
_5 = [598107714_i32,1061254691_i32,(-1640063774_i32),(-1693051100_i32),(-2122380022_i32),521601506_i32,(-1725888303_i32)];
_11 = !_8.1.0;
_3 = _1 <= _1;
_8.1.0 = _8.1.2 << _11;
_2 = !_8.0;
_4 = _8.1.2;
_4 = (-12297_i16) as i64;
_8.2 = RET;
_8.1.1 = 199_u8;
_8.1.0 = _8.2 as i64;
RET = _6;
_8.1.1 = !2_u8;
_5 = [1365399578_i32,142395218_i32,(-1609073469_i32),1010880594_i32,(-1388439683_i32),1249195553_i32,(-188287912_i32)];
_5 = [1209248237_i32,1022766594_i32,279423160_i32,(-1711754695_i32),1625823853_i32,(-1230693594_i32),(-1670854402_i32)];
_5 = [(-1680750759_i32),1149752028_i32,1445977619_i32,1917862700_i32,1443844921_i32,(-914340630_i32),(-1127133909_i32)];
Goto(bb4)
}
bb8 = {
_8.1.2 = _4;
Goto(bb3)
}
bb9 = {
_8.1.1 = !163_u8;
_8.3 = 1_usize as i128;
_8.0 = -_2;
_7 = [_8.0,_2,_8.0,_8.0];
_8.1.0 = !_4;
_8.3 = 91211805405590866799474274687477256125_i128 << _4;
_1 = !_3;
_2 = _8.0;
_8.2 = RET;
_8.3 = _8.1.1 as i128;
_5 = [(-2086303963_i32),(-1075480843_i32),(-544301896_i32),(-532337216_i32),68509924_i32,889240769_i32,(-1655495625_i32)];
Call(_9 = fn6(RET, _5), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb14 = {
_14 = core::ptr::addr_of_mut!(_12.0.0);
(*_14) = 31599_u16 - 41065_u16;
_15 = 9223372036854775807_isize as f64;
_17 = _8.3 ^ _8.3;
_1 = _3;
_8.1.1 = Field::<(i64, u8, i64)>(Variant(_13, 1), 4).1 + Field::<(i64, u8, i64)>(Variant(_13, 1), 4).1;
_4 = 3475532272_u32 as i64;
_17 = _8.3 - Field::<i128>(Variant(_13, 1), 1);
_8.0 = _15 as i8;
place!(Field::<f32>(Variant(_13, 1), 2)) = (-24772_i16) as f32;
_18 = _8.0 as f32;
place!(Field::<u128>(Variant(_13, 1), 0)) = (-9223372036854775808_isize) as u128;
_8.0 = _2 * _2;
_8.1.0 = Field::<(i64, u8, i64)>(Variant(_13, 1), 4).0;
_8.3 = -_17;
_6 = _8.2;
_8.2 = _6;
RET = _8.2;
place!(Field::<u128>(Variant(_13, 1), 0)) = !104573247323057153099932742031176519116_u128;
place!(Field::<u32>(Variant(_13, 1), 3)) = 4274227638_u32 | 1062081280_u32;
_1 = Field::<(i64, u8, i64)>(Variant(_13, 1), 4).0 != _8.1.0;
_2 = Field::<f32>(Variant(_13, 1), 2) as i8;
(*_14) = _8.0 as u16;
RET = _8.2;
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(5_usize, 5_usize, Move(_5), 6_usize, Move(_6), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_19 = dump_var(5_usize, 7_usize, Move(_7), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: char,mut _2: [i32; 7]) -> *mut Adt24 {
mir! {
type RET = *mut Adt24;
let _3: (i64, u8, i64);
let _4: [char; 2];
let _5: &'static u32;
let _6: [char; 2];
let _7: i32;
let _8: u128;
let _9: [i8; 8];
let _10: &'static (bool, (bool, u32, i32, bool), u128);
let _11: i128;
let _12: f64;
let _13: *mut u16;
let _14: f64;
let _15: (Adt47, &'static u32, &'static u128, u64);
let _16: (i8, (i64, u8, i64), char, i128);
let _17: f32;
let _18: isize;
let _19: Adt58;
let _20: Adt24;
let _21: u128;
let _22: ();
let _23: ();
{
_2 = [(-330054443_i32),1293878841_i32,1945230964_i32,(-953955109_i32),(-1778341574_i32),1901489199_i32,161774209_i32];
_1 = '\u{7519}';
_2 = [(-2141783260_i32),1183811049_i32,(-1551154526_i32),(-165424_i32),(-1350466462_i32),(-1206153741_i32),38629758_i32];
_1 = '\u{9fd5f}';
_1 = '\u{c1933}';
_1 = '\u{91d54}';
_1 = '\u{ef957}';
_2 = [985952500_i32,131623433_i32,1544196186_i32,(-246383934_i32),1295456338_i32,(-868621931_i32),(-1572177422_i32)];
_3.0 = 8834492875256678718_i64;
_3.0 = 2127738721_u32 as i64;
_2 = [(-242601004_i32),(-2027599622_i32),2007357741_i32,1598402189_i32,(-205581223_i32),(-244317710_i32),(-401340033_i32)];
_3.1 = !151_u8;
_2 = [(-696080013_i32),(-239973590_i32),258335676_i32,(-1283481229_i32),1271463875_i32,267942605_i32,573930887_i32];
_1 = '\u{ce650}';
_4 = [_1,_1];
_3.2 = _3.0;
_3 = ((-6802576367815662058_i64), 150_u8, (-6889120808164620798_i64));
_2 = [1262089883_i32,(-819859372_i32),1592178920_i32,(-1928193242_i32),(-580884755_i32),(-1058883104_i32),(-396240140_i32)];
_2 = [695476000_i32,(-445611611_i32),1352511846_i32,790105394_i32,1386261608_i32,627575784_i32,(-1092851614_i32)];
_3.1 = 51_u8 ^ 251_u8;
_3 = (4234450469177190589_i64, 113_u8, (-2573093081402212842_i64));
_2 = [1448555518_i32,760219582_i32,(-986804371_i32),1086472853_i32,(-738598695_i32),30518157_i32,(-1046764132_i32)];
_3 = (5587109963768276938_i64, 108_u8, 6283426558285135993_i64);
match _3.0 {
0 => bb1,
1 => bb2,
5587109963768276938 => bb4,
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
_1 = '\u{95f74}';
_3.2 = (-45_i8) as i64;
_3 = (1590877775378173037_i64, 151_u8, 2755070676630989524_i64);
_3.1 = 31_u8;
_3.2 = _3.0;
_1 = '\u{8b775}';
_3 = (6015209959891358043_i64, 196_u8, (-7523354593464268897_i64));
_2 = [663846940_i32,(-1897846687_i32),339013054_i32,1387991580_i32,2061403073_i32,1368459238_i32,650825916_i32];
_3.1 = 131_u8 + 234_u8;
_2 = [(-1959531304_i32),(-1812195266_i32),254527708_i32,(-287627799_i32),(-1572591532_i32),1412138728_i32,1198051139_i32];
match _3.0 {
0 => bb3,
6015209959891358043 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
_3.2 = (-51681841609789177021160767572222140418_i128) as i64;
_1 = '\u{b2ac5}';
_3 = (8539611742990838366_i64, 107_u8, (-3283736805236437502_i64));
_6 = [_1,_1];
match _3.0 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
8539611742990838366 => bb12,
_ => bb11
}
}
bb7 = {
Return()
}
bb8 = {
_1 = '\u{95f74}';
_3.2 = (-45_i8) as i64;
_3 = (1590877775378173037_i64, 151_u8, 2755070676630989524_i64);
_3.1 = 31_u8;
_3.2 = _3.0;
_1 = '\u{8b775}';
_3 = (6015209959891358043_i64, 196_u8, (-7523354593464268897_i64));
_2 = [663846940_i32,(-1897846687_i32),339013054_i32,1387991580_i32,2061403073_i32,1368459238_i32,650825916_i32];
_3.1 = 131_u8 + 234_u8;
_2 = [(-1959531304_i32),(-1812195266_i32),254527708_i32,(-287627799_i32),(-1572591532_i32),1412138728_i32,1198051139_i32];
match _3.0 {
0 => bb3,
6015209959891358043 => bb6,
_ => bb5
}
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
_1 = '\u{16970}';
match _3.2 {
0 => bb10,
1 => bb13,
340282366920938463460090870626531773954 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_1 = '\u{95f74}';
_3.2 = (-45_i8) as i64;
_3 = (1590877775378173037_i64, 151_u8, 2755070676630989524_i64);
_3.1 = 31_u8;
_3.2 = _3.0;
_1 = '\u{8b775}';
_3 = (6015209959891358043_i64, 196_u8, (-7523354593464268897_i64));
_2 = [663846940_i32,(-1897846687_i32),339013054_i32,1387991580_i32,2061403073_i32,1368459238_i32,650825916_i32];
_3.1 = 131_u8 + 234_u8;
_2 = [(-1959531304_i32),(-1812195266_i32),254527708_i32,(-287627799_i32),(-1572591532_i32),1412138728_i32,1198051139_i32];
match _3.0 {
0 => bb3,
6015209959891358043 => bb6,
_ => bb5
}
}
bb15 = {
Call(_6 = fn7(_3.2, _3.2, _3.2, _2, _3, _2, _3), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_8 = 86154776824607126102583369355344373914_u128;
_8 = 186477473850363163872818424939984943919_u128 - 215633138683607885982650747776431766569_u128;
_6 = _4;
_7 = -749441868_i32;
_3.1 = 70_u8;
_4 = [_1,_1];
_3.1 = _3.0 as u8;
match _3.2 {
0 => bb11,
1 => bb2,
2 => bb17,
3 => bb18,
340282366920938463460090870626531773954 => bb20,
_ => bb19
}
}
bb17 = {
_1 = '\u{95f74}';
_3.2 = (-45_i8) as i64;
_3 = (1590877775378173037_i64, 151_u8, 2755070676630989524_i64);
_3.1 = 31_u8;
_3.2 = _3.0;
_1 = '\u{8b775}';
_3 = (6015209959891358043_i64, 196_u8, (-7523354593464268897_i64));
_2 = [663846940_i32,(-1897846687_i32),339013054_i32,1387991580_i32,2061403073_i32,1368459238_i32,650825916_i32];
_3.1 = 131_u8 + 234_u8;
_2 = [(-1959531304_i32),(-1812195266_i32),254527708_i32,(-287627799_i32),(-1572591532_i32),1412138728_i32,1198051139_i32];
match _3.0 {
0 => bb3,
6015209959891358043 => bb6,
_ => bb5
}
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_3.1 = 118_u8;
_4 = _6;
_8 = 2156820843_u32 as u128;
_3.1 = 234_u8;
_7 = (-1234272446_i32) & 1031958759_i32;
_4 = [_1,_1];
_2 = [_7,_7,_7,_7,_7,_7,_7];
_9 = [111_i8,(-113_i8),(-12_i8),(-108_i8),(-13_i8),97_i8,78_i8,35_i8];
_3.2 = _3.0;
_3.2 = _3.0 - _3.0;
_7 = !603199253_i32;
_7 = -(-962734909_i32);
_9 = [(-109_i8),80_i8,1_i8,(-77_i8),(-87_i8),(-44_i8),75_i8,22_i8];
_8 = !244830005795422016875837784911443390283_u128;
match _3.1 {
0 => bb6,
1 => bb2,
2 => bb5,
3 => bb21,
234 => bb23,
_ => bb22
}
}
bb21 = {
_3.2 = (-51681841609789177021160767572222140418_i128) as i64;
_1 = '\u{b2ac5}';
_3 = (8539611742990838366_i64, 107_u8, (-3283736805236437502_i64));
_6 = [_1,_1];
match _3.0 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
8539611742990838366 => bb12,
_ => bb11
}
}
bb22 = {
Return()
}
bb23 = {
_3.0 = _3.2;
_1 = '\u{7c3c}';
_7 = !1823329771_i32;
_3.2 = -_3.0;
_12 = 5251650733311013616_u64 as f64;
_6 = _4;
_3.2 = true as i64;
_8 = 70821764170813509025714689555336959037_u128 << _7;
_2 = [_7,_7,_7,_7,_7,_7,_7];
_12 = _3.0 as f64;
_1 = '\u{8f96f}';
_3.2 = 3102560005_u32 as i64;
_6 = _4;
_9 = [(-119_i8),(-66_i8),(-111_i8),(-55_i8),51_i8,30_i8,99_i8,20_i8];
_14 = _12;
_8 = 22786132156070871148487345402199101176_u128;
_12 = -_14;
_2 = [_7,_7,_7,_7,_7,_7,_7];
_11 = _7 as i128;
_4 = _6;
Goto(bb24)
}
bb24 = {
_3.2 = _3.0;
_9 = [(-99_i8),98_i8,(-62_i8),(-126_i8),(-2_i8),(-11_i8),39_i8,124_i8];
_6 = [_1,_1];
_9 = [(-47_i8),(-46_i8),87_i8,83_i8,(-99_i8),38_i8,25_i8,114_i8];
_7 = -146955751_i32;
_3.0 = _7 as i64;
_3.2 = 868525144_u32 as i64;
_15.2 = &_8;
_15.2 = &_8;
_3.0 = -_3.2;
_7 = 172227875_i32;
_16.1.0 = _11 as i64;
_3.1 = (-9_i8) as u8;
_16.1 = (_3.2, _3.1, _3.0);
_16.0 = !(-20_i8);
_16 = ((-19_i8), _3, _1, _11);
_2 = [_7,_7,_7,_7,_7,_7,_7];
_4 = [_1,_16.2];
_16.1.2 = _3.0 & _3.0;
_16.0 = -(-126_i8);
_14 = _12 - _12;
match _7 {
0 => bb3,
1 => bb25,
172227875 => bb27,
_ => bb26
}
}
bb25 = {
Return()
}
bb26 = {
Return()
}
bb27 = {
_6 = _4;
_17 = 4161123135_u32 as f32;
_16.3 = -_11;
_18 = _16.3 as isize;
_3 = (_16.1.2, _16.1.1, _16.1.2);
_3 = (_16.1.2, _16.1.1, _16.1.0);
_7 = (-1934392939_i32);
_16.1.1 = _3.1;
_15.2 = &_8;
_3.2 = _7 as i64;
_16 = ((-84_i8), _3, _1, _11);
_20.fld1 = !9926068553106408404_usize;
_20.fld2 = _18 & _18;
_7 = (-757206190_i32);
match _8 {
22786132156070871148487345402199101176 => bb28,
_ => bb16
}
}
bb28 = {
RET = core::ptr::addr_of_mut!(_20);
_3.1 = 32301_u16 as u8;
_15.2 = &_8;
(*RET).fld1 = 9744782906435048858_u64 as usize;
_3.0 = _16.1.0 << (*RET).fld2;
(*RET).fld3 = -_16.0;
(*RET).fld4 = _20.fld3 as i16;
_16.1.0 = _3.0;
_15.2 = &_8;
_16.2 = _1;
(*RET).fld0 = [_3.1,_16.1.1,_3.1,_3.1];
_15.3 = 10749601297821447871_u64;
_16.1 = _3;
_2 = [_7,_7,_7,_7,_7,_7,_7];
(*RET).fld0 = [_3.1,_16.1.1,_3.1,_16.1.1];
_3 = _16.1;
_11 = _16.3 | _16.3;
_12 = _17 as f64;
_16.2 = _1;
_14 = _7 as f64;
_20.fld2 = 3855546231_u32 as isize;
(*RET).fld0 = [_3.1,_3.1,_3.1,_16.1.1];
_16.1.0 = _16.1.2;
_20.fld4 = (-13617_i16);
Goto(bb29)
}
bb29 = {
Call(_22 = dump_var(6_usize, 4_usize, Move(_4), 1_usize, Move(_1), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_22 = dump_var(6_usize, 6_usize, Move(_6), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: [i32; 7],mut _5: (i64, u8, i64),mut _6: [i32; 7],mut _7: (i64, u8, i64)) -> [char; 2] {
mir! {
type RET = [char; 2];
let _8: isize;
let _9: *mut Adt24;
let _10: usize;
let _11: &'static u32;
let _12: Adt30;
let _13: (usize, i16);
let _14: (((u16,),), Adt47, (bool, u32, i32, bool));
let _15: (i128, &'static (bool, (bool, u32, i32, bool), u128), u128, &'static &'static u128);
let _16: f64;
let _17: char;
let _18: u32;
let _19: u128;
let _20: *mut (usize, i16);
let _21: bool;
let _22: i64;
let _23: ([i32; 7], &'static Adt47);
let _24: i128;
let _25: (bool, (bool, u32, i32, bool), u128);
let _26: bool;
let _27: &'static (usize, i16);
let _28: Adt42;
let _29: i8;
let _30: Adt63;
let _31: i64;
let _32: ();
let _33: ();
{
_1 = _3;
_6 = _4;
RET = ['\u{39ac2}','\u{90df8}'];
_5 = (_7.0, _7.1, _2);
_4 = [(-169324644_i32),(-409758355_i32),601634332_i32,883121793_i32,(-2107384999_i32),12188115_i32,1910017457_i32];
_5.1 = !_7.1;
_5.2 = '\u{ed0e5}' as i64;
_5 = (_3, _7.1, _7.0);
RET = ['\u{f1412}','\u{c3c22}'];
_6 = [(-44909057_i32),794517070_i32,2048136434_i32,(-715525650_i32),1627208484_i32,937098316_i32,1459733225_i32];
_3 = -_1;
_7.1 = _7.0 as u8;
_8 = 9223372036854775807_isize | (-9223372036854775808_isize);
_5.0 = _5.2;
_2 = _7.0;
_7.0 = -_7.2;
_1 = -_5.0;
RET = ['\u{24e2f}','\u{e6f58}'];
Call(RET = fn8(_5.2, _7, _5.2, _5.0, _5.0, _2, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !2_usize;
_2 = _5.0 << _1;
_10 = !7_usize;
_7.2 = !_2;
_10 = 3_usize - 16201349281470723181_usize;
_3 = -_7.0;
_13 = (_10, 8072_i16);
Goto(bb2)
}
bb2 = {
_7.0 = _7.2 - _2;
RET = ['\u{1bb28}','\u{e42b4}'];
_14.2.0 = false;
_5 = (_3, _7.1, _3);
_14.2.3 = _14.2.0;
_14.0.0.0 = !9613_u16;
_7 = _5;
_14.2.1 = _5.1 as u32;
_14.0.0.0 = 65011_u16 << _5.2;
_15.0 = -(-3972089425072498034624897253583734791_i128);
_14.0.0 = (31275_u16,);
_11 = &_14.2.1;
_10 = _5.0 as usize;
RET = ['\u{8bb5f}','\u{84a85}'];
_14.0.0 = (26780_u16,);
_5.2 = _7.0;
_14.2 = (false, 3895901724_u32, 1191865546_i32, true);
_7.0 = !_7.2;
_15.2 = 334322985108728873181612326907260257185_u128 - 107127654399142657643290174260605280599_u128;
_1 = _3;
_14.0.0.0 = _5.0 as u16;
_7 = _5;
RET = ['\u{d848b}','\u{bfc7f}'];
_7.1 = _5.1;
_1 = _2 << _2;
_14.2.0 = _5.1 != _5.1;
_5.2 = _7.0;
Call(_16 = fn18(_5.1, _14.2.2, _14.2.2, _14.2.1, _7, _14.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13.0 = _10 * _10;
_1 = _2 * _2;
_2 = _14.2.3 as i64;
_8 = -9223372036854775807_isize;
_10 = _13.0 ^ _13.0;
_17 = '\u{4165b}';
_17 = '\u{97bdc}';
_5.2 = _1;
_18 = _14.2.1;
_19 = !_15.2;
_11 = &_14.2.1;
match _18 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
3895901724 => bb10,
_ => bb9
}
}
bb4 = {
_7.0 = _7.2 - _2;
RET = ['\u{1bb28}','\u{e42b4}'];
_14.2.0 = false;
_5 = (_3, _7.1, _3);
_14.2.3 = _14.2.0;
_14.0.0.0 = !9613_u16;
_7 = _5;
_14.2.1 = _5.1 as u32;
_14.0.0.0 = 65011_u16 << _5.2;
_15.0 = -(-3972089425072498034624897253583734791_i128);
_14.0.0 = (31275_u16,);
_11 = &_14.2.1;
_10 = _5.0 as usize;
RET = ['\u{8bb5f}','\u{84a85}'];
_14.0.0 = (26780_u16,);
_5.2 = _7.0;
_14.2 = (false, 3895901724_u32, 1191865546_i32, true);
_7.0 = !_7.2;
_15.2 = 334322985108728873181612326907260257185_u128 - 107127654399142657643290174260605280599_u128;
_1 = _3;
_14.0.0.0 = _5.0 as u16;
_7 = _5;
RET = ['\u{d848b}','\u{bfc7f}'];
_7.1 = _5.1;
_1 = _2 << _2;
_14.2.0 = _5.1 != _5.1;
_5.2 = _7.0;
Call(_16 = fn18(_5.1, _14.2.2, _14.2.2, _14.2.1, _7, _14.2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_10 = !2_usize;
_2 = _5.0 << _1;
_10 = !7_usize;
_7.2 = !_2;
_10 = 3_usize - 16201349281470723181_usize;
_3 = -_7.0;
_13 = (_10, 8072_i16);
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
Return()
}
bb10 = {
_11 = &(*_11);
_5.2 = _7.0;
_7.0 = _2;
_11 = &_14.2.1;
RET = [_17,_17];
_15.0 = 115680429816730296184647888479308057844_i128 ^ (-152729128617015156427866791443896574980_i128);
_14.0.0.0 = !35305_u16;
_6 = [_14.2.2,_14.2.2,_14.2.2,_14.2.2,_14.2.2,_14.2.2,_14.2.2];
_13.1 = !25636_i16;
Goto(bb11)
}
bb11 = {
_18 = _14.2.1 / _14.2.1;
_2 = _1 << _8;
_14.2.1 = !_18;
_23.1 = &_14.1;
_5 = (_1, _7.1, _1);
_14.2.0 = _14.2.3 | _14.2.3;
_23.1 = &_14.1;
_14.2 = (false, _18, (-125216282_i32), true);
_13.1 = 14404_i16;
_4 = _6;
_24 = 14665247991369422496_u64 as i128;
Goto(bb12)
}
bb12 = {
_7.2 = !_5.0;
_18 = !_14.2.1;
_7.1 = _14.2.1 as u8;
RET = [_17,_17];
_1 = _5.0 + _2;
_5.2 = _7.0 << _15.0;
_16 = _13.1 as f64;
_10 = _13.0 + _13.0;
_13.0 = _10 - _10;
RET = [_17,_17];
_22 = _1 << _13.0;
_23.0 = _6;
_14.2.0 = !_14.2.3;
_15.2 = !_19;
_23.1 = &_14.1;
_23.1 = &_14.1;
_25.2 = _1 as u128;
_7.2 = !_1;
Goto(bb13)
}
bb13 = {
_25.1.0 = _14.2.3;
_7.1 = _5.1 + _5.1;
_14.2.2 = 483076724_i32;
_22 = _14.2.2 as i64;
_25.1.2 = _14.2.2;
_13.0 = 6994475361046930530_u64 as usize;
_10 = _13.0 | _13.0;
_25 = (_14.2.3, _14.2, _19);
_10 = _13.0;
_26 = _25.1.0 & _14.2.0;
_25 = (_26, _14.2, _19);
_5.0 = -_7.0;
_5 = _7;
_22 = _18 as i64;
_20 = core::ptr::addr_of_mut!(_13);
(*_20).0 = 83_i8 as usize;
(*_20).0 = !_10;
_14.2.1 = _25.1.1 * _18;
_14.0.0 = (1389_u16,);
match (*_20).1 {
0 => bb14,
14404 => bb16,
_ => bb15
}
}
bb14 = {
_10 = !2_usize;
_2 = _5.0 << _1;
_10 = !7_usize;
_7.2 = !_2;
_10 = 3_usize - 16201349281470723181_usize;
_3 = -_7.0;
_13 = (_10, 8072_i16);
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
(*_20) = (_10, 11989_i16);
_25.1.1 = _8 as u32;
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(7_usize, 18_usize, Move(_18), 6_usize, Move(_6), 25_usize, Move(_25), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(7_usize, 5_usize, Move(_5), 17_usize, Move(_17), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i64,mut _2: (i64, u8, i64),mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: [i32; 7]) -> [char; 2] {
mir! {
type RET = [char; 2];
let _8: u8;
let _9: (i128, &'static (bool, (bool, u32, i32, bool), u128), u128, &'static &'static u128);
let _10: Adt47;
let _11: (i64, u8, i64);
let _12: *mut Adt24;
let _13: [i32; 7];
let _14: [i8; 4];
let _15: i64;
let _16: isize;
let _17: char;
let _18: *mut i8;
let _19: i128;
let _20: &'static (i64, u8, i64);
let _21: &'static ((u16,),);
let _22: f32;
let _23: isize;
let _24: i8;
let _25: &'static [u16; 4];
let _26: isize;
let _27: (bool, u32, i32, bool);
let _28: ([u16; 8],);
let _29: [i16; 8];
let _30: (bool, u32, i32, bool);
let _31: &'static &'static u32;
let _32: Adt53;
let _33: ();
let _34: ();
{
_3 = !_1;
RET = ['\u{c8813}','\u{6b373}'];
_7 = [(-1309258135_i32),2050247358_i32,534440225_i32,(-1702949314_i32),417878392_i32,716508248_i32,2130199032_i32];
_8 = _2.1;
_8 = 534631948_u32 as u8;
_1 = !_5;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
8539611742990838366 => bb5,
_ => bb4
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
_1 = _6 * _2.2;
_4 = 18475_u16 as i64;
_5 = (-32_isize) as i64;
_4 = -_3;
_2 = (_1, _8, _4);
_9.2 = 239824821311779045303151210629616775484_u128 ^ 314414085709688921285827196352116713756_u128;
_2 = (_3, _8, _6);
RET = ['\u{ae03c}','\u{223b3}'];
_11.1 = !_2.1;
_2 = (_3, _8, _5);
_9.0 = -(-134856860198067702294045743758386772861_i128);
_13 = [801930205_i32,1097861386_i32,(-1604064528_i32),2092458779_i32,(-838537315_i32),737817250_i32,1150807086_i32];
_9.0 = 136187544_i32 as i128;
_4 = _1;
Call(_5 = fn9(_2.0, _13, _13, _9.0, _4, _13, _9.0, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2.2 = _5 & _1;
_1 = (-12488_i16) as i64;
_6 = _2.2;
_16 = 26_i8 as isize;
_5 = _4;
_15 = !_2.2;
Goto(bb7)
}
bb7 = {
_15 = _4;
_11 = _2;
_14 = [(-38_i8),(-55_i8),72_i8,(-74_i8)];
_2.0 = _5 - _5;
_3 = _2.2 | _2.2;
_8 = !_11.1;
_11.0 = !_15;
_15 = _11.2 | _4;
_10 = Adt47::Variant0 { fld0: _3,fld1: _9.0,fld2: _8,fld3: (-46_i8),fld4: _14,fld5: 42534_u16 };
_1 = _5 >> _5;
RET = ['\u{7f761}','\u{a1718}'];
_4 = _11.2 & _5;
_18 = core::ptr::addr_of_mut!(place!(Field::<i8>(Variant(_10, 0), 3)));
_2 = (Field::<i64>(Variant(_10, 0), 0), _11.1, _11.0);
_11 = _2;
_3 = '\u{3c6c5}' as i64;
_18 = core::ptr::addr_of_mut!((*_18));
place!(Field::<u8>(Variant(_10, 0), 2)) = !_2.1;
_2 = _11;
_11 = (_2.2, _2.1, _5);
Goto(bb8)
}
bb8 = {
place!(Field::<[i8; 4]>(Variant(_10, 0), 4)) = [(-96_i8),124_i8,(-8_i8),(-68_i8)];
_11.0 = _6;
place!(Field::<[i8; 4]>(Variant(_10, 0), 4)) = [86_i8,35_i8,1_i8,54_i8];
_20 = &_2;
_8 = (*_20).1;
(*_18) = (-45_i8);
place!(Field::<u16>(Variant(_10, 0), 5)) = '\u{5c15a}' as u16;
_5 = 5143_i16 as i64;
_18 = core::ptr::addr_of_mut!((*_18));
_18 = core::ptr::addr_of_mut!((*_18));
place!(Field::<i64>(Variant(_10, 0), 0)) = _11.0 | _11.0;
_7 = [155035979_i32,(-1549816240_i32),(-605293145_i32),(-1443797756_i32),(-1463704798_i32),(-1356883173_i32),851649748_i32];
place!(Field::<u16>(Variant(_10, 0), 5)) = 0_usize as u16;
Call(_16 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3 = (*_20).0 & _2.0;
_1 = (-1052198322_i32) as i64;
_17 = '\u{d6885}';
Goto(bb10)
}
bb10 = {
_2.0 = _15 | _3;
(*_18) = (*_20).1 as i8;
_2.1 = !_11.1;
SetDiscriminant(_10, 1);
place!(Field::<(i64, u8, i64)>(Variant(_10, 1), 4)).2 = _15;
place!(Field::<u128>(Variant(_10, 1), 0)) = _9.2;
_11 = (_2.0, _2.1, _2.0);
Goto(bb11)
}
bb11 = {
_15 = _2.0 - _11.2;
place!(Field::<(i64, u8, i64)>(Variant(_10, 1), 4)).1 = _2.1;
_16 = _9.0 as isize;
_22 = _16 as f32;
place!(Field::<u128>(Variant(_10, 1), 0)) = _9.2 & _9.2;
_2.0 = (-14262_i16) as i64;
place!(Field::<f32>(Variant(_10, 1), 2)) = _22;
_6 = !_11.0;
_10 = Adt47::Variant0 { fld0: _11.0,fld1: _9.0,fld2: _2.1,fld3: (-126_i8),fld4: _14,fld5: 20533_u16 };
_9.2 = 1562_u16 as u128;
place!(Field::<i8>(Variant(_10, 0), 3)) = !(-42_i8);
place!(Field::<u16>(Variant(_10, 0), 5)) = 29621_u16 & 38470_u16;
place!(Field::<u16>(Variant(_10, 0), 5)) = 22876_u16;
_11 = (Field::<i64>(Variant(_10, 0), 0), _2.1, Field::<i64>(Variant(_10, 0), 0));
_2 = (_15, Field::<u8>(Variant(_10, 0), 2), _3);
place!(Field::<i8>(Variant(_10, 0), 3)) = 18_i8 ^ 38_i8;
_11 = _2;
place!(Field::<u8>(Variant(_10, 0), 2)) = 10320035431129182910_u64 as u8;
_18 = core::ptr::addr_of_mut!(place!(Field::<i8>(Variant(_10, 0), 3)));
_7 = [(-862234381_i32),(-1639102137_i32),1178904488_i32,(-1247770041_i32),1296041450_i32,(-327353965_i32),(-147157866_i32)];
_19 = -_9.0;
_11.1 = _8;
_2.0 = _6;
SetDiscriminant(_10, 1);
_15 = _11.0;
Goto(bb12)
}
bb12 = {
_1 = _2.0 + _15;
place!(Field::<(i64, u8, i64)>(Variant(_10, 1), 4)).0 = -_11.0;
_13 = [(-1968735610_i32),(-371535941_i32),993747124_i32,1883180626_i32,(-871799436_i32),1876002587_i32,1916322272_i32];
_19 = !_9.0;
_9.0 = _22 as i128;
RET = [_17,_17];
_11.1 = _2.1 + _8;
_24 = (-53_i8);
_3 = !_11.0;
_27.1 = _24 as u32;
_4 = _1;
_27 = (true, 532999604_u32, 1861746058_i32, false);
_27.3 = !_27.0;
_2.0 = (-30242_i16) as i64;
_1 = 19196_i16 as i64;
_20 = &place!(Field::<(i64, u8, i64)>(Variant(_10, 1), 4));
_17 = '\u{eb4bd}';
Goto(bb13)
}
bb13 = {
_10 = Adt47::Variant0 { fld0: _11.2,fld1: _19,fld2: _11.1,fld3: _24,fld4: _14,fld5: 15108_u16 };
_30.0 = _6 == _3;
_6 = _19 as i64;
_29 = [(-4831_i16),20646_i16,(-30445_i16),(-20072_i16),(-1192_i16),26761_i16,(-2758_i16),29653_i16];
_30.1 = _27.1;
_19 = Field::<i128>(Variant(_10, 0), 1) << _4;
_20 = &_2;
place!(Field::<u8>(Variant(_10, 0), 2)) = _2.1 << _3;
_10 = Adt47::Variant0 { fld0: (*_20).2,fld1: _19,fld2: (*_20).1,fld3: _24,fld4: _14,fld5: 8875_u16 };
_27 = (_30.0, _30.1, (-1106779987_i32), _30.0);
_10 = Adt47::Variant0 { fld0: _4,fld1: _19,fld2: _8,fld3: _24,fld4: _14,fld5: 35839_u16 };
_20 = &(*_20);
_20 = &(*_20);
_2.0 = Field::<i128>(Variant(_10, 0), 1) as i64;
Goto(bb14)
}
bb14 = {
_22 = 5221963917972127767_u64 as f32;
_5 = _17 as i64;
_27.0 = _27.3;
_17 = '\u{a7dcd}';
_30.3 = !_27.0;
_11 = (_15, _8, _3);
_26 = _30.3 as isize;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(8_usize, 16_usize, Move(_16), 15_usize, Move(_15), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(8_usize, 1_usize, Move(_1), 6_usize, Move(_6), 14_usize, Move(_14), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(8_usize, 11_usize, Move(_11), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i64,mut _2: [i32; 7],mut _3: [i32; 7],mut _4: i128,mut _5: i64,mut _6: [i32; 7],mut _7: i128,mut _8: i64) -> i64 {
mir! {
type RET = i64;
let _9: &'static &'static u32;
let _10: f64;
let _11: isize;
let _12: (&'static u128,);
let _13: [i32; 5];
let _14: (usize, u128);
let _15: u32;
let _16: f32;
let _17: isize;
let _18: isize;
let _19: f64;
let _20: bool;
let _21: &'static Adt63;
let _22: isize;
let _23: Adt58;
let _24: f64;
let _25: *const u32;
let _26: Adt24;
let _27: &'static Adt63;
let _28: &'static i8;
let _29: f64;
let _30: &'static [i32; 7];
let _31: usize;
let _32: [i8; 8];
let _33: (i128, *const usize, u8);
let _34: bool;
let _35: char;
let _36: (Adt47, &'static u32, &'static u128, u64);
let _37: char;
let _38: [isize; 6];
let _39: Adt30;
let _40: u32;
let _41: Adt53;
let _42: isize;
let _43: &'static i8;
let _44: [i64; 6];
let _45: [u8; 4];
let _46: u64;
let _47: Adt24;
let _48: [i32; 7];
let _49: isize;
let _50: [i32; 5];
let _51: f32;
let _52: i64;
let _53: *mut u16;
let _54: f32;
let _55: char;
let _56: i8;
let _57: [i32; 5];
let _58: (usize, u128);
let _59: &'static u32;
let _60: ();
let _61: ();
{
RET = 135773768917184666377027598253119806823_u128 as i64;
_3 = [1440039056_i32,2132176557_i32,(-1504910229_i32),2132610554_i32,399099254_i32,(-1200548123_i32),1681754277_i32];
_2 = [(-666772217_i32),(-1505730851_i32),(-1048758428_i32),(-1297728369_i32),(-1924937817_i32),1514825750_i32,(-2093212248_i32)];
Call(_4 = fn10(_6, _2, _1, _6, _8, _3, _5, _3, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !_8;
_5 = _8;
_7 = -_4;
_5 = -_8;
_1 = !_8;
_2 = [(-184751902_i32),(-1065134970_i32),(-1408586006_i32),714112954_i32,(-280690261_i32),(-11271300_i32),(-1087902282_i32)];
_10 = (-16311_i16) as f64;
RET = (-102_i8) as i64;
_7 = !_4;
_4 = _7 - _7;
_3 = [117224670_i32,943443577_i32,2029966132_i32,450023125_i32,478615924_i32,783757698_i32,(-1005184537_i32)];
_15 = 3103328716_u32;
_3 = _2;
_14.0 = 2_usize;
_3 = [(-381190759_i32),(-1517797049_i32),1466821852_i32,1123698872_i32,764052479_i32,1351597678_i32,749739849_i32];
_4 = _7 | _7;
RET = -_8;
_11 = (-34_isize) * 14_isize;
_11 = !(-9223372036854775808_isize);
_14.0 = !4125571333850713216_usize;
_12.0 = &_14.1;
_16 = 28931_u16 as f32;
Call(_4 = core::intrinsics::bswap(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = -_4;
_5 = !RET;
_13 = [(-1049139464_i32),(-1180854777_i32),1020245925_i32,722397561_i32,(-1743333628_i32)];
_14.1 = 31142551828690820747132391227490717235_u128;
_14.0 = 5_usize;
_11 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_13 = [1475055440_i32,(-871053709_i32),(-1303319525_i32),1829211491_i32,(-1131558710_i32)];
_8 = !_5;
Goto(bb3)
}
bb3 = {
_11 = (-9223372036854775808_isize);
_17 = true as isize;
_15 = 4024858030_u32 - 3626565936_u32;
_12.0 = &_14.1;
_20 = _7 != _4;
_6 = [590693146_i32,(-989189195_i32),(-529714554_i32),(-979734302_i32),(-626615838_i32),(-107391257_i32),(-2037668548_i32)];
_5 = (-9887_i16) as i64;
_7 = '\u{47748}' as i128;
_11 = _17 - _17;
_14.0 = 2112692308946363035_usize - 17813080949324441077_usize;
_4 = _20 as i128;
RET = _16 as i64;
_8 = -_1;
_17 = 4488_u16 as isize;
_14.0 = !4_usize;
_14.0 = 3_usize;
_8 = _1;
_14 = (9695307867533092179_usize, 40004160409034747486512041335512695403_u128);
_22 = _11;
_15 = _4 as u32;
_15 = _10 as u32;
_2 = [(-2121933523_i32),1959364244_i32,(-280420138_i32),573162824_i32,(-1662656749_i32),(-1549889418_i32),(-1390324908_i32)];
_18 = _22;
_22 = -_11;
_19 = _14.1 as f64;
_7 = _4;
_16 = 28619_i16 as f32;
_19 = -_10;
Goto(bb4)
}
bb4 = {
_8 = _1 << _4;
_6 = [176613808_i32,(-1703773390_i32),128984549_i32,(-1291019676_i32),(-1584503427_i32),(-1382029253_i32),(-1160379257_i32)];
_17 = _18 ^ _22;
_1 = _19 as i64;
_14.0 = !6_usize;
_10 = _19 - _19;
_4 = _7 ^ _7;
_12.0 = &_14.1;
_14.0 = 2_usize + 1_usize;
_22 = _18;
_25 = core::ptr::addr_of!(_15);
Goto(bb5)
}
bb5 = {
_26.fld0 = [174_u8,246_u8,153_u8,72_u8];
_6 = _2;
_4 = _7;
_20 = !false;
_26.fld2 = _17 - _11;
_20 = !true;
_22 = -_11;
_26.fld4 = 21888_i16;
_14 = (5_usize, 159847962448234540007944475543228195112_u128);
_12.0 = &_14.1;
_15 = 896587421_u32;
_22 = _11;
_19 = _10;
_26.fld3 = -(-26_i8);
_29 = _10 - _19;
_26.fld4 = 15184_i16 | 22205_i16;
_26.fld1 = 882651561092700000_u64 as usize;
_26.fld3 = (-18_i8);
_1 = !_8;
Goto(bb6)
}
bb6 = {
_16 = _26.fld3 as f32;
_31 = 9936077870882856612_u64 as usize;
_26.fld4 = 12379_i16;
(*_25) = _4 as u32;
_16 = _31 as f32;
_15 = 1106595632_i32 as u32;
_3 = _6;
RET = _20 as i64;
(*_25) = (-638250322_i32) as u32;
_31 = !_14.0;
_11 = _14.1 as isize;
_10 = _19 + _29;
match _26.fld3 {
340282366920938463463374607431768211438 => bb7,
_ => bb2
}
}
bb7 = {
_8 = -_1;
_29 = _19 * _10;
(*_25) = 2017969993_u32 << _1;
_24 = -_19;
_1 = _8 << _7;
_22 = _18 + _18;
_32 = [_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3];
_12.0 = &_14.1;
_24 = _10;
_29 = _24 + _10;
_19 = -_24;
Call(_8 = core::intrinsics::transmute(_1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = _6;
_10 = _24;
_28 = &_26.fld3;
_33.0 = _10 as i128;
_28 = &_26.fld3;
_5 = _8;
_26.fld3 = 731897067_i32 as i8;
_26.fld1 = _19 as usize;
_6 = [(-88694742_i32),(-1421410025_i32),(-1486491789_i32),(-1367510275_i32),608092641_i32,(-1762901383_i32),1138671464_i32];
_14.1 = '\u{31e54}' as u128;
_33.1 = core::ptr::addr_of!(_14.0);
_34 = _20;
_14.0 = (*_25) as usize;
_34 = !_20;
_11 = !_26.fld2;
_33.1 = core::ptr::addr_of!(_14.0);
Goto(bb9)
}
bb9 = {
_29 = _14.1 as f64;
_10 = -_24;
_37 = '\u{8be5a}';
_36.1 = &(*_25);
match _26.fld4 {
0 => bb1,
1 => bb2,
2 => bb3,
12379 => bb10,
_ => bb4
}
}
bb10 = {
_36.3 = _14.1 as u64;
_32 = [_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3,_26.fld3];
_33.2 = 169_u8 >> _15;
_26.fld1 = _26.fld3 as usize;
_12.0 = &_14.1;
_36.2 = Move(_12.0);
_36.1 = &_40;
_14 = (_26.fld1, 302152977978470541394088980258515742337_u128);
_42 = _26.fld3 as isize;
_25 = core::ptr::addr_of!((*_25));
_33.2 = 94_u8;
_19 = _24;
_37 = '\u{f8594}';
_15 = _36.3 as u32;
_2 = _6;
_43 = &_26.fld3;
Goto(bb11)
}
bb11 = {
_41 = Adt53::Variant1 { fld0: _34,fld1: _36.3 };
_17 = _22 + _18;
_38 = [_18,_17,_26.fld2,_17,_26.fld2,_11];
place!(Field::<bool>(Variant(_41, 1), 0)) = !_34;
_36.2 = &_14.1;
_35 = _37;
_12.0 = &_14.1;
_45 = _26.fld0;
_46 = _36.3;
_25 = core::ptr::addr_of!(_40);
_38 = [_22,_11,_22,_17,_26.fld2,_11];
_12.0 = &_14.1;
_14.1 = 84798355333243885991940944298563011614_u128;
_47.fld1 = !_31;
_42 = _26.fld2;
SetDiscriminant(_41, 1);
_19 = _10;
_26.fld4 = !15555_i16;
_44 = [_1,_5,_1,_5,_8,_1];
_7 = _33.2 as i128;
_31 = _47.fld1 - _47.fld1;
_9 = &_36.1;
_18 = !_11;
match _14.1 {
0 => bb10,
1 => bb2,
2 => bb3,
84798355333243885991940944298563011614 => bb13,
_ => bb12
}
}
bb12 = {
_29 = _14.1 as f64;
_10 = -_24;
_37 = '\u{8be5a}';
_36.1 = &(*_25);
match _26.fld4 {
0 => bb1,
1 => bb2,
2 => bb3,
12379 => bb10,
_ => bb4
}
}
bb13 = {
_16 = _26.fld4 as f32;
_50 = [(-1196802922_i32),(-1996134064_i32),1070490179_i32,(-1793120644_i32),(-1153007364_i32)];
_48 = _2;
_51 = _7 as f32;
_33.1 = core::ptr::addr_of!(_47.fld1);
_34 = _1 != _5;
_36.2 = &_14.1;
_28 = &_26.fld3;
_33.2 = 24_u8 ^ 119_u8;
_47 = Move(_26);
_25 = core::ptr::addr_of!(_15);
_36.1 = &_15;
_47.fld2 = _33.0 as isize;
_2 = [1727969900_i32,(-1196596906_i32),2036283651_i32,1675086044_i32,(-248784293_i32),(-1757686441_i32),1356014348_i32];
(*_25) = 1841787847_u32 - 2598257963_u32;
_29 = _19;
_28 = &_26.fld3;
_10 = _24 + _29;
match _14.1 {
0 => bb8,
1 => bb10,
2 => bb12,
3 => bb14,
84798355333243885991940944298563011614 => bb16,
_ => bb15
}
}
bb14 = {
_7 = -_4;
_5 = !RET;
_13 = [(-1049139464_i32),(-1180854777_i32),1020245925_i32,722397561_i32,(-1743333628_i32)];
_14.1 = 31142551828690820747132391227490717235_u128;
_14.0 = 5_usize;
_11 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_13 = [1475055440_i32,(-871053709_i32),(-1303319525_i32),1829211491_i32,(-1131558710_i32)];
_8 = !_5;
Goto(bb3)
}
bb15 = {
_26.fld0 = [174_u8,246_u8,153_u8,72_u8];
_6 = _2;
_4 = _7;
_20 = !false;
_26.fld2 = _17 - _11;
_20 = !true;
_22 = -_11;
_26.fld4 = 21888_i16;
_14 = (5_usize, 159847962448234540007944475543228195112_u128);
_12.0 = &_14.1;
_15 = 896587421_u32;
_22 = _11;
_19 = _10;
_26.fld3 = -(-26_i8);
_29 = _10 - _19;
_26.fld4 = 15184_i16 | 22205_i16;
_26.fld1 = 882651561092700000_u64 as usize;
_26.fld3 = (-18_i8);
_1 = !_8;
Goto(bb6)
}
bb16 = {
_43 = &(*_28);
_45 = _47.fld0;
_41 = Adt53::Variant0 { fld0: _31 };
_52 = _8 - _5;
_14 = (Field::<usize>(Variant(_41, 0), 0), 201926824888158443413158260144914424361_u128);
_26.fld0 = [_33.2,_33.2,_33.2,_33.2];
_31 = !Field::<usize>(Variant(_41, 0), 0);
_56 = _47.fld3 & _47.fld3;
_10 = _24;
_26 = Move(_47);
_38 = [_42,_17,_26.fld2,_11,_42,_17];
_45 = [_33.2,_33.2,_33.2,_33.2];
_26 = Adt24 { fld0: _45,fld1: _31,fld2: _42,fld3: _56,fld4: (-114_i16) };
place!(Field::<usize>(Variant(_41, 0), 0)) = !_31;
_41 = Adt53::Variant0 { fld0: _26.fld1 };
_43 = &_47.fld3;
_19 = _14.1 as f64;
_30 = &_6;
_26.fld2 = _11 - _42;
_16 = -_51;
_37 = _35;
Goto(bb17)
}
bb17 = {
Call(_60 = dump_var(9_usize, 5_usize, Move(_5), 13_usize, Move(_13), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_60 = dump_var(9_usize, 18_usize, Move(_18), 17_usize, Move(_17), 42_usize, Move(_42), 48_usize, Move(_48)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_60 = dump_var(9_usize, 2_usize, Move(_2), 4_usize, Move(_4), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_60 = dump_var(9_usize, 32_usize, Move(_32), 20_usize, Move(_20), 46_usize, Move(_46), 61_usize, _61), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i32; 7],mut _2: [i32; 7],mut _3: i64,mut _4: [i32; 7],mut _5: i64,mut _6: [i32; 7],mut _7: i64,mut _8: [i32; 7],mut _9: i64) -> i128 {
mir! {
type RET = i128;
let _10: (i128, *const usize, u8);
let _11: (usize, u128);
let _12: ([i32; 7], &'static Adt47);
let _13: *const ([u16; 8],);
let _14: char;
let _15: isize;
let _16: &'static (usize, i16);
let _17: [i32; 7];
let _18: isize;
let _19: f64;
let _20: [char; 2];
let _21: Adt42;
let _22: &'static Adt30;
let _23: char;
let _24: f64;
let _25: &'static u32;
let _26: i8;
let _27: u16;
let _28: char;
let _29: &'static Adt30;
let _30: isize;
let _31: *mut (i8, (i64, u8, i64), char, i128);
let _32: bool;
let _33: isize;
let _34: &'static &'static u32;
let _35: u32;
let _36: Adt63;
let _37: isize;
let _38: isize;
let _39: (&'static u128,);
let _40: u32;
let _41: (&'static i8, ((u16,),), Adt24);
let _42: &'static (usize, i16);
let _43: ();
let _44: ();
{
_8 = [1551731909_i32,1811025671_i32,(-1409038216_i32),(-922765255_i32),89548112_i32,(-583493832_i32),628837994_i32];
RET = '\u{5a019}' as i128;
_5 = 117316219200022982048685165186246505790_u128 as i64;
_2 = [(-2035697403_i32),(-2111984970_i32),990353721_i32,1397096589_i32,(-1881945295_i32),(-145966022_i32),(-2033091257_i32)];
_7 = _3;
_6 = _8;
_7 = !_3;
_1 = [(-726065806_i32),80989463_i32,(-1097741675_i32),(-565185505_i32),(-1551542599_i32),(-1340165639_i32),1721150937_i32];
RET = (-60938427924495263323369655565798613646_i128) | 146962891902465885753810803717862815889_i128;
_1 = _2;
_8 = [(-1659256036_i32),1435914481_i32,(-834689329_i32),(-2068759396_i32),(-286364071_i32),(-2145559873_i32),(-1526502920_i32)];
_8 = _2;
RET = 169966242705303384949399054185916110828_i128 >> _5;
_11.1 = 170592113355086036632208154242152635987_u128;
_7 = -_9;
_10.2 = 86_u8 | 241_u8;
_8 = [1063209704_i32,580977104_i32,1975393926_i32,2021033692_i32,816967628_i32,(-1894520250_i32),(-1522356638_i32)];
_4 = [1160047170_i32,327330746_i32,(-1838158877_i32),(-2109395220_i32),755594996_i32,218904526_i32,(-1547596178_i32)];
_15 = '\u{9c051}' as isize;
_10.0 = RET * RET;
RET = _11.1 as i128;
_6 = _1;
_11 = (12711227331703144987_usize, 307425579993130828579883173866765505108_u128);
match _11.0 {
12711227331703144987 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_12.0 = [(-1392953938_i32),670077927_i32,(-548585156_i32),(-1290086756_i32),(-1648987543_i32),301206719_i32,1928266865_i32];
_6 = _8;
_14 = '\u{a9995}';
_5 = _3 << _10.0;
_4 = _2;
_11.0 = 2_usize;
_3 = _5 << _9;
_14 = '\u{4d938}';
_7 = _9;
_6 = [(-397104673_i32),(-803261227_i32),(-2070377184_i32),924565962_i32,(-48396123_i32),(-1400438306_i32),(-1015397381_i32)];
_12.0 = _2;
_9 = !_5;
_18 = 32980_u16 as isize;
_18 = _15 >> _5;
_6 = _1;
RET = _10.0;
_6 = [1860149823_i32,1025953595_i32,(-119314744_i32),160132731_i32,(-924378770_i32),583383832_i32,889972743_i32];
_3 = -_9;
_17 = [(-530075679_i32),(-1123139385_i32),1740382717_i32,1347946905_i32,(-1174130160_i32),(-1029126062_i32),1298318550_i32];
match _11.0 {
0 => bb3,
1 => bb4,
2 => bb6,
_ => bb5
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
_9 = !_7;
RET = -_10.0;
RET = _10.0;
_15 = !_18;
_9 = _3 - _7;
_2 = [856846524_i32,1022523202_i32,3789510_i32,(-569265157_i32),(-1477707479_i32),(-1739610176_i32),(-1290119976_i32)];
_19 = 39555_u16 as f64;
_1 = _8;
_11.1 = !321705609220288457045323582766426878321_u128;
_23 = _14;
_10.2 = !191_u8;
_10.1 = core::ptr::addr_of!(_11.0);
Goto(bb7)
}
bb7 = {
_15 = _18 ^ _18;
_11.1 = 332874552074786139553263666881914997054_u128 & 21037648943287250271584189975258956286_u128;
_10.2 = 59_u8;
_7 = _10.0 as i64;
_20 = [_14,_14];
_11 = (3001124829504149982_usize, 232392943140483587758644766476222638431_u128);
_5 = _9;
_6 = _2;
_11.1 = 50133138169097225265961759015734956504_u128 >> _15;
_1 = _4;
_9 = (-114_i8) as i64;
_20 = [_23,_23];
_11.1 = !227130554738713234166222954786710443630_u128;
_17 = [1056186219_i32,1714038004_i32,(-560845653_i32),903222559_i32,(-736382964_i32),(-198010203_i32),(-1181408582_i32)];
Goto(bb8)
}
bb8 = {
RET = _10.0;
RET = _10.0 >> _15;
_19 = _15 as f64;
_19 = 1808693174_i32 as f64;
_28 = _14;
_18 = -_15;
_24 = -_19;
RET = _10.0;
_14 = _23;
_26 = 19_i8 + 71_i8;
_10.2 = _23 as u8;
_2 = [1711021854_i32,(-1445442821_i32),1228885290_i32,1654839788_i32,(-1664454221_i32),(-1505009581_i32),(-924119685_i32)];
Goto(bb9)
}
bb9 = {
_5 = !_7;
_30 = -_15;
_12.0 = [(-2031595671_i32),2100160970_i32,1481387223_i32,(-319130666_i32),1681914577_i32,1925584986_i32,(-1646453803_i32)];
_2 = [(-177844163_i32),1999380281_i32,1606004688_i32,1920944688_i32,1422518676_i32,80052266_i32,(-1649078549_i32)];
_3 = _5;
_7 = 28977_i16 as i64;
_11.1 = 151110213842736396960357269834572385257_u128 >> _15;
_9 = _5 >> _18;
_30 = -_15;
_9 = _3;
_11.0 = !2_usize;
_28 = _23;
_11.0 = 5_usize;
_11 = (4_usize, 275195321445255761304409404714676604414_u128);
_4 = [640518726_i32,1145632100_i32,(-1310168753_i32),(-935351075_i32),(-1252041149_i32),1887609201_i32,(-478117581_i32)];
_12.0 = [(-1619355384_i32),(-531205331_i32),391146332_i32,1995697091_i32,334610076_i32,505029897_i32,2043814485_i32];
Goto(bb10)
}
bb10 = {
_32 = !false;
RET = _10.0;
RET = _10.0 & _10.0;
_32 = !false;
_9 = -_5;
_27 = 10327_u16 * 796_u16;
_20 = [_28,_23];
_11.1 = 70378655331900402780804275842203185475_u128;
_24 = _26 as f64;
_20 = [_23,_14];
_27 = _11.1 as u16;
_7 = !_3;
_8 = [6655828_i32,(-1891723265_i32),326072121_i32,(-1543658171_i32),1752916816_i32,135745147_i32,(-54112232_i32)];
_2 = [2043520699_i32,(-809914707_i32),(-1358646184_i32),(-1170927255_i32),(-1995252651_i32),(-1075955101_i32),1967621377_i32];
_1 = [959224904_i32,465269920_i32,378899925_i32,2065525419_i32,1265781599_i32,(-467795464_i32),(-1960622025_i32)];
_11.1 = !72413619653766384057405425471828960306_u128;
_11.1 = 131566794379996580870395927631999846930_u128 * 58015461653701234276717153203588653181_u128;
match _11.0 {
0 => bb6,
1 => bb11,
4 => bb13,
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
_10.2 = 121_u8;
_15 = _30;
_11 = (6516029615483322785_usize, 262493686605542666666638791720529049518_u128);
_17 = [1542562666_i32,(-1750332126_i32),(-1379174844_i32),796402201_i32,3375876_i32,1100393577_i32,258435206_i32];
_11.1 = 337064377803020993101105728634281256492_u128;
RET = !_10.0;
_27 = 30464_u16 >> _15;
_3 = _10.0 as i64;
_26 = (-112_i8) | 31_i8;
RET = _10.0;
_1 = [1781422537_i32,965379379_i32,64140383_i32,(-1792217442_i32),(-366666842_i32),(-1127376785_i32),(-39659096_i32)];
_1 = [(-355150262_i32),(-1858062070_i32),(-1852716632_i32),(-752578848_i32),(-1793072520_i32),(-1110705718_i32),319535463_i32];
RET = !_10.0;
_3 = !_7;
_23 = _14;
_34 = &_25;
_33 = _10.2 as isize;
Call(_13 = fn11(_18), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15 = _18;
RET = _10.0;
_12.0 = _1;
_17 = _8;
_23 = _14;
_17 = [(-1318013633_i32),2106428096_i32,1328283963_i32,(-1173433935_i32),1225048577_i32,1690073512_i32,1817480554_i32];
RET = _10.0 * _10.0;
_34 = &(*_34);
_19 = _24 - _24;
_27 = _24 as u16;
_3 = _5;
_12.0 = [2064637165_i32,997002233_i32,1050196794_i32,(-1995752630_i32),(-1553147662_i32),423711384_i32,(-681332497_i32)];
_27 = 45615_u16;
_1 = _8;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(10_usize, 28_usize, Move(_28), 18_usize, Move(_18), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(10_usize, 5_usize, Move(_5), 20_usize, Move(_20), 32_usize, Move(_32), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(10_usize, 8_usize, Move(_8), 2_usize, Move(_2), 33_usize, Move(_33), 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize) -> *const ([u16; 8],) {
mir! {
type RET = *const ([u16; 8],);
let _2: char;
let _3: [i64; 6];
let _4: isize;
let _5: i32;
let _6: i8;
let _7: i64;
let _8: *const &'static u128;
let _9: i16;
let _10: [i64; 6];
let _11: f32;
let _12: Adt53;
let _13: [isize; 6];
let _14: i8;
let _15: isize;
let _16: (&'static u128,);
let _17: u128;
let _18: bool;
let _19: [i8; 4];
let _20: &'static u64;
let _21: [char; 2];
let _22: *mut (usize, i16);
let _23: bool;
let _24: f64;
let _25: char;
let _26: i32;
let _27: char;
let _28: bool;
let _29: f32;
let _30: f64;
let _31: Adt47;
let _32: char;
let _33: i128;
let _34: &'static ((u16,),);
let _35: Adt63;
let _36: &'static u128;
let _37: *mut (usize, i16);
let _38: (i128, *const usize, u8);
let _39: isize;
let _40: u32;
let _41: &'static i8;
let _42: i8;
let _43: i16;
let _44: f64;
let _45: (&'static u32, char);
let _46: isize;
let _47: ([u16; 8],);
let _48: (bool, (bool, u32, i32, bool), u128);
let _49: &'static [i32; 7];
let _50: isize;
let _51: &'static [u16; 4];
let _52: &'static u128;
let _53: bool;
let _54: char;
let _55: *mut Adt63;
let _56: f64;
let _57: usize;
let _58: (((u16,),), Adt47, (bool, u32, i32, bool));
let _59: [isize; 6];
let _60: (bool, u32, i32, bool);
let _61: isize;
let _62: ();
let _63: ();
{
_1 = (-16_isize) + (-9223372036854775808_isize);
_2 = '\u{102ce6}';
_2 = '\u{61433}';
_2 = '\u{4f41c}';
_2 = '\u{19ddd}';
_1 = -9223372036854775807_isize;
_2 = '\u{ea96a}';
_1 = (-45_isize) * (-9223372036854775808_isize);
_1 = 104_isize ^ 9223372036854775807_isize;
_2 = '\u{e3487}';
_2 = '\u{1cf76}';
_2 = '\u{58e81}';
_2 = '\u{fe5b8}';
_3 = [3838759960508020973_i64,(-3621450858955961091_i64),(-6919241136870506649_i64),(-3025776836443054336_i64),6516329671944942322_i64,335192538873433555_i64];
Goto(bb1)
}
bb1 = {
_1 = 9223372036854775807_isize | (-9223372036854775808_isize);
_1 = !9223372036854775807_isize;
_3 = [682593842599638363_i64,(-6586301122996075470_i64),8271193979890960281_i64,7354233657868517815_i64,4516206250803946635_i64,(-4900705859704157503_i64)];
_3 = [(-5330681567781067637_i64),6604633172062173870_i64,3295796150928472015_i64,(-5893224347581392333_i64),(-307351407855986377_i64),(-6530923445783146486_i64)];
_1 = 9223372036854775807_isize;
_1 = 234_u8 as isize;
_3 = [4171669683184210208_i64,(-6699310140933208119_i64),(-3792361775798656652_i64),(-1384453068244542498_i64),(-7429126832278020409_i64),2221024226028160535_i64];
_4 = -_1;
_1 = -_4;
_1 = true as isize;
_3 = [3770463361651407852_i64,677674448166234788_i64,(-9066981575494766952_i64),2608046738042522295_i64,(-2051286831138617799_i64),(-3133145447018464193_i64)];
_5 = 1232865236_i32;
_1 = _4;
_4 = -_1;
_6 = 88_i8;
_3 = [6880013126617375201_i64,7380835322007512137_i64,1439564531892916628_i64,7180670090125634057_i64,921308914223072423_i64,3486780126636348172_i64];
_2 = '\u{1e71a}';
_3 = [4365856836851082469_i64,6002182397316495245_i64,9038509378111399231_i64,8249182761533409335_i64,(-1239108650026030205_i64),506446626268587842_i64];
_3 = [(-6821909953361028162_i64),896367215713104598_i64,(-1276204952005241538_i64),4836804681506942549_i64,4673612297403338753_i64,(-6430945249660067456_i64)];
_4 = 55138_u16 as isize;
_6 = 63_i8 * 46_i8;
_6 = 44_i8;
_6 = 40_i8 | (-103_i8);
_5 = 68_u8 as i32;
_4 = !_1;
Goto(bb2)
}
bb2 = {
_4 = !_1;
_6 = -(-43_i8);
_1 = _4 * _4;
_4 = _1 - _1;
_9 = !(-21588_i16);
_10 = [(-6567384864331934632_i64),1763994951503855038_i64,8442993430398576296_i64,7552432578738996316_i64,8342242195409064988_i64,7335376059096329096_i64];
_3 = [(-5668532789129004966_i64),(-7129857212128012326_i64),5818883394488634277_i64,7069366493359098641_i64,1672424805574141722_i64,(-4819941266282046369_i64)];
_7 = -(-9007866813744206052_i64);
_3 = [_7,_7,_7,_7,_7,_7];
_9 = 32158_i16;
_9 = true as i16;
_10 = _3;
_1 = _4;
_7 = (-8078963262386789323_i64);
_7 = 6282921877915225034_i64 - 5671971503127157872_i64;
_9 = 1747_i16;
_9 = (-4807_i16) >> _6;
_2 = '\u{e2d67}';
_7 = 13751363157189699489_u64 as i64;
_2 = '\u{46686}';
_6 = 97_i8 ^ (-41_i8);
_1 = _4 + _4;
_7 = _6 as i64;
_10 = [_7,_7,_7,_7,_7,_7];
Goto(bb3)
}
bb3 = {
_3 = _10;
_2 = '\u{1006b5}';
_12 = Adt53::Variant1 { fld0: true,fld1: 304272467802417062_u64 };
_13 = [_1,_4,_1,_4,_1,_1];
_5 = _2 as i32;
_9 = (-6133_i16);
_4 = _1;
_2 = '\u{fe827}';
_6 = -99_i8;
_9 = !(-678_i16);
_11 = _4 as f32;
_2 = '\u{992a8}';
Goto(bb4)
}
bb4 = {
place!(Field::<bool>(Variant(_12, 1), 0)) = _1 == _1;
_1 = _4;
place!(Field::<u64>(Variant(_12, 1), 1)) = 9488813354722106867_u64;
_14 = _6;
_15 = 12445726996979374718426687478714224101_u128 as isize;
_3 = [_7,_7,_7,_7,_7,_7];
Call(_10 = fn12(_11, Field::<bool>(Variant(_12, 1), 0), _13, _13, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<u64>(Variant(_12, 1), 1)) = 5186127731569199812_u64;
_13 = [_4,_1,_4,_4,_4,_1];
place!(Field::<u64>(Variant(_12, 1), 1)) = 2368846176251984475_u64 - 7850984992737336296_u64;
place!(Field::<bool>(Variant(_12, 1), 0)) = false;
_14 = _6;
_2 = '\u{2dec6}';
_14 = _6 + _6;
_9 = 1931_u16 as i16;
Goto(bb6)
}
bb6 = {
SetDiscriminant(_12, 1);
place!(Field::<u64>(Variant(_12, 1), 1)) = 1226505722467549980_u64 - 12309256672452225226_u64;
place!(Field::<bool>(Variant(_12, 1), 0)) = !false;
place!(Field::<u64>(Variant(_12, 1), 1)) = !8491945611096963718_u64;
_7 = (-5035720763136889765_i64);
_8 = core::ptr::addr_of!(_16.0);
_2 = '\u{26b97}';
_4 = _1 >> _6;
_5 = _6 as i32;
_4 = !_1;
_1 = _4 << _4;
_6 = -_14;
SetDiscriminant(_12, 1);
_12 = Adt53::Variant1 { fld0: true,fld1: 5058558532610324339_u64 };
_5 = 14962962694319753089_u64 as i32;
_6 = !_14;
_13 = [_4,_1,_1,_4,_1,_1];
place!(Field::<u64>(Variant(_12, 1), 1)) = !12937553000918104553_u64;
_3 = _10;
_1 = _4;
_3 = _10;
_11 = _4 as f32;
(*_8) = &_17;
_13 = [_4,_1,_4,_1,_4,_15];
_12 = Adt53::Variant0 { fld0: 16172965699442916787_usize };
_12 = Adt53::Variant0 { fld0: 695778558964059498_usize };
_8 = core::ptr::addr_of!((*_8));
place!(Field::<usize>(Variant(_12, 0), 0)) = !17109866700860147350_usize;
_6 = (-169269943892811143106041108127806539883_i128) as i8;
Goto(bb7)
}
bb7 = {
_18 = _11 <= _11;
_13 = [_1,_1,_4,_1,_4,_4];
_17 = 173374384646696159238214862676014233938_u128;
_13 = [_4,_4,_4,_15,_1,_1];
_4 = !_1;
_9 = (-131287851079638097564462489844595411965_i128) as i16;
_9 = Field::<usize>(Variant(_12, 0), 0) as i16;
(*_8) = &_17;
_18 = true & true;
(*_8) = &_17;
_17 = 2123579761_u32 as u128;
_7 = (-6308040243419422703_i64) & 8846733300291526874_i64;
_9 = _14 as i16;
_18 = true | false;
(*_8) = &_17;
Goto(bb8)
}
bb8 = {
_13 = [_4,_1,_1,_1,_1,_4];
_8 = core::ptr::addr_of!((*_8));
_3 = [_7,_7,_7,_7,_7,_7];
_19 = [_6,_14,_14,_14];
_2 = '\u{1aeeb}';
_18 = !true;
place!(Field::<usize>(Variant(_12, 0), 0)) = 8495745296783448559_usize;
_21 = [_2,_2];
_8 = core::ptr::addr_of!((*_8));
_14 = _6;
_14 = -_6;
_14 = _6 << _4;
SetDiscriminant(_12, 2);
place!(Field::<bool>(Variant(_12, 2), 0)) = _18 | _18;
_13 = [_4,_4,_4,_4,_4,_1];
Call(_14 = core::intrinsics::transmute(Field::<bool>(Variant(_12, 2), 0)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = _9 as isize;
_11 = _4 as f32;
place!(Field::<i32>(Variant(_12, 2), 2)) = _17 as i32;
_12 = Adt53::Variant0 { fld0: 5_usize };
_21 = [_2,_2];
_2 = '\u{1048ce}';
_1 = _5 as isize;
_2 = '\u{61dff}';
_19 = [_14,_14,_14,_6];
(*_8) = &_17;
_26 = _5 ^ _5;
_6 = _7 as i8;
Goto(bb10)
}
bb10 = {
(*_8) = &_17;
_19 = [_14,_14,_6,_6];
_13 = [_4,_4,_4,_4,_1,_4];
_18 = !true;
_16.0 = &_17;
_28 = _18;
_13 = [_4,_4,_4,_4,_15,_4];
_6 = _14 & _14;
_27 = _2;
_21 = [_27,_27];
(*_8) = &_17;
_24 = 2624102298_u32 as f64;
Call(_29 = fn16(Move(_8), _18, _17, _6, _6, _11, _28), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_23 = _29 > _29;
_5 = _26;
_3 = [_7,_7,_7,_7,_7,_7];
_1 = !_4;
Goto(bb12)
}
bb12 = {
_15 = -_1;
_28 = !_23;
_2 = _27;
_3 = _10;
_3 = [_7,_7,_7,_7,_7,_7];
_28 = _23 | _23;
_24 = 15117673012852896826_usize as f64;
place!(Field::<usize>(Variant(_12, 0), 0)) = !654877805920727868_usize;
_17 = _1 as u128;
_9 = !11924_i16;
_31 = Adt47::Variant0 { fld0: _7,fld1: (-105036371323380065639998382940037066346_i128),fld2: 179_u8,fld3: _6,fld4: _19,fld5: 38800_u16 };
_26 = _5 >> _14;
_8 = core::ptr::addr_of!(_16.0);
_31 = Adt47::Variant0 { fld0: _7,fld1: 100383229999527132272502015145169168841_i128,fld2: 208_u8,fld3: _6,fld4: _19,fld5: 65217_u16 };
(*_8) = &_17;
_27 = _2;
place!(Field::<i128>(Variant(_31, 0), 1)) = _27 as i128;
_16.0 = &_17;
SetDiscriminant(_12, 0);
place!(Field::<i128>(Variant(_31, 0), 1)) = !(-136100230728676517872093668818818005053_i128);
place!(Field::<usize>(Variant(_12, 0), 0)) = 13963442604459846868_usize;
place!(Field::<i128>(Variant(_31, 0), 1)) = !23072122637094335248902281878447975634_i128;
_6 = _14;
Goto(bb13)
}
bb13 = {
_5 = _26;
_9 = !24842_i16;
_6 = _14 - _14;
_15 = _4;
_28 = !_23;
match Field::<usize>(Variant(_12, 0), 0) {
0 => bb12,
1 => bb14,
2 => bb15,
3 => bb16,
13963442604459846868 => bb18,
_ => bb17
}
}
bb14 = {
place!(Field::<bool>(Variant(_12, 1), 0)) = _1 == _1;
_1 = _4;
place!(Field::<u64>(Variant(_12, 1), 1)) = 9488813354722106867_u64;
_14 = _6;
_15 = 12445726996979374718426687478714224101_u128 as isize;
_3 = [_7,_7,_7,_7,_7,_7];
Call(_10 = fn12(_11, Field::<bool>(Variant(_12, 1), 0), _13, _13, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_1 = 9223372036854775807_isize | (-9223372036854775808_isize);
_1 = !9223372036854775807_isize;
_3 = [682593842599638363_i64,(-6586301122996075470_i64),8271193979890960281_i64,7354233657868517815_i64,4516206250803946635_i64,(-4900705859704157503_i64)];
_3 = [(-5330681567781067637_i64),6604633172062173870_i64,3295796150928472015_i64,(-5893224347581392333_i64),(-307351407855986377_i64),(-6530923445783146486_i64)];
_1 = 9223372036854775807_isize;
_1 = 234_u8 as isize;
_3 = [4171669683184210208_i64,(-6699310140933208119_i64),(-3792361775798656652_i64),(-1384453068244542498_i64),(-7429126832278020409_i64),2221024226028160535_i64];
_4 = -_1;
_1 = -_4;
_1 = true as isize;
_3 = [3770463361651407852_i64,677674448166234788_i64,(-9066981575494766952_i64),2608046738042522295_i64,(-2051286831138617799_i64),(-3133145447018464193_i64)];
_5 = 1232865236_i32;
_1 = _4;
_4 = -_1;
_6 = 88_i8;
_3 = [6880013126617375201_i64,7380835322007512137_i64,1439564531892916628_i64,7180670090125634057_i64,921308914223072423_i64,3486780126636348172_i64];
_2 = '\u{1e71a}';
_3 = [4365856836851082469_i64,6002182397316495245_i64,9038509378111399231_i64,8249182761533409335_i64,(-1239108650026030205_i64),506446626268587842_i64];
_3 = [(-6821909953361028162_i64),896367215713104598_i64,(-1276204952005241538_i64),4836804681506942549_i64,4673612297403338753_i64,(-6430945249660067456_i64)];
_4 = 55138_u16 as isize;
_6 = 63_i8 * 46_i8;
_6 = 44_i8;
_6 = 40_i8 | (-103_i8);
_5 = 68_u8 as i32;
_4 = !_1;
Goto(bb2)
}
bb16 = {
_13 = [_4,_1,_1,_1,_1,_4];
_8 = core::ptr::addr_of!((*_8));
_3 = [_7,_7,_7,_7,_7,_7];
_19 = [_6,_14,_14,_14];
_2 = '\u{1aeeb}';
_18 = !true;
place!(Field::<usize>(Variant(_12, 0), 0)) = 8495745296783448559_usize;
_21 = [_2,_2];
_8 = core::ptr::addr_of!((*_8));
_14 = _6;
_14 = -_6;
_14 = _6 << _4;
SetDiscriminant(_12, 2);
place!(Field::<bool>(Variant(_12, 2), 0)) = _18 | _18;
_13 = [_4,_4,_4,_4,_4,_1];
Call(_14 = core::intrinsics::transmute(Field::<bool>(Variant(_12, 2), 0)), ReturnTo(bb9), UnwindUnreachable())
}
bb17 = {
_3 = _10;
_2 = '\u{1006b5}';
_12 = Adt53::Variant1 { fld0: true,fld1: 304272467802417062_u64 };
_13 = [_1,_4,_1,_4,_1,_1];
_5 = _2 as i32;
_9 = (-6133_i16);
_4 = _1;
_2 = '\u{fe827}';
_6 = -99_i8;
_9 = !(-678_i16);
_11 = _4 as f32;
_2 = '\u{992a8}';
Goto(bb4)
}
bb18 = {
_7 = !Field::<i64>(Variant(_31, 0), 0);
_25 = _27;
place!(Field::<u8>(Variant(_31, 0), 2)) = 55_u8;
SetDiscriminant(_12, 2);
place!(Field::<(i8, bool, i64)>(Variant(_12, 2), 3)).2 = _27 as i64;
place!(Field::<(i8, bool, i64)>(Variant(_12, 2), 3)).1 = _28;
place!(Field::<u16>(Variant(_31, 0), 5)) = _26 as u16;
_30 = -_24;
place!(Field::<i128>(Variant(_12, 2), 1)) = _23 as i128;
_36 = &_17;
_16.0 = &(*_36);
_13 = [_15,_4,_15,_15,_1,_1];
SetDiscriminant(_31, 0);
(*_8) = &(*_36);
_28 = _6 < _14;
place!(Field::<i8>(Variant(_31, 0), 3)) = !_14;
_23 = _28;
Call(_9 = fn17(Move(_8), Move(_36), Field::<(i8, bool, i64)>(Variant(_12, 2), 3).1, _1, _11, Field::<i128>(Variant(_12, 2), 1), _3, Field::<i128>(Variant(_12, 2), 1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
place!(Field::<[i8; 4]>(Variant(_31, 0), 4)) = [_6,_6,Field::<i8>(Variant(_31, 0), 3),_14];
_24 = _29 as f64;
place!(Field::<(i8, bool, i64)>(Variant(_12, 2), 3)) = (Field::<i8>(Variant(_31, 0), 3), _18, _7);
_14 = !Field::<(i8, bool, i64)>(Variant(_12, 2), 3).0;
_24 = 14892580563848834716_u64 as f64;
place!(Field::<i64>(Variant(_31, 0), 0)) = _7 >> _17;
_32 = _2;
_21 = [_2,_32];
_16.0 = &_17;
_31 = Adt47::Variant0 { fld0: Field::<(i8, bool, i64)>(Variant(_12, 2), 3).2,fld1: Field::<i128>(Variant(_12, 2), 1),fld2: 131_u8,fld3: Field::<(i8, bool, i64)>(Variant(_12, 2), 3).0,fld4: _19,fld5: 7681_u16 };
_7 = Field::<(i8, bool, i64)>(Variant(_12, 2), 3).2 + Field::<(i8, bool, i64)>(Variant(_12, 2), 3).2;
_39 = _15 ^ _1;
place!(Field::<u8>(Variant(_31, 0), 2)) = !11_u8;
_19 = Field::<[i8; 4]>(Variant(_31, 0), 4);
_38.2 = _17 as u8;
place!(Field::<i32>(Variant(_12, 2), 2)) = -_26;
_16.0 = &_17;
_32 = _2;
_41 = &place!(Field::<i8>(Variant(_31, 0), 3));
_5 = _26 >> Field::<i128>(Variant(_12, 2), 1);
Goto(bb20)
}
bb20 = {
place!(Field::<u16>(Variant(_31, 0), 5)) = !51162_u16;
_38.0 = Field::<i128>(Variant(_12, 2), 1);
_40 = !362570856_u32;
SetDiscriminant(_31, 0);
_36 = &_17;
_3 = _10;
place!(Field::<u16>(Variant(_31, 0), 5)) = 40077_u16;
match Field::<u16>(Variant(_31, 0), 5) {
0 => bb10,
1 => bb3,
40077 => bb22,
_ => bb21
}
}
bb21 = {
place!(Field::<bool>(Variant(_12, 1), 0)) = _1 == _1;
_1 = _4;
place!(Field::<u64>(Variant(_12, 1), 1)) = 9488813354722106867_u64;
_14 = _6;
_15 = 12445726996979374718426687478714224101_u128 as isize;
_3 = [_7,_7,_7,_7,_7,_7];
Call(_10 = fn12(_11, Field::<bool>(Variant(_12, 1), 0), _13, _13, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb22 = {
_36 = &_17;
_14 = Field::<(i8, bool, i64)>(Variant(_12, 2), 3).0;
_38.2 = 137_u8 | 159_u8;
_41 = &_6;
_27 = _25;
_45.0 = &_40;
_44 = _24;
place!(Field::<bool>(Variant(_12, 2), 0)) = _23;
_26 = !_5;
SetDiscriminant(_12, 0);
_33 = _38.0;
place!(Field::<i128>(Variant(_31, 0), 1)) = _40 as i128;
_13 = [_4,_1,_39,_4,_4,_1];
place!(Field::<i128>(Variant(_31, 0), 1)) = -_38.0;
_43 = 17433319492880088700_usize as i16;
_24 = -_30;
_42 = (*_41);
_40 = 2044153114_u32;
_8 = core::ptr::addr_of!(_16.0);
_46 = -_1;
_12 = Adt53::Variant1 { fld0: _28,fld1: 17180643570023936899_u64 };
_48.1.2 = _26;
_36 = &(*_36);
place!(Field::<u8>(Variant(_31, 0), 2)) = _40 as u8;
_12 = Adt53::Variant1 { fld0: _28,fld1: 2219045492843355875_u64 };
place!(Field::<[i8; 4]>(Variant(_31, 0), 4)) = [_42,(*_41),(*_41),(*_41)];
_47.0 = [Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5)];
Goto(bb23)
}
bb23 = {
_48.2 = (*_36) * _17;
_45.1 = _32;
_1 = _39 & _4;
Goto(bb24)
}
bb24 = {
_48.1.1 = 15763402439644972433_u64 as u32;
_41 = &place!(Field::<i8>(Variant(_31, 0), 3));
_5 = -_48.1.2;
place!(Field::<u16>(Variant(_31, 0), 5)) = !39936_u16;
_48.0 = !Field::<bool>(Variant(_12, 1), 0);
_48.0 = _28;
place!(Field::<u8>(Variant(_31, 0), 2)) = _14 as u8;
_36 = &(*_36);
_47.0 = [Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5),Field::<u16>(Variant(_31, 0), 5)];
place!(Field::<i64>(Variant(_31, 0), 0)) = -_7;
_54 = _25;
_45.1 = _54;
_54 = _32;
_12 = Adt53::Variant1 { fld0: _23,fld1: 8434474159646181333_u64 };
_18 = !Field::<bool>(Variant(_12, 1), 0);
_31 = Adt47::Variant0 { fld0: _7,fld1: _38.0,fld2: _38.2,fld3: _42,fld4: _19,fld5: 64548_u16 };
Goto(bb25)
}
bb25 = {
_53 = _18;
place!(Field::<u8>(Variant(_31, 0), 2)) = (*_36) as u8;
_38.1 = core::ptr::addr_of!(_57);
_27 = _54;
_58.2.0 = _48.0;
Goto(bb26)
}
bb26 = {
_48.1.0 = _26 != _26;
_58.2.3 = _18;
_58.0.0 = (26699_u16,);
_58.1 = Adt47::Variant0 { fld0: Field::<i64>(Variant(_31, 0), 0),fld1: _33,fld2: Field::<u8>(Variant(_31, 0), 2),fld3: _42,fld4: _19,fld5: _58.0.0.0 };
place!(Field::<i128>(Variant(_31, 0), 1)) = !Field::<i128>(Variant(_58.1, 0), 1);
place!(Field::<i128>(Variant(_31, 0), 1)) = _38.0 + Field::<i128>(Variant(_58.1, 0), 1);
(*_8) = &(*_36);
place!(Field::<[i8; 4]>(Variant(_58.1, 0), 4)) = _19;
_34 = &_58.0;
_46 = Field::<i128>(Variant(_31, 0), 1) as isize;
_1 = _39 >> _5;
_58.1 = Adt47::Variant0 { fld0: Field::<i64>(Variant(_31, 0), 0),fld1: _33,fld2: Field::<u8>(Variant(_31, 0), 2),fld3: _42,fld4: Field::<[i8; 4]>(Variant(_31, 0), 4),fld5: _58.0.0.0 };
_44 = _24;
_19 = [Field::<i8>(Variant(_58.1, 0), 3),_6,Field::<i8>(Variant(_58.1, 0), 3),Field::<i8>(Variant(_31, 0), 3)];
RET = core::ptr::addr_of!(_47);
Goto(bb27)
}
bb27 = {
Call(_62 = dump_var(11_usize, 7_usize, Move(_7), 5_usize, Move(_5), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_62 = dump_var(11_usize, 21_usize, Move(_21), 54_usize, Move(_54), 9_usize, Move(_9), 19_usize, Move(_19)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_62 = dump_var(11_usize, 27_usize, Move(_27), 14_usize, Move(_14), 43_usize, Move(_43), 28_usize, Move(_28)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_62 = dump_var(11_usize, 6_usize, Move(_6), 46_usize, Move(_46), 42_usize, Move(_42), 63_usize, _63), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: f32,mut _2: bool,mut _3: [isize; 6],mut _4: [isize; 6],mut _5: isize) -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _6: f64;
let _7: (bool, (bool, u32, i32, bool), u128);
let _8: u64;
let _9: Adt66;
let _10: isize;
let _11: char;
let _12: isize;
let _13: i16;
let _14: u128;
let _15: f32;
let _16: bool;
let _17: *mut u16;
let _18: u16;
let _19: isize;
let _20: Adt47;
let _21: isize;
let _22: i128;
let _23: &'static Adt47;
let _24: char;
let _25: ();
let _26: ();
{
RET = [(-1511356866057728132_i64),(-2898485182694411256_i64),8429498242161570276_i64,4191428088235690098_i64,(-8036621947109383642_i64),4174848002011512677_i64];
_2 = false;
_3 = _4;
_3 = _4;
_3 = [_5,_5,_5,_5,_5,_5];
_3 = [_5,_5,_5,_5,_5,_5];
_6 = 2498_u16 as f64;
_6 = 1639157468_i32 as f64;
_2 = false;
_5 = 9223372036854775807_isize + (-9223372036854775808_isize);
_7.2 = !80799640229046066729629928847849173532_u128;
_7.1.1 = 1942545205_u32 * 2568867237_u32;
_7.1 = (_2, 1697402905_u32, 1673777693_i32, _2);
_3 = [_5,_5,_5,_5,_5,_5];
RET = [6605532795279236087_i64,6616846795048012003_i64,(-4179245263018996962_i64),(-4964639742251062293_i64),1925079666057395373_i64,71779138490822565_i64];
RET = [531555629845492959_i64,(-2717563651867423371_i64),(-724833515151932904_i64),(-7910632744211421642_i64),(-6460251485653780293_i64),(-1005278249011996749_i64)];
_4 = [_5,_5,_5,_5,_5,_5];
_3 = [_5,_5,_5,_5,_5,_5];
_1 = 7_usize as f32;
_7.0 = !_7.1.0;
_7.0 = _2 & _7.1.0;
Call(RET = fn13(_5, _4, _7, _7.1.2, _7.1.2, _7.1.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-8580893704565291659_i64),(-5288926819475547728_i64),(-4469444043524862554_i64),(-1772079289400705082_i64),(-7956022894912863900_i64),4884779350696077519_i64];
_5 = -(-9223372036854775808_isize);
_1 = 9_u8 as f32;
_8 = 10866757496946336028_u64 - 14257360870824435536_u64;
_7.1 = (_7.0, 925348348_u32, 1693502420_i32, _7.0);
_3 = _4;
_7.0 = _7.1.3;
_7.1.1 = !4096962988_u32;
_7.0 = _7.1.2 != _7.1.2;
_2 = _7.0 & _7.1.3;
match _7.1.2 {
0 => bb2,
1 => bb3,
2 => bb4,
1693502420 => bb6,
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
_3 = [_5,_5,_5,_5,_5,_5];
_7.0 = !_2;
_10 = _6 as isize;
_5 = !_10;
_4 = [_5,_10,_5,_5,_5,_10];
_5 = _10 | _10;
_7.1.3 = _7.0;
_7.1.2 = (-823413555_i32) << _8;
_7.1.1 = 982_i16 as u32;
_2 = _7.0;
_7.1.1 = !3027092178_u32;
_8 = 26147_u16 as u64;
_11 = '\u{a9ad0}';
_7.1 = (_7.0, 1924606646_u32, 1520888000_i32, _7.0);
_6 = _7.2 as f64;
_7.1 = (_7.0, 2204664499_u32, (-638838425_i32), _2);
_3 = [_10,_10,_5,_5,_5,_10];
_8 = 2993007885188175880_u64;
_12 = _10;
_3 = _4;
_2 = _7.1.3;
_7.0 = _2;
RET = [(-1239652532052997403_i64),8373414360879872389_i64,(-8676025614556578507_i64),(-5390774417295374067_i64),2864427388750548693_i64,6819916621936785513_i64];
_7.1.1 = !2962066213_u32;
_7.1.3 = _7.1.2 == _7.1.2;
match _7.1.2 {
0 => bb5,
1 => bb2,
2 => bb4,
3 => bb7,
340282366920938463463374607431129373031 => bb9,
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
_12 = !_10;
_8 = 3953699779416205758_u64;
_7.2 = 73219303323944289939861013609075405777_u128;
_8 = !11924307107899100270_u64;
_8 = !8705764836602435285_u64;
_6 = _8 as f64;
_4 = [_5,_5,_12,_12,_5,_5];
_6 = 164987703208273106914914583653890077615_i128 as f64;
_8 = 19448_u16 as u64;
_7.1.0 = !_7.1.3;
_7.1.2 = !(-1753464440_i32);
_5 = _10 | _12;
_14 = (-21_i8) as u128;
_3 = [_10,_10,_10,_10,_12,_10];
_15 = _1 - _1;
Call(_7.1.1 = core::intrinsics::transmute(_11), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7.2 = !_14;
_15 = _5 as f32;
_10 = _5 << _12;
_13 = 56_i8 as i16;
Goto(bb11)
}
bb11 = {
_7.1.1 = !108184237_u32;
_7.1.2 = !(-844801914_i32);
_11 = '\u{54ec7}';
_13 = 5737_i16 ^ (-20353_i16);
_5 = 22068_u16 as isize;
_16 = _7.1.0;
_13 = (-22_i8) as i16;
_10 = _5 + _12;
_14 = _7.2;
_3 = [_12,_12,_5,_10,_5,_10];
_7.2 = _14 - _14;
_7.0 = _7.1.0 ^ _16;
Call(_12 = fn15(_7.1.3, _7, _7.1.3, _10, _7.1.3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_15 = _1 + _1;
_7.1.2 = _11 as i32;
_3 = [_12,_12,_12,_12,_12,_12];
_19 = _12 & _12;
RET = [(-5517320011731186394_i64),(-6477874906909093772_i64),(-2213756172785667947_i64),1287465700532774329_i64,(-6479095115950656549_i64),5436786926740509622_i64];
_4 = [_19,_12,_19,_12,_19,_12];
RET = [8134626543289808318_i64,(-1379981816557588958_i64),4033298718424174304_i64,(-5680285524311518432_i64),(-7926770758375005933_i64),8568109420327778382_i64];
_7.1 = (_7.0, 955387002_u32, (-1420707861_i32), _16);
_2 = _7.1.0 > _7.1.0;
_12 = _7.1.2 as isize;
_19 = _12 + _12;
_6 = _15 as f64;
_19 = !_12;
_10 = _12 >> _7.1.1;
RET = [(-2673469059547803621_i64),3637515311919512940_i64,4345375526603095828_i64,5427863858560427579_i64,(-3584061163730694815_i64),(-6730851811124847208_i64)];
_7.2 = _14;
_18 = 39016_u16 | 7379_u16;
_15 = -_1;
_10 = _12 + _12;
_5 = _19;
Goto(bb13)
}
bb13 = {
_16 = _7.1.0;
_1 = _15;
Goto(bb14)
}
bb14 = {
_18 = 57768_u16;
_4 = [_5,_5,_5,_5,_19,_12];
_7.1.1 = _12 as u32;
_4 = _3;
_19 = _12;
_2 = _16;
_22 = (-133039516512324165914439076303153030445_i128);
_7.1.2 = (-982385604_i32);
_3 = [_5,_19,_19,_10,_19,_12];
_7.2 = _14 & _14;
_18 = _7.1.2 as u16;
_18 = _19 as u16;
_6 = _1 as f64;
_11 = '\u{ede35}';
_21 = _8 as isize;
_6 = _15 as f64;
_14 = _7.2 << _12;
_10 = !_5;
_14 = _7.2;
_13 = 223_u8 as i16;
_21 = _7.0 as isize;
_18 = _11 as u16;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(12_usize, 14_usize, Move(_14), 16_usize, Move(_16), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(12_usize, 18_usize, Move(_18), 3_usize, Move(_3), 21_usize, Move(_21), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: [isize; 6],mut _3: (bool, (bool, u32, i32, bool), u128),mut _4: i32,mut _5: i32,mut _6: i32) -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _7: (&'static u128,);
let _8: u32;
let _9: f64;
let _10: usize;
let _11: char;
let _12: (usize, u128);
let _13: (u16,);
let _14: (&'static u128,);
let _15: *mut [char; 2];
let _16: ();
let _17: ();
{
_3.0 = _1 < _1;
_4 = _3.1.2;
_3.1.1 = !1217633336_u32;
_3.1 = (_3.0, 170527446_u32, _6, _3.0);
RET = [(-6423111020921837350_i64),8674069234623812794_i64,(-9066488921520790972_i64),7742059724709223313_i64,6453095886204044605_i64,8579484953657182396_i64];
_2 = [_1,_1,_1,_1,_1,_1];
_1 = 4477204520243453231_i64 as isize;
_3.1.0 = _3.1.3 & _3.0;
_2 = [_1,_1,_1,_1,_1,_1];
_3.1.2 = _4 - _6;
_3.1 = (_3.0, 2883352770_u32, _6, _3.0);
_7.0 = &_3.2;
_4 = _3.1.2;
_8 = _3.1.1;
_3.1.0 = _3.0;
Goto(bb1)
}
bb1 = {
_3.1.2 = -_5;
_3.1.1 = (-17624_i16) as u32;
_6 = !_4;
_3.1.1 = _8 % _8;
_1 = 9223372036854775807_isize + 111_isize;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1673777693 => bb7,
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
_3.1.3 = !_3.0;
_3.2 = 216068692335450804094024139930418012040_u128 << _3.1.1;
_10 = 15638511632231145485_u64 as usize;
_11 = '\u{4e65b}';
_6 = _10 as i32;
_5 = _4 >> _3.1.1;
_8 = _3.1.1 - _3.1.1;
_3.2 = _3.1.1 as u128;
RET = [(-2040349184103937534_i64),3855369402173473606_i64,3046691635695769023_i64,(-8401924349705325568_i64),(-1988846374409192646_i64),2356665237619640294_i64];
_3.0 = _3.1.3 ^ _3.1.3;
_3.1.1 = (-13524_i16) as u32;
_9 = 139216025141389973119904757871769719396_i128 as f64;
_6 = !_5;
_4 = _5 >> _8;
RET = [4148159197975288472_i64,(-7096572483102971971_i64),(-689892899921769550_i64),(-7031839652996861543_i64),3978626692062305685_i64,(-5461619199419853064_i64)];
_3.1 = (_3.0, _8, _4, _3.0);
RET = [8311066527000002655_i64,4278376875116927493_i64,(-2400005817293442081_i64),(-7035378580971508746_i64),(-8326945300785396803_i64),(-4067932596928096538_i64)];
_8 = !_3.1.1;
_3.1.2 = _4;
_12 = (_10, _3.2);
RET = [(-8524504096507840133_i64),(-190622754750311078_i64),(-445454021915066091_i64),(-1313131054743764302_i64),(-2814805217825815895_i64),5705969759101547906_i64];
_3.1.0 = !_3.0;
_12 = (_10, _3.2);
_8 = _3.1.1;
RET = [(-7887199759257497223_i64),(-8598395693731138942_i64),(-4290149716809636579_i64),6185832232010234643_i64,5971217986181869108_i64,2133864056743969344_i64];
_3.1.0 = _4 < _6;
_10 = !_12.0;
Call(RET = fn14(_4, _3.1, _3, _3.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7.0 = &_3.2;
_4 = _3.1.2;
_11 = '\u{42505}';
_3.1.0 = _3.0;
_3.2 = _12.1;
_3.1 = (_3.0, _8, _5, _3.0);
_5 = _11 as i32;
_3.1.1 = _8;
_12.0 = _10;
_3.1.2 = _6;
_10 = _8 as usize;
_3.1.0 = !_3.1.3;
_11 = '\u{94cf8}';
_7.0 = &_3.2;
_3.1.1 = !_8;
_1 = !(-9223372036854775808_isize);
Goto(bb9)
}
bb9 = {
_12 = (_10, _3.2);
_3.1 = (_3.0, _8, _4, _3.0);
_3.2 = _12.1;
_3.1.2 = !_4;
_12.0 = _10 >> _4;
_7.0 = &_12.1;
_12.0 = _10 << _10;
_13 = (46285_u16,);
match _13.0 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
46285 => bb15,
_ => bb14
}
}
bb10 = {
_7.0 = &_3.2;
_4 = _3.1.2;
_11 = '\u{42505}';
_3.1.0 = _3.0;
_3.2 = _12.1;
_3.1 = (_3.0, _8, _5, _3.0);
_5 = _11 as i32;
_3.1.1 = _8;
_12.0 = _10;
_3.1.2 = _6;
_10 = _8 as usize;
_3.1.0 = !_3.1.3;
_11 = '\u{94cf8}';
_7.0 = &_3.2;
_3.1.1 = !_8;
_1 = !(-9223372036854775808_isize);
Goto(bb9)
}
bb11 = {
_3.1.3 = !_3.0;
_3.2 = 216068692335450804094024139930418012040_u128 << _3.1.1;
_10 = 15638511632231145485_u64 as usize;
_11 = '\u{4e65b}';
_6 = _10 as i32;
_5 = _4 >> _3.1.1;
_8 = _3.1.1 - _3.1.1;
_3.2 = _3.1.1 as u128;
RET = [(-2040349184103937534_i64),3855369402173473606_i64,3046691635695769023_i64,(-8401924349705325568_i64),(-1988846374409192646_i64),2356665237619640294_i64];
_3.0 = _3.1.3 ^ _3.1.3;
_3.1.1 = (-13524_i16) as u32;
_9 = 139216025141389973119904757871769719396_i128 as f64;
_6 = !_5;
_4 = _5 >> _8;
RET = [4148159197975288472_i64,(-7096572483102971971_i64),(-689892899921769550_i64),(-7031839652996861543_i64),3978626692062305685_i64,(-5461619199419853064_i64)];
_3.1 = (_3.0, _8, _4, _3.0);
RET = [8311066527000002655_i64,4278376875116927493_i64,(-2400005817293442081_i64),(-7035378580971508746_i64),(-8326945300785396803_i64),(-4067932596928096538_i64)];
_8 = !_3.1.1;
_3.1.2 = _4;
_12 = (_10, _3.2);
RET = [(-8524504096507840133_i64),(-190622754750311078_i64),(-445454021915066091_i64),(-1313131054743764302_i64),(-2814805217825815895_i64),5705969759101547906_i64];
_3.1.0 = !_3.0;
_12 = (_10, _3.2);
_8 = _3.1.1;
RET = [(-7887199759257497223_i64),(-8598395693731138942_i64),(-4290149716809636579_i64),6185832232010234643_i64,5971217986181869108_i64,2133864056743969344_i64];
_3.1.0 = _4 < _6;
_10 = !_12.0;
Call(RET = fn14(_4, _3.1, _3, _3.1), ReturnTo(bb8), UnwindUnreachable())
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
_7.0 = &_3.2;
_7.0 = &_12.1;
_2 = [_1,_1,_1,_1,_1,_1];
_3.2 = !_12.1;
_1 = 7735926645266940809_u64 as isize;
_8 = 2132499652561125451_u64 as u32;
_9 = _3.2 as f64;
_12 = (_10, _3.2);
_3.1.1 = !_8;
_12.1 = _3.2 - _3.2;
_3.1.3 = _3.0;
_2 = [_1,_1,_1,_1,_1,_1];
_13.0 = 63687_u16;
Goto(bb16)
}
bb16 = {
Call(_16 = dump_var(13_usize, 2_usize, Move(_2), 10_usize, Move(_10), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_16 = dump_var(13_usize, 6_usize, Move(_6), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i32,mut _2: (bool, u32, i32, bool),mut _3: (bool, (bool, u32, i32, bool), u128),mut _4: (bool, u32, i32, bool)) -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _5: [i16; 8];
let _6: f32;
let _7: f64;
let _8: *mut u16;
let _9: f64;
let _10: &'static u32;
let _11: isize;
let _12: (i8, bool, i64);
let _13: (i8, bool, i64);
let _14: isize;
let _15: [u16; 4];
let _16: Adt24;
let _17: f64;
let _18: *mut (usize, i16);
let _19: [char; 2];
let _20: isize;
let _21: &'static &'static u32;
let _22: (((u16,),), Adt47, (bool, u32, i32, bool));
let _23: ();
let _24: ();
{
_3.0 = !_4.0;
_5 = [(-31905_i16),(-7641_i16),17924_i16,(-11398_i16),(-12715_i16),10123_i16,(-23432_i16),(-21061_i16)];
_3 = (_2.0, _4, 274315930101033273873395381701744668560_u128);
_3.1.3 = _1 != _4.2;
_3.1 = (_4.0, _2.1, _4.2, _3.0);
RET = [5891974077700964025_i64,9126318552991081486_i64,1132639308233473016_i64,135567273843551721_i64,(-3460664722339087028_i64),(-7736870395192273901_i64)];
_3.1.1 = !_2.1;
_3.2 = 9223372036854775807_isize as u128;
_6 = 59891_u16 as f32;
_4.2 = !_2.2;
_7 = 14788944068181530865_usize as f64;
_3.1.2 = -_2.2;
Goto(bb1)
}
bb1 = {
_3.1.2 = 9223372036854775807_isize as i32;
_3.2 = !315855999652810430889839229722039637155_u128;
_4.2 = _1;
_3.2 = !34322963735320866493069772815527079917_u128;
_3 = (_4.0, _2, 244191915310557047802070099344072137966_u128);
_3.1.1 = !_4.1;
_4.0 = _3.1.0 & _2.0;
match _3.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
244191915310557047802070099344072137966 => bb10,
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
_2.1 = _3.1.1;
_7 = 155_u8 as f64;
_4.2 = _4.1 as i32;
_2.2 = _1;
_2.1 = _3.1.1;
_9 = -_7;
_6 = _4.2 as f32;
_3.0 = _4.0 <= _2.0;
_13.1 = !_3.1.0;
_5 = [32404_i16,3273_i16,10820_i16,30087_i16,(-29576_i16),20816_i16,(-19900_i16),21312_i16];
_4.0 = !_2.0;
_2.2 = _3.1.2;
_3.1.3 = _2.0 & _2.0;
_3.2 = 30330586049626503731751506123486549550_u128;
_6 = _3.1.1 as f32;
_2 = (_4.0, _3.1.1, _4.2, _3.1.0);
_3.1.3 = !_3.1.0;
_12 = (59_i8, _3.1.3, (-2550791376564909698_i64));
match _12.2 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb11,
4 => bb12,
340282366920938463460823816055203301758 => bb14,
_ => bb13
}
}
bb11 = {
Return()
}
bb12 = {
_3.1.2 = 9223372036854775807_isize as i32;
_3.2 = !315855999652810430889839229722039637155_u128;
_4.2 = _1;
_3.2 = !34322963735320866493069772815527079917_u128;
_3 = (_4.0, _2, 244191915310557047802070099344072137966_u128);
_3.1.1 = !_4.1;
_4.0 = _3.1.0 & _2.0;
match _3.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
244191915310557047802070099344072137966 => bb10,
_ => bb9
}
}
bb13 = {
Return()
}
bb14 = {
_11 = _6 as isize;
_13.1 = !_12.1;
_15 = [47828_u16,12902_u16,60780_u16,6295_u16];
_12 = ((-30_i8), _3.1.0, (-7140275331765363752_i64));
_3.1 = (_2.3, _2.1, _4.2, _4.0);
_5 = [21471_i16,(-9161_i16),6224_i16,15838_i16,(-19890_i16),(-4764_i16),17406_i16,32229_i16];
_4.3 = _3.1.3;
_3.1.1 = 18349558218381461945_u64 as u32;
_14 = !_11;
_13 = _12;
_15 = [50228_u16,16802_u16,40135_u16,18071_u16];
_17 = -_9;
_7 = -_9;
_16.fld0 = [166_u8,130_u8,126_u8,233_u8];
_3.2 = 315455866956016457929125342813857474959_u128;
_16.fld2 = 159266572291271730570107577250729592182_i128 as isize;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(14_usize, 2_usize, Move(_2), 13_usize, Move(_13), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(14_usize, 14_usize, Move(_14), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: bool,mut _2: (bool, (bool, u32, i32, bool), u128),mut _3: bool,mut _4: isize,mut _5: bool) -> isize {
mir! {
type RET = isize;
let _6: isize;
let _7: char;
let _8: *const usize;
let _9: u16;
let _10: [u16; 4];
let _11: &'static (usize, i16);
let _12: f32;
let _13: i64;
let _14: *mut Adt24;
let _15: (*const usize, [u8; 4], u16, ([u16; 8],));
let _16: isize;
let _17: Adt58;
let _18: (i64, u8, i64);
let _19: [isize; 6];
let _20: (i128, *const usize, u8);
let _21: (&'static u128,);
let _22: *const ([u16; 8],);
let _23: (((u16,),), Adt47, (bool, u32, i32, bool));
let _24: (&'static i8, ((u16,),), Adt24);
let _25: &'static (bool, (bool, u32, i32, bool), u128);
let _26: f32;
let _27: (&'static u128,);
let _28: char;
let _29: i128;
let _30: bool;
let _31: [i32; 7];
let _32: &'static (usize, i16);
let _33: isize;
let _34: ();
let _35: ();
{
_5 = !_2.1.0;
_2.1.1 = 2223520651_u32 + 2383921527_u32;
Goto(bb1)
}
bb1 = {
_5 = _3;
_6 = 4005414002935062676_i64 as isize;
_7 = '\u{41933}';
RET = _4 ^ _4;
_5 = !_1;
_2.1 = (_3, 1893204879_u32, (-783829697_i32), _2.0);
_5 = _1 != _1;
_4 = !RET;
_7 = '\u{83fa3}';
_3 = !_2.1.0;
_2.1.1 = 173648001_u32 + 675800087_u32;
Call(_2.2 = core::intrinsics::bswap(234362265180808185908019293602475836128_u128), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2.1.0 = _5;
_4 = RET ^ RET;
_2.1.1 = 3900868137_u32 * 1007066621_u32;
_6 = _4;
_2.2 = !278981746628632048696136808371763220893_u128;
_7 = '\u{4fd3b}';
_3 = !_2.1.3;
_2.1.1 = 3045866294_u32 | 2791122036_u32;
_9 = !48433_u16;
_10 = [_9,_9,_9,_9];
_2.1.2 = -(-1164774547_i32);
Goto(bb3)
}
bb3 = {
_6 = _2.2 as isize;
RET = _2.1.2 as isize;
_2.1.3 = _2.1.0 > _2.1.0;
_4 = _6;
_9 = 58752_u16 - 8896_u16;
_6 = _4;
_5 = !_1;
_9 = !37435_u16;
_2.1.3 = _1;
_5 = _2.0;
RET = 17878433506537561839_u64 as isize;
RET = 186_u8 as isize;
_2.1.3 = _2.1.0;
_9 = 12265_u16 | 38725_u16;
_2.1.0 = _3 != _3;
Goto(bb4)
}
bb4 = {
_6 = -_4;
_2.1.1 = _7 as u32;
_5 = _2.0 == _2.0;
RET = -_4;
_1 = !_3;
_9 = !33438_u16;
_4 = !_6;
_1 = _2.1.0;
_2.1.3 = _5;
_13 = 8461287006410743951_i64 * 4413656740889051517_i64;
_2.1.2 = -973855835_i32;
_2.1 = (_3, 268981577_u32, 871170478_i32, _3);
_9 = !52692_u16;
_10 = [_9,_9,_9,_9];
_15.1 = [70_u8,144_u8,178_u8,185_u8];
_12 = _2.2 as f32;
_5 = !_2.0;
_2.2 = 79786770436270583890177253122621164040_u128;
_2.0 = !_3;
_16 = -RET;
_13 = (-623525896947785505_i64);
_6 = !_4;
match _2.1.1 {
268981577 => bb5,
_ => bb3
}
}
bb5 = {
_2.0 = _1;
_2.1 = (_3, 1298920315_u32, (-357463163_i32), _1);
_15.3.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_15.3.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
Goto(bb6)
}
bb6 = {
_7 = '\u{103d60}';
_3 = !_1;
RET = -_16;
_13 = _2.1.2 as i64;
_15.2 = !_9;
RET = _16 & _4;
_16 = !_4;
_5 = _3 < _2.0;
RET = _6 << _13;
RET = _4 ^ _16;
_2.1 = (_5, 3640173074_u32, 628081334_i32, _2.0);
_1 = _2.0 ^ _5;
_16 = 20849_i16 as isize;
_15.3.0 = [_9,_15.2,_9,_9,_15.2,_9,_9,_15.2];
_15.2 = 7652560399891085187_usize as u16;
match _2.1.2 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb7,
628081334 => bb9,
_ => bb8
}
}
bb7 = {
_5 = _3;
_6 = 4005414002935062676_i64 as isize;
_7 = '\u{41933}';
RET = _4 ^ _4;
_5 = !_1;
_2.1 = (_3, 1893204879_u32, (-783829697_i32), _2.0);
_5 = _1 != _1;
_4 = !RET;
_7 = '\u{83fa3}';
_3 = !_2.1.0;
_2.1.1 = 173648001_u32 + 675800087_u32;
Call(_2.2 = core::intrinsics::bswap(234362265180808185908019293602475836128_u128), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_6 = _2.2 as isize;
RET = _2.1.2 as isize;
_2.1.3 = _2.1.0 > _2.1.0;
_4 = _6;
_9 = 58752_u16 - 8896_u16;
_6 = _4;
_5 = !_1;
_9 = !37435_u16;
_2.1.3 = _1;
_5 = _2.0;
RET = 17878433506537561839_u64 as isize;
RET = 186_u8 as isize;
_2.1.3 = _2.1.0;
_9 = 12265_u16 | 38725_u16;
_2.1.0 = _3 != _3;
Goto(bb4)
}
bb9 = {
_16 = !_4;
_18.1 = (-123_i8) as u8;
_4 = RET;
_18.0 = _2.1.3 as i64;
_18.1 = !46_u8;
_19 = [RET,_4,_4,_16,RET,RET];
_9 = _15.2 | _15.2;
_5 = !_2.0;
_18.2 = _18.0;
RET = -_6;
_12 = _18.1 as f32;
_18.1 = 238_u8 >> _13;
_2.1.2 = 1559941754_i32;
_3 = !_5;
_12 = (-26812_i16) as f32;
_2.1.1 = !4084455858_u32;
_15.3.0 = [_9,_9,_15.2,_15.2,_15.2,_9,_9,_15.2];
match _2.2 {
0 => bb1,
1 => bb7,
2 => bb3,
79786770436270583890177253122621164040 => bb10,
_ => bb6
}
}
bb10 = {
_5 = !_2.1.0;
_9 = _6 as u16;
_2.1.2 = (-913665150_i32) + 564706303_i32;
_9 = _15.2 - _15.2;
_7 = '\u{6d653}';
_21.0 = &_2.2;
_9 = !_15.2;
_1 = _2.1.3;
RET = 18_i8 as isize;
_9 = _18.0 as u16;
_13 = _6 as i64;
_18 = (_13, 186_u8, _13);
_7 = '\u{94236}';
_2.1 = (_5, 3613458159_u32, (-330843519_i32), _1);
_2.2 = 296118253641760360342508563457059598030_u128;
_3 = _2.1.0;
_2.2 = _2.1.2 as u128;
_18.2 = _2.0 as i64;
_13 = -_18.2;
_16 = _18.2 as isize;
_23.2.3 = !_2.1.0;
_21.0 = &_2.2;
_2.1.0 = _5 | _1;
_24.2.fld1 = 7_usize << _9;
Goto(bb11)
}
bb11 = {
_24.2.fld1 = !2_usize;
_22 = core::ptr::addr_of!(_15.3);
_24.2 = Adt24 { fld0: _15.1,fld1: 1_usize,fld2: RET,fld3: (-64_i8),fld4: (-16183_i16) };
_15.1 = [_18.1,_18.1,_18.1,_18.1];
_15.1 = [_18.1,_18.1,_18.1,_18.1];
_25 = &_2;
_1 = _2.0;
_10 = [_9,_9,_9,_9];
_24.2 = Adt24 { fld0: _15.1,fld1: 0_usize,fld2: _16,fld3: (-128_i8),fld4: 11765_i16 };
_23.2.0 = _5 != (*_25).1.3;
_22 = core::ptr::addr_of!((*_22));
_7 = '\u{29679}';
_20.0 = !(-86743643371856176314083719623356403802_i128);
_18.2 = _13;
_24.0 = &_24.2.fld3;
_23.2.0 = _24.2.fld2 == _24.2.fld2;
_2.1.2 = 974473390_i32;
_20.2 = _18.1 % _18.1;
Goto(bb12)
}
bb12 = {
_23.0.0 = (_9,);
_14 = core::ptr::addr_of_mut!(_24.2);
_2.1 = (_2.0, 3271460134_u32, 654282567_i32, _5);
(*_14).fld1 = 4_usize;
_5 = _24.2.fld2 < _24.2.fld2;
_20.1 = core::ptr::addr_of!((*_14).fld1);
_20.0 = 104148389657201956627651945532672670488_i128 * 17689290389635108761723733885777173240_i128;
_20.0 = 114316442625400776354889537927916938187_i128;
_20.1 = core::ptr::addr_of!(_24.2.fld1);
_19 = [(*_14).fld2,(*_14).fld2,(*_14).fld2,(*_14).fld2,_16,_16];
_24.1.0 = (_9,);
_23.2.2 = (*_25).2 as i32;
_2.1 = (_2.0, 3985838441_u32, _23.2.2, _3);
_2.0 = _13 >= _13;
_31 = [_23.2.2,_23.2.2,_2.1.2,_2.1.2,_23.2.2,_2.1.2,_23.2.2];
_23.2.1 = !_2.1.1;
_30 = !_5;
_27.0 = &(*_25).2;
RET = (*_14).fld2;
_16 = (*_14).fld2 | RET;
_10 = [_9,_23.0.0.0,_24.1.0.0,_23.0.0.0];
(*_14).fld1 = _9 as usize;
_24.1.0 = (_23.0.0.0,);
RET = (*_14).fld2;
Goto(bb13)
}
bb13 = {
Call(_34 = dump_var(15_usize, 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_34 = dump_var(15_usize, 9_usize, Move(_9), 18_usize, Move(_18), 19_usize, Move(_19), 35_usize, _35), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *const &'static u128,mut _2: bool,mut _3: u128,mut _4: i8,mut _5: i8,mut _6: f32,mut _7: bool) -> f32 {
mir! {
type RET = f32;
let _8: &'static [u16; 4];
let _9: f64;
let _10: [i64; 6];
let _11: [i64; 6];
let _12: ();
let _13: ();
{
_4 = (-9223372036854775808_isize) as i8;
RET = _6;
_6 = -RET;
RET = _6 + _6;
_5 = _4;
_7 = _6 != _6;
RET = _6 - _6;
_3 = !178341534011750954441023739975278519456_u128;
_9 = _3 as f64;
_3 = 251092571301708498086573488578348317203_u128;
_5 = _4;
_4 = _5;
_10 = [(-3813359269667161381_i64),(-4496181327661716871_i64),(-6972123101497206146_i64),6354428374017956732_i64,(-9026389488783062073_i64),2936898349566091837_i64];
_4 = (-41542223765143747544825353736696662193_i128) as i8;
_6 = RET;
_5 = _4;
_9 = 2027336155_i32 as f64;
RET = -_6;
_11 = [8018978219119630903_i64,3235754157166704588_i64,364652171209654158_i64,1991936218640678711_i64,4657665661702258934_i64,(-2605939012356710647_i64)];
RET = -_6;
_7 = _2;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(16_usize, 11_usize, Move(_11), 10_usize, Move(_10), 3_usize, Move(_3), 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *const &'static u128,mut _2: &'static u128,mut _3: bool,mut _4: isize,mut _5: f32,mut _6: i128,mut _7: [i64; 6],mut _8: i128) -> i16 {
mir! {
type RET = i16;
let _9: u32;
let _10: u16;
let _11: Adt58;
let _12: isize;
let _13: i16;
let _14: Adt53;
let _15: i16;
let _16: &'static [i32; 7];
let _17: f32;
let _18: &'static (i64, u8, i64);
let _19: u16;
let _20: isize;
let _21: isize;
let _22: i128;
let _23: i8;
let _24: f64;
let _25: ();
let _26: ();
{
_7 = [3788182679440507406_i64,(-5538184987542946688_i64),(-3741523746407244413_i64),9118635528313973108_i64,7008102725579719689_i64,(-3656503275936094095_i64)];
RET = (-27025_i16);
RET = !32696_i16;
_5 = 26283_u16 as f32;
RET = -10397_i16;
RET = 16624_i16 + (-18093_i16);
_1 = core::ptr::addr_of!(_2);
_3 = !false;
_7 = [(-1979600653354641197_i64),3120901159647573496_i64,5179348142868280758_i64,(-2526439589554801342_i64),(-2241427858301397527_i64),(-7179272704572856250_i64)];
_7 = [(-4485956338283408771_i64),(-936725824104634205_i64),(-3737905140152946701_i64),5790417316852499000_i64,4849049611930660749_i64,4734904440213677844_i64];
_10 = 43578_u16;
_9 = !170458994_u32;
_6 = !_8;
RET = !31230_i16;
Goto(bb1)
}
bb1 = {
_4 = (-117_isize) ^ 63_isize;
_4 = !(-9223372036854775808_isize);
RET = (-27964_i16) - (-27493_i16);
_4 = 29_u8 as isize;
RET = 9329_i16;
_8 = 229788615724135267427250050354931976197_u128 as i128;
_1 = core::ptr::addr_of!(_2);
_10 = 63158_u16 - 49476_u16;
_8 = _6 << _6;
RET = _4 as i16;
_1 = core::ptr::addr_of!((*_1));
_10 = 2776_u16 & 60681_u16;
_12 = !_4;
_4 = _12 + _12;
RET = (-31392_i16);
_7 = [(-5189749448000657980_i64),(-5139264745070513183_i64),(-3688053499090352140_i64),(-3920589922727452783_i64),1295061430524824152_i64,(-6196494448199678722_i64)];
_9 = 1077619689_u32;
_13 = 47_i8 as i16;
_4 = _12;
_10 = 10364_u16;
_3 = !true;
_15 = _13;
_3 = _6 < _8;
_9 = _4 as u32;
match _10 {
10364 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_4 = _12 + _12;
RET = _15;
_1 = core::ptr::addr_of!((*_1));
_7 = [(-5668033000090991270_i64),6190241776352014382_i64,(-3488053450466762129_i64),(-5183898858817488574_i64),462325851916994447_i64,3480136993501462387_i64];
match _10 {
0 => bb1,
1 => bb2,
2 => bb4,
10364 => bb6,
_ => bb5
}
}
bb4 = {
Return()
}
bb5 = {
_4 = (-117_isize) ^ 63_isize;
_4 = !(-9223372036854775808_isize);
RET = (-27964_i16) - (-27493_i16);
_4 = 29_u8 as isize;
RET = 9329_i16;
_8 = 229788615724135267427250050354931976197_u128 as i128;
_1 = core::ptr::addr_of!(_2);
_10 = 63158_u16 - 49476_u16;
_8 = _6 << _6;
RET = _4 as i16;
_1 = core::ptr::addr_of!((*_1));
_10 = 2776_u16 & 60681_u16;
_12 = !_4;
_4 = _12 + _12;
RET = (-31392_i16);
_7 = [(-5189749448000657980_i64),(-5139264745070513183_i64),(-3688053499090352140_i64),(-3920589922727452783_i64),1295061430524824152_i64,(-6196494448199678722_i64)];
_9 = 1077619689_u32;
_13 = 47_i8 as i16;
_4 = _12;
_10 = 10364_u16;
_3 = !true;
_15 = _13;
_3 = _6 < _8;
_9 = _4 as u32;
match _10 {
10364 => bb3,
_ => bb2
}
}
bb6 = {
_7 = [6840349671703563932_i64,(-958194233308555863_i64),8149405677273018697_i64,3455853090894631657_i64,(-6885574464479308342_i64),7493156693574894724_i64];
RET = !_13;
RET = '\u{67353}' as i16;
_15 = RET | _13;
_7 = [(-5908141425994135962_i64),(-5231908002667860118_i64),(-719229240956170534_i64),(-7008284896747127525_i64),(-769228991187566202_i64),769826911363206748_i64];
_7 = [5272052002817237056_i64,6169120082394715164_i64,(-5791081299708928603_i64),(-6414426502242720807_i64),5737147358363225504_i64,8486097138043506380_i64];
_4 = 10942441521110184890_u64 as isize;
RET = -_15;
_15 = _13;
_12 = _4;
RET = _13;
_14 = Adt53::Variant0 { fld0: 3_usize };
place!(Field::<usize>(Variant(_14, 0), 0)) = 10310979523678834788_u64 as usize;
_8 = _6;
RET = 270838655766542805421256012939605486566_u128 as i16;
_15 = !_13;
RET = !_13;
_15 = _13;
_9 = !2934555395_u32;
_19 = (-5571745780940342731_i64) as u16;
_1 = core::ptr::addr_of!(_2);
Goto(bb7)
}
bb7 = {
RET = -_15;
_20 = Field::<usize>(Variant(_14, 0), 0) as isize;
RET = _15 << _6;
_4 = _20 + _12;
_22 = _19 as i128;
_5 = _9 as f32;
Goto(bb8)
}
bb8 = {
Call(_25 = dump_var(17_usize, 8_usize, Move(_8), 9_usize, Move(_9), 20_usize, Move(_20), 12_usize, Move(_12)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_25 = dump_var(17_usize, 10_usize, Move(_10), 19_usize, Move(_19), 26_usize, _26, 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u8,mut _2: i32,mut _3: i32,mut _4: u32,mut _5: (i64, u8, i64),mut _6: (bool, u32, i32, bool)) -> f64 {
mir! {
type RET = f64;
let _7: Adt30;
let _8: *mut (i8, (i64, u8, i64), char, i128);
let _9: f64;
let _10: i16;
let _11: *mut (i8, (i64, u8, i64), char, i128);
let _12: Adt63;
let _13: i8;
let _14: (Adt47, &'static u32, &'static u128, u64);
let _15: &'static ((u16,),);
let _16: &'static &'static u128;
let _17: &'static (((u16,),), Adt47, (bool, u32, i32, bool));
let _18: [u8; 4];
let _19: ();
let _20: ();
{
_5.0 = _5.2 >> _6.2;
_5.2 = _5.0 - _5.0;
_5.0 = !_5.2;
_6 = (true, _4, _2, true);
_6 = (true, _4, _2, true);
_6.2 = _5.0 as i32;
_5.0 = _5.2 | _5.2;
_4 = _6.1 & _6.1;
_6.0 = _6.3;
_5.2 = _5.0;
_5.0 = _5.2 | _5.2;
_1 = _5.1;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1191865546 => bb7,
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
_5.1 = _1 << _5.0;
_9 = _5.0 as f64;
_6.3 = _6.0 ^ _6.0;
_5 = ((-2112637931379780837_i64), _1, (-5907086496520063389_i64));
_4 = '\u{108a1d}' as u32;
_6.2 = -_3;
_5.0 = _5.2 - _5.2;
RET = _9;
_14.1 = &_4;
_10 = !(-30565_i16);
RET = _9;
_2 = -_6.2;
Goto(bb8)
}
bb8 = {
Call(_19 = dump_var(18_usize, 1_usize, Move(_1), 10_usize, Move(_10), 2_usize, Move(_2), 20_usize, _20), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{b5312}'), std::hint::black_box((-35_isize)), std::hint::black_box((-112_i8)), std::hint::black_box((-3600_i16)), std::hint::black_box((-56860535_i32)), std::hint::black_box(5029286841906672050_i64), std::hint::black_box(202492067843960827721022456895443223735_u128), std::hint::black_box(0_usize), std::hint::black_box(5416798183547343483_u64), std::hint::black_box(60342_u16), std::hint::black_box(1732733392_u32));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt24{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt24 {
fld0: [u8; 4],
fld1: usize,
fld2: isize,
fld3: i8,
fld4: i16,
}
impl PrintFDebug for Adt30{
	unsafe fn printf_debug(&self){unsafe{printf("Adt30::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt30 {
Variant0{
fld0: (bool, (bool, u32, i32, bool), u128),
fld1: char,
fld2: isize,
fld3: [u8; 4],

},
Variant1{
fld0: [u16; 8],
fld1: u8,
fld2: u16,
fld3: i8,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: u8,
fld2: [u8; 4],
fld3: Adt24,
fld4: u16,
fld5: (bool, (bool, u32, i32, bool), u128),
fld6: usize,

},
Variant1{
fld0: bool,
fld1: (bool, (bool, u32, i32, bool), u128),
fld2: isize,
fld3: (i8, (i64, u8, i64), char, i128),

}}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: i64,
fld1: i128,
fld2: u8,
fld3: i8,
fld4: [i8; 4],
fld5: u16,

},
Variant1{
fld0: u128,
fld1: i128,
fld2: f32,
fld3: u32,
fld4: (i64, u8, i64),
fld5: Adt30,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: usize,

},
Variant1{
fld0: bool,
fld1: u64,

},
Variant2{
fld0: bool,
fld1: i128,
fld2: i32,
fld3: (i8, bool, i64),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [u8; 4],
fld1: ((u16,),),
fld2: Adt47,
fld3: [i32; 7],
fld4: u8,
fld5: [i32; 5],

},
Variant1{
fld0: Adt42,
fld1: u128,
fld2: (bool, (bool, u32, i32, bool), u128),
fld3: [char; 2],

}}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: u64,
fld1: u128,
fld2: Adt24,
fld3: i8,
fld4: [i8; 4],
fld5: u8,
fld6: [usize; 2],
fld7: [i64; 6],

},
Variant1{
fld0: (usize, i16),
fld1: u8,
fld2: (bool, (bool, u32, i32, bool), u128),
fld3: *const usize,
fld4: [u8; 4],
fld5: [i32; 5],

},
Variant2{
fld0: char,

}}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){unsafe{printf("Adt66::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt66 {
Variant0{
fld0: (i8, (i64, u8, i64), char, i128),
fld1: i64,
fld2: f32,
fld3: (i128, *const usize, u8),
fld4: *const usize,
fld5: [i8; 4],

},
Variant1{
fld0: u32,
fld1: (i8, bool, i64),
fld2: u16,
fld3: ((u16,),),
fld4: [i64; 6],
fld5: [char; 2],
fld6: [i8; 4],

},
Variant2{
fld0: *mut (i8, (i64, u8, i64), char, i128),
fld1: u16,
fld2: (i8, bool, i64),
fld3: usize,

},
Variant3{
fld0: *mut [char; 2],
fld1: u32,

}}

