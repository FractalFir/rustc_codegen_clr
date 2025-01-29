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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> (i16, *mut u128, u8, usize, char) {
mir! {
type RET = (i16, *mut u128, u8, usize, char);
let _15: Adt59;
let _16: (bool, *const i128, u128, *const i128, u8);
let _17: char;
let _18: [bool; 5];
let _19: *mut i8;
let _20: (i16, *mut u128, u8, usize, char);
let _21: usize;
let _22: char;
let _23: char;
let _24: Adt55;
let _25: isize;
let _26: i16;
let _27: *mut u128;
let _28: isize;
let _29: bool;
let _30: i128;
let _31: (i8, [i64; 7]);
let _32: (i8, [i64; 7]);
let _33: ();
let _34: ();
{
_4 = 1285192671_u32 as i8;
_5 = 7912_i16 << _4;
_7 = 8518756906016379320_i64;
RET.4 = '\u{101dc7}';
RET.1 = core::ptr::addr_of_mut!(_14);
_5 = false as i16;
Call(_3 = fn1(RET.1, RET.1, _7, _4, RET.1, RET.1, RET.1, RET.1, _7, _7, RET.1, RET.1, _5, RET.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.2 = 180_u8;
_13 = 1306594705698068133_u64;
_9 = 0_usize + 7_usize;
RET.2 = 25_u8 >> _14;
_2 = RET.4;
_12 = 383433488_u32;
_11 = 15370_u16 & 7770_u16;
_16.0 = false;
RET.0 = _5;
_16.0 = true;
_1 = _16.0 < _16.0;
_5 = RET.0;
_16.3 = core::ptr::addr_of!(_8);
_18 = [_1,_16.0,_1,_1,_1];
_16.1 = core::ptr::addr_of!(_8);
Goto(bb2)
}
bb2 = {
_8 = -(-152302221518135739822347275757033124200_i128);
_5 = _8 as i16;
_16.4 = RET.2;
_16.0 = _1;
_16.2 = _14;
RET.2 = _1 as u8;
_6 = !(-942504653_i32);
RET.1 = core::ptr::addr_of_mut!(_14);
_3 = 9223372036854775807_isize;
RET.3 = _9 - _9;
RET.0 = -_5;
_20.3 = !RET.3;
RET.4 = _2;
_20.1 = RET.1;
_17 = _2;
RET = (_5, _20.1, _16.4, _20.3, _17);
RET.4 = _17;
_10 = !RET.2;
_23 = RET.4;
_6 = (-361562033_i32);
match _3 {
0 => bb3,
1 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb3 = {
RET.2 = 180_u8;
_13 = 1306594705698068133_u64;
_9 = 0_usize + 7_usize;
RET.2 = 25_u8 >> _14;
_2 = RET.4;
_12 = 383433488_u32;
_11 = 15370_u16 & 7770_u16;
_16.0 = false;
RET.0 = _5;
_16.0 = true;
_1 = _16.0 < _16.0;
_5 = RET.0;
_16.3 = core::ptr::addr_of!(_8);
_18 = [_1,_16.0,_1,_1,_1];
_16.1 = core::ptr::addr_of!(_8);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_6 = 2130450078_i32;
Goto(bb7)
}
bb7 = {
RET.0 = !_5;
_12 = 1374987640_u32 << _10;
_6 = _16.0 as i32;
_5 = -RET.0;
_1 = RET.3 != _9;
_20 = (RET.0, RET.1, RET.2, RET.3, _17);
_9 = _13 as usize;
_11 = 26892_u16 & 49566_u16;
_18 = [_1,_1,_1,_1,_1];
_12 = !1381247547_u32;
_21 = _9 ^ _9;
RET.4 = _23;
RET.2 = _10;
_16.3 = _16.1;
_20.0 = -RET.0;
match _14 {
249216564053335560644431771772474141506 => bb8,
_ => bb3
}
}
bb8 = {
RET.4 = _23;
_14 = !_16.2;
_25 = _10 as isize;
_16.2 = _14;
Goto(bb9)
}
bb9 = {
_26 = _5 & _20.0;
_17 = _23;
_16.3 = _16.1;
_29 = _20.2 < _16.4;
_16.1 = core::ptr::addr_of!(_8);
Goto(bb10)
}
bb10 = {
_2 = RET.4;
_20.1 = core::ptr::addr_of_mut!(_14);
_17 = RET.4;
_17 = _2;
_20.3 = RET.3 ^ _9;
_19 = core::ptr::addr_of_mut!(_4);
_16.3 = _16.1;
_3 = _25;
RET = (_20.0, _20.1, _10, _20.3, _20.4);
_10 = _16.4 ^ _16.4;
_18 = [_1,_29,_1,_1,_1];
_23 = RET.4;
_17 = _20.4;
_16.1 = _16.3;
RET.4 = _20.4;
_20.0 = !RET.0;
_20.2 = !_16.4;
_19 = core::ptr::addr_of_mut!(_4);
Goto(bb11)
}
bb11 = {
_16.3 = _16.1;
_30 = !_8;
(*_19) = (-124_i8) - (-70_i8);
(*_19) = (-98_i8);
_7 = (-8894439732262913222_i64) >> _21;
(*_19) = (-5_i8);
_20.2 = _10 - _16.4;
_31.0 = _4;
_3 = _12 as isize;
(*_19) = _13 as i8;
match _31.0 {
0 => bb8,
1 => bb5,
2 => bb6,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211451 => bb16,
_ => bb15
}
}
bb12 = {
_2 = RET.4;
_20.1 = core::ptr::addr_of_mut!(_14);
_17 = RET.4;
_17 = _2;
_20.3 = RET.3 ^ _9;
_19 = core::ptr::addr_of_mut!(_4);
_16.3 = _16.1;
_3 = _25;
RET = (_20.0, _20.1, _10, _20.3, _20.4);
_10 = _16.4 ^ _16.4;
_18 = [_1,_29,_1,_1,_1];
_23 = RET.4;
_17 = _20.4;
_16.1 = _16.3;
RET.4 = _20.4;
_20.0 = !RET.0;
_20.2 = !_16.4;
_19 = core::ptr::addr_of_mut!(_4);
Goto(bb11)
}
bb13 = {
RET.2 = 180_u8;
_13 = 1306594705698068133_u64;
_9 = 0_usize + 7_usize;
RET.2 = 25_u8 >> _14;
_2 = RET.4;
_12 = 383433488_u32;
_11 = 15370_u16 & 7770_u16;
_16.0 = false;
RET.0 = _5;
_16.0 = true;
_1 = _16.0 < _16.0;
_5 = RET.0;
_16.3 = core::ptr::addr_of!(_8);
_18 = [_1,_16.0,_1,_1,_1];
_16.1 = core::ptr::addr_of!(_8);
Goto(bb2)
}
bb14 = {
_8 = -(-152302221518135739822347275757033124200_i128);
_5 = _8 as i16;
_16.4 = RET.2;
_16.0 = _1;
_16.2 = _14;
RET.2 = _1 as u8;
_6 = !(-942504653_i32);
RET.1 = core::ptr::addr_of_mut!(_14);
_3 = 9223372036854775807_isize;
RET.3 = _9 - _9;
RET.0 = -_5;
_20.3 = !RET.3;
RET.4 = _2;
_20.1 = RET.1;
_17 = _2;
RET = (_5, _20.1, _16.4, _20.3, _17);
RET.4 = _17;
_10 = !RET.2;
_23 = RET.4;
_6 = (-361562033_i32);
match _3 {
0 => bb3,
1 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb15 = {
RET.2 = 180_u8;
_13 = 1306594705698068133_u64;
_9 = 0_usize + 7_usize;
RET.2 = 25_u8 >> _14;
_2 = RET.4;
_12 = 383433488_u32;
_11 = 15370_u16 & 7770_u16;
_16.0 = false;
RET.0 = _5;
_16.0 = true;
_1 = _16.0 < _16.0;
_5 = RET.0;
_16.3 = core::ptr::addr_of!(_8);
_18 = [_1,_16.0,_1,_1,_1];
_16.1 = core::ptr::addr_of!(_8);
Goto(bb2)
}
bb16 = {
_31.1 = [_7,_7,_7,_7,_7,_7,_7];
_7 = !(-183199718930074487_i64);
_28 = !_3;
RET.0 = _5;
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(0_usize, 28_usize, Move(_28), 21_usize, Move(_21), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(0_usize, 29_usize, Move(_29), 9_usize, Move(_9), 6_usize, Move(_6), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(0_usize, 30_usize, Move(_30), 7_usize, Move(_7), 1_usize, Move(_1), 25_usize, Move(_25)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: *mut u128,mut _2: *mut u128,mut _3: i64,mut _4: i8,mut _5: *mut u128,mut _6: *mut u128,mut _7: *mut u128,mut _8: *mut u128,mut _9: i64,mut _10: i64,mut _11: *mut u128,mut _12: *mut u128,mut _13: i16,mut _14: *mut u128) -> isize {
mir! {
type RET = isize;
let _15: Adt47;
let _16: u8;
let _17: Adt48;
let _18: Adt55;
let _19: Adt44;
let _20: Adt51;
let _21: bool;
let _22: f32;
let _23: (f64, i8, f32);
let _24: f32;
let _25: [bool; 5];
let _26: u128;
let _27: f64;
let _28: *const *const i128;
let _29: i8;
let _30: [i8; 5];
let _31: u64;
let _32: char;
let _33: [i8; 5];
let _34: [i8; 5];
let _35: i128;
let _36: u32;
let _37: ();
let _38: ();
{
_2 = core::ptr::addr_of_mut!((*_5));
(*_2) = 254721600250608292855150790410794200088_u128 - 113082335742108535230351543110525042694_u128;
(*_6) = 56216055320449003943010937272456666585_u128;
(*_11) = _13 as u128;
_5 = core::ptr::addr_of_mut!((*_11));
_7 = _2;
RET = !9223372036854775807_isize;
_7 = _12;
_11 = _6;
_4 = (*_5) as i8;
_12 = core::ptr::addr_of_mut!((*_8));
(*_2) = 118812767869099392357661424836127633467_u128 >> RET;
_12 = core::ptr::addr_of_mut!((*_14));
_10 = (*_14) as i64;
Goto(bb1)
}
bb1 = {
(*_6) = 225786987765865373024092668341124753863_u128 ^ 167736597009039762534686559863059859476_u128;
(*_14) = 211306294922884295493555719254071607367_u128;
(*_6) = 311265031017715791463604772016734241205_u128;
_6 = core::ptr::addr_of_mut!((*_14));
(*_11) = !3623198298174875606885820565190708400_u128;
Goto(bb2)
}
bb2 = {
_16 = 113_u8 ^ 225_u8;
RET = _10 as isize;
_7 = _14;
(*_12) = !204001379256035702909841842254152449654_u128;
(*_14) = !158048205032418953288534470459828223800_u128;
_13 = 2_usize as i16;
(*_7) = !212550722801513713138296829702529074070_u128;
(*_2) = 327859940017086215641314090786783696054_u128;
(*_5) = 16997921774147732394_u64 as u128;
_9 = _3 >> _16;
(*_6) = 243801027446733046930318026950156205841_u128;
(*_14) = 14426_u16 as u128;
_6 = core::ptr::addr_of_mut!((*_11));
_8 = _1;
(*_11) = !300333905168598446884660515232091326268_u128;
_5 = _2;
_21 = !true;
match _3 {
0 => bb3,
8518756906016379320 => bb5,
_ => bb4
}
}
bb3 = {
(*_6) = 225786987765865373024092668341124753863_u128 ^ 167736597009039762534686559863059859476_u128;
(*_14) = 211306294922884295493555719254071607367_u128;
(*_6) = 311265031017715791463604772016734241205_u128;
_6 = core::ptr::addr_of_mut!((*_14));
(*_11) = !3623198298174875606885820565190708400_u128;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
(*_2) = _4 as u128;
_16 = !242_u8;
(*_8) = 180434925990753305080240816332128597357_u128 << _16;
(*_5) = 295722529542091976859336826108721574_u128;
(*_12) = _10 as u128;
_16 = !0_u8;
_2 = _11;
(*_5) = 107582439738974962896073221461482331893_u128;
_1 = core::ptr::addr_of_mut!((*_12));
(*_11) = 119567089825862479650144933508400247473_u128 * 83029336245014966726576428916058683817_u128;
(*_1) = 201518750675611398028458842908276623019_u128;
_4 = RET as i8;
_5 = _6;
_6 = core::ptr::addr_of_mut!((*_2));
(*_14) = 272807774068625226138716427896754890432_u128 - 247685899669222155865506705045452322863_u128;
(*_11) = 169798086922140568768465295250530950152_u128 >> _9;
_5 = core::ptr::addr_of_mut!((*_11));
_2 = _7;
_22 = 1513679961_i32 as f32;
_23.2 = _22 + _22;
_22 = _10 as f32;
(*_11) = 40469550971640144978465109996704173805_u128;
_1 = _8;
(*_11) = 258286613845995243032489285907348843587_u128;
_11 = _6;
_21 = false & true;
(*_6) = 138405146012442288710906095575952962584_i128 as u128;
RET = (-9223372036854775808_isize) ^ (-52_isize);
_10 = _22 as i64;
match _3 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
8518756906016379320 => bb11,
_ => bb10
}
}
bb6 = {
Return()
}
bb7 = {
(*_6) = 225786987765865373024092668341124753863_u128 ^ 167736597009039762534686559863059859476_u128;
(*_14) = 211306294922884295493555719254071607367_u128;
(*_6) = 311265031017715791463604772016734241205_u128;
_6 = core::ptr::addr_of_mut!((*_14));
(*_11) = !3623198298174875606885820565190708400_u128;
Goto(bb2)
}
bb8 = {
_16 = 113_u8 ^ 225_u8;
RET = _10 as isize;
_7 = _14;
(*_12) = !204001379256035702909841842254152449654_u128;
(*_14) = !158048205032418953288534470459828223800_u128;
_13 = 2_usize as i16;
(*_7) = !212550722801513713138296829702529074070_u128;
(*_2) = 327859940017086215641314090786783696054_u128;
(*_5) = 16997921774147732394_u64 as u128;
_9 = _3 >> _16;
(*_6) = 243801027446733046930318026950156205841_u128;
(*_14) = 14426_u16 as u128;
_6 = core::ptr::addr_of_mut!((*_11));
_8 = _1;
(*_11) = !300333905168598446884660515232091326268_u128;
_5 = _2;
_21 = !true;
match _3 {
0 => bb3,
8518756906016379320 => bb5,
_ => bb4
}
}
bb9 = {
(*_6) = 225786987765865373024092668341124753863_u128 ^ 167736597009039762534686559863059859476_u128;
(*_14) = 211306294922884295493555719254071607367_u128;
(*_6) = 311265031017715791463604772016734241205_u128;
_6 = core::ptr::addr_of_mut!((*_14));
(*_11) = !3623198298174875606885820565190708400_u128;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_25 = [_21,_21,_21,_21,_21];
(*_1) = !87233992063259179564679125970008760497_u128;
(*_6) = 132660059910747612798937986858998246693_u128;
_10 = !_3;
(*_7) = 47321103485843523187211749727679843960_u128;
RET = (-114_isize) - 81_isize;
_23.1 = RET as i8;
_16 = 49_u8 * 94_u8;
(*_8) = _16 as u128;
_3 = -_9;
_27 = RET as f64;
(*_7) = 147659709148926409348264239610588841061_u128 >> _4;
RET = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
(*_6) = 248711362526966460805951743310326161968_u128 | 216685480881044309677092962994558828373_u128;
_25 = [_21,_21,_21,_21,_21];
_2 = core::ptr::addr_of_mut!((*_1));
_22 = _23.2 + _23.2;
_15 = Adt47::Variant0 { fld0: _22,fld1: _13,fld2: _10 };
Call(_23 = fn2(_2, _15, (*_2), _1, _14, _25, _8, (*_8), Field::<f32>(Variant(_15, 0), 0)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = _14;
_14 = _8;
_24 = 1526222607_i32 as f32;
_21 = (*_1) < (*_12);
_9 = -_3;
_1 = core::ptr::addr_of_mut!((*_6));
Goto(bb13)
}
bb13 = {
(*_7) = _16 as u128;
_33 = [_23.1,_23.1,_23.1,_23.1,_23.1];
_2 = _14;
(*_7) = 189018051848826064458812291316829803769_u128 - 260978103605583931542002733149858410776_u128;
_30 = [_23.1,_4,_23.1,_4,_4];
(*_2) = 150937080340988943143206261478712733531_u128 >> Field::<i16>(Variant(_15, 0), 1);
_1 = core::ptr::addr_of_mut!((*_12));
_22 = Field::<f32>(Variant(_15, 0), 0);
_1 = core::ptr::addr_of_mut!((*_11));
place!(Field::<f32>(Variant(_15, 0), 0)) = _23.2 - _23.2;
_3 = Field::<i64>(Variant(_15, 0), 2);
_23.0 = -_27;
_32 = '\u{10fdf2}';
(*_2) = 103066034304722693959154249847737010579_u128;
match (*_5) {
0 => bb1,
103066034304722693959154249847737010579 => bb14,
_ => bb7
}
}
bb14 = {
_25 = [_21,_21,_21,_21,_21];
_27 = _23.0;
_32 = '\u{5d3bb}';
(*_2) = RET as u128;
_4 = -_23.1;
(*_14) = 199874001501661678274453694253757786865_u128;
_11 = core::ptr::addr_of_mut!((*_14));
(*_5) = 249216564053335560644431771772474141506_u128;
_9 = _10;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(1_usize, 16_usize, Move(_16), 3_usize, Move(_3), 9_usize, Move(_9), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(1_usize, 13_usize, Move(_13), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: *mut u128,mut _2: Adt47,mut _3: u128,mut _4: *mut u128,mut _5: *mut u128,mut _6: [bool; 5],mut _7: *mut u128,mut _8: u128,mut _9: f32) -> (f64, i8, f32) {
mir! {
type RET = (f64, i8, f32);
let _10: i128;
let _11: f64;
let _12: bool;
let _13: ((usize, bool, *const i128), i8, (bool, *const i128, u128, *const i128, u8), (usize, bool, *const i128), [u32; 7]);
let _14: i64;
let _15: f32;
let _16: u8;
let _17: u16;
let _18: Adt45;
let _19: bool;
let _20: u8;
let _21: (*mut *const i128,);
let _22: [i8; 5];
let _23: Adt54;
let _24: [i8; 5];
let _25: Adt59;
let _26: Adt53;
let _27: bool;
let _28: Adt50;
let _29: [i8; 5];
let _30: [u32; 7];
let _31: bool;
let _32: [u32; 7];
let _33: isize;
let _34: char;
let _35: [u32; 1];
let _36: Adt49;
let _37: (i8, [i64; 7]);
let _38: i16;
let _39: (i16, *mut u128, u8, usize, char);
let _40: [u64; 2];
let _41: ();
let _42: ();
{
RET.0 = 375553388_i32 as f64;
RET.2 = 4850370030557598076_u64 as f32;
(*_5) = _8 << _3;
RET.1 = (-119419371232681784991635161478946545665_i128) as i8;
RET.2 = Field::<f32>(Variant(_2, 0), 0) * Field::<f32>(Variant(_2, 0), 0);
place!(Field::<f32>(Variant(_2, 0), 0)) = RET.2;
RET.1 = 100_i8 >> (*_5);
_1 = core::ptr::addr_of_mut!((*_5));
place!(Field::<f32>(Variant(_2, 0), 0)) = Field::<i16>(Variant(_2, 0), 1) as f32;
RET.1 = 63_i8 >> (*_4);
place!(Field::<i64>(Variant(_2, 0), 2)) = Field::<i16>(Variant(_2, 0), 1) as i64;
_2 = Adt47::Variant0 { fld0: RET.2,fld1: (-25824_i16),fld2: (-9098231993560452265_i64) };
_1 = _4;
place!(Field::<i16>(Variant(_2, 0), 1)) = 2991_i16;
place!(Field::<f32>(Variant(_2, 0), 0)) = 198_u8 as f32;
(*_5) = _8 + _8;
_6 = [true,false,false,false,false];
_4 = core::ptr::addr_of_mut!(_3);
_3 = Field::<i16>(Variant(_2, 0), 1) as u128;
_10 = (-99921904904578680892002987878980284998_i128);
_13.2.3 = core::ptr::addr_of!(_10);
_13.2.2 = RET.0 as u128;
Goto(bb1)
}
bb1 = {
(*_7) = _8;
RET.0 = (-9223372036854775808_isize) as f64;
_15 = _9;
(*_7) = !(*_4);
_13.1 = RET.1 + RET.1;
_11 = RET.0;
_13.3 = (6_usize, false, _13.2.3);
(*_4) = RET.0 as u128;
_15 = -RET.2;
Goto(bb2)
}
bb2 = {
_14 = !(-50566890134055278_i64);
Call(_17 = fn3(_15, _15, (*_5), _13.3, _9, _9, _3, _13.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = _17 as f64;
_18 = Adt45::Variant0 { fld0: (-1799730048_i32) };
_3 = !_8;
Goto(bb4)
}
bb4 = {
_13.1 = !RET.1;
_9 = -_15;
(*_4) = _8 ^ _8;
_4 = _1;
RET = (_11, _13.1, _15);
place!(Field::<i64>(Variant(_2, 0), 2)) = !_14;
_2 = Adt47::Variant0 { fld0: _9,fld1: (-28924_i16),fld2: _14 };
_13.2.4 = 208_u8 - 236_u8;
_13.4 = [3428526832_u32,1772849440_u32,3329886558_u32,3672265058_u32,1271935414_u32,4263045927_u32,2390216461_u32];
_13.2.0 = !_13.3.1;
_16 = _13.2.4 * _13.2.4;
_11 = RET.0;
_17 = !32191_u16;
_13.2.3 = core::ptr::addr_of!(_10);
_3 = (*_7) ^ (*_1);
(*_4) = !_13.2.2;
_13.2 = (_13.3.1, _13.3.2, (*_4), _13.3.2, _16);
place!(Field::<i64>(Variant(_2, 0), 2)) = _14;
_13.0 = (_13.3.0, _13.3.1, _13.3.2);
_12 = _13.0.1;
_21.0 = core::ptr::addr_of_mut!(_13.2.1);
_13.2.1 = _13.3.2;
_10 = (-128344594481153025834223664613866531200_i128);
Goto(bb5)
}
bb5 = {
_13.3 = _13.0;
Goto(bb6)
}
bb6 = {
_13.2.1 = _13.2.3;
RET.1 = !_13.1;
RET.1 = _13.3.0 as i8;
_13.2.2 = (*_5);
_13.4 = [1970305628_u32,2381341583_u32,1300638507_u32,1199185445_u32,796580442_u32,3178853_u32,2553132031_u32];
_22 = [_13.1,RET.1,_13.1,_13.1,_13.1];
_24 = [RET.1,_13.1,_13.1,_13.1,_13.1];
(*_4) = _8 << _17;
_3 = (*_7) | (*_7);
(*_4) = _3 + _3;
_13.0 = (_13.3.0, _13.2.0, _13.3.2);
RET.2 = -Field::<f32>(Variant(_2, 0), 0);
_14 = !Field::<i64>(Variant(_2, 0), 2);
_13.0.0 = !_13.3.0;
RET = (_11, _13.1, _15);
(*_4) = _8 | _13.2.2;
place!(Field::<i32>(Variant(_18, 0), 0)) = -(-1615067510_i32);
RET.1 = !_13.1;
Goto(bb7)
}
bb7 = {
RET.1 = _13.1 & _13.1;
_13.2.4 = _16;
place!(Field::<i32>(Variant(_18, 0), 0)) = (-794138528_i32);
_22 = [RET.1,RET.1,RET.1,RET.1,RET.1];
_24 = [RET.1,RET.1,_13.1,RET.1,_13.1];
_3 = (*_4) & (*_7);
SetDiscriminant(_18, 1);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)).2 = !(*_1);
place!(Field::<i16>(Variant(_2, 0), 1)) = !9736_i16;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)).3 = _13.2.1;
_19 = _13.3.1 & _13.2.0;
_13.0.2 = core::ptr::addr_of!(_10);
Goto(bb8)
}
bb8 = {
_7 = _1;
RET = (_11, _13.1, _9);
_3 = !(*_7);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)).4 = _13.2.4;
_29 = [RET.1,RET.1,RET.1,_13.1,RET.1];
RET.1 = !_13.1;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)).0 = !_19;
_32 = _13.4;
_6 = [_13.0.1,Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).0,Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).0,_13.0.1,_19];
match _10 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
211937772439785437629150942817901680256 => bb9,
_ => bb6
}
}
bb9 = {
place!(Field::<[u32; 1]>(Variant(_18, 1), 2)) = [795462460_u32];
_13.2 = (_13.0.1, Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).3, (*_1), Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).3, _16);
_13.2.2 = (*_7);
_22 = [RET.1,_13.1,_13.1,_13.1,_13.1];
_22 = [RET.1,RET.1,RET.1,RET.1,_13.1];
place!(Field::<[u32; 1]>(Variant(_18, 1), 2)) = [4096219542_u32];
_30 = _13.4;
(*_1) = Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).2;
_13.2.4 = !_16;
Goto(bb10)
}
bb10 = {
_31 = _19;
SetDiscriminant(_2, 2);
_24 = [RET.1,_13.1,RET.1,_13.1,_13.1];
place!(Field::<(i16, *mut u128, u8, usize, char)>(Variant(_2, 2), 0)).0 = (-600_i16);
_7 = core::ptr::addr_of_mut!((*_7));
_6 = [_13.3.1,_13.0.1,_13.0.1,_19,Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).0];
_13.0.1 = !Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).0;
(*_5) = RET.2 as u128;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)) = _13.2;
place!(Field::<*mut i8>(Variant(_2, 2), 1)) = core::ptr::addr_of_mut!(_13.1);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)).2 = 1687000328_u32 as u128;
Goto(bb11)
}
bb11 = {
_13.0.2 = Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).1;
_7 = core::ptr::addr_of_mut!(_3);
_13.0.1 = _31;
_9 = -_15;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3)).3 = core::ptr::addr_of!(_10);
_9 = _15 - RET.2;
match Field::<(i16, *mut u128, u8, usize, char)>(Variant(_2, 2), 0).0 {
340282366920938463463374607431768210856 => bb13,
_ => bb12
}
}
bb12 = {
_13.2.1 = _13.2.3;
RET.1 = !_13.1;
RET.1 = _13.3.0 as i8;
_13.2.2 = (*_5);
_13.4 = [1970305628_u32,2381341583_u32,1300638507_u32,1199185445_u32,796580442_u32,3178853_u32,2553132031_u32];
_22 = [_13.1,RET.1,_13.1,_13.1,_13.1];
_24 = [RET.1,_13.1,_13.1,_13.1,_13.1];
(*_4) = _8 << _17;
_3 = (*_7) | (*_7);
(*_4) = _3 + _3;
_13.0 = (_13.3.0, _13.2.0, _13.3.2);
RET.2 = -Field::<f32>(Variant(_2, 0), 0);
_14 = !Field::<i64>(Variant(_2, 0), 2);
_13.0.0 = !_13.3.0;
RET = (_11, _13.1, _15);
(*_4) = _8 | _13.2.2;
place!(Field::<i32>(Variant(_18, 0), 0)) = -(-1615067510_i32);
RET.1 = !_13.1;
Goto(bb7)
}
bb13 = {
place!(Field::<(i16, *mut u128, u8, usize, char)>(Variant(_2, 2), 0)).2 = Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_18, 1), 3).4 >> (*_7);
RET.0 = 68794469_i32 as f64;
(*_1) = _3;
_36.fld1 = _21.0;
_19 = _31 ^ _13.0.1;
place!(Field::<i16>(Variant(_2, 2), 2)) = Field::<(i16, *mut u128, u8, usize, char)>(Variant(_2, 2), 0).0 * Field::<(i16, *mut u128, u8, usize, char)>(Variant(_2, 2), 0).0;
(*_5) = !_13.2.2;
_34 = '\u{c830a}';
_16 = Field::<(i16, *mut u128, u8, usize, char)>(Variant(_2, 2), 0).2;
_2 = Adt47::Variant0 { fld0: RET.2,fld1: 7966_i16,fld2: _14 };
_39.4 = _34;
_13.3.0 = !_13.0.0;
Goto(bb14)
}
bb14 = {
(*_4) = !_13.2.2;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(2_usize, 24_usize, Move(_24), 29_usize, Move(_29), 14_usize, Move(_14), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(2_usize, 8_usize, Move(_8), 17_usize, Move(_17), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f32,mut _2: f32,mut _3: u128,mut _4: (usize, bool, *const i128),mut _5: f32,mut _6: f32,mut _7: u128,mut _8: i8) -> u16 {
mir! {
type RET = u16;
let _9: [u32; 7];
let _10: isize;
let _11: [u128; 1];
let _12: (i8, [i64; 7]);
let _13: i128;
let _14: [u32; 1];
let _15: i8;
let _16: [i8; 5];
let _17: [bool; 5];
let _18: isize;
let _19: isize;
let _20: ();
let _21: ();
{
RET = '\u{591ea}' as u16;
_4.1 = _2 < _2;
_4.1 = false;
_5 = -_6;
RET = 39382_u16;
_4.0 = !10199685854530856819_usize;
_5 = 20226_i16 as f32;
_9 = [473584965_u32,3500771705_u32,1364237940_u32,2024084576_u32,621253404_u32,4064033322_u32,1539170609_u32];
_2 = -_1;
_7 = !_3;
_1 = _6 - _2;
_2 = _1;
_6 = 757119906190930628_i64 as f32;
_4.1 = _2 >= _1;
_7 = _3 * _3;
_7 = _4.1 as u128;
match RET {
39382 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_7 = !_3;
_4.0 = 2_usize >> _8;
_2 = 147765998_u32 as f32;
RET = _1 as u16;
_7 = !_3;
RET = 57786_u16 - 4430_u16;
_3 = 813053037_u32 as u128;
_7 = !_3;
RET = 6685_u16 ^ 57187_u16;
_9 = [3842076526_u32,1424689839_u32,1941880120_u32,4043950501_u32,4110246937_u32,1972506099_u32,2158847309_u32];
_4.1 = _8 > _8;
RET = !41935_u16;
_3 = _7;
_6 = -_1;
_9 = [1610951545_u32,3860918915_u32,3849074464_u32,743040670_u32,3561077515_u32,773281783_u32,3874470772_u32];
_4.0 = RET as usize;
Call(_4.2 = fn4(_4.1, _1, _8, _4.1, _6, _4.1, _7, _8, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {

RET = 44022_u16 * 12072_u16;
Call(_20 = dump_var(3_usize, 4_usize, Move(_4), 1_usize, Move(_1), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(3_usize, 7_usize, Move(_7), 3_usize, _3, 21_usize, _21, 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: f32,mut _3: i8,mut _4: bool,mut _5: f32,mut _6: bool,mut _7: u128,mut _8: i8,mut _9: u128) -> *const i128 {
mir! {
type RET = *const i128;
let _10: isize;
let _11: isize;
let _12: f64;
let _13: [bool; 5];
let _14: Adt50;
let _15: (i16, *mut u128, u8, usize, char);
let _16: f32;
let _17: [char; 6];
let _18: Adt49;
let _19: [u64; 2];
let _20: (i8, [i64; 7]);
let _21: f64;
let _22: isize;
let _23: *const *mut u128;
let _24: bool;
let _25: Adt52;
let _26: [usize; 7];
let _27: u32;
let _28: i8;
let _29: (*mut *const i128,);
let _30: [u32; 7];
let _31: u128;
let _32: u32;
let _33: bool;
let _34: f64;
let _35: *mut *const i128;
let _36: u64;
let _37: [bool; 5];
let _38: Adt51;
let _39: (i16, *mut u128, u8, usize, char);
let _40: f64;
let _41: i128;
let _42: *const *const i128;
let _43: i128;
let _44: ();
let _45: ();
{
_10 = 117_isize ^ 9223372036854775807_isize;
_1 = _6;
_11 = 5_usize as isize;
_2 = _5 * _5;
_5 = -_2;
_6 = _2 != _5;
_13 = [_1,_1,_1,_6,_6];
_12 = 31061_u16 as f64;
_4 = _6;
_10 = _11 + _11;
_7 = 13089197540555906451_u64 as u128;
_3 = _8 >> _7;
_16 = (-29906_i16) as f32;
_15.3 = 3_usize;
_15.0 = 23639_i16;
_15.4 = '\u{d8f0a}';
_15.1 = core::ptr::addr_of_mut!(_7);
_15.2 = 18_u8;
_1 = _4 >= _6;
_11 = _10;
_15.2 = 124_u8;
_15.0 = _5 as i16;
_13 = [_6,_4,_1,_4,_1];
_15.1 = core::ptr::addr_of_mut!(_7);
_9 = !_7;
_17 = [_15.4,_15.4,_15.4,_15.4,_15.4,_15.4];
_15.2 = 178_u8 * 254_u8;
_17 = [_15.4,_15.4,_15.4,_15.4,_15.4,_15.4];
_18.fld2 = Adt45::Variant0 { fld0: 123365666_i32 };
_2 = _5 - _5;
_18.fld0 = core::ptr::addr_of_mut!(_8);
_19 = [4340887969662122023_u64,13999130087992089548_u64];
_20.1 = [(-3815464143673625934_i64),(-7107046773800785162_i64),(-4552266517558406216_i64),(-4380910105551590026_i64),4520249811259067894_i64,(-4694315809086255163_i64),(-477259887993907948_i64)];
_7 = _9 * _9;
_18.fld1 = core::ptr::addr_of_mut!(RET);
_15.3 = _4 as usize;
_1 = _16 < _16;
_3 = _8 - _8;
_20.0 = _3 + _3;
_23 = core::ptr::addr_of!(_15.1);
_24 = _4 | _4;
_20.0 = _3;
(*_23) = core::ptr::addr_of_mut!(_7);
_16 = 2965_u16 as f32;
_16 = 1780939326_i32 as f32;
_15.0 = 11553623301210238130_u64 as i16;


_16 = _2;
_10 = !_11;
_24 = _1 & _1;
_22 = _15.3 as isize;
_8 = !_3;
_1 = _24;
_7 = !_9;
(*_23) = core::ptr::addr_of_mut!(_7);
_28 = !_8;
_33 = _2 <= _2;
_9 = _7 | _7;
_13 = [_24,_33,_33,_1,_24];
_32 = _9 as u32;
_10 = _22;
_33 = !_1;
RET = core::ptr::addr_of!(_41);
_8 = !_28;
(*RET) = 14675936918923860084_u64 as i128;
_17 = ['\u{e6b78}','\u{e6b78}','\u{e6b78}','\u{e6b78}','\u{e6b78}','\u{e6b78}'];
Call(_44 = dump_var(4_usize, 2_usize, Move(_2), 2_usize, Move(_2), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(4_usize, 33_usize, Move(_33), 24_usize, Move(_24), 10_usize, Move(_10), 17_usize, Move(_17)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(4_usize, 8_usize, Move(_8), 32_usize, Move(_32), 22_usize, Move(_22), 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: Adt49,mut _2: [bool; 5],mut _3: f32,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: [usize; 7],mut _8: f32,mut _9: [bool; 5]) -> *mut *const i128 {
mir! {
type RET = *mut *const i128;
let _10: [char; 6];
let _11: *mut u128;
let _12: i8;
let _13: isize;
let _14: (i8, [i64; 7]);
let _15: i16;
let _16: [usize; 7];
let _17: [char; 6];
let _18: &'static i32;
let _19: Adt47;
let _20: i32;
let _21: [u128; 1];
let _22: Adt51;
let _23: Adt44;
let _24: u16;
let _25: isize;
let _26: Adt55;
let _27: u16;
let _28: [u64; 2];
let _29: u64;
let _30: bool;
let _31: [u32; 1];
let _32: *const *const i128;
let _33: *mut u128;
let _34: ();
let _35: ();
{
_5 = _4;
_9 = [_4,_6,_4,_4,_6];
_5 = !_6;
_9 = [_4,_6,_4,_5,_5];
_4 = _6;
_6 = _4;
_10 = ['\u{c1d4e}','\u{79c03}','\u{9d30d}','\u{2049f}','\u{57506}','\u{bcf55}'];
_2 = _9;
_8 = Field::<i32>(Variant(_1.fld2, 0), 0) as f32;
_8 = _3 + _3;
_7 = [4_usize,11105358434440248087_usize,3_usize,3_usize,12547818892584202901_usize,4_usize,12926571728405607583_usize];
_5 = _6 & _6;
_4 = _6 | _5;
SetDiscriminant(_1.fld2, 0);
_1.fld2 = Adt45::Variant0 { fld0: (-226480462_i32) };
Call(_1 = fn6(_7, _2, _9, _4, _9, _5, _9, _3, _5, _3, _3, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !_6;
RET = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).1);
(*RET) = Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).3;
SetDiscriminant(_1.fld2, 0);
_9 = [_6,_6,_6,_4,_6];
_2 = [_6,_4,_5,_4,_5];
_7 = [1269888737600602728_usize,17183464481707717228_usize,13297791132327135044_usize,8645728320232870149_usize,17590566738372505044_usize,3_usize,15182308776724637098_usize];
_1.fld0 = core::ptr::addr_of_mut!(_12);
_9 = [_6,_5,_5,_5,_5];
_1.fld1 = RET;
_12 = -(-23_i8);
_1.fld1 = RET;
_2 = _9;
_14.1 = [(-7320173861129941640_i64),(-6342973970005620584_i64),(-7805971085812262592_i64),(-3226065781595111207_i64),(-5782975437687875389_i64),(-564636083458268038_i64),5002997522721443121_i64];
_12 = (-51_i8);
_14.0 = _12;
_7 = [7_usize,12739577009311439380_usize,13757943759814117590_usize,2080822791912419310_usize,3629144530282944007_usize,9404823932519153208_usize,6695772919381157817_usize];
_6 = !_4;
_10 = ['\u{f005d}','\u{92443}','\u{10421f}','\u{1cb8d}','\u{cbfae}','\u{5ce93}'];
_13 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_6 = _5;
_9 = _2;
_14.1 = [(-9179312247300406979_i64),(-4995546660435514466_i64),(-1453472451748669520_i64),4601893035600476313_i64,2966807366414995094_i64,(-3687147519310565200_i64),(-4918793722814265627_i64)];
_1.fld0 = core::ptr::addr_of_mut!(_12);
Goto(bb2)
}
bb2 = {
_2 = [_6,_5,_5,_4,_6];
_1.fld0 = core::ptr::addr_of_mut!(_12);
_10 = ['\u{ab657}','\u{2be6e}','\u{11ce3}','\u{655db}','\u{d76fb}','\u{ce25}'];
_5 = _4;
place!(Field::<i32>(Variant(_1.fld2, 0), 0)) = 4273539321919295176_u64 as i32;
_15 = 21848_i16 << _13;
_14.0 = _12 * _12;
_14.0 = -_12;
_10 = ['\u{4b307}','\u{230e7}','\u{35d1c}','\u{cee46}','\u{b6020}','\u{100810}'];
_12 = _14.0 - _14.0;
_1.fld1 = RET;
SetDiscriminant(_1.fld2, 0);
_6 = !_5;
RET = _1.fld1;
RET = _1.fld1;
_1.fld0 = core::ptr::addr_of_mut!(_12);
_15 = (-14213_i16);
place!(Field::<i32>(Variant(_1.fld2, 0), 0)) = -331926274_i32;
_3 = -_8;
_12 = _14.0;
_13 = (-67_isize);
match _13 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768211389 => bb10,
_ => bb9
}
}
bb3 = {
_5 = !_6;
RET = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).1);
(*RET) = Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).3;
SetDiscriminant(_1.fld2, 0);
_9 = [_6,_6,_6,_4,_6];
_2 = [_6,_4,_5,_4,_5];
_7 = [1269888737600602728_usize,17183464481707717228_usize,13297791132327135044_usize,8645728320232870149_usize,17590566738372505044_usize,3_usize,15182308776724637098_usize];
_1.fld0 = core::ptr::addr_of_mut!(_12);
_9 = [_6,_5,_5,_5,_5];
_1.fld1 = RET;
_12 = -(-23_i8);
_1.fld1 = RET;
_2 = _9;
_14.1 = [(-7320173861129941640_i64),(-6342973970005620584_i64),(-7805971085812262592_i64),(-3226065781595111207_i64),(-5782975437687875389_i64),(-564636083458268038_i64),5002997522721443121_i64];
_12 = (-51_i8);
_14.0 = _12;
_7 = [7_usize,12739577009311439380_usize,13757943759814117590_usize,2080822791912419310_usize,3629144530282944007_usize,9404823932519153208_usize,6695772919381157817_usize];
_6 = !_4;
_10 = ['\u{f005d}','\u{92443}','\u{10421f}','\u{1cb8d}','\u{cbfae}','\u{5ce93}'];
_13 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_6 = _5;
_9 = _2;
_14.1 = [(-9179312247300406979_i64),(-4995546660435514466_i64),(-1453472451748669520_i64),4601893035600476313_i64,2966807366414995094_i64,(-3687147519310565200_i64),(-4918793722814265627_i64)];
_1.fld0 = core::ptr::addr_of_mut!(_12);
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
_10 = ['\u{49101}','\u{89765}','\u{cb4f8}','\u{79a2d}','\u{6d9b9}','\u{897ea}'];
_12 = Field::<i32>(Variant(_1.fld2, 0), 0) as i8;
SetDiscriminant(_1.fld2, 1);
RET = _1.fld1;
_14.0 = (-1179831099006419863_i64) as i8;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).4 = 1203733040662552297_i64 as u8;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).0 = !_5;
_11 = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).2);
_12 = _4 as i8;
_3 = _8 * _8;
(*_11) = !57295430416470359977772735003744694322_u128;
_1.fld0 = core::ptr::addr_of_mut!(_12);
_3 = _8 - _8;
_20 = 1947222280_i32;
_16 = _7;
RET = _1.fld1;
place!(Field::<[char; 6]>(Variant(_1.fld2, 1), 1)) = ['\u{30ca}','\u{1356f}','\u{1aeec}','\u{e1327}','\u{d3efa}','\u{d0a27}'];
RET = core::ptr::addr_of_mut!(place!(Field::<*const i128>(Variant(_1.fld2, 1), 0)));
_21 = [(*_11)];
(*_11) = Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).4 as u128;
_1.fld1 = core::ptr::addr_of_mut!((*RET));
Goto(bb11)
}
bb11 = {
place!(Field::<[char; 6]>(Variant(_1.fld2, 1), 1)) = _10;
_18 = &_20;
_13 = 9223372036854775807_isize;
_15 = 30090_i16;
_15 = -(-13376_i16);
_11 = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).2);
_12 = -_14.0;
_17 = ['\u{f141a}','\u{b2e45}','\u{16284}','\u{60d51}','\u{108c91}','\u{cf9f3}'];
_15 = 6265_i16 | 1836_i16;
_19 = Adt47::Variant0 { fld0: _8,fld1: _15,fld2: (-4877838412351454362_i64) };
(*_11) = !96422390119479654405241094604102299156_u128;
_4 = _5;
place!(Field::<i64>(Variant(_19, 0), 2)) = -(-4340396687060986469_i64);
_6 = !Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).0;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).0 = !_5;
Goto(bb12)
}
bb12 = {
_15 = Field::<i16>(Variant(_19, 0), 1);
_13 = _4 as isize;
_2 = _9;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).0 = !_6;
SetDiscriminant(_19, 2);
_13 = (-9223372036854775808_isize);
place!(Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0)) = (_15, _11, Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).4, 15806552054264212361_usize, '\u{91de5}');
place!(Field::<[u32; 1]>(Variant(_1.fld2, 1), 2)) = [4261010766_u32];
_13 = (-9223372036854775808_isize) >> (*_18);
_19 = Adt47::Variant0 { fld0: _3,fld1: _15,fld2: 4833398955711334003_i64 };
_14.1 = [(-2605363799909163686_i64),4221735225957492979_i64,6371580037354276251_i64,(-8848834200903995268_i64),(-5745425872467671379_i64),7869936304999188174_i64,(-8689789223588041281_i64)];
_10 = _17;
_14.1 = [(-4909945117847656521_i64),5279777773433283840_i64,(-8391906740220718531_i64),1173974733101311650_i64,4345111452952134274_i64,(-1743480089598388237_i64),8436050896696921181_i64];
Goto(bb13)
}
bb13 = {
place!(Field::<f32>(Variant(_19, 0), 0)) = _3;
_24 = !63429_u16;
_25 = !_13;
(*_11) = !176206947122281940465787087890440196666_u128;
_6 = !_5;
_12 = _14.0 >> Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).4;
_14.0 = 1564154385349992618_u64 as i8;
_28 = [3999032589316867172_u64,9421510698185381629_u64];
_27 = _24 << _20;
_13 = !_25;
place!(Field::<i16>(Variant(_19, 0), 1)) = _15;
_8 = -_3;
place!(Field::<i64>(Variant(_19, 0), 2)) = 1444957937205788581_i64 << (*_11);
_18 = &(*_18);
_14.1 = [Field::<i64>(Variant(_19, 0), 2),Field::<i64>(Variant(_19, 0), 2),Field::<i64>(Variant(_19, 0), 2),Field::<i64>(Variant(_19, 0), 2),Field::<i64>(Variant(_19, 0), 2),Field::<i64>(Variant(_19, 0), 2),Field::<i64>(Variant(_19, 0), 2)];
SetDiscriminant(_19, 2);
_27 = _24;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).2 = !138890069483123513299847954071656734950_u128;
place!(Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0)).3 = !2_usize;
_16 = _7;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).0 = !_4;
place!(Field::<i16>(Variant(_19, 2), 2)) = _15 - _15;
_2 = _9;
place!(Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0)) = (Field::<i16>(Variant(_19, 2), 2), _11, Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3).4, 0_usize, '\u{5befe}');
Goto(bb14)
}
bb14 = {
_28 = [14293276868889625502_u64,7162763560972350583_u64];
_4 = _6;
_14.1 = [194311214672386303_i64,(-7839786490273569941_i64),(-257556352580661494_i64),8527726744030260743_i64,(-3339138993323981466_i64),4466761516538146254_i64,2336764334022009613_i64];
_9 = _2;
_10 = [Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0).4,Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0).4,Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0).4,Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0).4,Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0).4,Field::<(i16, *mut u128, u8, usize, char)>(Variant(_19, 2), 0).4];
_25 = 125142123064738094167440252516443845523_i128 as isize;
_32 = core::ptr::addr_of!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_1.fld2, 1), 3)).3);
_1.fld0 = core::ptr::addr_of_mut!(_12);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(5_usize, 14_usize, Move(_14), 17_usize, Move(_17), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(5_usize, 5_usize, Move(_5), 24_usize, Move(_24), 25_usize, Move(_25), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(5_usize, 6_usize, Move(_6), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [usize; 7],mut _2: [bool; 5],mut _3: [bool; 5],mut _4: bool,mut _5: [bool; 5],mut _6: bool,mut _7: [bool; 5],mut _8: f32,mut _9: bool,mut _10: f32,mut _11: f32,mut _12: bool,mut _13: bool) -> Adt49 {
mir! {
type RET = Adt49;
let _14: [u32; 1];
let _15: i128;
let _16: (bool, *const i128, u128, *const i128, u8);
let _17: f64;
let _18: isize;
let _19: char;
let _20: f64;
let _21: u8;
let _22: Adt52;
let _23: [i64; 7];
let _24: u32;
let _25: char;
let _26: bool;
let _27: i128;
let _28: [bool; 5];
let _29: (f64, i8, f32);
let _30: [char; 6];
let _31: [bool; 5];
let _32: Adt47;
let _33: Adt52;
let _34: ();
let _35: ();
{
_2 = [_9,_6,_4,_6,_13];
_5 = [_4,_6,_4,_4,_6];
_12 = _9 & _9;
_1 = [6645491364236322130_usize,0_usize,7_usize,5_usize,2325801976128187979_usize,4_usize,2_usize];
_4 = !_12;
_10 = 134544299117071212649267631174578358086_u128 as f32;
_9 = !_13;
_6 = _8 <= _8;
_6 = !_12;
_7 = [_13,_13,_9,_4,_13];
Goto(bb1)
}
bb1 = {
_9 = _11 != _11;
_16.2 = (-664683297_i32) as u128;
_16.1 = core::ptr::addr_of!(_15);
_16.4 = 59_u8;
RET.fld1 = core::ptr::addr_of_mut!(_16.3);
_15 = -33409226405901216768490674753329979864_i128;
_10 = -_11;
_16.2 = _9 as u128;
_19 = '\u{735b5}';
_12 = _13 & _9;
Call(RET = fn7(_16.2, _8, _5, _3, _7, _16.2, _5, _16.2, _10, _7, _10, _13, _9, _3, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
SetDiscriminant(RET.fld2, 1);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_15);
_16.0 = !_13;
_10 = _11;
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = [_19,_19,_19,_19,_19,_19];
_16.4 = 111_u8 - 223_u8;
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = [_19,_19,_19,_19,_19,_19];
RET.fld2 = Adt45::Variant0 { fld0: 1189602953_i32 };
_23 = [1514758906340949774_i64,(-5550917897751457581_i64),8423705565393803779_i64,8686978082369856578_i64,(-9014994125401370751_i64),(-8567544470441931577_i64),(-7118573028928080415_i64)];
_20 = _15 as f64;
_14 = [1716985607_u32];
_16.1 = core::ptr::addr_of!(_15);
_16.2 = 255763044466912530095017472709192516191_u128 + 194744655494803930231623573723208926636_u128;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = 400491799_i32;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = 827092005_i32 + 1690258895_i32;
_17 = -_20;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = (-200011742_i32);
_19 = '\u{2b23a}';
_23 = [2170720668286831904_i64,(-7584221618463271767_i64),660697910883050378_i64,5035534483857840276_i64,2063371601395853963_i64,(-3107467537355163445_i64),(-3510761543603582362_i64)];
_6 = !_9;
_16.4 = 198_u8 & 238_u8;
_10 = -_11;
_17 = (-58_i8) as f64;
_17 = -_20;
match Field::<i32>(Variant(RET.fld2, 0), 0) {
0 => bb1,
1 => bb3,
340282366920938463463374607431568199714 => bb5,
_ => bb4
}
}
bb3 = {
_9 = _11 != _11;
_16.2 = (-664683297_i32) as u128;
_16.1 = core::ptr::addr_of!(_15);
_16.4 = 59_u8;
RET.fld1 = core::ptr::addr_of_mut!(_16.3);
_15 = -33409226405901216768490674753329979864_i128;
_10 = -_11;
_16.2 = _9 as u128;
_19 = '\u{735b5}';
_12 = _13 & _9;
Call(RET = fn7(_16.2, _8, _5, _3, _7, _16.2, _5, _16.2, _10, _7, _10, _13, _9, _3, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_20 = _17;
_8 = _11 * _11;
_13 = _12 == _9;
_28 = _3;
_8 = -_10;
_29.0 = _16.2 as f64;
_24 = 696243647_u32;
_30 = [_19,_19,_19,_19,_19,_19];
_29 = (_17, (-87_i8), _10);
_16.0 = !_9;
match _29.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431768211369 => bb10,
_ => bb9
}
}
bb6 = {
Return()
}
bb7 = {
_9 = _11 != _11;
_16.2 = (-664683297_i32) as u128;
_16.1 = core::ptr::addr_of!(_15);
_16.4 = 59_u8;
RET.fld1 = core::ptr::addr_of_mut!(_16.3);
_15 = -33409226405901216768490674753329979864_i128;
_10 = -_11;
_16.2 = _9 as u128;
_19 = '\u{735b5}';
_12 = _13 & _9;
Call(RET = fn7(_16.2, _8, _5, _3, _7, _16.2, _5, _16.2, _10, _7, _10, _13, _9, _3, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
SetDiscriminant(RET.fld2, 1);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_15);
_16.0 = !_13;
_10 = _11;
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = [_19,_19,_19,_19,_19,_19];
_16.4 = 111_u8 - 223_u8;
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = [_19,_19,_19,_19,_19,_19];
RET.fld2 = Adt45::Variant0 { fld0: 1189602953_i32 };
_23 = [1514758906340949774_i64,(-5550917897751457581_i64),8423705565393803779_i64,8686978082369856578_i64,(-9014994125401370751_i64),(-8567544470441931577_i64),(-7118573028928080415_i64)];
_20 = _15 as f64;
_14 = [1716985607_u32];
_16.1 = core::ptr::addr_of!(_15);
_16.2 = 255763044466912530095017472709192516191_u128 + 194744655494803930231623573723208926636_u128;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = 400491799_i32;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = 827092005_i32 + 1690258895_i32;
_17 = -_20;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = (-200011742_i32);
_19 = '\u{2b23a}';
_23 = [2170720668286831904_i64,(-7584221618463271767_i64),660697910883050378_i64,5035534483857840276_i64,2063371601395853963_i64,(-3107467537355163445_i64),(-3510761543603582362_i64)];
_6 = !_9;
_16.4 = 198_u8 & 238_u8;
_10 = -_11;
_17 = (-58_i8) as f64;
_17 = -_20;
match Field::<i32>(Variant(RET.fld2, 0), 0) {
0 => bb1,
1 => bb3,
340282366920938463463374607431568199714 => bb5,
_ => bb4
}
}
bb9 = {
_9 = _11 != _11;
_16.2 = (-664683297_i32) as u128;
_16.1 = core::ptr::addr_of!(_15);
_16.4 = 59_u8;
RET.fld1 = core::ptr::addr_of_mut!(_16.3);
_15 = -33409226405901216768490674753329979864_i128;
_10 = -_11;
_16.2 = _9 as u128;
_19 = '\u{735b5}';
_12 = _13 & _9;
Call(RET = fn7(_16.2, _8, _5, _3, _7, _16.2, _5, _16.2, _10, _7, _10, _13, _9, _3, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_2 = [_9,_16.0,_4,_13,_6];
_1 = [1393949465599001067_usize,17260402444913714845_usize,17958981387484354308_usize,14576639445481126869_usize,6_usize,6314467610869985917_usize,0_usize];
_30 = [_19,_19,_19,_19,_19,_19];
_29.1 = 43_i8;
SetDiscriminant(RET.fld2, 1);
_29 = (_17, (-53_i8), _8);
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = [_19,_19,_19,_19,_19,_19];
place!(Field::<*const i128>(Variant(RET.fld2, 1), 0)) = _16.1;
_25 = _19;
_4 = !_16.0;
_29 = (_20, 79_i8, _11);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).0 = !_9;
_28 = _5;
_17 = _20;
_14 = [_24];
RET.fld1 = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1);
_16 = (Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3).0, Field::<*const i128>(Variant(RET.fld2, 1), 0), 247618918901168729001650453115804568511_u128, Field::<*const i128>(Variant(RET.fld2, 1), 0), 17_u8);
RET.fld2 = Adt45::Variant1 { fld0: _16.1,fld1: _30,fld2: _14,fld3: _16 };
_13 = _16.2 != Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3).2;
_4 = _11 == _8;
Goto(bb11)
}
bb11 = {
Call(_34 = dump_var(6_usize, 12_usize, Move(_12), 28_usize, Move(_28), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_34 = dump_var(6_usize, 15_usize, Move(_15), 14_usize, Move(_14), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_34 = dump_var(6_usize, 25_usize, Move(_25), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u128,mut _2: f32,mut _3: [bool; 5],mut _4: [bool; 5],mut _5: [bool; 5],mut _6: u128,mut _7: [bool; 5],mut _8: u128,mut _9: f32,mut _10: [bool; 5],mut _11: f32,mut _12: bool,mut _13: bool,mut _14: [bool; 5],mut _15: bool) -> Adt49 {
mir! {
type RET = Adt49;
let _16: [i8; 5];
let _17: i32;
let _18: Adt54;
let _19: Adt48;
let _20: [usize; 7];
let _21: [i64; 7];
let _22: i32;
let _23: u64;
let _24: isize;
let _25: [u128; 1];
let _26: [char; 6];
let _27: [bool; 5];
let _28: u128;
let _29: [u32; 7];
let _30: [i64; 7];
let _31: i8;
let _32: Adt56;
let _33: [bool; 5];
let _34: (usize, bool, *const i128);
let _35: [u32; 7];
let _36: u64;
let _37: [u32; 7];
let _38: i128;
let _39: Adt54;
let _40: bool;
let _41: isize;
let _42: i16;
let _43: Adt56;
let _44: Adt49;
let _45: Adt60;
let _46: f32;
let _47: [u32; 7];
let _48: ();
let _49: ();
{
_14 = [_12,_15,_12,_12,_13];
_9 = 740552211_i32 as f32;
_9 = _2 * _11;
RET.fld2 = Adt45::Variant0 { fld0: 1190552285_i32 };
RET.fld2 = Adt45::Variant0 { fld0: (-1290749550_i32) };
RET.fld2 = Adt45::Variant0 { fld0: (-926022905_i32) };
_15 = _13;
RET.fld2 = Adt45::Variant0 { fld0: (-501606492_i32) };
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = '\u{10f227}' as i32;
_4 = _3;
_17 = Field::<i32>(Variant(RET.fld2, 0), 0);
_11 = -_9;
_7 = [_15,_15,_13,_15,_13];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
_5 = [_15,_12,_12,_15,_12];
_16 = [(-99_i8),71_i8,69_i8,106_i8,(-101_i8)];
_15 = _13;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _17;
_14 = [_12,_15,_12,_12,_13];
_5 = [_12,_15,_13,_15,_12];
_12 = _13;
Goto(bb1)
}
bb1 = {
_15 = !_12;
_1 = !_6;
_5 = _3;
_15 = _2 >= _11;
_2 = -_9;
_6 = (-7878_i16) as u128;
_12 = _15;
_9 = _11 * _2;
_7 = _3;
_9 = _2 - _11;
_1 = !_8;
_3 = [_13,_15,_12,_13,_13];
Goto(bb2)
}
bb2 = {
_7 = [_13,_13,_12,_13,_12];
_8 = _1;
_13 = !_15;
_7 = [_13,_13,_15,_15,_15];
_10 = [_12,_15,_12,_13,_13];
_4 = _14;
_16 = [(-65_i8),(-2_i8),(-4_i8),15_i8,40_i8];
_8 = _1 << _1;
_10 = [_12,_13,_12,_13,_13];
_14 = [_15,_12,_15,_12,_13];
_17 = !Field::<i32>(Variant(RET.fld2, 0), 0);
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = !_17;
_7 = [_15,_12,_15,_13,_12];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
RET.fld2 = Adt45::Variant0 { fld0: _17 };
Call(place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = fn8(_9, _12, _7, _2, _2, _10, _14, _1, _2, _7, _15, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = [69_i8,(-48_i8),(-11_i8),(-15_i8),(-86_i8)];
_20 = [3387881348012868948_usize,3053300862051252600_usize,12763628861428068092_usize,7_usize,1_usize,12295263188761292048_usize,2_usize];
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _17 * _17;
_9 = _2;
_3 = [_13,_12,_13,_13,_15];
_10 = _4;
_2 = _9 * _11;
_13 = _15;
SetDiscriminant(RET.fld2, 0);
Call(_8 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = _1 >> _1;
_22 = '\u{7cd47}' as i32;
_21 = [571760042676189400_i64,6639641909351290477_i64,4608420516914892778_i64,5085413610162400748_i64,(-9064703036984979140_i64),8619905958194513576_i64,(-4714897942481692200_i64)];
_13 = !_15;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _22;
_4 = [_12,_15,_12,_15,_15];
_17 = _22 * Field::<i32>(Variant(RET.fld2, 0), 0);
_12 = _13;
_3 = _4;
_23 = 16727395052239814184_u64 | 2702684463836440788_u64;
Goto(bb5)
}
bb5 = {
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = !_17;
_11 = 157705244133670399706242687798511467131_i128 as f32;
Goto(bb6)
}
bb6 = {
_4 = [_15,_15,_15,_12,_12];
_20 = [2938350096323102102_usize,4369642002055315688_usize,11712256459792521308_usize,1_usize,0_usize,6334862101449675898_usize,12631762611589456961_usize];
_16 = [(-89_i8),26_i8,38_i8,(-102_i8),(-7_i8)];
_27 = [_13,_13,_15,_15,_13];
_26 = ['\u{2261}','\u{5d7b5}','\u{f8c91}','\u{b3b8e}','\u{42c73}','\u{107aa4}'];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
SetDiscriminant(RET.fld2, 0);
Goto(bb7)
}
bb7 = {
_8 = !_6;
_4 = _7;
_20 = [2206203466138222317_usize,3895870553546546697_usize,939938923561466095_usize,3_usize,6_usize,6_usize,16205042833588227289_usize];
_23 = !3737203673683668228_u64;
_14 = [_15,_13,_15,_13,_12];
_3 = [_13,_13,_15,_15,_13];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
_10 = _7;
_17 = Field::<i32>(Variant(RET.fld2, 0), 0);
_25 = [_1];
_29 = [3020897496_u32,2742455836_u32,180382620_u32,1285908322_u32,2826236454_u32,1138746738_u32,2319106969_u32];
_12 = !_15;
_25 = [_1];
_9 = -_2;
_22 = Field::<i32>(Variant(RET.fld2, 0), 0) - _17;
_16 = [(-88_i8),(-114_i8),(-19_i8),13_i8,(-87_i8)];
_22 = 3217218400_u32 as i32;
RET.fld0 = core::ptr::addr_of_mut!(_31);
_16 = [50_i8,(-71_i8),114_i8,42_i8,121_i8];
_21 = [(-1362672548899520566_i64),4434612636101254182_i64,717704197518494631_i64,(-764027492552589544_i64),4223641041486734579_i64,(-6227049872559308026_i64),(-6439605705530424443_i64)];
_1 = _8 ^ _6;
_31 = 10_i8;
_12 = _15;
_11 = -_9;
_24 = (-35_isize);
_5 = _4;
_22 = -_17;
_16 = [_31,_31,_31,_31,_31];
match _24 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
340282366920938463463374607431768211421 => bb10,
_ => bb9
}
}
bb8 = {
_7 = [_13,_13,_12,_13,_12];
_8 = _1;
_13 = !_15;
_7 = [_13,_13,_15,_15,_15];
_10 = [_12,_15,_12,_13,_13];
_4 = _14;
_16 = [(-65_i8),(-2_i8),(-4_i8),15_i8,40_i8];
_8 = _1 << _1;
_10 = [_12,_13,_12,_13,_13];
_14 = [_15,_12,_15,_12,_13];
_17 = !Field::<i32>(Variant(RET.fld2, 0), 0);
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = !_17;
_7 = [_15,_12,_15,_13,_12];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
RET.fld2 = Adt45::Variant0 { fld0: _17 };
Call(place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = fn8(_9, _12, _7, _2, _2, _10, _14, _1, _2, _7, _15, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_6 = _1 >> _1;
_22 = '\u{7cd47}' as i32;
_21 = [571760042676189400_i64,6639641909351290477_i64,4608420516914892778_i64,5085413610162400748_i64,(-9064703036984979140_i64),8619905958194513576_i64,(-4714897942481692200_i64)];
_13 = !_15;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _22;
_4 = [_12,_15,_12,_15,_15];
_17 = _22 * Field::<i32>(Variant(RET.fld2, 0), 0);
_12 = _13;
_3 = _4;
_23 = 16727395052239814184_u64 | 2702684463836440788_u64;
Goto(bb5)
}
bb10 = {
_20 = [9838842644875873102_usize,3_usize,16365247111312567260_usize,13783433552170188976_usize,4_usize,11835455425233678177_usize,2031877897054648339_usize];
_22 = !_17;
_6 = _1;
_27 = _7;
_29 = [1461523623_u32,3614776828_u32,2620503669_u32,3442780397_u32,4188923200_u32,196025814_u32,3006397179_u32];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
_11 = _2 * _9;
match _31 {
0 => bb7,
1 => bb4,
2 => bb11,
3 => bb12,
10 => bb14,
_ => bb13
}
}
bb11 = {
_6 = _1 >> _1;
_22 = '\u{7cd47}' as i32;
_21 = [571760042676189400_i64,6639641909351290477_i64,4608420516914892778_i64,5085413610162400748_i64,(-9064703036984979140_i64),8619905958194513576_i64,(-4714897942481692200_i64)];
_13 = !_15;
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _22;
_4 = [_12,_15,_12,_15,_15];
_17 = _22 * Field::<i32>(Variant(RET.fld2, 0), 0);
_12 = _13;
_3 = _4;
_23 = 16727395052239814184_u64 | 2702684463836440788_u64;
Goto(bb5)
}
bb12 = {
_7 = [_13,_13,_12,_13,_12];
_8 = _1;
_13 = !_15;
_7 = [_13,_13,_15,_15,_15];
_10 = [_12,_15,_12,_13,_13];
_4 = _14;
_16 = [(-65_i8),(-2_i8),(-4_i8),15_i8,40_i8];
_8 = _1 << _1;
_10 = [_12,_13,_12,_13,_13];
_14 = [_15,_12,_15,_12,_13];
_17 = !Field::<i32>(Variant(RET.fld2, 0), 0);
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = !_17;
_7 = [_15,_12,_15,_13,_12];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
RET.fld2 = Adt45::Variant0 { fld0: _17 };
Call(place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = fn8(_9, _12, _7, _2, _2, _10, _14, _1, _2, _7, _15, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_16 = [69_i8,(-48_i8),(-11_i8),(-15_i8),(-86_i8)];
_20 = [3387881348012868948_usize,3053300862051252600_usize,12763628861428068092_usize,7_usize,1_usize,12295263188761292048_usize,2_usize];
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _17 * _17;
_9 = _2;
_3 = [_13,_12,_13,_13,_15];
_10 = _4;
_2 = _9 * _11;
_13 = _15;
SetDiscriminant(RET.fld2, 0);
Call(_8 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_12 = !_15;
_20 = [2917405304700224056_usize,4924150813929545494_usize,2_usize,1_usize,2_usize,9725859608639174887_usize,0_usize];
_1 = _12 as u128;
_33 = _4;
_31 = !89_i8;
_26 = ['\u{a49fc}','\u{95d31}','\u{bc9c6}','\u{ac989}','\u{ee36e}','\u{c5fd}'];
_4 = [_13,_15,_15,_13,_15];
_6 = !_8;
_34.1 = _1 == _6;
_36 = _23 - _23;
_2 = -_11;
_29 = [2617775426_u32,2004808382_u32,3263715713_u32,2738308435_u32,3904357484_u32,2751576099_u32,3342827728_u32];
_4 = _33;
_20 = [1235505312958653264_usize,11645658031128178508_usize,5_usize,2_usize,2_usize,1_usize,2935561125510827441_usize];
RET.fld0 = core::ptr::addr_of_mut!(_31);
_25 = [_6];
_29 = [3571398705_u32,2420401030_u32,176867284_u32,1581190799_u32,3939006023_u32,1523438969_u32,1896245090_u32];
Goto(bb15)
}
bb15 = {
_16 = [_31,_31,_31,_31,_31];
SetDiscriminant(RET.fld2, 1);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_38);
_37 = [1857530640_u32,2609582851_u32,1893883521_u32,1451710513_u32,3271423003_u32,2298750454_u32,2955667950_u32];
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_38);
_36 = _31 as u64;
_30 = _21;
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = _26;
_27 = [_13,_34.1,_34.1,_13,_15];
_29 = [771620787_u32,1562539212_u32,1043226225_u32,1195054621_u32,4265013630_u32,1416168006_u32,3834319845_u32];
_27 = [_34.1,_15,_12,_15,_34.1];
_27 = [_34.1,_13,_12,_34.1,_12];
RET.fld1 = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1);
_9 = -_11;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).3 = core::ptr::addr_of!(_38);
place!(Field::<*const i128>(Variant(RET.fld2, 1), 0)) = core::ptr::addr_of!(_38);
_41 = _24 ^ _24;
_33 = [_13,_13,_13,_34.1,_13];
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_38);
_36 = _23;
_12 = _13;
Goto(bb16)
}
bb16 = {
_15 = !_13;
_8 = !_1;
_9 = _11;
_21 = [(-5940052551529333252_i64),6267285199165986481_i64,7936745230914367559_i64,(-5299442827053556721_i64),5310661891065970393_i64,4038493647750661085_i64,(-4218896070183144092_i64)];
_4 = [_13,_13,_13,_13,_13];
_34.2 = core::ptr::addr_of!(_38);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).2 = !_8;
match _24 {
0 => bb10,
1 => bb2,
2 => bb17,
3 => bb18,
4 => bb19,
5 => bb20,
6 => bb21,
340282366920938463463374607431768211421 => bb23,
_ => bb22
}
}
bb17 = {
_16 = [_31,_31,_31,_31,_31];
SetDiscriminant(RET.fld2, 1);
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_38);
_37 = [1857530640_u32,2609582851_u32,1893883521_u32,1451710513_u32,3271423003_u32,2298750454_u32,2955667950_u32];
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_38);
_36 = _31 as u64;
_30 = _21;
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = _26;
_27 = [_13,_34.1,_34.1,_13,_15];
_29 = [771620787_u32,1562539212_u32,1043226225_u32,1195054621_u32,4265013630_u32,1416168006_u32,3834319845_u32];
_27 = [_34.1,_15,_12,_15,_34.1];
_27 = [_34.1,_13,_12,_34.1,_12];
RET.fld1 = core::ptr::addr_of_mut!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1);
_9 = -_11;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).3 = core::ptr::addr_of!(_38);
place!(Field::<*const i128>(Variant(RET.fld2, 1), 0)) = core::ptr::addr_of!(_38);
_41 = _24 ^ _24;
_33 = [_13,_13,_13,_34.1,_13];
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).1 = core::ptr::addr_of!(_38);
_36 = _23;
_12 = _13;
Goto(bb16)
}
bb18 = {
_12 = !_15;
_20 = [2917405304700224056_usize,4924150813929545494_usize,2_usize,1_usize,2_usize,9725859608639174887_usize,0_usize];
_1 = _12 as u128;
_33 = _4;
_31 = !89_i8;
_26 = ['\u{a49fc}','\u{95d31}','\u{bc9c6}','\u{ac989}','\u{ee36e}','\u{c5fd}'];
_4 = [_13,_15,_15,_13,_15];
_6 = !_8;
_34.1 = _1 == _6;
_36 = _23 - _23;
_2 = -_11;
_29 = [2617775426_u32,2004808382_u32,3263715713_u32,2738308435_u32,3904357484_u32,2751576099_u32,3342827728_u32];
_4 = _33;
_20 = [1235505312958653264_usize,11645658031128178508_usize,5_usize,2_usize,2_usize,1_usize,2935561125510827441_usize];
RET.fld0 = core::ptr::addr_of_mut!(_31);
_25 = [_6];
_29 = [3571398705_u32,2420401030_u32,176867284_u32,1581190799_u32,3939006023_u32,1523438969_u32,1896245090_u32];
Goto(bb15)
}
bb19 = {
_7 = [_13,_13,_12,_13,_12];
_8 = _1;
_13 = !_15;
_7 = [_13,_13,_15,_15,_15];
_10 = [_12,_15,_12,_13,_13];
_4 = _14;
_16 = [(-65_i8),(-2_i8),(-4_i8),15_i8,40_i8];
_8 = _1 << _1;
_10 = [_12,_13,_12,_13,_13];
_14 = [_15,_12,_15,_12,_13];
_17 = !Field::<i32>(Variant(RET.fld2, 0), 0);
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = !_17;
_7 = [_15,_12,_15,_13,_12];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
RET.fld2 = Adt45::Variant0 { fld0: _17 };
Call(place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = fn8(_9, _12, _7, _2, _2, _10, _14, _1, _2, _7, _15, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb20 = {
_4 = [_15,_15,_15,_12,_12];
_20 = [2938350096323102102_usize,4369642002055315688_usize,11712256459792521308_usize,1_usize,0_usize,6334862101449675898_usize,12631762611589456961_usize];
_16 = [(-89_i8),26_i8,38_i8,(-102_i8),(-7_i8)];
_27 = [_13,_13,_15,_15,_13];
_26 = ['\u{2261}','\u{5d7b5}','\u{f8c91}','\u{b3b8e}','\u{42c73}','\u{107aa4}'];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
SetDiscriminant(RET.fld2, 0);
Goto(bb7)
}
bb21 = {
_16 = [69_i8,(-48_i8),(-11_i8),(-15_i8),(-86_i8)];
_20 = [3387881348012868948_usize,3053300862051252600_usize,12763628861428068092_usize,7_usize,1_usize,12295263188761292048_usize,2_usize];
place!(Field::<i32>(Variant(RET.fld2, 0), 0)) = _17 * _17;
_9 = _2;
_3 = [_13,_12,_13,_13,_15];
_10 = _4;
_2 = _9 * _11;
_13 = _15;
SetDiscriminant(RET.fld2, 0);
Call(_8 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb22 = {
_8 = !_6;
_4 = _7;
_20 = [2206203466138222317_usize,3895870553546546697_usize,939938923561466095_usize,3_usize,6_usize,6_usize,16205042833588227289_usize];
_23 = !3737203673683668228_u64;
_14 = [_15,_13,_15,_13,_12];
_3 = [_13,_13,_15,_15,_13];
RET.fld2 = Adt45::Variant0 { fld0: _17 };
_10 = _7;
_17 = Field::<i32>(Variant(RET.fld2, 0), 0);
_25 = [_1];
_29 = [3020897496_u32,2742455836_u32,180382620_u32,1285908322_u32,2826236454_u32,1138746738_u32,2319106969_u32];
_12 = !_15;
_25 = [_1];
_9 = -_2;
_22 = Field::<i32>(Variant(RET.fld2, 0), 0) - _17;
_16 = [(-88_i8),(-114_i8),(-19_i8),13_i8,(-87_i8)];
_22 = 3217218400_u32 as i32;
RET.fld0 = core::ptr::addr_of_mut!(_31);
_16 = [50_i8,(-71_i8),114_i8,42_i8,121_i8];
_21 = [(-1362672548899520566_i64),4434612636101254182_i64,717704197518494631_i64,(-764027492552589544_i64),4223641041486734579_i64,(-6227049872559308026_i64),(-6439605705530424443_i64)];
_1 = _8 ^ _6;
_31 = 10_i8;
_12 = _15;
_11 = -_9;
_24 = (-35_isize);
_5 = _4;
_22 = -_17;
_16 = [_31,_31,_31,_31,_31];
match _24 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
340282366920938463463374607431768211421 => bb10,
_ => bb9
}
}
bb23 = {
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).4 = _22 as u8;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).0 = !_12;
_34.1 = _15;
_40 = _34.1 ^ Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3).0;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(RET.fld2, 1), 3)).3 = core::ptr::addr_of!(_38);
place!(Field::<[char; 6]>(Variant(RET.fld2, 1), 1)) = ['\u{f710e}','\u{dd345}','\u{1003f7}','\u{7532}','\u{1c25}','\u{a62cc}'];
place!(Field::<[u32; 1]>(Variant(RET.fld2, 1), 2)) = [2679006394_u32];
_1 = _24 as u128;
place!(Field::<*const i128>(Variant(RET.fld2, 1), 0)) = core::ptr::addr_of!(_38);
_34.0 = 492519048022078913_usize + 17547851760254911512_usize;
Goto(bb24)
}
bb24 = {
Call(_48 = dump_var(7_usize, 12_usize, Move(_12), 4_usize, Move(_4), 1_usize, Move(_1), 22_usize, Move(_22)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_48 = dump_var(7_usize, 13_usize, Move(_13), 14_usize, Move(_14), 10_usize, Move(_10), 41_usize, Move(_41)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_48 = dump_var(7_usize, 24_usize, Move(_24), 37_usize, Move(_37), 27_usize, Move(_27), 26_usize, Move(_26)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_48 = dump_var(7_usize, 31_usize, Move(_31), 29_usize, Move(_29), 8_usize, Move(_8), 49_usize, _49), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: f32,mut _2: bool,mut _3: [bool; 5],mut _4: f32,mut _5: f32,mut _6: [bool; 5],mut _7: [bool; 5],mut _8: u128,mut _9: f32,mut _10: [bool; 5],mut _11: bool,mut _12: bool) -> i32 {
mir! {
type RET = i32;
let _13: f32;
let _14: u64;
let _15: (*mut *const i128,);
let _16: f32;
let _17: Adt44;
let _18: [i64; 7];
let _19: Adt52;
let _20: i32;
let _21: i32;
let _22: [i8; 5];
let _23: u32;
let _24: isize;
let _25: bool;
let _26: [bool; 5];
let _27: [u32; 1];
let _28: isize;
let _29: [bool; 5];
let _30: char;
let _31: f64;
let _32: ();
let _33: ();
{
_13 = 1627579484_u32 as f32;
_7 = [_12,_2,_11,_2,_12];
_7 = [_11,_2,_11,_2,_2];
RET = (-1950144190_i32) * 1117611979_i32;
_12 = _2;
_10 = [_2,_12,_11,_12,_2];
_13 = _4 * _5;
_12 = !_11;
_3 = _7;
RET = (-140322696_i32) ^ 1335562811_i32;
_10 = _6;
_14 = 11588048962059254568_u64 & 8139105605018634641_u64;
Goto(bb1)
}
bb1 = {
_14 = !4721668567424878244_u64;
_11 = !_12;
_8 = 87_isize as u128;
_13 = _1 * _9;
_13 = RET as f32;
_11 = !_2;
_2 = _11 ^ _12;
_3 = [_2,_12,_12,_12,_12];
_3 = [_2,_11,_2,_11,_2];
_6 = _3;
_14 = _4 as u64;
_3 = _7;
_3 = _6;
_8 = !314873603242364977858806026662149529230_u128;
_8 = (-107874170775136662767391790565755461798_i128) as u128;
_1 = RET as f32;
_9 = _5 * _5;
Call(_11 = fn9(_12, _5, _9, _6, _6, _14, _10, _4, _6, _12, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = -_5;
_1 = _9 * _4;
_16 = _13 * _5;
_7 = [_11,_2,_11,_11,_11];
_12 = _2;
_7 = _6;
_12 = _11 != _2;
_3 = [_11,_12,_11,_2,_2];
_5 = _16;
_1 = (-97_i8) as f32;
_2 = _12;
Goto(bb3)
}
bb3 = {
_3 = [_11,_11,_2,_12,_2];
_3 = [_2,_2,_2,_12,_11];
_14 = !17102087616290426615_u64;
_11 = _12 | _12;
_3 = [_2,_2,_11,_11,_12];
_16 = _4 * _5;
_10 = [_2,_12,_12,_12,_11];
_5 = _16 * _9;
_13 = -_16;
_4 = _5 + _9;
_18 = [(-8674570007642947966_i64),(-3255263739981842072_i64),9088338041222023554_i64,(-5552182493654051489_i64),8115418472082877736_i64,(-6574079855901305361_i64),3204774753382099301_i64];
_8 = 193890297518515866183439830248126613837_u128;
_20 = RET << RET;
RET = _20 + _20;
_2 = _4 <= _5;
_20 = -RET;
_10 = _6;
_20 = RET + RET;
_20 = RET * RET;
_1 = 3380877254524444166_i64 as f32;
_5 = _4;
Goto(bb4)
}
bb4 = {
_2 = !_11;
_16 = _4;
_1 = -_9;
_23 = 155_u8 as u32;
_3 = _7;
_22 = [(-112_i8),126_i8,(-33_i8),(-107_i8),126_i8];
_6 = [_2,_2,_11,_12,_11];
_21 = !RET;
_23 = 76_i8 as u32;
_7 = _10;
_21 = 7_usize as i32;
_25 = _11 != _11;
_3 = [_11,_12,_25,_11,_12];
_24 = (-123_isize);
_13 = _14 as f32;
match _8 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
193890297518515866183439830248126613837 => bb11,
_ => bb10
}
}
bb5 = {
_3 = [_11,_11,_2,_12,_2];
_3 = [_2,_2,_2,_12,_11];
_14 = !17102087616290426615_u64;
_11 = _12 | _12;
_3 = [_2,_2,_11,_11,_12];
_16 = _4 * _5;
_10 = [_2,_12,_12,_12,_11];
_5 = _16 * _9;
_13 = -_16;
_4 = _5 + _9;
_18 = [(-8674570007642947966_i64),(-3255263739981842072_i64),9088338041222023554_i64,(-5552182493654051489_i64),8115418472082877736_i64,(-6574079855901305361_i64),3204774753382099301_i64];
_8 = 193890297518515866183439830248126613837_u128;
_20 = RET << RET;
RET = _20 + _20;
_2 = _4 <= _5;
_20 = -RET;
_10 = _6;
_20 = RET + RET;
_20 = RET * RET;
_1 = 3380877254524444166_i64 as f32;
_5 = _4;
Goto(bb4)
}
bb6 = {
_4 = -_5;
_1 = _9 * _4;
_16 = _13 * _5;
_7 = [_11,_2,_11,_11,_11];
_12 = _2;
_7 = _6;
_12 = _11 != _2;
_3 = [_11,_12,_11,_2,_2];
_5 = _16;
_1 = (-97_i8) as f32;
_2 = _12;
Goto(bb3)
}
bb7 = {
_14 = !4721668567424878244_u64;
_11 = !_12;
_8 = 87_isize as u128;
_13 = _1 * _9;
_13 = RET as f32;
_11 = !_2;
_2 = _11 ^ _12;
_3 = [_2,_12,_12,_12,_12];
_3 = [_2,_11,_2,_11,_2];
_6 = _3;
_14 = _4 as u64;
_3 = _7;
_3 = _6;
_8 = !314873603242364977858806026662149529230_u128;
_8 = (-107874170775136662767391790565755461798_i128) as u128;
_1 = RET as f32;
_9 = _5 * _5;
Call(_11 = fn9(_12, _5, _9, _6, _6, _14, _10, _4, _6, _12, _10), ReturnTo(bb2), UnwindUnreachable())
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
_1 = _14 as f32;
_1 = -_9;
_8 = 254778510542851148383910860426129500866_u128 & 219383193871956161168976461662341064653_u128;
Call(_14 = fn10(_6, _25, _2, _3, _4, _5, _11, RET, _11, _2, _9, _7), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_24 = (-37_isize) << _8;
_14 = (-5087195467170521502_i64) as u64;
_18 = [(-932674179753475070_i64),1213730907744249792_i64,1765429440582807276_i64,(-6547233129458695717_i64),308512869199299559_i64,7789547552987242658_i64,(-9200718132128506975_i64)];
_3 = [_12,_12,_25,_12,_25];
_28 = _24 >> _20;
_5 = _1 * _1;
_26 = [_25,_2,_12,_12,_25];
RET = _20 ^ _20;
_26 = [_25,_11,_12,_2,_2];
_2 = !_11;
_28 = -_24;
Goto(bb13)
}
bb13 = {
Call(_32 = dump_var(8_usize, 18_usize, Move(_18), 11_usize, Move(_11), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_32 = dump_var(8_usize, 6_usize, Move(_6), 26_usize, Move(_26), 10_usize, Move(_10), 23_usize, Move(_23)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_32 = dump_var(8_usize, 28_usize, Move(_28), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: f32,mut _3: f32,mut _4: [bool; 5],mut _5: [bool; 5],mut _6: u64,mut _7: [bool; 5],mut _8: f32,mut _9: [bool; 5],mut _10: bool,mut _11: [bool; 5]) -> bool {
mir! {
type RET = bool;
let _12: f64;
let _13: ();
let _14: ();
{
RET = !_10;
_9 = [_1,_1,_10,_1,RET];
_7 = [_10,_1,RET,RET,_1];
_11 = _5;
_6 = 18180292400007295391_u64 ^ 5394043436427968008_u64;
_11 = [_10,_10,_10,_10,RET];
_5 = _11;
_12 = 23_u8 as f64;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(9_usize, 6_usize, Move(_6), 1_usize, Move(_1), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [bool; 5],mut _2: bool,mut _3: bool,mut _4: [bool; 5],mut _5: f32,mut _6: f32,mut _7: bool,mut _8: i32,mut _9: bool,mut _10: bool,mut _11: f32,mut _12: [bool; 5]) -> u64 {
mir! {
type RET = u64;
let _13: i8;
let _14: bool;
let _15: [u32; 1];
let _16: bool;
let _17: [usize; 7];
let _18: [i8; 5];
let _19: usize;
let _20: u64;
let _21: Adt59;
let _22: (bool, *const i128, u128, *const i128, u8);
let _23: bool;
let _24: [i64; 7];
let _25: Adt56;
let _26: usize;
let _27: isize;
let _28: *mut u128;
let _29: bool;
let _30: *mut char;
let _31: *const i128;
let _32: Adt55;
let _33: ();
let _34: ();
{
_5 = _6;
Goto(bb1)
}
bb1 = {
_5 = _11 - _11;
_7 = _9 >= _9;
_11 = 5_usize as f32;
_5 = _6;
RET = 8353350620646825831_u64 >> _8;
_1 = [_3,_3,_2,_10,_3];
RET = 3688950860696584873_u64;
RET = 8563492886047306638_u64 * 9531251288597074229_u64;
_13 = !4_i8;
RET = !11326353626939246092_u64;
_13 = 116326043527423349811056557040769661773_i128 as i8;
_14 = _9;
_11 = 152_u8 as f32;
_13 = 15693701627272436620_usize as i8;
_9 = _14;
_6 = 2642318011_u32 as f32;
_14 = !_9;
_10 = _3 < _3;
_5 = -_6;
_7 = !_3;
Call(_5 = fn11(_14, _12, _2, _1, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _3;
_9 = _2;
_10 = !_14;
_12 = _1;
_2 = _3 ^ _14;
_15 = [992099659_u32];
_13 = (-127_i8);
_5 = _8 as f32;
_9 = _10;
_1 = [_2,_14,_9,_7,_7];
_9 = !_7;
_9 = _3 > _2;
_15 = [2506131905_u32];
_14 = _9;
_16 = !_10;
_6 = -_5;
_4 = _1;
RET = 3836965087250135715_u64 >> _8;
Call(_19 = core::intrinsics::bswap(1364176359954138208_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_22.4 = 44_u8;
_4 = [_10,_10,_10,_16,_14];
_23 = !_16;
_3 = _16 <= _16;
_17 = [9115337010666404977_usize,8341343126030304335_usize,2_usize,3_usize,2268964937617375328_usize,15574313007293454836_usize,2_usize];
_22.0 = _14 > _7;
_23 = _3;
_22.0 = !_23;
RET = 3512890745671241351_u64;
_16 = !_2;
_16 = _7 < _14;
_3 = _14 != _9;
_9 = !_2;
_2 = _23;
_19 = 1815704665_u32 as usize;
_24 = [4160556835877531234_i64,3959733752669088797_i64,(-3459728009676983795_i64),5537188565955121800_i64,7509237657615649260_i64,(-5463527699099565029_i64),(-1155809226706669542_i64)];
_24 = [(-962205265271303320_i64),3557626025397897600_i64,595062247218119082_i64,1308194931352665904_i64,6995080364750281747_i64,259238317090639980_i64,2261421955197638734_i64];
_22.2 = RET as u128;
RET = 129561699171731329_u64;
RET = 15389854222918805879_u64;
match _22.4 {
0 => bb4,
1 => bb5,
2 => bb6,
44 => bb8,
_ => bb7
}
}
bb4 = {
_7 = _3;
_9 = _2;
_10 = !_14;
_12 = _1;
_2 = _3 ^ _14;
_15 = [992099659_u32];
_13 = (-127_i8);
_5 = _8 as f32;
_9 = _10;
_1 = [_2,_14,_9,_7,_7];
_9 = !_7;
_9 = _3 > _2;
_15 = [2506131905_u32];
_14 = _9;
_16 = !_10;
_6 = -_5;
_4 = _1;
RET = 3836965087250135715_u64 >> _8;
Call(_19 = core::intrinsics::bswap(1364176359954138208_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = _11 - _11;
_7 = _9 >= _9;
_11 = 5_usize as f32;
_5 = _6;
RET = 8353350620646825831_u64 >> _8;
_1 = [_3,_3,_2,_10,_3];
RET = 3688950860696584873_u64;
RET = 8563492886047306638_u64 * 9531251288597074229_u64;
_13 = !4_i8;
RET = !11326353626939246092_u64;
_13 = 116326043527423349811056557040769661773_i128 as i8;
_14 = _9;
_11 = 152_u8 as f32;
_13 = 15693701627272436620_usize as i8;
_9 = _14;
_6 = 2642318011_u32 as f32;
_14 = !_9;
_10 = _3 < _3;
_5 = -_6;
_7 = !_3;
Call(_5 = fn11(_14, _12, _2, _1, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_24 = [(-3259219813449062549_i64),1215589202438047075_i64,7486930913381466200_i64,(-5889918872369793171_i64),4807883217397590531_i64,(-1942590798380025833_i64),(-8038121800187399263_i64)];
_20 = RET;
match _22.4 {
44 => bb10,
_ => bb9
}
}
bb9 = {
_5 = _11 - _11;
_7 = _9 >= _9;
_11 = 5_usize as f32;
_5 = _6;
RET = 8353350620646825831_u64 >> _8;
_1 = [_3,_3,_2,_10,_3];
RET = 3688950860696584873_u64;
RET = 8563492886047306638_u64 * 9531251288597074229_u64;
_13 = !4_i8;
RET = !11326353626939246092_u64;
_13 = 116326043527423349811056557040769661773_i128 as i8;
_14 = _9;
_11 = 152_u8 as f32;
_13 = 15693701627272436620_usize as i8;
_9 = _14;
_6 = 2642318011_u32 as f32;
_14 = !_9;
_10 = _3 < _3;
_5 = -_6;
_7 = !_3;
Call(_5 = fn11(_14, _12, _2, _1, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_19 = 11048898550085227806_usize;
_3 = !_10;
Goto(bb11)
}
bb11 = {
_17 = [_19,_19,_19,_19,_19,_19,_19];
_4 = [_16,_10,_9,_22.0,_9];
_6 = _5;
_14 = _3 == _2;
_24 = [1297898945156413228_i64,6871213143844552438_i64,(-4060658555880649383_i64),(-3075682886959297591_i64),6350459770771667206_i64,(-2264526892253879805_i64),1501566157074960117_i64];
RET = !_20;
_22.4 = 126_u8;
_15 = [1455423955_u32];
_27 = !55_isize;
_19 = !16427133626932711958_usize;
_18 = [_13,_13,_13,_13,_13];
_22.2 = 297442854774711524890126576543809539250_u128 | 136711632375468110756356494015544468124_u128;
_22.0 = !_14;
_10 = !_22.0;
_5 = -_11;
_10 = _14;
_22.4 = _22.2 as u8;
_1 = [_14,_3,_23,_2,_14];
_23 = _2 < _16;
_5 = _6;
Goto(bb12)
}
bb12 = {
_9 = !_7;
_26 = _19;
_4 = [_10,_2,_7,_23,_14];
_18 = [_13,_13,_13,_13,_13];
_13 = -57_i8;
_2 = !_3;
_4 = _1;
_24 = [6375232650854823677_i64,(-7602190902806466929_i64),(-4429628836197471876_i64),(-1491539539555739281_i64),(-1071551025226117235_i64),518911147880760612_i64,5971111570438551880_i64];
_29 = _8 > _8;
RET = _20 | _20;
_1 = [_9,_3,_9,_16,_2];
match _20 {
0 => bb13,
15389854222918805879 => bb15,
_ => bb14
}
}
bb13 = {
_5 = _11 - _11;
_7 = _9 >= _9;
_11 = 5_usize as f32;
_5 = _6;
RET = 8353350620646825831_u64 >> _8;
_1 = [_3,_3,_2,_10,_3];
RET = 3688950860696584873_u64;
RET = 8563492886047306638_u64 * 9531251288597074229_u64;
_13 = !4_i8;
RET = !11326353626939246092_u64;
_13 = 116326043527423349811056557040769661773_i128 as i8;
_14 = _9;
_11 = 152_u8 as f32;
_13 = 15693701627272436620_usize as i8;
_9 = _14;
_6 = 2642318011_u32 as f32;
_14 = !_9;
_10 = _3 < _3;
_5 = -_6;
_7 = !_3;
Call(_5 = fn11(_14, _12, _2, _1, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_19 = 11048898550085227806_usize;
_3 = !_10;
Goto(bb11)
}
bb15 = {
_22.4 = _8 as u8;
_18 = [_13,_13,_13,_13,_13];
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(10_usize, 9_usize, Move(_9), 27_usize, Move(_27), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(10_usize, 1_usize, Move(_1), 23_usize, Move(_23), 29_usize, Move(_29), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(10_usize, 16_usize, Move(_16), 7_usize, Move(_7), 14_usize, Move(_14), 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: bool,mut _2: [bool; 5],mut _3: bool,mut _4: [bool; 5],mut _5: bool) -> f32 {
mir! {
type RET = f32;
let _6: bool;
let _7: i8;
let _8: u16;
let _9: Adt45;
let _10: *const *mut u128;
let _11: i64;
let _12: [bool; 5];
let _13: *mut u128;
let _14: bool;
let _15: char;
let _16: i64;
let _17: [i128; 5];
let _18: f64;
let _19: (f64, i8, f32);
let _20: [u128; 1];
let _21: f64;
let _22: f32;
let _23: usize;
let _24: ();
let _25: ();
{
_3 = !_1;
RET = 43384_u16 as f32;
_2 = _4;
_4 = [_1,_1,_1,_5,_5];
_6 = _3 | _5;
RET = 3_usize as f32;
_3 = _6 != _6;
_3 = _1;
_5 = !_1;
_1 = _3;
RET = (-9223372036854775808_isize) as f32;
RET = 21994_i16 as f32;
_4 = _2;
RET = 22_i8 as f32;
_1 = !_6;
_3 = !_1;
_2 = _4;
_6 = _1;
_1 = _3;
_1 = _6;
_1 = !_6;
Goto(bb1)
}
bb1 = {
_6 = _5;
_4 = [_1,_5,_5,_6,_3];
RET = 7992614561880926132_i64 as f32;
_4 = _2;
_3 = !_6;
_3 = !_5;
_1 = !_6;
RET = (-3028426071154707331_i64) as f32;
_3 = _1 & _6;
RET = 4_usize as f32;
RET = 497106135_i32 as f32;
_1 = _6 <= _3;
_5 = _1 != _6;
_12 = [_3,_3,_6,_6,_5];
_5 = !_3;
_11 = (-282069768805358172_i64) * (-1740037370322041307_i64);
_6 = !_1;
_8 = !63297_u16;
_3 = _1;
RET = _11 as f32;
_1 = !_6;
_3 = _6;
_8 = 438875416_u32 as u16;
Call(_7 = fn12(_12, _3, _2, _6, _1, _3, _2, _2, RET, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [_1,_6,_1,_5,_6];
Goto(bb3)
}
bb3 = {
_14 = _3;
_1 = _14;
_9 = Adt45::Variant0 { fld0: 1019551903_i32 };
_16 = !_11;
_8 = 42100_u16;
RET = _7 as f32;
_3 = _14 < _1;
_12 = [_3,_6,_3,_5,_3];
place!(Field::<i32>(Variant(_9, 0), 0)) = 1289532342_i32 & (-622908199_i32);
_12 = _2;
_15 = '\u{1f62b}';
place!(Field::<i32>(Variant(_9, 0), 0)) = 1921131918_i32;
_17 = [(-99089233640770604337835024260533633045_i128),(-822645177247935849865557120324444666_i128),(-13279449980786751114282550787109432216_i128),(-126838252067741193949766454088356341009_i128),(-74555308832452033342840339571734845695_i128)];
place!(Field::<i32>(Variant(_9, 0), 0)) = _11 as i32;
_11 = _16;
_12 = [_1,_6,_6,_14,_3];
_12 = [_1,_5,_6,_14,_5];
place!(Field::<i32>(Variant(_9, 0), 0)) = (-2080920232_i32);
_5 = _3;
_11 = _16 * _16;
_14 = _3 & _5;
_17 = [135715991920152710556522050685314539133_i128,51871075906478811059682854305481131400_i128,(-131287353138080000070099610419305962153_i128),(-45568493087333846108782496505073908541_i128),60460834090698770298815323678957993741_i128];
_16 = _11;
Goto(bb4)
}
bb4 = {
_2 = [_6,_1,_14,_5,_1];
_12 = _4;
_18 = _7 as f64;
SetDiscriminant(_9, 0);
_3 = _14;
_1 = !_3;
RET = _7 as f32;
_2 = _12;
_10 = core::ptr::addr_of!(_13);
_4 = [_5,_14,_14,_6,_1];
_7 = 78_i8;
_18 = 159466097134986528210395696335507733178_u128 as f64;
_2 = [_1,_1,_14,_1,_5];
_14 = !_6;
_8 = 40903_u16;
_1 = !_6;
_3 = !_6;
_18 = (-3814_i16) as f64;
_8 = 1159692902_i32 as u16;
_19.1 = 16202_i16 as i8;
_2 = [_1,_1,_1,_1,_14];
_5 = _6;
match _7 {
0 => bb3,
78 => bb6,
_ => bb5
}
}
bb5 = {
_6 = _5;
_4 = [_1,_5,_5,_6,_3];
RET = 7992614561880926132_i64 as f32;
_4 = _2;
_3 = !_6;
_3 = !_5;
_1 = !_6;
RET = (-3028426071154707331_i64) as f32;
_3 = _1 & _6;
RET = 4_usize as f32;
RET = 497106135_i32 as f32;
_1 = _6 <= _3;
_5 = _1 != _6;
_12 = [_3,_3,_6,_6,_5];
_5 = !_3;
_11 = (-282069768805358172_i64) * (-1740037370322041307_i64);
_6 = !_1;
_8 = !63297_u16;
_3 = _1;
RET = _11 as f32;
_1 = !_6;
_3 = _6;
_8 = 438875416_u32 as u16;
Call(_7 = fn12(_12, _3, _2, _6, _1, _3, _2, _2, RET, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_9 = Adt45::Variant0 { fld0: (-1764176215_i32) };
_19.1 = _7;
_15 = '\u{c73da}';
_1 = !_3;
_11 = 554476518_i32 as i64;
match _19.1 {
0 => bb5,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
78 => bb14,
_ => bb13
}
}
bb7 = {
_6 = _5;
_4 = [_1,_5,_5,_6,_3];
RET = 7992614561880926132_i64 as f32;
_4 = _2;
_3 = !_6;
_3 = !_5;
_1 = !_6;
RET = (-3028426071154707331_i64) as f32;
_3 = _1 & _6;
RET = 4_usize as f32;
RET = 497106135_i32 as f32;
_1 = _6 <= _3;
_5 = _1 != _6;
_12 = [_3,_3,_6,_6,_5];
_5 = !_3;
_11 = (-282069768805358172_i64) * (-1740037370322041307_i64);
_6 = !_1;
_8 = !63297_u16;
_3 = _1;
RET = _11 as f32;
_1 = !_6;
_3 = _6;
_8 = 438875416_u32 as u16;
Call(_7 = fn12(_12, _3, _2, _6, _1, _3, _2, _2, RET, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_2 = [_6,_1,_14,_5,_1];
_12 = _4;
_18 = _7 as f64;
SetDiscriminant(_9, 0);
_3 = _14;
_1 = !_3;
RET = _7 as f32;
_2 = _12;
_10 = core::ptr::addr_of!(_13);
_4 = [_5,_14,_14,_6,_1];
_7 = 78_i8;
_18 = 159466097134986528210395696335507733178_u128 as f64;
_2 = [_1,_1,_14,_1,_5];
_14 = !_6;
_8 = 40903_u16;
_1 = !_6;
_3 = !_6;
_18 = (-3814_i16) as f64;
_8 = 1159692902_i32 as u16;
_19.1 = 16202_i16 as i8;
_2 = [_1,_1,_1,_1,_14];
_5 = _6;
match _7 {
0 => bb3,
78 => bb6,
_ => bb5
}
}
bb9 = {
_14 = _3;
_1 = _14;
_9 = Adt45::Variant0 { fld0: 1019551903_i32 };
_16 = !_11;
_8 = 42100_u16;
RET = _7 as f32;
_3 = _14 < _1;
_12 = [_3,_6,_3,_5,_3];
place!(Field::<i32>(Variant(_9, 0), 0)) = 1289532342_i32 & (-622908199_i32);
_12 = _2;
_15 = '\u{1f62b}';
place!(Field::<i32>(Variant(_9, 0), 0)) = 1921131918_i32;
_17 = [(-99089233640770604337835024260533633045_i128),(-822645177247935849865557120324444666_i128),(-13279449980786751114282550787109432216_i128),(-126838252067741193949766454088356341009_i128),(-74555308832452033342840339571734845695_i128)];
place!(Field::<i32>(Variant(_9, 0), 0)) = _11 as i32;
_11 = _16;
_12 = [_1,_6,_6,_14,_3];
_12 = [_1,_5,_6,_14,_5];
place!(Field::<i32>(Variant(_9, 0), 0)) = (-2080920232_i32);
_5 = _3;
_11 = _16 * _16;
_14 = _3 & _5;
_17 = [135715991920152710556522050685314539133_i128,51871075906478811059682854305481131400_i128,(-131287353138080000070099610419305962153_i128),(-45568493087333846108782496505073908541_i128),60460834090698770298815323678957993741_i128];
_16 = _11;
Goto(bb4)
}
bb10 = {
_2 = [_1,_6,_1,_5,_6];
Goto(bb3)
}
bb11 = {
_6 = _5;
_4 = [_1,_5,_5,_6,_3];
RET = 7992614561880926132_i64 as f32;
_4 = _2;
_3 = !_6;
_3 = !_5;
_1 = !_6;
RET = (-3028426071154707331_i64) as f32;
_3 = _1 & _6;
RET = 4_usize as f32;
RET = 497106135_i32 as f32;
_1 = _6 <= _3;
_5 = _1 != _6;
_12 = [_3,_3,_6,_6,_5];
_5 = !_3;
_11 = (-282069768805358172_i64) * (-1740037370322041307_i64);
_6 = !_1;
_8 = !63297_u16;
_3 = _1;
RET = _11 as f32;
_1 = !_6;
_3 = _6;
_8 = 438875416_u32 as u16;
Call(_7 = fn12(_12, _3, _2, _6, _1, _3, _2, _2, RET, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_20 = [292954475740666932196770318983519881287_u128];
_17 = [(-9931753220364537426474387660387576833_i128),27657217296922604766082896933346904652_i128,45568976281492768448332483672337131783_i128,(-25814873420739134454826242380208649475_i128),(-142033664998960501840362252702386646234_i128)];
_6 = !_1;
_1 = _6 != _6;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(11_usize, 15_usize, Move(_15), 7_usize, Move(_7), 16_usize, Move(_16), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(11_usize, 12_usize, Move(_12), 2_usize, Move(_2), 11_usize, Move(_11), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [bool; 5],mut _2: bool,mut _3: [bool; 5],mut _4: bool,mut _5: bool,mut _6: bool,mut _7: [bool; 5],mut _8: [bool; 5],mut _9: f32,mut _10: bool) -> i8 {
mir! {
type RET = i8;
let _11: i8;
let _12: isize;
let _13: [i64; 7];
let _14: Adt50;
let _15: isize;
let _16: f64;
let _17: [i64; 7];
let _18: [i64; 7];
let _19: isize;
let _20: u32;
let _21: char;
let _22: [char; 6];
let _23: (i8, [i64; 7]);
let _24: char;
let _25: i32;
let _26: bool;
let _27: i128;
let _28: i8;
let _29: *const *mut u128;
let _30: ();
let _31: ();
{
_2 = _4;
RET = 97_i8 | 32_i8;
_6 = !_4;
RET = (-18_i8);
_3 = [_4,_2,_6,_5,_2];
_8 = _3;
_4 = _10 != _5;
_4 = _5 | _6;
_1 = _3;
_4 = _10 > _10;
_11 = !RET;
_5 = _4;
_10 = _2;
_1 = [_6,_10,_6,_10,_5];
_9 = 9223372036854775807_isize as f32;
RET = _11 << _11;
_7 = [_2,_2,_6,_6,_4];
_8 = [_10,_6,_6,_10,_2];
RET = 5073547_u32 as i8;
RET = _11 << _11;
_5 = _2;
RET = _11 - _11;
Call(_8 = fn13(_7, _6, _10, _3, _6, _2, _2, _5, _3, _5, _6, _10, _4, _4, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [_5,_4,_6,_5,_6];
_10 = _4;
RET = 3109509022082262044_usize as i8;
RET = _11 - _11;
_1 = _7;
_3 = [_5,_5,_4,_10,_10];
Goto(bb2)
}
bb2 = {
_12 = _9 as isize;
_8 = [_6,_2,_6,_2,_4];
_9 = _11 as f32;
_3 = [_2,_6,_4,_10,_4];
_5 = _6 == _4;
_11 = RET + RET;
_11 = RET;
_1 = _7;
Goto(bb3)
}
bb3 = {
_2 = !_6;
_9 = 106_u8 as f32;
_9 = (-7508_i16) as f32;
Goto(bb4)
}
bb4 = {
_9 = (-17527_i16) as f32;
_3 = [_5,_5,_10,_2,_10];
_1 = _7;
_2 = _6;
_8 = _1;
_4 = !_6;
_3 = _1;
_2 = _6;
_15 = _12 * _12;
_7 = [_2,_10,_2,_2,_5];
_1 = [_6,_6,_10,_5,_4];
_13 = [(-1931944841616939309_i64),8579808640377882420_i64,6171455957143989013_i64,(-8513845345764289534_i64),3887775550404637017_i64,3245803086527556932_i64,1377258791362433420_i64];
_1 = [_6,_6,_2,_10,_6];
_12 = _15;
_13 = [(-7472879731016036413_i64),8953414074050571510_i64,(-8367121054516944364_i64),6694668501073459102_i64,4211298026982105887_i64,(-1850681018083847344_i64),5362670883796014559_i64];
_16 = 47_u8 as f64;
_4 = _12 < _15;
_15 = _9 as isize;
_9 = _16 as f32;
_11 = RET;
Goto(bb5)
}
bb5 = {
_7 = [_6,_10,_10,_5,_2];
_3 = [_5,_6,_5,_10,_6];
_15 = _12;
_8 = [_5,_6,_6,_10,_10];
_18 = [5974874668173760029_i64,(-4012927782159623173_i64),(-4355836401075011568_i64),(-8130983732234608132_i64),4439575453762447709_i64,9049180148606646110_i64,(-6712388127127225771_i64)];
_18 = _13;
_15 = 37689_u16 as isize;
_2 = _6 ^ _6;
_16 = 6813724800396129633_i64 as f64;
_11 = 1596801143_i32 as i8;
_13 = [6254718431031627067_i64,(-1355149487721903576_i64),(-7337320865582095510_i64),8742787074382299412_i64,1941864351972862395_i64,1237818039169149127_i64,407511995249662038_i64];
_17 = _18;
Goto(bb6)
}
bb6 = {
_10 = !_5;
_8 = _7;
_10 = _5;
_18 = [7530399981552871051_i64,(-3855530404670289724_i64),(-6500787289266541973_i64),727143551145861774_i64,270346397328007128_i64,(-6777978936431649015_i64),(-1536511110890923120_i64)];
_12 = !_15;
_2 = _5;
_2 = !_5;
_18 = [6901887215431285553_i64,2348465866550743034_i64,4344962462807652614_i64,174940434560880541_i64,4760144702005958038_i64,(-8796328900745207855_i64),183644157923745249_i64];
_3 = [_2,_2,_6,_2,_10];
_17 = [(-5244805009457772640_i64),3882085003749003625_i64,(-638884220585007562_i64),5191783137265267796_i64,(-7588254512154015016_i64),6380185202681716442_i64,4420458237319041553_i64];
_16 = (-12382_i16) as f64;
_2 = _10 ^ _10;
_13 = _17;
_1 = [_5,_6,_6,_4,_2];
_15 = _9 as isize;
_20 = _9 as u32;
_3 = _1;
_21 = '\u{2d3dc}';
RET = -_11;
RET = 168_u8 as i8;
_7 = [_2,_10,_5,_6,_2];
_21 = '\u{7d457}';
_11 = 245_u8 as i8;
_19 = _15;
_1 = [_5,_5,_6,_6,_6];
Call(_15 = fn14(_1, _7, _6, _1, _1, _7, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = !_6;
Goto(bb8)
}
bb8 = {
_2 = _6;
_8 = [_10,_10,_6,_10,_2];
_10 = !_6;
_4 = _5 == _10;
_7 = _8;
_7 = _1;
_16 = (-872192838_i32) as f64;
_8 = [_2,_2,_5,_4,_2];
_1 = [_2,_10,_2,_5,_5];
RET = _11 | _11;
_9 = _20 as f32;
RET = -_11;
_1 = _8;
_5 = _10 == _6;
_3 = _1;
_22 = [_21,_21,_21,_21,_21,_21];
Goto(bb9)
}
bb9 = {
_11 = RET * RET;
_6 = _2 == _5;
_11 = -RET;
_22 = [_21,_21,_21,_21,_21,_21];
_16 = (-31230598280883878111725849319544974495_i128) as f64;
_15 = _12 | _19;
_7 = [_5,_2,_10,_5,_4];
_12 = -_15;
_9 = (-151375284516905821087856687995005225744_i128) as f32;
_5 = !_6;
_17 = _13;
RET = 21725809444233522055493977826691761731_u128 as i8;
_12 = _19;
Call(_12 = core::intrinsics::transmute(_15), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8 = _1;
Goto(bb11)
}
bb11 = {
_22 = [_21,_21,_21,_21,_21,_21];
_10 = _6;
_23 = (_11, _17);
_1 = [_10,_5,_6,_6,_2];
_6 = !_10;
_7 = [_10,_10,_2,_10,_2];
_25 = 9007_i16 as i32;
Goto(bb12)
}
bb12 = {
_13 = _18;
_11 = _21 as i8;
_6 = !_10;
_3 = [_2,_5,_10,_10,_10];
_25 = (-1291215581_i32) | 1334592975_i32;
_6 = _4 == _5;
_5 = _4;
RET = _11 >> _19;
_23.1 = [(-3271543573955727946_i64),(-3999089115667398772_i64),1778789156358936550_i64,(-4132540072630363984_i64),(-7510695017279786275_i64),3906851111605663163_i64,(-3270840082535927624_i64)];
_2 = _10;
_22 = [_21,_21,_21,_21,_21,_21];
_15 = !_12;
_11 = RET;
_6 = _2;
RET = 12497257874122479473_u64 as i8;
_5 = !_6;
_24 = _21;
_12 = !_15;
_28 = RET >> _11;
_8 = [_4,_4,_2,_2,_5];
Goto(bb13)
}
bb13 = {
_23.1 = [7132881302052144799_i64,(-8335764691692071894_i64),3812569952387810911_i64,(-9192089382338952399_i64),7186843402396585684_i64,4133322606511862140_i64,559633359234246550_i64];
_6 = _10;
_12 = _19 ^ _15;
Goto(bb14)
}
bb14 = {
RET = _23.0;
_22 = [_24,_24,_21,_21,_24,_21];
_19 = -_12;
_1 = [_10,_10,_5,_2,_6];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(12_usize, 20_usize, Move(_20), 23_usize, Move(_23), 17_usize, Move(_17), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(12_usize, 24_usize, Move(_24), 22_usize, Move(_22), 2_usize, Move(_2), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(12_usize, 28_usize, Move(_28), 21_usize, Move(_21), 12_usize, Move(_12), 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [bool; 5],mut _2: bool,mut _3: bool,mut _4: [bool; 5],mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: [bool; 5],mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _16: [i128; 5];
let _17: ();
let _18: ();
{
_11 = _6;
_1 = _9;
RET = _9;
_4 = [_2,_7,_3,_6,_3];
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(13_usize, 8_usize, Move(_8), 14_usize, Move(_14), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(13_usize, 15_usize, Move(_15), 3_usize, Move(_3), 1_usize, Move(_1), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [bool; 5],mut _2: [bool; 5],mut _3: bool,mut _4: [bool; 5],mut _5: [bool; 5],mut _6: [bool; 5],mut _7: bool) -> isize {
mir! {
type RET = isize;
let _8: *mut char;
let _9: bool;
let _10: Adt45;
let _11: [bool; 5];
let _12: (f64, i8, f32);
let _13: [i64; 7];
let _14: char;
let _15: Adt44;
let _16: f32;
let _17: [u128; 1];
let _18: u16;
let _19: *const *const i128;
let _20: u8;
let _21: [u64; 2];
let _22: isize;
let _23: Adt45;
let _24: [char; 6];
let _25: i16;
let _26: *mut *const i128;
let _27: i32;
let _28: Adt46;
let _29: bool;
let _30: Adt60;
let _31: [usize; 7];
let _32: [bool; 5];
let _33: i8;
let _34: Adt50;
let _35: char;
let _36: isize;
let _37: u16;
let _38: [u64; 2];
let _39: u16;
let _40: u64;
let _41: [char; 6];
let _42: [i64; 7];
let _43: [bool; 5];
let _44: char;
let _45: ();
let _46: ();
{
_1 = [_7,_7,_3,_3,_3];
RET = 2134_u16 as isize;
_6 = [_3,_7,_3,_3,_3];
_5 = _1;
_6 = _2;
_9 = _7;
_3 = _9;
_4 = [_7,_9,_3,_3,_7];
_7 = _9;
RET = -38_isize;
_4 = [_9,_9,_9,_3,_3];
_1 = [_7,_9,_9,_7,_7];
_6 = [_7,_7,_7,_9,_7];
_8 = core::ptr::addr_of_mut!(_14);
_2 = _1;
Goto(bb1)
}
bb1 = {
_12.1 = 1247785423_u32 as i8;
(*_8) = '\u{e51fc}';
RET = 9223372036854775807_isize;
_11 = [_9,_3,_7,_9,_3];
_7 = _9;
_2 = [_9,_3,_9,_9,_3];
_14 = '\u{10caa3}';
_8 = core::ptr::addr_of_mut!((*_8));
_10 = Adt45::Variant0 { fld0: (-2086823841_i32) };
_5 = [_9,_7,_7,_9,_9];
_9 = !_3;
(*_8) = '\u{59ab6}';
_13 = [(-3595458172257788583_i64),(-4031259017597823742_i64),2444735687823270898_i64,(-1600022722938012384_i64),4008529114193494304_i64,7546008639490373718_i64,1297057273955711751_i64];
_17 = [197492099574450092605632436979293152834_u128];
_12.2 = 1975718896_u32 as f32;
Call(RET = fn15(_11, _9, _4, _2, _1, _1, _3, _3, _11, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.0 = 35886108332324523188368183919974561568_u128 as f64;
_11 = [_7,_7,_3,_7,_7];
_12.2 = 1064790135833032110_i64 as f32;
_16 = -_12.2;
_14 = '\u{a992f}';
place!(Field::<i32>(Variant(_10, 0), 0)) = RET as i32;
(*_8) = '\u{4a14f}';
_10 = Adt45::Variant0 { fld0: 1801353818_i32 };
_10 = Adt45::Variant0 { fld0: 1855474268_i32 };
_8 = core::ptr::addr_of_mut!(_14);
(*_8) = '\u{8b83c}';
(*_8) = '\u{cd859}';
(*_8) = '\u{9eed8}';
RET = 21413_u16 as isize;
(*_8) = '\u{7143b}';
_13 = [2797216922879054949_i64,(-4360548632899294998_i64),3103180173089139550_i64,(-4206534742480423840_i64),995725020469923287_i64,(-6767197514398083950_i64),(-2231742516633624878_i64)];
_8 = core::ptr::addr_of_mut!((*_8));
_11 = [_7,_9,_7,_3,_3];
(*_8) = '\u{100e3d}';
_9 = _7 == _7;
_1 = [_7,_9,_7,_9,_9];
place!(Field::<i32>(Variant(_10, 0), 0)) = -(-460991603_i32);
_20 = 56_u8 | 78_u8;
Goto(bb3)
}
bb3 = {
_13 = [(-5857611135556614010_i64),(-1863370414560351364_i64),(-6306914787881399062_i64),6623316357256711200_i64,2839516301733703345_i64,(-1336206457982887438_i64),(-5409190312386929221_i64)];
_4 = _11;
_7 = _9;
_12.1 = 1_usize as i8;
(*_8) = '\u{bba0d}';
(*_8) = '\u{b05ca}';
_23 = Move(_10);
_8 = core::ptr::addr_of_mut!((*_8));
RET = (-55_isize);
_16 = _12.0 as f32;
_14 = '\u{7d4d1}';
_18 = (*_8) as u16;
_22 = RET & RET;
Goto(bb4)
}
bb4 = {
_20 = 16_u8;
_3 = _7 & _9;
_22 = _12.1 as isize;
_22 = 2330984558797031924_i64 as isize;
_21 = [11919669061218715381_u64,7365509978020474643_u64];
_18 = !34671_u16;
_17 = [153644461438755575513739167736816091832_u128];
_16 = 227662032342871559794509805242777760090_u128 as f32;
_13 = [9033934420997788607_i64,7856207567584033063_i64,8448288995420605920_i64,(-4994014319178021195_i64),(-6450961890343186215_i64),6687779625799821510_i64,9191195241055914819_i64];
_2 = [_3,_3,_3,_9,_3];
_27 = Field::<i32>(Variant(_23, 0), 0) >> _18;
_25 = 16218_i16;
(*_8) = '\u{e87a8}';
_14 = '\u{a3dc9}';
_4 = [_9,_3,_9,_9,_7];
_20 = !146_u8;
_8 = core::ptr::addr_of_mut!((*_8));
_1 = [_7,_3,_3,_3,_9];
_5 = [_3,_9,_7,_3,_9];
_6 = _11;
_13 = [(-2423182424806437650_i64),(-5969140083942354831_i64),(-3561936931382823928_i64),4387117491299433464_i64,(-1563258077800079533_i64),(-8701731161210149960_i64),(-3582564716790786232_i64)];
_10 = Adt45::Variant0 { fld0: Field::<i32>(Variant(_23, 0), 0) };
_17 = [87022026084533514721517120937993239938_u128];
_24 = [_14,(*_8),(*_8),_14,(*_8),_14];
_28 = Adt46::Variant1 { fld0: _13 };
Goto(bb5)
}
bb5 = {
_7 = _3;
_14 = '\u{d701}';
_12.0 = _22 as f64;
_13 = Field::<[i64; 7]>(Variant(_28, 1), 0);
_12.1 = (-8729930423442163624_i64) as i8;
RET = -_22;
place!(Field::<[i64; 7]>(Variant(_28, 1), 0)) = [(-1682079693462766394_i64),9156812488930528393_i64,(-4069857155677132015_i64),(-7826649212977240261_i64),8280233483934321718_i64,(-5222522819374047922_i64),(-7254803012543673563_i64)];
_22 = _18 as isize;
_16 = 816431043_u32 as f32;
_18 = 7403_u16 << Field::<i32>(Variant(_23, 0), 0);
_9 = _7 & _7;
_2 = [_7,_9,_3,_3,_3];
(*_8) = '\u{46c02}';
_20 = 167_u8 - 49_u8;
RET = _22;
_14 = '\u{55ec1}';
_29 = !_9;
_29 = _9;
_18 = !8745_u16;
_32 = [_29,_9,_7,_3,_29];
_29 = !_9;
_12.1 = _16 as i8;
match _25 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
4 => bb7,
16218 => bb9,
_ => bb8
}
}
bb6 = {
_20 = 16_u8;
_3 = _7 & _9;
_22 = _12.1 as isize;
_22 = 2330984558797031924_i64 as isize;
_21 = [11919669061218715381_u64,7365509978020474643_u64];
_18 = !34671_u16;
_17 = [153644461438755575513739167736816091832_u128];
_16 = 227662032342871559794509805242777760090_u128 as f32;
_13 = [9033934420997788607_i64,7856207567584033063_i64,8448288995420605920_i64,(-4994014319178021195_i64),(-6450961890343186215_i64),6687779625799821510_i64,9191195241055914819_i64];
_2 = [_3,_3,_3,_9,_3];
_27 = Field::<i32>(Variant(_23, 0), 0) >> _18;
_25 = 16218_i16;
(*_8) = '\u{e87a8}';
_14 = '\u{a3dc9}';
_4 = [_9,_3,_9,_9,_7];
_20 = !146_u8;
_8 = core::ptr::addr_of_mut!((*_8));
_1 = [_7,_3,_3,_3,_9];
_5 = [_3,_9,_7,_3,_9];
_6 = _11;
_13 = [(-2423182424806437650_i64),(-5969140083942354831_i64),(-3561936931382823928_i64),4387117491299433464_i64,(-1563258077800079533_i64),(-8701731161210149960_i64),(-3582564716790786232_i64)];
_10 = Adt45::Variant0 { fld0: Field::<i32>(Variant(_23, 0), 0) };
_17 = [87022026084533514721517120937993239938_u128];
_24 = [_14,(*_8),(*_8),_14,(*_8),_14];
_28 = Adt46::Variant1 { fld0: _13 };
Goto(bb5)
}
bb7 = {
_13 = [(-5857611135556614010_i64),(-1863370414560351364_i64),(-6306914787881399062_i64),6623316357256711200_i64,2839516301733703345_i64,(-1336206457982887438_i64),(-5409190312386929221_i64)];
_4 = _11;
_7 = _9;
_12.1 = 1_usize as i8;
(*_8) = '\u{bba0d}';
(*_8) = '\u{b05ca}';
_23 = Move(_10);
_8 = core::ptr::addr_of_mut!((*_8));
RET = (-55_isize);
_16 = _12.0 as f32;
_14 = '\u{7d4d1}';
_18 = (*_8) as u16;
_22 = RET & RET;
Goto(bb4)
}
bb8 = {
_12.1 = 1247785423_u32 as i8;
(*_8) = '\u{e51fc}';
RET = 9223372036854775807_isize;
_11 = [_9,_3,_7,_9,_3];
_7 = _9;
_2 = [_9,_3,_9,_9,_3];
_14 = '\u{10caa3}';
_8 = core::ptr::addr_of_mut!((*_8));
_10 = Adt45::Variant0 { fld0: (-2086823841_i32) };
_5 = [_9,_7,_7,_9,_9];
_9 = !_3;
(*_8) = '\u{59ab6}';
_13 = [(-3595458172257788583_i64),(-4031259017597823742_i64),2444735687823270898_i64,(-1600022722938012384_i64),4008529114193494304_i64,7546008639490373718_i64,1297057273955711751_i64];
_17 = [197492099574450092605632436979293152834_u128];
_12.2 = 1975718896_u32 as f32;
Call(RET = fn15(_11, _9, _4, _2, _1, _1, _3, _3, _11, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_5 = _6;
_31 = [15408472375390498973_usize,5439934489261034718_usize,15458512956201583652_usize,4044924602694021504_usize,5298998643431736121_usize,5_usize,13074309094448964760_usize];
place!(Field::<i32>(Variant(_23, 0), 0)) = _27;
_16 = _12.2;
_29 = _7;
_4 = _32;
_11 = [_7,_9,_29,_3,_29];
_33 = _12.1 ^ _12.1;
SetDiscriminant(_28, 2);
_24 = [(*_8),(*_8),(*_8),(*_8),(*_8),(*_8)];
_11 = [_29,_7,_29,_7,_7];
_3 = !_9;
RET = _22 * _22;
_33 = _12.1 + _12.1;
_35 = _14;
_35 = _14;
_13 = [2409729549600606395_i64,(-1536912308655476553_i64),4740365010557040402_i64,(-3176858899126007460_i64),3612327344166266503_i64,(-67576022351402011_i64),7087745400102942394_i64];
_5 = _6;
(*_8) = _35;
_3 = _7 == _9;
_3 = !_9;
_36 = _27 as isize;
_13 = [8038099436284749148_i64,(-8185784847283839650_i64),3079394682372520924_i64,(-5586557391551334144_i64),1125938432371280515_i64,7338523558205594150_i64,(-5680088914265613535_i64)];
_14 = _35;
SetDiscriminant(_10, 0);
Goto(bb10)
}
bb10 = {
_16 = _18 as f32;
_13 = [(-7762083589826106863_i64),(-2713154304772284094_i64),(-4350579706095570309_i64),1816928122303534778_i64,(-2055891740224293618_i64),(-1439925811180478243_i64),654850005253782038_i64];
_38 = _21;
_6 = _1;
_5 = [_3,_9,_29,_3,_7];
_6 = [_9,_3,_7,_7,_29];
_2 = [_7,_3,_3,_7,_29];
_24 = [(*_8),_14,(*_8),_14,(*_8),(*_8)];
_21 = [11510639507421315339_u64,4533412790420537874_u64];
_8 = core::ptr::addr_of_mut!(_14);
place!(Field::<i32>(Variant(_10, 0), 0)) = _27;
_12.2 = _16 - _16;
_17 = [9263667116727295712171602874387261994_u128];
_25 = !23511_i16;
place!(Field::<i32>(Variant(_23, 0), 0)) = -_27;
_11 = [_3,_29,_3,_3,_29];
Goto(bb11)
}
bb11 = {
_39 = 10742251007569777721_u64 as u16;
_17 = [56380420682564289677865367086299716743_u128];
_21 = [11867954765707592494_u64,4610814092509457189_u64];
_18 = _20 as u16;
_11 = [_3,_9,_9,_9,_29];
_37 = !_39;
SetDiscriminant(_23, 0);
RET = _36 ^ _36;
_23 = Move(_10);
_27 = _29 as i32;
SetDiscriminant(_23, 1);
_9 = _3 > _7;
place!(Field::<[u32; 1]>(Variant(_23, 1), 2)) = [3219438326_u32];
_10 = Adt45::Variant0 { fld0: _27 };
_40 = !965186488778306342_u64;
Goto(bb12)
}
bb12 = {
_11 = _4;
RET = _36;
_36 = -_22;
_19 = core::ptr::addr_of!(place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_23, 1), 3)).1);
_29 = Field::<i32>(Variant(_10, 0), 0) != _27;
_12.2 = _16 - _16;
Goto(bb13)
}
bb13 = {
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_23, 1), 3)).0 = !_9;
_13 = [(-3562023392510766026_i64),2937459741847001066_i64,(-4533051741824440718_i64),9171280102258198143_i64,(-4786166992120452439_i64),5505081613309232952_i64,3777643714100404586_i64];
_40 = _20 as u64;
_41 = [(*_8),_14,_35,_35,_35,(*_8)];
_11 = [Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_23, 1), 3).0,_29,_9,_3,_3];
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_23, 1), 3)).0 = _3 ^ _9;
_22 = RET + _36;
_13 = [(-2888549536363207627_i64),2592745523701241444_i64,(-1508966745658502425_i64),7601173700129811936_i64,1567593648772106330_i64,6688144079372637064_i64,(-4978899420187913077_i64)];
_13 = [(-5393061865920316338_i64),(-1575235841584415358_i64),(-9170990787483533050_i64),2730067118903368561_i64,(-3354438092545025414_i64),4137264848904153362_i64,8507626833126758333_i64];
_36 = -RET;
place!(Field::<[char; 6]>(Variant(_23, 1), 1)) = [(*_8),_35,(*_8),_14,(*_8),_35];
_21 = [_40,_40];
_14 = _35;
_16 = RET as f32;
_11 = [_3,_3,_9,Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_23, 1), 3).0,Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_23, 1), 3).0];
_6 = _5;
_40 = _33 as u64;
Call(_3 = fn19(_9, _29, _6, _7, _29, _7, _5, _5, Move(_10), _4, _9, _7), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_42 = [5100357397199662410_i64,(-231687342607164901_i64),(-3473350522157844592_i64),(-1742067589439489011_i64),3404129658139934885_i64,(-2265657173935490878_i64),(-7386020157081887569_i64)];
_19 = core::ptr::addr_of!((*_19));
_24 = [(*_8),_35,(*_8),_35,_35,(*_8)];
_39 = !_18;
RET = _20 as isize;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(14_usize, 20_usize, Move(_20), 41_usize, Move(_41), 29_usize, Move(_29), 39_usize, Move(_39)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(14_usize, 1_usize, Move(_1), 18_usize, Move(_18), 27_usize, Move(_27), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(14_usize, 14_usize, Move(_14), 2_usize, Move(_2), 25_usize, Move(_25), 40_usize, Move(_40)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(14_usize, 38_usize, Move(_38), 6_usize, Move(_6), 11_usize, Move(_11), 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [bool; 5],mut _2: bool,mut _3: [bool; 5],mut _4: [bool; 5],mut _5: [bool; 5],mut _6: [bool; 5],mut _7: bool,mut _8: bool,mut _9: [bool; 5],mut _10: [bool; 5],mut _11: [bool; 5]) -> isize {
mir! {
type RET = isize;
let _12: bool;
let _13: Adt46;
let _14: [u128; 1];
let _15: Adt49;
let _16: isize;
let _17: [bool; 5];
let _18: (bool, *const i128, u128, *const i128, u8);
let _19: [u128; 1];
let _20: Adt55;
let _21: [u64; 2];
let _22: u16;
let _23: u8;
let _24: [i8; 5];
let _25: f32;
let _26: isize;
let _27: ();
let _28: ();
{
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
_8 = _7;
_1 = [_7,_8,_2,_2,_2];
_6 = [_8,_7,_8,_7,_8];
_4 = [_8,_2,_8,_2,_7];
_12 = _2;
_12 = _2 > _8;
RET = 2338492582625015406_i64 as isize;
_6 = _10;
_1 = [_12,_2,_12,_2,_2];
_4 = [_8,_2,_8,_8,_7];
_11 = _1;
_5 = _3;
Goto(bb1)
}
bb1 = {
_9 = [_12,_12,_7,_7,_12];
_12 = _2;
_8 = !_2;
_5 = _6;
_2 = _12;
_5 = [_12,_2,_7,_7,_2];
_12 = _7 >= _2;
_12 = _8 > _7;
_15.fld2 = Adt45::Variant0 { fld0: 1207562581_i32 };
_10 = [_2,_8,_2,_2,_2];
_16 = RET << RET;
_4 = [_2,_8,_7,_12,_8];
_4 = [_12,_8,_7,_7,_8];
_3 = _5;
_11 = _3;
_14 = [331710228737214258157647167151804969356_u128];
Goto(bb2)
}
bb2 = {
_10 = [_8,_8,_8,_8,_8];
_15.fld2 = Adt45::Variant0 { fld0: (-2011067566_i32) };
_6 = _10;
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = (-1542826422_i32);
_3 = [_8,_12,_12,_12,_8];
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = 1426520507_i32;
_2 = !_7;
RET = _16;
_7 = !_2;
_14 = [211094969420559806362862001810232624827_u128];
_12 = _2 | _7;
_4 = [_8,_7,_7,_7,_12];
_6 = [_2,_8,_2,_12,_8];
Call(_3 = fn16(_10, _11, _10, _11, _4, RET, _1, _4, _9, _11, _5, _2, _7, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_18.2 = 237026336588345287833856953934400791371_u128;
_11 = [_7,_7,_2,_7,_8];
_18.4 = 118_i8 as u8;
_8 = _12 != _2;
_8 = !_12;
Goto(bb4)
}
bb4 = {
_14 = [_18.2];
_11 = [_7,_12,_12,_7,_2];
_11 = [_8,_2,_12,_12,_12];
RET = _16;
SetDiscriminant(_15.fld2, 0);
_15.fld1 = core::ptr::addr_of_mut!(_18.3);
_15.fld2 = Adt45::Variant0 { fld0: 867685049_i32 };
_5 = [_7,_8,_2,_7,_2];
_3 = _1;
_2 = _12 == _12;
_14 = [_18.2];
_4 = [_7,_7,_2,_12,_2];
Call(_19 = fn17(_10, _12, _10, _10, _16, _11, _5, _6, _5, _9, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_15.fld2 = Adt45::Variant0 { fld0: 1889656400_i32 };
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = 72_i8 as i32;
_17 = [_7,_12,_12,_7,_8];
_1 = [_2,_7,_8,_7,_12];
_10 = [_8,_8,_7,_7,_7];
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = !743200293_i32;
_18.2 = 263776821378063657625431365690753920637_u128;
_8 = _12;
_5 = [_7,_7,_2,_8,_7];
_8 = !_12;
_21 = [9983632482767557018_u64,4993142664117976340_u64];
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = 629561394_i32 + 370905933_i32;
_5 = [_7,_12,_7,_7,_12];
_5 = _17;
_18.0 = _8;
_23 = (-28967_i16) as u8;
match _18.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
263776821378063657625431365690753920637 => bb9,
_ => bb8
}
}
bb6 = {
_14 = [_18.2];
_11 = [_7,_12,_12,_7,_2];
_11 = [_8,_2,_12,_12,_12];
RET = _16;
SetDiscriminant(_15.fld2, 0);
_15.fld1 = core::ptr::addr_of_mut!(_18.3);
_15.fld2 = Adt45::Variant0 { fld0: 867685049_i32 };
_5 = [_7,_8,_2,_7,_2];
_3 = _1;
_2 = _12 == _12;
_14 = [_18.2];
_4 = [_7,_7,_2,_12,_2];
Call(_19 = fn17(_10, _12, _10, _10, _16, _11, _5, _6, _5, _9, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_9 = [_12,_12,_7,_7,_12];
_12 = _2;
_8 = !_2;
_5 = _6;
_2 = _12;
_5 = [_12,_2,_7,_7,_2];
_12 = _7 >= _2;
_12 = _8 > _7;
_15.fld2 = Adt45::Variant0 { fld0: 1207562581_i32 };
_10 = [_2,_8,_2,_2,_2];
_16 = RET << RET;
_4 = [_2,_8,_7,_12,_8];
_4 = [_12,_8,_7,_7,_8];
_3 = _5;
_11 = _3;
_14 = [331710228737214258157647167151804969356_u128];
Goto(bb2)
}
bb8 = {
_10 = [_8,_8,_8,_8,_8];
_15.fld2 = Adt45::Variant0 { fld0: (-2011067566_i32) };
_6 = _10;
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = (-1542826422_i32);
_3 = [_8,_12,_12,_12,_8];
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = 1426520507_i32;
_2 = !_7;
RET = _16;
_7 = !_2;
_14 = [211094969420559806362862001810232624827_u128];
_12 = _2 | _7;
_4 = [_8,_7,_7,_7,_12];
_6 = [_2,_8,_2,_12,_8];
Call(_3 = fn16(_10, _11, _10, _11, _4, RET, _1, _4, _9, _11, _5, _2, _7, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_2 = _18.0;
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_9 = [_2,_12,_2,_7,_7];
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_11 = [_12,_12,_8,_18.0,_8];
_1 = [_2,_8,_12,_7,_2];
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_12 = !_2;
_8 = _2 >= _2;
_15.fld2 = Adt45::Variant0 { fld0: 924202010_i32 };
_2 = _12 | _8;
Call(_13 = fn18(_8, _2, _18.0, _11, _2, _3, _4, _10, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = _10;
_3 = _11;
_15.fld1 = core::ptr::addr_of_mut!(_18.3);
_6 = [_7,_2,_2,_12,_8];
_22 = 5155_u16 << RET;
place!(Field::<[i64; 7]>(Variant(_13, 1), 0)) = [(-4292781979241459683_i64),6802759169857799685_i64,7672064276085903967_i64,(-3512802676703691282_i64),(-1087506591708529687_i64),(-6946577000728908684_i64),824004831778003325_i64];
_12 = _8 | _8;
_4 = [_18.0,_2,_8,_12,_18.0];
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_18.2 = 1722_i16 as u128;
_18.0 = _2;
place!(Field::<i32>(Variant(_15.fld2, 0), 0)) = _18.4 as i32;
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_19 = [_18.2];
_18.0 = _2 < _8;
_17 = [_7,_12,_18.0,_7,_2];
_16 = 0_usize as isize;
SetDiscriminant(_15.fld2, 1);
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_12 = !_7;
_18.0 = _12;
_9 = [_2,_8,_18.0,_7,_12];
RET = _16;
Goto(bb11)
}
bb11 = {
place!(Field::<[i64; 7]>(Variant(_13, 1), 0)) = [9186056979968016292_i64,5858816886927332144_i64,(-3242346151680216092_i64),3157200371218313869_i64,3448910863686533092_i64,6368253339148929025_i64,(-1879880771825277997_i64)];
_21 = [543486278631020199_u64,12148573727411568896_u64];
_19 = [_18.2];
place!(Field::<[i64; 7]>(Variant(_13, 1), 0)) = [(-1166968174366662624_i64),(-105478467984618692_i64),(-8256184380786045115_i64),6193899809039977633_i64,2439971059897534761_i64,1072969912916058432_i64,2926019378703346609_i64];
place!(Field::<[u32; 1]>(Variant(_15.fld2, 1), 2)) = [1078799446_u32];
_12 = _18.0;
RET = -_16;
RET = _2 as isize;
_18.0 = _12 == _8;
place!(Field::<(bool, *const i128, u128, *const i128, u8)>(Variant(_15.fld2, 1), 3)).2 = _18.2;
Goto(bb12)
}
bb12 = {
Call(_27 = dump_var(15_usize, 5_usize, Move(_5), 7_usize, Move(_7), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_27 = dump_var(15_usize, 16_usize, Move(_16), 8_usize, Move(_8), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_27 = dump_var(15_usize, 12_usize, Move(_12), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [bool; 5],mut _2: [bool; 5],mut _3: [bool; 5],mut _4: [bool; 5],mut _5: [bool; 5],mut _6: isize,mut _7: [bool; 5],mut _8: [bool; 5],mut _9: [bool; 5],mut _10: [bool; 5],mut _11: [bool; 5],mut _12: bool,mut _13: bool,mut _14: [bool; 5]) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _15: [char; 6];
let _16: [u128; 1];
let _17: f32;
let _18: [u128; 1];
let _19: ();
let _20: ();
{
_5 = [_13,_13,_12,_12,_13];
_2 = _14;
RET = [_13,_12,_12,_13,_12];
_9 = [_12,_13,_13,_12,_12];
_3 = [_12,_12,_12,_13,_12];
_16 = [13530346244254010538347492620440976998_u128];
_13 = !_12;
_15 = ['\u{10b632}','\u{6edaf}','\u{af6fc}','\u{d4a2e}','\u{f00fd}','\u{10dce7}'];
_5 = _11;
_16 = [196899881346071188952305363362475458460_u128];
_14 = _11;
_8 = _11;
_5 = [_12,_12,_12,_13,_13];
_13 = _12 ^ _12;
_13 = _12;
_15 = ['\u{4f5a}','\u{10a38f}','\u{bc2ef}','\u{8fd2f}','\u{13349}','\u{3687c}'];
RET = [_12,_12,_13,_12,_13];
_14 = [_12,_12,_13,_12,_13];
_4 = _5;
_3 = [_12,_12,_13,_12,_13];
_11 = [_12,_13,_13,_13,_12];
_17 = 286787052080495775627310052912998327840_u128 as f32;
RET = [_12,_13,_13,_12,_13];
_5 = [_13,_12,_13,_12,_12];
_16 = [188171444776175523325206806726690282850_u128];
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(16_usize, 8_usize, Move(_8), 7_usize, Move(_7), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(16_usize, 16_usize, Move(_16), 11_usize, Move(_11), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: [bool; 5],mut _2: bool,mut _3: [bool; 5],mut _4: [bool; 5],mut _5: isize,mut _6: [bool; 5],mut _7: [bool; 5],mut _8: [bool; 5],mut _9: [bool; 5],mut _10: [bool; 5],mut _11: bool) -> [u128; 1] {
mir! {
type RET = [u128; 1];
let _12: (f64, i8, f32);
let _13: u32;
let _14: char;
let _15: [u64; 2];
let _16: [bool; 5];
let _17: (bool, *const i128, u128, *const i128, u8);
let _18: f32;
let _19: [u32; 7];
let _20: u8;
let _21: bool;
let _22: [i128; 5];
let _23: [bool; 5];
let _24: bool;
let _25: i32;
let _26: [i8; 5];
let _27: bool;
let _28: isize;
let _29: f32;
let _30: isize;
let _31: i32;
let _32: Adt48;
let _33: u128;
let _34: char;
let _35: i16;
let _36: ();
let _37: ();
{
RET = [217729046260346587720712869843338687731_u128];
_10 = [_11,_11,_2,_11,_11];
_4 = [_2,_2,_11,_2,_11];
_8 = [_2,_11,_2,_11,_11];
_9 = [_2,_11,_11,_2,_2];
_12.0 = 2120041828_u32 as f64;
_3 = _6;
_3 = [_11,_2,_2,_2,_11];
_8 = _6;
RET = [152154280494078750752946773554935645742_u128];
_11 = _2;
_3 = [_11,_11,_11,_2,_2];
_12.2 = 17014165163955212786_u64 as f32;
_13 = 46822_u16 as u32;
RET = [295567921493555993357668457248757757485_u128];
_7 = _9;
_1 = [_11,_11,_11,_11,_2];
_12.2 = _5 as f32;
_13 = 2361036114_u32 + 2147663522_u32;
_2 = !_11;
_14 = '\u{88ca1}';
_15 = [8390267752466245853_u64,16885872345162033971_u64];
_6 = _7;
Call(_12.0 = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = [_11,_11,_11,_11,_11];
_17.4 = 25_u8;
_1 = _8;
_17.0 = _11;
_17.2 = !238352819620422783727174323118408279972_u128;
_13 = 1683220074_u32 | 3904779617_u32;
_3 = [_11,_17.0,_11,_2,_11];
_7 = [_11,_17.0,_17.0,_17.0,_11];
_8 = [_11,_2,_2,_17.0,_11];
_15 = [11250611587096794595_u64,13894526158796961609_u64];
_8 = _10;
_10 = _3;
_9 = [_11,_11,_11,_11,_17.0];
_6 = _1;
match _17.4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
25 => bb10,
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
_3 = [_11,_17.0,_2,_11,_17.0];
RET = [_17.2];
RET = [_17.2];
_11 = _17.0;
_19 = [_13,_13,_13,_13,_13,_13,_13];
_1 = _7;
_16 = [_17.0,_11,_17.0,_17.0,_11];
_19 = [_13,_13,_13,_13,_13,_13,_13];
_12.1 = 33_i8 << _17.2;
_17.0 = !_2;
_17.2 = 338660837631280102304533525747610155358_u128 - 238845773474243100150921824556188437115_u128;
_16 = _9;
_14 = '\u{7fcf3}';
_15 = [7784087380455244749_u64,14324074600184789704_u64];
_1 = [_17.0,_11,_17.0,_2,_11];
_14 = '\u{266a6}';
_12.2 = _17.4 as f32;
_23 = _3;
_18 = -_12.2;
match _17.4 {
0 => bb4,
1 => bb11,
25 => bb13,
_ => bb12
}
}
bb11 = {
_9 = [_11,_11,_11,_11,_11];
_17.4 = 25_u8;
_1 = _8;
_17.0 = _11;
_17.2 = !238352819620422783727174323118408279972_u128;
_13 = 1683220074_u32 | 3904779617_u32;
_3 = [_11,_17.0,_11,_2,_11];
_7 = [_11,_17.0,_17.0,_17.0,_11];
_8 = [_11,_2,_2,_17.0,_11];
_15 = [11250611587096794595_u64,13894526158796961609_u64];
_8 = _10;
_10 = _3;
_9 = [_11,_11,_11,_11,_17.0];
_6 = _1;
match _17.4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
25 => bb10,
_ => bb9
}
}
bb12 = {
Return()
}
bb13 = {
_20 = _17.4;
_22 = [68857158855486104095445680945000199548_i128,(-115204564965111523125829929681968407395_i128),(-61913782865674602230223233817418117227_i128),41070336581283981824848363940856608437_i128,(-62145583494985276288175203093017877818_i128)];
RET = [_17.2];
_3 = [_2,_17.0,_11,_2,_17.0];
_5 = 12832293821426990013_u64 as isize;
_4 = _7;
_13 = !1694725993_u32;
_16 = _4;
_26 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_13 = !2771818235_u32;
_18 = _17.2 as f32;
_15 = [17441093835178773903_u64,13628846238311680510_u64];
_21 = _2;
RET = [_17.2];
_20 = (-20668_i16) as u8;
match _17.4 {
25 => bb14,
_ => bb4
}
}
bb14 = {
_23 = _9;
_25 = 1224209894_i32 ^ (-1814281750_i32);
_29 = (-21925_i16) as f32;
_19 = [_13,_13,_13,_13,_13,_13,_13];
_24 = _2 >= _21;
_12.1 = 104_i8 & (-86_i8);
_22 = [146105837123020345342617600089609290981_i128,(-120542280205180553917069116623188083051_i128),(-119050263816301389132857457609160851393_i128),32447414461279630769537827936545865825_i128,(-157374914848317612314973223501028567810_i128)];
_25 = 466179707_i32;
_4 = [_17.0,_11,_24,_17.0,_21];
_27 = _24 == _11;
_5 = (-9223372036854775808_isize) + 89_isize;
_17.4 = !_20;
_20 = _13 as u8;
_29 = (-21870_i16) as f32;
_33 = !_17.2;
_27 = _24;
_12.2 = -_18;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(17_usize, 22_usize, Move(_22), 26_usize, Move(_26), 20_usize, Move(_20), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(17_usize, 4_usize, Move(_4), 3_usize, Move(_3), 11_usize, Move(_11), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(17_usize, 27_usize, Move(_27), 24_usize, Move(_24), 9_usize, Move(_9), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: [bool; 5],mut _5: bool,mut _6: [bool; 5],mut _7: [bool; 5],mut _8: [bool; 5],mut _9: [bool; 5]) -> Adt46 {
mir! {
type RET = Adt46;
let _10: Adt59;
let _11: [char; 6];
let _12: Adt44;
let _13: [char; 6];
let _14: [bool; 5];
let _15: u32;
let _16: u64;
let _17: isize;
let _18: isize;
let _19: i16;
let _20: f64;
let _21: f64;
let _22: u16;
let _23: isize;
let _24: *const *mut u128;
let _25: [i64; 7];
let _26: bool;
let _27: isize;
let _28: i16;
let _29: *mut u16;
let _30: isize;
let _31: [i128; 5];
let _32: [i64; 7];
let _33: i32;
let _34: [i8; 5];
let _35: *mut *const i128;
let _36: [u32; 1];
let _37: isize;
let _38: *mut u16;
let _39: isize;
let _40: isize;
let _41: Adt60;
let _42: ();
let _43: ();
{
_5 = !_1;
_4 = [_2,_5,_5,_1,_1];
_6 = _4;
_7 = [_2,_2,_2,_1,_3];
_2 = _3 <= _1;
_1 = !_5;
_9 = _4;
_6 = [_2,_2,_5,_1,_5];
_2 = _1 <= _5;
_11 = ['\u{eaf73}','\u{924ce}','\u{c4351}','\u{3bb40}','\u{13a64}','\u{767cc}'];
_11 = ['\u{676fc}','\u{a93a9}','\u{e8f07}','\u{10aa20}','\u{e75c8}','\u{66470}'];
_9 = _7;
_5 = _3 == _3;
_7 = [_3,_1,_2,_3,_1];
_3 = !_5;
_11 = ['\u{1fe70}','\u{6ecd3}','\u{4f35c}','\u{17bcc}','\u{888ee}','\u{4d969}'];
_5 = _3 & _3;
_2 = _3 < _1;
_5 = _1;
Goto(bb1)
}
bb1 = {
_7 = _9;
_8 = [_5,_5,_5,_3,_5];
_4 = [_1,_1,_1,_1,_2];
_4 = _7;
_7 = [_2,_2,_1,_2,_1];
_8 = _9;
Goto(bb2)
}
bb2 = {
_9 = [_5,_1,_2,_5,_5];
_7 = [_1,_2,_3,_3,_1];
_7 = _6;
_11 = ['\u{c198f}','\u{23c20}','\u{a78ba}','\u{e001c}','\u{e28f0}','\u{c2d78}'];
_9 = _4;
_11 = ['\u{3a215}','\u{2281e}','\u{880a3}','\u{bc606}','\u{ee1b2}','\u{2b181}'];
_1 = _3;
_4 = [_3,_2,_3,_2,_3];
_5 = !_1;
_1 = _2;
_5 = _3;
Goto(bb3)
}
bb3 = {
_3 = !_5;
_5 = _2;
_6 = [_5,_1,_5,_5,_3];
_8 = _6;
_5 = _2 != _2;
_2 = _1;
_8 = _6;
_11 = ['\u{a0211}','\u{109ec5}','\u{139d8}','\u{1055a1}','\u{548e0}','\u{7badc}'];
_3 = _1 ^ _5;
_3 = _5;
_5 = _1 != _2;
_8 = [_2,_3,_3,_3,_5];
_11 = ['\u{105ce1}','\u{a2d4b}','\u{dbfff}','\u{3693c}','\u{8723e}','\u{d5235}'];
_9 = [_2,_2,_2,_3,_2];
_6 = _8;
Goto(bb4)
}
bb4 = {
_11 = ['\u{3342}','\u{10b89b}','\u{e41a0}','\u{3f766}','\u{10d8a7}','\u{b49ff}'];
_1 = !_5;
Goto(bb5)
}
bb5 = {
_4 = [_5,_3,_2,_5,_5];
_8 = [_1,_5,_2,_1,_1];
_6 = [_3,_5,_3,_1,_3];
_9 = [_2,_5,_3,_2,_3];
_6 = _4;
_4 = [_1,_3,_3,_1,_2];
_9 = [_2,_3,_1,_5,_1];
_1 = !_2;
_5 = _3 == _2;
_1 = !_2;
_3 = !_5;
_13 = ['\u{940f2}','\u{f8cb9}','\u{58173}','\u{fc4bf}','\u{14d5d}','\u{59226}'];
_8 = _9;
_11 = ['\u{d30f3}','\u{3f6d7}','\u{1c0e2}','\u{fe1c2}','\u{849ee}','\u{432a1}'];
_8 = [_1,_3,_2,_3,_1];
_4 = [_2,_5,_3,_1,_2];
_1 = _2 ^ _3;
_14 = [_2,_5,_2,_5,_3];
_4 = [_5,_1,_1,_5,_2];
_13 = ['\u{59f8}','\u{914c7}','\u{588d9}','\u{c1b2e}','\u{ba374}','\u{a8ebf}'];
_5 = _1;
_15 = 1422088978_u32;
_1 = _2 < _3;
_7 = [_2,_5,_1,_5,_1];
_16 = _1 as u64;
_6 = _8;
match _15 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
1422088978 => bb10,
_ => bb9
}
}
bb6 = {
_11 = ['\u{3342}','\u{10b89b}','\u{e41a0}','\u{3f766}','\u{10d8a7}','\u{b49ff}'];
_1 = !_5;
Goto(bb5)
}
bb7 = {
_3 = !_5;
_5 = _2;
_6 = [_5,_1,_5,_5,_3];
_8 = _6;
_5 = _2 != _2;
_2 = _1;
_8 = _6;
_11 = ['\u{a0211}','\u{109ec5}','\u{139d8}','\u{1055a1}','\u{548e0}','\u{7badc}'];
_3 = _1 ^ _5;
_3 = _5;
_5 = _1 != _2;
_8 = [_2,_3,_3,_3,_5];
_11 = ['\u{105ce1}','\u{a2d4b}','\u{dbfff}','\u{3693c}','\u{8723e}','\u{d5235}'];
_9 = [_2,_2,_2,_3,_2];
_6 = _8;
Goto(bb4)
}
bb8 = {
_9 = [_5,_1,_2,_5,_5];
_7 = [_1,_2,_3,_3,_1];
_7 = _6;
_11 = ['\u{c198f}','\u{23c20}','\u{a78ba}','\u{e001c}','\u{e28f0}','\u{c2d78}'];
_9 = _4;
_11 = ['\u{3a215}','\u{2281e}','\u{880a3}','\u{bc606}','\u{ee1b2}','\u{2b181}'];
_1 = _3;
_4 = [_3,_2,_3,_2,_3];
_5 = !_1;
_1 = _2;
_5 = _3;
Goto(bb3)
}
bb9 = {
_7 = _9;
_8 = [_5,_5,_5,_3,_5];
_4 = [_1,_1,_1,_1,_2];
_4 = _7;
_7 = [_2,_2,_1,_2,_1];
_8 = _9;
Goto(bb2)
}
bb10 = {
_14 = [_1,_3,_1,_3,_3];
_15 = (-162875774097919671052637251221280302559_i128) as u32;
_17 = 164433341978420879009850448260809198971_i128 as isize;
_15 = (-31_i8) as u32;
_1 = !_2;
_2 = _1 | _3;
_15 = !727455995_u32;
_18 = _15 as isize;
_13 = ['\u{14bd8}','\u{1f151}','\u{7858c}','\u{cf4e0}','\u{9f8b2}','\u{8935a}'];
_3 = _1;
_8 = [_2,_3,_3,_3,_5];
_5 = _2;
_2 = _5;
_14 = [_2,_2,_3,_1,_1];
_21 = (-64_i8) as f64;
_19 = 10521_u16 as i16;
Goto(bb11)
}
bb11 = {
_4 = [_5,_1,_3,_2,_3];
_8 = [_2,_1,_5,_2,_5];
_26 = _2 <= _3;
_22 = 30243_u16 | 6718_u16;
_6 = [_1,_1,_1,_5,_1];
_23 = _17;
_16 = 325975534022907439468857562689372975996_u128 as u64;
_19 = 4768_i16;
_22 = !33081_u16;
_18 = (-34_i8) as isize;
_15 = 1141101567_u32 * 3868930598_u32;
_25 = [(-3998518979786023012_i64),(-1561997432139163264_i64),5103097677130283836_i64,2981189774743734785_i64,4600484799963736798_i64,1211027101386558742_i64,2112625196003922755_i64];
_17 = _23 & _18;
_4 = _8;
_9 = _8;
_2 = !_1;
_22 = !43988_u16;
_3 = !_5;
_1 = _5 >= _3;
_27 = _23;
_1 = _2;
_20 = 43_i8 as f64;
_5 = _2 & _2;
_17 = _27 + _27;
Goto(bb12)
}
bb12 = {
_3 = _1 & _5;
_15 = 7730361_u32;
_17 = _21 as isize;
_29 = core::ptr::addr_of_mut!(_22);
_5 = _1;
_28 = _19;
_25 = [(-7624622185153446054_i64),6302502614483948990_i64,(-6405911344223177411_i64),(-3103427586573084079_i64),252939765685859830_i64,(-9100768803048461382_i64),3150360552748383476_i64];
(*_29) = !8658_u16;
_19 = _28 | _28;
_19 = !_28;
_13 = ['\u{dc22e}','\u{63044}','\u{a77ec}','\u{ce545}','\u{33875}','\u{6f922}'];
_22 = 33612_u16 ^ 22298_u16;
_6 = [_1,_3,_3,_3,_26];
_30 = _18 & _18;
(*_29) = 42721_u16 * 49672_u16;
_7 = [_2,_2,_5,_26,_1];
_30 = _16 as isize;
_18 = !_30;
_19 = _28 >> _22;
_20 = -_21;
(*_29) = 50821484820977532931532566009110995047_i128 as u16;
Goto(bb13)
}
bb13 = {
(*_29) = !21086_u16;
(*_29) = 88219558549548706244681798864157446767_i128 as u16;
_5 = !_1;
_13 = ['\u{a219c}','\u{9c085}','\u{10e437}','\u{105cfc}','\u{9385e}','\u{7e4aa}'];
_5 = _1 ^ _3;
(*_29) = !47997_u16;
_19 = !_28;
_5 = _1 ^ _3;
_13 = ['\u{100526}','\u{52d0}','\u{ca44e}','\u{388df}','\u{529c5}','\u{3ae8e}'];
_31 = [(-78804596534622023301254116306521393930_i128),65633582083593569951846151537507844817_i128,(-1286631298327283111209991602814741959_i128),119237065593480943993702540452849706469_i128,(-60096559086454970852300897805733877112_i128)];
_20 = (-52324742745135413862419436656171311449_i128) as f64;
_28 = _19;
_14 = [_26,_2,_5,_2,_26];
_28 = !_19;
_3 = _1;
_32 = [491347057721284724_i64,(-3998535772505046863_i64),7025401616189106108_i64,8282969936852293590_i64,6451616802946085913_i64,(-1581991263192842633_i64),(-6124149238989169741_i64)];
_30 = _27 << _18;
_13 = _11;
_25 = [5766713411989993000_i64,(-1625374292810955056_i64),(-5625417693401964142_i64),(-1920595889287328800_i64),295308157209744841_i64,(-6839035981979488086_i64),(-1121294484520786490_i64)];
_22 = 50109_u16;
Goto(bb14)
}
bb14 = {
_2 = !_5;
match (*_29) {
0 => bb10,
50109 => bb16,
_ => bb15
}
}
bb15 = {
_14 = [_1,_3,_1,_3,_3];
_15 = (-162875774097919671052637251221280302559_i128) as u32;
_17 = 164433341978420879009850448260809198971_i128 as isize;
_15 = (-31_i8) as u32;
_1 = !_2;
_2 = _1 | _3;
_15 = !727455995_u32;
_18 = _15 as isize;
_13 = ['\u{14bd8}','\u{1f151}','\u{7858c}','\u{cf4e0}','\u{9f8b2}','\u{8935a}'];
_3 = _1;
_8 = [_2,_3,_3,_3,_5];
_5 = _2;
_2 = _5;
_14 = [_2,_2,_3,_1,_1];
_21 = (-64_i8) as f64;
_19 = 10521_u16 as i16;
Goto(bb11)
}
bb16 = {
_13 = _11;
_22 = !20735_u16;
_19 = _28;
_30 = _27;
_20 = -_21;
_25 = [5878911249498831626_i64,7268640201732425872_i64,(-4625212847522989332_i64),258269543592822640_i64,(-5245067338312318507_i64),3056001406065978279_i64,8499175308562901395_i64];
_34 = [72_i8,(-20_i8),65_i8,26_i8,66_i8];
(*_29) = 56405_u16 << _18;
_9 = _14;
_34 = [93_i8,(-95_i8),84_i8,(-32_i8),111_i8];
_32 = _25;
_33 = (-1619619481_i32);
_13 = ['\u{27bf0}','\u{3b485}','\u{ed2dc}','\u{c188}','\u{949c9}','\u{10823}'];
_3 = _2;
Call(_16 = core::intrinsics::bswap(6560626532978152708_u64), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_14 = _8;
_7 = [_2,_5,_5,_1,_1];
_17 = _16 as isize;
_23 = !_18;
_19 = _28 & _28;
_32 = [6032283608866663378_i64,(-3030311159454590166_i64),8312731043509490074_i64,174397672675400524_i64,6427540669486875271_i64,(-4359480397399572205_i64),(-69331960476741079_i64)];
RET = Adt46::Variant1 { fld0: _25 };
_5 = !_1;
_25 = _32;
_22 = 25656_u16 & 54726_u16;
_28 = 284911129292559086405041597667999651727_u128 as i16;
_22 = 6_usize as u16;
_34 = [32_i8,102_i8,(-23_i8),(-17_i8),55_i8];
_32 = [(-8005059596241459692_i64),(-8458999465305222747_i64),8887937973915753052_i64,2445870619470019340_i64,2993561147535649100_i64,(-1444604023432558970_i64),5367303487719811317_i64];
place!(Field::<[i64; 7]>(Variant(RET, 1), 0)) = [6957645831243866331_i64,7498256819646142639_i64,(-7494824242548785915_i64),8355153990056575816_i64,1363252276931294218_i64,5956597343840626878_i64,6462982181026416566_i64];
_38 = core::ptr::addr_of_mut!(_22);
(*_38) = 48772_u16;
_1 = _2 == _5;
(*_29) = _19 as u16;
_25 = [5009669124280093906_i64,398390455109175805_i64,(-3361958488290074957_i64),(-3625681580034461857_i64),(-2006182897209636301_i64),(-4785593195610900_i64),351658705612684787_i64];
_29 = core::ptr::addr_of_mut!((*_29));
_39 = _17 << _33;
_39 = 3812695787563759702_usize as isize;
Goto(bb18)
}
bb18 = {
Call(_42 = dump_var(18_usize, 39_usize, Move(_39), 23_usize, Move(_23), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(18_usize, 34_usize, Move(_34), 17_usize, Move(_17), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_42 = dump_var(18_usize, 14_usize, Move(_14), 18_usize, Move(_18), 31_usize, Move(_31), 11_usize, Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_42 = dump_var(18_usize, 15_usize, Move(_15), 32_usize, Move(_32), 43_usize, _43, 43_usize, _43), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: bool,mut _2: bool,mut _3: [bool; 5],mut _4: bool,mut _5: bool,mut _6: bool,mut _7: [bool; 5],mut _8: [bool; 5],mut _9: Adt45,mut _10: [bool; 5],mut _11: bool,mut _12: bool) -> bool {
mir! {
type RET = bool;
let _13: u64;
let _14: i128;
let _15: f32;
let _16: ();
let _17: ();
{
_6 = _1;
_5 = !_4;
RET = !_12;
RET = !_4;
_11 = !_4;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(19_usize, 4_usize, Move(_4), 1_usize, Move(_1), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(19_usize, 10_usize, Move(_10), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{3d022}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(36_i8), std::hint::black_box(17258_i16), std::hint::black_box(781316102_i32), std::hint::black_box((-8230647398923213200_i64)), std::hint::black_box(119645051273590128198100497033719736472_i128), std::hint::black_box(5_usize), std::hint::black_box(152_u8), std::hint::black_box(65058_u16), std::hint::black_box(2049202978_u32), std::hint::black_box(6597727627350748864_u64), std::hint::black_box(226674313752560783149186538797687629187_u128));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [char; 6],
fld1: *const *const i128,
fld2: *const u32,

},
Variant1{
fld0: *const *const i128,
fld1: *mut i8,
fld2: *const i128,
fld3: [char; 6],
fld4: u16,
fld5: i32,
fld6: u8,

},
Variant2{
fld0: (bool, *const i128, u128, *const i128, u8),

},
Variant3{
fld0: [u64; 2],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i32,

},
Variant1{
fld0: *const i128,
fld1: [char; 6],
fld2: [u32; 1],
fld3: (bool, *const i128, u128, *const i128, u8),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f64,
fld1: [i64; 7],
fld2: (i8, [i64; 7]),
fld3: usize,

},
Variant1{
fld0: [i64; 7],

},
Variant2{
fld0: *const u32,

},
Variant3{
fld0: *const i128,
fld1: *const *mut u128,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: f32,
fld1: i16,
fld2: i64,

},
Variant1{
fld0: *mut i8,
fld1: [i128; 5],
fld2: [char; 6],
fld3: (*mut *const i128,),
fld4: *mut u128,
fld5: [i8; 5],
fld6: i64,

},
Variant2{
fld0: (i16, *mut u128, u8, usize, char),
fld1: *mut i8,
fld2: i16,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i128; 5],
fld1: Adt46,
fld2: [i64; 7],
fld3: (i16, *mut u128, u8, usize, char),
fld4: u64,
fld5: usize,

},
Variant1{
fld0: f64,
fld1: i128,
fld2: [i64; 7],
fld3: [u128; 1],

},
Variant2{
fld0: bool,
fld1: char,
fld2: (i8, [i64; 7]),
fld3: ((usize, bool, *const i128), i8, (bool, *const i128, u128, *const i128, u8), (usize, bool, *const i128), [u32; 7]),
fld4: u32,
fld5: Adt44,
fld6: *mut char,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: *mut i8,
fld1: *mut *const i128,
fld2: Adt45,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: *mut u128,

},
Variant1{
fld0: [bool; 5],
fld1: char,
fld2: Adt49,
fld3: *mut i8,
fld4: f64,
fld5: (i8, [i64; 7]),
fld6: Adt47,

},
Variant2{
fld0: *mut char,
fld1: u64,
fld2: isize,
fld3: *const *mut u128,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *mut i8,
fld1: (i16, *mut u128, u8, usize, char),
fld2: u128,
fld3: u64,

},
Variant1{
fld0: *const u32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: bool,
fld1: Adt48,
fld2: Adt44,
fld3: u64,
fld4: i16,
fld5: u16,

},
Variant1{
fld0: (f64, i8, f32),
fld1: u16,

},
Variant2{
fld0: (f64, i8, f32),

},
Variant3{
fld0: ((usize, bool, *const i128), i8, (bool, *const i128, u128, *const i128, u8), (usize, bool, *const i128), [u32; 7]),
fld1: Adt49,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u128; 1],

},
Variant1{
fld0: i8,

},
Variant2{
fld0: *const u32,
fld1: [usize; 7],
fld2: Adt52,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt44,
fld1: char,
fld2: u32,
fld3: *mut *const i128,
fld4: *mut i8,
fld5: [u32; 1],
fld6: [i8; 5],
fld7: i128,

},
Variant1{
fld0: Adt45,
fld1: char,
fld2: (i16, *mut u128, u8, usize, char),

},
Variant2{
fld0: u16,
fld1: [char; 6],
fld2: (usize, bool, *const i128),

},
Variant3{
fld0: [i64; 7],
fld1: u32,
fld2: [i8; 5],
fld3: Adt45,
fld4: Adt47,
fld5: *const i128,

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt49,
fld1: u32,
fld2: isize,

},
Variant1{
fld0: Adt53,
fld1: [i64; 7],
fld2: u64,
fld3: Adt44,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: i16,
fld1: u16,
fld2: (i16, *mut u128, u8, usize, char),
fld3: (bool, *const i128, u128, *const i128, u8),

},
Variant1{
fld0: [u64; 2],
fld1: u8,
fld2: (*mut *const i128,),

},
Variant2{
fld0: bool,
fld1: u128,
fld2: [char; 6],
fld3: (*mut *const i128,),
fld4: *const *mut u128,
fld5: Adt55,
fld6: Adt47,
fld7: Adt45,

},
Variant3{
fld0: Adt52,
fld1: [u32; 1],
fld2: Adt48,
fld3: u16,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt52,
fld1: usize,
fld2: isize,
fld3: Adt55,
fld4: Adt50,

},
Variant1{
fld0: bool,
fld1: [i64; 7],

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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt49,
fld3: (i16, *mut u128, u8, usize, char),
fld4: (f64, i8, f32),
fld5: *mut *const i128,

},
Variant1{
fld0: (usize, bool, *const i128),
fld1: Adt53,
fld2: (f64, i8, f32),
fld3: i8,
fld4: *mut u128,
fld5: ((usize, bool, *const i128), i8, (bool, *const i128, u128, *const i128, u8), (usize, bool, *const i128), [u32; 7]),
fld6: [u32; 1],
fld7: Adt44,

},
Variant2{
fld0: (bool, *const i128, u128, *const i128, u8),
fld1: Adt55,
fld2: i64,
fld3: *const u32,
fld4: [u64; 2],
fld5: i32,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: u16,
fld1: *mut char,
fld2: Adt47,
fld3: i64,

},
Variant1{
fld0: Adt52,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt45,
fld1: Adt53,

},
Variant1{
fld0: usize,
fld1: Adt58,
fld2: u64,
fld3: Adt50,
fld4: *const *const i128,

}}

