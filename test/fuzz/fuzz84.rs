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
pub fn fn0(mut _1: usize,mut _2: char,mut _3: u64) -> i128 {
mir! {
type RET = i128;
let _4: char;
let _5: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _6: f32;
let _7: [u32; 1];
let _8: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _9: f64;
let _10: (bool, u32, bool, u32);
let _11: Adt48;
let _12: Adt58;
let _13: Adt44;
let _14: u64;
let _15: (u64, char);
let _16: ((i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)), [bool; 7], u64, *mut f32, (u64, char));
let _17: *const u8;
let _18: (i16, f32);
let _19: i8;
let _20: [i64; 5];
let _21: *mut f32;
let _22: i64;
let _23: f64;
let _24: [u32; 1];
let _25: (i16, bool, u64);
let _26: isize;
let _27: [u32; 3];
let _28: f64;
let _29: (bool, u32, bool, u32);
let _30: isize;
let _31: ();
let _32: ();
{
_2 = '\u{f0283}';
_3 = 504489061_i32 as u64;
_2 = '\u{79ec3}';
_3 = 16501283551116907533_u64 << 7869511222697377945_i64;
_3 = 1148579218472757275_u64;
RET = _3 as i128;
RET = 2_usize as i128;
_3 = 2409239699007298888_u64 << RET;
_1 = 5_usize * 1_usize;
_1 = RET as usize;
_1 = 9517545931075854599_usize & 5_usize;
_4 = _2;
_5.5.2 = !_3;
_5.1 = (-95_i8) as f32;
_5.5 = (8498_i16, false, _3);
_3 = _5.5.0 as u64;
_4 = _2;
_5.2 = !_5.5.1;
Goto(bb1)
}
bb1 = {
_4 = _2;
_5.2 = _4 == _4;
_1 = _5.1 as usize;
_2 = _4;
_5.2 = !_5.5.1;
_7 = [3408032612_u32];
_5.4 = _1 + _1;
_5.0 = [250629689285174015802518112336394518012_u128,113661569834173800661901352800063138947_u128,322315099984878959851671816268435745832_u128,64748319857906238257390103447663247719_u128,67870648396223680267543817251889114971_u128];
_5.5 = (10605_i16, _5.2, _3);
_5.1 = 23_i8 as f32;
Goto(bb2)
}
bb2 = {
_5.5.2 = 32050_u16 as u64;
_5.5.0 = (-30189_i16);
_5.5 = ((-14102_i16), _5.2, _3);
_5.2 = _5.5.1;
_3 = _5.5.2;
_8.0.3 = !_1;
_8.1 = (_5.0, _5.1, _5.5.1, _5.4, _1, _5.5);
_8.1.4 = _5.4;
_8.0.5.1 = _8.1.5.0 >= _5.5.0;
_8.0.5.2 = !_5.5.2;
_8.0.5 = (_5.5.0, _8.1.2, _8.1.5.2);
_4 = _2;
_8.0.3 = 3220680708_u32 as usize;
_5.4 = _8.1.4 & _8.1.4;
_5 = _8.1;
_8.0.5 = (_5.5.0, _5.5.1, _5.5.2);
_5.5.2 = _8.1.5.2;
_6 = -_5.1;
_1 = _5.4 + _5.4;
Goto(bb3)
}
bb3 = {
_9 = _1 as f64;
_8.1.5.2 = !_8.0.5.2;
_10 = (_5.5.1, 1210636974_u32, _5.2, 12912034_u32);
_9 = _5.5.0 as f64;
_8.1.4 = !_1;
RET = _5.5.2 as i128;
_8.0.0 = _5.0;
_2 = _4;
_5.4 = _1;
_8.0.3 = _1;
Call(_10.3 = fn1(_8.0.3, _8.1.5, _1, _8.0.0, _5.5.0, _8.0.5.0, _1, _10.1, _10.1, _5.5, _8.1.4, _8.1.0, _8.1, _10.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8.0.5.2 = _5.5.2 << _8.0.3;
_3 = !_8.0.5.2;
_10.2 = _5.2;
_5.1 = -_6;
_9 = _10.3 as f64;
_11.fld0.fld1 = [_10.3];
_8.1.5.1 = !_5.2;
_4 = _2;
match _5.5.0 {
0 => bb3,
1 => bb5,
2 => bb6,
340282366920938463463374607431768197354 => bb8,
_ => bb7
}
}
bb5 = {
_9 = _1 as f64;
_8.1.5.2 = !_8.0.5.2;
_10 = (_5.5.1, 1210636974_u32, _5.2, 12912034_u32);
_9 = _5.5.0 as f64;
_8.1.4 = !_1;
RET = _5.5.2 as i128;
_8.0.0 = _5.0;
_2 = _4;
_5.4 = _1;
_8.0.3 = _1;
Call(_10.3 = fn1(_8.0.3, _8.1.5, _1, _8.0.0, _5.5.0, _8.0.5.0, _1, _10.1, _10.1, _5.5, _8.1.4, _8.1.0, _8.1, _10.1), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_5.5.2 = 32050_u16 as u64;
_5.5.0 = (-30189_i16);
_5.5 = ((-14102_i16), _5.2, _3);
_5.2 = _5.5.1;
_3 = _5.5.2;
_8.0.3 = !_1;
_8.1 = (_5.0, _5.1, _5.5.1, _5.4, _1, _5.5);
_8.1.4 = _5.4;
_8.0.5.1 = _8.1.5.0 >= _5.5.0;
_8.0.5.2 = !_5.5.2;
_8.0.5 = (_5.5.0, _8.1.2, _8.1.5.2);
_4 = _2;
_8.0.3 = 3220680708_u32 as usize;
_5.4 = _8.1.4 & _8.1.4;
_5 = _8.1;
_8.0.5 = (_5.5.0, _5.5.1, _5.5.2);
_5.5.2 = _8.1.5.2;
_6 = -_5.1;
_1 = _5.4 + _5.4;
Goto(bb3)
}
bb7 = {
_4 = _2;
_5.2 = _4 == _4;
_1 = _5.1 as usize;
_2 = _4;
_5.2 = !_5.5.1;
_7 = [3408032612_u32];
_5.4 = _1 + _1;
_5.0 = [250629689285174015802518112336394518012_u128,113661569834173800661901352800063138947_u128,322315099984878959851671816268435745832_u128,64748319857906238257390103447663247719_u128,67870648396223680267543817251889114971_u128];
_5.5 = (10605_i16, _5.2, _3);
_5.1 = 23_i8 as f32;
Goto(bb2)
}
bb8 = {
_8.0.2 = !_10.2;
_8.0.4 = _8.1.4 ^ _5.4;
_11.fld0.fld0 = !_8.0.4;
_8 = (_5, _5);
_8.1.3 = _10.1 as usize;
_9 = 715436667_i32 as f64;
_8.0.4 = _4 as usize;
_12.fld3.1 = _2;
_4 = _2;
_5.3 = 47228_u16 as usize;
_11.fld0.fld4 = 319767318422962873988765482843517602181_u128 ^ 189779264113376511352031404984743213279_u128;
_10.2 = !_8.0.5.1;
_8.1 = _8.0;
_12.fld1 = _6 * _5.1;
_3 = _8.1.5.2 ^ _5.5.2;
_12.fld3.0 = _10.3 as u64;
_10.0 = _10.2;
_16.0.5.1 = _10.3;
_16.0.2 = core::ptr::addr_of!(_16.0.5.0.2);
_16.0.3.2 = !_5.2;
_16.4.1 = _2;
_5.5.2 = _9 as u64;
_8.1.3 = !_1;
match _5.5.0 {
340282366920938463463374607431768197354 => bb9,
_ => bb5
}
}
bb9 = {
_5.5 = (_8.0.5.0, _8.0.5.1, _12.fld3.0);
_16.3 = core::ptr::addr_of_mut!(_16.0.5.0.3.1);
_16.0.3.5.1 = _10.0;
_10.3 = _16.0.5.1;
_16.2 = _11.fld0.fld4 as u64;
_16.0.5.0.3.1 = _12.fld1;
_9 = (-121_i8) as f64;
_16.0.3.4 = _4 as usize;
_16.0.5.1 = !_10.3;
_8.1 = (_8.0.0, _16.0.5.0.3.1, _8.0.5.1, _1, _1, _5.5);
_16.0.3.5.1 = _5.5.1;
_16.0.3.5.0 = -_5.5.0;
_5.1 = _6;
_16.0.1.0.4 = !_10.0;
_10.1 = !_10.3;
_16.0.5.0.2 = (-5_isize) as i8;
_16.0.5.1 = _4 as u32;
_8.0.4 = !_1;
_16.0.3.0 = [_11.fld0.fld4,_11.fld0.fld4,_11.fld0.fld4,_11.fld0.fld4,_11.fld0.fld4];
_16.0.5.0.3 = (_16.0.3.5.0, _8.1.1);
_23 = _9;
_4 = _2;
_16.0.3.5.1 = _16.0.3.2;
_10.1 = _8.0.5.0 as u32;
_16.0.5.0.0 = _5.0;
Goto(bb10)
}
bb10 = {
_22 = _16.0.1.0.4 as i64;
_5.5.2 = _12.fld3.0 - _12.fld3.0;
_16.0.3 = _5;
_16.0.1.0.1 = _5.5.1 as i16;
_10.2 = _16.0.5.1 != _10.3;
_11.fld0.fld2 = (-9223372036854775808_isize) ^ (-93_isize);
_12.fld1 = _22 as f32;
match _8.0.5.0 {
0 => bb6,
1 => bb9,
2 => bb3,
3 => bb11,
4 => bb12,
5 => bb13,
340282366920938463463374607431768197354 => bb15,
_ => bb14
}
}
bb11 = {
_9 = _1 as f64;
_8.1.5.2 = !_8.0.5.2;
_10 = (_5.5.1, 1210636974_u32, _5.2, 12912034_u32);
_9 = _5.5.0 as f64;
_8.1.4 = !_1;
RET = _5.5.2 as i128;
_8.0.0 = _5.0;
_2 = _4;
_5.4 = _1;
_8.0.3 = _1;
Call(_10.3 = fn1(_8.0.3, _8.1.5, _1, _8.0.0, _5.5.0, _8.0.5.0, _1, _10.1, _10.1, _5.5, _8.1.4, _8.1.0, _8.1, _10.1), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_9 = _1 as f64;
_8.1.5.2 = !_8.0.5.2;
_10 = (_5.5.1, 1210636974_u32, _5.2, 12912034_u32);
_9 = _5.5.0 as f64;
_8.1.4 = !_1;
RET = _5.5.2 as i128;
_8.0.0 = _5.0;
_2 = _4;
_5.4 = _1;
_8.0.3 = _1;
Call(_10.3 = fn1(_8.0.3, _8.1.5, _1, _8.0.0, _5.5.0, _8.0.5.0, _1, _10.1, _10.1, _5.5, _8.1.4, _8.1.0, _8.1, _10.1), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
_4 = _2;
_5.2 = _4 == _4;
_1 = _5.1 as usize;
_2 = _4;
_5.2 = !_5.5.1;
_7 = [3408032612_u32];
_5.4 = _1 + _1;
_5.0 = [250629689285174015802518112336394518012_u128,113661569834173800661901352800063138947_u128,322315099984878959851671816268435745832_u128,64748319857906238257390103447663247719_u128,67870648396223680267543817251889114971_u128];
_5.5 = (10605_i16, _5.2, _3);
_5.1 = 23_i8 as f32;
Goto(bb2)
}
bb14 = {
_8.0.5.2 = _5.5.2 << _8.0.3;
_3 = !_8.0.5.2;
_10.2 = _5.2;
_5.1 = -_6;
_9 = _10.3 as f64;
_11.fld0.fld1 = [_10.3];
_8.1.5.1 = !_5.2;
_4 = _2;
match _5.5.0 {
0 => bb3,
1 => bb5,
2 => bb6,
340282366920938463463374607431768197354 => bb8,
_ => bb7
}
}
bb15 = {
_16.0.3.5.0 = !_16.0.1.0.1;
_22 = -873457150525610718_i64;
_12.fld3.0 = _8.1.5.2;
_8.1.2 = _5.2;
_5.4 = _11.fld0.fld0;
_16.0.3.5 = _8.0.5;
_15.0 = !_12.fld3.0;
_16.0.0 = _16.0.5.0.2;
_12.fld0 = _10.3 as u128;
_16.0.1.0.3.0 = (-1620026828_i32) as i16;
_8.1.5 = _5.5;
_8.1.4 = !_16.0.3.4;
_2 = _12.fld3.1;
_14 = !_15.0;
_8.1 = (_5.0, _16.0.5.0.3.1, _16.0.3.5.1, _5.4, _11.fld0.fld0, _5.5);
_12.fld3.0 = _12.fld3.1 as u64;
_16.0.5.0.4 = _5.2;
_19 = _16.0.5.0.2;
_16.0.4 = [_22,_22,_22,_22,_22];
_9 = _23;
_24 = [_10.3];
_26 = -_11.fld0.fld2;
_8.1.2 = !_8.1.5.1;
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(0_usize, 3_usize, Move(_3), 24_usize, Move(_24), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(0_usize, 10_usize, Move(_10), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: usize,mut _2: (i16, bool, u64),mut _3: usize,mut _4: [u128; 5],mut _5: i16,mut _6: i16,mut _7: usize,mut _8: u32,mut _9: u32,mut _10: (i16, bool, u64),mut _11: usize,mut _12: [u128; 5],mut _13: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)),mut _14: u32) -> u32 {
mir! {
type RET = u32;
let _15: char;
let _16: (i16, f32);
let _17: usize;
let _18: f32;
let _19: [bool; 7];
let _20: i128;
let _21: char;
let _22: *mut ([u128; 5], i16, i8, (i16, f32), bool);
let _23: f64;
let _24: f64;
let _25: i128;
let _26: isize;
let _27: *mut *const (u64, char);
let _28: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _29: i64;
let _30: [i32; 1];
let _31: u64;
let _32: ();
let _33: ();
{
_10 = _2;
_2.0 = _2.2 as i16;
RET = !_8;
_5 = -_13.5.0;
_1 = _8 as usize;
Call(_2.2 = fn2(_12, _6, _10.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _9;
_12 = [108188965800336397556304120339860076495_u128,317146853438002184008998957748254768939_u128,168649121595033404054934408502267431424_u128,255915018710898176005813328622654294232_u128,170557570388485754029101689766869287690_u128];
_2.2 = _13.5.2;
match _13.5.0 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768197354 => bb6,
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
_13.2 = !_10.1;
_3 = _13.4;
_13.5.2 = _2.2;
_13.5 = (_10.0, _13.2, _2.2);
_13.0 = [118421600795374570055761205229596330342_u128,161164838086867750039677424224418096667_u128,182773303287806380885202790548982717785_u128,165605766713242347092204132365123131983_u128,178434504979429579772825113354681552373_u128];
_13.5.0 = _5 | _6;
_8 = _9;
_17 = _3 & _13.3;
_14 = !RET;
_5 = _2.0 ^ _2.0;
_15 = '\u{e3385}';
_5 = -_10.0;
_21 = _15;
_18 = _13.1 - _13.1;
_1 = _17;
_18 = _13.1;
Call(_13.5.0 = fn3(_2, _2.2, _10.1, _8, _9), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Goto(bb8)
}
bb8 = {
_5 = _13.5.0 ^ _13.5.0;
_14 = (-22_i8) as u32;
_14 = !RET;
_13.5.2 = !_2.2;
_1 = _13.4;
_16 = (_13.5.0, _13.1);
_19 = [_13.5.1,_13.5.1,_10.1,_13.2,_2.1,_13.5.1,_10.1];
Goto(bb9)
}
bb9 = {
_16.1 = _18 + _18;
_20 = -88007412715927146454851470384038318591_i128;
_16 = (_13.5.0, _18);
_25 = !_20;
_17 = _13.3;
_5 = _13.5.0;
_24 = _1 as f64;
_19 = [_13.5.1,_13.5.1,_13.2,_13.2,_13.5.1,_10.1,_2.1];
RET = _8;
_11 = (-40_i8) as usize;
_23 = -_24;
_12 = [153910722841956155924669983413361209162_u128,28705294683185614350392950403045889312_u128,332942852877189163928630739628852861131_u128,237238206430019767678415948565033410560_u128,98371132550142534741906373418851119476_u128];
_12 = [259580325478484525705318887584137157545_u128,90875176903552947067461014923227611130_u128,286916723604777833761125244910817655819_u128,339390882345504271515663863400016528918_u128,289433702555413250117861774383611294490_u128];
_25 = 226163808378448578300859362472809094031_u128 as i128;
_15 = _21;
_19 = [_13.5.1,_10.1,_13.2,_10.1,_13.5.1,_2.1,_10.1];
_13.4 = !_1;
_13.0 = [29339856052913710244270341148650392999_u128,282301584434537510269844943103004277646_u128,190272644467568222657208949436099704471_u128,184899385731111864260823313934732502344_u128,284895524125049446992253466442976460823_u128];
_20 = !_25;
_23 = _13.5.2 as f64;
_15 = _21;
_23 = -_24;
_16.1 = _24 as f32;
_4 = [286643887753360564583701014006870971180_u128,8446652521099139750248258694284007303_u128,261257215425157652841649484791277332448_u128,136270026256705320970493860527511127935_u128,21312826141134332659713860187824532712_u128];
_25 = _20 + _20;
Goto(bb10)
}
bb10 = {
_13.0 = _12;
_1 = _2.2 as usize;
_1 = _3 - _3;
_23 = _24 + _24;
_2.2 = _13.5.2;
_8 = _9 >> _2.0;
_11 = !_3;
_10.2 = !_2.2;
_28.0.1 = _16.1;
_29 = 6278402262267766839_i64;
RET = _8 | _8;
_26 = -(-9223372036854775808_isize);
_2.0 = _13.5.0;
_28.0.5.1 = !_2.1;
_16.1 = _14 as f32;
_2.1 = _28.0.5.1;
_5 = !_2.0;
_2.2 = _13.5.2 * _10.2;
_28.1.3 = _29 as usize;
_19 = [_28.0.5.1,_28.0.5.1,_28.0.5.1,_2.1,_28.0.5.1,_13.5.1,_2.1];
_28.1.4 = _28.1.3;
_2 = (_16.0, _28.0.5.1, _13.5.2);
_21 = _15;
_26 = (-9223372036854775808_isize);
_31 = _10.2;
_28.0.5 = (_13.5.0, _13.2, _2.2);
_26 = 9223372036854775807_isize & 9223372036854775807_isize;
Goto(bb11)
}
bb11 = {
Call(_32 = dump_var(1_usize, 15_usize, Move(_15), 7_usize, Move(_7), 1_usize, Move(_1), 19_usize, Move(_19)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_32 = dump_var(1_usize, 2_usize, Move(_2), 5_usize, Move(_5), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_32 = dump_var(1_usize, 31_usize, Move(_31), 20_usize, Move(_20), 8_usize, Move(_8), 33_usize, _33), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [u128; 5],mut _2: i16,mut _3: bool) -> u64 {
mir! {
type RET = u64;
let _4: u128;
let _5: char;
let _6: i32;
let _7: (u64, char);
let _8: (bool, u32, bool, u32);
let _9: [bool; 7];
let _10: char;
let _11: bool;
let _12: (i16, f32);
let _13: (u64, char);
let _14: ();
let _15: ();
{
RET = !4336602964568526752_u64;
RET = !6045914285677635631_u64;
_1 = [33020053068330737439608207397763545488_u128,122338793489275874129884393404213641379_u128,229062863460035066530099855370827471443_u128,53095335974579963548582144193602721481_u128,286151378964674687689832242779899255725_u128];
_4 = 20705_u16 as u128;
_3 = true;
_5 = '\u{1d57e}';
RET = 16755675926175848097_u64;
RET = _3 as u64;
_2 = 11762_i16;
_6 = -(-1487014516_i32);
_1 = [_4,_4,_4,_4,_4];
_6 = RET as i32;
RET = !18422193817932293607_u64;
_3 = _6 < _6;
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
11762 => bb9,
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
_8.2 = _6 != _6;
_8.3 = 2695332712_u32;
_7.1 = _5;
RET = 130857360576016033_u64 | 9091768668543009928_u64;
match _2 {
0 => bb10,
1 => bb11,
2 => bb12,
11762 => bb14,
_ => bb13
}
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
_7.1 = _5;
_7.0 = RET << _8.3;
_8 = (_3, 2024545727_u32, _3, 1712746435_u32);
_9 = [_8.2,_8.2,_3,_3,_3,_3,_8.0];
_4 = 73335916038798916631983355381666505538_u128;
_8.2 = _8.0;
_10 = _5;
_7.0 = RET >> _8.3;
_6 = (-635810349_i32);
_2 = 64519_u16 as i16;
_12.1 = 1662521552640878082_i64 as f32;
_8 = (_3, 651382791_u32, _3, 91415179_u32);
_11 = !_8.0;
_9 = [_3,_8.2,_8.2,_3,_8.2,_11,_3];
_8.3 = _8.1 ^ _8.1;
_13.0 = RET;
_8.2 = _8.0 & _3;
_12.0 = _2;
_7.0 = _13.0 | _13.0;
Goto(bb15)
}
bb15 = {
Call(_14 = dump_var(2_usize, 2_usize, Move(_2), 3_usize, Move(_3), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_14 = dump_var(2_usize, 4_usize, Move(_4), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: (i16, bool, u64),mut _2: u64,mut _3: bool,mut _4: u32,mut _5: u32) -> i16 {
mir! {
type RET = i16;
let _6: u64;
let _7: isize;
let _8: [u128; 5];
let _9: f64;
let _10: u16;
let _11: isize;
let _12: Adt57;
let _13: i32;
let _14: Adt49;
let _15: Adt53;
let _16: f32;
let _17: [u32; 3];
let _18: f32;
let _19: [u32; 3];
let _20: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _21: f32;
let _22: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _23: char;
let _24: (bool, [i64; 5], u128);
let _25: usize;
let _26: (usize, i16, i64);
let _27: &'static i128;
let _28: [bool; 7];
let _29: (usize, i16, i64);
let _30: isize;
let _31: ();
let _32: ();
{
_5 = !_4;
_6 = 7478967578361076892_i64 as u64;
_5 = !_4;
_3 = !_1.1;
_7 = !(-9223372036854775808_isize);
RET = _1.0 + _1.0;
_1.2 = _2 + _2;
_4 = _5;
_4 = _5 * _5;
RET = -_1.0;
_6 = !_2;
_1 = (RET, _3, _2);
RET = _1.0;
_1.1 = _5 < _5;
RET = -_1.0;
_1.0 = RET << _5;
_1.0 = RET & RET;
_1.0 = RET;
_1 = (RET, _3, _6);
_3 = _1.1;
_3 = !_1.1;
_1.0 = !RET;
Call(_7 = fn4(_4, _1.1, _4, _4, _4, _1, _3, _1.1, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [54483071584717628630610735987545489076_u128,121580057869242264274921346009397473937_u128,99415446333415346150357220154085816253_u128,23860614097014852349084409539218672810_u128,92191441257973291937151604346383682512_u128];
_4 = 82268906643879775866459811694672593485_i128 as u32;
Goto(bb2)
}
bb2 = {
_5 = !_4;
_1.2 = 22465_u16 as u64;
RET = (-2934716838418019325_i64) as i16;
_5 = _4 + _4;
Goto(bb3)
}
bb3 = {
_1.1 = _3 & _3;
RET = !_1.0;
_2 = _5 as u64;
_9 = RET as f64;
_9 = _5 as f64;
RET = 160700089145974581334746457061460539771_u128 as i16;
_6 = _2;
_6 = _2 + _2;
_4 = _5;
_4 = _5;
_11 = _7;
_6 = _2 * _1.2;
_6 = 5_usize as u64;
_4 = !_5;
_10 = 56917_u16 + 61085_u16;
_4 = _5;
_11 = !_7;
_11 = _7;
_10 = 9171_u16 & 55124_u16;
_1 = (RET, _3, _2);
_10 = 19393_u16;
_13 = (-880236220_i32) << _11;
_1.0 = 150715423411642273890302011484873434852_i128 as i16;
match _10 {
0 => bb1,
19393 => bb4,
_ => bb2
}
}
bb4 = {
RET = _1.0 * _1.0;
_12 = Adt57::Variant0 { fld0: _5 };
_5 = _7 as u32;
_9 = 20971102772866093824841489360182276328_i128 as f64;
_1.2 = _6;
_1.1 = _3;
_8 = [22877173216338528522698799628961410567_u128,16230218003558978486270996003640164446_u128,197458925708019805573517427187523602308_u128,44776540614030930777465339813055493116_u128,104442349680491067493897160779988143951_u128];
_3 = !_1.1;
SetDiscriminant(_12, 0);
_1.2 = !_2;
_11 = _7 - _7;
RET = 181325389113060575784258137074090044122_u128 as i16;
place!(Field::<u32>(Variant(_12, 0), 0)) = _5 >> _5;
_10 = _13 as u16;
_10 = RET as u16;
Goto(bb5)
}
bb5 = {
_12 = Adt57::Variant0 { fld0: _5 };
_2 = (-11_i8) as u64;
place!(Field::<u32>(Variant(_12, 0), 0)) = _5 << _4;
_5 = !Field::<u32>(Variant(_12, 0), 0);
_8 = [289309304844380125884498226379438340351_u128,167220192455130608679503193295094979370_u128,160476103858773746004473602521230057520_u128,238421098088337554484689284756890373280_u128,7941317068448926155233100352710716950_u128];
place!(Field::<u32>(Variant(_12, 0), 0)) = !_5;
_1.2 = _13 as u64;
_11 = _7 + _7;
_6 = _1.2 << _11;
_11 = _7;
Goto(bb6)
}
bb6 = {
_1.0 = 18993534786465148869924385225103311335_u128 as i16;
_14 = Adt49::Variant1 { fld0: Field::<u32>(Variant(_12, 0), 0) };
Call(_1.0 = core::intrinsics::transmute(_10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
place!(Field::<u32>(Variant(_12, 0), 0)) = _13 as u32;
place!(Field::<u32>(Variant(_12, 0), 0)) = Field::<u32>(Variant(_14, 1), 0);
_4 = !Field::<u32>(Variant(_12, 0), 0);
_11 = _7;
_4 = (-6906566019906447507_i64) as u32;
_11 = _7 * _7;
_8 = [70338576303132098444971623306658056842_u128,264436213919285623405193428791434842650_u128,42090091306601673968743587439396257429_u128,61659571982243216886037030401434304788_u128,334430538648909000531577394432249019192_u128];
_9 = 5428387923775672151_i64 as f64;
RET = (-76458757164192406536307057314555443796_i128) as i16;
_1 = (RET, _3, _6);
_13 = -1653620828_i32;
_6 = !_1.2;
_5 = !Field::<u32>(Variant(_14, 1), 0);
RET = _1.0 & _1.0;
_1.2 = _10 as u64;
Goto(bb8)
}
bb8 = {
_9 = _6 as f64;
SetDiscriminant(_12, 1);
_1.0 = _1.1 as i16;
SetDiscriminant(_14, 1);
_1.1 = _5 != _5;
_1 = (RET, _3, _6);
_4 = (-4432461870826829657712196793434910451_i128) as u32;
_17 = [_5,_5,_5];
_3 = !_1.1;
_9 = 13988560380979939215187424462608651709_i128 as f64;
_11 = !_7;
place!(Field::<i32>(Variant(_12, 1), 0)) = _7 as i32;
_10 = 38642_u16 - 28705_u16;
_13 = 56_i8 as i32;
_14 = Adt49::Variant1 { fld0: _5 };
_14 = Adt49::Variant1 { fld0: _5 };
_3 = _1.0 <= RET;
RET = _1.0;
Call(_4 = core::intrinsics::bswap(Field::<u32>(Variant(_14, 1), 0)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1.0 = RET;
_7 = _11;
SetDiscriminant(_14, 0);
_10 = _6 as u16;
SetDiscriminant(_12, 2);
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).3.0 = [24392896202857205308953883368214829723_u128,31392521182612468293046884471779922312_u128,177341106597272784740364171092554031732_u128,267477921129232704851093843291758658400_u128,288924113488295190868901642574003903741_u128];
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).3.5.2 = _1.2 ^ _6;
place!(Field::<(usize, i16, i64)>(Variant(_14, 0), 3)).1 = _1.0;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).1.0.1 = _4 as i16;
place!(Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5)).3 = _5;
_1.0 = RET;
_12 = Adt57::Variant0 { fld0: _5 };
_1 = (Field::<(usize, i16, i64)>(Variant(_14, 0), 3).1, _3, _6);
place!(Field::<isize>(Variant(_14, 0), 2)) = _1.0 as isize;
_1.0 = Field::<(usize, i16, i64)>(Variant(_14, 0), 3).1 << _7;
_6 = _1.2 - _1.2;
SetDiscriminant(_12, 2);
_22.1.2 = _3;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).4 = [6591180996956625931_i64,(-4321117110915717170_i64),5495718671036144734_i64,8581183797434155321_i64,2589693653584168949_i64];
place!(Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5)).0 = _22.1.2 ^ _3;
_23 = '\u{9cec3}';
Goto(bb10)
}
bb10 = {
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).3.3 = _10 as usize;
place!(Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5)).3 = _23 as u32;
_14 = Adt49::Variant1 { fld0: _5 };
_22.0.5.2 = _23 as u64;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).5.0.3.0 = _13 as i16;
place!(Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5)).1 = _5;
_22.1.4 = Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).3.3 & Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).3.3;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).5.0.2 = _9 as i8;
_16 = (-10184521855601126950610710711886088990_i128) as f32;
Goto(bb11)
}
bb11 = {
_20.2 = Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).0;
_24.2 = _9 as u128;
_17 = [_5,Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).1,Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).1];
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).3.4 = !_22.1.4;
_22.1.4 = Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).3.4 * Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).3.3;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).0 = Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).5.0.2 >> Field::<u32>(Variant(_14, 1), 0);
_22.1.5.1 = _22.1.2 ^ Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).0;
_22.1.1 = _9 as f32;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).5.0.3.0 = _6 as i16;
_20.0 = _8;
_28 = [Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).0,_22.1.5.1,_20.2,_22.1.5.1,_20.2,_22.1.5.1,_20.2];
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).5.0.4 = _3 ^ Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).0;
place!(Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5)).3 = Field::<u32>(Variant(_14, 1), 0) ^ Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).1;
Call(_29.1 = core::intrinsics::bswap(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).5.0.3.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22.0.5 = (Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).5.0.3.0, Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).5.0.4, _1.2);
_3 = _22.1.2;
_5 = Field::<(bool, u32, bool, u32)>(Variant(_12, 2), 5).3;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4)).1.0.1 = !Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).5.0.3.0;
_24.0 = !_22.1.2;
_22.1.5.0 = (-152015485086135188671966434602173200153_i128) as i16;
RET = Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_12, 2), 4).1.0.1 + _22.0.5.0;
_22.0.0 = [_24.2,_24.2,_24.2,_24.2,_24.2];
_22.1.0 = _8;
Goto(bb13)
}
bb13 = {
Call(_31 = dump_var(3_usize, 2_usize, Move(_2), 1_usize, Move(_1), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_31 = dump_var(3_usize, 13_usize, Move(_13), 28_usize, Move(_28), 3_usize, Move(_3), 32_usize, _32), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u32,mut _2: bool,mut _3: u32,mut _4: u32,mut _5: u32,mut _6: (i16, bool, u64),mut _7: bool,mut _8: bool,mut _9: u64) -> isize {
mir! {
type RET = isize;
let _10: i128;
let _11: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _12: f64;
let _13: *const i8;
let _14: &'static i128;
let _15: char;
let _16: u32;
let _17: (i16, bool, u64);
let _18: i64;
let _19: Adt60;
let _20: i32;
let _21: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _22: u32;
let _23: i128;
let _24: bool;
let _25: u32;
let _26: bool;
let _27: ();
let _28: ();
{
_6.1 = _1 > _5;
_1 = _5;
RET = _1 as isize;
RET = (-9223372036854775808_isize) + 9223372036854775807_isize;
_11.0.1 = _6.0 << _9;
_11.0.0 = [208552460795297055975600003607435985212_u128,54274590149569067702065379728296765759_u128,162555071115615624019319406916292491478_u128,6545995485721934331570381695498641919_u128,27928779482405470086533708758151858393_u128];
_9 = _6.1 as u64;
_9 = _6.0 as u64;
_6 = (_11.0.1, _2, _9);
_7 = _2 <= _8;
_10 = 112293550339064594873489718728126894774_i128 << _11.0.1;
_11.0.4 = !_6.1;
_3 = _9 as u32;
_11.0.1 = -_6.0;
_9 = 8_i8 as u64;
_5 = _10 as u32;
Call(_7 = fn5(_6.0, RET, _4, _11.0.4, _2, _6.0, _4, _3, _5, _6.1, _6, _5, _6.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.2 = _9 * _9;
_6.1 = _7;
_9 = !_6.2;
_6 = (_11.0.1, _7, _9);
_5 = 1227754836_i32 as u32;
_5 = _1;
_11.0.3.1 = (-87_i8) as f32;
_3 = 2_usize as u32;
_6.2 = _6.0 as u64;
_11.0.4 = !_7;
_11.0.2 = (-37_i8) & 64_i8;
_6.0 = 174709123102183750494912933456466702767_u128 as i16;
_11.0.0 = [214929561552896442295008577218679410443_u128,92014974998813869604030772666685756495_u128,109145698865926227497073616306252682790_u128,180918965034160271668498154524327250707_u128,202965002636818205693782400146905549880_u128];
_11.1 = 100166051751665554140648833820343413916_u128 as u32;
_8 = !_6.1;
_2 = _7 <= _8;
_14 = &_10;
_11.0.3.1 = (-403485534_i32) as f32;
_6.0 = _11.0.1;
_11.0.0 = [229808875311496847385731367509058171536_u128,30075768950446702140266507576239364982_u128,173213152763273549130742132122571808174_u128,200182962845783112872260270008245947039_u128,149701847440592473833671684366824623903_u128];
_8 = _11.0.4;
_11.0.4 = _7 ^ _2;
_6.2 = !_9;
_17.1 = _7 <= _11.0.4;
Call(_11.0.2 = fn7(_5, _17.1, (*_14), _6, _7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = core::ptr::addr_of!(_11.0.2);
_11.0.1 = -_6.0;
_11.0.0 = [289672925929640445787921253574624742056_u128,309311403447043043370394230004647364369_u128,44864429690868782929249381457531588650_u128,168523929460822125891143472185872341574_u128,72045883594689618955479513182818659257_u128];
RET = 115_isize;
_11.0.0 = [112649427548090832388151735398386417043_u128,297181755896362944961354359355009134759_u128,200258234715182662317097505861082366502_u128,5215238110613684641453040534343765998_u128,57402661574611766530299041325882639725_u128];
_11.0.0 = [202426561187206835718309617362442594323_u128,334294732512110145643708075642513987105_u128,234473343786116260827684296563479618983_u128,113778820073361382054149301440820605818_u128,172726902506160887546673360474856339766_u128];
_8 = !_17.1;
_11.1 = _1;
_8 = !_17.1;
_13 = core::ptr::addr_of!((*_13));
_12 = 97_u8 as f64;
_17.1 = _2 == _6.1;
_4 = !_5;
_6.1 = _8 != _8;
RET = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_6.0 = !_11.0.1;
_17.0 = _6.0;
_6.0 = _11.0.1;
_9 = !_6.2;
_9 = !_6.2;
_1 = _5 * _11.1;
RET = !9223372036854775807_isize;
_1 = 44188321024077187745657806615337649508_u128 as u32;
Call(_11.0.2 = core::intrinsics::bswap((-83_i8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = '\u{5f49}';
_18 = (-4469549980006221134_i64) << (*_13);
_17.2 = _15 as u64;
Call((*_13) = core::intrinsics::transmute(_6.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_13) = !22_i8;
_16 = RET as u32;
_11.1 = !_4;
_2 = !_7;
_6.0 = _11.0.1;
RET = (-110_isize);
RET = 9223372036854775807_isize;
_15 = '\u{d2b1}';
_12 = RET as f64;
_11.0.4 = _2;
_11.1 = _4;
RET = (-7_isize) * 9223372036854775807_isize;
_11.0.4 = !_6.1;
_3 = _12 as u32;
_11.0.3.1 = 278312093_i32 as f32;
RET = 65602865744544743220520687610851752030_u128 as isize;
_11.0.3.0 = _17.0;
_13 = core::ptr::addr_of!(_11.0.2);
_11.1 = !_5;
Goto(bb5)
}
bb5 = {
_11.0.1 = _15 as i16;
_9 = 41175_u16 as u64;
_21.2 = _8 | _7;
_21.5.1 = !_17.1;
Goto(bb6)
}
bb6 = {
_21 = (_11.0.0, _11.0.3.1, _11.0.4, 6_usize, 6_usize, _6);
_17 = (_6.0, _8, _21.5.2);
_11.0.3.0 = _6.0 << _17.0;
_21.2 = _8 | _2;
match _21.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb9,
_ => bb8
}
}
bb7 = {
_13 = core::ptr::addr_of!(_11.0.2);
_11.0.1 = -_6.0;
_11.0.0 = [289672925929640445787921253574624742056_u128,309311403447043043370394230004647364369_u128,44864429690868782929249381457531588650_u128,168523929460822125891143472185872341574_u128,72045883594689618955479513182818659257_u128];
RET = 115_isize;
_11.0.0 = [112649427548090832388151735398386417043_u128,297181755896362944961354359355009134759_u128,200258234715182662317097505861082366502_u128,5215238110613684641453040534343765998_u128,57402661574611766530299041325882639725_u128];
_11.0.0 = [202426561187206835718309617362442594323_u128,334294732512110145643708075642513987105_u128,234473343786116260827684296563479618983_u128,113778820073361382054149301440820605818_u128,172726902506160887546673360474856339766_u128];
_8 = !_17.1;
_11.1 = _1;
_8 = !_17.1;
_13 = core::ptr::addr_of!((*_13));
_12 = 97_u8 as f64;
_17.1 = _2 == _6.1;
_4 = !_5;
_6.1 = _8 != _8;
RET = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_6.0 = !_11.0.1;
_17.0 = _6.0;
_6.0 = _11.0.1;
_9 = !_6.2;
_9 = !_6.2;
_1 = _5 * _11.1;
RET = !9223372036854775807_isize;
_1 = 44188321024077187745657806615337649508_u128 as u32;
Call(_11.0.2 = core::intrinsics::bswap((-83_i8)), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_6.2 = _9 * _9;
_6.1 = _7;
_9 = !_6.2;
_6 = (_11.0.1, _7, _9);
_5 = 1227754836_i32 as u32;
_5 = _1;
_11.0.3.1 = (-87_i8) as f32;
_3 = 2_usize as u32;
_6.2 = _6.0 as u64;
_11.0.4 = !_7;
_11.0.2 = (-37_i8) & 64_i8;
_6.0 = 174709123102183750494912933456466702767_u128 as i16;
_11.0.0 = [214929561552896442295008577218679410443_u128,92014974998813869604030772666685756495_u128,109145698865926227497073616306252682790_u128,180918965034160271668498154524327250707_u128,202965002636818205693782400146905549880_u128];
_11.1 = 100166051751665554140648833820343413916_u128 as u32;
_8 = !_6.1;
_2 = _7 <= _8;
_14 = &_10;
_11.0.3.1 = (-403485534_i32) as f32;
_6.0 = _11.0.1;
_11.0.0 = [229808875311496847385731367509058171536_u128,30075768950446702140266507576239364982_u128,173213152763273549130742132122571808174_u128,200182962845783112872260270008245947039_u128,149701847440592473833671684366824623903_u128];
_8 = _11.0.4;
_11.0.4 = _7 ^ _2;
_6.2 = !_9;
_17.1 = _7 <= _11.0.4;
Call(_11.0.2 = fn7(_5, _17.1, (*_14), _6, _7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_23 = _10 ^ (*_14);
_21.5 = (_17.0, _2, _6.2);
Goto(bb10)
}
bb10 = {
_8 = _11.0.4;
_10 = _23 << _21.3;
_21.5.2 = _4 as u64;
_7 = _2;
_3 = _21.5.2 as u32;
_20 = (-345724196_i32);
_11.0.2 = (-64_i8) & (-37_i8);
_11.0.1 = _11.0.3.0 ^ _11.0.3.0;
_21.5.1 = _21.4 <= _21.3;
_5 = !_3;
_21.5.0 = _11.0.1;
_3 = _11.1 & _11.1;
RET = 54913_u16 as isize;
_14 = &_10;
_15 = '\u{70204}';
_6 = (_11.0.1, _2, _21.5.2);
_23 = (*_14) - (*_14);
_7 = !_21.5.1;
_8 = !_6.1;
_10 = _23;
_1 = RET as u32;
_8 = _17.1 ^ _21.5.1;
Goto(bb11)
}
bb11 = {
_11.0.4 = _21.2 < _17.1;
_21.1 = _11.0.3.1 + _11.0.3.1;
RET = _11.1 as isize;
_7 = _6.1;
_17.0 = _23 as i16;
_11.0.0 = [124334104111038818559444443146856767412_u128,273934367118105704582112363971591370055_u128,81351346844945987384169122184934663645_u128,337205201358147669857409874039206184667_u128,235983365704419862387593156056383725810_u128];
_11.0.4 = _2 & _6.1;
_25 = _11.1;
_16 = _4 - _3;
Goto(bb12)
}
bb12 = {
Call(_27 = dump_var(4_usize, 3_usize, Move(_3), 10_usize, Move(_10), 7_usize, Move(_7), 23_usize, Move(_23)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_27 = dump_var(4_usize, 6_usize, Move(_6), 16_usize, Move(_16), 8_usize, Move(_8), 17_usize, Move(_17)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i16,mut _2: isize,mut _3: u32,mut _4: bool,mut _5: bool,mut _6: i16,mut _7: u32,mut _8: u32,mut _9: u32,mut _10: bool,mut _11: (i16, bool, u64),mut _12: u32,mut _13: i16) -> bool {
mir! {
type RET = bool;
let _14: Adt58;
let _15: (u64, char);
let _16: u128;
let _17: *mut f32;
let _18: (i16, f32);
let _19: (bool, u32, bool, u32);
let _20: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _21: isize;
let _22: ();
let _23: ();
{
_11 = (_1, _4, 5341751753363353080_u64);
_11.1 = !_10;
_12 = 46772_u16 as u32;
_13 = _1;
RET = _5;
_11.0 = -_1;
_11.0 = _1;
_9 = _3 - _7;
_11.0 = !_1;
_8 = '\u{39a4b}' as u32;
_7 = !_3;
_11 = (_13, _10, 1031839480988512532_u64);
_6 = -_1;
match _11.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1031839480988512532 => bb7,
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
_15.0 = '\u{be6b8}' as u64;
_14.fld0 = 23729171213523502419439313018040922772_u128;
RET = !_4;
_11.2 = _15.0 & _15.0;
_15.1 = '\u{b1496}';
_8 = !_9;
_8 = _3;
_5 = RET;
RET = _10;
_15.0 = _11.2;
_15.1 = '\u{cb637}';
_3 = !_9;
_10 = _9 == _9;
_5 = _10;
_16 = _14.fld0 << _7;
_16 = _14.fld0;
Call(_14.fld3.0 = fn6(_10, _3, _7, _5, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15.1 = '\u{ddc26}';
_14.fld1 = _2 as f32;
Goto(bb9)
}
bb9 = {
_14.fld3.1 = _15.1;
_6 = _1 ^ _13;
_18 = (_6, _14.fld1);
_17 = core::ptr::addr_of_mut!(_14.fld1);
_1 = 6123865812529851082_i64 as i16;
Goto(bb10)
}
bb10 = {
RET = _5 >= _10;
_15 = _14.fld3;
_19 = (RET, _9, _10, _8);
_19 = (RET, _7, _10, _7);
_6 = _9 as i16;
_7 = _3 << _14.fld3.0;
_5 = _19.2 | RET;
(*_17) = _18.1;
_18.0 = 5_i8 as i16;
_14.fld0 = _16 | _16;
_19.2 = _7 <= _8;
_6 = (-82_i8) as i16;
_19 = (RET, _7, _5, _7);
_14.fld3.0 = _15.0;
(*_17) = _18.1 + _18.1;
_15.1 = _14.fld3.1;
_12 = (-108_i8) as u32;
Goto(bb11)
}
bb11 = {
Call(_22 = dump_var(5_usize, 6_usize, Move(_6), 9_usize, Move(_9), 16_usize, Move(_16), 3_usize, Move(_3)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_22 = dump_var(5_usize, 7_usize, Move(_7), 19_usize, Move(_19), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: u32,mut _3: u32,mut _4: bool,mut _5: i16) -> u64 {
mir! {
type RET = u64;
let _6: usize;
let _7: [i64; 5];
let _8: isize;
let _9: Adt58;
let _10: f32;
let _11: i128;
let _12: [i64; 5];
let _13: (bool, u32, bool, u32);
let _14: Adt57;
let _15: ();
let _16: ();
{
_6 = 0_usize;
_4 = !_1;
RET = (-166234761050992194106877710496064178639_i128) as u64;
_7 = [(-2289669990093720026_i64),2209985031034767459_i64,4530319246953478573_i64,(-2942358589379774505_i64),(-6096128536334364001_i64)];
_1 = _3 < _3;
_8 = !(-9223372036854775808_isize);
_2 = !_3;
_9.fld3.1 = '\u{e1ad1}';
_9.fld0 = 521861883343980222144728019612855931_i128 as u128;
_7[_6] = 1506213344334846371_i64 >> _3;
_9.fld3.0 = 612189856_i32 as u64;
RET = _9.fld3.0;
_9.fld0 = !204458089737024058840318922872198855858_u128;
_1 = !_4;
_8 = (-9223372036854775808_isize);
_3 = _2 | _2;
_13.0 = _4;
Goto(bb1)
}
bb1 = {
_13.2 = _13.0 < _1;
RET = _13.0 as u64;
_9.fld3 = (RET, '\u{41b1a}');
_12[_6] = _7[_6] | _7[_6];
_13 = (_1, _3, _4, _2);
_7 = [_12[_6],_12[_6],_12[_6],_12[_6],_12[_6]];
_14 = Adt57::Variant1 { fld0: (-1312671621_i32) };
place!(Field::<i32>(Variant(_14, 1), 0)) = _6 as i32;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(6_usize, 13_usize, Move(_13), 2_usize, Move(_2), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u32,mut _2: bool,mut _3: i128,mut _4: (i16, bool, u64),mut _5: bool,mut _6: bool) -> i8 {
mir! {
type RET = i8;
let _7: (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32));
let _8: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _9: f64;
let _10: i32;
let _11: f64;
let _12: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _13: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _14: (bool, u32, bool, u32);
let _15: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _16: i8;
let _17: (u64, char);
let _18: ();
let _19: ();
{
_4.1 = _2 == _5;
_3 = 12_i8 as i128;
_4.0 = 26157_i16 << _1;
_6 = _2;
_2 = _4.1 != _4.1;
_2 = !_5;
_4.1 = _5 >= _5;
_5 = _6 | _2;
_4.2 = '\u{ba927}' as u64;
_7.3.5.1 = _6 ^ _6;
_7.3.2 = !_2;
_7.1.0.2 = (-644833388_i32) as i8;
_7.5.0.3.0 = !_4.0;
_7.5.0.1 = _3 as i16;
_7.5.0.0 = [242258096746355697616573031716017585950_u128,75469910776883669711927024019487629888_u128,72850821796486036354762392247994892715_u128,93707120658343243750731123907880702363_u128,337351307074952644222667489445744657084_u128];
Goto(bb1)
}
bb1 = {
_7.2 = core::ptr::addr_of!(_7.1.0.2);
_7.5.0.2 = !_7.1.0.2;
_7.1.0.3.1 = (-1830946510335344324_i64) as f32;
_2 = _6;
_7.3.2 = _5;
_7.3.3 = 13983331289067492673_usize;
_7.0 = -_7.1.0.2;
RET = _7.1.0.2;
RET = _7.1.0.2 ^ _7.5.0.2;
RET = -_7.1.0.2;
_7.5.0.3.0 = _4.0 - _4.0;
_7.3.5.2 = _4.2;
_8.0.1 = -_4.0;
_7.3.4 = !_7.3.3;
_7.1.0.1 = !_7.5.0.3.0;
_4.1 = _7.3.5.1;
_7.3.1 = (-3318405047366686867_i64) as f32;
_7.5.0.0 = [178549268496095473737565146525927805551_u128,284134593384516658818056211320755178728_u128,267790349439926583406566437816064080912_u128,40414281380988756098150418173905168318_u128,314303687750962880492529438999301355830_u128];
_7.3.5 = (_7.5.0.3.0, _7.3.2, _4.2);
Goto(bb2)
}
bb2 = {
_7.3.0 = _7.5.0.0;
_7.1.1 = !_1;
_7.1.0.0 = [769813815567109422679692597964121098_u128,302530603327430685182315798858929573548_u128,41022435385086708017976387109155848558_u128,180787870866081360024691596967342228544_u128,56972668421159379260122100202341407335_u128];
_6 = _2;
_7.1.0.4 = _2;
_8.0.3 = (_7.3.5.0, _7.3.1);
_4.0 = _7.3.5.2 as i16;
_8.0.4 = _4.1;
_8.0.3 = (_7.3.5.0, _7.1.0.3.1);
_11 = 66765403756013992350798767892164886850_u128 as f64;
_7.5.0.3.1 = _8.0.3.1 - _8.0.3.1;
_7.1.0.3 = (_7.5.0.3.0, _7.5.0.3.1);
_7.3.5.0 = _11 as i16;
Call(_7 = fn8(_2, _4.1, _5, _6, _8.0.4, _8.0.4, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7.5.0.4 = !_7.1.0.4;
_12.3 = _7.1.0.3.1 as usize;
match _7.3.4 {
0 => bb1,
1 => bb2,
2 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
3 => bb10,
_ => bb9
}
}
bb4 = {
_7.3.0 = _7.5.0.0;
_7.1.1 = !_1;
_7.1.0.0 = [769813815567109422679692597964121098_u128,302530603327430685182315798858929573548_u128,41022435385086708017976387109155848558_u128,180787870866081360024691596967342228544_u128,56972668421159379260122100202341407335_u128];
_6 = _2;
_7.1.0.4 = _2;
_8.0.3 = (_7.3.5.0, _7.3.1);
_4.0 = _7.3.5.2 as i16;
_8.0.4 = _4.1;
_8.0.3 = (_7.3.5.0, _7.1.0.3.1);
_11 = 66765403756013992350798767892164886850_u128 as f64;
_7.5.0.3.1 = _8.0.3.1 - _8.0.3.1;
_7.1.0.3 = (_7.5.0.3.0, _7.5.0.3.1);
_7.3.5.0 = _11 as i16;
Call(_7 = fn8(_2, _4.1, _5, _6, _8.0.4, _8.0.4, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7.2 = core::ptr::addr_of!(_7.1.0.2);
_7.5.0.2 = !_7.1.0.2;
_7.1.0.3.1 = (-1830946510335344324_i64) as f32;
_2 = _6;
_7.3.2 = _5;
_7.3.3 = 13983331289067492673_usize;
_7.0 = -_7.1.0.2;
RET = _7.1.0.2;
RET = _7.1.0.2 ^ _7.5.0.2;
RET = -_7.1.0.2;
_7.5.0.3.0 = _4.0 - _4.0;
_7.3.5.2 = _4.2;
_8.0.1 = -_4.0;
_7.3.4 = !_7.3.3;
_7.1.0.1 = !_7.5.0.3.0;
_4.1 = _7.3.5.1;
_7.3.1 = (-3318405047366686867_i64) as f32;
_7.5.0.0 = [178549268496095473737565146525927805551_u128,284134593384516658818056211320755178728_u128,267790349439926583406566437816064080912_u128,40414281380988756098150418173905168318_u128,314303687750962880492529438999301355830_u128];
_7.3.5 = (_7.5.0.3.0, _7.3.2, _4.2);
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
_12.1 = -_7.5.0.3.1;
_7.1 = _7.5;
_7.0 = 36239_u16 as i8;
RET = _7.1.0.2;
_7.5.0.1 = _7.3.4 as i16;
_4 = (_7.1.0.1, _7.1.0.4, _7.3.5.2);
_7.5.0 = _7.1.0;
_7.3.5.0 = _8.0.3.1 as i16;
_7.3.2 = _4.1 & _7.5.0.4;
_13.0.3.0 = _7.5.0.1 | _8.0.3.0;
_7.5.0.3.0 = (-8336134806295715095_i64) as i16;
_13.0.2 = 257232889801757402809961326081763553143_u128 as i8;
_13.0.3 = (_7.5.0.1, _7.5.0.3.1);
Goto(bb11)
}
bb11 = {
Call(_18 = dump_var(7_usize, 5_usize, Move(_5), 1_usize, Move(_1), 4_usize, Move(_4), 19_usize, _19), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: (i16, bool, u64)) -> (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)) {
mir! {
type RET = (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32));
let _8: f64;
let _9: Adt54;
let _10: u64;
let _11: isize;
let _12: Adt58;
let _13: [i64; 5];
let _14: [i32; 1];
let _15: isize;
let _16: isize;
let _17: isize;
let _18: char;
let _19: [u128; 5];
let _20: f64;
let _21: f64;
let _22: u64;
let _23: char;
let _24: [u128; 5];
let _25: i16;
let _26: u128;
let _27: u8;
let _28: ();
let _29: ();
{
RET.3.4 = !3679811455314649667_usize;
RET.3.1 = 515771194730210040_i64 as f32;
RET.1.0.1 = !_7.0;
RET.1.0.4 = !_3;
RET.3.1 = 56232_u16 as f32;
RET.5.0.1 = RET.1.0.1;
RET.5.1 = 8060_u16 as u32;
RET.3.2 = _5;
RET.1.0.0 = [278663911928604992646195575417404604966_u128,236877512927344137270478968349767738520_u128,17758372573595224828926603493542638570_u128,328748028462911113588421573491505997687_u128,316322069744869018220791353461028498747_u128];
_11 = (-9223372036854775808_isize);
_12.fld0 = !157481294388990453647281066110443863177_u128;
RET.5.0.0 = [_12.fld0,_12.fld0,_12.fld0,_12.fld0,_12.fld0];
RET.3.4 = RET.5.1 as usize;
RET.2 = core::ptr::addr_of!(RET.1.0.2);
RET.1.0.1 = _7.0 * _7.0;
RET.3.0 = [_12.fld0,_12.fld0,_12.fld0,_12.fld0,_12.fld0];
RET.1.0.4 = _6 == _7.1;
_7 = (RET.1.0.1, RET.1.0.4, 4587798406484354574_u64);
RET.3.1 = RET.3.4 as f32;
RET.5.0.3.0 = RET.1.0.1 & RET.1.0.1;
RET.1.0.3.1 = -RET.3.1;
RET.4 = [(-307594428799342763_i64),(-7309512828474830458_i64),8021414219217295951_i64,2778792554570326459_i64,2378084731785338399_i64];
Call(RET.5.0.2 = fn9(_2, _2, _6, _7.2, RET.1.0.4, _1, _3, _7.2, _1, RET.3.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.fld3 = (_7.2, '\u{de405}');
_14 = [(-2083065057_i32)];
_6 = !_1;
RET.1.0.2 = (-1377790609_i32) as i8;
RET.3.5 = (RET.1.0.1, _5, _12.fld3.0);
_4 = !RET.3.2;
Call(RET.1.0.2 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _6;
RET.3 = (RET.1.0.0, RET.1.0.3.1, _5, 106164070599463209_usize, 0_usize, _7);
_2 = _1 <= _1;
RET.1.0.4 = _2;
_15 = _11;
_16 = _2 as isize;
RET.5.1 = 3224731655_u32;
RET.3.2 = !RET.1.0.4;
_12.fld3.0 = 9804949024519290_i64 as u64;
RET.1.0.3 = (RET.5.0.3.0, RET.3.1);
_7.0 = RET.1.0.3.0 + RET.3.5.0;
RET.5.0.4 = RET.1.0.4;
RET.1.0.3 = (_7.0, RET.3.1);
RET.5.0 = (RET.3.0, RET.1.0.3.0, RET.1.0.2, RET.1.0.3, _6);
Goto(bb3)
}
bb3 = {
RET.1.0.1 = !RET.5.0.1;
RET.1.0.1 = RET.3.5.0;
RET.5.0.3.0 = _7.0 * RET.1.0.1;
match RET.3.5.2 {
4587798406484354574 => bb4,
_ => bb1
}
}
bb4 = {
RET.5.1 = (-2084930574_i32) as u32;
RET.3.2 = _4;
RET.3.5.1 = _5 | RET.1.0.4;
RET.3.5.0 = !_7.0;
_7 = (RET.5.0.1, RET.1.0.4, RET.3.5.2);
RET.3.5.2 = !_7.2;
RET.3.3 = RET.3.4;
RET.5.0.3 = RET.1.0.3;
RET.3.2 = !RET.1.0.4;
RET.1.0.0 = [_12.fld0,_12.fld0,_12.fld0,_12.fld0,_12.fld0];
RET.1 = RET.5;
RET.3 = (RET.5.0.0, RET.1.0.3.1, RET.5.0.4, 3_usize, 11394380588241552102_usize, _7);
RET.5.0.3 = RET.1.0.3;
_7.0 = !RET.5.0.1;
RET.1.0.1 = 199677168177595103_i64 as i16;
_7 = (RET.1.0.1, RET.1.0.4, RET.3.5.2);
_11 = _16 + _16;
RET.1.0.0 = RET.3.0;
RET.3.0 = RET.5.0.0;
_16 = _11;
RET.4 = [4107551937214261178_i64,8930291146377724157_i64,3453600663540183694_i64,(-6185623014416859137_i64),(-2739280520765448837_i64)];
match _7.2 {
0 => bb2,
4587798406484354574 => bb6,
_ => bb5
}
}
bb5 = {
_12.fld3 = (_7.2, '\u{de405}');
_14 = [(-2083065057_i32)];
_6 = !_1;
RET.1.0.2 = (-1377790609_i32) as i8;
RET.3.5 = (RET.1.0.1, _5, _12.fld3.0);
_4 = !RET.3.2;
Call(RET.1.0.2 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_23 = _12.fld3.1;
RET.3.4 = RET.3.3;
RET.1.1 = (-7629582124124748117091265252667257848_i128) as u32;
_18 = _23;
RET.0 = -RET.1.0.2;
RET.1.0 = (RET.5.0.0, RET.3.5.0, RET.0, RET.5.0.3, RET.5.0.4);
RET.3.5 = (RET.5.0.3.0, _4, _7.2);
RET.5.0 = RET.1.0;
_8 = RET.1.0.1 as f64;
RET.3.5.2 = 1594390958_i32 as u64;
RET.1.0.3 = (RET.5.0.1, RET.3.1);
_24 = [_12.fld0,_12.fld0,_12.fld0,_12.fld0,_12.fld0];
RET.5 = RET.1;
RET.3.5.0 = 87_u8 as i16;
RET.3.4 = RET.3.3;
RET.1.0.3 = (RET.1.0.1, RET.3.1);
RET.5.0.4 = _16 <= _16;
RET.3.5 = _7;
_10 = !RET.3.5.2;
Goto(bb7)
}
bb7 = {
Call(_28 = dump_var(8_usize, 3_usize, Move(_3), 5_usize, Move(_5), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_28 = dump_var(8_usize, 24_usize, Move(_24), 2_usize, Move(_2), 15_usize, Move(_15), 29_usize, _29), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: u64,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: u64,mut _9: bool,mut _10: bool) -> i8 {
mir! {
type RET = i8;
let _11: char;
let _12: isize;
let _13: isize;
let _14: [u32; 1];
let _15: [i64; 5];
let _16: f32;
let _17: isize;
let _18: i8;
let _19: Adt55;
let _20: i16;
let _21: ((u64, char), i128, (i16, f32));
let _22: char;
let _23: [u32; 1];
let _24: bool;
let _25: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _26: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _27: Adt55;
let _28: (bool, [i64; 5], u128);
let _29: u128;
let _30: Adt55;
let _31: [i32; 1];
let _32: *const &'static i128;
let _33: (bool, u32, bool, u32);
let _34: bool;
let _35: (bool, [i64; 5], u128);
let _36: f64;
let _37: u8;
let _38: i64;
let _39: usize;
let _40: Adt49;
let _41: (bool, u32, bool, u32);
let _42: Adt53;
let _43: isize;
let _44: i8;
let _45: u16;
let _46: f32;
let _47: *mut ([u128; 5], i16, i8, (i16, f32), bool);
let _48: Adt54;
let _49: f64;
let _50: ();
let _51: ();
{
_3 = _10 <= _9;
RET = 88_i8 * (-19_i8);
RET = 614210353_i32 as i8;
Call(_4 = core::intrinsics::bswap(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _5 & _9;
_3 = !_7;
_2 = _7 & _9;
RET = 21_i8 & (-100_i8);
_7 = _2;
_7 = _3 != _2;
_2 = _6 >= _3;
_3 = !_10;
_3 = _6 != _2;
_7 = !_2;
_10 = _5 != _9;
_5 = !_7;
_3 = !_2;
_12 = 62343_u16 as isize;
_8 = _4 - _4;
_12 = _5 as isize;
_8 = 18276_u16 as u64;
_8 = _4;
RET = (-111_i8);
Goto(bb2)
}
bb2 = {
_9 = _3;
_13 = _12;
_12 = !_13;
_1 = !_6;
_11 = '\u{9863d}';
Call(RET = fn10(_3, _13, _9, _13, _12, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = '\u{f284b}';
RET = 50_i8 + 21_i8;
_4 = (-1231828379_i32) as u64;
_1 = _6;
_13 = !_12;
_2 = _1 > _3;
_7 = !_9;
_15 = [2764450661082558874_i64,(-2281885070455366387_i64),(-2332961990734261412_i64),(-3413836760429227321_i64),(-7634487297862698514_i64)];
_13 = !_12;
_17 = _13 << _8;
_5 = _7;
RET = 34_i8 + (-120_i8);
_6 = _1;
_10 = _3 <= _5;
_13 = _17;
_16 = _12 as f32;
_16 = (-25192_i16) as f32;
_12 = _17;
_18 = RET;
_17 = _13;
_5 = _9 != _7;
_12 = _17 - _13;
_16 = 2028282830_i32 as f32;
_3 = !_2;
_4 = _8;
_20 = !(-1934_i16);
_9 = _2;
Goto(bb4)
}
bb4 = {
RET = !_18;
_14 = [1378645060_u32];
RET = !_18;
_16 = _4 as f32;
_15 = [2620240498685344269_i64,1906301665205424267_i64,2450853227390311015_i64,3121665511561223956_i64,9034288280281056877_i64];
_14 = [881348658_u32];
_14 = [946018632_u32];
_2 = _3 < _7;
_8 = _4;
_12 = !_13;
_1 = !_9;
_21.2.1 = _16 + _16;
_17 = _12 | _13;
_4 = _8;
_21.0.0 = _8;
_18 = -RET;
_16 = _21.2.1 + _21.2.1;
_21.2 = (_20, _16);
_4 = _21.0.0;
_1 = _12 != _12;
_21.1 = 114773197717665453792303492245843083903_i128 | 131754771655457010022146229771547131554_i128;
_2 = _12 > _12;
_21.2 = (_20, _16);
_5 = _9;
_9 = !_10;
_23 = [1522740002_u32];
_21.0.1 = _11;
Call(_21.2.0 = fn17(_10, _12, _21.0.0, _10, _16, _16, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = !_21.0.0;
_21.1 = 96882831303311548702178491564695519684_i128 << _12;
_14 = [1291773525_u32];
_21.0 = (_4, _11);
_12 = _11 as isize;
_24 = !_2;
_15 = [(-3145410446085636563_i64),399175730332531048_i64,(-8726341348953251558_i64),(-8664280937728145710_i64),(-191459511631759915_i64)];
_14 = [2344153204_u32];
_8 = !_4;
_21.2.0 = _20 - _20;
_21.0.0 = _8 ^ _4;
_21.0.0 = !_4;
_18 = _21.1 as i8;
_7 = _13 >= _13;
_5 = _24 & _3;
_12 = 170_u8 as isize;
_21.2.0 = _21.0.0 as i16;
_21.1 = !(-46521662436733530046937775443871395799_i128);
_25.2 = !_5;
_25.3 = 4_usize;
_26.0.1 = _21.2.1;
_26.0.2 = _21.2.0 == _21.2.0;
_26.0.0 = [110403609086734690215857301007682077139_u128,298832023194622488045686451638942940663_u128,245652467572892103578960445131178732682_u128,241182821527452753905157190472953049564_u128,822843290221813831821795863327893875_u128];
_25.2 = !_6;
Call(_25.4 = core::intrinsics::transmute(_21.0.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26.1.5.2 = _21.0.0 * _4;
_22 = _21.0.1;
_26.0.5.2 = _25.4 as u64;
_26.1.5 = (_21.2.0, _25.2, _4);
_26.0.3 = !_25.4;
Call(_27 = fn18(_26.1.5.1, _3, _21, _21.0, _26.1.5, _17, _9, _13, _3, _13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_25.0 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).0;
_26.0.4 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).3;
_26.0.0 = [Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2,Field::<u128>(Variant(_27, 2), 2),Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2,Field::<u128>(Variant(_27, 2), 2),Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2];
_19 = Adt55::Variant2 { fld0: Field::<i128>(Variant(_27, 2), 0),fld1: Field::<u32>(Variant(_27, 2), 1),fld2: Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2,fld3: Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3),fld4: Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4) };
_26.0.5.1 = !_5;
place!(Field::<(bool, [i64; 5], u128)>(Variant(_19, 2), 4)).2 = _26.1.5.0 as u128;
_25.5 = (Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).5.0, Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).5.1, _26.0.5.2);
_5 = _26.0.5.1;
_26.1.4 = !Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).4;
_26.1.1 = RET as f32;
_26.0 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3);
_26.1.2 = _21.2.1 == Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).1;
_24 = _1 <= _9;
_3 = _24 & Field::<(bool, [i64; 5], u128)>(Variant(_19, 2), 4).0;
_26.0.1 = (-641979911_i32) as f32;
SetDiscriminant(_19, 2);
_2 = _9;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).3 = _26.0.4 & _26.0.3;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).3 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).3;
SetDiscriminant(_27, 2);
Goto(bb8)
}
bb8 = {
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)) = _26.0;
_23 = [4130173724_u32];
_26.0.2 = _8 == _4;
place!(Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4)).0 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).2 != _6;
place!(Field::<u32>(Variant(_19, 2), 1)) = 78314596194451901279561232406583877325_u128 as u32;
Goto(bb9)
}
bb9 = {
_3 = _26.0.5.1;
_26.0.3 = _25.4;
_26.1.3 = Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).0 as usize;
_21.2.0 = Field::<u32>(Variant(_19, 2), 1) as i16;
_35.0 = _9;
_26.1 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3);
_35.0 = !_9;
_33.3 = _13 as u32;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).5.0 = 18648_u16 as i16;
_21.2 = (_26.1.5.0, _16);
place!(Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4)).1 = [8780727973971526493_i64,(-177819350146873386_i64),(-7670064284514470872_i64),(-2148714527120082378_i64),2848725247467302357_i64];
_34 = !_6;
_26.0.2 = _26.1.2;
_26.0 = _26.1;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).0 = [101914438014809848809628854939902546439_u128,65682821034983657366548819055454445096_u128,49726240902006943773673208985547000996_u128,21603153903399581600803416080348576866_u128,123242662421071947591157790106071767600_u128];
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).1 = _21.2.1;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).5.1 = !_26.0.5.1;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).2 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).5.1;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).4 = !_26.1.4;
_21.2 = (Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).5.0, Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).1);
match _25.3 {
0 => bb10,
1 => bb11,
4 => bb13,
_ => bb12
}
}
bb10 = {
_7 = _5 & _9;
_3 = !_7;
_2 = _7 & _9;
RET = 21_i8 & (-100_i8);
_7 = _2;
_7 = _3 != _2;
_2 = _6 >= _3;
_3 = !_10;
_3 = _6 != _2;
_7 = !_2;
_10 = _5 != _9;
_5 = !_7;
_3 = !_2;
_12 = 62343_u16 as isize;
_8 = _4 - _4;
_12 = _5 as isize;
_8 = 18276_u16 as u64;
_8 = _4;
RET = (-111_i8);
Goto(bb2)
}
bb11 = {
_25.0 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).0;
_26.0.4 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).3;
_26.0.0 = [Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2,Field::<u128>(Variant(_27, 2), 2),Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2,Field::<u128>(Variant(_27, 2), 2),Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2];
_19 = Adt55::Variant2 { fld0: Field::<i128>(Variant(_27, 2), 0),fld1: Field::<u32>(Variant(_27, 2), 1),fld2: Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).2,fld3: Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3),fld4: Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4) };
_26.0.5.1 = !_5;
place!(Field::<(bool, [i64; 5], u128)>(Variant(_19, 2), 4)).2 = _26.1.5.0 as u128;
_25.5 = (Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).5.0, Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).5.1, _26.0.5.2);
_5 = _26.0.5.1;
_26.1.4 = !Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).4;
_26.1.1 = RET as f32;
_26.0 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3);
_26.1.2 = _21.2.1 == Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).1;
_24 = _1 <= _9;
_3 = _24 & Field::<(bool, [i64; 5], u128)>(Variant(_19, 2), 4).0;
_26.0.1 = (-641979911_i32) as f32;
SetDiscriminant(_19, 2);
_2 = _9;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).3 = _26.0.4 & _26.0.3;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).3 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).3;
SetDiscriminant(_27, 2);
Goto(bb8)
}
bb12 = {
_9 = _3;
_13 = _12;
_12 = !_13;
_1 = !_6;
_11 = '\u{9863d}';
Call(RET = fn10(_3, _13, _9, _13, _12, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_33 = (_26.1.2, Field::<u32>(Variant(_19, 2), 1), Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).5.1, Field::<u32>(Variant(_19, 2), 1));
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).5 = _26.0.5;
_28 = (_33.2, _15, 190794205898034957072415375856575947262_u128);
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3)).3 = !Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).3;
_20 = (-870011745_i32) as i16;
_27 = Adt55::Variant2 { fld0: _21.1,fld1: _33.1,fld2: _28.2,fld3: _26.0,fld4: _28 };
_25.4 = _26.0.3 | _26.1.4;
_44 = _18 & _18;
_41 = _33;
_26.0 = (_26.1.0, _21.2.1, _26.1.5.1, _26.1.3, _26.1.4, _25.5);
place!(Field::<(bool, [i64; 5], u128)>(Variant(_19, 2), 4)) = _28;
_26.0.4 = _26.0.3;
_1 = Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_27, 2), 3).2;
_25.2 = _5;
match Field::<(bool, [i64; 5], u128)>(Variant(_19, 2), 4).2 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb12,
4 => bb8,
190794205898034957072415375856575947262 => bb14,
_ => bb6
}
}
bb14 = {
_31 = [180052985_i32];
place!(Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4)).0 = _33.2 <= _6;
_35.1 = Field::<(bool, [i64; 5], u128)>(Variant(_27, 2), 4).1;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).5 = _26.0.5;
_25 = _26.0;
_26 = (_25, Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3));
_41.3 = Field::<u32>(Variant(_27, 2), 1) & _33.1;
_20 = _13 as i16;
place!(Field::<i128>(Variant(_27, 2), 0)) = _26.0.5.1 as i128;
_24 = _18 <= _44;
_39 = !Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3).3;
_37 = !200_u8;
_35.2 = !_28.2;
_26.1.5.1 = _33.0;
_17 = _13 << _44;
place!(Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(_19, 2), 3)).2 = _35.0;
place!(Field::<i128>(Variant(_27, 2), 0)) = _26.0.1 as i128;
_16 = _21.2.1;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(9_usize, 6_usize, Move(_6), 23_usize, Move(_23), 44_usize, Move(_44), 39_usize, Move(_39)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(9_usize, 10_usize, Move(_10), 35_usize, Move(_35), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(9_usize, 3_usize, Move(_3), 5_usize, Move(_5), 41_usize, Move(_41), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(9_usize, 24_usize, Move(_24), 33_usize, Move(_33), 1_usize, Move(_1), 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: bool,mut _2: isize,mut _3: bool,mut _4: isize,mut _5: isize,mut _6: isize) -> i8 {
mir! {
type RET = i8;
let _7: Adt53;
let _8: ((u64, char), i128, (i16, f32));
let _9: Adt45;
let _10: (usize, i16, i64);
let _11: i32;
let _12: ([u128; 5], i16, i8, (i16, f32), bool);
let _13: [i32; 1];
let _14: u16;
let _15: (bool, u32, bool, u32);
let _16: i16;
let _17: [bool; 7];
let _18: f64;
let _19: (i16, f32);
let _20: ();
let _21: ();
{
_2 = 1345500712_i32 as isize;
_3 = _1;
_6 = !_4;
_2 = _6;
_5 = _4;
_8.2.0 = _3 as i16;
_8.0.0 = 85_i8 as u64;
_8.2.1 = 6483701708584399349_usize as f32;
_8.2.0 = _8.2.1 as i16;
_8.1 = (-132747372793517454887313914632368968677_i128);
RET = _8.2.1 as i8;
_8.2.0 = 29131_i16 >> _4;
_6 = !_4;
_1 = _3;
_4 = _6;
Call(_8.0 = fn11(_6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.0.1 = '\u{ab103}';
_10.1 = _8.2.0 >> _4;
_10.0 = !2_usize;
_10.2 = -8643233129881130473_i64;
_6 = _2 & _4;
_4 = -_6;
RET = (-125_i8) ^ 20_i8;
_11 = !452663651_i32;
_10.0 = 12918853755792675404_usize & 9171757791147761217_usize;
_4 = RET as isize;
_10.0 = 18407775_u32 as usize;
_8.0 = (10360241063948743153_u64, '\u{cd20d}');
match _8.0.0 {
0 => bb2,
10360241063948743153 => bb4,
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
_4 = _5 - _5;
_13 = [_11];
_10 = (5_usize, _8.2.0, 659920057123313084_i64);
_13 = [_11];
_8.2.1 = _8.1 as f32;
_8.0.1 = '\u{e4442}';
_14 = 4833_u16;
_15 = (_1, 2605910230_u32, _1, 2992117554_u32);
_8.1 = (-25485702280024504299049790063056482579_i128);
RET = 69_i8;
_10 = (4_usize, _8.2.0, (-3100207248996592959_i64));
_13 = [_11];
_11 = _15.0 as i32;
_15.2 = !_3;
_15.3 = !_15.1;
match _15.1 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
2605910230 => bb11,
_ => bb10
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_8.0.1 = '\u{ab103}';
_10.1 = _8.2.0 >> _4;
_10.0 = !2_usize;
_10.2 = -8643233129881130473_i64;
_6 = _2 & _4;
_4 = -_6;
RET = (-125_i8) ^ 20_i8;
_11 = !452663651_i32;
_10.0 = 12918853755792675404_usize & 9171757791147761217_usize;
_4 = RET as isize;
_10.0 = 18407775_u32 as usize;
_8.0 = (10360241063948743153_u64, '\u{cd20d}');
match _8.0.0 {
0 => bb2,
10360241063948743153 => bb4,
_ => bb3
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
_12.1 = _8.2.0;
_16 = _8.2.0;
RET = (-46_i8) << _15.1;
_16 = _12.1;
_12.3.1 = 168_u8 as f32;
_15.3 = _15.1;
_5 = !_4;
_10 = (3298949481376370493_usize, _12.1, 1902125898683325505_i64);
_12.1 = _8.0.1 as i16;
_16 = 179_u8 as i16;
_12.3.1 = _10.2 as f32;
_8.2.1 = _12.3.1 + _12.3.1;
_13 = [_11];
_8.2.0 = _8.0.0 as i16;
_12.2 = _8.1 as i8;
_12.2 = RET - RET;
_14 = _15.3 as u16;
_10.0 = 135756265154402587013532675453137225535_u128 as usize;
_16 = _10.1 >> RET;
_4 = !_2;
_12.3 = (_10.1, _8.2.1);
_19 = _12.3;
Goto(bb12)
}
bb12 = {
Call(_20 = dump_var(10_usize, 13_usize, Move(_13), 15_usize, Move(_15), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_20 = dump_var(10_usize, 10_usize, Move(_10), 16_usize, Move(_16), 21_usize, _21, 21_usize, _21), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: isize) -> (u64, char) {
mir! {
type RET = (u64, char);
let _3: f32;
let _4: bool;
let _5: (bool, u32, bool, u32);
let _6: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _7: Adt60;
let _8: f64;
let _9: char;
let _10: Adt49;
let _11: f32;
let _12: Adt46;
let _13: f64;
let _14: (i16, f32);
let _15: isize;
let _16: ();
let _17: ();
{
RET = (17445865559619727590_u64, '\u{fabbf}');
_1 = 84766064472561099084094918332160245799_i128 as isize;
RET.0 = _2 as u64;
RET.1 = '\u{95e69}';
RET.0 = 16112348336313751204_u64 ^ 14309346500129197571_u64;
_1 = _2 ^ _2;
RET.0 = 15675456331673143096_u64 + 8965102759111547258_u64;
RET = (11917600566517559526_u64, '\u{80d83}');
RET.1 = '\u{bd8c6}';
RET.1 = '\u{1bfe8}';
RET.0 = 48252041093387628202076303957896400508_u128 as u64;
RET = (4030594937985751987_u64, '\u{aa1a1}');
RET.0 = 12375759187155332317_u64;
RET.0 = 4384267099630352146_u64 * 10285119353902430611_u64;
_4 = _1 >= _1;
RET.0 = 38288_u16 as u64;
_2 = _1;
_4 = true;
_6.0 = [326319522814557237683479378805237156298_u128,116746352251352969768474270065315103983_u128,59353599494012571657233593495955614238_u128,321801946929060890998551336185462176411_u128,155165292252909423314385709395407506890_u128];
_6.5.2 = RET.0 & RET.0;
_3 = 108_u8 as f32;
_5.1 = 1305318612_u32 >> _2;
_6.2 = _4;
_5.2 = _6.2;
_5.3 = (-31103_i16) as u32;
_5 = (_6.2, 3891975174_u32, _4, 1877873447_u32);
Call(RET.1 = fn12(_2, _1, _1, _2, _1, _1, _2, _2, _1, _5.3, _2, _2, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.1 = '\u{ae59c}';
_6.4 = 4532728129925015871_usize;
_2 = 12593_u16 as isize;
match _5.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1877873447 => bb7,
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
_6.4 = !0_usize;
RET.0 = !_6.5.2;
match _5.1 {
3891975174 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_6.0 = [258905950512383021604315071825841231925_u128,127952239727391450503529026435491444463_u128,250530039512110498411268815868533236445_u128,228480326180949799897196271260642530284_u128,321303706579095342220356839469540964350_u128];
Goto(bb10)
}
bb10 = {
_5 = (_4, 3607636622_u32, _6.2, 475274076_u32);
_1 = !_2;
_5.1 = (-8289800175800147594_i64) as u32;
match _5.3 {
0 => bb9,
475274076 => bb12,
_ => bb11
}
}
bb11 = {
RET.1 = '\u{ae59c}';
_6.4 = 4532728129925015871_usize;
_2 = 12593_u16 as isize;
match _5.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
1877873447 => bb7,
_ => bb6
}
}
bb12 = {
_6.1 = (-28629_i16) as f32;
_6.4 = 17880493220910892041_usize;
RET.0 = RET.1 as u64;
Goto(bb13)
}
bb13 = {
_6.4 = !7360678113395150651_usize;
_6.5 = (10478_i16, _5.2, RET.0);
RET.1 = '\u{f58d2}';
_8 = _6.5.0 as f64;
_6.2 = _8 <= _8;
_4 = _6.2;
RET.1 = '\u{5ecf2}';
RET = (_6.5.2, '\u{779d}');
Goto(bb14)
}
bb14 = {
_6.4 = 4_usize;
_5.2 = _4 < _4;
_5.1 = _5.3 % _5.3;
_13 = -_8;
_2 = _3 as isize;
_6.5.0 = 18139_i16 >> _6.4;
_3 = _6.1;
_14 = (_6.5.0, _3);
_5 = (_4, 1625979902_u32, _6.2, 972848068_u32);
_6.1 = _14.1 * _14.1;
_6.5.0 = _6.4 as i16;
_1 = !_2;
_6.5 = (_14.0, _5.2, RET.0);
_14 = (_6.5.0, _6.1);
_6.5.1 = !_4;
RET.0 = _6.5.2;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(11_usize, 5_usize, Move(_5), 1_usize, Move(_1), 17_usize, _17, 17_usize, _17), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: u32,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> char {
mir! {
type RET = char;
let _15: isize;
let _16: bool;
let _17: bool;
let _18: (bool, [i64; 5], u128);
let _19: char;
let _20: [u32; 3];
let _21: f32;
let _22: isize;
let _23: i8;
let _24: f32;
let _25: isize;
let _26: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _27: usize;
let _28: i64;
let _29: (i16, f32);
let _30: Adt50;
let _31: i64;
let _32: [i32; 1];
let _33: f64;
let _34: f32;
let _35: isize;
let _36: isize;
let _37: [u32; 3];
let _38: i8;
let _39: Adt51;
let _40: bool;
let _41: isize;
let _42: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _43: (bool, [i64; 5], u128);
let _44: ();
let _45: ();
{
_8 = _13;
_11 = 8_i8 as isize;
_1 = !_12;
RET = '\u{807bb}';
_5 = _3 & _6;
_5 = 56229_u16 as isize;
_12 = -_1;
RET = '\u{8a53e}';
RET = '\u{f02dd}';
_6 = _12 - _3;
_6 = (-3652251381373747990_i64) as isize;
_2 = _4 * _7;
_15 = !_13;
_15 = 6088708006425941727473710849211869469_u128 as isize;
_8 = _9 + _9;
_5 = _8;
_3 = _9;
_16 = false;
_4 = !_3;
_5 = _12;
_12 = _3;
_7 = _12 << _9;
_10 = !2126446757_u32;
_12 = 54075_u16 as isize;
_4 = !_9;
_16 = _5 < _13;
_15 = _8 + _2;
Call(_1 = fn13(_8, _7, _7, _9, _5, _3, _9, _4, _8, _13, _5, _14, _9, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _7;
_13 = RET as isize;
_11 = _3;
_15 = -_9;
_15 = !_7;
_10 = 15860619616152819977_u64 as u32;
_14 = _8 - _4;
_13 = _11;
_12 = !_1;
_5 = -_3;
RET = '\u{2e482}';
_14 = 3176025199172446876_i64 as isize;
RET = '\u{b92e9}';
_16 = _7 < _11;
_3 = _2 & _13;
_2 = -_4;
_2 = 0_usize as isize;
_6 = _5;
_6 = !_11;
RET = '\u{57d28}';
_3 = _11;
Call(_13 = fn14(_12, _12, _1, _8, _3, _1, _3, _6, _15, _12, _11, _7, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = !true;
_15 = _4 ^ _6;
_5 = !_1;
_4 = !_13;
RET = '\u{e223d}';
_16 = _15 <= _9;
_18.2 = 246123483692020581562285043632508923690_u128;
RET = '\u{41839}';
_16 = true;
_17 = !_16;
_19 = RET;
_19 = RET;
_14 = _7;
_18.0 = _4 < _3;
_18.1 = [9006873859764607246_i64,(-8334819394086322903_i64),(-6202693245788754196_i64),6449555939132189727_i64,9149632659542904316_i64];
_7 = 819003208_i32 as isize;
_6 = _15;
_16 = _18.0 & _18.0;
_18.2 = !147701388054268415791041797784599346900_u128;
_20 = [_10,_10,_10];
_10 = _14 as u32;
_18.1 = [(-6619723568126263711_i64),(-8995106123424878119_i64),(-6916614447468394725_i64),7736161239085535230_i64,(-3968060226752892319_i64)];
_4 = _1 << _14;
_6 = 2396_u16 as isize;
Goto(bb3)
}
bb3 = {
_21 = _10 as f32;
_12 = !_11;
_19 = RET;
RET = _19;
_15 = !_8;
_6 = -_8;
_3 = RET as isize;
_26.0.4 = _18.0 & _16;
_23 = !108_i8;
_8 = -_14;
_23 = 54_i8;
_25 = _18.2 as isize;
_26.0.3.1 = _21 + _21;
_8 = 11015_u16 as isize;
_18.1 = [(-4899002322765446692_i64),(-7224045598893702590_i64),237612481109393581_i64,1319948273321325051_i64,(-8810299588347563922_i64)];
_18.0 = !_26.0.4;
_8 = _19 as isize;
_19 = RET;
_4 = _15 & _13;
_3 = _14 | _5;
_4 = _3;
_24 = _26.0.3.1 + _21;
_6 = _4;
Call(_21 = fn15(_9, _11, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = 4_u8 as isize;
_26.0.3.1 = 2_usize as f32;
_28 = 8848459089494962353_i64;
_5 = _11 >> _9;
_26.0.1 = (-13914_i16) + 7043_i16;
match _28 {
0 => bb5,
8848459089494962353 => bb7,
_ => bb6
}
}
bb5 = {
_21 = _10 as f32;
_12 = !_11;
_19 = RET;
RET = _19;
_15 = !_8;
_6 = -_8;
_3 = RET as isize;
_26.0.4 = _18.0 & _16;
_23 = !108_i8;
_8 = -_14;
_23 = 54_i8;
_25 = _18.2 as isize;
_26.0.3.1 = _21 + _21;
_8 = 11015_u16 as isize;
_18.1 = [(-4899002322765446692_i64),(-7224045598893702590_i64),237612481109393581_i64,1319948273321325051_i64,(-8810299588347563922_i64)];
_18.0 = !_26.0.4;
_8 = _19 as isize;
_19 = RET;
_4 = _15 & _13;
_3 = _14 | _5;
_4 = _3;
_24 = _26.0.3.1 + _21;
_6 = _4;
Call(_21 = fn15(_9, _11, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_16 = !true;
_15 = _4 ^ _6;
_5 = !_1;
_4 = !_13;
RET = '\u{e223d}';
_16 = _15 <= _9;
_18.2 = 246123483692020581562285043632508923690_u128;
RET = '\u{41839}';
_16 = true;
_17 = !_16;
_19 = RET;
_19 = RET;
_14 = _7;
_18.0 = _4 < _3;
_18.1 = [9006873859764607246_i64,(-8334819394086322903_i64),(-6202693245788754196_i64),6449555939132189727_i64,9149632659542904316_i64];
_7 = 819003208_i32 as isize;
_6 = _15;
_16 = _18.0 & _18.0;
_18.2 = !147701388054268415791041797784599346900_u128;
_20 = [_10,_10,_10];
_10 = _14 as u32;
_18.1 = [(-6619723568126263711_i64),(-8995106123424878119_i64),(-6916614447468394725_i64),7736161239085535230_i64,(-3968060226752892319_i64)];
_4 = _1 << _14;
_6 = 2396_u16 as isize;
Goto(bb3)
}
bb7 = {
_18.1 = [_28,_28,_28,_28,_28];
_26.0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
_26.0.3.0 = _26.0.1;
_27 = !5_usize;
_28 = _13 as i64;
_4 = !_14;
_29.0 = -_26.0.1;
_32 = [62669030_i32];
_16 = !_18.0;
_1 = -_13;
Call(_8 = core::intrinsics::bswap(_1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = _19;
RET = _19;
_26.0.3 = (_29.0, _24);
_10 = !2048134470_u32;
RET = _19;
_26.0.1 = !_26.0.3.0;
_25 = _6 - _3;
Call(_25 = core::intrinsics::bswap(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_18.0 = _26.0.4;
_26.0.2 = _23 & _23;
_26.0.4 = _16 | _18.0;
_29.1 = _26.0.3.1 * _24;
_33 = _18.2 as f64;
_36 = _28 as isize;
_6 = !_25;
_26.1 = !_10;
_10 = 5553432621791571700_u64 as u32;
_26.0.4 = _16 | _16;
RET = _19;
_18.1 = [_28,_28,_28,_28,_28];
_7 = _27 as isize;
_26.0.3.1 = _29.1;
_6 = _4 & _4;
match _23 {
0 => bb4,
1 => bb10,
2 => bb11,
54 => bb13,
_ => bb12
}
}
bb10 = {
_5 = 4_u8 as isize;
_26.0.3.1 = 2_usize as f32;
_28 = 8848459089494962353_i64;
_5 = _11 >> _9;
_26.0.1 = (-13914_i16) + 7043_i16;
match _28 {
0 => bb5,
8848459089494962353 => bb7,
_ => bb6
}
}
bb11 = {
_1 = _7;
_13 = RET as isize;
_11 = _3;
_15 = -_9;
_15 = !_7;
_10 = 15860619616152819977_u64 as u32;
_14 = _8 - _4;
_13 = _11;
_12 = !_1;
_5 = -_3;
RET = '\u{2e482}';
_14 = 3176025199172446876_i64 as isize;
RET = '\u{b92e9}';
_16 = _7 < _11;
_3 = _2 & _13;
_2 = -_4;
_2 = 0_usize as isize;
_6 = _5;
_6 = !_11;
RET = '\u{57d28}';
_3 = _11;
Call(_13 = fn14(_12, _12, _1, _8, _3, _1, _3, _6, _15, _12, _11, _7, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_21 = _10 as f32;
_12 = !_11;
_19 = RET;
RET = _19;
_15 = !_8;
_6 = -_8;
_3 = RET as isize;
_26.0.4 = _18.0 & _16;
_23 = !108_i8;
_8 = -_14;
_23 = 54_i8;
_25 = _18.2 as isize;
_26.0.3.1 = _21 + _21;
_8 = 11015_u16 as isize;
_18.1 = [(-4899002322765446692_i64),(-7224045598893702590_i64),237612481109393581_i64,1319948273321325051_i64,(-8810299588347563922_i64)];
_18.0 = !_26.0.4;
_8 = _19 as isize;
_19 = RET;
_4 = _15 & _13;
_3 = _14 | _5;
_4 = _3;
_24 = _26.0.3.1 + _21;
_6 = _4;
Call(_21 = fn15(_9, _11, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
_26.0.1 = -_26.0.3.0;
_37 = [_10,_10,_26.1];
_29.1 = _26.0.3.1;
_26.0.3 = _29;
_31 = _28;
_32 = [(-27360812_i32)];
_7 = _10 as isize;
_15 = _3;
_38 = !_26.0.2;
_18.1 = [_28,_28,_28,_31,_28];
match _23 {
0 => bb11,
1 => bb14,
54 => bb16,
_ => bb15
}
}
bb14 = {
_21 = _10 as f32;
_12 = !_11;
_19 = RET;
RET = _19;
_15 = !_8;
_6 = -_8;
_3 = RET as isize;
_26.0.4 = _18.0 & _16;
_23 = !108_i8;
_8 = -_14;
_23 = 54_i8;
_25 = _18.2 as isize;
_26.0.3.1 = _21 + _21;
_8 = 11015_u16 as isize;
_18.1 = [(-4899002322765446692_i64),(-7224045598893702590_i64),237612481109393581_i64,1319948273321325051_i64,(-8810299588347563922_i64)];
_18.0 = !_26.0.4;
_8 = _19 as isize;
_19 = RET;
_4 = _15 & _13;
_3 = _14 | _5;
_4 = _3;
_24 = _26.0.3.1 + _21;
_6 = _4;
Call(_21 = fn15(_9, _11, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
RET = _19;
RET = _19;
_26.0.3 = (_29.0, _24);
_10 = !2048134470_u32;
RET = _19;
_26.0.1 = !_26.0.3.0;
_25 = _6 - _3;
Call(_25 = core::intrinsics::bswap(_12), ReturnTo(bb9), UnwindUnreachable())
}
bb16 = {
_5 = _12 & _36;
_22 = -_25;
_23 = _38;
_35 = _15;
_11 = _6 ^ _1;
_29.0 = _26.0.1 ^ _26.0.1;
_2 = 128_u8 as isize;
_4 = _15;
_41 = _18.0 as isize;
_42.0.1 = _26.0.3.0 + _26.0.3.0;
RET = _19;
_43.1 = [_31,_31,_28,_28,_28];
_26.0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
_34 = 9269033512943900749_u64 as f32;
Goto(bb17)
}
bb17 = {
Call(_44 = dump_var(12_usize, 36_usize, Move(_36), 31_usize, Move(_31), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(12_usize, 12_usize, Move(_12), 1_usize, Move(_1), 35_usize, Move(_35), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(12_usize, 27_usize, Move(_27), 14_usize, Move(_14), 10_usize, Move(_10), 13_usize, Move(_13)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(12_usize, 11_usize, Move(_11), 37_usize, Move(_37), 25_usize, Move(_25), 32_usize, Move(_32)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> isize {
mir! {
type RET = isize;
let _15: i128;
let _16: isize;
let _17: f64;
let _18: Adt59;
let _19: ([u128; 5], i16, i8, (i16, f32), bool);
let _20: (bool, u32, bool, u32);
let _21: (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32));
let _22: f64;
let _23: ();
let _24: ();
{
_2 = !_13;
_14 = _3 - _5;
_2 = 1726094212_u32 as isize;
RET = _11 >> _14;
RET = _12;
_5 = _8 >> _6;
_6 = _10 - _7;
_15 = 64539934170489042285222576582829878144_i128 - 33920979066347128325651145512785393141_i128;
_7 = _11;
_17 = (-24414_i16) as f64;
_8 = _4;
_19.0 = [45816744665842878512316444141971331952_u128,158668066881935227046010331357300273873_u128,14150612085568559712374127096728513779_u128,274862314293650761793642299335697032607_u128,173754534631087211469368399327383738951_u128];
_19.3.1 = (-6320412360207420502_i64) as f32;
_1 = _3;
_19.2 = !(-37_i8);
_12 = 1279561911_i32 as isize;
_20.2 = false;
_15 = 110925878044264388280353489024997193855_i128;
_20.0 = !_20.2;
_19.3.0 = (-16667_i16);
_21.3.0 = [281491898063591688612019974785110418011_u128,293945351251111849133314092574000580496_u128,78068148294812496837445175575507472301_u128,195112193431893906333249660174507757227_u128,158546588650397931453555513061334416231_u128];
_21.5.0 = (_21.3.0, _19.3.0, _19.2, _19.3, _20.0);
_21.4 = [8892566826814742938_i64,7951734949431442398_i64,9031573053831634461_i64,3556219584468233981_i64,(-7122793255076710528_i64)];
Goto(bb1)
}
bb1 = {
Call(_23 = dump_var(13_usize, 3_usize, Move(_3), 11_usize, Move(_11), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_23 = dump_var(13_usize, 15_usize, Move(_15), 12_usize, Move(_12), 2_usize, Move(_2), 24_usize, _24), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: Adt45;
let _15: char;
let _16: [u32; 3];
let _17: ();
let _18: ();
{
RET = -_9;
_8 = 164937967950719924305242537335102110438_u128 as isize;
_2 = 97_i8 as isize;
_13 = 1085007187_i32 as isize;
_6 = 56991_u16 as isize;
_8 = -_7;
_9 = _7;
_2 = _9 - _1;
RET = _1 ^ _12;
_11 = 19_i8 as isize;
_3 = _12;
_1 = RET;
_8 = -_4;
_16 = [3323345737_u32,1792034900_u32,1237480839_u32];
_15 = '\u{87207}';
_3 = _5;
_9 = 10834039403070198636_u64 as isize;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(14_usize, 16_usize, Move(_16), 9_usize, Move(_9), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_17 = dump_var(14_usize, 13_usize, Move(_13), 2_usize, Move(_2), 15_usize, Move(_15), 18_usize, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: isize) -> f32 {
mir! {
type RET = f32;
let _4: isize;
let _5: i16;
let _6: f64;
let _7: bool;
let _8: bool;
let _9: bool;
let _10: u32;
let _11: isize;
let _12: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _13: ();
let _14: ();
{
_1 = !_2;
_3 = _2 << _1;
RET = (-99_i8) as f32;
_4 = !_1;
Goto(bb1)
}
bb1 = {
_3 = _2 >> _2;
RET = 70085322211039295166902995657193953059_u128 as f32;
_2 = -_4;
RET = 58463_u16 as f32;
RET = 85459963826587582082847535633310257760_i128 as f32;
_2 = '\u{bda9}' as isize;
_1 = 3795763877895546792_i64 as isize;
_1 = _3 & _3;
_4 = _1 | _3;
_2 = !_3;
_3 = !_4;
_1 = _4 * _2;
RET = 2856989414606184954_i64 as f32;
_3 = !_1;
_1 = !_4;
_1 = _2;
RET = 7651160090376323543_u64 as f32;
RET = 3807644817_u32 as f32;
_3 = _2 ^ _2;
RET = 5_usize as f32;
_4 = _1;
Goto(bb2)
}
bb2 = {
_2 = -_1;
_1 = RET as isize;
_3 = !_2;
_3 = true as isize;
Call(_2 = fn16(_4, _4, _4, _4, _4, _4, _4, _4, _4, _3, _4, _4, _4, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = 227_u8 as isize;
_2 = _4;
_5 = (-6555_i16);
RET = (-580366994_i32) as f32;
_2 = !_4;
_2 = 1226065375_u32 as isize;
_3 = _4 & _4;
_6 = RET as f64;
_4 = _3 + _3;
_9 = !false;
_8 = _9;
_9 = _8;
_5 = 12998_i16 & (-7298_i16);
_5 = 17288_i16 << _3;
Goto(bb4)
}
bb4 = {
_9 = _8 ^ _8;
RET = 3_usize as f32;
_4 = !_3;
_1 = _4 ^ _3;
RET = 17158317100730761273_u64 as f32;
_3 = RET as isize;
_1 = _4;
_1 = !_4;
RET = 15430_u16 as f32;
RET = (-52300829884586012441189051080973863177_i128) as f32;
_7 = _8 | _9;
RET = 264947683191341489307356566473650049537_u128 as f32;
RET = _1 as f32;
_7 = _9;
_12.1 = 149829501010482299077957343249548696773_i128 as f32;
Goto(bb5)
}
bb5 = {
Call(_13 = dump_var(15_usize, 5_usize, Move(_5), 2_usize, Move(_2), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> isize {
mir! {
type RET = isize;
let _15: u16;
let _16: Adt44;
let _17: char;
let _18: (bool, u32, bool, u32);
let _19: Adt52;
let _20: u16;
let _21: (i16, f32);
let _22: [bool; 7];
let _23: (i16, f32);
let _24: Adt55;
let _25: *mut f32;
let _26: *mut ([u128; 5], i16, i8, (i16, f32), bool);
let _27: i8;
let _28: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _29: ();
let _30: ();
{
_2 = (-33_i8) as isize;
RET = 79248832619070265448560649623437327687_u128 as isize;
_11 = _5;
_9 = _8 & _5;
_9 = 86_u8 as isize;
_8 = 1229345557_i32 as isize;
_7 = (-22127211787484122115431062183657553622_i128) as isize;
_2 = (-150563915457017898303715771260749016155_i128) as isize;
_6 = _12;
_14 = 742219822_u32 as isize;
_2 = !_1;
_13 = !_4;
RET = 171484301_i32 as isize;
_15 = !9881_u16;
_10 = _13;
_14 = _6 >> _5;
_6 = _4;
_1 = _4 ^ _11;
_1 = 18202279657975067018_u64 as isize;
_14 = 181_u8 as isize;
_4 = !_3;
_18.3 = 1916698467_u32;
_6 = _5 * _11;
_14 = !_13;
_14 = _12;
match _18.3 {
1916698467 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_10 = _14 + _11;
_18.1 = _18.3 * _18.3;
_17 = '\u{f011f}';
_23.1 = _3 as f32;
_7 = _14;
_14 = -_13;
_22 = [false,true,false,false,false,false,true];
_4 = !_11;
_8 = _3;
_18.0 = !true;
RET = _10;
RET = _8 & _3;
_18.0 = true & true;
_20 = !_15;
_28.0.2 = (-26371199425731115144392497306272904244_i128) as i8;
_28.0.3.1 = _23.1 - _23.1;
_18.2 = _5 > _14;
_18.0 = _18.2 ^ _18.2;
Goto(bb3)
}
bb3 = {
Call(_29 = dump_var(16_usize, 4_usize, Move(_4), 13_usize, Move(_13), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_29 = dump_var(16_usize, 15_usize, Move(_15), 10_usize, Move(_10), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_29 = dump_var(16_usize, 1_usize, Move(_1), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: bool,mut _2: isize,mut _3: u64,mut _4: bool,mut _5: f32,mut _6: f32,mut _7: bool) -> i16 {
mir! {
type RET = i16;
let _8: [u32; 3];
let _9: (u64, char);
let _10: ();
let _11: ();
{
_4 = _7;
RET = _3 as i16;
Goto(bb1)
}
bb1 = {
Call(_10 = dump_var(17_usize, 7_usize, Move(_7), 4_usize, Move(_4), 11_usize, _11, 11_usize, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: bool,mut _2: bool,mut _3: ((u64, char), i128, (i16, f32)),mut _4: (u64, char),mut _5: (i16, bool, u64),mut _6: isize,mut _7: bool,mut _8: isize,mut _9: bool,mut _10: isize) -> Adt55 {
mir! {
type RET = Adt55;
let _11: f32;
let _12: ((u64, char), i128, (i16, f32));
let _13: (i16, f32);
let _14: Adt44;
let _15: char;
let _16: isize;
let _17: isize;
let _18: (bool, [i64; 5], u128);
let _19: char;
let _20: u8;
let _21: f32;
let _22: *mut ([u128; 5], i16, i8, (i16, f32), bool);
let _23: Adt46;
let _24: bool;
let _25: usize;
let _26: f64;
let _27: Adt55;
let _28: f64;
let _29: Adt52;
let _30: [i64; 5];
let _31: Adt56;
let _32: [u32; 1];
let _33: *mut *const (u64, char);
let _34: [u32; 3];
let _35: u32;
let _36: isize;
let _37: [bool; 7];
let _38: i128;
let _39: f32;
let _40: bool;
let _41: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _42: [u32; 1];
let _43: *const i8;
let _44: i128;
let _45: (i16, bool, u64);
let _46: ((u64, char), i128, (i16, f32));
let _47: (i16, f32);
let _48: isize;
let _49: (bool, [i64; 5], u128);
let _50: f64;
let _51: isize;
let _52: isize;
let _53: i32;
let _54: f32;
let _55: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64));
let _56: [i32; 1];
let _57: f32;
let _58: (usize, i16, i64);
let _59: [i32; 1];
let _60: char;
let _61: isize;
let _62: isize;
let _63: isize;
let _64: [i64; 5];
let _65: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _66: [u128; 5];
let _67: [u32; 3];
let _68: isize;
let _69: f64;
let _70: char;
let _71: char;
let _72: (i16, bool, u64);
let _73: [bool; 7];
let _74: isize;
let _75: char;
let _76: i32;
let _77: (bool, [i64; 5], u128);
let _78: isize;
let _79: (bool, u32, bool, u32);
let _80: isize;
let _81: f64;
let _82: [u32; 1];
let _83: u128;
let _84: char;
let _85: [bool; 7];
let _86: f64;
let _87: (i16, bool, u64);
let _88: char;
let _89: ();
let _90: ();
{
_5.2 = 1_usize as u64;
_5.0 = _3.2.0;
_5 = (_3.2.0, _2, _3.0.0);
_5.1 = !_9;
_4 = _3.0;
_3.2.0 = _5.0 | _5.0;
_2 = _5.1;
_3.0.0 = !_4.0;
_5.0 = -_3.2.0;
_5.1 = _4.0 == _5.2;
_3.1 = (-116764421176993965556064588359238877408_i128) >> _5.0;
_3.2.1 = _5.0 as f32;
_2 = !_9;
_8 = _6;
_9 = !_5.1;
_7 = !_2;
_12.2.0 = _5.0 ^ _5.0;
_12.0.0 = _5.2;
_5 = (_12.2.0, _2, _4.0);
_4.1 = _3.0.1;
_10 = _6 * _8;
_11 = 10807981811603744003_usize as f32;
_6 = _10 << _3.0.0;
_2 = !_1;
Goto(bb1)
}
bb1 = {
_12 = (_4, _3.1, _3.2);
_18.1 = [(-2821731820456114514_i64),7450620246140650049_i64,6615123398588643046_i64,5203428500135737374_i64,3779799694655095227_i64];
_4.1 = _3.0.1;
_5 = (_3.2.0, _9, _12.0.0);
_12.0.1 = _3.0.1;
_5 = (_3.2.0, _7, _12.0.0);
_20 = 28_u8 * 178_u8;
_6 = _8 >> _12.0.0;
_7 = _5.1;
_5 = (_3.2.0, _2, _3.0.0);
_19 = _12.0.1;
_13.0 = !_5.0;
_12.0.0 = _4.0;
Goto(bb2)
}
bb2 = {
_8 = _10 | _6;
_1 = _7 & _7;
_12 = _3;
_1 = _2 != _7;
_5.1 = !_1;
_18.2 = !250084152001975785797148228664514502357_u128;
_12.1 = _3.1;
_9 = !_1;
_13.1 = -_12.2.1;
_11 = _12.2.1 - _3.2.1;
_12.0.0 = !_3.0.0;
_15 = _4.1;
_1 = _2 & _5.1;
_13.1 = _3.2.1 - _11;
_3.0 = (_5.2, _12.0.1);
_12.0 = _3.0;
Goto(bb3)
}
bb3 = {
_3.1 = _12.1;
_3.0.1 = _4.1;
_20 = 186_u8;
_3.2 = (_12.2.0, _13.1);
_12.0.0 = _3.0.0;
_12.0.1 = _3.0.1;
Call(_14 = fn19(_12.1, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = _19;
_1 = _5.1;
_10 = _3.1 as isize;
_5 = (_3.2.0, Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4, _12.0.0);
_11 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as f32;
_17 = _5.0 as isize;
_3.0.0 = _17 as u64;
_7 = _5.1;
_19 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_3 = _12;
_16 = _8 >> _12.0.0;
_7 = _1 ^ Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4;
_3.0 = (_5.2, _19);
_18.2 = 296227363242349661852817312662984087308_u128;
Goto(bb5)
}
bb5 = {
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.2 = (-40_i8);
_15 = _3.0.1;
place!(Field::<usize>(Variant(_14, 1), 2)) = 16052111858926284843_usize;
_3.0.0 = _13.0 as u64;
_3.0.1 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (_3.2.0, _3.2.1);
_21 = -_12.2.1;
_3.0.0 = !Field::<(u64, char)>(Variant(_14, 1), 1).0;
_13.1 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1 + Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1;
_24 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1 > Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1;
_16 = Field::<i128>(Variant(_14, 1), 3) as isize;
_30 = [4571458424728632468_i64,1659126066247578726_i64,(-8276237310361906957_i64),(-4984078522986350293_i64),(-2281698682096000475_i64)];
_5.2 = !Field::<(u64, char)>(Variant(_14, 1), 1).0;
_23 = Adt46::Variant1 { fld0: _3.2.0,fld1: Field::<usize>(Variant(_14, 1), 2),fld2: Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.2 };
_12.2.1 = _3.2.1 * Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1;
_28 = _4.0 as f64;
Goto(bb6)
}
bb6 = {
Goto(bb7)
}
bb7 = {
SetDiscriminant(_14, 1);
_3.0.1 = _12.0.1;
SetDiscriminant(_23, 1);
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
_8 = _6 >> _10;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.4 = _24;
_7 = _9 < _2;
_7 = !_9;
_12.2.0 = _13.0;
_8 = _10 - _16;
_16 = _17 >> _6;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).1 = !2057198798_u32;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.2 = -38_i8;
place!(Field::<u8>(Variant(_14, 1), 6)) = 4338642276697109194_i64 as u8;
_24 = _1;
_7 = !_24;
_21 = _12.2.1;
_23 = Adt46::Variant1 { fld0: _5.0,fld1: 14143962776445202175_usize,fld2: Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.2 };
place!(Field::<[u32; 3]>(Variant(_14, 1), 5)) = [Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1];
_18.0 = _2;
_19 = _4.1;
_25 = !7_usize;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)).0 = _4.0 >> _17;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)) = (_3.0.0, _15);
_10 = _8;
_18.1 = _30;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (_3.2.0, _3.2.1);
match _18.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
296227363242349661852817312662984087308 => bb13,
_ => bb12
}
}
bb8 = {
Goto(bb7)
}
bb9 = {
_12 = (_4, _3.1, _3.2);
_18.1 = [(-2821731820456114514_i64),7450620246140650049_i64,6615123398588643046_i64,5203428500135737374_i64,3779799694655095227_i64];
_4.1 = _3.0.1;
_5 = (_3.2.0, _9, _12.0.0);
_12.0.1 = _3.0.1;
_5 = (_3.2.0, _7, _12.0.0);
_20 = 28_u8 * 178_u8;
_6 = _8 >> _12.0.0;
_7 = _5.1;
_5 = (_3.2.0, _2, _3.0.0);
_19 = _12.0.1;
_13.0 = !_5.0;
_12.0.0 = _4.0;
Goto(bb2)
}
bb10 = {
_15 = _19;
_1 = _5.1;
_10 = _3.1 as isize;
_5 = (_3.2.0, Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4, _12.0.0);
_11 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as f32;
_17 = _5.0 as isize;
_3.0.0 = _17 as u64;
_7 = _5.1;
_19 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_3 = _12;
_16 = _8 >> _12.0.0;
_7 = _1 ^ Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4;
_3.0 = (_5.2, _19);
_18.2 = 296227363242349661852817312662984087308_u128;
Goto(bb5)
}
bb11 = {
_3.1 = _12.1;
_3.0.1 = _4.1;
_20 = 186_u8;
_3.2 = (_12.2.0, _13.1);
_12.0.0 = _3.0.0;
_12.0.1 = _3.0.1;
Call(_14 = fn19(_12.1, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_8 = _10 | _6;
_1 = _7 & _7;
_12 = _3;
_1 = _2 != _7;
_5.1 = !_1;
_18.2 = !250084152001975785797148228664514502357_u128;
_12.1 = _3.1;
_9 = !_1;
_13.1 = -_12.2.1;
_11 = _12.2.1 - _3.2.1;
_12.0.0 = !_3.0.0;
_15 = _4.1;
_1 = _2 & _5.1;
_13.1 = _3.2.1 - _11;
_3.0 = (_5.2, _12.0.1);
_12.0 = _3.0;
Goto(bb3)
}
bb13 = {
_9 = _1 < Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4;
_3.0 = (Field::<(u64, char)>(Variant(_14, 1), 1).0, Field::<(u64, char)>(Variant(_14, 1), 1).1);
_20 = Field::<u8>(Variant(_14, 1), 6) * Field::<u8>(Variant(_14, 1), 6);
place!(Field::<i16>(Variant(_14, 1), 4)) = _1 as i16;
_5.0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.0 + _12.2.0;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)).1 = _12.0.1;
_15 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_12.1 = _3.1 + _3.1;
_5.0 = !Field::<i16>(Variant(_14, 1), 4);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.4 = _1 ^ _7;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<i16>(Variant(_14, 1), 4)) = _13.0;
_32 = [Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1];
_40 = _5.1;
Goto(bb14)
}
bb14 = {
_12.1 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as i128;
_13.1 = _3.1 as f32;
_13.0 = !_12.2.0;
_41.0.4 = !_24;
Goto(bb15)
}
bb15 = {
place!(Field::<usize>(Variant(_23, 1), 1)) = Field::<i8>(Variant(_23, 1), 2) as usize;
_6 = _8;
_19 = _3.0.1;
_3.1 = _12.1 - _12.1;
_22 = core::ptr::addr_of_mut!(_41.0);
_41.0.3.0 = _18.2 as i16;
(*_22).0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.0;
_2 = _13.0 < _3.2.0;
place!(Field::<i128>(Variant(_14, 1), 3)) = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as i128;
_4 = (_5.2, _12.0.1);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (Field::<i16>(Variant(_23, 1), 0), _3.2.1);
(*_22) = (Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.0, _12.2.0, Field::<i8>(Variant(_23, 1), 2), _13, _2);
_18.0 = _41.0.4;
_43 = core::ptr::addr_of!((*_22).2);
_43 = core::ptr::addr_of!((*_22).2);
Goto(bb16)
}
bb16 = {
_21 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.1 = 366_u16 as i16;
_41.0.4 = !_7;
place!(Field::<i8>(Variant(_23, 1), 2)) = (*_43) + (*_22).2;
_41 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0);
(*_22).3.1 = _3.2.1;
_6 = _20 as isize;
_12 = _3;
_3.0.0 = _5.2 << _12.0.0;
SetDiscriminant(_23, 0);
_18.1 = _30;
_21 = _41.0.3.1;
_25 = Field::<(u64, char)>(Variant(_14, 1), 1).1 as usize;
place!(Field::<isize>(Variant(_23, 0), 2)) = !_10;
match _18.2 {
0 => bb10,
1 => bb7,
296227363242349661852817312662984087308 => bb18,
_ => bb17
}
}
bb17 = {
_12 = (_4, _3.1, _3.2);
_18.1 = [(-2821731820456114514_i64),7450620246140650049_i64,6615123398588643046_i64,5203428500135737374_i64,3779799694655095227_i64];
_4.1 = _3.0.1;
_5 = (_3.2.0, _9, _12.0.0);
_12.0.1 = _3.0.1;
_5 = (_3.2.0, _7, _12.0.0);
_20 = 28_u8 * 178_u8;
_6 = _8 >> _12.0.0;
_7 = _5.1;
_5 = (_3.2.0, _2, _3.0.0);
_19 = _12.0.1;
_13.0 = !_5.0;
_12.0.0 = _4.0;
Goto(bb2)
}
bb18 = {
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
place!(Field::<Adt45>(Variant(_23, 0), 1)) = Adt45::Variant0 { fld0: _12.1,fld1: (*_22).0,fld2: _16,fld3: _12.0.0,fld4: _18,fld5: (-1070891464_i32),fld6: _30 };
match _18.2 {
0 => bb13,
1 => bb6,
2 => bb9,
3 => bb7,
4 => bb19,
296227363242349661852817312662984087308 => bb21,
_ => bb20
}
}
bb19 = {
_12 = (_4, _3.1, _3.2);
_18.1 = [(-2821731820456114514_i64),7450620246140650049_i64,6615123398588643046_i64,5203428500135737374_i64,3779799694655095227_i64];
_4.1 = _3.0.1;
_5 = (_3.2.0, _9, _12.0.0);
_12.0.1 = _3.0.1;
_5 = (_3.2.0, _7, _12.0.0);
_20 = 28_u8 * 178_u8;
_6 = _8 >> _12.0.0;
_7 = _5.1;
_5 = (_3.2.0, _2, _3.0.0);
_19 = _12.0.1;
_13.0 = !_5.0;
_12.0.0 = _4.0;
Goto(bb2)
}
bb20 = {
SetDiscriminant(_14, 1);
_3.0.1 = _12.0.1;
SetDiscriminant(_23, 1);
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
_8 = _6 >> _10;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.4 = _24;
_7 = _9 < _2;
_7 = !_9;
_12.2.0 = _13.0;
_8 = _10 - _16;
_16 = _17 >> _6;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).1 = !2057198798_u32;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.2 = -38_i8;
place!(Field::<u8>(Variant(_14, 1), 6)) = 4338642276697109194_i64 as u8;
_24 = _1;
_7 = !_24;
_21 = _12.2.1;
_23 = Adt46::Variant1 { fld0: _5.0,fld1: 14143962776445202175_usize,fld2: Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.2 };
place!(Field::<[u32; 3]>(Variant(_14, 1), 5)) = [Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1];
_18.0 = _2;
_19 = _4.1;
_25 = !7_usize;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)).0 = _4.0 >> _17;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)) = (_3.0.0, _15);
_10 = _8;
_18.1 = _30;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (_3.2.0, _3.2.1);
match _18.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
296227363242349661852817312662984087308 => bb13,
_ => bb12
}
}
bb21 = {
_46.0.0 = Field::<u64>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 3) | Field::<u64>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 3);
Goto(bb22)
}
bb22 = {
_47.1 = -_41.0.3.1;
_46.2.1 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.1 + (*_22).3.1;
_12.1 = _8 as i128;
_4.1 = _19;
_13.0 = -(*_22).3.0;
_5.2 = _3.0.0;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)) = (_5.2, _3.0.1);
_37 = [(*_22).4,_40,_5.1,(*_22).4,_1,(*_22).4,(*_22).4];
(*_22).3.1 = _47.1 + _47.1;
(*_43) = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.2;
(*_22).2 = 56489791747138044_i64 as i8;
(*_22).3 = (Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.0, _13.1);
_45.0 = _3.2.0;
match Field::<(bool, [i64; 5], u128)>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 4).2 {
0 => bb1,
1 => bb2,
2 => bb21,
3 => bb23,
4 => bb24,
5 => bb25,
296227363242349661852817312662984087308 => bb27,
_ => bb26
}
}
bb23 = {
_12.1 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as i128;
_13.1 = _3.1 as f32;
_13.0 = !_12.2.0;
_41.0.4 = !_24;
Goto(bb15)
}
bb24 = {
_8 = _10 | _6;
_1 = _7 & _7;
_12 = _3;
_1 = _2 != _7;
_5.1 = !_1;
_18.2 = !250084152001975785797148228664514502357_u128;
_12.1 = _3.1;
_9 = !_1;
_13.1 = -_12.2.1;
_11 = _12.2.1 - _3.2.1;
_12.0.0 = !_3.0.0;
_15 = _4.1;
_1 = _2 & _5.1;
_13.1 = _3.2.1 - _11;
_3.0 = (_5.2, _12.0.1);
_12.0 = _3.0;
Goto(bb3)
}
bb25 = {
_15 = _19;
_1 = _5.1;
_10 = _3.1 as isize;
_5 = (_3.2.0, Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4, _12.0.0);
_11 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as f32;
_17 = _5.0 as isize;
_3.0.0 = _17 as u64;
_7 = _5.1;
_19 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_3 = _12;
_16 = _8 >> _12.0.0;
_7 = _1 ^ Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4;
_3.0 = (_5.2, _19);
_18.2 = 296227363242349661852817312662984087308_u128;
Goto(bb5)
}
bb26 = {
place!(Field::<usize>(Variant(_23, 1), 1)) = Field::<i8>(Variant(_23, 1), 2) as usize;
_6 = _8;
_19 = _3.0.1;
_3.1 = _12.1 - _12.1;
_22 = core::ptr::addr_of_mut!(_41.0);
_41.0.3.0 = _18.2 as i16;
(*_22).0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.0;
_2 = _13.0 < _3.2.0;
place!(Field::<i128>(Variant(_14, 1), 3)) = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as i128;
_4 = (_5.2, _12.0.1);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (Field::<i16>(Variant(_23, 1), 0), _3.2.1);
(*_22) = (Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.0, _12.2.0, Field::<i8>(Variant(_23, 1), 2), _13, _2);
_18.0 = _41.0.4;
_43 = core::ptr::addr_of!((*_22).2);
_43 = core::ptr::addr_of!((*_22).2);
Goto(bb16)
}
bb27 = {
_13.1 = _46.2.1;
_49.2 = _28 as u128;
_17 = (-379296481183663663_i64) as isize;
_50 = _28;
_39 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as f32;
_38 = !_12.1;
place!(Field::<usize>(Variant(_23, 0), 0)) = _25;
Goto(bb28)
}
bb28 = {
_3.2.0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.0 * (*_22).3.0;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3.0 = 40977_u16 as i16;
_48 = Field::<isize>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 2);
_36 = Field::<isize>(Variant(_23, 0), 2);
_54 = _13.1 - (*_22).3.1;
(*_22).3.1 = Field::<isize>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 2) as f32;
_55.5.1 = !(*_22).4;
_41.0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0;
_45.1 = Field::<(bool, [i64; 5], u128)>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 4).0;
_12.0.0 = _46.0.0;
_10 = Field::<isize>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 2);
_41.0.3.0 = -_12.2.0;
_49 = Field::<(bool, [i64; 5], u128)>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 4);
_55.5.0 = (*_22).3.0 ^ _45.0;
_16 = _10;
_40 = _5.1 & _2;
_12.0 = Field::<(u64, char)>(Variant(_14, 1), 1);
_41.0.4 = _46.0.0 >= Field::<u64>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 3);
match _18.2 {
0 => bb27,
1 => bb9,
2 => bb12,
3 => bb4,
4 => bb5,
5 => bb6,
296227363242349661852817312662984087308 => bb29,
_ => bb17
}
}
bb29 = {
_55.2 = !_55.5.1;
_55.1 = -_47.1;
_7 = _45.1;
_5.0 = Field::<i16>(Variant(_14, 1), 4);
_45.2 = Field::<u64>(Variant(Field::<Adt45>(Variant(_23, 0), 1), 0), 3) >> _8;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3.0 = _41.0.3.0;
_53 = _12.1 as i32;
_39 = 24031_u16 as f32;
_25 = _12.1 as usize;
_23 = Adt46::Variant1 { fld0: _3.2.0,fld1: _25,fld2: (*_43) };
_3.0.1 = _19;
_42 = [_41.1];
_55.2 = _24;
_37 = [(*_22).4,_1,(*_22).4,_9,_7,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4,_40];
_3.2.1 = _12.2.1 + _55.1;
place!(Field::<u8>(Variant(_14, 1), 6)) = _20;
match _49.2 {
0 => bb16,
1 => bb12,
2 => bb30,
296227363242349661852817312662984087308 => bb32,
_ => bb31
}
}
bb30 = {
_9 = _1 < Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4;
_3.0 = (Field::<(u64, char)>(Variant(_14, 1), 1).0, Field::<(u64, char)>(Variant(_14, 1), 1).1);
_20 = Field::<u8>(Variant(_14, 1), 6) * Field::<u8>(Variant(_14, 1), 6);
place!(Field::<i16>(Variant(_14, 1), 4)) = _1 as i16;
_5.0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.0 + _12.2.0;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)).1 = _12.0.1;
_15 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_12.1 = _3.1 + _3.1;
_5.0 = !Field::<i16>(Variant(_14, 1), 4);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.4 = _1 ^ _7;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<i16>(Variant(_14, 1), 4)) = _13.0;
_32 = [Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1];
_40 = _5.1;
Goto(bb14)
}
bb31 = {
SetDiscriminant(_14, 1);
_3.0.1 = _12.0.1;
SetDiscriminant(_23, 1);
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
_8 = _6 >> _10;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.4 = _24;
_7 = _9 < _2;
_7 = !_9;
_12.2.0 = _13.0;
_8 = _10 - _16;
_16 = _17 >> _6;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).1 = !2057198798_u32;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.2 = -38_i8;
place!(Field::<u8>(Variant(_14, 1), 6)) = 4338642276697109194_i64 as u8;
_24 = _1;
_7 = !_24;
_21 = _12.2.1;
_23 = Adt46::Variant1 { fld0: _5.0,fld1: 14143962776445202175_usize,fld2: Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.2 };
place!(Field::<[u32; 3]>(Variant(_14, 1), 5)) = [Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1];
_18.0 = _2;
_19 = _4.1;
_25 = !7_usize;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)).0 = _4.0 >> _17;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.0 = [_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<u8>(Variant(_14, 1), 6)) = !_20;
place!(Field::<(u64, char)>(Variant(_14, 1), 1)) = (_3.0.0, _15);
_10 = _8;
_18.1 = _30;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (_3.2.0, _3.2.1);
match _18.2 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
296227363242349661852817312662984087308 => bb13,
_ => bb12
}
}
bb32 = {
_54 = _20 as f32;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0 = (_41.0.0, _5.0, Field::<i8>(Variant(_23, 1), 2), _13, _5.1);
(*_22).3.1 = _55.1;
_13 = (_55.5.0, (*_22).3.1);
SetDiscriminant(_23, 0);
_60 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_15 = _60;
_47.0 = Field::<i16>(Variant(_14, 1), 4) | _55.5.0;
_55.1 = _13.1 - _12.2.1;
_57 = _21 - (*_22).3.1;
Goto(bb33)
}
bb33 = {
_5 = (_55.5.0, _7, _12.0.0);
_21 = _3.2.1;
_13.0 = (*_22).3.0;
_41.1 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1;
_63 = _10;
_3 = (_4, _12.1, _47);
_12 = (Field::<(u64, char)>(Variant(_14, 1), 1), _3.1, _3.2);
_66 = [_18.2,_49.2,_49.2,_49.2,_18.2];
(*_22).0 = [_49.2,_18.2,_18.2,_18.2,_18.2];
_55.5 = (Field::<i16>(Variant(_14, 1), 4), _40, _45.2);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.1 = _3.2.0 * Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.3.0;
_65.0.0 = (*_22).0;
_44 = _3.1 >> _13.0;
_18.2 = (*_22).4 as u128;
_26 = -_50;
_5.2 = _55.5.2;
_64 = _18.1;
Goto(bb34)
}
bb34 = {
place!(Field::<(u64, char)>(Variant(_14, 1), 1)).1 = _19;
_65.0 = (*_22);
match _49.2 {
0 => bb22,
296227363242349661852817312662984087308 => bb35,
_ => bb8
}
}
bb35 = {
_52 = 52444_u16 as isize;
_46.1 = _38 & _12.1;
_43 = core::ptr::addr_of!((*_22).2);
_55.4 = _25 & _25;
_5.2 = !_46.0.0;
_12.2.1 = -_55.1;
_55.3 = !_55.4;
match _49.2 {
0 => bb17,
1 => bb18,
2 => bb21,
3 => bb16,
296227363242349661852817312662984087308 => bb37,
_ => bb36
}
}
bb36 = {
Goto(bb7)
}
bb37 = {
_47.0 = _13.0 ^ (*_22).3.0;
_67 = [_41.1,_41.1,Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1];
_58 = (_25, _55.5.0, (-8148818875984050736_i64));
_47.0 = _3.2.0 - _13.0;
_16 = !_36;
_4.1 = _3.0.1;
_3 = _12;
_55.5.1 = !_18.0;
_3.2.1 = -_41.0.3.1;
_51 = _16;
place!(Field::<f64>(Variant(_23, 0), 3)) = _50 - _28;
_41.0.1 = _46.1 as i16;
_59 = [_53];
_21 = _53 as f32;
_49.2 = _18.2 | _18.2;
Goto(bb38)
}
bb38 = {
_65.1 = _41.1 << _13.0;
_29 = Adt52::Variant3 { fld0: _58 };
_41.0.3.1 = _46.2.1;
_45 = ((*_22).1, Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4, Field::<(u64, char)>(Variant(_14, 1), 1).0);
_46.2.0 = _65.0.3.0;
_68 = _36;
_46.2.0 = -_13.0;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3.0 = Field::<(usize, i16, i64)>(Variant(_29, 3), 0).2 as i16;
_55.3 = _58.0;
place!(Field::<[u32; 3]>(Variant(_14, 1), 5)) = _67;
_72.1 = _55.2;
_44 = _3.1;
_8 = _68;
place!(Field::<(usize, i16, i64)>(Variant(_29, 3), 0)).0 = !_25;
_50 = _26 + Field::<f64>(Variant(_23, 0), 3);
_7 = _58.1 == _47.0;
_12 = (Field::<(u64, char)>(Variant(_14, 1), 1), _38, _46.2);
_77.0 = _1;
_71 = _19;
_38 = _50 as i128;
Goto(bb39)
}
bb39 = {
place!(Field::<usize>(Variant(_23, 0), 0)) = _1 as usize;
SetDiscriminant(_29, 1);
_55.0 = [_49.2,_18.2,_18.2,_18.2,_49.2];
place!(Field::<i16>(Variant(_14, 1), 4)) = _12.2.0;
_10 = _12.0.1 as isize;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0 = _65.0;
_46.0.1 = _71;
_20 = Field::<u8>(Variant(_14, 1), 6) - Field::<u8>(Variant(_14, 1), 6);
_79.1 = _18.2 as u32;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3)).3.2 = _24;
_42 = _32;
_80 = _28 as isize;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3)) = (_41.0.2, _65, _43, _55, _18.1, _41);
_18.1 = [_58.2,_58.2,_58.2,_58.2,_58.2];
_65.0.4 = _40;
_78 = _25 as isize;
_41.0.2 = _65.0.2;
_55.0 = [_49.2,_18.2,_18.2,_18.2,_49.2];
_25 = !_58.0;
match _58.2 {
0 => bb5,
1 => bb6,
2 => bb35,
3 => bb30,
4 => bb40,
5 => bb41,
6 => bb42,
340282366920938463455225788555784160720 => bb44,
_ => bb43
}
}
bb40 = {
_15 = _19;
_1 = _5.1;
_10 = _3.1 as isize;
_5 = (_3.2.0, Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4, _12.0.0);
_11 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as f32;
_17 = _5.0 as isize;
_3.0.0 = _17 as u64;
_7 = _5.1;
_19 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_3 = _12;
_16 = _8 >> _12.0.0;
_7 = _1 ^ Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.4;
_3.0 = (_5.2, _19);
_18.2 = 296227363242349661852817312662984087308_u128;
Goto(bb5)
}
bb41 = {
_54 = _20 as f32;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0 = (_41.0.0, _5.0, Field::<i8>(Variant(_23, 1), 2), _13, _5.1);
(*_22).3.1 = _55.1;
_13 = (_55.5.0, (*_22).3.1);
SetDiscriminant(_23, 0);
_60 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_15 = _60;
_47.0 = Field::<i16>(Variant(_14, 1), 4) | _55.5.0;
_55.1 = _13.1 - _12.2.1;
_57 = _21 - (*_22).3.1;
Goto(bb33)
}
bb42 = {
_12 = (_4, _3.1, _3.2);
_18.1 = [(-2821731820456114514_i64),7450620246140650049_i64,6615123398588643046_i64,5203428500135737374_i64,3779799694655095227_i64];
_4.1 = _3.0.1;
_5 = (_3.2.0, _9, _12.0.0);
_12.0.1 = _3.0.1;
_5 = (_3.2.0, _7, _12.0.0);
_20 = 28_u8 * 178_u8;
_6 = _8 >> _12.0.0;
_7 = _5.1;
_5 = (_3.2.0, _2, _3.0.0);
_19 = _12.0.1;
_13.0 = !_5.0;
_12.0.0 = _4.0;
Goto(bb2)
}
bb43 = {
place!(Field::<usize>(Variant(_23, 1), 1)) = Field::<i8>(Variant(_23, 1), 2) as usize;
_6 = _8;
_19 = _3.0.1;
_3.1 = _12.1 - _12.1;
_22 = core::ptr::addr_of_mut!(_41.0);
_41.0.3.0 = _18.2 as i16;
(*_22).0 = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.0;
_2 = _13.0 < _3.2.0;
place!(Field::<i128>(Variant(_14, 1), 3)) = Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).1 as i128;
_4 = (_5.2, _12.0.1);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (Field::<i16>(Variant(_23, 1), 0), _3.2.1);
(*_22) = (Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.0, _12.2.0, Field::<i8>(Variant(_23, 1), 2), _13, _2);
_18.0 = _41.0.4;
_43 = core::ptr::addr_of!((*_22).2);
_43 = core::ptr::addr_of!((*_22).2);
Goto(bb16)
}
bb44 = {
_55 = Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).3;
_37 = [_1,Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).5.0.4,_45.1,Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).3.2,_55.5.1,_77.0,_1];
_79.0 = !_2;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3)).5.0.3.1 = -_12.2.1;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3)).3.0 = _55.0;
_55.5 = (Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).3.5.0, _24, _4.0);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.3 = (_5.0, (*_22).3.1);
_79 = ((*_22).4, Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).1.1, (*_22).4, _65.1);
_12.0.1 = _3.0.1;
Goto(bb45)
}
bb45 = {
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).0.2 = (*_43);
_41.0.2 = !Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0).0.2;
_49.0 = _58.1 > _41.0.3.0;
place!(Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3)).5.0 = (*_22);
_15 = Field::<(u64, char)>(Variant(_14, 1), 1).1;
_12 = _46;
_57 = _21;
(*_22).2 = _26 as i8;
RET = Adt55::Variant2 { fld0: _3.1,fld1: _79.1,fld2: _49.2,fld3: Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).3,fld4: _18 };
_55.2 = !Field::<([u128; 5], f32, bool, usize, usize, (i16, bool, u64))>(Variant(RET, 2), 3).5.1;
_45.2 = Field::<(u64, char)>(Variant(_14, 1), 1).0 ^ _46.0.0;
SetDiscriminant(RET, 2);
RET = Adt55::Variant2 { fld0: _46.1,fld1: Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).1.1,fld2: _49.2,fld3: _55,fld4: _18 };
_81 = Field::<f64>(Variant(_23, 0), 3) + _26;
_84 = _71;
_38 = _50 as i128;
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)).1 = Field::<u32>(Variant(RET, 2), 1);
place!(Field::<(([u128; 5], i16, i8, (i16, f32), bool), u32)>(Variant(_14, 1), 0)) = ((*_22), Field::<u32>(Variant(RET, 2), 1));
_41.0.1 = !Field::<i16>(Variant(_14, 1), 4);
_85 = _37;
_87 = (Field::<(i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32))>(Variant(_29, 1), 3).1.0.3.0, _7, _12.0.0);
(*_22) = _65.0;
place!(Field::<(bool, [i64; 5], u128)>(Variant(RET, 2), 4)) = _18;
(*_22).3.0 = _5.1 as i16;
Goto(bb46)
}
bb46 = {
Call(_89 = dump_var(18_usize, 10_usize, Move(_10), 30_usize, Move(_30), 64_usize, Move(_64), 44_usize, Move(_44)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_89 = dump_var(18_usize, 37_usize, Move(_37), 51_usize, Move(_51), 67_usize, Move(_67), 49_usize, Move(_49)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_89 = dump_var(18_usize, 60_usize, Move(_60), 66_usize, Move(_66), 87_usize, Move(_87), 6_usize, Move(_6)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_89 = dump_var(18_usize, 45_usize, Move(_45), 5_usize, Move(_5), 68_usize, Move(_68), 2_usize, Move(_2)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_89 = dump_var(18_usize, 9_usize, Move(_9), 32_usize, Move(_32), 85_usize, Move(_85), 16_usize, Move(_16)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_89 = dump_var(18_usize, 52_usize, Move(_52), 1_usize, Move(_1), 58_usize, Move(_58), 90_usize, _90), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i128,mut _2: isize) -> Adt44 {
mir! {
type RET = Adt44;
let _3: [bool; 7];
let _4: f64;
let _5: f32;
let _6: bool;
let _7: i64;
let _8: *mut f32;
let _9: (usize, i16, i64);
let _10: isize;
let _11: Adt58;
let _12: u16;
let _13: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _14: f32;
let _15: Adt45;
let _16: isize;
let _17: i32;
let _18: usize;
let _19: [u32; 1];
let _20: bool;
let _21: bool;
let _22: (([u128; 5], i16, i8, (i16, f32), bool), u32);
let _23: i64;
let _24: i64;
let _25: i32;
let _26: Adt59;
let _27: isize;
let _28: [u128; 5];
let _29: Adt51;
let _30: *mut ([u128; 5], i16, i8, (i16, f32), bool);
let _31: [bool; 7];
let _32: i16;
let _33: char;
let _34: u8;
let _35: isize;
let _36: i64;
let _37: (i16, bool, u64);
let _38: u8;
let _39: [u32; 1];
let _40: char;
let _41: (i16, bool, u64);
let _42: char;
let _43: f64;
let _44: Adt60;
let _45: Adt46;
let _46: i128;
let _47: Adt44;
let _48: isize;
let _49: u64;
let _50: Adt46;
let _51: isize;
let _52: *mut *const (u64, char);
let _53: bool;
let _54: (i16, bool, u64);
let _55: Adt46;
let _56: Adt44;
let _57: ((u64, char), i128, (i16, f32));
let _58: [i32; 1];
let _59: (usize, i16, i64);
let _60: Adt54;
let _61: *const i8;
let _62: u32;
let _63: i64;
let _64: (bool, u32, bool, u32);
let _65: bool;
let _66: (i16, f32);
let _67: [i64; 5];
let _68: isize;
let _69: u32;
let _70: f32;
let _71: Adt55;
let _72: isize;
let _73: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _74: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)));
let _75: bool;
let _76: ();
let _77: ();
{
_2 = -(-27_isize);
_1 = '\u{afe5a}' as i128;
_3 = [true,true,false,true,false,false,false];
_1 = (-98384373033329059193194199766127829140_i128);
_3 = [true,false,false,true,true,true,false];
_2 = (-54_isize);
_1 = 205690695_i32 as i128;
_1 = !1560829734059336720466745936935056266_i128;
_2 = 5_isize ^ (-9223372036854775808_isize);
_4 = 1410502895_u32 as f64;
_1 = -(-39152327224623790816670438916414731056_i128);
_3 = [true,false,true,true,false,true,true];
_1 = (-82040231647615488516312095565838231978_i128) * 78287341771041623902721039763535137639_i128;
_5 = _2 as f32;
_4 = 17914_u16 as f64;
_6 = _4 < _4;
_5 = 8935388877978247406_u64 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_1 = !(-98624759096737419165914907676940460440_i128);
_6 = _1 == _1;
Goto(bb1)
}
bb1 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb2 = {
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _9.0 as f64;
_10 = _2 + _2;
_8 = core::ptr::addr_of_mut!(_5);
_9.1 = (-18403_i16);
(*_8) = _1 as f32;
_8 = core::ptr::addr_of_mut!(_5);
_5 = _4 as f32;
_9.2 = _7;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = (620806433955160851_usize, (-30142_i16), _7);
_9.2 = -_7;
match _9.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768181314 => bb8,
_ => bb7
}
}
bb3 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
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
_8 = core::ptr::addr_of_mut!((*_8));
_1 = 71_i8 as i128;
_1 = _9.1 as i128;
_9.0 = _5 as usize;
_7 = _9.2;
_9.0 = _1 as usize;
_11.fld0 = 269357807138764296664410520927880244981_u128 | 4551909779931183658734671697266761604_u128;
_9 = (0_usize, (-23569_i16), _7);
_11.fld1 = _5;
_10 = -_2;
_9 = (6757364834811852040_usize, 5726_i16, _7);
_5 = _11.fld1 + _11.fld1;
_8 = core::ptr::addr_of_mut!(_5);
match _9.0 {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6757364834811852040 => bb15,
_ => bb14
}
}
bb9 = {
Return()
}
bb10 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb14 = {
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _9.0 as f64;
_10 = _2 + _2;
_8 = core::ptr::addr_of_mut!(_5);
_9.1 = (-18403_i16);
(*_8) = _1 as f32;
_8 = core::ptr::addr_of_mut!(_5);
_5 = _4 as f32;
_9.2 = _7;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = (620806433955160851_usize, (-30142_i16), _7);
_9.2 = -_7;
match _9.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768181314 => bb8,
_ => bb7
}
}
bb15 = {
_13.0.4 = !_9.0;
_13.1.4 = _1 as usize;
_13.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.5.0 = !_9.1;
_13.0.5.1 = !_6;
_11.fld3 = (4221575975865133574_u64, '\u{57881}');
_9.0 = !_13.1.4;
_13.0.5 = (_9.1, _6, _11.fld3.0);
_13.1.3 = 41730_u16 as usize;
_13.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.5 = (_13.0.5.0, _13.0.5.1, _13.0.5.2);
_13.1.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.1 = -(*_8);
_1 = (-137509275318949829136467210566091437970_i128);
(*_8) = _13.1.1;
_13.0.5.0 = -_9.1;
_13.0.5.2 = _11.fld3.1 as u64;
_13.0.5.2 = _11.fld3.0 / _11.fld3.0;
_9.2 = !_7;
_13.1.5.1 = _6 & _6;
_11.fld0 = !189053088634238036237631560367062983140_u128;
_13.0 = (_13.1.0, (*_8), _13.1.5.1, _9.0, _13.1.4, _13.1.5);
_13.0.5.1 = !_6;
_13.1 = _13.0;
match _9.1 {
0 => bb1,
1 => bb13,
2 => bb7,
3 => bb14,
4 => bb5,
5726 => bb16,
_ => bb11
}
}
bb16 = {
_13.1.4 = !_9.0;
_10 = _1 as isize;
_16 = _13.0.5.2 as isize;
_7 = _9.2;
match _13.0.5.0 {
0 => bb1,
1 => bb7,
2 => bb9,
5726 => bb17,
_ => bb4
}
}
bb17 = {
_5 = -_13.1.1;
_17 = _11.fld0 as i32;
_13.0.1 = _13.1.1;
_3 = [_13.0.2,_13.0.5.1,_13.1.2,_13.1.5.1,_6,_13.1.2,_13.1.2];
_4 = 24951_u16 as f64;
_13.0.5.2 = _13.1.5.2 | _13.1.5.2;
(*_8) = _9.2 as f32;
_11.fld3.1 = '\u{7adda}';
_12 = !12670_u16;
_13.1.5.1 = _13.0.2 == _13.1.2;
_5 = _1 as f32;
_11.fld3.1 = '\u{55de1}';
_13.0.2 = !_13.1.5.1;
_13.1.5.2 = _13.0.5.2 % _11.fld3.0;
_9.0 = _1 as usize;
_13.1.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_11.fld3 = (_13.0.5.2, '\u{7a962}');
_14 = -_13.1.1;
_9.1 = _13.0.5.0;
_2 = _16;
_1 = 3921722257_u32 as i128;
_18 = _13.1.3;
_18 = _17 as usize;
_9 = (_13.1.3, _13.0.5.0, _7);
_13.1.1 = (-125_i8) as f32;
_13.1.5.1 = !_13.0.2;
_11.fld3.0 = _13.1.5.2 ^ _13.1.5.2;
match _9.1 {
0 => bb5,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
6 => bb23,
5726 => bb25,
_ => bb24
}
}
bb18 = {
_13.1.4 = !_9.0;
_10 = _1 as isize;
_16 = _13.0.5.2 as isize;
_7 = _9.2;
match _13.0.5.0 {
0 => bb1,
1 => bb7,
2 => bb9,
5726 => bb17,
_ => bb4
}
}
bb19 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb20 = {
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _9.0 as f64;
_10 = _2 + _2;
_8 = core::ptr::addr_of_mut!(_5);
_9.1 = (-18403_i16);
(*_8) = _1 as f32;
_8 = core::ptr::addr_of_mut!(_5);
_5 = _4 as f32;
_9.2 = _7;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = (620806433955160851_usize, (-30142_i16), _7);
_9.2 = -_7;
match _9.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768181314 => bb8,
_ => bb7
}
}
bb21 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb22 = {
Return()
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_13.1.1 = -(*_8);
_20 = !_13.1.5.1;
_7 = -_9.2;
_10 = _4 as isize;
match _9.1 {
0 => bb1,
1 => bb5,
2 => bb10,
5726 => bb27,
_ => bb26
}
}
bb26 = {
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _9.0 as f64;
_10 = _2 + _2;
_8 = core::ptr::addr_of_mut!(_5);
_9.1 = (-18403_i16);
(*_8) = _1 as f32;
_8 = core::ptr::addr_of_mut!(_5);
_5 = _4 as f32;
_9.2 = _7;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = (620806433955160851_usize, (-30142_i16), _7);
_9.2 = -_7;
match _9.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768181314 => bb8,
_ => bb7
}
}
bb27 = {
_13.1.3 = _18;
_11.fld1 = -_13.0.1;
_20 = _13.0.2 > _6;
_22.1 = 1174445931_u32 - 56798505_u32;
_1 = _7 as i128;
_2 = _10;
_8 = core::ptr::addr_of_mut!(_14);
_1 = 30919944901883024992409865310787446621_i128 - (-84127233577859751388677383116880368527_i128);
_22.0.4 = !_20;
_22.0.2 = 93_i8;
_13.0.5.0 = !_13.1.5.0;
_22.1 = 4212757294_u32;
(*_8) = 11_u8 as f32;
_11.fld3.1 = '\u{c2a5}';
_19 = [_22.1];
_22.0.1 = _9.1;
(*_8) = _11.fld1;
_13.1.5.1 = _20;
_13.1.5.0 = _9.1 & _13.0.5.0;
_24 = _4 as i64;
_13.0 = (_13.1.0, _11.fld1, _20, _13.1.4, _9.0, _13.1.5);
_7 = (*_8) as i64;
_3 = [_22.0.4,_20,_20,_13.1.2,_22.0.4,_13.1.5.1,_22.0.4];
_23 = _7;
_8 = core::ptr::addr_of_mut!((*_8));
Call(_24 = core::intrinsics::bswap(_23), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_22.0.1 = _13.1.5.0 & _13.1.5.0;
_22.0.3.0 = !_22.0.1;
_1 = _12 as i128;
(*_8) = _5 * _11.fld1;
_4 = _11.fld0 as f64;
_3 = [_22.0.4,_13.0.5.1,_13.0.5.1,_13.0.2,_13.0.5.1,_22.0.4,_13.0.5.1];
_11.fld1 = _14;
_3 = [_13.0.2,_22.0.4,_13.0.5.1,_13.0.5.1,_22.0.4,_22.0.4,_13.0.5.1];
_13.0.5.1 = !_13.1.2;
_27 = _16;
_9.2 = !_23;
_24 = -_23;
_8 = core::ptr::addr_of_mut!(_5);
_13.0.5.1 = _20;
_22.0.4 = _13.0.5.0 == _22.0.1;
_13.0.4 = !_13.0.3;
_28 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.2 = _20;
_22.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_22.0.1 = _22.0.3.0 + _22.0.3.0;
_13.1.1 = -_11.fld1;
_11.fld3 = (_13.1.5.2, '\u{bdd70}');
_13.0.3 = _9.0 + _9.0;
_22.0.3 = (_22.0.1, _13.0.1);
Goto(bb29)
}
bb29 = {
_7 = _9.2;
_22.1 = 2096246812_u32 * 3250961396_u32;
_31 = _3;
(*_8) = _13.1.1;
_13.0.1 = _14;
_3 = _31;
_13.1.5 = (_22.0.3.0, _13.0.5.1, _11.fld3.0);
_11.fld0 = !34178621781444062632386470874349776464_u128;
_5 = _13.0.4 as f32;
_21 = _22.0.4 | _13.0.5.1;
_29 = Adt51::Variant0 { fld0: _13.1.5,fld1: _11.fld3.1 };
_22.1 = _13.1.5.2 as u32;
_6 = _13.0.5.1;
_11.fld2 = core::ptr::addr_of!(_34);
_32 = _22.0.3.0 >> _12;
_28 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_7 = _24;
_9.0 = _2 as usize;
SetDiscriminant(_29, 1);
_22.0.2 = _12 as i8;
_36 = -_23;
_13.1 = (_22.0.0, _13.0.1, _13.0.2, _13.0.3, _13.0.3, _13.0.5);
_6 = _21;
_30 = core::ptr::addr_of_mut!(_22.0);
_2 = (*_30).3.0 as isize;
_14 = _13.0.1;
(*_30).0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
match _9.1 {
0 => bb30,
5726 => bb32,
_ => bb31
}
}
bb30 = {
_5 = 44725963_u32 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = !89_isize;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_4 = _1 as f64;
_3 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _6 as isize;
_9 = (0_usize, (-13233_i16), (-5744368224793028165_i64));
_9.0 = 6_usize;
_5 = 54_i8 as f32;
_9.0 = 6_usize | 7_usize;
_9.2 = 2667364365592937231_i64 * (-1938130941882342509_i64);
_10 = _5 as isize;
_8 = core::ptr::addr_of_mut!(_5);
_9.0 = (*_8) as usize;
_7 = _9.2;
_9.2 = 225935124844203181863115941611143791265_u128 as i64;
_2 = _9.1 as isize;
_1 = (-64578541498507904639763428451419610024_i128) >> _9.1;
(*_8) = _9.2 as f32;
_3 = [_6,_6,_6,_6,_6,_6,_6];
Goto(bb2)
}
bb31 = {
_13.0.4 = !_9.0;
_13.1.4 = _1 as usize;
_13.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.5.0 = !_9.1;
_13.0.5.1 = !_6;
_11.fld3 = (4221575975865133574_u64, '\u{57881}');
_9.0 = !_13.1.4;
_13.0.5 = (_9.1, _6, _11.fld3.0);
_13.1.3 = 41730_u16 as usize;
_13.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.5 = (_13.0.5.0, _13.0.5.1, _13.0.5.2);
_13.1.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.1.1 = -(*_8);
_1 = (-137509275318949829136467210566091437970_i128);
(*_8) = _13.1.1;
_13.0.5.0 = -_9.1;
_13.0.5.2 = _11.fld3.1 as u64;
_13.0.5.2 = _11.fld3.0 / _11.fld3.0;
_9.2 = !_7;
_13.1.5.1 = _6 & _6;
_11.fld0 = !189053088634238036237631560367062983140_u128;
_13.0 = (_13.1.0, (*_8), _13.1.5.1, _9.0, _13.1.4, _13.1.5);
_13.0.5.1 = !_6;
_13.1 = _13.0;
match _9.1 {
0 => bb1,
1 => bb13,
2 => bb7,
3 => bb14,
4 => bb5,
5726 => bb16,
_ => bb11
}
}
bb32 = {
_13.1.3 = _13.0.3 + _13.0.3;
_9 = (_13.1.4, (*_30).1, _36);
_29 = Adt51::Variant0 { fld0: _13.1.5,fld1: _11.fld3.1 };
(*_8) = 35_u8 as f32;
(*_30).4 = !_21;
(*_30).3.1 = -_13.0.1;
_18 = _13.1.3 - _13.1.3;
_13.1.4 = !_13.0.4;
_37.2 = _13.0.5.2;
_18 = _11.fld3.1 as usize;
(*_30).4 = _6 & _21;
place!(Field::<(i16, bool, u64)>(Variant(_29, 0), 0)).0 = Field::<char>(Variant(_29, 0), 1) as i16;
Goto(bb33)
}
bb33 = {
_33 = _11.fld3.1;
(*_8) = _22.0.3.1 + (*_30).3.1;
_10 = _12 as isize;
SetDiscriminant(_29, 0);
(*_30).0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_27 = _2 | _2;
(*_30).3.1 = -_11.fld1;
place!(Field::<(i16, bool, u64)>(Variant(_29, 0), 0)).0 = _13.1.3 as i16;
_3 = _31;
_20 = !(*_30).4;
Call(_37.2 = core::intrinsics::transmute(_2), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_25 = _17 * _17;
place!(Field::<(i16, bool, u64)>(Variant(_29, 0), 0)) = ((*_30).3.0, _20, _13.0.5.2);
_35 = _17 as isize;
_22.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_37.1 = !(*_30).4;
(*_30).1 = Field::<(i16, bool, u64)>(Variant(_29, 0), 0).0 * _32;
_39 = _19;
_3 = [_37.1,Field::<(i16, bool, u64)>(Variant(_29, 0), 0).1,_6,_13.1.5.1,(*_30).4,_37.1,Field::<(i16, bool, u64)>(Variant(_29, 0), 0).1];
_34 = 52_u8 + 220_u8;
_37 = (_9.1, (*_30).4, _13.1.5.2);
_6 = _37.1;
(*_30).2 = (-98_i8) + 63_i8;
(*_30).0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_13.0.4 = _13.1.3 * _9.0;
_2 = Field::<(i16, bool, u64)>(Variant(_29, 0), 0).2 as isize;
_9.1 = (*_30).4 as i16;
_23 = _24;
_8 = core::ptr::addr_of_mut!((*_30).3.1);
_32 = Field::<(i16, bool, u64)>(Variant(_29, 0), 0).0 * (*_30).1;
_13.1.1 = (*_8) + _14;
_22.0.3 = (_9.1, _14);
_37.0 = _22.0.3.0;
_40 = _11.fld3.1;
Goto(bb35)
}
bb35 = {
_19 = [_22.1];
_11.fld2 = core::ptr::addr_of!(_38);
(*_30).1 = !(*_30).3.0;
_22.0.3.1 = -_13.0.1;
_13.1.5 = Field::<(i16, bool, u64)>(Variant(_29, 0), 0);
_27 = _4 as isize;
_13.1.4 = _13.0.3;
_41 = _13.1.5;
_31 = _3;
_41.2 = _13.1.5.2;
_22.0.4 = _41.1;
_37.0 = _12 as i16;
_37.2 = _22.0.4 as u64;
_13.0.5 = (_13.1.5.0, _13.1.5.1, _11.fld3.0);
_5 = -_11.fld1;
(*_30).4 = !_20;
_35 = _2 & _2;
_9.2 = _13.1.5.1 as i64;
place!(Field::<char>(Variant(_29, 0), 1)) = _33;
(*_30).1 = _2 as i16;
(*_30).0 = _28;
_21 = (*_30).3.0 > _9.1;
Goto(bb36)
}
bb36 = {
_11.fld3 = (_37.2, _33);
(*_30).3 = (_41.0, _5);
(*_30).4 = _32 >= _13.0.5.0;
(*_30).3.1 = _13.1.1;
_11.fld3 = (_37.2, Field::<char>(Variant(_29, 0), 1));
_13.0 = ((*_30).0, (*_8), _41.1, _18, _13.1.4, _41);
_32 = -_13.0.5.0;
SetDiscriminant(_29, 0);
(*_30).3.1 = _11.fld1 - _13.0.1;
_33 = _40;
_36 = -_7;
(*_30).4 = _20;
_38 = !_34;
_27 = _10 + _35;
_42 = _33;
_29 = Adt51::Variant0 { fld0: _13.1.5,fld1: _40 };
(*_30).3 = (_41.0, _5);
_13.1.2 = Field::<(i16, bool, u64)>(Variant(_29, 0), 0).1;
_13.1.5.2 = _40 as u64;
_26 = Adt59::Variant0 { fld0: _8,fld1: Move(_29) };
_43 = _4 + _4;
(*_30).1 = _13.0.5.0 >> _13.1.5.0;
place!(Field::<(i16, bool, u64)>(Variant(place!(Field::<Adt51>(Variant(_26, 0), 1)), 0), 0)).1 = !(*_30).4;
_43 = _9.2 as f64;
_9.2 = _36;
_42 = Field::<char>(Variant(Field::<Adt51>(Variant(_26, 0), 1), 0), 1);
_41.1 = !_13.1.2;
_28 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
Call(place!(Field::<(i16, bool, u64)>(Variant(place!(Field::<Adt51>(Variant(_26, 0), 1)), 0), 0)).0 = core::intrinsics::bswap((*_30).1), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
SetDiscriminant(_26, 3);
_38 = _22.1 as u8;
_43 = _4 - _4;
(*_30).3.0 = _41.0;
_11.fld3.1 = _33;
place!(Field::<[u32; 3]>(Variant(_26, 3), 4)) = [_22.1,_22.1,_22.1];
_37 = (_32, (*_30).4, _11.fld3.0);
_45 = Adt46::Variant1 { fld0: (*_30).1,fld1: _9.0,fld2: (*_30).2 };
SetDiscriminant(_45, 1);
_46 = _9.1 as i128;
_13.1.5 = ((*_30).1, _20, _11.fld3.0);
Call(_37.0 = core::intrinsics::transmute(_13.0.5.0), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
_13.0.5.0 = (*_30).3.0;
_22.1 = 3260661874_u32 | 2630277994_u32;
_41.0 = _46 as i16;
_22.0.3.0 = _4 as i16;
_11.fld1 = _13.1.1;
_13.1.5.0 = (*_30).1;
_22.0.3.0 = _9.1;
_34 = !_38;
place!(Field::<i16>(Variant(_45, 1), 0)) = _9.1 << _22.0.3.0;
place!(Field::<[i32; 1]>(Variant(_26, 3), 2)) = [_17];
_13.1.5.2 = _37.2;
_9 = (_13.0.3, (*_30).1, _36);
(*_30).3 = ((*_30).1, _14);
_53 = _6;
_50 = Adt46::Variant1 { fld0: _9.1,fld1: _13.0.3,fld2: _22.0.2 };
_23 = _24;
_27 = -_35;
_5 = _37.2 as f32;
_13.1.2 = !_21;
_11.fld3.0 = _37.2 * _37.2;
_7 = (*_30).2 as i64;
_57.2.1 = -_5;
_49 = _13.1.5.2;
_54 = (_32, _13.1.5.1, _49);
Goto(bb39)
}
bb39 = {
_22.1 = 244846075_u32 + 3031460464_u32;
_22.0.4 = _13.0.2;
(*_30).1 = _13.0.5.0 ^ _32;
_13.1.5.1 = (*_30).4 ^ (*_30).4;
_59.1 = (*_30).1;
_6 = _13.0.5.1;
_13.0.5.0 = _13.1.4 as i16;
_27 = _10 ^ _35;
_14 = _11.fld1;
_37.2 = !_11.fld3.0;
Goto(bb40)
}
bb40 = {
(*_30).4 = _54.1;
_57.0.1 = _33;
_14 = _57.2.1;
_44 = Adt60::Variant2 { fld0: _53,fld1: _22.1,fld2: _9 };
_55 = Move(_50);
Call(_11.fld3.0 = core::intrinsics::bswap(_54.2), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
_51 = _16;
_10 = _22.1 as isize;
(*_30).3 = (_59.1, _5);
_50 = Adt46::Variant1 { fld0: (*_30).1,fld1: _18,fld2: (*_30).2 };
place!(Field::<i8>(Variant(_50, 1), 2)) = Field::<i8>(Variant(_55, 1), 2);
_13.0.4 = !Field::<usize>(Variant(_50, 1), 1);
_40 = _57.0.1;
place!(Field::<usize>(Variant(_26, 3), 5)) = !Field::<usize>(Variant(_50, 1), 1);
_53 = _21 | _6;
_57.0.0 = _49 + _54.2;
_18 = _43 as usize;
_58 = Field::<[i32; 1]>(Variant(_26, 3), 2);
_23 = _22.0.2 as i64;
_21 = _20;
_13.0.5.1 = (*_30).4;
_11.fld3 = _57.0;
_32 = -_9.1;
place!(Field::<i8>(Variant(_26, 3), 3)) = (*_30).2;
_54.0 = _59.1;
place!(Field::<i8>(Variant(_26, 3), 3)) = -(*_30).2;
_37.2 = _34 as u64;
(*_30).3.1 = _5 + _5;
place!(Field::<(usize, i16, i64)>(Variant(_44, 2), 2)) = (_9.0, _22.0.3.0, _36);
Goto(bb42)
}
bb42 = {
place!(Field::<(usize, i16, i64)>(Variant(_44, 2), 2)).2 = _7;
_64.3 = !_22.1;
_23 = _2 as i64;
_3 = [_41.1,_53,_13.1.5.1,_6,_37.1,_13.1.2,_21];
_13.0.5.1 = _13.1.5.1 == _6;
_20 = !_13.1.5.1;
_13.1.3 = _11.fld0 as usize;
_55 = Move(_50);
Call(_41.2 = core::intrinsics::bswap(_13.1.5.2), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
(*_30).4 = _13.0.2;
_35 = _27;
(*_30).0 = _13.0.0;
SetDiscriminant(_55, 0);
_13.1 = ((*_30).0, _22.0.3.1, _6, Field::<usize>(Variant(_26, 3), 5), Field::<(usize, i16, i64)>(Variant(_44, 2), 2).0, _13.0.5);
_5 = (*_8);
_54 = (Field::<i16>(Variant(_45, 1), 0), _13.1.2, _49);
_63 = _7 << _54.0;
_70 = _22.1 as f32;
_13.0.4 = _54.2 as usize;
_13.1.1 = _46 as f32;
_55 = Adt46::Variant1 { fld0: Field::<(usize, i16, i64)>(Variant(_44, 2), 2).1,fld1: _13.0.4,fld2: (*_30).2 };
_13.0.3 = Field::<usize>(Variant(_55, 1), 1) | _13.0.4;
_13.0.5.0 = !Field::<i16>(Variant(_45, 1), 0);
_13.0.0 = [_11.fld0,_11.fld0,_11.fld0,_11.fld0,_11.fld0];
_51 = _17 as isize;
_1 = -_46;
place!(Field::<usize>(Variant(_26, 3), 5)) = _13.0.4;
(*_30).4 = _41.1;
_37.0 = !_41.0;
_59 = (_13.0.4, (*_30).1, _63);
_73.0.3 = _13.0.3 + _13.0.3;
(*_30).2 = !Field::<i8>(Variant(_55, 1), 2);
_73.0.4 = Field::<usize>(Variant(_26, 3), 5);
place!(Field::<f64>(Variant(_26, 3), 1)) = (*_30).2 as f64;
_13.0.5 = _41;
(*_30).3 = (_59.1, _5);
_13.0.4 = _13.0.3;
Goto(bb44)
}
bb44 = {
_73.1.5.1 = _13.0.2;
_57.2 = _22.0.3;
place!(Field::<[i32; 1]>(Variant(_26, 3), 2)) = [_17];
RET = Adt44::Variant1 { fld0: _22,fld1: _11.fld3,fld2: _13.0.3,fld3: _46,fld4: _22.0.3.0,fld5: Field::<[u32; 3]>(Variant(_26, 3), 4),fld6: _34 };
_37.2 = !_11.fld3.0;
_22.0.4 = _21 & _37.1;
_59.1 = Field::<i8>(Variant(_26, 3), 3) as i16;
_22.0.4 = !_13.0.5.1;
(*_30).4 = !_13.1.5.1;
_74.1.3 = !_9.0;
place!(Field::<i16>(Variant(RET, 1), 4)) = _54.0;
Goto(bb45)
}
bb45 = {
Call(_76 = dump_var(19_usize, 21_usize, Move(_21), 35_usize, Move(_35), 18_usize, Move(_18), 3_usize, Move(_3)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_76 = dump_var(19_usize, 2_usize, Move(_2), 39_usize, Move(_39), 58_usize, Move(_58), 37_usize, Move(_37)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_76 = dump_var(19_usize, 34_usize, Move(_34), 33_usize, Move(_33), 31_usize, Move(_31), 53_usize, Move(_53)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_76 = dump_var(19_usize, 28_usize, Move(_28), 54_usize, Move(_54), 17_usize, Move(_17), 46_usize, Move(_46)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_76 = dump_var(19_usize, 38_usize, Move(_38), 9_usize, Move(_9), 63_usize, Move(_63), 77_usize, _77), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(6_usize), std::hint::black_box('\u{6987f}'), std::hint::black_box(6351206091670167298_u64));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *const (u64, char),
fld1: *const i8,
fld2: [u32; 1],
fld3: i128,
fld4: u8,
fld5: i32,
fld6: i64,

},
Variant1{
fld0: (([u128; 5], i16, i8, (i16, f32), bool), u32),
fld1: (u64, char),
fld2: usize,
fld3: i128,
fld4: i16,
fld5: [u32; 3],
fld6: u8,

},
Variant2{
fld0: (bool, u32, bool, u32),
fld1: char,
fld2: isize,
fld3: (u64, char),
fld4: f64,
fld5: (bool, [i64; 5], u128),
fld6: (i16, f32),

},
Variant3{
fld0: *const u8,
fld1: [bool; 7],
fld2: isize,
fld3: *const i8,
fld4: i16,
fld5: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64))),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i128,
fld1: [u128; 5],
fld2: isize,
fld3: u64,
fld4: (bool, [i64; 5], u128),
fld5: i32,
fld6: [i64; 5],

},
Variant1{
fld0: u64,
fld1: (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)),
fld2: [u32; 3],
fld3: usize,

},
Variant2{
fld0: [bool; 7],
fld1: i128,
fld2: isize,
fld3: i8,
fld4: (usize, i16, i64),
fld5: ((i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)), [bool; 7], u64, *mut f32, (u64, char)),

},
Variant3{
fld0: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64))),
fld1: char,
fld2: (bool, u32, bool, u32),
fld3: (i16, bool, u64),
fld4: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)),

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: usize,
fld1: Adt45,
fld2: isize,
fld3: f64,

},
Variant1{
fld0: i16,
fld1: usize,
fld2: i8,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: usize,
fld1: [u32; 1],
fld2: isize,
fld3: Adt44,
fld4: u128,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: Adt47,
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u16,
fld1: Adt44,
fld2: isize,
fld3: (usize, i16, i64),

},
Variant1{
fld0: u32,

},
Variant2{
fld0: *const (u64, char),
fld1: *const i8,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt45,
fld1: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)),
fld2: u16,
fld3: Adt49,
fld4: [u128; 5],
fld5: *const i8,
fld6: i128,

},
Variant1{
fld0: [u32; 3],

},
Variant2{
fld0: *mut f32,
fld1: [i32; 1],
fld2: isize,

},
Variant3{
fld0: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64))),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
fld0: (i16, bool, u64),
fld1: char,

},
Variant1{
fld0: Adt44,

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *mut f32,
fld1: [i64; 5],
fld2: i8,

},
Variant1{
fld0: *const (u64, char),
fld1: char,
fld2: u64,
fld3: (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)),

},
Variant2{
fld0: u16,
fld1: Adt45,
fld2: Adt47,

},
Variant3{
fld0: (usize, i16, i64),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *const (u64, char),
fld1: ([u128; 5], i16, i8, (i16, f32), bool),

},
Variant1{
fld0: bool,
fld1: *const i8,
fld2: u8,
fld3: (u64, char),
fld4: i16,
fld5: Adt51,
fld6: i64,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [i64; 5],

},
Variant1{
fld0: *const u8,

},
Variant2{
fld0: *const i8,
fld1: char,
fld2: f64,
fld3: *const (u64, char),
fld4: f32,
fld5: (i16, bool, u64),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: i64,
fld1: Adt49,
fld2: Adt53,
fld3: (bool, [i64; 5], u128),

},
Variant1{
fld0: (i16, bool, u64),
fld1: Adt48,
fld2: Adt46,
fld3: [bool; 7],
fld4: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)),
fld5: i32,

},
Variant2{
fld0: i128,
fld1: u32,
fld2: u128,
fld3: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)),
fld4: (bool, [i64; 5], u128),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt49,

},
Variant1{
fld0: f32,
fld1: Adt52,
fld2: u128,
fld3: Adt49,

},
Variant2{
fld0: Adt50,
fld1: char,
fld2: *const i8,
fld3: f64,
fld4: *const u8,
fld5: i32,
fld6: [u32; 1],

},
Variant3{
fld0: (usize, i16, i64),
fld1: Adt45,
fld2: (bool, u32, bool, u32),
fld3: Adt44,
fld4: (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)),
fld5: Adt52,
fld6: Adt48,
fld7: [bool; 7],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u32,

},
Variant1{
fld0: i32,

},
Variant2{
fld0: Adt50,
fld1: *const i8,
fld2: isize,
fld3: u16,
fld4: (i8, (([u128; 5], i16, i8, (i16, f32), bool), u32), *const i8, ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), [i64; 5], (([u128; 5], i16, i8, (i16, f32), bool), u32)),
fld5: (bool, u32, bool, u32),
fld6: *const u8,

},
Variant3{
fld0: i16,
fld1: ([u128; 5], f32, bool, usize, usize, (i16, bool, u64)),
fld2: (u64, char),
fld3: [u128; 5],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: u128,
fld1: f32,
fld2: *const u8,
fld3: (u64, char),
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *mut f32,
fld1: Adt51,

},
Variant1{
fld0: Adt58,
fld1: Adt49,
fld2: ((u64, char), i128, (i16, f32)),

},
Variant2{
fld0: (bool, [i64; 5], u128),
fld1: usize,

},
Variant3{
fld0: Adt49,
fld1: f64,
fld2: [i32; 1],
fld3: i8,
fld4: [u32; 3],
fld5: usize,
fld6: [i64; 5],

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: usize,
fld1: char,
fld2: ((u64, char), i128, (i16, f32)),
fld3: [u128; 5],
fld4: u16,

},
Variant1{
fld0: u32,
fld1: char,
fld2: [i64; 5],
fld3: Adt51,
fld4: usize,
fld5: Adt49,
fld6: i64,
fld7: (([u128; 5], f32, bool, usize, usize, (i16, bool, u64)), ([u128; 5], f32, bool, usize, usize, (i16, bool, u64))),

},
Variant2{
fld0: bool,
fld1: u32,
fld2: (usize, i16, i64),

}}

