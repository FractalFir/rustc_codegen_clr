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
pub fn fn0(mut _1: u8,mut _2: char,mut _3: isize,mut _4: i8,mut _5: u64,mut _6: usize,mut _7: u32,mut _8: i128) -> isize {
mir! {
type RET = isize;
let _9: *const *mut f32;
let _10: (u32, u16, i128);
let _11: [u32; 8];
let _12: bool;
let _13: isize;
let _14: f64;
let _15: *const i128;
let _16: (isize, &'static isize, *mut f32, i8);
let _17: f64;
let _18: [u64; 4];
let _19: u128;
let _20: f64;
let _21: *mut f32;
let _22: isize;
let _23: u64;
let _24: bool;
let _25: *mut *const i16;
let _26: (u16,);
let _27: bool;
let _28: u64;
let _29: u8;
let _30: [u8; 7];
let _31: f64;
let _32: char;
let _33: usize;
let _34: ();
let _35: ();
{
RET = (-9223372036854775808_isize);
_6 = 144_u8 as usize;
_4 = (-378143432_i32) as i8;
_7 = false as u32;
_6 = 76868209355398908913520087493728479292_u128 as usize;
_6 = 7_usize << _4;
_1 = 207_u8 ^ 226_u8;
_6 = 7773793280314504955_usize & 12862047881845926198_usize;
_3 = RET;
_2 = '\u{1553c}';
_8 = true as i128;
_5 = 11588785622721929253_u64 >> _4;
_6 = !8504868144969282477_usize;
_2 = '\u{f3a54}';
_4 = 35_i8;
_5 = 11482458189534372568_u64;
_4 = 21_i8 - (-86_i8);
Goto(bb1)
}
bb1 = {
_8 = 2045557101_i32 as i128;
_2 = '\u{92d21}';
RET = -_3;
_3 = -RET;
_4 = -10_i8;
_1 = 93_u8;
_2 = '\u{32e20}';
_4 = (-100_i8);
_7 = _4 as u32;
_8 = (-98398685379907713739526923589458542296_i128);
RET = (-825276659_i32) as isize;
RET = -_3;
_4 = _8 as i8;
_10.0 = _7;
RET = _3 * _3;
_12 = true;
Goto(bb2)
}
bb2 = {
_7 = _10.0;
_15 = core::ptr::addr_of!(_8);
_5 = !2687097699806365280_u64;
_1 = 29_u8;
Goto(bb3)
}
bb3 = {
_13 = -RET;
_16.3 = _4;
Goto(bb4)
}
bb4 = {
_10.2 = -(*_15);
_16.1 = &_16.0;
match _1 {
0 => bb1,
29 => bb6,
_ => bb5
}
}
bb5 = {
_13 = -RET;
_16.3 = _4;
Goto(bb4)
}
bb6 = {
RET = -_13;
(*_15) = _10.2;
_16.1 = &_13;
_3 = !RET;
RET = _13 - _3;
_10.1 = 59618_u16;
_9 = core::ptr::addr_of!(_16.2);
_7 = _10.0;
_7 = _10.0;
_10.0 = _7;
Call(_10.2 = core::intrinsics::transmute(_8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = -RET;
Call((*_15) = fn1(RET, _10.1, _3, RET, _2, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = '\u{3a2d8}';
_10.1 = _7 as u16;
_3 = !_13;
_5 = !12924807053690323863_u64;
_2 = '\u{93610}';
_4 = _16.3 | _16.3;
_9 = core::ptr::addr_of!(_16.2);
_16.0 = _3 | _3;
_19 = 225546287446511063231233108216131671833_u128 & 295643723144286689884810457455356005657_u128;
RET = _13 ^ _16.0;
_22 = 190239039557382888_i64 as isize;
_8 = _10.2 + _10.2;
_5 = 12890487620586560229_u64;
RET = _10.1 as isize;
_12 = _16.0 != _16.0;
_18 = [_5,_5,_5,_5];
_3 = _16.0 ^ _16.0;
_19 = 129287948286393928736228499287744205237_u128 ^ 1540515015257130293644893386266137657_u128;
_15 = core::ptr::addr_of!(_8);
_18 = [_5,_5,_5,_5];
_24 = !_12;
_7 = !_10.0;
_23 = !_5;
_17 = _10.1 as f64;
_3 = _13 ^ _16.0;
_1 = 37_u8 * 51_u8;
Goto(bb9)
}
bb9 = {
_3 = _19 as isize;
_26.0 = _10.1 | _10.1;
_16.3 = 713918288_i32 as i8;
_11 = [_7,_7,_10.0,_7,_7,_7,_10.0,_7];
_18 = [_23,_23,_5,_5];
_12 = _24;
_26.0 = _19 as u16;
_18 = [_23,_5,_5,_5];
match _5 {
0 => bb8,
12890487620586560229 => bb11,
_ => bb10
}
}
bb10 = {
_2 = '\u{3a2d8}';
_10.1 = _7 as u16;
_3 = !_13;
_5 = !12924807053690323863_u64;
_2 = '\u{93610}';
_4 = _16.3 | _16.3;
_9 = core::ptr::addr_of!(_16.2);
_16.0 = _3 | _3;
_19 = 225546287446511063231233108216131671833_u128 & 295643723144286689884810457455356005657_u128;
RET = _13 ^ _16.0;
_22 = 190239039557382888_i64 as isize;
_8 = _10.2 + _10.2;
_5 = 12890487620586560229_u64;
RET = _10.1 as isize;
_12 = _16.0 != _16.0;
_18 = [_5,_5,_5,_5];
_3 = _16.0 ^ _16.0;
_19 = 129287948286393928736228499287744205237_u128 ^ 1540515015257130293644893386266137657_u128;
_15 = core::ptr::addr_of!(_8);
_18 = [_5,_5,_5,_5];
_24 = !_12;
_7 = !_10.0;
_23 = !_5;
_17 = _10.1 as f64;
_3 = _13 ^ _16.0;
_1 = 37_u8 * 51_u8;
Goto(bb9)
}
bb11 = {
_17 = _10.0 as f64;
RET = _26.0 as isize;
_15 = core::ptr::addr_of!((*_15));
_20 = -_17;
_12 = _24;
_2 = '\u{fc794}';
_3 = _22;
_16.0 = RET;
_10 = (_7, _26.0, (*_15));
_10.1 = !_26.0;
_6 = _24 as usize;
_18 = [_23,_5,_5,_23];
(*_15) = _12 as i128;
_14 = _17;
Goto(bb12)
}
bb12 = {
_27 = _16.0 > _22;
_2 = '\u{f5ad9}';
_24 = !_12;
Goto(bb13)
}
bb13 = {
_4 = -_16.3;
_13 = _16.0 * RET;
RET = _4 as isize;
_26 = (_10.1,);
Goto(bb14)
}
bb14 = {
_28 = _5 + _23;
_26.0 = !_10.1;
_12 = !_24;
_16.1 = &RET;
_10.2 = _8 << _8;
_10.1 = !_26.0;
_26.0 = !_10.1;
_31 = _20 - _17;
_16.1 = &_22;
_14 = _17;
_7 = _24 as u32;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(0_usize, 10_usize, Move(_10), 4_usize, Move(_4), 22_usize, Move(_22), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(0_usize, 27_usize, Move(_27), 3_usize, Move(_3), 28_usize, Move(_28), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(0_usize, 2_usize, Move(_2), 7_usize, Move(_7), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: u16,mut _3: isize,mut _4: isize,mut _5: char,mut _6: char) -> i128 {
mir! {
type RET = i128;
let _7: [u32; 1];
let _8: Adt75;
let _9: isize;
let _10: *const Adt57;
let _11: f64;
let _12: u8;
let _13: u128;
let _14: Adt50;
let _15: *const u128;
let _16: Adt33;
let _17: (f32,);
let _18: usize;
let _19: i32;
let _20: u16;
let _21: [i16; 7];
let _22: isize;
let _23: i32;
let _24: Adt50;
let _25: [i16; 7];
let _26: &'static *const i16;
let _27: [u128; 3];
let _28: bool;
let _29: f64;
let _30: u64;
let _31: &'static (*mut usize, i64);
let _32: u8;
let _33: &'static i64;
let _34: isize;
let _35: u16;
let _36: isize;
let _37: *mut u16;
let _38: [char; 2];
let _39: ();
let _40: ();
{
_4 = _3 | _1;
RET = !(-36753234912514186654564482024267942636_i128);
_7 = [3326514651_u32];
RET = 42_u8 as i128;
_4 = RET as isize;
_7 = [730734350_u32];
_7 = [3811126683_u32];
RET = 9690364688218257494343076659137689181_i128 - 35530436884460113236495810488815700465_i128;
_1 = _3;
_6 = _5;
RET = !(-113298522230150137224513344034858094134_i128);
_9 = RET as isize;
_7 = [2372972268_u32];
_6 = _5;
_2 = !17677_u16;
_5 = _6;
_5 = _6;
_6 = _5;
Call(_8 = fn2(_4, _1, _3, _1, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = Field::<isize>(Variant(_8, 0), 2);
RET = 16506756658171042405_u64 as i128;
place!(Field::<i64>(Variant(_8, 0), 0)) = (-8205035169231875955_i64) - (-2421185168149410449_i64);
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = (-1398987687_i32);
_3 = _4;
place!(Field::<i64>(Variant(_8, 0), 0)) = (-616635998666667818_i64);
RET = 5595826317299939274509971895416616290_i128;
place!(Field::<i64>(Variant(_8, 0), 0)) = 9023089580941405533_i64 + (-645567372691243549_i64);
_4 = 43493192852520869506719208455780338183_u128 as isize;
_3 = 13158697245005033473_u64 as isize;
_11 = (-28_i8) as f64;
_9 = Field::<isize>(Variant(_8, 0), 2);
_6 = _5;
RET = 69036897318649570882867916721243202969_i128 ^ 115053376686471895896488256910444122277_i128;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = ((-1008643094_i32),);
_3 = Field::<u16>(Variant(_8, 0), 5) as isize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _2 as i32;
_9 = Field::<isize>(Variant(_8, 0), 2) * Field::<isize>(Variant(_8, 0), 2);
SetDiscriminant(_8, 0);
_5 = _6;
_6 = _5;
place!(Field::<[u32; 8]>(Variant(_8, 0), 3)) = [4103439870_u32,3749667016_u32,2868965744_u32,1275847591_u32,1994632008_u32,2609595342_u32,4056276519_u32,1399625486_u32];
Goto(bb2)
}
bb2 = {
_1 = _9 >> _9;
_12 = 6_u8 & 234_u8;
_7 = [120176256_u32];
_5 = _6;
_15 = core::ptr::addr_of!(_13);
_13 = _11 as u128;
(*_15) = 118852924484931863633111346403366086153_u128;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = (360878473_i32,);
place!(Field::<u16>(Variant(_8, 0), 5)) = Field::<(i32,)>(Variant(_8, 0), 4).0 as u16;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = (1245525395_i32,);
_5 = _6;
_1 = _9 | _9;
_1 = -_9;
_7 = [2414441225_u32];
place!(Field::<isize>(Variant(_8, 0), 2)) = !_9;
Goto(bb3)
}
bb3 = {
place!(Field::<isize>(Variant(_8, 0), 2)) = -_9;
_9 = -Field::<isize>(Variant(_8, 0), 2);
(*_15) = !106425602872763839596089865899163973233_u128;
_9 = 1255975106_u32 as isize;
RET = 9141061581978542088_i64 as i128;
_18 = 10983712590642441416_usize;
_20 = _13 as u16;
place!(Field::<i64>(Variant(_8, 0), 0)) = !(-4829610203341175695_i64);
place!(Field::<isize>(Variant(_8, 0), 2)) = _1 & _1;
_21 = [24099_i16,(-25018_i16),8072_i16,(-2488_i16),23165_i16,(-7561_i16),30742_i16];
_18 = !622530689950209440_usize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = ((-1407466921_i32),);
(*_15) = RET as u128;
_22 = -Field::<isize>(Variant(_8, 0), 2);
_3 = _2 as isize;
_17.0 = _1 as f32;
_2 = _22 as u16;
place!(Field::<u16>(Variant(_8, 0), 5)) = !_2;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = -466280003_i32;
_3 = _1;
_21 = [11690_i16,25796_i16,20884_i16,31946_i16,15215_i16,(-22790_i16),(-5271_i16)];
_19 = Field::<(i32,)>(Variant(_8, 0), 4).0;
Goto(bb4)
}
bb4 = {
_21 = [2544_i16,21564_i16,19050_i16,4054_i16,5166_i16,(-10662_i16),16749_i16];
_27 = [(*_15),(*_15),(*_15)];
RET = -140580739820783184703557489849168687160_i128;
_23 = _19 >> Field::<u16>(Variant(_8, 0), 5);
_15 = core::ptr::addr_of!((*_15));
_6 = _5;
place!(Field::<[u32; 8]>(Variant(_8, 0), 3)) = [2956908039_u32,3035147788_u32,2821863563_u32,2054542208_u32,2065621270_u32,2694545406_u32,3878939391_u32,3423661067_u32];
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _23;
_15 = core::ptr::addr_of!((*_15));
_23 = Field::<(i32,)>(Variant(_8, 0), 4).0 ^ Field::<(i32,)>(Variant(_8, 0), 4).0;
_25 = _21;
_7 = [2751284245_u32];
_25 = _21;
_23 = _19 * Field::<(i32,)>(Variant(_8, 0), 4).0;
_28 = _3 == _22;
_18 = (-45_i8) as usize;
_30 = _5 as u64;
_29 = _11;
place!(Field::<i64>(Variant(_8, 0), 0)) = (-863727873279953890_i64);
_17.0 = 786354389_u32 as f32;
_25 = [31950_i16,30036_i16,19961_i16,(-17413_i16),15703_i16,(-16005_i16),19927_i16];
_30 = 10281047576478933085_u64 ^ 7326532685073833933_u64;
_21 = [(-1064_i16),31800_i16,7533_i16,10353_i16,30683_i16,15457_i16,(-5967_i16)];
_1 = _3;
Call(RET = core::intrinsics::bswap((-154517667773976014342496530031896457549_i128)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6 = _5;
place!(Field::<i64>(Variant(_8, 0), 0)) = _12 as i64;
_23 = -Field::<(i32,)>(Variant(_8, 0), 4).0;
_19 = Field::<(i32,)>(Variant(_8, 0), 4).0;
_7 = [2535270234_u32];
_32 = _12 << _3;
place!(Field::<u16>(Variant(_8, 0), 5)) = 10016_i16 as u16;
place!(Field::<isize>(Variant(_8, 0), 2)) = _22;
_15 = core::ptr::addr_of!((*_15));
_7 = [635855825_u32];
_11 = _29 + _29;
_6 = _5;
_27 = [(*_15),(*_15),_13];
_34 = !_22;
_23 = (-124_i8) as i32;
_9 = 3468232063_u32 as isize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = RET as i32;
_2 = !Field::<u16>(Variant(_8, 0), 5);
_19 = _23;
place!(Field::<isize>(Variant(_8, 0), 2)) = _34 - _1;
_13 = 257975710388518105393636253229039199290_u128;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _19 ^ _23;
_3 = _22;
_7 = [1463579551_u32];
Goto(bb6)
}
bb6 = {
_5 = _6;
_1 = _22;
_7 = [2731479648_u32];
_16 = Adt33::Variant3 { fld0: _27,fld1: _6 };
_35 = _2;
RET = !(-2252768381189292363368255820210079459_i128);
_35 = !Field::<u16>(Variant(_8, 0), 5);
_18 = (-18_i8) as usize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = 8500_i16 as i32;
match _13 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
257975710388518105393636253229039199290 => bb12,
_ => bb11
}
}
bb7 = {
_6 = _5;
place!(Field::<i64>(Variant(_8, 0), 0)) = _12 as i64;
_23 = -Field::<(i32,)>(Variant(_8, 0), 4).0;
_19 = Field::<(i32,)>(Variant(_8, 0), 4).0;
_7 = [2535270234_u32];
_32 = _12 << _3;
place!(Field::<u16>(Variant(_8, 0), 5)) = 10016_i16 as u16;
place!(Field::<isize>(Variant(_8, 0), 2)) = _22;
_15 = core::ptr::addr_of!((*_15));
_7 = [635855825_u32];
_11 = _29 + _29;
_6 = _5;
_27 = [(*_15),(*_15),_13];
_34 = !_22;
_23 = (-124_i8) as i32;
_9 = 3468232063_u32 as isize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = RET as i32;
_2 = !Field::<u16>(Variant(_8, 0), 5);
_19 = _23;
place!(Field::<isize>(Variant(_8, 0), 2)) = _34 - _1;
_13 = 257975710388518105393636253229039199290_u128;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _19 ^ _23;
_3 = _22;
_7 = [1463579551_u32];
Goto(bb6)
}
bb8 = {
_21 = [2544_i16,21564_i16,19050_i16,4054_i16,5166_i16,(-10662_i16),16749_i16];
_27 = [(*_15),(*_15),(*_15)];
RET = -140580739820783184703557489849168687160_i128;
_23 = _19 >> Field::<u16>(Variant(_8, 0), 5);
_15 = core::ptr::addr_of!((*_15));
_6 = _5;
place!(Field::<[u32; 8]>(Variant(_8, 0), 3)) = [2956908039_u32,3035147788_u32,2821863563_u32,2054542208_u32,2065621270_u32,2694545406_u32,3878939391_u32,3423661067_u32];
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _23;
_15 = core::ptr::addr_of!((*_15));
_23 = Field::<(i32,)>(Variant(_8, 0), 4).0 ^ Field::<(i32,)>(Variant(_8, 0), 4).0;
_25 = _21;
_7 = [2751284245_u32];
_25 = _21;
_23 = _19 * Field::<(i32,)>(Variant(_8, 0), 4).0;
_28 = _3 == _22;
_18 = (-45_i8) as usize;
_30 = _5 as u64;
_29 = _11;
place!(Field::<i64>(Variant(_8, 0), 0)) = (-863727873279953890_i64);
_17.0 = 786354389_u32 as f32;
_25 = [31950_i16,30036_i16,19961_i16,(-17413_i16),15703_i16,(-16005_i16),19927_i16];
_30 = 10281047576478933085_u64 ^ 7326532685073833933_u64;
_21 = [(-1064_i16),31800_i16,7533_i16,10353_i16,30683_i16,15457_i16,(-5967_i16)];
_1 = _3;
Call(RET = core::intrinsics::bswap((-154517667773976014342496530031896457549_i128)), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
place!(Field::<isize>(Variant(_8, 0), 2)) = -_9;
_9 = -Field::<isize>(Variant(_8, 0), 2);
(*_15) = !106425602872763839596089865899163973233_u128;
_9 = 1255975106_u32 as isize;
RET = 9141061581978542088_i64 as i128;
_18 = 10983712590642441416_usize;
_20 = _13 as u16;
place!(Field::<i64>(Variant(_8, 0), 0)) = !(-4829610203341175695_i64);
place!(Field::<isize>(Variant(_8, 0), 2)) = _1 & _1;
_21 = [24099_i16,(-25018_i16),8072_i16,(-2488_i16),23165_i16,(-7561_i16),30742_i16];
_18 = !622530689950209440_usize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = ((-1407466921_i32),);
(*_15) = RET as u128;
_22 = -Field::<isize>(Variant(_8, 0), 2);
_3 = _2 as isize;
_17.0 = _1 as f32;
_2 = _22 as u16;
place!(Field::<u16>(Variant(_8, 0), 5)) = !_2;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = -466280003_i32;
_3 = _1;
_21 = [11690_i16,25796_i16,20884_i16,31946_i16,15215_i16,(-22790_i16),(-5271_i16)];
_19 = Field::<(i32,)>(Variant(_8, 0), 4).0;
Goto(bb4)
}
bb10 = {
_1 = _9 >> _9;
_12 = 6_u8 & 234_u8;
_7 = [120176256_u32];
_5 = _6;
_15 = core::ptr::addr_of!(_13);
_13 = _11 as u128;
(*_15) = 118852924484931863633111346403366086153_u128;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = (360878473_i32,);
place!(Field::<u16>(Variant(_8, 0), 5)) = Field::<(i32,)>(Variant(_8, 0), 4).0 as u16;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = (1245525395_i32,);
_5 = _6;
_1 = _9 | _9;
_1 = -_9;
_7 = [2414441225_u32];
place!(Field::<isize>(Variant(_8, 0), 2)) = !_9;
Goto(bb3)
}
bb11 = {
_4 = Field::<isize>(Variant(_8, 0), 2);
RET = 16506756658171042405_u64 as i128;
place!(Field::<i64>(Variant(_8, 0), 0)) = (-8205035169231875955_i64) - (-2421185168149410449_i64);
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = (-1398987687_i32);
_3 = _4;
place!(Field::<i64>(Variant(_8, 0), 0)) = (-616635998666667818_i64);
RET = 5595826317299939274509971895416616290_i128;
place!(Field::<i64>(Variant(_8, 0), 0)) = 9023089580941405533_i64 + (-645567372691243549_i64);
_4 = 43493192852520869506719208455780338183_u128 as isize;
_3 = 13158697245005033473_u64 as isize;
_11 = (-28_i8) as f64;
_9 = Field::<isize>(Variant(_8, 0), 2);
_6 = _5;
RET = 69036897318649570882867916721243202969_i128 ^ 115053376686471895896488256910444122277_i128;
place!(Field::<(i32,)>(Variant(_8, 0), 4)) = ((-1008643094_i32),);
_3 = Field::<u16>(Variant(_8, 0), 5) as isize;
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _2 as i32;
_9 = Field::<isize>(Variant(_8, 0), 2) * Field::<isize>(Variant(_8, 0), 2);
SetDiscriminant(_8, 0);
_5 = _6;
_6 = _5;
place!(Field::<[u32; 8]>(Variant(_8, 0), 3)) = [4103439870_u32,3749667016_u32,2868965744_u32,1275847591_u32,1994632008_u32,2609595342_u32,4056276519_u32,1399625486_u32];
Goto(bb2)
}
bb12 = {
SetDiscriminant(_16, 3);
place!(Field::<(i32,)>(Variant(_8, 0), 4)).0 = _23 >> _3;
Goto(bb13)
}
bb13 = {
_27 = [(*_15),(*_15),_13];
_25 = [3957_i16,(-18230_i16),(-26900_i16),(-7207_i16),(-24794_i16),16359_i16,(-19234_i16)];
_25 = [(-18200_i16),(-26722_i16),(-25842_i16),2170_i16,(-11189_i16),(-23817_i16),29586_i16];
place!(Field::<[u32; 8]>(Variant(_8, 0), 3)) = [2381090605_u32,2280241540_u32,3605274566_u32,2906162154_u32,333476806_u32,2603482859_u32,1817610361_u32,756349921_u32];
_36 = -Field::<isize>(Variant(_8, 0), 2);
_23 = Field::<(i32,)>(Variant(_8, 0), 4).0;
_28 = !false;
_34 = _22 ^ _1;
_29 = _11;
place!(Field::<char>(Variant(_16, 3), 1)) = _6;
_15 = core::ptr::addr_of!((*_15));
_9 = _22;
_34 = -_1;
_17.0 = Field::<i64>(Variant(_8, 0), 0) as f32;
_9 = _36 | _3;
_23 = !Field::<(i32,)>(Variant(_8, 0), 4).0;
_4 = -_1;
_16 = Adt33::Variant3 { fld0: _27,fld1: _5 };
SetDiscriminant(_16, 2);
_11 = _29;
place!(Field::<i64>(Variant(_8, 0), 0)) = !(-4065777270865367451_i64);
Call(_13 = core::intrinsics::bswap(298210772802072877298387169182760392306_u128), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17.0 = _11 as f32;
_34 = _1;
_1 = Field::<isize>(Variant(_8, 0), 2) & _3;
_33 = &place!(Field::<i64>(Variant(_8, 0), 0));
place!(Field::<*mut usize>(Variant(_16, 2), 0)) = core::ptr::addr_of_mut!(_18);
_9 = !_34;
_35 = !Field::<u16>(Variant(_8, 0), 5);
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(1_usize, 1_usize, Move(_1), 23_usize, Move(_23), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(1_usize, 32_usize, Move(_32), 18_usize, Move(_18), 13_usize, Move(_13), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(1_usize, 5_usize, Move(_5), 20_usize, Move(_20), 34_usize, Move(_34), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> Adt75 {
mir! {
type RET = Adt75;
let _7: &'static i128;
let _8: ([u64; 4], (u16,));
let _9: u128;
let _10: i128;
let _11: (char, i16, char);
let _12: (char, i16, char);
let _13: (char, i16, char);
let _14: *const *mut f32;
let _15: &'static [u8; 5];
let _16: usize;
let _17: *const i128;
let _18: (isize, &'static isize, *mut f32, i8);
let _19: char;
let _20: (*mut i64, (char, i16, char));
let _21: &'static (f32,);
let _22: Adt25;
let _23: *mut Adt24;
let _24: f32;
let _25: char;
let _26: i8;
let _27: u64;
let _28: [u16; 8];
let _29: char;
let _30: f32;
let _31: isize;
let _32: i8;
let _33: f64;
let _34: f64;
let _35: f64;
let _36: Adt25;
let _37: u32;
let _38: i32;
let _39: isize;
let _40: [char; 2];
let _41: isize;
let _42: f64;
let _43: i128;
let _44: u64;
let _45: [i32; 3];
let _46: [i128; 4];
let _47: Adt75;
let _48: f64;
let _49: Adt50;
let _50: isize;
let _51: ();
let _52: ();
{
_3 = _6 + _4;
Goto(bb1)
}
bb1 = {
_5 = _3 & _3;
_6 = 26_i8 as isize;
_1 = -_5;
_4 = _3;
_2 = -_3;
_5 = _4;
_1 = _3 & _3;
_3 = _1 + _4;
_3 = _4 + _1;
_8.0 = [18342293001244467577_u64,1541362941614901139_u64,9522172373154669750_u64,8269137164476268551_u64];
_8.1.0 = !54959_u16;
_8.1.0 = 64625_u16 & 64097_u16;
_1 = _4;
_8.1 = (34764_u16,);
_6 = _5;
_4 = _3 | _3;
_8.1 = (46774_u16,);
_1 = (-3656264932954058266_i64) as isize;
match _8.1.0 {
0 => bb2,
46774 => bb4,
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
_6 = _4;
_5 = _3 | _2;
_7 = &_10;
_10 = '\u{2bb39}' as i128;
_11.0 = '\u{8b73f}';
_8.0 = [18341500608105900302_u64,7490014344556173473_u64,7450314121062294551_u64,4583993866244326981_u64];
_4 = _3 | _5;
_8.0 = [14826739940546982380_u64,6299814119792992183_u64,18254588940460198529_u64,10408481820112957093_u64];
Goto(bb5)
}
bb5 = {
_11.1 = 239_u8 as i16;
_10 = (-49347524126434438059472615201663328975_i128);
_12 = (_11.0, _11.1, _11.0);
_11.1 = _12.1;
_9 = !40482717821754293883919802071935564055_u128;
_11.1 = _10 as i16;
_12 = (_11.0, _11.1, _11.0);
_7 = &_10;
_1 = _5 * _4;
_2 = -_1;
_11 = (_12.2, _12.1, _12.2);
Call(_10 = fn3(_2, _6, _2, _6, _1, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = -(-125011401479835168552452657440714567324_i128);
_8.1 = (50243_u16,);
_11.0 = _11.2;
_4 = !_1;
_13.2 = _11.0;
_8.0 = [3334037111770489553_u64,14840136289372614856_u64,8196761592152235283_u64,9183469242461228358_u64];
_13.0 = _12.0;
_10 = 66668245615973509470855863481252309211_i128;
_5 = _2;
_12.0 = _11.0;
_4 = _2;
_4 = -_1;
_7 = &_10;
_10 = 149550704034456612541522449922823568826_i128 >> _5;
_8.0 = [4545055792556075700_u64,1761326362382838952_u64,13312704994419032788_u64,17802648047003506660_u64];
_13 = _11;
_4 = -_6;
_6 = !_4;
Goto(bb7)
}
bb7 = {
_16 = 3_usize * 12019223504046311153_usize;
_2 = _1 * _5;
_13.1 = -_12.1;
_2 = -_5;
_11 = (_13.2, _13.1, _13.2);
_12 = _13;
_6 = _10 as isize;
match _8.1.0 {
0 => bb3,
50243 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_8.0 = [1580646063841095421_u64,12614260082767839355_u64,15970446928551670694_u64,7759234938556359350_u64];
_8.1 = (47794_u16,);
_7 = &_10;
_18.0 = _5;
_20.1.1 = !_12.1;
_14 = core::ptr::addr_of!(_18.2);
_19 = _13.0;
_2 = _1 & _5;
_8.0 = [8549152271480439807_u64,15523814876627804027_u64,7207541464041446466_u64,17473339521608453184_u64];
_11.2 = _13.2;
_6 = _5 ^ _18.0;
Goto(bb10)
}
bb10 = {
_12.2 = _12.0;
_8.1.0 = _6 as u16;
_19 = _12.2;
_13.1 = _11.1;
_18.1 = &_3;
_6 = _18.0 << _1;
_20.1.0 = _12.2;
_5 = _1;
_20.1.2 = _11.0;
_11 = (_13.2, _12.1, _20.1.0);
_25 = _20.1.0;
_7 = &(*_7);
_8.1 = (14784_u16,);
(*_14) = core::ptr::addr_of_mut!(_24);
_18.3 = -100_i8;
_7 = &(*_7);
_12.2 = _11.2;
_16 = 5139676738165895455_usize;
_20.1.0 = _11.2;
_14 = core::ptr::addr_of!((*_14));
match _8.1.0 {
0 => bb8,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
14784 => bb16,
_ => bb15
}
}
bb11 = {
Return()
}
bb12 = {
_5 = _3 & _3;
_6 = 26_i8 as isize;
_1 = -_5;
_4 = _3;
_2 = -_3;
_5 = _4;
_1 = _3 & _3;
_3 = _1 + _4;
_3 = _4 + _1;
_8.0 = [18342293001244467577_u64,1541362941614901139_u64,9522172373154669750_u64,8269137164476268551_u64];
_8.1.0 = !54959_u16;
_8.1.0 = 64625_u16 & 64097_u16;
_1 = _4;
_8.1 = (34764_u16,);
_6 = _5;
_4 = _3 | _3;
_8.1 = (46774_u16,);
_1 = (-3656264932954058266_i64) as isize;
match _8.1.0 {
0 => bb2,
46774 => bb4,
_ => bb3
}
}
bb13 = {
_16 = 3_usize * 12019223504046311153_usize;
_2 = _1 * _5;
_13.1 = -_12.1;
_2 = -_5;
_11 = (_13.2, _13.1, _13.2);
_12 = _13;
_6 = _10 as isize;
match _8.1.0 {
0 => bb3,
50243 => bb9,
_ => bb8
}
}
bb14 = {
Return()
}
bb15 = {
_11.1 = 239_u8 as i16;
_10 = (-49347524126434438059472615201663328975_i128);
_12 = (_11.0, _11.1, _11.0);
_11.1 = _12.1;
_9 = !40482717821754293883919802071935564055_u128;
_11.1 = _10 as i16;
_12 = (_11.0, _11.1, _11.0);
_7 = &_10;
_1 = _5 * _4;
_2 = -_1;
_11 = (_12.2, _12.1, _12.2);
Call(_10 = fn3(_2, _6, _2, _6, _1, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
_5 = _4;
_4 = !_2;
_18.1 = &_6;
_8.0 = [18125466102204172308_u64,16323466567900236503_u64,4394233726995899180_u64,7171959282816666521_u64];
_28 = [_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0];
_5 = _1 * _2;
_18.3 = -107_i8;
_20.1.1 = !_11.1;
_8.0 = [565741217130223890_u64,14800990254614620262_u64,15686857032827778312_u64,12595911486256408175_u64];
_13.0 = _25;
(*_14) = core::ptr::addr_of_mut!(_24);
_5 = -_2;
_26 = _18.3 - _18.3;
_3 = false as isize;
_10 = (-102468353370567276475140703007280975337_i128) >> _18.0;
_5 = -_1;
_2 = _18.0 - _5;
_11 = (_13.2, _12.1, _25);
_10 = (-71050017023331399660350261651822323620_i128) + 76966622214319286238994511993288658903_i128;
_13 = (_20.1.0, _20.1.1, _20.1.0);
_12.1 = !_13.1;
_16 = 3_usize;
_24 = 88_u8 as f32;
_18.3 = -_26;
(*_14) = core::ptr::addr_of_mut!(_24);
Call(_20.0 = fn4(_20.1.0, _5, _2, _2, _2, _2, _18.0, _5, _1, _1), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_20.1.0 = _25;
_27 = !3195418193656248540_u64;
_6 = _4;
_2 = !_4;
_2 = -_6;
_8.1.0 = _10 as u16;
_3 = _4;
_8.1.0 = 2939_u16;
_17 = core::ptr::addr_of!(_10);
_18.2 = core::ptr::addr_of_mut!(_24);
_10 = !(-89664595648950684253403592745562607723_i128);
_17 = core::ptr::addr_of!((*_17));
_14 = core::ptr::addr_of!((*_14));
_12.0 = _25;
_9 = 175686361866022173670460068388006185598_u128;
_29 = _13.2;
_13 = _12;
_20.1.0 = _11.0;
_20.1.2 = _25;
match _16 {
0 => bb13,
1 => bb18,
2 => bb19,
4 => bb21,
5 => bb22,
6 => bb23,
3 => bb25,
_ => bb24
}
}
bb18 = {
_5 = _4;
_4 = !_2;
_18.1 = &_6;
_8.0 = [18125466102204172308_u64,16323466567900236503_u64,4394233726995899180_u64,7171959282816666521_u64];
_28 = [_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0];
_5 = _1 * _2;
_18.3 = -107_i8;
_20.1.1 = !_11.1;
_8.0 = [565741217130223890_u64,14800990254614620262_u64,15686857032827778312_u64,12595911486256408175_u64];
_13.0 = _25;
(*_14) = core::ptr::addr_of_mut!(_24);
_5 = -_2;
_26 = _18.3 - _18.3;
_3 = false as isize;
_10 = (-102468353370567276475140703007280975337_i128) >> _18.0;
_5 = -_1;
_2 = _18.0 - _5;
_11 = (_13.2, _12.1, _25);
_10 = (-71050017023331399660350261651822323620_i128) + 76966622214319286238994511993288658903_i128;
_13 = (_20.1.0, _20.1.1, _20.1.0);
_12.1 = !_13.1;
_16 = 3_usize;
_24 = 88_u8 as f32;
_18.3 = -_26;
(*_14) = core::ptr::addr_of_mut!(_24);
Call(_20.0 = fn4(_20.1.0, _5, _2, _2, _2, _2, _18.0, _5, _1, _1), ReturnTo(bb17), UnwindUnreachable())
}
bb19 = {
_11.1 = 239_u8 as i16;
_10 = (-49347524126434438059472615201663328975_i128);
_12 = (_11.0, _11.1, _11.0);
_11.1 = _12.1;
_9 = !40482717821754293883919802071935564055_u128;
_11.1 = _10 as i16;
_12 = (_11.0, _11.1, _11.0);
_7 = &_10;
_1 = _5 * _4;
_2 = -_1;
_11 = (_12.2, _12.1, _12.2);
Call(_10 = fn3(_2, _6, _2, _6, _1, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb20 = {
_6 = _4;
_5 = _3 | _2;
_7 = &_10;
_10 = '\u{2bb39}' as i128;
_11.0 = '\u{8b73f}';
_8.0 = [18341500608105900302_u64,7490014344556173473_u64,7450314121062294551_u64,4583993866244326981_u64];
_4 = _3 | _5;
_8.0 = [14826739940546982380_u64,6299814119792992183_u64,18254588940460198529_u64,10408481820112957093_u64];
Goto(bb5)
}
bb21 = {
_16 = 3_usize * 12019223504046311153_usize;
_2 = _1 * _5;
_13.1 = -_12.1;
_2 = -_5;
_11 = (_13.2, _13.1, _13.2);
_12 = _13;
_6 = _10 as isize;
match _8.1.0 {
0 => bb3,
50243 => bb9,
_ => bb8
}
}
bb22 = {
_5 = _3 & _3;
_6 = 26_i8 as isize;
_1 = -_5;
_4 = _3;
_2 = -_3;
_5 = _4;
_1 = _3 & _3;
_3 = _1 + _4;
_3 = _4 + _1;
_8.0 = [18342293001244467577_u64,1541362941614901139_u64,9522172373154669750_u64,8269137164476268551_u64];
_8.1.0 = !54959_u16;
_8.1.0 = 64625_u16 & 64097_u16;
_1 = _4;
_8.1 = (34764_u16,);
_6 = _5;
_4 = _3 | _3;
_8.1 = (46774_u16,);
_1 = (-3656264932954058266_i64) as isize;
match _8.1.0 {
0 => bb2,
46774 => bb4,
_ => bb3
}
}
bb23 = {
Return()
}
bb24 = {
_10 = -(-125011401479835168552452657440714567324_i128);
_8.1 = (50243_u16,);
_11.0 = _11.2;
_4 = !_1;
_13.2 = _11.0;
_8.0 = [3334037111770489553_u64,14840136289372614856_u64,8196761592152235283_u64,9183469242461228358_u64];
_13.0 = _12.0;
_10 = 66668245615973509470855863481252309211_i128;
_5 = _2;
_12.0 = _11.0;
_4 = _2;
_4 = -_1;
_7 = &_10;
_10 = 149550704034456612541522449922823568826_i128 >> _5;
_8.0 = [4545055792556075700_u64,1761326362382838952_u64,13312704994419032788_u64,17802648047003506660_u64];
_13 = _11;
_4 = -_6;
_6 = !_4;
Goto(bb7)
}
bb25 = {
_31 = _4;
match _8.1.0 {
0 => bb9,
1 => bb21,
2 => bb26,
3 => bb27,
2939 => bb29,
_ => bb28
}
}
bb26 = {
Return()
}
bb27 = {
_10 = -(-125011401479835168552452657440714567324_i128);
_8.1 = (50243_u16,);
_11.0 = _11.2;
_4 = !_1;
_13.2 = _11.0;
_8.0 = [3334037111770489553_u64,14840136289372614856_u64,8196761592152235283_u64,9183469242461228358_u64];
_13.0 = _12.0;
_10 = 66668245615973509470855863481252309211_i128;
_5 = _2;
_12.0 = _11.0;
_4 = _2;
_4 = -_1;
_7 = &_10;
_10 = 149550704034456612541522449922823568826_i128 >> _5;
_8.0 = [4545055792556075700_u64,1761326362382838952_u64,13312704994419032788_u64,17802648047003506660_u64];
_13 = _11;
_4 = -_6;
_6 = !_4;
Goto(bb7)
}
bb28 = {
_11.1 = 239_u8 as i16;
_10 = (-49347524126434438059472615201663328975_i128);
_12 = (_11.0, _11.1, _11.0);
_11.1 = _12.1;
_9 = !40482717821754293883919802071935564055_u128;
_11.1 = _10 as i16;
_12 = (_11.0, _11.1, _11.0);
_7 = &_10;
_1 = _5 * _4;
_2 = -_1;
_11 = (_12.2, _12.1, _12.2);
Call(_10 = fn3(_2, _6, _2, _6, _1, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb29 = {
_12.2 = _11.2;
_3 = _18.3 as isize;
_9 = 249894939391278446441665284055892178263_u128 - 238668996573754458555388600832995522571_u128;
_30 = (*_17) as f32;
_9 = 318367243897377802516032057081826385154_u128;
_8.1 = (45113_u16,);
_8.1 = (8280_u16,);
_18.0 = _4;
_24 = _30;
_28 = [_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0];
_20.1.2 = _12.0;
_13 = _12;
_10 = (-647842454_i32) as i128;
_34 = _18.3 as f64;
_8.0 = [_27,_27,_27,_27];
_2 = _25 as isize;
Call(_17 = fn5(_1, _6, _4, _6, _5, _4, _18.0, _1, _4), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_7 = &_10;
_27 = 17391973404801392892_u64 & 15086005582811051630_u64;
_8.1.0 = !31774_u16;
_7 = &(*_7);
_3 = true as isize;
_27 = !18001385296444396721_u64;
Goto(bb31)
}
bb31 = {
_13.0 = _13.2;
_11.1 = _12.1 >> _4;
_28 = [_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0];
_37 = _30 as u32;
_18.3 = _26;
_26 = !_18.3;
_3 = _4;
_20.1 = (_19, _11.1, _12.2);
_18.0 = _4 << _3;
_12.1 = _6 as i16;
_11.2 = _25;
_7 = &(*_7);
_35 = _34;
_25 = _13.0;
_25 = _12.2;
_12.1 = _11.1;
_39 = -_3;
RET = Adt75::Variant1 { fld0: Move(_17),fld1: Move(_14) };
place!(Field::<*const i128>(Variant(RET, 1), 0)) = core::ptr::addr_of!(_10);
SetDiscriminant(RET, 0);
_31 = _16 as isize;
_18.1 = &_3;
_34 = _35 + _35;
Goto(bb32)
}
bb32 = {
_31 = !_4;
_34 = -_35;
_40 = [_13.0,_25];
_13.1 = _20.1.1 & _11.1;
match _9 {
0 => bb12,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
5 => bb37,
318367243897377802516032057081826385154 => bb39,
_ => bb38
}
}
bb33 = {
Return()
}
bb34 = {
_5 = _4;
_4 = !_2;
_18.1 = &_6;
_8.0 = [18125466102204172308_u64,16323466567900236503_u64,4394233726995899180_u64,7171959282816666521_u64];
_28 = [_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0];
_5 = _1 * _2;
_18.3 = -107_i8;
_20.1.1 = !_11.1;
_8.0 = [565741217130223890_u64,14800990254614620262_u64,15686857032827778312_u64,12595911486256408175_u64];
_13.0 = _25;
(*_14) = core::ptr::addr_of_mut!(_24);
_5 = -_2;
_26 = _18.3 - _18.3;
_3 = false as isize;
_10 = (-102468353370567276475140703007280975337_i128) >> _18.0;
_5 = -_1;
_2 = _18.0 - _5;
_11 = (_13.2, _12.1, _25);
_10 = (-71050017023331399660350261651822323620_i128) + 76966622214319286238994511993288658903_i128;
_13 = (_20.1.0, _20.1.1, _20.1.0);
_12.1 = !_13.1;
_16 = 3_usize;
_24 = 88_u8 as f32;
_18.3 = -_26;
(*_14) = core::ptr::addr_of_mut!(_24);
Call(_20.0 = fn4(_20.1.0, _5, _2, _2, _2, _2, _18.0, _5, _1, _1), ReturnTo(bb17), UnwindUnreachable())
}
bb35 = {
_11.1 = 239_u8 as i16;
_10 = (-49347524126434438059472615201663328975_i128);
_12 = (_11.0, _11.1, _11.0);
_11.1 = _12.1;
_9 = !40482717821754293883919802071935564055_u128;
_11.1 = _10 as i16;
_12 = (_11.0, _11.1, _11.0);
_7 = &_10;
_1 = _5 * _4;
_2 = -_1;
_11 = (_12.2, _12.1, _12.2);
Call(_10 = fn3(_2, _6, _2, _6, _1, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb36 = {
_6 = _4;
_5 = _3 | _2;
_7 = &_10;
_10 = '\u{2bb39}' as i128;
_11.0 = '\u{8b73f}';
_8.0 = [18341500608105900302_u64,7490014344556173473_u64,7450314121062294551_u64,4583993866244326981_u64];
_4 = _3 | _5;
_8.0 = [14826739940546982380_u64,6299814119792992183_u64,18254588940460198529_u64,10408481820112957093_u64];
Goto(bb5)
}
bb37 = {
_10 = -(-125011401479835168552452657440714567324_i128);
_8.1 = (50243_u16,);
_11.0 = _11.2;
_4 = !_1;
_13.2 = _11.0;
_8.0 = [3334037111770489553_u64,14840136289372614856_u64,8196761592152235283_u64,9183469242461228358_u64];
_13.0 = _12.0;
_10 = 66668245615973509470855863481252309211_i128;
_5 = _2;
_12.0 = _11.0;
_4 = _2;
_4 = -_1;
_7 = &_10;
_10 = 149550704034456612541522449922823568826_i128 >> _5;
_8.0 = [4545055792556075700_u64,1761326362382838952_u64,13312704994419032788_u64,17802648047003506660_u64];
_13 = _11;
_4 = -_6;
_6 = !_4;
Goto(bb7)
}
bb38 = {
_5 = _4;
_4 = !_2;
_18.1 = &_6;
_8.0 = [18125466102204172308_u64,16323466567900236503_u64,4394233726995899180_u64,7171959282816666521_u64];
_28 = [_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0,_8.1.0];
_5 = _1 * _2;
_18.3 = -107_i8;
_20.1.1 = !_11.1;
_8.0 = [565741217130223890_u64,14800990254614620262_u64,15686857032827778312_u64,12595911486256408175_u64];
_13.0 = _25;
(*_14) = core::ptr::addr_of_mut!(_24);
_5 = -_2;
_26 = _18.3 - _18.3;
_3 = false as isize;
_10 = (-102468353370567276475140703007280975337_i128) >> _18.0;
_5 = -_1;
_2 = _18.0 - _5;
_11 = (_13.2, _12.1, _25);
_10 = (-71050017023331399660350261651822323620_i128) + 76966622214319286238994511993288658903_i128;
_13 = (_20.1.0, _20.1.1, _20.1.0);
_12.1 = !_13.1;
_16 = 3_usize;
_24 = 88_u8 as f32;
_18.3 = -_26;
(*_14) = core::ptr::addr_of_mut!(_24);
Call(_20.0 = fn4(_20.1.0, _5, _2, _2, _2, _2, _18.0, _5, _1, _1), ReturnTo(bb17), UnwindUnreachable())
}
bb39 = {
_20.1.2 = _13.2;
_20.1.0 = _11.0;
place!(Field::<*const *mut f32>(Variant(RET, 0), 1)) = core::ptr::addr_of!(_18.2);
_13.0 = _13.2;
_10 = (-98851878676120022902402850083804066948_i128);
_22 = Adt25::Variant3 { fld0: _27 };
_31 = _18.0;
place!(Field::<u16>(Variant(RET, 0), 5)) = _8.1.0;
place!(Field::<[u32; 8]>(Variant(RET, 0), 3)) = [_37,_37,_37,_37,_37,_37,_37,_37];
_36 = Move(_22);
_8.1 = (Field::<u16>(Variant(RET, 0), 5),);
_25 = _20.1.0;
_18.1 = &_39;
_20.1.0 = _25;
place!(Field::<u64>(Variant(_36, 3), 0)) = _27 * _27;
place!(Field::<(i32,)>(Variant(RET, 0), 4)).0 = -1607816688_i32;
_20.1.1 = -_12.1;
_20.1.1 = _11.1 - _13.1;
place!(Field::<[u32; 8]>(Variant(RET, 0), 3)) = [_37,_37,_37,_37,_37,_37,_37,_37];
_24 = 108_u8 as f32;
match _10 {
0 => bb1,
1 => bb28,
2 => bb37,
3 => bb36,
4 => bb31,
5 => bb40,
6 => bb41,
241430488244818440560971757347964144508 => bb43,
_ => bb42
}
}
bb40 = {
_11.1 = 239_u8 as i16;
_10 = (-49347524126434438059472615201663328975_i128);
_12 = (_11.0, _11.1, _11.0);
_11.1 = _12.1;
_9 = !40482717821754293883919802071935564055_u128;
_11.1 = _10 as i16;
_12 = (_11.0, _11.1, _11.0);
_7 = &_10;
_1 = _5 * _4;
_2 = -_1;
_11 = (_12.2, _12.1, _12.2);
Call(_10 = fn3(_2, _6, _2, _6, _1, _2, _11.0), ReturnTo(bb6), UnwindUnreachable())
}
bb41 = {
_5 = _3 & _3;
_6 = 26_i8 as isize;
_1 = -_5;
_4 = _3;
_2 = -_3;
_5 = _4;
_1 = _3 & _3;
_3 = _1 + _4;
_3 = _4 + _1;
_8.0 = [18342293001244467577_u64,1541362941614901139_u64,9522172373154669750_u64,8269137164476268551_u64];
_8.1.0 = !54959_u16;
_8.1.0 = 64625_u16 & 64097_u16;
_1 = _4;
_8.1 = (34764_u16,);
_6 = _5;
_4 = _3 | _3;
_8.1 = (46774_u16,);
_1 = (-3656264932954058266_i64) as isize;
match _8.1.0 {
0 => bb2,
46774 => bb4,
_ => bb3
}
}
bb42 = {
_31 = !_4;
_34 = -_35;
_40 = [_13.0,_25];
_13.1 = _20.1.1 & _11.1;
match _9 {
0 => bb12,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
5 => bb37,
318367243897377802516032057081826385154 => bb39,
_ => bb38
}
}
bb43 = {
place!(Field::<[u32; 8]>(Variant(RET, 0), 3)) = [_37,_37,_37,_37,_37,_37,_37,_37];
place!(Field::<u16>(Variant(RET, 0), 5)) = _8.1.0;
place!(Field::<(i32,)>(Variant(RET, 0), 4)).0 = (-474949970_i32);
_20.1.2 = _19;
_29 = _13.0;
match _16 {
0 => bb7,
1 => bb13,
2 => bb39,
3 => bb44,
_ => bb4
}
}
bb44 = {
_41 = _5;
_5 = _18.0;
_24 = -_30;
_29 = _20.1.0;
_26 = _3 as i8;
place!(Field::<isize>(Variant(RET, 0), 2)) = _3 ^ _6;
_7 = &_43;
_31 = _41;
_12.1 = _16 as i16;
_34 = -_35;
_17 = core::ptr::addr_of!((*_7));
_46 = [_10,_10,_10,_10];
_43 = !_10;
Goto(bb45)
}
bb45 = {
_18.1 = &_4;
_13.1 = _20.1.1;
_43 = !_10;
_13.0 = _19;
_46 = [_43,_10,_43,_43];
_42 = 181_u8 as f64;
place!(Field::<i64>(Variant(RET, 0), 0)) = 8821780076376132173_i64 | (-2654997637983219278_i64);
_34 = Field::<u16>(Variant(RET, 0), 5) as f64;
Goto(bb46)
}
bb46 = {
Call(_51 = dump_var(2_usize, 27_usize, Move(_27), 1_usize, Move(_1), 19_usize, Move(_19), 40_usize, Move(_40)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_51 = dump_var(2_usize, 29_usize, Move(_29), 41_usize, Move(_41), 26_usize, Move(_26), 25_usize, Move(_25)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_51 = dump_var(2_usize, 37_usize, Move(_37), 46_usize, Move(_46), 31_usize, Move(_31), 11_usize, Move(_11)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_51 = dump_var(2_usize, 2_usize, Move(_2), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: char) -> i128 {
mir! {
type RET = i128;
let _8: ((char, i16, char), &'static *const usize, (i32,), *mut u16);
let _9: &'static &'static u16;
let _10: (u32, u16, i128);
let _11: isize;
let _12: isize;
let _13: [char; 2];
let _14: ();
let _15: ();
{
_4 = _6;
_5 = _3;
_3 = !_4;
_2 = (-1649395879_i32) as isize;
Goto(bb1)
}
bb1 = {
RET = 109066515658517786533589763397244482428_i128;
_2 = _3;
_5 = RET as isize;
_8.2.0 = -(-851046823_i32);
RET = 91036106110529303610464590054895545759_i128 - (-96478293081062676612331544107786032822_i128);
_7 = '\u{9d547}';
_8.0.2 = _7;
RET = -95705799606895143316502613490015914972_i128;
_8.0 = (_7, 27302_i16, _7);
_6 = _4 & _2;
_3 = _2 * _6;
_10.1 = !6242_u16;
_10 = (600495391_u32, 31158_u16, RET);
_8.2 = (1552871011_i32,);
_8.0.2 = _8.0.0;
_6 = _2;
_8.3 = core::ptr::addr_of_mut!(_10.1);
_6 = -_3;
Goto(bb2)
}
bb2 = {
RET = _10.2;
match _10.0 {
0 => bb1,
600495391 => bb4,
_ => bb3
}
}
bb3 = {
RET = 109066515658517786533589763397244482428_i128;
_2 = _3;
_5 = RET as isize;
_8.2.0 = -(-851046823_i32);
RET = 91036106110529303610464590054895545759_i128 - (-96478293081062676612331544107786032822_i128);
_7 = '\u{9d547}';
_8.0.2 = _7;
RET = -95705799606895143316502613490015914972_i128;
_8.0 = (_7, 27302_i16, _7);
_6 = _4 & _2;
_3 = _2 * _6;
_10.1 = !6242_u16;
_10 = (600495391_u32, 31158_u16, RET);
_8.2 = (1552871011_i32,);
_8.0.2 = _8.0.0;
_6 = _2;
_8.3 = core::ptr::addr_of_mut!(_10.1);
_6 = -_3;
Goto(bb2)
}
bb4 = {
_11 = _6;
RET = 16604669119513367389_u64 as i128;
_10.2 = 2_usize as i128;
_10.0 = _8.2.0 as u32;
_4 = _11;
RET = _10.2 >> _4;
_4 = -_1;
_10.2 = RET ^ RET;
_3 = -_6;
_7 = _8.0.0;
_5 = 4454564179870910504_i64 as isize;
_10.0 = 818814105_u32;
Goto(bb5)
}
bb5 = {
Call(_14 = dump_var(3_usize, 6_usize, Move(_6), 2_usize, Move(_2), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: char,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> *mut i64 {
mir! {
type RET = *mut i64;
let _11: (u32, *const *mut f32, Adt53, Adt33);
let _12: char;
let _13: [char; 2];
let _14: bool;
let _15: isize;
let _16: (*mut usize, i64);
let _17: bool;
let _18: char;
let _19: ();
let _20: ();
{
_5 = !_9;
_8 = _5 + _9;
_2 = !_6;
_8 = _3;
_9 = _2 >> _5;
_11.0 = 69_u8 as u32;
_8 = _2 * _10;
_9 = !_8;
_9 = _10;
_4 = _8 >> _5;
_8 = (-9053_i16) as isize;
_8 = -_3;
_11.0 = 3474718048_u32 & 710479573_u32;
_12 = _1;
_2 = _4;
_4 = -_5;
_9 = _6;
_8 = 8021792087537226995_u64 as isize;
_1 = _12;
_1 = _12;
_10 = -_3;
_6 = _2;
_8 = 9451090764862836460_u64 as isize;
_13 = [_12,_12];
_11.0 = 478834632_u32 - 2229330023_u32;
_11.0 = !2218336829_u32;
_11.0 = !358387493_u32;
Goto(bb1)
}
bb1 = {
_7 = (-7139631608561277590756885813425518300_i128) as isize;
_2 = _6;
_5 = !_2;
_15 = !_3;
_13 = [_1,_12];
Call(_15 = core::intrinsics::bswap(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = !_9;
_14 = true;
_8 = _2 + _9;
_14 = false & false;
_4 = (-941823373_i32) as isize;
_7 = -_8;
_16.1 = (-8632485039276799117_i64) * (-235004832645264674_i64);
RET = core::ptr::addr_of_mut!(_16.1);
_13 = [_12,_12];
_5 = _8 * _15;
_14 = false;
_3 = 205_u8 as isize;
(*RET) = !(-7965289082820008224_i64);
RET = core::ptr::addr_of_mut!((*RET));
_17 = _2 > _6;
(*RET) = _17 as i64;
_7 = !_9;
_3 = _6 * _2;
_12 = _1;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 7249894638448595935_i64;
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(4_usize, 9_usize, Move(_9), 4_usize, Move(_4), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_19 = dump_var(4_usize, 3_usize, Move(_3), 8_usize, Move(_8), 15_usize, Move(_15), 20_usize, _20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize) -> *const i128 {
mir! {
type RET = *const i128;
let _10: f32;
let _11: char;
let _12: [i32; 3];
let _13: u32;
let _14: isize;
let _15: u64;
let _16: f32;
let _17: isize;
let _18: Adt33;
let _19: f64;
let _20: [u8; 7];
let _21: (char, i16, char);
let _22: &'static (f32,);
let _23: [u32; 1];
let _24: Adt25;
let _25: i128;
let _26: *mut u32;
let _27: Adt73;
let _28: isize;
let _29: [i32; 8];
let _30: [i16; 7];
let _31: [i128; 4];
let _32: *const Adt53;
let _33: &'static (*mut usize, i64);
let _34: u16;
let _35: ();
let _36: ();
{
_8 = _3;
_6 = _2 ^ _2;
_8 = _7;
_9 = _6 - _6;
_7 = 16971062662248554010_u64 as isize;
_1 = '\u{cd78}' as isize;
_10 = (-14048_i16) as f32;
_8 = false as isize;
_7 = _6;
_2 = !_4;
_2 = _9 * _5;
_1 = !_7;
_5 = _1 >> _6;
_4 = -_6;
_2 = 46628_u16 as isize;
Goto(bb1)
}
bb1 = {
_1 = _5 ^ _3;
Goto(bb2)
}
bb2 = {
_11 = '\u{a19aa}';
_6 = -_4;
_2 = 1267650476_i32 as isize;
_4 = _9 << _6;
_5 = _10 as isize;
_3 = !_9;
Call(_10 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = '\u{42eaf}';
_4 = 3655078559_u32 as isize;
_11 = '\u{481b0}';
_12 = [(-2116915813_i32),372108110_i32,(-669417934_i32)];
_2 = 9003664874431935085_usize as isize;
_3 = -_1;
_8 = (-89441863607058551138031483847318299122_i128) as isize;
_7 = _9 * _6;
_6 = 252746291294400047894621025221460375710_u128 as isize;
_10 = _1 as f32;
_7 = _9 - _3;
_3 = _7 | _7;
_6 = _11 as isize;
_4 = !_3;
_2 = _7;
_8 = -_7;
_2 = !_4;
_3 = _2 - _4;
_11 = '\u{ac92c}';
_13 = !745735711_u32;
_15 = !17344399119301453784_u64;
_5 = _4;
_1 = -_5;
_13 = !3941595370_u32;
_14 = _13 as isize;
_3 = _4;
Goto(bb4)
}
bb4 = {
_2 = -_3;
_9 = _5;
_7 = !_8;
_11 = '\u{9f708}';
_14 = _11 as isize;
_15 = 1755312266418635630_u64 | 15701525222118308136_u64;
_12 = [(-107467298_i32),700807466_i32,1930984411_i32];
_13 = _11 as u32;
_15 = !12307265315174882962_u64;
_1 = _11 as isize;
_9 = -_5;
_8 = -_5;
_9 = _8 >> _7;
_15 = !3951705803136830610_u64;
_6 = _9 << _8;
_7 = !_3;
_5 = _9 + _4;
_5 = _6 >> _2;
_10 = 187_u8 as f32;
_9 = _15 as isize;
Call(_12 = fn6(_7, _7, _4, _7, _2, _8, _4, _4, _2, _4, _3, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = 563293107_i32 as isize;
Call(_15 = core::intrinsics::bswap(4640314038812275825_u64), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_17 = !_5;
_19 = 68564810434355687047641885356756794220_u128 as f64;
_15 = 11707610053292647282_u64 | 9787847147285235734_u64;
_11 = '\u{d3c1f}';
_16 = _10;
_17 = -_4;
_2 = -_3;
_20 = [60_u8,246_u8,37_u8,154_u8,37_u8,144_u8,127_u8];
_10 = _16;
_4 = _2;
_20 = [67_u8,94_u8,79_u8,62_u8,143_u8,32_u8,135_u8];
_6 = _15 as isize;
Goto(bb7)
}
bb7 = {
_15 = 851665740361357213_u64;
_1 = (-8787733010941308307_i64) as isize;
_19 = _16 as f64;
_9 = !_6;
match _15 {
0 => bb5,
1 => bb6,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
851665740361357213 => bb12,
_ => bb11
}
}
bb8 = {
_1 = _5 ^ _3;
Goto(bb2)
}
bb9 = {
_11 = '\u{a19aa}';
_6 = -_4;
_2 = 1267650476_i32 as isize;
_4 = _9 << _6;
_5 = _10 as isize;
_3 = !_9;
Call(_10 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_2 = -_3;
_9 = _5;
_7 = !_8;
_11 = '\u{9f708}';
_14 = _11 as isize;
_15 = 1755312266418635630_u64 | 15701525222118308136_u64;
_12 = [(-107467298_i32),700807466_i32,1930984411_i32];
_13 = _11 as u32;
_15 = !12307265315174882962_u64;
_1 = _11 as isize;
_9 = -_5;
_8 = -_5;
_9 = _8 >> _7;
_15 = !3951705803136830610_u64;
_6 = _9 << _8;
_7 = !_3;
_5 = _9 + _4;
_5 = _6 >> _2;
_10 = 187_u8 as f32;
_9 = _15 as isize;
Call(_12 = fn6(_7, _7, _4, _7, _2, _8, _4, _4, _2, _4, _3, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_11 = '\u{42eaf}';
_4 = 3655078559_u32 as isize;
_11 = '\u{481b0}';
_12 = [(-2116915813_i32),372108110_i32,(-669417934_i32)];
_2 = 9003664874431935085_usize as isize;
_3 = -_1;
_8 = (-89441863607058551138031483847318299122_i128) as isize;
_7 = _9 * _6;
_6 = 252746291294400047894621025221460375710_u128 as isize;
_10 = _1 as f32;
_7 = _9 - _3;
_3 = _7 | _7;
_6 = _11 as isize;
_4 = !_3;
_2 = _7;
_8 = -_7;
_2 = !_4;
_3 = _2 - _4;
_11 = '\u{ac92c}';
_13 = !745735711_u32;
_15 = !17344399119301453784_u64;
_5 = _4;
_1 = -_5;
_13 = !3941595370_u32;
_14 = _13 as isize;
_3 = _4;
Goto(bb4)
}
bb12 = {
_9 = _7;
_15 = 2956225433193853665_u64;
_19 = 2_usize as f64;
_6 = !_4;
_16 = 2494903076868591941_i64 as f32;
_10 = _16;
_9 = !_14;
_7 = _5 >> _3;
_13 = 2731407878_u32 | 3906366129_u32;
_17 = _4 ^ _4;
match _15 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb13,
2956225433193853665 => bb15,
_ => bb14
}
}
bb13 = {
_11 = '\u{a19aa}';
_6 = -_4;
_2 = 1267650476_i32 as isize;
_4 = _9 << _6;
_5 = _10 as isize;
_3 = !_9;
Call(_10 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_1 = _5 ^ _3;
Goto(bb2)
}
bb15 = {
_5 = _6 | _8;
_16 = -_10;
_20 = [151_u8,117_u8,101_u8,215_u8,209_u8,9_u8,140_u8];
_21.0 = _11;
_9 = _7 | _4;
_13 = _21.0 as u32;
_21.2 = _11;
_21.1 = 12899_i16;
_4 = _15 as isize;
_10 = _16;
_21 = (_11, 17423_i16, _11);
_2 = _11 as isize;
_21.1 = 5859919469154991358_i64 as i16;
_21 = (_11, (-24002_i16), _11);
_12 = [(-1069258124_i32),1398786834_i32,1754152854_i32];
_6 = _5 << _17;
_7 = 85459879_i32 as isize;
_2 = _5 ^ _4;
_1 = !_5;
_21 = (_11, 27933_i16, _11);
Call(_5 = fn19(_2, _8, _1, _1, _8, _3, _9, _8, _17, _8), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_20 = [142_u8,175_u8,54_u8,30_u8,230_u8,40_u8,111_u8];
_13 = !891624499_u32;
_19 = _1 as f64;
_13 = 35210_u16 as u32;
_23 = [_13];
_13 = !1957773008_u32;
_15 = 8535640894092310589_u64;
_6 = !_5;
_9 = _17 >> _6;
_4 = -_17;
_4 = _2 + _8;
_16 = _10;
_15 = 16541041231863653832_u64 << _1;
Goto(bb17)
}
bb17 = {
_26 = core::ptr::addr_of_mut!(_13);
_21.2 = _21.0;
_28 = _4 >> _1;
_20 = [154_u8,247_u8,149_u8,117_u8,83_u8,45_u8,169_u8];
_10 = -_16;
_26 = core::ptr::addr_of_mut!(_13);
_7 = _28;
match _21.1 {
0 => bb4,
1 => bb2,
2 => bb18,
3 => bb19,
4 => bb20,
5 => bb21,
27933 => bb23,
_ => bb22
}
}
bb18 = {
_20 = [142_u8,175_u8,54_u8,30_u8,230_u8,40_u8,111_u8];
_13 = !891624499_u32;
_19 = _1 as f64;
_13 = 35210_u16 as u32;
_23 = [_13];
_13 = !1957773008_u32;
_15 = 8535640894092310589_u64;
_6 = !_5;
_9 = _17 >> _6;
_4 = -_17;
_4 = _2 + _8;
_16 = _10;
_15 = 16541041231863653832_u64 << _1;
Goto(bb17)
}
bb19 = {
_5 = _6 | _8;
_16 = -_10;
_20 = [151_u8,117_u8,101_u8,215_u8,209_u8,9_u8,140_u8];
_21.0 = _11;
_9 = _7 | _4;
_13 = _21.0 as u32;
_21.2 = _11;
_21.1 = 12899_i16;
_4 = _15 as isize;
_10 = _16;
_21 = (_11, 17423_i16, _11);
_2 = _11 as isize;
_21.1 = 5859919469154991358_i64 as i16;
_21 = (_11, (-24002_i16), _11);
_12 = [(-1069258124_i32),1398786834_i32,1754152854_i32];
_6 = _5 << _17;
_7 = 85459879_i32 as isize;
_2 = _5 ^ _4;
_1 = !_5;
_21 = (_11, 27933_i16, _11);
Call(_5 = fn19(_2, _8, _1, _1, _8, _3, _9, _8, _17, _8), ReturnTo(bb16), UnwindUnreachable())
}
bb20 = {
_11 = '\u{a19aa}';
_6 = -_4;
_2 = 1267650476_i32 as isize;
_4 = _9 << _6;
_5 = _10 as isize;
_3 = !_9;
Call(_10 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb21 = {
_11 = '\u{a19aa}';
_6 = -_4;
_2 = 1267650476_i32 as isize;
_4 = _9 << _6;
_5 = _10 as isize;
_3 = !_9;
Call(_10 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb22 = {
_2 = -_3;
_9 = _5;
_7 = !_8;
_11 = '\u{9f708}';
_14 = _11 as isize;
_15 = 1755312266418635630_u64 | 15701525222118308136_u64;
_12 = [(-107467298_i32),700807466_i32,1930984411_i32];
_13 = _11 as u32;
_15 = !12307265315174882962_u64;
_1 = _11 as isize;
_9 = -_5;
_8 = -_5;
_9 = _8 >> _7;
_15 = !3951705803136830610_u64;
_6 = _9 << _8;
_7 = !_3;
_5 = _9 + _4;
_5 = _6 >> _2;
_10 = 187_u8 as f32;
_9 = _15 as isize;
Call(_12 = fn6(_7, _7, _4, _7, _2, _8, _4, _4, _2, _4, _3, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb23 = {
_20 = [32_u8,156_u8,152_u8,215_u8,4_u8,129_u8,117_u8];
_25 = _10 as i128;
(*_26) = _25 as u32;
_19 = _15 as f64;
_16 = _10 * _10;
_26 = core::ptr::addr_of_mut!(_13);
match _21.1 {
0 => bb1,
1 => bb14,
2 => bb19,
3 => bb21,
4 => bb24,
27933 => bb26,
_ => bb25
}
}
bb24 = {
_2 = -_3;
_9 = _5;
_7 = !_8;
_11 = '\u{9f708}';
_14 = _11 as isize;
_15 = 1755312266418635630_u64 | 15701525222118308136_u64;
_12 = [(-107467298_i32),700807466_i32,1930984411_i32];
_13 = _11 as u32;
_15 = !12307265315174882962_u64;
_1 = _11 as isize;
_9 = -_5;
_8 = -_5;
_9 = _8 >> _7;
_15 = !3951705803136830610_u64;
_6 = _9 << _8;
_7 = !_3;
_5 = _9 + _4;
_5 = _6 >> _2;
_10 = 187_u8 as f32;
_9 = _15 as isize;
Call(_12 = fn6(_7, _7, _4, _7, _2, _8, _4, _4, _2, _4, _3, _8), ReturnTo(bb5), UnwindUnreachable())
}
bb25 = {
_1 = _5 ^ _3;
Goto(bb2)
}
bb26 = {
_26 = core::ptr::addr_of_mut!((*_26));
_29 = [1676662299_i32,(-1172629325_i32),(-1306804299_i32),288219396_i32,(-446578819_i32),1658806548_i32,1385297678_i32,(-1732645282_i32)];
(*_26) = 391633746_u32;
RET = core::ptr::addr_of!(_25);
_27 = Adt73::Variant0 { fld0: 221607834939887689230883020289127530670_u128 };
_21.1 = (-23_i8) as i16;
_25 = (-89580008234622240059694632277324818008_i128);
_11 = _21.0;
_34 = 42751_u16;
Goto(bb27)
}
bb27 = {
Call(_35 = dump_var(5_usize, 11_usize, Move(_11), 29_usize, Move(_29), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_35 = dump_var(5_usize, 8_usize, Move(_8), 3_usize, Move(_3), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_35 = dump_var(5_usize, 2_usize, Move(_2), 15_usize, Move(_15), 7_usize, Move(_7), 36_usize, _36), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _13: &'static f64;
let _14: f64;
let _15: [u128; 3];
let _16: f32;
let _17: bool;
let _18: (&'static isize, Adt24, i16);
let _19: *mut usize;
let _20: *const u128;
let _21: i16;
let _22: isize;
let _23: [i16; 7];
let _24: i128;
let _25: &'static (*mut i64, (char, i16, char));
let _26: bool;
let _27: bool;
let _28: *mut usize;
let _29: isize;
let _30: char;
let _31: *mut usize;
let _32: &'static u32;
let _33: ();
let _34: ();
{
_4 = _1 * _5;
_11 = _3 - _1;
_5 = _12;
_2 = _9 ^ _1;
_11 = -_12;
Call(RET = fn7(_10, _7, _1, _7, _9, _3, _11, _4, _8, _3, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = &_14;
_13 = &(*_13);
_13 = &_14;
_6 = -_12;
_11 = !_9;
Goto(bb2)
}
bb2 = {
_14 = 337812593_i32 as f64;
_5 = _9;
_3 = -_4;
_8 = 5605449298875577338_i64 as isize;
_5 = _6;
_3 = _5;
_6 = 4301504936156191225_i64 as isize;
_17 = _7 <= _7;
_12 = _4 & _2;
_13 = &_14;
Goto(bb3)
}
bb3 = {
_12 = _5 << _7;
_8 = !_7;
_4 = _17 as isize;
_16 = 6800845338895495447_u64 as f32;
RET = [1832775194_i32,1201777122_i32,(-1415107885_i32)];
_8 = 35_u8 as isize;
Goto(bb4)
}
bb4 = {
_8 = -_4;
_1 = -_9;
_15 = [304349659253319472586518869845634380306_u128,272811670657578494390687896490445183223_u128,228093082332956699453847175208510536685_u128];
Goto(bb5)
}
bb5 = {
_16 = 390113118_i32 as f32;
_17 = !false;
_10 = _5;
Goto(bb6)
}
bb6 = {
_11 = _8;
Goto(bb7)
}
bb7 = {
_18.2 = (-166207551788822810215133173495654348635_i128) as i16;
_9 = _12 << _1;
RET = [(-683220230_i32),945730608_i32,(-1438968579_i32)];
_18.0 = &_1;
_15 = [264240831897664243355967868392838142736_u128,50564279623852389600472028165682206920_u128,310995310838956542905265052067159646410_u128];
_18.0 = &_1;
_3 = -_5;
_18.0 = &_4;
RET = [1400279714_i32,(-1260304396_i32),503374848_i32];
RET = [(-1956088331_i32),2127616500_i32,(-1258990844_i32)];
_15 = [72969834316885049765714128738925794970_u128,157265662367602390324875545095627580575_u128,80230415097372549887697186030053010196_u128];
_1 = _3 | _8;
_15 = [266429656377413388297633270121635567788_u128,172458914774442500132947073799628337283_u128,116720525549932670277009906017388502818_u128];
_3 = 335736501556460100996616690013037680638_u128 as isize;
_2 = !_9;
_18.2 = -19983_i16;
_3 = -_11;
_8 = 0_usize as isize;
_21 = -_18.2;
_18.2 = _21;
Goto(bb8)
}
bb8 = {
_1 = _5 >> _11;
_18.2 = _21 << _9;
RET = [1907406543_i32,1110838426_i32,722870663_i32];
_11 = -_5;
_3 = _7 | _4;
_1 = _9 & _5;
_2 = _18.2 as isize;
_13 = &_14;
_13 = &(*_13);
_5 = _4;
_22 = _5;
_18.0 = &_5;
_14 = 318114086240949790434336533786141296612_u128 as f64;
_18.2 = _21;
_13 = &_14;
_13 = &(*_13);
_13 = &(*_13);
_16 = (-11_i8) as f32;
_7 = _10 | _10;
_4 = _3 & _10;
RET = [2054572596_i32,(-1579939080_i32),(-545597744_i32)];
_18.0 = &_4;
Goto(bb9)
}
bb9 = {
_5 = !_10;
_18.2 = 286131178_i32 as i16;
_22 = _5;
_11 = _3 - _10;
_24 = (-62506398960335057214627990301406019135_i128);
_6 = _4;
_26 = _17;
_23 = [_21,_18.2,_18.2,_21,_21,_18.2,_21];
_13 = &_14;
RET = [(-916552685_i32),(-1963666306_i32),849432127_i32];
_11 = 12175269787176387268_usize as isize;
_10 = _6;
RET = [772114078_i32,(-465897045_i32),162511668_i32];
_22 = _3;
_16 = 9052347404975292365_usize as f32;
_11 = '\u{924c}' as isize;
_15 = [334966632720003012544470280352578889343_u128,21099168550308128385250974765816610268_u128,217335789685575361953301367633247122332_u128];
_13 = &(*_13);
_24 = 120718410750840140726844001548430427835_i128;
_24 = (-71620827254065642518333504020613248988_i128);
_13 = &(*_13);
_8 = !_5;
_3 = _12;
RET = [(-1741164883_i32),135558996_i32,(-163592912_i32)];
_10 = _2 << _2;
_11 = _17 as isize;
match _24 {
0 => bb5,
1 => bb7,
2 => bb10,
268661539666872820945041103411154962468 => bb12,
_ => bb11
}
}
bb10 = {
_1 = _5 >> _11;
_18.2 = _21 << _9;
RET = [1907406543_i32,1110838426_i32,722870663_i32];
_11 = -_5;
_3 = _7 | _4;
_1 = _9 & _5;
_2 = _18.2 as isize;
_13 = &_14;
_13 = &(*_13);
_5 = _4;
_22 = _5;
_18.0 = &_5;
_14 = 318114086240949790434336533786141296612_u128 as f64;
_18.2 = _21;
_13 = &_14;
_13 = &(*_13);
_13 = &(*_13);
_16 = (-11_i8) as f32;
_7 = _10 | _10;
_4 = _3 & _10;
RET = [2054572596_i32,(-1579939080_i32),(-545597744_i32)];
_18.0 = &_4;
Goto(bb9)
}
bb11 = {
_8 = -_4;
_1 = -_9;
_15 = [304349659253319472586518869845634380306_u128,272811670657578494390687896490445183223_u128,228093082332956699453847175208510536685_u128];
Goto(bb5)
}
bb12 = {
_14 = 271683058485702701039419168890657749336_u128 as f64;
_24 = (-131473311928448890366966140778202037629_i128) >> _7;
_24 = 141320207955644928228617267387236650975_i128;
RET = [1920883067_i32,748373070_i32,1578783907_i32];
_27 = _26;
_23 = [_21,_21,_18.2,_18.2,_21,_18.2,_18.2];
_18.2 = 97_i8 as i16;
_10 = _2;
_26 = _27;
_4 = _14 as isize;
match _24 {
0 => bb13,
1 => bb14,
141320207955644928228617267387236650975 => bb16,
_ => bb15
}
}
bb13 = {
_8 = -_4;
_1 = -_9;
_15 = [304349659253319472586518869845634380306_u128,272811670657578494390687896490445183223_u128,228093082332956699453847175208510536685_u128];
Goto(bb5)
}
bb14 = {
_16 = 390113118_i32 as f32;
_17 = !false;
_10 = _5;
Goto(bb6)
}
bb15 = {
_12 = _5 << _7;
_8 = !_7;
_4 = _17 as isize;
_16 = 6800845338895495447_u64 as f32;
RET = [1832775194_i32,1201777122_i32,(-1415107885_i32)];
_8 = 35_u8 as isize;
Goto(bb4)
}
bb16 = {
_5 = _12;
_8 = 180854953022047873151849076019624691546_u128 as isize;
_12 = !_2;
_12 = !_5;
_1 = !_3;
_4 = _3 ^ _5;
_9 = _5 & _3;
_27 = _17;
_1 = _4 | _2;
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(6_usize, 7_usize, Move(_7), 10_usize, Move(_10), 12_usize, Move(_12), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(6_usize, 23_usize, Move(_23), 1_usize, Move(_1), 6_usize, Move(_6), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(6_usize, 21_usize, Move(_21), 2_usize, Move(_2), 34_usize, _34, 34_usize, _34), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _13: bool;
let _14: ([u64; 4], (u16,));
let _15: char;
let _16: char;
let _17: u8;
let _18: *mut u32;
let _19: i128;
let _20: i128;
let _21: [u32; 8];
let _22: Adt50;
let _23: ([u64; 4], (u16,));
let _24: &'static i64;
let _25: u128;
let _26: Adt57;
let _27: u128;
let _28: ();
let _29: ();
{
_13 = false & true;
_3 = _10 >> _4;
_5 = _2 - _4;
_12 = _3 ^ _9;
RET = [879334570_i32,23332147_i32,(-1455595886_i32)];
RET = [522031907_i32,(-949862997_i32),309712546_i32];
_5 = -_8;
Call(RET = fn8(_12, _10, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14.1 = (46932_u16,);
_13 = !false;
_11 = -_8;
_14.0 = [3313938961308356669_u64,10364067546496453241_u64,15115434609992769266_u64,9659057296141587600_u64];
match _14.1.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
46932 => bb10,
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
_10 = !_5;
_15 = '\u{dddfe}';
RET = [572677495_i32,1785433952_i32,(-931939383_i32)];
_2 = -_4;
_14.0 = [11162399310047498327_u64,15872442061841736138_u64,1965652680756501557_u64,7113069775387663930_u64];
Goto(bb11)
}
bb11 = {
_6 = !_4;
_12 = _4 - _9;
RET = [(-2018271324_i32),(-548848330_i32),1101215058_i32];
_9 = _10 | _6;
_14.1 = (463_u16,);
_15 = '\u{c6efd}';
_15 = '\u{fedc2}';
_10 = 2111720936_u32 as isize;
_20 = 0_usize as i128;
_10 = 220304822270140129074133128174384349128_u128 as isize;
Goto(bb12)
}
bb12 = {
_15 = '\u{37b11}';
_16 = _15;
_20 = !159471508986470382653421537071788768528_i128;
_14.0 = [1348632608649655356_u64,17021503407926813804_u64,12892943894715484918_u64,10267626432523363638_u64];
_9 = _4 ^ _3;
RET = [(-1807630996_i32),(-1099525471_i32),1440913316_i32];
_21 = [1179848921_u32,1422134631_u32,3530597494_u32,3628156069_u32,968239009_u32,1136286627_u32,2000819362_u32,1645837196_u32];
_7 = _9;
_5 = !_2;
_8 = 9251800105510932859_u64 as isize;
_19 = _20 + _20;
_11 = _13 as isize;
RET = [(-1922596579_i32),(-692660219_i32),(-495095133_i32)];
_8 = _7;
_5 = 13508120456850849980_u64 as isize;
_5 = 77_u8 as isize;
_23.0 = _14.0;
_10 = !_12;
_2 = _9;
match _14.1.0 {
463 => bb14,
_ => bb13
}
}
bb13 = {
_14.1 = (46932_u16,);
_13 = !false;
_11 = -_8;
_14.0 = [3313938961308356669_u64,10364067546496453241_u64,15115434609992769266_u64,9659057296141587600_u64];
match _14.1.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
46932 => bb10,
_ => bb9
}
}
bb14 = {
_20 = _19;
_3 = _10 >> _2;
_11 = _1 * _6;
_23.0 = _14.0;
_13 = false;
_2 = -_11;
_23.1 = (_14.1.0,);
_2 = 6_usize as isize;
_21 = [2993590799_u32,938957374_u32,3924346217_u32,236481085_u32,3631512073_u32,3854809496_u32,158642323_u32,276295875_u32];
_15 = _16;
_12 = !_10;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(7_usize, 6_usize, Move(_6), 15_usize, Move(_15), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(7_usize, 12_usize, Move(_12), 1_usize, Move(_1), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(7_usize, 4_usize, Move(_4), 20_usize, Move(_20), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _4: &'static u16;
let _5: (char, i16, char);
let _6: *mut (isize, &'static isize, *mut f32, i8);
let _7: char;
let _8: i16;
let _9: f32;
let _10: (*mut usize, i64);
let _11: ([u64; 4], (u16,));
let _12: i16;
let _13: [u8; 5];
let _14: i16;
let _15: [u8; 7];
let _16: &'static u32;
let _17: [u32; 8];
let _18: f32;
let _19: [u32; 8];
let _20: &'static u32;
let _21: *mut usize;
let _22: Adt25;
let _23: u64;
let _24: i64;
let _25: ([u32; 8], (isize, &'static isize, *mut f32, i8), f32, (*const i16,));
let _26: i128;
let _27: i64;
let _28: [u32; 1];
let _29: [char; 2];
let _30: ();
let _31: ();
{
_2 = -_1;
Call(RET = fn9(_3, _3, _2, _3, _3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [231754484_i32,(-1999360246_i32),928001848_i32];
RET = [(-1042737808_i32),(-788682905_i32),(-491491800_i32)];
RET = [(-516635047_i32),(-1573728727_i32),(-1469847493_i32)];
RET = [282916235_i32,38590031_i32,1747871081_i32];
RET = [(-1433056710_i32),2101584701_i32,1106445265_i32];
_1 = _2 + _3;
RET = [(-1563253621_i32),1514516272_i32,(-411535656_i32)];
_3 = _1 ^ _2;
_1 = -_2;
RET = [1204239939_i32,759094205_i32,1001184607_i32];
_1 = !_3;
RET = [1620238369_i32,502591273_i32,(-267922013_i32)];
_1 = _3 & _3;
RET = [1188062316_i32,265483911_i32,321324451_i32];
_5.2 = '\u{a5fa6}';
_5.1 = (-30378_i16) - (-2828_i16);
_3 = (-46_i8) as isize;
RET = [2065180806_i32,(-774406867_i32),422260467_i32];
Goto(bb2)
}
bb2 = {
_7 = _5.2;
Goto(bb3)
}
bb3 = {
_5 = (_7, (-9566_i16), _7);
RET = [(-596767204_i32),2050613581_i32,(-425079694_i32)];
_2 = -_1;
RET = [(-1696330108_i32),1563039769_i32,1049236897_i32];
_7 = _5.0;
_7 = _5.2;
_5 = (_7, (-27113_i16), _7);
match _5.1 {
340282366920938463463374607431768184343 => bb4,
_ => bb1
}
}
bb4 = {
_5.0 = _7;
_5 = (_7, (-14655_i16), _7);
_5.1 = 4305_i16;
_3 = _1;
_5.2 = _7;
_2 = _3;
RET = [1911240114_i32,(-626142634_i32),535463283_i32];
_2 = _3;
_5.1 = 3_usize as i16;
_5.2 = _7;
RET = [292763441_i32,32213379_i32,505977486_i32];
_7 = _5.0;
RET = [853677475_i32,(-1368065093_i32),(-349009944_i32)];
_5.2 = _5.0;
_3 = _2 * _2;
RET = [1261826056_i32,1547346684_i32,2137650178_i32];
_5.0 = _7;
RET = [(-570210775_i32),96109339_i32,465711082_i32];
RET = [824097340_i32,1916920760_i32,205062896_i32];
RET = [2077532245_i32,1852049046_i32,(-1078862855_i32)];
Goto(bb5)
}
bb5 = {
_5 = (_7, 10896_i16, _7);
RET = [1208935446_i32,(-1463993206_i32),2100388652_i32];
_8 = _5.1;
_11.1.0 = _5.0 as u16;
_5.0 = _5.2;
_4 = &_11.1.0;
_5.0 = _7;
_13 = [5_u8,63_u8,114_u8,232_u8,79_u8];
_11.1 = (62995_u16,);
_5.0 = _5.2;
_10.1 = 4101465921255702211_i64;
_9 = 504886878_i32 as f32;
_4 = &_11.1.0;
_1 = _3 * _2;
Goto(bb6)
}
bb6 = {
_10.1 = !(-3770808181842851334_i64);
_9 = 7390061789621652618_usize as f32;
_12 = _5.1 << _1;
_11.0 = [18436060233302493717_u64,13709672079377331484_u64,13451047263363788352_u64,1267739576853479839_u64];
_3 = _2;
_11.1 = (14208_u16,);
_5.1 = _12 ^ _12;
RET = [1641346570_i32,(-11859247_i32),1477706852_i32];
_7 = _5.0;
_15 = [183_u8,62_u8,222_u8,94_u8,13_u8,137_u8,136_u8];
_8 = 6691782675481694355_u64 as i16;
RET = [1824254958_i32,1854216191_i32,(-777944427_i32)];
_1 = 9630886197949796780_u64 as isize;
_5.1 = false as i16;
_12 = 75199997050605140948069917832223840431_u128 as i16;
match _11.1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
14208 => bb8,
_ => bb7
}
}
bb7 = {
RET = [231754484_i32,(-1999360246_i32),928001848_i32];
RET = [(-1042737808_i32),(-788682905_i32),(-491491800_i32)];
RET = [(-516635047_i32),(-1573728727_i32),(-1469847493_i32)];
RET = [282916235_i32,38590031_i32,1747871081_i32];
RET = [(-1433056710_i32),2101584701_i32,1106445265_i32];
_1 = _2 + _3;
RET = [(-1563253621_i32),1514516272_i32,(-411535656_i32)];
_3 = _1 ^ _2;
_1 = -_2;
RET = [1204239939_i32,759094205_i32,1001184607_i32];
_1 = !_3;
RET = [1620238369_i32,502591273_i32,(-267922013_i32)];
_1 = _3 & _3;
RET = [1188062316_i32,265483911_i32,321324451_i32];
_5.2 = '\u{a5fa6}';
_5.1 = (-30378_i16) - (-2828_i16);
_3 = (-46_i8) as isize;
RET = [2065180806_i32,(-774406867_i32),422260467_i32];
Goto(bb2)
}
bb8 = {
_15 = [101_u8,79_u8,222_u8,216_u8,49_u8,141_u8,50_u8];
_17 = [42518903_u32,1013486609_u32,3840766017_u32,2664464062_u32,2543761795_u32,2896479087_u32,4002792482_u32,2549566316_u32];
_17 = [2796050774_u32,2912571392_u32,3215773448_u32,920922934_u32,861239496_u32,1924925389_u32,1969484982_u32,3732927545_u32];
_5.1 = !_12;
_10.1 = !6880435624969576953_i64;
_2 = -_3;
RET = [(-1582686205_i32),(-14688056_i32),1772673278_i32];
RET = [(-720790477_i32),1944690214_i32,(-483964075_i32)];
_1 = _3 & _2;
_10.1 = 3575561440285741321_i64 * 9184198545792622969_i64;
RET = [(-931351862_i32),(-1066239313_i32),1309180239_i32];
RET = [387076411_i32,612752891_i32,427919496_i32];
_11.1.0 = 2_usize as u16;
_15 = [73_u8,194_u8,121_u8,156_u8,231_u8,43_u8,166_u8];
_9 = _3 as f32;
_5.0 = _5.2;
_5 = (_7, _8, _7);
RET = [104660955_i32,1549235031_i32,(-465197406_i32)];
Goto(bb9)
}
bb9 = {
_4 = &_11.1.0;
_2 = _1 >> _3;
_2 = -_1;
_8 = !_5.1;
_5.0 = _7;
Goto(bb10)
}
bb10 = {
_11.1 = (21091_u16,);
_18 = _9;
_4 = &_11.1.0;
_3 = !_2;
_5.1 = _12;
_19 = [518659660_u32,218855723_u32,2534203634_u32,2290036355_u32,1944298155_u32,2018859483_u32,3353987475_u32,3414683587_u32];
_4 = &(*_4);
_6 = core::ptr::addr_of_mut!(_25.1);
_23 = 5410602363855390372_u64 - 10125700447949043050_u64;
_2 = _1;
Goto(bb11)
}
bb11 = {
_25.1.3 = 48_i8;
_6 = core::ptr::addr_of_mut!(_25.1);
RET = [(-1106120054_i32),(-301847628_i32),(-1808858060_i32)];
(*_6).2 = core::ptr::addr_of_mut!(_9);
_5.1 = !_12;
(*_6).3 = (-124_i8) * (-73_i8);
_9 = _18;
_18 = _9 * _9;
RET = [(-505430605_i32),(-1874706900_i32),(-1212763997_i32)];
_8 = !_12;
_7 = _5.0;
_25.2 = _9 + _18;
match (*_4) {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
21091 => bb12,
_ => bb8
}
}
bb12 = {
_4 = &_11.1.0;
_22 = Adt25::Variant3 { fld0: _23 };
(*_6).1 = &(*_6).0;
(*_6).2 = core::ptr::addr_of_mut!(_18);
_25.0 = [4076423621_u32,2663499270_u32,218995008_u32,2944165736_u32,165974926_u32,1636285415_u32,3505497321_u32,1547231744_u32];
_25.1.3 = !55_i8;
_17 = [2725473994_u32,2862904427_u32,3444138619_u32,3305864235_u32,1844118549_u32,3609319823_u32,1755399576_u32,437724980_u32];
Goto(bb13)
}
bb13 = {
_5.1 = _12;
_25.3.0 = core::ptr::addr_of!(_8);
_25.2 = _9 * _9;
Goto(bb14)
}
bb14 = {
_15 = [187_u8,240_u8,233_u8,97_u8,184_u8,196_u8,21_u8];
_3 = !_2;
(*_6).0 = _2;
_26 = _5.2 as i128;
RET = [(-1569106694_i32),578397508_i32,(-1149580343_i32)];
_25.1.2 = core::ptr::addr_of_mut!(_18);
_7 = _5.2;
_5.2 = _7;
_22 = Adt25::Variant3 { fld0: _23 };
(*_6).1 = &(*_6).0;
_13 = [161_u8,92_u8,192_u8,55_u8,4_u8];
_10.1 = -8462307596221399611_i64;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(8_usize, 13_usize, Move(_13), 2_usize, Move(_2), 19_usize, Move(_19), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(8_usize, 12_usize, Move(_12), 17_usize, Move(_17), 5_usize, Move(_5), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _7: [u8; 5];
let _8: f32;
let _9: *const i16;
let _10: *mut (isize, &'static isize, *mut f32, i8);
let _11: (u32, u16, i128);
let _12: i64;
let _13: bool;
let _14: u128;
let _15: f32;
let _16: u8;
let _17: (*const i16,);
let _18: i128;
let _19: (isize, &'static isize, *mut f32, i8);
let _20: [u32; 1];
let _21: f64;
let _22: *const *mut f32;
let _23: f64;
let _24: &'static i32;
let _25: [u8; 5];
let _26: [u32; 1];
let _27: ((char, i16, char), &'static *const usize, (i32,), *mut u16);
let _28: (f32,);
let _29: ((char, i16, char), &'static *const usize, (i32,), *mut u16);
let _30: bool;
let _31: i16;
let _32: isize;
let _33: char;
let _34: [u16; 8];
let _35: (char, i16, char);
let _36: u32;
let _37: [u128; 3];
let _38: char;
let _39: [u32; 1];
let _40: char;
let _41: i16;
let _42: ();
let _43: ();
{
RET = [1767810333_i32,(-1607150094_i32),639210725_i32];
_3 = _4 * _4;
_6 = (-6112908581557373096_i64) as isize;
_3 = _2 ^ _5;
_7 = [225_u8,60_u8,146_u8,58_u8,120_u8];
_7 = [143_u8,149_u8,149_u8,26_u8,140_u8];
_8 = 4986629487617781870_u64 as f32;
_2 = false as isize;
_4 = _8 as isize;
_7 = [18_u8,205_u8,41_u8,106_u8,125_u8];
_6 = _1 & _1;
_8 = 11519522295448528588_u64 as f32;
_5 = 29144_i16 as isize;
RET = [(-1611481503_i32),981801617_i32,(-1320962088_i32)];
_8 = 25092_u16 as f32;
Call(RET = fn10(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = -_6;
Goto(bb2)
}
bb2 = {
_8 = 3671474351_u32 as f32;
_1 = _5;
_11.0 = 61_i8 as u32;
_4 = _6;
_8 = 871_u16 as f32;
_7 = [166_u8,236_u8,192_u8,215_u8,80_u8];
_8 = 1016_i16 as f32;
Goto(bb3)
}
bb3 = {
_11.1 = 53223_u16;
_14 = (-1482540016_i32) as u128;
_5 = _1 * _6;
_11.2 = !(-39085348256161231002178627662841286106_i128);
_16 = 151_u8;
_13 = !false;
_13 = !true;
_13 = true | false;
_19.0 = _6;
_12 = (-8322831303825779148_i64);
Call(_15 = core::intrinsics::transmute(_11.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = !_19.0;
_18 = _11.2 >> _6;
_20 = [_11.0];
_19.0 = (-501998378_i32) as isize;
_15 = -_8;
_8 = _15;
RET = [914060868_i32,300545296_i32,(-487202001_i32)];
_6 = _5;
_15 = _8 + _8;
_10 = core::ptr::addr_of_mut!(_19);
_14 = 185411725395338907738758254930038488031_u128;
(*_10).1 = &_19.0;
_21 = _8 as f64;
_3 = -_5;
(*_10).2 = core::ptr::addr_of_mut!(_15);
_11.2 = _18;
(*_10).0 = !_4;
_22 = core::ptr::addr_of!(_19.2);
_19.3 = -(-79_i8);
_19.3 = 4_usize as i8;
_16 = !151_u8;
_23 = _21 - _21;
_19.1 = &_19.0;
_11.1 = 42754_u16 ^ 45409_u16;
Call(_26 = core::intrinsics::transmute(_20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_22 = core::ptr::addr_of!((*_22));
_11.0 = _13 as u32;
_27.2.0 = !325614287_i32;
_14 = _11.1 as u128;
match _12 {
0 => bb1,
1 => bb2,
340282366920938463455051776127942432308 => bb6,
_ => bb4
}
}
bb6 = {
(*_10).3 = (-106_i8);
Goto(bb7)
}
bb7 = {
_17.0 = core::ptr::addr_of!(_27.0.1);
_27.0.1 = -1651_i16;
_27.0 = ('\u{9cc7}', 18771_i16, '\u{16034}');
Goto(bb8)
}
bb8 = {
_2 = !_19.0;
_27.3 = core::ptr::addr_of_mut!(_11.1);
_19.2 = core::ptr::addr_of_mut!(_28.0);
_27.0 = ('\u{575c6}', 7553_i16, '\u{94888}');
_12 = _13 as i64;
_28.0 = _15 + _8;
_29.2 = _27.2;
(*_10).0 = _11.0 as isize;
match _27.0.1 {
0 => bb1,
1 => bb4,
7553 => bb9,
_ => bb7
}
}
bb9 = {
_29.3 = Move(_27.3);
_3 = !_4;
_9 = Move(_17.0);
_14 = _29.2.0 as u128;
_31 = _27.0.1 ^ _27.0.1;
_30 = !_13;
_12 = (-6349585302219693386_i64) << _11.2;
_5 = _6 & _1;
(*_10).0 = _4;
_21 = -_23;
_29.0.1 = _31 >> (*_10).0;
_11 = (328073427_u32, 47194_u16, _18);
_9 = core::ptr::addr_of!(_29.0.1);
Call((*_10).2 = fn18(), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_10).2 = core::ptr::addr_of_mut!(_28.0);
_27.2 = _29.2;
_27.0.2 = _27.0.0;
_3 = _1 | _1;
_17 = (Move(_9),);
_23 = _21 - _21;
_28.0 = _15 + _15;
_6 = _14 as isize;
_29.3 = core::ptr::addr_of_mut!(_11.1);
_4 = _30 as isize;
_31 = _29.0.1;
_11 = (2917068647_u32, 8135_u16, _18);
(*_10).0 = _27.2.0 as isize;
match _11.0 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb11,
2917068647 => bb13,
_ => bb12
}
}
bb11 = {
_11.1 = 53223_u16;
_14 = (-1482540016_i32) as u128;
_5 = _1 * _6;
_11.2 = !(-39085348256161231002178627662841286106_i128);
_16 = 151_u8;
_13 = !false;
_13 = !true;
_13 = true | false;
_19.0 = _6;
_12 = (-8322831303825779148_i64);
Call(_15 = core::intrinsics::transmute(_11.0), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_3 = !_19.0;
_18 = _11.2 >> _6;
_20 = [_11.0];
_19.0 = (-501998378_i32) as isize;
_15 = -_8;
_8 = _15;
RET = [914060868_i32,300545296_i32,(-487202001_i32)];
_6 = _5;
_15 = _8 + _8;
_10 = core::ptr::addr_of_mut!(_19);
_14 = 185411725395338907738758254930038488031_u128;
(*_10).1 = &_19.0;
_21 = _8 as f64;
_3 = -_5;
(*_10).2 = core::ptr::addr_of_mut!(_15);
_11.2 = _18;
(*_10).0 = !_4;
_22 = core::ptr::addr_of!(_19.2);
_19.3 = -(-79_i8);
_19.3 = 4_usize as i8;
_16 = !151_u8;
_23 = _21 - _21;
_19.1 = &_19.0;
_11.1 = 42754_u16 ^ 45409_u16;
Call(_26 = core::intrinsics::transmute(_20), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_20 = [_11.0];
(*_10).1 = &_2;
_27.2.0 = _27.0.2 as i32;
(*_10).0 = _5 >> _11.1;
_29.0.1 = _31;
_26 = [_11.0];
(*_22) = core::ptr::addr_of_mut!(_8);
(*_10).2 = core::ptr::addr_of_mut!(_28.0);
_24 = &_29.2.0;
_27.0 = ('\u{c2d0b}', _31, '\u{8acda}');
_29.3 = core::ptr::addr_of_mut!(_11.1);
Call(_23 = core::intrinsics::transmute(_2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
(*_10).0 = _13 as isize;
(*_10).1 = &_1;
_29.0.2 = _27.0.2;
_6 = _23 as isize;
_29.3 = core::ptr::addr_of_mut!(_11.1);
RET = [(*_24),_27.2.0,_27.2.0];
_27.0.0 = _27.0.2;
_35 = _27.0;
_28.0 = _15 - _8;
_18 = _16 as i128;
_19.1 = &_1;
_6 = _1 - _2;
_28 = (_15,);
_41 = _15 as i16;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(9_usize, 20_usize, Move(_20), 4_usize, Move(_4), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(9_usize, 35_usize, Move(_35), 41_usize, Move(_41), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(9_usize, 1_usize, Move(_1), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10() -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _1: &'static i64;
let _2: [i32; 8];
let _3: [char; 2];
let _4: char;
let _5: bool;
let _6: Adt24;
let _7: (u32, u16, i128);
let _8: &'static *const usize;
let _9: [u128; 3];
let _10: f64;
let _11: &'static f64;
let _12: Adt33;
let _13: ([u64; 4], (u16,));
let _14: &'static f64;
let _15: isize;
let _16: &'static i32;
let _17: u8;
let _18: u8;
let _19: f32;
let _20: Adt57;
let _21: f64;
let _22: char;
let _23: i8;
let _24: *const i16;
let _25: Adt24;
let _26: char;
let _27: f64;
let _28: *mut i64;
let _29: (u32, *const *mut f32, Adt53, Adt33);
let _30: (char, i16, char);
let _31: i32;
let _32: [char; 2];
let _33: [u32; 8];
let _34: ();
let _35: ();
{
RET = [(-803678912_i32),(-294584159_i32),638955471_i32];
RET = [1572758036_i32,(-948317365_i32),1771328005_i32];
RET = [(-677768226_i32),(-154772027_i32),1316415495_i32];
RET = [854217710_i32,881464411_i32,(-265008384_i32)];
RET = [(-313326245_i32),398620122_i32,1133675109_i32];
RET = [592020335_i32,(-547007337_i32),363751998_i32];
Goto(bb1)
}
bb1 = {
RET = [(-1566468732_i32),(-1591472400_i32),1647376695_i32];
RET = [1752978109_i32,633169235_i32,(-617069902_i32)];
RET = [(-131763414_i32),1591287395_i32,(-1921189044_i32)];
_2 = [(-649257626_i32),(-610738470_i32),1236013763_i32,(-1393339733_i32),(-963415593_i32),(-1783642186_i32),(-1185395747_i32),432888209_i32];
RET = [(-808019554_i32),1397509171_i32,(-453954334_i32)];
_3 = ['\u{9f1a9}','\u{104560}'];
_2 = [509399344_i32,1151373418_i32,(-1054939661_i32),(-1205552792_i32),(-941670935_i32),(-775165407_i32),876272195_i32,28358149_i32];
_3 = ['\u{5a8c2}','\u{1b57f}'];
_3 = ['\u{68b35}','\u{a5e5e}'];
_3 = ['\u{7405c}','\u{c8abe}'];
_4 = '\u{3a047}';
RET = [735642569_i32,992323108_i32,(-141440384_i32)];
RET = [(-2099032431_i32),(-1661184129_i32),703827976_i32];
_4 = '\u{10ae92}';
RET = [(-167931959_i32),(-318848990_i32),1777895672_i32];
_4 = '\u{6cc8a}';
_7.0 = !1253658358_u32;
_7.1 = 49_i8 as u16;
RET = [(-1317475584_i32),(-332040120_i32),1772927812_i32];
_5 = !true;
_7.0 = 823103430_u32 * 2591696769_u32;
_7.1 = 29_isize as u16;
_2 = [(-1724692131_i32),2068832890_i32,764960748_i32,(-1746163891_i32),(-174956154_i32),(-2072720505_i32),(-1491231033_i32),(-1800412245_i32)];
RET = [(-1765174822_i32),(-1947884935_i32),1879945072_i32];
Call(RET = fn11(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = (1416712223_u32, 6380_u16, (-12035789913224081043244015892429952160_i128));
_5 = !false;
_7.0 = !2498660265_u32;
_9 = [84402866358540903689051200979303762063_u128,123713452911801310132356587583237936664_u128,275882109155377078494402871585530961644_u128];
_5 = _7.2 > _7.2;
_7.1 = !51519_u16;
RET = [56109235_i32,855141724_i32,856519675_i32];
_7 = (1208251276_u32, 8042_u16, (-152516528767664029064754194471107357483_i128));
_13.0 = [6259639127846072743_u64,15083242474413047430_u64,14587238802348165436_u64,15311645389785180237_u64];
_12 = Adt33::Variant3 { fld0: _9,fld1: _4 };
_13.1 = (_7.1,);
_14 = &_10;
_9 = [71870100341400981165409165453526104895_u128,251560754851756616632982762019885675330_u128,230077802225183396609484629619487031038_u128];
RET = [(-1606839809_i32),(-323300655_i32),1754325701_i32];
_13.1 = (_7.1,);
_7 = (2357625670_u32, _13.1.0, (-106096174577576761258839853609611630668_i128));
_13.1.0 = _7.1;
_2 = [91581653_i32,846924549_i32,823566807_i32,(-1024415673_i32),(-24519032_i32),(-301435028_i32),(-1855964133_i32),1585588624_i32];
_7.2 = 491967733633885271_i64 as i128;
Goto(bb3)
}
bb3 = {
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = [317128091416670792968686246987759683930_u128,151431890797423949175670294366782785685_u128,169725922073387094240369332439503300357_u128];
RET = [(-823724300_i32),(-688753666_i32),(-1843310104_i32)];
_14 = &(*_14);
_9 = [83969679028149020118829092441372149228_u128,75384389180824134976391413725726525506_u128,187174824104154810186081698988872172960_u128];
_10 = (-89289157_i32) as f64;
_3 = [_4,_4];
_11 = &_10;
_9 = [210365523036919159468302211387876676292_u128,92576799395933854876603349593104535963_u128,34015489678726420987476785495790634232_u128];
_4 = Field::<char>(Variant(_12, 3), 1);
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = [140215008681826769284616933795895309202_u128,114282241712258702272686040378633996397_u128,312436706990120851013275870983768415567_u128];
_7.0 = 2638321980_u32 + 3851939605_u32;
_7.0 = 4233261240_u32 + 4280904666_u32;
_7.1 = _13.1.0 - _13.1.0;
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = [198434635550075353868054840771561566332_u128,260030524435744002267865121611455420780_u128,227615038644400444345964899556210759556_u128];
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = _9;
_11 = &(*_11);
_13.1.0 = 3143394149250066898_usize as u16;
_4 = Field::<char>(Variant(_12, 3), 1);
_11 = &(*_11);
place!(Field::<char>(Variant(_12, 3), 1)) = _4;
_3 = [Field::<char>(Variant(_12, 3), 1),_4];
_4 = Field::<char>(Variant(_12, 3), 1);
SetDiscriminant(_12, 2);
RET = [(-296237015_i32),830343147_i32,(-1875605689_i32)];
_1 = &place!(Field::<i64>(Variant(_12, 2), 4));
RET = [(-1957085581_i32),(-1081958155_i32),1529440718_i32];
_9 = [246900355131918949306256674489413898364_u128,299628323834798613652237907552503378785_u128,23193128549920437036718151974831697289_u128];
place!(Field::<i64>(Variant(_12, 2), 4)) = (-9146238820459505569_i64) | (-6223388014979977883_i64);
_15 = (-11_isize) - (-9223372036854775808_isize);
Goto(bb4)
}
bb4 = {
_1 = &place!(Field::<i64>(Variant(_12, 2), 4));
_11 = &(*_11);
_3 = [_4,_4];
_13.1.0 = _7.1 * _7.1;
_3 = [_4,_4];
place!(Field::<i8>(Variant(_12, 2), 3)) = _7.2 as i8;
_11 = &(*_11);
_14 = &(*_11);
_10 = 3388682750813286649_usize as f64;
_14 = &_10;
place!(Field::<i64>(Variant(_12, 2), 4)) = 8257685862473818356_i64 + 4250126361850087966_i64;
place!(Field::<u8>(Variant(_12, 2), 2)) = 8_u8;
_10 = 1147370978_i32 as f64;
_17 = !Field::<u8>(Variant(_12, 2), 2);
_18 = _17 + _17;
_1 = &place!(Field::<i64>(Variant(_12, 2), 4));
place!(Field::<u8>(Variant(_12, 2), 2)) = !_18;
Goto(bb5)
}
bb5 = {
_21 = -_10;
place!(Field::<u8>(Variant(_12, 2), 2)) = _18 ^ _17;
_9 = [335995758097855401511067456858596975851_u128,54026653788852857865364511107020298766_u128,310386944316244401930570789074311625687_u128];
place!(Field::<i8>(Variant(_12, 2), 3)) = _21 as i8;
_22 = _4;
RET = [1438645983_i32,(-66572134_i32),162217822_i32];
_22 = _4;
_14 = &_21;
_11 = Move(_14);
_13.1.0 = _5 as u16;
_7 = (2862765479_u32, _13.1.0, (-153549316752231597836155908769277564165_i128));
_13.1 = (_7.1,);
_19 = 12769548094508314957_u64 as f32;
match _7.0 {
0 => bb2,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
2862765479 => bb11,
_ => bb10
}
}
bb6 = {
_1 = &place!(Field::<i64>(Variant(_12, 2), 4));
_11 = &(*_11);
_3 = [_4,_4];
_13.1.0 = _7.1 * _7.1;
_3 = [_4,_4];
place!(Field::<i8>(Variant(_12, 2), 3)) = _7.2 as i8;
_11 = &(*_11);
_14 = &(*_11);
_10 = 3388682750813286649_usize as f64;
_14 = &_10;
place!(Field::<i64>(Variant(_12, 2), 4)) = 8257685862473818356_i64 + 4250126361850087966_i64;
place!(Field::<u8>(Variant(_12, 2), 2)) = 8_u8;
_10 = 1147370978_i32 as f64;
_17 = !Field::<u8>(Variant(_12, 2), 2);
_18 = _17 + _17;
_1 = &place!(Field::<i64>(Variant(_12, 2), 4));
place!(Field::<u8>(Variant(_12, 2), 2)) = !_18;
Goto(bb5)
}
bb7 = {
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = [317128091416670792968686246987759683930_u128,151431890797423949175670294366782785685_u128,169725922073387094240369332439503300357_u128];
RET = [(-823724300_i32),(-688753666_i32),(-1843310104_i32)];
_14 = &(*_14);
_9 = [83969679028149020118829092441372149228_u128,75384389180824134976391413725726525506_u128,187174824104154810186081698988872172960_u128];
_10 = (-89289157_i32) as f64;
_3 = [_4,_4];
_11 = &_10;
_9 = [210365523036919159468302211387876676292_u128,92576799395933854876603349593104535963_u128,34015489678726420987476785495790634232_u128];
_4 = Field::<char>(Variant(_12, 3), 1);
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = [140215008681826769284616933795895309202_u128,114282241712258702272686040378633996397_u128,312436706990120851013275870983768415567_u128];
_7.0 = 2638321980_u32 + 3851939605_u32;
_7.0 = 4233261240_u32 + 4280904666_u32;
_7.1 = _13.1.0 - _13.1.0;
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = [198434635550075353868054840771561566332_u128,260030524435744002267865121611455420780_u128,227615038644400444345964899556210759556_u128];
place!(Field::<[u128; 3]>(Variant(_12, 3), 0)) = _9;
_11 = &(*_11);
_13.1.0 = 3143394149250066898_usize as u16;
_4 = Field::<char>(Variant(_12, 3), 1);
_11 = &(*_11);
place!(Field::<char>(Variant(_12, 3), 1)) = _4;
_3 = [Field::<char>(Variant(_12, 3), 1),_4];
_4 = Field::<char>(Variant(_12, 3), 1);
SetDiscriminant(_12, 2);
RET = [(-296237015_i32),830343147_i32,(-1875605689_i32)];
_1 = &place!(Field::<i64>(Variant(_12, 2), 4));
RET = [(-1957085581_i32),(-1081958155_i32),1529440718_i32];
_9 = [246900355131918949306256674489413898364_u128,299628323834798613652237907552503378785_u128,23193128549920437036718151974831697289_u128];
place!(Field::<i64>(Variant(_12, 2), 4)) = (-9146238820459505569_i64) | (-6223388014979977883_i64);
_15 = (-11_isize) - (-9223372036854775808_isize);
Goto(bb4)
}
bb8 = {
_7 = (1416712223_u32, 6380_u16, (-12035789913224081043244015892429952160_i128));
_5 = !false;
_7.0 = !2498660265_u32;
_9 = [84402866358540903689051200979303762063_u128,123713452911801310132356587583237936664_u128,275882109155377078494402871585530961644_u128];
_5 = _7.2 > _7.2;
_7.1 = !51519_u16;
RET = [56109235_i32,855141724_i32,856519675_i32];
_7 = (1208251276_u32, 8042_u16, (-152516528767664029064754194471107357483_i128));
_13.0 = [6259639127846072743_u64,15083242474413047430_u64,14587238802348165436_u64,15311645389785180237_u64];
_12 = Adt33::Variant3 { fld0: _9,fld1: _4 };
_13.1 = (_7.1,);
_14 = &_10;
_9 = [71870100341400981165409165453526104895_u128,251560754851756616632982762019885675330_u128,230077802225183396609484629619487031038_u128];
RET = [(-1606839809_i32),(-323300655_i32),1754325701_i32];
_13.1 = (_7.1,);
_7 = (2357625670_u32, _13.1.0, (-106096174577576761258839853609611630668_i128));
_13.1.0 = _7.1;
_2 = [91581653_i32,846924549_i32,823566807_i32,(-1024415673_i32),(-24519032_i32),(-301435028_i32),(-1855964133_i32),1585588624_i32];
_7.2 = 491967733633885271_i64 as i128;
Goto(bb3)
}
bb9 = {
RET = [(-1566468732_i32),(-1591472400_i32),1647376695_i32];
RET = [1752978109_i32,633169235_i32,(-617069902_i32)];
RET = [(-131763414_i32),1591287395_i32,(-1921189044_i32)];
_2 = [(-649257626_i32),(-610738470_i32),1236013763_i32,(-1393339733_i32),(-963415593_i32),(-1783642186_i32),(-1185395747_i32),432888209_i32];
RET = [(-808019554_i32),1397509171_i32,(-453954334_i32)];
_3 = ['\u{9f1a9}','\u{104560}'];
_2 = [509399344_i32,1151373418_i32,(-1054939661_i32),(-1205552792_i32),(-941670935_i32),(-775165407_i32),876272195_i32,28358149_i32];
_3 = ['\u{5a8c2}','\u{1b57f}'];
_3 = ['\u{68b35}','\u{a5e5e}'];
_3 = ['\u{7405c}','\u{c8abe}'];
_4 = '\u{3a047}';
RET = [735642569_i32,992323108_i32,(-141440384_i32)];
RET = [(-2099032431_i32),(-1661184129_i32),703827976_i32];
_4 = '\u{10ae92}';
RET = [(-167931959_i32),(-318848990_i32),1777895672_i32];
_4 = '\u{6cc8a}';
_7.0 = !1253658358_u32;
_7.1 = 49_i8 as u16;
RET = [(-1317475584_i32),(-332040120_i32),1772927812_i32];
_5 = !true;
_7.0 = 823103430_u32 * 2591696769_u32;
_7.1 = 29_isize as u16;
_2 = [(-1724692131_i32),2068832890_i32,764960748_i32,(-1746163891_i32),(-174956154_i32),(-2072720505_i32),(-1491231033_i32),(-1800412245_i32)];
RET = [(-1765174822_i32),(-1947884935_i32),1879945072_i32];
Call(RET = fn11(), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
place!(Field::<u64>(Variant(_12, 2), 1)) = _13.1.0 as u64;
_13.0 = [Field::<u64>(Variant(_12, 2), 1),Field::<u64>(Variant(_12, 2), 1),Field::<u64>(Variant(_12, 2), 1),Field::<u64>(Variant(_12, 2), 1)];
_5 = !false;
_12 = Adt33::Variant3 { fld0: _9,fld1: _4 };
_23 = !116_i8;
_27 = _10;
Goto(bb12)
}
bb12 = {
RET = [1796647375_i32,(-773851330_i32),1185535608_i32];
_2 = [652219842_i32,641220304_i32,(-1387548714_i32),819308341_i32,(-121916446_i32),1082816516_i32,(-1654561137_i32),(-206285007_i32)];
place!(Field::<char>(Variant(_12, 3), 1)) = _22;
_13.1.0 = _7.1 - _7.1;
SetDiscriminant(_12, 1);
_23 = (-5_i8) << _7.0;
_13.1.0 = _7.1 + _7.1;
place!(Field::<bool>(Variant(_12, 1), 0)) = !_5;
place!(Field::<*mut u32>(Variant(_12, 1), 2)) = core::ptr::addr_of_mut!(_29.0);
_27 = _21;
_11 = &_27;
place!(Field::<u128>(Variant(_12, 1), 4)) = (-10192_i16) as u128;
_9 = [Field::<u128>(Variant(_12, 1), 4),Field::<u128>(Variant(_12, 1), 4),Field::<u128>(Variant(_12, 1), 4)];
Call(_7.0 = fn14(Move(_11), _7.2, _23), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<*mut u32>(Variant(_12, 1), 2)) = core::ptr::addr_of_mut!(_29.0);
Goto(bb14)
}
bb14 = {
place!(Field::<*mut u32>(Variant(_12, 1), 2)) = core::ptr::addr_of_mut!(_7.0);
_13.1.0 = !_7.1;
_5 = !Field::<bool>(Variant(_12, 1), 0);
_13.1 = (_7.1,);
_33 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_30.2 = _4;
place!(Field::<i128>(Variant(_12, 1), 1)) = _23 as i128;
_26 = _30.2;
_17 = _19 as u8;
_10 = -_21;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(10_usize, 26_usize, Move(_26), 3_usize, Move(_3), 15_usize, Move(_15), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(10_usize, 2_usize, Move(_2), 5_usize, Move(_5), 7_usize, Move(_7), 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _1: [u16; 8];
let _2: ((char, i16, char), &'static *const usize, (i32,), *mut u16);
let _3: bool;
let _4: f64;
let _5: Adt75;
let _6: f64;
let _7: *mut usize;
let _8: (*mut usize, i64);
let _9: isize;
let _10: i32;
let _11: char;
let _12: f32;
let _13: f64;
let _14: u64;
let _15: Adt25;
let _16: char;
let _17: i32;
let _18: &'static isize;
let _19: u64;
let _20: ();
let _21: ();
{
RET = [662383522_i32,(-4650203_i32),(-459383607_i32)];
RET = [(-1891847921_i32),(-1009152139_i32),308970491_i32];
RET = [(-871480251_i32),(-1065667336_i32),(-1964776858_i32)];
RET = [1733333628_i32,(-1791562390_i32),(-436129521_i32)];
RET = [(-945333985_i32),(-338371150_i32),1100594503_i32];
RET = [635567640_i32,(-1162595170_i32),1429694706_i32];
RET = [(-1809267192_i32),(-66452571_i32),273829399_i32];
RET = [455037715_i32,520724487_i32,154591358_i32];
RET = [(-878307691_i32),1448649910_i32,1828386651_i32];
RET = [(-2092836245_i32),1250534164_i32,(-1477670292_i32)];
RET = [(-1442012738_i32),1148382044_i32,(-816436330_i32)];
_1 = [16808_u16,44135_u16,54155_u16,58796_u16,54491_u16,47932_u16,49664_u16,39726_u16];
_1 = [15403_u16,55337_u16,63837_u16,454_u16,54289_u16,28186_u16,31166_u16,2313_u16];
RET = [205350126_i32,(-1860001026_i32),1711007865_i32];
RET = [(-835965141_i32),(-1604657776_i32),2059029404_i32];
RET = [(-868059687_i32),(-640288157_i32),(-1706335324_i32)];
RET = [533664641_i32,(-1968307584_i32),(-311357736_i32)];
RET = [86175714_i32,(-103916951_i32),(-180779934_i32)];
_1 = [1014_u16,55165_u16,41227_u16,56171_u16,57256_u16,49370_u16,29849_u16,11262_u16];
_1 = [7992_u16,29258_u16,10172_u16,45863_u16,8529_u16,17143_u16,57464_u16,16995_u16];
_2.0.1 = (-24622_i16) * 18422_i16;
_2.2 = (1135131927_i32,);
Call(_2.0.2 = fn12(RET, RET, _2.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.2 = ((-672078478_i32),);
RET = [_2.2.0,_2.2.0,_2.2.0];
_3 = true;
_3 = !true;
_2.2 = ((-1814491641_i32),);
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.0.0 = _2.0.2;
_2.2 = (1441701527_i32,);
_4 = 11960962277196086088_u64 as f64;
_2.0.0 = _2.0.2;
_3 = _2.0.1 > _2.0.1;
RET = [_2.2.0,_2.2.0,_2.2.0];
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.2 = (1600735111_i32,);
_2.0.0 = _2.0.2;
_2.0 = ('\u{10b272}', 23569_i16, '\u{6363c}');
_6 = -_4;
_2.0.1 = -(-28257_i16);
_4 = 56858903_u32 as f64;
_2.0.1 = 18013_u16 as i16;
_1 = [64399_u16,18772_u16,38820_u16,4547_u16,21673_u16,25675_u16,23343_u16,61865_u16];
match _2.2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1600735111 => bb8,
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
_2.0.1 = !5371_i16;
match _2.2.0 {
0 => bb9,
1600735111 => bb11,
_ => bb10
}
}
bb9 = {
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.2 = ((-672078478_i32),);
RET = [_2.2.0,_2.2.0,_2.2.0];
_3 = true;
_3 = !true;
_2.2 = ((-1814491641_i32),);
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.0.0 = _2.0.2;
_2.2 = (1441701527_i32,);
_4 = 11960962277196086088_u64 as f64;
_2.0.0 = _2.0.2;
_3 = _2.0.1 > _2.0.1;
RET = [_2.2.0,_2.2.0,_2.2.0];
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.2 = (1600735111_i32,);
_2.0.0 = _2.0.2;
_2.0 = ('\u{10b272}', 23569_i16, '\u{6363c}');
_6 = -_4;
_2.0.1 = -(-28257_i16);
_4 = 56858903_u32 as f64;
_2.0.1 = 18013_u16 as i16;
_1 = [64399_u16,18772_u16,38820_u16,4547_u16,21673_u16,25675_u16,23343_u16,61865_u16];
match _2.2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
1600735111 => bb8,
_ => bb7
}
}
bb10 = {
Return()
}
bb11 = {
_2.2.0 = 1948711656_i32;
_3 = true;
_2.2 = ((-1641130676_i32),);
_2.0 = ('\u{5b9f}', 28657_i16, '\u{3209d}');
_1 = [8497_u16,58834_u16,60366_u16,46457_u16,62602_u16,62224_u16,27639_u16,44664_u16];
_3 = !true;
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.0.1 = 5732_i16 >> _2.2.0;
_3 = !false;
RET = [_2.2.0,_2.2.0,_2.2.0];
_6 = _4 + _4;
match _2.2.0 {
340282366920938463463374607430127080780 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_2.0.2 = _2.0.0;
_2.2 = (1270791371_i32,);
_8.1 = !(-5968715248963629534_i64);
_9 = (-58_isize);
_2.0.1 = 7699911948416640377_u64 as i16;
_6 = _4 - _4;
_2.2.0 = _4 as i32;
_2.2 = (337397117_i32,);
_2.2.0 = 1487247064_i32 * (-1370678605_i32);
_11 = _2.0.2;
_11 = _2.0.0;
_2.0.1 = _8.1 as i16;
_8.1 = 5852068109556753429_i64 ^ 2521566681191077471_i64;
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.0.0 = _11;
_13 = _6;
_13 = _6;
match _9 {
0 => bb12,
1 => bb4,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
340282366920938463463374607431768211398 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_2.2.0 = 1948711656_i32;
_3 = true;
_2.2 = ((-1641130676_i32),);
_2.0 = ('\u{5b9f}', 28657_i16, '\u{3209d}');
_1 = [8497_u16,58834_u16,60366_u16,46457_u16,62602_u16,62224_u16,27639_u16,44664_u16];
_3 = !true;
RET = [_2.2.0,_2.2.0,_2.2.0];
_2.0.1 = 5732_i16 >> _2.2.0;
_3 = !false;
RET = [_2.2.0,_2.2.0,_2.2.0];
_6 = _4 + _4;
match _2.2.0 {
340282366920938463463374607430127080780 => bb13,
_ => bb12
}
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
_10 = _2.2.0 & _2.2.0;
RET = [_10,_10,_10];
_2.0.0 = _11;
_2.2 = (_10,);
_2.2.0 = _10;
RET = [_2.2.0,_10,_2.2.0];
_12 = 4801028414174890647_u64 as f32;
_2.2 = (_10,);
_12 = 5339_u16 as f32;
_17 = _2.2.0 + _10;
_2.0.0 = _11;
_2.2 = (_17,);
_18 = &_9;
_16 = _2.0.2;
_14 = 1381378403000859936_u64 >> _8.1;
RET = [_10,_2.2.0,_17];
_9 = (-102_i8) as isize;
Goto(bb20)
}
bb20 = {
Call(_20 = dump_var(11_usize, 9_usize, Move(_9), 16_usize, Move(_16), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [i32; 3],mut _2: [i32; 3],mut _3: (i32,)) -> char {
mir! {
type RET = char;
let _4: *mut f32;
let _5: f32;
let _6: [u32; 1];
let _7: f32;
let _8: i128;
let _9: u32;
let _10: bool;
let _11: [u32; 8];
let _12: i32;
let _13: &'static (*mut usize, i64);
let _14: &'static i128;
let _15: f32;
let _16: i8;
let _17: [i32; 3];
let _18: [u32; 8];
let _19: &'static *const i16;
let _20: Adt24;
let _21: [u128; 3];
let _22: &'static (*mut usize, i64);
let _23: isize;
let _24: f64;
let _25: (char, i16, char);
let _26: u128;
let _27: isize;
let _28: ([u32; 8], (isize, &'static isize, *mut f32, i8), f32, (*const i16,));
let _29: char;
let _30: u32;
let _31: i32;
let _32: (u16,);
let _33: &'static usize;
let _34: u16;
let _35: *mut u32;
let _36: ();
let _37: ();
{
RET = '\u{d6a96}';
_3.0 = -602762346_i32;
_1 = [_3.0,_3.0,_3.0];
_3 = (745877193_i32,);
RET = '\u{f68b}';
_2 = [_3.0,_3.0,_3.0];
_2 = [_3.0,_3.0,_3.0];
_2 = _1;
_1 = [_3.0,_3.0,_3.0];
_2 = [_3.0,_3.0,_3.0];
RET = '\u{ee13f}';
_3.0 = 1739828271_i32;
_3 = (1129048131_i32,);
_2 = [_3.0,_3.0,_3.0];
_2 = [_3.0,_3.0,_3.0];
_1 = _2;
RET = '\u{10d03a}';
_3 = ((-781989109_i32),);
_2 = [_3.0,_3.0,_3.0];
_4 = core::ptr::addr_of_mut!(_5);
_3 = (1586939796_i32,);
(*_4) = 138148429916562446044008057056748452326_i128 as f32;
RET = '\u{782bb}';
Goto(bb1)
}
bb1 = {
RET = '\u{e33e4}';
_2 = _1;
(*_4) = (-1078890610733164475_i64) as f32;
_5 = 17904_i16 as f32;
_3.0 = (-434582532_i32) & 1130935149_i32;
(*_4) = 108_i8 as f32;
(*_4) = 23332_i16 as f32;
(*_4) = 97_u8 as f32;
RET = '\u{25a6c}';
Call(_5 = fn13(_3.0, Move(_4), _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = (1980889670_i32,);
RET = '\u{4c24a}';
_1 = [_3.0,_3.0,_3.0];
_6 = [2284476938_u32];
RET = '\u{d342b}';
_4 = core::ptr::addr_of_mut!(_7);
Goto(bb3)
}
bb3 = {
_7 = -_5;
_1 = [_3.0,_3.0,_3.0];
_1 = [_3.0,_3.0,_3.0];
RET = '\u{f6bbb}';
(*_4) = 29513_i16 as f32;
_8 = !(-107812789422751174387113086785316926819_i128);
_10 = !true;
_9 = 1441384531_u32 + 3793468525_u32;
_2 = [_3.0,_3.0,_3.0];
_5 = -(*_4);
Goto(bb4)
}
bb4 = {
_1 = _2;
RET = '\u{40c9b}';
_3.0 = -(-1911149380_i32);
_9 = !1465234568_u32;
_9 = 729412694_u32;
_1 = [_3.0,_3.0,_3.0];
_12 = _5 as i32;
_11 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = (*_4) - (*_4);
_7 = _5 - _5;
RET = '\u{bf6c}';
_11 = [_9,_9,_9,_9,_9,_9,_9,_9];
(*_4) = 592958430408995447_u64 as f32;
RET = '\u{a2d2d}';
_12 = _3.0;
Goto(bb5)
}
bb5 = {
_12 = _3.0;
_1 = _2;
_3 = (_12,);
_12 = _3.0 - _3.0;
_10 = !true;
RET = '\u{da5c9}';
_10 = false;
(*_4) = _5;
_2 = [_12,_12,_3.0];
Goto(bb6)
}
bb6 = {
(*_4) = _5 - _5;
_7 = _5;
_6 = [_9];
RET = '\u{77ba9}';
_8 = 110109052668754033180917930853891879923_i128 | 152839923376483768868689139283128280865_i128;
_16 = 122_i8;
Goto(bb7)
}
bb7 = {
(*_4) = _5;
_14 = &_8;
_10 = true;
_12 = _3.0 * _3.0;
_1 = [_12,_12,_3.0];
_9 = !1197220848_u32;
_3.0 = 9223372036854775807_isize as i32;
_17 = _2;
_17 = [_12,_12,_12];
_6 = [_9];
_12 = !_3.0;
_7 = (-9223372036854775808_isize) as f32;
_10 = (*_14) <= (*_14);
_4 = core::ptr::addr_of_mut!((*_4));
_18 = [_9,_9,_9,_9,_9,_9,_9,_9];
(*_4) = -_5;
_11 = _18;
_3.0 = 8100_i16 as i32;
RET = '\u{c38ed}';
_15 = -_7;
_3.0 = (*_14) as i32;
_4 = core::ptr::addr_of_mut!((*_4));
_16 = -(-91_i8);
_21 = [202745781876608318593478802682941686114_u128,154062804134763771535058478263147296068_u128,59780367584532672246740747331628675239_u128];
_10 = true;
RET = '\u{10f6b3}';
_4 = core::ptr::addr_of_mut!(_15);
_11 = _18;
Goto(bb8)
}
bb8 = {
RET = '\u{81346}';
_2 = [_12,_3.0,_12];
Goto(bb9)
}
bb9 = {
_6 = [_9];
_21 = [214686523993408483830348942288068191625_u128,310795799826440624123310734668382533437_u128,329500483976800876065204736128032619078_u128];
_21 = [43504635461998905685018055775217901294_u128,313301114885468964008480631262259452666_u128,49358756337568119888958619157536077449_u128];
_8 = !43770366503789972492722348353163118014_i128;
_11 = [_9,_9,_9,_9,_9,_9,_9,_9];
_7 = _15;
_2 = [_12,_12,_3.0];
(*_4) = _7;
Goto(bb10)
}
bb10 = {
_15 = _7;
_4 = core::ptr::addr_of_mut!(_7);
_15 = _7 + _7;
(*_4) = 27570_i16 as f32;
_18 = _11;
_4 = core::ptr::addr_of_mut!((*_4));
RET = '\u{10be0a}';
_24 = (-701510657294201830_i64) as f64;
Goto(bb11)
}
bb11 = {
_4 = core::ptr::addr_of_mut!(_15);
_25.0 = RET;
_9 = 4076957768_u32 * 2476489428_u32;
_26 = 181634220803768752351483285005886063314_u128;
_7 = -_15;
_25 = (RET, 16264_i16, RET);
_27 = 9223372036854775807_isize;
_21 = [_26,_26,_26];
_28.3.0 = core::ptr::addr_of!(_25.1);
_25.0 = _25.2;
_15 = _7;
_25 = (RET, (-18840_i16), RET);
_8 = 159065462427073231787225569178814130780_i128 + 145642412369795374062661274358931708113_i128;
_7 = -(*_4);
match _25.1 {
0 => bb12,
1 => bb13,
340282366920938463463374607431768192616 => bb15,
_ => bb14
}
}
bb12 = {
_15 = _7;
_4 = core::ptr::addr_of_mut!(_7);
_15 = _7 + _7;
(*_4) = 27570_i16 as f32;
_18 = _11;
_4 = core::ptr::addr_of_mut!((*_4));
RET = '\u{10be0a}';
_24 = (-701510657294201830_i64) as f64;
Goto(bb11)
}
bb13 = {
_12 = _3.0;
_1 = _2;
_3 = (_12,);
_12 = _3.0 - _3.0;
_10 = !true;
RET = '\u{da5c9}';
_10 = false;
(*_4) = _5;
_2 = [_12,_12,_3.0];
Goto(bb6)
}
bb14 = {
_1 = _2;
RET = '\u{40c9b}';
_3.0 = -(-1911149380_i32);
_9 = !1465234568_u32;
_9 = 729412694_u32;
_1 = [_3.0,_3.0,_3.0];
_12 = _5 as i32;
_11 = [_9,_9,_9,_9,_9,_9,_9,_9];
_5 = (*_4) - (*_4);
_7 = _5 - _5;
RET = '\u{bf6c}';
_11 = [_9,_9,_9,_9,_9,_9,_9,_9];
(*_4) = 592958430408995447_u64 as f32;
RET = '\u{a2d2d}';
_12 = _3.0;
Goto(bb5)
}
bb15 = {
_28.1.0 = !_27;
_7 = (*_4) + _15;
_5 = -_15;
(*_4) = _7;
_1 = [_3.0,_3.0,_3.0];
_32 = (52789_u16,);
_1 = [_3.0,_12,_3.0];
_25.0 = _25.2;
_25.1 = _26 as i16;
_18 = [_9,_9,_9,_9,_9,_9,_9,_9];
_28.1.3 = !_16;
_18 = [_9,_9,_9,_9,_9,_9,_9,_9];
_27 = !_28.1.0;
_29 = RET;
_28.1.2 = Move(_4);
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(12_usize, 8_usize, Move(_8), 17_usize, Move(_17), 3_usize, Move(_3), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(12_usize, 2_usize, Move(_2), 21_usize, Move(_21), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(12_usize, 29_usize, Move(_29), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i32,mut _2: *mut f32,mut _3: [i32; 3]) -> f32 {
mir! {
type RET = f32;
let _4: i64;
let _5: Adt57;
let _6: bool;
let _7: *const usize;
let _8: *const i16;
let _9: [char; 2];
let _10: (u32, u16, i128);
let _11: (&'static isize, Adt24, i16);
let _12: f64;
let _13: [u64; 4];
let _14: isize;
let _15: ([u32; 8], (isize, &'static isize, *mut f32, i8), f32, (*const i16,));
let _16: u32;
let _17: [i128; 4];
let _18: char;
let _19: Adt53;
let _20: isize;
let _21: f32;
let _22: ([u32; 8], (isize, &'static isize, *mut f32, i8), f32, (*const i16,));
let _23: ();
let _24: ();
{
RET = 25741_u16 as f32;
_3 = [_1,_1,_1];
_3 = [_1,_1,_1];
RET = 77_isize as f32;
RET = 23_i8 as f32;
_2 = core::ptr::addr_of_mut!(RET);
RET = 12170503059457726675_usize as f32;
_1 = (-397681113_i32) | (-301623266_i32);
(*_2) = (-1900655458244816932_i64) as f32;
_1 = 9223372036854775807_isize as i32;
_1 = (-670582477_i32) * (-2050234631_i32);
_4 = 5376357582476957186_i64 >> _1;
_4 = _1 as i64;
_2 = core::ptr::addr_of_mut!((*_2));
_1 = -(-267215477_i32);
_4 = (-3187058694057571757_i64);
_1 = 442596655_i32;
match _4 {
340282366920938463460187548737710639699 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_1 = (-15150_i16) as i32;
_6 = !false;
_6 = !true;
RET = 11132826215930097172_usize as f32;
_1 = 14596480277950564456_u64 as i32;
_6 = !false;
_4 = !(-3304145622160870868_i64);
_10.1 = !35338_u16;
_10 = (3594062992_u32, 25816_u16, 44338134469119656876083573728887144795_i128);
RET = _10.0 as f32;
_10.1 = !54513_u16;
_3 = [_1,_1,_1];
_3 = [_1,_1,_1];
_11.2 = (-18041_i16);
_13 = [12755176045424242267_u64,1940634651137970136_u64,7902367894482215559_u64,2673463599866229316_u64];
_1 = (-847297955_i32) - 1889761280_i32;
_4 = 17199511369734684761_usize as i64;
Goto(bb3)
}
bb3 = {
_10.1 = 32527_u16 + 13104_u16;
_13 = [13688535543263873735_u64,6827968827135007643_u64,4871468909562601162_u64,14493065071176307106_u64];
_10 = (4100801255_u32, 53075_u16, 64623540718774061486903967094065299053_i128);
_1 = 1779800653_i32 | 1868949704_i32;
_1 = -1026264263_i32;
_13 = [1965876585463028635_u64,13660061381857193027_u64,17775047443594068528_u64,8478685137642071258_u64];
match _10.2 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
64623540718774061486903967094065299053 => bb11,
_ => bb10
}
}
bb4 = {
_1 = (-15150_i16) as i32;
_6 = !false;
_6 = !true;
RET = 11132826215930097172_usize as f32;
_1 = 14596480277950564456_u64 as i32;
_6 = !false;
_4 = !(-3304145622160870868_i64);
_10.1 = !35338_u16;
_10 = (3594062992_u32, 25816_u16, 44338134469119656876083573728887144795_i128);
RET = _10.0 as f32;
_10.1 = !54513_u16;
_3 = [_1,_1,_1];
_3 = [_1,_1,_1];
_11.2 = (-18041_i16);
_13 = [12755176045424242267_u64,1940634651137970136_u64,7902367894482215559_u64,2673463599866229316_u64];
_1 = (-847297955_i32) - 1889761280_i32;
_4 = 17199511369734684761_usize as i64;
Goto(bb3)
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
_11.0 = &_14;
_13 = [14689712507487285100_u64,5249101384612806131_u64,5123106161982331660_u64,8440710391441354146_u64];
_15.1.0 = 11106141341400277100_u64 as isize;
_9 = ['\u{7876e}','\u{d890c}'];
_13 = [7009503816221823378_u64,4969507889498343718_u64,14059048150442116232_u64,4936347848642352287_u64];
_14 = _15.1.0;
RET = _1 as f32;
_10.2 = !41351289795117279756737937343868635269_i128;
_18 = '\u{f5fa0}';
_2 = core::ptr::addr_of_mut!((*_2));
_10 = (653023476_u32, 12959_u16, 24360062957042481575243398012914832561_i128);
match _10.2 {
24360062957042481575243398012914832561 => bb12,
_ => bb9
}
}
bb12 = {
_3 = [_1,_1,_1];
_10.0 = !1983750406_u32;
Goto(bb13)
}
bb13 = {
(*_2) = 223043164856033246983575789726441422718_u128 as f32;
_10.2 = _4 as i128;
_15.2 = _15.1.0 as f32;
_17 = [_10.2,_10.2,_10.2,_10.2];
match _10.1 {
12959 => bb14,
_ => bb6
}
}
bb14 = {
_8 = core::ptr::addr_of!(_11.2);
_15.1.2 = core::ptr::addr_of_mut!(_15.2);
_3 = [_1,_1,_1];
_10.2 = 67499340618097664149719019523165634091_i128 << _11.2;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(13_usize, 1_usize, Move(_1), 18_usize, Move(_18), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(13_usize, 9_usize, Move(_9), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: &'static f64,mut _2: i128,mut _3: i8) -> u32 {
mir! {
type RET = u32;
let _4: Adt25;
let _5: (&'static isize, Adt24, i16);
let _6: *mut f32;
let _7: Adt73;
let _8: f64;
let _9: isize;
let _10: [i128; 4];
let _11: [u16; 8];
let _12: isize;
let _13: isize;
let _14: [char; 2];
let _15: *mut (isize, &'static isize, *mut f32, i8);
let _16: u64;
let _17: bool;
let _18: &'static (*mut usize, i64);
let _19: &'static usize;
let _20: ();
let _21: ();
{
_3 = 12_i8;
RET = 11741084141442357659_u64 as u32;
_3 = (-14_i8);
RET = 3117376477_u32;
RET = 1324807201_u32 | 214648148_u32;
_5.2 = (-8850_i16) & (-13438_i16);
_3 = !(-110_i8);
_3 = (-55_i8) << RET;
_5.2 = -(-31832_i16);
_3 = 39355_u16 as i8;
_4 = Adt25::Variant3 { fld0: 6076082824458752947_u64 };
_2 = (-99404068944302201549822572077996671915_i128) * (-122438105768881846621094926770891220714_i128);
_2 = (-95491183376493731634092318011495918298_i128);
_5.2 = 24227_i16 & (-21058_i16);
_5.2 = -28203_i16;
place!(Field::<u64>(Variant(_4, 3), 0)) = 15252733626140763972_u64 & 6392221063104163886_u64;
Goto(bb1)
}
bb1 = {
_1 = &_8;
place!(Field::<u64>(Variant(_4, 3), 0)) = _3 as u64;
RET = !2642479658_u32;
_5.0 = &_9;
_9 = Field::<u64>(Variant(_4, 3), 0) as isize;
_8 = 357313370_i32 as f64;
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
244791183544444731829282289420272293158 => bb10,
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
_1 = &_8;
_1 = &(*_1);
_8 = RET as f64;
_3 = 76_i8 << _9;
_12 = !_9;
_8 = 53032_u16 as f64;
RET = 3618731828_u32 - 578984245_u32;
_10 = [_2,_2,_2,_2];
_7 = Adt73::Variant0 { fld0: 65261376377551140377598078054940745496_u128 };
RET = Field::<u64>(Variant(_4, 3), 0) as u32;
place!(Field::<u128>(Variant(_7, 0), 0)) = 180399041254446119599720353646670604237_u128;
_11 = [10232_u16,58886_u16,46357_u16,55926_u16,55185_u16,53469_u16,62720_u16,1885_u16];
_12 = _2 as isize;
_3 = !6_i8;
Call(_6 = fn15(_11, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<u64>(Variant(_4, 3), 0)) = 7131670089882878088_u64;
_3 = (-51_i8);
_2 = 11864398048550836309859784701230936756_i128 & (-31455492870460812774347777709931777197_i128);
_10 = [_2,_2,_2,_2];
SetDiscriminant(_7, 0);
Goto(bb12)
}
bb12 = {
_10 = [_2,_2,_2,_2];
_3 = 59_i8 << RET;
_13 = 22_u8 as isize;
place!(Field::<u128>(Variant(_7, 0), 0)) = 60954612465440898164148876136792520334_u128;
_13 = -_12;
_5.2 = 21843_u16 as i16;
_3 = 9_i8;
_5.0 = &_13;
place!(Field::<u128>(Variant(_7, 0), 0)) = 206321962408218245981796264289601663671_u128 - 144543100573836097378666900872616560806_u128;
Goto(bb13)
}
bb13 = {
_1 = &_8;
_10 = [_2,_2,_2,_2];
_12 = _9 + _9;
_16 = Field::<u64>(Variant(_4, 3), 0) & Field::<u64>(Variant(_4, 3), 0);
_16 = Field::<u64>(Variant(_4, 3), 0);
_17 = !false;
_12 = _5.2 as isize;
_9 = _12;
_3 = 69_i8;
_16 = Field::<u64>(Variant(_4, 3), 0) | Field::<u64>(Variant(_4, 3), 0);
_14 = ['\u{70488}','\u{87817}'];
SetDiscriminant(_4, 3);
_10 = [_2,_2,_2,_2];
Goto(bb14)
}
bb14 = {
_12 = -_9;
_4 = Adt25::Variant3 { fld0: _16 };
place!(Field::<u64>(Variant(_4, 3), 0)) = _16 << Field::<u128>(Variant(_7, 0), 0);
_12 = _9;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(14_usize, 14_usize, Move(_14), 16_usize, Move(_16), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(14_usize, 13_usize, Move(_13), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [u16; 8],mut _2: [i128; 4]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _3: f64;
let _4: &'static i64;
let _5: char;
let _6: f32;
let _7: isize;
let _8: &'static (*mut i64, (char, i16, char));
let _9: *const Adt57;
let _10: i32;
let _11: &'static isize;
let _12: i8;
let _13: i16;
let _14: i128;
let _15: [u64; 4];
let _16: *mut *const i16;
let _17: isize;
let _18: char;
let _19: [i16; 7];
let _20: bool;
let _21: i128;
let _22: *const i16;
let _23: (f32,);
let _24: &'static isize;
let _25: &'static isize;
let _26: &'static i64;
let _27: ();
let _28: ();
{
_1 = [1335_u16,45731_u16,38516_u16,17761_u16,9635_u16,54911_u16,36550_u16,41315_u16];
_1 = [36271_u16,48783_u16,12410_u16,18574_u16,51692_u16,59529_u16,47518_u16,39113_u16];
_2 = [(-127930511398241295743156464018497047656_i128),(-99301393005754288129538928283636575026_i128),(-136833939805429514268918314903446228847_i128),27818212423500611617686376575612641274_i128];
_2 = [(-52198577348199512325049056862131918424_i128),(-38793863822711905188287258608747918791_i128),(-141477704562157265542282836759574989023_i128),7809908223896781062047878081683755770_i128];
_3 = 6480029694996302509341441109799945170_i128 as f64;
Goto(bb1)
}
bb1 = {
_3 = 2490151244_u32 as f64;
Goto(bb2)
}
bb2 = {
_5 = '\u{4b411}';
_2 = [(-121573735539582591929316817452473554624_i128),(-15941037349919327176887281576628209506_i128),(-150871772108956515429425390185330841896_i128),138679351445557916939675798140089174178_i128];
_1 = [52670_u16,613_u16,41070_u16,14341_u16,29571_u16,44497_u16,33498_u16,7653_u16];
_1 = [42696_u16,59240_u16,1758_u16,18808_u16,33802_u16,51975_u16,14681_u16,770_u16];
RET = core::ptr::addr_of_mut!(_6);
(*RET) = 139_u8 as f32;
(*RET) = (-1115878903_i32) as f32;
_7 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_1 = [10623_u16,34298_u16,18411_u16,639_u16,30807_u16,42226_u16,34325_u16,5401_u16];
_1 = [13616_u16,49589_u16,57999_u16,60409_u16,37791_u16,20250_u16,10648_u16,13089_u16];
RET = core::ptr::addr_of_mut!((*RET));
_6 = 5230815734995423171_u64 as f32;
_5 = '\u{84c3a}';
_5 = '\u{7e4fe}';
_1 = [35968_u16,14016_u16,22752_u16,61377_u16,58124_u16,6307_u16,3178_u16,34778_u16];
_10 = 2097134684_i32;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
2097134684 => bb6,
_ => bb5
}
}
bb3 = {
_3 = 2490151244_u32 as f64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_7 = 136893704_u32 as isize;
(*RET) = 4_usize as f32;
_7 = (*RET) as isize;
_12 = -56_i8;
_5 = '\u{84be0}';
_11 = &_7;
(*RET) = (-27898_i16) as f32;
Goto(bb7)
}
bb7 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = (-1817121568753768362_i64) as f32;
RET = core::ptr::addr_of_mut!((*RET));
_3 = 3593318215_u32 as f64;
_3 = 250320166873997181281903538606690026675_u128 as f64;
_10 = 282505476_i32;
(*RET) = _10 as f32;
_14 = 18120795002668137088_u64 as i128;
_3 = _14 as f64;
Call(RET = fn16(Move(_11), _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11 = &_7;
_14 = -(-142965651976160825251603142778134089133_i128);
_10 = 1394669243_i32 ^ 1810616795_i32;
_15 = [11364922406966502445_u64,12675885580014069824_u64,16891577822209076408_u64,11649403061709725538_u64];
_11 = &(*_11);
_14 = 157454296446593760242317700352128353793_i128 - 64277093533462637871162902714762619851_i128;
_11 = &(*_11);
_6 = 53661_u16 as f32;
_2 = [_14,_14,_14,_14];
_13 = 12384752800804693918_u64 as i16;
_1 = [452_u16,60534_u16,30740_u16,50197_u16,49181_u16,64740_u16,10184_u16,62778_u16];
_3 = 2034721536654383639_i64 as f64;
_11 = &(*_11);
_13 = !(-17359_i16);
_14 = -(-147733014257581736701380033704320575225_i128);
RET = core::ptr::addr_of_mut!(_6);
_3 = _13 as f64;
(*RET) = 425608242_u32 as f32;
_3 = (-3572861173743953830_i64) as f64;
Goto(bb9)
}
bb9 = {
_15 = [11786993918498936849_u64,5850514166680352435_u64,16756987663110648499_u64,7699260224724159168_u64];
_13 = 22538_i16;
_13 = _14 as i16;
_15 = [12241688381373241647_u64,266446094082848336_u64,2876426583747946372_u64,3522738510144522215_u64];
_2 = [_14,_14,_14,_14];
_5 = '\u{962a0}';
_7 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_10 = (-45545964_i32) & 188499801_i32;
_2 = [_14,_14,_14,_14];
RET = core::ptr::addr_of_mut!((*RET));
_10 = !(-118311599_i32);
(*RET) = _14 as f32;
(*RET) = 17900832039195040004_u64 as f32;
_3 = 11663500283298150549_u64 as f64;
_10 = !1177391955_i32;
_6 = 7307085390172822565_u64 as f32;
(*RET) = 16634362446174666714_u64 as f32;
(*RET) = 5_usize as f32;
_3 = _7 as f64;
Call(_10 = core::intrinsics::transmute(_5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = _12 as f64;
_19 = [_13,_13,_13,_13,_13,_13,_13];
_10 = 524302748_i32;
_7 = 9223372036854775807_isize * 9223372036854775807_isize;
_21 = _14 ^ _14;
_1 = [48713_u16,43042_u16,12796_u16,20926_u16,64450_u16,21902_u16,60477_u16,38112_u16];
match _10 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb11,
6 => bb12,
524302748 => bb14,
_ => bb13
}
}
bb11 = {
_7 = 136893704_u32 as isize;
(*RET) = 4_usize as f32;
_7 = (*RET) as isize;
_12 = -56_i8;
_5 = '\u{84be0}';
_11 = &_7;
(*RET) = (-27898_i16) as f32;
Goto(bb7)
}
bb12 = {
_3 = 2490151244_u32 as f64;
Goto(bb2)
}
bb13 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = (-1817121568753768362_i64) as f32;
RET = core::ptr::addr_of_mut!((*RET));
_3 = 3593318215_u32 as f64;
_3 = 250320166873997181281903538606690026675_u128 as f64;
_10 = 282505476_i32;
(*RET) = _10 as f32;
_14 = 18120795002668137088_u64 as i128;
_3 = _14 as f64;
Call(RET = fn16(Move(_11), _1), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_18 = _5;
_13 = !17246_i16;
_10 = -(-990151808_i32);
(*RET) = 4149237069_u32 as f32;
_18 = _5;
_18 = _5;
_18 = _5;
(*RET) = 4043046357767448206_usize as f32;
_3 = _13 as f64;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(15_usize, 13_usize, Move(_13), 5_usize, Move(_5), 19_usize, Move(_19), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(15_usize, 7_usize, Move(_7), 14_usize, Move(_14), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: &'static isize,mut _2: [u16; 8]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _3: Adt50;
let _4: bool;
let _5: [char; 2];
let _6: Adt75;
let _7: [u8; 7];
let _8: &'static u32;
let _9: [u32; 1];
let _10: Adt53;
let _11: isize;
let _12: isize;
let _13: Adt33;
let _14: i16;
let _15: &'static u32;
let _16: char;
let _17: ([u64; 4], (u16,));
let _18: &'static [u8; 5];
let _19: isize;
let _20: u16;
let _21: &'static i128;
let _22: &'static usize;
let _23: &'static i64;
let _24: u32;
let _25: *mut *const i16;
let _26: [i16; 7];
let _27: i128;
let _28: &'static i32;
let _29: *mut f32;
let _30: bool;
let _31: u8;
let _32: &'static i64;
let _33: *const i16;
let _34: f32;
let _35: [u128; 3];
let _36: ();
let _37: ();
{
_2 = [17250_u16,59777_u16,19813_u16,34242_u16,36814_u16,21741_u16,22059_u16,21652_u16];
_2 = [52205_u16,31862_u16,59308_u16,46072_u16,31115_u16,36295_u16,38725_u16,61207_u16];
_2 = [35880_u16,35883_u16,33075_u16,39124_u16,63299_u16,10910_u16,21494_u16,11954_u16];
_2 = [50270_u16,41372_u16,6729_u16,19985_u16,13771_u16,5325_u16,56492_u16,16945_u16];
_2 = [18455_u16,52198_u16,11340_u16,59991_u16,36714_u16,12824_u16,22393_u16,15590_u16];
_2 = [6154_u16,2367_u16,21891_u16,55270_u16,21693_u16,51261_u16,24792_u16,56204_u16];
_2 = [60928_u16,55242_u16,16646_u16,51008_u16,57994_u16,29754_u16,47415_u16,21913_u16];
_2 = [55686_u16,28085_u16,341_u16,53543_u16,45559_u16,16749_u16,18297_u16,9077_u16];
_2 = [37692_u16,16602_u16,25656_u16,57428_u16,16846_u16,60377_u16,30545_u16,32101_u16];
_2 = [13849_u16,52872_u16,18787_u16,60194_u16,17448_u16,59994_u16,15195_u16,48499_u16];
_2 = [54924_u16,5755_u16,43404_u16,55010_u16,45439_u16,60433_u16,2862_u16,59571_u16];
_2 = [43631_u16,5079_u16,789_u16,15557_u16,8138_u16,10951_u16,3850_u16,32186_u16];
_2 = [19785_u16,49607_u16,47981_u16,16856_u16,2023_u16,45429_u16,62256_u16,7812_u16];
_2 = [58602_u16,44755_u16,30045_u16,42772_u16,38473_u16,42236_u16,35333_u16,24516_u16];
_2 = [37252_u16,9740_u16,41714_u16,30844_u16,58837_u16,34940_u16,4195_u16,1589_u16];
_2 = [55506_u16,53409_u16,4389_u16,61732_u16,50875_u16,22962_u16,11266_u16,35037_u16];
_2 = [33479_u16,15464_u16,23928_u16,12889_u16,49952_u16,1376_u16,8223_u16,14709_u16];
_2 = [30946_u16,13946_u16,54831_u16,51598_u16,31186_u16,54596_u16,32198_u16,47259_u16];
Goto(bb1)
}
bb1 = {
_2 = [17271_u16,51293_u16,8582_u16,45938_u16,36195_u16,64571_u16,20604_u16,18003_u16];
Goto(bb2)
}
bb2 = {
_2 = [12738_u16,41250_u16,30877_u16,64986_u16,52083_u16,54610_u16,49842_u16,119_u16];
_2 = [17571_u16,4955_u16,17496_u16,51695_u16,12118_u16,60791_u16,19283_u16,35273_u16];
_2 = [30307_u16,35919_u16,54494_u16,10224_u16,52856_u16,23960_u16,3796_u16,58324_u16];
_4 = false;
_2 = [5055_u16,34089_u16,42192_u16,42076_u16,11226_u16,23788_u16,15033_u16,52529_u16];
_4 = 5743127113719266577_i64 > 1308407447926092966_i64;
_2 = [15055_u16,47774_u16,52626_u16,18902_u16,3023_u16,28899_u16,33277_u16,995_u16];
_4 = false | false;
_4 = false | true;
_5 = ['\u{ff9d7}','\u{4e6de}'];
_4 = false;
_4 = !false;
_4 = true;
_2 = [11481_u16,22450_u16,31632_u16,39059_u16,2417_u16,35082_u16,61996_u16,819_u16];
_2 = [24612_u16,26607_u16,35755_u16,63499_u16,61315_u16,55517_u16,48417_u16,49727_u16];
_2 = [36795_u16,29875_u16,26455_u16,2203_u16,22411_u16,57445_u16,1933_u16,59230_u16];
_5 = ['\u{ad207}','\u{20cd3}'];
_4 = !true;
_2 = [49406_u16,49788_u16,43665_u16,61441_u16,2915_u16,28132_u16,4006_u16,55990_u16];
_7 = [188_u8,71_u8,69_u8,177_u8,177_u8,127_u8,221_u8];
_4 = true;
_9 = [2705299555_u32];
_5 = ['\u{9954f}','\u{a8435}'];
Goto(bb3)
}
bb3 = {
_4 = true;
_5 = ['\u{cc4af}','\u{8fe87}'];
_2 = [23706_u16,51260_u16,55661_u16,37818_u16,59735_u16,42810_u16,40637_u16,57665_u16];
_2 = [14817_u16,4689_u16,39587_u16,32806_u16,63701_u16,17064_u16,36499_u16,61716_u16];
_7 = [96_u8,211_u8,244_u8,174_u8,129_u8,113_u8,140_u8];
_4 = false;
_4 = false;
_12 = (-9223372036854775808_isize);
_1 = &_12;
match _12 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb4 = {
_2 = [12738_u16,41250_u16,30877_u16,64986_u16,52083_u16,54610_u16,49842_u16,119_u16];
_2 = [17571_u16,4955_u16,17496_u16,51695_u16,12118_u16,60791_u16,19283_u16,35273_u16];
_2 = [30307_u16,35919_u16,54494_u16,10224_u16,52856_u16,23960_u16,3796_u16,58324_u16];
_4 = false;
_2 = [5055_u16,34089_u16,42192_u16,42076_u16,11226_u16,23788_u16,15033_u16,52529_u16];
_4 = 5743127113719266577_i64 > 1308407447926092966_i64;
_2 = [15055_u16,47774_u16,52626_u16,18902_u16,3023_u16,28899_u16,33277_u16,995_u16];
_4 = false | false;
_4 = false | true;
_5 = ['\u{ff9d7}','\u{4e6de}'];
_4 = false;
_4 = !false;
_4 = true;
_2 = [11481_u16,22450_u16,31632_u16,39059_u16,2417_u16,35082_u16,61996_u16,819_u16];
_2 = [24612_u16,26607_u16,35755_u16,63499_u16,61315_u16,55517_u16,48417_u16,49727_u16];
_2 = [36795_u16,29875_u16,26455_u16,2203_u16,22411_u16,57445_u16,1933_u16,59230_u16];
_5 = ['\u{ad207}','\u{20cd3}'];
_4 = !true;
_2 = [49406_u16,49788_u16,43665_u16,61441_u16,2915_u16,28132_u16,4006_u16,55990_u16];
_7 = [188_u8,71_u8,69_u8,177_u8,177_u8,127_u8,221_u8];
_4 = true;
_9 = [2705299555_u32];
_5 = ['\u{9954f}','\u{a8435}'];
Goto(bb3)
}
bb5 = {
_2 = [17271_u16,51293_u16,8582_u16,45938_u16,36195_u16,64571_u16,20604_u16,18003_u16];
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
_9 = [1561135932_u32];
_5 = ['\u{6abce}','\u{10f5c3}'];
_11 = (*_1) + _12;
_5 = ['\u{be442}','\u{2d362}'];
_2 = [12374_u16,2205_u16,23130_u16,63757_u16,22158_u16,19153_u16,201_u16,36103_u16];
_1 = &(*_1);
_12 = _11;
_2 = [4906_u16,25133_u16,35115_u16,45102_u16,61381_u16,20769_u16,50242_u16,53610_u16];
Goto(bb12)
}
bb12 = {
_7 = [102_u8,210_u8,117_u8,170_u8,173_u8,96_u8,223_u8];
Call(_5 = fn17(_4, _12, _2, _2, _2, _7, _7, _2, _12, _2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_1 = &_11;
_2 = [54046_u16,32640_u16,62497_u16,4778_u16,50752_u16,60142_u16,56022_u16,58114_u16];
Goto(bb14)
}
bb14 = {
_14 = (-30184_i16);
_2 = [44907_u16,41677_u16,23976_u16,21478_u16,3270_u16,34836_u16,3514_u16,44553_u16];
_1 = &_11;
_1 = &_11;
_7 = [220_u8,9_u8,124_u8,226_u8,105_u8,16_u8,153_u8];
_1 = &(*_1);
_2 = [20288_u16,63704_u16,33084_u16,1216_u16,59354_u16,866_u16,61604_u16,29191_u16];
_12 = (-78_i8) as isize;
_17.1.0 = 17297_u16;
_5 = ['\u{9dd13}','\u{43224}'];
_16 = '\u{1ebdb}';
_1 = &_11;
_2 = [_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0];
_11 = _12 << _14;
_17.1.0 = 62240205914467984238052576439409334106_i128 as u16;
_11 = _12 | _12;
_7 = [84_u8,229_u8,110_u8,202_u8,190_u8,95_u8,203_u8];
_14 = 1491_i16;
match _14 {
0 => bb5,
1 => bb9,
2 => bb11,
1491 => bb15,
_ => bb10
}
}
bb15 = {
_9 = [1182126439_u32];
_7 = [0_u8,149_u8,176_u8,109_u8,138_u8,21_u8,240_u8];
_17.0 = [17070916158755410757_u64,3479983176282961572_u64,12513967724111541059_u64,3805825122471144479_u64];
_9 = [3010969640_u32];
_11 = _12 * _12;
_17.1.0 = (-147548371187131311318313284992999524525_i128) as u16;
_20 = _17.1.0 * _17.1.0;
_11 = _12;
_5 = [_16,_16];
_17.1 = (_20,);
_11 = !_12;
_17.1.0 = !_20;
_11 = _12 & _12;
_4 = _16 >= _16;
_11 = _12 * _12;
_19 = 94_u8 as isize;
_2 = [_17.1.0,_17.1.0,_17.1.0,_20,_17.1.0,_20,_20,_20];
_4 = !true;
_19 = _12;
_9 = [310556920_u32];
_7 = [7_u8,139_u8,170_u8,211_u8,83_u8,16_u8,170_u8];
_17.1.0 = _20 | _20;
_20 = !_17.1.0;
_9 = [3689691965_u32];
match _14 {
0 => bb6,
1 => bb4,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
1491 => bb22,
_ => bb21
}
}
bb16 = {
_14 = (-30184_i16);
_2 = [44907_u16,41677_u16,23976_u16,21478_u16,3270_u16,34836_u16,3514_u16,44553_u16];
_1 = &_11;
_1 = &_11;
_7 = [220_u8,9_u8,124_u8,226_u8,105_u8,16_u8,153_u8];
_1 = &(*_1);
_2 = [20288_u16,63704_u16,33084_u16,1216_u16,59354_u16,866_u16,61604_u16,29191_u16];
_12 = (-78_i8) as isize;
_17.1.0 = 17297_u16;
_5 = ['\u{9dd13}','\u{43224}'];
_16 = '\u{1ebdb}';
_1 = &_11;
_2 = [_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0,_17.1.0];
_11 = _12 << _14;
_17.1.0 = 62240205914467984238052576439409334106_i128 as u16;
_11 = _12 | _12;
_7 = [84_u8,229_u8,110_u8,202_u8,190_u8,95_u8,203_u8];
_14 = 1491_i16;
match _14 {
0 => bb5,
1 => bb9,
2 => bb11,
1491 => bb15,
_ => bb10
}
}
bb17 = {
_1 = &_11;
_2 = [54046_u16,32640_u16,62497_u16,4778_u16,50752_u16,60142_u16,56022_u16,58114_u16];
Goto(bb14)
}
bb18 = {
_4 = true;
_5 = ['\u{cc4af}','\u{8fe87}'];
_2 = [23706_u16,51260_u16,55661_u16,37818_u16,59735_u16,42810_u16,40637_u16,57665_u16];
_2 = [14817_u16,4689_u16,39587_u16,32806_u16,63701_u16,17064_u16,36499_u16,61716_u16];
_7 = [96_u8,211_u8,244_u8,174_u8,129_u8,113_u8,140_u8];
_4 = false;
_4 = false;
_12 = (-9223372036854775808_isize);
_1 = &_12;
match _12 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb19 = {
_9 = [1561135932_u32];
_5 = ['\u{6abce}','\u{10f5c3}'];
_11 = (*_1) + _12;
_5 = ['\u{be442}','\u{2d362}'];
_2 = [12374_u16,2205_u16,23130_u16,63757_u16,22158_u16,19153_u16,201_u16,36103_u16];
_1 = &(*_1);
_12 = _11;
_2 = [4906_u16,25133_u16,35115_u16,45102_u16,61381_u16,20769_u16,50242_u16,53610_u16];
Goto(bb12)
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
_17.1.0 = _20 | _20;
_1 = &_12;
_17.1.0 = !_20;
_17.1 = (_20,);
_24 = !4062365255_u32;
_5 = [_16,_16];
_8 = &_24;
_1 = &_12;
_15 = Move(_8);
_21 = &_27;
_15 = &_24;
_19 = 789558428176550330_u64 as isize;
_9 = [(*_15)];
_20 = _17.1.0;
_17.1 = (_20,);
match _14 {
0 => bb13,
1 => bb4,
2 => bb23,
1491 => bb25,
_ => bb24
}
}
bb23 = {
_2 = [12738_u16,41250_u16,30877_u16,64986_u16,52083_u16,54610_u16,49842_u16,119_u16];
_2 = [17571_u16,4955_u16,17496_u16,51695_u16,12118_u16,60791_u16,19283_u16,35273_u16];
_2 = [30307_u16,35919_u16,54494_u16,10224_u16,52856_u16,23960_u16,3796_u16,58324_u16];
_4 = false;
_2 = [5055_u16,34089_u16,42192_u16,42076_u16,11226_u16,23788_u16,15033_u16,52529_u16];
_4 = 5743127113719266577_i64 > 1308407447926092966_i64;
_2 = [15055_u16,47774_u16,52626_u16,18902_u16,3023_u16,28899_u16,33277_u16,995_u16];
_4 = false | false;
_4 = false | true;
_5 = ['\u{ff9d7}','\u{4e6de}'];
_4 = false;
_4 = !false;
_4 = true;
_2 = [11481_u16,22450_u16,31632_u16,39059_u16,2417_u16,35082_u16,61996_u16,819_u16];
_2 = [24612_u16,26607_u16,35755_u16,63499_u16,61315_u16,55517_u16,48417_u16,49727_u16];
_2 = [36795_u16,29875_u16,26455_u16,2203_u16,22411_u16,57445_u16,1933_u16,59230_u16];
_5 = ['\u{ad207}','\u{20cd3}'];
_4 = !true;
_2 = [49406_u16,49788_u16,43665_u16,61441_u16,2915_u16,28132_u16,4006_u16,55990_u16];
_7 = [188_u8,71_u8,69_u8,177_u8,177_u8,127_u8,221_u8];
_4 = true;
_9 = [2705299555_u32];
_5 = ['\u{9954f}','\u{a8435}'];
Goto(bb3)
}
bb24 = {
Return()
}
bb25 = {
_1 = &(*_1);
_2 = [_20,_20,_20,_17.1.0,_17.1.0,_20,_20,_17.1.0];
_27 = 6508107787249936773620041893579600703_i128 << _20;
_16 = '\u{e7b40}';
_21 = &_27;
_1 = &(*_1);
_24 = 2946308493_u32;
_26 = [_14,_14,_14,_14,_14,_14,_14];
_9 = [_24];
_15 = &_24;
_27 = (-129964558641467530298265102474814756164_i128);
Goto(bb26)
}
bb26 = {
_8 = &(*_15);
_19 = (-7923559502373153167_i64) as isize;
_27 = _16 as i128;
_12 = _11;
_14 = (*_8) as i16;
_27 = -(-155849954183419311410987597565554307179_i128);
_2 = [_20,_17.1.0,_20,_17.1.0,_20,_17.1.0,_20,_20];
_1 = &_12;
_1 = &(*_1);
_30 = _20 == _17.1.0;
_17.0 = [13176243590240780993_u64,17530126142193011094_u64,729254699642762715_u64,17224808898848521046_u64];
_21 = &_27;
_1 = &(*_1);
_4 = _30;
_17.1 = (_20,);
_7 = [48_u8,178_u8,112_u8,203_u8,14_u8,28_u8,240_u8];
Goto(bb27)
}
bb27 = {
_31 = 235_u8;
_25 = core::ptr::addr_of_mut!(_33);
_7 = [_31,_31,_31,_31,_31,_31,_31];
_31 = 2343382788462269973_i64 as u8;
_33 = core::ptr::addr_of!(_14);
_7 = [_31,_31,_31,_31,_31,_31,_31];
_25 = core::ptr::addr_of_mut!(_33);
RET = core::ptr::addr_of_mut!(_34);
(*RET) = _31 as f32;
(*_25) = core::ptr::addr_of!(_14);
_17.0 = [4956519611308234587_u64,15323851238735529682_u64,6034396561080319795_u64,266086460303027553_u64];
_17.1.0 = _20;
_20 = !_17.1.0;
Goto(bb28)
}
bb28 = {
Call(_36 = dump_var(16_usize, 26_usize, Move(_26), 31_usize, Move(_31), 30_usize, Move(_30), 27_usize, Move(_27)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_36 = dump_var(16_usize, 19_usize, Move(_19), 16_usize, Move(_16), 14_usize, Move(_14), 24_usize, Move(_24)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: bool,mut _2: isize,mut _3: [u16; 8],mut _4: [u16; 8],mut _5: [u16; 8],mut _6: [u8; 7],mut _7: [u8; 7],mut _8: [u16; 8],mut _9: isize,mut _10: [u16; 8]) -> [char; 2] {
mir! {
type RET = [char; 2];
let _11: u32;
let _12: isize;
let _13: char;
let _14: Adt53;
let _15: *mut (isize, &'static isize, *mut f32, i8);
let _16: *const *mut f32;
let _17: char;
let _18: (char, i16, char);
let _19: *const *mut f32;
let _20: &'static isize;
let _21: u64;
let _22: bool;
let _23: bool;
let _24: u128;
let _25: &'static &'static u16;
let _26: bool;
let _27: isize;
let _28: isize;
let _29: [u64; 4];
let _30: *mut u8;
let _31: (char, i16, char);
let _32: Adt73;
let _33: u32;
let _34: ();
let _35: ();
{
_6 = [58_u8,122_u8,181_u8,81_u8,183_u8,39_u8,154_u8];
Goto(bb1)
}
bb1 = {
RET = ['\u{5078a}','\u{38f45}'];
_6 = [51_u8,149_u8,27_u8,106_u8,11_u8,20_u8,175_u8];
RET = ['\u{bc0df}','\u{bc0c7}'];
_4 = [877_u16,13539_u16,60251_u16,33308_u16,52116_u16,26002_u16,47062_u16,21624_u16];
_13 = '\u{ad5e4}';
Goto(bb2)
}
bb2 = {
_6 = [136_u8,181_u8,98_u8,174_u8,181_u8,94_u8,50_u8];
_7 = [140_u8,45_u8,210_u8,133_u8,202_u8,58_u8,235_u8];
_13 = '\u{aac98}';
_12 = !_9;
Goto(bb3)
}
bb3 = {
_4 = _5;
RET = [_13,_13];
_8 = [28134_u16,43104_u16,9992_u16,46344_u16,5158_u16,59345_u16,2702_u16,32831_u16];
_5 = [26326_u16,32852_u16,58345_u16,21385_u16,51683_u16,13934_u16,44528_u16,55042_u16];
_7 = [24_u8,130_u8,235_u8,108_u8,42_u8,243_u8,60_u8];
_3 = [36754_u16,27619_u16,57462_u16,29151_u16,14406_u16,19528_u16,10347_u16,35712_u16];
_6 = [111_u8,126_u8,209_u8,234_u8,27_u8,53_u8,252_u8];
_3 = _10;
_1 = true;
_9 = _2;
_2 = -_9;
_11 = 1064695947_u32;
_12 = _13 as isize;
_6 = [6_u8,150_u8,66_u8,60_u8,26_u8,25_u8,161_u8];
match _11 {
1064695947 => bb5,
_ => bb4
}
}
bb4 = {
_6 = [136_u8,181_u8,98_u8,174_u8,181_u8,94_u8,50_u8];
_7 = [140_u8,45_u8,210_u8,133_u8,202_u8,58_u8,235_u8];
_13 = '\u{aac98}';
_12 = !_9;
Goto(bb3)
}
bb5 = {
_12 = _13 as isize;
_2 = -_9;
_7 = [186_u8,16_u8,30_u8,184_u8,59_u8,120_u8,128_u8];
_7 = [90_u8,44_u8,83_u8,36_u8,175_u8,95_u8,202_u8];
_3 = _4;
_17 = _13;
_3 = [22270_u16,44930_u16,27383_u16,63882_u16,14392_u16,19723_u16,47470_u16,56272_u16];
RET = [_13,_17];
_18.2 = _17;
_7 = [238_u8,132_u8,70_u8,64_u8,77_u8,77_u8,57_u8];
_7 = [86_u8,170_u8,33_u8,241_u8,143_u8,151_u8,21_u8];
_18.0 = _17;
_20 = &_9;
_7 = [117_u8,255_u8,25_u8,49_u8,120_u8,127_u8,239_u8];
_11 = !2534894662_u32;
_21 = 9995624547773319819_u64 >> _9;
Goto(bb6)
}
bb6 = {
RET = [_17,_18.0];
_2 = _9;
_5 = [6883_u16,7563_u16,10079_u16,4176_u16,8561_u16,47664_u16,44304_u16,31740_u16];
_20 = &_12;
_20 = &_9;
RET = [_18.0,_17];
_18 = (_13, 1784_i16, _17);
_20 = &_2;
_18.1 = 21204_i16;
_3 = [23399_u16,8574_u16,54700_u16,33462_u16,10009_u16,59472_u16,4602_u16,22907_u16];
_18.0 = _13;
_6 = [190_u8,51_u8,25_u8,202_u8,183_u8,37_u8,190_u8];
_8 = _10;
match _18.1 {
0 => bb5,
1 => bb2,
2 => bb7,
3 => bb8,
21204 => bb10,
_ => bb9
}
}
bb7 = {
_12 = _13 as isize;
_2 = -_9;
_7 = [186_u8,16_u8,30_u8,184_u8,59_u8,120_u8,128_u8];
_7 = [90_u8,44_u8,83_u8,36_u8,175_u8,95_u8,202_u8];
_3 = _4;
_17 = _13;
_3 = [22270_u16,44930_u16,27383_u16,63882_u16,14392_u16,19723_u16,47470_u16,56272_u16];
RET = [_13,_17];
_18.2 = _17;
_7 = [238_u8,132_u8,70_u8,64_u8,77_u8,77_u8,57_u8];
_7 = [86_u8,170_u8,33_u8,241_u8,143_u8,151_u8,21_u8];
_18.0 = _17;
_20 = &_9;
_7 = [117_u8,255_u8,25_u8,49_u8,120_u8,127_u8,239_u8];
_11 = !2534894662_u32;
_21 = 9995624547773319819_u64 >> _9;
Goto(bb6)
}
bb8 = {
RET = ['\u{5078a}','\u{38f45}'];
_6 = [51_u8,149_u8,27_u8,106_u8,11_u8,20_u8,175_u8];
RET = ['\u{bc0df}','\u{bc0c7}'];
_4 = [877_u16,13539_u16,60251_u16,33308_u16,52116_u16,26002_u16,47062_u16,21624_u16];
_13 = '\u{ad5e4}';
Goto(bb2)
}
bb9 = {
_4 = _5;
RET = [_13,_13];
_8 = [28134_u16,43104_u16,9992_u16,46344_u16,5158_u16,59345_u16,2702_u16,32831_u16];
_5 = [26326_u16,32852_u16,58345_u16,21385_u16,51683_u16,13934_u16,44528_u16,55042_u16];
_7 = [24_u8,130_u8,235_u8,108_u8,42_u8,243_u8,60_u8];
_3 = [36754_u16,27619_u16,57462_u16,29151_u16,14406_u16,19528_u16,10347_u16,35712_u16];
_6 = [111_u8,126_u8,209_u8,234_u8,27_u8,53_u8,252_u8];
_3 = _10;
_1 = true;
_9 = _2;
_2 = -_9;
_11 = 1064695947_u32;
_12 = _13 as isize;
_6 = [6_u8,150_u8,66_u8,60_u8,26_u8,25_u8,161_u8];
match _11 {
1064695947 => bb5,
_ => bb4
}
}
bb10 = {
_18 = (_13, (-26342_i16), _13);
_11 = _1 as u32;
_1 = !false;
_18 = (_13, 32712_i16, _13);
_7 = [92_u8,41_u8,157_u8,35_u8,197_u8,183_u8,58_u8];
RET = [_17,_13];
_21 = !13144483033390937559_u64;
RET = [_17,_18.2];
_20 = &(*_20);
_18.1 = (-9653_i16) << (*_20);
_26 = !_1;
_8 = [24623_u16,33535_u16,60047_u16,14232_u16,32037_u16,51212_u16,24282_u16,28182_u16];
_21 = 830807680_i32 as u64;
RET = [_13,_18.0];
_4 = [64040_u16,37144_u16,44248_u16,5794_u16,62991_u16,14790_u16,2183_u16,2186_u16];
_17 = _18.0;
_9 = (*_20);
_10 = [6378_u16,22663_u16,59971_u16,51189_u16,52029_u16,64357_u16,62252_u16,51665_u16];
_20 = &_27;
_21 = 9220337968973461019_u64 & 17996091325832436885_u64;
_28 = 194_u8 as isize;
_18 = (_17, (-24898_i16), _13);
match _18.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
340282366920938463463374607431768186558 => bb16,
_ => bb15
}
}
bb11 = {
_4 = _5;
RET = [_13,_13];
_8 = [28134_u16,43104_u16,9992_u16,46344_u16,5158_u16,59345_u16,2702_u16,32831_u16];
_5 = [26326_u16,32852_u16,58345_u16,21385_u16,51683_u16,13934_u16,44528_u16,55042_u16];
_7 = [24_u8,130_u8,235_u8,108_u8,42_u8,243_u8,60_u8];
_3 = [36754_u16,27619_u16,57462_u16,29151_u16,14406_u16,19528_u16,10347_u16,35712_u16];
_6 = [111_u8,126_u8,209_u8,234_u8,27_u8,53_u8,252_u8];
_3 = _10;
_1 = true;
_9 = _2;
_2 = -_9;
_11 = 1064695947_u32;
_12 = _13 as isize;
_6 = [6_u8,150_u8,66_u8,60_u8,26_u8,25_u8,161_u8];
match _11 {
1064695947 => bb5,
_ => bb4
}
}
bb12 = {
RET = ['\u{5078a}','\u{38f45}'];
_6 = [51_u8,149_u8,27_u8,106_u8,11_u8,20_u8,175_u8];
RET = ['\u{bc0df}','\u{bc0c7}'];
_4 = [877_u16,13539_u16,60251_u16,33308_u16,52116_u16,26002_u16,47062_u16,21624_u16];
_13 = '\u{ad5e4}';
Goto(bb2)
}
bb13 = {
_6 = [136_u8,181_u8,98_u8,174_u8,181_u8,94_u8,50_u8];
_7 = [140_u8,45_u8,210_u8,133_u8,202_u8,58_u8,235_u8];
_13 = '\u{aac98}';
_12 = !_9;
Goto(bb3)
}
bb14 = {
RET = [_17,_18.0];
_2 = _9;
_5 = [6883_u16,7563_u16,10079_u16,4176_u16,8561_u16,47664_u16,44304_u16,31740_u16];
_20 = &_12;
_20 = &_9;
RET = [_18.0,_17];
_18 = (_13, 1784_i16, _17);
_20 = &_2;
_18.1 = 21204_i16;
_3 = [23399_u16,8574_u16,54700_u16,33462_u16,10009_u16,59472_u16,4602_u16,22907_u16];
_18.0 = _13;
_6 = [190_u8,51_u8,25_u8,202_u8,183_u8,37_u8,190_u8];
_8 = _10;
match _18.1 {
0 => bb5,
1 => bb2,
2 => bb7,
3 => bb8,
21204 => bb10,
_ => bb9
}
}
bb15 = {
_12 = _13 as isize;
_2 = -_9;
_7 = [186_u8,16_u8,30_u8,184_u8,59_u8,120_u8,128_u8];
_7 = [90_u8,44_u8,83_u8,36_u8,175_u8,95_u8,202_u8];
_3 = _4;
_17 = _13;
_3 = [22270_u16,44930_u16,27383_u16,63882_u16,14392_u16,19723_u16,47470_u16,56272_u16];
RET = [_13,_17];
_18.2 = _17;
_7 = [238_u8,132_u8,70_u8,64_u8,77_u8,77_u8,57_u8];
_7 = [86_u8,170_u8,33_u8,241_u8,143_u8,151_u8,21_u8];
_18.0 = _17;
_20 = &_9;
_7 = [117_u8,255_u8,25_u8,49_u8,120_u8,127_u8,239_u8];
_11 = !2534894662_u32;
_21 = 9995624547773319819_u64 >> _9;
Goto(bb6)
}
bb16 = {
_31.0 = _17;
_8 = [11826_u16,5126_u16,40772_u16,39975_u16,47292_u16,47997_u16,16336_u16,23245_u16];
_31.2 = _18.2;
_11 = 2815265062_u32 * 153937531_u32;
_22 = _1 & _1;
_1 = _2 <= _28;
_23 = _11 > _11;
_21 = 5121210651096313081_u64;
_24 = 807569315_i32 as u128;
_10 = [34700_u16,46011_u16,33456_u16,25807_u16,17752_u16,46244_u16,27170_u16,41872_u16];
_18.2 = _17;
_18.1 = (-27711_i16);
_11 = !3046817361_u32;
_20 = &_28;
_27 = 240_u8 as isize;
_18 = (_17, (-26755_i16), _31.0);
_31.1 = _18.1 + _18.1;
_10 = _8;
_3 = [16395_u16,64482_u16,835_u16,14039_u16,21475_u16,35419_u16,1038_u16,37519_u16];
_20 = &_9;
_28 = _9 & (*_20);
_18.0 = _18.2;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(17_usize, 9_usize, Move(_9), 3_usize, Move(_3), 18_usize, Move(_18), 31_usize, Move(_31)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(17_usize, 7_usize, Move(_7), 28_usize, Move(_28), 10_usize, Move(_10), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(17_usize, 17_usize, Move(_17), 26_usize, Move(_26), 2_usize, Move(_2), 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18() -> *mut f32 {
mir! {
type RET = *mut f32;
let _1: i128;
let _2: &'static *const i16;
let _3: *mut u16;
let _4: &'static (*mut usize, i64);
let _5: f32;
let _6: isize;
let _7: u16;
let _8: &'static f64;
let _9: &'static isize;
let _10: i64;
let _11: &'static u32;
let _12: (&'static isize, Adt24, i16);
let _13: [u8; 7];
let _14: u16;
let _15: &'static u16;
let _16: u64;
let _17: (&'static *const i16, *const (&'static isize, Adt24, i16), &'static *const i16, *mut u32);
let _18: isize;
let _19: isize;
let _20: f32;
let _21: ();
let _22: ();
{
_1 = 8099118386323023992_usize as i128;
_1 = (-87885906079229278719098765069783137029_i128) ^ (-3005755526096838195436721749619953858_i128);
_1 = !105647952079752970884346290735798931058_i128;
_1 = (-441093742_i32) as i128;
_1 = 92427035905080040280396940156362098129_i128 ^ 163486239574124806417550419075692418540_i128;
_1 = 76221258_i32 as i128;
_1 = (-5242815201295979894564519051054152660_i128) - 30362365614547333160331538132211198193_i128;
Goto(bb1)
}
bb1 = {
_1 = 30462932079163766839344377911355189137_i128;
_1 = (-24920496381785240149520162350378089112_i128);
_5 = 206_u8 as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 160639644171521185955550875694123714430_u128 as f32;
_6 = _1 as isize;
(*RET) = (-7310871051583484456_i64) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 219814602838086529705097481831485653012_u128 as f32;
_5 = 1642738308_i32 as f32;
RET = core::ptr::addr_of_mut!(_5);
_5 = (-10_i8) as f32;
(*RET) = 6_usize as f32;
(*RET) = 3479514507588791910_u64 as f32;
(*RET) = (-39431524_i32) as f32;
_6 = (-9223372036854775808_isize);
_3 = core::ptr::addr_of_mut!(_7);
Goto(bb2)
}
bb2 = {
(*RET) = (-281719149_i32) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*_3) = 13863_u16;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = _1 as f32;
(*RET) = _6 as f32;
_1 = 120413383_i32 as i128;
(*RET) = (-716506583_i32) as f32;
_10 = (-7240920635446965660_i64);
_10 = 2034489100076646013_i64;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
2034489100076646013 => bb6,
_ => bb5
}
}
bb3 = {
_1 = 30462932079163766839344377911355189137_i128;
_1 = (-24920496381785240149520162350378089112_i128);
_5 = 206_u8 as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 160639644171521185955550875694123714430_u128 as f32;
_6 = _1 as isize;
(*RET) = (-7310871051583484456_i64) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 219814602838086529705097481831485653012_u128 as f32;
_5 = 1642738308_i32 as f32;
RET = core::ptr::addr_of_mut!(_5);
_5 = (-10_i8) as f32;
(*RET) = 6_usize as f32;
(*RET) = 3479514507588791910_u64 as f32;
(*RET) = (-39431524_i32) as f32;
_6 = (-9223372036854775808_isize);
_3 = core::ptr::addr_of_mut!(_7);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
(*_3) = 47736_u16;
(*RET) = _1 as f32;
(*RET) = _10 as f32;
_6 = (-9223372036854775808_isize);
_9 = &_6;
_7 = 18861_u16 | 33718_u16;
match (*_9) {
0 => bb7,
1 => bb8,
2 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb7 = {
_1 = 30462932079163766839344377911355189137_i128;
_1 = (-24920496381785240149520162350378089112_i128);
_5 = 206_u8 as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 160639644171521185955550875694123714430_u128 as f32;
_6 = _1 as isize;
(*RET) = (-7310871051583484456_i64) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 219814602838086529705097481831485653012_u128 as f32;
_5 = 1642738308_i32 as f32;
RET = core::ptr::addr_of_mut!(_5);
_5 = (-10_i8) as f32;
(*RET) = 6_usize as f32;
(*RET) = 3479514507588791910_u64 as f32;
(*RET) = (-39431524_i32) as f32;
_6 = (-9223372036854775808_isize);
_3 = core::ptr::addr_of_mut!(_7);
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_1 = 30462932079163766839344377911355189137_i128;
_1 = (-24920496381785240149520162350378089112_i128);
_5 = 206_u8 as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 160639644171521185955550875694123714430_u128 as f32;
_6 = _1 as isize;
(*RET) = (-7310871051583484456_i64) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*RET) = 219814602838086529705097481831485653012_u128 as f32;
_5 = 1642738308_i32 as f32;
RET = core::ptr::addr_of_mut!(_5);
_5 = (-10_i8) as f32;
(*RET) = 6_usize as f32;
(*RET) = 3479514507588791910_u64 as f32;
(*RET) = (-39431524_i32) as f32;
_6 = (-9223372036854775808_isize);
_3 = core::ptr::addr_of_mut!(_7);
Goto(bb2)
}
bb10 = {
(*RET) = (-281719149_i32) as f32;
RET = core::ptr::addr_of_mut!(_5);
(*_3) = 13863_u16;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = _1 as f32;
(*RET) = _6 as f32;
_1 = 120413383_i32 as i128;
(*RET) = (-716506583_i32) as f32;
_10 = (-7240920635446965660_i64);
_10 = 2034489100076646013_i64;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
2034489100076646013 => bb6,
_ => bb5
}
}
bb11 = {
(*RET) = _10 as f32;
_7 = 60_u8 as u16;
_6 = !(-18_isize);
_9 = &_6;
RET = core::ptr::addr_of_mut!((*RET));
_9 = &_6;
(*RET) = (-13525_i16) as f32;
_7 = !18910_u16;
(*RET) = (-99_i8) as f32;
(*RET) = 2909952847_u32 as f32;
RET = core::ptr::addr_of_mut!((*RET));
_12.2 = _1 as i16;
RET = core::ptr::addr_of_mut!((*RET));
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
2034489100076646013 => bb12,
_ => bb7
}
}
bb12 = {
_9 = &(*_9);
_13 = [232_u8,176_u8,164_u8,96_u8,141_u8,60_u8,218_u8];
_1 = 148659204911813047383138438464065205420_i128;
_10 = (-3582002235459201217_i64) >> _6;
(*RET) = _12.2 as f32;
_6 = (-9223372036854775808_isize);
RET = core::ptr::addr_of_mut!(_5);
_12.0 = &_6;
_15 = &(*_3);
_16 = _10 as u64;
_14 = 3397009445_u32 as u16;
match _6 {
0 => bb8,
1 => bb2,
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_15 = &_14;
_12.0 = &_6;
_9 = &_19;
(*RET) = 49389506137415946651995340110506275842_u128 as f32;
RET = core::ptr::addr_of_mut!(_5);
_17.1 = core::ptr::addr_of!(_12);
_19 = _6;
_6 = _19;
_3 = core::ptr::addr_of_mut!((*_3));
_13 = [224_u8,228_u8,110_u8,242_u8,137_u8,49_u8,161_u8];
(*RET) = _10 as f32;
_3 = core::ptr::addr_of_mut!((*_15));
(*RET) = _16 as f32;
_12.0 = &_19;
_12.2 = 24500_i16 - 5716_i16;
_9 = Move(_12.0);
_9 = &_19;
_15 = &(*_15);
(*RET) = _10 as f32;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(18_usize, 13_usize, Move(_13), 16_usize, Move(_16), 19_usize, Move(_19), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> isize {
mir! {
type RET = isize;
let _11: Adt73;
let _12: &'static *const i16;
let _13: ();
let _14: ();
{
_2 = _9;
_8 = _6;
_10 = _5;
_8 = 14316637035747037500_u64 as isize;
Call(_3 = core::intrinsics::transmute(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = -_5;
_10 = _5;
_10 = -_3;
_6 = _7;
_9 = _2;
_2 = _10;
RET = false as isize;
_3 = _1 | _1;
_5 = 18024472558982657727_u64 as isize;
_7 = _4 >> _4;
_7 = _4 + _10;
RET = _3;
RET = _7;
_2 = !_6;
_8 = _9 * _2;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(19_usize, 9_usize, Move(_9), 4_usize, Move(_4), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(19_usize, 7_usize, Move(_7), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(51_u8), std::hint::black_box('\u{e21a2}'), std::hint::black_box((-75_isize)), std::hint::black_box(125_i8), std::hint::black_box(17068008320235332405_u64), std::hint::black_box(10811680090734097014_usize), std::hint::black_box(1505119301_u32), std::hint::black_box(76501786524373153461672767549940293795_i128));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: f64,
fld1: i64,
fld2: isize,
fld3: u32,
fld4: (f32,),
fld5: *mut u32,

},
Variant1{
fld0: f32,
fld1: char,
fld2: *mut usize,
fld3: i8,
fld4: f64,
fld5: usize,
fld6: u16,
fld7: i128,

}}
impl PrintFDebug for Adt25{
	unsafe fn printf_debug(&self){unsafe{printf("Adt25::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt25 {
Variant0{
fld0: (u32, u16, i128),

},
Variant1{
fld0: *mut usize,
fld1: Adt24,
fld2: u64,
fld3: i8,
fld4: usize,
fld5: i32,
fld6: (f32,),

},
Variant2{
fld0: *mut usize,
fld1: u64,
fld2: (u32, u16, i128),
fld3: i8,
fld4: i16,
fld5: u128,
fld6: u16,
fld7: i128,

},
Variant3{
fld0: u64,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){unsafe{printf("Adt33::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt33 {
Variant0{
fld0: *mut u16,
fld1: i32,
fld2: f64,
fld3: u8,

},
Variant1{
fld0: bool,
fld1: i128,
fld2: *mut u32,
fld3: *mut usize,
fld4: u128,

},
Variant2{
fld0: *mut usize,
fld1: u64,
fld2: u8,
fld3: i8,
fld4: i64,

},
Variant3{
fld0: [u128; 3],
fld1: char,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
fld0: *mut u8,

},
Variant1{
fld0: isize,
fld1: *mut usize,

},
Variant2{
fld0: u16,
fld1: [i16; 7],
fld2: u32,
fld3: u8,
fld4: i16,
fld5: *mut u16,
fld6: [u8; 5],
fld7: i128,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *const usize,
fld1: u64,
fld2: Adt25,

},
Variant1{
fld0: (*mut usize, i64),
fld1: i8,
fld2: u128,

},
Variant2{
fld0: (char, i16, char),
fld1: i128,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [u32; 8],
fld1: [u8; 5],
fld2: isize,
fld3: [i16; 7],

},
Variant1{
fld0: bool,
fld1: [i32; 8],
fld2: u16,
fld3: i8,
fld4: i16,
fld5: f32,

},
Variant2{
fld0: *const i128,
fld1: [i16; 7],
fld2: (*const i16,),
fld3: u32,
fld4: Adt24,
fld5: u16,
fld6: f32,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: u128,

},
Variant1{
fld0: *const usize,
fld1: char,
fld2: usize,

},
Variant2{
fld0: usize,
fld1: Adt33,
fld2: *const usize,
fld3: *const *mut f32,
fld4: Adt53,
fld5: [u32; 1],
fld6: *mut usize,

},
Variant3{
fld0: Adt33,
fld1: (u16,),

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
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: i64,
fld1: *const *mut f32,
fld2: isize,
fld3: [u32; 8],
fld4: (i32,),
fld5: u16,

},
Variant1{
fld0: *const i128,
fld1: *const *mut f32,

}}

