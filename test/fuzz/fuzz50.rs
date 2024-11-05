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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: u32,mut _5: u64,mut _6: u16,mut _7: i64,mut _8: i128,mut _9: usize) -> u128 {
mir! {
type RET = u128;
let _10: i8;
let _11: f32;
let _12: Adt54;
let _13: bool;
let _14: i64;
let _15: u128;
let _16: [bool; 1];
let _17: isize;
let _18: char;
let _19: isize;
let _20: [u32; 1];
let _21: isize;
let _22: bool;
let _23: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _24: f32;
let _25: Adt49;
let _26: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _27: ([bool; 1], usize, (u64, u16, [bool; 3], i128));
let _28: char;
let _29: f32;
let _30: (u16,);
let _31: ();
let _32: ();
{
_8 = 796054447_i32 as i128;
_7 = 4231528362574269604_i64 + (-8044348978868290120_i64);
RET = 224198108696861054860445652637542995142_u128;
_4 = (-1712507728_i32) as u32;
_10 = (-108_i8);
_1 = _7 != _7;
_11 = _4 as f32;
_12.fld0 = core::ptr::addr_of_mut!(_9);
_10 = (-1590346791_i32) as i8;
_12.fld4 = 8044480802064929778_u64;
_4 = 2302103980_u32 << _12.fld4;
_13 = _1;
_3 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_14 = !_7;
_12.fld3 = _4 >> _7;
_12.fld4 = 8288322403375816915_u64 + 14898312624079734197_u64;
match RET {
0 => bb1,
1 => bb2,
224198108696861054860445652637542995142 => bb4,
_ => bb3
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
_5 = RET as u64;
Call(_2 = fn1(_12.fld0, _7, _13, _12.fld0, _12.fld3, _13, _12.fld3, _1, _12.fld0, _12.fld3, _13, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6 = 63301_u16;
_16 = [_1];
_12.fld5 = 881810503_i32;
_12.fld3 = _4 * _4;
_11 = RET as f32;
_12.fld1 = RET;
_11 = 13_u8 as f32;
_3 = !9223372036854775807_isize;
RET = _12.fld1;
_16 = [_1];
_11 = _6 as f32;
_5 = _12.fld4;
_3 = (-9223372036854775808_isize);
_18 = _2;
match RET {
0 => bb1,
224198108696861054860445652637542995142 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_21 = !_3;
_12.fld1 = RET - RET;
_12.fld3 = _14 as u32;
_18 = _2;
_18 = _2;
_23.2.2.1 = _6 - _6;
_23.2.2.0 = _12.fld4 + _5;
RET = !_12.fld1;
_23.2.2.1 = !_6;
_20 = [_12.fld3];
_5 = !_23.2.2.0;
_20 = [_4];
Goto(bb8)
}
bb8 = {
_23.0.0 = !_23.2.2.1;
_13 = _1;
_11 = (-28581_i16) as f32;
_23.2.0 = [_13];
RET = _12.fld1;
_23.2.1 = _9 + _9;
_17 = _12.fld3 as isize;
_23.2.0 = _16;
_23.2.2.3 = _8 & _8;
_12.fld2 = Adt40::Variant1 { fld0: _2 };
_22 = _1;
_27.2.0 = _2 as u64;
_12.fld5 = (-314965492_i32);
_12.fld4 = _27.2.0;
_12.fld5 = (-900981096_i32);
_26.2.0 = _23.2.0;
SetDiscriminant(_12.fld2, 1);
_12.fld5 = (-917953559_i32) - (-1396206231_i32);
Call(_26.2.2.1 = core::intrinsics::bswap(_23.2.2.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_20 = [_12.fld3];
_1 = _13;
match _6 {
0 => bb10,
1 => bb11,
2 => bb12,
63301 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_21 = !_3;
_12.fld1 = RET - RET;
_12.fld3 = _14 as u32;
_18 = _2;
_18 = _2;
_23.2.2.1 = _6 - _6;
_23.2.2.0 = _12.fld4 + _5;
RET = !_12.fld1;
_23.2.2.1 = !_6;
_20 = [_12.fld3];
_5 = !_23.2.2.0;
_20 = [_4];
Goto(bb8)
}
bb12 = {
_5 = RET as u64;
Call(_2 = fn1(_12.fld0, _7, _13, _12.fld0, _12.fld3, _13, _12.fld3, _1, _12.fld0, _12.fld3, _13, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
_22 = !_13;
_27.2.1 = _23.2.2.1 & _23.0.0;
_15 = RET - _12.fld1;
_21 = _5 as isize;
_26.2.0 = _23.2.0;
_18 = _2;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(0_usize, 4_usize, Move(_4), 22_usize, Move(_22), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(0_usize, 5_usize, Move(_5), 14_usize, Move(_14), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(0_usize, 9_usize, Move(_9), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: *mut usize,mut _2: i64,mut _3: bool,mut _4: *mut usize,mut _5: u32,mut _6: bool,mut _7: u32,mut _8: bool,mut _9: *mut usize,mut _10: u32,mut _11: bool,mut _12: i64) -> char {
mir! {
type RET = char;
let _13: char;
let _14: f64;
let _15: *mut &'static isize;
let _16: Adt45;
let _17: Adt39;
let _18: f32;
let _19: char;
let _20: *mut usize;
let _21: isize;
let _22: i64;
let _23: [char; 8];
let _24: isize;
let _25: *mut &'static isize;
let _26: f64;
let _27: Adt55;
let _28: i32;
let _29: Adt44;
let _30: ();
let _31: ();
{
_2 = _12;
(*_4) = '\u{1099a9}' as usize;
_4 = _9;
_1 = core::ptr::addr_of_mut!((*_4));
(*_4) = (-28132_i16) as usize;
_11 = (*_9) > (*_9);
_9 = core::ptr::addr_of_mut!((*_1));
(*_1) = 1_usize;
_4 = core::ptr::addr_of_mut!((*_4));
_8 = !_6;
RET = '\u{c9e28}';
_4 = _9;
_2 = _12;
_7 = _5 ^ _10;
_13 = RET;
_8 = _3;
RET = _13;
match (*_4) {
0 => bb1,
1 => bb4,
_ => bb3
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
(*_9) = !9528567219563099530_usize;
_4 = _1;
(*_9) = !8177143161475212167_usize;
_3 = _6;
_6 = !_8;
(*_9) = 2333875101987568394_usize ^ 0_usize;
RET = _13;
Goto(bb5)
}
bb5 = {
_16.fld2 = Adt44::Variant1 { fld0: 13405437876123716934_u64 };
_6 = !_11;
(*_9) = 7_usize;
_8 = _3 ^ _11;
_14 = _10 as f64;
_7 = _5 * _10;
_16.fld0 = (_12,);
_16.fld3 = (-50_i8);
_17.fld3 = _16.fld3;
place!(Field::<u64>(Variant(_16.fld2, 1), 0)) = 28346_i16 as u64;
_6 = _7 <= _5;
_4 = _9;
_14 = 10_u8 as f64;
(*_9) = 1005039687206476351_usize + 1_usize;
_17.fld5 = _9;
(*_1) = 4_usize;
_16.fld1 = core::ptr::addr_of_mut!((*_9));
_17.fld1 = _17.fld3 as u16;
_18 = _12 as f32;
_18 = 150795636554821933326288618882582269700_u128 as f32;
Call((*_1) = fn2(_3, _6, _17.fld1, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
SetDiscriminant(_16.fld2, 1);
_17.fld4 = core::ptr::addr_of_mut!(_17.fld1);
_16.fld3 = -_17.fld3;
_19 = RET;
_17.fld4 = core::ptr::addr_of_mut!(_17.fld1);
match _17.fld3 {
340282366920938463463374607431768211406 => bb7,
_ => bb2
}
}
bb7 = {
(*_1) = !4_usize;
_20 = _9;
(*_4) = 11655756167302927563_usize;
match (*_20) {
0 => bb4,
11655756167302927563 => bb9,
_ => bb8
}
}
bb8 = {
_16.fld2 = Adt44::Variant1 { fld0: 13405437876123716934_u64 };
_6 = !_11;
(*_9) = 7_usize;
_8 = _3 ^ _11;
_14 = _10 as f64;
_7 = _5 * _10;
_16.fld0 = (_12,);
_16.fld3 = (-50_i8);
_17.fld3 = _16.fld3;
place!(Field::<u64>(Variant(_16.fld2, 1), 0)) = 28346_i16 as u64;
_6 = _7 <= _5;
_4 = _9;
_14 = 10_u8 as f64;
(*_9) = 1005039687206476351_usize + 1_usize;
_17.fld5 = _9;
(*_1) = 4_usize;
_16.fld1 = core::ptr::addr_of_mut!((*_9));
_17.fld1 = _17.fld3 as u16;
_18 = _12 as f32;
_18 = 150795636554821933326288618882582269700_u128 as f32;
Call((*_1) = fn2(_3, _6, _17.fld1, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb9 = {
(*_4) = 209_u8 as usize;
RET = _13;
_17.fld2 = [_19,RET,_19,RET,RET,RET,RET,_13];
_17.fld0 = [_8,_8,_3];
_17.fld3 = !_16.fld3;
_17.fld5 = _20;
Goto(bb10)
}
bb10 = {
(*_1) = _18 as usize;
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = _18 as usize;
_7 = !_5;
(*_4) = 15961760355631658127_usize * 1015415111018766262_usize;
_16.fld3 = !_17.fld3;
_17.fld0 = [_8,_6,_8];
_21 = !9223372036854775807_isize;
(*_4) = 12610580057547056105_usize & 6_usize;
_12 = _2;
_16.fld0 = (_2,);
_9 = core::ptr::addr_of_mut!((*_1));
_18 = _14 as f32;
Goto(bb11)
}
bb11 = {
_17.fld0 = [_6,_6,_6];
_19 = _13;
(*_1) = _12 as usize;
_17.fld4 = core::ptr::addr_of_mut!(_17.fld1);
Call((*_9) = core::intrinsics::bswap(6_usize), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _19;
_17.fld0 = [_6,_8,_8];
_16.fld0 = (_2,);
_20 = core::ptr::addr_of_mut!((*_4));
RET = _13;
_9 = _16.fld1;
_16.fld2 = Adt44::Variant1 { fld0: 11672671612925401709_u64 };
_18 = _5 as f32;
(*_1) = 2179151154785825508_usize - 12815934464642979221_usize;
_2 = -_16.fld0.0;
(*_9) = !8328829170414720489_usize;
(*_1) = 4_usize;
_17.fld4 = core::ptr::addr_of_mut!(_17.fld1);
_23 = [_19,_13,_19,_13,_19,RET,_19,_19];
_17.fld4 = core::ptr::addr_of_mut!(_17.fld1);
_13 = RET;
_13 = _19;
_6 = _8;
_5 = _7;
_18 = (-167399423785299539483394129048254214986_i128) as f32;
_5 = !_7;
_9 = _16.fld1;
RET = _13;
Call((*_4) = fn3(_20, _7, _17.fld4, _17, RET, _17, _17, _16.fld1, _10), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_1) = 6273_i16 as usize;
(*_1) = 16793737252316136596_usize;
_17.fld4 = core::ptr::addr_of_mut!(_17.fld1);
(*_4) = 2617498805644490629_usize;
RET = _13;
place!(Field::<u64>(Variant(_16.fld2, 1), 0)) = 12815448721429951760962534378085528319_i128 as u64;
_17.fld3 = -_16.fld3;
_5 = !_7;
_17.fld2 = _23;
(*_9) = !0_usize;
Goto(bb14)
}
bb14 = {
_26 = 231505584076507857616568008539155127892_u128 as f64;
RET = _13;
(*_4) = 5_usize;
(*_9) = 12764707911722327160_usize * 10287596603142523110_usize;
_22 = _16.fld0.0;
_12 = _22 - _22;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(1_usize, 10_usize, Move(_10), 7_usize, Move(_7), 19_usize, Move(_19), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(1_usize, 3_usize, Move(_3), 2_usize, Move(_2), 8_usize, Move(_8), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: bool,mut _3: u16,mut _4: bool) -> usize {
mir! {
type RET = usize;
let _5: [bool; 3];
let _6: ([bool; 1], usize, (u64, u16, [bool; 3], i128));
let _7: f64;
let _8: (u16,);
let _9: (i64,);
let _10: char;
let _11: [bool; 1];
let _12: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _13: *mut u16;
let _14: Adt42;
let _15: Adt42;
let _16: i128;
let _17: bool;
let _18: [bool; 3];
let _19: char;
let _20: isize;
let _21: f64;
let _22: u64;
let _23: f32;
let _24: (f64, u32, u8, bool);
let _25: Adt48;
let _26: ();
let _27: ();
{
RET = 174785966165228378043573078041053954593_u128 as usize;
_3 = 18692_u16;
_5 = [_2,_4,_1];
_4 = !_2;
_5 = [_2,_2,_4];
Goto(bb1)
}
bb1 = {
_6.2 = (6675866338224683488_u64, _3, _5, (-76254300931168057232419694107326257988_i128));
_6.1 = RET >> _3;
_1 = _4 ^ _2;
_6.2.2 = [_2,_1,_4];
_6.2.1 = _1 as u16;
_6.1 = RET << _6.2.1;
_7 = 26_isize as f64;
_4 = _1;
_3 = !_6.2.1;
_8.0 = _3;
_9 = ((-6061350176571825065_i64),);
_6.2.0 = !2976239208968114910_u64;
_6.2.2 = _5;
RET = (-32_i8) as usize;
_12.0.0 = !_6.2.1;
_6.2.0 = !6876712950117064689_u64;
RET = '\u{6030f}' as usize;
_12.2.2.1 = _8.0 + _12.0.0;
_4 = !_1;
_12.2.2.2 = [_4,_1,_1];
match _6.2.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
264028065989770406230954913324441953468 => bb10,
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
_12.2.2.3 = (-29970_i16) as i128;
_7 = 9223372036854775807_isize as f64;
_12.2.0 = [_1];
_17 = !_2;
match _6.2.3 {
0 => bb8,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
264028065989770406230954913324441953468 => bb18,
_ => bb17
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
_6.2 = (6675866338224683488_u64, _3, _5, (-76254300931168057232419694107326257988_i128));
_6.1 = RET >> _3;
_1 = _4 ^ _2;
_6.2.2 = [_2,_1,_4];
_6.2.1 = _1 as u16;
_6.1 = RET << _6.2.1;
_7 = 26_isize as f64;
_4 = _1;
_3 = !_6.2.1;
_8.0 = _3;
_9 = ((-6061350176571825065_i64),);
_6.2.0 = !2976239208968114910_u64;
_6.2.2 = _5;
RET = (-32_i8) as usize;
_12.0.0 = !_6.2.1;
_6.2.0 = !6876712950117064689_u64;
RET = '\u{6030f}' as usize;
_12.2.2.1 = _8.0 + _12.0.0;
_4 = !_1;
_12.2.2.2 = [_4,_1,_1];
match _6.2.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
264028065989770406230954913324441953468 => bb10,
_ => bb9
}
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_12.2.0 = [_4];
_12.2.2 = (_6.2.0, _3, _6.2.2, _6.2.3);
_6 = (_12.2.0, RET, _12.2.2);
_16 = _12.2.2.3 - _12.2.2.3;
_12 = (_8, _6.0, _6);
RET = _12.2.1;
_6.1 = !_12.2.1;
_12.0.0 = _8.0 ^ _3;
_6.2.2 = [_4,_17,_17];
_12.2.2.0 = _6.2.0;
_12 = (_8, _6.0, _6);
_21 = -_7;
_6 = (_12.1, _12.2.1, _12.2.2);
_11 = [_1];
_22 = _12.2.2.0;
_8 = _12.0;
RET = _12.2.1;
_17 = !_1;
_12.2.2.3 = -_16;
_23 = 11966_i16 as f32;
_12.2.2.1 = _3;
_23 = 149252304_u32 as f32;
_13 = core::ptr::addr_of_mut!(_3);
_12.2.2 = (_6.2.0, _3, _6.2.2, _16);
_6.2.1 = (*_13);
Goto(bb19)
}
bb19 = {
Call(_26 = dump_var(2_usize, 22_usize, Move(_22), 17_usize, Move(_17), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_26 = dump_var(2_usize, 4_usize, Move(_4), 2_usize, Move(_2), 27_usize, _27, 27_usize, _27), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: *mut usize,mut _2: u32,mut _3: *mut u16,mut _4: Adt39,mut _5: char,mut _6: Adt39,mut _7: Adt39,mut _8: *mut usize,mut _9: u32) -> usize {
mir! {
type RET = usize;
let _10: isize;
let _11: [bool; 3];
let _12: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _13: Adt48;
let _14: i8;
let _15: char;
let _16: char;
let _17: f64;
let _18: (u16,);
let _19: bool;
let _20: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _21: i64;
let _22: char;
let _23: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _24: Adt45;
let _25: i128;
let _26: isize;
let _27: i128;
let _28: ();
let _29: ();
{
_6.fld4 = _3;
_4.fld5 = _7.fld5;
_3 = _7.fld4;
Call(_7.fld0 = fn4(_4, _4, _4, _4.fld3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 13279526407448270546_usize & 2_usize;
_4 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _6.fld2,fld3: _7.fld3,fld4: _3,fld5: _8 };
_4 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4,fld5: _7.fld5 };
_9 = _2 * _2;
_3 = core::ptr::addr_of_mut!((*_3));
_4.fld5 = core::ptr::addr_of_mut!(RET);
RET = _9 as usize;
Goto(bb2)
}
bb2 = {
_7 = _4;
_4.fld3 = 30348_i16 as i8;
_7 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _4.fld2,fld3: _4.fld3,fld4: _3,fld5: _4.fld5 };
Goto(bb3)
}
bb3 = {
_6.fld4 = _4.fld4;
(*_3) = _7.fld1 + _7.fld1;
_7.fld1 = true as u16;
_6.fld0 = [true,true,true];
_2 = _9;
_4 = Adt39 { fld0: _7.fld0,fld1: (*_3),fld2: _6.fld2,fld3: _7.fld3,fld4: _7.fld4,fld5: _7.fld5 };
_4.fld4 = _6.fld4;
_12.2.2 = (11854766455514971261_u64, _4.fld1, _4.fld0, 77449033490033568642957433028960715994_i128);
_7.fld3 = _6.fld3;
_12.0.0 = !(*_3);
_12.1 = [true];
_12.2.0 = [false];
_12.2.2.1 = !(*_3);
_12.2.1 = RET;
_12.2.2.1 = !(*_3);
_12.2.2.1 = !_12.0.0;
_2 = _9 << (*_3);
Goto(bb4)
}
bb4 = {
_13.fld3.0 = !_12.0.0;
_7.fld1 = _12.2.2.1;
_9 = !_2;
_14 = _12.2.2.3 as i8;
_5 = '\u{108b3b}';
_13.fld2.0 = 7306301044081457870_i64;
_12.2.2.0 = !15515626460716614093_u64;
_12.2.2.0 = (-9223372036854775808_isize) as u64;
_6.fld1 = !_13.fld3.0;
_15 = _5;
_9 = _15 as u32;
_3 = core::ptr::addr_of_mut!(_4.fld1);
_13.fld4 = Adt39 { fld0: _7.fld0,fld1: _12.0.0,fld2: _6.fld2,fld3: _14,fld4: _3,fld5: _1 };
_7.fld5 = core::ptr::addr_of_mut!(_12.2.1);
_15 = _5;
_11 = [false,false,false];
_13.fld4.fld3 = true as i8;
_13.fld1 = !1540890562_i32;
_6.fld5 = core::ptr::addr_of_mut!(RET);
_9 = !_2;
Goto(bb5)
}
bb5 = {
_19 = true;
_17 = _12.2.2.3 as f64;
_13.fld2 = (783667373290963181_i64,);
_4.fld2 = [_15,_5,_15,_5,_15,_15,_5,_15];
_20.0.1 = 15108_i16 as u32;
_4.fld1 = (-16566_i16) as u16;
_7.fld1 = _12.2.1 as u16;
_6.fld1 = _9 as u16;
match _12.2.2.3 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
77449033490033568642957433028960715994 => bb10,
_ => bb9
}
}
bb6 = {
_13.fld3.0 = !_12.0.0;
_7.fld1 = _12.2.2.1;
_9 = !_2;
_14 = _12.2.2.3 as i8;
_5 = '\u{108b3b}';
_13.fld2.0 = 7306301044081457870_i64;
_12.2.2.0 = !15515626460716614093_u64;
_12.2.2.0 = (-9223372036854775808_isize) as u64;
_6.fld1 = !_13.fld3.0;
_15 = _5;
_9 = _15 as u32;
_3 = core::ptr::addr_of_mut!(_4.fld1);
_13.fld4 = Adt39 { fld0: _7.fld0,fld1: _12.0.0,fld2: _6.fld2,fld3: _14,fld4: _3,fld5: _1 };
_7.fld5 = core::ptr::addr_of_mut!(_12.2.1);
_15 = _5;
_11 = [false,false,false];
_13.fld4.fld3 = true as i8;
_13.fld1 = !1540890562_i32;
_6.fld5 = core::ptr::addr_of_mut!(RET);
_9 = !_2;
Goto(bb5)
}
bb7 = {
_6.fld4 = _4.fld4;
(*_3) = _7.fld1 + _7.fld1;
_7.fld1 = true as u16;
_6.fld0 = [true,true,true];
_2 = _9;
_4 = Adt39 { fld0: _7.fld0,fld1: (*_3),fld2: _6.fld2,fld3: _7.fld3,fld4: _7.fld4,fld5: _7.fld5 };
_4.fld4 = _6.fld4;
_12.2.2 = (11854766455514971261_u64, _4.fld1, _4.fld0, 77449033490033568642957433028960715994_i128);
_7.fld3 = _6.fld3;
_12.0.0 = !(*_3);
_12.1 = [true];
_12.2.0 = [false];
_12.2.2.1 = !(*_3);
_12.2.1 = RET;
_12.2.2.1 = !(*_3);
_12.2.2.1 = !_12.0.0;
_2 = _9 << (*_3);
Goto(bb4)
}
bb8 = {
_7 = _4;
_4.fld3 = 30348_i16 as i8;
_7 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _4.fld2,fld3: _4.fld3,fld4: _3,fld5: _4.fld5 };
Goto(bb3)
}
bb9 = {
RET = 13279526407448270546_usize & 2_usize;
_4 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _6.fld2,fld3: _7.fld3,fld4: _3,fld5: _8 };
_4 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4,fld5: _7.fld5 };
_9 = _2 * _2;
_3 = core::ptr::addr_of_mut!((*_3));
_4.fld5 = core::ptr::addr_of_mut!(RET);
RET = _9 as usize;
Goto(bb2)
}
bb10 = {
_12.0.0 = 99706281584448381582060001241698380881_u128 as u16;
_1 = _4.fld5;
_18 = _13.fld3;
_13.fld4.fld0 = [_19,_19,_19];
_17 = _13.fld2.0 as f64;
_13.fld4.fld0 = [_19,_19,_19];
_6.fld3 = _6.fld1 as i8;
_8 = core::ptr::addr_of_mut!((*_1));
_7.fld1 = _12.2.2.1 * _18.0;
_13.fld0 = [_19];
_4.fld0 = [_19,_19,_19];
_13.fld4 = Adt39 { fld0: _12.2.2.2,fld1: _13.fld3.0,fld2: _6.fld2,fld3: _14,fld4: _7.fld4,fld5: _4.fld5 };
(*_1) = _12.2.1;
_12.2.0 = [_19];
_3 = core::ptr::addr_of_mut!(_13.fld3.0);
_20.0 = (_17, _2, 98_u8, _19);
_7.fld3 = _14 * _14;
_12.2.1 = !RET;
Goto(bb11)
}
bb11 = {
_7.fld1 = (*_3);
_13.fld4 = Adt39 { fld0: _12.2.2.2,fld1: _13.fld3.0,fld2: _4.fld2,fld3: _7.fld3,fld4: _4.fld4,fld5: _6.fld5 };
_7.fld1 = (*_3);
_20.2 = [_15,_5,_15,_5,_15,_15,_15,_5];
_24.fld0.0 = _13.fld2.0 | _13.fld2.0;
_20.0.3 = _19;
_23.0 = _20.0;
_14 = _7.fld3;
_20.0.2 = _23.0.2 >> _13.fld3.0;
_4 = Adt39 { fld0: _13.fld4.fld0,fld1: _7.fld1,fld2: _13.fld4.fld2,fld3: _7.fld3,fld4: _13.fld4.fld4,fld5: _13.fld4.fld5 };
_17 = _20.0.0 + _23.0.0;
_20 = (_23.0, _18, _13.fld4.fld2);
_14 = _12.2.1 as i8;
_6.fld1 = _20.1.0 >> _13.fld4.fld1;
_23.0.1 = _20.0.1 >> _7.fld3;
_20.0.0 = -_17;
Call(_23 = fn16(_9, _4.fld4, _20.1.0, _8, _13.fld4.fld4, _20.0.2, (*_3), _4.fld1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10 = (-9223372036854775808_isize);
_12.2.2.3 = (-108812693123203868415744793640614645340_i128);
match _20.0.2 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
98 => bb18,
_ => bb17
}
}
bb13 = {
_7.fld1 = (*_3);
_13.fld4 = Adt39 { fld0: _12.2.2.2,fld1: _13.fld3.0,fld2: _4.fld2,fld3: _7.fld3,fld4: _4.fld4,fld5: _6.fld5 };
_7.fld1 = (*_3);
_20.2 = [_15,_5,_15,_5,_15,_15,_15,_5];
_24.fld0.0 = _13.fld2.0 | _13.fld2.0;
_20.0.3 = _19;
_23.0 = _20.0;
_14 = _7.fld3;
_20.0.2 = _23.0.2 >> _13.fld3.0;
_4 = Adt39 { fld0: _13.fld4.fld0,fld1: _7.fld1,fld2: _13.fld4.fld2,fld3: _7.fld3,fld4: _13.fld4.fld4,fld5: _13.fld4.fld5 };
_17 = _20.0.0 + _23.0.0;
_20 = (_23.0, _18, _13.fld4.fld2);
_14 = _12.2.1 as i8;
_6.fld1 = _20.1.0 >> _13.fld4.fld1;
_23.0.1 = _20.0.1 >> _7.fld3;
_20.0.0 = -_17;
Call(_23 = fn16(_9, _4.fld4, _20.1.0, _8, _13.fld4.fld4, _20.0.2, (*_3), _4.fld1), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_12.0.0 = 99706281584448381582060001241698380881_u128 as u16;
_1 = _4.fld5;
_18 = _13.fld3;
_13.fld4.fld0 = [_19,_19,_19];
_17 = _13.fld2.0 as f64;
_13.fld4.fld0 = [_19,_19,_19];
_6.fld3 = _6.fld1 as i8;
_8 = core::ptr::addr_of_mut!((*_1));
_7.fld1 = _12.2.2.1 * _18.0;
_13.fld0 = [_19];
_4.fld0 = [_19,_19,_19];
_13.fld4 = Adt39 { fld0: _12.2.2.2,fld1: _13.fld3.0,fld2: _6.fld2,fld3: _14,fld4: _7.fld4,fld5: _4.fld5 };
(*_1) = _12.2.1;
_12.2.0 = [_19];
_3 = core::ptr::addr_of_mut!(_13.fld3.0);
_20.0 = (_17, _2, 98_u8, _19);
_7.fld3 = _14 * _14;
_12.2.1 = !RET;
Goto(bb11)
}
bb15 = {
RET = 13279526407448270546_usize & 2_usize;
_4 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _6.fld2,fld3: _7.fld3,fld4: _3,fld5: _8 };
_4 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _7.fld2,fld3: _7.fld3,fld4: _7.fld4,fld5: _7.fld5 };
_9 = _2 * _2;
_3 = core::ptr::addr_of_mut!((*_3));
_4.fld5 = core::ptr::addr_of_mut!(RET);
RET = _9 as usize;
Goto(bb2)
}
bb16 = {
_7 = _4;
_4.fld3 = 30348_i16 as i8;
_7 = Adt39 { fld0: _6.fld0,fld1: (*_3),fld2: _4.fld2,fld3: _4.fld3,fld4: _3,fld5: _4.fld5 };
Goto(bb3)
}
bb17 = {
_19 = true;
_17 = _12.2.2.3 as f64;
_13.fld2 = (783667373290963181_i64,);
_4.fld2 = [_15,_5,_15,_5,_15,_15,_5,_15];
_20.0.1 = 15108_i16 as u32;
_4.fld1 = (-16566_i16) as u16;
_7.fld1 = _12.2.1 as u16;
_6.fld1 = _9 as u16;
match _12.2.2.3 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
77449033490033568642957433028960715994 => bb10,
_ => bb9
}
}
bb18 = {
_26 = _13.fld1 as isize;
_12.1 = [_20.0.3];
_27 = _12.0.0 as i128;
Goto(bb19)
}
bb19 = {
Call(_28 = dump_var(3_usize, 10_usize, Move(_10), 18_usize, Move(_18), 19_usize, Move(_19), 14_usize, Move(_14)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(3_usize, 26_usize, Move(_26), 12_usize, Move(_12), 29_usize, _29, 29_usize, _29), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: Adt39,mut _2: Adt39,mut _3: Adt39,mut _4: i8) -> [bool; 3] {
mir! {
type RET = [bool; 3];
let _5: u64;
let _6: Adt41;
let _7: Adt44;
let _8: &'static isize;
let _9: isize;
let _10: [u128; 8];
let _11: isize;
let _12: f32;
let _13: [u128; 8];
let _14: bool;
let _15: (i64,);
let _16: (*const u8, u8, *const u8, &'static isize);
let _17: f64;
let _18: bool;
let _19: usize;
let _20: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _21: &'static isize;
let _22: i16;
let _23: [u128; 8];
let _24: bool;
let _25: isize;
let _26: *mut u16;
let _27: usize;
let _28: bool;
let _29: ([bool; 1], usize, (u64, u16, [bool; 3], i128));
let _30: isize;
let _31: f32;
let _32: (u16,);
let _33: (u16,);
let _34: char;
let _35: usize;
let _36: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _37: u128;
let _38: bool;
let _39: bool;
let _40: ([u32; 1], &'static isize, &'static isize, [bool; 1]);
let _41: Adt43;
let _42: ();
let _43: ();
{
_4 = !_1.fld3;
_1.fld5 = _3.fld5;
_3.fld3 = _3.fld1 as i8;
_3.fld3 = 11106784143165374332_u64 as i8;
_1.fld0 = [false,false,true];
_3.fld5 = _2.fld5;
_2 = Adt39 { fld0: _3.fld0,fld1: _1.fld1,fld2: _3.fld2,fld3: _3.fld3,fld4: _1.fld4,fld5: _1.fld5 };
_2.fld1 = _3.fld1 | _3.fld1;
_4 = _3.fld3;
_3.fld3 = (-73_isize) as i8;
_4 = _3.fld3;
_3.fld2 = _1.fld2;
_1.fld5 = _3.fld5;
RET = _1.fld0;
_2.fld2 = ['\u{83962}','\u{83831}','\u{e895c}','\u{a99b9}','\u{dff24}','\u{104f13}','\u{30864}','\u{d7009}'];
_1.fld3 = _2.fld3 | _4;
_2.fld4 = _1.fld4;
_9 = 207_u8 as isize;
_3.fld2 = _2.fld2;
RET = [false,true,true];
Call(_3.fld0 = fn5(_2, _2.fld5, _2.fld0, _2.fld0, _2, _2, _2.fld2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.fld3 = 60588080964146439883288740173873760898_i128 as i8;
_5 = 9289328095858366932_u64 >> _2.fld3;
Call(_3.fld4 = fn7(_1.fld3, _1.fld0, _2, _1, _3.fld0, _2.fld0, _3.fld3, _2, _1, _1, _2.fld2, _2.fld3, _2.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _5 as isize;
_2.fld5 = _1.fld5;
_3.fld5 = _1.fld5;
_1.fld0 = [true,true,true];
_5 = 15247394205815533504_u64;
_3.fld4 = core::ptr::addr_of_mut!(_1.fld1);
RET = [false,true,true];
_6 = Adt41::Variant0 { fld0: 131730049_i32 };
_10 = [126363144134255988623874437278831487573_u128,20150027383764483539864337687922736058_u128,167693219113470218678256489068340187439_u128,109789397996666114343709935005626622869_u128,321662376977846447811082208938030487948_u128,77454841133059711958358998975938051583_u128,195387540289279544176874551225232468104_u128,144230372456576643929351222038544444180_u128];
_1.fld3 = -_2.fld3;
_1 = _3;
_2.fld1 = _3.fld1;
_1.fld4 = _2.fld4;
_2.fld2 = ['\u{f24b7}','\u{4de97}','\u{1ea01}','\u{be439}','\u{6d76e}','\u{8dbd4}','\u{b7478}','\u{929d4}'];
_2.fld5 = _1.fld5;
place!(Field::<i32>(Variant(_6, 0), 0)) = (-1873916421_i32);
_2.fld5 = _1.fld5;
_2.fld3 = _4;
_7 = Adt44::Variant1 { fld0: _5 };
_3.fld5 = _2.fld5;
place!(Field::<i32>(Variant(_6, 0), 0)) = false as i32;
_14 = !true;
_2.fld5 = _3.fld5;
match _5 {
0 => bb1,
1 => bb3,
2 => bb4,
15247394205815533504 => bb6,
_ => bb5
}
}
bb3 = {
_3.fld3 = 60588080964146439883288740173873760898_i128 as i8;
_5 = 9289328095858366932_u64 >> _2.fld3;
Call(_3.fld4 = fn7(_1.fld3, _1.fld0, _2, _1, _3.fld0, _2.fld0, _3.fld3, _2, _1, _1, _2.fld2, _2.fld3, _2.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_1.fld4 = core::ptr::addr_of_mut!(_3.fld1);
_3.fld0 = [_14,_14,_14];
_2.fld4 = core::ptr::addr_of_mut!(_1.fld1);
RET = [_14,_14,_14];
_2.fld3 = !_3.fld3;
_1.fld1 = _3.fld1 - _2.fld1;
_1.fld5 = _2.fld5;
_2.fld5 = _1.fld5;
Goto(bb7)
}
bb7 = {
RET = [_14,_14,_14];
_13 = [285437114569415594362632596041061749969_u128,139066230991355090106254557073575289368_u128,206330953144633582513297257806818473966_u128,233128132770015436422274867022042403547_u128,68349257600209184260994955726823168166_u128,157665915700744953202351788427675852716_u128,65716575323384324711501294684125086286_u128,16422081133563265710664213300190299599_u128];
_15.0 = 16480942_u32 as i64;
_2.fld5 = _3.fld5;
_1.fld5 = _2.fld5;
_3 = Adt39 { fld0: _2.fld0,fld1: _1.fld1,fld2: _2.fld2,fld3: _1.fld3,fld4: _2.fld4,fld5: _1.fld5 };
_2.fld4 = core::ptr::addr_of_mut!(_1.fld1);
_18 = _14;
_10 = _13;
_1.fld3 = -_2.fld3;
_20.2.0 = [_14];
_1.fld4 = _2.fld4;
_18 = _14;
_22 = 1298_i16;
_20.0.0 = _22 as u16;
Goto(bb8)
}
bb8 = {
_20.2.2.2 = [_14,_18,_14];
_16.1 = !230_u8;
RET = [_18,_18,_18];
_20.2.1 = _20.0.0 as usize;
_15 = ((-7281093131539779044_i64),);
_2 = _1;
_3.fld0 = [_14,_18,_14];
_16.0 = core::ptr::addr_of!(_16.1);
_19 = !_20.2.1;
_1.fld2 = _3.fld2;
_20.0 = (_1.fld1,);
_2.fld5 = _3.fld5;
_16.2 = _16.0;
SetDiscriminant(_7, 0);
_17 = 126890929493767899290091220450307417301_u128 as f64;
_20.2.1 = _22 as usize;
_20.0.0 = !_2.fld1;
_20.1 = [_18];
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).0 = _17;
_29.2.2 = [_18,_18,_18];
_19 = !_20.2.1;
_2.fld2 = ['\u{5eca0}','\u{3186c}','\u{1c67e}','\u{793b3}','\u{13f40}','\u{166c}','\u{b172a}','\u{3514}'];
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).3 = _1.fld1 == _2.fld1;
_28 = _14;
Call(_12 = fn14(_2, _2.fld3, _17, _9, _2.fld3, _3, _3, _29.2.2, _29.2.2, _2.fld5, _1.fld0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3.fld3 = 1034225056_u32 as i8;
_26 = core::ptr::addr_of_mut!(_20.0.0);
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).2 = !_16.1;
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).2 = !_16.1;
place!(Field::<*const (u16,)>(Variant(_7, 0), 1)) = core::ptr::addr_of!(_20.0);
_8 = &_25;
_26 = _3.fld4;
SetDiscriminant(_6, 0);
_29.0 = [_18];
place!(Field::<bool>(Variant(_7, 0), 0)) = Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7).3;
_20.0.0 = (*_26) - _1.fld1;
_12 = 2337496861_u32 as f32;
_26 = _3.fld4;
_30 = !_9;
_15.0 = -6435766958230823610_i64;
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).2 = !_16.1;
_32 = (_1.fld1,);
_20.2.2.0 = Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7).3 as u64;
_17 = Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7).0 - Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7).0;
_29.2 = (_20.2.2.0, _2.fld1, _1.fld0, 74384391082027868039607747943977494313_i128);
match _29.2.3 {
74384391082027868039607747943977494313 => bb10,
_ => bb6
}
}
bb10 = {
place!(Field::<bool>(Variant(_7, 0), 0)) = _29.2.3 >= _29.2.3;
Goto(bb11)
}
bb11 = {
_29.2.1 = !_20.0.0;
place!(Field::<*mut u16>(Variant(_7, 0), 6)) = core::ptr::addr_of_mut!(place!(Field::<(u16,)>(Variant(_7, 0), 4)).0);
_19 = _20.2.1;
_31 = _12 - _12;
_4 = Field::<bool>(Variant(_7, 0), 0) as i8;
_20.0.0 = _20.2.2.0 as u16;
_20.2.0 = _29.0;
place!(Field::<*const (u16,)>(Variant(_7, 0), 1)) = core::ptr::addr_of!(place!(Field::<(u16,)>(Variant(_7, 0), 4)));
_3.fld5 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_7, 0), 3)));
_3.fld4 = core::ptr::addr_of_mut!(_20.0.0);
_29.2.3 = 247300130_u32 as i128;
place!(Field::<(u16,)>(Variant(_7, 0), 4)) = (_29.2.1,);
_16.0 = core::ptr::addr_of!(_36.0.2);
_33 = (_20.0.0,);
_10 = [224648871349112003504309268685550653073_u128,187326962879482267812548366793557821947_u128,45575009203120465452368555735481978264_u128,240338647038628197178724238739167293420_u128,285385652036830922328021960515270886567_u128,215591352106249617428645900981191622561_u128,218689562774086172527533494754227508162_u128,46021961685150086450979255332708575480_u128];
_10 = [332202611559898093754467482681880410528_u128,202136860738788169581597889906144166529_u128,265442032792296970243672424265665234469_u128,235336093246590572188273774826462666616_u128,268452393044079685568784388414095945103_u128,16352257315361819790006366624834143697_u128,221131562988565723591494267871651928453_u128,291733114214576323326190668786741080584_u128];
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).2 = !_16.1;
_34 = '\u{f0079}';
Goto(bb12)
}
bb12 = {
place!(Field::<(u16,)>(Variant(_7, 0), 4)).0 = _1.fld1;
place!(Field::<*const (u16,)>(Variant(_7, 0), 1)) = core::ptr::addr_of!(place!(Field::<(u16,)>(Variant(_7, 0), 4)));
place!(Field::<i32>(Variant(_6, 0), 0)) = (-864288002_i32);
_20.2 = (_20.1, _19, _29.2);
_36.0 = (_17, 6837822_u32, _16.1, Field::<bool>(Variant(_7, 0), 0));
place!(Field::<usize>(Variant(_7, 0), 3)) = _16.1 as usize;
match _36.0.1 {
0 => bb10,
6837822 => bb13,
_ => bb9
}
}
bb13 = {
_16.0 = core::ptr::addr_of!(_36.0.2);
place!(Field::<(u16,)>(Variant(_7, 0), 4)).0 = _12 as u16;
_23 = [313205803050612581438081172917858951685_u128,84212959095103628599888883140373264665_u128,265564226200451067921061395481169456426_u128,327682107974351111146788348127795240479_u128,303342098246220915216772515265165699977_u128,239847901107148229354158189138202343248_u128,24051059863177197171482010388963243691_u128,265807872042348990147110731190813745467_u128];
_35 = _20.2.1 | Field::<usize>(Variant(_7, 0), 3);
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).1 = _36.0.1;
_33 = (_32.0,);
_10 = [126318627911666198595886625044955223415_u128,89804285948990523676030426423498706089_u128,56782897568846863199215626066201996731_u128,216067048590505371108155733633820481762_u128,222656540073850618400345761066858172636_u128,1651398966309756043849224728211770252_u128,296155126577309738531786648085397514777_u128,338360286793712282664074465669277993825_u128];
match _36.0.1 {
0 => bb10,
1 => bb11,
2 => bb14,
6837822 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_20.2.2.2 = [_14,_18,_14];
_16.1 = !230_u8;
RET = [_18,_18,_18];
_20.2.1 = _20.0.0 as usize;
_15 = ((-7281093131539779044_i64),);
_2 = _1;
_3.fld0 = [_14,_18,_14];
_16.0 = core::ptr::addr_of!(_16.1);
_19 = !_20.2.1;
_1.fld2 = _3.fld2;
_20.0 = (_1.fld1,);
_2.fld5 = _3.fld5;
_16.2 = _16.0;
SetDiscriminant(_7, 0);
_17 = 126890929493767899290091220450307417301_u128 as f64;
_20.2.1 = _22 as usize;
_20.0.0 = !_2.fld1;
_20.1 = [_18];
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).0 = _17;
_29.2.2 = [_18,_18,_18];
_19 = !_20.2.1;
_2.fld2 = ['\u{5eca0}','\u{3186c}','\u{1c67e}','\u{793b3}','\u{13f40}','\u{166c}','\u{b172a}','\u{3514}'];
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).3 = _1.fld1 == _2.fld1;
_28 = _14;
Call(_12 = fn14(_2, _2.fld3, _17, _9, _2.fld3, _3, _3, _29.2.2, _29.2.2, _2.fld5, _1.fld0), ReturnTo(bb9), UnwindUnreachable())
}
bb16 = {
_33.0 = _20.0.0 << Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7).1;
SetDiscriminant(_6, 1);
place!(Field::<(f64, u32, u8, bool)>(Variant(_7, 0), 7)).1 = _36.0.1;
(*_26) = _33.0 + _29.2.1;
_2.fld0 = [Field::<bool>(Variant(_7, 0), 0),Field::<bool>(Variant(_7, 0), 0),_36.0.3];
_36.0.1 = _36.0.0 as u32;
_20.2.2.1 = _33.0 * _33.0;
_32.0 = !_20.2.2.1;
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(4_usize, 15_usize, Move(_15), 28_usize, Move(_28), 23_usize, Move(_23), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(4_usize, 14_usize, Move(_14), 13_usize, Move(_13), 20_usize, Move(_20), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(4_usize, 32_usize, Move(_32), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: Adt39,mut _2: *mut usize,mut _3: [bool; 3],mut _4: [bool; 3],mut _5: Adt39,mut _6: Adt39,mut _7: [char; 8]) -> [bool; 3] {
mir! {
type RET = [bool; 3];
let _8: Adt45;
let _9: Adt48;
let _10: ();
let _11: ();
{
_1.fld1 = _6.fld1;
_1.fld2 = ['\u{81b0b}','\u{1077e4}','\u{228c}','\u{b24f5}','\u{d0cac}','\u{401b8}','\u{54d3a}','\u{33ef0}'];
_1.fld0 = _5.fld0;
_7 = ['\u{514e5}','\u{515b}','\u{13d4d}','\u{deceb}','\u{77ba9}','\u{594f2}','\u{543c3}','\u{77d84}'];
_6.fld0 = _1.fld0;
_5 = _1;
_1 = _6;
_6 = _5;
_1.fld2 = _6.fld2;
Goto(bb1)
}
bb1 = {
_8.fld0.0 = !7772108461379596194_i64;
_1.fld3 = _5.fld3;
_6.fld5 = _2;
Goto(bb2)
}
bb2 = {
_9.fld3.0 = _5.fld1 & _6.fld1;
_9.fld4.fld5 = _6.fld5;
_5.fld5 = _1.fld5;
Call(_9.fld4 = fn6(_3, _6.fld5, _6, _6.fld3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.fld4.fld5 = _1.fld5;
_3 = [false,false,true];
_5.fld4 = _6.fld4;
_8.fld0.0 = !(-6601771028779277892_i64);
_1.fld1 = _9.fld3.0 | _9.fld4.fld1;
_5 = Adt39 { fld0: _4,fld1: _9.fld4.fld1,fld2: _9.fld4.fld2,fld3: _9.fld4.fld3,fld4: _9.fld4.fld4,fld5: _1.fld5 };
_8.fld1 = _2;
_9.fld2 = (_8.fld0.0,);
_9.fld4.fld2 = _1.fld2;
Goto(bb4)
}
bb4 = {
_4 = [false,false,false];
RET = _5.fld0;
_2 = _6.fld5;
_9.fld2 = _8.fld0;
_1.fld2 = _6.fld2;
_9.fld4.fld1 = !_1.fld1;
Goto(bb5)
}
bb5 = {
Call(_10 = dump_var(5_usize, 3_usize, Move(_3), 11_usize, _11, 11_usize, _11, 11_usize, _11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [bool; 3],mut _2: *mut usize,mut _3: Adt39,mut _4: i8) -> Adt39 {
mir! {
type RET = Adt39;
let _5: ([u32; 1], &'static isize, &'static isize, [bool; 1]);
let _6: [bool; 1];
let _7: i32;
let _8: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _9: [bool; 7];
let _10: ();
let _11: ();
{
RET.fld5 = _2;
RET = Adt39 { fld0: _1,fld1: _3.fld1,fld2: _3.fld2,fld3: _3.fld3,fld4: _3.fld4,fld5: _2 };
RET.fld0 = _3.fld0;
RET.fld3 = RET.fld1 as i8;
_5.3 = [false];
_5.0 = [122795292_u32];
RET.fld4 = core::ptr::addr_of_mut!(_3.fld1);
_3 = Adt39 { fld0: RET.fld0,fld1: RET.fld1,fld2: RET.fld2,fld3: RET.fld3,fld4: RET.fld4,fld5: _2 };
_3.fld3 = _4;
RET = Adt39 { fld0: _1,fld1: _3.fld1,fld2: _3.fld2,fld3: _3.fld3,fld4: _3.fld4,fld5: _3.fld5 };
RET.fld4 = _3.fld4;
_3.fld5 = RET.fld5;
_8.0 = (_3.fld1,);
_4 = RET.fld3;
_4 = !RET.fld3;
RET.fld4 = core::ptr::addr_of_mut!(_3.fld1);
_3 = RET;
_8.2.2.2 = [true,true,true];
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(6_usize, 4_usize, Move(_4), 11_usize, _11, 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i8,mut _2: [bool; 3],mut _3: Adt39,mut _4: Adt39,mut _5: [bool; 3],mut _6: [bool; 3],mut _7: i8,mut _8: Adt39,mut _9: Adt39,mut _10: Adt39,mut _11: [char; 8],mut _12: i8,mut _13: u16) -> *mut u16 {
mir! {
type RET = *mut u16;
let _14: f64;
let _15: i128;
let _16: *mut &'static isize;
let _17: isize;
let _18: u16;
let _19: Adt48;
let _20: (f64, u32, u8, bool);
let _21: u32;
let _22: u64;
let _23: i64;
let _24: Adt52;
let _25: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _26: [u32; 1];
let _27: (i64,);
let _28: f32;
let _29: (i64,);
let _30: Adt39;
let _31: Adt49;
let _32: i8;
let _33: Adt40;
let _34: u128;
let _35: Adt46;
let _36: char;
let _37: i128;
let _38: u32;
let _39: u128;
let _40: i8;
let _41: i32;
let _42: [u128; 8];
let _43: isize;
let _44: Adt55;
let _45: (u16,);
let _46: (i64,);
let _47: isize;
let _48: f32;
let _49: f32;
let _50: [bool; 1];
let _51: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _52: u64;
let _53: i16;
let _54: isize;
let _55: bool;
let _56: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _57: Adt49;
let _58: (i64,);
let _59: [bool; 7];
let _60: ();
let _61: ();
{
_10.fld1 = _4.fld1;
_14 = (-7379_i16) as f64;
_4.fld3 = !_8.fld3;
_10.fld1 = !_13;
_8 = _10;
_5 = _3.fld0;
_10.fld4 = core::ptr::addr_of_mut!(_4.fld1);
_10.fld5 = _8.fld5;
_3.fld1 = !_10.fld1;
_1 = -_8.fld3;
_11 = ['\u{276c7}','\u{bff8f}','\u{162a2}','\u{b6bb0}','\u{7c4a2}','\u{ecbdc}','\u{9edce}','\u{eb940}'];
_10.fld0 = _5;
_5 = [true,false,false];
_1 = _10.fld3;
_9.fld1 = _13 & _13;
_10.fld4 = core::ptr::addr_of_mut!(_8.fld1);
_10 = Adt39 { fld0: _3.fld0,fld1: _13,fld2: _11,fld3: _3.fld3,fld4: _3.fld4,fld5: _4.fld5 };
_14 = 80803393825194021_i64 as f64;
_10.fld0 = _3.fld0;
_4.fld1 = _10.fld1;
_10 = Adt39 { fld0: _6,fld1: _4.fld1,fld2: _9.fld2,fld3: _1,fld4: _8.fld4,fld5: _3.fld5 };
RET = core::ptr::addr_of_mut!(_13);
_9.fld1 = _8.fld1;
_10.fld2 = _3.fld2;
Goto(bb1)
}
bb1 = {
_19.fld2 = ((-1404230567380567751_i64),);
_6 = [true,true,false];
_12 = _8.fld3;
_17 = 37_u8 as isize;
_7 = _9.fld3;
_3.fld1 = !_4.fld1;
_10.fld2 = _9.fld2;
(*RET) = _10.fld1;
RET = _9.fld4;
_8.fld0 = _10.fld0;
(*RET) = !_4.fld1;
_4.fld5 = _10.fld5;
_4.fld0 = [false,true,false];
_18 = _3.fld1 - _3.fld1;
_4.fld0 = _8.fld0;
_20.2 = !32_u8;
_21 = !2665245779_u32;
RET = _4.fld4;
_9.fld0 = _4.fld0;
_19.fld4.fld1 = _18;
_20.0 = -_14;
_10.fld5 = _9.fld5;
_8 = Adt39 { fld0: _4.fld0,fld1: (*RET),fld2: _3.fld2,fld3: _3.fld3,fld4: _4.fld4,fld5: _9.fld5 };
_3.fld5 = _4.fld5;
_19.fld4.fld5 = _9.fld5;
_20.3 = !false;
Call(_4.fld3 = core::intrinsics::bswap(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.fld0 = [_20.3,_20.3,_20.3];
_19.fld4.fld2 = ['\u{689f9}','\u{71dad}','\u{e1884}','\u{bbbf9}','\u{ab361}','\u{10cb41}','\u{5c7f9}','\u{5ad71}'];
_17 = -(-109_isize);
_19.fld0 = [_20.3];
_13 = _19.fld4.fld1 + _19.fld4.fld1;
_19.fld3.0 = _10.fld1 << _19.fld2.0;
_19.fld4.fld3 = -_9.fld3;
_20 = (_14, _21, 198_u8, false);
_6 = _9.fld0;
_25.0 = (_19.fld4.fld1,);
_25.2.1 = (-45500154_i32) as usize;
_25.0.0 = _18;
_25.2.2.3 = 100127670377224182278377783821086522083_i128 & 107570184270050898815370828352006416075_i128;
_10 = _3;
(*RET) = _25.2.2.3 as u16;
_29 = _19.fld2;
_19.fld4 = Adt39 { fld0: _6,fld1: _19.fld3.0,fld2: _8.fld2,fld3: _1,fld4: _4.fld4,fld5: _8.fld5 };
_25.0.0 = !_19.fld4.fld1;
_19.fld4.fld3 = _25.2.2.3 as i8;
_3.fld1 = _13 - _25.0.0;
_19.fld0 = [_20.3];
_9.fld4 = core::ptr::addr_of_mut!(_30.fld1);
Call(_19.fld0 = fn8(_3, _3, _20.2, _4.fld0, _6, _8.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10.fld5 = _8.fld5;
_19.fld4.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_4.fld1 = _13 * _18;
_15 = !_25.2.2.3;
_25.0.0 = !(*RET);
_26 = [_20.1];
_30.fld5 = _10.fld5;
_10.fld5 = _3.fld5;
_9 = Adt39 { fld0: _3.fld0,fld1: _25.0.0,fld2: _3.fld2,fld3: _10.fld3,fld4: _3.fld4,fld5: _3.fld5 };
_25.2.2.1 = _25.0.0 + _25.0.0;
_4.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_10.fld1 = _25.2.2.1;
_27.0 = !_19.fld2.0;
_27 = _19.fld2;
_19.fld3 = (_10.fld1,);
_1 = _7;
_19.fld2 = _29;
_2 = _10.fld0;
_25.0.0 = _19.fld3.0 - _9.fld1;
_25.2.2 = (12002523601025157936_u64, (*RET), _9.fld0, _15);
_25.2.2.2 = [_20.3,_20.3,_20.3];
_30.fld0 = [_20.3,_20.3,_20.3];
_19.fld4 = _10;
match _20.2 {
0 => bb1,
1 => bb4,
198 => bb6,
_ => bb5
}
}
bb4 = {
_10.fld0 = [_20.3,_20.3,_20.3];
_19.fld4.fld2 = ['\u{689f9}','\u{71dad}','\u{e1884}','\u{bbbf9}','\u{ab361}','\u{10cb41}','\u{5c7f9}','\u{5ad71}'];
_17 = -(-109_isize);
_19.fld0 = [_20.3];
_13 = _19.fld4.fld1 + _19.fld4.fld1;
_19.fld3.0 = _10.fld1 << _19.fld2.0;
_19.fld4.fld3 = -_9.fld3;
_20 = (_14, _21, 198_u8, false);
_6 = _9.fld0;
_25.0 = (_19.fld4.fld1,);
_25.2.1 = (-45500154_i32) as usize;
_25.0.0 = _18;
_25.2.2.3 = 100127670377224182278377783821086522083_i128 & 107570184270050898815370828352006416075_i128;
_10 = _3;
(*RET) = _25.2.2.3 as u16;
_29 = _19.fld2;
_19.fld4 = Adt39 { fld0: _6,fld1: _19.fld3.0,fld2: _8.fld2,fld3: _1,fld4: _4.fld4,fld5: _8.fld5 };
_25.0.0 = !_19.fld4.fld1;
_19.fld4.fld3 = _25.2.2.3 as i8;
_3.fld1 = _13 - _25.0.0;
_19.fld0 = [_20.3];
_9.fld4 = core::ptr::addr_of_mut!(_30.fld1);
Call(_19.fld0 = fn8(_3, _3, _20.2, _4.fld0, _6, _8.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_19.fld2 = ((-1404230567380567751_i64),);
_6 = [true,true,false];
_12 = _8.fld3;
_17 = 37_u8 as isize;
_7 = _9.fld3;
_3.fld1 = !_4.fld1;
_10.fld2 = _9.fld2;
(*RET) = _10.fld1;
RET = _9.fld4;
_8.fld0 = _10.fld0;
(*RET) = !_4.fld1;
_4.fld5 = _10.fld5;
_4.fld0 = [false,true,false];
_18 = _3.fld1 - _3.fld1;
_4.fld0 = _8.fld0;
_20.2 = !32_u8;
_21 = !2665245779_u32;
RET = _4.fld4;
_9.fld0 = _4.fld0;
_19.fld4.fld1 = _18;
_20.0 = -_14;
_10.fld5 = _9.fld5;
_8 = Adt39 { fld0: _4.fld0,fld1: (*RET),fld2: _3.fld2,fld3: _3.fld3,fld4: _4.fld4,fld5: _9.fld5 };
_3.fld5 = _4.fld5;
_19.fld4.fld5 = _9.fld5;
_20.3 = !false;
Call(_4.fld3 = core::intrinsics::bswap(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_10.fld2 = ['\u{109183}','\u{9c895}','\u{ebc7f}','\u{68245}','\u{7f15b}','\u{829ea}','\u{9b429}','\u{7aadc}'];
_38 = _21 - _20.1;
_27 = _19.fld2;
_25.2.2 = (7662800870307163831_u64, _10.fld1, _19.fld4.fld0, _15);
_19.fld4.fld2 = ['\u{a6d68}','\u{8808f}','\u{ae6d9}','\u{e3092}','\u{22a90}','\u{271b2}','\u{a23bf}','\u{d2f5c}'];
_25.1 = _19.fld0;
_38 = _20.1 << _25.0.0;
_13 = _19.fld4.fld1 ^ _19.fld4.fld1;
_20.2 = _29.0 as u8;
Call(_29.0 = core::intrinsics::transmute(_25.2.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = _6;
_23 = !_29.0;
_25.2.2.0 = !18019279599797580751_u64;
_25.0.0 = _25.2.2.1 ^ _19.fld4.fld1;
_25.2.2.0 = 15980270241161173489_u64 + 3690322993733559033_u64;
_19.fld0 = [_20.3];
_25.2.2.2 = _19.fld4.fld0;
_30.fld0 = _8.fld0;
_4.fld4 = RET;
_41 = !760968004_i32;
_19.fld4.fld3 = _1 & _12;
_4.fld0 = [_20.3,_20.3,_20.3];
_30.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_10.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_32 = _12 ^ _12;
_25.2.2.1 = _13;
_30.fld3 = _3.fld3 ^ _1;
_35 = Adt46::Variant0 { fld0: 65113648667372093998308925616787656868_u128,fld1: _25.2.1,fld2: _38,fld3: _9,fld4: _2,fld5: _25.2.2.0,fld6: _20.0 };
_33 = Adt40::Variant0 { fld0: _41,fld1: _15,fld2: _3.fld5 };
place!(Field::<Adt39>(Variant(_35, 0), 3)).fld1 = Field::<u64>(Variant(_35, 0), 5) as u16;
_4.fld0 = [_20.3,_20.3,_20.3];
_35 = Adt46::Variant0 { fld0: 195131935361003097013997455475488939218_u128,fld1: _25.2.1,fld2: _38,fld3: _3,fld4: _30.fld0,fld5: _25.2.2.0,fld6: _14 };
_13 = _17 as u16;
Goto(bb8)
}
bb8 = {
place!(Field::<u64>(Variant(_35, 0), 5)) = !_25.2.2.0;
_3.fld0 = [_20.3,_20.3,_20.3];
_9.fld1 = _25.0.0 << _25.0.0;
place!(Field::<*mut usize>(Variant(_33, 0), 2)) = core::ptr::addr_of_mut!(_25.2.1);
_19.fld4.fld5 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_35, 0), 1)));
_29.0 = _23 & _23;
_10.fld2 = ['\u{f1b3}','\u{bde5c}','\u{d285c}','\u{9e1eb}','\u{d0a60}','\u{8b52e}','\u{d80b8}','\u{49a71}'];
_1 = _32 >> _19.fld4.fld3;
_4.fld5 = _10.fld5;
_25.1 = [_20.3];
_37 = 26397_i16 as i128;
_25.2.1 = !Field::<usize>(Variant(_35, 0), 1);
_41 = Field::<i32>(Variant(_33, 0), 0) - Field::<i32>(Variant(_33, 0), 0);
_44 = Adt55::Variant1 { fld0: _41,fld1: 6931_i16 };
place!(Field::<i16>(Variant(_44, 1), 1)) = (-215_i16);
_19.fld4.fld0 = [_20.3,_20.3,_20.3];
_9.fld1 = _20.2 as u16;
_4.fld3 = _9.fld3;
_30.fld3 = -_1;
_30.fld1 = _19.fld3.0;
_8.fld2 = _3.fld2;
_9.fld3 = !_32;
_30.fld4 = RET;
_47 = _17;
_25.2.2.1 = _25.0.0 ^ _25.0.0;
_29.0 = -_23;
_4.fld1 = !_25.0.0;
SetDiscriminant(_33, 1);
_19.fld4.fld4 = core::ptr::addr_of_mut!(_30.fld1);
match _19.fld2.0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
340282366920938463461970376864387643705 => bb10,
_ => bb9
}
}
bb9 = {
_10.fld2 = ['\u{109183}','\u{9c895}','\u{ebc7f}','\u{68245}','\u{7f15b}','\u{829ea}','\u{9b429}','\u{7aadc}'];
_38 = _21 - _20.1;
_27 = _19.fld2;
_25.2.2 = (7662800870307163831_u64, _10.fld1, _19.fld4.fld0, _15);
_19.fld4.fld2 = ['\u{a6d68}','\u{8808f}','\u{ae6d9}','\u{e3092}','\u{22a90}','\u{271b2}','\u{a23bf}','\u{d2f5c}'];
_25.1 = _19.fld0;
_38 = _20.1 << _25.0.0;
_13 = _19.fld4.fld1 ^ _19.fld4.fld1;
_20.2 = _29.0 as u8;
Call(_29.0 = core::intrinsics::transmute(_25.2.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb10 = {
Goto(bb11)
}
bb11 = {
_9.fld2 = ['\u{53c26}','\u{71903}','\u{5d50c}','\u{7da53}','\u{645b4}','\u{e85cd}','\u{627dd}','\u{69ebe}'];
_28 = Field::<usize>(Variant(_35, 0), 1) as f32;
_20 = (_14, _38, 104_u8, false);
place!(Field::<u128>(Variant(_35, 0), 0)) = _20.2 as u128;
_30 = Adt39 { fld0: Field::<[bool; 3]>(Variant(_35, 0), 4),fld1: _19.fld4.fld1,fld2: Field::<Adt39>(Variant(_35, 0), 3).fld2,fld3: _1,fld4: RET,fld5: _19.fld4.fld5 };
_30 = Adt39 { fld0: _9.fld0,fld1: Field::<Adt39>(Variant(_35, 0), 3).fld1,fld2: _8.fld2,fld3: _19.fld4.fld3,fld4: _10.fld4,fld5: _10.fld5 };
_19.fld4.fld3 = _9.fld3;
SetDiscriminant(_35, 1);
place!(Field::<i8>(Variant(_35, 1), 3)) = _7 >> _19.fld3.0;
_47 = _17;
_43 = !_47;
_10.fld1 = _4.fld1 >> (*RET);
_8.fld3 = _1 ^ Field::<i8>(Variant(_35, 1), 3);
_51.0 = _20;
_51.1 = _25.0;
_10.fld2 = _8.fld2;
_34 = 144028803524134473910662136676199639692_u128 | 309779635227996040493547182885717317478_u128;
_25.2.0 = [_51.0.3];
_51.2 = ['\u{7d9b7}','\u{adaeb}','\u{c43a2}','\u{c0890}','\u{b1fd6}','\u{d50d6}','\u{6577a}','\u{21f78}'];
_36 = '\u{8498e}';
_21 = _51.0.1 * _51.0.1;
match _51.0.2 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
104 => bb19,
_ => bb18
}
}
bb12 = {
_10.fld5 = _8.fld5;
_19.fld4.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_4.fld1 = _13 * _18;
_15 = !_25.2.2.3;
_25.0.0 = !(*RET);
_26 = [_20.1];
_30.fld5 = _10.fld5;
_10.fld5 = _3.fld5;
_9 = Adt39 { fld0: _3.fld0,fld1: _25.0.0,fld2: _3.fld2,fld3: _10.fld3,fld4: _3.fld4,fld5: _3.fld5 };
_25.2.2.1 = _25.0.0 + _25.0.0;
_4.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_10.fld1 = _25.2.2.1;
_27.0 = !_19.fld2.0;
_27 = _19.fld2;
_19.fld3 = (_10.fld1,);
_1 = _7;
_19.fld2 = _29;
_2 = _10.fld0;
_25.0.0 = _19.fld3.0 - _9.fld1;
_25.2.2 = (12002523601025157936_u64, (*RET), _9.fld0, _15);
_25.2.2.2 = [_20.3,_20.3,_20.3];
_30.fld0 = [_20.3,_20.3,_20.3];
_19.fld4 = _10;
match _20.2 {
0 => bb1,
1 => bb4,
198 => bb6,
_ => bb5
}
}
bb13 = {
_10.fld2 = ['\u{109183}','\u{9c895}','\u{ebc7f}','\u{68245}','\u{7f15b}','\u{829ea}','\u{9b429}','\u{7aadc}'];
_38 = _21 - _20.1;
_27 = _19.fld2;
_25.2.2 = (7662800870307163831_u64, _10.fld1, _19.fld4.fld0, _15);
_19.fld4.fld2 = ['\u{a6d68}','\u{8808f}','\u{ae6d9}','\u{e3092}','\u{22a90}','\u{271b2}','\u{a23bf}','\u{d2f5c}'];
_25.1 = _19.fld0;
_38 = _20.1 << _25.0.0;
_13 = _19.fld4.fld1 ^ _19.fld4.fld1;
_20.2 = _29.0 as u8;
Call(_29.0 = core::intrinsics::transmute(_25.2.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb14 = {
_19.fld2 = ((-1404230567380567751_i64),);
_6 = [true,true,false];
_12 = _8.fld3;
_17 = 37_u8 as isize;
_7 = _9.fld3;
_3.fld1 = !_4.fld1;
_10.fld2 = _9.fld2;
(*RET) = _10.fld1;
RET = _9.fld4;
_8.fld0 = _10.fld0;
(*RET) = !_4.fld1;
_4.fld5 = _10.fld5;
_4.fld0 = [false,true,false];
_18 = _3.fld1 - _3.fld1;
_4.fld0 = _8.fld0;
_20.2 = !32_u8;
_21 = !2665245779_u32;
RET = _4.fld4;
_9.fld0 = _4.fld0;
_19.fld4.fld1 = _18;
_20.0 = -_14;
_10.fld5 = _9.fld5;
_8 = Adt39 { fld0: _4.fld0,fld1: (*RET),fld2: _3.fld2,fld3: _3.fld3,fld4: _4.fld4,fld5: _9.fld5 };
_3.fld5 = _4.fld5;
_19.fld4.fld5 = _9.fld5;
_20.3 = !false;
Call(_4.fld3 = core::intrinsics::bswap(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_2 = _6;
_23 = !_29.0;
_25.2.2.0 = !18019279599797580751_u64;
_25.0.0 = _25.2.2.1 ^ _19.fld4.fld1;
_25.2.2.0 = 15980270241161173489_u64 + 3690322993733559033_u64;
_19.fld0 = [_20.3];
_25.2.2.2 = _19.fld4.fld0;
_30.fld0 = _8.fld0;
_4.fld4 = RET;
_41 = !760968004_i32;
_19.fld4.fld3 = _1 & _12;
_4.fld0 = [_20.3,_20.3,_20.3];
_30.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_10.fld5 = core::ptr::addr_of_mut!(_25.2.1);
_32 = _12 ^ _12;
_25.2.2.1 = _13;
_30.fld3 = _3.fld3 ^ _1;
_35 = Adt46::Variant0 { fld0: 65113648667372093998308925616787656868_u128,fld1: _25.2.1,fld2: _38,fld3: _9,fld4: _2,fld5: _25.2.2.0,fld6: _20.0 };
_33 = Adt40::Variant0 { fld0: _41,fld1: _15,fld2: _3.fld5 };
place!(Field::<Adt39>(Variant(_35, 0), 3)).fld1 = Field::<u64>(Variant(_35, 0), 5) as u16;
_4.fld0 = [_20.3,_20.3,_20.3];
_35 = Adt46::Variant0 { fld0: 195131935361003097013997455475488939218_u128,fld1: _25.2.1,fld2: _38,fld3: _3,fld4: _30.fld0,fld5: _25.2.2.0,fld6: _14 };
_13 = _17 as u16;
Goto(bb8)
}
bb16 = {
_10.fld2 = ['\u{109183}','\u{9c895}','\u{ebc7f}','\u{68245}','\u{7f15b}','\u{829ea}','\u{9b429}','\u{7aadc}'];
_38 = _21 - _20.1;
_27 = _19.fld2;
_25.2.2 = (7662800870307163831_u64, _10.fld1, _19.fld4.fld0, _15);
_19.fld4.fld2 = ['\u{a6d68}','\u{8808f}','\u{ae6d9}','\u{e3092}','\u{22a90}','\u{271b2}','\u{a23bf}','\u{d2f5c}'];
_25.1 = _19.fld0;
_38 = _20.1 << _25.0.0;
_13 = _19.fld4.fld1 ^ _19.fld4.fld1;
_20.2 = _29.0 as u8;
Call(_29.0 = core::intrinsics::transmute(_25.2.2.0), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
_10.fld0 = [_20.3,_20.3,_20.3];
_19.fld4.fld2 = ['\u{689f9}','\u{71dad}','\u{e1884}','\u{bbbf9}','\u{ab361}','\u{10cb41}','\u{5c7f9}','\u{5ad71}'];
_17 = -(-109_isize);
_19.fld0 = [_20.3];
_13 = _19.fld4.fld1 + _19.fld4.fld1;
_19.fld3.0 = _10.fld1 << _19.fld2.0;
_19.fld4.fld3 = -_9.fld3;
_20 = (_14, _21, 198_u8, false);
_6 = _9.fld0;
_25.0 = (_19.fld4.fld1,);
_25.2.1 = (-45500154_i32) as usize;
_25.0.0 = _18;
_25.2.2.3 = 100127670377224182278377783821086522083_i128 & 107570184270050898815370828352006416075_i128;
_10 = _3;
(*RET) = _25.2.2.3 as u16;
_29 = _19.fld2;
_19.fld4 = Adt39 { fld0: _6,fld1: _19.fld3.0,fld2: _8.fld2,fld3: _1,fld4: _4.fld4,fld5: _8.fld5 };
_25.0.0 = !_19.fld4.fld1;
_19.fld4.fld3 = _25.2.2.3 as i8;
_3.fld1 = _13 - _25.0.0;
_19.fld0 = [_20.3];
_9.fld4 = core::ptr::addr_of_mut!(_30.fld1);
Call(_19.fld0 = fn8(_3, _3, _20.2, _4.fld0, _6, _8.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb18 = {
_10.fld0 = [_20.3,_20.3,_20.3];
_19.fld4.fld2 = ['\u{689f9}','\u{71dad}','\u{e1884}','\u{bbbf9}','\u{ab361}','\u{10cb41}','\u{5c7f9}','\u{5ad71}'];
_17 = -(-109_isize);
_19.fld0 = [_20.3];
_13 = _19.fld4.fld1 + _19.fld4.fld1;
_19.fld3.0 = _10.fld1 << _19.fld2.0;
_19.fld4.fld3 = -_9.fld3;
_20 = (_14, _21, 198_u8, false);
_6 = _9.fld0;
_25.0 = (_19.fld4.fld1,);
_25.2.1 = (-45500154_i32) as usize;
_25.0.0 = _18;
_25.2.2.3 = 100127670377224182278377783821086522083_i128 & 107570184270050898815370828352006416075_i128;
_10 = _3;
(*RET) = _25.2.2.3 as u16;
_29 = _19.fld2;
_19.fld4 = Adt39 { fld0: _6,fld1: _19.fld3.0,fld2: _8.fld2,fld3: _1,fld4: _4.fld4,fld5: _8.fld5 };
_25.0.0 = !_19.fld4.fld1;
_19.fld4.fld3 = _25.2.2.3 as i8;
_3.fld1 = _13 - _25.0.0;
_19.fld0 = [_20.3];
_9.fld4 = core::ptr::addr_of_mut!(_30.fld1);
Call(_19.fld0 = fn8(_3, _3, _20.2, _4.fld0, _6, _8.fld4), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_42 = [_34,_34,_34,_34,_34,_34,_34,_34];
_10.fld0 = [_51.0.3,_51.0.3,_20.3];
_30.fld5 = _19.fld4.fld5;
_22 = _25.2.2.0 + _25.2.2.0;
_56.0.3 = !_51.0.3;
_51.0.3 = _23 <= _29.0;
_32 = Field::<i8>(Variant(_35, 1), 3) | _8.fld3;
(*RET) = !_4.fld1;
_33 = Adt40::Variant1 { fld0: _36 };
place!(Field::<(u64, u16, [bool; 3], i128)>(Variant(_35, 1), 4)).2 = [_20.3,_56.0.3,_20.3];
_25.0.0 = (*RET) << _38;
_51.2 = [_36,_36,Field::<char>(Variant(_33, 1), 0),Field::<char>(Variant(_33, 1), 0),_36,Field::<char>(Variant(_33, 1), 0),_36,_36];
_20.2 = _19.fld3.0 as u8;
_9.fld2 = _30.fld2;
_25.2.2 = (_22, (*RET), _10.fld0, _37);
_25.1 = [_51.0.3];
_3.fld1 = _25.2.2.1 * (*RET);
_8.fld0 = [_51.0.3,_51.0.3,_20.3];
_52 = _25.2.2.0 ^ _22;
Goto(bb20)
}
bb20 = {
Call(_60 = dump_var(7_usize, 21_usize, Move(_21), 37_usize, Move(_37), 1_usize, Move(_1), 32_usize, Move(_32)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_60 = dump_var(7_usize, 34_usize, Move(_34), 12_usize, Move(_12), 42_usize, Move(_42), 41_usize, Move(_41)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_60 = dump_var(7_usize, 22_usize, Move(_22), 11_usize, Move(_11), 7_usize, Move(_7), 23_usize, Move(_23)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_60 = dump_var(7_usize, 25_usize, Move(_25), 47_usize, Move(_47), 61_usize, _61, 61_usize, _61), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: Adt39,mut _2: Adt39,mut _3: u8,mut _4: [bool; 3],mut _5: [bool; 3],mut _6: *mut u16) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _7: char;
let _8: [char; 8];
let _9: Adt50;
let _10: u8;
let _11: f32;
let _12: (f64, u32, u8, bool);
let _13: u128;
let _14: ([u32; 1], &'static isize, &'static isize, [bool; 1]);
let _15: (u64, u16, [bool; 3], i128);
let _16: i128;
let _17: f32;
let _18: Adt41;
let _19: f64;
let _20: Adt47;
let _21: isize;
let _22: f32;
let _23: Adt48;
let _24: bool;
let _25: [char; 8];
let _26: ();
let _27: ();
{
_1.fld0 = [false,true,false];
_1.fld2 = ['\u{a012a}','\u{10b890}','\u{877fd}','\u{103839}','\u{ba182}','\u{c0595}','\u{10ffa1}','\u{c76c7}'];
_1.fld2 = ['\u{e6966}','\u{3c2ef}','\u{2c7cf}','\u{cb0e7}','\u{2d19d}','\u{33bda}','\u{4b1a9}','\u{eaee7}'];
(*_6) = _2.fld1;
Goto(bb1)
}
bb1 = {
_6 = _1.fld4;
_2.fld3 = (*_6) as i8;
_2 = Adt39 { fld0: _4,fld1: _1.fld1,fld2: _1.fld2,fld3: _1.fld3,fld4: _1.fld4,fld5: _1.fld5 };
RET = [false];
_2.fld5 = _1.fld5;
_1 = Adt39 { fld0: _4,fld1: (*_6),fld2: _2.fld2,fld3: _2.fld3,fld4: _2.fld4,fld5: _2.fld5 };
_2.fld2 = ['\u{d9523}','\u{7fab}','\u{ee9da}','\u{e4c5}','\u{8adc2}','\u{8ec21}','\u{cf24a}','\u{5f8b4}'];
RET = [true];
_2.fld0 = [false,true,true];
_1.fld1 = !(*_6);
RET = [true];
_7 = '\u{7e52f}';
_1.fld1 = 7971313548873068398_u64 as u16;
_1.fld0 = _4;
_2.fld1 = (*_6) >> (*_6);
Goto(bb2)
}
bb2 = {
_2.fld0 = _4;
_1.fld4 = _6;
_4 = [true,false,true];
_1.fld4 = core::ptr::addr_of_mut!(_2.fld1);
_2.fld0 = [false,true,true];
_1.fld1 = (*_6) + (*_6);
_2.fld3 = _1.fld3 | _1.fld3;
_2.fld1 = 30444_i16 as u16;
_1.fld1 = (*_6) ^ (*_6);
_2.fld3 = _1.fld3;
_2.fld5 = _1.fld5;
_2.fld1 = _1.fld1;
_2.fld1 = _1.fld1;
_1.fld3 = _2.fld3 * _2.fld3;
_10 = _3;
_1.fld1 = (-19396_i16) as u16;
_1.fld2 = [_7,_7,_7,_7,_7,_7,_7,_7];
_11 = 476082747_u32 as f32;
_1.fld2 = _2.fld2;
_8 = [_7,_7,_7,_7,_7,_7,_7,_7];
_1.fld3 = _2.fld3 | _2.fld3;
_1.fld0 = [true,true,true];
_8 = [_7,_7,_7,_7,_7,_7,_7,_7];
_2.fld0 = [false,true,true];
_12.2 = (-1861066319_i32) as u8;
_2.fld1 = (*_6) ^ _1.fld1;
_11 = _2.fld3 as f32;
Call(_12 = fn9((*_6), _2.fld2, _4, _1.fld2, _1, _2.fld4, (*_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_6) = _12.1 as u16;
_2.fld4 = _6;
_14.3 = RET;
_1.fld0 = _5;
_1.fld1 = _12.0 as u16;
_14.3 = RET;
_12.0 = _2.fld3 as f64;
_3 = _10;
_5 = [_12.3,_12.3,_12.3];
_1.fld1 = _2.fld1;
_10 = _12.2;
(*_6) = _2.fld1 << _1.fld1;
_15.2 = [_12.3,_12.3,_12.3];
_1.fld3 = -_2.fld3;
_14.0 = [_12.1];
_1.fld3 = !_2.fld3;
_4 = [_12.3,_12.3,_12.3];
_13 = 55761421264953690677405060222372516808_i128 as u128;
_15.3 = -113160575724160592921116154610145482769_i128;
_15 = (6141119304048980920_u64, _1.fld1, _5, (-158293549268976709348294579342374277553_i128));
_2.fld0 = _5;
_2.fld0 = _15.2;
_15.1 = _12.1 as u16;
_2 = Adt39 { fld0: _15.2,fld1: (*_6),fld2: _8,fld3: _1.fld3,fld4: _6,fld5: _1.fld5 };
_1.fld0 = [_12.3,_12.3,_12.3];
Goto(bb4)
}
bb4 = {
_14.3 = [_12.3];
_15.0 = 15123254211715381983_u64;
_2.fld5 = _1.fld5;
_5 = [_12.3,_12.3,_12.3];
_20.fld1.fld0.0 = (-830410328249100624_i64);
_2.fld1 = _15.1 - (*_6);
_16 = -_15.3;
_20.fld1.fld3 = _1.fld3;
_15.3 = _16;
_20.fld0 = _20.fld1.fld0.0;
Call(_20.fld0 = fn13(_1.fld1, _16, _1.fld0, _1.fld4, _15.2, _15.3, _16, _2, _12.3, _15, _1.fld0, _15, _1, _4, _5, _15.2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_6) = !_2.fld1;
RET = [_12.3];
_12.1 = !955495807_u32;
_1.fld2 = [_7,_7,_7,_7,_7,_7,_7,_7];
_4 = [_12.3,_12.3,_12.3];
_1.fld4 = _2.fld4;
_24 = _12.3;
_23.fld1 = (-1026069002_i32) ^ 1430684055_i32;
_23.fld2.0 = _20.fld1.fld0.0;
Goto(bb6)
}
bb6 = {
Call(_26 = dump_var(8_usize, 15_usize, Move(_15), 4_usize, Move(_4), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_26 = dump_var(8_usize, 13_usize, Move(_13), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u16,mut _2: [char; 8],mut _3: [bool; 3],mut _4: [char; 8],mut _5: Adt39,mut _6: *mut u16,mut _7: u16) -> (f64, u32, u8, bool) {
mir! {
type RET = (f64, u32, u8, bool);
let _8: u32;
let _9: i128;
let _10: Adt39;
let _11: u128;
let _12: Adt47;
let _13: Adt49;
let _14: f64;
let _15: isize;
let _16: Adt50;
let _17: [bool; 3];
let _18: Adt45;
let _19: u8;
let _20: [bool; 7];
let _21: i16;
let _22: ();
let _23: ();
{
_2 = _5.fld2;
_2 = ['\u{70057}','\u{1b409}','\u{1995a}','\u{e4b0e}','\u{fdd5}','\u{edc84}','\u{49ca7}','\u{957d2}'];
_5.fld4 = core::ptr::addr_of_mut!(_7);
(*_6) = !_1;
RET.1 = 152832752591427294259699330577720602226_i128 as u32;
RET.0 = 1715698491774507251_u64 as f64;
RET.3 = (*_6) != (*_6);
RET.3 = true;
(*_6) = _7;
_10.fld5 = _5.fld5;
_5.fld1 = (*_6) + (*_6);
RET.1 = _5.fld3 as u32;
_5.fld0 = [RET.3,RET.3,RET.3];
RET.2 = !84_u8;
_11 = 24723_i16 as u128;
_8 = RET.1 & RET.1;
(*_6) = !_1;
RET.2 = _5.fld3 as u8;
_10.fld1 = _5.fld1 >> _7;
Goto(bb1)
}
bb1 = {
_4 = ['\u{eed9b}','\u{aa16}','\u{bcc3}','\u{dbf9b}','\u{cd61a}','\u{5260a}','\u{704bd}','\u{8bcba}'];
_10.fld3 = _5.fld3 - _5.fld3;
_12.fld1.fld3 = _5.fld3 * _5.fld3;
_10.fld4 = core::ptr::addr_of_mut!((*_6));
_5.fld3 = -_12.fld1.fld3;
_5.fld1 = _11 as u16;
_4 = ['\u{d90b7}','\u{56b0c}','\u{59b5a}','\u{453d8}','\u{4f402}','\u{c1e60}','\u{10b3ca}','\u{bdf6b}'];
_10.fld0 = [RET.3,RET.3,RET.3];
Call(_5 = fn10(_10.fld1, _6, _4, _1, _10.fld1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.fld5 = _10.fld5;
RET.0 = 9223372036854775807_isize as f64;
_12.fld1.fld0 = (4199728386743106970_i64,);
_12.fld1.fld0 = ((-2581158489343365046_i64),);
_10.fld2 = _5.fld2;
_10.fld2 = ['\u{2855}','\u{b08c0}','\u{75f16}','\u{10765a}','\u{fc01d}','\u{ff393}','\u{b4b14}','\u{66878}'];
_7 = _5.fld1;
_5.fld4 = core::ptr::addr_of_mut!(_5.fld1);
_9 = -143494475532551767930374215880248938465_i128;
(*_6) = _5.fld1;
_12.fld1.fld0.0 = 7_isize as i64;
_10.fld5 = _5.fld5;
_10.fld4 = core::ptr::addr_of_mut!(_10.fld1);
_12.fld1.fld1 = _5.fld5;
RET.2 = 80_u8;
_12.fld1.fld3 = _10.fld3;
RET.3 = _8 <= RET.1;
_10.fld2 = ['\u{28cbc}','\u{4497f}','\u{72f42}','\u{3c537}','\u{f41d2}','\u{fbe1c}','\u{b37c3}','\u{f86d3}'];
RET.3 = !false;
_10.fld2 = _4;
_15 = (-9223372036854775808_isize) & (-83_isize);
_8 = _11 as u32;
RET.1 = 6932565289952588022_u64 as u32;
_12.fld1.fld0 = (728386722656358121_i64,);
_5.fld1 = (*_6);
Call(RET.3 = fn12(_7, _10.fld4, _5.fld4, _5.fld4, _7, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5.fld5 = _10.fld5;
RET.2 = !240_u8;
_15 = (-9223372036854775808_isize);
_4 = _2;
_18.fld1 = _10.fld5;
_10 = _5;
_18.fld2 = Adt44::Variant1 { fld0: 357905218517347158_u64 };
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(9_usize, 9_usize, Move(_9), 7_usize, Move(_7), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u16,mut _2: *mut u16,mut _3: [char; 8],mut _4: u16,mut _5: u16) -> Adt39 {
mir! {
type RET = Adt39;
let _6: [u32; 1];
let _7: isize;
let _8: *mut usize;
let _9: *mut usize;
let _10: char;
let _11: [bool; 7];
let _12: Adt47;
let _13: u8;
let _14: f64;
let _15: (u16,);
let _16: isize;
let _17: char;
let _18: char;
let _19: [bool; 7];
let _20: Adt52;
let _21: i32;
let _22: [bool; 1];
let _23: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _24: u64;
let _25: [bool; 7];
let _26: isize;
let _27: i16;
let _28: ();
let _29: ();
{
_2 = core::ptr::addr_of_mut!(RET.fld1);
RET.fld0 = [false,false,true];
RET.fld1 = _5 ^ _5;
RET.fld4 = core::ptr::addr_of_mut!(RET.fld1);
RET.fld0 = [false,false,false];
RET.fld3 = (-21_i8) >> (*_2);
_3 = ['\u{e98b3}','\u{69ca8}','\u{4442d}','\u{106d1f}','\u{3b80a}','\u{93e34}','\u{ec7e3}','\u{bdd6d}'];
RET.fld3 = !46_i8;
_1 = RET.fld1 * (*_2);
Goto(bb1)
}
bb1 = {
_2 = core::ptr::addr_of_mut!((*_2));
_4 = !(*_2);
RET.fld1 = _1;
_7 = !(-9223372036854775808_isize);
RET.fld3 = -(-21_i8);
RET.fld3 = (-2_i8);
match RET.fld3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211454 => bb9,
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
RET.fld2 = _3;
RET.fld0 = [true,false,true];
RET.fld1 = _1;
RET.fld3 = !85_i8;
Call(_5 = fn11((*_2), RET.fld4, (*_2), _4, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_11 = [true,true,false,true,true,false,true];
RET.fld0 = [false,false,false];
(*_2) = !_1;
Goto(bb11)
}
bb11 = {
RET.fld0 = [false,true,true];
_12.fld1.fld0 = ((-7843368899493051111_i64),);
_12.fld1.fld0.0 = 4697681273685177003_i64;
RET.fld1 = _1 >> _1;
RET.fld3 = (-116_i8) + (-49_i8);
_12.fld0 = _12.fld1.fld0.0 - _12.fld1.fld0.0;
RET.fld3 = 69_u8 as i8;
_5 = 7_usize as u16;
_3 = ['\u{a6c76}','\u{213e4}','\u{a5149}','\u{2d0e4}','\u{b6fe5}','\u{25ee6}','\u{9c1e6}','\u{610a8}'];
(*_2) = !_1;
_12.fld0 = _12.fld1.fld0.0 * _12.fld1.fld0.0;
RET.fld4 = _2;
RET.fld2 = ['\u{4d887}','\u{b606e}','\u{e7219}','\u{65754}','\u{4b77f}','\u{6f59b}','\u{e6efb}','\u{89b48}'];
_2 = core::ptr::addr_of_mut!(_1);
_11 = [true,false,true,true,true,true,false];
RET.fld2 = ['\u{1c86f}','\u{b5ef7}','\u{95a05}','\u{18707}','\u{29929}','\u{30d08}','\u{3669b}','\u{8ae55}'];
_2 = core::ptr::addr_of_mut!(_4);
_13 = 533725939_i32 as u8;
_12.fld0 = (-1171150709_i32) as i64;
_14 = RET.fld1 as f64;
_12.fld0 = -_12.fld1.fld0.0;
RET.fld1 = _4;
_14 = 23438_i16 as f64;
_14 = RET.fld3 as f64;
RET.fld0 = [false,true,false];
_3 = ['\u{5423d}','\u{8fcea}','\u{7ce99}','\u{f9b34}','\u{50b27}','\u{feaab}','\u{32045}','\u{b0f5c}'];
RET.fld3 = -(-88_i8);
match _12.fld1.fld0.0 {
0 => bb1,
1 => bb2,
4697681273685177003 => bb12,
_ => bb9
}
}
bb12 = {
_15 = (RET.fld1,);
RET.fld2 = _3;
_12.fld1.fld3 = RET.fld3;
_14 = (-2037517657_i32) as f64;
RET.fld1 = !_1;
RET.fld0 = [false,true,true];
_2 = RET.fld4;
_6 = [3098252335_u32];
_12.fld1.fld3 = -RET.fld3;
RET.fld0 = [true,false,false];
RET.fld4 = _2;
RET.fld4 = core::ptr::addr_of_mut!(_5);
_12.fld1.fld0.0 = !_12.fld0;
_6 = [3322337670_u32];
_15.0 = (*_2);
_12.fld0 = RET.fld3 as i64;
RET.fld2 = ['\u{eca0d}','\u{558a3}','\u{4940}','\u{f14c5}','\u{8d049}','\u{7be7}','\u{801ca}','\u{92447}'];
_12.fld1.fld3 = -RET.fld3;
_3 = ['\u{64f68}','\u{f0257}','\u{62f5e}','\u{9d4bc}','\u{ce5e}','\u{4130}','\u{f4b51}','\u{ae417}'];
_10 = '\u{43102}';
RET.fld3 = _12.fld1.fld3;
_16 = 92932181493797336842694919015095065242_u128 as isize;
Goto(bb13)
}
bb13 = {
_12.fld1.fld2 = Adt44::Variant1 { fld0: 16991785327207507446_u64 };
_15 = (_4,);
_10 = '\u{6f629}';
_1 = (*_2) + (*_2);
place!(Field::<u64>(Variant(_12.fld1.fld2, 1), 0)) = !10095278360090605258_u64;
RET.fld3 = _12.fld1.fld3 + _12.fld1.fld3;
Goto(bb14)
}
bb14 = {
_12.fld0 = _12.fld1.fld0.0;
_4 = (*_2);
place!(Field::<u64>(Variant(_12.fld1.fld2, 1), 0)) = 8437819864266932332_u64 & 1240553039651501619_u64;
_14 = _16 as f64;
_12.fld1.fld0.0 = _14 as i64;
_4 = _1;
_18 = _10;
(*_2) = !_1;
RET.fld2 = [_10,_18,_10,_18,_18,_18,_18,_18];
RET.fld1 = _15.0;
_11 = [true,false,true,false,false,true,true];
_5 = _15.0 * (*_2);
_18 = _10;
SetDiscriminant(_12.fld1.fld2, 0);
_12.fld1.fld1 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_12.fld1.fld2, 0), 3)));
_9 = core::ptr::addr_of_mut!(place!(Field::<usize>(Variant(_12.fld1.fld2, 0), 3)));
_14 = (-8653_i16) as f64;
place!(Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7)).0 = _14;
(*_9) = (-24455_i16) as usize;
_13 = RET.fld3 as u8;
_4 = (*_2) * RET.fld1;
_15 = (_4,);
place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)) = Adt43::Variant2 { fld0: _12.fld1.fld0.0 };
_17 = _10;
(*_9) = _14 as usize;
RET.fld1 = !_15.0;
_8 = _9;
RET.fld3 = _12.fld1.fld3;
Goto(bb15)
}
bb15 = {
SetDiscriminant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 2);
place!(Field::<(u16,)>(Variant(_12.fld1.fld2, 0), 4)) = ((*_2),);
_21 = 1830671789_i32;
_19 = [false,false,true,true,false,true,true];
place!(Field::<i64>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 2), 0)) = _12.fld1.fld0.0 | _12.fld1.fld0.0;
place!(Field::<*const (u16,)>(Variant(_12.fld1.fld2, 0), 1)) = core::ptr::addr_of!(_15);
RET.fld4 = _2;
_17 = _10;
_8 = _9;
_23.2.2.1 = _1;
SetDiscriminant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0);
_16 = Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).0 as isize;
place!(Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7)).3 = (*_2) < _1;
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).2.1 = (-111530058486658528789773800746709879360_i128) as usize;
place!(Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7)).3 = false ^ false;
Goto(bb16)
}
bb16 = {
place!(Field::<([bool; 1], usize, (u64, u16, [bool; 3], i128))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 2)).1 = RET.fld3 as usize;
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).2.2.2 = RET.fld0;
_1 = !Field::<(u16,)>(Variant(_12.fld1.fld2, 0), 4).0;
RET.fld4 = core::ptr::addr_of_mut!(_15.0);
_23.2.2 = (13230877388864764897_u64, _15.0, Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).2.2.2, 86215492486200754157845765295167271190_i128);
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).2.0 = [Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3];
RET.fld2 = _3;
_25 = [Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3,Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3,Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3,Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3,Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3,Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3,Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7).3];
_7 = _16 << (*_2);
place!(Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7)) = (_14, 3863458229_u32, _13, true);
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).0 = _15;
_9 = core::ptr::addr_of_mut!(place!(Field::<([bool; 1], usize, (u64, u16, [bool; 3], i128))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 2)).1);
place!(Field::<([bool; 1], usize, (u64, u16, [bool; 3], i128))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 2)).2 = (_23.2.2.0, (*_2), Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).2.2.2, _23.2.2.3);
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).1 = Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).2.0;
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).0.0 = _7 as u16;
place!(Field::<([bool; 1], usize, (u64, u16, [bool; 3], i128))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 2)) = (Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).2.0, Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).2.1, _23.2.2);
Goto(bb17)
}
bb17 = {
place!(Field::<*const u8>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 5)) = core::ptr::addr_of!(_13);
_12.fld1.fld0 = (_12.fld0,);
_23.2.0 = Field::<([bool; 1], usize, (u64, u16, [bool; 3], i128))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 2).0;
place!(Field::<(f64, u32, u8, bool)>(Variant(_12.fld1.fld2, 0), 7)).0 = _14;
place!(Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 0), 1)).2.2.1 = !RET.fld1;
_23 = (Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).0, Field::<((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 1).1, Field::<([bool; 1], usize, (u64, u16, [bool; 3], i128))>(Variant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 0), 2));
_12.fld1.fld0 = (_12.fld0,);
place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)) = Adt43::Variant1 { fld0: _23.0.0 };
match _23.2.2.0 {
13230877388864764897 => bb18,
_ => bb16
}
}
bb18 = {
RET = Adt39 { fld0: _23.2.2.2,fld1: _5,fld2: _3,fld3: _12.fld1.fld3,fld4: _2,fld5: _9 };
RET.fld5 = core::ptr::addr_of_mut!(_23.2.1);
_12.fld0 = _12.fld1.fld0.0;
place!(Field::<(u16,)>(Variant(_12.fld1.fld2, 0), 4)) = ((*_2),);
_23.2.1 = Field::<usize>(Variant(_12.fld1.fld2, 0), 3) + Field::<usize>(Variant(_12.fld1.fld2, 0), 3);
_26 = _7 * _7;
(*_2) = _12.fld1.fld0.0 as u16;
SetDiscriminant(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5), 3);
place!(Field::<usize>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 3), 1)) = !(*_8);
place!(Field::<*mut u16>(Variant(_12.fld1.fld2, 0), 6)) = _2;
place!(Field::<i16>(Variant(place!(Field::<Adt43>(Variant(_12.fld1.fld2, 0), 5)), 3), 4)) = 8878_i16;
_23.2.1 = _23.2.2.0 as usize;
(*_2) = Field::<(u16,)>(Variant(_12.fld1.fld2, 0), 4).0 << Field::<(u16,)>(Variant(_12.fld1.fld2, 0), 4).0;
Goto(bb19)
}
bb19 = {
Call(_28 = dump_var(10_usize, 16_usize, Move(_16), 5_usize, Move(_5), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(10_usize, 1_usize, Move(_1), 15_usize, Move(_15), 25_usize, Move(_25), 26_usize, Move(_26)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_28 = dump_var(10_usize, 21_usize, Move(_21), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u16,mut _2: *mut u16,mut _3: u16,mut _4: u16,mut _5: *mut u16) -> u16 {
mir! {
type RET = u16;
let _6: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _7: isize;
let _8: isize;
let _9: ();
let _10: ();
{
(*_2) = !_4;
RET = !(*_5);
(*_2) = _4;
(*_5) = 935908871_u32 as u16;
(*_5) = (-16278015457513206290938437954917749509_i128) as u16;
(*_5) = !_4;
RET = !_4;
_4 = !(*_2);
(*_2) = _4 * _4;
_6.0.2 = !250_u8;
(*_5) = !_3;
_3 = _1 - (*_2);
_3 = _1 ^ (*_2);
_6.2 = ['\u{236ad}','\u{cfd13}','\u{400a2}','\u{106cc5}','\u{dee4d}','\u{c3309}','\u{f08ca}','\u{52b9a}'];
(*_2) = !_3;
_6.1 = (RET,);
_6.0.3 = true;
_6.2 = ['\u{1263e}','\u{4071c}','\u{32baf}','\u{99f9b}','\u{9a28f}','\u{10a1cf}','\u{827e0}','\u{4b5be}'];
_6.2 = ['\u{1c6f9}','\u{215}','\u{cfdc0}','\u{1252}','\u{87cac}','\u{bbb46}','\u{15a0}','\u{14a01}'];
_4 = (*_5) >> (*_2);
_6.0.0 = 157858499116713609496200719022886538327_u128 as f64;
_4 = !(*_2);
_6.0.3 = !false;
_7 = (-8887496961543632241_i64) as isize;
(*_5) = _4;
_6.0.2 = _7 as u8;
_6.1 = ((*_2),);
_6.0.3 = _3 == _4;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(11_usize, 7_usize, Move(_7), 3_usize, Move(_3), 10_usize, _10, 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: u16,mut _2: *mut u16,mut _3: *mut u16,mut _4: *mut u16,mut _5: u16,mut _6: *mut u16) -> bool {
mir! {
type RET = bool;
let _7: bool;
let _8: Adt50;
let _9: u8;
let _10: [u128; 8];
let _11: isize;
let _12: i8;
let _13: *mut usize;
let _14: Adt45;
let _15: Adt45;
let _16: u128;
let _17: isize;
let _18: [u32; 1];
let _19: bool;
let _20: i64;
let _21: isize;
let _22: isize;
let _23: u128;
let _24: [bool; 1];
let _25: Adt55;
let _26: f32;
let _27: Adt45;
let _28: bool;
let _29: ();
let _30: ();
{
RET = false;
_3 = _6;
_1 = (*_6);
RET = (*_3) != _5;
_2 = _3;
(*_3) = !(*_4);
(*_2) = _1;
(*_6) = 421439221763890722_u64 as u16;
(*_3) = (*_4) >> (*_4);
_2 = _3;
(*_6) = '\u{56af9}' as u16;
_2 = core::ptr::addr_of_mut!((*_4));
(*_2) = 220_u8 as u16;
RET = false;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_6) = !_1;
(*_3) = RET as u16;
(*_3) = _1;
_6 = core::ptr::addr_of_mut!((*_2));
_2 = core::ptr::addr_of_mut!((*_4));
(*_3) = RET as u16;
_6 = _2;
(*_6) = (-43_i8) as u16;
_6 = _3;
(*_3) = '\u{5d0fb}' as u16;
_6 = _2;
(*_2) = 217613869795771877172894512912876758140_u128 as u16;
_9 = 99_u8 - 229_u8;
Goto(bb2)
}
bb2 = {
(*_3) = _5;
_6 = _3;
_6 = core::ptr::addr_of_mut!((*_3));
(*_2) = (*_6);
_4 = core::ptr::addr_of_mut!((*_6));
_2 = core::ptr::addr_of_mut!((*_2));
RET = true;
(*_6) = 1145093542_i32 as u16;
_7 = RET;
_14.fld3 = 7719983714028422716_usize as i8;
(*_4) = (*_2);
RET = _7;
_14.fld3 = -9_i8;
_1 = (*_3) >> (*_2);
(*_3) = _1;
_10 = [159450139730641816837931477858667920465_u128,117220553797671483193806188942409201538_u128,200537237475167738890410888419113572692_u128,305534396062437948651313205266914987153_u128,19561336955725986374342672706930426716_u128,336758224385152795205833285965028674307_u128,164436717477765455431055006905791143786_u128,53869658426116982247894931651055799298_u128];
_14.fld3 = (-34_i8) * 88_i8;
_3 = _2;
_15.fld3 = -_14.fld3;
(*_2) = !_1;
_8 = Adt50::Variant3 { fld0: 200204272609457038177771610005646928008_u128,fld1: _9 };
_11 = 3_usize as isize;
Goto(bb3)
}
bb3 = {
_15.fld0 = (6772138582828506189_i64,);
place!(Field::<u128>(Variant(_8, 3), 0)) = 329921675986344461175010563153324270233_u128 >> (*_3);
_9 = (*_6) as u8;
_15.fld3 = !_14.fld3;
_15.fld2 = Adt44::Variant1 { fld0: 12427922443164648241_u64 };
_5 = (*_6);
_14.fld0.0 = -_15.fld0.0;
_17 = _11 + _11;
_15.fld0 = (_14.fld0.0,);
_7 = RET;
place!(Field::<u64>(Variant(_15.fld2, 1), 0)) = 16576656005249361149_u64;
_14.fld3 = _15.fld3;
place!(Field::<u8>(Variant(_8, 3), 1)) = _5 as u8;
_16 = Field::<u128>(Variant(_8, 3), 0);
_12 = -_14.fld3;
_14.fld0 = _15.fld0;
(*_2) = !(*_6);
_12 = -_14.fld3;
_3 = _6;
_2 = core::ptr::addr_of_mut!((*_2));
_9 = Field::<u8>(Variant(_8, 3), 1);
_14.fld2 = Move(_15.fld2);
_11 = _17 - _17;
_2 = core::ptr::addr_of_mut!((*_6));
_9 = Field::<u8>(Variant(_8, 3), 1);
_10 = [Field::<u128>(Variant(_8, 3), 0),_16,_16,Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0),_16,Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0)];
_15.fld2 = Adt44::Variant1 { fld0: Field::<u64>(Variant(_14.fld2, 1), 0) };
_5 = (*_2);
(*_2) = _5;
_14.fld2 = Move(_15.fld2);
_6 = core::ptr::addr_of_mut!((*_6));
Goto(bb4)
}
bb4 = {
_4 = _3;
_9 = !Field::<u8>(Variant(_8, 3), 1);
_10 = [Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0),Field::<u128>(Variant(_8, 3), 0),_16,Field::<u128>(Variant(_8, 3), 0)];
(*_2) = !_1;
_14.fld3 = (-97004337573695973004348825679943377498_i128) as i8;
SetDiscriminant(_14.fld2, 1);
_16 = !Field::<u128>(Variant(_8, 3), 0);
(*_4) = 1_usize as u16;
SetDiscriminant(_8, 3);
place!(Field::<u64>(Variant(_14.fld2, 1), 0)) = 13956078036120107252_u64 << _1;
_19 = Field::<u64>(Variant(_14.fld2, 1), 0) == Field::<u64>(Variant(_14.fld2, 1), 0);
place!(Field::<u128>(Variant(_8, 3), 0)) = _16;
_15.fld2 = Move(_14.fld2);
(*_4) = !_5;
_15.fld0.0 = 6509358740413174419_usize as i64;
(*_3) = !_1;
_18 = [3948382702_u32];
_21 = -_11;
_15.fld0.0 = _5 as i64;
SetDiscriminant(_15.fld2, 2);
_3 = _6;
_11 = 5_usize as isize;
_3 = _4;
_1 = _19 as u16;
place!(Field::<Adt39>(Variant(_15.fld2, 2), 4)).fld1 = _12 as u16;
Goto(bb5)
}
bb5 = {
(*_3) = 10274_i16 as u16;
place!(Field::<u64>(Variant(_15.fld2, 2), 2)) = 2310780032_u32 as u64;
(*_3) = !_5;
_14.fld0 = _15.fld0;
place!(Field::<Adt39>(Variant(_15.fld2, 2), 4)).fld3 = _19 as i8;
_24 = [_19];
_11 = !_21;
_12 = 25563620_i32 as i8;
place!(Field::<[char; 8]>(Variant(_15.fld2, 2), 3)) = ['\u{cc687}','\u{10ed}','\u{3a6e0}','\u{1ae65}','\u{867e8}','\u{636a3}','\u{1bd85}','\u{644e2}'];
_20 = _15.fld0.0 - _15.fld0.0;
place!(Field::<u128>(Variant(_15.fld2, 2), 0)) = Field::<u128>(Variant(_8, 3), 0);
place!(Field::<Adt39>(Variant(_15.fld2, 2), 4)).fld2 = ['\u{adc70}','\u{5316b}','\u{b3b3c}','\u{94b42}','\u{ca918}','\u{5a049}','\u{1a8f8}','\u{105ed7}'];
_1 = !(*_4);
RET = _19;
place!(Field::<Adt39>(Variant(_15.fld2, 2), 4)).fld3 = _12;
_7 = !RET;
(*_6) = _5;
_1 = !(*_4);
place!(Field::<Adt39>(Variant(_15.fld2, 2), 4)).fld0 = [_7,_7,_7];
_24 = [RET];
_14.fld0 = (_15.fld0.0,);
_21 = 1_usize as isize;
_21 = _11 >> (*_6);
Goto(bb6)
}
bb6 = {
Call(_29 = dump_var(12_usize, 11_usize, Move(_11), 20_usize, Move(_20), 18_usize, Move(_18), 21_usize, Move(_21)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_29 = dump_var(12_usize, 5_usize, Move(_5), 12_usize, Move(_12), 19_usize, Move(_19), 30_usize, _30), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: u16,mut _2: i128,mut _3: [bool; 3],mut _4: *mut u16,mut _5: [bool; 3],mut _6: i128,mut _7: i128,mut _8: Adt39,mut _9: bool,mut _10: (u64, u16, [bool; 3], i128),mut _11: [bool; 3],mut _12: (u64, u16, [bool; 3], i128),mut _13: Adt39,mut _14: [bool; 3],mut _15: [bool; 3],mut _16: [bool; 3]) -> i64 {
mir! {
type RET = i64;
let _17: f32;
let _18: f32;
let _19: u64;
let _20: [char; 8];
let _21: [bool; 7];
let _22: i8;
let _23: isize;
let _24: [bool; 7];
let _25: Adt41;
let _26: [bool; 7];
let _27: Adt50;
let _28: char;
let _29: u64;
let _30: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _31: usize;
let _32: (u16,);
let _33: u32;
let _34: [bool; 7];
let _35: [u128; 8];
let _36: [u128; 8];
let _37: i64;
let _38: bool;
let _39: (i64,);
let _40: [char; 8];
let _41: char;
let _42: ();
let _43: ();
{
_13.fld5 = _8.fld5;
_8.fld3 = _13.fld3 * _13.fld3;
_8.fld3 = _13.fld3 & _13.fld3;
_6 = !_10.3;
_13.fld1 = _1 * (*_4);
_15 = _16;
_18 = 60_isize as f32;
_13.fld0 = _14;
(*_4) = !_8.fld1;
_10.3 = !_7;
_10.2 = [_9,_9,_9];
_13.fld2 = _8.fld2;
_13 = Adt39 { fld0: _15,fld1: _8.fld1,fld2: _8.fld2,fld3: _8.fld3,fld4: _8.fld4,fld5: _8.fld5 };
_17 = -_18;
_18 = 9223372036854775807_isize as f32;
(*_4) = !_1;
_10 = (_12.0, _8.fld1, _11, _7);
_19 = _10.0 + _12.0;
(*_4) = !_10.1;
_10.3 = !_2;
_21 = [_9,_9,_9,_9,_9,_9,_9];
_5 = _13.fld0;
RET = 8535064101034705291_i64;
RET = 1529943381136190039_i64;
_11 = [_9,_9,_9];
Call(_1 = core::intrinsics::bswap((*_4)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_4) = !_10.1;
_22 = _8.fld3 - _13.fld3;
_23 = !(-56_isize);
_8 = Adt39 { fld0: _12.2,fld1: _13.fld1,fld2: _13.fld2,fld3: _22,fld4: _4,fld5: _13.fld5 };
_8.fld3 = _22 & _13.fld3;
_12.3 = _6 - _10.3;
_4 = core::ptr::addr_of_mut!((*_4));
_10.2 = _8.fld0;
_4 = _13.fld4;
_6 = _9 as i128;
_13 = Adt39 { fld0: _8.fld0,fld1: _1,fld2: _8.fld2,fld3: _8.fld3,fld4: _8.fld4,fld5: _8.fld5 };
_8 = _13;
_18 = _17 + _17;
_2 = _9 as i128;
_8.fld4 = _13.fld4;
_10.1 = !_8.fld1;
_8.fld5 = _13.fld5;
match _12.0 {
0 => bb2,
1 => bb3,
2 => bb4,
15123254211715381983 => bb6,
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
_24 = _21;
_22 = _8.fld3 * _8.fld3;
_14 = _8.fld0;
_19 = !_10.0;
(*_4) = !_13.fld1;
_13 = Adt39 { fld0: _11,fld1: (*_4),fld2: _8.fld2,fld3: _22,fld4: _8.fld4,fld5: _8.fld5 };
_10 = _12;
_8.fld5 = _13.fld5;
_21 = [_9,_9,_9,_9,_9,_9,_9];
_1 = (*_4);
_13.fld3 = _8.fld3 << RET;
_8.fld5 = _13.fld5;
_4 = core::ptr::addr_of_mut!(_8.fld1);
_23 = !127_isize;
_9 = false;
_8.fld1 = _13.fld1 ^ _13.fld1;
_8.fld0 = _10.2;
_23 = (-68_isize) * (-93_isize);
_16 = [_9,_9,_9];
_23 = 32_isize;
match _12.0 {
0 => bb4,
15123254211715381983 => bb8,
_ => bb7
}
}
bb7 = {
(*_4) = !_10.1;
_22 = _8.fld3 - _13.fld3;
_23 = !(-56_isize);
_8 = Adt39 { fld0: _12.2,fld1: _13.fld1,fld2: _13.fld2,fld3: _22,fld4: _4,fld5: _13.fld5 };
_8.fld3 = _22 & _13.fld3;
_12.3 = _6 - _10.3;
_4 = core::ptr::addr_of_mut!((*_4));
_10.2 = _8.fld0;
_4 = _13.fld4;
_6 = _9 as i128;
_13 = Adt39 { fld0: _8.fld0,fld1: _1,fld2: _8.fld2,fld3: _8.fld3,fld4: _8.fld4,fld5: _8.fld5 };
_8 = _13;
_18 = _17 + _17;
_2 = _9 as i128;
_8.fld4 = _13.fld4;
_10.1 = !_8.fld1;
_8.fld5 = _13.fld5;
match _12.0 {
0 => bb2,
1 => bb3,
2 => bb4,
15123254211715381983 => bb6,
_ => bb5
}
}
bb8 = {
_31 = 2_usize;
_7 = _6 & _10.3;
_31 = 16405059783954725455_usize - 3_usize;
_7 = -_10.3;
_13.fld4 = _4;
_30.2.2.1 = _13.fld3 as u16;
_32 = (_8.fld1,);
_30.0.0 = 16243_i16 as u16;
_5 = _15;
_30.2.0 = [_9];
_12.0 = !_19;
_30.2.2.1 = _32.0;
_12.3 = _10.3;
_15 = [_9,_9,_9];
_30.2.1 = _31 - _31;
_30.2.2.3 = -_10.3;
_8.fld3 = !_13.fld3;
_14 = [_9,_9,_9];
_4 = core::ptr::addr_of_mut!(_10.1);
_12 = _10;
_30.0 = (_30.2.2.1,);
_8.fld4 = core::ptr::addr_of_mut!(_13.fld1);
_25 = Adt41::Variant0 { fld0: (-1192150443_i32) };
_24 = _21;
_10.3 = _6 >> _13.fld1;
match _12.0 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
15123254211715381983 => bb14,
_ => bb13
}
}
bb9 = {
(*_4) = !_10.1;
_22 = _8.fld3 - _13.fld3;
_23 = !(-56_isize);
_8 = Adt39 { fld0: _12.2,fld1: _13.fld1,fld2: _13.fld2,fld3: _22,fld4: _4,fld5: _13.fld5 };
_8.fld3 = _22 & _13.fld3;
_12.3 = _6 - _10.3;
_4 = core::ptr::addr_of_mut!((*_4));
_10.2 = _8.fld0;
_4 = _13.fld4;
_6 = _9 as i128;
_13 = Adt39 { fld0: _8.fld0,fld1: _1,fld2: _8.fld2,fld3: _8.fld3,fld4: _8.fld4,fld5: _8.fld5 };
_8 = _13;
_18 = _17 + _17;
_2 = _9 as i128;
_8.fld4 = _13.fld4;
_10.1 = !_8.fld1;
_8.fld5 = _13.fld5;
match _12.0 {
0 => bb2,
1 => bb3,
2 => bb4,
15123254211715381983 => bb6,
_ => bb5
}
}
bb10 = {
_24 = _21;
_22 = _8.fld3 * _8.fld3;
_14 = _8.fld0;
_19 = !_10.0;
(*_4) = !_13.fld1;
_13 = Adt39 { fld0: _11,fld1: (*_4),fld2: _8.fld2,fld3: _22,fld4: _8.fld4,fld5: _8.fld5 };
_10 = _12;
_8.fld5 = _13.fld5;
_21 = [_9,_9,_9,_9,_9,_9,_9];
_1 = (*_4);
_13.fld3 = _8.fld3 << RET;
_8.fld5 = _13.fld5;
_4 = core::ptr::addr_of_mut!(_8.fld1);
_23 = !127_isize;
_9 = false;
_8.fld1 = _13.fld1 ^ _13.fld1;
_8.fld0 = _10.2;
_23 = (-68_isize) * (-93_isize);
_16 = [_9,_9,_9];
_23 = 32_isize;
match _12.0 {
0 => bb4,
15123254211715381983 => bb8,
_ => bb7
}
}
bb11 = {
Return()
}
bb12 = {
(*_4) = !_10.1;
_22 = _8.fld3 - _13.fld3;
_23 = !(-56_isize);
_8 = Adt39 { fld0: _12.2,fld1: _13.fld1,fld2: _13.fld2,fld3: _22,fld4: _4,fld5: _13.fld5 };
_8.fld3 = _22 & _13.fld3;
_12.3 = _6 - _10.3;
_4 = core::ptr::addr_of_mut!((*_4));
_10.2 = _8.fld0;
_4 = _13.fld4;
_6 = _9 as i128;
_13 = Adt39 { fld0: _8.fld0,fld1: _1,fld2: _8.fld2,fld3: _8.fld3,fld4: _8.fld4,fld5: _8.fld5 };
_8 = _13;
_18 = _17 + _17;
_2 = _9 as i128;
_8.fld4 = _13.fld4;
_10.1 = !_8.fld1;
_8.fld5 = _13.fld5;
match _12.0 {
0 => bb2,
1 => bb3,
2 => bb4,
15123254211715381983 => bb6,
_ => bb5
}
}
bb13 = {
Return()
}
bb14 = {
_23 = 9223372036854775807_isize;
_23 = 18713_i16 as isize;
_9 = _13.fld1 >= _30.2.2.1;
_30.2.2.1 = _32.0 + _1;
_30.2.2.0 = _2 as u64;
_34 = _24;
_3 = [_9,_9,_9];
_19 = _30.2.2.0 >> _30.2.2.3;
_8.fld1 = _30.0.0 & _30.2.2.1;
_7 = _10.3;
_8.fld4 = _13.fld4;
_19 = '\u{2e151}' as u64;
place!(Field::<i32>(Variant(_25, 0), 0)) = 1314641168_i32 & 1314527717_i32;
_38 = _9;
_24 = [_9,_9,_9,_9,_9,_38,_9];
_35 = [303618011664065844283260622992596313810_u128,103461031601734700775548319581276068918_u128,144181948055991296746302690281802579027_u128,292564233461340180216064912274357226145_u128,140353847251355504215308363582143374958_u128,268564059191451834628287851626911146962_u128,174432195281872126361975157235183482493_u128,136526931297588607235289581504458294235_u128];
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(13_usize, 22_usize, Move(_22), 5_usize, Move(_5), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(13_usize, 11_usize, Move(_11), 23_usize, Move(_23), 2_usize, Move(_2), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(13_usize, 15_usize, Move(_15), 12_usize, Move(_12), 14_usize, Move(_14), 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: Adt39,mut _2: i8,mut _3: f64,mut _4: isize,mut _5: i8,mut _6: Adt39,mut _7: Adt39,mut _8: [bool; 3],mut _9: [bool; 3],mut _10: *mut usize,mut _11: [bool; 3]) -> f32 {
mir! {
type RET = f32;
let _12: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _13: [bool; 1];
let _14: isize;
let _15: char;
let _16: f64;
let _17: *mut u16;
let _18: [char; 8];
let _19: i128;
let _20: i128;
let _21: isize;
let _22: [bool; 1];
let _23: bool;
let _24: bool;
let _25: char;
let _26: *const u8;
let _27: f64;
let _28: Adt51;
let _29: u8;
let _30: (u16,);
let _31: f64;
let _32: Adt49;
let _33: ();
let _34: ();
{
_6.fld0 = [true,true,true];
_6.fld3 = _1.fld3;
_1.fld0 = [true,true,false];
_12.1 = (_6.fld1,);
_1 = _7;
_6.fld1 = _12.1.0 << _4;
_12.2 = ['\u{cff}','\u{9225}','\u{22810}','\u{10dc48}','\u{e12f9}','\u{4274c}','\u{b7231}','\u{8f99a}'];
_7.fld2 = ['\u{956f8}','\u{b2a08}','\u{6f9b5}','\u{10b06f}','\u{2159}','\u{8907f}','\u{80496}','\u{e1479}'];
_6.fld0 = _11;
_12.0.2 = 78_u8 * 18_u8;
_10 = _1.fld5;
Goto(bb1)
}
bb1 = {
_14 = !_4;
_14 = _4;
_9 = [false,false,false];
_2 = _6.fld3;
_15 = '\u{2c59f}';
_1.fld1 = !_7.fld1;
_1.fld4 = core::ptr::addr_of_mut!(_1.fld1);
Call(_12.0.0 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _5;
_17 = _1.fld4;
_1.fld4 = _7.fld4;
(*_17) = 123544499462691373899127985099265400830_u128 as u16;
_20 = 156143586911619220348385394588163356257_i128 * (-143127608337089754349465247400863213998_i128);
Goto(bb3)
}
bb3 = {
_12.0.3 = !false;
_12.0.3 = _12.1.0 > _1.fld1;
_18 = [_15,_15,_15,_15,_15,_15,_15,_15];
_13 = [_12.0.3];
_6.fld4 = core::ptr::addr_of_mut!(_6.fld1);
Goto(bb4)
}
bb4 = {
_4 = _14;
_1.fld1 = _6.fld1;
_14 = 134827603443817300241068178045624039056_u128 as isize;
_8 = _6.fld0;
_22 = _13;
_7.fld3 = !_5;
_7.fld5 = _10;
Goto(bb5)
}
bb5 = {
_12.0.2 = _3 as u8;
_11 = [_12.0.3,_12.0.3,_12.0.3];
_14 = _4 | _4;
_1.fld0 = [_12.0.3,_12.0.3,_12.0.3];
_5 = !_7.fld3;
_6.fld0 = [_12.0.3,_12.0.3,_12.0.3];
_7 = Adt39 { fld0: _6.fld0,fld1: _6.fld1,fld2: _12.2,fld3: _6.fld3,fld4: _17,fld5: _10 };
_8 = [_12.0.3,_12.0.3,_12.0.3];
_6.fld2 = _1.fld2;
_2 = _1.fld3;
_1.fld4 = _7.fld4;
_7.fld1 = _12.1.0;
_10 = _1.fld5;
_6.fld3 = _2 & _1.fld3;
_4 = _14 & _14;
_11 = _9;
(*_17) = _12.1.0;
(*_17) = _12.0.3 as u16;
_27 = -_12.0.0;
_1.fld1 = 15426738071168840746_u64 as u16;
_6.fld3 = _5 ^ _7.fld3;
Goto(bb6)
}
bb6 = {
_29 = _12.0.2;
Call(_23 = fn15(_4, _6.fld5, _1.fld5, _1, _6.fld4, _17, _18, _12.1, _1.fld4, _1, _6.fld4, _1.fld0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6.fld1 = 59720333925816607417684408737918337565_u128 as u16;
_8 = _9;
_1.fld5 = _7.fld5;
_21 = 180188418457641248525283288819349629960_u128 as isize;
_26 = core::ptr::addr_of!(_12.0.2);
_7.fld2 = [_15,_15,_15,_15,_15,_15,_15,_15];
_5 = -_7.fld3;
_12.0 = (_27, 3323048624_u32, _29, _23);
_7.fld1 = !_1.fld1;
_3 = _12.0.0;
_12.2 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET = _1.fld1 as f32;
_25 = _15;
_1.fld2 = [_25,_15,_15,_15,_25,_25,_15,_25];
_6.fld0 = _7.fld0;
_5 = RET as i8;
Goto(bb8)
}
bb8 = {
Call(_33 = dump_var(14_usize, 21_usize, Move(_21), 14_usize, Move(_14), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_33 = dump_var(14_usize, 4_usize, Move(_4), 25_usize, Move(_25), 15_usize, Move(_15), 5_usize, Move(_5)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: *mut usize,mut _3: *mut usize,mut _4: Adt39,mut _5: *mut u16,mut _6: *mut u16,mut _7: [char; 8],mut _8: (u16,),mut _9: *mut u16,mut _10: Adt39,mut _11: *mut u16,mut _12: [bool; 3]) -> bool {
mir! {
type RET = bool;
let _13: [u128; 8];
let _14: [u32; 1];
let _15: u16;
let _16: f32;
let _17: [u128; 8];
let _18: usize;
let _19: char;
let _20: [bool; 7];
let _21: Adt54;
let _22: bool;
let _23: ((f64, u32, u8, bool), (u16,), [char; 8]);
let _24: [char; 8];
let _25: u32;
let _26: u128;
let _27: ([u32; 1], &'static isize, &'static isize, [bool; 1]);
let _28: f32;
let _29: [bool; 1];
let _30: bool;
let _31: [bool; 3];
let _32: usize;
let _33: Adt40;
let _34: (u16,);
let _35: isize;
let _36: bool;
let _37: bool;
let _38: (*const u8, u8, *const u8, &'static isize);
let _39: u128;
let _40: (i64,);
let _41: u64;
let _42: (f64, u32, u8, bool);
let _43: bool;
let _44: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128)));
let _45: f32;
let _46: ();
let _47: ();
{
_11 = _5;
_10.fld1 = 2_usize as u16;
_7 = ['\u{44943}','\u{4165e}','\u{e40b5}','\u{aff69}','\u{85e29}','\u{5c516}','\u{fe32b}','\u{a866c}'];
(*_11) = _8.0;
_10 = Adt39 { fld0: _4.fld0,fld1: (*_5),fld2: _7,fld3: _4.fld3,fld4: _9,fld5: _3 };
_10 = _4;
_5 = _4.fld4;
RET = true;
_12 = [RET,RET,RET];
_10.fld2 = ['\u{4335f}','\u{17559}','\u{98a3c}','\u{10138a}','\u{1e5dd}','\u{85263}','\u{45197}','\u{d0351}'];
_9 = _5;
_1 = (-1276793161_i32) as isize;
_12 = [RET,RET,RET];
_10.fld4 = _11;
_4.fld4 = core::ptr::addr_of_mut!((*_9));
(*_5) = _8.0 ^ (*_11);
Goto(bb1)
}
bb1 = {
_4.fld0 = [RET,RET,RET];
_2 = _3;
_5 = core::ptr::addr_of_mut!((*_9));
(*_6) = !_4.fld1;
_15 = (-390235423_i32) as u16;
(*_11) = _4.fld1;
_10 = _4;
_6 = core::ptr::addr_of_mut!((*_5));
_4.fld5 = _10.fld5;
(*_5) = RET as u16;
_5 = _10.fld4;
_9 = _11;
_10.fld1 = (*_11);
_10.fld3 = _4.fld3;
_10.fld2 = _4.fld2;
_10.fld0 = [RET,RET,RET];
_10 = _4;
_18 = 5056409036112365812_usize + 17187223556434034790_usize;
Goto(bb2)
}
bb2 = {
(*_5) = !_10.fld1;
_16 = 600671563_i32 as f32;
(*_9) = (*_6) - _4.fld1;
_4.fld4 = core::ptr::addr_of_mut!(_4.fld1);
_10.fld5 = _4.fld5;
(*_11) = !_8.0;
_10.fld4 = core::ptr::addr_of_mut!((*_9));
(*_6) = (*_11);
_10.fld3 = _18 as i8;
_10.fld5 = core::ptr::addr_of_mut!(_18);
_4.fld1 = !(*_11);
(*_5) = RET as u16;
_4.fld2 = ['\u{22400}','\u{3d6de}','\u{6a640}','\u{967c5}','\u{aac2e}','\u{4bdf3}','\u{c0470}','\u{96979}'];
(*_9) = (-19730_i16) as u16;
_10.fld2 = _7;
_8 = ((*_11),);
_11 = core::ptr::addr_of_mut!((*_5));
Goto(bb3)
}
bb3 = {
_10.fld0 = [RET,RET,RET];
(*_9) = _18 as u16;
_16 = 271632918037795614199990018057785549233_u128 as f32;
_14 = [194403547_u32];
_23.0.0 = (-1172823957_i32) as f64;
_20 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb4)
}
bb4 = {
_24 = ['\u{3cd89}','\u{4844}','\u{ea5a9}','\u{6d7c1}','\u{ab464}','\u{5692c}','\u{9bb49}','\u{499c6}'];
_21.fld4 = 12367409115688660106_u64;
_21.fld4 = 10274_i16 as u64;
_10.fld1 = _15;
_10.fld2 = _4.fld2;
(*_9) = _16 as u16;
Goto(bb5)
}
bb5 = {
RET = !true;
Goto(bb6)
}
bb6 = {
_27.0 = [3590420473_u32];
_27.2 = &_1;
_27.3 = [RET];
_17 = [7693119916097575020743178451094705236_u128,229406308631559398848647632659819835073_u128,23997516184224963179150240388450856402_u128,281111004912254138168961214919159607983_u128,307197096414921184296463456279250219092_u128,297570819304153005110092930816677511549_u128,57094683274843035654441192971563415382_u128,152820136278224898007130907166453791198_u128];
RET = _8.0 < _10.fld1;
RET = true ^ true;
_5 = core::ptr::addr_of_mut!((*_5));
_27.1 = &_1;
_21.fld2 = Adt40::Variant1 { fld0: '\u{1c1}' };
_26 = '\u{c9f59}' as u128;
(*_6) = (*_9) + _4.fld1;
(*_9) = (*_5);
_21.fld0 = core::ptr::addr_of_mut!(_18);
_23.1 = _8;
_23.1.0 = (*_9);
_8.0 = (*_6);
Goto(bb7)
}
bb7 = {
_21.fld5 = (-1254972928_i32) >> _23.1.0;
_5 = core::ptr::addr_of_mut!((*_9));
_26 = 232224456380808503978914432202938554196_u128 << (*_11);
_13 = _17;
_23.0.3 = _10.fld3 <= _10.fld3;
_5 = _4.fld4;
_21.fld0 = core::ptr::addr_of_mut!(_18);
_10.fld3 = -_4.fld3;
_4 = Adt39 { fld0: _12,fld1: _23.1.0,fld2: _24,fld3: _10.fld3,fld4: _10.fld4,fld5: _3 };
_23.0.3 = !RET;
(*_9) = _18 as u16;
place!(Field::<char>(Variant(_21.fld2, 1), 0)) = '\u{6817f}';
_4 = Adt39 { fld0: _12,fld1: (*_11),fld2: _24,fld3: _10.fld3,fld4: _11,fld5: _21.fld0 };
_4.fld3 = _1 as i8;
_4 = Adt39 { fld0: _12,fld1: (*_6),fld2: _7,fld3: _10.fld3,fld4: _6,fld5: _10.fld5 };
_18 = 9242178103774326345_usize & 15134562527530041244_usize;
_27.0 = _14;
_20 = [RET,_23.0.3,_23.0.3,_23.0.3,RET,RET,_23.0.3];
_3 = _4.fld5;
Goto(bb8)
}
bb8 = {
SetDiscriminant(_21.fld2, 1);
_23.0.3 = (*_11) > _23.1.0;
_27.2 = Move(_27.1);
place!(Field::<char>(Variant(_21.fld2, 1), 0)) = '\u{a3f16}';
_29 = [_23.0.3];
_23.0.2 = !95_u8;
_31 = [_23.0.3,_23.0.3,RET];
_6 = core::ptr::addr_of_mut!((*_5));
_22 = !_23.0.3;
(*_3) = 6_usize ^ 5_usize;
_30 = _22;
(*_9) = !_4.fld1;
_31 = [_22,_30,_30];
_27.2 = &_1;
Goto(bb9)
}
bb9 = {
_23.0.2 = 157_u8 - 113_u8;
Goto(bb10)
}
bb10 = {
_9 = _6;
_5 = core::ptr::addr_of_mut!((*_6));
_19 = Field::<char>(Variant(_21.fld2, 1), 0);
_6 = core::ptr::addr_of_mut!((*_5));
_13 = [_26,_26,_26,_26,_26,_26,_26,_26];
_10.fld0 = _31;
_8 = _23.1;
(*_9) = _15;
_24 = _4.fld2;
_22 = _30 ^ _30;
_21.fld5 = (-99009482729844194252282401553067674450_i128) as i32;
_26 = 332838989888322818956909215297890705498_u128 | 14357384424981441667093744831105735333_u128;
_29 = [_22];
_5 = core::ptr::addr_of_mut!((*_9));
_23.1.0 = _18 as u16;
_21.fld3 = 3861375514_u32;
_23.0.3 = !_22;
_9 = _6;
_15 = !_23.1.0;
_9 = _10.fld4;
_8.0 = 131391105610754500527526906049602649938_i128 as u16;
_1 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_16 = (-15726_i16) as f32;
_19 = Field::<char>(Variant(_21.fld2, 1), 0);
(*_11) = (*_9);
Goto(bb11)
}
bb11 = {
_8.0 = _23.0.2 as u16;
_23.1.0 = _21.fld4 as u16;
_25 = _21.fld3;
_23.1.0 = _10.fld3 as u16;
_34 = ((*_9),);
_10.fld2 = _4.fld2;
_27.2 = &_1;
(*_9) = !(*_5);
_4.fld0 = [_30,RET,_22];
_20 = [_23.0.3,_22,_23.0.3,RET,_22,_30,_22];
(*_11) = (*_6);
_6 = core::ptr::addr_of_mut!((*_9));
(*_5) = _15 + _34.0;
_25 = !_21.fld3;
_10 = Adt39 { fld0: _12,fld1: (*_5),fld2: _24,fld3: _4.fld3,fld4: _9,fld5: _21.fld0 };
_31 = _4.fld0;
_33 = Adt40::Variant0 { fld0: _21.fld5,fld1: 26900195967401893912043883265665172513_i128,fld2: _21.fld0 };
_38.1 = !_23.0.2;
(*_9) = !_15;
_21.fld1 = !_26;
_37 = _22 != _22;
_21.fld5 = -Field::<i32>(Variant(_33, 0), 0);
_33 = Move(_21.fld2);
SetDiscriminant(_33, 1);
_32 = !_18;
_4 = Adt39 { fld0: _31,fld1: _34.0,fld2: _7,fld3: _10.fld3,fld4: _6,fld5: _10.fld5 };
Goto(bb12)
}
bb12 = {
_26 = _21.fld1 ^ _21.fld1;
_35 = _1 & _1;
_10.fld1 = _21.fld4 as u16;
_38.3 = Move(_27.2);
_9 = _5;
_10 = Adt39 { fld0: _4.fld0,fld1: (*_5),fld2: _24,fld3: _4.fld3,fld4: _5,fld5: _2 };
_21.fld5 = !(-924343878_i32);
_3 = core::ptr::addr_of_mut!(_32);
_39 = _26 - _26;
_23.0.1 = _39 as u32;
_4.fld5 = core::ptr::addr_of_mut!((*_3));
_28 = _4.fld3 as f32;
_8 = (_4.fld1,);
_10.fld4 = core::ptr::addr_of_mut!((*_5));
_10.fld2 = [_19,_19,_19,_19,_19,_19,_19,_19];
_10.fld1 = _32 as u16;
_25 = !_23.0.1;
_4.fld4 = core::ptr::addr_of_mut!((*_9));
_10.fld4 = core::ptr::addr_of_mut!((*_5));
Goto(bb13)
}
bb13 = {
_15 = !_8.0;
_1 = _35 << (*_6);
_38.3 = &_35;
_27.2 = Move(_38.3);
_23.0.2 = _38.1;
_34 = _8;
_38.2 = core::ptr::addr_of!(_38.1);
_6 = _5;
_23.1 = ((*_9),);
_35 = -_1;
_23.2 = [_19,_19,_19,_19,_19,_19,_19,_19];
_15 = _39 as u16;
_16 = _28;
_1 = _21.fld4 as isize;
_43 = !_22;
(*_3) = !_18;
_27.3 = _29;
(*_11) = _4.fld1 << _15;
_4.fld4 = _6;
_21.fld2 = Adt40::Variant0 { fld0: _21.fld5,fld1: (-49215886118516136058714840554469008329_i128),fld2: _10.fld5 };
_10.fld3 = !_4.fld3;
_12 = _31;
(*_5) = _35 as u16;
_40 = ((-1157068242873922541_i64),);
_44.0.0 = !(*_5);
Goto(bb14)
}
bb14 = {
_23.0.3 = !_30;
_21.fld5 = Field::<i32>(Variant(_21.fld2, 0), 0);
_10.fld0 = [_22,_43,_22];
place!(Field::<char>(Variant(_33, 1), 0)) = _19;
RET = !_22;
_45 = _16;
_42.2 = _38.1;
_44.2.2.3 = -(-66899491517988012256589189284426250930_i128);
_21 = Adt54 { fld0: _4.fld5,fld1: _39,fld2: Move(_33),fld3: _25,fld4: 5553628930819158634_u64,fld5: (-1892787493_i32) };
_18 = !(*_3);
_6 = _10.fld4;
_4.fld0 = [_22,_43,_43];
_44.2.1 = _18;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(15_usize, 12_usize, Move(_12), 29_usize, Move(_29), 35_usize, Move(_35), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(15_usize, 30_usize, Move(_30), 7_usize, Move(_7), 26_usize, Move(_26), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(15_usize, 1_usize, Move(_1), 18_usize, Move(_18), 34_usize, Move(_34), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: u32,mut _2: *mut u16,mut _3: u16,mut _4: *mut usize,mut _5: *mut u16,mut _6: u8,mut _7: u16,mut _8: u16) -> ((f64, u32, u8, bool), (u16,), [char; 8]) {
mir! {
type RET = ((f64, u32, u8, bool), (u16,), [char; 8]);
let _9: i128;
let _10: isize;
let _11: *mut &'static isize;
let _12: f64;
let _13: [bool; 1];
let _14: ();
let _15: ();
{
RET.1 = (_7,);
RET.2 = ['\u{d04ef}','\u{adfa8}','\u{b27a0}','\u{fe47c}','\u{8dabe}','\u{80395}','\u{86c6c}','\u{8138d}'];
RET.2 = ['\u{a9595}','\u{c0f4c}','\u{1191a}','\u{4b043}','\u{62f35}','\u{95963}','\u{ae101}','\u{d648}'];
RET.1 = ((*_5),);
RET.0.3 = !false;
_3 = (*_2) | (*_5);
_9 = 82206239889527992485695367760232300935_i128;
_7 = (*_2) ^ (*_5);
(*_2) = (-22349_i16) as u16;
(*_4) = 18154015421537161876_usize * 1_usize;
RET.0.3 = false;
_10 = !17_isize;
RET.0.1 = 19_i8 as u32;
(*_5) = _3 * _3;
RET.0.2 = _6;
RET.0.0 = 14_i8 as f64;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(16_usize, 1_usize, Move(_1), 6_usize, Move(_6), 9_usize, Move(_9), 15_usize, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{5ecda}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(2228502873_u32), std::hint::black_box(18440207882931705874_u64), std::hint::black_box(24712_u16), std::hint::black_box((-5884824953037907212_i64)), std::hint::black_box(169166906039014936937443275347709631554_i128), std::hint::black_box(5_usize));
                
            }
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt39{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt39 {
fld0: [bool; 3],
fld1: u16,
fld2: [char; 8],
fld3: i8,
fld4: *mut u16,
fld5: *mut usize,
}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: i32,
fld1: i128,
fld2: *mut usize,

},
Variant1{
fld0: char,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: i32,

},
Variant1{
fld0: *const (u16,),
fld1: (f64, u32, u8, bool),
fld2: [bool; 3],
fld3: *mut usize,
fld4: Adt39,
fld5: i32,
fld6: [u32; 1],

},
Variant2{
fld0: u8,
fld1: char,
fld2: [bool; 7],
fld3: (u64, u16, [bool; 3], i128),
fld4: ((f64, u32, u8, bool), (u16,), [char; 8]),

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128))),
fld1: (f64, u32, u8, bool),

},
Variant1{
fld0: (u16,),
fld1: (i64,),
fld2: u32,
fld3: [bool; 7],
fld4: [u32; 1],

},
Variant2{
fld0: bool,
fld1: ((f64, u32, u8, bool), (u16,), [char; 8]),
fld2: *const (u16,),
fld3: u8,
fld4: (u64, u16, [bool; 3], i128),
fld5: [bool; 3],
fld6: *const u8,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128))),
fld2: ([bool; 1], usize, (u64, u16, [bool; 3], i128)),
fld3: *mut usize,
fld4: i16,
fld5: *const u8,
fld6: Adt42,

},
Variant1{
fld0: u16,

},
Variant2{
fld0: i64,

},
Variant3{
fld0: f64,
fld1: usize,
fld2: *const (u16,),
fld3: Adt42,
fld4: i16,
fld5: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128))),
fld6: i64,
fld7: u128,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: *const (u16,),
fld2: *mut usize,
fld3: usize,
fld4: (u16,),
fld5: Adt43,
fld6: *mut u16,
fld7: (f64, u32, u8, bool),

},
Variant1{
fld0: u64,

},
Variant2{
fld0: u128,
fld1: f32,
fld2: u64,
fld3: [char; 8],
fld4: Adt39,
fld5: *const (u16,),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: (i64,),
fld1: *mut usize,
fld2: Adt44,
fld3: i8,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u128,
fld1: usize,
fld2: u32,
fld3: Adt39,
fld4: [bool; 3],
fld5: u64,
fld6: f64,

},
Variant1{
fld0: f32,
fld1: char,
fld2: isize,
fld3: i8,
fld4: (u64, u16, [bool; 3], i128),
fld5: Adt42,
fld6: Adt41,
fld7: u32,

},
Variant2{
fld0: [bool; 3],
fld1: *mut u16,
fld2: *const (u16,),
fld3: Adt39,
fld4: Adt41,
fld5: u64,
fld6: *mut usize,

},
Variant3{
fld0: Adt44,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: i64,
fld1: Adt45,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: [bool; 1],
fld1: i32,
fld2: (i64,),
fld3: (u16,),
fld4: Adt39,
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
fld0: ((f64, u32, u8, bool), (u16,), [char; 8]),
fld1: Adt47,
fld2: (u16,),

},
Variant1{
fld0: u32,
fld1: ([bool; 1], usize, (u64, u16, [bool; 3], i128)),
fld2: i64,

},
Variant2{
fld0: (f64, u32, u8, bool),
fld1: char,
fld2: Adt46,
fld3: i128,
fld4: Adt43,
fld5: *mut u16,
fld6: ((f64, u32, u8, bool), (u16,), [char; 8]),

},
Variant3{
fld0: u64,
fld1: [bool; 7],
fld2: isize,
fld3: [bool; 3],
fld4: i16,
fld5: (u64, u16, [bool; 3], i128),

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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut u16,
fld1: Adt47,
fld2: Adt43,
fld3: i8,
fld4: Adt42,
fld5: Adt45,

},
Variant1{
fld0: ([bool; 1], usize, (u64, u16, [bool; 3], i128)),
fld1: Adt43,
fld2: Adt42,
fld3: [bool; 1],
fld4: f64,
fld5: Adt49,

},
Variant2{
fld0: *mut usize,
fld1: char,
fld2: usize,

},
Variant3{
fld0: u128,
fld1: u8,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt46,
fld1: (u16,),
fld2: [bool; 7],

},
Variant1{
fld0: Adt41,
fld1: ((f64, u32, u8, bool), (u16,), [char; 8]),
fld2: *mut usize,
fld3: u128,
fld4: *const u8,
fld5: ([bool; 1], usize, (u64, u16, [bool; 3], i128)),
fld6: Adt45,
fld7: i128,

},
Variant2{
fld0: (f64, u32, u8, bool),

},
Variant3{
fld0: Adt43,
fld1: Adt50,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: i32,
fld1: Adt50,
fld2: [bool; 3],
fld3: ((u16,), [bool; 1], ([bool; 1], usize, (u64, u16, [bool; 3], i128))),

},
Variant1{
fld0: Adt45,
fld1: Adt40,
fld2: [u32; 1],
fld3: [bool; 7],
fld4: (u64, u16, [bool; 3], i128),

},
Variant2{
fld0: [u128; 8],
fld1: u128,
fld2: Adt49,
fld3: [bool; 1],
fld4: Adt51,
fld5: f32,
fld6: Adt45,
fld7: [u32; 1],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: (u16,),
fld1: u32,
fld2: ([bool; 1], usize, (u64, u16, [bool; 3], i128)),

},
Variant1{
fld0: usize,
fld1: Adt51,
fld2: ([bool; 1], usize, (u64, u16, [bool; 3], i128)),

},
Variant2{
fld0: *const (u16,),

},
Variant3{
fld0: u16,
fld1: f64,
fld2: *const (u16,),
fld3: u64,
fld4: (u64, u16, [bool; 3], i128),
fld5: *mut usize,
fld6: i64,
fld7: u128,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: *mut usize,
fld1: u128,
fld2: Adt40,
fld3: u32,
fld4: u64,
fld5: i32,
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
fld0: f32,
fld1: u128,
fld2: Adt45,
fld3: *mut usize,
fld4: i16,

},
Variant1{
fld0: i32,
fld1: i16,

},
Variant2{
fld0: Adt50,
fld1: u16,
fld2: Adt53,
fld3: [u128; 8],
fld4: (u64, u16, [bool; 3], i128),
fld5: Adt54,
fld6: Adt49,

}}

