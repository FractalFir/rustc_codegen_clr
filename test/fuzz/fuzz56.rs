#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics)]
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
            printf(c"%i".as_ptr(),*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf(c"[".as_ptr());
            for b in self{
                b.printf_debug();
                printf(c",".as_ptr());
            }
            printf(c"]".as_ptr());
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf(c"%lx%lx".as_ptr(), (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self as usize);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf(c"true".as_ptr());
            }
            else{
                printf(c"false".as_ptr());
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf(c"()".as_ptr());
        }
    } 
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",)".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c",".as_ptr());
            self.11.printf_debug();
            printf(c")".as_ptr());
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
            printf(c"fn%u:_%u = ".as_ptr(),f,var0);
            val0.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var1);
            val1.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var2);
            val2.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var3);
            val3.printf_debug();
            printf(c"\n".as_ptr());
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: u128,mut _2: u16) -> Adt49 {
mir! {
type RET = Adt49;
let _3: isize;
let _4: [bool; 1];
let _5: u64;
let _6: i8;
let _7: ();
let _8: ();
{
_1 = 9084516509566910046168383132456809284_u128;
_2 = 1003865811438999930_usize as u16;
_2 = (-547358356_i32) as u16;
_3 = (-9223372036854775808_isize);
_1 = (-1649663914194381788_i64) as u128;
_2 = 8047_u16 & 23576_u16;
_1 = 98437855808083948578988376978320375279_u128 | 140186297795819111837073394533793385393_u128;
_3 = -97_isize;
_1 = 221497095265620947551355689560066444893_u128 + 244119327145755907578593645672559668791_u128;
_5 = !13684456963796482552_u64;
_4 = [true];
Call(RET = fn1(_3, _1, _2, _1, _5, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [197_u8,94_u8];
_2 = 60386_u16 * 30134_u16;
_2 = 5132434342617358239_usize as u16;
_4 = [false];
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [149_u8,251_u8];
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [112_u8,142_u8];
_4 = [false];
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [253_u8,208_u8];
_1 = 148897883462496378986231904420771034635_u128;
_2 = !11653_u16;
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [152_u8,139_u8];
_3 = !9223372036854775807_isize;
_1 = 131317944128143439519285944283671119227_u128 + 275325272580793220107695974721737371399_u128;
_1 = 273580343806013328059174410233804021261_u128;
_1 = !60735750420258673079040742983372498705_u128;
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [160_u8,15_u8];
_6 = (-85_i8) * (-119_i8);
_2 = 24049_u16;
_3 = (-9223372036854775808_isize) << _6;
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [118_u8,181_u8];
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [60_u8,235_u8];
_4 = [true];
place!(Field::<[u8; 2]>(Variant(RET, 2), 0)) = [115_u8,250_u8];
_5 = 23769_i16 as u64;
Goto(bb2)
}
bb2 = {
Call(_7 = dump_var(0_usize, 5_usize, Move(_5), 1_usize, Move(_1), 3_usize, Move(_3), 8_usize, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: u128,mut _3: u16,mut _4: u128,mut _5: u64,mut _6: u16,mut _7: u16) -> Adt49 {
mir! {
type RET = Adt49;
let _8: Adt49;
let _9: isize;
let _10: i16;
let _11: ((i64, (i64,)), u16);
let _12: Adt62;
let _13: [i16; 4];
let _14: f32;
let _15: (i64,);
let _16: [u16; 6];
let _17: char;
let _18: isize;
let _19: [i64; 1];
let _20: f64;
let _21: *mut char;
let _22: u8;
let _23: (char, usize, u64, i128, i8);
let _24: isize;
let _25: [u8; 2];
let _26: Adt57;
let _27: [bool; 1];
let _28: [u8; 7];
let _29: (usize,);
let _30: *mut i64;
let _31: bool;
let _32: [i64; 6];
let _33: *mut char;
let _34: f64;
let _35: Adt50;
let _36: u8;
let _37: u8;
let _38: isize;
let _39: ((i64, (i64,)), u16);
let _40: Adt51;
let _41: f32;
let _42: u32;
let _43: [usize; 8];
let _44: (*mut i64,);
let _45: ([i16; 4], [u16; 1], (i64, (i64,)), u128);
let _46: isize;
let _47: f64;
let _48: f64;
let _49: ();
let _50: ();
{
_2 = _4;
_1 = '\u{bfaa0}' as isize;
_5 = 11525977327246528465_u64;
_5 = 6602533792856405376_u64;
_6 = _5 as u16;
_3 = _7;
_2 = _4 >> _4;
_1 = 9194_i16 as isize;
_3 = _7 >> _4;
_1 = !(-9223372036854775808_isize);
_6 = _3 | _3;
_1 = 97_isize << _6;
_2 = _4;
_4 = false as u128;
_4 = _2;
_9 = _1;
_7 = _6;
_2 = _4 | _4;
_4 = _2 - _2;
Goto(bb1)
}
bb1 = {
_11.0.1 = (8808755041665943516_i64,);
_11.0.0 = _11.0.1.0 ^ _11.0.1.0;
_7 = !_6;
_3 = _7 << _6;
_6 = _3 >> _4;
Goto(bb2)
}
bb2 = {
_14 = 28478_i16 as f32;
_7 = _3 - _3;
_11.0.0 = _11.0.1.0;
_4 = _2;
_1 = _7 as isize;
_5 = _14 as u64;
_11.1 = _6;
_13 = [(-1473_i16),(-10923_i16),16361_i16,29985_i16];
_13 = [952_i16,16342_i16,83_i16,2578_i16];
_15.0 = -_11.0.1.0;
_11.0.1.0 = _15.0 - _11.0.0;
_16 = [_7,_7,_11.1,_6,_6,_7];
_1 = 160976613969636936714580571243440788387_i128 as isize;
_1 = !_9;
_16 = [_3,_6,_6,_3,_3,_7];
_6 = _15.0 as u16;
_7 = !_11.1;
_2 = true as u128;
_4 = _2 & _2;
_11.1 = _3 - _7;
_14 = (-127_i8) as f32;
_2 = _9 as u128;
Goto(bb3)
}
bb3 = {
_17 = '\u{68ea2}';
_20 = _11.0.1.0 as f64;
_15 = _11.0.1;
_11.0.1 = (_11.0.0,);
_9 = _1;
_16 = [_7,_11.1,_7,_7,_11.1,_11.1];
_21 = core::ptr::addr_of_mut!(_17);
_20 = 3092959946_u32 as f64;
_7 = !_11.1;
_22 = !89_u8;
(*_21) = '\u{84618}';
_10 = 26608_i16;
_15 = (_11.0.1.0,);
_14 = _3 as f32;
_11.0 = (_15.0, _15);
_7 = _3 + _11.1;
_20 = _2 as f64;
_1 = !_9;
_21 = core::ptr::addr_of_mut!(_17);
_23.3 = !165664818657682671296591710672289486214_i128;
_11.1 = _7;
_22 = !123_u8;
Goto(bb4)
}
bb4 = {
_23.4 = (-32_i8);
_25 = [_22,_22];
_11.0.1 = (_15.0,);
_2 = !_4;
_23.0 = _17;
_6 = _11.1;
_18 = -_1;
_11.0 = (_15.0, _15);
_23.2 = _5;
_22 = 224_u8 * 140_u8;
_4 = _2 + _2;
_23.3 = 132165300394264104557323221733028505607_i128;
Goto(bb5)
}
bb5 = {
_5 = _23.2 & _23.2;
_24 = 1690390114_u32 as isize;
_22 = 149_u8;
_24 = _9;
_11.0 = (_15.0, _15);
_22 = !12_u8;
_5 = _23.2 * _23.2;
_11.1 = 4140039937_u32 as u16;
_20 = _6 as f64;
_23.3 = _9 as i128;
_11.0.1 = (_15.0,);
_18 = _1;
_14 = _22 as f32;
_13 = [_10,_10,_10,_10];
_10 = true as i16;
Call(_5 = core::intrinsics::bswap(_23.2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
(*_21) = _23.0;
_18 = _1;
_29.0 = 6_usize;
_2 = !_4;
_25 = [_22,_22];
_18 = _23.4 as isize;
_10 = _5 as i16;
_29.0 = 4974207786356152023_usize ^ 4498978790477653967_usize;
_31 = true | true;
_33 = _21;
_15 = _11.0.1;
_23.3 = 11326878053879177653791339090734592739_i128;
_33 = _21;
_18 = _11.0.0 as isize;
(*_21) = _23.0;
_23.1 = _29.0;
Call(_20 = fn2(_16, (*_21), (*_21)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_24 = 2612672054_u32 as isize;
_33 = _21;
(*_21) = _23.0;
_25 = [_22,_22];
_18 = _9 | _1;
_16 = [_6,_7,_7,_6,_6,_6];
Goto(bb8)
}
bb8 = {
_16 = [_7,_7,_6,_7,_7,_7];
_18 = _9 >> _7;
_15.0 = -_11.0.1.0;
_15.0 = _11.0.0 - _11.0.1.0;
_14 = _23.1 as f32;
_23.4 = 97_i8 & 22_i8;
_25 = [_22,_22];
_22 = 11_u8 * 228_u8;
_17 = _23.0;
_11.1 = _6 * _7;
_29 = (_23.1,);
match _23.3 {
11326878053879177653791339090734592739 => bb9,
_ => bb2
}
}
bb9 = {
_19 = [_15.0];
_26 = Adt57::Variant1 { fld0: _18 };
SetDiscriminant(_26, 1);
_31 = !true;
_34 = _20;
place!(Field::<isize>(Variant(_26, 1), 0)) = _15.0 as isize;
_29 = (_23.1,);
_2 = _4 | _4;
_34 = _5 as f64;
_32 = [_11.0.0,_15.0,_15.0,_15.0,_15.0,_11.0.0];
_11.0 = (_15.0, _15);
_16 = [_7,_11.1,_11.1,_6,_7,_11.1];
_23.1 = _29.0 ^ _29.0;
_11.0.0 = _11.0.1.0 ^ _11.0.1.0;
_30 = core::ptr::addr_of_mut!(_11.0.0);
_11.0.1.0 = !(*_30);
_23.4 = (-127_i8) & (-12_i8);
SetDiscriminant(_26, 0);
_31 = _6 > _11.1;
_2 = _4 ^ _4;
place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_26, 0), 0)).0 = _23.2;
(*_21) = _23.0;
(*_33) = _23.0;
_25 = [_22,_22];
Call(place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_26, 0), 0)).2 = core::intrinsics::transmute(_11.0.1.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_23.1 = _29.0 << _11.1;
place!(Field::<f32>(Variant(_26, 0), 3)) = _14 + _14;
place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_26, 0), 0)).0 = _5;
_27 = [_31];
_36 = _23.1 as u8;
_11.0 = (_15.0, _15);
Goto(bb11)
}
bb11 = {
_4 = _2 - _2;
_37 = _36 ^ _36;
_39.0.1 = ((*_30),);
_23.2 = _5 * Field::<(u64, (i64,), [i16; 4])>(Variant(_26, 0), 0).0;
_21 = core::ptr::addr_of_mut!((*_33));
_16 = [_11.1,_7,_11.1,_7,_7,_11.1];
_23.3 = (-106734560870868074471550022234362980096_i128);
match _23.3 {
0 => bb4,
1 => bb9,
233547806050070388991824585197405231360 => bb12,
_ => bb10
}
}
bb12 = {
_19 = [_39.0.1.0];
(*_21) = _23.0;
_39.0 = (_15.0, _11.0.1);
_39.0 = ((*_30), _11.0.1);
_23.1 = _29.0;
_34 = _20 - _20;
_30 = core::ptr::addr_of_mut!(place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_26, 0), 0)).1.0);
_39.0 = _11.0;
_38 = -_18;
(*_21) = _23.0;
_11.0.0 = _4 as i64;
_11.1 = _3;
_39.0.1.0 = -_11.0.0;
_41 = -Field::<f32>(Variant(_26, 0), 3);
_26 = Adt57::Variant1 { fld0: _38 };
_15 = (_11.0.0,);
_42 = 2192865849_u32 >> _37;
_28 = [_36,_36,_36,_36,_36,_36,_37];
_11 = (_39.0, _7);
_3 = _6 * _11.1;
_39 = (_11.0, _3);
_29 = (_23.1,);
_20 = _34;
_29 = (_23.1,);
_24 = _18 << _39.1;
_13 = [_10,_10,_10,_10];
Goto(bb13)
}
bb13 = {
_16 = [_6,_6,_3,_39.1,_3,_6];
_11.1 = !_6;
_45.1 = [_7];
_7 = _10 as u16;
_44.0 = core::ptr::addr_of_mut!(_45.2.0);
SetDiscriminant(_26, 1);
_40 = Adt51::Variant1 { fld0: _31 };
place!(Field::<isize>(Variant(_26, 1), 0)) = _24;
SetDiscriminant(_26, 2);
place!(Field::<bool>(Variant(_40, 1), 0)) = !_31;
_39 = (_11.0, _3);
_6 = !_39.1;
_14 = _41;
_40 = Adt51::Variant2 { fld0: _25 };
_45.0 = _13;
_31 = _38 >= _24;
_15 = (_39.0.1.0,);
_40 = Adt51::Variant1 { fld0: _31 };
place!(Field::<[char; 3]>(Variant(_26, 2), 1)) = [_17,(*_33),_23.0];
_11.0 = (_15.0, _15);
place!(Field::<[char; 3]>(Variant(_26, 2), 1)) = [(*_21),(*_21),(*_21)];
_46 = _41 as isize;
_19 = [_11.0.0];
_15.0 = -_11.0.1.0;
Goto(bb14)
}
bb14 = {
_18 = -_1;
_39.0 = (_11.0.0, _15);
_45.2 = (_39.0.1.0, _15);
_45.3 = (*_33) as u128;
_39.0.1 = _11.0.1;
_23 = ((*_21), _29.0, _5, 136055055029291242456796466234081720671_i128, 106_i8);
Goto(bb15)
}
bb15 = {
_6 = _3 & _39.1;
_39.0.1 = (_15.0,);
_23.3 = (-62718163740020264320236823276653434629_i128);
RET = Adt49::Variant2 { fld0: _25,fld1: _30 };
_45.2.1.0 = _45.2.0;
_41 = _14 - _14;
_39.0.1 = (_45.2.1.0,);
_34 = _20;
_30 = Field::<*mut i64>(Variant(RET, 2), 1);
_11 = _39;
_23.3 = _23.4 as i128;
(*_21) = _23.0;
_23 = ((*_21), _29.0, _5, 23916579571302530232368565974059434165_i128, 58_i8);
Goto(bb16)
}
bb16 = {
Call(_49 = dump_var(1_usize, 4_usize, Move(_4), 37_usize, Move(_37), 38_usize, Move(_38), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(1_usize, 27_usize, Move(_27), 17_usize, Move(_17), 32_usize, Move(_32), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(1_usize, 7_usize, Move(_7), 23_usize, Move(_23), 25_usize, Move(_25), 29_usize, Move(_29)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_49 = dump_var(1_usize, 46_usize, Move(_46), 9_usize, Move(_9), 28_usize, Move(_28), 11_usize, Move(_11)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u16; 6],mut _2: char,mut _3: char) -> f64 {
mir! {
type RET = f64;
let _4: i32;
let _5: isize;
let _6: u64;
let _7: usize;
let _8: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _9: Adt54;
let _10: f64;
let _11: f32;
let _12: [i64; 6];
let _13: isize;
let _14: [i16; 4];
let _15: (char, usize, u64, i128, i8);
let _16: [i16; 4];
let _17: f64;
let _18: i16;
let _19: [u8; 7];
let _20: (char, usize, u64, i128, i8);
let _21: [bool; 1];
let _22: i32;
let _23: [i16; 4];
let _24: char;
let _25: *mut char;
let _26: *mut *mut i64;
let _27: (usize,);
let _28: i32;
let _29: i16;
let _30: Adt49;
let _31: Adt51;
let _32: *mut *mut i64;
let _33: Adt65;
let _34: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _35: isize;
let _36: ();
let _37: ();
{
_1 = [31654_u16,12069_u16,50081_u16,64280_u16,65262_u16,42876_u16];
_4 = 2092185224_i32 ^ 1613267444_i32;
_4 = 1346535137_i32 - 745967787_i32;
_3 = _2;
_4 = (-1648542370_i32);
_5 = 120_isize & (-32_isize);
_5 = -101_isize;
RET = 56634074047375514765967444397156952965_u128 as f64;
_4 = !2136599523_i32;
RET = 4099565261_u32 as f64;
RET = 242_u8 as f64;
RET = 991195252115452999_u64 as f64;
_2 = _3;
_3 = _2;
_7 = !6_usize;
_6 = !10144116663247736671_u64;
RET = _4 as f64;
_4 = (-2124754004_i32) - (-964936894_i32);
_1 = [49190_u16,4015_u16,61288_u16,26450_u16,62906_u16,44960_u16];
_2 = _3;
RET = _4 as f64;
_7 = 5_usize * 5_usize;
_2 = _3;
Call(_1 = fn3(RET, _4, _3, _4, _4, _7, _7, _7, _4, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _3;
_8.3.0 = 100_u8 as f32;
_8.1 = 3374784058_u32 * 1605189436_u32;
RET = 25116_i16 as f64;
_8.2 = (-2248_i16) * 16037_i16;
_8.5 = [23184_u16];
_8.5 = [7612_u16];
RET = _4 as f64;
_8.4 = (-128309533078224840139582549956845045041_i128) ^ (-119299277990483816503437423684025733939_i128);
_5 = (-9223372036854775808_isize);
RET = _7 as f64;
_10 = -RET;
_1 = [54031_u16,4394_u16,62852_u16,1750_u16,35273_u16,20668_u16];
Goto(bb2)
}
bb2 = {
_1 = [22157_u16,65521_u16,8251_u16,1439_u16,48555_u16,36198_u16];
_8.1 = !44912767_u32;
_8.3.1 = _3;
_1 = [65417_u16,4821_u16,44691_u16,21656_u16,6720_u16,38599_u16];
Goto(bb3)
}
bb3 = {
RET = _10;
_2 = _3;
Call(_7 = core::intrinsics::bswap(15472691820213093353_usize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8.0.1.0 = (-3583133939261910035_i64) | 6005488187441390444_i64;
_11 = _8.3.0;
RET = _10;
_8.2 = 14489_i16;
_8.3.0 = _11 + _11;
_8.0.1.0 = (-6601753781554135782_i64) + (-3345181011244423384_i64);
_8.0.0 = _8.0.1.0;
_5 = -125_isize;
_6 = false as u64;
_2 = _8.3.1;
_8.0.1.0 = 49917_u16 as i64;
_10 = RET;
_13 = 7959_u16 as isize;
_13 = !_5;
_7 = _6 as usize;
_8.4 = 142635018731041991597676038777290039818_i128 & (-3820829957670731881358857256677073791_i128);
match _8.2 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
14489 => bb9,
_ => bb8
}
}
bb5 = {
RET = _10;
_2 = _3;
Call(_7 = core::intrinsics::bswap(15472691820213093353_usize), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_1 = [22157_u16,65521_u16,8251_u16,1439_u16,48555_u16,36198_u16];
_8.1 = !44912767_u32;
_8.3.1 = _3;
_1 = [65417_u16,4821_u16,44691_u16,21656_u16,6720_u16,38599_u16];
Goto(bb3)
}
bb7 = {
_2 = _3;
_8.3.0 = 100_u8 as f32;
_8.1 = 3374784058_u32 * 1605189436_u32;
RET = 25116_i16 as f64;
_8.2 = (-2248_i16) * 16037_i16;
_8.5 = [23184_u16];
_8.5 = [7612_u16];
RET = _4 as f64;
_8.4 = (-128309533078224840139582549956845045041_i128) ^ (-119299277990483816503437423684025733939_i128);
_5 = (-9223372036854775808_isize);
RET = _7 as f64;
_10 = -RET;
_1 = [54031_u16,4394_u16,62852_u16,1750_u16,35273_u16,20668_u16];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_8.3 = (_11, _2);
_8.0.1.0 = false as i64;
_8.5 = [46372_u16];
_8.5 = [50793_u16];
_12 = [_8.0.1.0,_8.0.0,_8.0.0,_8.0.1.0,_8.0.1.0,_8.0.1.0];
_8.4 = (-106791278374627954642775186020790136970_i128);
_15.3 = !_8.4;
_15.2 = _6;
_8.3.1 = _2;
_15.3 = _7 as i128;
_13 = _5;
_15 = (_3, _7, _6, _8.4, 74_i8);
_8.0.1.0 = _8.0.0 * _8.0.0;
_8.3.0 = -_11;
_15.0 = _8.3.1;
_17 = RET - RET;
_16 = [_8.2,_8.2,_8.2,_8.2];
_6 = _15.2 + _15.2;
_20.1 = _3 as usize;
_20.2 = !_6;
_5 = _13 * _13;
_5 = _13 & _13;
_12 = [_8.0.1.0,_8.0.1.0,_8.0.1.0,_8.0.0,_8.0.0,_8.0.1.0];
_15.1 = !_7;
_6 = _20.2 + _20.2;
Call(_8.4 = core::intrinsics::transmute(_15.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10 = _17 + _17;
_21 = [false];
_20.2 = _6 << _8.0.0;
_16 = [_8.2,_8.2,_8.2,_8.2];
_15.2 = _6;
_8.4 = _4 as i128;
_18 = _15.2 as i16;
_20.4 = _20.1 as i8;
RET = 64_u8 as f64;
_23 = [_18,_18,_18,_8.2];
_8.0.1.0 = -_8.0.0;
_19 = [215_u8,94_u8,129_u8,183_u8,93_u8,12_u8,9_u8];
_20.3 = 63200621963039498344119250290125441559_u128 as i128;
Goto(bb11)
}
bb11 = {
_8.3 = (_11, _15.0);
_20.0 = _2;
_7 = _20.1 << _8.2;
_10 = _17;
_20.2 = _15.2 & _15.2;
_19 = [195_u8,88_u8,20_u8,212_u8,139_u8,176_u8,235_u8];
_18 = _8.2 - _8.2;
_15.1 = _7 + _7;
_22 = -_4;
_5 = false as isize;
_10 = _17;
_20.0 = _3;
_14 = [_18,_18,_18,_18];
RET = -_10;
RET = -_10;
match _15.4 {
0 => bb1,
1 => bb7,
2 => bb8,
74 => bb12,
_ => bb4
}
}
bb12 = {
match _15.4 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
74 => bb13,
_ => bb9
}
}
bb13 = {
_20.0 = _8.3.1;
_15 = _20;
_25 = core::ptr::addr_of_mut!(_8.3.1);
_8.0.1 = (_8.0.0,);
_18 = _8.2;
_8.3 = (_11, _20.0);
_8.5 = [50151_u16];
_8.0.1.0 = _8.0.0;
_8.1 = 679137876_u32 << _6;
_24 = _2;
RET = _17;
_12 = [_8.0.1.0,_8.0.1.0,_8.0.0,_8.0.1.0,_8.0.0,_8.0.0];
_20 = (_15.0, _7, _15.2, _8.4, _15.4);
_20.1 = _7;
(*_25) = _24;
_7 = _20.1 | _20.1;
_8.4 = -_20.3;
_20.2 = !_15.2;
_8.3 = (_11, _20.0);
_4 = _3 as i32;
Goto(bb14)
}
bb14 = {
_17 = RET;
_11 = _13 as f32;
_1 = [54957_u16,64068_u16,64805_u16,38912_u16,51059_u16,4521_u16];
_13 = -_5;
_9 = Adt54::Variant3 { fld0: _15.4,fld1: _25,fld2: _23 };
_15.4 = _20.4 | _20.4;
_4 = _11 as i32;
place!(Field::<*mut char>(Variant(_9, 3), 1)) = core::ptr::addr_of_mut!((*_25));
_18 = _11 as i16;
place!(Field::<[i16; 4]>(Variant(_9, 3), 2)) = _16;
_5 = _13 + _13;
_2 = _8.3.1;
_13 = _5;
_12 = [_8.0.1.0,_8.0.0,_8.0.0,_8.0.1.0,_8.0.1.0,_8.0.1.0];
_22 = _20.1 as i32;
_34.3.0 = _11;
_34.0 = (_8.0.0, _8.0.1);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(2_usize, 19_usize, Move(_19), 23_usize, Move(_23), 16_usize, Move(_16), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(2_usize, 14_usize, Move(_14), 6_usize, Move(_6), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(2_usize, 13_usize, Move(_13), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: f64,mut _2: i32,mut _3: char,mut _4: i32,mut _5: i32,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: i32,mut _10: char,mut _11: usize) -> [u16; 6] {
mir! {
type RET = [u16; 6];
let _12: u128;
let _13: f32;
let _14: [i16; 4];
let _15: f64;
let _16: u16;
let _17: u8;
let _18: (usize,);
let _19: Adt56;
let _20: char;
let _21: [u16; 6];
let _22: bool;
let _23: u64;
let _24: ([i16; 4], [u16; 1], (i64, (i64,)), u128);
let _25: Adt55;
let _26: *const (usize, u16, *const i64, bool);
let _27: Adt49;
let _28: char;
let _29: (char, usize, u64, i128, i8);
let _30: f32;
let _31: f64;
let _32: f64;
let _33: isize;
let _34: Adt60;
let _35: f32;
let _36: bool;
let _37: i16;
let _38: *const (usize, u16, *const i64, bool);
let _39: (*const (usize, u16, *const i64, bool), &'static [u8; 2]);
let _40: [char; 3];
let _41: Adt51;
let _42: isize;
let _43: u16;
let _44: [u16; 1];
let _45: ();
let _46: ();
{
_5 = _1 as i32;
_13 = 243_u8 as f32;
_6 = !_11;
_14 = [31857_i16,(-24455_i16),5135_i16,(-25656_i16)];
_15 = _1 - _1;
RET = [9130_u16,28344_u16,50443_u16,36327_u16,4029_u16,13889_u16];
_12 = 63942_u16 as u128;
_7 = _8;
_2 = _5;
_11 = _12 as usize;
_4 = _5 - _9;
_1 = _15 + _15;
_12 = 256962131772854010944965280162343217044_u128;
_5 = _4 ^ _4;
Goto(bb1)
}
bb1 = {
_10 = _3;
_11 = !_6;
_4 = -_2;
_8 = !_7;
RET = [30428_u16,5824_u16,57897_u16,51843_u16,39174_u16,50341_u16];
_13 = 7653480546577988875_u64 as f32;
_12 = 47492767550855548394031746459910189771_u128 << _5;
RET = [24867_u16,55304_u16,15272_u16,39230_u16,44287_u16,46688_u16];
_8 = _11;
_6 = _7;
_7 = _11;
_15 = _1 - _1;
Call(_15 = fn4(_1, _8, _9, _8, _9, _12, _12, _5, _11, _4, _10, _5, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _4;
_10 = _3;
_21 = [3631_u16,37079_u16,21032_u16,15086_u16,59054_u16,13825_u16];
_11 = !_7;
RET = _21;
_11 = _6;
_10 = _3;
_24.3 = _12;
Call(_5 = fn5(_14, _12, _24.3, _14, _21, _8, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_20 = _10;
_2 = 698899894_u32 as i32;
_17 = 68_u8;
_24.0 = [(-31868_i16),(-22109_i16),19295_i16,(-2912_i16)];
RET = _21;
_18.0 = _7;
_24.1 = [45551_u16];
_3 = _10;
_21 = [13543_u16,10310_u16,36474_u16,65415_u16,24824_u16,52769_u16];
_21 = [27607_u16,23925_u16,53051_u16,9911_u16,21155_u16,1455_u16];
_5 = _9 << _12;
_20 = _3;
Call(_24.2.1 = fn18(_24.0, _24.3, _6, _15, _2, _5, _15, _12, _5, _14, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24.2.0 = _24.2.1.0 | _24.2.1.0;
_3 = _20;
_23 = !9441457348645862751_u64;
_8 = _6 * _18.0;
_15 = -_1;
_21 = [26884_u16,19236_u16,13796_u16,3601_u16,50797_u16,37879_u16];
_28 = _20;
_23 = 9754185249570160782_u64 >> _24.2.0;
Goto(bb5)
}
bb5 = {
_11 = _6 >> _24.2.1.0;
_28 = _3;
_6 = _11 | _11;
_29.1 = !_6;
_3 = _20;
_4 = !_5;
_17 = 75_u8 | 54_u8;
_31 = _24.2.1.0 as f64;
_24.3 = _12;
_2 = 44_i8 as i32;
RET = _21;
_32 = _31;
_10 = _28;
Call(_24.3 = fn19(_24.2.1, _23, _24.2.0, _24.2, _32, _28), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_29 = (_3, _11, _23, 42968414994621146556099851313190075834_i128, (-29_i8));
_30 = _13 * _13;
_29.0 = _28;
_29.0 = _10;
_29.2 = !_23;
_13 = -_30;
_23 = _29.2;
_29 = (_10, _6, _23, (-39958570460682224721394082003487561111_i128), 3_i8);
_31 = _32 - _1;
_24.1 = [30103_u16];
_9 = !_4;
_29.3 = (-22034_i16) as i128;
_13 = 8570_i16 as f32;
_24.2.1 = (_24.2.0,);
_1 = _32;
_28 = _29.0;
_24.3 = 9691_i16 as u128;
match _29.4 {
3 => bb8,
_ => bb7
}
}
bb7 = {
_11 = _6 >> _24.2.1.0;
_28 = _3;
_6 = _11 | _11;
_29.1 = !_6;
_3 = _20;
_4 = !_5;
_17 = 75_u8 | 54_u8;
_31 = _24.2.1.0 as f64;
_24.3 = _12;
_2 = 44_i8 as i32;
RET = _21;
_32 = _31;
_10 = _28;
Call(_24.3 = fn19(_24.2.1, _23, _24.2.0, _24.2, _32, _28), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_17 = 151_u8 << _5;
_13 = _30;
_37 = _28 as i16;
_2 = _9 - _5;
_2 = _11 as i32;
_29 = (_28, _11, _23, (-33216548263514636969107620719981434592_i128), 58_i8);
_42 = _6 as isize;
Goto(bb9)
}
bb9 = {
_16 = 44994_u16 | 12641_u16;
_18.0 = _13 as usize;
RET = [_16,_16,_16,_16,_16,_16];
_16 = 16535_u16 ^ 52872_u16;
Goto(bb10)
}
bb10 = {
_16 = _31 as u16;
_3 = _29.0;
_24.2.1.0 = _24.2.0 ^ _24.2.0;
_24.2.1 = (_24.2.0,);
_21 = [_16,_16,_16,_16,_16,_16];
_40 = [_3,_29.0,_10];
_18 = (_6,);
_24.0 = [_37,_37,_37,_37];
_32 = -_1;
_29.0 = _3;
_34.fld0 = [_24.2.0];
_4 = !_2;
match _29.4 {
0 => bb4,
1 => bb2,
2 => bb9,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
58 => bb16,
_ => bb15
}
}
bb11 = {
_20 = _10;
_2 = 698899894_u32 as i32;
_17 = 68_u8;
_24.0 = [(-31868_i16),(-22109_i16),19295_i16,(-2912_i16)];
RET = _21;
_18.0 = _7;
_24.1 = [45551_u16];
_3 = _10;
_21 = [13543_u16,10310_u16,36474_u16,65415_u16,24824_u16,52769_u16];
_21 = [27607_u16,23925_u16,53051_u16,9911_u16,21155_u16,1455_u16];
_5 = _9 << _12;
_20 = _3;
Call(_24.2.1 = fn18(_24.0, _24.3, _6, _15, _2, _5, _15, _12, _5, _14, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_17 = 151_u8 << _5;
_13 = _30;
_37 = _28 as i16;
_2 = _9 - _5;
_2 = _11 as i32;
_29 = (_28, _11, _23, (-33216548263514636969107620719981434592_i128), 58_i8);
_42 = _6 as isize;
Goto(bb9)
}
bb13 = {
_11 = _6 >> _24.2.1.0;
_28 = _3;
_6 = _11 | _11;
_29.1 = !_6;
_3 = _20;
_4 = !_5;
_17 = 75_u8 | 54_u8;
_31 = _24.2.1.0 as f64;
_24.3 = _12;
_2 = 44_i8 as i32;
RET = _21;
_32 = _31;
_10 = _28;
Call(_24.3 = fn19(_24.2.1, _23, _24.2.0, _24.2, _32, _28), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_24.2.0 = _24.2.1.0 | _24.2.1.0;
_3 = _20;
_23 = !9441457348645862751_u64;
_8 = _6 * _18.0;
_15 = -_1;
_21 = [26884_u16,19236_u16,13796_u16,3601_u16,50797_u16,37879_u16];
_28 = _20;
_23 = 9754185249570160782_u64 >> _24.2.0;
Goto(bb5)
}
bb15 = {
_9 = _4;
_10 = _3;
_21 = [3631_u16,37079_u16,21032_u16,15086_u16,59054_u16,13825_u16];
_11 = !_7;
RET = _21;
_11 = _6;
_10 = _3;
_24.3 = _12;
Call(_5 = fn5(_14, _12, _24.3, _14, _21, _8, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_23 = !_29.2;
_28 = _29.0;
_24.0 = _14;
_7 = _29.3 as usize;
_40 = [_10,_20,_29.0];
_24.1 = [_16];
_29.3 = _32 as i128;
_44 = _24.1;
_43 = _16;
_1 = _32 * _31;
_3 = _20;
_33 = _42 | _42;
_29.1 = _11 << _43;
Goto(bb17)
}
bb17 = {
Call(_45 = dump_var(3_usize, 18_usize, Move(_18), 8_usize, Move(_8), 20_usize, Move(_20), 42_usize, Move(_42)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(3_usize, 14_usize, Move(_14), 23_usize, Move(_23), 10_usize, Move(_10), 33_usize, Move(_33)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(3_usize, 12_usize, Move(_12), 2_usize, Move(_2), 5_usize, Move(_5), 28_usize, Move(_28)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(3_usize, 21_usize, Move(_21), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f64,mut _2: usize,mut _3: i32,mut _4: usize,mut _5: i32,mut _6: u128,mut _7: u128,mut _8: i32,mut _9: usize,mut _10: i32,mut _11: char,mut _12: i32,mut _13: u128) -> f64 {
mir! {
type RET = f64;
let _14: ();
let _15: ();
{
RET = _1 * _1;
_2 = _9;
_4 = _9;
_12 = _3 - _8;
_7 = _6;
_5 = _8;
_13 = _7 | _7;
_1 = (-9223372036854775808_isize) as f64;
_13 = _7;
_5 = -_12;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(4_usize, 4_usize, Move(_4), 7_usize, Move(_7), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(4_usize, 2_usize, Move(_2), 11_usize, Move(_11), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [i16; 4],mut _2: u128,mut _3: u128,mut _4: [i16; 4],mut _5: [u16; 6],mut _6: usize,mut _7: [u16; 6]) -> i32 {
mir! {
type RET = i32;
let _8: [usize; 8];
let _9: isize;
let _10: char;
let _11: Adt63;
let _12: *mut char;
let _13: isize;
let _14: f32;
let _15: u128;
let _16: (u64, (i64,), [i16; 4]);
let _17: f64;
let _18: (usize,);
let _19: isize;
let _20: u32;
let _21: *const i64;
let _22: (u16, i16);
let _23: isize;
let _24: Adt52;
let _25: Adt59;
let _26: char;
let _27: (f32, char);
let _28: Adt63;
let _29: bool;
let _30: char;
let _31: char;
let _32: u64;
let _33: *const (usize, u16, *const i64, bool);
let _34: u32;
let _35: [i16; 4];
let _36: i16;
let _37: Adt53;
let _38: [i128; 3];
let _39: Adt55;
let _40: Adt55;
let _41: *const (usize, u16, *const i64, bool);
let _42: f32;
let _43: (usize,);
let _44: char;
let _45: u8;
let _46: [i64; 6];
let _47: Adt58;
let _48: Adt58;
let _49: isize;
let _50: [u16; 1];
let _51: ();
let _52: ();
{
_3 = !_2;
_3 = _2;
Goto(bb1)
}
bb1 = {
_5 = [2193_u16,42914_u16,29156_u16,49978_u16,53631_u16,36415_u16];
_7 = _5;
_8 = [_6,_6,_6,_6,_6,_6,_6,_6];
Call(_3 = fn6(_4, _6, _2, _6, _2, _2, _2, _5, _6, _6, _7, _5, _5, _7, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = -1533802062_i32;
_8 = [_6,_6,_6,_6,_6,_6,_6,_6];
_3 = _2;
_9 = !81_isize;
_3 = !_2;
_7 = [10335_u16,4080_u16,55560_u16,28182_u16,41335_u16,61337_u16];
_1 = [(-4362_i16),5288_i16,(-26387_i16),17010_i16];
_2 = !_3;
RET = !1342631417_i32;
_3 = _2;
_1 = _4;
RET = !90162995_i32;
_1 = [(-25087_i16),15884_i16,(-2462_i16),(-22790_i16)];
_9 = !(-111_isize);
_6 = (-14517_i16) as usize;
Call(RET = fn15(_3, _5, _7, _5, _7, _3, _2, _7, _2, _9, _4, _4, _4, _6, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _2 * _2;
_2 = '\u{3b6fe}' as u128;
_12 = core::ptr::addr_of_mut!(_10);
_4 = [32447_i16,26006_i16,(-12927_i16),(-5457_i16)];
_7 = [61030_u16,39112_u16,12439_u16,10139_u16,50084_u16,50850_u16];
RET = -191657801_i32;
(*_12) = '\u{78d11}';
_16.0 = !7591408534213068487_u64;
_16.0 = 761148877_u32 as u64;
_16.1.0 = (-4548997638157338664_i64);
_16.1.0 = -8207086632298213018_i64;
_13 = _9 * _9;
(*_12) = '\u{bf473}';
RET = 151238826_i32;
_7 = [10061_u16,24938_u16,57689_u16,32839_u16,34782_u16,511_u16];
_16.2 = _1;
_16.0 = 7606342062986379417_u64 & 10346762184031822509_u64;
_10 = '\u{a09e4}';
RET = 1318529301_i32 << _3;
RET = (-73_i8) as i32;
_16.1.0 = (-5410786099709793129_i64);
_16.1 = ((-9128056885867200289_i64),);
_6 = 7890759207447526647_usize;
_4 = [(-30465_i16),(-19383_i16),4199_i16,(-10539_i16)];
_16.0 = !13567307589911136841_u64;
Goto(bb4)
}
bb4 = {
_15 = RET as u128;
_13 = -_9;
_10 = '\u{4003a}';
(*_12) = '\u{ffc67}';
_6 = 0_usize & 7_usize;
_7 = [41365_u16,22333_u16,16793_u16,36385_u16,25201_u16,56551_u16];
_17 = (-35_i8) as f64;
_1 = _16.2;
_4 = [3491_i16,(-26042_i16),(-23386_i16),(-19378_i16)];
_14 = (-8948_i16) as f32;
_13 = -_9;
_9 = -_13;
_14 = _13 as f32;
_4 = _16.2;
_5 = [15046_u16,19094_u16,35876_u16,17981_u16,37586_u16,48704_u16];
RET = !1198422903_i32;
(*_12) = '\u{cac74}';
_15 = _3 ^ _3;
_10 = '\u{11ecf}';
_6 = 1147601350021605414_usize * 14758329233799639673_usize;
_9 = _13 - _13;
_17 = RET as f64;
Call(_18.0 = core::intrinsics::bswap(_6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16.1 = (6004575222245499912_i64,);
_19 = _9;
_17 = _16.0 as f64;
_16.2 = _4;
_16.1 = ((-4546607338883561618_i64),);
_5 = _7;
_19 = _13 - _13;
(*_12) = '\u{338ab}';
(*_12) = '\u{20763}';
_12 = core::ptr::addr_of_mut!((*_12));
_9 = _13;
_8 = [_6,_6,_6,_6,_6,_6,_6,_6];
_16.0 = !11490452401503267968_u64;
_3 = true as u128;
_3 = 27_u8 as u128;
_13 = _19 & _19;
_5 = [49348_u16,53996_u16,30582_u16,29303_u16,55558_u16,55272_u16];
_9 = (-97323966993227670392982335872440822711_i128) as isize;
Call(_22.1 = core::intrinsics::bswap(5466_i16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_22.0 = RET as u16;
_22 = (44304_u16, (-17372_i16));
_20 = !1218442845_u32;
_8 = [_6,_6,_6,_6,_6,_6,_6,_6];
_6 = !0_usize;
_5 = _7;
_3 = _15;
_18 = (_6,);
_23 = 59_i8 as isize;
_2 = !_3;
(*_12) = '\u{76eea}';
_16.1.0 = RET as i64;
(*_12) = '\u{64261}';
_8 = [_18.0,_18.0,_6,_6,_18.0,_18.0,_6,_18.0];
_17 = _16.0 as f64;
_12 = core::ptr::addr_of_mut!((*_12));
_12 = core::ptr::addr_of_mut!((*_12));
_23 = _13;
match _22.0 {
0 => bb1,
1 => bb2,
2 => bb4,
44304 => bb8,
_ => bb7
}
}
bb7 = {
_16.1 = (6004575222245499912_i64,);
_19 = _9;
_17 = _16.0 as f64;
_16.2 = _4;
_16.1 = ((-4546607338883561618_i64),);
_5 = _7;
_19 = _13 - _13;
(*_12) = '\u{338ab}';
(*_12) = '\u{20763}';
_12 = core::ptr::addr_of_mut!((*_12));
_9 = _13;
_8 = [_6,_6,_6,_6,_6,_6,_6,_6];
_16.0 = !11490452401503267968_u64;
_3 = true as u128;
_3 = 27_u8 as u128;
_13 = _19 & _19;
_5 = [49348_u16,53996_u16,30582_u16,29303_u16,55558_u16,55272_u16];
_9 = (-97323966993227670392982335872440822711_i128) as isize;
Call(_22.1 = core::intrinsics::bswap(5466_i16), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_22 = (17267_u16, (-11229_i16));
_7 = _5;
_2 = _3;
_21 = core::ptr::addr_of!(_16.1.0);
_24 = Adt52::Variant3 { fld0: _21,fld1: 124841689497326095202404843819461480787_i128,fld2: 227_u8 };
_22 = (22278_u16, (-13111_i16));
_19 = _23 >> _3;
_9 = _19;
_18 = (_6,);
_8 = [_18.0,_6,_6,_18.0,_6,_18.0,_18.0,_18.0];
RET = -1693011522_i32;
_22 = (27335_u16, 996_i16);
_5 = _7;
_26 = (*_12);
_20 = !3103706675_u32;
_10 = _26;
place!(Field::<u8>(Variant(_24, 3), 2)) = 187_u8;
_19 = _23;
(*_12) = _26;
_27 = (_14, (*_12));
_9 = _23 << _15;
Goto(bb9)
}
bb9 = {
_13 = -_9;
place!(Field::<i128>(Variant(_24, 3), 1)) = (-23341629031797197654991925291650093512_i128) - 133678225306829606904222950547435974385_i128;
_22.0 = 33139_u16 & 16939_u16;
(*_12) = _26;
_28 = Adt63::Variant1 { fld0: _18 };
_1 = _4;
_16.0 = 18377351885593327546_u64;
_8 = [Field::<(usize,)>(Variant(_28, 1), 0).0,Field::<(usize,)>(Variant(_28, 1), 0).0,_6,Field::<(usize,)>(Variant(_28, 1), 0).0,_6,Field::<(usize,)>(Variant(_28, 1), 0).0,_6,Field::<(usize,)>(Variant(_28, 1), 0).0];
place!(Field::<i128>(Variant(_24, 3), 1)) = 90422010678551918022088487693376674324_i128 >> _19;
_11 = Move(_28);
Goto(bb10)
}
bb10 = {
_26 = (*_12);
_26 = _27.1;
_27.1 = _26;
_16.1 = (890576131498103305_i64,);
(*_12) = _27.1;
place!(Field::<(usize,)>(Variant(_11, 1), 0)) = (_18.0,);
(*_12) = _26;
place!(Field::<(usize,)>(Variant(_11, 1), 0)).0 = !_6;
_23 = !_13;
_16.1 = ((-4973422581932703690_i64),);
Call(_27.1 = fn16(_9, (*_21), _13, _24, _13, _9, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_27.0 = _14 * _14;
_27.1 = _26;
_9 = _23;
_32 = _16.0 ^ _16.0;
_22.1 = (-31794_i16) << _2;
_16.1.0 = (-8487731893872249772_i64) >> _3;
_28 = Adt63::Variant1 { fld0: Field::<(usize,)>(Variant(_11, 1), 0) };
_26 = _27.1;
_9 = _20 as isize;
_22.0 = !6808_u16;
_7 = _5;
RET = !1146818418_i32;
(*_21) = (-1051002512195631948_i64);
_16.2 = _4;
_24 = Adt52::Variant3 { fld0: _21,fld1: (-18933746942599767893688921445552754736_i128),fld2: 193_u8 };
place!(Field::<(usize,)>(Variant(_28, 1), 0)) = Field::<(usize,)>(Variant(_11, 1), 0);
_30 = _27.1;
_4 = [_22.1,_22.1,_22.1,_22.1];
_2 = _3;
_30 = _27.1;
_27 = (_14, _10);
_29 = _2 > _3;
(*_12) = _26;
RET = _6 as i32;
_13 = _23 & _19;
_12 = core::ptr::addr_of_mut!(_26);
Goto(bb12)
}
bb12 = {
_16.1 = (7181891378325833595_i64,);
_24 = Adt52::Variant0 { fld0: _8,fld1: _30,fld2: _21 };
_27 = (_14, _26);
SetDiscriminant(_28, 1);
_5 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_34 = _32 as u32;
_27.1 = _30;
_10 = _27.1;
(*_21) = !8341980475320075568_i64;
_6 = _18.0;
SetDiscriminant(_24, 3);
_34 = _20 & _20;
SetDiscriminant(_11, 0);
_32 = _16.0;
_7 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_11 = Adt63::Variant1 { fld0: _18 };
_10 = (*_12);
_22.0 = 49583_u16 * 60931_u16;
(*_21) = RET as i64;
_22.0 = 51124_u16 & 54924_u16;
place!(Field::<u8>(Variant(_24, 3), 2)) = 229_u8 | 128_u8;
_31 = (*_12);
_21 = core::ptr::addr_of!(_16.1.0);
_27.1 = _31;
_13 = !_19;
Goto(bb13)
}
bb13 = {
place!(Field::<(usize,)>(Variant(_28, 1), 0)).0 = _6;
_11 = Move(_28);
_17 = _3 as f64;
_22.0 = _22.1 as u16;
SetDiscriminant(_11, 0);
_43.0 = !_6;
_14 = _27.0 - _27.0;
_16.0 = RET as u64;
_16.2 = [_22.1,_22.1,_22.1,_22.1];
place!(Field::<*const i64>(Variant(_24, 3), 0)) = _21;
_21 = Field::<*const i64>(Variant(_24, 3), 0);
_9 = !_23;
_5 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_13 = _23 >> _2;
place!(Field::<char>(Variant(_11, 0), 1)) = _27.1;
_7 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
(*_12) = _10;
_17 = 122_i8 as f64;
(*_12) = _10;
(*_12) = _30;
place!(Field::<char>(Variant(_11, 0), 1)) = _27.1;
_36 = _22.1 << _13;
(*_12) = _31;
_45 = Field::<u8>(Variant(_24, 3), 2) - Field::<u8>(Variant(_24, 3), 2);
_28 = Adt63::Variant1 { fld0: _43 };
Goto(bb14)
}
bb14 = {
_34 = _20;
_18.0 = _6;
_43 = (_18.0,);
SetDiscriminant(_28, 1);
_31 = Field::<char>(Variant(_11, 0), 1);
_31 = _26;
_12 = core::ptr::addr_of_mut!(_26);
place!(Field::<*const i64>(Variant(_24, 3), 0)) = _21;
_32 = 38713622556831937187706607901801543612_i128 as u64;
_31 = _26;
_16.1 = (1653241962202186889_i64,);
_6 = _18.0;
_20 = _3 as u32;
place!(Field::<i8>(Variant(_11, 0), 3)) = -102_i8;
place!(Field::<(*mut i64,)>(Variant(_11, 0), 4)).0 = core::ptr::addr_of_mut!((*_21));
_46 = [(*_21),(*_21),(*_21),(*_21),(*_21),_16.1.0];
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(5_usize, 34_usize, Move(_34), 13_usize, Move(_13), 46_usize, Move(_46), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(5_usize, 29_usize, Move(_29), 30_usize, Move(_30), 15_usize, Move(_15), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(5_usize, 8_usize, Move(_8), 31_usize, Move(_31), 2_usize, Move(_2), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(5_usize, 3_usize, Move(_3), 6_usize, Move(_6), 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i16; 4],mut _2: usize,mut _3: u128,mut _4: usize,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: [u16; 6],mut _9: usize,mut _10: usize,mut _11: [u16; 6],mut _12: [u16; 6],mut _13: [u16; 6],mut _14: [u16; 6],mut _15: [i16; 4]) -> u128 {
mir! {
type RET = u128;
let _16: [bool; 1];
let _17: &'static [u8; 2];
let _18: u8;
let _19: f32;
let _20: ();
let _21: ();
{
_7 = 227462189_i32 as u128;
_5 = _6 << _6;
_15 = _1;
_13 = [4198_u16,60547_u16,9965_u16,10973_u16,58449_u16,7200_u16];
_14 = [5179_u16,9168_u16,60681_u16,34174_u16,43860_u16,19136_u16];
_7 = 9223372036854775807_isize as u128;
_13 = [30233_u16,22160_u16,13548_u16,34758_u16,45652_u16,42798_u16];
_12 = [2290_u16,57316_u16,56467_u16,38060_u16,45197_u16,41592_u16];
_8 = [46973_u16,35890_u16,39297_u16,35044_u16,26729_u16,55960_u16];
_9 = !_4;
RET = _3;
_2 = _4;
_13 = [16943_u16,44431_u16,36296_u16,57908_u16,39440_u16,49055_u16];
RET = !_3;
_5 = _3 * _6;
_4 = _2 << _5;
_11 = _8;
_18 = 120_u8 << _3;
RET = _7;
Call(RET = fn7(_5, _6, _4, _4, _3, _13, _6, _2, _12, _3, _18, _12, _15, _15, _5, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [6491_u16,29518_u16,1672_u16,1505_u16,57817_u16,45627_u16];
_12 = [11448_u16,30586_u16,24033_u16,40517_u16,20903_u16,23449_u16];
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(6_usize, 8_usize, Move(_8), 3_usize, Move(_3), 14_usize, Move(_14), 9_usize, Move(_9)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(6_usize, 7_usize, Move(_7), 12_usize, Move(_12), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u128,mut _2: u128,mut _3: usize,mut _4: usize,mut _5: u128,mut _6: [u16; 6],mut _7: u128,mut _8: usize,mut _9: [u16; 6],mut _10: u128,mut _11: u8,mut _12: [u16; 6],mut _13: [i16; 4],mut _14: [i16; 4],mut _15: u128,mut _16: usize) -> u128 {
mir! {
type RET = u128;
let _17: ([i16; 4], [u16; 1], (i64, (i64,)), u128);
let _18: Adt53;
let _19: Adt61;
let _20: [u8; 7];
let _21: Adt55;
let _22: (u16, i16);
let _23: u16;
let _24: (f32, char);
let _25: (i64, (i64,));
let _26: bool;
let _27: (i64,);
let _28: (i64,);
let _29: u64;
let _30: bool;
let _31: *mut char;
let _32: [i64; 1];
let _33: isize;
let _34: Adt49;
let _35: ();
let _36: ();
{
_6 = _12;
_3 = !_4;
_8 = _3 + _4;
_12 = [40885_u16,32803_u16,12973_u16,14825_u16,32161_u16,13809_u16];
_8 = _16 & _16;
_7 = _15;
_14 = [23877_i16,4137_i16,26014_i16,(-28507_i16)];
_8 = !_3;
RET = !_7;
_17.3 = !_5;
_17.1 = [25446_u16];
_17.0 = [(-21652_i16),25952_i16,32583_i16,19771_i16];
_2 = '\u{edb08}' as u128;
RET = _17.3 + _15;
_14 = [14444_i16,(-14465_i16),(-121_i16),12717_i16];
RET = _1;
_12 = [29024_u16,59060_u16,24805_u16,30917_u16,39319_u16,20493_u16];
_7 = !_15;
RET = !_5;
_11 = 227_u8 >> _1;
_17.0 = [27588_i16,(-12727_i16),(-19832_i16),(-17352_i16)];
_14 = [(-5813_i16),20515_i16,8827_i16,32368_i16];
_9 = _6;
_22.0 = (-103_i8) as u16;
Goto(bb1)
}
bb1 = {
_7 = _5 >> _5;
_20 = [_11,_11,_11,_11,_11,_11,_11];
_17.0 = [(-19590_i16),(-1211_i16),23075_i16,(-1112_i16)];
_20 = [_11,_11,_11,_11,_11,_11,_11];
_5 = !_17.3;
_16 = _3 | _4;
_23 = _22.0;
_9 = [_22.0,_23,_22.0,_23,_23,_23];
_4 = !_16;
_1 = RET;
_23 = _22.0 ^ _22.0;
_24.0 = _5 as f32;
_15 = RET;
_16 = RET as usize;
_26 = !true;
Call(_21 = fn8(_8, _8, _8, _3, _15, _4, _17.0, _13, _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_25.1.0 = (-2_i8) as i64;
SetDiscriminant(_21, 2);
_14 = _17.0;
_17.3 = _1;
_1 = '\u{a8bb}' as u128;
_6 = _12;
_17.1 = [_23];
_24.1 = '\u{b96f4}';
_30 = _26;
place!(Field::<i8>(Variant(_21, 2), 3)) = -(-77_i8);
_27 = _25.1;
_9 = _12;
_7 = _17.3 << _4;
Goto(bb3)
}
bb3 = {
_17.0 = [3290_i16,(-18818_i16),5391_i16,19157_i16];
_1 = _7 ^ _5;
_9 = _6;
_28 = (_27.0,);
_29 = 2862750118_u32 as u64;
RET = !_7;
place!(Field::<u128>(Variant(_21, 2), 5)) = 89601469617706787926207917923850202312_i128 as u128;
_17.0 = [11292_i16,21121_i16,27597_i16,748_i16];
_17.2.0 = !_25.1.0;
RET = _17.3 * _1;
_9 = [_23,_23,_22.0,_23,_22.0,_23];
_25 = (_17.2.0, _27);
place!(Field::<*mut char>(Variant(_21, 2), 4)) = core::ptr::addr_of_mut!(_24.1);
_33 = (-9223372036854775808_isize);
_31 = Field::<*mut char>(Variant(_21, 2), 4);
_7 = _1;
place!(Field::<[i64; 1]>(Variant(_21, 2), 1)) = [_27.0];
(*_31) = '\u{de051}';
place!(Field::<*mut char>(Variant(_21, 2), 4)) = core::ptr::addr_of_mut!(_24.1);
_20 = [_11,_11,_11,_11,_11,_11,_11];
Goto(bb4)
}
bb4 = {
Call(_35 = dump_var(7_usize, 15_usize, Move(_15), 26_usize, Move(_26), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_35 = dump_var(7_usize, 12_usize, Move(_12), 2_usize, Move(_2), 7_usize, Move(_7), 28_usize, Move(_28)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_35 = dump_var(7_usize, 3_usize, Move(_3), 9_usize, Move(_9), 23_usize, Move(_23), 13_usize, Move(_13)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: u128,mut _6: usize,mut _7: [i16; 4],mut _8: [i16; 4],mut _9: usize,mut _10: u128) -> Adt55 {
mir! {
type RET = Adt55;
let _11: Adt52;
let _12: isize;
let _13: char;
let _14: Adt63;
let _15: i32;
let _16: [i64; 6];
let _17: *mut *mut i64;
let _18: f32;
let _19: &'static [u8; 2];
let _20: char;
let _21: (i64,);
let _22: *mut *mut i64;
let _23: u64;
let _24: (i64,);
let _25: f32;
let _26: ();
let _27: ();
{
_5 = _10 >> _1;
_5 = !_10;
_2 = _6 * _4;
_4 = _2;
_9 = _3 >> _10;
_2 = _4 * _6;
_10 = _5;
_4 = !_6;
_10 = _5;
_1 = _4 ^ _4;
_9 = _6 & _6;
_3 = !_6;
_6 = 52823_u16 as usize;
_8 = [1371_i16,(-23373_i16),27809_i16,(-9212_i16)];
_5 = _10;
_6 = !_9;
_13 = '\u{efb09}';
_3 = _9 >> _4;
_8 = [1212_i16,8569_i16,12258_i16,3331_i16];
_8 = [(-2247_i16),22426_i16,(-6477_i16),(-12598_i16)];
Call(_2 = fn9(_9, _6, _6, _3, _3, _10, _3, _9, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = -9223372036854775807_isize;
_15 = !(-1085063665_i32);
_2 = _4;
_16 = [3789941964828199919_i64,8561960466908278710_i64,(-5598687552581831041_i64),5973157406064318492_i64,9104567323014421513_i64,686081469402579336_i64];
Call(_13 = fn10(_9, _4, _1, _3, _9, _9, _6, _3, _6, _6, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = 63_u16 as usize;
_4 = 12163109255586105027_u64 as usize;
_18 = _12 as f32;
_5 = !_10;
_10 = _5 * _5;
_7 = [(-6878_i16),(-29615_i16),895_i16,(-31900_i16)];
_1 = !_3;
_18 = _12 as f32;
_6 = (-4848411646940121431_i64) as usize;
_5 = _13 as u128;
_1 = !_9;
_6 = !_1;
_18 = (-7356344539093610786_i64) as f32;
_9 = _3 << _10;
_8 = _7;
_15 = -471490720_i32;
_10 = _12 as u128;
_18 = 1033749653_u32 as f32;
_15 = (-744791720_i32) & (-1681105167_i32);
_8 = [3911_i16,14323_i16,29639_i16,5950_i16];
_5 = !_10;
_15 = -1401650551_i32;
_10 = 6261712538646373806_i64 as u128;
_1 = _6;
_6 = _9 | _1;
_12 = -(-9223372036854775808_isize);
_3 = !_9;
_10 = _5 & _5;
_12 = 9223372036854775807_isize;
match _12 {
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
_12 = -9223372036854775807_isize;
_15 = !(-1085063665_i32);
_2 = _4;
_16 = [3789941964828199919_i64,8561960466908278710_i64,(-5598687552581831041_i64),5973157406064318492_i64,9104567323014421513_i64,686081469402579336_i64];
Call(_13 = fn10(_9, _4, _1, _3, _9, _9, _6, _3, _6, _6, _4), ReturnTo(bb2), UnwindUnreachable())
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
_16 = [4120003032695721505_i64,(-5675398933736986028_i64),8601019721964294252_i64,4262679807499773561_i64,5304663034757129283_i64,5795948178831932538_i64];
_16 = [4517314748834306157_i64,(-6283321531450818544_i64),(-5457366859061376632_i64),(-3169909120620617300_i64),(-5801870415299034199_i64),1396222487842418790_i64];
_15 = 498963806_i32;
Goto(bb11)
}
bb11 = {
_6 = _3 & _3;
Goto(bb12)
}
bb12 = {
_16 = [(-8021888633774161406_i64),(-8202472465029793620_i64),(-6690313784341942443_i64),3717728589773599440_i64,(-65585183825371754_i64),(-2548900489490120370_i64)];
_5 = _10;
_8 = [28940_i16,962_i16,30997_i16,(-4957_i16)];
_3 = _9 ^ _9;
_10 = _13 as u128;
_16 = [6925439280035700428_i64,1999988014878652814_i64,2804672397298793714_i64,(-5273649074249172964_i64),424733716426418936_i64,(-5144976330431519746_i64)];
_16 = [(-4037943476496329328_i64),(-4004605556535070069_i64),1244119803769539873_i64,(-332849691862115006_i64),(-7500751245722100942_i64),(-706090574698852783_i64)];
_9 = _1 << _6;
_5 = !_10;
_8 = [(-21623_i16),17283_i16,17138_i16,10204_i16];
_21 = ((-191924282206793181_i64),);
_15 = -2008316868_i32;
_10 = !_5;
Goto(bb13)
}
bb13 = {
_21 = ((-6065776588447821330_i64),);
_10 = !_5;
_1 = 29356_i16 as usize;
_10 = _5;
RET = Adt55::Variant1 { fld0: _21 };
_3 = !_9;
_2 = _9 >> _9;
_8 = [11005_i16,11118_i16,(-15485_i16),9888_i16];
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = (_21.0,);
_5 = _10 ^ _10;
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = _21;
_3 = _9;
_5 = _10;
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = _21;
_21 = (Field::<(i64,)>(Variant(RET, 1), 0).0,);
_4 = 116259487160668974_u64 as usize;
_10 = _5;
_21.0 = Field::<(i64,)>(Variant(RET, 1), 0).0;
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = (_21.0,);
_12 = 9223372036854775807_isize >> _3;
_10 = _5;
match _21.0 {
340282366920938463457308830843320390126 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
SetDiscriminant(RET, 2);
place!(Field::<u128>(Variant(RET, 2), 5)) = !_5;
_10 = _5 | _5;
_5 = _10;
_20 = _13;
place!(Field::<i8>(Variant(RET, 2), 3)) = -42_i8;
place!(Field::<*mut char>(Variant(RET, 2), 4)) = core::ptr::addr_of_mut!(_13);
Goto(bb16)
}
bb16 = {
RET = Adt55::Variant1 { fld0: _21 };
_6 = _3 >> _3;
_24.0 = 9951_i16 as i64;
SetDiscriminant(RET, 1);
_1 = _3;
_7 = [10514_i16,(-22060_i16),29559_i16,15878_i16];
_20 = _13;
_6 = !_3;
_6 = _3;
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = (_21.0,);
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = (_21.0,);
_5 = _10 >> _2;
_2 = _9 - _3;
_7 = [(-13994_i16),17291_i16,19635_i16,32706_i16];
_23 = 10206817473161390161_u64 >> _3;
place!(Field::<(i64,)>(Variant(RET, 1), 0)) = _21;
_13 = _20;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(8_usize, 1_usize, Move(_1), 7_usize, Move(_7), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(8_usize, 13_usize, Move(_13), 4_usize, Move(_4), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_26 = dump_var(8_usize, 21_usize, Move(_21), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: u128,mut _7: usize,mut _8: usize,mut _9: usize) -> usize {
mir! {
type RET = usize;
let _10: ();
let _11: ();
{
_4 = !_2;
_7 = _8 - _1;
_1 = false as usize;
_8 = _2 + _9;
_7 = 44_i8 as usize;
RET = _4 ^ _8;
_7 = !RET;
_3 = '\u{103226}' as usize;
_8 = (-1935526258_i32) as usize;
_3 = RET;
_1 = 2165751544111769150_u64 as usize;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(9_usize, 5_usize, Move(_5), 8_usize, Move(_8), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: usize,mut _11: usize) -> char {
mir! {
type RET = char;
let _12: [i128; 3];
let _13: (u64, (i64,), [i16; 4]);
let _14: isize;
let _15: Adt57;
let _16: isize;
let _17: i8;
let _18: isize;
let _19: [u16; 1];
let _20: *const (i64,);
let _21: Adt57;
let _22: &'static [u8; 2];
let _23: Adt61;
let _24: [char; 3];
let _25: (*const (usize, u16, *const i64, bool), &'static [u8; 2]);
let _26: (f32, char);
let _27: [i64; 1];
let _28: (usize, u16, *const i64, bool);
let _29: isize;
let _30: bool;
let _31: Adt52;
let _32: [i16; 4];
let _33: f64;
let _34: (char, usize, u64, i128, i8);
let _35: f64;
let _36: f32;
let _37: isize;
let _38: [u16; 6];
let _39: char;
let _40: i32;
let _41: isize;
let _42: ();
let _43: ();
{
_7 = _9 >> _8;
RET = '\u{27560}';
_9 = _5;
Goto(bb1)
}
bb1 = {
_5 = !_9;
_2 = 330413961978768112126087606815998070375_u128 as usize;
Goto(bb2)
}
bb2 = {
_7 = !_3;
_6 = _10 << _9;
RET = '\u{4c117}';
RET = '\u{b3dd}';
_1 = _11;
RET = '\u{b4f81}';
_4 = !_8;
_13.1.0 = _1 as i64;
_10 = (-32_isize) as usize;
_13.2 = [(-27889_i16),11759_i16,14201_i16,(-14486_i16)];
_12 = [(-80102446201478062739595827387446283507_i128),76322485453145153929478994085647133550_i128,69250842659090716904293619393443319395_i128];
RET = '\u{9ef55}';
_9 = _5 - _7;
_9 = (-27_i8) as usize;
Call(_3 = fn11(_7, _8, _7, _8, _8, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _6 << _6;
Goto(bb4)
}
bb4 = {
_15 = Adt57::Variant1 { fld0: 9223372036854775807_isize };
_15 = Adt57::Variant1 { fld0: (-9223372036854775808_isize) };
_13.2 = [982_i16,(-20289_i16),7232_i16,24826_i16];
_13.1.0 = -2539023539062469383_i64;
_18 = -9223372036854775807_isize;
_13.0 = !12650445932627463154_u64;
_12 = [89199377414643101266340242637902873981_i128,(-68148910306663365193072266589357984543_i128),(-152040573456076058498128025023438836976_i128)];
_12 = [83300307680581339870889456533409022319_i128,136519195392473674376438248916510301035_i128,(-60714628163300503941872687865303963442_i128)];
RET = '\u{4f8c8}';
_2 = _4;
_8 = _3;
RET = '\u{f2c73}';
_15 = Adt57::Variant1 { fld0: _18 };
_20 = core::ptr::addr_of!(_13.1);
_9 = _6 | _7;
Call(_14 = fn12(_8, _5, _5, _3, _6, _11, _1, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = !_6;
_7 = _8;
RET = '\u{1c7ac}';
(*_20) = ((-9210954273146133676_i64),);
RET = '\u{d204b}';
_18 = _14;
_11 = _6;
(*_20) = ((-989639252131937787_i64),);
(*_20).0 = 2680226850485769035_i64 >> _8;
_16 = _13.0 as isize;
SetDiscriminant(_15, 0);
_6 = !_4;
_2 = _11;
place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0)).1.0 = false as i64;
_6 = _8 + _9;
_20 = core::ptr::addr_of!(place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0)).1);
RET = '\u{18973}';
_16 = _14;
_13.0 = !9689299034899548878_u64;
_18 = !_16;
_4 = !_3;
_10 = _13.0 as usize;
place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0)).0 = !_13.0;
_13.1.0 = 4101001250_u32 as i64;
_7 = !_4;
_17 = -59_i8;
Goto(bb6)
}
bb6 = {
_1 = RET as usize;
place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0)).0 = _13.0;
_13.1.0 = Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0).1.0 + Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0).1.0;
Call((*_20).0 = fn13(_4, _14, _16, _18, _8, _7, _3, _9, _14, _16, _3, _16, _14), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_13.2 = [(-4168_i16),13605_i16,10053_i16,(-1370_i16)];
_7 = 4188187680_u32 as usize;
_4 = _5 << _11;
place!(Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0)).1 = _13.1;
_27 = [Field::<(u64, (i64,), [i16; 4])>(Variant(_15, 0), 0).1.0];
_15 = Adt57::Variant1 { fld0: _14 };
_19 = [28938_u16];
Goto(bb8)
}
bb8 = {
_21 = Move(_15);
_26.1 = RET;
_28.3 = _16 == Field::<isize>(Variant(_21, 1), 0);
SetDiscriminant(_21, 0);
_20 = core::ptr::addr_of!(_13.1);
Goto(bb9)
}
bb9 = {
_7 = !_2;
_11 = _28.3 as usize;
_1 = _8 + _5;
_15 = Adt57::Variant1 { fld0: _14 };
_21 = Move(_15);
_30 = !_28.3;
_4 = !_9;
RET = _26.1;
(*_20).0 = 9_u8 as i64;
(*_20).0 = !4551175359341520914_i64;
_30 = !_28.3;
_26.0 = _13.1.0 as f32;
place!(Field::<isize>(Variant(_21, 1), 0)) = _28.3 as isize;
_28.0 = _7 * _8;
_13.2 = [(-5822_i16),(-19080_i16),22607_i16,3408_i16];
_27 = [_13.1.0];
_19 = [13285_u16];
_24 = [_26.1,RET,RET];
_29 = _16 + _18;
Call(_32 = fn14(_3, _30, _4, _29, _1, _14, _16, _18, _18, _4, _14, _3, _16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_29 = _18 ^ Field::<isize>(Variant(_21, 1), 0);
(*_20).0 = (-6976152604300922524_i64);
place!(Field::<isize>(Variant(_21, 1), 0)) = 165381082805083209446442907715028284446_i128 as isize;
_1 = !_4;
(*_20).0 = (-59663305215295460_i64);
_9 = RET as usize;
_26.1 = RET;
_18 = _14 * _29;
_19 = [24843_u16];
RET = _26.1;
match _13.1.0 {
0 => bb5,
340282366920938463463314944126552915996 => bb11,
_ => bb2
}
}
bb11 = {
_28.3 = _30;
_6 = _4;
_20 = core::ptr::addr_of!((*_20));
_6 = !_7;
_33 = 3819773146_u32 as f64;
(*_20).0 = (-7056473952261182750_i64) | 3269535729983443798_i64;
(*_20) = ((-9198797174152728372_i64),);
Goto(bb12)
}
bb12 = {
_34.2 = _13.0 >> _29;
_15 = Move(_21);
_33 = _34.2 as f64;
_34.0 = _26.1;
(*_20) = (8541329767636893186_i64,);
_16 = _33 as isize;
_13.0 = _34.2;
_35 = _33;
_17 = !(-49_i8);
_28.2 = core::ptr::addr_of!((*_20).0);
_3 = _6 >> _28.0;
(*_20).0 = _29 as i64;
_19 = [60980_u16];
SetDiscriminant(_15, 1);
_11 = _7;
_14 = _29 | _18;
_33 = _35 - _35;
_28.0 = _6 | _8;
Goto(bb13)
}
bb13 = {
_5 = !_11;
_34 = (RET, _3, _13.0, (-27392245109182519666009822623005115006_i128), _17);
_31 = Adt52::Variant1 { fld0: _26 };
SetDiscriminant(_31, 3);
place!(Field::<*const i64>(Variant(_31, 3), 0)) = core::ptr::addr_of!((*_20).0);
_34.2 = _13.0 & _13.0;
_37 = -_29;
_13.1.0 = (-1683210754177070030_i64) | (-4874255076335932020_i64);
_5 = _2 >> _14;
_28.3 = !_30;
_39 = RET;
_34.2 = _13.0;
_34 = (_26.1, _5, _13.0, 76081807647868658902648294315920488367_i128, _17);
_13.1.0 = _33 as i64;
_6 = _28.3 as usize;
place!(Field::<isize>(Variant(_15, 1), 0)) = _35 as isize;
_25.0 = core::ptr::addr_of!(_28);
_28.2 = Field::<*const i64>(Variant(_31, 3), 0);
_28 = (_5, 63048_u16, Field::<*const i64>(Variant(_31, 3), 0), _30);
_16 = _14;
Call(place!(Field::<u8>(Variant(_31, 3), 2)) = core::intrinsics::transmute(_28.3), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(10_usize, 16_usize, Move(_16), 34_usize, Move(_34), 10_usize, Move(_10), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(10_usize, 12_usize, Move(_12), 14_usize, Move(_14), 27_usize, Move(_27), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(10_usize, 2_usize, Move(_2), 1_usize, Move(_1), 17_usize, Move(_17), 39_usize, Move(_39)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(10_usize, 6_usize, Move(_6), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize) -> usize {
mir! {
type RET = usize;
let _7: ();
let _8: ();
{
RET = _2 - _3;
_2 = !RET;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(11_usize, 4_usize, Move(_4), 2_usize, Move(_2), 5_usize, Move(_5), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: usize,mut _2: usize,mut _3: usize,mut _4: usize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize) -> isize {
mir! {
type RET = isize;
let _9: (u16, i16);
let _10: [i16; 4];
let _11: [u16; 1];
let _12: isize;
let _13: [u8; 2];
let _14: (usize,);
let _15: (u64, (i64,), [i16; 4]);
let _16: ();
let _17: ();
{
_2 = 10035919186265612462671971542741685799_i128 as usize;
_2 = _6 + _7;
_9 = (46209_u16, (-20730_i16));
_3 = '\u{2eed8}' as usize;
_7 = _4 ^ _8;
_8 = _4;
_4 = _1;
RET = _9.0 as isize;
_9.0 = _2 as u16;
_7 = 43854104590171908906619514410558280114_i128 as usize;
_10 = [_9.1,_9.1,_9.1,_9.1];
_9 = (15458_u16, 2500_i16);
match _9.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
2500 => bb6,
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
_12 = RET >> _2;
_10 = [_9.1,_9.1,_9.1,_9.1];
_10 = [_9.1,_9.1,_9.1,_9.1];
_9 = (47543_u16, 10788_i16);
Call(RET = core::intrinsics::transmute(_8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_12 = (-31001577303722042245533747224282996491_i128) as isize;
_8 = _4 >> _5;
_11 = [_9.0];
_1 = '\u{ccfce}' as usize;
_6 = !_8;
_9.1 = (-4742_i16) + 13628_i16;
_3 = !_2;
_6 = 2545996805362412494_u64 as usize;
_2 = !_4;
_10 = [_9.1,_9.1,_9.1,_9.1];
_2 = _4;
_6 = _8;
_9.1 = 8654_i16;
_12 = RET + RET;
_15.1 = ((-9093972843686973438_i64),);
_13 = [140_u8,143_u8];
_15.2 = [_9.1,_9.1,_9.1,_9.1];
_15.0 = !1804695570999434136_u64;
_5 = _3 * _4;
Goto(bb8)
}
bb8 = {
Call(_16 = dump_var(12_usize, 7_usize, Move(_7), 3_usize, Move(_3), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_16 = dump_var(12_usize, 12_usize, Move(_12), 13_usize, Move(_13), 1_usize, Move(_1), 17_usize, _17), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: usize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: usize,mut _6: usize,mut _7: usize,mut _8: usize,mut _9: isize,mut _10: isize,mut _11: usize,mut _12: isize,mut _13: isize) -> i64 {
mir! {
type RET = i64;
let _14: [i64; 6];
let _15: isize;
let _16: (*mut i64,);
let _17: [i64; 1];
let _18: bool;
let _19: Adt49;
let _20: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _21: i64;
let _22: f64;
let _23: Adt55;
let _24: u128;
let _25: ([i16; 4], [u16; 1], (i64, (i64,)), u128);
let _26: char;
let _27: isize;
let _28: (i64,);
let _29: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _30: (u16, i16);
let _31: [bool; 1];
let _32: [usize; 8];
let _33: (*mut i64,);
let _34: [i16; 4];
let _35: (u16, i16);
let _36: i32;
let _37: isize;
let _38: f32;
let _39: ();
let _40: ();
{
_12 = _4;
_13 = -_12;
RET = -(-7005851871689279078_i64);
_13 = _12;
_2 = _4;
_11 = 1199917383_i32 as usize;
_3 = _12;
_8 = 31462_i16 as usize;
_7 = 4225_i16 as usize;
Goto(bb1)
}
bb1 = {
_3 = -_13;
_12 = _4 ^ _9;
_6 = 1910735055_i32 as usize;
Goto(bb2)
}
bb2 = {
_11 = RET as usize;
_5 = _1 >> _1;
RET = (-8416365234012576988_i64) ^ 308274977881974580_i64;
_3 = -_2;
_7 = _1;
_9 = 1342747770_u32 as isize;
_11 = 131_u8 as usize;
_6 = _1;
Call(_4 = core::intrinsics::transmute(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = _5;
_16.0 = core::ptr::addr_of_mut!(RET);
_1 = _6;
_9 = _13;
_12 = 841498858_i32 as isize;
_8 = '\u{36894}' as usize;
_5 = _6 ^ _6;
Call(RET = core::intrinsics::bswap((-3970886497349040909_i64)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = 103916558424107639478843169153437508675_i128 as isize;
_11 = _1;
_1 = _6 & _5;
_6 = !_11;
_14 = [RET,RET,RET,RET,RET,RET];
_7 = _5;
_20.3.1 = '\u{5d9b3}';
_15 = -_9;
_7 = !_6;
_20.4 = (-47_i8) as i128;
_20.4 = 69461022458895551523536972120208045731_i128 >> _3;
_9 = !_10;
_20.2 = !(-13237_i16);
_6 = _11;
_17 = [RET];
_15 = _10;
Goto(bb5)
}
bb5 = {
_5 = 446291684_u32 as usize;
_12 = _13 * _9;
_20.5 = [8735_u16];
_10 = 52650_u16 as isize;
_13 = !_12;
_21 = RET + RET;
_18 = true | true;
_20.0.1 = (_21,);
_13 = !_9;
_20.4 = _18 as i128;
Goto(bb6)
}
bb6 = {
_4 = _15;
_2 = _13;
_20.3.0 = 23360_u16 as f32;
_18 = true;
_25.2.1.0 = -_20.0.1.0;
_26 = _20.3.1;
_11 = !_1;
_9 = -_15;
Goto(bb7)
}
bb7 = {
_25.2 = (_20.0.1.0, _20.0.1);
_2 = _4;
_22 = 8349845324584405882_u64 as f64;
_20.1 = _18 as u32;
_15 = _3;
_9 = 1914466195_i32 as isize;
_1 = 3148414799066817330_u64 as usize;
_14 = [_21,_25.2.1.0,_25.2.0,_20.0.1.0,_20.0.1.0,_25.2.1.0];
_29.4 = _20.4;
_8 = _11 * _7;
_22 = 11885079690810119460_u64 as f64;
_20.0 = (_25.2.1.0, _25.2.1);
_29 = (_25.2, _20.1, _20.2, _20.3, _20.4, _20.5);
_20.4 = -_29.4;
_22 = _20.3.0 as f64;
_30.0 = 58846_u16 << _13;
_29.0.1 = (_29.0.0,);
RET = _20.0.0 * _21;
_29.0 = (_20.0.0, _20.0.1);
_13 = _20.3.1 as isize;
_8 = !_6;
_29.5 = _20.5;
_1 = _8;
Goto(bb8)
}
bb8 = {
_29.0.1 = (_29.0.0,);
_25.3 = 82930624875285333950790003926166818031_u128 ^ 30138398243752356051670050164262735688_u128;
_29.3.1 = _26;
_29 = (_25.2, _20.1, _20.2, _20.3, _20.4, _20.5);
_8 = _29.3.1 as usize;
RET = _26 as i64;
_22 = (-893483260_i32) as f64;
RET = _21 ^ _25.2.1.0;
_31 = [_18];
_20.0.1.0 = _22 as i64;
_30.0 = !17782_u16;
_29.2 = 119_i8 as i16;
_8 = !_1;
_4 = _12;
_31 = [_18];
Goto(bb9)
}
bb9 = {
_21 = !_20.0.0;
_5 = _11 - _7;
_10 = _2 * _15;
_11 = _22 as usize;
_29.3.0 = _20.3.0 * _20.3.0;
_29.0.1 = (_25.2.0,);
_27 = (-87_i8) as isize;
_28 = (RET,);
_29.4 = _20.4;
_17 = [RET];
_2 = _20.3.0 as isize;
_29.0 = (_25.2.0, _28);
_32 = [_6,_8,_7,_1,_7,_7,_7,_5];
_29 = _20;
Call(_6 = core::intrinsics::transmute(_15), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_25.3 = 26425223300162298065687960655750788381_u128;
_35 = (_30.0, _20.2);
_25.0 = [_35.1,_29.2,_35.1,_35.1];
_16.0 = core::ptr::addr_of_mut!(_25.2.1.0);
_29.1 = 1914989152_i32 as u32;
_38 = -_20.3.0;
_34 = [_20.2,_29.2,_20.2,_20.2];
_29.0.1 = _28;
_16.0 = core::ptr::addr_of_mut!(_20.0.0);
RET = 2652239613121973437_u64 as i64;
RET = _21 & _28.0;
_35 = (_30.0, _29.2);
_20.0.1.0 = -_29.0.1.0;
_33 = (_16.0,);
Goto(bb11)
}
bb11 = {
Call(_39 = dump_var(13_usize, 15_usize, Move(_15), 11_usize, Move(_11), 2_usize, Move(_2), 35_usize, Move(_35)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_39 = dump_var(13_usize, 34_usize, Move(_34), 6_usize, Move(_6), 26_usize, Move(_26), 8_usize, Move(_8)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_39 = dump_var(13_usize, 12_usize, Move(_12), 32_usize, Move(_32), 18_usize, Move(_18), 13_usize, Move(_13)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: usize,mut _2: bool,mut _3: usize,mut _4: isize,mut _5: usize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: usize,mut _11: isize,mut _12: usize,mut _13: isize) -> [i16; 4] {
mir! {
type RET = [i16; 4];
let _14: *const i64;
let _15: f64;
let _16: bool;
let _17: (usize,);
let _18: [u16; 1];
let _19: Adt62;
let _20: (char, usize, u64, i128, i8);
let _21: i32;
let _22: [i128; 3];
let _23: bool;
let _24: [bool; 1];
let _25: u128;
let _26: [i128; 3];
let _27: f64;
let _28: i64;
let _29: Adt65;
let _30: f32;
let _31: u8;
let _32: bool;
let _33: i16;
let _34: (i64,);
let _35: ();
let _36: ();
{
RET = [22485_i16,(-18685_i16),25838_i16,(-30452_i16)];
_11 = _8;
_6 = (-134785359451593012627409380706601058637_i128) as isize;
_9 = 333848926012940793097373999680637367213_u128 as isize;
_1 = _10 + _3;
_8 = _7 & _4;
_12 = _1;
_1 = !_12;
Goto(bb1)
}
bb1 = {
_8 = 1607_i16 as isize;
_7 = _13;
_3 = !_10;
_15 = 4267179769958671994_u64 as f64;
_15 = (-39372526050334440341720421156199262270_i128) as f64;
_6 = _7 & _13;
_9 = _13 ^ _6;
_3 = _12;
_7 = _11;
_15 = 2857166691_u32 as f64;
_6 = !_9;
_13 = _7;
_16 = !_2;
_12 = 7890_i16 as usize;
_3 = _1 | _1;
_5 = '\u{362ab}' as usize;
_13 = _9;
_16 = _2;
_13 = _9;
_4 = _7 | _7;
_11 = _4 << _9;
_5 = 4278004157286812839_i64 as usize;
RET = [(-31053_i16),29875_i16,19361_i16,(-15043_i16)];
_17.0 = !_10;
_3 = _17.0;
Goto(bb2)
}
bb2 = {
_20.4 = -91_i8;
_20.3 = (-154201264968600113786723979642336101367_i128);
_20.3 = (-7524_i16) as i128;
_5 = !_1;
_17.0 = 203_u8 as usize;
_20.0 = '\u{9df6}';
_20.2 = !8108600390168865860_u64;
_12 = _1;
_3 = _10 - _12;
_17.0 = _5;
_13 = _6 | _9;
_17.0 = _5;
_20.0 = '\u{10cf55}';
_1 = _10 ^ _10;
_15 = (-5316817879053015462_i64) as f64;
_18 = [8585_u16];
_20.0 = '\u{e6735}';
_22 = [_20.3,_20.3,_20.3];
RET = [10347_i16,32118_i16,986_i16,2449_i16];
_8 = _6 - _13;
_20 = ('\u{a3352}', _12, 4885830264325561385_u64, 15876382668092221248661708051355840997_i128, (-20_i8));
_7 = _9;
_17 = (_12,);
_10 = 64115_u16 as usize;
Goto(bb3)
}
bb3 = {
_7 = _20.4 as isize;
_24 = [_2];
_20.2 = 5963033653893039122_u64 | 15447041013874321719_u64;
_24 = [_2];
_7 = !_11;
_11 = _3 as isize;
_4 = _11;
_8 = _13 & _6;
_20.1 = !_12;
_7 = _8 - _4;
_20.3 = 34914495268194950363921968098062010727_i128 ^ (-130334989786910264864887114476263829864_i128);
_23 = _2;
Goto(bb4)
}
bb4 = {
_15 = _20.3 as f64;
RET = [31957_i16,22416_i16,(-4185_i16),(-28427_i16)];
_27 = _15 * _15;
RET = [26346_i16,(-32350_i16),(-20047_i16),32007_i16];
_15 = _27;
_13 = _11;
RET = [(-30261_i16),14100_i16,22103_i16,23647_i16];
_20 = ('\u{5043a}', _5, 15205310072036888267_u64, (-108923564944782740933197896947658107919_i128), 70_i8);
_16 = _11 > _11;
_20.4 = 26404_i16 as i8;
_2 = _16 & _23;
_27 = 2434193707787979983_i64 as f64;
_14 = core::ptr::addr_of!(_28);
_21 = !(-747821589_i32);
_1 = _17.0;
_9 = -_6;
_5 = _17.0 - _17.0;
_20.0 = '\u{bce95}';
_20.4 = _20.2 as i8;
_22 = [_20.3,_20.3,_20.3];
_7 = -_13;
_8 = -_7;
_3 = _20.0 as usize;
RET = [27369_i16,18669_i16,(-12202_i16),(-9055_i16)];
_25 = 45815_u16 as u128;
(*_14) = _25 as i64;
Goto(bb5)
}
bb5 = {
(*_14) = (-3363459786291740793_i64) >> _1;
_21 = !(-305955529_i32);
_2 = _16;
_17.0 = _1;
_20.1 = _1 * _1;
_9 = _8 - _4;
_17.0 = _20.4 as usize;
_17.0 = !_20.1;
_23 = _16;
_24 = [_16];
RET = [8586_i16,7114_i16,(-18492_i16),14136_i16];
Goto(bb6)
}
bb6 = {
_17 = (_12,);
_26 = [_20.3,_20.3,_20.3];
_11 = _15 as isize;
_10 = _13 as usize;
Goto(bb7)
}
bb7 = {
_7 = -_13;
(*_14) = (-6523218169290210656_i64) - (-1136823273351629558_i64);
Goto(bb8)
}
bb8 = {
_28 = !5129302809959621213_i64;
_20.1 = (-8013_i16) as usize;
match _20.2 {
0 => bb9,
1 => bb10,
2 => bb11,
15205310072036888267 => bb13,
_ => bb12
}
}
bb9 = {
_8 = 1607_i16 as isize;
_7 = _13;
_3 = !_10;
_15 = 4267179769958671994_u64 as f64;
_15 = (-39372526050334440341720421156199262270_i128) as f64;
_6 = _7 & _13;
_9 = _13 ^ _6;
_3 = _12;
_7 = _11;
_15 = 2857166691_u32 as f64;
_6 = !_9;
_13 = _7;
_16 = !_2;
_12 = 7890_i16 as usize;
_3 = _1 | _1;
_5 = '\u{362ab}' as usize;
_13 = _9;
_16 = _2;
_13 = _9;
_4 = _7 | _7;
_11 = _4 << _9;
_5 = 4278004157286812839_i64 as usize;
RET = [(-31053_i16),29875_i16,19361_i16,(-15043_i16)];
_17.0 = !_10;
_3 = _17.0;
Goto(bb2)
}
bb10 = {
_17 = (_12,);
_26 = [_20.3,_20.3,_20.3];
_11 = _15 as isize;
_10 = _13 as usize;
Goto(bb7)
}
bb11 = {
(*_14) = (-3363459786291740793_i64) >> _1;
_21 = !(-305955529_i32);
_2 = _16;
_17.0 = _1;
_20.1 = _1 * _1;
_9 = _8 - _4;
_17.0 = _20.4 as usize;
_17.0 = !_20.1;
_23 = _16;
_24 = [_16];
RET = [8586_i16,7114_i16,(-18492_i16),14136_i16];
Goto(bb6)
}
bb12 = {
_7 = _20.4 as isize;
_24 = [_2];
_20.2 = 5963033653893039122_u64 | 15447041013874321719_u64;
_24 = [_2];
_7 = !_11;
_11 = _3 as isize;
_4 = _11;
_8 = _13 & _6;
_20.1 = !_12;
_7 = _8 - _4;
_20.3 = 34914495268194950363921968098062010727_i128 ^ (-130334989786910264864887114476263829864_i128);
_23 = _2;
Goto(bb4)
}
bb13 = {
_6 = -_8;
_22 = [_20.3,_20.3,_20.3];
_20.1 = _20.4 as usize;
_20.4 = 7579_u16 as i8;
_18 = [20096_u16];
_6 = _4;
_26 = [_20.3,_20.3,_20.3];
Goto(bb14)
}
bb14 = {
_17.0 = _12 + _12;
_13 = -_6;
_1 = _15 as usize;
_20.2 = 6608895282688476104_u64 ^ 5162776031282850539_u64;
_21 = 413524195_i32 << _10;
_13 = _6;
_20.0 = '\u{1f29a}';
_27 = _20.4 as f64;
_9 = _8 ^ _4;
_30 = _15 as f32;
_7 = -_6;
(*_14) = (-7150865545928214625_i64);
_30 = (*_14) as f32;
_27 = _15;
_17 = (_12,);
_9 = !_8;
_3 = !_20.1;
_34.0 = !(*_14);
_3 = _12 & _10;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(14_usize, 7_usize, Move(_7), 21_usize, Move(_21), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(14_usize, 5_usize, Move(_5), 25_usize, Move(_25), 22_usize, Move(_22), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(14_usize, 8_usize, Move(_8), 34_usize, Move(_34), 26_usize, Move(_26), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: u128,mut _2: [u16; 6],mut _3: [u16; 6],mut _4: [u16; 6],mut _5: [u16; 6],mut _6: u128,mut _7: u128,mut _8: [u16; 6],mut _9: u128,mut _10: isize,mut _11: [i16; 4],mut _12: [i16; 4],mut _13: [i16; 4],mut _14: usize,mut _15: [u16; 6]) -> i32 {
mir! {
type RET = i32;
let _16: bool;
let _17: u8;
let _18: isize;
let _19: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _20: Adt58;
let _21: (u16, i16);
let _22: i8;
let _23: Adt54;
let _24: *mut char;
let _25: [char; 3];
let _26: Adt53;
let _27: [u8; 2];
let _28: *const (i64,);
let _29: Adt55;
let _30: *mut (i64, (i64,));
let _31: u8;
let _32: f64;
let _33: ();
let _34: ();
{
_10 = (-9223372036854775808_isize) >> _6;
_11 = [8070_i16,(-18248_i16),(-16434_i16),(-23759_i16)];
_5 = [61381_u16,59785_u16,50056_u16,10289_u16,25437_u16,912_u16];
_14 = 3492072636_u32 as usize;
_18 = !_10;
_3 = [52625_u16,18832_u16,27489_u16,22679_u16,12354_u16,11139_u16];
_7 = (-1144226966_i32) as u128;
_21 = (519_u16, (-129_i16));
_5 = _2;
_3 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_19.4 = 148454130016634290063811244535752080619_i128;
_3 = _2;
match _19.4 {
148454130016634290063811244535752080619 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_8 = _2;
_2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_6 = !_9;
_19.1 = 2800505791_u32 << _9;
_24 = core::ptr::addr_of_mut!(_19.3.1);
_19.2 = _21.1 << _19.1;
_13 = [_19.2,_19.2,_19.2,_21.1];
_19.0.0 = (-8558799677176418471_i64) >> _19.2;
_19.5 = [_21.0];
_21 = (31679_u16, _19.2);
_7 = _1;
_4 = _5;
_6 = _1 | _1;
match _21.0 {
0 => bb3,
1 => bb4,
2 => bb5,
31679 => bb7,
_ => bb6
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
Return()
}
bb7 = {
_19.1 = 3293467537_u32 - 507506396_u32;
(*_24) = '\u{a2c87}';
_5 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_22 = 107_i8 * 59_i8;
_13 = _12;
_13 = [_21.1,_19.2,_21.1,_21.1];
_12 = [_21.1,_21.1,_21.1,_19.2];
RET = -(-1775504260_i32);
(*_24) = '\u{329bd}';
_25 = [_19.3.1,(*_24),_19.3.1];
_13 = [_19.2,_19.2,_19.2,_21.1];
_19.2 = _6 as i16;
_11 = [_21.1,_21.1,_19.2,_19.2];
match _21.0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
31679 => bb14,
_ => bb13
}
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
_8 = _2;
_2 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_6 = !_9;
_19.1 = 2800505791_u32 << _9;
_24 = core::ptr::addr_of_mut!(_19.3.1);
_19.2 = _21.1 << _19.1;
_13 = [_19.2,_19.2,_19.2,_21.1];
_19.0.0 = (-8558799677176418471_i64) >> _19.2;
_19.5 = [_21.0];
_21 = (31679_u16, _19.2);
_7 = _1;
_4 = _5;
_6 = _1 | _1;
match _21.0 {
0 => bb3,
1 => bb4,
2 => bb5,
31679 => bb7,
_ => bb6
}
}
bb13 = {
Return()
}
bb14 = {
_19.3.1 = '\u{90ce6}';
(*_24) = '\u{600d0}';
_16 = false;
_19.0.0 = (-3621941649386616132_i64);
_10 = _18 - _18;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(15_usize, 11_usize, Move(_11), 25_usize, Move(_25), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(15_usize, 12_usize, Move(_12), 1_usize, Move(_1), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(15_usize, 3_usize, Move(_3), 22_usize, Move(_22), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: i64,mut _3: isize,mut _4: Adt52,mut _5: isize,mut _6: isize,mut _7: isize) -> char {
mir! {
type RET = char;
let _8: i128;
let _9: (usize,);
let _10: *mut i64;
let _11: [u8; 7];
let _12: [i128; 3];
let _13: u64;
let _14: isize;
let _15: [u16; 1];
let _16: char;
let _17: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _18: (i64,);
let _19: Adt59;
let _20: isize;
let _21: Adt54;
let _22: ();
let _23: ();
{
RET = '\u{22c8a}';
RET = '\u{3bf48}';
_2 = 154786954365579967159922626660853509663_u128 as i64;
_2 = !6012315505729896355_i64;
RET = '\u{ff509}';
place!(Field::<*const i64>(Variant(_4, 3), 0)) = core::ptr::addr_of!(_2);
_1 = 328025300_u32 as isize;
_5 = -_3;
match Field::<u8>(Variant(_4, 3), 2) {
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
place!(Field::<*const i64>(Variant(_4, 3), 0)) = core::ptr::addr_of!(_2);
place!(Field::<i128>(Variant(_4, 3), 1)) = 2171655670_u32 as i128;
_2 = 8825033147002087012_i64;
_8 = Field::<i128>(Variant(_4, 3), 1);
_9.0 = _2 as usize;
place!(Field::<u8>(Variant(_4, 3), 2)) = 127_u8 * 99_u8;
_9.0 = 12237_i16 as usize;
_10 = core::ptr::addr_of_mut!(_2);
SetDiscriminant(_4, 0);
place!(Field::<char>(Variant(_4, 0), 1)) = RET;
_6 = !_7;
RET = Field::<char>(Variant(_4, 0), 1);
place!(Field::<[usize; 8]>(Variant(_4, 0), 0)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_11 = [140_u8,235_u8,23_u8,42_u8,84_u8,75_u8,6_u8];
match (*_10) {
0 => bb4,
1 => bb5,
8825033147002087012 => bb7,
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
_11 = [190_u8,26_u8,13_u8,158_u8,71_u8,236_u8,250_u8];
_12 = [_8,_8,_8];
_3 = !_5;
(*_10) = (-7428784867083382675_i64);
RET = Field::<char>(Variant(_4, 0), 1);
RET = Field::<char>(Variant(_4, 0), 1);
_5 = _1 ^ _7;
_16 = RET;
_3 = _6;
_5 = _6;
_17.1 = 2210021318_u32 + 3675458270_u32;
Call(_14 = fn17(_3, _6, _3, _5, _7, _5, _5, _3, _3, _3, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17.0.0 = (*_10) | (*_10);
_17.4 = (*_10) as i128;
_16 = RET;
_17.3.0 = 266665113432943351644389578449092023356_u128 as f32;
_17.2 = -10235_i16;
_7 = _5 | _14;
_10 = core::ptr::addr_of_mut!((*_10));
_17.3.1 = Field::<char>(Variant(_4, 0), 1);
place!(Field::<char>(Variant(_4, 0), 1)) = _16;
match (*_10) {
0 => bb1,
1 => bb9,
340282366920938463455945822564684828781 => bb11,
_ => bb10
}
}
bb9 = {
_11 = [190_u8,26_u8,13_u8,158_u8,71_u8,236_u8,250_u8];
_12 = [_8,_8,_8];
_3 = !_5;
(*_10) = (-7428784867083382675_i64);
RET = Field::<char>(Variant(_4, 0), 1);
RET = Field::<char>(Variant(_4, 0), 1);
_5 = _1 ^ _7;
_16 = RET;
_3 = _6;
_5 = _6;
_17.1 = 2210021318_u32 + 3675458270_u32;
Call(_14 = fn17(_3, _6, _3, _5, _7, _5, _5, _3, _3, _3, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_17.4 = _8;
_18.0 = _17.0.0 << _7;
_6 = -_7;
place!(Field::<char>(Variant(_4, 0), 1)) = _16;
_8 = _17.4;
_9.0 = 3_usize - 8861068178013303754_usize;
_8 = (-68_i8) as i128;
_1 = _9.0 as isize;
_14 = -_7;
_18.0 = 10507278583820665511_u64 as i64;
_12 = [_17.4,_8,_8];
_9.0 = !2_usize;
_17.5 = [13274_u16];
_7 = !_3;
_17.0.1 = _18;
match _2 {
0 => bb12,
1 => bb13,
2 => bb14,
340282366920938463455945822564684828781 => bb16,
_ => bb15
}
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
place!(Field::<*const i64>(Variant(_4, 3), 0)) = core::ptr::addr_of!(_2);
place!(Field::<i128>(Variant(_4, 3), 1)) = 2171655670_u32 as i128;
_2 = 8825033147002087012_i64;
_8 = Field::<i128>(Variant(_4, 3), 1);
_9.0 = _2 as usize;
place!(Field::<u8>(Variant(_4, 3), 2)) = 127_u8 * 99_u8;
_9.0 = 12237_i16 as usize;
_10 = core::ptr::addr_of_mut!(_2);
SetDiscriminant(_4, 0);
place!(Field::<char>(Variant(_4, 0), 1)) = RET;
_6 = !_7;
RET = Field::<char>(Variant(_4, 0), 1);
place!(Field::<[usize; 8]>(Variant(_4, 0), 0)) = [_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_11 = [140_u8,235_u8,23_u8,42_u8,84_u8,75_u8,6_u8];
match (*_10) {
0 => bb4,
1 => bb5,
8825033147002087012 => bb7,
_ => bb6
}
}
bb16 = {
_9 = (7_usize,);
_15 = [58220_u16];
_15 = [29574_u16];
_19 = Adt59::Variant0 { fld0: _12 };
_17.0 = ((*_10), _18);
SetDiscriminant(_19, 1);
_18.0 = _17.0.0;
_1 = 194160462509192883694663357401282327198_u128 as isize;
_17.0 = (_18.0, _18);
(*_10) = -_18.0;
_8 = _17.4 * _17.4;
place!(Field::<(u16, i16)>(Variant(_19, 1), 0)) = (7894_u16, _17.2);
RET = _16;
place!(Field::<char>(Variant(_4, 0), 1)) = _17.3.1;
place!(Field::<*const (i64,)>(Variant(_19, 1), 3)) = core::ptr::addr_of!(_17.0.1);
place!(Field::<(u16, i16)>(Variant(_19, 1), 0)).1 = !_17.2;
place!(Field::<f64>(Variant(_19, 1), 1)) = _17.3.0 as f64;
Goto(bb17)
}
bb17 = {
Call(_22 = dump_var(16_usize, 12_usize, Move(_12), 8_usize, Move(_8), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_22 = dump_var(16_usize, 6_usize, Move(_6), 16_usize, Move(_16), 18_usize, Move(_18), 23_usize, _23), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> isize {
mir! {
type RET = isize;
let _12: ();
let _13: ();
{
_3 = _2;
_8 = _3;
_6 = !_1;
_5 = _9 - _7;
_10 = !_9;
RET = -_6;
_7 = !_2;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(17_usize, 11_usize, Move(_11), 6_usize, Move(_6), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_12 = dump_var(17_usize, 2_usize, Move(_2), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [i16; 4],mut _2: u128,mut _3: usize,mut _4: f64,mut _5: i32,mut _6: i32,mut _7: f64,mut _8: u128,mut _9: i32,mut _10: [i16; 4],mut _11: i32) -> (i64,) {
mir! {
type RET = (i64,);
let _12: [i16; 4];
let _13: [bool; 1];
let _14: ([i16; 4], [u16; 1], (i64, (i64,)), u128);
let _15: Adt60;
let _16: u128;
let _17: char;
let _18: bool;
let _19: u64;
let _20: [u16; 1];
let _21: Adt49;
let _22: f32;
let _23: Adt53;
let _24: Adt58;
let _25: u64;
let _26: Adt52;
let _27: *const (usize, u16, *const i64, bool);
let _28: &'static [u8; 2];
let _29: (usize,);
let _30: u64;
let _31: *const (usize, u16, *const i64, bool);
let _32: f64;
let _33: i32;
let _34: bool;
let _35: ([i16; 4], [u16; 1], (i64, (i64,)), u128);
let _36: isize;
let _37: f64;
let _38: isize;
let _39: ();
let _40: ();
{
_4 = _7;
RET.0 = !563798445590436209_i64;
_12 = [23578_i16,26877_i16,28535_i16,(-25747_i16)];
_1 = [5750_i16,(-5967_i16),16552_i16,(-3614_i16)];
_5 = _6 * _11;
_14.2.1 = (RET.0,);
_14.3 = 19466_i16 as u128;
_10 = [2375_i16,(-20549_i16),(-17653_i16),(-19823_i16)];
_14.1 = [9443_u16];
_13 = [true];
_14.2.0 = _8 as i64;
_14.0 = [19380_i16,(-30625_i16),8976_i16,(-39_i16)];
_13 = [true];
_6 = _5 | _9;
_11 = _5;
_14.0 = [(-9893_i16),(-13879_i16),5052_i16,34_i16];
_14.2.0 = -RET.0;
RET = (_14.2.1.0,);
_14.2.0 = _14.2.1.0 + RET.0;
Call(_14.0 = core::intrinsics::transmute(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = -_5;
_6 = (-21474_i16) as i32;
Goto(bb2)
}
bb2 = {
_14.0 = [3378_i16,(-17909_i16),13043_i16,(-28215_i16)];
_16 = _8;
_14.2.1 = (_14.2.0,);
_7 = -_4;
_6 = 5513157287886287083_u64 as i32;
_17 = '\u{af504}';
Goto(bb3)
}
bb3 = {
_13 = [false];
Goto(bb4)
}
bb4 = {
_13 = [false];
_9 = _11;
_11 = _9;
_15.fld0 = [_14.2.0];
_13 = [true];
_6 = 16170939883173885044_u64 as i32;
_13 = [true];
_7 = _4;
_12 = [1966_i16,(-2083_i16),1915_i16,(-10875_i16)];
_17 = '\u{1c84c}';
_6 = 17343836622630857567_u64 as i32;
_18 = !false;
_16 = _2;
RET = _14.2.1;
_3 = 8900_i16 as usize;
_12 = _14.0;
_6 = _11 << _8;
_1 = [(-24629_i16),(-17908_i16),(-918_i16),(-23566_i16)];
_9 = _4 as i32;
_1 = _12;
_14.2.0 = (-51806063177106483144834669201030932962_i128) as i64;
_18 = false & true;
_2 = _16 ^ _8;
RET.0 = _17 as i64;
_14.3 = _8;
_1 = [(-27580_i16),(-11940_i16),16997_i16,(-15980_i16)];
_18 = false | false;
Goto(bb5)
}
bb5 = {
_14.1 = [24216_u16];
_14.2.1.0 = -_14.2.0;
_20 = _14.1;
_6 = _9 << _16;
RET = _14.2.1;
RET.0 = _14.2.1.0 & _14.2.0;
_7 = _3 as f64;
RET = _14.2.1;
_2 = _16 ^ _8;
_13 = [_18];
_14.2 = (RET.0, RET);
_1 = [(-3765_i16),19592_i16,(-21064_i16),(-13379_i16)];
_10 = [27391_i16,(-25418_i16),21266_i16,8153_i16];
_22 = 233_u8 as f32;
_13 = [_18];
_14.2 = (RET.0, RET);
_14.3 = !_2;
Call(_8 = core::intrinsics::bswap(_14.3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = !10049802625344004263_usize;
_19 = 11540526241385418450_u64 >> _5;
_20 = _14.1;
_9 = _6 >> _5;
_13 = [_18];
_8 = _14.3;
_14.2.1 = (RET.0,);
_8 = _14.3 << _9;
_2 = _3 as u128;
_18 = false;
_10 = [(-9563_i16),(-1362_i16),(-22677_i16),(-17080_i16)];
_19 = 9223372036854775807_isize as u64;
_12 = _1;
RET.0 = _14.2.1.0;
_9 = _11 | _11;
_9 = _17 as i32;
_12 = _10;
_14.1 = _20;
_14.2.1.0 = -RET.0;
_14.1 = [36832_u16];
_6 = _5 * _11;
RET.0 = _14.2.0;
RET = (_14.2.0,);
_29.0 = _3;
Goto(bb7)
}
bb7 = {
_18 = !false;
_25 = _19 >> RET.0;
_14.3 = _17 as u128;
_10 = _12;
_6 = _5;
_29 = (_3,);
_20 = _14.1;
_30 = 68631696_u32 as u64;
Goto(bb8)
}
bb8 = {
_14.0 = [13100_i16,7585_i16,(-23252_i16),(-9321_i16)];
_11 = _5 & _6;
_20 = _14.1;
_18 = !true;
_14.2.1.0 = RET.0 | _14.2.0;
_16 = _17 as u128;
_14.2.0 = RET.0;
_4 = _8 as f64;
_25 = _3 as u64;
_7 = _4 + _4;
_3 = _29.0 + _29.0;
_30 = _19 ^ _19;
RET.0 = !_14.2.1.0;
_30 = _19 ^ _19;
_29 = (_3,);
_14.0 = [24772_i16,(-12257_i16),(-12025_i16),14415_i16];
_25 = !_30;
_11 = _9;
_32 = -_7;
_29 = (_3,);
_3 = 121_i8 as usize;
_29.0 = _30 as usize;
_19 = _25 + _30;
_14.0 = [10890_i16,11653_i16,(-2030_i16),(-22550_i16)];
Goto(bb9)
}
bb9 = {
_16 = _8;
_22 = 44587_u16 as f32;
_4 = _7 + _7;
_30 = 22118_u16 as u64;
_30 = (-74623830704817277208477743630583484624_i128) as u64;
_14.2.0 = _14.2.1.0;
_4 = -_32;
_3 = !_29.0;
_2 = _25 as u128;
_34 = _18;
_11 = _30 as i32;
_3 = !_29.0;
_14.2.1 = (_14.2.0,);
_20 = [51634_u16];
_33 = _5 ^ _5;
_9 = -_33;
_35.2.1 = (_14.2.0,);
RET.0 = _14.2.1.0 ^ _14.2.0;
_18 = _34;
_35.0 = _1;
_14.0 = [16153_i16,(-23891_i16),(-5022_i16),11877_i16];
Goto(bb10)
}
bb10 = {
_1 = _10;
_12 = [(-16675_i16),21920_i16,(-2930_i16),9059_i16];
_35 = _14;
_35.2.1.0 = -_14.2.0;
_13 = [_34];
_15.fld0 = [_14.2.0];
_14 = _35;
_14.2.0 = RET.0 & RET.0;
_35.0 = [(-17093_i16),(-6024_i16),(-31929_i16),(-5867_i16)];
_5 = _22 as i32;
_14.2.0 = !RET.0;
_36 = !9223372036854775807_isize;
_6 = _9;
_14 = (_10, _35.1, _35.2, _8);
_2 = _8;
_36 = _14.2.1.0 as isize;
_2 = (-3857_i16) as u128;
_33 = _6;
Call(_35.2.1.0 = core::intrinsics::transmute(_19), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = !_14.3;
_29.0 = _22 as usize;
_14.3 = _16;
_22 = (-3959_i16) as f32;
_35.0 = [32445_i16,7856_i16,18736_i16,28298_i16];
Goto(bb12)
}
bb12 = {
_10 = [23183_i16,(-26463_i16),21745_i16,20493_i16];
_7 = _29.0 as f64;
_29.0 = 19_u8 as usize;
RET.0 = 162_u8 as i64;
_35.0 = [6494_i16,(-15325_i16),29559_i16,31810_i16];
_14.2 = (_35.2.0, _35.2.1);
RET.0 = !_35.2.1.0;
_15.fld0 = [RET.0];
RET.0 = _8 as i64;
_33 = !_9;
_14.2.0 = RET.0 ^ RET.0;
_37 = _32 + _4;
_2 = 3198311014_u32 as u128;
Goto(bb13)
}
bb13 = {
Call(_39 = dump_var(18_usize, 3_usize, Move(_3), 16_usize, Move(_16), 13_usize, Move(_13), 34_usize, Move(_34)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_39 = dump_var(18_usize, 33_usize, Move(_33), 5_usize, Move(_5), 29_usize, Move(_29), 12_usize, Move(_12)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_39 = dump_var(18_usize, 10_usize, Move(_10), 6_usize, Move(_6), 36_usize, Move(_36), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: (i64,),mut _2: u64,mut _3: i64,mut _4: (i64, (i64,)),mut _5: f64,mut _6: char) -> u128 {
mir! {
type RET = u128;
let _7: [u16; 6];
let _8: char;
let _9: Adt50;
let _10: ((i64, (i64,)), u16);
let _11: &'static [u8; 2];
let _12: Adt64;
let _13: &'static [u8; 2];
let _14: f64;
let _15: isize;
let _16: Adt54;
let _17: Adt50;
let _18: f32;
let _19: usize;
let _20: [usize; 8];
let _21: f32;
let _22: f32;
let _23: bool;
let _24: f64;
let _25: isize;
let _26: Adt60;
let _27: i16;
let _28: [bool; 1];
let _29: ((i64, (i64,)), u16);
let _30: u64;
let _31: isize;
let _32: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]);
let _33: i64;
let _34: *mut *mut i64;
let _35: char;
let _36: ();
let _37: ();
{
_5 = (-740897760_i32) as f64;
_4 = (_3, _1);
_4.0 = -_1.0;
_4.0 = _4.1.0 ^ _1.0;
_1 = (_4.1.0,);
_4.0 = _3;
_3 = 16600869545482992964_usize as i64;
_2 = (-147333994770084279942962765534938941175_i128) as u64;
RET = 299217503058038597299287600528858422666_u128 ^ 153582528774341638285338130135324576184_u128;
_4.1.0 = _1.0 - _1.0;
_4.1.0 = _1.0 - _1.0;
_4.1 = (_4.0,);
_8 = _6;
_6 = _8;
_7 = [13046_u16,55090_u16,36555_u16,18255_u16,45462_u16,28727_u16];
RET = !176300947214435627308957437382649418951_u128;
_10.0.1 = _1;
_8 = _6;
Goto(bb1)
}
bb1 = {
_10.0 = (_1.0, _1);
_2 = 230_u8 as u64;
_10.0.0 = !_4.0;
_10.0.0 = _2 as i64;
Goto(bb2)
}
bb2 = {
_12.fld4 = 42558_u16;
_6 = _8;
_12.fld2.0 = core::ptr::addr_of_mut!(_12.fld1.0.0);
_10 = (_4, _12.fld4);
_5 = 4140615563_u32 as f64;
_12.fld2.0 = core::ptr::addr_of_mut!(_1.0);
_12.fld0 = (_10.0.1.0, _10.0.1);
_12.fld5.0 = _12.fld4 % _12.fld4;
_10.0 = (_12.fld0.0, _12.fld0.1);
_12.fld0.0 = 105777752593324671717929966984253609966_i128 as i64;
_12.fld4 = _12.fld5.0;
_12.fld1.0.0 = !_10.0.0;
_14 = _5 * _5;
_10 = (_12.fld0, _12.fld4);
_18 = _14 as f32;
_12.fld1.3 = (_18, _6);
Call(_12.fld0.0 = core::intrinsics::bswap(_1.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.fld1.0 = (_4.1.0, _12.fld0.1);
_12.fld1.2 = (-11804_i16) * (-14233_i16);
_1.0 = 112_i8 as i64;
_12.fld5.1 = _12.fld1.2 << _12.fld0.1.0;
_12.fld1.4 = (-64364771009016964520020787472093844536_i128) & 20916882550721738848441774763042708930_i128;
_6 = _8;
_22 = _12.fld1.3.0 * _18;
Goto(bb4)
}
bb4 = {
_18 = 3_usize as f32;
_12.fld1.0 = _4;
RET = 182731535784778614177063285522076230475_u128;
_12.fld0.1.0 = _12.fld1.0.1.0;
_5 = _14;
_12.fld0.0 = 36_u8 as i64;
RET = 40731684245551877452633221776416835274_u128;
_12.fld0.0 = -_10.0.1.0;
_12.fld1.1 = 635358076_u32;
_26.fld0 = [_12.fld1.0.1.0];
Goto(bb5)
}
bb5 = {
_21 = _18 + _22;
_16 = Adt54::Variant0 { fld0: _10.0 };
SetDiscriminant(_16, 3);
Call(_4.0 = core::intrinsics::bswap(_12.fld0.1.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<[i16; 4]>(Variant(_16, 3), 2)) = [_12.fld5.1,_12.fld5.1,_12.fld5.1,_12.fld5.1];
RET = 170450293213590734148057623387209108716_u128;
_24 = _2 as f64;
_29.0.1 = _12.fld1.0.1;
match RET {
0 => bb7,
1 => bb8,
170450293213590734148057623387209108716 => bb10,
_ => bb9
}
}
bb7 = {
_21 = _18 + _22;
_16 = Adt54::Variant0 { fld0: _10.0 };
SetDiscriminant(_16, 3);
Call(_4.0 = core::intrinsics::bswap(_12.fld0.1.0), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_12.fld4 = 42558_u16;
_6 = _8;
_12.fld2.0 = core::ptr::addr_of_mut!(_12.fld1.0.0);
_10 = (_4, _12.fld4);
_5 = 4140615563_u32 as f64;
_12.fld2.0 = core::ptr::addr_of_mut!(_1.0);
_12.fld0 = (_10.0.1.0, _10.0.1);
_12.fld5.0 = _12.fld4 % _12.fld4;
_10.0 = (_12.fld0.0, _12.fld0.1);
_12.fld0.0 = 105777752593324671717929966984253609966_i128 as i64;
_12.fld4 = _12.fld5.0;
_12.fld1.0.0 = !_10.0.0;
_14 = _5 * _5;
_10 = (_12.fld0, _12.fld4);
_18 = _14 as f32;
_12.fld1.3 = (_18, _6);
Call(_12.fld0.0 = core::intrinsics::bswap(_1.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_10.0 = (_1.0, _1);
_2 = 230_u8 as u64;
_10.0.0 = !_4.0;
_10.0.0 = _2 as i64;
Goto(bb2)
}
bb10 = {
_12.fld0.1 = (_10.0.1.0,);
_10.0.1.0 = -_29.0.1.0;
_10.0.0 = _4.0;
_1.0 = _12.fld0.0 ^ _12.fld0.1.0;
match RET {
0 => bb11,
170450293213590734148057623387209108716 => bb13,
_ => bb12
}
}
bb11 = {
_21 = _18 + _22;
_16 = Adt54::Variant0 { fld0: _10.0 };
SetDiscriminant(_16, 3);
Call(_4.0 = core::intrinsics::bswap(_12.fld0.1.0), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_10.0 = (_1.0, _1);
_2 = 230_u8 as u64;
_10.0.0 = !_4.0;
_10.0.0 = _2 as i64;
Goto(bb2)
}
bb13 = {
_12.fld1.0 = _10.0;
place!(Field::<[i16; 4]>(Variant(_16, 3), 2)) = [_12.fld5.1,_12.fld5.1,_12.fld5.1,_12.fld5.1];
_12.fld1.0.1.0 = _10.0.0 + _12.fld1.0.0;
_12.fld1.2 = _12.fld5.1;
_21 = _22;
RET = 225136937696925776797321540032411064189_u128 * 51773703558137874269606253128378759487_u128;
_28 = [false];
place!(Field::<*mut char>(Variant(_16, 3), 1)) = core::ptr::addr_of_mut!(_6);
place!(Field::<i8>(Variant(_16, 3), 0)) = 43_u8 as i8;
_23 = false ^ true;
_19 = !2_usize;
_29.1 = _10.1 << _12.fld1.1;
_12.fld5 = (_12.fld4, _12.fld1.2);
_29.0 = (_12.fld1.0.1.0, _12.fld1.0.1);
_12.fld1.0.1 = _12.fld0.1;
_32.0.0 = -_12.fld1.0.0;
_12.fld1.4 = (-167668344561066665811852315951882074131_i128) - 58647810491826010506865629469964753887_i128;
SetDiscriminant(_16, 1);
_25 = (-9223372036854775808_isize) - 42_isize;
_4.1 = (_10.0.0,);
_32.2 = 128_u8 as i16;
RET = 303825918015099562935137566713513104616_u128 - 331532904685559882664134455245152809312_u128;
place!(Field::<u32>(Variant(_16, 1), 1)) = 451497034_i32 as u32;
_32.0 = (_12.fld1.0.0, _10.0.1);
Goto(bb14)
}
bb14 = {
_35 = _8;
_12.fld1.0.0 = (-3_i8) as i64;
_12.fld1.3.0 = _22;
_12.fld1.0.1 = _29.0.1;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(19_usize, 23_usize, Move(_23), 35_usize, Move(_35), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(19_usize, 10_usize, Move(_10), 7_usize, Move(_7), 2_usize, Move(_2), 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(237152723061358213262138472999219839058_u128), std::hint::black_box(1768_u16));
                
            }
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: bool,
fld1: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]),
fld2: *mut *mut i64,
fld3: (u16, i16),

},
Variant1{
fld0: (u16, i16),

},
Variant2{
fld0: [u8; 2],
fld1: *mut i64,

},
Variant3{
fld0: [usize; 8],
fld1: char,
fld2: usize,
fld3: u16,
fld4: (u64, (i64,), [i16; 4]),
fld5: i32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: bool,
fld1: u16,
fld2: (*mut i64,),
fld3: (u64, (i64,), [i16; 4]),
fld4: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]),
fld5: i32,

},
Variant1{
fld0: [u8; 2],
fld1: (f32, char),
fld2: isize,
fld3: [usize; 8],
fld4: [u16; 6],
fld5: u8,

},
Variant2{
fld0: (usize, u16, *const i64, bool),
fld1: (f32, char),
fld2: i32,
fld3: ([i16; 4], [u16; 1], (i64, (i64,)), u128),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *mut (i64, (i64,)),

},
Variant1{
fld0: bool,

},
Variant2{
fld0: [u8; 2],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [usize; 8],
fld1: char,
fld2: *const i64,

},
Variant1{
fld0: (f32, char),

},
Variant2{
fld0: [i16; 4],
fld1: u64,
fld2: isize,
fld3: [u8; 7],
fld4: [i64; 1],
fld5: (char, usize, u64, i128, i8),
fld6: [u16; 6],

},
Variant3{
fld0: *const i64,
fld1: i128,
fld2: u8,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: (u64, (i64,), [i16; 4]),
fld1: *const (i64,),
fld2: [i64; 6],
fld3: [u16; 6],
fld4: ((i64, (i64,)), u16),
fld5: i32,
fld6: [u8; 2],

},
Variant1{
fld0: *mut *mut i64,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: (i64, (i64,)),

},
Variant1{
fld0: i128,
fld1: u32,

},
Variant2{
fld0: [i16; 4],
fld1: [u16; 6],
fld2: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]),

},
Variant3{
fld0: i8,
fld1: *mut char,
fld2: [i16; 4],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [u8; 2],
fld1: [char; 3],
fld2: (char, usize, u64, i128, i8),

},
Variant1{
fld0: (i64,),

},
Variant2{
fld0: i128,
fld1: [i64; 1],
fld2: isize,
fld3: i8,
fld4: *mut char,
fld5: u128,
fld6: Adt53,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (f32, char),
fld1: u8,
fld2: [u8; 2],
fld3: f32,
fld4: u32,
fld5: i64,

},
Variant1{
fld0: [i128; 3],

},
Variant2{
fld0: u128,
fld1: [i64; 1],
fld2: (u16, i16),
fld3: ((i64, (i64,)), u16),
fld4: i16,
fld5: i32,
fld6: (usize,),
fld7: [u8; 7],

},
Variant3{
fld0: (usize, u16, *const i64, bool),
fld1: f32,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (u64, (i64,), [i16; 4]),
fld1: f64,
fld2: Adt56,
fld3: f32,

},
Variant1{
fld0: isize,

},
Variant2{
fld0: *mut *mut i64,
fld1: [char; 3],
fld2: i128,
fld3: u64,
fld4: Adt53,
fld5: Adt51,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [u8; 7],
fld1: (f32, char),
fld2: (*mut i64,),
fld3: *const (i64,),
fld4: u32,
fld5: *mut *mut i64,

},
Variant1{
fld0: *const i64,
fld1: Adt56,
fld2: *mut i64,
fld3: u16,
fld4: [i128; 3],
fld5: Adt49,
fld6: [i64; 1],

},
Variant2{
fld0: *const (usize, u16, *const i64, bool),
fld1: [bool; 1],
fld2: Adt56,
fld3: [u8; 2],
fld4: i16,
fld5: (char, usize, u64, i128, i8),
fld6: (u16, i16),
fld7: i128,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: [i128; 3],

},
Variant1{
fld0: (u16, i16),
fld1: f64,
fld2: (*mut i64,),
fld3: *const (i64,),

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: [i64; 1],
}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt61::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: [u16; 6],
fld1: usize,
fld2: Adt58,
fld3: [u8; 7],

},
Variant1{
fld0: *mut *mut i64,
fld1: char,
fld2: i32,
fld3: u8,
fld4: Adt52,

},
Variant2{
fld0: *const (i64,),
fld1: char,
fld2: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]),
fld3: Adt59,
fld4: Adt54,
fld5: [bool; 1],

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt62::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: u16,
fld1: [usize; 8],
fld2: [i128; 3],

},
Variant1{
fld0: [i128; 3],
fld1: [i64; 6],
fld2: (usize, u16, *const i64, bool),
fld3: Adt60,
fld4: i16,
fld5: ([i16; 4], [u16; 1], (i64, (i64,)), u128),

}}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt63::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: Adt50,
fld1: char,
fld2: [i128; 3],
fld3: i8,
fld4: (*mut i64,),

},
Variant1{
fld0: (usize,),

},
Variant2{
fld0: ([i16; 4], [u16; 1], (i64, (i64,)), u128),

}}
impl PrintFDebug for Adt64{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt64{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt64 {
fld0: (i64, (i64,)),
fld1: ((i64, (i64,)), u32, i16, (f32, char), i128, [u16; 1]),
fld2: (*mut i64,),
fld3: [char; 3],
fld4: u16,
fld5: (u16, i16),
fld6: usize,
fld7: *mut i64,
}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt65::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: bool,
fld1: *mut i64,
fld2: *const (usize, u16, *const i64, bool),
fld3: [i128; 3],
fld4: [usize; 8],
fld5: ([i16; 4], [u16; 1], (i64, (i64,)), u128),
fld6: f64,

},
Variant1{
fld0: Adt61,
fld1: f32,
fld2: *const (usize, u16, *const i64, bool),
fld3: [i16; 4],
fld4: u64,
fld5: Adt64,
fld6: *const (i64,),

},
Variant2{
fld0: Adt64,

}}

