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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u16,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u128,mut _8: i128,mut _9: usize,mut _10: u8) -> *const [u8; 2] {
mir! {
type RET = *const [u8; 2];
let _11: bool;
let _12: i8;
let _13: Adt32;
let _14: [u16; 3];
let _15: isize;
let _16: [u128; 1];
let _17: i8;
let _18: &'static Adt65;
let _19: *const u8;
let _20: isize;
let _21: isize;
let _22: [u8; 2];
let _23: ((&'static u32, u128, f64, char), bool);
let _24: isize;
let _25: u32;
let _26: Adt21;
let _27: [isize; 2];
let _28: i8;
let _29: f32;
let _30: (u16, ((&'static u32, u128, f64, char), bool), ((u16, i32, u32),));
let _31: (*mut f64, bool, (u16, i32, u32), isize);
let _32: (*const i8, u16, Adt32, &'static &'static bool);
let _33: *const [u128; 1];
let _34: u8;
let _35: [isize; 2];
let _36: f64;
let _37: isize;
let _38: char;
let _39: Adt17;
let _40: bool;
let _41: f32;
let _42: *const i8;
let _43: bool;
let _44: char;
let _45: ();
let _46: ();
{
_5 = 80_i8 as i16;
_2 = '\u{a6690}';
_6 = 1448181662_i32;
_4 = 16_i8 & 66_i8;
_7 = 194847873791867363747731685150740422041_u128 << _5;
_7 = 171489424752708734617593692055059531671_u128 ^ 133722768088515849914391884364300241112_u128;
_5 = -(-13537_i16);
_4 = -6_i8;
_9 = (-1972140242924919610_i64) as usize;
_10 = _4 as u8;
_3 = !34992_u16;
_4 = 36_i8 ^ 59_i8;
_8 = (-70004460792327621111671899966857919504_i128);
_5 = (-12435_i16) << _10;
_1 = true;
_7 = !210902397760101573397688326568237448616_u128;
_8 = 145942860375443334522804026675828551081_i128;
_7 = 117267205245190322882202366736709372513_u128;
_4 = -(-10_i8);
_8 = (-109606562284786485782275529347861196054_i128);
_1 = _5 != _5;
_11 = !_1;
_7 = 311604744515239175548931567318044028492_u128;
_8 = _2 as i128;
_2 = '\u{a2501}';
_2 = '\u{887ea}';
_7 = _9 as u128;
_7 = 28567328192418340062813586441841609265_u128;
match _6 {
1448181662 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_7 = 298269414010920076891972027200380954658_u128;
_12 = _4;
_9 = 2390955334029859627_usize * 1_usize;
_7 = !165921927072045436302632004540300089303_u128;
_8 = 18272455004961761628_u64 as i128;
_5 = 24253_i16;
_7 = 220672732877857178868208296069183433064_u128 ^ 206255479825611947634835701694136485473_u128;
_11 = _1;
_8 = _10 as i128;
_6 = (-1119400369_i32);
_10 = 3479760960_u32 as u8;
_2 = '\u{d89d5}';
_3 = _11 as u16;
_4 = _12 << _6;
_2 = '\u{2ccaf}';
_9 = 3_usize;
_16 = [_7];
_3 = !59222_u16;
_1 = _2 != _2;
_3 = 16610_u16;
_8 = 14642724117561981766_u64 as i128;
_8 = 34967559519432001486329849290803730822_i128;
Goto(bb3)
}
bb3 = {
_8 = (-153941496970719116169513867608137361273_i128) | (-168255192783616589447874802815914293247_i128);
_12 = -_4;
_3 = !61693_u16;
_7 = _3 as u128;
_11 = _1;
_11 = _1;
_3 = _4 as u16;
_17 = !_4;
_8 = 170215492_u32 as i128;
_11 = _1 & _1;
_9 = _7 as usize;
_10 = 161_u8 * 15_u8;
_15 = -(-38_isize);
_9 = !7_usize;
_3 = 51559_u16 - 660_u16;
_14 = [_3,_3,_3];
_9 = 11871963003684610401_usize;
_1 = _12 > _4;
_3 = _15 as u16;
_16 = [_7];
match _5 {
0 => bb2,
1 => bb4,
24253 => bb6,
_ => bb5
}
}
bb4 = {
_7 = 298269414010920076891972027200380954658_u128;
_12 = _4;
_9 = 2390955334029859627_usize * 1_usize;
_7 = !165921927072045436302632004540300089303_u128;
_8 = 18272455004961761628_u64 as i128;
_5 = 24253_i16;
_7 = 220672732877857178868208296069183433064_u128 ^ 206255479825611947634835701694136485473_u128;
_11 = _1;
_8 = _10 as i128;
_6 = (-1119400369_i32);
_10 = 3479760960_u32 as u8;
_2 = '\u{d89d5}';
_3 = _11 as u16;
_4 = _12 << _6;
_2 = '\u{2ccaf}';
_9 = 3_usize;
_16 = [_7];
_3 = !59222_u16;
_1 = _2 != _2;
_3 = 16610_u16;
_8 = 14642724117561981766_u64 as i128;
_8 = 34967559519432001486329849290803730822_i128;
Goto(bb3)
}
bb5 = {
Return()
}
bb6 = {
_4 = _12;
_12 = _15 as i8;
_6 = -(-1567959012_i32);
_17 = _4;
_6 = -(-1498170939_i32);
_16 = [_7];
_2 = '\u{10137c}';
_10 = 250_u8 << _3;
_6 = _10 as i32;
_6 = _10 as i32;
_22 = [_10,_10];
_8 = (-140154013630835074005154191460095932943_i128);
_1 = _11 > _11;
RET = core::ptr::addr_of!(_22);
(*RET) = [_10,_10];
_16 = [_7];
(*RET) = [_10,_10];
_1 = !_11;
_19 = core::ptr::addr_of!(_10);
match _8 {
200128353290103389458220415971672278513 => bb8,
_ => bb7
}
}
bb7 = {
_7 = 298269414010920076891972027200380954658_u128;
_12 = _4;
_9 = 2390955334029859627_usize * 1_usize;
_7 = !165921927072045436302632004540300089303_u128;
_8 = 18272455004961761628_u64 as i128;
_5 = 24253_i16;
_7 = 220672732877857178868208296069183433064_u128 ^ 206255479825611947634835701694136485473_u128;
_11 = _1;
_8 = _10 as i128;
_6 = (-1119400369_i32);
_10 = 3479760960_u32 as u8;
_2 = '\u{d89d5}';
_3 = _11 as u16;
_4 = _12 << _6;
_2 = '\u{2ccaf}';
_9 = 3_usize;
_16 = [_7];
_3 = !59222_u16;
_1 = _2 != _2;
_3 = 16610_u16;
_8 = 14642724117561981766_u64 as i128;
_8 = 34967559519432001486329849290803730822_i128;
Goto(bb3)
}
bb8 = {
_7 = 179561326916815127815927010888669196758_u128;
_2 = '\u{ed8fa}';
RET = core::ptr::addr_of!((*RET));
_12 = _8 as i8;
_14 = [_3,_3,_3];
_4 = _6 as i8;
_15 = -(-21_isize);
_24 = _9 as isize;
_26.fld2 = _6;
_26 = Adt21 { fld0: _5,fld1: _3,fld2: _6,fld3: 36316460_u32 };
_15 = _24;
_4 = _11 as i8;
_25 = _26.fld3 % _26.fld3;
_7 = 185587150529324429100243452027009661560_u128;
RET = core::ptr::addr_of!(_22);
_26.fld0 = !_5;
_14 = [_3,_3,_26.fld1];
match _26.fld3 {
0 => bb6,
36316460 => bb10,
_ => bb9
}
}
bb9 = {
_7 = 298269414010920076891972027200380954658_u128;
_12 = _4;
_9 = 2390955334029859627_usize * 1_usize;
_7 = !165921927072045436302632004540300089303_u128;
_8 = 18272455004961761628_u64 as i128;
_5 = 24253_i16;
_7 = 220672732877857178868208296069183433064_u128 ^ 206255479825611947634835701694136485473_u128;
_11 = _1;
_8 = _10 as i128;
_6 = (-1119400369_i32);
_10 = 3479760960_u32 as u8;
_2 = '\u{d89d5}';
_3 = _11 as u16;
_4 = _12 << _6;
_2 = '\u{2ccaf}';
_9 = 3_usize;
_16 = [_7];
_3 = !59222_u16;
_1 = _2 != _2;
_3 = 16610_u16;
_8 = 14642724117561981766_u64 as i128;
_8 = 34967559519432001486329849290803730822_i128;
Goto(bb3)
}
bb10 = {
_31.3 = _15;
_30.1.0.0 = &_26.fld3;
(*RET) = [_10,(*_19)];
_15 = _26.fld3 as isize;
_31.0 = core::ptr::addr_of_mut!(_30.1.0.2);
_10 = _15 as u8;
_7 = 247038979718407171868092921169702952685_u128;
Call((*RET) = fn1(Move(_30.1.0.0), _26.fld1, _4, _15, Move(RET), _8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_20 = _15 + _15;
_25 = !_26.fld3;
_31.2.2 = _25 - _26.fld3;
_28 = _4;
_30.2.0 = (_26.fld1, _26.fld2, _25);
_10 = 105_u8 ^ 1_u8;
_29 = _4 as f32;
_30.1.0.1 = _7;
_31.2 = _30.2.0;
_34 = _10;
_21 = -_15;
Goto(bb12)
}
bb12 = {
_31.3 = _20;
_30.1.0.3 = _2;
_30.2.0.1 = _31.2.2 as i32;
_30.2.0.0 = _31.2.0;
_31.1 = _26.fld3 >= _25;
_26.fld3 = _31.2.2 & _25;
_24 = _8 as isize;
_23.0.0 = &_30.2.0.2;
_31.2.2 = _30.2.0.2;
_16 = [_30.1.0.1];
_3 = _26.fld1 + _30.2.0.0;
_23.0.1 = !_30.1.0.1;
_23.0.3 = _30.1.0.3;
Goto(bb13)
}
bb13 = {
_33 = core::ptr::addr_of!(_16);
_8 = -(-147267922891879708215197113019723153532_i128);
_33 = core::ptr::addr_of!((*_33));
_4 = _28 ^ _17;
_23.0.2 = _9 as f64;
_23.0.3 = _2;
_35 = [_15,_20];
(*_19) = !_34;
_30.2.0 = (_3, _31.2.1, _26.fld3);
_31.0 = core::ptr::addr_of_mut!(_30.1.0.2);
_7 = !_30.1.0.1;
(*_19) = _9 as u8;
_30.2.0 = (_3, _26.fld2, _26.fld3);
RET = core::ptr::addr_of!(_22);
_31.3 = _20 >> _15;
_28 = -_4;
_32.0 = core::ptr::addr_of!(_17);
_32.1 = !_26.fld1;
_19 = core::ptr::addr_of!(_10);
_32.1 = _6 as u16;
_38 = _30.1.0.3;
_37 = _29 as isize;
_30.1.0.0 = &_25;
_31.3 = _21 & _37;
_30.2.0.1 = _31.2.1;
_31.2.1 = !_30.2.0.1;
_30.1.0.0 = &_25;
_25 = !_26.fld3;
match _9 {
0 => bb3,
11871963003684610401 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_39 = Adt17::Variant0 { fld0: _31.1,fld1: _30.1.0.3,fld2: _30.2.0.0,fld3: _25,fld4: (*_19),fld5: _30.1.0.1,fld6: (-3078223449381853627_i64) };
_30.2.0.1 = Field::<u16>(Variant(_39, 0), 2) as i32;
_30.1.0.0 = &place!(Field::<u32>(Variant(_39, 0), 3));
_36 = _23.0.2 + _23.0.2;
_8 = 57546143609349034485946453294295197373_i128;
_6 = -_31.2.1;
(*RET) = [_10,_34];
place!(Field::<u8>(Variant(_39, 0), 4)) = !_10;
_23.0.2 = -_36;
_23.1 = Field::<bool>(Variant(_39, 0), 0);
_30.1.0.2 = _36 + _36;
(*_19) = _26.fld0 as u8;
(*RET) = [_10,Field::<u8>(Variant(_39, 0), 4)];
(*_19) = _34;
(*_19) = _34 ^ _34;
_23.1 = Field::<bool>(Variant(_39, 0), 0);
Goto(bb16)
}
bb16 = {
Call(_45 = dump_var(0_usize, 17_usize, Move(_17), 5_usize, Move(_5), 38_usize, Move(_38), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(0_usize, 28_usize, Move(_28), 9_usize, Move(_9), 12_usize, Move(_12), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(0_usize, 21_usize, Move(_21), 24_usize, Move(_24), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(0_usize, 6_usize, Move(_6), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: &'static u32,mut _2: u16,mut _3: i8,mut _4: isize,mut _5: *const [u8; 2],mut _6: i128) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _7: &'static isize;
let _8: &'static &'static bool;
let _9: &'static &'static bool;
let _10: &'static u32;
let _11: isize;
let _12: (u16, ((&'static u32, u128, f64, char), bool), ((u16, i32, u32),));
let _13: [i128; 8];
let _14: [i128; 8];
let _15: (i8,);
let _16: *const u128;
let _17: f32;
let _18: i8;
let _19: &'static Adt17;
let _20: &'static Adt65;
let _21: isize;
let _22: isize;
let _23: &'static [u128; 1];
let _24: *mut f64;
let _25: *const [u128; 1];
let _26: isize;
let _27: isize;
let _28: i128;
let _29: i32;
let _30: f64;
let _31: [u8; 2];
let _32: isize;
let _33: i16;
let _34: f32;
let _35: [u128; 1];
let _36: *const [u128; 1];
let _37: f64;
let _38: u64;
let _39: isize;
let _40: i16;
let _41: Adt48;
let _42: *const u8;
let _43: [i64; 7];
let _44: [isize; 2];
let _45: &'static Adt65;
let _46: ();
let _47: ();
{
_6 = !57830244123125066842050390422399718817_i128;
RET = [150_u8,60_u8];
_3 = _6 as i8;
_2 = 0_usize as u16;
_3 = (-27_i8) + (-126_i8);
_7 = &_4;
_2 = !2865_u16;
_5 = core::ptr::addr_of!(RET);
_2 = 714715196_i32 as u16;
Goto(bb1)
}
bb1 = {
RET = [246_u8,84_u8];
_2 = !62791_u16;
_2 = 53588_u16;
_3 = -(-22_i8);
_5 = core::ptr::addr_of!((*_5));
(*_5) = [167_u8,64_u8];
_5 = core::ptr::addr_of!((*_5));
_6 = (-155584699364214605237097825229326358753_i128);
RET = [85_u8,148_u8];
RET = [63_u8,74_u8];
RET = [92_u8,167_u8];
(*_5) = [73_u8,68_u8];
_6 = -96404086574937323150650073454914809846_i128;
RET = [30_u8,49_u8];
(*_5) = [122_u8,152_u8];
_6 = (-39716932553007069407925508931289727772_i128);
_6 = 12169626000000118338926258312276200681_i128;
(*_5) = [241_u8,82_u8];
_4 = 1232414149_u32 as isize;
RET = [87_u8,27_u8];
RET = [82_u8,114_u8];
RET = [122_u8,76_u8];
_7 = &_4;
_4 = -9223372036854775807_isize;
_6 = 63051857490490877499599941587975246443_i128;
_3 = (-11_i8) & (-81_i8);
Call((*_5) = fn2(_3, Move(_5), _4, _3, _6, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = 1255_i16 as isize;
_3 = !(-119_i8);
_1 = &_12.2.0.2;
_12.2.0.1 = (-1735613940_i32) + (-491219692_i32);
_12.1.0.1 = !102387455989335065081710190452180833868_u128;
_2 = 12436_u16 >> _12.2.0.1;
_12.2.0.2 = 2764177230_u32;
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_12.1.1 = false;
RET = [209_u8,191_u8];
_5 = core::ptr::addr_of!(RET);
_18 = (-5281_i16) as i8;
_3 = 157_u8 as i8;
_1 = &_12.2.0.2;
_10 = &_12.2.0.2;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_3 = _18 ^ _18;
_7 = &_11;
_17 = _12.1.0.1 as f32;
_18 = _12.1.1 as i8;
_1 = Move(_10);
Goto(bb3)
}
bb3 = {
_12.0 = _2;
_12.2.0 = (_2, 2047760038_i32, 1631063918_u32);
_16 = core::ptr::addr_of!(_12.1.0.1);
_14 = _13;
_16 = core::ptr::addr_of!((*_16));
_12.2.0.0 = 216_u8 as u16;
_12.1.0.2 = _12.2.0.0 as f64;
(*_16) = 34376960008647905282951163792310891403_u128 | 330428402197437457408324004755745640064_u128;
_22 = (*_7) * (*_7);
_10 = &_12.2.0.2;
_2 = _12.0;
_21 = _22 * (*_7);
_13 = _14;
_10 = &(*_10);
_4 = _21 + (*_7);
_2 = _12.0;
_14 = _13;
_16 = core::ptr::addr_of!((*_16));
RET = [142_u8,204_u8];
_5 = core::ptr::addr_of!(RET);
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_16 = core::ptr::addr_of!((*_16));
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
(*_16) = 158278239297922170791906375674840419928_u128;
_7 = &_21;
_12.2.0.0 = !_2;
Goto(bb4)
}
bb4 = {
_24 = core::ptr::addr_of_mut!(_12.1.0.2);
(*_5) = [236_u8,12_u8];
_18 = 0_usize as i8;
(*_24) = _12.1.0.1 as f64;
_12.2.0 = (_12.0, (-1211758590_i32), 919294992_u32);
_12.1.0.0 = &_12.2.0.2;
_15 = (_18,);
Goto(bb5)
}
bb5 = {
(*_24) = _12.2.0.2 as f64;
_24 = core::ptr::addr_of_mut!(_30);
_17 = 1_usize as f32;
_21 = !_4;
_12.2.0 = (_2, (-1219285278_i32), 2966923195_u32);
_17 = 17652426476984292037_u64 as f32;
_12.2.0 = (_2, (-1079159912_i32), 1745481945_u32);
_12.2.0 = (_12.0, 1046825249_i32, 1489526305_u32);
match _12.2.0.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
1489526305 => bb10,
_ => bb9
}
}
bb6 = {
_24 = core::ptr::addr_of_mut!(_12.1.0.2);
(*_5) = [236_u8,12_u8];
_18 = 0_usize as i8;
(*_24) = _12.1.0.1 as f64;
_12.2.0 = (_12.0, (-1211758590_i32), 919294992_u32);
_12.1.0.0 = &_12.2.0.2;
_15 = (_18,);
Goto(bb5)
}
bb7 = {
_12.0 = _2;
_12.2.0 = (_2, 2047760038_i32, 1631063918_u32);
_16 = core::ptr::addr_of!(_12.1.0.1);
_14 = _13;
_16 = core::ptr::addr_of!((*_16));
_12.2.0.0 = 216_u8 as u16;
_12.1.0.2 = _12.2.0.0 as f64;
(*_16) = 34376960008647905282951163792310891403_u128 | 330428402197437457408324004755745640064_u128;
_22 = (*_7) * (*_7);
_10 = &_12.2.0.2;
_2 = _12.0;
_21 = _22 * (*_7);
_13 = _14;
_10 = &(*_10);
_4 = _21 + (*_7);
_2 = _12.0;
_14 = _13;
_16 = core::ptr::addr_of!((*_16));
RET = [142_u8,204_u8];
_5 = core::ptr::addr_of!(RET);
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_16 = core::ptr::addr_of!((*_16));
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
(*_16) = 158278239297922170791906375674840419928_u128;
_7 = &_21;
_12.2.0.0 = !_2;
Goto(bb4)
}
bb8 = {
_11 = 1255_i16 as isize;
_3 = !(-119_i8);
_1 = &_12.2.0.2;
_12.2.0.1 = (-1735613940_i32) + (-491219692_i32);
_12.1.0.1 = !102387455989335065081710190452180833868_u128;
_2 = 12436_u16 >> _12.2.0.1;
_12.2.0.2 = 2764177230_u32;
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_12.1.1 = false;
RET = [209_u8,191_u8];
_5 = core::ptr::addr_of!(RET);
_18 = (-5281_i16) as i8;
_3 = 157_u8 as i8;
_1 = &_12.2.0.2;
_10 = &_12.2.0.2;
_14 = [_6,_6,_6,_6,_6,_6,_6,_6];
_3 = _18 ^ _18;
_7 = &_11;
_17 = _12.1.0.1 as f32;
_18 = _12.1.1 as i8;
_1 = Move(_10);
Goto(bb3)
}
bb9 = {
RET = [246_u8,84_u8];
_2 = !62791_u16;
_2 = 53588_u16;
_3 = -(-22_i8);
_5 = core::ptr::addr_of!((*_5));
(*_5) = [167_u8,64_u8];
_5 = core::ptr::addr_of!((*_5));
_6 = (-155584699364214605237097825229326358753_i128);
RET = [85_u8,148_u8];
RET = [63_u8,74_u8];
RET = [92_u8,167_u8];
(*_5) = [73_u8,68_u8];
_6 = -96404086574937323150650073454914809846_i128;
RET = [30_u8,49_u8];
(*_5) = [122_u8,152_u8];
_6 = (-39716932553007069407925508931289727772_i128);
_6 = 12169626000000118338926258312276200681_i128;
(*_5) = [241_u8,82_u8];
_4 = 1232414149_u32 as isize;
RET = [87_u8,27_u8];
RET = [82_u8,114_u8];
RET = [122_u8,76_u8];
_7 = &_4;
_4 = -9223372036854775807_isize;
_6 = 63051857490490877499599941587975246443_i128;
_3 = (-11_i8) & (-81_i8);
Call((*_5) = fn2(_3, Move(_5), _4, _3, _6, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_21 = _11 + _22;
(*_24) = _12.1.0.2;
_4 = (*_16) as isize;
_3 = _15.0;
_12.1.0.2 = (*_24);
(*_24) = -_12.1.0.2;
_7 = &_11;
Call((*_5) = fn5(Move(_7), _12.2.0.2, _12.1.0.2, _12.2.0.1, _12.2.0.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.0 = _3 >> (*_16);
_15 = (_18,);
_28 = _6;
_12.1.1 = !true;
_12.2.0.1 = 138_u8 as i32;
_16 = core::ptr::addr_of!((*_16));
_11 = _22;
_21 = _11;
_15.0 = _18;
_33 = !10386_i16;
_1 = &_12.2.0.2;
_15 = (_3,);
_28 = -_6;
_5 = core::ptr::addr_of!(_31);
_12.2.0.2 = !3229613227_u32;
_12.2.0.2 = !586902784_u32;
(*_24) = _12.1.0.2;
_12.1.0.3 = '\u{7f63f}';
_12.2.0 = (_12.0, 595236837_i32, 3710470949_u32);
_29 = !_12.2.0.1;
(*_24) = _12.1.0.2;
_27 = _22 + _21;
(*_16) = 157727015272319021718090427339262237799_u128 ^ 94417710255177177140126528772241694291_u128;
Goto(bb12)
}
bb12 = {
_25 = core::ptr::addr_of!(_35);
(*_5) = RET;
_24 = core::ptr::addr_of_mut!(_30);
_12.2.0.2 = 4209523721_u32;
(*_25) = [(*_16)];
_15.0 = (*_16) as i8;
_10 = &_12.2.0.2;
_12.1.1 = true;
_5 = core::ptr::addr_of!((*_5));
_11 = -_27;
_16 = core::ptr::addr_of!((*_16));
_37 = _30 * _30;
_35 = [(*_16)];
RET = _31;
_12.1.0.0 = &_12.2.0.2;
_22 = !_27;
_21 = _11 ^ _27;
_1 = Move(_12.1.0.0);
_12.1.0.3 = '\u{205c6}';
_40 = -_33;
_12.1.0.3 = '\u{73699}';
match _12.2.0.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb11,
4 => bb10,
595236837 => bb13,
_ => bb6
}
}
bb13 = {
_5 = core::ptr::addr_of!((*_5));
_12.1.0.0 = &(*_10);
(*_16) = 121303305361533667891866561146489120985_u128;
_1 = Move(_12.1.0.0);
_25 = core::ptr::addr_of!((*_25));
_23 = &(*_25);
_21 = -_27;
_7 = &_27;
_12.2.0.0 = _12.0;
_36 = core::ptr::addr_of!((*_25));
_41 = Adt48::Variant2 { fld0: _12.2.0.1,fld1: (*_24),fld2: (-1895838218928401201_i64),fld3: (*_16),fld4: _40 };
_12.1.0.0 = Move(_10);
RET = [6_u8,249_u8];
_38 = 15481889659664479169_u64;
(*_25) = [_12.1.0.1];
_10 = &_12.2.0.2;
_3 = -_15.0;
_24 = core::ptr::addr_of_mut!((*_24));
_12.1.0.2 = _3 as f64;
_36 = Move(_25);
_13 = [_28,_6,_28,_28,_6,_28,_28,_28];
_34 = (-8486084481413543408_i64) as f32;
_12.1.0 = (Move(_10), Field::<u128>(Variant(_41, 2), 3), (*_24), '\u{4fd07}');
(*_16) = Field::<u128>(Variant(_41, 2), 3);
_34 = 113_u8 as f32;
_4 = -_21;
_14 = [_6,_28,_6,_6,_6,_6,_6,_28];
_24 = core::ptr::addr_of_mut!(place!(Field::<f64>(Variant(_41, 2), 1)));
Goto(bb14)
}
bb14 = {
_24 = core::ptr::addr_of_mut!((*_24));
_39 = _22 | _11;
_38 = !17070337240105789300_u64;
_32 = (*_7);
RET = [46_u8,174_u8];
_14 = [_6,_6,_28,_6,_6,_6,_28,_28];
_43 = [8274225601043617183_i64,(-1165994530772850071_i64),(-3225445460568682574_i64),7519533369278348083_i64,(-2866173831918980922_i64),2575741507822497230_i64,(-7487605177500775133_i64)];
_12.1.0.0 = &_12.2.0.2;
_40 = _33 - Field::<i16>(Variant(_41, 2), 4);
_2 = !_12.2.0.0;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(1_usize, 40_usize, Move(_40), 11_usize, Move(_11), 4_usize, Move(_4), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(1_usize, 2_usize, Move(_2), 22_usize, Move(_22), 33_usize, Move(_33), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(1_usize, 21_usize, Move(_21), 6_usize, Move(_6), 15_usize, Move(_15), 47_usize, _47), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i8,mut _2: *const [u8; 2],mut _3: isize,mut _4: i8,mut _5: i128,mut _6: i8) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _7: i8;
let _8: f64;
let _9: i16;
let _10: &'static u32;
let _11: i64;
let _12: isize;
let _13: ([i8; 4],);
let _14: ([bool; 7], [i32; 2]);
let _15: bool;
let _16: &'static u32;
let _17: f64;
let _18: i64;
let _19: char;
let _20: u64;
let _21: char;
let _22: isize;
let _23: [u64; 1];
let _24: u64;
let _25: ();
let _26: ();
{
RET = [158_u8,152_u8];
_3 = 2382884618134396074_u64 as isize;
_6 = _1 >> _4;
_6 = _4;
_6 = _3 as i8;
RET = [148_u8,22_u8];
RET = [17_u8,192_u8];
_3 = 9223372036854775807_isize + (-9223372036854775808_isize);
_1 = _4;
_7 = 2350166780_u32 as i8;
RET = [202_u8,45_u8];
_3 = 97_isize;
_7 = _1 + _1;
_4 = _7 + _7;
_8 = (-8747884443016115435_i64) as f64;
RET = [77_u8,40_u8];
_6 = _1 - _7;
_7 = _6 >> _4;
_1 = _4;
RET = [109_u8,93_u8];
_5 = -90682326982417360171740517585408961196_i128;
_7 = _1 & _1;
_9 = 2102394763_i32 as i16;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
97 => bb5,
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
_9 = -30254_i16;
Call(_11 = fn3(_6, _7, Move(_2), _1, _4, _3, _7, _1, _4, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_11 = 2979149255468408616_i64 ^ (-4782274687425636821_i64);
_8 = 5889_u16 as f64;
_9 = true as i16;
_2 = core::ptr::addr_of!(RET);
RET = [184_u8,88_u8];
_2 = core::ptr::addr_of!((*_2));
_7 = _6 * _1;
_9 = 7765_i16 >> _7;
_14.1 = [(-143632080_i32),991747630_i32];
(*_2) = [239_u8,159_u8];
_5 = -(-129800308623407584343575147625697976403_i128);
_14.0 = [true,false,true,true,false,true,false];
Goto(bb7)
}
bb7 = {
RET = [245_u8,7_u8];
_11 = (-6407425285781082495_i64) & 3958245614746870830_i64;
(*_2) = [96_u8,235_u8];
_12 = !_3;
_14.0 = [false,true,true,false,true,true,true];
_6 = 155_u8 as i8;
_13.0 = [_7,_7,_4,_7];
Goto(bb8)
}
bb8 = {
_11 = 3011583598413711120_i64 << _7;
_11 = !1478493281527922766_i64;
RET = [33_u8,71_u8];
_15 = true | true;
_4 = !_7;
match _3 {
0 => bb1,
97 => bb9,
_ => bb7
}
}
bb9 = {
_11 = 4682678448517859293_i64 ^ (-2663819576040122393_i64);
_9 = 3570_i16;
Call(_1 = fn4(_4, _13.0, _13, _7, _13, _13.0, Move(_2), _14.1, _4, _13), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_1 = _7;
_6 = _3 as i8;
_7 = _4;
_17 = -_8;
_17 = -_8;
_20 = '\u{9b1a8}' as u64;
_6 = !_1;
_18 = _3 as i64;
_20 = 557956689891943145_u64 * 6565917252848023722_u64;
_19 = '\u{3b3b2}';
Goto(bb11)
}
bb11 = {
_15 = true;
_15 = true;
_18 = _11 >> _9;
_1 = _6;
_3 = !_12;
_9 = (-14206_i16) & (-9683_i16);
_5 = 119521062734965947608759483658810063282_i128;
_18 = _11 & _11;
_2 = core::ptr::addr_of!(RET);
_14.1 = [(-1553808128_i32),1622869444_i32];
_14.1 = [1979350076_i32,1309229675_i32];
RET = [87_u8,211_u8];
_17 = (-1423749471_i32) as f64;
_11 = _18;
_6 = _1 - _7;
_14.1 = [(-1754871012_i32),864773549_i32];
_23 = [_20];
_13.0 = [_4,_6,_1,_6];
_19 = '\u{9ab5d}';
_20 = 3265182482399356574_u64;
(*_2) = [72_u8,201_u8];
match _20 {
0 => bb12,
3265182482399356574 => bb14,
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
_21 = _19;
_1 = !_4;
_12 = _3 - _3;
(*_2) = [238_u8,123_u8];
_18 = 1709065778_u32 as i64;
_2 = core::ptr::addr_of!((*_2));
_9 = _5 as i16;
_21 = _19;
_18 = _11;
_21 = _19;
_5 = (-126624109328858378915009451650650450375_i128);
_7 = 197_u8 as i8;
_18 = -_11;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(2_usize, 18_usize, Move(_18), 13_usize, Move(_13), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(2_usize, 12_usize, Move(_12), 1_usize, Move(_1), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i8,mut _2: i8,mut _3: *const [u8; 2],mut _4: i8,mut _5: i8,mut _6: isize,mut _7: i8,mut _8: i8,mut _9: i8,mut _10: i8) -> i64 {
mir! {
type RET = i64;
let _11: f64;
let _12: [i8; 4];
let _13: i8;
let _14: i64;
let _15: *const u8;
let _16: f32;
let _17: [usize; 5];
let _18: &'static [u128; 1];
let _19: Adt77;
let _20: (*const i8, u16, Adt32, &'static &'static bool);
let _21: &'static f32;
let _22: u16;
let _23: [u8; 2];
let _24: bool;
let _25: isize;
let _26: ();
let _27: ();
{
RET = _1 as i64;
_1 = !_8;
_4 = _7;
_5 = _8;
_7 = 2882_u16 as i8;
_11 = 19522_u16 as f64;
_6 = 52920_u16 as isize;
_10 = _5;
_10 = _5;
_10 = _1 >> _9;
_7 = !_8;
_7 = _8;
_2 = _9 ^ _4;
_8 = _2;
_12 = [_9,_10,_9,_2];
_7 = -_4;
_4 = _9;
_13 = _10 & _8;
_13 = _10;
_5 = _8 + _8;
_6 = 9223372036854775807_isize;
_11 = 211_u8 as f64;
Goto(bb1)
}
bb1 = {
RET = (-897817096223396854_i64);
_11 = 24225_i16 as f64;
_7 = _8;
_12 = [_5,_9,_8,_2];
RET = 87_u8 as i64;
_5 = !_7;
_10 = _2 + _7;
_4 = _13 << _10;
_11 = 4482499129726529665_usize as f64;
_16 = 5464_u16 as f32;
_8 = _6 as i8;
RET = (-67355388_i32) as i64;
_14 = !RET;
_5 = _10;
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
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
_17 = [4_usize,0_usize,4_usize,3_usize,1_usize];
_1 = _10 + _4;
_8 = !_13;
_10 = -_1;
_7 = _2;
_16 = 1218239547_i32 as f32;
_8 = _4;
_19.fld4.0 = core::ptr::addr_of_mut!(_11);
_19.fld1 = '\u{c5a3a}';
Goto(bb7)
}
bb7 = {
_19.fld4.2.1 = 291198576_i32 | 1344787742_i32;
_19.fld4.3 = !_6;
_7 = -_5;
_7 = 47258_u16 as i8;
_19.fld1 = '\u{3b084}';
_11 = (-65975392590372265950794448108217020497_i128) as f64;
_20.1 = 62635_u16;
_19.fld1 = '\u{13cfd}';
_19.fld1 = '\u{8e252}';
_20.0 = core::ptr::addr_of!(_13);
_7 = -_5;
_19.fld4.2.2 = 2633856993_u32;
_19.fld0 = [_14,_14,RET,_14,RET,_14,_14];
_11 = 2339_i16 as f64;
_19.fld4.1 = true;
_21 = &_16;
_10 = _1 | _4;
_13 = _16 as i8;
_13 = _10 << _10;
_19.fld4.2.0 = _20.1;
match _20.1 {
0 => bb5,
1 => bb6,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
62635 => bb14,
_ => bb13
}
}
bb8 = {
_17 = [4_usize,0_usize,4_usize,3_usize,1_usize];
_1 = _10 + _4;
_8 = !_13;
_10 = -_1;
_7 = _2;
_16 = 1218239547_i32 as f32;
_8 = _4;
_19.fld4.0 = core::ptr::addr_of_mut!(_11);
_19.fld1 = '\u{c5a3a}';
Goto(bb7)
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
RET = (-897817096223396854_i64);
_11 = 24225_i16 as f64;
_7 = _8;
_12 = [_5,_9,_8,_2];
RET = 87_u8 as i64;
_5 = !_7;
_10 = _2 + _7;
_4 = _13 << _10;
_11 = 4482499129726529665_usize as f64;
_16 = 5464_u16 as f32;
_8 = _6 as i8;
RET = (-67355388_i32) as i64;
_14 = !RET;
_5 = _10;
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb14 = {
_19.fld4.2 = (_20.1, 225176727_i32, 2493827726_u32);
_21 = &_16;
_23 = [175_u8,74_u8];
_17 = [12667150730756748785_usize,10359670554157448384_usize,1_usize,0_usize,2_usize];
_6 = _19.fld4.3 >> _4;
_5 = _1;
_3 = core::ptr::addr_of!(_23);
_19.fld2 = !_6;
_20.1 = !_19.fld4.2.0;
_9 = !_13;
_4 = _13 ^ _10;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(3_usize, 10_usize, Move(_10), 2_usize, Move(_2), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(3_usize, 9_usize, Move(_9), 7_usize, Move(_7), 8_usize, Move(_8), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i8,mut _2: [i8; 4],mut _3: ([i8; 4],),mut _4: i8,mut _5: ([i8; 4],),mut _6: [i8; 4],mut _7: *const [u8; 2],mut _8: [i32; 2],mut _9: i8,mut _10: ([i8; 4],)) -> i8 {
mir! {
type RET = i8;
let _11: char;
let _12: &'static [i8; 4];
let _13: [i32; 2];
let _14: ();
let _15: ();
{
_3.0 = [_4,_9,_9,_4];
RET = 2_usize as i8;
_5 = (_6,);
_4 = _1 | _9;
_8 = [754221962_i32,1856207202_i32];
_6 = [_1,_4,_4,_4];
_5 = (_6,);
RET = _1;
_8 = [97749480_i32,905685241_i32];
_2 = [_9,_1,_1,_1];
_5 = (_6,);
_10.0 = [_4,_4,_4,_1];
_10.0 = [_4,_4,_4,_1];
_11 = '\u{10a713}';
_3.0 = [RET,_4,_9,_9];
RET = _4 ^ _4;
RET = _9 + _4;
_10 = (_2,);
_4 = RET + RET;
_6 = [_4,_4,RET,_4];
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(4_usize, 5_usize, Move(_5), 2_usize, Move(_2), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(4_usize, 9_usize, Move(_9), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: &'static isize,mut _2: u32,mut _3: f64,mut _4: i32,mut _5: u32) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _6: isize;
let _7: &'static f32;
let _8: ([bool; 7], [i32; 2]);
let _9: (&'static i16, ([u8; 2],));
let _10: &'static [i32; 2];
let _11: u64;
let _12: char;
let _13: u8;
let _14: bool;
let _15: isize;
let _16: (u16, i32, u32);
let _17: [i16; 1];
let _18: f64;
let _19: *const [u8; 2];
let _20: isize;
let _21: [i8; 4];
let _22: &'static [i8; 4];
let _23: Adt17;
let _24: isize;
let _25: isize;
let _26: ((u16, i32, u32),);
let _27: i32;
let _28: (&'static u32, u128, f64, char);
let _29: [i8; 4];
let _30: (u32,);
let _31: ((u16, ((&'static u32, u128, f64, char), bool), ((u16, i32, u32),)), (u32,), u64);
let _32: ();
let _33: ();
{
RET = [95_u8,40_u8];
_4 = !947656031_i32;
_3 = 317805589155508657602681793733598867797_u128 as f64;
_4 = 9543030261965715789_u64 as i32;
_2 = _5;
RET = [43_u8,223_u8];
_6 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
RET = [66_u8,0_u8];
_6 = (-9223372036854775808_isize) << _5;
_1 = &_6;
_2 = (-31707256714982322006352071739486550828_i128) as u32;
RET = [164_u8,10_u8];
RET = [86_u8,149_u8];
RET = [162_u8,207_u8];
RET = [239_u8,166_u8];
_8.1 = [_4,_4];
_8.0 = [true,false,false,true,true,false,false];
_3 = 2794713132763541293_i64 as f64;
_9.1 = (RET,);
_10 = &_8.1;
_8.1 = [_4,_4];
_2 = 8334881678260001794_u64 as u32;
_8.1 = [_4,_4];
Goto(bb1)
}
bb1 = {
_3 = (*_1) as f64;
_4 = 220105839_i32;
Goto(bb2)
}
bb2 = {
RET = _9.1.0;
_9.1.0 = [179_u8,157_u8];
_13 = (-3356733280104699659_i64) as u8;
_13 = (-93065242929919946901090406365587548671_i128) as u8;
_4 = _2 as i32;
_9.1 = (RET,);
_10 = &_8.1;
_1 = &(*_1);
_11 = 2324239542369477108_u64 + 16522474418244410623_u64;
_16.2 = _5 & _2;
Call(_13 = core::intrinsics::bswap(199_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.1.0 = [_13,_13];
_6 = -9223372036854775807_isize;
_12 = '\u{29eed}';
_11 = _13 as u64;
_16.1 = _4;
_10 = &_8.1;
match _5 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
1489526305 => bb11,
_ => bb10
}
}
bb4 = {
RET = _9.1.0;
_9.1.0 = [179_u8,157_u8];
_13 = (-3356733280104699659_i64) as u8;
_13 = (-93065242929919946901090406365587548671_i128) as u8;
_4 = _2 as i32;
_9.1 = (RET,);
_10 = &_8.1;
_1 = &(*_1);
_11 = 2324239542369477108_u64 + 16522474418244410623_u64;
_16.2 = _5 & _2;
Call(_13 = core::intrinsics::bswap(199_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3 = (*_1) as f64;
_4 = 220105839_i32;
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
Return()
}
bb11 = {
_1 = &_6;
RET = [_13,_13];
_23 = Adt17::Variant0 { fld0: false,fld1: _12,fld2: 36576_u16,fld3: _16.2,fld4: _13,fld5: 56429215891405857147483533246877025965_u128,fld6: 3563220697936683928_i64 };
_19 = core::ptr::addr_of!(RET);
(*_19) = _9.1.0;
_13 = Field::<u8>(Variant(_23, 0), 4) + Field::<u8>(Variant(_23, 0), 4);
_5 = Field::<u32>(Variant(_23, 0), 3);
_9.1.0 = [Field::<u8>(Variant(_23, 0), 4),Field::<u8>(Variant(_23, 0), 4)];
(*_19) = [_13,Field::<u8>(Variant(_23, 0), 4)];
_13 = 117_i8 as u8;
Goto(bb12)
}
bb12 = {
place!(Field::<bool>(Variant(_23, 0), 0)) = !true;
place!(Field::<u8>(Variant(_23, 0), 4)) = 1_usize as u8;
_26.0.1 = -_4;
_16.0 = (-11852_i16) as u16;
_1 = &_24;
place!(Field::<i64>(Variant(_23, 0), 6)) = _11 as i64;
_6 = (-9223372036854775808_isize);
Call(_3 = fn6(_2, _8, (*_19), _6, Field::<bool>(Variant(_23, 0), 0), _16.2, _8, _8, RET), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16.1 = 17050993713401619633_usize as i32;
place!(Field::<u8>(Variant(_23, 0), 4)) = !_13;
_1 = &(*_1);
match _6 {
0 => bb11,
1 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
}
}
bb14 = {
RET = _9.1.0;
_9.1.0 = [179_u8,157_u8];
_13 = (-3356733280104699659_i64) as u8;
_13 = (-93065242929919946901090406365587548671_i128) as u8;
_4 = _2 as i32;
_9.1 = (RET,);
_10 = &_8.1;
_1 = &(*_1);
_11 = 2324239542369477108_u64 + 16522474418244410623_u64;
_16.2 = _5 & _2;
Call(_13 = core::intrinsics::bswap(199_u8), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_26 = (_16,);
_28.1 = 279414800105971537524538798058191676155_u128 >> _26.0.2;
_27 = _28.1 as i32;
_25 = _27 as isize;
_30.0 = !_26.0.2;
RET = _9.1.0;
_21 = [(-42_i8),(-52_i8),(-46_i8),104_i8];
_30.0 = _3 as u32;
_31.2 = _11;
place!(Field::<i64>(Variant(_23, 0), 6)) = -4195968361372792518_i64;
place!(Field::<u32>(Variant(_23, 0), 3)) = _16.2 + _30.0;
_14 = Field::<bool>(Variant(_23, 0), 0);
_4 = -_27;
_31.0.1.1 = _25 < _25;
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(5_usize, 6_usize, Move(_6), 12_usize, Move(_12), 21_usize, Move(_21), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(5_usize, 8_usize, Move(_8), 27_usize, Move(_27), 26_usize, Move(_26), 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u32,mut _2: ([bool; 7], [i32; 2]),mut _3: [u8; 2],mut _4: isize,mut _5: bool,mut _6: u32,mut _7: ([bool; 7], [i32; 2]),mut _8: ([bool; 7], [i32; 2]),mut _9: [u8; 2]) -> f64 {
mir! {
type RET = f64;
let _10: usize;
let _11: Adt76;
let _12: f32;
let _13: i16;
let _14: &'static isize;
let _15: ();
let _16: ();
{
_7.0 = [_5,_5,_5,_5,_5,_5,_5];
_8.0 = [_5,_5,_5,_5,_5,_5,_5];
_2 = _8;
_1 = 2595191068350664722_u64 as u32;
_5 = !false;
_1 = _6 | _6;
_11 = Adt76::Variant1 { fld0: '\u{26fdd}' };
_2.1 = [(-1960988979_i32),(-221432552_i32)];
_7 = (_8.0, _8.1);
_9 = _3;
_10 = !12343218548214199599_usize;
_3 = _9;
place!(Field::<char>(Variant(_11, 1), 0)) = '\u{da7b4}';
_2 = (_7.0, _8.1);
RET = 939306073_i32 as f64;
_6 = _1 & _1;
_3 = _9;
_12 = (-15205_i16) as f32;
_2 = _7;
place!(Field::<char>(Variant(_11, 1), 0)) = '\u{830d8}';
_8.0 = [_5,_5,_5,_5,_5,_5,_5];
_2.0 = _7.0;
_10 = _4 as usize;
_8 = (_7.0, _7.1);
_5 = false;
_2.0 = _7.0;
_9 = [230_u8,140_u8];
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463454151235394913435648 => bb5,
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
_1 = !_6;
_2 = _8;
_9 = [235_u8,225_u8];
SetDiscriminant(_11, 1);
_8 = (_2.0, _7.1);
match _4 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
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
Return()
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_2 = (_7.0, _8.1);
_4 = 72_isize;
_13 = 9789_i16 - (-19307_i16);
_5 = !true;
_13 = 16232_i16;
_10 = _4 as usize;
_11 = Adt76::Variant1 { fld0: '\u{aa07a}' };
_6 = _1;
place!(Field::<char>(Variant(_11, 1), 0)) = '\u{d479d}';
_8.0 = [_5,_5,_5,_5,_5,_5,_5];
_3 = [3_u8,217_u8];
place!(Field::<char>(Variant(_11, 1), 0)) = '\u{100ad1}';
_14 = &_4;
_6 = _1 * _1;
_12 = 96_u8 as f32;
RET = _10 as f64;
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(6_usize, 3_usize, Move(_3), 4_usize, Move(_4), 13_usize, Move(_13), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_15 = dump_var(6_usize, 5_usize, Move(_5), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{3e967}'), std::hint::black_box(48202_u16), std::hint::black_box((-119_i8)), std::hint::black_box((-6753_i16)), std::hint::black_box(448666361_i32), std::hint::black_box(174905768397249507394233565159816485783_u128), std::hint::black_box(166562298534349275715940794061998325350_i128), std::hint::black_box(3_usize), std::hint::black_box(15_u8));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){unsafe{printf("Adt17::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt17 {
Variant0{
fld0: bool,
fld1: char,
fld2: u16,
fld3: u32,
fld4: u8,
fld5: u128,
fld6: i64,

},
Variant1{
fld0: bool,
fld1: char,
fld2: isize,
fld3: u8,
fld4: i16,
fld5: i32,
fld6: i64,
fld7: f64,

},
Variant2{
fld0: i32,
fld1: u8,
fld2: isize,
fld3: i128,
fld4: u64,

},
Variant3{
fld0: i32,
fld1: char,
fld2: isize,
fld3: u64,

}}
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt21{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt21 {
fld0: i16,
fld1: u16,
fld2: i32,
fld3: u32,
}
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: i16,

},
Variant1{
fld0: u8,
fld1: (u32,),
fld2: u16,
fld3: i8,
fld4: i16,
fld5: usize,

},
Variant2{
fld0: Adt17,

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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: f64,
fld1: Adt17,
fld2: isize,
fld3: f32,
fld4: i16,
fld5: i32,
fld6: Adt24,
fld7: i128,

},
Variant1{
fld0: Adt17,
fld1: *const u128,
fld2: u8,
fld3: i8,
fld4: u64,
fld5: usize,

},
Variant2{
fld0: [i16; 3],
fld1: i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt24,

},
Variant1{
fld0: *const u8,
fld1: char,
fld2: Adt21,
fld3: i128,
fld4: [bool; 7],
fld5: u32,
fld6: ((u16, i32, u32),),

},
Variant2{
fld0: i32,
fld1: f64,
fld2: i64,
fld3: u128,
fld4: i16,

},
Variant3{
fld0: i64,
fld1: u16,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: u64,
fld1: [i8; 4],
fld2: u128,

},
Variant1{
fld0: [u32; 2],
fld1: u128,
fld2: [u128; 1],

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){unsafe{printf("Adt76::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt76 {
Variant0{
fld0: *const [u128; 1],
fld1: char,
fld2: u128,
fld3: usize,
fld4: f64,

},
Variant1{
fld0: char,

},
Variant2{
fld0: [i64; 3],
fld1: (u32,),
fld2: f32,
fld3: [u16; 3],
fld4: *mut f64,

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt77{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt77 {
fld0: [i64; 7],
fld1: char,
fld2: isize,
fld3: [i128; 8],
fld4: (*mut f64, bool, (u16, i32, u32), isize),
}

