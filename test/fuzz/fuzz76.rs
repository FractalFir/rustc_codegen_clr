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
pub fn fn0(mut _1: u16,mut _2: u32,mut _3: isize,mut _4: usize) -> [bool; 8] {
mir! {
type RET = [bool; 8];
let _5: f64;
let _6: (i32, i8);
let _7: Adt42;
let _8: f32;
let _9: char;
let _10: *const *const u128;
let _11: u16;
let _12: u32;
let _13: u64;
let _14: isize;
let _15: char;
let _16: (f32,);
let _17: i16;
let _18: (bool, [bool; 8], i8, u16, i16, i16);
let _19: f32;
let _20: f32;
let _21: i16;
let _22: char;
let _23: (f32,);
let _24: [u16; 7];
let _25: ();
let _26: ();
{
RET = [false,true,true,false,false,true,true,false];
_3 = (-29_isize) + (-94_isize);
_3 = '\u{108eb7}' as isize;
_3 = (-1865500737_i32) as isize;
RET = [true,false,false,false,true,true,true,false];
_2 = 1073267872_u32;
_6.1 = !111_i8;
_6.1 = -103_i8;
_6.0 = !(-1250984225_i32);
_6.1 = (-19_i8) ^ 33_i8;
_6.1 = 120_i8 | 86_i8;
_4 = 2_usize;
match _2 {
1073267872 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_5 = 580995242644133067_i64 as f64;
_6 = ((-1330042366_i32), (-18_i8));
RET[_4] = _6.0 <= _6.0;
_4 = _6.0 as usize;
_1 = 3250429631443947411_u64 as u16;
Goto(bb3)
}
bb3 = {
_6.1 = 177_u8 as i8;
_8 = 8727826075054934885_u64 as f32;
_5 = _4 as f64;
_8 = (-4222_i16) as f32;
match _6.0 {
340282366920938463463374607430438169090 => bb4,
_ => bb2
}
}
bb4 = {
_6.0 = 1755604021_i32;
RET = [true,false,false,true,true,true,false,true];
_6.0 = !328405771_i32;
_9 = '\u{bec37}';
_1 = 20103_i16 as u16;
_6 = (655143663_i32, 50_i8);
_6.0 = 6964_i16 as i32;
_5 = (-8686532122897924574_i64) as f64;
_8 = _6.1 as f32;
Goto(bb5)
}
bb5 = {
_6.1 = (-70_i8) << _6.0;
_11 = _1 - _1;
_2 = 3289555917_u32 >> _11;
_11 = _1 | _1;
RET = [true,true,false,false,false,true,false,false];
_6.1 = -36_i8;
RET = [true,true,true,true,false,false,false,false];
_4 = _6.1 as usize;
_6.0 = -1602434233_i32;
_9 = '\u{cd21a}';
_6.0 = 9506030651922732754_u64 as i32;
Goto(bb6)
}
bb6 = {
_3 = 108_isize;
_5 = (-26589_i16) as f64;
_9 = '\u{b21ff}';
_4 = !4_usize;
RET = [false,false,true,true,false,true,true,true];
RET = [true,true,false,false,true,false,true,false];
_11 = true as u16;
_12 = _2;
_1 = _11 * _11;
_2 = _12;
RET = [false,true,false,false,false,false,false,false];
_13 = 230_u8 as u64;
_4 = (-114333528158533665164727850300887732896_i128) as usize;
_6.0 = (-114632304_i32);
Call(RET = fn1(_13, _11, _4, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = _11 | _11;
_9 = '\u{d0d36}';
_3 = (-61_isize);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211395 => bb8,
_ => bb6
}
}
bb8 = {
RET = [false,true,false,true,false,true,false,true];
_4 = _8 as usize;
_6.1 = (-10_i8);
_2 = 146_u8 as u32;
_9 = '\u{4f48d}';
RET = [false,true,true,true,true,true,true,false];
_11 = 5521797118623649730_i64 as u16;
_12 = !_2;
_6 = (1088014580_i32, (-29_i8));
_6.0 = !2023454696_i32;
_3 = _6.0 as isize;
_15 = _9;
_13 = (-4300814878290004131_i64) as u64;
match _6.1 {
340282366920938463463374607431768211427 => bb10,
_ => bb9
}
}
bb9 = {
_6.1 = 177_u8 as i8;
_8 = 8727826075054934885_u64 as f32;
_5 = _4 as f64;
_8 = (-4222_i16) as f32;
match _6.0 {
340282366920938463463374607430438169090 => bb4,
_ => bb2
}
}
bb10 = {
RET = [false,false,false,false,false,false,true,true];
RET = [false,true,true,true,true,false,true,true];
_13 = 589607916843239897_u64 + 4966243404400910661_u64;
_15 = _9;
_15 = _9;
_5 = _11 as f64;
_15 = _9;
_15 = _9;
match _6.1 {
0 => bb5,
1 => bb8,
340282366920938463463374607431768211427 => bb11,
_ => bb4
}
}
bb11 = {
_6.1 = (-40_i8) << _12;
_16.0 = -_8;
_6.0 = (-5612146536875028415_i64) as i32;
_6.0 = (-1971652257_i32) * (-126904693_i32);
_15 = _9;
Call(_4 = core::intrinsics::transmute(_13), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_3 = !9223372036854775807_isize;
RET = [false,true,true,true,false,false,true,false];
_6.1 = 104_i8;
_9 = _15;
_11 = _6.1 as u16;
_6.0 = (-419333839_i32);
_6 = (139383273_i32, 112_i8);
_2 = !_12;
Goto(bb13)
}
bb13 = {
_16 = (_8,);
_4 = _6.0 as usize;
_8 = -_16.0;
_14 = _12 as isize;
_13 = false as u64;
_18.2 = !_6.1;
_5 = 1707_i16 as f64;
_18 = (false, RET, _6.1, _1, (-16900_i16), (-495_i16));
_5 = 75149503010501810973063055256788338134_i128 as f64;
_5 = 158894496742853800358232142175109505618_u128 as f64;
_17 = -_18.5;
_5 = _18.5 as f64;
match _18.4 {
0 => bb1,
1 => bb6,
2 => bb11,
3 => bb9,
4 => bb14,
5 => bb15,
340282366920938463463374607431768194556 => bb17,
_ => bb16
}
}
bb14 = {
_3 = !9223372036854775807_isize;
RET = [false,true,true,true,false,false,true,false];
_6.1 = 104_i8;
_9 = _15;
_11 = _6.1 as u16;
_6.0 = (-419333839_i32);
_6 = (139383273_i32, 112_i8);
_2 = !_12;
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
_1 = _11 | _11;
_9 = '\u{d0d36}';
_3 = (-61_isize);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211395 => bb8,
_ => bb6
}
}
bb17 = {
_1 = _18.3 * _18.3;
RET = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_8 = _6.1 as f32;
_12 = _2 | _2;
_14 = _3 + _3;
_13 = !3730794311459729479_u64;
_1 = !_18.3;
_19 = _8;
_9 = _15;
_18 = (false, RET, _6.1, _1, _17, _17);
_18.2 = _18.3 as i8;
_15 = _9;
_4 = 10442264905206552059_usize * 2700778657413055159_usize;
Goto(bb18)
}
bb18 = {
Call(_25 = dump_var(0_usize, 2_usize, Move(_2), 6_usize, Move(_6), 15_usize, Move(_15), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_25 = dump_var(0_usize, 11_usize, Move(_11), 9_usize, Move(_9), 26_usize, _26, 26_usize, _26), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u64,mut _2: u16,mut _3: usize,mut _4: u16) -> [bool; 8] {
mir! {
type RET = [bool; 8];
let _5: i16;
let _6: isize;
let _7: bool;
let _8: bool;
let _9: (isize,);
let _10: char;
let _11: u128;
let _12: (isize,);
let _13: Adt39;
let _14: isize;
let _15: [u16; 7];
let _16: i32;
let _17: [u16; 7];
let _18: bool;
let _19: bool;
let _20: isize;
let _21: i8;
let _22: [bool; 8];
let _23: bool;
let _24: Adt40;
let _25: bool;
let _26: ();
let _27: ();
{
RET = [true,true,true,true,true,true,true,true];
_1 = 4233117727_u32 as u64;
RET = [false,false,true,true,false,true,false,true];
_3 = 3_usize;
RET = [true,true,true,true,true,true,true,true];
_4 = _2;
RET = [false,false,true,true,true,false,true,true];
_4 = _2 ^ _2;
_4 = !_2;
_2 = !_4;
RET = [false,true,false,true,false,false,false,true];
RET[_3] = !true;
_2 = _4;
_2 = 3429832217_u32 as u16;
_1 = 16830186437328615423_u64;
RET[_3] = !false;
_5 = 1609335426_i32 as i16;
RET = [false,false,false,false,true,true,false,true];
RET[_3] = true;
RET[_3] = true ^ true;
_6 = (-9223372036854775808_isize) >> _3;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
4 => bb5,
3 => bb7,
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
_7 = _3 <= _3;
RET[_3] = _7;
_9.0 = _6 >> _3;
RET[_3] = !_7;
_2 = !_4;
_5 = 138223411963846918668725376596848425578_u128 as i16;
_8 = _7 < RET[_3];
_6 = (-147180561642593827600007419538599574633_i128) as isize;
_8 = RET[_3];
_5 = !(-19656_i16);
_1 = (-341518222_i32) as u64;
_4 = '\u{ce8ef}' as u16;
_1 = 12455295971207857122_u64 * 8826374018483846643_u64;
_7 = !RET[_3];
_3 = 9786728773123979056_usize;
_3 = 5_usize + 2_usize;
_2 = _4 ^ _4;
_7 = _2 < _2;
_11 = !192425742372330231190374835071111461262_u128;
_4 = _2;
Goto(bb8)
}
bb8 = {
_3 = 2754033671015155507_usize;
_14 = -_6;
_9 = (_6,);
_12.0 = _4 as isize;
_7 = _5 != _5;
_5 = -(-10943_i16);
_16 = (-1781551976_i32);
_15 = [_4,_4,_2,_2,_4,_4,_2];
_9.0 = -_6;
_10 = '\u{10acb2}';
_12 = _9;
_10 = '\u{10ca91}';
_9.0 = _12.0;
_16 = 352237357_i32;
_3 = 32380564259748396_usize ^ 1_usize;
_6 = -_14;
_11 = 118386962074759884175784980392227583453_i128 as u128;
_6 = 8502400679505393631_i64 as isize;
_10 = '\u{ec63e}';
_15 = [_4,_4,_4,_4,_2,_2,_4];
RET = [_8,_7,_7,_7,_7,_8,_8,_8];
_2 = _4;
_7 = _11 < _11;
Call(_4 = fn2(_3, _12.0, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = 6926446253452991775_u64;
_7 = _11 != _11;
_6 = _12.0 << _1;
_8 = !_7;
_12.0 = _9.0;
_5 = _7 as i16;
_9 = _12;
RET = [_7,_8,_7,_7,_8,_7,_8,_8];
_18 = _8;
_10 = '\u{5c6f0}';
_6 = _9.0;
_4 = !_2;
_9 = (_14,);
_5 = (-26444_i16);
_10 = '\u{18f16}';
_5 = 28933_i16;
_16 = (-823122223_i32);
_18 = !_8;
_16 = -929582495_i32;
_1 = 14902063034649266802_u64 + 6574549326948757577_u64;
_6 = _9.0;
_16 = !730482029_i32;
_17 = _15;
_1 = 6450561390876000806_u64;
match _1 {
0 => bb10,
6450561390876000806 => bb12,
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
_15 = _17;
_6 = _12.0;
_15 = [_2,_2,_2,_4,_4,_4,_2];
_1 = _7 as u64;
_8 = _18;
_20 = !_12.0;
_9 = _12;
match _5 {
28933 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_12 = _9;
_9.0 = 3775035512_u32 as isize;
_22 = [_18,_18,_18,_8,_18,_8,_8,_7];
_1 = !17157530482024829555_u64;
_7 = _8 | _18;
_17 = [_2,_2,_4,_4,_2,_2,_2];
RET = [_8,_8,_8,_7,_8,_7,_7,_18];
_1 = !755404403615049534_u64;
_20 = !_12.0;
_19 = _14 >= _6;
_23 = !_8;
_1 = !8381421347705491928_u64;
_7 = !_8;
_4 = _6 as u16;
RET = [_19,_7,_8,_23,_7,_19,_23,_8];
_7 = _18 | _23;
_16 = _2 as i32;
_8 = _7;
RET = _22;
_7 = !_19;
_6 = _20 - _14;
_12.0 = _11 as isize;
RET = [_23,_8,_8,_19,_8,_19,_19,_23];
_16 = -186546158_i32;
_9 = (_6,);
_4 = _2 * _2;
RET = [_19,_19,_18,_8,_18,_23,_23,_19];
_21 = 75_i8;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(1_usize, 11_usize, Move(_11), 22_usize, Move(_22), 8_usize, Move(_8), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(1_usize, 21_usize, Move(_21), 9_usize, Move(_9), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(1_usize, 3_usize, Move(_3), 18_usize, Move(_18), 2_usize, Move(_2), 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: usize,mut _2: isize,mut _3: bool) -> u16 {
mir! {
type RET = u16;
let _4: f32;
let _5: [u16; 7];
let _6: *const *const u128;
let _7: (i16,);
let _8: bool;
let _9: bool;
let _10: Adt51;
let _11: (isize,);
let _12: *mut (bool, [bool; 8], i8, u16, i16, i16);
let _13: (i16,);
let _14: char;
let _15: Adt50;
let _16: usize;
let _17: (f32,);
let _18: [bool; 8];
let _19: isize;
let _20: bool;
let _21: &'static bool;
let _22: *const u128;
let _23: f32;
let _24: isize;
let _25: *const u128;
let _26: i16;
let _27: isize;
let _28: [bool; 8];
let _29: [bool; 8];
let _30: Adt42;
let _31: (i32, i8);
let _32: ();
let _33: ();
{
RET = 3721_u16 ^ 46072_u16;
_2 = 9223372036854775807_isize * 61_isize;
RET = 52306_u16 * 46343_u16;
RET = !25034_u16;
_4 = 85_i8 as f32;
_3 = _1 == _1;
_3 = _4 != _4;
_2 = 11_isize >> _1;
_1 = 3_usize;
RET = !41956_u16;
_2 = (-75_i8) as isize;
_3 = !true;
_5[_1] = RET;
RET = !_5[_1];
_2 = _4 as isize;
_5 = [RET,RET,RET,RET,RET,RET,RET];
Call(_5 = fn3(_2, _4, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 9223372036854775807_isize;
_7.0 = RET as i16;
_7.0 = (-9758_i16);
RET = !52309_u16;
_5 = [RET,RET,RET,RET,RET,RET,RET];
_8 = !_3;
RET = !25094_u16;
_1 = !6601400195540032484_usize;
_2 = 9223372036854775807_isize;
RET = !38957_u16;
_9 = _8;
_7.0 = 27206_i16;
_1 = _4 as usize;
_2 = (-2502183399381417428_i64) as isize;
_10.fld3.fld5.0 = _4;
_10.fld0.2 = 71424309597847191605080086599113245232_i128 as i8;
_7 = (31950_i16,);
_10.fld3.fld5.0 = _4;
_1 = 0_usize + 16949354397518709255_usize;
_10.fld3.fld4 = _2 as u64;
_10.fld0.1 = [_8,_9,_8,_8,_8,_3,_9,_3];
_3 = !_9;
_10.fld3.fld4 = 6579579995517582689_u64;
Call(_10.fld0.5 = core::intrinsics::transmute(_7.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10.fld0.0 = _3 < _3;
_11 = (_2,);
_10.fld0.4 = _7.0;
_10.fld0.3 = !RET;
_13 = _7;
_10.fld3.fld1 = Adt38::Variant0 { fld0: (-134650614079974695172728560264881503911_i128),fld1: '\u{4b01f}',fld2: _5 };
place!(Field::<char>(Variant(_10.fld3.fld1, 0), 1)) = '\u{3c63b}';
_10.fld0.0 = _9;
RET = _10.fld0.3 | _10.fld0.3;
_10.fld3.fld5.0 = _4 + _4;
_10.fld1 = Field::<char>(Variant(_10.fld3.fld1, 0), 1);
_10.fld3.fld5 = (_4,);
_14 = _10.fld1;
_10.fld3.fld5 = (_4,);
_12 = core::ptr::addr_of_mut!(_10.fld0);
RET = (*_12).3;
_10.fld2 = core::ptr::addr_of_mut!(_10.fld3.fld0);
_1 = 15406011444607068192_usize;
place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)) = 147018855031883029111021588116670887841_i128;
(*_12).4 = RET as i16;
_10.fld0.2 = !93_i8;
_10.fld1 = _14;
match _1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
15406011444607068192 => bb8,
_ => bb7
}
}
bb3 = {
_2 = 9223372036854775807_isize;
_7.0 = RET as i16;
_7.0 = (-9758_i16);
RET = !52309_u16;
_5 = [RET,RET,RET,RET,RET,RET,RET];
_8 = !_3;
RET = !25094_u16;
_1 = !6601400195540032484_usize;
_2 = 9223372036854775807_isize;
RET = !38957_u16;
_9 = _8;
_7.0 = 27206_i16;
_1 = _4 as usize;
_2 = (-2502183399381417428_i64) as isize;
_10.fld3.fld5.0 = _4;
_10.fld0.2 = 71424309597847191605080086599113245232_i128 as i8;
_7 = (31950_i16,);
_10.fld3.fld5.0 = _4;
_1 = 0_usize + 16949354397518709255_usize;
_10.fld3.fld4 = _2 as u64;
_10.fld0.1 = [_8,_9,_8,_8,_8,_3,_9,_3];
_3 = !_9;
_10.fld3.fld4 = 6579579995517582689_u64;
Call(_10.fld0.5 = core::intrinsics::transmute(_7.0), ReturnTo(bb2), UnwindUnreachable())
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
(*_12).2 = 22_i8;
_10.fld0.2 = (-108_i8);
_10.fld3.fld5 = (_4,);
(*_12).3 = RET & RET;
_10.fld0.5 = _9 as i16;
_10.fld0.3 = 4164418643_u32 as u16;
_8 = !(*_12).0;
RET = 1365419081_u32 as u16;
(*_12).0 = _4 != _10.fld3.fld5.0;
SetDiscriminant(_10.fld3.fld1, 0);
_10.fld0.5 = _10.fld3.fld4 as i16;
_7 = (_13.0,);
_10.fld0.3 = RET;
RET = !(*_12).3;
_10.fld4 = _13.0 >> (*_12).2;
place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)) = !(-14392064077672051053066712544007230689_i128);
_16 = _1;
_9 = _3;
_7 = ((*_12).4,);
(*_12).3 = RET & RET;
_3 = _10.fld3.fld4 <= _10.fld3.fld4;
place!(Field::<[u16; 7]>(Variant(_10.fld3.fld1, 0), 2)) = [_10.fld0.3,(*_12).3,(*_12).3,RET,_10.fld0.3,(*_12).3,(*_12).3];
_10.fld0.3 = _1 as u16;
_10.fld1 = _14;
(*_12).2 = 113_i8;
_17.0 = -_10.fld3.fld5.0;
Call((*_12).4 = fn4((*_12).5, _10.fld4, _17.0, (*_12).0, _13.0, _10.fld3.fld5, _11, _13, _1, (*_12).1, _11.0, _14), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13 = ((*_12).5,);
Goto(bb10)
}
bb10 = {
_10.fld4 = _16 as i16;
_14 = _10.fld1;
_10.fld3.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)));
_9 = !_3;
(*_12).3 = !RET;
_9 = _3;
_10.fld3.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)));
place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)) = !(-3276262762654996280845194312159243782_i128);
_10.fld0.2 = 27_i8;
_20 = !_3;
_16 = (*_12).4 as usize;
(*_12).2 = 49_i8 + 90_i8;
_3 = _9 | _9;
_8 = !_20;
_17 = (_4,);
_4 = -_10.fld3.fld5.0;
_17 = (_10.fld3.fld5.0,);
(*_12).3 = RET;
(*_12).3 = RET;
_6 = core::ptr::addr_of!(_22);
_15.fld1 = Adt38::Variant0 { fld0: Field::<i128>(Variant(_10.fld3.fld1, 0), 0),fld1: _14,fld2: Field::<[u16; 7]>(Variant(_10.fld3.fld1, 0), 2) };
_15.fld0 = -_10.fld0.5;
place!(Field::<i128>(Variant(_15.fld1, 0), 0)) = Field::<char>(Variant(_15.fld1, 0), 1) as i128;
_6 = core::ptr::addr_of!((*_6));
Goto(bb11)
}
bb11 = {
_10.fld0.3 = RET * RET;
place!(Field::<char>(Variant(_10.fld3.fld1, 0), 1)) = Field::<char>(Variant(_15.fld1, 0), 1);
_15.fld1 = Move(_10.fld3.fld1);
_10.fld6 = core::ptr::addr_of!((*_6));
(*_12).4 = _10.fld3.fld4 as i16;
_7 = _13;
(*_12).1 = [_3,_3,_9,_3,_9,_3,_8,_20];
(*_12).1 = [_8,_9,_3,_3,_3,(*_12).0,(*_12).0,_3];
_10.fld2 = core::ptr::addr_of_mut!(_10.fld3.fld0);
_18 = [_3,_3,(*_12).0,_10.fld0.0,_8,_9,_3,_20];
_10.fld2 = core::ptr::addr_of_mut!(_10.fld3.fld0);
(*_12).0 = _10.fld0.4 == _13.0;
(*_12).4 = _10.fld0.5;
(*_12).2 = -70_i8;
(*_12).3 = RET;
(*_12).5 = _14 as i16;
_24 = _11.0;
Call(_10.fld0.5 = core::intrinsics::transmute((*_12).3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10.fld0.5 = _10.fld0.4;
_23 = _17.0 + _10.fld3.fld5.0;
_1 = _16;
_10.fld0.4 = -_7.0;
_17 = (_23,);
place!(Field::<char>(Variant(_15.fld1, 0), 1)) = _10.fld1;
_23 = -_10.fld3.fld5.0;
(*_12).0 = _3;
_23 = _4 * _17.0;
(*_12).1 = _18;
_21 = &(*_12).0;
(*_12).0 = _23 <= _23;
_9 = !(*_12).0;
(*_12).4 = _16 as i16;
_10.fld3.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_15.fld1, 0), 0)));
_27 = !_2;
Goto(bb13)
}
bb13 = {
_10.fld3.fld1 = Move(_15.fld1);
SetDiscriminant(_10.fld3.fld1, 0);
(*_12).0 = _14 > _14;
_10.fld0.3 = RET;
_10.fld1 = _14;
(*_12) = (_3, _18, 62_i8, RET, _15.fld0, _13.0);
RET = _10.fld0.3;
_14 = _10.fld1;
_10.fld3.fld5.0 = _17.0;
_7.0 = !(*_12).5;
place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)) = 6552911091881999295668018911714253691_i128;
_10.fld0.1 = [_8,_8,_10.fld0.0,_9,_20,(*_12).0,_9,(*_12).0];
_10.fld0.3 = !RET;
_21 = &(*_12).0;
_28 = _10.fld0.1;
_13 = ((*_12).4,);
_11.0 = 2955723907_u32 as isize;
_10.fld0.3 = RET | RET;
_10.fld3.fld1 = Adt38::Variant0 { fld0: 143710038201335752138953387584267217370_i128,fld1: _10.fld1,fld2: _5 };
_15.fld1 = Adt38::Variant0 { fld0: 150422652326617289604285694577889793387_i128,fld1: _10.fld1,fld2: Field::<[u16; 7]>(Variant(_10.fld3.fld1, 0), 2) };
match (*_12).2 {
0 => bb4,
62 => bb15,
_ => bb14
}
}
bb14 = {
_10.fld0.3 = RET * RET;
place!(Field::<char>(Variant(_10.fld3.fld1, 0), 1)) = Field::<char>(Variant(_15.fld1, 0), 1);
_15.fld1 = Move(_10.fld3.fld1);
_10.fld6 = core::ptr::addr_of!((*_6));
(*_12).4 = _10.fld3.fld4 as i16;
_7 = _13;
(*_12).1 = [_3,_3,_9,_3,_9,_3,_8,_20];
(*_12).1 = [_8,_9,_3,_3,_3,(*_12).0,(*_12).0,_3];
_10.fld2 = core::ptr::addr_of_mut!(_10.fld3.fld0);
_18 = [_3,_3,(*_12).0,_10.fld0.0,_8,_9,_3,_20];
_10.fld2 = core::ptr::addr_of_mut!(_10.fld3.fld0);
(*_12).0 = _10.fld0.4 == _13.0;
(*_12).4 = _10.fld0.5;
(*_12).2 = -70_i8;
(*_12).3 = RET;
(*_12).5 = _14 as i16;
_24 = _11.0;
Call(_10.fld0.5 = core::intrinsics::transmute((*_12).3), ReturnTo(bb12), UnwindUnreachable())
}
bb15 = {
_11 = (_24,);
_5 = Field::<[u16; 7]>(Variant(_10.fld3.fld1, 0), 2);
(*_12).1 = [_3,(*_21),_3,_9,(*_21),_10.fld0.0,(*_12).0,(*_21)];
_7 = ((*_12).4,);
_17 = _10.fld3.fld5;
_28 = [_9,(*_21),_10.fld0.0,_10.fld0.0,(*_21),_3,(*_21),(*_12).0];
_8 = !_9;
_10.fld3.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_10.fld3.fld1, 0), 0)));
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(2_usize, 8_usize, Move(_8), 28_usize, Move(_28), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(2_usize, 7_usize, Move(_7), 9_usize, Move(_9), 20_usize, Move(_20), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: f32,mut _3: usize,mut _4: f32) -> [u16; 7] {
mir! {
type RET = [u16; 7];
let _5: isize;
let _6: Adt49;
let _7: (i32, i8);
let _8: &'static bool;
let _9: isize;
let _10: Adt42;
let _11: u32;
let _12: ();
let _13: ();
{
_5 = 233841439483951220761950060585809588305_u128 as isize;
RET = [10967_u16,60763_u16,14310_u16,61028_u16,21153_u16,58457_u16,32179_u16];
_1 = _5 - _5;
_1 = _5;
match RET[_3] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
61028 => bb6,
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
_2 = _4;
_6.fld6.fld1 = Adt38::Variant0 { fld0: 18155847037163383751033445923043060129_i128,fld1: '\u{fc0bd}',fld2: RET };
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)) = [RET[_3],RET[_3],RET[_3],RET[_3],RET[_3],RET[_3],RET[_3]];
_6.fld1 = [Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],RET[_3],RET[_3],RET[_3],RET[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],RET[_3]];
_6.fld6.fld1 = Adt38::Variant0 { fld0: (-49684732421192770786659915446907954052_i128),fld1: '\u{9bbaa}',fld2: _6.fld1 };
_6.fld4 = (-45_i8) as u64;
_4 = _2 - _2;
place!(Field::<i128>(Variant(_6.fld6.fld1, 0), 0)) = !126884991461146764663844172236062083983_i128;
_6.fld6.fld5 = (_2,);
_6.fld6.fld5.0 = _4;
_6.fld5 = Adt41::Variant3 { fld0: RET };
_7.1 = (-6_i8) + 24_i8;
RET[_3] = !_6.fld1[_3];
_6.fld6.fld4 = (-479275368_i32) as u64;
match Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3] {
0 => bb1,
1 => bb2,
61028 => bb7,
_ => bb5
}
}
bb7 = {
_6.fld6.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_6.fld6.fld1, 0), 0)));
_7 = ((-1246674360_i32), 75_i8);
_5 = _4 as isize;
_6.fld6.fld4 = Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3] as u64;
_6.fld1[_3] = !RET[_3];
_7 = ((-1924509080_i32), (-74_i8));
_7 = (1630226901_i32, (-107_i8));
_6.fld6.fld5.0 = _2 - _4;
_6.fld6.fld5.0 = -_2;
_3 = 2_usize;
_7.0 = 136920012_i32;
_8 = &_6.fld0;
_6.fld6.fld1 = Adt38::Variant0 { fld0: 168853363543247975773964974221923927091_i128,fld1: '\u{4b521}',fld2: _6.fld1 };
_4 = -_6.fld6.fld5.0;
place!(Field::<char>(Variant(_6.fld6.fld1, 0), 1)) = '\u{9706c}';
_4 = -_2;
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2))[_3] = _6.fld1[_3];
_6.fld0 = true;
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)) = [Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],RET[_3],_6.fld1[_3]];
Goto(bb8)
}
bb8 = {
_9 = _1 - _5;
_6.fld6.fld5 = (_2,);
RET = [_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],_6.fld1[_3],_6.fld1[_3]];
place!(Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)) = [_6.fld1[_3],RET[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],RET[_3],RET[_3],RET[_3]];
RET[_3] = _6.fld1[_3];
_6.fld6.fld4 = !_6.fld4;
_6.fld1 = [RET[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],RET[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3]];
place!(Field::<char>(Variant(_6.fld6.fld1, 0), 1)) = '\u{13389}';
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)) = [RET[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],RET[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3]];
_6.fld0 = !true;
RET = Field::<[u16; 7]>(Variant(_6.fld5, 3), 0);
SetDiscriminant(_6.fld5, 3);
place!(Field::<char>(Variant(_6.fld6.fld1, 0), 1)) = '\u{9b11}';
place!(Field::<[u16; 7]>(Variant(_6.fld5, 3), 0))[_3] = RET[_3] ^ Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3];
RET[_3] = 4213420448_u32 as u16;
_6.fld5 = Adt41::Variant3 { fld0: Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2) };
place!(Field::<i128>(Variant(_6.fld6.fld1, 0), 0)) = Field::<char>(Variant(_6.fld6.fld1, 0), 1) as i128;
_3 = 226_u8 as usize;
_8 = &_6.fld0;
_7.0 = (-691239527_i32) - 1095773410_i32;
match _7.1 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb5,
5 => bb9,
340282366920938463463374607431768211349 => bb11,
_ => bb10
}
}
bb9 = {
_6.fld6.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_6.fld6.fld1, 0), 0)));
_7 = ((-1246674360_i32), 75_i8);
_5 = _4 as isize;
_6.fld6.fld4 = Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3] as u64;
_6.fld1[_3] = !RET[_3];
_7 = ((-1924509080_i32), (-74_i8));
_7 = (1630226901_i32, (-107_i8));
_6.fld6.fld5.0 = _2 - _4;
_6.fld6.fld5.0 = -_2;
_3 = 2_usize;
_7.0 = 136920012_i32;
_8 = &_6.fld0;
_6.fld6.fld1 = Adt38::Variant0 { fld0: 168853363543247975773964974221923927091_i128,fld1: '\u{4b521}',fld2: _6.fld1 };
_4 = -_6.fld6.fld5.0;
place!(Field::<char>(Variant(_6.fld6.fld1, 0), 1)) = '\u{9706c}';
_4 = -_2;
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2))[_3] = _6.fld1[_3];
_6.fld0 = true;
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)) = [Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],RET[_3],_6.fld1[_3]];
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_1 = !_5;
_6.fld6.fld5 = (_4,);
_6.fld0 = true;
match _7.1 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211349 => bb16,
_ => bb15
}
}
bb12 = {
Return()
}
bb13 = {
_6.fld6.fld0 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_6.fld6.fld1, 0), 0)));
_7 = ((-1246674360_i32), 75_i8);
_5 = _4 as isize;
_6.fld6.fld4 = Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3] as u64;
_6.fld1[_3] = !RET[_3];
_7 = ((-1924509080_i32), (-74_i8));
_7 = (1630226901_i32, (-107_i8));
_6.fld6.fld5.0 = _2 - _4;
_6.fld6.fld5.0 = -_2;
_3 = 2_usize;
_7.0 = 136920012_i32;
_8 = &_6.fld0;
_6.fld6.fld1 = Adt38::Variant0 { fld0: 168853363543247975773964974221923927091_i128,fld1: '\u{4b521}',fld2: _6.fld1 };
_4 = -_6.fld6.fld5.0;
place!(Field::<char>(Variant(_6.fld6.fld1, 0), 1)) = '\u{9706c}';
_4 = -_2;
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2))[_3] = _6.fld1[_3];
_6.fld0 = true;
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)) = [Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],_6.fld1[_3],Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3],RET[_3],_6.fld1[_3]];
Goto(bb8)
}
bb14 = {
_2 = _4;
_6.fld6.fld1 = Adt38::Variant0 { fld0: 18155847037163383751033445923043060129_i128,fld1: '\u{fc0bd}',fld2: RET };
place!(Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)) = [RET[_3],RET[_3],RET[_3],RET[_3],RET[_3],RET[_3],RET[_3]];
_6.fld1 = [Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],RET[_3],RET[_3],RET[_3],RET[_3],Field::<[u16; 7]>(Variant(_6.fld6.fld1, 0), 2)[_3],RET[_3]];
_6.fld6.fld1 = Adt38::Variant0 { fld0: (-49684732421192770786659915446907954052_i128),fld1: '\u{9bbaa}',fld2: _6.fld1 };
_6.fld4 = (-45_i8) as u64;
_4 = _2 - _2;
place!(Field::<i128>(Variant(_6.fld6.fld1, 0), 0)) = !126884991461146764663844172236062083983_i128;
_6.fld6.fld5 = (_2,);
_6.fld6.fld5.0 = _4;
_6.fld5 = Adt41::Variant3 { fld0: RET };
_7.1 = (-6_i8) + 24_i8;
RET[_3] = !_6.fld1[_3];
_6.fld6.fld4 = (-479275368_i32) as u64;
match Field::<[u16; 7]>(Variant(_6.fld5, 3), 0)[_3] {
0 => bb1,
1 => bb2,
61028 => bb7,
_ => bb5
}
}
bb15 = {
Return()
}
bb16 = {
Goto(bb17)
}
bb17 = {
Call(_12 = dump_var(3_usize, 3_usize, Move(_3), 7_usize, Move(_7), 13_usize, _13, 13_usize, _13), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i16,mut _2: i16,mut _3: f32,mut _4: bool,mut _5: i16,mut _6: (f32,),mut _7: (isize,),mut _8: (i16,),mut _9: usize,mut _10: [bool; 8],mut _11: isize,mut _12: char) -> i16 {
mir! {
type RET = i16;
let _13: (i32, i8);
let _14: ((isize,), (i32, i8), usize);
let _15: [u16; 7];
let _16: isize;
let _17: (i32, i8);
let _18: *const *const u128;
let _19: (isize,);
let _20: (isize,);
let _21: u32;
let _22: f64;
let _23: (f32,);
let _24: usize;
let _25: f32;
let _26: ();
let _27: ();
{
_8 = (_2,);
_3 = _6.0;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
15406011444607068192 => bb8,
_ => bb7
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
RET = _6.0 as i16;
_8 = (RET,);
_6 = (_3,);
_14.1.1 = (-82_i8) ^ (-102_i8);
_13.0 = 1715553083_i32;
_6 = (_3,);
_14.1.0 = _13.0;
_14.0 = _7;
_8 = (_1,);
_16 = !_11;
_10 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = !_8.0;
_14.0 = _7;
Call(_14.1 = fn5(_9, _4, _12, _12, _8.0, _7.0, _9, _7, _14.0, _5, _6.0, _7, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_13.0 = _14.1.0;
_5 = -_1;
_8 = (_2,);
_15 = [16208_u16,51934_u16,19375_u16,4691_u16,55410_u16,45087_u16,62900_u16];
Goto(bb10)
}
bb10 = {
_8.0 = _5;
_13.1 = _1 as i8;
_4 = false;
Goto(bb11)
}
bb11 = {
_6 = (_3,);
_6 = (_3,);
_11 = !_14.0.0;
_14 = (_7, _13, _9);
_7.0 = _11;
_17.1 = 45764_u16 as i8;
_13.0 = _14.1.0 >> _1;
_8.0 = _2;
_13.1 = _17.1;
_10 = [_4,_4,_4,_4,_4,_4,_4,_4];
_14 = (_7, _13, _9);
_14.2 = _9 - _9;
match _9 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb12,
15406011444607068192 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_8.0 = _2 << _1;
_14.2 = !_9;
_20.0 = !_7.0;
_14.0.0 = _7.0 & _11;
_20.0 = -_7.0;
_23 = (_3,);
_14.1.1 = -_17.1;
_17 = _13;
_3 = _23.0;
RET = -_1;
_8 = (_2,);
_23 = _6;
_8 = (_1,);
_14.0 = _20;
_14.0.0 = _14.1.1 as isize;
_16 = _7.0 * _11;
_19.0 = _14.0.0 * _16;
_19.0 = 2066941615203411381_u64 as isize;
_25 = -_6.0;
_4 = true;
_23.0 = 143543416030271541848395300817276311901_i128 as f32;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(4_usize, 15_usize, Move(_15), 1_usize, Move(_1), 19_usize, Move(_19), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(4_usize, 7_usize, Move(_7), 11_usize, Move(_11), 17_usize, Move(_17), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: usize,mut _2: bool,mut _3: char,mut _4: char,mut _5: i16,mut _6: isize,mut _7: usize,mut _8: (isize,),mut _9: (isize,),mut _10: i16,mut _11: f32,mut _12: (isize,),mut _13: (i16,)) -> (i32, i8) {
mir! {
type RET = (i32, i8);
let _14: bool;
let _15: (bool, [bool; 8], i8, u16, i16, i16);
let _16: [bool; 8];
let _17: isize;
let _18: (bool, [bool; 8], i8, u16, i16, i16);
let _19: ((isize,), (i32, i8), usize);
let _20: Adt37;
let _21: isize;
let _22: Adt40;
let _23: ((isize,), (i32, i8), usize);
let _24: char;
let _25: [bool; 8];
let _26: *const u128;
let _27: Adt46;
let _28: (bool, [bool; 8], i8, u16, i16, i16);
let _29: (i16,);
let _30: isize;
let _31: (i32, i8);
let _32: (isize,);
let _33: isize;
let _34: (isize,);
let _35: Adt44;
let _36: ((isize,), (i32, i8), usize);
let _37: (i16,);
let _38: bool;
let _39: isize;
let _40: Adt37;
let _41: [u16; 7];
let _42: Adt44;
let _43: Adt40;
let _44: bool;
let _45: i32;
let _46: bool;
let _47: Adt46;
let _48: *const *const u128;
let _49: (isize,);
let _50: *mut (bool, [bool; 8], i8, u16, i16, i16);
let _51: ();
let _52: ();
{
_9.0 = _11 as isize;
_13 = (_10,);
RET.1 = 105_i8 << _7;
_9 = (_8.0,);
_12.0 = !_6;
_1 = !_7;
_5 = -_10;
_8.0 = _12.0;
RET = (352161735_i32, 25_i8);
Goto(bb1)
}
bb1 = {
_15.0 = !_2;
_9.0 = _8.0;
_14 = !_2;
RET = ((-1342136988_i32), (-7_i8));
_11 = (-8942374909683877984_i64) as f32;
_15.2 = RET.1 - RET.1;
_15.0 = _7 == _7;
_15.3 = 3583_u16 - 38563_u16;
_13.0 = !_10;
_7 = _1;
RET.1 = (-784476200425047215_i64) as i8;
_10 = 323365709695299220690084153281868947254_u128 as i16;
_11 = 192_u8 as f32;
_1 = !_7;
_17 = _10 as isize;
RET.0 = 8015877379146248998_i64 as i32;
_18.1 = [_15.0,_14,_15.0,_2,_15.0,_15.0,_14,_14];
_15.0 = _14;
_18.3 = _15.3;
_14 = _2;
RET = (1045279510_i32, _15.2);
_15.3 = _12.0 as u16;
_19.0.0 = !_8.0;
Call(_15.5 = fn6(_18.3, RET.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18.2 = _15.2;
_12 = (_19.0.0,);
_12 = (_9.0,);
RET.0 = 1078362371_i32;
_19.2 = !_1;
_15.1 = [_14,_14,_2,_2,_2,_14,_15.0,_15.0];
_1 = !_19.2;
_18.0 = !_2;
_14 = _15.0 | _15.0;
_3 = _4;
_10 = _17 as i16;
_18.4 = _11 as i16;
_15.2 = !RET.1;
_18 = (_14, _15.1, _15.2, _15.3, _10, _13.0);
_16 = [_2,_14,_14,_14,_2,_14,_14,_18.0];
_18.1 = _15.1;
_19.0 = (_6,);
_12 = (_6,);
_13 = (_18.4,);
match RET.0 {
0 => bb1,
1 => bb3,
2 => bb4,
1078362371 => bb6,
_ => bb5
}
}
bb3 = {
_15.0 = !_2;
_9.0 = _8.0;
_14 = !_2;
RET = ((-1342136988_i32), (-7_i8));
_11 = (-8942374909683877984_i64) as f32;
_15.2 = RET.1 - RET.1;
_15.0 = _7 == _7;
_15.3 = 3583_u16 - 38563_u16;
_13.0 = !_10;
_7 = _1;
RET.1 = (-784476200425047215_i64) as i8;
_10 = 323365709695299220690084153281868947254_u128 as i16;
_11 = 192_u8 as f32;
_1 = !_7;
_17 = _10 as isize;
RET.0 = 8015877379146248998_i64 as i32;
_18.1 = [_15.0,_14,_15.0,_2,_15.0,_15.0,_14,_14];
_15.0 = _14;
_18.3 = _15.3;
_14 = _2;
RET = (1045279510_i32, _15.2);
_15.3 = _12.0 as u16;
_19.0.0 = !_8.0;
Call(_15.5 = fn6(_18.3, RET.1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_19.2 = 140_u8 as usize;
_11 = 12_u8 as f32;
_12.0 = -_8.0;
_15.4 = _11 as i16;
_18.3 = _11 as u16;
_15.0 = _18.0;
_16 = _15.1;
_13 = (_15.4,);
_15.3 = _18.3;
_7 = !_1;
_15.4 = !_5;
RET.0 = -2088858107_i32;
_6 = 79983168235500748017658594656888315901_i128 as isize;
_19.1.0 = RET.0 | RET.0;
_5 = 504608804_u32 as i16;
_12 = (_9.0,);
_11 = 17860131264958830893_u64 as f32;
_19 = (_8, RET, _1);
_13 = (_18.4,);
_9.0 = _12.0 << RET.0;
_12 = (_8.0,);
_1 = !_19.2;
Goto(bb7)
}
bb7 = {
RET = _19.1;
_7 = !_1;
_15.1 = [_14,_18.0,_15.0,_18.0,_15.0,_14,_18.0,_15.0];
_6 = !_19.0.0;
_11 = _18.5 as f32;
_23.0.0 = _6;
_20 = Adt37::Variant1 { fld0: _11 };
_18.0 = !_14;
_15 = _18;
RET = (_19.1.0, _19.1.1);
_2 = _12.0 <= _12.0;
_9 = _12;
RET.1 = !_19.1.1;
Goto(bb8)
}
bb8 = {
_13.0 = -_5;
_18.4 = Field::<f32>(Variant(_20, 1), 0) as i16;
_9 = (_8.0,);
_23 = (_8, _19.1, _19.2);
_8 = (_19.0.0,);
_19.1 = (_23.1.0, _15.2);
Goto(bb9)
}
bb9 = {
_18.0 = _14;
_3 = _4;
_12.0 = -_19.0.0;
_13 = (_15.4,);
_15 = (_14, _18.1, _19.1.1, _18.3, _5, _18.4);
Goto(bb10)
}
bb10 = {
SetDiscriminant(_20, 1);
_18 = (_14, _15.1, _19.1.1, _15.3, _13.0, _15.4);
_18.3 = !_15.3;
_19.1.0 = !_23.1.0;
_15.5 = _10;
_17 = _23.0.0 | _23.0.0;
place!(Field::<f32>(Variant(_20, 1), 0)) = _11;
_18.1 = [_15.0,_18.0,_2,_15.0,_15.0,_18.0,_14,_15.0];
RET = _23.1;
_6 = !_23.0.0;
_19.1.1 = _15.2 & _18.2;
_25 = [_15.0,_18.0,_14,_18.0,_2,_14,_15.0,_2];
_19.0.0 = _12.0 << _17;
_23 = (_9, RET, _1);
_18 = (_15.0, _25, _19.1.1, _15.3, _5, _15.4);
RET.1 = !_18.2;
_19.1 = (RET.0, _18.2);
_18.2 = !_19.1.1;
_16 = _18.1;
_4 = _3;
Goto(bb11)
}
bb11 = {
_15 = _18;
RET = (_19.1.0, _19.1.1);
_23 = (_12, _19.1, _7);
_30 = _8.0 * _8.0;
_23.0.0 = _19.0.0 ^ _9.0;
_28.2 = _15.2;
_28.1 = [_18.0,_2,_15.0,_18.0,_2,_15.0,_14,_14];
_24 = _3;
_31.1 = (-54948348003848392018930699261925451227_i128) as i8;
_19.2 = _1 ^ _7;
_30 = Field::<f32>(Variant(_20, 1), 0) as isize;
_29.0 = _15.3 as i16;
_33 = 250136129_u32 as isize;
place!(Field::<f32>(Variant(_20, 1), 0)) = _15.2 as f32;
SetDiscriminant(_20, 0);
_31.1 = !_18.2;
place!(Field::<i16>(Variant(_20, 0), 2)) = _5 | _10;
_2 = _18.0 > _18.0;
_21 = -_6;
_36.0.0 = !_6;
_36 = _23;
Goto(bb12)
}
bb12 = {
_20 = Adt37::Variant0 { fld0: 105_u8,fld1: _18.3,fld2: _5,fld3: RET.1 };
_29.0 = _4 as i16;
_23.0 = (_36.0.0,);
RET = (_23.1.0, _19.1.1);
place!(Field::<u8>(Variant(_20, 0), 0)) = 137_u8;
RET.1 = _36.1.1 << _18.5;
_14 = _2;
_18.1 = _28.1;
_9 = (_33,);
_37.0 = _29.0;
place!(Field::<i16>(Variant(_20, 0), 2)) = !_29.0;
_36 = (_9, _23.1, _19.2);
_15.0 = !_2;
_15.3 = Field::<i8>(Variant(_20, 0), 3) as u16;
_28.3 = _15.3;
_23.0 = (_36.0.0,);
_19.0 = (_33,);
_19.1.0 = -_23.1.0;
_31 = (_23.1.0, _36.1.1);
_28.4 = !_18.4;
Goto(bb13)
}
bb13 = {
_32.0 = !_36.0.0;
_18.3 = !_28.3;
_28.3 = _4 as u16;
RET.0 = -_23.1.0;
_39 = _12.0;
_5 = _18.4;
_8.0 = _17 + _32.0;
_8.0 = !_36.0.0;
_23.1 = RET;
_15.1 = [_14,_15.0,_15.0,_2,_15.0,_2,_2,_14];
SetDiscriminant(_20, 2);
_36.1 = _19.1;
_19.0 = (_8.0,);
_37.0 = _15.5;
_28.1 = _15.1;
place!(Field::<u64>(Variant(_20, 2), 5)) = 12706384487102400329_u64 & 10154474116955129947_u64;
_28.1 = [_18.0,_18.0,_15.0,_15.0,_2,_18.0,_15.0,_15.0];
_28.2 = !_23.1.1;
_19.2 = 137_u8 as usize;
_28.0 = !_14;
_15.0 = _12.0 < _21;
Goto(bb14)
}
bb14 = {
_32 = (_36.0.0,);
_48 = core::ptr::addr_of!(place!(Field::<*const u128>(Variant(_20, 2), 3)));
_31.1 = _23.1.1 >> _15.2;
RET.1 = _36.1.1 & _28.2;
_23 = (_32, _19.1, _19.2);
_28.1 = [_14,_14,_14,_14,_2,_14,_28.0,_14];
_23.1 = _36.1;
_23.2 = _36.2 - _36.2;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(5_usize, 25_usize, Move(_25), 13_usize, Move(_13), 10_usize, Move(_10), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(5_usize, 21_usize, Move(_21), 8_usize, Move(_8), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(5_usize, 31_usize, Move(_31), 29_usize, Move(_29), 23_usize, Move(_23), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(5_usize, 12_usize, Move(_12), 16_usize, Move(_16), 19_usize, Move(_19), 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u16,mut _2: i8) -> i16 {
mir! {
type RET = i16;
let _3: f32;
let _4: *const i128;
let _5: (i16,);
let _6: (i32, i8);
let _7: f64;
let _8: [u16; 7];
let _9: [bool; 8];
let _10: (bool, [bool; 8], i8, u16, i16, i16);
let _11: ((isize,), (i32, i8), usize);
let _12: f32;
let _13: [u16; 7];
let _14: isize;
let _15: (bool, [bool; 8], i8, u16, i16, i16);
let _16: i32;
let _17: (isize,);
let _18: Adt38;
let _19: Adt37;
let _20: i32;
let _21: char;
let _22: (i16,);
let _23: *mut (bool, [bool; 8], i8, u16, i16, i16);
let _24: ();
let _25: ();
{
RET = _1 as i16;
RET = (-686_i16) * (-22736_i16);
_5.0 = RET;
_6 = ((-2128508657_i32), _2);
_5.0 = 84_u8 as i16;
_6 = (559454193_i32, _2);
_6.0 = _1 as i32;
_5 = (RET,);
_6 = (1932522791_i32, _2);
_5 = (RET,);
_6.0 = !849295544_i32;
RET = _5.0 - _5.0;
_5 = (RET,);
_5 = (RET,);
_6 = ((-19730634_i32), _2);
_1 = !23795_u16;
_1 = !27149_u16;
_7 = 2091962350_u32 as f64;
match _6.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431748480822 => bb7,
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
_6 = (1264529482_i32, _2);
_8 = [_1,_1,_1,_1,_1,_1,_1];
_6.0 = _6.1 as i32;
_7 = _1 as f64;
Call(RET = fn7(_5.0, _5, _6, _5.0, _6.0, _2, _1, _7, _7, _7, _5, _6, _5.0, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5.0 = RET << _6.1;
_7 = _6.0 as f64;
RET = 12_u8 as i16;
_9 = [true,true,true,false,false,true,false,false];
_5.0 = RET * RET;
_7 = 1204602070420214149_u64 as f64;
_3 = 6_u8 as f32;
_2 = -_6.1;
_9 = [true,true,false,false,true,false,true,true];
_3 = 12193271967669554840_u64 as f32;
_9 = [true,true,false,false,true,true,false,true];
_8 = [_1,_1,_1,_1,_1,_1,_1];
RET = _5.0;
_5 = (RET,);
RET = !_5.0;
_6.1 = _2 << _6.0;
_3 = 126324339544575061927865458594848043411_i128 as f32;
_10 = (false, _9, _6.1, _1, RET, RET);
Goto(bb9)
}
bb9 = {
_3 = 1826681514_u32 as f32;
_1 = _10.3 & _10.3;
_6.0 = -1692604214_i32;
RET = _5.0;
_11.1.0 = -_6.0;
_5.0 = -RET;
_11.2 = 12125689722087837073_usize & 6609949580300836461_usize;
_11.1.1 = _10.2 << _6.1;
_6.1 = -_11.1.1;
_10.3 = _1;
_11.1.1 = _11.2 as i8;
_11.0.0 = _10.0 as isize;
_10.5 = !_5.0;
_8 = [_1,_1,_10.3,_10.3,_1,_10.3,_1];
_5 = (_10.5,);
_11.1.1 = _10.2;
_11.2 = _11.1.0 as usize;
_6.0 = 16453220267204617631_u64 as i32;
_10.4 = _10.5 | _10.5;
_8 = [_1,_10.3,_10.3,_10.3,_10.3,_10.3,_1];
_9 = _10.1;
_3 = RET as f32;
Goto(bb10)
}
bb10 = {
_1 = !_10.3;
_10.3 = _1 | _1;
_6.1 = -_11.1.1;
_10 = (false, _9, _11.1.1, _1, RET, RET);
_11.2 = 42978445760760462_u64 as usize;
_6 = (_11.1.0, _2);
_10.4 = _7 as i16;
_11.1.0 = -_6.0;
_10.0 = false;
_10.2 = _6.1 - _11.1.1;
_9 = _10.1;
_5.0 = _10.5 ^ _10.5;
_5.0 = _10.5 - _10.5;
_5.0 = !_10.5;
_10.5 = RET;
_10.5 = (-135518408574028050839748398684386225769_i128) as i16;
_12 = -_3;
_8 = [_10.3,_10.3,_10.3,_10.3,_10.3,_1,_10.3];
_11.0 = ((-106_isize),);
_9 = _10.1;
_6.1 = _11.1.1;
_10 = (false, _9, _2, _1, _5.0, RET);
_10.4 = 2074649822_u32 as i16;
Goto(bb11)
}
bb11 = {
_10.5 = _3 as i16;
_13 = _8;
_15.5 = 2230135766_u32 as i16;
_11.0 = (43_isize,);
_11.0.0 = !(-9223372036854775808_isize);
Goto(bb12)
}
bb12 = {
_15.1 = _9;
_7 = _12 as f64;
_7 = 2694221351612104390_i64 as f64;
RET = _11.1.0 as i16;
_6 = (_11.1.0, _10.2);
_11.0 = ((-75_isize),);
_17 = (_11.0.0,);
_10.3 = _1 * _1;
_6.1 = _10.2 >> _10.2;
_15 = (_10.0, _9, _6.1, _10.3, RET, _10.4);
_5 = (_10.5,);
_18 = Adt38::Variant0 { fld0: 78525445779117275715652794924397871596_i128,fld1: '\u{cd8d8}',fld2: _13 };
_4 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_18, 0), 0)));
_2 = -_6.1;
_19 = Adt37::Variant1 { fld0: _3 };
_1 = '\u{78f84}' as u16;
_17.0 = _11.0.0;
_18 = Adt38::Variant0 { fld0: (-41048659220322936535001029045339282567_i128),fld1: '\u{42625}',fld2: _8 };
_5.0 = !_15.5;
_6.0 = !_11.1.0;
_7 = _15.3 as f64;
_6.1 = _15.2 - _11.1.1;
_19 = Adt37::Variant0 { fld0: 41_u8,fld1: _10.3,fld2: _15.4,fld3: _10.2 };
_20 = _6.0;
place!(Field::<u8>(Variant(_19, 0), 0)) = !150_u8;
_13 = [Field::<u16>(Variant(_19, 0), 1),Field::<u16>(Variant(_19, 0), 1),Field::<u16>(Variant(_19, 0), 1),_10.3,_10.3,Field::<u16>(Variant(_19, 0), 1),_15.3];
Goto(bb13)
}
bb13 = {
_10 = (_15.0, _9, _15.2, Field::<u16>(Variant(_19, 0), 1), Field::<i16>(Variant(_19, 0), 2), _15.5);
_22 = (_15.5,);
_5.0 = _15.3 as i16;
_6.0 = _11.1.0;
place!(Field::<i128>(Variant(_18, 0), 0)) = (-40763910482484621132371530915650135468_i128);
place!(Field::<char>(Variant(_18, 0), 1)) = '\u{77e8}';
_6.0 = _11.1.0 ^ _11.1.0;
Goto(bb14)
}
bb14 = {
_1 = !Field::<u16>(Variant(_19, 0), 1);
_6.1 = _11.1.1 ^ _15.2;
_11.0 = (_17.0,);
_7 = _1 as f64;
_15.5 = _11.2 as i16;
_15.0 = !_10.0;
_10.2 = _6.1;
place!(Field::<i8>(Variant(_19, 0), 3)) = _15.2 - _6.1;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(6_usize, 5_usize, Move(_5), 8_usize, Move(_8), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(6_usize, 17_usize, Move(_17), 15_usize, Move(_15), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i16,mut _2: (i16,),mut _3: (i32, i8),mut _4: i16,mut _5: i32,mut _6: i8,mut _7: u16,mut _8: f64,mut _9: f64,mut _10: f64,mut _11: (i16,),mut _12: (i32, i8),mut _13: i16,mut _14: (i16,)) -> i16 {
mir! {
type RET = i16;
let _15: isize;
let _16: (bool, [bool; 8], i8, u16, i16, i16);
let _17: Adt52;
let _18: (f32,);
let _19: f32;
let _20: f64;
let _21: isize;
let _22: [u16; 7];
let _23: (i32, i8);
let _24: i64;
let _25: Adt43;
let _26: *mut *const i128;
let _27: isize;
let _28: (isize,);
let _29: (i32, i8);
let _30: f64;
let _31: i128;
let _32: char;
let _33: ();
let _34: ();
{
_9 = _10;
RET = _13;
_7 = '\u{f8e32}' as u16;
_10 = _9;
Call(_6 = fn8(_12, _3, _12.1, _2.0, _11, _14.0, _3.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _2.0;
_7 = 31048_u16 << _3.1;
_5 = -_12.0;
_11 = (RET,);
_10 = _8 - _9;
_1 = '\u{70c0b}' as i16;
_12.0 = !_5;
_16.4 = (-9223372036854775808_isize) as i16;
_3.0 = _5;
_16.5 = _2.0;
_16.1 = [false,false,false,true,false,true,false,true];
RET = _16.5 & _2.0;
Call(_17.fld1.3 = fn9(_7, _11, _5, _14, _16.1, _16.5, _12.0, _14.0, _6, _14.0, _14, _12.1, _6, _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _12.1 >> _16.5;
_12.1 = _6 >> _13;
_17.fld1.2 = -_6;
_17.fld1.1 = [false,false,false,false,false,false,true,true];
_17.fld1.0 = !true;
_18.0 = (-9223372036854775808_isize) as f32;
_21 = 4841371870627413115_i64 as isize;
_19 = -_18.0;
RET = _13;
_15 = _21 + _21;
_14 = (_11.0,);
_6 = _12.1 + _3.1;
_17.fld1.4 = 3408401567665664447_i64 as i16;
_17.fld1.4 = _17.fld1.0 as i16;
_16.3 = !_17.fld1.3;
_17.fld0.fld0 = _11.0;
_12.1 = _13 as i8;
_17.fld1.2 = !_6;
_15 = _21 + _21;
_21 = _15 | _15;
_15 = _21;
_16.0 = _17.fld1.0;
_17.fld2 = core::ptr::addr_of_mut!(_17.fld3);
_16.3 = !_7;
_17.fld1.5 = _13 << _17.fld1.2;
_16.3 = _17.fld1.3;
Goto(bb3)
}
bb3 = {
RET = 229_u8 as i16;
_17.fld1.0 = _16.0;
_6 = -_17.fld1.2;
_11 = (_17.fld1.5,);
_19 = _18.0 + _18.0;
_15 = _21 >> _21;
_14 = _11;
_11.0 = !_14.0;
_10 = 5_usize as f64;
_16.3 = !_7;
_18 = (_19,);
_23.1 = _6 - _17.fld1.2;
_17.fld1.2 = -_23.1;
_6 = !_23.1;
_15 = !_21;
Call(_12.1 = core::intrinsics::transmute(_17.fld1.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16.4 = _14.0 - _11.0;
_26 = core::ptr::addr_of_mut!(_17.fld3);
_23 = _12;
_11.0 = _17.fld1.5;
_17.fld1.1 = [_17.fld1.0,_17.fld1.0,_17.fld1.0,_16.0,_16.0,_17.fld1.0,_17.fld1.0,_17.fld1.0];
_23.1 = _12.1;
_17.fld2 = core::ptr::addr_of_mut!((*_26));
_17.fld1 = (_16.0, _16.1, _6, _16.3, _11.0, _14.0);
_2.0 = _11.0;
_3 = (_5, _17.fld1.2);
_22 = [_16.3,_16.3,_17.fld1.3,_16.3,_7,_17.fld1.3,_7];
_2.0 = 7_usize as i16;
_2.0 = _17.fld1.4;
_28 = (_15,);
_16.3 = _15 as u16;
_28.0 = _23.0 as isize;
Goto(bb5)
}
bb5 = {
_17.fld1.3 = _16.3 ^ _16.3;
_17.fld1.2 = _23.1;
_11 = (_16.5,);
_16.3 = !_17.fld1.3;
_7 = _17.fld1.3 + _16.3;
_5 = !_3.0;
_10 = _8 + _8;
_17.fld1.0 = _16.4 >= _17.fld1.5;
Goto(bb6)
}
bb6 = {
_17.fld2 = core::ptr::addr_of_mut!((*_26));
_17.fld1.4 = -_11.0;
_12 = _23;
_30 = 3120212514_u32 as f64;
_29.1 = _23.1 - _3.1;
_2.0 = 131269596681369205767538793347520593876_u128 as i16;
_29 = (_23.0, _23.1);
_17.fld0.fld1 = Adt38::Variant0 { fld0: 15978316281447254495101122657751431582_i128,fld1: '\u{598cc}',fld2: _22 };
_17.fld1.4 = _4;
place!(Field::<char>(Variant(_17.fld0.fld1, 0), 1)) = '\u{54b67}';
_29.1 = 49637360425314591950746197344095361226_u128 as i8;
_29.1 = _3.1 - _17.fld1.2;
_3.1 = _29.1;
Goto(bb7)
}
bb7 = {
_9 = -_10;
_17.fld1.4 = _16.4;
_17.fld2 = core::ptr::addr_of_mut!((*_26));
RET = _17.fld1.5;
_12.1 = _19 as i8;
Goto(bb8)
}
bb8 = {
Call(_33 = dump_var(7_usize, 7_usize, Move(_7), 1_usize, Move(_1), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_33 = dump_var(7_usize, 14_usize, Move(_14), 11_usize, Move(_11), 15_usize, Move(_15), 13_usize, Move(_13)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: (i32, i8),mut _2: (i32, i8),mut _3: i8,mut _4: i16,mut _5: (i16,),mut _6: i16,mut _7: i32) -> i8 {
mir! {
type RET = i8;
let _8: *const i128;
let _9: i32;
let _10: [u16; 7];
let _11: u32;
let _12: Adt47;
let _13: f64;
let _14: char;
let _15: ();
let _16: ();
{
_5.0 = _6;
_1.0 = !_2.0;
_2.0 = _1.0 + _7;
_4 = !_6;
RET = _3 << _2.0;
_5 = (_6,);
_4 = _6;
RET = _1.1;
Goto(bb1)
}
bb1 = {
_9 = 40759_u16 as i32;
_5.0 = !_6;
_1.0 = -_2.0;
RET = -_1.1;
_1 = (_2.0, RET);
RET = _2.1;
RET = _1.1;
Goto(bb2)
}
bb2 = {
_10 = [9207_u16,43876_u16,52269_u16,45406_u16,17376_u16,4430_u16,1202_u16];
_3 = RET;
RET = (-9223372036854775808_isize) as i8;
_10 = [59725_u16,6706_u16,50590_u16,2261_u16,23040_u16,64725_u16,54539_u16];
_7 = _2.0;
_11 = 3428993712_u32 - 2915825637_u32;
_5 = (_4,);
_4 = _6;
RET = -_3;
_2.1 = RET | RET;
_11 = 4084975076_u32 * 586948603_u32;
_5 = (_6,);
_9 = _1.0;
_9 = _6 as i32;
_4 = true as i16;
RET = _2.1 * _2.1;
_5 = (_6,);
_6 = _5.0;
_10 = [14890_u16,12647_u16,45614_u16,18041_u16,19224_u16,5533_u16,5530_u16];
Goto(bb3)
}
bb3 = {
Call(_15 = dump_var(8_usize, 2_usize, Move(_2), 11_usize, Move(_11), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_15 = dump_var(8_usize, 3_usize, Move(_3), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u16,mut _2: (i16,),mut _3: i32,mut _4: (i16,),mut _5: [bool; 8],mut _6: i16,mut _7: i32,mut _8: i16,mut _9: i8,mut _10: i16,mut _11: (i16,),mut _12: i8,mut _13: i8,mut _14: i16,mut _15: u16) -> u16 {
mir! {
type RET = u16;
let _16: isize;
let _17: (isize,);
let _18: Adt40;
let _19: bool;
let _20: u128;
let _21: u128;
let _22: usize;
let _23: f32;
let _24: Adt45;
let _25: i8;
let _26: *mut f32;
let _27: i8;
let _28: usize;
let _29: f32;
let _30: i64;
let _31: Adt45;
let _32: Adt50;
let _33: isize;
let _34: isize;
let _35: Adt47;
let _36: f32;
let _37: i64;
let _38: ();
let _39: ();
{
_7 = _3;
_4 = (_11.0,);
_4.0 = false as i16;
_6 = _14 ^ _10;
_8 = !_11.0;
RET = !_15;
_8 = 9207245554445858508_usize as i16;
_1 = 212_u8 as u16;
_4.0 = _11.0 + _2.0;
_4.0 = !_14;
_14 = _11.0 & _6;
_9 = 226128338007739690852731256864824342699_u128 as i8;
_4 = (_6,);
_16 = 62_isize << _14;
_17.0 = 5681653853648566284_i64 as isize;
Goto(bb1)
}
bb1 = {
RET = 1899296986_u32 as u16;
_6 = false as i16;
RET = _15;
_19 = false;
Goto(bb2)
}
bb2 = {
_2 = _4;
_14 = _4.0 << _2.0;
_16 = _17.0 ^ _17.0;
_20 = 6_usize as u128;
_13 = _9;
RET = _15 | _15;
_2 = (_14,);
_16 = _17.0 * _17.0;
_12 = _4.0 as i8;
_23 = _7 as f32;
_6 = _19 as i16;
_1 = _16 as u16;
_24.fld4 = 15906800726829455200_u64 | 12883472019955815999_u64;
_11.0 = -_2.0;
_10 = _16 as i16;
RET = !_1;
_11 = (_4.0,);
Call(_16 = fn10(_4.0, _14, _2.0, _11.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_24.fld5 = (_23,);
_10 = '\u{10dfdd}' as i16;
_23 = _15 as f32;
_1 = _12 as u16;
_17 = (_16,);
_2.0 = _14 & _4.0;
_11 = (_2.0,);
_5 = [_19,_19,_19,_19,_19,_19,_19,_19];
_24.fld4 = !5983874651081826162_u64;
_11.0 = _19 as i16;
_7 = _3;
_3 = _7 ^ _7;
_19 = false & false;
_10 = _2.0;
_2.0 = _14 & _14;
_24.fld5.0 = -_23;
_9 = _24.fld5.0 as i8;
_26 = core::ptr::addr_of_mut!(_24.fld5.0);
_13 = _9 + _9;
_15 = 106908456225227758970180176117232400296_i128 as u16;
_2.0 = 7495623673516664025_usize as i16;
_27 = _9 << _1;
_27 = !_12;
Goto(bb4)
}
bb4 = {
_21 = _20;
_1 = RET * RET;
_14 = (-49211009572957166899427899285639260582_i128) as i16;
_22 = !12905463052140315825_usize;
_24.fld4 = 11137738894665597459_u64;
_31.fld5.0 = _23;
_31.fld5.0 = 21_u8 as f32;
_24.fld4 = 6698341814186516410_u64;
_17.0 = !_16;
_31.fld5 = (_23,);
RET = !_1;
_32.fld0 = _10 | _10;
_15 = RET - _1;
_28 = _22;
_30 = -(-7991714303929974781_i64);
_11 = (_10,);
_12 = _3 as i8;
_15 = RET;
_9 = _27 << _11.0;
_2 = (_14,);
match _24.fld4 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6698341814186516410 => bb12,
_ => bb11
}
}
bb5 = {
_24.fld5 = (_23,);
_10 = '\u{10dfdd}' as i16;
_23 = _15 as f32;
_1 = _12 as u16;
_17 = (_16,);
_2.0 = _14 & _4.0;
_11 = (_2.0,);
_5 = [_19,_19,_19,_19,_19,_19,_19,_19];
_24.fld4 = !5983874651081826162_u64;
_11.0 = _19 as i16;
_7 = _3;
_3 = _7 ^ _7;
_19 = false & false;
_10 = _2.0;
_2.0 = _14 & _14;
_24.fld5.0 = -_23;
_9 = _24.fld5.0 as i8;
_26 = core::ptr::addr_of_mut!(_24.fld5.0);
_13 = _9 + _9;
_15 = 106908456225227758970180176117232400296_i128 as u16;
_2.0 = 7495623673516664025_usize as i16;
_27 = _9 << _1;
_27 = !_12;
Goto(bb4)
}
bb6 = {
_2 = _4;
_14 = _4.0 << _2.0;
_16 = _17.0 ^ _17.0;
_20 = 6_usize as u128;
_13 = _9;
RET = _15 | _15;
_2 = (_14,);
_16 = _17.0 * _17.0;
_12 = _4.0 as i8;
_23 = _7 as f32;
_6 = _19 as i16;
_1 = _16 as u16;
_24.fld4 = 15906800726829455200_u64 | 12883472019955815999_u64;
_11.0 = -_2.0;
_10 = _16 as i16;
RET = !_1;
_11 = (_4.0,);
Call(_16 = fn10(_4.0, _14, _2.0, _11.0), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = 1899296986_u32 as u16;
_6 = false as i16;
RET = _15;
_19 = false;
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
_24.fld4 = 7589942043049152706_u64 - 13212819566096130832_u64;
_19 = !true;
_31.fld4 = _32.fld0 as u64;
_13 = _27;
Goto(bb13)
}
bb13 = {
_27 = _9 & _12;
_6 = !_10;
_11.0 = _30 as i16;
_17.0 = _21 as isize;
_25 = -_13;
RET = _15;
_17 = (_16,);
_30 = (-6999776224065987301_i64);
_21 = _20;
_31.fld5.0 = _23;
_10 = _22 as i16;
(*_26) = -_31.fld5.0;
(*_26) = _23 + _31.fld5.0;
RET = _31.fld4 as u16;
_16 = _17.0 * _17.0;
_24.fld5.0 = -_31.fld5.0;
(*_26) = 3557288364_u32 as f32;
_25 = !_27;
RET = _15 ^ _15;
_22 = !_28;
_2 = _4;
_3 = _7;
_11.0 = _32.fld0;
_24.fld4 = 196_u8 as u64;
(*_26) = _23;
_22 = _31.fld4 as usize;
Goto(bb14)
}
bb14 = {
_22 = _20 as usize;
_32.fld0 = _11.0 - _11.0;
_27 = _9;
RET = _1;
_5 = [_19,_19,_19,_19,_19,_19,_19,_19];
_31.fld5.0 = (*_26) - (*_26);
_30 = 9058935709698846185_i64;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(9_usize, 15_usize, Move(_15), 8_usize, Move(_8), 19_usize, Move(_19), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(9_usize, 20_usize, Move(_20), 28_usize, Move(_28), 4_usize, Move(_4), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(9_usize, 30_usize, Move(_30), 10_usize, Move(_10), 21_usize, Move(_21), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16) -> isize {
mir! {
type RET = isize;
let _5: f32;
let _6: ((isize,), (i32, i8), usize);
let _7: [u16; 7];
let _8: f32;
let _9: bool;
let _10: i16;
let _11: u32;
let _12: (i16,);
let _13: f64;
let _14: usize;
let _15: (i32, i8);
let _16: isize;
let _17: u8;
let _18: [bool; 8];
let _19: u32;
let _20: (i16,);
let _21: [bool; 8];
let _22: f64;
let _23: f32;
let _24: [u16; 7];
let _25: f64;
let _26: (i16,);
let _27: isize;
let _28: ();
let _29: ();
{
_1 = -_3;
Goto(bb1)
}
bb1 = {
_1 = _2 - _4;
RET = 9223372036854775807_isize;
RET = (-65_isize);
_1 = -_2;
RET = 237125346386036121680195618379722502094_u128 as isize;
_6.0 = (RET,);
_5 = _1 as f32;
_6.0 = (RET,);
_6.2 = !12210021906261693687_usize;
_6.0.0 = false as isize;
_4 = '\u{65594}' as i16;
_6.0 = (RET,);
_6.0.0 = !RET;
_6.1.1 = (-82_i8) << _2;
_2 = _3 | _3;
_6.1.0 = '\u{10b37a}' as i32;
_8 = -_5;
_3 = _2 - _2;
_9 = true & false;
_8 = _5;
_6.2 = !6_usize;
_8 = _5;
_6.0 = (RET,);
_6.0.0 = RET + RET;
Call(_8 = fn11(_6.1.1, _3, _6, _3, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6.1.0 = !1446547947_i32;
_9 = _6.1.1 != _6.1.1;
_12.0 = 73_u8 as i16;
_9 = _3 >= _2;
_6.1.1 = 9_i8;
_8 = -_5;
_5 = _6.1.1 as f32;
_10 = 89_u8 as i16;
_2 = _3;
_7 = [29461_u16,12832_u16,8735_u16,27202_u16,2617_u16,63386_u16,51611_u16];
Goto(bb3)
}
bb3 = {
_5 = _8 + _8;
_6.1 = ((-1146673404_i32), 98_i8);
_13 = _6.1.0 as f64;
match _6.1.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
98 => bb9,
_ => bb8
}
}
bb4 = {
_6.1.0 = !1446547947_i32;
_9 = _6.1.1 != _6.1.1;
_12.0 = 73_u8 as i16;
_9 = _3 >= _2;
_6.1.1 = 9_i8;
_8 = -_5;
_5 = _6.1.1 as f32;
_10 = 89_u8 as i16;
_2 = _3;
_7 = [29461_u16,12832_u16,8735_u16,27202_u16,2617_u16,63386_u16,51611_u16];
Goto(bb3)
}
bb5 = {
_1 = _2 - _4;
RET = 9223372036854775807_isize;
RET = (-65_isize);
_1 = -_2;
RET = 237125346386036121680195618379722502094_u128 as isize;
_6.0 = (RET,);
_5 = _1 as f32;
_6.0 = (RET,);
_6.2 = !12210021906261693687_usize;
_6.0.0 = false as isize;
_4 = '\u{65594}' as i16;
_6.0 = (RET,);
_6.0.0 = !RET;
_6.1.1 = (-82_i8) << _2;
_2 = _3 | _3;
_6.1.0 = '\u{10b37a}' as i32;
_8 = -_5;
_3 = _2 - _2;
_9 = true & false;
_8 = _5;
_6.2 = !6_usize;
_8 = _5;
_6.0 = (RET,);
_6.0.0 = RET + RET;
Call(_8 = fn11(_6.1.1, _3, _6, _3, _1), ReturnTo(bb2), UnwindUnreachable())
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
_15.1 = _6.1.1;
_12.0 = _3 << _2;
_2 = !_1;
_11 = 561676767_u32 << _12.0;
_10 = !_12.0;
_16 = _9 as isize;
_6.2 = !6071988061634475901_usize;
_6.2 = !11937965690713096760_usize;
_6.1.1 = _15.1 ^ _15.1;
_1 = !_12.0;
_11 = !930372488_u32;
_16 = !_6.0.0;
_8 = -_5;
_10 = !_12.0;
_11 = 1041985195_u32 << _12.0;
Goto(bb10)
}
bb10 = {
_6.0 = (_16,);
RET = -_16;
_3 = _1;
_6.2 = 4_usize + 11008035714632096081_usize;
_13 = _6.1.1 as f64;
RET = -_16;
RET = _16 | _6.0.0;
_10 = _3 & _1;
_9 = !false;
_3 = _10;
_1 = -_3;
_20 = (_10,);
_1 = -_12.0;
_1 = 7681752212975105365_u64 as i16;
_9 = true;
_15 = _6.1;
_19 = _11 << _11;
_1 = _13 as i16;
Goto(bb11)
}
bb11 = {
_6.1 = (_15.0, _15.1);
_6.1.1 = !_15.1;
_2 = _10 + _20.0;
Goto(bb12)
}
bb12 = {
_21 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = !_10;
_6.1 = (_15.0, _15.1);
_19 = _11 | _11;
RET = -_6.0.0;
_15.1 = _6.1.1;
RET = -_16;
RET = _16;
_20.0 = _1;
_6.0 = (_16,);
_6.0.0 = _6.2 as isize;
_22 = _13 + _13;
RET = 187213154877157116328150499568198995039_u128 as isize;
_12 = _20;
_23 = _8 * _5;
_6.1 = _15;
_23 = _5 + _8;
_17 = _22 as u8;
_1 = _10;
match _15.0 {
0 => bb10,
1 => bb13,
2 => bb14,
340282366920938463463374607430621538052 => bb16,
_ => bb15
}
}
bb13 = {
_6.1.0 = !1446547947_i32;
_9 = _6.1.1 != _6.1.1;
_12.0 = 73_u8 as i16;
_9 = _3 >= _2;
_6.1.1 = 9_i8;
_8 = -_5;
_5 = _6.1.1 as f32;
_10 = 89_u8 as i16;
_2 = _3;
_7 = [29461_u16,12832_u16,8735_u16,27202_u16,2617_u16,63386_u16,51611_u16];
Goto(bb3)
}
bb14 = {
Return()
}
bb15 = {
_5 = _8 + _8;
_6.1 = ((-1146673404_i32), 98_i8);
_13 = _6.1.0 as f64;
match _6.1.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
98 => bb9,
_ => bb8
}
}
bb16 = {
_14 = _1 as usize;
_12.0 = _19 as i16;
_6.1.1 = _15.1 - _15.1;
_2 = _10 ^ _3;
_21 = [_9,_9,_9,_9,_9,_9,_9,_9];
_6.1.0 = _15.0;
_6.0 = (_16,);
_15.0 = !_6.1.0;
_25 = _22;
_20 = (_1,);
_16 = 38296_u16 as isize;
_15.1 = _6.1.1 ^ _6.1.1;
_6.2 = _6.1.0 as usize;
_13 = _6.0.0 as f64;
_13 = 315904522959924614510165195735061578777_u128 as f64;
RET = -_16;
_2 = _3 * _10;
_20 = _12;
_6.1.1 = _15.1 | _15.1;
_7 = [37204_u16,20982_u16,37963_u16,19885_u16,18863_u16,24969_u16,44086_u16];
_27 = RET & RET;
_16 = _19 as isize;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(10_usize, 3_usize, Move(_3), 12_usize, Move(_12), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(10_usize, 7_usize, Move(_7), 1_usize, Move(_1), 27_usize, Move(_27), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(10_usize, 17_usize, Move(_17), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i8,mut _2: i16,mut _3: ((isize,), (i32, i8), usize),mut _4: i16,mut _5: i16) -> f32 {
mir! {
type RET = f32;
let _6: isize;
let _7: i64;
let _8: bool;
let _9: (f32,);
let _10: u16;
let _11: f32;
let _12: i8;
let _13: (i32, i8);
let _14: f32;
let _15: [bool; 8];
let _16: u16;
let _17: [bool; 8];
let _18: (bool, [bool; 8], i8, u16, i16, i16);
let _19: *const i128;
let _20: f32;
let _21: usize;
let _22: Adt46;
let _23: (isize,);
let _24: Adt39;
let _25: (isize,);
let _26: f64;
let _27: f32;
let _28: isize;
let _29: Adt48;
let _30: isize;
let _31: [bool; 8];
let _32: i64;
let _33: f32;
let _34: bool;
let _35: ((isize,), (i32, i8), usize);
let _36: char;
let _37: (i16,);
let _38: [bool; 8];
let _39: [bool; 8];
let _40: ((isize,), (i32, i8), usize);
let _41: char;
let _42: i16;
let _43: (f32,);
let _44: char;
let _45: i32;
let _46: char;
let _47: [u16; 7];
let _48: i128;
let _49: isize;
let _50: isize;
let _51: (i16,);
let _52: (i32, i8);
let _53: isize;
let _54: isize;
let _55: ();
let _56: ();
{
_3.1.1 = _1 | _1;
_5 = 66409316311827488854933174762205453371_u128 as i16;
_3.2 = 12601000598717997795_usize >> _2;
_5 = -_4;
_3.0.0 = -9223372036854775807_isize;
RET = 25843689312975133678555582191373573297_u128 as f32;
_5 = _2 * _4;
_3.0.0 = 38_isize;
RET = _3.0.0 as f32;
_3.1 = ((-1294664291_i32), _1);
_3.0 = ((-9223372036854775808_isize),);
_6 = _3.0.0 | _3.0.0;
_3.0.0 = _6 & _6;
RET = (-119794050389280089400813008936267972057_i128) as f32;
_3.0.0 = _6;
_2 = _3.0.0 as i16;
_3.1 = ((-1680830463_i32), _1);
_3.2 = 5_usize;
_6 = 1307717650_u32 as isize;
_1 = !_3.1.1;
_3.1.0 = 666598248_i32 << _4;
_3.0.0 = _6 << _5;
Call(_3 = fn12(_6, _4, _4, _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.0.0 = _6 * _6;
RET = (-1818255356327416694_i64) as f32;
_3.0 = (_6,);
RET = _6 as f32;
RET = _3.1.0 as f32;
RET = 647947875_u32 as f32;
_6 = _3.0.0 | _3.0.0;
_6 = 874094657_u32 as isize;
_3.1.0 = (-758994489_i32);
_3.1.0 = 1545813339_i32 >> _4;
_5 = _4 * _4;
_1 = _3.1.1;
_3.0 = (_6,);
_3.1.1 = _1 - _1;
_1 = _3.1.1;
_6 = _3.0.0;
_1 = false as i8;
RET = (-2431419701816688482_i64) as f32;
_8 = !true;
Goto(bb2)
}
bb2 = {
_3.2 = 12832151948248979762_usize;
_2 = _5 << _3.1.0;
_5 = -_2;
_9 = (RET,);
_4 = _2 + _2;
_9.0 = RET;
RET = 1743932322473700652455613220946174867_i128 as f32;
_5 = -_4;
_5 = _4;
_6 = -_3.0.0;
_6 = _3.0.0;
RET = 3853895997_u32 as f32;
_9 = (RET,);
_5 = _2 >> _2;
_3.1 = ((-499510823_i32), _1);
Call(_4 = fn16(_5, _2, _3.1.0, _3.0, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3.0.0 = _6;
_2 = _3.1.0 as i16;
_1 = _3.1.1;
_3.1.1 = _1;
_5 = !_2;
_5 = 22219_u16 as i16;
_3.0 = (_6,);
_2 = 27_u8 as i16;
_3.0.0 = _6;
_3.0.0 = 4467_u16 as isize;
_11 = _3.1.0 as f32;
_13 = (_3.1.0, _3.1.1);
_3.1 = _13;
_10 = 38926_u16 >> _3.1.1;
_6 = _3.0.0;
_13.1 = _3.1.1 & _3.1.1;
_13 = (_3.1.0, _3.1.1);
Call(_3.2 = core::intrinsics::bswap(2390936533825500787_usize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14 = RET;
RET = _14 + _11;
_14 = RET;
_15 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3.2 = 4_usize;
_3.0.0 = -_6;
_3.2 = 10035060883554470128_usize & 0_usize;
_3.1.0 = _13.0 - _13.0;
_1 = _3.1.1 - _13.1;
_7 = 960512995652212436_i64 * (-1578432972382622938_i64);
_9 = (RET,);
_9.0 = RET * _14;
_16 = !_10;
_13.0 = _3.1.0 | _3.1.0;
_6 = _3.0.0 << _13.0;
_4 = _2 + _5;
_16 = _10 ^ _10;
Goto(bb5)
}
bb5 = {
_3.0.0 = _6;
_18.3 = _16 + _16;
_9 = (RET,);
_9.0 = RET;
_15 = [_8,_8,_8,_8,_8,_8,_8,_8];
_17 = [_8,_8,_8,_8,_8,_8,_8,_8];
_18.2 = !_1;
_6 = _8 as isize;
_18.1 = [_8,_8,_8,_8,_8,_8,_8,_8];
_18 = (_8, _17, _13.1, _10, _5, _2);
_3.0.0 = _6;
_18 = (_8, _15, _3.1.1, _10, _4, _4);
_18.3 = !_16;
_20 = RET;
RET = _10 as f32;
_7 = !8577330774704409011_i64;
_12 = _1;
_3.0.0 = _6;
_17 = [_18.0,_18.0,_18.0,_18.0,_8,_8,_18.0,_8];
_22 = Adt46::Variant0 { fld0: _18.3 };
_17 = [_18.0,_18.0,_8,_18.0,_8,_18.0,_18.0,_18.0];
_17 = [_8,_18.0,_8,_8,_8,_18.0,_18.0,_8];
_23.0 = _3.0.0;
Goto(bb6)
}
bb6 = {
_20 = RET;
_4 = -_5;
RET = -_14;
_14 = RET;
_17 = _18.1;
_3 = (_23, _13, 6_usize);
_3.1 = (_13.0, _1);
_18.4 = _2;
_4 = _18.5 - _5;
_4 = _8 as i16;
_18.0 = _8;
_3.0 = (_6,);
_3.0 = (_23.0,);
_20 = _9.0 - _9.0;
_13.0 = !_3.1.0;
Goto(bb7)
}
bb7 = {
_13.1 = _12 * _3.1.1;
_28 = _3.0.0 ^ _6;
SetDiscriminant(_22, 0);
_3.1.1 = !_13.1;
_3.2 = !1078072328154898395_usize;
_18.0 = _9.0 < _20;
_30 = -_3.0.0;
_35 = (_3.0, _3.1, _3.2);
_18.3 = _10 | _16;
_25.0 = _6 - _3.0.0;
_35.0.0 = _35.1.0 as isize;
_31 = [_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0];
_33 = _20 + _14;
_35.2 = _3.2;
_33 = _14;
_18.1 = _31;
_27 = -_9.0;
_1 = _13.1 - _3.1.1;
_37.0 = -_18.5;
_9 = (_33,);
_35.1.0 = _3.1.0 ^ _13.0;
_34 = _10 != _16;
_2 = _5;
_13 = (_35.1.0, _1);
Call(_16 = fn18(_3, _13), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_38 = [_34,_18.0,_8,_18.0,_18.0,_18.0,_18.0,_18.0];
_35.1.0 = _13.0 << _13.0;
_23.0 = !_30;
RET = _33;
_35.2 = _3.2 ^ _3.2;
_13 = (_3.1.0, _18.2);
_18.4 = _37.0 << _37.0;
_3.1 = _35.1;
_26 = 142_u8 as f64;
_35.0 = _25;
_2 = !_18.5;
_40.1.0 = _13.0;
_40 = (_3.0, _3.1, _35.2);
RET = -_14;
place!(Field::<u16>(Variant(_22, 0), 0)) = _16;
_3 = (_23, _13, _40.2);
_9.0 = 3485753390_u32 as f32;
_18.4 = _37.0 * _18.5;
_40.2 = 230_u8 as usize;
_37 = (_18.4,);
_37 = (_5,);
_13.0 = _35.1.0 & _3.1.0;
_33 = -_20;
_13.0 = -_35.1.0;
_34 = !_18.0;
Goto(bb9)
}
bb9 = {
_23 = (_6,);
_44 = '\u{b1c33}';
_40.2 = _35.2;
_3.1.1 = _1;
_13.1 = _1 * _3.1.1;
_4 = _5 * _18.4;
_17 = [_18.0,_34,_18.0,_34,_34,_18.0,_34,_18.0];
_7 = _44 as i64;
_23 = (_35.0.0,);
_3.1 = (_40.1.0, _1);
_35 = (_23, _3.1, _40.2);
RET = _20;
Goto(bb10)
}
bb10 = {
_41 = _44;
_13 = _3.1;
_13.0 = _3.1.0 & _40.1.0;
_1 = _35.1.1 * _13.1;
_35.1.1 = _1;
place!(Field::<u16>(Variant(_22, 0), 0)) = !_18.3;
_9.0 = Field::<u16>(Variant(_22, 0), 0) as f32;
Goto(bb11)
}
bb11 = {
_6 = _35.0.0 * _35.0.0;
_45 = _35.1.0;
_17 = _18.1;
Goto(bb12)
}
bb12 = {
RET = _9.0;
_18.5 = _2 << Field::<u16>(Variant(_22, 0), 0);
_18.0 = _45 != _3.1.0;
_7 = !7682065002887884757_i64;
RET = -_14;
_3.1.1 = _35.1.1 << _40.1.0;
_33 = 9564830347713848603_u64 as f32;
_47 = [Field::<u16>(Variant(_22, 0), 0),_10,Field::<u16>(Variant(_22, 0), 0),Field::<u16>(Variant(_22, 0), 0),Field::<u16>(Variant(_22, 0), 0),_10,Field::<u16>(Variant(_22, 0), 0)];
Goto(bb13)
}
bb13 = {
_3.1.1 = _35.1.1;
_35.1.1 = _1 & _13.1;
_40.1 = (_35.1.0, _35.1.1);
_15 = [_34,_18.0,_18.0,_18.0,_18.0,_18.0,_18.0,_34];
_12 = !_1;
_48 = !84560942226318376012632676073747686041_i128;
_36 = _41;
_26 = _13.0 as f64;
_42 = -_4;
_11 = _20 - _9.0;
_3.2 = _40.2;
_33 = -_11;
_47 = [_10,Field::<u16>(Variant(_22, 0), 0),_18.3,Field::<u16>(Variant(_22, 0), 0),_10,_18.3,_18.3];
_40.0 = (_28,);
_35.0.0 = -_25.0;
_41 = _36;
_23.0 = _16 as isize;
_35.0 = (_6,);
_40.0 = (_6,);
_35 = (_23, _40.1, _3.2);
Goto(bb14)
}
bb14 = {
_35 = _3;
RET = -_33;
_49 = _6;
_10 = _18.3 << _12;
_46 = _41;
_7 = 529188881175583252_i64;
_28 = _40.2 as isize;
_44 = _41;
_28 = _35.0.0 | _6;
_3.0 = _23;
Goto(bb15)
}
bb15 = {
Call(_55 = dump_var(11_usize, 34_usize, Move(_34), 8_usize, Move(_8), 4_usize, Move(_4), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_55 = dump_var(11_usize, 30_usize, Move(_30), 37_usize, Move(_37), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_55 = dump_var(11_usize, 47_usize, Move(_47), 36_usize, Move(_36), 13_usize, Move(_13), 49_usize, Move(_49)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_55 = dump_var(11_usize, 2_usize, Move(_2), 38_usize, Move(_38), 28_usize, Move(_28), 42_usize, Move(_42)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_55 = dump_var(11_usize, 6_usize, Move(_6), 56_usize, _56, 56_usize, _56, 56_usize, _56), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16) -> ((isize,), (i32, i8), usize) {
mir! {
type RET = ((isize,), (i32, i8), usize);
let _6: (i32, i8);
let _7: char;
let _8: i128;
let _9: i8;
let _10: f64;
let _11: (i16,);
let _12: u128;
let _13: i64;
let _14: (i16,);
let _15: f64;
let _16: (i16,);
let _17: usize;
let _18: [u16; 7];
let _19: (i16,);
let _20: isize;
let _21: [u16; 7];
let _22: ();
let _23: ();
{
RET.0.0 = -_1;
_2 = _1 as i16;
RET.1.0 = (-1525519838_i32);
_4 = -_5;
RET.0.0 = _1;
_1 = 5938962280435149871_i64 as isize;
_4 = '\u{c564}' as i16;
RET.2 = RET.1.0 as usize;
RET.1 = ((-1084326374_i32), 40_i8);
_5 = _3 >> _3;
_1 = -RET.0.0;
_2 = 47363_u16 as i16;
RET.1.1 = (-18_i8) | 79_i8;
_5 = 4228979672_u32 as i16;
RET.2 = 0_usize;
_6.1 = 48341479806468211526725764276203961882_u128 as i8;
Call(_6.0 = fn13(RET.0.0, _3, _3, _3, _3, _3, _3, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = RET.1.0 - RET.1.0;
_6.1 = 6144625039697451053_i64 as i8;
RET.1.1 = _6.1;
RET.1 = (_6.0, _6.1);
RET.1 = (_6.0, _6.1);
RET.1 = (_6.0, _6.1);
RET.0.0 = _1 << _3;
_3 = _2;
_7 = '\u{43d6b}';
RET.2 = 35199855017125106383672257712471709811_i128 as usize;
_4 = _5;
RET.0 = (_1,);
_4 = true as i16;
RET.0.0 = !_1;
RET.1.1 = -_6.1;
RET.1.1 = _6.1 - _6.1;
_3 = _2 + _5;
RET.1.1 = false as i8;
_8 = (-1097982869101920789154518515628940645_i128);
_6.1 = RET.1.1;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
339184384051836542674220088916139270811 => bb9,
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
RET.2 = 0_usize;
_8 = 48207_u16 as i128;
RET.0 = (_1,);
_4 = !_5;
RET.0.0 = _1 - _1;
RET.2 = 3_usize + 16664196203507449452_usize;
RET.1.0 = 49957_u16 as i32;
RET.1.1 = _6.1;
_6.1 = !RET.1.1;
RET.1.1 = !_6.1;
_1 = RET.0.0;
_9 = RET.1.1;
RET.1 = _6;
_9 = _6.0 as i8;
_11 = (_3,);
_4 = _5 + _3;
_11 = (_3,);
_1 = -RET.0.0;
Goto(bb10)
}
bb10 = {
RET.0.0 = _1 * _1;
_13 = !4982378707664597235_i64;
_7 = '\u{a1731}';
_6.0 = 64348152102185445316307065610482043355_u128 as i32;
Goto(bb11)
}
bb11 = {
_1 = 196302840664309422045304410685416771669_u128 as isize;
RET.0.0 = 3046044835_u32 as isize;
_14 = (_4,);
RET.1.1 = _9 & _9;
RET.1.0 = -_6.0;
RET.1.0 = _6.0 * _6.0;
RET.0 = (_1,);
_11.0 = _8 as i16;
_8 = !1270271718670074773635434972100818122_i128;
_6.0 = RET.1.0 << _9;
_4 = _3 | _3;
RET.1.1 = _6.1;
_6 = (RET.1.0, _9);
_15 = 2897215914_u32 as f64;
RET.1.1 = _15 as i8;
_16.0 = _3;
Goto(bb12)
}
bb12 = {
_1 = RET.0.0 + RET.0.0;
_2 = _4;
RET.1 = (_6.0, _9);
_14 = (_11.0,);
_15 = 221_u8 as f64;
Goto(bb13)
}
bb13 = {
_12 = _7 as u128;
_15 = 657288400_u32 as f64;
_10 = _15;
_14.0 = -_5;
_1 = RET.0.0;
_13 = 759571172582928341_i64 * (-2972280219166928914_i64);
_6.0 = !RET.1.0;
_4 = !_3;
_14.0 = RET.0.0 as i16;
_2 = _5;
RET.0 = (_1,);
_16 = (_14.0,);
RET.0 = (_1,);
_11.0 = _16.0;
_11 = _16;
Goto(bb14)
}
bb14 = {
_2 = 43022_u16 as i16;
_12 = !41874046573723799388376651223789538027_u128;
RET.0 = (_1,);
_2 = 625163650_u32 as i16;
_19.0 = 47903_u16 as i16;
RET.0.0 = -_1;
RET.1.1 = _13 as i8;
_13 = !(-553184837990450143_i64);
_15 = -_10;
RET.1.0 = RET.2 as i32;
RET.0.0 = _1;
RET.0 = (_1,);
RET.1 = (_6.0, _9);
_7 = '\u{87017}';
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(12_usize, 4_usize, Move(_4), 1_usize, Move(_1), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(12_usize, 6_usize, Move(_6), 14_usize, Move(_14), 8_usize, Move(_8), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: i16,mut _8: ((isize,), (i32, i8), usize)) -> i32 {
mir! {
type RET = i32;
let _9: (i16,);
let _10: isize;
let _11: i128;
let _12: f32;
let _13: Adt39;
let _14: Adt42;
let _15: u8;
let _16: u64;
let _17: isize;
let _18: [u16; 7];
let _19: u32;
let _20: u64;
let _21: char;
let _22: (bool, [bool; 8], i8, u16, i16, i16);
let _23: bool;
let _24: Adt49;
let _25: Adt51;
let _26: ();
let _27: ();
{
_3 = _5 ^ _6;
_8.1.0 = _2 as i32;
_6 = _3;
_8.1 = ((-238251525_i32), (-41_i8));
_8.1.0 = !(-1381010571_i32);
RET = _8.2 as i32;
_9.0 = _5 ^ _2;
_10 = _1;
_5 = _6;
_8.2 = 16264517943370715339_usize * 2_usize;
_10 = _8.1.1 as isize;
RET = !_8.1.0;
_8.0.0 = _10;
_11 = (-8208698656605529967_i64) as i128;
RET = _8.1.0;
_8.1 = (RET, (-127_i8));
RET = _8.1.0;
_8.2 = 7571283793280510735_u64 as usize;
Call(_9.0 = fn14(_7, _6, _5, _3, _2, _7, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = -_8.1.0;
Goto(bb2)
}
bb2 = {
_8.1.0 = RET * RET;
_8.0 = (_1,);
_8.1 = (RET, (-109_i8));
_8.2 = 155403929198084665384738274365416417692_u128 as usize;
_2 = _4 - _5;
_8.0.0 = -_10;
_3 = _2 & _9.0;
_8.2 = _8.1.0 as usize;
_8.1.1 = (-67_i8);
_2 = _10 as i16;
_4 = -_5;
_8.1.0 = 7692876164144299603_i64 as i32;
_10 = _8.0.0;
RET = _8.1.0 << _6;
_8.0 = (_10,);
_10 = _8.0.0 & _8.0.0;
_5 = _7 << _7;
_8.1.1 = 55_i8;
RET = -_8.1.0;
_8.2 = 2243653455382112742_usize >> _6;
_3 = _5;
_11 = !(-106454055819260408863165855057501665908_i128);
_2 = _9.0 - _9.0;
_12 = _9.0 as f32;
_12 = _8.1.1 as f32;
_9 = (_4,);
Goto(bb3)
}
bb3 = {
_12 = _11 as f32;
_9 = (_5,);
_3 = _11 as i16;
RET = _8.1.0 + _8.1.0;
_8.1.1 = _11 as i8;
_8.0 = (_10,);
RET = '\u{1097c4}' as i32;
_9.0 = _7;
_8.0 = (_10,);
_8.0 = (_10,);
_2 = _12 as i16;
_6 = !_4;
_9 = (_4,);
RET = _8.1.0;
_2 = _8.0.0 as i16;
_10 = _8.0.0 + _8.0.0;
_8.0 = (_10,);
_11 = 125997632516469760859798557077440371495_i128 - 30856746960295331165251164969124311597_i128;
_8.1.0 = RET;
_11 = (-95533512667243449293506293698666469029_i128);
_12 = _8.2 as f32;
_1 = -_8.0.0;
_15 = 86_u8 << _7;
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
244748854253695014169868313733101742427 => bb6,
_ => bb5
}
}
bb4 = {
_8.1.0 = RET * RET;
_8.0 = (_1,);
_8.1 = (RET, (-109_i8));
_8.2 = 155403929198084665384738274365416417692_u128 as usize;
_2 = _4 - _5;
_8.0.0 = -_10;
_3 = _2 & _9.0;
_8.2 = _8.1.0 as usize;
_8.1.1 = (-67_i8);
_2 = _10 as i16;
_4 = -_5;
_8.1.0 = 7692876164144299603_i64 as i32;
_10 = _8.0.0;
RET = _8.1.0 << _6;
_8.0 = (_10,);
_10 = _8.0.0 & _8.0.0;
_5 = _7 << _7;
_8.1.1 = 55_i8;
RET = -_8.1.0;
_8.2 = 2243653455382112742_usize >> _6;
_3 = _5;
_11 = !(-106454055819260408863165855057501665908_i128);
_2 = _9.0 - _9.0;
_12 = _9.0 as f32;
_12 = _8.1.1 as f32;
_9 = (_4,);
Goto(bb3)
}
bb5 = {
RET = -_8.1.0;
Goto(bb2)
}
bb6 = {
_5 = _7;
Goto(bb7)
}
bb7 = {
_16 = 204328035685206204609365919714925480672_u128 as u64;
_6 = _8.1.1 as i16;
_12 = 6208955048262580622_i64 as f32;
_8.1.1 = _11 as i8;
_8.2 = _11 as usize;
_9 = (_7,);
_8.0 = (_10,);
_12 = _5 as f32;
_3 = !_7;
_8.0 = (_10,);
_7 = !_4;
_4 = _3;
_8.0.0 = _8.1.1 as isize;
_8.1.1 = _16 as i8;
_2 = _4;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
244748854253695014169868313733101742427 => bb11,
_ => bb10
}
}
bb8 = {
_12 = _11 as f32;
_9 = (_5,);
_3 = _11 as i16;
RET = _8.1.0 + _8.1.0;
_8.1.1 = _11 as i8;
_8.0 = (_10,);
RET = '\u{1097c4}' as i32;
_9.0 = _7;
_8.0 = (_10,);
_8.0 = (_10,);
_2 = _12 as i16;
_6 = !_4;
_9 = (_4,);
RET = _8.1.0;
_2 = _8.0.0 as i16;
_10 = _8.0.0 + _8.0.0;
_8.0 = (_10,);
_11 = 125997632516469760859798557077440371495_i128 - 30856746960295331165251164969124311597_i128;
_8.1.0 = RET;
_11 = (-95533512667243449293506293698666469029_i128);
_12 = _8.2 as f32;
_1 = -_8.0.0;
_15 = 86_u8 << _7;
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
244748854253695014169868313733101742427 => bb6,
_ => bb5
}
}
bb9 = {
RET = -_8.1.0;
Goto(bb2)
}
bb10 = {
_8.1.0 = RET * RET;
_8.0 = (_1,);
_8.1 = (RET, (-109_i8));
_8.2 = 155403929198084665384738274365416417692_u128 as usize;
_2 = _4 - _5;
_8.0.0 = -_10;
_3 = _2 & _9.0;
_8.2 = _8.1.0 as usize;
_8.1.1 = (-67_i8);
_2 = _10 as i16;
_4 = -_5;
_8.1.0 = 7692876164144299603_i64 as i32;
_10 = _8.0.0;
RET = _8.1.0 << _6;
_8.0 = (_10,);
_10 = _8.0.0 & _8.0.0;
_5 = _7 << _7;
_8.1.1 = 55_i8;
RET = -_8.1.0;
_8.2 = 2243653455382112742_usize >> _6;
_3 = _5;
_11 = !(-106454055819260408863165855057501665908_i128);
_2 = _9.0 - _9.0;
_12 = _9.0 as f32;
_12 = _8.1.1 as f32;
_9 = (_4,);
Goto(bb3)
}
bb11 = {
_8.1 = (RET, 28_i8);
_4 = _8.1.0 as i16;
_15 = _12 as u8;
_8.1 = (RET, 113_i8);
_2 = _5;
_4 = _2;
_18 = [8671_u16,5741_u16,42462_u16,23724_u16,8181_u16,34279_u16,11826_u16];
_8.0 = (_1,);
_4 = -_5;
_20 = (-864607293957153252_i64) as u64;
_20 = _16;
_1 = _8.0.0 << _5;
_9 = (_7,);
_20 = _16 | _16;
_11 = _15 as i128;
_15 = !4_u8;
_2 = _20 as i16;
_16 = _20 >> _1;
Call(_8.2 = fn15(_4, _4, _9.0, _16, _7, _12, _9.0, _8.1, _7), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22.1 = [false,true,false,true,false,true,true,false];
_20 = _16;
_15 = 71_u8;
_9 = (_7,);
_8.0 = (_1,);
_8.1 = (RET, (-112_i8));
_11 = 63415853197997355618855996018588226421_u128 as i128;
_8.0 = (_1,);
Call(_5 = core::intrinsics::bswap(_4), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_22.1 = [false,false,false,true,false,true,false,true];
_15 = 3363761608_u32 as u8;
match _8.1.1 {
0 => bb10,
1 => bb2,
2 => bb14,
340282366920938463463374607431768211344 => bb16,
_ => bb15
}
}
bb14 = {
_5 = _7;
Goto(bb7)
}
bb15 = {
RET = -_8.1.0;
Goto(bb2)
}
bb16 = {
RET = _8.1.0 | _8.1.0;
_2 = _9.0 * _3;
_1 = _8.1.1 as isize;
_22.5 = _3;
_22.3 = 57948_u16 << _7;
_24.fld0 = false;
_22.3 = 28573_u16;
_19 = _8.2 as u32;
_10 = 62824101161765749520769592790795813486_u128 as isize;
_24.fld6.fld4 = _20 * _16;
_16 = _24.fld0 as u64;
_24.fld1 = [_22.3,_22.3,_22.3,_22.3,_22.3,_22.3,_22.3];
_24.fld6.fld5.0 = _22.3 as f32;
_8.1.1 = (-28_i8) * 68_i8;
_24.fld6.fld0 = core::ptr::addr_of!(_11);
_18 = _24.fld1;
_25.fld0.3 = _22.3 >> _5;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(13_usize, 2_usize, Move(_2), 7_usize, Move(_7), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(13_usize, 4_usize, Move(_4), 18_usize, Move(_18), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: i16,mut _7: i16) -> i16 {
mir! {
type RET = i16;
let _8: *const i128;
let _9: (i16,);
let _10: u32;
let _11: ();
let _12: ();
{
_3 = _4;
_2 = _1 - _7;
RET = (-54784186919626688676953839626166414725_i128) as i16;
_1 = _5;
_2 = 94925900091214011477218558333398051679_i128 as i16;
_6 = _3;
_3 = _7 << _7;
_1 = _7;
_4 = 201708966899555159744469627391657821085_u128 as i16;
_5 = _1;
RET = 58111_u16 as i16;
_7 = !_3;
RET = 130_u8 as i16;
_6 = _3 | _3;
RET = _6 - _7;
_1 = (-1040915767_i32) as i16;
_6 = '\u{2f65f}' as i16;
RET = !_7;
RET = -_7;
_5 = !_7;
_9.0 = _7;
_4 = _5 + _5;
_6 = (-1280471480_i32) as i16;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(14_usize, 1_usize, Move(_1), 4_usize, Move(_4), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: u64,mut _5: i16,mut _6: f32,mut _7: i16,mut _8: (i32, i8),mut _9: i16) -> usize {
mir! {
type RET = usize;
let _10: (i32, i8);
let _11: f64;
let _12: f64;
let _13: usize;
let _14: Adt36;
let _15: ((isize,), (i32, i8), usize);
let _16: Adt52;
let _17: *mut f32;
let _18: Adt49;
let _19: i16;
let _20: (f32,);
let _21: [bool; 8];
let _22: [u16; 7];
let _23: char;
let _24: (i16,);
let _25: (i32, i8);
let _26: [bool; 8];
let _27: f64;
let _28: isize;
let _29: (isize,);
let _30: Adt52;
let _31: bool;
let _32: (i16,);
let _33: f32;
let _34: Adt49;
let _35: Adt38;
let _36: (i16,);
let _37: ();
let _38: ();
{
RET = (-8783960924797641913_i64) as usize;
_7 = -_3;
RET = !2_usize;
_3 = (-1831478367346367015_i64) as i16;
_4 = 16700948538435171516_u64;
_11 = _8.0 as f64;
_8.0 = 6619401_i32;
_4 = 6329297435833275563_u64;
_6 = 3892939271_u32 as f32;
RET = 3_usize >> _1;
RET = !6_usize;
_11 = (-588398286505580936_i64) as f64;
_10.1 = -_8.1;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_12 = _11 - _11;
_15.0.0 = (-9223372036854775808_isize);
_4 = 8835042400680037860_u64 ^ 10824549273462706090_u64;
_15.2 = !RET;
_15.1.1 = _8.1;
_1 = _2 ^ _5;
_7 = -_5;
_15.0 = (9223372036854775807_isize,);
_5 = !_1;
_15.1 = _8;
_15.1.0 = _8.0 << _7;
_10.0 = _15.1.0;
_5 = _7 | _2;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld3);
_3 = _2 >> _5;
_11 = _9 as f64;
_4 = 7787233481988133372_u64;
_13 = RET;
_18.fld6.fld4 = _15.0.0 as u64;
_16.fld1.5 = _5;
match _15.0.0 {
9223372036854775807 => bb4,
_ => bb3
}
}
bb3 = {
Goto(bb2)
}
bb4 = {
_16.fld1.4 = _5 * _16.fld1.5;
_16.fld1.5 = _1;
_18.fld1 = [24181_u16,61018_u16,52280_u16,22592_u16,38257_u16,23848_u16,49303_u16];
_10.1 = -_8.1;
_16.fld1.5 = !_2;
_12 = _11;
_6 = 3077264302_u32 as f32;
_16.fld1.1 = [true,true,false,true,true,false,false,false];
_16.fld2 = core::ptr::addr_of_mut!(_18.fld6.fld0);
_16.fld0.fld1 = Adt38::Variant0 { fld0: 148691683511473542141614347345597267880_i128,fld1: '\u{ad05f}',fld2: _18.fld1 };
_15.1.1 = _8.1 | _8.1;
_16.fld3 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_16.fld0.fld1, 0), 0)));
_16.fld0.fld0 = -_1;
_8.0 = !_15.1.0;
_5 = -_1;
_16.fld0.fld1 = Adt38::Variant0 { fld0: (-154987145684964659968532604419942989355_i128),fld1: '\u{f7114}',fld2: _18.fld1 };
_18.fld4 = !_18.fld6.fld4;
_9 = _5 ^ _3;
Goto(bb5)
}
bb5 = {
_16.fld0.fld1 = Adt38::Variant0 { fld0: (-13737382424225097209269813725584354838_i128),fld1: '\u{55346}',fld2: _18.fld1 };
_18.fld6.fld4 = !_4;
_20 = (_6,);
_7 = _3 * _2;
place!(Field::<i128>(Variant(_16.fld0.fld1, 0), 0)) = -59612862692173020673525241041481880208_i128;
_2 = _7 << _5;
_15.0 = (32_isize,);
_16.fld1.0 = !false;
_9 = Field::<i128>(Variant(_16.fld0.fld1, 0), 0) as i16;
_18.fld6.fld5 = (_6,);
_20 = _18.fld6.fld5;
match _8.1 {
0 => bb4,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
113 => bb10,
_ => bb9
}
}
bb6 = {
_16.fld1.4 = _5 * _16.fld1.5;
_16.fld1.5 = _1;
_18.fld1 = [24181_u16,61018_u16,52280_u16,22592_u16,38257_u16,23848_u16,49303_u16];
_10.1 = -_8.1;
_16.fld1.5 = !_2;
_12 = _11;
_6 = 3077264302_u32 as f32;
_16.fld1.1 = [true,true,false,true,true,false,false,false];
_16.fld2 = core::ptr::addr_of_mut!(_18.fld6.fld0);
_16.fld0.fld1 = Adt38::Variant0 { fld0: 148691683511473542141614347345597267880_i128,fld1: '\u{ad05f}',fld2: _18.fld1 };
_15.1.1 = _8.1 | _8.1;
_16.fld3 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_16.fld0.fld1, 0), 0)));
_16.fld0.fld0 = -_1;
_8.0 = !_15.1.0;
_5 = -_1;
_16.fld0.fld1 = Adt38::Variant0 { fld0: (-154987145684964659968532604419942989355_i128),fld1: '\u{f7114}',fld2: _18.fld1 };
_18.fld4 = !_18.fld6.fld4;
_9 = _5 ^ _3;
Goto(bb5)
}
bb7 = {
Goto(bb2)
}
bb8 = {
_12 = _11 - _11;
_15.0.0 = (-9223372036854775808_isize);
_4 = 8835042400680037860_u64 ^ 10824549273462706090_u64;
_15.2 = !RET;
_15.1.1 = _8.1;
_1 = _2 ^ _5;
_7 = -_5;
_15.0 = (9223372036854775807_isize,);
_5 = !_1;
_15.1 = _8;
_15.1.0 = _8.0 << _7;
_10.0 = _15.1.0;
_5 = _7 | _2;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld3);
_3 = _2 >> _5;
_11 = _9 as f64;
_4 = 7787233481988133372_u64;
_13 = RET;
_18.fld6.fld4 = _15.0.0 as u64;
_16.fld1.5 = _5;
match _15.0.0 {
9223372036854775807 => bb4,
_ => bb3
}
}
bb9 = {
Goto(bb2)
}
bb10 = {
_16.fld0.fld1 = Adt38::Variant0 { fld0: (-53201292736649078009321232262731725239_i128),fld1: '\u{c4595}',fld2: _18.fld1 };
_16.fld2 = core::ptr::addr_of_mut!(_16.fld3);
_20 = (_18.fld6.fld5.0,);
place!(Field::<char>(Variant(_16.fld0.fld1, 0), 1)) = '\u{e6544}';
_21 = _16.fld1.1;
_24 = (_16.fld1.4,);
Goto(bb11)
}
bb11 = {
_14 = Adt36::Variant2 { fld0: 158_u8,fld1: _6 };
_18.fld4 = _4;
_23 = Field::<char>(Variant(_16.fld0.fld1, 0), 1);
_18.fld6.fld5.0 = _7 as f32;
RET = _15.2 << _16.fld0.fld0;
_22 = [10798_u16,49387_u16,1044_u16,54854_u16,49992_u16,16923_u16,59463_u16];
_18.fld6.fld0 = _16.fld3;
_9 = _24.0 * _3;
_16.fld3 = _18.fld6.fld0;
_15.0 = ((-61_isize),);
_16.fld1 = (false, _21, _15.1.1, 60777_u16, _16.fld0.fld0, _7);
_25 = _8;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld3);
_26 = _21;
_7 = _5 - _5;
RET = _25.1 as usize;
_25.0 = -_15.1.0;
_18.fld6.fld4 = !_4;
_18.fld0 = !_16.fld1.0;
_26 = _16.fld1.1;
_18.fld2 = Adt46::Variant0 { fld0: _16.fld1.3 };
_13 = _15.2 - RET;
place!(Field::<u16>(Variant(_18.fld2, 0), 0)) = !_16.fld1.3;
_16.fld3 = _18.fld6.fld0;
RET = _15.2;
_28 = _15.0.0;
_25 = (_8.0, _15.1.1);
Goto(bb12)
}
bb12 = {
_18.fld6.fld1 = Adt38::Variant0 { fld0: (-114099229319024499231029381912462408246_i128),fld1: Field::<char>(Variant(_16.fld0.fld1, 0), 1),fld2: _22 };
_1 = !_24.0;
_4 = _18.fld6.fld4 | _18.fld4;
_28 = !_15.0.0;
_20.0 = _18.fld6.fld5.0 - _18.fld6.fld5.0;
_19 = _16.fld1.4 >> Field::<u16>(Variant(_18.fld2, 0), 0);
_30.fld1.1 = [_16.fld1.0,_16.fld1.0,_18.fld0,_16.fld1.0,_16.fld1.0,_18.fld0,_18.fld0,_18.fld0];
place!(Field::<f32>(Variant(_14, 2), 1)) = _20.0;
_7 = _24.0 + _16.fld1.4;
_16.fld0.fld2 = Adt42::Variant0 { fld0: _24,fld1: _3,fld2: (-2086143113011023436_i64),fld3: _15 };
_16.fld1.5 = _5;
place!(Field::<u8>(Variant(_14, 2), 0)) = 199_u8 - 9_u8;
place!(Field::<f32>(Variant(_14, 2), 1)) = -_20.0;
_30.fld1.1 = [_18.fld0,_18.fld0,_16.fld1.0,_18.fld0,_18.fld0,_16.fld1.0,_18.fld0,_16.fld1.0];
place!(Field::<i128>(Variant(_18.fld6.fld1, 0), 0)) = -(-111239440212910677899023344941043495274_i128);
place!(Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3)).1.1 = _8.1 ^ _16.fld1.2;
_8 = _15.1;
_30.fld1.0 = !_16.fld1.0;
_16.fld2 = core::ptr::addr_of_mut!(_16.fld3);
place!(Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3)).1 = _25;
place!(Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3)) = (_15.0, _25, RET);
_30.fld1.4 = Field::<(i16,)>(Variant(_16.fld0.fld2, 0), 0).0 - _9;
_17 = core::ptr::addr_of_mut!(_6);
_28 = Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3).0.0 | Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3).0.0;
_30.fld1 = (_16.fld1.0, _26, _15.1.1, Field::<u16>(Variant(_18.fld2, 0), 0), _16.fld1.5, _3);
Goto(bb13)
}
bb13 = {
_31 = _16.fld1.0;
_17 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_14, 2), 1)));
_30.fld0.fld2 = Adt42::Variant0 { fld0: _24,fld1: _1,fld2: 8074397579320979396_i64,fld3: _15 };
_16.fld1.1 = [_30.fld1.0,_30.fld1.0,_30.fld1.0,_16.fld1.0,_16.fld1.0,_31,_30.fld1.0,_30.fld1.0];
_15.1.1 = _18.fld4 as i8;
RET = _18.fld4 as usize;
_2 = Field::<i16>(Variant(_16.fld0.fld2, 0), 1) - _3;
_8 = (_10.0, _30.fld1.2);
_8 = (_15.1.0, Field::<((isize,), (i32, i8), usize)>(Variant(_30.fld0.fld2, 0), 3).1.1);
SetDiscriminant(_18.fld6.fld1, 0);
place!(Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3)).2 = Field::<u8>(Variant(_14, 2), 0) as usize;
place!(Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3)).0.0 = 16411149068319125205930921820101765580_u128 as isize;
_32.0 = (-153070945967130251601094880901024075792_i128) as i16;
SetDiscriminant(_14, 2);
_16.fld1.1 = [_16.fld1.0,_30.fld1.0,_31,_16.fld1.0,_18.fld0,_30.fld1.0,_18.fld0,_30.fld1.0];
_15.2 = 169_u8 as usize;
Goto(bb14)
}
bb14 = {
_15 = (Field::<((isize,), (i32, i8), usize)>(Variant(_30.fld0.fld2, 0), 3).0, _10, _13);
_29.0 = !Field::<((isize,), (i32, i8), usize)>(Variant(_16.fld0.fld2, 0), 3).0.0;
place!(Field::<((isize,), (i32, i8), usize)>(Variant(_30.fld0.fld2, 0), 3)).2 = _4 as usize;
SetDiscriminant(_18.fld2, 1);
place!(Field::<u8>(Variant(_14, 2), 0)) = 90_u8 - 252_u8;
_10.0 = _23 as i32;
_15.1 = _25;
_27 = _12 * _11;
_30.fld2 = _16.fld2;
_1 = !Field::<i16>(Variant(_16.fld0.fld2, 0), 1);
_10.1 = _25.1 >> _10.0;
_17 = core::ptr::addr_of_mut!(_20.0);
_34.fld6.fld5.0 = (*_17) - (*_17);
_4 = _18.fld4 >> _1;
place!(Field::<f32>(Variant(_14, 2), 1)) = (*_17) - (*_17);
_16.fld3 = _18.fld6.fld0;
_34.fld4 = Field::<((isize,), (i32, i8), usize)>(Variant(_30.fld0.fld2, 0), 3).2 as u64;
_10 = (Field::<((isize,), (i32, i8), usize)>(Variant(_30.fld0.fld2, 0), 3).1.0, Field::<((isize,), (i32, i8), usize)>(Variant(_30.fld0.fld2, 0), 3).1.1);
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(15_usize, 19_usize, Move(_19), 24_usize, Move(_24), 23_usize, Move(_23), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(15_usize, 9_usize, Move(_9), 4_usize, Move(_4), 8_usize, Move(_8), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(15_usize, 3_usize, Move(_3), 5_usize, Move(_5), 10_usize, Move(_10), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: i16,mut _2: i16,mut _3: i32,mut _4: (isize,),mut _5: i16) -> i16 {
mir! {
type RET = i16;
let _6: bool;
let _7: char;
let _8: [bool; 8];
let _9: isize;
let _10: (isize,);
let _11: isize;
let _12: [u16; 7];
let _13: ((isize,), (i32, i8), usize);
let _14: bool;
let _15: i128;
let _16: Adt36;
let _17: f64;
let _18: i16;
let _19: bool;
let _20: [bool; 8];
let _21: Adt39;
let _22: [bool; 8];
let _23: Adt48;
let _24: Adt36;
let _25: i128;
let _26: f32;
let _27: ((isize,), (i32, i8), usize);
let _28: (i16,);
let _29: f64;
let _30: (bool, [bool; 8], i8, u16, i16, i16);
let _31: *mut u64;
let _32: ((isize,), (i32, i8), usize);
let _33: ();
let _34: ();
{
_3 = 2040020576_i32 >> _5;
_6 = false;
RET = 9849439540181229979_usize as i16;
_2 = _5 << _3;
_3 = (-1591282070_i32) << _2;
_6 = !false;
_4 = ((-9223372036854775808_isize),);
_6 = _3 < _3;
_6 = false | true;
_9 = (-126_i8) as isize;
_8 = [_6,_6,_6,_6,_6,_6,_6,_6];
_1 = 126_u8 as i16;
_5 = _2;
_11 = _4.0;
_12 = [35278_u16,36640_u16,49711_u16,12435_u16,37586_u16,30204_u16,62866_u16];
_10.0 = -_9;
_6 = _5 >= _2;
_7 = '\u{10732c}';
_13.0.0 = _11;
Goto(bb1)
}
bb1 = {
_13.1.0 = _3 | _3;
_6 = true & true;
Call(_15 = fn17(_5, _3, _5, _13.1.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13.1.1 = _3 as i8;
_14 = !_6;
_15 = 113477124791075498796742277124652850310_i128 << _2;
_11 = -_13.0.0;
_11 = 132_u8 as isize;
_10.0 = _13.0.0;
_3 = _13.1.0;
_13.0.0 = -_4.0;
_5 = _2;
_12 = [9478_u16,23946_u16,46443_u16,38706_u16,24423_u16,57914_u16,53219_u16];
_13.2 = 10897908377203115516_usize;
_12 = [64798_u16,8482_u16,6081_u16,18989_u16,42377_u16,53766_u16,41333_u16];
_3 = -_13.1.0;
_6 = _14;
_10 = (_9,);
match _4.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb3 = {
_13.1.0 = _3 | _3;
_6 = true & true;
Call(_15 = fn17(_5, _3, _5, _13.1.0), ReturnTo(bb2), UnwindUnreachable())
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
_13.0 = (_10.0,);
_6 = _14;
_3 = 2233106878_u32 as i32;
_13.0 = (_10.0,);
_18 = -_5;
_1 = _18;
_20 = _8;
_13.0 = (_4.0,);
_13.1.0 = _3 | _3;
_19 = !_14;
_3 = _7 as i32;
_11 = _13.1.1 as isize;
_11 = _13.0.0;
_1 = _5 << _5;
Call(_2 = core::intrinsics::transmute(_1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10 = _4;
_22 = [_6,_6,_6,_14,_19,_19,_14,_14];
_8 = [_19,_6,_6,_6,_14,_19,_19,_6];
_10 = _4;
_13.0 = (_10.0,);
_8 = [_6,_6,_19,_14,_19,_19,_19,_14];
_1 = _2 ^ _5;
_4 = _13.0;
_3 = _13.1.0;
_18 = _13.2 as i16;
_15 = 8560309080639737336632226781976216424_i128;
_15 = -(-68999191105388056406114441132887569449_i128);
_17 = 2244080637_u32 as f64;
_17 = _3 as f64;
match _13.2 {
0 => bb9,
1 => bb11,
10897908377203115516 => bb13,
_ => bb12
}
}
bb11 = {
_13.1.0 = _3 | _3;
_6 = true & true;
Call(_15 = fn17(_5, _3, _5, _13.1.0), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_7 = '\u{289b9}';
_13.1.0 = !_3;
_13.2 = 13558647595340425907_usize;
_13.2 = 6_usize ^ 6_usize;
_13.1.0 = _15 as i32;
_4 = (_13.0.0,);
_5 = _1 * _2;
_2 = _5;
_9 = _11;
_27.1.0 = -_3;
_9 = _7 as isize;
_10.0 = _4.0 + _13.0.0;
_27.2 = _13.2;
match _11 {
0 => bb5,
1 => bb12,
340282366920938463454151235394913435648 => bb14,
_ => bb3
}
}
bb14 = {
_15 = !(-75633047745603550586029666379193498982_i128);
_10 = (_11,);
_13.0.0 = _9 + _10.0;
_10.0 = _4.0;
_27 = _13;
_27.0 = _13.0;
_17 = 307657381521963311841712545356482164314_u128 as f64;
_28.0 = 119_u8 as i16;
_13 = (_4, _27.1, _27.2);
_17 = _27.2 as f64;
_25 = _13.2 as i128;
_27 = (_13.0, _13.1, _13.2);
_27.0 = (_4.0,);
_13.1.0 = _27.1.0 * _27.1.0;
_1 = 32234_u16 as i16;
_1 = _2;
_13.1 = (_3, _27.1.1);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(16_usize, 9_usize, Move(_9), 27_usize, Move(_27), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(16_usize, 22_usize, Move(_22), 6_usize, Move(_6), 20_usize, Move(_20), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(16_usize, 12_usize, Move(_12), 18_usize, Move(_18), 15_usize, Move(_15), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: i16,mut _2: i32,mut _3: i16,mut _4: i32) -> i128 {
mir! {
type RET = i128;
let _5: usize;
let _6: (i16,);
let _7: isize;
let _8: u64;
let _9: (i32, i8);
let _10: [bool; 8];
let _11: (i32, i8);
let _12: u8;
let _13: f64;
let _14: (i32, i8);
let _15: isize;
let _16: ();
let _17: ();
{
_4 = _2;
_3 = -_1;
_9 = (_2, 81_i8);
_9 = (_2, 44_i8);
RET = !165737720673397073921920849527059708418_i128;
_8 = !15002478219133869841_u64;
_7 = (-109_isize);
_6.0 = !_1;
_2 = _9.0;
RET = !(-22756053815558956479184801257748877165_i128);
_4 = _9.0;
_9 = (_4, 36_i8);
_4 = -_2;
Goto(bb1)
}
bb1 = {
_9 = (_2, 90_i8);
RET = -(-119102335990866295860327630686786180351_i128);
_11.0 = !_2;
_11.1 = !_9.1;
RET = _9.0 as i128;
_9 = _11;
_9.1 = -_11.1;
_7 = _9.1 as isize;
_13 = (-8799434046677755317_i64) as f64;
_4 = _11.0 | _2;
_1 = !_6.0;
_1 = true as i16;
_5 = 13979586852451536497_usize - 3_usize;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(17_usize, 11_usize, Move(_11), 7_usize, Move(_7), 8_usize, Move(_8), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_16 = dump_var(17_usize, 6_usize, Move(_6), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: ((isize,), (i32, i8), usize),mut _2: (i32, i8)) -> u16 {
mir! {
type RET = u16;
let _3: u128;
let _4: f32;
let _5: f64;
let _6: ((isize,), (i32, i8), usize);
let _7: (f32,);
let _8: (isize,);
let _9: [u16; 7];
let _10: bool;
let _11: Adt43;
let _12: (isize,);
let _13: isize;
let _14: (f32,);
let _15: char;
let _16: (f32,);
let _17: f64;
let _18: f32;
let _19: Adt44;
let _20: char;
let _21: f32;
let _22: f64;
let _23: char;
let _24: i8;
let _25: isize;
let _26: isize;
let _27: *const *const u128;
let _28: [bool; 8];
let _29: f64;
let _30: i8;
let _31: Adt38;
let _32: *mut u64;
let _33: ();
let _34: ();
{
_3 = 201866248777907143272081416894677478322_u128;
_1.2 = 3_usize ^ 1_usize;
_2.1 = _1.1.1 * _1.1.1;
_1.0.0 = (-109_isize);
_1.0.0 = 9223372036854775807_isize;
_3 = !320975269717341160735873317929721667108_u128;
_1.1.0 = _3 as i32;
_4 = _1.0.0 as f32;
RET = 28302_u16 >> _2.1;
_1.0.0 = 9223372036854775807_isize;
_2.1 = !_1.1.1;
_4 = 6394354737414726921_i64 as f32;
_1.1 = _2;
_5 = _1.0.0 as f64;
_1.0.0 = 9223372036854775807_isize;
_1.1.0 = _2.0 * _2.0;
RET = !35979_u16;
_3 = 112143031974358914519545331602044199654_u128 - 339549129047960963688358997845058054604_u128;
match _1.0.0 {
0 => bb1,
1 => bb2,
9223372036854775807 => bb4,
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
_6 = (_1.0, _1.1, _1.2);
_1.1 = (_2.0, _6.1.1);
_6.1.0 = _2.0 ^ _2.0;
_1.0 = (_6.0.0,);
_6 = (_1.0, _2, _1.2);
_1.0 = (_6.0.0,);
_6.0 = _1.0;
_6.2 = 97_u8 as usize;
RET = 31560_i16 as u16;
Goto(bb5)
}
bb5 = {
_2 = _1.1;
_6.1.0 = _2.0 | _1.1.0;
_7 = (_4,);
_9 = [RET,RET,RET,RET,RET,RET,RET];
_7 = (_4,);
_6.1.0 = _2.0;
_7.0 = _4;
_1.1 = (_6.1.0, _2.1);
_5 = _4 as f64;
_9 = [RET,RET,RET,RET,RET,RET,RET];
_1.1.1 = '\u{c17c6}' as i8;
_4 = 78_u8 as f32;
_6.1.0 = 7520917308569103923_i64 as i32;
_1.1.0 = _2.0;
match _1.0.0 {
9223372036854775807 => bb6,
_ => bb1
}
}
bb6 = {
_7 = (_4,);
_6.0 = (_1.0.0,);
_13 = RET as isize;
_1.2 = _6.2 << _2.0;
_8 = (_1.0.0,);
_7 = (_4,);
_6.1.0 = _2.0;
_6.1.1 = !_2.1;
_2 = _1.1;
_16.0 = _4 + _4;
RET = !1537_u16;
_10 = !true;
_16 = (_4,);
RET = _5 as u16;
_8 = (_13,);
_8 = (_6.0.0,);
_6.1 = (_1.1.0, _1.1.1);
_15 = '\u{fb9fc}';
_1.2 = _6.2;
Goto(bb7)
}
bb7 = {
_1.0 = (_13,);
_1.1.1 = _2.1;
_17 = _5;
_6.1 = (_2.0, _1.1.1);
_6.0 = (_8.0,);
_12.0 = _2.1 as isize;
_14.0 = _7.0 - _4;
_2.1 = _1.1.1 ^ _1.1.1;
RET = _15 as u16;
RET = !26462_u16;
_16.0 = _14.0;
RET = (-15162794079586151099599268847415513752_i128) as u16;
_14 = _16;
_18 = _7.0 + _14.0;
Goto(bb8)
}
bb8 = {
_6.2 = !_1.2;
RET = 11018_u16;
_14.0 = _6.1.1 as f32;
_21 = -_16.0;
_6.0 = (_13,);
_6.1.0 = _1.1.0;
_3 = 13010045306103345668_u64 as u128;
_14.0 = _17 as f32;
_1.0.0 = _12.0;
_22 = 24109_i16 as f64;
_1 = (_8, _6.1, _6.2);
_20 = _15;
match RET {
0 => bb9,
1 => bb10,
11018 => bb12,
_ => bb11
}
}
bb9 = {
_6 = (_1.0, _1.1, _1.2);
_1.1 = (_2.0, _6.1.1);
_6.1.0 = _2.0 ^ _2.0;
_1.0 = (_6.0.0,);
_6 = (_1.0, _2, _1.2);
_1.0 = (_6.0.0,);
_6.0 = _1.0;
_6.2 = 97_u8 as usize;
RET = 31560_i16 as u16;
Goto(bb5)
}
bb10 = {
_7 = (_4,);
_6.0 = (_1.0.0,);
_13 = RET as isize;
_1.2 = _6.2 << _2.0;
_8 = (_1.0.0,);
_7 = (_4,);
_6.1.0 = _2.0;
_6.1.1 = !_2.1;
_2 = _1.1;
_16.0 = _4 + _4;
RET = !1537_u16;
_10 = !true;
_16 = (_4,);
RET = _5 as u16;
_8 = (_13,);
_8 = (_6.0.0,);
_6.1 = (_1.1.0, _1.1.1);
_15 = '\u{fb9fc}';
_1.2 = _6.2;
Goto(bb7)
}
bb11 = {
_2 = _1.1;
_6.1.0 = _2.0 | _1.1.0;
_7 = (_4,);
_9 = [RET,RET,RET,RET,RET,RET,RET];
_7 = (_4,);
_6.1.0 = _2.0;
_7.0 = _4;
_1.1 = (_6.1.0, _2.1);
_5 = _4 as f64;
_9 = [RET,RET,RET,RET,RET,RET,RET];
_1.1.1 = '\u{c17c6}' as i8;
_4 = 78_u8 as f32;
_6.1.0 = 7520917308569103923_i64 as i32;
_1.1.0 = _2.0;
match _1.0.0 {
9223372036854775807 => bb6,
_ => bb1
}
}
bb12 = {
RET = 12289_u16;
_8 = (_13,);
_24 = _2.1;
RET = 58495_u16 - 34090_u16;
_1.0 = (_8.0,);
_1.0.0 = _12.0;
_7 = (_18,);
_2.1 = _24;
_23 = _20;
_10 = true;
_2.1 = _24;
_9 = [RET,RET,RET,RET,RET,RET,RET];
_1 = (_8, _6.1, _6.2);
_14.0 = _18 + _7.0;
_6.2 = _1.2 & _1.2;
_5 = _6.2 as f64;
_1 = (_8, _6.1, _6.2);
_1 = (_8, _2, _6.2);
_26 = !_12.0;
_1.2 = _6.2 >> _3;
_2.0 = !_1.1.0;
_25 = _1.0.0 ^ _6.0.0;
RET = 22834_u16 ^ 51197_u16;
Goto(bb13)
}
bb13 = {
_2 = _1.1;
_25 = !_6.0.0;
_12.0 = _8.0;
_12 = (_6.0.0,);
_14.0 = _1.2 as f32;
_1.1.0 = _6.1.0;
_3 = _17 as u128;
_29 = -_5;
_2 = _1.1;
Goto(bb14)
}
bb14 = {
_6.1.0 = -_1.1.0;
_1.0.0 = _13;
_6.0.0 = (-102509194593001299672795755207620992226_i128) as isize;
_28 = [_10,_10,_10,_10,_10,_10,_10,_10];
_6 = (_1.0, _2, _1.2);
_15 = _23;
_8.0 = _12.0 >> _6.1.0;
_6.0.0 = _1.0.0 & _8.0;
_6.2 = RET as usize;
_8 = _1.0;
_24 = _6.1.1 + _1.1.1;
_6.1.0 = _1.1.0 - _2.0;
_23 = _20;
_1.1 = _6.1;
_29 = _17 * _17;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(18_usize, 28_usize, Move(_28), 23_usize, Move(_23), 26_usize, Move(_26), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(18_usize, 10_usize, Move(_10), 8_usize, Move(_8), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(1102_u16), std::hint::black_box(2335007024_u32), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(379873442543202006_usize));
                
            }
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt36::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: usize,
fld1: char,
fld2: u8,
fld3: [u16; 7],
fld4: f64,
fld5: *const u128,
fld6: u128,
fld7: i128,

},
Variant1{
fld0: (f32,),

},
Variant2{
fld0: u8,
fld1: f32,

},
Variant3{
fld0: *const i128,
fld1: i8,
fld2: *mut *const i128,

}}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt37::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: u8,
fld1: u16,
fld2: i16,
fld3: i8,

},
Variant1{
fld0: f32,

},
Variant2{
fld0: Adt36,
fld1: char,
fld2: *mut (bool, [bool; 8], i8, u16, i16, i16),
fld3: *const u128,
fld4: u16,
fld5: u64,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt38::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: i128,
fld1: char,
fld2: [u16; 7],

},
Variant1{
fld0: usize,
fld1: Adt37,
fld2: isize,
fld3: i8,
fld4: (isize,),
fld5: (f64, *mut *const i128, i16, *mut *const u128, (bool, [bool; 8], i8, u16, i16, i16)),

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: *mut *const i128,
fld1: (f32,),
fld2: (f64, *mut *const i128, i16, *mut *const u128, (bool, [bool; 8], i8, u16, i16, i16)),
fld3: i8,
fld4: f32,
fld5: *const *const u128,
fld6: i64,
fld7: Adt37,

},
Variant1{
fld0: Adt37,
fld1: u32,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: u8,
fld1: (f64, *mut *const i128, i16, *mut *const u128, (bool, [bool; 8], i8, u16, i16, i16)),
fld2: *const *const u128,
fld3: i8,

},
Variant1{
fld0: (i16,),
fld1: char,
fld2: ((isize,), (i32, i8), usize),
fld3: u16,
fld4: (isize,),
fld5: u32,
fld6: Adt38,
fld7: (bool, [bool; 8], i8, u16, i16, i16),

}}
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: Adt38,
fld1: (f32,),
fld2: *const u128,
fld3: (f64, *mut *const i128, i16, *mut *const u128, (bool, [bool; 8], i8, u16, i16, i16)),
fld4: i16,
fld5: [bool; 8],
fld6: f64,
fld7: usize,

},
Variant1{
fld0: bool,
fld1: char,
fld2: u128,
fld3: usize,
fld4: f32,
fld5: f64,

},
Variant2{
fld0: bool,
fld1: ((isize,), (i32, i8), usize),
fld2: *mut f32,
fld3: i8,
fld4: *const u128,
fld5: *mut u64,
fld6: (bool, [bool; 8], i8, u16, i16, i16),
fld7: u64,

},
Variant3{
fld0: [u16; 7],

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (i16,),
fld1: i16,
fld2: i64,
fld3: ((isize,), (i32, i8), usize),

},
Variant1{
fld0: ((isize,), (i32, i8), usize),
fld1: Adt40,
fld2: u64,
fld3: (bool, [bool; 8], i8, u16, i16, i16),
fld4: i16,
fld5: *const *const u128,

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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: i32,
fld1: [bool; 8],
fld2: i128,
fld3: Adt41,
fld4: *mut f32,

},
Variant1{
fld0: *mut u64,
fld1: usize,
fld2: *const u128,

}}
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: Adt36,
fld1: i32,

},
Variant1{
fld0: bool,
fld1: char,
fld2: Adt42,
fld3: *mut u64,
fld4: (i32, i8),
fld5: Adt39,

},
Variant2{
fld0: f32,
fld1: [bool; 8],
fld2: *mut u64,
fld3: Adt40,
fld4: (bool, [bool; 8], i8, u16, i16, i16),

},
Variant3{
fld0: (f32,),
fld1: Adt37,
fld2: i128,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *const i128,
fld1: Adt38,
fld2: Adt43,
fld3: Adt40,
fld4: u64,
fld5: (f32,),
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u16,

},
Variant1{
fld0: i128,
fld1: *mut u64,
fld2: u16,

},
Variant2{
fld0: bool,
fld1: *mut f32,
fld2: i128,
fld3: Adt41,
fld4: *const *const u128,
fld5: [u16; 7],
fld6: (i16,),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: Adt42,
}
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: bool,
fld1: Adt45,
fld2: isize,
fld3: *const u128,
fld4: *mut u64,

},
Variant1{
fld0: Adt42,

},
Variant2{
fld0: (i32, i8),
fld1: *mut (bool, [bool; 8], i8, u16, i16, i16),
fld2: isize,
fld3: Adt42,
fld4: i128,
fld5: *mut *const i128,

},
Variant3{
fld0: Adt37,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: bool,
fld1: [u16; 7],
fld2: Adt46,
fld3: Adt42,
fld4: u64,
fld5: Adt41,
fld6: Adt45,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: i16,
fld1: Adt38,
fld2: Adt42,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: (bool, [bool; 8], i8, u16, i16, i16),
fld1: char,
fld2: *mut *const i128,
fld3: Adt45,
fld4: i16,
fld5: Adt42,
fld6: *const *const u128,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: Adt50,
fld1: (bool, [bool; 8], i8, u16, i16, i16),
fld2: *mut *const i128,
fld3: *const i128,
}

