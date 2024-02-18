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
pub fn fn0(mut _1: u32,mut _2: char,mut _3: isize,mut _4: u16,mut _5: i64,mut _6: u64) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _7: char;
let _8: *mut (usize, u16);
let _9: i128;
let _10: f64;
let _11: isize;
let _12: &'static [i32; 2];
let _13: i16;
let _14: bool;
let _15: [i16; 3];
let _16: i32;
let _17: char;
let _18: (*const ((i64, *const [i16; 5], i8), i8, Adt19), usize, [usize; 1]);
let _19: [i8; 4];
let _20: u64;
let _21: &'static (i64, *const [i16; 5], i8);
let _22: *const [i16; 5];
let _23: (*mut (usize, u16), &'static Adt53, Adt58, (i64, *const [i16; 5], i8));
let _24: bool;
let _25: &'static u8;
let _26: [bool; 2];
let _27: Adt21;
let _28: [bool; 2];
let _29: *const usize;
let _30: i128;
let _31: u64;
let _32: (f32, u8, u64);
let _33: u128;
let _34: i8;
let _35: [u8; 7];
let _36: &'static Adt75;
let _37: u16;
let _38: [u16; 8];
let _39: u8;
let _40: &'static Adt53;
let _41: u8;
let _42: &'static u32;
let _43: *const *const *const usize;
let _44: ();
let _45: ();
{
_6 = (-9223372036854775808_isize) as u64;
_4 = !44046_u16;
RET = [(-97708603949622217487136609815227931692_i128),(-109790947518296793916712354234157629086_i128),95771652442964879070294331232596944193_i128,(-166945971707018628932673563208808589486_i128)];
_4 = !10981_u16;
RET = [160197991385161972550531952330652956056_i128,89377679308890798203383254055364345145_i128,(-41416231192077452688999065576557664472_i128),(-58329045755748464545538847445855382368_i128)];
_3 = (-53_isize);
_3 = (-307_i16) as isize;
_4 = 17143_u16;
RET = [(-85975739508131563547529266458728911902_i128),119546502321872294689961499487921167698_i128,(-53945937161310965510670751392849180329_i128),(-143892113694292652641730436405682184243_i128)];
_2 = '\u{41acd}';
_5 = (-5812829289784748181_i64) + 5203898855700312850_i64;
_4 = 20841_u16;
_2 = '\u{efb70}';
_7 = _2;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
20841 => bb6,
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
_9 = -(-117542727089245918867033254208910541036_i128);
_7 = _2;
_1 = 1249347927_u32 * 1752147535_u32;
_9 = !(-170075162089662349586788712012983953770_i128);
_6 = 4580233865013349292_u64 << _5;
RET = [_9,_9,_9,_9];
RET = [_9,_9,_9,_9];
RET = [_9,_9,_9,_9];
RET = [_9,_9,_9,_9];
_1 = !650025292_u32;
_3 = (-9223372036854775808_isize);
RET = [_9,_9,_9,_9];
_1 = 1672405380_u32 >> _3;
_2 = _7;
_10 = 149_u8 as f64;
_3 = _1 as isize;
Call(_2 = fn1(_9, _1, _6, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = _5 as f64;
_9 = !59072258505325486563658624292188971340_i128;
RET = [_9,_9,_9,_9];
_9 = _3 as i128;
_10 = _3 as f64;
_7 = _2;
_9 = -72347403755196313421120811970593781332_i128;
_2 = _7;
_10 = _5 as f64;
_6 = 4278660134525615305_u64 | 3280454156534371969_u64;
_1 = !3369976782_u32;
_9 = (-34174835971169521664630449863353367816_i128);
_14 = false;
_7 = _2;
_5 = (-6824067168405606965_i64) >> _3;
_3 = -(-9223372036854775808_isize);
_3 = 4_usize as isize;
_15 = [22533_i16,10108_i16,(-25964_i16)];
_10 = 3_usize as f64;
_13 = (-27604_i16);
_14 = true;
RET = [_9,_9,_9,_9];
_18.1 = 14291640706286726817_usize | 1_usize;
Goto(bb8)
}
bb8 = {
_7 = _2;
_1 = _4 as u32;
_2 = _7;
_5 = !(-7304013076484363415_i64);
_3 = _10 as isize;
_23.2.fld6 = _4 ^ _4;
_23.2.fld5 = _1 as f32;
_20 = !_6;
_5 = _23.2.fld5 as i64;
_24 = _7 < _2;
_11 = 129037217510210571488396308247235067010_u128 as isize;
_13 = 30876_i16 * (-11269_i16);
_23.2.fld3 = _1 >> _6;
_10 = _23.2.fld5 as f64;
_16 = -(-1897551452_i32);
_11 = _3 >> _23.2.fld3;
match _9 {
0 => bb6,
1 => bb4,
306107530949768941798744157568414843640 => bb9,
_ => bb3
}
}
bb9 = {
_23.2.fld0 = _24;
_6 = _20 * _20;
_2 = _7;
_23.2.fld2 = [_5];
Goto(bb10)
}
bb10 = {
_23.3.2 = _1 as i8;
_27.fld0 = _14;
_23.3.0 = _5 | _5;
_11 = _23.2.fld6 as isize;
_19 = [_23.3.2,_23.3.2,_23.3.2,_23.3.2];
_23.3.0 = -_5;
_26 = [_23.2.fld0,_14];
_27 = Adt21 { fld0: _23.2.fld0,fld1: _6,fld2: _23.3.0,fld3: _18.1 };
_7 = _2;
_23.2.fld7 = !_9;
_6 = _20;
RET = [_23.2.fld7,_9,_9,_23.2.fld7];
_16 = (-1654060370_i32);
_23.1 = &_23.2.fld1;
_1 = _23.2.fld3 & _23.2.fld3;
_9 = _23.2.fld7 << _23.3.0;
_23.1 = &_23.2.fld1;
_10 = _18.1 as f64;
_31 = _5 as u64;
_26 = [_27.fld0,_24];
_27.fld1 = !_20;
_19 = [_23.3.2,_23.3.2,_23.3.2,_23.3.2];
_23.3.2 = (-115_i8) + 60_i8;
Goto(bb11)
}
bb11 = {
_32.2 = 29122333305348241092640214819771222766_u128 as u64;
_23.2.fld2 = [_27.fld2];
_24 = _23.2.fld0;
_14 = _24;
_13 = -(-25143_i16);
_27 = Adt21 { fld0: _24,fld1: _31,fld2: _5,fld3: _18.1 };
_1 = _5 as u32;
_15 = [_13,_13,_13];
Goto(bb12)
}
bb12 = {
_24 = !_23.2.fld0;
_23.2.fld0 = _20 <= _20;
_33 = 221775768149389859667221826814769021316_u128 | 22610089895477280269707085997390250926_u128;
_18.1 = _27.fld3;
_23.3.0 = _27.fld2 + _5;
match _4 {
0 => bb13,
1 => bb14,
2 => bb15,
20841 => bb17,
_ => bb16
}
}
bb13 = {
Return()
}
bb14 = {
_23.3.2 = _1 as i8;
_27.fld0 = _14;
_23.3.0 = _5 | _5;
_11 = _23.2.fld6 as isize;
_19 = [_23.3.2,_23.3.2,_23.3.2,_23.3.2];
_23.3.0 = -_5;
_26 = [_23.2.fld0,_14];
_27 = Adt21 { fld0: _23.2.fld0,fld1: _6,fld2: _23.3.0,fld3: _18.1 };
_7 = _2;
_23.2.fld7 = !_9;
_6 = _20;
RET = [_23.2.fld7,_9,_9,_23.2.fld7];
_16 = (-1654060370_i32);
_23.1 = &_23.2.fld1;
_1 = _23.2.fld3 & _23.2.fld3;
_9 = _23.2.fld7 << _23.3.0;
_23.1 = &_23.2.fld1;
_10 = _18.1 as f64;
_31 = _5 as u64;
_26 = [_27.fld0,_24];
_27.fld1 = !_20;
_19 = [_23.3.2,_23.3.2,_23.3.2,_23.3.2];
_23.3.2 = (-115_i8) + 60_i8;
Goto(bb11)
}
bb15 = {
_23.2.fld0 = _24;
_6 = _20 * _20;
_2 = _7;
_23.2.fld2 = [_5];
Goto(bb10)
}
bb16 = {
Return()
}
bb17 = {
_32.0 = _23.2.fld5;
_26 = [_14,_24];
_27.fld0 = _14 | _24;
_14 = !_23.2.fld0;
_37 = _23.2.fld6 * _23.2.fld6;
_25 = &_32.1;
_27.fld0 = !_23.2.fld0;
_27 = Adt21 { fld0: _23.2.fld0,fld1: _32.2,fld2: _5,fld3: _18.1 };
_23.2.fld3 = _1;
_21 = &_23.3;
_20 = _31 >> _37;
_41 = 210_u8 << _23.3.2;
_17 = _7;
_17 = _2;
_16 = !(-583574163_i32);
_23.2.fld4 = _13;
_18.2 = [_18.1];
_23.2.fld4 = _13 & _13;
_13 = _23.2.fld4;
_35 = [_41,_41,_41,_41,_41,_41,_41];
Goto(bb18)
}
bb18 = {
Call(_44 = dump_var(0_usize, 14_usize, Move(_14), 11_usize, Move(_11), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(0_usize, 33_usize, Move(_33), 41_usize, Move(_41), 31_usize, Move(_31), 9_usize, Move(_9)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(0_usize, 2_usize, Move(_2), 37_usize, Move(_37), 3_usize, Move(_3), 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i128,mut _2: u32,mut _3: u64,mut _4: isize) -> char {
mir! {
type RET = char;
let _5: [i128; 4];
let _6: [i128; 4];
let _7: i64;
let _8: [i16; 3];
let _9: (&'static (i64, *const [i16; 5], i8), (usize, u16), &'static i32, [bool; 2]);
let _10: [i16; 3];
let _11: i8;
let _12: bool;
let _13: &'static isize;
let _14: isize;
let _15: u64;
let _16: isize;
let _17: u64;
let _18: [bool; 2];
let _19: i128;
let _20: i128;
let _21: [i8; 4];
let _22: isize;
let _23: f32;
let _24: ();
let _25: ();
{
RET = '\u{84b83}';
_1 = (-124390676385204273929835047423931164671_i128) << _3;
_1 = 91791235364296443294589782867379868534_i128;
RET = '\u{2d6bc}';
_2 = 4073301033_u32;
RET = '\u{dfbb3}';
_1 = 65017790802161315982851985582079729909_i128 << _3;
_3 = 140612488142150573663477109915865963098_u128 as u64;
_2 = 425119231_u32 ^ 3866492189_u32;
_3 = !11050308202189945657_u64;
RET = '\u{f596e}';
_2 = !1525104211_u32;
RET = '\u{c5a73}';
_2 = !589017391_u32;
Call(_2 = fn2(_4, _4, _4, _1, _1, _1, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = '\u{b495e}';
_3 = _1 as u64;
_3 = (-313761556_i32) as u64;
_4 = (-9223372036854775808_isize);
RET = '\u{3b52}';
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_3 = 10593251361336535284_u64;
RET = '\u{ecb8}';
_1 = (-76634071263152674809683412971978906304_i128) * (-145696253626128099232799089981609897308_i128);
_3 = 15861053210595714286_u64 >> _2;
_3 = _2 as u64;
_6 = [_1,_1,_1,_1];
_2 = _4 as u32;
_3 = 10631731927876326472_u64;
_6 = [_1,_1,_1,_1];
_4 = -9223372036854775807_isize;
_3 = _4 as u64;
_6 = _5;
_5 = _6;
RET = '\u{10f090}';
_8 = [(-15563_i16),(-27862_i16),(-8896_i16)];
_1 = 66626951703018538001940431855570765834_i128 << _2;
Goto(bb2)
}
bb2 = {
_4 = 9223372036854775807_isize | (-9223372036854775808_isize);
_8 = [(-2291_i16),(-19797_i16),(-268_i16)];
_2 = 2711315695_u32 * 1576993847_u32;
RET = '\u{42fbe}';
RET = '\u{ae88c}';
_7 = _4 as i64;
_8 = [3999_i16,(-3332_i16),21117_i16];
RET = '\u{3bf3f}';
_6 = _5;
_4 = 121_isize | 9223372036854775807_isize;
_4 = (-60_isize) << _1;
_1 = (-28733824787558624651548664186769599661_i128) - 32201599716723422093648889806527717615_i128;
_7 = -(-828967573357171337_i64);
_4 = 9223372036854775807_isize >> _1;
_8 = [15476_i16,(-167_i16),20400_i16];
_3 = 27_i8 as u64;
_9.1.0 = 142863397299450185460327732992888718434_u128 as usize;
_2 = 3836559474_u32 | 16330602_u32;
_9.1 = (12411525122133816140_usize, 42111_u16);
_12 = !false;
match _9.1.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
42111 => bb10,
_ => bb9
}
}
bb3 = {
RET = '\u{b495e}';
_3 = _1 as u64;
_3 = (-313761556_i32) as u64;
_4 = (-9223372036854775808_isize);
RET = '\u{3b52}';
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_3 = 10593251361336535284_u64;
RET = '\u{ecb8}';
_1 = (-76634071263152674809683412971978906304_i128) * (-145696253626128099232799089981609897308_i128);
_3 = 15861053210595714286_u64 >> _2;
_3 = _2 as u64;
_6 = [_1,_1,_1,_1];
_2 = _4 as u32;
_3 = 10631731927876326472_u64;
_6 = [_1,_1,_1,_1];
_4 = -9223372036854775807_isize;
_3 = _4 as u64;
_6 = _5;
_5 = _6;
RET = '\u{10f090}';
_8 = [(-15563_i16),(-27862_i16),(-8896_i16)];
_1 = 66626951703018538001940431855570765834_i128 << _2;
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
_14 = _4 ^ _4;
_5 = _6;
_5 = _6;
RET = '\u{3235c}';
_9.3 = [_12,_12];
_10 = [(-2066_i16),(-23646_i16),22207_i16];
_10 = [(-1257_i16),(-11605_i16),25979_i16];
_16 = _4 & _14;
RET = '\u{e6c62}';
_3 = 11981257458450696382_u64;
_4 = _16 << _2;
_20 = (-286844185_i32) as i128;
_13 = &_14;
_9.1.0 = 11496766789614848500_usize & 15589590940630185709_usize;
_12 = !false;
match _3 {
0 => bb9,
1 => bb5,
2 => bb11,
11981257458450696382 => bb13,
_ => bb12
}
}
bb11 = {
RET = '\u{b495e}';
_3 = _1 as u64;
_3 = (-313761556_i32) as u64;
_4 = (-9223372036854775808_isize);
RET = '\u{3b52}';
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_3 = 10593251361336535284_u64;
RET = '\u{ecb8}';
_1 = (-76634071263152674809683412971978906304_i128) * (-145696253626128099232799089981609897308_i128);
_3 = 15861053210595714286_u64 >> _2;
_3 = _2 as u64;
_6 = [_1,_1,_1,_1];
_2 = _4 as u32;
_3 = 10631731927876326472_u64;
_6 = [_1,_1,_1,_1];
_4 = -9223372036854775807_isize;
_3 = _4 as u64;
_6 = _5;
_5 = _6;
RET = '\u{10f090}';
_8 = [(-15563_i16),(-27862_i16),(-8896_i16)];
_1 = 66626951703018538001940431855570765834_i128 << _2;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_2 = 2965057741_u32;
_9.1.1 = 51221950486574440553812438502421336513_u128 as u16;
_9.3 = [_12,_12];
RET = '\u{1f7fe}';
RET = '\u{f13b0}';
_9.1.0 = !5_usize;
match _2 {
0 => bb14,
2965057741 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
RET = '\u{b495e}';
_3 = _1 as u64;
_3 = (-313761556_i32) as u64;
_4 = (-9223372036854775808_isize);
RET = '\u{3b52}';
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_5 = [_1,_1,_1,_1];
_3 = 10593251361336535284_u64;
RET = '\u{ecb8}';
_1 = (-76634071263152674809683412971978906304_i128) * (-145696253626128099232799089981609897308_i128);
_3 = 15861053210595714286_u64 >> _2;
_3 = _2 as u64;
_6 = [_1,_1,_1,_1];
_2 = _4 as u32;
_3 = 10631731927876326472_u64;
_6 = [_1,_1,_1,_1];
_4 = -9223372036854775807_isize;
_3 = _4 as u64;
_6 = _5;
_5 = _6;
RET = '\u{10f090}';
_8 = [(-15563_i16),(-27862_i16),(-8896_i16)];
_1 = 66626951703018538001940431855570765834_i128 << _2;
Goto(bb2)
}
bb16 = {
_19 = -_1;
_13 = &(*_13);
_9.1.0 = 14836235646164521790_usize | 6_usize;
_15 = _3;
_20 = _1 - _19;
_10 = [25472_i16,22436_i16,(-29355_i16)];
_17 = _3;
_22 = _12 as isize;
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(1_usize, 10_usize, Move(_10), 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(1_usize, 17_usize, Move(_17), 20_usize, Move(_20), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i128,mut _5: i128,mut _6: i128,mut _7: u64,mut _8: i128) -> u32 {
mir! {
type RET = u32;
let _9: i8;
let _10: usize;
let _11: i8;
let _12: (Adt21, [i32; 2], i32);
let _13: f64;
let _14: &'static (usize, u16);
let _15: *mut &'static u8;
let _16: *const (usize, u16);
let _17: f32;
let _18: f32;
let _19: (f32, u8, u64);
let _20: i128;
let _21: [usize; 1];
let _22: u64;
let _23: *const [i16; 5];
let _24: i8;
let _25: f32;
let _26: f32;
let _27: &'static u32;
let _28: (*mut (usize, u16), &'static Adt53, Adt58, (i64, *const [i16; 5], i8));
let _29: [u16; 8];
let _30: (&'static u32, u16, *mut &'static u8, *mut (usize, u16));
let _31: char;
let _32: Adt21;
let _33: isize;
let _34: isize;
let _35: &'static Adt75;
let _36: *const *const *const usize;
let _37: [u16; 8];
let _38: i16;
let _39: Adt21;
let _40: i32;
let _41: ();
let _42: ();
{
RET = 2734983515_u32 | 4194534275_u32;
RET = 1495762369_u32;
_5 = _8;
_7 = 7588647562849433073_u64 & 350598395675787676_u64;
_5 = _8 + _4;
_2 = _1 ^ _3;
_8 = _6;
_9 = 68_i8 << _8;
_1 = _3 & _3;
RET = 3512032702_u32;
_10 = _7 as usize;
_9 = !(-56_i8);
RET = !1174606298_u32;
RET = 22863149_u32 * 1759367079_u32;
_7 = !6202321550431871293_u64;
_10 = !17999217641052343542_usize;
RET = 1802267116_u32;
_10 = !4648388134748932309_usize;
Call(_2 = fn3(_4, _1, _1, _8, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.0.fld0 = false;
_11 = !_9;
_11 = _9;
_8 = _4;
_12.1 = [(-582021951_i32),(-946127186_i32)];
_5 = _8;
_10 = 6_usize >> _2;
_12.0.fld2 = 3252150664584165294_i64 | 6659339202167413820_i64;
_12.2 = !(-1000873058_i32);
_9 = 41144_u16 as i8;
_12.1 = [_12.2,_12.2];
_12.0.fld1 = _7;
_9 = _11 ^ _11;
_12.2 = 738165887_i32 | 2077558314_i32;
RET = _12.0.fld1 as u32;
_9 = _10 as i8;
_12.0 = Adt21 { fld0: false,fld1: _7,fld2: 521708437681909013_i64,fld3: _10 };
RET = !1283342456_u32;
_8 = _5;
_3 = _8 as isize;
_12.0.fld3 = _10;
_13 = _12.0.fld3 as f64;
_13 = _12.0.fld2 as f64;
_7 = _12.0.fld1 & _12.0.fld1;
_12.0.fld0 = false;
_12.0.fld3 = _10 - _10;
match _12.0.fld2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
521708437681909013 => bb8,
_ => bb7
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
_3 = _2 | _2;
_12.0 = Adt21 { fld0: true,fld1: _7,fld2: 8881997399336181596_i64,fld3: _10 };
_12.1 = [_12.2,_12.2];
_9 = '\u{a03a9}' as i8;
_12.0.fld3 = _10 ^ _10;
_1 = !_2;
_10 = _12.0.fld3;
_4 = -_6;
_9 = _11 | _11;
_19.2 = _6 as u64;
_19.0 = _12.0.fld2 as f32;
_21 = [_12.0.fld3];
_8 = _1 as i128;
_5 = _8 | _8;
_22 = _12.0.fld1 + _7;
_21 = [_12.0.fld3];
Goto(bb9)
}
bb9 = {
_11 = _9 + _9;
_4 = _5;
_5 = _8 - _4;
_17 = _19.0 - _19.0;
_12.0.fld1 = _22;
_12.0.fld3 = (-27040_i16) as usize;
_20 = _4;
_2 = _3 - _1;
_13 = _10 as f64;
_28.2.fld3 = RET & RET;
RET = _28.2.fld3 | _28.2.fld3;
RET = !_28.2.fld3;
_22 = (-21030_i16) as u64;
_24 = _11 - _11;
_19.2 = _7;
_12.0.fld1 = '\u{ca7d}' as u64;
_18 = _19.0;
_9 = _11;
_28.2.fld4 = (-23566_i16) << _24;
_12.1 = [_12.2,_12.2];
_12.0.fld0 = !false;
_12.0.fld3 = !_10;
_27 = &RET;
_19.0 = _17;
Call(_30.3 = fn6(Move(_27), _12.0.fld2, _10, _3, _2, _17, _3, _12, _5, _10, _3, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_28.1 = &_28.2.fld1;
Goto(bb11)
}
bb11 = {
_12.0.fld1 = !_19.2;
_28.2.fld7 = _28.2.fld4 as i128;
_12.0.fld0 = !true;
_18 = _17;
_12.2 = (-690232865_i32);
_21 = [_10];
_28.0 = Move(_30.3);
_32.fld3 = RET as usize;
_21 = [_10];
_28.2.fld3 = !RET;
_20 = _8 >> _8;
_19.1 = 192_u8 * 21_u8;
_26 = _4 as f32;
_31 = '\u{831eb}';
_32.fld3 = _12.0.fld3;
_11 = _9 - _9;
_3 = _2;
_2 = _1;
_19.2 = _10 as u64;
_30.1 = 28025_u16;
_30.0 = &_28.2.fld3;
_33 = _12.0.fld2 as isize;
_27 = Move(_30.0);
_4 = _13 as i128;
_21 = [_32.fld3];
_30.0 = &_28.2.fld3;
Goto(bb12)
}
bb12 = {
_12.1 = [_12.2,_12.2];
_32 = Adt21 { fld0: _12.0.fld0,fld1: _19.2,fld2: _12.0.fld2,fld3: _12.0.fld3 };
_4 = _32.fld3 as i128;
_3 = !_1;
_26 = _18 * _19.0;
_28.2.fld5 = -_18;
_30.3 = Move(_28.0);
_17 = _28.2.fld5;
_33 = -_1;
_28.1 = &_28.2.fld1;
match _32.fld2 {
0 => bb11,
1 => bb2,
8881997399336181596 => bb14,
_ => bb13
}
}
bb13 = {
_28.1 = &_28.2.fld1;
Goto(bb11)
}
bb14 = {
_37 = [_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1];
_7 = _19.2 | _32.fld1;
_28.2.fld5 = -_18;
_32.fld0 = _33 < _1;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(2_usize, 8_usize, Move(_8), 1_usize, Move(_1), 31_usize, Move(_31), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(2_usize, 24_usize, Move(_24), 4_usize, Move(_4), 21_usize, Move(_21), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(2_usize, 11_usize, Move(_11), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: i128,mut _2: isize,mut _3: isize,mut _4: i128,mut _5: i128) -> isize {
mir! {
type RET = isize;
let _6: f32;
let _7: bool;
let _8: isize;
let _9: [i16; 5];
let _10: ();
let _11: ();
{
RET = _2;
_3 = -RET;
_1 = (-59_i8) as i128;
RET = !_2;
_5 = !_4;
_5 = _1 ^ _1;
_6 = (-67128076_i32) as f32;
_1 = _4 >> _5;
_2 = RET ^ _3;
_4 = _1;
_3 = RET & _2;
_4 = _1;
RET = _3;
_5 = _1;
_1 = !_4;
RET = '\u{54643}' as isize;
RET = _3 | _2;
_6 = 162523567287424373255117914561316241778_u128 as f32;
RET = -_2;
RET = (-19338_i16) as isize;
_1 = _5 | _4;
_2 = _3 ^ _3;
Goto(bb1)
}
bb1 = {
RET = 8563927274959169632_i64 as isize;
_5 = _1;
_4 = _1 * _5;
_6 = _1 as f32;
_2 = !_3;
_3 = _2 * _2;
_2 = 25_i8 as isize;
RET = 16573_u16 as isize;
_1 = _4;
_1 = _4;
_1 = _4;
_3 = RET & _2;
_1 = _4 | _5;
_1 = _4;
RET = _3 * _3;
_7 = !false;
_6 = 961042912_u32 as f32;
_1 = (-1695470162_i32) as i128;
Call(RET = fn4(_4, _5, _5, _4, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = 4289210059_u32 as i128;
_7 = false;
Call(_8 = fn5(_5, RET, RET, _4, _5, RET, _6, _3, RET, RET, RET, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = _4 | _1;
Goto(bb4)
}
bb4 = {
_2 = 258728014107412296615219578037305002005_u128 as isize;
_5 = _1 << _8;
_7 = _5 > _5;
RET = _2;
RET = _8;
RET = _8;
RET = _3 + _8;
_9 = [(-1384_i16),(-18324_i16),11579_i16,19553_i16,25741_i16];
_7 = !true;
_4 = _5 - _5;
_9 = [5832_i16,9209_i16,23418_i16,19577_i16,11759_i16];
_5 = _4;
_4 = !_5;
_4 = !_5;
_2 = RET + _8;
_7 = !true;
_3 = RET * _2;
_5 = 18112_u16 as i128;
_1 = '\u{ac269}' as i128;
RET = (-1805341624876592138_i64) as isize;
_1 = _4 * _4;
_2 = _3 - _3;
_2 = _3 & _3;
_3 = 4228_i16 as isize;
_7 = _1 != _4;
_1 = _4 | _4;
_4 = -_1;
RET = !_2;
_2 = RET;
_4 = _1 * _1;
Goto(bb5)
}
bb5 = {
Call(_10 = dump_var(3_usize, 4_usize, Move(_4), 9_usize, Move(_9), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i128,mut _2: i128,mut _3: i128,mut _4: i128,mut _5: i128) -> isize {
mir! {
type RET = isize;
let _6: [i8; 4];
let _7: isize;
let _8: [i128; 4];
let _9: i64;
let _10: char;
let _11: i32;
let _12: [u64; 5];
let _13: Adt21;
let _14: &'static *mut i16;
let _15: i128;
let _16: i128;
let _17: Adt19;
let _18: [i128; 2];
let _19: char;
let _20: *mut (usize, u16);
let _21: [u8; 7];
let _22: ();
let _23: ();
{
RET = (-21_isize);
RET = _4 as isize;
_4 = !_1;
_3 = -_4;
_1 = !_4;
RET = !(-9223372036854775808_isize);
RET = 73_isize;
_1 = _4 ^ _4;
_2 = 3_usize as i128;
_2 = false as i128;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
73 => bb6,
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
_4 = _1 ^ _1;
RET = -(-9223372036854775808_isize);
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
RET = (-78_isize) + (-9223372036854775808_isize);
_7 = RET & RET;
_1 = _7 as i128;
RET = -_7;
RET = _7 ^ _7;
_2 = true as i128;
_8 = [_3,_4,_4,_3];
RET = (-1192920910_i32) as isize;
_4 = _3 - _3;
_6 = [89_i8,98_i8,8_i8,50_i8];
_2 = _4;
_4 = _2 * _2;
RET = _7 * _7;
_8 = [_4,_2,_4,_2];
_4 = !_3;
_3 = _2;
_3 = 126_u8 as i128;
RET = !_7;
RET = _7;
_1 = _2;
_6 = [113_i8,16_i8,40_i8,(-48_i8)];
Goto(bb7)
}
bb7 = {
_2 = _1 << _1;
_2 = _1 >> _1;
_5 = _2 - _1;
_8 = [_2,_5,_5,_1];
_4 = _5;
Goto(bb8)
}
bb8 = {
_3 = 3625120483_u32 as i128;
_6 = [26_i8,(-20_i8),(-29_i8),(-34_i8)];
_3 = -_5;
_3 = (-1884098788_i32) as i128;
_2 = (-7155831102845707370_i64) as i128;
_8 = [_5,_1,_4,_5];
RET = _7;
RET = _7;
_3 = -_4;
_6 = [(-69_i8),(-36_i8),(-89_i8),2_i8];
RET = _7;
_8 = [_3,_1,_4,_3];
_4 = _1;
_4 = !_5;
RET = 8182_i16 as isize;
_5 = _4 ^ _3;
RET = _7;
_6 = [(-56_i8),85_i8,31_i8,(-7_i8)];
RET = _7;
RET = _7;
RET = 8772366900018197632_i64 as isize;
_6 = [33_i8,37_i8,76_i8,(-101_i8)];
RET = 0_usize as isize;
_9 = _4 as i64;
_6 = [21_i8,(-91_i8),(-48_i8),(-98_i8)];
_7 = RET - RET;
_1 = _4;
Goto(bb9)
}
bb9 = {
RET = !_7;
_6 = [(-56_i8),60_i8,77_i8,(-89_i8)];
_10 = '\u{b8546}';
_4 = !_3;
_10 = '\u{21e10}';
RET = 3278140853_u32 as isize;
_11 = (-89_i8) as i32;
_9 = 8998767891267534732_i64 + 8930427149297935815_i64;
_5 = -_3;
_5 = _3 + _3;
_3 = -_4;
_4 = _5;
_9 = -(-4624840139416865282_i64);
_6 = [(-71_i8),79_i8,(-37_i8),(-5_i8)];
_1 = -_4;
_1 = _3;
Goto(bb10)
}
bb10 = {
_2 = _4;
_3 = _7 as i128;
RET = !_7;
RET = _7;
_1 = RET as i128;
_3 = _4;
RET = _7 ^ _7;
RET = _7;
RET = _7 + _7;
RET = -_7;
RET = _7;
_13.fld2 = 12790983297099757593_u64 as i64;
RET = _7 | _7;
_6 = [(-28_i8),68_i8,(-127_i8),99_i8];
_13.fld0 = !true;
_6 = [103_i8,(-35_i8),37_i8,(-39_i8)];
Goto(bb11)
}
bb11 = {
_6 = [99_i8,118_i8,105_i8,64_i8];
_9 = _13.fld2;
_8 = [_5,_2,_5,_3];
_10 = '\u{2a368}';
_8 = [_4,_5,_3,_3];
_13.fld1 = !5341384465308740786_u64;
_13.fld1 = !364399814185336864_u64;
Goto(bb12)
}
bb12 = {
_3 = 220839516479200086159452138086527187332_u128 as i128;
_7 = RET & RET;
_13.fld0 = !true;
_15 = _5;
_8 = [_5,_4,_4,_5];
_5 = _4;
_16 = -_5;
_17.fld2 = 159860329147085735158874537244698064687_u128 + 322186478359318573832549919349733529518_u128;
_2 = -_15;
Goto(bb13)
}
bb13 = {
_10 = '\u{f39ec}';
_5 = _17.fld2 as i128;
_13 = Adt21 { fld0: true,fld1: 4342306334315369559_u64,fld2: _9,fld3: 3_usize };
_6 = [96_i8,58_i8,126_i8,(-110_i8)];
_2 = _11 as i128;
_4 = 244_u8 as i128;
Goto(bb14)
}
bb14 = {
_8 = [_15,_16,_16,_15];
_12 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1,_13.fld1];
RET = -_7;
_13 = Adt21 { fld0: false,fld1: 11649569358597057496_u64,fld2: _9,fld3: 0_usize };
_13.fld2 = _9 >> _16;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(4_usize, 1_usize, Move(_1), 4_usize, Move(_4), 9_usize, Move(_9), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(4_usize, 2_usize, Move(_2), 12_usize, Move(_12), 8_usize, Move(_8), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i128,mut _2: isize,mut _3: isize,mut _4: i128,mut _5: i128,mut _6: isize,mut _7: f32,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: (&'static (i64, *const [i16; 5], i8), (usize, u16), &'static i32, [bool; 2]);
let _14: isize;
let _15: usize;
let _16: u16;
let _17: &'static Adt53;
let _18: isize;
let _19: [usize; 1];
let _20: &'static Adt53;
let _21: f32;
let _22: ();
let _23: ();
{
_8 = _3;
_12 = 45911_u16 as isize;
_9 = _8;
RET = _2 | _8;
_4 = -_5;
_5 = _4;
RET = _3 >> _5;
_5 = _4 ^ _4;
_8 = 12009376236797231902_u64 as isize;
RET = _3;
_13.1.0 = '\u{3810e}' as usize;
_13.1 = (12759668012285844837_usize, 4238_u16);
_13.3 = [true,true];
_3 = -_2;
_13.1.1 = 85095463_u32 as u16;
_16 = _13.1.1 * _13.1.1;
_10 = -_12;
_16 = _13.1.1 ^ _13.1.1;
_10 = RET;
_4 = !_5;
match _13.1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
12759668012285844837 => bb9,
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
_3 = 417647377595049271_u64 as isize;
_15 = false as usize;
_10 = _6 & _9;
_13.1.0 = 191660473139507067037053699311221615332_u128 as usize;
_14 = _2 * RET;
_9 = 7638985660887601485_i64 as isize;
_6 = _10;
_3 = _14 * _11;
_18 = _16 as isize;
_3 = _14 >> _5;
RET = _10 >> _14;
Goto(bb10)
}
bb10 = {
Call(_22 = dump_var(5_usize, 18_usize, Move(_18), 14_usize, Move(_14), 9_usize, Move(_9), 16_usize, Move(_16)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_22 = dump_var(5_usize, 6_usize, Move(_6), 8_usize, Move(_8), 10_usize, Move(_10), 23_usize, _23), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: &'static u32,mut _2: i64,mut _3: usize,mut _4: isize,mut _5: isize,mut _6: f32,mut _7: isize,mut _8: (Adt21, [i32; 2], i32),mut _9: i128,mut _10: usize,mut _11: isize,mut _12: isize) -> *mut (usize, u16) {
mir! {
type RET = *mut (usize, u16);
let _13: Adt21;
let _14: [u8; 7];
let _15: &'static (&'static u32, u16, *mut &'static u8, *mut (usize, u16));
let _16: isize;
let _17: bool;
let _18: isize;
let _19: *mut char;
let _20: f32;
let _21: char;
let _22: isize;
let _23: isize;
let _24: [i8; 4];
let _25: u32;
let _26: i32;
let _27: ([i32; 2], (i64, *const [i16; 5], i8), bool, *mut i16);
let _28: isize;
let _29: [i128; 2];
let _30: i64;
let _31: i128;
let _32: &'static u8;
let _33: f64;
let _34: bool;
let _35: (*mut (usize, u16), &'static Adt53, Adt58, (i64, *const [i16; 5], i8));
let _36: bool;
let _37: [u64; 5];
let _38: [u64; 5];
let _39: (usize, u16);
let _40: Adt53;
let _41: bool;
let _42: isize;
let _43: u64;
let _44: char;
let _45: ();
let _46: ();
{
_7 = -_4;
_8.0.fld1 = !8878390856557972490_u64;
_8.0.fld3 = _10 + _3;
_3 = _10;
_4 = -_12;
_6 = (-22078_i16) as f32;
_10 = _3;
_8.0.fld0 = true | false;
_8.0 = Adt21 { fld0: false,fld1: 5624687716738320420_u64,fld2: _2,fld3: _3 };
_8.0.fld3 = _10 & _3;
_5 = _7;
_10 = _8.0.fld3 * _3;
_13 = Adt21 { fld0: _8.0.fld0,fld1: _8.0.fld1,fld2: _2,fld3: _10 };
_6 = _2 as f32;
_8.0.fld1 = _13.fld1;
_6 = (-16805_i16) as f32;
_11 = _7;
_8.0.fld3 = _4 as usize;
_8.1 = [_8.2,_8.2];
_11 = _12 << _4;
_8.0.fld2 = _9 as i64;
Call(_11 = fn7(_13.fld1, _8, _8.0, _13.fld1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = Adt21 { fld0: _8.0.fld0,fld1: _8.0.fld1,fld2: _8.0.fld2,fld3: _8.0.fld3 };
_4 = !_12;
_7 = _4;
_13.fld1 = !_8.0.fld1;
_13.fld3 = _7 as usize;
_10 = _13.fld3;
_6 = 57175_u16 as f32;
_5 = _7;
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
8881997399336181596 => bb8,
_ => bb7
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
_13.fld3 = _10 - _3;
_13.fld3 = _3 << _8.0.fld2;
_8.0.fld1 = !_13.fld1;
_5 = _7 - _12;
_13.fld3 = _8.0.fld3;
Call(_8.2 = fn9(_13.fld0, _8.0.fld2, _8.0, _8.0.fld3, _12, _7, _7, _13.fld3, _12, _13.fld3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14 = [67_u8,220_u8,190_u8,100_u8,171_u8,70_u8,27_u8];
_7 = (-17633_i16) as isize;
_5 = 276392299626746853078239089483281420995_u128 as isize;
_4 = _3 as isize;
_7 = _4;
_4 = !_7;
_2 = !_13.fld2;
_18 = 120991374071621115770477922737335439850_u128 as isize;
_9 = '\u{8c912}' as i128;
_18 = -_12;
_8.0.fld2 = -_13.fld2;
_2 = _13.fld2;
Goto(bb10)
}
bb10 = {
_13 = _8.0;
_8.0.fld3 = _9 as usize;
_8.0.fld3 = _13.fld3;
_16 = -_12;
_3 = _13.fld3;
_8.1 = [_8.2,_8.2];
_13.fld0 = _8.0.fld0;
_4 = -_12;
_8.0 = Adt21 { fld0: _13.fld0,fld1: _13.fld1,fld2: _2,fld3: _13.fld3 };
_13.fld3 = !_10;
_10 = 54492537539203925591673768331641545323_u128 as usize;
_17 = _13.fld0;
_8.0.fld1 = !_13.fld1;
_7 = !_4;
_13.fld1 = _8.0.fld1 & _8.0.fld1;
_10 = _8.0.fld3 >> _8.0.fld3;
_9 = 128811799598325836520246100351250758537_i128 & (-134929074463331811077029640151468152332_i128);
_8.0 = _13;
_9 = _8.2 as i128;
_8.0 = Adt21 { fld0: _13.fld0,fld1: _13.fld1,fld2: _13.fld2,fld3: _3 };
_20 = -_6;
_12 = _18 + _18;
_3 = _10;
_11 = _18 << _13.fld3;
_8.0.fld3 = _13.fld3 ^ _10;
_8.0.fld3 = _10;
_9 = 131561597772667130871685874638289064347_i128 - (-71232760530943398791258105472122161622_i128);
Goto(bb11)
}
bb11 = {
_16 = !_7;
_14 = [58_u8,113_u8,112_u8,221_u8,97_u8,237_u8,116_u8];
_8.0 = Adt21 { fld0: _17,fld1: _13.fld1,fld2: _2,fld3: _10 };
_4 = _11 >> _8.0.fld3;
_14 = [186_u8,125_u8,176_u8,5_u8,164_u8,231_u8,252_u8];
_13.fld2 = !_2;
_6 = _20 + _20;
_18 = !_4;
_8.0.fld2 = 728_u16 as i64;
_3 = _8.0.fld3 >> _13.fld2;
Call(_19 = fn10(_4, _13.fld3, _13.fld1, _12, _11, _18, _11, _18, _4), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_8.1 = [_8.2,_8.2];
_18 = _16 ^ _16;
_18 = _20 as isize;
_8.0.fld0 = _13.fld2 < _2;
_7 = -_16;
_13.fld0 = _10 <= _8.0.fld3;
_13 = Adt21 { fld0: _8.0.fld0,fld1: _8.0.fld1,fld2: _2,fld3: _3 };
_8.0.fld0 = _13.fld0;
_17 = _8.0.fld0;
_4 = (-19490_i16) as isize;
_8.1 = [_8.2,_8.2];
_21 = '\u{247ba}';
_6 = _20 - _20;
_2 = _13.fld2;
_16 = _7;
_2 = _13.fld2;
_4 = _16 >> _11;
_13.fld2 = _2;
_19 = core::ptr::addr_of_mut!(_21);
_24 = [(-59_i8),1_i8,(-85_i8),(-70_i8)];
_4 = !_7;
_13.fld1 = 13795_i16 as u64;
Goto(bb13)
}
bb13 = {
_8.1 = [_8.2,_8.2];
_18 = _11 - _11;
_21 = '\u{f539d}';
_13.fld3 = _3 + _10;
_23 = _11 + _12;
_16 = 192812659067902372435291693649292986536_u128 as isize;
_16 = 158_u8 as isize;
_1 = &_25;
_12 = _18 << _3;
_4 = 2876667472_u32 as isize;
_25 = 2438974958_u32 + 3117558951_u32;
_13.fld2 = _2;
_21 = '\u{1097ea}';
_13.fld2 = 32032_u16 as i64;
_9 = !66375878234556004627861970399657809247_i128;
(*_19) = '\u{859f8}';
Goto(bb14)
}
bb14 = {
_13.fld3 = _8.0.fld3;
_7 = _23;
_27.1.0 = !_2;
_14 = [7_u8,61_u8,94_u8,6_u8,182_u8,252_u8,219_u8];
_27.1.2 = 18_i8 | (-88_i8);
_1 = &_25;
_8.0.fld0 = _17 ^ _17;
_6 = _20 * _20;
_23 = _11 ^ _11;
_12 = _7 * _18;
_14 = [9_u8,72_u8,154_u8,173_u8,67_u8,127_u8,192_u8];
_19 = core::ptr::addr_of_mut!(_21);
_20 = _6 + _6;
(*_19) = '\u{9a7b1}';
_6 = _20;
_3 = !_13.fld3;
_12 = _11;
_27.2 = !_17;
Call(_26 = core::intrinsics::bswap(_8.2), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_8.0 = Adt21 { fld0: _27.2,fld1: _13.fld1,fld2: _27.1.0,fld3: _13.fld3 };
_9 = (-115131846849379308195278734027092519457_i128);
_13.fld2 = _8.0.fld2 ^ _8.0.fld2;
_16 = _23 | _18;
_8.0.fld1 = _13.fld1 * _13.fld1;
_27.0 = [_8.2,_8.2];
_27.0 = [_8.2,_8.2];
_19 = core::ptr::addr_of_mut!((*_19));
_13.fld1 = _8.0.fld1;
_25 = 2770094329_u32 >> _13.fld2;
_29 = [_9,_9];
_10 = !_13.fld3;
_7 = _12;
_22 = _11 << _12;
_9 = 39_u8 as i128;
_9 = !(-3297875480733000073460842696810302443_i128);
_31 = !_9;
Goto(bb16)
}
bb16 = {
_13.fld0 = !_17;
_13.fld2 = _27.1.0;
_28 = _20 as isize;
(*_19) = '\u{4f06b}';
_13.fld3 = !_3;
_10 = !_3;
_28 = _16 & _18;
_1 = &_25;
_16 = _2 as isize;
_35.3.0 = _8.0.fld2;
_35.2.fld6 = !55822_u16;
_39.0 = _11 as usize;
_35.2.fld3 = (*_1);
Goto(bb17)
}
bb17 = {
_35.0 = core::ptr::addr_of_mut!(_39);
_39.1 = _35.2.fld6;
_17 = _27.2;
_18 = _2 as isize;
_41 = !_8.0.fld0;
_27.3 = core::ptr::addr_of_mut!(_35.2.fld4);
_4 = -_18;
_13 = Adt21 { fld0: _41,fld1: _8.0.fld1,fld2: _8.0.fld2,fld3: _8.0.fld3 };
_13 = Adt21 { fld0: _8.0.fld0,fld1: _8.0.fld1,fld2: _8.0.fld2,fld3: _10 };
_1 = &(*_1);
(*_19) = '\u{1b5ae}';
_25 = _27.1.2 as u32;
_16 = _11 >> _8.0.fld3;
Goto(bb18)
}
bb18 = {
_35.2.fld0 = _27.1.0 == _8.0.fld2;
_37 = [_8.0.fld1,_13.fld1,_13.fld1,_8.0.fld1,_8.0.fld1];
_35.2.fld7 = _9 ^ _9;
_8.2 = -723932366_i32;
RET = core::ptr::addr_of_mut!(_39);
_35.2.fld5 = _20;
_10 = _8.0.fld3;
(*RET).0 = _3;
_35.2.fld0 = _35.2.fld3 <= _35.2.fld3;
Goto(bb19)
}
bb19 = {
Call(_45 = dump_var(6_usize, 24_usize, Move(_24), 10_usize, Move(_10), 17_usize, Move(_17), 41_usize, Move(_41)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(6_usize, 2_usize, Move(_2), 12_usize, Move(_12), 18_usize, Move(_18), 26_usize, Move(_26)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_45 = dump_var(6_usize, 11_usize, Move(_11), 5_usize, Move(_5), 21_usize, Move(_21), 23_usize, Move(_23)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u64,mut _2: (Adt21, [i32; 2], i32),mut _3: Adt21,mut _4: u64) -> isize {
mir! {
type RET = isize;
let _5: bool;
let _6: *const (usize, u16);
let _7: (&'static (i64, *const [i16; 5], i8), (usize, u16), &'static i32, [bool; 2]);
let _8: f64;
let _9: bool;
let _10: (((i64, *const [i16; 5], i8),), i32);
let _11: [u16; 8];
let _12: [u16; 6];
let _13: u128;
let _14: [usize; 1];
let _15: Adt58;
let _16: u128;
let _17: &'static *const ((i64, *const [i16; 5], i8), i8, Adt19);
let _18: char;
let _19: f64;
let _20: ();
let _21: ();
{
RET = (-40_isize) * (-9223372036854775808_isize);
_2.0.fld1 = !_4;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
5624687716738320420 => bb8,
_ => bb7
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
_2.0.fld3 = _3.fld3;
_5 = !_3.fld0;
_3.fld0 = _5;
RET = 9223372036854775807_isize ^ 112_isize;
_2.1 = [_2.2,_2.2];
_4 = _2.0.fld1;
RET = (-9223372036854775808_isize);
_4 = _3.fld0 as u64;
_3 = Adt21 { fld0: _2.0.fld0,fld1: _4,fld2: _2.0.fld2,fld3: _2.0.fld3 };
Call(_2.0.fld2 = core::intrinsics::transmute(_3.fld3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_4 = 11_i8 as u64;
_4 = _1 - _2.0.fld1;
_2.0.fld2 = _3.fld2 * _3.fld2;
_3.fld2 = _2.0.fld2 + _2.0.fld2;
_2.0.fld0 = !_5;
RET = (-9223372036854775808_isize);
_7.1 = (_2.0.fld3, 57327_u16);
_2.0 = _3;
RET = !9223372036854775807_isize;
_7.3 = [_2.0.fld0,_3.fld0];
_9 = !_5;
_2.0.fld1 = !_4;
_7.1 = (_3.fld3, 21960_u16);
_7.1 = (_2.0.fld3, 30053_u16);
_7.1.1 = !40235_u16;
_2.0.fld3 = _7.1.0;
_2.1 = [_2.2,_2.2];
_6 = core::ptr::addr_of!(_7.1);
Goto(bb10)
}
bb10 = {
(*_6) = (_2.0.fld3, 54576_u16);
_3.fld2 = _2.0.fld2;
_7.2 = &_10.1;
_13 = !23760398601373613267460664993703411223_u128;
_10.0.0.2 = _3.fld2 as i8;
_3 = Adt21 { fld0: _9,fld1: _2.0.fld1,fld2: _2.0.fld2,fld3: _2.0.fld3 };
_10.1 = !_2.2;
RET = (-9223372036854775808_isize);
_4 = _2.0.fld1;
_3 = Adt21 { fld0: _9,fld1: _4,fld2: _2.0.fld2,fld3: (*_6).0 };
_5 = _9;
_7.1.0 = _2.0.fld3 - _2.0.fld3;
_12 = [(*_6).1,(*_6).1,(*_6).1,(*_6).1,(*_6).1,(*_6).1];
_10.1 = !_2.2;
_7.2 = &_2.2;
_3.fld3 = (*_6).0;
_7.0 = &_10.0.0;
Call(_2.0.fld0 = fn8(Move(_7.0), Move(_7.2), (*_6).1, _2.0.fld2, _2.0.fld3, (*_6)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.fld6 = !(*_6).1;
_15.fld7 = (-154746014315313020080985079173853609327_i128) + 120547579727061648563988715835407028291_i128;
_12 = [_15.fld6,(*_6).1,_15.fld6,_7.1.1,_15.fld6,_15.fld6];
RET = 9223372036854775807_isize - 9223372036854775807_isize;
_15.fld2 = [_3.fld2];
(*_6).1 = _15.fld6 | _15.fld6;
_3.fld0 = _1 <= _2.0.fld1;
(*_6).1 = !_15.fld6;
_14 = [_7.1.0];
_9 = _2.0.fld0 & _2.0.fld0;
_6 = core::ptr::addr_of!((*_6));
_14 = [(*_6).0];
_2.0.fld0 = _5;
_7.1.1 = !_15.fld6;
(*_6).1 = _13 as u16;
_8 = (*_6).0 as f64;
_15.fld0 = !_5;
(*_6) = (_3.fld3, _15.fld6);
_7.2 = &_2.2;
_2.1 = [_2.2,_2.2];
_5 = _2.0.fld0;
_15.fld2 = [_3.fld2];
_3.fld2 = RET as i64;
_15.fld2 = [_2.0.fld2];
(*_6).1 = _15.fld6 ^ _15.fld6;
_15.fld3 = _3.fld3 as u32;
match _1 {
0 => bb12,
5624687716738320420 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_15.fld4 = (-17279_i16);
_1 = _2.0.fld2 as u64;
_2.0.fld0 = _9 & _15.fld0;
_2.0.fld0 = !_9;
_3.fld2 = _2.0.fld2;
_16 = RET as u128;
_2.2 = !_10.1;
_2.2 = _10.1 ^ _10.1;
_10.0.0.0 = -_3.fld2;
_12 = [_7.1.1,_15.fld6,_7.1.1,(*_6).1,_7.1.1,_15.fld6];
_15.fld2 = [_10.0.0.0];
_15.fld7 = !(-108658389703123902550090060492406709011_i128);
_7.2 = &_10.1;
_9 = !_5;
_11 = [(*_6).1,_7.1.1,(*_6).1,_7.1.1,(*_6).1,_7.1.1,(*_6).1,_7.1.1];
_15.fld5 = _15.fld4 as f32;
_2.0.fld3 = _3.fld3 | (*_6).0;
_15.fld4 = _2.2 as i16;
_12 = [_15.fld6,(*_6).1,_15.fld6,_15.fld6,_15.fld6,_15.fld6];
(*_6).1 = !_15.fld6;
_15.fld7 = !91230148648901264764921752332439530242_i128;
_10.0.0.0 = _3.fld2 ^ _2.0.fld2;
(*_6) = (_3.fld3, _15.fld6);
_11 = [(*_6).1,(*_6).1,_15.fld6,(*_6).1,_7.1.1,(*_6).1,(*_6).1,_7.1.1];
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(7_usize, 9_usize, Move(_9), 5_usize, Move(_5), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: &'static (i64, *const [i16; 5], i8),mut _2: &'static i32,mut _3: u16,mut _4: i64,mut _5: usize,mut _6: (usize, u16)) -> bool {
mir! {
type RET = bool;
let _7: char;
let _8: (f32, u8, u64);
let _9: ((i64, *const [i16; 5], i8),);
let _10: *mut &'static u8;
let _11: ();
let _12: ();
{
_6.0 = !_5;
_4 = 3451241221584099160_i64;
_5 = _6.0;
_5 = _6.0 << _3;
RET = !false;
_3 = _6.1 ^ _6.1;
_7 = '\u{3eb18}';
_7 = '\u{2d458}';
_6.0 = _5;
RET = !true;
_3 = 11802347035969736350_u64 as u16;
RET = false;
_7 = '\u{ae95a}';
RET = false;
RET = true;
_8.2 = 17884255275014396559_u64;
_3 = _6.1;
_6.1 = _3 & _3;
RET = _5 >= _5;
_3 = _6.1 + _6.1;
_6.0 = _6.1 as usize;
_5 = _6.0;
_1 = &_9.0;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(8_usize, 6_usize, Move(_6), 5_usize, Move(_5), 12_usize, _12, 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: i64,mut _3: Adt21,mut _4: usize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: usize,mut _9: isize,mut _10: usize) -> i32 {
mir! {
type RET = i32;
let _11: ((*const [i16; 5],), *const [i16; 5], ((i64, *const [i16; 5], i8),));
let _12: char;
let _13: *mut [i32; 2];
let _14: ();
let _15: ();
{
_2 = _3.fld2;
_3.fld3 = _8 << _5;
_3.fld2 = _10 as i64;
_10 = 563059234_u32 as usize;
_5 = 101_u8 as isize;
_7 = 140_u8 as isize;
Goto(bb1)
}
bb1 = {
_1 = !_3.fld0;
_3.fld0 = _1;
RET = (-949527208_i32);
_3.fld1 = !17371551285956756597_u64;
_3.fld2 = _2;
_1 = !_3.fld0;
_8 = _2 as usize;
_5 = 208_u8 as isize;
_1 = _4 > _3.fld3;
_4 = _8;
_6 = _9 - _9;
_9 = _6;
Goto(bb2)
}
bb2 = {
_3 = Adt21 { fld0: _1,fld1: 308278201267348370_u64,fld2: _2,fld3: _8 };
_8 = (-80583497376393759976105851376148160773_i128) as usize;
_3.fld1 = !17733512315174562060_u64;
_3.fld2 = _2 + _2;
_7 = _9;
_4 = _3.fld3 + _3.fld3;
_9 = _6;
_11.2.0.0 = _3.fld2 & _3.fld2;
_9 = _6 + _6;
match RET {
340282366920938463463374607430818684248 => bb4,
_ => bb3
}
}
bb3 = {
_1 = !_3.fld0;
_3.fld0 = _1;
RET = (-949527208_i32);
_3.fld1 = !17371551285956756597_u64;
_3.fld2 = _2;
_1 = !_3.fld0;
_8 = _2 as usize;
_5 = 208_u8 as isize;
_1 = _4 > _3.fld3;
_4 = _8;
_6 = _9 - _9;
_9 = _6;
Goto(bb2)
}
bb4 = {
_9 = -_6;
_10 = 1851123354_u32 as usize;
_3.fld3 = 2843205524_u32 as usize;
RET = 1190522883_i32 << _2;
_3.fld2 = _11.2.0.0 ^ _11.2.0.0;
_12 = '\u{98198}';
_6 = _9 - _9;
_1 = _3.fld0;
_3.fld3 = _4;
_9 = _6;
_3.fld2 = !_11.2.0.0;
_7 = -_9;
_3.fld2 = 73473041834906175454423289156461328152_i128 as i64;
_1 = _3.fld0;
Goto(bb5)
}
bb5 = {
Call(_14 = dump_var(9_usize, 4_usize, Move(_4), 12_usize, Move(_12), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_14 = dump_var(9_usize, 8_usize, Move(_8), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: usize,mut _3: u64,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize) -> *mut char {
mir! {
type RET = *mut char;
let _10: char;
let _11: (*mut (usize, u16), &'static Adt53, Adt58, (i64, *const [i16; 5], i8));
let _12: &'static ([i32; 2], (i64, *const [i16; 5], i8), bool, *mut i16);
let _13: isize;
let _14: &'static *const (&'static u32, u16, *mut &'static u8, *mut (usize, u16));
let _15: (f32, u8, u64);
let _16: i8;
let _17: Adt75;
let _18: isize;
let _19: (usize, u16);
let _20: (f32, u8, u64);
let _21: [u64; 5];
let _22: i128;
let _23: i32;
let _24: i32;
let _25: isize;
let _26: bool;
let _27: (&'static u32, u16, *mut &'static u8, *mut (usize, u16));
let _28: ((*const [i16; 5],), *const [i16; 5], ((i64, *const [i16; 5], i8),));
let _29: u8;
let _30: *mut char;
let _31: usize;
let _32: i16;
let _33: Adt19;
let _34: [i128; 2];
let _35: [i64; 3];
let _36: [i128; 4];
let _37: i128;
let _38: f32;
let _39: *const usize;
let _40: Adt21;
let _41: (usize, u16);
let _42: [i16; 3];
let _43: ();
let _44: ();
{
_5 = 7600_u16 as isize;
_7 = _4;
_9 = 106_u8 as isize;
_8 = _1;
_5 = !_6;
_10 = '\u{830fa}';
RET = core::ptr::addr_of_mut!(_10);
Goto(bb1)
}
bb1 = {
_3 = _7 as u64;
_11.2.fld7 = -69251400859923977693224634445229634416_i128;
_11.3.0 = (-5071443491884539789_i64) * 981952223081131715_i64;
_11.2.fld4 = 23243_i16 & 18379_i16;
_6 = _4;
_3 = 17890159373939013772_u64 >> _2;
(*RET) = '\u{3e60f}';
_11.2.fld5 = _5 as f32;
_7 = _5 >> _1;
(*RET) = '\u{5b4c6}';
_11.2.fld2 = [_11.3.0];
_11.2.fld3 = _11.2.fld4 as u32;
_1 = -_8;
_9 = _5 ^ _6;
_11.1 = &_11.2.fld1;
_10 = '\u{fd4fe}';
_11.2.fld6 = 104_i8 as u16;
RET = core::ptr::addr_of_mut!((*RET));
_7 = _6;
_10 = '\u{256c0}';
Goto(bb2)
}
bb2 = {
_13 = !_4;
_7 = _9;
_3 = !4584347296063952777_u64;
_6 = _7 >> _4;
_8 = -_4;
_2 = _11.2.fld6 as usize;
_11.1 = &_11.2.fld1;
_11.2.fld3 = _11.2.fld4 as u32;
_2 = 13_u8 as usize;
_11.3.2 = !(-15_i8);
_1 = !_4;
(*RET) = '\u{9485}';
(*RET) = '\u{1ef94}';
Goto(bb3)
}
bb3 = {
_11.2.fld2 = [_11.3.0];
_15.0 = -_11.2.fld5;
(*RET) = '\u{4888b}';
_15.2 = _3;
_11.2.fld4 = 6659_i16;
_11.1 = &_11.2.fld1;
_8 = _9;
(*RET) = '\u{4b8e7}';
_4 = false as isize;
_11.3.0 = !1905932450625888875_i64;
RET = core::ptr::addr_of_mut!(_10);
_15.2 = _3 * _3;
_11.3.2 = _11.2.fld4 as i8;
_10 = '\u{6903a}';
(*RET) = '\u{85ca9}';
_3 = _11.2.fld5 as u64;
_11.2.fld5 = _15.0;
_11.2.fld7 = -113940049246423574753168561064197266595_i128;
_2 = 3_usize * 5_usize;
_21 = [_3,_3,_3,_3,_3];
match _11.2.fld4 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
6659 => bb9,
_ => bb8
}
}
bb4 = {
_13 = !_4;
_7 = _9;
_3 = !4584347296063952777_u64;
_6 = _7 >> _4;
_8 = -_4;
_2 = _11.2.fld6 as usize;
_11.1 = &_11.2.fld1;
_11.2.fld3 = _11.2.fld4 as u32;
_2 = 13_u8 as usize;
_11.3.2 = !(-15_i8);
_1 = !_4;
(*RET) = '\u{9485}';
(*RET) = '\u{1ef94}';
Goto(bb3)
}
bb5 = {
_3 = _7 as u64;
_11.2.fld7 = -69251400859923977693224634445229634416_i128;
_11.3.0 = (-5071443491884539789_i64) * 981952223081131715_i64;
_11.2.fld4 = 23243_i16 & 18379_i16;
_6 = _4;
_3 = 17890159373939013772_u64 >> _2;
(*RET) = '\u{3e60f}';
_11.2.fld5 = _5 as f32;
_7 = _5 >> _1;
(*RET) = '\u{5b4c6}';
_11.2.fld2 = [_11.3.0];
_11.2.fld3 = _11.2.fld4 as u32;
_1 = -_8;
_9 = _5 ^ _6;
_11.1 = &_11.2.fld1;
_10 = '\u{fd4fe}';
_11.2.fld6 = 104_i8 as u16;
RET = core::ptr::addr_of_mut!((*RET));
_7 = _6;
_10 = '\u{256c0}';
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
_18 = _7;
_20.1 = _11.2.fld4 as u8;
_19.1 = !_11.2.fld6;
_15.2 = _3;
_16 = -_11.3.2;
(*RET) = '\u{123bd}';
_19.0 = _3 as usize;
_11.0 = core::ptr::addr_of_mut!(_19);
_20.2 = _15.2 << _9;
_24 = 1596536444_i32;
_4 = _18 << _5;
_11.2.fld2 = [_11.3.0];
(*RET) = '\u{5717d}';
_18 = -_9;
_20.0 = _24 as f32;
_8 = !_9;
_23 = _19.0 as i32;
_1 = _20.2 as isize;
_22 = _4 as i128;
_24 = _11.2.fld4 as i32;
_19 = (_2, _11.2.fld6);
RET = core::ptr::addr_of_mut!((*RET));
match _11.2.fld4 {
6659 => bb10,
_ => bb8
}
}
bb10 = {
_15.2 = _23 as u64;
_20.0 = _11.2.fld5;
_15.1 = _15.2 as u8;
_11.1 = &_11.2.fld1;
_11.3.0 = (-1866406835585987031_i64) | 1603670411171142102_i64;
_26 = _5 != _5;
(*RET) = '\u{41ba7}';
_11.2.fld0 = _26;
_20.2 = !_3;
_27.0 = &_11.2.fld3;
Goto(bb11)
}
bb11 = {
_16 = _15.1 as i8;
_20 = (_11.2.fld5, _15.1, _15.2);
_16 = _11.3.2 << _19.0;
_25 = _5 & _8;
_11.2.fld2 = [_11.3.0];
_9 = _25 * _1;
(*RET) = '\u{b6035}';
_24 = !_23;
_28.2.0.2 = !_11.3.2;
(*RET) = '\u{1adbf}';
_11.2.fld3 = 551643527_u32 * 1199010381_u32;
_2 = _19.0;
_11.2.fld4 = _16 as i16;
_11.1 = &_11.2.fld1;
_13 = -_4;
_27.3 = Move(_11.0);
_27.3 = core::ptr::addr_of_mut!(_19);
_11.1 = &_11.2.fld1;
_11.0 = core::ptr::addr_of_mut!(_19);
_20.0 = _11.2.fld5 * _11.2.fld5;
_2 = _19.0;
_29 = _15.1;
Goto(bb12)
}
bb12 = {
_15.1 = _29 + _29;
_20.2 = _22 as u64;
_15.0 = -_11.2.fld5;
_3 = !_15.2;
_20.2 = _15.2;
_18 = 250418913734461643864472652535871137534_u128 as isize;
_11.2.fld5 = _15.0 - _15.0;
(*RET) = '\u{7b358}';
_11.2.fld3 = 1664593776_u32 + 3056024766_u32;
_26 = _11.2.fld0 ^ _11.2.fld0;
_13 = _20.2 as isize;
_31 = _2 & _19.0;
_11.2.fld6 = !_19.1;
_30 = core::ptr::addr_of_mut!(_10);
(*RET) = '\u{c8862}';
_24 = _19.1 as i32;
Goto(bb13)
}
bb13 = {
_27.0 = &_11.2.fld3;
_20.0 = _22 as f32;
_28.2.0.0 = _11.3.0;
_16 = !_28.2.0.2;
_1 = !_6;
_4 = -_8;
_10 = '\u{71f32}';
_10 = '\u{5870f}';
_33.fld1 = _20.1;
_28.2.0.0 = _11.3.0 >> _15.2;
(*_30) = '\u{7f144}';
_30 = Move(RET);
_33.fld2 = _10 as u128;
_26 = _11.2.fld0;
_33.fld2 = _28.2.0.0 as u128;
Call(_20 = fn11(Move(_27.0), _5, _28.2.0.0, _15.1, _11.2.fld0, _4, _15, _15.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_24 = !_23;
_16 = _28.2.0.2;
_15.0 = _20.0;
_26 = _11.2.fld0;
_11.2.fld4 = _15.2 as i16;
_5 = _11.2.fld3 as isize;
_16 = !_11.3.2;
_2 = _31 ^ _31;
_30 = core::ptr::addr_of_mut!(_10);
_15 = (_11.2.fld5, _33.fld1, _20.2);
(*_30) = '\u{6966}';
_28.2.0.0 = _11.3.0 * _11.3.0;
_15.0 = _20.0 * _20.0;
_20.0 = _11.2.fld5 - _15.0;
_35 = [_11.3.0,_28.2.0.0,_28.2.0.0];
_7 = _6 * _6;
_18 = _25;
_15 = (_20.0, _33.fld1, _20.2);
Call(_11.2.fld6 = core::intrinsics::bswap(_19.1), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_23 = _20.1 as i32;
_11.2.fld4 = (-6994_i16) >> _24;
_9 = _15.1 as isize;
_40.fld2 = _11.3.0;
_22 = _11.2.fld7 | _11.2.fld7;
(*_30) = '\u{8422}';
RET = Move(_30);
_40.fld3 = !_31;
_15.1 = _29 + _33.fld1;
_11.2.fld5 = _7 as f32;
_41.0 = _2 + _19.0;
_29 = _15.1 & _20.1;
_20.0 = _33.fld2 as f32;
_11.2.fld3 = 2031519777_u32 * 1750099555_u32;
_15 = (_11.2.fld5, _20.1, _3);
_11.2.fld6 = !_19.1;
_8 = _33.fld2 as isize;
_33 = Adt19 { fld0: _11.2.fld0,fld1: _20.1,fld2: 177703846037025927788036687311358561269_u128 };
_13 = _40.fld2 as isize;
_40 = Adt21 { fld0: _33.fld0,fld1: _20.2,fld2: _11.3.0,fld3: _41.0 };
_40 = Adt21 { fld0: _11.2.fld0,fld1: _15.2,fld2: _28.2.0.0,fld3: _2 };
Goto(bb16)
}
bb16 = {
Call(_43 = dump_var(10_usize, 18_usize, Move(_18), 9_usize, Move(_9), 23_usize, Move(_23), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(10_usize, 16_usize, Move(_16), 4_usize, Move(_4), 1_usize, Move(_1), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(10_usize, 19_usize, Move(_19), 5_usize, Move(_5), 35_usize, Move(_35), 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: &'static u32,mut _2: isize,mut _3: i64,mut _4: u8,mut _5: bool,mut _6: isize,mut _7: (f32, u8, u64),mut _8: f32) -> (f32, u8, u64) {
mir! {
type RET = (f32, u8, u64);
let _9: char;
let _10: ();
let _11: ();
{
RET.1 = _4 | _4;
_3 = 5297415994886960794_i64;
RET.0 = _8 * _8;
_7.0 = -_8;
RET = (_7.0, _4, _7.2);
RET.1 = _7.1 << RET.2;
RET.0 = (-65_i8) as f32;
RET.2 = _7.2;
RET.1 = _4;
RET.0 = _3 as f32;
_7.2 = !RET.2;
_6 = _2 - _2;
_7.1 = _4;
_7.2 = RET.2;
_2 = 61442_u16 as isize;
RET.0 = _8 * _7.0;
_7 = RET;
_8 = _7.0 + _7.0;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(11_usize, 2_usize, Move(_2), 5_usize, Move(_5), 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(3225761160_u32), std::hint::black_box('\u{e832f}'), std::hint::black_box((-72_isize)), std::hint::black_box(29021_u16), std::hint::black_box((-4853882400246357886_i64)), std::hint::black_box(6254439307620190245_u64));
                
            }
impl PrintFDebug for Adt19{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt19{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt19 {
fld0: bool,
fld1: u8,
fld2: u128,
}
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt21{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt21 {
fld0: bool,
fld1: u64,
fld2: i64,
fld3: usize,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: f32,
fld1: [bool; 2],
fld2: *const f64,
fld3: *const ((i64, *const [i16; 5], i8), i8, Adt19),

},
Variant1{
fld0: u128,
fld1: (i64, *const [i16; 5], i8),
fld2: usize,
fld3: i8,
fld4: i16,
fld5: *mut [i32; 2],

},
Variant2{
fld0: *mut *const f64,
fld1: char,
fld2: i16,
fld3: i8,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: *mut (usize, u16),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: bool,
fld1: Adt53,
fld2: [i64; 1],
fld3: u32,
fld4: i16,
fld5: f32,
fld6: u16,
fld7: i128,
}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){unsafe{printf("Adt66::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt66 {
Variant0{
fld0: (((i64, *const [i16; 5], i8),), i32),
fld1: usize,
fld2: [u8; 7],
fld3: (i64, *const [i16; 5], i8),

},
Variant1{
fld0: *mut *const f64,
fld1: *const usize,
fld2: Adt58,
fld3: ((i64, *const [i16; 5], i8),),
fld4: (((i64, *const [i16; 5], i8),), i32),
fld5: Adt55,
fld6: i64,
fld7: [bool; 2],

},
Variant2{
fld0: Adt53,
fld1: (Adt21, [i32; 2], i32),
fld2: (i64, *const [i16; 5], i8),
fld3: f64,
fld4: ([i32; 2], (i64, *const [i16; 5], i8), bool, *mut i16),
fld5: [u8; 7],
fld6: Adt19,

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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt74 {
Variant0{
fld0: *const u32,

},
Variant1{
fld0: *mut *const f64,
fld1: u16,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){unsafe{printf("Adt75::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: *const usize,
fld1: *const [i16; 5],
fld2: Adt21,
fld3: *mut [i32; 2],
fld4: [i16; 5],
fld5: Adt19,

},
Variant1{
fld0: i64,
fld1: Adt55,
fld2: (f32, u8, u64),
fld3: *mut i16,
fld4: i128,
fld5: [i8; 4],

},
Variant2{
fld0: ((i64, *const [i16; 5], i8), i8, Adt19),
fld1: i32,
fld2: [u64; 5],

},
Variant3{
fld0: Adt55,
fld1: ((*const [i16; 5],), *const [i16; 5], ((i64, *const [i16; 5], i8),)),
fld2: Adt66,
fld3: [i16; 5],

}}

