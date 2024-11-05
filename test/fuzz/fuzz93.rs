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
pub fn fn0(mut _1: bool,mut _2: u64,mut _3: isize,mut _4: i8,mut _5: u8,mut _6: u128) -> f32 {
mir! {
type RET = f32;
let _7: char;
let _8: [u32; 2];
let _9: *const (bool, f64, i8, u64, [u64; 1]);
let _10: char;
let _11: (usize,);
let _12: (bool, f64, i8, u64, [u64; 1]);
let _13: u8;
let _14: (usize,);
let _15: f32;
let _16: u32;
let _17: (bool,);
let _18: isize;
let _19: f32;
let _20: (bool,);
let _21: Adt42;
let _22: ();
let _23: ();
{
_3 = -(-9223372036854775808_isize);
RET = 278425384505486478164699536658450360158_u128 as f32;
_2 = 27293_u16 as u64;
_4 = !16_i8;
_4 = 76_i8 >> _3;
_6 = 138728628581587765761036944397630765986_u128;
_7 = '\u{dcf62}';
RET = _3 as f32;
_2 = 15380332577173680401_u64 + 10676661330599180876_u64;
_7 = '\u{741c5}';
RET = 3819809009_u32 as f32;
_3 = 9223372036854775807_isize - (-7_isize);
_1 = !false;
_2 = (-4584836125829732961_i64) as u64;
_5 = 253_u8;
_2 = 10776626542519111552_u64;
_4 = _2 as i8;
_1 = _3 <= _3;
_6 = 319067686211205540286883400724243878390_u128 | 94191396201244396631643611428057569307_u128;
RET = _4 as f32;
_7 = '\u{cbed}';
_8 = [1593879517_u32,626835487_u32];
RET = _4 as f32;
Goto(bb1)
}
bb1 = {
_7 = '\u{3c772}';
_8 = [2041683107_u32,2532670273_u32];
_5 = 97_u8;
_2 = 16591839726970105747_u64 & 10507810275479944728_u64;
_10 = _7;
_3 = 9223372036854775807_isize;
_1 = false ^ true;
_2 = 10810235408322711889_u64;
_8 = [1760117342_u32,4282451144_u32];
_5 = (-50721889653106634694966934005509192700_i128) as u8;
RET = _2 as f32;
_2 = _1 as u64;
_11.0 = 16573100367711655941_usize;
RET = _5 as f32;
_4 = 9208759528421287657_i64 as i8;
_10 = _7;
_11.0 = 4_usize << _3;
RET = _2 as f32;
_10 = _7;
_12.2 = _4;
_1 = false ^ true;
_12.2 = -_4;
_11.0 = 1_usize & 2_usize;
Call(_1 = fn1(_4, _5, _6, RET, _2, _8, _3, _11.0, _10, _7, RET, _3, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [3583525345_u32,4145176475_u32];
_12.4 = [_2];
_9 = core::ptr::addr_of!(_12);
RET = _12.2 as f32;
(*_9).3 = _2;
_2 = !(*_9).3;
(*_9).1 = RET as f64;
(*_9).1 = 26137928303363755633981907701481374319_i128 as f64;
_11.0 = 14655188729378087950_usize * 3444784090635247594_usize;
(*_9).3 = !_2;
(*_9).2 = _4;
_12.3 = _2;
_12.0 = _1;
(*_9).2 = _4;
RET = _11.0 as f32;
Goto(bb3)
}
bb3 = {
_11 = (3_usize,);
(*_9).2 = -_4;
(*_9).4 = [_12.3];
(*_9).3 = _2;
_12.0 = !_1;
_10 = _7;
(*_9).2 = !_4;
_13 = _5 - _5;
(*_9).0 = !_1;
_12.4 = [_2];
_14 = (_11.0,);
_1 = !(*_9).0;
_4 = (*_9).2;
Goto(bb4)
}
bb4 = {
_6 = 159023187235811523780479725611796183664_u128 >> _13;
_11.0 = _14.0;
_12.0 = _1;
_5 = _13;
_3 = (-9223372036854775808_isize) - 86_isize;
_5 = _6 as u8;
match _14.0 {
0 => bb5,
1 => bb6,
2 => bb7,
4 => bb9,
5 => bb10,
6 => bb11,
3 => bb13,
_ => bb12
}
}
bb5 = {
_11 = (3_usize,);
(*_9).2 = -_4;
(*_9).4 = [_12.3];
(*_9).3 = _2;
_12.0 = !_1;
_10 = _7;
(*_9).2 = !_4;
_13 = _5 - _5;
(*_9).0 = !_1;
_12.4 = [_2];
_14 = (_11.0,);
_1 = !(*_9).0;
_4 = (*_9).2;
Goto(bb4)
}
bb6 = {
_8 = [3583525345_u32,4145176475_u32];
_12.4 = [_2];
_9 = core::ptr::addr_of!(_12);
RET = _12.2 as f32;
(*_9).3 = _2;
_2 = !(*_9).3;
(*_9).1 = RET as f64;
(*_9).1 = 26137928303363755633981907701481374319_i128 as f64;
_11.0 = 14655188729378087950_usize * 3444784090635247594_usize;
(*_9).3 = !_2;
(*_9).2 = _4;
_12.3 = _2;
_12.0 = _1;
(*_9).2 = _4;
RET = _11.0 as f32;
Goto(bb3)
}
bb7 = {
_7 = '\u{3c772}';
_8 = [2041683107_u32,2532670273_u32];
_5 = 97_u8;
_2 = 16591839726970105747_u64 & 10507810275479944728_u64;
_10 = _7;
_3 = 9223372036854775807_isize;
_1 = false ^ true;
_2 = 10810235408322711889_u64;
_8 = [1760117342_u32,4282451144_u32];
_5 = (-50721889653106634694966934005509192700_i128) as u8;
RET = _2 as f32;
_2 = _1 as u64;
_11.0 = 16573100367711655941_usize;
RET = _5 as f32;
_4 = 9208759528421287657_i64 as i8;
_10 = _7;
_11.0 = 4_usize << _3;
RET = _2 as f32;
_10 = _7;
_12.2 = _4;
_1 = false ^ true;
_12.2 = -_4;
_11.0 = 1_usize & 2_usize;
Call(_1 = fn1(_4, _5, _6, RET, _2, _8, _3, _11.0, _10, _7, RET, _3, _8), ReturnTo(bb2), UnwindUnreachable())
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
_12.2 = -_4;
(*_9).0 = !_1;
(*_9).4 = [(*_9).3];
_16 = 506428783_u32 + 3628892585_u32;
_9 = core::ptr::addr_of!((*_9));
(*_9).1 = _6 as f64;
_12.1 = _16 as f64;
_12.1 = _3 as f64;
_15 = -RET;
_6 = 330161583001726821259886815043218748479_u128 >> _12.2;
_1 = !(*_9).0;
_13 = _5;
_15 = _13 as f32;
(*_9).0 = _2 == _12.3;
_3 = _1 as isize;
_20 = (_1,);
_8 = [_16,_16];
_8 = [_16,_16];
match _14.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb19,
_ => bb18
}
}
bb14 = {
_6 = 159023187235811523780479725611796183664_u128 >> _13;
_11.0 = _14.0;
_12.0 = _1;
_5 = _13;
_3 = (-9223372036854775808_isize) - 86_isize;
_5 = _6 as u8;
match _14.0 {
0 => bb5,
1 => bb6,
2 => bb7,
4 => bb9,
5 => bb10,
6 => bb11,
3 => bb13,
_ => bb12
}
}
bb15 = {
Return()
}
bb16 = {
_11 = (3_usize,);
(*_9).2 = -_4;
(*_9).4 = [_12.3];
(*_9).3 = _2;
_12.0 = !_1;
_10 = _7;
(*_9).2 = !_4;
_13 = _5 - _5;
(*_9).0 = !_1;
_12.4 = [_2];
_14 = (_11.0,);
_1 = !(*_9).0;
_4 = (*_9).2;
Goto(bb4)
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_17.0 = !_20.0;
(*_9).3 = _2 - _2;
(*_9).2 = !_4;
(*_9).1 = _11.0 as f64;
Goto(bb20)
}
bb20 = {
Call(_22 = dump_var(0_usize, 6_usize, Move(_6), 13_usize, Move(_13), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_22 = dump_var(0_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 23_usize, _23), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8,mut _2: u8,mut _3: u128,mut _4: f32,mut _5: u64,mut _6: [u32; 2],mut _7: isize,mut _8: usize,mut _9: char,mut _10: char,mut _11: f32,mut _12: isize,mut _13: [u32; 2]) -> bool {
mir! {
type RET = bool;
let _14: f64;
let _15: char;
let _16: [char; 1];
let _17: [i32; 6];
let _18: Adt48;
let _19: u8;
let _20: [i8; 5];
let _21: i32;
let _22: Adt47;
let _23: i16;
let _24: isize;
let _25: u16;
let _26: *const (u8,);
let _27: i32;
let _28: [u64; 1];
let _29: u128;
let _30: *const (bool, f64, i8, u64, [u64; 1]);
let _31: (u16, bool);
let _32: f32;
let _33: (f32, f64, i128, f32);
let _34: bool;
let _35: f32;
let _36: Adt52;
let _37: (bool,);
let _38: i128;
let _39: [isize; 5];
let _40: bool;
let _41: (bool,);
let _42: f64;
let _43: *mut (bool, f64, i8, u64, [u64; 1]);
let _44: isize;
let _45: ([u64; 1],);
let _46: [u64; 1];
let _47: f32;
let _48: ();
let _49: ();
{
RET = !false;
_12 = _7 << _3;
_7 = _11 as isize;
_10 = _9;
_3 = !88162059022908122260420727696391501166_u128;
_5 = _4 as u64;
RET = !true;
_5 = !10294314054684365395_u64;
_9 = _10;
_6 = [2218707355_u32,106654004_u32];
_12 = -_7;
_4 = 4586_i16 as f32;
RET = true;
_8 = 887554023695399670_usize;
_1 = (-23_i8) >> _7;
_7 = _12;
_13 = [3662821132_u32,141323900_u32];
_2 = !48_u8;
_11 = _4;
_11 = -_4;
Goto(bb1)
}
bb1 = {
_6 = [2242838467_u32,346936070_u32];
_14 = 42784_u16 as f64;
_13 = [1262885732_u32,1355623081_u32];
_15 = _10;
_9 = _10;
_8 = 9378644742509986442_usize;
_16 = [_15];
_5 = 7003689283891736824_u64;
_10 = _15;
_6 = [3490806476_u32,1907143258_u32];
_19 = _8 as u8;
_1 = 555188239_i32 as i8;
_12 = _7 * _7;
Call(_20 = fn2(_10, _13, _4, _15, _5, _7, _5, _12, RET, _5, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = _12 as f64;
_16 = [_9];
_9 = _15;
_3 = 183275977314741438210094689998697918417_u128 * 335180178155595132478452745432972657001_u128;
_4 = _11;
Goto(bb3)
}
bb3 = {
_12 = _7;
_17 = [1326795955_i32,1833326964_i32,2062470319_i32,1032268039_i32,(-1500152584_i32),(-694798223_i32)];
_7 = _12 * _12;
_10 = _15;
_2 = _5 as u8;
_19 = _2;
_5 = !2364004243187006444_u64;
_16 = [_15];
_21 = !(-73885981_i32);
_8 = 4_usize;
_9 = _15;
_6 = [2734591753_u32,1021504678_u32];
_27 = _17[_8];
_16 = [_10];
_12 = -_7;
_17[_8] = _27;
_9 = _15;
_15 = _9;
_24 = _12;
_27 = _17[_8] ^ _17[_8];
_12 = _7;
_20[_8] = _1 | _1;
_23 = 2646972274_u32 as i16;
_25 = 18700_u16;
_28 = [_5];
match _17[_8] {
0 => bb2,
1 => bb4,
340282366920938463463374607430268058872 => bb6,
_ => bb5
}
}
bb4 = {
_14 = _12 as f64;
_16 = [_9];
_9 = _15;
_3 = 183275977314741438210094689998697918417_u128 * 335180178155595132478452745432972657001_u128;
_4 = _11;
Goto(bb3)
}
bb5 = {
_6 = [2242838467_u32,346936070_u32];
_14 = 42784_u16 as f64;
_13 = [1262885732_u32,1355623081_u32];
_15 = _10;
_9 = _10;
_8 = 9378644742509986442_usize;
_16 = [_15];
_5 = 7003689283891736824_u64;
_10 = _15;
_6 = [3490806476_u32,1907143258_u32];
_19 = _8 as u8;
_1 = 555188239_i32 as i8;
_12 = _7 * _7;
Call(_20 = fn2(_10, _13, _4, _15, _5, _7, _5, _12, RET, _5, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_6 = [581182808_u32,34487888_u32];
_19 = !_2;
_20[_8] = _1 & _1;
_17[_8] = !_27;
_9 = _15;
_25 = !58884_u16;
_19 = _2 >> _27;
_1 = _20[_8] + _20[_8];
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
_ => bb5
}
}
bb7 = {
_5 = !8309947127102439005_u64;
RET = !false;
_23 = 20635_i16;
_14 = _1 as f64;
_2 = !_19;
_21 = _5 as i32;
_14 = _12 as f64;
Call(_2 = core::intrinsics::bswap(_19), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_27 = -_17[_8];
_7 = _24;
_29 = _3;
_27 = _21 | _17[_8];
_24 = !_7;
_14 = _7 as f64;
_11 = -_4;
_31 = (_25, RET);
_27 = _17[_8] << _29;
_23 = _5 as i16;
_25 = _5 as u16;
_20[_8] = _5 as i8;
_10 = _9;
_7 = _8 as isize;
_33.2 = 44561750994441787860381931245266094324_i128 * 159027558022685574421003621706974769336_i128;
_3 = 6796111533386244550_i64 as u128;
Call(_29 = core::intrinsics::transmute(_33.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = !_31.1;
_28 = [_5];
_13 = [3129099761_u32,3965718886_u32];
_32 = _25 as f32;
Goto(bb10)
}
bb10 = {
_13 = [1261269903_u32,2388204184_u32];
_4 = -_11;
RET = _31.1;
_15 = _9;
_33.3 = _31.0 as f32;
_27 = _17[_8];
_33.1 = _5 as f64;
_1 = _20[_8];
_2 = !_19;
_6 = [149150891_u32,3003432761_u32];
_20 = [_1,_1,_1,_1,_1];
_34 = RET;
_37 = (RET,);
Goto(bb11)
}
bb11 = {
_3 = _29 + _29;
_37 = (_34,);
_37 = (RET,);
_39 = [_12,_12,_12,_12,_24];
_33 = (_11, _14, (-39098075289633120418380291480134000235_i128), _11);
_33.3 = _11 - _33.0;
_39 = [_12,_24,_24,_12,_24];
_16 = [_15];
_32 = _24 as f32;
_38 = _33.2;
_31 = (_25, _37.0);
_38 = _33.2 - _33.2;
_41 = (RET,);
_31 = (_25, RET);
_33 = (_32, _14, _38, _32);
_4 = _23 as f32;
_33.1 = -_14;
_1 = _20[_8] << _38;
_28 = [_5];
_33.2 = _38;
_9 = _15;
RET = _10 > _9;
_44 = _12;
Goto(bb12)
}
bb12 = {
_7 = !_39[_8];
_12 = _39[_8] ^ _39[_8];
_13 = [3968525639_u32,2796695598_u32];
_5 = !6884944866607316798_u64;
RET = _38 == _38;
_24 = !_12;
_24 = RET as isize;
_31.0 = !_25;
_44 = _24 + _24;
_39[_8] = _24;
_31.0 = _25;
_8 = _14 as usize;
_44 = _33.0 as isize;
Goto(bb13)
}
bb13 = {
Call(_48 = dump_var(1_usize, 39_usize, Move(_39), 15_usize, Move(_15), 41_usize, Move(_41), 3_usize, Move(_3)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_48 = dump_var(1_usize, 9_usize, Move(_9), 2_usize, Move(_2), 31_usize, Move(_31), 25_usize, Move(_25)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_48 = dump_var(1_usize, 10_usize, Move(_10), 24_usize, Move(_24), 34_usize, Move(_34), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(1_usize, 28_usize, Move(_28), 29_usize, Move(_29), 20_usize, Move(_20), 49_usize, _49), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: char,mut _2: [u32; 2],mut _3: f32,mut _4: char,mut _5: u64,mut _6: isize,mut _7: u64,mut _8: isize,mut _9: bool,mut _10: u64,mut _11: bool) -> [i8; 5] {
mir! {
type RET = [i8; 5];
let _12: bool;
let _13: char;
let _14: Adt40;
let _15: char;
let _16: Adt52;
let _17: *mut (bool, f64, i8, u64, [u64; 1]);
let _18: isize;
let _19: Adt42;
let _20: Adt56;
let _21: [isize; 5];
let _22: f32;
let _23: *const isize;
let _24: u8;
let _25: Adt46;
let _26: *mut (bool, f64, i8, u64, [u64; 1]);
let _27: *const isize;
let _28: Adt43;
let _29: [i32; 6];
let _30: f32;
let _31: [i8; 5];
let _32: i16;
let _33: u128;
let _34: (f32, f64, i128, f32);
let _35: (bool,);
let _36: i64;
let _37: [char; 1];
let _38: [i32; 6];
let _39: isize;
let _40: ();
let _41: ();
{
_1 = _4;
_9 = !_11;
RET = [105_i8,(-46_i8),(-49_i8),(-25_i8),(-115_i8)];
_4 = _1;
_5 = (-32958840715787583638284039154474142282_i128) as u64;
_7 = _10 * _10;
_3 = 45_u8 as f32;
_2 = [663534350_u32,1518879178_u32];
_10 = !_7;
_7 = _5;
_8 = _6 >> _7;
RET = [(-59_i8),(-56_i8),(-88_i8),(-107_i8),116_i8];
_7 = _10;
RET = [127_i8,(-31_i8),(-93_i8),69_i8,77_i8];
_2 = [2066626329_u32,61588536_u32];
_4 = _1;
_2 = [658560625_u32,1401160626_u32];
RET = [(-45_i8),7_i8,119_i8,(-15_i8),72_i8];
_5 = _10 & _10;
_1 = _4;
RET = [31_i8,86_i8,40_i8,53_i8,4_i8];
Call(_9 = fn3(_10, _6, _7, _5, _5, RET, _11, RET, _2, _6, _5, _7, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _7 as f32;
_6 = 212_u8 as isize;
_13 = _4;
_11 = _9 ^ _9;
_12 = !_9;
_9 = _10 == _7;
_5 = !_10;
RET = [17_i8,(-102_i8),72_i8,112_i8,22_i8];
_12 = !_11;
_6 = (-9_i8) as isize;
_8 = _6;
_1 = _13;
_3 = 992869171_u32 as f32;
Goto(bb2)
}
bb2 = {
_2 = [1406816462_u32,3361249561_u32];
_13 = _4;
_15 = _1;
_6 = _8 | _8;
_7 = 16154659804969367886335996442376937019_u128 as u64;
_8 = -_6;
RET = [24_i8,51_i8,(-7_i8),51_i8,50_i8];
Goto(bb3)
}
bb3 = {
_13 = _4;
RET = [5_i8,(-56_i8),68_i8,(-3_i8),41_i8];
_15 = _1;
_3 = 150070268225839720817292690600576632310_i128 as f32;
_12 = !_11;
RET = [(-67_i8),(-50_i8),4_i8,(-116_i8),(-7_i8)];
_15 = _13;
_3 = 98_u8 as f32;
_11 = _12;
_8 = -_6;
_1 = _15;
_4 = _15;
_13 = _15;
_10 = _5 - _5;
_2 = [2208059104_u32,3195578367_u32];
_11 = _9;
_1 = _15;
_15 = _1;
_18 = _6;
_11 = _9 ^ _12;
_9 = _12;
Goto(bb4)
}
bb4 = {
_9 = !_11;
_18 = _8;
_1 = _15;
_2 = [653011871_u32,3740117630_u32];
_9 = !_12;
_10 = (-27817_i16) as u64;
RET = [112_i8,8_i8,(-102_i8),(-4_i8),(-102_i8)];
_8 = _11 as isize;
_9 = _12 < _12;
_4 = _13;
Goto(bb5)
}
bb5 = {
_5 = _10;
_18 = _9 as isize;
_1 = _13;
RET = [26_i8,(-62_i8),74_i8,31_i8,(-46_i8)];
_2 = [2236974766_u32,3689535207_u32];
_12 = !_9;
_21 = [_8,_8,_18,_18,_8];
_9 = !_11;
_15 = _1;
_6 = _18;
_12 = _11;
_8 = !_18;
_15 = _1;
_18 = _6;
_4 = _1;
_4 = _1;
_6 = _18;
Goto(bb6)
}
bb6 = {
RET = [(-14_i8),41_i8,(-123_i8),(-10_i8),60_i8];
_19 = Adt42::Variant2 { fld0: _10,fld1: (-8629719251210715626_i64),fld2: _21,fld3: _3,fld4: 152108813364605551364831307219473092251_u128 };
_3 = _6 as f32;
RET = [95_i8,60_i8,(-70_i8),80_i8,20_i8];
_12 = _9;
RET = [113_i8,87_i8,64_i8,75_i8,(-39_i8)];
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = _21;
_12 = !_11;
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = [_6,_8,_6,_18,_18];
place!(Field::<i64>(Variant(_19, 2), 1)) = _8 as i64;
_4 = _13;
_22 = -_3;
_9 = _12;
_10 = Field::<u64>(Variant(_19, 2), 0);
Call(_16 = fn4(_9, Field::<i64>(Variant(_19, 2), 1), _18, _3, _3, Field::<[isize; 5]>(Variant(_19, 2), 2), _12, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = _21;
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = [_6,_6,_8,_6,_8];
_26 = core::ptr::addr_of_mut!(place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 1), 1)));
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = _21;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 1), 1)).2 = (-88_i8) & (-108_i8);
_13 = _1;
place!(Field::<[i8; 5]>(Variant(_16.fld1, 1), 0)) = RET;
(*_26).3 = !_5;
_18 = (*_26).0 as isize;
_6 = -_18;
(*_26).3 = !_7;
Goto(bb8)
}
bb8 = {
_22 = Field::<i64>(Variant(_19, 2), 1) as f32;
_5 = _10;
_11 = !(*_26).0;
(*_26).4 = [(*_26).3];
_31 = [Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 1), 1).2,(*_26).2,(*_26).2,(*_26).2,(*_26).2];
Goto(bb9)
}
bb9 = {
_27 = core::ptr::addr_of!(_8);
_25 = Adt46::Variant0 { fld0: _4 };
_29 = [(-1498053236_i32),1621446187_i32,(-354675765_i32),1539350783_i32,1064449699_i32,518407297_i32];
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = _21;
SetDiscriminant(_25, 0);
_1 = _15;
_2 = [24320439_u32,1502168877_u32];
_15 = _13;
(*_27) = _6;
place!(Field::<u128>(Variant(_19, 2), 4)) = 323232797016922932810420045436066344520_u128;
_33 = Field::<u128>(Variant(_19, 2), 4) * Field::<u128>(Variant(_19, 2), 4);
(*_26).2 = (-43_i8) - (-1_i8);
place!(Field::<[isize; 5]>(Variant(_19, 2), 2)) = [(*_27),_8,_6,_18,_6];
_2 = [2216363363_u32,673261585_u32];
_22 = _3;
_5 = (*_26).3 | _10;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 1), 1)).2 = _33 as i8;
SetDiscriminant(_19, 1);
_34.3 = -_3;
_30 = _33 as f32;
_23 = _27;
place!(Field::<(bool,)>(Variant(_19, 1), 3)) = ((*_26).0,);
_11 = !Field::<(bool,)>(Variant(_19, 1), 3).0;
place!(Field::<i16>(Variant(_19, 1), 4)) = (-7002_i16);
match Field::<i16>(Variant(_19, 1), 4) {
340282366920938463463374607431768204454 => bb11,
_ => bb10
}
}
bb10 = {
_22 = Field::<i64>(Variant(_19, 2), 1) as f32;
_5 = _10;
_11 = !(*_26).0;
(*_26).4 = [(*_26).3];
_31 = [Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 1), 1).2,(*_26).2,(*_26).2,(*_26).2,(*_26).2];
Goto(bb9)
}
bb11 = {
_35 = (Field::<(bool,)>(Variant(_19, 1), 3).0,);
_11 = !_35.0;
(*_26).2 = !(-36_i8);
(*_26).2 = 68_i8 << _6;
_6 = (*_27);
(*_26).1 = (*_26).2 as f64;
place!(Field::<(bool,)>(Variant(_19, 1), 3)).0 = (*_26).0;
place!(Field::<(u16, bool)>(Variant(_19, 1), 2)).0 = (*_26).1 as u16;
_36 = 578196300230188268_i64 >> _6;
RET = [(*_26).2,(*_26).2,(*_26).2,Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 1), 1).2,(*_26).2];
SetDiscriminant(_16.fld1, 2);
_24 = !203_u8;
place!(Field::<(u16, bool)>(Variant(_19, 1), 2)).1 = !_11;
place!(Field::<([u64; 1],)>(Variant(_19, 1), 1)).0 = [_5];
place!(Field::<(u16, bool)>(Variant(_19, 1), 2)).0 = !27150_u16;
_39 = _6 >> (*_27);
_17 = _26;
place!(Field::<*mut (bool, f64, i8, u64, [u64; 1])>(Variant(_16.fld1, 2), 4)) = _17;
place!(Field::<char>(Variant(_25, 0), 0)) = _15;
place!(Field::<u32>(Variant(_16.fld1, 2), 1)) = !1258713415_u32;
_23 = core::ptr::addr_of!(_18);
Goto(bb12)
}
bb12 = {
Call(_40 = dump_var(2_usize, 12_usize, Move(_12), 11_usize, Move(_11), 10_usize, Move(_10), 39_usize, Move(_39)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_40 = dump_var(2_usize, 18_usize, Move(_18), 6_usize, Move(_6), 35_usize, Move(_35), 13_usize, Move(_13)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_40 = dump_var(2_usize, 9_usize, Move(_9), 31_usize, Move(_31), 29_usize, Move(_29), 41_usize, _41), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u64,mut _2: isize,mut _3: u64,mut _4: u64,mut _5: u64,mut _6: [i8; 5],mut _7: bool,mut _8: [i8; 5],mut _9: [u32; 2],mut _10: isize,mut _11: u64,mut _12: u64,mut _13: [i8; 5]) -> bool {
mir! {
type RET = bool;
let _14: f64;
let _15: [isize; 5];
let _16: u8;
let _17: (f32, f64, i128, f32);
let _18: i64;
let _19: isize;
let _20: f32;
let _21: Adt44;
let _22: (bool, f64, i8, u64, [u64; 1]);
let _23: u128;
let _24: isize;
let _25: char;
let _26: [i32; 2];
let _27: [i32; 8];
let _28: ();
let _29: ();
{
_3 = _11 >> _4;
RET = !_7;
_14 = 154_u8 as f64;
RET = _7;
_1 = !_4;
_7 = RET | RET;
_7 = !RET;
_3 = _4;
_5 = 5231_u16 as u64;
_4 = _11;
_6 = _13;
_9 = [472216294_u32,738431182_u32];
_8 = _13;
_15 = [_10,_10,_2,_10,_10];
_2 = !_10;
_11 = !_3;
_9 = [585393382_u32,332790193_u32];
_4 = _3 ^ _3;
_9 = [3868722978_u32,637190309_u32];
RET = _7 | _7;
_4 = _1;
_1 = _11 | _5;
_3 = _12;
_12 = _11 + _11;
Call(_1 = core::intrinsics::bswap(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _2;
_3 = 518687457387435372931773920667410962_u128 as u64;
RET = !_7;
_13 = [(-94_i8),33_i8,(-116_i8),(-105_i8),(-13_i8)];
_17.2 = (-20530583191092272407478032911576043450_i128);
RET = _7;
_9 = [1466184371_u32,275249456_u32];
_17.1 = (-2055117145746720799_i64) as f64;
_17.0 = 11436_u16 as f32;
_7 = RET;
_14 = _17.1;
_17.3 = _17.0 * _17.0;
_11 = _12 << _1;
_19 = _10;
_9 = [2465089519_u32,583230937_u32];
_8 = [13_i8,45_i8,(-63_i8),19_i8,80_i8];
_17.0 = -_17.3;
_3 = !_12;
_16 = !80_u8;
_3 = _5 | _11;
Call(_1 = core::intrinsics::transmute(_10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = !_3;
_8 = [(-101_i8),(-33_i8),113_i8,(-34_i8),(-51_i8)];
_2 = 51616740568381247252421162358451713174_u128 as isize;
_21 = Adt44::Variant0 { fld0: _11 };
place!(Field::<u64>(Variant(_21, 0), 0)) = _3;
_15 = [_2,_2,_10,_10,_19];
_5 = _17.2 as u64;
_11 = !_12;
_2 = !_19;
_10 = !_19;
_11 = 1272531767_i32 as u64;
_15 = [_2,_10,_19,_19,_2];
RET = !_7;
_12 = _3 << _2;
_15 = [_2,_19,_2,_19,_10];
_11 = !_4;
_3 = _17.2 as u64;
match _17.2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
319751783729846191055896574520192168006 => bb8,
_ => bb7
}
}
bb3 = {
_10 = _2;
_3 = 518687457387435372931773920667410962_u128 as u64;
RET = !_7;
_13 = [(-94_i8),33_i8,(-116_i8),(-105_i8),(-13_i8)];
_17.2 = (-20530583191092272407478032911576043450_i128);
RET = _7;
_9 = [1466184371_u32,275249456_u32];
_17.1 = (-2055117145746720799_i64) as f64;
_17.0 = 11436_u16 as f32;
_7 = RET;
_14 = _17.1;
_17.3 = _17.0 * _17.0;
_11 = _12 << _1;
_19 = _10;
_9 = [2465089519_u32,583230937_u32];
_8 = [13_i8,45_i8,(-63_i8),19_i8,80_i8];
_17.0 = -_17.3;
_3 = !_12;
_16 = !80_u8;
_3 = _5 | _11;
Call(_1 = core::intrinsics::transmute(_10), ReturnTo(bb2), UnwindUnreachable())
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
_20 = _17.0 * _17.0;
_4 = Field::<u64>(Variant(_21, 0), 0) - _11;
_7 = !RET;
_3 = _12;
_18 = (-387473399839778753_i64) << _12;
_21 = Adt44::Variant0 { fld0: _11 };
_17.3 = _17.2 as f32;
RET = !_7;
_2 = -_10;
_16 = !211_u8;
_17.3 = _20 * _20;
_7 = RET;
Goto(bb9)
}
bb9 = {
_6 = [(-106_i8),89_i8,4_i8,86_i8,(-32_i8)];
_22.0 = _7 ^ RET;
_4 = _17.2 as u64;
_11 = _12;
_18 = (-3526668004127698606_i64);
_10 = (-1720899714_i32) as isize;
_22.1 = -_14;
_9 = [290160235_u32,4024011996_u32];
_8 = [(-30_i8),43_i8,88_i8,(-65_i8),12_i8];
RET = _3 <= Field::<u64>(Variant(_21, 0), 0);
_19 = !_2;
_17 = (_20, _14, (-152631278283169638547097733070913287211_i128), _20);
RET = !_22.0;
_22.4 = [_3];
_15 = [_2,_2,_10,_19,_2];
_20 = _17.0;
_14 = -_17.1;
_8 = [(-2_i8),87_i8,(-31_i8),(-8_i8),(-66_i8)];
_22.2 = 5_usize as i8;
Goto(bb10)
}
bb10 = {
RET = _7 | _22.0;
match _17.2 {
0 => bb7,
1 => bb11,
2 => bb12,
187651088637768824916276874360854924245 => bb14,
_ => bb13
}
}
bb11 = {
_6 = [(-106_i8),89_i8,4_i8,86_i8,(-32_i8)];
_22.0 = _7 ^ RET;
_4 = _17.2 as u64;
_11 = _12;
_18 = (-3526668004127698606_i64);
_10 = (-1720899714_i32) as isize;
_22.1 = -_14;
_9 = [290160235_u32,4024011996_u32];
_8 = [(-30_i8),43_i8,88_i8,(-65_i8),12_i8];
RET = _3 <= Field::<u64>(Variant(_21, 0), 0);
_19 = !_2;
_17 = (_20, _14, (-152631278283169638547097733070913287211_i128), _20);
RET = !_22.0;
_22.4 = [_3];
_15 = [_2,_2,_10,_19,_2];
_20 = _17.0;
_14 = -_17.1;
_8 = [(-2_i8),87_i8,(-31_i8),(-8_i8),(-66_i8)];
_22.2 = 5_usize as i8;
Goto(bb10)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_2 = _22.2 as isize;
_10 = !_2;
_22.3 = _11;
_5 = !Field::<u64>(Variant(_21, 0), 0);
_5 = !_22.3;
_10 = -_19;
_11 = !_12;
_7 = RET | _22.0;
_22.2 = '\u{1000d9}' as i8;
_12 = !_22.3;
_11 = _5 - Field::<u64>(Variant(_21, 0), 0);
_11 = !_5;
SetDiscriminant(_21, 1);
_22.3 = !_11;
_17.1 = _22.1 - _22.1;
_22.3 = _22.2 as u64;
_1 = !_12;
_3 = _11;
place!(Field::<(usize,)>(Variant(_21, 1), 2)) = (5932833962262169966_usize,);
_12 = !_5;
_8 = [_22.2,_22.2,_22.2,_22.2,_22.2];
_11 = _12 + _12;
_22.2 = (-78_i8) * 25_i8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(3_usize, 7_usize, Move(_7), 1_usize, Move(_1), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(3_usize, 11_usize, Move(_11), 19_usize, Move(_19), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: i64,mut _3: isize,mut _4: f32,mut _5: f32,mut _6: [isize; 5],mut _7: bool,mut _8: f32) -> Adt52 {
mir! {
type RET = Adt52;
let _9: &'static i64;
let _10: isize;
let _11: u8;
let _12: [isize; 5];
let _13: f64;
let _14: i128;
let _15: (bool,);
let _16: u8;
let _17: isize;
let _18: i32;
let _19: i128;
let _20: [char; 1];
let _21: usize;
let _22: f64;
let _23: u32;
let _24: i32;
let _25: ([u64; 1],);
let _26: char;
let _27: isize;
let _28: ([u64; 1],);
let _29: [i32; 8];
let _30: [i32; 6];
let _31: u64;
let _32: (bool, f64, i8, u64, [u64; 1]);
let _33: char;
let _34: Adt47;
let _35: Adt55;
let _36: Adt52;
let _37: (u16, bool);
let _38: isize;
let _39: f32;
let _40: *const (u8,);
let _41: u128;
let _42: [u32; 2];
let _43: [i8; 5];
let _44: Adt41;
let _45: u64;
let _46: i16;
let _47: Adt53;
let _48: i128;
let _49: Adt53;
let _50: i128;
let _51: [u64; 1];
let _52: bool;
let _53: bool;
let _54: Adt45;
let _55: Adt47;
let _56: (bool, f64, i8, u64, [u64; 1]);
let _57: i64;
let _58: [i32; 6];
let _59: i128;
let _60: f64;
let _61: f64;
let _62: (usize,);
let _63: usize;
let _64: Adt50;
let _65: f32;
let _66: isize;
let _67: &'static i64;
let _68: *const (bool, f64, i8, u64, [u64; 1]);
let _69: i16;
let _70: *mut i128;
let _71: bool;
let _72: i128;
let _73: [isize; 5];
let _74: [i32; 2];
let _75: f32;
let _76: &'static i64;
let _77: [i32; 8];
let _78: i8;
let _79: [isize; 5];
let _80: [char; 1];
let _81: char;
let _82: [char; 1];
let _83: Adt43;
let _84: (usize,);
let _85: [i32; 6];
let _86: Adt45;
let _87: Adt42;
let _88: f32;
let _89: f64;
let _90: [u64; 1];
let _91: f32;
let _92: (usize,);
let _93: (f32, f64, i128, f32);
let _94: [i32; 6];
let _95: usize;
let _96: f64;
let _97: u32;
let _98: i8;
let _99: isize;
let _100: bool;
let _101: isize;
let _102: [u32; 2];
let _103: [i32; 2];
let _104: Adt51;
let _105: (bool, f64, i8, u64, [u64; 1]);
let _106: Adt56;
let _107: Adt47;
let _108: f32;
let _109: f32;
let _110: Adt48;
let _111: [char; 1];
let _112: isize;
let _113: f32;
let _114: char;
let _115: ();
let _116: ();
{
_4 = -_8;
_5 = -_8;
_4 = -_5;
_6 = [_3,_3,_3,_3,_3];
_9 = &_2;
_9 = &(*_9);
_3 = -79_isize;
_10 = _3 | _3;
_9 = &_2;
_10 = -_3;
_6 = [_3,_10,_3,_3,_10];
Goto(bb1)
}
bb1 = {
_9 = &_2;
_4 = -_8;
_4 = _8 + _5;
_9 = &_2;
_3 = _10 ^ _10;
_1 = _7;
_4 = _5 - _5;
_4 = _8;
_12 = [_10,_10,_3,_3,_3];
_9 = &_2;
_9 = &(*_9);
_4 = 99_u8 as f32;
_8 = _5 + _5;
_12 = [_10,_3,_3,_3,_3];
_6 = [_3,_3,_3,_3,_3];
_13 = (-138457162173335070622997282226801995560_i128) as f64;
Goto(bb2)
}
bb2 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb3 = {
_2 = 6197681639477001887_i64;
_9 = &_2;
match (*_9) {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
6197681639477001887 => bb7,
_ => bb6
}
}
bb4 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb5 = {
_9 = &_2;
_4 = -_8;
_4 = _8 + _5;
_9 = &_2;
_3 = _10 ^ _10;
_1 = _7;
_4 = _5 - _5;
_4 = _8;
_12 = [_10,_10,_3,_3,_3];
_9 = &_2;
_9 = &(*_9);
_4 = 99_u8 as f32;
_8 = _5 + _5;
_12 = [_10,_3,_3,_3,_3];
_6 = [_3,_3,_3,_3,_3];
_13 = (-138457162173335070622997282226801995560_i128) as f64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_15 = (_7,);
_3 = _10;
_16 = _11;
_12 = [_10,_3,_3,_3,_3];
_10 = _3 * _3;
_17 = 6252005557224644749_u64 as isize;
_10 = -_3;
_15.0 = !_7;
_13 = _2 as f64;
_18 = (-935362859_i32);
_9 = &(*_9);
_7 = _15.0 != _15.0;
_18 = 32352_i16 as i32;
_15.0 = !_7;
_20 = ['\u{73820}'];
_12 = [_3,_3,_10,_3,_10];
_17 = 17873_u16 as isize;
_18 = (-1083115645_i32);
_9 = &(*_9);
match _18 {
0 => bb1,
1 => bb8,
2 => bb9,
340282366920938463463374607430685095811 => bb11,
_ => bb10
}
}
bb8 = {
_9 = &_2;
_4 = -_8;
_4 = _8 + _5;
_9 = &_2;
_3 = _10 ^ _10;
_1 = _7;
_4 = _5 - _5;
_4 = _8;
_12 = [_10,_10,_3,_3,_3];
_9 = &_2;
_9 = &(*_9);
_4 = 99_u8 as f32;
_8 = _5 + _5;
_12 = [_10,_3,_3,_3,_3];
_6 = [_3,_3,_3,_3,_3];
_13 = (-138457162173335070622997282226801995560_i128) as f64;
Goto(bb2)
}
bb9 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb10 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb11 = {
_16 = !_11;
_15 = (_7,);
_5 = (*_9) as f32;
_12 = [_3,_17,_10,_10,_17];
match _18 {
0 => bb5,
1 => bb12,
2 => bb13,
340282366920938463463374607430685095811 => bb15,
_ => bb14
}
}
bb12 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb13 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb14 = {
_9 = &_2;
_4 = -_8;
_4 = _8 + _5;
_9 = &_2;
_3 = _10 ^ _10;
_1 = _7;
_4 = _5 - _5;
_4 = _8;
_12 = [_10,_10,_3,_3,_3];
_9 = &_2;
_9 = &(*_9);
_4 = 99_u8 as f32;
_8 = _5 + _5;
_12 = [_10,_3,_3,_3,_3];
_6 = [_3,_3,_3,_3,_3];
_13 = (-138457162173335070622997282226801995560_i128) as f64;
Goto(bb2)
}
bb15 = {
_12 = [_10,_17,_10,_3,_3];
_18 = -(-754361984_i32);
_9 = &(*_9);
_12 = _6;
_9 = &(*_9);
_9 = &(*_9);
_8 = _4;
_4 = -_8;
_16 = _14 as u8;
_20 = ['\u{73698}'];
_11 = 16399_u16 as u8;
_11 = !_16;
_11 = _16 | _16;
_4 = _8 * _8;
_13 = 65298_u16 as f64;
_12 = [_10,_17,_17,_10,_3];
_15 = (_1,);
_11 = _16;
_20 = ['\u{7b164}'];
_22 = _13 - _13;
Call(_15.0 = fn5(_4, _7, _4, _5, _6), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_23 = _2 as u32;
_24 = _18;
Goto(bb17)
}
bb17 = {
_13 = 1341_u16 as f64;
_14 = 111522615428476794723978273787692216078_i128 + (-60157196210077206924185011511688401950_i128);
_25.0 = [12616554940901586578_u64];
_19 = -_14;
_16 = 8580684949617847521_u64 as u8;
_9 = &(*_9);
_3 = _17 >> _14;
_23 = 527250458_u32;
_21 = 2847107299971408911_usize - 2043770040423395586_usize;
_13 = _21 as f64;
_12 = [_10,_17,_3,_3,_17];
_22 = _13 - _13;
_19 = _11 as i128;
_9 = &_2;
_13 = _18 as f64;
_26 = '\u{e26d5}';
_17 = _18 as isize;
_9 = &(*_9);
_20 = [_26];
_21 = (*_9) as usize;
Goto(bb18)
}
bb18 = {
_1 = _15.0;
_19 = !_14;
_12 = [_3,_3,_3,_3,_3];
_21 = !11606740618329780692_usize;
_6 = [_17,_17,_10,_3,_17];
_3 = -_17;
_17 = _10;
_18 = _24;
_30 = [_24,_18,_18,_18,_24,_18];
_7 = _15.0;
_28 = (_25.0,);
_15.0 = _1;
_29 = [_18,_18,_24,_24,_24,_18,_24,_18];
_17 = _10 * _3;
_27 = _10;
_25.0 = _28.0;
_10 = _17;
_7 = !_15.0;
_32 = (_1, _13, 116_i8, 9412403209522274001_u64, _25.0);
_9 = &(*_9);
_32.2 = _21 as i8;
match _32.3 {
0 => bb15,
1 => bb6,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
9412403209522274001 => bb24,
_ => bb23
}
}
bb19 = {
_2 = 6197681639477001887_i64;
_9 = &_2;
match (*_9) {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
6197681639477001887 => bb7,
_ => bb6
}
}
bb20 = {
Return()
}
bb21 = {
_16 = !_11;
_15 = (_7,);
_5 = (*_9) as f32;
_12 = [_3,_17,_10,_10,_17];
match _18 {
0 => bb5,
1 => bb12,
2 => bb13,
340282366920938463463374607430685095811 => bb15,
_ => bb14
}
}
bb22 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb23 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb24 = {
_12 = [_17,_10,_10,_10,_10];
_18 = (-24561_i16) as i32;
_7 = _16 != _11;
_33 = _26;
_22 = _13;
_15.0 = !_1;
_3 = _10 ^ _27;
_9 = &(*_9);
_22 = _4 as f64;
_2 = _19 as i64;
_21 = 3_usize;
_6[_21] = _22 as isize;
Goto(bb25)
}
bb25 = {
_29[_21] = 215566230622652240673694757768139685449_u128 as i32;
_9 = &_2;
_8 = _4;
_32.3 = !759552208708643329_u64;
_37.0 = !23603_u16;
_37.0 = 43603_u16 * 39912_u16;
_18 = _29[_21] - _29[_21];
_6[_21] = _10;
_30 = [_24,_18,_18,_24,_18,_18];
_28 = _25;
_8 = -_4;
_25 = (_32.4,);
_21 = 2557955531172574308_usize >> _11;
_28.0 = [_32.3];
_11 = _16 | _16;
_32.4 = [_32.3];
_14 = _19 ^ _19;
_1 = !_32.0;
_32 = (_1, _22, 82_i8, 7418608544189282147_u64, _25.0);
Call(_13 = fn6(_8, _32.2, _32.2, _32.4, _25, _32.2, _15, _32.0, _32, _20, _19, _32.3, _4, _32, _32.1, _15.0), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_13 = 176940019199574606252665380247972174058_u128 as f64;
_26 = _33;
_21 = !2_usize;
_37.1 = _32.0 ^ _32.0;
_26 = _33;
_32.0 = _1 == _37.1;
_7 = _1;
_17 = !_10;
_30 = [_24,_18,_18,_18,_18,_18];
_7 = _32.0 <= _15.0;
_31 = _32.3 | _32.3;
_26 = _33;
_32.0 = _31 == _31;
_15 = (_32.0,);
_21 = 14608974175737480393_usize - 14387096413773092386_usize;
_41 = 179779279697255420189302570906701325431_u128 * 87971454386546714510028475281485912088_u128;
_19 = !_14;
match _32.3 {
0 => bb11,
1 => bb2,
2 => bb3,
3 => bb13,
4 => bb5,
5 => bb9,
6 => bb14,
7418608544189282147 => bb27,
_ => bb12
}
}
bb27 = {
_15.0 = !_7;
_23 = _8 as u32;
_19 = !_14;
_33 = _26;
_9 = &(*_9);
_12 = [_10,_27,_10,_3,_10];
_42 = [_23,_23];
_4 = _8 + _8;
_28.0 = _32.4;
_22 = _32.1 + _32.1;
_45 = !_31;
_24 = _18 + _18;
_10 = _37.0 as isize;
_3 = -_10;
_9 = &_2;
_22 = -_32.1;
_8 = _4;
_18 = _14 as i32;
_19 = !_14;
_16 = _11 - _11;
_45 = _31 % _32.3;
_37 = Checked(19242_u16 * 17448_u16);
match _32.2 {
82 => bb29,
_ => bb28
}
}
bb28 = {
_13 = 1341_u16 as f64;
_14 = 111522615428476794723978273787692216078_i128 + (-60157196210077206924185011511688401950_i128);
_25.0 = [12616554940901586578_u64];
_19 = -_14;
_16 = 8580684949617847521_u64 as u8;
_9 = &(*_9);
_3 = _17 >> _14;
_23 = 527250458_u32;
_21 = 2847107299971408911_usize - 2043770040423395586_usize;
_13 = _21 as f64;
_12 = [_10,_17,_3,_3,_17];
_22 = _13 - _13;
_19 = _11 as i128;
_9 = &_2;
_13 = _18 as f64;
_26 = '\u{e26d5}';
_17 = _18 as isize;
_9 = &(*_9);
_20 = [_26];
_21 = (*_9) as usize;
Goto(bb18)
}
bb29 = {
_7 = _1;
_37.0 = 34507_u16 ^ 41994_u16;
_1 = _32.0;
_14 = _11 as i128;
_32.4 = [_32.3];
_27 = _10;
_12 = [_27,_10,_10,_27,_17];
_27 = _3 & _17;
_29 = [_18,_24,_24,_24,_24,_24,_18,_18];
_16 = (*_9) as u8;
_44 = Adt41::Variant0 { fld0: _4 };
place!(Field::<f32>(Variant(_44, 0), 0)) = _19 as f32;
_39 = _4;
_39 = _19 as f32;
_37.0 = !51257_u16;
_32.2 = _41 as i8;
_14 = _19 >> _32.3;
_33 = _26;
_10 = -_27;
SetDiscriminant(_44, 1);
_33 = _26;
_46 = (-24968_i16) << _31;
_5 = -_4;
match _32.3 {
0 => bb20,
1 => bb30,
2 => bb31,
3 => bb32,
4 => bb33,
5 => bb34,
6 => bb35,
7418608544189282147 => bb37,
_ => bb36
}
}
bb30 = {
_1 = _15.0;
_19 = !_14;
_12 = [_3,_3,_3,_3,_3];
_21 = !11606740618329780692_usize;
_6 = [_17,_17,_10,_3,_17];
_3 = -_17;
_17 = _10;
_18 = _24;
_30 = [_24,_18,_18,_18,_24,_18];
_7 = _15.0;
_28 = (_25.0,);
_15.0 = _1;
_29 = [_18,_18,_24,_24,_24,_18,_24,_18];
_17 = _10 * _3;
_27 = _10;
_25.0 = _28.0;
_10 = _17;
_7 = !_15.0;
_32 = (_1, _13, 116_i8, 9412403209522274001_u64, _25.0);
_9 = &(*_9);
_32.2 = _21 as i8;
match _32.3 {
0 => bb15,
1 => bb6,
2 => bb19,
3 => bb20,
4 => bb21,
5 => bb22,
9412403209522274001 => bb24,
_ => bb23
}
}
bb31 = {
_12 = [_10,_17,_10,_3,_3];
_18 = -(-754361984_i32);
_9 = &(*_9);
_12 = _6;
_9 = &(*_9);
_9 = &(*_9);
_8 = _4;
_4 = -_8;
_16 = _14 as u8;
_20 = ['\u{73698}'];
_11 = 16399_u16 as u8;
_11 = !_16;
_11 = _16 | _16;
_4 = _8 * _8;
_13 = 65298_u16 as f64;
_12 = [_10,_17,_17,_10,_3];
_15 = (_1,);
_11 = _16;
_20 = ['\u{7b164}'];
_22 = _13 - _13;
Call(_15.0 = fn5(_4, _7, _4, _5, _6), ReturnTo(bb16), UnwindUnreachable())
}
bb32 = {
_16 = !_11;
_15 = (_7,);
_5 = (*_9) as f32;
_12 = [_3,_17,_10,_10,_17];
match _18 {
0 => bb5,
1 => bb12,
2 => bb13,
340282366920938463463374607430685095811 => bb15,
_ => bb14
}
}
bb33 = {
_1 = !_7;
_2 = (-3704766691591542621_i64) >> _3;
_3 = 7646_i16 as isize;
_11 = !225_u8;
_14 = 20339394890199831087314369640156118453_i128;
_6 = [_10,_10,_10,_10,_3];
_15.0 = !_7;
_3 = !_10;
_11 = !220_u8;
_14 = _3 as i128;
_4 = _5;
_14 = _11 as i128;
_1 = !_15.0;
_5 = _8 - _8;
_8 = _5;
_16 = !_11;
_15.0 = !_7;
Goto(bb3)
}
bb34 = {
_2 = 6197681639477001887_i64;
_9 = &_2;
match (*_9) {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
6197681639477001887 => bb7,
_ => bb6
}
}
bb35 = {
Return()
}
bb36 = {
_15 = (_7,);
_3 = _10;
_16 = _11;
_12 = [_10,_3,_3,_3,_3];
_10 = _3 * _3;
_17 = 6252005557224644749_u64 as isize;
_10 = -_3;
_15.0 = !_7;
_13 = _2 as f64;
_18 = (-935362859_i32);
_9 = &(*_9);
_7 = _15.0 != _15.0;
_18 = 32352_i16 as i32;
_15.0 = !_7;
_20 = ['\u{73820}'];
_12 = [_3,_3,_10,_3,_10];
_17 = 17873_u16 as isize;
_18 = (-1083115645_i32);
_9 = &(*_9);
match _18 {
0 => bb1,
1 => bb8,
2 => bb9,
340282366920938463463374607430685095811 => bb11,
_ => bb10
}
}
bb37 = {
_7 = !_1;
place!(Field::<i8>(Variant(_44, 1), 3)) = _32.2;
_26 = _33;
_2 = _1 as i64;
_9 = &_2;
_15.0 = _32.0;
_5 = _27 as f32;
place!(Field::<i64>(Variant(_44, 1), 6)) = -(*_9);
_16 = _11;
_22 = _32.1 + _32.1;
_51 = _32.4;
place!(Field::<([u64; 1],)>(Variant(_44, 1), 2)) = (_51,);
_38 = -_3;
_7 = _1;
_37.1 = _32.0;
place!(Field::<[u64; 1]>(Variant(_44, 1), 4)) = Field::<([u64; 1],)>(Variant(_44, 1), 2).0;
Goto(bb38)
}
bb38 = {
_53 = _7 ^ _1;
_11 = Field::<i8>(Variant(_44, 1), 3) as u8;
_44 = Adt41::Variant0 { fld0: _4 };
_41 = 75042704684162455078622270365924091422_u128 >> _46;
_60 = -_32.1;
_56.1 = -_32.1;
_59 = _14;
_32.0 = _15.0;
_31 = _45 | _45;
_48 = -_14;
_56 = (_15.0, _22, _32.2, _31, _51);
_28 = _25;
_20 = [_33];
_56.2 = _32.2 & _32.2;
_20 = [_26];
_1 = _53;
_26 = _33;
_61 = _56.1 * _60;
_32.4 = [_45];
_64.fld3 = [_33];
_64.fld1 = _37.0 & _37.0;
Call(_50 = core::intrinsics::bswap(_59), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
_3 = _56.2 as isize;
_19 = !_48;
_19 = _48 - _59;
_1 = _32.0;
_38 = _17 | _27;
_64.fld4 = _22;
_43 = [_56.2,_56.2,_56.2,_32.2,_56.2];
_28 = (_51,);
_9 = &(*_9);
_33 = _26;
_32.0 = !_15.0;
Call(_64.fld6 = core::intrinsics::transmute(_1), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
_21 = !15052145868271947804_usize;
_52 = _32.0;
_65 = -_4;
_64.fld2 = Adt42::Variant2 { fld0: _56.3,fld1: (*_9),fld2: _12,fld3: _4,fld4: _41 };
_39 = _65;
place!(Field::<u128>(Variant(_64.fld2, 2), 4)) = _41 >> _56.3;
_64.fld0 = core::ptr::addr_of_mut!(_30);
_32.1 = -_61;
_63 = _23 as usize;
_53 = _7 >= _32.0;
_36.fld1 = Adt47::Variant1 { fld0: _43,fld1: _32 };
_9 = &_57;
_16 = _3 as u8;
_19 = !_48;
_57 = Field::<i64>(Variant(_64.fld2, 2), 1);
_17 = _38;
_37.0 = _64.fld1;
_70 = core::ptr::addr_of_mut!(_59);
_42 = [_23,_23];
_29 = [_24,_24,_18,_18,_18,_24,_18,_18];
_32.4 = [_56.3];
match _32.3 {
0 => bb25,
1 => bb23,
2 => bb3,
3 => bb36,
4 => bb5,
5 => bb29,
6 => bb7,
7418608544189282147 => bb41,
_ => bb35
}
}
bb41 = {
_9 = &_2;
_5 = -_65;
_55 = _36.fld1;
_39 = _65;
SetDiscriminant(_55, 1);
place!(Field::<[i8; 5]>(Variant(_36.fld1, 1), 0)) = [_32.2,Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).2,_56.2,_32.2,_56.2];
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1)).3 = !_32.3;
_73 = [_17,_27,_3,_27,_27];
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1)).1 = -_32.1;
_58 = [_24,_24,_24,_18,_24,_18];
_18 = _24 & _24;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1)) = (_53, _22, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).2, Field::<u64>(Variant(_64.fld2, 2), 0), _28.0);
_8 = _41 as f32;
_69 = _46;
_55 = _36.fld1;
RET.fld1 = _36.fld1;
_64.fld0 = core::ptr::addr_of_mut!(_30);
Call(_73 = fn7(_32, Move(_64.fld2), Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1), _56, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 1), 1).0, _32), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1)) = _56;
_68 = core::ptr::addr_of!(place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1)));
SetDiscriminant(_36.fld1, 1);
(*_68).2 = -Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 1), 1).2;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 1), 1)).0 = _52 & _1;
_36.fld1 = _55;
_4 = _63 as f32;
SetDiscriminant(RET.fld1, 2);
_81 = _26;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).1 = _2 as f64;
_71 = (*_68).0;
_62 = (_63,);
_43 = [Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).2,_56.2,(*_68).2,_32.2,_56.2];
_71 = !_37.1;
place!(Field::<(bool,)>(Variant(RET.fld1, 2), 2)) = (_71,);
_22 = _41 as f64;
place!(Field::<i32>(Variant(RET.fld1, 2), 5)) = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).1 as i32;
_64.fld4 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).1 * (*_68).1;
place!(Field::<*mut i128>(Variant(RET.fld1, 2), 3)) = _70;
_56.4 = (*_68).4;
_82 = _64.fld3;
Goto(bb43)
}
bb43 = {
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)) = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1);
place!(Field::<*mut (bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 4)) = core::ptr::addr_of_mut!((*_68));
_55 = _36.fld1;
_57 = Field::<i32>(Variant(RET.fld1, 2), 5) as i64;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1)).4 = _32.4;
Goto(bb44)
}
bb44 = {
_76 = &(*_9);
_16 = _64.fld6 << _69;
place!(Field::<u32>(Variant(RET.fld1, 2), 1)) = _26 as u32;
place!(Field::<*mut (bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 4)) = core::ptr::addr_of_mut!(_56);
_13 = -_32.1;
_62.0 = !_63;
_65 = Field::<f32>(Variant(_44, 0), 0) + _8;
_32.1 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).1 - _64.fld4;
_2 = _57 - _57;
_61 = _13;
place!(Field::<u32>(Variant(RET.fld1, 2), 1)) = _23;
_54 = Adt45::Variant2 { fld0: Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).4,fld1: Field::<[i8; 5]>(Variant(_55, 1), 0) };
_13 = _41 as f64;
_31 = _33 as u64;
_64.fld6 = _16 << (*_70);
_64.fld6 = _16 << Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).3;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1)).4 = [_32.3];
_17 = _10;
place!(Field::<(bool,)>(Variant(RET.fld1, 2), 2)).0 = _7;
_24 = -Field::<i32>(Variant(RET.fld1, 2), 5);
SetDiscriminant(_36.fld1, 0);
_2 = _57;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).0 = _2 == _2;
_92 = _62;
Call(place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).3 = core::intrinsics::transmute(_32.3), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
RET.fld1 = _55;
_70 = core::ptr::addr_of_mut!(_48);
_62 = _92;
_79 = _6;
_73 = [_38,_3,_3,_27,_27];
_43 = Field::<[i8; 5]>(Variant(RET.fld1, 1), 0);
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 1), 1)) = _32;
SetDiscriminant(RET.fld1, 2);
_32.0 = _57 > _57;
_72 = -_14;
_60 = _22 * Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).1;
place!(Field::<([u64; 1],)>(Variant(_36.fld1, 0), 4)).0 = _32.4;
_71 = _1;
_93 = (_39, _32.1, _19, _39);
place!(Field::<*mut i128>(Variant(RET.fld1, 2), 3)) = _70;
_67 = &_57;
place!(Field::<(bool,)>(Variant(RET.fld1, 2), 2)) = _15;
_22 = -_60;
SetDiscriminant(_44, 1);
place!(Field::<i32>(Variant(RET.fld1, 2), 5)) = -_24;
_64.fld3 = [_33];
_32.0 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).3 == _56.3;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).2 = !Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).2;
_78 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).2;
_65 = _8;
_56 = (_52, _60, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).2, _45, _28.0);
place!(Field::<*mut [i32; 6]>(Variant(_36.fld1, 0), 1)) = _64.fld0;
match _32.3 {
0 => bb35,
1 => bb36,
2 => bb38,
3 => bb28,
4 => bb46,
7418608544189282147 => bb48,
_ => bb47
}
}
bb46 = {
_2 = 6197681639477001887_i64;
_9 = &_2;
match (*_9) {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
6197681639477001887 => bb7,
_ => bb6
}
}
bb47 = {
_12 = [_10,_17,_10,_3,_3];
_18 = -(-754361984_i32);
_9 = &(*_9);
_12 = _6;
_9 = &(*_9);
_9 = &(*_9);
_8 = _4;
_4 = -_8;
_16 = _14 as u8;
_20 = ['\u{73698}'];
_11 = 16399_u16 as u8;
_11 = !_16;
_11 = _16 | _16;
_4 = _8 * _8;
_13 = 65298_u16 as f64;
_12 = [_10,_17,_17,_10,_3];
_15 = (_1,);
_11 = _16;
_20 = ['\u{7b164}'];
_22 = _13 - _13;
Call(_15.0 = fn5(_4, _7, _4, _5, _6), ReturnTo(bb16), UnwindUnreachable())
}
bb48 = {
_58 = _30;
_79 = [_38,_3,_27,_27,_27];
_25.0 = [_45];
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)) = (_53, _32.1, _78, _45, Field::<[u64; 1]>(Variant(_54, 2), 0));
_79 = [_38,_38,_3,_17,_38];
_20 = [_33];
place!(Field::<([u64; 1],)>(Variant(_36.fld1, 0), 4)).0 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).4;
_32 = (_7, _56.1, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).2, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).3, Field::<([u64; 1],)>(Variant(_36.fld1, 0), 4).0);
_64.fld0 = core::ptr::addr_of_mut!(_94);
_93.0 = _8 * _5;
_92.0 = !_62.0;
_81 = _26;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).3 = !_56.3;
place!(Field::<u32>(Variant(RET.fld1, 2), 1)) = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).1 as u32;
_27 = -_10;
_43 = [_32.2,_56.2,Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).2,Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).2,Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).2];
SetDiscriminant(_55, 0);
SetDiscriminant(_54, 3);
Goto(bb49)
}
bb49 = {
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).0 = _56.0;
_55 = Adt47::Variant1 { fld0: _43,fld1: Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6) };
place!(Field::<i64>(Variant(_44, 1), 6)) = !(*_67);
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).0 = _52;
place!(Field::<i8>(Variant(_44, 1), 3)) = _78;
_22 = _61;
_93.0 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 1), 1).3 as f32;
SetDiscriminant(_55, 2);
_19 = !_59;
place!(Field::<*const (u8,)>(Variant(RET.fld1, 2), 0)) = core::ptr::addr_of!(place!(Field::<(u8,)>(Variant(_54, 3), 1)));
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)).2 = _46 as i8;
_2 = -(*_67);
place!(Field::<i8>(Variant(_36.fld1, 0), 3)) = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).2 << _32.3;
_62.0 = Field::<i32>(Variant(RET.fld1, 2), 5) as usize;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 2), 6)).0 = _56.0;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 2), 6)).2 = _65 as i8;
Goto(bb50)
}
bb50 = {
_80 = [_26];
_36.fld1 = Adt47::Variant1 { fld0: _43,fld1: _56 };
_82 = _64.fld3;
_64.fld3 = _82;
_88 = _2 as f32;
_66 = !_3;
_46 = _69;
_40 = Field::<*const (u8,)>(Variant(RET.fld1, 2), 0);
_43 = [Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).2,_78,Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).2,Field::<i8>(Variant(_44, 1), 3),Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).2];
_102 = [_23,Field::<u32>(Variant(RET.fld1, 2), 1)];
_76 = &(*_67);
_7 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6).0;
_63 = _41 as usize;
place!(Field::<*mut [i32; 6]>(Variant(_44, 1), 0)) = core::ptr::addr_of_mut!(_58);
_32.2 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 2), 6).2;
place!(Field::<i64>(Variant(_44, 1), 6)) = Field::<u32>(Variant(RET.fld1, 2), 1) as i64;
_93 = (_88, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).1, _72, _5);
_87 = Adt42::Variant2 { fld0: _56.3,fld1: _57,fld2: _79,fld3: _4,fld4: _41 };
Goto(bb51)
}
bb51 = {
_36.fld0 = core::ptr::addr_of!((*_40));
_64.fld0 = core::ptr::addr_of_mut!(_30);
RET.fld0 = _40;
RET.fld1 = _36.fld1;
_93.3 = _8;
_103 = [_18,_24];
_108 = _65;
place!(Field::<(usize,)>(Variant(_44, 1), 5)) = (_63,);
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1)) = (Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 1), 1).0, _61, _32.2, _56.3, _32.4);
place!(Field::<(u16, bool)>(Variant(_44, 1), 1)) = (_64.fld1, Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).0);
_24 = _22 as i32;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1)).1 = _16 as f64;
_81 = _33;
place!(Field::<(u16, bool)>(Variant(_44, 1), 1)) = (_37.0, _32.0);
_87 = Adt42::Variant2 { fld0: _45,fld1: (*_67),fld2: _79,fld3: _108,fld4: _41 };
place!(Field::<u64>(Variant(_87, 2), 0)) = !_32.3;
place!(Field::<*mut i128>(Variant(_55, 2), 3)) = _70;
_23 = !583485909_u32;
SetDiscriminant(RET.fld1, 2);
_85 = [_24,_24,_24,_18,_24,_24];
_48 = _19;
place!(Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(RET.fld1, 2), 6)) = _56;
Goto(bb52)
}
bb52 = {
place!(Field::<*mut [i32; 6]>(Variant(_44, 1), 0)) = _64.fld0;
_93.0 = -_108;
_21 = _93.2 as usize;
RET = Adt52 { fld0: _40,fld1: _36.fld1 };
_74 = [_24,_24];
place!(Field::<(u8,)>(Variant(_54, 3), 1)) = (_64.fld6,);
place!(Field::<(u16, bool)>(Variant(_44, 1), 1)) = (_37.0, _71);
SetDiscriminant(_36.fld1, 2);
place!(Field::<i8>(Variant(_44, 1), 3)) = -Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_55, 2), 6).2;
_112 = !_66;
_4 = -Field::<f32>(Variant(_87, 2), 3);
place!(Field::<u128>(Variant(_87, 2), 4)) = _41 + _41;
place!(Field::<*mut [i32; 6]>(Variant(_44, 1), 0)) = core::ptr::addr_of_mut!(_58);
place!(Field::<(bool,)>(Variant(_55, 2), 2)) = _15;
_36.fld1 = RET.fld1;
_36.fld0 = core::ptr::addr_of!(place!(Field::<(u8,)>(Variant(_54, 3), 1)));
place!(Field::<i32>(Variant(_55, 2), 5)) = _24 + _24;
_60 = Field::<(bool, f64, i8, u64, [u64; 1])>(Variant(_36.fld1, 1), 1).1;
_37.0 = !Field::<(u16, bool)>(Variant(_44, 1), 1).0;
_64.fld2 = Move(_87);
_37 = Field::<(u16, bool)>(Variant(_44, 1), 1);
Goto(bb53)
}
bb53 = {
Call(_115 = dump_var(4_usize, 14_usize, Move(_14), 52_usize, Move(_52), 42_usize, Move(_42), 50_usize, Move(_50)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_115 = dump_var(4_usize, 2_usize, Move(_2), 7_usize, Move(_7), 102_usize, Move(_102), 71_usize, Move(_71)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_115 = dump_var(4_usize, 72_usize, Move(_72), 82_usize, Move(_82), 112_usize, Move(_112), 58_usize, Move(_58)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_115 = dump_var(4_usize, 25_usize, Move(_25), 20_usize, Move(_20), 26_usize, Move(_26), 19_usize, Move(_19)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_115 = dump_var(4_usize, 6_usize, Move(_6), 69_usize, Move(_69), 10_usize, Move(_10), 23_usize, Move(_23)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_115 = dump_var(4_usize, 59_usize, Move(_59), 33_usize, Move(_33), 92_usize, Move(_92), 3_usize, Move(_3)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_115 = dump_var(4_usize, 63_usize, Move(_63), 18_usize, Move(_18), 80_usize, Move(_80), 1_usize, Move(_1)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_115 = dump_var(4_usize, 11_usize, Move(_11), 116_usize, _116, 116_usize, _116, 116_usize, _116), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: f32,mut _2: bool,mut _3: f32,mut _4: f32,mut _5: [isize; 5]) -> bool {
mir! {
type RET = bool;
let _6: ();
let _7: ();
{
_4 = -_3;
_5 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-35_isize),9223372036854775807_isize];
_4 = 96162558763246130832853890623559819110_u128 as f32;
_3 = 1_usize as f32;
RET = _2;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(5_usize, 2_usize, Move(_2), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: f32,mut _2: i8,mut _3: i8,mut _4: [u64; 1],mut _5: ([u64; 1],),mut _6: i8,mut _7: (bool,),mut _8: bool,mut _9: (bool, f64, i8, u64, [u64; 1]),mut _10: [char; 1],mut _11: i128,mut _12: u64,mut _13: f32,mut _14: (bool, f64, i8, u64, [u64; 1]),mut _15: f64,mut _16: bool) -> f64 {
mir! {
type RET = f64;
let _17: [u32; 2];
let _18: isize;
let _19: ();
let _20: ();
{
_2 = -_3;
_9.4 = _5.0;
_2 = _9.2 << _6;
_4 = _5.0;
_10 = ['\u{10bc59}'];
_1 = 3400072310_u32 as f32;
RET = (-9223372036854775808_isize) as f64;
RET = -_14.1;
_3 = 68_isize as i8;
_4 = [_14.3];
_7 = (_14.0,);
_11 = (-6194550437890937243762904800990724098_i128);
RET = _14.1 * _14.1;
_17 = [673592126_u32,2322692802_u32];
_12 = !_14.3;
_14.0 = !_8;
_4 = [_9.3];
_9.2 = !_6;
_2 = !_9.2;
_5 = (_4,);
_9.3 = 2_usize as u64;
_10 = ['\u{41a0a}'];
_6 = -_2;
_4 = [_14.3];
_14.3 = !_12;
RET = -_14.1;
_7.0 = _8 != _8;
_14 = (_7.0, _15, _6, _12, _4);
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(6_usize, 11_usize, Move(_11), 4_usize, Move(_4), 16_usize, Move(_16), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(6_usize, 12_usize, Move(_12), 17_usize, Move(_17), 20_usize, _20, 20_usize, _20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: (bool, f64, i8, u64, [u64; 1]),mut _2: Adt42,mut _3: (bool, f64, i8, u64, [u64; 1]),mut _4: (bool, f64, i8, u64, [u64; 1]),mut _5: bool,mut _6: (bool, f64, i8, u64, [u64; 1])) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _7: [u64; 1];
let _8: ([u64; 1],);
let _9: [i32; 8];
let _10: [i32; 6];
let _11: *mut [i32; 6];
let _12: bool;
let _13: i128;
let _14: bool;
let _15: bool;
let _16: i16;
let _17: f32;
let _18: (u8,);
let _19: Adt54;
let _20: [i32; 6];
let _21: i16;
let _22: i16;
let _23: [u32; 2];
let _24: (bool, f64, i8, u64, [u64; 1]);
let _25: f64;
let _26: isize;
let _27: i128;
let _28: isize;
let _29: bool;
let _30: f32;
let _31: f32;
let _32: Adt51;
let _33: i64;
let _34: [i32; 6];
let _35: Adt44;
let _36: usize;
let _37: (u16, bool);
let _38: (usize,);
let _39: isize;
let _40: i16;
let _41: (f32, f64, i128, f32);
let _42: u128;
let _43: Adt42;
let _44: f64;
let _45: f32;
let _46: isize;
let _47: ();
let _48: ();
{
RET = [(-9223372036854775808_isize),(-47_isize),9223372036854775807_isize,42_isize,9223372036854775807_isize];
RET = Field::<[isize; 5]>(Variant(_2, 2), 2);
place!(Field::<f32>(Variant(_2, 2), 3)) = Field::<i64>(Variant(_2, 2), 1) as f32;
_4 = _3;
_4.0 = _5;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<[isize; 5]>(Variant(_2, 2), 2)) = [9223372036854775807_isize,9223372036854775807_isize,(-113_isize),(-9223372036854775808_isize),(-19_isize)];
Call(_3.4 = fn8(_1.4, Move(_2), _6.1, _1.0, _3.3, _6.3, _4.4, _4.4, _5, _1.4, _1.3, _4.4, _6.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.0 = _1.0;
_8.0 = _1.4;
_1.3 = _3.3 + _6.3;
_4.4 = [_6.3];
_4.2 = _3.2;
_3.3 = _1.3 & _6.3;
_6 = (_3.0, _4.1, _3.2, _4.3, _1.4);
_1 = _6;
_6.3 = _1.3 * _4.3;
_6.4 = _4.4;
_4.0 = _1.1 != _6.1;
_4.1 = _1.1 + _3.1;
RET = [(-9223372036854775808_isize),(-89_isize),(-9223372036854775808_isize),(-91_isize),(-9223372036854775808_isize)];
_6.1 = -_3.1;
_3.3 = !_4.3;
_10 = [(-13477261_i32),(-230640184_i32),102059438_i32,(-1472752102_i32),(-1571571645_i32),1032494959_i32];
_3 = (_4.0, _1.1, _1.2, _6.3, _6.4);
RET = [(-9223372036854775808_isize),32_isize,37_isize,36_isize,9223372036854775807_isize];
_1 = (_6.0, _6.1, _3.2, _6.3, _3.4);
_5 = _6.0;
_1.1 = -_4.1;
_6.2 = _4.2 ^ _3.2;
_11 = core::ptr::addr_of_mut!(_10);
_4.4 = [_6.3];
_4.1 = _1.1 - _6.1;
_12 = _4.1 <= _4.1;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-102_isize),(-9223372036854775808_isize),9223372036854775807_isize];
Goto(bb2)
}
bb2 = {
_8.0 = [_3.3];
_6.0 = _12 ^ _1.0;
_1 = (_3.0, _3.1, _3.2, _3.3, _4.4);
_6 = (_3.0, _1.1, _3.2, _1.3, _8.0);
_7 = [_1.3];
_3.2 = _4.1 as i8;
_1.1 = _3.1;
_10 = [(-1154096830_i32),(-1410659055_i32),(-1947062330_i32),745357603_i32,694080912_i32,1642545598_i32];
_5 = !_4.0;
Goto(bb3)
}
bb3 = {
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
Call(RET = fn9(_3.0, _4.3, _1.0, _3, _1.1, _3, _5, _1.0, _4, _7, _6, _1.0, _3, _4.3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_11) = [(-5254917_i32),583422553_i32,(-2057234725_i32),(-1438881253_i32),926325529_i32,356022040_i32];
_13 = 2130873982_i32 as i128;
_6.4 = _1.4;
_4.2 = _3.2 ^ _3.2;
_4.1 = -_3.1;
_15 = !_3.0;
_6 = (_15, _4.1, _4.2, _3.3, _8.0);
_11 = core::ptr::addr_of_mut!((*_11));
_16 = (-4837_i16);
_4.2 = _3.2;
(*_11) = [1076686854_i32,(-1295909981_i32),(-901478124_i32),(-537658234_i32),1978085651_i32,1594044197_i32];
_16 = 31515_i16;
_3.1 = -_4.1;
_4 = (_3.0, _3.1, _6.2, _3.3, _8.0);
RET = [9223372036854775807_isize,(-26_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4.1 = _6.2 as f64;
_1.1 = _6.1 - _6.1;
_4.3 = !_6.3;
_14 = !_6.0;
_5 = _3.1 > _1.1;
_11 = core::ptr::addr_of_mut!(_10);
_4.2 = _6.1 as i8;
_6.2 = !_3.2;
_3.2 = _4.2;
_1.2 = -_6.2;
Call(_4.1 = fn17(_1.2, _8.0, _12, _14, _8.0, _10, _14, _3.0, _15, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = core::ptr::addr_of_mut!((*_11));
_19.fld0 = [1379993517_u32,1176970543_u32];
_4 = (_12, _1.1, _1.2, _6.3, _8.0);
_18.0 = _13 as u8;
_4.1 = _3.1;
_4.1 = (-2124641297_i32) as f64;
_5 = _6.0;
_4.2 = 60071_u16 as i8;
_4.4 = _1.4;
_10 = [(-190681288_i32),1209440_i32,230769859_i32,(-192570378_i32),1819298520_i32,1646388222_i32];
_4.3 = 1608981155_u32 as u64;
_5 = _3.0;
_4 = (_14, _3.1, _1.2, _3.3, _8.0);
_13 = (-111843590087607957751802149961606984501_i128) + 54614874639184120312427516616710240613_i128;
_19.fld2 = _8;
(*_11) = [(-1775624234_i32),1996684010_i32,1085503300_i32,(-1050427393_i32),(-504583945_i32),(-1722336344_i32)];
_4.0 = !_12;
(*_11) = [(-1368889956_i32),(-910006612_i32),(-541575598_i32),(-60231403_i32),(-2035043864_i32),1062665381_i32];
Goto(bb6)
}
bb6 = {
RET = [75_isize,9223372036854775807_isize,(-9223372036854775808_isize),98_isize,9223372036854775807_isize];
_17 = _16 as f32;
_28 = (-9223372036854775808_isize);
_24.3 = _18.0 as u64;
_25 = _1.1 * _6.1;
_27 = _13;
_1.2 = _17 as i8;
_3.0 = _1.3 == _4.3;
_19.fld2 = (_6.4,);
RET = [_28,_28,_28,_28,_28];
_6.1 = _4.1;
_13 = _18.0 as i128;
_5 = !_12;
_24.2 = -_4.2;
_4.3 = !_6.3;
_1.3 = !_3.3;
_24.4 = [_3.3];
_18 = (19_u8,);
Goto(bb7)
}
bb7 = {
_20 = [(-454482869_i32),1300834033_i32,476077862_i32,(-284827406_i32),(-312829051_i32),2111762616_i32];
_4 = _1;
_8.0 = _19.fld2.0;
_33 = 8730803211729123710_i64 + 346131083656718520_i64;
_25 = -_3.1;
_1.4 = [_1.3];
RET = [_28,_28,_28,_28,_28];
_1.0 = _4.0;
_8.0 = [_4.3];
_18 = (253_u8,);
_13 = _3.3 as i128;
_27 = _6.1 as i128;
_4 = (_1.0, _3.1, _6.2, _3.3, _19.fld2.0);
_3.4 = [_3.3];
_30 = _17;
_8.0 = [_3.3];
_29 = _6.3 != _6.3;
_7 = [_3.3];
_7 = _1.4;
_37.1 = _6.2 < _24.2;
_6.1 = -_4.1;
_3.1 = _25;
_4.0 = !_3.0;
_38.0 = 2_usize;
_5 = _1.0 >= _3.0;
match _18.0 {
253 => bb9,
_ => bb8
}
}
bb8 = {
_8.0 = [_3.3];
_6.0 = _12 ^ _1.0;
_1 = (_3.0, _3.1, _3.2, _3.3, _4.4);
_6 = (_3.0, _1.1, _3.2, _1.3, _8.0);
_7 = [_1.3];
_3.2 = _4.1 as i8;
_1.1 = _3.1;
_10 = [(-1154096830_i32),(-1410659055_i32),(-1947062330_i32),745357603_i32,694080912_i32,1642545598_i32];
_5 = !_4.0;
Goto(bb3)
}
bb9 = {
_26 = _4.2 as isize;
_6.1 = _3.1 - _25;
_2 = Adt42::Variant2 { fld0: _3.3,fld1: _33,fld2: RET,fld3: _30,fld4: 199074156583777026636647074347579901608_u128 };
_22 = _16 - _16;
_4.4 = _7;
_18 = (228_u8,);
_37.0 = !20588_u16;
_26 = _28 + _28;
_3.2 = _24.2;
place!(Field::<f32>(Variant(_2, 2), 3)) = _17;
_41.0 = Field::<f32>(Variant(_2, 2), 3) - Field::<f32>(Variant(_2, 2), 3);
_4.0 = _3.0 & _3.0;
_23 = [389611990_u32,196557384_u32];
_6.0 = _14;
_18.0 = 211_u8;
_27 = _13 + _13;
_1 = (_29, _3.1, _6.2, Field::<u64>(Variant(_2, 2), 0), _4.4);
RET = [_26,_26,_26,_26,_26];
Call(_41.2 = core::intrinsics::bswap(_27), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
place!(Field::<u64>(Variant(_2, 2), 0)) = !_6.3;
_13 = _18.0 as i128;
_31 = _30;
_41.3 = _41.0 + _17;
_40 = -_22;
_37.1 = _29 & _5;
_42 = _33 as u128;
_6.3 = Field::<u64>(Variant(_2, 2), 0) ^ _4.3;
match _16 {
0 => bb1,
1 => bb2,
31515 => bb11,
_ => bb4
}
}
bb11 = {
_17 = _41.3;
_21 = '\u{83d49}' as i16;
_6 = (_37.1, _4.1, _4.2, _1.3, _1.4);
_9 = [1455440702_i32,1191172590_i32,769852525_i32,1495260341_i32,1037696866_i32,(-1226980611_i32),2000739506_i32,(-779654969_i32)];
match _38.0 {
0 => bb10,
1 => bb2,
3 => bb12,
4 => bb13,
2 => bb15,
_ => bb14
}
}
bb12 = {
_20 = [(-454482869_i32),1300834033_i32,476077862_i32,(-284827406_i32),(-312829051_i32),2111762616_i32];
_4 = _1;
_8.0 = _19.fld2.0;
_33 = 8730803211729123710_i64 + 346131083656718520_i64;
_25 = -_3.1;
_1.4 = [_1.3];
RET = [_28,_28,_28,_28,_28];
_1.0 = _4.0;
_8.0 = [_4.3];
_18 = (253_u8,);
_13 = _3.3 as i128;
_27 = _6.1 as i128;
_4 = (_1.0, _3.1, _6.2, _3.3, _19.fld2.0);
_3.4 = [_3.3];
_30 = _17;
_8.0 = [_3.3];
_29 = _6.3 != _6.3;
_7 = [_3.3];
_7 = _1.4;
_37.1 = _6.2 < _24.2;
_6.1 = -_4.1;
_3.1 = _25;
_4.0 = !_3.0;
_38.0 = 2_usize;
_5 = _1.0 >= _3.0;
match _18.0 {
253 => bb9,
_ => bb8
}
}
bb13 = {
_26 = _4.2 as isize;
_6.1 = _3.1 - _25;
_2 = Adt42::Variant2 { fld0: _3.3,fld1: _33,fld2: RET,fld3: _30,fld4: 199074156583777026636647074347579901608_u128 };
_22 = _16 - _16;
_4.4 = _7;
_18 = (228_u8,);
_37.0 = !20588_u16;
_26 = _28 + _28;
_3.2 = _24.2;
place!(Field::<f32>(Variant(_2, 2), 3)) = _17;
_41.0 = Field::<f32>(Variant(_2, 2), 3) - Field::<f32>(Variant(_2, 2), 3);
_4.0 = _3.0 & _3.0;
_23 = [389611990_u32,196557384_u32];
_6.0 = _14;
_18.0 = 211_u8;
_27 = _13 + _13;
_1 = (_29, _3.1, _6.2, Field::<u64>(Variant(_2, 2), 0), _4.4);
RET = [_26,_26,_26,_26,_26];
Call(_41.2 = core::intrinsics::bswap(_27), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
(*_11) = [(-5254917_i32),583422553_i32,(-2057234725_i32),(-1438881253_i32),926325529_i32,356022040_i32];
_13 = 2130873982_i32 as i128;
_6.4 = _1.4;
_4.2 = _3.2 ^ _3.2;
_4.1 = -_3.1;
_15 = !_3.0;
_6 = (_15, _4.1, _4.2, _3.3, _8.0);
_11 = core::ptr::addr_of_mut!((*_11));
_16 = (-4837_i16);
_4.2 = _3.2;
(*_11) = [1076686854_i32,(-1295909981_i32),(-901478124_i32),(-537658234_i32),1978085651_i32,1594044197_i32];
_16 = 31515_i16;
_3.1 = -_4.1;
_4 = (_3.0, _3.1, _6.2, _3.3, _8.0);
RET = [9223372036854775807_isize,(-26_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4.1 = _6.2 as f64;
_1.1 = _6.1 - _6.1;
_4.3 = !_6.3;
_14 = !_6.0;
_5 = _3.1 > _1.1;
_11 = core::ptr::addr_of_mut!(_10);
_4.2 = _6.1 as i8;
_6.2 = !_3.2;
_3.2 = _4.2;
_1.2 = -_6.2;
Call(_4.1 = fn17(_1.2, _8.0, _12, _14, _8.0, _10, _14, _3.0, _15, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_1.2 = -_3.2;
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(7_usize, 14_usize, Move(_14), 27_usize, Move(_27), 13_usize, Move(_13), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(7_usize, 40_usize, Move(_40), 5_usize, Move(_5), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(7_usize, 7_usize, Move(_7), 10_usize, Move(_10), 38_usize, Move(_38), 29_usize, Move(_29)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [u64; 1],mut _2: Adt42,mut _3: f64,mut _4: bool,mut _5: u64,mut _6: u64,mut _7: [u64; 1],mut _8: [u64; 1],mut _9: bool,mut _10: [u64; 1],mut _11: u64,mut _12: [u64; 1],mut _13: bool) -> [u64; 1] {
mir! {
type RET = [u64; 1];
let _14: usize;
let _15: (usize,);
let _16: bool;
let _17: *mut i128;
let _18: Adt50;
let _19: f64;
let _20: u32;
let _21: ();
let _22: ();
{
_5 = !_6;
_3 = Field::<u128>(Variant(_2, 2), 4) as f64;
_3 = 37403_u16 as f64;
_13 = !_4;
_7 = [_5];
place!(Field::<u64>(Variant(_2, 2), 0)) = _6 / _11;
RET = [_6];
_12 = _10;
_5 = Field::<i64>(Variant(_2, 2), 1) as u64;
place!(Field::<u64>(Variant(_2, 2), 0)) = _5;
_12 = _10;
SetDiscriminant(_2, 2);
_5 = _11 % _11;
_6 = !_5;
_18.fld1 = 17829_u16;
_18.fld1 = !29516_u16;
_18.fld3 = ['\u{42594}'];
place!(Field::<u64>(Variant(_2, 2), 0)) = _11 % _11;
Goto(bb1)
}
bb1 = {
Call(_21 = dump_var(8_usize, 4_usize, Move(_4), 13_usize, Move(_13), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_21 = dump_var(8_usize, 9_usize, Move(_9), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: u64,mut _3: bool,mut _4: (bool, f64, i8, u64, [u64; 1]),mut _5: f64,mut _6: (bool, f64, i8, u64, [u64; 1]),mut _7: bool,mut _8: bool,mut _9: (bool, f64, i8, u64, [u64; 1]),mut _10: [u64; 1],mut _11: (bool, f64, i8, u64, [u64; 1]),mut _12: bool,mut _13: (bool, f64, i8, u64, [u64; 1]),mut _14: u64) -> [isize; 5] {
mir! {
type RET = [isize; 5];
let _15: Adt42;
let _16: [i8; 5];
let _17: (usize,);
let _18: isize;
let _19: char;
let _20: ([u64; 1],);
let _21: *mut (bool, f64, i8, u64, [u64; 1]);
let _22: (f32, f64, i128, f32);
let _23: (bool,);
let _24: char;
let _25: f64;
let _26: [i8; 5];
let _27: [u32; 2];
let _28: isize;
let _29: f32;
let _30: bool;
let _31: u128;
let _32: Adt48;
let _33: f32;
let _34: bool;
let _35: [u64; 1];
let _36: (usize,);
let _37: isize;
let _38: i64;
let _39: Adt43;
let _40: isize;
let _41: isize;
let _42: (bool, f64, i8, u64, [u64; 1]);
let _43: ();
let _44: ();
{
_3 = _11.0 | _11.0;
Goto(bb1)
}
bb1 = {
Call(_9.3 = fn10(_1, _6.2, _9.1, _9.0, _13.3, _4.0, _13.3, _6.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = !_7;
_17.0 = (-51_isize) as usize;
_13.4 = [_9.3];
_7 = _4.2 != _4.2;
_18 = (-32_isize) + 6_isize;
_9.4 = _6.4;
_13.2 = _4.2;
_6.0 = !_8;
_17.0 = 14971568313985097422_usize + 0_usize;
RET = [_18,_18,_18,_18,_18];
_9 = _11;
_13.1 = _9.1 + _6.1;
_11.3 = _13.2 as u64;
Goto(bb3)
}
bb3 = {
_20.0 = [_6.3];
_9.1 = _5;
_13.0 = _12 | _6.0;
_13.0 = _3;
_4 = (_6.0, _6.1, _13.2, _11.3, _9.4);
Call(_11.2 = core::intrinsics::bswap(_6.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.4 = _13.4;
_4.0 = _13.0;
_21 = core::ptr::addr_of_mut!(_6);
_13.2 = !_4.2;
Goto(bb5)
}
bb5 = {
_6.4 = [_13.3];
_13.3 = !(*_21).3;
_13.4 = _9.4;
_9.0 = _8;
_21 = core::ptr::addr_of_mut!((*_21));
_9.4 = _20.0;
_13.2 = _8 as i8;
_13.3 = _4.3 << _6.3;
_9.1 = _4.1 - _5;
(*_21).1 = -_11.1;
_3 = _11.0;
_4.4 = [_13.3];
_22.2 = (-31782680629321171021190668065798546520_i128);
(*_21).3 = !_9.3;
_16 = [_13.2,_6.2,(*_21).2,_6.2,(*_21).2];
_6.1 = _4.1;
_22.2 = !(-109505861714986281973047434195461270502_i128);
_20 = ((*_21).4,);
Goto(bb6)
}
bb6 = {
_13.0 = !_8;
_1 = !(*_21).0;
_22.0 = 60179126270820559523324155313388648910_u128 as f32;
_25 = _9.1 * (*_21).1;
_13.0 = !_1;
_20 = (_13.4,);
_17 = (420454025207713570_usize,);
_3 = _1 <= _13.0;
_10 = (*_21).4;
_11.3 = _22.2 as u64;
_9.1 = (*_21).1;
_11.2 = !_9.2;
Goto(bb7)
}
bb7 = {
_6.4 = _4.4;
_9.2 = _6.2 + (*_21).2;
_17.0 = _9.0 as usize;
_6.1 = 3308046623304263386_i64 as f64;
_19 = '\u{eab74}';
_9.1 = _4.1 * _25;
_24 = _19;
_19 = _24;
_4.1 = _13.1;
_11.3 = (*_21).3 << (*_21).3;
_23 = (_1,);
_23 = (_1,);
_11.3 = _13.3 << _6.2;
_22.0 = _22.2 as f32;
_11.4 = _20.0;
(*_21).3 = !_13.3;
_6.0 = (*_21).3 < _9.3;
_6.2 = _22.2 as i8;
(*_21).4 = [_9.3];
(*_21) = _4;
_28 = -_18;
Call(_21 = fn11(_9.1, (*_21), _6.0, _6.2, _17.0, _9.2, _4.2, _17, _13, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = (_23.0, _11.1, _4.2, _9.3, _6.4);
_6 = _13;
_22.3 = _22.0;
_11.1 = _18 as f64;
_20.0 = [_9.3];
_22.1 = _13.1 - _4.1;
_31 = 173829687488816649484099350537106324217_u128 * 93787622048554085276453408882109935212_u128;
_23.0 = _7 <= _3;
_27 = [2746608904_u32,3946525364_u32];
_11.0 = !_8;
Call(_4.3 = core::intrinsics::bswap(_13.3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6.1 = _9.1 - _9.1;
RET = [_28,_18,_28,_18,_28];
Goto(bb10)
}
bb10 = {
_14 = _13.3 * _6.3;
_13.4 = [_4.3];
_9 = (_6.0, _22.1, _4.2, _13.3, _13.4);
_6.0 = _12;
_4.1 = _5 * _5;
_11 = _4;
_6.4 = _20.0;
_6.4 = [_13.3];
_11.3 = _13.3 - _14;
_24 = _19;
_22.2 = _31 as i128;
_9.3 = !_13.3;
_11 = (_3, _25, _6.2, _9.3, _4.4);
_20.0 = [_6.3];
_3 = _6.0 >= _9.0;
_29 = _22.3 - _22.0;
_13.4 = [_14];
_11.4 = [_13.3];
_6.3 = _22.2 as u64;
_35 = [_2];
_22.3 = -_29;
_3 = !_8;
_23 = (_13.0,);
_11.0 = !_7;
_2 = 23_u8 as u64;
_9.4 = [_9.3];
Goto(bb11)
}
bb11 = {
_36 = (_17.0,);
_27 = [4035703230_u32,3857257432_u32];
_9.2 = _13.2;
_13.3 = !_11.3;
_34 = _7;
Call(_15 = fn12(_21, _11.4, _10), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_30 = _8;
_11.3 = _14;
_33 = _13.2 as f32;
_4.4 = _13.4;
_4.4 = _9.4;
_6.1 = _11.1 + _4.1;
_19 = _24;
_9.1 = _11.1;
Goto(bb13)
}
bb13 = {
_9 = (_8, _25, _6.2, _4.3, _35);
_4.0 = Field::<(u16, bool)>(Variant(_15, 1), 2).1;
_13.1 = -_11.1;
_13.3 = !_4.3;
_23 = (_8,);
_13.3 = !_4.3;
_36.0 = _17.0 >> _11.3;
_18 = _28 | _28;
place!(Field::<(u16, bool)>(Variant(_15, 1), 2)).0 = Field::<u16>(Variant(_15, 1), 0) * Field::<u16>(Variant(_15, 1), 0);
_27 = [4232521574_u32,830950868_u32];
_26 = [_6.2,_9.2,_9.2,_11.2,_11.2];
_37 = _19 as isize;
_17.0 = _36.0 >> _11.2;
_4.3 = _13.3;
_6.2 = _9.2;
_41 = _6.1 as isize;
_23.0 = !_4.0;
_13.2 = _9.2;
_7 = _9.0;
_8 = _14 >= _9.3;
_42.1 = -_5;
Goto(bb14)
}
bb14 = {
place!(Field::<(bool,)>(Variant(_15, 1), 3)) = _23;
SetDiscriminant(_15, 0);
_9.3 = !_14;
place!(Field::<(bool,)>(Variant(_15, 0), 3)) = _23;
_11.3 = _13.3;
_9.2 = _13.2;
place!(Field::<(f32, f64, i128, f32)>(Variant(_15, 0), 1)) = (_33, _6.1, _22.2, _33);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(9_usize, 27_usize, Move(_27), 17_usize, Move(_17), 16_usize, Move(_16), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(9_usize, 23_usize, Move(_23), 35_usize, Move(_35), 37_usize, Move(_37), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(9_usize, 30_usize, Move(_30), 8_usize, Move(_8), 12_usize, Move(_12), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: bool,mut _2: i8,mut _3: f64,mut _4: bool,mut _5: u64,mut _6: bool,mut _7: u64,mut _8: u64) -> u64 {
mir! {
type RET = u64;
let _9: f32;
let _10: ();
let _11: ();
{
RET = (-402260547_i32) as u64;
_6 = !_1;
_3 = 31525_i16 as f64;
_5 = _8 - _8;
_3 = 4130575536936801547_i64 as f64;
_6 = _2 <= _2;
_7 = (-7038335642113499970_i64) as u64;
_4 = _1 <= _6;
Call(_3 = core::intrinsics::transmute(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _5 | _5;
_6 = _1 | _4;
_3 = 1931539805_u32 as f64;
_8 = RET;
_7 = !_8;
RET = _8;
RET = (-25994_i16) as u64;
_6 = !_4;
_7 = _5 ^ _5;
_6 = !_1;
RET = !_8;
_9 = 126495749798425132543796565791347583790_u128 as f32;
_5 = _8 >> _8;
_2 = !(-75_i8);
_6 = !_4;
_8 = _7;
Goto(bb2)
}
bb2 = {
Call(_10 = dump_var(10_usize, 6_usize, Move(_6), 8_usize, Move(_8), 1_usize, Move(_1), 11_usize, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: f64,mut _2: (bool, f64, i8, u64, [u64; 1]),mut _3: bool,mut _4: i8,mut _5: usize,mut _6: i8,mut _7: i8,mut _8: (usize,),mut _9: (bool, f64, i8, u64, [u64; 1]),mut _10: [i8; 5]) -> *mut (bool, f64, i8, u64, [u64; 1]) {
mir! {
type RET = *mut (bool, f64, i8, u64, [u64; 1]);
let _11: ();
let _12: ();
{
RET = core::ptr::addr_of_mut!(_9);
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(11_usize, 4_usize, Move(_4), 7_usize, Move(_7), 3_usize, Move(_3), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *mut (bool, f64, i8, u64, [u64; 1]),mut _2: [u64; 1],mut _3: [u64; 1]) -> Adt42 {
mir! {
type RET = Adt42;
let _4: bool;
let _5: Adt48;
let _6: [char; 1];
let _7: isize;
let _8: Adt44;
let _9: f32;
let _10: Adt54;
let _11: Adt54;
let _12: i128;
let _13: (u8,);
let _14: Adt56;
let _15: f32;
let _16: ([u64; 1],);
let _17: Adt50;
let _18: [u32; 2];
let _19: (f32, f64, i128, f32);
let _20: isize;
let _21: f64;
let _22: Adt45;
let _23: f32;
let _24: [u64; 1];
let _25: usize;
let _26: bool;
let _27: [i8; 5];
let _28: isize;
let _29: (u8,);
let _30: (usize,);
let _31: isize;
let _32: isize;
let _33: isize;
let _34: [u32; 2];
let _35: (usize,);
let _36: Adt55;
let _37: [i8; 5];
let _38: (u8,);
let _39: Adt53;
let _40: u64;
let _41: ([u64; 1],);
let _42: *const *const (bool, f64, i8, u64, [u64; 1]);
let _43: isize;
let _44: char;
let _45: isize;
let _46: f64;
let _47: (u8,);
let _48: ();
let _49: ();
{
_2 = _3;
_3 = _2;
_2 = [15753137644314801003_u64];
Goto(bb1)
}
bb1 = {
_4 = !true;
_4 = false ^ true;
_4 = !false;
_3 = _2;
_4 = !false;
_3 = [3487558627908679165_u64];
_2 = [13204875374639850805_u64];
_4 = false;
_4 = !false;
_2 = _3;
_2 = [2336645857221276564_u64];
_4 = !false;
_3 = _2;
_3 = [8332982583400787763_u64];
_4 = true ^ false;
_2 = [6612180888440374516_u64];
_3 = [10538725801412961918_u64];
_2 = [7429305373372616430_u64];
_3 = [6206020598278395849_u64];
Goto(bb2)
}
bb2 = {
_2 = _3;
_4 = 129148224779601796677759623149868310393_i128 > (-162060575017055763661173275903724978188_i128);
_4 = false;
_3 = [1411339906520539636_u64];
_3 = [4154102501982281332_u64];
_3 = [14124857197606310524_u64];
_3 = _2;
_4 = !false;
_6 = ['\u{80c34}'];
Goto(bb3)
}
bb3 = {
_6 = ['\u{8097a}'];
_4 = !false;
_3 = [8920108621967247045_u64];
_6 = ['\u{48039}'];
_6 = ['\u{b3486}'];
_2 = _3;
_3 = [3182799765981757875_u64];
_2 = [12235949999246718420_u64];
_7 = (-1155954464_i32) as isize;
_2 = [14279622199306708842_u64];
_2 = _3;
_4 = true;
_6 = ['\u{60d6c}'];
_9 = (-1578654343_i32) as f32;
_3 = [7886178767207430764_u64];
_3 = [16898414578319856927_u64];
_4 = _9 > _9;
_3 = _2;
_10.fld0 = [1562630245_u32,4001577111_u32];
_11.fld0 = [412761212_u32,3946236075_u32];
_10.fld2.0 = [13105552369760308444_u64];
_11.fld2.0 = _3;
_3 = [5328158945734317656_u64];
_4 = _7 != _7;
_10.fld2 = _11.fld2;
Call(RET = fn13(_1, _10.fld0, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
place!(Field::<i16>(Variant(RET, 1), 4)) = -10514_i16;
_10.fld2.0 = [9185036746593908554_u64];
SetDiscriminant(RET, 1);
place!(Field::<u16>(Variant(RET, 1), 0)) = 30859_u16;
place!(Field::<([u64; 1],)>(Variant(RET, 1), 1)) = (_2,);
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)).0 = '\u{e94df}' as u16;
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)).1 = _4;
Goto(bb5)
}
bb5 = {
_12 = -(-130852053365552175653816112822444160277_i128);
place!(Field::<(bool,)>(Variant(RET, 1), 3)).0 = !_4;
place!(Field::<i16>(Variant(RET, 1), 4)) = 24113_i16;
_17.fld2 = Move(RET);
RET = Move(_17.fld2);
place!(Field::<([u64; 1],)>(Variant(RET, 1), 1)) = _11.fld2;
place!(Field::<(bool,)>(Variant(RET, 1), 3)) = (Field::<(u16, bool)>(Variant(RET, 1), 2).1,);
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)).0 = !Field::<u16>(Variant(RET, 1), 0);
_2 = [9765182278113801098_u64];
_16.0 = _10.fld2.0;
_19.0 = _9 * _9;
_15 = _7 as f32;
_17.fld4 = _7 as f64;
_19.1 = _17.fld4;
_17.fld4 = -_19.1;
match Field::<u16>(Variant(RET, 1), 0) {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
30859 => bb8,
_ => bb7
}
}
bb6 = {
_4 = !true;
_4 = false ^ true;
_4 = !false;
_3 = _2;
_4 = !false;
_3 = [3487558627908679165_u64];
_2 = [13204875374639850805_u64];
_4 = false;
_4 = !false;
_2 = _3;
_2 = [2336645857221276564_u64];
_4 = !false;
_3 = _2;
_3 = [8332982583400787763_u64];
_4 = true ^ false;
_2 = [6612180888440374516_u64];
_3 = [10538725801412961918_u64];
_2 = [7429305373372616430_u64];
_3 = [6206020598278395849_u64];
Goto(bb2)
}
bb7 = {
_2 = _3;
_4 = 129148224779601796677759623149868310393_i128 > (-162060575017055763661173275903724978188_i128);
_4 = false;
_3 = [1411339906520539636_u64];
_3 = [4154102501982281332_u64];
_3 = [14124857197606310524_u64];
_3 = _2;
_4 = !false;
_6 = ['\u{80c34}'];
Goto(bb3)
}
bb8 = {
place!(Field::<i16>(Variant(RET, 1), 4)) = (-16857_i16);
_3 = _11.fld2.0;
_17.fld6 = !174_u8;
_19.1 = _17.fld6 as f64;
_6 = ['\u{49b1b}'];
_20 = _7;
_7 = _20;
match Field::<u16>(Variant(RET, 1), 0) {
0 => bb9,
30859 => bb11,
_ => bb10
}
}
bb9 = {
_4 = !true;
_4 = false ^ true;
_4 = !false;
_3 = _2;
_4 = !false;
_3 = [3487558627908679165_u64];
_2 = [13204875374639850805_u64];
_4 = false;
_4 = !false;
_2 = _3;
_2 = [2336645857221276564_u64];
_4 = !false;
_3 = _2;
_3 = [8332982583400787763_u64];
_4 = true ^ false;
_2 = [6612180888440374516_u64];
_3 = [10538725801412961918_u64];
_2 = [7429305373372616430_u64];
_3 = [6206020598278395849_u64];
Goto(bb2)
}
bb10 = {
place!(Field::<i16>(Variant(RET, 1), 4)) = -10514_i16;
_10.fld2.0 = [9185036746593908554_u64];
SetDiscriminant(RET, 1);
place!(Field::<u16>(Variant(RET, 1), 0)) = 30859_u16;
place!(Field::<([u64; 1],)>(Variant(RET, 1), 1)) = (_2,);
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)).0 = '\u{e94df}' as u16;
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)).1 = _4;
Goto(bb5)
}
bb11 = {
_16 = _10.fld2;
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)) = (Field::<u16>(Variant(RET, 1), 0), Field::<(bool,)>(Variant(RET, 1), 3).0);
place!(Field::<(bool,)>(Variant(RET, 1), 3)).0 = Field::<(u16, bool)>(Variant(RET, 1), 2).1;
_12 = '\u{d5b90}' as i128;
_19.1 = -_17.fld4;
_18 = [2239283005_u32,409093453_u32];
SetDiscriminant(RET, 0);
_4 = true;
_9 = _19.0 + _19.0;
place!(Field::<([u64; 1],)>(Variant(RET, 0), 2)) = _10.fld2;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)) = (_9, _17.fld4, _12, _9);
_16 = _11.fld2;
_19 = (Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3, Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).1, Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).2, Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3);
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).3 = -_19.3;
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _4;
Goto(bb12)
}
bb12 = {
_9 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0 - Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0;
_10.fld0 = _18;
_21 = _17.fld4 * _19.1;
place!(Field::<(bool,)>(Variant(RET, 0), 3)) = (_4,);
place!(Field::<(bool,)>(Variant(RET, 0), 3)) = (_4,);
_11.fld0 = [1286379090_u32,793629933_u32];
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).0 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3;
_11.fld2.0 = [9873965110016200047_u64];
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _15 <= _9;
Goto(bb13)
}
bb13 = {
place!(Field::<[isize; 5]>(Variant(RET, 0), 5)) = [_20,_20,_7,_20,_7];
_17.fld7 = Adt41::Variant0 { fld0: Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3 };
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).3 = -_9;
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _4;
Goto(bb14)
}
bb14 = {
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _4;
SetDiscriminant(_17.fld7, 1);
place!(Field::<[i32; 8]>(Variant(_17.fld7, 1), 7)) = [(-1916047730_i32),(-464582175_i32),171405279_i32,(-2114369061_i32),1614592157_i32,1101765763_i32,114001473_i32,(-143743897_i32)];
place!(Field::<(u16, bool)>(Variant(_17.fld7, 1), 1)).0 = 44859_u16;
_17.fld3 = ['\u{8927}'];
_17.fld5 = Adt46::Variant0 { fld0: '\u{10f0f4}' };
_23 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0 + _19.0;
place!(Field::<[u64; 1]>(Variant(_17.fld7, 1), 4)) = [15056435306087359753_u64];
match Field::<(u16, bool)>(Variant(_17.fld7, 1), 1).0 {
0 => bb1,
1 => bb2,
2 => bb13,
3 => bb7,
4 => bb12,
44859 => bb15,
_ => bb11
}
}
bb15 = {
place!(Field::<*const (u8,)>(Variant(RET, 0), 4)) = core::ptr::addr_of!(_13);
place!(Field::<(u16, bool)>(Variant(_17.fld7, 1), 1)).0 = 0_usize as u16;
place!(Field::<([u64; 1],)>(Variant(RET, 0), 2)) = (_10.fld2.0,);
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).0 = -_23;
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = !_4;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).1 = -_21;
_16 = (_3,);
_31 = _20 + _7;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).0 = _23 - _23;
place!(Field::<(u16, bool)>(Variant(_17.fld7, 1), 1)).1 = !Field::<(bool,)>(Variant(RET, 0), 3).0;
Goto(bb16)
}
bb16 = {
_19.1 = -_21;
_17.fld2 = Adt42::Variant2 { fld0: 16901248508655764860_u64,fld1: (-736831937858690931_i64),fld2: Field::<[isize; 5]>(Variant(RET, 0), 5),fld3: _19.0,fld4: 95084539200367394814914087039614391325_u128 };
_10.fld0 = _18;
_2 = _3;
place!(Field::<i64>(Variant(_17.fld7, 1), 6)) = 7_usize as i64;
place!(Field::<[isize; 5]>(Variant(RET, 0), 5)) = [_31,_20,_7,_20,_31];
_11.fld2.0 = [7232231933681172082_u64];
_28 = -_20;
_17.fld2 = Adt42::Variant2 { fld0: 13027342068542529993_u64,fld1: Field::<i64>(Variant(_17.fld7, 1), 6),fld2: Field::<[isize; 5]>(Variant(RET, 0), 5),fld3: _9,fld4: 76138043037877258691724108215414218043_u128 };
_30.0 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3 as usize;
_12 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).2 << _30.0;
_30 = (11516482051314442852_usize,);
place!(Field::<[isize; 5]>(Variant(_17.fld2, 2), 2)) = [_28,_20,_31,_28,_31];
place!(Field::<u64>(Variant(_17.fld2, 2), 0)) = 268247618479247884084657786792134723517_u128 as u64;
_14 = Adt56::Variant0 { fld0: _30 };
_26 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).1 < _17.fld4;
SetDiscriminant(_14, 2);
_6 = ['\u{95f59}'];
place!(Field::<(u16, bool)>(Variant(_17.fld7, 1), 1)).1 = Field::<(bool,)>(Variant(RET, 0), 3).0 | _26;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).2 = _19.2;
Goto(bb17)
}
bb17 = {
_17.fld4 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).1 - _21;
_13.0 = Field::<u64>(Variant(_17.fld2, 2), 0) as u8;
_24 = [Field::<u64>(Variant(_17.fld2, 2), 0)];
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = Field::<(u16, bool)>(Variant(_17.fld7, 1), 1).1;
_18 = [1871000196_u32,1115384086_u32];
Goto(bb18)
}
bb18 = {
_29 = (_13.0,);
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).2 = (-31446_i16) as i128;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)) = (_23, _21, _12, _19.0);
_8 = Adt44::Variant0 { fld0: Field::<u64>(Variant(_17.fld2, 2), 0) };
place!(Field::<[i32; 8]>(Variant(_14, 2), 2)) = [(-1177803730_i32),(-820835032_i32),(-1263812652_i32),909720232_i32,1823954932_i32,(-1962699725_i32),1978590434_i32,195285062_i32];
match _30.0 {
0 => bb9,
1 => bb5,
2 => bb7,
3 => bb16,
4 => bb19,
5 => bb20,
11516482051314442852 => bb22,
_ => bb21
}
}
bb19 = {
place!(Field::<[isize; 5]>(Variant(RET, 0), 5)) = [_20,_20,_7,_20,_7];
_17.fld7 = Adt41::Variant0 { fld0: Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3 };
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).3 = -_9;
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _4;
Goto(bb14)
}
bb20 = {
_16 = _10.fld2;
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)) = (Field::<u16>(Variant(RET, 1), 0), Field::<(bool,)>(Variant(RET, 1), 3).0);
place!(Field::<(bool,)>(Variant(RET, 1), 3)).0 = Field::<(u16, bool)>(Variant(RET, 1), 2).1;
_12 = '\u{d5b90}' as i128;
_19.1 = -_17.fld4;
_18 = [2239283005_u32,409093453_u32];
SetDiscriminant(RET, 0);
_4 = true;
_9 = _19.0 + _19.0;
place!(Field::<([u64; 1],)>(Variant(RET, 0), 2)) = _10.fld2;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)) = (_9, _17.fld4, _12, _9);
_16 = _11.fld2;
_19 = (Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3, Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).1, Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).2, Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3);
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).3 = -_19.3;
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _4;
Goto(bb12)
}
bb21 = {
_12 = -(-130852053365552175653816112822444160277_i128);
place!(Field::<(bool,)>(Variant(RET, 1), 3)).0 = !_4;
place!(Field::<i16>(Variant(RET, 1), 4)) = 24113_i16;
_17.fld2 = Move(RET);
RET = Move(_17.fld2);
place!(Field::<([u64; 1],)>(Variant(RET, 1), 1)) = _11.fld2;
place!(Field::<(bool,)>(Variant(RET, 1), 3)) = (Field::<(u16, bool)>(Variant(RET, 1), 2).1,);
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)).0 = !Field::<u16>(Variant(RET, 1), 0);
_2 = [9765182278113801098_u64];
_16.0 = _10.fld2.0;
_19.0 = _9 * _9;
_15 = _7 as f32;
_17.fld4 = _7 as f64;
_19.1 = _17.fld4;
_17.fld4 = -_19.1;
match Field::<u16>(Variant(RET, 1), 0) {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
30859 => bb8,
_ => bb7
}
}
bb22 = {
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = Field::<(u16, bool)>(Variant(_17.fld7, 1), 1).1;
place!(Field::<f32>(Variant(_17.fld2, 2), 3)) = -_23;
_3 = [Field::<u64>(Variant(_8, 0), 0)];
_32 = _28;
_12 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).2 - Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).2;
_10.fld2.0 = [Field::<u64>(Variant(_17.fld2, 2), 0)];
place!(Field::<Adt42>(Variant(_14, 2), 3)) = Adt42::Variant1 { fld0: Field::<(u16, bool)>(Variant(_17.fld7, 1), 1).0,fld1: _16,fld2: Field::<(u16, bool)>(Variant(_17.fld7, 1), 1),fld3: Field::<(bool,)>(Variant(RET, 0), 3),fld4: 15642_i16 };
_19.1 = 16451_i16 as f64;
place!(Field::<f32>(Variant(_17.fld2, 2), 3)) = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0 + Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).3 = _15 * Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)) = (Field::<f32>(Variant(_17.fld2, 2), 3), _17.fld4, _12, _19.0);
match _30.0 {
0 => bb19,
1 => bb2,
2 => bb3,
3 => bb12,
4 => bb23,
5 => bb24,
6 => bb25,
11516482051314442852 => bb27,
_ => bb26
}
}
bb23 = {
_19.1 = -_21;
_17.fld2 = Adt42::Variant2 { fld0: 16901248508655764860_u64,fld1: (-736831937858690931_i64),fld2: Field::<[isize; 5]>(Variant(RET, 0), 5),fld3: _19.0,fld4: 95084539200367394814914087039614391325_u128 };
_10.fld0 = _18;
_2 = _3;
place!(Field::<i64>(Variant(_17.fld7, 1), 6)) = 7_usize as i64;
place!(Field::<[isize; 5]>(Variant(RET, 0), 5)) = [_31,_20,_7,_20,_31];
_11.fld2.0 = [7232231933681172082_u64];
_28 = -_20;
_17.fld2 = Adt42::Variant2 { fld0: 13027342068542529993_u64,fld1: Field::<i64>(Variant(_17.fld7, 1), 6),fld2: Field::<[isize; 5]>(Variant(RET, 0), 5),fld3: _9,fld4: 76138043037877258691724108215414218043_u128 };
_30.0 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).3 as usize;
_12 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).2 << _30.0;
_30 = (11516482051314442852_usize,);
place!(Field::<[isize; 5]>(Variant(_17.fld2, 2), 2)) = [_28,_20,_31,_28,_31];
place!(Field::<u64>(Variant(_17.fld2, 2), 0)) = 268247618479247884084657786792134723517_u128 as u64;
_14 = Adt56::Variant0 { fld0: _30 };
_26 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).1 < _17.fld4;
SetDiscriminant(_14, 2);
_6 = ['\u{95f59}'];
place!(Field::<(u16, bool)>(Variant(_17.fld7, 1), 1)).1 = Field::<(bool,)>(Variant(RET, 0), 3).0 | _26;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).2 = _19.2;
Goto(bb17)
}
bb24 = {
_4 = !true;
_4 = false ^ true;
_4 = !false;
_3 = _2;
_4 = !false;
_3 = [3487558627908679165_u64];
_2 = [13204875374639850805_u64];
_4 = false;
_4 = !false;
_2 = _3;
_2 = [2336645857221276564_u64];
_4 = !false;
_3 = _2;
_3 = [8332982583400787763_u64];
_4 = true ^ false;
_2 = [6612180888440374516_u64];
_3 = [10538725801412961918_u64];
_2 = [7429305373372616430_u64];
_3 = [6206020598278395849_u64];
Goto(bb2)
}
bb25 = {
_2 = _3;
_4 = 129148224779601796677759623149868310393_i128 > (-162060575017055763661173275903724978188_i128);
_4 = false;
_3 = [1411339906520539636_u64];
_3 = [4154102501982281332_u64];
_3 = [14124857197606310524_u64];
_3 = _2;
_4 = !false;
_6 = ['\u{80c34}'];
Goto(bb3)
}
bb26 = {
place!(Field::<(bool,)>(Variant(RET, 0), 3)).0 = _4;
SetDiscriminant(_17.fld7, 1);
place!(Field::<[i32; 8]>(Variant(_17.fld7, 1), 7)) = [(-1916047730_i32),(-464582175_i32),171405279_i32,(-2114369061_i32),1614592157_i32,1101765763_i32,114001473_i32,(-143743897_i32)];
place!(Field::<(u16, bool)>(Variant(_17.fld7, 1), 1)).0 = 44859_u16;
_17.fld3 = ['\u{8927}'];
_17.fld5 = Adt46::Variant0 { fld0: '\u{10f0f4}' };
_23 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0 + _19.0;
place!(Field::<[u64; 1]>(Variant(_17.fld7, 1), 4)) = [15056435306087359753_u64];
match Field::<(u16, bool)>(Variant(_17.fld7, 1), 1).0 {
0 => bb1,
1 => bb2,
2 => bb13,
3 => bb7,
4 => bb12,
44859 => bb15,
_ => bb11
}
}
bb27 = {
_6 = ['\u{97334}'];
_7 = _19.1 as isize;
place!(Field::<[i32; 8]>(Variant(_17.fld7, 1), 7)) = [247971552_i32,1238880129_i32,(-2033809737_i32),465105581_i32,(-1717676186_i32),(-494158559_i32),1606481660_i32,1476207343_i32];
_13.0 = !_29.0;
place!(Field::<([u64; 1],)>(Variant(place!(Field::<Adt42>(Variant(_14, 2), 3)), 1), 1)) = (_2,);
place!(Field::<([u64; 1],)>(Variant(_17.fld7, 1), 2)) = Field::<([u64; 1],)>(Variant(RET, 0), 2);
_19.3 = _23 * Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0;
_26 = !Field::<(bool,)>(Variant(Field::<Adt42>(Variant(_14, 2), 3), 1), 3).0;
SetDiscriminant(_8, 0);
place!(Field::<char>(Variant(_14, 2), 1)) = '\u{1322}';
_26 = Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1).0 < _19.3;
_36 = Adt55::Variant1 { fld0: _30 };
place!(Field::<(usize,)>(Variant(_36, 1), 0)).0 = !_30.0;
_18 = _11.fld0;
SetDiscriminant(_36, 1);
place!(Field::<(usize,)>(Variant(_17.fld7, 1), 5)) = (_30.0,);
Goto(bb28)
}
bb28 = {
place!(Field::<(usize,)>(Variant(_36, 1), 0)) = _30;
_13 = (_17.fld6,);
_33 = _32;
place!(Field::<(f32, f64, i128, f32)>(Variant(RET, 0), 1)).2 = !_12;
place!(Field::<([u64; 1],)>(Variant(RET, 0), 2)) = (_24,);
RET = Adt42::Variant1 { fld0: Field::<(u16, bool)>(Variant(Field::<Adt42>(Variant(_14, 2), 3), 1), 2).0,fld1: _16,fld2: Field::<(u16, bool)>(Variant(_17.fld7, 1), 1),fld3: Field::<(bool,)>(Variant(Field::<Adt42>(Variant(_14, 2), 3), 1), 3),fld4: 18266_i16 };
place!(Field::<i16>(Variant(RET, 1), 4)) = Field::<i64>(Variant(_17.fld7, 1), 6) as i16;
place!(Field::<i16>(Variant(RET, 1), 4)) = _17.fld4 as i16;
_45 = 145995618198352747274052150884541366758_u128 as isize;
place!(Field::<(u16, bool)>(Variant(RET, 1), 2)) = (Field::<u16>(Variant(RET, 1), 0), _26);
_12 = _19.2 * _19.2;
place!(Field::<(usize,)>(Variant(_17.fld7, 1), 5)).0 = Field::<i16>(Variant(RET, 1), 4) as usize;
place!(Field::<([u64; 1],)>(Variant(place!(Field::<Adt42>(Variant(_14, 2), 3)), 1), 1)) = (_10.fld2.0,);
place!(Field::<i64>(Variant(_17.fld7, 1), 6)) = Field::<i64>(Variant(_17.fld2, 2), 1);
Goto(bb29)
}
bb29 = {
Call(_48 = dump_var(12_usize, 45_usize, Move(_45), 28_usize, Move(_28), 16_usize, Move(_16), 30_usize, Move(_30)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_48 = dump_var(12_usize, 2_usize, Move(_2), 26_usize, Move(_26), 31_usize, Move(_31), 33_usize, Move(_33)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_48 = dump_var(12_usize, 4_usize, Move(_4), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *mut (bool, f64, i8, u64, [u64; 1]),mut _2: [u32; 2],mut _3: *mut (bool, f64, i8, u64, [u64; 1]),mut _4: *mut (bool, f64, i8, u64, [u64; 1]),mut _5: *mut (bool, f64, i8, u64, [u64; 1]),mut _6: *mut (bool, f64, i8, u64, [u64; 1]),mut _7: *mut (bool, f64, i8, u64, [u64; 1]),mut _8: *mut (bool, f64, i8, u64, [u64; 1]),mut _9: *mut (bool, f64, i8, u64, [u64; 1]),mut _10: *mut (bool, f64, i8, u64, [u64; 1]),mut _11: *mut (bool, f64, i8, u64, [u64; 1]),mut _12: *mut (bool, f64, i8, u64, [u64; 1]),mut _13: *mut (bool, f64, i8, u64, [u64; 1]),mut _14: *mut (bool, f64, i8, u64, [u64; 1]),mut _15: *mut (bool, f64, i8, u64, [u64; 1]),mut _16: *mut (bool, f64, i8, u64, [u64; 1])) -> Adt42 {
mir! {
type RET = Adt42;
let _17: (bool,);
let _18: [i32; 8];
let _19: isize;
let _20: u32;
let _21: Adt53;
let _22: char;
let _23: [u64; 1];
let _24: isize;
let _25: f32;
let _26: char;
let _27: bool;
let _28: f32;
let _29: Adt44;
let _30: u16;
let _31: f32;
let _32: isize;
let _33: [isize; 5];
let _34: isize;
let _35: f64;
let _36: [char; 1];
let _37: Adt41;
let _38: Adt54;
let _39: [i32; 2];
let _40: (bool,);
let _41: isize;
let _42: (u8,);
let _43: f64;
let _44: f32;
let _45: Adt50;
let _46: f64;
let _47: u64;
let _48: (u8,);
let _49: [u32; 2];
let _50: f64;
let _51: isize;
let _52: Adt50;
let _53: isize;
let _54: ();
let _55: ();
{
_1 = _11;
_5 = _4;
_14 = _15;
_15 = _5;
_5 = _7;
_7 = _12;
_2 = [2578987414_u32,1180928157_u32];
_17 = (false,);
_18 = [(-1453757197_i32),(-2009039242_i32),204697549_i32,(-1043387999_i32),(-2120042252_i32),(-1860081738_i32),(-20622669_i32),1974646339_i32];
_4 = _8;
_5 = _8;
_13 = _16;
_2 = [1158281371_u32,3669767846_u32];
_17.0 = true;
_17.0 = false;
_17.0 = true;
_10 = _8;
_8 = _14;
_8 = _15;
_14 = _5;
_1 = _5;
Call(_13 = fn14(_9, _6, _7, _9, _1, _7, _6, _1, _15, _15, _8, _1, _7, _10, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17 = (false,);
_18 = [626945609_i32,(-1021482943_i32),(-501472956_i32),(-90317257_i32),1211624735_i32,(-1301943456_i32),1475488513_i32,1872437235_i32];
_8 = _5;
_17 = (true,);
_1 = _12;
_7 = _8;
_9 = _11;
Goto(bb2)
}
bb2 = {
_23 = [13337026514277612287_u64];
_13 = _6;
_13 = _8;
_14 = _11;
_17.0 = true ^ true;
_3 = _1;
_13 = _16;
_23 = [16723552330067349850_u64];
_3 = _9;
_18 = [445416404_i32,1784793116_i32,461187645_i32,(-1947824977_i32),430713532_i32,(-1276329718_i32),1541541112_i32,(-2033439423_i32)];
_6 = _8;
_5 = _7;
_9 = _12;
_18 = [(-483360954_i32),1850734949_i32,1296328559_i32,1862580940_i32,(-675616356_i32),(-1555395447_i32),(-2002480816_i32),(-132432439_i32)];
_17 = (false,);
_25 = 17398467733790677002_u64 as f32;
_12 = _4;
_22 = '\u{69d10}';
_8 = _11;
_22 = '\u{3e449}';
_19 = -(-9223372036854775808_isize);
_25 = 33_u8 as f32;
_1 = _8;
_16 = _13;
_2 = [1857048648_u32,753341285_u32];
Goto(bb3)
}
bb3 = {
_4 = _1;
_5 = _10;
_11 = _6;
_15 = _14;
_10 = _14;
_6 = _12;
_20 = 2822752824_u32;
Goto(bb4)
}
bb4 = {
_11 = _15;
_3 = _10;
_20 = !1073017629_u32;
_2 = [_20,_20];
_14 = _13;
_24 = _25 as isize;
_12 = _9;
_24 = _19 << _19;
_26 = _22;
Goto(bb5)
}
bb5 = {
_26 = _22;
_2 = [_20,_20];
_25 = 115_u8 as f32;
_10 = _6;
_20 = _22 as u32;
_27 = _17.0;
_16 = _1;
_25 = 4678953752689592666_u64 as f32;
_14 = _8;
Goto(bb6)
}
bb6 = {
_17 = (_27,);
_9 = _10;
_10 = _16;
Goto(bb7)
}
bb7 = {
_8 = _6;
_26 = _22;
_10 = _11;
_24 = -_19;
_25 = _19 as f32;
_22 = _26;
Goto(bb8)
}
bb8 = {
_6 = _12;
_26 = _22;
_6 = _8;
_17.0 = _20 > _20;
Goto(bb9)
}
bb9 = {
_20 = !2468888315_u32;
_18 = [(-1554637962_i32),(-1549556685_i32),(-1675854948_i32),2128683451_i32,(-1167132631_i32),1079706243_i32,(-158508849_i32),(-657519111_i32)];
_3 = _6;
_19 = -_24;
_12 = _15;
_16 = _5;
_28 = -_25;
_19 = 294144240431035745326789607586872920962_u128 as isize;
_9 = _1;
_15 = _8;
_10 = _13;
Call(_18 = fn15(_14, _6, _11, _1, _12), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = _8;
_30 = _25 as u16;
_11 = _16;
_26 = _22;
_9 = _14;
_6 = _12;
_8 = _14;
Goto(bb11)
}
bb11 = {
_18 = [295986405_i32,(-19435980_i32),1283029868_i32,(-19352386_i32),109360017_i32,(-972563796_i32),(-431839575_i32),1550283109_i32];
_7 = _16;
_17.0 = !_27;
_20 = _17.0 as u32;
_15 = _16;
_19 = _24 + _24;
_9 = _7;
_1 = _14;
_23 = [17332896573767762389_u64];
_28 = 4857041114238476632_i64 as f32;
_22 = _26;
_18 = [1850401360_i32,(-1704135799_i32),2126142134_i32,518445535_i32,1945781105_i32,1731026988_i32,1470275185_i32,1062849899_i32];
_33 = [_19,_24,_24,_19,_24];
_34 = _19;
_10 = _11;
_33 = [_34,_24,_34,_24,_34];
_36 = [_22];
_12 = _14;
_33 = [_19,_24,_24,_24,_34];
Goto(bb12)
}
bb12 = {
_6 = _11;
_17 = (_27,);
Goto(bb13)
}
bb13 = {
_19 = _34;
_28 = 2_u8 as f32;
_35 = 13971459657334284418_u64 as f64;
_2 = [_20,_20];
_34 = _19;
Goto(bb14)
}
bb14 = {
_38.fld0 = [_20,_20];
_18 = [300830818_i32,399558233_i32,(-838922884_i32),(-1152003093_i32),(-1137649624_i32),(-1947765887_i32),655586578_i32,2019412863_i32];
_25 = (-32_i8) as f32;
_39 = [1026586054_i32,(-2064911499_i32)];
_40 = (_17.0,);
_23 = [8975049001407637511_u64];
_25 = _28;
_39 = [(-237746314_i32),(-90447276_i32)];
_34 = _24;
_41 = _34;
_32 = _41;
_37 = Adt41::Variant0 { fld0: _28 };
_8 = _6;
_45.fld1 = !_30;
_45.fld4 = -_35;
_18 = [(-320293765_i32),791026894_i32,825416593_i32,1665062354_i32,(-2068882205_i32),1886191742_i32,722201430_i32,(-721683176_i32)];
_45.fld1 = !_30;
_43 = -_35;
RET = Adt42::Variant2 { fld0: 3015516282660205941_u64,fld1: 7302099600117048077_i64,fld2: _33,fld3: _25,fld4: 41677404133391799729195601509029073696_u128 };
Goto(bb15)
}
bb15 = {
_42.0 = 13_u8;
_11 = _16;
_31 = -_25;
_46 = 1796269952_i32 as f64;
_22 = _26;
_20 = 2924460999_u32 + 2371040488_u32;
_11 = _5;
_32 = _19 << _41;
SetDiscriminant(_37, 1);
_45.fld5 = Adt46::Variant0 { fld0: _22 };
Goto(bb16)
}
bb16 = {
_45.fld6 = !_42.0;
_11 = _13;
place!(Field::<i64>(Variant(RET, 2), 1)) = _43 as i64;
place!(Field::<[u64; 1]>(Variant(_37, 1), 4)) = [3471205196051038374_u64];
_8 = _4;
SetDiscriminant(_45.fld5, 0);
place!(Field::<[isize; 5]>(Variant(RET, 2), 2)) = _33;
_27 = _45.fld6 != _42.0;
RET = Adt42::Variant2 { fld0: 10830622592014311331_u64,fld1: (-6329267397373074486_i64),fld2: _33,fld3: _25,fld4: 44854567572342834892972077605053029502_u128 };
place!(Field::<[i32; 8]>(Variant(_37, 1), 7)) = [622974593_i32,721640219_i32,1410610667_i32,1940702834_i32,1317011601_i32,726096048_i32,(-1156460387_i32),1802644181_i32];
place!(Field::<u128>(Variant(RET, 2), 4)) = _32 as u128;
place!(Field::<(u16, bool)>(Variant(_37, 1), 1)).1 = _27;
place!(Field::<i64>(Variant(_37, 1), 6)) = (-3135924895059799819_i64) - 7147637307215738465_i64;
_45.fld3 = [_22];
_31 = _25;
_42.0 = _26 as u8;
place!(Field::<i64>(Variant(_37, 1), 6)) = (-994119058516409418_i64);
place!(Field::<i8>(Variant(_37, 1), 3)) = !(-15_i8);
_8 = _11;
place!(Field::<(usize,)>(Variant(_37, 1), 5)).0 = 10135783200274855530_usize;
_45.fld2 = Adt42::Variant2 { fld0: 1003565712701843762_u64,fld1: Field::<i64>(Variant(_37, 1), 6),fld2: _33,fld3: _31,fld4: Field::<u128>(Variant(RET, 2), 4) };
_40.0 = Field::<(u16, bool)>(Variant(_37, 1), 1).1 & Field::<(u16, bool)>(Variant(_37, 1), 1).1;
place!(Field::<(u16, bool)>(Variant(_37, 1), 1)).1 = _24 != _32;
_10 = _11;
match Field::<(usize,)>(Variant(_37, 1), 5).0 {
0 => bb7,
1 => bb8,
2 => bb12,
3 => bb10,
4 => bb9,
5 => bb11,
6 => bb17,
10135783200274855530 => bb19,
_ => bb18
}
}
bb17 = {
_4 = _1;
_5 = _10;
_11 = _6;
_15 = _14;
_10 = _14;
_6 = _12;
_20 = 2822752824_u32;
Goto(bb4)
}
bb18 = {
_26 = _22;
_2 = [_20,_20];
_25 = 115_u8 as f32;
_10 = _6;
_20 = _22 as u32;
_27 = _17.0;
_16 = _1;
_25 = 4678953752689592666_u64 as f32;
_14 = _8;
Goto(bb6)
}
bb19 = {
place!(Field::<[i32; 8]>(Variant(_37, 1), 7)) = [(-2105251698_i32),(-1249383208_i32),(-359822994_i32),(-662842441_i32),1839162025_i32,1606200465_i32,(-623245730_i32),1643713379_i32];
_26 = _22;
_2 = _38.fld0;
place!(Field::<i8>(Variant(_37, 1), 3)) = (-89_i8);
_45.fld7 = Adt41::Variant0 { fld0: Field::<f32>(Variant(_45.fld2, 2), 3) };
_45.fld1 = 10963651435486635086_u64 as u16;
_25 = Field::<u128>(Variant(_45.fld2, 2), 4) as f32;
place!(Field::<(u16, bool)>(Variant(_37, 1), 1)).0 = _30 * _30;
_39 = [1701399104_i32,37424167_i32];
_48.0 = !_42.0;
_45.fld2 = Adt42::Variant2 { fld0: 14173255808824156468_u64,fld1: Field::<i64>(Variant(_37, 1), 6),fld2: _33,fld3: _25,fld4: Field::<u128>(Variant(RET, 2), 4) };
_43 = _25 as f64;
place!(Field::<i64>(Variant(RET, 2), 1)) = -Field::<i64>(Variant(_37, 1), 6);
_46 = _43 * _43;
match Field::<i64>(Variant(_45.fld2, 2), 1) {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb18,
4 => bb8,
5 => bb20,
340282366920938463462380488373251802038 => bb22,
_ => bb21
}
}
bb20 = {
_20 = !2468888315_u32;
_18 = [(-1554637962_i32),(-1549556685_i32),(-1675854948_i32),2128683451_i32,(-1167132631_i32),1079706243_i32,(-158508849_i32),(-657519111_i32)];
_3 = _6;
_19 = -_24;
_12 = _15;
_16 = _5;
_28 = -_25;
_19 = 294144240431035745326789607586872920962_u128 as isize;
_9 = _1;
_15 = _8;
_10 = _13;
Call(_18 = fn15(_14, _6, _11, _1, _12), ReturnTo(bb10), UnwindUnreachable())
}
bb21 = {
_8 = _6;
_26 = _22;
_10 = _11;
_24 = -_19;
_25 = _19 as f32;
_22 = _26;
Goto(bb8)
}
bb22 = {
_44 = _25 + Field::<f32>(Variant(_45.fld2, 2), 3);
place!(Field::<([u64; 1],)>(Variant(_37, 1), 2)) = (_23,);
_12 = _13;
_45.fld4 = _44 as f64;
RET = Adt42::Variant1 { fld0: _30,fld1: Field::<([u64; 1],)>(Variant(_37, 1), 2),fld2: Field::<(u16, bool)>(Variant(_37, 1), 1),fld3: _17,fld4: 3973_i16 };
place!(Field::<i16>(Variant(RET, 1), 4)) = !(-7762_i16);
_52.fld2 = Move(RET);
RET = Move(_52.fld2);
Goto(bb23)
}
bb23 = {
Call(_54 = dump_var(13_usize, 19_usize, Move(_19), 27_usize, Move(_27), 40_usize, Move(_40), 17_usize, Move(_17)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_54 = dump_var(13_usize, 42_usize, Move(_42), 32_usize, Move(_32), 30_usize, Move(_30), 22_usize, Move(_22)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_54 = dump_var(13_usize, 48_usize, Move(_48), 39_usize, Move(_39), 55_usize, _55, 55_usize, _55), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *mut (bool, f64, i8, u64, [u64; 1]),mut _2: *mut (bool, f64, i8, u64, [u64; 1]),mut _3: *mut (bool, f64, i8, u64, [u64; 1]),mut _4: *mut (bool, f64, i8, u64, [u64; 1]),mut _5: *mut (bool, f64, i8, u64, [u64; 1]),mut _6: *mut (bool, f64, i8, u64, [u64; 1]),mut _7: *mut (bool, f64, i8, u64, [u64; 1]),mut _8: *mut (bool, f64, i8, u64, [u64; 1]),mut _9: *mut (bool, f64, i8, u64, [u64; 1]),mut _10: *mut (bool, f64, i8, u64, [u64; 1]),mut _11: *mut (bool, f64, i8, u64, [u64; 1]),mut _12: *mut (bool, f64, i8, u64, [u64; 1]),mut _13: *mut (bool, f64, i8, u64, [u64; 1]),mut _14: *mut (bool, f64, i8, u64, [u64; 1]),mut _15: *mut (bool, f64, i8, u64, [u64; 1])) -> *mut (bool, f64, i8, u64, [u64; 1]) {
mir! {
type RET = *mut (bool, f64, i8, u64, [u64; 1]);
let _16: i8;
let _17: [isize; 5];
let _18: ();
let _19: ();
{
RET = _9;
_8 = _14;
_7 = _12;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: *mut (bool, f64, i8, u64, [u64; 1]),mut _2: *mut (bool, f64, i8, u64, [u64; 1]),mut _3: *mut (bool, f64, i8, u64, [u64; 1]),mut _4: *mut (bool, f64, i8, u64, [u64; 1]),mut _5: *mut (bool, f64, i8, u64, [u64; 1])) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _6: u8;
let _7: isize;
let _8: [i32; 2];
let _9: u32;
let _10: ([u64; 1],);
let _11: [i8; 5];
let _12: [isize; 5];
let _13: i16;
let _14: (usize,);
let _15: usize;
let _16: (f32, f64, i128, f32);
let _17: bool;
let _18: bool;
let _19: [u64; 1];
let _20: ([u64; 1],);
let _21: u32;
let _22: u32;
let _23: (bool,);
let _24: isize;
let _25: u128;
let _26: f64;
let _27: u64;
let _28: ();
let _29: ();
{
RET = [(-1961987904_i32),375489887_i32,1016227914_i32,1071668838_i32,(-2079676717_i32),(-39316062_i32),1859129643_i32,(-859623680_i32)];
_5 = _3;
_1 = _2;
_2 = _4;
_5 = _1;
_5 = _4;
_5 = _1;
_3 = _4;
_4 = _5;
_3 = _4;
_7 = -(-9223372036854775808_isize);
_7 = -(-114_isize);
_1 = _5;
_2 = _4;
_7 = (-9223372036854775808_isize) >> 42_i8;
RET = [1591228508_i32,(-1349740726_i32),750422913_i32,(-772103721_i32),(-548494388_i32),923724243_i32,2111013689_i32,(-902516570_i32)];
RET = [1361081547_i32,(-123543887_i32),(-1054048880_i32),1722444503_i32,1031931795_i32,(-1894526710_i32),156675152_i32,(-2464785_i32)];
Goto(bb1)
}
bb1 = {
_2 = _1;
_2 = _1;
_1 = _5;
_1 = _4;
_2 = _5;
_2 = _3;
_1 = _5;
_8 = [(-279376450_i32),(-2094787767_i32)];
_6 = true as u8;
RET = [(-1848116321_i32),(-493411093_i32),(-1592450172_i32),1994943850_i32,79986087_i32,91642614_i32,2067451509_i32,(-1582751685_i32)];
_4 = _3;
RET = [2044786765_i32,2085131175_i32,993767526_i32,(-416931735_i32),2117593781_i32,386071328_i32,(-919766282_i32),334655594_i32];
RET = [1227348567_i32,359155746_i32,2140765186_i32,1447287915_i32,(-894395723_i32),(-396872477_i32),(-1070909080_i32),(-1871481344_i32)];
_9 = 2684981253_u32;
_8 = [1502081815_i32,606606107_i32];
Call(_10.0 = fn16(_4, _5, _4, _3, _5, _2, _4, _2, _2, _1, _5, _3, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = 1858815919_u32;
_2 = _1;
_2 = _4;
_5 = _1;
_3 = _2;
_4 = _1;
_2 = _4;
_8 = [1460431410_i32,1690378930_i32];
_9 = !3962605451_u32;
_6 = 246_u8 >> _7;
_8 = [(-1791252802_i32),629046547_i32];
_13 = 31229_i16;
RET = [(-213813908_i32),1951419996_i32,1403425880_i32,(-1407737613_i32),(-1873625477_i32),(-1726846770_i32),(-644186130_i32),235096179_i32];
_13 = !15881_i16;
_6 = 155_u8 << _7;
_13 = (-20331_i16) & 1572_i16;
_5 = _4;
_14 = (16655500556727488788_usize,);
_14 = (16865218536893113492_usize,);
_12 = [_7,_7,_7,_7,_7];
_6 = 240333663789189130500688554167471188212_u128 as u8;
_11 = [(-20_i8),108_i8,87_i8,25_i8,(-55_i8)];
_9 = 3061213652_u32 ^ 2902858168_u32;
_7 = 189864785207222337688001340461758573815_u128 as isize;
_8 = [1028042444_i32,2061924668_i32];
_7 = 72_isize ^ (-9223372036854775808_isize);
_7 = 125102625779309958580349464195350412557_i128 as isize;
Goto(bb3)
}
bb3 = {
_11 = [(-89_i8),119_i8,111_i8,0_i8,53_i8];
match _14.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
16865218536893113492 => bb9,
_ => bb8
}
}
bb4 = {
_9 = 1858815919_u32;
_2 = _1;
_2 = _4;
_5 = _1;
_3 = _2;
_4 = _1;
_2 = _4;
_8 = [1460431410_i32,1690378930_i32];
_9 = !3962605451_u32;
_6 = 246_u8 >> _7;
_8 = [(-1791252802_i32),629046547_i32];
_13 = 31229_i16;
RET = [(-213813908_i32),1951419996_i32,1403425880_i32,(-1407737613_i32),(-1873625477_i32),(-1726846770_i32),(-644186130_i32),235096179_i32];
_13 = !15881_i16;
_6 = 155_u8 << _7;
_13 = (-20331_i16) & 1572_i16;
_5 = _4;
_14 = (16655500556727488788_usize,);
_14 = (16865218536893113492_usize,);
_12 = [_7,_7,_7,_7,_7];
_6 = 240333663789189130500688554167471188212_u128 as u8;
_11 = [(-20_i8),108_i8,87_i8,25_i8,(-55_i8)];
_9 = 3061213652_u32 ^ 2902858168_u32;
_7 = 189864785207222337688001340461758573815_u128 as isize;
_8 = [1028042444_i32,2061924668_i32];
_7 = 72_isize ^ (-9223372036854775808_isize);
_7 = 125102625779309958580349464195350412557_i128 as isize;
Goto(bb3)
}
bb5 = {
_2 = _1;
_2 = _1;
_1 = _5;
_1 = _4;
_2 = _5;
_2 = _3;
_1 = _5;
_8 = [(-279376450_i32),(-2094787767_i32)];
_6 = true as u8;
RET = [(-1848116321_i32),(-493411093_i32),(-1592450172_i32),1994943850_i32,79986087_i32,91642614_i32,2067451509_i32,(-1582751685_i32)];
_4 = _3;
RET = [2044786765_i32,2085131175_i32,993767526_i32,(-416931735_i32),2117593781_i32,386071328_i32,(-919766282_i32),334655594_i32];
RET = [1227348567_i32,359155746_i32,2140765186_i32,1447287915_i32,(-894395723_i32),(-396872477_i32),(-1070909080_i32),(-1871481344_i32)];
_9 = 2684981253_u32;
_8 = [1502081815_i32,606606107_i32];
Call(_10.0 = fn16(_4, _5, _4, _3, _5, _2, _4, _2, _2, _1, _5, _3, _5, _2), ReturnTo(bb2), UnwindUnreachable())
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
_4 = _3;
_8 = [(-366535873_i32),1140276668_i32];
_4 = _2;
_12 = [_7,_7,_7,_7,_7];
_7 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_13 = 11209_i16;
_2 = _4;
_5 = _1;
_10.0 = [15776372517461434058_u64];
_10.0 = [10001524888259432082_u64];
_5 = _4;
_16.0 = 17903206719041504548_u64 as f32;
_13 = 21647_i16;
_10.0 = [9544163088328453636_u64];
_15 = !_14.0;
_16.0 = _6 as f32;
_6 = 233_u8 * 41_u8;
_10.0 = [772682580108739446_u64];
_16.0 = 2913032092085115881_u64 as f32;
_6 = !213_u8;
_8 = [1171813154_i32,(-1203004645_i32)];
_9 = 962952415_u32 + 3787098955_u32;
_4 = _1;
_9 = !3303113104_u32;
_18 = _14.0 > _15;
_16.3 = _16.0;
_18 = false;
_16.2 = 144864475014060017770029946062736537919_i128 << _6;
match _14.0 {
0 => bb1,
1 => bb5,
2 => bb10,
3 => bb11,
16865218536893113492 => bb13,
_ => bb12
}
}
bb10 = {
_11 = [(-89_i8),119_i8,111_i8,0_i8,53_i8];
match _14.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
16865218536893113492 => bb9,
_ => bb8
}
}
bb11 = {
Return()
}
bb12 = {
_2 = _1;
_2 = _1;
_1 = _5;
_1 = _4;
_2 = _5;
_2 = _3;
_1 = _5;
_8 = [(-279376450_i32),(-2094787767_i32)];
_6 = true as u8;
RET = [(-1848116321_i32),(-493411093_i32),(-1592450172_i32),1994943850_i32,79986087_i32,91642614_i32,2067451509_i32,(-1582751685_i32)];
_4 = _3;
RET = [2044786765_i32,2085131175_i32,993767526_i32,(-416931735_i32),2117593781_i32,386071328_i32,(-919766282_i32),334655594_i32];
RET = [1227348567_i32,359155746_i32,2140765186_i32,1447287915_i32,(-894395723_i32),(-396872477_i32),(-1070909080_i32),(-1871481344_i32)];
_9 = 2684981253_u32;
_8 = [1502081815_i32,606606107_i32];
Call(_10.0 = fn16(_4, _5, _4, _3, _5, _2, _4, _2, _2, _1, _5, _3, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_17 = _18;
RET = [1634016396_i32,860124506_i32,(-543768673_i32),(-1752927614_i32),(-1657972988_i32),(-1661507184_i32),1485816607_i32,1067596178_i32];
_10.0 = [11192615372868616518_u64];
_16.0 = _16.3;
_11 = [(-56_i8),37_i8,51_i8,78_i8,(-56_i8)];
_12 = [_7,_7,_7,_7,_7];
_10.0 = [6649991803130473893_u64];
_5 = _3;
_20.0 = [16160374696133516189_u64];
_1 = _5;
_14 = (_15,);
_19 = _20.0;
_16.1 = 80_i8 as f64;
_16.2 = 95696843123064394846190392631649894444_i128;
_15 = _14.0;
_18 = !_17;
_15 = '\u{620df}' as usize;
_10.0 = _20.0;
_22 = _9 ^ _9;
_23 = (_18,);
_19 = [5699382747535275932_u64];
_16.2 = (-99783198963154980585550557489289196584_i128) & 58000921676385344736767497870558546421_i128;
match _13 {
0 => bb4,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
21647 => bb20,
_ => bb19
}
}
bb14 = {
_9 = 1858815919_u32;
_2 = _1;
_2 = _4;
_5 = _1;
_3 = _2;
_4 = _1;
_2 = _4;
_8 = [1460431410_i32,1690378930_i32];
_9 = !3962605451_u32;
_6 = 246_u8 >> _7;
_8 = [(-1791252802_i32),629046547_i32];
_13 = 31229_i16;
RET = [(-213813908_i32),1951419996_i32,1403425880_i32,(-1407737613_i32),(-1873625477_i32),(-1726846770_i32),(-644186130_i32),235096179_i32];
_13 = !15881_i16;
_6 = 155_u8 << _7;
_13 = (-20331_i16) & 1572_i16;
_5 = _4;
_14 = (16655500556727488788_usize,);
_14 = (16865218536893113492_usize,);
_12 = [_7,_7,_7,_7,_7];
_6 = 240333663789189130500688554167471188212_u128 as u8;
_11 = [(-20_i8),108_i8,87_i8,25_i8,(-55_i8)];
_9 = 3061213652_u32 ^ 2902858168_u32;
_7 = 189864785207222337688001340461758573815_u128 as isize;
_8 = [1028042444_i32,2061924668_i32];
_7 = 72_isize ^ (-9223372036854775808_isize);
_7 = 125102625779309958580349464195350412557_i128 as isize;
Goto(bb3)
}
bb15 = {
_2 = _1;
_2 = _1;
_1 = _5;
_1 = _4;
_2 = _5;
_2 = _3;
_1 = _5;
_8 = [(-279376450_i32),(-2094787767_i32)];
_6 = true as u8;
RET = [(-1848116321_i32),(-493411093_i32),(-1592450172_i32),1994943850_i32,79986087_i32,91642614_i32,2067451509_i32,(-1582751685_i32)];
_4 = _3;
RET = [2044786765_i32,2085131175_i32,993767526_i32,(-416931735_i32),2117593781_i32,386071328_i32,(-919766282_i32),334655594_i32];
RET = [1227348567_i32,359155746_i32,2140765186_i32,1447287915_i32,(-894395723_i32),(-396872477_i32),(-1070909080_i32),(-1871481344_i32)];
_9 = 2684981253_u32;
_8 = [1502081815_i32,606606107_i32];
Call(_10.0 = fn16(_4, _5, _4, _3, _5, _2, _4, _2, _2, _1, _5, _3, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_11 = [(-89_i8),119_i8,111_i8,0_i8,53_i8];
match _14.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
16865218536893113492 => bb9,
_ => bb8
}
}
bb17 = {
_4 = _3;
_8 = [(-366535873_i32),1140276668_i32];
_4 = _2;
_12 = [_7,_7,_7,_7,_7];
_7 = (-9223372036854775808_isize) + 9223372036854775807_isize;
_13 = 11209_i16;
_2 = _4;
_5 = _1;
_10.0 = [15776372517461434058_u64];
_10.0 = [10001524888259432082_u64];
_5 = _4;
_16.0 = 17903206719041504548_u64 as f32;
_13 = 21647_i16;
_10.0 = [9544163088328453636_u64];
_15 = !_14.0;
_16.0 = _6 as f32;
_6 = 233_u8 * 41_u8;
_10.0 = [772682580108739446_u64];
_16.0 = 2913032092085115881_u64 as f32;
_6 = !213_u8;
_8 = [1171813154_i32,(-1203004645_i32)];
_9 = 962952415_u32 + 3787098955_u32;
_4 = _1;
_9 = !3303113104_u32;
_18 = _14.0 > _15;
_16.3 = _16.0;
_18 = false;
_16.2 = 144864475014060017770029946062736537919_i128 << _6;
match _14.0 {
0 => bb1,
1 => bb5,
2 => bb10,
3 => bb11,
16865218536893113492 => bb13,
_ => bb12
}
}
bb18 = {
_2 = _1;
_2 = _1;
_1 = _5;
_1 = _4;
_2 = _5;
_2 = _3;
_1 = _5;
_8 = [(-279376450_i32),(-2094787767_i32)];
_6 = true as u8;
RET = [(-1848116321_i32),(-493411093_i32),(-1592450172_i32),1994943850_i32,79986087_i32,91642614_i32,2067451509_i32,(-1582751685_i32)];
_4 = _3;
RET = [2044786765_i32,2085131175_i32,993767526_i32,(-416931735_i32),2117593781_i32,386071328_i32,(-919766282_i32),334655594_i32];
RET = [1227348567_i32,359155746_i32,2140765186_i32,1447287915_i32,(-894395723_i32),(-396872477_i32),(-1070909080_i32),(-1871481344_i32)];
_9 = 2684981253_u32;
_8 = [1502081815_i32,606606107_i32];
Call(_10.0 = fn16(_4, _5, _4, _3, _5, _2, _4, _2, _2, _1, _5, _3, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb19 = {
Return()
}
bb20 = {
_8 = [1167822253_i32,718984914_i32];
_6 = !159_u8;
_2 = _4;
_17 = _18;
_16.2 = -102485278762415570927879972762804881267_i128;
_10.0 = [5722330556595666831_u64];
_16.0 = _16.3 - _16.3;
_9 = _22 + _22;
_3 = _4;
_22 = _9 - _9;
_16.2 = _7 as i128;
RET = [1371754075_i32,196055195_i32,(-112683556_i32),952194785_i32,1517863789_i32,(-1294108909_i32),(-1048741853_i32),(-1855078088_i32)];
_14.0 = _15 & _15;
_5 = _2;
_20 = _10;
RET = [1466375364_i32,958694306_i32,701217265_i32,(-1315214709_i32),1348767646_i32,(-1966117399_i32),(-2092113548_i32),(-1357201258_i32)];
_26 = _15 as f64;
_10.0 = _20.0;
_23 = (_17,);
_18 = !_17;
Goto(bb21)
}
bb21 = {
Call(_28 = dump_var(15_usize, 11_usize, Move(_11), 22_usize, Move(_22), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_28 = dump_var(15_usize, 8_usize, Move(_8), 13_usize, Move(_13), 20_usize, Move(_20), 10_usize, Move(_10)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *mut (bool, f64, i8, u64, [u64; 1]),mut _2: *mut (bool, f64, i8, u64, [u64; 1]),mut _3: *mut (bool, f64, i8, u64, [u64; 1]),mut _4: *mut (bool, f64, i8, u64, [u64; 1]),mut _5: *mut (bool, f64, i8, u64, [u64; 1]),mut _6: *mut (bool, f64, i8, u64, [u64; 1]),mut _7: *mut (bool, f64, i8, u64, [u64; 1]),mut _8: *mut (bool, f64, i8, u64, [u64; 1]),mut _9: *mut (bool, f64, i8, u64, [u64; 1]),mut _10: *mut (bool, f64, i8, u64, [u64; 1]),mut _11: *mut (bool, f64, i8, u64, [u64; 1]),mut _12: *mut (bool, f64, i8, u64, [u64; 1]),mut _13: *mut (bool, f64, i8, u64, [u64; 1]),mut _14: *mut (bool, f64, i8, u64, [u64; 1])) -> [u64; 1] {
mir! {
type RET = [u64; 1];
let _15: u64;
let _16: bool;
let _17: u16;
let _18: f32;
let _19: i64;
let _20: bool;
let _21: u128;
let _22: char;
let _23: char;
let _24: isize;
let _25: isize;
let _26: Adt56;
let _27: isize;
let _28: u64;
let _29: (f32, f64, i128, f32);
let _30: [i32; 8];
let _31: [i32; 8];
let _32: (bool, f64, i8, u64, [u64; 1]);
let _33: f32;
let _34: Adt43;
let _35: Adt49;
let _36: *mut i128;
let _37: [isize; 5];
let _38: bool;
let _39: [i8; 5];
let _40: [i32; 2];
let _41: (u16, bool);
let _42: u32;
let _43: i128;
let _44: *mut (bool, f64, i8, u64, [u64; 1]);
let _45: f32;
let _46: isize;
let _47: ();
let _48: ();
{
_10 = _4;
_2 = _9;
_10 = _13;
_10 = _8;
_9 = _5;
RET = [13571715557673090506_u64];
_7 = _12;
_7 = _2;
RET = [11084113160777855069_u64];
_11 = _13;
_6 = _12;
_15 = !1782852914932438463_u64;
_12 = _8;
_5 = _11;
_8 = _6;
_8 = _11;
_9 = _5;
_10 = _6;
_9 = _14;
_2 = _14;
_1 = _3;
_8 = _12;
_11 = _5;
_5 = _4;
_5 = _13;
_4 = _1;
Goto(bb1)
}
bb1 = {
_15 = 488124188311388750_u64;
_12 = _2;
_16 = false;
_18 = (-56_i8) as f32;
_17 = !50523_u16;
_13 = _3;
_17 = 41814_u16 & 12846_u16;
_3 = _5;
_17 = (-1460779578_i32) as u16;
_11 = _14;
_15 = 5416001810933891074_u64 * 18018000811117815594_u64;
_17 = !10437_u16;
_3 = _2;
_19 = (-913710438104157701_i64) - 4564294722194685033_i64;
_18 = 9223372036854775807_isize as f32;
_8 = _3;
_6 = _11;
_22 = '\u{10021d}';
_15 = !14781631006983384663_u64;
_1 = _8;
RET = [_15];
_23 = _22;
_10 = _14;
_6 = _7;
Goto(bb2)
}
bb2 = {
_10 = _1;
Goto(bb3)
}
bb3 = {
_18 = 166752424597292521992860491167212632368_u128 as f32;
RET = [_15];
_19 = _18 as i64;
_23 = _22;
_11 = _10;
_21 = 203916920150505945369860726589391235134_u128 >> _17;
_7 = _4;
_15 = 11624999111055033786_u64 * 15261282362108024417_u64;
RET = [_15];
RET = [_15];
_22 = _23;
RET = [_15];
_20 = !_16;
_27 = (-119_isize);
_24 = _27 - _27;
_17 = !47651_u16;
_15 = 1646317959_u32 as u64;
RET = [_15];
_10 = _14;
_5 = _7;
_20 = _16;
_6 = _8;
_2 = _9;
_21 = 86476399978244798624146152538378773223_u128;
_20 = !_16;
_3 = _2;
_17 = !22364_u16;
_21 = 133739151776159269342603329574894003898_u128 ^ 187330011673727786173361217950297813400_u128;
Goto(bb4)
}
bb4 = {
_29.3 = _18 * _18;
_18 = _29.3;
_1 = _2;
_15 = 16600896293664512447_u64 ^ 10687294420174753939_u64;
_21 = !56893732997737732680207613820395593657_u128;
_22 = _23;
_8 = _12;
_13 = _7;
_14 = _12;
_29.0 = _29.3;
_29.2 = 3184_i16 as i128;
_18 = _29.3 * _29.0;
_25 = _24;
_3 = _7;
_12 = _7;
_24 = -_25;
_29.1 = _19 as f64;
_21 = 262497278297779093049261860624976588613_u128 >> _27;
_29.3 = _18;
_3 = _10;
_1 = _8;
_2 = _14;
_19 = _15 as i64;
_28 = _15 >> _24;
Goto(bb5)
}
bb5 = {
_20 = !_16;
match _27 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768211337 => bb11,
_ => bb10
}
}
bb6 = {
_29.3 = _18 * _18;
_18 = _29.3;
_1 = _2;
_15 = 16600896293664512447_u64 ^ 10687294420174753939_u64;
_21 = !56893732997737732680207613820395593657_u128;
_22 = _23;
_8 = _12;
_13 = _7;
_14 = _12;
_29.0 = _29.3;
_29.2 = 3184_i16 as i128;
_18 = _29.3 * _29.0;
_25 = _24;
_3 = _7;
_12 = _7;
_24 = -_25;
_29.1 = _19 as f64;
_21 = 262497278297779093049261860624976588613_u128 >> _27;
_29.3 = _18;
_3 = _10;
_1 = _8;
_2 = _14;
_19 = _15 as i64;
_28 = _15 >> _24;
Goto(bb5)
}
bb7 = {
_18 = 166752424597292521992860491167212632368_u128 as f32;
RET = [_15];
_19 = _18 as i64;
_23 = _22;
_11 = _10;
_21 = 203916920150505945369860726589391235134_u128 >> _17;
_7 = _4;
_15 = 11624999111055033786_u64 * 15261282362108024417_u64;
RET = [_15];
RET = [_15];
_22 = _23;
RET = [_15];
_20 = !_16;
_27 = (-119_isize);
_24 = _27 - _27;
_17 = !47651_u16;
_15 = 1646317959_u32 as u64;
RET = [_15];
_10 = _14;
_5 = _7;
_20 = _16;
_6 = _8;
_2 = _9;
_21 = 86476399978244798624146152538378773223_u128;
_20 = !_16;
_3 = _2;
_17 = !22364_u16;
_21 = 133739151776159269342603329574894003898_u128 ^ 187330011673727786173361217950297813400_u128;
Goto(bb4)
}
bb8 = {
_10 = _1;
Goto(bb3)
}
bb9 = {
_15 = 488124188311388750_u64;
_12 = _2;
_16 = false;
_18 = (-56_i8) as f32;
_17 = !50523_u16;
_13 = _3;
_17 = 41814_u16 & 12846_u16;
_3 = _5;
_17 = (-1460779578_i32) as u16;
_11 = _14;
_15 = 5416001810933891074_u64 * 18018000811117815594_u64;
_17 = !10437_u16;
_3 = _2;
_19 = (-913710438104157701_i64) - 4564294722194685033_i64;
_18 = 9223372036854775807_isize as f32;
_8 = _3;
_6 = _11;
_22 = '\u{10021d}';
_15 = !14781631006983384663_u64;
_1 = _8;
RET = [_15];
_23 = _22;
_10 = _14;
_6 = _7;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_32.0 = !_20;
_32.0 = _16 > _20;
_9 = _12;
_32.4 = RET;
_31 = [(-745795268_i32),978428463_i32,(-2130858084_i32),(-211651253_i32),353102543_i32,1194256_i32,(-1431045445_i32),959815125_i32];
_1 = _5;
_21 = !153424791795137758082505710393037300508_u128;
_32.1 = _29.1 - _29.1;
_36 = core::ptr::addr_of_mut!(_29.2);
_3 = _7;
_16 = _32.0;
_25 = _24 << _15;
_10 = core::ptr::addr_of_mut!(_32);
_6 = core::ptr::addr_of_mut!((*_10));
_22 = _23;
(*_10).2 = (-128_i8);
_32 = (_16, _29.1, (-40_i8), _28, RET);
_27 = _25 * _25;
_12 = _3;
_16 = _20;
_35.fld0 = 763686806_u32 | 878328455_u32;
(*_36) = (-134012788_i32) as i128;
match _32.2 {
0 => bb12,
340282366920938463463374607431768211416 => bb14,
_ => bb13
}
}
bb12 = {
_18 = 166752424597292521992860491167212632368_u128 as f32;
RET = [_15];
_19 = _18 as i64;
_23 = _22;
_11 = _10;
_21 = 203916920150505945369860726589391235134_u128 >> _17;
_7 = _4;
_15 = 11624999111055033786_u64 * 15261282362108024417_u64;
RET = [_15];
RET = [_15];
_22 = _23;
RET = [_15];
_20 = !_16;
_27 = (-119_isize);
_24 = _27 - _27;
_17 = !47651_u16;
_15 = 1646317959_u32 as u64;
RET = [_15];
_10 = _14;
_5 = _7;
_20 = _16;
_6 = _8;
_2 = _9;
_21 = 86476399978244798624146152538378773223_u128;
_20 = !_16;
_3 = _2;
_17 = !22364_u16;
_21 = 133739151776159269342603329574894003898_u128 ^ 187330011673727786173361217950297813400_u128;
Goto(bb4)
}
bb13 = {
_10 = _1;
Goto(bb3)
}
bb14 = {
_36 = core::ptr::addr_of_mut!(_29.2);
_33 = _35.fld0 as f32;
_41.1 = _32.0;
(*_6).4 = RET;
_41 = (_17, _20);
_37 = [_27,_27,_24,_24,_25];
(*_36) = !(-137797457193096768748584387612304032865_i128);
_5 = core::ptr::addr_of_mut!((*_10));
(*_10).3 = _28;
_32 = (_41.1, _29.1, (-11_i8), _28, RET);
_40 = [(-887108391_i32),78119871_i32];
_39 = [(*_10).2,(*_10).2,(*_10).2,_32.2,(*_5).2];
_42 = _35.fld0 ^ _35.fld0;
(*_10).2 = -(-2_i8);
(*_10).3 = _28 * _28;
(*_10).0 = !_20;
_2 = _8;
(*_10).4 = [(*_5).3];
(*_5).3 = _28;
(*_5).2 = _22 as i8;
_43 = (*_6).0 as i128;
_32 = (_41.1, _29.1, (-6_i8), _28, RET);
(*_10) = (_20, _29.1, 21_i8, _15, RET);
(*_6).0 = !_41.1;
(*_10).4 = RET;
(*_6).1 = _29.1;
_21 = (*_6).1 as u128;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(16_usize, 15_usize, Move(_15), 37_usize, Move(_37), 27_usize, Move(_27), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(16_usize, 42_usize, Move(_42), 22_usize, Move(_22), 39_usize, Move(_39), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(16_usize, 28_usize, Move(_28), 48_usize, _48, 48_usize, _48, 48_usize, _48), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: i8,mut _2: [u64; 1],mut _3: bool,mut _4: bool,mut _5: [u64; 1],mut _6: [i32; 6],mut _7: bool,mut _8: bool,mut _9: bool,mut _10: [u64; 1]) -> f64 {
mir! {
type RET = f64;
let _11: [i8; 5];
let _12: Adt42;
let _13: char;
let _14: [i32; 6];
let _15: char;
let _16: *mut [i32; 6];
let _17: (f32, f64, i128, f32);
let _18: [i32; 8];
let _19: Adt51;
let _20: [i32; 8];
let _21: Adt47;
let _22: *mut i128;
let _23: i16;
let _24: [i32; 2];
let _25: f32;
let _26: (usize,);
let _27: (bool, f64, i8, u64, [u64; 1]);
let _28: i128;
let _29: [i8; 5];
let _30: bool;
let _31: i16;
let _32: *mut i128;
let _33: Adt54;
let _34: (bool,);
let _35: i128;
let _36: ();
let _37: ();
{
_10 = [2317426966585408058_u64];
_8 = !_4;
_10 = [4324467335849352060_u64];
_2 = [15006179962892459027_u64];
RET = 266234886477445309841168749161444348279_u128 as f64;
_1 = (-7_i8) >> 30_i8;
_11 = [_1,_1,_1,_1,_1];
_4 = !_3;
_13 = '\u{a0337}';
RET = _1 as f64;
_8 = _4 > _3;
RET = _1 as f64;
_13 = '\u{78422}';
_5 = _2;
_3 = _7;
Goto(bb1)
}
bb1 = {
_10 = [17898168445636068645_u64];
_3 = _7;
RET = 13947785495329338656_u64 as f64;
_2 = [9714084478887449605_u64];
_1 = (-4357_i16) as i8;
RET = 23_u8 as f64;
_14 = [1695305597_i32,(-137794702_i32),(-621861785_i32),(-174026064_i32),671921467_i32,(-836343351_i32)];
_8 = _4;
_2 = [8834869337907882668_u64];
Goto(bb2)
}
bb2 = {
_6 = [(-1425951428_i32),(-1387185374_i32),541388075_i32,(-338896217_i32),(-1521440103_i32),(-1081650524_i32)];
_7 = _8;
_9 = _4;
_7 = _9 ^ _4;
_13 = '\u{ee01c}';
_16 = core::ptr::addr_of_mut!(_14);
_8 = !_3;
_1 = 31_i8;
_13 = '\u{cd9c9}';
_14 = [(-1523108997_i32),(-1787360009_i32),1718446867_i32,20696249_i32,1700321679_i32,(-130168490_i32)];
_7 = !_3;
_9 = _8 >= _3;
_4 = _8 <= _3;
_7 = !_9;
_9 = _3 == _3;
_11 = [_1,_1,_1,_1,_1];
_3 = _4;
(*_16) = [(-1965251675_i32),(-742794197_i32),(-1305354781_i32),(-616526416_i32),(-1797829656_i32),1453291138_i32];
match _1 {
0 => bb1,
1 => bb3,
2 => bb4,
31 => bb6,
_ => bb5
}
}
bb3 = {
_10 = [17898168445636068645_u64];
_3 = _7;
RET = 13947785495329338656_u64 as f64;
_2 = [9714084478887449605_u64];
_1 = (-4357_i16) as i8;
RET = 23_u8 as f64;
_14 = [1695305597_i32,(-137794702_i32),(-621861785_i32),(-174026064_i32),671921467_i32,(-836343351_i32)];
_8 = _4;
_2 = [8834869337907882668_u64];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_15 = _13;
_17.2 = 139591116410139961715034616118657171855_i128;
_17.3 = _1 as f32;
_17.2 = !(-162699703359809681781134239848296954654_i128);
_2 = _10;
_14 = [1924262412_i32,908530265_i32,222140136_i32,(-1486169756_i32),(-533559894_i32),1433006385_i32];
_5 = [7016026008770593593_u64];
_17.1 = 1_usize as f64;
_6 = [(-774658859_i32),(-1713786048_i32),1345460728_i32,(-21523792_i32),1332393931_i32,(-467921764_i32)];
_17.3 = _1 as f32;
_17.1 = RET * RET;
_1 = (-90_i8);
(*_16) = [(-896691559_i32),159059845_i32,(-499554124_i32),1793484222_i32,(-222148288_i32),1694825838_i32];
_15 = _13;
_8 = !_9;
_14 = _6;
RET = _17.1;
_10 = [16811902437206976224_u64];
_4 = _3;
RET = _17.1 - _17.1;
(*_16) = [1781647822_i32,(-1722901050_i32),(-1196202280_i32),(-1944347090_i32),(-1513645414_i32),(-843801932_i32)];
(*_16) = _6;
_2 = _10;
Call(_10 = fn18(_9, _9, _4, _7, _4, _9, _7, _8, _9, _7, _9, _9, _16, _14, _7, _17.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_17.1 = 3_u8 as f64;
_17.0 = -_17.3;
_16 = core::ptr::addr_of_mut!((*_16));
_5 = _10;
_11 = [_1,_1,_1,_1,_1];
_11 = [_1,_1,_1,_1,_1];
_14 = [2089246101_i32,102488366_i32,1586305148_i32,(-855257473_i32),95127114_i32,1124378815_i32];
_20 = [1464305788_i32,921589253_i32,1576312730_i32,1891042393_i32,62450844_i32,1911931542_i32,637472560_i32,(-2123324049_i32)];
RET = 9223372036854775807_isize as f64;
RET = 58538_u16 as f64;
RET = 65155_u16 as f64;
_17.2 = 67962075673544964523415188523024538359_i128 | (-94800053094321661545092756259746509204_i128);
_4 = !_7;
_7 = _8;
_13 = _15;
_17.2 = (-65818386213792846814833846815438485189_i128);
_17.1 = (-9223372036854775808_isize) as f64;
_5 = [10173866347756195094_u64];
_18 = _20;
_18 = [(-1636211994_i32),(-762200740_i32),(-824106849_i32),(-1748847191_i32),201950309_i32,(-174957026_i32),(-1474121438_i32),531296342_i32];
RET = -_17.1;
_17.2 = 44623942262461161771593507280060893919_i128;
_9 = _3;
_20 = [(-1427048939_i32),254048387_i32,(-776177229_i32),(-1042971716_i32),(-2069778106_i32),596018813_i32,(-60380690_i32),1341234653_i32];
Goto(bb8)
}
bb8 = {
_9 = !_7;
_8 = !_3;
_6 = _14;
_2 = [18351343671103261224_u64];
RET = _17.1 * _17.1;
_7 = _8 == _3;
_7 = _4;
_15 = _13;
_18 = [(-172604561_i32),(-796452036_i32),(-1058104726_i32),40519908_i32,(-1974302659_i32),(-1446863778_i32),(-1897921613_i32),1183537056_i32];
_18 = [(-1613750745_i32),638490074_i32,1797228485_i32,80404866_i32,1658707851_i32,988274756_i32,(-1210419239_i32),(-877280842_i32)];
_22 = core::ptr::addr_of_mut!(_17.2);
Goto(bb9)
}
bb9 = {
_17.3 = _17.0 + _17.0;
_4 = _9;
_22 = core::ptr::addr_of_mut!(_17.2);
_23 = (-6592_i16);
_10 = _5;
_16 = core::ptr::addr_of_mut!((*_16));
_10 = [11201889765657754381_u64];
(*_22) = (-75373828736753054824788567464561881033_i128) >> _1;
_3 = _8;
_18 = [(-203674758_i32),(-1723694362_i32),1845874166_i32,(-29880273_i32),(-198890163_i32),(-196445487_i32),35259685_i32,1728981973_i32];
(*_22) = -(-125014758383724954784563869646840306759_i128);
_26.0 = !6_usize;
_27.4 = _2;
_24 = [(-916445751_i32),(-1463443963_i32)];
_27.1 = RET;
_28 = (*_22);
_28 = (*_22) >> _17.2;
_14 = [(-884642540_i32),(-2062634236_i32),1188624211_i32,572443158_i32,1497711352_i32,(-1897420241_i32)];
_27.2 = _1;
_8 = !_3;
_27.0 = !_7;
Goto(bb10)
}
bb10 = {
(*_22) = _28 & _28;
_4 = !_3;
RET = _26.0 as f64;
_26.0 = 0_usize * 3_usize;
_34.0 = _7;
_33.fld0 = [2321488219_u32,1076395524_u32];
_26.0 = 10664695919883559546_usize;
_34.0 = !_4;
_30 = _9;
_4 = _7 & _3;
Goto(bb11)
}
bb11 = {
_4 = _3 | _8;
_14 = [(-884996913_i32),809322580_i32,1301961395_i32,(-1620052677_i32),1595424452_i32,848719672_i32];
_33.fld2.0 = [4229530665710429765_u64];
_17.0 = _17.3;
_29 = [_1,_27.2,_27.2,_27.2,_27.2];
_9 = !_30;
_15 = _13;
_27.1 = RET;
_22 = core::ptr::addr_of_mut!(_17.2);
_9 = _4;
_9 = !_7;
_34 = (_27.0,);
_24 = [(-941294067_i32),2109276735_i32];
_8 = _3 != _7;
_27.3 = !2433778922949243891_u64;
match _23 {
0 => bb12,
1 => bb13,
340282366920938463463374607431768204864 => bb15,
_ => bb14
}
}
bb12 = {
(*_22) = _28 & _28;
_4 = !_3;
RET = _26.0 as f64;
_26.0 = 0_usize * 3_usize;
_34.0 = _7;
_33.fld0 = [2321488219_u32,1076395524_u32];
_26.0 = 10664695919883559546_usize;
_34.0 = !_4;
_30 = _9;
_4 = _7 & _3;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3 = _8 == _8;
_32 = core::ptr::addr_of_mut!((*_22));
(*_16) = [(-370201852_i32),(-306325597_i32),(-1536759871_i32),1062876299_i32,1961118170_i32,1435845335_i32];
_5 = [_27.3];
_27.3 = 3738783506893347672_u64;
(*_32) = _28 - _28;
_17.0 = -_17.3;
(*_16) = _6;
_14 = [1943026304_i32,(-470263980_i32),(-2031699913_i32),959111017_i32,1659314994_i32,(-1963499618_i32)];
_26.0 = !7_usize;
_17.2 = _15 as i128;
_3 = !_27.0;
(*_32) = -_28;
_29 = [_27.2,_1,_1,_1,_27.2];
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(17_usize, 4_usize, Move(_4), 7_usize, Move(_7), 1_usize, Move(_1), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(17_usize, 3_usize, Move(_3), 34_usize, Move(_34), 14_usize, Move(_14), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(17_usize, 13_usize, Move(_13), 15_usize, Move(_15), 23_usize, Move(_23), 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: *mut [i32; 6],mut _14: [i32; 6],mut _15: bool,mut _16: f64) -> [u64; 1] {
mir! {
type RET = [u64; 1];
let _17: i32;
let _18: u64;
let _19: char;
let _20: (bool,);
let _21: *const *const (bool, f64, i8, u64, [u64; 1]);
let _22: usize;
let _23: i128;
let _24: *const *const (bool, f64, i8, u64, [u64; 1]);
let _25: [i32; 6];
let _26: Adt52;
let _27: isize;
let _28: isize;
let _29: [i32; 6];
let _30: (bool, f64, i8, u64, [u64; 1]);
let _31: *const isize;
let _32: (bool,);
let _33: [i32; 8];
let _34: bool;
let _35: f32;
let _36: Adt55;
let _37: ();
let _38: ();
{
_16 = 3458320194700504084_u64 as f64;
_4 = _8 >= _10;
_9 = _4;
RET = [5756997403816291099_u64];
_18 = 3008451693779858975_u64;
_11 = _6 > _15;
_2 = _8 != _5;
_8 = !_4;
_13 = core::ptr::addr_of_mut!((*_13));
RET = [_18];
_14 = [(-171806140_i32),(-911399926_i32),1131663785_i32,(-743559067_i32),(-99148010_i32),(-1824285261_i32)];
_5 = !_8;
_18 = 945642463_u32 as u64;
Call(_8 = fn19(_7, _2, _11, _12, _2, _5, _1, _3, _5, _15, _15), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17 = -522105831_i32;
_1 = _15 | _7;
_13 = core::ptr::addr_of_mut!(_14);
_13 = core::ptr::addr_of_mut!((*_13));
_10 = !_6;
_18 = 2547047619389127263_u64;
_8 = _2 & _7;
_18 = (-62_i8) as u64;
_20 = (_11,);
_11 = !_5;
RET = [_18];
_4 = !_11;
_10 = _11;
RET = [_18];
_3 = _4;
(*_13) = [_17,_17,_17,_17,_17,_17];
_4 = !_8;
_13 = core::ptr::addr_of_mut!((*_13));
_4 = !_20.0;
(*_13) = [_17,_17,_17,_17,_17,_17];
_16 = (-7_i8) as f64;
_20.0 = !_12;
_5 = !_3;
_20 = (_4,);
_12 = _9;
_19 = '\u{dd09a}';
Goto(bb2)
}
bb2 = {
_5 = _7;
_22 = _2 as usize;
_2 = _5 ^ _12;
_15 = !_20.0;
_10 = _9 < _12;
_23 = !145703086972225983599039071589871725046_i128;
(*_13) = [_17,_17,_17,_17,_17,_17];
_7 = _5;
Goto(bb3)
}
bb3 = {
_15 = !_11;
_23 = !(-109610953783481349336198357988519351313_i128);
_25 = [_17,_17,_17,_17,_17,_17];
_19 = '\u{c5904}';
_12 = _2 < _1;
_16 = _17 as f64;
_6 = _5;
_20.0 = !_3;
_3 = _11 | _8;
_27 = 87_isize ^ 9223372036854775807_isize;
Goto(bb4)
}
bb4 = {
_28 = _27;
_10 = _7 != _11;
_6 = _4 < _11;
_8 = _7;
_23 = (-60478918789077386542671433655810256479_i128);
_17 = !1743787717_i32;
_2 = _20.0;
_28 = _27 >> _22;
_16 = _28 as f64;
(*_13) = [_17,_17,_17,_17,_17,_17];
_22 = 10500201610124747902_usize;
(*_13) = [_17,_17,_17,_17,_17,_17];
_25 = [_17,_17,_17,_17,_17,_17];
(*_13) = [_17,_17,_17,_17,_17,_17];
_17 = 2053569926_i32;
_29 = [_17,_17,_17,_17,_17,_17];
_27 = _28 << _28;
_15 = _9;
_6 = !_11;
_13 = core::ptr::addr_of_mut!(_29);
_15 = !_2;
(*_13) = [_17,_17,_17,_17,_17,_17];
_2 = !_3;
_15 = _5 >= _10;
_10 = _4;
_13 = core::ptr::addr_of_mut!((*_13));
_3 = _6;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2053569926 => bb10,
_ => bb9
}
}
bb5 = {
_15 = !_11;
_23 = !(-109610953783481349336198357988519351313_i128);
_25 = [_17,_17,_17,_17,_17,_17];
_19 = '\u{c5904}';
_12 = _2 < _1;
_16 = _17 as f64;
_6 = _5;
_20.0 = !_3;
_3 = _11 | _8;
_27 = 87_isize ^ 9223372036854775807_isize;
Goto(bb4)
}
bb6 = {
_5 = _7;
_22 = _2 as usize;
_2 = _5 ^ _12;
_15 = !_20.0;
_10 = _9 < _12;
_23 = !145703086972225983599039071589871725046_i128;
(*_13) = [_17,_17,_17,_17,_17,_17];
_7 = _5;
Goto(bb3)
}
bb7 = {
_17 = -522105831_i32;
_1 = _15 | _7;
_13 = core::ptr::addr_of_mut!(_14);
_13 = core::ptr::addr_of_mut!((*_13));
_10 = !_6;
_18 = 2547047619389127263_u64;
_8 = _2 & _7;
_18 = (-62_i8) as u64;
_20 = (_11,);
_11 = !_5;
RET = [_18];
_4 = !_11;
_10 = _11;
RET = [_18];
_3 = _4;
(*_13) = [_17,_17,_17,_17,_17,_17];
_4 = !_8;
_13 = core::ptr::addr_of_mut!((*_13));
_4 = !_20.0;
(*_13) = [_17,_17,_17,_17,_17,_17];
_16 = (-7_i8) as f64;
_20.0 = !_12;
_5 = !_3;
_20 = (_4,);
_12 = _9;
_19 = '\u{dd09a}';
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_8 = _10;
_3 = _1;
_30.0 = !_7;
_14 = _25;
_31 = core::ptr::addr_of!(_28);
_27 = _1 as isize;
_22 = 15237723365306155409_usize >> _28;
_30.4 = [_18];
_18 = 16585593161615615756_u64;
(*_31) = _27;
RET = [_18];
_29 = _25;
_6 = _7;
_30.2 = (-33_i8);
_33 = [_17,_17,_17,_17,_17,_17,_17,_17];
_3 = _5;
_27 = -(*_31);
_8 = _22 != _22;
_30.3 = _19 as u64;
_30 = (_8, _16, 28_i8, _18, RET);
_30.0 = _11 | _1;
Goto(bb11)
}
bb11 = {
_23 = -(-131909322019110157555858205631905352670_i128);
(*_31) = _27;
Call(_16 = core::intrinsics::transmute((*_31)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_28 = _27 + _27;
_27 = (*_31);
_11 = !_1;
_32 = (_9,);
match _30.2 {
0 => bb10,
1 => bb7,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb13,
28 => bb15,
_ => bb14
}
}
bb13 = {
_15 = !_11;
_23 = !(-109610953783481349336198357988519351313_i128);
_25 = [_17,_17,_17,_17,_17,_17];
_19 = '\u{c5904}';
_12 = _2 < _1;
_16 = _17 as f64;
_6 = _5;
_20.0 = !_3;
_3 = _11 | _8;
_27 = 87_isize ^ 9223372036854775807_isize;
Goto(bb4)
}
bb14 = {
_8 = _10;
_3 = _1;
_30.0 = !_7;
_14 = _25;
_31 = core::ptr::addr_of!(_28);
_27 = _1 as isize;
_22 = 15237723365306155409_usize >> _28;
_30.4 = [_18];
_18 = 16585593161615615756_u64;
(*_31) = _27;
RET = [_18];
_29 = _25;
_6 = _7;
_30.2 = (-33_i8);
_33 = [_17,_17,_17,_17,_17,_17,_17,_17];
_3 = _5;
_27 = -(*_31);
_8 = _22 != _22;
_30.3 = _19 as u64;
_30 = (_8, _16, 28_i8, _18, RET);
_30.0 = _11 | _1;
Goto(bb11)
}
bb15 = {
_14 = [_17,_17,_17,_17,_17,_17];
_5 = !_1;
_32.0 = !_11;
_30.2 = 90_i8;
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(18_usize, 23_usize, Move(_23), 6_usize, Move(_6), 29_usize, Move(_29), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(18_usize, 18_usize, Move(_18), 4_usize, Move(_4), 12_usize, Move(_12), 28_usize, Move(_28)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(18_usize, 20_usize, Move(_20), 11_usize, Move(_11), 33_usize, Move(_33), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(18_usize, 7_usize, Move(_7), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool) -> bool {
mir! {
type RET = bool;
let _12: *const *const (bool, f64, i8, u64, [u64; 1]);
let _13: ();
let _14: ();
{
RET = _4;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(19_usize, 7_usize, Move(_7), 3_usize, Move(_3), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(19_usize, 1_usize, Move(_1), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(2999116463602278121_u64), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box((-60_i8)), std::hint::black_box(79_u8), std::hint::black_box(320747297770949756162794790944155165300_u128));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: *mut (bool, f64, i8, u64, [u64; 1]),
fld1: *mut [i32; 6],

},
Variant1{
fld0: *const (u8,),
fld1: char,
fld2: usize,
fld3: [i32; 2],

},
Variant2{
fld0: *const (u8,),
fld1: [i8; 5],
fld2: *const (bool, f64, i8, u64, [u64; 1]),
fld3: (bool, f64, i8, u64, [u64; 1]),

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: f32,

},
Variant1{
fld0: *mut [i32; 6],
fld1: (u16, bool),
fld2: ([u64; 1],),
fld3: i8,
fld4: [u64; 1],
fld5: (usize,),
fld6: i64,
fld7: [i32; 8],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: Adt40,
fld1: (f32, f64, i128, f32),
fld2: ([u64; 1],),
fld3: (bool,),
fld4: *const (u8,),
fld5: [isize; 5],

},
Variant1{
fld0: u16,
fld1: ([u64; 1],),
fld2: (u16, bool),
fld3: (bool,),
fld4: i16,

},
Variant2{
fld0: u64,
fld1: i64,
fld2: [isize; 5],
fld3: f32,
fld4: u128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u32; 2],
fld1: (f32, f64, i128, f32),
fld2: i32,
fld3: Adt40,
fld4: usize,

},
Variant1{
fld0: [i32; 2],
fld1: u16,

},
Variant2{
fld0: [u64; 1],
fld1: i128,

},
Variant3{
fld0: *mut [i32; 6],
fld1: (u16, bool),
fld2: u16,
fld3: (bool,),
fld4: Adt40,
fld5: i32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u64,

},
Variant1{
fld0: Adt41,
fld1: *const (u8,),
fld2: (usize,),
fld3: *mut i128,
fld4: [u32; 2],
fld5: u128,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i64,
fld1: Adt43,
fld2: (u8,),
fld3: u16,

},
Variant1{
fld0: bool,
fld1: Adt43,
fld2: u8,
fld3: u128,

},
Variant2{
fld0: [u64; 1],
fld1: [i8; 5],

},
Variant3{
fld0: i128,
fld1: (u8,),
fld2: u32,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: char,

},
Variant1{
fld0: i128,
fld1: Adt40,
fld2: u8,
fld3: *mut i128,
fld4: Adt43,
fld5: Adt42,

},
Variant2{
fld0: u128,
fld1: (u16, bool),
fld2: [i8; 5],
fld3: u8,
fld4: (bool,),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const (bool, f64, i8, u64, [u64; 1]),
fld1: *mut [i32; 6],
fld2: *mut (bool, f64, i8, u64, [u64; 1]),
fld3: i8,
fld4: ([u64; 1],),

},
Variant1{
fld0: [i8; 5],
fld1: (bool, f64, i8, u64, [u64; 1]),

},
Variant2{
fld0: *const (u8,),
fld1: u32,
fld2: (bool,),
fld3: *mut i128,
fld4: *mut (bool, f64, i8, u64, [u64; 1]),
fld5: i32,
fld6: (bool, f64, i8, u64, [u64; 1]),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i8; 5],
fld1: (bool,),
fld2: *mut i128,
fld3: Adt43,
fld4: i16,
fld5: u8,
fld6: (bool, f64, i8, u64, [u64; 1]),

},
Variant1{
fld0: bool,
fld1: [i32; 8],
fld2: ([u64; 1],),
fld3: i8,
fld4: [u64; 1],
fld5: *mut i128,
fld6: *mut (bool, f64, i8, u64, [u64; 1]),

},
Variant2{
fld0: u64,
fld1: *const (u8,),
fld2: (u16, bool),
fld3: (u8,),
fld4: ([u64; 1],),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: u32,
fld1: Adt45,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: *mut [i32; 6],
fld1: u16,
fld2: Adt42,
fld3: [char; 1],
fld4: f64,
fld5: Adt46,
fld6: u8,
fld7: Adt41,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *const (bool, f64, i8, u64, [u64; 1]),

},
Variant1{
fld0: bool,
fld1: ([u64; 1],),
fld2: u16,
fld3: [i32; 2],
fld4: u128,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: *const (u8,),
fld1: Adt47,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [isize; 5],
fld1: i128,
fld2: isize,
fld3: i8,
fld4: Adt42,

},
Variant1{
fld0: u16,
fld1: u32,
fld2: u64,
fld3: *const (u8,),
fld4: ([u64; 1],),

},
Variant2{
fld0: Adt41,
fld1: (f32, f64, i128, f32),
fld2: *const (bool, f64, i8, u64, [u64; 1]),
fld3: [i32; 2],
fld4: Adt49,

},
Variant3{
fld0: Adt51,
fld1: [char; 1],
fld2: f64,
fld3: Adt50,
fld4: Adt52,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: [u32; 2],
fld1: Adt53,
fld2: ([u64; 1],),
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt46,
fld1: char,
fld2: (bool,),
fld3: i32,
fld4: u16,

},
Variant1{
fld0: (usize,),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (usize,),

},
Variant1{
fld0: [i32; 2],
fld1: char,
fld2: *const isize,
fld3: Adt40,
fld4: [i32; 8],
fld5: f64,
fld6: [char; 1],

},
Variant2{
fld0: Adt46,
fld1: char,
fld2: [i32; 8],
fld3: Adt42,

}}

