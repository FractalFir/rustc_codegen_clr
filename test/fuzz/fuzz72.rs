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
pub fn fn0(mut _1: isize,mut _2: u16) -> *mut u8 {
mir! {
type RET = *mut u8;
let _3: Adt55;
let _4: (isize,);
let _5: (isize,);
let _6: Adt49;
let _7: Adt49;
let _8: char;
let _9: (bool, i64, u16);
let _10: u128;
let _11: usize;
let _12: (f32, char, u8, f32);
let _13: isize;
let _14: i128;
let _15: char;
let _16: char;
let _17: Adt51;
let _18: i64;
let _19: i64;
let _20: Adt40;
let _21: i8;
let _22: u128;
let _23: Adt40;
let _24: Adt48;
let _25: ([i32; 7], u64);
let _26: ([i32; 7], u64);
let _27: &'static isize;
let _28: isize;
let _29: f64;
let _30: (f32, char, u8, f32);
let _31: f64;
let _32: isize;
let _33: u16;
let _34: char;
let _35: [i128; 6];
let _36: Adt43;
let _37: isize;
let _38: (u64, f32, i16, isize);
let _39: i128;
let _40: usize;
let _41: *mut (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _42: [isize; 8];
let _43: (bool, i64, u16);
let _44: isize;
let _45: usize;
let _46: i16;
let _47: char;
let _48: [i32; 7];
let _49: usize;
let _50: isize;
let _51: f32;
let _52: f64;
let _53: bool;
let _54: [i32; 7];
let _55: char;
let _56: (isize,);
let _57: u64;
let _58: Adt47;
let _59: i8;
let _60: ();
let _61: ();
{
_1 = 14_isize;
_2 = '\u{8d94d}' as u16;
_1 = 9223372036854775807_isize;
_2 = 275045015405576972188645038337062363536_u128 as u16;
_1 = 27856_i16 as isize;
_4 = (_1,);
_5 = _4;
_5.0 = _4.0;
_2 = 9404_u16 >> _5.0;
_1 = !_4.0;
_5.0 = _1;
_2 = 42447_u16;
_1 = _5.0 & _4.0;
_4.0 = _5.0;
_4 = (_5.0,);
_4 = _5;
_8 = '\u{10ce4d}';
_1 = _4.0 | _5.0;
Call(_5 = fn1(_8, _8, _1, _1, _2, _1, _4.0, _4.0, _2, _4, _1, _2, _1, _1, _8, _4.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _5;
_8 = '\u{bbad1}';
_5.0 = _4.0;
_2 = !43980_u16;
_5 = (_4.0,);
_2 = !13028_u16;
_10 = 194409313419017008367051104705559428851_u128;
_1 = _4.0;
_4 = _5;
_11 = _10 as usize;
_9.0 = !false;
_10 = 138115052958903644859145377143197079254_u128 & 92456369125902085286374491521623663292_u128;
_1 = _4.0 >> _4.0;
_8 = '\u{10d982}';
Call(_11 = fn2(_5, _4.0, _5, _8, _4, _1, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.2 = _4.0 as u16;
_4 = (_1,);
_2 = !_9.2;
_12.1 = _8;
_12.0 = 211_u8 as f32;
_2 = _12.0 as u16;
RET = core::ptr::addr_of_mut!(_12.2);
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = _12.0 as u8;
_9.2 = _2;
_14 = !71492176680611312752502888491473022717_i128;
_12.2 = 152_u8 + 147_u8;
Goto(bb3)
}
bb3 = {
RET = core::ptr::addr_of_mut!((*RET));
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 23_u8 * 139_u8;
_13 = !_1;
(*RET) = _5.0 as u8;
_19 = (-7670840100143407648_i64) + (-1755082683714188064_i64);
(*RET) = 7_u8;
(*RET) = 65_u8;
_12.3 = -_12.0;
RET = core::ptr::addr_of_mut!(_12.2);
Goto(bb4)
}
bb4 = {
(*RET) = !117_u8;
_14 = 1591934007_u32 as i128;
_2 = _9.2 | _9.2;
_1 = -_13;
_1 = _11 as isize;
(*RET) = 182_u8 - 231_u8;
_22 = _14 as u128;
_1 = _13;
_16 = _12.1;
_15 = _16;
_5.0 = _9.0 as isize;
_5.0 = _4.0;
RET = core::ptr::addr_of_mut!(_12.2);
_22 = (*RET) as u128;
_15 = _16;
_25.0 = [(-1360084131_i32),(-392258435_i32),1803411263_i32,49847218_i32,1953261485_i32,1894707891_i32,(-1385618652_i32)];
_9.1 = -_19;
Call(_26.0 = fn5(_1, _13, _1, _5, _5, _13, _1, _4.0, _5, _5.0, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19 = !_9.1;
RET = core::ptr::addr_of_mut!(_30.2);
_12.3 = 5887029810864029204_u64 as f32;
_25 = (_26.0, 7254031759525408483_u64);
_14 = (-108424856990758152178059921237053834774_i128);
_12.2 = 68_u8;
_30.1 = _15;
_18 = _19 ^ _9.1;
_1 = _13;
(*RET) = !_12.2;
_26.1 = !_25.1;
_22 = !_10;
_11 = 1_usize;
_29 = 56_i8 as f64;
_9.2 = _5.0 as u16;
_30.1 = _8;
_28 = (-23_i8) as isize;
_31 = -_29;
_9 = (false, _18, _2);
_9.2 = !_2;
_9.2 = _2 + _2;
Goto(bb6)
}
bb6 = {
_12.1 = _15;
_31 = (-10053_i16) as f64;
(*RET) = !_12.2;
_12.0 = -_12.3;
_9.2 = _2;
_12.3 = _12.0;
_1 = _25.1 as isize;
_25.0 = [_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11]];
_16 = _12.1;
RET = core::ptr::addr_of_mut!((*RET));
Goto(bb7)
}
bb7 = {
_26.0 = [_25.0[_11],_25.0[_11],_25.0[_11],_25.0[_11],_25.0[_11],_25.0[_11],_25.0[_11]];
_33 = !_9.2;
_30.0 = -_12.0;
_5.0 = _13 + _13;
_25.0 = [_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11],_26.0[_11]];
_30.0 = _12.0 * _12.3;
RET = core::ptr::addr_of_mut!(_30.2);
_9 = (false, _18, _33);
_30.2 = _12.2 % _12.2;
_4.0 = _13 * _13;
_12.2 = (*RET);
_28 = (*RET) as isize;
_11 = 2_usize & 0_usize;
_30 = (_12.0, _15, _12.2, _12.0);
_9.2 = !_33;
_19 = -_18;
(*RET) = _12.2;
Goto(bb8)
}
bb8 = {
_31 = _33 as f64;
_36.fld1.2 = !_14;
_25.1 = _26.1;
_26.0 = [(-1256102078_i32),1958299081_i32,1833708607_i32,946006405_i32,(-1296367586_i32),(-1103339415_i32),(-1186634622_i32)];
_30 = (_12.3, _12.1, _12.2, _12.0);
_33 = _9.2;
_36.fld1.3 = (_9.0, _19, _33);
_12.1 = _8;
_12.1 = _30.1;
_26.0 = [(-1176200107_i32),242090988_i32,(-201303259_i32),(-159410500_i32),(-709295522_i32),(-930327279_i32),790695114_i32];
(*RET) = _22 as u8;
_27 = &_4.0;
_26.1 = _25.1;
RET = core::ptr::addr_of_mut!(_12.2);
(*RET) = _30.2;
_32 = _5.0;
_30.0 = _12.2 as f32;
_36.fld1.4 = [_19];
_12.1 = _16;
_1 = -_4.0;
_9.0 = _36.fld1.3.0 & _36.fld1.3.0;
Goto(bb9)
}
bb9 = {
_5.0 = (*RET) as isize;
_15 = _12.1;
(*RET) = !_30.2;
_14 = (-29612_i16) as i128;
_38.1 = _30.3 + _12.0;
_4 = (_1,);
Call(_37 = fn17(_4.0, _4.0, _1, _4.0, _25.0, _13, _4.0, _33), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_36.fld1.2 = _14 << _4.0;
_40 = _11;
_36.fld1.5 = !3862321962_u32;
_26.1 = _25.1;
_34 = _15;
_8 = _15;
_25 = (_26.0, _26.1);
_5.0 = _4.0;
_41 = core::ptr::addr_of_mut!(_36.fld1);
_39 = (*_41).2;
_5 = (_1,);
_27 = &_37;
_1 = (*_27) & _5.0;
_33 = (*_41).2 as u16;
_5 = ((*_27),);
_43 = _9;
Goto(bb11)
}
bb11 = {
_9 = (_43.0, _19, _33);
_38 = (_26.1, _30.0, (-21535_i16), _5.0);
_42 = [_37,_32,_32,_37,_4.0,(*_27),_38.3,(*_27)];
_36.fld1.3 = (_9.0, _19, _9.2);
(*_41).1 = [(-1437725877_i32),2112689197_i32,(-972520978_i32),(-1560609136_i32),1866071053_i32,1781633284_i32,(-957073078_i32)];
_36.fld1.0 = !_36.fld1.5;
_11 = _40;
_44 = (*_27) << _10;
_36.fld1.3.1 = _18 << _44;
_38.2 = 28209_i16 | 26948_i16;
_38.0 = _22 as u64;
_26.0 = _36.fld1.1;
_4 = _5;
_2 = (*_41).3.2;
_36.fld0 = _38.0 << (*_41).2;
(*_41).5 = (*_41).0;
_12 = (_30.0, _16, _30.2, _30.3);
_9.1 = _22 as i64;
_36.fld1.0 = _36.fld1.5 ^ (*_41).5;
Call((*_41).5 = core::intrinsics::bswap((*_41).0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_30.1 = _16;
_41 = core::ptr::addr_of_mut!((*_41));
_39 = !_36.fld1.2;
_12.2 = !_30.2;
_45 = _31 as usize;
_30.1 = _16;
_4.0 = _32 << _9.2;
_40 = !_45;
_36.fld1.1 = _25.0;
_26.0 = [(-224854773_i32),(-1213311037_i32),(-101234960_i32),(-1945574597_i32),(-1682461997_i32),(-1725051118_i32),711473609_i32];
_45 = !_11;
_14 = _36.fld1.2;
_45 = _40;
_36.fld0 = _25.1 + _26.1;
_32 = _37;
(*_41).3 = (_43.0, _9.1, _2);
_13 = (*_27);
Goto(bb13)
}
bb13 = {
(*_41).3 = (_9.0, _18, _2);
Goto(bb14)
}
bb14 = {
(*RET) = _30.2 >> _4.0;
_56.0 = _5.0 << (*_27);
RET = core::ptr::addr_of_mut!(_30.2);
_5 = (_1,);
(*_41).4 = [_36.fld1.3.1];
_29 = -_31;
_28 = _5.0 >> _9.2;
_54 = _26.0;
_12.0 = -_30.3;
_56 = _5;
_28 = -_32;
_47 = _8;
_43.1 = (*_41).3.1;
_36.fld0 = 1826084712_i32 as u64;
Goto(bb15)
}
bb15 = {
Call(_60 = dump_var(0_usize, 32_usize, Move(_32), 18_usize, Move(_18), 22_usize, Move(_22), 45_usize, Move(_45)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_60 = dump_var(0_usize, 47_usize, Move(_47), 28_usize, Move(_28), 10_usize, Move(_10), 56_usize, Move(_56)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_60 = dump_var(0_usize, 33_usize, Move(_33), 2_usize, Move(_2), 11_usize, Move(_11), 37_usize, Move(_37)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_60 = dump_var(0_usize, 9_usize, Move(_9), 15_usize, Move(_15), 34_usize, Move(_34), 61_usize, _61), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: char,mut _3: isize,mut _4: isize,mut _5: u16,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: u16,mut _10: (isize,),mut _11: isize,mut _12: u16,mut _13: isize,mut _14: isize,mut _15: char,mut _16: isize) -> (isize,) {
mir! {
type RET = (isize,);
let _17: u8;
let _18: ([i32; 7], u64);
let _19: char;
let _20: isize;
let _21: i64;
let _22: ();
let _23: ();
{
RET.0 = _11 ^ _13;
_1 = _15;
_14 = _13 + RET.0;
_9 = 194_u8 as u16;
_3 = 606766707_u32 as isize;
_7 = _15 as isize;
_16 = 16159628476692405327_usize as isize;
_1 = _2;
_5 = !_12;
RET.0 = -_11;
_12 = _9 * _9;
_16 = 12645496487257712206_u64 as isize;
_16 = !_13;
_8 = _6;
_15 = _1;
RET = (_14,);
_14 = _6 * RET.0;
_18.1 = 5014491058561822984703964101581401707_u128 as u64;
_10.0 = -RET.0;
_16 = !_4;
_7 = !_16;
_7 = RET.0 - _14;
_16 = false as isize;
RET.0 = _7;
_21 = !(-3825316869588776599_i64);
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(1_usize, 10_usize, Move(_10), 21_usize, Move(_21), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(1_usize, 6_usize, Move(_6), 16_usize, Move(_16), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: (isize,),mut _2: isize,mut _3: (isize,),mut _4: char,mut _5: (isize,),mut _6: isize,mut _7: isize) -> usize {
mir! {
type RET = usize;
let _8: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32));
let _9: [usize; 2];
let _10: u32;
let _11: char;
let _12: i64;
let _13: i16;
let _14: usize;
let _15: Adt45;
let _16: isize;
let _17: [bool; 6];
let _18: u32;
let _19: f64;
let _20: Adt42;
let _21: Adt55;
let _22: usize;
let _23: [i32; 7];
let _24: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32));
let _25: f32;
let _26: [i32; 7];
let _27: (u64, f32, i16, isize);
let _28: Adt44;
let _29: Adt40;
let _30: i8;
let _31: f64;
let _32: Adt52;
let _33: [char; 3];
let _34: i16;
let _35: f64;
let _36: f64;
let _37: f64;
let _38: Adt44;
let _39: isize;
let _40: ([i32; 7], u64);
let _41: Adt44;
let _42: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _43: [char; 7];
let _44: i64;
let _45: ();
let _46: ();
{
_4 = '\u{37cb6}';
_5.0 = _1.0;
_1.0 = _3.0 - _6;
_1.0 = _6;
_8.1.0 = !173954481_u32;
_8.0 = 66_i8 as f32;
Call(_1.0 = fn3(_6, _4, _7, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.1.2 = (-3197827640366677799_i64) as i128;
_8.1.5 = _8.1.0;
_8.1.1 = [(-1572460004_i32),(-1675520678_i32),281232585_i32,1780013616_i32,315484119_i32,(-2096153507_i32),(-1113618164_i32)];
_11 = _4;
_8.1.3.2 = 47437_u16 ^ 27567_u16;
_8.1.1 = [(-1426445454_i32),150307073_i32,1436267468_i32,228284308_i32,1312524014_i32,1389857995_i32,1148143875_i32];
_11 = _4;
_8.1.3.1 = !(-430523310045672655_i64);
Goto(bb2)
}
bb2 = {
_11 = _4;
RET = 17919091668972786250_usize >> _3.0;
RET = 3_usize | 10918234541840873331_usize;
Call(_8.1.3.2 = fn4(_8.1.1, _7, _1, _6, _7, _5.0, _2, _5, _1.0, _6, _5.0, _1, _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _8.1.0 as isize;
_13 = (-13785_i16) * 20895_i16;
_8.1.3 = (false, (-3388447409754731418_i64), 16238_u16);
_4 = _11;
_12 = _8.1.3.1;
_17 = [_8.1.3.0,_8.1.3.0,_8.1.3.0,_8.1.3.0,_8.1.3.0,_8.1.3.0];
_15.fld3 = [_8.1.3.1];
_15.fld2.fld1.3.0 = !_8.1.3.0;
_4 = _11;
_5.0 = 12132137951193670672_u64 as isize;
_8.1.3 = (_15.fld2.fld1.3.0, _12, 19907_u16);
Goto(bb4)
}
bb4 = {
_15.fld1 = _4;
_15.fld2.fld1.3.1 = _8.1.3.2 as i64;
_18 = _8.1.0;
RET = _8.0 as usize;
_10 = _8.1.0 >> _8.1.3.1;
_15.fld2.fld1.5 = !_10;
_15.fld2.fld1.3.0 = !_8.1.3.0;
_8.1.4 = _15.fld3;
_15.fld2.fld1.3 = _8.1.3;
_20 = Adt42::Variant0 { fld0: _8.0 };
_15.fld2.fld1.3.2 = !_8.1.3.2;
SetDiscriminant(_20, 1);
_8.1.4 = [_15.fld2.fld1.3.1];
_15.fld2.fld1.2 = _8.1.2 >> RET;
_14 = RET ^ RET;
_15.fld2.fld1.0 = !_18;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).0 = _15.fld2.fld1.5;
place!(Field::<(isize,)>(Variant(_20, 1), 2)).0 = _14 as isize;
_15.fld1 = _4;
_15.fld2 = Adt43 { fld0: 11203320064957895876_u64,fld1: _8.1 };
place!(Field::<u32>(Variant(_20, 1), 5)) = _13 as u32;
_15.fld2.fld1.3 = _8.1.3;
_9 = [_14,_14];
_1.0 = !_2;
match _8.1.3.2 {
19907 => bb5,
_ => bb2
}
}
bb5 = {
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).3 = _8.1.2 as f32;
place!(Field::<(isize,)>(Variant(_20, 1), 2)).0 = -_2;
_15.fld2.fld1.1 = [326277744_i32,(-1411580533_i32),2068899678_i32,1773410576_i32,(-1035440857_i32),1656518490_i32,1404243035_i32];
_15.fld0 = _8.1.2;
_5 = (_2,);
_24.1.3.2 = !_15.fld2.fld1.3.2;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)) = (_8.0, _4, 147_u8, _8.0);
_15.fld2.fld1.2 = _15.fld0 + _8.1.2;
_24.1.3 = (_8.1.3.0, _12, _15.fld2.fld1.3.2);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).1 = [997238509_i32,45798163_i32,415435393_i32,(-1877324653_i32),1596234341_i32,921718250_i32,1987552647_i32];
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).3 = _8.0 * Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).0;
_23 = [(-935079106_i32),(-1284001374_i32),903570413_i32,1348537266_i32,(-1344214674_i32),(-2112116564_i32),1327049821_i32];
_23 = [486277519_i32,755614111_i32,2031016230_i32,522972336_i32,(-1154841181_i32),(-1143457752_i32),(-21653785_i32)];
_27.3 = _1.0 - _5.0;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).0 = !_10;
_4 = _15.fld1;
_12 = RET as i64;
place!(Field::<*const (bool, i64, u16)>(Variant(_20, 1), 0)) = core::ptr::addr_of!(_15.fld2.fld1.3);
_26 = [220993773_i32,(-2123851267_i32),(-320089500_i32),(-1025081931_i32),(-1312415290_i32),1840401128_i32,(-635859685_i32)];
_10 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).0 << _27.3;
_14 = RET;
Goto(bb6)
}
bb6 = {
_26 = [(-971668287_i32),(-1526451794_i32),(-122024547_i32),434960333_i32,1393530470_i32,(-317643719_i32),(-195712557_i32)];
_4 = _15.fld1;
_27.3 = _15.fld2.fld1.2 as isize;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)) = (_8.0, _15.fld1, 6_u8, _8.0);
_30 = -108_i8;
_3 = (_27.3,);
Call(place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).5 = core::intrinsics::transmute(_10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_24.0 = _8.0;
_3 = (_2,);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3.2 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).2 as u16;
_24 = (_8.0, _15.fld2.fld1);
_27.0 = _15.fld2.fld0;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)) = (_8.0, _15.fld1, 96_u8, _8.0);
_22 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).2 as usize;
_2 = _3.0 - _3.0;
_24.1.3.1 = _15.fld2.fld1.3.1 << _7;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).0 = _10;
RET = _22 * _22;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3 = (_15.fld2.fld1.3.0, _24.1.3.1, _15.fld2.fld1.3.2);
_19 = _30 as f64;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3 = (_15.fld2.fld1.3.0, _24.1.3.1, _8.1.3.2);
_31 = _19;
_27.3 = -_1.0;
_8.1.1 = _15.fld2.fld1.1;
_25 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).3;
Goto(bb8)
}
bb8 = {
_10 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).0;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).0 = _24.0;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).0 = -_25;
_15.fld2 = Adt43 { fld0: _27.0,fld1: _8.1 };
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).0 = -_24.0;
_24.1.0 = _10;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3.2 = _8.1.3.2 ^ _15.fld2.fld1.3.2;
_12 = _2 as i64;
_8.1.1 = [411433967_i32,(-211606570_i32),(-560684824_i32),(-1533742889_i32),190975607_i32,662295615_i32,339872457_i32];
match _15.fld2.fld1.3.2 {
0 => bb9,
1 => bb10,
19907 => bb12,
_ => bb11
}
}
bb9 = {
_24.0 = _8.0;
_3 = (_2,);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3.2 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).2 as u16;
_24 = (_8.0, _15.fld2.fld1);
_27.0 = _15.fld2.fld0;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)) = (_8.0, _15.fld1, 96_u8, _8.0);
_22 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).2 as usize;
_2 = _3.0 - _3.0;
_24.1.3.1 = _15.fld2.fld1.3.1 << _7;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).0 = _10;
RET = _22 * _22;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3 = (_15.fld2.fld1.3.0, _24.1.3.1, _15.fld2.fld1.3.2);
_19 = _30 as f64;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3 = (_15.fld2.fld1.3.0, _24.1.3.1, _8.1.3.2);
_31 = _19;
_27.3 = -_1.0;
_8.1.1 = _15.fld2.fld1.1;
_25 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).3;
Goto(bb8)
}
bb10 = {
_26 = [(-971668287_i32),(-1526451794_i32),(-122024547_i32),434960333_i32,1393530470_i32,(-317643719_i32),(-195712557_i32)];
_4 = _15.fld1;
_27.3 = _15.fld2.fld1.2 as isize;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)) = (_8.0, _15.fld1, 6_u8, _8.0);
_30 = -108_i8;
_3 = (_27.3,);
Call(place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).5 = core::intrinsics::transmute(_10), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_11 = _4;
RET = 17919091668972786250_usize >> _3.0;
RET = 3_usize | 10918234541840873331_usize;
Call(_8.1.3.2 = fn4(_8.1.1, _7, _1, _6, _7, _5.0, _2, _5, _1.0, _6, _5.0, _1, _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_8.1.2 = _27.0 as i128;
_8.1.3.0 = _12 < _12;
_31 = _19;
_27.1 = Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).0 + Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).3;
_1.0 = _3.0 | _3.0;
place!(Field::<(isize,)>(Variant(_20, 1), 2)) = (_5.0,);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).1 = [(-1584299054_i32),1296634189_i32,297358122_i32,2138156631_i32,(-1732342409_i32),(-1820597916_i32),1449004230_i32];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).4 = [_24.1.3.1];
_24.1.2 = _15.fld2.fld0 as i128;
_8.1.3 = (Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).3.0, Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).3.1, _15.fld2.fld1.3.2);
_9 = [_22,RET];
_25 = -_27.1;
_33 = [_15.fld1,_15.fld1,_15.fld1];
_12 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).3.1;
_18 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).5;
_37 = _31;
_24.1 = _8.1;
_37 = 22341869608941714814925849210286502968_u128 as f64;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).0 = 11816065323477419991499814994632245302_u128 as u32;
_24.1.3.2 = !_15.fld2.fld1.3.2;
match _15.fld2.fld0 {
0 => bb9,
1 => bb2,
2 => bb8,
3 => bb13,
4 => bb14,
11203320064957895876 => bb16,
_ => bb15
}
}
bb13 = {
_10 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).0;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).0 = _24.0;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).0 = -_25;
_15.fld2 = Adt43 { fld0: _27.0,fld1: _8.1 };
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).0 = -_24.0;
_24.1.0 = _10;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).3.2 = _8.1.3.2 ^ _15.fld2.fld1.3.2;
_12 = _2 as i64;
_8.1.1 = [411433967_i32,(-211606570_i32),(-560684824_i32),(-1533742889_i32),190975607_i32,662295615_i32,339872457_i32];
match _15.fld2.fld1.3.2 {
0 => bb9,
1 => bb10,
19907 => bb12,
_ => bb11
}
}
bb14 = {
_26 = [(-971668287_i32),(-1526451794_i32),(-122024547_i32),434960333_i32,1393530470_i32,(-317643719_i32),(-195712557_i32)];
_4 = _15.fld1;
_27.3 = _15.fld2.fld1.2 as isize;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)) = (_8.0, _15.fld1, 6_u8, _8.0);
_30 = -108_i8;
_3 = (_27.3,);
Call(place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).5 = core::intrinsics::transmute(_10), ReturnTo(bb7), UnwindUnreachable())
}
bb15 = {
_6 = _8.1.0 as isize;
_13 = (-13785_i16) * 20895_i16;
_8.1.3 = (false, (-3388447409754731418_i64), 16238_u16);
_4 = _11;
_12 = _8.1.3.1;
_17 = [_8.1.3.0,_8.1.3.0,_8.1.3.0,_8.1.3.0,_8.1.3.0,_8.1.3.0];
_15.fld3 = [_8.1.3.1];
_15.fld2.fld1.3.0 = !_8.1.3.0;
_4 = _11;
_5.0 = 12132137951193670672_u64 as isize;
_8.1.3 = (_15.fld2.fld1.3.0, _12, 19907_u16);
Goto(bb4)
}
bb16 = {
_14 = RET;
place!(Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3)).3 = -_25;
_24.1.1 = _15.fld2.fld1.1;
_6 = -_1.0;
_40.1 = _27.0 - _27.0;
_42.5 = !Field::<u32>(Variant(_20, 1), 5);
_24.1.2 = _8.1.2;
_24.1 = (Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).5, _15.fld2.fld1.1, _8.1.2, _8.1.3, Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).4, Field::<u32>(Variant(_20, 1), 5));
_24.1.3 = _8.1.3;
_42.3.1 = _24.1.3.1 & _15.fld2.fld1.3.1;
_42.2 = _24.1.2;
_36 = _37 - _31;
_24.1 = _15.fld2.fld1;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4)).0 = !_10;
_42.3.2 = _24.1.3.2;
_4 = _11;
_24.1.5 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).0 ^ _18;
_40 = (Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).1, _27.0);
_44 = !_8.1.3.1;
_31 = -_36;
_15.fld2.fld1.5 = 24143361831410319820844589856873739683_u128 as u32;
_5.0 = (-1043804185_i32) as isize;
_40 = (Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_20, 1), 4).1, _27.0);
_28 = Adt44::Variant2 { fld0: _10,fld1: Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).1,fld2: Field::<*const (bool, i64, u16)>(Variant(_20, 1), 0),fld3: Field::<(f32, char, u8, f32)>(Variant(_20, 1), 3).2 };
_15.fld2.fld1.3.2 = _42.3.2;
_8.1.3 = (_24.1.3.0, _12, _42.3.2);
Goto(bb17)
}
bb17 = {
Call(_45 = dump_var(2_usize, 1_usize, Move(_1), 14_usize, Move(_14), 44_usize, Move(_44), 40_usize, Move(_40)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(2_usize, 11_usize, Move(_11), 13_usize, Move(_13), 4_usize, Move(_4), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(2_usize, 23_usize, Move(_23), 9_usize, Move(_9), 18_usize, Move(_18), 46_usize, _46), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: char,mut _3: isize,mut _4: (isize,)) -> isize {
mir! {
type RET = isize;
let _5: Adt52;
let _6: (u64, f32, i16, isize);
let _7: isize;
let _8: isize;
let _9: Adt55;
let _10: [i128; 6];
let _11: usize;
let _12: isize;
let _13: ();
let _14: ();
{
RET = 31095_u16 as isize;
_4.0 = _3;
RET = (-99174161233925724099527219591100828320_i128) as isize;
_6.2 = (-374_i16);
_6.1 = 18318979382383632558845860628879717013_u128 as f32;
_1 = -_3;
_6.0 = 17475993541141130795_u64;
Call(_3 = core::intrinsics::transmute(_4.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.3 = -_3;
_4.0 = -_3;
_4.0 = -_1;
_7 = -_6.3;
_6.0 = (-4163179260309564068_i64) as u64;
_4 = (_7,);
_6.0 = _2 as u64;
_7 = -_4.0;
_1 = _6.3 * _7;
_4.0 = _1 & _3;
RET = !_1;
_6.1 = _3 as f32;
_6.3 = _4.0 << _1;
_1 = _6.3;
RET = _6.3 & _4.0;
_8 = _1 & _6.3;
_3 = _2 as isize;
_6.3 = _8;
RET = _4.0;
_6.0 = !13402149112618851282_u64;
_12 = _6.2 as isize;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(3_usize, 12_usize, Move(_12), 2_usize, Move(_2), 8_usize, Move(_8), 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [i32; 7],mut _2: isize,mut _3: (isize,),mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: (isize,),mut _9: isize,mut _10: isize,mut _11: isize,mut _12: (isize,),mut _13: isize) -> u16 {
mir! {
type RET = u16;
let _14: *mut u8;
let _15: *const [i32; 7];
let _16: char;
let _17: ();
let _18: ();
{
RET = 12771163633445349617990589405229907630_u128 as u16;
_3.0 = _7 & _13;
_8 = _12;
_2 = !_7;
_6 = 130_u8 as isize;
_12.0 = (-1749095809_i32) as isize;
_3.0 = !_8.0;
RET = 58808_u16 - 65127_u16;
_9 = -_3.0;
_1 = [112722534_i32,1358220949_i32,1839488690_i32,1719567803_i32,(-1566984813_i32),(-1277388186_i32),(-678240664_i32)];
_2 = -_3.0;
_12.0 = _3.0 ^ _3.0;
_3.0 = !_10;
_13 = _8.0 | _4;
_12 = _3;
_11 = !_9;
_16 = '\u{f4e35}';
RET = 53129_u16 << _9;
_12.0 = _11 * _9;
_5 = _8.0 - _12.0;
_3 = (_5,);
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(4_usize, 13_usize, Move(_13), 12_usize, Move(_12), 7_usize, Move(_7), 16_usize, Move(_16)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(4_usize, 5_usize, Move(_5), 1_usize, Move(_1), 2_usize, Move(_2), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: (isize,),mut _5: (isize,),mut _6: isize,mut _7: isize,mut _8: isize,mut _9: (isize,),mut _10: isize,mut _11: isize) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _12: [char; 7];
let _13: isize;
let _14: u64;
let _15: bool;
let _16: (f32, char, u8, f32);
let _17: Adt48;
let _18: bool;
let _19: u8;
let _20: [usize; 2];
let _21: Adt42;
let _22: i32;
let _23: Adt50;
let _24: isize;
let _25: (f32, char, u8, f32);
let _26: [i128; 6];
let _27: Adt48;
let _28: *const [i32; 7];
let _29: char;
let _30: isize;
let _31: isize;
let _32: u128;
let _33: bool;
let _34: Adt41;
let _35: Adt49;
let _36: ();
let _37: ();
{
_2 = _7 << _5.0;
RET = [(-570171289_i32),4292888_i32,(-603454152_i32),(-266486293_i32),(-1494745885_i32),(-1992323427_i32),1690388706_i32];
_6 = _2 + _5.0;
_4 = (_6,);
_2 = _7;
_12 = ['\u{6d078}','\u{743f5}','\u{10dbe2}','\u{cccba}','\u{c5dc2}','\u{dfe6e}','\u{29390}'];
_7 = _10;
RET = [(-895177732_i32),(-1177223509_i32),(-647051664_i32),(-119014755_i32),647065442_i32,978509072_i32,808442416_i32];
RET = [(-570627152_i32),(-1138293688_i32),36110987_i32,(-566645362_i32),724937331_i32,(-1135103622_i32),(-390170101_i32)];
_9 = (_1,);
RET = [(-222245693_i32),(-188222792_i32),1918094539_i32,1493872944_i32,(-2062447125_i32),1565837459_i32,233249274_i32];
_5.0 = !_2;
_12 = ['\u{14f78}','\u{a544d}','\u{4e71}','\u{b66f5}','\u{fe9c9}','\u{cbb0e}','\u{7c48d}'];
_4 = (_6,);
_6 = -_3;
Goto(bb1)
}
bb1 = {
_9 = (_5.0,);
_9.0 = '\u{e0714}' as isize;
_4 = _5;
Goto(bb2)
}
bb2 = {
_5 = _4;
_8 = 220066902260160811604011481890369590932_u128 as isize;
_7 = _2 - _6;
_2 = _5.0 | _4.0;
_12 = ['\u{27ff5}','\u{1559b}','\u{46998}','\u{5a7e2}','\u{3321c}','\u{b8321}','\u{1015c4}'];
_3 = (-36_i8) as isize;
_12 = ['\u{8e3e9}','\u{32872}','\u{cb34e}','\u{bcc5e}','\u{2e048}','\u{6e3af}','\u{f404f}'];
_3 = -_7;
_12 = ['\u{4fd21}','\u{24340}','\u{15b46}','\u{711e0}','\u{d24cf}','\u{f4f3b}','\u{35b7e}'];
_8 = _7;
_7 = !_8;
_1 = !_5.0;
_4.0 = _2;
RET = [379157394_i32,(-739163846_i32),1232176522_i32,(-1410453627_i32),684831868_i32,1193443789_i32,(-913130627_i32)];
_8 = -_1;
_1 = 114_u8 as isize;
_13 = 186_u8 as isize;
_15 = false ^ false;
Goto(bb3)
}
bb3 = {
_2 = 32983_u16 as isize;
_13 = -_3;
_4.0 = 172_u8 as isize;
_4 = (_3,);
_5.0 = _7 >> _8;
_14 = (-7510494354350626672_i64) as u64;
_4 = (_13,);
RET = [263427287_i32,(-878751495_i32),(-567809137_i32),328145179_i32,1983479584_i32,(-1825507123_i32),(-1452293646_i32)];
_12 = ['\u{1b9b7}','\u{c8f5c}','\u{8edb2}','\u{ea73a}','\u{9dc00}','\u{62b09}','\u{638c9}'];
_14 = 3971443375046135720_u64;
_2 = _4.0 - _5.0;
_10 = !_13;
_16.1 = '\u{ff500}';
_11 = 876422271_i32 as isize;
_16.2 = (-94809396054699401848088896463624244451_i128) as u8;
_8 = _3 + _5.0;
RET = [538161823_i32,(-816144095_i32),1785542963_i32,1744420962_i32,(-1895622801_i32),(-828778603_i32),1815117434_i32];
_16.0 = 2_usize as f32;
_16.3 = -_16.0;
_14 = 5947897710343221023_u64;
Goto(bb4)
}
bb4 = {
_4 = _5;
_9.0 = _13;
_12 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
_11 = _16.1 as isize;
_11 = (-4059647940597511558_i64) as isize;
_19 = !_16.2;
_4 = (_8,);
_16.0 = _16.3;
_4.0 = _8;
_10 = _13;
_7 = !_2;
_18 = !_15;
_9.0 = _2 * _2;
_12 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
_6 = _9.0 >> _2;
_16.2 = _19;
_1 = -_4.0;
_1 = _13;
_21 = Adt42::Variant0 { fld0: _16.3 };
_20 = [4214095032880131255_usize,3278626762819883712_usize];
_6 = _2;
Call(_7 = fn6(_4.0, _9, _5, _10, _8, _5, _1, _2, _5, _9.0, _3, _8, _10, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = [13718106324366253952_usize,15135966311038200424_usize];
_16.3 = (-35_i8) as f32;
_16.2 = _19 >> _13;
_16.1 = '\u{2e788}';
RET = [(-1361431937_i32),66510974_i32,458752183_i32,(-521506207_i32),(-1819987897_i32),(-1362460432_i32),(-50825936_i32)];
_16 = (Field::<f32>(Variant(_21, 0), 0), '\u{8abe8}', _19, Field::<f32>(Variant(_21, 0), 0));
_4 = _5;
_10 = _1;
_8 = !_10;
_19 = _16.2 << _7;
_2 = (-62458121336334016181906673307432644501_i128) as isize;
match _14 {
0 => bb3,
5947897710343221023 => bb7,
_ => bb6
}
}
bb6 = {
_2 = 32983_u16 as isize;
_13 = -_3;
_4.0 = 172_u8 as isize;
_4 = (_3,);
_5.0 = _7 >> _8;
_14 = (-7510494354350626672_i64) as u64;
_4 = (_13,);
RET = [263427287_i32,(-878751495_i32),(-567809137_i32),328145179_i32,1983479584_i32,(-1825507123_i32),(-1452293646_i32)];
_12 = ['\u{1b9b7}','\u{c8f5c}','\u{8edb2}','\u{ea73a}','\u{9dc00}','\u{62b09}','\u{638c9}'];
_14 = 3971443375046135720_u64;
_2 = _4.0 - _5.0;
_10 = !_13;
_16.1 = '\u{ff500}';
_11 = 876422271_i32 as isize;
_16.2 = (-94809396054699401848088896463624244451_i128) as u8;
_8 = _3 + _5.0;
RET = [538161823_i32,(-816144095_i32),1785542963_i32,1744420962_i32,(-1895622801_i32),(-828778603_i32),1815117434_i32];
_16.0 = 2_usize as f32;
_16.3 = -_16.0;
_14 = 5947897710343221023_u64;
Goto(bb4)
}
bb7 = {
_24 = 5_i8 as isize;
_2 = !_9.0;
_12 = [_16.1,_16.1,_16.1,_16.1,_16.1,_16.1,_16.1];
RET = [(-877346619_i32),495392606_i32,1308816637_i32,(-1423166466_i32),344625051_i32,(-937038079_i32),579848766_i32];
_16.2 = !_19;
_4.0 = -_6;
_15 = !_18;
_16.3 = Field::<f32>(Variant(_21, 0), 0) - Field::<f32>(Variant(_21, 0), 0);
_25 = _16;
_22 = 882621644_i32 * 583308640_i32;
RET = [_22,_22,_22,_22,_22,_22,_22];
_16.1 = _25.1;
_18 = _15 | _15;
_16.2 = _14 as u8;
_25.2 = !_19;
_3 = _9.0;
RET = [_22,_22,_22,_22,_22,_22,_22];
_19 = _25.2 | _25.2;
_25.2 = !_19;
_7 = _2;
_16.3 = -Field::<f32>(Variant(_21, 0), 0);
_16.3 = -_25.0;
_5 = _4;
_2 = 48_i8 as isize;
_7 = _3 | _6;
_24 = _9.0 ^ _9.0;
_10 = -_4.0;
Call(_24 = core::intrinsics::transmute(_10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_26 = [108451915816426039678141543638324798923_i128,(-93089310298576607661818519257241884907_i128),25811326412265992833702699431324446528_i128,146433983718221665172090151291564396763_i128,(-16954374748388411989829716315365436221_i128),(-94749363759789810331488292206226495923_i128)];
SetDiscriminant(_21, 2);
_18 = _4.0 == _8;
RET = [_22,_22,_22,_22,_22,_22,_22];
RET = [_22,_22,_22,_22,_22,_22,_22];
_19 = 537_u16 as u8;
_25.2 = _19;
_20 = [0_usize,12701281700225939878_usize];
_1 = _9.0 * _3;
_16.3 = _25.0;
_24 = 3198087520_u32 as isize;
_9.0 = _7;
_28 = core::ptr::addr_of!(RET);
_25.1 = _16.1;
_1 = _3;
_14 = 40230_u16 as u64;
_15 = !_18;
_1 = _5.0;
_15 = _18 == _18;
Goto(bb9)
}
bb9 = {
_20 = [4_usize,4_usize];
_13 = -_4.0;
Call(_21 = fn7(_4, _10, _5.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<f32>(Variant(_21, 0), 0)) = (-61_i8) as f32;
_20 = [1_usize,3_usize];
RET = [_22,_22,_22,_22,_22,_22,_22];
_6 = _19 as isize;
_16.2 = _25.2;
_22 = -(-49398440_i32);
_16.0 = (-88_i8) as f32;
_19 = !_25.2;
_16.3 = _16.0 * _25.0;
_16.0 = -_25.3;
_24 = _22 as isize;
_16 = _25;
_16.1 = _25.1;
place!(Field::<f32>(Variant(_21, 0), 0)) = -_25.3;
place!(Field::<f32>(Variant(_21, 0), 0)) = 2565533585_u32 as f32;
_6 = _4.0;
_16 = _25;
_3 = _22 as isize;
_16.0 = -Field::<f32>(Variant(_21, 0), 0);
_5.0 = -_10;
_30 = -_5.0;
_20 = [14048189294589100388_usize,7616514327821212455_usize];
_11 = !_4.0;
SetDiscriminant(_21, 1);
place!(Field::<(isize,)>(Variant(_21, 1), 2)) = _9;
Goto(bb11)
}
bb11 = {
_1 = _18 as isize;
_5 = _9;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_21, 1), 4)).2 = 76915629939684416080983896860282433044_i128;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_21, 1), 4)).3.2 = !32626_u16;
place!(Field::<*const (bool, i64, u16)>(Variant(_21, 1), 0)) = core::ptr::addr_of!(place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_21, 1), 4)).3);
place!(Field::<(f32, char, u8, f32)>(Variant(_21, 1), 3)).0 = 16849048259040529622_usize as f32;
match Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(_21, 1), 4).2 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb12,
76915629939684416080983896860282433044 => bb14,
_ => bb13
}
}
bb12 = {
_20 = [13718106324366253952_usize,15135966311038200424_usize];
_16.3 = (-35_i8) as f32;
_16.2 = _19 >> _13;
_16.1 = '\u{2e788}';
RET = [(-1361431937_i32),66510974_i32,458752183_i32,(-521506207_i32),(-1819987897_i32),(-1362460432_i32),(-50825936_i32)];
_16 = (Field::<f32>(Variant(_21, 0), 0), '\u{8abe8}', _19, Field::<f32>(Variant(_21, 0), 0));
_4 = _5;
_10 = _1;
_8 = !_10;
_19 = _16.2 << _7;
_2 = (-62458121336334016181906673307432644501_i128) as isize;
match _14 {
0 => bb3,
5947897710343221023 => bb7,
_ => bb6
}
}
bb13 = {
_20 = [4_usize,4_usize];
_13 = -_4.0;
Call(_21 = fn7(_4, _10, _5.0), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
_16.3 = -_25.3;
_9.0 = -_1;
_21 = Adt42::Variant0 { fld0: _25.3 };
_29 = _25.1;
_32 = 309974105512592519149249268037536021731_u128;
_26 = [167394810683083629136229291685612957122_i128,(-75355636895291760919195574071247310332_i128),(-118180725090056845909367272701746156100_i128),6096734957924538069626724719741500837_i128,68814489791665727792487695271658409051_i128,(-109050203530087997907242947671626235838_i128)];
_33 = _11 > _6;
RET = [_22,_22,_22,_22,_22,_22,_22];
RET = [_22,_22,_22,_22,_22,_22,_22];
_10 = -_7;
_16.2 = !_25.2;
_31 = 46377865853288205977679741038628756153_i128 as isize;
_19 = !_25.2;
_30 = _6;
_6 = -_13;
_5.0 = 11_i8 as isize;
_6 = _8;
_28 = core::ptr::addr_of!(RET);
_12 = [_29,_29,_16.1,_25.1,_25.1,_29,_16.1];
_1 = _10;
_16.1 = _29;
_16.2 = _25.2 ^ _25.2;
_28 = core::ptr::addr_of!(RET);
_5 = (_6,);
_6 = _11;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(5_usize, 14_usize, Move(_14), 15_usize, Move(_15), 31_usize, Move(_31), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(5_usize, 22_usize, Move(_22), 2_usize, Move(_2), 20_usize, Move(_20), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(5_usize, 30_usize, Move(_30), 13_usize, Move(_13), 18_usize, Move(_18), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(5_usize, 8_usize, Move(_8), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: (isize,),mut _3: (isize,),mut _4: isize,mut _5: isize,mut _6: (isize,),mut _7: isize,mut _8: isize,mut _9: (isize,),mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: (isize,)) -> isize {
mir! {
type RET = isize;
let _15: i128;
let _16: (u64, f32, i16, isize);
let _17: char;
let _18: i64;
let _19: ();
let _20: ();
{
_3.0 = -_11;
_6 = (_5,);
_14.0 = -_1;
_8 = !_1;
_14.0 = !_5;
Goto(bb1)
}
bb1 = {
_6.0 = !_14.0;
RET = _2.0;
RET = _5;
_16.3 = (-45_i8) as isize;
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(6_usize, 7_usize, Move(_7), 10_usize, Move(_10), 4_usize, Move(_4), 14_usize, Move(_14)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(6_usize, 6_usize, Move(_6), 11_usize, Move(_11), 12_usize, Move(_12), 20_usize, _20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: (isize,),mut _2: isize,mut _3: isize) -> Adt42 {
mir! {
type RET = Adt42;
let _4: (i16,);
let _5: i64;
let _6: Adt46;
let _7: ([i32; 7], u64);
let _8: bool;
let _9: (f32, char, u8, f32);
let _10: [usize; 2];
let _11: (i16,);
let _12: (*mut u8, [char; 7]);
let _13: isize;
let _14: i64;
let _15: f32;
let _16: Adt41;
let _17: Adt56;
let _18: *mut u8;
let _19: [i64; 1];
let _20: char;
let _21: f64;
let _22: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _23: u32;
let _24: Adt48;
let _25: (*mut u8, [char; 7]);
let _26: i8;
let _27: isize;
let _28: (isize,);
let _29: (bool, i64, u16);
let _30: char;
let _31: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32));
let _32: [i32; 7];
let _33: (i16,);
let _34: char;
let _35: [i32; 7];
let _36: bool;
let _37: f32;
let _38: i8;
let _39: ();
let _40: ();
{
_1.0 = _3;
_2 = _1.0 ^ _1.0;
_1 = (_3,);
_3 = _1.0;
_1 = (_3,);
_4 = ((-23596_i16),);
_2 = _1.0 | _1.0;
_4.0 = 1112022312_i32 as i16;
_4 = ((-24113_i16),);
_2 = 128_u8 as isize;
_1.0 = _3;
_1 = (_3,);
_2 = _3;
_5 = -3925563828475051990_i64;
_1.0 = 3353103596_u32 as isize;
_5 = (-145076800949513350_i64) + (-4859451949008349659_i64);
_1 = (_3,);
_2 = (-106_i8) as isize;
Call(_4.0 = fn8(_1.0, _1, _1.0, _3, _3, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _3 - _1.0;
_2 = _3 * _1.0;
_3 = _2 << _2;
_1 = (_3,);
_1.0 = _2;
_1.0 = '\u{18e31}' as isize;
_1.0 = 54_i8 as isize;
_4 = ((-14622_i16),);
_3 = 10207056259535180473210733034287077161_i128 as isize;
_5 = (-1032344943301640111_i64) * (-8215018233904766649_i64);
_1.0 = !_2;
_4 = ((-19748_i16),);
_1 = (_2,);
_1.0 = !_2;
_3 = !_2;
_5 = !(-1091955744156118047_i64);
_1.0 = _3 ^ _2;
_1.0 = _2 & _3;
_4.0 = (-26344_i16) << _3;
_5 = !5469581536802938762_i64;
_4 = (9792_i16,);
_7.0 = [(-1993973804_i32),(-752106517_i32),1486091172_i32,(-1091022919_i32),1042304563_i32,119656802_i32,2032362917_i32];
Goto(bb2)
}
bb2 = {
_9.0 = _4.0 as f32;
_4.0 = _9.0 as i16;
_1 = (_3,);
RET = Adt42::Variant0 { fld0: _9.0 };
_9 = (Field::<f32>(Variant(RET, 0), 0), '\u{9fc56}', 149_u8, Field::<f32>(Variant(RET, 0), 0));
RET = Adt42::Variant0 { fld0: _9.0 };
_7.1 = 2076587682_i32 as u64;
_8 = true & false;
_9.2 = !176_u8;
_9 = (Field::<f32>(Variant(RET, 0), 0), '\u{fcfa5}', 111_u8, Field::<f32>(Variant(RET, 0), 0));
_9.0 = -Field::<f32>(Variant(RET, 0), 0);
Goto(bb3)
}
bb3 = {
_9.3 = _9.0;
_5 = !1322015104197509993_i64;
_5 = 8547584914678511373_i64 * (-2808222119689153040_i64);
_7.1 = (-98272417485872681242148123009690895303_i128) as u64;
_8 = _1.0 > _3;
SetDiscriminant(RET, 1);
_9.1 = '\u{97522}';
_5 = (-9065895869814600360_i64);
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).2 = 21949_u16 as u8;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).0 = _9.3 + _9.3;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.1 = _5;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)) = _9;
place!(Field::<*const (bool, i64, u16)>(Variant(RET, 1), 0)) = core::ptr::addr_of!(place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).2 = (-148661399075827654735658925683382051347_i128);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).4 = [_5];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).2 = !(-56533327039639598742764094068849211128_i128);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).5 = 1612040678_u32 >> _3;
_10 = [10098940311889125366_usize,16211488125588044046_usize];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).5 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).2 as u32;
place!(Field::<u32>(Variant(RET, 1), 5)) = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).2 as u32;
_13 = _3;
place!(Field::<(isize,)>(Variant(RET, 1), 2)) = _1;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.0 = _8;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.1 = _5;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).0 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).5 - Field::<u32>(Variant(RET, 1), 5);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).1 = [221014069_i32,(-1696470949_i32),1497312676_i32,360818713_i32,1768188961_i32,898516899_i32,805551001_i32];
_3 = _13 & _1.0;
match Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).3.1 {
340282366920938463454308711561953611096 => bb4,
_ => bb2
}
}
bb4 = {
place!(Field::<u32>(Variant(RET, 1), 5)) = 19791_u16 as u32;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).1 = [(-282000081_i32),(-34295062_i32),(-1695713247_i32),(-463309285_i32),908620336_i32,(-241707165_i32),(-195288022_i32)];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.2 = 53407_u16 & 51750_u16;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).4 = [_5];
_11.0 = _9.3 as i16;
_12.1 = [_9.1,Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3).1,_9.1,_9.1,Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3).1,_9.1,_9.1];
place!(Field::<(isize,)>(Variant(RET, 1), 2)) = _1;
_15 = Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3).3 * Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3).0;
_11 = _4;
_12.0 = core::ptr::addr_of_mut!(place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).2);
_4 = (_11.0,);
_7.1 = !16554046850099841330_u64;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3 = (_8, _5, 15380_u16);
_1.0 = _13;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)) = _9;
place!(Field::<(isize,)>(Variant(RET, 1), 2)).0 = _13 - _13;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).0 = Field::<u32>(Variant(RET, 1), 5) << Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).3.2;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).1 = _9.1;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).4 = [_5];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).4 = [_5];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.2 = 21943_u16;
match _9.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
111 => bb8,
_ => bb7
}
}
bb5 = {
_9.3 = _9.0;
_5 = !1322015104197509993_i64;
_5 = 8547584914678511373_i64 * (-2808222119689153040_i64);
_7.1 = (-98272417485872681242148123009690895303_i128) as u64;
_8 = _1.0 > _3;
SetDiscriminant(RET, 1);
_9.1 = '\u{97522}';
_5 = (-9065895869814600360_i64);
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).2 = 21949_u16 as u8;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).0 = _9.3 + _9.3;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.1 = _5;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)) = _9;
place!(Field::<*const (bool, i64, u16)>(Variant(RET, 1), 0)) = core::ptr::addr_of!(place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).2 = (-148661399075827654735658925683382051347_i128);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).4 = [_5];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).2 = !(-56533327039639598742764094068849211128_i128);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).5 = 1612040678_u32 >> _3;
_10 = [10098940311889125366_usize,16211488125588044046_usize];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).5 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).2 as u32;
place!(Field::<u32>(Variant(RET, 1), 5)) = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).2 as u32;
_13 = _3;
place!(Field::<(isize,)>(Variant(RET, 1), 2)) = _1;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.0 = _8;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.1 = _5;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).0 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).5 - Field::<u32>(Variant(RET, 1), 5);
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).1 = [221014069_i32,(-1696470949_i32),1497312676_i32,360818713_i32,1768188961_i32,898516899_i32,805551001_i32];
_3 = _13 & _1.0;
match Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).3.1 {
340282366920938463454308711561953611096 => bb4,
_ => bb2
}
}
bb6 = {
_9.0 = _4.0 as f32;
_4.0 = _9.0 as i16;
_1 = (_3,);
RET = Adt42::Variant0 { fld0: _9.0 };
_9 = (Field::<f32>(Variant(RET, 0), 0), '\u{9fc56}', 149_u8, Field::<f32>(Variant(RET, 0), 0));
RET = Adt42::Variant0 { fld0: _9.0 };
_7.1 = 2076587682_i32 as u64;
_8 = true & false;
_9.2 = !176_u8;
_9 = (Field::<f32>(Variant(RET, 0), 0), '\u{fcfa5}', 111_u8, Field::<f32>(Variant(RET, 0), 0));
_9.0 = -Field::<f32>(Variant(RET, 0), 0);
Goto(bb3)
}
bb7 = {
_2 = _3 - _1.0;
_2 = _3 * _1.0;
_3 = _2 << _2;
_1 = (_3,);
_1.0 = _2;
_1.0 = '\u{18e31}' as isize;
_1.0 = 54_i8 as isize;
_4 = ((-14622_i16),);
_3 = 10207056259535180473210733034287077161_i128 as isize;
_5 = (-1032344943301640111_i64) * (-8215018233904766649_i64);
_1.0 = !_2;
_4 = ((-19748_i16),);
_1 = (_2,);
_1.0 = !_2;
_3 = !_2;
_5 = !(-1091955744156118047_i64);
_1.0 = _3 ^ _2;
_1.0 = _2 & _3;
_4.0 = (-26344_i16) << _3;
_5 = !5469581536802938762_i64;
_4 = (9792_i16,);
_7.0 = [(-1993973804_i32),(-752106517_i32),1486091172_i32,(-1091022919_i32),1042304563_i32,119656802_i32,2032362917_i32];
Goto(bb2)
}
bb8 = {
_1.0 = Field::<(isize,)>(Variant(RET, 1), 2).0 >> _2;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).2 = _9.2;
_9.2 = Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3).2 + Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3).2;
_1.0 = _2 >> _3;
_1.0 = !_3;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).5 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).0 * Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).0;
_4.0 = -_11.0;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).4 = [Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).3.1];
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).3.2 = 45963_u16;
_11 = (_4.0,);
_7 = (Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).1, 17828530349924685248_u64);
_4 = _11;
_9.0 = _9.3 + _15;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).1 = _9.1;
_5 = -Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).3.1;
_4.0 = _11.0 * _11.0;
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).2 = 108293952034361862276248344236263773249_i128;
_3 = Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4).3.2 as isize;
Goto(bb9)
}
bb9 = {
place!(Field::<(u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)>(Variant(RET, 1), 4)).2 = 150190865463019284137562565187689505291_i128 + 163886495627799043452053886157457057847_i128;
place!(Field::<(f32, char, u8, f32)>(Variant(RET, 1), 3)).0 = _9.0;
RET = Adt42::Variant0 { fld0: _9.0 };
_7.0 = [1353639938_i32,(-1757989370_i32),(-451311685_i32),174225982_i32,189382825_i32,(-1083970056_i32),1656966390_i32];
_14 = !_5;
_14 = _5 ^ _5;
_8 = !true;
_9.3 = Field::<f32>(Variant(RET, 0), 0);
_19 = [_14];
_15 = -Field::<f32>(Variant(RET, 0), 0);
_13 = Field::<f32>(Variant(RET, 0), 0) as isize;
_7.0 = [1288251394_i32,(-84592739_i32),(-1701294388_i32),1111635162_i32,(-1920669110_i32),(-1487953839_i32),537089116_i32];
_22.1 = [(-1707332659_i32),(-1608785713_i32),(-129584538_i32),(-290020940_i32),915364905_i32,1544033788_i32,95640338_i32];
place!(Field::<f32>(Variant(RET, 0), 0)) = _9.3 - _15;
place!(Field::<f32>(Variant(RET, 0), 0)) = -_15;
_13 = _2;
_22.2 = (-153571437106073401921304252381135618675_i128);
_20 = _9.1;
_8 = !true;
_22.3.2 = !51864_u16;
_4.0 = _11.0;
_21 = _14 as f64;
_8 = _2 <= _1.0;
_2 = _1.0 ^ _13;
_1 = (_2,);
Goto(bb10)
}
bb10 = {
_19 = [_5];
_22.3 = (_8, _14, 44328_u16);
_23 = !3700245751_u32;
_23 = _21 as u32;
_22.3.1 = _9.2 as i64;
_12.0 = core::ptr::addr_of_mut!(_9.2);
_22.4 = _19;
_3 = _13;
_9.3 = _11.0 as f32;
_9.2 = 228_u8 | 101_u8;
SetDiscriminant(RET, 2);
_22.5 = !_23;
_3 = !_2;
_11 = _4;
_20 = _9.1;
_22.3.1 = _14 - _5;
_15 = _9.0;
Goto(bb11)
}
bb11 = {
_16 = Adt41::Variant0 { fld0: _21,fld1: _9.1,fld2: _22.3.1,fld3: _22.4,fld4: _22.2 };
_12.1 = [Field::<char>(Variant(_16, 0), 1),Field::<char>(Variant(_16, 0), 1),_20,_20,Field::<char>(Variant(_16, 0), 1),_20,_9.1];
SetDiscriminant(_16, 1);
_2 = _3 | _3;
_28 = (_13,);
_8 = !_22.3.0;
place!(Field::<(bool, i64, u16)>(Variant(_16, 1), 6)).0 = !_22.3.0;
Goto(bb12)
}
bb12 = {
_22.4 = [_22.3.1];
_30 = _20;
_31.1.0 = _23 * _23;
place!(Field::<(bool, i64, u16)>(Variant(_16, 1), 6)).2 = !_22.3.2;
_9 = (_15, _20, 198_u8, _15);
_31.1.3.1 = 249131766299411415799845300244173410694_u128 as i64;
RET = Adt42::Variant0 { fld0: _15 };
_31.1.3.1 = _22.3.1;
_26 = (-123_i8) << _1.0;
_31.1.3.0 = !_22.3.0;
match _9.2 {
0 => bb2,
198 => bb14,
_ => bb13
}
}
bb13 = {
_9.0 = _4.0 as f32;
_4.0 = _9.0 as i16;
_1 = (_3,);
RET = Adt42::Variant0 { fld0: _9.0 };
_9 = (Field::<f32>(Variant(RET, 0), 0), '\u{9fc56}', 149_u8, Field::<f32>(Variant(RET, 0), 0));
RET = Adt42::Variant0 { fld0: _9.0 };
_7.1 = 2076587682_i32 as u64;
_8 = true & false;
_9.2 = !176_u8;
_9 = (Field::<f32>(Variant(RET, 0), 0), '\u{fcfa5}', 111_u8, Field::<f32>(Variant(RET, 0), 0));
_9.0 = -Field::<f32>(Variant(RET, 0), 0);
Goto(bb3)
}
bb14 = {
_18 = core::ptr::addr_of_mut!(_9.2);
_31.1.3.0 = !Field::<(bool, i64, u16)>(Variant(_16, 1), 6).0;
_22.0 = 338644925293914759694086523782863776509_u128 as u32;
_29.0 = !_22.3.0;
_4 = (_11.0,);
place!(Field::<u32>(Variant(_16, 1), 3)) = 193403103_i32 as u32;
SetDiscriminant(RET, 0);
_31 = (_15, _22);
_15 = _31.0;
place!(Field::<f32>(Variant(RET, 0), 0)) = -_15;
_29.1 = _22.3.1 * _5;
_25.1 = [_9.1,_9.1,_20,_30,_9.1,_30,_20];
_22.2 = _31.1.2 | _31.1.2;
_9.0 = _31.1.3.2 as f32;
_22.3.1 = _22.3.0 as i64;
_1.0 = _4.0 as isize;
_17 = Adt56::Variant2 { fld0: _18 };
place!(Field::<(bool, i64, u16)>(Variant(_16, 1), 6)).2 = _22.3.2;
place!(Field::<f32>(Variant(RET, 0), 0)) = _9.0;
_22.4 = _31.1.4;
_31.1.3.2 = _21 as u16;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(7_usize, 23_usize, Move(_23), 14_usize, Move(_14), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(7_usize, 22_usize, Move(_22), 19_usize, Move(_19), 28_usize, Move(_28), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(7_usize, 3_usize, Move(_3), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: (isize,),mut _3: isize,mut _4: isize,mut _5: isize,mut _6: (isize,),mut _7: (isize,)) -> i16 {
mir! {
type RET = i16;
let _8: *const [i32; 7];
let _9: i8;
let _10: [i32; 7];
let _11: bool;
let _12: *const (bool, i64, u16);
let _13: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _14: char;
let _15: u8;
let _16: bool;
let _17: Adt53;
let _18: isize;
let _19: u64;
let _20: [isize; 8];
let _21: [isize; 8];
let _22: (i16,);
let _23: bool;
let _24: char;
let _25: isize;
let _26: i32;
let _27: Adt43;
let _28: Adt48;
let _29: i64;
let _30: [i128; 6];
let _31: *mut (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _32: f64;
let _33: Adt46;
let _34: isize;
let _35: isize;
let _36: f64;
let _37: u8;
let _38: char;
let _39: u64;
let _40: u16;
let _41: u64;
let _42: Adt47;
let _43: (*mut u8, [char; 7]);
let _44: ();
let _45: ();
{
_7.0 = _1;
_6.0 = _2.0;
_2.0 = 11500743994738351156_u64 as isize;
_5 = 13071668458829490948_u64 as isize;
_2 = (_1,);
RET = (-21875_i16);
_6 = (_4,);
Call(_5 = core::intrinsics::transmute(_7.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _7;
_6.0 = !_3;
RET = 777165245532451339935454667780133553_i128 as i16;
_7.0 = -_6.0;
_2.0 = _5;
Goto(bb2)
}
bb2 = {
_2.0 = _7.0 >> _6.0;
_7 = _2;
_7 = (_1,);
_4 = _1 ^ _5;
_9 = RET as i8;
_6.0 = -_7.0;
_8 = core::ptr::addr_of!(_10);
_10 = [325366028_i32,1098554758_i32,(-2050691357_i32),(-2085677421_i32),(-1738366685_i32),950892326_i32,(-539044788_i32)];
_1 = _5 << _4;
_11 = true;
RET = 30609_i16 ^ 24893_i16;
_1 = _5;
_3 = 76_u8 as isize;
_4 = _1;
_8 = core::ptr::addr_of!((*_8));
_2.0 = !_7.0;
_7 = (_5,);
_6 = _2;
_7 = (_2.0,);
_7.0 = -_4;
Call(_2.0 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13.5 = 417911322_u32 & 2899938658_u32;
_2 = (_1,);
_13.2 = (-150526358127469503540483469463989396113_i128);
_13.5 = 47771_u16 as u32;
_12 = core::ptr::addr_of!(_13.3);
_12 = core::ptr::addr_of!((*_12));
_2 = (_7.0,);
(*_8) = [965262919_i32,200127387_i32,521410687_i32,(-1767975328_i32),1355186279_i32,(-1979778208_i32),1139489793_i32];
(*_12) = (_11, 7054863371690545811_i64, 53294_u16);
_13.3 = (_11, 6945168558303201985_i64, 57784_u16);
(*_12).1 = 8267136728450975613_i64 & 2222007025014321964_i64;
_13.4 = [_13.3.1];
_13.1 = (*_8);
(*_12).1 = 5436338743673833565_i64 >> _2.0;
RET = 28457_i16;
_13.3.2 = !27782_u16;
RET = 31588_i16;
_11 = _13.3.0;
_14 = '\u{feda8}';
(*_12).0 = _11;
_13.3 = (_11, (-1637204628443796021_i64), 61351_u16);
_13.2 = 291206703065895946890578073523756733947_u128 as i128;
_13.1 = [1327179407_i32,(-1896187672_i32),1781328178_i32,1920351947_i32,799671078_i32,729844461_i32,(-1454951250_i32)];
_13.5 = !3496059911_u32;
_7 = (_4,);
_9 = !(-102_i8);
Call((*_12).0 = fn9(_4, _2, _7, _7.0, _6, _2.0, _7, _6.0, _6.0, _5, _4, _7.0, _7.0, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_12).0 = _11;
_13.4 = [_13.3.1];
_5 = _13.5 as isize;
_8 = core::ptr::addr_of!(_10);
_13.4 = [(*_12).1];
_7.0 = -_6.0;
(*_8) = _13.1;
_7.0 = _4;
_20 = [_7.0,_7.0,_1,_6.0,_1,_4,_2.0,_1];
(*_8) = [1031354313_i32,1159145527_i32,(-2068673054_i32),(-1224275821_i32),(-2113080712_i32),2117401522_i32,1426177515_i32];
_10 = _13.1;
(*_8) = [1391745560_i32,724466631_i32,87710333_i32,(-511115743_i32),827789851_i32,1962469808_i32,702397622_i32];
_3 = !_4;
_18 = _2.0 - _4;
_21 = [_6.0,_1,_7.0,_7.0,_6.0,_18,_1,_6.0];
_13.2 = !52778528962744143713732576804312952587_i128;
_13.3 = (_11, 6385417153220819572_i64, 34639_u16);
_13.3.1 = _13.3.0 as i64;
_6.0 = _1;
_12 = core::ptr::addr_of!((*_12));
_3 = _7.0;
_13.5 = 579675866_u32;
_23 = _13.3.0;
_13.3 = (_11, 6805270698194585387_i64, 47802_u16);
_15 = !173_u8;
_22 = (RET,);
Goto(bb5)
}
bb5 = {
_19 = _15 as u64;
_13.5 = 173702518308082682413335794403397651086_u128 as u32;
_19 = _2.0 as u64;
_2.0 = -_6.0;
_19 = 5069748623867237261_u64;
_6 = (_1,);
_21 = _20;
_14 = '\u{18653}';
_18 = _1;
_13.1 = (*_8);
_12 = core::ptr::addr_of!((*_12));
_13.5 = _14 as u32;
_13.5 = 2945715933_u32;
_13.0 = _13.5;
RET = _22.0;
(*_12).1 = (-647007854438966465_i64) - (-8353002592894035569_i64);
_13.3.2 = 7281_u16;
Call(_14 = fn10(_18, _20, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_23 = (*_12).0 ^ (*_12).0;
(*_12).2 = _13.5 as u16;
(*_8) = [672240661_i32,2103870117_i32,(-1565249815_i32),1334957_i32,1146565761_i32,(-334661819_i32),(-1626250717_i32)];
(*_12).1 = 3291821154617920887_i64 >> _7.0;
_10 = [(-1031778970_i32),1293763110_i32,586220706_i32,(-1390972072_i32),(-35912065_i32),(-113145612_i32),(-2074719593_i32)];
_23 = !(*_12).0;
(*_12).0 = _6.0 >= _7.0;
_13.3 = (_11, 2712113264510382333_i64, 23351_u16);
_16 = !_13.3.0;
_16 = _13.3.0;
_14 = '\u{9bdac}';
_4 = _19 as isize;
_13.4 = [_13.3.1];
_13.5 = _13.0;
_21 = _20;
_25 = _1;
_7.0 = _18 * _2.0;
(*_12).1 = -(-6407446113982916854_i64);
_5 = _7.0;
Call((*_12) = fn15(_20, _20, _25, _25, _6.0, _2.0, _21), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_20 = _21;
_5 = !_6.0;
RET = -_22.0;
_1 = !_5;
_19 = _13.3.1 as u64;
_10 = _13.1;
_9 = 41_i8 >> _3;
_2.0 = -_1;
(*_12).0 = _11;
_26 = -(-616521753_i32);
(*_8) = _13.1;
_27.fld1.3.0 = (*_12).0;
_7.0 = -_3;
_27.fld1.3.1 = (*_12).1;
_16 = !(*_12).0;
_9 = (-51_i8) >> _3;
_13.3 = (_16, _27.fld1.3.1, 2632_u16);
_26 = (-1208155089_i32);
_10 = [_26,_26,_26,_26,_26,_26,_26];
Goto(bb8)
}
bb8 = {
(*_12).0 = !_27.fld1.3.0;
_27.fld1.2 = _13.2 >> _18;
_27.fld1 = (_13.5, _13.1, _13.2, (*_12), _13.4, _13.5);
_13.1 = [_26,_26,_26,_26,_26,_26,_26];
_2.0 = _26 as isize;
_4 = (*_12).1 as isize;
_27.fld1.3 = (*_12);
_13.2 = -_27.fld1.2;
_3 = -_18;
Goto(bb9)
}
bb9 = {
_27.fld1.0 = !_27.fld1.5;
match (*_12).2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb7,
2632 => bb11,
_ => bb10
}
}
bb10 = {
(*_12).0 = !_27.fld1.3.0;
_27.fld1.2 = _13.2 >> _18;
_27.fld1 = (_13.5, _13.1, _13.2, (*_12), _13.4, _13.5);
_13.1 = [_26,_26,_26,_26,_26,_26,_26];
_2.0 = _26 as isize;
_4 = (*_12).1 as isize;
_27.fld1.3 = (*_12);
_13.2 = -_27.fld1.2;
_3 = -_18;
Goto(bb9)
}
bb11 = {
_21 = [_3,_5,_6.0,_7.0,_6.0,_3,_6.0,_6.0];
_30 = [_27.fld1.2,_27.fld1.2,_13.2,_27.fld1.2,_13.2,_27.fld1.2];
_27.fld1 = (_13.0, _10, _13.2, _13.3, _13.4, _13.0);
_27.fld1.3 = (*_12);
_27.fld1.3.0 = _16;
_24 = _14;
_27.fld1.5 = _26 as u32;
_22.0 = _15 as i16;
_10 = _27.fld1.1;
_6 = _7;
_32 = _19 as f64;
_9 = 67_i8;
_27.fld1.3.1 = (*_12).1 * _13.3.1;
_21 = _20;
_18 = _25;
_6.0 = _1;
_6.0 = _7.0;
_29 = 7_usize as i64;
_13.3.2 = _27.fld1.3.2;
_27.fld1.1 = _13.1;
(*_12).0 = _16;
_13.1 = [_26,_26,_26,_26,_26,_26,_26];
_27.fld1.0 = _19 as u32;
_13.5 = _3 as u32;
Goto(bb12)
}
bb12 = {
_5 = _18;
_6 = (_25,);
_9 = (-28_i8) | 54_i8;
_13.3.2 = _27.fld1.3.2;
_15 = 80_u8;
_13.1 = [_26,_26,_26,_26,_26,_26,_26];
_25 = _1;
_31 = core::ptr::addr_of_mut!(_13);
(*_12).2 = _27.fld1.3.2 % _27.fld1.3.2;
(*_31).3.1 = -_27.fld1.3.1;
_13.4 = [(*_31).3.1];
(*_31).1 = _10;
_6 = (_3,);
_13 = (_27.fld1.5, (*_8), _27.fld1.2, _27.fld1.3, _27.fld1.4, _27.fld1.0);
match _27.fld1.3.2 {
0 => bb10,
1 => bb4,
2 => bb5,
2632 => bb14,
_ => bb13
}
}
bb13 = {
(*_12).0 = _11;
_13.4 = [_13.3.1];
_5 = _13.5 as isize;
_8 = core::ptr::addr_of!(_10);
_13.4 = [(*_12).1];
_7.0 = -_6.0;
(*_8) = _13.1;
_7.0 = _4;
_20 = [_7.0,_7.0,_1,_6.0,_1,_4,_2.0,_1];
(*_8) = [1031354313_i32,1159145527_i32,(-2068673054_i32),(-1224275821_i32),(-2113080712_i32),2117401522_i32,1426177515_i32];
_10 = _13.1;
(*_8) = [1391745560_i32,724466631_i32,87710333_i32,(-511115743_i32),827789851_i32,1962469808_i32,702397622_i32];
_3 = !_4;
_18 = _2.0 - _4;
_21 = [_6.0,_1,_7.0,_7.0,_6.0,_18,_1,_6.0];
_13.2 = !52778528962744143713732576804312952587_i128;
_13.3 = (_11, 6385417153220819572_i64, 34639_u16);
_13.3.1 = _13.3.0 as i64;
_6.0 = _1;
_12 = core::ptr::addr_of!((*_12));
_3 = _7.0;
_13.5 = 579675866_u32;
_23 = _13.3.0;
_13.3 = (_11, 6805270698194585387_i64, 47802_u16);
_15 = !173_u8;
_22 = (RET,);
Goto(bb5)
}
bb14 = {
_18 = _9 as isize;
(*_12).1 = _29;
(*_12).2 = !_27.fld1.3.2;
_9 = 49_i8 + 118_i8;
_22 = (RET,);
_2.0 = _26 as isize;
(*_31).3.1 = _32 as i64;
_20 = [_5,_7.0,_6.0,_3,_7.0,_5,_25,_5];
(*_12) = (_23, _27.fld1.3.1, _27.fld1.3.2);
_10 = [_26,_26,_26,_26,_26,_26,_26];
(*_31).3 = (_23, _27.fld1.3.1, _27.fld1.3.2);
(*_12).1 = _29;
(*_31).3.2 = _11 as u16;
_13.3.0 = !_16;
_7 = _6;
_25 = _32 as isize;
_27.fld1.3.2 = _26 as u16;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(8_usize, 6_usize, Move(_6), 16_usize, Move(_16), 21_usize, Move(_21), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(8_usize, 2_usize, Move(_2), 25_usize, Move(_25), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(8_usize, 19_usize, Move(_19), 9_usize, Move(_9), 11_usize, Move(_11), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: (isize,),mut _3: (isize,),mut _4: isize,mut _5: (isize,),mut _6: isize,mut _7: (isize,),mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> bool {
mir! {
type RET = bool;
let _15: Adt40;
let _16: i128;
let _17: bool;
let _18: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _19: ([i32; 7], u64);
let _20: ();
let _21: ();
{
_6 = _8;
_9 = _11 * _8;
_5.0 = !_6;
_4 = (-76_i8) as isize;
_13 = !_11;
_6 = !_11;
_3.0 = 122_u8 as isize;
_2 = _7;
_3 = (_14,);
_5.0 = 91_i8 as isize;
_9 = true as isize;
_7.0 = _10 >> _6;
_5.0 = !_12;
_7 = (_10,);
_13 = _10 | _7.0;
_2.0 = _3.0;
_8 = _11 | _5.0;
_17 = true;
_7 = (_8,);
_8 = 67594682_i32 as isize;
_6 = !_11;
Call(_3.0 = core::intrinsics::transmute(_7.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = (_1,);
RET = _14 > _6;
_16 = _11 as i128;
_18.0 = 3710038367_u32 + 547260728_u32;
_2 = (_6,);
_18.0 = 2888389160_u32 >> _16;
_18.1 = [(-552849526_i32),1914936037_i32,(-763039260_i32),2089443432_i32,739392197_i32,(-241854149_i32),(-1921489239_i32)];
_12 = !_3.0;
_6 = _12 ^ _1;
_14 = _5.0;
_18.5 = (-853013133_i32) as u32;
RET = _11 != _11;
RET = _17;
_17 = _10 == _12;
_19 = (_18.1, 2258259617792396128_u64);
RET = _17;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(9_usize, 6_usize, Move(_6), 5_usize, Move(_5), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(9_usize, 16_usize, Move(_16), 13_usize, Move(_13), 17_usize, Move(_17), 11_usize, Move(_11)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: [isize; 8],mut _3: isize) -> char {
mir! {
type RET = char;
let _4: i32;
let _5: Adt44;
let _6: u8;
let _7: Adt52;
let _8: *mut u8;
let _9: Adt53;
let _10: isize;
let _11: bool;
let _12: [bool; 6];
let _13: [char; 7];
let _14: Adt42;
let _15: *const [i32; 7];
let _16: char;
let _17: bool;
let _18: u8;
let _19: (f32, char, u8, f32);
let _20: char;
let _21: Adt49;
let _22: Adt49;
let _23: [char; 3];
let _24: [i128; 6];
let _25: f64;
let _26: Adt48;
let _27: char;
let _28: [isize; 8];
let _29: char;
let _30: Adt40;
let _31: char;
let _32: ();
let _33: ();
{
RET = '\u{68788}';
_2 = [_3,_3,_3,_1,_1,_3,_1,_1];
_3 = !_1;
_3 = !_1;
_4 = 1102815417_i32 | (-307740010_i32);
RET = '\u{6b42d}';
Call(_4 = fn11(_2, _3, _2, _3, _1, _3, _3, _2, _1, _3, _2, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-3316888449107891278138201549531955298_i128) as i32;
_1 = -_3;
_4 = true as i32;
_1 = _3;
_2 = [_1,_3,_1,_1,_3,_1,_1,_1];
_2 = [_1,_1,_1,_1,_1,_3,_3,_1];
_4 = (-1292064985_i32);
RET = '\u{f181e}';
match _4 {
340282366920938463463374607430476146471 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_3 = RET as isize;
RET = '\u{5fd1e}';
RET = '\u{d7fc5}';
_6 = 224_u8;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
340282366920938463463374607430476146471 => bb7,
_ => bb6
}
}
bb4 = {
Return()
}
bb5 = {
_4 = (-3316888449107891278138201549531955298_i128) as i32;
_1 = -_3;
_4 = true as i32;
_1 = _3;
_2 = [_1,_3,_1,_1,_3,_1,_1,_1];
_2 = [_1,_1,_1,_1,_1,_3,_3,_1];
_4 = (-1292064985_i32);
RET = '\u{f181e}';
match _4 {
340282366920938463463374607430476146471 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
RET = '\u{9f2b0}';
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
match _4 {
0 => bb5,
1 => bb2,
2 => bb3,
340282366920938463463374607430476146471 => bb8,
_ => bb6
}
}
bb8 = {
RET = '\u{c1af5}';
_4 = 1749699691_i32;
_8 = core::ptr::addr_of_mut!(_6);
_10 = -_1;
(*_8) = 101_u8;
_10 = _1 - _1;
(*_8) = 164_u8;
RET = '\u{ca83d}';
_1 = -_10;
(*_8) = 219578924703297355650435787928970341720_u128 as u8;
_1 = _10 | _10;
_12 = [false,false,false,true,false,false];
(*_8) = 22_u8;
_6 = 2723646081124302573_u64 as u8;
_8 = core::ptr::addr_of_mut!((*_8));
(*_8) = 135_u8;
_13 = [RET,RET,RET,RET,RET,RET,RET];
_3 = -_1;
_6 = 71_u8;
match (*_8) {
0 => bb1,
1 => bb2,
2 => bb3,
71 => bb9,
_ => bb4
}
}
bb9 = {
_11 = _1 >= _1;
_11 = !false;
_4 = -2130449986_i32;
_6 = !68_u8;
_2 = [_1,_1,_10,_10,_1,_1,_10,_1];
_3 = (-6197_i16) as isize;
_2 = [_10,_10,_1,_1,_10,_10,_10,_1];
_13 = [RET,RET,RET,RET,RET,RET,RET];
_6 = 132_u8;
_13 = [RET,RET,RET,RET,RET,RET,RET];
_4 = (-1429535917_i32) << _10;
RET = '\u{50705}';
_11 = _10 >= _10;
_10 = -_1;
_2 = [_1,_10,_1,_10,_1,_10,_10,_10];
_6 = 5201_i16 as u8;
_4 = (-1123716489_i32);
_2 = [_10,_10,_10,_1,_10,_1,_1,_1];
_3 = !_10;
Call(_11 = fn13(_1, _1, _1, _3, _3, _2, _1, _1, _3, _10, _10, _1, _10, _1, _10, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_8) = !134_u8;
Goto(bb11)
}
bb11 = {
RET = '\u{570de}';
_16 = RET;
_10 = -_1;
_13 = [RET,_16,RET,_16,RET,RET,RET];
RET = _16;
_4 = (-1393326873_i32) - 1837105943_i32;
Goto(bb12)
}
bb12 = {
_2 = [_10,_3,_10,_3,_10,_1,_3,_10];
_19.3 = 72_i8 as f32;
_2 = [_3,_3,_3,_10,_1,_3,_3,_3];
_8 = core::ptr::addr_of_mut!(_19.2);
(*_8) = _6 >> _1;
_17 = _3 >= _10;
_2 = [_3,_3,_3,_3,_3,_3,_3,_1];
_23 = [_16,RET,RET];
_2 = [_3,_10,_3,_3,_10,_3,_3,_1];
_19.2 = _6 + _6;
Goto(bb13)
}
bb13 = {
RET = _16;
RET = _16;
_19.3 = 3570817668_u32 as f32;
_3 = _1 - _10;
_8 = core::ptr::addr_of_mut!((*_8));
_20 = _16;
(*_8) = _19.3 as u8;
Goto(bb14)
}
bb14 = {
_25 = (-4626918517921514316_i64) as f64;
_18 = _19.2;
_6 = !_19.2;
_19.0 = 49393_u16 as f32;
_10 = -_3;
_2 = [_10,_3,_10,_3,_10,_1,_3,_1];
(*_8) = _18 + _6;
_19.1 = RET;
_1 = -_10;
_19.1 = _16;
_27 = _16;
_18 = _20 as u8;
(*_8) = !_18;
_4 = (-1085241119_i32);
_13 = [_16,_27,_27,_20,_19.1,RET,_16];
_20 = _19.1;
_28 = _2;
_14 = Adt42::Variant0 { fld0: _19.3 };
_23 = [_27,_19.1,_20];
SetDiscriminant(_14, 1);
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(10_usize, 17_usize, Move(_17), 18_usize, Move(_18), 11_usize, Move(_11), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(10_usize, 4_usize, Move(_4), 28_usize, Move(_28), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [isize; 8],mut _2: isize,mut _3: [isize; 8],mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [isize; 8],mut _9: isize,mut _10: isize,mut _11: [isize; 8],mut _12: isize) -> i32 {
mir! {
type RET = i32;
let _13: isize;
let _14: f64;
let _15: *const [i32; 7];
let _16: isize;
let _17: f32;
let _18: char;
let _19: Adt46;
let _20: Adt52;
let _21: f32;
let _22: char;
let _23: char;
let _24: u16;
let _25: i8;
let _26: f32;
let _27: ([i32; 7], u64);
let _28: bool;
let _29: Adt48;
let _30: (isize,);
let _31: i32;
let _32: (*mut u8, [char; 7]);
let _33: [i128; 6];
let _34: ();
let _35: ();
{
_2 = false as isize;
RET = -(-1222754896_i32);
RET = (-1429_i16) as i32;
_9 = !_12;
_5 = !_6;
_11 = [_9,_4,_10,_7,_6,_4,_10,_9];
_7 = 3903404872_u32 as isize;
_8 = [_4,_10,_4,_10,_6,_5,_6,_9];
_3 = [_9,_5,_10,_5,_12,_12,_10,_9];
_6 = _9 - _9;
_5 = _4 + _9;
RET = 1125245098_i32;
_10 = 221708614173179343759889569854217693366_u128 as isize;
_14 = 144324051754275710532366681906927769685_u128 as f64;
RET = -(-230071704_i32);
_1 = [_4,_9,_6,_4,_5,_5,_6,_4];
_10 = 2785041558_u32 as isize;
_17 = 3222221974294289875_i64 as f32;
_13 = !_4;
_8 = _11;
_4 = _13 >> _6;
_14 = 3545707322510715899_i64 as f64;
_6 = _12;
_16 = _12 | _9;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = (-1581339173276115293_i64) as isize;
_17 = RET as f32;
_16 = 15_u8 as isize;
_16 = !_4;
_4 = -_6;
_16 = _12;
_10 = 13772_u16 as isize;
_8 = [_5,_13,_6,_12,_16,_16,_5,_16];
_1 = [_5,_12,_13,_5,_12,_12,_6,_12];
_10 = _6 >> _13;
_6 = !_10;
_5 = 189_u8 as isize;
_16 = -_4;
_4 = _13;
_7 = -_6;
_8 = [_12,_4,_10,_10,_12,_7,_16,_6];
_18 = '\u{efaed}';
RET = 2056399320_i32;
_12 = !_16;
_21 = -_17;
_1 = [_16,_12,_12,_10,_16,_12,_10,_10];
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2056399320 => bb10,
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
_16 = !_12;
_14 = (-54_i8) as f64;
_10 = (-33_i8) as isize;
_14 = (-4321773879025261616_i64) as f64;
_4 = 124_i8 as isize;
_23 = _18;
_16 = !_13;
_23 = _18;
_22 = _23;
_6 = _12;
_18 = _22;
RET = 1170957062_i32 | (-1711387448_i32);
_26 = _21;
_25 = -(-47_i8);
_13 = _7 | _12;
_24 = 43395_u16;
_16 = 50655382846618265674337289172156087506_i128 as isize;
_2 = !_6;
_2 = _17 as isize;
_11 = [_13,_6,_13,_12,_6,_7,_13,_13];
_26 = _21;
_22 = _23;
_13 = -_12;
match _24 {
0 => bb8,
1 => bb7,
43395 => bb11,
_ => bb3
}
}
bb11 = {
_13 = -_7;
_27.0 = [RET,RET,RET,RET,RET,RET,RET];
_6 = 145625392440040206188564510193871838992_u128 as isize;
_7 = _12;
_15 = core::ptr::addr_of!(_27.0);
_10 = -_13;
RET = (-86148718_i32);
_4 = _23 as isize;
_8 = [_7,_13,_13,_13,_13,_7,_7,_7];
_6 = _10;
_24 = _17 as u16;
_21 = _17 * _26;
_24 = !57781_u16;
_9 = _13;
_31 = _6 as i32;
Call(_12 = fn12(_7, _31, _6, _13, _6, _1, _1, _1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6 = _13;
_27.1 = 5233036017078050678_u64;
_27.0 = [_31,_31,_31,_31,_31,_31,_31];
match _27.1 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5233036017078050678 => bb19,
_ => bb18
}
}
bb13 = {
Return()
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
_7 = _9 ^ _10;
_6 = _12 - _10;
_25 = !87_i8;
_26 = _27.1 as f32;
_27.0 = [_31,_31,_31,_31,_31,_31,_31];
_32.1 = [_18,_22,_22,_22,_22,_22,_23];
_30.0 = (-141425677781537171410997087028455219683_i128) as isize;
_32.1 = [_22,_22,_22,_18,_18,_23,_23];
_6 = _13 >> _10;
_10 = _6 - _12;
Goto(bb20)
}
bb20 = {
Call(_34 = dump_var(11_usize, 6_usize, Move(_6), 24_usize, Move(_24), 13_usize, Move(_13), 2_usize, Move(_2)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_34 = dump_var(11_usize, 3_usize, Move(_3), 5_usize, Move(_5), 4_usize, Move(_4), 22_usize, Move(_22)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_34 = dump_var(11_usize, 9_usize, Move(_9), 7_usize, Move(_7), 18_usize, Move(_18), 35_usize, _35), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: i32,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [isize; 8],mut _7: [isize; 8],mut _8: [isize; 8]) -> isize {
mir! {
type RET = isize;
let _9: Adt53;
let _10: [bool; 6];
let _11: Adt45;
let _12: isize;
let _13: ();
let _14: ();
{
RET = -_4;
RET = _3;
_6 = [RET,_1,_3,_5,RET,RET,_4,_4];
_1 = RET ^ _5;
RET = _5 | _5;
_6 = [_5,_1,_1,_1,_3,_3,_5,_1];
_1 = _4;
_7 = _6;
_11.fld2.fld1.2 = 12658317081600161406_usize as i128;
RET = _5 ^ _3;
_11.fld2.fld1.4 = [(-8115357306074544770_i64)];
_11.fld2.fld1.0 = 2091963022_u32;
_10 = [true,false,false,true,true,true];
_5 = 74_i8 as isize;
_10 = [false,false,true,false,false,false];
_11.fld2.fld1.3.0 = false;
_8 = _6;
_2 = !(-278800348_i32);
_11.fld1 = '\u{10ef5}';
_11.fld2.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
_11.fld2.fld1.3.1 = (-6675577122503445701_i64) * 233273375990725721_i64;
_11.fld2.fld1.1 = [_2,_2,_2,_2,_2,_2,_2];
_5 = RET;
_11.fld2.fld1.3.0 = false;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(12_usize, 1_usize, Move(_1), 5_usize, Move(_5), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [isize; 8],mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: [isize; 8]) -> bool {
mir! {
type RET = bool;
let _17: i8;
let _18: u16;
let _19: Adt41;
let _20: isize;
let _21: i16;
let _22: i32;
let _23: f64;
let _24: Adt40;
let _25: *const (bool, i64, u16);
let _26: ();
let _27: ();
{
_11 = 385264729_u32 as isize;
_6 = _16;
_6 = _16;
_7 = _5 << _2;
Goto(bb1)
}
bb1 = {
_6 = _16;
_17 = (-104_i8);
_3 = _2;
_1 = _8;
_11 = false as isize;
_5 = !_10;
_3 = _10;
match _17 {
340282366920938463463374607431768211352 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_10 = -_1;
_6 = _16;
match _17 {
0 => bb2,
1 => bb4,
2 => bb5,
340282366920938463463374607431768211352 => bb7,
_ => bb6
}
}
bb4 = {
Return()
}
bb5 = {
_6 = _16;
_17 = (-104_i8);
_3 = _2;
_1 = _8;
_11 = false as isize;
_5 = !_10;
_3 = _10;
match _17 {
340282366920938463463374607431768211352 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
_12 = 29762_u16 as isize;
RET = !true;
_8 = !_3;
_2 = _10 | _9;
RET = !false;
_5 = _8 | _15;
_3 = _1 ^ _2;
Call(RET = fn14(_6, _4, _1, _2, _3, _5, _15, _6, _9, _2, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = -_7;
_20 = (-13102_i16) as isize;
_18 = 50169_u16 >> _1;
_12 = _13;
_11 = _4 ^ _3;
_9 = _7;
_22 = 14246588499858378127_u64 as i32;
_2 = 103839251009555252_u64 as isize;
match _17 {
340282366920938463463374607431768211352 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_2 = !_5;
_23 = 8310504262829495144_u64 as f64;
_6 = [_9,_9,_9,_10,_14,_15,_11,_15];
_9 = 27185775791200792299968054351135458229_u128 as isize;
_16 = _6;
_9 = 2809591728_u32 as isize;
_12 = -_14;
_22 = 92010375_i32 | 711985222_i32;
_10 = 1158460591_u32 as isize;
RET = _4 > _4;
Goto(bb11)
}
bb11 = {
Call(_26 = dump_var(13_usize, 7_usize, Move(_7), 12_usize, Move(_12), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_26 = dump_var(13_usize, 10_usize, Move(_10), 22_usize, Move(_22), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_26 = dump_var(13_usize, 2_usize, Move(_2), 3_usize, Move(_3), 27_usize, _27, 27_usize, _27), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [isize; 8],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [isize; 8],mut _9: isize,mut _10: isize,mut _11: isize) -> bool {
mir! {
type RET = bool;
let _12: [i32; 7];
let _13: i8;
let _14: *const (bool, i64, u16);
let _15: [usize; 2];
let _16: u8;
let _17: [i64; 1];
let _18: f32;
let _19: u64;
let _20: [char; 7];
let _21: (f32, char, u8, f32);
let _22: (isize,);
let _23: isize;
let _24: f32;
let _25: i16;
let _26: ();
let _27: ();
{
RET = !false;
_5 = _7;
_4 = _3 ^ _7;
_7 = _3 ^ _6;
_6 = _5 * _10;
_5 = _2;
Call(_8 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [_2,_7,_2,_7,_7,_6,_10,_6];
_3 = 17573241219682159362_u64 as isize;
_8 = [_11,_10,_2,_10,_6,_4,_9,_6];
_7 = (-69_i8) as isize;
_13 = !(-114_i8);
_1 = [_2,_2,_4,_9,_11,_2,_6,_10];
RET = false ^ true;
_13 = -105_i8;
RET = _2 != _11;
_16 = !21_u8;
_6 = !_2;
_3 = -_11;
_16 = 3548404945_u32 as u8;
_12 = [(-1965369518_i32),278317889_i32,(-658508873_i32),329619918_i32,(-1896844862_i32),(-1022659247_i32),(-631500374_i32)];
_7 = _6 + _3;
_16 = 171977849_u32 as u8;
_15 = [1_usize,4790327181194482570_usize];
RET = true;
_21.0 = 3467658641472457503_u64 as f32;
_17 = [2262172250776257776_i64];
Goto(bb2)
}
bb2 = {
_6 = _5 ^ _4;
_18 = _21.0 + _21.0;
_16 = 44_u8 + 163_u8;
_15 = [4_usize,1_usize];
_21.1 = '\u{4254e}';
_10 = _4;
_15 = [7_usize,3_usize];
_21 = (_18, '\u{e4cc7}', _16, _18);
_18 = _21.0 * _21.0;
Goto(bb3)
}
bb3 = {
RET = !false;
RET = false;
_13 = (-17_i8) + 93_i8;
_5 = !_7;
_10 = _7;
_2 = _3 ^ _3;
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13 = (-61_i8);
_8 = [_6,_10,_10,_11,_5,_6,_4,_2];
_8 = _1;
_5 = (-8697726095212184197_i64) as isize;
_21.2 = _16;
_3 = _6 | _9;
_22 = (_3,);
_18 = _21.0 - _21.3;
_19 = 10569922759458327964_u64;
_21.0 = _21.3 * _21.3;
Goto(bb5)
}
bb5 = {
_2 = _6;
_21.1 = '\u{9cf00}';
_4 = !_6;
_3 = _9 << _7;
_21.3 = _18;
_15 = [10853894260489670496_usize,4_usize];
_4 = !_7;
_21 = (_18, '\u{88c13}', _16, _18);
_11 = -_4;
_18 = _21.3 * _21.3;
_17 = [(-3418239317550843311_i64)];
_1 = _8;
match _19 {
0 => bb6,
10569922759458327964 => bb8,
_ => bb7
}
}
bb6 = {
_13 = (-61_i8);
_8 = [_6,_10,_10,_11,_5,_6,_4,_2];
_8 = _1;
_5 = (-8697726095212184197_i64) as isize;
_21.2 = _16;
_3 = _6 | _9;
_22 = (_3,);
_18 = _21.0 - _21.3;
_19 = 10569922759458327964_u64;
_21.0 = _21.3 * _21.3;
Goto(bb5)
}
bb7 = {
_8 = [_2,_7,_2,_7,_7,_6,_10,_6];
_3 = 17573241219682159362_u64 as isize;
_8 = [_11,_10,_2,_10,_6,_4,_9,_6];
_7 = (-69_i8) as isize;
_13 = !(-114_i8);
_1 = [_2,_2,_4,_9,_11,_2,_6,_10];
RET = false ^ true;
_13 = -105_i8;
RET = _2 != _11;
_16 = !21_u8;
_6 = !_2;
_3 = -_11;
_16 = 3548404945_u32 as u8;
_12 = [(-1965369518_i32),278317889_i32,(-658508873_i32),329619918_i32,(-1896844862_i32),(-1022659247_i32),(-631500374_i32)];
_7 = _6 + _3;
_16 = 171977849_u32 as u8;
_15 = [1_usize,4790327181194482570_usize];
RET = true;
_21.0 = 3467658641472457503_u64 as f32;
_17 = [2262172250776257776_i64];
Goto(bb2)
}
bb8 = {
_1 = _8;
_10 = _21.0 as isize;
_3 = _4 >> _22.0;
_6 = _9 | _3;
_2 = _3;
_12 = [(-1177754122_i32),953337593_i32,(-1809668638_i32),(-1628574587_i32),1331140796_i32,(-976154282_i32),(-900538873_i32)];
_11 = !_2;
_22.0 = _7 * _6;
_12 = [(-458131082_i32),(-1385751462_i32),1207689615_i32,(-1501928503_i32),(-981398086_i32),880607074_i32,257973667_i32];
_20 = [_21.1,_21.1,_21.1,_21.1,_21.1,_21.1,_21.1];
_21.3 = _18;
_4 = -_7;
match _13 {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
340282366920938463463374607431768211395 => bb16,
_ => bb15
}
}
bb9 = {
_8 = [_2,_7,_2,_7,_7,_6,_10,_6];
_3 = 17573241219682159362_u64 as isize;
_8 = [_11,_10,_2,_10,_6,_4,_9,_6];
_7 = (-69_i8) as isize;
_13 = !(-114_i8);
_1 = [_2,_2,_4,_9,_11,_2,_6,_10];
RET = false ^ true;
_13 = -105_i8;
RET = _2 != _11;
_16 = !21_u8;
_6 = !_2;
_3 = -_11;
_16 = 3548404945_u32 as u8;
_12 = [(-1965369518_i32),278317889_i32,(-658508873_i32),329619918_i32,(-1896844862_i32),(-1022659247_i32),(-631500374_i32)];
_7 = _6 + _3;
_16 = 171977849_u32 as u8;
_15 = [1_usize,4790327181194482570_usize];
RET = true;
_21.0 = 3467658641472457503_u64 as f32;
_17 = [2262172250776257776_i64];
Goto(bb2)
}
bb10 = {
_13 = (-61_i8);
_8 = [_6,_10,_10,_11,_5,_6,_4,_2];
_8 = _1;
_5 = (-8697726095212184197_i64) as isize;
_21.2 = _16;
_3 = _6 | _9;
_22 = (_3,);
_18 = _21.0 - _21.3;
_19 = 10569922759458327964_u64;
_21.0 = _21.3 * _21.3;
Goto(bb5)
}
bb11 = {
_2 = _6;
_21.1 = '\u{9cf00}';
_4 = !_6;
_3 = _9 << _7;
_21.3 = _18;
_15 = [10853894260489670496_usize,4_usize];
_4 = !_7;
_21 = (_18, '\u{88c13}', _16, _18);
_11 = -_4;
_18 = _21.3 * _21.3;
_17 = [(-3418239317550843311_i64)];
_1 = _8;
match _19 {
0 => bb6,
10569922759458327964 => bb8,
_ => bb7
}
}
bb12 = {
_13 = (-61_i8);
_8 = [_6,_10,_10,_11,_5,_6,_4,_2];
_8 = _1;
_5 = (-8697726095212184197_i64) as isize;
_21.2 = _16;
_3 = _6 | _9;
_22 = (_3,);
_18 = _21.0 - _21.3;
_19 = 10569922759458327964_u64;
_21.0 = _21.3 * _21.3;
Goto(bb5)
}
bb13 = {
RET = !false;
RET = false;
_13 = (-17_i8) + 93_i8;
_5 = !_7;
_10 = _7;
_2 = _3 ^ _3;
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_6 = _5 ^ _4;
_18 = _21.0 + _21.0;
_16 = 44_u8 + 163_u8;
_15 = [4_usize,1_usize];
_21.1 = '\u{4254e}';
_10 = _4;
_15 = [7_usize,3_usize];
_21 = (_18, '\u{e4cc7}', _16, _18);
_18 = _21.0 * _21.0;
Goto(bb3)
}
bb15 = {
_8 = [_2,_7,_2,_7,_7,_6,_10,_6];
_3 = 17573241219682159362_u64 as isize;
_8 = [_11,_10,_2,_10,_6,_4,_9,_6];
_7 = (-69_i8) as isize;
_13 = !(-114_i8);
_1 = [_2,_2,_4,_9,_11,_2,_6,_10];
RET = false ^ true;
_13 = -105_i8;
RET = _2 != _11;
_16 = !21_u8;
_6 = !_2;
_3 = -_11;
_16 = 3548404945_u32 as u8;
_12 = [(-1965369518_i32),278317889_i32,(-658508873_i32),329619918_i32,(-1896844862_i32),(-1022659247_i32),(-631500374_i32)];
_7 = _6 + _3;
_16 = 171977849_u32 as u8;
_15 = [1_usize,4790327181194482570_usize];
RET = true;
_21.0 = 3467658641472457503_u64 as f32;
_17 = [2262172250776257776_i64];
Goto(bb2)
}
bb16 = {
_21 = (_18, '\u{5e45e}', _16, _18);
_24 = 297291347166804793320281921815823053885_u128 as f32;
_8 = [_6,_22.0,_3,_2,_11,_3,_4,_4];
_9 = !_11;
_2 = _9 & _3;
_9 = _22.0 << _2;
_16 = 2901370463_u32 as u8;
_8 = [_11,_22.0,_3,_4,_11,_2,_11,_3];
_8 = [_22.0,_11,_2,_4,_9,_6,_11,_6];
_21.3 = 1406741804_i32 as f32;
_23 = -_6;
_19 = 2_usize as u64;
_16 = _21.2 * _21.2;
_21.3 = -_24;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(14_usize, 23_usize, Move(_23), 8_usize, Move(_8), 19_usize, Move(_19), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(14_usize, 22_usize, Move(_22), 4_usize, Move(_4), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_26 = dump_var(14_usize, 10_usize, Move(_10), 20_usize, Move(_20), 27_usize, _27, 27_usize, _27), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [isize; 8],mut _2: [isize; 8],mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: [isize; 8]) -> (bool, i64, u16) {
mir! {
type RET = (bool, i64, u16);
let _8: f32;
let _9: f64;
let _10: i32;
let _11: Adt49;
let _12: isize;
let _13: Adt54;
let _14: (isize,);
let _15: (*mut u8, [char; 7]);
let _16: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32));
let _17: f64;
let _18: i8;
let _19: f64;
let _20: Adt44;
let _21: [char; 7];
let _22: *mut u8;
let _23: u32;
let _24: Adt45;
let _25: i32;
let _26: ();
let _27: ();
{
RET.2 = !64534_u16;
_5 = 2448375052844060016379809311442042007_i128 as isize;
RET.0 = !false;
RET.2 = 8259425672034464941_u64 as u16;
RET.1 = !(-3314527529155234946_i64);
RET.2 = 50148_u16 ^ 45683_u16;
RET.1 = 7500001536710627836_i64 + 3018625746040444790_i64;
_7 = [_4,_6,_3,_4,_6,_3,_6,_6];
_1 = [_6,_6,_4,_3,_4,_3,_6,_3];
Goto(bb1)
}
bb1 = {
RET.0 = true ^ false;
_5 = RET.2 as isize;
_2 = [_4,_4,_3,_3,_6,_6,_3,_3];
_3 = !_4;
_2 = [_6,_4,_6,_3,_4,_6,_4,_3];
_5 = _4;
_9 = 160_u8 as f64;
RET.2 = 21563_u16;
RET = (false, 829390460144923268_i64, 11627_u16);
_3 = _5;
_3 = !_4;
RET.1 = (-73838539634134908738907930107997064995_i128) as i64;
_10 = -(-2135711441_i32);
_8 = RET.2 as f32;
_1 = [_5,_5,_3,_6,_3,_3,_4,_4];
_9 = _10 as f64;
_10 = 1295411309_i32;
_12 = _9 as isize;
Goto(bb2)
}
bb2 = {
RET.2 = _9 as u16;
_2 = _1;
RET.1 = 2413865035122038782_i64 << _6;
_8 = 1029469714_u32 as f32;
RET.0 = _6 < _6;
_3 = _4 * _5;
RET = (true, (-508417332830493960_i64), 6112_u16);
_14.0 = !_5;
_6 = _5;
_8 = (-119_i8) as f32;
RET.0 = true;
_1 = [_6,_14.0,_6,_4,_5,_6,_6,_3];
_5 = _3;
_6 = _8 as isize;
RET.2 = 59219_u16 * 46693_u16;
_6 = _3 | _4;
_13 = Adt54::Variant0 { fld0: 11767651363195173419_u64 };
Call(_15 = fn16(_4, _5, _6, _6, _14.0, _1, _6, _4, _2, RET, _7, _7, _3, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = _1;
_14 = (_5,);
match RET.1 {
0 => bb1,
340282366920938463462866190098937717496 => bb5,
_ => bb4
}
}
bb4 = {
RET.0 = true ^ false;
_5 = RET.2 as isize;
_2 = [_4,_4,_3,_3,_6,_6,_3,_3];
_3 = !_4;
_2 = [_6,_4,_6,_3,_4,_6,_4,_3];
_5 = _4;
_9 = 160_u8 as f64;
RET.2 = 21563_u16;
RET = (false, 829390460144923268_i64, 11627_u16);
_3 = _5;
_3 = !_4;
RET.1 = (-73838539634134908738907930107997064995_i128) as i64;
_10 = -(-2135711441_i32);
_8 = RET.2 as f32;
_1 = [_5,_5,_3,_6,_3,_3,_4,_4];
_9 = _10 as f64;
_10 = 1295411309_i32;
_12 = _9 as isize;
Goto(bb2)
}
bb5 = {
_16.1.5 = 18433969794645556889_u64 as u32;
_4 = !_14.0;
RET.0 = false;
RET.0 = !true;
RET.1 = -8073554525513068661_i64;
RET = (false, 3243148591630021324_i64, 65061_u16);
_16.1.3.1 = _16.1.5 as i64;
_19 = _9 - _9;
_16.1.4 = [RET.1];
_16.1.0 = _16.1.5;
RET.1 = _16.1.3.1 ^ _16.1.3.1;
_16.1.3.2 = RET.2;
_3 = _14.0 << _5;
_16.0 = -_8;
_9 = (-21300_i16) as f64;
Goto(bb6)
}
bb6 = {
_16.1.3.0 = RET.0;
_18 = 26_i8;
_2 = [_4,_4,_5,_4,_3,_14.0,_4,_3];
RET = (_16.1.3.0, _16.1.3.1, _16.1.3.2);
_21 = ['\u{32952}','\u{102816}','\u{6ac9a}','\u{f685e}','\u{f091e}','\u{c8cec}','\u{992f8}'];
_16.1.2 = -(-69718692957848446831869783824193052072_i128);
_13 = Adt54::Variant0 { fld0: 8992190430940674408_u64 };
_7 = _2;
_17 = _19;
_7 = _1;
_14 = (_5,);
match RET.2 {
65061 => bb7,
_ => bb4
}
}
bb7 = {
_23 = _16.1.5;
_24.fld2.fld0 = _19 as u64;
_2 = [_6,_14.0,_4,_4,_6,_3,_3,_4];
RET.0 = _16.1.3.0;
_16.1.3 = RET;
_16.1.3.1 = 180064949189968363110851278104574239384_u128 as i64;
_24.fld2.fld1.0 = _16.1.5;
RET.0 = _14.0 < _5;
_24.fld2.fld1.3 = RET;
_16.1.4 = [_16.1.3.1];
_6 = _5;
_23 = _16.1.0 << _4;
_16.1.5 = _16.0 as u32;
_14 = (_4,);
_24.fld2.fld1.3.2 = _16.1.2 as u16;
_9 = _10 as f64;
_24.fld2.fld1.1 = [_10,_10,_10,_10,_10,_10,_10];
_14 = (_3,);
Goto(bb8)
}
bb8 = {
Call(_26 = dump_var(15_usize, 2_usize, Move(_2), 3_usize, Move(_3), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_26 = dump_var(15_usize, 18_usize, Move(_18), 23_usize, Move(_23), 27_usize, _27, 27_usize, _27), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [isize; 8],mut _7: isize,mut _8: isize,mut _9: [isize; 8],mut _10: (bool, i64, u16),mut _11: [isize; 8],mut _12: [isize; 8],mut _13: isize,mut _14: [isize; 8],mut _15: [isize; 8]) -> (*mut u8, [char; 7]) {
mir! {
type RET = (*mut u8, [char; 7]);
let _16: [usize; 2];
let _17: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32);
let _18: Adt46;
let _19: isize;
let _20: isize;
let _21: usize;
let _22: char;
let _23: f64;
let _24: (f32, char, u8, f32);
let _25: bool;
let _26: (i16,);
let _27: i128;
let _28: i8;
let _29: u32;
let _30: u64;
let _31: i16;
let _32: Adt43;
let _33: (u64, f32, i16, isize);
let _34: [isize; 8];
let _35: [char; 3];
let _36: (isize,);
let _37: (isize,);
let _38: Adt50;
let _39: [bool; 6];
let _40: isize;
let _41: [bool; 6];
let _42: Adt48;
let _43: [isize; 8];
let _44: [usize; 2];
let _45: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32));
let _46: isize;
let _47: isize;
let _48: ();
let _49: ();
{
_1 = !_7;
_10.2 = 59597_u16 * 52412_u16;
_10.1 = 1062374511213343848_i64;
_15 = [_7,_2,_13,_8,_5,_8,_8,_4];
_11 = _12;
_14 = _9;
_15 = _14;
_10 = (false, 9084830537041598523_i64, 5159_u16);
_8 = -_3;
_11 = _6;
_10.1 = (-2434360287254230090_i64);
_10.0 = true;
RET.1 = ['\u{961b5}','\u{f9f0a}','\u{3f82d}','\u{1016ea}','\u{13b6d}','\u{2de45}','\u{4e726}'];
_2 = -_7;
_7 = !_13;
_12 = [_7,_4,_5,_5,_7,_4,_8,_7];
_12 = _14;
_10 = (true, (-4016765849474965466_i64), 48086_u16);
_12 = [_4,_8,_8,_13,_4,_3,_5,_8];
_6 = _11;
_10.0 = true;
_11 = _6;
_10 = (true, (-3424017420056688763_i64), 33524_u16);
_16 = [2687390001638869958_usize,17513091350171695477_usize];
_17.4 = [_10.1];
Call(_17.3.1 = core::intrinsics::bswap(_10.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17.3 = (_10.0, _10.1, _10.2);
_11 = [_7,_2,_8,_5,_13,_13,_4,_5];
_17.0 = 0_usize as u32;
_3 = _5;
RET.1 = ['\u{a2272}','\u{90e8e}','\u{822d6}','\u{b40a4}','\u{14061}','\u{2c57b}','\u{e66c2}'];
_12 = _6;
_17.2 = (-109_i8) as i128;
_17.3 = (_10.0, _10.1, _10.2);
_3 = _2 & _1;
_11 = [_2,_4,_8,_1,_13,_1,_7,_3];
_17.5 = _17.2 as u32;
_5 = _7;
_17.1 = [(-423274114_i32),(-1854029582_i32),(-430172299_i32),1753503316_i32,470589800_i32,(-991089531_i32),(-1501207086_i32)];
_17.3.2 = 6237276588258688632_u64 as u16;
_17.1 = [68065190_i32,(-658422241_i32),(-379620810_i32),1345944626_i32,(-898180196_i32),(-510411376_i32),(-619571377_i32)];
Goto(bb2)
}
bb2 = {
_10.0 = _17.3.0 & _17.3.0;
_20 = (-1467615065_i32) as isize;
_5 = _4;
_15 = _11;
_24.1 = '\u{32cc7}';
_17.3 = (_10.0, _10.1, _10.2);
_17.5 = !_17.0;
_17.3.1 = _10.1;
_7 = (-648303115_i32) as isize;
_2 = !_5;
RET.1 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_21 = !11836710396821479959_usize;
_23 = 662021981_i32 as f64;
_6 = _12;
RET.0 = core::ptr::addr_of_mut!(_24.2);
Goto(bb3)
}
bb3 = {
_17.3.1 = _10.1 * _10.1;
_11 = _15;
_26 = ((-19414_i16),);
_24.2 = 141_u8 ^ 182_u8;
_29 = 14154182873908429572_u64 as u32;
_19 = -_1;
_25 = _10.0 | _10.0;
_17.4 = [_10.1];
_17.0 = _29 & _17.5;
_24.0 = _26.0 as f32;
_10 = (_25, _17.3.1, _17.3.2);
_9 = _12;
_26 = ((-23333_i16),);
_2 = -_13;
Call(_30 = core::intrinsics::transmute(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = _24.1 as isize;
_27 = _17.2;
_21 = 0_usize & 0_usize;
_31 = _24.0 as i16;
_15 = _11;
_17.4 = [_10.1];
_24.0 = _30 as f32;
_8 = _19 & _3;
_27 = -_17.2;
_15 = _12;
RET.1 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_15 = _14;
_32.fld1.3.1 = _10.1;
_21 = 14107248118245681816_usize & 9819512550488454729_usize;
_4 = (-67_i8) as isize;
_3 = _21 as isize;
_32.fld1 = (_17.0, _17.1, _17.2, _10, _17.4, _17.5);
_28 = -(-31_i8);
_32 = Adt43 { fld0: _30,fld1: _17 };
_32.fld1.5 = _24.2 as u32;
_33.0 = _32.fld0 - _30;
_10.1 = (-403106392_i32) as i64;
_10.1 = _32.fld0 as i64;
match _10.2 {
0 => bb5,
33524 => bb7,
_ => bb6
}
}
bb5 = {
_17.3.1 = _10.1 * _10.1;
_11 = _15;
_26 = ((-19414_i16),);
_24.2 = 141_u8 ^ 182_u8;
_29 = 14154182873908429572_u64 as u32;
_19 = -_1;
_25 = _10.0 | _10.0;
_17.4 = [_10.1];
_17.0 = _29 & _17.5;
_24.0 = _26.0 as f32;
_10 = (_25, _17.3.1, _17.3.2);
_9 = _12;
_26 = ((-23333_i16),);
_2 = -_13;
Call(_30 = core::intrinsics::transmute(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_17.3 = (_10.0, _10.1, _10.2);
_11 = [_7,_2,_8,_5,_13,_13,_4,_5];
_17.0 = 0_usize as u32;
_3 = _5;
RET.1 = ['\u{a2272}','\u{90e8e}','\u{822d6}','\u{b40a4}','\u{14061}','\u{2c57b}','\u{e66c2}'];
_12 = _6;
_17.2 = (-109_i8) as i128;
_17.3 = (_10.0, _10.1, _10.2);
_3 = _2 & _1;
_11 = [_2,_4,_8,_1,_13,_1,_7,_3];
_17.5 = _17.2 as u32;
_5 = _7;
_17.1 = [(-423274114_i32),(-1854029582_i32),(-430172299_i32),1753503316_i32,470589800_i32,(-991089531_i32),(-1501207086_i32)];
_17.3.2 = 6237276588258688632_u64 as u16;
_17.1 = [68065190_i32,(-658422241_i32),(-379620810_i32),1345944626_i32,(-898180196_i32),(-510411376_i32),(-619571377_i32)];
Goto(bb2)
}
bb7 = {
_17.3.0 = !_25;
_35 = [_24.1,_24.1,_24.1];
_30 = _32.fld0;
_6 = [_8,_2,_8,_8,_2,_19,_19,_19];
_17.3.1 = _2 as i64;
_26 = (_31,);
_17.3.0 = _25 | _25;
_11 = _14;
_32.fld1.3.0 = _24.0 > _24.0;
_14 = [_2,_13,_5,_2,_3,_13,_19,_2];
_23 = _17.0 as f64;
_11 = [_5,_8,_8,_2,_5,_5,_2,_8];
_19 = _8 * _13;
_19 = _8 + _2;
_10.1 = _17.3.1 >> _19;
_32.fld1.3.0 = !_17.3.0;
_33.0 = _30 * _32.fld0;
_24.3 = _24.0;
RET.1 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_17.0 = _32.fld1.5 | _17.5;
_5 = _28 as isize;
Goto(bb8)
}
bb8 = {
_6 = _11;
_14 = [_8,_19,_19,_19,_19,_13,_8,_2];
_39 = [_32.fld1.3.0,_17.3.0,_17.3.0,_10.0,_25,_25];
_17.3.0 = !_32.fld1.3.0;
Goto(bb9)
}
bb9 = {
_37.0 = _8;
_11 = [_19,_8,_13,_19,_19,_8,_37.0,_19];
_17.3.2 = _10.2;
_36 = _37;
_33.0 = _32.fld0 + _32.fld0;
_19 = _28 as isize;
_36.0 = _8;
_23 = _24.2 as f64;
RET.0 = core::ptr::addr_of_mut!(_24.2);
_17.1 = [475033508_i32,(-1046046480_i32),(-542084345_i32),(-608365124_i32),(-778380351_i32),2103484017_i32,150161156_i32];
_33.3 = _26.0 as isize;
_10 = (_25, _17.3.1, _32.fld1.3.2);
_34 = _11;
_36 = (_37.0,);
_33.2 = !_31;
_22 = _24.1;
_32.fld0 = _17.3.2 as u64;
_32.fld1.3.2 = _17.3.2 % _17.3.2;
_7 = _8;
_27 = _31 as i128;
_10.1 = _17.3.1 >> _37.0;
_15 = _14;
_41 = [_32.fld1.3.0,_25,_10.0,_10.0,_10.0,_32.fld1.3.0];
Goto(bb10)
}
bb10 = {
_4 = _7;
_11 = [_13,_4,_8,_36.0,_37.0,_8,_4,_7];
_17.5 = _17.0 + _29;
_16 = [_21,_21];
_17.3.2 = _10.2;
match _17.3.2 {
0 => bb6,
1 => bb3,
2 => bb11,
3 => bb12,
4 => bb13,
33524 => bb15,
_ => bb14
}
}
bb11 = {
_10.0 = _17.3.0 & _17.3.0;
_20 = (-1467615065_i32) as isize;
_5 = _4;
_15 = _11;
_24.1 = '\u{32cc7}';
_17.3 = (_10.0, _10.1, _10.2);
_17.5 = !_17.0;
_17.3.1 = _10.1;
_7 = (-648303115_i32) as isize;
_2 = !_5;
RET.1 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_21 = !11836710396821479959_usize;
_23 = 662021981_i32 as f64;
_6 = _12;
RET.0 = core::ptr::addr_of_mut!(_24.2);
Goto(bb3)
}
bb12 = {
_1 = _24.1 as isize;
_27 = _17.2;
_21 = 0_usize & 0_usize;
_31 = _24.0 as i16;
_15 = _11;
_17.4 = [_10.1];
_24.0 = _30 as f32;
_8 = _19 & _3;
_27 = -_17.2;
_15 = _12;
RET.1 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_15 = _14;
_32.fld1.3.1 = _10.1;
_21 = 14107248118245681816_usize & 9819512550488454729_usize;
_4 = (-67_i8) as isize;
_3 = _21 as isize;
_32.fld1 = (_17.0, _17.1, _17.2, _10, _17.4, _17.5);
_28 = -(-31_i8);
_32 = Adt43 { fld0: _30,fld1: _17 };
_32.fld1.5 = _24.2 as u32;
_33.0 = _32.fld0 - _30;
_10.1 = (-403106392_i32) as i64;
_10.1 = _32.fld0 as i64;
match _10.2 {
0 => bb5,
33524 => bb7,
_ => bb6
}
}
bb13 = {
_17.3.1 = _10.1 * _10.1;
_11 = _15;
_26 = ((-19414_i16),);
_24.2 = 141_u8 ^ 182_u8;
_29 = 14154182873908429572_u64 as u32;
_19 = -_1;
_25 = _10.0 | _10.0;
_17.4 = [_10.1];
_17.0 = _29 & _17.5;
_24.0 = _26.0 as f32;
_10 = (_25, _17.3.1, _17.3.2);
_9 = _12;
_26 = ((-23333_i16),);
_2 = -_13;
Call(_30 = core::intrinsics::transmute(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_17.3 = (_10.0, _10.1, _10.2);
_11 = [_7,_2,_8,_5,_13,_13,_4,_5];
_17.0 = 0_usize as u32;
_3 = _5;
RET.1 = ['\u{a2272}','\u{90e8e}','\u{822d6}','\u{b40a4}','\u{14061}','\u{2c57b}','\u{e66c2}'];
_12 = _6;
_17.2 = (-109_i8) as i128;
_17.3 = (_10.0, _10.1, _10.2);
_3 = _2 & _1;
_11 = [_2,_4,_8,_1,_13,_1,_7,_3];
_17.5 = _17.2 as u32;
_5 = _7;
_17.1 = [(-423274114_i32),(-1854029582_i32),(-430172299_i32),1753503316_i32,470589800_i32,(-991089531_i32),(-1501207086_i32)];
_17.3.2 = 6237276588258688632_u64 as u16;
_17.1 = [68065190_i32,(-658422241_i32),(-379620810_i32),1345944626_i32,(-898180196_i32),(-510411376_i32),(-619571377_i32)];
Goto(bb2)
}
bb15 = {
_13 = _2 - _36.0;
RET.1 = [_24.1,_24.1,_24.1,_22,_24.1,_24.1,_24.1];
_27 = _32.fld1.2;
_33 = (_30, _24.3, _31, _36.0);
_45.1.1 = [(-155563319_i32),1646323849_i32,(-173365821_i32),731324882_i32,117706901_i32,(-420067400_i32),800302143_i32];
_33.3 = _37.0;
_10.0 = _25;
_45.1.3.0 = _17.3.0;
_29 = _32.fld1.5 + _17.0;
_24.2 = 95_u8;
_3 = _37.0;
_10.0 = _24.3 > _24.0;
_15 = _14;
_31 = _26.0;
_24.1 = _22;
_31 = _33.2;
RET.0 = core::ptr::addr_of_mut!(_24.2);
_17.4 = _32.fld1.4;
_10.1 = _17.3.1 >> _30;
RET.1 = [_24.1,_24.1,_24.1,_22,_24.1,_22,_24.1];
_10 = (_25, _17.3.1, _32.fld1.3.2);
_43 = [_33.3,_2,_4,_33.3,_13,_37.0,_33.3,_2];
_45.1.3.1 = _17.3.1;
Goto(bb16)
}
bb16 = {
Call(_48 = dump_var(16_usize, 34_usize, Move(_34), 35_usize, Move(_35), 9_usize, Move(_9), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(16_usize, 31_usize, Move(_31), 7_usize, Move(_7), 28_usize, Move(_28), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(16_usize, 15_usize, Move(_15), 37_usize, Move(_37), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(16_usize, 2_usize, Move(_2), 26_usize, Move(_26), 13_usize, Move(_13), 43_usize, Move(_43)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(16_usize, 4_usize, Move(_4), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [i32; 7],mut _6: isize,mut _7: isize,mut _8: u16) -> isize {
mir! {
type RET = isize;
let _9: ();
let _10: ();
{
_8 = !10531_u16;
RET = -_1;
_3 = 295823463006175697530833752974440827619_u128 as isize;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(17_usize, 1_usize, Move(_1), 6_usize, Move(_6), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(9223372036854775807_isize), std::hint::black_box(21536_u16));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: bool,
fld1: [i128; 6],
fld2: u16,
fld3: (isize,),
fld4: i16,
fld5: i32,
fld6: [bool; 6],
fld7: [i64; 1],

},
Variant1{
fld0: *const [i32; 7],

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: f64,
fld1: char,
fld2: i64,
fld3: [i64; 1],
fld4: i128,

},
Variant1{
fld0: bool,
fld1: char,
fld2: [i128; 6],
fld3: u32,
fld4: *const (bool, i64, u16),
fld5: i32,
fld6: (bool, i64, u16),
fld7: i128,

},
Variant2{
fld0: *const (bool, i64, u16),
fld1: (bool, i64, u16),
fld2: (u64, f32, i16, isize),
fld3: [isize; 8],
fld4: i16,
fld5: i32,
fld6: (isize,),

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: f32,

},
Variant1{
fld0: *const (bool, i64, u16),
fld1: Adt41,
fld2: (isize,),
fld3: (f32, char, u8, f32),
fld4: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
fld5: u32,

},
Variant2{
fld0: *mut (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
fld1: u64,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: u64,
fld1: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i64; 1],
fld1: char,
fld2: u16,
fld3: Adt42,

},
Variant1{
fld0: ([i32; 7], u64),
fld1: f64,
fld2: *const i128,
fld3: Adt41,
fld4: *mut (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
fld5: [i128; 6],
fld6: i64,
fld7: Adt40,

},
Variant2{
fld0: u32,
fld1: char,
fld2: *const (bool, i64, u16),
fld3: u8,

},
Variant3{
fld0: *const [i32; 7],
fld1: (f32, char, u8, f32),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: i128,
fld1: char,
fld2: Adt43,
fld3: [i64; 1],
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: (i16,),
fld1: *mut u8,
fld2: f64,
fld3: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)),

},
Variant1{
fld0: bool,
fld1: Adt41,
fld2: [isize; 8],
fld3: *const i128,
fld4: u64,
fld5: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
fld6: u128,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [bool; 6],
fld1: char,
fld2: Adt41,
fld3: u16,
fld4: Adt42,
fld5: (u64, f32, i16, isize),
fld6: *mut u8,
fld7: [i128; 6],

},
Variant1{
fld0: u32,
fld1: *const i128,
fld2: [usize; 2],
fld3: usize,
fld4: i16,
fld5: u64,
fld6: i64,

},
Variant2{
fld0: [i64; 1],
fld1: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
fld2: Adt45,
fld3: Adt42,
fld4: (i16,),
fld5: Adt43,
fld6: ([i32; 7], u64),

},
Variant3{
fld0: bool,
fld1: i64,
fld2: Adt41,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (u64, f32, i16, isize),
fld1: ([i32; 7], u64),
fld2: *const (bool, i64, u16),
fld3: (f32, char, u8, f32),

},
Variant1{
fld0: [i64; 1],
fld1: ([i32; 7], u64),
fld2: Adt40,
fld3: [usize; 2],

},
Variant2{
fld0: bool,
fld1: [char; 3],
fld2: [i64; 1],
fld3: i8,
fld4: f64,
fld5: Adt46,
fld6: (isize,),

},
Variant3{
fld0: u64,
fld1: [isize; 8],
fld2: isize,
fld3: (f32, char, u8, f32),
fld4: (u64, f32, i16, isize),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: bool,
fld1: [char; 7],
fld2: (f32, char, u8, f32),
fld3: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)),
fld4: f32,
fld5: (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32),
fld6: Adt47,

},
Variant1{
fld0: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)),
fld1: (u64, f32, i16, isize),
fld2: [usize; 2],
fld3: (i16,),
fld4: [i64; 1],
fld5: u16,

},
Variant2{
fld0: Adt47,
fld1: Adt46,
fld2: (*mut u8, [char; 7]),
fld3: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)),

},
Variant3{
fld0: (i16,),
fld1: char,
fld2: u128,
fld3: [char; 7],
fld4: f64,
fld5: u8,
fld6: ([i32; 7], u64),
fld7: f32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: *const (bool, i64, u16),
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (f32, char, u8, f32),
fld1: u8,
fld2: isize,
fld3: (u64, f32, i16, isize),
fld4: i16,
fld5: Adt48,
fld6: Adt43,

},
Variant1{
fld0: Adt49,

},
Variant2{
fld0: Adt49,
fld1: [bool; 6],
fld2: (*mut u8, [char; 7]),
fld3: u32,
fld4: (f32, char, u8, f32),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: i64,
fld1: Adt50,
fld2: isize,
fld3: Adt44,
fld4: (isize,),

},
Variant1{
fld0: Adt45,

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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: bool,
fld1: u128,
fld2: f32,
fld3: u64,
fld4: Adt47,
fld5: (f32, char, u8, f32),
fld6: [i32; 7],
fld7: (*mut u8, [char; 7]),

},
Variant1{
fld0: (*mut u8, [char; 7]),
fld1: (u64, f32, i16, isize),

},
Variant2{
fld0: Adt48,

},
Variant3{
fld0: [i128; 6],
fld1: Adt47,
fld2: Adt44,
fld3: Adt42,
fld4: i16,
fld5: [i64; 1],
fld6: [i32; 7],
fld7: (bool, i64, u16),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: u64,

},
Variant1{
fld0: Adt40,
fld1: (u64, f32, i16, isize),
fld2: Adt51,
fld3: (bool, i64, u16),
fld4: Adt48,
fld5: Adt49,

},
Variant2{
fld0: u32,
fld1: (u64, f32, i16, isize),
fld2: [char; 7],
fld3: (i16,),
fld4: ([i32; 7], u64),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt46,
fld1: [bool; 6],
fld2: *const i128,
fld3: usize,

},
Variant1{
fld0: [usize; 2],
fld1: isize,

},
Variant2{
fld0: i8,
fld1: char,
fld2: Adt51,

},
Variant3{
fld0: bool,
fld1: [char; 7],
fld2: *mut u8,
fld3: *const (bool, i64, u16),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: f64,
fld1: u128,
fld2: Adt55,
fld3: *const i128,

},
Variant1{
fld0: bool,
fld1: [isize; 8],
fld2: *const [i32; 7],
fld3: (i16,),
fld4: Adt54,
fld5: (f32, (u32, [i32; 7], i128, (bool, i64, u16), [i64; 1], u32)),
fld6: Adt42,

},
Variant2{
fld0: *mut u8,

},
Variant3{
fld0: char,

}}

