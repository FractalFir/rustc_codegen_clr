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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u64,mut _4: i8,mut _5: i16,mut _6: u32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16) -> bool {
mir! {
type RET = bool;
let _12: bool;
let _13: f64;
let _14: (*mut (((u32,),), *mut *mut &'static i32), u16, &'static *mut i128);
let _15: f32;
let _16: i8;
let _17: ((u32,),);
let _18: char;
let _19: f64;
let _20: f32;
let _21: (u32,);
let _22: i128;
let _23: *mut usize;
let _24: isize;
let _25: f32;
let _26: char;
let _27: *mut ([u32; 7], [u8; 7], &'static Adt22);
let _28: usize;
let _29: u64;
let _30: char;
let _31: (f64, (u16,));
let _32: (u16,);
let _33: ();
let _34: ();
{
RET = !false;
RET = !true;
_7 = (-2977579452893511693_i64) * (-5560739992091242245_i64);
_1 = !RET;
_5 = (-21388_i16);
_3 = 167467141837396502683898826538063992511_u128 as u64;
_6 = !97756307_u32;
RET = _7 > _7;
_1 = !RET;
Goto(bb1)
}
bb1 = {
_12 = _1 == _1;
_9 = !11547659874151998092_usize;
_7 = 3700909674987273415_i64;
_10 = 9223372036854775807_isize as u8;
_8 = (-64562219671146127417501853883749388585_i128) & (-53313631259192751383827637413206748123_i128);
_8 = (-4106153694938516827316836425364507511_i128) & (-47518131423345797158042657348346180332_i128);
_7 = _5 as i64;
_6 = 1648980959_u32 + 1029895215_u32;
_11 = !40276_u16;
_8 = -(-78752249369776949436989362615861481374_i128);
_10 = !10_u8;
_11 = _5 as u16;
_10 = (-1926581972_i32) as u8;
_17.0.0 = _6;
_15 = _3 as f32;
_8 = -(-87933573419447273838253137361252464031_i128);
_4 = _1 as i8;
_2 = '\u{1a1fb}';
match _5 {
340282366920938463463374607431768190068 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_13 = _9 as f64;
_16 = _9 as i8;
_4 = _16;
_13 = _10 as f64;
RET = _1;
_11 = 22048_u16;
_17.0.0 = _9 as u32;
_2 = '\u{e79f9}';
_8 = -(-109116391549533638314432870603247157124_i128);
_13 = _10 as f64;
_18 = _2;
RET = !_1;
_1 = _12 ^ _12;
_14.1 = !_11;
_17.0 = (_6,);
_1 = _12;
Goto(bb4)
}
bb4 = {
_19 = _13 * _13;
_20 = _15;
_5 = (-27067_i16) ^ 25249_i16;
_14.1 = !_11;
_19 = -_13;
RET = !_1;
_18 = _2;
_1 = !RET;
RET = _3 >= _3;
_3 = !15320029968384567205_u64;
_18 = _2;
_1 = _12;
_21.0 = _3 as u32;
_16 = -_4;
Goto(bb5)
}
bb5 = {
_10 = 251_u8 & 128_u8;
_22 = _3 as i128;
_11 = _14.1;
_12 = _1 < _1;
_21.0 = _17.0.0;
_17.0.0 = _6;
_8 = _22;
_22 = _8;
_13 = _7 as f64;
_8 = _2 as i128;
_12 = _1;
_2 = _18;
_7 = 4291526771720750432_i64 + 1036086062752800544_i64;
_10 = _3 as u8;
_5 = 21722_i16;
_12 = !_1;
_5 = 21514_i16;
_14.1 = _11 * _11;
_20 = _15 - _15;
_22 = _8 | _8;
_15 = 94577219918060932860040155597981973769_u128 as f32;
Call(_24 = fn1(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26 = _18;
_14.1 = _11;
_4 = _16;
_2 = _26;
_10 = _15 as u8;
_6 = _21.0;
_5 = _1 as i16;
_21 = _17.0;
_11 = !_14.1;
_15 = _20 * _20;
_9 = 5_usize | 16363007157480337594_usize;
_21.0 = _17.0.0;
_17.0.0 = _6;
_21.0 = _17.0.0;
_15 = -_20;
_21.0 = !_17.0.0;
_14.1 = !_11;
_15 = _20 + _20;
_12 = _1 ^ _1;
_1 = _12;
_5 = _1 as i16;
_23 = core::ptr::addr_of_mut!(_28);
_22 = -_8;
_2 = _26;
_7 = 394274543981599348_i64;
_7 = !(-2789867177805608102_i64);
_20 = -_15;
_7 = _10 as i64;
_7 = 288012030590725648_i64 << _5;
Call(_12 = fn16(_17.0, _22, _15, _5, _7, _5, _1, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = RET | _12;
_15 = _20;
_17 = (_21,);
_1 = _7 <= _7;
_1 = _11 >= _11;
_21.0 = _6;
_12 = _7 < _7;
_2 = _26;
Call(_25 = core::intrinsics::transmute(_6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = (_21,);
RET = _22 <= _22;
_5 = _26 as i16;
_13 = _19;
_17.0.0 = _6;
_7 = (-1588516813811885782_i64);
_17 = (_21,);
_6 = !_21.0;
_12 = _1;
_9 = 2_usize ^ 15534046316417884717_usize;
_29 = !_3;
_2 = _26;
_12 = RET;
_28 = _9 ^ _9;
_2 = _26;
_26 = _18;
_26 = _18;
RET = _12 | _12;
_18 = _2;
_18 = _2;
_17.0.0 = !_6;
match _7 {
0 => bb7,
1 => bb5,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
340282366920938463461786090617956325674 => bb14,
_ => bb13
}
}
bb9 = {
_12 = _1 == _1;
_9 = !11547659874151998092_usize;
_7 = 3700909674987273415_i64;
_10 = 9223372036854775807_isize as u8;
_8 = (-64562219671146127417501853883749388585_i128) & (-53313631259192751383827637413206748123_i128);
_8 = (-4106153694938516827316836425364507511_i128) & (-47518131423345797158042657348346180332_i128);
_7 = _5 as i64;
_6 = 1648980959_u32 + 1029895215_u32;
_11 = !40276_u16;
_8 = -(-78752249369776949436989362615861481374_i128);
_10 = !10_u8;
_11 = _5 as u16;
_10 = (-1926581972_i32) as u8;
_17.0.0 = _6;
_15 = _3 as f32;
_8 = -(-87933573419447273838253137361252464031_i128);
_4 = _1 as i8;
_2 = '\u{1a1fb}';
match _5 {
340282366920938463463374607431768190068 => bb3,
_ => bb2
}
}
bb10 = {
_26 = _18;
_14.1 = _11;
_4 = _16;
_2 = _26;
_10 = _15 as u8;
_6 = _21.0;
_5 = _1 as i16;
_21 = _17.0;
_11 = !_14.1;
_15 = _20 * _20;
_9 = 5_usize | 16363007157480337594_usize;
_21.0 = _17.0.0;
_17.0.0 = _6;
_21.0 = _17.0.0;
_15 = -_20;
_21.0 = !_17.0.0;
_14.1 = !_11;
_15 = _20 + _20;
_12 = _1 ^ _1;
_1 = _12;
_5 = _1 as i16;
_23 = core::ptr::addr_of_mut!(_28);
_22 = -_8;
_2 = _26;
_7 = 394274543981599348_i64;
_7 = !(-2789867177805608102_i64);
_20 = -_15;
_7 = _10 as i64;
_7 = 288012030590725648_i64 << _5;
Call(_12 = fn16(_17.0, _22, _15, _5, _7, _5, _1, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_10 = 251_u8 & 128_u8;
_22 = _3 as i128;
_11 = _14.1;
_12 = _1 < _1;
_21.0 = _17.0.0;
_17.0.0 = _6;
_8 = _22;
_22 = _8;
_13 = _7 as f64;
_8 = _2 as i128;
_12 = _1;
_2 = _18;
_7 = 4291526771720750432_i64 + 1036086062752800544_i64;
_10 = _3 as u8;
_5 = 21722_i16;
_12 = !_1;
_5 = 21514_i16;
_14.1 = _11 * _11;
_20 = _15 - _15;
_22 = _8 | _8;
_15 = 94577219918060932860040155597981973769_u128 as f32;
Call(_24 = fn1(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_19 = _13 * _13;
_20 = _15;
_5 = (-27067_i16) ^ 25249_i16;
_14.1 = !_11;
_19 = -_13;
RET = !_1;
_18 = _2;
_1 = !RET;
RET = _3 >= _3;
_3 = !15320029968384567205_u64;
_18 = _2;
_1 = _12;
_21.0 = _3 as u32;
_16 = -_4;
Goto(bb5)
}
bb13 = {
_13 = _9 as f64;
_16 = _9 as i8;
_4 = _16;
_13 = _10 as f64;
RET = _1;
_11 = 22048_u16;
_17.0.0 = _9 as u32;
_2 = '\u{e79f9}';
_8 = -(-109116391549533638314432870603247157124_i128);
_13 = _10 as f64;
_18 = _2;
RET = !_1;
_1 = _12 ^ _12;
_14.1 = !_11;
_17.0 = (_6,);
_1 = _12;
Goto(bb4)
}
bb14 = {
_31.0 = _19;
_15 = _20;
_21.0 = _17.0.0 * _6;
RET = _12;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 22_usize, Move(_22), 4_usize, Move(_4), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 26_usize, Move(_26), 1_usize, Move(_1), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 28_usize, Move(_28), 9_usize, Move(_9), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char) -> isize {
mir! {
type RET = isize;
let _2: bool;
let _3: f32;
let _4: Adt68;
let _5: (((u32, (i32,), char),), Adt22, i128, &'static &'static i8);
let _6: u128;
let _7: Adt69;
let _8: f64;
let _9: [u8; 5];
let _10: *mut *mut &'static i32;
let _11: (Adt74,);
let _12: i32;
let _13: i32;
let _14: (u32, (i32,), char);
let _15: isize;
let _16: f32;
let _17: &'static *mut isize;
let _18: ((u32, (i32,), char),);
let _19: bool;
let _20: [u32; 2];
let _21: *mut *mut &'static i32;
let _22: u32;
let _23: u32;
let _24: ();
let _25: ();
{
RET = (-6326303939191920381_i64) as isize;
RET = (-60_isize) << 6_usize;
RET = 293615262507749628793216462245405910295_u128 as isize;
Call(RET = fn2(_1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 24_isize - (-9223372036854775808_isize);
_1 = '\u{c3eb9}';
_2 = true;
_2 = false;
RET = 13382406394079497559_u64 as isize;
_1 = '\u{c4716}';
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_2 = true;
RET = 24_isize;
_1 = '\u{b440f}';
_2 = true | false;
_1 = '\u{d5c99}';
RET = 17268_i16 as isize;
RET = 123_isize + (-9223372036854775808_isize);
_2 = !true;
_2 = true | true;
_3 = 7556312569096233671_u64 as f32;
_1 = '\u{60484}';
_1 = '\u{a6fbf}';
_2 = _3 == _3;
_1 = '\u{8edb8}';
RET = (-8324418820720603587_i64) as isize;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = 62556_u16 as f32;
_1 = '\u{336ab}';
_2 = false;
RET = -9223372036854775807_isize;
_1 = '\u{65de4}';
_1 = '\u{35741}';
RET = !(-34_isize);
_3 = 751581187_u32 as f32;
_1 = '\u{47c40}';
_3 = 0_i8 as f32;
RET = -(-9223372036854775808_isize);
_1 = '\u{c24e9}';
RET = !107_isize;
RET = -(-9223372036854775808_isize);
_2 = true | false;
_1 = '\u{dd79f}';
_3 = 6573817306910536379_usize as f32;
RET = (-9223372036854775808_isize);
RET = -(-63_isize);
_2 = false;
Goto(bb3)
}
bb3 = {
RET = 119164572398693230624523963585267105865_u128 as isize;
_5.0.0.0 = 379956479_u32 + 3111941504_u32;
_6 = 34846959043275086811254248123987878257_u128 * 269832649123942013396521676041172373173_u128;
_5.0.0.2 = _1;
RET = -9223372036854775807_isize;
RET = 25851_u16 as isize;
_5.2 = 30335_i16 as i128;
_5.0.0.1.0 = 38825708_i32 << _5.2;
_5.0.0.1 = ((-904341762_i32),);
RET = (-39_isize) + 22_isize;
_5.0.0.0 = 6954_i16 as u32;
RET = 9223372036854775807_isize;
_8 = _5.0.0.0 as f64;
match RET {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb4 = {
_5.0.0.2 = _1;
_5.0.0.1 = (1523794768_i32,);
_5.0.0.1.0 = 721489343_i32 & (-1954115594_i32);
_5.0.0.1.0 = -378626487_i32;
_5.0.0.2 = _1;
_8 = 103_i8 as f64;
_6 = _5.2 as u128;
_5.0.0.1.0 = (-1839609035_i32) >> RET;
_2 = _5.0.0.1.0 <= _5.0.0.1.0;
_6 = !318864206417010895334432920090652739086_u128;
_9 = [53_u8,55_u8,117_u8,133_u8,187_u8];
_5.0.0.2 = _1;
_5.2 = _3 as i128;
_5.2 = _8 as i128;
_12 = _8 as i32;
_1 = _5.0.0.2;
_3 = _5.2 as f32;
_5.2 = 155618831744650638794793054345405546920_i128 ^ (-104559983143522723881561316955577338096_i128);
RET = -(-9223372036854775808_isize);
_11.0.fld0 = (-27215_i16) ^ 3815_i16;
_13 = !_12;
_5.0.0.1 = (_12,);
_5.0.0.0 = 1043560223_u32;
_5.0.0.1 = (_12,);
Goto(bb5)
}
bb5 = {
_14.0 = !_5.0.0.0;
_9 = [153_u8,56_u8,206_u8,37_u8,3_u8];
RET = !(-9223372036854775808_isize);
_14.0 = !_5.0.0.0;
_8 = _5.0.0.1.0 as f64;
_5.0.0.1 = (_12,);
_14.1 = (_5.0.0.1.0,);
_14.0 = !_5.0.0.0;
_16 = _13 as f32;
_5.0.0.1.0 = _2 as i32;
_15 = RET;
_18.0.1 = (_14.1.0,);
_18.0 = (_5.0.0.0, _5.0.0.1, _5.0.0.2);
_14 = (_5.0.0.0, _5.0.0.1, _5.0.0.2);
_5.0.0.1.0 = _14.1.0;
_5.0.0.2 = _14.2;
_18.0.1.0 = _11.0.fld0 as i32;
_5.0.0.1.0 = _6 as i32;
_5.0.0 = (_14.0, _14.1, _14.2);
_19 = !_2;
_18 = (_14,);
match _5.0.0.0 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
1043560223 => bb12,
_ => bb11
}
}
bb6 = {
_5.0.0.2 = _1;
_5.0.0.1 = (1523794768_i32,);
_5.0.0.1.0 = 721489343_i32 & (-1954115594_i32);
_5.0.0.1.0 = -378626487_i32;
_5.0.0.2 = _1;
_8 = 103_i8 as f64;
_6 = _5.2 as u128;
_5.0.0.1.0 = (-1839609035_i32) >> RET;
_2 = _5.0.0.1.0 <= _5.0.0.1.0;
_6 = !318864206417010895334432920090652739086_u128;
_9 = [53_u8,55_u8,117_u8,133_u8,187_u8];
_5.0.0.2 = _1;
_5.2 = _3 as i128;
_5.2 = _8 as i128;
_12 = _8 as i32;
_1 = _5.0.0.2;
_3 = _5.2 as f32;
_5.2 = 155618831744650638794793054345405546920_i128 ^ (-104559983143522723881561316955577338096_i128);
RET = -(-9223372036854775808_isize);
_11.0.fld0 = (-27215_i16) ^ 3815_i16;
_13 = !_12;
_5.0.0.1 = (_12,);
_5.0.0.0 = 1043560223_u32;
_5.0.0.1 = (_12,);
Goto(bb5)
}
bb7 = {
RET = 119164572398693230624523963585267105865_u128 as isize;
_5.0.0.0 = 379956479_u32 + 3111941504_u32;
_6 = 34846959043275086811254248123987878257_u128 * 269832649123942013396521676041172373173_u128;
_5.0.0.2 = _1;
RET = -9223372036854775807_isize;
RET = 25851_u16 as isize;
_5.2 = 30335_i16 as i128;
_5.0.0.1.0 = 38825708_i32 << _5.2;
_5.0.0.1 = ((-904341762_i32),);
RET = (-39_isize) + 22_isize;
_5.0.0.0 = 6954_i16 as u32;
RET = 9223372036854775807_isize;
_8 = _5.0.0.0 as f64;
match RET {
0 => bb1,
9223372036854775807 => bb4,
_ => bb2
}
}
bb8 = {
_3 = 62556_u16 as f32;
_1 = '\u{336ab}';
_2 = false;
RET = -9223372036854775807_isize;
_1 = '\u{65de4}';
_1 = '\u{35741}';
RET = !(-34_isize);
_3 = 751581187_u32 as f32;
_1 = '\u{47c40}';
_3 = 0_i8 as f32;
RET = -(-9223372036854775808_isize);
_1 = '\u{c24e9}';
RET = !107_isize;
RET = -(-9223372036854775808_isize);
_2 = true | false;
_1 = '\u{dd79f}';
_3 = 6573817306910536379_usize as f32;
RET = (-9223372036854775808_isize);
RET = -(-63_isize);
_2 = false;
Goto(bb3)
}
bb9 = {
RET = 24_isize - (-9223372036854775808_isize);
_1 = '\u{c3eb9}';
_2 = true;
_2 = false;
RET = 13382406394079497559_u64 as isize;
_1 = '\u{c4716}';
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_2 = true;
RET = 24_isize;
_1 = '\u{b440f}';
_2 = true | false;
_1 = '\u{d5c99}';
RET = 17268_i16 as isize;
RET = 123_isize + (-9223372036854775808_isize);
_2 = !true;
_2 = true | true;
_3 = 7556312569096233671_u64 as f32;
_1 = '\u{60484}';
_1 = '\u{a6fbf}';
_2 = _3 == _3;
_1 = '\u{8edb8}';
RET = (-8324418820720603587_i64) as isize;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_13 = _5.0.0.1.0 ^ _14.1.0;
_18.0 = (_5.0.0.0, _14.1, _1);
_18.0.2 = _14.2;
_5.0.0 = (_18.0.0, _18.0.1, _18.0.2);
_19 = !_2;
_18.0.1.0 = _13;
RET = _15 << _14.1.0;
_19 = _2;
_11.0.fld0 = -(-21165_i16);
_16 = _3 * _3;
_11.0.fld0 = _18.0.2 as i16;
_2 = !_19;
_14.1.0 = 1498343375666043732_u64 as i32;
RET = _15;
_16 = _3 + _3;
match _18.0.0 {
0 => bb10,
1 => bb4,
2 => bb9,
3 => bb13,
1043560223 => bb15,
_ => bb14
}
}
bb13 = {
RET = 24_isize - (-9223372036854775808_isize);
_1 = '\u{c3eb9}';
_2 = true;
_2 = false;
RET = 13382406394079497559_u64 as isize;
_1 = '\u{c4716}';
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_2 = true;
RET = 24_isize;
_1 = '\u{b440f}';
_2 = true | false;
_1 = '\u{d5c99}';
RET = 17268_i16 as isize;
RET = 123_isize + (-9223372036854775808_isize);
_2 = !true;
_2 = true | true;
_3 = 7556312569096233671_u64 as f32;
_1 = '\u{60484}';
_1 = '\u{a6fbf}';
_2 = _3 == _3;
_1 = '\u{8edb8}';
RET = (-8324418820720603587_i64) as isize;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_20 = [_5.0.0.0,_5.0.0.0];
_14.0 = !_18.0.0;
_9 = [52_u8,226_u8,166_u8,106_u8,218_u8];
RET = !_15;
_13 = -_18.0.1.0;
_12 = _18.0.1.0;
_14.2 = _1;
_11.0 = Adt74 { fld0: (-9876_i16) };
_18.0 = (_5.0.0.0, _5.0.0.1, _1);
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(1_usize, 20_usize, Move(_20), 1_usize, Move(_1), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(1_usize, 6_usize, Move(_6), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: char,mut _2: char,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: char) -> isize {
mir! {
type RET = isize;
let _13: &'static i8;
let _14: u32;
let _15: &'static &'static *mut i128;
let _16: [char; 7];
let _17: i16;
let _18: bool;
let _19: (*mut (((u32,),), *mut *mut &'static i32), u16, &'static *mut i128);
let _20: &'static &'static i8;
let _21: i8;
let _22: *mut isize;
let _23: ((u32, (i32,), char),);
let _24: f64;
let _25: Adt69;
let _26: ((u16,),);
let _27: &'static *mut i128;
let _28: *mut &'static i32;
let _29: ();
let _30: ();
{
_2 = _10;
_12 = _4;
RET = (-9223372036854775808_isize);
_10 = _2;
_9 = _7;
_3 = _8;
_1 = _6;
_7 = _8;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb7,
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
RET = (-5995298233249082738_i64) as isize;
_8 = _2;
_14 = 12804409315767850703_u64 as u32;
_4 = _10;
_6 = _5;
RET = (-90_isize) | 9223372036854775807_isize;
_6 = _7;
_1 = _3;
_3 = _11;
_1 = _10;
_11 = _10;
RET = 9223372036854775807_isize + (-9223372036854775808_isize);
_9 = _11;
_14 = !210765214_u32;
_12 = _8;
Goto(bb8)
}
bb8 = {
_7 = _2;
_10 = _6;
RET = (-16_isize) << _14;
_4 = _6;
_4 = _8;
_1 = _6;
_4 = _5;
_10 = _3;
_4 = _1;
Call(_11 = fn3(_12, RET, _7, _4, _7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = 9223372036854775807_isize;
_7 = _2;
RET = 26_isize & 24_isize;
_7 = _1;
_8 = _12;
_5 = _9;
_16 = [_7,_5,_3,_12,_10,_12,_12];
_4 = _11;
_1 = _2;
_5 = _11;
_3 = _1;
_2 = _4;
_12 = _10;
Goto(bb10)
}
bb10 = {
_17 = (-31419_i16) >> RET;
_10 = _1;
Goto(bb11)
}
bb11 = {
_18 = false;
_2 = _3;
_7 = _11;
_15 = &_19.2;
_15 = &(*_15);
_6 = _11;
Goto(bb12)
}
bb12 = {
_7 = _8;
_16 = [_2,_10,_4,_12,_9,_8,_3];
_7 = _10;
_15 = &(*_15);
_10 = _9;
_11 = _1;
_7 = _9;
_6 = _1;
_16 = [_10,_7,_1,_6,_11,_5,_1];
_1 = _8;
Goto(bb13)
}
bb13 = {
_19.1 = 2908_u16;
_19.1 = 43379_u16 - 58012_u16;
_5 = _7;
_19.1 = 49510_u16 + 6849_u16;
_14 = 676385043_u32;
_5 = _4;
RET = !44_isize;
_7 = _8;
_12 = _10;
_4 = _6;
_14 = _19.1 as u32;
_16 = [_4,_12,_2,_11,_7,_7,_9];
_5 = _10;
_23.0.1 = ((-1035597138_i32),);
_3 = _4;
_12 = _9;
_23.0.1 = ((-471984063_i32),);
_6 = _4;
_21 = -(-12_i8);
_20 = &_13;
_23.0.1.0 = 472352975_i32 - (-996503228_i32);
_23.0.2 = _11;
_22 = core::ptr::addr_of_mut!(RET);
_4 = _6;
_16 = [_7,_7,_6,_5,_23.0.2,_2,_23.0.2];
_22 = core::ptr::addr_of_mut!((*_22));
(*_22) = 74_u8 as isize;
_22 = core::ptr::addr_of_mut!((*_22));
_12 = _6;
Goto(bb14)
}
bb14 = {
_12 = _6;
_9 = _4;
_10 = _11;
_23.0.1.0 = -566040127_i32;
_2 = _9;
RET = !50_isize;
(*_22) = !(-128_isize);
RET = 44_isize;
_13 = &_21;
_17 = (*_22) as i16;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 2_usize, Move(_2), 6_usize, Move(_6), 12_usize, Move(_12), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 1_usize, Move(_1), 3_usize, Move(_3), 7_usize, Move(_7), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: char,mut _2: isize,mut _3: char,mut _4: char,mut _5: char) -> char {
mir! {
type RET = char;
let _6: f64;
let _7: (i32,);
let _8: isize;
let _9: [usize; 2];
let _10: usize;
let _11: bool;
let _12: &'static i8;
let _13: &'static i8;
let _14: Adt79;
let _15: Adt29;
let _16: isize;
let _17: [u64; 7];
let _18: u8;
let _19: isize;
let _20: Adt79;
let _21: *const ((u32, (i32,), char),);
let _22: ();
let _23: ();
{
RET = _5;
Goto(bb1)
}
bb1 = {
_3 = _4;
_4 = _5;
_4 = _3;
_3 = _4;
RET = _5;
_2 = !(-9223372036854775808_isize);
_1 = RET;
_7.0 = !19348837_i32;
_1 = _4;
Goto(bb2)
}
bb2 = {
_5 = _3;
_6 = 8058_i16 as f64;
_6 = (-11519_i16) as f64;
_3 = _4;
_1 = RET;
_2 = (-9223372036854775808_isize);
_8 = !_2;
_2 = -_8;
_8 = !_2;
_9 = [11953323994597464477_usize,1_usize];
Call(_4 = fn4(_9, _6, _1, _3, _8, _3, _6, _9, _8, _3, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _5;
_5 = _1;
_11 = !false;
_7.0 = 1741663617_i32 ^ (-458814129_i32);
_8 = 26075_i16 as isize;
_1 = _3;
_5 = _4;
_6 = 26616_u16 as f64;
_6 = 1223966114_u32 as f64;
_4 = _3;
_10 = !11120805671292000063_usize;
RET = _4;
_8 = 277842846321651729500216785784851720063_u128 as isize;
_6 = (-25257_i16) as f64;
Goto(bb4)
}
bb4 = {
_7.0 = (-1068074713_i32) >> _8;
_7 = ((-223994673_i32),);
RET = _5;
_9 = [_10,_10];
_10 = 5288_u16 as usize;
_3 = _1;
_8 = -_2;
_11 = true & false;
match _7.0 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431544216783 => bb9,
_ => bb8
}
}
bb5 = {
RET = _5;
_5 = _1;
_11 = !false;
_7.0 = 1741663617_i32 ^ (-458814129_i32);
_8 = 26075_i16 as isize;
_1 = _3;
_5 = _4;
_6 = 26616_u16 as f64;
_6 = 1223966114_u32 as f64;
_4 = _3;
_10 = !11120805671292000063_usize;
RET = _4;
_8 = 277842846321651729500216785784851720063_u128 as isize;
_6 = (-25257_i16) as f64;
Goto(bb4)
}
bb6 = {
_5 = _3;
_6 = 8058_i16 as f64;
_6 = (-11519_i16) as f64;
_3 = _4;
_1 = RET;
_2 = (-9223372036854775808_isize);
_8 = !_2;
_2 = -_8;
_8 = !_2;
_9 = [11953323994597464477_usize,1_usize];
Call(_4 = fn4(_9, _6, _1, _3, _8, _3, _6, _9, _8, _3, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = _4;
_4 = _5;
_4 = _3;
_3 = _4;
RET = _5;
_2 = !(-9223372036854775808_isize);
_1 = RET;
_7.0 = !19348837_i32;
_1 = _4;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_4 = _5;
_9 = [_10,_10];
match _7.0 {
0 => bb2,
340282366920938463463374607431544216783 => bb11,
_ => bb10
}
}
bb10 = {
_5 = _3;
_6 = 8058_i16 as f64;
_6 = (-11519_i16) as f64;
_3 = _4;
_1 = RET;
_2 = (-9223372036854775808_isize);
_8 = !_2;
_2 = -_8;
_8 = !_2;
_9 = [11953323994597464477_usize,1_usize];
Call(_4 = fn4(_9, _6, _1, _3, _8, _3, _6, _9, _8, _3, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
Goto(bb12)
}
bb12 = {
_6 = 3407744363_u32 as f64;
_6 = (-111063714223327131157004903061544679773_i128) as f64;
RET = _5;
_2 = _8;
_2 = _8 + _8;
_8 = _2;
_1 = RET;
_7 = (1580190016_i32,);
_10 = 50485_u16 as usize;
_11 = !true;
_3 = _4;
_2 = (-123157215463146884314421893094872537116_i128) as isize;
_2 = -_8;
_11 = !true;
_5 = _4;
_5 = RET;
_7 = (1014993612_i32,);
RET = _5;
_1 = _4;
_4 = _1;
_7 = ((-1264565966_i32),);
_7.0 = _10 as i32;
Goto(bb13)
}
bb13 = {
_6 = 10_u8 as f64;
_3 = RET;
_11 = false;
Goto(bb14)
}
bb14 = {
_11 = false;
_16 = _8 & _2;
_9 = [_10,_10];
_17 = [4648354397693542867_u64,13213652240384035242_u64,6227296880552714034_u64,14053522102053100597_u64,16134072204928785573_u64,7282368558205659721_u64,9566763490893900257_u64];
_6 = (-71309651757702501873791352004090684000_i128) as f64;
_9 = [_10,_10];
_5 = _3;
_6 = 3279018196659802598_i64 as f64;
_3 = _1;
_6 = 244_u8 as f64;
_5 = _1;
_7 = ((-764310092_i32),);
RET = _4;
_18 = _6 as u8;
_19 = !_8;
_11 = !true;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(3_usize, 16_usize, Move(_16), 2_usize, Move(_2), 19_usize, Move(_19), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(3_usize, 1_usize, Move(_1), 7_usize, Move(_7), 4_usize, Move(_4), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [usize; 2],mut _2: f64,mut _3: char,mut _4: char,mut _5: isize,mut _6: char,mut _7: f64,mut _8: [usize; 2],mut _9: isize,mut _10: char,mut _11: (i32,)) -> char {
mir! {
type RET = char;
let _12: (u32,);
let _13: f32;
let _14: ([u32; 7], [u8; 7], &'static Adt22);
let _15: u8;
let _16: *const [i64; 6];
let _17: isize;
let _18: (((u32,),), *mut *mut &'static i32);
let _19: char;
let _20: &'static *mut i128;
let _21: ([u32; 7], [u8; 7], &'static Adt22);
let _22: &'static &'static i8;
let _23: &'static &'static i8;
let _24: ([i64; 8], u16, [u32; 7], i64);
let _25: &'static i16;
let _26: [u8; 7];
let _27: *mut i128;
let _28: i32;
let _29: isize;
let _30: ();
let _31: ();
{
_7 = _2;
_1 = [1647103898478791736_usize,1_usize];
_3 = _10;
_6 = _3;
_8 = _1;
_5 = _11.0 as isize;
_12 = (1207469775_u32,);
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1207469775 => bb9,
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
_14.0 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
RET = _6;
_12.0 = 2683192661_u32;
_11 = (1830028107_i32,);
_8 = [3_usize,12852928205526365164_usize];
_5 = 206315524494882256005424730301583091278_u128 as isize;
_13 = _9 as f32;
_11 = ((-2104604749_i32),);
_12.0 = 5_usize as u32;
RET = _10;
_7 = _2 - _2;
_17 = _5;
Call(_16 = fn5(_11.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19 = _10;
match _11.0 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb11,
4 => bb12,
5 => bb13,
340282366920938463463374607429663606707 => bb15,
_ => bb14
}
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
Return()
}
bb15 = {
_18.0.0 = (_12.0,);
_9 = !_17;
_11.0 = (-1603755133_i32) ^ (-522609654_i32);
_8 = [0_usize,5_usize];
_21.0 = _14.0;
_6 = _4;
_9 = _17 * _5;
_1 = _8;
_24.2 = [_12.0,_18.0.0.0,_12.0,_18.0.0.0,_18.0.0.0,_12.0,_18.0.0.0];
_24.1 = 4974_u16;
_21.1 = [42_u8,182_u8,61_u8,238_u8,234_u8,176_u8,170_u8];
_24.3 = 7244677113281739262_i64;
_21.1 = [153_u8,106_u8,145_u8,254_u8,201_u8,90_u8,251_u8];
_14.0 = [_12.0,_12.0,_12.0,_18.0.0.0,_12.0,_18.0.0.0,_12.0];
_14.0 = [_12.0,_12.0,_18.0.0.0,_18.0.0.0,_12.0,_18.0.0.0,_12.0];
_13 = _11.0 as f32;
_5 = !_17;
Goto(bb16)
}
bb16 = {
Call(_30 = dump_var(4_usize, 17_usize, Move(_17), 4_usize, Move(_4), 11_usize, Move(_11), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(4_usize, 3_usize, Move(_3), 6_usize, Move(_6), 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i32) -> *const [i64; 6] {
mir! {
type RET = *const [i64; 6];
let _2: i64;
let _3: [bool; 7];
let _4: isize;
let _5: [bool; 7];
let _6: i128;
let _7: i8;
let _8: (i32,);
let _9: (Adt74,);
let _10: bool;
let _11: f64;
let _12: (u32,);
let _13: char;
let _14: *const &'static &'static i8;
let _15: &'static &'static *mut i128;
let _16: char;
let _17: Adt29;
let _18: isize;
let _19: [u16; 6];
let _20: [u16; 6];
let _21: f32;
let _22: &'static u64;
let _23: isize;
let _24: isize;
let _25: [i64; 6];
let _26: f32;
let _27: ();
let _28: ();
{
_1 = 9100827799500181833_usize as i32;
_1 = 534591080_i32;
_1 = (-38_isize) as i32;
_2 = (-5986301411022073444_i64) | (-4524791083966539170_i64);
_2 = 727253219_u32 as i64;
_2 = !7604177914790405516_i64;
_1 = !1692901195_i32;
_1 = 6431751_i32 | 1191033217_i32;
_1 = 883230172_i32 ^ (-2139370291_i32);
_2 = 2431091002899866332_i64 << _1;
_4 = 86_isize & 9223372036854775807_isize;
_3 = [true,true,true,false,false,true,false];
_3 = [false,false,false,true,false,false,false];
_3 = [true,true,true,true,true,false,true];
Goto(bb1)
}
bb1 = {
_5 = [false,false,false,true,false,true,false];
_2 = 66155586396340855_i64;
_1 = -707314077_i32;
_4 = -(-9223372036854775808_isize);
_3 = [true,false,true,false,false,false,false];
_1 = (-657898816_i32);
_5 = [false,false,true,true,true,true,false];
_3 = _5;
_3 = _5;
_3 = [true,false,true,true,true,false,false];
_4 = 63325_u16 as isize;
match _1 {
0 => bb2,
340282366920938463463374607431110312640 => bb4,
_ => bb3
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
_2 = 163740207995169665724485926264194779744_u128 as i64;
_4 = 24_isize;
_6 = 15020225417070242753425692203409239764_i128;
_1 = (-1553848759_i32) + 574577603_i32;
_3 = _5;
_1 = 251988357_i32;
_3 = _5;
_6 = 182067656640667357904288353088391263199_u128 as i128;
_1 = !(-379688843_i32);
_6 = false as i128;
_4 = (-85_isize);
_4 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = [false,true,true,false,true,true,false];
_3 = [true,true,false,true,true,true,true];
_1 = (-1185781057_i32) + 1147704829_i32;
_4 = (-9223372036854775808_isize);
_3 = _5;
_1 = -(-1597319571_i32);
_6 = -(-47184591376807826689611058362233557158_i128);
_2 = 4697669620558012499_i64;
_4 = 148_u8 as isize;
_6 = 4124193439_u32 as i128;
_7 = (-74_i8);
_1 = 727559312_i32 - 74018781_i32;
_3 = [true,true,true,false,false,true,false];
_4 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb5)
}
bb5 = {
_1 = false as i32;
_6 = 97351707164428194319305730913363014771_i128;
_4 = !(-54_isize);
_6 = (-68018350486656407804720353053198690961_i128);
_6 = !(-93279002656610517409345472719890877511_i128);
_7 = -(-41_i8);
Goto(bb6)
}
bb6 = {
_2 = (-2290163142405682900_i64) & (-6434164691711314151_i64);
_9.0 = Adt74 { fld0: 26440_i16 };
_4 = 6_usize as isize;
_8.0 = -_1;
_5 = _3;
_4 = (-88_isize);
_9.0 = Adt74 { fld0: 16533_i16 };
_9.0.fld0 = _7 as i16;
_10 = !false;
_5 = [_10,_10,_10,_10,_10,_10,_10];
_3 = [_10,_10,_10,_10,_10,_10,_10];
_2 = -4471295857373418178_i64;
Call(_10 = fn6(_4, _8.0, _5, _9.0, _3, _4, _3, _3, _8.0, _9.0, _7, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = _8.0 as f64;
_1 = !_8.0;
_1 = 3_usize as i32;
_3 = _5;
_11 = _2 as f64;
_8 = (_1,);
_3 = _5;
_9.0.fld0 = (-2209_i16) * (-7605_i16);
_8.0 = 7419488116536076753_u64 as i32;
_8.0 = !_1;
_5 = _3;
_12.0 = _9.0.fld0 as u32;
_8 = (_1,);
_13 = '\u{ce562}';
_8 = (_1,);
_7 = _11 as i8;
_2 = (-3786512825685364508_i64) - (-7533809852473660221_i64);
_3 = [_10,_10,_10,_10,_10,_10,_10];
_16 = _13;
_12 = (736229147_u32,);
_9.0.fld0 = !15237_i16;
_5 = [_10,_10,_10,_10,_10,_10,_10];
_12.0 = 2108599897_u32 + 1934138933_u32;
_11 = 6698335384457328980_usize as f64;
match _4 {
0 => bb8,
1 => bb9,
2 => bb10,
340282366920938463463374607431768211368 => bb12,
_ => bb11
}
}
bb8 = {
_2 = (-2290163142405682900_i64) & (-6434164691711314151_i64);
_9.0 = Adt74 { fld0: 26440_i16 };
_4 = 6_usize as isize;
_8.0 = -_1;
_5 = _3;
_4 = (-88_isize);
_9.0 = Adt74 { fld0: 16533_i16 };
_9.0.fld0 = _7 as i16;
_10 = !false;
_5 = [_10,_10,_10,_10,_10,_10,_10];
_3 = [_10,_10,_10,_10,_10,_10,_10];
_2 = -4471295857373418178_i64;
Call(_10 = fn6(_4, _8.0, _5, _9.0, _3, _4, _3, _3, _8.0, _9.0, _7, _7), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
Return()
}
bb10 = {
_2 = 163740207995169665724485926264194779744_u128 as i64;
_4 = 24_isize;
_6 = 15020225417070242753425692203409239764_i128;
_1 = (-1553848759_i32) + 574577603_i32;
_3 = _5;
_1 = 251988357_i32;
_3 = _5;
_6 = 182067656640667357904288353088391263199_u128 as i128;
_1 = !(-379688843_i32);
_6 = false as i128;
_4 = (-85_isize);
_4 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = [false,true,true,false,true,true,false];
_3 = [true,true,false,true,true,true,true];
_1 = (-1185781057_i32) + 1147704829_i32;
_4 = (-9223372036854775808_isize);
_3 = _5;
_1 = -(-1597319571_i32);
_6 = -(-47184591376807826689611058362233557158_i128);
_2 = 4697669620558012499_i64;
_4 = 148_u8 as isize;
_6 = 4124193439_u32 as i128;
_7 = (-74_i8);
_1 = 727559312_i32 - 74018781_i32;
_3 = [true,true,true,false,false,true,false];
_4 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb5)
}
bb11 = {
Return()
}
bb12 = {
_8 = (_1,);
_12.0 = _11 as u32;
_10 = true;
_20 = [63114_u16,35267_u16,54938_u16,29392_u16,1648_u16,44709_u16];
_8.0 = _1 & _1;
_3 = [_10,_10,_10,_10,_10,_10,_10];
_18 = !_4;
_16 = _13;
_5 = [_10,_10,_10,_10,_10,_10,_10];
_12 = (569405018_u32,);
_4 = 11879860102235051603_usize as isize;
_18 = _4;
_10 = true;
_11 = 53561_u16 as f64;
match _12.0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb19,
569405018 => bb21,
_ => bb20
}
}
bb13 = {
Return()
}
bb14 = {
_2 = 163740207995169665724485926264194779744_u128 as i64;
_4 = 24_isize;
_6 = 15020225417070242753425692203409239764_i128;
_1 = (-1553848759_i32) + 574577603_i32;
_3 = _5;
_1 = 251988357_i32;
_3 = _5;
_6 = 182067656640667357904288353088391263199_u128 as i128;
_1 = !(-379688843_i32);
_6 = false as i128;
_4 = (-85_isize);
_4 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = [false,true,true,false,true,true,false];
_3 = [true,true,false,true,true,true,true];
_1 = (-1185781057_i32) + 1147704829_i32;
_4 = (-9223372036854775808_isize);
_3 = _5;
_1 = -(-1597319571_i32);
_6 = -(-47184591376807826689611058362233557158_i128);
_2 = 4697669620558012499_i64;
_4 = 148_u8 as isize;
_6 = 4124193439_u32 as i128;
_7 = (-74_i8);
_1 = 727559312_i32 - 74018781_i32;
_3 = [true,true,true,false,false,true,false];
_4 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb5)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_11 = _8.0 as f64;
_1 = !_8.0;
_1 = 3_usize as i32;
_3 = _5;
_11 = _2 as f64;
_8 = (_1,);
_3 = _5;
_9.0.fld0 = (-2209_i16) * (-7605_i16);
_8.0 = 7419488116536076753_u64 as i32;
_8.0 = !_1;
_5 = _3;
_12.0 = _9.0.fld0 as u32;
_8 = (_1,);
_13 = '\u{ce562}';
_8 = (_1,);
_7 = _11 as i8;
_2 = (-3786512825685364508_i64) - (-7533809852473660221_i64);
_3 = [_10,_10,_10,_10,_10,_10,_10];
_16 = _13;
_12 = (736229147_u32,);
_9.0.fld0 = !15237_i16;
_5 = [_10,_10,_10,_10,_10,_10,_10];
_12.0 = 2108599897_u32 + 1934138933_u32;
_11 = 6698335384457328980_usize as f64;
match _4 {
0 => bb8,
1 => bb9,
2 => bb10,
340282366920938463463374607431768211368 => bb12,
_ => bb11
}
}
bb18 = {
_5 = [false,false,false,true,false,true,false];
_2 = 66155586396340855_i64;
_1 = -707314077_i32;
_4 = -(-9223372036854775808_isize);
_3 = [true,false,true,false,false,false,false];
_1 = (-657898816_i32);
_5 = [false,false,true,true,true,true,false];
_3 = _5;
_3 = _5;
_3 = [true,false,true,true,true,false,false];
_4 = 63325_u16 as isize;
match _1 {
0 => bb2,
340282366920938463463374607431110312640 => bb4,
_ => bb3
}
}
bb19 = {
Return()
}
bb20 = {
_2 = 163740207995169665724485926264194779744_u128 as i64;
_4 = 24_isize;
_6 = 15020225417070242753425692203409239764_i128;
_1 = (-1553848759_i32) + 574577603_i32;
_3 = _5;
_1 = 251988357_i32;
_3 = _5;
_6 = 182067656640667357904288353088391263199_u128 as i128;
_1 = !(-379688843_i32);
_6 = false as i128;
_4 = (-85_isize);
_4 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = [false,true,true,false,true,true,false];
_3 = [true,true,false,true,true,true,true];
_1 = (-1185781057_i32) + 1147704829_i32;
_4 = (-9223372036854775808_isize);
_3 = _5;
_1 = -(-1597319571_i32);
_6 = -(-47184591376807826689611058362233557158_i128);
_2 = 4697669620558012499_i64;
_4 = 148_u8 as isize;
_6 = 4124193439_u32 as i128;
_7 = (-74_i8);
_1 = 727559312_i32 - 74018781_i32;
_3 = [true,true,true,false,false,true,false];
_4 = (-9223372036854775808_isize) - 9223372036854775807_isize;
Goto(bb5)
}
bb21 = {
_10 = true;
_1 = _8.0;
_1 = _8.0;
_8 = (_1,);
_21 = _11 as f32;
_5 = [_10,_10,_10,_10,_10,_10,_10];
_8 = (_1,);
_8 = (_1,);
_12.0 = _2 as u32;
_4 = -_18;
_9.0.fld0 = 1773567789075964851_u64 as i16;
_18 = _4;
_8.0 = 7600004816358488064_u64 as i32;
_7 = (-106_i8) & (-16_i8);
_23 = _4 - _4;
_10 = true;
_12 = (1787044689_u32,);
_7 = _13 as i8;
_11 = 396819633147107336_u64 as f64;
_19 = _20;
Goto(bb22)
}
bb22 = {
_3 = [_10,_10,_10,_10,_10,_10,_10];
_11 = 5482483267036899960_u64 as f64;
_8 = (_1,);
_24 = _18;
_19 = [24394_u16,15526_u16,42046_u16,18423_u16,29124_u16,19921_u16];
_19 = [18274_u16,10237_u16,2170_u16,60197_u16,23996_u16,9533_u16];
_8.0 = _1;
_23 = _24;
_25 = [_2,_2,_2,_2,_2,_2];
_19 = [60652_u16,18578_u16,33557_u16,11756_u16,54243_u16,63686_u16];
_20 = [23392_u16,36066_u16,27065_u16,15514_u16,50678_u16,27589_u16];
_21 = _12.0 as f32;
_25 = [_2,_2,_2,_2,_2,_2];
_11 = _21 as f64;
_11 = 4_usize as f64;
_9.0 = Adt74 { fld0: (-21487_i16) };
_6 = 5586836598032901606_usize as i128;
RET = core::ptr::addr_of!(_25);
_12 = (4219011731_u32,);
Goto(bb23)
}
bb23 = {
Call(_27 = dump_var(5_usize, 1_usize, Move(_1), 25_usize, Move(_25), 13_usize, Move(_13), 16_usize, Move(_16)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_27 = dump_var(5_usize, 23_usize, Move(_23), 18_usize, Move(_18), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_27 = dump_var(5_usize, 4_usize, Move(_4), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: i32,mut _3: [bool; 7],mut _4: Adt74,mut _5: [bool; 7],mut _6: isize,mut _7: [bool; 7],mut _8: [bool; 7],mut _9: i32,mut _10: Adt74,mut _11: i8,mut _12: i8) -> bool {
mir! {
type RET = bool;
let _13: isize;
let _14: isize;
let _15: &'static char;
let _16: isize;
let _17: f64;
let _18: char;
let _19: *mut (u32, (i32,), char);
let _20: *mut bool;
let _21: *mut usize;
let _22: isize;
let _23: f32;
let _24: u128;
let _25: i64;
let _26: i32;
let _27: u8;
let _28: f32;
let _29: f32;
let _30: (((u16,),), &'static ([i64; 8], u16, [u32; 7], i64), [usize; 3]);
let _31: *mut isize;
let _32: (u16,);
let _33: Adt81;
let _34: (u16,);
let _35: (bool, i128);
let _36: ();
let _37: ();
{
_3 = _7;
_9 = _2 - _2;
_1 = _6;
_10.fld0 = _4.fld0;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211368 => bb5,
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
_10 = Adt74 { fld0: _4.fld0 };
_2 = _9;
_1 = _6 << _9;
_1 = _6;
RET = !true;
_4.fld0 = 239295330613735298355294045912869720676_u128 as i16;
_11 = !_12;
_4 = Adt74 { fld0: _10.fld0 };
_8 = [RET,RET,RET,RET,RET,RET,RET];
_9 = 119_u8 as i32;
_10.fld0 = -_4.fld0;
_5 = [RET,RET,RET,RET,RET,RET,RET];
_10.fld0 = _11 as i16;
_13 = _1;
_12 = -_11;
_2 = 1647740206_u32 as i32;
_12 = _11 * _11;
_10 = Adt74 { fld0: _4.fld0 };
_5 = _3;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_10 = Adt74 { fld0: _4.fld0 };
_14 = _13 * _13;
_9 = _2 | _2;
_1 = _13;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211368 => bb6,
_ => bb4
}
}
bb6 = {
_1 = _14;
_9 = _2;
_13 = _14;
_10 = _4;
_10.fld0 = _4.fld0;
_12 = -_11;
_9 = _2 & _2;
_4.fld0 = 51_u8 as i16;
_10.fld0 = _4.fld0 - _4.fld0;
_18 = '\u{2b029}';
_14 = _13 ^ _13;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_10 = Adt74 { fld0: _4.fld0 };
_4 = _10;
_1 = _14 & _13;
_20 = core::ptr::addr_of_mut!(RET);
match _6 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431768211368 => bb11,
_ => bb10
}
}
bb7 = {
_10 = Adt74 { fld0: _4.fld0 };
_2 = _9;
_1 = _6 << _9;
_1 = _6;
RET = !true;
_4.fld0 = 239295330613735298355294045912869720676_u128 as i16;
_11 = !_12;
_4 = Adt74 { fld0: _10.fld0 };
_8 = [RET,RET,RET,RET,RET,RET,RET];
_9 = 119_u8 as i32;
_10.fld0 = -_4.fld0;
_5 = [RET,RET,RET,RET,RET,RET,RET];
_10.fld0 = _11 as i16;
_13 = _1;
_12 = -_11;
_2 = 1647740206_u32 as i32;
_12 = _11 * _11;
_10 = Adt74 { fld0: _4.fld0 };
_5 = _3;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_10 = Adt74 { fld0: _4.fld0 };
_14 = _13 * _13;
_9 = _2 | _2;
_1 = _13;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211368 => bb6,
_ => bb4
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
_15 = &_18;
_10 = _4;
(*_20) = !false;
_9 = _2;
_3 = _8;
_14 = !_1;
(*_20) = true;
_14 = !_13;
_16 = _13 ^ _1;
_26 = _2 - _2;
_25 = 955_u16 as i64;
_13 = _1;
_22 = _1 ^ _13;
_17 = _2 as f64;
_6 = _16 * _13;
_24 = 200743990563403903322094348908672274968_u128;
_3 = [(*_20),(*_20),(*_20),RET,(*_20),(*_20),(*_20)];
_1 = _18 as isize;
_10.fld0 = _6 as i16;
_24 = !100710091824605960879906069794013618367_u128;
(*_20) = !false;
RET = false;
_20 = core::ptr::addr_of_mut!(RET);
_24 = 48765919332640649398455285929194499958_u128;
Call(_22 = fn7(Move(_15), _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_8 = [RET,(*_20),(*_20),(*_20),(*_20),(*_20),(*_20)];
_20 = core::ptr::addr_of_mut!(RET);
_8 = _3;
_28 = _17 as f32;
RET = !true;
_23 = 19009_u16 as f32;
_6 = -_16;
_30.2 = [0_usize,5_usize,3_usize];
_10 = Adt74 { fld0: _4.fld0 };
_26 = _28 as i32;
(*_20) = _13 <= _6;
_5 = _3;
(*_20) = false;
_32.0 = 118919828241765866177039997910406437550_i128 as u16;
_13 = _6 - _6;
_30.0 = (_32,);
_2 = _9;
_34 = (_30.0.0.0,);
_7 = _3;
_7 = [RET,(*_20),RET,(*_20),(*_20),RET,(*_20)];
_13 = _22;
_33.fld4 = !_10.fld0;
RET = false;
_5 = _8;
match _24 {
0 => bb5,
1 => bb4,
48765919332640649398455285929194499958 => bb14,
_ => bb13
}
}
bb13 = {
_10 = Adt74 { fld0: _4.fld0 };
_2 = _9;
_1 = _6 << _9;
_1 = _6;
RET = !true;
_4.fld0 = 239295330613735298355294045912869720676_u128 as i16;
_11 = !_12;
_4 = Adt74 { fld0: _10.fld0 };
_8 = [RET,RET,RET,RET,RET,RET,RET];
_9 = 119_u8 as i32;
_10.fld0 = -_4.fld0;
_5 = [RET,RET,RET,RET,RET,RET,RET];
_10.fld0 = _11 as i16;
_13 = _1;
_12 = -_11;
_2 = 1647740206_u32 as i32;
_12 = _11 * _11;
_10 = Adt74 { fld0: _4.fld0 };
_5 = _3;
_3 = [RET,RET,RET,RET,RET,RET,RET];
_10 = Adt74 { fld0: _4.fld0 };
_14 = _13 * _13;
_9 = _2 | _2;
_1 = _13;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768211368 => bb6,
_ => bb4
}
}
bb14 = {
_35.1 = 54006073815749779651543476759067615404_i128 ^ (-152560478809732053871997038452717234968_i128);
(*_20) = false;
RET = !false;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(6_usize, 8_usize, Move(_8), 13_usize, Move(_13), 26_usize, Move(_26), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(6_usize, 9_usize, Move(_9), 18_usize, Move(_18), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(6_usize, 32_usize, Move(_32), 24_usize, Move(_24), 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: &'static char,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: bool;
let _4: &'static u64;
let _5: isize;
let _6: isize;
let _7: &'static &'static *mut i128;
let _8: f32;
let _9: u8;
let _10: u16;
let _11: i16;
let _12: f64;
let _13: f32;
let _14: u16;
let _15: &'static (((u16,),), &'static ([i64; 8], u16, [u32; 7], i64), [usize; 3]);
let _16: [u64; 6];
let _17: ([u32; 7], [u8; 7], &'static Adt22);
let _18: &'static u64;
let _19: (Adt74,);
let _20: ();
let _21: ();
{
_2 = 42037_u16 as isize;
RET = _2 - _2;
_2 = RET;
Goto(bb1)
}
bb1 = {
RET = 6310563913433859657_usize as isize;
_2 = -RET;
RET = -_2;
RET = -_2;
RET = 1920713672_i32 as isize;
_3 = !true;
_5 = RET;
_5 = _2;
_2 = RET ^ RET;
_5 = !RET;
Call(_3 = fn8(RET, _2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _2;
_6 = 149_u8 as isize;
_6 = _5;
_2 = _6 >> _6;
_2 = _6 + RET;
_6 = _5 * RET;
_2 = !RET;
_6 = 324543235605220946071302026386694398910_u128 as isize;
RET = _6;
_3 = _5 != _2;
_2 = 4186349467980179810_usize as isize;
_3 = true | false;
RET = _6;
_2 = -RET;
_2 = _6 ^ RET;
_2 = 93855651020129145097205445535031654637_u128 as isize;
Goto(bb3)
}
bb3 = {
RET = _6;
_6 = 155_u8 as isize;
_6 = _5 + _2;
_8 = 1489456659_u32 as f32;
_2 = -_5;
_6 = (-27_i8) as isize;
RET = _5;
_5 = !_6;
_9 = !197_u8;
_9 = 67_u8;
match _9 {
0 => bb1,
67 => bb5,
_ => bb4
}
}
bb4 = {
_6 = _2;
_6 = 149_u8 as isize;
_6 = _5;
_2 = _6 >> _6;
_2 = _6 + RET;
_6 = _5 * RET;
_2 = !RET;
_6 = 324543235605220946071302026386694398910_u128 as isize;
RET = _6;
_3 = _5 != _2;
_2 = 4186349467980179810_usize as isize;
_3 = true | false;
RET = _6;
_2 = -RET;
_2 = _6 ^ RET;
_2 = 93855651020129145097205445535031654637_u128 as isize;
Goto(bb3)
}
bb5 = {
RET = _2;
_5 = _2 * RET;
_2 = _3 as isize;
_11 = 5885_i16 + 5348_i16;
_11 = 8394_i16;
_2 = RET;
RET = _6 >> _11;
_3 = false;
_9 = !133_u8;
_5 = 123504005969555444807474619842376632266_i128 as isize;
Goto(bb6)
}
bb6 = {
_8 = (-534619746_i32) as f32;
_12 = 324831281288002585292573632644110335979_u128 as f64;
_3 = false;
_2 = !_5;
match _11 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
8394 => bb8,
_ => bb7
}
}
bb7 = {
RET = _6;
_6 = 155_u8 as isize;
_6 = _5 + _2;
_8 = 1489456659_u32 as f32;
_2 = -_5;
_6 = (-27_i8) as isize;
RET = _5;
_5 = !_6;
_9 = !197_u8;
_9 = 67_u8;
match _9 {
0 => bb1,
67 => bb5,
_ => bb4
}
}
bb8 = {
_5 = _8 as isize;
_6 = _2;
_8 = (-1537829004_i32) as f32;
_3 = !true;
_10 = 2322_u16;
_2 = _12 as isize;
_14 = !_10;
_16 = [4868551059929138350_u64,2658321225653896865_u64,14476191404213573523_u64,11485293415592299969_u64,13002786484881373212_u64,11327257688482977481_u64];
_12 = _14 as f64;
Goto(bb9)
}
bb9 = {
_2 = RET >> _5;
_5 = 7556779812009332665401401355531686983_i128 as isize;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
8394 => bb10,
_ => bb8
}
}
bb10 = {
_12 = 11163946380804372401_u64 as f64;
_9 = 79_u8 & 48_u8;
_11 = 18976_i16 & 15998_i16;
_3 = false;
_10 = (-21_i8) as u16;
_2 = 872290861_i32 as isize;
_6 = RET | _2;
_9 = 195_u8;
_12 = 965391532_i32 as f64;
_8 = _2 as f32;
_5 = _6;
_5 = RET << RET;
_11 = 7260_i16 >> _6;
_17.0 = [3661413282_u32,2496419877_u32,2228117625_u32,4189538597_u32,3484473343_u32,1488966607_u32,2696698482_u32];
_2 = _6;
_5 = _12 as isize;
_3 = !true;
_6 = _2;
_16 = [1072020586395470666_u64,9918074913383844817_u64,6103893393357940397_u64,13400312344806267313_u64,4727292468044884679_u64,9518471961788593821_u64];
RET = 842941471_u32 as isize;
_6 = RET;
RET = -_5;
_16 = [112586697359267877_u64,287932695549688881_u64,18144335505549951145_u64,13964849005517589789_u64,4268618914273790171_u64,9309284741570907221_u64];
Goto(bb11)
}
bb11 = {
_17.1 = [_9,_9,_9,_9,_9,_9,_9];
_10 = _14 - _14;
_12 = 234918700401710685064778311707912961955_u128 as f64;
_10 = !_14;
_8 = _11 as f32;
_10 = _14 >> _11;
_17.0 = [1494811765_u32,1010019683_u32,2387798700_u32,4247539573_u32,3311055952_u32,2016109731_u32,3679617557_u32];
Goto(bb12)
}
bb12 = {
_9 = !141_u8;
_2 = _6 & _5;
_9 = RET as u8;
_14 = !_10;
_2 = -_6;
_13 = _8;
_6 = -_5;
_5 = _11 as isize;
RET = _5;
_3 = true;
_12 = 81_i8 as f64;
_8 = _13;
_8 = _13 * _13;
_3 = !false;
RET = _5 + _5;
_3 = !false;
Goto(bb13)
}
bb13 = {
RET = -_5;
Call(RET = core::intrinsics::transmute(_5), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10 = !_14;
_5 = -RET;
_19.0 = Adt74 { fld0: _11 };
RET = _6 ^ _2;
_5 = RET - RET;
_11 = _19.0.fld0 << RET;
_8 = _13;
_11 = 3611272374559401112_usize as i16;
_12 = 12225927078186831425_u64 as f64;
_14 = _12 as u16;
_19.0.fld0 = !_11;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(7_usize, 9_usize, Move(_9), 6_usize, Move(_6), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize) -> bool {
mir! {
type RET = bool;
let _4: ([bool; 7], &'static u64, &'static i8, [i64; 8]);
let _5: i32;
let _6: [bool; 6];
let _7: [u8; 5];
let _8: bool;
let _9: isize;
let _10: &'static i16;
let _11: bool;
let _12: ((u16,),);
let _13: f32;
let _14: char;
let _15: i8;
let _16: &'static char;
let _17: (i32,);
let _18: u16;
let _19: *mut &'static i32;
let _20: char;
let _21: ();
let _22: ();
{
_4.3 = [1675100403479429894_i64,9090759500377697169_i64,(-2891327503034879482_i64),5767439329472785083_i64,(-8630367162742518429_i64),4637759249202821982_i64,(-8797397705498384828_i64),4857805287821603835_i64];
RET = _3 > _1;
Goto(bb1)
}
bb1 = {
_3 = 98250270913753117416878838889256959678_u128 as isize;
_4.0 = [RET,RET,RET,RET,RET,RET,RET];
_5 = 502757059_i32;
_3 = -_1;
Call(RET = fn9(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = !_1;
_4.3 = [(-5718872180631238585_i64),8911086188549627238_i64,(-5316432462799483468_i64),(-1299958069183614680_i64),312366754210884962_i64,6164787243146260762_i64,97520845944009227_i64,169312120770840058_i64];
_3 = !_1;
_1 = _2;
_4.3 = [(-4416510677382070073_i64),(-1935416345533940691_i64),(-3102690096687603272_i64),(-2961458395050923322_i64),8414513843218677097_i64,6396935800620205396_i64,1746501719195556376_i64,(-2447587594713110232_i64)];
_2 = _3;
_1 = _3 >> _2;
_4.0 = [RET,RET,RET,RET,RET,RET,RET];
match _5 {
0 => bb3,
502757059 => bb5,
_ => bb4
}
}
bb3 = {
_3 = 98250270913753117416878838889256959678_u128 as isize;
_4.0 = [RET,RET,RET,RET,RET,RET,RET];
_5 = 502757059_i32;
_3 = -_1;
Call(RET = fn9(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_5 = 957479582_i32;
_1 = _2;
_3 = _1 >> _5;
_4.3 = [6734539574718999263_i64,1123302766547892951_i64,(-6641471180538342477_i64),5665003117447193259_i64,(-3160335758476060701_i64),9109503447584203366_i64,6886150891167836997_i64,2838213081033283444_i64];
_2 = !_3;
RET = false;
_6 = [RET,RET,RET,RET,RET,RET];
_2 = _3 >> _3;
_5 = -449422295_i32;
_7 = [107_u8,244_u8,181_u8,166_u8,54_u8];
Goto(bb6)
}
bb6 = {
_4.3 = [(-6307835790052201824_i64),4222143107255323534_i64,3132735901783239359_i64,(-7813052627000182118_i64),(-1906619614512231648_i64),(-6272954312743110951_i64),6591761434436965849_i64,(-3731818657860805596_i64)];
_4.0 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
_5 = !(-1293116266_i32);
_4.3 = [(-5923156962983357639_i64),(-447322581280009048_i64),(-4063781231035356057_i64),4541675891604750062_i64,4018038791545660993_i64,(-1153710486780951126_i64),(-68216815132517040_i64),(-8588439448865055579_i64)];
_4.0 = [RET,RET,RET,RET,RET,RET,RET];
_5 = '\u{ce6f8}' as i32;
_7 = [7_u8,234_u8,4_u8,148_u8,89_u8];
_9 = 58_i8 as isize;
_1 = !_9;
_5 = 316767773622838872757732461595516446091_u128 as i32;
_2 = !_3;
_8 = RET;
RET = !_8;
_8 = RET;
_3 = _2;
_11 = !RET;
Goto(bb7)
}
bb7 = {
_1 = _2 >> _5;
_12.0 = (38085_u16,);
RET = _8;
RET = _8 ^ _8;
RET = !_8;
_3 = -_2;
_9 = _2 ^ _2;
_4.0 = [RET,_11,_11,RET,_11,RET,_8];
_4.3 = [(-4823964828218423167_i64),8142149352504726061_i64,4921958520730923547_i64,1135946680584600724_i64,6037434317288629505_i64,(-5367559392555469048_i64),(-4733165176663293486_i64),(-758776625469015175_i64)];
_9 = _1 * _1;
_2 = _1;
_4.3 = [3343010123199460828_i64,(-7887427647897862974_i64),(-3532055784314029643_i64),6736728805062040871_i64,375608125689645240_i64,7369191749219584661_i64,(-5584211217486363751_i64),8661709458545032676_i64];
_15 = '\u{8a49d}' as i8;
_7 = [214_u8,19_u8,5_u8,217_u8,197_u8];
_13 = _12.0.0 as f32;
_9 = _12.0.0 as isize;
Goto(bb8)
}
bb8 = {
_6 = [RET,_8,_11,_11,_11,RET];
_14 = '\u{4d97c}';
_1 = 8753_i16 as isize;
_11 = _1 > _1;
_16 = &_14;
_4.3 = [(-6050384495183168420_i64),8070571861114654105_i64,1051671717269978427_i64,(-983553512072347936_i64),5452078397212771638_i64,2133755239163307981_i64,8873777016667611529_i64,7062310222961813291_i64];
_8 = !_11;
_17 = (_5,);
Goto(bb9)
}
bb9 = {
_16 = &_14;
match _12.0.0 {
0 => bb5,
1 => bb10,
2 => bb11,
38085 => bb13,
_ => bb12
}
}
bb10 = {
_6 = [RET,_8,_11,_11,_11,RET];
_14 = '\u{4d97c}';
_1 = 8753_i16 as isize;
_11 = _1 > _1;
_16 = &_14;
_4.3 = [(-6050384495183168420_i64),8070571861114654105_i64,1051671717269978427_i64,(-983553512072347936_i64),5452078397212771638_i64,2133755239163307981_i64,8873777016667611529_i64,7062310222961813291_i64];
_8 = !_11;
_17 = (_5,);
Goto(bb9)
}
bb11 = {
_5 = 957479582_i32;
_1 = _2;
_3 = _1 >> _5;
_4.3 = [6734539574718999263_i64,1123302766547892951_i64,(-6641471180538342477_i64),5665003117447193259_i64,(-3160335758476060701_i64),9109503447584203366_i64,6886150891167836997_i64,2838213081033283444_i64];
_2 = !_3;
RET = false;
_6 = [RET,RET,RET,RET,RET,RET];
_2 = _3 >> _3;
_5 = -449422295_i32;
_7 = [107_u8,244_u8,181_u8,166_u8,54_u8];
Goto(bb6)
}
bb12 = {
Return()
}
bb13 = {
_15 = _12.0.0 as i8;
RET = _8;
match _12.0.0 {
0 => bb11,
1 => bb4,
2 => bb14,
3 => bb15,
38085 => bb17,
_ => bb16
}
}
bb14 = {
_6 = [RET,_8,_11,_11,_11,RET];
_14 = '\u{4d97c}';
_1 = 8753_i16 as isize;
_11 = _1 > _1;
_16 = &_14;
_4.3 = [(-6050384495183168420_i64),8070571861114654105_i64,1051671717269978427_i64,(-983553512072347936_i64),5452078397212771638_i64,2133755239163307981_i64,8873777016667611529_i64,7062310222961813291_i64];
_8 = !_11;
_17 = (_5,);
Goto(bb9)
}
bb15 = {
_3 = 98250270913753117416878838889256959678_u128 as isize;
_4.0 = [RET,RET,RET,RET,RET,RET,RET];
_5 = 502757059_i32;
_3 = -_1;
Call(RET = fn9(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_6 = [RET,_8,_11,_11,_11,RET];
_14 = '\u{4d97c}';
_1 = 8753_i16 as isize;
_11 = _1 > _1;
_16 = &_14;
_4.3 = [(-6050384495183168420_i64),8070571861114654105_i64,1051671717269978427_i64,(-983553512072347936_i64),5452078397212771638_i64,2133755239163307981_i64,8873777016667611529_i64,7062310222961813291_i64];
_8 = !_11;
_17 = (_5,);
Goto(bb9)
}
bb17 = {
_17 = (_5,);
_4.2 = &_15;
_14 = '\u{1ebc1}';
_18 = _12.0.0 * _12.0.0;
_11 = _9 >= _2;
_7 = [75_u8,116_u8,222_u8,95_u8,172_u8];
_3 = -_1;
_15 = (-5_i8) ^ 53_i8;
RET = _8;
_4.2 = &_15;
_16 = &_14;
_7 = [202_u8,216_u8,180_u8,10_u8,94_u8];
Goto(bb18)
}
bb18 = {
Call(_21 = dump_var(8_usize, 17_usize, Move(_17), 2_usize, Move(_2), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_21 = dump_var(8_usize, 15_usize, Move(_15), 11_usize, Move(_11), 8_usize, Move(_8), 22_usize, _22), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i32) -> bool {
mir! {
type RET = bool;
let _2: i64;
let _3: isize;
let _4: char;
let _5: i128;
let _6: [bool; 7];
let _7: u8;
let _8: bool;
let _9: [bool; 6];
let _10: u64;
let _11: [i64; 6];
let _12: Adt79;
let _13: i8;
let _14: f64;
let _15: i32;
let _16: isize;
let _17: &'static &'static i8;
let _18: &'static i32;
let _19: f64;
let _20: u8;
let _21: &'static i32;
let _22: Adt29;
let _23: [u32; 2];
let _24: [bool; 6];
let _25: [u64; 7];
let _26: Adt29;
let _27: (bool, i128);
let _28: isize;
let _29: (u32,);
let _30: [u32; 7];
let _31: *mut usize;
let _32: *mut isize;
let _33: usize;
let _34: u64;
let _35: ();
let _36: ();
{
RET = true;
RET = _1 <= _1;
RET = !false;
_1 = (-68091611_i32);
RET = !false;
_1 = 1082573719_i32 ^ 962148840_i32;
_1 = 216_u8 as i32;
_1 = 1799010273_i32;
RET = _1 >= _1;
RET = _1 < _1;
RET = !true;
RET = _1 != _1;
_2 = !8057591640391148193_i64;
_1 = 1643741722_i32;
_2 = (-8167217624604543883_i64) + 2217823420969495410_i64;
RET = true;
RET = false & true;
RET = true;
_1 = _2 as i32;
RET = !false;
_2 = (-1663224599243680548_i64) >> _1;
_3 = 88_isize;
RET = _2 < _2;
_2 = RET as i64;
RET = !true;
RET = false;
RET = !true;
Goto(bb1)
}
bb1 = {
_5 = 1494703662_u32 as i128;
_6 = [RET,RET,RET,RET,RET,RET,RET];
_2 = (-1155167644514992346_i64);
_2 = (-3853279657412661830_i64);
_7 = 19_u8;
_3 = (-9223372036854775808_isize) << _1;
_7 = !247_u8;
RET = false | true;
_3 = 95_isize - 9223372036854775807_isize;
_5 = 109799321356409440577997472602106788071_i128;
_4 = '\u{8bdfb}';
RET = _1 <= _1;
_2 = -(-1163972382894401566_i64);
_8 = RET;
_2 = !(-1347237638617617089_i64);
_2 = (-4790449150307899139_i64) >> _1;
_2 = !5387952132080201019_i64;
_7 = 126_u8 + 252_u8;
_9 = [RET,RET,RET,RET,RET,RET];
_4 = '\u{59818}';
_1 = _3 as i32;
_7 = 75_u8;
_4 = '\u{500d1}';
_10 = _7 as u64;
Goto(bb2)
}
bb2 = {
RET = _8;
_5 = (-151258430270890557040665561438935406838_i128) >> _2;
_11 = [_2,_2,_2,_2,_2,_2];
_4 = '\u{af71b}';
_7 = !95_u8;
_5 = (-168355193823934757951078015565178888357_i128) - 99226567300092761075141110630537692575_i128;
RET = _8;
_2 = (-2572556854649641905_i64);
_1 = -551228802_i32;
_1 = -(-1359053749_i32);
_5 = (-142351404497168256945703524395771357995_i128);
_6 = [RET,_8,_8,_8,_8,_8,RET];
RET = !_8;
_6 = [_8,_8,RET,RET,RET,RET,RET];
_1 = (-460081261_i32) * 1026099794_i32;
Goto(bb3)
}
bb3 = {
RET = !_8;
_4 = '\u{5d49d}';
_3 = _4 as isize;
_1 = -967381343_i32;
_15 = (-31292_i16) as i32;
_7 = 61_u8;
_13 = -(-20_i8);
_16 = _3 << _1;
_7 = _15 as u8;
_15 = _2 as i32;
_15 = _1 ^ _1;
_13 = 17_i8;
_14 = _5 as f64;
Goto(bb4)
}
bb4 = {
RET = !_8;
_15 = !_1;
_9 = [RET,_8,RET,RET,RET,RET];
_8 = RET;
_19 = _14;
_13 = _16 as i8;
_9 = [_8,RET,_8,_8,_8,RET];
_9 = [_8,_8,_8,_8,_8,_8];
_4 = '\u{c3220}';
_20 = _7 & _7;
RET = _10 > _10;
_18 = &_15;
_7 = _5 as u8;
_3 = _16;
_7 = _20;
_11 = [_2,_2,_2,_2,_2,_2];
match _2 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463460802050577118569551 => bb9,
_ => bb8
}
}
bb5 = {
RET = !_8;
_4 = '\u{5d49d}';
_3 = _4 as isize;
_1 = -967381343_i32;
_15 = (-31292_i16) as i32;
_7 = 61_u8;
_13 = -(-20_i8);
_16 = _3 << _1;
_7 = _15 as u8;
_15 = _2 as i32;
_15 = _1 ^ _1;
_13 = 17_i8;
_14 = _5 as f64;
Goto(bb4)
}
bb6 = {
RET = _8;
_5 = (-151258430270890557040665561438935406838_i128) >> _2;
_11 = [_2,_2,_2,_2,_2,_2];
_4 = '\u{af71b}';
_7 = !95_u8;
_5 = (-168355193823934757951078015565178888357_i128) - 99226567300092761075141110630537692575_i128;
RET = _8;
_2 = (-2572556854649641905_i64);
_1 = -551228802_i32;
_1 = -(-1359053749_i32);
_5 = (-142351404497168256945703524395771357995_i128);
_6 = [RET,_8,_8,_8,_8,_8,RET];
RET = !_8;
_6 = [_8,_8,RET,RET,RET,RET,RET];
_1 = (-460081261_i32) * 1026099794_i32;
Goto(bb3)
}
bb7 = {
_5 = 1494703662_u32 as i128;
_6 = [RET,RET,RET,RET,RET,RET,RET];
_2 = (-1155167644514992346_i64);
_2 = (-3853279657412661830_i64);
_7 = 19_u8;
_3 = (-9223372036854775808_isize) << _1;
_7 = !247_u8;
RET = false | true;
_3 = 95_isize - 9223372036854775807_isize;
_5 = 109799321356409440577997472602106788071_i128;
_4 = '\u{8bdfb}';
RET = _1 <= _1;
_2 = -(-1163972382894401566_i64);
_8 = RET;
_2 = !(-1347237638617617089_i64);
_2 = (-4790449150307899139_i64) >> _1;
_2 = !5387952132080201019_i64;
_7 = 126_u8 + 252_u8;
_9 = [RET,RET,RET,RET,RET,RET];
_4 = '\u{59818}';
_1 = _3 as i32;
_7 = 75_u8;
_4 = '\u{500d1}';
_10 = _7 as u64;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_15 = _1;
Goto(bb10)
}
bb10 = {
RET = !_8;
_9 = [_8,RET,_8,_8,_8,RET];
_8 = RET;
_10 = 10411068634148964308_u64;
_14 = _2 as f64;
RET = !_8;
_9 = [RET,RET,RET,_8,RET,RET];
_2 = -(-599539674350883464_i64);
_6 = [_8,RET,RET,_8,_8,_8,_8];
_7 = _20;
RET = _8 | _8;
_18 = &_1;
_3 = _16;
_11 = [_2,_2,_2,_2,_2,_2];
_21 = &_1;
RET = _1 < (*_18);
_9 = [_8,RET,_8,RET,RET,_8];
_16 = 3454284655_u32 as isize;
Call(_19 = fn10(Move(_21), Move(_18)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9 = [_8,_8,_8,_8,_8,_8];
_18 = &_15;
_6 = [_8,RET,RET,RET,_8,RET,_8];
_5 = _7 as i128;
_15 = !_1;
_18 = &_15;
_8 = RET & RET;
_3 = _16 * _16;
_3 = 204891194971856417357440011131168864744_u128 as isize;
_6 = [_8,_8,_8,RET,_8,_8,_8];
_8 = !RET;
_8 = !RET;
_18 = &(*_18);
_19 = _5 as f64;
_21 = &(*_18);
_11 = [_2,_2,_2,_2,_2,_2];
_19 = _14 * _14;
_5 = _10 as i128;
_3 = _16;
_5 = 163490531432169723642328893677151679873_i128 * (-9285536936823054173795467318221787597_i128);
Goto(bb12)
}
bb12 = {
_6 = [RET,RET,_8,_8,_8,_8,RET];
_9 = [_8,_8,RET,RET,_8,_8];
_11 = [_2,_2,_2,_2,_2,_2];
RET = _8;
_7 = !_20;
_10 = _8 as u64;
_5 = _14 as i128;
_9 = [RET,RET,RET,_8,_8,RET];
_2 = !(-5697444905320235496_i64);
_21 = &(*_18);
_27.1 = (*_21) as i128;
_4 = '\u{80da1}';
_10 = !14096656732028267095_u64;
_2 = _13 as i64;
_25 = [_10,_10,_10,_10,_10,_10,_10];
_14 = _20 as f64;
Goto(bb13)
}
bb13 = {
_21 = &(*_18);
Goto(bb14)
}
bb14 = {
_19 = _14;
_28 = _3;
_29 = (3838099845_u32,);
_13 = (-38_i8);
_5 = -_27.1;
_16 = _13 as isize;
_30 = [_29.0,_29.0,_29.0,_29.0,_29.0,_29.0,_29.0];
_1 = _20 as i32;
RET = !_8;
_27.0 = RET;
_7 = _20;
_14 = _29.0 as f64;
_11 = [_2,_2,_2,_2,_2,_2];
_27.0 = _29.0 > _29.0;
_5 = _19 as i128;
_6 = [RET,RET,_27.0,_27.0,_8,_27.0,_27.0];
_19 = -_14;
_1 = (*_18) & (*_18);
_18 = &_1;
_21 = &(*_21);
_10 = _5 as u64;
_27 = (RET, _5);
_28 = _16;
_6 = [RET,RET,_27.0,_8,_27.0,_8,_27.0];
RET = _27.0;
_9 = [_27.0,RET,_27.0,RET,_27.0,RET];
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(9_usize, 9_usize, Move(_9), 15_usize, Move(_15), 30_usize, Move(_30), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(9_usize, 25_usize, Move(_25), 27_usize, Move(_27), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(9_usize, 5_usize, Move(_5), 3_usize, Move(_3), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: &'static i32,mut _2: &'static i32) -> f64 {
mir! {
type RET = f64;
let _3: bool;
let _4: f32;
let _5: u16;
let _6: &'static &'static i8;
let _7: &'static char;
let _8: (u32,);
let _9: Adt79;
let _10: (u16,);
let _11: char;
let _12: (((u32, (i32,), char),), Adt22, i128, &'static &'static i8);
let _13: [u16; 6];
let _14: *mut u32;
let _15: u32;
let _16: *mut (((u32,),), *mut *mut &'static i32);
let _17: [bool; 6];
let _18: &'static &'static *mut i128;
let _19: f64;
let _20: f64;
let _21: f32;
let _22: isize;
let _23: Adt22;
let _24: f32;
let _25: bool;
let _26: *const ((u32, (i32,), char),);
let _27: [bool; 7];
let _28: *const &'static &'static i8;
let _29: f32;
let _30: ([bool; 7], &'static u64, &'static i8, [i64; 8]);
let _31: usize;
let _32: i64;
let _33: ([i64; 8], u16, [u32; 7], i64);
let _34: *mut u32;
let _35: f32;
let _36: *mut &'static i32;
let _37: *mut *mut &'static i32;
let _38: f32;
let _39: [u64; 7];
let _40: Adt68;
let _41: *const &'static &'static i8;
let _42: ();
let _43: ();
{
RET = 542516856_u32 as f64;
RET = 556024878_u32 as f64;
RET = 3361536179_u32 as f64;
RET = (-11046_i16) as f64;
RET = 8_u8 as f64;
RET = 32_u8 as f64;
Goto(bb1)
}
bb1 = {
RET = (-58_isize) as f64;
_3 = RET <= RET;
RET = (-9223372036854775808_isize) as f64;
RET = 13926042177085618473_u64 as f64;
_3 = !false;
RET = (-9223372036854775808_isize) as f64;
RET = (-1818513715460451382_i64) as f64;
_3 = false;
RET = 14587_i16 as f64;
_3 = RET != RET;
_3 = false ^ true;
_3 = !false;
_3 = true;
RET = 26_i8 as f64;
_3 = false;
_3 = RET != RET;
RET = 92234867889877405493820331124664976663_u128 as f64;
_3 = RET == RET;
_4 = (-1046067450_i32) as f32;
RET = (-1239821679_i32) as f64;
Goto(bb2)
}
bb2 = {
_3 = !false;
RET = 1933777037_i32 as f64;
RET = 688670449995310184_u64 as f64;
_5 = 40725_u16;
_8.0 = 1337350847_u32;
_3 = _5 >= _5;
RET = 249_u8 as f64;
_4 = 97_u8 as f32;
_8 = (2676127535_u32,);
_3 = !false;
_4 = 167_u8 as f32;
RET = 96492819083407760671023194921583363092_u128 as f64;
_8 = (653057403_u32,);
_3 = true;
_4 = 15_i8 as f32;
_8.0 = (-1436343770_i32) as u32;
_3 = true;
_4 = 34_i8 as f32;
_4 = _8.0 as f32;
_10 = (_5,);
Goto(bb3)
}
bb3 = {
_8.0 = !2738017899_u32;
_3 = _10.0 >= _10.0;
_5 = _10.0 << _8.0;
RET = (-4106062344885711075_i64) as f64;
_11 = '\u{6b7d8}';
_1 = &_12.0.0.1.0;
RET = 145_u8 as f64;
RET = 721428296_i32 as f64;
_12.0.0.2 = _11;
RET = _8.0 as f64;
_1 = &(*_1);
_7 = &_12.0.0.2;
_8.0 = 2203594616_u32 ^ 1316493582_u32;
_12.0.0.2 = _11;
RET = 6873258841427151591_i64 as f64;
_12.0.0.0 = !_8.0;
_2 = &(*_1);
_10.0 = _5 - _5;
_11 = _12.0.0.2;
_13 = [_10.0,_10.0,_10.0,_10.0,_5,_10.0];
_4 = 17199897911082926639_u64 as f32;
_14 = core::ptr::addr_of_mut!(_8.0);
_8.0 = _12.0.0.0 << _12.0.0.0;
_12.2 = 6_usize as i128;
_1 = &_12.0.0.1.0;
_10 = (_5,);
_12.2 = 17552651401228365840761648645482010353_i128 + 90322201919238822381436396230192258465_i128;
Goto(bb4)
}
bb4 = {
_12.0.0.1 = ((-258177611_i32),);
_7 = &_11;
_1 = &_12.0.0.1.0;
_4 = _12.0.0.0 as f32;
_2 = &_12.0.0.1.0;
_10 = (_5,);
(*_14) = _12.2 as u32;
_12.0.0.0 = !(*_14);
_12.0.0.0 = _8.0 & _8.0;
_10 = (_5,);
_8 = (_12.0.0.0,);
(*_14) = _12.0.0.0;
_10.0 = _5 + _5;
_11 = _12.0.0.2;
_8 = (_12.0.0.0,);
_12.0.0.1.0 = -1641237314_i32;
_12.0.0.0 = (*_14);
_12.0.0.1 = ((-1795038360_i32),);
_1 = &_12.0.0.1.0;
_19 = -RET;
(*_14) = _12.0.0.0 - _12.0.0.0;
RET = -_19;
_12.2 = (*_1) as i128;
_12.0.0.2 = _11;
_14 = core::ptr::addr_of_mut!(_15);
RET = -_19;
match (*_1) {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607429973173096 => bb6,
_ => bb5
}
}
bb5 = {
RET = (-58_isize) as f64;
_3 = RET <= RET;
RET = (-9223372036854775808_isize) as f64;
RET = 13926042177085618473_u64 as f64;
_3 = !false;
RET = (-9223372036854775808_isize) as f64;
RET = (-1818513715460451382_i64) as f64;
_3 = false;
RET = 14587_i16 as f64;
_3 = RET != RET;
_3 = false ^ true;
_3 = !false;
_3 = true;
RET = 26_i8 as f64;
_3 = false;
_3 = RET != RET;
RET = 92234867889877405493820331124664976663_u128 as f64;
_3 = RET == RET;
_4 = (-1046067450_i32) as f32;
RET = (-1239821679_i32) as f64;
Goto(bb2)
}
bb6 = {
_10 = (_5,);
_7 = &_11;
(*_14) = !_12.0.0.0;
_12.0.0.1 = ((-1917651238_i32),);
_4 = (-64_i8) as f32;
_13 = [_5,_10.0,_5,_10.0,_10.0,_10.0];
_20 = 26188_i16 as f64;
_2 = &_12.0.0.1.0;
_21 = 3_usize as f32;
_13 = [_10.0,_5,_5,_5,_5,_5];
_1 = Move(_2);
_12.0.0.0 = _20 as u32;
Call(_12.0.0 = fn11(), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = &_12.0.0.1.0;
_12.0.0.1 = (1516611945_i32,);
_8.0 = !(*_14);
_19 = _10.0 as f64;
_12.2 = -13604312067277708438341464820197306835_i128;
_8.0 = !_12.0.0.0;
_1 = &_12.0.0.1.0;
_11 = _12.0.0.2;
_7 = &_11;
_8 = ((*_14),);
_24 = (-17561_i16) as f32;
_2 = Move(_1);
_21 = _4;
_22 = _12.0.0.1.0 as isize;
_19 = _20 * RET;
_1 = &_12.0.0.1.0;
_12.0.0.1 = ((-1988500319_i32),);
_12.0.0.2 = _11;
RET = -_20;
_12.0.0.2 = _11;
_24 = _21 * _21;
_26 = core::ptr::addr_of!(_12.0);
RET = _19 * _20;
_10.0 = _5 >> (*_14);
_12.0.0.2 = (*_7);
match _12.0.0.1.0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb8,
340282366920938463463374607429779711137 => bb10,
_ => bb9
}
}
bb8 = {
_3 = !false;
RET = 1933777037_i32 as f64;
RET = 688670449995310184_u64 as f64;
_5 = 40725_u16;
_8.0 = 1337350847_u32;
_3 = _5 >= _5;
RET = 249_u8 as f64;
_4 = 97_u8 as f32;
_8 = (2676127535_u32,);
_3 = !false;
_4 = 167_u8 as f32;
RET = 96492819083407760671023194921583363092_u128 as f64;
_8 = (653057403_u32,);
_3 = true;
_4 = 15_i8 as f32;
_8.0 = (-1436343770_i32) as u32;
_3 = true;
_4 = 34_i8 as f32;
_4 = _8.0 as f32;
_10 = (_5,);
Goto(bb3)
}
bb9 = {
RET = (-58_isize) as f64;
_3 = RET <= RET;
RET = (-9223372036854775808_isize) as f64;
RET = 13926042177085618473_u64 as f64;
_3 = !false;
RET = (-9223372036854775808_isize) as f64;
RET = (-1818513715460451382_i64) as f64;
_3 = false;
RET = 14587_i16 as f64;
_3 = RET != RET;
_3 = false ^ true;
_3 = !false;
_3 = true;
RET = 26_i8 as f64;
_3 = false;
_3 = RET != RET;
RET = 92234867889877405493820331124664976663_u128 as f64;
_3 = RET == RET;
_4 = (-1046067450_i32) as f32;
RET = (-1239821679_i32) as f64;
Goto(bb2)
}
bb10 = {
_2 = &(*_26).0.1.0;
_14 = core::ptr::addr_of_mut!((*_14));
_8.0 = !(*_14);
_1 = Move(_2);
_12.0.0.1 = ((-759746796_i32),);
_21 = _24 * _24;
(*_26).0.0 = _15;
_5 = _10.0 & _10.0;
Goto(bb11)
}
bb11 = {
_12.2 = 2944878349334238340_u64 as i128;
_13 = [_10.0,_5,_5,_5,_5,_5];
_17 = [_3,_3,_3,_3,_3,_3];
_2 = &(*_26).0.1.0;
_21 = _24;
_25 = _5 > _10.0;
_10.0 = _5 >> _15;
_12.0.0.0 = !(*_14);
_1 = Move(_2);
_20 = RET + _19;
_15 = (*_26).0.0;
_3 = !_25;
(*_26).0.2 = _11;
_28 = core::ptr::addr_of!(_6);
Goto(bb12)
}
bb12 = {
_1 = &_12.0.0.1.0;
(*_26).0.1 = (1038032303_i32,);
_2 = &_12.0.0.1.0;
(*_26).0.1.0 = !164647661_i32;
_1 = &(*_26).0.1.0;
(*_26).0.1.0 = 5_usize as i32;
_10.0 = _5 + _5;
(*_26).0.0 = (*_14);
_10 = (_5,);
_22 = !(-9223372036854775808_isize);
_8 = ((*_14),);
_3 = _15 < _15;
_1 = &(*_26).0.1.0;
_17 = [_3,_3,_25,_25,_3,_3];
_12.3 = &_30.2;
Goto(bb13)
}
bb13 = {
_32 = !(-4008027619745557493_i64);
_12.3 = &_30.2;
_28 = core::ptr::addr_of!((*_28));
_1 = &_12.0.0.1.0;
(*_26).0.1 = ((-1531761923_i32),);
_19 = -_20;
(*_14) = (*_26).0.0 - (*_26).0.0;
_33.2 = [(*_26).0.0,(*_26).0.0,_15,(*_14),_15,_15,(*_14)];
(*_26).0.2 = (*_7);
_12.0.0.1.0 = 1660108259_i32;
_2 = &(*_26).0.1.0;
_22 = (-9223372036854775808_isize);
(*_28) = &_30.2;
_36 = core::ptr::addr_of_mut!(_2);
_8.0 = !_15;
Call(_30.3 = fn15(Move(_36), Move(_7), Move((*_36))), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_28) = &_30.2;
_7 = &(*_26).0.2;
_36 = core::ptr::addr_of_mut!(_1);
_34 = Move(_14);
_33.1 = _10.0 ^ _10.0;
(*_26).0.2 = _11;
_37 = core::ptr::addr_of_mut!(_36);
_12.0.0.1 = (1065626956_i32,);
_33.0 = [_32,_32,_32,_32,_32,_32,_32,_32];
RET = 169_u8 as f64;
_8 = (_12.0.0.0,);
_33.3 = !_32;
_38 = -_21;
_8 = ((*_26).0.0,);
_28 = core::ptr::addr_of!(_12.3);
_2 = &(*_26).0.1.0;
_33.2 = [(*_26).0.0,_12.0.0.0,(*_26).0.0,_8.0,_15,_15,(*_26).0.0];
_31 = !11114766794315874555_usize;
_26 = core::ptr::addr_of!((*_26));
_15 = _38 as u32;
_36 = core::ptr::addr_of_mut!((*_36));
_27 = [_3,_25,_25,_25,_3,_25,_3];
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(10_usize, 33_usize, Move(_33), 15_usize, Move(_15), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(10_usize, 13_usize, Move(_13), 25_usize, Move(_25), 31_usize, Move(_31), 43_usize, _43), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11() -> (u32, (i32,), char) {
mir! {
type RET = (u32, (i32,), char);
let _1: &'static Adt22;
let _2: char;
let _3: f64;
let _4: &'static Adt22;
let _5: (((u32,),), *mut *mut &'static i32);
let _6: &'static u64;
let _7: i128;
let _8: [u64; 6];
let _9: f64;
let _10: [u64; 7];
let _11: *mut usize;
let _12: (i32,);
let _13: &'static *mut isize;
let _14: isize;
let _15: isize;
let _16: ([u32; 7], [u8; 7], &'static Adt22);
let _17: u32;
let _18: *const &'static &'static i8;
let _19: [bool; 7];
let _20: u8;
let _21: [i64; 6];
let _22: i16;
let _23: &'static &'static i8;
let _24: ();
let _25: ();
{
RET.1.0 = 1051673014_i32;
RET.1 = (1252668666_i32,);
RET.0 = 441733954_u32 << RET.1.0;
RET.2 = '\u{c65e6}';
Goto(bb1)
}
bb1 = {
RET.1.0 = (-1669423017_i32) ^ (-1171629888_i32);
RET.0 = !3957663678_u32;
RET.1 = (2108437415_i32,);
_2 = RET.2;
_2 = RET.2;
RET.1.0 = (-108129149_i32);
RET.1.0 = (-231730728_i32);
RET.2 = _2;
_2 = RET.2;
_5.0.0 = (RET.0,);
_5.0.0 = (RET.0,);
match RET.1.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431536480728 => bb10,
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
RET.1 = (916124487_i32,);
_5.0.0 = (RET.0,);
RET.1.0 = 103612255_i32 << RET.0;
RET.1.0 = -284921507_i32;
RET.2 = _2;
_5.0.0 = (RET.0,);
RET.1.0 = (-495877719_i32);
RET.0 = 15532086040082589588_u64 as u32;
RET.1 = ((-1461525791_i32),);
RET.2 = _2;
RET.0 = _5.0.0.0 << RET.1.0;
RET.1.0 = 34625475156668920281957986508181231209_u128 as i32;
_5.0.0.0 = 9223372036854775807_isize as u32;
RET.1.0 = 844346988_i32 * (-102327899_i32);
_2 = RET.2;
Goto(bb11)
}
bb11 = {
_5.0.0.0 = RET.0;
_8 = [4842134989352262471_u64,17765595814299051017_u64,1091971791101757418_u64,12466757672779569514_u64,16647403523417075334_u64,18275064121780803208_u64];
RET.0 = 50687_u16 as u32;
_10 = [17665023336345871864_u64,7059040786646608790_u64,15626843270431415597_u64,10566430476127640823_u64,15953520605774003849_u64,10041167584592305058_u64,15203329870085359558_u64];
_7 = 132521482387337059808696493214271818423_i128 + 23698562999708509978326534451774846978_i128;
_10 = [17883242663100970756_u64,13381089978544840654_u64,6710747607916039898_u64,838131989481540777_u64,9665193614208142857_u64,13924722006525647731_u64,10571444599853889650_u64];
_5.0.0.0 = 18408140194664038725_usize as u32;
Goto(bb12)
}
bb12 = {
RET.0 = _5.0.0.0;
RET.1.0 = !(-1139677185_i32);
_10 = [15278914010461209802_u64,1166377218805956828_u64,1756786307335481092_u64,4123647846530097549_u64,4341503061512800426_u64,13999816267826889153_u64,8122212829718056781_u64];
_3 = _5.0.0.0 as f64;
_2 = RET.2;
_9 = _3;
RET.0 = RET.1.0 as u32;
_7 = 135413285090661008378545490101792081639_i128;
_5.0.0 = (RET.0,);
RET.1.0 = 52281_u16 as i32;
RET.1.0 = 1331751036_i32 & (-1715146749_i32);
_9 = -_3;
_10 = [17129064569398247712_u64,2199891021339791338_u64,4914978142471482794_u64,7400103166036779080_u64,7492005291478010159_u64,8529147043994407547_u64,12770922370778060337_u64];
_8 = [13220444172034317917_u64,13171420721747840735_u64,16401698847056077648_u64,9931180528466311481_u64,16605078383899208339_u64,18014330472894694629_u64];
RET.1 = ((-1373846533_i32),);
RET.1 = (1551008074_i32,);
_15 = (-1939_i16) as isize;
_12 = (RET.1.0,);
_2 = RET.2;
Goto(bb13)
}
bb13 = {
RET.2 = _2;
_8 = [4004902235265866494_u64,11454238223908463138_u64,2889823626607869421_u64,13508407409271328254_u64,6520329370085707037_u64,12410783046389808589_u64];
_7 = 15863959224773062614_usize as i128;
RET = (_5.0.0.0, _12, _2);
_7 = (-85007742900742214373631983248777147595_i128);
_7 = _12.0 as i128;
_2 = RET.2;
_5.0.0 = (RET.0,);
_2 = RET.2;
RET.1.0 = -_12.0;
_16.1 = [113_u8,172_u8,37_u8,0_u8,169_u8,94_u8,18_u8];
_8 = [1713358061448540629_u64,14045781812743351907_u64,1641075339799729846_u64,15433537827091956898_u64,9206508791626264225_u64,16468561417018186130_u64];
_12.0 = RET.1.0;
RET.1.0 = -_12.0;
_8 = [3638327975644981755_u64,4462380353528471063_u64,6066952691107880963_u64,6526988035073769913_u64,1722214344965688874_u64,15438261443473738130_u64];
RET.0 = _5.0.0.0 | _5.0.0.0;
RET = (_5.0.0.0, _12, _2);
Call(RET.0 = fn12(_16.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_16.0 = [RET.0,RET.0,RET.0,RET.0,RET.0,RET.0,RET.0];
_9 = _3;
_21 = [3049771299266125253_i64,(-8857292689943682560_i64),(-933061867799503716_i64),(-8180540734645255205_i64),(-291003826757228469_i64),(-3101743419923850419_i64)];
RET.2 = _2;
RET.1 = (_12.0,);
_14 = _15 << RET.0;
RET.1.0 = _12.0 + _12.0;
RET.2 = _2;
RET.1.0 = _12.0;
_20 = 311556572574553350910201527466432942007_u128 as u8;
_8 = [12237052119561488487_u64,13352431714005528323_u64,3399458795982277881_u64,12232987433748480072_u64,17789272815049978619_u64,2426888985791023295_u64];
_3 = RET.0 as f64;
_12.0 = _14 as i32;
RET = (_5.0.0.0, _12, _2);
_22 = (-1259_i16) ^ 13773_i16;
_16.0 = [_5.0.0.0,_5.0.0.0,RET.0,RET.0,_5.0.0.0,RET.0,_5.0.0.0];
RET.0 = _14 as u32;
_19 = [false,false,false,true,true,false,true];
_12 = RET.1;
_22 = (-27957_i16);
RET = (_5.0.0.0, _12, _2);
RET.2 = _2;
RET.1 = _12;
_12 = (RET.1.0,);
_15 = _14;
_16.1 = [_20,_20,_20,_20,_20,_20,_20];
_9 = _20 as f64;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(11_usize, 20_usize, Move(_20), 8_usize, Move(_8), 21_usize, Move(_21), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(11_usize, 15_usize, Move(_15), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [u8; 7]) -> u32 {
mir! {
type RET = u32;
let _2: u16;
let _3: i16;
let _4: usize;
let _5: u64;
let _6: *mut (u32, (i32,), char);
let _7: Adt79;
let _8: &'static [i64; 4];
let _9: *mut ([u32; 7], [u8; 7], &'static Adt22);
let _10: bool;
let _11: i128;
let _12: isize;
let _13: char;
let _14: f32;
let _15: Adt68;
let _16: isize;
let _17: ();
let _18: ();
{
RET = 3266858019_u32;
_1 = [72_u8,89_u8,141_u8,80_u8,7_u8,98_u8,176_u8];
RET = !2847035669_u32;
RET = !3486428467_u32;
_1 = [64_u8,226_u8,68_u8,136_u8,184_u8,21_u8,219_u8];
_1 = [105_u8,60_u8,72_u8,243_u8,159_u8,15_u8,180_u8];
RET = 1349115485_u32 >> 85_u8;
_2 = 4947_u16 - 64446_u16;
Goto(bb1)
}
bb1 = {
_3 = (-13752_i16) << _2;
_2 = !15714_u16;
_3 = 32003_i16;
_3 = 17419_i16;
_5 = !5136142548942378385_u64;
_2 = 22932_u16 | 42834_u16;
_1 = [104_u8,123_u8,253_u8,53_u8,140_u8,18_u8,229_u8];
_2 = 39278_u16;
RET = 7_usize as u32;
RET = 67_i8 as u32;
_3 = _5 as i16;
RET = 2193531766_u32 >> _5;
Goto(bb2)
}
bb2 = {
RET = (-4801975689107392981_i64) as u32;
_5 = 18196388770911869344_u64 << RET;
_10 = false;
RET = 954283937_i32 as u32;
RET = 2800153669_u32 - 2838970455_u32;
RET = !1749967236_u32;
_4 = 2_usize;
_2 = 33772_u16;
_10 = !false;
_5 = (-158554537539219910641075757901256674856_i128) as u64;
_1[_4] = RET as u8;
_11 = 96764299159660946880676158952403823240_i128 | 133578529329517586021126145541839482383_i128;
_5 = (-995963063_i32) as u64;
_10 = _5 == _5;
_1[_4] = !123_u8;
_3 = -(-5971_i16);
_1[_4] = _4 as u8;
_12 = !9223372036854775807_isize;
_2 = !9515_u16;
_12 = _5 as isize;
RET = 2760383612_u32;
_12 = '\u{bc61e}' as isize;
_1 = [219_u8,30_u8,71_u8,173_u8,227_u8,57_u8,1_u8];
RET = 3370708318_u32;
_13 = '\u{c8d98}';
_13 = '\u{45e77}';
_1 = [73_u8,108_u8,9_u8,146_u8,136_u8,46_u8,230_u8];
match _1[_4] {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9 => bb10,
_ => bb9
}
}
bb3 = {
_3 = (-13752_i16) << _2;
_2 = !15714_u16;
_3 = 32003_i16;
_3 = 17419_i16;
_5 = !5136142548942378385_u64;
_2 = 22932_u16 | 42834_u16;
_1 = [104_u8,123_u8,253_u8,53_u8,140_u8,18_u8,229_u8];
_2 = 39278_u16;
RET = 7_usize as u32;
RET = 67_i8 as u32;
_3 = _5 as i16;
RET = 2193531766_u32 >> _5;
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
RET = 1609088189_u32 | 1591776223_u32;
_10 = !false;
_2 = RET as u16;
Call(RET = fn13(_11, _1[_4], _1, _1[_4], _5, _1[_4], _4), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_10 = true;
_5 = 880819712314819840_u64 << _11;
_3 = (-5577_i16) * (-7462_i16);
_11 = !74495473550614665618539701525825036269_i128;
_13 = '\u{f035f}';
_14 = _3 as f32;
Goto(bb12)
}
bb12 = {
Call(_17 = dump_var(12_usize, 1_usize, Move(_1), 10_usize, Move(_10), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i128,mut _2: u8,mut _3: [u8; 7],mut _4: u8,mut _5: u64,mut _6: u8,mut _7: usize) -> u32 {
mir! {
type RET = u32;
let _8: &'static *mut i128;
let _9: f32;
let _10: ((u32,),);
let _11: [bool; 6];
let _12: ((u32,),);
let _13: i8;
let _14: bool;
let _15: [u64; 6];
let _16: f64;
let _17: (((u32,),), *mut *mut &'static i32);
let _18: [u16; 6];
let _19: ();
let _20: ();
{
_5 = !7317613473418040925_u64;
_1 = 165632221956302696973636870855182098047_i128;
_5 = 679593682_u32 as u64;
_6 = _4 << _3[_7];
_4 = _5 as u8;
_3 = [_2,_6,_6,_6,_6,_6,_6];
_3[_7] = !_6;
RET = !1666118214_u32;
_1 = 15008137044798573385267745285920231120_i128 & (-110055076639795300541623686542850104488_i128);
_6 = 39247_u16 as u8;
_3[_7] = RET as u8;
_9 = (-108_i8) as f32;
_12.0.0 = RET ^ RET;
_10.0.0 = RET * _12.0.0;
_12.0.0 = _10.0.0;
_2 = !_6;
_11 = [false,false,false,false,false,false];
_5 = !14487711874479999608_u64;
_7 = !5_usize;
_6 = _2;
_6 = !_2;
_14 = _9 == _9;
Call(_12.0 = fn14(_1, _3, _10, _10.0.0, _3, _10.0, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !_12.0.0;
_7 = RET as usize;
_9 = (-22_i8) as f32;
_7 = 2209923023957824128_usize ^ 1_usize;
_2 = _6;
_9 = 47472_u16 as f32;
_13 = !109_i8;
_5 = 17134971025004136826_u64 & 13820614829548472565_u64;
_10.0.0 = RET ^ RET;
_2 = !_6;
_1 = _7 as i128;
_9 = _13 as f32;
_15 = [_5,_5,_5,_5,_5,_5];
_12.0.0 = RET | _10.0.0;
_6 = _2;
_12.0 = (RET,);
_17.0.0.0 = !_10.0.0;
_16 = _1 as f64;
_10.0.0 = _12.0.0 + RET;
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(13_usize, 13_usize, Move(_13), 3_usize, Move(_3), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(13_usize, 1_usize, Move(_1), 14_usize, Move(_14), 20_usize, _20, 20_usize, _20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i128,mut _2: [u8; 7],mut _3: ((u32,),),mut _4: u32,mut _5: [u8; 7],mut _6: (u32,),mut _7: [u8; 7],mut _8: [u8; 7]) -> (u32,) {
mir! {
type RET = (u32,);
let _9: u64;
let _10: i32;
let _11: bool;
let _12: u128;
let _13: *mut ([u32; 7], [u8; 7], &'static Adt22);
let _14: i128;
let _15: i16;
let _16: f32;
let _17: u32;
let _18: *mut ([u32; 7], [u8; 7], &'static Adt22);
let _19: ();
let _20: ();
{
_7 = [39_u8,249_u8,131_u8,139_u8,201_u8,26_u8,74_u8];
_2 = [78_u8,130_u8,21_u8,1_u8,200_u8,35_u8,40_u8];
RET.0 = _4;
_6.0 = _4 + _3.0.0;
_4 = 3010170712563003939_i64 as u32;
_3.0.0 = !_4;
RET.0 = !_6.0;
_3.0.0 = RET.0;
RET = (_6.0,);
_6.0 = _3.0.0 + RET.0;
_9 = 135878182228037249281561624218925644031_u128 as u64;
_6 = _3.0;
_8 = [74_u8,106_u8,206_u8,6_u8,202_u8,146_u8,97_u8];
_3 = (RET,);
_1 = 118126323919922617227643133548671500826_i128 ^ (-67294109892975383915607452263174605138_i128);
_5 = [64_u8,48_u8,75_u8,117_u8,250_u8,50_u8,217_u8];
Call(_4 = core::intrinsics::bswap(_6.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = _3.0.0 ^ _6.0;
RET.0 = 2095_u16 as u32;
_11 = !true;
_9 = 65415617768164297_u64 - 590097289868783915_u64;
_3.0 = (_6.0,);
_7 = [78_u8,15_u8,48_u8,61_u8,172_u8,189_u8,247_u8];
RET.0 = _3.0.0;
RET.0 = 84793910275753954728733801291663578417_u128 as u32;
_11 = true | true;
_5 = [124_u8,123_u8,41_u8,101_u8,126_u8,81_u8,89_u8];
RET.0 = _4;
_12 = 330786238170501470826585368438984120451_u128;
RET.0 = _3.0.0 | _4;
_3.0 = RET;
_7 = [126_u8,212_u8,255_u8,149_u8,116_u8,26_u8,219_u8];
_2 = [183_u8,70_u8,236_u8,181_u8,77_u8,191_u8,143_u8];
_7 = [127_u8,187_u8,184_u8,214_u8,160_u8,40_u8,115_u8];
_7 = [99_u8,253_u8,223_u8,122_u8,222_u8,73_u8,58_u8];
_12 = !182087146384944444109225112524158960518_u128;
Goto(bb2)
}
bb2 = {
_11 = true | false;
RET = _6;
_4 = _6.0 >> _3.0.0;
_3.0.0 = !_4;
_8 = [62_u8,87_u8,97_u8,4_u8,120_u8,212_u8,214_u8];
_3 = (RET,);
RET.0 = !_4;
_10 = RET.0 as i32;
_9 = _4 as u64;
_15 = _9 as i16;
_3 = (RET,);
_3.0.0 = _6.0;
_15 = 31450_i16;
_5 = [2_u8,173_u8,8_u8,69_u8,36_u8,76_u8,160_u8];
_10 = -(-14023609_i32);
_1 = -(-152988893590723256306153582065666353921_i128);
_3 = (RET,);
_7 = [208_u8,112_u8,2_u8,81_u8,14_u8,227_u8,3_u8];
_15 = (-701_i16) ^ 18072_i16;
_15 = '\u{e7a9a}' as i16;
_2 = _8;
_5 = _2;
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(14_usize, 9_usize, Move(_9), 8_usize, Move(_8), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_19 = dump_var(14_usize, 2_usize, Move(_2), 7_usize, Move(_7), 20_usize, _20, 20_usize, _20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: *mut &'static i32,mut _2: &'static char,mut _3: &'static i32) -> [i64; 8] {
mir! {
type RET = [i64; 8];
let _4: u32;
let _5: Adt79;
let _6: f64;
let _7: *mut isize;
let _8: i64;
let _9: &'static &'static *mut i128;
let _10: u8;
let _11: Adt74;
let _12: f64;
let _13: u128;
let _14: ();
let _15: ();
{
RET = [6320214149288799769_i64,(-767913296996561862_i64),2396184422341216391_i64,(-7748402831493771390_i64),3637439267489098047_i64,9053237603346431937_i64,9048151265139682144_i64,(-2522948416363688153_i64)];
_1 = core::ptr::addr_of_mut!(_3);
RET = [(-6962198090609933321_i64),4560810671591193359_i64,(-3653283305367814335_i64),6354040917544003363_i64,(-10519427772125247_i64),679446496779236848_i64,2782700939161195861_i64,(-5933733468694322039_i64)];
RET = [3317423588666936042_i64,(-7141222414094871007_i64),(-7387288330778672847_i64),(-8501380272806855891_i64),(-7759420458218765906_i64),3499186145877712337_i64,8196466374670762154_i64,7480727371507469992_i64];
_1 = core::ptr::addr_of_mut!(_3);
RET = [(-8695811689660662336_i64),1524900077062488038_i64,7917964711732219863_i64,6803422492105224487_i64,4966917041882574659_i64,(-598609445216006267_i64),3690215080215738733_i64,(-3866311995203046106_i64)];
RET = [3810327027541115618_i64,5955398996983478573_i64,2326897181020873693_i64,(-2797884457769312640_i64),1339533834051777963_i64,5034767115922376888_i64,7730326591674151392_i64,(-5938727450586668328_i64)];
_4 = 1618712294_u32;
RET = [(-4836582113230393999_i64),4200399021746383807_i64,18070979592271873_i64,5311482362862197694_i64,4293473636186880074_i64,(-4273605429284280463_i64),(-2413397475852804934_i64),(-2321372466428219308_i64)];
_1 = core::ptr::addr_of_mut!((*_1));
RET = [123946062002878891_i64,(-873902276707704793_i64),(-4001650834114322479_i64),(-6730844753776913853_i64),3431553994556757648_i64,6656238027395608738_i64,(-4617305480280630828_i64),(-8265680057535277721_i64)];
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb1)
}
bb1 = {
_4 = !2507339477_u32;
_4 = 25698244_u32;
RET = [5201757277085987956_i64,(-7664329459198524571_i64),3622319840917525426_i64,(-8286354029192312345_i64),3421446888662188033_i64,8506454541793999048_i64,(-8260305587847923423_i64),6462837405907944271_i64];
RET = [3511464576248250278_i64,2063154684205576097_i64,(-6331666138381810621_i64),4055856903084216212_i64,148548742077339071_i64,677456946130889887_i64,4168665308817101164_i64,(-5406858297931694694_i64)];
_4 = 679035088_u32;
_4 = 89_u8 as u32;
_1 = core::ptr::addr_of_mut!(_3);
RET = [(-206597228831958313_i64),5290480635275875301_i64,(-9024949421222371778_i64),7763391173961360497_i64,(-3836655616626808481_i64),(-3960971147438198511_i64),(-7893636355151349373_i64),(-9157972668434555315_i64)];
RET = [1210527914327094417_i64,168646466439477110_i64,5645907916580424335_i64,(-4343436872144653305_i64),(-2938380459235509455_i64),(-7061339720736347703_i64),2850337944522282008_i64,(-8343710247604512165_i64)];
_1 = core::ptr::addr_of_mut!((*_1));
_4 = 340755834_u32 >> 4_usize;
RET = [(-2687070565181133296_i64),(-2923363997040257267_i64),(-6740712230746967661_i64),5819925972024146022_i64,(-6024645042110196561_i64),593595898659239119_i64,(-2413393293781347819_i64),(-4476347759877319889_i64)];
RET = [(-8106427474267501006_i64),102600852872008704_i64,(-1506796483968688445_i64),7319544399396683297_i64,6222836488731093818_i64,3540434217483920554_i64,(-2530559970276484276_i64),(-8557679248474540482_i64)];
RET = [(-4803501154194213681_i64),4831544829824085734_i64,3134541338855110503_i64,(-5434330756872119175_i64),(-306566836216597198_i64),(-10761801404756534_i64),(-12068911785979525_i64),4512788818953043817_i64];
RET = [(-4968591803358353553_i64),2325632731987519942_i64,5695687050284535463_i64,8424848658867522093_i64,(-2377453538854481015_i64),(-4074823523067762828_i64),6335507005808550095_i64,(-5668709809033556949_i64)];
_6 = (-9223372036854775808_isize) as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!((*_1));
_4 = 3816255574_u32;
RET = [(-6504922626580132502_i64),1999229841839493269_i64,6550346734359503202_i64,9032687854610745950_i64,(-1340664303006427201_i64),4562664698926452686_i64,(-127582307902657209_i64),5111134829090861032_i64];
_4 = !2690112856_u32;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = (-49_i8) as u32;
RET = [4070389449636545806_i64,8414568376850632809_i64,(-6888240329098528178_i64),9089283096873019274_i64,(-6112546919085446351_i64),(-682737344642346385_i64),7874798800667999678_i64,(-4484532545467549615_i64)];
_4 = (-81048197660082932579470775555651040391_i128) as u32;
RET = [(-2433412359985815294_i64),3710936826201298608_i64,(-3484404742145842063_i64),(-2556606719628825150_i64),5908386050340769379_i64,(-7034491595679085084_i64),(-3643991549071522598_i64),8307802142129900639_i64];
Goto(bb2)
}
bb2 = {
_6 = 155518409458106263166007852587767833066_u128 as f64;
_8 = 9146554567595939343_i64;
_4 = 3383004213_u32;
match _4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
3383004213 => bb11,
_ => bb10
}
}
bb3 = {
_4 = !2507339477_u32;
_4 = 25698244_u32;
RET = [5201757277085987956_i64,(-7664329459198524571_i64),3622319840917525426_i64,(-8286354029192312345_i64),3421446888662188033_i64,8506454541793999048_i64,(-8260305587847923423_i64),6462837405907944271_i64];
RET = [3511464576248250278_i64,2063154684205576097_i64,(-6331666138381810621_i64),4055856903084216212_i64,148548742077339071_i64,677456946130889887_i64,4168665308817101164_i64,(-5406858297931694694_i64)];
_4 = 679035088_u32;
_4 = 89_u8 as u32;
_1 = core::ptr::addr_of_mut!(_3);
RET = [(-206597228831958313_i64),5290480635275875301_i64,(-9024949421222371778_i64),7763391173961360497_i64,(-3836655616626808481_i64),(-3960971147438198511_i64),(-7893636355151349373_i64),(-9157972668434555315_i64)];
RET = [1210527914327094417_i64,168646466439477110_i64,5645907916580424335_i64,(-4343436872144653305_i64),(-2938380459235509455_i64),(-7061339720736347703_i64),2850337944522282008_i64,(-8343710247604512165_i64)];
_1 = core::ptr::addr_of_mut!((*_1));
_4 = 340755834_u32 >> 4_usize;
RET = [(-2687070565181133296_i64),(-2923363997040257267_i64),(-6740712230746967661_i64),5819925972024146022_i64,(-6024645042110196561_i64),593595898659239119_i64,(-2413393293781347819_i64),(-4476347759877319889_i64)];
RET = [(-8106427474267501006_i64),102600852872008704_i64,(-1506796483968688445_i64),7319544399396683297_i64,6222836488731093818_i64,3540434217483920554_i64,(-2530559970276484276_i64),(-8557679248474540482_i64)];
RET = [(-4803501154194213681_i64),4831544829824085734_i64,3134541338855110503_i64,(-5434330756872119175_i64),(-306566836216597198_i64),(-10761801404756534_i64),(-12068911785979525_i64),4512788818953043817_i64];
RET = [(-4968591803358353553_i64),2325632731987519942_i64,5695687050284535463_i64,8424848658867522093_i64,(-2377453538854481015_i64),(-4074823523067762828_i64),6335507005808550095_i64,(-5668709809033556949_i64)];
_6 = (-9223372036854775808_isize) as f64;
_1 = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!((*_1));
_4 = 3816255574_u32;
RET = [(-6504922626580132502_i64),1999229841839493269_i64,6550346734359503202_i64,9032687854610745950_i64,(-1340664303006427201_i64),4562664698926452686_i64,(-127582307902657209_i64),5111134829090861032_i64];
_4 = !2690112856_u32;
_1 = core::ptr::addr_of_mut!((*_1));
_4 = (-49_i8) as u32;
RET = [4070389449636545806_i64,8414568376850632809_i64,(-6888240329098528178_i64),9089283096873019274_i64,(-6112546919085446351_i64),(-682737344642346385_i64),7874798800667999678_i64,(-4484532545467549615_i64)];
_4 = (-81048197660082932579470775555651040391_i128) as u32;
RET = [(-2433412359985815294_i64),3710936826201298608_i64,(-3484404742145842063_i64),(-2556606719628825150_i64),5908386050340769379_i64,(-7034491595679085084_i64),(-3643991549071522598_i64),8307802142129900639_i64];
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
Return()
}
bb11 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_4 = !2360575756_u32;
_1 = core::ptr::addr_of_mut!(_3);
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
Goto(bb12)
}
bb12 = {
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_4 = 2678070427_u32;
_10 = 23_u8 - 132_u8;
_8 = '\u{df66a}' as i64;
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_11.fld0 = (-5142_i16) - 14402_i16;
match _4 {
0 => bb10,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb11,
6 => bb8,
2678070427 => bb14,
_ => bb13
}
}
bb13 = {
_1 = core::ptr::addr_of_mut!((*_1));
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_4 = !2360575756_u32;
_1 = core::ptr::addr_of_mut!(_3);
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
Goto(bb12)
}
bb14 = {
_8 = -(-6991441532219251726_i64);
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_4 = 17574844792536192359_usize as u32;
_11.fld0 = (-2513_i16);
_6 = 25705_u16 as f64;
_12 = _6 - _6;
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_10 = 187_u8 ^ 90_u8;
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_4 = _11.fld0 as u32;
_4 = 3428334624_u32;
_4 = 296523058_u32 & 3179079176_u32;
_10 = 52_u8 >> _11.fld0;
_11.fld0 = (-25773_i16) << _8;
RET = [_8,_8,_8,_8,_8,_8,_8,_8];
_11 = Adt74 { fld0: 11354_i16 };
_13 = 322245444535312580959268446332117301787_u128 | 337254496727520358624384370771585319884_u128;
_6 = _12;
_11 = Adt74 { fld0: (-8107_i16) };
Goto(bb15)
}
bb15 = {
Call(_14 = dump_var(15_usize, 13_usize, Move(_13), 8_usize, Move(_8), 15_usize, _15, 15_usize, _15), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: (u32,),mut _2: i128,mut _3: f32,mut _4: i16,mut _5: i64,mut _6: i16,mut _7: bool,mut _8: i64) -> bool {
mir! {
type RET = bool;
let _9: [u64; 7];
let _10: Adt76;
let _11: Adt22;
let _12: f64;
let _13: [bool; 6];
let _14: *mut usize;
let _15: [usize; 3];
let _16: ();
let _17: ();
{
RET = _7 | _7;
_6 = _4;
RET = _7;
_7 = _6 >= _4;
_2 = -(-40489061940332468884196458688092127822_i128);
RET = !_7;
_5 = _8 & _8;
_7 = _8 <= _5;
_12 = 1615377357_i32 as f64;
_5 = _8 << _8;
_1 = (2240603701_u32,);
_9 = [9953601368702895746_u64,2377904565551842715_u64,13553018565829235505_u64,247566509860373543_u64,12193278681637497302_u64,13060054152474204043_u64,8434374727835887005_u64];
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(16_usize, 9_usize, Move(_9), 1_usize, Move(_1), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{5074f}'), std::hint::black_box(5573866039081780502_u64), std::hint::black_box((-64_i8)), std::hint::black_box((-6832_i16)), std::hint::black_box(1278620538_u32), std::hint::black_box((-3796547357230315461_i64)), std::hint::black_box((-108005489670522227524560391641791503611_i128)), std::hint::black_box(3_usize), std::hint::black_box(179_u8), std::hint::black_box(2180_u16));
                
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: bool,
fld1: [u8; 7],
fld2: isize,
fld3: i128,
fld4: i16,
fld5: usize,

},
Variant1{
fld0: usize,
fld1: [u8; 7],

}}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){unsafe{printf("Adt29::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
#[derive(Copy,Clone)]pub enum Adt29 {
Variant0{
fld0: ((u16,),),
fld1: char,
fld2: *mut u32,

},
Variant1{
fld0: u16,
fld1: *mut u32,
fld2: ((u16,),),
fld3: (u16,),
fld4: *const i128,

},
Variant2{
fld0: i16,
fld1: char,
fld2: ((u16,),),

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){unsafe{printf("Adt68::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt68 {
Variant0{
fld0: bool,
fld1: (f64, (u16,)),
fld2: *mut isize,
fld3: *const i128,
fld4: ((u16,),),

},
Variant1{
fld0: ((u32,),),
fld1: char,
fld2: Adt29,
fld3: u8,
fld4: i16,
fld5: [usize; 3],
fld6: *mut i128,
fld7: u128,

}}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){unsafe{printf("Adt69::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt69 {
Variant0{
fld0: (u32, (i32,), char),
fld1: char,
fld2: Adt68,
fld3: [u8; 7],
fld4: ((u16,),),
fld5: Adt29,
fld6: i64,

},
Variant1{
fld0: *const ((u32, (i32,), char),),
fld1: char,
fld2: Adt29,
fld3: f64,
fld4: (f64, (u16,)),
fld5: [u64; 6],
fld6: [i64; 6],

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt74{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt74 {
fld0: i16,
}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){unsafe{printf("Adt76::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt76 {
Variant0{
fld0: Adt68,
fld1: Adt22,
fld2: [u32; 2],

},
Variant1{
fld0: Adt29,
fld1: f32,
fld2: [i64; 6],
fld3: [bool; 6],
fld4: i64,
fld5: u16,

}}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){unsafe{printf("Adt79::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: (i32,),
fld1: Adt22,
fld2: usize,
fld3: i64,

},
Variant1{
fld0: Adt76,

},
Variant2{
fld0: bool,
fld1: *const ((u32, (i32,), char),),
fld2: u128,
fld3: ((u32, (i32,), char),),
fld4: *mut u32,
fld5: *mut i128,
fld6: [i64; 8],
fld7: i128,

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt81{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt81 {
fld0: u64,
fld1: u8,
fld2: [u8; 5],
fld3: [i64; 8],
fld4: i16,
fld5: [bool; 7],
}

