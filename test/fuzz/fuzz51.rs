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
pub fn fn0(mut _1: usize,mut _2: char,mut _3: u16,mut _4: u32) -> isize {
mir! {
type RET = isize;
let _5: &'static i16;
let _6: &'static Adt36;
let _7: i32;
let _8: char;
let _9: [isize; 1];
let _10: u128;
let _11: Adt81;
let _12: [u128; 4];
let _13: [i8; 6];
let _14: f32;
let _15: bool;
let _16: (u64, [u32; 1], f32);
let _17: *const [u32; 7];
let _18: u16;
let _19: isize;
let _20: Adt72;
let _21: char;
let _22: isize;
let _23: [usize; 1];
let _24: i8;
let _25: f64;
let _26: f64;
let _27: &'static u128;
let _28: isize;
let _29: f32;
let _30: (u64, [u32; 1], f32);
let _31: Adt81;
let _32: bool;
let _33: i16;
let _34: f64;
let _35: [isize; 6];
let _36: (u8, [u64; 3], u8, [char; 6]);
let _37: (Adt36, u32, i8);
let _38: &'static [u64; 3];
let _39: f64;
let _40: (&'static *const bool, isize, (u32, u32, i128), (u32, u32, i128));
let _41: ();
let _42: ();
{
RET = 1752026127_i32 as isize;
RET = -9223372036854775807_isize;
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_2 = '\u{b9be3}';
RET = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_4 = _2 as u32;
RET = (-9223372036854775808_isize) >> _4;
_8 = _2;
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
_1 = 17836_i16 as usize;
_8 = _2;
_9 = [RET];
_7 = 678081717_i32;
RET = 54077041227784896288361127049915585964_i128 as isize;
_8 = _2;
_4 = 1542496747_u32;
_3 = 30108333121014791456975152315072049175_u128 as u16;
_2 = _8;
Call(RET = core::intrinsics::transmute(_9), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _8 as isize;
_9 = [RET];
_4 = 127_i8 as u32;
_1 = _3 as usize;
_10 = 261092940225215152551754093887494229667_u128 ^ 280514343730726007891301530849680230261_u128;
_7 = 276310813_i32 - (-1634223882_i32);
_8 = _2;
_9 = [RET];
RET = !(-9223372036854775808_isize);
_8 = _2;
RET = 9223372036854775807_isize;
_8 = _2;
_10 = 255038484114806293450607916085139359693_u128;
RET = -(-9223372036854775808_isize);
_7 = RET as i32;
_10 = 268973830526814335110375929395506106946_u128;
RET = 9223372036854775807_isize;
_12 = [_10,_10,_10,_10];
Call(_10 = fn1(_2, RET, RET, _1, _7, RET, _9, RET, RET, _2, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10 = (-59265358009316739702924813325672303633_i128) as u128;
_16.1 = [_4];
_4 = !2405376896_u32;
_14 = _7 as f32;
RET = -(-28_isize);
_14 = RET as f32;
_14 = (-138319108629802847954747346506212814792_i128) as f32;
RET = !119_isize;
_16.2 = _14;
_10 = 67749314323484558284960572796011150300_u128 | 61549260243977296287450205160272041065_u128;
_8 = _2;
_10 = 277165703548624578197624583368706101045_u128 * 215508623421291343873008387490039121028_u128;
_13 = [(-111_i8),(-89_i8),2_i8,40_i8,(-58_i8),61_i8];
_16.0 = 3165968111289638564_u64;
RET = -9223372036854775807_isize;
_15 = !true;
Goto(bb6)
}
bb6 = {
_9 = [RET];
_18 = 14854_i16 as u16;
_16.2 = _14 + _14;
_10 = 307153645980920504466638559232327634385_u128;
_19 = RET + RET;
_16.0 = 8978936981243011772_u64 | 9307176286325688840_u64;
_12 = [_10,_10,_10,_10];
_3 = _18 & _18;
RET = _19 << _3;
Call(_17 = fn10(RET, _2, _18, RET), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = -_16.2;
_14 = _19 as f32;
_10 = 272049812729965548604915777485114456094_u128 * 317869338774428790787127640747141573501_u128;
_9 = [RET];
_10 = 143880774217090582124509780082109593514_u128 | 286033668274474352064855946999131898788_u128;
RET = _18 as isize;
_21 = _8;
_22 = _2 as isize;
RET = -_19;
_8 = _21;
_16.0 = _10 as u64;
_7 = _10 as i32;
_12 = [_10,_10,_10,_10];
_19 = (-33_i8) as isize;
_19 = _22;
_19 = _8 as isize;
_13 = [127_i8,(-44_i8),99_i8,31_i8,24_i8,(-120_i8)];
_12 = [_10,_10,_10,_10];
_3 = _18;
_24 = _16.0 as i8;
RET = -_22;
_16.0 = 11253739971699069066_u64 | 18430653063956054607_u64;
_24 = !(-24_i8);
_1 = 2156159750895065284_usize;
_13 = [_24,_24,_24,_24,_24,_24];
match _1 {
0 => bb5,
1 => bb8,
2 => bb9,
2156159750895065284 => bb11,
_ => bb10
}
}
bb8 = {
_9 = [RET];
_18 = 14854_i16 as u16;
_16.2 = _14 + _14;
_10 = 307153645980920504466638559232327634385_u128;
_19 = RET + RET;
_16.0 = 8978936981243011772_u64 | 9307176286325688840_u64;
_12 = [_10,_10,_10,_10];
_3 = _18 & _18;
RET = _19 << _3;
Call(_17 = fn10(RET, _2, _18, RET), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_1 = 17836_i16 as usize;
_8 = _2;
_9 = [RET];
_7 = 678081717_i32;
RET = 54077041227784896288361127049915585964_i128 as isize;
_8 = _2;
_4 = 1542496747_u32;
_3 = 30108333121014791456975152315072049175_u128 as u16;
_2 = _8;
Call(RET = core::intrinsics::transmute(_9), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_25 = (-130034189597791536430092182728039610151_i128) as f64;
_21 = _2;
RET = _7 as isize;
_24 = _16.2 as i8;
_27 = &_10;
_24 = 12_i8 * (-33_i8);
RET = _7 as isize;
_15 = _16.2 <= _14;
RET = 60_u8 as isize;
_23 = [_1];
_9 = [_22];
_4 = 2889198140_u32;
_23 = [_1];
_15 = true;
_25 = 119910434191953679190918125386931723834_i128 as f64;
_26 = _25 - _25;
_14 = _16.2 * _16.2;
_19 = _22 << RET;
_29 = _1 as f32;
_15 = true & false;
_30.1 = [_4];
Goto(bb12)
}
bb12 = {
_30.2 = _24 as f32;
_24 = 96_i8 + (-102_i8);
_30.2 = _14;
_28 = _19;
_30.0 = _16.0;
_26 = _25;
_2 = _8;
_30 = (_16.0, _16.1, _14);
RET = !_28;
RET = _4 as isize;
RET = _30.0 as isize;
_18 = _3;
_30.0 = !_16.0;
_12 = [(*_27),(*_27),(*_27),_10];
_18 = _3;
_29 = _14;
_14 = _30.2;
_30 = _16;
_16.0 = _18 as u64;
_19 = RET;
_27 = &_10;
_30.1 = _16.1;
_23 = [_1];
_7 = 1839815365_i32 - (-154643256_i32);
match _1 {
2156159750895065284 => bb13,
_ => bb6
}
}
bb13 = {
_30.1 = [_4];
_29 = _24 as f32;
_16 = (_30.0, _30.1, _14);
_14 = _30.2 + _29;
_4 = !2589712443_u32;
_22 = !RET;
Goto(bb14)
}
bb14 = {
_21 = _8;
_2 = _8;
_4 = 240_u8 as u32;
_16.0 = _30.0;
_8 = _2;
_36.0 = 127_u8;
_16 = (_30.0, _30.1, _30.2);
_16.1 = [_4];
_21 = _2;
_27 = &_10;
_3 = _18 + _18;
_35 = [_22,_22,_19,_19,_22,_22];
_37.1 = _1 as u32;
_36.3 = [_2,_8,_2,_8,_8,_21];
_38 = &_36.1;
_16 = _30;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(0_usize, 2_usize, Move(_2), 18_usize, Move(_18), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(0_usize, 28_usize, Move(_28), 3_usize, Move(_3), 1_usize, Move(_1), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(0_usize, 23_usize, Move(_23), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: isize,mut _3: isize,mut _4: usize,mut _5: i32,mut _6: isize,mut _7: [isize; 1],mut _8: isize,mut _9: isize,mut _10: char,mut _11: char) -> u128 {
mir! {
type RET = u128;
let _12: &'static &'static i16;
let _13: &'static u128;
let _14: &'static u128;
let _15: Adt65;
let _16: i128;
let _17: *const i32;
let _18: &'static f64;
let _19: [isize; 6];
let _20: *mut i32;
let _21: (*mut i32, [u32; 7]);
let _22: isize;
let _23: isize;
let _24: [u64; 3];
let _25: u32;
let _26: (Adt45,);
let _27: bool;
let _28: char;
let _29: f64;
let _30: (usize, *const bool, Adt36, &'static f64);
let _31: u16;
let _32: *const bool;
let _33: [i16; 8];
let _34: ([u64; 3], &'static u128, f64, [u32; 1]);
let _35: char;
let _36: u128;
let _37: Adt45;
let _38: f32;
let _39: (f64, &'static i16, i128, *mut ([u64; 6], char, [u32; 1], f64));
let _40: u16;
let _41: [i16; 8];
let _42: f32;
let _43: isize;
let _44: i16;
let _45: f32;
let _46: u8;
let _47: u32;
let _48: ((u8, [u64; 3], u8, [char; 6]), i128, (char, isize, u16, &'static i128));
let _49: i32;
let _50: bool;
let _51: f32;
let _52: i16;
let _53: isize;
let _54: [bool; 2];
let _55: Adt81;
let _56: f64;
let _57: [usize; 1];
let _58: f32;
let _59: ([u64; 6], char, [u32; 1], f64);
let _60: (u64, [u32; 1], f32);
let _61: &'static Adt36;
let _62: *mut (u8, [u64; 3], u8, [char; 6]);
let _63: isize;
let _64: char;
let _65: ();
let _66: ();
{
_11 = _10;
_7 = [_2];
_11 = _1;
_10 = _11;
RET = !88840693829581291762790374831074997000_u128;
_4 = !17474893267372159664_usize;
_3 = -_8;
_1 = _11;
_9 = 53_u8 as isize;
RET = 335468768316973484817328069050091914808_u128 | 333698964298347037494921145782261511856_u128;
_2 = -_8;
_6 = _8 * _2;
RET = 310611676005727575218409675357246434284_u128 << _2;
_3 = 56838_u16 as isize;
RET = 3322129373543158943723072910813033997_u128;
Goto(bb1)
}
bb1 = {
_3 = _6;
_4 = 4460796313814145051_usize;
_7 = [_3];
_5 = !(-375741275_i32);
_9 = _4 as isize;
_10 = _1;
RET = (-29579_i16) as u128;
_16 = 135163122445655148580275793451729441109_i128 ^ 148360872757934520898740519825441042658_i128;
_13 = &RET;
_11 = _10;
_11 = _10;
_13 = &RET;
_3 = 46586_u16 as isize;
_9 = !_6;
_5 = _4 as i32;
_17 = core::ptr::addr_of!(_5);
match _4 {
4460796313814145051 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_9 = _6;
_16 = _3 as i128;
(*_17) = (-913424598_i32);
_6 = -_9;
Goto(bb4)
}
bb4 = {
_2 = _8;
_14 = &(*_13);
(*_17) = !(-1150522528_i32);
_14 = &RET;
_21.0 = core::ptr::addr_of_mut!((*_17));
_22 = _9;
_5 = 221_i16 as i32;
_17 = core::ptr::addr_of!((*_17));
_24 = [12215975735396006562_u64,5574343344048093425_u64,10017567036920303582_u64];
_16 = 96158580427339589105899148100491393928_i128;
_10 = _1;
_20 = Move(_21.0);
(*_17) = 64608_u16 as i32;
_6 = _3 + _9;
_9 = _6 >> (*_14);
_11 = _1;
_21.1 = [17662224_u32,575454900_u32,3996356798_u32,4114004539_u32,1531003745_u32,3624097424_u32,2491446992_u32];
_7 = [_2];
_23 = false as isize;
_24 = [1440867867823679319_u64,5791482358426545447_u64,17832016880301161068_u64];
_19 = [_23,_9,_23,_2,_9,_9];
_11 = _1;
_22 = 32436_u16 as isize;
match _8 {
9223372036854775807 => bb5,
_ => bb1
}
}
bb5 = {
RET = 284898595948903219494976950314972815913_u128;
_14 = &RET;
_18 = &_29;
_27 = !true;
_15 = Adt65::Variant1 { fld0: 42653573_u32,fld1: (*_17),fld2: _7,fld3: (-10_i8) };
_5 = Field::<i32>(Variant(_15, 1), 1);
_7 = [_3];
_13 = Move(_14);
_16 = _5 as i128;
_21.1 = [1737795807_u32,2779778637_u32,2170412970_u32,856344276_u32,962587301_u32,1206151622_u32,1957450565_u32];
_24 = [15412177696848167557_u64,1887824013582630376_u64,14527043030265881294_u64];
_2 = (-18096_i16) as isize;
_11 = _10;
match _8 {
0 => bb6,
1 => bb7,
9223372036854775807 => bb9,
_ => bb8
}
}
bb6 = {
_2 = _8;
_14 = &(*_13);
(*_17) = !(-1150522528_i32);
_14 = &RET;
_21.0 = core::ptr::addr_of_mut!((*_17));
_22 = _9;
_5 = 221_i16 as i32;
_17 = core::ptr::addr_of!((*_17));
_24 = [12215975735396006562_u64,5574343344048093425_u64,10017567036920303582_u64];
_16 = 96158580427339589105899148100491393928_i128;
_10 = _1;
_20 = Move(_21.0);
(*_17) = 64608_u16 as i32;
_6 = _3 + _9;
_9 = _6 >> (*_14);
_11 = _1;
_21.1 = [17662224_u32,575454900_u32,3996356798_u32,4114004539_u32,1531003745_u32,3624097424_u32,2491446992_u32];
_7 = [_2];
_23 = false as isize;
_24 = [1440867867823679319_u64,5791482358426545447_u64,17832016880301161068_u64];
_19 = [_23,_9,_23,_2,_9,_9];
_11 = _1;
_22 = 32436_u16 as isize;
match _8 {
9223372036854775807 => bb5,
_ => bb1
}
}
bb7 = {
_9 = _6;
_16 = _3 as i128;
(*_17) = (-913424598_i32);
_6 = -_9;
Goto(bb4)
}
bb8 = {
_3 = _6;
_4 = 4460796313814145051_usize;
_7 = [_3];
_5 = !(-375741275_i32);
_9 = _4 as isize;
_10 = _1;
RET = (-29579_i16) as u128;
_16 = 135163122445655148580275793451729441109_i128 ^ 148360872757934520898740519825441042658_i128;
_13 = &RET;
_11 = _10;
_11 = _10;
_13 = &RET;
_3 = 46586_u16 as isize;
_9 = !_6;
_5 = _4 as i32;
_17 = core::ptr::addr_of!(_5);
match _4 {
4460796313814145051 => bb3,
_ => bb2
}
}
bb9 = {
_29 = (-3367_i16) as f64;
_3 = !_9;
_21.1 = [2908373159_u32,1000846028_u32,3163831713_u32,386329156_u32,3608927036_u32,559016340_u32,207439023_u32];
_32 = core::ptr::addr_of!(_27);
_34.3 = [1125988995_u32];
_1 = _10;
_17 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_15, 1), 1)));
_25 = !3596548296_u32;
RET = 241269513610504502767796053064118888837_u128 * 263458808903493599419225753712510912289_u128;
_24 = [12167285249045932813_u64,6768755325540239483_u64,14466618965035462906_u64];
_19 = [_3,_9,_3,_23,_9,_3];
_13 = &RET;
_34.2 = (*_13) as f64;
_23 = _11 as isize;
_34.2 = _29 - _29;
_30.3 = &_34.2;
_11 = _10;
_34.0 = [1290278896662670481_u64,4517093014746412331_u64,9264343948513740631_u64];
place!(Field::<[isize; 1]>(Variant(_15, 1), 2)) = [_9];
_18 = Move(_30.3);
_19 = [_2,_6,_8,_9,_3,_6];
_15 = Adt65::Variant1 { fld0: _25,fld1: _5,fld2: _7,fld3: 110_i8 };
(*_32) = false;
_30.0 = _5 as usize;
place!(Field::<u32>(Variant(_15, 1), 0)) = _25 & _25;
Call(_30.1 = fn2(Move(_13), _6, _23, _6, _8, RET, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_7 = [_6];
_31 = 7251512732880720576_u64 as u16;
(*_32) = !false;
_20 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_15, 1), 1)));
_32 = Move(_30.1);
_13 = &_36;
_36 = RET | RET;
_34.1 = &_36;
_38 = _36 as f32;
_13 = Move(_34.1);
_43 = _6 * _6;
_21.0 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_15, 1), 1)));
_12 = &_39.1;
_39.2 = _16 >> _3;
_21.0 = Move(_20);
_10 = _11;
_13 = &_36;
_42 = _38 * _38;
Goto(bb11)
}
bb11 = {
_45 = _25 as f32;
_16 = _39.2;
_48.1 = _16 & _16;
_25 = !Field::<u32>(Variant(_15, 1), 0);
_21.0 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_15, 1), 1)));
Goto(bb12)
}
bb12 = {
_48.0.2 = 149_u8;
_45 = -_42;
_40 = _31;
_21.1 = [_25,Field::<u32>(Variant(_15, 1), 0),Field::<u32>(Variant(_15, 1), 0),_25,Field::<u32>(Variant(_15, 1), 0),_25,Field::<u32>(Variant(_15, 1), 0)];
_14 = Move(_13);
_33 = [23435_i16,(-16889_i16),9806_i16,26079_i16,29602_i16,17772_i16,16311_i16,12332_i16];
_13 = &_36;
_39.0 = _34.2 - _34.2;
_48.2.1 = _9 | _43;
_48.2.3 = &_48.1;
_39.1 = &_44;
_18 = &_39.0;
place!(Field::<[isize; 1]>(Variant(_15, 1), 2)) = _7;
_52 = !22945_i16;
_30.3 = &_34.2;
_23 = _40 as isize;
_48.0.1 = _24;
_32 = core::ptr::addr_of!(_27);
place!(Field::<i8>(Variant(_15, 1), 3)) = 26_i8;
_46 = _48.0.2;
_14 = Move(_13);
Call(_10 = fn9(Move(_18), Move(_48.2.3)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = _43;
SetDiscriminant(_15, 0);
(*_32) = false;
_41 = [_52,_52,_52,_52,_52,_52,_52,_52];
_34.2 = _39.0 * _39.0;
_43 = _48.0.2 as isize;
_56 = _29 + _29;
_2 = -_23;
_3 = _16 as isize;
_43 = 44_i8 as isize;
_30.3 = &_56;
_47 = _25 >> _48.1;
_52 = 30104_i16 * 29776_i16;
_35 = _10;
_4 = _30.0;
place!(Field::<u32>(Variant(_15, 0), 6)) = _47 >> _47;
_50 = Field::<u32>(Variant(_15, 0), 6) == Field::<u32>(Variant(_15, 0), 6);
_36 = !RET;
place!(Field::<u8>(Variant(_15, 0), 5)) = _5 as u8;
_4 = !_30.0;
_33 = [_52,_52,_52,_52,_52,_52,_52,_52];
_3 = _6 >> Field::<u32>(Variant(_15, 0), 6);
Goto(bb14)
}
bb14 = {
_48.2.3 = &place!(Field::<i128>(Variant(_15, 0), 7));
_48.0.3 = [_1,_1,_1,_35,_35,_35];
_12 = &_39.1;
_36 = _42 as u128;
_57 = [_4];
_30.1 = core::ptr::addr_of!(_50);
place!(Field::<i128>(Variant(_15, 0), 7)) = _39.2;
_48.1 = _16 << Field::<i128>(Variant(_15, 0), 7);
_60.1 = [_47];
_48.2.2 = _40;
_57 = [_30.0];
_34.1 = &RET;
_27 = _50;
_44 = !_52;
_39.1 = &_52;
_34.2 = _39.0;
_48.1 = -_39.2;
_6 = _35 as isize;
_27 = _50;
_59.2 = [Field::<u32>(Variant(_15, 0), 6)];
_63 = _3;
Goto(bb15)
}
bb15 = {
Call(_65 = dump_var(1_usize, 47_usize, Move(_47), 63_usize, Move(_63), 6_usize, Move(_6), 50_usize, Move(_50)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_65 = dump_var(1_usize, 1_usize, Move(_1), 22_usize, Move(_22), 43_usize, Move(_43), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_65 = dump_var(1_usize, 25_usize, Move(_25), 4_usize, Move(_4), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_65 = dump_var(1_usize, 57_usize, Move(_57), 27_usize, Move(_27), 16_usize, Move(_16), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: &'static u128,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u128,mut _7: char) -> *const bool {
mir! {
type RET = *const bool;
let _8: i8;
let _9: (u16, Adt45, *mut i32, (char, isize, u16, &'static i128));
let _10: [i8; 6];
let _11: isize;
let _12: f32;
let _13: Adt72;
let _14: ();
let _15: ();
{
_7 = '\u{eacfd}';
_3 = _2;
_1 = &_6;
_1 = &(*_1);
_4 = !_3;
_2 = _3;
_7 = '\u{4164b}';
_2 = _4 | _4;
_6 = !21490742487555246735214827401916908887_u128;
_2 = 3368999670488084098_i64 as isize;
_5 = true as isize;
_4 = 30237978852297984183803399753956556980_i128 as isize;
Goto(bb1)
}
bb1 = {
_8 = 125_i8;
_1 = &_6;
_3 = -_5;
_8 = 63_i8 - (-19_i8);
_6 = 115824608247394293472869246716554679074_u128 | 210872029742216779701203449702957809575_u128;
_3 = _4;
_5 = _2;
_1 = &_6;
_3 = _5 << (*_1);
_4 = _3 << _3;
_7 = '\u{92fd4}';
_4 = -_2;
_9.3.0 = _7;
_4 = _3 | _5;
_5 = _3;
_9.3.2 = 1917_u16 ^ 33224_u16;
_9.3.0 = _7;
_8 = 126_i8 + 49_i8;
_1 = &_6;
_1 = &(*_1);
_3 = _5 >> _6;
_12 = (-1936318339_i32) as f32;
Call(RET = fn3(_5, _9.3.0, (*_1), _3, _3, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.3.0 = _7;
_9.3.2 = !32127_u16;
_9.3.0 = _7;
_3 = !_2;
_3 = _4;
_9.3.2 = 58423_u16;
_9.0 = _9.3.2;
_11 = _4 << _5;
_9.3.0 = _7;
_1 = &(*_1);
_7 = _9.3.0;
_9.0 = _9.3.2 - _9.3.2;
_3 = _11;
_2 = !_4;
_9.3.1 = -_5;
_2 = _9.3.1 << _11;
_5 = !_2;
Goto(bb3)
}
bb3 = {
Call(_14 = dump_var(2_usize, 4_usize, Move(_4), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: char,mut _3: u128,mut _4: isize,mut _5: isize,mut _6: i8) -> *const bool {
mir! {
type RET = *const bool;
let _7: [i8; 3];
let _8: isize;
let _9: bool;
let _10: &'static &'static i16;
let _11: isize;
let _12: bool;
let _13: (&'static i128, isize, char);
let _14: [i16; 8];
let _15: [isize; 1];
let _16: &'static *const i32;
let _17: u64;
let _18: ();
let _19: ();
{
_1 = _5;
Call(RET = fn4(_1, _3, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = '\u{4786a}';
_6 = (-70_i8) * (-65_i8);
_4 = _1;
_1 = -_5;
_7 = [_6,_6,_6];
_8 = _4 & _4;
_2 = '\u{d5f2}';
_1 = _4;
_3 = 110758476872380934604659588843044863665_u128;
_5 = _8 | _8;
_9 = !true;
_4 = _5 | _8;
RET = core::ptr::addr_of!(_9);
RET = core::ptr::addr_of!(_9);
match _3 {
0 => bb2,
1 => bb3,
110758476872380934604659588843044863665 => bb5,
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
_2 = '\u{b5da6}';
_5 = _4 + _4;
_4 = _5;
_7 = [_6,_6,_6];
_11 = _5 >> _4;
_5 = 6034347442682345055_u64 as isize;
_2 = '\u{25bc8}';
_9 = _4 == _4;
_3 = 8348338632563458069_u64 as u128;
_4 = _11 * _8;
_5 = _8;
_11 = !_4;
_5 = _4 | _11;
_1 = _5;
_3 = 178_u8 as u128;
_3 = 216164666533006509008797616450020756255_u128;
Goto(bb6)
}
bb6 = {
_12 = (*RET);
_8 = !_11;
(*RET) = _12;
_13.1 = 7305590256892049784_u64 as isize;
_13.2 = _2;
(*RET) = !_12;
_3 = 85057524485149109223580502878331006707_u128;
match _3 {
0 => bb4,
1 => bb5,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
85057524485149109223580502878331006707 => bb11,
_ => bb10
}
}
bb7 = {
_2 = '\u{b5da6}';
_5 = _4 + _4;
_4 = _5;
_7 = [_6,_6,_6];
_11 = _5 >> _4;
_5 = 6034347442682345055_u64 as isize;
_2 = '\u{25bc8}';
_9 = _4 == _4;
_3 = 8348338632563458069_u64 as u128;
_4 = _11 * _8;
_5 = _8;
_11 = !_4;
_5 = _4 | _11;
_1 = _5;
_3 = 178_u8 as u128;
_3 = 216164666533006509008797616450020756255_u128;
Goto(bb6)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_2 = '\u{4786a}';
_6 = (-70_i8) * (-65_i8);
_4 = _1;
_1 = -_5;
_7 = [_6,_6,_6];
_8 = _4 & _4;
_2 = '\u{d5f2}';
_1 = _4;
_3 = 110758476872380934604659588843044863665_u128;
_5 = _8 | _8;
_9 = !true;
_4 = _5 | _8;
RET = core::ptr::addr_of!(_9);
RET = core::ptr::addr_of!(_9);
match _3 {
0 => bb2,
1 => bb3,
110758476872380934604659588843044863665 => bb5,
_ => bb4
}
}
bb11 = {
(*RET) = _11 > _8;
_6 = 4_usize as i8;
_3 = _4 as u128;
RET = core::ptr::addr_of!(_9);
_15 = [_11];
_13.1 = 13445_i16 as isize;
(*RET) = !_12;
_2 = _13.2;
_8 = 9149368040299268465_i64 as isize;
RET = core::ptr::addr_of!(_9);
_2 = _13.2;
Goto(bb12)
}
bb12 = {
Call(_18 = dump_var(3_usize, 3_usize, Move(_3), 1_usize, Move(_1), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_18 = dump_var(3_usize, 12_usize, Move(_12), 2_usize, Move(_2), 19_usize, _19, 19_usize, _19), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: u128,mut _3: isize) -> *const bool {
mir! {
type RET = *const bool;
let _4: isize;
let _5: [isize; 2];
let _6: f64;
let _7: f64;
let _8: isize;
let _9: isize;
let _10: &'static [u64; 3];
let _11: (char, isize, u16, &'static i128);
let _12: isize;
let _13: &'static ([u64; 3], &'static u128, f64, [u32; 1]);
let _14: f32;
let _15: bool;
let _16: bool;
let _17: *mut i32;
let _18: f64;
let _19: usize;
let _20: isize;
let _21: [isize; 1];
let _22: [isize; 2];
let _23: ();
let _24: ();
{
_3 = _1;
_3 = 5_usize as isize;
_2 = 10760939906328020217_u64 as u128;
_1 = true as isize;
_2 = 2158137812_u32 as u128;
_2 = 42738676731623420419548253639710820684_u128 * 67320356381958581688086226731155573835_u128;
_2 = 135477124354362842010183305726602209657_u128 >> _3;
_3 = !_1;
_5 = [_1,_1];
_4 = 794279748_i32 as isize;
_2 = !145627953143002695789802002506517158544_u128;
_1 = !_4;
_2 = 287175108456494436659724847408928266949_u128;
_2 = 299295629189887533366653993973446751885_u128;
_2 = 201025976568975363179757076708488885471_u128 * 193255981025843555730181726723011388200_u128;
_4 = _3 | _3;
_4 = 10862905120652380285_u64 as isize;
Call(_5 = fn5(_2, _2, _4, _3, _2, _3, _2, _3, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 113398636577801988676669914239495637316_u128 << _1;
_3 = _1;
_3 = _1 | _4;
_1 = 38906_u16 as isize;
_4 = !_3;
_4 = !_3;
_6 = 160_u8 as f64;
_5 = [_3,_3];
_1 = _2 as isize;
_2 = _4 as u128;
_5 = [_3,_4];
_7 = 74645246332115825054959284688314315580_i128 as f64;
_3 = _4;
_7 = 496699095_u32 as f64;
_2 = !163023233504377470914669538459933405911_u128;
_3 = !_4;
_5 = [_3,_4];
_4 = _1 - _3;
Call(_7 = fn8(_3, _1, _5, _1, _4, _4, _1, _1, _4, _6, _3, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = !_4;
_1 = '\u{1059ed}' as isize;
_5 = [_3,_4];
_6 = -_7;
_11.0 = '\u{2d521}';
_11.1 = 25_u8 as isize;
_2 = 148345443453770514241875290887664420123_i128 as u128;
_1 = -_8;
Goto(bb3)
}
bb3 = {
_5 = [_1,_8];
_1 = -_8;
_8 = _4;
_11.2 = true as u16;
_3 = !_1;
_18 = -_6;
RET = core::ptr::addr_of!(_15);
(*RET) = !false;
_9 = _3 ^ _3;
_14 = _2 as f32;
_15 = true & false;
_18 = -_6;
_16 = _18 <= _7;
_14 = 11081629042459147504_usize as f32;
_12 = -_8;
Goto(bb4)
}
bb4 = {
_18 = _6;
_18 = -_7;
(*RET) = !_16;
_5 = [_1,_8];
_7 = _18;
_18 = _6;
(*RET) = _3 >= _9;
_18 = _7;
(*RET) = !_16;
_7 = -_18;
_14 = 26483_i16 as f32;
RET = core::ptr::addr_of!(_15);
(*RET) = _4 == _9;
RET = core::ptr::addr_of!((*RET));
_16 = !(*RET);
_9 = 126_i8 as isize;
_8 = _4;
_1 = _3;
_9 = _4 << _12;
_7 = _18 * _6;
RET = core::ptr::addr_of!(_15);
_19 = !14518553743586111719_usize;
Goto(bb5)
}
bb5 = {
Call(_23 = dump_var(4_usize, 5_usize, Move(_5), 4_usize, Move(_4), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_23 = dump_var(4_usize, 1_usize, Move(_1), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: u128,mut _2: u128,mut _3: isize,mut _4: isize,mut _5: u128,mut _6: isize,mut _7: u128,mut _8: isize,mut _9: isize,mut _10: isize) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _11: *mut (u8, [u64; 3], u8, [char; 6]);
let _12: f64;
let _13: i32;
let _14: u16;
let _15: u32;
let _16: f64;
let _17: [isize; 8];
let _18: *mut i32;
let _19: u128;
let _20: *mut [isize; 1];
let _21: [isize; 1];
let _22: isize;
let _23: (Adt36, u32, i8);
let _24: (*mut i32, [u32; 7]);
let _25: ();
let _26: ();
{
RET = [_6,_6];
_4 = _8;
_12 = (-1393717767_i32) as f64;
_10 = 82_i8 as isize;
_1 = !_5;
_6 = _4;
_13 = 1786130794_i32 * 1332015759_i32;
_4 = _6;
RET = [_9,_8];
RET = [_3,_6];
_10 = 1290898901_u32 as isize;
_2 = _1 ^ _1;
_5 = _1 - _1;
_15 = 358822913_u32;
_2 = !_1;
_2 = !_7;
_9 = _10;
_1 = _2 - _7;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
358822913 => bb5,
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
_14 = _1 as u16;
_6 = _3 << _5;
_4 = false as isize;
_2 = _5 * _1;
_9 = _6 - _8;
_15 = 288235052_u32 | 3809125503_u32;
RET = [_8,_6];
_17 = [_9,_6,_6,_9,_6,_4,_8,_9];
_9 = _6;
RET = [_9,_4];
_10 = (-5472_i16) as isize;
_4 = 10856501923006391932_u64 as isize;
Goto(bb6)
}
bb6 = {
_16 = _12;
_5 = !_7;
Call(_4 = core::intrinsics::bswap(_9), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = -_9;
_16 = _12 + _12;
_19 = _2 << _7;
_18 = core::ptr::addr_of_mut!(_13);
_17 = [_9,_10,_6,_9,_8,_9,_10,_9];
Call(_3 = fn6(Move(_18), _6, _6, _8, _17, _14), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15 = 3682522647_u32;
_14 = 7532664873362798421_i64 as u16;
_1 = _19;
RET = [_3,_8];
_2 = _1 << _3;
_19 = !_2;
_4 = _13 as isize;
_5 = !_1;
_8 = _6;
_15 = 1268034658_u32;
_17 = [_10,_3,_6,_3,_3,_8,_6,_3];
_1 = !_7;
_22 = _15 as isize;
_19 = _2 | _2;
_21 = [_9];
_4 = _3 | _6;
_2 = 4069436683357103496_i64 as u128;
_2 = _19;
_18 = core::ptr::addr_of_mut!(_13);
_21 = [_3];
_18 = core::ptr::addr_of_mut!((*_18));
_13 = 126_i8 as i32;
(*_18) = (-966055865_i32) & (-1244668744_i32);
Goto(bb9)
}
bb9 = {
Call(_25 = dump_var(5_usize, 5_usize, Move(_5), 4_usize, Move(_4), 22_usize, Move(_22), 19_usize, Move(_19)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_25 = dump_var(5_usize, 9_usize, Move(_9), 7_usize, Move(_7), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: *mut i32,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 8],mut _6: u16) -> isize {
mir! {
type RET = isize;
let _7: i8;
let _8: &'static &'static i16;
let _9: ((u8, [u64; 3], u8, [char; 6]), i128, (char, isize, u16, &'static i128));
let _10: bool;
let _11: bool;
let _12: char;
let _13: &'static &'static i16;
let _14: f32;
let _15: ();
let _16: ();
{
RET = 7927452271435316947_u64 as isize;
_3 = _2;
RET = !_2;
RET = !_2;
_4 = !RET;
_5 = [_2,RET,RET,RET,_3,_4,RET,_3];
_5 = [_4,_2,_2,RET,RET,_4,_2,_2];
_6 = 26893_u16;
_5 = [RET,RET,_3,RET,RET,_2,_3,RET];
_4 = _2;
RET = -_4;
_5 = [_3,_2,_4,_4,_2,_2,RET,_4];
_6 = 10434_u16;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
10434 => bb6,
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
_6 = 46313_u16 + 32101_u16;
_5 = [_2,_4,_4,_3,_2,_2,_4,_3];
_2 = RET + _3;
Call(_6 = fn7(_5, _2, _5, _2, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _2;
_7 = !22_i8;
_7 = _6 as i8;
_3 = -_2;
_9.2.0 = '\u{888eb}';
_9.0.1 = [5079583226140359194_u64,14708782192507667825_u64,11258367163632981960_u64];
_2 = _3;
_9.0.1 = [9525443034088435909_u64,16698573408591003348_u64,550316912515778652_u64];
_9.2.3 = &_9.1;
_7 = 85_u8 as i8;
_2 = 85332367260059281674857212988598214832_u128 as isize;
_9.0.2 = 234_u8;
_9.2.1 = 3004864833_u32 as isize;
_7 = !(-116_i8);
_9.2.2 = _6 ^ _6;
_7 = 76_i8;
_9.1 = (-10383488391957967961073192399659336979_i128);
RET = _9.1 as isize;
Call(_6 = core::intrinsics::bswap(_9.2.2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9.2.2 = !_6;
_9.0.3 = [_9.2.0,_9.2.0,_9.2.0,_9.2.0,_9.2.0,_9.2.0];
_9.2.3 = &_9.1;
_10 = !false;
_4 = _3;
_9.0.0 = 14757372878633712416_usize as u8;
_9.0.2 = _9.0.0;
RET = -_4;
_3 = (-26445_i16) as isize;
_9.0.1 = [11737866670873484189_u64,4483521017996839218_u64,1547903827566063159_u64];
_9.2.2 = _6;
_4 = _9.2.1 >> _9.2.2;
_10 = false | false;
_4 = RET;
_9.2.0 = '\u{9ca86}';
_9.0.2 = _9.0.0 ^ _9.0.0;
_3 = _4;
RET = !_2;
_4 = !_3;
_10 = true;
_9.0.2 = _9.0.0;
match _7 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb6,
76 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_9.0.1 = [12223272140747163808_u64,12646641768924735621_u64,11075669434498149889_u64];
_9.2.2 = _7 as u16;
_7 = 109_i8 + (-51_i8);
_11 = _10;
_10 = _11;
_12 = _9.2.0;
_7 = _12 as i8;
RET = _4 * _4;
_9.2.3 = &_9.1;
_10 = _11 & _11;
_9.2.0 = _12;
_9.0.2 = !_9.0.0;
_5 = [RET,RET,_3,RET,_4,RET,_3,_3];
_9.2.2 = _6 >> _6;
Goto(bb11)
}
bb11 = {
Call(_15 = dump_var(6_usize, 6_usize, Move(_6), 2_usize, Move(_2), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [isize; 8],mut _2: isize,mut _3: [isize; 8],mut _4: isize,mut _5: isize) -> u16 {
mir! {
type RET = u16;
let _6: u64;
let _7: [char; 6];
let _8: i64;
let _9: u8;
let _10: [i16; 8];
let _11: i32;
let _12: isize;
let _13: *const u128;
let _14: u64;
let _15: *mut (u8, [u64; 3], u8, [char; 6]);
let _16: [isize; 8];
let _17: &'static &'static i16;
let _18: [bool; 2];
let _19: ();
let _20: ();
{
RET = 55169_u16;
_4 = _2 | _5;
RET = 25493_u16 + 43245_u16;
RET = true as u16;
RET = 7226955969370305411_usize as u16;
_4 = _2 - _5;
RET = 177589173602793397861417025154310230230_u128 as u16;
_7 = ['\u{32239}','\u{34aa1}','\u{2f2c3}','\u{d73d1}','\u{f3aa1}','\u{9a401}'];
_5 = _4;
_3 = _1;
_2 = _4 - _5;
RET = 23852_u16;
_6 = !7859246094450181639_u64;
_4 = -_2;
_8 = (-1444124057473421048_i64) | 7823837523627678662_i64;
RET = !55965_u16;
RET = 6021_u16;
_2 = -_5;
_10 = [(-7753_i16),(-12830_i16),(-6972_i16),20906_i16,750_i16,(-29993_i16),(-29773_i16),(-971_i16)];
Goto(bb1)
}
bb1 = {
_6 = 9928564357841940880_u64 + 2047873516566530642_u64;
_11 = 881348230_i32 + (-454405265_i32);
_6 = !136108666996668848_u64;
_5 = 41346398338367950011368060516415371928_u128 as isize;
_10 = [(-31742_i16),(-15908_i16),8307_i16,(-12882_i16),21708_i16,7781_i16,31274_i16,27823_i16];
_7 = ['\u{68b3a}','\u{9f065}','\u{fd6b1}','\u{644bc}','\u{ce8d}','\u{2f219}'];
_11 = 875879844_i32 ^ 1287519928_i32;
_11 = 315334769_i32 << _2;
_11 = (-170608847_i32);
_4 = -_2;
_5 = !_2;
_12 = _5;
_10 = [(-18937_i16),(-18361_i16),(-22713_i16),1287_i16,5153_i16,2505_i16,(-15143_i16),(-24841_i16)];
_9 = 218_u8 | 199_u8;
_3 = [_12,_5,_5,_5,_5,_2,_2,_12];
Goto(bb2)
}
bb2 = {
RET = !53851_u16;
_1 = _3;
_1 = _3;
RET = 2019_u16;
_9 = 12_u8 * 236_u8;
_10 = [3981_i16,(-28891_i16),13238_i16,(-11113_i16),(-28458_i16),2957_i16,(-30631_i16),23521_i16];
_9 = !125_u8;
_5 = _4;
_6 = 7285850508404909213_u64 - 15840026621475977241_u64;
_10 = [8435_i16,5441_i16,17732_i16,(-4691_i16),(-9925_i16),(-991_i16),17165_i16,4618_i16];
_7 = ['\u{3830}','\u{69f00}','\u{a87a1}','\u{59d3f}','\u{2c4cd}','\u{d9725}'];
_7 = ['\u{624b4}','\u{93f95}','\u{8ae35}','\u{73fdd}','\u{75c1c}','\u{df77c}'];
_1 = [_4,_4,_5,_2,_5,_5,_12,_2];
RET = 34397_u16;
_11 = 2041949424_i32;
_7 = ['\u{6acc5}','\u{584cb}','\u{f52c9}','\u{a3512}','\u{f131c}','\u{10248a}'];
RET = !55302_u16;
RET = !29246_u16;
_5 = 193255957051180312470513902243453010563_u128 as isize;
_12 = -_4;
_12 = _4;
_11 = (-1805372156_i32) ^ (-367743483_i32);
_11 = 3941956936_u32 as i32;
_16 = [_2,_2,_12,_5,_2,_4,_2,_12];
_3 = [_5,_4,_2,_4,_12,_4,_4,_2];
_14 = _6;
Goto(bb3)
}
bb3 = {
_14 = _6;
_3 = _1;
_5 = _12 ^ _12;
_2 = 12731_i16 as isize;
_1 = [_5,_5,_12,_12,_12,_5,_5,_12];
_1 = [_5,_5,_4,_12,_5,_5,_4,_4];
_8 = -4037045274115808658_i64;
_8 = 4679731396638771373_i64 * (-4178310225893809595_i64);
_11 = RET as i32;
_5 = _4;
_1 = [_5,_4,_4,_4,_12,_5,_5,_5];
RET = _12 as u16;
_10 = [(-32000_i16),6451_i16,29150_i16,23803_i16,26847_i16,(-26165_i16),11904_i16,(-12255_i16)];
_3 = [_5,_4,_12,_12,_5,_12,_5,_12];
_16 = _1;
_6 = !_14;
_10 = [(-28859_i16),(-12218_i16),12969_i16,(-32372_i16),(-18251_i16),(-17731_i16),3905_i16,(-27190_i16)];
Goto(bb4)
}
bb4 = {
Call(_19 = dump_var(7_usize, 11_usize, Move(_11), 8_usize, Move(_8), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_19 = dump_var(7_usize, 12_usize, Move(_12), 14_usize, Move(_14), 1_usize, Move(_1), 20_usize, _20), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: isize,mut _3: [isize; 2],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: f64,mut _11: isize,mut _12: isize) -> f64 {
mir! {
type RET = f64;
let _13: Adt36;
let _14: bool;
let _15: bool;
let _16: &'static *const isize;
let _17: ();
let _18: ();
{
_3 = [_8,_6];
RET = 3505170896033337614696494607049504616_u128 as f64;
_7 = _5 & _5;
_5 = (-113_i8) as isize;
_3 = [_12,_2];
_9 = !_6;
Goto(bb1)
}
bb1 = {
_1 = _6 ^ _11;
_11 = _1 ^ _7;
_10 = -RET;
_9 = _1;
_5 = !_11;
_7 = -_12;
_9 = -_1;
_10 = -RET;
_14 = !true;
_14 = !true;
RET = _10;
RET = 204_u8 as f64;
_8 = _5;
_10 = RET;
_3 = [_8,_11];
_1 = -_9;
_2 = 103222851936434721757927387003900854598_u128 as isize;
_2 = -_1;
_6 = !_9;
_10 = -RET;
_12 = -_9;
Goto(bb2)
}
bb2 = {
_14 = !false;
_8 = 218719335892381143063465649251153724768_u128 as isize;
RET = _11 as f64;
_7 = _6 - _12;
Goto(bb3)
}
bb3 = {
Call(_17 = dump_var(8_usize, 9_usize, Move(_9), 8_usize, Move(_8), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_17 = dump_var(8_usize, 7_usize, Move(_7), 4_usize, Move(_4), 18_usize, _18, 18_usize, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: &'static f64,mut _2: &'static i128) -> char {
mir! {
type RET = char;
let _3: usize;
let _4: *mut u16;
let _5: (f32, (&'static u32, &'static i16, (&'static i16,)));
let _6: ([u64; 6], char, [u32; 1], f64);
let _7: i64;
let _8: [u32; 1];
let _9: (*mut i32, [u32; 7]);
let _10: *const isize;
let _11: bool;
let _12: [u32; 7];
let _13: ();
let _14: ();
{
RET = '\u{39af3}';
RET = '\u{71466}';
RET = '\u{e1541}';
RET = '\u{84acb}';
RET = '\u{3c37a}';
RET = '\u{90382}';
RET = '\u{cb7cd}';
RET = '\u{1290f}';
RET = '\u{625c9}';
RET = '\u{69c8b}';
RET = '\u{63df6}';
RET = '\u{a80db}';
RET = '\u{58be}';
_3 = 164573266923527284_i64 as usize;
_3 = RET as usize;
_3 = 8750484805050498511_u64 as usize;
RET = '\u{105e67}';
RET = '\u{6d235}';
RET = '\u{557a4}';
_3 = 3_usize << 146197841856652605661694718312074054607_u128;
RET = '\u{5855b}';
RET = '\u{c58ed}';
_3 = !7_usize;
_5.0 = 6507675754010348206_i64 as f32;
Goto(bb1)
}
bb1 = {
RET = '\u{a3e5b}';
RET = '\u{5d7e0}';
_3 = 11422032208413988155_usize | 7380551125319153378_usize;
RET = '\u{a67cf}';
_3 = 3_usize;
RET = '\u{7dbf9}';
_5.0 = 40524742718118193173848253335248830490_i128 as f32;
_5.0 = (-80_i8) as f32;
RET = '\u{2249e}';
_3 = 15867121043240090438_usize;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
15867121043240090438 => bb6,
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
_3 = 16097155882024880126_usize | 4_usize;
RET = '\u{19dd2}';
RET = '\u{67a9b}';
RET = '\u{585da}';
RET = '\u{170eb}';
RET = '\u{f85b1}';
_6.1 = RET;
_6.1 = RET;
_6.1 = RET;
RET = _6.1;
_6.3 = _5.0 as f64;
_6.2 = [3183923178_u32];
_6.0 = [8883929363612970484_u64,13415780402459829085_u64,15348634848828167143_u64,14515749209731037133_u64,553713633299288669_u64,16992085107456591744_u64];
_6.1 = RET;
Goto(bb7)
}
bb7 = {
_1 = &_6.3;
_7 = (-2602775812907858992_i64);
_6.1 = RET;
RET = _6.1;
_6.0 = [8793240518142217079_u64,5044196166805384384_u64,2818823240017443830_u64,12042248218072784209_u64,5010997225545863900_u64,14086305705219184644_u64];
RET = _6.1;
_6.2 = [1683500763_u32];
_7 = 7906057461776916589_i64 >> _3;
_6.1 = RET;
_5.0 = (-1610550531_i32) as f32;
_6.2 = [3332925142_u32];
_6.0 = [2105592810245285167_u64,8205149038043808914_u64,9551550491014992373_u64,16657845621326585176_u64,17039478192547920684_u64,7638418900469768179_u64];
RET = _6.1;
_7 = -(-197434798343815172_i64);
_6.3 = 2396639183909523524_u64 as f64;
_6.3 = 97_i8 as f64;
RET = _6.1;
_1 = &_6.3;
_8 = _6.2;
Goto(bb8)
}
bb8 = {
_3 = 10767866226504815131_usize;
_6.2 = [292910529_u32];
Goto(bb9)
}
bb9 = {
_6.0 = [623805650879714103_u64,1191835392395653499_u64,15451633438118933244_u64,16242123683995492296_u64,5429029867701979875_u64,10588069031995095905_u64];
RET = _6.1;
RET = _6.1;
_5.0 = 19801_i16 as f32;
_6.2 = [1216375470_u32];
match _3 {
0 => bb7,
1 => bb10,
2 => bb11,
3 => bb12,
10767866226504815131 => bb14,
_ => bb13
}
}
bb10 = {
_3 = 10767866226504815131_usize;
_6.2 = [292910529_u32];
Goto(bb9)
}
bb11 = {
_1 = &_6.3;
_7 = (-2602775812907858992_i64);
_6.1 = RET;
RET = _6.1;
_6.0 = [8793240518142217079_u64,5044196166805384384_u64,2818823240017443830_u64,12042248218072784209_u64,5010997225545863900_u64,14086305705219184644_u64];
RET = _6.1;
_6.2 = [1683500763_u32];
_7 = 7906057461776916589_i64 >> _3;
_6.1 = RET;
_5.0 = (-1610550531_i32) as f32;
_6.2 = [3332925142_u32];
_6.0 = [2105592810245285167_u64,8205149038043808914_u64,9551550491014992373_u64,16657845621326585176_u64,17039478192547920684_u64,7638418900469768179_u64];
RET = _6.1;
_7 = -(-197434798343815172_i64);
_6.3 = 2396639183909523524_u64 as f64;
_6.3 = 97_i8 as f64;
RET = _6.1;
_1 = &_6.3;
_8 = _6.2;
Goto(bb8)
}
bb12 = {
_3 = 16097155882024880126_usize | 4_usize;
RET = '\u{19dd2}';
RET = '\u{67a9b}';
RET = '\u{585da}';
RET = '\u{170eb}';
RET = '\u{f85b1}';
_6.1 = RET;
_6.1 = RET;
_6.1 = RET;
RET = _6.1;
_6.3 = _5.0 as f64;
_6.2 = [3183923178_u32];
_6.0 = [8883929363612970484_u64,13415780402459829085_u64,15348634848828167143_u64,14515749209731037133_u64,553713633299288669_u64,16992085107456591744_u64];
_6.1 = RET;
Goto(bb7)
}
bb13 = {
RET = '\u{a3e5b}';
RET = '\u{5d7e0}';
_3 = 11422032208413988155_usize | 7380551125319153378_usize;
RET = '\u{a67cf}';
_3 = 3_usize;
RET = '\u{7dbf9}';
_5.0 = 40524742718118193173848253335248830490_i128 as f32;
_5.0 = (-80_i8) as f32;
RET = '\u{2249e}';
_3 = 15867121043240090438_usize;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
15867121043240090438 => bb6,
_ => bb5
}
}
bb14 = {
_9.1 = [631462222_u32,2022494938_u32,3217023838_u32,643206153_u32,4204065067_u32,815680317_u32,177793073_u32];
_7 = RET as i64;
_6.2 = [130511684_u32];
_6.0 = [7353296755989854438_u64,9094998004196218498_u64,10489574080302649343_u64,9020269824898440614_u64,11586505410567030451_u64,10149257439531656291_u64];
_5.0 = 20129_u16 as f32;
_6.3 = 108_i8 as f64;
RET = _6.1;
RET = _6.1;
_6.1 = RET;
RET = _6.1;
_6.2 = [3323800610_u32];
_7 = (-56_i8) as i64;
_6.0 = [2655999004670243071_u64,7417887205407857626_u64,10277439772137086630_u64,9086351356017813086_u64,13760922918931499363_u64,17174390394025380436_u64];
_12 = [723037981_u32,2390532126_u32,2778163938_u32,540697281_u32,766986609_u32,2376897281_u32,1491051510_u32];
_11 = _5.0 != _5.0;
_1 = &_6.3;
_5.0 = _3 as f32;
_3 = 83_u8 as usize;
_6.1 = RET;
_8 = [3969128963_u32];
_8 = [2632501417_u32];
_9.1 = [2745714710_u32,3735848677_u32,2654239370_u32,4282480941_u32,3346298725_u32,86192944_u32,2588850108_u32];
Goto(bb15)
}
bb15 = {
Call(_13 = dump_var(9_usize, 12_usize, Move(_12), 3_usize, Move(_3), 14_usize, _14, 14_usize, _14), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: char,mut _3: u16,mut _4: isize) -> *const [u32; 7] {
mir! {
type RET = *const [u32; 7];
let _5: i64;
let _6: [char; 6];
let _7: [isize; 8];
let _8: isize;
let _9: ([u64; 3], &'static u128, f64, [u32; 1]);
let _10: *mut u16;
let _11: f64;
let _12: *const isize;
let _13: [u32; 7];
let _14: u64;
let _15: f64;
let _16: Adt45;
let _17: [char; 6];
let _18: f32;
let _19: f32;
let _20: isize;
let _21: isize;
let _22: *const i32;
let _23: isize;
let _24: (*mut i32, [u32; 7]);
let _25: [isize; 1];
let _26: Adt45;
let _27: ();
let _28: ();
{
_2 = '\u{acd49}';
_1 = 152_u8 as isize;
_4 = _1;
_2 = '\u{6e455}';
_5 = 247_u8 as i64;
_4 = 138308104756745998503426825187348354456_i128 as isize;
_7 = [_1,_4,_1,_1,_4,_4,_4,_4];
_4 = (-330410057_i32) as isize;
_8 = (-111_i8) as isize;
_6 = [_2,_2,_2,_2,_2,_2];
_4 = _8;
Call(_1 = fn11(_8, _7, _3, _4, _8, _2, _2, _7, _4, _5, _5, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.0 = [11839966927770024452_u64,8704621361082806352_u64,14396424328488448055_u64];
_9.3 = [2655518155_u32];
_10 = core::ptr::addr_of_mut!(_3);
Goto(bb2)
}
bb2 = {
_4 = -_1;
_8 = _1 & _1;
_9.2 = (-16_i8) as f64;
_11 = -_9.2;
_9.3 = [2539935703_u32];
_9.0 = [1917511013527443033_u64,13957305408388032495_u64,7306444791819073453_u64];
_1 = 208033193323555720400128081912379812106_u128 as isize;
(*_10) = 39759_u16;
_1 = _8 << _8;
_10 = core::ptr::addr_of_mut!((*_10));
_9.2 = _11 + _11;
_2 = '\u{5e13d}';
RET = core::ptr::addr_of!(_13);
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
39759 => bb7,
_ => bb6
}
}
bb3 = {
_9.0 = [11839966927770024452_u64,8704621361082806352_u64,14396424328488448055_u64];
_9.3 = [2655518155_u32];
_10 = core::ptr::addr_of_mut!(_3);
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
_10 = core::ptr::addr_of_mut!(_3);
_5 = (-2166795636738732735_i64);
(*RET) = [2801975653_u32,3002589679_u32,562362068_u32,1393665503_u32,286809348_u32,1801516796_u32,853161557_u32];
(*RET) = [473131919_u32,3168641131_u32,1235299035_u32,1413088947_u32,3012255614_u32,4048311618_u32,3916866342_u32];
_10 = core::ptr::addr_of_mut!(_3);
_5 = 480337531901188341_i64;
_6 = [_2,_2,_2,_2,_2,_2];
_5 = 4355719935847821399_u64 as i64;
_9.3 = [310692330_u32];
_8 = _4 & _1;
Call((*_10) = fn13(_1, _8, Move(RET), _8, _1, _1, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = _8;
_12 = core::ptr::addr_of!(_4);
_13 = [3942456394_u32,3934604657_u32,2587959447_u32,2389006246_u32,307877998_u32,2633270055_u32,4015190333_u32];
Goto(bb9)
}
bb9 = {
Goto(bb10)
}
bb10 = {
(*_12) = (-15420_i16) as isize;
_3 = 2_usize as u16;
_13 = [3907884207_u32,1056582250_u32,3714294972_u32,704613475_u32,3188156141_u32,2213848031_u32,3717988946_u32];
(*_10) = 40521_u16;
_12 = core::ptr::addr_of!((*_12));
RET = core::ptr::addr_of!(_13);
_15 = -_9.2;
_12 = core::ptr::addr_of!(_4);
(*RET) = [2760233857_u32,1342100104_u32,3634849651_u32,368719959_u32,2642687959_u32,1957801068_u32,287919453_u32];
(*RET) = [3948588696_u32,4160030245_u32,640821626_u32,4170114099_u32,2864048371_u32,4130613994_u32,1935196049_u32];
_1 = _8 >> _8;
(*_10) = !21242_u16;
(*RET) = [3288901109_u32,2840635789_u32,1301310714_u32,1162820535_u32,1604075305_u32,2673448499_u32,2639307771_u32];
_18 = 134426670821925336697607286738578341853_i128 as f32;
_4 = 91861281892361298604139939856429399448_i128 as isize;
_4 = _8 + _1;
_11 = _9.2 - _9.2;
_15 = _5 as f64;
_14 = 15308612107605417783_u64;
_9.3 = [3713076791_u32];
_6 = [_2,_2,_2,_2,_2,_2];
_10 = core::ptr::addr_of_mut!((*_10));
_9.0 = [_14,_14,_14];
Goto(bb11)
}
bb11 = {
_13 = [1769219604_u32,797059878_u32,2777203165_u32,4038373085_u32,839065482_u32,1816526804_u32,1951436972_u32];
(*RET) = [724092871_u32,2763123121_u32,1450181693_u32,3861319156_u32,2851222642_u32,3590924187_u32,2354950102_u32];
(*_12) = _9.2 as isize;
(*RET) = [2493366358_u32,1938723516_u32,3119910882_u32,333433240_u32,3997303587_u32,2043961639_u32,2016801693_u32];
_5 = 1299327783733251438_i64 >> _1;
_19 = _18;
(*_12) = -_1;
(*_10) = !52394_u16;
(*RET) = [3923051161_u32,4191024072_u32,2203495194_u32,1262067592_u32,2934183814_u32,1536032435_u32,4156196187_u32];
_2 = '\u{fa2a5}';
_20 = 24616_i16 as isize;
(*RET) = [4227429719_u32,256333234_u32,3717916750_u32,3688741421_u32,3460925741_u32,3366329544_u32,2615903172_u32];
_17 = [_2,_2,_2,_2,_2,_2];
(*RET) = [4166147426_u32,767964762_u32,3717452224_u32,2338830503_u32,2873741565_u32,438945829_u32,705666541_u32];
_15 = 47222548352536513581983426139564655232_i128 as f64;
_24.1 = [2610404481_u32,2408039962_u32,218789552_u32,1161532494_u32,33048837_u32,1604839291_u32,240587005_u32];
_15 = _9.2 + _9.2;
_3 = !16887_u16;
_12 = core::ptr::addr_of!(_4);
_5 = 2670766302335014474_i64 - 4875907006785223078_i64;
_20 = (*_12) - _4;
_17 = _6;
_12 = core::ptr::addr_of!((*_12));
_24.1 = [2637027633_u32,1966181254_u32,1245422256_u32,1341636700_u32,1923925414_u32,980818978_u32,2423420926_u32];
match _14 {
0 => bb8,
1 => bb9,
2 => bb3,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
15308612107605417783 => bb17,
_ => bb16
}
}
bb12 = {
(*_12) = (-15420_i16) as isize;
_3 = 2_usize as u16;
_13 = [3907884207_u32,1056582250_u32,3714294972_u32,704613475_u32,3188156141_u32,2213848031_u32,3717988946_u32];
(*_10) = 40521_u16;
_12 = core::ptr::addr_of!((*_12));
RET = core::ptr::addr_of!(_13);
_15 = -_9.2;
_12 = core::ptr::addr_of!(_4);
(*RET) = [2760233857_u32,1342100104_u32,3634849651_u32,368719959_u32,2642687959_u32,1957801068_u32,287919453_u32];
(*RET) = [3948588696_u32,4160030245_u32,640821626_u32,4170114099_u32,2864048371_u32,4130613994_u32,1935196049_u32];
_1 = _8 >> _8;
(*_10) = !21242_u16;
(*RET) = [3288901109_u32,2840635789_u32,1301310714_u32,1162820535_u32,1604075305_u32,2673448499_u32,2639307771_u32];
_18 = 134426670821925336697607286738578341853_i128 as f32;
_4 = 91861281892361298604139939856429399448_i128 as isize;
_4 = _8 + _1;
_11 = _9.2 - _9.2;
_15 = _5 as f64;
_14 = 15308612107605417783_u64;
_9.3 = [3713076791_u32];
_6 = [_2,_2,_2,_2,_2,_2];
_10 = core::ptr::addr_of_mut!((*_10));
_9.0 = [_14,_14,_14];
Goto(bb11)
}
bb13 = {
Goto(bb10)
}
bb14 = {
_9.0 = [11839966927770024452_u64,8704621361082806352_u64,14396424328488448055_u64];
_9.3 = [2655518155_u32];
_10 = core::ptr::addr_of_mut!(_3);
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_4 = -_1;
_8 = _1 & _1;
_9.2 = (-16_i8) as f64;
_11 = -_9.2;
_9.3 = [2539935703_u32];
_9.0 = [1917511013527443033_u64,13957305408388032495_u64,7306444791819073453_u64];
_1 = 208033193323555720400128081912379812106_u128 as isize;
(*_10) = 39759_u16;
_1 = _8 << _8;
_10 = core::ptr::addr_of_mut!((*_10));
_9.2 = _11 + _11;
_2 = '\u{5e13d}';
RET = core::ptr::addr_of!(_13);
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
39759 => bb7,
_ => bb6
}
}
bb17 = {
(*_10) = 24819_u16;
_21 = !(*_12);
_8 = 5312_i16 as isize;
(*_10) = 6148_u16;
_4 = _21;
(*RET) = [1354205092_u32,25121673_u32,1550118243_u32,2468432639_u32,1891093277_u32,1740281470_u32,1788848757_u32];
_12 = core::ptr::addr_of!(_1);
_9.3 = [2727287104_u32];
_25 = [(*_12)];
_15 = _11;
_10 = core::ptr::addr_of_mut!((*_10));
Goto(bb18)
}
bb18 = {
Call(_27 = dump_var(10_usize, 20_usize, Move(_20), 8_usize, Move(_8), 4_usize, Move(_4), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(10_usize, 13_usize, Move(_13), 6_usize, Move(_6), 2_usize, Move(_2), 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: [isize; 8],mut _3: u16,mut _4: isize,mut _5: isize,mut _6: char,mut _7: char,mut _8: [isize; 8],mut _9: isize,mut _10: i64,mut _11: i64,mut _12: [isize; 8]) -> isize {
mir! {
type RET = isize;
let _13: isize;
let _14: (&'static &'static i16, u64);
let _15: (u16, Adt45, *mut i32, (char, isize, u16, &'static i128));
let _16: [u64; 6];
let _17: isize;
let _18: [usize; 2];
let _19: isize;
let _20: u8;
let _21: bool;
let _22: Adt81;
let _23: isize;
let _24: f64;
let _25: [isize; 6];
let _26: f32;
let _27: i16;
let _28: [isize; 6];
let _29: [u32; 7];
let _30: u8;
let _31: (&'static i16,);
let _32: *const i32;
let _33: *mut i32;
let _34: char;
let _35: u64;
let _36: isize;
let _37: u16;
let _38: (&'static i16,);
let _39: Adt45;
let _40: f64;
let _41: *mut i32;
let _42: Adt45;
let _43: Adt45;
let _44: (u16, Adt45, *mut i32, (char, isize, u16, &'static i128));
let _45: [i8; 3];
let _46: bool;
let _47: ();
let _48: ();
{
_9 = (-19395_i16) as isize;
_8 = [_1,_4,_9,_4,_5,_4,_1,_5];
_6 = _7;
_9 = !_4;
_10 = _11;
_13 = !_1;
_7 = _6;
_15.3.1 = -_9;
_15.3.1 = !_1;
RET = _4;
_10 = _11;
_14.1 = 13546454746129578949_u64;
_3 = 52781_u16 ^ 52003_u16;
_15.3.1 = !_13;
_1 = false as isize;
_4 = _9;
Goto(bb1)
}
bb1 = {
_4 = _13;
_15.3.0 = _7;
_3 = _15.3.0 as u16;
_15.3.0 = _7;
_5 = -_15.3.1;
_15.0 = 6339_i16 as u16;
_7 = _6;
RET = _5 + _9;
_13 = RET + _9;
_8 = [RET,_13,_1,_9,_9,_13,_1,_13];
_5 = (-4061_i16) as isize;
_9 = _4 | RET;
_9 = -_13;
_15.0 = !_3;
_4 = 10814904165419274938_usize as isize;
_15.3.2 = 13971_i16 as u16;
RET = !_15.3.1;
_1 = 21438_i16 as isize;
_4 = _14.1 as isize;
Goto(bb2)
}
bb2 = {
_7 = _15.3.0;
_18 = [3_usize,6_usize];
_8 = [_15.3.1,_9,_13,_9,_1,_13,_1,_9];
_9 = (-121_i8) as isize;
_15.3.2 = _3 + _15.0;
_18 = [10683431146391331728_usize,5_usize];
RET = _9;
_2 = [_5,_15.3.1,_1,_5,RET,_9,RET,_13];
_16 = [_14.1,_14.1,_14.1,_14.1,_14.1,_14.1];
_9 = _4 - _5;
_2 = [_13,_9,_5,_9,_4,_13,RET,_4];
_15.3.2 = !_15.0;
_21 = false | true;
_9 = _5 ^ _1;
_21 = false ^ true;
_5 = -RET;
_15.3.1 = _4 >> _4;
_21 = false ^ true;
_8 = [RET,_5,RET,_15.3.1,_9,_15.3.1,_15.3.1,_13];
_15.3.2 = _21 as u16;
_17 = !RET;
_7 = _6;
_19 = _1 ^ _15.3.1;
_9 = 3375620896590391033_usize as isize;
_18 = [13969699273870366796_usize,11664451958049367639_usize];
_15.3.2 = _6 as u16;
Goto(bb3)
}
bb3 = {
_20 = !186_u8;
_19 = _15.3.1 ^ _9;
_5 = -_4;
RET = _21 as isize;
_23 = _13 ^ RET;
_19 = -_1;
_13 = 183151274959993436137384800673626446966_u128 as isize;
_7 = _15.3.0;
_10 = !_11;
_18 = [7_usize,5449249260709387804_usize];
_24 = _11 as f64;
_14.1 = !5022962748487650296_u64;
_16 = [_14.1,_14.1,_14.1,_14.1,_14.1,_14.1];
_27 = 30002_i16;
RET = _9;
_15.0 = _3;
_13 = RET;
_29 = [2126680341_u32,1832172848_u32,3448387788_u32,2851193262_u32,4166995382_u32,2453833282_u32,2235865719_u32];
_13 = _20 as isize;
_27 = 16701_i16 * 66_i16;
Call(_23 = core::intrinsics::transmute(_15.3.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15.0 = _15.3.0 as u16;
_13 = !_5;
_25 = [_15.3.1,_4,_23,_9,_5,_15.3.1];
_5 = _27 as isize;
Call(_10 = core::intrinsics::bswap(_11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14.1 = 79_i8 as u64;
_30 = _20 & _20;
_30 = _20;
_18 = [15585780378765940336_usize,4_usize];
_2 = _8;
_12 = [_13,_13,_15.3.1,RET,_4,_23,_1,_19];
_11 = -_10;
_9 = _20 as isize;
RET = _5 | _4;
_24 = 13867660419732876833_usize as f64;
_8 = [_15.3.1,RET,_5,_15.3.1,_5,_15.3.1,_23,_9];
_10 = _11;
_25 = [_15.3.1,_23,RET,RET,_1,RET];
_4 = _11 as isize;
_7 = _15.3.0;
_8 = [_1,_1,_13,_15.3.1,_23,_15.3.1,RET,_1];
_1 = _11 as isize;
_13 = RET;
_4 = _9;
_19 = -RET;
_18 = [8299365123224910045_usize,13457320974987863365_usize];
_19 = -_23;
_15.3.2 = _15.0;
_13 = -_4;
_14.1 = 3731658190732038476_u64;
_26 = (-2_i8) as f32;
_20 = !_30;
_31.0 = &_27;
_3 = !_15.0;
_14.0 = &_31.0;
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26 = _10 as f32;
_26 = _20 as f32;
_26 = 1539456548_u32 as f32;
_10 = _11;
Goto(bb7)
}
bb7 = {
_19 = _23;
_12 = [_23,_9,_19,_1,_23,_5,_15.3.1,_17];
_17 = _13 ^ _23;
_24 = 147873329572769573707551407999095736586_u128 as f64;
_19 = -_17;
_27 = 29482_i16 ^ (-5242_i16);
_25 = [_19,_4,_19,_15.3.1,_17,_15.3.1];
_34 = _7;
_26 = 234965158709591188867429617905527057019_u128 as f32;
_10 = _11;
_15.0 = _3;
_15.0 = (-20495942854853150305435373435974410352_i128) as u16;
_25 = [_19,_19,_17,_19,_19,_4];
_17 = _19 * _19;
_26 = _14.1 as f32;
_35 = !_14.1;
_21 = false;
_12 = [_9,_17,_17,_9,_15.3.1,_15.3.1,_5,_17];
_11 = _10;
_25 = [_15.3.1,_19,_5,_17,_17,_13];
_29 = [2998923647_u32,3336400583_u32,3078275577_u32,1386584296_u32,1719812079_u32,4162326776_u32,4259678955_u32];
_25 = [_5,_17,_15.3.1,_5,_19,_17];
Call(_5 = core::intrinsics::transmute(_10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = _15.0 as isize;
_15.0 = !_15.3.2;
_24 = 1665507163_i32 as f64;
_31.0 = &_27;
_9 = _17 | _19;
_11 = _10 ^ _10;
_25 = [RET,_9,_17,_15.3.1,_9,_9];
_23 = _9;
_9 = 250540457078669293326835535584942904715_u128 as isize;
_36 = _23;
Call(_15.3.1 = core::intrinsics::bswap(_36), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_27 = (-14938_i16) * 23957_i16;
_24 = 3732785588_u32 as f64;
_5 = _36;
_15.3.1 = _5;
_14.0 = &_31.0;
_25 = [_23,_4,_5,_36,_23,_5];
_17 = _36 << _23;
_37 = _15.3.2;
_28 = [_36,_17,_15.3.1,_5,_23,_17];
Goto(bb10)
}
bb10 = {
_15.3.1 = _35 as isize;
_4 = !_36;
_15.3.1 = !_19;
_1 = _15.3.1 + _4;
_4 = _7 as isize;
RET = !_15.3.1;
_14.0 = &_31.0;
_40 = _24;
_10 = _27 as i64;
_15.0 = (-126745203255926759496044026628067456043_i128) as u16;
_38.0 = &_27;
_40 = _24 + _24;
_23 = _17 - _36;
_14.0 = &_31.0;
_6 = _15.3.0;
Goto(bb11)
}
bb11 = {
_25 = _28;
Call(_1 = fn12(Move(_38), _17, _36, _15.3.1, _23, _23), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_11 = _7 as i64;
_6 = _34;
_14.1 = _35;
_14.1 = _35;
Goto(bb13)
}
bb13 = {
_1 = -_36;
_17 = _23;
RET = _23 ^ _5;
_1 = _17;
_14.0 = &_31.0;
_15.0 = _1 as u16;
_34 = _15.3.0;
Goto(bb14)
}
bb14 = {
Call(_47 = dump_var(11_usize, 28_usize, Move(_28), 5_usize, Move(_5), 25_usize, Move(_25), 30_usize, Move(_30)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_47 = dump_var(11_usize, 16_usize, Move(_16), 11_usize, Move(_11), 23_usize, Move(_23), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(11_usize, 10_usize, Move(_10), 36_usize, Move(_36), 21_usize, Move(_21), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(11_usize, 29_usize, Move(_29), 20_usize, Move(_20), 48_usize, _48, 48_usize, _48), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (&'static i16,),mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> isize {
mir! {
type RET = isize;
let _7: &'static i128;
let _8: *const (char, isize, u16, &'static i128);
let _9: ();
let _10: ();
{
_6 = !_4;
_6 = !_2;
_3 = !_5;
RET = !_5;
_6 = _5;
_2 = _5;
_6 = _2;
RET = _5 & _2;
RET = -_2;
_3 = !RET;
RET = 189_u8 as isize;
_6 = -_5;
_6 = _3;
Goto(bb1)
}
bb1 = {
_2 = _6 >> _3;
_4 = _2;
_3 = _4;
RET = 289606409506211941470859051469121001590_u128 as isize;
_2 = (-32582_i16) as isize;
_6 = _3 + _4;
_3 = !_6;
_2 = _3;
RET = 127_i8 as isize;
_5 = _6;
_2 = !_6;
RET = _3;
RET = !_5;
RET = _6 << _5;
_5 = (-114181638935657560373024007502098571750_i128) as isize;
_5 = _2;
Goto(bb2)
}
bb2 = {
Call(_9 = dump_var(12_usize, 4_usize, Move(_4), 3_usize, Move(_3), 10_usize, _10, 10_usize, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: *const [u32; 7],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize) -> u16 {
mir! {
type RET = u16;
let _8: *mut (u8, [u64; 3], u8, [char; 6]);
let _9: bool;
let _10: [i8; 6];
let _11: i128;
let _12: *mut [isize; 1];
let _13: f64;
let _14: u64;
let _15: f32;
let _16: &'static f64;
let _17: usize;
let _18: isize;
let _19: char;
let _20: isize;
let _21: *const [u32; 7];
let _22: u128;
let _23: Adt39;
let _24: u128;
let _25: i64;
let _26: f32;
let _27: [i8; 3];
let _28: (u64, [u32; 1], f32);
let _29: isize;
let _30: (u16, Adt45, *mut i32, (char, isize, u16, &'static i128));
let _31: u64;
let _32: isize;
let _33: char;
let _34: ();
let _35: ();
{
RET = 65947746637413498389789262997737512491_i128 as u16;
_1 = _6 ^ _5;
_5 = _6;
_1 = _2 >> _2;
_2 = _4 | _1;
RET = !10787_u16;
RET = 25521_u16 - 52040_u16;
_5 = -_2;
RET = (-1269178856366163619_i64) as u16;
Goto(bb1)
}
bb1 = {
_7 = _4 * _5;
_2 = -_7;
_6 = true as isize;
RET = 25661_u16;
_10 = [44_i8,(-113_i8),(-36_i8),(-31_i8),(-71_i8),(-50_i8)];
_9 = true ^ true;
_7 = -_5;
_9 = true;
_10 = [80_i8,113_i8,(-34_i8),38_i8,53_i8,126_i8];
_4 = -_5;
_11 = (-128145894544080611407333257092939103035_i128) | 103881825136927211468097702750799345059_i128;
_11 = 1892329536_u32 as i128;
_1 = _7;
Call(_12 = fn14(_7, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = 316357794240334044843419690103516293337_u128 as f64;
_10 = [(-45_i8),125_i8,(-16_i8),34_i8,27_i8,85_i8];
_2 = _1;
_2 = !_1;
_13 = 1363186625_i32 as f64;
_7 = _4;
_2 = '\u{3c56a}' as isize;
_2 = _4 & _5;
_2 = _7 * _5;
_14 = 9953013404343240616_u64 | 2650758718451239632_u64;
_11 = 93277955603066439742450173511067696298_i128;
_10 = [49_i8,(-82_i8),(-73_i8),(-119_i8),88_i8,(-110_i8)];
_17 = 4956815834035952354_usize & 12018030914955091320_usize;
_4 = !_5;
_2 = -_4;
_5 = !_4;
_2 = _5;
_16 = &_13;
_4 = _2;
_7 = _2 | _2;
_1 = -_5;
Goto(bb3)
}
bb3 = {
_1 = _5 - _5;
_7 = _1 << _4;
_17 = 0_usize;
_11 = (-158547684594684741045575925799719937259_i128);
_14 = 7632018737715949302_u64;
RET = 18058_u16 * 45941_u16;
_16 = &_13;
RET = 5336552692088162407_i64 as u16;
_16 = &(*_16);
_7 = _4 - _2;
_6 = !_5;
_4 = -_6;
_13 = (-2573_i16) as f64;
_19 = '\u{a5cf7}';
_5 = RET as isize;
_13 = (-487556518_i32) as f64;
_13 = _11 as f64;
_11 = _17 as i128;
_6 = !_4;
_10[_17] = (-85_i8);
_15 = _14 as f32;
_7 = _10[_17] as isize;
_14 = (-2731779341001095000_i64) as u64;
_20 = _17 as isize;
_15 = 221_u8 as f32;
Call(_18 = fn18(_6, _1, _4, _2, Move(_12), _6, _2, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_20 = _18;
_5 = 680192264497989261_i64 as isize;
_7 = _19 as isize;
_11 = 3972257193_u32 as i128;
_6 = -_18;
RET = !43647_u16;
_6 = !_20;
RET = (-768625657_i32) as u16;
_16 = &_13;
_13 = 204326125327981999545953504816435821694_u128 as f64;
_19 = '\u{70913}';
_24 = _14 as u128;
match _17 {
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
0 => bb11,
_ => bb10
}
}
bb5 = {
_1 = _5 - _5;
_7 = _1 << _4;
_17 = 0_usize;
_11 = (-158547684594684741045575925799719937259_i128);
_14 = 7632018737715949302_u64;
RET = 18058_u16 * 45941_u16;
_16 = &_13;
RET = 5336552692088162407_i64 as u16;
_16 = &(*_16);
_7 = _4 - _2;
_6 = !_5;
_4 = -_6;
_13 = (-2573_i16) as f64;
_19 = '\u{a5cf7}';
_5 = RET as isize;
_13 = (-487556518_i32) as f64;
_13 = _11 as f64;
_11 = _17 as i128;
_6 = !_4;
_10[_17] = (-85_i8);
_15 = _14 as f32;
_7 = _10[_17] as isize;
_14 = (-2731779341001095000_i64) as u64;
_20 = _17 as isize;
_15 = 221_u8 as f32;
Call(_18 = fn18(_6, _1, _4, _2, Move(_12), _6, _2, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_13 = 316357794240334044843419690103516293337_u128 as f64;
_10 = [(-45_i8),125_i8,(-16_i8),34_i8,27_i8,85_i8];
_2 = _1;
_2 = !_1;
_13 = 1363186625_i32 as f64;
_7 = _4;
_2 = '\u{3c56a}' as isize;
_2 = _4 & _5;
_2 = _7 * _5;
_14 = 9953013404343240616_u64 | 2650758718451239632_u64;
_11 = 93277955603066439742450173511067696298_i128;
_10 = [49_i8,(-82_i8),(-73_i8),(-119_i8),88_i8,(-110_i8)];
_17 = 4956815834035952354_usize & 12018030914955091320_usize;
_4 = !_5;
_2 = -_4;
_5 = !_4;
_2 = _5;
_16 = &_13;
_4 = _2;
_7 = _2 | _2;
_1 = -_5;
Goto(bb3)
}
bb7 = {
_7 = _4 * _5;
_2 = -_7;
_6 = true as isize;
RET = 25661_u16;
_10 = [44_i8,(-113_i8),(-36_i8),(-31_i8),(-71_i8),(-50_i8)];
_9 = true ^ true;
_7 = -_5;
_9 = true;
_10 = [80_i8,113_i8,(-34_i8),38_i8,53_i8,126_i8];
_4 = -_5;
_11 = (-128145894544080611407333257092939103035_i128) | 103881825136927211468097702750799345059_i128;
_11 = 1892329536_u32 as i128;
_1 = _7;
Call(_12 = fn14(_7, _2), ReturnTo(bb2), UnwindUnreachable())
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
_21 = Move(_3);
_9 = true;
_10 = [(-24_i8),38_i8,(-69_i8),(-31_i8),(-30_i8),47_i8];
RET = 26217_u16 & 29758_u16;
_17 = 5_usize & 4_usize;
_25 = -(-4025767363187180683_i64);
_5 = 1943918661_i32 as isize;
_18 = _6;
_13 = RET as f64;
RET = 3789_u16;
_10 = [7_i8,89_i8,(-34_i8),32_i8,(-51_i8),(-98_i8)];
_1 = _4;
_20 = _4;
_26 = _1 as f32;
_1 = _19 as isize;
_10 = [25_i8,40_i8,(-78_i8),(-105_i8),(-11_i8),(-3_i8)];
_15 = -_26;
_19 = '\u{44072}';
RET = !55696_u16;
_11 = 4037401125_u32 as i128;
_28.0 = !_14;
_28.0 = _13 as u64;
_13 = _6 as f64;
Goto(bb12)
}
bb12 = {
_3 = Move(_21);
_16 = &_13;
_1 = _15 as isize;
_15 = _26;
_28.2 = -_15;
_11 = (-54239644861894836836031306164972958024_i128);
_28.1 = [3218366708_u32];
_27 = [(-121_i8),93_i8,(-121_i8)];
_21 = Move(_3);
_30.3.1 = _18 * _6;
match _11 {
0 => bb13,
1 => bb14,
286042722059043626627343301266795253432 => bb16,
_ => bb15
}
}
bb13 = {
_21 = Move(_3);
_9 = true;
_10 = [(-24_i8),38_i8,(-69_i8),(-31_i8),(-30_i8),47_i8];
RET = 26217_u16 & 29758_u16;
_17 = 5_usize & 4_usize;
_25 = -(-4025767363187180683_i64);
_5 = 1943918661_i32 as isize;
_18 = _6;
_13 = RET as f64;
RET = 3789_u16;
_10 = [7_i8,89_i8,(-34_i8),32_i8,(-51_i8),(-98_i8)];
_1 = _4;
_20 = _4;
_26 = _1 as f32;
_1 = _19 as isize;
_10 = [25_i8,40_i8,(-78_i8),(-105_i8),(-11_i8),(-3_i8)];
_15 = -_26;
_19 = '\u{44072}';
RET = !55696_u16;
_11 = 4037401125_u32 as i128;
_28.0 = !_14;
_28.0 = _13 as u64;
_13 = _6 as f64;
Goto(bb12)
}
bb14 = {
_7 = _4 * _5;
_2 = -_7;
_6 = true as isize;
RET = 25661_u16;
_10 = [44_i8,(-113_i8),(-36_i8),(-31_i8),(-71_i8),(-50_i8)];
_9 = true ^ true;
_7 = -_5;
_9 = true;
_10 = [80_i8,113_i8,(-34_i8),38_i8,53_i8,126_i8];
_4 = -_5;
_11 = (-128145894544080611407333257092939103035_i128) | 103881825136927211468097702750799345059_i128;
_11 = 1892329536_u32 as i128;
_1 = _7;
Call(_12 = fn14(_7, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_6 = 1508493897_i32 as isize;
_28.1 = [852532665_u32];
_17 = 5_usize ^ 7474247180479672486_usize;
_3 = Move(_21);
_5 = _4;
_9 = true;
_19 = '\u{1f48b}';
_9 = !true;
_29 = _20;
_7 = _20 * _20;
_30.3.1 = _14 as isize;
_30.0 = !RET;
_28.1 = [3991478922_u32];
_19 = '\u{a7622}';
RET = _30.0;
_30.3.2 = _11 as u16;
_2 = _4 ^ _18;
RET = _30.3.2 * _30.3.2;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(13_usize, 4_usize, Move(_4), 2_usize, Move(_2), 9_usize, Move(_9), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(13_usize, 1_usize, Move(_1), 11_usize, Move(_11), 7_usize, Move(_7), 20_usize, Move(_20)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(13_usize, 6_usize, Move(_6), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: isize) -> *mut [isize; 1] {
mir! {
type RET = *mut [isize; 1];
let _3: f32;
let _4: i128;
let _5: u128;
let _6: (u8, [u64; 3], u8, [char; 6]);
let _7: i8;
let _8: i16;
let _9: *const i32;
let _10: ([u64; 6], char, [u32; 1], f64);
let _11: (f32, (&'static u32, &'static i16, (&'static i16,)));
let _12: (*mut i32, [u32; 7]);
let _13: isize;
let _14: [u32; 7];
let _15: isize;
let _16: isize;
let _17: u128;
let _18: &'static *const isize;
let _19: f32;
let _20: bool;
let _21: [usize; 2];
let _22: Adt40;
let _23: [isize; 2];
let _24: [isize; 6];
let _25: f64;
let _26: char;
let _27: i8;
let _28: &'static u128;
let _29: isize;
let _30: isize;
let _31: &'static [i8; 3];
let _32: i64;
let _33: isize;
let _34: *const (char, isize, u16, &'static i128);
let _35: u16;
let _36: u64;
let _37: [isize; 6];
let _38: u64;
let _39: *const i32;
let _40: u64;
let _41: isize;
let _42: (*mut i32, [u32; 7]);
let _43: &'static *const isize;
let _44: &'static i16;
let _45: [isize; 6];
let _46: *mut [isize; 1];
let _47: char;
let _48: usize;
let _49: u16;
let _50: [isize; 1];
let _51: *const u128;
let _52: bool;
let _53: (u16, Adt45, *mut i32, (char, isize, u16, &'static i128));
let _54: u64;
let _55: f64;
let _56: [usize; 1];
let _57: f32;
let _58: [isize; 1];
let _59: [isize; 2];
let _60: (char, isize, u16, &'static i128);
let _61: [u128; 4];
let _62: bool;
let _63: u128;
let _64: u8;
let _65: u32;
let _66: i64;
let _67: isize;
let _68: f32;
let _69: char;
let _70: [i8; 3];
let _71: [u64; 6];
let _72: ();
let _73: ();
{
_1 = !_2;
_1 = 165_u8 as isize;
_2 = _1;
_2 = _1 & _1;
_2 = _1 & _1;
_1 = _2;
Goto(bb1)
}
bb1 = {
_2 = _1;
_1 = true as isize;
_1 = _2;
_1 = _2 + _2;
_2 = 3708169898941786755_i64 as isize;
_2 = -_1;
_1 = _2;
_1 = _2 >> _2;
_1 = 7730014379135372640_usize as isize;
_1 = _2 >> _2;
_1 = 5382093040655625838_u64 as isize;
Goto(bb2)
}
bb2 = {
_3 = 152_u8 as f32;
_2 = _1;
_2 = (-76_i8) as isize;
_4 = 294569534996998354764695830602323072_i128 >> _2;
_2 = !_1;
_1 = -_2;
_3 = 35_u8 as f32;
_2 = _1 ^ _1;
_1 = _2;
_4 = (-133989885357313110920376691339623034397_i128) - 35733564976626589992833609611230768473_i128;
_3 = (-27009_i16) as f32;
_2 = !_1;
_3 = 31567_u16 as f32;
_5 = 244657297440537024515117720531546657587_u128;
_3 = (-1325635511541950556_i64) as f32;
_6.2 = !168_u8;
Goto(bb3)
}
bb3 = {
_6.3 = ['\u{ad5e9}','\u{d4541}','\u{7678e}','\u{bcc1}','\u{57465}','\u{e7812}'];
_1 = (-17_i8) as isize;
_7 = (-10_i8) * (-109_i8);
_6.0 = false as u8;
_3 = _4 as f32;
_6.0 = _6.2;
_4 = !77993913343586991474943693062752973277_i128;
_1 = _2 | _2;
_6.0 = _6.2;
_5 = 85174561406446411373092452606285063454_u128 * 253970878793557805802318208213754127251_u128;
_8 = (-6490_i16) + (-1361_i16);
_6.1 = [7833221903193450294_u64,1467864599703550805_u64,8726331644333548219_u64];
_4 = 112669505737276674756661451054792932678_i128;
_3 = 3735054572855466732_i64 as f32;
_8 = (-5236_i16) & (-29813_i16);
_3 = 1946004534_i32 as f32;
_7 = 96_i8 << _2;
_6.1 = [16231167860844979496_u64,7156580675718724268_u64,5123799632710634298_u64];
_3 = 34795_u16 as f32;
_1 = 854330598_u32 as isize;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
112669505737276674756661451054792932678 => bb9,
_ => bb8
}
}
bb4 = {
_3 = 152_u8 as f32;
_2 = _1;
_2 = (-76_i8) as isize;
_4 = 294569534996998354764695830602323072_i128 >> _2;
_2 = !_1;
_1 = -_2;
_3 = 35_u8 as f32;
_2 = _1 ^ _1;
_1 = _2;
_4 = (-133989885357313110920376691339623034397_i128) - 35733564976626589992833609611230768473_i128;
_3 = (-27009_i16) as f32;
_2 = !_1;
_3 = 31567_u16 as f32;
_5 = 244657297440537024515117720531546657587_u128;
_3 = (-1325635511541950556_i64) as f32;
_6.2 = !168_u8;
Goto(bb3)
}
bb5 = {
_2 = _1;
_1 = true as isize;
_1 = _2;
_1 = _2 + _2;
_2 = 3708169898941786755_i64 as isize;
_2 = -_1;
_1 = _2;
_1 = _2 >> _2;
_1 = 7730014379135372640_usize as isize;
_1 = _2 >> _2;
_1 = 5382093040655625838_u64 as isize;
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
_3 = _1 as f32;
_6.3 = ['\u{a631a}','\u{ffc77}','\u{b54b9}','\u{2913b}','\u{ff704}','\u{2da86}'];
_6.3 = ['\u{ee270}','\u{efba9}','\u{361d}','\u{bbb33}','\u{ce87d}','\u{57d48}'];
_6.0 = !_6.2;
_4 = 15325115154266449693_usize as i128;
_6.2 = 14251101479063354886_u64 as u8;
_4 = 117855560903350492661425205468758635661_i128 | 96658013751787726775780004191799621785_i128;
_8 = (-516_i16);
_7 = 21_i8 - 101_i8;
_3 = _8 as f32;
_2 = _1;
_4 = !(-136902227550325184108095151008311428214_i128);
_7 = 3699054525_u32 as i8;
_8 = 28275_i16 & 5457_i16;
_2 = -_1;
_6.1 = [8936468795104852437_u64,8506782270738074439_u64,8933795680158237852_u64];
_6.1 = [9460460612316576965_u64,9546262461674706597_u64,5423236474103048606_u64];
_4 = (-77034582646439279540972305746722642505_i128);
_8 = !(-11637_i16);
_7 = !(-116_i8);
_7 = (-11_i8) << _8;
_11.1.1 = &_8;
_6.0 = !_6.2;
_11.0 = _3;
_10.0 = [17829361536671221853_u64,14888545750998673755_u64,11415240375774500304_u64,13988566690685152003_u64,14701376243216649687_u64,8103071103372934610_u64];
_4 = -120344878053410870001695263736977588318_i128;
Goto(bb10)
}
bb10 = {
_13 = _2;
_8 = 7119_i16 * (-14060_i16);
_11.1.2.0 = &_8;
_12.1 = [2541008234_u32,1969005024_u32,182534459_u32,2604687332_u32,73066666_u32,2805038479_u32,1758054067_u32];
_6.0 = _6.2;
_10.3 = 57382_u16 as f64;
_6.2 = _6.0;
_6.3 = ['\u{74e71}','\u{1a096}','\u{1423d}','\u{bcdc3}','\u{8564b}','\u{fdae8}'];
_1 = (-519324165_i32) as isize;
_12.1 = [1910567507_u32,2311180198_u32,3898238845_u32,2308813360_u32,3780500019_u32,2210896965_u32,952754824_u32];
_10.3 = _3 as f64;
_10.1 = '\u{2525}';
_1 = _13;
_10.3 = _4 as f64;
_12.1 = [1088583077_u32,1555887019_u32,3492210953_u32,4231306249_u32,3917420273_u32,777396136_u32,3634834462_u32];
_10.3 = _6.0 as f64;
_1 = _2 * _13;
_10.0 = [1516364242799694546_u64,17608891538177696502_u64,10287170574749946082_u64,1093784957195007780_u64,3789961182774841840_u64,15973878500857766987_u64];
_14 = [382387903_u32,3038021270_u32,3920183451_u32,1996800278_u32,628097514_u32,1705680830_u32,3943333632_u32];
Goto(bb11)
}
bb11 = {
_11.1.2.0 = &_8;
_12.1 = [2426592144_u32,949111380_u32,2604473911_u32,2254586226_u32,1294915543_u32,3055553885_u32,3804356542_u32];
_16 = -_2;
_10.2 = [3844745759_u32];
_16 = _1 + _1;
_6.2 = _6.0 & _6.0;
_15 = !_1;
_17 = 37364_u16 as u128;
_12.1 = _14;
_8 = 3147_i16 + 17677_i16;
_8 = !(-22505_i16);
_10.3 = _6.2 as f64;
Goto(bb12)
}
bb12 = {
_10.3 = _4 as f64;
_6.2 = _6.0 + _6.0;
_6.1 = [4246474390451871893_u64,12185831807520180651_u64,3186443871401426211_u64];
_13 = _16;
_10.3 = _7 as f64;
_7 = (-117_i8) | 32_i8;
_4 = -(-20538355694946413229021355706813751970_i128);
_10.2 = [3616079415_u32];
_10.2 = [668990735_u32];
_6.2 = _6.0 | _6.0;
_11.0 = _3 - _3;
_13 = _1 << _16;
_6.1 = [10499199526979549533_u64,12997159896112223821_u64,663380173976152573_u64];
_19 = _11.0;
_12.1 = [3383472625_u32,1603936978_u32,3112484089_u32,4137855144_u32,2871276931_u32,1171919037_u32,3080015889_u32];
_14 = _12.1;
_6.1 = [4931766716243967138_u64,7105234990505980330_u64,8328528052148471310_u64];
_10.3 = _7 as f64;
_14 = [1276743148_u32,505477450_u32,1876453408_u32,4202853167_u32,962334813_u32,2943392996_u32,1577913463_u32];
_2 = _13 | _15;
_6.2 = false as u8;
_20 = !false;
_6.2 = _6.0;
_17 = _5;
_15 = _13 & _2;
_10.0 = [1411228127985579046_u64,17745267438203241889_u64,273391655542293499_u64,1922111080261514737_u64,14470297747076765305_u64,13279926460475986054_u64];
_11.1.1 = &_8;
Goto(bb13)
}
bb13 = {
_16 = 7_usize as isize;
_14 = [598982415_u32,406731251_u32,2484473869_u32,3595017589_u32,764995670_u32,3372711191_u32,1715173241_u32];
_6.2 = !_6.0;
_14 = _12.1;
_21 = [9305529541008640048_usize,10295488020056590104_usize];
_11.0 = _19 + _19;
_11.0 = _3;
_6.1 = [5207477835132054864_u64,15643883779391338866_u64,11600231630219843841_u64];
_7 = (-406792595_i32) as i8;
_17 = _5;
_10.3 = 3793941634811610837_i64 as f64;
_10.2 = [2874567062_u32];
_8 = !5347_i16;
_10.3 = 28901_u16 as f64;
_11.0 = _19;
_12.1 = [1546755189_u32,321983807_u32,1794085649_u32,452762132_u32,2408133044_u32,3784140354_u32,2869795648_u32];
_10.0 = [16007714258718679295_u64,14347979876569039671_u64,9051862629388505842_u64,14698661522012850842_u64,12388827252955351268_u64,15782920081426426713_u64];
_6.0 = _6.2;
_11.1.1 = &_8;
_11.1.1 = &_8;
_12.1 = _14;
_10.2 = [4087404051_u32];
_11.1.1 = &_8;
_11.1.1 = &_8;
_11.1.2.0 = &_8;
_11.1.1 = Move(_11.1.2.0);
Goto(bb14)
}
bb14 = {
_10.2 = [1055596808_u32];
_12.1 = _14;
Goto(bb15)
}
bb15 = {
_20 = true & false;
_10.1 = '\u{b6e35}';
_11.1.2.0 = &_8;
_10.1 = '\u{b1c10}';
_6.3 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_2 = _13 - _15;
_6.3 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_1 = !_15;
_10.0 = [16184943985655881602_u64,11069580281985177098_u64,5099007953675909564_u64,13248324077599041919_u64,10986482488021682588_u64,2171746883014507845_u64];
_4 = _20 as i128;
_10.2 = [2738129889_u32];
_1 = !_2;
_7 = -(-101_i8);
_4 = (-104407473379511507565634547270743919518_i128) >> _1;
_5 = _17;
_11.1.1 = &_8;
_23 = [_2,_2];
_27 = 1944743029_u32 as i8;
_12.1 = [2183492643_u32,1197297576_u32,899353825_u32,775746480_u32,2426704060_u32,3570288788_u32,2797640924_u32];
_29 = _1 & _13;
_11.1.2 = (Move(_11.1.1),);
_6.3 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_21 = [4924266212998827272_usize,4_usize];
_10.3 = 5749820917774583545_i64 as f64;
_30 = !_1;
_16 = _1;
Call(_16 = fn15(), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_23 = [_16,_15];
_15 = _2 & _30;
_8 = _27 as i16;
_10.2 = [4294492384_u32];
_7 = _27;
_10.0 = [8549425227260385695_u64,16954416616842590932_u64,16985726861565844612_u64,13020272851162913771_u64,480292036229849539_u64,2283808024995165642_u64];
_17 = !_5;
_30 = !_29;
_8 = (-15539_i16);
_25 = _10.3;
_6.2 = !_6.0;
_28 = &_5;
_3 = -_19;
_10.1 = '\u{c9a36}';
_33 = _25 as isize;
_11.1.2.0 = &_8;
_12.1 = [3547734573_u32,1425333455_u32,2112206910_u32,239259832_u32,2579308159_u32,1880985064_u32,2992080018_u32];
_10.2 = [3534667377_u32];
_10.3 = _16 as f64;
_20 = _30 <= _2;
_6.3 = [_10.1,_10.1,_10.1,_10.1,_10.1,_10.1];
_6.1 = [2625656305283656064_u64,11144999489497858327_u64,12028467562190006262_u64];
match _8 {
0 => bb1,
1 => bb15,
2 => bb3,
3 => bb13,
4 => bb17,
5 => bb18,
340282366920938463463374607431768195917 => bb20,
_ => bb19
}
}
bb17 = {
_3 = _1 as f32;
_6.3 = ['\u{a631a}','\u{ffc77}','\u{b54b9}','\u{2913b}','\u{ff704}','\u{2da86}'];
_6.3 = ['\u{ee270}','\u{efba9}','\u{361d}','\u{bbb33}','\u{ce87d}','\u{57d48}'];
_6.0 = !_6.2;
_4 = 15325115154266449693_usize as i128;
_6.2 = 14251101479063354886_u64 as u8;
_4 = 117855560903350492661425205468758635661_i128 | 96658013751787726775780004191799621785_i128;
_8 = (-516_i16);
_7 = 21_i8 - 101_i8;
_3 = _8 as f32;
_2 = _1;
_4 = !(-136902227550325184108095151008311428214_i128);
_7 = 3699054525_u32 as i8;
_8 = 28275_i16 & 5457_i16;
_2 = -_1;
_6.1 = [8936468795104852437_u64,8506782270738074439_u64,8933795680158237852_u64];
_6.1 = [9460460612316576965_u64,9546262461674706597_u64,5423236474103048606_u64];
_4 = (-77034582646439279540972305746722642505_i128);
_8 = !(-11637_i16);
_7 = !(-116_i8);
_7 = (-11_i8) << _8;
_11.1.1 = &_8;
_6.0 = !_6.2;
_11.0 = _3;
_10.0 = [17829361536671221853_u64,14888545750998673755_u64,11415240375774500304_u64,13988566690685152003_u64,14701376243216649687_u64,8103071103372934610_u64];
_4 = -120344878053410870001695263736977588318_i128;
Goto(bb10)
}
bb18 = {
Return()
}
bb19 = {
_10.3 = _4 as f64;
_6.2 = _6.0 + _6.0;
_6.1 = [4246474390451871893_u64,12185831807520180651_u64,3186443871401426211_u64];
_13 = _16;
_10.3 = _7 as f64;
_7 = (-117_i8) | 32_i8;
_4 = -(-20538355694946413229021355706813751970_i128);
_10.2 = [3616079415_u32];
_10.2 = [668990735_u32];
_6.2 = _6.0 | _6.0;
_11.0 = _3 - _3;
_13 = _1 << _16;
_6.1 = [10499199526979549533_u64,12997159896112223821_u64,663380173976152573_u64];
_19 = _11.0;
_12.1 = [3383472625_u32,1603936978_u32,3112484089_u32,4137855144_u32,2871276931_u32,1171919037_u32,3080015889_u32];
_14 = _12.1;
_6.1 = [4931766716243967138_u64,7105234990505980330_u64,8328528052148471310_u64];
_10.3 = _7 as f64;
_14 = [1276743148_u32,505477450_u32,1876453408_u32,4202853167_u32,962334813_u32,2943392996_u32,1577913463_u32];
_2 = _13 | _15;
_6.2 = false as u8;
_20 = !false;
_6.2 = _6.0;
_17 = _5;
_15 = _13 & _2;
_10.0 = [1411228127985579046_u64,17745267438203241889_u64,273391655542293499_u64,1922111080261514737_u64,14470297747076765305_u64,13279926460475986054_u64];
_11.1.1 = &_8;
Goto(bb13)
}
bb20 = {
_14 = [729977530_u32,1007191724_u32,3725084403_u32,464268872_u32,2539389035_u32,207020731_u32,2461269974_u32];
_17 = (*_28);
_11.1.2.0 = &_8;
_11.1.1 = &_8;
_36 = _19 as u64;
_21 = [3_usize,13029506895608700751_usize];
_35 = _10.1 as u16;
_23 = [_1,_16];
_13 = _10.3 as isize;
_6.2 = _6.0 << _13;
_13 = _20 as isize;
_1 = -_29;
_11.1.1 = &_8;
_21 = [0_usize,7313460452345066191_usize];
_26 = _10.1;
_12.1 = [3377904768_u32,3153811082_u32,555383976_u32,4164313075_u32,115110598_u32,3948793094_u32,3757134316_u32];
_35 = !21012_u16;
_6.1 = [_36,_36,_36];
_20 = false;
_38 = !_36;
Goto(bb21)
}
bb21 = {
_6.0 = 1700769865_i32 as u8;
_10.0 = [_38,_36,_38,_38,_38,_38];
_6.3 = [_26,_10.1,_10.1,_10.1,_10.1,_26];
_6.0 = _6.2 + _6.2;
_17 = (-8167216854918879132_i64) as u128;
_23 = [_29,_2];
_40 = _8 as u64;
_10.3 = -_25;
_10.3 = _25 + _25;
_16 = _13 << _6.2;
_11.1.2.0 = &_8;
_24 = [_13,_16,_16,_16,_30,_30];
_6.2 = !_6.0;
_19 = _11.0 - _3;
_1 = _13 >> _6.2;
_37 = _24;
_20 = false;
_32 = (-2590839538170783926_i64);
_11.1.1 = &_8;
_11.1.2.0 = &_8;
_11.1.1 = &_8;
_15 = (-967433382_i32) as isize;
_6.3 = [_10.1,_10.1,_10.1,_26,_26,_26];
_11.1.2.0 = &_8;
_28 = &(*_28);
_20 = !false;
_6.0 = _6.2 + _6.2;
match _32 {
0 => bb12,
1 => bb19,
2 => bb14,
3 => bb13,
4 => bb22,
340282366920938463460783767893597427530 => bb24,
_ => bb23
}
}
bb22 = {
_14 = [729977530_u32,1007191724_u32,3725084403_u32,464268872_u32,2539389035_u32,207020731_u32,2461269974_u32];
_17 = (*_28);
_11.1.2.0 = &_8;
_11.1.1 = &_8;
_36 = _19 as u64;
_21 = [3_usize,13029506895608700751_usize];
_35 = _10.1 as u16;
_23 = [_1,_16];
_13 = _10.3 as isize;
_6.2 = _6.0 << _13;
_13 = _20 as isize;
_1 = -_29;
_11.1.1 = &_8;
_21 = [0_usize,7313460452345066191_usize];
_26 = _10.1;
_12.1 = [3377904768_u32,3153811082_u32,555383976_u32,4164313075_u32,115110598_u32,3948793094_u32,3757134316_u32];
_35 = !21012_u16;
_6.1 = [_36,_36,_36];
_20 = false;
_38 = !_36;
Goto(bb21)
}
bb23 = {
_10.2 = [1055596808_u32];
_12.1 = _14;
Goto(bb15)
}
bb24 = {
_29 = _16 & _13;
_41 = 1428677040_i32 as isize;
_30 = (-843633877_i32) as isize;
_21 = [9813464716463047496_usize,3_usize];
_19 = -_11.0;
_26 = _10.1;
_41 = 2_usize as isize;
_10.2 = [140531965_u32];
_23 = [_13,_1];
Goto(bb25)
}
bb25 = {
_7 = _27;
_3 = _19 + _19;
_15 = !_16;
_19 = _6.0 as f32;
_6.0 = _6.2;
_29 = _1 << _1;
_36 = !_40;
_17 = (*_28);
_45 = [_16,_29,_13,_16,_29,_29];
_10.3 = _25;
_38 = !_36;
_37 = [_16,_1,_29,_2,_29,_16];
_6.1 = [_38,_40,_36];
_6.3 = [_10.1,_10.1,_10.1,_26,_26,_26];
Goto(bb26)
}
bb26 = {
_11.0 = -_19;
Goto(bb27)
}
bb27 = {
_47 = _26;
_11.0 = -_19;
_2 = !_16;
_8 = 16446524420134066814_usize as i16;
_4 = 24254544491535773975890348806164761389_i128;
_11.0 = _19 * _19;
_14 = _12.1;
_48 = 2_usize >> _16;
_35 = !30677_u16;
_21 = [_48,_48];
_49 = !_35;
_11.1.2.0 = &_8;
match _32 {
340282366920938463460783767893597427530 => bb28,
_ => bb11
}
}
bb28 = {
_37 = _45;
_32 = 2789084661117754086_i64 >> _29;
_53.3.2 = _8 as u16;
_53.3.1 = !_15;
_30 = !_13;
_32 = 3653637011911625881_i64 << _2;
_6.2 = _6.0;
_53.3.3 = &_4;
_44 = Move(_11.1.2.0);
_11.1.1 = &_8;
_32 = _53.3.2 as i64;
_24 = _45;
_26 = _10.1;
_35 = _49;
_55 = _10.3;
_46 = core::ptr::addr_of_mut!(_50);
_2 = _7 as isize;
_13 = _16 | _16;
_53.3.3 = &_4;
_8 = -8435_i16;
_10.3 = -_25;
match _4 {
0 => bb29,
1 => bb30,
24254544491535773975890348806164761389 => bb32,
_ => bb31
}
}
bb29 = {
_2 = _1;
_1 = true as isize;
_1 = _2;
_1 = _2 + _2;
_2 = 3708169898941786755_i64 as isize;
_2 = -_1;
_1 = _2;
_1 = _2 >> _2;
_1 = 7730014379135372640_usize as isize;
_1 = _2 >> _2;
_1 = 5382093040655625838_u64 as isize;
Goto(bb2)
}
bb30 = {
_10.2 = [1055596808_u32];
_12.1 = _14;
Goto(bb15)
}
bb31 = {
_6.3 = ['\u{ad5e9}','\u{d4541}','\u{7678e}','\u{bcc1}','\u{57465}','\u{e7812}'];
_1 = (-17_i8) as isize;
_7 = (-10_i8) * (-109_i8);
_6.0 = false as u8;
_3 = _4 as f32;
_6.0 = _6.2;
_4 = !77993913343586991474943693062752973277_i128;
_1 = _2 | _2;
_6.0 = _6.2;
_5 = 85174561406446411373092452606285063454_u128 * 253970878793557805802318208213754127251_u128;
_8 = (-6490_i16) + (-1361_i16);
_6.1 = [7833221903193450294_u64,1467864599703550805_u64,8726331644333548219_u64];
_4 = 112669505737276674756661451054792932678_i128;
_3 = 3735054572855466732_i64 as f32;
_8 = (-5236_i16) & (-29813_i16);
_3 = 1946004534_i32 as f32;
_7 = 96_i8 << _2;
_6.1 = [16231167860844979496_u64,7156580675718724268_u64,5123799632710634298_u64];
_3 = 34795_u16 as f32;
_1 = 854330598_u32 as isize;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
112669505737276674756661451054792932678 => bb9,
_ => bb8
}
}
bb32 = {
_53.3.3 = &_4;
Goto(bb33)
}
bb33 = {
_35 = _49 - _49;
_3 = -_19;
Goto(bb34)
}
bb34 = {
_1 = (-1133857022_i32) as isize;
_50 = [_15];
_2 = !_29;
RET = core::ptr::addr_of_mut!(_50);
_46 = Move(RET);
_14 = [1222344456_u32,668999551_u32,1552618362_u32,4160478677_u32,3762585453_u32,1150676590_u32,979110942_u32];
_53.3.3 = &_4;
_57 = _3;
_53.3.0 = _10.1;
_14 = [713586325_u32,964012371_u32,240073470_u32,1439612176_u32,3326970730_u32,1858473128_u32,1537128894_u32];
_34 = core::ptr::addr_of!(_53.3);
_33 = -_16;
_51 = core::ptr::addr_of!((*_28));
(*_34).3 = &_4;
_10.0 = [_40,_38,_40,_40,_40,_38];
_11.0 = 2117874042_i32 as f32;
_16 = !_30;
_35 = _48 as u16;
Goto(bb35)
}
bb35 = {
(*_34).3 = &_4;
_17 = !(*_51);
_1 = _13;
_36 = _38;
_11.1.1 = &_8;
_45 = [_1,_15,_2,_33,_16,_33];
_40 = !_36;
_44 = Move(_11.1.1);
_11.1.1 = &_8;
RET = Move(_46);
_28 = &_17;
_11.0 = _3 - _57;
_17 = (*_51);
_46 = Move(RET);
_60.3 = &_4;
(*_34).0 = _26;
_2 = -_13;
_55 = _25 * _10.3;
_15 = _29;
(*_34).2 = _35 ^ _35;
match _4 {
0 => bb3,
1 => bb36,
2 => bb37,
24254544491535773975890348806164761389 => bb39,
_ => bb38
}
}
bb36 = {
_7 = _27;
_3 = _19 + _19;
_15 = !_16;
_19 = _6.0 as f32;
_6.0 = _6.2;
_29 = _1 << _1;
_36 = !_40;
_17 = (*_28);
_45 = [_16,_29,_13,_16,_29,_29];
_10.3 = _25;
_38 = !_36;
_37 = [_16,_1,_29,_2,_29,_16];
_6.1 = [_38,_40,_36];
_6.3 = [_10.1,_10.1,_10.1,_26,_26,_26];
Goto(bb26)
}
bb37 = {
_16 = 7_usize as isize;
_14 = [598982415_u32,406731251_u32,2484473869_u32,3595017589_u32,764995670_u32,3372711191_u32,1715173241_u32];
_6.2 = !_6.0;
_14 = _12.1;
_21 = [9305529541008640048_usize,10295488020056590104_usize];
_11.0 = _19 + _19;
_11.0 = _3;
_6.1 = [5207477835132054864_u64,15643883779391338866_u64,11600231630219843841_u64];
_7 = (-406792595_i32) as i8;
_17 = _5;
_10.3 = 3793941634811610837_i64 as f64;
_10.2 = [2874567062_u32];
_8 = !5347_i16;
_10.3 = 28901_u16 as f64;
_11.0 = _19;
_12.1 = [1546755189_u32,321983807_u32,1794085649_u32,452762132_u32,2408133044_u32,3784140354_u32,2869795648_u32];
_10.0 = [16007714258718679295_u64,14347979876569039671_u64,9051862629388505842_u64,14698661522012850842_u64,12388827252955351268_u64,15782920081426426713_u64];
_6.0 = _6.2;
_11.1.1 = &_8;
_11.1.1 = &_8;
_12.1 = _14;
_10.2 = [4087404051_u32];
_11.1.1 = &_8;
_11.1.1 = &_8;
_11.1.2.0 = &_8;
_11.1.1 = Move(_11.1.2.0);
Goto(bb14)
}
bb38 = {
_3 = 152_u8 as f32;
_2 = _1;
_2 = (-76_i8) as isize;
_4 = 294569534996998354764695830602323072_i128 >> _2;
_2 = !_1;
_1 = -_2;
_3 = 35_u8 as f32;
_2 = _1 ^ _1;
_1 = _2;
_4 = (-133989885357313110920376691339623034397_i128) - 35733564976626589992833609611230768473_i128;
_3 = (-27009_i16) as f32;
_2 = !_1;
_3 = 31567_u16 as f32;
_5 = 244657297440537024515117720531546657587_u128;
_3 = (-1325635511541950556_i64) as f32;
_6.2 = !168_u8;
Goto(bb3)
}
bb39 = {
_1 = _15;
_53.3 = (_26, _15, _35, Move(_60.3));
_29 = _33;
_10.3 = _25;
_27 = -_7;
_11.1.2.0 = &_8;
_64 = _6.0;
_53.3.0 = _47;
_30 = -_1;
_29 = -(*_34).1;
match _4 {
0 => bb30,
1 => bb20,
2 => bb31,
3 => bb40,
4 => bb41,
5 => bb42,
6 => bb43,
24254544491535773975890348806164761389 => bb45,
_ => bb44
}
}
bb40 = {
_37 = _45;
_32 = 2789084661117754086_i64 >> _29;
_53.3.2 = _8 as u16;
_53.3.1 = !_15;
_30 = !_13;
_32 = 3653637011911625881_i64 << _2;
_6.2 = _6.0;
_53.3.3 = &_4;
_44 = Move(_11.1.2.0);
_11.1.1 = &_8;
_32 = _53.3.2 as i64;
_24 = _45;
_26 = _10.1;
_35 = _49;
_55 = _10.3;
_46 = core::ptr::addr_of_mut!(_50);
_2 = _7 as isize;
_13 = _16 | _16;
_53.3.3 = &_4;
_8 = -8435_i16;
_10.3 = -_25;
match _4 {
0 => bb29,
1 => bb30,
24254544491535773975890348806164761389 => bb32,
_ => bb31
}
}
bb41 = {
_11.1.2.0 = &_8;
_12.1 = [2426592144_u32,949111380_u32,2604473911_u32,2254586226_u32,1294915543_u32,3055553885_u32,3804356542_u32];
_16 = -_2;
_10.2 = [3844745759_u32];
_16 = _1 + _1;
_6.2 = _6.0 & _6.0;
_15 = !_1;
_17 = 37364_u16 as u128;
_12.1 = _14;
_8 = 3147_i16 + 17677_i16;
_8 = !(-22505_i16);
_10.3 = _6.2 as f64;
Goto(bb12)
}
bb42 = {
_47 = _26;
_11.0 = -_19;
_2 = !_16;
_8 = 16446524420134066814_usize as i16;
_4 = 24254544491535773975890348806164761389_i128;
_11.0 = _19 * _19;
_14 = _12.1;
_48 = 2_usize >> _16;
_35 = !30677_u16;
_21 = [_48,_48];
_49 = !_35;
_11.1.2.0 = &_8;
match _32 {
340282366920938463460783767893597427530 => bb28,
_ => bb11
}
}
bb43 = {
_10.3 = _4 as f64;
_6.2 = _6.0 + _6.0;
_6.1 = [4246474390451871893_u64,12185831807520180651_u64,3186443871401426211_u64];
_13 = _16;
_10.3 = _7 as f64;
_7 = (-117_i8) | 32_i8;
_4 = -(-20538355694946413229021355706813751970_i128);
_10.2 = [3616079415_u32];
_10.2 = [668990735_u32];
_6.2 = _6.0 | _6.0;
_11.0 = _3 - _3;
_13 = _1 << _16;
_6.1 = [10499199526979549533_u64,12997159896112223821_u64,663380173976152573_u64];
_19 = _11.0;
_12.1 = [3383472625_u32,1603936978_u32,3112484089_u32,4137855144_u32,2871276931_u32,1171919037_u32,3080015889_u32];
_14 = _12.1;
_6.1 = [4931766716243967138_u64,7105234990505980330_u64,8328528052148471310_u64];
_10.3 = _7 as f64;
_14 = [1276743148_u32,505477450_u32,1876453408_u32,4202853167_u32,962334813_u32,2943392996_u32,1577913463_u32];
_2 = _13 | _15;
_6.2 = false as u8;
_20 = !false;
_6.2 = _6.0;
_17 = _5;
_15 = _13 & _2;
_10.0 = [1411228127985579046_u64,17745267438203241889_u64,273391655542293499_u64,1922111080261514737_u64,14470297747076765305_u64,13279926460475986054_u64];
_11.1.1 = &_8;
Goto(bb13)
}
bb44 = {
_3 = _1 as f32;
_6.3 = ['\u{a631a}','\u{ffc77}','\u{b54b9}','\u{2913b}','\u{ff704}','\u{2da86}'];
_6.3 = ['\u{ee270}','\u{efba9}','\u{361d}','\u{bbb33}','\u{ce87d}','\u{57d48}'];
_6.0 = !_6.2;
_4 = 15325115154266449693_usize as i128;
_6.2 = 14251101479063354886_u64 as u8;
_4 = 117855560903350492661425205468758635661_i128 | 96658013751787726775780004191799621785_i128;
_8 = (-516_i16);
_7 = 21_i8 - 101_i8;
_3 = _8 as f32;
_2 = _1;
_4 = !(-136902227550325184108095151008311428214_i128);
_7 = 3699054525_u32 as i8;
_8 = 28275_i16 & 5457_i16;
_2 = -_1;
_6.1 = [8936468795104852437_u64,8506782270738074439_u64,8933795680158237852_u64];
_6.1 = [9460460612316576965_u64,9546262461674706597_u64,5423236474103048606_u64];
_4 = (-77034582646439279540972305746722642505_i128);
_8 = !(-11637_i16);
_7 = !(-116_i8);
_7 = (-11_i8) << _8;
_11.1.1 = &_8;
_6.0 = !_6.2;
_11.0 = _3;
_10.0 = [17829361536671221853_u64,14888545750998673755_u64,11415240375774500304_u64,13988566690685152003_u64,14701376243216649687_u64,8103071103372934610_u64];
_4 = -120344878053410870001695263736977588318_i128;
Goto(bb10)
}
bb45 = {
_6.3 = [(*_34).0,_47,_47,(*_34).0,(*_34).0,_53.3.0];
_53.0 = _35 >> _53.3.2;
_56 = [_48];
_23 = [_33,(*_34).1];
_2 = -_1;
_65 = 405992902_u32;
match _4 {
0 => bb26,
1 => bb46,
2 => bb47,
24254544491535773975890348806164761389 => bb49,
_ => bb48
}
}
bb46 = {
_14 = [729977530_u32,1007191724_u32,3725084403_u32,464268872_u32,2539389035_u32,207020731_u32,2461269974_u32];
_17 = (*_28);
_11.1.2.0 = &_8;
_11.1.1 = &_8;
_36 = _19 as u64;
_21 = [3_usize,13029506895608700751_usize];
_35 = _10.1 as u16;
_23 = [_1,_16];
_13 = _10.3 as isize;
_6.2 = _6.0 << _13;
_13 = _20 as isize;
_1 = -_29;
_11.1.1 = &_8;
_21 = [0_usize,7313460452345066191_usize];
_26 = _10.1;
_12.1 = [3377904768_u32,3153811082_u32,555383976_u32,4164313075_u32,115110598_u32,3948793094_u32,3757134316_u32];
_35 = !21012_u16;
_6.1 = [_36,_36,_36];
_20 = false;
_38 = !_36;
Goto(bb21)
}
bb47 = {
_10.2 = [1055596808_u32];
_12.1 = _14;
Goto(bb15)
}
bb48 = {
_37 = _45;
_32 = 2789084661117754086_i64 >> _29;
_53.3.2 = _8 as u16;
_53.3.1 = !_15;
_30 = !_13;
_32 = 3653637011911625881_i64 << _2;
_6.2 = _6.0;
_53.3.3 = &_4;
_44 = Move(_11.1.2.0);
_11.1.1 = &_8;
_32 = _53.3.2 as i64;
_24 = _45;
_26 = _10.1;
_35 = _49;
_55 = _10.3;
_46 = core::ptr::addr_of_mut!(_50);
_2 = _7 as isize;
_13 = _16 | _16;
_53.3.3 = &_4;
_8 = -8435_i16;
_10.3 = -_25;
match _4 {
0 => bb29,
1 => bb30,
24254544491535773975890348806164761389 => bb32,
_ => bb31
}
}
bb49 = {
(*_34).0 = _10.1;
_10.1 = (*_34).0;
_36 = !_38;
_53.3.3 = &_4;
_46 = core::ptr::addr_of_mut!(_58);
_65 = _27 as u32;
_60.1 = (*_34).1;
_3 = _11.0 + _11.0;
_19 = -_11.0;
_12.1 = [_65,_65,_65,_65,_65,_65,_65];
_6.1 = [_38,_36,_36];
_28 = &_5;
RET = core::ptr::addr_of_mut!(_50);
_35 = !(*_34).2;
_29 = (-500459710_i32) as isize;
_60.2 = _53.0;
_71 = _10.0;
_60.0 = _53.3.0;
Goto(bb50)
}
bb50 = {
Call(_72 = dump_var(14_usize, 30_usize, Move(_30), 64_usize, Move(_64), 41_usize, Move(_41), 29_usize, Move(_29)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_72 = dump_var(14_usize, 56_usize, Move(_56), 1_usize, Move(_1), 5_usize, Move(_5), 23_usize, Move(_23)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_72 = dump_var(14_usize, 8_usize, Move(_8), 49_usize, Move(_49), 24_usize, Move(_24), 4_usize, Move(_4)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_72 = dump_var(14_usize, 6_usize, Move(_6), 21_usize, Move(_21), 20_usize, Move(_20), 33_usize, Move(_33)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_72 = dump_var(14_usize, 45_usize, Move(_45), 35_usize, Move(_35), 73_usize, _73, 73_usize, _73), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15() -> isize {
mir! {
type RET = isize;
let _1: (*mut i32, [u32; 7]);
let _2: *const bool;
let _3: isize;
let _4: [bool; 2];
let _5: [isize; 8];
let _6: u16;
let _7: (u8, [u64; 3], u8, [char; 6]);
let _8: u32;
let _9: ();
let _10: ();
{
RET = 29_isize + 9223372036854775807_isize;
RET = '\u{b52c5}' as isize;
RET = 9223372036854775807_isize;
_1.1 = [4115221987_u32,300836829_u32,3438473153_u32,2505740927_u32,3565199263_u32,1838853970_u32,1919276661_u32];
_1.1 = [3585072773_u32,1125341084_u32,4076838809_u32,181956019_u32,876183980_u32,2440531892_u32,4236696076_u32];
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
RET = 81_isize;
RET = false as isize;
RET = -9223372036854775807_isize;
RET = (-157774191503764089583802435247589462448_i128) as isize;
RET = 471198064_u32 as isize;
RET = (-9223372036854775808_isize) >> 4_usize;
RET = (-68_isize);
RET = (-9223372036854775808_isize) >> 116_u8;
_4 = [true,false];
_3 = RET << RET;
_5 = [RET,_3,_3,_3,_3,RET,RET,RET];
RET = -_3;
_4 = [true,true];
_5 = [RET,RET,RET,_3,_3,_3,RET,_3];
_1.1 = [3298938970_u32,178562347_u32,3719993725_u32,2363439120_u32,3901596510_u32,1105526129_u32,2562985475_u32];
_4 = [true,true];
Call(_1 = fn16(RET, _3, _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = RET;
RET = (-71_i8) as isize;
_6 = 40203_u16;
_5 = [_3,RET,_3,_3,RET,_3,_3,RET];
RET = !_3;
RET = false as isize;
RET = _3;
_4 = [false,false];
_6 = 29641467329548081925455164190916096246_u128 as u16;
_4 = [false,false];
_4 = [true,true];
_4 = [true,false];
_6 = !58271_u16;
_1.1 = [2441238119_u32,1570869845_u32,1806325135_u32,1211263700_u32,2241897851_u32,2774102106_u32,2023503011_u32];
_3 = RET;
RET = 1834536415_u32 as isize;
RET = _3;
_1.1 = [724507451_u32,335041124_u32,3398185767_u32,185311738_u32,2710011248_u32,3172120998_u32,3353363404_u32];
_3 = RET << RET;
_7.1 = [16073612879800716594_u64,5816575382235489147_u64,6184075380316783230_u64];
RET = _3 - _3;
Goto(bb2)
}
bb2 = {
Call(_9 = dump_var(15_usize, 5_usize, Move(_5), 3_usize, Move(_3), 10_usize, _10, 10_usize, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: isize,mut _3: [isize; 8],mut _4: [isize; 8]) -> (*mut i32, [u32; 7]) {
mir! {
type RET = (*mut i32, [u32; 7]);
let _5: char;
let _6: [isize; 8];
let _7: isize;
let _8: char;
let _9: Adt45;
let _10: u128;
let _11: ([u64; 6], char, [u32; 1], f64);
let _12: *const u128;
let _13: f32;
let _14: i16;
let _15: f64;
let _16: char;
let _17: *const (char, isize, u16, &'static i128);
let _18: Adt72;
let _19: bool;
let _20: u8;
let _21: isize;
let _22: i8;
let _23: Adt45;
let _24: [i8; 6];
let _25: i128;
let _26: i64;
let _27: *mut ([u64; 6], char, [u32; 1], f64);
let _28: *mut (u8, [u64; 3], u8, [char; 6]);
let _29: f32;
let _30: &'static *mut ([u64; 6], char, [u32; 1], f64);
let _31: (*mut i32, [u32; 7]);
let _32: &'static [u64; 3];
let _33: usize;
let _34: *const &'static i128;
let _35: &'static Adt36;
let _36: [u32; 7];
let _37: [isize; 2];
let _38: *const (char, isize, u16, &'static i128);
let _39: u128;
let _40: [isize; 6];
let _41: &'static *const i32;
let _42: u32;
let _43: isize;
let _44: char;
let _45: f64;
let _46: u32;
let _47: (char, isize, u16, &'static i128);
let _48: i64;
let _49: bool;
let _50: i32;
let _51: u64;
let _52: ();
let _53: ();
{
_1 = _2 + _2;
_2 = _1 << _1;
_2 = _1;
_4 = [_2,_2,_1,_1,_2,_1,_1,_1];
RET.1 = [1403679243_u32,3782693044_u32,1226620780_u32,1314690034_u32,424564706_u32,3575619778_u32,807868258_u32];
_3 = [_2,_1,_1,_1,_1,_1,_2,_2];
_2 = _1;
_3 = _4;
_2 = _1 ^ _1;
RET.1 = [3554613839_u32,640195429_u32,112134540_u32,2498167848_u32,1712271394_u32,1619895834_u32,1091411818_u32];
RET.1 = [186166199_u32,2488303407_u32,826756488_u32,2844020621_u32,3774247708_u32,1149865363_u32,2822025495_u32];
_4 = [_2,_1,_2,_2,_2,_2,_2,_2];
_3 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET.1 = [2032369926_u32,2611516094_u32,3675105714_u32,172276994_u32,3375492790_u32,458502022_u32,1926331757_u32];
RET.1 = [3897725061_u32,32940385_u32,2973615337_u32,1115903303_u32,2682521279_u32,3810488349_u32,2193315994_u32];
_5 = '\u{bc157}';
_2 = _1;
RET.1 = [1265192567_u32,3692986960_u32,133380820_u32,455325166_u32,1359689911_u32,1829048753_u32,3508234076_u32];
Goto(bb1)
}
bb1 = {
_5 = '\u{90293}';
_8 = _5;
_6 = _3;
Goto(bb2)
}
bb2 = {
_2 = 959778790_i32 as isize;
RET.1 = [4054722354_u32,3420415916_u32,657495592_u32,2597765498_u32,1408741630_u32,2134863364_u32,3720248008_u32];
_8 = _5;
_3 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb3)
}
bb3 = {
_1 = !_2;
_7 = 40009_u16 as isize;
RET.1 = [1544199707_u32,1059388163_u32,1286809540_u32,2213131904_u32,3937418136_u32,2891859077_u32,3007808232_u32];
_4 = _3;
_11.2 = [3845275810_u32];
_8 = _5;
_12 = core::ptr::addr_of!(_10);
(*_12) = 240131140446004200313882700246788441062_u128;
_8 = _5;
_11.0 = [11943229261485631094_u64,4734871827297176024_u64,11820757836534120223_u64,6703451668937885322_u64,5841775379026563365_u64,11105150305764139544_u64];
_1 = _10 as isize;
_11.3 = 159_u8 as f64;
_11.1 = _5;
_15 = 186_u8 as f64;
_3 = [_2,_1,_2,_2,_1,_7,_7,_1];
Goto(bb4)
}
bb4 = {
_16 = _8;
_15 = _11.3 - _11.3;
RET.1 = [3931304594_u32,2801143258_u32,2222779053_u32,2276999258_u32,1710526641_u32,3624662877_u32,498322549_u32];
_15 = _11.3 + _11.3;
_13 = 3356165359_u32 as f32;
_2 = (-91_i8) as isize;
RET.1 = [1092671760_u32,2539847161_u32,4211172114_u32,3240151027_u32,2223199728_u32,3912390643_u32,3852117216_u32];
_11.1 = _16;
_5 = _16;
_8 = _11.1;
_10 = !60111219252611787299318618711959109412_u128;
_14 = (-17245_i16) ^ (-27598_i16);
Goto(bb5)
}
bb5 = {
_11.0 = [17887695099823800026_u64,3942458206084491129_u64,10101726073124335914_u64,4840132832024686332_u64,12328887759649672144_u64,11524655897590709876_u64];
RET.1 = [300383789_u32,2693128143_u32,465909970_u32,1854688336_u32,2940284862_u32,3305416354_u32,712780125_u32];
_3 = _4;
Goto(bb6)
}
bb6 = {
_16 = _8;
RET.1 = [137537082_u32,103181609_u32,2560741428_u32,4191851570_u32,2731572955_u32,3617805354_u32,2143628343_u32];
_5 = _11.1;
_11.0 = [10548652944362404834_u64,16187900893004072164_u64,8012249116867766842_u64,15543152442447570182_u64,10981405456725099442_u64,12195182914149336007_u64];
_19 = _11.1 < _11.1;
(*_12) = !300879511797130290450050303650874515076_u128;
_8 = _5;
(*_12) = 181874639641112380558661236553042042998_u128 ^ 224252315782144086559432006946980721444_u128;
_8 = _16;
_10 = 53055792376483989287520943436655864250_u128;
_2 = -_1;
_5 = _11.1;
_8 = _16;
(*_12) = 175436268322754889681931679031654950999_u128 ^ 294463746064710084843084639742962661315_u128;
Call(_21 = core::intrinsics::bswap(_2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_19 = _11.1 >= _16;
_19 = false & true;
_19 = true;
RET.1 = [835626817_u32,786439487_u32,3840092230_u32,2654183358_u32,2524660968_u32,918571297_u32,6408360_u32];
_1 = _2 ^ _7;
_21 = _7;
_16 = _8;
_16 = _5;
_10 = 219229628336775489848514109228866185768_u128;
_1 = _7 * _7;
_11.3 = _15 * _15;
_20 = 28705_u16 as u8;
Goto(bb8)
}
bb8 = {
_11.0 = [12984297056290132294_u64,15007771920666108653_u64,4124316037014516108_u64,8582953590559836008_u64,3935886076932251957_u64,592928186540807951_u64];
_4 = [_21,_1,_21,_21,_21,_2,_1,_1];
Goto(bb9)
}
bb9 = {
_16 = _11.1;
_6 = [_1,_7,_2,_21,_1,_21,_7,_1];
_15 = -_11.3;
_22 = 14091_u16 as i8;
RET.1 = [1875978093_u32,2363310935_u32,701707410_u32,344099784_u32,1292762895_u32,2206407467_u32,2863531383_u32];
_20 = _13 as u8;
_11.0 = [11322952949191975035_u64,270429435848846615_u64,3277256993563096775_u64,12198197692352276280_u64,12002516807835619692_u64,10637567678090956244_u64];
_22 = 48170_u16 as i8;
RET.1 = [3961803099_u32,2618998480_u32,2716521812_u32,1940447294_u32,2382770912_u32,1541682288_u32,131219735_u32];
_8 = _11.1;
_12 = core::ptr::addr_of!((*_12));
_10 = 18005717356610404736588756406445590392_u128;
_11.1 = _8;
_13 = 11329571632032660370_usize as f32;
_20 = 54_u8 & 202_u8;
_11.3 = -_15;
_19 = _5 == _8;
_25 = 3430745474075261625797097610547146174_i128;
_24 = [_22,_22,_22,_22,_22,_22];
_11.1 = _5;
_11.2 = [534814782_u32];
_3 = [_7,_21,_21,_21,_21,_1,_2,_2];
_7 = _1;
_11.2 = [1555044314_u32];
_4 = [_21,_1,_7,_2,_1,_21,_2,_7];
match _25 {
0 => bb8,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
3430745474075261625797097610547146174 => bb15,
_ => bb14
}
}
bb10 = {
_5 = '\u{90293}';
_8 = _5;
_6 = _3;
Goto(bb2)
}
bb11 = {
_19 = _11.1 >= _16;
_19 = false & true;
_19 = true;
RET.1 = [835626817_u32,786439487_u32,3840092230_u32,2654183358_u32,2524660968_u32,918571297_u32,6408360_u32];
_1 = _2 ^ _7;
_21 = _7;
_16 = _8;
_16 = _5;
_10 = 219229628336775489848514109228866185768_u128;
_1 = _7 * _7;
_11.3 = _15 * _15;
_20 = 28705_u16 as u8;
Goto(bb8)
}
bb12 = {
_1 = !_2;
_7 = 40009_u16 as isize;
RET.1 = [1544199707_u32,1059388163_u32,1286809540_u32,2213131904_u32,3937418136_u32,2891859077_u32,3007808232_u32];
_4 = _3;
_11.2 = [3845275810_u32];
_8 = _5;
_12 = core::ptr::addr_of!(_10);
(*_12) = 240131140446004200313882700246788441062_u128;
_8 = _5;
_11.0 = [11943229261485631094_u64,4734871827297176024_u64,11820757836534120223_u64,6703451668937885322_u64,5841775379026563365_u64,11105150305764139544_u64];
_1 = _10 as isize;
_11.3 = 159_u8 as f64;
_11.1 = _5;
_15 = 186_u8 as f64;
_3 = [_2,_1,_2,_2,_1,_7,_7,_1];
Goto(bb4)
}
bb13 = {
_11.0 = [17887695099823800026_u64,3942458206084491129_u64,10101726073124335914_u64,4840132832024686332_u64,12328887759649672144_u64,11524655897590709876_u64];
RET.1 = [300383789_u32,2693128143_u32,465909970_u32,1854688336_u32,2940284862_u32,3305416354_u32,712780125_u32];
_3 = _4;
Goto(bb6)
}
bb14 = {
_16 = _8;
_15 = _11.3 - _11.3;
RET.1 = [3931304594_u32,2801143258_u32,2222779053_u32,2276999258_u32,1710526641_u32,3624662877_u32,498322549_u32];
_15 = _11.3 + _11.3;
_13 = 3356165359_u32 as f32;
_2 = (-91_i8) as isize;
RET.1 = [1092671760_u32,2539847161_u32,4211172114_u32,3240151027_u32,2223199728_u32,3912390643_u32,3852117216_u32];
_11.1 = _16;
_5 = _16;
_8 = _11.1;
_10 = !60111219252611787299318618711959109412_u128;
_14 = (-17245_i16) ^ (-27598_i16);
Goto(bb5)
}
bb15 = {
_11.3 = 417_u16 as f64;
_11.0 = [17192147245286979600_u64,12779622145334827709_u64,5623189737111858002_u64,9125052288685231300_u64,6643367961129301380_u64,11298431670813450667_u64];
_11.0 = [11408874543072713084_u64,6271647566562393484_u64,17499249248201303480_u64,8931071931505855252_u64,16185601699962814127_u64,13952669817361010836_u64];
_22 = !(-35_i8);
_6 = _4;
_3 = _4;
_19 = true & false;
_12 = core::ptr::addr_of!((*_12));
_2 = _7;
_11.1 = _16;
_8 = _5;
_11.0 = [10659546241403339843_u64,4608059430180525876_u64,15881496723907504061_u64,2408590474165221044_u64,13401707152353499483_u64,3175397430206595680_u64];
match _10 {
0 => bb7,
1 => bb2,
2 => bb6,
3 => bb9,
4 => bb5,
18005717356610404736588756406445590392 => bb17,
_ => bb16
}
}
bb16 = {
_16 = _8;
_15 = _11.3 - _11.3;
RET.1 = [3931304594_u32,2801143258_u32,2222779053_u32,2276999258_u32,1710526641_u32,3624662877_u32,498322549_u32];
_15 = _11.3 + _11.3;
_13 = 3356165359_u32 as f32;
_2 = (-91_i8) as isize;
RET.1 = [1092671760_u32,2539847161_u32,4211172114_u32,3240151027_u32,2223199728_u32,3912390643_u32,3852117216_u32];
_11.1 = _16;
_5 = _16;
_8 = _11.1;
_10 = !60111219252611787299318618711959109412_u128;
_14 = (-17245_i16) ^ (-27598_i16);
Goto(bb5)
}
bb17 = {
_5 = _11.1;
_22 = !(-87_i8);
_22 = 88_i8;
_11.3 = _15;
_10 = 316882724749971506173115118117438997863_u128 + 153784966806593093941890275064752386322_u128;
_24 = [_22,_22,_22,_22,_22,_22];
_27 = core::ptr::addr_of_mut!(_11);
_26 = 4787577597971224538_i64;
RET.1 = [465246658_u32,1711093476_u32,3749581355_u32,1196305118_u32,2415009786_u32,2843494267_u32,3508288627_u32];
(*_12) = 290696986823131991560424777090747331349_u128;
(*_27).2 = [3422298685_u32];
_14 = 1656345767_i32 as i16;
_11.1 = _16;
(*_12) = _20 as u128;
_25 = (-298874546132280701017403486467479538_i128);
Goto(bb18)
}
bb18 = {
(*_27).1 = _8;
_11.0 = [9592916440897356975_u64,1501100290320062332_u64,17894703642884421754_u64,14506454270036103728_u64,13812890257720039380_u64,2485030605950238287_u64];
(*_27).2 = [3418281771_u32];
_11.1 = _16;
_29 = _13 - _13;
_21 = _1;
_21 = _2 * _7;
match _25 {
339983492374806182762357203945300731918 => bb19,
_ => bb11
}
}
bb19 = {
_16 = (*_27).1;
(*_27).2 = [585553161_u32];
_6 = [_1,_1,_1,_7,_1,_1,_7,_2];
_31.1 = [3107817109_u32,1891643922_u32,2450368239_u32,986591204_u32,1719219495_u32,2375628543_u32,2057147565_u32];
Call((*_27) = fn17(_25), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
(*_27).0 = [13868139709499153736_u64,7067997152790197420_u64,16109160479817521726_u64,12592130923452578721_u64,11101590643673205769_u64,604237645291173746_u64];
(*_27).2 = [136935629_u32];
_10 = 326902857970049641605614351868788195355_u128;
_8 = (*_27).1;
_30 = &_27;
_15 = -(*_27).3;
_21 = 5_usize as isize;
_27 = core::ptr::addr_of_mut!(_11);
(*_27).1 = _16;
_24 = [_22,_22,_22,_22,_22,_22];
_16 = (*_27).1;
_37 = [_1,_7];
Goto(bb21)
}
bb21 = {
_1 = _21 * _7;
_8 = (*_27).1;
_6 = [_7,_21,_1,_1,_7,_2,_2,_1];
_27 = core::ptr::addr_of_mut!(_11);
_7 = _1 << _20;
_1 = (*_27).3 as isize;
_12 = core::ptr::addr_of!(_10);
_16 = (*_27).1;
_40 = [_7,_7,_1,_7,_7,_1];
_19 = !true;
_5 = (*_27).1;
_12 = core::ptr::addr_of!((*_12));
RET.1 = [428705538_u32,1736987277_u32,1541654935_u32,3215729191_u32,3575777217_u32,2646767188_u32,978135223_u32];
_26 = _19 as i64;
(*_27).0 = [717330845872591008_u64,15921990336754684976_u64,12238543384658856464_u64,16795224123024502876_u64,8855619013770333619_u64,1981710491044523099_u64];
_30 = &_27;
_11.2 = [677788072_u32];
_11.2 = [3097319253_u32];
Goto(bb22)
}
bb22 = {
_36 = [1278750543_u32,3334396220_u32,2669221250_u32,1494490925_u32,1074933399_u32,1351200387_u32,1819424163_u32];
_11.2 = [3948598249_u32];
_5 = _8;
_45 = _11.3 * _15;
_39 = (*_12);
_37 = [_7,_7];
_30 = &(*_30);
_44 = (*_27).1;
_2 = !_7;
_11.0 = [10522841445368975554_u64,16617206017614341273_u64,16525808399489087692_u64,547401139325004720_u64,13660160042369049800_u64,16912801085139953579_u64];
_39 = !(*_12);
_1 = _13 as isize;
_47.3 = &_25;
(*_27).0 = [17383378527347369454_u64,2612882808805269227_u64,16852057761959810825_u64,12732080032819812465_u64,12477593620468324869_u64,5495162324150681346_u64];
_31.1 = [2863454015_u32,310822632_u32,2172379957_u32,1352425344_u32,115696676_u32,885484723_u32,4245206364_u32];
(*_27).2 = [1331376647_u32];
_46 = !2414220465_u32;
_42 = _46 * _46;
_47.1 = _20 as isize;
_3 = [_2,_2,_2,_7,_2,_2,_2,_2];
_6 = _3;
_4 = _3;
RET.1 = _36;
_5 = (*_27).1;
match _10 {
0 => bb11,
1 => bb16,
2 => bb3,
3 => bb4,
326902857970049641605614351868788195355 => bb23,
_ => bb12
}
}
bb23 = {
_11.1 = _8;
_42 = _46;
_21 = _7 & _7;
match (*_12) {
0 => bb10,
326902857970049641605614351868788195355 => bb24,
_ => bb19
}
}
bb24 = {
_47.0 = _8;
match _10 {
0 => bb25,
1 => bb26,
2 => bb27,
326902857970049641605614351868788195355 => bb29,
_ => bb28
}
}
bb25 = {
_11.1 = _8;
_42 = _46;
_21 = _7 & _7;
match (*_12) {
0 => bb10,
326902857970049641605614351868788195355 => bb24,
_ => bb19
}
}
bb26 = {
_1 = !_2;
_7 = 40009_u16 as isize;
RET.1 = [1544199707_u32,1059388163_u32,1286809540_u32,2213131904_u32,3937418136_u32,2891859077_u32,3007808232_u32];
_4 = _3;
_11.2 = [3845275810_u32];
_8 = _5;
_12 = core::ptr::addr_of!(_10);
(*_12) = 240131140446004200313882700246788441062_u128;
_8 = _5;
_11.0 = [11943229261485631094_u64,4734871827297176024_u64,11820757836534120223_u64,6703451668937885322_u64,5841775379026563365_u64,11105150305764139544_u64];
_1 = _10 as isize;
_11.3 = 159_u8 as f64;
_11.1 = _5;
_15 = 186_u8 as f64;
_3 = [_2,_1,_2,_2,_1,_7,_7,_1];
Goto(bb4)
}
bb27 = {
_11.0 = [17887695099823800026_u64,3942458206084491129_u64,10101726073124335914_u64,4840132832024686332_u64,12328887759649672144_u64,11524655897590709876_u64];
RET.1 = [300383789_u32,2693128143_u32,465909970_u32,1854688336_u32,2940284862_u32,3305416354_u32,712780125_u32];
_3 = _4;
Goto(bb6)
}
bb28 = {
_16 = _11.1;
_6 = [_1,_7,_2,_21,_1,_21,_7,_1];
_15 = -_11.3;
_22 = 14091_u16 as i8;
RET.1 = [1875978093_u32,2363310935_u32,701707410_u32,344099784_u32,1292762895_u32,2206407467_u32,2863531383_u32];
_20 = _13 as u8;
_11.0 = [11322952949191975035_u64,270429435848846615_u64,3277256993563096775_u64,12198197692352276280_u64,12002516807835619692_u64,10637567678090956244_u64];
_22 = 48170_u16 as i8;
RET.1 = [3961803099_u32,2618998480_u32,2716521812_u32,1940447294_u32,2382770912_u32,1541682288_u32,131219735_u32];
_8 = _11.1;
_12 = core::ptr::addr_of!((*_12));
_10 = 18005717356610404736588756406445590392_u128;
_11.1 = _8;
_13 = 11329571632032660370_usize as f32;
_20 = 54_u8 & 202_u8;
_11.3 = -_15;
_19 = _5 == _8;
_25 = 3430745474075261625797097610547146174_i128;
_24 = [_22,_22,_22,_22,_22,_22];
_11.1 = _5;
_11.2 = [534814782_u32];
_3 = [_7,_21,_21,_21,_21,_1,_2,_2];
_7 = _1;
_11.2 = [1555044314_u32];
_4 = [_21,_1,_7,_2,_1,_21,_2,_7];
match _25 {
0 => bb8,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
3430745474075261625797097610547146174 => bb15,
_ => bb14
}
}
bb29 = {
_19 = _2 <= _2;
_45 = 11817_u16 as f64;
_34 = core::ptr::addr_of!(_47.3);
_31.1 = _36;
RET.0 = core::ptr::addr_of_mut!(_50);
_43 = _2 ^ _21;
_16 = _44;
_43 = _7;
_46 = _42 * _42;
_27 = core::ptr::addr_of_mut!(_11);
RET.0 = core::ptr::addr_of_mut!(_50);
_47.0 = _5;
Goto(bb30)
}
bb30 = {
Call(_52 = dump_var(16_usize, 5_usize, Move(_5), 39_usize, Move(_39), 36_usize, Move(_36), 4_usize, Move(_4)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_52 = dump_var(16_usize, 46_usize, Move(_46), 8_usize, Move(_8), 19_usize, Move(_19), 6_usize, Move(_6)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_52 = dump_var(16_usize, 44_usize, Move(_44), 20_usize, Move(_20), 1_usize, Move(_1), 40_usize, Move(_40)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_52 = dump_var(16_usize, 26_usize, Move(_26), 53_usize, _53, 53_usize, _53, 53_usize, _53), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i128) -> ([u64; 6], char, [u32; 1], f64) {
mir! {
type RET = ([u64; 6], char, [u32; 1], f64);
let _2: u16;
let _3: *mut (u8, [u64; 3], u8, [char; 6]);
let _4: Adt40;
let _5: isize;
let _6: isize;
let _7: (*mut i32, [u32; 7]);
let _8: &'static *const bool;
let _9: isize;
let _10: i8;
let _11: f32;
let _12: f32;
let _13: u64;
let _14: *const [u32; 7];
let _15: [u32; 7];
let _16: [i8; 6];
let _17: (&'static u32, &'static i16, (&'static i16,));
let _18: i64;
let _19: Adt40;
let _20: (u64, [u32; 1], f32);
let _21: [usize; 2];
let _22: [u64; 6];
let _23: ();
let _24: ();
{
RET.0 = [7768853254606704691_u64,2460850012208122135_u64,12623044817950926203_u64,14459624379057580071_u64,5352113818080911491_u64,5738285555500332460_u64];
RET.3 = _1 as f64;
RET.0 = [15489115343975940868_u64,3387246660884211416_u64,17685961737107983419_u64,6293509422016644522_u64,2898539734403057215_u64,2798083924385625556_u64];
RET.2 = [3425962571_u32];
_1 = 2776076657_u32 as i128;
Goto(bb1)
}
bb1 = {
_2 = !32673_u16;
Goto(bb2)
}
bb2 = {
RET.1 = '\u{6503d}';
RET.0 = [13803487263858887747_u64,3008988328735020908_u64,11681829607693351109_u64,16949573645158139278_u64,12985165023698691230_u64,7517817184541625188_u64];
RET.3 = (-2803171379125523249_i64) as f64;
RET.2 = [2147312792_u32];
RET.0 = [7246929466701614458_u64,8297105663007710093_u64,12503410044819396736_u64,18339753433780937720_u64,8876986994389756716_u64,9369165977853453999_u64];
RET.2 = [3261193747_u32];
RET.1 = '\u{f1ba4}';
RET.1 = '\u{a2276}';
RET.2 = [94828895_u32];
_2 = 13361_u16;
_1 = 18190744391770014392109341214417621022_i128 * (-166810384943212490354516451341283551372_i128);
RET.1 = '\u{e71c6}';
RET.1 = '\u{3c9a0}';
RET.2 = [4050102951_u32];
RET.1 = '\u{771b3}';
RET.0 = [4176091463727016617_u64,2760227842139905672_u64,10652373580188709819_u64,9974285760875499033_u64,598304356707723858_u64,10514700875616260842_u64];
RET.1 = '\u{798d2}';
RET.1 = '\u{fc589}';
RET.2 = [1257529685_u32];
_5 = 9223372036854775807_isize;
Goto(bb3)
}
bb3 = {
RET.1 = '\u{10038a}';
RET.0 = [7082902640521049189_u64,10879126723732399983_u64,16971293677052566217_u64,10688011635476317779_u64,6611839708143198682_u64,13614554590299184746_u64];
RET.2 = [2893857112_u32];
RET.2 = [680961416_u32];
_6 = _5;
RET.2 = [211567209_u32];
_5 = -_6;
RET.3 = (-3498_i16) as f64;
RET.3 = 2652052949_u32 as f64;
RET.0 = [1481605428374882824_u64,10090058095487327683_u64,11975128194906878066_u64,10500191071004028824_u64,17673559039191444737_u64,12278111125162820193_u64];
RET.2 = [842141629_u32];
RET.0 = [7166513637167216364_u64,16700799357416097163_u64,11740685969347492024_u64,2875568700204337801_u64,11052570171563598379_u64,10539720031256950861_u64];
_6 = !_5;
RET.3 = 1059648854016709249_usize as f64;
match _2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
13361 => bb7,
_ => bb6
}
}
bb4 = {
RET.1 = '\u{6503d}';
RET.0 = [13803487263858887747_u64,3008988328735020908_u64,11681829607693351109_u64,16949573645158139278_u64,12985165023698691230_u64,7517817184541625188_u64];
RET.3 = (-2803171379125523249_i64) as f64;
RET.2 = [2147312792_u32];
RET.0 = [7246929466701614458_u64,8297105663007710093_u64,12503410044819396736_u64,18339753433780937720_u64,8876986994389756716_u64,9369165977853453999_u64];
RET.2 = [3261193747_u32];
RET.1 = '\u{f1ba4}';
RET.1 = '\u{a2276}';
RET.2 = [94828895_u32];
_2 = 13361_u16;
_1 = 18190744391770014392109341214417621022_i128 * (-166810384943212490354516451341283551372_i128);
RET.1 = '\u{e71c6}';
RET.1 = '\u{3c9a0}';
RET.2 = [4050102951_u32];
RET.1 = '\u{771b3}';
RET.0 = [4176091463727016617_u64,2760227842139905672_u64,10652373580188709819_u64,9974285760875499033_u64,598304356707723858_u64,10514700875616260842_u64];
RET.1 = '\u{798d2}';
RET.1 = '\u{fc589}';
RET.2 = [1257529685_u32];
_5 = 9223372036854775807_isize;
Goto(bb3)
}
bb5 = {
_2 = !32673_u16;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
RET.1 = '\u{94150}';
RET.2 = [397651319_u32];
RET.0 = [17605398833575296678_u64,12056074600509224267_u64,6949812055949689664_u64,18413366311807824120_u64,12613530144820316196_u64,10655976871261153214_u64];
RET.3 = 680564223_u32 as f64;
RET.2 = [2511458966_u32];
_2 = 62179_u16;
RET.2 = [400257558_u32];
RET.3 = _6 as f64;
_6 = _5 << _5;
RET.3 = 17_u8 as f64;
RET.0 = [205324694400752658_u64,5030519282023360798_u64,9290033907753564025_u64,12351988611800489701_u64,13136033904876033509_u64,2337394874179403089_u64];
RET.3 = 0_usize as f64;
RET.3 = 2090947407857110594_usize as f64;
Goto(bb8)
}
bb8 = {
_7.1 = [1697258046_u32,3769134359_u32,2132469299_u32,2548917897_u32,823491614_u32,3872370448_u32,1442668677_u32];
_7.1 = [2518723160_u32,2177044719_u32,3196012170_u32,2595288152_u32,2789864541_u32,4209148827_u32,1434630982_u32];
RET.0 = [17912235448851790662_u64,1999035590428701508_u64,16919208112546121410_u64,4986554175944830728_u64,1918228523355880061_u64,1025618352628093627_u64];
_10 = 96_i8;
RET.2 = [2187622287_u32];
RET.1 = '\u{d2faa}';
RET.1 = '\u{2ddf2}';
_12 = (-807835677936568979_i64) as f32;
_6 = _5 ^ _5;
_9 = !_6;
RET.0 = [8420389729715828867_u64,4141679532517321051_u64,5450327430327006218_u64,10964405096170962595_u64,17281928777124570575_u64,16980029760670503575_u64];
_1 = (-34315478180575743816415665414494194013_i128);
_6 = _9;
_14 = core::ptr::addr_of!(_7.1);
_5 = _9;
_12 = 8078288821217925760_i64 as f32;
RET.3 = 1014093957_u32 as f64;
RET.1 = '\u{10d165}';
_6 = -_5;
RET.0 = [14110351735669949097_u64,1291382290339970592_u64,15029782196161932081_u64,14498368891056069388_u64,12173744772976747096_u64,7710058287557695412_u64];
RET.2 = [3845660131_u32];
Goto(bb9)
}
bb9 = {
_2 = !3371_u16;
_2 = 40348_u16;
RET.0 = [2393167150140047043_u64,14355683797370472303_u64,12588381826727519997_u64,9406407356898725689_u64,11363490379417170684_u64,5709697646516520830_u64];
_13 = _10 as u64;
_12 = _2 as f32;
_1 = 103883168528039190608007769335707757217_i128;
RET.0 = [_13,_13,_13,_13,_13,_13];
_15 = _7.1;
RET.3 = 814109154_u32 as f64;
RET.3 = 24838172812242931722763683502145451291_u128 as f64;
RET.0 = [_13,_13,_13,_13,_13,_13];
_5 = _9 >> _10;
_14 = core::ptr::addr_of!(_15);
_12 = 6_u8 as f32;
RET.0 = [_13,_13,_13,_13,_13,_13];
_1 = 90989765531828302122768562856982595077_i128 - 105523188577083599658278903332753410718_i128;
_6 = !_9;
_14 = core::ptr::addr_of!((*_14));
_11 = _12 - _12;
RET.2 = [3956046699_u32];
_18 = (-5546210500879624082_i64) * 5616132908486606033_i64;
Call((*_14) = core::intrinsics::transmute(_7.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16 = [_10,_10,_10,_10,_10,_10];
RET.1 = '\u{bc264}';
_18 = 57326672095357698182435330873315092455_u128 as i64;
RET.2 = [1708412068_u32];
_14 = core::ptr::addr_of!(_7.1);
_1 = (-19122816953032956105141287862754253138_i128);
_9 = RET.3 as isize;
RET.1 = '\u{b08d}';
_16 = [_10,_10,_10,_10,_10,_10];
_18 = 3472400814710645362_i64;
RET.2 = [1750492165_u32];
_20.0 = _13;
RET.2 = [3390123348_u32];
_18 = 4982344744936721448_i64;
_18 = (-8056989726209923314_i64) << _10;
_21 = [914677432403338349_usize,2_usize];
_20.2 = _11 - _12;
match _1 {
0 => bb7,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
321159549967905507358233319569013958318 => bb18,
_ => bb17
}
}
bb11 = {
RET.1 = '\u{6503d}';
RET.0 = [13803487263858887747_u64,3008988328735020908_u64,11681829607693351109_u64,16949573645158139278_u64,12985165023698691230_u64,7517817184541625188_u64];
RET.3 = (-2803171379125523249_i64) as f64;
RET.2 = [2147312792_u32];
RET.0 = [7246929466701614458_u64,8297105663007710093_u64,12503410044819396736_u64,18339753433780937720_u64,8876986994389756716_u64,9369165977853453999_u64];
RET.2 = [3261193747_u32];
RET.1 = '\u{f1ba4}';
RET.1 = '\u{a2276}';
RET.2 = [94828895_u32];
_2 = 13361_u16;
_1 = 18190744391770014392109341214417621022_i128 * (-166810384943212490354516451341283551372_i128);
RET.1 = '\u{e71c6}';
RET.1 = '\u{3c9a0}';
RET.2 = [4050102951_u32];
RET.1 = '\u{771b3}';
RET.0 = [4176091463727016617_u64,2760227842139905672_u64,10652373580188709819_u64,9974285760875499033_u64,598304356707723858_u64,10514700875616260842_u64];
RET.1 = '\u{798d2}';
RET.1 = '\u{fc589}';
RET.2 = [1257529685_u32];
_5 = 9223372036854775807_isize;
Goto(bb3)
}
bb12 = {
_7.1 = [1697258046_u32,3769134359_u32,2132469299_u32,2548917897_u32,823491614_u32,3872370448_u32,1442668677_u32];
_7.1 = [2518723160_u32,2177044719_u32,3196012170_u32,2595288152_u32,2789864541_u32,4209148827_u32,1434630982_u32];
RET.0 = [17912235448851790662_u64,1999035590428701508_u64,16919208112546121410_u64,4986554175944830728_u64,1918228523355880061_u64,1025618352628093627_u64];
_10 = 96_i8;
RET.2 = [2187622287_u32];
RET.1 = '\u{d2faa}';
RET.1 = '\u{2ddf2}';
_12 = (-807835677936568979_i64) as f32;
_6 = _5 ^ _5;
_9 = !_6;
RET.0 = [8420389729715828867_u64,4141679532517321051_u64,5450327430327006218_u64,10964405096170962595_u64,17281928777124570575_u64,16980029760670503575_u64];
_1 = (-34315478180575743816415665414494194013_i128);
_6 = _9;
_14 = core::ptr::addr_of!(_7.1);
_5 = _9;
_12 = 8078288821217925760_i64 as f32;
RET.3 = 1014093957_u32 as f64;
RET.1 = '\u{10d165}';
_6 = -_5;
RET.0 = [14110351735669949097_u64,1291382290339970592_u64,15029782196161932081_u64,14498368891056069388_u64,12173744772976747096_u64,7710058287557695412_u64];
RET.2 = [3845660131_u32];
Goto(bb9)
}
bb13 = {
RET.1 = '\u{94150}';
RET.2 = [397651319_u32];
RET.0 = [17605398833575296678_u64,12056074600509224267_u64,6949812055949689664_u64,18413366311807824120_u64,12613530144820316196_u64,10655976871261153214_u64];
RET.3 = 680564223_u32 as f64;
RET.2 = [2511458966_u32];
_2 = 62179_u16;
RET.2 = [400257558_u32];
RET.3 = _6 as f64;
_6 = _5 << _5;
RET.3 = 17_u8 as f64;
RET.0 = [205324694400752658_u64,5030519282023360798_u64,9290033907753564025_u64,12351988611800489701_u64,13136033904876033509_u64,2337394874179403089_u64];
RET.3 = 0_usize as f64;
RET.3 = 2090947407857110594_usize as f64;
Goto(bb8)
}
bb14 = {
Return()
}
bb15 = {
_2 = !32673_u16;
Goto(bb2)
}
bb16 = {
RET.1 = '\u{6503d}';
RET.0 = [13803487263858887747_u64,3008988328735020908_u64,11681829607693351109_u64,16949573645158139278_u64,12985165023698691230_u64,7517817184541625188_u64];
RET.3 = (-2803171379125523249_i64) as f64;
RET.2 = [2147312792_u32];
RET.0 = [7246929466701614458_u64,8297105663007710093_u64,12503410044819396736_u64,18339753433780937720_u64,8876986994389756716_u64,9369165977853453999_u64];
RET.2 = [3261193747_u32];
RET.1 = '\u{f1ba4}';
RET.1 = '\u{a2276}';
RET.2 = [94828895_u32];
_2 = 13361_u16;
_1 = 18190744391770014392109341214417621022_i128 * (-166810384943212490354516451341283551372_i128);
RET.1 = '\u{e71c6}';
RET.1 = '\u{3c9a0}';
RET.2 = [4050102951_u32];
RET.1 = '\u{771b3}';
RET.0 = [4176091463727016617_u64,2760227842139905672_u64,10652373580188709819_u64,9974285760875499033_u64,598304356707723858_u64,10514700875616260842_u64];
RET.1 = '\u{798d2}';
RET.1 = '\u{fc589}';
RET.2 = [1257529685_u32];
_5 = 9223372036854775807_isize;
Goto(bb3)
}
bb17 = {
RET.1 = '\u{10038a}';
RET.0 = [7082902640521049189_u64,10879126723732399983_u64,16971293677052566217_u64,10688011635476317779_u64,6611839708143198682_u64,13614554590299184746_u64];
RET.2 = [2893857112_u32];
RET.2 = [680961416_u32];
_6 = _5;
RET.2 = [211567209_u32];
_5 = -_6;
RET.3 = (-3498_i16) as f64;
RET.3 = 2652052949_u32 as f64;
RET.0 = [1481605428374882824_u64,10090058095487327683_u64,11975128194906878066_u64,10500191071004028824_u64,17673559039191444737_u64,12278111125162820193_u64];
RET.2 = [842141629_u32];
RET.0 = [7166513637167216364_u64,16700799357416097163_u64,11740685969347492024_u64,2875568700204337801_u64,11052570171563598379_u64,10539720031256950861_u64];
_6 = !_5;
RET.3 = 1059648854016709249_usize as f64;
match _2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
13361 => bb7,
_ => bb6
}
}
bb18 = {
_1 = (-4877448887116625066154990505367202879_i128) ^ 31310461872369959949928271092328817677_i128;
Goto(bb19)
}
bb19 = {
Call(_23 = dump_var(17_usize, 2_usize, Move(_2), 15_usize, Move(_15), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_23 = dump_var(17_usize, 10_usize, Move(_10), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: *mut [isize; 1],mut _6: isize,mut _7: isize,mut _8: i128) -> isize {
mir! {
type RET = isize;
let _9: (f32, (&'static u32, &'static i16, (&'static i16,)));
let _10: char;
let _11: ();
let _12: ();
{
_4 = _6;
_7 = _8 as isize;
RET = _3;
_1 = 311693158787056467309000084630216686522_u128 as isize;
RET = _4 * _4;
_4 = 11913705136955752009_u64 as isize;
_7 = 70_i8 as isize;
_6 = _3;
_8 = 45320499469666110738749984143768876522_i128;
RET = !_6;
_4 = _3;
_2 = !_4;
_9.0 = 3061650493_u32 as f32;
_3 = _2 >> _2;
RET = !_4;
_8 = (-29872_i16) as i128;
RET = !_6;
RET = _3 * _4;
_2 = _3;
_2 = RET;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(18_usize, 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(9058584758129608097_usize), std::hint::black_box('\u{776fa}'), std::hint::black_box(34418_u16), std::hint::black_box(2469081011_u32));
                
            }
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: i32,
fld1: char,
fld2: [u128; 4],

},
Variant1{
fld0: bool,
fld1: u64,
fld2: *const isize,
fld3: u32,
fld4: [u128; 4],
fld5: i32,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: i64,
fld1: *mut i32,
fld2: u128,
fld3: [u64; 6],
fld4: i32,

},
Variant1{
fld0: bool,
fld1: u64,
fld2: [u64; 6],
fld3: Adt32,
fld4: u128,
fld5: [u32; 1],

},
Variant2{
fld0: bool,
fld1: [u64; 6],
fld2: f32,
fld3: u64,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: (u32, u32, i128),

},
Variant1{
fld0: u16,
fld1: u8,
fld2: (*const i32, bool, *const f64, u32),
fld3: Adt32,

},
Variant2{
fld0: *const i32,
fld1: *mut i32,
fld2: i16,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: f64,
fld1: char,
fld2: [usize; 2],
fld3: i8,
fld4: [u128; 4],

},
Variant1{
fld0: bool,
fld1: (u32, u32, i128),
fld2: isize,
fld3: [char; 6],
fld4: i16,
fld5: u8,
fld6: *const f64,
fld7: f64,

},
Variant2{
fld0: (u8, [u64; 3], u8, [char; 6]),
fld1: f32,
fld2: Adt32,
fld3: u64,
fld4: [u64; 6],
fld5: *const i32,
fld6: i64,
fld7: ([u64; 6], char, [u32; 1], f64),

},
Variant3{
fld0: bool,
fld1: *mut i32,
fld2: *const isize,
fld3: f64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i128,
fld1: u128,
fld2: (*const i32, bool, *const f64, u32),
fld3: i8,
fld4: [u32; 1],
fld5: f32,
fld6: u32,

},
Variant1{
fld0: bool,
fld1: *mut i32,
fld2: [u128; 4],

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: Adt45,
fld1: Adt40,
fld2: *const f64,
fld3: usize,
fld4: [u32; 7],
fld5: u8,
fld6: u32,
fld7: i128,

},
Variant1{
fld0: u32,
fld1: i32,
fld2: [isize; 1],
fld3: i8,

},
Variant2{
fld0: u64,
fld1: Adt32,

}}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){unsafe{printf("Adt72::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt72 {
Variant0{
fld0: u16,
fld1: usize,
fld2: *mut i32,
fld3: *const f64,
fld4: i16,
fld5: Adt39,
fld6: *mut (u8, [u64; 3], u8, [char; 6]),

},
Variant1{
fld0: usize,
fld1: (u8, [u64; 3], u8, [char; 6]),
fld2: [isize; 1],
fld3: [isize; 2],
fld4: Adt40,
fld5: *const f64,

},
Variant2{
fld0: (u64, [u32; 1], f32),
fld1: (u8, [u64; 3], u8, [char; 6]),
fld2: [u128; 4],

},
Variant3{
fld0: u32,
fld1: [u64; 3],
fld2: isize,
fld3: [i8; 3],
fld4: *mut (u8, [u64; 3], u8, [char; 6]),
fld5: (u8, [u64; 3], u8, [char; 6]),

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: ([u64; 6], char, [u32; 1], f64),
fld1: (Adt36, u32, i8),
fld2: [u32; 1],
fld3: [u64; 3],
fld4: [usize; 2],

},
Variant1{
fld0: [u32; 1],
fld1: [usize; 2],

},
Variant2{
fld0: [char; 6],
fld1: [isize; 6],
fld2: [i16; 8],
fld3: i64,
fld4: (*const i32, bool, *const f64, u32),

},
Variant3{
fld0: u32,
fld1: *mut ([u64; 6], char, [u32; 1], f64),

}}

