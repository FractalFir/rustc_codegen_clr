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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> [char; 8] {
mir! {
type RET = [char; 8];
let _15: Adt42;
let _16: Adt45;
let _17: *mut f32;
let _18: [bool; 5];
let _19: f32;
let _20: (u64,);
let _21: Adt45;
let _22: char;
let _23: u32;
let _24: isize;
let _25: bool;
let _26: ([char; 8],);
let _27: f32;
let _28: char;
let _29: [u8; 6];
let _30: isize;
let _31: i64;
let _32: bool;
let _33: u16;
let _34: [u8; 6];
let _35: (u8, i16, u16);
let _36: Adt54;
let _37: [char; 7];
let _38: Adt57;
let _39: u8;
let _40: isize;
let _41: isize;
let _42: ([char; 8],);
let _43: Adt45;
let _44: Adt46;
let _45: [u128; 5];
let _46: i32;
let _47: bool;
let _48: i8;
let _49: Adt53;
let _50: isize;
let _51: [char; 7];
let _52: Adt51;
let _53: ();
let _54: ();
{
_5 = (-26414_i16) ^ (-20428_i16);
_4 = 90_i8 | 95_i8;
_4 = 5515710628546748657_i64 as i8;
_7 = (-4246905075160333035_i64) | (-6921725652375945747_i64);
Goto(bb1)
}
bb1 = {
_8 = (-114448949878229443548894889100321848348_i128);
_9 = 1664890897_u32 as usize;
RET = ['\u{87a00}','\u{83dcf}','\u{31919}','\u{86f3e}','\u{3722e}','\u{a8709}','\u{44747}','\u{ff98a}'];
_5 = (-20546_i16);
_2 = '\u{ee3bc}';
_12 = 33051256_u32 - 189157555_u32;
_6 = _5 as i32;
_8 = 41290080451541943124126968543033631801_i128 | 80612741599719072543479101745897805530_i128;
_6 = _5 as i32;
_5 = (-663_i16);
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
Goto(bb2)
}
bb2 = {
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_10 = 247_u8 >> _9;
_1 = true;
_8 = 125718315250086506446847901641298179768_i128;
_6 = -(-1600184562_i32);
_4 = !61_i8;
_9 = (-9223372036854775808_isize) as usize;
_2 = '\u{384ff}';
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_16.fld1 = 34723_u16;
_14 = 223687688017214770498210445980190290441_u128 | 42486648645359213656967671000711811142_u128;
_9 = !6524384932717123921_usize;
_18 = [_1,_1,_1,_1,_1];
_6 = _9 as i32;
_12 = 992370421_u32;
_12 = 4121480480_u32;
_16.fld1 = 31640_u16 ^ 62448_u16;
_19 = _7 as f32;
_2 = '\u{42032}';
_16 = Adt45 { fld0: (-9223372036854775808_isize),fld1: 39596_u16 };
_11 = _16.fld1 * _16.fld1;
_18 = [_1,_1,_1,_1,_1];
_13 = _19 as u64;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_8 = !(-61102748544153034080314711268517478485_i128);
Call(_10 = fn1(_12, _16.fld1, _11, _16.fld0, _6, _5, _16.fld0, _19, _11, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = 3563078918413018779_i64;
_1 = false;
_3 = _5 as isize;
_8 = (-83592922854279368743720397325911100840_i128) ^ 88686646613938111762959935376722043948_i128;
_14 = !275617611424950781574037874621768244681_u128;
_16 = Adt45 { fld0: _3,fld1: _11 };
_14 = 270093284867888495686095312875564645729_u128 & 62858120532850082937904857588295083718_u128;
_11 = _16.fld1 | _16.fld1;
_17 = core::ptr::addr_of_mut!(_19);
_2 = '\u{a85a6}';
_12 = !1680824831_u32;
_10 = 150_u8 & 163_u8;
_7 = -(-5928399271507203616_i64);
_5 = _2 as i16;
_3 = _14 as isize;
Call(_4 = fn2(_8, _11, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_21.fld1 = _5 as u16;
_21 = Adt45 { fld0: _16.fld0,fld1: _11 };
_16.fld0 = _3 + _3;
_1 = true;
_1 = false;
_22 = _2;
Goto(bb5)
}
bb5 = {
_13 = _5 as u64;
_23 = _12 & _12;
RET = [_22,_2,_2,_22,_22,_2,_22,_2];
_9 = 1_usize >> _21.fld1;
_7 = !(-858977971926566832_i64);
_12 = !_23;
_11 = _5 as u16;
_21.fld0 = _16.fld0 + _3;
_11 = _16.fld1 & _21.fld1;
_18 = [_1,_1,_1,_1,_1];
_23 = _2 as u32;
_26 = (RET,);
_21.fld1 = _11 | _11;
RET = _26.0;
_3 = _13 as isize;
_6 = (-1986723916_i32) << _21.fld1;
_2 = _22;
_1 = !false;
RET = [_2,_2,_22,_2,_22,_2,_22,_22];
_21 = _16;
Call(_16.fld1 = core::intrinsics::bswap(_21.fld1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = Adt45 { fld0: _21.fld0,fld1: _11 };
_16.fld0 = !_21.fld0;
_14 = 18429790263507477112245539792960255944_u128;
(*_17) = _13 as f32;
_23 = _10 as u32;
_27 = -(*_17);
_13 = 55696099048007714_u64 ^ 12105047629227241425_u64;
_27 = _8 as f32;
_1 = !true;
_25 = _11 >= _11;
_17 = core::ptr::addr_of_mut!((*_17));
Goto(bb7)
}
bb7 = {
_8 = !149366726014784552398121904860700204417_i128;
_14 = 279481033332018552128700325825520837605_u128;
Goto(bb8)
}
bb8 = {
_28 = _2;
_2 = _28;
_21.fld1 = _16.fld1 & _16.fld1;
_16.fld1 = _4 as u16;
_13 = !7690601642929354941_u64;
_25 = _1 ^ _1;
_21 = _16;
_22 = _2;
_10 = _5 as u8;
_20.0 = !_13;
_7 = _25 as i64;
_29 = [_10,_10,_10,_10,_10,_10];
_29 = [_10,_10,_10,_10,_10,_10];
_16 = Adt45 { fld0: _21.fld0,fld1: _11 };
_14 = _20.0 as u128;
_3 = -_21.fld0;
_3 = !_16.fld0;
_16.fld1 = _11;
_26 = (RET,);
_27 = (*_17);
_24 = _3 + _21.fld0;
(*_17) = _4 as f32;
Goto(bb9)
}
bb9 = {
RET = _26.0;
RET = _26.0;
(*_17) = _27;
_28 = _22;
_26 = (RET,);
_28 = _2;
_28 = _22;
_30 = _24;
_18 = [_25,_25,_1,_25,_1];
_21 = Adt45 { fld0: _16.fld0,fld1: _11 };
_26 = (RET,);
_12 = _23 ^ _23;
_18 = [_1,_25,_25,_25,_25];
Goto(bb10)
}
bb10 = {
_14 = _10 as u128;
_10 = 71_u8 + 71_u8;
_2 = _22;
_1 = _25;
_11 = _16.fld1 ^ _16.fld1;
RET = _26.0;
(*_17) = -_27;
_18 = [_25,_25,_25,_1,_25];
_33 = _11 & _11;
_5 = _8 as i16;
_35.1 = _5;
_32 = !_25;
Goto(bb11)
}
bb11 = {
_16 = Adt45 { fld0: _21.fld0,fld1: _11 };
_29 = [_10,_10,_10,_10,_10,_10];
_33 = _16.fld1 >> _21.fld1;
_18 = [_1,_32,_32,_32,_1];
_2 = _22;
_21.fld1 = _33 - _33;
_31 = -_7;
RET = [_2,_28,_22,_28,_28,_28,_28,_22];
Goto(bb12)
}
bb12 = {
_7 = _31 * _31;
_35.2 = !_33;
_14 = !125712202747922443873311241946127688750_u128;
_34 = [_10,_10,_10,_10,_10,_10];
_22 = _2;
_24 = _16.fld0;
_18 = [_32,_25,_1,_25,_25];
_34 = _29;
_35.2 = !_21.fld1;
_39 = _10 >> _35.2;
_22 = _2;
_18 = [_25,_1,_1,_32,_25];
_21 = _16;
_37 = [_2,_28,_2,_22,_2,_28,_2];
_2 = _22;
_17 = core::ptr::addr_of_mut!((*_17));
_26 = (RET,);
_4 = -57_i8;
_1 = _39 <= _39;
_14 = _30 as u128;
_25 = !_1;
_9 = 3_usize * 4_usize;
_21.fld1 = _6 as u16;
_10 = _39;
Goto(bb13)
}
bb13 = {
_30 = _16.fld0 << _39;
_35 = (_10, _5, _33);
_4 = _5 as i8;
_42 = _26;
_26 = (_42.0,);
_42.0 = [_28,_22,_22,_28,_28,_28,_28,_2];
(*_17) = _27 + _27;
_21.fld0 = !_30;
_43.fld0 = !_21.fld0;
_16.fld1 = !_21.fld1;
_43 = Adt45 { fld0: _21.fld0,fld1: _11 };
_17 = core::ptr::addr_of_mut!((*_17));
_1 = _25;
_46 = _14 as i32;
_39 = _10;
_35 = (_10, _5, _33);
_16.fld0 = -_21.fld0;
_26 = (RET,);
_31 = _7;
_35 = (_10, _5, _16.fld1);
_35 = (_39, _5, _16.fld1);
Goto(bb14)
}
bb14 = {
_29 = [_39,_10,_39,_39,_10,_35.0];
_16.fld1 = !_33;
_17 = core::ptr::addr_of_mut!(_19);
RET = _26.0;
RET = [_2,_2,_2,_22,_2,_2,_2,_28];
_15 = Adt42::Variant0 { fld0: _25 };
_16 = _43;
_47 = !Field::<bool>(Variant(_15, 0), 0);
_6 = _46;
(*_17) = _27 + _27;
_26 = (_42.0,);
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(0_usize, 23_usize, Move(_23), 22_usize, Move(_22), 28_usize, Move(_28), 47_usize, Move(_47)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(0_usize, 12_usize, Move(_12), 46_usize, Move(_46), 29_usize, Move(_29), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(0_usize, 4_usize, Move(_4), 39_usize, Move(_39), 18_usize, Move(_18), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(0_usize, 34_usize, Move(_34), 10_usize, Move(_10), 33_usize, Move(_33), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(0_usize, 24_usize, Move(_24), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u32,mut _2: u16,mut _3: u16,mut _4: isize,mut _5: i32,mut _6: i16,mut _7: isize,mut _8: f32,mut _9: u16,mut _10: i8) -> u8 {
mir! {
type RET = u8;
let _11: [u8; 6];
let _12: u8;
let _13: i128;
let _14: isize;
let _15: f64;
let _16: ();
let _17: ();
{
_10 = (-123_i8) & 110_i8;
_6 = _4 as i16;
_4 = _3 as isize;
_2 = _3 + _3;
Call(RET = core::intrinsics::bswap(47_u8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = -(-1366005379_i32);
_11 = [182_u8,222_u8,86_u8,176_u8,25_u8,20_u8];
_1 = 12617280979738804559_u64 as u32;
_1 = _4 as u32;
_11 = [144_u8,110_u8,34_u8,0_u8,45_u8,56_u8];
RET = !240_u8;
_4 = _7 >> _5;
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463454151235394913435648 => bb6,
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
_8 = _1 as f32;
_12 = !RET;
RET = _12;
_6 = (-2985_i16);
RET = !_12;
_1 = 3550625615_u32 | 3855839167_u32;
_8 = _10 as f32;
RET = _12;
_5 = 1435212178_i32 >> _2;
_10 = (-56_i8) >> _5;
_2 = !_3;
RET = _12;
_13 = (-143676325886745966958261014692263206738_i128) ^ 80932175060431450547965583816366501778_i128;
_9 = _3 << _1;
_1 = !1972553607_u32;
RET = !_12;
_13 = 18412765208438215929_u64 as i128;
_10 = !(-29_i8);
_1 = _13 as u32;
match _6 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
340282366920938463463374607431768208471 => bb12,
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
Return()
}
bb10 = {
Return()
}
bb11 = {
_5 = -(-1366005379_i32);
_11 = [182_u8,222_u8,86_u8,176_u8,25_u8,20_u8];
_1 = 12617280979738804559_u64 as u32;
_1 = _4 as u32;
_11 = [144_u8,110_u8,34_u8,0_u8,45_u8,56_u8];
RET = !240_u8;
_4 = _7 >> _5;
match _7 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb12 = {
_13 = -126405878975633670267133772157288945545_i128;
RET = _8 as u8;
_6 = (-13369_i16);
_3 = _9 * _9;
_12 = !RET;
RET = _12;
_12 = RET >> _2;
_4 = '\u{7fc21}' as isize;
_7 = _12 as isize;
RET = _12;
_13 = _1 as i128;
_12 = RET;
RET = _12 - _12;
_4 = _7;
_9 = _3;
_7 = -_4;
_14 = _3 as isize;
_12 = !RET;
_11 = [RET,_12,RET,_12,RET,RET];
RET = _12 ^ _12;
_15 = 14064178750613666595_u64 as f64;
_12 = 5403938619505682364_i64 as u8;
_2 = _9 >> _9;
_15 = 303246622985769073503214889878174640026_u128 as f64;
Goto(bb13)
}
bb13 = {
Call(_16 = dump_var(1_usize, 6_usize, Move(_6), 7_usize, Move(_7), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_16 = dump_var(1_usize, 14_usize, Move(_14), 10_usize, Move(_10), 17_usize, _17, 17_usize, _17), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: u16,mut _3: i16) -> i8 {
mir! {
type RET = i8;
let _4: &'static [u8; 6];
let _5: i8;
let _6: Adt52;
let _7: (u32, char, char);
let _8: isize;
let _9: bool;
let _10: isize;
let _11: isize;
let _12: u8;
let _13: &'static [u8; 6];
let _14: u8;
let _15: isize;
let _16: [u128; 1];
let _17: i128;
let _18: Adt41;
let _19: char;
let _20: *const i16;
let _21: isize;
let _22: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16));
let _23: Adt52;
let _24: u16;
let _25: char;
let _26: Adt41;
let _27: f32;
let _28: u8;
let _29: u16;
let _30: ();
let _31: ();
{
RET = 120_i8 & (-76_i8);
RET = !4_i8;
RET = _3 as i8;
RET = true as i8;
_2 = 37276_u16;
_2 = !27310_u16;
_3 = (-31840_i16);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768179616 => bb7,
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
RET = !103_i8;
Goto(bb8)
}
bb8 = {
_7.1 = '\u{721c6}';
Call(_7.0 = fn3(_7.1, _2, _3, _1, _1, RET, _3, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7.2 = _7.1;
_7 = (4109283446_u32, '\u{dedff}', '\u{23cf3}');
_7 = (3883558710_u32, '\u{65543}', '\u{1093b2}');
match _7.0 {
0 => bb8,
1 => bb2,
3883558710 => bb10,
_ => bb3
}
}
bb10 = {
_5 = (-2380076862254992764_i64) as i8;
RET = false as i8;
_7.1 = _7.2;
RET = 266091615780684846033152082354673748339_u128 as i8;
_5 = -RET;
_3 = (-238_i16);
_3 = (-29233_i16) * (-29975_i16);
_7 = (615151591_u32, '\u{46f42}', '\u{f8445}');
_7.1 = _7.2;
RET = _5 ^ _5;
_5 = RET * RET;
_9 = true;
_2 = _9 as u16;
_8 = -(-9223372036854775808_isize);
_5 = 323892233503206709169216055460494437885_u128 as i8;
_9 = _7.0 <= _7.0;
_7.2 = _7.1;
_11 = 1101586442_i32 as isize;
_14 = !135_u8;
_8 = _11;
_7 = (952765696_u32, '\u{ed07a}', '\u{193e9}');
_11 = _8;
_7 = (3617466624_u32, '\u{bd020}', '\u{fe6d}');
_8 = -_11;
_7.1 = _7.2;
match _7.0 {
0 => bb5,
1 => bb11,
3617466624 => bb13,
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
_9 = false;
_2 = 50929_u16 - 44105_u16;
_10 = _8 - _11;
_7.1 = _7.2;
_5 = _2 as i8;
_16 = [215899334296169057066535948369727405026_u128];
_15 = _10 + _10;
RET = _5 & _5;
_9 = RET == RET;
_2 = 47722_u16;
_10 = _15;
RET = _5;
_1 = (-25460729910003867314759427294746502085_i128);
_18 = Adt41::Variant2 { fld0: 15307065237201284700_u64 };
_7 = (1945496104_u32, '\u{9e259}', '\u{9ebf7}');
_18 = Adt41::Variant2 { fld0: 16109282035730047749_u64 };
RET = _5;
RET = _5;
_8 = _1 as isize;
_7 = (2825046552_u32, '\u{34d56}', '\u{8535d}');
_7 = (1471238589_u32, '\u{622da}', '\u{b3646}');
_20 = core::ptr::addr_of!(_3);
_7.1 = _7.2;
Goto(bb14)
}
bb14 = {
_15 = _10;
(*_20) = !(-18018_i16);
_11 = _15 ^ _10;
_18 = Adt41::Variant2 { fld0: 17911757403977704791_u64 };
place!(Field::<u64>(Variant(_18, 2), 0)) = 2583377188622894356_u64;
_9 = true;
RET = _5;
_7.1 = _7.2;
_22.4 = (_7.0, _7.2, _7.1);
_12 = _1 as u8;
_15 = 488121238_i32 as isize;
_22.4.1 = _7.1;
_21 = !_10;
_7.2 = _7.1;
_22.0 = [_7.1,_22.4.2,_7.1,_7.1,_22.4.1];
_7.0 = _22.4.0 | _22.4.0;
_24 = _2 / _2;
_5 = !RET;
SetDiscriminant(_18, 0);
_27 = 8460284901972489138_u64 as f32;
_7.0 = !_22.4.0;
_25 = _7.1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(2_usize, 9_usize, Move(_9), 1_usize, Move(_1), 25_usize, Move(_25), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(2_usize, 12_usize, Move(_12), 10_usize, Move(_10), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: u16,mut _3: i16,mut _4: i128,mut _5: i128,mut _6: i8,mut _7: i16,mut _8: i128) -> u32 {
mir! {
type RET = u32;
let _9: Adt51;
let _10: Adt55;
let _11: i128;
let _12: i16;
let _13: u16;
let _14: [char; 8];
let _15: &'static [u8; 6];
let _16: *mut f32;
let _17: isize;
let _18: i128;
let _19: i64;
let _20: isize;
let _21: u64;
let _22: Adt42;
let _23: u32;
let _24: (u8, i16, u16);
let _25: ([char; 8],);
let _26: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16));
let _27: (bool,);
let _28: [bool; 5];
let _29: Adt43;
let _30: ([char; 8],);
let _31: f64;
let _32: bool;
let _33: ();
let _34: ();
{
RET = !719869163_u32;
_5 = _4;
_2 = 56834133903204017607020545177283894863_u128 as u16;
_3 = _7;
_3 = 12870888474431757099_usize as i16;
_3 = !_7;
_7 = _2 as i16;
_7 = _3 - _3;
_2 = _6 as u16;
_4 = _8;
_1 = '\u{6bb03}';
_3 = true as i16;
_1 = '\u{4db1f}';
_10.fld0 = core::ptr::addr_of!(_7);
_10.fld0 = core::ptr::addr_of!(_3);
_5 = _2 as i128;
RET = 2325952562_u32 << _2;
_10.fld0 = core::ptr::addr_of!(_12);
_8 = _5;
_10.fld0 = core::ptr::addr_of!(_7);
_11 = -_5;
_1 = '\u{13849}';
RET = !1327156474_u32;
_3 = _7 << _4;
_4 = !_11;
Call(_1 = fn4(_7, _3, _7, _6, _7, _3, _10.fld0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = -_3;
_11 = _4;
RET = !3245633973_u32;
_12 = !_7;
RET = 3697361126_u32 << _5;
RET = 3599597730_u32 * 2045829967_u32;
_3 = _7;
_12 = _7 * _3;
_13 = _3 as u16;
_4 = !_11;
_11 = -_4;
_2 = !_13;
Goto(bb2)
}
bb2 = {
_5 = _4;
_7 = _12;
_8 = 251070895901293558961836419384940130783_u128 as i128;
_18 = _8;
_10.fld0 = core::ptr::addr_of!(_7);
_2 = _1 as u16;
_14 = [_1,_1,_1,_1,_1,_1,_1,_1];
_8 = _11;
_1 = '\u{9d16b}';
_17 = (-1918384684_i32) as isize;
_14 = [_1,_1,_1,_1,_1,_1,_1,_1];
_1 = '\u{92224}';
_19 = 6199034804081960224_i64;
_5 = _11 << _6;
_4 = _13 as i128;
Call(_20 = core::intrinsics::bswap(_17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _11;
_23 = !RET;
_10.fld0 = core::ptr::addr_of!(_12);
_17 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
RET = _23 & _23;
_10.fld0 = core::ptr::addr_of!(_7);
_24 = (200_u8, _7, _13);
_24.2 = _17 as u16;
_10.fld0 = core::ptr::addr_of!(_3);
_26.4.2 = _1;
_16 = core::ptr::addr_of_mut!(_26.3);
_6 = 4030253349968422931_usize as i8;
_24 = (106_u8, _7, _13);
_8 = _6 as i128;
match _24.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
106 => bb9,
_ => bb8
}
}
bb4 = {
_5 = _4;
_7 = _12;
_8 = 251070895901293558961836419384940130783_u128 as i128;
_18 = _8;
_10.fld0 = core::ptr::addr_of!(_7);
_2 = _1 as u16;
_14 = [_1,_1,_1,_1,_1,_1,_1,_1];
_8 = _11;
_1 = '\u{9d16b}';
_17 = (-1918384684_i32) as isize;
_14 = [_1,_1,_1,_1,_1,_1,_1,_1];
_1 = '\u{92224}';
_19 = 6199034804081960224_i64;
_5 = _11 << _6;
_4 = _13 as i128;
Call(_20 = core::intrinsics::bswap(_17), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7 = -_3;
_11 = _4;
RET = !3245633973_u32;
_12 = !_7;
RET = 3697361126_u32 << _5;
RET = 3599597730_u32 * 2045829967_u32;
_3 = _7;
_12 = _7 * _3;
_13 = _3 as u16;
_4 = !_11;
_11 = -_4;
_2 = !_13;
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
_26.5 = core::ptr::addr_of!(_24);
_1 = _26.4.2;
_12 = -_24.1;
_4 = 274997968960747488208071988358117894265_u128 as i128;
_25 = (_14,);
_27 = (false,);
(*_16) = _24.2 as f32;
_4 = _5 * _11;
Goto(bb10)
}
bb10 = {
_26.4.1 = _26.4.2;
(*_16) = _11 as f32;
_25.0 = [_26.4.2,_1,_26.4.2,_26.4.2,_26.4.2,_1,_1,_26.4.2];
RET = _23 & _23;
_26.4.0 = _23 << _24.0;
_23 = _26.4.0;
_26.4.1 = _1;
_11 = _4;
_4 = 3641072440222172241_u64 as i128;
_12 = _24.1;
_4 = -_8;
_26.0 = [_26.4.2,_1,_26.4.1,_1,_1];
Call(_8 = core::intrinsics::bswap(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = _12 - _24.1;
_25 = (_14,);
_8 = _11;
_20 = !_17;
_19 = 2174875315920185337_i64 | (-2560266850863605548_i64);
(*_16) = _19 as f32;
_19 = -(-1694333537132881011_i64);
RET = _26.4.0;
_25.0 = _14;
_31 = _17 as f64;
_10.fld0 = core::ptr::addr_of!(_7);
_16 = core::ptr::addr_of_mut!((*_16));
_13 = 50542786548692033979563438423281780372_u128 as u16;
_6 = 60_i8;
_26.0 = [_26.4.2,_1,_26.4.2,_26.4.2,_26.4.2];
_30.0 = [_26.4.1,_1,_1,_26.4.2,_1,_1,_26.4.2,_26.4.1];
_25.0 = _14;
_30.0 = [_26.4.2,_26.4.2,_26.4.2,_26.4.1,_1,_1,_1,_26.4.1];
RET = _23 | _26.4.0;
(*_16) = RET as f32;
_26.4 = (_23, _1, _1);
_10.fld0 = core::ptr::addr_of!(_12);
Goto(bb12)
}
bb12 = {
Call(_33 = dump_var(3_usize, 1_usize, Move(_1), 11_usize, Move(_11), 25_usize, Move(_25), 20_usize, Move(_20)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_33 = dump_var(3_usize, 2_usize, Move(_2), 7_usize, Move(_7), 8_usize, Move(_8), 3_usize, Move(_3)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_33 = dump_var(3_usize, 24_usize, Move(_24), 27_usize, Move(_27), 34_usize, _34, 34_usize, _34), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i8,mut _5: i16,mut _6: i16,mut _7: *const i16) -> char {
mir! {
type RET = char;
let _8: f32;
let _9: u128;
let _10: Adt45;
let _11: isize;
let _12: f64;
let _13: [u8; 6];
let _14: isize;
let _15: [char; 5];
let _16: u64;
let _17: char;
let _18: *const (u8, i16, u16);
let _19: f64;
let _20: isize;
let _21: Adt42;
let _22: char;
let _23: *const u8;
let _24: [u128; 1];
let _25: [char; 8];
let _26: ([char; 8],);
let _27: ();
let _28: ();
{
RET = '\u{460f2}';
_1 = false as i16;
_6 = (*_7) << _3;
_7 = core::ptr::addr_of!(_5);
(*_7) = RET as i16;
_1 = _2 >> _2;
_1 = false as i16;
_4 = 84_i8;
RET = '\u{1677a}';
_1 = 1035230644_u32 as i16;
(*_7) = -_2;
(*_7) = -_6;
_3 = !_6;
_6 = -(*_7);
_6 = _2 | _5;
_3 = _2;
RET = '\u{b2215}';
_8 = _4 as f32;
(*_7) = _6;
_9 = !171207338083801149469616238082022416937_u128;
_7 = core::ptr::addr_of!(_2);
_3 = _5;
_1 = !(*_7);
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
84 => bb7,
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
_3 = (*_7) >> _6;
_8 = (-2287648674967284052_i64) as f32;
_6 = -_1;
RET = '\u{526df}';
RET = '\u{84807}';
(*_7) = 43120_u16 as i16;
_7 = core::ptr::addr_of!(_1);
_3 = (*_7) ^ _5;
(*_7) = (-9223372036854775808_isize) as i16;
_2 = _5;
_6 = _5;
_4 = !25_i8;
_9 = _8 as u128;
(*_7) = _8 as i16;
_9 = !144169065106167691808268291755919659177_u128;
RET = '\u{16ce0}';
_9 = _4 as u128;
_7 = core::ptr::addr_of!(_1);
_10.fld1 = _5 as u16;
_1 = false as i16;
_5 = 15587891396567982188_u64 as i16;
_4 = 23_i8 * (-76_i8);
Call(_6 = fn5(_4, RET, _10.fld1, _3, _10.fld1, _2, _3, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_7) = -_6;
(*_7) = _6 | _6;
_11 = (-2766378205627831026_i64) as isize;
_10.fld0 = _11;
_11 = _10.fld0;
_4 = 2_i8;
_15 = [RET,RET,RET,RET,RET];
_5 = -_6;
_1 = _5 >> _6;
Call(_8 = fn7(_5, (*_7), _6, _7, _5, _7, _1, _6, _7, _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10.fld0 = _11;
Call(_6 = fn8(_3, _1, _8, _5, (*_7), _7, (*_7), (*_7), _8, _5, _8), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = -(*_7);
_14 = -_10.fld0;
_17 = RET;
_16 = !4363009357444147429_u64;
RET = _17;
_4 = !67_i8;
_3 = _6;
_3 = !(*_7);
RET = _17;
_3 = _5;
(*_7) = !_2;
_12 = _16 as f64;
_9 = 181451421178724648765427362133787396759_u128;
Call(_13 = fn9(_6, _10.fld0, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20 = _11;
_9 = 257569704240641326450744267137751686162_u128 + 123708901576653543047917364460693151847_u128;
_1 = !_6;
_19 = _12;
_10.fld1 = _4 as u16;
Goto(bb12)
}
bb12 = {
_3 = _4 as i16;
_20 = _11 >> _6;
_5 = _10.fld1 as i16;
_10.fld0 = _9 as isize;
_13 = [163_u8,234_u8,246_u8,192_u8,115_u8,206_u8];
(*_7) = -_6;
_17 = RET;
_8 = _16 as f32;
_10.fld1 = 18302_u16;
_16 = !4247145693984931408_u64;
_21 = Adt42::Variant0 { fld0: true };
_3 = (*_7) >> _6;
_4 = 150_u8 as i8;
_2 = -_1;
_8 = (-146712884999724012498727400990683717660_i128) as f32;
_1 = _3 | _3;
_9 = 78914180259417172834758672275799199261_u128 << _20;
_19 = 138682145_u32 as f64;
_2 = true as i16;
Call(_23 = fn19(_7, (*_7), _1, _7, _3, _7, (*_7)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_7) = _3;
_20 = _9 as isize;
_6 = _4 as i16;
_17 = RET;
_24 = [_9];
_3 = !(*_7);
RET = _17;
_25 = [RET,_17,_17,_17,RET,RET,RET,RET];
_3 = !_1;
_19 = _12 - _12;
RET = _17;
Goto(bb14)
}
bb14 = {
_24 = [_9];
(*_7) = _3;
_5 = -_1;
_26.0 = [_17,_17,_17,RET,RET,RET,RET,_17];
RET = _17;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(4_usize, 24_usize, Move(_24), 25_usize, Move(_25), 17_usize, Move(_17), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(4_usize, 11_usize, Move(_11), 20_usize, Move(_20), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i8,mut _2: char,mut _3: u16,mut _4: i16,mut _5: u16,mut _6: i16,mut _7: i16,mut _8: i16) -> i16 {
mir! {
type RET = i16;
let _9: f32;
let _10: *mut i64;
let _11: char;
let _12: [char; 7];
let _13: Adt45;
let _14: ();
let _15: ();
{
_5 = _3 ^ _3;
_5 = (-68878818166693459521203926412451102374_i128) as u16;
Goto(bb1)
}
bb1 = {
RET = (-21710902119549431041697614687008335443_i128) as i16;
_8 = _4;
_6 = 3869439579668181162_usize as i16;
_3 = _5 - _5;
_2 = '\u{6ff61}';
_8 = _7;
_9 = 37498144984313508314804264899036512089_i128 as f32;
_1 = (-113_i8) >> _4;
_6 = _8;
_9 = 8569059275999823853_usize as f32;
_1 = -115_i8;
RET = !_6;
_1 = (-92_i8);
RET = -_7;
_3 = _5;
_11 = _2;
RET = _6 << _7;
_2 = _11;
_4 = RET;
RET = _3 as i16;
Call(RET = fn6(_5, _6, _4, _8, _7, _8, _7, _6, _8, _6, _6, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _3;
_1 = -(-114_i8);
_12 = [_11,_11,_2,_11,_11,_2,_11];
_5 = !_3;
_13.fld1 = _5;
_4 = RET - _7;
_9 = (-7886899556547239228_i64) as f32;
_13.fld0 = _3 as isize;
RET = _6 >> _4;
Goto(bb3)
}
bb3 = {
Call(_14 = dump_var(5_usize, 2_usize, Move(_2), 1_usize, Move(_1), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_14 = dump_var(5_usize, 12_usize, Move(_12), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: i16,mut _8: i16,mut _9: i16,mut _10: i16,mut _11: i16,mut _12: i16) -> i16 {
mir! {
type RET = i16;
let _13: isize;
let _14: ();
let _15: ();
{
_6 = !_12;
_9 = _4;
RET = _5 | _4;
RET = _10 << _2;
_3 = -_12;
_9 = 1504227307_i32 as i16;
_1 = 271451521004281138858976971984833543224_u128 as u16;
_6 = _3 - _2;
_11 = RET;
_9 = RET ^ _8;
_3 = !_11;
_3 = _8 * _9;
RET = -_9;
_9 = -_6;
_5 = 4010084006439426671_i64 as i16;
_1 = 64901_u16 * 6763_u16;
_11 = _3 ^ _12;
RET = _11 & _3;
_12 = RET ^ _2;
_8 = _6;
_1 = 87675235430430793943756589664019455094_u128 as u16;
_1 = !16815_u16;
_6 = _3;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(6_usize, 6_usize, Move(_6), 10_usize, Move(_10), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(6_usize, 3_usize, Move(_3), 12_usize, Move(_12), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: *const i16,mut _5: i16,mut _6: *const i16,mut _7: i16,mut _8: i16,mut _9: *const i16,mut _10: Adt45) -> f32 {
mir! {
type RET = f32;
let _11: *const i32;
let _12: usize;
let _13: isize;
let _14: ();
let _15: ();
{
_9 = core::ptr::addr_of!((*_4));
_3 = -_1;
(*_4) = _5 ^ _3;
(*_4) = 9826945820182354864_u64 as i16;
_8 = _1;
(*_6) = _7;
_1 = _3 + (*_4);
_12 = 30463713820983589293991418612537435160_u128 as usize;
_9 = core::ptr::addr_of!((*_4));
_10 = Adt45 { fld0: 9223372036854775807_isize,fld1: 25523_u16 };
_2 = !(*_4);
(*_4) = _2 & _7;
(*_9) = _8;
_6 = core::ptr::addr_of!(_8);
(*_6) = -_1;
RET = _12 as f32;
_6 = _4;
RET = (*_4) as f32;
(*_4) = 7332933474981997851137512401244984919_u128 as i16;
_10 = Adt45 { fld0: (-81_isize),fld1: 31383_u16 };
_12 = 4_usize + 3_usize;
(*_9) = _7;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(7_usize, 12_usize, Move(_12), 5_usize, Move(_5), 7_usize, Move(_7), 15_usize, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i16,mut _2: i16,mut _3: f32,mut _4: i16,mut _5: i16,mut _6: *const i16,mut _7: i16,mut _8: i16,mut _9: f32,mut _10: i16,mut _11: f32) -> i16 {
mir! {
type RET = i16;
let _12: i128;
let _13: isize;
let _14: Adt46;
let _15: f64;
let _16: (u8, i16, u16);
let _17: i64;
let _18: (u64,);
let _19: *mut [char; 7];
let _20: ([char; 8],);
let _21: isize;
let _22: ();
let _23: ();
{
_10 = 10089154174933950769_u64 as i16;
_3 = 61898_u16 as f32;
_11 = _9 + _9;
RET = 16290_u16 as i16;
_11 = -_9;
_7 = 1114677642_u32 as i16;
(*_6) = 113693507100926050705478905000792134288_i128 as i16;
(*_6) = _2;
_13 = 124_isize * 118_isize;
_3 = -_9;
(*_6) = 2087349087_u32 as i16;
Goto(bb1)
}
bb1 = {
_16.1 = _8 + _5;
_17 = !(-8000305617619101259_i64);
_15 = _17 as f64;
_16.2 = !9910_u16;
(*_6) = -_5;
_18 = (6089544503135981764_u64,);
_15 = _17 as f64;
_18.0 = !512342793086588539_u64;
_2 = !(*_6);
_11 = _9 * _3;
_1 = (*_6);
RET = !_8;
_12 = -19096818207075741117056021288603280650_i128;
(*_6) = _5;
_2 = -_4;
_5 = _1 >> _4;
_18 = (1703992417376836910_u64,);
_12 = -(-51536112552076588919709854050925374103_i128);
_20.0 = ['\u{b7e6e}','\u{945e7}','\u{a36d4}','\u{79e3e}','\u{69cce}','\u{4aaeb}','\u{cdcdc}','\u{e4c43}'];
_6 = core::ptr::addr_of!((*_6));
_16 = (124_u8, (*_6), 14498_u16);
_4 = _16.1 + _1;
_18.0 = 9402591451311359189_u64 ^ 4735556919144935575_u64;
_21 = _16.2 as isize;
_11 = _9 * _3;
_5 = RET;
_10 = (*_6);
_3 = _21 as f32;
Goto(bb2)
}
bb2 = {
Call(_22 = dump_var(8_usize, 8_usize, Move(_8), 2_usize, Move(_2), 16_usize, Move(_16), 13_usize, Move(_13)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_22 = dump_var(8_usize, 18_usize, Move(_18), 10_usize, Move(_10), 12_usize, Move(_12), 23_usize, _23), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i16,mut _2: isize,mut _3: Adt45) -> [u8; 6] {
mir! {
type RET = [u8; 6];
let _4: u16;
let _5: (bool,);
let _6: [bool; 5];
let _7: [u8; 6];
let _8: f64;
let _9: char;
let _10: Adt45;
let _11: *mut i64;
let _12: *mut i64;
let _13: (u64,);
let _14: Adt54;
let _15: f32;
let _16: [char; 8];
let _17: u64;
let _18: u32;
let _19: i64;
let _20: Adt43;
let _21: (bool,);
let _22: bool;
let _23: [char; 8];
let _24: i16;
let _25: isize;
let _26: Adt45;
let _27: isize;
let _28: [bool; 5];
let _29: [char; 5];
let _30: i16;
let _31: i8;
let _32: bool;
let _33: *const i16;
let _34: f64;
let _35: char;
let _36: Adt56;
let _37: &'static [u8; 6];
let _38: Adt48;
let _39: f64;
let _40: i8;
let _41: ();
let _42: ();
{
RET = [29_u8,199_u8,37_u8,91_u8,30_u8,96_u8];
RET = [185_u8,58_u8,105_u8,22_u8,201_u8,241_u8];
RET = [90_u8,12_u8,138_u8,188_u8,106_u8,67_u8];
_3 = Adt45 { fld0: _2,fld1: 32620_u16 };
RET = [230_u8,97_u8,45_u8,179_u8,150_u8,69_u8];
_3 = Adt45 { fld0: _2,fld1: 43702_u16 };
_3.fld1 = 1449050396302481351_i64 as u16;
_2 = !_3.fld0;
_3 = Adt45 { fld0: _2,fld1: 20845_u16 };
_3 = Adt45 { fld0: _2,fld1: 56643_u16 };
_2 = _3.fld0;
_1 = -(-19312_i16);
RET = [252_u8,55_u8,11_u8,170_u8,88_u8,38_u8];
_2 = 76_i8 as isize;
_3.fld0 = _2 * _2;
_1 = 15204_i16;
RET = [252_u8,100_u8,145_u8,206_u8,150_u8,3_u8];
RET = [220_u8,169_u8,55_u8,157_u8,65_u8,68_u8];
_1 = 15649_i16 ^ (-18257_i16);
_1 = !20878_i16;
_3 = Adt45 { fld0: _2,fld1: 17239_u16 };
_2 = _3.fld0;
_4 = _3.fld1 % _3.fld1;
RET = [219_u8,26_u8,175_u8,95_u8,167_u8,153_u8];
RET = [3_u8,174_u8,158_u8,139_u8,203_u8,192_u8];
match _3.fld1 {
17239 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_3 = Adt45 { fld0: _2,fld1: _4 };
_2 = !_3.fld0;
_4 = _3.fld1 >> _1;
_3.fld1 = 60_u8 as u16;
_1 = -(-25149_i16);
RET = [92_u8,37_u8,234_u8,7_u8,95_u8,73_u8];
_4 = _3.fld1;
_3.fld0 = _2 << _2;
RET = [166_u8,87_u8,97_u8,146_u8,109_u8,136_u8];
_4 = 3512360289500053988_usize as u16;
_3.fld1 = _4;
_1 = 30776_i16 & (-23004_i16);
Goto(bb3)
}
bb3 = {
_5.0 = true ^ false;
_4 = (-49_i8) as u16;
_4 = 242060714990786430016129800703517186118_u128 as u16;
_3 = Adt45 { fld0: _2,fld1: _4 };
_6 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_7 = [163_u8,126_u8,188_u8,27_u8,106_u8,70_u8];
_5.0 = !true;
_3.fld1 = _4;
_3 = Adt45 { fld0: _2,fld1: _4 };
Call(_1 = core::intrinsics::bswap(8141_i16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = [_5.0,_5.0,_5.0,_5.0,_5.0];
RET = _7;
_3.fld1 = _2 as u16;
_8 = _4 as f64;
_5 = (false,);
_3.fld0 = _2;
RET = [173_u8,14_u8,215_u8,173_u8,5_u8,125_u8];
_3.fld1 = _4 ^ _4;
_3.fld0 = !_2;
_3.fld0 = 7456282434096002484_i64 as isize;
_2 = _3.fld0 * _3.fld0;
_5 = (false,);
_3 = Adt45 { fld0: _2,fld1: _4 };
_10 = Adt45 { fld0: _2,fld1: _4 };
_10.fld0 = _3.fld0 & _3.fld0;
_10.fld0 = _3.fld0;
_3.fld1 = !_4;
_9 = '\u{6dea5}';
_10 = Adt45 { fld0: _2,fld1: _3.fld1 };
_8 = _3.fld1 as f64;
_3.fld1 = !_10.fld1;
_6 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_10.fld1 = _4 - _3.fld1;
Goto(bb5)
}
bb5 = {
_3.fld0 = _10.fld0 << _10.fld0;
_3.fld0 = _2 - _2;
_10.fld1 = !_4;
RET = [28_u8,103_u8,227_u8,11_u8,13_u8,240_u8];
RET = [62_u8,40_u8,78_u8,55_u8,209_u8,147_u8];
_6 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_3 = _10;
_8 = 6385207402170904213_u64 as f64;
_2 = _10.fld0;
_10.fld1 = 2071339649_i32 as u16;
RET = [232_u8,218_u8,200_u8,90_u8,44_u8,97_u8];
RET = [244_u8,28_u8,96_u8,115_u8,155_u8,86_u8];
_10.fld0 = _3.fld0;
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_13 = (1241734804631523708_u64,);
_15 = _3.fld1 as f32;
Call(_15 = fn10(_1, _10.fld0, _5.0, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10.fld1 = _3.fld1 * _4;
_2 = _8 as isize;
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_9 = '\u{f7fad}';
_2 = -_3.fld0;
_5 = (false,);
_10.fld0 = _10.fld1 as isize;
_17 = _13.0 + _13.0;
_10.fld0 = _15 as isize;
_10.fld0 = _3.fld0;
_2 = 41015951864726236684049957768695362811_u128 as isize;
_1 = 112208457_u32 as i16;
_8 = (-51_i8) as f64;
_4 = _10.fld1 + _10.fld1;
_5 = (true,);
_17 = _13.0;
_3 = _10;
_17 = _13.0 * _13.0;
_18 = !2550945548_u32;
_11 = core::ptr::addr_of_mut!(_19);
(*_11) = !(-4769536660648858715_i64);
Goto(bb7)
}
bb7 = {
_9 = '\u{ddcb8}';
_11 = core::ptr::addr_of_mut!((*_11));
_10.fld0 = 174514884056349667368430727629851493187_u128 as isize;
RET = _7;
_12 = core::ptr::addr_of_mut!(_19);
_10 = Adt45 { fld0: _2,fld1: _4 };
_22 = _17 != _17;
_21.0 = _5.0 >= _5.0;
RET = [251_u8,162_u8,206_u8,31_u8,232_u8,13_u8];
_21.0 = _22;
_4 = _18 as u16;
_5.0 = _21.0 <= _21.0;
_15 = 249425520654476192973943014091175070337_u128 as f32;
match _13.0 {
1241734804631523708 => bb9,
_ => bb8
}
}
bb8 = {
_3.fld0 = _10.fld0 << _10.fld0;
_3.fld0 = _2 - _2;
_10.fld1 = !_4;
RET = [28_u8,103_u8,227_u8,11_u8,13_u8,240_u8];
RET = [62_u8,40_u8,78_u8,55_u8,209_u8,147_u8];
_6 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_3 = _10;
_8 = 6385207402170904213_u64 as f64;
_2 = _10.fld0;
_10.fld1 = 2071339649_i32 as u16;
RET = [232_u8,218_u8,200_u8,90_u8,44_u8,97_u8];
RET = [244_u8,28_u8,96_u8,115_u8,155_u8,86_u8];
_10.fld0 = _3.fld0;
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_13 = (1241734804631523708_u64,);
_15 = _3.fld1 as f32;
Call(_15 = fn10(_1, _10.fld0, _5.0, _7), ReturnTo(bb6), UnwindUnreachable())
}
bb9 = {
_21 = (_5.0,);
_13.0 = !_17;
_5.0 = !_21.0;
_11 = _12;
_10.fld0 = _3.fld0 ^ _3.fld0;
_25 = _10.fld0;
_11 = _12;
_17 = !_13.0;
_7 = [59_u8,125_u8,145_u8,182_u8,210_u8,120_u8];
RET = _7;
_26 = _3;
_13 = (_17,);
(*_11) = (-927216988850597345_i64);
_23 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10 = Adt45 { fld0: _25,fld1: _26.fld1 };
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10.fld0 = _26.fld0 - _26.fld0;
_1 = 12224_i16 >> _3.fld1;
(*_12) = 5038007261890987436_i64 + 4021396012261743346_i64;
_21 = _5;
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_19 = -4990208457705716932_i64;
_2 = _10.fld0 << (*_12);
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_13.0 = _17;
_3.fld0 = _2;
_27 = _9 as isize;
(*_11) = !6749556419406319536_i64;
Goto(bb10)
}
bb10 = {
_2 = !_3.fld0;
_28 = [_22,_21.0,_21.0,_5.0,_21.0];
_10.fld0 = _2 << _4;
_28 = [_21.0,_21.0,_5.0,_5.0,_22];
_13.0 = _17 + _17;
_5 = (_21.0,);
_8 = (-152920808457633259071775656956105906971_i128) as f64;
_3.fld1 = !_10.fld1;
_7 = RET;
_7 = [127_u8,198_u8,100_u8,222_u8,92_u8,116_u8];
_16 = _23;
_16 = _23;
(*_11) = (-3727964045444004038_i64) * 5441011115187036149_i64;
(*_11) = 3049745158108377605_i64 - (-4959437259548613203_i64);
_21 = _5;
_8 = (-87638691_i32) as f64;
_12 = _11;
(*_12) = (-68_i8) as i64;
_5.0 = _21.0;
Call(_7 = fn18(_13.0, _25, _28, _28, _21.0, _10.fld0, _10, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_24 = _1 >> _10.fld1;
RET = [165_u8,57_u8,206_u8,96_u8,202_u8,127_u8];
_1 = _18 as i16;
(*_11) = (-8767215878644956410_i64);
_10.fld0 = _2;
_10.fld0 = 140666579455958155255323217329303817104_i128 as isize;
_10.fld1 = _26.fld1;
(*_11) = (-8109469697591245157_i64) | 4639612485632273637_i64;
_16 = [_9,_9,_9,_9,_9,_9,_9,_9];
_3.fld1 = !_4;
Goto(bb12)
}
bb12 = {
_5.0 = !_21.0;
_24 = 80_u8 as i16;
_29 = [_9,_9,_9,_9,_9];
_26.fld1 = (*_12) as u16;
_26.fld0 = _15 as isize;
_31 = 5_i8 * 127_i8;
_10.fld1 = !_26.fld1;
(*_11) = (-893664674534944738_i64) ^ 9066936564139030318_i64;
_35 = _9;
_26.fld0 = (-1459326896_i32) as isize;
RET = [17_u8,119_u8,203_u8,100_u8,200_u8,122_u8];
_33 = core::ptr::addr_of!(_24);
_16 = _23;
_31 = -32_i8;
_36.fld2 = 178938675851473481951931299023962885402_u128 as u32;
Goto(bb13)
}
bb13 = {
_2 = _3.fld0;
_12 = core::ptr::addr_of_mut!((*_12));
_30 = -(*_33);
_10.fld1 = _3.fld1 | _26.fld1;
_26.fld0 = 2004729596_i32 as isize;
_11 = _12;
Goto(bb14)
}
bb14 = {
_10.fld1 = _13.0 as u16;
(*_11) = 3473077448706326501_i64;
_36.fld2 = _2 as u32;
_12 = _11;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(9_usize, 13_usize, Move(_13), 35_usize, Move(_35), 21_usize, Move(_21), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(9_usize, 16_usize, Move(_16), 2_usize, Move(_2), 18_usize, Move(_18), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(9_usize, 19_usize, Move(_19), 6_usize, Move(_6), 4_usize, Move(_4), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: i16,mut _2: isize,mut _3: bool,mut _4: [u8; 6]) -> f32 {
mir! {
type RET = f32;
let _5: Adt41;
let _6: i128;
let _7: [char; 7];
let _8: char;
let _9: i128;
let _10: *const i16;
let _11: [char; 7];
let _12: [char; 7];
let _13: u128;
let _14: *const (u8, i16, u16);
let _15: *const i32;
let _16: isize;
let _17: u16;
let _18: f64;
let _19: (u32, char, char);
let _20: [u128; 5];
let _21: Adt57;
let _22: Adt45;
let _23: bool;
let _24: Adt44;
let _25: *const u8;
let _26: char;
let _27: f64;
let _28: f32;
let _29: Adt47;
let _30: i32;
let _31: Adt46;
let _32: ();
let _33: ();
{
RET = (-41962435805593609059380311488236677943_i128) as f32;
_1 = 28262_i16 - 3501_i16;
_1 = 29729_i16;
_1 = 245212580675847186174458422368129227872_u128 as i16;
_3 = false | true;
_2 = RET as isize;
RET = 87_i8 as f32;
RET = _2 as f32;
_3 = !true;
_3 = !true;
_5 = Adt41::Variant2 { fld0: 7705058506277912179_u64 };
_5 = Adt41::Variant2 { fld0: 7279140036167460255_u64 };
_1 = -(-7896_i16);
RET = 59_u8 as f32;
Goto(bb1)
}
bb1 = {
_5 = Adt41::Variant2 { fld0: 6420104777739406881_u64 };
_1 = '\u{71a00}' as i16;
_6 = (-876763468_i32) as i128;
_2 = 14543110580619095729_u64 as isize;
_7 = ['\u{7356e}','\u{65003}','\u{d341c}','\u{d6ee4}','\u{ccb82}','\u{6ba44}','\u{4441c}'];
_3 = !true;
_1 = 9501_i16 << _6;
_10 = core::ptr::addr_of!(_1);
_9 = -_6;
_8 = '\u{bf974}';
_9 = -_6;
place!(Field::<u64>(Variant(_5, 2), 0)) = 240_u8 as u64;
_6 = _9;
RET = Field::<u64>(Variant(_5, 2), 0) as f32;
_9 = _6;
_5 = Adt41::Variant1 { fld0: 51709_u16,fld1: 642426318_u32 };
_7 = [_8,_8,_8,_8,_8,_8,_8];
place!(Field::<u32>(Variant(_5, 1), 1)) = 3416364964_u32;
(*_10) = (-9264_i16);
_3 = !true;
_6 = -_9;
_11 = [_8,_8,_8,_8,_8,_8,_8];
place!(Field::<u16>(Variant(_5, 1), 0)) = !9754_u16;
match (*_10) {
0 => bb2,
1 => bb3,
340282366920938463463374607431768202192 => bb5,
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
_2 = 9223372036854775807_isize;
place!(Field::<u16>(Variant(_5, 1), 0)) = 62397_u16 & 6941_u16;
_11 = _7;
_8 = '\u{fe794}';
_12 = [_8,_8,_8,_8,_8,_8,_8];
_8 = '\u{25581}';
(*_10) = 8215060932118453143_i64 as i16;
(*_10) = (-82_i8) as i16;
_4 = [179_u8,95_u8,185_u8,22_u8,237_u8,40_u8];
_8 = '\u{f21c4}';
_13 = RET as u128;
place!(Field::<u16>(Variant(_5, 1), 0)) = !37939_u16;
_11 = _7;
SetDiscriminant(_5, 1);
_8 = '\u{f588e}';
_10 = core::ptr::addr_of!(_1);
_1 = (-20549_i16) | (-27635_i16);
_8 = '\u{6020}';
_2 = (-9223372036854775808_isize);
_3 = false;
_13 = 95995577827951095858398357985567434322_u128 >> (*_10);
Goto(bb6)
}
bb6 = {
RET = 5_usize as f32;
_9 = _6;
place!(Field::<u16>(Variant(_5, 1), 0)) = !34012_u16;
_3 = true & false;
(*_10) = 28262_i16 - (-7009_i16);
_4 = [190_u8,82_u8,109_u8,209_u8,246_u8,136_u8];
_11 = _7;
place!(Field::<u32>(Variant(_5, 1), 1)) = 2029599832_u32;
_1 = -2074_i16;
_7 = [_8,_8,_8,_8,_8,_8,_8];
_13 = !279970575633377181742810712525444895289_u128;
RET = _13 as f32;
_7 = [_8,_8,_8,_8,_8,_8,_8];
_10 = core::ptr::addr_of!((*_10));
_3 = false & false;
Goto(bb7)
}
bb7 = {
_3 = (*_10) > (*_10);
_10 = core::ptr::addr_of!((*_10));
_9 = -_6;
_7 = [_8,_8,_8,_8,_8,_8,_8];
_7 = [_8,_8,_8,_8,_8,_8,_8];
_12 = [_8,_8,_8,_8,_8,_8,_8];
SetDiscriminant(_5, 1);
_1 = 18482_i16 | 10492_i16;
_1 = (-5182_i16);
_2 = -(-9223372036854775808_isize);
_5 = Adt41::Variant2 { fld0: 15088123472347300941_u64 };
_13 = 287175934452125333507536105744240933268_u128;
_8 = '\u{d2b61}';
_3 = _2 < _2;
_2 = (-9223372036854775808_isize);
_2 = 5_usize as isize;
_5 = Adt41::Variant2 { fld0: 4338128358369818495_u64 };
_16 = _2 & _2;
place!(Field::<u64>(Variant(_5, 2), 0)) = !8238247429780974282_u64;
Goto(bb8)
}
bb8 = {
_6 = _9;
_11 = _7;
_17 = 25814_u16 | 13595_u16;
_10 = core::ptr::addr_of!((*_10));
place!(Field::<u64>(Variant(_5, 2), 0)) = 10687113408200110538_u64 ^ 6794171745292560024_u64;
SetDiscriminant(_5, 2);
place!(Field::<u64>(Variant(_5, 2), 0)) = _13 as u64;
_1 = _3 as i16;
_1 = !22170_i16;
_3 = !true;
_17 = !55933_u16;
_2 = _16;
_9 = -_6;
_19.2 = _8;
(*_10) = !(-18086_i16);
Call(_19.0 = core::intrinsics::transmute(_19.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16 = !_2;
_17 = !25785_u16;
_20 = [_13,_13,_13,_13,_13];
_19.0 = (-1348054200777053669_i64) as u32;
_18 = _9 as f64;
Call(RET = fn11(_4, _4, _2, _11, _4, _2, _19.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19.2 = _8;
_19 = (3251869776_u32, _8, _8);
(*_10) = 5_usize as i16;
_17 = _19.0 as u16;
(*_10) = _19.1 as i16;
RET = _18 as f32;
_22.fld0 = !_2;
place!(Field::<u64>(Variant(_5, 2), 0)) = 4033570412361419895_u64;
place!(Field::<u64>(Variant(_5, 2), 0)) = 2815457202966429140_u64 >> _19.0;
_23 = _3;
_4 = [185_u8,111_u8,147_u8,140_u8,103_u8,80_u8];
match _19.0 {
3251869776 => bb12,
_ => bb11
}
}
bb11 = {
_2 = 9223372036854775807_isize;
place!(Field::<u16>(Variant(_5, 1), 0)) = 62397_u16 & 6941_u16;
_11 = _7;
_8 = '\u{fe794}';
_12 = [_8,_8,_8,_8,_8,_8,_8];
_8 = '\u{25581}';
(*_10) = 8215060932118453143_i64 as i16;
(*_10) = (-82_i8) as i16;
_4 = [179_u8,95_u8,185_u8,22_u8,237_u8,40_u8];
_8 = '\u{f21c4}';
_13 = RET as u128;
place!(Field::<u16>(Variant(_5, 1), 0)) = !37939_u16;
_11 = _7;
SetDiscriminant(_5, 1);
_8 = '\u{f588e}';
_10 = core::ptr::addr_of!(_1);
_1 = (-20549_i16) | (-27635_i16);
_8 = '\u{6020}';
_2 = (-9223372036854775808_isize);
_3 = false;
_13 = 95995577827951095858398357985567434322_u128 >> (*_10);
Goto(bb6)
}
bb12 = {
(*_10) = (-18651_i16) ^ 22118_i16;
_9 = _6;
_19.2 = _8;
_22.fld0 = _18 as isize;
_9 = _6 ^ _6;
_19 = (3649143609_u32, _8, _8);
RET = (*_10) as f32;
RET = _19.0 as f32;
Call(RET = fn15(_19.1, _2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = _17 as f32;
_16 = _2;
_20 = [_13,_13,_13,_13,_13];
_22.fld1 = !_17;
(*_10) = 31544_i16;
_4 = [238_u8,136_u8,19_u8,112_u8,1_u8,49_u8];
_20 = [_13,_13,_13,_13,_13];
_23 = _3 ^ _3;
_12 = [_19.1,_19.2,_19.1,_8,_8,_19.2,_8];
_1 = 32576_i16;
_10 = core::ptr::addr_of!((*_10));
_5 = Adt41::Variant1 { fld0: _17,fld1: _19.0 };
_23 = _3;
_17 = _22.fld1 & Field::<u16>(Variant(_5, 1), 0);
_22.fld0 = _16;
_19.0 = Field::<u32>(Variant(_5, 1), 1) & Field::<u32>(Variant(_5, 1), 1);
_13 = 37292637222017158051265429343583935579_u128 << Field::<u32>(Variant(_5, 1), 1);
_18 = 5194411584024575824_i64 as f64;
Goto(bb14)
}
bb14 = {
_16 = _2 << (*_10);
_19.0 = !Field::<u32>(Variant(_5, 1), 1);
_26 = _19.1;
_6 = -_9;
_22 = Adt45 { fld0: _16,fld1: _17 };
_27 = _18;
_10 = core::ptr::addr_of!((*_10));
_16 = -_2;
_18 = -_27;
_9 = _6;
_20 = [_13,_13,_13,_13,_13];
RET = _22.fld1 as f32;
_30 = 1974582076219864680_u64 as i32;
_5 = Adt41::Variant1 { fld0: _17,fld1: _19.0 };
_20 = [_13,_13,_13,_13,_13];
_1 = (-22652_i16);
_1 = !27981_i16;
_6 = -_9;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(10_usize, 9_usize, Move(_9), 26_usize, Move(_26), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(10_usize, 3_usize, Move(_3), 4_usize, Move(_4), 12_usize, Move(_12), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(10_usize, 2_usize, Move(_2), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [u8; 6],mut _2: [u8; 6],mut _3: isize,mut _4: [char; 7],mut _5: [u8; 6],mut _6: isize,mut _7: char) -> f32 {
mir! {
type RET = f32;
let _8: [u128; 5];
let _9: [char; 8];
let _10: [char; 8];
let _11: u128;
let _12: usize;
let _13: Adt41;
let _14: [u128; 1];
let _15: [u128; 1];
let _16: f64;
let _17: (u64,);
let _18: bool;
let _19: [u128; 5];
let _20: i8;
let _21: ();
let _22: ();
{
_5 = [55_u8,164_u8,124_u8,240_u8,94_u8,79_u8];
_7 = '\u{57150}';
RET = _3 as f32;
_6 = _3 >> _3;
_4 = [_7,_7,_7,_7,_7,_7,_7];
_5 = [78_u8,172_u8,255_u8,65_u8,236_u8,238_u8];
_1 = [103_u8,104_u8,144_u8,182_u8,183_u8,213_u8];
_8 = [136605497162903485514599718195632083204_u128,240531982600022104772433186744453122492_u128,94169559009758733952467340377913765091_u128,140087274687364632050311911011081659496_u128,331509730381500657326887365245963959977_u128];
Goto(bb1)
}
bb1 = {
_3 = _6 >> _6;
_1 = [151_u8,32_u8,158_u8,225_u8,135_u8,41_u8];
_4 = [_7,_7,_7,_7,_7,_7,_7];
_6 = (-2719450145900522691_i64) as isize;
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_7 = '\u{f8d54}';
_1 = [243_u8,50_u8,230_u8,173_u8,105_u8,39_u8];
RET = 10273883925114514611_u64 as f32;
_6 = !_3;
_5 = [232_u8,39_u8,106_u8,105_u8,178_u8,48_u8];
_1 = _2;
_7 = '\u{45162}';
_8 = [258145812972797646427640177347443626431_u128,137611084955613958915287884787222182226_u128,271332008218843443833283919712186262377_u128,77657555040704376837269398823211967471_u128,163409913462592856885600381260404652325_u128];
_8 = [282221275019375538527713794234233906748_u128,122661432824461560082641225211422106629_u128,27561863097057581638313837161718831142_u128,194247972347786594992430782676184968870_u128,145116968221395905728687199210506425999_u128];
_1 = [100_u8,38_u8,92_u8,181_u8,115_u8,163_u8];
_2 = [121_u8,20_u8,64_u8,195_u8,242_u8,206_u8];
_4 = [_7,_7,_7,_7,_7,_7,_7];
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_7 = '\u{4b603}';
_7 = '\u{34679}';
Call(RET = fn12(_4, _8, _6, _8, _3, _6, _6, _3, _5, _3, _6, _5, _8, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = [_7,_7,_7,_7,_7,_7,_7,_7];
_5 = _1;
_12 = (-5263254059654464776_i64) as usize;
_6 = 4001347077746447299_u64 as isize;
_14 = [114044161869273890263617214513749400889_u128];
_15 = _14;
_15 = [15971620849261771932139995837121494607_u128];
_10 = _9;
_11 = 272575881758616462642738407542850791615_u128 << _6;
_8 = [_11,_11,_11,_11,_11];
_1 = _5;
_16 = _12 as f64;
_12 = 8961287097251217234_usize - 891080552170408680_usize;
_2 = _1;
RET = 52281_u16 as f32;
_14 = [_11];
Goto(bb3)
}
bb3 = {
_6 = _3;
_10 = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = [109_u8,93_u8,1_u8,202_u8,187_u8,161_u8];
_12 = 3_usize - 3_usize;
RET = (-114037904_i32) as f32;
_6 = _3;
_14 = [_11];
_15 = [_11];
_13 = Adt41::Variant1 { fld0: 11194_u16,fld1: 3826811137_u32 };
_4 = [_7,_7,_7,_7,_7,_7,_7];
RET = 11003_i16 as f32;
_15 = [_11];
_1 = _2;
_8 = [_11,_11,_11,_11,_11];
_5 = [14_u8,228_u8,237_u8,44_u8,115_u8,117_u8];
_17 = (196456148954424846_u64,);
_2 = _1;
_14 = [_11];
_10 = [_7,_7,_7,_7,_7,_7,_7,_7];
place!(Field::<u16>(Variant(_13, 1), 0)) = !25561_u16;
RET = (-8_i8) as f32;
RET = (-91260255638455430895986035300832750606_i128) as f32;
_17 = (12619057831862627177_u64,);
place!(Field::<u32>(Variant(_13, 1), 1)) = 3612929973_u32;
_2 = [224_u8,145_u8,104_u8,168_u8,3_u8,92_u8];
_13 = Adt41::Variant2 { fld0: _17.0 };
Call(_4 = fn13(_12, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18 = !false;
_10 = _9;
_16 = 13385_u16 as f64;
_6 = _3;
Call(_17.0 = core::intrinsics::transmute(Field::<u64>(Variant(_13, 2), 0)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_2 = [38_u8,43_u8,247_u8,97_u8,225_u8,201_u8];
_4 = [_7,_7,_7,_7,_7,_7,_7];
_12 = 53_i8 as usize;
_4 = [_7,_7,_7,_7,_7,_7,_7];
RET = 13615_u16 as f32;
_3 = _6 * _6;
RET = _3 as f32;
_3 = -_6;
_18 = RET < RET;
_3 = !_6;
_7 = '\u{80c5}';
_16 = (-4578750250416778763_i64) as f64;
_16 = _11 as f64;
_16 = 148384332306388405628045832222939301347_i128 as f64;
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_10 = [_7,_7,_7,_7,_7,_7,_7,_7];
_17 = (Field::<u64>(Variant(_13, 2), 0),);
_3 = _6;
_11 = 308338895480651741875053581362691836609_u128 | 98308716899136750130359064874932218182_u128;
_20 = _7 as i8;
_18 = !false;
Goto(bb6)
}
bb6 = {
Call(_21 = dump_var(11_usize, 17_usize, Move(_17), 14_usize, Move(_14), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_21 = dump_var(11_usize, 18_usize, Move(_18), 7_usize, Move(_7), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [char; 7],mut _2: [u128; 5],mut _3: isize,mut _4: [u128; 5],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: [u8; 6],mut _10: isize,mut _11: isize,mut _12: [u8; 6],mut _13: [u128; 5],mut _14: [u128; 5]) -> f32 {
mir! {
type RET = f32;
let _15: i128;
let _16: isize;
let _17: i128;
let _18: char;
let _19: ([char; 8],);
let _20: i8;
let _21: [bool; 5];
let _22: u128;
let _23: *const (u8, i16, u16);
let _24: i8;
let _25: u8;
let _26: i32;
let _27: [char; 8];
let _28: char;
let _29: Adt41;
let _30: (u8, i16, u16);
let _31: f64;
let _32: Adt45;
let _33: (bool,);
let _34: u64;
let _35: Adt42;
let _36: (bool,);
let _37: u8;
let _38: *const u8;
let _39: u8;
let _40: bool;
let _41: Adt51;
let _42: Adt42;
let _43: ();
let _44: ();
{
_4 = [340019178759696504386930486955165740132_u128,72310921502184297294759933015882775387_u128,155901960911022770210405114857240641519_u128,32258281918937380733398182522692739665_u128,249293235076226480920017323162008674787_u128];
_4 = [309783577896912069164277064786135592926_u128,91279050779004768219381287596740372406_u128,86386975482008280574355203232662957251_u128,14349183950198222513755903649722040662_u128,335496304482558500359053687299225725499_u128];
_16 = -_8;
_1 = ['\u{56657}','\u{cacb}','\u{fd7ed}','\u{107f82}','\u{4f1ea}','\u{611b5}','\u{15486}'];
_7 = !_6;
_15 = -(-118252984320761943092405891689710379130_i128);
Call(_15 = core::intrinsics::bswap((-117477672454084712500502530844364404074_i128)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = [185_u8,201_u8,178_u8,127_u8,219_u8,5_u8];
_6 = _7 >> _10;
_8 = _11;
RET = 80_i8 as f32;
Goto(bb2)
}
bb2 = {
Goto(bb3)
}
bb3 = {
_4 = [185601610374883124832669814733819910660_u128,310132216198449793350224333978936458020_u128,259677166963395407087538030115498075957_u128,178947583131596920368555928423560977804_u128,229561861838324151590520764237523919250_u128];
_10 = _3 | _3;
_19.0 = ['\u{1037ec}','\u{43618}','\u{67168}','\u{a70a4}','\u{5e60b}','\u{1ae4}','\u{41692}','\u{60c91}'];
RET = 12994248880375959735_usize as f32;
_19.0 = ['\u{5e4db}','\u{da3e4}','\u{5fc43}','\u{9a2b8}','\u{64c01}','\u{1f6c5}','\u{1a87a}','\u{a0270}'];
RET = 633826171_i32 as f32;
_18 = '\u{1a67a}';
_6 = -_10;
_11 = 61_u8 as isize;
_9 = [162_u8,105_u8,227_u8,20_u8,165_u8,79_u8];
_17 = !_15;
_11 = 11976_u16 as isize;
_13 = [43486846750245772147895328248479492678_u128,97319591115067152185810087823934227596_u128,600385885024693189071035895647007084_u128,331604282448235655559639105781780419574_u128,333308434463590890303127747757933432306_u128];
_7 = _17 as isize;
_4 = [96313103711638362776528583229595428767_u128,20570716900977191572189395052892196515_u128,133692574482785240121092803146680397164_u128,96460266838908661735154792813885070898_u128,270180512934213946354890562241380236203_u128];
_18 = '\u{e16cb}';
_12 = _9;
_21 = [true,false,true,true,true];
_20 = -(-72_i8);
_9 = [104_u8,12_u8,70_u8,208_u8,47_u8,173_u8];
_5 = 1777671827_i32 as isize;
_22 = !187732257693862422046668426141363784683_u128;
_10 = _8;
RET = 17616_u16 as f32;
_5 = !_8;
_14 = [_22,_22,_22,_22,_22];
_18 = '\u{c1336}';
Call(RET = core::intrinsics::transmute(_18), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = !_17;
_8 = _3 - _3;
_7 = !_16;
_21 = [true,true,false,true,true];
_18 = '\u{aefd4}';
_24 = -_20;
_3 = !_16;
_14 = [_22,_22,_22,_22,_22];
_25 = _18 as u8;
_12 = [_25,_25,_25,_25,_25,_25];
_24 = _20;
_1 = [_18,_18,_18,_18,_18,_18,_18];
_18 = '\u{42d2e}';
_15 = _17 << _5;
_10 = RET as isize;
Goto(bb5)
}
bb5 = {
_25 = !63_u8;
_9 = _12;
_15 = _17 * _17;
_8 = _6 | _6;
_19.0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_13 = [_22,_22,_22,_22,_22];
_4 = _2;
_1 = [_18,_18,_18,_18,_18,_18,_18];
_25 = !57_u8;
_25 = !158_u8;
_13 = _2;
_4 = [_22,_22,_22,_22,_22];
_21 = [true,false,false,false,false];
_27 = [_18,_18,_18,_18,_18,_18,_18,_18];
_10 = !_8;
_6 = -_10;
_13 = [_22,_22,_22,_22,_22];
_3 = _6 + _16;
_12 = [_25,_25,_25,_25,_25,_25];
_21 = [false,false,false,true,false];
_23 = core::ptr::addr_of!(_30);
_10 = 6573584142699691112_i64 as isize;
_30 = (_25, (-28517_i16), 44691_u16);
match _30.2 {
0 => bb6,
1 => bb7,
44691 => bb9,
_ => bb8
}
}
bb6 = {
_15 = !_17;
_8 = _3 - _3;
_7 = !_16;
_21 = [true,true,false,true,true];
_18 = '\u{aefd4}';
_24 = -_20;
_3 = !_16;
_14 = [_22,_22,_22,_22,_22];
_25 = _18 as u8;
_12 = [_25,_25,_25,_25,_25,_25];
_24 = _20;
_1 = [_18,_18,_18,_18,_18,_18,_18];
_18 = '\u{42d2e}';
_15 = _17 << _5;
_10 = RET as isize;
Goto(bb5)
}
bb7 = {
_12 = [185_u8,201_u8,178_u8,127_u8,219_u8,5_u8];
_6 = _7 >> _10;
_8 = _11;
RET = 80_i8 as f32;
Goto(bb2)
}
bb8 = {
Goto(bb3)
}
bb9 = {
_5 = _6 + _6;
_20 = _24 << _3;
_2 = _13;
_30.1 = (-20176_i16);
_17 = _15;
_17 = -_15;
(*_23) = (_25, 21904_i16, 59173_u16);
_19 = (_27,);
(*_23) = (_25, (-2206_i16), 16976_u16);
_8 = _6;
_2 = [_22,_22,_22,_22,_22];
_7 = _3 ^ _5;
_28 = _18;
_32 = Adt45 { fld0: _5,fld1: (*_23).2 };
_26 = -1331823088_i32;
(*_23).2 = !_32.fld1;
_20 = _24;
_28 = _18;
_4 = _2;
_29 = Adt41::Variant1 { fld0: _30.2,fld1: 899022799_u32 };
Goto(bb10)
}
bb10 = {
_27 = _19.0;
RET = _24 as f32;
_32.fld1 = !_30.2;
_22 = (*_23).0 as u128;
_15 = _17;
_32 = Adt45 { fld0: _7,fld1: Field::<u16>(Variant(_29, 1), 0) };
_16 = _7;
(*_23).2 = !Field::<u16>(Variant(_29, 1), 0);
place!(Field::<u32>(Variant(_29, 1), 1)) = !4210105340_u32;
(*_23) = (_25, 13932_i16, Field::<u16>(Variant(_29, 1), 0));
SetDiscriminant(_29, 1);
_31 = 3139888655_u32 as f64;
_32.fld1 = !_30.2;
_32.fld1 = (*_23).2 & _30.2;
_12 = [_30.0,_30.0,_30.0,(*_23).0,_30.0,_30.0];
_32 = Adt45 { fld0: _5,fld1: _30.2 };
place!(Field::<u16>(Variant(_29, 1), 0)) = 5853564520833189322_i64 as u16;
_24 = _20 | _20;
_29 = Adt41::Variant1 { fld0: _30.2,fld1: 1288407836_u32 };
match _30.1 {
0 => bb11,
1 => bb12,
13932 => bb14,
_ => bb13
}
}
bb11 = {
_4 = [185601610374883124832669814733819910660_u128,310132216198449793350224333978936458020_u128,259677166963395407087538030115498075957_u128,178947583131596920368555928423560977804_u128,229561861838324151590520764237523919250_u128];
_10 = _3 | _3;
_19.0 = ['\u{1037ec}','\u{43618}','\u{67168}','\u{a70a4}','\u{5e60b}','\u{1ae4}','\u{41692}','\u{60c91}'];
RET = 12994248880375959735_usize as f32;
_19.0 = ['\u{5e4db}','\u{da3e4}','\u{5fc43}','\u{9a2b8}','\u{64c01}','\u{1f6c5}','\u{1a87a}','\u{a0270}'];
RET = 633826171_i32 as f32;
_18 = '\u{1a67a}';
_6 = -_10;
_11 = 61_u8 as isize;
_9 = [162_u8,105_u8,227_u8,20_u8,165_u8,79_u8];
_17 = !_15;
_11 = 11976_u16 as isize;
_13 = [43486846750245772147895328248479492678_u128,97319591115067152185810087823934227596_u128,600385885024693189071035895647007084_u128,331604282448235655559639105781780419574_u128,333308434463590890303127747757933432306_u128];
_7 = _17 as isize;
_4 = [96313103711638362776528583229595428767_u128,20570716900977191572189395052892196515_u128,133692574482785240121092803146680397164_u128,96460266838908661735154792813885070898_u128,270180512934213946354890562241380236203_u128];
_18 = '\u{e16cb}';
_12 = _9;
_21 = [true,false,true,true,true];
_20 = -(-72_i8);
_9 = [104_u8,12_u8,70_u8,208_u8,47_u8,173_u8];
_5 = 1777671827_i32 as isize;
_22 = !187732257693862422046668426141363784683_u128;
_10 = _8;
RET = 17616_u16 as f32;
_5 = !_8;
_14 = [_22,_22,_22,_22,_22];
_18 = '\u{c1336}';
Call(RET = core::intrinsics::transmute(_18), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_15 = !_17;
_8 = _3 - _3;
_7 = !_16;
_21 = [true,true,false,true,true];
_18 = '\u{aefd4}';
_24 = -_20;
_3 = !_16;
_14 = [_22,_22,_22,_22,_22];
_25 = _18 as u8;
_12 = [_25,_25,_25,_25,_25,_25];
_24 = _20;
_1 = [_18,_18,_18,_18,_18,_18,_18];
_18 = '\u{42d2e}';
_15 = _17 << _5;
_10 = RET as isize;
Goto(bb5)
}
bb13 = {
Goto(bb3)
}
bb14 = {
_23 = core::ptr::addr_of!((*_23));
_36 = (true,);
_7 = _6 + _5;
(*_23).0 = _25;
_1 = [_28,_28,_18,_28,_18,_28,_18];
_35 = Adt42::Variant0 { fld0: _36.0 };
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(12_usize, 27_usize, Move(_27), 7_usize, Move(_7), 17_usize, Move(_17), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(12_usize, 14_usize, Move(_14), 1_usize, Move(_1), 21_usize, Move(_21), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(12_usize, 6_usize, Move(_6), 19_usize, Move(_19), 8_usize, Move(_8), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(12_usize, 10_usize, Move(_10), 28_usize, Move(_28), 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: usize,mut _2: [u8; 6]) -> [char; 7] {
mir! {
type RET = [char; 7];
let _3: (u32, char, char);
let _4: isize;
let _5: [u128; 1];
let _6: u64;
let _7: u128;
let _8: [u8; 6];
let _9: Adt57;
let _10: Adt54;
let _11: [char; 7];
let _12: i32;
let _13: [bool; 5];
let _14: ([char; 8],);
let _15: f32;
let _16: bool;
let _17: f64;
let _18: *const u128;
let _19: (u8, i16, u16);
let _20: isize;
let _21: Adt44;
let _22: usize;
let _23: i16;
let _24: Adt41;
let _25: bool;
let _26: f32;
let _27: i16;
let _28: Adt54;
let _29: [char; 5];
let _30: i64;
let _31: *mut f32;
let _32: ();
let _33: ();
{
RET = ['\u{fdb96}','\u{101762}','\u{10d89a}','\u{2a7af}','\u{17e8d}','\u{577cb}','\u{82e47}'];
_2 = [107_u8,50_u8,226_u8,213_u8,11_u8,252_u8];
_3 = (1731314829_u32, '\u{78c5b}', '\u{806f}');
RET = [_3.1,_3.2,_3.2,_3.2,_3.1,_3.1,_3.2];
RET = [_3.2,_3.2,_3.2,_3.2,_3.1,_3.2,_3.2];
_5 = [130595813819362520026052339467083497169_u128];
_4 = 9223372036854775807_isize;
_3 = (2012782291_u32, '\u{251e}', '\u{678de}');
_1 = !5_usize;
_1 = !4_usize;
RET = [_3.1,_3.1,_3.2,_3.1,_3.1,_3.1,_3.2];
RET = [_3.2,_3.2,_3.2,_3.1,_3.2,_3.2,_3.2];
Call(_4 = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = (107460895_u32, '\u{2a16c}', '\u{105175}');
_6 = !877266131756826599_u64;
RET = [_3.2,_3.1,_3.2,_3.1,_3.1,_3.1,_3.2];
_3.1 = _3.2;
_4 = 233318988857064922211104359807437795703_u128 as isize;
_2 = [48_u8,246_u8,19_u8,103_u8,68_u8,70_u8];
_3.1 = _3.2;
_1 = 0_usize >> _3.0;
_3.2 = _3.1;
_2 = [200_u8,43_u8,206_u8,160_u8,160_u8,29_u8];
_3 = (1356905283_u32, '\u{d518}', '\u{9f259}');
_3.2 = _3.1;
_3.2 = _3.1;
_5 = [231346630238900438633097644527817279045_u128];
_2 = [152_u8,124_u8,98_u8,81_u8,224_u8,99_u8];
_8 = _2;
_6 = _1 as u64;
_2 = [102_u8,74_u8,214_u8,116_u8,241_u8,227_u8];
Goto(bb2)
}
bb2 = {
_2 = _8;
_7 = !89425963477744832085713748609337290941_u128;
_3.1 = _3.2;
_3 = (1970326484_u32, '\u{5dea0}', '\u{f492e}');
_8 = [145_u8,148_u8,94_u8,253_u8,164_u8,73_u8];
RET = [_3.1,_3.1,_3.2,_3.2,_3.1,_3.1,_3.2];
_2 = _8;
_3 = (3709971857_u32, '\u{d5770}', '\u{583c8}');
RET = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.2,_3.2];
RET = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.2,_3.2];
_11 = [_3.2,_3.2,_3.1,_3.1,_3.2,_3.2,_3.2];
_4 = (-3_isize) >> _6;
_12 = 1373973384_i32 - (-1086268908_i32);
RET = _11;
_3.0 = 21017_u16 as u32;
_13 = [false,false,false,true,true];
Goto(bb3)
}
bb3 = {
_14.0 = [_3.2,_3.2,_3.2,_3.1,_3.2,_3.2,_3.2,_3.1];
Goto(bb4)
}
bb4 = {
_6 = 3711100268876006337_u64 << _4;
_6 = 168573321429775711050239387713141298063_i128 as u64;
_4 = !91_isize;
_2 = _8;
_6 = 6426640184928565539_u64;
_16 = true;
_3 = (4071815791_u32, '\u{5c2a5}', '\u{90435}');
_1 = 4_usize;
RET = [_3.1,_14.0[_1],_3.2,_11[_1],_3.2,_3.1,_11[_1]];
_14.0 = [_11[_1],_3.1,_11[_1],_3.2,RET[_1],RET[_1],RET[_1],_3.1];
_12 = _4 as i32;
_8 = _2;
_11[_1] = _14.0[_1];
_14.0 = [_3.2,_3.2,_11[_1],_3.2,_3.1,_11[_1],RET[_1],_11[_1]];
_13[_1] = !_16;
_18 = core::ptr::addr_of!(_7);
_3 = (2818411551_u32, _11[_1], _11[_1]);
_8[_1] = !_2[_1];
_15 = _3.0 as f32;
RET = [_11[_1],_3.1,_11[_1],_3.1,_11[_1],_14.0[_1],_11[_1]];
_16 = _13[_1];
match _1 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb9,
_ => bb8
}
}
bb5 = {
_14.0 = [_3.2,_3.2,_3.2,_3.1,_3.2,_3.2,_3.2,_3.1];
Goto(bb4)
}
bb6 = {
_2 = _8;
_7 = !89425963477744832085713748609337290941_u128;
_3.1 = _3.2;
_3 = (1970326484_u32, '\u{5dea0}', '\u{f492e}');
_8 = [145_u8,148_u8,94_u8,253_u8,164_u8,73_u8];
RET = [_3.1,_3.1,_3.2,_3.2,_3.1,_3.1,_3.2];
_2 = _8;
_3 = (3709971857_u32, '\u{d5770}', '\u{583c8}');
RET = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.2,_3.2];
RET = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.2,_3.2];
_11 = [_3.2,_3.2,_3.1,_3.1,_3.2,_3.2,_3.2];
_4 = (-3_isize) >> _6;
_12 = 1373973384_i32 - (-1086268908_i32);
RET = _11;
_3.0 = 21017_u16 as u32;
_13 = [false,false,false,true,true];
Goto(bb3)
}
bb7 = {
_3 = (107460895_u32, '\u{2a16c}', '\u{105175}');
_6 = !877266131756826599_u64;
RET = [_3.2,_3.1,_3.2,_3.1,_3.1,_3.1,_3.2];
_3.1 = _3.2;
_4 = 233318988857064922211104359807437795703_u128 as isize;
_2 = [48_u8,246_u8,19_u8,103_u8,68_u8,70_u8];
_3.1 = _3.2;
_1 = 0_usize >> _3.0;
_3.2 = _3.1;
_2 = [200_u8,43_u8,206_u8,160_u8,160_u8,29_u8];
_3 = (1356905283_u32, '\u{d518}', '\u{9f259}');
_3.2 = _3.1;
_3.2 = _3.1;
_5 = [231346630238900438633097644527817279045_u128];
_2 = [152_u8,124_u8,98_u8,81_u8,224_u8,99_u8];
_8 = _2;
_6 = _1 as u64;
_2 = [102_u8,74_u8,214_u8,116_u8,241_u8,227_u8];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_3.2 = _3.1;
_19.2 = 15766_u16 & 58958_u16;
RET[_1] = _11[_1];
RET[_1] = _14.0[_1];
_19.1 = -(-2624_i16);
RET[_1] = _3.2;
_3.2 = _11[_1];
_19 = (_8[_1], (-2613_i16), 47068_u16);
_19 = (_2[_1], (-5538_i16), 32613_u16);
_16 = !_13[_1];
_11[_1] = _3.2;
_3.2 = _14.0[_1];
_8 = [_2[_1],_19.0,_2[_1],_2[_1],_2[_1],_2[_1]];
RET = _11;
_3.0 = !217829289_u32;
_16 = _13[_1];
_13 = [_16,_16,_16,_16,_16];
_14.0 = [_3.1,RET[_1],RET[_1],_11[_1],RET[_1],_11[_1],_11[_1],_3.1];
_2[_1] = _19.0 | _19.0;
match _19.1 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb4,
4 => bb10,
340282366920938463463374607431768205918 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_2 = _8;
_7 = !89425963477744832085713748609337290941_u128;
_3.1 = _3.2;
_3 = (1970326484_u32, '\u{5dea0}', '\u{f492e}');
_8 = [145_u8,148_u8,94_u8,253_u8,164_u8,73_u8];
RET = [_3.1,_3.1,_3.2,_3.2,_3.1,_3.1,_3.2];
_2 = _8;
_3 = (3709971857_u32, '\u{d5770}', '\u{583c8}');
RET = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.2,_3.2];
RET = [_3.1,_3.1,_3.1,_3.1,_3.1,_3.2,_3.2];
_11 = [_3.2,_3.2,_3.1,_3.1,_3.2,_3.2,_3.2];
_4 = (-3_isize) >> _6;
_12 = 1373973384_i32 - (-1086268908_i32);
RET = _11;
_3.0 = 21017_u16 as u32;
_13 = [false,false,false,true,true];
Goto(bb3)
}
bb12 = {
(*_18) = !78675754080324237146758296882607100585_u128;
_3.2 = _11[_1];
_15 = 51_i8 as f32;
_2[_1] = _8[_1] * _19.0;
_23 = _19.1;
_22 = (-60_i8) as usize;
_19.1 = -_23;
_3.1 = _14.0[_1];
_5 = [_7];
_17 = (-125_i8) as f64;
_19.2 = 32487_u16 << _2[_1];
_11[_1] = _3.1;
Call(_18 = fn14(_19, RET[_1], RET, _3.1, _17, _1, _23, _19.2, _19.2, RET, _11[_1], _2, _2, _19.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16 = _3.1 <= _3.2;
_1 = _4 as usize;
_2 = _8;
_8 = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
_3.0 = !2094952436_u32;
_26 = _22 as f32;
_20 = _4 & _4;
Goto(bb14)
}
bb14 = {
_8 = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
_19 = (94_u8, _23, 34046_u16);
_4 = !_20;
_5 = [_7];
_19.1 = -_23;
_20 = _4 + _4;
_3.1 = _3.2;
_31 = core::ptr::addr_of_mut!(_15);
_29 = [_3.2,_3.2,_3.1,_3.2,_3.1];
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(13_usize, 3_usize, Move(_3), 14_usize, Move(_14), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(13_usize, 11_usize, Move(_11), 6_usize, Move(_6), 22_usize, Move(_22), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(13_usize, 12_usize, Move(_12), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (u8, i16, u16),mut _2: char,mut _3: [char; 7],mut _4: char,mut _5: f64,mut _6: usize,mut _7: i16,mut _8: u16,mut _9: u16,mut _10: [char; 7],mut _11: char,mut _12: [u8; 6],mut _13: [u8; 6],mut _14: u8) -> *const u128 {
mir! {
type RET = *const u128;
let _15: (&'static [u8; 6], f32, *const i32, u64);
let _16: f32;
let _17: Adt54;
let _18: (u8, i16, u16);
let _19: u8;
let _20: Adt51;
let _21: *const u8;
let _22: f64;
let _23: [char; 5];
let _24: bool;
let _25: isize;
let _26: i16;
let _27: usize;
let _28: f32;
let _29: f32;
let _30: ();
let _31: ();
{
_1.0 = _12[_6] * _13[_6];
_9 = _8 + _1.2;
_7 = _1.1 ^ _1.1;
_13 = [_12[_6],_1.0,_14,_12[_6],_1.0,_14];
_5 = (-165740990461695652221715919616183897354_i128) as f64;
_13 = _12;
_3 = [_10[_6],_11,_11,_4,_11,_11,_2];
_5 = 50_i8 as f64;
_1 = (_12[_6], _7, _9);
_10[_6] = _2;
_3[_6] = _2;
_10[_6] = _2;
_9 = _8 | _1.2;
Goto(bb1)
}
bb1 = {
_15.0 = &_13;
_3[_6] = _11;
_15.3 = !292266773388889864_u64;
_11 = _3[_6];
_4 = _10[_6];
_11 = _10[_6];
_7 = _1.1 >> _12[_6];
_4 = _2;
_6 = 5_usize;
match _12[_6] {
0 => bb2,
1 => bb3,
2 => bb4,
73 => bb6,
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
_6 = 266085854883180556344035230284746944318_u128 as usize;
_18.0 = !_14;
_18 = (_1.0, _7, _9);
_3 = [_11,_2,_2,_4,_2,_4,_11];
_12 = _13;
_12 = [_1.0,_14,_14,_18.0,_14,_1.0];
_18.2 = !_1.2;
_16 = _6 as f32;
_1.1 = -_18.1;
_14 = (-37_i8) as u8;
_18.1 = -_1.1;
Call(_14 = core::intrinsics::bswap(_18.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = (-5684861016068941301_i64) as f64;
_1.2 = false as u16;
_18 = _1;
_11 = _4;
_4 = _2;
_1 = _18;
_6 = !7_usize;
_2 = _4;
_6 = 76_i8 as usize;
Goto(bb8)
}
bb8 = {
_1.0 = _18.0 >> _9;
_1.2 = _9 & _9;
_6 = 14216396587171519630_usize * 2_usize;
_15.1 = -_16;
_18.2 = !_9;
_19 = !_1.0;
_11 = _4;
_15.0 = &_13;
_15.0 = &_13;
_1.1 = _18.1 | _18.1;
_1.0 = !_18.0;
_17 = Adt54::Variant1 { fld0: _15.1,fld1: _3 };
_15.0 = &_12;
_9 = _15.3 as u16;
Goto(bb9)
}
bb9 = {
_18.1 = 986507757_i32 as i16;
_19 = _1.0;
_13 = _12;
_13 = [_18.0,_18.0,_1.0,_19,_1.0,_1.0];
_4 = _11;
_1.0 = Field::<f32>(Variant(_17, 1), 0) as u8;
_5 = _19 as f64;
_5 = _15.3 as f64;
_10 = [_11,_11,_2,_4,_11,_2,_11];
_6 = _5 as usize;
_21 = core::ptr::addr_of!(_18.0);
_1.1 = !_7;
place!(Field::<f32>(Variant(_17, 1), 0)) = _18.2 as f32;
_21 = core::ptr::addr_of!((*_21));
_24 = !true;
SetDiscriminant(_17, 0);
_18.1 = _15.3 as i16;
place!(Field::<u128>(Variant(_17, 0), 0)) = 325313834477645722845557619838130559362_u128 >> (*_21);
Goto(bb10)
}
bb10 = {
_12 = [_14,_18.0,_1.0,(*_21),(*_21),_18.0];
place!(Field::<u16>(Variant(_17, 0), 2)) = _18.2 - _1.2;
RET = core::ptr::addr_of!(place!(Field::<u128>(Variant(_17, 0), 0)));
(*RET) = 320758270789311324932600099831714787691_u128 & 119105952406061975018780401451171162965_u128;
_21 = core::ptr::addr_of!(_19);
_6 = 3_usize ^ 3_usize;
_27 = _6 << Field::<u16>(Variant(_17, 0), 2);
_23 = [_11,_2,_2,_11,_4];
_15.3 = !4033295425520497481_u64;
_15.2 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_17, 0), 1)));
_21 = core::ptr::addr_of!(_1.0);
_26 = _16 as i16;
_1.2 = Field::<u16>(Variant(_17, 0), 2);
(*RET) = 96280236749201601183941737067468959665_u128 ^ 27477991371184460527575743853757929797_u128;
place!(Field::<i32>(Variant(_17, 0), 1)) = (-1275782520_i32);
_3 = [_2,_4,_2,_4,_2,_11,_11];
(*RET) = _24 as u128;
_13 = [_19,_19,_19,_18.0,_14,_19];
_13 = _12;
_10 = [_11,_4,_11,_11,_4,_4,_2];
_18.1 = (-26_i8) as i16;
_7 = 123_i8 as i16;
place!(Field::<u128>(Variant(_17, 0), 0)) = 9_i8 as u128;
_19 = (*_21) * _18.0;
place!(Field::<i32>(Variant(_17, 0), 1)) = (-1526957844_i32) ^ 651289138_i32;
_15.2 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_17, 0), 1)));
_18.0 = !(*_21);
_5 = 25_isize as f64;
_15.3 = Field::<i32>(Variant(_17, 0), 1) as u64;
_15.1 = Field::<i32>(Variant(_17, 0), 1) as f32;
Goto(bb11)
}
bb11 = {
Call(_30 = dump_var(14_usize, 12_usize, Move(_12), 6_usize, Move(_6), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_30 = dump_var(14_usize, 27_usize, Move(_27), 3_usize, Move(_3), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_30 = dump_var(14_usize, 9_usize, Move(_9), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: char,mut _2: isize) -> f32 {
mir! {
type RET = f32;
let _3: [char; 5];
let _4: Adt49;
let _5: ([char; 8],);
let _6: [bool; 5];
let _7: ([char; 8],);
let _8: [char; 7];
let _9: [char; 7];
let _10: isize;
let _11: bool;
let _12: isize;
let _13: (bool,);
let _14: Adt52;
let _15: i128;
let _16: Adt56;
let _17: [u8; 6];
let _18: Adt42;
let _19: f32;
let _20: [u128; 5];
let _21: i128;
let _22: (u32, char, char);
let _23: u128;
let _24: [u128; 5];
let _25: &'static [u8; 6];
let _26: Adt45;
let _27: u32;
let _28: [char; 7];
let _29: i128;
let _30: (bool,);
let _31: [char; 8];
let _32: ();
let _33: ();
{
RET = 144_u8 as f32;
Goto(bb1)
}
bb1 = {
_2 = _1 as isize;
RET = (-8644988228145034947_i64) as f32;
RET = 46_i8 as f32;
_1 = '\u{1f688}';
RET = 206572107337067715561310225583496584455_u128 as f32;
_2 = RET as isize;
RET = 114_i8 as f32;
_1 = '\u{c2873}';
_3 = [_1,_1,_1,_1,_1];
_2 = 19811637863513040746806010910774383408_i128 as isize;
RET = 76_u8 as f32;
Goto(bb2)
}
bb2 = {
_1 = '\u{d0b73}';
_6 = [false,true,false,true,true];
_2 = 9223372036854775807_isize;
RET = _2 as f32;
RET = 794984404_i32 as f32;
_6 = [true,true,false,false,false];
match _2 {
0 => bb3,
1 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb3 = {
_2 = _1 as isize;
RET = (-8644988228145034947_i64) as f32;
RET = 46_i8 as f32;
_1 = '\u{1f688}';
RET = 206572107337067715561310225583496584455_u128 as f32;
_2 = RET as isize;
RET = 114_i8 as f32;
_1 = '\u{c2873}';
_3 = [_1,_1,_1,_1,_1];
_2 = 19811637863513040746806010910774383408_i128 as isize;
RET = 76_u8 as f32;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_6 = [true,true,false,false,false];
_1 = '\u{71015}';
_7.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = 3_usize as f32;
_2 = 9223372036854775807_isize;
RET = (-789250842_i32) as f32;
_1 = '\u{87fc0}';
_7.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_5 = (_7.0,);
_5 = (_7.0,);
_5 = (_7.0,);
_8 = [_1,_1,_1,_1,_1,_1,_1];
_5.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_7.0 = _5.0;
_8 = [_1,_1,_1,_1,_1,_1,_1];
RET = (-21605_i16) as f32;
_2 = -9223372036854775807_isize;
RET = 96957607449599556968289356235904273343_i128 as f32;
_3 = [_1,_1,_1,_1,_1];
_9 = _8;
_2 = 15060917051983776813_u64 as isize;
_6 = [true,false,true,true,false];
RET = 15_u8 as f32;
_11 = _2 == _2;
_7 = (_5.0,);
_7.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_11 = false;
Call(_9 = fn16(_11, RET, _5, _7, _6, _6, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = _8;
_5 = _7;
_2 = RET as isize;
Call(_12 = core::intrinsics::transmute(_2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = '\u{5c845}';
Goto(bb9)
}
bb9 = {
_6 = [_11,_11,_11,_11,_11];
_9 = _8;
Goto(bb10)
}
bb10 = {
_7.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = (-680813993_i32) as isize;
_3 = [_1,_1,_1,_1,_1];
_6 = [_11,_11,_11,_11,_11];
_9 = [_1,_1,_1,_1,_1,_1,_1];
_13.0 = _11 & _11;
_10 = 307124332076204461882493073161709940849_u128 as isize;
Goto(bb11)
}
bb11 = {
_13 = (_11,);
_11 = _13.0;
RET = (-94965796933086445864523571483590360582_i128) as f32;
_12 = _1 as isize;
_13 = (_11,);
_2 = _12;
_5.0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_16.fld0 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = _10 ^ _10;
RET = 9201_u16 as f32;
_15 = 9011957593621077290_usize as i128;
_13.0 = _11;
Goto(bb12)
}
bb12 = {
_15 = 4658496495260260946_i64 as i128;
RET = _10 as f32;
_16.fld2 = RET as u32;
_5.0 = _16.fld0;
_17 = [169_u8,189_u8,254_u8,95_u8,6_u8,45_u8];
_3 = [_1,_1,_1,_1,_1];
RET = 29213_u16 as f32;
_5 = (_7.0,);
_16.fld2 = 905843627596085432_i64 as u32;
_16.fld2 = 1835820593_u32;
_7 = (_16.fld0,);
_22.2 = _1;
_12 = -_2;
_7.0 = _16.fld0;
Goto(bb13)
}
bb13 = {
_15 = -3445760215848966582122026070064710623_i128;
_22.0 = _16.fld2;
RET = (-117_i8) as f32;
_5.0 = _16.fld0;
_23 = _1 as u128;
_22.2 = _1;
_13 = (_11,);
_7 = (_5.0,);
RET = 32691_i16 as f32;
_19 = RET;
_13.0 = _2 > _2;
_24 = [_23,_23,_23,_23,_23];
_22.1 = _1;
_7.0 = _5.0;
_26.fld1 = !21595_u16;
_26 = Adt45 { fld0: _12,fld1: 9586_u16 };
_10 = !_2;
_16.fld2 = _15 as u32;
_26 = Adt45 { fld0: _12,fld1: 4934_u16 };
_25 = &_17;
_10 = !_26.fld0;
_25 = &(*_25);
_28 = _8;
_2 = _10 - _10;
Goto(bb14)
}
bb14 = {
RET = _19 - _19;
_25 = &(*_25);
_22 = (_16.fld2, _1, _1);
_21 = RET as i128;
_30.0 = _22.2 < _1;
_26.fld0 = _2;
_30.0 = !_13.0;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(15_usize, 21_usize, Move(_21), 15_usize, Move(_15), 30_usize, Move(_30), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(15_usize, 22_usize, Move(_22), 17_usize, Move(_17), 10_usize, Move(_10), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(15_usize, 1_usize, Move(_1), 11_usize, Move(_11), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: f32,mut _3: ([char; 8],),mut _4: ([char; 8],),mut _5: [bool; 5],mut _6: [bool; 5],mut _7: [bool; 5]) -> [char; 7] {
mir! {
type RET = [char; 7];
let _8: f32;
let _9: *mut [char; 7];
let _10: [u128; 1];
let _11: [char; 7];
let _12: bool;
let _13: i64;
let _14: u16;
let _15: (bool,);
let _16: (u64,);
let _17: [char; 5];
let _18: isize;
let _19: *const (u8, i16, u16);
let _20: u128;
let _21: *const (&'static [u8; 6], f32, *const i32, u64);
let _22: [u128; 1];
let _23: f32;
let _24: [u128; 5];
let _25: *const u128;
let _26: Adt49;
let _27: Adt44;
let _28: Adt52;
let _29: i16;
let _30: Adt42;
let _31: f64;
let _32: ();
let _33: ();
{
RET = ['\u{bdc4f}','\u{44e65}','\u{b5567}','\u{aa49}','\u{3722f}','\u{12665}','\u{2905}'];
RET = ['\u{36623}','\u{47375}','\u{e43e3}','\u{e883f}','\u{727fe}','\u{5ac1b}','\u{d1c47}'];
_3 = _4;
_3.0 = ['\u{fb9ef}','\u{240eb}','\u{80520}','\u{d5be1}','\u{9c2e9}','\u{10ceeb}','\u{3724f}','\u{1b01b}'];
_2 = 45015_u16 as f32;
_8 = -_2;
_8 = -_2;
_7 = _5;
_8 = 8600077903452108551_usize as f32;
_3 = _4;
_5 = [_1,_1,_1,_1,_1];
_9 = core::ptr::addr_of_mut!(RET);
_1 = !true;
_4 = _3;
RET = ['\u{a69d5}','\u{be50b}','\u{ada3e}','\u{55a68}','\u{352a4}','\u{85d2e}','\u{7004c}'];
_4.0 = ['\u{3f370}','\u{26efd}','\u{27a92}','\u{d4081}','\u{47806}','\u{12ca9}','\u{ba94f}','\u{a589a}'];
_5 = [_1,_1,_1,_1,_1];
RET = ['\u{87af4}','\u{d9ee6}','\u{a9594}','\u{19c5f}','\u{a2a96}','\u{a377a}','\u{1ffe9}'];
RET = ['\u{76cee}','\u{2c73a}','\u{660e4}','\u{a5e48}','\u{f74a5}','\u{88a7}','\u{10d373}'];
_5 = _7;
_6 = _7;
_10 = [323929681845694688331050716036497073712_u128];
Goto(bb1)
}
bb1 = {
RET = ['\u{ecfbe}','\u{acb96}','\u{7cc06}','\u{9f547}','\u{42e4}','\u{b8606}','\u{7b049}'];
_2 = _8 + _8;
_10 = [149448439009906439330391044046734211932_u128];
_7 = [_1,_1,_1,_1,_1];
_10 = [69342116209873125708708431183231320763_u128];
(*_9) = ['\u{b1f45}','\u{99a1f}','\u{411fc}','\u{78565}','\u{6129}','\u{c714d}','\u{a4bc8}'];
Goto(bb2)
}
bb2 = {
_3.0 = _4.0;
_5 = [_1,_1,_1,_1,_1];
_8 = _2 + _2;
_12 = _1;
_13 = !953444221242716433_i64;
_4 = _3;
Goto(bb3)
}
bb3 = {
_5 = [_12,_12,_1,_12,_12];
_3.0 = ['\u{5fe40}','\u{a6936}','\u{ecb5d}','\u{66582}','\u{79b17}','\u{86228}','\u{9c779}','\u{c5827}'];
_6 = [_1,_12,_12,_12,_12];
_8 = 129694045715764029089683551489966977555_i128 as f32;
_11 = ['\u{73983}','\u{5bd83}','\u{24236}','\u{ef003}','\u{c04ba}','\u{381c}','\u{6c522}'];
_3 = _4;
_7 = [_1,_12,_12,_12,_1];
RET = ['\u{d022a}','\u{1aa6c}','\u{23527}','\u{156c}','\u{6d52f}','\u{e93ae}','\u{fd8bd}'];
RET = ['\u{d165c}','\u{bed1a}','\u{11c67}','\u{10b876}','\u{1fe47}','\u{4e1e6}','\u{b6389}'];
(*_9) = ['\u{e2d54}','\u{843d7}','\u{e416d}','\u{2c766}','\u{b6261}','\u{ce95}','\u{a0a53}'];
_3.0 = ['\u{ec67f}','\u{7fe23}','\u{8fe30}','\u{8b349}','\u{8654d}','\u{5251a}','\u{f50a2}','\u{4b5a4}'];
_12 = _2 < _2;
_6 = _5;
_8 = _2;
_7 = _5;
_15.0 = !_1;
_16.0 = !1208833766045014771_u64;
Goto(bb4)
}
bb4 = {
_15 = (_12,);
_16.0 = _13 as u64;
_3 = (_4.0,);
_2 = _8;
_13 = -(-4414449865231895273_i64);
_5 = _7;
(*_9) = ['\u{a5f4}','\u{6d6b0}','\u{32f3b}','\u{655b3}','\u{6dad}','\u{b5f60}','\u{30eb5}'];
_5 = _6;
Goto(bb5)
}
bb5 = {
_4 = _3;
_13 = (-9055935448750420862_i64);
_3 = (_4.0,);
_17 = ['\u{13bcc}','\u{3d462}','\u{10602e}','\u{acad0}','\u{fccf1}'];
_8 = -_2;
_16 = (17457664369090320597_u64,);
RET = ['\u{9f124}','\u{3314d}','\u{3f12f}','\u{e125c}','\u{d9370}','\u{5dad5}','\u{7183}'];
_18 = 76_isize;
_4.0 = ['\u{466e6}','\u{e5c04}','\u{38df3}','\u{1b42e}','\u{4f3c}','\u{ab95e}','\u{11882}','\u{b868e}'];
_15.0 = _1;
_16.0 = !10340006261443682478_u64;
RET = ['\u{77935}','\u{72b0}','\u{4e9c0}','\u{d122f}','\u{1f8e}','\u{a02a6}','\u{36eea}'];
_7 = [_12,_12,_12,_12,_1];
_10 = [150845554552318528562218091656454556663_u128];
_2 = _8;
_16 = (1449187811471139928_u64,);
_12 = _1;
_15.0 = _1;
_12 = _8 < _8;
Call(_9 = fn17(_17, _18, _12, _7, (*_9), _3.0, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = [167626746321409510485408769959903919126_u128];
_3 = (_4.0,);
_20 = !95667967560111166371323752451161105432_u128;
_4 = (_3.0,);
_14 = 62801_u16;
_15.0 = _12;
_22 = [_20];
_4.0 = ['\u{57bb}','\u{10e128}','\u{16695}','\u{dfdf3}','\u{10c103}','\u{d50d4}','\u{c0e7}','\u{328d2}'];
Goto(bb7)
}
bb7 = {
_10 = [_20];
_18 = _13 as isize;
_2 = _8 * _8;
_4.0 = ['\u{244ca}','\u{d149b}','\u{8711b}','\u{9df22}','\u{c2719}','\u{bc9b8}','\u{d1576}','\u{d2149}'];
_17 = ['\u{a4fe1}','\u{63df2}','\u{5df62}','\u{10e147}','\u{f8b03}'];
RET = _11;
_11 = ['\u{8311}','\u{2a767}','\u{78d59}','\u{28ab5}','\u{8ec3c}','\u{bb558}','\u{23874}'];
_20 = 19729238729238414344638101643536320461_u128 << _14;
_12 = _15.0 | _15.0;
_15.0 = _12 & _12;
_14 = 48513_u16 - 34045_u16;
_25 = core::ptr::addr_of!(_20);
_12 = _15.0 & _1;
_4.0 = _3.0;
_7 = [_15.0,_15.0,_15.0,_15.0,_12];
_17 = ['\u{a2191}','\u{82f42}','\u{a8d21}','\u{5aa6a}','\u{639b}'];
match _16.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
1449187811471139928 => bb10,
_ => bb9
}
}
bb8 = {
_3.0 = _4.0;
_5 = [_1,_1,_1,_1,_1];
_8 = _2 + _2;
_12 = _1;
_13 = !953444221242716433_i64;
_4 = _3;
Goto(bb3)
}
bb9 = {
_4 = _3;
_13 = (-9055935448750420862_i64);
_3 = (_4.0,);
_17 = ['\u{13bcc}','\u{3d462}','\u{10602e}','\u{acad0}','\u{fccf1}'];
_8 = -_2;
_16 = (17457664369090320597_u64,);
RET = ['\u{9f124}','\u{3314d}','\u{3f12f}','\u{e125c}','\u{d9370}','\u{5dad5}','\u{7183}'];
_18 = 76_isize;
_4.0 = ['\u{466e6}','\u{e5c04}','\u{38df3}','\u{1b42e}','\u{4f3c}','\u{ab95e}','\u{11882}','\u{b868e}'];
_15.0 = _1;
_16.0 = !10340006261443682478_u64;
RET = ['\u{77935}','\u{72b0}','\u{4e9c0}','\u{d122f}','\u{1f8e}','\u{a02a6}','\u{36eea}'];
_7 = [_12,_12,_12,_12,_1];
_10 = [150845554552318528562218091656454556663_u128];
_2 = _8;
_16 = (1449187811471139928_u64,);
_12 = _1;
_15.0 = _1;
_12 = _8 < _8;
Call(_9 = fn17(_17, _18, _12, _7, (*_9), _3.0, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
RET = ['\u{81f94}','\u{155e0}','\u{2cde3}','\u{a88e4}','\u{69946}','\u{3ab4c}','\u{ebbca}'];
_12 = !_15.0;
_23 = _2;
_5 = [_15.0,_12,_12,_12,_15.0];
match _16.0 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
1449187811471139928 => bb18,
_ => bb17
}
}
bb11 = {
_4 = _3;
_13 = (-9055935448750420862_i64);
_3 = (_4.0,);
_17 = ['\u{13bcc}','\u{3d462}','\u{10602e}','\u{acad0}','\u{fccf1}'];
_8 = -_2;
_16 = (17457664369090320597_u64,);
RET = ['\u{9f124}','\u{3314d}','\u{3f12f}','\u{e125c}','\u{d9370}','\u{5dad5}','\u{7183}'];
_18 = 76_isize;
_4.0 = ['\u{466e6}','\u{e5c04}','\u{38df3}','\u{1b42e}','\u{4f3c}','\u{ab95e}','\u{11882}','\u{b868e}'];
_15.0 = _1;
_16.0 = !10340006261443682478_u64;
RET = ['\u{77935}','\u{72b0}','\u{4e9c0}','\u{d122f}','\u{1f8e}','\u{a02a6}','\u{36eea}'];
_7 = [_12,_12,_12,_12,_1];
_10 = [150845554552318528562218091656454556663_u128];
_2 = _8;
_16 = (1449187811471139928_u64,);
_12 = _1;
_15.0 = _1;
_12 = _8 < _8;
Call(_9 = fn17(_17, _18, _12, _7, (*_9), _3.0, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_3.0 = _4.0;
_5 = [_1,_1,_1,_1,_1];
_8 = _2 + _2;
_12 = _1;
_13 = !953444221242716433_i64;
_4 = _3;
Goto(bb3)
}
bb13 = {
_10 = [_20];
_18 = _13 as isize;
_2 = _8 * _8;
_4.0 = ['\u{244ca}','\u{d149b}','\u{8711b}','\u{9df22}','\u{c2719}','\u{bc9b8}','\u{d1576}','\u{d2149}'];
_17 = ['\u{a4fe1}','\u{63df2}','\u{5df62}','\u{10e147}','\u{f8b03}'];
RET = _11;
_11 = ['\u{8311}','\u{2a767}','\u{78d59}','\u{28ab5}','\u{8ec3c}','\u{bb558}','\u{23874}'];
_20 = 19729238729238414344638101643536320461_u128 << _14;
_12 = _15.0 | _15.0;
_15.0 = _12 & _12;
_14 = 48513_u16 - 34045_u16;
_25 = core::ptr::addr_of!(_20);
_12 = _15.0 & _1;
_4.0 = _3.0;
_7 = [_15.0,_15.0,_15.0,_15.0,_12];
_17 = ['\u{a2191}','\u{82f42}','\u{a8d21}','\u{5aa6a}','\u{639b}'];
match _16.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
1449187811471139928 => bb10,
_ => bb9
}
}
bb14 = {
_10 = [167626746321409510485408769959903919126_u128];
_3 = (_4.0,);
_20 = !95667967560111166371323752451161105432_u128;
_4 = (_3.0,);
_14 = 62801_u16;
_15.0 = _12;
_22 = [_20];
_4.0 = ['\u{57bb}','\u{10e128}','\u{16695}','\u{dfdf3}','\u{10c103}','\u{d50d4}','\u{c0e7}','\u{328d2}'];
Goto(bb7)
}
bb15 = {
_4 = _3;
_13 = (-9055935448750420862_i64);
_3 = (_4.0,);
_17 = ['\u{13bcc}','\u{3d462}','\u{10602e}','\u{acad0}','\u{fccf1}'];
_8 = -_2;
_16 = (17457664369090320597_u64,);
RET = ['\u{9f124}','\u{3314d}','\u{3f12f}','\u{e125c}','\u{d9370}','\u{5dad5}','\u{7183}'];
_18 = 76_isize;
_4.0 = ['\u{466e6}','\u{e5c04}','\u{38df3}','\u{1b42e}','\u{4f3c}','\u{ab95e}','\u{11882}','\u{b868e}'];
_15.0 = _1;
_16.0 = !10340006261443682478_u64;
RET = ['\u{77935}','\u{72b0}','\u{4e9c0}','\u{d122f}','\u{1f8e}','\u{a02a6}','\u{36eea}'];
_7 = [_12,_12,_12,_12,_1];
_10 = [150845554552318528562218091656454556663_u128];
_2 = _8;
_16 = (1449187811471139928_u64,);
_12 = _1;
_15.0 = _1;
_12 = _8 < _8;
Call(_9 = fn17(_17, _18, _12, _7, (*_9), _3.0, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
_15 = (_12,);
_16.0 = _13 as u64;
_3 = (_4.0,);
_2 = _8;
_13 = -(-4414449865231895273_i64);
_5 = _7;
(*_9) = ['\u{a5f4}','\u{6d6b0}','\u{32f3b}','\u{655b3}','\u{6dad}','\u{b5f60}','\u{30eb5}'];
_5 = _6;
Goto(bb5)
}
bb17 = {
_5 = [_12,_12,_1,_12,_12];
_3.0 = ['\u{5fe40}','\u{a6936}','\u{ecb5d}','\u{66582}','\u{79b17}','\u{86228}','\u{9c779}','\u{c5827}'];
_6 = [_1,_12,_12,_12,_12];
_8 = 129694045715764029089683551489966977555_i128 as f32;
_11 = ['\u{73983}','\u{5bd83}','\u{24236}','\u{ef003}','\u{c04ba}','\u{381c}','\u{6c522}'];
_3 = _4;
_7 = [_1,_12,_12,_12,_1];
RET = ['\u{d022a}','\u{1aa6c}','\u{23527}','\u{156c}','\u{6d52f}','\u{e93ae}','\u{fd8bd}'];
RET = ['\u{d165c}','\u{bed1a}','\u{11c67}','\u{10b876}','\u{1fe47}','\u{4e1e6}','\u{b6389}'];
(*_9) = ['\u{e2d54}','\u{843d7}','\u{e416d}','\u{2c766}','\u{b6261}','\u{ce95}','\u{a0a53}'];
_3.0 = ['\u{ec67f}','\u{7fe23}','\u{8fe30}','\u{8b349}','\u{8654d}','\u{5251a}','\u{f50a2}','\u{4b5a4}'];
_12 = _2 < _2;
_6 = _5;
_8 = _2;
_7 = _5;
_15.0 = !_1;
_16.0 = !1208833766045014771_u64;
Goto(bb4)
}
bb18 = {
_23 = _18 as f32;
_15 = (_12,);
_15 = (_12,);
_2 = _23 * _23;
_14 = 34879_u16 ^ 62056_u16;
_16 = (13221670682667220346_u64,);
_10 = [(*_25)];
_29 = !(-21112_i16);
RET = _11;
_9 = core::ptr::addr_of_mut!(_11);
RET = ['\u{8fae0}','\u{e2fe2}','\u{f5ee5}','\u{ed613}','\u{cf409}','\u{231bf}','\u{b0c3}'];
_29 = 16226_i16;
_4.0 = ['\u{305cb}','\u{420c1}','\u{83bb5}','\u{aaff6}','\u{742f4}','\u{da31d}','\u{76481}','\u{3acef}'];
_30 = Adt42::Variant0 { fld0: _12 };
_15 = (Field::<bool>(Variant(_30, 0), 0),);
(*_25) = 84248042086859996688398058067165236360_u128;
_31 = _29 as f64;
_3 = (_4.0,);
Goto(bb19)
}
bb19 = {
Call(_32 = dump_var(16_usize, 17_usize, Move(_17), 5_usize, Move(_5), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(16_usize, 20_usize, Move(_20), 14_usize, Move(_14), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(16_usize, 13_usize, Move(_13), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [char; 5],mut _2: isize,mut _3: bool,mut _4: [bool; 5],mut _5: [char; 7],mut _6: [char; 8],mut _7: bool) -> *mut [char; 7] {
mir! {
type RET = *mut [char; 7];
let _8: bool;
let _9: f32;
let _10: usize;
let _11: (u64,);
let _12: i8;
let _13: Adt46;
let _14: isize;
let _15: isize;
let _16: Adt44;
let _17: ();
let _18: ();
{
_3 = _7 <= _7;
_1 = ['\u{10481a}','\u{cf589}','\u{19e42}','\u{f7bad}','\u{9e870}'];
_3 = _7;
match _2 {
0 => bb1,
1 => bb2,
76 => bb4,
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
RET = core::ptr::addr_of_mut!(_5);
_2 = !9223372036854775807_isize;
_1 = ['\u{4f01e}','\u{8a8f}','\u{97057}','\u{86b9c}','\u{641f1}'];
RET = core::ptr::addr_of_mut!(_5);
_6 = ['\u{a65ff}','\u{93ddd}','\u{168e9}','\u{a8ac1}','\u{22cfe}','\u{54320}','\u{a617a}','\u{1f828}'];
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = ['\u{f90ed}','\u{54ed5}','\u{10dc8e}','\u{106f2e}','\u{d96e7}','\u{fd3ec}','\u{efded}'];
_5 = ['\u{66ced}','\u{102687}','\u{4a747}','\u{aefd8}','\u{acb4d}','\u{f56d2}','\u{8d7e8}'];
_2 = 1715827150_u32 as isize;
_6 = ['\u{1e995}','\u{7a1ba}','\u{a57a8}','\u{10aad}','\u{76870}','\u{7e4c5}','\u{354ef}','\u{1a9ba}'];
(*RET) = ['\u{80cc7}','\u{b7a78}','\u{e9a5c}','\u{b61b7}','\u{3c987}','\u{61ea}','\u{6cc9f}'];
(*RET) = ['\u{7524d}','\u{68d39}','\u{e999c}','\u{dcb6d}','\u{407e6}','\u{10ed69}','\u{cb57f}'];
_5 = ['\u{f39b6}','\u{61667}','\u{cd7d6}','\u{f4375}','\u{c3e04}','\u{8c83}','\u{f87e6}'];
_8 = !_7;
(*RET) = ['\u{a9a4f}','\u{3a5fc}','\u{fbe36}','\u{2cc77}','\u{24736}','\u{3cded}','\u{3f1f1}'];
(*RET) = ['\u{191cd}','\u{ecf9d}','\u{93828}','\u{103bf}','\u{caf99}','\u{98c59}','\u{20be6}'];
Goto(bb5)
}
bb5 = {
_6 = ['\u{d2b5d}','\u{109109}','\u{e48ed}','\u{4e51b}','\u{4cf69}','\u{6ae2e}','\u{97645}','\u{bab97}'];
_3 = _8;
_1 = ['\u{dfab7}','\u{8647e}','\u{70555}','\u{93a6b}','\u{3d5a3}'];
_1 = ['\u{d6847}','\u{1ef92}','\u{67cd4}','\u{c9bf5}','\u{5fe2c}'];
Goto(bb6)
}
bb6 = {
_3 = _8;
RET = core::ptr::addr_of_mut!((*RET));
_7 = _3 == _8;
RET = core::ptr::addr_of_mut!(_5);
_1 = ['\u{2dad2}','\u{60208}','\u{b07aa}','\u{7e840}','\u{10a4a3}'];
(*RET) = ['\u{1021cd}','\u{cae99}','\u{c2e07}','\u{b1646}','\u{bfab2}','\u{fbdd4}','\u{ae272}'];
RET = core::ptr::addr_of_mut!((*RET));
_4 = [_3,_7,_8,_7,_8];
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = ['\u{38aa4}','\u{47b2c}','\u{6db68}','\u{10d359}','\u{11a2e}','\u{10ad35}','\u{21d7c}'];
(*RET) = ['\u{130d2}','\u{db700}','\u{17643}','\u{6ffd5}','\u{a7267}','\u{10f88e}','\u{13f4e}'];
(*RET) = ['\u{c790f}','\u{f881d}','\u{3aeb5}','\u{10b40f}','\u{87887}','\u{35574}','\u{e20ca}'];
RET = core::ptr::addr_of_mut!((*RET));
_3 = !_8;
_6 = ['\u{4f591}','\u{ad1eb}','\u{bb692}','\u{9ecfb}','\u{59901}','\u{37f69}','\u{a3dba}','\u{3e8bd}'];
_10 = !2_usize;
(*RET) = ['\u{60c77}','\u{2bbe1}','\u{345c1}','\u{261cc}','\u{cbca8}','\u{ca84}','\u{9ccf}'];
_11.0 = 10555190653488548324_u64;
(*RET) = ['\u{8c0da}','\u{9ba08}','\u{44346}','\u{aa7c6}','\u{cf2a3}','\u{1eea8}','\u{a45b3}'];
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = ['\u{af695}','\u{62d08}','\u{254d}','\u{ed6db}','\u{a86c9}','\u{22c5c}','\u{95e95}'];
match _11.0 {
0 => bb1,
1 => bb5,
2 => bb7,
10555190653488548324 => bb9,
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
(*RET) = ['\u{aef81}','\u{d0b5e}','\u{e5671}','\u{5cbf6}','\u{3c582}','\u{d09be}','\u{a3868}'];
Goto(bb10)
}
bb10 = {
_11 = (15046902470045329336_u64,);
(*RET) = ['\u{55dcd}','\u{c78c5}','\u{9d990}','\u{6ab79}','\u{80fbb}','\u{475fe}','\u{48935}'];
(*RET) = ['\u{5b023}','\u{10a695}','\u{3da2f}','\u{72f02}','\u{73058}','\u{175ff}','\u{3483b}'];
(*RET) = ['\u{3f05b}','\u{f6a5}','\u{22f06}','\u{39a49}','\u{9a353}','\u{5e4ea}','\u{10040f}'];
_4 = [_8,_8,_7,_3,_3];
(*RET) = ['\u{e1c44}','\u{5637a}','\u{10b072}','\u{a52fd}','\u{72766}','\u{39524}','\u{e12b9}'];
_8 = _7;
_7 = !_3;
(*RET) = ['\u{5fa2e}','\u{4e738}','\u{5b429}','\u{a257c}','\u{b0d07}','\u{9d744}','\u{a981f}'];
(*RET) = ['\u{ce1c5}','\u{5c83f}','\u{d0ed3}','\u{cd5ed}','\u{720d4}','\u{407}','\u{86977}'];
match _11.0 {
0 => bb6,
1 => bb2,
2 => bb11,
15046902470045329336 => bb13,
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
_9 = (-29187_i16) as f32;
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!(_5);
_11.0 = (-1439092466_i32) as u64;
(*RET) = ['\u{8d33c}','\u{9fde5}','\u{395b3}','\u{b91c}','\u{231eb}','\u{2028e}','\u{448b1}'];
_3 = _8 <= _7;
(*RET) = ['\u{a5f7e}','\u{af814}','\u{789f4}','\u{2f5c1}','\u{38f05}','\u{50bb7}','\u{e06e8}'];
_11 = (6543486820859593191_u64,);
_10 = 330986777328304041136612659749318959129_u128 as usize;
_14 = !_2;
RET = core::ptr::addr_of_mut!((*RET));
_5 = ['\u{3bcf0}','\u{6fce4}','\u{49849}','\u{9fd87}','\u{8b74b}','\u{12ff7}','\u{f88bf}'];
_12 = 94_i8;
_7 = !_3;
_11.0 = '\u{cb42}' as u64;
_7 = _3;
_11 = (15378228436243287265_u64,);
_8 = _3;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb6,
94 => bb14,
_ => bb7
}
}
bb14 = {
_12 = 1_i8;
_15 = 23024_i16 as isize;
_11.0 = 42624_u16 as u64;
_11 = (13083597099733810658_u64,);
(*RET) = ['\u{10e74b}','\u{b6449}','\u{a0cd5}','\u{bc492}','\u{fb1f6}','\u{1a997}','\u{cb94}'];
_3 = _7 ^ _8;
(*RET) = ['\u{27be4}','\u{33518}','\u{e95a5}','\u{10876d}','\u{cabee}','\u{a9d8e}','\u{102dde}'];
(*RET) = ['\u{6934d}','\u{864fe}','\u{a223d}','\u{b688c}','\u{44b47}','\u{e8bee}','\u{e333d}'];
_15 = _2;
(*RET) = ['\u{2efc7}','\u{d050}','\u{d73ab}','\u{8b03d}','\u{3f0e7}','\u{104157}','\u{45e91}'];
_8 = _7;
_9 = _15 as f32;
_1 = ['\u{7051a}','\u{a84df}','\u{9a54c}','\u{f4076}','\u{9e6aa}'];
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(17_usize, 10_usize, Move(_10), 7_usize, Move(_7), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_17 = dump_var(17_usize, 5_usize, Move(_5), 6_usize, Move(_6), 18_usize, _18, 18_usize, _18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u64,mut _2: isize,mut _3: [bool; 5],mut _4: [bool; 5],mut _5: bool,mut _6: isize,mut _7: Adt45,mut _8: [u8; 6]) -> [u8; 6] {
mir! {
type RET = [u8; 6];
let _9: (u32, char, char);
let _10: [u128; 1];
let _11: isize;
let _12: u64;
let _13: i16;
let _14: i128;
let _15: [char; 5];
let _16: Adt53;
let _17: u8;
let _18: [bool; 5];
let _19: *const i16;
let _20: i32;
let _21: ();
let _22: ();
{
_7 = Adt45 { fld0: _6,fld1: 31716_u16 };
_4 = _3;
RET = [8_u8,59_u8,23_u8,8_u8,195_u8,127_u8];
_7.fld1 = !33935_u16;
_7 = Adt45 { fld0: _6,fld1: 49458_u16 };
_7.fld0 = _5 as isize;
_7 = Adt45 { fld0: _6,fld1: 2949_u16 };
RET = _8;
_7.fld0 = (-103242170117448829929443334292020000250_i128) as isize;
_7.fld0 = _6 >> _2;
_7 = Adt45 { fld0: _6,fld1: 54083_u16 };
_8 = RET;
_7.fld1 = 39013_u16 << _1;
_6 = _2;
_4 = [_5,_5,_5,_5,_5];
_5 = _6 < _2;
_8 = [41_u8,163_u8,5_u8,227_u8,67_u8,150_u8];
_9.1 = '\u{6ac43}';
_9.2 = _9.1;
_5 = !false;
_9 = (2747613811_u32, '\u{cdecd}', '\u{80747}');
_4 = [_5,_5,_5,_5,_5];
_1 = !1270662614809292904_u64;
_3 = [_5,_5,_5,_5,_5];
_10 = [283692039373459495108595543002751761789_u128];
RET = [207_u8,185_u8,30_u8,212_u8,135_u8,64_u8];
_6 = _2 << _7.fld1;
_11 = !_7.fld0;
_9.1 = _9.2;
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
2747613811 => bb5,
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
_12 = _1;
_9.1 = _9.2;
_11 = _5 as isize;
_14 = -(-14545773186929410233560410325297666676_i128);
_14 = (-49618095611922067883774359639916478455_i128);
_7 = Adt45 { fld0: _6,fld1: 53895_u16 };
_9.0 = 2498697207_u32 << _6;
Goto(bb6)
}
bb6 = {
_1 = _12 >> _6;
_13 = 199_u8 as i16;
_10 = [296544372919505586728158504221177705094_u128];
_3 = [_5,_5,_5,_5,_5];
_13 = !(-15894_i16);
_6 = _7.fld0;
RET = [131_u8,74_u8,186_u8,21_u8,60_u8,171_u8];
_9.0 = !2085204642_u32;
RET = [150_u8,255_u8,211_u8,5_u8,184_u8,191_u8];
_10 = [292145467997581768744299835617241805702_u128];
_3 = _4;
_10 = [147303617861401922959536062717246461605_u128];
_9.0 = !3266873288_u32;
match _7.fld1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
53895 => bb13,
_ => bb12
}
}
bb7 = {
_12 = _1;
_9.1 = _9.2;
_11 = _5 as isize;
_14 = -(-14545773186929410233560410325297666676_i128);
_14 = (-49618095611922067883774359639916478455_i128);
_7 = Adt45 { fld0: _6,fld1: 53895_u16 };
_9.0 = 2498697207_u32 << _6;
Goto(bb6)
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
Return()
}
bb13 = {
_9.0 = 3_i8 as u32;
_15 = [_9.2,_9.2,_9.1,_9.2,_9.2];
RET = [91_u8,107_u8,235_u8,59_u8,71_u8,158_u8];
_9.0 = 2338576523_u32;
_3 = [_5,_5,_5,_5,_5];
_7.fld0 = !_2;
RET = [204_u8,170_u8,76_u8,228_u8,131_u8,101_u8];
_7.fld0 = _6;
_8 = RET;
_1 = !_12;
_3 = _4;
_8 = [52_u8,72_u8,43_u8,127_u8,101_u8,195_u8];
_7.fld0 = _6 >> _2;
_1 = 301422879825019769551627015724866256825_u128 as u64;
_9.1 = _9.2;
Goto(bb14)
}
bb14 = {
_15 = [_9.1,_9.1,_9.1,_9.2,_9.1];
_18 = _3;
_14 = (-128_i8) as i128;
_7 = Adt45 { fld0: _6,fld1: 28740_u16 };
_17 = 253_u8 + 32_u8;
_20 = !(-434181822_i32);
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(18_usize, 1_usize, Move(_1), 17_usize, Move(_17), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(18_usize, 4_usize, Move(_4), 13_usize, Move(_13), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: *const i16,mut _2: i16,mut _3: i16,mut _4: *const i16,mut _5: i16,mut _6: *const i16,mut _7: i16) -> *const u8 {
mir! {
type RET = *const u8;
let _8: u8;
let _9: isize;
let _10: [u128; 1];
let _11: Adt45;
let _12: (&'static [u8; 6], f32, *const i32, u64);
let _13: [char; 5];
let _14: char;
let _15: Adt45;
let _16: ([char; 8],);
let _17: [char; 8];
let _18: *const (u8, i16, u16);
let _19: [char; 7];
let _20: ([char; 8],);
let _21: u32;
let _22: Adt51;
let _23: (u8, i16, u16);
let _24: *const (u8, i16, u16);
let _25: i64;
let _26: *const (u8, i16, u16);
let _27: Adt48;
let _28: Adt47;
let _29: (bool,);
let _30: (u32, char, char);
let _31: *const u128;
let _32: ();
let _33: ();
{
(*_1) = _3;
_7 = _3 * (*_4);
_1 = core::ptr::addr_of!((*_1));
_4 = _6;
_4 = _1;
(*_6) = !_2;
(*_1) = 3613924284002942305_usize as i16;
(*_6) = _2 >> _5;
(*_6) = '\u{a6560}' as i16;
_3 = _5;
(*_6) = _2;
RET = core::ptr::addr_of!(_8);
(*RET) = 156_u8 ^ 171_u8;
(*_1) = 4934129935266366592_u64 as i16;
(*_4) = -_2;
(*_4) = _3 + _2;
_8 = 223_u8;
(*_4) = -_2;
(*RET) = !242_u8;
_9 = 9223372036854775807_isize;
(*_6) = !_2;
_5 = _7 ^ (*_1);
(*RET) = 10972097272190782561_usize as u8;
(*RET) = !89_u8;
_9 = 9223372036854775807_isize;
_8 = !67_u8;
_9 = 29_i8 as isize;
_11.fld0 = _9;
Goto(bb1)
}
bb1 = {
_9 = _2 as isize;
RET = core::ptr::addr_of!((*RET));
_10 = [251602570904752594407770035165300643032_u128];
(*_6) = -_7;
_4 = core::ptr::addr_of!((*_6));
(*RET) = 4002439770567112224_usize as u8;
(*_1) = -_7;
(*RET) = 157_u8;
_7 = _2;
_12.3 = !4048358466331439084_u64;
_1 = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!(_8);
(*_6) = _2;
RET = core::ptr::addr_of!((*RET));
_9 = (-4471557141984915987_i64) as isize;
_3 = (*_6);
_13 = ['\u{29a27}','\u{ed08e}','\u{107199}','\u{630d0}','\u{49520}'];
(*_4) = _3 & _5;
_2 = (*_6) & (*_4);
(*_1) = !_2;
(*RET) = 94_u8 * 3_u8;
_6 = _4;
(*_4) = !_3;
_11.fld1 = 4_usize as u16;
_7 = _3;
_12.1 = _11.fld1 as f32;
Goto(bb2)
}
bb2 = {
(*_6) = (*_1) + _5;
RET = core::ptr::addr_of!(_8);
Goto(bb3)
}
bb3 = {
(*RET) = 121_u8;
_11.fld0 = !_9;
_3 = (*_1);
(*_1) = -(*_6);
(*RET) = _12.3 as u8;
_13 = ['\u{57c71}','\u{e70fe}','\u{dc31f}','\u{10325b}','\u{d7e53}'];
(*_4) = 118356618934737776925330910828438614827_i128 as i16;
(*_4) = _11.fld1 as i16;
_5 = _2;
_11 = Adt45 { fld0: _9,fld1: 53037_u16 };
_15.fld1 = _7 as u16;
_15.fld0 = _9;
(*_1) = _3;
(*RET) = 187_u8 & 247_u8;
match _11.fld1 {
0 => bb1,
1 => bb2,
2 => bb4,
53037 => bb6,
_ => bb5
}
}
bb4 = {
(*_6) = (*_1) + _5;
RET = core::ptr::addr_of!(_8);
Goto(bb3)
}
bb5 = {
_9 = _2 as isize;
RET = core::ptr::addr_of!((*RET));
_10 = [251602570904752594407770035165300643032_u128];
(*_6) = -_7;
_4 = core::ptr::addr_of!((*_6));
(*RET) = 4002439770567112224_usize as u8;
(*_1) = -_7;
(*RET) = 157_u8;
_7 = _2;
_12.3 = !4048358466331439084_u64;
_1 = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!(_8);
(*_6) = _2;
RET = core::ptr::addr_of!((*RET));
_9 = (-4471557141984915987_i64) as isize;
_3 = (*_6);
_13 = ['\u{29a27}','\u{ed08e}','\u{107199}','\u{630d0}','\u{49520}'];
(*_4) = _3 & _5;
_2 = (*_6) & (*_4);
(*_1) = !_2;
(*RET) = 94_u8 * 3_u8;
_6 = _4;
(*_4) = !_3;
_11.fld1 = 4_usize as u16;
_7 = _3;
_12.1 = _11.fld1 as f32;
Goto(bb2)
}
bb6 = {
_4 = core::ptr::addr_of!(_7);
(*_4) = -_2;
_17 = ['\u{7b53d}','\u{7081a}','\u{87625}','\u{1a87}','\u{3ba3e}','\u{9dce3}','\u{2160c}','\u{f08e}'];
_15 = Adt45 { fld0: _11.fld0,fld1: _11.fld1 };
Goto(bb7)
}
bb7 = {
_15.fld0 = !_9;
(*_6) = -_7;
_15.fld0 = 9893253753073686432_usize as isize;
(*RET) = 86_u8;
_19 = ['\u{4d57d}','\u{5b3cf}','\u{7769e}','\u{7cf13}','\u{996b1}','\u{eadce}','\u{a2c0a}'];
_4 = core::ptr::addr_of!(_5);
(*_1) = -_2;
(*RET) = !242_u8;
_2 = -(*_6);
_20.0 = ['\u{dfce5}','\u{170f2}','\u{bf7fc}','\u{25093}','\u{b67b}','\u{41c98}','\u{671b3}','\u{75f97}'];
_16.0 = ['\u{8edb3}','\u{f0ef8}','\u{2d9bc}','\u{b5547}','\u{897a}','\u{a35a1}','\u{8026d}','\u{2abb3}'];
_16 = (_17,);
_14 = '\u{7614d}';
_4 = _1;
_8 = 216_u8;
_10 = [302166559978411743843628131433134259823_u128];
_23 = (_8, _3, _15.fld1);
_11 = _15;
_23 = (_8, (*_6), _15.fld1);
_23.0 = (*RET);
(*RET) = 15639244280658494401_usize as u8;
_10 = [338910296110572942185960723913480672104_u128];
Goto(bb8)
}
bb8 = {
_6 = core::ptr::addr_of!((*_1));
_8 = 2056883447_u32 as u8;
(*_4) = _2;
_19 = [_14,_14,_14,_14,_14,_14,_14];
_11.fld0 = !_9;
_9 = (-49160996916265547608494663163995916172_i128) as isize;
_13 = [_14,_14,_14,_14,_14];
_6 = core::ptr::addr_of!(_7);
(*_1) = 316128307548621074076645963730147659033_u128 as i16;
(*_1) = !_2;
_25 = (-3539053202277358440_i64) | 7453806996546665985_i64;
match _15.fld1 {
0 => bb5,
1 => bb6,
2 => bb3,
3 => bb7,
53037 => bb10,
_ => bb9
}
}
bb9 = {
(*RET) = 121_u8;
_11.fld0 = !_9;
_3 = (*_1);
(*_1) = -(*_6);
(*RET) = _12.3 as u8;
_13 = ['\u{57c71}','\u{e70fe}','\u{dc31f}','\u{10325b}','\u{d7e53}'];
(*_4) = 118356618934737776925330910828438614827_i128 as i16;
(*_4) = _11.fld1 as i16;
_5 = _2;
_11 = Adt45 { fld0: _9,fld1: 53037_u16 };
_15.fld1 = _7 as u16;
_15.fld0 = _9;
(*_1) = _3;
(*RET) = 187_u8 & 247_u8;
match _11.fld1 {
0 => bb1,
1 => bb2,
2 => bb4,
53037 => bb6,
_ => bb5
}
}
bb10 = {
_21 = (-1862371936_i32) as u32;
(*RET) = _23.0 % _23.0;
(*_1) = _15.fld1 as i16;
_20 = (_16.0,);
(*RET) = _23.0;
_16.0 = [_14,_14,_14,_14,_14,_14,_14,_14];
_13 = [_14,_14,_14,_14,_14];
RET = core::ptr::addr_of!(_8);
_5 = _3 - _23.1;
_25 = _23.2 as i64;
_15.fld1 = _11.fld1 >> _5;
_20 = _16;
_16 = (_17,);
(*_1) = _5;
_4 = _6;
_7 = _2 + _5;
_15.fld0 = _9 & _9;
_2 = _5 + _7;
_23.0 = (*RET);
(*RET) = _23.0;
_11 = _15;
_4 = core::ptr::addr_of!(_2);
(*_6) = (*_4);
RET = core::ptr::addr_of!((*RET));
(*_4) = _3;
match _23.0 {
0 => bb1,
1 => bb3,
2 => bb11,
3 => bb12,
216 => bb14,
_ => bb13
}
}
bb11 = {
_9 = _2 as isize;
RET = core::ptr::addr_of!((*RET));
_10 = [251602570904752594407770035165300643032_u128];
(*_6) = -_7;
_4 = core::ptr::addr_of!((*_6));
(*RET) = 4002439770567112224_usize as u8;
(*_1) = -_7;
(*RET) = 157_u8;
_7 = _2;
_12.3 = !4048358466331439084_u64;
_1 = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!(_8);
(*_6) = _2;
RET = core::ptr::addr_of!((*RET));
_9 = (-4471557141984915987_i64) as isize;
_3 = (*_6);
_13 = ['\u{29a27}','\u{ed08e}','\u{107199}','\u{630d0}','\u{49520}'];
(*_4) = _3 & _5;
_2 = (*_6) & (*_4);
(*_1) = !_2;
(*RET) = 94_u8 * 3_u8;
_6 = _4;
(*_4) = !_3;
_11.fld1 = 4_usize as u16;
_7 = _3;
_12.1 = _11.fld1 as f32;
Goto(bb2)
}
bb12 = {
_9 = _2 as isize;
RET = core::ptr::addr_of!((*RET));
_10 = [251602570904752594407770035165300643032_u128];
(*_6) = -_7;
_4 = core::ptr::addr_of!((*_6));
(*RET) = 4002439770567112224_usize as u8;
(*_1) = -_7;
(*RET) = 157_u8;
_7 = _2;
_12.3 = !4048358466331439084_u64;
_1 = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!(_8);
(*_6) = _2;
RET = core::ptr::addr_of!((*RET));
_9 = (-4471557141984915987_i64) as isize;
_3 = (*_6);
_13 = ['\u{29a27}','\u{ed08e}','\u{107199}','\u{630d0}','\u{49520}'];
(*_4) = _3 & _5;
_2 = (*_6) & (*_4);
(*_1) = !_2;
(*RET) = 94_u8 * 3_u8;
_6 = _4;
(*_4) = !_3;
_11.fld1 = 4_usize as u16;
_7 = _3;
_12.1 = _11.fld1 as f32;
Goto(bb2)
}
bb13 = {
(*RET) = 121_u8;
_11.fld0 = !_9;
_3 = (*_1);
(*_1) = -(*_6);
(*RET) = _12.3 as u8;
_13 = ['\u{57c71}','\u{e70fe}','\u{dc31f}','\u{10325b}','\u{d7e53}'];
(*_4) = 118356618934737776925330910828438614827_i128 as i16;
(*_4) = _11.fld1 as i16;
_5 = _2;
_11 = Adt45 { fld0: _9,fld1: 53037_u16 };
_15.fld1 = _7 as u16;
_15.fld0 = _9;
(*_1) = _3;
(*RET) = 187_u8 & 247_u8;
match _11.fld1 {
0 => bb1,
1 => bb2,
2 => bb4,
53037 => bb6,
_ => bb5
}
}
bb14 = {
_11.fld1 = !_15.fld1;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(19_usize, 13_usize, Move(_13), 8_usize, Move(_8), 17_usize, Move(_17), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(19_usize, 5_usize, Move(_5), 7_usize, Move(_7), 9_usize, Move(_9), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{282c0}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(110_i8), std::hint::black_box(30435_i16), std::hint::black_box((-1607345575_i32)), std::hint::black_box((-9066490284755714045_i64)), std::hint::black_box((-99859033837894611582432412134708443729_i128)), std::hint::black_box(3_usize), std::hint::black_box(188_u8), std::hint::black_box(482_u16), std::hint::black_box(3696730436_u32), std::hint::black_box(14446107856435788844_u64), std::hint::black_box(308413472012588894426230231541286077270_u128));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: f64,
fld1: ([char; 8],),

},
Variant1{
fld0: u16,
fld1: u32,

},
Variant2{
fld0: u64,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,

},
Variant1{
fld0: f64,
fld1: [char; 8],
fld2: isize,
fld3: [u8; 6],
fld4: (u8, i16, u16),
fld5: i32,
fld6: *const u8,
fld7: i128,

},
Variant2{
fld0: i16,
fld1: char,
fld2: (bool,),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [char; 8],
fld1: [char; 7],
fld2: i128,
fld3: (bool,),

},
Variant1{
fld0: i32,
fld1: usize,
fld2: [char; 5],
fld3: u128,

},
Variant2{
fld0: ([char; 8],),
fld1: *const u128,
fld2: (u8, i16, u16),

},
Variant3{
fld0: usize,
fld1: [char; 5],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
fld0: Adt42,
fld1: i64,
fld2: *mut f32,

},
Variant1{
fld0: *const (u8, i16, u16),
fld1: *mut f32,
fld2: (u64,),
fld3: f32,
fld4: [u8; 6],
fld5: [char; 5],
fld6: *mut i64,

},
Variant2{
fld0: u64,
fld1: [u128; 5],
fld2: (u32, char, char),

},
Variant3{
fld0: usize,
fld1: f64,
fld2: *const u128,
fld3: [char; 5],
fld4: i16,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: isize,
fld1: u16,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u64,
fld1: char,
fld2: [bool; 5],
fld3: *const u8,

},
Variant1{
fld0: Adt45,
fld1: u16,
fld2: (bool,),
fld3: (u64,),
fld4: *const u8,
fld5: i32,

},
Variant2{
fld0: bool,
fld1: char,
fld2: f64,
fld3: [u8; 6],
fld4: Adt44,
fld5: *mut [char; 7],
fld6: [char; 7],
fld7: i128,

},
Variant3{
fld0: *mut f32,
fld1: [u8; 6],
fld2: f64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt42,
fld1: [u128; 5],
fld2: *const (u8, i16, u16),
fld3: ([char; 8],),

},
Variant1{
fld0: f64,
fld1: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16)),
fld2: (u64,),
fld3: f32,
fld4: ([char; 8],),

},
Variant2{
fld0: u128,
fld1: [char; 7],
fld2: *const u8,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: bool,
fld1: Adt46,
fld2: *mut f32,

},
Variant1{
fld0: Adt46,
fld1: *mut [char; 7],
fld2: *const (u8, i16, u16),
fld3: *const u128,
fld4: [char; 7],
fld5: *mut f32,
fld6: (u32, char, char),
fld7: u8,

},
Variant2{
fld0: f64,
fld1: u128,
fld2: isize,
fld3: Adt43,
fld4: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16)),
fld5: *const u128,
fld6: u32,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt46,
fld1: Adt44,
fld2: *mut f32,

},
Variant1{
fld0: (u8, i16, u16),
fld1: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16)),
fld2: Adt41,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [char; 7],
fld1: (bool,),
fld2: (u32, char, char),
fld3: u128,
fld4: *mut f32,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u8,
fld1: Adt45,
fld2: u16,
fld3: Adt48,
fld4: [char; 7],
fld5: i32,

},
Variant1{
fld0: Adt42,
fld1: *mut [char; 7],
fld2: usize,
fld3: (u32, char, char),
fld4: i16,
fld5: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16)),
fld6: *const i32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const i32,
fld1: Adt47,
fld2: *mut i64,

},
Variant1{
fld0: Adt41,
fld1: char,
fld2: *mut i64,
fld3: ([char; 8],),
fld4: *const i16,

},
Variant2{
fld0: [u128; 1],
fld1: f32,
fld2: f64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *const u128,
fld1: char,
fld2: *mut f32,
fld3: u32,
fld4: Adt43,
fld5: *const (u8, i16, u16),
fld6: (u8, i16, u16),
fld7: *const i32,

},
Variant1{
fld0: bool,
fld1: *const i16,
fld2: [char; 5],
fld3: i8,
fld4: (u8, i16, u16),
fld5: *const (u8, i16, u16),
fld6: u64,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: u128,
fld1: i32,
fld2: u16,
fld3: Adt49,
fld4: *const u128,

},
Variant1{
fld0: f32,
fld1: [char; 7],

},
Variant2{
fld0: Adt48,
fld1: [char; 5],
fld2: [u128; 1],
fld3: (u64,),
fld4: usize,
fld5: i32,
fld6: Adt52,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: *const i16,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: [char; 8],
fld1: *const u128,
fld2: u32,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt43,
fld1: char,
fld2: *mut [char; 7],
fld3: *const i32,
fld4: *const u128,
fld5: [char; 5],
fld6: f32,
fld7: [u128; 1],

},
Variant1{
fld0: [u128; 5],
fld1: *mut f32,
fld2: *const u128,
fld3: Adt54,
fld4: i16,
fld5: (u32, char, char),
fld6: u16,

},
Variant2{
fld0: Adt41,
fld1: Adt49,
fld2: isize,
fld3: *mut f32,
fld4: [u128; 1],
fld5: (u64,),
fld6: ([char; 5], *mut [char; 7], *mut [char; 7], f32, (u32, char, char), *const (u8, i16, u16)),
fld7: Adt56,

},
Variant3{
fld0: [char; 7],
fld1: Adt48,
fld2: [u128; 5],
fld3: Adt55,
fld4: *const i32,
fld5: u32,
fld6: usize,
fld7: *const i16,

}}

