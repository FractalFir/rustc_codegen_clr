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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> f64 {
mir! {
type RET = f64;
let _15: [isize; 3];
let _16: *mut u32;
let _17: Adt40;
let _18: [char; 6];
let _19: bool;
let _20: f32;
let _21: u8;
let _22: isize;
let _23: i16;
let _24: i32;
let _25: *mut u16;
let _26: [isize; 3];
let _27: [u8; 5];
let _28: *mut char;
let _29: u8;
let _30: ();
let _31: ();
{
_8 = 10324211985281501421514187062560435664_i128 | (-156838501259783800997626123554773136687_i128);
_5 = !3502_i16;
_4 = -(-118_i8);
Call(_12 = fn1(_4, _8, _5, _4, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = true;
_2 = '\u{9619b}';
_1 = false | false;
_13 = 4813165413516119047_u64 & 4818052741035317469_u64;
_7 = 8986149474201570702_i64 & (-2870064009542016050_i64);
_7 = 7327949309957474973_i64 + 3692772786915436430_i64;
_5 = !(-372_i16);
RET = _5 as f64;
_9 = 16875916958099159903_usize + 1_usize;
_10 = 219_u8;
_14 = !110715377241971909131658115956227416113_u128;
_8 = 69174937857871614005704017248557152195_i128;
_5 = 33438_u16 as i16;
_6 = !348583860_i32;
_3 = 49_isize;
_10 = !102_u8;
_9 = 6_usize;
_11 = 19269_u16 * 60130_u16;
_13 = _9 as u64;
RET = _7 as f64;
_11 = _13 as u16;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
69174937857871614005704017248557152195 => bb6,
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
_14 = _5 as u128;
_2 = '\u{95f30}';
_10 = !22_u8;
_15 = [_3,_3,_3];
_9 = 2_usize;
_14 = 144490783182203993702572889933204810296_u128 | 107707030677531834038144898127907684974_u128;
_2 = '\u{33894}';
_4 = !64_i8;
_7 = _5 as i64;
_17.fld4 = core::ptr::addr_of!(_17.fld1.1.2);
_15[_9] = RET as isize;
_17.fld1.1.1 = -_7;
_17.fld0.0 = core::ptr::addr_of!(_17.fld1.1.1);
_13 = _3 as u64;
_17.fld1.3 = RET;
_4 = _17.fld1.1.1 as i8;
_17.fld1.1.1 = _7;
_3 = _15[_9];
_17.fld3 = _4 << _12;
_18[_9] = _2;
Goto(bb7)
}
bb7 = {
_16 = core::ptr::addr_of_mut!(_17.fld1.1.3);
_17.fld1.1.4 = _11 | _11;
_1 = false | false;
_20 = _13 as f32;
_17.fld3 = _4;
_3 = -_15[_9];
_17.fld0.0 = core::ptr::addr_of!(_17.fld1.1.1);
_12 = 2675299894_u32;
match _9 {
0 => bb8,
2 => bb10,
_ => bb9
}
}
bb8 = {
_14 = _5 as u128;
_2 = '\u{95f30}';
_10 = !22_u8;
_15 = [_3,_3,_3];
_9 = 2_usize;
_14 = 144490783182203993702572889933204810296_u128 | 107707030677531834038144898127907684974_u128;
_2 = '\u{33894}';
_4 = !64_i8;
_7 = _5 as i64;
_17.fld4 = core::ptr::addr_of!(_17.fld1.1.2);
_15[_9] = RET as isize;
_17.fld1.1.1 = -_7;
_17.fld0.0 = core::ptr::addr_of!(_17.fld1.1.1);
_13 = _3 as u64;
_17.fld1.3 = RET;
_4 = _17.fld1.1.1 as i8;
_17.fld1.1.1 = _7;
_3 = _15[_9];
_17.fld3 = _4 << _12;
_18[_9] = _2;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
_17.fld1.1.5 = _20;
RET = _17.fld1.3;
_6 = _7 as i32;
_13 = 15353307286015987638_u64;
(*_16) = _12 / _12;
_11 = _13 as u16;
_17.fld1.2 = _17.fld0.0;
_17.fld1.1.2 = _7 * _7;
_12 = !(*_16);
Goto(bb11)
}
bb11 = {
_17.fld1.0[_9] = !_17.fld1.1.3;
_14 = 272815799083279735806996711026089992487_u128 + 299273712217937923060118510541836220948_u128;
_17.fld1.0 = [_12,(*_16),_12,_12];
_11 = _17.fld3 as u16;
RET = _17.fld1.3 * _17.fld1.3;
_6 = 946802647_i32;
_17.fld0 = (_17.fld4, _20);
_12 = !(*_16);
_14 = 10398851546383164375294855916624580689_u128 >> _17.fld1.1.4;
_13 = !8660620289522311641_u64;
_17.fld5 = _1 as u16;
_27 = [_10,_10,_10,_10,_10];
_4 = _17.fld3 + _17.fld3;
_1 = false;
_3 = _15[_9] + _15[_9];
_15 = [_3,_3,_3];
_26[_9] = _15[_9];
_18[_9] = _2;
_21 = _27[_9] | _27[_9];
_17.fld1.4 = _17.fld3 | _4;
_17.fld0.1 = _20;
_24 = _6;
match _6 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
946802647 => bb18,
_ => bb17
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_14 = _5 as u128;
_2 = '\u{95f30}';
_10 = !22_u8;
_15 = [_3,_3,_3];
_9 = 2_usize;
_14 = 144490783182203993702572889933204810296_u128 | 107707030677531834038144898127907684974_u128;
_2 = '\u{33894}';
_4 = !64_i8;
_7 = _5 as i64;
_17.fld4 = core::ptr::addr_of!(_17.fld1.1.2);
_15[_9] = RET as isize;
_17.fld1.1.1 = -_7;
_17.fld0.0 = core::ptr::addr_of!(_17.fld1.1.1);
_13 = _3 as u64;
_17.fld1.3 = RET;
_4 = _17.fld1.1.1 as i8;
_17.fld1.1.1 = _7;
_3 = _15[_9];
_17.fld3 = _4 << _12;
_18[_9] = _2;
Goto(bb7)
}
bb15 = {
_16 = core::ptr::addr_of_mut!(_17.fld1.1.3);
_17.fld1.1.4 = _11 | _11;
_1 = false | false;
_20 = _13 as f32;
_17.fld3 = _4;
_3 = -_15[_9];
_17.fld0.0 = core::ptr::addr_of!(_17.fld1.1.1);
_12 = 2675299894_u32;
match _9 {
0 => bb8,
2 => bb10,
_ => bb9
}
}
bb16 = {
Return()
}
bb17 = {
_1 = true;
_2 = '\u{9619b}';
_1 = false | false;
_13 = 4813165413516119047_u64 & 4818052741035317469_u64;
_7 = 8986149474201570702_i64 & (-2870064009542016050_i64);
_7 = 7327949309957474973_i64 + 3692772786915436430_i64;
_5 = !(-372_i16);
RET = _5 as f64;
_9 = 16875916958099159903_usize + 1_usize;
_10 = 219_u8;
_14 = !110715377241971909131658115956227416113_u128;
_8 = 69174937857871614005704017248557152195_i128;
_5 = 33438_u16 as i16;
_6 = !348583860_i32;
_3 = 49_isize;
_10 = !102_u8;
_9 = 6_usize;
_11 = 19269_u16 * 60130_u16;
_13 = _9 as u64;
RET = _7 as f64;
_11 = _13 as u16;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
69174937857871614005704017248557152195 => bb6,
_ => bb5
}
}
bb18 = {
_23 = _5 ^ _5;
_5 = _23;
_16 = core::ptr::addr_of_mut!(_17.fld2);
_18[_9] = _2;
_17.fld1.1.4 = _17.fld5 | _17.fld5;
_17.fld0 = (_17.fld4, _20);
_12 = _17.fld1.0[_9];
_27 = [_21,_21,_10,_10,_21];
_17.fld1.1 = (_23, _7, _7, _17.fld1.0[_9], _17.fld5, _17.fld0.1);
_20 = _17.fld0.1;
_18[_9] = _2;
Goto(bb19)
}
bb19 = {
Call(_30 = dump_var(0_usize, 8_usize, Move(_8), 11_usize, Move(_11), 14_usize, Move(_14), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(0_usize, 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4), 27_usize, Move(_27)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_30 = dump_var(0_usize, 3_usize, Move(_3), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8,mut _2: i128,mut _3: i16,mut _4: i8,mut _5: i128) -> u32 {
mir! {
type RET = u32;
let _6: Adt48;
let _7: Adt39;
let _8: (*const i64, f32);
let _9: u128;
let _10: bool;
let _11: Adt53;
let _12: (i16, i64, i64, u32, u16, f32);
let _13: i8;
let _14: u64;
let _15: char;
let _16: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _17: Adt39;
let _18: bool;
let _19: bool;
let _20: f32;
let _21: [char; 5];
let _22: [u8; 5];
let _23: isize;
let _24: u128;
let _25: i64;
let _26: [char; 6];
let _27: [char; 5];
let _28: [char; 5];
let _29: (*const i64, f32);
let _30: [char; 5];
let _31: Adt46;
let _32: i16;
let _33: [char; 5];
let _34: Adt40;
let _35: ();
let _36: ();
{
RET = 3598001925_u32;
_3 = true as i16;
_1 = _3 as i8;
_2 = _5;
_3 = !25789_i16;
_5 = -_2;
_6 = Adt48::Variant2 { fld0: 13796735467861900162_u64 };
_4 = '\u{ff325}' as i8;
_3 = 25535_i16 | (-23686_i16);
_1 = !_4;
place!(Field::<u64>(Variant(_6, 2), 0)) = 5298611556670939536_i64 as u64;
_10 = _2 < _2;
_8.1 = 93_u8 as f32;
Goto(bb1)
}
bb1 = {
_2 = _5;
_3 = (-24920_i16);
_4 = -_1;
_10 = !false;
place!(Field::<u64>(Variant(_6, 2), 0)) = !9645871546197686635_u64;
SetDiscriminant(_6, 2);
place!(Field::<u64>(Variant(_6, 2), 0)) = !7324679225781929661_u64;
_12.1 = 6969540466774468279_i64 ^ (-3010080160194401727_i64);
_8.0 = core::ptr::addr_of!(_12.1);
_12.2 = !_12.1;
Call(_12.1 = fn2(Move(_6), _5, _5, _5, _8.1, _8, _5, _10, _8, _2, _8.1, _8.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !2249023984_u32;
_12.0 = '\u{67b39}' as i16;
_3 = _12.0 >> _1;
_12.5 = _8.1 + _8.1;
_12.3 = !RET;
_14 = 16561410899850528654_u64;
_1 = _4;
_10 = true | true;
_9 = !289311561276800824297426790150529012109_u128;
_12.2 = _12.1;
_10 = !false;
_16.5.0 = _3;
_16.0.3 = RET;
_16.0.5 = _8.1 * _12.5;
_9 = 96258008970971930997777315282523303166_u128;
_5 = _2 >> _3;
_12 = (_3, (-6230587845540898433_i64), 7003161803687958196_i64, RET, 12311_u16, _16.0.5);
_16.0.3 = RET;
_2 = _5;
_10 = !false;
_16.0.4 = _12.4;
_8.0 = core::ptr::addr_of!(_16.0.1);
RET = !_12.3;
match _12.4 {
0 => bb3,
1 => bb4,
12311 => bb6,
_ => bb5
}
}
bb3 = {
_2 = _5;
_3 = (-24920_i16);
_4 = -_1;
_10 = !false;
place!(Field::<u64>(Variant(_6, 2), 0)) = !9645871546197686635_u64;
SetDiscriminant(_6, 2);
place!(Field::<u64>(Variant(_6, 2), 0)) = !7324679225781929661_u64;
_12.1 = 6969540466774468279_i64 ^ (-3010080160194401727_i64);
_8.0 = core::ptr::addr_of!(_12.1);
_12.2 = !_12.1;
Call(_12.1 = fn2(Move(_6), _5, _5, _5, _8.1, _8, _5, _10, _8, _2, _8.1, _8.0), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_16.4 = _9 as f64;
_16.0.0 = _12.0;
_16.5.1 = _12.2;
_16.5.2 = _16.5.1;
_16.4 = _4 as f64;
_16.0.4 = _12.4 + _12.4;
_16.0.1 = _12.2;
_13 = !_4;
_16.0.1 = _12.1;
_10 = _12.4 < _12.4;
_16.4 = _16.0.4 as f64;
_12.3 = !RET;
_19 = _10;
_16.1 = 6_u8 * 216_u8;
_16.5.5 = _14 as f32;
_16.0.4 = !_12.4;
_16.6 = '\u{cfe1e}' as u8;
_16.5 = (_12.0, _12.2, _12.1, _12.3, _12.4, _12.5);
_16.5.4 = _12.4;
_16.5 = (_3, _16.0.1, _16.0.1, _16.0.3, _12.4, _16.0.5);
Goto(bb7)
}
bb7 = {
_16.5 = (_16.0.0, _12.1, _12.1, _16.0.3, _16.0.4, _12.5);
_20 = -_16.0.5;
RET = !_12.3;
_12.4 = _16.0.4;
_16.0.0 = _16.5.0 | _16.5.0;
RET = _16.0.3;
_18 = _10 & _10;
_16.3 = [RET,_16.5.3,_16.0.3,_16.0.3];
_10 = !_19;
_9 = _16.5.0 as u128;
_16.0 = (_16.5.0, _16.5.2, _12.1, _16.5.3, _12.4, _20);
_23 = 27_isize;
_15 = '\u{56fa1}';
_12.5 = -_16.0.5;
_16.5.1 = _16.4 as i64;
match _12.2 {
0 => bb6,
1 => bb8,
2 => bb9,
3 => bb10,
7003161803687958196 => bb12,
_ => bb11
}
}
bb8 = {
_16.4 = _9 as f64;
_16.0.0 = _12.0;
_16.5.1 = _12.2;
_16.5.2 = _16.5.1;
_16.4 = _4 as f64;
_16.0.4 = _12.4 + _12.4;
_16.0.1 = _12.2;
_13 = !_4;
_16.0.1 = _12.1;
_10 = _12.4 < _12.4;
_16.4 = _16.0.4 as f64;
_12.3 = !RET;
_19 = _10;
_16.1 = 6_u8 * 216_u8;
_16.5.5 = _14 as f32;
_16.0.4 = !_12.4;
_16.6 = '\u{cfe1e}' as u8;
_16.5 = (_12.0, _12.2, _12.1, _12.3, _12.4, _12.5);
_16.5.4 = _12.4;
_16.5 = (_3, _16.0.1, _16.0.1, _16.0.3, _12.4, _16.0.5);
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
RET = !2249023984_u32;
_12.0 = '\u{67b39}' as i16;
_3 = _12.0 >> _1;
_12.5 = _8.1 + _8.1;
_12.3 = !RET;
_14 = 16561410899850528654_u64;
_1 = _4;
_10 = true | true;
_9 = !289311561276800824297426790150529012109_u128;
_12.2 = _12.1;
_10 = !false;
_16.5.0 = _3;
_16.0.3 = RET;
_16.0.5 = _8.1 * _12.5;
_9 = 96258008970971930997777315282523303166_u128;
_5 = _2 >> _3;
_12 = (_3, (-6230587845540898433_i64), 7003161803687958196_i64, RET, 12311_u16, _16.0.5);
_16.0.3 = RET;
_2 = _5;
_10 = !false;
_16.0.4 = _12.4;
_8.0 = core::ptr::addr_of!(_16.0.1);
RET = !_12.3;
match _12.4 {
0 => bb3,
1 => bb4,
12311 => bb6,
_ => bb5
}
}
bb11 = {
_2 = _5;
_3 = (-24920_i16);
_4 = -_1;
_10 = !false;
place!(Field::<u64>(Variant(_6, 2), 0)) = !9645871546197686635_u64;
SetDiscriminant(_6, 2);
place!(Field::<u64>(Variant(_6, 2), 0)) = !7324679225781929661_u64;
_12.1 = 6969540466774468279_i64 ^ (-3010080160194401727_i64);
_8.0 = core::ptr::addr_of!(_12.1);
_12.2 = !_12.1;
Call(_12.1 = fn2(Move(_6), _5, _5, _5, _8.1, _8, _5, _10, _8, _2, _8.1, _8.0), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_16.5.1 = _16.0.4 as i64;
_8.0 = core::ptr::addr_of!(_25);
_22 = [_16.6,_16.6,_16.1,_16.1,_16.1];
_12.1 = _16.5.1 | _12.2;
_16.5.1 = _12.1;
_16.0 = _16.5;
_10 = !_19;
Call(_23 = core::intrinsics::bswap((-27_isize)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_24 = _12.3 as u128;
_12 = (_16.0.0, _16.5.1, _16.5.2, _16.5.3, _16.5.4, _16.0.5);
_29.1 = -_12.5;
_24 = _9;
_30 = [_15,_15,_15,_15,_15];
_16.5.3 = _12.3;
_26 = [_15,_15,_15,_15,_15,_15];
_16.2 = _23;
_15 = '\u{f069f}';
_23 = _16.2 + _16.2;
_29.1 = _16.5.5;
_4 = _1 << _5;
_19 = !_10;
RET = _12.3 >> _16.1;
_8.0 = core::ptr::addr_of!(_12.1);
_1 = _16.5.4 as i8;
_26 = [_15,_15,_15,_15,_15,_15];
_10 = !_18;
_9 = !_24;
_27 = _30;
_16.5.1 = !_12.2;
_4 = !_1;
_12.2 = _16.5.3 as i64;
_12.3 = _16.0.3 | _16.0.3;
_21 = _30;
_4 = _1 ^ _1;
Goto(bb14)
}
bb14 = {
_29 = (_8.0, _20);
_32 = _2 as i16;
_21 = [_15,_15,_15,_15,_15];
_1 = _4;
_13 = -_1;
_22 = [_16.1,_16.1,_16.6,_16.6,_16.1];
_26 = [_15,_15,_15,_15,_15,_15];
_8 = (_29.0, _29.1);
_13 = _1;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(1_usize, 24_usize, Move(_24), 30_usize, Move(_30), 15_usize, Move(_15), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(1_usize, 26_usize, Move(_26), 27_usize, Move(_27), 32_usize, Move(_32), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(1_usize, 22_usize, Move(_22), 4_usize, Move(_4), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: Adt48,mut _2: i128,mut _3: i128,mut _4: i128,mut _5: f32,mut _6: (*const i64, f32),mut _7: i128,mut _8: bool,mut _9: (*const i64, f32),mut _10: i128,mut _11: f32,mut _12: *const i64) -> i64 {
mir! {
type RET = i64;
let _13: f32;
let _14: Adt46;
let _15: [char; 6];
let _16: usize;
let _17: Adt44;
let _18: f64;
let _19: u32;
let _20: [char; 5];
let _21: *const i64;
let _22: Adt41;
let _23: f64;
let _24: i64;
let _25: (i16, i64, i64, u32, u16, f32);
let _26: u128;
let _27: char;
let _28: bool;
let _29: isize;
let _30: f64;
let _31: i64;
let _32: f64;
let _33: isize;
let _34: f64;
let _35: [u32; 4];
let _36: [i16; 5];
let _37: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _38: isize;
let _39: [i16; 5];
let _40: [i16; 5];
let _41: char;
let _42: (i16, i64, i64, u32, u16, f32);
let _43: ();
let _44: ();
{
SetDiscriminant(_1, 3);
place!(Field::<(*const i64, f32)>(Variant(_1, 3), 2)) = (_9.0, _5);
place!(Field::<(*const i64, f32)>(Variant(_1, 3), 2)) = (_6.0, _5);
_6.1 = -_5;
place!(Field::<Adt42>(Variant(_1, 3), 1)) = Adt42::Variant0 { fld0: 697398380036186676_i64,fld1: 7388471861252542109_usize,fld2: 9223372036854775807_isize };
Call(place!(Field::<Adt42>(Variant(_1, 3), 1)) = fn3(_6.1, Field::<(*const i64, f32)>(Variant(_1, 3), 2), Field::<(*const i64, f32)>(Variant(_1, 3), 2).1, _10, _11, Field::<(*const i64, f32)>(Variant(_1, 3), 2).1, _9, _3, _3, _9.0, _10, _6.0, _11, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = (_12, _5);
_9 = (Field::<(*const i64, f32)>(Variant(_1, 3), 2).0, _11);
_6 = _9;
RET = 44_i8 as i64;
Goto(bb2)
}
bb2 = {
_4 = Field::<usize>(Variant(Field::<Adt42>(Variant(_1, 3), 1), 0), 1) as i128;
_4 = 107_u8 as i128;
place!(Field::<isize>(Variant(place!(Field::<Adt42>(Variant(_1, 3), 1)), 0), 2)) = (-9223372036854775808_isize) | 9223372036854775807_isize;
place!(Field::<(*const i64, f32)>(Variant(_1, 3), 2)).0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(place!(Field::<Adt42>(Variant(_1, 3), 1)), 0), 0)));
_6 = Field::<(*const i64, f32)>(Variant(_1, 3), 2);
_2 = 85_i8 as i128;
_7 = _10 & _3;
_9.0 = _12;
_15 = ['\u{e8663}','\u{a2b58}','\u{a27a4}','\u{6d0a9}','\u{fffcd}','\u{590f0}'];
_7 = !_10;
_17.fld4 = Field::<usize>(Variant(Field::<Adt42>(Variant(_1, 3), 1), 0), 1);
place!(Field::<u16>(Variant(_1, 3), 3)) = 53384_u16;
_4 = Field::<u16>(Variant(_1, 3), 3) as i128;
_7 = _2;
_1 = Adt48::Variant2 { fld0: 5441452628834264870_u64 };
place!(Field::<u64>(Variant(_1, 2), 0)) = (-31484_i16) as u64;
RET = (-1888335684_i32) as i64;
_13 = -_11;
_16 = !_17.fld4;
_6 = _9;
_2 = _3 << _7;
_17.fld1 = '\u{af7d}';
_5 = -_13;
_17.fld0 = [62_u8,159_u8,56_u8,139_u8,29_u8];
_17.fld5 = core::ptr::addr_of!(RET);
_4 = _2 ^ _10;
_17.fld3 = !Field::<u64>(Variant(_1, 2), 0);
_7 = _8 as i128;
Call(_17.fld4 = core::intrinsics::transmute(_17.fld3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _4;
_9.1 = -_11;
_16 = _17.fld4 - _17.fld4;
_17.fld0 = [175_u8,137_u8,44_u8,192_u8,219_u8];
_9.0 = _12;
_19 = 1580516603_u32;
_6 = _9;
_6 = _9;
_17.fld2 = RET as isize;
_10 = _3 + _3;
_16 = 1976158549_i32 as usize;
_6 = _9;
_6 = _9;
_17.fld5 = core::ptr::addr_of!(RET);
_15 = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1];
place!(Field::<u64>(Variant(_1, 2), 0)) = _17.fld3;
Goto(bb4)
}
bb4 = {
_9.1 = Field::<u64>(Variant(_1, 2), 0) as f32;
_18 = 51906475311417444281303645740980707383_u128 as f64;
_8 = false;
_18 = 6889_u16 as f64;
_9 = _6;
_18 = _6.1 as f64;
_13 = RET as f32;
_3 = _4;
_17.fld2 = _19 as isize;
_9 = (_17.fld5, _11);
_23 = _18;
_9 = (_12, _6.1);
_17.fld5 = core::ptr::addr_of!(RET);
_4 = _10;
_17.fld0 = [148_u8,160_u8,242_u8,208_u8,40_u8];
_18 = 668291862_i32 as f64;
_21 = core::ptr::addr_of!(RET);
_19 = 16611_i16 as u32;
_16 = _17.fld4 << _4;
SetDiscriminant(_1, 0);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).3 = _23 + _23;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1 = ((-30256_i16), (*_21), RET, _19, 58474_u16, _13);
_23 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).3;
_17.fld1 = '\u{171ca}';
Goto(bb5)
}
bb5 = {
_17.fld0 = [189_u8,12_u8,26_u8,129_u8,193_u8];
_19 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).3 as u32;
_6.0 = core::ptr::addr_of!(_25.1);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.1 = -(*_21);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.2 = RET;
_6.0 = _21;
_17.fld4 = _17.fld2 as usize;
Goto(bb6)
}
bb6 = {
_21 = core::ptr::addr_of!(_25.1);
_7 = RET as i128;
_2 = _3;
_6.1 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.5;
_17.fld0 = [110_u8,27_u8,94_u8,147_u8,232_u8];
_26 = 255856336451730617162916791403743330671_u128;
_15 = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1];
_25.3 = _5 as u32;
_25.2 = _17.fld1 as i64;
_25.0 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.0;
place!(Field::<f64>(Variant(_1, 0), 3)) = _23;
match Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb7,
58474 => bb9,
_ => bb8
}
}
bb7 = {
_17.fld0 = [189_u8,12_u8,26_u8,129_u8,193_u8];
_19 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).3 as u32;
_6.0 = core::ptr::addr_of!(_25.1);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.1 = -(*_21);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.2 = RET;
_6.0 = _21;
_17.fld4 = _17.fld2 as usize;
Goto(bb6)
}
bb8 = {
_9 = (_12, _5);
_9 = (Field::<(*const i64, f32)>(Variant(_1, 3), 2).0, _11);
_6 = _9;
RET = 44_i8 as i64;
Goto(bb2)
}
bb9 = {
_25.4 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.4;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).4 = 14_i8 << _4;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.2 = _25.2;
_29 = _17.fld2;
_28 = !_8;
_17.fld3 = !6774767388127032797_u64;
(*_21) = RET + _25.2;
place!(Field::<[u8; 5]>(Variant(_1, 0), 0)) = [24_u8,181_u8,186_u8,4_u8,150_u8];
_25.5 = _11 * _13;
_25.5 = -_9.1;
_20 = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1];
_4 = _2 + _2;
_23 = Field::<f64>(Variant(_1, 0), 3) + _18;
place!(Field::<[char; 6]>(Variant(_1, 0), 4)) = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1];
Call(place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.1 = core::intrinsics::bswap((*_21)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = Adt44 { fld0: Field::<[u8; 5]>(Variant(_1, 0), 0),fld1: '\u{99c5b}',fld2: _29,fld3: 9964803728539557732_u64,fld4: _16,fld5: _6.0 };
Goto(bb11)
}
bb11 = {
place!(Field::<[char; 6]>(Variant(_1, 0), 4)) = [_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1,_17.fld1];
_6.1 = _25.5 - Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.5;
_25.2 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.1;
_32 = Field::<f64>(Variant(_1, 0), 3) - _18;
Call(_3 = core::intrinsics::bswap(_10), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = (*_21) ^ (*_21);
_6.0 = core::ptr::addr_of!(place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.1);
_5 = _26 as f32;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.0 = _25.0 + _25.0;
_17.fld2 = -_29;
_32 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).3;
RET = !Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.2;
_2 = _4;
_24 = _25.1;
_25.2 = !(*_21);
_3 = !_10;
Call(_20 = fn17(_17.fld1, _2, Move(_17), Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1 = (_25.0, (*_21), _25.1, _19, _25.4, _5);
_34 = _2 as f64;
RET = (*_21) << Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.1;
_25.4 = !Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.4;
_30 = -_34;
_6.1 = _25.5;
_17.fld5 = core::ptr::addr_of!(RET);
_31 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.1 + RET;
Call(_17.fld3 = fn18(_30, _20, _20), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_21 = _9.0;
_37.1.5.3 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.3;
_31 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.2 | _25.2;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2)).1.2 = _17.fld3 as i64;
_37.1.3 = [Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.3,_19,Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.3,_25.3];
_6.1 = _25.4 as f32;
_38 = _29;
_37.2 = [_25.0,Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.0,_25.0,_25.0,Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.0];
_29 = _38 << Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).4;
_25.3 = _16 as u32;
_37.1.5.5 = _6.1 + Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.5;
_42 = (_25.0, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.1, RET, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.3, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_1, 0), 2).1.4, _37.1.5.5);
_37.0 = _25;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(2_usize, 7_usize, Move(_7), 24_usize, Move(_24), 38_usize, Move(_38), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(2_usize, 29_usize, Move(_29), 26_usize, Move(_26), 15_usize, Move(_15), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: f32,mut _2: (*const i64, f32),mut _3: f32,mut _4: i128,mut _5: f32,mut _6: f32,mut _7: (*const i64, f32),mut _8: i128,mut _9: i128,mut _10: *const i64,mut _11: i128,mut _12: *const i64,mut _13: f32,mut _14: i128) -> Adt42 {
mir! {
type RET = Adt42;
let _15: f32;
let _16: f32;
let _17: f32;
let _18: bool;
let _19: [char; 5];
let _20: isize;
let _21: [u8; 5];
let _22: [i16; 5];
let _23: isize;
let _24: i128;
let _25: [isize; 3];
let _26: f32;
let _27: u64;
let _28: f32;
let _29: *const u32;
let _30: bool;
let _31: [char; 6];
let _32: *mut char;
let _33: Adt50;
let _34: (*const i64, f32);
let _35: char;
let _36: f64;
let _37: [u8; 5];
let _38: ();
let _39: ();
{
_11 = !_14;
_4 = _8 >> _14;
_2.1 = _5;
_13 = -_1;
Goto(bb1)
}
bb1 = {
_5 = -_7.1;
_6 = -_2.1;
_7.1 = 5_usize as f32;
_2.0 = _7.0;
Goto(bb2)
}
bb2 = {
_3 = -_2.1;
_8 = !_14;
_2.0 = _12;
RET = Adt42::Variant0 { fld0: (-488675349234382545_i64),fld1: 11410069136703548733_usize,fld2: 9223372036854775807_isize };
place!(Field::<isize>(Variant(RET, 0), 2)) = false as isize;
place!(Field::<i64>(Variant(RET, 0), 0)) = 5953075785679161956_usize as i64;
place!(Field::<isize>(Variant(RET, 0), 2)) = -(-9223372036854775808_isize);
_3 = -_7.1;
place!(Field::<usize>(Variant(RET, 0), 1)) = 7_usize;
_3 = _13 * _6;
_2.1 = _3;
_5 = _3;
_16 = -_1;
place!(Field::<usize>(Variant(RET, 0), 1)) = 8395959932963174879_usize - 2667313234514010823_usize;
place!(Field::<i64>(Variant(RET, 0), 0)) = false as i64;
Goto(bb3)
}
bb3 = {
_7.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
_19 = ['\u{a18ec}','\u{d9d0d}','\u{cb597}','\u{81cae}','\u{8ea1}'];
_7 = _2;
_21 = [181_u8,88_u8,204_u8,136_u8,112_u8];
place!(Field::<isize>(Variant(RET, 0), 2)) = 9223372036854775807_isize;
_3 = -_1;
Call(_20 = fn4(_7.0, _7, Field::<isize>(Variant(RET, 0), 2), RET, _2.1, _8, Field::<isize>(Variant(RET, 0), 2), _16, _7, _7.0, Field::<isize>(Variant(RET, 0), 2), _21, _2, _12, Field::<i64>(Variant(RET, 0), 0), _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = (_12, _5);
_6 = _5 + _5;
_23 = Field::<isize>(Variant(RET, 0), 2) + Field::<isize>(Variant(RET, 0), 2);
_8 = _4 * _4;
_15 = 54256_u16 as f32;
_14 = !_11;
_7.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
_13 = _15 + _6;
place!(Field::<i64>(Variant(RET, 0), 0)) = (-4158528905180399720_i64);
_23 = _20 + Field::<isize>(Variant(RET, 0), 2);
_5 = -_13;
_3 = -_1;
_18 = _5 == _15;
place!(Field::<usize>(Variant(RET, 0), 1)) = _18 as usize;
place!(Field::<isize>(Variant(RET, 0), 2)) = _14 as isize;
match Field::<i64>(Variant(RET, 0), 0) {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463459216078526587811736 => bb12,
_ => bb11
}
}
bb5 = {
_7.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
_19 = ['\u{a18ec}','\u{d9d0d}','\u{cb597}','\u{81cae}','\u{8ea1}'];
_7 = _2;
_21 = [181_u8,88_u8,204_u8,136_u8,112_u8];
place!(Field::<isize>(Variant(RET, 0), 2)) = 9223372036854775807_isize;
_3 = -_1;
Call(_20 = fn4(_7.0, _7, Field::<isize>(Variant(RET, 0), 2), RET, _2.1, _8, Field::<isize>(Variant(RET, 0), 2), _16, _7, _7.0, Field::<isize>(Variant(RET, 0), 2), _21, _2, _12, Field::<i64>(Variant(RET, 0), 0), _11), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_3 = -_2.1;
_8 = !_14;
_2.0 = _12;
RET = Adt42::Variant0 { fld0: (-488675349234382545_i64),fld1: 11410069136703548733_usize,fld2: 9223372036854775807_isize };
place!(Field::<isize>(Variant(RET, 0), 2)) = false as isize;
place!(Field::<i64>(Variant(RET, 0), 0)) = 5953075785679161956_usize as i64;
place!(Field::<isize>(Variant(RET, 0), 2)) = -(-9223372036854775808_isize);
_3 = -_7.1;
place!(Field::<usize>(Variant(RET, 0), 1)) = 7_usize;
_3 = _13 * _6;
_2.1 = _3;
_5 = _3;
_16 = -_1;
place!(Field::<usize>(Variant(RET, 0), 1)) = 8395959932963174879_usize - 2667313234514010823_usize;
place!(Field::<i64>(Variant(RET, 0), 0)) = false as i64;
Goto(bb3)
}
bb7 = {
_5 = -_7.1;
_6 = -_2.1;
_7.1 = 5_usize as f32;
_2.0 = _7.0;
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
_9 = _23 as i128;
_12 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
_7 = (_2.0, _5);
_17 = 8778261034080778913_u64 as f32;
_1 = _16;
_22 = [(-2434_i16),9715_i16,2305_i16,6372_i16,(-1764_i16)];
SetDiscriminant(RET, 1);
_4 = 10697517775485604082_u64 as i128;
place!(Field::<i32>(Variant(RET, 1), 1)) = (-1071195416_i32) >> _23;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(RET, 1), 0)).1.2 = (-9137923712013345994_i64) & (-7464919869522894734_i64);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(RET, 1), 0)).1 = (10384_i16, (-4487149636562485336_i64), (-6374650721312702755_i64), 2301951417_u32, 30034_u16, _5);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(RET, 1), 0)).1.3 = 103526483_u32 << _23;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(RET, 1), 0)).1.0 = -(-4891_i16);
RET = Adt42::Variant0 { fld0: (-2091834997395171652_i64),fld1: 3_usize,fld2: _23 };
place!(Field::<i64>(Variant(RET, 0), 0)) = (-21433591519093989_i64) << _4;
_6 = -_1;
_8 = _14 << _11;
_7.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
place!(Field::<usize>(Variant(RET, 0), 1)) = 62_u8 as usize;
Call(_26 = fn5(Field::<isize>(Variant(RET, 0), 2), _10, _4, _19, RET, _7.0, _13), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_26 = -_5;
_2.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
SetDiscriminant(RET, 0);
_20 = _23;
_12 = core::ptr::addr_of!(place!(Field::<i64>(Variant(RET, 0), 0)));
place!(Field::<usize>(Variant(RET, 0), 1)) = 2_usize;
_14 = -_4;
_24 = _9 * _9;
_15 = _7.1 - _13;
place!(Field::<usize>(Variant(RET, 0), 1)) = '\u{22e07}' as usize;
_1 = -_7.1;
place!(Field::<i64>(Variant(RET, 0), 0)) = !(-2811404524639834800_i64);
_2 = (_12, _13);
_13 = _23 as f32;
_10 = _7.0;
Goto(bb14)
}
bb14 = {
_13 = -_7.1;
_4 = _24 ^ _8;
_21 = [19_u8,249_u8,40_u8,243_u8,143_u8];
_13 = _15 - _5;
_23 = _20 ^ _20;
_28 = _5 + _3;
_19 = ['\u{77a13}','\u{f9d7a}','\u{e6773}','\u{d3162}','\u{cfee2}'];
_28 = -_5;
_23 = _20;
_7.1 = _13 * _2.1;
_2.0 = _10;
_25 = [_23,_20,_20];
Goto(bb15)
}
bb15 = {
_24 = -_4;
_6 = _2.1 + _28;
place!(Field::<isize>(Variant(RET, 0), 2)) = _23;
_24 = 122129127475036412200694401946907447209_u128 as i128;
_2.1 = (-316928693_i32) as f32;
_7.1 = 1776353608_u32 as f32;
_20 = 300238576601460661924606794796419757477_u128 as isize;
_10 = _2.0;
place!(Field::<usize>(Variant(RET, 0), 1)) = !4158872450380760341_usize;
_27 = 13713342045572310471_u64 ^ 17027802058410077317_u64;
_31 = ['\u{21e50}','\u{d135a}','\u{542ed}','\u{ce2aa}','\u{a7353}','\u{50c7f}'];
_8 = -_4;
_13 = _5;
_3 = _15;
_34.0 = _7.0;
_17 = _27 as f32;
_34 = _7;
_16 = _15 + _26;
place!(Field::<i64>(Variant(RET, 0), 0)) = 6348826746998562723_i64 & 2378430700237860035_i64;
_17 = _6 * _28;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(3_usize, 8_usize, Move(_8), 19_usize, Move(_19), 11_usize, Move(_11), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(3_usize, 27_usize, Move(_27), 9_usize, Move(_9), 25_usize, Move(_25), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: *const i64,mut _2: (*const i64, f32),mut _3: isize,mut _4: Adt42,mut _5: f32,mut _6: i128,mut _7: isize,mut _8: f32,mut _9: (*const i64, f32),mut _10: *const i64,mut _11: isize,mut _12: [u8; 5],mut _13: (*const i64, f32),mut _14: *const i64,mut _15: i64,mut _16: i128) -> isize {
mir! {
type RET = isize;
let _17: [isize; 3];
let _18: bool;
let _19: *const u32;
let _20: isize;
let _21: [char; 6];
let _22: [char; 6];
let _23: ();
let _24: ();
{
_8 = Field::<i64>(Variant(_4, 0), 0) as f32;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
9223372036854775807 => bb7,
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
_2 = _13;
_16 = _6 * _6;
_9.1 = -_13.1;
_13.0 = _10;
_2.1 = _13.1 - _13.1;
_17 = [_7,_11,_11];
_13.0 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_4, 0), 0)));
_13.1 = _7 as f32;
place!(Field::<usize>(Variant(_4, 0), 1)) = !17072983589692820597_usize;
Goto(bb8)
}
bb8 = {
_6 = !_16;
place!(Field::<i64>(Variant(_4, 0), 0)) = _15;
SetDiscriminant(_4, 0);
_3 = _11;
match _11 {
0 => bb9,
1 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
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
_7 = _3;
place!(Field::<usize>(Variant(_4, 0), 1)) = 13400645262715445803_usize - 7658018315127908057_usize;
_17 = [_7,_7,_11];
place!(Field::<i64>(Variant(_4, 0), 0)) = !_15;
_9.0 = core::ptr::addr_of!(_15);
_2 = _13;
_2.1 = _5;
match _7 {
0 => bb7,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb5,
5 => bb13,
9223372036854775807 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
RET = (-71_i8) as isize;
_21 = ['\u{7ba3b}','\u{7240f}','\u{8e6a4}','\u{6564f}','\u{aada2}','\u{860fe}'];
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(4_usize, 16_usize, Move(_16), 11_usize, Move(_11), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: *const i64,mut _3: i128,mut _4: [char; 5],mut _5: Adt42,mut _6: *const i64,mut _7: f32) -> f32 {
mir! {
type RET = f32;
let _8: bool;
let _9: bool;
let _10: char;
let _11: u8;
let _12: isize;
let _13: i16;
let _14: &'static usize;
let _15: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _16: [char; 6];
let _17: [u8; 5];
let _18: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _19: isize;
let _20: [u8; 5];
let _21: Adt39;
let _22: i16;
let _23: f32;
let _24: f32;
let _25: f64;
let _26: [u32; 4];
let _27: i32;
let _28: Adt40;
let _29: char;
let _30: bool;
let _31: i64;
let _32: ();
let _33: ();
{
_7 = (-29894_i16) as f32;
_9 = false;
(*_6) = -Field::<i64>(Variant(_5, 0), 0);
RET = _7 - _7;
_6 = _2;
Call(_11 = fn6(_6, Field::<isize>(Variant(_5, 0), 2), Field::<i64>(Variant(_5, 0), 0), RET, Field::<isize>(Variant(_5, 0), 2), _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = Adt42::Variant0 { fld0: (-2387302787676842491_i64),fld1: 1_usize,fld2: _1 };
_6 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_5, 0), 0)));
RET = _7 - _7;
_10 = '\u{14f75}';
(*_6) = -(-6962714946359924447_i64);
_10 = '\u{d1de3}';
_3 = (-43355766850511422235119481055968339465_i128) + (-18687983928261944051062231054580077289_i128);
_4 = [_10,_10,_10,_10,_10];
place!(Field::<isize>(Variant(_5, 0), 2)) = (*_6) as isize;
_8 = _9;
_13 = 23823_i16 * 32311_i16;
place!(Field::<usize>(Variant(_5, 0), 1)) = 5_usize ^ 3_usize;
Call(place!(Field::<i64>(Variant(_5, 0), 0)) = fn11(RET, Field::<usize>(Variant(_5, 0), 1), _13, _9, _8, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _7 - _7;
(*_6) = 1775473113940670036_i64;
_11 = !230_u8;
place!(Field::<i64>(Variant(_5, 0), 0)) = (-6636061105102399284_i64) + 8025855178137862003_i64;
_4 = [_10,_10,_10,_10,_10];
_2 = _6;
(*_6) = !(-456635453949445814_i64);
_11 = (-45_i8) as u8;
_12 = _1;
_13 = 1954_i16 & 23072_i16;
place!(Field::<isize>(Variant(_5, 0), 2)) = _12 | _12;
RET = (-63_i8) as f32;
place!(Field::<i64>(Variant(_5, 0), 0)) = 3698502431156766887_i64 * 8535986950972815148_i64;
_12 = _1;
(*_6) = Field::<usize>(Variant(_5, 0), 1) as i64;
_13 = (-28_i8) as i16;
SetDiscriminant(_5, 0);
place!(Field::<i64>(Variant(_5, 0), 0)) = !2247819583762769817_i64;
place!(Field::<usize>(Variant(_5, 0), 1)) = 6_usize;
Goto(bb3)
}
bb3 = {
_15.5 = (_13, Field::<i64>(Variant(_5, 0), 0), Field::<i64>(Variant(_5, 0), 0), 2662943764_u32, 3823_u16, _7);
_8 = _9;
_15.0 = (_15.5.0, _15.5.1, _15.5.1, _15.5.3, _15.5.4, RET);
_5 = Adt42::Variant0 { fld0: _15.5.2,fld1: 3772474388986071291_usize,fld2: _1 };
_2 = core::ptr::addr_of!(_15.0.1);
_15.5.5 = _12 as f32;
_15.1 = _11;
_10 = '\u{10cdaa}';
place!(Field::<isize>(Variant(_5, 0), 2)) = _12;
_16 = [_10,_10,_10,_10,_10,_10];
(*_2) = Field::<i64>(Variant(_5, 0), 0);
_15.1 = !_11;
_14 = &place!(Field::<usize>(Variant(_5, 0), 1));
match _15.0.4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
3823 => bb8,
_ => bb7
}
}
bb4 = {
RET = _7 - _7;
(*_6) = 1775473113940670036_i64;
_11 = !230_u8;
place!(Field::<i64>(Variant(_5, 0), 0)) = (-6636061105102399284_i64) + 8025855178137862003_i64;
_4 = [_10,_10,_10,_10,_10];
_2 = _6;
(*_6) = !(-456635453949445814_i64);
_11 = (-45_i8) as u8;
_12 = _1;
_13 = 1954_i16 & 23072_i16;
place!(Field::<isize>(Variant(_5, 0), 2)) = _12 | _12;
RET = (-63_i8) as f32;
place!(Field::<i64>(Variant(_5, 0), 0)) = 3698502431156766887_i64 * 8535986950972815148_i64;
_12 = _1;
(*_6) = Field::<usize>(Variant(_5, 0), 1) as i64;
_13 = (-28_i8) as i16;
SetDiscriminant(_5, 0);
place!(Field::<i64>(Variant(_5, 0), 0)) = !2247819583762769817_i64;
place!(Field::<usize>(Variant(_5, 0), 1)) = 6_usize;
Goto(bb3)
}
bb5 = {
_5 = Adt42::Variant0 { fld0: (-2387302787676842491_i64),fld1: 1_usize,fld2: _1 };
_6 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_5, 0), 0)));
RET = _7 - _7;
_10 = '\u{14f75}';
(*_6) = -(-6962714946359924447_i64);
_10 = '\u{d1de3}';
_3 = (-43355766850511422235119481055968339465_i128) + (-18687983928261944051062231054580077289_i128);
_4 = [_10,_10,_10,_10,_10];
place!(Field::<isize>(Variant(_5, 0), 2)) = (*_6) as isize;
_8 = _9;
_13 = 23823_i16 * 32311_i16;
place!(Field::<usize>(Variant(_5, 0), 1)) = 5_usize ^ 3_usize;
Call(place!(Field::<i64>(Variant(_5, 0), 0)) = fn11(RET, Field::<usize>(Variant(_5, 0), 1), _13, _9, _8, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_18.5.4 = _15.0.4 + _15.0.4;
_19 = _12;
_15.5.5 = -_15.0.5;
_18.2 = _12 >> Field::<i64>(Variant(_5, 0), 0);
_6 = core::ptr::addr_of!(_18.5.1);
_17 = [_11,_15.1,_15.1,_11,_11];
_18.3 = [_15.5.3,_15.0.3,_15.5.3,_15.5.3];
place!(Field::<usize>(Variant(_5, 0), 1)) = 8749437075623021987_usize >> _15.0.1;
_18.5 = (_13, (*_2), Field::<i64>(Variant(_5, 0), 0), _15.0.3, _15.5.4, _7);
_20 = _17;
_18.4 = _15.0.3 as f64;
_18.0.1 = Field::<i64>(Variant(_5, 0), 0);
_11 = !_15.1;
_15.6 = !_11;
_17 = _20;
_18.0.5 = -_15.0.5;
_18.0.4 = _15.5.4 << _18.5.2;
_18.0 = (_13, _15.5.1, (*_6), _15.0.3, _18.5.4, _7);
_15.0.0 = _15.5.0 | _18.0.0;
_15.0.1 = _15.5.1 | _15.5.1;
SetDiscriminant(_5, 0);
_2 = _6;
_14 = &place!(Field::<usize>(Variant(_5, 0), 1));
_18.2 = _19;
_18.5.0 = _15.6 as i16;
(*_6) = _18.0.1 - _15.0.2;
match _18.0.3 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb9,
4 => bb10,
2662943764 => bb12,
_ => bb11
}
}
bb9 = {
_5 = Adt42::Variant0 { fld0: (-2387302787676842491_i64),fld1: 1_usize,fld2: _1 };
_6 = core::ptr::addr_of!(place!(Field::<i64>(Variant(_5, 0), 0)));
RET = _7 - _7;
_10 = '\u{14f75}';
(*_6) = -(-6962714946359924447_i64);
_10 = '\u{d1de3}';
_3 = (-43355766850511422235119481055968339465_i128) + (-18687983928261944051062231054580077289_i128);
_4 = [_10,_10,_10,_10,_10];
place!(Field::<isize>(Variant(_5, 0), 2)) = (*_6) as isize;
_8 = _9;
_13 = 23823_i16 * 32311_i16;
place!(Field::<usize>(Variant(_5, 0), 1)) = 5_usize ^ 3_usize;
Call(place!(Field::<i64>(Variant(_5, 0), 0)) = fn11(RET, Field::<usize>(Variant(_5, 0), 1), _13, _9, _8, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
RET = _7 - _7;
(*_6) = 1775473113940670036_i64;
_11 = !230_u8;
place!(Field::<i64>(Variant(_5, 0), 0)) = (-6636061105102399284_i64) + 8025855178137862003_i64;
_4 = [_10,_10,_10,_10,_10];
_2 = _6;
(*_6) = !(-456635453949445814_i64);
_11 = (-45_i8) as u8;
_12 = _1;
_13 = 1954_i16 & 23072_i16;
place!(Field::<isize>(Variant(_5, 0), 2)) = _12 | _12;
RET = (-63_i8) as f32;
place!(Field::<i64>(Variant(_5, 0), 0)) = 3698502431156766887_i64 * 8535986950972815148_i64;
_12 = _1;
(*_6) = Field::<usize>(Variant(_5, 0), 1) as i64;
_13 = (-28_i8) as i16;
SetDiscriminant(_5, 0);
place!(Field::<i64>(Variant(_5, 0), 0)) = !2247819583762769817_i64;
place!(Field::<usize>(Variant(_5, 0), 1)) = 6_usize;
Goto(bb3)
}
bb11 = {
_15.5 = (_13, Field::<i64>(Variant(_5, 0), 0), Field::<i64>(Variant(_5, 0), 0), 2662943764_u32, 3823_u16, _7);
_8 = _9;
_15.0 = (_15.5.0, _15.5.1, _15.5.1, _15.5.3, _15.5.4, RET);
_5 = Adt42::Variant0 { fld0: _15.5.2,fld1: 3772474388986071291_usize,fld2: _1 };
_2 = core::ptr::addr_of!(_15.0.1);
_15.5.5 = _12 as f32;
_15.1 = _11;
_10 = '\u{10cdaa}';
place!(Field::<isize>(Variant(_5, 0), 2)) = _12;
_16 = [_10,_10,_10,_10,_10,_10];
(*_2) = Field::<i64>(Variant(_5, 0), 0);
_15.1 = !_11;
_14 = &place!(Field::<usize>(Variant(_5, 0), 1));
match _15.0.4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
3823 => bb8,
_ => bb7
}
}
bb12 = {
_6 = _2;
Call((*_6) = core::intrinsics::bswap(_18.0.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_15.2 = !_18.2;
_19 = _15.2;
_18.6 = !_11;
(*_6) = _19 as i64;
_18.6 = _15.1 | _15.6;
_15.4 = _18.4;
_6 = _2;
_15.1 = _11 * _11;
_17 = [_18.6,_15.6,_18.6,_18.6,_18.6];
_15.5 = (_13, _18.0.2, (*_6), _15.0.3, _18.0.4, RET);
_15.0 = _15.5;
_18.6 = _15.6;
_15.5.0 = 1872757542_i32 as i16;
place!(Field::<i64>(Variant(_5, 0), 0)) = -_18.0.2;
_15.0.4 = _18.5.4 << _15.5.4;
_15.5.0 = _13 + _15.0.0;
_17 = _20;
_15.5.0 = _18.0.0 << _15.0.3;
place!(Field::<i64>(Variant(_5, 0), 0)) = _15.1 as i64;
place!(Field::<usize>(Variant(_5, 0), 1)) = !3_usize;
_14 = &place!(Field::<usize>(Variant(_5, 0), 1));
_7 = _15.5.5 * RET;
_23 = _18.0.5;
_25 = _18.4 * _15.4;
match _18.5.3 {
0 => bb1,
1 => bb12,
2 => bb3,
3 => bb4,
2662943764 => bb15,
_ => bb14
}
}
bb14 = {
RET = _7 - _7;
(*_6) = 1775473113940670036_i64;
_11 = !230_u8;
place!(Field::<i64>(Variant(_5, 0), 0)) = (-6636061105102399284_i64) + 8025855178137862003_i64;
_4 = [_10,_10,_10,_10,_10];
_2 = _6;
(*_6) = !(-456635453949445814_i64);
_11 = (-45_i8) as u8;
_12 = _1;
_13 = 1954_i16 & 23072_i16;
place!(Field::<isize>(Variant(_5, 0), 2)) = _12 | _12;
RET = (-63_i8) as f32;
place!(Field::<i64>(Variant(_5, 0), 0)) = 3698502431156766887_i64 * 8535986950972815148_i64;
_12 = _1;
(*_6) = Field::<usize>(Variant(_5, 0), 1) as i64;
_13 = (-28_i8) as i16;
SetDiscriminant(_5, 0);
place!(Field::<i64>(Variant(_5, 0), 0)) = !2247819583762769817_i64;
place!(Field::<usize>(Variant(_5, 0), 1)) = 6_usize;
Goto(bb3)
}
bb15 = {
_18.5.3 = (*_14) as u32;
_6 = _2;
_18.0.4 = !_15.0.4;
_28.fld0 = (_2, _7);
_28.fld1.1.5 = -_15.0.5;
_28.fld1.1 = (_18.5.0, _18.5.1, _18.5.2, _15.0.3, _15.5.4, _18.0.5);
_28.fld1.2 = _6;
_15.5 = (_28.fld1.1.0, (*_6), (*_2), _28.fld1.1.3, _18.5.4, _18.5.5);
(*_6) = 93867902874263358370868126645875587191_u128 as i64;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(5_usize, 1_usize, Move(_1), 8_usize, Move(_8), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(5_usize, 16_usize, Move(_16), 4_usize, Move(_4), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *const i64,mut _2: isize,mut _3: i64,mut _4: f32,mut _5: isize,mut _6: Adt42,mut _7: Adt42) -> u8 {
mir! {
type RET = u8;
let _8: Adt38;
let _9: (i16, i64, i64, u32, u16, f32);
let _10: Adt53;
let _11: char;
let _12: [char; 6];
let _13: u128;
let _14: (i16, i64, i64, u32, u16, f32);
let _15: char;
let _16: [u8; 5];
let _17: i16;
let _18: i8;
let _19: (i16, i64, i64, u32, u16, f32);
let _20: Adt47;
let _21: bool;
let _22: [char; 5];
let _23: (*const i64, f32);
let _24: u8;
let _25: f32;
let _26: (i16, i64, i64, u32, u16, f32);
let _27: *const i64;
let _28: (i16, i64, i64, u32, u16, f32);
let _29: char;
let _30: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _31: Adt49;
let _32: f32;
let _33: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _34: ();
let _35: ();
{
_7 = _6;
_3 = Field::<i64>(Variant(_7, 0), 0) | Field::<i64>(Variant(_7, 0), 0);
_5 = Field::<isize>(Variant(_6, 0), 2);
Call(_5 = core::intrinsics::transmute(Field::<isize>(Variant(_7, 0), 2)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-98_i8) as u8;
RET = 151_u8 - 24_u8;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_7, 0), 1) + Field::<usize>(Variant(_7, 0), 1);
_5 = 4071755338_u32 as isize;
_9.1 = !_3;
_9 = (12654_i16, Field::<i64>(Variant(_7, 0), 0), Field::<i64>(Variant(_6, 0), 0), 897455015_u32, 51127_u16, _4);
_9.4 = 15535943116182586876_u64 as u16;
_3 = Field::<isize>(Variant(_6, 0), 2) as i64;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_7, 0), 1);
_7 = _6;
place!(Field::<i64>(Variant(_7, 0), 0)) = (-54269254_i32) as i64;
_9.1 = _3;
_9.3 = !4077328913_u32;
_11 = '\u{10d142}';
Goto(bb2)
}
bb2 = {
place!(Field::<isize>(Variant(_7, 0), 2)) = -_2;
Goto(bb3)
}
bb3 = {
_6 = _7;
place!(Field::<i64>(Variant(_7, 0), 0)) = _9.2 & _9.1;
match _9.0 {
12654 => bb4,
_ => bb2
}
}
bb4 = {
_14.0 = _9.0 << _9.0;
_11 = '\u{eb70c}';
_9.0 = -_14.0;
_9 = (_14.0, Field::<i64>(Variant(_6, 0), 0), Field::<i64>(Variant(_7, 0), 0), 21122970_u32, 56573_u16, _4);
_12 = [_11,_11,_11,_11,_11,_11];
place!(Field::<i64>(Variant(_6, 0), 0)) = -_3;
RET = 29_u8;
SetDiscriminant(_6, 1);
_9.3 = 245144068_u32;
_9.4 = 19148_u16 | 30889_u16;
_12 = [_11,_11,_11,_11,_11,_11];
_3 = _9.2;
place!(Field::<*const usize>(Variant(_6, 1), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_7, 0), 1)));
_15 = _11;
_14.2 = Field::<usize>(Variant(_7, 0), 1) as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1.2 = _9.0 as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1 = (_9.0, Field::<i64>(Variant(_7, 0), 0), _3, _9.3, _9.4, _9.5);
_14.1 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0).1.1;
_11 = _15;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1.3 = !_9.3;
RET = 200_u8 * 91_u8;
_14.5 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0).1.5;
_6 = _7;
Call(_18 = fn7(Field::<isize>(Variant(_7, 0), 2), _9.0, _6, _9.0, _6, _9, _6, _6, _14.0, _14.0, _9.2, _9, _9.5, _9, Field::<i64>(Variant(_7, 0), 0)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14.4 = !_9.4;
_19.4 = _14.4 + _14.4;
_14.1 = Field::<isize>(Variant(_6, 0), 2) as i64;
_16 = [RET,RET,RET,RET,RET];
_16 = [RET,RET,RET,RET,RET];
_9.5 = _4;
_9.2 = !_3;
RET = 1_u8 << _14.4;
match _9.3 {
245144068 => bb6,
_ => bb4
}
}
bb6 = {
_2 = -_5;
_14.2 = Field::<i64>(Variant(_7, 0), 0) | Field::<i64>(Variant(_7, 0), 0);
_14.0 = -_9.0;
_13 = 249118216086434946453035486110497259828_u128 >> Field::<i64>(Variant(_6, 0), 0);
_14.3 = _9.3 + _9.3;
_9.0 = RET as i16;
_9.4 = _15 as u16;
match _9.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
245144068 => bb10,
_ => bb9
}
}
bb7 = {
_14.4 = !_9.4;
_19.4 = _14.4 + _14.4;
_14.1 = Field::<isize>(Variant(_6, 0), 2) as i64;
_16 = [RET,RET,RET,RET,RET];
_16 = [RET,RET,RET,RET,RET];
_9.5 = _4;
_9.2 = !_3;
RET = 1_u8 << _14.4;
match _9.3 {
245144068 => bb6,
_ => bb4
}
}
bb8 = {
_14.0 = _9.0 << _9.0;
_11 = '\u{eb70c}';
_9.0 = -_14.0;
_9 = (_14.0, Field::<i64>(Variant(_6, 0), 0), Field::<i64>(Variant(_7, 0), 0), 21122970_u32, 56573_u16, _4);
_12 = [_11,_11,_11,_11,_11,_11];
place!(Field::<i64>(Variant(_6, 0), 0)) = -_3;
RET = 29_u8;
SetDiscriminant(_6, 1);
_9.3 = 245144068_u32;
_9.4 = 19148_u16 | 30889_u16;
_12 = [_11,_11,_11,_11,_11,_11];
_3 = _9.2;
place!(Field::<*const usize>(Variant(_6, 1), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_7, 0), 1)));
_15 = _11;
_14.2 = Field::<usize>(Variant(_7, 0), 1) as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1.2 = _9.0 as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1 = (_9.0, Field::<i64>(Variant(_7, 0), 0), _3, _9.3, _9.4, _9.5);
_14.1 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0).1.1;
_11 = _15;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1.3 = !_9.3;
RET = 200_u8 * 91_u8;
_14.5 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0).1.5;
_6 = _7;
Call(_18 = fn7(Field::<isize>(Variant(_7, 0), 2), _9.0, _6, _9.0, _6, _9, _6, _6, _14.0, _14.0, _9.2, _9, _9.5, _9, Field::<i64>(Variant(_7, 0), 0)), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
RET = (-98_i8) as u8;
RET = 151_u8 - 24_u8;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_7, 0), 1) + Field::<usize>(Variant(_7, 0), 1);
_5 = 4071755338_u32 as isize;
_9.1 = !_3;
_9 = (12654_i16, Field::<i64>(Variant(_7, 0), 0), Field::<i64>(Variant(_6, 0), 0), 897455015_u32, 51127_u16, _4);
_9.4 = 15535943116182586876_u64 as u16;
_3 = Field::<isize>(Variant(_6, 0), 2) as i64;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_7, 0), 1);
_7 = _6;
place!(Field::<i64>(Variant(_7, 0), 0)) = (-54269254_i32) as i64;
_9.1 = _3;
_9.3 = !4077328913_u32;
_11 = '\u{10d142}';
Goto(bb2)
}
bb10 = {
_26.3 = _14.3 >> _9.0;
_9.2 = _13 as i64;
match _9.3 {
0 => bb4,
1 => bb5,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
245144068 => bb16,
_ => bb15
}
}
bb11 = {
RET = (-98_i8) as u8;
RET = 151_u8 - 24_u8;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_7, 0), 1) + Field::<usize>(Variant(_7, 0), 1);
_5 = 4071755338_u32 as isize;
_9.1 = !_3;
_9 = (12654_i16, Field::<i64>(Variant(_7, 0), 0), Field::<i64>(Variant(_6, 0), 0), 897455015_u32, 51127_u16, _4);
_9.4 = 15535943116182586876_u64 as u16;
_3 = Field::<isize>(Variant(_6, 0), 2) as i64;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_7, 0), 1);
_7 = _6;
place!(Field::<i64>(Variant(_7, 0), 0)) = (-54269254_i32) as i64;
_9.1 = _3;
_9.3 = !4077328913_u32;
_11 = '\u{10d142}';
Goto(bb2)
}
bb12 = {
_14.0 = _9.0 << _9.0;
_11 = '\u{eb70c}';
_9.0 = -_14.0;
_9 = (_14.0, Field::<i64>(Variant(_6, 0), 0), Field::<i64>(Variant(_7, 0), 0), 21122970_u32, 56573_u16, _4);
_12 = [_11,_11,_11,_11,_11,_11];
place!(Field::<i64>(Variant(_6, 0), 0)) = -_3;
RET = 29_u8;
SetDiscriminant(_6, 1);
_9.3 = 245144068_u32;
_9.4 = 19148_u16 | 30889_u16;
_12 = [_11,_11,_11,_11,_11,_11];
_3 = _9.2;
place!(Field::<*const usize>(Variant(_6, 1), 2)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_7, 0), 1)));
_15 = _11;
_14.2 = Field::<usize>(Variant(_7, 0), 1) as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1.2 = _9.0 as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1 = (_9.0, Field::<i64>(Variant(_7, 0), 0), _3, _9.3, _9.4, _9.5);
_14.1 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0).1.1;
_11 = _15;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0)).1.3 = !_9.3;
RET = 200_u8 * 91_u8;
_14.5 = -Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_6, 1), 0).1.5;
_6 = _7;
Call(_18 = fn7(Field::<isize>(Variant(_7, 0), 2), _9.0, _6, _9.0, _6, _9, _6, _6, _14.0, _14.0, _9.2, _9, _9.5, _9, Field::<i64>(Variant(_7, 0), 0)), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
place!(Field::<isize>(Variant(_7, 0), 2)) = -_2;
Goto(bb3)
}
bb14 = {
_2 = -_5;
_14.2 = Field::<i64>(Variant(_7, 0), 0) | Field::<i64>(Variant(_7, 0), 0);
_14.0 = -_9.0;
_13 = 249118216086434946453035486110497259828_u128 >> Field::<i64>(Variant(_6, 0), 0);
_14.3 = _9.3 + _9.3;
_9.0 = RET as i16;
_9.4 = _15 as u16;
match _9.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
6 => bb8,
245144068 => bb10,
_ => bb9
}
}
bb15 = {
_14.4 = !_9.4;
_19.4 = _14.4 + _14.4;
_14.1 = Field::<isize>(Variant(_6, 0), 2) as i64;
_16 = [RET,RET,RET,RET,RET];
_16 = [RET,RET,RET,RET,RET];
_9.5 = _4;
_9.2 = !_3;
RET = 1_u8 << _14.4;
match _9.3 {
245144068 => bb6,
_ => bb4
}
}
bb16 = {
_13 = 73153196618746954739758902536526694292_u128 | 89523662915061956966172862627561596569_u128;
_14.1 = !Field::<i64>(Variant(_6, 0), 0);
_3 = _19.4 as i64;
_26 = (_14.0, _3, _3, _9.3, _19.4, _14.5);
_19.2 = !_14.2;
_19.0 = -_26.0;
_9.4 = !_19.4;
_14.1 = true as i64;
place!(Field::<usize>(Variant(_6, 0), 1)) = !Field::<usize>(Variant(_7, 0), 1);
_30.5.2 = _19.2;
_28.4 = !_14.4;
SetDiscriminant(_6, 3);
_30.2 = _28.4 as isize;
place!(Field::<char>(Variant(_6, 3), 1)) = _15;
_23.1 = _4;
_30.1 = true as u8;
_30.5.2 = _5 as i64;
_26.2 = Field::<i64>(Variant(_7, 0), 0);
SetDiscriminant(_7, 0);
place!(Field::<usize>(Variant(_7, 0), 1)) = _26.5 as usize;
_9.0 = _14.0;
_29 = Field::<char>(Variant(_6, 3), 1);
_30.1 = RET | RET;
_28.3 = Field::<usize>(Variant(_7, 0), 1) as u32;
_19 = _14;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(6_usize, 18_usize, Move(_18), 3_usize, Move(_3), 11_usize, Move(_11), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(6_usize, 15_usize, Move(_15), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: i16,mut _3: Adt42,mut _4: i16,mut _5: Adt42,mut _6: (i16, i64, i64, u32, u16, f32),mut _7: Adt42,mut _8: Adt42,mut _9: i16,mut _10: i16,mut _11: i64,mut _12: (i16, i64, i64, u32, u16, f32),mut _13: f32,mut _14: (i16, i64, i64, u32, u16, f32),mut _15: i64) -> i8 {
mir! {
type RET = i8;
let _16: Adt38;
let _17: i64;
let _18: f32;
let _19: char;
let _20: isize;
let _21: u64;
let _22: [u8; 5];
let _23: Adt43;
let _24: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _25: Adt44;
let _26: Adt51;
let _27: i32;
let _28: f64;
let _29: i128;
let _30: i16;
let _31: Adt45;
let _32: isize;
let _33: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _34: Adt51;
let _35: [i16; 5];
let _36: isize;
let _37: u8;
let _38: i64;
let _39: f64;
let _40: ();
let _41: ();
{
place!(Field::<isize>(Variant(_7, 0), 2)) = Field::<isize>(Variant(_8, 0), 2);
_14.2 = Field::<i64>(Variant(_3, 0), 0) >> _9;
RET = 50_i8;
_14 = _6;
place!(Field::<usize>(Variant(_8, 0), 1)) = !Field::<usize>(Variant(_5, 0), 1);
_6.3 = Field::<i64>(Variant(_3, 0), 0) as u32;
_6.1 = Field::<i64>(Variant(_3, 0), 0) << _4;
_4 = 235_u8 as i16;
_10 = false as i16;
place!(Field::<i64>(Variant(_3, 0), 0)) = _6.1 | _15;
_12.0 = _12.5 as i16;
_6.4 = Field::<i64>(Variant(_3, 0), 0) as u16;
_11 = Field::<i64>(Variant(_8, 0), 0) - _6.1;
_11 = !Field::<i64>(Variant(_3, 0), 0);
_14.0 = !_2;
_12.0 = _2 | _2;
Goto(bb1)
}
bb1 = {
_14.5 = _1 as f32;
place!(Field::<usize>(Variant(_3, 0), 1)) = Field::<usize>(Variant(_5, 0), 1) >> Field::<i64>(Variant(_3, 0), 0);
place!(Field::<usize>(Variant(_5, 0), 1)) = Field::<usize>(Variant(_7, 0), 1) + Field::<usize>(Variant(_3, 0), 1);
_6.2 = _11 + _11;
_11 = Field::<usize>(Variant(_3, 0), 1) as i64;
_6.2 = _6.1 ^ _6.1;
place!(Field::<usize>(Variant(_7, 0), 1)) = !Field::<usize>(Variant(_5, 0), 1);
place!(Field::<isize>(Variant(_7, 0), 2)) = -Field::<isize>(Variant(_5, 0), 2);
_14.3 = _6.3 % _12.3;
_6.4 = _14.4;
_1 = !Field::<isize>(Variant(_8, 0), 2);
_14 = (_12.0, Field::<i64>(Variant(_7, 0), 0), _15, _12.3, _6.4, _13);
RET = 82_i8 * 34_i8;
place!(Field::<i64>(Variant(_5, 0), 0)) = 2547112677539553725_u64 as i64;
SetDiscriminant(_3, 0);
_5 = _7;
_6.5 = _14.5;
_14.0 = _12.0;
_24.0.1 = _6.1 & _12.1;
match _12.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
245144068 => bb7,
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
SetDiscriminant(_5, 0);
_7 = _8;
_25.fld0 = [22_u8,105_u8,192_u8,89_u8,159_u8];
_24.5.5 = _6.5;
_5 = _7;
place!(Field::<isize>(Variant(_8, 0), 2)) = Field::<isize>(Variant(_7, 0), 2) * Field::<isize>(Variant(_7, 0), 2);
_24.5.2 = -_11;
place!(Field::<isize>(Variant(_7, 0), 2)) = !Field::<isize>(Variant(_8, 0), 2);
_24.0 = (_2, _6.2, _6.2, _6.3, _6.4, _13);
_24.3 = [_14.3,_24.0.3,_6.3,_24.0.3];
_13 = _14.5 + _6.5;
place!(Field::<i64>(Variant(_3, 0), 0)) = _6.1 | _11;
_27 = Field::<usize>(Variant(_7, 0), 1) as i32;
_14.3 = _6.3 / _12.3;
place!(Field::<i64>(Variant(_7, 0), 0)) = _11;
_2 = _24.0.0;
_25.fld4 = !Field::<usize>(Variant(_8, 0), 1);
_24.0.4 = Field::<isize>(Variant(_8, 0), 2) as u16;
Call(_24.0.5 = fn8(_24.5.2, _24.0.1, _24.0.1, _24.5.5, Field::<i64>(Variant(_3, 0), 0), _8, _6.1, _6.2, _24.0.2, _14, _8, _24.0.1, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = -_12.5;
place!(Field::<i64>(Variant(_3, 0), 0)) = _24.0.1 * _24.0.1;
_29 = !(-62751763842189648253384501432057417790_i128);
place!(Field::<usize>(Variant(_8, 0), 1)) = !_25.fld4;
_22 = [164_u8,1_u8,148_u8,160_u8,106_u8];
place!(Field::<usize>(Variant(_5, 0), 1)) = Field::<isize>(Variant(_7, 0), 2) as usize;
_6.5 = _12.0 as f32;
_6.3 = _14.3 % _12.3;
_31.fld4 = core::ptr::addr_of_mut!(_19);
_24.5.1 = Field::<i64>(Variant(_3, 0), 0);
_5 = _7;
place!(Field::<isize>(Variant(_3, 0), 2)) = Field::<isize>(Variant(_7, 0), 2);
SetDiscriminant(_5, 2);
_31.fld2.3 = Field::<isize>(Variant(_8, 0), 2) as f64;
_12.2 = _13 as i64;
match _12.3 {
0 => bb1,
1 => bb7,
2 => bb5,
3 => bb4,
4 => bb9,
5 => bb10,
245144068 => bb12,
_ => bb11
}
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
_22 = _25.fld0;
_24.4 = _31.fld2.3;
_31.fld2.1.0 = _24.0.4 as i16;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2)).1 = _14;
_22 = _25.fld0;
_31.fld2.4 = -RET;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2)).1.0 = -_12.0;
_31.fld2.1.2 = _6.2 | Field::<i64>(Variant(_3, 0), 0);
_19 = '\u{24fae}';
_24.2 = _27 as isize;
_12.1 = !_31.fld2.1.2;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2)).4 = 112545651963090838736042656802421278736_u128 as i8;
_20 = !_24.2;
_33.0.0 = _24.0.0;
_24.6 = !138_u8;
_24.0.4 = _14.4;
SetDiscriminant(_8, 1);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2)).4 = RET;
SetDiscriminant(_7, 3);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_8, 1), 0)).1.3 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2).1.3;
_25.fld3 = 15304271378296671125_u64;
_31.fld2.1 = (_14.0, _24.5.2, _24.0.1, _14.3, _24.0.4, _12.5);
Goto(bb13)
}
bb13 = {
_31.fld0 = _25.fld3 as usize;
Call(_6.2 = core::intrinsics::transmute(_24.0.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<isize>(Variant(_3, 0), 2)) = _1;
_33.5.2 = _24.0.4 as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2)).1.2 = _31.fld0 as i64;
_31.fld1.fld0 = [_24.6,_24.6,_24.6,_24.6,_24.6];
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2)).2 = core::ptr::addr_of!(_14.2);
_31.fld1.fld3 = _31.fld2.3 as u64;
_33.0.1 = _31.fld2.1.2 >> _24.0.1;
_33.0.3 = _6.3 + Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2).1.3;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_8, 1), 0)).3 = _27 as f64;
_21 = _29 as u64;
_16 = Adt38::Variant2 { fld0: Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_8, 1), 0).3 };
_31.fld2.0 = _24.3;
_24.6 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2).1.4 as u8;
_24.5.2 = _24.0.0 as i64;
_6.2 = _12.1;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_8, 1), 0)).1.3 = _31.fld2.1.3;
_12.4 = _14.4;
_25.fld5 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_5, 2), 2).2;
place!(Field::<f64>(Variant(_16, 2), 0)) = -_31.fld2.3;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_8, 1), 0)).1.1 = _12.1 * _12.1;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_8, 1), 0)).1.4 = _31.fld2.1.4 | _31.fld2.1.4;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(7_usize, 1_usize, Move(_1), 9_usize, Move(_9), 11_usize, Move(_11), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(7_usize, 19_usize, Move(_19), 21_usize, Move(_21), 41_usize, _41, 41_usize, _41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: f32,mut _5: i64,mut _6: Adt42,mut _7: i64,mut _8: i64,mut _9: i64,mut _10: (i16, i64, i64, u32, u16, f32),mut _11: Adt42,mut _12: i64,mut _13: Adt42) -> f32 {
mir! {
type RET = f32;
let _14: (i16, i64, i64, u32, u16, f32);
let _15: Adt47;
let _16: f64;
let _17: Adt47;
let _18: char;
let _19: i128;
let _20: [i16; 5];
let _21: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _22: [i16; 5];
let _23: Adt43;
let _24: Adt47;
let _25: u64;
let _26: char;
let _27: [isize; 3];
let _28: f64;
let _29: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _30: *const u32;
let _31: (*const i64, f32);
let _32: u16;
let _33: [u8; 5];
let _34: Adt46;
let _35: [u8; 5];
let _36: isize;
let _37: usize;
let _38: ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8));
let _39: [char; 6];
let _40: f64;
let _41: Adt45;
let _42: [u32; 4];
let _43: [isize; 3];
let _44: [char; 5];
let _45: [char; 6];
let _46: i16;
let _47: f32;
let _48: f64;
let _49: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _50: i128;
let _51: [char; 5];
let _52: &'static usize;
let _53: Adt49;
let _54: [u32; 4];
let _55: i8;
let _56: u16;
let _57: i64;
let _58: [u32; 4];
let _59: ();
let _60: ();
{
_12 = _5;
place!(Field::<i64>(Variant(_13, 0), 0)) = _8 * _7;
_10.2 = _3 + _9;
_10.2 = _1;
place!(Field::<isize>(Variant(_11, 0), 2)) = Field::<isize>(Variant(_6, 0), 2);
_8 = _10.2;
place!(Field::<usize>(Variant(_13, 0), 1)) = !Field::<usize>(Variant(_11, 0), 1);
_10.2 = _10.0 as i64;
_10.2 = _12 << _10.0;
_9 = _8 ^ _5;
place!(Field::<isize>(Variant(_6, 0), 2)) = _7 as isize;
RET = _10.5;
_5 = !Field::<i64>(Variant(_13, 0), 0);
_6 = Adt42::Variant0 { fld0: Field::<i64>(Variant(_13, 0), 0),fld1: Field::<usize>(Variant(_13, 0), 1),fld2: Field::<isize>(Variant(_13, 0), 2) };
place!(Field::<usize>(Variant(_11, 0), 1)) = Field::<i64>(Variant(_13, 0), 0) as usize;
_12 = !Field::<i64>(Variant(_6, 0), 0);
_10.0 = (-14017_i16);
_1 = -Field::<i64>(Variant(_13, 0), 0);
_2 = -_10.2;
SetDiscriminant(_13, 2);
_10.2 = 78_i8 as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.1 = _5 + _5;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).3 = 79237219037715866595838687603580949401_u128 as f64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.0 = _10.0 ^ _10.0;
Goto(bb1)
}
bb1 = {
RET = -_10.5;
place!(Field::<u8>(Variant(_13, 2), 0)) = 66801401870342646999616304509819777325_u128 as u8;
_4 = RET;
_12 = 81746533587489665459811411615303911193_i128 as i64;
place!(Field::<f32>(Variant(_13, 2), 4)) = Field::<u8>(Variant(_13, 2), 0) as f32;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).2 = core::ptr::addr_of!(_10.2);
place!(Field::<i64>(Variant(_6, 0), 0)) = _3 + _9;
place!(Field::<u16>(Variant(_13, 2), 3)) = !_10.4;
_14.3 = _10.3;
_12 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.1;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.3 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).3 as u32;
_14.0 = _10.0;
_21.1.0.2 = !Field::<i64>(Variant(_6, 0), 0);
_21.1.0.1 = _12;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).4 = (-43_i8) ^ 64_i8;
_10.5 = RET;
_10 = (_14.0, _12, _3, _14.3, Field::<u16>(Variant(_13, 2), 3), RET);
_10 = (Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.0, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.1, _8, _14.3, Field::<u16>(Variant(_13, 2), 3), _4);
_22 = [_10.0,Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.0,_10.0,_10.0,_14.0];
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.4 = Field::<u16>(Variant(_13, 2), 3);
match _14.0 {
0 => bb2,
340282366920938463463374607431768197439 => bb4,
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
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.5 = Field::<u8>(Variant(_13, 2), 0) as f32;
_21.0.3 = _10.3 >> _1;
_21.0.0 = _10.0 >> _21.1.0.1;
_21.1.5 = (_21.0.0, _1, _21.1.0.1, _21.0.3, _10.4, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.5);
_21.1.0.0 = _21.0.0;
place!(Field::<i64>(Variant(_6, 0), 0)) = _5 | _9;
_21.1.0.3 = Field::<isize>(Variant(_6, 0), 2) as u32;
_21.1.1 = Field::<u8>(Variant(_13, 2), 0);
place!(Field::<i64>(Variant(_11, 0), 0)) = false as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.2 = _21.1.0.1 ^ _5;
_29.1.0.4 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.4;
_13 = _11;
_29.1.2 = Field::<isize>(Variant(_11, 0), 2) & Field::<isize>(Variant(_13, 0), 2);
_29.1.0 = _21.1.5;
_29.1.5 = _10;
_14.4 = _10.4 + _29.1.5.4;
_10 = (_21.1.5.0, _21.1.5.2, _29.1.0.1, _29.1.0.3, _14.4, _29.1.0.5);
_25 = 2241785087218120053_u64;
Call(_10.0 = fn9(_21.1.5.1, _29.1.0.3, _12, _29.1.5.2, _29.1.0.1, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
SetDiscriminant(_13, 0);
_13 = _6;
_29.1.4 = Field::<usize>(Variant(_11, 0), 1) as f64;
place!(Field::<i64>(Variant(_11, 0), 0)) = _29.1.5.1 ^ _29.1.5.1;
_21.1.4 = _29.1.4 * _29.1.4;
_21.1.0.5 = _29.1.5.5;
_19 = (-11328475941744911914785867549600685735_i128) - (-59455836307002501352158508860988903504_i128);
_11 = _13;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_13, 0), 1) >> _21.1.5.2;
RET = _21.1.5.5 * _29.1.5.5;
_21.0.5 = _4 - _21.1.0.5;
_29.1.5.5 = RET;
_29.1.5.2 = _29.1.0.1 * Field::<i64>(Variant(_13, 0), 0);
SetDiscriminant(_13, 3);
_19 = 88034790724483325399864980545916019017_i128 | 138729794825531290808487023678631467143_i128;
_10.5 = _29.1.4 as f32;
_29.1.0.1 = _10.1;
place!(Field::<usize>(Variant(_6, 0), 1)) = Field::<usize>(Variant(_11, 0), 1) - Field::<usize>(Variant(_11, 0), 1);
_21.1.6 = _21.1.1 + _21.1.1;
match _14.0 {
0 => bb3,
1 => bb6,
340282366920938463463374607431768197439 => bb8,
_ => bb7
}
}
bb6 = {
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.5 = Field::<u8>(Variant(_13, 2), 0) as f32;
_21.0.3 = _10.3 >> _1;
_21.0.0 = _10.0 >> _21.1.0.1;
_21.1.5 = (_21.0.0, _1, _21.1.0.1, _21.0.3, _10.4, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.5);
_21.1.0.0 = _21.0.0;
place!(Field::<i64>(Variant(_6, 0), 0)) = _5 | _9;
_21.1.0.3 = Field::<isize>(Variant(_6, 0), 2) as u32;
_21.1.1 = Field::<u8>(Variant(_13, 2), 0);
place!(Field::<i64>(Variant(_11, 0), 0)) = false as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2)).1.2 = _21.1.0.1 ^ _5;
_29.1.0.4 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_13, 2), 2).1.4;
_13 = _11;
_29.1.2 = Field::<isize>(Variant(_11, 0), 2) & Field::<isize>(Variant(_13, 0), 2);
_29.1.0 = _21.1.5;
_29.1.5 = _10;
_14.4 = _10.4 + _29.1.5.4;
_10 = (_21.1.5.0, _21.1.5.2, _29.1.0.1, _29.1.0.3, _14.4, _29.1.0.5);
_25 = 2241785087218120053_u64;
Call(_10.0 = fn9(_21.1.5.1, _29.1.0.3, _12, _29.1.5.2, _29.1.0.1, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
Return()
}
bb8 = {
_31.0 = core::ptr::addr_of!(_14.1);
_21.1.4 = _29.1.4 + _29.1.4;
_29.1.3 = [_21.0.3,_21.1.5.3,_21.1.5.3,_29.1.0.3];
place!(Field::<char>(Variant(_13, 3), 1)) = '\u{8bd51}';
_31.0 = core::ptr::addr_of!(_21.0.2);
place!(Field::<[char; 6]>(Variant(_13, 3), 0)) = [Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1)];
_8 = Field::<i64>(Variant(_6, 0), 0) + _21.1.0.2;
SetDiscriminant(_11, 1);
_29.1.5.3 = _21.1.5.3;
_29.1.0.1 = -_1;
Call(place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).0 = fn10(_10.1, _21.1.5.3, _9, _8, _29.1.0.3, _21.1.0.2, Field::<char>(Variant(_13, 3), 1), _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3 = _9 << _29.1.0.3;
_21.0.2 = _1 - _21.1.0.2;
_29.1.0.0 = _21.1.5.0;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1.4 = !_10.4;
_29.1.5.4 = !Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).1.4;
_16 = _21.1.4 + _29.1.4;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1.2 = Field::<char>(Variant(_13, 3), 1) as i64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).3 = _21.1.5.3 as f64;
_29.0.2 = _29.1.5.1;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1.0 = _19 as i16;
_7 = _29.1.0.2 >> _21.1.5.2;
_38.1.1.4 = 26526157039750385239744939693897560211_u128 as u16;
_38.1.1.1 = -_10.1;
_29.0.0 = _25 as i16;
match _25 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
2241785087218120053 => bb10,
_ => bb8
}
}
bb10 = {
_7 = _21.0.2;
_37 = Field::<usize>(Variant(_6, 0), 1) >> _12;
_14.2 = _29.1.0.3 as i64;
_21.1 = (_10, 124_u8, _29.1.2, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).0, Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).3, _29.1.5, 217_u8);
_26 = Field::<char>(Variant(_13, 3), 1);
Goto(bb11)
}
bb11 = {
place!(Field::<i32>(Variant(_11, 1), 1)) = 1814072688_i32 ^ (-441044767_i32);
_41.fld2.1.4 = !_10.4;
_14.2 = _21.1.1 as i64;
_41.fld2.4 = !(-105_i8);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1.3 = _10.3 & _10.3;
_10.4 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).1.4;
_41.fld2.3 = _25 as f64;
_3 = !_29.0.2;
_21.1.5.5 = _10.5 * _29.1.5.5;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1.0 = _21.0.0 >> _21.1.0.1;
SetDiscriminant(_6, 0);
place!(Field::<isize>(Variant(_6, 0), 2)) = _29.1.2 | _21.1.2;
_41.fld2.1.3 = true as u32;
_30 = core::ptr::addr_of!(_41.fld2.1.3);
_38.1.2 = _31.0;
_38.1.1.5 = _10.5;
_21.0.1 = _37 as i64;
_21.1.0.1 = _37 as i64;
_32 = _16 as u16;
_29.1.0 = _21.1.5;
_7 = Field::<char>(Variant(_13, 3), 1) as i64;
_21.1.2 = !Field::<isize>(Variant(_6, 0), 2);
Call(_38.1.1.2 = core::intrinsics::transmute(_5), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_38.1.1 = (_10.0, _12, _2, _10.3, _32, _29.1.0.5);
_41.fld1.fld2 = _21.1.2 & Field::<isize>(Variant(_6, 0), 2);
_41.fld2.1.5 = _21.1.0.5;
place!(Field::<*const usize>(Variant(_11, 1), 2)) = core::ptr::addr_of!(_37);
_21.2 = [_38.1.1.0,Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).1.0,_10.0,_38.1.1.0,_21.1.0.0];
_41.fld1.fld0 = [_21.1.1,_21.1.6,_21.1.6,_21.1.6,_21.1.1];
_25 = !4658051265980745565_u64;
_29.0.3 = _41.fld2.1.5 as u32;
_21.0 = _29.1.5;
_29.1.0.4 = _32 + _38.1.1.4;
_41.fld2.1.2 = _10.1 << _29.1.0.4;
_29 = _21;
_37 = _29.1.0.0 as usize;
_41.fld1.fld5 = _31.0;
_32 = _38.1.1.4;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).2 = core::ptr::addr_of!(_1);
_41.fld2.1.1 = 147029885272083112522060420988725241095_u128 as i64;
_38.1.3 = Field::<i32>(Variant(_11, 1), 1) as f64;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).2 = core::ptr::addr_of!(_29.1.0.1);
_29 = (_21.0, _21.1, _21.2);
_49.6 = !_29.1.1;
_49.5.2 = _10.2 - _29.1.0.1;
_21.1.1 = Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).1.0 as u8;
match _29.1.1 {
0 => bb1,
124 => bb13,
_ => bb8
}
}
bb13 = {
_29.0.5 = _29.1.5.5 + _29.1.0.5;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).2 = _31.0;
place!(Field::<i32>(Variant(_11, 1), 1)) = 1860448081_i32;
_38.1.1.3 = _10.3 & Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0).1.3;
_21.1.5 = (_21.1.0.0, _10.1, _12, _29.1.5.3, _38.1.1.4, _38.1.1.5);
_31 = (_41.fld1.fld5, _21.1.5.5);
_29.0.3 = _38.1.1.3 | _29.1.0.3;
_38.1.1.1 = _21.1.0.2;
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1.3 = !_10.3;
_49.2 = _41.fld1.fld2;
_21.1.5.2 = _21.1.0.1;
_29.0.3 = _21.1.5.3 * _21.0.3;
_21.0.5 = -_29.1.5.5;
_45 = [Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),Field::<char>(Variant(_13, 3), 1),_26];
_14.5 = -_21.1.0.5;
Goto(bb14)
}
bb14 = {
_41.fld2.4 = (-119_i8);
place!(Field::<([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)>(Variant(_11, 1), 0)).1 = (_21.1.5.0, _49.5.2, _29.0.1, _21.1.0.3, _32, _14.5);
_49.5.0 = _41.fld2.4 as i16;
_36 = _41.fld1.fld2;
_21.1.5.3 = !_29.1.0.3;
_21.0.0 = _37 as i16;
Goto(bb15)
}
bb15 = {
Call(_59 = dump_var(8_usize, 45_usize, Move(_45), 9_usize, Move(_9), 37_usize, Move(_37), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_59 = dump_var(8_usize, 1_usize, Move(_1), 25_usize, Move(_25), 26_usize, Move(_26), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: i64,mut _2: u32,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64) -> i16 {
mir! {
type RET = i16;
let _7: isize;
let _8: Adt38;
let _9: (i16, i64, i64, u32, u16, f32);
let _10: isize;
let _11: f64;
let _12: i128;
let _13: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _14: (i16, i64, i64, u32, u16, f32);
let _15: char;
let _16: Adt49;
let _17: f64;
let _18: f32;
let _19: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _20: bool;
let _21: ();
let _22: ();
{
_1 = _5;
RET = 17032_i16 ^ (-904_i16);
_5 = 17917075173882844936_usize as i64;
_4 = !_1;
_1 = _2 as i64;
_4 = _6 + _3;
_9.5 = 89353556518568849869096957958145532353_u128 as f32;
_9.1 = -_1;
_4 = !_9.1;
_9.5 = _9.1 as f32;
_9.2 = _4;
Goto(bb1)
}
bb1 = {
_9.2 = (-1489271457_i32) as i64;
_4 = -_9.1;
_9.4 = 28725_u16 ^ 16673_u16;
_3 = _9.1 | _4;
_7 = -9223372036854775807_isize;
_2 = !3739131140_u32;
_10 = _7;
_9.2 = -_9.1;
_9.5 = 289886812125856014743980481786553100232_u128 as f32;
_1 = _9.1 | _4;
_6 = _4;
_9.0 = RET;
_9.3 = _2;
_1 = _9.1;
_10 = -_7;
_5 = _2 as i64;
_9.2 = -_6;
_12 = !(-155725016747018088703869318213893535910_i128);
_13.0.1 = -_4;
_13.1.0.4 = 1689690724_i32 as u16;
_13.0.2 = _1 & _4;
_13.0.3 = _2 >> _1;
_13.0.5 = _13.0.3 as f32;
_11 = 203335922641433374320660989152358277211_u128 as f64;
Goto(bb2)
}
bb2 = {
_13.1.2 = -_10;
_13.1.5.0 = RET ^ _9.0;
_9 = (RET, _6, _13.0.1, _13.0.3, _13.1.0.4, _13.0.5);
_9.1 = _9.2 << _3;
_13.0.0 = _13.1.5.0;
_5 = -_1;
_13.1.0.5 = -_9.5;
_13.1.2 = _7 + _10;
_13.1.5.0 = _13.0.0;
_9.5 = _13.0.5 * _13.1.0.5;
_13.1.5.0 = _13.0.0 ^ _13.0.0;
_5 = _12 as i64;
_3 = -_9.1;
_13.1.5.3 = !_9.3;
_13.1.5 = _9;
RET = _13.0.0;
_13.1.0.0 = -_9.0;
_4 = _13.0.1 >> _6;
_5 = _13.0.2;
_13.2 = [_13.1.0.0,RET,_13.0.0,RET,_13.1.0.0];
_13.1.0.5 = 16798780452110539718_u64 as f32;
_13.1.0 = (_9.0, _9.2, _9.1, _13.1.5.3, _9.4, _13.0.5);
_13.1.5.5 = 140213568454882899994136089561322473327_u128 as f32;
_13.2 = [RET,_13.1.0.0,RET,RET,RET];
_1 = !_13.0.2;
_13.1.5.0 = -_9.0;
_9.2 = -_13.1.0.2;
_13.1.3 = [_9.3,_13.1.0.3,_13.0.3,_13.1.5.3];
_9.1 = _13.1.0.2;
_13.1.4 = _7 as f64;
Call(_6 = core::intrinsics::bswap(_9.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13.1.5.2 = _9.1 ^ _9.1;
_13.1.5.0 = RET << _9.1;
_13.0.0 = _13.1.5.0 & _13.1.5.0;
_13.1.4 = _11;
_14.4 = _13.1.0.4 & _9.4;
_5 = _13.0.2 << _13.1.0.2;
_14.5 = _13.0.5 + _13.1.0.5;
_13.0.0 = _13.1.5.0;
_9.0 = !_13.0.0;
_17 = _13.1.4;
_13.1.5.0 = _9.0 + _9.0;
_14.3 = _13.1.5.3;
_19.1.0.4 = !_13.1.5.4;
_13.1.1 = 223_u8 >> _13.0.2;
_1 = _13.1.5.2;
_1 = _13.0.1 * _13.0.2;
RET = !_9.0;
Goto(bb4)
}
bb4 = {
Call(_21 = dump_var(9_usize, 4_usize, Move(_4), 2_usize, Move(_2), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i64,mut _2: u32,mut _3: i64,mut _4: i64,mut _5: u32,mut _6: i64,mut _7: char,mut _8: i64) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _9: Adt48;
let _10: char;
let _11: Adt45;
let _12: u8;
let _13: bool;
let _14: f32;
let _15: ();
let _16: ();
{
_5 = _2;
_1 = _3 | _4;
Goto(bb1)
}
bb1 = {
_6 = !_4;
_1 = 4352655369322981111_usize as i64;
RET = [_5,_2,_2,_2];
_3 = _4;
_8 = _6;
_2 = _5;
_1 = _6;
_1 = _6 << _3;
_8 = _3 & _4;
_10 = _7;
_11.fld2.1.1 = 8788483475443657450_u64 as i64;
_11.fld2.1.5 = 6512108480620925694_u64 as f32;
_11.fld1.fld3 = 202547832530216178_u64 ^ 8001387143656441939_u64;
_11.fld1.fld0 = [235_u8,149_u8,45_u8,51_u8,232_u8];
_11.fld2.1.3 = _5;
RET = [_11.fld2.1.3,_2,_11.fld2.1.3,_2];
_11.fld1.fld1 = _7;
_4 = _6 + _1;
_11.fld2.1.1 = 167382188513937044038629167952634248079_i128 as i64;
_11.fld2.3 = 32297_u16 as f64;
_11.fld2.4 = _11.fld1.fld3 as i8;
_11.fld1.fld5 = core::ptr::addr_of!(_1);
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(10_usize, 3_usize, Move(_3), 1_usize, Move(_1), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: f32,mut _2: usize,mut _3: i16,mut _4: bool,mut _5: bool,mut _6: *const i64) -> i64 {
mir! {
type RET = i64;
let _7: [u32; 4];
let _8: *mut char;
let _9: isize;
let _10: bool;
let _11: [isize; 3];
let _12: Adt44;
let _13: isize;
let _14: *const usize;
let _15: [i16; 5];
let _16: [char; 5];
let _17: i64;
let _18: bool;
let _19: isize;
let _20: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _21: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _22: i8;
let _23: Adt40;
let _24: i8;
let _25: [u8; 5];
let _26: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]));
let _27: bool;
let _28: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8);
let _29: isize;
let _30: isize;
let _31: bool;
let _32: u64;
let _33: Adt42;
let _34: [i16; 5];
let _35: [char; 6];
let _36: u8;
let _37: bool;
let _38: f32;
let _39: [isize; 3];
let _40: [u32; 4];
let _41: Adt39;
let _42: bool;
let _43: isize;
let _44: isize;
let _45: Adt43;
let _46: i128;
let _47: *const (*const i64, f32);
let _48: f64;
let _49: ();
let _50: ();
{
_7 = [2253577632_u32,1349103964_u32,1024969413_u32,360423246_u32];
RET = !8881615982068353315_i64;
_6 = core::ptr::addr_of!(RET);
_1 = (-152508576828016150123843708207859492155_i128) as f32;
_10 = !_4;
_5 = _4;
(*_6) = -4829963289259652497_i64;
(*_6) = 4256842845247210558_i64;
_9 = (-9223372036854775808_isize);
_1 = _9 as f32;
_9 = (-79_isize);
RET = (-4900175638596239939_i64) - 3063348024277328414_i64;
_1 = (-66080640998139573561457021507015598575_i128) as f32;
_7 = [2795783635_u32,2777218039_u32,3229016525_u32,4261804120_u32];
RET = _2 as i64;
_1 = 60975_u16 as f32;
_6 = core::ptr::addr_of!((*_6));
_9 = (-9223372036854775808_isize) - 62_isize;
_6 = core::ptr::addr_of!((*_6));
RET = -6738606882968593205_i64;
_2 = 3945758546703070293_usize + 1533534017352895903_usize;
_6 = core::ptr::addr_of!((*_6));
_2 = _1 as usize;
_9 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = 25041_i16;
_10 = !_5;
Call(_11 = fn12(_3, _3, _4, _2, _2, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = [51442633_u32,1677009788_u32,2527237517_u32,2517474357_u32];
RET = 5946269885022702501_i64;
_1 = _9 as f32;
_5 = _10;
_5 = _1 < _1;
_7 = [2293357346_u32,2577522384_u32,3781609976_u32,2081940213_u32];
_8 = core::ptr::addr_of_mut!(_12.fld1);
(*_8) = '\u{10ed9a}';
_6 = core::ptr::addr_of!((*_6));
_12.fld3 = 11599068501810625023_u64 << (*_6);
(*_8) = '\u{56e4}';
_7 = [325879838_u32,185418634_u32,4221553813_u32,262315588_u32];
RET = (-2483857529430561892_i64);
_12.fld1 = '\u{d8b93}';
Call(_12.fld2 = fn14((*_8), _12.fld3, (*_8), _11, _11, _7, (*_6), _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = (-7972762125557383478_i64);
_6 = core::ptr::addr_of!((*_6));
_12.fld0 = [78_u8,68_u8,138_u8,224_u8,147_u8];
_3 = (-20497_i16) & (-21195_i16);
_8 = core::ptr::addr_of_mut!((*_8));
(*_6) = -4489642704048952200_i64;
_20.1.2 = _2 as isize;
_20.1.2 = _12.fld2 * _12.fld2;
_20.0.2 = _12.fld3 as i64;
_20.1.6 = 217_u8 * 35_u8;
_20.0.0 = _3 + _3;
_10 = _4;
_20.0 = (_3, RET, (*_6), 2030510406_u32, 16539_u16, _1);
_23.fld1.0 = [_20.0.3,_20.0.3,_20.0.3,_20.0.3];
_20.1.0.2 = _20.0.3 as i64;
_13 = _12.fld3 as isize;
_20.0.3 = !1674771989_u32;
_26.6.1.6 = _20.1.6;
Goto(bb3)
}
bb3 = {
_26.6.1.5.5 = -_20.0.5;
_21.5.4 = !_20.0.4;
_26.6.1.6 = !_20.1.6;
_20.1.0 = _20.0;
_21.2 = !_12.fld2;
_7 = _23.fld1.0;
_21.5.3 = _3 as u32;
_12.fld4 = _2;
_26.6.0.4 = _21.5.4 & _20.1.0.4;
_26.2.1.1.1 = _1 as i64;
_17 = 1411757825_i32 as i64;
_15 = [_3,_20.0.0,_20.0.0,_20.1.0.0,_20.0.0];
_26.0.0.0 = _6;
_22 = -(-28_i8);
_28.0 = _7;
_26.2.1.1 = (_20.1.0.0, _20.0.1, _17, _21.5.3, _26.6.0.4, _26.6.1.5.5);
_29 = -_21.2;
_26.6.1.5.2 = _26.2.1.1.2 << _20.1.0.4;
_30 = !_12.fld2;
_21.5.0 = (-32590566966011335247829872474198150939_i128) as i16;
_20.1.0.4 = !_20.0.4;
Call(_26.0.1.1.0 = core::intrinsics::transmute(_26.6.0.4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_26.5 = [(*_8),_12.fld1,(*_8),_12.fld1,(*_8)];
_23.fld3 = 1754044996_i32 as i8;
_26.0.1.1.3 = _26.2.1.1.3;
_28.1.5 = _12.fld3 as f32;
_26.3 = [_20.1.0.3,_21.5.3,_20.1.0.3,_26.0.1.1.3];
_21.1 = _20.1.6 << _20.0.4;
_21.1 = _26.0.1.1.0 as u8;
_26.2.0.0 = core::ptr::addr_of!(_20.1.5.1);
_26.6.1.4 = _12.fld3 as f64;
Call(_20.1.0.0 = core::intrinsics::transmute(_3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = _26.6.1.5.5 as i64;
_26.2.1.1.1 = _26.6.1.5.2 << _26.2.1.1.4;
_23.fld1 = (_28.0, _26.2.1.1, _26.0.0.0, _26.6.1.4, _23.fld3);
_21.1 = _26.6.1.6 | _26.6.1.6;
_20.1.5.0 = (-1887454101_i32) as i16;
_26.6.0.4 = _21.5.4 & _23.fld1.1.4;
_20.1.5.4 = !_26.6.0.4;
_21.2 = _20.1.2;
_21.0.0 = _26.0.1.1.0;
_20.0.3 = !_26.2.1.1.3;
_26.6.0.3 = !_21.5.3;
_31 = _26.6.0.4 == _26.2.1.1.4;
_26.6.1.2 = _30;
_26.2.1.2 = _26.0.0.0;
_36 = _26.6.1.6;
_34 = [_26.0.1.1.0,_21.0.0,_26.0.1.1.0,_21.0.0,_21.0.0];
_23.fld0.0 = core::ptr::addr_of!(_21.0.1);
_23.fld0.1 = _20.1.0.5;
_20.1.4 = _23.fld1.3 - _23.fld1.3;
match _20.0.4 {
0 => bb4,
16539 => bb6,
_ => bb2
}
}
bb6 = {
_21.5.1 = _26.6.1.5.2;
_20.1 = (_23.fld1.1, _21.1, _26.6.1.2, _28.0, _23.fld1.3, _26.2.1.1, _21.1);
_20.2 = [_26.0.1.1.0,_26.0.1.1.0,_26.0.1.1.0,_3,_21.0.0];
_23.fld3 = _22;
_5 = !_31;
_26.0.1.1.2 = !_23.fld1.1.1;
_26.2.1.4 = -_23.fld3;
Call(_20.1.1 = fn15(_20.1.0.4, _26.3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_26.6.1.0.2 = -_20.1.5.1;
_26.6.1.0.0 = _21.0.0;
_23.fld0 = (_26.2.0.0, _20.1.0.5);
_26.0.1.0 = [_26.6.0.3,_23.fld1.1.3,_20.0.3,_26.6.0.3];
_26.6.0 = (_26.6.1.0.0, _26.0.1.1.2, _21.5.1, _26.0.1.1.3, _20.1.0.4, _28.1.5);
_26.1 = -_1;
_21.0.1 = (*_6) + _21.5.1;
_26.0.0 = _23.fld0;
_23.fld3 = (-148703355526461797748276468342191147557_i128) as i8;
_38 = _21.0.0 as f32;
_21.5.4 = _20.1.0.4 & _23.fld1.1.4;
_26.6.1.2 = _20.1.2;
_28.1.1 = _20.1.0.1;
_26.2.1.1.5 = _38 - _38;
_21.5.2 = _20.1.5.1;
_27 = !_5;
match _20.0.4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb8,
5 => bb9,
6 => bb10,
16539 => bb12,
_ => bb11
}
}
bb8 = {
RET = (-7972762125557383478_i64);
_6 = core::ptr::addr_of!((*_6));
_12.fld0 = [78_u8,68_u8,138_u8,224_u8,147_u8];
_3 = (-20497_i16) & (-21195_i16);
_8 = core::ptr::addr_of_mut!((*_8));
(*_6) = -4489642704048952200_i64;
_20.1.2 = _2 as isize;
_20.1.2 = _12.fld2 * _12.fld2;
_20.0.2 = _12.fld3 as i64;
_20.1.6 = 217_u8 * 35_u8;
_20.0.0 = _3 + _3;
_10 = _4;
_20.0 = (_3, RET, (*_6), 2030510406_u32, 16539_u16, _1);
_23.fld1.0 = [_20.0.3,_20.0.3,_20.0.3,_20.0.3];
_20.1.0.2 = _20.0.3 as i64;
_13 = _12.fld3 as isize;
_20.0.3 = !1674771989_u32;
_26.6.1.6 = _20.1.6;
Goto(bb3)
}
bb9 = {
RET = _26.6.1.5.5 as i64;
_26.2.1.1.1 = _26.6.1.5.2 << _26.2.1.1.4;
_23.fld1 = (_28.0, _26.2.1.1, _26.0.0.0, _26.6.1.4, _23.fld3);
_21.1 = _26.6.1.6 | _26.6.1.6;
_20.1.5.0 = (-1887454101_i32) as i16;
_26.6.0.4 = _21.5.4 & _23.fld1.1.4;
_20.1.5.4 = !_26.6.0.4;
_21.2 = _20.1.2;
_21.0.0 = _26.0.1.1.0;
_20.0.3 = !_26.2.1.1.3;
_26.6.0.3 = !_21.5.3;
_31 = _26.6.0.4 == _26.2.1.1.4;
_26.6.1.2 = _30;
_26.2.1.2 = _26.0.0.0;
_36 = _26.6.1.6;
_34 = [_26.0.1.1.0,_21.0.0,_26.0.1.1.0,_21.0.0,_21.0.0];
_23.fld0.0 = core::ptr::addr_of!(_21.0.1);
_23.fld0.1 = _20.1.0.5;
_20.1.4 = _23.fld1.3 - _23.fld1.3;
match _20.0.4 {
0 => bb4,
16539 => bb6,
_ => bb2
}
}
bb10 = {
_26.5 = [(*_8),_12.fld1,(*_8),_12.fld1,(*_8)];
_23.fld3 = 1754044996_i32 as i8;
_26.0.1.1.3 = _26.2.1.1.3;
_28.1.5 = _12.fld3 as f32;
_26.3 = [_20.1.0.3,_21.5.3,_20.1.0.3,_26.0.1.1.3];
_21.1 = _20.1.6 << _20.0.4;
_21.1 = _26.0.1.1.0 as u8;
_26.2.0.0 = core::ptr::addr_of!(_20.1.5.1);
_26.6.1.4 = _12.fld3 as f64;
Call(_20.1.0.0 = core::intrinsics::transmute(_3), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_26.6.1.5.5 = -_20.0.5;
_21.5.4 = !_20.0.4;
_26.6.1.6 = !_20.1.6;
_20.1.0 = _20.0;
_21.2 = !_12.fld2;
_7 = _23.fld1.0;
_21.5.3 = _3 as u32;
_12.fld4 = _2;
_26.6.0.4 = _21.5.4 & _20.1.0.4;
_26.2.1.1.1 = _1 as i64;
_17 = 1411757825_i32 as i64;
_15 = [_3,_20.0.0,_20.0.0,_20.1.0.0,_20.0.0];
_26.0.0.0 = _6;
_22 = -(-28_i8);
_28.0 = _7;
_26.2.1.1 = (_20.1.0.0, _20.0.1, _17, _21.5.3, _26.6.0.4, _26.6.1.5.5);
_29 = -_21.2;
_26.6.1.5.2 = _26.2.1.1.2 << _20.1.0.4;
_30 = !_12.fld2;
_21.5.0 = (-32590566966011335247829872474198150939_i128) as i16;
_20.1.0.4 = !_20.0.4;
Call(_26.0.1.1.0 = core::intrinsics::transmute(_26.6.0.4), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_17 = -_26.2.1.1.1;
_8 = core::ptr::addr_of_mut!(_12.fld1);
_18 = !_27;
_26.6.1.0.4 = !_26.2.1.1.4;
_26.2.1.0 = [_26.0.1.1.3,_21.5.3,_21.5.3,_26.0.1.1.3];
_26.6.1.5 = _20.0;
_26.0.1.1.1 = !_21.0.1;
_26.6 = (_26.2.1.1, _20.1, _20.2);
_26.2.1.1.1 = _26.6.0.1;
_26.0.1.1 = (_20.0.0, _26.2.1.1.1, _26.6.1.0.1, _26.6.1.5.3, _26.2.1.1.4, _26.6.0.5);
_20.1.0 = (_21.0.0, _21.5.1, _20.1.5.1, _26.6.1.0.3, _20.1.5.4, _23.fld1.1.5);
_19 = _12.fld2 ^ _21.2;
_26.6.1.5.2 = _26.6.0.1 & _20.1.5.1;
match _20.0.4 {
0 => bb11,
1 => bb10,
2 => bb3,
3 => bb13,
4 => bb14,
5 => bb15,
16539 => bb17,
_ => bb16
}
}
bb13 = {
_26.6.1.5.5 = -_20.0.5;
_21.5.4 = !_20.0.4;
_26.6.1.6 = !_20.1.6;
_20.1.0 = _20.0;
_21.2 = !_12.fld2;
_7 = _23.fld1.0;
_21.5.3 = _3 as u32;
_12.fld4 = _2;
_26.6.0.4 = _21.5.4 & _20.1.0.4;
_26.2.1.1.1 = _1 as i64;
_17 = 1411757825_i32 as i64;
_15 = [_3,_20.0.0,_20.0.0,_20.1.0.0,_20.0.0];
_26.0.0.0 = _6;
_22 = -(-28_i8);
_28.0 = _7;
_26.2.1.1 = (_20.1.0.0, _20.0.1, _17, _21.5.3, _26.6.0.4, _26.6.1.5.5);
_29 = -_21.2;
_26.6.1.5.2 = _26.2.1.1.2 << _20.1.0.4;
_30 = !_12.fld2;
_21.5.0 = (-32590566966011335247829872474198150939_i128) as i16;
_20.1.0.4 = !_20.0.4;
Call(_26.0.1.1.0 = core::intrinsics::transmute(_26.6.0.4), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_26.6.1.5.5 = -_20.0.5;
_21.5.4 = !_20.0.4;
_26.6.1.6 = !_20.1.6;
_20.1.0 = _20.0;
_21.2 = !_12.fld2;
_7 = _23.fld1.0;
_21.5.3 = _3 as u32;
_12.fld4 = _2;
_26.6.0.4 = _21.5.4 & _20.1.0.4;
_26.2.1.1.1 = _1 as i64;
_17 = 1411757825_i32 as i64;
_15 = [_3,_20.0.0,_20.0.0,_20.1.0.0,_20.0.0];
_26.0.0.0 = _6;
_22 = -(-28_i8);
_28.0 = _7;
_26.2.1.1 = (_20.1.0.0, _20.0.1, _17, _21.5.3, _26.6.0.4, _26.6.1.5.5);
_29 = -_21.2;
_26.6.1.5.2 = _26.2.1.1.2 << _20.1.0.4;
_30 = !_12.fld2;
_21.5.0 = (-32590566966011335247829872474198150939_i128) as i16;
_20.1.0.4 = !_20.0.4;
Call(_26.0.1.1.0 = core::intrinsics::transmute(_26.6.0.4), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
RET = _26.6.1.5.5 as i64;
_26.2.1.1.1 = _26.6.1.5.2 << _26.2.1.1.4;
_23.fld1 = (_28.0, _26.2.1.1, _26.0.0.0, _26.6.1.4, _23.fld3);
_21.1 = _26.6.1.6 | _26.6.1.6;
_20.1.5.0 = (-1887454101_i32) as i16;
_26.6.0.4 = _21.5.4 & _23.fld1.1.4;
_20.1.5.4 = !_26.6.0.4;
_21.2 = _20.1.2;
_21.0.0 = _26.0.1.1.0;
_20.0.3 = !_26.2.1.1.3;
_26.6.0.3 = !_21.5.3;
_31 = _26.6.0.4 == _26.2.1.1.4;
_26.6.1.2 = _30;
_26.2.1.2 = _26.0.0.0;
_36 = _26.6.1.6;
_34 = [_26.0.1.1.0,_21.0.0,_26.0.1.1.0,_21.0.0,_21.0.0];
_23.fld0.0 = core::ptr::addr_of!(_21.0.1);
_23.fld0.1 = _20.1.0.5;
_20.1.4 = _23.fld1.3 - _23.fld1.3;
match _20.0.4 {
0 => bb4,
16539 => bb6,
_ => bb2
}
}
bb16 = {
_21.5.1 = _26.6.1.5.2;
_20.1 = (_23.fld1.1, _21.1, _26.6.1.2, _28.0, _23.fld1.3, _26.2.1.1, _21.1);
_20.2 = [_26.0.1.1.0,_26.0.1.1.0,_26.0.1.1.0,_3,_21.0.0];
_23.fld3 = _22;
_5 = !_31;
_26.0.1.1.2 = !_23.fld1.1.1;
_26.2.1.4 = -_23.fld3;
Call(_20.1.1 = fn15(_20.1.0.4, _26.3), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
_43 = _23.fld0.1 as isize;
_1 = -_26.2.1.1.5;
_26.0.1.4 = -_26.2.1.4;
_26.6.0.5 = _26.2.1.1.5;
_20.0.1 = !_26.6.1.0.1;
RET = _20.0.3 as i64;
_20.0.1 = _23.fld1.1.1;
_26.2.1.2 = _6;
_25 = _12.fld0;
_12.fld0 = [_20.1.6,_20.1.6,_20.1.6,_20.1.6,_20.1.1];
_3 = _26.6.1.6 as i16;
_20.1.0.5 = -_1;
_26.6.1.5 = (_23.fld1.1.0, _26.2.1.1.1, _26.0.1.1.2, _23.fld1.1.3, _20.0.4, _20.1.0.5);
_39 = [_29,_29,_19];
_20.1.0.4 = _26.2.1.1.4 << _20.1.5.4;
_21.6 = (*_8) as u8;
_26.6.1.6 = _21.1 + _26.6.1.1;
_20.0.0 = _20.1.0.0 & _21.0.0;
_26.2.1.1.0 = _20.0.0 * _26.0.1.1.0;
Goto(bb18)
}
bb18 = {
Call(_49 = dump_var(11_usize, 17_usize, Move(_17), 13_usize, Move(_13), 22_usize, Move(_22), 36_usize, Move(_36)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_49 = dump_var(11_usize, 5_usize, Move(_5), 39_usize, Move(_39), 2_usize, Move(_2), 30_usize, Move(_30)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_49 = dump_var(11_usize, 25_usize, Move(_25), 7_usize, Move(_7), 43_usize, Move(_43), 50_usize, _50), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i16,mut _2: i16,mut _3: bool,mut _4: usize,mut _5: usize,mut _6: bool) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _7: isize;
let _8: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8);
let _9: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _10: Adt43;
let _11: isize;
let _12: f32;
let _13: char;
let _14: isize;
let _15: [u32; 4];
let _16: [char; 6];
let _17: Adt47;
let _18: isize;
let _19: isize;
let _20: &'static usize;
let _21: Adt54;
let _22: ();
let _23: ();
{
RET = [9223372036854775807_isize,(-46_isize),(-9223372036854775808_isize)];
_4 = !_5;
RET = [(-9223372036854775808_isize),(-11_isize),9223372036854775807_isize];
_7 = 9223372036854775807_isize | (-9223372036854775808_isize);
_8.1.2 = 6504429271974600389_i64 ^ (-5059387224062984710_i64);
_9.1.2 = -_7;
_5 = _4 - _4;
_9.1.5.2 = !_8.1.2;
_8.3 = 881605004_u32 as f64;
_7 = _9.1.2 | _9.1.2;
_9.1.5.3 = 1883403027_u32;
_8.1.3 = _9.1.5.3;
_9.1.0.4 = !26637_u16;
_9.1.0.2 = _9.1.5.2 + _9.1.5.2;
_8.0 = [_8.1.3,_8.1.3,_8.1.3,_8.1.3];
_8.4 = -3_i8;
_7 = 13552554705485424496_u64 as isize;
_9.1.5.5 = _8.3 as f32;
_9.1.5.4 = !_9.1.0.4;
_9.1.0.0 = 221801868750484893181517568342315957941_u128 as i16;
_8.4 = (-104_i8) * (-55_i8);
_9.2 = [_1,_9.1.0.0,_2,_9.1.0.0,_1];
_9.0 = (_9.1.0.0, _9.1.0.2, _9.1.0.2, _9.1.5.3, _9.1.0.4, _9.1.5.5);
match _9.0.3 {
1883403027 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_9.1.5 = (_1, _9.0.1, _9.0.1, _9.0.3, _9.1.0.4, _9.0.5);
_9.1.0.2 = !_9.1.5.2;
_9.1.5.0 = _9.1.0.0 & _1;
_9.0.1 = _8.1.3 as i64;
_7 = _9.1.2;
_7 = _9.1.2 & _9.1.2;
_12 = _9.0.5;
_9.1.3 = [_9.1.5.3,_8.1.3,_8.1.3,_9.0.3];
_9.1.0.1 = !_8.1.2;
RET = [_7,_7,_7];
_8.1.4 = !_9.0.4;
_9.1.6 = 26_u8 * 173_u8;
_8.2 = core::ptr::addr_of!(_9.0.1);
_5 = _4 & _4;
_6 = _3;
_12 = _9.1.5.5 + _9.1.5.5;
_9.1.5.3 = !_9.0.3;
_9.1.5.5 = _12 * _12;
_15 = [_9.1.5.3,_8.1.3,_8.1.3,_9.1.5.3];
_12 = _9.1.5.5 - _9.1.5.5;
_8.1.5 = _8.3 as f32;
_9.1.0 = _9.0;
_9.0.3 = !_9.1.5.3;
Call(_12 = fn13(_9.0.2, _8.4, _8.1.2, RET, _9.0.0, RET, RET, _4, _9.1.0.0, _2, _9.1.5.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.1.5.5 = -_9.1.0.5;
_8.1 = (_1, _9.1.5.2, _9.1.0.1, _9.1.5.3, _9.0.4, _9.1.5.5);
_12 = _9.0.3 as f32;
_9.1.0.1 = _9.1.5.1;
_9.0.1 = _9.1.5.2 & _9.1.5.1;
_8.4 = 117_i8;
_14 = _7 * _9.1.2;
_8.1.1 = _9.1.0.1 & _9.1.5.2;
_9.1.0.4 = _9.1.5.4;
_2 = -_9.0.0;
RET = [_14,_14,_9.1.2];
Goto(bb4)
}
bb4 = {
RET = [_7,_14,_7];
_7 = _9.1.2 | _14;
_8.1.2 = _8.3 as i64;
_12 = _8.3 as f32;
match _8.1.0 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
25041 => bb12,
_ => bb11
}
}
bb5 = {
_9.1.5.5 = -_9.1.0.5;
_8.1 = (_1, _9.1.5.2, _9.1.0.1, _9.1.5.3, _9.0.4, _9.1.5.5);
_12 = _9.0.3 as f32;
_9.1.0.1 = _9.1.5.1;
_9.0.1 = _9.1.5.2 & _9.1.5.1;
_8.4 = 117_i8;
_14 = _7 * _9.1.2;
_8.1.1 = _9.1.0.1 & _9.1.5.2;
_9.1.0.4 = _9.1.5.4;
_2 = -_9.0.0;
RET = [_14,_14,_9.1.2];
Goto(bb4)
}
bb6 = {
_9.1.5 = (_1, _9.0.1, _9.0.1, _9.0.3, _9.1.0.4, _9.0.5);
_9.1.0.2 = !_9.1.5.2;
_9.1.5.0 = _9.1.0.0 & _1;
_9.0.1 = _8.1.3 as i64;
_7 = _9.1.2;
_7 = _9.1.2 & _9.1.2;
_12 = _9.0.5;
_9.1.3 = [_9.1.5.3,_8.1.3,_8.1.3,_9.0.3];
_9.1.0.1 = !_8.1.2;
RET = [_7,_7,_7];
_8.1.4 = !_9.0.4;
_9.1.6 = 26_u8 * 173_u8;
_8.2 = core::ptr::addr_of!(_9.0.1);
_5 = _4 & _4;
_6 = _3;
_12 = _9.1.5.5 + _9.1.5.5;
_9.1.5.3 = !_9.0.3;
_9.1.5.5 = _12 * _12;
_15 = [_9.1.5.3,_8.1.3,_8.1.3,_9.1.5.3];
_12 = _9.1.5.5 - _9.1.5.5;
_8.1.5 = _8.3 as f32;
_9.1.0 = _9.0;
_9.0.3 = !_9.1.5.3;
Call(_12 = fn13(_9.0.2, _8.4, _8.1.2, RET, _9.0.0, RET, RET, _4, _9.1.0.0, _2, _9.1.5.3), ReturnTo(bb3), UnwindUnreachable())
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
Return()
}
bb12 = {
_16 = ['\u{92603}','\u{8eb38}','\u{d79d1}','\u{10fdd7}','\u{3ebaa}','\u{83f94}'];
_9.1.0.5 = _8.1.1 as f32;
_18 = !_7;
_9.1.2 = _18 >> _9.0.1;
_9.2 = [_8.1.0,_2,_8.1.0,_9.1.5.0,_1];
match _1 {
0 => bb6,
25041 => bb13,
_ => bb4
}
}
bb13 = {
_9.2 = [_1,_1,_8.1.0,_8.1.0,_2];
_9.1.5.0 = !_8.1.0;
_9.1.1 = _8.3 as u8;
_9.1.0.1 = -_9.0.1;
_3 = _18 > _14;
Goto(bb14)
}
bb14 = {
_9.1.2 = _18 >> _9.1.1;
_9.0.5 = -_9.1.0.5;
_9.2 = [_2,_8.1.0,_9.0.0,_9.0.0,_9.1.0.0];
_8.0 = _9.1.3;
_9.1 = (_9.0, 224_u8, _18, _8.0, _8.3, _8.1, 47_u8);
_9.0 = (_8.1.0, _9.1.0.1, _9.1.0.2, _9.1.0.3, _8.1.4, _9.1.0.5);
_9.1.0.1 = 1655069638_i32 as i64;
_20 = &_4;
_5 = (*_20);
_9.1.5.0 = _2 * _2;
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(12_usize, 14_usize, Move(_14), 3_usize, Move(_3), 2_usize, Move(_2), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(12_usize, 5_usize, Move(_5), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i64,mut _2: i8,mut _3: i64,mut _4: [isize; 3],mut _5: i16,mut _6: [isize; 3],mut _7: [isize; 3],mut _8: usize,mut _9: i16,mut _10: i16,mut _11: u32) -> f32 {
mir! {
type RET = f32;
let _12: u16;
let _13: isize;
let _14: char;
let _15: [u8; 5];
let _16: Adt46;
let _17: [i16; 5];
let _18: [u8; 5];
let _19: usize;
let _20: i8;
let _21: Adt54;
let _22: i32;
let _23: bool;
let _24: [u32; 4];
let _25: char;
let _26: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _27: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _28: i128;
let _29: f64;
let _30: [isize; 3];
let _31: [isize; 3];
let _32: ();
let _33: ();
{
RET = _2 as f32;
_5 = _9;
_8 = 4_usize * 2_usize;
_1 = -_3;
_6 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_1 = !_3;
_12 = 52982_u16;
RET = (-9223372036854775808_isize) as f32;
_10 = _12 as i16;
_4 = _7;
RET = _11 as f32;
RET = _12 as f32;
_2 = !(-83_i8);
_1 = -_3;
RET = _1 as f32;
_10 = RET as i16;
_7 = _6;
_6 = [54_isize,62_isize,39_isize];
_4 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_9 = _10;
_13 = (-28_isize);
_13 = -(-9223372036854775808_isize);
_11 = !2901925017_u32;
_4 = [_13,_13,_13];
_9 = _10 | _10;
_14 = '\u{d315c}';
_9 = !_10;
_13 = (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_1 = !_3;
_11 = 73405199270626335992022467212275152283_i128 as u32;
_7 = [_13,_13,_13];
Goto(bb2)
}
bb2 = {
RET = _12 as f32;
_8 = false as usize;
_9 = (-1232616616_i32) as i16;
RET = _1 as f32;
_15 = [154_u8,243_u8,32_u8,163_u8,94_u8];
match _12 {
0 => bb1,
1 => bb3,
52982 => bb5,
_ => bb4
}
}
bb3 = {
_1 = !_3;
_11 = 73405199270626335992022467212275152283_i128 as u32;
_7 = [_13,_13,_13];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_8 = !6_usize;
_17 = [_9,_5,_10,_9,_9];
_2 = _9 as i8;
match _13 {
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb6 = {
_1 = !_3;
_11 = 73405199270626335992022467212275152283_i128 as u32;
_7 = [_13,_13,_13];
Goto(bb2)
}
bb7 = {
_11 = 217_u8 as u32;
_2 = (-46_i8);
_12 = 3287_u16 + 9746_u16;
_11 = 167_u8 as u32;
match _2 {
0 => bb3,
340282366920938463463374607431768211410 => bb9,
_ => bb8
}
}
bb8 = {
_1 = !_3;
_11 = 73405199270626335992022467212275152283_i128 as u32;
_7 = [_13,_13,_13];
Goto(bb2)
}
bb9 = {
_10 = _5;
_15 = [31_u8,103_u8,251_u8,75_u8,225_u8];
_2 = -(-127_i8);
_10 = 119_u8 as i16;
_19 = _8;
_11 = 587688685_u32;
_3 = _11 as i64;
_18 = [108_u8,247_u8,238_u8,178_u8,77_u8];
_11 = !644256592_u32;
_13 = 49_isize ^ 9223372036854775807_isize;
_8 = _19 ^ _19;
_2 = !88_i8;
RET = 115107244764070877672879763614226871870_u128 as f32;
_17 = [_10,_9,_10,_5,_9];
RET = _11 as f32;
_17 = [_10,_9,_5,_5,_10];
_7 = _4;
_14 = '\u{5e75d}';
_13 = (-9223372036854775808_isize);
RET = 189_u8 as f32;
_4 = [_13,_13,_13];
Call(_9 = core::intrinsics::transmute(_10), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9 = _5 << _10;
_5 = -_9;
_22 = RET as i32;
_22 = -959123699_i32;
_23 = _11 >= _11;
_9 = _1 as i16;
_20 = _22 as i8;
_2 = _12 as i8;
_14 = '\u{2549a}';
_14 = '\u{10a323}';
_2 = _20;
_8 = _19;
_15 = [243_u8,185_u8,89_u8,210_u8,57_u8];
_2 = _20;
_14 = '\u{28591}';
_3 = -_1;
_18 = [176_u8,99_u8,7_u8,121_u8,74_u8];
_17 = [_9,_5,_5,_5,_5];
_24 = [_11,_11,_11,_11];
_2 = _20;
_6 = _4;
_15 = [67_u8,189_u8,236_u8,36_u8,187_u8];
RET = 163772417849248401639611099514470362012_i128 as f32;
Goto(bb11)
}
bb11 = {
_12 = !15626_u16;
_8 = _19 ^ _19;
_18 = _15;
_11 = _8 as u32;
_24 = [_11,_11,_11,_11];
_14 = '\u{40e20}';
_2 = _20;
RET = _22 as f32;
_23 = !false;
_10 = 12288261041563886777505597115732696098_i128 as i16;
_24 = [_11,_11,_11,_11];
RET = _22 as f32;
_7 = [_13,_13,_13];
_3 = _1 >> _5;
_4 = [_13,_13,_13];
_20 = _2;
_4 = _7;
_26.1.0.1 = _3;
_14 = '\u{87642}';
match _13 {
0 => bb5,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb12 = {
_9 = _5 << _10;
_5 = -_9;
_22 = RET as i32;
_22 = -959123699_i32;
_23 = _11 >= _11;
_9 = _1 as i16;
_20 = _22 as i8;
_2 = _12 as i8;
_14 = '\u{2549a}';
_14 = '\u{10a323}';
_2 = _20;
_8 = _19;
_15 = [243_u8,185_u8,89_u8,210_u8,57_u8];
_2 = _20;
_14 = '\u{28591}';
_3 = -_1;
_18 = [176_u8,99_u8,7_u8,121_u8,74_u8];
_17 = [_9,_5,_5,_5,_5];
_24 = [_11,_11,_11,_11];
_2 = _20;
_6 = _4;
_15 = [67_u8,189_u8,236_u8,36_u8,187_u8];
RET = 163772417849248401639611099514470362012_i128 as f32;
Goto(bb11)
}
bb13 = {
_1 = !_3;
_11 = 73405199270626335992022467212275152283_i128 as u32;
_7 = [_13,_13,_13];
Goto(bb2)
}
bb14 = {
RET = _12 as f32;
_8 = false as usize;
_9 = (-1232616616_i32) as i16;
RET = _1 as f32;
_15 = [154_u8,243_u8,32_u8,163_u8,94_u8];
match _12 {
0 => bb1,
1 => bb3,
52982 => bb5,
_ => bb4
}
}
bb15 = {
_11 = 217_u8 as u32;
_2 = (-46_i8);
_12 = 3287_u16 + 9746_u16;
_11 = 167_u8 as u32;
match _2 {
0 => bb3,
340282366920938463463374607431768211410 => bb9,
_ => bb8
}
}
bb16 = {
_1 = !_3;
_11 = 73405199270626335992022467212275152283_i128 as u32;
_7 = [_13,_13,_13];
Goto(bb2)
}
bb17 = {
_8 = !6_usize;
_17 = [_9,_5,_10,_9,_9];
_2 = _9 as i8;
match _13 {
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb18 = {
Return()
}
bb19 = {
_27.0.0 = !_10;
_17 = [_10,_5,_9,_9,_5];
_26.1.5.5 = RET;
_26.1.5.4 = !_12;
_26.1.0.4 = _12;
_27.0.5 = RET;
_7 = [_13,_13,_13];
_7 = _6;
_27.0.3 = _11;
_25 = _14;
_26.1.5.0 = -_9;
_29 = 274240506514696795926145850349934683653_u128 as f64;
_27.0.0 = _26.1.5.0 + _26.1.5.0;
_27.1.6 = 18364033572590676979_u64 as u8;
_27.1.3 = [_27.0.3,_27.0.3,_27.0.3,_27.0.3];
_7 = _4;
_27.0.0 = RET as i16;
_26.1.0.2 = _19 as i64;
_27.1.0 = (_9, _26.1.0.1, _26.1.0.1, _11, _26.1.0.4, _26.1.5.5);
_20 = -_2;
_27.1.5.3 = _20 as u32;
_26.1.0.2 = !_3;
_27.1.1 = _27.1.6;
_15 = _18;
_6 = [_13,_13,_13];
_26.1.5.4 = _26.1.0.4;
_23 = _26.1.5.4 > _26.1.5.4;
Goto(bb20)
}
bb20 = {
Call(_32 = dump_var(13_usize, 14_usize, Move(_14), 8_usize, Move(_8), 10_usize, Move(_10), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(13_usize, 13_usize, Move(_13), 9_usize, Move(_9), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_32 = dump_var(13_usize, 3_usize, Move(_3), 17_usize, Move(_17), 15_usize, Move(_15), 33_usize, _33), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: char,mut _2: u64,mut _3: char,mut _4: [isize; 3],mut _5: [isize; 3],mut _6: [u32; 4],mut _7: i64,mut _8: bool) -> isize {
mir! {
type RET = isize;
let _9: isize;
let _10: [char; 6];
let _11: i16;
let _12: f64;
let _13: f64;
let _14: i32;
let _15: i16;
let _16: Adt45;
let _17: Adt49;
let _18: (i16, i64, i64, u32, u16, f32);
let _19: *const usize;
let _20: char;
let _21: [u8; 5];
let _22: u8;
let _23: ();
let _24: ();
{
RET = _1 as isize;
_1 = _3;
_3 = _1;
_5 = [RET,RET,RET];
_4 = _5;
_6 = [478781063_u32,541477005_u32,2910424838_u32,2087521851_u32];
_1 = _3;
_8 = !true;
_6 = [3514837504_u32,3149011431_u32,2615563561_u32,693895971_u32];
_4 = _5;
_1 = _3;
_1 = _3;
_1 = _3;
Goto(bb1)
}
bb1 = {
_8 = true ^ false;
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _7 < _7;
RET = 9223372036854775807_isize << _2;
Goto(bb3)
}
bb3 = {
_6 = [3845812189_u32,4125205252_u32,3203106544_u32,4096532823_u32];
_8 = _1 <= _3;
_5 = [RET,RET,RET];
_9 = !RET;
_7 = 3321299327578384231_i64 + (-3651909773612880676_i64);
_9 = _2 as isize;
_7 = (-2758235632497677809_i64);
Goto(bb4)
}
bb4 = {
_2 = (-109_i8) as u64;
_4 = [RET,RET,RET];
_10 = [_3,_1,_3,_3,_1,_3];
_5 = [RET,_9,_9];
_1 = _3;
_11 = !(-2540_i16);
_1 = _3;
match _7 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463460616371799270533647 => bb11,
_ => bb10
}
}
bb5 = {
_6 = [3845812189_u32,4125205252_u32,3203106544_u32,4096532823_u32];
_8 = _1 <= _3;
_5 = [RET,RET,RET];
_9 = !RET;
_7 = 3321299327578384231_i64 + (-3651909773612880676_i64);
_9 = _2 as isize;
_7 = (-2758235632497677809_i64);
Goto(bb4)
}
bb6 = {
_8 = _7 < _7;
RET = 9223372036854775807_isize << _2;
Goto(bb3)
}
bb7 = {
_8 = true ^ false;
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
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
_12 = RET as f64;
_4 = [RET,RET,RET];
_11 = 30302_i16;
_8 = true | false;
_3 = _1;
_3 = _1;
_4 = _5;
_6 = [3017516828_u32,2371182777_u32,1171642538_u32,3656330484_u32];
_5 = [_9,RET,_9];
_5 = _4;
_1 = _3;
_6 = [317846960_u32,1044080459_u32,3024966712_u32,3498281288_u32];
_13 = _12 * _12;
_3 = _1;
_2 = 2590761509994308814_u64 << _9;
_11 = 9997_i16 << RET;
_3 = _1;
_7 = 44680554284433064_i64;
_15 = _11 ^ _11;
_16.fld2.2 = core::ptr::addr_of!(_16.fld2.1.2);
_16.fld3 = Adt42::Variant3 { fld0: _10,fld1: _1,fld2: 82516051582666541121362212992242770165_i128 };
_16.fld2.1.5 = _12 as f32;
_16.fld2.1.1 = !_7;
match _7 {
0 => bb9,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
44680554284433064 => bb18,
_ => bb17
}
}
bb12 = {
_6 = [3845812189_u32,4125205252_u32,3203106544_u32,4096532823_u32];
_8 = _1 <= _3;
_5 = [RET,RET,RET];
_9 = !RET;
_7 = 3321299327578384231_i64 + (-3651909773612880676_i64);
_9 = _2 as isize;
_7 = (-2758235632497677809_i64);
Goto(bb4)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_8 = true ^ false;
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_8 = _7 < _7;
RET = 9223372036854775807_isize << _2;
Goto(bb3)
}
bb17 = {
_8 = true ^ false;
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_16.fld2.1.4 = !14925_u16;
_16.fld2.1.0 = _15 & _15;
_16.fld1.fld1 = _1;
_16.fld3 = Adt42::Variant0 { fld0: _7,fld1: 3768185688669545372_usize,fld2: RET };
RET = Field::<isize>(Variant(_16.fld3, 0), 2) - Field::<isize>(Variant(_16.fld3, 0), 2);
_7 = -_16.fld2.1.1;
_2 = 6537617163731167635_u64;
_14 = -1716686244_i32;
_4 = [RET,_9,RET];
_16.fld0 = 0_usize | 9801664946639222314_usize;
_20 = _16.fld1.fld1;
_16.fld2.1.4 = !48780_u16;
_16.fld1.fld0 = [17_u8,111_u8,72_u8,143_u8,116_u8];
_16.fld2.1.2 = _16.fld2.1.1 - Field::<i64>(Variant(_16.fld3, 0), 0);
_18.4 = !_16.fld2.1.4;
_16.fld2.4 = !(-43_i8);
_16.fld3 = Adt42::Variant3 { fld0: _10,fld1: _16.fld1.fld1,fld2: (-133993645667407820077914653757321203882_i128) };
_15 = _16.fld2.1.0;
_16.fld2.1.2 = _13 as i64;
_16.fld2.1.3 = 612668046_u32;
_11 = _15 + _15;
Goto(bb19)
}
bb19 = {
Call(_23 = dump_var(14_usize, 3_usize, Move(_3), 1_usize, Move(_1), 20_usize, Move(_20), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_23 = dump_var(14_usize, 15_usize, Move(_15), 4_usize, Move(_4), 6_usize, Move(_6), 24_usize, _24), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: u16,mut _2: [u32; 4]) -> u8 {
mir! {
type RET = u8;
let _3: i64;
let _4: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _5: u16;
let _6: [char; 6];
let _7: Adt47;
let _8: Adt45;
let _9: isize;
let _10: u8;
let _11: Adt40;
let _12: *mut u32;
let _13: [char; 6];
let _14: char;
let _15: char;
let _16: *mut u32;
let _17: isize;
let _18: [i16; 5];
let _19: char;
let _20: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]));
let _21: [u8; 5];
let _22: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]));
let _23: f32;
let _24: u32;
let _25: Adt46;
let _26: Adt52;
let _27: [isize; 3];
let _28: bool;
let _29: f32;
let _30: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _31: Adt49;
let _32: ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8));
let _33: isize;
let _34: f64;
let _35: isize;
let _36: ();
let _37: ();
{
RET = !44_u8;
_1 = 11924_u16 - 52115_u16;
RET = !50_u8;
RET = 141_u8;
RET = !36_u8;
RET = !20_u8;
RET = 134814366519078505678517994721541111084_i128 as u8;
RET = !132_u8;
_2 = [1205723221_u32,3288949237_u32,409204017_u32,2340238123_u32];
RET = 122_u8 & 149_u8;
_1 = 21710_u16 | 23494_u16;
RET = 154_u8 << _1;
_1 = 1082_u16 ^ 26534_u16;
_1 = 37281_u16;
Goto(bb1)
}
bb1 = {
_1 = !31518_u16;
RET = !28_u8;
_1 = 63208_u16;
RET = !181_u8;
RET = 171_u8 * 235_u8;
_4.0.4 = _1;
_4.1.5.0 = -2651_i16;
_4.1.5.3 = !718939991_u32;
_4.0.5 = RET as f32;
_4.0.5 = _4.1.5.0 as f32;
_4.1.0.4 = _4.1.5.3 as u16;
_8.fld1.fld4 = !10563171850776557312_usize;
_8.fld1.fld5 = core::ptr::addr_of!(_4.0.2);
_4.1.5.4 = _4.1.0.4;
Goto(bb2)
}
bb2 = {
_8.fld2.1.5 = 9223372036854775807_isize as f32;
_4.1.4 = (-10828442010727090849916843997561028234_i128) as f64;
_6 = ['\u{72462}','\u{cbab5}','\u{40475}','\u{6dabb}','\u{af433}','\u{6538a}'];
_8.fld1.fld5 = core::ptr::addr_of!(_4.0.2);
_4.1.0.0 = _4.1.5.0;
_4.1.0.5 = 104_i8 as f32;
_8.fld2.2 = core::ptr::addr_of!(_4.1.5.1);
_4.1.0 = (_4.1.5.0, 2225280545894084500_i64, (-8424687654538524171_i64), _4.1.5.3, _1, _8.fld2.1.5);
_4.0.0 = _4.1.5.0 ^ _4.1.0.0;
_8.fld4 = core::ptr::addr_of_mut!(_8.fld1.fld1);
_2 = [_4.1.0.3,_4.1.0.3,_4.1.5.3,_4.1.0.3];
_8.fld2.1.0 = _4.0.0;
_4.0.3 = _4.1.5.3;
_8.fld2.4 = 100_i8 ^ 100_i8;
_8.fld1.fld0 = [RET,RET,RET,RET,RET];
_11.fld1.1.1 = -_4.1.0.1;
_4.1.1 = _8.fld2.4 as u8;
Goto(bb3)
}
bb3 = {
_4.1.5.2 = _4.1.0.2 >> RET;
_8.fld1.fld3 = _4.1.0.2 as u64;
_14 = '\u{29b51}';
_13 = _6;
_4.1.3 = [_4.1.5.3,_4.0.3,_4.1.5.3,_4.0.3];
_11.fld1.1.2 = _11.fld1.1.1;
_8.fld2.1.4 = _1;
_4.2 = [_4.0.0,_4.1.5.0,_8.fld2.1.0,_8.fld2.1.0,_4.0.0];
_4.1.2 = (-80_isize) + 9223372036854775807_isize;
_8.fld2.3 = -_4.1.4;
_11.fld1.1.5 = _8.fld1.fld3 as f32;
_4.1.5.5 = _11.fld1.1.5;
_4.2 = [_8.fld2.1.0,_8.fld2.1.0,_4.0.0,_4.0.0,_8.fld2.1.0];
_8.fld4 = core::ptr::addr_of_mut!(_14);
_11.fld2 = _4.0.3 + _4.1.5.3;
_4.1.5 = (_4.0.0, _11.fld1.1.2, _4.1.0.2, _4.1.0.3, _4.0.4, _11.fld1.1.5);
_11.fld4 = _8.fld2.2;
_8.fld0 = !_8.fld1.fld4;
_8.fld2.1.3 = !_11.fld2;
_11.fld1.0 = [_8.fld2.1.3,_11.fld2,_8.fld2.1.3,_11.fld2];
_4.0.2 = _4.1.0.2;
_11.fld1.1.2 = _8.fld2.1.4 as i64;
Goto(bb4)
}
bb4 = {
_8.fld2.1.1 = _4.0.2;
_4.1.5.3 = _8.fld2.1.3;
_11.fld0.0 = core::ptr::addr_of!(_4.1.0.2);
_4.1.5.3 = _4.1.0.2 as u32;
_19 = _14;
_4.1.5.4 = _14 as u16;
_4.0.2 = _4.1.2 as i64;
_20.0.1.1.1 = _4.1.0.1 + _11.fld1.1.1;
_3 = _4.1.0.2 & _8.fld2.1.1;
_20.6.1.5.5 = _8.fld2.1.5 - _4.0.5;
_22.6.1.5.4 = _4.1.5.5 as u16;
_22.6.1.5.2 = _4.1.2 as i64;
_20.6.1.4 = _8.fld0 as f64;
_8.fld2.4 = _8.fld1.fld3 as i8;
_22.6.0.2 = _20.0.1.1.1 << _3;
_22.6.1.6 = RET << _22.6.0.2;
_22.6.2 = [_8.fld2.1.0,_4.1.5.0,_4.0.0,_4.1.5.0,_4.1.5.0];
_4.1.5.3 = !_11.fld2;
Goto(bb5)
}
bb5 = {
_22.2.1.1 = _4.1.5;
_22.6.1.0.1 = -_3;
Call(_20.0.1 = fn16(_22.6.1.6, _4.2, _8.fld4, _4.1.5.2, _4.1.5.2, _4.1.3, _14, _11.fld0.0, _4.1.5, _4.1.0, _22.6.2, _11.fld4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8.fld2.1.3 = _22.2.1.1.3;
_20.2.0 = (_20.0.1.2, _22.2.1.1.5);
_22.2 = (_20.2.0, _20.0.1);
_22.2.1.2 = core::ptr::addr_of!(_22.6.1.0.2);
_20.0.1.0 = _22.2.1.0;
_4.1.5.1 = _11.fld1.1.1 * _8.fld2.1.1;
_22.6.1.0.2 = _22.6.1.6 as i64;
_20.6.1.5.4 = _20.0.1.1.4;
_22.0.0 = (_11.fld4, _20.2.0.1);
_22.3 = _20.0.1.0;
_22.6.0.0 = _4.0.0 * _8.fld2.1.0;
_26.fld3.0.0 = _22.6.1.6 as i16;
_22.6.1.0.0 = _26.fld3.0.0 << _26.fld3.0.0;
_20.6.1.5.3 = _4.1.5.5 as u32;
Call(_11.fld5 = core::intrinsics::transmute(_22.6.1.0.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11.fld1.1 = (_26.fld3.0.0, _22.6.0.2, _22.6.1.0.2, _11.fld2, _11.fld5, _22.2.1.1.5);
_20.0 = _22.2;
_22.0.0.1 = -_20.6.1.5.5;
Call(_20.2.1.1.1 = core::intrinsics::bswap(_11.fld1.1.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11.fld3 = _20.0.1.4 - _22.2.1.4;
_26.fld3.5 = (_22.6.0.0, _4.1.5.1, _11.fld1.1.2, _20.6.1.5.3, _11.fld5, _22.0.0.1);
_8.fld2.1.3 = _20.6.1.5.3 & _4.0.3;
_30 = (_26.fld3.5, _22.6.1.6, _4.1.2, _22.2.1.0, _8.fld2.3, _22.2.1.1, _22.6.1.6);
_22.6.0.3 = !_11.fld2;
Goto(bb9)
}
bb9 = {
_20.6.1.0.2 = 148512405468040971998986337241587574405_u128 as i64;
_20.0.1.1.2 = _22.6.1.5.2 | _3;
Goto(bb10)
}
bb10 = {
_26.fld3 = _30;
match _8.fld2.1.1 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463454949919777229687285 => bb17,
_ => bb16
}
}
bb11 = {
_1 = !31518_u16;
RET = !28_u8;
_1 = 63208_u16;
RET = !181_u8;
RET = 171_u8 * 235_u8;
_4.0.4 = _1;
_4.1.5.0 = -2651_i16;
_4.1.5.3 = !718939991_u32;
_4.0.5 = RET as f32;
_4.0.5 = _4.1.5.0 as f32;
_4.1.0.4 = _4.1.5.3 as u16;
_8.fld1.fld4 = !10563171850776557312_usize;
_8.fld1.fld5 = core::ptr::addr_of!(_4.0.2);
_4.1.5.4 = _4.1.0.4;
Goto(bb2)
}
bb12 = {
_11.fld3 = _20.0.1.4 - _22.2.1.4;
_26.fld3.5 = (_22.6.0.0, _4.1.5.1, _11.fld1.1.2, _20.6.1.5.3, _11.fld5, _22.0.0.1);
_8.fld2.1.3 = _20.6.1.5.3 & _4.0.3;
_30 = (_26.fld3.5, _22.6.1.6, _4.1.2, _22.2.1.0, _8.fld2.3, _22.2.1.1, _22.6.1.6);
_22.6.0.3 = !_11.fld2;
Goto(bb9)
}
bb13 = {
_11.fld1.1 = (_26.fld3.0.0, _22.6.0.2, _22.6.1.0.2, _11.fld2, _11.fld5, _22.2.1.1.5);
_20.0 = _22.2;
_22.0.0.1 = -_20.6.1.5.5;
Call(_20.2.1.1.1 = core::intrinsics::bswap(_11.fld1.1.1), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_4.1.5.2 = _4.1.0.2 >> RET;
_8.fld1.fld3 = _4.1.0.2 as u64;
_14 = '\u{29b51}';
_13 = _6;
_4.1.3 = [_4.1.5.3,_4.0.3,_4.1.5.3,_4.0.3];
_11.fld1.1.2 = _11.fld1.1.1;
_8.fld2.1.4 = _1;
_4.2 = [_4.0.0,_4.1.5.0,_8.fld2.1.0,_8.fld2.1.0,_4.0.0];
_4.1.2 = (-80_isize) + 9223372036854775807_isize;
_8.fld2.3 = -_4.1.4;
_11.fld1.1.5 = _8.fld1.fld3 as f32;
_4.1.5.5 = _11.fld1.1.5;
_4.2 = [_8.fld2.1.0,_8.fld2.1.0,_4.0.0,_4.0.0,_8.fld2.1.0];
_8.fld4 = core::ptr::addr_of_mut!(_14);
_11.fld2 = _4.0.3 + _4.1.5.3;
_4.1.5 = (_4.0.0, _11.fld1.1.2, _4.1.0.2, _4.1.0.3, _4.0.4, _11.fld1.1.5);
_11.fld4 = _8.fld2.2;
_8.fld0 = !_8.fld1.fld4;
_8.fld2.1.3 = !_11.fld2;
_11.fld1.0 = [_8.fld2.1.3,_11.fld2,_8.fld2.1.3,_11.fld2];
_4.0.2 = _4.1.0.2;
_11.fld1.1.2 = _8.fld2.1.4 as i64;
Goto(bb4)
}
bb15 = {
_22.2.1.1 = _4.1.5;
_22.6.1.0.1 = -_3;
Call(_20.0.1 = fn16(_22.6.1.6, _4.2, _8.fld4, _4.1.5.2, _4.1.5.2, _4.1.3, _14, _11.fld0.0, _4.1.5, _4.1.0, _22.6.2, _11.fld4), ReturnTo(bb6), UnwindUnreachable())
}
bb16 = {
_8.fld2.1.1 = _4.0.2;
_4.1.5.3 = _8.fld2.1.3;
_11.fld0.0 = core::ptr::addr_of!(_4.1.0.2);
_4.1.5.3 = _4.1.0.2 as u32;
_19 = _14;
_4.1.5.4 = _14 as u16;
_4.0.2 = _4.1.2 as i64;
_20.0.1.1.1 = _4.1.0.1 + _11.fld1.1.1;
_3 = _4.1.0.2 & _8.fld2.1.1;
_20.6.1.5.5 = _8.fld2.1.5 - _4.0.5;
_22.6.1.5.4 = _4.1.5.5 as u16;
_22.6.1.5.2 = _4.1.2 as i64;
_20.6.1.4 = _8.fld0 as f64;
_8.fld2.4 = _8.fld1.fld3 as i8;
_22.6.0.2 = _20.0.1.1.1 << _3;
_22.6.1.6 = RET << _22.6.0.2;
_22.6.2 = [_8.fld2.1.0,_4.1.5.0,_4.0.0,_4.1.5.0,_4.1.5.0];
_4.1.5.3 = !_11.fld2;
Goto(bb5)
}
bb17 = {
_22.6.1.0.5 = _8.fld1.fld3 as f32;
_4.1.2 = true as isize;
_9 = _26.fld3.2;
_22.6.1.5.1 = -_22.2.1.1.1;
_8.fld2.1.5 = _26.fld3.5.5;
_4.0.1 = -_30.0.2;
_20.2.0.1 = -_11.fld1.1.5;
_11.fld1.3 = -_4.1.4;
_22.6.1.0.3 = _8.fld1.fld3 as u32;
_8.fld1.fld1 = _19;
_30.0.0 = !_22.6.1.0.0;
_20.2.1.1.2 = _11.fld1.1.2;
_4.2 = [_22.6.0.0,_22.6.1.0.0,_22.6.1.0.0,_22.6.1.0.0,_22.6.0.0];
_20.6.0.4 = _26.fld3.0.4;
_20.0.1.1.2 = _30.0.0 as i64;
_8.fld0 = _26.fld3.0.3 as usize;
_30.4 = _20.6.1.4 - _8.fld2.3;
_32.1.1.0 = _22.6.1.0.0;
_20.0.1 = (_26.fld3.3, _30.5, _8.fld2.2, _30.4, _11.fld3);
_26.fld1 = !_11.fld1.1.4;
_8.fld3 = Adt42::Variant0 { fld0: _30.0.2,fld1: _8.fld0,fld2: _30.2 };
_32.1.3 = _30.0.0 as f64;
_29 = _8.fld2.1.5 - _11.fld1.1.5;
_8.fld2.1.5 = _8.fld1.fld3 as f32;
_22.2.1.1.2 = !_30.5.2;
_20.1 = _30.4 as f32;
Goto(bb18)
}
bb18 = {
Call(_36 = dump_var(15_usize, 13_usize, Move(_13), 1_usize, Move(_1), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: u8,mut _2: [i16; 5],mut _3: *mut char,mut _4: i64,mut _5: i64,mut _6: [u32; 4],mut _7: char,mut _8: *const i64,mut _9: (i16, i64, i64, u32, u16, f32),mut _10: (i16, i64, i64, u32, u16, f32),mut _11: [i16; 5],mut _12: *const i64) -> ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8) {
mir! {
type RET = ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8);
let _13: bool;
let _14: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _15: [u32; 4];
let _16: [char; 5];
let _17: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _18: isize;
let _19: u128;
let _20: ();
let _21: ();
{
RET.1.0 = _1 as i16;
_10.4 = !_9.4;
RET.1.3 = _9.3 - _10.3;
_7 = (*_3);
RET.1 = (_9.0, _10.1, (*_12), _9.3, _10.4, _9.5);
(*_3) = _7;
RET.1 = (_10.0, (*_8), _10.2, _10.3, _9.4, _10.5);
RET.1.5 = _10.0 as f32;
RET.0 = [_9.3,_10.3,RET.1.3,RET.1.3];
_9 = (RET.1.0, (*_8), _5, RET.1.3, RET.1.4, _10.5);
_9.5 = RET.1.5 - RET.1.5;
_11 = [_9.0,_10.0,_10.0,_9.0,_10.0];
_7 = (*_3);
_10.3 = RET.1.3 - _9.3;
_10.3 = _9.3;
_10 = RET.1;
_7 = (*_3);
_8 = core::ptr::addr_of!((*_12));
RET.3 = 6_usize as f64;
_9.0 = 7246548808519241533_usize as i16;
_10.1 = !_10.2;
_9.5 = _1 as f32;
_9.4 = _10.4;
_11 = _2;
RET.1.5 = _9.5;
RET.1.3 = _10.3;
_14 = (_9, _1, 9223372036854775807_isize, RET.0, RET.3, RET.1, _1);
Goto(bb1)
}
bb1 = {
_10.3 = _9.3 >> _1;
Goto(bb2)
}
bb2 = {
RET.3 = _14.4 - _14.4;
_5 = !_14.0.1;
RET.1.2 = !RET.1.1;
RET.1.1 = _14.5.2 * _9.1;
_17 = (RET.1, _14, _11);
_1 = false as u8;
_14.5.0 = 971552146_i32 as i16;
RET.4 = (-93_i8) << _14.0.4;
(*_8) = -_17.1.0.1;
_17.1.2 = _14.2;
_14.4 = _17.1.4 * RET.3;
RET.1.5 = -_17.0.5;
_17.1.1 = _14.6 >> (*_8);
_15 = [_10.3,_10.3,RET.1.3,_10.3];
_14 = _17.1;
RET.1.1 = _17.0.1 << _10.3;
RET = (_15, _14.5, _12, _14.4, 79_i8);
_9.5 = -RET.1.5;
Goto(bb3)
}
bb3 = {
Call(_20 = dump_var(16_usize, 2_usize, Move(_2), 7_usize, Move(_7), 11_usize, Move(_11), 15_usize, Move(_15)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: char,mut _2: i128,mut _3: Adt44,mut _4: i8) -> [char; 5] {
mir! {
type RET = [char; 5];
let _5: f64;
let _6: ();
let _7: ();
{
_2 = (-111219127161870962488277481749067568100_i128);
_3.fld2 = (-9223372036854775808_isize);
_3.fld1 = _1;
match _3.fld3 {
0 => bb1,
9964803728539557732 => bb3,
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
_3.fld1 = _1;
RET = [_1,_3.fld1,_3.fld1,_3.fld1,_1];
_3.fld0 = [59_u8,25_u8,62_u8,42_u8,36_u8];
_5 = (-189505188_i32) as f64;
_2 = _4 as i128;
_5 = 2115674219_i32 as f64;
_4 = (-81_i8);
RET = [_1,_1,_3.fld1,_1,_1];
Goto(bb4)
}
bb4 = {
Call(_6 = dump_var(17_usize, 4_usize, Move(_4), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: f64,mut _2: [char; 5],mut _3: [char; 5]) -> u64 {
mir! {
type RET = u64;
let _4: [isize; 3];
let _5: f64;
let _6: [char; 6];
let _7: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _8: [char; 5];
let _9: char;
let _10: isize;
let _11: bool;
let _12: u8;
let _13: char;
let _14: i32;
let _15: i32;
let _16: [char; 5];
let _17: [u32; 4];
let _18: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]);
let _19: isize;
let _20: i16;
let _21: f64;
let _22: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _23: *const usize;
let _24: u16;
let _25: [u32; 4];
let _26: *mut u16;
let _27: f64;
let _28: [u8; 5];
let _29: f64;
let _30: f64;
let _31: Adt41;
let _32: isize;
let _33: bool;
let _34: Adt38;
let _35: char;
let _36: u8;
let _37: [char; 5];
let _38: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _39: bool;
let _40: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _41: Adt47;
let _42: Adt45;
let _43: Adt48;
let _44: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8);
let _45: f64;
let _46: [u32; 4];
let _47: [u8; 5];
let _48: [u8; 5];
let _49: ();
let _50: ();
{
_1 = 221237120_u32 as f64;
RET = '\u{505db}' as u64;
RET = !7778276156319771106_u64;
_2 = _3;
RET = 2160814117_u32 as u64;
_1 = 108_i8 as f64;
_3 = ['\u{d7614}','\u{2dc1b}','\u{aa7d5}','\u{237b2}','\u{9608b}'];
_4 = [28_isize,9223372036854775807_isize,9223372036854775807_isize];
_5 = RET as f64;
RET = 2883239033674696959_u64 + 3219185681120827839_u64;
_3 = ['\u{f1769}','\u{624}','\u{3d111}','\u{eb28e}','\u{30768}'];
_6 = ['\u{ec434}','\u{ad22}','\u{3db8}','\u{d581b}','\u{a1af1}','\u{831e2}'];
_6 = ['\u{9aa8f}','\u{cb2db}','\u{5b6bb}','\u{8ec8b}','\u{f951c}','\u{ffa29}'];
_1 = _5 * _5;
_2 = ['\u{7ca4d}','\u{d3857}','\u{958c1}','\u{d1164}','\u{b9fcd}'];
_5 = -_1;
_5 = _1;
_6 = ['\u{8f7a9}','\u{189a0}','\u{9cf5b}','\u{f0c82}','\u{e2ae2}','\u{59be3}'];
_1 = _5 - _5;
_7.0.3 = 1653489133575358493_i64 as u32;
Goto(bb1)
}
bb1 = {
_2 = ['\u{232cb}','\u{a1f1f}','\u{d3cad}','\u{107209}','\u{f47d5}'];
_7.5.2 = -3818895999284075526_i64;
_7.5.4 = (-10085426951069454017858579807185194410_i128) as u16;
_7.5.5 = RET as f32;
Goto(bb2)
}
bb2 = {
_7.0.4 = _7.5.4 | _7.5.4;
_7.0.3 = RET as u32;
_4 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_7.4 = -_1;
_7.0.5 = _7.5.5 - _7.5.5;
_7.5.3 = !_7.0.3;
_4 = [0_isize,9223372036854775807_isize,9223372036854775807_isize];
_7.5.3 = _7.0.3;
_7.5.1 = 757464660_i32 as i64;
_7.5.0 = (-32473_i16);
_7.1 = 190_u8 * 69_u8;
_7.3 = [_7.0.3,_7.5.3,_7.0.3,_7.5.3];
_4 = [(-30_isize),(-92_isize),9223372036854775807_isize];
_7.2 = 119_isize;
_7.6 = _7.5.5 as u8;
_3 = ['\u{ee425}','\u{e1830}','\u{5cbf7}','\u{103ae7}','\u{92fbc}'];
_2 = ['\u{20c62}','\u{89a18}','\u{101840}','\u{52b4d}','\u{be4d6}'];
_7.0.4 = !_7.5.4;
_7.5.3 = _7.0.3 & _7.0.3;
_7.2 = 10956537085099339588_usize as isize;
_9 = '\u{fbb85}';
_7.0 = (_7.5.0, _7.5.2, _7.5.1, _7.5.3, _7.5.4, _7.5.5);
_7.4 = _7.2 as f64;
_8 = [_9,_9,_9,_9,_9];
_7.3 = [_7.0.3,_7.0.3,_7.0.3,_7.5.3];
_13 = _9;
Call(_10 = core::intrinsics::bswap(_7.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = 13069178884499472889_u64 << _7.0.3;
Goto(bb4)
}
bb4 = {
_7.0.1 = _7.5.2;
_7.5.1 = _7.0.1;
_7.0.2 = _7.5.2;
_7.5.0 = _7.0.0 & _7.0.0;
_7.0.2 = _7.5.2;
_11 = true;
_7.5.1 = _7.0.1 | _7.0.1;
_9 = _13;
_11 = !true;
_16 = [_13,_9,_9,_13,_9];
_7.4 = -_1;
_7.0.0 = _7.5.0 * _7.5.0;
_18.1.0.0 = !_7.5.0;
_18.1.3 = [_7.0.3,_7.5.3,_7.5.3,_7.0.3];
RET = _7.2 as u64;
_18.0.4 = _7.0.4;
Goto(bb5)
}
bb5 = {
_7.5.2 = _7.5.1 << _7.5.3;
_18.1.1 = (-150736871995653067002764196921674057818_i128) as u8;
_18.0.3 = _7.0.3;
_18.0.0 = _7.5.0;
_18.1.5.4 = !_7.0.4;
_18.1.6 = _7.0.0 as u8;
_9 = _13;
_7.5.1 = !_7.0.1;
_18.1.5.3 = !_18.0.3;
_18.1 = (_7.0, _7.1, _7.2, _7.3, _1, _7.0, _7.6);
_18.1.0.2 = _18.1.5.1;
_7.4 = _18.1.4 - _1;
_18.1.5.4 = _7.0.4 + _18.0.4;
_18.1.5.2 = _18.1.5.1 * _7.5.2;
_7.6 = _18.1.6 & _18.1.6;
_17 = [_18.1.0.3,_18.1.5.3,_18.0.3,_18.1.5.3];
_18.0.5 = _7.0.1 as f32;
_6 = [_13,_13,_9,_13,_13,_13];
_8 = _3;
_7.0.0 = -_18.1.5.0;
_3 = [_13,_9,_13,_9,_9];
_7.0 = (_18.1.0.0, _18.1.5.1, _18.1.5.2, _7.5.3, _18.1.0.4, _7.5.5);
_18.1.5.0 = !_18.0.0;
_10 = _7.2;
Goto(bb6)
}
bb6 = {
_18.1.0.0 = _7.4 as i16;
_18.0.2 = _18.1.5.1;
_22 = _7;
_7.0 = (_18.1.0.0, _22.5.2, _7.5.2, _7.5.3, _18.1.5.4, _22.5.5);
_18.1.1 = _18.1.6;
_19 = !_10;
_18.1.3 = [_18.1.0.3,_7.0.3,_18.0.3,_18.1.5.3];
_7.5.3 = _7.0.3;
_20 = _7.5.0 & _22.5.0;
_18.1.0.1 = _22.0.2 ^ _7.0.1;
_18.0.5 = _18.1.0.5;
_18.2 = [_7.0.0,_18.1.0.0,_18.1.0.0,_7.0.0,_18.1.0.0];
_27 = _7.4 - _18.1.4;
_7.0.3 = _18.1.5.3 + _18.1.0.3;
Call(_18.0.3 = core::intrinsics::transmute(_7.0.3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_18.1.5.0 = !_18.1.0.0;
_15 = !263268869_i32;
_3 = _2;
_7.3 = [_7.0.3,_7.5.3,_7.0.3,_22.0.3];
_22.5.0 = _19 as i16;
_7.5.2 = _22.0.2;
_24 = _7.0.4;
_7.5 = (_18.1.5.0, _22.5.2, _7.0.1, _7.0.3, _7.0.4, _18.1.5.5);
_22.0.5 = -_18.1.5.5;
_22.4 = _1;
_18.1.4 = _1 + _7.4;
_22 = _18.1;
_18.1.5.5 = _7.0.5;
_18.1.5.0 = _22.5.0 | _7.0.0;
_7.0.4 = _7.5.4;
_6 = [_9,_13,_9,_13,_9,_9];
_3 = [_9,_9,_13,_13,_13];
_18.0.0 = _7.5.0 * _22.5.0;
_7.2 = 6_usize as isize;
_22.0.0 = _18.0.0 + _18.1.5.0;
_10 = RET as isize;
RET = _15 as u64;
_22.6 = _18.1.1;
Goto(bb8)
}
bb8 = {
_7.5.4 = _22.0.5 as u16;
_18.1.0.1 = _7.0.1 ^ _7.5.2;
_18.1.6 = _18.1.1 | _7.6;
_22.4 = _1;
_22.0.0 = _7.6 as i16;
_18.0 = (_22.5.0, _7.0.1, _22.0.1, _18.1.5.3, _22.5.4, _22.0.5);
_18.1.0.1 = _7.6 as i64;
_21 = -_7.4;
Goto(bb9)
}
bb9 = {
_10 = _18.1.2;
_7.5.3 = !_22.0.3;
_3 = [_9,_13,_13,_9,_9];
_17 = [_18.1.0.3,_22.5.3,_7.0.3,_22.5.3];
_8 = [_9,_13,_9,_13,_13];
_31 = Adt41::Variant1 { fld0: _22.0 };
_18.1.5.3 = _22.0.0 as u32;
Goto(bb10)
}
bb10 = {
_22.5.5 = _7.0.5 * _7.0.5;
_26 = core::ptr::addr_of_mut!(_24);
_29 = _27 + _18.1.4;
_7.4 = -_1;
Goto(bb11)
}
bb11 = {
_22.5.4 = _18.1.6 as u16;
_19 = _7.2 & _7.2;
_22 = (_18.1.5, _18.1.6, _19, _17, _18.1.4, Field::<(i16, i64, i64, u32, u16, f32)>(Variant(_31, 1), 0), _7.6);
_2 = [_9,_9,_13,_13,_13];
_7.0.2 = !_18.0.2;
Goto(bb12)
}
bb12 = {
_12 = _22.5.3 as u8;
place!(Field::<(i16, i64, i64, u32, u16, f32)>(Variant(_31, 1), 0)).1 = _7.5.1;
_20 = _9 as i16;
_25 = [_18.1.5.3,Field::<(i16, i64, i64, u32, u16, f32)>(Variant(_31, 1), 0).3,_7.5.3,_22.5.3];
_15 = 5056870763108083821_usize as i32;
_22.5.3 = _7.0.3;
_8 = [_13,_9,_13,_9,_13];
Goto(bb13)
}
bb13 = {
_33 = _22.5.3 > _7.0.3;
_18.1.0 = (_22.0.0, _18.1.5.2, _7.5.1, _7.0.3, _7.0.4, _7.0.5);
_38.5.5 = _22.0.5 * _18.0.5;
_38 = (_22.0, _18.1.6, _19, _18.1.3, _1, _18.1.0, _22.6);
_38.5.5 = _22.4 as f32;
_18.1.0.1 = _19 as i64;
_38.5 = (_7.5.0, _18.1.0.2, _18.0.2, _22.5.3, _18.0.4, _7.0.5);
_8 = [_9,_9,_13,_13,_9];
_40.2 = _22.1 as isize;
_40.6 = _38.5.1 as u8;
_22.0.2 = _18.0.2;
_8 = _2;
_40 = (_18.1.5, _22.1, _22.2, _17, _22.4, _38.0, _7.6);
_35 = _9;
SetDiscriminant(_31, 0);
_7.0.0 = 2768790893452391142771723848918588743_u128 as i16;
_18.1.1 = _15 as u8;
_5 = _29;
_18.1.0.4 = (*_26);
_7.0.3 = _22.0.3;
_7.5.0 = _18.1.5.0 | _22.5.0;
RET = !3660799413111871854_u64;
_22.5.4 = !_18.0.4;
_22.0.0 = _40.5.0 >> _18.0.0;
_40.2 = -_18.1.2;
_22.5.4 = (*_26);
_40.2 = _10;
Goto(bb14)
}
bb14 = {
_7.5 = (_40.5.0, _40.0.2, _7.0.2, _18.1.0.3, _7.0.4, _38.0.5);
_23 = core::ptr::addr_of!(_42.fld0);
(*_23) = !17330160215509624282_usize;
_42.fld2.1.0 = -_7.5.0;
_18.1.5.0 = _38.0.0 + _42.fld2.1.0;
_44.3 = _25;
_44.3 = [_38.0.3,_18.0.3,_40.0.3,_7.5.3];
_40.5.5 = -_22.5.5;
_44.2 = (-127_i8) as isize;
_7.5.1 = (-127120706640235389395636939066660007094_i128) as i64;
_7.0.3 = !_22.5.3;
_42.fld2.3 = _40.5.0 as f64;
_7.0 = _18.1.0;
_7.5.5 = -_38.5.5;
_38.0 = _38.5;
_18.1.0.4 = _22.5.4;
_42.fld1.fld4 = _40.2 as usize;
_7.0 = (_18.1.5.0, _38.0.1, _40.5.2, _38.5.3, _24, _18.1.0.5);
_30 = -_38.4;
_32 = _38.2 | _22.2;
_42.fld2.1.4 = _40.5.4;
_42.fld0 = _42.fld1.fld4;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(18_usize, 6_usize, Move(_6), 25_usize, Move(_25), 11_usize, Move(_11), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(18_usize, 13_usize, Move(_13), 15_usize, Move(_15), 33_usize, Move(_33), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(18_usize, 17_usize, Move(_17), 3_usize, Move(_3), 50_usize, _50, 50_usize, _50), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{1cb9e}'), std::hint::black_box((-10_isize)), std::hint::black_box(11_i8), std::hint::black_box(1543_i16), std::hint::black_box(831526513_i32), std::hint::black_box((-4169480224218735522_i64)), std::hint::black_box(2352666331786253457740644676188876266_i128), std::hint::black_box(6_usize), std::hint::black_box(90_u8), std::hint::black_box(10378_u16), std::hint::black_box(343886003_u32), std::hint::black_box(10118046198371631461_u64), std::hint::black_box(25116997316737087629575440172656775339_u128));
                
            }
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt38::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: i16,
fld1: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld2: [char; 5],
fld3: *mut char,

},
Variant1{
fld0: f64,
fld1: *const (*const i64, f32),
fld2: *const u32,
fld3: [u32; 4],
fld4: *const usize,
fld5: usize,
fld6: u128,

},
Variant2{
fld0: f64,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]),

},
Variant1{
fld0: bool,
fld1: *mut u32,
fld2: Adt38,
fld3: i8,
fld4: f64,
fld5: u128,
fld6: (i16, i64, i64, u32, u16, f32),

},
Variant2{
fld0: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8),
fld1: char,
fld2: (i16, i64, i64, u32, u16, f32),
fld3: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5])),
fld4: *const usize,
fld5: f32,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: (*const i64, f32),
fld1: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld2: u32,
fld3: i8,
fld4: *const i64,
fld5: u16,
}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *const u32,

},
Variant1{
fld0: (i16, i64, i64, u32, u16, f32),

},
Variant2{
fld0: *const (*const i64, f32),
fld1: u32,
fld2: u128,

},
Variant3{
fld0: *const usize,
fld1: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld2: isize,
fld3: i8,
fld4: Adt39,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: i64,
fld1: usize,
fld2: isize,

},
Variant1{
fld0: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld1: i32,
fld2: *const usize,

},
Variant2{
fld0: u8,
fld1: i64,
fld2: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld3: u16,
fld4: f32,

},
Variant3{
fld0: [char; 6],
fld1: char,
fld2: i128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5])),
fld1: *const i64,
fld2: isize,
fld3: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]),
fld4: [isize; 3],
fld5: Adt41,
fld6: u32,
fld7: Adt40,

},
Variant1{
fld0: *mut u32,
fld1: i64,
fld2: u8,
fld3: i8,

},
Variant2{
fld0: *mut u32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: [u8; 5],
fld1: char,
fld2: isize,
fld3: u64,
fld4: usize,
fld5: *const i64,
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: usize,
fld1: Adt44,
fld2: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld3: Adt42,
fld4: *mut char,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: Adt39,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)),

},
Variant1{
fld0: (i16, i64, i64, u32, u16, f32),
fld1: f32,
fld2: Adt41,
fld3: i8,
fld4: [u8; 5],
fld5: u16,
fld6: Adt46,
fld7: u128,

},
Variant2{
fld0: Adt40,
fld1: Adt45,
fld2: [isize; 3],

},
Variant3{
fld0: [char; 6],
fld1: u64,
fld2: *mut u16,
fld3: i8,
fld4: *const i64,
fld5: i32,
fld6: *const usize,
fld7: Adt38,

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [u8; 5],
fld1: Adt47,
fld2: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld3: f64,
fld4: [char; 6],
fld5: *const u32,

},
Variant1{
fld0: (i16, i64, i64, u32, u16, f32),
fld1: *const usize,
fld2: ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)),
fld3: Adt43,
fld4: Adt47,
fld5: i32,
fld6: *const (*const i64, f32),

},
Variant2{
fld0: u64,

},
Variant3{
fld0: Adt47,
fld1: Adt42,
fld2: (*const i64, f32),
fld3: u16,

}}
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u64,
fld1: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5])),
fld2: i32,

},
Variant1{
fld0: Adt47,
fld1: [char; 6],
fld2: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5])),
fld3: (*const i64, f32),
fld4: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]),
fld5: u64,
fld6: f32,
fld7: Adt43,

}}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5]),
fld1: char,
fld2: [u32; 4],
fld3: Adt49,
fld4: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld5: Adt44,
fld6: [i16; 5],
fld7: Adt42,

},
Variant1{
fld0: Adt40,
fld1: u128,
fld2: isize,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *mut u32,
fld1: Adt48,
fld2: (i16, i64, i64, u32, u16, f32),

},
Variant1{
fld0: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8),
fld1: u16,
fld2: f32,
fld3: [char; 5],
fld4: i16,
fld5: Adt47,
fld6: u128,
fld7: Adt40,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: i64,
fld1: u16,
fld2: *const i64,
fld3: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8),
}
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *const u32,
fld1: *const usize,
fld2: (((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), f32, ((*const i64, f32), ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8)), [u32; 4], i32, [char; 5], ((i16, i64, i64, u32, u16, f32), ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8), [i16; 5])),

},
Variant1{
fld0: *const (*const i64, f32),
fld1: *const usize,

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u8; 5],
fld1: Adt40,
fld2: ((i16, i64, i64, u32, u16, f32), u8, isize, [u32; 4], f64, (i16, i64, i64, u32, u16, f32), u8),
fld3: i8,
fld4: *mut u32,
fld5: Adt49,

},
Variant1{
fld0: Adt53,
fld1: [isize; 3],
fld2: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),

},
Variant2{
fld0: ([u32; 4], (i16, i64, i64, u32, u16, f32), *const i64, f64, i8),
fld1: i64,

},
Variant3{
fld0: Adt43,

}}

