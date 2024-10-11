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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u16,mut _4: i8,mut _5: u64,mut _6: i32,mut _7: i64,mut _8: u8,mut _9: usize) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _10: bool;
let _11: f64;
let _12: bool;
let _13: f64;
let _14: Adt49;
let _15: (i8, i32);
let _16: (i8, i32);
let _17: f32;
let _18: (u16, u16);
let _19: f32;
let _20: *const (i8, i32);
let _21: u16;
let _22: [char; 8];
let _23: Adt52;
let _24: u8;
let _25: u32;
let _26: bool;
let _27: (i128, char, bool, f32);
let _28: f64;
let _29: Adt58;
let _30: ();
let _31: ();
{
_2 = '\u{10389a}';
_1 = true;
RET = [(-32929665129609876032904344135413226553_i128),10787227582321219512874121545340468327_i128,(-72344878286092207620052735297242202383_i128),(-44976790797022494912512933371746006090_i128)];
_7 = 5784455096865535258_i64;
_4 = 29_i8;
RET = [137076464250210871723284569141200418791_i128,24640824026671619630850980536374754470_i128,166327859772151269588636612918657850768_i128,(-107670436795196042100035200760069895319_i128)];
RET = [105783955432047364603540347396601897347_i128,(-165706687316428123119290566178314654471_i128),(-36239176760827348213537284675772299172_i128),152225758327300982067252345083381632586_i128];
_13 = _7 as f64;
_9 = 3_usize & 6380472489300466926_usize;
_14.fld1 = 4053423856_u32 - 2278760948_u32;
_8 = 141_u8 & 12_u8;
_5 = 8948928257032944183_u64 - 13465104644805907361_u64;
_7 = !6686007961071061476_i64;
_13 = 38568_u16 as f64;
_15 = (_4, (-1660949317_i32));
_8 = !80_u8;
_16.0 = _4;
_16.1 = _2 as i32;
_13 = 3822_i16 as f64;
_1 = _14.fld1 != _14.fld1;
_14.fld0 = [_2,_2,_2,_2,_2,_2,_2,_2];
_2 = '\u{23318}';
match _16.0 {
0 => bb1,
29 => bb3,
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
RET = [(-154724244375350166464742276804845989742_i128),108874454538209255959300354986141003714_i128,9086880410501337200995830604457622454_i128,(-21160210012775614119697885055787920687_i128)];
_18.1 = 27288_u16 * 25906_u16;
match _15.0 {
0 => bb1,
29 => bb4,
_ => bb2
}
}
bb4 = {
_3 = !_18.1;
_14.fld1 = 342860058_u32 | 1730302773_u32;
_11 = _13;
_10 = _1 ^ _1;
_16.1 = _15.1 - _15.1;
_16.0 = 23027895396033351768632100599660330177_u128 as i8;
_1 = _10 & _10;
_2 = '\u{89a67}';
_6 = -_15.1;
_12 = _1 & _10;
_14.fld0 = [_2,_2,_2,_2,_2,_2,_2,_2];
_17 = _16.0 as f32;
_12 = _1 > _10;
_20 = core::ptr::addr_of!(_16);
_6 = (*_20).0 as i32;
_17 = _9 as f32;
(*_20).1 = _15.1 - _15.1;
_23 = Adt52::Variant3 { fld0: _8 };
_7 = (-223795377031176161_i64);
Call((*_20).1 = fn1((*_20).0, _15.0, _17, _1, _17, _15, _5, _20, _17), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = _11 * _11;
_25 = _14.fld1 ^ _14.fld1;
_26 = _16.1 < (*_20).1;
_5 = 1508303485941902463_u64;
_21 = _3;
_22 = [_2,_2,_2,_2,_2,_2,_2,_2];
(*_20).1 = _13 as i32;
_12 = !_26;
_27.3 = (-10796656110875923146133525080876296141_i128) as f32;
_27 = ((-168235256392915520242722445039023177174_i128), _2, _12, _17);
match _27.0 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
172047110528022943220652162392745034282 => bb11,
_ => bb10
}
}
bb6 = {
_3 = !_18.1;
_14.fld1 = 342860058_u32 | 1730302773_u32;
_11 = _13;
_10 = _1 ^ _1;
_16.1 = _15.1 - _15.1;
_16.0 = 23027895396033351768632100599660330177_u128 as i8;
_1 = _10 & _10;
_2 = '\u{89a67}';
_6 = -_15.1;
_12 = _1 & _10;
_14.fld0 = [_2,_2,_2,_2,_2,_2,_2,_2];
_17 = _16.0 as f32;
_12 = _1 > _10;
_20 = core::ptr::addr_of!(_16);
_6 = (*_20).0 as i32;
_17 = _9 as f32;
(*_20).1 = _15.1 - _15.1;
_23 = Adt52::Variant3 { fld0: _8 };
_7 = (-223795377031176161_i64);
Call((*_20).1 = fn1((*_20).0, _15.0, _17, _1, _17, _15, _5, _20, _17), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
RET = [(-154724244375350166464742276804845989742_i128),108874454538209255959300354986141003714_i128,9086880410501337200995830604457622454_i128,(-21160210012775614119697885055787920687_i128)];
_18.1 = 27288_u16 * 25906_u16;
match _15.0 {
0 => bb1,
29 => bb4,
_ => bb2
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
_20 = core::ptr::addr_of!((*_20));
_15.1 = _16.1;
_5 = 16706119845572913968_u64;
_3 = _5 as u16;
Call(_15.0 = fn5(Move(_14), _16.1, _12, _27.2, _27.0, _26, _27.0, _12, _27.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5 = !13124575903847802104_u64;
_16 = (_15.0, _15.1);
_1 = _27.0 != _27.0;
_27.2 = !_12;
_9 = _13 as usize;
(*_20).1 = !_6;
_27.3 = _17;
_24 = Field::<u8>(Variant(_23, 3), 0);
(*_20).0 = _15.0 - _15.0;
_18.0 = _3 << _3;
(*_20) = (_4, _15.1);
_24 = _8 - Field::<u8>(Variant(_23, 3), 0);
_27.1 = _2;
_26 = _12 > _10;
_17 = -_27.3;
_14.fld0 = _22;
_21 = !_18.0;
_28 = _11;
_28 = Field::<u8>(Variant(_23, 3), 0) as f64;
_16.1 = _15.1 >> _27.0;
_24 = _8;
match _27.0 {
0 => bb13,
1 => bb14,
172047110528022943220652162392745034282 => bb16,
_ => bb15
}
}
bb13 = {
_13 = _11 * _11;
_25 = _14.fld1 ^ _14.fld1;
_26 = _16.1 < (*_20).1;
_5 = 1508303485941902463_u64;
_21 = _3;
_22 = [_2,_2,_2,_2,_2,_2,_2,_2];
(*_20).1 = _13 as i32;
_12 = !_26;
_27.3 = (-10796656110875923146133525080876296141_i128) as f32;
_27 = ((-168235256392915520242722445039023177174_i128), _2, _12, _17);
match _27.0 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
172047110528022943220652162392745034282 => bb11,
_ => bb10
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
SetDiscriminant(_23, 3);
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(0_usize, 25_usize, Move(_25), 6_usize, Move(_6), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(0_usize, 3_usize, Move(_3), 7_usize, Move(_7), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(0_usize, 24_usize, Move(_24), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i8,mut _2: i8,mut _3: f32,mut _4: bool,mut _5: f32,mut _6: (i8, i32),mut _7: u64,mut _8: *const (i8, i32),mut _9: f32) -> i32 {
mir! {
type RET = i32;
let _10: Adt49;
let _11: f32;
let _12: [i128; 4];
let _13: i128;
let _14: [char; 8];
let _15: *mut &'static i16;
let _16: Adt55;
let _17: u64;
let _18: Adt52;
let _19: i128;
let _20: ();
let _21: ();
{
_9 = -_5;
RET = _6.1;
_10.fld0 = ['\u{30618}','\u{bb83b}','\u{5ea9f}','\u{64fb9}','\u{43ded}','\u{1bce1}','\u{90934}','\u{75d24}'];
RET = _6.1;
_3 = -_5;
_2 = (*_8).0 * (*_8).0;
_9 = _5 * _3;
_7 = (-31784_i16) as u64;
_6.1 = 37876_u16 as i32;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607430107262139 => bb6,
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
(*_8).0 = 13274_u16 as i8;
RET = _6.1 >> _7;
_5 = _9;
_3 = _5;
(*_8).0 = _2 * _1;
(*_8).0 = !_2;
RET = -_6.1;
_2 = _3 as i8;
_14 = ['\u{bb9e8}','\u{2d5d8}','\u{48843}','\u{ac186}','\u{749de}','\u{64537}','\u{22482}','\u{b83ba}'];
_9 = _5;
_10.fld1 = 3327164996_u32 - 1880436094_u32;
RET = _6.1 | _6.1;
Call(_11 = fn2(_10.fld0, _4, _10.fld1, _4, Move(_10), _1, _2, _14, _2, _7, _2, _8, _2, _2, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10.fld1 = !1387150910_u32;
_10.fld0 = ['\u{2014f}','\u{1094e1}','\u{22980}','\u{e3143}','\u{d6417}','\u{4d76b}','\u{8a475}','\u{33383}'];
_4 = _2 < (*_8).0;
RET = -_6.1;
_8 = core::ptr::addr_of!(_6);
(*_8) = (_2, RET);
RET = _4 as i32;
_10 = Adt49 { fld0: _14,fld1: 2476060182_u32 };
_10 = Adt49 { fld0: _14,fld1: 2119670908_u32 };
_9 = _5;
_1 = _2 | _2;
_5 = _9 + _9;
(*_8).1 = 2933391279913107480_usize as i32;
(*_8).0 = _1 - _1;
_2 = !(*_8).0;
(*_8).1 = RET;
_5 = 9223372036854775807_isize as f32;
RET = -(*_8).1;
_16 = Adt55::Variant0 { fld0: 2446346774963982731_i64,fld1: (-9223372036854775808_isize) };
_10 = Adt49 { fld0: _14,fld1: 1804094343_u32 };
_1 = (*_8).0 ^ (*_8).0;
_13 = 38995748293280171223578158962459015893_i128;
_13 = (-16474198257849223001071683019534831482_i128) - (-139695336311454450496250188997771817907_i128);
(*_8).1 = -RET;
_10.fld0 = ['\u{109601}','\u{96c65}','\u{e6946}','\u{b05bb}','\u{77c7d}','\u{42280}','\u{57faa}','\u{879d6}'];
Call(place!(Field::<isize>(Variant(_16, 0), 1)) = fn4(_1, _1, _4, _3, _6.0, _6, _4, (*_8), _1, _10.fld0, _8, _8, (*_8), _5, _4, (*_8).1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9 = _3;
_10.fld1 = 1064460142_u32 + 1943129010_u32;
place!(Field::<isize>(Variant(_16, 0), 1)) = (-9223372036854775808_isize);
_4 = !false;
(*_8).1 = !RET;
_5 = Field::<isize>(Variant(_16, 0), 1) as f32;
_1 = _6.0 >> (*_8).0;
_4 = true | true;
_10.fld1 = '\u{106a57}' as u32;
RET = (*_8).1 + (*_8).1;
(*_8) = (_2, RET);
_18 = Adt52::Variant2 { fld0: _10.fld1 };
place!(Field::<isize>(Variant(_16, 0), 1)) = 9223372036854775807_isize - 9223372036854775807_isize;
_19 = 6_usize as i128;
_5 = 15840917732156317177_usize as f32;
Goto(bb9)
}
bb9 = {
Call(_20 = dump_var(1_usize, 7_usize, Move(_7), 13_usize, Move(_13), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [char; 8],mut _2: bool,mut _3: u32,mut _4: bool,mut _5: Adt49,mut _6: i8,mut _7: i8,mut _8: [char; 8],mut _9: i8,mut _10: u64,mut _11: i8,mut _12: *const (i8, i32),mut _13: i8,mut _14: i8,mut _15: bool) -> f32 {
mir! {
type RET = f32;
let _16: ((i8, i32), (u32,), *mut u64);
let _17: char;
let _18: (u32,);
let _19: (u16, u16);
let _20: u32;
let _21: (i8, i32);
let _22: f64;
let _23: i32;
let _24: (*mut &'static i16,);
let _25: f64;
let _26: Adt53;
let _27: u128;
let _28: (u32,);
let _29: [i128; 4];
let _30: [char; 8];
let _31: (i64, i128, u16, f64, i8);
let _32: f32;
let _33: [i128; 4];
let _34: (u32,);
let _35: u128;
let _36: ();
let _37: ();
{
_10 = 1135841788231027337_u64 + 6840607681763198423_u64;
(*_12).0 = _7;
_5 = Adt49 { fld0: _1,fld1: _3 };
_14 = -(*_12).0;
RET = 7654998426603825256_i64 as f32;
_2 = _4;
(*_12).0 = -_11;
_1 = _5.fld0;
_16.0.0 = 52667_u16 as i8;
(*_12).0 = _6;
(*_12).0 = -_7;
_11 = -_16.0.0;
_18 = (_5.fld1,);
_17 = '\u{12f5c}';
_12 = core::ptr::addr_of!(_16.0);
_4 = !_2;
RET = 23_isize as f32;
_8 = _1;
(*_12).0 = 101_u8 as i8;
_19 = (3770_u16, 59888_u16);
_15 = _13 < _13;
_5 = Adt49 { fld0: _1,fld1: _18.0 };
(*_12).0 = _9;
_19 = (46543_u16, 59874_u16);
Call(_19.0 = core::intrinsics::bswap(_19.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17 = '\u{a31f7}';
_19.0 = (-11750_i16) as u16;
_21.0 = _13 >> _13;
_17 = '\u{a061a}';
_19.0 = !_19.1;
_5.fld1 = _3 << _3;
_16.1 = (_18.0,);
Goto(bb2)
}
bb2 = {
_18 = (_5.fld1,);
_8 = [_17,_17,_17,_17,_17,_17,_17,_17];
(*_12) = (_14, (-1159704778_i32));
_20 = 225917830753283937667739388953413693987_u128 as u32;
match _16.0.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607430608506678 => bb10,
_ => bb9
}
}
bb3 = {
_17 = '\u{a31f7}';
_19.0 = (-11750_i16) as u16;
_21.0 = _13 >> _13;
_17 = '\u{a061a}';
_19.0 = !_19.1;
_5.fld1 = _3 << _3;
_16.1 = (_18.0,);
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
_10 = !7363451978407986321_u64;
_2 = _4;
_14 = _9 >> _13;
_19.0 = 9223372036854775807_isize as u16;
_19.1 = _7 as u16;
_16.1 = (_18.0,);
(*_12) = (_9, 1960106125_i32);
_5 = Adt49 { fld0: _1,fld1: _18.0 };
_11 = (*_12).0 ^ _14;
_6 = !_21.0;
_21.1 = !(*_12).1;
(*_12) = (_11, _21.1);
_3 = _17 as u32;
_19.1 = _19.0;
Goto(bb11)
}
bb11 = {
_22 = _16.0.1 as f64;
_27 = !25714131960926936547879727276996566846_u128;
_26.fld0.0 = [(-67520908668822948265343849109595029489_i128),14608325885377880186548906977420179987_i128,81254236203924031289711125640687038969_i128,156721447288975210254584878834468551502_i128];
_26.fld0.1 = _26.fld0.0;
_26.fld1.fld2.0.1 = (*_12).1;
_16.1.0 = _3;
_7 = (*_12).0 & (*_12).0;
_31 = (6181529643604578429_i64, 94105727842965384556791499699765856662_i128, _19.0, _22, _7);
_30 = _5.fld0;
_18 = (_3,);
_3 = _5.fld1;
Goto(bb12)
}
bb12 = {
_10 = 2534799761037882432_u64;
_26.fld0.1 = [_31.1,_31.1,_31.1,_31.1];
_6 = (*_12).0 | (*_12).0;
_19.0 = _31.2;
_20 = _3;
_19 = (_31.2, _31.2);
_10 = RET as u64;
Call(_20 = core::intrinsics::transmute((*_12).1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_12) = _21;
Call(_33 = fn3(_7, _31.0, _31, _31.0, _15, _31.0, _31.0, _27, _6, _31.0, _7, _31.0, _31, _21.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_28.0 = _10 as u32;
_25 = _22;
_26.fld1.fld2.2 = !_3;
(*_12).1 = _21.1;
_26.fld1.fld2.0.0 = _26.fld1.fld2.2 as i8;
_31.1 = 37590357907678595177076989615605631227_i128 << _11;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(2_usize, 30_usize, Move(_30), 2_usize, Move(_2), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(2_usize, 21_usize, Move(_21), 20_usize, Move(_20), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(2_usize, 13_usize, Move(_13), 19_usize, Move(_19), 27_usize, Move(_27), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: i8,mut _2: i64,mut _3: (i64, i128, u16, f64, i8),mut _4: i64,mut _5: bool,mut _6: i64,mut _7: i64,mut _8: u128,mut _9: i8,mut _10: i64,mut _11: i8,mut _12: i64,mut _13: (i64, i128, u16, f64, i8),mut _14: i8) -> [i128; 4] {
mir! {
type RET = [i128; 4];
let _15: i64;
let _16: usize;
let _17: f64;
let _18: char;
let _19: (i64, i128, u16, f64, i8);
let _20: [i64; 7];
let _21: Adt56;
let _22: ();
let _23: ();
{
RET = [_3.1,_3.1,_13.1,_3.1];
_2 = -_6;
_10 = _13.0 >> _6;
_13.0 = 9223372036854775807_isize as i64;
RET = [_3.1,_13.1,_3.1,_3.1];
_9 = (-9223372036854775808_isize) as i8;
_3.1 = _13.1;
_13.0 = -_4;
_13.0 = _7 & _4;
_9 = _3.1 as i8;
_12 = !_2;
_12 = 2442163981_u32 as i64;
_3 = _13;
_9 = _13.4 >> _13.0;
_2 = _3.0 & _6;
_8 = _5 as u128;
_19 = (_13.0, _13.1, _3.2, _3.3, _13.4);
_7 = _11 as i64;
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(3_usize, 7_usize, Move(_7), 5_usize, Move(_5), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(3_usize, 10_usize, Move(_10), 8_usize, Move(_8), 23_usize, _23, 23_usize, _23), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i8,mut _2: i8,mut _3: bool,mut _4: f32,mut _5: i8,mut _6: (i8, i32),mut _7: bool,mut _8: (i8, i32),mut _9: i8,mut _10: [char; 8],mut _11: *const (i8, i32),mut _12: *const (i8, i32),mut _13: (i8, i32),mut _14: f32,mut _15: bool,mut _16: i32) -> isize {
mir! {
type RET = isize;
let _17: ([i128; 4], [i128; 4], *mut u64);
let _18: bool;
let _19: f32;
let _20: (i64, i128, u16, f64, i8);
let _21: (i8, i32);
let _22: f32;
let _23: isize;
let _24: ();
let _25: ();
{
RET = 19_isize - (-9223372036854775808_isize);
RET = 9223372036854775807_isize + 9223372036854775807_isize;
(*_11).0 = !_2;
_13 = (_8.0, (*_12).1);
(*_11).0 = _8.0 << (*_12).1;
_6.0 = !_1;
(*_11).0 = _9;
(*_11).1 = -_16;
RET = (-9223372036854775808_isize);
_13.0 = (*_11).0 * (*_11).0;
_5 = -_6.0;
(*_11) = _8;
_17.0 = [36575188249275834904381690686798202603_i128,117028332791247624251242844307040433162_i128,110123076409039542799806992259090381675_i128,(-18706507759699816071383282655441884738_i128)];
(*_12).1 = _6.1;
(*_11) = (_2, _13.1);
(*_12).1 = _6.1 | _6.1;
(*_11).1 = _13.1 & _6.1;
_7 = !_15;
_6.1 = 212034579152193214688383463123550613655_u128 as i32;
_2 = -_13.0;
_3 = _2 == _9;
_7 = _3 | _3;
(*_12) = (_13.0, _16);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463454151235394913435648 => bb9,
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
_1 = _7 as i8;
(*_12).0 = _13.0;
_3 = _7 == _7;
_8 = (*_11);
(*_12).1 = (*_12).0 as i32;
(*_12).1 = _14 as i32;
_17.1 = _17.0;
_4 = -_14;
(*_12) = _13;
_13 = ((*_12).0, _8.1);
_18 = _6.0 == (*_11).0;
_14 = 3740574375_u32 as f32;
match RET {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463454151235394913435648 => bb14,
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
_19 = -_4;
_17.1 = [147340231415973929488030553125085154362_i128,105433819835808542758547584936473852526_i128,(-169945043923257313221659506764830701014_i128),55274104807828298010391522253424526320_i128];
_16 = (*_11).1;
_5 = _8.0 >> (*_11).0;
(*_11).0 = 53882_u16 as i8;
_8 = (_2, (*_12).1);
_7 = _18;
_20.3 = 78319339336738869064648500125780661304_i128 as f64;
(*_11).0 = !_9;
(*_11) = _13;
_15 = _7;
_20.1 = RET as i128;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(4_usize, 13_usize, Move(_13), 6_usize, Move(_6), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(4_usize, 2_usize, Move(_2), 18_usize, Move(_18), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: Adt49,mut _2: i32,mut _3: bool,mut _4: bool,mut _5: i128,mut _6: bool,mut _7: i128,mut _8: bool,mut _9: i128) -> i8 {
mir! {
type RET = i8;
let _10: [char; 8];
let _11: Adt44;
let _12: Adt54;
let _13: (i128, char, bool, f32);
let _14: f64;
let _15: u16;
let _16: i8;
let _17: i32;
let _18: i32;
let _19: i128;
let _20: isize;
let _21: char;
let _22: u32;
let _23: bool;
let _24: f32;
let _25: char;
let _26: isize;
let _27: (*mut &'static i16, (i64, i128, u16, f64, i8));
let _28: Adt52;
let _29: ();
let _30: ();
{
RET = 12050100111440644919_usize as i8;
_5 = -_7;
_9 = _5;
_3 = _8;
_8 = _7 < _5;
_8 = _3;
_10 = ['\u{a31a2}','\u{53a21}','\u{14d12}','\u{100945}','\u{c378f}','\u{24268}','\u{9be4c}','\u{8af7c}'];
_9 = _7 * _5;
_10 = ['\u{107701}','\u{9ece6}','\u{2b307}','\u{fee02}','\u{10948c}','\u{ef828}','\u{34864}','\u{184ae}'];
RET = 33_i8 << _5;
_3 = _4;
_8 = _3 <= _3;
_8 = _3 ^ _6;
_3 = _9 < _9;
_8 = _4;
RET = 7_i8;
_3 = _4;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
172047110528022943220652162392745034282 => bb7,
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
_13.3 = RET as f32;
_6 = _3;
_5 = -_9;
_1.fld0 = ['\u{9728b}','\u{16af5}','\u{2f406}','\u{4c8a5}','\u{93a2c}','\u{d7d1}','\u{9c2fe}','\u{46f03}'];
_9 = _5;
RET = -(-21_i8);
_13.0 = _1.fld1 as i128;
_13.1 = '\u{40503}';
_13.0 = !_5;
_1.fld0 = _10;
_14 = 304836232124280071863592907335173175372_u128 as f64;
_16 = RET * RET;
_7 = !_13.0;
_13.0 = -_7;
_18 = _2;
_15 = 62153_u16 >> _13.0;
_16 = -RET;
_13.1 = '\u{bea27}';
_13.2 = _8;
_1 = Adt49 { fld0: _10,fld1: 2692260201_u32 };
_16 = RET;
_18 = -_2;
_14 = 70_isize as f64;
_16 = -RET;
_3 = _13.2;
_19 = !_7;
_3 = !_6;
match _1.fld1 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
2692260201 => bb12,
_ => bb11
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
Return()
}
bb12 = {
_17 = _2;
_4 = _8 ^ _6;
_6 = !_13.2;
_1 = Adt49 { fld0: _10,fld1: 2443360093_u32 };
_4 = !_8;
_13.2 = _4;
_13.3 = 15596462042051190679_u64 as f32;
_10 = _1.fld0;
_13.1 = '\u{c973a}';
_15 = !42247_u16;
_4 = _8;
_13.2 = _6;
_9 = _5;
_6 = !_8;
_8 = !_3;
_1 = Adt49 { fld0: _10,fld1: 2737865264_u32 };
_6 = !_4;
_13.1 = '\u{65f52}';
_13.0 = _7;
_8 = _13.2;
_21 = _13.1;
match _1.fld1 {
0 => bb7,
1 => bb10,
2 => bb13,
2737865264 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_13.3 = RET as f32;
_6 = _3;
_5 = -_9;
_1.fld0 = ['\u{9728b}','\u{16af5}','\u{2f406}','\u{4c8a5}','\u{93a2c}','\u{d7d1}','\u{9c2fe}','\u{46f03}'];
_9 = _5;
RET = -(-21_i8);
_13.0 = _1.fld1 as i128;
_13.1 = '\u{40503}';
_13.0 = !_5;
_1.fld0 = _10;
_14 = 304836232124280071863592907335173175372_u128 as f64;
_16 = RET * RET;
_7 = !_13.0;
_13.0 = -_7;
_18 = _2;
_15 = 62153_u16 >> _13.0;
_16 = -RET;
_13.1 = '\u{bea27}';
_13.2 = _8;
_1 = Adt49 { fld0: _10,fld1: 2692260201_u32 };
_16 = RET;
_18 = -_2;
_14 = 70_isize as f64;
_16 = -RET;
_3 = _13.2;
_19 = !_7;
_3 = !_6;
match _1.fld1 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
2692260201 => bb12,
_ => bb11
}
}
bb15 = {
_22 = _1.fld1;
_4 = _19 < _7;
_9 = !_19;
_23 = _4 >= _4;
_19 = -_13.0;
_13.2 = !_3;
_8 = !_23;
_23 = _3;
_4 = !_8;
_20 = !(-9223372036854775808_isize);
_27.1.2 = 2_usize as u16;
_15 = _27.1.2 * _27.1.2;
_8 = !_6;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(5_usize, 3_usize, Move(_3), 20_usize, Move(_20), 17_usize, Move(_17), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(5_usize, 9_usize, Move(_9), 8_usize, Move(_8), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(5_usize, 19_usize, Move(_19), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{fc66f}'), std::hint::black_box(20063_u16), std::hint::black_box((-123_i8)), std::hint::black_box(14330254556546404550_u64), std::hint::black_box(14763206_i32), std::hint::black_box(3380562466335519366_i64), std::hint::black_box(168_u8), std::hint::black_box(7738522257092314715_usize));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: *mut u64,
fld1: *const f64,
fld2: *const (i8, i32),
}
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: ((i8, i32), (u32,), *mut u64),
fld2: [i64; 7],
fld3: u16,
fld4: (i8, i32),
fld5: i32,
fld6: *mut u8,
fld7: (u16, u16),

},
Variant1{
fld0: i16,

},
Variant2{
fld0: (u32,),
fld1: ([i128; 4], [i128; 4], *mut u64),

},
Variant3{
fld0: u8,
fld1: u32,
fld2: usize,
fld3: u64,
fld4: f32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut u64,
fld1: u8,
fld2: (i128, char, bool, f32),
fld3: i32,

},
Variant1{
fld0: Adt42,

},
Variant2{
fld0: (i8, i32),
fld1: *mut u64,
fld2: u128,
fld3: f64,
fld4: u64,
fld5: i32,
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut u8,

},
Variant1{
fld0: [u64; 2],
fld1: u32,
fld2: f32,
fld3: i128,
fld4: (i128, char, bool, f32),
fld5: u64,
fld6: (u16, u16),

},
Variant2{
fld0: u32,
fld1: u64,
fld2: ((i8, i32), *const f64, u32),
fld3: f64,
fld4: (i64, i128, u16, f64, i8),
fld5: i32,
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: u128,
fld1: *const (i8, i32),
fld2: ((i8, i32), *const f64, u32),
fld3: i8,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: ((i8, i32), *const f64, u32),
fld1: f32,

},
Variant1{
fld0: Adt42,
fld1: (i128, char, bool, f32),
fld2: *mut u8,
fld3: ([i128; 4], [i128; 4], *mut u64),
fld4: i32,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const u128,
fld1: char,
fld2: u16,

},
Variant1{
fld0: usize,
fld1: (u32,),
fld2: *mut *mut u8,

},
Variant2{
fld0: (u16, u16),
fld1: u8,

},
Variant3{
fld0: [i128; 4],
fld1: (u16, u16),
fld2: u8,
fld3: (i64, i128, u16, f64, i8),
fld4: u64,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: [char; 8],
fld1: u32,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt45,
fld1: ((i8, i32), (u32,), *mut u64),
fld2: isize,
fld3: usize,
fld4: i16,

},
Variant1{
fld0: u32,
fld1: Adt44,
fld2: ([i128; 4], [i128; 4], *mut u64),
fld3: i8,
fld4: ((i8, i32), *const f64, u32),
fld5: i32,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [u64; 2],
fld1: *mut u64,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: Adt43,

},
Variant1{
fld0: [i64; 7],
fld1: ((i8, i32), *const f64, u32),

},
Variant2{
fld0: u32,

},
Variant3{
fld0: u8,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: ([i128; 4], [i128; 4], *mut u64),
fld1: Adt46,
}
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
fld0: Adt53,
fld1: *const f64,
fld2: isize,
fld3: (i64, i128, u16, f64, i8),
fld4: *const u128,
fld5: u16,
fld6: ([i128; 4], [i128; 4], *mut u64),
fld7: Adt51,

},
Variant1{
fld0: Adt44,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: i64,
fld1: isize,

},
Variant1{
fld0: Adt42,
fld1: char,
fld2: *mut *mut u8,
fld3: i8,
fld4: i32,

},
Variant2{
fld0: Adt50,
fld1: [u64; 2],
fld2: u128,
fld3: u64,
fld4: *const u128,
fld5: Adt44,
fld6: i64,

},
Variant3{
fld0: *mut *mut u8,
fld1: f32,
fld2: Adt47,
fld3: u8,
fld4: Adt44,
fld5: ((i8, i32), (u32,), *mut u64),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (i128, char, bool, f32),
fld1: u64,
fld2: u32,

},
Variant1{
fld0: (i128, char, bool, f32),
fld1: Adt50,
fld2: isize,
fld3: ((i8, i32), (u32,), *mut u64),
fld4: usize,
fld5: i128,
fld6: Adt42,

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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u16,
fld1: *const f64,
fld2: Adt52,
fld3: Adt56,
fld4: Adt45,

},
Variant1{
fld0: usize,
fld1: Adt47,
fld2: [char; 8],
fld3: (i8, i32),
fld4: i16,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [i64; 7],
fld1: (i8, i32),
fld2: isize,
fld3: ((i8, i32), (u32,), *mut u64),
fld4: Adt55,
fld5: *const isize,
fld6: Adt45,

},
Variant1{
fld0: bool,
fld1: Adt49,
fld2: isize,
fld3: *const isize,
fld4: ([i128; 4], [i128; 4], *mut u64),
fld5: Adt53,

}}

