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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: u128,mut _10: u8,mut _11: u16,mut _12: u32) -> char {
mir! {
type RET = char;
let _13: *mut [u8; 6];
let _14: f64;
let _15: char;
let _16: char;
let _17: *mut bool;
let _18: *mut [i64; 7];
let _19: [isize; 8];
let _20: (*mut (((char,),), u8), (isize,));
let _21: isize;
let _22: char;
let _23: u32;
let _24: f32;
let _25: char;
let _26: isize;
let _27: bool;
let _28: *mut *mut &'static ((char,),);
let _29: &'static [i16; 8];
let _30: ();
let _31: ();
{
_3 = (-9223372036854775808_isize);
_6 = _3 as i32;
RET = '\u{ed897}';
_2 = RET;
RET = _2;
_6 = 2079187356_i32 - 1101744047_i32;
_10 = true as u8;
_11 = 42544_u16 >> _10;
_9 = 264985781865809611075400220673077079815_u128;
_1 = true;
_1 = _11 <= _11;
_3 = _11 as isize;
Call(_11 = fn1(_3, _6, _6, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = -(-9088_i16);
_6 = 624533129_i32;
_12 = 1964831173_u32;
_8 = !(-89142531889158445873648396216383586534_i128);
_4 = -(-57_i8);
_8 = 42977408371424808847866107192229294518_i128 + 118888652671599098825109918906214739440_i128;
RET = _2;
RET = _2;
_9 = 63040609399335787540157754998350640916_u128 ^ 222321294027998600861526944472381286527_u128;
_10 = _9 as u8;
_2 = RET;
_11 = _4 as u16;
RET = _2;
RET = _2;
_9 = !191018105800633165889554706891142048046_u128;
_11 = 46959_u16 ^ 28037_u16;
_14 = _12 as f64;
_3 = 9223372036854775807_isize;
_8 = (-138314327435371165377785750289333089984_i128) >> _10;
_2 = RET;
_2 = RET;
_15 = _2;
_6 = (-1065950310_i32);
Goto(bb2)
}
bb2 = {
_8 = 115791461321962589346421449015390770275_i128 & (-94512061747802638491418837448307433921_i128);
_3 = 9223372036854775807_isize << _8;
_9 = _11 as u128;
_6 = !832419126_i32;
_7 = _6 as i64;
Goto(bb3)
}
bb3 = {
_12 = !3518391703_u32;
_16 = _2;
_3 = 9223372036854775807_isize;
_10 = !119_u8;
_10 = 7_usize as u8;
_16 = _2;
RET = _16;
_17 = core::ptr::addr_of_mut!(_1);
_19 = [_3,_3,_3,_3,_3,_3,_3,_3];
_8 = 29761264981957113992679036596538247214_i128;
_19 = [_3,_3,_3,_3,_3,_3,_3,_3];
_9 = 157015321585583795985425148445493634217_u128 - 228167852056000538363552409169342033810_u128;
_4 = _1 as i8;
_14 = _4 as f64;
_14 = _3 as f64;
_11 = 24582_u16;
_14 = 1_usize as f64;
RET = _15;
_15 = _2;
(*_17) = !false;
_16 = RET;
_5 = !32091_i16;
_15 = RET;
_8 = !(-64021297602199912844730082118315833208_i128);
match _3 {
9223372036854775807 => bb4,
_ => bb1
}
}
bb4 = {
_15 = _2;
Goto(bb5)
}
bb5 = {
_10 = !31_u8;
_21 = _12 as isize;
_9 = 68063924579878896789958851171278079737_u128;
_3 = _21 | _21;
_16 = _15;
_20.1.0 = _3;
_2 = _15;
_4 = !(-72_i8);
_14 = _10 as f64;
_16 = _15;
_22 = _15;
_10 = !54_u8;
(*_17) = !true;
_24 = _10 as f32;
_19 = [_21,_20.1.0,_3,_3,_21,_20.1.0,_20.1.0,_3];
match _9 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
68063924579878896789958851171278079737 => bb13,
_ => bb12
}
}
bb6 = {
_15 = _2;
Goto(bb5)
}
bb7 = {
_12 = !3518391703_u32;
_16 = _2;
_3 = 9223372036854775807_isize;
_10 = !119_u8;
_10 = 7_usize as u8;
_16 = _2;
RET = _16;
_17 = core::ptr::addr_of_mut!(_1);
_19 = [_3,_3,_3,_3,_3,_3,_3,_3];
_8 = 29761264981957113992679036596538247214_i128;
_19 = [_3,_3,_3,_3,_3,_3,_3,_3];
_9 = 157015321585583795985425148445493634217_u128 - 228167852056000538363552409169342033810_u128;
_4 = _1 as i8;
_14 = _4 as f64;
_14 = _3 as f64;
_11 = 24582_u16;
_14 = 1_usize as f64;
RET = _15;
_15 = _2;
(*_17) = !false;
_16 = RET;
_5 = !32091_i16;
_15 = RET;
_8 = !(-64021297602199912844730082118315833208_i128);
match _3 {
9223372036854775807 => bb4,
_ => bb1
}
}
bb8 = {
_8 = 115791461321962589346421449015390770275_i128 & (-94512061747802638491418837448307433921_i128);
_3 = 9223372036854775807_isize << _8;
_9 = _11 as u128;
_6 = !832419126_i32;
_7 = _6 as i64;
Goto(bb3)
}
bb9 = {
_5 = -(-9088_i16);
_6 = 624533129_i32;
_12 = 1964831173_u32;
_8 = !(-89142531889158445873648396216383586534_i128);
_4 = -(-57_i8);
_8 = 42977408371424808847866107192229294518_i128 + 118888652671599098825109918906214739440_i128;
RET = _2;
RET = _2;
_9 = 63040609399335787540157754998350640916_u128 ^ 222321294027998600861526944472381286527_u128;
_10 = _9 as u8;
_2 = RET;
_11 = _4 as u16;
RET = _2;
RET = _2;
_9 = !191018105800633165889554706891142048046_u128;
_11 = 46959_u16 ^ 28037_u16;
_14 = _12 as f64;
_3 = 9223372036854775807_isize;
_8 = (-138314327435371165377785750289333089984_i128) >> _10;
_2 = RET;
_2 = RET;
_15 = _2;
_6 = (-1065950310_i32);
Goto(bb2)
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
_21 = -_3;
_17 = core::ptr::addr_of_mut!(_1);
_6 = (-2055105768_i32) | 328519825_i32;
_16 = RET;
_25 = _22;
_23 = _14 as u32;
_3 = -_21;
_1 = !true;
_2 = _15;
_24 = _4 as f32;
_21 = _10 as isize;
_26 = _20.1.0;
_4 = (*_17) as i8;
(*_17) = _20.1.0 > _3;
_16 = RET;
_1 = !false;
_5 = !(-7381_i16);
_10 = _1 as u8;
(*_17) = true;
_12 = !_23;
_26 = !_3;
(*_17) = false;
match _11 {
0 => bb12,
1 => bb9,
2 => bb6,
3 => bb14,
4 => bb15,
24582 => bb17,
_ => bb16
}
}
bb14 = {
_5 = -(-9088_i16);
_6 = 624533129_i32;
_12 = 1964831173_u32;
_8 = !(-89142531889158445873648396216383586534_i128);
_4 = -(-57_i8);
_8 = 42977408371424808847866107192229294518_i128 + 118888652671599098825109918906214739440_i128;
RET = _2;
RET = _2;
_9 = 63040609399335787540157754998350640916_u128 ^ 222321294027998600861526944472381286527_u128;
_10 = _9 as u8;
_2 = RET;
_11 = _4 as u16;
RET = _2;
RET = _2;
_9 = !191018105800633165889554706891142048046_u128;
_11 = 46959_u16 ^ 28037_u16;
_14 = _12 as f64;
_3 = 9223372036854775807_isize;
_8 = (-138314327435371165377785750289333089984_i128) >> _10;
_2 = RET;
_2 = RET;
_15 = _2;
_6 = (-1065950310_i32);
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_25 = _15;
_25 = _16;
_24 = 7470148143647664675_u64 as f32;
_2 = _15;
_5 = _12 as i16;
_25 = RET;
_21 = 6152521497797776100_u64 as isize;
_2 = _25;
_2 = RET;
_12 = !_23;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(0_usize, 16_usize, Move(_16), 4_usize, Move(_4), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(0_usize, 11_usize, Move(_11), 22_usize, Move(_22), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(0_usize, 19_usize, Move(_19), 15_usize, Move(_15), 31_usize, _31, 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: i32,mut _3: i32,mut _4: bool) -> u16 {
mir! {
type RET = u16;
let _5: isize;
let _6: Adt48;
let _7: u16;
let _8: i64;
let _9: isize;
let _10: f32;
let _11: bool;
let _12: char;
let _13: [bool; 3];
let _14: ([u64; 1], *mut &'static ((char,),));
let _15: [bool; 5];
let _16: isize;
let _17: (Adt45,);
let _18: i16;
let _19: isize;
let _20: [bool; 3];
let _21: u64;
let _22: &'static Adt44;
let _23: i64;
let _24: f32;
let _25: u128;
let _26: u128;
let _27: f32;
let _28: u64;
let _29: isize;
let _30: [bool; 5];
let _31: [u64; 2];
let _32: u32;
let _33: f32;
let _34: [bool; 5];
let _35: ();
let _36: ();
{
RET = 53521_u16;
_3 = _2 << _1;
_1 = 9223372036854775807_isize;
_3 = '\u{84009}' as i32;
RET = 166_u8 as u16;
_3 = _2;
_3 = _2;
_4 = true;
RET = 37285_u16 - 8541_u16;
_5 = _1;
RET = 20019_u16 & 40609_u16;
_1 = (-49_i8) as isize;
_4 = false;
_5 = _1;
_1 = _5;
RET = !16936_u16;
Goto(bb1)
}
bb1 = {
RET = _5 as u16;
_5 = _1;
_7 = _5 as u16;
_7 = !RET;
_2 = _3 >> _7;
RET = _7 - _7;
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _7 | _7;
_5 = 23_i8 as isize;
_1 = _5;
_1 = 195562571727930004095092561691971096863_u128 as isize;
_8 = 1035230644_u32 as i64;
_5 = -_1;
_7 = RET >> RET;
_7 = RET * RET;
Goto(bb3)
}
bb3 = {
_7 = !RET;
_1 = _5 * _5;
_2 = -_3;
_4 = true;
_1 = !_5;
_2 = '\u{a22c6}' as i32;
_1 = _5;
RET = _7 & _7;
_5 = (-126232192558570005830781946632410772885_i128) as isize;
Call(_1 = fn2(_4, _5, RET, _5, _5, _3, _8, RET, _3, _7, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _1 as u16;
_10 = (-19_i8) as f32;
_2 = '\u{ae491}' as i32;
_11 = !_4;
RET = 88_i8 as u16;
RET = _7;
_3 = !_2;
_3 = _2;
RET = !_7;
_11 = !_4;
_11 = _4 & _4;
RET = _7;
RET = !_7;
RET = _7;
_1 = _10 as isize;
_8 = !(-6090915085365919232_i64);
_4 = _7 > RET;
_3 = _2;
_5 = -_1;
_12 = '\u{33dff}';
_5 = (-21680_i16) as isize;
_3 = _2 + _2;
_9 = _5 | _5;
_13 = [_4,_11,_4];
_7 = RET - RET;
_4 = _3 >= _2;
_7 = _1 as u16;
_1 = _5 | _5;
Goto(bb5)
}
bb5 = {
_8 = !(-2856153856952155284_i64);
_10 = 9_u8 as f32;
_7 = 13516250627407196524_u64 as u16;
_5 = _2 as isize;
_9 = -_5;
_1 = !_9;
_8 = -(-19429680275966667_i64);
_3 = -_2;
_13 = [_11,_4,_4];
_13 = [_4,_11,_11];
_8 = 5429459911313821793_i64 * 5565241129507298366_i64;
_10 = _3 as f32;
_14.0 = [1509911032337018444_u64];
Goto(bb6)
}
bb6 = {
_4 = _12 < _12;
RET = _7;
_16 = _2 as isize;
_15 = [_4,_11,_11,_11,_4];
_2 = _3 << _7;
_1 = _12 as isize;
_10 = 18223809964950811671_u64 as f32;
_19 = !_9;
_14.0 = [12690809803805891362_u64];
_8 = _3 as i64;
Goto(bb7)
}
bb7 = {
_6 = Adt48::Variant1 { fld0: (-133637087665359078689933179088789031778_i128),fld1: _13,fld2: _15 };
_15 = Field::<[bool; 5]>(Variant(_6, 1), 2);
_16 = -_1;
_20 = [_11,_4,_4];
_5 = _2 as isize;
_12 = '\u{a132}';
_20 = _13;
_9 = _11 as isize;
_8 = 7864337553129672220_i64 & (-4385037001804473525_i64);
_12 = '\u{e81d}';
place!(Field::<i128>(Variant(_6, 1), 0)) = 4_usize as i128;
place!(Field::<[bool; 5]>(Variant(_6, 1), 2)) = [_4,_11,_4,_11,_11];
SetDiscriminant(_6, 1);
_12 = '\u{8ccf2}';
_10 = RET as f32;
Goto(bb8)
}
bb8 = {
_24 = _10;
Goto(bb9)
}
bb9 = {
place!(Field::<[bool; 3]>(Variant(_6, 1), 1)) = _13;
_26 = !8496745148418333428180780010148592725_u128;
_26 = 107_i8 as u128;
_18 = (-449_i16);
_3 = -_2;
_19 = _16 - _5;
place!(Field::<[bool; 5]>(Variant(_6, 1), 2)) = [_11,_11,_4,_11,_11];
_21 = !10786188837616155127_u64;
_10 = _24 * _24;
_3 = _2 | _2;
_10 = -_24;
_2 = _3 - _3;
_21 = 25_i8 as u64;
_28 = _21 * _21;
_4 = !_11;
_31 = [_21,_21];
_30 = Field::<[bool; 5]>(Variant(_6, 1), 2);
_4 = !_11;
place!(Field::<[bool; 3]>(Variant(_6, 1), 1)) = _13;
_27 = 215_u8 as f32;
place!(Field::<i128>(Variant(_6, 1), 0)) = (-133466329963764895855754624596691005677_i128);
_25 = _26;
RET = _7;
_16 = _4 as isize;
_20 = Field::<[bool; 3]>(Variant(_6, 1), 1);
Call(_32 = core::intrinsics::transmute(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_13 = [_4,_11,_4];
_1 = -_9;
_29 = _9;
SetDiscriminant(_6, 1);
place!(Field::<i128>(Variant(_6, 1), 0)) = !27746542876838565312090453496851877311_i128;
_14.0 = [_28];
_19 = _8 as isize;
_11 = _16 <= _9;
_29 = -_9;
match _18 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463463374607431768211007 => bb17,
_ => bb16
}
}
bb11 = {
place!(Field::<[bool; 3]>(Variant(_6, 1), 1)) = _13;
_26 = !8496745148418333428180780010148592725_u128;
_26 = 107_i8 as u128;
_18 = (-449_i16);
_3 = -_2;
_19 = _16 - _5;
place!(Field::<[bool; 5]>(Variant(_6, 1), 2)) = [_11,_11,_4,_11,_11];
_21 = !10786188837616155127_u64;
_10 = _24 * _24;
_3 = _2 | _2;
_10 = -_24;
_2 = _3 - _3;
_21 = 25_i8 as u64;
_28 = _21 * _21;
_4 = !_11;
_31 = [_21,_21];
_30 = Field::<[bool; 5]>(Variant(_6, 1), 2);
_4 = !_11;
place!(Field::<[bool; 3]>(Variant(_6, 1), 1)) = _13;
_27 = 215_u8 as f32;
place!(Field::<i128>(Variant(_6, 1), 0)) = (-133466329963764895855754624596691005677_i128);
_25 = _26;
RET = _7;
_16 = _4 as isize;
_20 = Field::<[bool; 3]>(Variant(_6, 1), 1);
Call(_32 = core::intrinsics::transmute(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_24 = _10;
Goto(bb9)
}
bb13 = {
_6 = Adt48::Variant1 { fld0: (-133637087665359078689933179088789031778_i128),fld1: _13,fld2: _15 };
_15 = Field::<[bool; 5]>(Variant(_6, 1), 2);
_16 = -_1;
_20 = [_11,_4,_4];
_5 = _2 as isize;
_12 = '\u{a132}';
_20 = _13;
_9 = _11 as isize;
_8 = 7864337553129672220_i64 & (-4385037001804473525_i64);
_12 = '\u{e81d}';
place!(Field::<i128>(Variant(_6, 1), 0)) = 4_usize as i128;
place!(Field::<[bool; 5]>(Variant(_6, 1), 2)) = [_4,_11,_4,_11,_11];
SetDiscriminant(_6, 1);
_12 = '\u{8ccf2}';
_10 = RET as f32;
Goto(bb8)
}
bb14 = {
_7 = !RET;
_1 = _5 * _5;
_2 = -_3;
_4 = true;
_1 = !_5;
_2 = '\u{a22c6}' as i32;
_1 = _5;
RET = _7 & _7;
_5 = (-126232192558570005830781946632410772885_i128) as isize;
Call(_1 = fn2(_4, _5, RET, _5, _5, _3, _8, RET, _3, _7, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
RET = _7 | _7;
_5 = 23_i8 as isize;
_1 = _5;
_1 = 195562571727930004095092561691971096863_u128 as isize;
_8 = 1035230644_u32 as i64;
_5 = -_1;
_7 = RET >> RET;
_7 = RET * RET;
Goto(bb3)
}
bb16 = {
RET = _1 as u16;
_10 = (-19_i8) as f32;
_2 = '\u{ae491}' as i32;
_11 = !_4;
RET = 88_i8 as u16;
RET = _7;
_3 = !_2;
_3 = _2;
RET = !_7;
_11 = !_4;
_11 = _4 & _4;
RET = _7;
RET = !_7;
RET = _7;
_1 = _10 as isize;
_8 = !(-6090915085365919232_i64);
_4 = _7 > RET;
_3 = _2;
_5 = -_1;
_12 = '\u{33dff}';
_5 = (-21680_i16) as isize;
_3 = _2 + _2;
_9 = _5 | _5;
_13 = [_4,_11,_4];
_7 = RET - RET;
_4 = _3 >= _2;
_7 = _1 as u16;
_1 = _5 | _5;
Goto(bb5)
}
bb17 = {
_1 = !_29;
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(1_usize, 19_usize, Move(_19), 30_usize, Move(_30), 18_usize, Move(_18), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(1_usize, 31_usize, Move(_31), 20_usize, Move(_20), 7_usize, Move(_7), 28_usize, Move(_28)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(1_usize, 5_usize, Move(_5), 12_usize, Move(_12), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: isize,mut _3: u16,mut _4: isize,mut _5: isize,mut _6: i32,mut _7: i64,mut _8: u16,mut _9: i32,mut _10: u16,mut _11: i32) -> isize {
mir! {
type RET = isize;
let _12: Adt44;
let _13: isize;
let _14: usize;
let _15: [i16; 8];
let _16: isize;
let _17: f64;
let _18: [i16; 8];
let _19: *mut Adt57;
let _20: char;
let _21: char;
let _22: isize;
let _23: usize;
let _24: (((char,),), u8);
let _25: &'static u128;
let _26: isize;
let _27: i32;
let _28: u8;
let _29: *const *mut [i64; 7];
let _30: *mut &'static &'static *mut u128;
let _31: i8;
let _32: u16;
let _33: usize;
let _34: *mut &'static &'static *mut u128;
let _35: ([bool; 5],);
let _36: ();
let _37: ();
{
_10 = _8;
_3 = !_8;
_7 = -(-1356673653055349950_i64);
_7 = 6994993365418532261_i64;
_1 = true;
RET = _5;
RET = !_2;
_1 = true;
RET = _11 as isize;
_5 = RET;
match _7 {
0 => bb1,
6994993365418532261 => bb3,
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
_4 = _2 >> _3;
_3 = (-29584980052012172079269609111334273541_i128) as u16;
_11 = _9 ^ _9;
_11 = (-168695850392542940185117031939049304481_i128) as i32;
RET = !_4;
_5 = _4 + RET;
_10 = _8 << _5;
_11 = _6;
_1 = !true;
_3 = _8;
_2 = 7_usize as isize;
_1 = !true;
_2 = _4;
_7 = -7588339392370880313_i64;
_4 = !_2;
Goto(bb4)
}
bb4 = {
_1 = _10 < _3;
_1 = false;
_10 = _8 >> RET;
_11 = _1 as i32;
_10 = !_8;
RET = _4 - _4;
_1 = !true;
_11 = -_6;
Call(_9 = fn3(_4, _2, _4, RET, _5, _4, _5, RET), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = !_10;
RET = -_4;
_6 = _9 | _11;
_7 = 1363714636920958131_i64;
_15 = [14875_i16,(-8847_i16),22732_i16,1498_i16,(-6519_i16),(-22677_i16),25611_i16,(-25452_i16)];
_14 = _3 as usize;
_14 = 4_usize & 14445366012075576263_usize;
_10 = _1 as u16;
_15 = [21867_i16,(-14913_i16),(-118_i16),28713_i16,(-11891_i16),(-3467_i16),(-29605_i16),24723_i16];
_9 = -_6;
Goto(bb6)
}
bb6 = {
_10 = _3;
_18 = [(-31891_i16),9108_i16,(-25632_i16),(-22422_i16),19502_i16,18639_i16,(-17409_i16),(-9810_i16)];
_18 = [(-2865_i16),(-23507_i16),(-19583_i16),27452_i16,(-14326_i16),(-31628_i16),2547_i16,7365_i16];
RET = 14_i8 as isize;
_14 = 2_usize & 3_usize;
_8 = !_10;
_9 = 4177933571_u32 as i32;
_10 = !_3;
_13 = 112_u8 as isize;
_2 = '\u{68cc6}' as isize;
_5 = _4;
_13 = RET + RET;
_17 = _8 as f64;
_2 = !_5;
_16 = _5 + RET;
Goto(bb7)
}
bb7 = {
_2 = !_13;
_2 = (-129835583867913607832488797526678251406_i128) as isize;
_10 = 69_i8 as u16;
_14 = !2795416234202954604_usize;
_18 = [(-32484_i16),6038_i16,(-14818_i16),(-15085_i16),3581_i16,(-1294_i16),(-27445_i16),17980_i16];
_16 = (-74981907503881366877015549780674041426_i128) as isize;
_9 = _5 as i32;
_8 = _3;
_5 = _4;
_5 = (-303_i16) as isize;
_16 = _4;
_4 = -_16;
_15 = [(-12420_i16),16386_i16,(-11185_i16),5793_i16,20637_i16,(-25_i16),(-27179_i16),21594_i16];
_15 = [(-20833_i16),30130_i16,28270_i16,(-17826_i16),32460_i16,(-19168_i16),18702_i16,(-25535_i16)];
_8 = _3;
_18 = _15;
RET = (-8094_i16) as isize;
_16 = _13;
_14 = !1_usize;
_4 = -_2;
_8 = _3;
RET = _16;
_22 = _13;
match _7 {
0 => bb5,
1 => bb2,
2 => bb3,
1363714636920958131 => bb8,
_ => bb6
}
}
bb8 = {
_13 = 456453987_u32 as isize;
_20 = '\u{aee47}';
RET = _16 - _2;
_6 = _9;
Goto(bb9)
}
bb9 = {
_2 = -RET;
_8 = _3;
Goto(bb10)
}
bb10 = {
_6 = 201281513817298959572262808990480263410_u128 as i32;
_17 = 11502456310633338336_u64 as f64;
_6 = _5 as i32;
_24.1 = 45_u8 + 74_u8;
_16 = _8 as isize;
_14 = !6_usize;
_11 = _9 ^ _6;
_22 = _16;
_7 = 3082571912686727226_i64;
_14 = 38642148815865156013187450287626170301_i128 as usize;
_7 = (-8817230672721531480_i64);
_11 = _9 & _9;
Goto(bb11)
}
bb11 = {
_21 = _20;
_7 = -(-2115796652547729144_i64);
_3 = (-65_i8) as u16;
_26 = !_16;
_24.0.0.0 = _21;
_22 = -_26;
_23 = _14;
_20 = _24.0.0.0;
_7 = (-9051354678914967704_i64);
_11 = _6 & _9;
_22 = _24.1 as isize;
_16 = -_22;
_5 = !_2;
_2 = _13;
_9 = !_11;
Goto(bb12)
}
bb12 = {
_15 = _18;
_24.0.0 = (_21,);
_24.1 = 191_u8 | 64_u8;
RET = _13;
_21 = _24.0.0.0;
_20 = _24.0.0.0;
_32 = _8 - _8;
_35.0 = [_1,_1,_1,_1,_1];
Call(_12 = fn5(_11, _22, _11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23 = !_14;
RET = _26 ^ _26;
_10 = (-26407_i16) as u16;
_17 = Field::<f64>(Variant(_12, 0), 4);
_18 = [(-2621_i16),20784_i16,(-27965_i16),(-13654_i16),(-10191_i16),(-7412_i16),(-28227_i16),(-28023_i16)];
_28 = Field::<u8>(Variant(_12, 0), 0) * Field::<u8>(Variant(_12, 0), 0);
SetDiscriminant(_12, 1);
place!(Field::<char>(Variant(_12, 1), 1)) = _24.0.0.0;
_10 = !_32;
_9 = _11 ^ _11;
_32 = _8;
place!(Field::<char>(Variant(_12, 1), 1)) = _20;
Goto(bb14)
}
bb14 = {
_14 = 1258938200_u32 as usize;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(2_usize, 18_usize, Move(_18), 21_usize, Move(_21), 8_usize, Move(_8), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(2_usize, 1_usize, Move(_1), 13_usize, Move(_13), 35_usize, Move(_35), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(2_usize, 23_usize, Move(_23), 16_usize, Move(_16), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> i32 {
mir! {
type RET = i32;
let _9: i32;
let _10: f64;
let _11: *const i32;
let _12: i128;
let _13: [bool; 3];
let _14: [bool; 3];
let _15: *mut &'static &'static *mut u128;
let _16: f64;
let _17: f32;
let _18: [char; 5];
let _19: f32;
let _20: u8;
let _21: isize;
let _22: char;
let _23: isize;
let _24: isize;
let _25: f32;
let _26: ();
let _27: ();
{
_2 = _7;
_3 = 158071243416815611677714214539301429001_u128 as isize;
_3 = -_2;
_5 = 219_u8 as isize;
RET = 43381395654945948240638177533883079843_i128 as i32;
RET = 1903879811_i32;
_3 = -_4;
_9 = RET;
_8 = _4;
RET = 8441819917464552549_u64 as i32;
_10 = 13938_u16 as f64;
RET = _9;
_8 = 12234945572192502303_usize as isize;
_11 = core::ptr::addr_of!(_9);
_2 = _7;
_13 = [true,true,true];
_5 = _3 - _4;
Call(_14 = fn4(_4, _3, _5, _5, _5, _7, _13, _1, _2, _3, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _4;
(*_11) = RET;
_14 = [true,false,false];
_7 = -_5;
_12 = 39110686247293956256824981200363077475_i128 ^ (-12573167745287374699393465540369226648_i128);
_12 = -(-146622247529448622157531304678464243_i128);
_7 = '\u{609f5}' as isize;
_1 = _5;
_16 = RET as f64;
RET = (*_11);
_10 = -_16;
_7 = !_2;
_14 = _13;
(*_11) = RET >> _7;
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17 = 318134287263004838639417286180348776877_u128 as f32;
_2 = 274994271769270801701115323662118868497_u128 as isize;
RET = !(*_11);
_11 = core::ptr::addr_of!((*_11));
_12 = 132408856777344265414167581682973896512_i128;
_7 = _3;
_7 = _1;
_22 = '\u{8d69a}';
_3 = _5 - _4;
_19 = _17;
_6 = 49_i8 as isize;
_14 = _13;
_23 = -_4;
_10 = -_16;
_5 = 1299530177_u32 as isize;
_24 = _7 ^ _23;
_25 = -_19;
_25 = -_17;
RET = -_9;
_12 = !(-13989396816404912069548206358675751911_i128);
_16 = _7 as f64;
Goto(bb3)
}
bb3 = {
Call(_26 = dump_var(3_usize, 3_usize, Move(_3), 12_usize, Move(_12), 13_usize, Move(_13), 22_usize, Move(_22)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_26 = dump_var(3_usize, 1_usize, Move(_1), 6_usize, Move(_6), 14_usize, Move(_14), 27_usize, _27), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: [bool; 3],mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> [bool; 3] {
mir! {
type RET = [bool; 3];
let _13: f32;
let _14: isize;
let _15: *mut &'static &'static *mut u128;
let _16: char;
let _17: usize;
let _18: bool;
let _19: f64;
let _20: [u64; 2];
let _21: ([u64; 1], *mut &'static ((char,),));
let _22: *mut u128;
let _23: isize;
let _24: [i8; 5];
let _25: &'static f64;
let _26: (&'static usize, *const (((char,),), u8), [i16; 8]);
let _27: [bool; 5];
let _28: [i64; 7];
let _29: isize;
let _30: Adt48;
let _31: f64;
let _32: i64;
let _33: i8;
let _34: [u8; 6];
let _35: [u64; 3];
let _36: f32;
let _37: isize;
let _38: (Adt45,);
let _39: f64;
let _40: &'static ((((char,),), u8), *const (((char,),), u8), (i16,));
let _41: (isize,);
let _42: [bool; 3];
let _43: (isize,);
let _44: &'static Adt44;
let _45: ();
let _46: ();
{
_7 = [false,false,true];
_10 = _3 * _6;
RET = _7;
_13 = 680859002_u32 as f32;
_3 = 7453979842563096206_i64 as isize;
_5 = (-15079_i16) as isize;
_14 = _13 as isize;
_8 = 87675235430430793943756589664019455094_u128 as isize;
_5 = -_10;
_14 = _2 << _1;
_11 = (-38823480328477085316906824800554193579_i128) as isize;
_10 = -_2;
_12 = (-29144_i16) as isize;
_10 = _5 << _4;
_12 = false as isize;
_14 = _5 + _10;
_6 = !_5;
Goto(bb1)
}
bb1 = {
_9 = _13 as isize;
_2 = _10;
_11 = _5;
_1 = _14;
RET = [false,true,false];
RET = [false,true,true];
_11 = -_6;
_17 = 4675889876264050559_usize | 5_usize;
RET = [false,true,true];
_4 = _14 - _14;
_17 = 6522761402542491099_i64 as usize;
_11 = 30463713820983589293991418612537435160_u128 as isize;
_17 = !2_usize;
RET = [false,false,true];
_18 = !false;
_13 = 1633165833_u32 as f32;
Call(_3 = core::intrinsics::transmute(_10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_19 = (-3_i8) as f64;
_12 = _4 & _14;
_19 = _17 as f64;
_17 = 16485272649806545275_usize;
_4 = _10;
_17 = _13 as usize;
_16 = '\u{ac85e}';
_9 = -_6;
Goto(bb3)
}
bb3 = {
RET = [_18,_18,_18];
_20 = [10089154174933950769_u64,15098324769544351121_u64];
_23 = _1 >> _2;
_12 = -_1;
_6 = _10 & _14;
_14 = _23;
_20 = [11886083711405722162_u64,15086272045498291309_u64];
_17 = !14924644874521429776_usize;
_13 = 248554973602253818388432214410414417926_u128 as f32;
_8 = _10;
_24 = [(-84_i8),(-12_i8),118_i8,23_i8,61_i8];
_2 = _12 << _1;
_14 = _3 & _4;
_21.0 = [14040112153707800983_u64];
Goto(bb4)
}
bb4 = {
_10 = !_14;
_19 = 4677782184671635128_u64 as f64;
_19 = (-101_i8) as f64;
_20 = [2239747688179439968_u64,14102295405085497710_u64];
_16 = '\u{65eae}';
_11 = _9 + _23;
_1 = _6 - _8;
_20 = [6313671701302922746_u64,13248230506146036581_u64];
_2 = _23;
_24 = [(-21_i8),100_i8,108_i8,(-34_i8),(-19_i8)];
_29 = 44371_u16 as isize;
_3 = _14 | _11;
_27 = [_18,_18,_18,_18,_18];
Goto(bb5)
}
bb5 = {
_26.0 = &_17;
_26.2 = [24011_i16,(-19970_i16),26240_i16,(-9741_i16),(-17103_i16),19694_i16,5114_i16,(-23587_i16)];
_6 = _3 << _3;
RET = [_18,_18,_18];
_32 = _17 as i64;
_28 = [_32,_32,_32,_32,_32,_32,_32];
_1 = !_12;
_25 = &_19;
_16 = '\u{910a6}';
_23 = _29 - _10;
_17 = 1506976396326819891_usize;
_13 = (-11527367318754819095253526481777229904_i128) as f32;
_9 = !_10;
_25 = &_19;
_33 = (-48_i8);
_26.2 = [(-31655_i16),(-9137_i16),14911_i16,(-2309_i16),29947_i16,14362_i16,10252_i16,4477_i16];
_34 = [159_u8,119_u8,155_u8,183_u8,65_u8,128_u8];
_19 = _13 as f64;
_28 = [_32,_32,_32,_32,_32,_32,_32];
_18 = _6 == _11;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
1506976396326819891 => bb9,
_ => bb8
}
}
bb6 = {
_10 = !_14;
_19 = 4677782184671635128_u64 as f64;
_19 = (-101_i8) as f64;
_20 = [2239747688179439968_u64,14102295405085497710_u64];
_16 = '\u{65eae}';
_11 = _9 + _23;
_1 = _6 - _8;
_20 = [6313671701302922746_u64,13248230506146036581_u64];
_2 = _23;
_24 = [(-21_i8),100_i8,108_i8,(-34_i8),(-19_i8)];
_29 = 44371_u16 as isize;
_3 = _14 | _11;
_27 = [_18,_18,_18,_18,_18];
Goto(bb5)
}
bb7 = {
_9 = _13 as isize;
_2 = _10;
_11 = _5;
_1 = _14;
RET = [false,true,false];
RET = [false,true,true];
_11 = -_6;
_17 = 4675889876264050559_usize | 5_usize;
RET = [false,true,true];
_4 = _14 - _14;
_17 = 6522761402542491099_i64 as usize;
_11 = 30463713820983589293991418612537435160_u128 as isize;
_17 = !2_usize;
RET = [false,false,true];
_18 = !false;
_13 = 1633165833_u32 as f32;
Call(_3 = core::intrinsics::transmute(_10), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_19 = (-3_i8) as f64;
_12 = _4 & _14;
_19 = _17 as f64;
_17 = 16485272649806545275_usize;
_4 = _10;
_17 = _13 as usize;
_16 = '\u{ac85e}';
_9 = -_6;
Goto(bb3)
}
bb9 = {
_27 = [_18,_18,_18,_18,_18];
_36 = (-13277_i16) as f32;
_8 = _18 as isize;
_21.0 = [12215958003207465873_u64];
_41.0 = _14;
_42 = [_18,_18,_18];
_42 = [_18,_18,_18];
_2 = -_4;
_12 = _33 as isize;
_26.0 = &_17;
_17 = 8121861410987337735_usize;
_23 = 2305246447320193321_u64 as isize;
_39 = _19 - _19;
_26.0 = &_17;
_7 = [_18,_18,_18];
RET = [_18,_18,_18];
_38.0 = Adt45::Variant1 { fld0: _18,fld1: _41,fld2: _17,fld3: 53_u8,fld4: _24 };
_31 = _39 + _19;
Goto(bb10)
}
bb10 = {
Call(_45 = dump_var(4_usize, 3_usize, Move(_3), 33_usize, Move(_33), 41_usize, Move(_41), 1_usize, Move(_1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_45 = dump_var(4_usize, 6_usize, Move(_6), 32_usize, Move(_32), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_45 = dump_var(4_usize, 18_usize, Move(_18), 29_usize, Move(_29), 12_usize, Move(_12), 23_usize, Move(_23)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_45 = dump_var(4_usize, 16_usize, Move(_16), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: i32,mut _2: isize,mut _3: i32) -> Adt44 {
mir! {
type RET = Adt44;
let _4: (((char,),), u8);
let _5: *mut u128;
let _6: &'static usize;
let _7: *const &'static &'static *mut u128;
let _8: f32;
let _9: i16;
let _10: (&'static usize, *const (((char,),), u8), [i16; 8]);
let _11: char;
let _12: *mut u128;
let _13: i16;
let _14: bool;
let _15: &'static *mut u128;
let _16: &'static *mut u128;
let _17: &'static [i8; 5];
let _18: [u64; 2];
let _19: u128;
let _20: Adt24;
let _21: ();
let _22: ();
{
_2 = 12242_u16 as isize;
_1 = 18279135868979837528_u64 as i32;
_1 = !_3;
_4.0.0.0 = '\u{922e9}';
_4.0.0 = ('\u{c9e5}',);
_2 = (-81_isize) | (-9223372036854775808_isize);
_2 = -124_isize;
_4.0.0.0 = '\u{823d5}';
_2 = (-6823124750371896224669025989715824626_i128) as isize;
_2 = 9223372036854775807_isize;
_4.0.0.0 = '\u{c4b5}';
_4.1 = 96839328060064283129348140284424117468_u128 as u8;
_1 = 16706561702335560799_u64 as i32;
_3 = -_1;
_4.0.0.0 = '\u{ec5a4}';
_1 = !_3;
_2 = 4150139840_u32 as isize;
_2 = (-2656_i16) as isize;
_4.0.0.0 = '\u{d9ceb}';
_1 = _3;
_4.0.0 = ('\u{c8262}',);
_1 = !_3;
_4.0.0 = ('\u{2a3e9}',);
_4.1 = !77_u8;
_3 = _1;
_1 = false as i32;
Goto(bb1)
}
bb1 = {
_1 = 10389284473129276526_usize as i32;
_8 = _3 as f32;
_1 = _3 >> _3;
_3 = _1;
_4.0.0.0 = '\u{ed1cc}';
_4.0.0.0 = '\u{711b}';
_8 = _2 as f32;
_2 = -(-9223372036854775808_isize);
_4.0.0 = ('\u{63ec9}',);
_10.2 = [30366_i16,(-3300_i16),32001_i16,(-7963_i16),16431_i16,(-25301_i16),(-32273_i16),(-13491_i16)];
_8 = _4.1 as f32;
_8 = 2395843913_u32 as f32;
_1 = _3 & _3;
_4.1 = (-3113_i16) as u8;
_4.1 = 139_u8;
_4.1 = !192_u8;
_3 = _1 ^ _1;
_10.2 = [(-9275_i16),(-24856_i16),(-22415_i16),(-8645_i16),21736_i16,(-16689_i16),22159_i16,2786_i16];
_9 = (-128_i8) as i16;
_1 = 90_i8 as i32;
_10.1 = core::ptr::addr_of!(_4);
_9 = 22338_i16 & 9292_i16;
_3 = _1 >> _4.1;
_4.0.0.0 = '\u{b402b}';
_10.2 = [_9,_9,_9,_9,_9,_9,_9,_9];
_10.2 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4.1 = !214_u8;
Goto(bb2)
}
bb2 = {
_9 = _8 as i16;
_11 = _4.0.0.0;
_8 = 3620664615634043623_u64 as f32;
_10.2 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11 = _4.0.0.0;
_8 = 113972188355286616242320006940437949902_u128 as f32;
_9 = -16510_i16;
_4.1 = !190_u8;
_2 = _4.1 as isize;
_9 = (-5625_i16) * (-32498_i16);
_8 = _9 as f32;
_4.1 = !126_u8;
_4.1 = !26_u8;
_13 = _9 & _9;
_4.1 = 213_u8 ^ 11_u8;
_1 = 102_i8 as i32;
_10.2 = [_9,_13,_9,_13,_13,_13,_13,_9];
_2 = -(-116_isize);
_4.0.0.0 = _11;
_11 = _4.0.0.0;
_8 = 861417000591688244_i64 as f32;
Goto(bb3)
}
bb3 = {
_4.0.0.0 = _11;
_3 = 119975138541649832932459538206815681499_u128 as i32;
_1 = _2 as i32;
_10.1 = core::ptr::addr_of!(_4);
_8 = 2350730083_u32 as f32;
_14 = false;
_9 = _4.1 as i16;
_4.0.0 = (_11,);
_10.2 = [_13,_13,_13,_13,_13,_9,_9,_13];
_4.1 = !118_u8;
_15 = &_5;
_10.1 = core::ptr::addr_of!(_4);
Goto(bb4)
}
bb4 = {
_10.1 = core::ptr::addr_of!(_4);
_15 = &(*_15);
_14 = !true;
Goto(bb5)
}
bb5 = {
_4.0.0.0 = _11;
_11 = _4.0.0.0;
_3 = _11 as i32;
_4.0.0 = (_11,);
_4.0.0 = (_11,);
_13 = _9 & _9;
_1 = _3 + _3;
_14 = true;
_3 = !_1;
_8 = 8961_u16 as f32;
_10.2 = [_13,_9,_13,_13,_9,_9,_13,_13];
_16 = &(*_15);
_14 = false;
_15 = &_12;
_8 = 2545401473_u32 as f32;
_15 = &(*_16);
_4.0.0.0 = _11;
_3 = _1;
_10.1 = core::ptr::addr_of!(_4);
_2 = _13 as isize;
_2 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_13 = _9;
_4.1 = 96_u8 * 161_u8;
_4.1 = 16_u8 << _9;
Call(RET = fn6(_14, _4.1, _4, Move(_10.1), _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
place!(Field::<char>(Variant(RET, 0), 1)) = _11;
place!(Field::<isize>(Variant(RET, 0), 2)) = 4068693679_u32 as isize;
place!(Field::<u16>(Variant(RET, 0), 5)) = _9 as u16;
_4.0.0.0 = Field::<char>(Variant(RET, 0), 1);
Goto(bb7)
}
bb7 = {
Call(_21 = dump_var(5_usize, 9_usize, Move(_9), 2_usize, Move(_2), 14_usize, Move(_14), 13_usize, Move(_13)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: u8,mut _3: (((char,),), u8),mut _4: *const (((char,),), u8),mut _5: i32) -> Adt44 {
mir! {
type RET = Adt44;
let _6: (isize,);
let _7: char;
let _8: char;
let _9: &'static usize;
let _10: i128;
let _11: ([u64; 3], isize);
let _12: [i16; 8];
let _13: bool;
let _14: &'static *mut u128;
let _15: *mut i128;
let _16: f64;
let _17: usize;
let _18: *const *mut [i64; 7];
let _19: f32;
let _20: isize;
let _21: u32;
let _22: [bool; 5];
let _23: isize;
let _24: [i8; 3];
let _25: f64;
let _26: f32;
let _27: isize;
let _28: bool;
let _29: *mut i128;
let _30: *const i32;
let _31: char;
let _32: (*mut (((char,),), u8), (isize,));
let _33: [u8; 6];
let _34: i16;
let _35: u128;
let _36: *mut u64;
let _37: [i8; 5];
let _38: *mut [u8; 6];
let _39: &'static [bool; 5];
let _40: f32;
let _41: f32;
let _42: f64;
let _43: [u64; 1];
let _44: &'static &'static *mut u128;
let _45: [u64; 1];
let _46: f64;
let _47: isize;
let _48: ([u64; 3], isize);
let _49: *mut bool;
let _50: (u8, &'static Adt44);
let _51: [u64; 5];
let _52: isize;
let _53: *const Adt24;
let _54: *const &'static &'static *mut u128;
let _55: *const &'static usize;
let _56: &'static u128;
let _57: [isize; 8];
let _58: isize;
let _59: isize;
let _60: (*mut (((char,),), u8), (isize,));
let _61: u128;
let _62: f64;
let _63: char;
let _64: (char,);
let _65: *mut [u8; 6];
let _66: f32;
let _67: (isize,);
let _68: Adt57;
let _69: i8;
let _70: i16;
let _71: Adt45;
let _72: isize;
let _73: isize;
let _74: isize;
let _75: isize;
let _76: u8;
let _77: isize;
let _78: i32;
let _79: f32;
let _80: f32;
let _81: isize;
let _82: u128;
let _83: u64;
let _84: i32;
let _85: &'static u128;
let _86: i64;
let _87: char;
let _88: [u8; 6];
let _89: *const Adt24;
let _90: isize;
let _91: u8;
let _92: &'static *mut u128;
let _93: (((char,),), u8);
let _94: &'static [i16; 8];
let _95: [char; 5];
let _96: *mut i128;
let _97: &'static ((((char,),), u8), *const (((char,),), u8), (i16,));
let _98: isize;
let _99: *mut [bool; 5];
let _100: i8;
let _101: [i64; 7];
let _102: f64;
let _103: isize;
let _104: &'static ((((char,),), u8), *const (((char,),), u8), (i16,));
let _105: u128;
let _106: Adt69;
let _107: [i8; 5];
let _108: ();
let _109: ();
{
_3.1 = !_2;
_3.1 = 19_i8 as u8;
_6 = ((-106_isize),);
_3.1 = _2;
_6.0 = (-9223372036854775808_isize) - 116_isize;
_3.1 = !_2;
_3.0.0.0 = '\u{8cb8c}';
_6.0 = _2 as isize;
_3.1 = _2;
_10 = (-73529050835525976729167781744075077219_i128);
_7 = _3.0.0.0;
_8 = _3.0.0.0;
_4 = core::ptr::addr_of!(_3);
_3.1 = !_2;
_7 = (*_4).0.0.0;
_2 = (*_4).1 << _5;
_5 = (-549173284_i32) & (-331713381_i32);
(*_4).0.0.0 = _8;
_4 = core::ptr::addr_of!(_3);
(*_4).0.0 = (_8,);
(*_4).0.0.0 = _7;
_3.0.0 = (_8,);
_3.0.0.0 = _7;
_3.0.0 = (_8,);
_7 = _8;
Goto(bb1)
}
bb1 = {
_3.0.0.0 = _8;
_3.0.0 = (_7,);
(*_4).0.0.0 = _7;
_5 = (-749335304_i32) >> (*_4).1;
Goto(bb2)
}
bb2 = {
(*_4).0.0.0 = _8;
_8 = (*_4).0.0.0;
_8 = _3.0.0.0;
_3.1 = _2;
_12 = [8265_i16,26702_i16,8774_i16,7741_i16,(-16074_i16),(-4935_i16),(-21883_i16),(-6435_i16)];
(*_4).0.0 = (_7,);
(*_4).0.0 = (_8,);
_12 = [3435_i16,(-7718_i16),(-21609_i16),(-1578_i16),(-23481_i16),(-8711_i16),15914_i16,(-18774_i16)];
_13 = _1;
_3.1 = 14889150197529871939_u64 as u8;
_6.0 = 12036037750578202070_u64 as isize;
(*_4).0.0 = (_7,);
_2 = (-17755_i16) as u8;
_7 = (*_4).0.0.0;
(*_4).1 = !_2;
_8 = _3.0.0.0;
_3.0.0 = (_7,);
(*_4).0.0 = (_7,);
_12 = [(-8451_i16),4444_i16,(-24917_i16),25075_i16,28386_i16,19747_i16,(-19899_i16),(-26544_i16)];
_11.0 = [15951484692660290505_u64,6576001470001482018_u64,16229401652674799292_u64];
_7 = (*_4).0.0.0;
_1 = _13;
_8 = (*_4).0.0.0;
_3.1 = _2;
(*_4).0.0 = (_8,);
_3.0.0 = (_7,);
Goto(bb3)
}
bb3 = {
_6.0 = 17720878158764448654_u64 as isize;
_3.0.0.0 = _8;
_3.0.0.0 = _7;
_6 = ((-82_isize),);
_11.1 = -_6.0;
(*_4).1 = _2 | _2;
_3.1 = !_2;
_6.0 = 1039855384_u32 as isize;
Call(_11.1 = fn7((*_4).0.0, _11.0, _6.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = core::ptr::addr_of_mut!(_10);
_2 = (-67_i8) as u8;
_11.0 = [10602537739508792798_u64,6169967045881820168_u64,6631613286779947892_u64];
(*_4).0.0.0 = _8;
match (*_15) {
266753316085412486734206825687693134237 => bb5,
_ => bb3
}
}
bb5 = {
_13 = _1;
_15 = core::ptr::addr_of_mut!(_10);
_11.1 = _6.0 * _6.0;
_7 = (*_4).0.0.0;
_4 = core::ptr::addr_of!((*_4));
_6.0 = !_11.1;
_6 = (_11.1,);
_3.0.0 = (_7,);
_7 = (*_4).0.0.0;
match (*_15) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
266753316085412486734206825687693134237 => bb9,
_ => bb8
}
}
bb6 = {
_15 = core::ptr::addr_of_mut!(_10);
_2 = (-67_i8) as u8;
_11.0 = [10602537739508792798_u64,6169967045881820168_u64,6631613286779947892_u64];
(*_4).0.0.0 = _8;
match (*_15) {
266753316085412486734206825687693134237 => bb5,
_ => bb3
}
}
bb7 = {
_6.0 = 17720878158764448654_u64 as isize;
_3.0.0.0 = _8;
_3.0.0.0 = _7;
_6 = ((-82_isize),);
_11.1 = -_6.0;
(*_4).1 = _2 | _2;
_3.1 = !_2;
_6.0 = 1039855384_u32 as isize;
Call(_11.1 = fn7((*_4).0.0, _11.0, _6.0), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_3.0.0.0 = _8;
_3.0.0 = (_7,);
(*_4).0.0.0 = _7;
_5 = (-749335304_i32) >> (*_4).1;
Goto(bb2)
}
bb9 = {
_3.0.0 = (_7,);
(*_4).0.0 = (_8,);
_3.1 = _2;
(*_4).1 = _2;
_12 = [12105_i16,(-25344_i16),(-270_i16),15686_i16,28755_i16,15111_i16,25070_i16,2917_i16];
_8 = _7;
_4 = core::ptr::addr_of!(_3);
(*_4).1 = _2 * _2;
_11.0 = [5619921705920131844_u64,16763154806542482990_u64,1400687931757615108_u64];
_1 = _13 | _13;
(*_4).0.0.0 = _8;
_3.0.0 = (_8,);
_13 = _1;
_3.1 = 13532975808721017901_u64 as u8;
_3.0.0.0 = _7;
(*_4).1 = _2;
_19 = 3241872327712708116_i64 as f32;
_3.0.0 = (_7,);
_13 = !_1;
(*_4).1 = _5 as u8;
_17 = 7009326329641336785_usize;
Call((*_4) = fn8(_7, _6, _7, _12, _7, (*_15), _12, _11), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6.0 = !_11.1;
(*_4).0.0.0 = _7;
_1 = !_13;
match (*_4).1 {
0 => bb5,
1 => bb7,
82 => bb12,
_ => bb11
}
}
bb11 = {
_3.0.0 = (_7,);
(*_4).0.0 = (_8,);
_3.1 = _2;
(*_4).1 = _2;
_12 = [12105_i16,(-25344_i16),(-270_i16),15686_i16,28755_i16,15111_i16,25070_i16,2917_i16];
_8 = _7;
_4 = core::ptr::addr_of!(_3);
(*_4).1 = _2 * _2;
_11.0 = [5619921705920131844_u64,16763154806542482990_u64,1400687931757615108_u64];
_1 = _13 | _13;
(*_4).0.0.0 = _8;
_3.0.0 = (_8,);
_13 = _1;
_3.1 = 13532975808721017901_u64 as u8;
_3.0.0.0 = _7;
(*_4).1 = _2;
_19 = 3241872327712708116_i64 as f32;
_3.0.0 = (_7,);
_13 = !_1;
(*_4).1 = _5 as u8;
_17 = 7009326329641336785_usize;
Call((*_4) = fn8(_7, _6, _7, _12, _7, (*_15), _12, _11), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_9 = &_17;
Goto(bb13)
}
bb13 = {
_17 = (*_15) as usize;
_16 = _19 as f64;
_8 = (*_4).0.0.0;
_20 = _11.1 | _6.0;
_11.0 = [10793895860242572763_u64,4323158849940582331_u64,2925208780482560920_u64];
_9 = &_17;
(*_4).0.0.0 = _7;
Goto(bb14)
}
bb14 = {
_12 = [19411_i16,11292_i16,(-17044_i16),20230_i16,(-11778_i16),(-5541_i16),535_i16,(-24655_i16)];
(*_4).1 = _2 ^ _2;
_22 = [_13,_13,_13,_13,_13];
(*_4).1 = _2 * _2;
_7 = _8;
_3.0.0.0 = _7;
(*_15) = 1883221165_u32 as i128;
_15 = core::ptr::addr_of_mut!((*_15));
_3.1 = _2;
_3.0.0.0 = _7;
_11.0 = [12148092286478380395_u64,16726256410080544172_u64,3958500193238501587_u64];
(*_15) = 50131_u16 as i128;
_11.1 = -_20;
_24 = [(-21_i8),(-84_i8),(-104_i8)];
(*_15) = 139801193640006342590335898080745293560_i128 ^ (-47435566464273443003885496813339192793_i128);
Goto(bb15)
}
bb15 = {
(*_4).0.0 = (_8,);
(*_4).1 = !_2;
_23 = 30928_i16 as isize;
_17 = 1_usize + 5_usize;
_13 = _1 ^ _1;
_5 = (-1560849032_i32);
_22 = [_13,_1,_13,_13,_1];
_29 = Move(_15);
_21 = _17 as u32;
_21 = _16 as u32;
_22 = [_13,_1,_13,_13,_13];
_26 = _19 + _19;
_22 = [_13,_1,_13,_1,_13];
(*_4).0.0 = (_7,);
_20 = _13 as isize;
(*_4).0.0 = (_8,);
_29 = core::ptr::addr_of_mut!(_10);
match _5 {
0 => bb13,
1 => bb10,
2 => bb16,
3 => bb17,
340282366920938463463374607430207362424 => bb19,
_ => bb18
}
}
bb16 = {
_3.0.0.0 = _8;
_3.0.0 = (_7,);
(*_4).0.0.0 = _7;
_5 = (-749335304_i32) >> (*_4).1;
Goto(bb2)
}
bb17 = {
_13 = _1;
_15 = core::ptr::addr_of_mut!(_10);
_11.1 = _6.0 * _6.0;
_7 = (*_4).0.0.0;
_4 = core::ptr::addr_of!((*_4));
_6.0 = !_11.1;
_6 = (_11.1,);
_3.0.0 = (_7,);
_7 = (*_4).0.0.0;
match (*_15) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
266753316085412486734206825687693134237 => bb9,
_ => bb8
}
}
bb18 = {
_6.0 = 17720878158764448654_u64 as isize;
_3.0.0.0 = _8;
_3.0.0.0 = _7;
_6 = ((-82_isize),);
_11.1 = -_6.0;
(*_4).1 = _2 | _2;
_3.1 = !_2;
_6.0 = 1039855384_u32 as isize;
Call(_11.1 = fn7((*_4).0.0, _11.0, _6.0), ReturnTo(bb4), UnwindUnreachable())
}
bb19 = {
_28 = _13;
_3.0.0.0 = _7;
_21 = _10 as u32;
_9 = &_17;
_29 = core::ptr::addr_of_mut!(_10);
_25 = 5546958019940472572345052890922497081_u128 as f64;
_27 = 25_i8 as isize;
_9 = &(*_9);
(*_29) = (-50441402265754933618786368453949495288_i128);
(*_29) = 103319561182603707566594704674546455580_i128;
_21 = 23073_u16 as u32;
_30 = core::ptr::addr_of!(_5);
_17 = 5385181249781884018_usize - 18086877715411980720_usize;
_6.0 = !_11.1;
_12 = [(-7311_i16),30174_i16,(-605_i16),24464_i16,(-27446_i16),(-32056_i16),(-176_i16),(-27939_i16)];
_32.1 = (_6.0,);
_33 = [(*_4).1,_3.1,_3.1,_2,(*_4).1,_3.1];
_32.1.0 = _27 >> _27;
_31 = _7;
Call(_35 = core::intrinsics::transmute(_12), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_7 = (*_4).0.0.0;
_24 = [18_i8,(-69_i8),120_i8];
_6 = _32.1;
(*_29) = _17 as i128;
_6.0 = _27;
_3.0.0.0 = _31;
_34 = -(-27778_i16);
_11.1 = -_23;
_26 = _21 as f32;
(*_4).1 = _2 - _2;
match (*_30) {
0 => bb1,
1 => bb5,
2 => bb21,
3 => bb22,
4 => bb23,
340282366920938463463374607430207362424 => bb25,
_ => bb24
}
}
bb21 = {
_15 = core::ptr::addr_of_mut!(_10);
_2 = (-67_i8) as u8;
_11.0 = [10602537739508792798_u64,6169967045881820168_u64,6631613286779947892_u64];
(*_4).0.0.0 = _8;
match (*_15) {
266753316085412486734206825687693134237 => bb5,
_ => bb3
}
}
bb22 = {
_15 = core::ptr::addr_of_mut!(_10);
_2 = (-67_i8) as u8;
_11.0 = [10602537739508792798_u64,6169967045881820168_u64,6631613286779947892_u64];
(*_4).0.0.0 = _8;
match (*_15) {
266753316085412486734206825687693134237 => bb5,
_ => bb3
}
}
bb23 = {
_12 = [19411_i16,11292_i16,(-17044_i16),20230_i16,(-11778_i16),(-5541_i16),535_i16,(-24655_i16)];
(*_4).1 = _2 ^ _2;
_22 = [_13,_13,_13,_13,_13];
(*_4).1 = _2 * _2;
_7 = _8;
_3.0.0.0 = _7;
(*_15) = 1883221165_u32 as i128;
_15 = core::ptr::addr_of_mut!((*_15));
_3.1 = _2;
_3.0.0.0 = _7;
_11.0 = [12148092286478380395_u64,16726256410080544172_u64,3958500193238501587_u64];
(*_15) = 50131_u16 as i128;
_11.1 = -_20;
_24 = [(-21_i8),(-84_i8),(-104_i8)];
(*_15) = 139801193640006342590335898080745293560_i128 ^ (-47435566464273443003885496813339192793_i128);
Goto(bb15)
}
bb24 = {
_17 = (*_15) as usize;
_16 = _19 as f64;
_8 = (*_4).0.0.0;
_20 = _11.1 | _6.0;
_11.0 = [10793895860242572763_u64,4323158849940582331_u64,2925208780482560920_u64];
_9 = &_17;
(*_4).0.0.0 = _7;
Goto(bb14)
}
bb25 = {
_27 = _20 * _6.0;
_25 = _16 * _16;
_32.0 = core::ptr::addr_of_mut!((*_4));
_30 = core::ptr::addr_of!((*_30));
_7 = _31;
(*_30) = 99_i8 as i32;
_31 = _7;
_15 = Move(_29);
(*_30) = 1262714235_i32 ^ (-1235801584_i32);
_22 = [_28,_28,_13,_28,_13];
_37 = [12_i8,13_i8,(-103_i8),(-97_i8),49_i8];
_3.0.0.0 = _8;
_3.0.0.0 = _8;
_4 = core::ptr::addr_of!((*_4));
_37 = [84_i8,21_i8,28_i8,81_i8,65_i8];
_17 = 5_usize * 0_usize;
_27 = _20 ^ _20;
(*_4).0.0.0 = _8;
(*_4).0.0.0 = _8;
_43 = [10177589620031435487_u64];
(*_4).0.0.0 = _31;
(*_4).1 = _34 as u8;
Goto(bb26)
}
bb26 = {
_4 = core::ptr::addr_of!((*_4));
_30 = core::ptr::addr_of!((*_30));
_21 = 3657580606_u32 << _27;
_44 = &_14;
_8 = _31;
_1 = _28;
_31 = _3.0.0.0;
Goto(bb27)
}
bb27 = {
_23 = (*_4).1 as isize;
_46 = _10 as f64;
_3.1 = _2;
(*_30) = (-20642296_i32);
_6 = _32.1;
(*_4).0.0 = (_31,);
_42 = -_46;
_41 = _19;
_6 = (_27,);
_28 = _13 ^ _1;
_32.1.0 = _23 | _6.0;
_7 = _3.0.0.0;
match (*_30) {
0 => bb1,
340282366920938463463374607431747569160 => bb28,
_ => bb15
}
}
bb28 = {
_23 = _5 as isize;
(*_4).0.0.0 = _8;
(*_4).0.0.0 = _7;
_48.0 = _11.0;
_39 = &_22;
_37 = [(-90_i8),(-1_i8),(-87_i8),(-99_i8),(-41_i8)];
(*_30) = (*_4).1 as i32;
_49 = core::ptr::addr_of_mut!(_28);
_27 = (-6294563649911595802_i64) as isize;
_7 = _31;
_6.0 = -_32.1.0;
_32.0 = core::ptr::addr_of_mut!(_3);
_3.0.0 = (_7,);
_17 = (*_30) as usize;
_45 = [5422820543342591229_u64];
_44 = &(*_44);
_23 = !_6.0;
_29 = Move(_15);
_17 = 3_usize * 2_usize;
_32.1.0 = _6.0 | _23;
_10 = (*_30) as i128;
_25 = 6431281024805064395_u64 as f64;
_48.0 = _11.0;
Goto(bb29)
}
bb29 = {
_13 = _1;
_41 = _46 as f32;
_54 = core::ptr::addr_of!(_44);
_39 = &(*_39);
_50.0 = !_2;
_4 = core::ptr::addr_of!(_3);
(*_54) = &(*_44);
_25 = -_46;
_12 = [_34,_34,_34,_34,_34,_34,_34,_34];
_50.1 = &RET;
(*_4).1 = _2;
Goto(bb30)
}
bb30 = {
_24 = [48_i8,(-67_i8),118_i8];
_29 = core::ptr::addr_of_mut!(_10);
(*_54) = &(*_44);
_29 = core::ptr::addr_of_mut!((*_29));
_55 = core::ptr::addr_of!(_9);
_51 = [18322082198456483205_u64,5191363752907278595_u64,9184354448354018543_u64,171682544408159624_u64,9785615354203557444_u64];
_48 = (_11.0, _6.0);
_56 = &_35;
_55 = core::ptr::addr_of!((*_55));
_60.1 = (_23,);
_40 = 28013_u16 as f32;
_49 = core::ptr::addr_of_mut!(_13);
_63 = _8;
_64.0 = _63;
_38 = core::ptr::addr_of_mut!(_33);
(*_54) = &(*_44);
(*_29) = 87982195386533604431002910916462580008_i128 ^ (-40592412308954278076813794727205507634_i128);
_67.0 = _32.1.0;
Goto(bb31)
}
bb31 = {
(*_54) = &(*_44);
_57 = [_23,_48.1,_32.1.0,_48.1,_67.0,_23,_32.1.0,_32.1.0];
(*_38) = [_2,_3.1,_3.1,_3.1,(*_4).1,_3.1];
(*_4).1 = _2;
_13 = !_28;
_60 = (Move(_32.0), _67);
_45 = _43;
_46 = -_42;
(*_49) = _1;
_39 = &_22;
(*_4).0.0.0 = _64.0;
_20 = -_60.1.0;
_74 = _23;
_62 = _5 as f64;
_65 = Move(_38);
_20 = -_67.0;
_16 = _25;
_31 = _3.0.0.0;
_22 = [_13,_28,(*_49),(*_49),_28];
_50.0 = _2;
_60.1.0 = _34 as isize;
Call(_45 = core::intrinsics::transmute(_6.0), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_32.0 = Move(_60.0);
_46 = _62 + _62;
(*_4).0 = (_64,);
_39 = &_22;
_31 = (*_4).0.0.0;
_58 = -_67.0;
(*_55) = &_17;
Goto(bb33)
}
bb33 = {
_60.1.0 = _20;
_69 = 22_i8;
_75 = _23;
(*_4).1 = _2;
_28 = _13;
_6 = (_20,);
_38 = Move(_65);
_77 = -_58;
_45 = _43;
_34 = (-11012_i16) * 3800_i16;
_3.0 = (_64,);
_49 = core::ptr::addr_of_mut!((*_49));
_63 = _8;
match _69 {
0 => bb34,
22 => bb36,
_ => bb35
}
}
bb34 = {
_7 = (*_4).0.0.0;
_24 = [18_i8,(-69_i8),120_i8];
_6 = _32.1;
(*_29) = _17 as i128;
_6.0 = _27;
_3.0.0.0 = _31;
_34 = -(-27778_i16);
_11.1 = -_23;
_26 = _21 as f32;
(*_4).1 = _2 - _2;
match (*_30) {
0 => bb1,
1 => bb5,
2 => bb21,
3 => bb22,
4 => bb23,
340282366920938463463374607430207362424 => bb25,
_ => bb24
}
}
bb35 = {
_6.0 = 17720878158764448654_u64 as isize;
_3.0.0.0 = _8;
_3.0.0.0 = _7;
_6 = ((-82_isize),);
_11.1 = -_6.0;
(*_4).1 = _2 | _2;
_3.1 = !_2;
_6.0 = 1039855384_u32 as isize;
Call(_11.1 = fn7((*_4).0.0, _11.0, _6.0), ReturnTo(bb4), UnwindUnreachable())
}
bb36 = {
_51 = [11009646739889871545_u64,9621903202054143387_u64,7443859042820486545_u64,15926666399712610816_u64,16858779040931458639_u64];
_6.0 = !_77;
_3.0.0 = _64;
_40 = _46 as f32;
_45 = _43;
_46 = -_25;
_5 = (-915040314_i32) ^ 1566323294_i32;
_69 = !(-47_i8);
Goto(bb37)
}
bb37 = {
_57 = [_58,_20,_11.1,_67.0,_32.1.0,_32.1.0,_77,_6.0];
Goto(bb38)
}
bb38 = {
_32.0 = core::ptr::addr_of_mut!(_3);
_29 = core::ptr::addr_of_mut!((*_29));
_59 = _26 as isize;
(*_54) = &(*_44);
_27 = _6.0;
_60.0 = Move(_32.0);
Goto(bb39)
}
bb39 = {
_67 = (_58,);
_75 = _48.1;
_48.1 = 4582_u16 as isize;
_10 = (-854130041410594452107345264438212005_i128);
_78 = -(*_30);
_79 = _40 - _40;
_32 = (Move(_60.0), _67);
_15 = core::ptr::addr_of_mut!(_10);
_32.0 = core::ptr::addr_of_mut!(_3);
_65 = core::ptr::addr_of_mut!(_33);
_32.0 = core::ptr::addr_of_mut!(_3);
_82 = (*_56);
_8 = _64.0;
_85 = Move(_56);
(*_4).1 = _50.0 ^ _50.0;
Call(_1 = fn18(), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
_28 = _13 < _13;
_60.0 = Move(_32.0);
_11 = _48;
_4 = core::ptr::addr_of!((*_4));
(*_55) = &(*_9);
_70 = _34;
(*_4).0 = (_64,);
(*_4).0.0 = _64;
_35 = !_82;
_23 = _6.0;
_32.1.0 = _58;
(*_55) = &(*_9);
_23 = _27;
_3.0 = (_64,);
_50.1 = &RET;
_40 = _41 - _26;
_87 = (*_4).0.0.0;
_67.0 = _82 as isize;
_58 = _3.1 as isize;
_66 = -_79;
match (*_15) {
339428236879527869011267262167329999451 => bb41,
_ => bb27
}
}
bb41 = {
_24 = [_69,_69,_69];
_52 = _74 - _32.1.0;
_63 = (*_4).0.0.0;
_15 = core::ptr::addr_of_mut!((*_29));
_48.0 = [17325944114148571149_u64,11951229330432783944_u64,16722805293139903678_u64];
_32 = (Move(_60.0), _6);
_55 = core::ptr::addr_of!(_9);
_60.0 = Move(_32.0);
_93.0.0.0 = (*_4).0.0.0;
Goto(bb42)
}
bb42 = {
_60.1 = _32.1;
_30 = core::ptr::addr_of!(_78);
_26 = _66;
_60.1 = _6;
_64 = ((*_4).0.0.0,);
_72 = _42 as isize;
_11.1 = _28 as isize;
_70 = _34;
_11.1 = -_23;
(*_4).0.0.0 = _93.0.0.0;
_55 = core::ptr::addr_of!((*_55));
(*_4).0 = (_64,);
_3.0 = (_93.0.0,);
_44 = &(*_44);
_67 = (_60.1.0,);
_10 = (-3792667306722732967_i64) as i128;
_46 = _42;
_48.1 = _6.0;
_56 = &_61;
_55 = core::ptr::addr_of!(_9);
_8 = _87;
_69 = _31 as i8;
_60.1 = _6;
Goto(bb43)
}
bb43 = {
(*_49) = _28;
_38 = Move(_65);
_11 = (_48.0, _52);
_63 = _93.0.0.0;
Goto(bb44)
}
bb44 = {
(*_30) = -_5;
_95 = [_31,_3.0.0.0,(*_4).0.0.0,_87,_31];
_65 = core::ptr::addr_of_mut!(_88);
_64 = (_7,);
_85 = &(*_56);
RET = Adt44::Variant0 { fld0: (*_4).1,fld1: _63,fld2: _6.0,fld3: _95,fld4: _25,fld5: 40680_u16 };
_91 = !(*_4).1;
_32.0 = core::ptr::addr_of_mut!((*_4));
place!(Field::<char>(Variant(RET, 0), 1)) = _8;
_87 = (*_4).0.0.0;
_67.0 = _75 & _77;
place!(Field::<isize>(Variant(RET, 0), 2)) = (*_9) as isize;
_8 = _93.0.0.0;
Goto(bb45)
}
bb45 = {
_32 = (Move(_60.0), _67);
_33 = [Field::<u8>(Variant(RET, 0), 0),Field::<u8>(Variant(RET, 0), 0),Field::<u8>(Variant(RET, 0), 0),(*_4).1,_91,(*_4).1];
_54 = core::ptr::addr_of!((*_54));
_9 = &(*_9);
Goto(bb46)
}
bb46 = {
_51 = [3570199365637118919_u64,6792310504766097393_u64,5856262068463851833_u64,9211812085543826839_u64,12017621825395995368_u64];
(*_4) = (_93.0, Field::<u8>(Variant(RET, 0), 0));
_81 = _74 << _67.0;
_11.0 = _48.0;
_60 = (Move(_32.0), _67);
Goto(bb47)
}
bb47 = {
(*_4).0.0.0 = _93.0.0.0;
_48.0 = [9386090551581125252_u64,17178023742584792505_u64,10936736761129859427_u64];
(*_15) = (-139744999336469934546266860518664946726_i128) >> _32.1.0;
(*_4) = (_93.0, _2);
_48 = _11;
_99 = core::ptr::addr_of_mut!(_22);
_37 = [_69,_69,_69,_69,_69];
_54 = core::ptr::addr_of!(_44);
place!(Field::<char>(Variant(RET, 0), 1)) = _8;
_64 = (_87,);
_78 = -_5;
_73 = _81 >> _52;
place!(Field::<u16>(Variant(RET, 0), 5)) = _25 as u16;
(*_54) = &(*_44);
(*_30) = _5;
Goto(bb48)
}
bb48 = {
Call(_108 = dump_var(6_usize, 43_usize, Move(_43), 24_usize, Move(_24), 74_usize, Move(_74), 10_usize, Move(_10)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_108 = dump_var(6_usize, 23_usize, Move(_23), 22_usize, Move(_22), 70_usize, Move(_70), 63_usize, Move(_63)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_108 = dump_var(6_usize, 91_usize, Move(_91), 33_usize, Move(_33), 34_usize, Move(_34), 12_usize, Move(_12)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_108 = dump_var(6_usize, 64_usize, Move(_64), 1_usize, Move(_1), 57_usize, Move(_57), 28_usize, Move(_28)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_108 = dump_var(6_usize, 6_usize, Move(_6), 7_usize, Move(_7), 75_usize, Move(_75), 17_usize, Move(_17)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_108 = dump_var(6_usize, 81_usize, Move(_81), 3_usize, Move(_3), 48_usize, Move(_48), 45_usize, Move(_45)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: (char,),mut _2: [u64; 3],mut _3: isize) -> isize {
mir! {
type RET = isize;
let _4: f32;
let _5: u16;
let _6: i8;
let _7: &'static &'static *mut u128;
let _8: bool;
let _9: (isize,);
let _10: bool;
let _11: (i16,);
let _12: ();
let _13: ();
{
RET = _3 + _3;
_3 = RET << RET;
_3 = -RET;
_2 = [5005380453247581166_u64,12419249499244823597_u64,11342005054849079661_u64];
_2 = [16711501349424462106_u64,17684093793049654021_u64,1604850377421703596_u64];
_1 = ('\u{11cf}',);
_3 = RET & RET;
_1 = ('\u{1efe1}',);
_1 = ('\u{371f0}',);
_1 = ('\u{b09ad}',);
_4 = _3 as f32;
_4 = (-15133_i16) as f32;
_5 = 56227_u16;
_3 = RET;
_2 = [4657351257814887871_u64,1405158851743133684_u64,17831583398181031615_u64];
_1 = ('\u{a2a5b}',);
_4 = 0_usize as f32;
_5 = !41516_u16;
Goto(bb1)
}
bb1 = {
_1.0 = '\u{52f64}';
_5 = !21067_u16;
_1.0 = '\u{dd975}';
_3 = -RET;
_4 = (-59_i8) as f32;
_1 = ('\u{2b8e4}',);
_5 = 42700_u16;
_5 = !1764_u16;
_8 = !false;
_5 = 133812290_i32 as u16;
_9.0 = !_3;
Goto(bb2)
}
bb2 = {
RET = (-103_i8) as isize;
_3 = RET | _9.0;
_9.0 = 116_u8 as isize;
RET = 9409314002931172749_u64 as isize;
_8 = !true;
RET = _4 as isize;
_6 = !(-108_i8);
RET = _3 | _3;
_3 = RET & RET;
_5 = !13187_u16;
_4 = _6 as f32;
RET = _3;
_6 = 65_i8 ^ 121_i8;
RET = _9.0;
RET = 78961800253979901621185585815865506857_i128 as isize;
_2 = [15978360461803202953_u64,14269046598160561031_u64,14567795553056516198_u64];
_9 = (_3,);
RET = _3 & _9.0;
_3 = _9.0 ^ RET;
_5 = !24903_u16;
_6 = 103_i8;
_9 = (RET,);
_1 = ('\u{8d85f}',);
_6 = _4 as i8;
_1.0 = '\u{a285d}';
_6 = -(-67_i8);
_4 = 7939459948004969718_u64 as f32;
RET = _3 << _9.0;
RET = !_3;
_4 = 4099593130864531209_u64 as f32;
Goto(bb3)
}
bb3 = {
Call(_12 = dump_var(7_usize, 1_usize, Move(_1), 8_usize, Move(_8), 2_usize, Move(_2), 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: char,mut _2: (isize,),mut _3: char,mut _4: [i16; 8],mut _5: char,mut _6: i128,mut _7: [i16; 8],mut _8: ([u64; 3], isize)) -> (((char,),), u8) {
mir! {
type RET = (((char,),), u8);
let _9: [bool; 5];
let _10: (Adt45,);
let _11: *const Adt45;
let _12: Adt48;
let _13: bool;
let _14: ((char,),);
let _15: f64;
let _16: f64;
let _17: (char,);
let _18: Adt24;
let _19: u128;
let _20: &'static usize;
let _21: f64;
let _22: *const Adt45;
let _23: isize;
let _24: &'static usize;
let _25: &'static f64;
let _26: (Adt45,);
let _27: u8;
let _28: [u64; 5];
let _29: ();
let _30: ();
{
RET.0.0 = (_3,);
_2 = (_8.1,);
_3 = _1;
_2.0 = _3 as isize;
_4 = [30253_i16,19525_i16,(-10928_i16),19995_i16,(-16630_i16),(-24560_i16),(-24329_i16),(-5212_i16)];
_2.0 = 219168869888321453752768386147578750088_u128 as isize;
_2.0 = _8.1 | _8.1;
_5 = _1;
_6 = !(-129019235173517303641520377804759750500_i128);
_11 = core::ptr::addr_of!(_10.0);
_8.1 = 3545771794_u32 as isize;
Call(_1 = fn9(_2.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = (_8.1,);
RET.0.0 = (_5,);
RET.0.0 = (_5,);
_2 = (_8.1,);
RET.1 = 73_u8;
RET.0.0 = (_5,);
RET.1 = !115_u8;
_2 = (_8.1,);
RET.0.0 = (_3,);
_6 = _8.1 as i128;
_13 = true ^ true;
_3 = _1;
_9 = [_13,_13,_13,_13,_13];
_3 = _5;
RET.1 = !253_u8;
Goto(bb2)
}
bb2 = {
_11 = core::ptr::addr_of!((*_11));
_14.0.0 = _1;
_8.1 = -_2.0;
RET.1 = 137_u8 ^ 192_u8;
_7 = _4;
RET.0 = _14;
_14 = (RET.0.0,);
Goto(bb3)
}
bb3 = {
RET.0.0.0 = _3;
_14 = (RET.0.0,);
RET.0.0 = (_1,);
_16 = (-1703501358_i32) as f64;
_15 = (-53_i8) as f64;
_2 = (_8.1,);
RET = (_14, 82_u8);
_6 = 18113493224537969467_u64 as i128;
_5 = RET.0.0.0;
RET.0.0 = (_1,);
_17.0 = _5;
_16 = -_15;
_2 = (_8.1,);
_17 = _14.0;
RET.0.0.0 = _3;
_3 = RET.0.0.0;
_2.0 = 8864_u16 as isize;
_8.1 = _2.0;
_13 = true;
_2.0 = _8.1;
_8.0 = [17401411847515319953_u64,10640457656493546226_u64,15023578330901962059_u64];
_18 = Adt24 { fld0: _13,fld1: _6,fld2: RET.1,fld3: 7020973714159538922_u64 };
_14.0.0 = _17.0;
_8.1 = !_2.0;
_2 = (_8.1,);
Goto(bb4)
}
bb4 = {
_18 = Adt24 { fld0: _13,fld1: _6,fld2: RET.1,fld3: 7084124692042030766_u64 };
_14.0.0 = RET.0.0.0;
_18.fld2 = RET.1;
RET.0.0 = _14.0;
RET.0.0 = _14.0;
_18.fld3 = !17410669279683186068_u64;
RET.0.0.0 = _14.0.0;
RET.0 = (_17,);
RET.0 = (_17,);
RET.0 = (_17,);
_13 = _18.fld0;
_2.0 = 8436_i16 as isize;
_5 = _1;
_17 = (RET.0.0.0,);
_14.0 = (_5,);
_19 = 186436062258307791473422758451271506873_u128;
_11 = core::ptr::addr_of!((*_11));
RET.1 = (-79_i8) as u8;
_3 = _17.0;
RET.1 = _18.fld2 % _18.fld2;
RET.0.0 = (_1,);
_2.0 = _8.1 << _18.fld2;
_2.0 = _8.1;
match _19 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
186436062258307791473422758451271506873 => bb12,
_ => bb11
}
}
bb5 = {
RET.0.0.0 = _3;
_14 = (RET.0.0,);
RET.0.0 = (_1,);
_16 = (-1703501358_i32) as f64;
_15 = (-53_i8) as f64;
_2 = (_8.1,);
RET = (_14, 82_u8);
_6 = 18113493224537969467_u64 as i128;
_5 = RET.0.0.0;
RET.0.0 = (_1,);
_17.0 = _5;
_16 = -_15;
_2 = (_8.1,);
_17 = _14.0;
RET.0.0.0 = _3;
_3 = RET.0.0.0;
_2.0 = 8864_u16 as isize;
_8.1 = _2.0;
_13 = true;
_2.0 = _8.1;
_8.0 = [17401411847515319953_u64,10640457656493546226_u64,15023578330901962059_u64];
_18 = Adt24 { fld0: _13,fld1: _6,fld2: RET.1,fld3: 7020973714159538922_u64 };
_14.0.0 = _17.0;
_8.1 = !_2.0;
_2 = (_8.1,);
Goto(bb4)
}
bb6 = {
_11 = core::ptr::addr_of!((*_11));
_14.0.0 = _1;
_8.1 = -_2.0;
RET.1 = 137_u8 ^ 192_u8;
_7 = _4;
RET.0 = _14;
_14 = (RET.0.0,);
Goto(bb3)
}
bb7 = {
_2 = (_8.1,);
RET.0.0 = (_5,);
RET.0.0 = (_5,);
_2 = (_8.1,);
RET.1 = 73_u8;
RET.0.0 = (_5,);
RET.1 = !115_u8;
_2 = (_8.1,);
RET.0.0 = (_3,);
_6 = _8.1 as i128;
_13 = true ^ true;
_3 = _1;
_9 = [_13,_13,_13,_13,_13];
_3 = _5;
RET.1 = !253_u8;
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
_8.0 = [_18.fld3,_18.fld3,_18.fld3];
_19 = 225725403248126116962387214461914530309_u128 * 101212676887395052450894458196507526022_u128;
RET.1 = (-990481060_i32) as u8;
_7 = [20638_i16,(-28283_i16),28476_i16,27916_i16,(-13789_i16),(-7953_i16),30599_i16,(-28444_i16)];
_15 = _16 * _16;
RET.0.0 = _14.0;
_11 = core::ptr::addr_of!((*_11));
_3 = _1;
_3 = RET.0.0.0;
_21 = _15;
_17 = _14.0;
RET = (_14, _18.fld2);
_1 = RET.0.0.0;
_14.0.0 = _5;
_18.fld3 = 6640737679829926012_u64;
RET = (_14, _18.fld2);
_13 = !_18.fld0;
_2 = (_8.1,);
_8.0 = [_18.fld3,_18.fld3,_18.fld3];
_22 = core::ptr::addr_of!((*_11));
_14.0 = (_5,);
RET.1 = _18.fld2 * _18.fld2;
_16 = 60733_u16 as f64;
RET.1 = _18.fld2;
Goto(bb13)
}
bb13 = {
RET.0 = _14;
_14.0.0 = _17.0;
_6 = _18.fld0 as i128;
_18.fld3 = _18.fld0 as u64;
_8.1 = (-3041_i16) as isize;
_5 = RET.0.0.0;
_17.0 = RET.0.0.0;
_18.fld0 = !_13;
_2.0 = RET.1 as isize;
_5 = RET.0.0.0;
_14 = (RET.0.0,);
_22 = core::ptr::addr_of!((*_11));
_7 = [(-17342_i16),(-16107_i16),1658_i16,11934_i16,13875_i16,(-16041_i16),(-9191_i16),11910_i16];
RET.0 = _14;
_14.0 = (_1,);
_5 = _14.0.0;
RET.0.0 = (_5,);
_6 = _18.fld1;
_9 = [_13,_13,_18.fld0,_13,_13];
RET = (_14, _18.fld2);
_5 = _14.0.0;
_21 = _15 * _16;
RET.0 = _14;
match _18.fld2 {
0 => bb1,
1 => bb4,
2 => bb14,
3 => bb15,
4 => bb16,
82 => bb18,
_ => bb17
}
}
bb14 = {
RET.0.0.0 = _3;
_14 = (RET.0.0,);
RET.0.0 = (_1,);
_16 = (-1703501358_i32) as f64;
_15 = (-53_i8) as f64;
_2 = (_8.1,);
RET = (_14, 82_u8);
_6 = 18113493224537969467_u64 as i128;
_5 = RET.0.0.0;
RET.0.0 = (_1,);
_17.0 = _5;
_16 = -_15;
_2 = (_8.1,);
_17 = _14.0;
RET.0.0.0 = _3;
_3 = RET.0.0.0;
_2.0 = 8864_u16 as isize;
_8.1 = _2.0;
_13 = true;
_2.0 = _8.1;
_8.0 = [17401411847515319953_u64,10640457656493546226_u64,15023578330901962059_u64];
_18 = Adt24 { fld0: _13,fld1: _6,fld2: RET.1,fld3: 7020973714159538922_u64 };
_14.0.0 = _17.0;
_8.1 = !_2.0;
_2 = (_8.1,);
Goto(bb4)
}
bb15 = {
_11 = core::ptr::addr_of!((*_11));
_14.0.0 = _1;
_8.1 = -_2.0;
RET.1 = 137_u8 ^ 192_u8;
_7 = _4;
RET.0 = _14;
_14 = (RET.0.0,);
Goto(bb3)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET.0.0 = (_1,);
RET.0.0.0 = _17.0;
Goto(bb19)
}
bb19 = {
Call(_29 = dump_var(8_usize, 7_usize, Move(_7), 1_usize, Move(_1), 14_usize, Move(_14), 3_usize, Move(_3)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_29 = dump_var(8_usize, 13_usize, Move(_13), 19_usize, Move(_19), 30_usize, _30, 30_usize, _30), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize) -> char {
mir! {
type RET = char;
let _2: isize;
let _3: Adt48;
let _4: char;
let _5: &'static [i8; 5];
let _6: i32;
let _7: isize;
let _8: bool;
let _9: f32;
let _10: ([u64; 3], isize);
let _11: u32;
let _12: [u8; 6];
let _13: *const &'static usize;
let _14: char;
let _15: bool;
let _16: f32;
let _17: f64;
let _18: [u8; 6];
let _19: i128;
let _20: (char,);
let _21: &'static f64;
let _22: usize;
let _23: i32;
let _24: u32;
let _25: u16;
let _26: isize;
let _27: Adt44;
let _28: &'static [bool; 5];
let _29: f64;
let _30: char;
let _31: i32;
let _32: f64;
let _33: ();
let _34: ();
{
RET = '\u{c114e}';
_2 = -_1;
Call(RET = fn10(_2, _2, _1, _1, _1, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = RET;
_4 = RET;
_1 = 619554446_u32 as isize;
RET = _4;
_1 = 11336_u16 as isize;
_4 = RET;
_6 = 1375518148_i32 + 1246297573_i32;
RET = _4;
RET = _4;
RET = _4;
RET = _4;
_1 = _2;
_7 = 72_u8 as isize;
_4 = RET;
_1 = !_2;
_7 = !_2;
Call(RET = fn15(_2, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = !true;
_1 = -_7;
_8 = !false;
RET = _4;
_7 = 2570451067848647683_u64 as isize;
_4 = RET;
RET = _4;
_4 = RET;
Goto(bb3)
}
bb3 = {
_6 = RET as i32;
_10.0 = [866883584039039211_u64,4857244222598253432_u64,7394209992838693847_u64];
_10.0 = [10884508840218927150_u64,14396914991453622260_u64,8357965933744880786_u64];
_10.1 = _2;
RET = _4;
_10.1 = RET as isize;
_8 = !false;
_1 = _7;
_9 = (-25029_i16) as f32;
_12 = [86_u8,110_u8,230_u8,12_u8,67_u8,115_u8];
Goto(bb4)
}
bb4 = {
_9 = 50_i8 as f32;
_2 = !_1;
_1 = _7;
_17 = 174_u8 as f64;
_9 = 266823214_u32 as f32;
_9 = 1_usize as f32;
_15 = !_8;
_8 = _15;
_11 = 1853534658_u32;
_14 = RET;
_16 = _9;
_12 = [88_u8,73_u8,44_u8,97_u8,71_u8,75_u8];
_18 = [177_u8,105_u8,105_u8,122_u8,69_u8,89_u8];
_4 = RET;
match _11 {
0 => bb1,
1 => bb3,
2 => bb5,
1853534658 => bb7,
_ => bb6
}
}
bb5 = {
_4 = RET;
_4 = RET;
_1 = 619554446_u32 as isize;
RET = _4;
_1 = 11336_u16 as isize;
_4 = RET;
_6 = 1375518148_i32 + 1246297573_i32;
RET = _4;
RET = _4;
RET = _4;
RET = _4;
_1 = _2;
_7 = 72_u8 as isize;
_4 = RET;
_1 = !_2;
_7 = !_2;
Call(RET = fn15(_2, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_8 = !true;
_1 = -_7;
_8 = !false;
RET = _4;
_7 = 2570451067848647683_u64 as isize;
_4 = RET;
RET = _4;
_4 = RET;
Goto(bb3)
}
bb7 = {
_14 = _4;
_15 = _8;
_20.0 = _4;
_19 = (-141907100194410403417331178360770877640_i128);
_2 = _6 as isize;
_6 = 504633697_i32 - 350926977_i32;
_16 = -_9;
_21 = &_17;
_6 = 35161411761086262786400460244853541630_u128 as i32;
_14 = _4;
_10.0 = [999073319494577920_u64,11357671675591673376_u64,1485688212411022505_u64];
_11 = 3_usize as u32;
_14 = _4;
_2 = _6 as isize;
_14 = _4;
_4 = _20.0;
_19 = 144678833818300396986889043995932150740_i128 - (-79651931764535306755461198311495500462_i128);
_14 = _4;
Goto(bb8)
}
bb8 = {
_18 = _12;
_2 = 93_i8 as isize;
_17 = _6 as f64;
_23 = _6;
_15 = _8;
_17 = _19 as f64;
RET = _14;
_11 = 2490965736_u32 * 1079921627_u32;
_17 = _11 as f64;
RET = _14;
_2 = _15 as isize;
_1 = !_2;
_21 = &_17;
_11 = 2690923676_u32;
_25 = _23 as u16;
_14 = RET;
_6 = _23;
_22 = !0_usize;
_24 = _11;
_21 = &_17;
_15 = _10.1 < _1;
_16 = _9;
Goto(bb9)
}
bb9 = {
_16 = _9;
_17 = _22 as f64;
_22 = 0_usize << _7;
_20 = (RET,);
_26 = 69_i8 as isize;
_14 = RET;
_6 = _23;
_24 = _17 as u32;
RET = _20.0;
_16 = -_9;
match _11 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb11,
4 => bb12,
2690923676 => bb14,
_ => bb13
}
}
bb10 = {
_18 = _12;
_2 = 93_i8 as isize;
_17 = _6 as f64;
_23 = _6;
_15 = _8;
_17 = _19 as f64;
RET = _14;
_11 = 2490965736_u32 * 1079921627_u32;
_17 = _11 as f64;
RET = _14;
_2 = _15 as isize;
_1 = !_2;
_21 = &_17;
_11 = 2690923676_u32;
_25 = _23 as u16;
_14 = RET;
_6 = _23;
_22 = !0_usize;
_24 = _11;
_21 = &_17;
_15 = _10.1 < _1;
_16 = _9;
Goto(bb9)
}
bb11 = {
_9 = 50_i8 as f32;
_2 = !_1;
_1 = _7;
_17 = 174_u8 as f64;
_9 = 266823214_u32 as f32;
_9 = 1_usize as f32;
_15 = !_8;
_8 = _15;
_11 = 1853534658_u32;
_14 = RET;
_16 = _9;
_12 = [88_u8,73_u8,44_u8,97_u8,71_u8,75_u8];
_18 = [177_u8,105_u8,105_u8,122_u8,69_u8,89_u8];
_4 = RET;
match _11 {
0 => bb1,
1 => bb3,
2 => bb5,
1853534658 => bb7,
_ => bb6
}
}
bb12 = {
_6 = RET as i32;
_10.0 = [866883584039039211_u64,4857244222598253432_u64,7394209992838693847_u64];
_10.0 = [10884508840218927150_u64,14396914991453622260_u64,8357965933744880786_u64];
_10.1 = _2;
RET = _4;
_10.1 = RET as isize;
_8 = !false;
_1 = _7;
_9 = (-25029_i16) as f32;
_12 = [86_u8,110_u8,230_u8,12_u8,67_u8,115_u8];
Goto(bb4)
}
bb13 = {
_4 = RET;
_4 = RET;
_1 = 619554446_u32 as isize;
RET = _4;
_1 = 11336_u16 as isize;
_4 = RET;
_6 = 1375518148_i32 + 1246297573_i32;
RET = _4;
RET = _4;
RET = _4;
RET = _4;
_1 = _2;
_7 = 72_u8 as isize;
_4 = RET;
_1 = !_2;
_7 = !_2;
Call(RET = fn15(_2, _6, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_20.0 = RET;
_20.0 = _14;
_22 = _24 as usize;
_30 = _14;
_29 = _17 - _17;
_16 = _17 as f32;
_21 = &_29;
_22 = 8437857178953584601_usize;
_31 = _6 | _6;
_20.0 = RET;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(9_usize, 22_usize, Move(_22), 12_usize, Move(_12), 10_usize, Move(_10), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(9_usize, 24_usize, Move(_24), 14_usize, Move(_14), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(9_usize, 26_usize, Move(_26), 6_usize, Move(_6), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize) -> char {
mir! {
type RET = char;
let _8: ([bool; 5],);
let _9: [i16; 8];
let _10: i8;
let _11: &'static [u8; 6];
let _12: *mut [bool; 5];
let _13: isize;
let _14: isize;
let _15: isize;
let _16: (*mut (((char,),), u8), (isize,));
let _17: *mut [i64; 7];
let _18: isize;
let _19: char;
let _20: [char; 5];
let _21: [char; 5];
let _22: &'static isize;
let _23: i32;
let _24: [char; 5];
let _25: i16;
let _26: *const &'static &'static *mut u128;
let _27: bool;
let _28: (((char,),), u8);
let _29: (isize,);
let _30: Adt45;
let _31: [u64; 2];
let _32: char;
let _33: isize;
let _34: Adt45;
let _35: isize;
let _36: [i64; 7];
let _37: *mut i128;
let _38: ();
let _39: ();
{
_2 = _4;
RET = '\u{b0403}';
_1 = _5;
_4 = 268387410599677652433932865971614651673_u128 as isize;
RET = '\u{f0dce}';
_3 = RET as isize;
_3 = !_2;
RET = '\u{88458}';
_7 = _2 >> _5;
RET = '\u{c4eda}';
_1 = _5;
RET = '\u{105126}';
_8.0 = [false,true,false,false,false];
Goto(bb1)
}
bb1 = {
_9 = [3631_i16,(-5231_i16),(-24416_i16),(-23549_i16),22152_i16,10153_i16,12265_i16,17642_i16];
RET = '\u{be213}';
_3 = !_1;
_10 = 7_i8;
_9 = [(-28993_i16),1145_i16,(-2832_i16),(-29479_i16),(-17370_i16),2716_i16,6708_i16,1131_i16];
_2 = -_7;
_7 = -_5;
_12 = core::ptr::addr_of_mut!(_8.0);
_7 = _6 << _1;
_6 = _2 + _7;
_14 = 6_usize as isize;
(*_12) = [false,false,false,true,false];
_13 = (-1436728066_i32) as isize;
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
7 => bb9,
_ => bb8
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
_10 = !(-75_i8);
_5 = -_7;
_3 = 95_u8 as isize;
RET = '\u{842fc}';
_12 = core::ptr::addr_of_mut!((*_12));
_16.1 = (_6,);
(*_12) = [false,true,true,false,false];
_12 = core::ptr::addr_of_mut!(_8.0);
_5 = 36502537036463973744083380344383222132_u128 as isize;
_14 = -_6;
_8.0 = [false,false,false,true,false];
_10 = RET as i8;
RET = '\u{d1ea4}';
_14 = _6;
_16.1.0 = _2;
_7 = !_14;
Call(_5 = fn11(), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = RET as isize;
_4 = _2;
_6 = _7 ^ _16.1.0;
_6 = _7;
(*_12) = [false,false,true,true,true];
_7 = _4 + _14;
Goto(bb11)
}
bb11 = {
_16.1 = (_7,);
(*_12) = [false,true,true,false,false];
_12 = core::ptr::addr_of_mut!((*_12));
(*_12) = [false,true,true,true,true];
(*_12) = [true,false,false,true,true];
_18 = 197_u8 as isize;
_19 = RET;
_13 = 164_u8 as isize;
_6 = _7 << _14;
Goto(bb12)
}
bb12 = {
_10 = 14_i8 ^ 53_i8;
_21 = [_19,RET,_19,_19,RET];
RET = _19;
_5 = -_7;
_8.0 = [false,true,true,true,false];
_3 = -_14;
_2 = _3 - _7;
_9 = [(-5028_i16),13073_i16,(-3824_i16),22208_i16,9277_i16,(-26691_i16),(-11299_i16),9822_i16];
_23 = (-1764140516_i32);
_20 = [RET,_19,_19,_19,_19];
_1 = !_5;
_23 = 16699563930319610849_u64 as i32;
_9 = [(-17765_i16),(-19847_i16),12380_i16,(-30067_i16),8041_i16,(-27178_i16),(-29528_i16),25530_i16];
_19 = RET;
_7 = -_6;
_24 = _21;
Goto(bb13)
}
bb13 = {
_10 = !32_i8;
_14 = !_6;
_4 = _5 ^ _14;
_15 = _7 + _4;
_16.1 = (_5,);
_28.0.0 = (RET,);
_5 = 150_u8 as isize;
_15 = 3564_u16 as isize;
_13 = !_4;
_10 = 93_i8;
_2 = _4;
_29 = (_14,);
(*_12) = [false,true,false,true,true];
_28.0.0 = (RET,);
_14 = _2;
_2 = _13 + _7;
_29.0 = true as isize;
_16.1 = (_7,);
RET = _19;
Call(_12 = fn12(_16.1, _14, _7, _14), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_4 = 2665039014_u32 as isize;
_29 = _16.1;
_28.1 = 75_u8;
_13 = _2 & _7;
_27 = !false;
_4 = 8835510733950602503_i64 as isize;
_29 = _16.1;
RET = _28.0.0.0;
_8.0 = [_27,_27,_27,_27,_27];
_32 = RET;
_22 = &_5;
_29.0 = _14;
_5 = !_29.0;
_12 = core::ptr::addr_of_mut!(_8.0);
RET = _28.0.0.0;
_28.0.0.0 = _32;
_14 = !_29.0;
_23 = _28.1 as i32;
_13 = 3_usize as isize;
_15 = _28.0.0.0 as isize;
_8.0 = [_27,_27,_27,_27,_27];
_14 = _5 >> _5;
_28.0.0 = (_19,);
_33 = -_6;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(10_usize, 20_usize, Move(_20), 33_usize, Move(_33), 29_usize, Move(_29), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(10_usize, 9_usize, Move(_9), 3_usize, Move(_3), 24_usize, Move(_24), 27_usize, Move(_27)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(10_usize, 32_usize, Move(_32), 4_usize, Move(_4), 13_usize, Move(_13), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> isize {
mir! {
type RET = isize;
let _1: i64;
let _2: ([u64; 3], isize);
let _3: ([bool; 5],);
let _4: [u64; 5];
let _5: &'static ((((char,),), u8), *const (((char,),), u8), (i16,));
let _6: Adt44;
let _7: [u64; 1];
let _8: i16;
let _9: u128;
let _10: u64;
let _11: isize;
let _12: u16;
let _13: *const Adt45;
let _14: i8;
let _15: *mut bool;
let _16: bool;
let _17: [i64; 7];
let _18: isize;
let _19: isize;
let _20: char;
let _21: isize;
let _22: (&'static usize, *const (((char,),), u8), [i16; 8]);
let _23: (*const Adt45, *mut [bool; 5], Adt48, usize);
let _24: *mut &'static ((char,),);
let _25: (&'static usize, *const (((char,),), u8), [i16; 8]);
let _26: ();
let _27: ();
{
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize - 34_isize;
RET = (-9223372036854775808_isize);
_1 = 14559438554475996339_u64 as i64;
RET = !9223372036854775807_isize;
_2.1 = RET;
_1 = (-1993342190046745482_i64) * (-6343368910342379039_i64);
RET = _2.1 & _2.1;
RET = _2.1;
RET = 2777228985_u32 as isize;
RET = '\u{36800}' as isize;
_2.0 = [8315253984712510120_u64,14963708697467120812_u64,12016123439408050137_u64];
_2.0 = [6481090524574618204_u64,1944210692609187854_u64,18105560257063299014_u64];
_2.1 = !RET;
_2.1 = RET >> _1;
_1 = _2.1 as i64;
_2.1 = 1319217561_u32 as isize;
_3.0 = [true,true,false,false,true];
Call(RET = core::intrinsics::bswap(_2.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.0 = [8617699968097908481_u64,15448132674046502669_u64,12616772145580625928_u64];
_2.0 = [17359140589504821151_u64,15121850746208512000_u64,9780447473731466213_u64];
_2.0 = [1769242509416316726_u64,9543915432670963866_u64,9284922514165243255_u64];
RET = _1 as isize;
Goto(bb2)
}
bb2 = {
_1 = 292866970_u32 as i64;
_1 = (-6132981392429633665_i64);
_4 = [1942930983308398105_u64,18177657402287007350_u64,17643364458367560905_u64,7914137133322650372_u64,15719050866273201138_u64];
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
340282366920938463457241626039338577791 => bb6,
_ => bb5
}
}
bb3 = {
_2.0 = [8617699968097908481_u64,15448132674046502669_u64,12616772145580625928_u64];
_2.0 = [17359140589504821151_u64,15121850746208512000_u64,9780447473731466213_u64];
_2.0 = [1769242509416316726_u64,9543915432670963866_u64,9284922514165243255_u64];
RET = _1 as isize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_2.0 = [15327900463889884297_u64,405337140738179344_u64,12934634099234226758_u64];
_4 = [16093267740916247087_u64,11075227121301472961_u64,11037245645929784209_u64,8547824965313805027_u64,17602695220001126060_u64];
Goto(bb7)
}
bb7 = {
_1 = 4699861158404260198_i64;
_1 = -1467771803643637326_i64;
RET = !_2.1;
_1 = -4968991687472719367_i64;
RET = -_2.1;
RET = _2.1;
_1 = true as i64;
_8 = (-26523_i16) + (-15161_i16);
_7 = [5363837153626914698_u64];
_9 = _1 as u128;
_2.0 = [5152218535221730276_u64,13360453335460458766_u64,1801279724218403105_u64];
_1 = 3888120843565164570_i64;
_2.1 = RET | RET;
_1 = 7036805066354829373_i64 * (-758690902771292334_i64);
_10 = !5226305805359146796_u64;
_1 = -(-1087918034820969721_i64);
_2.0 = [_10,_10,_10];
_10 = 11356373820938507396_u64;
_10 = 740335069_i32 as u64;
_10 = 6451334622148308702_u64 * 882879550174259949_u64;
_3.0 = [true,false,true,false,true];
RET = -_2.1;
_3.0 = [true,true,false,true,true];
_11 = !_2.1;
_11 = -_2.1;
Goto(bb8)
}
bb8 = {
RET = _9 as isize;
_4 = [_10,_10,_10,_10,_10];
_10 = !12083164849672447270_u64;
_9 = 259078747608443924280527771676522045186_u128 * 126115534107217124456152700066162822419_u128;
_12 = 43700_u16;
_3.0 = [true,false,false,false,true];
RET = _11 >> _9;
_15 = core::ptr::addr_of_mut!(_16);
RET = 976031082_u32 as isize;
_14 = 59_i8;
RET = _11;
(*_15) = _9 < _9;
_4 = [_10,_10,_10,_10,_10];
_16 = false;
_14 = '\u{100f08}' as i8;
_16 = _11 >= _2.1;
RET = 225_u8 as isize;
RET = !_2.1;
_17 = [_1,_1,_1,_1,_1,_1,_1];
_3.0 = [_16,_16,(*_15),(*_15),(*_15)];
_18 = RET;
RET = !_2.1;
(*_15) = _2.1 == _11;
(*_15) = true | true;
_12 = 25034_u16;
match _12 {
0 => bb9,
1 => bb10,
2 => bb11,
25034 => bb13,
_ => bb12
}
}
bb9 = {
_2.0 = [8617699968097908481_u64,15448132674046502669_u64,12616772145580625928_u64];
_2.0 = [17359140589504821151_u64,15121850746208512000_u64,9780447473731466213_u64];
_2.0 = [1769242509416316726_u64,9543915432670963866_u64,9284922514165243255_u64];
RET = _1 as isize;
Goto(bb2)
}
bb10 = {
_2.0 = [15327900463889884297_u64,405337140738179344_u64,12934634099234226758_u64];
_4 = [16093267740916247087_u64,11075227121301472961_u64,11037245645929784209_u64,8547824965313805027_u64,17602695220001126060_u64];
Goto(bb7)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_3.0 = [(*_15),_16,(*_15),_16,(*_15)];
_17 = [_1,_1,_1,_1,_1,_1,_1];
_3.0 = [_16,(*_15),(*_15),(*_15),(*_15)];
_10 = !14008641758524479801_u64;
_11 = 0_usize as isize;
_3.0 = [(*_15),(*_15),_16,(*_15),(*_15)];
(*_15) = true;
_17 = [_1,_1,_1,_1,_1,_1,_1];
_9 = !276804336461643339716293183553790130555_u128;
_1 = 512131852350830774_i64 >> _18;
_18 = _2.1 ^ RET;
_1 = 2924414490776068563_i64;
_15 = core::ptr::addr_of_mut!((*_15));
_18 = _2.1 | RET;
_1 = !(-3332067765278961849_i64);
_3.0 = [_16,(*_15),_16,(*_15),(*_15)];
_23.3 = 619138930128502311_usize << _8;
_18 = _2.1 ^ _11;
_22.2 = [_8,_8,_8,_8,_8,_8,_8,_8];
_19 = _18 | _18;
_17 = [_1,_1,_1,_1,_1,_1,_1];
_25.0 = &_23.3;
Goto(bb14)
}
bb14 = {
_15 = core::ptr::addr_of_mut!((*_15));
_25.2 = _22.2;
_7 = [_10];
_16 = _2.1 == _19;
_17 = [_1,_1,_1,_1,_1,_1,_1];
_8 = (-2336579121428885985588791787759002689_i128) as i16;
RET = _19;
_15 = core::ptr::addr_of_mut!((*_15));
_18 = _9 as isize;
_25.0 = &_23.3;
RET = _11 * _18;
_23.1 = core::ptr::addr_of_mut!(_3.0);
_15 = core::ptr::addr_of_mut!((*_15));
_15 = core::ptr::addr_of_mut!(_16);
_7 = [_10];
_21 = _19;
_8 = _21 as i16;
_25.2 = [_8,_8,_8,_8,_8,_8,_8,_8];
_17 = [_1,_1,_1,_1,_1,_1,_1];
_2.0 = [_10,_10,_10];
_4 = [_10,_10,_10,_10,_10];
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(11_usize, 12_usize, Move(_12), 10_usize, Move(_10), 14_usize, Move(_14), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(11_usize, 2_usize, Move(_2), 1_usize, Move(_1), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (isize,),mut _2: isize,mut _3: isize,mut _4: isize) -> *mut [bool; 5] {
mir! {
type RET = *mut [bool; 5];
let _5: &'static f64;
let _6: Adt34;
let _7: &'static Adt44;
let _8: char;
let _9: isize;
let _10: *mut &'static &'static *mut u128;
let _11: Adt24;
let _12: char;
let _13: isize;
let _14: f64;
let _15: [u64; 2];
let _16: [i8; 5];
let _17: Adt69;
let _18: isize;
let _19: &'static usize;
let _20: bool;
let _21: (char,);
let _22: [u64; 3];
let _23: isize;
let _24: bool;
let _25: [u64; 1];
let _26: Adt44;
let _27: u64;
let _28: [i64; 7];
let _29: f32;
let _30: char;
let _31: &'static [bool; 5];
let _32: u16;
let _33: isize;
let _34: [bool; 3];
let _35: bool;
let _36: f32;
let _37: i16;
let _38: ([u64; 3], isize);
let _39: bool;
let _40: u128;
let _41: (char,);
let _42: i8;
let _43: (i16,);
let _44: [u8; 6];
let _45: i64;
let _46: char;
let _47: *mut *mut &'static ((char,),);
let _48: f32;
let _49: &'static usize;
let _50: ((char,),);
let _51: bool;
let _52: f64;
let _53: Adt44;
let _54: f32;
let _55: f32;
let _56: [u64; 1];
let _57: u8;
let _58: &'static f64;
let _59: *const i32;
let _60: (Adt45,);
let _61: &'static ((((char,),), u8), *const (((char,),), u8), (i16,));
let _62: isize;
let _63: Adt24;
let _64: *mut i128;
let _65: isize;
let _66: [bool; 5];
let _67: i64;
let _68: isize;
let _69: *mut &'static ((char,),);
let _70: [i64; 7];
let _71: [i64; 7];
let _72: ();
let _73: ();
{
_1.0 = !_4;
_1 = (_2,);
_1.0 = 5997626598697536829_i64 as isize;
_1.0 = -_4;
_3 = _4 + _4;
_3 = _2 << _1.0;
_2 = _3;
_1 = (_2,);
_2 = _4;
_1 = (_2,);
_3 = -_1.0;
Goto(bb1)
}
bb1 = {
_4 = _3;
_4 = -_3;
_1 = (_4,);
_1 = (_2,);
_1.0 = -_2;
_1 = (_2,);
_3 = _2;
_4 = !_3;
_3 = _1.0 >> _2;
_3 = _4;
_4 = _3;
_2 = -_3;
_2 = _3 + _1.0;
_2 = _1.0;
_8 = '\u{b1a99}';
_3 = 32630179906997130469666385957029334351_i128 as isize;
_1.0 = _4;
_1 = (_2,);
_8 = '\u{5d2b3}';
Goto(bb2)
}
bb2 = {
_8 = '\u{b9784}';
_3 = _1.0;
_1 = (_3,);
_8 = '\u{d9781}';
_2 = _1.0 >> _4;
_4 = -_3;
_4 = -_2;
_4 = _3 + _1.0;
_1 = (_4,);
_1.0 = _3;
_9 = -_4;
_11.fld0 = !true;
_2 = _4 - _9;
_2 = !_9;
_11.fld3 = 318539425884222343_u64;
_11.fld1 = -111268271118435156198862046718890834440_i128;
_9 = _2 >> _2;
_11 = Adt24 { fld0: true,fld1: 48986513861490712177378119164863664429_i128,fld2: 169_u8,fld3: 7661880221014143101_u64 };
_11 = Adt24 { fld0: false,fld1: 149549278545261367824802723041352307010_i128,fld2: 252_u8,fld3: 1859960772472690606_u64 };
_1.0 = _9;
_11 = Adt24 { fld0: true,fld1: 115231752828117715679476524901210698870_i128,fld2: 237_u8,fld3: 14508897029972980879_u64 };
match _11.fld1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
115231752828117715679476524901210698870 => bb8,
_ => bb7
}
}
bb3 = {
_4 = _3;
_4 = -_3;
_1 = (_4,);
_1 = (_2,);
_1.0 = -_2;
_1 = (_2,);
_3 = _2;
_4 = !_3;
_3 = _1.0 >> _2;
_3 = _4;
_4 = _3;
_2 = -_3;
_2 = _3 + _1.0;
_2 = _1.0;
_8 = '\u{b1a99}';
_3 = 32630179906997130469666385957029334351_i128 as isize;
_1.0 = _4;
_1 = (_2,);
_8 = '\u{5d2b3}';
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
_11.fld0 = false;
_11 = Adt24 { fld0: true,fld1: 169058814953782664005881660346645710255_i128,fld2: 165_u8,fld3: 13191278810684948369_u64 };
_3 = _1.0;
_1 = (_2,);
_11 = Adt24 { fld0: false,fld1: 136166734644101639197387668212345552119_i128,fld2: 139_u8,fld3: 13732730697680420456_u64 };
_12 = _8;
_9 = _3;
_11 = Adt24 { fld0: false,fld1: 140644511769997560281220905105883706145_i128,fld2: 91_u8,fld3: 5400533869899298246_u64 };
_13 = _3;
_15 = [_11.fld3,_11.fld3];
_9 = _4 + _2;
_4 = _11.fld3 as isize;
_5 = &_14;
_16 = [50_i8,(-24_i8),56_i8,14_i8,(-69_i8)];
_13 = !_9;
_16 = [81_i8,(-49_i8),23_i8,(-38_i8),53_i8];
_11 = Adt24 { fld0: true,fld1: (-133474904569035794536329357578239554751_i128),fld2: 65_u8,fld3: 14943910582257387045_u64 };
_13 = 3028560723_u32 as isize;
_12 = _8;
_14 = 5568829878593316285_i64 as f64;
_12 = _8;
_8 = _12;
_4 = _2 >> _3;
_11 = Adt24 { fld0: false,fld1: (-101211015921059357179161863562348763991_i128),fld2: 26_u8,fld3: 8146338890791977777_u64 };
_2 = _3 & _1.0;
Goto(bb9)
}
bb9 = {
_8 = _12;
_2 = 54538_u16 as isize;
_16 = [17_i8,94_i8,(-72_i8),43_i8,102_i8];
_8 = _12;
_11.fld0 = !false;
_5 = &_14;
_16 = [(-15_i8),(-126_i8),(-40_i8),(-84_i8),(-123_i8)];
_9 = _3;
_16 = [(-126_i8),(-102_i8),78_i8,85_i8,(-23_i8)];
_15 = [_11.fld3,_11.fld3];
_8 = _12;
_11.fld1 = (-118334014290193456326263959377862119839_i128) + 67781193181260928862521520971176752811_i128;
match _11.fld2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb11,
26 => bb13,
_ => bb12
}
}
bb10 = {
_4 = _3;
_4 = -_3;
_1 = (_4,);
_1 = (_2,);
_1.0 = -_2;
_1 = (_2,);
_3 = _2;
_4 = !_3;
_3 = _1.0 >> _2;
_3 = _4;
_4 = _3;
_2 = -_3;
_2 = _3 + _1.0;
_2 = _1.0;
_8 = '\u{b1a99}';
_3 = 32630179906997130469666385957029334351_i128 as isize;
_1.0 = _4;
_1 = (_2,);
_8 = '\u{5d2b3}';
Goto(bb2)
}
bb11 = {
_4 = _3;
_4 = -_3;
_1 = (_4,);
_1 = (_2,);
_1.0 = -_2;
_1 = (_2,);
_3 = _2;
_4 = !_3;
_3 = _1.0 >> _2;
_3 = _4;
_4 = _3;
_2 = -_3;
_2 = _3 + _1.0;
_2 = _1.0;
_8 = '\u{b1a99}';
_3 = 32630179906997130469666385957029334351_i128 as isize;
_1.0 = _4;
_1 = (_2,);
_8 = '\u{5d2b3}';
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_13 = 24009_i16 as isize;
_6 = Adt34::Variant1 { fld0: _11.fld0,fld1: _11.fld2,fld2: _14,fld3: (-90_i8) };
_11.fld3 = 8333484908906071660_u64;
_9 = -_3;
_14 = -Field::<f64>(Variant(_6, 1), 2);
_18 = _4;
_11.fld1 = 169098710022648461209439625230056027776_i128;
_9 = 0_usize as isize;
_2 = _3 - _4;
_11.fld3 = 14395479754068219369_u64;
place!(Field::<i8>(Variant(_6, 1), 3)) = 29_i8;
_20 = _18 > _3;
_18 = _4 ^ _2;
_12 = _8;
place!(Field::<u8>(Variant(_6, 1), 1)) = 2648786748398340120_i64 as u8;
SetDiscriminant(_6, 1);
_5 = &place!(Field::<f64>(Variant(_6, 1), 2));
_5 = &place!(Field::<f64>(Variant(_6, 1), 2));
place!(Field::<f64>(Variant(_6, 1), 2)) = -_14;
_11.fld2 = _8 as u8;
_1 = (_18,);
place!(Field::<i8>(Variant(_6, 1), 3)) = -22_i8;
_20 = !_11.fld0;
_12 = _8;
place!(Field::<f64>(Variant(_6, 1), 2)) = -_14;
_1 = (_4,);
_3 = !_18;
_3 = !_4;
_16 = [Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3)];
match _11.fld1 {
0 => bb6,
1 => bb4,
2 => bb3,
169098710022648461209439625230056027776 => bb15,
_ => bb14
}
}
bb14 = {
_8 = _12;
_2 = 54538_u16 as isize;
_16 = [17_i8,94_i8,(-72_i8),43_i8,102_i8];
_8 = _12;
_11.fld0 = !false;
_5 = &_14;
_16 = [(-15_i8),(-126_i8),(-40_i8),(-84_i8),(-123_i8)];
_9 = _3;
_16 = [(-126_i8),(-102_i8),78_i8,85_i8,(-23_i8)];
_15 = [_11.fld3,_11.fld3];
_8 = _12;
_11.fld1 = (-118334014290193456326263959377862119839_i128) + 67781193181260928862521520971176752811_i128;
match _11.fld2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb11,
26 => bb13,
_ => bb12
}
}
bb15 = {
_1.0 = _2 - _18;
_11.fld3 = 3890680089282433471_i64 as u64;
_16 = [Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3)];
_5 = &_14;
_1.0 = _11.fld2 as isize;
_20 = _4 >= _2;
place!(Field::<u8>(Variant(_6, 1), 1)) = _11.fld2;
_21.0 = _8;
_11 = Adt24 { fld0: _20,fld1: 40934408200577706061895555093504580456_i128,fld2: Field::<u8>(Variant(_6, 1), 1),fld3: 4927743462519921626_u64 };
_22 = [_11.fld3,_11.fld3,_11.fld3];
_5 = &(*_5);
_23 = _3 + _3;
match _11.fld1 {
0 => bb5,
1 => bb14,
2 => bb11,
40934408200577706061895555093504580456 => bb16,
_ => bb13
}
}
bb16 = {
_9 = (-5498459333929222854_i64) as isize;
_1.0 = !_3;
_22 = [_11.fld3,_11.fld3,_11.fld3];
_18 = !_2;
place!(Field::<f64>(Variant(_6, 1), 2)) = 3811183359336536322_i64 as f64;
_11.fld2 = Field::<u8>(Variant(_6, 1), 1);
_11.fld3 = 6674769131491564715_u64 >> _4;
match _11.fld1 {
0 => bb9,
1 => bb15,
2 => bb12,
3 => bb4,
4 => bb5,
5 => bb17,
40934408200577706061895555093504580456 => bb19,
_ => bb18
}
}
bb17 = {
Return()
}
bb18 = {
_11.fld0 = false;
_11 = Adt24 { fld0: true,fld1: 169058814953782664005881660346645710255_i128,fld2: 165_u8,fld3: 13191278810684948369_u64 };
_3 = _1.0;
_1 = (_2,);
_11 = Adt24 { fld0: false,fld1: 136166734644101639197387668212345552119_i128,fld2: 139_u8,fld3: 13732730697680420456_u64 };
_12 = _8;
_9 = _3;
_11 = Adt24 { fld0: false,fld1: 140644511769997560281220905105883706145_i128,fld2: 91_u8,fld3: 5400533869899298246_u64 };
_13 = _3;
_15 = [_11.fld3,_11.fld3];
_9 = _4 + _2;
_4 = _11.fld3 as isize;
_5 = &_14;
_16 = [50_i8,(-24_i8),56_i8,14_i8,(-69_i8)];
_13 = !_9;
_16 = [81_i8,(-49_i8),23_i8,(-38_i8),53_i8];
_11 = Adt24 { fld0: true,fld1: (-133474904569035794536329357578239554751_i128),fld2: 65_u8,fld3: 14943910582257387045_u64 };
_13 = 3028560723_u32 as isize;
_12 = _8;
_14 = 5568829878593316285_i64 as f64;
_12 = _8;
_8 = _12;
_4 = _2 >> _3;
_11 = Adt24 { fld0: false,fld1: (-101211015921059357179161863562348763991_i128),fld2: 26_u8,fld3: 8146338890791977777_u64 };
_2 = _3 & _1.0;
Goto(bb9)
}
bb19 = {
_2 = 22030_i16 as isize;
_27 = _11.fld3 * _11.fld3;
_6 = Adt34::Variant1 { fld0: _11.fld0,fld1: _11.fld2,fld2: (*_5),fld3: 2_i8 };
_11 = Adt24 { fld0: _20,fld1: 116068787009399370063276646573554463781_i128,fld2: Field::<u8>(Variant(_6, 1), 1),fld3: _27 };
_8 = _12;
_20 = !_11.fld0;
_21 = (_12,);
_21 = (_8,);
_28 = [(-7771468100345313629_i64),(-6419295437146505881_i64),(-4818666059160475717_i64),(-6075402902450171079_i64),(-1863357492445322290_i64),(-5007596730533862396_i64),1643740065535007244_i64];
_24 = _11.fld0 | Field::<bool>(Variant(_6, 1), 0);
place!(Field::<f64>(Variant(_6, 1), 2)) = 747074568602655226_i64 as f64;
_25 = [_11.fld3];
_5 = &(*_5);
_13 = -_23;
_20 = _3 != _13;
match _11.fld1 {
116068787009399370063276646573554463781 => bb21,
_ => bb20
}
}
bb20 = {
Return()
}
bb21 = {
_18 = _3 ^ _4;
_23 = -_4;
_30 = _12;
Call(_2 = fn13(_11.fld1, _25, _1.0, _1.0), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_28 = [1577489671641298819_i64,(-4617527677843906534_i64),1088053099060282060_i64,5054372781531727978_i64,103453661303639440_i64,459114360750733876_i64,(-4875749163604604931_i64)];
_5 = &place!(Field::<f64>(Variant(_6, 1), 2));
_5 = &place!(Field::<f64>(Variant(_6, 1), 2));
_3 = _18 | _18;
_13 = _23 * _4;
place!(Field::<i8>(Variant(_6, 1), 3)) = 80_i8;
_32 = _24 as u16;
place!(Field::<f64>(Variant(_6, 1), 2)) = (-15284_i16) as f64;
_25 = [_27];
_14 = Field::<f64>(Variant(_6, 1), 2) + Field::<f64>(Variant(_6, 1), 2);
_28 = [7477387589359330533_i64,(-7048821382850287563_i64),(-8155755273390271620_i64),1196058010324811481_i64,(-1970769801022615079_i64),(-2911174303826869973_i64),(-3202702430229614895_i64)];
_33 = _23 >> _23;
_1.0 = _13;
_34 = [_24,Field::<bool>(Variant(_6, 1), 0),_24];
_14 = Field::<f64>(Variant(_6, 1), 2);
_11.fld3 = Field::<i8>(Variant(_6, 1), 3) as u64;
Goto(bb23)
}
bb23 = {
_23 = (-23214_i16) as isize;
_22 = [_27,_27,_27];
_5 = &_14;
_23 = !_1.0;
_2 = -_3;
_15 = [_27,_27];
_29 = (*_5) as f32;
_7 = &_26;
_7 = &_26;
_25 = [_27];
_3 = !_33;
SetDiscriminant(_6, 0);
_16 = [(-58_i8),28_i8,(-112_i8),(-10_i8),75_i8];
place!(Field::<(isize,)>(Variant(_6, 0), 5)) = _1;
_7 = &_26;
place!(Field::<f32>(Variant(_6, 0), 7)) = -_29;
_21.0 = _30;
place!(Field::<[char; 5]>(Variant(_6, 0), 1)) = [_30,_12,_30,_8,_12];
_21.0 = _8;
_11.fld2 = 134_u8;
place!(Field::<((char,),)>(Variant(_6, 0), 2)).0 = _21;
place!(Field::<((char,),)>(Variant(_6, 0), 2)) = (_21,);
_36 = Field::<f32>(Variant(_6, 0), 7) * Field::<f32>(Variant(_6, 0), 7);
place!(Field::<f64>(Variant(_6, 0), 3)) = _11.fld2 as f64;
Goto(bb24)
}
bb24 = {
place!(Field::<bool>(Variant(_6, 0), 0)) = _24;
_35 = _24;
_11.fld0 = Field::<bool>(Variant(_6, 0), 0);
_5 = &(*_5);
_18 = -_3;
_6 = Adt34::Variant1 { fld0: _35,fld1: _11.fld2,fld2: (*_5),fld3: 82_i8 };
_15 = [_27,_27];
_11.fld0 = _35 & _24;
Goto(bb25)
}
bb25 = {
_38 = (_22, _13);
Goto(bb26)
}
bb26 = {
_36 = _29;
place!(Field::<f64>(Variant(_6, 1), 2)) = (*_5) + (*_5);
match _11.fld1 {
0 => bb14,
1 => bb12,
2 => bb24,
3 => bb27,
4 => bb28,
5 => bb29,
6 => bb30,
116068787009399370063276646573554463781 => bb32,
_ => bb31
}
}
bb27 = {
_11.fld0 = false;
_11 = Adt24 { fld0: true,fld1: 169058814953782664005881660346645710255_i128,fld2: 165_u8,fld3: 13191278810684948369_u64 };
_3 = _1.0;
_1 = (_2,);
_11 = Adt24 { fld0: false,fld1: 136166734644101639197387668212345552119_i128,fld2: 139_u8,fld3: 13732730697680420456_u64 };
_12 = _8;
_9 = _3;
_11 = Adt24 { fld0: false,fld1: 140644511769997560281220905105883706145_i128,fld2: 91_u8,fld3: 5400533869899298246_u64 };
_13 = _3;
_15 = [_11.fld3,_11.fld3];
_9 = _4 + _2;
_4 = _11.fld3 as isize;
_5 = &_14;
_16 = [50_i8,(-24_i8),56_i8,14_i8,(-69_i8)];
_13 = !_9;
_16 = [81_i8,(-49_i8),23_i8,(-38_i8),53_i8];
_11 = Adt24 { fld0: true,fld1: (-133474904569035794536329357578239554751_i128),fld2: 65_u8,fld3: 14943910582257387045_u64 };
_13 = 3028560723_u32 as isize;
_12 = _8;
_14 = 5568829878593316285_i64 as f64;
_12 = _8;
_8 = _12;
_4 = _2 >> _3;
_11 = Adt24 { fld0: false,fld1: (-101211015921059357179161863562348763991_i128),fld2: 26_u8,fld3: 8146338890791977777_u64 };
_2 = _3 & _1.0;
Goto(bb9)
}
bb28 = {
place!(Field::<bool>(Variant(_6, 0), 0)) = _24;
_35 = _24;
_11.fld0 = Field::<bool>(Variant(_6, 0), 0);
_5 = &(*_5);
_18 = -_3;
_6 = Adt34::Variant1 { fld0: _35,fld1: _11.fld2,fld2: (*_5),fld3: 82_i8 };
_15 = [_27,_27];
_11.fld0 = _35 & _24;
Goto(bb25)
}
bb29 = {
_23 = (-23214_i16) as isize;
_22 = [_27,_27,_27];
_5 = &_14;
_23 = !_1.0;
_2 = -_3;
_15 = [_27,_27];
_29 = (*_5) as f32;
_7 = &_26;
_7 = &_26;
_25 = [_27];
_3 = !_33;
SetDiscriminant(_6, 0);
_16 = [(-58_i8),28_i8,(-112_i8),(-10_i8),75_i8];
place!(Field::<(isize,)>(Variant(_6, 0), 5)) = _1;
_7 = &_26;
place!(Field::<f32>(Variant(_6, 0), 7)) = -_29;
_21.0 = _30;
place!(Field::<[char; 5]>(Variant(_6, 0), 1)) = [_30,_12,_30,_8,_12];
_21.0 = _8;
_11.fld2 = 134_u8;
place!(Field::<((char,),)>(Variant(_6, 0), 2)).0 = _21;
place!(Field::<((char,),)>(Variant(_6, 0), 2)) = (_21,);
_36 = Field::<f32>(Variant(_6, 0), 7) * Field::<f32>(Variant(_6, 0), 7);
place!(Field::<f64>(Variant(_6, 0), 3)) = _11.fld2 as f64;
Goto(bb24)
}
bb30 = {
_9 = (-5498459333929222854_i64) as isize;
_1.0 = !_3;
_22 = [_11.fld3,_11.fld3,_11.fld3];
_18 = !_2;
place!(Field::<f64>(Variant(_6, 1), 2)) = 3811183359336536322_i64 as f64;
_11.fld2 = Field::<u8>(Variant(_6, 1), 1);
_11.fld3 = 6674769131491564715_u64 >> _4;
match _11.fld1 {
0 => bb9,
1 => bb15,
2 => bb12,
3 => bb4,
4 => bb5,
5 => bb17,
40934408200577706061895555093504580456 => bb19,
_ => bb18
}
}
bb31 = {
_18 = _3 ^ _4;
_23 = -_4;
_30 = _12;
Call(_2 = fn13(_11.fld1, _25, _1.0, _1.0), ReturnTo(bb22), UnwindUnreachable())
}
bb32 = {
_30 = _8;
_38.1 = !_3;
_9 = _23 << _18;
_41.0 = _30;
place!(Field::<i8>(Variant(_6, 1), 3)) = -(-2_i8);
_35 = _3 >= _4;
_25 = [_27];
_2 = -_33;
_3 = _38.1;
_2 = _23;
_29 = _36 + _36;
_11.fld3 = 181049434225398975660636510841081858079_u128 as u64;
_11.fld0 = _20;
_11.fld1 = (-26444971738842190871314702927106097250_i128) - 67197628629054711664559634911135516740_i128;
place!(Field::<u8>(Variant(_6, 1), 1)) = _11.fld2;
SetDiscriminant(_6, 2);
_34 = [_11.fld0,_11.fld0,_35];
place!(Field::<bool>(Variant(_6, 2), 0)) = !_20;
_25 = [_27];
_11.fld0 = _9 != _9;
_9 = _1.0;
match _11.fld2 {
0 => bb18,
1 => bb27,
134 => bb33,
_ => bb15
}
}
bb33 = {
_41 = _21;
place!(Field::<i32>(Variant(_6, 2), 2)) = !(-454586550_i32);
_11.fld0 = _35 <= _35;
_18 = 13924_i16 as isize;
_2 = _33 * _3;
_33 = _9;
_13 = _1.0;
_20 = _38.1 < _4;
_11.fld3 = _27 << _3;
_37 = 15389_i16 * (-20893_i16);
_3 = !_13;
_44 = [_11.fld2,_11.fld2,_11.fld2,_11.fld2,_11.fld2,_11.fld2];
_4 = _1.0;
_28 = [(-2892698258696259593_i64),4969687335763601889_i64,8178986010126970646_i64,7548078581726996731_i64,(-766155684305897855_i64),2146515435610452321_i64,(-6691270238465896886_i64)];
_33 = Field::<i32>(Variant(_6, 2), 2) as isize;
_36 = _29 + _29;
_42 = (-106_i8) << _38.1;
_16 = [_42,_42,_42,_42,_42];
_38 = (_22, _23);
_3 = -_38.1;
_3 = _9;
_3 = !_13;
_27 = _11.fld3 << _32;
match _11.fld2 {
0 => bb15,
1 => bb34,
134 => bb36,
_ => bb35
}
}
bb34 = {
_13 = 24009_i16 as isize;
_6 = Adt34::Variant1 { fld0: _11.fld0,fld1: _11.fld2,fld2: _14,fld3: (-90_i8) };
_11.fld3 = 8333484908906071660_u64;
_9 = -_3;
_14 = -Field::<f64>(Variant(_6, 1), 2);
_18 = _4;
_11.fld1 = 169098710022648461209439625230056027776_i128;
_9 = 0_usize as isize;
_2 = _3 - _4;
_11.fld3 = 14395479754068219369_u64;
place!(Field::<i8>(Variant(_6, 1), 3)) = 29_i8;
_20 = _18 > _3;
_18 = _4 ^ _2;
_12 = _8;
place!(Field::<u8>(Variant(_6, 1), 1)) = 2648786748398340120_i64 as u8;
SetDiscriminant(_6, 1);
_5 = &place!(Field::<f64>(Variant(_6, 1), 2));
_5 = &place!(Field::<f64>(Variant(_6, 1), 2));
place!(Field::<f64>(Variant(_6, 1), 2)) = -_14;
_11.fld2 = _8 as u8;
_1 = (_18,);
place!(Field::<i8>(Variant(_6, 1), 3)) = -22_i8;
_20 = !_11.fld0;
_12 = _8;
place!(Field::<f64>(Variant(_6, 1), 2)) = -_14;
_1 = (_4,);
_3 = !_18;
_3 = !_4;
_16 = [Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3),Field::<i8>(Variant(_6, 1), 3)];
match _11.fld1 {
0 => bb6,
1 => bb4,
2 => bb3,
169098710022648461209439625230056027776 => bb15,
_ => bb14
}
}
bb35 = {
Return()
}
bb36 = {
_24 = Field::<bool>(Variant(_6, 2), 0);
_16 = [_42,_42,_42,_42,_42];
place!(Field::<[u64; 3]>(Variant(_6, 2), 1)) = [_27,_27,_11.fld3];
_2 = -_3;
_21 = _41;
_38.1 = Field::<i32>(Variant(_6, 2), 2) as isize;
_11.fld0 = !_24;
_3 = !_23;
place!(Field::<[u8; 6]>(Variant(_6, 2), 3)) = [_11.fld2,_11.fld2,_11.fld2,_11.fld2,_11.fld2,_11.fld2];
_9 = 105805046058696326545872015415918679356_u128 as isize;
_21.0 = _8;
_43.0 = _37 >> _23;
_41.0 = _8;
_18 = !_23;
_45 = (-5711267755754769968_i64) >> _1.0;
_11 = Adt24 { fld0: Field::<bool>(Variant(_6, 2), 0),fld1: 75560813814127768886872684877116211898_i128,fld2: 167_u8,fld3: _27 };
_6 = Adt34::Variant2 { fld0: _20,fld1: _22,fld2: 1041672130_i32,fld3: _44,fld4: _11.fld2 };
_24 = _32 < _32;
_41.0 = _30;
_11.fld2 = _42 as u8;
Goto(bb37)
}
bb37 = {
_41 = (_21.0,);
_11.fld0 = _35;
_25 = [_27];
place!(Field::<u8>(Variant(_6, 2), 4)) = _11.fld2;
Goto(bb38)
}
bb38 = {
_48 = -_29;
_42 = _32 as i8;
_30 = _41.0;
_43 = (_37,);
_41 = (_21.0,);
_29 = _36;
_28 = [_45,_45,_45,_45,_45,_45,_45];
_25 = [_27];
_14 = (-1905290131_i32) as f64;
_43 = (_37,);
_25 = [_11.fld3];
_3 = _45 as isize;
place!(Field::<i32>(Variant(_6, 2), 2)) = !402280576_i32;
_28 = [_45,_45,_45,_45,_45,_45,_45];
_40 = _11.fld1 as u128;
_46 = _8;
_11.fld2 = Field::<u8>(Variant(_6, 2), 4);
SetDiscriminant(_6, 2);
_40 = 152513764891321855444222168813446903507_u128;
_33 = -_18;
_36 = _48 * _29;
_36 = _29;
_11.fld0 = _24;
place!(Field::<bool>(Variant(_6, 2), 0)) = _24 > _11.fld0;
Goto(bb39)
}
bb39 = {
_3 = 758755737_u32 as isize;
_24 = _20;
_5 = &_14;
_11.fld3 = (*_5) as u64;
_2 = _13;
_39 = _20 > _35;
_22 = _38.0;
place!(Field::<bool>(Variant(_6, 2), 0)) = _20 & _11.fld0;
_34 = [_20,_20,Field::<bool>(Variant(_6, 2), 0)];
_1 = (_2,);
_43 = (_37,);
_6 = Adt34::Variant2 { fld0: _20,fld1: _22,fld2: 1389554135_i32,fld3: _44,fld4: _11.fld2 };
_21 = (_30,);
_27 = !_11.fld3;
_18 = -_33;
place!(Field::<i32>(Variant(_6, 2), 2)) = (-1067059030_i32);
_3 = -_1.0;
_43.0 = _37;
_11.fld1 = -(-149255975943410176157888343767052843391_i128);
_32 = !15529_u16;
match Field::<i32>(Variant(_6, 2), 2) {
340282366920938463463374607430701152426 => bb41,
_ => bb40
}
}
bb40 = {
_41 = (_21.0,);
_11.fld0 = _35;
_25 = [_27];
place!(Field::<u8>(Variant(_6, 2), 4)) = _11.fld2;
Goto(bb38)
}
bb41 = {
_52 = _27 as f64;
_32 = 15043_u16 - 5990_u16;
SetDiscriminant(_6, 0);
place!(Field::<((char,),)>(Variant(_6, 0), 2)) = (_21,);
_34 = [_39,_39,_11.fld0];
_50.0 = _21;
_48 = -_36;
_51 = _24;
_43 = (_37,);
_6 = Adt34::Variant1 { fld0: _20,fld1: _11.fld2,fld2: _14,fld3: _42 };
_18 = -_3;
place!(Field::<bool>(Variant(_6, 1), 0)) = _23 < _4;
_27 = _14 as u64;
_46 = _30;
Goto(bb42)
}
bb42 = {
place!(Field::<bool>(Variant(_6, 1), 0)) = _24 ^ _35;
_39 = !Field::<bool>(Variant(_6, 1), 0);
_4 = _18;
_11.fld1 = _8 as i128;
_30 = _8;
_7 = &(*_7);
Goto(bb43)
}
bb43 = {
_55 = _48;
_21.0 = _30;
_21.0 = _46;
_4 = _50.0.0 as isize;
_42 = _36 as i8;
_25 = [_11.fld3];
_32 = 1_usize as u16;
_37 = !_43.0;
_20 = !_24;
_28 = [_45,_45,_45,_45,_45,_45,_45];
_14 = _52;
_7 = &_26;
_28 = [_45,_45,_45,_45,_45,_45,_45];
_50.0.0 = _46;
_5 = &_14;
_54 = _55 + _29;
_56 = [_11.fld3];
place!(Field::<bool>(Variant(_6, 1), 0)) = !_39;
place!(Field::<f64>(Variant(_6, 1), 2)) = -_52;
_45 = 2511506758127675767_i64 ^ 5802081125469040116_i64;
_51 = _11.fld0;
_38.1 = _2;
_20 = _39;
_45 = 2318128821119848977_i64;
Goto(bb44)
}
bb44 = {
_45 = -(-309007354993401875_i64);
SetDiscriminant(_6, 0);
_27 = _11.fld3;
_43.0 = _51 as i16;
place!(Field::<((char,),)>(Variant(_6, 0), 2)) = (_21,);
place!(Field::<f32>(Variant(_6, 0), 7)) = _36;
Call(_33 = fn14(Move(_5), _22, _23, _38.0, _43, _39, Move(_11), _3), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
_4 = _38.1 ^ _13;
match _40 {
0 => bb17,
1 => bb46,
152513764891321855444222168813446903507 => bb48,
_ => bb47
}
}
bb46 = {
_30 = _8;
_38.1 = !_3;
_9 = _23 << _18;
_41.0 = _30;
place!(Field::<i8>(Variant(_6, 1), 3)) = -(-2_i8);
_35 = _3 >= _4;
_25 = [_27];
_2 = -_33;
_3 = _38.1;
_2 = _23;
_29 = _36 + _36;
_11.fld3 = 181049434225398975660636510841081858079_u128 as u64;
_11.fld0 = _20;
_11.fld1 = (-26444971738842190871314702927106097250_i128) - 67197628629054711664559634911135516740_i128;
place!(Field::<u8>(Variant(_6, 1), 1)) = _11.fld2;
SetDiscriminant(_6, 2);
_34 = [_11.fld0,_11.fld0,_35];
place!(Field::<bool>(Variant(_6, 2), 0)) = !_20;
_25 = [_27];
_11.fld0 = _9 != _9;
_9 = _1.0;
match _11.fld2 {
0 => bb18,
1 => bb27,
134 => bb33,
_ => bb15
}
}
bb47 = {
_18 = _3 ^ _4;
_23 = -_4;
_30 = _12;
Call(_2 = fn13(_11.fld1, _25, _1.0, _1.0), ReturnTo(bb22), UnwindUnreachable())
}
bb48 = {
_23 = _32 as isize;
_11.fld2 = 145_u8 | 251_u8;
_50 = (Field::<((char,),)>(Variant(_6, 0), 2).0,);
Goto(bb49)
}
bb49 = {
_40 = 324380522133951003915494413869645156335_u128 + 3504038834021870936716812702920442927_u128;
_43.0 = _37 << _3;
_13 = !_2;
_24 = _39 >= _39;
_42 = (-1185267133_i32) as i8;
_38.1 = _11.fld2 as isize;
place!(Field::<[char; 5]>(Variant(_6, 0), 1)) = [_8,_30,_21.0,_30,_41.0];
_11.fld3 = _27 * _27;
_58 = &place!(Field::<f64>(Variant(_6, 0), 3));
_16 = [_42,_42,_42,_42,_42];
_55 = _40 as f32;
_11 = Adt24 { fld0: _24,fld1: 69004226037512135259312951299442032382_i128,fld2: 192_u8,fld3: _27 };
_26 = Adt44::Variant0 { fld0: _11.fld2,fld1: _50.0.0,fld2: _3,fld3: Field::<[char; 5]>(Variant(_6, 0), 1),fld4: _14,fld5: _32 };
place!(Field::<(isize,)>(Variant(_6, 0), 5)) = (_3,);
Goto(bb50)
}
bb50 = {
_21 = (_50.0.0,);
_52 = -Field::<f64>(Variant(_26, 0), 4);
_41 = (_46,);
place!(Field::<((char,),)>(Variant(_6, 0), 2)) = _50;
_30 = _41.0;
_11.fld2 = !Field::<u8>(Variant(_26, 0), 0);
place!(Field::<[u8; 6]>(Variant(_6, 0), 4)) = _44;
_2 = _4 * Field::<(isize,)>(Variant(_6, 0), 5).0;
_11.fld3 = _30 as u64;
place!(Field::<isize>(Variant(_26, 0), 2)) = _2;
_24 = _20;
_67 = _45;
_54 = Field::<f32>(Variant(_6, 0), 7) + _36;
_20 = _3 != Field::<(isize,)>(Variant(_6, 0), 5).0;
_63.fld0 = _11.fld0 | _11.fld0;
place!(Field::<char>(Variant(_26, 0), 1)) = _8;
_63.fld3 = _27 * _27;
_32 = Field::<u16>(Variant(_26, 0), 5) & Field::<u16>(Variant(_26, 0), 5);
_6 = Adt34::Variant1 { fld0: _35,fld1: _11.fld2,fld2: _52,fld3: _42 };
Goto(bb51)
}
bb51 = {
_11 = Adt24 { fld0: _20,fld1: (-120120534945958311378729096460048880334_i128),fld2: Field::<u8>(Variant(_26, 0), 0),fld3: _63.fld3 };
_11.fld2 = _45 as u8;
_1 = (_33,);
_64 = core::ptr::addr_of_mut!(_11.fld1);
_2 = _13 | _1.0;
_44 = [Field::<u8>(Variant(_6, 1), 1),Field::<u8>(Variant(_26, 0), 0),Field::<u8>(Variant(_26, 0), 0),Field::<u8>(Variant(_6, 1), 1),Field::<u8>(Variant(_6, 1), 1),Field::<u8>(Variant(_26, 0), 0)];
(*_64) = 156330930599452571979559386144470276281_i128;
_46 = _30;
_11.fld0 = !_24;
SetDiscriminant(_26, 0);
_34 = [_39,_35,_39];
RET = core::ptr::addr_of_mut!(_66);
_33 = -_1.0;
_68 = !_33;
_57 = !Field::<u8>(Variant(_6, 1), 1);
RET = core::ptr::addr_of_mut!(_66);
_50.0 = (_8,);
SetDiscriminant(_6, 0);
RET = core::ptr::addr_of_mut!((*RET));
_56 = [_11.fld3];
_43.0 = !_37;
_42 = (-2_i8) ^ (-76_i8);
place!(Field::<bool>(Variant(_6, 0), 0)) = _35 | _11.fld0;
_1 = (_18,);
_22 = _38.0;
Goto(bb52)
}
bb52 = {
Call(_72 = dump_var(12_usize, 46_usize, Move(_46), 15_usize, Move(_15), 43_usize, Move(_43), 30_usize, Move(_30)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_72 = dump_var(12_usize, 13_usize, Move(_13), 22_usize, Move(_22), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_72 = dump_var(12_usize, 39_usize, Move(_39), 38_usize, Move(_38), 24_usize, Move(_24), 35_usize, Move(_35)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_72 = dump_var(12_usize, 42_usize, Move(_42), 21_usize, Move(_21), 27_usize, Move(_27), 18_usize, Move(_18)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_72 = dump_var(12_usize, 32_usize, Move(_32), 56_usize, Move(_56), 33_usize, Move(_33), 16_usize, Move(_16)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i128,mut _2: [u64; 1],mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: ((((char,),), u8), *const (((char,),), u8), (i16,));
let _6: *mut u64;
let _7: isize;
let _8: (i16,);
let _9: bool;
let _10: ();
let _11: ();
{
_2 = [1484564158335845092_u64];
_2 = [15408075352022398149_u64];
_4 = -_3;
_5.1 = core::ptr::addr_of!(_5.0);
_5.0.0.0 = ('\u{b584b}',);
_5.2.0 = !21454_i16;
_5.0.1 = true as u8;
Goto(bb1)
}
bb1 = {
_3 = !_4;
RET = -_4;
_8 = (_5.2.0,);
_5.0.0.0 = ('\u{10f91b}',);
_8 = (_5.2.0,);
_5.0.0.0.0 = '\u{1e0dd}';
_5.0.1 = !62_u8;
_8.0 = _5.2.0 ^ _5.2.0;
_5.0.1 = !104_u8;
_5.0.0.0.0 = '\u{659cd}';
_5.0.1 = 251_u8;
_5.0.0.0 = ('\u{f60d9}',);
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(13_usize, 3_usize, Move(_3), 2_usize, Move(_2), 11_usize, _11, 11_usize, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static f64,mut _2: [u64; 3],mut _3: isize,mut _4: [u64; 3],mut _5: (i16,),mut _6: bool,mut _7: Adt24,mut _8: isize) -> isize {
mir! {
type RET = isize;
let _9: char;
let _10: [u64; 1];
let _11: ();
let _12: ();
{
RET = 1231791612_u32 as isize;
_4 = [_7.fld3,_7.fld3,_7.fld3];
_7.fld1 = (-82811688655778262929914713597748186584_i128);
_7 = Adt24 { fld0: _6,fld1: 57624069538868360170063831014715714335_i128,fld2: 158_u8,fld3: 5435332079694906892_u64 };
_7.fld3 = 7566200512283104412_u64 | 7188590769120705067_u64;
_7.fld1 = !245413979629888972923791181633258847_i128;
_4 = [_7.fld3,_7.fld3,_7.fld3];
_2 = _4;
_2 = [_7.fld3,_7.fld3,_7.fld3];
RET = !_8;
RET = _8;
_5.0 = 24106_i16 << _3;
_3 = !_8;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(14_usize, 6_usize, Move(_6), 2_usize, Move(_2), 4_usize, Move(_4), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: i32,mut _3: isize) -> char {
mir! {
type RET = char;
let _4: [u8; 6];
let _5: &'static Adt44;
let _6: u32;
let _7: i16;
let _8: u32;
let _9: i8;
let _10: u32;
let _11: bool;
let _12: f64;
let _13: isize;
let _14: [u64; 5];
let _15: *mut Adt57;
let _16: ([bool; 5],);
let _17: (char,);
let _18: *mut &'static &'static *mut u128;
let _19: &'static ((char,),);
let _20: bool;
let _21: *mut i128;
let _22: (*const Adt45, *mut [bool; 5], Adt48, usize);
let _23: isize;
let _24: ((((char,),), u8), *const (((char,),), u8), (i16,));
let _25: f32;
let _26: [u64; 3];
let _27: isize;
let _28: isize;
let _29: *const *mut [i64; 7];
let _30: &'static isize;
let _31: [bool; 3];
let _32: isize;
let _33: (((char,),), u8);
let _34: f64;
let _35: i64;
let _36: isize;
let _37: (char,);
let _38: *mut (((char,),), u8);
let _39: u64;
let _40: [i64; 7];
let _41: bool;
let _42: [u64; 5];
let _43: ((char,),);
let _44: Adt45;
let _45: f32;
let _46: i128;
let _47: ();
let _48: ();
{
RET = '\u{9bbff}';
_4 = [229_u8,70_u8,135_u8,232_u8,75_u8,189_u8];
_2 = (-899429942_i32) & (-227857475_i32);
_4 = [69_u8,58_u8,193_u8,220_u8,68_u8,138_u8];
RET = '\u{10cdbc}';
RET = '\u{df5bf}';
RET = '\u{a025f}';
_2 = 1763205982_i32;
_3 = _1 + _1;
RET = '\u{1f5ec}';
_3 = 93014137346689812737384658218461707135_u128 as isize;
_1 = -_3;
_1 = 136129410427266238313355021242761299844_i128 as isize;
_3 = _1 << _2;
_2 = false as i32;
_1 = _3 << _3;
_6 = 4104747454_u32;
_1 = -_3;
_6 = 104_i8 as u32;
_7 = 28267_i16;
_4 = [110_u8,88_u8,185_u8,206_u8,80_u8,51_u8];
RET = '\u{af6da}';
_4 = [218_u8,235_u8,21_u8,126_u8,77_u8,134_u8];
RET = '\u{e6291}';
_6 = _2 as u32;
RET = '\u{fb1f3}';
RET = '\u{f2d3f}';
match _7 {
28267 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2 = (-222773365_i32) >> _6;
_6 = 2031249407_u32;
_7 = _2 as i16;
RET = '\u{82886}';
RET = '\u{1e2e6}';
_3 = 4_usize as isize;
_4 = [147_u8,208_u8,93_u8,162_u8,223_u8,182_u8];
_3 = _1 + _1;
_1 = true as isize;
RET = '\u{6c2f6}';
_3 = 5_usize as isize;
_8 = !_6;
_1 = !_3;
RET = '\u{11e64}';
_7 = (-3518_i16) & 2141_i16;
_10 = !_6;
_9 = _7 as i8;
RET = '\u{f7fc7}';
match _6 {
2031249407 => bb3,
_ => bb1
}
}
bb3 = {
_3 = _1 << _1;
RET = '\u{c5bac}';
_3 = -_1;
_6 = _9 as u32;
_4 = [71_u8,62_u8,153_u8,208_u8,115_u8,138_u8];
_11 = false & false;
_3 = _1;
_4 = [101_u8,150_u8,204_u8,169_u8,18_u8,96_u8];
Goto(bb4)
}
bb4 = {
_4 = [235_u8,142_u8,89_u8,61_u8,212_u8,11_u8];
_7 = _11 as i16;
_6 = !_8;
Goto(bb5)
}
bb5 = {
_1 = _3 ^ _3;
_7 = 8432733667749664187_u64 as i16;
RET = '\u{58ebf}';
_13 = _3;
_16.0 = [_11,_11,_11,_11,_11];
_1 = _13 >> _6;
_9 = !41_i8;
_10 = _8;
_3 = 4_usize as isize;
_9 = !109_i8;
_10 = !_6;
_10 = !_6;
_11 = true & false;
_13 = _1;
_4 = [64_u8,129_u8,136_u8,111_u8,25_u8,153_u8];
_14 = [5078583360253558081_u64,7905098527417568542_u64,5124263016718135014_u64,14930035844006337700_u64,7684045200883864715_u64];
_3 = _1 ^ _1;
_8 = !_10;
_14 = [7048729317875677811_u64,13527119312788807497_u64,9873520836882277148_u64,17672295563961200559_u64,18408802121285131533_u64];
RET = '\u{4a6de}';
Goto(bb6)
}
bb6 = {
_6 = _8 - _8;
_3 = _1 * _13;
_3 = _9 as isize;
_12 = _9 as f64;
_17 = (RET,);
_12 = 5752417911193838332_i64 as f64;
_1 = !_3;
_12 = 158129338130645584_i64 as f64;
_17.0 = RET;
_17 = (RET,);
_10 = _6;
_6 = 84343640353020613480731712453923173889_i128 as u32;
_13 = -_3;
_10 = !_6;
_17.0 = RET;
_16.0 = [_11,_11,_11,_11,_11];
RET = _17.0;
_12 = _2 as f64;
_3 = !_13;
_10 = _8;
_7 = (-7182_i16);
_9 = -63_i8;
_1 = !_13;
_2 = 1067600995_i32 ^ (-2135285949_i32);
_7 = !15140_i16;
Call(_16 = fn16(_4, _13, _4, _14, _14, _4, RET, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _1 | _13;
_2 = 2924433235320132349_usize as i32;
RET = _17.0;
_16.0 = [_11,_11,_11,_11,_11];
_8 = _6;
Goto(bb8)
}
bb8 = {
_12 = _13 as f64;
_16.0 = [_11,_11,_11,_11,_11];
_14 = [438366060236626016_u64,15835074799353174855_u64,11498864118759046544_u64,12595315237589745544_u64,18271520729818399022_u64];
_9 = _6 as i8;
_22.3 = _2 as usize;
_22.1 = core::ptr::addr_of_mut!(_16.0);
_17.0 = RET;
_13 = _22.3 as isize;
_12 = 56_u8 as f64;
_3 = (-30215157840333559014590716549344755077_i128) as isize;
_7 = 27283_i16;
_8 = _6;
_22.3 = !0_usize;
RET = _17.0;
_7 = 3853_i16;
_2 = 2129971409_i32;
_6 = _8 - _8;
_4 = [154_u8,22_u8,158_u8,130_u8,226_u8,99_u8];
_4 = [112_u8,84_u8,82_u8,96_u8,227_u8,155_u8];
_7 = !10961_i16;
_17.0 = RET;
_13 = _1;
_20 = _11;
_2 = -374123812_i32;
_22.3 = 5_usize | 1_usize;
_12 = _22.3 as f64;
_3 = _1;
_10 = !_6;
RET = _17.0;
_22.3 = 3_usize & 7_usize;
Call(_23 = core::intrinsics::bswap(_3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16.0 = [_20,_20,_11,_20,_20];
_19 = &_24.0.0;
_24.2.0 = -_7;
_22.1 = core::ptr::addr_of_mut!(_16.0);
_12 = _1 as f64;
_24.0.0.0 = _17;
_24.0.0.0.0 = RET;
_20 = _11;
Goto(bb10)
}
bb10 = {
_1 = _2 as isize;
_22.1 = core::ptr::addr_of_mut!(_16.0);
_4 = [206_u8,55_u8,48_u8,181_u8,245_u8,45_u8];
_24.0.1 = _3 as u8;
_14 = [1983988085628846958_u64,17197089873716760625_u64,16011875431087966233_u64,3204799187218781941_u64,9910415537401440378_u64];
_2 = -(-262455988_i32);
_9 = 5203118828661609838_u64 as i8;
_2 = _7 as i32;
RET = _24.0.0.0.0;
_2 = -1998780635_i32;
_28 = _3 >> _3;
_27 = _3 ^ _28;
_28 = -_1;
_7 = _24.2.0 - _24.2.0;
_24.1 = core::ptr::addr_of!(_24.0);
_4 = [_24.0.1,_24.0.1,_24.0.1,_24.0.1,_24.0.1,_24.0.1];
_24.0.1 = !192_u8;
_10 = !_6;
_6 = 21486_u16 as u32;
_23 = !_27;
_10 = 3781756693381487296_u64 as u32;
Goto(bb11)
}
bb11 = {
_27 = -_1;
_26 = [4870607317847775831_u64,8704139599643346018_u64,16681208587344535610_u64];
_12 = _22.3 as f64;
_24.1 = core::ptr::addr_of!(_24.0);
_32 = !_28;
_14 = [17755556856098447733_u64,5962525898149164391_u64,14714696219386899209_u64,10933206868121147788_u64,1016172404709871362_u64];
_34 = _12 * _12;
_11 = _22.3 > _22.3;
Goto(bb12)
}
bb12 = {
_6 = _8;
_34 = _23 as f64;
_36 = _11 as isize;
_13 = !_36;
_2 = 335391397005864453177018713471977712556_u128 as i32;
_31 = [_20,_11,_11];
_26 = [1911995544521608763_u64,9506349863895966182_u64,1771570795320954164_u64];
_33.0.0 = (_24.0.0.0.0,);
_22.2 = Adt48::Variant0 { fld0: Move(_24.1) };
_31 = [_11,_11,_11];
_8 = !_10;
_19 = &_24.0.0;
_26 = [6467611866812961951_u64,14480483159014723815_u64,10349612948978119805_u64];
_26 = [347512101061126136_u64,15060066224042086301_u64,18026994758768708133_u64];
_33.0.0 = _17;
_24.0.0 = (_17,);
_27 = (-3924959226110717245_i64) as isize;
_31 = [_11,_11,_20];
_36 = _9 as isize;
_33.0.0 = _17;
_33.0 = (_17,);
Goto(bb13)
}
bb13 = {
_31 = [_11,_11,_20];
Call(_41 = fn17(_33.0.0, _31, _20, _16.0, _32, _33.0.0.0, _31, _36, _12, _13, _28), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_24.0.0.0 = _17;
_19 = &_24.0.0;
_27 = _41 as isize;
_6 = _10;
_34 = _12;
_42 = _14;
_3 = -_28;
SetDiscriminant(_22.2, 1);
_39 = !14129175816030632738_u64;
_43.0 = _33.0.0;
_24.1 = core::ptr::addr_of!(_33);
_43.0 = ((*_19).0.0,);
_8 = _10 * _10;
_21 = core::ptr::addr_of_mut!(place!(Field::<i128>(Variant(_22.2, 1), 0)));
_24.0.0.0.0 = _17.0;
_33 = _24.0;
_26 = [_39,_39,_39];
_43.0 = _17;
_12 = -_34;
_24.0.0.0 = (_43.0.0,);
_20 = _41;
_19 = &_43;
_43.0.0 = _17.0;
(*_21) = -(-73313545318943496263055156311520947548_i128);
_10 = _8 + _8;
_28 = _23 + _13;
_24.0 = (_33.0, _33.1);
_33.0.0 = _43.0;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(15_usize, 33_usize, Move(_33), 1_usize, Move(_1), 39_usize, Move(_39), 42_usize, Move(_42)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(15_usize, 17_usize, Move(_17), 20_usize, Move(_20), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(15_usize, 6_usize, Move(_6), 32_usize, Move(_32), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(15_usize, 27_usize, Move(_27), 48_usize, _48, 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [u8; 6],mut _2: isize,mut _3: [u8; 6],mut _4: [u64; 5],mut _5: [u64; 5],mut _6: [u8; 6],mut _7: char,mut _8: [u8; 6]) -> ([bool; 5],) {
mir! {
type RET = ([bool; 5],);
let _9: char;
let _10: &'static usize;
let _11: *const (((char,),), u8);
let _12: [u64; 3];
let _13: Adt44;
let _14: [i8; 5];
let _15: (char,);
let _16: bool;
let _17: *const &'static &'static *mut u128;
let _18: Adt69;
let _19: ([bool; 5],);
let _20: ();
let _21: ();
{
_8 = [100_u8,107_u8,31_u8,72_u8,0_u8,176_u8];
RET.0 = [false,true,true,true,false];
_3 = [149_u8,105_u8,246_u8,47_u8,60_u8,171_u8];
_3 = [55_u8,125_u8,231_u8,159_u8,86_u8,22_u8];
RET.0 = [true,true,true,true,false];
_2 = (-49_i8) as isize;
_9 = _7;
RET.0 = [false,false,true,false,false];
_8 = [174_u8,151_u8,83_u8,26_u8,229_u8,68_u8];
_9 = _7;
_4 = [7634803197158008428_u64,17569853911936348551_u64,17975886144791991078_u64,6938831309795700774_u64,10310262092036386180_u64];
_7 = _9;
RET.0 = [false,true,true,false,false];
_1 = _8;
_6 = _3;
_4 = [1677939650961509642_u64,10296330683749667286_u64,6948347057226931643_u64,7304348333471124970_u64,7880349687581884098_u64];
RET.0 = [true,false,false,false,true];
_4 = [5515276510205230327_u64,17310571792151909889_u64,15877833513199480260_u64,4478899404120591819_u64,9304484093629912319_u64];
_5 = [6348591686990065946_u64,5404392932589013111_u64,5209480842876959731_u64,15026226492940710485_u64,1199508256911925878_u64];
_6 = [155_u8,127_u8,21_u8,68_u8,241_u8,186_u8];
_9 = _7;
_1 = _8;
_12 = [9134114323833154449_u64,2397009271319841832_u64,6684175281748273958_u64];
Goto(bb1)
}
bb1 = {
_15.0 = _9;
_12 = [5572608753235981103_u64,10848723890911006687_u64,6842153794551035548_u64];
_6 = [102_u8,209_u8,61_u8,222_u8,106_u8,101_u8];
_8 = [24_u8,143_u8,139_u8,157_u8,50_u8,87_u8];
_2 = 45_isize;
_14 = [42_i8,(-6_i8),28_i8,(-12_i8),27_i8];
_1 = [145_u8,28_u8,247_u8,83_u8,145_u8,116_u8];
_7 = _15.0;
_16 = false;
_4 = [13678956822930784774_u64,2731555104596089492_u64,18005756476966770573_u64,16514405786473618839_u64,570663528709500145_u64];
_2 = 119_isize;
_15 = (_9,);
_15 = (_9,);
match _2 {
0 => bb2,
1 => bb3,
119 => bb5,
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
_15.0 = _9;
_16 = _7 == _7;
_4 = [10503989633295851490_u64,1073055272232424743_u64,4158885219635369260_u64,8314881539863911345_u64,17111838066120317011_u64];
_15 = (_7,);
_6 = _1;
match _2 {
0 => bb1,
1 => bb6,
2 => bb7,
119 => bb9,
_ => bb8
}
}
bb6 = {
Return()
}
bb7 = {
_15.0 = _9;
_12 = [5572608753235981103_u64,10848723890911006687_u64,6842153794551035548_u64];
_6 = [102_u8,209_u8,61_u8,222_u8,106_u8,101_u8];
_8 = [24_u8,143_u8,139_u8,157_u8,50_u8,87_u8];
_2 = 45_isize;
_14 = [42_i8,(-6_i8),28_i8,(-12_i8),27_i8];
_1 = [145_u8,28_u8,247_u8,83_u8,145_u8,116_u8];
_7 = _15.0;
_16 = false;
_4 = [13678956822930784774_u64,2731555104596089492_u64,18005756476966770573_u64,16514405786473618839_u64,570663528709500145_u64];
_2 = 119_isize;
_15 = (_9,);
_15 = (_9,);
match _2 {
0 => bb2,
1 => bb3,
119 => bb5,
_ => bb4
}
}
bb8 = {
Return()
}
bb9 = {
_6 = [100_u8,48_u8,2_u8,99_u8,52_u8,97_u8];
_12 = [16678180600339369300_u64,4874501494830515329_u64,6904002844025922667_u64];
_14 = [124_i8,(-41_i8),91_i8,(-37_i8),121_i8];
_15.0 = _7;
_6 = _1;
_6 = [173_u8,76_u8,66_u8,168_u8,183_u8,158_u8];
_14 = [53_i8,57_i8,(-74_i8),72_i8,(-117_i8)];
RET.0 = [_16,_16,_16,_16,_16];
RET.0 = [_16,_16,_16,_16,_16];
_8 = _3;
_9 = _15.0;
match _2 {
0 => bb6,
1 => bb2,
2 => bb4,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
119 => bb15,
_ => bb14
}
}
bb10 = {
Return()
}
bb11 = {
_15.0 = _9;
_12 = [5572608753235981103_u64,10848723890911006687_u64,6842153794551035548_u64];
_6 = [102_u8,209_u8,61_u8,222_u8,106_u8,101_u8];
_8 = [24_u8,143_u8,139_u8,157_u8,50_u8,87_u8];
_2 = 45_isize;
_14 = [42_i8,(-6_i8),28_i8,(-12_i8),27_i8];
_1 = [145_u8,28_u8,247_u8,83_u8,145_u8,116_u8];
_7 = _15.0;
_16 = false;
_4 = [13678956822930784774_u64,2731555104596089492_u64,18005756476966770573_u64,16514405786473618839_u64,570663528709500145_u64];
_2 = 119_isize;
_15 = (_9,);
_15 = (_9,);
match _2 {
0 => bb2,
1 => bb3,
119 => bb5,
_ => bb4
}
}
bb12 = {
Return()
}
bb13 = {
_15.0 = _9;
_16 = _7 == _7;
_4 = [10503989633295851490_u64,1073055272232424743_u64,4158885219635369260_u64,8314881539863911345_u64,17111838066120317011_u64];
_15 = (_7,);
_6 = _1;
match _2 {
0 => bb1,
1 => bb6,
2 => bb7,
119 => bb9,
_ => bb8
}
}
bb14 = {
Return()
}
bb15 = {
_5 = [348205039988397575_u64,4159647939403668821_u64,16566173575044225151_u64,8968237569508750298_u64,8138905289394656538_u64];
_14 = [24_i8,115_i8,(-30_i8),(-92_i8),(-60_i8)];
_15 = (_7,);
_16 = _7 <= _9;
_14 = [(-59_i8),3_i8,(-32_i8),(-95_i8),119_i8];
RET.0 = [_16,_16,_16,_16,_16];
_8 = [90_u8,69_u8,208_u8,225_u8,172_u8,244_u8];
_12 = [4049223525890726982_u64,18109834635512945116_u64,16275032970604884368_u64];
_4 = [6048077154717188063_u64,364036580965571371_u64,4218993747046717810_u64,11214884303787670378_u64,5446034642668132450_u64];
_19.0 = [_16,_16,_16,_16,_16];
RET.0 = _19.0;
_15 = (_9,);
RET.0 = _19.0;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(16_usize, 3_usize, Move(_3), 12_usize, Move(_12), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(16_usize, 5_usize, Move(_5), 6_usize, Move(_6), 4_usize, Move(_4), 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (char,),mut _2: [bool; 3],mut _3: bool,mut _4: [bool; 5],mut _5: isize,mut _6: char,mut _7: [bool; 3],mut _8: isize,mut _9: f64,mut _10: isize,mut _11: isize) -> bool {
mir! {
type RET = bool;
let _12: bool;
let _13: u128;
let _14: *const &'static &'static *mut u128;
let _15: Adt65;
let _16: (((char,),), u8);
let _17: f64;
let _18: u8;
let _19: *const (((char,),), u8);
let _20: Adt65;
let _21: i64;
let _22: u16;
let _23: *mut bool;
let _24: ((char,),);
let _25: *const Adt45;
let _26: *const (((char,),), u8);
let _27: bool;
let _28: i32;
let _29: bool;
let _30: *mut (((char,),), u8);
let _31: u128;
let _32: i64;
let _33: isize;
let _34: Adt57;
let _35: *const &'static &'static *mut u128;
let _36: &'static &'static *mut u128;
let _37: ();
let _38: ();
{
_4 = [_3,_3,_3,_3,_3];
_6 = _1.0;
_1 = (_6,);
_8 = -_10;
_7 = [_3,_3,_3];
_9 = 26017_u16 as f64;
_6 = _1.0;
_3 = true;
_2 = _7;
_7 = [_3,_3,_3];
RET = _9 <= _9;
_3 = RET;
_6 = _1.0;
RET = !_3;
_2 = [_3,RET,_3];
_9 = 127_u8 as f64;
_8 = _10;
RET = _3;
_12 = _3;
_1.0 = _6;
Goto(bb1)
}
bb1 = {
_1.0 = _6;
_3 = RET;
_4 = [RET,RET,_3,_12,RET];
_2 = _7;
_13 = _5 as u128;
_13 = !255343452054111657241474540326030728612_u128;
_7 = [RET,_12,RET];
_1 = (_6,);
_4 = [RET,RET,_3,_3,RET];
_1 = (_6,);
_1.0 = _6;
_4 = [_3,RET,RET,_12,_12];
_9 = 19006_u16 as f64;
_1.0 = _6;
_1 = (_6,);
RET = _8 >= _11;
Goto(bb2)
}
bb2 = {
_12 = _3;
_1 = (_6,);
_1 = (_6,);
_4 = [RET,RET,RET,_12,RET];
_6 = _1.0;
_2 = [RET,_3,RET];
_9 = 17286186072418254226_u64 as f64;
_6 = _1.0;
_16.0.0 = _1;
_1.0 = _6;
_16.0.0.0 = _1.0;
_5 = !_8;
_17 = -_9;
_16.0.0.0 = _1.0;
_16.1 = !6_u8;
_13 = 276663928947872351228354077453320101312_u128;
RET = _12;
match _13 {
276663928947872351228354077453320101312 => bb3,
_ => bb1
}
}
bb3 = {
_10 = -_8;
_3 = _12 | _12;
_17 = -_9;
_18 = _16.1 | _16.1;
_16.0.0.0 = _1.0;
_16.1 = _18 | _18;
_5 = !_10;
_19 = core::ptr::addr_of!(_16);
(*_19).1 = _18 << _8;
_9 = _13 as f64;
_21 = 1643455281_u32 as i64;
_22 = !62453_u16;
(*_19).0.0 = _1;
(*_19).0.0.0 = _6;
match _13 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
276663928947872351228354077453320101312 => bb8,
_ => bb7
}
}
bb4 = {
_12 = _3;
_1 = (_6,);
_1 = (_6,);
_4 = [RET,RET,RET,_12,RET];
_6 = _1.0;
_2 = [RET,_3,RET];
_9 = 17286186072418254226_u64 as f64;
_6 = _1.0;
_16.0.0 = _1;
_1.0 = _6;
_16.0.0.0 = _1.0;
_5 = !_8;
_17 = -_9;
_16.0.0.0 = _1.0;
_16.1 = !6_u8;
_13 = 276663928947872351228354077453320101312_u128;
RET = _12;
match _13 {
276663928947872351228354077453320101312 => bb3,
_ => bb1
}
}
bb5 = {
_1.0 = _6;
_3 = RET;
_4 = [RET,RET,_3,_12,RET];
_2 = _7;
_13 = _5 as u128;
_13 = !255343452054111657241474540326030728612_u128;
_7 = [RET,_12,RET];
_1 = (_6,);
_4 = [RET,RET,_3,_3,RET];
_1 = (_6,);
_1.0 = _6;
_4 = [_3,RET,RET,_12,_12];
_9 = 19006_u16 as f64;
_1.0 = _6;
_1 = (_6,);
RET = _8 >= _11;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_12 = !RET;
_5 = _11 << _10;
_3 = !RET;
_10 = !_11;
_16.0 = (_1,);
_11 = _5;
_2 = [_12,_12,_12];
(*_19).1 = _18 >> _8;
(*_19).0.0.0 = _1.0;
_21 = (-2984_i16) as i64;
_9 = _17 + _17;
Goto(bb9)
}
bb9 = {
_18 = !_16.1;
_3 = !_12;
_16.0.0.0 = _1.0;
(*_19).0 = (_1,);
_24 = (_1,);
(*_19).0.0 = (_6,);
_11 = _8 ^ _8;
_16.0.0.0 = _6;
_18 = _16.1;
_16.1 = _18 ^ _18;
_6 = _24.0.0;
_12 = RET;
_6 = _16.0.0.0;
_1.0 = _24.0.0;
_7 = [_3,_3,RET];
_22 = 43701_u16;
(*_19).0.0.0 = _1.0;
(*_19).0.0 = _24.0;
_3 = _12 & _12;
_16.0.0.0 = _6;
match _13 {
0 => bb3,
1 => bb7,
276663928947872351228354077453320101312 => bb11,
_ => bb10
}
}
bb10 = {
_12 = _3;
_1 = (_6,);
_1 = (_6,);
_4 = [RET,RET,RET,_12,RET];
_6 = _1.0;
_2 = [RET,_3,RET];
_9 = 17286186072418254226_u64 as f64;
_6 = _1.0;
_16.0.0 = _1;
_1.0 = _6;
_16.0.0.0 = _1.0;
_5 = !_8;
_17 = -_9;
_16.0.0.0 = _1.0;
_16.1 = !6_u8;
_13 = 276663928947872351228354077453320101312_u128;
RET = _12;
match _13 {
276663928947872351228354077453320101312 => bb3,
_ => bb1
}
}
bb11 = {
_9 = _17 + _17;
(*_19).0.0 = (_6,);
_11 = !_5;
(*_19) = (_24, _18);
RET = _12;
(*_19).1 = _18;
(*_19).1 = 973627763_u32 as u8;
_26 = core::ptr::addr_of!((*_19));
(*_19).0.0 = (_24.0.0,);
(*_19).0.0 = (_6,);
_16.0 = (_1,);
_24.0 = (*_26).0.0;
(*_26).0.0 = _24.0;
_27 = RET;
_7 = [_3,RET,_3];
_16 = (_24, _18);
(*_19).0.0 = (_6,);
_16 = (_24, _18);
(*_26).0.0 = _1;
_16.1 = 507374765_u32 as u8;
_21 = (-3349020095062033090_i64) * 7398583468508284635_i64;
Goto(bb12)
}
bb12 = {
_9 = _17 + _17;
(*_19).1 = _3 as u8;
_23 = core::ptr::addr_of_mut!(_29);
_6 = _16.0.0.0;
(*_19).0.0.0 = _6;
_17 = 4142524404_u32 as f64;
_31 = _13;
match _13 {
0 => bb6,
276663928947872351228354077453320101312 => bb13,
_ => bb11
}
}
bb13 = {
_28 = !265913188_i32;
_5 = -_11;
_24.0 = (_1.0,);
_12 = _3;
_6 = (*_26).0.0.0;
(*_26).0.0.0 = _1.0;
_17 = _9;
_24 = (_16.0.0,);
(*_26).1 = !_18;
(*_19).0 = _24;
(*_26).0 = (_1,);
_16.1 = 7914501497258905045_usize as u8;
(*_19).1 = _18;
_16.0 = _24;
_3 = _12;
RET = !_3;
Goto(bb14)
}
bb14 = {
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(17_usize, 13_usize, Move(_13), 11_usize, Move(_11), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(17_usize, 28_usize, Move(_28), 5_usize, Move(_5), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(17_usize, 24_usize, Move(_24), 1_usize, Move(_1), 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18() -> bool {
mir! {
type RET = bool;
let _1: &'static usize;
let _2: (u8, &'static Adt44);
let _3: f32;
let _4: &'static ((char,),);
let _5: &'static &'static *mut u128;
let _6: [u64; 1];
let _7: *mut u64;
let _8: *const &'static &'static *mut u128;
let _9: *const Adt45;
let _10: ([u64; 3], isize);
let _11: [bool; 5];
let _12: [i8; 5];
let _13: i8;
let _14: f64;
let _15: bool;
let _16: f64;
let _17: *mut u128;
let _18: isize;
let _19: char;
let _20: char;
let _21: bool;
let _22: Adt24;
let _23: i32;
let _24: bool;
let _25: [u64; 3];
let _26: Adt48;
let _27: isize;
let _28: bool;
let _29: ();
let _30: ();
{
RET = 87734746533399660293956747925997019391_u128 >= 68187382495747616496783608432654596001_u128;
RET = (-9223372036854775808_isize) != (-9223372036854775808_isize);
RET = !false;
RET = !true;
RET = true & false;
RET = !true;
RET = 15469582969567381244_u64 <= 5734284502569238279_u64;
RET = !false;
_2.0 = 188_u8 >> 1151_i16;
_3 = 47738_u16 as f32;
RET = !true;
Goto(bb1)
}
bb1 = {
_2.0 = 126_u8;
_3 = 1077357545_i32 as f32;
RET = false;
RET = _3 == _3;
_2.0 = 63686_u16 as u8;
Goto(bb2)
}
bb2 = {
RET = true & true;
_2.0 = 189_u8 << 9223372036854775807_isize;
RET = true;
_3 = 146070628985877878422266775342773959936_i128 as f32;
RET = false;
RET = false;
_3 = _2.0 as f32;
RET = _3 >= _3;
Goto(bb3)
}
bb3 = {
_6 = [1536362494326892927_u64];
_2.0 = 189_u8 - 51_u8;
_6 = [5892816510490679505_u64];
_2.0 = (-55_i8) as u8;
_2.0 = RET as u8;
RET = true;
RET = true;
RET = true & true;
Goto(bb4)
}
bb4 = {
RET = !false;
RET = _2.0 == _2.0;
RET = true | false;
_2.0 = (-130786_i32) as u8;
_6 = [2122074515667050475_u64];
RET = true;
_6 = [5423418322846970528_u64];
RET = _3 < _3;
Goto(bb5)
}
bb5 = {
_6 = [18179015661814311060_u64];
_6 = [1834927802715323906_u64];
RET = false;
RET = _2.0 < _2.0;
RET = true & false;
_6 = [6939480461443382666_u64];
RET = true;
_3 = 23045_u16 as f32;
Goto(bb6)
}
bb6 = {
_2.0 = !77_u8;
_2.0 = !234_u8;
RET = _3 != _3;
RET = !false;
_3 = (-1372034654_i32) as f32;
_6 = [13947415464986409733_u64];
RET = !false;
RET = true;
RET = false;
_2.0 = !65_u8;
RET = false ^ true;
_8 = core::ptr::addr_of!(_5);
RET = !false;
RET = !true;
Goto(bb7)
}
bb7 = {
_10.1 = (-9223372036854775808_isize) << _2.0;
_10.1 = (-9223372036854775808_isize) | 82_isize;
_10.0 = [4711524129651657664_u64,16627229785381674974_u64,15538120344494954729_u64];
_3 = 6628323906339919916_i64 as f32;
_10.1 = -(-9223372036854775808_isize);
RET = !true;
RET = _10.1 == _10.1;
_10.0 = [10806661375885624793_u64,9381786581692587410_u64,18263953232671778842_u64];
RET = !false;
RET = true;
Goto(bb8)
}
bb8 = {
_11 = [RET,RET,RET,RET,RET];
RET = !false;
_11 = [RET,RET,RET,RET,RET];
_10.1 = !(-24_isize);
_12 = [27_i8,25_i8,(-24_i8),(-99_i8),(-75_i8)];
_10.1 = (-20922642890625439778654124453867265912_i128) as isize;
_12 = [21_i8,100_i8,11_i8,112_i8,11_i8];
_3 = (-93394060891889490768179363803799939073_i128) as f32;
_8 = core::ptr::addr_of!((*_8));
_10.0 = [12540822952456314976_u64,10861595671454972960_u64,12990095693609978238_u64];
_11 = [RET,RET,RET,RET,RET];
_10.1 = 9223372036854775807_isize - (-9223372036854775808_isize);
RET = true;
_14 = _10.1 as f64;
_16 = 84786207187048501474276784401350362812_u128 as f64;
_11 = [RET,RET,RET,RET,RET];
_13 = _2.0 as i8;
_6 = [4940691721211550291_u64];
_10.1 = 9223372036854775807_isize * (-96_isize);
RET = !true;
_11 = [RET,RET,RET,RET,RET];
_15 = _10.1 > _10.1;
RET = _15;
RET = _13 <= _13;
_18 = _10.1;
RET = _15;
Goto(bb9)
}
bb9 = {
_10.0 = [7252366995357979428_u64,14877859499165336266_u64,14379355758372423046_u64];
_12 = [_13,_13,_13,_13,_13];
RET = !_15;
_8 = core::ptr::addr_of!((*_8));
_19 = '\u{3abe1}';
Goto(bb10)
}
bb10 = {
_16 = _14 + _14;
RET = _14 >= _16;
RET = _15;
RET = !_15;
_20 = _19;
_13 = (-27_i8) + (-23_i8);
_8 = core::ptr::addr_of!((*_8));
_12 = [_13,_13,_13,_13,_13];
_18 = _10.1;
_13 = (-6727971090740833389_i64) as i8;
_12 = [_13,_13,_13,_13,_13];
_12 = [_13,_13,_13,_13,_13];
RET = !_15;
_22.fld3 = 8561031069347764860_u64;
_7 = core::ptr::addr_of_mut!(_22.fld3);
_18 = 58672374172924849913746965608751479728_u128 as isize;
_20 = _19;
RET = _15;
_12 = [_13,_13,_13,_13,_13];
_16 = _14 * _14;
_24 = _15 ^ RET;
_21 = !_15;
_23 = 152425199_i32 & 242049198_i32;
RET = _24;
match (*_7) {
0 => bb3,
1 => bb11,
2 => bb12,
3 => bb13,
8561031069347764860 => bb15,
_ => bb14
}
}
bb11 = {
_10.0 = [7252366995357979428_u64,14877859499165336266_u64,14379355758372423046_u64];
_12 = [_13,_13,_13,_13,_13];
RET = !_15;
_8 = core::ptr::addr_of!((*_8));
_19 = '\u{3abe1}';
Goto(bb10)
}
bb12 = {
RET = true & true;
_2.0 = 189_u8 << 9223372036854775807_isize;
RET = true;
_3 = 146070628985877878422266775342773959936_i128 as f32;
RET = false;
RET = false;
_3 = _2.0 as f32;
RET = _3 >= _3;
Goto(bb3)
}
bb13 = {
_6 = [18179015661814311060_u64];
_6 = [1834927802715323906_u64];
RET = false;
RET = _2.0 < _2.0;
RET = true & false;
_6 = [6939480461443382666_u64];
RET = true;
_3 = 23045_u16 as f32;
Goto(bb6)
}
bb14 = {
RET = !false;
RET = _2.0 == _2.0;
RET = true | false;
_2.0 = (-130786_i32) as u8;
_6 = [2122074515667050475_u64];
RET = true;
_6 = [5423418322846970528_u64];
RET = _3 < _3;
Goto(bb5)
}
bb15 = {
_10.0 = [_22.fld3,(*_7),(*_7)];
_22.fld2 = (*_7) as u8;
_12 = [_13,_13,_13,_13,_13];
_25 = [_22.fld3,(*_7),(*_7)];
_21 = _19 <= _20;
_22 = Adt24 { fld0: _21,fld1: (-20853790044822260124117689689231355417_i128),fld2: _2.0,fld3: 11891634324912843332_u64 };
RET = !_22.fld0;
_23 = 2115385138_i32;
_25 = _10.0;
_20 = _19;
_18 = _14 as isize;
_22.fld2 = _14 as u8;
_22.fld1 = (-146419626260933219976777465297540195670_i128);
_27 = _3 as isize;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(18_usize, 24_usize, Move(_24), 10_usize, Move(_10), 27_usize, Move(_27), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(18_usize, 19_usize, Move(_19), 23_usize, Move(_23), 13_usize, Move(_13), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{3075f}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(94_i8), std::hint::black_box((-26048_i16)), std::hint::black_box((-1890331457_i32)), std::hint::black_box(7730254579131015984_i64), std::hint::black_box(63395357049228163232785330066048735619_i128), std::hint::black_box(78352760584793583418606829336322517703_u128), std::hint::black_box(6_u8), std::hint::black_box(46934_u16), std::hint::black_box(2408532276_u32));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt24{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt24 {
fld0: bool,
fld1: i128,
fld2: u8,
fld3: u64,
}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){unsafe{printf("Adt34::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt34 {
Variant0{
fld0: bool,
fld1: [char; 5],
fld2: ((char,),),
fld3: f64,
fld4: [u8; 6],
fld5: (isize,),
fld6: *const (((char,),), u8),
fld7: f32,

},
Variant1{
fld0: bool,
fld1: u8,
fld2: f64,
fld3: i8,

},
Variant2{
fld0: bool,
fld1: [u64; 3],
fld2: i32,
fld3: [u8; 6],
fld4: u8,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u8,
fld1: char,
fld2: isize,
fld3: [char; 5],
fld4: f64,
fld5: u16,

},
Variant1{
fld0: *mut u128,
fld1: char,
fld2: f64,
fld3: *const (((char,),), u8),
fld4: usize,
fld5: *mut (((char,),), u8),
fld6: u32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: Adt34,
fld1: u8,
fld2: isize,
fld3: [bool; 5],
fld4: [i16; 8],

},
Variant1{
fld0: bool,
fld1: (isize,),
fld2: usize,
fld3: u8,
fld4: [i8; 5],

},
Variant2{
fld0: u64,
fld1: (i16,),
fld2: f32,
fld3: f64,
fld4: u16,
fld5: [i64; 7],

},
Variant3{
fld0: i128,
fld1: ((char,),),
fld2: u8,
fld3: (i16,),
fld4: [u64; 1],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const (((char,),), u8),

},
Variant1{
fld0: i128,
fld1: [bool; 3],
fld2: [bool; 5],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt24,
fld1: i128,

},
Variant1{
fld0: u64,
fld1: *mut [u8; 6],
fld2: (isize,),

},
Variant2{
fld0: usize,
fld1: *const (((char,),), u8),
fld2: *mut u64,
fld3: *mut (((char,),), u8),
fld4: [char; 5],
fld5: [i8; 5],
fld6: Adt24,

},
Variant3{
fld0: Adt34,
fld1: char,
fld2: isize,
fld3: *mut u128,
fld4: *mut (((char,),), u8),
fld5: Adt45,
fld6: f64,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: [i64; 7],
fld1: Adt24,
fld2: isize,
fld3: *const i32,
fld4: *const (((char,),), u8),
fld5: *mut (((char,),), u8),
fld6: *mut [i64; 7],

},
Variant1{
fld0: *const i32,
fld1: *mut i128,
fld2: f32,
fld3: ((char,),),
fld4: [i8; 5],

},
Variant2{
fld0: u16,
fld1: *const (((char,),), u8),
fld2: f64,
fld3: *mut i128,
fld4: ((char,),),
fld5: (Adt45,),
fld6: (((char,),), u8),

}}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){unsafe{printf("Adt69::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt69 {
Variant0{
fld0: *const Adt24,
fld1: *const i32,
fld2: isize,
fld3: Adt57,
fld4: *mut [u8; 6],
fld5: [i64; 7],

},
Variant1{
fld0: i64,
fld1: usize,
fld2: *mut [u8; 6],
fld3: [i8; 5],
fld4: (i16,),

}}

