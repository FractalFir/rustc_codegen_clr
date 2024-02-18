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
pub fn fn0(mut _1: bool,mut _2: i128,mut _3: u128,mut _4: u64,mut _5: i16,mut _6: u32,mut _7: i64) -> char {
mir! {
type RET = char;
let _8: f64;
let _9: *mut f32;
let _10: f64;
let _11: isize;
let _12: i128;
let _13: f32;
let _14: &'static ([i32; 5],);
let _15: isize;
let _16: (u32, &'static &'static bool, u16);
let _17: isize;
let _18: f32;
let _19: u32;
let _20: [bool; 8];
let _21: &'static &'static i128;
let _22: i16;
let _23: char;
let _24: ([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]);
let _25: u16;
let _26: [i32; 4];
let _27: [i64; 3];
let _28: char;
let _29: [i32; 4];
let _30: [isize; 2];
let _31: usize;
let _32: i8;
let _33: char;
let _34: *const *const isize;
let _35: usize;
let _36: Adt39;
let _37: i128;
let _38: &'static &'static &'static &'static bool;
let _39: ();
let _40: ();
{
_4 = 11120158474798908779_u64;
_5 = 4080_i16;
_6 = !4208772198_u32;
RET = '\u{8f4c5}';
_7 = 4342640768110249820_i64;
_7 = !578833768344706497_i64;
_6 = !461248637_u32;
Call(RET = fn1(_4, _4, _5, _5, _5, _7, _4, _4, _6, _7, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = !(-24829822601063235402115289257121674283_i128);
_3 = _4 as u128;
RET = '\u{e64d7}';
_5 = !20715_i16;
_3 = 264993660447964438757608456498189492656_u128;
_2 = -24339555471390657790769672997782572556_i128;
RET = '\u{bdb4d}';
_1 = _7 < _7;
RET = '\u{92635}';
_8 = 99_u8 as f64;
RET = '\u{aed23}';
_11 = -(-9223372036854775808_isize);
_10 = _8 * _8;
_8 = _10 + _10;
_10 = -_8;
_3 = !76235636767760486576059016371338235642_u128;
Call(_3 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _5 as u32;
_7 = (-3767522680763177626_i64);
_11 = (-9223372036854775808_isize);
_12 = _2;
_1 = !false;
RET = '\u{e8dbf}';
_6 = 1707854978_u32;
_4 = 5098587120126122458_u64 - 14074973145492370271_u64;
_9 = core::ptr::addr_of_mut!(_13);
(*_9) = _11 as f32;
_1 = false;
RET = '\u{1d3fc}';
_13 = 197_u8 as f32;
(*_9) = _10 as f32;
_15 = (-10_i8) as isize;
Goto(bb3)
}
bb3 = {
_7 = !(-5378653688228211374_i64);
_2 = _12;
_11 = _15;
_16.0 = 166_u8 as u32;
_2 = _7 as i128;
_13 = _8 as f32;
_17 = 18_u8 as isize;
_16.2 = _16.0 as u16;
_5 = !12048_i16;
RET = '\u{754a3}';
_2 = !_12;
_18 = 216_u8 as f32;
_20 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb4)
}
bb4 = {
(*_9) = -_18;
_19 = _6;
_4 = 7919775882900595745_u64;
_16.0 = _19;
_7 = 7912739349411963729_i64 + 4446789451257278736_i64;
_13 = _18;
_4 = 14708163900865475395_u64 * 9621121335707152832_u64;
RET = '\u{5a10d}';
_5 = (-25170_i16);
_13 = _7 as f32;
_5 = 115_i8 as i16;
_8 = _5 as f64;
_12 = _2 + _2;
_8 = _10 + _10;
Call(_4 = core::intrinsics::bswap(11458822826900771194_u64), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3 = !79555383281098516121107113968026594662_u128;
_13 = -_18;
_18 = (*_9) - (*_9);
_5 = (-2202_i16) + (-15832_i16);
_3 = 281693719091467754877942958734314558332_u128 ^ 149014137878246848624159169803286762208_u128;
_7 = (-552353828608466455_i64) - (-7704760109705580308_i64);
_22 = _5 - _5;
match _6 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
1707854978 => bb10,
_ => bb9
}
}
bb6 = {
(*_9) = -_18;
_19 = _6;
_4 = 7919775882900595745_u64;
_16.0 = _19;
_7 = 7912739349411963729_i64 + 4446789451257278736_i64;
_13 = _18;
_4 = 14708163900865475395_u64 * 9621121335707152832_u64;
RET = '\u{5a10d}';
_5 = (-25170_i16);
_13 = _7 as f32;
_5 = 115_i8 as i16;
_8 = _5 as f64;
_12 = _2 + _2;
_8 = _10 + _10;
Call(_4 = core::intrinsics::bswap(11458822826900771194_u64), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_7 = !(-5378653688228211374_i64);
_2 = _12;
_11 = _15;
_16.0 = 166_u8 as u32;
_2 = _7 as i128;
_13 = _8 as f32;
_17 = 18_u8 as isize;
_16.2 = _16.0 as u16;
_5 = !12048_i16;
RET = '\u{754a3}';
_2 = !_12;
_18 = 216_u8 as f32;
_20 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb4)
}
bb8 = {
_6 = _5 as u32;
_7 = (-3767522680763177626_i64);
_11 = (-9223372036854775808_isize);
_12 = _2;
_1 = !false;
RET = '\u{e8dbf}';
_6 = 1707854978_u32;
_4 = 5098587120126122458_u64 - 14074973145492370271_u64;
_9 = core::ptr::addr_of_mut!(_13);
(*_9) = _11 as f32;
_1 = false;
RET = '\u{1d3fc}';
_13 = 197_u8 as f32;
(*_9) = _10 as f32;
_15 = (-10_i8) as isize;
Goto(bb3)
}
bb9 = {
_2 = !(-24829822601063235402115289257121674283_i128);
_3 = _4 as u128;
RET = '\u{e64d7}';
_5 = !20715_i16;
_3 = 264993660447964438757608456498189492656_u128;
_2 = -24339555471390657790769672997782572556_i128;
RET = '\u{bdb4d}';
_1 = _7 < _7;
RET = '\u{92635}';
_8 = 99_u8 as f64;
RET = '\u{aed23}';
_11 = -(-9223372036854775808_isize);
_10 = _8 * _8;
_8 = _10 + _10;
_10 = -_8;
_3 = !76235636767760486576059016371338235642_u128;
Call(_3 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_23 = RET;
_18 = (*_9);
_18 = _16.0 as f32;
_2 = _5 as i128;
_5 = _22;
_6 = _19;
_2 = _12 & _12;
_16.0 = _1 as u32;
_13 = _18;
_25 = _16.2 * _16.2;
_24.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_15 = _17 - _17;
(*_9) = _18;
_17 = _15 | _11;
_6 = _16.0;
_4 = _7 as u64;
Goto(bb11)
}
bb11 = {
_7 = _15 as i64;
_2 = !_12;
_2 = -_12;
_9 = core::ptr::addr_of_mut!(_18);
_1 = _15 > _11;
_24.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
Goto(bb12)
}
bb12 = {
_28 = RET;
_11 = -_15;
_30 = [_15,_11];
_1 = true;
_25 = 7_usize as u16;
_27 = [_7,_7,_7];
_32 = _23 as i8;
_25 = _16.2 >> _15;
_31 = !7_usize;
_13 = -(*_9);
_3 = 112251578275700556801951919791848294479_u128 ^ 30219174970158142270310218730929804982_u128;
_10 = _7 as f64;
_29 = [1502736246_i32,1188250460_i32,(-373536036_i32),2057505837_i32];
Goto(bb13)
}
bb13 = {
_30 = [_15,_11];
_4 = _3 as u64;
_31 = 13759524698240520172_usize << _2;
_31 = 6_usize;
_3 = _24.0[_31] ^ _24.0[_31];
_19 = _3 as u32;
_18 = _13 - _13;
Goto(bb14)
}
bb14 = {
_15 = _8 as isize;
_19 = _2 as u32;
_11 = -_17;
_23 = RET;
_31 = !2_usize;
_37 = _12 & _12;
_33 = _28;
_22 = 593775301_i32 as i16;
RET = _23;
_8 = (*_9) as f64;
_10 = _16.0 as f64;
_29 = [(-1789693529_i32),(-1750346343_i32),(-2027380481_i32),(-85039165_i32)];
_27 = [_7,_7,_7];
_18 = _13;
_28 = _33;
_24.0 = [_3,_3,_3,_3,_3,_3,_3,_3];
_33 = RET;
_13 = _7 as f32;
_26 = [(-557701934_i32),111759157_i32,1028150134_i32,494298687_i32];
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(0_usize, 15_usize, Move(_15), 28_usize, Move(_28), 30_usize, Move(_30), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(0_usize, 19_usize, Move(_19), 5_usize, Move(_5), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(0_usize, 2_usize, Move(_2), 11_usize, Move(_11), 12_usize, Move(_12), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u64,mut _2: u64,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i64,mut _7: u64,mut _8: u64,mut _9: u32,mut _10: i64,mut _11: i16) -> char {
mir! {
type RET = char;
let _12: i8;
let _13: i64;
let _14: bool;
let _15: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), *mut f32, [i8; 2]);
let _16: f32;
let _17: [i16; 4];
let _18: *mut f32;
let _19: usize;
let _20: [i32; 5];
let _21: *mut f32;
let _22: i128;
let _23: &'static &'static i32;
let _24: [isize; 2];
let _25: ((i32, (u8, i8), isize, u128),);
let _26: isize;
let _27: i16;
let _28: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), *mut f32, [i8; 2]);
let _29: char;
let _30: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), *mut f32, [i8; 2]);
let _31: bool;
let _32: f64;
let _33: u64;
let _34: &'static bool;
let _35: ();
let _36: ();
{
_13 = _11 as i64;
_6 = -_10;
_12 = (-28_i8) ^ 121_i8;
_11 = -_5;
RET = '\u{448a6}';
_2 = _1 ^ _8;
_3 = true as i16;
RET = '\u{3b832}';
_9 = 89887743702913849235870657532687254651_u128 as u32;
_15.0.3 = &_15.2;
_15.0.0 = [44913579574521738354706610026097112452_u128,90638715420793955978045415141614868996_u128,279732536609405344237699399833564871076_u128,278600144462002569301494205267889404401_u128,289200635186218394277454696599295379322_u128,243550150172227107843427505369698607441_u128,104258448218363628009968430136069587390_u128,128115167791934713086202868316389862549_u128];
_3 = 60468_u16 as i16;
Goto(bb1)
}
bb1 = {
_12 = !117_i8;
_9 = _10 as u32;
_11 = _5 & _5;
_14 = !false;
_10 = _13 * _13;
_15.0.0 = [246939408355616287187211986310858200811_u128,145261871121671741816205148165187135896_u128,334128384427450247504000660940640194392_u128,79736083419740874426720463616722826029_u128,244818034839249039805317349225020642306_u128,87198992517776798469934946864299922770_u128,173716034380378020722081378484894948325_u128,32361637573428467014415106061034085230_u128];
_11 = _3 + _5;
_2 = _7;
_4 = 162030666896875422639236575360320005650_i128 as i16;
_4 = !_11;
_7 = !_2;
_6 = !_10;
_15.0.3 = &_15.2;
_11 = 57595251524663632982478910420949021734_u128 as i16;
_13 = _10 & _6;
_7 = _12 as u64;
_15.2 = [_12,_12];
_1 = !_7;
_9 = _8 as u32;
_13 = _10;
Call(_12 = core::intrinsics::transmute(_14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = _7 % _2;
_15.0.0 = [65142852724690850229511543926978344019_u128,329446979438434977591767990379196940059_u128,116073130063851457910764028463455475828_u128,80853474107358036731602369136048246390_u128,161359210676396193243720600018465777515_u128,111643084765341146604371110845539134430_u128,301460025825584725317019570753655645650_u128,203486541285870284204868987519278834445_u128];
_15.0.0 = [3417533683600126879507234421723761071_u128,50142604608765644737216577082139438924_u128,249768514734794176391108159856741090700_u128,48798111016513724996173248216967338833_u128,10248496137208660553613839444827142618_u128,311556864540360198605905445521782053731_u128,2098611704632981731708964171406946534_u128,205420295783046681757560639433972807607_u128];
Call(_2 = fn2(_13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = 9223372036854775807_isize as f32;
_3 = _5;
_16 = _3 as f32;
_18 = core::ptr::addr_of_mut!(_16);
_15.0.3 = &_15.2;
_17 = [_4,_11,_4,_4];
_19 = !1_usize;
_12 = !(-89_i8);
_10 = RET as i64;
_6 = _13 ^ _13;
_3 = _5 + _5;
_9 = RET as u32;
_3 = -_5;
_4 = -_11;
_7 = !_1;
_15.0.3 = &_15.2;
(*_18) = _5 as f32;
_16 = _9 as f32;
_16 = _12 as f32;
_4 = _11;
_1 = _2 + _7;
_14 = !false;
_1 = _7;
Goto(bb4)
}
bb4 = {
RET = '\u{74dd6}';
_15.0.3 = &_15.2;
_17 = [_4,_3,_11,_4];
_17 = [_3,_11,_4,_5];
_9 = 1134532915_u32 >> _6;
_12 = (-42_i8);
_1 = !_8;
_2 = !_8;
_6 = _10 | _13;
_13 = _6 | _6;
_21 = core::ptr::addr_of_mut!(_16);
(*_18) = _12 as f32;
_17 = [_11,_3,_5,_4];
_9 = !729694468_u32;
_4 = RET as i16;
match _12 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431768211414 => bb11,
_ => bb10
}
}
bb5 = {
_16 = 9223372036854775807_isize as f32;
_3 = _5;
_16 = _3 as f32;
_18 = core::ptr::addr_of_mut!(_16);
_15.0.3 = &_15.2;
_17 = [_4,_11,_4,_4];
_19 = !1_usize;
_12 = !(-89_i8);
_10 = RET as i64;
_6 = _13 ^ _13;
_3 = _5 + _5;
_9 = RET as u32;
_3 = -_5;
_4 = -_11;
_7 = !_1;
_15.0.3 = &_15.2;
(*_18) = _5 as f32;
_16 = _9 as f32;
_16 = _12 as f32;
_4 = _11;
_1 = _2 + _7;
_14 = !false;
_1 = _7;
Goto(bb4)
}
bb6 = {
_1 = _7 % _2;
_15.0.0 = [65142852724690850229511543926978344019_u128,329446979438434977591767990379196940059_u128,116073130063851457910764028463455475828_u128,80853474107358036731602369136048246390_u128,161359210676396193243720600018465777515_u128,111643084765341146604371110845539134430_u128,301460025825584725317019570753655645650_u128,203486541285870284204868987519278834445_u128];
_15.0.0 = [3417533683600126879507234421723761071_u128,50142604608765644737216577082139438924_u128,249768514734794176391108159856741090700_u128,48798111016513724996173248216967338833_u128,10248496137208660553613839444827142618_u128,311556864540360198605905445521782053731_u128,2098611704632981731708964171406946534_u128,205420295783046681757560639433972807607_u128];
Call(_2 = fn2(_13), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_12 = !117_i8;
_9 = _10 as u32;
_11 = _5 & _5;
_14 = !false;
_10 = _13 * _13;
_15.0.0 = [246939408355616287187211986310858200811_u128,145261871121671741816205148165187135896_u128,334128384427450247504000660940640194392_u128,79736083419740874426720463616722826029_u128,244818034839249039805317349225020642306_u128,87198992517776798469934946864299922770_u128,173716034380378020722081378484894948325_u128,32361637573428467014415106061034085230_u128];
_11 = _3 + _5;
_2 = _7;
_4 = 162030666896875422639236575360320005650_i128 as i16;
_4 = !_11;
_7 = !_2;
_6 = !_10;
_15.0.3 = &_15.2;
_11 = 57595251524663632982478910420949021734_u128 as i16;
_13 = _10 & _6;
_7 = _12 as u64;
_15.2 = [_12,_12];
_1 = !_7;
_9 = _8 as u32;
_13 = _10;
Call(_12 = core::intrinsics::transmute(_14), ReturnTo(bb2), UnwindUnreachable())
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
(*_21) = _7 as f32;
_6 = !_13;
_9 = !4179869970_u32;
(*_21) = 192503378988215489662729337185834522064_u128 as f32;
_15.0.3 = &_15.2;
_14 = false & false;
_25.0.3 = 103375092510084579996987073013746325149_u128;
_15.0.3 = &_15.2;
_8 = _6 as u64;
_15.0.3 = &_15.2;
_1 = _8;
_5 = 42852_u16 as i16;
_25.0.1.1 = _12 + _12;
_20 = [2055404859_i32,(-2090894996_i32),1563248759_i32,1242630662_i32,437923660_i32];
_15.0.0 = [_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3];
_25.0.1 = (113_u8, _12);
(*_21) = (-17_isize) as f32;
_25.0.3 = !86462443211216930783684902745239741497_u128;
(*_18) = _25.0.3 as f32;
_25.0.2 = _9 as isize;
_15.0.1 = core::ptr::addr_of!(_25.0.1);
_25.0.2 = (-20_isize);
_2 = _8;
_28.0.3 = &_28.2;
match _25.0.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb10,
340282366920938463463374607431768211436 => bb12,
_ => bb6
}
}
bb12 = {
_5 = _3 - _11;
_17 = [_11,_5,_5,_11];
_29 = RET;
_16 = _19 as f32;
_25.0.0 = (-230331938_i32) | 86900977_i32;
_25.0.1.0 = 48_u8 * 129_u8;
_11 = _5;
RET = _29;
_11 = _5 * _3;
match _25.0.2 {
0 => bb5,
1 => bb7,
2 => bb13,
3 => bb14,
340282366920938463463374607431768211436 => bb16,
_ => bb15
}
}
bb13 = {
(*_21) = _7 as f32;
_6 = !_13;
_9 = !4179869970_u32;
(*_21) = 192503378988215489662729337185834522064_u128 as f32;
_15.0.3 = &_15.2;
_14 = false & false;
_25.0.3 = 103375092510084579996987073013746325149_u128;
_15.0.3 = &_15.2;
_8 = _6 as u64;
_15.0.3 = &_15.2;
_1 = _8;
_5 = 42852_u16 as i16;
_25.0.1.1 = _12 + _12;
_20 = [2055404859_i32,(-2090894996_i32),1563248759_i32,1242630662_i32,437923660_i32];
_15.0.0 = [_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3];
_25.0.1 = (113_u8, _12);
(*_21) = (-17_isize) as f32;
_25.0.3 = !86462443211216930783684902745239741497_u128;
(*_18) = _25.0.3 as f32;
_25.0.2 = _9 as isize;
_15.0.1 = core::ptr::addr_of!(_25.0.1);
_25.0.2 = (-20_isize);
_2 = _8;
_28.0.3 = &_28.2;
match _25.0.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb10,
340282366920938463463374607431768211436 => bb12,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_16 = 9223372036854775807_isize as f32;
_3 = _5;
_16 = _3 as f32;
_18 = core::ptr::addr_of_mut!(_16);
_15.0.3 = &_15.2;
_17 = [_4,_11,_4,_4];
_19 = !1_usize;
_12 = !(-89_i8);
_10 = RET as i64;
_6 = _13 ^ _13;
_3 = _5 + _5;
_9 = RET as u32;
_3 = -_5;
_4 = -_11;
_7 = !_1;
_15.0.3 = &_15.2;
(*_18) = _5 as f32;
_16 = _9 as f32;
_16 = _12 as f32;
_4 = _11;
_1 = _2 + _7;
_14 = !false;
_1 = _7;
Goto(bb4)
}
bb16 = {
_15.1 = Move(_18);
_5 = _11 + _4;
_20 = [_25.0.0,_25.0.0,_25.0.0,_25.0.0,_25.0.0];
_30.0.0 = [_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3];
_16 = _5 as f32;
_15.0.3 = &_15.2;
_26 = !_25.0.2;
_28.0.1 = core::ptr::addr_of!(_25.0.1);
_30.0.0 = [_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3,_25.0.3];
_12 = _25.0.3 as i8;
(*_21) = _8 as f32;
_22 = (-98917061736422520516180982601372892051_i128);
_30.0.3 = &_28.2;
_28.2 = [_25.0.1.1,_12];
RET = _29;
_25.0.1 = (53_u8, _12);
_28.1 = core::ptr::addr_of_mut!((*_21));
(*_21) = _25.0.1.0 as f32;
_7 = !_2;
_24 = [_26,_25.0.2];
_20 = [_25.0.0,_25.0.0,_25.0.0,_25.0.0,_25.0.0];
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(1_usize, 1_usize, Move(_1), 2_usize, Move(_2), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(1_usize, 6_usize, Move(_6), 10_usize, Move(_10), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(1_usize, 20_usize, Move(_20), 29_usize, Move(_29), 11_usize, Move(_11), 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i64) -> u64 {
mir! {
type RET = u64;
let _2: *const *const isize;
let _3: f64;
let _4: [char; 4];
let _5: i8;
let _6: Adt32;
let _7: &'static (i32, (u8, i8), isize, u128);
let _8: [bool; 2];
let _9: isize;
let _10: [u128; 8];
let _11: f64;
let _12: [i32; 4];
let _13: u64;
let _14: f32;
let _15: &'static i32;
let _16: &'static u8;
let _17: u32;
let _18: f64;
let _19: [char; 4];
let _20: isize;
let _21: u128;
let _22: isize;
let _23: i16;
let _24: [i8; 8];
let _25: Adt61;
let _26: &'static &'static i32;
let _27: &'static i128;
let _28: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), u32, *const [u128; 8], &'static *const [u128; 8]);
let _29: char;
let _30: u128;
let _31: char;
let _32: &'static *mut *const (u8, i8);
let _33: f32;
let _34: [i32; 4];
let _35: *mut f64;
let _36: isize;
let _37: char;
let _38: Adt31;
let _39: [bool; 8];
let _40: &'static [char; 6];
let _41: *mut &'static u8;
let _42: &'static [i32; 5];
let _43: bool;
let _44: f32;
let _45: &'static &'static i32;
let _46: (&'static u16, Adt31);
let _47: ();
let _48: ();
{
_1 = 1634365506670877243_i64 | 5413107610136138887_i64;
RET = !12705011635780973261_u64;
RET = 5651421524692950306_u64 >> _1;
_1 = 2211614816_u32 as i64;
_1 = (-3309148333072637880_i64);
RET = !14844708459763287469_u64;
RET = !6014132768335588141_u64;
_1 = 30327_u16 as i64;
RET = 18007145063697919718_u64 >> _1;
RET = 56999_u16 as u64;
_3 = 88_u8 as f64;
RET = 19154713_u32 as u64;
_4 = ['\u{dab6a}','\u{59c3d}','\u{12da5}','\u{f7be8}'];
_4 = ['\u{af2ed}','\u{a3974}','\u{2655}','\u{db700}'];
_1 = (-4748170426554050321_i64) >> RET;
_4 = ['\u{8ad7}','\u{e2dea}','\u{5a718}','\u{2ac0b}'];
Call(_1 = fn3(_4, _4, _4, RET, _4, _4, _4, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !235840748657561653_u64;
_3 = RET as f64;
_4 = ['\u{171c6}','\u{e2aea}','\u{a3bc6}','\u{47c6}'];
RET = 15230823970009794164_u64 + 14949197045205269946_u64;
RET = 9223372036854775807_isize as u64;
_5 = 164683398699959701860532978982372222884_u128 as i8;
_5 = _1 as i8;
_1 = !(-5670416790351272335_i64);
RET = 7129528920735917275_u64 - 13682496810941013079_u64;
_5 = 60_i8 * 10_i8;
RET = !16464826707466344270_u64;
RET = 3351013015612518423_u64;
_8 = [true,false];
RET = !13008311317323895115_u64;
_12 = [1692182077_i32,1864630116_i32,1137983260_i32,(-1177450436_i32)];
Goto(bb2)
}
bb2 = {
RET = !15815939723498064375_u64;
Call(_3 = fn4(_8, _12, _4, _4, _12, _4, _12, _12, _4, _12, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8 = [true,false];
_9 = 9223372036854775807_isize;
_8 = [false,false];
RET = 3309342106859711819_u64;
_4 = ['\u{1cdb3}','\u{f7d6b}','\u{bef38}','\u{73f2d}'];
_10 = [89321725715173020831689810336683796954_u128,14198617673815624174437341977835838159_u128,331567780526307811647976623497883428037_u128,307695534045403979809255509194350459991_u128,200986322940167854425947551754264001094_u128,66699274816136373841952449259386459038_u128,241167445557948165411682823717379096802_u128,198671545978325413619836042760669842189_u128];
_8 = [false,false];
_3 = 177144217476350074997441979497383741785_u128 as f64;
_5 = !(-62_i8);
_13 = 2798701714692164067589816208479854286_u128 as u64;
_14 = 1911970254_u32 as f32;
_14 = (-32618_i16) as f32;
_17 = 555196769_u32 ^ 382729379_u32;
_10 = [218951516966520839972815805877898699544_u128,177672725061639042287731045139676365610_u128,127305208293823858264517130977652390681_u128,212786600626039542024274869656665706947_u128,35612592933164636574239970802887053531_u128,184897477839071939372215712964437880194_u128,110566872352360317999897488302570630564_u128,13333876890493740487240353875451044179_u128];
_12 = [(-1428358210_i32),920208210_i32,(-202113987_i32),(-174399767_i32)];
_11 = _3 + _3;
_17 = 95270248_u32 - 625184606_u32;
_5 = 105_i8;
RET = _13 ^ _13;
_18 = _3;
_9 = 20_isize;
_14 = _5 as f32;
_13 = RET & RET;
_9 = 9223372036854775807_isize | 9223372036854775807_isize;
_10 = [15497007747293715387082358234532429045_u128,72085869326923967866627995859461150216_u128,313921323829238792396481161601714635189_u128,53918722490571850918620876417365094689_u128,235818447058055013077413440512940592377_u128,324295465214678232842349985144893116433_u128,38511356276994119259292159565008284092_u128,69247922442220721603915200985189216009_u128];
_13 = (-2109260774_i32) as u64;
match _5 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
105 => bb9,
_ => bb8
}
}
bb4 = {
RET = !15815939723498064375_u64;
Call(_3 = fn4(_8, _12, _4, _4, _12, _4, _12, _12, _4, _12, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = !235840748657561653_u64;
_3 = RET as f64;
_4 = ['\u{171c6}','\u{e2aea}','\u{a3bc6}','\u{47c6}'];
RET = 15230823970009794164_u64 + 14949197045205269946_u64;
RET = 9223372036854775807_isize as u64;
_5 = 164683398699959701860532978982372222884_u128 as i8;
_5 = _1 as i8;
_1 = !(-5670416790351272335_i64);
RET = 7129528920735917275_u64 - 13682496810941013079_u64;
_5 = 60_i8 * 10_i8;
RET = !16464826707466344270_u64;
RET = 3351013015612518423_u64;
_8 = [true,false];
RET = !13008311317323895115_u64;
_12 = [1692182077_i32,1864630116_i32,1137983260_i32,(-1177450436_i32)];
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
_10 = [155089647840913741390812098914603423983_u128,30054374351085854426512450501181191416_u128,29100430868253380827182323195066147750_u128,64607958964109920720547687970165837495_u128,117488695401363201397011422342880284415_u128,149295390867632271870897612821705474450_u128,188504053092782714485162663805872722341_u128,66352601794607309902285277053647624216_u128];
_3 = _5 as f64;
_1 = (-27633_i16) as i64;
_13 = RET;
_4 = ['\u{20df2}','\u{ef613}','\u{450a7}','\u{ddecf}'];
_20 = _9;
_12 = [161841728_i32,1504477264_i32,1550971667_i32,(-1823227521_i32)];
_23 = !19201_i16;
_12 = [1437203044_i32,(-506053912_i32),(-1537469690_i32),(-1236480945_i32)];
_21 = 36364617730384185088185437769779720266_u128 * 171264645574220926144901250651957254401_u128;
_19 = ['\u{5a051}','\u{959aa}','\u{27057}','\u{ddde7}'];
_1 = (-548135214524831503_i64);
_17 = !3762705946_u32;
_3 = -_11;
RET = !_13;
_10 = [_21,_21,_21,_21,_21,_21,_21,_21];
_23 = -(-6753_i16);
_12 = [(-1764129427_i32),(-2135497202_i32),(-982243064_i32),1160894603_i32];
_20 = _9 ^ _9;
_4 = _19;
_24 = [_5,_5,_5,_5,_5,_5,_5,_5];
_5 = _14 as i8;
_18 = _11;
_3 = 111878831785968980696406335875660294832_i128 as f64;
_8 = [true,true];
Goto(bb10)
}
bb10 = {
_25.fld0 = Adt41::Variant0 { fld0: _1 };
_12 = [1820236398_i32,(-722645702_i32),(-821808894_i32),2101918688_i32];
_4 = ['\u{1b5d0}','\u{e3e7f}','\u{b7c68}','\u{6a3fa}'];
_23 = !(-6647_i16);
_26 = &_15;
_22 = _20 >> RET;
_22 = _20;
_10 = [_21,_21,_21,_21,_21,_21,_21,_21];
_4 = _19;
_28.0.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_28.3 = &_28.2;
Goto(bb11)
}
bb11 = {
_28.1 = _22 as u32;
_8 = [true,true];
_28.0.2 = &_15;
_28.1 = _17 << _22;
RET = _13;
_26 = &(*_26);
_12 = [1816638602_i32,406174498_i32,(-397939605_i32),1625072877_i32];
_20 = _22 & _22;
_1 = Field::<i64>(Variant(_25.fld0, 0), 0) >> _20;
_13 = RET >> _1;
_4 = ['\u{1d5da}','\u{3e8f8}','\u{d76bf}','\u{537a}'];
_20 = -_22;
_28.0.2 = &(*_26);
_23 = -(-25550_i16);
SetDiscriminant(_25.fld0, 0);
_14 = _1 as f32;
_22 = _13 as isize;
RET = 176_u8 as u64;
_23 = (-141134202443304390569295766342713925122_i128) as i16;
_28.3 = &_28.2;
_12 = [(-1709792770_i32),151752198_i32,1291147414_i32,(-1247525578_i32)];
_33 = _11 as f32;
_28.0.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
place!(Field::<i64>(Variant(_25.fld0, 0), 0)) = _1;
Goto(bb12)
}
bb12 = {
_26 = &_15;
_5 = 7_i8;
_24 = [_5,_5,_5,_5,_5,_5,_5,_5];
_28.0.2 = &(*_26);
_24 = [_5,_5,_5,_5,_5,_5,_5,_5];
_28.0.0 = [_21,_21,_21,_21,_21,_21,_21,_21];
_28.0.2 = &(*_26);
_14 = _33 + _33;
_29 = '\u{74d25}';
_17 = _28.1;
_28.0.2 = &(*_26);
_28.3 = &_28.2;
_34 = [(-870299402_i32),1434498581_i32,(-942841378_i32),1054802290_i32];
_29 = '\u{d516c}';
_28.3 = &_28.2;
_11 = _23 as f64;
_28.2 = core::ptr::addr_of!(_28.0.0);
_29 = '\u{cf6fc}';
_35 = core::ptr::addr_of_mut!(_11);
_28.3 = &_28.2;
Goto(bb13)
}
bb13 = {
place!(Field::<i64>(Variant(_25.fld0, 0), 0)) = _1;
_30 = !_21;
SetDiscriminant(_25.fld0, 0);
_29 = '\u{e77bd}';
_31 = _29;
_13 = !RET;
match _5 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
7 => bb19,
_ => bb18
}
}
bb14 = {
RET = !15815939723498064375_u64;
Call(_3 = fn4(_8, _12, _4, _4, _12, _4, _12, _12, _4, _12, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
RET = !235840748657561653_u64;
_3 = RET as f64;
_4 = ['\u{171c6}','\u{e2aea}','\u{a3bc6}','\u{47c6}'];
RET = 15230823970009794164_u64 + 14949197045205269946_u64;
RET = 9223372036854775807_isize as u64;
_5 = 164683398699959701860532978982372222884_u128 as i8;
_5 = _1 as i8;
_1 = !(-5670416790351272335_i64);
RET = 7129528920735917275_u64 - 13682496810941013079_u64;
_5 = 60_i8 * 10_i8;
RET = !16464826707466344270_u64;
RET = 3351013015612518423_u64;
_8 = [true,false];
RET = !13008311317323895115_u64;
_12 = [1692182077_i32,1864630116_i32,1137983260_i32,(-1177450436_i32)];
Goto(bb2)
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
_38.fld2 = _14 as i16;
_13 = RET;
_28.3 = &_28.2;
place!(Field::<i64>(Variant(_25.fld0, 0), 0)) = _1;
SetDiscriminant(_25.fld0, 1);
place!(Field::<i8>(Variant(_25.fld0, 1), 2)) = _5;
_20 = 5093_u16 as isize;
_36 = _9;
_41 = core::ptr::addr_of_mut!(_16);
_5 = Field::<i8>(Variant(_25.fld0, 1), 2);
_23 = _38.fld2;
_8 = [false,false];
(*_35) = (-99178497740664566748157340842352013686_i128) as f64;
_43 = _30 >= _30;
_44 = -_14;
_1 = !5886996913443294454_i64;
_23 = _38.fld2 + _38.fld2;
_28.0.2 = &(*_26);
_14 = -_44;
_8 = [_43,_43];
_44 = _14;
_22 = _36;
_43 = false;
Goto(bb20)
}
bb20 = {
Call(_47 = dump_var(2_usize, 43_usize, Move(_43), 31_usize, Move(_31), 22_usize, Move(_22), 21_usize, Move(_21)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_47 = dump_var(2_usize, 4_usize, Move(_4), 12_usize, Move(_12), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_47 = dump_var(2_usize, 13_usize, Move(_13), 36_usize, Move(_36), 48_usize, _48, 48_usize, _48), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [char; 4],mut _2: [char; 4],mut _3: [char; 4],mut _4: u64,mut _5: [char; 4],mut _6: [char; 4],mut _7: [char; 4],mut _8: f64) -> i64 {
mir! {
type RET = i64;
let _9: char;
let _10: [i64; 3];
let _11: isize;
let _12: isize;
let _13: &'static u8;
let _14: [u128; 8];
let _15: &'static u16;
let _16: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), u32, *const [u128; 8], &'static *const [u128; 8]);
let _17: i8;
let _18: isize;
let _19: ();
let _20: ();
{
RET = 4123293243738315685_i64 + (-5594459447741270419_i64);
_1 = _3;
_8 = 144314360262713157825956051219100247825_u128 as f64;
_1 = _2;
_7 = _5;
_4 = 6851800025761236957_u64;
RET = !(-4771300610532320882_i64);
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
6851800025761236957 => bb7,
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
_3 = ['\u{10e9c4}','\u{c9fc0}','\u{26c57}','\u{24efa}'];
_1 = ['\u{2ce12}','\u{b8288}','\u{c140c}','\u{86580}'];
_2 = ['\u{f360e}','\u{6d5f0}','\u{6eb57}','\u{334cf}'];
_7 = _5;
RET = 267807055_i32 as i64;
RET = 5294663571944040704_i64;
_6 = ['\u{102464}','\u{d3644}','\u{fde7d}','\u{a5364}'];
RET = !(-1841860336657361948_i64);
_4 = _8 as u64;
_6 = ['\u{6e16b}','\u{19ce9}','\u{2d03e}','\u{38df0}'];
RET = 8653853472990333840_i64 & 1259675059321948124_i64;
_6 = ['\u{c25d1}','\u{cfa99}','\u{10d389}','\u{dd289}'];
RET = !(-4673475634607035849_i64);
_3 = ['\u{1871f}','\u{1015d7}','\u{fb846}','\u{97967}'];
RET = (-1405684021162452588_i64);
RET = 7341_u16 as i64;
_5 = ['\u{aea37}','\u{83018}','\u{86d83}','\u{a9dd}'];
_6 = ['\u{d58ea}','\u{76cba}','\u{cf6d5}','\u{f70b3}'];
_8 = 1997856666_i32 as f64;
RET = true as i64;
_1 = ['\u{2b485}','\u{fe90c}','\u{974fe}','\u{73dbc}'];
_4 = true as u64;
_4 = 9525894773741668588_u64;
_6 = ['\u{fd31d}','\u{e5a96}','\u{dcf85}','\u{2a76f}'];
_2 = _1;
_7 = _6;
_9 = '\u{487e3}';
Goto(bb8)
}
bb8 = {
_7 = [_9,_9,_9,_9];
_2 = [_9,_9,_9,_9];
_4 = (-128_i8) as u64;
_10 = [RET,RET,RET];
RET = !691458365450972189_i64;
_4 = 9438709053736953385_u64;
_9 = '\u{98234}';
_10 = [RET,RET,RET];
_2 = _7;
_2 = _5;
_10 = [RET,RET,RET];
_6 = [_9,_9,_9,_9];
Goto(bb9)
}
bb9 = {
_8 = RET as f64;
_2 = _3;
_6 = [_9,_9,_9,_9];
_1 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = -(-1166785623824105929_i64);
_4 = _8 as u64;
_5 = _3;
RET = _9 as i64;
RET = 2699850232177931948_i64;
_4 = 6030242700856665891_u64 >> RET;
_5 = [_9,_9,_9,_9];
_11 = (-38_isize);
_11 = !9223372036854775807_isize;
RET = 5922862914253922126_i64 - (-5544289133930797275_i64);
_4 = 6867387285356355582_u64;
_6 = _2;
RET = 31130_u16 as i64;
_2 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
_11 = (-69_isize);
RET = !2776607082559272948_i64;
match _11 {
0 => bb9,
1 => bb4,
2 => bb11,
340282366920938463463374607431768211387 => bb13,
_ => bb12
}
}
bb11 = {
_8 = RET as f64;
_2 = _3;
_6 = [_9,_9,_9,_9];
_1 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_8 = (-27606_i16) as f64;
_12 = _11 << _4;
_3 = [_9,_9,_9,_9];
_14 = [176826012859230912564547443500030998062_u128,160599128675717530720722974267145993375_u128,141642833358840431825917488081847005577_u128,12594656885718081756907186199664823161_u128,151373499349401687307783745888039690653_u128,34192003052943188526281809968239069469_u128,260181508057262124134141298575417830510_u128,1958046833267252472583004744867194628_u128];
_14 = [338997277171607291390455035102518713864_u128,266678400224195224446173167653124321245_u128,61325933563573494691389742744858151583_u128,129672726105701609619401361757469169235_u128,62230635921786927428122050082186733805_u128,252598869075835175936530337232065154571_u128,303988988360027890316308071932958218365_u128,306543155442419313449374178572633820297_u128];
_9 = '\u{60e6e}';
_6 = [_9,_9,_9,_9];
_10 = [RET,RET,RET];
_11 = _12;
match _4 {
0 => bb11,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
6867387285356355582 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_8 = RET as f64;
_2 = _3;
_6 = [_9,_9,_9,_9];
_1 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb16 = {
RET = -(-1166785623824105929_i64);
_4 = _8 as u64;
_5 = _3;
RET = _9 as i64;
RET = 2699850232177931948_i64;
_4 = 6030242700856665891_u64 >> RET;
_5 = [_9,_9,_9,_9];
_11 = (-38_isize);
_11 = !9223372036854775807_isize;
RET = 5922862914253922126_i64 - (-5544289133930797275_i64);
_4 = 6867387285356355582_u64;
_6 = _2;
RET = 31130_u16 as i64;
_2 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
_11 = (-69_isize);
RET = !2776607082559272948_i64;
match _11 {
0 => bb9,
1 => bb4,
2 => bb11,
340282366920938463463374607431768211387 => bb13,
_ => bb12
}
}
bb17 = {
_8 = RET as f64;
_2 = _3;
_6 = [_9,_9,_9,_9];
_1 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
RET = 7452064608870063322_i64;
_14 = [145575132613863554417450230278899843819_u128,21060702908442796605631011566589063150_u128,190499474726578171820603543067331749895_u128,218544166514491160086531911808564793042_u128,97647021562386830608594678758518474423_u128,117324102136916735079323407246726012891_u128,327626724847416753961219639320494916958_u128,166852208846394503222107163675742902818_u128];
_2 = [_9,_9,_9,_9];
_5 = [_9,_9,_9,_9];
_5 = _6;
_6 = [_9,_9,_9,_9];
_6 = _7;
_5 = [_9,_9,_9,_9];
_3 = [_9,_9,_9,_9];
_11 = _12;
RET = 7905807316506985968_i64 << _4;
RET = 137425800442495985_i64;
Goto(bb20)
}
bb20 = {
Call(_19 = dump_var(3_usize, 11_usize, Move(_11), 12_usize, Move(_12), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_19 = dump_var(3_usize, 2_usize, Move(_2), 4_usize, Move(_4), 20_usize, _20, 20_usize, _20), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [bool; 2],mut _2: [i32; 4],mut _3: [char; 4],mut _4: [char; 4],mut _5: [i32; 4],mut _6: [char; 4],mut _7: [i32; 4],mut _8: [i32; 4],mut _9: [char; 4],mut _10: [i32; 4],mut _11: [i32; 4]) -> f64 {
mir! {
type RET = f64;
let _12: isize;
let _13: ([u8; 2], [isize; 2], u32, *mut f32);
let _14: *const *const isize;
let _15: u16;
let _16: [u8; 2];
let _17: bool;
let _18: &'static *mut [i32; 4];
let _19: u128;
let _20: *mut &'static u8;
let _21: *const [i32; 6];
let _22: i32;
let _23: u32;
let _24: f32;
let _25: f64;
let _26: [i32; 5];
let _27: *const [i32; 6];
let _28: (&'static u16, Adt31);
let _29: [char; 6];
let _30: [isize; 6];
let _31: isize;
let _32: u8;
let _33: f64;
let _34: usize;
let _35: u32;
let _36: &'static i128;
let _37: f64;
let _38: [isize; 2];
let _39: u16;
let _40: &'static [char; 6];
let _41: ();
let _42: ();
{
RET = 45975_u16 as f64;
_2 = [473125730_i32,(-1251862944_i32),(-878459717_i32),(-1602499848_i32)];
RET = (-37963235946076449992577594547383517290_i128) as f64;
_8 = _5;
_4 = ['\u{d885c}','\u{5068a}','\u{fd055}','\u{bdf15}'];
_9 = _6;
_4 = ['\u{950e6}','\u{698bc}','\u{236e3}','\u{6c805}'];
_11 = _2;
RET = 1105044385_i32 as f64;
_11 = [(-615371886_i32),(-1144868432_i32),469328888_i32,(-1187807857_i32)];
_13.1 = [9223372036854775807_isize,27_isize];
_13.0 = [144_u8,204_u8];
_4 = ['\u{7ad6e}','\u{7b3a5}','\u{7671}','\u{e1471}'];
_6 = ['\u{105764}','\u{bb9dc}','\u{6880f}','\u{c5c5b}'];
_12 = 1348472246_u32 as isize;
_1 = [true,true];
_11 = [(-403634890_i32),1786055963_i32,(-851146660_i32),(-1018837562_i32)];
_13.0 = [35_u8,25_u8];
_13.2 = 2250643676_u32 ^ 3945084788_u32;
_4 = _3;
Goto(bb1)
}
bb1 = {
_13.1 = [_12,_12];
_8 = [(-455269300_i32),1602095029_i32,(-1676367910_i32),1307253195_i32];
_13.1 = [_12,_12];
_9 = ['\u{93756}','\u{9042}','\u{49033}','\u{10b0f8}'];
_2 = _11;
_13.2 = 18346_i16 as u32;
_13.1 = [_12,_12];
RET = 1_u8 as f64;
_2 = _5;
RET = 9521_u16 as f64;
Goto(bb2)
}
bb2 = {
_12 = false as isize;
_7 = [(-1379915850_i32),1633058999_i32,(-665900174_i32),(-1085969641_i32)];
_17 = true;
RET = (-112643784174676409374356256509683990995_i128) as f64;
_15 = 40513_u16 | 11985_u16;
_3 = ['\u{6d93a}','\u{c8e32}','\u{92cf1}','\u{101f33}'];
_4 = _6;
_3 = ['\u{706a7}','\u{624d5}','\u{13a8d}','\u{6d7c}'];
_13.2 = 3666277582_u32;
_13.0 = [119_u8,207_u8];
_10 = _7;
_12 = 9223372036854775807_isize - (-9223372036854775808_isize);
_13.1 = [_12,_12];
_13.1 = [_12,_12];
_19 = !29412522972186468649349291623263534461_u128;
RET = 8350625628843275093_u64 as f64;
RET = 9260609394340929084_u64 as f64;
_13.2 = 2468933163540576290_i64 as u32;
_12 = -(-9223372036854775808_isize);
RET = _12 as f64;
_8 = [(-315355949_i32),(-906021912_i32),950224095_i32,(-1530126084_i32)];
_10 = [1888942935_i32,(-256551195_i32),257798073_i32,1450032236_i32];
Goto(bb3)
}
bb3 = {
_16 = [10_u8,213_u8];
_17 = false;
_17 = true;
_12 = (-9223372036854775808_isize);
_13.0 = [3_u8,190_u8];
_8 = [1291002044_i32,(-1997408374_i32),1992941971_i32,(-988046358_i32)];
_5 = [1961706252_i32,(-965682552_i32),(-326652097_i32),(-1209797209_i32)];
_13.1 = [_12,_12];
_1 = [_17,_17];
_4 = _3;
_13.0 = [205_u8,247_u8];
_9 = ['\u{1f369}','\u{974a9}','\u{10024a}','\u{49a56}'];
_9 = ['\u{a7817}','\u{4ceed}','\u{35d4d}','\u{b78bb}'];
_13.0 = _16;
_11 = [2088247098_i32,(-745228091_i32),(-1340914703_i32),355685583_i32];
RET = 8103642050506435149_usize as f64;
_3 = _6;
_8 = [(-1688321151_i32),(-32974221_i32),204420244_i32,(-1767435129_i32)];
_23 = !_13.2;
_23 = _17 as u32;
_17 = true;
_7 = [(-377749807_i32),657409319_i32,(-1213164509_i32),(-1137317767_i32)];
_9 = ['\u{20fb8}','\u{17253}','\u{30811}','\u{18017}'];
_15 = 11207932640652531209_usize as u16;
match _12 {
0 => bb1,
1 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb4 = {
_12 = false as isize;
_7 = [(-1379915850_i32),1633058999_i32,(-665900174_i32),(-1085969641_i32)];
_17 = true;
RET = (-112643784174676409374356256509683990995_i128) as f64;
_15 = 40513_u16 | 11985_u16;
_3 = ['\u{6d93a}','\u{c8e32}','\u{92cf1}','\u{101f33}'];
_4 = _6;
_3 = ['\u{706a7}','\u{624d5}','\u{13a8d}','\u{6d7c}'];
_13.2 = 3666277582_u32;
_13.0 = [119_u8,207_u8];
_10 = _7;
_12 = 9223372036854775807_isize - (-9223372036854775808_isize);
_13.1 = [_12,_12];
_13.1 = [_12,_12];
_19 = !29412522972186468649349291623263534461_u128;
RET = 8350625628843275093_u64 as f64;
RET = 9260609394340929084_u64 as f64;
_13.2 = 2468933163540576290_i64 as u32;
_12 = -(-9223372036854775808_isize);
RET = _12 as f64;
_8 = [(-315355949_i32),(-906021912_i32),950224095_i32,(-1530126084_i32)];
_10 = [1888942935_i32,(-256551195_i32),257798073_i32,1450032236_i32];
Goto(bb3)
}
bb5 = {
_13.1 = [_12,_12];
_8 = [(-455269300_i32),1602095029_i32,(-1676367910_i32),1307253195_i32];
_13.1 = [_12,_12];
_9 = ['\u{93756}','\u{9042}','\u{49033}','\u{10b0f8}'];
_2 = _11;
_13.2 = 18346_i16 as u32;
_13.1 = [_12,_12];
RET = 1_u8 as f64;
_2 = _5;
RET = 9521_u16 as f64;
Goto(bb2)
}
bb6 = {
_1 = [_17,_17];
_7 = [(-1866505668_i32),(-131357729_i32),(-788783136_i32),563793730_i32];
_11 = [(-1591345439_i32),(-1756625283_i32),(-1580614970_i32),(-2101076182_i32)];
_13.0 = _16;
_22 = (-865251175_i32);
_22 = _12 as i32;
_3 = ['\u{12672}','\u{30aac}','\u{ac0}','\u{e5af1}'];
_3 = _4;
_16 = _13.0;
Call(_13.1 = fn5(RET, _12, _4, _19, _19, _5, _6, _22, _8, _15, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = ['\u{9f39a}','\u{d0e6c}','\u{3e9c5}','\u{ceb49}'];
_8 = [_22,_22,_22,_22];
RET = 65_i8 as f64;
_25 = 0_usize as f64;
_13.3 = core::ptr::addr_of_mut!(_24);
_13.1 = [_12,_12];
_17 = _15 <= _15;
_24 = 17396263096526468645_usize as f32;
_1 = [_17,_17];
_19 = 10210_i16 as u128;
_13.0 = [59_u8,62_u8];
_16 = [239_u8,238_u8];
_2 = [_22,_22,_22,_22];
_26 = [_22,_22,_22,_22,_22];
_12 = 163070850078775233640435388078810711649_i128 as isize;
_3 = ['\u{889a1}','\u{75d40}','\u{306a4}','\u{6999e}'];
_16 = _13.0;
_17 = RET < RET;
_4 = ['\u{5d0cb}','\u{100437}','\u{108ba1}','\u{e876b}'];
Goto(bb8)
}
bb8 = {
_16 = [68_u8,124_u8];
_12 = (-6_isize);
_28.1.fld0 = 52412734056768211399684912099359834078_i128 as u32;
_30 = [_12,_12,_12,_12,_12,_12];
_28.0 = &_15;
_28.1.fld1 = [_22,_22,_22,_22];
_30 = [_12,_12,_12,_12,_12,_12];
_12 = !9223372036854775807_isize;
_9 = ['\u{10f534}','\u{dbc51}','\u{fa2bb}','\u{b31b2}'];
_31 = _12;
Call(_28.1.fld3 = fn7(Move(_28.0), _13.2, _13.0, _23, _9, _3, _10, _10, _5, _3, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_30 = [_12,_31,_12,_12,_31,_12];
Call(_4 = fn9(_16, _11, _28.1.fld3, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_22 = (-329280051_i32);
_32 = _28.1.fld0 as u8;
_7 = [_22,_22,_22,_22];
_28.1.fld2 = -20847_i16;
_28.1 = Adt31 { fld0: _13.2,fld1: _11,fld2: (-13619_i16),fld3: _16 };
RET = _25 - _25;
_32 = !42_u8;
_33 = RET;
Call(_29 = fn10(_24, _28.1.fld3, _24, Move(_13), _6, _28.1.fld3, _28.1, _28.1.fld3, _28.1.fld2, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_28.1.fld2 = (-945_i16) - 11357_i16;
_28.1.fld3 = _16;
_10 = [_22,_22,_22,_22];
_19 = _15 as u128;
_1 = [_17,_17];
_28.1 = Adt31 { fld0: _23,fld1: _10,fld2: 18098_i16,fld3: _16 };
_32 = 191_u8 | 86_u8;
_3 = ['\u{11927}','\u{ff351}','\u{84114}','\u{a12d9}'];
_34 = 3408779243798623921_usize;
_1 = [_17,_17];
Goto(bb12)
}
bb12 = {
_13.1 = [_31,_12];
_17 = !true;
_2 = [_22,_22,_22,_22];
_13.2 = !_28.1.fld0;
_28.1.fld1 = _8;
_32 = 50_u8;
_4 = ['\u{20210}','\u{4c2f6}','\u{46335}','\u{31669}'];
_24 = 141901327368128901098382098584541863375_i128 as f32;
_22 = 1825871321_i32;
_28.1.fld3 = [_32,_32];
_28.1.fld2 = 13680_i16 + 2985_i16;
_37 = RET + RET;
_30 = [_12,_31,_12,_31,_31,_31];
match _34 {
0 => bb7,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
3408779243798623921 => bb20,
_ => bb19
}
}
bb13 = {
_16 = [10_u8,213_u8];
_17 = false;
_17 = true;
_12 = (-9223372036854775808_isize);
_13.0 = [3_u8,190_u8];
_8 = [1291002044_i32,(-1997408374_i32),1992941971_i32,(-988046358_i32)];
_5 = [1961706252_i32,(-965682552_i32),(-326652097_i32),(-1209797209_i32)];
_13.1 = [_12,_12];
_1 = [_17,_17];
_4 = _3;
_13.0 = [205_u8,247_u8];
_9 = ['\u{1f369}','\u{974a9}','\u{10024a}','\u{49a56}'];
_9 = ['\u{a7817}','\u{4ceed}','\u{35d4d}','\u{b78bb}'];
_13.0 = _16;
_11 = [2088247098_i32,(-745228091_i32),(-1340914703_i32),355685583_i32];
RET = 8103642050506435149_usize as f64;
_3 = _6;
_8 = [(-1688321151_i32),(-32974221_i32),204420244_i32,(-1767435129_i32)];
_23 = !_13.2;
_23 = _17 as u32;
_17 = true;
_7 = [(-377749807_i32),657409319_i32,(-1213164509_i32),(-1137317767_i32)];
_9 = ['\u{20fb8}','\u{17253}','\u{30811}','\u{18017}'];
_15 = 11207932640652531209_usize as u16;
match _12 {
0 => bb1,
1 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
}
}
bb14 = {
_13.1 = [_12,_12];
_8 = [(-455269300_i32),1602095029_i32,(-1676367910_i32),1307253195_i32];
_13.1 = [_12,_12];
_9 = ['\u{93756}','\u{9042}','\u{49033}','\u{10b0f8}'];
_2 = _11;
_13.2 = 18346_i16 as u32;
_13.1 = [_12,_12];
RET = 1_u8 as f64;
_2 = _5;
RET = 9521_u16 as f64;
Goto(bb2)
}
bb15 = {
_30 = [_12,_31,_12,_12,_31,_12];
Call(_4 = fn9(_16, _11, _28.1.fld3, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb16 = {
_16 = [68_u8,124_u8];
_12 = (-6_isize);
_28.1.fld0 = 52412734056768211399684912099359834078_i128 as u32;
_30 = [_12,_12,_12,_12,_12,_12];
_28.0 = &_15;
_28.1.fld1 = [_22,_22,_22,_22];
_30 = [_12,_12,_12,_12,_12,_12];
_12 = !9223372036854775807_isize;
_9 = ['\u{10f534}','\u{dbc51}','\u{fa2bb}','\u{b31b2}'];
_31 = _12;
Call(_28.1.fld3 = fn7(Move(_28.0), _13.2, _13.0, _23, _9, _3, _10, _10, _5, _3, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb17 = {
_9 = ['\u{9f39a}','\u{d0e6c}','\u{3e9c5}','\u{ceb49}'];
_8 = [_22,_22,_22,_22];
RET = 65_i8 as f64;
_25 = 0_usize as f64;
_13.3 = core::ptr::addr_of_mut!(_24);
_13.1 = [_12,_12];
_17 = _15 <= _15;
_24 = 17396263096526468645_usize as f32;
_1 = [_17,_17];
_19 = 10210_i16 as u128;
_13.0 = [59_u8,62_u8];
_16 = [239_u8,238_u8];
_2 = [_22,_22,_22,_22];
_26 = [_22,_22,_22,_22,_22];
_12 = 163070850078775233640435388078810711649_i128 as isize;
_3 = ['\u{889a1}','\u{75d40}','\u{306a4}','\u{6999e}'];
_16 = _13.0;
_17 = RET < RET;
_4 = ['\u{5d0cb}','\u{100437}','\u{108ba1}','\u{e876b}'];
Goto(bb8)
}
bb18 = {
_1 = [_17,_17];
_7 = [(-1866505668_i32),(-131357729_i32),(-788783136_i32),563793730_i32];
_11 = [(-1591345439_i32),(-1756625283_i32),(-1580614970_i32),(-2101076182_i32)];
_13.0 = _16;
_22 = (-865251175_i32);
_22 = _12 as i32;
_3 = ['\u{12672}','\u{30aac}','\u{ac0}','\u{e5af1}'];
_3 = _4;
_16 = _13.0;
Call(_13.1 = fn5(RET, _12, _4, _19, _19, _5, _6, _22, _8, _15, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb19 = {
_12 = false as isize;
_7 = [(-1379915850_i32),1633058999_i32,(-665900174_i32),(-1085969641_i32)];
_17 = true;
RET = (-112643784174676409374356256509683990995_i128) as f64;
_15 = 40513_u16 | 11985_u16;
_3 = ['\u{6d93a}','\u{c8e32}','\u{92cf1}','\u{101f33}'];
_4 = _6;
_3 = ['\u{706a7}','\u{624d5}','\u{13a8d}','\u{6d7c}'];
_13.2 = 3666277582_u32;
_13.0 = [119_u8,207_u8];
_10 = _7;
_12 = 9223372036854775807_isize - (-9223372036854775808_isize);
_13.1 = [_12,_12];
_13.1 = [_12,_12];
_19 = !29412522972186468649349291623263534461_u128;
RET = 8350625628843275093_u64 as f64;
RET = 9260609394340929084_u64 as f64;
_13.2 = 2468933163540576290_i64 as u32;
_12 = -(-9223372036854775808_isize);
RET = _12 as f64;
_8 = [(-315355949_i32),(-906021912_i32),950224095_i32,(-1530126084_i32)];
_10 = [1888942935_i32,(-256551195_i32),257798073_i32,1450032236_i32];
Goto(bb3)
}
bb20 = {
_28.1.fld0 = _13.2;
_13.1 = [_31,_31];
_28.1.fld0 = _23 << _28.1.fld2;
_28.0 = &_39;
_22 = -(-1064962330_i32);
_28.1.fld1 = _5;
_26 = [_22,_22,_22,_22,_22];
_33 = _32 as f64;
Goto(bb21)
}
bb21 = {
Call(_41 = dump_var(4_usize, 6_usize, Move(_6), 3_usize, Move(_3), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_41 = dump_var(4_usize, 12_usize, Move(_12), 26_usize, Move(_26), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_41 = dump_var(4_usize, 32_usize, Move(_32), 31_usize, Move(_31), 29_usize, Move(_29), 19_usize, Move(_19)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: f64,mut _2: isize,mut _3: [char; 4],mut _4: u128,mut _5: u128,mut _6: [i32; 4],mut _7: [char; 4],mut _8: i32,mut _9: [i32; 4],mut _10: u16,mut _11: [char; 4]) -> [isize; 2] {
mir! {
type RET = [isize; 2];
let _12: ([i32; 5],);
let _13: isize;
let _14: f32;
let _15: bool;
let _16: [i16; 4];
let _17: isize;
let _18: &'static &'static i128;
let _19: isize;
let _20: &'static Adt55;
let _21: u64;
let _22: Adt41;
let _23: Adt39;
let _24: ();
let _25: ();
{
_9 = _6;
_6 = _9;
_3 = _7;
_11 = _3;
_7 = ['\u{77b70}','\u{47364}','\u{e0ea}','\u{83f}'];
_8 = !(-86053048_i32);
_8 = 244454562_i32 >> _10;
_9 = [_8,_8,_8,_8];
_8 = (-1930733938_i32) ^ (-525884000_i32);
_8 = -1650449376_i32;
RET = [_2,_2];
_9 = [_8,_8,_8,_8];
match _2 {
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
RET = [_2,_2];
_3 = ['\u{d317a}','\u{6d0b5}','\u{dbbfa}','\u{5698c}'];
_4 = _5;
_12.0 = [_8,_8,_8,_8,_8];
_11 = ['\u{8dfe}','\u{ea07b}','\u{10d271}','\u{64cda}'];
_6 = [_8,_8,_8,_8];
RET = [_2,_2];
_1 = _10 as f64;
_1 = (-89_i8) as f64;
_10 = 63342_u16;
_5 = _4;
_7 = _11;
_11 = ['\u{72689}','\u{6b7d8}','\u{a201e}','\u{3cecb}'];
RET = [_2,_2];
_6 = [_8,_8,_8,_8];
_5 = _4;
_7 = ['\u{74008}','\u{377ba}','\u{e0b42}','\u{b35d0}'];
_1 = _10 as f64;
_9 = [_8,_8,_8,_8];
_4 = _1 as u128;
_6 = [_8,_8,_8,_8];
_8 = _5 as i32;
_5 = !_4;
_9 = _6;
Call(_9 = fn6(_11, _3, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_13 = -_2;
_6 = _9;
_2 = _13;
_15 = false;
_7 = _3;
_1 = _10 as f64;
_4 = !_5;
_2 = _13;
match _10 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
63342 => bb13,
_ => bb12
}
}
bb7 = {
RET = [_2,_2];
_3 = ['\u{d317a}','\u{6d0b5}','\u{dbbfa}','\u{5698c}'];
_4 = _5;
_12.0 = [_8,_8,_8,_8,_8];
_11 = ['\u{8dfe}','\u{ea07b}','\u{10d271}','\u{64cda}'];
_6 = [_8,_8,_8,_8];
RET = [_2,_2];
_1 = _10 as f64;
_1 = (-89_i8) as f64;
_10 = 63342_u16;
_5 = _4;
_7 = _11;
_11 = ['\u{72689}','\u{6b7d8}','\u{a201e}','\u{3cecb}'];
RET = [_2,_2];
_6 = [_8,_8,_8,_8];
_5 = _4;
_7 = ['\u{74008}','\u{377ba}','\u{e0b42}','\u{b35d0}'];
_1 = _10 as f64;
_9 = [_8,_8,_8,_8];
_4 = _1 as u128;
_6 = [_8,_8,_8,_8];
_8 = _5 as i32;
_5 = !_4;
_9 = _6;
Call(_9 = fn6(_11, _3, _8), ReturnTo(bb6), UnwindUnreachable())
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
_16 = [(-25918_i16),(-20667_i16),(-31100_i16),(-25571_i16)];
_7 = ['\u{a1103}','\u{1264}','\u{4849e}','\u{13d9f}'];
_17 = !_2;
_12.0 = [_8,_8,_8,_8,_8];
_10 = 44919_u16;
_5 = _4;
RET = [_2,_13];
_9 = [_8,_8,_8,_8];
_1 = 65090050881184644601664882371826267504_i128 as f64;
_17 = (-4979819766598042068_i64) as isize;
_12.0 = [_8,_8,_8,_8,_8];
_16 = [(-29274_i16),14201_i16,16819_i16,(-19655_i16)];
_6 = [_8,_8,_8,_8];
_19 = _13 & _13;
_10 = !211_u16;
_17 = _19 ^ _13;
_16 = [29132_i16,22938_i16,(-667_i16),(-10453_i16)];
_14 = (-7294714948284536321_i64) as f32;
_12.0 = [_8,_8,_8,_8,_8];
_3 = ['\u{804f6}','\u{baffe}','\u{93f36}','\u{d8d0e}'];
_19 = _17;
_10 = 37079_u16;
_15 = true;
_3 = ['\u{60fbc}','\u{59411}','\u{5d656}','\u{6e363}'];
_7 = _3;
Goto(bb14)
}
bb14 = {
_4 = _5 | _5;
_7 = ['\u{8fd01}','\u{661a9}','\u{3c89}','\u{c6dd3}'];
_17 = _1 as isize;
_11 = ['\u{10e265}','\u{914dc}','\u{8ffe}','\u{70111}'];
_13 = _2;
_2 = -_19;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(5_usize, 4_usize, Move(_4), 15_usize, Move(_15), 12_usize, Move(_12), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(5_usize, 9_usize, Move(_9), 2_usize, Move(_2), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [char; 4],mut _2: [char; 4],mut _3: i32) -> [i32; 4] {
mir! {
type RET = [i32; 4];
let _4: &'static u16;
let _5: u32;
let _6: u32;
let _7: &'static (i32, (u8, i8), isize, u128);
let _8: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), u32, *const [u128; 8], &'static *const [u128; 8]);
let _9: [u8; 2];
let _10: i64;
let _11: char;
let _12: (i32, (u8, i8), isize, u128);
let _13: &'static i32;
let _14: (u32, &'static &'static bool, u16);
let _15: Adt32;
let _16: [i8; 8];
let _17: i8;
let _18: [bool; 8];
let _19: usize;
let _20: [isize; 6];
let _21: *mut &'static u8;
let _22: [i8; 2];
let _23: [isize; 8];
let _24: [i32; 5];
let _25: [char; 4];
let _26: i8;
let _27: ();
let _28: ();
{
RET = [_3,_3,_3,_3];
RET = [_3,_3,_3,_3];
_2 = ['\u{efd50}','\u{f6ec4}','\u{c6522}','\u{a43dd}'];
_2 = ['\u{f3855}','\u{8d053}','\u{7d113}','\u{105cab}'];
_3 = -(-1901115343_i32);
_2 = ['\u{66e5d}','\u{bcbb0}','\u{398d7}','\u{1f807}'];
_2 = ['\u{c8db3}','\u{1e45b}','\u{cd7f3}','\u{3e5be}'];
_1 = _2;
RET = [_3,_3,_3,_3];
RET = [_3,_3,_3,_3];
_2 = _1;
RET = [_3,_3,_3,_3];
Goto(bb1)
}
bb1 = {
RET = [_3,_3,_3,_3];
_6 = 807138736_u32 - 1374766622_u32;
_6 = 69_u8 as u32;
_8.2 = core::ptr::addr_of!(_8.0.0);
_8.2 = core::ptr::addr_of!(_8.0.0);
_5 = _6 << _3;
_8.3 = &_8.2;
_8.1 = _5;
_8.0.0 = [297714536730699513557074734562283441853_u128,235698599510677963627204597947051488594_u128,187339681766734303397811394304347313514_u128,177400163819095509862054213206804286689_u128,184379355982574944875236984481343145034_u128,243771129540527702077468365364758664717_u128,249948248683368419682547527024456897628_u128,283942568243417819960484205787909753300_u128];
_9 = [168_u8,30_u8];
_9 = [187_u8,125_u8];
RET = [_3,_3,_3,_3];
_1 = _2;
_9 = [217_u8,201_u8];
_9 = [99_u8,82_u8];
_8.1 = false as u32;
Call(_10 = core::intrinsics::bswap(79421224330095652_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.0.0 = [112629559301690473485844533494929332591_u128,152499181352815518465646440413768735686_u128,175408162919152810705002340483736263967_u128,279901004320987386077989965138533514615_u128,180881833702028569985810081146077482765_u128,227387800763228880052349141737116112041_u128,126047095704278443400773203308979336103_u128,238030087887129623679627047913764420211_u128];
_12.3 = 113592197210208626155308380312454709751_u128 - 6516407206598830842791336020402581660_u128;
_12.2 = 9223372036854775807_isize;
_3 = (-1561507307_i32);
_12.1 = (84_u8, (-120_i8));
_8.0.2 = &_13;
_6 = !_5;
_8.0.1 = core::ptr::addr_of!(_12.1);
_13 = &_12.0;
_11 = '\u{3c092}';
_12.1.1 = (-27_i8) - (-22_i8);
_12.0 = _3 & _3;
_12.2 = _12.3 as isize;
_8.1 = !_5;
match _12.1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
84 => bb6,
_ => bb5
}
}
bb3 = {
RET = [_3,_3,_3,_3];
_6 = 807138736_u32 - 1374766622_u32;
_6 = 69_u8 as u32;
_8.2 = core::ptr::addr_of!(_8.0.0);
_8.2 = core::ptr::addr_of!(_8.0.0);
_5 = _6 << _3;
_8.3 = &_8.2;
_8.1 = _5;
_8.0.0 = [297714536730699513557074734562283441853_u128,235698599510677963627204597947051488594_u128,187339681766734303397811394304347313514_u128,177400163819095509862054213206804286689_u128,184379355982574944875236984481343145034_u128,243771129540527702077468365364758664717_u128,249948248683368419682547527024456897628_u128,283942568243417819960484205787909753300_u128];
_9 = [168_u8,30_u8];
_9 = [187_u8,125_u8];
RET = [_3,_3,_3,_3];
_1 = _2;
_9 = [217_u8,201_u8];
_9 = [99_u8,82_u8];
_8.1 = false as u32;
Call(_10 = core::intrinsics::bswap(79421224330095652_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_12.1 = (241_u8, 103_i8);
_8.0.1 = core::ptr::addr_of!(_12.1);
_12.2 = (-51_isize) ^ 9223372036854775807_isize;
_14.0 = !_8.1;
_12.0 = _3 | _3;
_13 = &_3;
_12.1.1 = _8.1 as i8;
_1 = [_11,_11,_11,_11];
_8.0.1 = core::ptr::addr_of!(_12.1);
Goto(bb7)
}
bb7 = {
_5 = !_6;
_14.2 = 3310_u16;
Goto(bb8)
}
bb8 = {
RET = [_12.0,(*_13),_3,_12.0];
_7 = &_12;
RET = [(*_13),(*_13),_3,_12.0];
_12.0 = 1244417504581503472765642775084332608_i128 as i32;
_14.0 = 7181074051629979305_u64 as u32;
_2 = [_11,_11,_11,_11];
RET = [_12.0,(*_13),_12.0,(*_13)];
_2 = [_11,_11,_11,_11];
_7 = &_12;
_7 = &(*_7);
_8.3 = &_8.2;
match (*_7).1.0 {
0 => bb4,
241 => bb9,
_ => bb5
}
}
bb9 = {
_1 = [_11,_11,_11,_11];
_12.1 = (111_u8, (-100_i8));
_7 = &_12;
_6 = _5 ^ _8.1;
_17 = -(*_7).1.1;
_10 = 7926995093583941487_i64 << _12.3;
_8.0.1 = core::ptr::addr_of!((*_7).1);
_23 = [_12.2,(*_7).2,(*_7).2,_12.2,(*_7).2,(*_7).2,(*_7).2,_12.2];
_19 = _17 as usize;
_17 = (*_7).1.1 * (*_7).1.1;
_16 = [(*_7).1.1,(*_7).1.1,_17,_17,_12.1.1,_17,_17,_12.1.1];
_8.3 = &_8.2;
_10 = (-2665448771930761498_i64) << (*_7).3;
_11 = '\u{af7b}';
_20 = [_12.2,(*_7).2,(*_7).2,(*_7).2,(*_7).2,_12.2];
match _14.2 {
0 => bb8,
1 => bb10,
2 => bb11,
3 => bb12,
3310 => bb14,
_ => bb13
}
}
bb10 = {
RET = [_3,_3,_3,_3];
_6 = 807138736_u32 - 1374766622_u32;
_6 = 69_u8 as u32;
_8.2 = core::ptr::addr_of!(_8.0.0);
_8.2 = core::ptr::addr_of!(_8.0.0);
_5 = _6 << _3;
_8.3 = &_8.2;
_8.1 = _5;
_8.0.0 = [297714536730699513557074734562283441853_u128,235698599510677963627204597947051488594_u128,187339681766734303397811394304347313514_u128,177400163819095509862054213206804286689_u128,184379355982574944875236984481343145034_u128,243771129540527702077468365364758664717_u128,249948248683368419682547527024456897628_u128,283942568243417819960484205787909753300_u128];
_9 = [168_u8,30_u8];
_9 = [187_u8,125_u8];
RET = [_3,_3,_3,_3];
_1 = _2;
_9 = [217_u8,201_u8];
_9 = [99_u8,82_u8];
_8.1 = false as u32;
Call(_10 = core::intrinsics::bswap(79421224330095652_i64), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_8.0.0 = [112629559301690473485844533494929332591_u128,152499181352815518465646440413768735686_u128,175408162919152810705002340483736263967_u128,279901004320987386077989965138533514615_u128,180881833702028569985810081146077482765_u128,227387800763228880052349141737116112041_u128,126047095704278443400773203308979336103_u128,238030087887129623679627047913764420211_u128];
_12.3 = 113592197210208626155308380312454709751_u128 - 6516407206598830842791336020402581660_u128;
_12.2 = 9223372036854775807_isize;
_3 = (-1561507307_i32);
_12.1 = (84_u8, (-120_i8));
_8.0.2 = &_13;
_6 = !_5;
_8.0.1 = core::ptr::addr_of!(_12.1);
_13 = &_12.0;
_11 = '\u{3c092}';
_12.1.1 = (-27_i8) - (-22_i8);
_12.0 = _3 & _3;
_12.2 = _12.3 as isize;
_8.1 = !_5;
match _12.1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
84 => bb6,
_ => bb5
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_12.2 = !12_isize;
_16 = [_17,_17,(*_7).1.1,_17,_17,_12.1.1,(*_7).1.1,(*_7).1.1];
_8.0.2 = &_13;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(6_usize, 10_usize, Move(_10), 23_usize, Move(_23), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(6_usize, 5_usize, Move(_5), 6_usize, Move(_6), 17_usize, Move(_17), 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: &'static u16,mut _2: u32,mut _3: [u8; 2],mut _4: u32,mut _5: [char; 4],mut _6: [char; 4],mut _7: [i32; 4],mut _8: [i32; 4],mut _9: [i32; 4],mut _10: [char; 4],mut _11: [char; 4]) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _12: usize;
let _13: *const [u128; 8];
let _14: isize;
let _15: &'static &'static &'static bool;
let _16: [i32; 4];
let _17: &'static &'static bool;
let _18: *mut *const (u8, i8);
let _19: f32;
let _20: &'static [i32; 5];
let _21: *const isize;
let _22: &'static &'static bool;
let _23: [u64; 6];
let _24: char;
let _25: u32;
let _26: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), u32, *const [u128; 8], &'static *const [u128; 8]);
let _27: f64;
let _28: bool;
let _29: &'static [char; 6];
let _30: isize;
let _31: i128;
let _32: [char; 6];
let _33: [bool; 8];
let _34: &'static ([i32; 5],);
let _35: &'static *const [u128; 8];
let _36: *mut f64;
let _37: Adt32;
let _38: &'static i64;
let _39: bool;
let _40: &'static i32;
let _41: &'static i64;
let _42: i128;
let _43: [i8; 8];
let _44: ();
let _45: ();
{
_4 = !_2;
_7 = [(-676328738_i32),350749879_i32,(-1942085782_i32),1792987712_i32];
RET = [9_u8,249_u8];
_7 = _9;
_7 = _9;
_5 = ['\u{3861e}','\u{d3be8}','\u{51806}','\u{4cfb2}'];
RET = [59_u8,189_u8];
_9 = [(-183521940_i32),138468113_i32,2142809526_i32,(-599247834_i32)];
_10 = ['\u{6d58c}','\u{1005bb}','\u{4b1db}','\u{1282c}'];
RET = [30_u8,92_u8];
_7 = [(-2121148700_i32),(-106832705_i32),(-1837171268_i32),(-406904849_i32)];
_12 = 104_i8 as usize;
_5 = ['\u{4a672}','\u{757c2}','\u{2c557}','\u{2f6a2}'];
_7 = [526194764_i32,657569876_i32,1405352510_i32,(-214301768_i32)];
RET = _3;
_7 = [(-1700158562_i32),(-938239719_i32),587400465_i32,871094127_i32];
_11 = ['\u{105d0f}','\u{b2e58}','\u{55829}','\u{849ee}'];
_7 = _9;
_5 = _10;
_14 = !(-9223372036854775808_isize);
_4 = _2;
_11 = _10;
_16 = _8;
Goto(bb1)
}
bb1 = {
_6 = ['\u{68594}','\u{71942}','\u{e61cf}','\u{42c7b}'];
_12 = 4_usize;
_4 = _2;
_11 = ['\u{36898}','\u{1a0f8}','\u{be7ef}','\u{54c28}'];
_11 = ['\u{2f1c2}','\u{89f66}','\u{ab797}','\u{966f1}'];
_7 = _16;
RET = [19_u8,205_u8];
_9 = [(-781380535_i32),(-897703380_i32),960901840_i32,1001373213_i32];
_3 = [210_u8,12_u8];
_2 = _4 | _4;
_19 = 261984404044286434720814016799997139831_u128 as f32;
_9 = [(-21132281_i32),1004861897_i32,(-1713919433_i32),(-986179465_i32)];
_15 = &_17;
_9 = _7;
_4 = !_2;
_2 = _4;
_14 = (-9223372036854775808_isize) << _12;
_7 = [(-747603878_i32),(-557053137_i32),1789244837_i32,(-1522915311_i32)];
Goto(bb2)
}
bb2 = {
_6 = ['\u{48ebe}','\u{4eef6}','\u{4ffb}','\u{49eb2}'];
_14 = -9223372036854775807_isize;
RET = [251_u8,227_u8];
_10 = ['\u{a2122}','\u{53562}','\u{e1fcc}','\u{c6408}'];
_9 = [(-151454439_i32),2121661805_i32,(-1482406816_i32),(-1825152517_i32)];
_9 = [388696518_i32,(-1882294557_i32),(-2014591272_i32),1557163359_i32];
_8 = [410474339_i32,(-1924047009_i32),(-1199894308_i32),724528135_i32];
_10 = ['\u{ddae8}','\u{c5085}','\u{25cbb}','\u{9d805}'];
_7 = _8;
_12 = 8667437188280738398_usize * 5_usize;
_21 = core::ptr::addr_of!(_14);
_5 = ['\u{9d603}','\u{a94}','\u{ae5ec}','\u{9e3e}'];
_2 = _4;
(*_21) = 9223372036854775807_isize >> _4;
_21 = core::ptr::addr_of!(_14);
_15 = &_22;
_15 = &_17;
Goto(bb3)
}
bb3 = {
_5 = _6;
_19 = _4 as f32;
_9 = [1977350676_i32,424655750_i32,1586216436_i32,2122731266_i32];
RET = [126_u8,164_u8];
_21 = core::ptr::addr_of!(_14);
_3 = [144_u8,15_u8];
_14 = 12_isize - 9223372036854775807_isize;
RET = [35_u8,45_u8];
_24 = '\u{b7525}';
(*_21) = (-9223372036854775808_isize) - 81_isize;
_25 = _4;
_6 = [_24,_24,_24,_24];
_7 = [1755604162_i32,238645897_i32,157085730_i32,1595127868_i32];
_26.0.0 = [302876801553966948280093185077610292475_u128,256256336227777004304321602033430223942_u128,134762163650238397329467384177718533161_u128,215058339545076356824060191137270645547_u128,261436709248675930124481745540273776310_u128,112733927806312757912574733861036367408_u128,215915751430367740522667332068175511604_u128,186675566542170755640249089748923274807_u128];
_13 = core::ptr::addr_of!(_26.0.0);
_13 = core::ptr::addr_of!(_26.0.0);
Call(_2 = core::intrinsics::bswap(_25), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_21) = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
Goto(bb5)
}
bb5 = {
_13 = core::ptr::addr_of!((*_13));
_26.0.0 = [30524166360907098595813541983145926562_u128,153575342541988403094982866763578867844_u128,83549374226435541568785676556502206518_u128,336074478557232178107860289548827265434_u128,336791173720329662725820210933141554685_u128,239119984240257906323747938426729022577_u128,138786895324183792960961034081976953466_u128,184745223801846647547312874877754753050_u128];
_26.2 = Move(_13);
_9 = [1353879576_i32,(-953412214_i32),(-5694696_i32),1848680079_i32];
_18 = core::ptr::addr_of_mut!(_26.0.1);
_5 = [_24,_24,_24,_24];
RET = _3;
Call(RET = fn8(_7, _4, Move(_26.2), _11, _16, _11, _9, Move(_21)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_29 = &_32;
_6 = _5;
_3 = [97_u8,89_u8];
_26.0.0 = [221442810848639431408042570650586079520_u128,290556355785572575963510102608266849348_u128,23928928939816147417175710435647998773_u128,209288518105888304300606703951543722336_u128,63398777682253488976334982382623449599_u128,136168571726608032532964873625409930514_u128,6923491702604259665188978894000272912_u128,214735078733539559790072165693952584026_u128];
_15 = &(*_15);
RET = [137_u8,199_u8];
_28 = false;
_8 = _7;
_31 = (-101445813472121674123336096082067247680_i128);
_27 = (-38_i8) as f64;
_29 = &(*_29);
_9 = [(-1650716502_i32),(-807497973_i32),2047820991_i32,1351518804_i32];
_5 = _11;
_21 = core::ptr::addr_of!(_30);
_25 = _4;
_7 = _16;
_26.3 = &_13;
_15 = &_22;
_29 = &_32;
_29 = &(*_29);
_35 = &_26.2;
RET = [171_u8,85_u8];
match _31 {
0 => bb3,
1 => bb7,
238836553448816789340038511349700963776 => bb9,
_ => bb8
}
}
bb7 = {
_6 = ['\u{48ebe}','\u{4eef6}','\u{4ffb}','\u{49eb2}'];
_14 = -9223372036854775807_isize;
RET = [251_u8,227_u8];
_10 = ['\u{a2122}','\u{53562}','\u{e1fcc}','\u{c6408}'];
_9 = [(-151454439_i32),2121661805_i32,(-1482406816_i32),(-1825152517_i32)];
_9 = [388696518_i32,(-1882294557_i32),(-2014591272_i32),1557163359_i32];
_8 = [410474339_i32,(-1924047009_i32),(-1199894308_i32),724528135_i32];
_10 = ['\u{ddae8}','\u{c5085}','\u{25cbb}','\u{9d805}'];
_7 = _8;
_12 = 8667437188280738398_usize * 5_usize;
_21 = core::ptr::addr_of!(_14);
_5 = ['\u{9d603}','\u{a94}','\u{ae5ec}','\u{9e3e}'];
_2 = _4;
(*_21) = 9223372036854775807_isize >> _4;
_21 = core::ptr::addr_of!(_14);
_15 = &_22;
_15 = &_17;
Goto(bb3)
}
bb8 = {
_6 = ['\u{68594}','\u{71942}','\u{e61cf}','\u{42c7b}'];
_12 = 4_usize;
_4 = _2;
_11 = ['\u{36898}','\u{1a0f8}','\u{be7ef}','\u{54c28}'];
_11 = ['\u{2f1c2}','\u{89f66}','\u{ab797}','\u{966f1}'];
_7 = _16;
RET = [19_u8,205_u8];
_9 = [(-781380535_i32),(-897703380_i32),960901840_i32,1001373213_i32];
_3 = [210_u8,12_u8];
_2 = _4 | _4;
_19 = 261984404044286434720814016799997139831_u128 as f32;
_9 = [(-21132281_i32),1004861897_i32,(-1713919433_i32),(-986179465_i32)];
_15 = &_17;
_9 = _7;
_4 = !_2;
_2 = _4;
_14 = (-9223372036854775808_isize) << _12;
_7 = [(-747603878_i32),(-557053137_i32),1789244837_i32,(-1522915311_i32)];
Goto(bb2)
}
bb9 = {
_39 = !_28;
match _31 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
238836553448816789340038511349700963776 => bb15,
_ => bb14
}
}
bb10 = {
_6 = ['\u{68594}','\u{71942}','\u{e61cf}','\u{42c7b}'];
_12 = 4_usize;
_4 = _2;
_11 = ['\u{36898}','\u{1a0f8}','\u{be7ef}','\u{54c28}'];
_11 = ['\u{2f1c2}','\u{89f66}','\u{ab797}','\u{966f1}'];
_7 = _16;
RET = [19_u8,205_u8];
_9 = [(-781380535_i32),(-897703380_i32),960901840_i32,1001373213_i32];
_3 = [210_u8,12_u8];
_2 = _4 | _4;
_19 = 261984404044286434720814016799997139831_u128 as f32;
_9 = [(-21132281_i32),1004861897_i32,(-1713919433_i32),(-986179465_i32)];
_15 = &_17;
_9 = _7;
_4 = !_2;
_2 = _4;
_14 = (-9223372036854775808_isize) << _12;
_7 = [(-747603878_i32),(-557053137_i32),1789244837_i32,(-1522915311_i32)];
Goto(bb2)
}
bb11 = {
_6 = ['\u{48ebe}','\u{4eef6}','\u{4ffb}','\u{49eb2}'];
_14 = -9223372036854775807_isize;
RET = [251_u8,227_u8];
_10 = ['\u{a2122}','\u{53562}','\u{e1fcc}','\u{c6408}'];
_9 = [(-151454439_i32),2121661805_i32,(-1482406816_i32),(-1825152517_i32)];
_9 = [388696518_i32,(-1882294557_i32),(-2014591272_i32),1557163359_i32];
_8 = [410474339_i32,(-1924047009_i32),(-1199894308_i32),724528135_i32];
_10 = ['\u{ddae8}','\u{c5085}','\u{25cbb}','\u{9d805}'];
_7 = _8;
_12 = 8667437188280738398_usize * 5_usize;
_21 = core::ptr::addr_of!(_14);
_5 = ['\u{9d603}','\u{a94}','\u{ae5ec}','\u{9e3e}'];
_2 = _4;
(*_21) = 9223372036854775807_isize >> _4;
_21 = core::ptr::addr_of!(_14);
_15 = &_22;
_15 = &_17;
Goto(bb3)
}
bb12 = {
_5 = _6;
_19 = _4 as f32;
_9 = [1977350676_i32,424655750_i32,1586216436_i32,2122731266_i32];
RET = [126_u8,164_u8];
_21 = core::ptr::addr_of!(_14);
_3 = [144_u8,15_u8];
_14 = 12_isize - 9223372036854775807_isize;
RET = [35_u8,45_u8];
_24 = '\u{b7525}';
(*_21) = (-9223372036854775808_isize) - 81_isize;
_25 = _4;
_6 = [_24,_24,_24,_24];
_7 = [1755604162_i32,238645897_i32,157085730_i32,1595127868_i32];
_26.0.0 = [302876801553966948280093185077610292475_u128,256256336227777004304321602033430223942_u128,134762163650238397329467384177718533161_u128,215058339545076356824060191137270645547_u128,261436709248675930124481745540273776310_u128,112733927806312757912574733861036367408_u128,215915751430367740522667332068175511604_u128,186675566542170755640249089748923274807_u128];
_13 = core::ptr::addr_of!(_26.0.0);
_13 = core::ptr::addr_of!(_26.0.0);
Call(_2 = core::intrinsics::bswap(_25), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
_13 = core::ptr::addr_of!((*_13));
_26.0.0 = [30524166360907098595813541983145926562_u128,153575342541988403094982866763578867844_u128,83549374226435541568785676556502206518_u128,336074478557232178107860289548827265434_u128,336791173720329662725820210933141554685_u128,239119984240257906323747938426729022577_u128,138786895324183792960961034081976953466_u128,184745223801846647547312874877754753050_u128];
_26.2 = Move(_13);
_9 = [1353879576_i32,(-953412214_i32),(-5694696_i32),1848680079_i32];
_18 = core::ptr::addr_of_mut!(_26.0.1);
_5 = [_24,_24,_24,_24];
RET = _3;
Call(RET = fn8(_7, _4, Move(_26.2), _11, _16, _11, _9, Move(_21)), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
(*_21) = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
Goto(bb5)
}
bb15 = {
_28 = _39 & _39;
_5 = [_24,_24,_24,_24];
_8 = [(-2011354283_i32),(-851655213_i32),298823071_i32,1754356953_i32];
_8 = _16;
(*_21) = -_14;
_29 = &(*_29);
_36 = core::ptr::addr_of_mut!(_27);
_36 = core::ptr::addr_of_mut!(_27);
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(7_usize, 8_usize, Move(_8), 4_usize, Move(_4), 3_usize, Move(_3), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(7_usize, 10_usize, Move(_10), 5_usize, Move(_5), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(7_usize, 39_usize, Move(_39), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [i32; 4],mut _2: u32,mut _3: *const [u128; 8],mut _4: [char; 4],mut _5: [i32; 4],mut _6: [char; 4],mut _7: [i32; 4],mut _8: *const isize) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _9: *mut f32;
let _10: isize;
let _11: u16;
let _12: &'static *mut [i32; 4];
let _13: f32;
let _14: usize;
let _15: u32;
let _16: *mut f32;
let _17: u64;
let _18: u32;
let _19: u64;
let _20: f32;
let _21: &'static f32;
let _22: isize;
let _23: &'static &'static bool;
let _24: bool;
let _25: (&'static u16, Adt31);
let _26: i8;
let _27: u32;
let _28: ();
let _29: ();
{
RET = [68_u8,84_u8];
_6 = ['\u{5bb0d}','\u{f844}','\u{32427}','\u{10f3c5}'];
_8 = core::ptr::addr_of!(_10);
_4 = ['\u{d71f9}','\u{dd60f}','\u{f6d}','\u{90aa6}'];
_11 = !27064_u16;
_9 = core::ptr::addr_of_mut!(_13);
_14 = 11061145475024641900_usize & 4955760104178380593_usize;
_10 = !(-9223372036854775808_isize);
(*_9) = (-1334165890874751808_i64) as f32;
_11 = 19236_u16 ^ 57785_u16;
_10 = !(-9223372036854775808_isize);
_15 = _2 << _2;
Goto(bb1)
}
bb1 = {
_8 = core::ptr::addr_of!(_10);
_15 = _2 >> _10;
_1 = [(-747572345_i32),1784600511_i32,(-1706783902_i32),889973671_i32];
_2 = !_15;
_15 = !_2;
RET = [94_u8,229_u8];
_1 = [1524177018_i32,(-1646326166_i32),583735453_i32,(-653120468_i32)];
(*_9) = 614743791062161093_i64 as f32;
_8 = core::ptr::addr_of!(_10);
(*_8) = !(-9223372036854775808_isize);
_10 = !9223372036854775807_isize;
_1 = [(-979744239_i32),(-401374350_i32),(-823259178_i32),(-1656833863_i32)];
(*_9) = (-65_i8) as f32;
_11 = !20577_u16;
(*_9) = (*_8) as f32;
(*_8) = 101_isize & (-9223372036854775808_isize);
_11 = 5809291358891326257_i64 as u16;
_16 = core::ptr::addr_of_mut!((*_9));
_20 = (*_16) + (*_16);
_17 = 11294409802563884374_u64;
_5 = _7;
match _17 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
11294409802563884374 => bb7,
_ => bb6
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
(*_8) = (-202_i16) as isize;
_22 = _10 ^ _10;
Goto(bb8)
}
bb8 = {
_19 = _17;
_9 = Move(_16);
RET = [205_u8,63_u8];
RET = [116_u8,68_u8];
_21 = &_20;
_2 = !_15;
_1 = [(-1390956034_i32),335628335_i32,(-113238571_i32),(-714734808_i32)];
Goto(bb9)
}
bb9 = {
_1 = [783888327_i32,2127119329_i32,1888241036_i32,(-1592554399_i32)];
_24 = _2 > _15;
_5 = [(-333625447_i32),1122618994_i32,(-899095042_i32),(-1568041526_i32)];
match _17 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb10,
4 => bb11,
11294409802563884374 => bb13,
_ => bb12
}
}
bb10 = {
_8 = core::ptr::addr_of!(_10);
_15 = _2 >> _10;
_1 = [(-747572345_i32),1784600511_i32,(-1706783902_i32),889973671_i32];
_2 = !_15;
_15 = !_2;
RET = [94_u8,229_u8];
_1 = [1524177018_i32,(-1646326166_i32),583735453_i32,(-653120468_i32)];
(*_9) = 614743791062161093_i64 as f32;
_8 = core::ptr::addr_of!(_10);
(*_8) = !(-9223372036854775808_isize);
_10 = !9223372036854775807_isize;
_1 = [(-979744239_i32),(-401374350_i32),(-823259178_i32),(-1656833863_i32)];
(*_9) = (-65_i8) as f32;
_11 = !20577_u16;
(*_9) = (*_8) as f32;
(*_8) = 101_isize & (-9223372036854775808_isize);
_11 = 5809291358891326257_i64 as u16;
_16 = core::ptr::addr_of_mut!((*_9));
_20 = (*_16) + (*_16);
_17 = 11294409802563884374_u64;
_5 = _7;
match _17 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
11294409802563884374 => bb7,
_ => bb6
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_24 = !true;
_21 = &(*_21);
_25.1.fld0 = !_2;
_10 = _22 + _22;
_25.1.fld0 = !_15;
_25.1.fld3 = [65_u8,153_u8];
match _19 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb12,
11294409802563884374 => bb15,
_ => bb14
}
}
bb14 = {
_8 = core::ptr::addr_of!(_10);
_15 = _2 >> _10;
_1 = [(-747572345_i32),1784600511_i32,(-1706783902_i32),889973671_i32];
_2 = !_15;
_15 = !_2;
RET = [94_u8,229_u8];
_1 = [1524177018_i32,(-1646326166_i32),583735453_i32,(-653120468_i32)];
(*_9) = 614743791062161093_i64 as f32;
_8 = core::ptr::addr_of!(_10);
(*_8) = !(-9223372036854775808_isize);
_10 = !9223372036854775807_isize;
_1 = [(-979744239_i32),(-401374350_i32),(-823259178_i32),(-1656833863_i32)];
(*_9) = (-65_i8) as f32;
_11 = !20577_u16;
(*_9) = (*_8) as f32;
(*_8) = 101_isize & (-9223372036854775808_isize);
_11 = 5809291358891326257_i64 as u16;
_16 = core::ptr::addr_of_mut!((*_9));
_20 = (*_16) + (*_16);
_17 = 11294409802563884374_u64;
_5 = _7;
match _17 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
11294409802563884374 => bb7,
_ => bb6
}
}
bb15 = {
_19 = _20 as u64;
RET = [45_u8,148_u8];
_8 = core::ptr::addr_of!((*_8));
_25.1.fld2 = !7915_i16;
(*_8) = -_22;
_14 = 22_i8 as usize;
_2 = _15;
(*_8) = _22;
_8 = core::ptr::addr_of!(_10);
_26 = (-69_i8) | 43_i8;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(8_usize, 1_usize, Move(_1), 2_usize, Move(_2), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(8_usize, 15_usize, Move(_15), 19_usize, Move(_19), 10_usize, Move(_10), 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [u8; 2],mut _2: [i32; 4],mut _3: [u8; 2],mut _4: [i32; 4]) -> [char; 4] {
mir! {
type RET = [char; 4];
let _5: &'static &'static bool;
let _6: u16;
let _7: char;
let _8: *mut *const (u8, i8);
let _9: f64;
let _10: *const *const isize;
let _11: [i64; 3];
let _12: u32;
let _13: f64;
let _14: *const [i32; 6];
let _15: &'static ([i32; 5],);
let _16: ();
let _17: ();
{
RET = ['\u{1c1cf}','\u{25417}','\u{adba4}','\u{c8cf0}'];
RET = ['\u{b49f4}','\u{361f2}','\u{8f138}','\u{5793f}'];
_3 = [205_u8,226_u8];
RET = ['\u{5a43c}','\u{c08a1}','\u{faa5d}','\u{6cd6a}'];
RET = ['\u{1059b9}','\u{fa8b6}','\u{5e7d1}','\u{edba8}'];
_1 = [233_u8,73_u8];
_4 = [(-1488792320_i32),(-1156396501_i32),285902865_i32,(-1240385542_i32)];
RET = ['\u{de4df}','\u{533e8}','\u{106584}','\u{4df65}'];
_1 = [101_u8,48_u8];
_6 = !44314_u16;
RET = ['\u{67fe}','\u{2e7d4}','\u{8f7cc}','\u{1dec6}'];
_1 = [94_u8,244_u8];
RET = ['\u{c2e39}','\u{a8922}','\u{21200}','\u{da3a3}'];
_7 = '\u{4036c}';
_3 = [196_u8,66_u8];
RET = [_7,_7,_7,_7];
RET = [_7,_7,_7,_7];
_7 = '\u{3da40}';
RET = [_7,_7,_7,_7];
_6 = 29512_u16;
_1 = _3;
_7 = '\u{10fa3d}';
_1 = _3;
_3 = [139_u8,123_u8];
Goto(bb1)
}
bb1 = {
_2 = [1660950993_i32,(-743099240_i32),(-1837772008_i32),(-1096145830_i32)];
_1 = _3;
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
29512 => bb10,
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
_3 = _1;
_2 = _4;
_4 = [(-1406177089_i32),(-336829443_i32),1966988451_i32,674875369_i32];
_9 = 44_isize as f64;
_6 = false as u16;
_9 = 9443809613652392738_u64 as f64;
_11 = [6323375813072012999_i64,(-3576928939542000587_i64),1630918914844571614_i64];
_11 = [142197393018744714_i64,(-5015608369109102957_i64),8499668056673867249_i64];
_4 = [2092749877_i32,(-212594330_i32),1691206415_i32,1997543064_i32];
_1 = [252_u8,28_u8];
_13 = -_9;
_6 = 9292_u16;
RET = [_7,_7,_7,_7];
_12 = 2928336089_u32;
_6 = 63762_u16 * 16176_u16;
_7 = '\u{862a2}';
_6 = 196_u8 as u16;
match _12 {
0 => bb7,
1 => bb8,
2 => bb11,
2928336089 => bb13,
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
_2 = _4;
_7 = '\u{1fff5}';
_11 = [6810911310686754779_i64,(-7632168437857591759_i64),(-8139097726472844832_i64)];
_7 = '\u{595f9}';
_9 = 7_usize as f64;
_2 = [177307182_i32,(-2076638176_i32),(-1701017730_i32),(-1036899319_i32)];
_7 = '\u{2560b}';
_12 = !1229404153_u32;
RET = [_7,_7,_7,_7];
_6 = (-139477647804584954983390708933159197659_i128) as u16;
_9 = _13;
_11 = [197792058448788210_i64,(-6618454402611795185_i64),163926894006501623_i64];
_11 = [(-7190278964503801019_i64),(-4367274301987388511_i64),(-3255787399006139322_i64)];
_3 = _1;
_12 = 1818497858_u32;
_3 = [250_u8,89_u8];
_4 = _2;
_7 = '\u{63a4a}';
_2 = [421009088_i32,1792484293_i32,2113147002_i32,(-361509176_i32)];
_7 = '\u{9d9c0}';
_7 = '\u{ccd40}';
_6 = !5094_u16;
RET = [_7,_7,_7,_7];
_3 = _1;
match _12 {
0 => bb1,
1 => bb10,
2 => bb11,
3 => bb7,
4 => bb5,
5 => bb6,
1818497858 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_6 = !64071_u16;
_12 = !2634222882_u32;
RET = [_7,_7,_7,_7];
_11 = [(-4653631509271879683_i64),524327349223699637_i64,(-489652688273130328_i64)];
_9 = _13 + _13;
RET = [_7,_7,_7,_7];
_1 = _3;
_1 = _3;
RET = [_7,_7,_7,_7];
_6 = 9223372036854775807_isize as u16;
_9 = _13;
RET = [_7,_7,_7,_7];
_11 = [(-8250751882793824840_i64),(-1831810173317090501_i64),1543144837880620787_i64];
_3 = _1;
_1 = _3;
_12 = !3863889917_u32;
RET = [_7,_7,_7,_7];
_6 = 20140_u16;
_12 = 115469300_u32;
_12 = !2968470648_u32;
Goto(bb16)
}
bb16 = {
Call(_16 = dump_var(9_usize, 11_usize, Move(_11), 7_usize, Move(_7), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: f32,mut _2: [u8; 2],mut _3: f32,mut _4: ([u8; 2], [isize; 2], u32, *mut f32),mut _5: [char; 4],mut _6: [u8; 2],mut _7: Adt31,mut _8: [u8; 2],mut _9: i16,mut _10: [i32; 4]) -> [char; 6] {
mir! {
type RET = [char; 6];
let _11: f32;
let _12: [isize; 3];
let _13: u128;
let _14: isize;
let _15: isize;
let _16: [i32; 6];
let _17: f32;
let _18: char;
let _19: [i16; 4];
let _20: *mut [i32; 4];
let _21: f64;
let _22: &'static (i32, (u8, i8), isize, u128);
let _23: ();
let _24: ();
{
_2 = _7.fld3;
_4.3 = core::ptr::addr_of_mut!(_1);
_10 = [1023041746_i32,1076229621_i32,298690110_i32,1223757074_i32];
RET = ['\u{391a9}','\u{81806}','\u{688bc}','\u{3adb6}','\u{8b7c1}','\u{1043e}'];
_8 = [75_u8,249_u8];
_3 = _1 - _1;
RET = ['\u{e4831}','\u{2b329}','\u{88e2d}','\u{33544}','\u{972b}','\u{f87f7}'];
RET = ['\u{7666b}','\u{8e314}','\u{709aa}','\u{2eb26}','\u{9815e}','\u{9cc4b}'];
_7.fld0 = !_4.2;
_7.fld2 = -_9;
_11 = (-3703272820794355773_i64) as f32;
_11 = _1;
_6 = [67_u8,219_u8];
_8 = _2;
_7.fld0 = _4.2;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize),75_isize];
RET = ['\u{10b37e}','\u{52424}','\u{5518c}','\u{3af26}','\u{ce826}','\u{b2848}'];
_7.fld1 = [(-1752378493_i32),(-1714978700_i32),1676216935_i32,(-241892676_i32)];
_8 = [96_u8,224_u8];
_7.fld1 = _10;
_7.fld1 = [1756830695_i32,1462899723_i32,(-2035564082_i32),(-493068483_i32)];
_13 = 330376176191971823869183382018758635822_u128;
RET = ['\u{8aabe}','\u{5dd16}','\u{4717a}','\u{b8b12}','\u{10df69}','\u{c5428}'];
_4.0 = [218_u8,199_u8];
_11 = -_1;
_1 = _11 - _11;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431768197837 => bb5,
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
_6 = [178_u8,115_u8];
_1 = _3 * _3;
_9 = -_7.fld2;
_4.2 = _7.fld0;
_7.fld1 = [(-1537930222_i32),1438489858_i32,(-32625557_i32),(-1349771619_i32)];
match _13 {
0 => bb1,
1 => bb3,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
330376176191971823869183382018758635822 => bb11,
_ => bb10
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
_4.3 = core::ptr::addr_of_mut!(_11);
_4.2 = _7.fld0;
_16 = [1379759936_i32,(-621587780_i32),(-1298351775_i32),(-895094512_i32),(-1367016961_i32),(-87302378_i32)];
_16 = [(-189979837_i32),(-248722194_i32),1502948793_i32,1685440841_i32,(-127393618_i32),571452877_i32];
_4.0 = [229_u8,24_u8];
_14 = 10581496258263479943_u64 as isize;
RET = ['\u{db5f2}','\u{36d3e}','\u{2ecee}','\u{6c92c}','\u{ab830}','\u{3eed6}'];
_7.fld2 = 76_u8 as i16;
_12 = [_14,_14,_14];
_15 = -_14;
_7.fld0 = _4.2 & _4.2;
_7.fld3 = [105_u8,96_u8];
_17 = 25484_u16 as f32;
_4.1 = [_14,_15];
_15 = _14;
_16 = [1457374176_i32,(-1960868357_i32),(-1764724463_i32),(-1350953534_i32),(-2039534814_i32),1758325_i32];
Call(_4.2 = fn11(RET, _16, RET, RET, RET, _7, _10, _16, _16, _7.fld1, _9, _2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_16 = [(-982006683_i32),(-1388472072_i32),(-2115784716_i32),(-1859538533_i32),179084910_i32,(-1006016600_i32)];
_4.1 = [_15,_15];
_2 = [198_u8,40_u8];
_5 = ['\u{4e998}','\u{ee8e5}','\u{dc9d2}','\u{f0e3a}'];
_2 = _4.0;
_3 = _1 - _1;
_1 = _3 * _3;
_11 = _3;
_11 = 66_u8 as f32;
_7 = Adt31 { fld0: _4.2,fld1: _10,fld2: _9,fld3: _2 };
RET = ['\u{104936}','\u{bcd0c}','\u{62110}','\u{b14e9}','\u{c8b65}','\u{10a4b3}'];
RET = ['\u{afdda}','\u{d3db9}','\u{23538}','\u{e9676}','\u{10d52a}','\u{81ae5}'];
_7.fld1 = _10;
_1 = _17;
match _13 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
330376176191971823869183382018758635822 => bb20,
_ => bb19
}
}
bb13 = {
_4.3 = core::ptr::addr_of_mut!(_11);
_4.2 = _7.fld0;
_16 = [1379759936_i32,(-621587780_i32),(-1298351775_i32),(-895094512_i32),(-1367016961_i32),(-87302378_i32)];
_16 = [(-189979837_i32),(-248722194_i32),1502948793_i32,1685440841_i32,(-127393618_i32),571452877_i32];
_4.0 = [229_u8,24_u8];
_14 = 10581496258263479943_u64 as isize;
RET = ['\u{db5f2}','\u{36d3e}','\u{2ecee}','\u{6c92c}','\u{ab830}','\u{3eed6}'];
_7.fld2 = 76_u8 as i16;
_12 = [_14,_14,_14];
_15 = -_14;
_7.fld0 = _4.2 & _4.2;
_7.fld3 = [105_u8,96_u8];
_17 = 25484_u16 as f32;
_4.1 = [_14,_15];
_15 = _14;
_16 = [1457374176_i32,(-1960868357_i32),(-1764724463_i32),(-1350953534_i32),(-2039534814_i32),1758325_i32];
Call(_4.2 = fn11(RET, _16, RET, RET, RET, _7, _10, _16, _16, _7.fld1, _9, _2), ReturnTo(bb12), UnwindUnreachable())
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
Return()
}
bb20 = {
_7.fld2 = _9 - _9;
_9 = 8188622766271798777_usize as i16;
_17 = _3;
_19 = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_16 = [(-1179728196_i32),(-1615659886_i32),1647954465_i32,(-488886919_i32),(-1613122780_i32),307612584_i32];
RET = ['\u{470c0}','\u{7ff1f}','\u{281eb}','\u{2a3c5}','\u{68e42}','\u{e8acc}'];
_15 = 492439583958216178_u64 as isize;
_2 = [106_u8,86_u8];
_18 = '\u{ec4cb}';
_7.fld0 = _4.2 * _4.2;
_19 = [_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_14 = _15;
_7 = Adt31 { fld0: _4.2,fld1: _10,fld2: _9,fld3: _2 };
RET = [_18,_18,_18,_18,_18,_18];
_12 = [_15,_15,_15];
_14 = _15 | _15;
_3 = 10375432945866866229_usize as f32;
_14 = _15;
_18 = '\u{1b871}';
RET = [_18,_18,_18,_18,_18,_18];
_12 = [_14,_15,_14];
_12 = [_15,_15,_15];
_3 = _17 + _17;
_20 = core::ptr::addr_of_mut!(_10);
_2 = _6;
RET = [_18,_18,_18,_18,_18,_18];
Goto(bb21)
}
bb21 = {
Call(_23 = dump_var(10_usize, 9_usize, Move(_9), 6_usize, Move(_6), 18_usize, Move(_18), 5_usize, Move(_5)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_23 = dump_var(10_usize, 16_usize, Move(_16), 12_usize, Move(_12), 24_usize, _24, 24_usize, _24), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [char; 6],mut _2: [i32; 6],mut _3: [char; 6],mut _4: [char; 6],mut _5: [char; 6],mut _6: Adt31,mut _7: [i32; 4],mut _8: [i32; 6],mut _9: [i32; 6],mut _10: [i32; 4],mut _11: i16,mut _12: [u8; 2]) -> u32 {
mir! {
type RET = u32;
let _13: bool;
let _14: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), u32, *const [u128; 8], &'static *const [u128; 8]);
let _15: [i8; 2];
let _16: isize;
let _17: (i32, (u8, i8), isize, u128);
let _18: bool;
let _19: i16;
let _20: isize;
let _21: i32;
let _22: &'static i32;
let _23: isize;
let _24: f64;
let _25: f32;
let _26: (i32, (u8, i8), isize, u128);
let _27: u128;
let _28: u32;
let _29: (&'static u16, Adt31);
let _30: &'static *mut *const (u8, i8);
let _31: f64;
let _32: [isize; 3];
let _33: &'static &'static i128;
let _34: &'static *const [u128; 8];
let _35: &'static [char; 6];
let _36: &'static &'static i32;
let _37: usize;
let _38: &'static f32;
let _39: Adt39;
let _40: [char; 6];
let _41: char;
let _42: isize;
let _43: isize;
let _44: f32;
let _45: u128;
let _46: i8;
let _47: f64;
let _48: i64;
let _49: &'static [char; 6];
let _50: [i32; 4];
let _51: [isize; 8];
let _52: [char; 6];
let _53: u16;
let _54: &'static [char; 6];
let _55: *mut [i8; 2];
let _56: ();
let _57: ();
{
_6.fld2 = _11;
_4 = _3;
RET = 5_usize as u32;
_3 = ['\u{4b687}','\u{f10a3}','\u{59621}','\u{a80d6}','\u{a9f23}','\u{b5b0c}'];
_8 = [80456917_i32,(-463838888_i32),(-1158288172_i32),(-1130189017_i32),153244635_i32,(-1975234779_i32)];
_3 = ['\u{8101}','\u{77027}','\u{34dfa}','\u{bdc93}','\u{f91de}','\u{52ae7}'];
_6.fld1 = [2113273869_i32,(-2054716867_i32),2112130847_i32,1828481662_i32];
_6.fld3 = _12;
_7 = _10;
_9 = [1093213135_i32,(-2044672826_i32),(-1409295724_i32),1637814702_i32,(-1275087064_i32),(-1962887514_i32)];
_6.fld0 = RET;
_1 = ['\u{ae84f}','\u{89748}','\u{cb1e9}','\u{a55f0}','\u{86b4f}','\u{40910}'];
RET = _6.fld0;
_14.2 = core::ptr::addr_of!(_14.0.0);
Call(_14.1 = fn12(_6.fld2, _9, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _1;
_13 = !false;
_14.1 = 254051547136784803018615873737997577120_u128 as u32;
_12 = [89_u8,154_u8];
_8 = [(-938541595_i32),200241975_i32,1878812459_i32,(-367652942_i32),445112481_i32,1465482274_i32];
RET = !_14.1;
_2 = _8;
_14.0.3 = &_15;
RET = !_6.fld0;
_6.fld0 = !RET;
_17.2 = 9223372036854775807_isize ^ 91_isize;
_7 = [907660601_i32,(-485580162_i32),(-1559384777_i32),1456444522_i32];
_14.0.1 = core::ptr::addr_of!(_17.1);
_14.3 = &_14.2;
_14.0.0 = [330458102321861355951357016078907117275_u128,274402210180561301705001101205365652017_u128,289770993590166323678530517876383479142_u128,193334598588585373823034540945529942336_u128,248878387844603887719179840248229956576_u128,252638589257447418443296623503153529848_u128,276508999540812897638164942774431025397_u128,15373099252875301139774371181096218074_u128];
Goto(bb2)
}
bb2 = {
_11 = _6.fld2 | _6.fld2;
_6.fld0 = !_14.1;
_14.3 = &_14.2;
_4 = ['\u{9f325}','\u{3e9d0}','\u{1e96f}','\u{14e08}','\u{97e37}','\u{18105}'];
_3 = ['\u{16bcd}','\u{4e2ba}','\u{d755f}','\u{6beee}','\u{da44c}','\u{c09e0}'];
_14.1 = RET + RET;
_14.0.2 = &_22;
_6.fld2 = !_11;
RET = !_14.1;
_20 = -_17.2;
_24 = _6.fld2 as f64;
_26.3 = !36150327000672546521410023014162589937_u128;
_17.1.1 = _20 as i8;
_22 = &_21;
_17.3 = !_26.3;
_14.1 = 98_u8 as u32;
_26.3 = '\u{5836d}' as u128;
_11 = _6.fld2 ^ _6.fld2;
Call(_26.2 = core::intrinsics::transmute(_17.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = !_11;
_17.1 = (176_u8, 96_i8);
_11 = _19 << _19;
_26.1 = _17.1;
_17.2 = _20;
_5 = _1;
_14.2 = core::ptr::addr_of!(_14.0.0);
_17.2 = -_20;
_14.0.2 = &_22;
_29.1.fld2 = _11;
_26.0 = (-17067991_i32);
_26.2 = -_20;
_2 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_14.0.0 = [_26.3,_26.3,_26.3,_17.3,_17.3,_26.3,_26.3,_26.3];
_26.0 = 404204700_i32;
_29.1.fld3 = [_26.1.0,_26.1.0];
_17.1 = _26.1;
RET = _14.1 - _6.fld0;
_15 = [_26.1.1,_17.1.1];
_29.1.fld0 = !RET;
_6.fld2 = _19 + _11;
_2 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_8 = _2;
match _17.1.1 {
0 => bb4,
96 => bb6,
_ => bb5
}
}
bb4 = {
_11 = _6.fld2 | _6.fld2;
_6.fld0 = !_14.1;
_14.3 = &_14.2;
_4 = ['\u{9f325}','\u{3e9d0}','\u{1e96f}','\u{14e08}','\u{97e37}','\u{18105}'];
_3 = ['\u{16bcd}','\u{4e2ba}','\u{d755f}','\u{6beee}','\u{da44c}','\u{c09e0}'];
_14.1 = RET + RET;
_14.0.2 = &_22;
_6.fld2 = !_11;
RET = !_14.1;
_20 = -_17.2;
_24 = _6.fld2 as f64;
_26.3 = !36150327000672546521410023014162589937_u128;
_17.1.1 = _20 as i8;
_22 = &_21;
_17.3 = !_26.3;
_14.1 = 98_u8 as u32;
_26.3 = '\u{5836d}' as u128;
_11 = _6.fld2 ^ _6.fld2;
Call(_26.2 = core::intrinsics::transmute(_17.2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = _1;
_13 = !false;
_14.1 = 254051547136784803018615873737997577120_u128 as u32;
_12 = [89_u8,154_u8];
_8 = [(-938541595_i32),200241975_i32,1878812459_i32,(-367652942_i32),445112481_i32,1465482274_i32];
RET = !_14.1;
_2 = _8;
_14.0.3 = &_15;
RET = !_6.fld0;
_6.fld0 = !RET;
_17.2 = 9223372036854775807_isize ^ 91_isize;
_7 = [907660601_i32,(-485580162_i32),(-1559384777_i32),1456444522_i32];
_14.0.1 = core::ptr::addr_of!(_17.1);
_14.3 = &_14.2;
_14.0.0 = [330458102321861355951357016078907117275_u128,274402210180561301705001101205365652017_u128,289770993590166323678530517876383479142_u128,193334598588585373823034540945529942336_u128,248878387844603887719179840248229956576_u128,252638589257447418443296623503153529848_u128,276508999540812897638164942774431025397_u128,15373099252875301139774371181096218074_u128];
Goto(bb2)
}
bb6 = {
_2 = [_26.0,_26.0,_26.0,_26.0,_26.0,_26.0];
_29.1.fld2 = _11 + _11;
_26.3 = _17.3;
_20 = _17.2 - _17.2;
_17.1 = (_26.1.0, _26.1.1);
_26.1.1 = _17.2 as i8;
_14.0.3 = &_15;
_17.1.1 = _26.1.1;
_24 = 11025069840009053856_u64 as f64;
_19 = _11;
_17.0 = _26.0 + _26.0;
_25 = _29.1.fld2 as f32;
_17.1 = (_26.1.0, _26.1.1);
_11 = 0_usize as i16;
_21 = 131809980537268537287988761347829423254_i128 as i32;
_17.3 = _19 as u128;
_29.1 = _6;
_14.0.2 = &_22;
_28 = RET;
_18 = !_13;
_8 = _9;
_8 = [_17.0,_17.0,_21,_17.0,_21,_21];
_14.0.3 = &_15;
_20 = -_17.2;
_12 = [_26.1.0,_26.1.0];
_9 = [_26.0,_17.0,_17.0,_17.0,_17.0,_17.0];
_18 = _13;
_14.0.2 = &_22;
match _26.0 {
404204700 => bb7,
_ => bb1
}
}
bb7 = {
_9 = [_17.0,_26.0,_26.0,_17.0,_17.0,_17.0];
_8 = [_17.0,_26.0,_21,_21,_26.0,_17.0];
_36 = &_22;
_25 = _6.fld2 as f32;
_14.0.3 = &_15;
RET = _26.1.0 as u32;
_6 = Adt31 { fld0: _29.1.fld0,fld1: _10,fld2: _19,fld3: _29.1.fld3 };
Call(_12 = fn14(_6, _20, _6.fld2, _6.fld2, _25, _6, _29.1, _25), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_29.1.fld0 = _6.fld0;
_8 = [_26.0,_17.0,_26.0,_26.0,_26.0,_17.0];
_35 = &_3;
_17 = (_21, _26.1, _20, _26.3);
_38 = &_25;
_13 = _18;
_15 = [_26.1.1,_17.1.1];
_14.0.3 = &_15;
_26 = (_17.0, _17.1, _20, _17.3);
_17.1 = (_26.1.0, _26.1.1);
_14.0.0 = [_17.3,_26.3,_17.3,_17.3,_17.3,_26.3,_17.3,_17.3];
_14.2 = core::ptr::addr_of!(_14.0.0);
_14.0.0 = [_17.3,_26.3,_26.3,_26.3,_17.3,_26.3,_26.3,_17.3];
_17.1.1 = _26.1.1;
match _26.1.0 {
0 => bb9,
176 => bb11,
_ => bb10
}
}
bb9 = {
_9 = [_17.0,_26.0,_26.0,_17.0,_17.0,_17.0];
_8 = [_17.0,_26.0,_21,_21,_26.0,_17.0];
_36 = &_22;
_25 = _6.fld2 as f32;
_14.0.3 = &_15;
RET = _26.1.0 as u32;
_6 = Adt31 { fld0: _29.1.fld0,fld1: _10,fld2: _19,fld3: _29.1.fld3 };
Call(_12 = fn14(_6, _20, _6.fld2, _6.fld2, _25, _6, _29.1, _25), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_11 = _6.fld2 | _6.fld2;
_6.fld0 = !_14.1;
_14.3 = &_14.2;
_4 = ['\u{9f325}','\u{3e9d0}','\u{1e96f}','\u{14e08}','\u{97e37}','\u{18105}'];
_3 = ['\u{16bcd}','\u{4e2ba}','\u{d755f}','\u{6beee}','\u{da44c}','\u{c09e0}'];
_14.1 = RET + RET;
_14.0.2 = &_22;
_6.fld2 = !_11;
RET = !_14.1;
_20 = -_17.2;
_24 = _6.fld2 as f64;
_26.3 = !36150327000672546521410023014162589937_u128;
_17.1.1 = _20 as i8;
_22 = &_21;
_17.3 = !_26.3;
_14.1 = 98_u8 as u32;
_26.3 = '\u{5836d}' as u128;
_11 = _6.fld2 ^ _6.fld2;
Call(_26.2 = core::intrinsics::transmute(_17.2), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_18 = !_13;
_36 = &(*_36);
_15 = [_26.1.1,_17.1.1];
_23 = !_26.2;
RET = !_29.1.fld0;
_6.fld2 = _29.1.fld2 | _29.1.fld2;
_29.1.fld2 = _6.fld2 - _6.fld2;
_18 = _13;
_31 = _24 * _24;
_10 = [_21,_17.0,_17.0,_17.0];
_12 = [_26.1.0,_17.1.0];
Goto(bb12)
}
bb12 = {
_22 = &_21;
_6.fld2 = (-139245631095851228514387742257431402310_i128) as i16;
_37 = !681045359854863599_usize;
_15 = [_17.1.1,_17.1.1];
_43 = '\u{40434}' as isize;
_29.1 = _6;
_6.fld3 = [_26.1.0,_17.1.0];
_40 = ['\u{bde86}','\u{e85c6}','\u{b688c}','\u{12cab}','\u{54cbf}','\u{2c8bf}'];
_14.0.1 = core::ptr::addr_of!(_26.1);
_26.1 = (_17.1.0, _17.1.1);
_29.1.fld0 = _6.fld0;
_29.1 = Adt31 { fld0: _14.1,fld1: _7,fld2: _19,fld3: _6.fld3 };
_6.fld0 = _29.1.fld0 * _28;
_6.fld0 = _29.1.fld0 * RET;
_42 = _26.2 ^ _17.2;
Goto(bb13)
}
bb13 = {
_36 = &_22;
_37 = !18022902770952652073_usize;
_20 = (-7081000496089851290_i64) as isize;
_29.1.fld3 = [_17.1.0,_26.1.0];
_26.1 = (_17.1.0, _17.1.1);
_17.1.1 = !_26.1.1;
_6 = _29.1;
_34 = &_14.2;
_18 = _13;
_17.3 = '\u{83211}' as u128;
_21 = _31 as i32;
_50 = _10;
_32 = [_42,_26.2,_17.2];
_34 = &(*_34);
Goto(bb14)
}
bb14 = {
_41 = '\u{f7939}';
_51 = [_17.2,_42,_42,_23,_26.2,_42,_43,_17.2];
_27 = !_17.3;
_41 = '\u{bfee3}';
_17 = _26;
_31 = -_24;
_26.2 = _42;
_26.2 = _42 >> _19;
_24 = -_31;
_26.3 = !_27;
_17.1.0 = _26.1.0;
_17.2 = _27 as isize;
_34 = &_14.2;
_40 = [_41,_41,_41,_41,_41,_41];
_17.2 = !_26.2;
_14.3 = &(*_34);
Goto(bb15)
}
bb15 = {
Call(_56 = dump_var(11_usize, 26_usize, Move(_26), 13_usize, Move(_13), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_56 = dump_var(11_usize, 15_usize, Move(_15), 17_usize, Move(_17), 7_usize, Move(_7), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_56 = dump_var(11_usize, 10_usize, Move(_10), 8_usize, Move(_8), 23_usize, Move(_23), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(11_usize, 51_usize, Move(_51), 4_usize, Move(_4), 50_usize, Move(_50), 57_usize, _57), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i16,mut _2: [i32; 6],mut _3: [i32; 6]) -> u32 {
mir! {
type RET = u32;
let _4: isize;
let _5: isize;
let _6: usize;
let _7: *mut [i8; 2];
let _8: i128;
let _9: usize;
let _10: u64;
let _11: ([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]);
let _12: (&'static u16, Adt31);
let _13: ();
let _14: ();
{
_3 = _2;
_1 = (-14473_i16);
_1 = 12198_i16;
RET = (-126_i8) as u32;
Goto(bb1)
}
bb1 = {
RET = 298459371_u32;
RET = !327816113_u32;
_1 = (-29203_i16) + 9444_i16;
_3 = [1753528075_i32,(-1765336155_i32),1964171116_i32,1434368580_i32,1243027175_i32,(-48486718_i32)];
_4 = !(-9223372036854775808_isize);
_3 = [(-1472722080_i32),(-531841248_i32),2097742217_i32,1592975269_i32,512178905_i32,(-1892719228_i32)];
RET = 4150696756_u32;
_3 = [(-1246294218_i32),(-1080510446_i32),1884924755_i32,261623649_i32,552148507_i32,(-1421789898_i32)];
RET = 340992981_u32;
_3 = [2073002784_i32,2088834317_i32,(-176345418_i32),1874278338_i32,(-223801385_i32),1272196745_i32];
_3 = [(-1214215531_i32),(-87440831_i32),616071746_i32,(-1729335578_i32),(-186083717_i32),(-1533709741_i32)];
Goto(bb2)
}
bb2 = {
RET = (-1727233949813389152_i64) as u32;
_1 = 20511_i16 >> _4;
RET = !1793308455_u32;
RET = '\u{196c4}' as u32;
RET = !2509630097_u32;
_3 = [(-2047530995_i32),1569151509_i32,351744452_i32,(-1451408412_i32),(-995338472_i32),461853555_i32];
_3 = [258228082_i32,733158601_i32,2024178480_i32,1013757132_i32,840219643_i32,334796420_i32];
_4 = -9223372036854775807_isize;
_4 = _1 as isize;
RET = 3166237309_u32;
Goto(bb3)
}
bb3 = {
RET = _1 as u32;
RET = 303466905913702012935059073186031440011_u128 as u32;
_6 = false as usize;
_2 = _3;
RET = 1667639698_u32;
_2 = _3;
_5 = (-75753167464058125042078269318859667963_i128) as isize;
Goto(bb4)
}
bb4 = {
_2 = [(-917788251_i32),(-142647972_i32),664326199_i32,(-177997856_i32),206892154_i32,(-2089808591_i32)];
_4 = '\u{85cad}' as isize;
_6 = 11408653126049721047_usize & 220728471836285134_usize;
_3 = [(-1560264438_i32),1092417171_i32,(-1585079591_i32),1514749045_i32,(-1268118693_i32),(-1260021720_i32)];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
1667639698 => bb9,
_ => bb8
}
}
bb5 = {
RET = _1 as u32;
RET = 303466905913702012935059073186031440011_u128 as u32;
_6 = false as usize;
_2 = _3;
RET = 1667639698_u32;
_2 = _3;
_5 = (-75753167464058125042078269318859667963_i128) as isize;
Goto(bb4)
}
bb6 = {
RET = (-1727233949813389152_i64) as u32;
_1 = 20511_i16 >> _4;
RET = !1793308455_u32;
RET = '\u{196c4}' as u32;
RET = !2509630097_u32;
_3 = [(-2047530995_i32),1569151509_i32,351744452_i32,(-1451408412_i32),(-995338472_i32),461853555_i32];
_3 = [258228082_i32,733158601_i32,2024178480_i32,1013757132_i32,840219643_i32,334796420_i32];
_4 = -9223372036854775807_isize;
_4 = _1 as isize;
RET = 3166237309_u32;
Goto(bb3)
}
bb7 = {
RET = 298459371_u32;
RET = !327816113_u32;
_1 = (-29203_i16) + 9444_i16;
_3 = [1753528075_i32,(-1765336155_i32),1964171116_i32,1434368580_i32,1243027175_i32,(-48486718_i32)];
_4 = !(-9223372036854775808_isize);
_3 = [(-1472722080_i32),(-531841248_i32),2097742217_i32,1592975269_i32,512178905_i32,(-1892719228_i32)];
RET = 4150696756_u32;
_3 = [(-1246294218_i32),(-1080510446_i32),1884924755_i32,261623649_i32,552148507_i32,(-1421789898_i32)];
RET = 340992981_u32;
_3 = [2073002784_i32,2088834317_i32,(-176345418_i32),1874278338_i32,(-223801385_i32),1272196745_i32];
_3 = [(-1214215531_i32),(-87440831_i32),616071746_i32,(-1729335578_i32),(-186083717_i32),(-1533709741_i32)];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_6 = 4_usize * 7_usize;
Goto(bb10)
}
bb10 = {
RET = 2895808754_u32;
_2 = _3;
RET = !3446371055_u32;
RET = 4280303601_u32;
_2 = [807495437_i32,250359698_i32,697777093_i32,(-667890735_i32),(-2007536004_i32),387597835_i32];
_3 = [1085748601_i32,1592467331_i32,(-1780896296_i32),(-116221682_i32),585569709_i32,(-1929164933_i32)];
_1 = (-17700_i16);
_1 = !11402_i16;
_3 = [943863269_i32,901327931_i32,(-301968828_i32),770247978_i32,(-1469922654_i32),186159993_i32];
_6 = _1 as usize;
_2 = _3;
_2 = _3;
_1 = 21540_i16;
_3 = _2;
_1 = (-27621_i16);
_5 = -_4;
RET = 2301082391_u32;
RET = (-45358211976542820496063664287835681433_i128) as u32;
Goto(bb11)
}
bb11 = {
_4 = _5 - _5;
RET = 167172386_u32;
_6 = 2025360532821598751_usize | 5_usize;
_5 = _4 + _4;
_5 = _4;
RET = 400481845_u32 * 4207117455_u32;
RET = !2048018964_u32;
RET = !2339141474_u32;
_4 = -_5;
_4 = _5 >> _5;
_9 = 43189_u16 as usize;
_8 = 103283142611612908562543885609952511991_i128;
RET = 3268210883_u32 ^ 277879485_u32;
Call(_8 = core::intrinsics::bswap(155303924508779793577092515512354570079_i128), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = _4 as usize;
_4 = -_5;
RET = 402466326_u32;
_1 = 16481_i16;
_10 = 13662548966419458432_u64;
_2 = [(-1664602453_i32),1459152403_i32,(-1806664343_i32),(-159580555_i32),1962618825_i32,(-1718964106_i32)];
_4 = !_5;
_6 = true as usize;
_2 = [2064976072_i32,1494831095_i32,312385266_i32,1040023858_i32,(-1661222429_i32),1305455948_i32];
_4 = _5 ^ _5;
_2 = [1497363167_i32,(-1815518014_i32),1342338103_i32,1702956593_i32,(-780294246_i32),(-1004867683_i32)];
_5 = _4 + _4;
Call(_4 = fn13(_5, _5, _2, _5, _9, _3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = 1361549426_u32;
RET = !1810007692_u32;
RET = (-1964718994_i32) as u32;
_2 = [(-163193022_i32),(-1606610302_i32),1730259166_i32,(-1844543633_i32),1045222067_i32,538547587_i32];
_5 = _4;
_3 = _2;
_10 = 10939657176892008357_u64 >> _8;
_10 = 16606833159854793420_u64 ^ 10806884069141357599_u64;
_4 = _5 >> _5;
_2 = _3;
_5 = _8 as isize;
_2 = [(-1489516315_i32),(-107647729_i32),(-661499788_i32),(-1495119529_i32),1797279933_i32,(-524545778_i32)];
RET = 4023126246_u32 & 2775909573_u32;
_6 = '\u{994ef}' as usize;
RET = 1124353627_u32;
_1 = 134_u8 as i16;
_12.1.fld3 = [239_u8,198_u8];
_11.0 = [22654440366515748444903482281485560886_u128,297089896047816591959494756193764668684_u128,191634383147819830254965082392534839284_u128,51954293355846574967413601041359810080_u128,114018272164195512377831448934512802133_u128,265494768290700650919275014091171295248_u128,27820168369049914285294714766551533783_u128,156995031201363343668440463513530890105_u128];
_6 = (-6082379232426550931_i64) as usize;
_12.1.fld1 = [1741564559_i32,1409536418_i32,(-70954977_i32),(-2110907724_i32)];
RET = _4 as u32;
Goto(bb14)
}
bb14 = {
Call(_13 = dump_var(12_usize, 4_usize, Move(_4), 9_usize, Move(_9), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: [i32; 6],mut _4: isize,mut _5: usize,mut _6: [i32; 6]) -> isize {
mir! {
type RET = isize;
let _7: ();
let _8: ();
{
RET = _1 | _1;
_6 = [(-1246056494_i32),1734728576_i32,(-749799380_i32),(-1761113708_i32),(-1487423927_i32),(-1534411396_i32)];
_4 = (-1844156153_i32) as isize;
_4 = !RET;
_5 = 5_usize << RET;
_1 = !_4;
RET = 66708119088190344217265151231633710569_u128 as isize;
RET = !_4;
_4 = 1367692651347200585_i64 as isize;
_2 = RET;
RET = 26439_u16 as isize;
_2 = _1;
RET = _1;
RET = _2 | _2;
_6 = [(-1417385433_i32),(-1268300616_i32),(-1833797800_i32),(-162272396_i32),189340465_i32,1513196545_i32];
_3 = _6;
RET = _1 * _1;
_3 = [(-1039402901_i32),(-217007153_i32),(-528197628_i32),(-24355892_i32),(-1085952225_i32),(-1618625972_i32)];
_3 = [(-776756094_i32),(-560940692_i32),2101572791_i32,(-897213170_i32),626139128_i32,(-754257414_i32)];
_1 = !RET;
_4 = -RET;
RET = _5 as isize;
RET = -_2;
_1 = 39_u8 as isize;
_4 = RET << _2;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(13_usize, 1_usize, Move(_1), 5_usize, Move(_5), 3_usize, Move(_3), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: Adt31,mut _2: isize,mut _3: i16,mut _4: i16,mut _5: f32,mut _6: Adt31,mut _7: Adt31,mut _8: f32) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _9: i16;
let _10: f64;
let _11: (char, [char; 6]);
let _12: [i8; 2];
let _13: f32;
let _14: [i16; 4];
let _15: i16;
let _16: char;
let _17: bool;
let _18: *mut *const (u8, i8);
let _19: &'static i64;
let _20: u16;
let _21: i64;
let _22: ((i32, (u8, i8), isize, u128),);
let _23: Adt41;
let _24: *const [i32; 6];
let _25: [i8; 2];
let _26: [u64; 6];
let _27: (u32, &'static &'static bool, u16);
let _28: u8;
let _29: f64;
let _30: ([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]);
let _31: u8;
let _32: Adt31;
let _33: &'static &'static &'static &'static bool;
let _34: f32;
let _35: *const *const isize;
let _36: &'static [i32; 5];
let _37: f32;
let _38: u128;
let _39: &'static [char; 6];
let _40: isize;
let _41: bool;
let _42: u32;
let _43: ();
let _44: ();
{
_6.fld0 = _1.fld0 ^ _1.fld0;
_8 = -_5;
RET = _7.fld3;
_6.fld1 = [951470506_i32,(-1041132763_i32),1474722878_i32,(-1604397430_i32)];
_6.fld2 = !_4;
_6.fld3 = [88_u8,177_u8];
_1.fld1 = _6.fld1;
_7.fld0 = !_1.fld0;
_7.fld2 = -_1.fld2;
Goto(bb1)
}
bb1 = {
_7.fld0 = !_1.fld0;
_1 = Adt31 { fld0: _7.fld0,fld1: _6.fld1,fld2: _7.fld2,fld3: _7.fld3 };
_11.1 = ['\u{d1383}','\u{249b}','\u{81a4a}','\u{bf5f2}','\u{10c039}','\u{30c8c}'];
_7.fld3 = [224_u8,53_u8];
_6.fld3 = [215_u8,124_u8];
_6.fld3 = RET;
_6.fld2 = -_1.fld2;
_9 = _6.fld2;
_8 = _5 * _5;
_6.fld1 = [(-1144232510_i32),1117019507_i32,(-1970644846_i32),2082122796_i32];
_1.fld0 = _6.fld0 | _6.fld0;
RET = [183_u8,168_u8];
_7.fld3 = [150_u8,67_u8];
_6.fld1 = [(-1439000125_i32),(-407853109_i32),(-1798535586_i32),(-6279778_i32)];
_6 = Adt31 { fld0: _1.fld0,fld1: _7.fld1,fld2: _9,fld3: _7.fld3 };
_1.fld3 = [255_u8,81_u8];
_16 = '\u{f2050}';
_1.fld2 = -_3;
_4 = _5 as i16;
_6.fld0 = 17566_u16 as u32;
Goto(bb2)
}
bb2 = {
_16 = '\u{f642c}';
_5 = _8 - _8;
_7.fld1 = _1.fld1;
_10 = _1.fld0 as f64;
_1 = Adt31 { fld0: _6.fld0,fld1: _7.fld1,fld2: _6.fld2,fld3: _6.fld3 };
_3 = -_7.fld2;
_14 = [_7.fld2,_9,_3,_9];
_17 = !true;
_9 = _7.fld2 + _6.fld2;
_7.fld1 = [704242455_i32,837209417_i32,(-1124263231_i32),1951955893_i32];
_4 = _9;
_13 = _8;
Call(_8 = fn15(_6, _2, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11.0 = _16;
_5 = _8 * _8;
RET = [15_u8,166_u8];
_11.0 = _16;
_1 = Adt31 { fld0: _7.fld0,fld1: _6.fld1,fld2: _6.fld2,fld3: _6.fld3 };
_7.fld1 = [350116197_i32,(-827871465_i32),(-1392329640_i32),105040599_i32];
_6 = _7;
_1.fld3 = [132_u8,189_u8];
_5 = _8;
_6.fld0 = _7.fld0;
_8 = _5 + _13;
_2 = !9223372036854775807_isize;
_3 = (-8959692495363106632_i64) as i16;
_7.fld3 = [205_u8,116_u8];
_1.fld2 = -_9;
_15 = !_4;
_22.0.2 = _2 >> _4;
_9 = 884954423370566380_i64 as i16;
_19 = &_21;
_11.1 = [_16,_16,_16,_16,_11.0,_11.0];
Call(_1.fld0 = fn16(_8, _14, _13, _1.fld2, _6, _6.fld2, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_22.0.1.1 = (-103_i8) + 90_i8;
_20 = 44829_u16;
_12 = [_22.0.1.1,_22.0.1.1];
_20 = 19411_u16 | 28722_u16;
_7.fld1 = [(-94145379_i32),1329323879_i32,2097438494_i32,(-1682502731_i32)];
_6.fld0 = _1.fld0 | _1.fld0;
_6 = Adt31 { fld0: _1.fld0,fld1: _7.fld1,fld2: _1.fld2,fld3: _1.fld3 };
_2 = _22.0.2;
_22.0.1.0 = _22.0.1.1 as u8;
_22.0.1 = (210_u8, (-128_i8));
_26 = [2451588108238749500_u64,15701916240953970384_u64,15492039185980413266_u64,18140079829677858075_u64,3889319257116436542_u64,10243051411368859604_u64];
_7.fld0 = _6.fld0 * _1.fld0;
_23 = Adt41::Variant0 { fld0: 1388943523057188087_i64 };
_6.fld3 = [_22.0.1.0,_22.0.1.0];
_25 = _12;
_26 = [6164572102175244622_u64,382853762302701174_u64,1895133013312767459_u64,1952759998245183492_u64,14636367982244396236_u64,5680061600523296295_u64];
_27.0 = !_1.fld0;
_2 = _22.0.2 & _22.0.2;
_1.fld0 = 1157461436_i32 as u32;
_6.fld1 = [(-2030641955_i32),1076757314_i32,(-1555331136_i32),210052584_i32];
_6.fld2 = -_1.fld2;
_20 = !7333_u16;
_7.fld3 = [_22.0.1.0,_22.0.1.0];
Call(_7 = fn17(_15, _6.fld0, _2, _1, _27.0, _22.0.2, _15, _15, _22.0.2, _1.fld2, _6.fld0, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6.fld2 = _7.fld2;
_9 = -_1.fld2;
_6.fld0 = (-80150116662180984625813428657887979508_i128) as u32;
_7.fld2 = _6.fld2;
_22.0.1.1 = -17_i8;
_30.3 = &_12;
_27.2 = 4401625845929721064_i64 as u16;
_31 = _22.0.1.0 + _22.0.1.0;
_6.fld0 = _27.0 & _27.0;
_31 = _22.0.1.0;
_21 = _22.0.1.1 as i64;
_22.0.1.1 = (-934477234_i32) as i8;
_19 = &_21;
_28 = 142835281625317455729758142133155375162_i128 as u8;
_22.0.0 = _7.fld2 as i32;
RET = [_22.0.1.0,_22.0.1.0];
Goto(bb6)
}
bb6 = {
_4 = _22.0.0 as i16;
_16 = _11.0;
_13 = _5 + _5;
Call(_14 = fn18(Move(_30.3), Move(_19), _6.fld0, _6.fld2, _27.0, _6.fld2, _4, _7.fld0, _27.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6.fld3 = [_22.0.1.0,_31];
_22.0.1 = (_28, 4_i8);
_32.fld1 = [_22.0.0,_22.0.0,_22.0.0,_22.0.0];
_21 = 7365293777288586625_i64;
_32.fld2 = _1.fld2 & _7.fld2;
_30.0 = [34174763912485732086553813936258805924_u128,59105235919068442346727391318969197164_u128,183892813954216680847372520269587948411_u128,219788204599404611230189376550764295119_u128,101601210610769443198855371031265954045_u128,124196559447570450510381050442326886504_u128,223090626693928732338502549987129378376_u128,329481474626216664537019051396315379013_u128];
_32.fld0 = !_7.fld0;
Goto(bb8)
}
bb8 = {
_34 = _13 - _13;
_7.fld3 = RET;
_18 = core::ptr::addr_of_mut!(_30.1);
_30.1 = core::ptr::addr_of!(_22.0.1);
place!(Field::<i64>(Variant(_23, 0), 0)) = _27.0 as i64;
_12 = [_22.0.1.1,_22.0.1.1];
_4 = _1.fld2;
_13 = -_8;
_19 = &_21;
_10 = _32.fld0 as f64;
_8 = _13 + _13;
_22.0.1 = (_31, 0_i8);
_1.fld1 = [_22.0.0,_22.0.0,_22.0.0,_22.0.0];
_32.fld1 = [_22.0.0,_22.0.0,_22.0.0,_22.0.0];
_19 = &place!(Field::<i64>(Variant(_23, 0), 0));
RET = [_22.0.1.0,_31];
match _22.0.1.1 {
1 => bb9,
2 => bb10,
3 => bb11,
0 => bb13,
_ => bb12
}
}
bb9 = {
_6.fld3 = [_22.0.1.0,_31];
_22.0.1 = (_28, 4_i8);
_32.fld1 = [_22.0.0,_22.0.0,_22.0.0,_22.0.0];
_21 = 7365293777288586625_i64;
_32.fld2 = _1.fld2 & _7.fld2;
_30.0 = [34174763912485732086553813936258805924_u128,59105235919068442346727391318969197164_u128,183892813954216680847372520269587948411_u128,219788204599404611230189376550764295119_u128,101601210610769443198855371031265954045_u128,124196559447570450510381050442326886504_u128,223090626693928732338502549987129378376_u128,329481474626216664537019051396315379013_u128];
_32.fld0 = !_7.fld0;
Goto(bb8)
}
bb10 = {
_4 = _22.0.0 as i16;
_16 = _11.0;
_13 = _5 + _5;
Call(_14 = fn18(Move(_30.3), Move(_19), _6.fld0, _6.fld2, _27.0, _6.fld2, _4, _7.fld0, _27.0), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_6.fld2 = _7.fld2;
_9 = -_1.fld2;
_6.fld0 = (-80150116662180984625813428657887979508_i128) as u32;
_7.fld2 = _6.fld2;
_22.0.1.1 = -17_i8;
_30.3 = &_12;
_27.2 = 4401625845929721064_i64 as u16;
_31 = _22.0.1.0 + _22.0.1.0;
_6.fld0 = _27.0 & _27.0;
_31 = _22.0.1.0;
_21 = _22.0.1.1 as i64;
_22.0.1.1 = (-934477234_i32) as i8;
_19 = &_21;
_28 = 142835281625317455729758142133155375162_i128 as u8;
_22.0.0 = _7.fld2 as i32;
RET = [_22.0.1.0,_22.0.1.0];
Goto(bb6)
}
bb12 = {
_16 = '\u{f642c}';
_5 = _8 - _8;
_7.fld1 = _1.fld1;
_10 = _1.fld0 as f64;
_1 = Adt31 { fld0: _6.fld0,fld1: _7.fld1,fld2: _6.fld2,fld3: _6.fld3 };
_3 = -_7.fld2;
_14 = [_7.fld2,_9,_3,_9];
_17 = !true;
_9 = _7.fld2 + _6.fld2;
_7.fld1 = [704242455_i32,837209417_i32,(-1124263231_i32),1951955893_i32];
_4 = _9;
_13 = _8;
Call(_8 = fn15(_6, _2, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_22.0.2 = _2 >> _32.fld2;
_1.fld2 = _22.0.0 as i16;
_1.fld1 = _32.fld1;
_21 = _7.fld2 as i64;
_21 = Field::<i64>(Variant(_23, 0), 0);
_38 = 253592173808527879033684050717928060456_u128 - 286819794328867960896100856847350474584_u128;
_6.fld3 = _7.fld3;
_22.0.2 = _2;
_5 = _13 - _34;
_12 = [_22.0.1.1,_22.0.1.1];
_6.fld2 = _7.fld2 + _4;
RET = [_31,_22.0.1.0];
_12 = [_22.0.1.1,_22.0.1.1];
SetDiscriminant(_23, 1);
_37 = _34 * _5;
_31 = _2 as u8;
_7.fld1 = [_22.0.0,_22.0.0,_22.0.0,_22.0.0];
_30.3 = &_25;
_5 = _13;
_32.fld3 = _6.fld3;
Goto(bb14)
}
bb14 = {
_22.0.1 = (_31, (-84_i8));
_6.fld1 = _1.fld1;
_41 = !_17;
_1.fld2 = !_32.fld2;
_21 = -8281260805122556330_i64;
RET = [_31,_22.0.1.0];
RET = [_31,_31];
place!(Field::<bool>(Variant(_23, 1), 0)) = _41 & _41;
_22.0.2 = _32.fld0 as isize;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(14_usize, 21_usize, Move(_21), 28_usize, Move(_28), 2_usize, Move(_2), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(14_usize, 9_usize, Move(_9), 16_usize, Move(_16), 20_usize, Move(_20), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(14_usize, 26_usize, Move(_26), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: Adt31,mut _2: isize,mut _3: f32) -> f32 {
mir! {
type RET = f32;
let _4: i32;
let _5: *mut f32;
let _6: ();
let _7: ();
{
RET = -_3;
_1.fld1 = [(-345822804_i32),1455170820_i32,243470789_i32,(-351610808_i32)];
_1.fld1 = [794289861_i32,(-558812056_i32),1536696690_i32,62388120_i32];
_1.fld1 = [(-890013148_i32),1567828825_i32,1290297160_i32,633089005_i32];
_1.fld2 = (-85_i8) as i16;
_1.fld3 = [218_u8,247_u8];
_4 = -1930960810_i32;
RET = 105815012369906112984147147942353448179_u128 as f32;
RET = 6268411307360449627_u64 as f32;
RET = _3;
_1.fld0 = 2123701061_u32 << _4;
RET = _3 - _3;
_5 = core::ptr::addr_of_mut!(RET);
_3 = _1.fld2 as f32;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(15_usize, 2_usize, Move(_2), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: f32,mut _2: [i16; 4],mut _3: f32,mut _4: i16,mut _5: Adt31,mut _6: i16,mut _7: [i16; 4]) -> u32 {
mir! {
type RET = u32;
let _8: ([char; 6], f32, i64);
let _9: i128;
let _10: ([char; 6], f32, i64);
let _11: &'static Adt55;
let _12: i16;
let _13: &'static f32;
let _14: isize;
let _15: bool;
let _16: i8;
let _17: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), *mut f32, [i8; 2]);
let _18: bool;
let _19: i128;
let _20: &'static Adt55;
let _21: *mut f64;
let _22: isize;
let _23: (([u128; 8], *const (u8, i8), &'static &'static i32, &'static [i8; 2]), u32, *const [u128; 8], &'static *const [u128; 8]);
let _24: u16;
let _25: Adt31;
let _26: ();
let _27: ();
{
_3 = _1 - _1;
_7 = [_6,_6,_6,_5.fld2];
_3 = _1 + _1;
_6 = _5.fld2 - _4;
_5.fld1 = [1727274233_i32,1560285391_i32,1477636468_i32,(-1931082457_i32)];
_5.fld3 = [188_u8,75_u8];
_10.0 = ['\u{74553}','\u{10922b}','\u{1961a}','\u{10843}','\u{76c07}','\u{68c5e}'];
_8 = (_10.0, _1, (-971883609229864694_i64));
_8.2 = (-4586289895428890751_i64);
_9 = 836621727503574358_u64 as i128;
_5.fld1 = [1549345516_i32,425190076_i32,(-1066728636_i32),577828385_i32];
_10.1 = 31941868383728218980933520462215305148_u128 as f32;
_3 = -_1;
_3 = _1 - _8.1;
_10.0 = _8.0;
_10 = _8;
_5.fld2 = 13819063624969742072950746149179970724_u128 as i16;
_10 = _8;
Goto(bb1)
}
bb1 = {
_1 = _8.1;
_5.fld1 = [(-654001097_i32),(-1640505666_i32),140988826_i32,(-1406831078_i32)];
_1 = _8.1 + _8.1;
_10.2 = (-9223372036854775808_isize) as i64;
_4 = _6 + _6;
RET = !_5.fld0;
_2 = [_6,_6,_6,_4];
_5.fld3 = [11_u8,72_u8];
_3 = _10.1 * _10.1;
_8.1 = _3;
_5.fld1 = [664832076_i32,91716123_i32,2133990528_i32,(-2046557096_i32)];
_1 = -_3;
_12 = -_4;
_8.2 = !_10.2;
_5.fld2 = 3_usize as i16;
_5.fld1 = [1583966352_i32,1264170577_i32,(-2036641707_i32),269036175_i32];
_4 = _6;
Goto(bb2)
}
bb2 = {
_4 = -_6;
_3 = _4 as f32;
_2 = [_6,_6,_6,_4];
_5.fld1 = [(-507869404_i32),895046402_i32,(-1899333907_i32),1167206798_i32];
_3 = _10.1;
_4 = 741038007353406488_u64 as i16;
_10.0 = ['\u{c6896}','\u{5ee0b}','\u{756}','\u{d3de}','\u{520d9}','\u{65999}'];
_7 = [_6,_6,_12,_6];
_9 = 161602502537069362679657978648630793010_i128;
_13 = &_8.1;
_10.0 = _8.0;
_5.fld1 = [1361399956_i32,1862027901_i32,490918885_i32,(-679001174_i32)];
_7 = _2;
_10.0 = ['\u{10b104}','\u{9f37a}','\u{57c75}','\u{70088}','\u{54332}','\u{7e046}'];
_9 = (-54702630434090432435879009920955837820_i128) ^ 43673000315970851770165420113230186977_i128;
_12 = 61166950573007903004711966410815623746_u128 as i16;
_6 = _12;
_5.fld2 = _12;
Goto(bb3)
}
bb3 = {
_8.0 = ['\u{682e8}','\u{e18e6}','\u{ebc01}','\u{10e584}','\u{70ec4}','\u{94c15}'];
_8.1 = _10.1 - _3;
_14 = -9223372036854775807_isize;
_10.2 = _8.2;
_7 = _2;
RET = _5.fld0;
_8 = (_10.0, _10.1, _10.2);
_8.0 = ['\u{29b7a}','\u{8450f}','\u{90120}','\u{fa37b}','\u{45034}','\u{9c39f}'];
_15 = !true;
_9 = -71904394094915954959086626213272561951_i128;
_8.1 = _9 as f32;
Goto(bb4)
}
bb4 = {
_3 = 187552555745881637879837355539462160398_u128 as f32;
_5.fld1 = [1193845_i32,(-144218112_i32),51285222_i32,(-1503467784_i32)];
_17.1 = core::ptr::addr_of_mut!(_1);
_5.fld2 = 320635673424008378007636425888147078331_u128 as i16;
_17.2 = [(-117_i8),(-113_i8)];
_3 = _1 - _1;
_17.0.0 = [254547293292834350822730799027919953381_u128,60926315775512509721059026233433784525_u128,56331936343953813505422381261957077912_u128,234401121732270023088323728624116541653_u128,165720138884255243103367687649199200639_u128,55888273148078090640805907284833447696_u128,205158121264909669317284357758099658400_u128,279055055087307174739341551483694372020_u128];
_13 = &_3;
_8 = (_10.0, (*_13), _10.2);
_8.1 = (*_13);
_15 = true & false;
RET = _8.2 as u32;
_10.0 = ['\u{cb31}','\u{10ee32}','\u{b9e90}','\u{fe294}','\u{2c425}','\u{280e3}'];
_6 = _5.fld2;
_10.1 = 60325_u16 as f32;
_13 = &(*_13);
_8 = (_10.0, (*_13), _10.2);
_5.fld3 = [158_u8,208_u8];
_9 = 5_usize as i128;
_6 = '\u{41ef2}' as i16;
_1 = (*_13);
_8 = (_10.0, (*_13), _10.2);
_3 = _14 as f32;
Goto(bb5)
}
bb5 = {
_1 = _8.1;
_19 = !_9;
_13 = &_3;
_19 = _9;
_17.0.0 = [313685610156026398166051273876025574289_u128,63312299364020293212317447633488147602_u128,134178742685392710924489756605171098883_u128,67571019723733223832664739525206192134_u128,7589722694317725037406173009902120222_u128,225635349470528838349660359825170049514_u128,205855193682567155717755525714978471702_u128,135860021133335056064487999002898031264_u128];
_7 = [_5.fld2,_5.fld2,_4,_4];
_10.0 = ['\u{5ece9}','\u{df4b1}','\u{cb336}','\u{82b7f}','\u{44fa9}','\u{4d3de}'];
_17.0.3 = &_17.2;
RET = 51676275_i32 as u32;
_8 = _10;
_19 = _10.2 as i128;
_14 = _15 as isize;
_8.2 = _10.2;
_19 = _9 << _4;
_18 = _15;
_17.0.0 = [149613915137277662787017919155938212922_u128,112397538874755217081570704114416138823_u128,3705558865124408952018900436617800398_u128,38679430540979904529897986847721109891_u128,308591086793503219154492163893317491220_u128,274256386895762449598412254129653434251_u128,151359565427665555084075067579793683119_u128,325028954156701464474108441700726905880_u128];
Goto(bb6)
}
bb6 = {
_3 = _4 as f32;
_12 = _5.fld2;
_19 = '\u{72e5e}' as i128;
_4 = _5.fld2 | _6;
_17.0.0 = [200579628264594821316662504872383683238_u128,78865749822236430560098759440021428034_u128,44302599071123808746546053533263188108_u128,10760342353410146052189586322220555245_u128,71859836121694522800640398327505590309_u128,197059656649328218250921228287569383288_u128,176405006468504020041426053819047906721_u128,190921308040158498154654870043717824225_u128];
_8 = _10;
_9 = _19 * _19;
_19 = -_9;
_22 = _1 as isize;
_17.0.0 = [328514437776961332529662921094910579710_u128,273147407620407757692416056123574199671_u128,268186265004350563115568642617379501682_u128,209352407167278075873688732251697566550_u128,314141972016538100351936991946027025975_u128,286659915682983972833994947380095811999_u128,119579843017200235766356294483977824787_u128,52090975153483026102882224762829901318_u128];
_18 = !_15;
_17.1 = core::ptr::addr_of_mut!(_3);
_13 = &_10.1;
_16 = 1818874961842764651_u64 as i8;
_18 = _1 < _1;
_23.1 = !RET;
RET = _18 as u32;
Goto(bb7)
}
bb7 = {
Call(_26 = dump_var(16_usize, 6_usize, Move(_6), 18_usize, Move(_18), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_26 = dump_var(16_usize, 19_usize, Move(_19), 9_usize, Move(_9), 27_usize, _27, 27_usize, _27), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i16,mut _2: u32,mut _3: isize,mut _4: Adt31,mut _5: u32,mut _6: isize,mut _7: i16,mut _8: i16,mut _9: isize,mut _10: i16,mut _11: u32,mut _12: Adt31) -> Adt31 {
mir! {
type RET = Adt31;
let _13: (&'static u16, Adt31);
let _14: bool;
let _15: i16;
let _16: ();
let _17: ();
{
_12.fld2 = (-1025074993764638613_i64) as i16;
RET = Adt31 { fld0: _5,fld1: _4.fld1,fld2: _4.fld2,fld3: _12.fld3 };
_4.fld1 = [(-2130000094_i32),161334678_i32,835757065_i32,528587824_i32];
_4.fld2 = -_10;
_9 = -_3;
RET.fld1 = [(-1230801884_i32),(-1630711538_i32),(-788283599_i32),(-824644984_i32)];
_13.1.fld2 = !_4.fld2;
_4 = RET;
_8 = _4.fld2;
_13.1 = Adt31 { fld0: _5,fld1: RET.fld1,fld2: _7,fld3: RET.fld3 };
RET.fld3 = [208_u8,208_u8];
_5 = _2;
_3 = 10_u8 as isize;
_2 = (-6480696955548042044_i64) as u32;
_4.fld1 = _13.1.fld1;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(17_usize, 5_usize, Move(_5), 2_usize, Move(_2), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(17_usize, 3_usize, Move(_3), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: &'static [i8; 2],mut _2: &'static i64,mut _3: u32,mut _4: i16,mut _5: u32,mut _6: i16,mut _7: i16,mut _8: u32,mut _9: u32) -> [i16; 4] {
mir! {
type RET = [i16; 4];
let _10: ();
let _11: ();
{
_9 = _5 << _7;
_3 = !_9;
_9 = !_3;
_9 = _8;
RET = [_7,_4,_4,_4];
_5 = !_8;
_8 = _5 ^ _5;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(18_usize, 6_usize, Move(_6), 5_usize, Move(_5), 4_usize, Move(_4), 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(57467033861030233021012587508176732809_i128), std::hint::black_box(201877837586703436849546134406827470709_u128), std::hint::black_box(1428237996297787320_u64), std::hint::black_box(9304_i16), std::hint::black_box(3900810488_u32), std::hint::black_box((-7687220262557972844_i64)));
                
            }
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt31{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt31 {
fld0: u32,
fld1: [i32; 4],
fld2: i16,
fld3: [u8; 2],
}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: i128,
fld1: char,
fld2: [u8; 2],
fld3: u16,
fld4: [i8; 2],
fld5: [u128; 8],
fld6: [char; 6],

},
Variant1{
fld0: [u64; 6],
fld1: *const (u8, i8),

},
Variant2{
fld0: u16,
fld1: i128,
fld2: [char; 6],

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: (u8, i8),
fld1: u64,
fld2: u8,
fld3: [char; 6],
fld4: ([char; 6], f32, i64),
fld5: i32,
fld6: i64,
fld7: (i32, (u8, i8), isize, u128),

},
Variant1{
fld0: u32,
fld1: *const isize,
fld2: u16,
fld3: i128,

},
Variant2{
fld0: *const isize,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: i64,

},
Variant1{
fld0: bool,
fld1: Adt39,
fld2: i8,

},
Variant2{
fld0: u128,
fld1: [i8; 2],
fld2: [char; 4],
fld3: [u64; 6],

},
Variant3{
fld0: [char; 6],
fld1: Adt39,
fld2: isize,
fld3: i128,
fld4: *mut [i32; 4],
fld5: (i32, (u8, i8), isize, u128),
fld6: [u128; 8],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [u8; 2],
fld1: *mut f32,
fld2: *const (u8, i8),

},
Variant1{
fld0: *mut [i32; 4],
fld1: [i32; 5],
fld2: f32,
fld3: *const (u8, i8),
fld4: u128,
fld5: i32,
fld6: Adt32,
fld7: *const [u128; 8],

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt61{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt61 {
fld0: Adt41,
}
impl PrintFDebug for Adt87{
	unsafe fn printf_debug(&self){unsafe{printf("Adt87::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt87 {
Variant0{
fld0: Adt32,
fld1: u64,
fld2: Adt61,
fld3: [isize; 2],
fld4: i16,
fld5: [char; 3],
fld6: (u8, i8),
fld7: i128,

},
Variant1{
fld0: *const [u128; 8],

}}
impl PrintFDebug for Adt88{
	unsafe fn printf_debug(&self){unsafe{printf("Adt88::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt88 {
Variant0{
fld0: Adt61,
fld1: (u8, i8),
fld2: *mut [i8; 2],
fld3: *mut f64,
fld4: ([i32; 5],),

},
Variant1{
fld0: *mut f32,
fld1: u64,
fld2: [i32; 4],
fld3: i128,
fld4: i16,
fld5: [isize; 8],

},
Variant2{
fld0: Adt87,
fld1: f32,
fld2: isize,
fld3: *mut f64,
fld4: u16,
fld5: ([i32; 5],),
fld6: [i32; 6],

},
Variant3{
fld0: [char; 3],
fld1: char,
fld2: *const [u128; 8],
fld3: [char; 4],
fld4: *mut f32,
fld5: i32,
fld6: [i16; 4],
fld7: i128,

}}

