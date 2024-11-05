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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: u16,mut _4: i8,mut _5: i16,mut _6: u32,mut _7: u128,mut _8: i128,mut _9: usize) -> [char; 2] {
mir! {
type RET = [char; 2];
let _10: i8;
let _11: f64;
let _12: [u32; 1];
let _13: Adt55;
let _14: *mut *mut i64;
let _15: (i8, i64);
let _16: bool;
let _17: [u8; 6];
let _18: Adt56;
let _19: bool;
let _20: char;
let _21: (i8, i64);
let _22: f64;
let _23: [i128; 8];
let _24: i32;
let _25: isize;
let _26: isize;
let _27: [char; 2];
let _28: ();
let _29: ();
{
_6 = 813364088_u32 | 386798960_u32;
RET = ['\u{6149e}','\u{58dd8}'];
_4 = !(-34_i8);
_7 = 321648627932006948561617932201549943009_u128;
_6 = !1186924463_u32;
_6 = 3194669783_u32;
_10 = -_4;
Goto(bb1)
}
bb1 = {
_11 = 9223372036854775807_isize as f64;
_2 = 241_u8 << _4;
_3 = 36199_u16 & 19883_u16;
_9 = 3729_i16 as usize;
_4 = _10;
_3 = 9885_u16;
_2 = 17_u8;
_7 = 114927277224431622452427491532234193393_u128;
_12 = [_6];
_9 = (-28985_i16) as usize;
_11 = 824163753_i32 as f64;
_6 = 1984276756_u32 - 565380279_u32;
RET = ['\u{12a05}','\u{efba4}'];
Call(_8 = fn1(_2, _7, _4, _7, _12, _2, _2, RET, RET, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _10 >> _8;
_5 = -13894_i16;
_1 = true;
Call(_3 = fn11(_10, _4, _8, _7, RET, _12, _12, _7, _9, _7, _5, _2, _6, _2, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = !3869122027_u32;
_2 = _11 as u8;
_7 = !332715348386119855119877965221793696341_u128;
_12 = [_6];
_15.1 = '\u{a841a}' as i64;
_8 = -57708064284270647768751569816688345556_i128;
_15 = (_4, 4360369335707878948_i64);
_4 = _15.0;
_16 = _5 >= _5;
_18.fld3 = _4 << _7;
_17 = [_2,_2,_2,_2,_2,_2];
_11 = _15.1 as f64;
match _15.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4360369335707878948 => bb7,
_ => bb6
}
}
bb4 = {
_4 = _10 >> _8;
_5 = -13894_i16;
_1 = true;
Call(_3 = fn11(_10, _4, _8, _7, RET, _12, _12, _7, _9, _7, _5, _2, _6, _2, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_11 = 9223372036854775807_isize as f64;
_2 = 241_u8 << _4;
_3 = 36199_u16 & 19883_u16;
_9 = 3729_i16 as usize;
_4 = _10;
_3 = 9885_u16;
_2 = 17_u8;
_7 = 114927277224431622452427491532234193393_u128;
_12 = [_6];
_9 = (-28985_i16) as usize;
_11 = 824163753_i32 as f64;
_6 = 1984276756_u32 - 565380279_u32;
RET = ['\u{12a05}','\u{efba4}'];
Call(_8 = fn1(_2, _7, _4, _7, _12, _2, _2, RET, RET, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_18.fld5 = _6;
_12 = [_18.fld5];
_2 = !255_u8;
_12 = [_6];
_15 = (_18.fld3, (-3178344155833861417_i64));
_15 = (_18.fld3, (-2934286459331295494_i64));
_18.fld2 = (-9223372036854775808_isize);
_18.fld0 = core::ptr::addr_of_mut!(_18.fld5);
_7 = !256681142455403107097319151969601573625_u128;
_5 = (-18767_i16);
_5 = (-28521_i16) - (-25394_i16);
_10 = _18.fld3 >> _4;
_19 = _1 == _1;
_5 = _7 as i16;
_18.fld3 = _15.0 >> _7;
_21 = _15;
_21.0 = -_10;
_18.fld1 = '\u{b40b8}';
_18.fld5 = _6 | _6;
_16 = _19;
_21.1 = _15.0 as i64;
_15.0 = _10;
_4 = !_15.0;
_18.fld3 = _21.0 ^ _4;
_15 = _21;
_18.fld3 = _15.0 + _4;
match _3 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
40011 => bb14,
_ => bb13
}
}
bb8 = {
Return()
}
bb9 = {
_11 = 9223372036854775807_isize as f64;
_2 = 241_u8 << _4;
_3 = 36199_u16 & 19883_u16;
_9 = 3729_i16 as usize;
_4 = _10;
_3 = 9885_u16;
_2 = 17_u8;
_7 = 114927277224431622452427491532234193393_u128;
_12 = [_6];
_9 = (-28985_i16) as usize;
_11 = 824163753_i32 as f64;
_6 = 1984276756_u32 - 565380279_u32;
RET = ['\u{12a05}','\u{efba4}'];
Call(_8 = fn1(_2, _7, _4, _7, _12, _2, _2, RET, RET, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_4 = _10 >> _8;
_5 = -13894_i16;
_1 = true;
Call(_3 = fn11(_10, _4, _8, _7, RET, _12, _12, _7, _9, _7, _5, _2, _6, _2, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_6 = !3869122027_u32;
_2 = _11 as u8;
_7 = !332715348386119855119877965221793696341_u128;
_12 = [_6];
_15.1 = '\u{a841a}' as i64;
_8 = -57708064284270647768751569816688345556_i128;
_15 = (_4, 4360369335707878948_i64);
_4 = _15.0;
_16 = _5 >= _5;
_18.fld3 = _4 << _7;
_17 = [_2,_2,_2,_2,_2,_2];
_11 = _15.1 as f64;
match _15.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4360369335707878948 => bb7,
_ => bb6
}
}
bb12 = {
_4 = _10 >> _8;
_5 = -13894_i16;
_1 = true;
Call(_3 = fn11(_10, _4, _8, _7, RET, _12, _12, _7, _9, _7, _5, _2, _6, _2, _1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_11 = 9223372036854775807_isize as f64;
_2 = 241_u8 << _4;
_3 = 36199_u16 & 19883_u16;
_9 = 3729_i16 as usize;
_4 = _10;
_3 = 9885_u16;
_2 = 17_u8;
_7 = 114927277224431622452427491532234193393_u128;
_12 = [_6];
_9 = (-28985_i16) as usize;
_11 = 824163753_i32 as f64;
_6 = 1984276756_u32 - 565380279_u32;
RET = ['\u{12a05}','\u{efba4}'];
Call(_8 = fn1(_2, _7, _4, _7, _12, _2, _2, RET, RET, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_25 = _18.fld2 + _18.fld2;
_21 = (_4, _15.1);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(0_usize, 2_usize, Move(_2), 17_usize, Move(_17), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(0_usize, 25_usize, Move(_25), 8_usize, Move(_8), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u8,mut _2: u128,mut _3: i8,mut _4: u128,mut _5: [u32; 1],mut _6: u8,mut _7: u8,mut _8: [char; 2],mut _9: [char; 2],mut _10: u8,mut _11: u32) -> i128 {
mir! {
type RET = i128;
let _12: usize;
let _13: i8;
let _14: f64;
let _15: u64;
let _16: u32;
let _17: (u64, u8);
let _18: f32;
let _19: Adt49;
let _20: bool;
let _21: isize;
let _22: f32;
let _23: [i128; 8];
let _24: *mut u32;
let _25: f64;
let _26: (i8, i64);
let _27: (&'static f64,);
let _28: [char; 2];
let _29: usize;
let _30: [u32; 1];
let _31: u128;
let _32: [char; 1];
let _33: *mut u16;
let _34: usize;
let _35: [char; 1];
let _36: isize;
let _37: bool;
let _38: Adt56;
let _39: i32;
let _40: Adt42;
let _41: char;
let _42: Adt51;
let _43: char;
let _44: Adt50;
let _45: *mut u32;
let _46: i128;
let _47: [char; 1];
let _48: [i128; 8];
let _49: i8;
let _50: isize;
let _51: i64;
let _52: u16;
let _53: [char; 1];
let _54: [u8; 6];
let _55: f32;
let _56: ();
let _57: ();
{
_5 = [_11];
_6 = _2 as u8;
_2 = _4 | _4;
_9 = _8;
_8 = ['\u{4afdd}','\u{a4907}'];
RET = 70214445289794327001714830936765545831_i128;
RET = 136143344045130676597773581525332395313_i128 >> _6;
_9 = ['\u{d4c0c}','\u{6ea44}'];
_4 = !_2;
_1 = RET as u8;
_9 = _8;
RET = (-167264571414051826010988733394385789651_i128);
_5 = [_11];
_7 = !_1;
_10 = !_1;
Goto(bb1)
}
bb1 = {
_2 = !_4;
_13 = !_3;
_12 = _2 as usize;
_12 = !1_usize;
RET = 39998_u16 as i128;
RET = (-100863304161624829932335095983315741096_i128);
_14 = RET as f64;
_14 = 6746429773497481168_u64 as f64;
_10 = _6 | _7;
_6 = _11 as u8;
_12 = 7_usize;
_2 = !_4;
_12 = !12660159193696123430_usize;
_1 = 9223372036854775807_isize as u8;
_13 = _4 as i8;
RET = !116476694070483773267843602082599100491_i128;
RET = (-39443543806406933514734952958439604854_i128);
_15 = 11427471590109945279_u64 & 3689781885807544679_u64;
_1 = _7 - _6;
_13 = _15 as i8;
_17 = (_15, _10);
_8 = ['\u{56d29}','\u{1707b}'];
_16 = _11 << _4;
_8 = ['\u{ecc0e}','\u{1026dd}'];
RET = 9223372036854775807_isize as i128;
_7 = _17.1 >> _16;
Call(_10 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _8;
_17.1 = !_10;
_17.0 = _15;
_17.0 = 58485_u16 as u64;
RET = (-9223372036854775808_isize) as i128;
_17 = (_15, _7);
RET = 56451090969326898810554335939248877128_i128;
_18 = _2 as f32;
_12 = 2065573036_i32 as usize;
_5 = [_16];
_16 = !_11;
_3 = _13;
_9 = _8;
RET = 115398984845992460689226245906233320000_i128;
_17.1 = !_1;
_8 = ['\u{a19de}','\u{8b52d}'];
RET = 123931356197243539527113520099342531344_i128 << _4;
_18 = _17.0 as f32;
_2 = _4;
_20 = false;
_14 = 16752_i16 as f64;
_1 = _17.1 << _3;
RET = (-17545701206373361994325257983342487964_i128);
_17.0 = _15 & _15;
RET = !(-27021474383739762327597923449634314494_i128);
_6 = _17.1 - _10;
_17.1 = _10 >> _3;
Call(_17.0 = core::intrinsics::transmute(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = !_6;
_13 = _3;
_10 = (-5271488778124057653_i64) as u8;
_17.0 = !_15;
_15 = _17.0 - _17.0;
_3 = _13 - _13;
_21 = 9223372036854775807_isize * 122_isize;
_14 = _13 as f64;
_18 = _15 as f32;
_9 = ['\u{5af9c}','\u{6bcc3}'];
_10 = _7 & _17.1;
_10 = _6 ^ _1;
_6 = !_10;
_21 = 2_isize * 109_isize;
_20 = !false;
_6 = _10;
_2 = _6 as u128;
RET = _21 as i128;
_25 = _13 as f64;
_1 = _6 << _11;
_14 = -_25;
_23 = [RET,RET,RET,RET,RET,RET,RET,RET];
Call(_14 = core::intrinsics::fmaf64(_25, _25, _25), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = !_11;
_7 = !_1;
_1 = (-8527024145074382898_i64) as u8;
_24 = core::ptr::addr_of_mut!(_11);
_21 = 58_isize & (-9223372036854775808_isize);
(*_24) = _16;
_17 = (_15, _10);
_20 = true;
Goto(bb5)
}
bb5 = {
_18 = _3 as f32;
_21 = (-105_isize) - (-9223372036854775808_isize);
_14 = -_25;
_15 = _2 as u64;
_26.0 = _13 + _13;
_2 = _4 + _4;
_29 = !_12;
_26 = (_3, 6760558111058493420_i64);
_26.1 = -(-4429238726918490492_i64);
_16 = (*_24) + (*_24);
_30 = [_16];
_6 = _17.1;
_25 = -_14;
_24 = core::ptr::addr_of_mut!(_16);
_25 = _14 - _14;
_6 = _7 >> _3;
_26 = (_3, 8768645932723309061_i64);
Call(_15 = fn2(_10, _3, _17, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_22 = -_18;
_13 = _3 << _26.1;
_16 = _11;
_16 = _11 * _11;
_8 = _9;
_32 = ['\u{275bb}'];
_31 = !_2;
_2 = _4;
_11 = (*_24) - (*_24);
_26.1 = (-7353157609175134420_i64);
RET = -11986107009964773750967306123538381848_i128;
_14 = _25 * _25;
_6 = _20 as u8;
_27.0 = &_25;
_11 = (*_24);
_35 = _32;
_17 = (_15, _7);
_26.1 = 6133003950528230495_i64;
_16 = 22243_u16 as u32;
(*_24) = _11;
_7 = RET as u8;
_34 = 2043749674_i32 as usize;
(*_24) = _11 & _11;
_1 = !_10;
_26.0 = _13;
_28 = ['\u{8fa69}','\u{d357e}'];
Goto(bb7)
}
bb7 = {
_18 = _22;
_21 = -(-9223372036854775808_isize);
_31 = _2;
_38.fld5 = _22 as u32;
_29 = _12 - _12;
_38.fld0 = _24;
_30 = [(*_24)];
_26.1 = 6818658824657970815_i64 | 1952272024195558218_i64;
(*_24) = !_38.fld5;
_39 = _26.0 as i32;
_4 = _31;
_42.fld2 = _26.0 as u8;
_11 = !_16;
_7 = _17.1;
_38.fld5 = !_16;
_38.fld3 = _13 >> _7;
_41 = '\u{6f7d6}';
_11 = !(*_24);
_6 = _17.1 ^ _10;
_38.fld0 = _24;
Goto(bb8)
}
bb8 = {
_31 = _4;
_46 = RET ^ RET;
_47 = [_41];
_39 = _13 as i32;
_16 = _38.fld5;
_32 = [_41];
_20 = true;
_50 = _21;
Goto(bb9)
}
bb9 = {
_43 = _41;
_26.1 = (-3691360661964945772_i64);
_42.fld0 = core::ptr::addr_of!(_7);
Goto(bb10)
}
bb10 = {
_43 = _41;
_21 = -_50;
_21 = _50;
_8 = [_43,_41];
match _26.1 {
0 => bb4,
1 => bb7,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463459683246769803265684 => bb15,
_ => bb14
}
}
bb11 = {
_43 = _41;
_26.1 = (-3691360661964945772_i64);
_42.fld0 = core::ptr::addr_of!(_7);
Goto(bb10)
}
bb12 = {
_31 = _4;
_46 = RET ^ RET;
_47 = [_41];
_39 = _13 as i32;
_16 = _38.fld5;
_32 = [_41];
_20 = true;
_50 = _21;
Goto(bb9)
}
bb13 = {
_2 = !_4;
_13 = !_3;
_12 = _2 as usize;
_12 = !1_usize;
RET = 39998_u16 as i128;
RET = (-100863304161624829932335095983315741096_i128);
_14 = RET as f64;
_14 = 6746429773497481168_u64 as f64;
_10 = _6 | _7;
_6 = _11 as u8;
_12 = 7_usize;
_2 = !_4;
_12 = !12660159193696123430_usize;
_1 = 9223372036854775807_isize as u8;
_13 = _4 as i8;
RET = !116476694070483773267843602082599100491_i128;
RET = (-39443543806406933514734952958439604854_i128);
_15 = 11427471590109945279_u64 & 3689781885807544679_u64;
_1 = _7 - _6;
_13 = _15 as i8;
_17 = (_15, _10);
_8 = ['\u{56d29}','\u{1707b}'];
_16 = _11 << _4;
_8 = ['\u{ecc0e}','\u{1026dd}'];
RET = 9223372036854775807_isize as i128;
_7 = _17.1 >> _16;
Call(_10 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_18 = _3 as f32;
_21 = (-105_isize) - (-9223372036854775808_isize);
_14 = -_25;
_15 = _2 as u64;
_26.0 = _13 + _13;
_2 = _4 + _4;
_29 = !_12;
_26 = (_3, 6760558111058493420_i64);
_26.1 = -(-4429238726918490492_i64);
_16 = (*_24) + (*_24);
_30 = [_16];
_6 = _17.1;
_25 = -_14;
_24 = core::ptr::addr_of_mut!(_16);
_25 = _14 - _14;
_6 = _7 >> _3;
_26 = (_3, 8768645932723309061_i64);
Call(_15 = fn2(_10, _3, _17, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_45 = _38.fld0;
_24 = core::ptr::addr_of_mut!(_38.fld5);
_49 = _13 - _38.fld3;
_42.fld2 = !_6;
Goto(bb16)
}
bb16 = {
Call(_56 = dump_var(1_usize, 35_usize, Move(_35), 12_usize, Move(_12), 46_usize, Move(_46), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_56 = dump_var(1_usize, 8_usize, Move(_8), 32_usize, Move(_32), 1_usize, Move(_1), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(1_usize, 9_usize, Move(_9), 20_usize, Move(_20), 17_usize, Move(_17), 47_usize, Move(_47)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(1_usize, 50_usize, Move(_50), 3_usize, Move(_3), 28_usize, Move(_28), 41_usize, Move(_41)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_56 = dump_var(1_usize, 49_usize, Move(_49), 57_usize, _57, 57_usize, _57, 57_usize, _57), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u8,mut _2: i8,mut _3: (u64, u8),mut _4: [u32; 1]) -> u64 {
mir! {
type RET = u64;
let _5: Adt52;
let _6: f64;
let _7: u128;
let _8: isize;
let _9: bool;
let _10: f64;
let _11: bool;
let _12: Adt48;
let _13: [char; 2];
let _14: isize;
let _15: i64;
let _16: [u8; 6];
let _17: Adt42;
let _18: (isize, isize, i128);
let _19: [char; 2];
let _20: i8;
let _21: char;
let _22: i128;
let _23: Adt55;
let _24: isize;
let _25: Adt52;
let _26: [char; 2];
let _27: [char; 2];
let _28: (&'static f64,);
let _29: i64;
let _30: *const f64;
let _31: f32;
let _32: [char; 1];
let _33: f32;
let _34: f32;
let _35: f64;
let _36: [char; 2];
let _37: Adt43;
let _38: *mut i8;
let _39: [u32; 1];
let _40: [isize; 1];
let _41: ();
let _42: ();
{
_3.1 = (-841060742_i32) as u8;
RET = _3.0;
_3.0 = RET ^ RET;
_3.1 = _1;
_3.1 = _1;
_3.0 = RET * RET;
_2 = -81_i8;
RET = !_3.0;
_2 = !(-80_i8);
_1 = _3.1 & _3.1;
RET = !_3.0;
RET = _3.0;
RET = !_3.0;
RET = _3.0;
_3.0 = !RET;
_3.0 = 3_usize as u64;
_7 = 289181872481914081768230932464625538507_u128;
_3.0 = 13379_u16 as u64;
_6 = 3806991694_u32 as f64;
_6 = (-69_isize) as f64;
RET = _3.0 ^ _3.0;
_7 = 275965968216134670068699714193974720550_u128;
_4 = [3123440813_u32];
Call(_3 = fn3(_1, _1, _6, _1, _1, RET, _1, _1, _7, _1, _1, _7, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 5764580832462861689_i64 as u64;
_4 = [1872381560_u32];
_7 = 215984291808514768374256397120620136663_u128 + 294997428541075106229728628171983543160_u128;
RET = _3.0;
_2 = 66_i8;
_6 = 16557221211807245923_usize as f64;
_3.0 = RET;
RET = _1 as u64;
_4 = [2675721894_u32];
_2 = 2_usize as i8;
RET = _3.0;
RET = _3.0;
_3 = (RET, _1);
_2 = !(-66_i8);
_3.0 = 1388644734_u32 as u64;
_8 = -(-9223372036854775808_isize);
_2 = !117_i8;
_4 = [1611228061_u32];
_6 = 5862_u16 as f64;
_4 = [4084432505_u32];
_2 = (-102_i8) & (-110_i8);
_3.1 = _1 >> _1;
Call(_7 = fn4(_1, _3.1, _1, _3.1, _3, _3.1, _3.1, _1, _1, _3.1, _3, _3, _3.1, _3, _3.1, _3.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.1 = _1;
_10 = _7 as f64;
_2 = (-5155583737450977190_i64) as i8;
_11 = false;
_9 = _7 >= _7;
_3 = (RET, _1);
_4 = [838332024_u32];
_10 = _6 - _6;
_10 = -_6;
_6 = _10 * _10;
_4 = [1964282805_u32];
_3.0 = 4930325018882852633_usize as u64;
_3.0 = !RET;
_3.0 = RET;
_15 = -(-6973469349862610357_i64);
_6 = _10;
_13 = ['\u{90d0f}','\u{509bf}'];
_8 = 9223372036854775807_isize;
match _8 {
0 => bb3,
9223372036854775807 => bb5,
_ => bb4
}
}
bb3 = {
RET = 5764580832462861689_i64 as u64;
_4 = [1872381560_u32];
_7 = 215984291808514768374256397120620136663_u128 + 294997428541075106229728628171983543160_u128;
RET = _3.0;
_2 = 66_i8;
_6 = 16557221211807245923_usize as f64;
_3.0 = RET;
RET = _1 as u64;
_4 = [2675721894_u32];
_2 = 2_usize as i8;
RET = _3.0;
RET = _3.0;
_3 = (RET, _1);
_2 = !(-66_i8);
_3.0 = 1388644734_u32 as u64;
_8 = -(-9223372036854775808_isize);
_2 = !117_i8;
_4 = [1611228061_u32];
_6 = 5862_u16 as f64;
_4 = [4084432505_u32];
_2 = (-102_i8) & (-110_i8);
_3.1 = _1 >> _1;
Call(_7 = fn4(_1, _3.1, _1, _3.1, _3, _3.1, _3.1, _1, _1, _3.1, _3, _3, _3.1, _3, _3.1, _3.1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_3.1 = !_1;
_6 = -_10;
_3 = (RET, _1);
_14 = _8 ^ _8;
_13 = ['\u{ccfc1}','\u{5536b}'];
_3.0 = RET;
_9 = !_11;
RET = !_3.0;
_15 = -(-7735085522462772099_i64);
_16 = [_1,_3.1,_1,_3.1,_1,_1];
_2 = (-107_i8) | (-115_i8);
_9 = !_11;
_3 = (RET, _1);
Call(RET = core::intrinsics::bswap(_3.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = 38492010344093646819316534507254764250_i128 as f64;
_10 = _7 as f64;
Call(RET = fn7(_10, _10, _3.1, _10, _1, _10, _7, _7, _7, _3.1, _1, _10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_16 = [_1,_3.1,_1,_1,_1,_3.1];
_4 = [3076011011_u32];
_9 = !_11;
_7 = 259776547716167413020554196531224011080_u128;
RET = _3.0 - _3.0;
RET = _3.0;
_18.0 = _14;
Call(_8 = fn10(_1, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_19 = ['\u{6f6d5}','\u{edb85}'];
_4 = [117606827_u32];
_10 = -_6;
_14 = !_8;
_18.2 = 91089651547669835326108865735566262558_i128;
_15 = 1087711293137227566_i64 ^ 6339427553313742641_i64;
_8 = _3.1 as isize;
_18.1 = _14;
_13 = ['\u{f6a99}','\u{578d0}'];
_20 = _2;
_14 = -_18.1;
_27 = ['\u{83327}','\u{c1565}'];
_26 = ['\u{5a510}','\u{4c960}'];
_22 = _14 as i128;
_18 = (_14, _8, _22);
_9 = _3.1 >= _1;
_13 = ['\u{d3ac0}','\u{34ef2}'];
_24 = -_14;
_27 = ['\u{5cf55}','\u{189d4}'];
_3.0 = 47567_u16 as u64;
_19 = _27;
_3.1 = _1 + _1;
_20 = _9 as i8;
match _7 {
0 => bb3,
1 => bb5,
259776547716167413020554196531224011080 => bb10,
_ => bb9
}
}
bb9 = {
RET = 5764580832462861689_i64 as u64;
_4 = [1872381560_u32];
_7 = 215984291808514768374256397120620136663_u128 + 294997428541075106229728628171983543160_u128;
RET = _3.0;
_2 = 66_i8;
_6 = 16557221211807245923_usize as f64;
_3.0 = RET;
RET = _1 as u64;
_4 = [2675721894_u32];
_2 = 2_usize as i8;
RET = _3.0;
RET = _3.0;
_3 = (RET, _1);
_2 = !(-66_i8);
_3.0 = 1388644734_u32 as u64;
_8 = -(-9223372036854775808_isize);
_2 = !117_i8;
_4 = [1611228061_u32];
_6 = 5862_u16 as f64;
_4 = [4084432505_u32];
_2 = (-102_i8) & (-110_i8);
_3.1 = _1 >> _1;
Call(_7 = fn4(_1, _3.1, _1, _3.1, _3, _3.1, _3.1, _1, _1, _3.1, _3, _3, _3.1, _3, _3.1, _3.1), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
RET = _15 as u64;
_10 = _6 * _6;
_2 = !_20;
_3 = (RET, _1);
_30 = core::ptr::addr_of!(_6);
_16 = [_1,_1,_1,_1,_3.1,_3.1];
(*_30) = _10;
_18 = (_14, _24, _22);
_28.0 = &(*_30);
_2 = _20 - _20;
_2 = _20 >> _24;
_18.2 = _22 >> _18.0;
_3.1 = _1;
_29 = _15;
_29 = !_15;
_6 = -_10;
_18.2 = (*_30) as i128;
_1 = _3.1;
_6 = _10 - _10;
_31 = (-27566064_i32) as f32;
_19 = ['\u{d5c1}','\u{db431}'];
_9 = _11;
match _7 {
0 => bb9,
1 => bb7,
2 => bb6,
3 => bb4,
259776547716167413020554196531224011080 => bb11,
_ => bb5
}
}
bb11 = {
RET = (-24568_i16) as u64;
_11 = _24 != _8;
_18.1 = _10 as isize;
_3.0 = RET;
_16 = [_3.1,_1,_1,_3.1,_1,_3.1];
_3.0 = RET;
_14 = _18.0 * _24;
_26 = ['\u{103468}','\u{a5197}'];
_7 = !179741348748829906608123123951926342139_u128;
_10 = _6 + (*_30);
_9 = _18.0 >= _24;
_33 = _31 - _31;
_16 = [_1,_3.1,_3.1,_3.1,_3.1,_3.1];
_21 = '\u{66d16}';
_20 = 0_usize as i8;
_6 = _3.1 as f64;
_34 = -_33;
Goto(bb12)
}
bb12 = {
Goto(bb13)
}
bb13 = {
_18 = (_24, _24, _22);
_11 = !_9;
_13 = [_21,_21];
_8 = !_18.1;
_16 = [_3.1,_1,_3.1,_1,_1,_1];
RET = 129516100_i32 as u64;
_32 = [_21];
_3.0 = RET << _14;
_30 = core::ptr::addr_of!(_35);
_13 = [_21,_21];
_11 = !_9;
_3.0 = RET | RET;
_18.1 = _11 as isize;
_21 = '\u{9e538}';
_18.0 = _14;
_13 = [_21,_21];
_33 = _34;
_18 = (_24, _8, _22);
Call(_8 = core::intrinsics::transmute(_18.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7 = !73884301720762633902032083911223837000_u128;
_28.0 = &(*_30);
_11 = _9;
_7 = 119692537110988732930079545702100325488_u128;
_28.0 = &_10;
_14 = _9 as isize;
(*_30) = _6;
_6 = -(*_30);
_19 = _27;
_9 = !_11;
_18.2 = _22 & _22;
_34 = _31 * _33;
_34 = _31;
_14 = -_18.1;
_6 = -(*_30);
_1 = _3.1 + _3.1;
_28.0 = &_6;
_37.fld1 = [_21,_21];
_38 = core::ptr::addr_of_mut!(_2);
_3.0 = RET;
_18.0 = _21 as isize;
_3 = (RET, _1);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(2_usize, 18_usize, Move(_18), 2_usize, Move(_2), 1_usize, Move(_1), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(2_usize, 15_usize, Move(_15), 26_usize, Move(_26), 29_usize, Move(_29), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(2_usize, 11_usize, Move(_11), 7_usize, Move(_7), 20_usize, Move(_20), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u8,mut _2: u8,mut _3: f64,mut _4: u8,mut _5: u8,mut _6: u64,mut _7: u8,mut _8: u8,mut _9: u128,mut _10: u8,mut _11: u8,mut _12: u128,mut _13: u8,mut _14: u8) -> (u64, u8) {
mir! {
type RET = (u64, u8);
let _15: char;
let _16: ();
let _17: ();
{
_7 = _14;
RET = (_6, _5);
_3 = 41077_u16 as f64;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(3_usize, 7_usize, Move(_7), 4_usize, Move(_4), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(3_usize, 13_usize, Move(_13), 11_usize, Move(_11), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u8,mut _2: u8,mut _3: u8,mut _4: u8,mut _5: (u64, u8),mut _6: u8,mut _7: u8,mut _8: u8,mut _9: u8,mut _10: u8,mut _11: (u64, u8),mut _12: (u64, u8),mut _13: u8,mut _14: (u64, u8),mut _15: u8,mut _16: u8) -> u128 {
mir! {
type RET = u128;
let _17: [isize; 1];
let _18: u8;
let _19: Adt57;
let _20: isize;
let _21: *mut *mut i64;
let _22: *mut i64;
let _23: (isize, isize, i128);
let _24: [char; 1];
let _25: isize;
let _26: isize;
let _27: [char; 2];
let _28: *mut *mut isize;
let _29: usize;
let _30: f64;
let _31: ();
let _32: ();
{
_14.0 = _11.0 * _11.0;
_2 = _11.1;
_12.0 = _5.0 & _14.0;
RET = 161511378932784116630739524815625337413_u128;
_7 = _2 ^ _16;
_11.1 = _7;
_14 = _12;
_15 = _9;
_20 = (-1222081476_i32) as isize;
_3 = !_1;
_11 = (_12.0, _16);
_12 = _11;
_17 = [_20];
_20 = !9223372036854775807_isize;
_5.0 = _14.0;
_12.1 = 60802489904689328525902427445126949338_i128 as u8;
_11.0 = _12.0;
_14 = (_12.0, _16);
Goto(bb1)
}
bb1 = {
_16 = _2;
_18 = 2230595259945231662_usize as u8;
_17 = [_20];
_11.0 = !_5.0;
_4 = _5.1;
_8 = !_6;
_8 = _15;
_5.1 = _14.1;
_17 = [_20];
_12.0 = _7 as u64;
_12.0 = _11.0 + _5.0;
_11.0 = _14.0 - _14.0;
_15 = (-1897111623_i32) as u8;
_12.0 = _11.0;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
161511378932784116630739524815625337413 => bb10,
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
_7 = _3 >> _5.1;
_23.2 = (-273040070857501938_i64) as i128;
_24 = ['\u{10dbbb}'];
_1 = _2 << _4;
_23.0 = -_20;
_10 = _9 | _13;
_23.1 = -_20;
Call(_9 = fn5(_14, _5.1, _1, _1, _16, _1, _2, _4, _11, _14, _1, _1, _5, _14, _7, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = !_4;
_25 = !_20;
_5 = (_12.0, _7);
RET = 109100239334886978438229764332356165729_u128 * 261396726601324737235280490708072879819_u128;
_18 = _8;
_21 = core::ptr::addr_of_mut!(_22);
_6 = _1 << _18;
_12.0 = _11.0;
_26 = _25 << _14.1;
_5 = _11;
_8 = (-18281_i16) as u8;
_25 = _26;
_27 = ['\u{1083ba}','\u{b3486}'];
_17 = [_25];
_3 = _9;
_2 = 65446_u16 as u8;
_29 = 6_usize;
_14.0 = 37_i8 as u64;
_12.1 = !_9;
_14 = _12;
_30 = _9 as f64;
match _29 {
0 => bb1,
1 => bb8,
2 => bb6,
3 => bb10,
4 => bb5,
6 => bb13,
_ => bb12
}
}
bb12 = {
_7 = _3 >> _5.1;
_23.2 = (-273040070857501938_i64) as i128;
_24 = ['\u{10dbbb}'];
_1 = _2 << _4;
_23.0 = -_20;
_10 = _9 | _13;
_23.1 = -_20;
Call(_9 = fn5(_14, _5.1, _1, _1, _16, _1, _2, _4, _11, _14, _1, _1, _5, _14, _7, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_27 = ['\u{a23ea}','\u{c7519}'];
_23 = (_25, _26, (-74146072253651135060159008889386294998_i128));
_3 = (-3675104197913951691_i64) as u8;
RET = 106639450417882305179408220184149432177_u128 >> _25;
Goto(bb14)
}
bb14 = {
Call(_31 = dump_var(4_usize, 18_usize, Move(_18), 23_usize, Move(_23), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_31 = dump_var(4_usize, 9_usize, Move(_9), 6_usize, Move(_6), 16_usize, Move(_16), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(4_usize, 17_usize, Move(_17), 15_usize, Move(_15), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: (u64, u8),mut _2: u8,mut _3: u8,mut _4: u8,mut _5: u8,mut _6: u8,mut _7: u8,mut _8: u8,mut _9: (u64, u8),mut _10: (u64, u8),mut _11: u8,mut _12: u8,mut _13: (u64, u8),mut _14: (u64, u8),mut _15: u8,mut _16: u128) -> u8 {
mir! {
type RET = u8;
let _17: [u32; 1];
let _18: bool;
let _19: u8;
let _20: u8;
let _21: bool;
let _22: [char; 1];
let _23: Adt47;
let _24: u32;
let _25: Adt45;
let _26: (u64, u8);
let _27: usize;
let _28: (isize, isize, i128);
let _29: u64;
let _30: bool;
let _31: [char; 1];
let _32: ();
let _33: ();
{
_15 = _7 - _6;
_5 = _13.0 as u8;
_14.1 = (-903534611_i32) as u8;
_9 = _1;
_17 = [290962047_u32];
_12 = _7;
_12 = _16 as u8;
RET = _2;
_9.1 = !_6;
_13.1 = (-160522436394932522742767463526539540152_i128) as u8;
_1 = _9;
_18 = true;
_8 = _7 * _15;
_8 = _15 >> _2;
_1 = (_9.0, _6);
_19 = !_7;
RET = _9.1 ^ _15;
_2 = (-7213_i16) as u8;
_5 = !_11;
RET = !_13.1;
RET = (-3865283260479910437_i64) as u8;
_16 = !181132656434450275711443766099697835758_u128;
Call(_6 = fn6(_5, _10, _15, _15, _4, _1, _4, _9, _1.1, _15), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_22 = ['\u{a9b23}'];
RET = !_6;
_13 = (_9.0, RET);
_1.0 = 4028646276075828695_usize as u64;
_1.0 = (-1753148788_i32) as u64;
_6 = !_5;
_22 = ['\u{3aca8}'];
_13.0 = _14.0 & _10.0;
_28.0 = !9223372036854775807_isize;
_11 = _8;
_20 = _28.0 as u8;
_21 = !_18;
_10.0 = _13.0 | _13.0;
_29 = _10.0 << _6;
_28 = ((-9223372036854775808_isize), (-5_isize), 102246692224211052435521302272720639730_i128);
_12 = 2_usize as u8;
_7 = 3403688247392897985_usize as u8;
_4 = _11 & RET;
_17 = [753878168_u32];
_22 = ['\u{11376}'];
_26.0 = _28.1 as u64;
Goto(bb2)
}
bb2 = {
Call(_32 = dump_var(5_usize, 17_usize, Move(_17), 10_usize, Move(_10), 3_usize, Move(_3), 22_usize, Move(_22)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_32 = dump_var(5_usize, 16_usize, Move(_16), 13_usize, Move(_13), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_32 = dump_var(5_usize, 28_usize, Move(_28), 1_usize, Move(_1), 21_usize, Move(_21), 20_usize, Move(_20)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u8,mut _2: (u64, u8),mut _3: u8,mut _4: u8,mut _5: u8,mut _6: (u64, u8),mut _7: u8,mut _8: (u64, u8),mut _9: u8,mut _10: u8) -> u8 {
mir! {
type RET = u8;
let _11: (isize, isize, i128);
let _12: Adt48;
let _13: ();
let _14: ();
{
_8.0 = _2.0;
RET = !_8.1;
_3 = _7;
_10 = _9 * _5;
_8.1 = !_4;
_2.0 = _6.0;
_2.0 = '\u{e2a86}' as u64;
_4 = RET + RET;
_8.1 = 142573272356538302015605441292756243202_i128 as u8;
_8.0 = 18334_i16 as u64;
_9 = _7;
_2.1 = !_5;
_7 = _6.0 as u8;
RET = _5;
_2.0 = _6.0;
_8.1 = 336428065786393481803345062866996516429_u128 as u8;
RET = !_2.1;
_11.1 = !40_isize;
_4 = _9 * _9;
_11 = (9223372036854775807_isize, 9223372036854775807_isize, (-72093390927256456726395776562438432179_i128));
_11.1 = _11.0;
_8.1 = 15581_i16 as u8;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(6_usize, 11_usize, Move(_11), 10_usize, Move(_10), 5_usize, Move(_5), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(6_usize, 9_usize, Move(_9), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: f64,mut _2: f64,mut _3: u8,mut _4: f64,mut _5: u8,mut _6: f64,mut _7: u128,mut _8: u128,mut _9: u128,mut _10: u8,mut _11: u8,mut _12: f64) -> u64 {
mir! {
type RET = u64;
let _13: isize;
let _14: f32;
let _15: bool;
let _16: Adt44;
let _17: isize;
let _18: [i128; 8];
let _19: f32;
let _20: f64;
let _21: *const bool;
let _22: [u8; 6];
let _23: i8;
let _24: bool;
let _25: (*const f32, i128, *mut *mut isize);
let _26: i32;
let _27: isize;
let _28: [u32; 1];
let _29: u128;
let _30: bool;
let _31: u128;
let _32: u32;
let _33: ();
let _34: ();
{
_7 = _8;
_9 = '\u{c78bf}' as u128;
_8 = 15559891125465380124_u64 as u128;
_10 = 7554435885236258549_usize as u8;
_1 = _4;
RET = 13973146241480137619_u64;
_14 = RET as f32;
_4 = _7 as f64;
_4 = _6 * _2;
_14 = 47737_u16 as f32;
_8 = !_7;
_3 = (-22082319950747080042957172746520172331_i128) as u8;
_13 = (-9223372036854775808_isize);
_7 = '\u{b23d4}' as u128;
_2 = _1;
_5 = (-29_i8) as u8;
_7 = _8 << _8;
_15 = !true;
_13 = _15 as isize;
_12 = -_1;
_11 = _5 | _10;
_16 = Adt44::Variant0 { fld0: 26031249_u32,fld1: 10912477726395902718_usize };
_16 = Adt44::Variant0 { fld0: 2420342156_u32,fld1: 3331083144500834276_usize };
_9 = _7;
_13 = (-7403_i16) as isize;
_1 = 3957483475_u32 as f64;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
13973146241480137619 => bb5,
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
_17 = !_13;
_17 = !_13;
_11 = 3571011177_u32 as u8;
_9 = _7 & _8;
_8 = _9 << _7;
_15 = true ^ false;
_14 = (-28128_i16) as f32;
place!(Field::<usize>(Variant(_16, 0), 1)) = 8884952090051515054_usize - 1_usize;
place!(Field::<usize>(Variant(_16, 0), 1)) = 9593001747781699705_usize;
_9 = _8 << _7;
Goto(bb6)
}
bb6 = {
_13 = _17;
_4 = -_6;
_5 = _10 << _9;
Call(_17 = fn8(_6, _8, _2, _7, _2, _4, _5, _9, _12), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = 1648258062_u32 as f32;
_18 = [(-52803947640327305957763876472184886769_i128),(-169442887261543355662750113911252684141_i128),(-158908044799514204303506775259947843787_i128),(-76069812953527566458002329085039703633_i128),11953641370682763389183449456426726305_i128,(-33902337191593417644014888779753333731_i128),18773704191431455026579604141285458026_i128,52887606692270713230039142076063001173_i128];
_6 = _14 as f64;
_17 = !_13;
_18 = [32007734163692737965146506158298471374_i128,(-10728982174313950049327297594151635979_i128),(-76701812393569635464716971882918052605_i128),31797550895708756770429856731526971552_i128,(-13696725279435644916049360257435876036_i128),43693480669057479259301960191564164340_i128,24426661313540307430277804480956185628_i128,(-53634118343977061060568591614186451571_i128)];
_20 = (-107_i8) as f64;
_15 = _5 != _5;
_20 = 27_i8 as f64;
place!(Field::<usize>(Variant(_16, 0), 1)) = 11960983943544426664_usize << _7;
_4 = _12;
_19 = (-85_i8) as f32;
_5 = _3 >> _9;
place!(Field::<usize>(Variant(_16, 0), 1)) = 8812898994882585495_usize;
_3 = _5;
_9 = _8;
_1 = _2 * _4;
place!(Field::<u32>(Variant(_16, 0), 0)) = !4272977324_u32;
_21 = core::ptr::addr_of!(_15);
(*_21) = _12 > _2;
_21 = core::ptr::addr_of!(_15);
_12 = _2;
_2 = -_12;
_1 = _4 + _4;
_19 = -_14;
_1 = _4 + _2;
place!(Field::<usize>(Variant(_16, 0), 1)) = (-4429783369978977459_i64) as usize;
SetDiscriminant(_16, 2);
_15 = _3 >= _3;
match RET {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
13973146241480137619 => bb11,
_ => bb10
}
}
bb8 = {
_13 = _17;
_4 = -_6;
_5 = _10 << _9;
Call(_17 = fn8(_6, _8, _2, _7, _2, _4, _5, _9, _12), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_17 = !_13;
_17 = !_13;
_11 = 3571011177_u32 as u8;
_9 = _7 & _8;
_8 = _9 << _7;
_15 = true ^ false;
_14 = (-28128_i16) as f32;
place!(Field::<usize>(Variant(_16, 0), 1)) = 8884952090051515054_usize - 1_usize;
place!(Field::<usize>(Variant(_16, 0), 1)) = 9593001747781699705_usize;
_9 = _8 << _7;
Goto(bb6)
}
bb10 = {
Return()
}
bb11 = {
_14 = _19 + _19;
_23 = 33_i8 + (-13_i8);
_2 = -_1;
_19 = RET as f32;
_8 = !_7;
_14 = _19;
_14 = _19 - _19;
_12 = _13 as f64;
_5 = _3 & _3;
_8 = !_9;
_19 = _14;
_24 = !(*_21);
_6 = _1 + _4;
_17 = 7_usize as isize;
_24 = (*_21);
_22 = [_5,_5,_3,_5,_5,_5];
_7 = !_8;
_13 = _17;
_25.1 = !167932106869053350679366371941788149687_i128;
_12 = _2;
_19 = _14 + _14;
_11 = _5;
_10 = _3;
Goto(bb12)
}
bb12 = {
_20 = _6 * _1;
_20 = 5037366653755451107_usize as f64;
_12 = _4;
_16 = Adt44::Variant0 { fld0: 3329906233_u32,fld1: 1_usize };
match RET {
0 => bb1,
1 => bb4,
2 => bb13,
3 => bb14,
13973146241480137619 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_17 = !_13;
_17 = !_13;
_11 = 3571011177_u32 as u8;
_9 = _7 & _8;
_8 = _9 << _7;
_15 = true ^ false;
_14 = (-28128_i16) as f32;
place!(Field::<usize>(Variant(_16, 0), 1)) = 8884952090051515054_usize - 1_usize;
place!(Field::<usize>(Variant(_16, 0), 1)) = 9593001747781699705_usize;
_9 = _8 << _7;
Goto(bb6)
}
bb16 = {
_2 = _23 as f64;
_25.1 = !133207884751080562701751956554661176759_i128;
_24 = _15;
_20 = -_6;
_7 = !_8;
place!(Field::<usize>(Variant(_16, 0), 1)) = _19 as usize;
_28 = [155113091_u32];
_2 = _20;
_26 = (-2505838519899160819_i64) as i32;
place!(Field::<u32>(Variant(_16, 0), 0)) = '\u{1570e}' as u32;
_25.0 = core::ptr::addr_of!(_19);
_24 = !(*_21);
_1 = _6;
SetDiscriminant(_16, 0);
_14 = 9903_u16 as f32;
(*_21) = _11 > _11;
_4 = _12 + _20;
_22 = [_5,_11,_3,_11,_3,_3];
_15 = _24;
_24 = (*_21);
_5 = _9 as u8;
_4 = -_20;
_7 = !_8;
_13 = !_17;
(*_21) = !_24;
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(7_usize, 7_usize, Move(_7), 18_usize, Move(_18), 24_usize, Move(_24), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(7_usize, 3_usize, Move(_3), 15_usize, Move(_15), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: f64,mut _2: u128,mut _3: f64,mut _4: u128,mut _5: f64,mut _6: f64,mut _7: u8,mut _8: u128,mut _9: f64) -> isize {
mir! {
type RET = isize;
let _10: isize;
let _11: [u32; 1];
let _12: Adt54;
let _13: usize;
let _14: isize;
let _15: Adt54;
let _16: [isize; 1];
let _17: u64;
let _18: Adt44;
let _19: ();
let _20: ();
{
_3 = _7 as f64;
_7 = 52_i8 as u8;
_3 = 2363939005740039070_i64 as f64;
Call(_6 = fn9(_4, _8, _1, _1, _8, _5, _4, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = !59_u8;
RET = -(-9223372036854775808_isize);
_6 = _7 as f64;
_4 = !_8;
RET = !(-104_isize);
RET = 9223372036854775807_isize;
_3 = _9 - _1;
_10 = RET + RET;
_7 = 93_u8 >> _8;
_9 = _10 as f64;
_11 = [2684434147_u32];
Goto(bb2)
}
bb2 = {
_5 = _1 + _3;
_5 = _9 * _3;
_11 = [3679552741_u32];
_6 = _1 * _1;
_1 = _6 + _3;
_8 = _4;
_11 = [515235360_u32];
_4 = !_8;
_13 = 3_usize;
_1 = -_6;
_11 = [988101163_u32];
_5 = _3 * _3;
_7 = 32_u8 >> _4;
_9 = -_6;
_10 = !RET;
match RET {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
9223372036854775807 => bb7,
_ => bb6
}
}
bb3 = {
_7 = !59_u8;
RET = -(-9223372036854775808_isize);
_6 = _7 as f64;
_4 = !_8;
RET = !(-104_isize);
RET = 9223372036854775807_isize;
_3 = _9 - _1;
_10 = RET + RET;
_7 = 93_u8 >> _8;
_9 = _10 as f64;
_11 = [2684434147_u32];
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
_11 = [1233390244_u32];
_2 = !_4;
_14 = _10 ^ RET;
_4 = !_2;
RET = _14;
_9 = _6;
_11 = [3358603221_u32];
_10 = _14;
_2 = 26557_i16 as u128;
RET = _10;
RET = _8 as isize;
_2 = !_4;
RET = _14 >> _4;
_8 = (-7142046680777805540_i64) as u128;
_16 = [RET];
_5 = _3 * _6;
Goto(bb8)
}
bb8 = {
Call(_19 = dump_var(8_usize, 11_usize, Move(_11), 4_usize, Move(_4), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u128,mut _2: u128,mut _3: f64,mut _4: f64,mut _5: u128,mut _6: f64,mut _7: u128,mut _8: u128) -> f64 {
mir! {
type RET = f64;
let _9: ();
let _10: ();
{
RET = -_3;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(9_usize, 5_usize, Move(_5), 1_usize, Move(_1), 10_usize, _10, 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u8,mut _2: [u8; 6]) -> isize {
mir! {
type RET = isize;
let _3: [u8; 6];
let _4: char;
let _5: f64;
let _6: (u64, u8);
let _7: (isize, isize, i128);
let _8: isize;
let _9: [i128; 8];
let _10: [char; 1];
let _11: [char; 1];
let _12: Adt51;
let _13: i64;
let _14: (isize, isize, i128);
let _15: (isize, isize, i128);
let _16: Adt56;
let _17: isize;
let _18: *mut *mut isize;
let _19: char;
let _20: isize;
let _21: char;
let _22: Adt44;
let _23: f32;
let _24: (isize, isize, i128);
let _25: (i8, i64);
let _26: ();
let _27: ();
{
RET = (-9223372036854775808_isize) * 29_isize;
RET = true as isize;
RET = 19_isize + (-9223372036854775808_isize);
_2 = [_1,_1,_1,_1,_1,_1];
_5 = (-786126025466070522_i64) as f64;
_4 = '\u{9779f}';
_6.0 = !5827364673379562776_u64;
_4 = '\u{54c7c}';
_2 = [_1,_1,_1,_1,_1,_1];
_6.0 = 2154721784298641792_u64;
_6 = (5100865272403038641_u64, _1);
_7.2 = -13916042520393414401021813098751661709_i128;
_6 = (8618254154379063119_u64, _1);
_6.1 = _1;
_7.0 = RET;
_8 = 2057655973327014019_usize as isize;
RET = _7.0;
_8 = -RET;
_7.1 = RET;
_7.2 = (-9810308629833476561365465629624440704_i128);
_9 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_3 = [_6.1,_1,_1,_6.1,_1,_6.1];
Goto(bb1)
}
bb1 = {
_3 = _2;
_6 = (17086621616499344382_u64, _1);
RET = _7.0;
_1 = !_6.1;
_7.2 = -(-83892606873189875215714986326509651762_i128);
_7.2 = -(-29240860790848964632116522362703560081_i128);
_10 = [_4];
_7.0 = _4 as isize;
_12.fld0 = core::ptr::addr_of!(_12.fld2);
_14 = (_8, RET, _7.2);
_1 = _6.1 * _6.1;
_1 = _6.1 | _6.1;
Goto(bb2)
}
bb2 = {
_15.0 = -_14.0;
RET = !_7.1;
_12.fld2 = _1;
_6.0 = 13333676187804333599_u64 ^ 17537835275135281786_u64;
_13 = -(-7444365387342495172_i64);
_16.fld3 = -(-28_i8);
_15.2 = _14.2 - _7.2;
_15.0 = -_8;
_16.fld5 = 60956_u16 as u32;
_17 = !_14.0;
_17 = _15.0;
_15.1 = _16.fld3 as isize;
_14.2 = _15.2 & _15.2;
_13 = 6921113971838224589_i64;
_11 = _10;
_6 = (8621735615305780014_u64, _1);
RET = _14.0 & _17;
_2 = [_12.fld2,_6.1,_12.fld2,_6.1,_6.1,_12.fld2];
_13 = (-6721243989706179392_i64);
_12.fld2 = _6.1 - _6.1;
_12.fld4 = 5511_i16 | (-23876_i16);
_15.2 = 35537983370961504599150769049217581801_u128 as i128;
_2 = [_12.fld2,_6.1,_6.1,_12.fld2,_12.fld2,_12.fld2];
_6.1 = _12.fld2;
_2 = _3;
_20 = _7.1 & RET;
Goto(bb3)
}
bb3 = {
_7.1 = !_20;
_3 = [_6.1,_12.fld2,_12.fld2,_12.fld2,_6.1,_12.fld2];
_16.fld5 = !2389892330_u32;
_15.1 = _8;
_2 = [_1,_1,_12.fld2,_6.1,_6.1,_12.fld2];
_14.1 = _12.fld4 as isize;
_15.1 = 27875_u16 as isize;
_6.0 = _7.1 as u64;
_14 = (_7.1, _20, _7.2);
_21 = _4;
_15.2 = _7.2;
_19 = _21;
_8 = _17 << _1;
_12.fld4 = _16.fld3 as i16;
match _13 {
0 => bb2,
1 => bb4,
2 => bb5,
340282366920938463456653363442062032064 => bb7,
_ => bb6
}
}
bb4 = {
_15.0 = -_14.0;
RET = !_7.1;
_12.fld2 = _1;
_6.0 = 13333676187804333599_u64 ^ 17537835275135281786_u64;
_13 = -(-7444365387342495172_i64);
_16.fld3 = -(-28_i8);
_15.2 = _14.2 - _7.2;
_15.0 = -_8;
_16.fld5 = 60956_u16 as u32;
_17 = !_14.0;
_17 = _15.0;
_15.1 = _16.fld3 as isize;
_14.2 = _15.2 & _15.2;
_13 = 6921113971838224589_i64;
_11 = _10;
_6 = (8621735615305780014_u64, _1);
RET = _14.0 & _17;
_2 = [_12.fld2,_6.1,_12.fld2,_6.1,_6.1,_12.fld2];
_13 = (-6721243989706179392_i64);
_12.fld2 = _6.1 - _6.1;
_12.fld4 = 5511_i16 | (-23876_i16);
_15.2 = 35537983370961504599150769049217581801_u128 as i128;
_2 = [_12.fld2,_6.1,_6.1,_12.fld2,_12.fld2,_12.fld2];
_6.1 = _12.fld2;
_2 = _3;
_20 = _7.1 & RET;
Goto(bb3)
}
bb5 = {
_3 = _2;
_6 = (17086621616499344382_u64, _1);
RET = _7.0;
_1 = !_6.1;
_7.2 = -(-83892606873189875215714986326509651762_i128);
_7.2 = -(-29240860790848964632116522362703560081_i128);
_10 = [_4];
_7.0 = _4 as isize;
_12.fld0 = core::ptr::addr_of!(_12.fld2);
_14 = (_8, RET, _7.2);
_1 = _6.1 * _6.1;
_1 = _6.1 | _6.1;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_12.fld0 = core::ptr::addr_of!(_6.1);
_7 = _14;
_14 = _7;
_16.fld0 = core::ptr::addr_of_mut!(_16.fld5);
_9 = [_15.2,_15.2,_7.2,_15.2,_7.2,_15.2,_15.2,_7.2];
_16.fld5 = !4025332675_u32;
_19 = _4;
RET = _14.0 - _8;
_7.0 = RET | RET;
_11 = [_19];
_12.fld0 = core::ptr::addr_of!(_6.1);
_12.fld0 = core::ptr::addr_of!(_12.fld2);
_22 = Adt44::Variant0 { fld0: _16.fld5,fld1: 4_usize };
RET = _7.0;
_4 = _21;
_15.0 = 110590116210719489677685455606434054773_u128 as isize;
_24.1 = -_8;
Goto(bb8)
}
bb8 = {
Call(_26 = dump_var(10_usize, 20_usize, Move(_20), 13_usize, Move(_13), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_26 = dump_var(10_usize, 17_usize, Move(_17), 2_usize, Move(_2), 11_usize, Move(_11), 19_usize, Move(_19)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i8,mut _2: i8,mut _3: i128,mut _4: u128,mut _5: [char; 2],mut _6: [u32; 1],mut _7: [u32; 1],mut _8: u128,mut _9: usize,mut _10: u128,mut _11: i16,mut _12: u8,mut _13: u32,mut _14: u8,mut _15: bool,mut _16: bool) -> u16 {
mir! {
type RET = u16;
let _17: i128;
let _18: isize;
let _19: bool;
let _20: [char; 1];
let _21: [i128; 8];
let _22: [char; 2];
let _23: [char; 2];
let _24: isize;
let _25: u32;
let _26: isize;
let _27: f64;
let _28: [isize; 1];
let _29: [isize; 1];
let _30: *mut *mut isize;
let _31: *mut i64;
let _32: *mut *mut isize;
let _33: u64;
let _34: [char; 1];
let _35: usize;
let _36: [char; 2];
let _37: [char; 2];
let _38: u32;
let _39: char;
let _40: [u32; 1];
let _41: f64;
let _42: u8;
let _43: [u32; 1];
let _44: (*const f32, i128, *mut *mut isize);
let _45: i8;
let _46: usize;
let _47: (isize, isize, i128);
let _48: (isize, isize, i128);
let _49: (u64, u8);
let _50: [u32; 1];
let _51: (u64, u8);
let _52: isize;
let _53: (i8, i64);
let _54: isize;
let _55: char;
let _56: ();
let _57: ();
{
_13 = !2582499164_u32;
_9 = 60402_u16 as usize;
_17 = _3 >> _1;
_12 = _14;
_11 = !(-24018_i16);
_3 = _17 - _17;
_2 = _16 as i8;
_4 = _11 as u128;
_3 = -_17;
RET = 43151_u16 * 18165_u16;
_5 = ['\u{3e88}','\u{62aba}'];
_7 = [_13];
_8 = _1 as u128;
_7 = _6;
_18 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_9 = _14 as usize;
_17 = !_3;
_19 = !_16;
_1 = !_2;
_7 = _6;
_16 = !_19;
_6 = _7;
_16 = !_19;
_18 = 13985624819037447917_u64 as isize;
_10 = _13 as u128;
_6 = _7;
_13 = 101839443_u32;
Call(_12 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18 = RET as isize;
_6 = _7;
RET = 13653_u16;
_12 = !_14;
_2 = -_1;
_1 = _4 as i8;
_17 = _3 + _3;
_14 = _12 << _18;
_11 = RET as i16;
_18 = 9223372036854775807_isize >> _13;
_13 = 1994682773_u32 & 4096153706_u32;
_21 = [_3,_3,_3,_3,_17,_17,_3,_17];
_16 = _15 ^ _19;
_8 = _4;
RET = !37285_u16;
_10 = _4;
RET = '\u{f4cb2}' as u16;
_6 = [_13];
_22 = ['\u{29bcc}','\u{1107d}'];
_22 = ['\u{a7028}','\u{e5e52}'];
RET = 9421_u16 & 21503_u16;
Goto(bb2)
}
bb2 = {
_14 = _11 as u8;
_18 = 105_isize - 9223372036854775807_isize;
_8 = _4 - _10;
_11 = (-15771_i16) >> _17;
_1 = _10 as i8;
_14 = !_12;
_15 = _2 != _2;
_9 = 6709579831646738390_usize;
_1 = _10 as i8;
_20 = ['\u{2839a}'];
_6 = _7;
_8 = _10 & _10;
_22 = _5;
_10 = _18 as u128;
_7 = [_13];
_24 = _18;
_4 = _24 as u128;
_23 = _22;
_20 = ['\u{ad981}'];
_14 = _12;
_7 = [_13];
_7 = [_13];
_25 = _11 as u32;
_17 = _24 as i128;
_14 = _12 >> _11;
Goto(bb3)
}
bb3 = {
_18 = _24;
_16 = _15;
Goto(bb4)
}
bb4 = {
_12 = _14 & _14;
_2 = _25 as i8;
_10 = !_4;
_2 = (-1392426957437463016_i64) as i8;
_9 = '\u{102381}' as usize;
RET = _1 as u16;
_14 = _15 as u8;
RET = 28567_u16;
_4 = _17 as u128;
RET = _12 as u16;
_4 = _8;
_5 = _23;
_7 = [_13];
_19 = !_16;
_20 = ['\u{4dd00}'];
_22 = ['\u{e86d8}','\u{2512f}'];
RET = _2 as u16;
_25 = _12 as u32;
_20 = ['\u{e65ac}'];
_23 = ['\u{97b8d}','\u{34f79}'];
_21 = [_3,_3,_3,_3,_17,_3,_3,_3];
_26 = _24;
Goto(bb5)
}
bb5 = {
_23 = ['\u{ca3f4}','\u{50097}'];
_9 = 7_usize - 4_usize;
_4 = _8 << _25;
_19 = _13 >= _13;
_36 = ['\u{94b8}','\u{7658c}'];
_29 = [_18];
RET = 13363_u16;
_34 = _20;
_1 = _2 >> _2;
_16 = _15;
_13 = !_25;
_14 = (-1679167979_i32) as u8;
_20 = ['\u{a262d}'];
_16 = _15;
_1 = _2;
_27 = _13 as f64;
_15 = _19 & _16;
_35 = _9;
_38 = _13 << _25;
_29 = [_18];
_19 = _15;
_25 = _38 << _14;
_25 = _38 + _38;
_8 = !_10;
Goto(bb6)
}
bb6 = {
_2 = _1;
_14 = !_12;
_22 = ['\u{62fe0}','\u{6ffe5}'];
_16 = _15 & _19;
_19 = _13 < _25;
_27 = RET as f64;
_2 = '\u{b2b8}' as i8;
_17 = _3;
_26 = _24;
_26 = _11 as isize;
_13 = !_25;
_38 = _25 & _13;
_28 = _29;
_13 = _38;
_37 = ['\u{c41c9}','\u{d5f90}'];
_23 = ['\u{f57fe}','\u{a5fc0}'];
_7 = [_38];
_13 = !_38;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
13363 => bb10,
_ => bb9
}
}
bb7 = {
_14 = _11 as u8;
_18 = 105_isize - 9223372036854775807_isize;
_8 = _4 - _10;
_11 = (-15771_i16) >> _17;
_1 = _10 as i8;
_14 = !_12;
_15 = _2 != _2;
_9 = 6709579831646738390_usize;
_1 = _10 as i8;
_20 = ['\u{2839a}'];
_6 = _7;
_8 = _10 & _10;
_22 = _5;
_10 = _18 as u128;
_7 = [_13];
_24 = _18;
_4 = _24 as u128;
_23 = _22;
_20 = ['\u{ad981}'];
_14 = _12;
_7 = [_13];
_7 = [_13];
_25 = _11 as u32;
_17 = _24 as i128;
_14 = _12 >> _11;
Goto(bb3)
}
bb8 = {
_12 = _14 & _14;
_2 = _25 as i8;
_10 = !_4;
_2 = (-1392426957437463016_i64) as i8;
_9 = '\u{102381}' as usize;
RET = _1 as u16;
_14 = _15 as u8;
RET = 28567_u16;
_4 = _17 as u128;
RET = _12 as u16;
_4 = _8;
_5 = _23;
_7 = [_13];
_19 = !_16;
_20 = ['\u{4dd00}'];
_22 = ['\u{e86d8}','\u{2512f}'];
RET = _2 as u16;
_25 = _12 as u32;
_20 = ['\u{e65ac}'];
_23 = ['\u{97b8d}','\u{34f79}'];
_21 = [_3,_3,_3,_3,_17,_3,_3,_3];
_26 = _24;
Goto(bb5)
}
bb9 = {
_18 = _24;
_16 = _15;
Goto(bb4)
}
bb10 = {
_38 = !_25;
_20 = _34;
RET = !59373_u16;
_39 = '\u{10a304}';
_6 = _7;
_8 = _27 as u128;
_47.0 = 1010439966_i32 as isize;
_4 = _9 as u128;
_13 = 7916333240309701074_u64 as u32;
Call(_31 = fn12(_7, _7, _12, _5, _6, _5, _6, _10, _7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_47 = (_24, _26, _3);
Goto(bb12)
}
bb12 = {
_25 = _38 * _38;
_5 = [_39,_39];
_47.1 = _18;
_22 = [_39,_39];
_41 = _27 + _27;
_47.1 = _26 ^ _47.0;
_47.2 = _17;
_21 = [_3,_47.2,_17,_17,_3,_17,_47.2,_17];
_47.0 = _47.1;
_46 = _35;
_49.1 = _12;
_2 = _1;
RET = 40011_u16;
_51.0 = !9052744134052056861_u64;
_48 = (_26, _47.1, _47.2);
_37 = [_39,_39];
_45 = _4 as i8;
_39 = '\u{25960}';
_6 = [_25];
_35 = !_46;
_16 = _19;
Goto(bb13)
}
bb13 = {
_10 = _8;
_33 = _38 as u64;
_6 = _7;
_20 = [_39];
_51.1 = _4 as u8;
_22 = _5;
_44.1 = _3;
_18 = _47.0 >> _25;
_6 = _7;
_6 = [_25];
_24 = _49.1 as isize;
_27 = -_41;
_19 = _16;
_21 = [_17,_44.1,_47.2,_3,_47.2,_47.2,_47.2,_44.1];
_27 = _41;
_37 = [_39,_39];
_52 = _18;
_33 = !_51.0;
_10 = !_4;
_6 = [_25];
_6 = _7;
_6 = _7;
_53 = (_45, (-5350904669045802957_i64));
match RET {
0 => bb14,
40011 => bb16,
_ => bb15
}
}
bb14 = {
_18 = RET as isize;
_6 = _7;
RET = 13653_u16;
_12 = !_14;
_2 = -_1;
_1 = _4 as i8;
_17 = _3 + _3;
_14 = _12 << _18;
_11 = RET as i16;
_18 = 9223372036854775807_isize >> _13;
_13 = 1994682773_u32 & 4096153706_u32;
_21 = [_3,_3,_3,_3,_17,_17,_3,_17];
_16 = _15 ^ _19;
_8 = _4;
RET = !37285_u16;
_10 = _4;
RET = '\u{f4cb2}' as u16;
_6 = [_13];
_22 = ['\u{29bcc}','\u{1107d}'];
_22 = ['\u{a7028}','\u{e5e52}'];
RET = 9421_u16 & 21503_u16;
Goto(bb2)
}
bb15 = {
_47 = (_24, _26, _3);
Goto(bb12)
}
bb16 = {
_53.1 = !8589064768524621637_i64;
_12 = _49.1 ^ _14;
_15 = _16;
_49.0 = _15 as u64;
_42 = _12;
Goto(bb17)
}
bb17 = {
Call(_56 = dump_var(11_usize, 13_usize, Move(_13), 36_usize, Move(_36), 23_usize, Move(_23), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(11_usize, 8_usize, Move(_8), 33_usize, Move(_33), 7_usize, Move(_7), 39_usize, Move(_39)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(11_usize, 51_usize, Move(_51), 45_usize, Move(_45), 18_usize, Move(_18), 3_usize, Move(_3)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_56 = dump_var(11_usize, 53_usize, Move(_53), 26_usize, Move(_26), 12_usize, Move(_12), 48_usize, Move(_48)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_56 = dump_var(11_usize, 28_usize, Move(_28), 37_usize, Move(_37), 35_usize, Move(_35), 34_usize, Move(_34)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_56 = dump_var(11_usize, 20_usize, Move(_20), 24_usize, Move(_24), 57_usize, _57, 57_usize, _57), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [u32; 1],mut _2: [u32; 1],mut _3: u8,mut _4: [char; 2],mut _5: [u32; 1],mut _6: [char; 2],mut _7: [u32; 1],mut _8: u128,mut _9: [u32; 1]) -> *mut i64 {
mir! {
type RET = *mut i64;
let _10: (i8, i64);
let _11: (u64, u8);
let _12: *mut i64;
let _13: isize;
let _14: *const f32;
let _15: i128;
let _16: [i128; 8];
let _17: *const f64;
let _18: u8;
let _19: (isize, isize, i128);
let _20: (&'static f64,);
let _21: [i128; 8];
let _22: usize;
let _23: (i8, i64);
let _24: ();
let _25: ();
{
_2 = _5;
_7 = [2471482299_u32];
_5 = [56469706_u32];
RET = core::ptr::addr_of_mut!(_10.1);
_11.0 = 5228706433975388874_u64 | 3977077726766191968_u64;
(*RET) = (-277915246379158822_i64);
RET = core::ptr::addr_of_mut!((*RET));
_10 = (34_i8, 8269475624294677031_i64);
_11.1 = _3 & _3;
(*RET) = (-5210477360684564335_i64);
_10.1 = (-4942371789042532805_i64);
(*RET) = -1241825413996238601_i64;
_11 = (14617001922872825410_u64, _3);
_9 = [2056558081_u32];
_2 = [4090499207_u32];
_6 = ['\u{7a72b}','\u{f65cc}'];
_5 = _1;
_11.0 = 5215225880459812070_u64 & 2971885505052726154_u64;
_11.0 = !14249969409139982093_u64;
_11.0 = 406864663420424613_u64 + 6701405430523828314_u64;
_10 = ((-31_i8), (-8435778217025256211_i64));
_11.0 = 11421873608155347166_u64 >> (*RET);
_1 = [1818143948_u32];
_2 = [3030149890_u32];
_10.0 = !8_i8;
_3 = (-3706_i16) as u8;
Call(_11.0 = core::intrinsics::bswap(8340766653947097385_u64), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.1 = -1861621730982435363_i64;
_11.0 = 5088431699783446493_u64;
RET = core::ptr::addr_of_mut!((*RET));
_3 = _11.1;
_12 = core::ptr::addr_of_mut!((*RET));
(*_12) = 379499267432872095_i64;
_11.0 = (-16351_i16) as u64;
match (*_12) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
379499267432872095 => bb10,
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
_19.1 = _8 as isize;
_19.0 = -_19.1;
(*_12) = !5608436965159692970_i64;
_3 = _11.0 as u8;
_8 = 28225585311457999123245829275773232585_u128;
_9 = _5;
_13 = _19.1 ^ _19.1;
_11 = (7664402652294750028_u64, _3);
Goto(bb11)
}
bb11 = {
_16 = [(-37055302938069461942590962760637094450_i128),(-57736237962961993752594085563764432649_i128),(-146228787908945430967460351611386891861_i128),76541216847985086731994468364299911870_i128,4344962643882469993341002437920812924_i128,72839906286767771024144187172256660686_i128,78019017549914794346176237366821244984_i128,165173575318376360788619931552423879318_i128];
_19.2 = -5160553335647598724876363760508040189_i128;
_10.0 = 20_i8;
(*RET) = 1662723817_i32 as i64;
_3 = _11.1 >> (*RET);
match _10.0 {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
20 => bb16,
_ => bb15
}
}
bb12 = {
_10.1 = -1861621730982435363_i64;
_11.0 = 5088431699783446493_u64;
RET = core::ptr::addr_of_mut!((*RET));
_3 = _11.1;
_12 = core::ptr::addr_of_mut!((*RET));
(*_12) = 379499267432872095_i64;
_11.0 = (-16351_i16) as u64;
match (*_12) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
379499267432872095 => bb10,
_ => bb9
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
(*_12) = '\u{d04c2}' as i64;
_6 = ['\u{92c2a}','\u{10d98}'];
_16 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_7 = [3573680986_u32];
_15 = _19.2 | _19.2;
_19.1 = -_13;
(*_12) = 2748189746121461918_i64 - (-8494754912177612221_i64);
_10.1 = 460231574276435214_i64;
_19.1 = _13 + _19.0;
_12 = core::ptr::addr_of_mut!((*_12));
_10.0 = !(-114_i8);
_11.0 = 5190311124575257973_u64 & 8444288728693163193_u64;
_21 = [_19.2,_19.2,_15,_15,_19.2,_15,_15,_15];
_12 = RET;
_1 = _9;
RET = core::ptr::addr_of_mut!((*_12));
_15 = _19.2 << _19.0;
_10 = ((-88_i8), (-2234256112422890489_i64));
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(12_usize, 2_usize, Move(_2), 3_usize, Move(_3), 19_usize, Move(_19), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(12_usize, 5_usize, Move(_5), 1_usize, Move(_1), 15_usize, Move(_15), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(157_u8), std::hint::black_box(51426_u16), std::hint::black_box((-72_i8)), std::hint::black_box((-25490_i16)), std::hint::black_box(2978972335_u32), std::hint::black_box(9880908704917253006481348546383491911_u128), std::hint::black_box((-42166128349838721822058993789721424190_i128)), std::hint::black_box(4_usize));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: u64,
fld1: *const f32,
fld2: isize,
fld3: *mut u16,
fld4: u16,
fld5: [u32; 1],
fld6: (u64, u8),
fld7: i128,

},
Variant1{
fld0: *mut isize,

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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (isize, isize, i128),
fld1: *const u8,

},
Variant1{
fld0: Adt41,
fld1: f64,
fld2: isize,
fld3: usize,
fld4: *const f64,
fld5: i128,
fld6: *mut *mut i64,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: *mut *mut isize,
fld1: [char; 2],
fld2: Adt42,
fld3: [isize; 1],
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u32,
fld1: usize,

},
Variant1{
fld0: (u64, u8),
fld1: Adt42,
fld2: u128,
fld3: *const bool,
fld4: *mut i64,
fld5: usize,
fld6: *const f32,

},
Variant2{
fld0: *mut *mut i64,
fld1: *const bool,
fld2: [char; 1],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *mut *mut isize,
fld1: usize,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u128,
fld1: Adt45,
fld2: [u32; 1],

},
Variant1{
fld0: Adt41,
fld1: u128,
fld2: *mut i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const f32,
fld1: *const bool,
fld2: Adt42,
fld3: *mut i16,
fld4: Adt44,
fld5: *mut u16,

},
Variant1{
fld0: *const f32,
fld1: u16,
fld2: *mut i8,
fld3: i8,
fld4: (isize, isize, i128),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (*const f32, i128, *mut *mut isize),
fld1: usize,
fld2: u8,
fld3: [isize; 1],
fld4: Adt43,

},
Variant1{
fld0: [char; 1],
fld1: usize,
fld2: *mut *mut i64,
fld3: *mut i16,
fld4: i16,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: *const bool,
fld1: (i8, i64),

},
Variant1{
fld0: *const u8,
fld1: u8,
fld2: Adt48,
fld3: Adt41,
fld4: Adt46,
fld5: u64,
fld6: i64,
fld7: [char; 2],

},
Variant2{
fld0: [isize; 1],

},
Variant3{
fld0: Adt46,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut *mut i64,
fld1: *mut i8,

},
Variant1{
fld0: f64,
fld1: *const f32,
fld2: *mut u16,
fld3: i8,

},
Variant2{
fld0: (isize, isize, i128),
fld1: *mut *mut isize,
fld2: (*const f32, i128, *mut *mut isize),
fld3: u16,
fld4: i16,

},
Variant3{
fld0: (i8, i64),
fld1: [char; 1],
fld2: f64,
fld3: i8,
fld4: *mut u32,
fld5: *const f32,
fld6: i64,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: *const u8,
fld1: Adt46,
fld2: u8,
fld3: Adt50,
fld4: i16,
}
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u128,
fld1: Adt51,
fld2: i128,

},
Variant1{
fld0: u16,
fld1: *mut *mut isize,
fld2: Adt51,
fld3: [u8; 6],
fld4: i128,
fld5: *const f32,
fld6: i64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt48,
fld1: char,
fld2: Adt50,
fld3: Adt46,

},
Variant1{
fld0: [i128; 8],
fld1: [u32; 1],
fld2: u128,
fld3: usize,

},
Variant2{
fld0: [char; 2],
fld1: [i128; 8],
fld2: Adt48,
fld3: *mut u16,
fld4: i16,
fld5: i64,

},
Variant3{
fld0: Adt42,
fld1: *mut *mut i64,
fld2: *mut isize,
fld3: Adt45,
fld4: Adt50,
fld5: Adt44,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: usize,
fld1: Adt42,
fld2: isize,
fld3: *mut u16,
fld4: Adt51,
fld5: (isize, isize, i128),

},
Variant1{
fld0: Adt42,

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [u8; 6],
fld1: u8,
fld2: *mut i16,

},
Variant1{
fld0: Adt45,
fld1: Adt49,
fld2: [char; 2],
fld3: *const bool,
fld4: i16,
fld5: *mut i16,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: *mut u32,
fld1: char,
fld2: isize,
fld3: i8,
fld4: *mut *mut isize,
fld5: u32,
fld6: Adt49,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u64,
fld1: u8,
fld2: Adt45,
fld3: [u8; 6],
fld4: (*const f32, i128, *mut *mut isize),
fld5: [isize; 1],
fld6: Adt52,
fld7: *mut *mut i64,

},
Variant1{
fld0: Adt46,
fld1: isize,

},
Variant2{
fld0: bool,
fld1: *mut isize,
fld2: *mut i16,
fld3: Adt56,
fld4: i16,
fld5: Adt47,
fld6: usize,

}}

