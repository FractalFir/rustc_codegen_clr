#![allow(dead_code,unused_variables)]#![recursion_limit = "1024"]
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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: u32,mut _6: i32,mut _7: u64,mut _8: i128,mut _9: u16) -> bool {
mir! {
type RET = bool;
let _10: Adt52;
let _11: Adt54;
let _12: isize;
let _13: [u16; 4];
let _14: [i8; 4];
let _15: *mut [bool; 1];
let _16: i8;
let _17: [bool; 1];
let _18: f32;
let _19: Adt51;
let _20: Adt60;
let _21: Adt47;
let _22: f64;
let _23: [i64; 8];
let _24: (u16, bool);
let _25: isize;
let _26: u8;
let _27: [char; 8];
let _28: ();
let _29: ();
{
_3 = 9223372036854775807_isize & 111_isize;
_7 = !5478726433319380068_u64;
_5 = 1250908450_u32 | 3221436935_u32;
_1 = _5 >= _5;
_4 = 40_i8;
_6 = _3 as i32;
RET = !_1;
_1 = RET;
_9 = !7138_u16;
_8 = -(-15454295558679530407768066224482076590_i128);
_7 = (-1720413512642330828_i64) as u64;
_5 = 3280282297_u32 << _7;
_3 = 9223372036854775807_isize;
_5 = _3 as u32;
_1 = RET;
_5 = 3915631631_u32;
RET = _1;
_1 = !RET;
Call(_2 = fn1(_3, _3, _4, _4, _5, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = 33526_u16 | 19152_u16;
RET = _1;
_7 = !8139189264997227849_u64;
_6 = (-1663001859_i32) >> _9;
_4 = (-101_i8);
_8 = (-60341740350380714619913546259066572200_i128);
_12 = _3 >> _4;
_7 = !17217010593002289752_u64;
_2 = !258422426102276010392732924286097371208_u128;
_2 = 12129327522578970425254690235646142405_u128;
_5 = 430531354_u32;
RET = _1;
RET = _1 | _1;
RET = _6 <= _6;
_6 = !(-1179502915_i32);
_2 = 54698158944911896302385554066899958764_u128;
_2 = 20183150495039005785155095062997765361_u128 >> _7;
RET = !_1;
RET = _3 > _12;
_13 = [_9,_9,_9,_9];
_14 = [_4,_4,_4,_4];
_3 = _12 | _12;
_2 = 103805405029198191703910333097202488835_u128;
_9 = _8 as u16;
_16 = -_4;
_8 = -93725001098523918302600642728446846645_i128;
Goto(bb2)
}
bb2 = {
_12 = _3 << _3;
_16 = RET as i8;
_15 = core::ptr::addr_of_mut!(_17);
_5 = 6_usize as u32;
RET = !_1;
_1 = _12 > _12;
_18 = 8772626115040817844_i64 as f32;
_6 = _2 as i32;
_7 = 14343176511009199441_u64 - 1281975168368574434_u64;
(*_15) = [_1];
_5 = !66561499_u32;
_3 = _12;
_8 = (-22477611371936040437436339356540392447_i128);
RET = _9 != _9;
Goto(bb3)
}
bb3 = {
_3 = !_12;
_9 = 9489_u16 ^ 12330_u16;
(*_15) = [_1];
_12 = _3 * _3;
_2 = 20405629284943281572301684177097607519_u128;
_9 = 1183_u16;
_14 = [_16,_16,_16,_16];
_8 = !60867527474050835674047568814253658238_i128;
_8 = _9 as i128;
_23 = [(-6616384463691976970_i64),(-5719772943538394814_i64),(-1472035649648887178_i64),(-6357814978760572008_i64),410757467280741904_i64,(-3537308776751701313_i64),5548974456032293222_i64,3172415912949030120_i64];
RET = _1;
_22 = 30785_i16 as f64;
_1 = !RET;
_24 = (_9, _1);
RET = _12 < _12;
_24 = (_9, RET);
_7 = 10049940091531648570_u64 ^ 11738625435313877189_u64;
_25 = _12 & _3;
_22 = 253_u8 as f64;
_26 = _5 as u8;
Goto(bb4)
}
bb4 = {
Call(_28 = dump_var(0_usize, 4_usize, Move(_4), 13_usize, Move(_13), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_28 = dump_var(0_usize, 25_usize, Move(_25), 6_usize, Move(_6), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_28 = dump_var(0_usize, 3_usize, Move(_3), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: isize,mut _3: i8,mut _4: i8,mut _5: u32,mut _6: bool) -> u128 {
mir! {
type RET = u128;
let _7: f32;
let _8: isize;
let _9: f32;
let _10: Adt54;
let _11: bool;
let _12: f64;
let _13: Adt46;
let _14: u32;
let _15: i128;
let _16: isize;
let _17: f64;
let _18: [char; 8];
let _19: f32;
let _20: Adt45;
let _21: char;
let _22: *const char;
let _23: char;
let _24: [u16; 4];
let _25: (u16, bool);
let _26: f64;
let _27: [i32; 6];
let _28: (*const i16, (&'static char, &'static char, isize, usize), (&'static char, f32), i8);
let _29: &'static char;
let _30: [i32; 6];
let _31: (*const i16, (&'static char, &'static char, isize, usize), (&'static char, f32), i8);
let _32: isize;
let _33: char;
let _34: u64;
let _35: f64;
let _36: u8;
let _37: isize;
let _38: [i8; 4];
let _39: f64;
let _40: ();
let _41: ();
{
_5 = !3516438031_u32;
RET = 84265308035156803127740852952325283853_u128 << _3;
RET = 303860118288987919491412380452037771503_u128;
RET = !241284497468327967271358213770547560883_u128;
_8 = '\u{fe263}' as isize;
_1 = _8 & _8;
_9 = 188_u8 as f32;
_3 = _4;
_4 = _3;
_6 = false;
_5 = 7977057227498284718_u64 as u32;
_5 = _9 as u32;
_11 = _6;
_8 = _1 + _1;
_12 = _9 as f64;
_7 = _9 + _9;
_6 = _1 == _2;
_2 = _8;
_2 = -_8;
_3 = _4 + _4;
Call(_2 = fn2(_7, _11, _12, _7, _3, _12), ReturnTo(bb1), UnwindUnreachable())
}

bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f32,mut _2: bool,mut _3: f64,mut _4: f32,mut _5: i8,mut _6: f64) -> isize {
mir! {
type RET = isize;
let _7: (*const u64, [i32; 6], (u16, bool), [i32; 6]);
let _8: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]);
let _9: [i64; 8];
let _10: bool;
let _11: isize;
let _12: u32;
let _13: u128;
let _14: char;
let _15: isize;
let _16: [bool; 1];
let _17: u32;
let _18: u8;
let _19: bool;
let _20: u128;
let _21: [i64; 8];
let _22: ();
let _23: ();
{
_4 = _1 * _1;
RET = _4 as isize;
RET = 13039_u16 as isize;
_3 = _6;
_6 = -_3;
RET = -9223372036854775807_isize;
_1 = _4 - _4;
RET = 127_isize | (-48_isize);
Call(_6 = fn3(RET, _2, RET, _4, _5, _4, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: bool,mut _3: isize,mut _4: f32,mut _5: i8,mut _6: f32,mut _7: f32,mut _8: f32) -> f64 {
mir! {
type RET = f64;
let _9: Adt44;
let _10: Adt46;
let _11: Adt54;
let _12: Adt46;
let _13: (*mut [bool; 1],);
let _14: [bool; 1];
let _15: f32;
let _16: ();
let _17: ();
{
RET = 2503358456_u32 as f64;
_2 = false;
_7 = _6 + _8;
_1 = (-1268131455_i32) as isize;
_2 = _5 <= _5;
_7 = 29411_u16 as f32;
_2 = true;
Goto(bb1)
}
bb1 = {
RET = 222_u8 as f64;
_7 = _6 + _8;
_5 = 153041907287541130633516429294724119467_i128 as i8;
RET = 34038_u16 as f64;
RET = _1 as f64;
RET = 28904_i16 as f64;
_5 = (-82_i8);
_8 = -_6;
_7 = _8 * _8;
_1 = _3;
_4 = _7 + _6;
_1 = 1390895706102768230_i64 as isize;
_7 = _4 + _4;
_8 = -_7;
_7 = 394889736_u32 as f32;
_8 = _6 + _4;
RET = 337152400_i32 as f64;
_1 = !_3;
RET = 1354478051254772497_i64 as f64;
_7 = _8 * _6;
RET = 15364065397879008600_u64 as f64;
RET = (-16730_i16) as f64;
RET = 6803712304052257765_u64 as f64;
RET = 233713891366715275222006234170185665105_u128 as f64;
_14 = [_2];
_1 = _3;
RET = _7 as f64;
_14 = [_2];
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(3_usize, 2_usize, Move(_2), 5_usize, Move(_5), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: f32,mut _3: bool,mut _4: isize,mut _5: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _6: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _7: [i32; 6],mut _8: (u16, bool),mut _9: [bool; 7],mut _10: f32,mut _11: [i32; 6],mut _12: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _13: [bool; 1]) -> [i64; 8] {
mir! {
type RET = [i64; 8];
let _14: (*const u64, [i32; 6], (u16, bool), [i32; 6]);
let _15: isize;
let _16: [u16; 4];
let _17: u32;
let _18: *mut ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]);
let _19: f32;
let _20: Adt55;
let _21: i64;
let _22: [i64; 8];
let _23: [i32; 6];
let _24: isize;
let _25: f32;
let _26: [i32; 6];
let _27: isize;
let _28: i128;
let _29: i8;
let _30: f64;
let _31: u64;
let _32: [bool; 7];
let _33: bool;
let _34: f64;
let _35: Adt47;
let _36: isize;
let _37: f64;
let _38: [i8; 4];
let _39: Adt48;
let _40: isize;
let _41: [i64; 8];
let _42: isize;
let _43: Adt56;
let _44: ();
let _45: ();
{
_8.1 = _12.1 >= _5.1;
_10 = _5.1;
Call(_12.5 = fn5(_12.2, _5, _6.3, _4, _6.5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-8038469066910570299_i64),(-6991869502385615421_i64),(-8955012516783246264_i64),(-3917095571561186625_i64),876129652486958088_i64,5240992423532770679_i64,(-4327546437515802157_i64),9096453307208559838_i64];
_5.2 = _8.0 as u128;
RET = [1874079202904070960_i64,7429065437070720337_i64,(-700991715639623830_i64),(-4303108348369700531_i64),6622983991511503180_i64,6357437761456904615_i64,(-7153556521902246431_i64),(-1905899262739469805_i64)];
_6 = (_9, _5.1, _12.2, _12.3, _7, _5.3);
_9 = _5.0;
_14.2.0 = _8.0 * _8.0;
_5.0 = [_1,_3,_1,_1,_1,_1,_1];
_6.0 = [_1,_8.1,_1,_1,_3,_1,_1];
Goto(bb2)
}
bb2 = {
_14.3 = _5.4;
_13 = [_1];
RET = [7654423399079196723_i64,6680987552150223100_i64,7190712075862960945_i64,8320832855668259353_i64,(-1961682703272546652_i64),(-1550327061470161076_i64),3182354861922607445_i64,(-3894568148705558708_i64)];
Goto(bb3)
}
bb3 = {
_5.0 = _9;
_5.3 = core::ptr::addr_of_mut!(_13);
_12.5 = core::ptr::addr_of_mut!(_13);
_14.2.1 = _1;
_8 = Checked(_14.2.0 * _14.2.0);
_15 = !_4;
_1 = _8.1 & _14.2.1;
_16 = [_8.0,_14.2.0,_14.2.0,_8.0];
_6.4 = [(-655658064_i32),(-175984268_i32),1160155572_i32,(-1616946767_i32),1416022595_i32,(-910948460_i32)];
_15 = (-89_i8) as isize;
_5 = (_9, _10, _6.2, _12.5, _6.4, _6.5);
_6.2 = _12.2 >> _8.0;
_19 = (-754755599427159929_i64) as f32;
_12.0 = [_1,_8.1,_1,_8.1,_1,_1,_14.2.1];
_5 = (_12.0, _2, _6.2, _12.5, _7, _6.5);
_12.4 = [(-1207639023_i32),(-1642605408_i32),(-1502633219_i32),1304302661_i32,1968528953_i32,120305227_i32];
Goto(bb4)
}
bb4 = {
_16 = [_14.2.0,_8.0,_8.0,_14.2.0];
_14.3 = _12.4;
_6.3 = _12.3;
RET = [325918297083265401_i64,(-3983591988787831863_i64),5769216192331996340_i64,7714685271057181427_i64,(-277560458947547891_i64),(-3367258257628840608_i64),1926915986016320826_i64,(-8863158168891155129_i64)];
_6 = _12;
_6.5 = _5.3;
_10 = -_6.1;
_6.5 = core::ptr::addr_of_mut!(_13);
_5.1 = _4 as f32;
_10 = _5.1;
_5.0 = [_1,_1,_14.2.1,_8.1,_8.1,_14.2.1,_14.2.1];
_6.5 = _5.3;
_5.0 = [_1,_1,_14.2.1,_1,_1,_8.1,_1];
_15 = -_4;
_12.0 = [_14.2.1,_14.2.1,_8.1,_14.2.1,_1,_8.1,_1];
_17 = 971147901_u32;
RET = [(-1712420734764989187_i64),5306619398183552834_i64,(-8681429872812162010_i64),295502144176852115_i64,(-4024633820886503624_i64),8408283768643830111_i64,(-13686673996574124_i64),5076048096764296702_i64];
_21 = !3095499707834099335_i64;
_25 = _6.2 as f32;
Goto(bb5)
}
bb5 = {
_10 = _6.1;
_5.2 = '\u{c96d3}' as u128;
_8.0 = _14.2.0;
_6.0 = _5.0;
_14.2.1 = _1;
_9 = [_14.2.1,_1,_1,_1,_14.2.1,_8.1,_1];
_12.5 = core::ptr::addr_of_mut!(_13);
_18 = core::ptr::addr_of_mut!(_12);
(*_18).3 = core::ptr::addr_of_mut!(_13);
_16 = [_8.0,_14.2.0,_14.2.0,_8.0];
_8.1 = _1;
Goto(bb6)
}
bb6 = {
_12 = (_6.0, _5.1, _5.2, _6.3, _7, _6.3);
_21 = 6382234526228845736_i64 - 4003322182982782360_i64;
_27 = _17 as isize;
_5 = (_9, (*_18).1, _6.2, _6.5, (*_18).4, _6.5);
_12.4 = [(-1649158099_i32),(-776471741_i32),(-796636044_i32),(-1924595763_i32),962173534_i32,(-248391321_i32)];
_6.5 = core::ptr::addr_of_mut!(_13);
(*_18) = (_6.0, _5.1, _5.2, _5.5, _5.4, _5.3);
(*_18).3 = core::ptr::addr_of_mut!(_13);
_18 = core::ptr::addr_of_mut!(_6);
_21 = (*_18).2 as i64;
_23 = [(-250369074_i32),261857387_i32,635747742_i32,(-889523220_i32),(-27408683_i32),(-1965025806_i32)];
_17 = 121141463725575474912365336553095708493_i128 as u32;
(*_18).0 = [_8.1,_1,_8.1,_1,_8.1,_8.1,_14.2.1];
_1 = _14.2.1 <= _8.1;
_21 = (-5773343262590240353_i64);
_12.1 = _5.1 - _25;
_5.2 = _12.1 as u128;
_31 = 11366705041896150037_u64 + 8193174534983533031_u64;
(*_18).0 = _12.0;
(*_18).0 = _12.0;
(*_18).4 = _23;
(*_18).3 = (*_18).5;
RET = [_21,_21,_21,_21,_21,_21,_21,_21];
match _21 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
340282366920938463457601264169177971103 => bb10,
_ => bb9
}
}
bb7 = {
_10 = _6.1;
_5.2 = '\u{c96d3}' as u128;
_8.0 = _14.2.0;
_6.0 = _5.0;
_14.2.1 = _1;
_9 = [_14.2.1,_1,_1,_1,_14.2.1,_8.1,_1];
_12.5 = core::ptr::addr_of_mut!(_13);
_18 = core::ptr::addr_of_mut!(_12);
(*_18).3 = core::ptr::addr_of_mut!(_13);
_16 = [_8.0,_14.2.0,_14.2.0,_8.0];
_8.1 = _1;
Goto(bb6)
}
bb8 = {
_14.3 = _5.4;
_13 = [_1];
RET = [7654423399079196723_i64,6680987552150223100_i64,7190712075862960945_i64,8320832855668259353_i64,(-1961682703272546652_i64),(-1550327061470161076_i64),3182354861922607445_i64,(-3894568148705558708_i64)];
Goto(bb3)
}
bb9 = {
RET = [(-8038469066910570299_i64),(-6991869502385615421_i64),(-8955012516783246264_i64),(-3917095571561186625_i64),876129652486958088_i64,5240992423532770679_i64,(-4327546437515802157_i64),9096453307208559838_i64];
_5.2 = _8.0 as u128;
RET = [1874079202904070960_i64,7429065437070720337_i64,(-700991715639623830_i64),(-4303108348369700531_i64),6622983991511503180_i64,6357437761456904615_i64,(-7153556521902246431_i64),(-1905899262739469805_i64)];
_6 = (_9, _5.1, _12.2, _12.3, _7, _5.3);
_9 = _5.0;
_14.2.0 = _8.0 * _8.0;
_5.0 = [_1,_3,_1,_1,_1,_1,_1];
_6.0 = [_1,_8.1,_1,_1,_3,_1,_1];
Goto(bb2)
}
bb10 = {
(*_18).5 = _5.5;
_27 = _4;
_8.0 = _14.2.0 & _14.2.0;
_19 = -(*_18).1;
_6.0 = [_8.1,_8.1,_8.1,_1,_8.1,_14.2.1,_14.2.1];
_12.1 = 98_i8 as f32;
_12.3 = core::ptr::addr_of_mut!(_13);
_33 = _1 ^ _14.2.1;
(*_18).5 = _12.5;
(*_18).0 = [_1,_1,_1,_1,_14.2.1,_1,_33];
_5.2 = _6.2;
_14.3 = (*_18).4;
(*_18).3 = _5.3;
_32 = [_8.1,_33,_33,_8.1,_1,_14.2.1,_8.1];
_6.2 = _5.2 | _5.2;
_34 = 0_usize as f64;
_6 = (_32, _25, _12.2, _5.3, _23, _5.3);
match _21 {
340282366920938463457601264169177971103 => bb11,
_ => bb1
}
}
bb11 = {
(*_18) = _5;
Call(_14.2.0 = fn17(_12.3, _1, (*_18).3, (*_18).1, (*_18).0, _6.3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2 = _5.1 + (*_18).1;
_4 = _15 | _27;
(*_18).4 = [(-660617028_i32),1076809525_i32,(-988950424_i32),(-2138389688_i32),(-1643756215_i32),(-898550388_i32)];
_12.2 = (-29520_i16) as u128;
_6.2 = 14949766398302411738_usize as u128;
(*_18).0 = _32;
(*_18).5 = _12.3;
_14.2.1 = _8.1 < _33;
_5.1 = _2;
_14.2.0 = _8.0 | _8.0;
_16 = [_8.0,_14.2.0,_14.2.0,_14.2.0];
_12.4 = _6.4;
(*_18).4 = [1820009098_i32,1013514821_i32,1269224169_i32,1132293369_i32,1937611572_i32,(-1695481635_i32)];
(*_18) = _12;
_22 = [_21,_21,_21,_21,_21,_21,_21,_21];
(*_18).4 = [668337611_i32,1844890166_i32,(-2120860503_i32),(-1117733637_i32),880681024_i32,2089134390_i32];
RET = _22;
_24 = _27;
_3 = !_33;
_23 = [(-1457059433_i32),1485717224_i32,(-1965495781_i32),2017154517_i32,874427809_i32,(-1323892267_i32)];
(*_18).5 = (*_18).3;
(*_18).5 = _12.3;
(*_18) = (_32, _2, _12.2, _12.5, _5.4, _5.3);
_17 = _21 as u32;
Goto(bb13)
}
bb13 = {
_8.1 = !_1;
_8 = Checked(_14.2.0 - _14.2.0);
_12.4 = [(-137773963_i32),(-1356217560_i32),1943373197_i32,1009713109_i32,(-337499005_i32),(-1722271273_i32)];
_9 = [_14.2.1,_33,_33,_8.1,_14.2.1,_8.1,_14.2.1];
_21 = (-7063920161383409458_i64) * (-373233763406773140_i64);
_28 = 67340561415314011054361472993584368795_i128;
(*_18).1 = _5.1 * _25;
_29 = _31 as i8;
_6.1 = _2;
_12.5 = (*_18).5;
_17 = 1144408056_u32;
_2 = (*_18).1;
_33 = !_3;
match _17 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
1144408056 => bb20,
_ => bb19
}
}
bb14 = {
_2 = _5.1 + (*_18).1;
_4 = _15 | _27;
(*_18).4 = [(-660617028_i32),1076809525_i32,(-988950424_i32),(-2138389688_i32),(-1643756215_i32),(-898550388_i32)];
_12.2 = (-29520_i16) as u128;
_6.2 = 14949766398302411738_usize as u128;
(*_18).0 = _32;
(*_18).5 = _12.3;
_14.2.1 = _8.1 < _33;
_5.1 = _2;
_14.2.0 = _8.0 | _8.0;
_16 = [_8.0,_14.2.0,_14.2.0,_14.2.0];
_12.4 = _6.4;
(*_18).4 = [1820009098_i32,1013514821_i32,1269224169_i32,1132293369_i32,1937611572_i32,(-1695481635_i32)];
(*_18) = _12;
_22 = [_21,_21,_21,_21,_21,_21,_21,_21];
(*_18).4 = [668337611_i32,1844890166_i32,(-2120860503_i32),(-1117733637_i32),880681024_i32,2089134390_i32];
RET = _22;
_24 = _27;
_3 = !_33;
_23 = [(-1457059433_i32),1485717224_i32,(-1965495781_i32),2017154517_i32,874427809_i32,(-1323892267_i32)];
(*_18).5 = (*_18).3;
(*_18).5 = _12.3;
(*_18) = (_32, _2, _12.2, _12.5, _5.4, _5.3);
_17 = _21 as u32;
Goto(bb13)
}
bb15 = {
_16 = [_14.2.0,_8.0,_8.0,_14.2.0];
_14.3 = _12.4;
_6.3 = _12.3;
RET = [325918297083265401_i64,(-3983591988787831863_i64),5769216192331996340_i64,7714685271057181427_i64,(-277560458947547891_i64),(-3367258257628840608_i64),1926915986016320826_i64,(-8863158168891155129_i64)];
_6 = _12;
_6.5 = _5.3;
_10 = -_6.1;
_6.5 = core::ptr::addr_of_mut!(_13);
_5.1 = _4 as f32;
_10 = _5.1;
_5.0 = [_1,_1,_14.2.1,_8.1,_8.1,_14.2.1,_14.2.1];
_6.5 = _5.3;
_5.0 = [_1,_1,_14.2.1,_1,_1,_8.1,_1];
_15 = -_4;
_12.0 = [_14.2.1,_14.2.1,_8.1,_14.2.1,_1,_8.1,_1];
_17 = 971147901_u32;
RET = [(-1712420734764989187_i64),5306619398183552834_i64,(-8681429872812162010_i64),295502144176852115_i64,(-4024633820886503624_i64),8408283768643830111_i64,(-13686673996574124_i64),5076048096764296702_i64];
_21 = !3095499707834099335_i64;
_25 = _6.2 as f32;
Goto(bb5)
}
bb16 = {
(*_18).5 = _5.5;
_27 = _4;
_8.0 = _14.2.0 & _14.2.0;
_19 = -(*_18).1;
_6.0 = [_8.1,_8.1,_8.1,_1,_8.1,_14.2.1,_14.2.1];
_12.1 = 98_i8 as f32;
_12.3 = core::ptr::addr_of_mut!(_13);
_33 = _1 ^ _14.2.1;
(*_18).5 = _12.5;
(*_18).0 = [_1,_1,_1,_1,_14.2.1,_1,_33];
_5.2 = _6.2;
_14.3 = (*_18).4;
(*_18).3 = _5.3;
_32 = [_8.1,_33,_33,_8.1,_1,_14.2.1,_8.1];
_6.2 = _5.2 | _5.2;
_34 = 0_usize as f64;
_6 = (_32, _25, _12.2, _5.3, _23, _5.3);
match _21 {
340282366920938463457601264169177971103 => bb11,
_ => bb1
}
}
bb17 = {
_10 = _6.1;
_5.2 = '\u{c96d3}' as u128;
_8.0 = _14.2.0;
_6.0 = _5.0;
_14.2.1 = _1;
_9 = [_14.2.1,_1,_1,_1,_14.2.1,_8.1,_1];
_12.5 = core::ptr::addr_of_mut!(_13);
_18 = core::ptr::addr_of_mut!(_12);
(*_18).3 = core::ptr::addr_of_mut!(_13);
_16 = [_8.0,_14.2.0,_14.2.0,_8.0];
_8.1 = _1;
Goto(bb6)
}
bb18 = {
_12 = (_6.0, _5.1, _5.2, _6.3, _7, _6.3);
_21 = 6382234526228845736_i64 - 4003322182982782360_i64;
_27 = _17 as isize;
_5 = (_9, (*_18).1, _6.2, _6.5, (*_18).4, _6.5);
_12.4 = [(-1649158099_i32),(-776471741_i32),(-796636044_i32),(-1924595763_i32),962173534_i32,(-248391321_i32)];
_6.5 = core::ptr::addr_of_mut!(_13);
(*_18) = (_6.0, _5.1, _5.2, _5.5, _5.4, _5.3);
(*_18).3 = core::ptr::addr_of_mut!(_13);
_18 = core::ptr::addr_of_mut!(_6);
_21 = (*_18).2 as i64;
_23 = [(-250369074_i32),261857387_i32,635747742_i32,(-889523220_i32),(-27408683_i32),(-1965025806_i32)];
_17 = 121141463725575474912365336553095708493_i128 as u32;
(*_18).0 = [_8.1,_1,_8.1,_1,_8.1,_8.1,_14.2.1];
_1 = _14.2.1 <= _8.1;
_21 = (-5773343262590240353_i64);
_12.1 = _5.1 - _25;
_5.2 = _12.1 as u128;
_31 = 11366705041896150037_u64 + 8193174534983533031_u64;
(*_18).0 = _12.0;
(*_18).0 = _12.0;
(*_18).4 = _23;
(*_18).3 = (*_18).5;
RET = [_21,_21,_21,_21,_21,_21,_21,_21];
match _21 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb8,
340282366920938463457601264169177971103 => bb10,
_ => bb9
}
}
bb19 = {
_5.0 = _9;
_5.3 = core::ptr::addr_of_mut!(_13);
_12.5 = core::ptr::addr_of_mut!(_13);
_14.2.1 = _1;
_8 = Checked(_14.2.0 * _14.2.0);
_15 = !_4;
_1 = _8.1 & _14.2.1;
_16 = [_8.0,_14.2.0,_14.2.0,_8.0];
_6.4 = [(-655658064_i32),(-175984268_i32),1160155572_i32,(-1616946767_i32),1416022595_i32,(-910948460_i32)];
_15 = (-89_i8) as isize;
_5 = (_9, _10, _6.2, _12.5, _6.4, _6.5);
_6.2 = _12.2 >> _8.0;
_19 = (-754755599427159929_i64) as f32;
_12.0 = [_1,_8.1,_1,_8.1,_1,_1,_14.2.1];
_5 = (_12.0, _2, _6.2, _12.5, _7, _6.5);
_12.4 = [(-1207639023_i32),(-1642605408_i32),(-1502633219_i32),1304302661_i32,1968528953_i32,120305227_i32];
Goto(bb4)
}
bb20 = {
_14.1 = [(-94258906_i32),510761734_i32,1961638638_i32,821521235_i32,(-869170021_i32),(-1849193782_i32)];
_5.1 = (*_18).1;
_5.3 = (*_18).5;
_27 = _4;
_6 = (_12.0, _5.1, _5.2, _5.3, _14.1, _5.5);
_26 = [1701729652_i32,(-1205312989_i32),(-514802433_i32),(-1245213404_i32),(-1606094499_i32),658775904_i32];
_16 = [_14.2.0,_8.0,_8.0,_14.2.0];
_14.2.1 = _1;
_6 = (_9, _5.1, _5.2, _12.3, _7, _5.5);
_11 = [(-234946968_i32),501906406_i32,1167215416_i32,(-76909222_i32),1652944516_i32,2091427233_i32];
_42 = _27 & _24;
_12.1 = -_5.1;
_11 = [(-763897814_i32),252146592_i32,2104298729_i32,(-1363063211_i32),1732010802_i32,1342772268_i32];
(*_18).0 = [_1,_3,_1,_14.2.1,_3,_3,_3];
_5.2 = (*_18).2 * (*_18).2;
_12.1 = _2;
_40 = _42;
Goto(bb21)
}
bb21 = {
Call(_44 = dump_var(4_usize, 31_usize, Move(_31), 3_usize, Move(_3), 21_usize, Move(_21), 13_usize, Move(_13)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_44 = dump_var(4_usize, 28_usize, Move(_28), 33_usize, Move(_33), 24_usize, Move(_24), 4_usize, Move(_4)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_44 = dump_var(4_usize, 16_usize, Move(_16), 26_usize, Move(_26), 22_usize, Move(_22), 8_usize, Move(_8)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: u128,mut _2: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _3: *mut [bool; 1],mut _4: isize,mut _5: *mut [bool; 1]) -> *mut [bool; 1] {
mir! {
type RET = *mut [bool; 1];
let _6: [u16; 1];
let _7: u8;
let _8: (*mut [bool; 1],);
let _9: (*const u64, f32);
let _10: bool;
let _11: isize;
let _12: Adt54;
let _13: f64;
let _14: u32;
let _15: bool;
let _16: (u32, u32);
let _17: i128;
let _18: *const char;
let _19: i8;
let _20: Adt47;
let _21: ();
let _22: ();
{
_2.4 = [(-1720455409_i32),404341776_i32,(-2036708538_i32),149150379_i32,267630455_i32,(-363599475_i32)];
_4 = 120_isize >> _2.2;
RET = _5;
RET = _2.3;
(*RET) = [true];
(*_3) = [false];
_5 = core::ptr::addr_of_mut!((*_3));
RET = core::ptr::addr_of_mut!((*RET));
(*_3) = [true];
_2.4 = [1130503040_i32,(-1374867239_i32),(-355049162_i32),1506193562_i32,919865715_i32,(-619323439_i32)];
_2.2 = _1 << _4;
(*RET) = [false];
_2.2 = _1;
_4 = 6_isize ^ 9223372036854775807_isize;
_8.0 = core::ptr::addr_of_mut!((*_5));
_8.0 = _2.5;
_2.3 = _8.0;
RET = _3;
Call(_4 = core::intrinsics::bswap((-9223372036854775808_isize)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*RET) = [true];
_7 = 11727910755857345139_u64 as u8;
Goto(bb2)
}
bb2 = {
(*RET) = [false];
_3 = _8.0;
_2.5 = RET;
_8 = (RET,);
_8 = (_2.5,);
RET = _5;
_5 = core::ptr::addr_of_mut!((*_5));
Call(_9.0 = fn6(_2.0, _2, _2.4, _2.0, _2, _1, _2, RET, _8, _2.3, _2, _2.0, _8, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_5) = [false];
(*RET) = [true];
_2.1 = 141213906479880742665475890731655651038_i128 as f32;
_2.3 = core::ptr::addr_of_mut!((*_3));
_2.4 = [511729862_i32,(-562505639_i32),(-1560126628_i32),(-1828561041_i32),(-565694003_i32),1862746606_i32];
_2.1 = _4 as f32;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = [true];
_9.1 = _2.1;
_2.4 = [1347500811_i32,1168239341_i32,(-75819162_i32),(-1685618145_i32),(-779926594_i32),1486691170_i32];
_10 = _9.1 > _2.1;
(*_5) = [_10];
_8.0 = _5;
_1 = 139965896305810881194706480477340757959_i128 as u128;
_13 = 1279463770_u32 as f64;
_9.1 = -_2.1;
(*RET) = [_10];
_13 = 58186_u16 as f64;
Goto(bb4)
}
bb4 = {
_11 = 15052273030150998811_u64 as isize;
(*RET) = [_10];
(*RET) = [_10];
_6 = [15068_u16];
_6 = [24853_u16];
_3 = core::ptr::addr_of_mut!((*_3));
(*RET) = [_10];
_3 = core::ptr::addr_of_mut!((*RET));
(*RET) = [_10];
_8.0 = _2.5;
_3 = _5;
_13 = 39492_u16 as f64;
_14 = !827335564_u32;
(*_3) = [_10];
_2.1 = -_9.1;
_11 = -_4;
_16.0 = !_14;
_8 = (_2.3,);
(*_5) = [_10];
_8 = (_5,);
_4 = -_11;
_5 = core::ptr::addr_of_mut!((*RET));
_15 = !_10;
_3 = core::ptr::addr_of_mut!((*_5));
_17 = (-167415367108018477199945497473015486870_i128);
Goto(bb5)
}
bb5 = {
_7 = 98_u8 + 227_u8;
match _17 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
172866999812919986263429109958752724586 => bb13,
_ => bb12
}
}
bb6 = {
_11 = 15052273030150998811_u64 as isize;
(*RET) = [_10];
(*RET) = [_10];
_6 = [15068_u16];
_6 = [24853_u16];
_3 = core::ptr::addr_of_mut!((*_3));
(*RET) = [_10];
_3 = core::ptr::addr_of_mut!((*RET));
(*RET) = [_10];
_8.0 = _2.5;
_3 = _5;
_13 = 39492_u16 as f64;
_14 = !827335564_u32;
(*_3) = [_10];
_2.1 = -_9.1;
_11 = -_4;
_16.0 = !_14;
_8 = (_2.3,);
(*_5) = [_10];
_8 = (_5,);
_4 = -_11;
_5 = core::ptr::addr_of_mut!((*RET));
_15 = !_10;
_3 = core::ptr::addr_of_mut!((*_5));
_17 = (-167415367108018477199945497473015486870_i128);
Goto(bb5)
}
bb7 = {
(*_5) = [false];
(*RET) = [true];
_2.1 = 141213906479880742665475890731655651038_i128 as f32;
_2.3 = core::ptr::addr_of_mut!((*_3));
_2.4 = [511729862_i32,(-562505639_i32),(-1560126628_i32),(-1828561041_i32),(-565694003_i32),1862746606_i32];
_2.1 = _4 as f32;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = [true];
_9.1 = _2.1;
_2.4 = [1347500811_i32,1168239341_i32,(-75819162_i32),(-1685618145_i32),(-779926594_i32),1486691170_i32];
_10 = _9.1 > _2.1;
(*_5) = [_10];
_8.0 = _5;
_1 = 139965896305810881194706480477340757959_i128 as u128;
_13 = 1279463770_u32 as f64;
_9.1 = -_2.1;
(*RET) = [_10];
_13 = 58186_u16 as f64;
Goto(bb4)
}
bb8 = {
(*RET) = [false];
_3 = _8.0;
_2.5 = RET;
_8 = (RET,);
_8 = (_2.5,);
RET = _5;
_5 = core::ptr::addr_of_mut!((*_5));
Call(_9.0 = fn6(_2.0, _2, _2.4, _2.0, _2, _1, _2, RET, _8, _2.3, _2, _2.0, _8, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
(*RET) = [true];
_7 = 11727910755857345139_u64 as u8;
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
_10 = !_15;
_7 = 8722_i16 as u8;
Goto(bb14)
}
bb14 = {
_11 = -_4;
(*RET) = [_10];
_8 = (RET,);
(*_5) = [_10];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(5_usize, 1_usize, Move(_1), 14_usize, Move(_14), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [bool; 7],mut _2: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _3: [i32; 6],mut _4: [bool; 7],mut _5: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _6: u128,mut _7: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _8: *mut [bool; 1],mut _9: (*mut [bool; 1],),mut _10: *mut [bool; 1],mut _11: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _12: [bool; 7],mut _13: (*mut [bool; 1],),mut _14: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1])) -> *const u64 {
mir! {
type RET = *const u64;
let _15: (*mut [bool; 1],);
let _16: [bool; 7];
let _17: f32;
let _18: f32;
let _19: [u16; 4];
let _20: [i8; 4];
let _21: [u16; 4];
let _22: u8;
let _23: char;
let _24: i16;
let _25: f32;
let _26: *const char;
let _27: i16;
let _28: [usize; 1];
let _29: [u16; 1];
let _30: (u16, bool);
let _31: isize;
let _32: i32;
let _33: i8;
let _34: u16;
let _35: char;
let _36: (u16, bool);
let _37: [char; 8];
let _38: *const u64;
let _39: isize;
let _40: f32;
let _41: Adt45;
let _42: bool;
let _43: [char; 8];
let _44: u64;
let _45: isize;
let _46: isize;
let _47: u8;
let _48: isize;
let _49: Adt56;
let _50: u128;
let _51: f32;
let _52: u32;
let _53: (u32, u32);
let _54: [u16; 1];
let _55: isize;
let _56: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]);
let _57: f32;
let _58: char;
let _59: f32;
let _60: ((u16, bool), char, i16, char);
let _61: char;
let _62: Adt45;
let _63: f32;
let _64: char;
let _65: [i8; 4];
let _66: isize;
let _67: ((u16, bool), char, i16, char);
let _68: ();
let _69: ();
{
_3 = [(-2019209486_i32),575400275_i32,(-247722913_i32),(-1741139972_i32),(-870325289_i32),(-1145329639_i32)];
_14.1 = _11.1;
_7 = _2;
_5.4 = _7.4;
_14.3 = core::ptr::addr_of_mut!((*_8));
_20 = [(-57_i8),122_i8,(-101_i8),115_i8];
_14.2 = !_2.2;
_14.1 = 1706282702_i32 as f32;
_7.3 = core::ptr::addr_of_mut!((*_8));
_18 = _14.1 - _7.1;
(*_8) = [false];
_15.0 = _13.0;
_17 = _5.1 - _5.1;
_20 = [80_i8,81_i8,(-62_i8),66_i8];
_8 = _2.3;
_9.0 = core::ptr::addr_of_mut!((*_8));
_12 = _4;
_9.0 = _14.3;
Goto(bb1)
}
bb1 = {
_11.0 = _14.0;
_15.0 = core::ptr::addr_of_mut!((*_8));
_5.5 = core::ptr::addr_of_mut!((*_10));
_2 = (_7.0, _18, _5.2, _5.5, _14.4, _7.3);
_6 = (-1680282986_i32) as u128;
_11.5 = _7.3;
_3 = [(-39860442_i32),(-778136197_i32),1814728797_i32,(-339374337_i32),866506433_i32,826715467_i32];
_24 = 33964348_u32 as i16;
_17 = _18 * _18;
(*_10) = [false];
_19 = [27625_u16,31481_u16,54048_u16,22159_u16];
_4 = [false,true,true,true,false,false,false];
_13 = (_2.3,);
_28 = [1_usize];
Goto(bb2)
}
bb2 = {
_27 = _24 >> _7.2;
Goto(bb3)
}
bb3 = {
_7.5 = _14.5;
_5.5 = _14.3;
(*_10) = [true];
_5.0 = [true,true,true,false,true,false,false];
_14.2 = !_2.2;
(*_8) = [true];
_22 = 183_u8;
_6 = (-91679049707785904595910296776055877862_i128) as u128;
_11 = (_7.0, _5.1, _7.2, _7.5, _3, _10);
_2.1 = -_7.1;
_21 = [30759_u16,49635_u16,47122_u16,36587_u16];
_14.2 = 3828751279_u32 as u128;
_27 = !_24;
_29 = [1784_u16];
_16 = _4;
_5.4 = [1909071746_i32,1408535212_i32,(-621851550_i32),1248999085_i32,(-394893672_i32),1325507462_i32];
_6 = _2.2 >> _11.2;
_5.5 = _7.3;
(*_10) = [true];
Call(_25 = fn7(_14, _21, _5, _12, _18, _5.2, _22), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_8) = [true];
_5.0 = _12;
_14.4 = [114115510_i32,(-619378998_i32),1619729496_i32,(-2073857635_i32),(-1413013537_i32),(-297229074_i32)];
_21 = [58655_u16,10480_u16,11962_u16,13292_u16];
_5.5 = _11.5;
_5.4 = [(-1542592164_i32),(-2114119238_i32),1980900350_i32,517446591_i32,1824681516_i32,(-584323976_i32)];
_23 = '\u{491e2}';
_23 = '\u{10c7db}';
_26 = core::ptr::addr_of!(_23);
_5 = (_11.0, _17, _6, _15.0, _11.4, _7.3);
_5.3 = core::ptr::addr_of_mut!((*_8));
_31 = (-65_isize) - (-9223372036854775808_isize);
_15 = (_2.5,);
_7.2 = 7_usize as u128;
_4 = [false,false,false,true,false,true,false];
_1 = [false,true,true,false,true,false,false];
_14.4 = _5.4;
Call(_11.2 = fn12(_14, _1, _5.1, _20, _15, _1, _7, _2, _22, _27, _14.0, (*_26), _12, _1, _7, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_29 = [58733_u16];
_31 = (-9223372036854775808_isize);
(*_10) = [true];
_11.3 = core::ptr::addr_of_mut!((*_10));
_30 = (317_u16, false);
_17 = 1951514174_u32 as f32;
_14 = (_1, _11.1, _11.2, _11.3, _7.4, _11.3);
_14.0 = _1;
_34 = !_30.0;
match _30.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
317 => bb12,
_ => bb11
}
}
bb6 = {
(*_8) = [true];
_5.0 = _12;
_14.4 = [114115510_i32,(-619378998_i32),1619729496_i32,(-2073857635_i32),(-1413013537_i32),(-297229074_i32)];
_21 = [58655_u16,10480_u16,11962_u16,13292_u16];
_5.5 = _11.5;
_5.4 = [(-1542592164_i32),(-2114119238_i32),1980900350_i32,517446591_i32,1824681516_i32,(-584323976_i32)];
_23 = '\u{491e2}';
_23 = '\u{10c7db}';
_26 = core::ptr::addr_of!(_23);
_5 = (_11.0, _17, _6, _15.0, _11.4, _7.3);
_5.3 = core::ptr::addr_of_mut!((*_8));
_31 = (-65_isize) - (-9223372036854775808_isize);
_15 = (_2.5,);
_7.2 = 7_usize as u128;
_4 = [false,false,false,true,false,true,false];
_1 = [false,true,true,false,true,false,false];
_14.4 = _5.4;
Call(_11.2 = fn12(_14, _1, _5.1, _20, _15, _1, _7, _2, _22, _27, _14.0, (*_26), _12, _1, _7, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_7.5 = _14.5;
_5.5 = _14.3;
(*_10) = [true];
_5.0 = [true,true,true,false,true,false,false];
_14.2 = !_2.2;
(*_8) = [true];
_22 = 183_u8;
_6 = (-91679049707785904595910296776055877862_i128) as u128;
_11 = (_7.0, _5.1, _7.2, _7.5, _3, _10);
_2.1 = -_7.1;
_21 = [30759_u16,49635_u16,47122_u16,36587_u16];
_14.2 = 3828751279_u32 as u128;
_27 = !_24;
_29 = [1784_u16];
_16 = _4;
_5.4 = [1909071746_i32,1408535212_i32,(-621851550_i32),1248999085_i32,(-394893672_i32),1325507462_i32];
_6 = _2.2 >> _11.2;
_5.5 = _7.3;
(*_10) = [true];
Call(_25 = fn7(_14, _21, _5, _12, _18, _5.2, _22), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_27 = _24 >> _7.2;
Goto(bb3)
}
bb9 = {
_11.0 = _14.0;
_15.0 = core::ptr::addr_of_mut!((*_8));
_5.5 = core::ptr::addr_of_mut!((*_10));
_2 = (_7.0, _18, _5.2, _5.5, _14.4, _7.3);
_6 = (-1680282986_i32) as u128;
_11.5 = _7.3;
_3 = [(-39860442_i32),(-778136197_i32),1814728797_i32,(-339374337_i32),866506433_i32,826715467_i32];
_24 = 33964348_u32 as i16;
_17 = _18 * _18;
(*_10) = [false];
_19 = [27625_u16,31481_u16,54048_u16,22159_u16];
_4 = [false,true,true,true,false,false,false];
_13 = (_2.3,);
_28 = [1_usize];
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_2.5 = core::ptr::addr_of_mut!((*_8));
_10 = _14.3;
_1 = [_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1];
_32 = !947880574_i32;
_41 = Adt45 { fld0: _20,fld1: (-89_i8) };
_36.0 = 3899614078_u32 as u16;
_37 = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_11 = _5;
_5.3 = core::ptr::addr_of_mut!((*_10));
_14.5 = _10;
_37 = [_23,(*_26),(*_26),(*_26),(*_26),_23,(*_26),(*_26)];
(*_8) = [_30.1];
_2.0 = [_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1];
_36 = _30;
_4 = [_30.1,_30.1,_36.1,_30.1,_30.1,_36.1,_30.1];
_7.4 = [_32,_32,_32,_32,_32,_32];
_11.2 = _14.2;
Goto(bb13)
}
bb13 = {
_11.0 = [_30.1,_30.1,_30.1,_30.1,_36.1,_30.1,_36.1];
_14.5 = _13.0;
(*_10) = [_36.1];
_44 = 16772422021987728489_u64;
_9 = (_14.3,);
_11 = (_7.0, _17, _14.2, _7.5, _14.4, _5.3);
_7.4 = _14.4;
_35 = (*_26);
_31 = 9223372036854775807_isize;
match _41.fld1 {
340282366920938463463374607431768211367 => bb14,
_ => bb1
}
}
bb14 = {
_16 = [_30.1,_36.1,_30.1,_36.1,_36.1,_36.1,_36.1];
_9.0 = _7.5;
Goto(bb15)
}
bb15 = {
_4 = [_36.1,_30.1,_36.1,_36.1,_36.1,_36.1,_36.1];
_49.fld2 = [_34,_36.0,_36.0,_30.0];
_13.0 = _10;
_34 = _36.0;
_26 = core::ptr::addr_of!(_35);
_45 = _31;
Goto(bb16)
}
bb16 = {
_17 = _5.1 + _7.1;
_14 = (_5.0, _2.1, _11.2, _10, _7.4, _11.3);
_30.1 = _11.2 == _11.2;
_49.fld3 = [_34];
_7.5 = _15.0;
_7 = (_5.0, _17, _14.2, _13.0, _5.4, _11.5);
(*_10) = [_30.1];
_50 = _7.2;
_13 = (_2.3,);
_39 = !_31;
_48 = _45 << _50;
_38 = core::ptr::addr_of!(_44);
_2.3 = core::ptr::addr_of_mut!((*_10));
_22 = !236_u8;
_47 = (*_26) as u8;
_27 = !_24;
Goto(bb17)
}
bb17 = {
_7.2 = _50;
_28 = [17416158355374743254_usize];
_41.fld1 = !39_i8;
_14.0 = [_30.1,_30.1,_36.1,_30.1,_30.1,_30.1,_30.1];
(*_10) = [_30.1];
(*_8) = [_30.1];
_19 = [_34,_30.0,_30.0,_34];
_11.3 = core::ptr::addr_of_mut!(_49.fld6);
_23 = (*_26);
_2.4 = [_32,_32,_32,_32,_32,_32];
_14.2 = _2.2;
_16 = _2.0;
Goto(bb18)
}
bb18 = {
_45 = _48 * _48;
_11.1 = _5.1 - _7.1;
_5.2 = _47 as u128;
_40 = _7.1 * _11.1;
_49.fld2 = [_30.0,_30.0,_34,_36.0];
_5.4 = [_32,_32,_32,_32,_32,_32];
_14.0 = [_30.1,_30.1,_30.1,_30.1,_30.1,_30.1,_30.1];
_5.4 = [_32,_32,_32,_32,_32,_32];
_5.3 = core::ptr::addr_of_mut!(_49.fld6);
_2.0 = _14.0;
_29 = [_36.0];
_55 = -_45;
_56.5 = _14.5;
_2.2 = _7.2;
_52 = _36.0 as u32;
_25 = _7.1;
_44 = 10296333156501036267_u64 - 13533706443081830772_u64;
(*_38) = !13514640939195968652_u64;
_41.fld0 = _20;
Goto(bb19)
}
bb19 = {
(*_10) = [_30.1];
_14.2 = !_2.2;
_33 = -_41.fld1;
_42 = _30.1;
_44 = !15921206367683382369_u64;
Goto(bb20)
}
bb20 = {
_2.5 = _7.3;
_53.1 = (*_38) as u32;
_5.3 = _2.5;
_53 = (_52, _52);
_12 = [_42,_30.1,_30.1,_30.1,_42,_30.1,_42];
_7 = (_2.0, _40, _50, _11.5, _11.4, _2.3);
_50 = _2.2;
_56.4 = _7.4;
_5.2 = !_7.2;
_13.0 = core::ptr::addr_of_mut!((*_10));
_14.3 = core::ptr::addr_of_mut!((*_10));
_11.2 = _30.0 as u128;
_2.2 = _23 as u128;
_49.fld4 = [_41.fld1,_41.fld1,_41.fld1,_33];
_60.3 = (*_26);
_60.0 = Checked(_30.0 * _36.0);
_49.fld5.1 = -_17;
_2.2 = _11.2 - _50;
(*_26) = _23;
_65 = [_33,_33,_41.fld1,_41.fld1];
_64 = (*_26);
Call(_53.1 = core::intrinsics::transmute(_52), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
RET = _38;
_62.fld1 = !_33;
_17 = _32 as f32;
_11.0 = [_30.1,_42,_30.1,_42,_30.1,_42,_42];
_50 = _7.2 & _2.2;
_11 = _14;
Goto(bb22)
}
bb22 = {
Call(_68 = dump_var(6_usize, 48_usize, Move(_48), 3_usize, Move(_3), 16_usize, Move(_16), 55_usize, Move(_55)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_68 = dump_var(6_usize, 29_usize, Move(_29), 20_usize, Move(_20), 50_usize, Move(_50), 4_usize, Move(_4)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_68 = dump_var(6_usize, 21_usize, Move(_21), 27_usize, Move(_27), 45_usize, Move(_45), 36_usize, Move(_36)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_68 = dump_var(6_usize, 47_usize, Move(_47), 23_usize, Move(_23), 39_usize, Move(_39), 32_usize, Move(_32)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_68 = dump_var(6_usize, 34_usize, Move(_34), 69_usize, _69, 69_usize, _69, 69_usize, _69), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _2: [u16; 4],mut _3: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _4: [bool; 7],mut _5: f32,mut _6: u128,mut _7: u8) -> f32 {
mir! {
type RET = f32;
let _8: [i32; 6];
let _9: i16;
let _10: (f32, (*const u64, [i32; 6], (u16, bool), [i32; 6]), *const f32, *mut [bool; 1]);
let _11: Adt52;
let _12: i64;
let _13: isize;
let _14: (&'static char, &'static char, isize, usize);
let _15: [i64; 8];
let _16: [i64; 8];
let _17: isize;
let _18: char;
let _19: (u16, bool);
let _20: isize;
let _21: [usize; 1];
let _22: [char; 8];
let _23: [bool; 1];
let _24: *const u64;
let _25: f32;
let _26: i32;
let _27: Adt45;
let _28: [char; 8];
let _29: u16;
let _30: u16;
let _31: [usize; 1];
let _32: f32;
let _33: [bool; 7];
let _34: [i64; 8];
let _35: [usize; 1];
let _36: char;
let _37: ();
let _38: ();
{
_3 = (_4, _1.1, _6, _1.3, _1.4, _1.3);
_7 = !177_u8;
RET = (-457331200_i32) as f32;
_3.2 = _1.2 << _7;
_1.2 = 1786133592_i32 as u128;
_3.0 = _1.0;
_4 = _3.0;
_5 = -_1.1;
_10.1.2.0 = 9223372036854775807_isize as u16;
_10.1.2.0 = !15045_u16;
_10.1.2.1 = !true;
_8 = [1452799288_i32,(-518646911_i32),(-195403067_i32),1260231807_i32,(-1638571_i32),(-416849964_i32)];
_12 = 6900994365222420399_i64;
_10.1.2 = (31970_u16, true);
_10.1.2.0 = 10820_u16 - 38917_u16;
_7 = (-2002785853_i32) as u8;
_10.1.2 = (27393_u16, true);
Goto(bb1)
}
bb1 = {
_1.1 = RET;
_3.2 = _10.1.2.0 as u128;
_4 = [_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1];
_1 = _3;
_10.1.3 = [1023482099_i32,(-394510881_i32),(-285726013_i32),(-206493529_i32),2083552895_i32,696885632_i32];
_1.2 = _6;
RET = 1979544994_u32 as f32;
_1 = _3;
_1.0 = [_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1];
_9 = _7 as i16;
_4 = [_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1];
_10.1.2.0 = 29398_u16;
_4 = [_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1,_10.1.2.1];
_3 = (_4, _5, _6, _1.5, _10.1.3, _1.3);
RET = _5;
_2 = [_10.1.2.0,_10.1.2.0,_10.1.2.0,_10.1.2.0];
_8 = [(-240458683_i32),244663220_i32,(-1761197634_i32),(-771094926_i32),(-717517181_i32),(-86594504_i32)];
_3 = (_4, _1.1, _1.2, _1.3, _8, _1.3);
_14.3 = !3_usize;
_3.5 = _3.3;
_1 = _3;
_1.5 = _1.3;
RET = _1.1 * _5;
_4 = _1.0;
_10.1.1 = _8;
_18 = '\u{8c37d}';
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6900994365222420399 => bb9,
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
_19.0 = !_10.1.2.0;
_2 = [_10.1.2.0,_19.0,_19.0,_10.1.2.0];
_20 = _14.3 as isize;
_19 = (_10.1.2.0, _10.1.2.1);
_14.0 = &_18;
_10.2 = core::ptr::addr_of!(_1.1);
_1 = _3;
_18 = '\u{ab479}';
_1.4 = _10.1.1;
_14.1 = &_18;
_21 = [_14.3];
_2 = [_19.0,_19.0,_10.1.2.0,_10.1.2.0];
_16 = [_12,_12,_12,_12,_12,_12,_12,_12];
_14.1 = &_18;
_10.1.3 = _3.4;
_2 = [_19.0,_19.0,_10.1.2.0,_19.0];
_1.2 = _6;
_1.4 = [(-1007249233_i32),(-726464958_i32),737142887_i32,(-1777057617_i32),1990814634_i32,(-949122082_i32)];
_14.3 = 17180395868401786302_usize;
_3.2 = _1.2;
_12 = _20 as i64;
_22 = [_18,_18,_18,_18,_18,_18,_18,_18];
_6 = _12 as u128;
match _19.0 {
0 => bb7,
1 => bb6,
2 => bb10,
29398 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_27.fld0 = [(-12_i8),(-112_i8),26_i8,(-112_i8)];
_3.1 = _5 - RET;
_10.1.3 = [(-883934813_i32),374295907_i32,(-1464306169_i32),(-1277312654_i32),(-1000930215_i32),677711505_i32];
_22 = [_18,_18,_18,_18,_18,_18,_18,_18];
_7 = _3.2 as u8;
_21 = [_14.3];
_14.1 = &_18;
_2 = [_10.1.2.0,_19.0,_19.0,_19.0];
_3.1 = -_5;
_23 = [_10.1.2.1];
_1.4 = [(-5579863_i32),(-1982460425_i32),(-1041526443_i32),328735303_i32,(-557940836_i32),(-903479981_i32)];
_3.0 = [_19.1,_10.1.2.1,_10.1.2.1,_19.1,_19.1,_10.1.2.1,_10.1.2.1];
_1 = (_4, RET, _3.2, _3.5, _10.1.3, _3.3);
_23 = [_19.1];
_28 = [_18,_18,_18,_18,_18,_18,_18,_18];
_17 = _20 << _12;
_20 = _17;
_10.1.2 = Checked(_19.0 + _19.0);
_29 = !_19.0;
_1.3 = core::ptr::addr_of_mut!(_23);
_9 = !12709_i16;
_8 = _10.1.1;
_17 = 1821476710_u32 as isize;
_4 = _3.0;
_20 = _17 & _17;
Call(_13 = fn8(_20, _1.4, _14.3, _1.3, _19.1, _1.4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_22 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25 = _1.1;
_19 = Checked(_10.1.2.0 * _10.1.2.0);
_1.5 = _1.3;
_26 = 18802227292448347111163392647010773427_i128 as i32;
Goto(bb14)
}
bb14 = {
_19.0 = 1240215675_u32 as u16;
_13 = _10.1.2.1 as isize;
_20 = _3.2 as isize;
_5 = RET;
_9 = _10.1.2.1 as i16;
_28 = [_18,_18,_18,_18,_18,_18,_18,_18];
_14.0 = &_18;
_30 = !_29;
_10.1.2.0 = 411475150_u32 as u16;
_1.2 = _3.2 >> _29;
_3 = (_4, RET, _1.2, _1.5, _10.1.1, _1.3);
_19.1 = _13 != _13;
_32 = _5 - _3.1;
_16 = [_12,_12,_12,_12,_12,_12,_12,_12];
_10.1.2 = _19;
_30 = 74655396974258600577520075401571279651_i128 as u16;
_10.1.3 = [_26,_26,_26,_26,_26,_26];
RET = _12 as f32;
_9 = 11090_i16 ^ (-3218_i16);
_10.3 = _3.3;
_29 = _19.0 >> _6;
_3.3 = _10.3;
_7 = _20 as u8;
RET = _32;
_10.2 = core::ptr::addr_of!(_1.1);
_27.fld1 = 26_i8 >> _30;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 8_usize, Move(_8), 16_usize, Move(_16), 29_usize, Move(_29), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 17_usize, Move(_17), 19_usize, Move(_19), 4_usize, Move(_4), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(7_usize, 20_usize, Move(_20), 23_usize, Move(_23), 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: [i32; 6],mut _3: usize,mut _4: *mut [bool; 1],mut _5: bool,mut _6: [i32; 6]) -> isize {
mir! {
type RET = isize;
let _7: bool;
let _8: u8;
let _9: Adt45;
let _10: (u16, bool);
let _11: [char; 8];
let _12: [i64; 8];
let _13: bool;
let _14: Adt45;
let _15: char;
let _16: [i64; 8];
let _17: [u16; 1];
let _18: (&'static char, f32);
let _19: (u32, u32);
let _20: i32;
let _21: f64;
let _22: Adt50;
let _23: [i8; 4];
let _24: [usize; 1];
let _25: isize;
let _26: char;
let _27: ((u16, bool), char, i16, char);
let _28: Adt50;
let _29: i32;
let _30: i16;
let _31: (u16, bool);
let _32: (u16, bool);
let _33: [char; 8];
let _34: ();
let _35: ();
{
RET = _1 & _1;
_3 = 1569307434634664107_usize * 13593380749277783674_usize;
RET = _1;
_3 = !3_usize;
(*_4) = [_5];
_6 = _2;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _6;
RET = _1 ^ _1;
_7 = !_5;
_4 = core::ptr::addr_of_mut!((*_4));
RET = !_1;
_5 = _7 <= _7;
_3 = 6_usize << RET;
_1 = RET << _3;
_8 = 85_u8 ^ 28_u8;
_7 = _5;
_1 = !RET;
Call(_10.1 = fn9((*_4), _6, _3, _1, _3, _2, (*_4), _1, _7, _3, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [(-519016966_i32),(-33035674_i32),(-282744116_i32),(-1289693950_i32),861819687_i32,1809877686_i32];
_8 = 209_u8 >> _1;
_3 = 11172846332290786811_usize;
Goto(bb3)
}
bb3 = {
_11 = ['\u{4380f}','\u{d09d0}','\u{161f6}','\u{fb328}','\u{8afab}','\u{5d9d1}','\u{e53a5}','\u{11eb3}'];
RET = !_1;
RET = !_1;
_11 = ['\u{dfb62}','\u{f3021}','\u{5ce3c}','\u{416d9}','\u{5bdd8}','\u{2c210}','\u{b169d}','\u{1dd7c}'];
_7 = !_10.1;
_10 = (41763_u16, _7);
_9.fld1 = !(-111_i8);
_4 = core::ptr::addr_of_mut!((*_4));
_7 = _10.0 < _10.0;
RET = -_1;
_13 = _10.0 == _10.0;
(*_4) = [_13];
_10 = Checked(65160_u16 * 38933_u16);
_2 = [1622078062_i32,196750304_i32,597337303_i32,(-1289475590_i32),33303468_i32,1270428433_i32];
_11 = ['\u{6a888}','\u{dd5c5}','\u{a5857}','\u{9dd75}','\u{136c9}','\u{57860}','\u{35563}','\u{df2a3}'];
_14.fld0 = [_9.fld1,_9.fld1,_9.fld1,_9.fld1];
_10 = Checked(54676_u16 + 26616_u16);
match _3 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
11172846332290786811 => bb11,
_ => bb10
}
}
bb4 = {
_2 = [(-519016966_i32),(-33035674_i32),(-282744116_i32),(-1289693950_i32),861819687_i32,1809877686_i32];
_8 = 209_u8 >> _1;
_3 = 11172846332290786811_usize;
Goto(bb3)
}
bb5 = {
_2 = _6;
RET = _1 ^ _1;
_7 = !_5;
_4 = core::ptr::addr_of_mut!((*_4));
RET = !_1;
_5 = _7 <= _7;
_3 = 6_usize << RET;
_1 = RET << _3;
_8 = 85_u8 ^ 28_u8;
_7 = _5;
_1 = !RET;
Call(_10.1 = fn9((*_4), _6, _3, _1, _3, _2, (*_4), _1, _7, _3, _7, _6), ReturnTo(bb2), UnwindUnreachable())
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
_17 = [_10.0];
_14.fld0 = [_9.fld1,_9.fld1,_9.fld1,_9.fld1];
RET = (-142974272488628026192246255812219299698_i128) as isize;
_14.fld1 = !_9.fld1;
_19.0 = 992630818_u32;
_9.fld0 = _14.fld0;
_16 = [216038212522092485_i64,2799819622252039789_i64,(-6783651709064699607_i64),(-7751441403236894504_i64),3877384927906396218_i64,(-8143386803785058902_i64),2361559753180952490_i64,(-1942673864162903906_i64)];
_15 = '\u{c9ce1}';
_21 = _8 as f64;
_14.fld0 = [_9.fld1,_14.fld1,_9.fld1,_9.fld1];
_12 = [2138557990106341152_i64,(-2153415291259115643_i64),(-7706776272394280537_i64),(-783499066189219665_i64),5497800536195824436_i64,(-6468529239661563982_i64),(-4256056684539846088_i64),6670171187074196340_i64];
_3 = 1_usize;
_11[_3] = _15;
_18.0 = &_11[_3];
Goto(bb12)
}
bb12 = {
_12[_3] = _16[_3] - _16[_3];
_8 = 65_u8;
_9 = _14;
_12 = [_16[_3],_16[_3],_16[_3],_16[_3],_16[_3],_16[_3],_16[_3],_16[_3]];
_18.1 = _21 as f32;
_19.1 = !_19.0;
_14.fld0[_3] = _9.fld0[_3];
_2 = _6;
_16 = [_12[_3],_12[_3],_12[_3],_12[_3],_12[_3],_12[_3],_12[_3],_12[_3]];
_14 = Adt45 { fld0: _9.fld0,fld1: _9.fld1 };
_18.0 = &_15;
_9.fld0 = [_14.fld1,_9.fld1,_14.fld0[_3],_9.fld1];
_5 = _2[_3] > _6[_3];
_1 = RET;
_12[_3] = -_16[_3];
_8 = 206_u8 << _14.fld0[_3];
_14.fld0[_3] = _9.fld1 << _2[_3];
_9.fld1 = _2[_3] as i8;
_9.fld0 = _14.fld0;
_10.0 = 12633_u16 * 45637_u16;
_16 = [_12[_3],_12[_3],_12[_3],_12[_3],_12[_3],_12[_3],_12[_3],_12[_3]];
_27.1 = _11[_3];
_9.fld1 = _13 as i8;
match _2[_3] {
0 => bb8,
1 => bb13,
374295907 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_2 = _6;
RET = _1 ^ _1;
_7 = !_5;
_4 = core::ptr::addr_of_mut!((*_4));
RET = !_1;
_5 = _7 <= _7;
_3 = 6_usize << RET;
_1 = RET << _3;
_8 = 85_u8 ^ 28_u8;
_7 = _5;
_1 = !RET;
Call(_10.1 = fn9((*_4), _6, _3, _1, _3, _2, (*_4), _1, _7, _3, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_13 = _5 | _7;
_26 = _15;
_5 = _13;
_27.0.1 = _5;
_20 = !_6[_3];
_19.0 = !_19.1;
_4 = core::ptr::addr_of_mut!((*_4));
_27.1 = _15;
_12 = [_16[_3],_16[_3],_16[_3],_16[_3],_16[_3],_16[_3],_16[_3],_16[_3]];
_24 = [_3];
_32 = (_10.0, _13);
_2 = _6;
_27.1 = _26;
_2 = [_6[_3],_6[_3],_20,_20,_20,_20];
_19 = (1971899024_u32, 727936369_u32);
_23[_3] = _9.fld1;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(8_usize, 10_usize, Move(_10), 15_usize, Move(_15), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(8_usize, 5_usize, Move(_5), 3_usize, Move(_3), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(8_usize, 17_usize, Move(_17), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [bool; 1],mut _2: [i32; 6],mut _3: usize,mut _4: isize,mut _5: usize,mut _6: [i32; 6],mut _7: [bool; 1],mut _8: isize,mut _9: bool,mut _10: usize,mut _11: bool,mut _12: [i32; 6]) -> bool {
mir! {
type RET = bool;
let _13: ((u16, bool), char, i16, char);
let _14: bool;
let _15: Adt60;
let _16: f32;
let _17: ();
let _18: ();
{
_6 = _12;
RET = _11;
_12 = [(-1232230316_i32),(-1212070127_i32),1569444481_i32,(-354598523_i32),(-1266802373_i32),1540574081_i32];
_11 = RET & RET;
Call(RET = fn10(_6, _2, _8, _11, _10, _7, _5, _2, _5, _2, _11, _8, _3, _12, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = !_11;
_1 = [RET];
_4 = _8 - _8;
Goto(bb2)
}
bb2 = {
Call(_17 = dump_var(9_usize, 5_usize, Move(_5), 12_usize, Move(_12), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_17 = dump_var(9_usize, 11_usize, Move(_11), 2_usize, Move(_2), 18_usize, _18, 18_usize, _18), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i32; 6],mut _2: [i32; 6],mut _3: isize,mut _4: bool,mut _5: usize,mut _6: [bool; 1],mut _7: usize,mut _8: [i32; 6],mut _9: usize,mut _10: [i32; 6],mut _11: bool,mut _12: isize,mut _13: usize,mut _14: [i32; 6],mut _15: [i32; 6]) -> bool {
mir! {
type RET = bool;
let _16: bool;
let _17: char;
let _18: isize;
let _19: [i8; 4];
let _20: ();
let _21: ();
{
_3 = _12 - _12;
_13 = _9 | _9;
_13 = _9;
_7 = !_9;
_5 = _9 + _9;
_13 = _7;
_3 = _12 * _12;
_12 = _3 & _3;
_9 = !_5;
_6 = [_4];
_1 = _15;
_1 = [(-439889952_i32),(-1551027145_i32),1333433428_i32,1124268748_i32,(-1830276680_i32),359120515_i32];
Call(_16 = fn11(_1, _10, _3, _4, _6, _14, _5, _12, _14, _1, _11, _13, _12, _8, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _8;
_5 = _7 & _9;
_2 = _8;
_3 = !_12;
RET = _16 < _4;
_8 = [(-55603812_i32),(-681934612_i32),1516170630_i32,(-278389490_i32),(-310069378_i32),1395304202_i32];
_2 = _1;
_14 = _1;
RET = !_11;
_7 = !_5;
RET = _12 != _12;
_18 = '\u{61b3e}' as isize;
_9 = _7;
_10 = _14;
_13 = 14_u8 as usize;
_1 = [(-768272557_i32),(-1644468437_i32),(-1101764580_i32),(-647254836_i32),1256695940_i32,(-2058003402_i32)];
RET = _16 < _16;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(10_usize, 6_usize, Move(_6), 1_usize, Move(_1), 14_usize, Move(_14), 16_usize, Move(_16)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(10_usize, 18_usize, Move(_18), 12_usize, Move(_12), 8_usize, Move(_8), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [i32; 6],mut _2: [i32; 6],mut _3: isize,mut _4: bool,mut _5: [bool; 1],mut _6: [i32; 6],mut _7: usize,mut _8: isize,mut _9: [i32; 6],mut _10: [i32; 6],mut _11: bool,mut _12: usize,mut _13: isize,mut _14: [i32; 6],mut _15: [i32; 6]) -> bool {
mir! {
type RET = bool;
let _16: isize;
let _17: [bool; 1];
let _18: [usize; 1];
let _19: i32;
let _20: [char; 8];
let _21: *const i16;
let _22: ();
let _23: ();
{
_9 = _2;
_12 = !_7;
_14 = [(-1359244000_i32),(-1664410874_i32),2003090814_i32,(-2008945234_i32),1884382719_i32,1946528782_i32];
_10 = [189652544_i32,1498858146_i32,788428189_i32,715463246_i32,342961375_i32,(-54792210_i32)];
_9 = [(-744208433_i32),(-586432239_i32),(-1117337009_i32),(-1149946085_i32),547394072_i32,(-86406410_i32)];
RET = _11 > _11;
_4 = !_11;
RET = _11 < _4;
_18 = [_12];
_4 = RET;
_1 = _10;
_4 = !RET;
_20 = ['\u{4b2f4}','\u{b086c}','\u{37899}','\u{47259}','\u{aad77}','\u{4ebe3}','\u{8a33f}','\u{e900c}'];
RET = !_4;
_16 = _13;
_5 = [_11];
_20 = ['\u{bc6f0}','\u{a68bb}','\u{e7eec}','\u{3d98c}','\u{1072b8}','\u{608a3}','\u{aacb7}','\u{ff844}'];
_8 = _16 ^ _13;
RET = _12 < _12;
_6 = [523795028_i32,(-507104648_i32),(-371288590_i32),(-967699943_i32),2068533372_i32,(-885410347_i32)];
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(11_usize, 7_usize, Move(_7), 18_usize, Move(_18), 14_usize, Move(_14), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(11_usize, 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, Move(_12), 16_usize, Move(_16)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_22 = dump_var(11_usize, 20_usize, Move(_20), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _2: [bool; 7],mut _3: f32,mut _4: [i8; 4],mut _5: (*mut [bool; 1],),mut _6: [bool; 7],mut _7: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _8: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _9: u8,mut _10: i16,mut _11: [bool; 7],mut _12: char,mut _13: [bool; 7],mut _14: [bool; 7],mut _15: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),mut _16: [i32; 6]) -> u128 {
mir! {
type RET = u128;
let _17: isize;
let _18: usize;
let _19: f64;
let _20: [bool; 1];
let _21: (*const u64, f32);
let _22: u16;
let _23: u32;
let _24: ((u16, bool), char, i16, char);
let _25: ();
let _26: ();
{
RET = _8.2;
_4 = [59_i8,(-105_i8),(-64_i8),(-16_i8)];
match _9 {
183 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_6 = _11;
_9 = 180_u8 >> _10;
_15.2 = !RET;
_3 = -_8.1;
_8.0 = [true,false,true,true,true,true,true];
_8.1 = 45783_u16 as f32;
_1.2 = _8.2;
_13 = [true,true,false,false,false,false,false];
_15.5 = _15.3;
_8.5 = _15.5;
_18 = !5954996272060836930_usize;
_8.4 = _7.4;
_1.2 = 8405122512919468620_i64 as u128;
Call(_17 = core::intrinsics::transmute(_18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = [true,false,false,false,true,true,false];
_4 = [16_i8,69_i8,93_i8,127_i8];
_1.4 = [1811196450_i32,584561404_i32,985753648_i32,828294896_i32,1331698517_i32,359649874_i32];
_16 = [(-1789171430_i32),1354353046_i32,1446666291_i32,1541543609_i32,1175320515_i32,1608484115_i32];
_7.0 = _14;
_6 = _1.0;
_7.2 = _1.2;
_15.5 = _8.5;
RET = _10 as u128;
_15.4 = _8.4;
Call(_15 = fn13(_16, _13, _8.4, _8.0, _1.4, _13, _8.4, _8.4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15.0 = _11;
_7.4 = _16;
_20 = [true];
_1.0 = [false,false,true,false,true,true,true];
match _15.2 {
0 => bb1,
249325357501681595307245092061671571208 => bb5,
_ => bb2
}
}
bb5 = {
_14 = _6;
_13 = _8.0;
_21.1 = -_7.1;
_1.0 = [false,false,true,false,true,false,true];
_8.2 = !_1.2;
_18 = _10 as usize;
_21.1 = _17 as f32;
match _15.2 {
249325357501681595307245092061671571208 => bb7,
_ => bb6
}
}
bb6 = {
_2 = [true,false,false,false,true,true,false];
_4 = [16_i8,69_i8,93_i8,127_i8];
_1.4 = [1811196450_i32,584561404_i32,985753648_i32,828294896_i32,1331698517_i32,359649874_i32];
_16 = [(-1789171430_i32),1354353046_i32,1446666291_i32,1541543609_i32,1175320515_i32,1608484115_i32];
_7.0 = _14;
_6 = _1.0;
_7.2 = _1.2;
_15.5 = _8.5;
RET = _10 as u128;
_15.4 = _8.4;
Call(_15 = fn13(_16, _13, _8.4, _8.0, _1.4, _13, _8.4, _8.4), ReturnTo(bb4), UnwindUnreachable())
}
bb7 = {
_15.5 = _15.3;
_1.4 = _7.4;
_1.2 = !_15.2;
_7.0 = _6;
_11 = _14;
_8.1 = _3;
_23 = (-27_i8) as u32;
RET = !_15.2;
_21.1 = -_7.1;
_8.5 = core::ptr::addr_of_mut!(_20);
_17 = (-1_isize);
_9 = 25_u8 >> _1.2;
Goto(bb8)
}
bb8 = {
Call(_25 = dump_var(12_usize, 13_usize, Move(_13), 16_usize, Move(_16), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_25 = dump_var(12_usize, 14_usize, Move(_14), 2_usize, Move(_2), 23_usize, Move(_23), 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i32; 6],mut _2: [bool; 7],mut _3: [i32; 6],mut _4: [bool; 7],mut _5: [i32; 6],mut _6: [bool; 7],mut _7: [i32; 6],mut _8: [i32; 6]) -> ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]) {
mir! {
type RET = ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]);
let _9: i32;
let _10: Adt60;
let _11: u32;
let _12: [u16; 1];
let _13: ((u16, bool), char, i16, char);
let _14: *mut [bool; 1];
let _15: [i8; 4];
let _16: f64;
let _17: [i64; 8];
let _18: Adt57;
let _19: bool;
let _20: isize;
let _21: i8;
let _22: [i8; 4];
let _23: isize;
let _24: Adt46;
let _25: Adt55;
let _26: char;
let _27: char;
let _28: [i64; 8];
let _29: (*mut [bool; 1],);
let _30: bool;
let _31: [bool; 7];
let _32: Adt45;
let _33: u64;
let _34: Adt45;
let _35: f64;
let _36: isize;
let _37: f32;
let _38: bool;
let _39: Adt56;
let _40: u32;
let _41: Adt59;
let _42: i8;
let _43: f32;
let _44: f64;
let _45: [i32; 6];
let _46: ();
let _47: ();
{
RET.4 = _5;
_1 = [(-2046133584_i32),839721909_i32,(-595375904_i32),(-502138679_i32),760553844_i32,(-185069318_i32)];
RET.4 = [(-167356476_i32),(-114909724_i32),(-1130890027_i32),1167244788_i32,199046566_i32,(-238934737_i32)];
_3 = [564594718_i32,(-330441834_i32),(-1924244617_i32),1979803268_i32,(-228203952_i32),(-2075960113_i32)];
RET.0 = [false,false,true,true,false,false,true];
_7 = [395495285_i32,1716610334_i32,2103964153_i32,(-431104816_i32),1776666286_i32,(-1088929189_i32)];
_8 = [985266102_i32,(-653489620_i32),(-636305580_i32),1770512611_i32,(-431823342_i32),1221519091_i32];
_4 = [false,false,true,true,true,true,true];
_1 = [1323858568_i32,(-1592168457_i32),(-2145095609_i32),1735114122_i32,1908267236_i32,(-1932523489_i32)];
RET.1 = (-21989_i16) as f32;
_9 = !269036886_i32;
_3 = [_9,_9,_9,_9,_9,_9];
_2 = [false,true,true,false,true,false,true];
RET.0 = _6;
_8 = [_9,_9,_9,_9,_9,_9];
RET.0 = [true,true,false,true,false,true,true];
_4 = [true,false,true,false,true,true,false];
_5 = [_9,_9,_9,_9,_9,_9];
_5 = [_9,_9,_9,_9,_9,_9];
_5 = [_9,_9,_9,_9,_9,_9];
RET.1 = 31_i8 as f32;
Goto(bb1)
}
bb1 = {
_1 = _7;
_11 = (-9223372036854775808_isize) as u32;
RET.1 = _11 as f32;
Goto(bb2)
}
bb2 = {
_3 = [_9,_9,_9,_9,_9,_9];
_7 = [_9,_9,_9,_9,_9,_9];
_13.0 = (9186_u16, false);
_12 = [_13.0.0];
_8 = _3;
_8 = _1;
_8 = [_9,_9,_9,_9,_9,_9];
_13.2 = (-4326_i16) & 13446_i16;
_2 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.3 = '\u{95d41}';
_4 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.3 = '\u{6ddd2}';
_5 = [_9,_9,_9,_9,_9,_9];
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_11 = 3839300834_u32;
RET.2 = !217681558040014140746378180824944302680_u128;
_5 = [_9,_9,_9,_9,_9,_9];
_9 = !1111594796_i32;
Goto(bb3)
}
bb3 = {
_2 = _4;
_13.1 = _13.3;
_15 = [(-31_i8),(-89_i8),(-66_i8),(-76_i8)];
RET.4 = [_9,_9,_9,_9,_9,_9];
_13.2 = _13.1 as i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_1 = _5;
_6 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_16 = RET.2 as f64;
_13.0 = Checked(13748_u16 * 64995_u16);
_13.1 = _13.3;
_13.3 = _13.1;
RET.2 = 203738470139901268210040508022818393329_u128;
RET.4 = _3;
_13.2 = 836_i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.2 = !(-28213_i16);
RET.4 = [_9,_9,_9,_9,_9,_9];
_15 = [(-61_i8),(-117_i8),(-1_i8),(-26_i8)];
_13.1 = _13.3;
RET.0 = _2;
_16 = _9 as f64;
RET.1 = _16 as f32;
_17 = [(-3720037063480436980_i64),6942634374672446956_i64,5432615303376783732_i64,179849242297957510_i64,(-6494880270467268261_i64),(-7628553733193713603_i64),7105221465022277036_i64,(-8113625179584755324_i64)];
_12 = [_13.0.0];
_13.2 = 48_u8 as i16;
_11 = !2920083558_u32;
match RET.2 {
203738470139901268210040508022818393329 => bb4,
_ => bb1
}
}
bb4 = {
RET.0 = _6;
_4 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_12 = [_13.0.0];
_13.1 = _13.3;
_13.1 = _13.3;
_15 = [0_i8,88_i8,110_i8,(-36_i8)];
_17 = [4531985984477315267_i64,(-7024029373401159487_i64),6638766709980639146_i64,7243470877611120429_i64,(-5436721264731805309_i64),(-6372806183367466828_i64),(-4961798424472261811_i64),7445245618844645982_i64];
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_16 = _11 as f64;
_13.0 = (36858_u16, false);
_11 = !3247919795_u32;
_2 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
RET.2 = 326230587926867438236069284335304585994_u128;
_8 = [_9,_9,_9,_9,_9,_9];
_13.0.0 = _11 as u16;
_1 = [_9,_9,_9,_9,_9,_9];
_19 = _13.0.1;
_13.0 = (16434_u16, _19);
_19 = _13.0.1;
RET.4 = [_9,_9,_9,_9,_9,_9];
_7 = [_9,_9,_9,_9,_9,_9];
_13.3 = _13.1;
RET.4 = [_9,_9,_9,_9,_9,_9];
Goto(bb5)
}
bb5 = {
_13.2 = 27771_i16 >> _9;
_21 = (-51_i8) & 81_i8;
_1 = [_9,_9,_9,_9,_9,_9];
_13.0.1 = _19;
_13.0 = (16708_u16, _19);
_19 = _13.0.1 > _13.0.1;
_13.2 = !6983_i16;
_13.3 = _13.1;
_2 = _6;
_20 = _21 as isize;
_21 = _11 as i8;
_8 = [_9,_9,_9,_9,_9,_9];
RET.2 = !202988826064738475752881134271785136465_u128;
_8 = [_9,_9,_9,_9,_9,_9];
_1 = _8;
_13.0 = Checked(51525_u16 - 49512_u16);
RET.0 = [_19,_19,_13.0.1,_19,_19,_13.0.1,_19];
_3 = [_9,_9,_9,_9,_9,_9];
_26 = _13.3;
RET.2 = !317743054041833152051026531369149763906_u128;
_20 = 58_isize;
_23 = -_20;
_19 = _16 > _16;
_15 = [_21,_21,_21,_21];
match _20 {
0 => bb2,
58 => bb7,
_ => bb6
}
}
bb6 = {
_2 = _4;
_13.1 = _13.3;
_15 = [(-31_i8),(-89_i8),(-66_i8),(-76_i8)];
RET.4 = [_9,_9,_9,_9,_9,_9];
_13.2 = _13.1 as i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_1 = _5;
_6 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_16 = RET.2 as f64;
_13.0 = Checked(13748_u16 * 64995_u16);
_13.1 = _13.3;
_13.3 = _13.1;
RET.2 = 203738470139901268210040508022818393329_u128;
RET.4 = _3;
_13.2 = 836_i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.2 = !(-28213_i16);
RET.4 = [_9,_9,_9,_9,_9,_9];
_15 = [(-61_i8),(-117_i8),(-1_i8),(-26_i8)];
_13.1 = _13.3;
RET.0 = _2;
_16 = _9 as f64;
RET.1 = _16 as f32;
_17 = [(-3720037063480436980_i64),6942634374672446956_i64,5432615303376783732_i64,179849242297957510_i64,(-6494880270467268261_i64),(-7628553733193713603_i64),7105221465022277036_i64,(-8113625179584755324_i64)];
_12 = [_13.0.0];
_13.2 = 48_u8 as i16;
_11 = !2920083558_u32;
match RET.2 {
203738470139901268210040508022818393329 => bb4,
_ => bb1
}
}
bb7 = {
_13.1 = _26;
_13.0.1 = _19;
_7 = _8;
_15 = [_21,_21,_21,_21];
_22 = [_21,_21,_21,_21];
_7 = _8;
_5 = [_9,_9,_9,_9,_9,_9];
_11 = 2562781119_u32 + 2370749902_u32;
_13.1 = _13.3;
Goto(bb8)
}
bb8 = {
_17 = [4944974171854431775_i64,681046193267070942_i64,(-3246548178169835439_i64),(-2115423712017713447_i64),1661448439035979396_i64,6856191789714755123_i64,5547154108471585863_i64,827039170133579107_i64];
_6 = [_13.0.1,_19,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_23 = _20 | _20;
_27 = _13.3;
RET.0 = _2;
_8 = [_9,_9,_9,_9,_9,_9];
RET.0 = [_19,_13.0.1,_13.0.1,_19,_19,_13.0.1,_19];
_6 = RET.0;
_32.fld1 = _21;
_13.3 = _13.1;
_22 = _15;
_27 = _26;
_15 = _22;
_32 = Adt45 { fld0: _22,fld1: _21 };
_1 = [_9,_9,_9,_9,_9,_9];
_13.2 = 2549_i16 & (-18868_i16);
_27 = _26;
_13.3 = _27;
_34 = _32;
_7 = [_9,_9,_9,_9,_9,_9];
match _20 {
0 => bb7,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
58 => bb16,
_ => bb15
}
}
bb9 = {
_13.1 = _26;
_13.0.1 = _19;
_7 = _8;
_15 = [_21,_21,_21,_21];
_22 = [_21,_21,_21,_21];
_7 = _8;
_5 = [_9,_9,_9,_9,_9,_9];
_11 = 2562781119_u32 + 2370749902_u32;
_13.1 = _13.3;
Goto(bb8)
}
bb10 = {
_2 = _4;
_13.1 = _13.3;
_15 = [(-31_i8),(-89_i8),(-66_i8),(-76_i8)];
RET.4 = [_9,_9,_9,_9,_9,_9];
_13.2 = _13.1 as i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_1 = _5;
_6 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_16 = RET.2 as f64;
_13.0 = Checked(13748_u16 * 64995_u16);
_13.1 = _13.3;
_13.3 = _13.1;
RET.2 = 203738470139901268210040508022818393329_u128;
RET.4 = _3;
_13.2 = 836_i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.2 = !(-28213_i16);
RET.4 = [_9,_9,_9,_9,_9,_9];
_15 = [(-61_i8),(-117_i8),(-1_i8),(-26_i8)];
_13.1 = _13.3;
RET.0 = _2;
_16 = _9 as f64;
RET.1 = _16 as f32;
_17 = [(-3720037063480436980_i64),6942634374672446956_i64,5432615303376783732_i64,179849242297957510_i64,(-6494880270467268261_i64),(-7628553733193713603_i64),7105221465022277036_i64,(-8113625179584755324_i64)];
_12 = [_13.0.0];
_13.2 = 48_u8 as i16;
_11 = !2920083558_u32;
match RET.2 {
203738470139901268210040508022818393329 => bb4,
_ => bb1
}
}
bb11 = {
_13.2 = 27771_i16 >> _9;
_21 = (-51_i8) & 81_i8;
_1 = [_9,_9,_9,_9,_9,_9];
_13.0.1 = _19;
_13.0 = (16708_u16, _19);
_19 = _13.0.1 > _13.0.1;
_13.2 = !6983_i16;
_13.3 = _13.1;
_2 = _6;
_20 = _21 as isize;
_21 = _11 as i8;
_8 = [_9,_9,_9,_9,_9,_9];
RET.2 = !202988826064738475752881134271785136465_u128;
_8 = [_9,_9,_9,_9,_9,_9];
_1 = _8;
_13.0 = Checked(51525_u16 - 49512_u16);
RET.0 = [_19,_19,_13.0.1,_19,_19,_13.0.1,_19];
_3 = [_9,_9,_9,_9,_9,_9];
_26 = _13.3;
RET.2 = !317743054041833152051026531369149763906_u128;
_20 = 58_isize;
_23 = -_20;
_19 = _16 > _16;
_15 = [_21,_21,_21,_21];
match _20 {
0 => bb2,
58 => bb7,
_ => bb6
}
}
bb12 = {
RET.0 = _6;
_4 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_12 = [_13.0.0];
_13.1 = _13.3;
_13.1 = _13.3;
_15 = [0_i8,88_i8,110_i8,(-36_i8)];
_17 = [4531985984477315267_i64,(-7024029373401159487_i64),6638766709980639146_i64,7243470877611120429_i64,(-5436721264731805309_i64),(-6372806183367466828_i64),(-4961798424472261811_i64),7445245618844645982_i64];
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_16 = _11 as f64;
_13.0 = (36858_u16, false);
_11 = !3247919795_u32;
_2 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
RET.2 = 326230587926867438236069284335304585994_u128;
_8 = [_9,_9,_9,_9,_9,_9];
_13.0.0 = _11 as u16;
_1 = [_9,_9,_9,_9,_9,_9];
_19 = _13.0.1;
_13.0 = (16434_u16, _19);
_19 = _13.0.1;
RET.4 = [_9,_9,_9,_9,_9,_9];
_7 = [_9,_9,_9,_9,_9,_9];
_13.3 = _13.1;
RET.4 = [_9,_9,_9,_9,_9,_9];
Goto(bb5)
}
bb13 = {
_2 = _4;
_13.1 = _13.3;
_15 = [(-31_i8),(-89_i8),(-66_i8),(-76_i8)];
RET.4 = [_9,_9,_9,_9,_9,_9];
_13.2 = _13.1 as i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_1 = _5;
_6 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_16 = RET.2 as f64;
_13.0 = Checked(13748_u16 * 64995_u16);
_13.1 = _13.3;
_13.3 = _13.1;
RET.2 = 203738470139901268210040508022818393329_u128;
RET.4 = _3;
_13.2 = 836_i16;
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.2 = !(-28213_i16);
RET.4 = [_9,_9,_9,_9,_9,_9];
_15 = [(-61_i8),(-117_i8),(-1_i8),(-26_i8)];
_13.1 = _13.3;
RET.0 = _2;
_16 = _9 as f64;
RET.1 = _16 as f32;
_17 = [(-3720037063480436980_i64),6942634374672446956_i64,5432615303376783732_i64,179849242297957510_i64,(-6494880270467268261_i64),(-7628553733193713603_i64),7105221465022277036_i64,(-8113625179584755324_i64)];
_12 = [_13.0.0];
_13.2 = 48_u8 as i16;
_11 = !2920083558_u32;
match RET.2 {
203738470139901268210040508022818393329 => bb4,
_ => bb1
}
}
bb14 = {
_3 = [_9,_9,_9,_9,_9,_9];
_7 = [_9,_9,_9,_9,_9,_9];
_13.0 = (9186_u16, false);
_12 = [_13.0.0];
_8 = _3;
_8 = _1;
_8 = [_9,_9,_9,_9,_9,_9];
_13.2 = (-4326_i16) & 13446_i16;
_2 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.3 = '\u{95d41}';
_4 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_13.3 = '\u{6ddd2}';
_5 = [_9,_9,_9,_9,_9,_9];
RET.0 = [_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1,_13.0.1];
_11 = 3839300834_u32;
RET.2 = !217681558040014140746378180824944302680_u128;
_5 = [_9,_9,_9,_9,_9,_9];
_9 = !1111594796_i32;
Goto(bb3)
}
bb15 = {
_1 = _7;
_11 = (-9223372036854775808_isize) as u32;
RET.1 = _11 as f32;
Goto(bb2)
}
bb16 = {
_5 = [_9,_9,_9,_9,_9,_9];
_37 = RET.1;
_19 = _13.0.1 ^ _13.0.1;
_13.1 = _13.3;
_13.2 = 7801_i16 - 25919_i16;
_32.fld1 = -_34.fld1;
_34.fld1 = _32.fld1 & _21;
_30 = !_19;
_35 = _16;
_13.3 = _26;
_29.0 = core::ptr::addr_of_mut!(_39.fld6);
_33 = 6634599005754088464_i64 as u64;
Call(_14 = fn14(_34, _19, _19, _17, _27, _11, _34, _6, _12, _30, RET.0, _19, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_29.0 = _14;
_6 = [_19,_30,_19,_30,_19,_30,_19];
RET.5 = core::ptr::addr_of_mut!(_39.fld6);
_32 = Adt45 { fld0: _22,fld1: _34.fld1 };
RET = (_6, _37, 117722331789340727848052445727133203636_u128, _14, _5, _14);
_39.fld2 = [_13.0.0,_13.0.0,_13.0.0,_13.0.0];
_42 = _34.fld1;
_17 = [4509148957852608547_i64,(-7329192141874594510_i64),6623442152110919880_i64,(-4324587127096533271_i64),(-747673970712438100_i64),(-801838297584839809_i64),6965451037009963290_i64,5352397236779189719_i64];
RET.1 = _16 as f32;
_34.fld1 = -_42;
RET.4 = [_9,_9,_9,_9,_9,_9];
_4 = [_19,_13.0.1,_19,_13.0.1,_13.0.1,_30,_13.0.1];
_12 = [_13.0.0];
RET = (_4, _37, 249325357501681595307245092061671571208_u128, _29.0, _7, _14);
RET.5 = _29.0;
_28 = [4080614934153649218_i64,5884282245933800104_i64,6928282135000999657_i64,(-6616221450603008971_i64),1019120459643392952_i64,(-1721435112791008471_i64),8106956904529656312_i64,(-4056728495493359488_i64)];
_2 = RET.0;
_13.0 = Checked(50537_u16 - 25198_u16);
_20 = _23;
_37 = RET.1 * RET.1;
Goto(bb18)
}
bb18 = {
Call(_46 = dump_var(13_usize, 26_usize, Move(_26), 8_usize, Move(_8), 2_usize, Move(_2), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(13_usize, 13_usize, Move(_13), 17_usize, Move(_17), 30_usize, Move(_30), 33_usize, Move(_33)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_46 = dump_var(13_usize, 5_usize, Move(_5), 23_usize, Move(_23), 42_usize, Move(_42), 22_usize, Move(_22)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: Adt45,mut _2: bool,mut _3: bool,mut _4: [i64; 8],mut _5: char,mut _6: u32,mut _7: Adt45,mut _8: [bool; 7],mut _9: [u16; 1],mut _10: bool,mut _11: [bool; 7],mut _12: bool,mut _13: Adt45) -> *mut [bool; 1] {
mir! {
type RET = *mut [bool; 1];
let _14: u64;
let _15: Adt46;
let _16: i128;
let _17: [u16; 4];
let _18: [u16; 4];
let _19: f64;
let _20: f32;
let _21: (&'static char, f32);
let _22: [i64; 8];
let _23: (*mut [bool; 1],);
let _24: i128;
let _25: [i8; 4];
let _26: isize;
let _27: f64;
let _28: [bool; 1];
let _29: (&'static char, f32);
let _30: f64;
let _31: f64;
let _32: [u16; 4];
let _33: i8;
let _34: i16;
let _35: [u16; 1];
let _36: (*const i16, (&'static char, &'static char, isize, usize), (&'static char, f32), i8);
let _37: isize;
let _38: [bool; 1];
let _39: *const char;
let _40: Adt49;
let _41: *const char;
let _42: f64;
let _43: [u16; 1];
let _44: f64;
let _45: isize;
let _46: ();
let _47: ();
{
_7.fld0 = _13.fld0;
_12 = _2;
_11 = [_10,_3,_12,_12,_10,_2,_3];
_1.fld1 = !_13.fld1;
_7.fld0 = [_1.fld1,_1.fld1,_7.fld1,_7.fld1];
_7.fld1 = _13.fld1;
_10 = !_12;
_1.fld0 = [_13.fld1,_13.fld1,_13.fld1,_13.fld1];
_13.fld1 = 164_u8 as i8;
_13 = _7;
_13 = Adt45 { fld0: _7.fld0,fld1: _7.fld1 };
_12 = _2;
_13.fld1 = -_7.fld1;
_6 = (-3592587517761414623_i64) as u32;
_5 = '\u{7e30f}';
_13.fld1 = _3 as i8;
_7 = Adt45 { fld0: _1.fld0,fld1: _13.fld1 };
_1.fld0 = [_7.fld1,_13.fld1,_7.fld1,_13.fld1];
_4 = [4222568073514960366_i64,(-1406452639999564593_i64),(-2703211901544316524_i64),(-4964941894981612574_i64),8286606552145457790_i64,8366357250614844049_i64,(-4561077338614084246_i64),(-1425562189033182311_i64)];
_9 = [1616_u16];
_13.fld1 = _7.fld1 >> _7.fld1;
_7.fld0 = _13.fld0;
_7 = _13;
_8 = _11;
_16 = (-163467478525890236460698359342212511503_i128);
_5 = '\u{303cf}';
Call(_7.fld1 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.fld1 = _13.fld1;
_17 = [44473_u16,17062_u16,6479_u16,6291_u16];
_4 = [6080094514003539134_i64,(-3334655776763352237_i64),7875444833841014483_i64,8082505453009536174_i64,(-4863069047823918089_i64),1860949924193019476_i64,3998696919038911573_i64,(-1789861780321627985_i64)];
_1.fld1 = _3 as i8;
_21.1 = 116724747251885356341822201685102631103_u128 as f32;
_21.0 = &_5;
_11 = _8;
_16 = !(-45303153626979852701277027012417531385_i128);
_9 = [59334_u16];
_16 = _7.fld1 as i128;
_10 = _3;
_18 = _17;
_12 = _2 ^ _2;
Goto(bb2)
}
bb2 = {
_22 = [(-5247917339567297569_i64),(-65584167780456916_i64),(-6309290325857772919_i64),7132775813258966043_i64,8826523722293653847_i64,(-9108415989943984898_i64),(-3282430598803962281_i64),(-2178405120055531517_i64)];
_7 = Adt45 { fld0: _1.fld0,fld1: _1.fld1 };
_3 = _12 | _12;
_17 = [34344_u16,64594_u16,49657_u16,21712_u16];
_1.fld1 = (-3172862743874594203_i64) as i8;
_21.1 = 49_u8 as f32;
_19 = _16 as f64;
_20 = _21.1 * _21.1;
_22 = _4;
_14 = 2113710663070835132_u64 | 14420962730197333596_u64;
_1.fld1 = -_7.fld1;
_19 = 11477017232636541441_usize as f64;
_5 = '\u{331a}';
_12 = _3;
_14 = 8109519381257742105_u64;
_24 = 44174_u16 as i128;
_13 = Adt45 { fld0: _1.fld0,fld1: _7.fld1 };
_4 = [2102689931225573311_i64,(-2124798839535261646_i64),5712509649222595624_i64,7315070853111662378_i64,4609554697120312066_i64,6626026216242208380_i64,6209956212772146132_i64,840107089501847376_i64];
_1 = Adt45 { fld0: _13.fld0,fld1: _7.fld1 };
_13.fld1 = -_1.fld1;
_21.1 = _20 * _20;
_8 = [_10,_12,_12,_3,_3,_12,_3];
_5 = '\u{99324}';
_3 = _16 >= _16;
Call(_19 = fn15(_17, _17, _12, _17, _22, _16, _12, _12, _3, _3, _1.fld1, _13.fld0, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_20 = _16 as f32;
_21.0 = &_5;
_21.0 = &_5;
_27 = -_19;
_22 = [4819755880305836747_i64,(-371637459496847399_i64),4643386922702835310_i64,2444940597525015471_i64,1260177034965302215_i64,(-4195604963045453216_i64),3906435374319470387_i64,8856653375941515334_i64];
_26 = -13_isize;
_28 = [_2];
_31 = _19;
_23.0 = core::ptr::addr_of_mut!(_28);
_29.0 = &_5;
_21 = (Move(_29.0), _20);
_29.0 = &_5;
_1 = Adt45 { fld0: _7.fld0,fld1: _13.fld1 };
_32 = [29722_u16,387_u16,50399_u16,17521_u16];
_8 = [_12,_12,_3,_12,_3,_12,_12];
_9 = [47940_u16];
_4 = _22;
RET = core::ptr::addr_of_mut!(_28);
(*RET) = [_2];
match _14 {
0 => bb1,
8109519381257742105 => bb4,
_ => bb2
}
}
bb4 = {
_19 = -_31;
_21.1 = _20 - _20;
RET = core::ptr::addr_of_mut!((*RET));
_11 = _8;
_31 = _26 as f64;
_36.0 = core::ptr::addr_of!(_34);
_27 = _14 as f64;
_31 = _19 * _27;
_37 = -_26;
RET = core::ptr::addr_of_mut!((*RET));
_36.2 = (Move(_29.0), _21.1);
_29.0 = &_5;
Goto(bb5)
}
bb5 = {
_7 = Adt45 { fld0: _1.fld0,fld1: _13.fld1 };
_31 = _14 as f64;
_26 = _37 + _37;
_35 = _9;
_25 = [_7.fld1,_7.fld1,_13.fld1,_13.fld1];
Call(_36.1.3 = core::intrinsics::transmute(_26), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1.fld0 = _7.fld0;
_6 = !2508865060_u32;
_16 = _24 >> _1.fld1;
_36.1.3 = 2434697372081675692_usize | 9590140611741717063_usize;
_36.2 = (Move(_29.0), _21.1);
_36.3 = _13.fld1;
_7.fld0 = [_7.fld1,_7.fld1,_1.fld1,_13.fld1];
_33 = _1.fld1;
_29.0 = &_5;
Goto(bb7)
}
bb7 = {
_36.2.0 = Move(_29.0);
_33 = _13.fld1;
_5 = '\u{575a8}';
_34 = !25247_i16;
_39 = core::ptr::addr_of!(_5);
_36.1.2 = _26;
(*_39) = '\u{c8f46}';
_31 = _27 * _19;
_5 = '\u{f4604}';
_30 = 94352667480246727951708611364554965804_u128 as f64;
_36.1.1 = &(*_39);
(*RET) = [_12];
(*RET) = [_12];
_21.0 = Move(_36.1.1);
_10 = _12;
(*RET) = [_12];
_33 = _36.3 << _13.fld1;
match _14 {
8109519381257742105 => bb9,
_ => bb8
}
}
bb8 = {
_19 = -_31;
_21.1 = _20 - _20;
RET = core::ptr::addr_of_mut!((*RET));
_11 = _8;
_31 = _26 as f64;
_36.0 = core::ptr::addr_of!(_34);
_27 = _14 as f64;
_31 = _19 * _27;
_37 = -_26;
RET = core::ptr::addr_of_mut!((*RET));
_36.2 = (Move(_29.0), _21.1);
_29.0 = &_5;
Goto(bb5)
}
bb9 = {
_36.3 = 873590829_i32 as i8;
_36.2.0 = &_5;
_18 = _17;
_36.2 = Move(_21);
_7.fld1 = _33;
_36.1.1 = &(*_39);
_36.1.2 = _36.1.3 as isize;
_23 = (RET,);
_29.1 = _36.2.1 * _20;
(*RET) = [_10];
_1.fld0 = [_7.fld1,_33,_7.fld1,_33];
_36.3 = !_7.fld1;
_36.2 = (Move(_36.1.1), _29.1);
_40.fld2 = !17090_u16;
_40.fld1.0 = core::ptr::addr_of_mut!(_28);
_36.2.0 = &(*_39);
_36.1.1 = &_5;
_41 = _39;
_21.1 = _13.fld1 as f32;
_33 = -_7.fld1;
_36.2.0 = Move(_36.1.1);
(*RET) = [_12];
_3 = _10 ^ _12;
_42 = _30 + _30;
_33 = _36.3 ^ _13.fld1;
_28 = [_3];
_9 = [_40.fld2];
Goto(bb10)
}
bb10 = {
_8 = [_3,_3,_2,_3,_10,_3,_3];
_29.1 = _36.2.1 + _21.1;
_32 = [_40.fld2,_40.fld2,_40.fld2,_40.fld2];
_1 = Adt45 { fld0: _7.fld0,fld1: _7.fld1 };
_38 = [_3];
_34 = 21444_i16 | (-20288_i16);
_43 = _35;
_20 = (-396827042_i32) as f32;
_40 = Adt49 { fld0: 144232504794960102007031780686721104105_u128,fld1: _23,fld2: 49430_u16 };
_38 = [_2];
_29.0 = &(*_39);
_2 = !_10;
_21.0 = &_5;
(*_39) = '\u{4a132}';
_36.1.1 = &(*_39);
_29 = (Move(_36.1.1), _36.2.1);
_44 = _31 - _30;
_1 = Adt45 { fld0: _7.fld0,fld1: _7.fld1 };
_40.fld1 = (_23.0,);
_36.2.0 = Move(_29.0);
_21 = Move(_36.2);
_27 = _30;
_29.1 = -_21.1;
_21.0 = &(*_39);
_40.fld1.0 = RET;
_3 = !_10;
RET = _40.fld1.0;
(*RET) = [_10];
Goto(bb11)
}
bb11 = {
Call(_46 = dump_var(14_usize, 17_usize, Move(_17), 38_usize, Move(_38), 10_usize, Move(_10), 32_usize, Move(_32)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_46 = dump_var(14_usize, 25_usize, Move(_25), 3_usize, Move(_3), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_46 = dump_var(14_usize, 8_usize, Move(_8), 9_usize, Move(_9), 26_usize, Move(_26), 28_usize, Move(_28)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_46 = dump_var(14_usize, 16_usize, Move(_16), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [u16; 4],mut _2: [u16; 4],mut _3: bool,mut _4: [u16; 4],mut _5: [i64; 8],mut _6: i128,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: i8,mut _12: [i8; 4],mut _13: bool) -> f64 {
mir! {
type RET = f64;
let _14: [u16; 4];
let _15: (u32, u32);
let _16: char;
let _17: Adt44;
let _18: isize;
let _19: i64;
let _20: char;
let _21: Adt57;
let _22: u16;
let _23: [i8; 4];
let _24: bool;
let _25: f32;
let _26: i32;
let _27: Adt60;
let _28: f64;
let _29: Adt45;
let _30: usize;
let _31: f32;
let _32: [char; 8];
let _33: u16;
let _34: Adt52;
let _35: isize;
let _36: isize;
let _37: (&'static char, f32);
let _38: f32;
let _39: i128;
let _40: i8;
let _41: ();
let _42: ();
{
_5 = [(-336267784030655438_i64),8670326448615472667_i64,5877466794576753561_i64,3534599852141246992_i64,7242551240112290960_i64,(-7147177809906169560_i64),6012445907727735904_i64,(-5969400183610643263_i64)];
_4 = [48326_u16,53627_u16,38319_u16,2723_u16];
_2 = [20_u16,59228_u16,59005_u16,62269_u16];
RET = _11 as f64;
_6 = !(-167546401377096549630970593949998095838_i128);
_12 = [_11,_11,_11,_11];
Goto(bb1)
}
bb1 = {
_2 = [12913_u16,1016_u16,26201_u16,56449_u16];
_15.0 = 3147909175_u32 >> _6;
_15.0 = 2471624169_u32 - 2927677580_u32;
RET = 24635_u16 as f64;
_15 = (1662451612_u32, 1274562828_u32);
_9 = !_3;
_5 = [4858327632750225559_i64,954593202427691918_i64,624085836193392109_i64,2042087226770612771_i64,(-6948258165262555778_i64),8648199663316855803_i64,(-8558416847188686571_i64),(-3323598335925990740_i64)];
Call(_15 = fn16(_7, _9, _7, _9, _5, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = '\u{188c3}';
_13 = _15.1 <= _15.0;
_14 = [7718_u16,17781_u16,55936_u16,2931_u16];
_16 = '\u{fdb90}';
_4 = [51768_u16,31311_u16,27868_u16,50313_u16];
_14 = [25462_u16,41520_u16,32742_u16,34690_u16];
_18 = -(-9223372036854775808_isize);
Goto(bb3)
}
bb3 = {
_18 = (-9223372036854775808_isize);
_1 = [21934_u16,50855_u16,46623_u16,8899_u16];
_2 = [62283_u16,7252_u16,6095_u16,51104_u16];
_3 = !_13;
_4 = [30426_u16,50610_u16,9377_u16,13666_u16];
RET = (-8606204651790133178_i64) as f64;
_10 = _9 <= _3;
_15.0 = 139_i16 as u32;
_15.1 = _15.0 ^ _15.0;
_14 = [62936_u16,6953_u16,56547_u16,21924_u16];
_15 = (2262112691_u32, 3391384029_u32);
_15 = (905160806_u32, 3878252536_u32);
_13 = _7;
Goto(bb4)
}
bb4 = {
_7 = _3;
_10 = _3 != _3;
_3 = _7;
_14 = [13573_u16,65407_u16,57212_u16,55266_u16];
_3 = _7;
_16 = '\u{c4541}';
_4 = [33599_u16,59634_u16,40531_u16,2127_u16];
_19 = (-7024549733425536350_i64);
_3 = _10;
_23 = [_11,_11,_11,_11];
_22 = _15.1 as u16;
_6 = (-167304824104903867770325699792638090468_i128);
_2 = _1;
_15.1 = !_15.0;
_26 = !(-3317215_i32);
_6 = 118401318469648275388632362843712860839_u128 as i128;
_15.1 = !_15.0;
_26 = 1332456697_i32 * 207147381_i32;
_19 = 4359729793539907438_i64;
_16 = '\u{7874d}';
_4 = _14;
_26 = 275660034_i32;
match _15.0 {
0 => bb5,
1 => bb6,
2 => bb7,
905160806 => bb9,
_ => bb8
}
}
bb5 = {
_18 = (-9223372036854775808_isize);
_1 = [21934_u16,50855_u16,46623_u16,8899_u16];
_2 = [62283_u16,7252_u16,6095_u16,51104_u16];
_3 = !_13;
_4 = [30426_u16,50610_u16,9377_u16,13666_u16];
RET = (-8606204651790133178_i64) as f64;
_10 = _9 <= _3;
_15.0 = 139_i16 as u32;
_15.1 = _15.0 ^ _15.0;
_14 = [62936_u16,6953_u16,56547_u16,21924_u16];
_15 = (2262112691_u32, 3391384029_u32);
_15 = (905160806_u32, 3878252536_u32);
_13 = _7;
Goto(bb4)
}
bb6 = {
_16 = '\u{188c3}';
_13 = _15.1 <= _15.0;
_14 = [7718_u16,17781_u16,55936_u16,2931_u16];
_16 = '\u{fdb90}';
_4 = [51768_u16,31311_u16,27868_u16,50313_u16];
_14 = [25462_u16,41520_u16,32742_u16,34690_u16];
_18 = -(-9223372036854775808_isize);
Goto(bb3)
}
bb7 = {
_2 = [12913_u16,1016_u16,26201_u16,56449_u16];
_15.0 = 3147909175_u32 >> _6;
_15.0 = 2471624169_u32 - 2927677580_u32;
RET = 24635_u16 as f64;
_15 = (1662451612_u32, 1274562828_u32);
_9 = !_3;
_5 = [4858327632750225559_i64,954593202427691918_i64,624085836193392109_i64,2042087226770612771_i64,(-6948258165262555778_i64),8648199663316855803_i64,(-8558416847188686571_i64),(-3323598335925990740_i64)];
Call(_15 = fn16(_7, _9, _7, _9, _5, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_1 = [_22,_22,_22,_22];
_15.0 = !_15.1;
_25 = _6 as f32;
_12 = [_11,_11,_11,_11];
Goto(bb10)
}
bb10 = {
_5 = [_19,_19,_19,_19,_19,_19,_19,_19];
_29.fld1 = _11 + _11;
_26 = _15.0 as i32;
_11 = _25 as i8;
_15.0 = _15.1 * _15.1;
RET = _6 as f64;
_25 = _22 as f32;
_22 = 59367_u16;
_3 = _10 == _10;
_10 = !_3;
_3 = !_7;
_28 = _19 as f64;
RET = 31166_i16 as f64;
_25 = 95075215480823075085357355704211460314_u128 as f32;
match _22 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb4,
4 => bb11,
59367 => bb13,
_ => bb12
}
}
bb11 = {
_16 = '\u{188c3}';
_13 = _15.1 <= _15.0;
_14 = [7718_u16,17781_u16,55936_u16,2931_u16];
_16 = '\u{fdb90}';
_4 = [51768_u16,31311_u16,27868_u16,50313_u16];
_14 = [25462_u16,41520_u16,32742_u16,34690_u16];
_18 = -(-9223372036854775808_isize);
Goto(bb3)
}
bb12 = {
_2 = [12913_u16,1016_u16,26201_u16,56449_u16];
_15.0 = 3147909175_u32 >> _6;
_15.0 = 2471624169_u32 - 2927677580_u32;
RET = 24635_u16 as f64;
_15 = (1662451612_u32, 1274562828_u32);
_9 = !_3;
_5 = [4858327632750225559_i64,954593202427691918_i64,624085836193392109_i64,2042087226770612771_i64,(-6948258165262555778_i64),8648199663316855803_i64,(-8558416847188686571_i64),(-3323598335925990740_i64)];
Call(_15 = fn16(_7, _9, _7, _9, _5, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_6 = !93470379256929579667895982280084721502_i128;
_24 = !_7;
_4 = [_22,_22,_22,_22];
_29.fld0 = [_29.fld1,_29.fld1,_29.fld1,_29.fld1];
_15 = (956066456_u32, 83888695_u32);
_1 = [_22,_22,_22,_22];
match _18 {
0 => bb11,
1 => bb6,
340282366920938463454151235394913435648 => bb14,
_ => bb9
}
}
bb14 = {
_32 = [_16,_16,_16,_16,_16,_16,_16,_16];
RET = 189336312999521939647978693010805090015_u128 as f64;
_10 = _13 & _7;
_6 = _18 as i128;
_26 = 1015704194_i32 * 58996536_i32;
_1 = [_22,_22,_22,_22];
_15 = (3159829249_u32, 3187155805_u32);
_30 = !7_usize;
_9 = _3;
_30 = !5_usize;
_29.fld1 = _11;
_15.0 = _15.1;
_26 = !59920248_i32;
_12 = _29.fld0;
_29 = Adt45 { fld0: _12,fld1: _11 };
_1 = [_22,_22,_22,_22];
_16 = '\u{3e05c}';
_23 = _29.fld0;
RET = _26 as f64;
_23 = _29.fld0;
_36 = _18;
_23 = _12;
_1 = [_22,_22,_22,_22];
_19 = _25 as i64;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(15_usize, 10_usize, Move(_10), 14_usize, Move(_14), 15_usize, Move(_15), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(15_usize, 4_usize, Move(_4), 8_usize, Move(_8), 32_usize, Move(_32), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(15_usize, 13_usize, Move(_13), 3_usize, Move(_3), 30_usize, Move(_30), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: [i64; 8],mut _6: bool) -> (u32, u32) {
mir! {
type RET = (u32, u32);
let _7: f64;
let _8: usize;
let _9: [u16; 4];
let _10: u32;
let _11: isize;
let _12: [i8; 4];
let _13: bool;
let _14: [u16; 1];
let _15: (u32, u32);
let _16: char;
let _17: ();
let _18: ();
{
_3 = _2;
RET = (791412090_u32, 3533871584_u32);
RET.1 = RET.0 & RET.0;
_7 = 7319852327125591748_i64 as f64;
_7 = 32163_i16 as f64;
RET.1 = !RET.0;
_4 = _3;
_7 = 11153189332915190852_usize as f64;
_8 = RET.1 as usize;
RET = (3357775708_u32, 2114071994_u32);
_1 = _4 | _3;
RET.0 = RET.1 / RET.1;
RET.1 = RET.0;
_2 = _4 ^ _6;
RET.0 = 926118333124378577_u64 as u32;
RET.1 = RET.0;
RET.0 = _1 as u32;
RET = (2698040243_u32, 1386141579_u32);
_9 = [27699_u16,35869_u16,9972_u16,48213_u16];
_8 = 3_usize;
RET = (1765179763_u32, 3939271121_u32);
_9[_8] = 4213_u16;
_5 = [8379610492449668666_i64,(-5144149740758348411_i64),3284844238859722734_i64,7450158381909011017_i64,2387172858682207044_i64,5198075555522157942_i64,3678981551774030169_i64,(-8992079997751816903_i64)];
_9 = [31164_u16,49643_u16,18998_u16,11492_u16];
RET.0 = _5[_8] as u32;
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
3939271121 => bb9,
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
RET = (1997650563_u32, 214826983_u32);
_8 = 3_usize;
RET.1 = RET.0;
RET.0 = RET.1 | RET.1;
_10 = _9[_8] as u32;
RET.1 = RET.0;
_6 = !_2;
match _5[_8] {
0 => bb10,
1 => bb11,
2 => bb12,
7450158381909011017 => bb14,
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
RET.1 = !_10;
RET = (_10, _10);
_9[_8] = 9902_u16;
_11 = !(-105_isize);
_10 = !RET.1;
_9 = [11481_u16,57187_u16,55598_u16,60819_u16];
RET = (_10, _10);
RET.1 = _6 as u32;
_8 = 140674976493098434176538844434607329466_u128 as usize;
RET.0 = _10 + RET.1;
_3 = RET.0 > RET.0;
_4 = _3 <= _1;
_12 = [(-111_i8),(-30_i8),(-94_i8),(-79_i8)];
_5 = [(-772887683172975536_i64),(-8868858984475173974_i64),(-3981345209858311571_i64),7717412797058358333_i64,4166273746672178209_i64,7433401255909228928_i64,6933354141147216746_i64,(-1255884733356313727_i64)];
_15 = (RET.1, RET.0);
_14 = [42149_u16];
_15.1 = RET.0;
_7 = (-103_i8) as f64;
_15 = (RET.0, RET.1);
_15.1 = _4 as u32;
_15.1 = _15.0;
_15.0 = 8683312890190107893_u64 as u32;
_5 = [(-1064653869888482032_i64),(-2704094633345681982_i64),719094108806971221_i64,(-4427759164038451453_i64),6240573136067280078_i64,(-6422653632697703760_i64),7914576199753094080_i64,(-6100677971632466893_i64)];
_15.0 = _8 as u32;
_16 = '\u{f25c9}';
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(16_usize, 16_usize, Move(_16), 12_usize, Move(_12), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_17 = dump_var(16_usize, 4_usize, Move(_4), 15_usize, Move(_15), 2_usize, Move(_2), 18_usize, _18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *mut [bool; 1],mut _2: bool,mut _3: *mut [bool; 1],mut _4: f32,mut _5: [bool; 7],mut _6: *mut [bool; 1]) -> u16 {
mir! {
type RET = u16;
let _7: i64;
let _8: u64;
let _9: u64;
let _10: isize;
let _11: [i8; 4];
let _12: f64;
let _13: [bool; 7];
let _14: Adt57;
let _15: [i32; 6];
let _16: f32;
let _17: [u16; 4];
let _18: char;
let _19: ();
let _20: ();
{
RET = 32060_u16;
(*_1) = [_2];
_7 = (-1052084565521517820_i64) & (-8173801557802485114_i64);
RET = 57698_u16;
_5 = [_2,_2,_2,_2,_2,_2,_2];
_8 = 8267554966904533714_u64 - 1660090671489792065_u64;
_9 = _8;
_10 = RET as isize;
RET = _2 as u16;
_1 = core::ptr::addr_of_mut!((*_3));
_12 = _7 as f64;
RET = 51919_u16;
_4 = _8 as f32;
(*_6) = [_2];
(*_1) = [_2];
_10 = 3207617729_u32 as isize;
_11 = [(-73_i8),3_i8,(-113_i8),(-39_i8)];
_9 = !_8;
Goto(bb1)
}
bb1 = {
_4 = 54689868818907524339981341133680833627_i128 as f32;
(*_6) = [_2];
_6 = core::ptr::addr_of_mut!((*_6));
_15 = [(-1617380153_i32),1555765333_i32,(-913361943_i32),1877152030_i32,1868143895_i32,(-489820404_i32)];
_3 = core::ptr::addr_of_mut!((*_1));
(*_3) = [_2];
(*_3) = [_2];
(*_6) = [_2];
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
51919 => bb10,
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
(*_3) = [_2];
_11 = [(-117_i8),96_i8,37_i8,48_i8];
_11 = [103_i8,70_i8,(-12_i8),(-93_i8)];
(*_6) = [_2];
_12 = 151923040616181749157816255582775810491_i128 as f64;
_7 = -(-8864152773259892791_i64);
_11 = [(-85_i8),99_i8,(-66_i8),(-66_i8)];
match RET {
0 => bb4,
1 => bb11,
2 => bb12,
51919 => bb14,
_ => bb13
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
_12 = 1_usize as f64;
RET = 41428_u16 & 23794_u16;
_1 = core::ptr::addr_of_mut!((*_1));
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(17_usize, 10_usize, Move(_10), 8_usize, Move(_8), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: bool,mut _2: f32,mut _3: bool,mut _4: isize,mut _5: u128) -> u32 {
mir! {
type RET = u32;
let _6: u16;
let _7: i32;
let _8: f32;
let _9: f64;
let _10: i128;
let _11: [bool; 7];
let _12: [i32; 6];
let _13: isize;
let _14: [i8; 4];
let _15: [bool; 1];
let _16: Adt55;
let _17: Adt45;
let _18: [i8; 4];
let _19: i8;
let _20: char;
let _21: Adt46;
let _22: char;
let _23: isize;
let _24: [u16; 4];
let _25: f64;
let _26: u32;
let _27: isize;
let _28: u32;
let _29: i128;
let _30: [bool; 1];
let _31: [u16; 4];
let _32: (f32, (*const u64, [i32; 6], (u16, bool), [i32; 6]), *const f32, *mut [bool; 1]);
let _33: Adt53;
let _34: char;
let _35: u64;
let _36: u64;
let _37: Adt56;
let _38: i64;
let _39: [i32; 6];
let _40: bool;
let _41: i64;
let _42: [char; 8];
let _43: u32;
let _44: i32;
let _45: isize;
let _46: f32;
let _47: ();
let _48: ();
{
RET = 3121283919_u32;
_5 = 132282113454989358433836368138115747762_u128 * 240078417497545038248620475281504032904_u128;
RET = !3496690465_u32;
_1 = _3;
_7 = !(-235436483_i32);
_6 = 13508_u16;
_1 = _3;
RET = 2795070373_u32;
_2 = RET as f32;
RET = !4282975604_u32;
RET = _6 as u32;
_6 = 59351_u16;
RET = 1022427223_u32;
_7 = -1601370889_i32;
_6 = !24810_u16;
_1 = _3;
_2 = _5 as f32;
_3 = _1 == _1;
Goto(bb1)
}
bb1 = {
_9 = RET as f64;
_6 = 19427_u16;
_9 = _2 as f64;
_9 = 3485155261905275396_i64 as f64;
_7 = 7311861833659646159_i64 as i32;
_6 = 57259_u16 + 47282_u16;
_1 = !_3;
_11 = [_3,_3,_1,_1,_3,_3,_1];
_5 = _4 as u128;
_8 = 9_u8 as f32;
RET = 4051582367_u32;
_3 = _1;
Goto(bb2)
}
bb2 = {
_11 = [_1,_1,_3,_3,_3,_3,_1];
_10 = !(-64322829646519696061153277837205933947_i128);
_11 = [_1,_3,_3,_1,_3,_1,_1];
_5 = RET as u128;
_9 = _2 as f64;
_2 = -_8;
_10 = 137801662379805224735531248109762622147_i128;
_1 = _3 | _3;
_5 = 145_u8 as u128;
_4 = !59_isize;
_3 = _1 & _1;
_9 = (-108_i8) as f64;
RET = _5 as u32;
_5 = 240402495805685166796354589778182558150_u128;
RET = 1098685327_u32;
_4 = -101_isize;
_7 = (-2773260561018317114_i64) as i32;
_9 = _7 as f64;
_13 = !_4;
_7 = 1722657540_i32;
_13 = -_4;
_12 = [_7,_7,_7,_7,_7,_7];
_14 = [(-38_i8),57_i8,15_i8,79_i8];
Goto(bb3)
}
bb3 = {
RET = _9 as u32;
_2 = (-102_i8) as f32;
_3 = _1 ^ _1;
_10 = _7 as i128;
_5 = !316599274075346658679228252606651732621_u128;
_9 = 28_i8 as f64;
_4 = _13 >> _5;
_4 = _13 << _6;
_1 = _3;
_10 = (-126543648031389988211411310032951037592_i128) | 49055601111947093121724505964678074045_i128;
_15 = [_3];
RET = 100_u8 as u32;
_9 = 31164_i16 as f64;
_8 = _2 + _2;
Call(_4 = fn19(_15, _3, _11, _15, _1, _11, _1, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = [_1];
_9 = 1483_i16 as f64;
_4 = RET as isize;
Goto(bb5)
}
bb5 = {
_23 = -_13;
_5 = !113129380884043582514720510660675618492_u128;
_11 = [_3,_3,_3,_3,_3,_3,_1];
_18 = _14;
_14 = _18;
_20 = '\u{95efa}';
_17 = Adt45 { fld0: _14,fld1: (-47_i8) };
_2 = _10 as f32;
_17.fld0 = _18;
_11 = [_1,_1,_1,_1,_3,_3,_1];
_17.fld1 = (-119_i8) & (-54_i8);
_24 = [_6,_6,_6,_6];
_20 = '\u{14358}';
RET = 3290746870_u32;
_26 = 3_usize as u32;
_4 = _13 << _13;
_24 = [_6,_6,_6,_6];
_1 = !_3;
_1 = !_3;
_27 = -_13;
_28 = RET;
RET = _26;
_9 = _5 as f64;
_19 = _17.fld1;
_7 = 895970643_i32;
_25 = _9 + _9;
match _28 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
3290746870 => bb7,
_ => bb6
}
}
bb6 = {
RET = _9 as u32;
_2 = (-102_i8) as f32;
_3 = _1 ^ _1;
_10 = _7 as i128;
_5 = !316599274075346658679228252606651732621_u128;
_9 = 28_i8 as f64;
_4 = _13 >> _5;
_4 = _13 << _6;
_1 = _3;
_10 = (-126543648031389988211411310032951037592_i128) | 49055601111947093121724505964678074045_i128;
_15 = [_3];
RET = 100_u8 as u32;
_9 = 31164_i16 as f64;
_8 = _2 + _2;
Call(_4 = fn19(_15, _3, _11, _15, _1, _11, _1, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb7 = {
_7 = !216824189_i32;
Goto(bb8)
}
bb8 = {
_28 = RET;
_22 = _20;
_18 = [_19,_17.fld1,_17.fld1,_19];
_13 = !_27;
_11 = [_3,_3,_3,_1,_3,_1,_1];
_11 = [_3,_3,_3,_3,_1,_1,_3];
_1 = _3 & _3;
_22 = _20;
_32.1.1 = [_7,_7,_7,_7,_7,_7];
_22 = _20;
_27 = !_4;
_30 = _15;
_32.0 = _25 as f32;
_32.3 = core::ptr::addr_of_mut!(_15);
_32.1.2 = (_6, _3);
_17 = Adt45 { fld0: _18,fld1: _19 };
_17.fld0 = _14;
Goto(bb9)
}
bb9 = {
_9 = _25;
_7 = 851961409_i32;
_34 = _20;
_32.1.3 = _32.1.1;
_31 = _24;
_32.1.3 = _12;
_32.2 = core::ptr::addr_of!(_8);
_32.1.2.1 = !_1;
_17 = Adt45 { fld0: _14,fld1: _19 };
_13 = _4;
_8 = _32.0;
_24 = _31;
_17.fld1 = _19 - _19;
_3 = _32.1.2.1 > _32.1.2.1;
_29 = _7 as i128;
Goto(bb10)
}
bb10 = {
_13 = -_23;
Call(_9 = core::intrinsics::transmute(_4), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_8 = _32.0;
_4 = _27 | _23;
_17.fld0 = [_17.fld1,_19,_17.fld1,_19];
_8 = _2;
_24 = _31;
_32.1.2 = Checked(_6 + _6);
_3 = !_1;
_26 = !RET;
_11 = [_3,_3,_1,_3,_3,_3,_1];
_1 = !_3;
_42 = [_34,_20,_22,_20,_34,_34,_20,_22];
_37.fld5.1 = _2 * _2;
_25 = _7 as f64;
_32.1.1 = [_7,_7,_7,_7,_7,_7];
_11 = [_1,_1,_1,_1,_3,_3,_3];
_32.1.1 = _32.1.3;
_32.3 = core::ptr::addr_of_mut!(_15);
_35 = 6548560091545886633_u64;
_7 = (-1534833097_i32);
_32.1.0 = core::ptr::addr_of!(_35);
match _35 {
0 => bb12,
1 => bb13,
6548560091545886633 => bb15,
_ => bb14
}
}
bb12 = {
_11 = [_1,_1,_3,_3,_3,_3,_1];
_10 = !(-64322829646519696061153277837205933947_i128);
_11 = [_1,_3,_3,_1,_3,_1,_1];
_5 = RET as u128;
_9 = _2 as f64;
_2 = -_8;
_10 = 137801662379805224735531248109762622147_i128;
_1 = _3 | _3;
_5 = 145_u8 as u128;
_4 = !59_isize;
_3 = _1 & _1;
_9 = (-108_i8) as f64;
RET = _5 as u32;
_5 = 240402495805685166796354589778182558150_u128;
RET = 1098685327_u32;
_4 = -101_isize;
_7 = (-2773260561018317114_i64) as i32;
_9 = _7 as f64;
_13 = !_4;
_7 = 1722657540_i32;
_13 = -_4;
_12 = [_7,_7,_7,_7,_7,_7];
_14 = [(-38_i8),57_i8,15_i8,79_i8];
Goto(bb3)
}
bb13 = {
RET = _9 as u32;
_2 = (-102_i8) as f32;
_3 = _1 ^ _1;
_10 = _7 as i128;
_5 = !316599274075346658679228252606651732621_u128;
_9 = 28_i8 as f64;
_4 = _13 >> _5;
_4 = _13 << _6;
_1 = _3;
_10 = (-126543648031389988211411310032951037592_i128) | 49055601111947093121724505964678074045_i128;
_15 = [_3];
RET = 100_u8 as u32;
_9 = 31164_i16 as f64;
_8 = _2 + _2;
Call(_4 = fn19(_15, _3, _11, _15, _1, _11, _1, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_28 = RET;
_22 = _20;
_18 = [_19,_17.fld1,_17.fld1,_19];
_13 = !_27;
_11 = [_3,_3,_3,_1,_3,_1,_1];
_11 = [_3,_3,_3,_3,_1,_1,_3];
_1 = _3 & _3;
_22 = _20;
_32.1.1 = [_7,_7,_7,_7,_7,_7];
_22 = _20;
_27 = !_4;
_30 = _15;
_32.0 = _25 as f32;
_32.3 = core::ptr::addr_of_mut!(_15);
_32.1.2 = (_6, _3);
_17 = Adt45 { fld0: _18,fld1: _19 };
_17.fld0 = _14;
Goto(bb9)
}
bb15 = {
_41 = -(-3442023443441246791_i64);
_32.3 = core::ptr::addr_of_mut!(_15);
_10 = _29 + _29;
_20 = _34;
_44 = _7;
_7 = !_44;
_37.fld2 = [_32.1.2.0,_6,_32.1.2.0,_32.1.2.0];
_32.1.2.1 = !_3;
_11 = [_3,_3,_1,_3,_3,_32.1.2.1,_32.1.2.1];
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(18_usize, 12_usize, Move(_12), 19_usize, Move(_19), 41_usize, Move(_41), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(18_usize, 14_usize, Move(_14), 34_usize, Move(_34), 42_usize, Move(_42), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(18_usize, 28_usize, Move(_28), 1_usize, Move(_1), 15_usize, Move(_15), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_47 = dump_var(18_usize, 23_usize, Move(_23), 27_usize, Move(_27), 48_usize, _48, 48_usize, _48), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [bool; 1],mut _2: bool,mut _3: [bool; 7],mut _4: [bool; 1],mut _5: bool,mut _6: [bool; 7],mut _7: bool,mut _8: [bool; 1]) -> isize {
mir! {
type RET = isize;
let _9: Adt45;
let _10: f32;
let _11: u16;
let _12: (u16, bool);
let _13: (*const u64, [i32; 6], (u16, bool), [i32; 6]);
let _14: [u16; 4];
let _15: [bool; 1];
let _16: usize;
let _17: [i32; 6];
let _18: bool;
let _19: char;
let _20: &'static char;
let _21: ((u16, bool), char, i16, char);
let _22: isize;
let _23: Adt47;
let _24: usize;
let _25: ();
let _26: ();
{
_1 = _8;
RET = (-9223372036854775808_isize) >> 9223372036854775807_isize;
_7 = _2;
_9.fld0 = [22_i8,6_i8,91_i8,(-80_i8)];
_10 = (-1111805908_i32) as f32;
RET = 24_isize;
RET = (-9223372036854775808_isize);
_2 = _5;
_7 = _2 >= _5;
_6 = [_7,_2,_2,_5,_7,_7,_5];
_5 = _2;
_8 = _1;
_8 = [_2];
_12.0 = _5 as u16;
_6 = _3;
_13.1 = [(-12840158_i32),1605864353_i32,(-595812975_i32),(-1152344546_i32),(-1310045113_i32),(-1244385337_i32)];
_3 = [_5,_5,_7,_7,_2,_5,_7];
_13.2.0 = _12.0;
_4 = [_2];
_12.1 = _7;
_12.0 = _13.2.0 - _13.2.0;
_1 = [_2];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
_ => bb5
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
_13.2.0 = _12.0 & _12.0;
_13.2.1 = _5;
_5 = !_7;
RET = 81_isize;
_14 = [_12.0,_12.0,_12.0,_13.2.0];
_10 = 185_u8 as f32;
_6 = _3;
_11 = _12.0;
_8 = [_2];
_2 = _11 != _11;
_11 = _12.0;
_1 = [_13.2.1];
_12.0 = _11 << _11;
_13.2.0 = !_12.0;
_12.0 = !_13.2.0;
RET = -(-69_isize);
_4 = _1;
_4 = _1;
_2 = !_7;
_8 = [_5];
RET = (-98_isize);
Goto(bb7)
}
bb7 = {
_18 = _12.1 >= _2;
_15 = [_13.2.1];
_12.0 = _10 as u16;
_12.1 = _13.2.1 & _13.2.1;
_9.fld1 = 80_i8;
_17 = _13.1;
_12 = Checked(_13.2.0 - _11);
_20 = &_19;
_12.0 = 8218524913498689590_i64 as u16;
_13.2.0 = _13.2.1 as u16;
_16 = 221111688491142218938299651865882101891_u128 as usize;
_16 = _10 as usize;
_9.fld0 = [_9.fld1,_9.fld1,_9.fld1,_9.fld1];
_14 = [_11,_13.2.0,_13.2.0,_11];
_6 = [_2,_5,_13.2.1,_5,_12.1,_12.1,_2];
RET = 9223372036854775807_isize;
_7 = _18 > _12.1;
_15 = [_13.2.1];
_17 = _13.1;
_14 = [_13.2.0,_13.2.0,_13.2.0,_11];
_13.3 = [(-1073828530_i32),1451923880_i32,(-1064604277_i32),(-1180052424_i32),(-1994560627_i32),(-1854622901_i32)];
_20 = &(*_20);
RET = _18 as isize;
_12 = Checked(_13.2.0 - _13.2.0);
_6 = [_13.2.1,_12.1,_2,_2,_2,_13.2.1,_7];
Goto(bb8)
}
bb8 = {
Call(_25 = dump_var(19_usize, 8_usize, Move(_8), 2_usize, Move(_2), 15_usize, Move(_15), 5_usize, Move(_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_25 = dump_var(19_usize, 4_usize, Move(_4), 16_usize, Move(_16), 14_usize, Move(_14), 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(110356291807882727968396853526060973954_u128), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(3_i8), std::hint::black_box(2219574505_u32), std::hint::black_box(1860744575_i32), std::hint::black_box(12076518668761418737_u64), std::hint::black_box(121462566065578196987832115166089358538_i128), std::hint::black_box(15895_u16));
                
            }
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: (u32, u32),
fld1: [i32; 6],
fld2: [i8; 4],
fld3: *const f32,
fld4: u32,
fld5: [u16; 4],
fld6: i64,
fld7: [char; 8],

},
Variant1{
fld0: [usize; 1],
fld1: u64,
fld2: f64,
fld3: i8,
fld4: (*const u64, [i32; 6], (u16, bool), [i32; 6]),
fld5: i128,
fld6: usize,

},
Variant2{
fld0: (u16, bool),
fld1: *mut ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),
fld2: (f32, (*const u64, [i32; 6], (u16, bool), [i32; 6]), *const f32, *mut [bool; 1]),
fld3: *const i16,
fld4: u16,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: [i8; 4],
fld1: i8,
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u8,
fld1: Adt44,
fld2: usize,

},
Variant1{
fld0: u64,
fld1: Adt44,
fld2: isize,
fld3: i8,
fld4: u32,
fld5: [u16; 4],

},
Variant2{
fld0: (*const u64, f32),
fld1: [i32; 6],
fld2: f32,
fld3: i8,
fld4: u64,
fld5: (u16, bool),
fld6: [usize; 1],
fld7: (*mut [bool; 1],),

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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (f32, (*const u64, [i32; 6], (u16, bool), [i32; 6]), *const f32, *mut [bool; 1]),
fld1: (u16, bool),
fld2: *mut ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),

},
Variant1{
fld0: u64,
fld1: [bool; 7],
fld2: [i32; 6],
fld3: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),
fld4: (u32, u32),

},
Variant2{
fld0: *const char,
fld1: [char; 8],
fld2: isize,
fld3: f64,
fld4: i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),
fld1: *mut [bool; 1],
fld2: [i8; 4],
fld3: i64,

},
Variant1{
fld0: (*mut [bool; 1],),
fld1: Adt47,

},
Variant2{
fld0: Adt47,
fld1: f64,

},
Variant3{
fld0: Adt47,
fld1: [i32; 6],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: u128,
fld1: (*mut [bool; 1],),
fld2: u16,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: usize,
fld1: Adt48,
fld2: f32,
fld3: [usize; 1],
fld4: i16,
fld5: i32,
fld6: (f32, (*const u64, [i32; 6], (u16, bool), [i32; 6]), *const f32, *mut [bool; 1]),
fld7: (*const u64, [i32; 6], (u16, bool), [i32; 6]),

},
Variant1{
fld0: *const u64,
fld1: Adt46,
fld2: Adt49,

},
Variant2{
fld0: i32,

},
Variant3{
fld0: *const u64,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: Adt46,
fld2: isize,
fld3: Adt44,
fld4: [char; 8],
fld5: Adt49,
fld6: (*const u64, f32),
fld7: [u16; 4],

},
Variant1{
fld0: u16,
fld1: i128,
fld2: i64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const i16,
fld1: i128,
fld2: isize,
fld3: Adt44,
fld4: (u16, bool),

},
Variant1{
fld0: u8,
fld1: [i64; 8],
fld2: isize,
fld3: [u16; 4],
fld4: [i32; 6],
fld5: Adt48,
fld6: ((u16, bool), char, i16, char),
fld7: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),

},
Variant2{
fld0: [bool; 7],
fld1: (*const u64, f32),
fld2: isize,

},
Variant3{
fld0: *const i16,
fld1: u16,
fld2: Adt47,

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
fld0: bool,
fld1: ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),
fld2: [u16; 1],
fld3: *const char,

},
Variant1{
fld0: Adt48,
fld1: u32,
fld2: *const u64,
fld3: f32,
fld4: i16,

},
Variant2{
fld0: (*mut [bool; 1],),
fld1: f64,
fld2: [bool; 1],
fld3: Adt52,
fld4: f32,
fld5: u128,
fld6: *const f32,
fld7: (u16, bool),

},
Variant3{
fld0: i128,
fld1: char,
fld2: Adt50,
fld3: u64,
fld4: *const char,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: (*const u64, f32),
fld1: i64,
fld2: [usize; 1],

},
Variant1{
fld0: u128,
fld1: Adt52,
fld2: (*mut [bool; 1],),

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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt52,
fld1: [u16; 1],
fld2: isize,

},
Variant1{
fld0: Adt47,
fld1: [u16; 4],
fld2: Adt50,
fld3: (u16, bool),
fld4: u32,
fld5: (*mut [bool; 1],),

},
Variant2{
fld0: *const i16,
fld1: f64,

},
Variant3{
fld0: Adt47,
fld1: f32,
fld2: [u16; 4],
fld3: u128,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: Adt48,
fld1: Adt52,
fld2: [u16; 4],
fld3: [u16; 1],
fld4: [i8; 4],
fld5: (*const u64, f32),
fld6: [bool; 1],
}
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (u32, u32),
fld1: i64,
fld2: [i8; 4],
fld3: *mut ([bool; 7], f32, u128, *mut [bool; 1], [i32; 6], *mut [bool; 1]),
fld4: u32,

},
Variant1{
fld0: usize,
fld1: (*mut [bool; 1],),
fld2: [i8; 4],
fld3: u16,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
fld0: [u16; 1],
fld1: (*const u64, f32),

},
Variant1{
fld0: (*const u64, [i32; 6], (u16, bool), [i32; 6]),
fld1: char,
fld2: Adt50,
fld3: Adt56,
fld4: (*const u64, f32),

},
Variant2{
fld0: [usize; 1],
fld1: char,
fld2: u8,
fld3: Adt48,
fld4: Adt54,
fld5: (u16, bool),

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: bool,
fld1: u128,
fld2: (*const u64, f32),
fld3: (u16, bool),
fld4: i16,
fld5: f32,

},
Variant1{
fld0: u32,

},
Variant2{
fld0: Adt58,
fld1: (*const u64, f32),
fld2: *const u64,

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
fld0: (f32, (*const u64, [i32; 6], (u16, bool), [i32; 6]), *const f32, *mut [bool; 1]),
fld1: Adt56,
fld2: isize,
fld3: [usize; 1],
fld4: [i32; 6],

},
Variant1{
fld0: u16,
fld1: Adt55,
fld2: [u16; 1],
fld3: [i8; 4],
fld4: Adt50,

}}

