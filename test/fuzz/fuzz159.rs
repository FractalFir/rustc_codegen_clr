#![allow(dead_code,unused_variables)]#![recursion_limit = "1024"]
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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u8,mut _8: i128,mut _9: u32) -> [i16; 6] {
mir! {
type RET = [i16; 6];
let _10: u128;
let _11: u32;
let _12: Adt52;
let _13: i64;
let _14: f32;
let _15: (usize, bool);
let _16: u8;
let _17: Adt58;
let _18: (i64,);
let _19: (u32, f64, i8, f64, bool);
let _20: char;
let _21: (u32, f64, i8, f64, bool);
let _22: usize;
let _23: *const bool;
let _24: *const bool;
let _25: i16;
let _26: Adt53;
let _27: usize;
let _28: f64;
let _29: (u8, i64, u64, [i128; 7]);
let _30: bool;
let _31: Adt55;
let _32: (u8, i64, u64, [i128; 7]);
let _33: isize;
let _34: usize;
let _35: (f64, i32);
let _36: i64;
let _37: (usize, bool);
let _38: Adt56;
let _39: bool;
let _40: Adt58;
let _41: f64;
let _42: [u128; 6];
let _43: Adt53;
let _44: Adt54;
let _45: Adt57;
let _46: i16;
let _47: i64;
let _48: i128;
let _49: [u128; 6];
let _50: i64;
let _51: *const ([i128; 7], bool);
let _52: usize;
let _53: ();
let _54: ();
{
RET = [(-16283_i16),14361_i16,(-1723_i16),3548_i16,(-17718_i16),26132_i16];
_1 = 4732119290548943638_u64 + 11678760815541705210_u64;
_3 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_9 = 29624468923325849511379615100652686006_u128 as u32;
_3 = (-32356_i16) as isize;
_5 = 208181663475343824619158135494818982859_u128 as i16;
_1 = 9647814684545921197_u64;
_11 = _9;
_9 = _11;
_7 = 86_u8 - 232_u8;
_2 = '\u{ceedd}';
_11 = _9 - _9;
_1 = 10980263762319361731_u64;
RET = [_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5];
RET = [_5,_5,_5,_5,_5,_5];
_3 = 23542355673523599401096060795901325298_i128 as isize;
_6 = (-357697997_i32) * 294573530_i32;
_5 = -(-21961_i16);
_1 = !13120163494117259700_u64;
Goto(bb1)
}
bb1 = {
_15 = (6_usize, false);
_15.1 = !false;
_4 = (-47_i8) + 8_i8;
_15.1 = false;
_16 = _6 as u8;
_19.2 = _4 + _4;
_11 = _9;
_17.fld0 = (_15.0, _15.1);
_8 = -(-88570225150150940754087734528585658834_i128);
_13 = _15.0 as i64;
_13 = _15.0 as i64;
RET = [_5,_5,_5,_5,_5,_5];
_7 = _3 as u8;
_17.fld1 = [_17.fld0.1,_17.fld0.1,_17.fld0.1,_15.1,_15.1,_17.fld0.1,_17.fld0.1];
_20 = _2;
_21.3 = _6 as f64;
_11 = _9 << _15.0;
_21.1 = -_21.3;
_10 = !339314894169625145628268398405962162255_u128;
_19.0 = !_11;
_18.0 = -_13;
_19.0 = _11;
_18 = (_13,);
_17.fld0.0 = !_15.0;
_14 = 19807_u16 as f32;
Goto(bb2)
}
bb2 = {
_15.1 = _17.fld0.1;
_23 = core::ptr::addr_of!(_21.4);
(*_23) = _17.fld0.1;
_15 = _17.fld0;
_18 = (_13,);
_19.1 = _21.1 - _21.3;
_7 = _16;
_24 = core::ptr::addr_of!(_17.fld0.1);
_19.1 = -_21.1;
Call(_19.1 = fn1(_18, (*_24), _17.fld0.0, _21.3, (*_24), _18.0, _23, _7, _3, _16, _21.1, _23, _20, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_18 = (_13,);
_19.0 = !_9;
_21.0 = _11;
_17.fld6 = _5 as i64;
(*_24) = !(*_23);
_21.2 = -_19.2;
_23 = _24;
_7 = _16;
_17.fld2 = [_10,_10,_10,_10,_10,_10];
_14 = _8 as f32;
_17.fld3.0 = _19.1;
_14 = _1 as f32;
_17.fld4 = _5 - _5;
_4 = _19.2;
Goto(bb4)
}
bb4 = {
_19.4 = _17.fld3.0 != _17.fld3.0;
_19.3 = _19.1 * _19.1;
_15.0 = _17.fld0.0;
Goto(bb5)
}
bb5 = {
_19.1 = _19.3;
Goto(bb6)
}
bb6 = {
_19.1 = 52252_u16 as f64;
(*_24) = !_19.4;
_29.1 = -_18.0;
_19.4 = (*_23);
_15.1 = (*_24);
_30 = _17.fld0.1 ^ _19.4;
(*_23) = _15.1 >= _15.1;
_21 = (_11, _19.3, _4, _17.fld3.0, _15.1);
_21.4 = (*_24);
_17.fld3.0 = _21.1;
_18 = (_13,);
_22 = _17.fld0.0;
_25 = _17.fld4;
_28 = -_17.fld3.0;
Goto(bb7)
}
bb7 = {
Goto(bb8)
}
bb8 = {
_35 = (_19.3, _6);
_9 = _21.0 + _11;
_25 = _5 << _4;
_15.0 = _22 | _22;
_2 = _20;
_8 = _21.3 as i128;
_32.3 = [_8,_8,_8,_8,_8,_8,_8];
(*_23) = _30 & _21.4;
(*_24) = _21.4 | _21.4;
_29.2 = _14 as u64;
_36 = _2 as i64;
(*_24) = _19.4 & _30;
_34 = !_22;
_17.fld0.1 = _19.1 < _35.0;
_17.fld3.1 = _6;
_29.2 = _1;
Goto(bb9)
}
bb9 = {
_17.fld3 = (_21.1, _35.1);
_23 = core::ptr::addr_of!(_19.4);
_24 = core::ptr::addr_of!((*_23));
_32.3 = [_8,_8,_8,_8,_8,_8,_8];
_17.fld0.0 = _22;
_17.fld1 = [_30,_30,_21.4,_15.1,(*_24),_15.1,(*_23)];
_40.fld2 = _17.fld2;
_40.fld3 = (_28, _6);
Call(_5 = fn9((*_23), _8, _32.3, _23, _32.3, _17.fld3.0, _23, (*_24), (*_23), _35, _40.fld3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_33 = _3 - _3;
_40.fld3.1 = _35.1;
_17.fld0.1 = !(*_24);
_39 = _19.4;
_42 = [_10,_10,_10,_10,_10,_10];
_18.0 = _13;
_29.3 = [_8,_8,_8,_8,_8,_8,_8];
_37 = Checked(_15.0 * _17.fld0.0);
_32.2 = _1;
Goto(bb11)
}
bb11 = {
_41 = _21.3 * _21.1;
_21.2 = -_19.2;
_32.0 = _20 as u8;
_3 = _33;
_24 = _23;
_18 = (_29.1,);
_21.3 = _40.fld3.0 + _28;
_17.fld6 = _13;
_27 = _17.fld3.0 as usize;
_17.fld2 = _40.fld2;
_17.fld1 = [_19.4,_39,(*_23),(*_23),(*_24),(*_23),_19.4];
Goto(bb12)
}
bb12 = {
_40.fld3.1 = _6 & _6;
_21.4 = (*_24) >= _39;
_29.1 = _18.0;
RET = [_5,_17.fld4,_5,_5,_5,_5];
(*_24) = _17.fld0.1;
_40.fld3 = (_19.3, _35.1);
_3 = _33;
_17.fld0.0 = !_27;
_6 = _16 as i32;
_15.0 = _17.fld0.0 ^ _17.fld0.0;
_29.2 = _3 as u64;
_32.3 = [_8,_8,_8,_8,_8,_8,_8];
_40.fld4 = _20 as i16;
_19.1 = _35.0 + _40.fld3.0;
_17.fld0.1 = _37.1;
_17.fld1 = [(*_23),_39,_21.4,(*_24),_30,(*_24),_30];
_17.fld2 = [_10,_10,_10,_10,_10,_10];
_52 = _2 as usize;
_25 = _5 - _5;
Goto(bb13)
}
bb13 = {
Call(_53 = dump_var(0_usize, 4_usize, Move(_4), 20_usize, Move(_20), 2_usize, Move(_2), 25_usize, Move(_25)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_53 = dump_var(0_usize, 37_usize, Move(_37), 11_usize, Move(_11), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_53 = dump_var(0_usize, 8_usize, Move(_8), 33_usize, Move(_33), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(0_usize, 30_usize, Move(_30), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: (i64,),mut _2: bool,mut _3: usize,mut _4: f64,mut _5: bool,mut _6: i64,mut _7: *const bool,mut _8: u8,mut _9: isize,mut _10: u8,mut _11: f64,mut _12: *const bool,mut _13: char,mut _14: [i16; 6]) -> f64 {
mir! {
type RET = f64;
let _15: (i16, bool);
let _16: [bool; 7];
let _17: isize;
let _18: isize;
let _19: bool;
let _20: f32;
let _21: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32);
let _22: f64;
let _23: [isize; 8];
let _24: isize;
let _25: u8;
let _26: [usize; 7];
let _27: i32;
let _28: bool;
let _29: [bool; 7];
let _30: [bool; 7];
let _31: ();
let _32: ();
{
_3 = 2333_u16 as usize;
_11 = -_4;
_3 = 0_usize;
_4 = _11 * _11;
_2 = (*_12) >= _5;
_3 = !15046480167055002913_usize;
_15.1 = (*_12);
_10 = !_8;
_4 = _9 as f64;
_15.0 = 4479560983469015536_u64 as i16;
(*_12) = !_5;
_8 = !_10;
RET = (-99726084266838900668525993506113172497_i128) as f64;
_6 = _1.0;
RET = _6 as f64;
_8 = _10;
(*_7) = _2 < _5;
_13 = '\u{f2f33}';
Call(_14 = fn2((*_12), _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = -_11;
(*_7) = !_15.1;
(*_12) = !_15.1;
_15.1 = !(*_12);
_16 = [_5,_15.1,(*_12),(*_12),(*_7),(*_7),_2];
_15 = Checked((-2150_i16) + (-21040_i16));
(*_12) = _15.1;
(*_12) = _5;
(*_12) = !_2;
_3 = !14293154393104156458_usize;
_3 = !10229156085972944008_usize;
(*_12) = !_15.1;
RET = 1413540702_i32 as f64;
(*_7) = _15.1;
_3 = _10 as usize;
_10 = _8 * _8;
(*_12) = _5;
(*_7) = _15.0 > _15.0;
_1 = (_6,);
_12 = core::ptr::addr_of!(_2);
(*_7) = !_15.1;
_16 = [(*_12),_15.1,(*_7),_2,(*_12),(*_12),(*_7)];
_3 = _15.0 as usize;
_2 = _15.1;
_4 = -_11;
Goto(bb2)
}
bb2 = {
_15 = ((-2256_i16), (*_12));
_14 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_12 = core::ptr::addr_of!((*_7));
_17 = 1871351763_i32 as isize;
_11 = _3 as f64;
RET = _11;
_4 = RET;
RET = _4;
(*_7) = _15.1;
_17 = _9;
_15.1 = (*_7);
_1.0 = RET as i64;
_18 = _17;
_6 = _1.0;
_5 = !(*_7);
_1.0 = _6 & _6;
Call(RET = core::intrinsics::fmaf64(_4, _11, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = 183843253183685991996162849503313445074_u128 as i64;
_1.0 = _6;
_9 = 1426164799_i32 as isize;
_6 = _1.0 * _1.0;
_1 = (_6,);
_15.0 = 42561_u16 as i16;
_1.0 = _6;
(*_7) = !_5;
_12 = _7;
_6 = 3475322911_u32 as i64;
_7 = core::ptr::addr_of!(_5);
(*_12) = _5;
_16 = [_15.1,(*_7),(*_12),(*_12),_5,(*_7),(*_12)];
_6 = _1.0;
_21.1.2.0 = [85150730647484095307822347223514369034_i128,9867565638745596818994136063172196397_i128,158120845297923023061249908329246354338_i128,55829866466044222811133417435507187508_i128,100009714823479812155272696237171224801_i128,139038474535703437021642442234762060558_i128,(-137068397417789166852044182676836796354_i128)];
_21.0.2 = 18216122971147193041_u64;
_8 = 27370_u16 as u8;
(*_12) = (*_7);
Goto(bb4)
}
bb4 = {
_17 = _18;
_21.0.2 = 6612546456357166844_u64 - 2695850107085455755_u64;
_9 = _17 ^ _17;
_21.0 = (_10, _6, 17058268263175413685_u64, _21.1.2.0);
_21.2 = _15.0 as f32;
_21.0.1 = _1.0;
_21.0.3 = [(-73947755220529818517727828527026854884_i128),(-36723319109989950703978122345820155519_i128),85952719965684054733108446083816483038_i128,(-107259273432795761900212019393454514503_i128),162818455753242314477719280412889764494_i128,33239470803492760753310161698710781140_i128,12708891053983307477473007693025219928_i128];
_21.0 = (_10, _1.0, 3452867357146154327_u64, _21.1.2.0);
_3 = 87748357147196271854335599375168267769_i128 as usize;
_15.0 = _17 as i16;
_21.0 = (_8, _6, 10736025240386870545_u64, _21.1.2.0);
_20 = -_21.2;
_21.1.2.0 = [135952720760416057147721160545782096382_i128,169034579365433841079945419631611055237_i128,(-148285765677798137690674091679646974337_i128),22207422492409829137994241375168510633_i128,80582519291250488625328189834931961772_i128,(-64832761106616373586207350836228228936_i128),147469355689449176586908458043315950674_i128];
_11 = RET - RET;
_21.0.1 = (-37_i8) as i64;
_16 = [_15.1,(*_12),_5,_5,(*_7),(*_7),(*_7)];
_21.1.0 = !_3;
_22 = _11 - _11;
_23 = [_17,_18,_18,_17,_17,_9,_17,_17];
Call(RET = fn7(_21.0.0, _21.0.2, _21.0.1, _8, _21.0.2, _13, _21.0.2, _23, _21.0.2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_21.0 = (_10, _1.0, 15908805907414283389_u64, _21.1.2.0);
_21.1.2 = (_21.0.3, _15.1);
_24 = -_17;
_1 = (_21.0.1,);
_18 = _21.2 as isize;
_8 = 1463944651_i32 as u8;
_12 = core::ptr::addr_of!((*_12));
_18 = -_9;
_26 = [_3,_3,_21.1.0,_3,_21.1.0,_21.1.0,_21.1.0];
_2 = (*_7);
_11 = -RET;
_16 = [_2,(*_7),_5,(*_12),(*_7),(*_12),(*_12)];
_16 = [_2,(*_7),(*_12),_2,_5,_5,(*_12)];
_21.0.1 = -_6;
_30 = _16;
_21.1.2 = (_21.0.3, _2);
_21.0.1 = -_1.0;
Goto(bb6)
}
bb6 = {
Call(_31 = dump_var(1_usize, 24_usize, Move(_24), 8_usize, Move(_8), 1_usize, Move(_1), 30_usize, Move(_30)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_31 = dump_var(1_usize, 3_usize, Move(_3), 15_usize, Move(_15), 23_usize, Move(_23), 13_usize, Move(_13)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_31 = dump_var(1_usize, 18_usize, Move(_18), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: f64) -> [i16; 6] {
mir! {
type RET = [i16; 6];
let _3: isize;
let _4: f32;
let _5: isize;
let _6: isize;
let _7: (i64,);
let _8: isize;
let _9: *const bool;
let _10: bool;
let _11: f64;
let _12: u32;
let _13: usize;
let _14: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32);
let _15: f32;
let _16: isize;
let _17: bool;
let _18: bool;
let _19: char;
let _20: [i32; 8];
let _21: f64;
let _22: isize;
let _23: char;
let _24: isize;
let _25: isize;
let _26: i32;
let _27: bool;
let _28: u16;
let _29: f64;
let _30: ();
let _31: ();
{
_2 = 45_i8 as f64;
RET = [23548_i16,24702_i16,(-5512_i16),(-17224_i16),(-16963_i16),(-21794_i16)];
Call(_2 = fn3(RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [14289_i16,(-25960_i16),(-564_i16),(-3112_i16),(-31892_i16),(-29942_i16)];
_3 = 9223372036854775807_isize;
_1 = _2 >= _2;
_4 = 33284_u16 as f32;
_3 = (-9223372036854775808_isize) + (-29_isize);
RET = [20969_i16,(-20999_i16),7742_i16,12344_i16,4028_i16,25626_i16];
_4 = _3 as f32;
_3 = 513052425171571166_u64 as isize;
_7 = ((-1825230656221976093_i64),);
_2 = 5950916786270277181_usize as f64;
_7 = (3470658077891716030_i64,);
_8 = 93_i8 as isize;
_6 = !_8;
RET = [15882_i16,(-20609_i16),(-14761_i16),22655_i16,25977_i16,(-1706_i16)];
Goto(bb2)
}
bb2 = {
_6 = _8;
_7.0 = 252536084865505299687220718883001812272_u128 as i64;
_4 = 1494132393_u32 as f32;
_11 = 42_u8 as f64;
_9 = core::ptr::addr_of!(_1);
_7.0 = 5554947097189361517_i64;
RET = [9419_i16,8623_i16,29102_i16,(-31471_i16),27472_i16,32219_i16];
_9 = core::ptr::addr_of!((*_9));
_7.0 = 32174_u16 as i64;
_13 = 9367160866435180579_usize << _6;
_7.0 = 26047_i16 as i64;
_9 = core::ptr::addr_of!(_1);
_9 = core::ptr::addr_of!((*_9));
_7.0 = 4170505624139126731_i64;
_9 = core::ptr::addr_of!((*_9));
_6 = _8 ^ _3;
_13 = 2739382827729581939_usize;
Call((*_9) = fn4(_3, RET, _6, _9, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = 1451415443_i32 as f32;
_11 = _2;
_7.0 = 1001393434491738355_i64;
_11 = -_2;
_9 = core::ptr::addr_of!(_1);
(*_9) = !false;
Goto(bb4)
}
bb4 = {
_10 = _13 >= _13;
_7.0 = !3373195533523018634_i64;
_2 = _11 * _11;
_14.1.2.1 = _8 != _6;
_12 = 544687422_u32;
_14.1.2.1 = !_10;
_11 = 15405184166138752280_u64 as f64;
RET = [9150_i16,(-3971_i16),(-29632_i16),21596_i16,17768_i16,(-26114_i16)];
_14.0.2 = 6723276844023224983_u64 | 7775052317599646701_u64;
RET = [31538_i16,(-5271_i16),13918_i16,20908_i16,12025_i16,21117_i16];
_14.0.3 = [(-113015927738976134511946041842831632043_i128),(-165932026119878133061014781052456647835_i128),26284487134975597718057761100926661876_i128,(-49722731440766565518998774457864881704_i128),(-102851777605731948003068545189847710955_i128),32118317224238970186245939538291230720_i128,161228926857302757577278710166158515824_i128];
(*_9) = !_10;
_6 = _4 as isize;
_14.0.1 = _7.0 >> _14.0.2;
_5 = _3 >> _7.0;
_16 = !_3;
_2 = _11;
_4 = (-32296_i16) as f32;
match _12 {
0 => bb1,
1 => bb3,
2 => bb5,
544687422 => bb7,
_ => bb6
}
}
bb5 = {
_4 = 1451415443_i32 as f32;
_11 = _2;
_7.0 = 1001393434491738355_i64;
_11 = -_2;
_9 = core::ptr::addr_of!(_1);
(*_9) = !false;
Goto(bb4)
}
bb6 = {
RET = [14289_i16,(-25960_i16),(-564_i16),(-3112_i16),(-31892_i16),(-29942_i16)];
_3 = 9223372036854775807_isize;
_1 = _2 >= _2;
_4 = 33284_u16 as f32;
_3 = (-9223372036854775808_isize) + (-29_isize);
RET = [20969_i16,(-20999_i16),7742_i16,12344_i16,4028_i16,25626_i16];
_4 = _3 as f32;
_3 = 513052425171571166_u64 as isize;
_7 = ((-1825230656221976093_i64),);
_2 = 5950916786270277181_usize as f64;
_7 = (3470658077891716030_i64,);
_8 = 93_i8 as isize;
_6 = !_8;
RET = [15882_i16,(-20609_i16),(-14761_i16),22655_i16,25977_i16,(-1706_i16)];
Goto(bb2)
}
bb7 = {
_14.0.1 = _7.0;
_14.1.0 = _13;
_14.1.2.0 = _14.0.3;
_14.1.1 = 1454027406_i32 as u64;
_14.0 = (214_u8, _7.0, _14.1.1, _14.1.2.0);
_14.0.2 = _14.1.1;
_11 = -_2;
match _14.0.0 {
0 => bb8,
214 => bb10,
_ => bb9
}
}
bb8 = {
_6 = _8;
_7.0 = 252536084865505299687220718883001812272_u128 as i64;
_4 = 1494132393_u32 as f32;
_11 = 42_u8 as f64;
_9 = core::ptr::addr_of!(_1);
_7.0 = 5554947097189361517_i64;
RET = [9419_i16,8623_i16,29102_i16,(-31471_i16),27472_i16,32219_i16];
_9 = core::ptr::addr_of!((*_9));
_7.0 = 32174_u16 as i64;
_13 = 9367160866435180579_usize << _6;
_7.0 = 26047_i16 as i64;
_9 = core::ptr::addr_of!(_1);
_9 = core::ptr::addr_of!((*_9));
_7.0 = 4170505624139126731_i64;
_9 = core::ptr::addr_of!((*_9));
_6 = _8 ^ _3;
_13 = 2739382827729581939_usize;
Call((*_9) = fn4(_3, RET, _6, _9, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_4 = 1451415443_i32 as f32;
_11 = _2;
_7.0 = 1001393434491738355_i64;
_11 = -_2;
_9 = core::ptr::addr_of!(_1);
(*_9) = !false;
Goto(bb4)
}
bb10 = {
_13 = _14.1.0;
_6 = _5;
_7 = (_14.0.1,);
_14.1.2.1 = !_10;
_19 = '\u{5a2c2}';
_14.0.1 = !_7.0;
_23 = _19;
_8 = _16;
_14.1.2.1 = _10;
(*_9) = !_10;
_14.0.1 = 17739_u16 as i64;
_21 = -_2;
_9 = core::ptr::addr_of!(_18);
_14.0 = (20_u8, _7.0, _14.1.1, _14.1.2.0);
_14.0.0 = 88_u8;
_9 = core::ptr::addr_of!((*_9));
_1 = _10;
_22 = 52692_u16 as isize;
(*_9) = !_10;
Goto(bb11)
}
bb11 = {
RET = [(-11357_i16),26182_i16,6402_i16,(-7894_i16),7038_i16,(-24922_i16)];
_7.0 = !_14.0.1;
_13 = 37305266741689253091039725777695681071_i128 as usize;
_4 = _14.0.1 as f32;
_18 = _13 != _13;
RET = [17958_i16,16746_i16,6252_i16,31337_i16,(-11965_i16),(-28703_i16)];
match _14.0.0 {
0 => bb8,
1 => bb12,
2 => bb13,
3 => bb14,
88 => bb16,
_ => bb15
}
}
bb12 = {
_6 = _8;
_7.0 = 252536084865505299687220718883001812272_u128 as i64;
_4 = 1494132393_u32 as f32;
_11 = 42_u8 as f64;
_9 = core::ptr::addr_of!(_1);
_7.0 = 5554947097189361517_i64;
RET = [9419_i16,8623_i16,29102_i16,(-31471_i16),27472_i16,32219_i16];
_9 = core::ptr::addr_of!((*_9));
_7.0 = 32174_u16 as i64;
_13 = 9367160866435180579_usize << _6;
_7.0 = 26047_i16 as i64;
_9 = core::ptr::addr_of!(_1);
_9 = core::ptr::addr_of!((*_9));
_7.0 = 4170505624139126731_i64;
_9 = core::ptr::addr_of!((*_9));
_6 = _8 ^ _3;
_13 = 2739382827729581939_usize;
Call((*_9) = fn4(_3, RET, _6, _9, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
RET = [14289_i16,(-25960_i16),(-564_i16),(-3112_i16),(-31892_i16),(-29942_i16)];
_3 = 9223372036854775807_isize;
_1 = _2 >= _2;
_4 = 33284_u16 as f32;
_3 = (-9223372036854775808_isize) + (-29_isize);
RET = [20969_i16,(-20999_i16),7742_i16,12344_i16,4028_i16,25626_i16];
_4 = _3 as f32;
_3 = 513052425171571166_u64 as isize;
_7 = ((-1825230656221976093_i64),);
_2 = 5950916786270277181_usize as f64;
_7 = (3470658077891716030_i64,);
_8 = 93_i8 as isize;
_6 = !_8;
RET = [15882_i16,(-20609_i16),(-14761_i16),22655_i16,25977_i16,(-1706_i16)];
Goto(bb2)
}
bb14 = {
_6 = _8;
_7.0 = 252536084865505299687220718883001812272_u128 as i64;
_4 = 1494132393_u32 as f32;
_11 = 42_u8 as f64;
_9 = core::ptr::addr_of!(_1);
_7.0 = 5554947097189361517_i64;
RET = [9419_i16,8623_i16,29102_i16,(-31471_i16),27472_i16,32219_i16];
_9 = core::ptr::addr_of!((*_9));
_7.0 = 32174_u16 as i64;
_13 = 9367160866435180579_usize << _6;
_7.0 = 26047_i16 as i64;
_9 = core::ptr::addr_of!(_1);
_9 = core::ptr::addr_of!((*_9));
_7.0 = 4170505624139126731_i64;
_9 = core::ptr::addr_of!((*_9));
_6 = _8 ^ _3;
_13 = 2739382827729581939_usize;
Call((*_9) = fn4(_3, RET, _6, _9, _9, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_4 = 1451415443_i32 as f32;
_11 = _2;
_7.0 = 1001393434491738355_i64;
_11 = -_2;
_9 = core::ptr::addr_of!(_1);
(*_9) = !false;
Goto(bb4)
}
bb16 = {
_14.1.2.0 = [(-13807817571424570960042731646346727330_i128),(-110134515120191664822707424321442579981_i128),(-79377124774835317099619541291853458528_i128),113507817999069656832568871668623563360_i128,(-145586003206370590216953225192418943924_i128),(-1863769849034694372514144869119158404_i128),(-154983290737028359309903270721472989298_i128)];
_14.0.1 = _7.0;
_26 = 876863166_i32;
(*_9) = _1;
_14.2 = _14.0.0 as f32;
_17 = _1;
RET = [21567_i16,25597_i16,16309_i16,(-21012_i16),(-31650_i16),23519_i16];
_12 = 2193_u16 as u32;
_14.0.2 = _14.1.1 * _14.1.1;
_16 = _5;
_16 = _6 & _5;
_1 = !_17;
_14.0.3 = [77639490183854039565284739912067198218_i128,(-126380970057653240328680546238559741955_i128),106762625138714153413857010518016851447_i128,(-140807376892964354150601623564037023152_i128),108372461424091673635835515206021038174_i128,39037896818981377088332712301888715924_i128,(-165591844048834059150019370053546850002_i128)];
_20 = [_26,_26,_26,_26,_26,_26,_26,_26];
_27 = _14.0.2 != _14.0.2;
_14.1.2.1 = !_18;
_5 = !_16;
_1 = _27;
_12 = 3212526914_u32;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(2_usize, 13_usize, Move(_13), 18_usize, Move(_18), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(2_usize, 27_usize, Move(_27), 5_usize, Move(_5), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(2_usize, 3_usize, Move(_3), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [i16; 6],mut _2: [i16; 6]) -> f64 {
mir! {
type RET = f64;
let _3: Adt58;
let _4: u32;
let _5: f64;
let _6: char;
let _7: isize;
let _8: (u8, i64, u64, [i128; 7]);
let _9: [u8; 1];
let _10: Adt44;
let _11: isize;
let _12: u32;
let _13: ();
let _14: ();
{
_1 = [19425_i16,(-30924_i16),19491_i16,(-14628_i16),26657_i16,7506_i16];
_2 = [10329_i16,9042_i16,9712_i16,31570_i16,(-7444_i16),(-28657_i16)];
RET = 21170_u16 as f64;
_3.fld0.1 = true | false;
_3.fld4 = 21664_i16 * (-2044_i16);
_3.fld3 = (RET, 613161476_i32);
_3.fld3.1 = (-2120719154_i32);
RET = _3.fld3.0 + _3.fld3.0;
_3.fld2 = [185971879349681558496233278657598165784_u128,77952654355642950402568603352950833238_u128,74058336229166239746225364581023992311_u128,23537296336610441062083487181414101380_u128,171058793216329707227911011825480106903_u128,280767632991375302702412810946808891307_u128];
_3.fld2 = [296505914430708062554297336843595253406_u128,60048199779861259313273090609857135706_u128,33054173317946020235844132813034252174_u128,247754478932061040186306319816731989170_u128,98661641668106130622504229907184170912_u128,128386079329099178389045496764577056585_u128];
_3.fld1 = [_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1];
_3.fld0 = Checked(14385937092912417439_usize + 17923029804846158128_usize);
RET = _3.fld3.0;
_3.fld1 = [_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1];
_3.fld6 = (-2316331294748585342_i64) - (-3109446937677220323_i64);
_2 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_3.fld0 = (18129516852820044836_usize, true);
_4 = !192915077_u32;
_3.fld3.0 = RET * RET;
match _3.fld0.0 {
0 => bb1,
1 => bb2,
18129516852820044836 => bb4,
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
_3.fld2 = [300503871799785975987035868308721126784_u128,60637677885732787854714249019805639125_u128,124276116559307624662826505436894996579_u128,331253671547783809138949421705835170770_u128,105373785454564539643430742211616819284_u128,39073819601684545898837984510020140242_u128];
_3.fld2 = [258998085474352615709506730048453020188_u128,138695312398776759959614667703296349247_u128,174438583901027382674853448541955301673_u128,150442826325840298489825132419784125334_u128,164226435061429333659101944544998646946_u128,164585473633454652011578979919053865332_u128];
_4 = 1813486180_u32 - 2759975048_u32;
_3.fld3 = (RET, (-1198524556_i32));
RET = 9444057815126307969255880193384110458_i128 as f64;
_1 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_4 = 3207429861_u32;
_3.fld3.0 = -RET;
Goto(bb5)
}
bb5 = {
_3.fld1 = [_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1];
RET = _3.fld3.0;
_3.fld2 = [14631560781774486372030170027277156290_u128,332702249760485410229319664364358517156_u128,314413624799210048674652519793145324363_u128,180055520037632946609297522560079718031_u128,320966377170311645255131905686499269124_u128,251240091452373784770642868548021788300_u128];
_3.fld3 = (RET, 1113868505_i32);
_5 = _3.fld3.0;
_1 = _2;
_3.fld3.0 = -_5;
_3.fld0 = Checked(5189425854706635106_usize + 3_usize);
_5 = 225_u8 as f64;
_2 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_3.fld2 = [41593390315898193712788631993494762161_u128,52825809700024819110281884503656365704_u128,53919151341526352605120373224987495649_u128,52233747967505451582757158637777621583_u128,236963925230348851067999740866424280412_u128,169129248164591398669897330197388494656_u128];
_3.fld2 = [323209615021920221983687420226846134268_u128,267684876917090277250838886843185021855_u128,223162754777784311587435667259353512567_u128,231776780232642468999245376227549306737_u128,177495742780188175154578625759190780408_u128,176334023879658660695080479546307687550_u128];
_3.fld3.0 = RET + RET;
_1 = _2;
_3.fld0 = (1661643359227690753_usize, false);
_6 = '\u{d640}';
_3.fld6 = (-4344405681435238047_i64) << _3.fld4;
_3.fld3 = (RET, (-799199187_i32));
_3.fld3.1 = 204508374_i32 & (-332417755_i32);
match _3.fld0.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
1661643359227690753 => bb11,
_ => bb10
}
}
bb6 = {
_3.fld2 = [300503871799785975987035868308721126784_u128,60637677885732787854714249019805639125_u128,124276116559307624662826505436894996579_u128,331253671547783809138949421705835170770_u128,105373785454564539643430742211616819284_u128,39073819601684545898837984510020140242_u128];
_3.fld2 = [258998085474352615709506730048453020188_u128,138695312398776759959614667703296349247_u128,174438583901027382674853448541955301673_u128,150442826325840298489825132419784125334_u128,164226435061429333659101944544998646946_u128,164585473633454652011578979919053865332_u128];
_4 = 1813486180_u32 - 2759975048_u32;
_3.fld3 = (RET, (-1198524556_i32));
RET = 9444057815126307969255880193384110458_i128 as f64;
_1 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_4 = 3207429861_u32;
_3.fld3.0 = -RET;
Goto(bb5)
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
_6 = '\u{e2ca1}';
match _4 {
0 => bb7,
1 => bb12,
2 => bb13,
3207429861 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_3.fld2 = [300503871799785975987035868308721126784_u128,60637677885732787854714249019805639125_u128,124276116559307624662826505436894996579_u128,331253671547783809138949421705835170770_u128,105373785454564539643430742211616819284_u128,39073819601684545898837984510020140242_u128];
_3.fld2 = [258998085474352615709506730048453020188_u128,138695312398776759959614667703296349247_u128,174438583901027382674853448541955301673_u128,150442826325840298489825132419784125334_u128,164226435061429333659101944544998646946_u128,164585473633454652011578979919053865332_u128];
_4 = 1813486180_u32 - 2759975048_u32;
_3.fld3 = (RET, (-1198524556_i32));
RET = 9444057815126307969255880193384110458_i128 as f64;
_1 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_4 = 3207429861_u32;
_3.fld3.0 = -RET;
Goto(bb5)
}
bb15 = {
_2 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_2 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_3.fld0.0 = 3_usize;
_3.fld4 = (-8792_i16);
_8.0 = !159_u8;
_4 = !1736590496_u32;
_3.fld3.0 = RET - _5;
RET = -_3.fld3.0;
_3.fld1 = [_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1,_3.fld0.1];
_8.2 = !4974103278452837194_u64;
_1 = [_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4,_3.fld4];
_8.2 = 8165589749714470144_u64;
_3.fld0.0 = 3_usize << _3.fld6;
Goto(bb16)
}
bb16 = {
Call(_13 = dump_var(3_usize, 2_usize, Move(_2), 1_usize, Move(_1), 14_usize, _14, 14_usize, _14), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: [i16; 6],mut _3: isize,mut _4: *const bool,mut _5: *const bool,mut _6: [i16; 6]) -> bool {
mir! {
type RET = bool;
let _7: Adt45;
let _8: i128;
let _9: *mut ([i128; 7], bool);
let _10: isize;
let _11: Adt47;
let _12: (usize, bool);
let _13: f64;
let _14: u16;
let _15: isize;
let _16: f32;
let _17: i16;
let _18: ();
let _19: ();
{
_2 = _6;
_1 = '\u{84936}' as isize;
RET = false;
RET = true;
_5 = core::ptr::addr_of!(RET);
_5 = _4;
Call(_1 = fn5(_3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !false;
_6 = _2;
_2 = [(-17020_i16),(-4256_i16),(-27019_i16),(-3556_i16),26518_i16,(-15917_i16)];
_6 = [(-26376_i16),12997_i16,24025_i16,23408_i16,14528_i16,(-20135_i16)];
_3 = !_1;
RET = _3 != _1;
RET = _1 >= _1;
RET = _1 > _1;
RET = false;
RET = false;
_5 = _4;
_5 = core::ptr::addr_of!(RET);
_1 = _3 * _3;
_8 = -(-54600588030634527751979659045181681917_i128);
RET = _3 <= _1;
(*_5) = false;
_10 = _1 >> _3;
_6 = _2;
RET = !false;
RET = !true;
RET = false;
RET = !false;
_2 = [16806_i16,(-14026_i16),30169_i16,(-9143_i16),9471_i16,(-14453_i16)];
(*_5) = true;
_2 = [(-21344_i16),31351_i16,(-25312_i16),(-2687_i16),11076_i16,1601_i16];
Goto(bb2)
}
bb2 = {
Call(_3 = fn6(_10, _10, _1, _10, _10, _1, _10, _1, _10, _1, _1, _10, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_5) = !true;
RET = true;
_12.1 = (*_5);
_10 = _1;
Goto(bb4)
}
bb4 = {
_12.0 = 3_usize + 7425003710492294905_usize;
_8 = 51869243188096376549871358570518770326_i128 + 123207344773080738477060015574472345074_i128;
RET = _12.1;
_4 = core::ptr::addr_of!((*_5));
Goto(bb5)
}
bb5 = {
_3 = -_10;
RET = _10 == _3;
_8 = (-57611257380468981212068273976521944329_i128);
_13 = _1 as f64;
_13 = 157244504117548680631689875860332890288_u128 as f64;
_10 = '\u{86a1a}' as isize;
_1 = _3 & _3;
_10 = 58793_u16 as isize;
_13 = (-23_i8) as f64;
RET = !_12.1;
_1 = _10;
(*_4) = !_12.1;
RET = !_12.1;
_4 = core::ptr::addr_of!((*_5));
_10 = _3;
_3 = _10;
_6 = [(-13469_i16),21949_i16,27491_i16,(-15226_i16),(-29077_i16),(-10497_i16)];
_5 = core::ptr::addr_of!(_12.1);
_3 = _10 | _10;
_12.0 = 3_usize;
_8 = 57355119505577569597980047940113849007_i128 ^ (-136436914754442833270949592662211173185_i128);
(*_4) = !(*_5);
RET = !(*_5);
_10 = !_3;
_5 = _4;
_1 = _3 >> _3;
_6 = [(-12176_i16),27236_i16,24166_i16,(-19681_i16),(-10346_i16),(-4394_i16)];
Goto(bb6)
}
bb6 = {
(*_5) = _12.1;
_8 = 146026310658067336433954724684386325049_i128;
_14 = 2987_u16;
_13 = 7019409125316817032_i64 as f64;
_12.0 = (-1492898516_i32) as usize;
(*_5) = !_12.1;
_4 = _5;
_14 = 41857_u16 * 56584_u16;
_4 = _5;
RET = _3 >= _1;
_14 = 38732_u16;
_5 = core::ptr::addr_of!(RET);
_10 = !_3;
_16 = _12.0 as f32;
Goto(bb7)
}
bb7 = {
Call(_18 = dump_var(4_usize, 8_usize, Move(_8), 3_usize, Move(_3), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: (usize, bool);
let _4: f32;
let _5: char;
let _6: bool;
let _7: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]);
let _8: ();
let _9: ();
{
RET = !_1;
_1 = -RET;
_2 = _1 & RET;
_3 = Checked(381976246652158028_usize * 0_usize);
_3.0 = _2 as usize;
_1 = RET - RET;
RET = _1;
_3 = Checked(7_usize - 6_usize);
RET = _2 - _2;
_3 = (5_usize, true);
_3.1 = false;
_4 = _3.0 as f32;
_2 = _1 * RET;
_1 = 96920508752034153110744362844408234067_i128 as isize;
_3.0 = _4 as usize;
RET = 2039_i16 as isize;
_3.1 = !false;
_5 = '\u{5a4b0}';
Goto(bb1)
}
bb1 = {
_1 = -_2;
_5 = '\u{4658f}';
_4 = _3.0 as f32;
RET = _2;
RET = _1 << _2;
_3.1 = true;
_3.1 = !true;
_5 = '\u{f538c}';
_2 = _1;
RET = !_2;
_3 = Checked(4421453521983547210_usize + 12608469381982365653_usize);
_6 = RET <= RET;
RET = 50473469495569892275683479165063927559_u128 as isize;
_3.1 = _6 == _6;
RET = _2;
_7.1.0.3 = [(-35903237199877738859701267836257133260_i128),4199020613421147308477698546105951995_i128,86187639996540586805415627160594080875_i128,42823561503933912659982406172915851417_i128,(-135990152054275163738402164206090561677_i128),34541685559261182333338412436927721156_i128,(-72259779295852235590342492544710794202_i128)];
_7.1.0.2 = 21598539301806544615295788309725424633_u128 as u64;
_7.1.1.2.1 = _3.1 > _3.1;
_7.1.1.2 = (_7.1.0.3, _3.1);
_3 = (1904049353816147691_usize, _7.1.1.2.1);
_7.0 = [_3.1,_3.1,_7.1.1.2.1,_3.1,_7.1.1.2.1,_7.1.1.2.1,_6];
_7.4 = [251_u8];
_7.0 = [_3.1,_7.1.1.2.1,_7.1.1.2.1,_3.1,_3.1,_7.1.1.2.1,_3.1];
Goto(bb2)
}
bb2 = {
Call(_8 = dump_var(5_usize, 1_usize, Move(_1), 6_usize, Move(_6), 9_usize, _9, 9_usize, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: f32;
let _15: i64;
let _16: f32;
let _17: [i32; 8];
let _18: (i64,);
let _19: Adt55;
let _20: isize;
let _21: char;
let _22: bool;
let _23: ();
let _24: ();
{
_4 = 9180453567422721426_i64 as isize;
RET = 77_i8 as isize;
_12 = (-129786456204298940633441820734477369715_i128) as isize;
_8 = !_10;
RET = _7;
_5 = !_8;
_10 = _6 >> RET;
RET = 27744_u16 as isize;
_14 = (-125293124822497321259920359799454316583_i128) as f32;
_4 = !_2;
_12 = _4;
_9 = _7 + _8;
RET = -_3;
_13 = -_1;
_1 = _8 >> _12;
_15 = (-5993720960732330200_i64) | (-3901402721838963751_i64);
RET = 166533523469245338285272956967938836630_i128 as isize;
_5 = _8;
_6 = _12 - _2;
_10 = 125_u8 as isize;
_9 = 35_u8 as isize;
_1 = _5;
_5 = _6;
_4 = -_7;
_13 = _12 * _5;
_1 = _2;
Goto(bb1)
}
bb1 = {
_8 = !_13;
_9 = !_6;
_8 = 22_i8 as isize;
_14 = _15 as f32;
_9 = true as isize;
Goto(bb2)
}
bb2 = {
_9 = 2_usize as isize;
_6 = 59542_u16 as isize;
_11 = _4 - _1;
_4 = _13;
_17 = [1307823612_i32,(-1503915222_i32),1156607175_i32,(-861751582_i32),1557105936_i32,490874062_i32,(-25605552_i32),(-1501632912_i32)];
_17 = [(-2069013439_i32),(-256536671_i32),(-918927160_i32),869039206_i32,1714777403_i32,(-604118665_i32),(-213763203_i32),923305755_i32];
_15 = (-5996590774671421304_i64) - (-8333568967142696578_i64);
Goto(bb3)
}
bb3 = {
_11 = -_7;
_14 = 690804307237961395889375283472135252_i128 as f32;
_7 = !_1;
_7 = _13 + _3;
_21 = '\u{bf948}';
RET = !_3;
_15 = (-6073534081680128554_i64) & (-7831777140714446679_i64);
_16 = 1686011237_i32 as f32;
_12 = 18178_u16 as isize;
_11 = _5 ^ _4;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(6_usize, 13_usize, Move(_13), 3_usize, Move(_3), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(6_usize, 11_usize, Move(_11), 10_usize, Move(_10), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u8,mut _2: u64,mut _3: i64,mut _4: u8,mut _5: u64,mut _6: char,mut _7: u64,mut _8: [isize; 8],mut _9: u64) -> f64 {
mir! {
type RET = f64;
let _10: Adt45;
let _11: usize;
let _12: i16;
let _13: ();
let _14: ();
{
RET = 68_i8 as f64;
_6 = '\u{dfe78}';
_8 = [(-97_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-13_isize),76_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = _7 * _5;
Call(_2 = fn8(_9, _9, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-31058_i16) as f64;
_5 = false as u64;
_5 = !_2;
_7 = _2 & _2;
_2 = _4 as u64;
_5 = _7 / _9;
_8 = [(-12_isize),68_isize,9223372036854775807_isize,9223372036854775807_isize,16_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = _1 >> _5;
_2 = _7 | _5;
_6 = '\u{6cd59}';
_4 = _1 << _7;
_6 = '\u{41088}';
RET = 3_usize as f64;
RET = _4 as f64;
_2 = _5;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(7_usize, 6_usize, Move(_6), 1_usize, Move(_1), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: u64,mut _2: u64,mut _3: u64) -> u64 {
mir! {
type RET = u64;
let _4: (i16, bool);
let _5: char;
let _6: ([i128; 7], bool);
let _7: [u8; 1];
let _8: char;
let _9: Adt53;
let _10: isize;
let _11: ();
let _12: ();
{
RET = _1;
RET = !_3;
RET = _3 ^ _1;
_1 = !_2;
_3 = 518167550_i32 as u64;
_2 = !RET;
_1 = RET * RET;
RET = _1;
_2 = RET;
_4.0 = 92821479746317001652111042455829662618_i128 as i16;
_5 = '\u{4949f}';
_1 = RET << _2;
_1 = !RET;
RET = _1 + _2;
_4.1 = true & true;
_3 = !_1;
_3 = !_1;
RET = _4.1 as u64;
Goto(bb1)
}
bb1 = {
_4 = (24088_i16, true);
_3 = 158365699324686079411793813331580464465_i128 as u64;
RET = _2 >> _1;
_4.0 = (-11884_i16) & (-28768_i16);
_1 = RET * _2;
_2 = _1 - RET;
_5 = '\u{142a0}';
_6.1 = !_4.1;
_1 = RET | RET;
_6.0 = [82342110619168508634912122617673030562_i128,81028432652441487671658013694530221850_i128,(-80615652185132323374344486040195252623_i128),(-153905343267996373865796426689289064095_i128),45661357629644655805645359125485305172_i128,(-167894796743327405634999063259667403202_i128),(-131643230074318760564859570211137541582_i128)];
_5 = '\u{11b17}';
_3 = !RET;
_4.0 = (-11878_i16) * 11317_i16;
_1 = 46228_u16 as u64;
_7 = [67_u8];
_4.1 = _6.1 | _6.1;
_4.1 = _3 != _2;
_6.0 = [(-164401854410187874092103931742519403271_i128),(-88583528356431390571016888736685669683_i128),(-110276344642131127328245622892159844897_i128),23998380054072023292599922166475744403_i128,27660542714660523836433012184589490794_i128,(-111207250512292314636768183125438093024_i128),(-168201637404344879334022019290017849339_i128)];
_3 = _2 | _2;
_2 = !RET;
_3 = _2 & RET;
_6.1 = _4.1 != _4.1;
RET = _2;
RET = !_2;
_8 = _5;
_7 = [224_u8];
_1 = RET << RET;
Goto(bb2)
}
bb2 = {
Call(_11 = dump_var(8_usize, 1_usize, Move(_1), 3_usize, Move(_3), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: i128,mut _3: [i128; 7],mut _4: *const bool,mut _5: [i128; 7],mut _6: f64,mut _7: *const bool,mut _8: bool,mut _9: bool,mut _10: (f64, i32),mut _11: (f64, i32)) -> i16 {
mir! {
type RET = i16;
let _12: Adt48;
let _13: &'static f32;
let _14: u32;
let _15: Adt46;
let _16: (u8, i64, u64, [i128; 7]);
let _17: ();
let _18: ();
{
_3 = [_2,_2,_2,_2,_2,_2,_2];
_11 = _10;
Call(_4 = core::intrinsics::arith_offset(_7, 9223372036854775807_isize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_7) = !_8;
_9 = !_1;
_8 = !(*_7);
(*_7) = !_9;
_6 = _10.0;
RET = 1524974547_u32 as i16;
(*_7) = !_8;
_9 = (*_7) >= _8;
_11 = (_10.0, _10.1);
_7 = _4;
_11.0 = 9223372036854775807_isize as f64;
_9 = _8 >= _8;
_11.0 = -_6;
_5 = _3;
_11.1 = 125419518973346656993743299913388899690_u128 as i32;
RET = !28166_i16;
_11.0 = _10.0 * _6;
_9 = _8 ^ _8;
_8 = _9 ^ _9;
_11.1 = -_10.1;
Call(_10 = fn10(_4, _4, _7, _1, _4, _4, _9, _7, _4, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11.1 = _10.1 * _10.1;
_11.1 = -_10.1;
RET = !13262_i16;
_6 = -_11.0;
_10.1 = !_11.1;
_11 = (_6, _10.1);
_14 = !3670183956_u32;
_11.1 = -_10.1;
RET = (-31602_i16);
_6 = _10.0 * _11.0;
_2 = (-134607964243825500421602390624910111615_i128);
Goto(bb3)
}
bb3 = {
_15 = Adt46::Variant1 { fld0: _2,fld1: 1302572788903617422_usize,fld2: (-9223372036854775808_isize),fld3: _11.0 };
_11.1 = _10.1;
_16.2 = !17862817474972002542_u64;
RET = 21163_i16 + 13831_i16;
place!(Field::<isize>(Variant(_15, 1), 2)) = (-9223372036854775808_isize);
_16.2 = 12325745944509345578_u64 << RET;
_1 = _9;
place!(Field::<i128>(Variant(_15, 1), 0)) = Field::<f64>(Variant(_15, 1), 3) as i128;
_15 = Adt46::Variant0 { fld0: 1_usize };
_16.3 = _5;
_9 = _8 ^ _1;
_15 = Adt46::Variant1 { fld0: _2,fld1: 7176725034084095320_usize,fld2: 9223372036854775807_isize,fld3: _10.0 };
_16 = (91_u8, 8368268174524862224_i64, 3537992751841332680_u64, _3);
place!(Field::<i128>(Variant(_15, 1), 0)) = _2 - _2;
RET = 14196_i16 << _16.2;
_15 = Adt46::Variant0 { fld0: 4_usize };
_8 = _1;
_11.1 = -_10.1;
_16.2 = 15908186625346241478_u64 >> _16.1;
_16.3 = [_2,_2,_2,_2,_2,_2,_2];
_16.3 = [_2,_2,_2,_2,_2,_2,_2];
_10 = (_11.0, _11.1);
_10.0 = _11.0;
_11 = (_6, _10.1);
Goto(bb4)
}
bb4 = {
Call(_17 = dump_var(9_usize, 1_usize, Move(_1), 8_usize, Move(_8), 5_usize, Move(_5), 16_usize, Move(_16)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: bool,mut _5: *const bool,mut _6: *const bool,mut _7: bool,mut _8: *const bool,mut _9: *const bool,mut _10: [i128; 7]) -> (f64, i32) {
mir! {
type RET = (f64, i32);
let _11: i8;
let _12: f64;
let _13: [i16; 6];
let _14: [i32; 8];
let _15: Adt46;
let _16: ((f64, i32),);
let _17: Adt49;
let _18: u16;
let _19: Adt60;
let _20: f32;
let _21: ();
let _22: ();
{
RET.1 = 523322039_i32;
_4 = !_7;
RET.1 = (-2096620774_i32);
_7 = !_4;
RET.0 = (-143309756067652848527979522422812146050_i128) as f64;
RET.0 = 3203681291_u32 as f64;
_11 = 34_isize as i8;
RET.1 = 1451922295_i32;
_4 = _7;
RET.0 = (-74_isize) as f64;
_11 = -(-80_i8);
_4 = _7 != _7;
RET.0 = _11 as f64;
_7 = _4;
_11 = 118_i8 & (-87_i8);
RET.1 = 9223372036854775807_isize as i32;
RET.1 = (-2076121005_i32);
Goto(bb1)
}
bb1 = {
_7 = _4 > _4;
_13 = [6393_i16,26742_i16,31155_i16,(-11247_i16),(-31513_i16),21018_i16];
_13 = [(-30520_i16),16765_i16,19343_i16,(-26209_i16),15510_i16,(-20258_i16)];
_12 = RET.0;
_11 = (-71_i8) + (-122_i8);
RET.0 = -_12;
_14 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
RET = (_12, (-1124038896_i32));
RET.1 = 1822876873_i32 & 1007182318_i32;
RET = (_12, (-1570771073_i32));
_12 = RET.0 - RET.0;
RET = (_12, 513430207_i32);
_16.0.1 = RET.1;
RET = (_12, _16.0.1);
_16.0.1 = -RET.1;
_10 = [(-159198881748909514784043515348254093846_i128),134484192639964008914637685070362674059_i128,158787682747039844807282511278508201270_i128,(-156484326246329203407474455911180782516_i128),41359441639778400509646772433627553922_i128,50335238551596760565721992373405645356_i128,71799525224801264851214099128360632312_i128];
_11 = (-98_isize) as i8;
_16 = (RET,);
RET.0 = _12 + _12;
_7 = !_4;
_11 = !71_i8;
RET = (_12, _16.0.1);
Call(RET = fn11(_1, _9, _5, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.1 = 6_usize as i32;
_16.0.0 = RET.0 + RET.0;
_13 = [(-25618_i16),(-5112_i16),31550_i16,5557_i16,7297_i16,16004_i16];
RET.1 = 13470_u16 as i32;
_16.0.0 = RET.0 * RET.0;
_18 = 75137783969601269210354262014534070355_i128 as u16;
RET.0 = -_16.0.0;
_19 = Adt60::Variant0 { fld0: _18,fld1: '\u{a2c6d}' };
Goto(bb3)
}
bb3 = {
Call(_21 = dump_var(10_usize, 14_usize, Move(_14), 13_usize, Move(_13), 18_usize, Move(_18), 22_usize, _22), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool) -> (f64, i32) {
mir! {
type RET = (f64, i32);
let _5: i32;
let _6: *const bool;
let _7: (u8, i64, u64, [i128; 7]);
let _8: (usize, bool);
let _9: *const ([i128; 7], bool);
let _10: Adt51;
let _11: u64;
let _12: usize;
let _13: f32;
let _14: ();
let _15: ();
{
RET.1 = (-1367149555_i32) * 728878227_i32;
RET.1 = -834074900_i32;
RET.1 = 3292826243_u32 as i32;
RET.0 = (-9223372036854775808_isize) as f64;
RET.1 = !2109819020_i32;
RET.1 = 241423625208608179369519057605147017088_u128 as i32;
RET.1 = 130863613_i32;
RET.0 = RET.1 as f64;
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
130863613 => bb6,
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
RET.1 = (-195996112_i32) * 1305824255_i32;
RET.0 = (-938_i16) as f64;
RET.1 = 82091098991012116841394858961810350277_i128 as i32;
RET.1 = !(-494425856_i32);
RET.1 = (-1019607154_i32) | (-87158826_i32);
RET.1 = 909006912_i32 + (-1611866002_i32);
RET.0 = (-31788_i16) as f64;
RET.0 = RET.1 as f64;
RET.0 = 99_i8 as f64;
RET.1 = 44390964_i32 + 409975534_i32;
RET.1 = true as i32;
RET.0 = 61_i8 as f64;
RET.1 = -(-298842426_i32);
RET.0 = RET.1 as f64;
RET.1 = 6671481628464766037_i64 as i32;
RET.1 = true as i32;
RET.1 = 1091133397_i32;
RET.1 = -(-2050843824_i32);
_5 = RET.1;
RET.0 = 112311674805474857599777637854988624971_u128 as f64;
_5 = 8_u8 as i32;
RET.1 = !_5;
RET.0 = 252786942314134726509593330402698658451_u128 as f64;
_5 = -RET.1;
RET.0 = 8958965473730496533_u64 as f64;
RET.1 = _5;
RET.1 = -_5;
_6 = _2;
_5 = RET.1 & RET.1;
Call(RET = fn12(_2, _6, _3, _6, _3, _1, _1, _4, _3, _4, _2, _1, _1, _4, _2, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7.1 = 10443550827897444773_u64 as i64;
_8.0 = 1814600097453535805_usize;
_8.1 = false;
RET.0 = RET.1 as f64;
_8.1 = false;
RET.1 = -_5;
_7.1 = (-4699103277010480216_i64) ^ (-3576761400117028652_i64);
RET.1 = _5 << _7.1;
_8 = (4_usize, false);
_8.0 = 1479434193249299115_usize;
_7.2 = !3353271901493920212_u64;
_5 = RET.1 - RET.1;
_11 = _7.2;
_8 = Checked(6_usize - 12874958173505181351_usize);
_8 = (9120245190287046644_usize, true);
Goto(bb8)
}
bb8 = {
Call(_14 = dump_var(11_usize, 5_usize, Move(_5), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool,mut _8: *const bool,mut _9: *const bool,mut _10: *const bool,mut _11: *const bool,mut _12: *const bool,mut _13: *const bool,mut _14: *const bool,mut _15: *const bool,mut _16: *const bool) -> (f64, i32) {
mir! {
type RET = (f64, i32);
let _17: [i16; 6];
let _18: (([i128; 7], bool),);
let _19: Adt47;
let _20: Adt52;
let _21: ();
let _22: ();
{
RET.1 = 66_u8 as i32;
RET.0 = 2073118800_u32 as f64;
_18.0.0 = [(-116458984021749762531966501845261307227_i128),(-75688406767389320632601205146116964076_i128),(-40907258969021162301387795727611151997_i128),120402449023702584449079037518464275949_i128,31528962304595725979631416228749910565_i128,(-51948303308009898967484145543193042135_i128),7691397394829386058553735197749845857_i128];
_17 = [(-26222_i16),23108_i16,3068_i16,(-20249_i16),(-5293_i16),8976_i16];
RET.0 = 225_u8 as f64;
_18.0.1 = false;
RET.1 = (-1554040524_i32) ^ (-1094626631_i32);
_18.0.0 = [(-57942316075912616395774048442657454848_i128),139400215339406802274852658098846206392_i128,(-122822776582831559293594231507468568568_i128),160214548418367720474750588522412663121_i128,3764681052977280675740036746984678001_i128,(-153732547548100743841142199373475658466_i128),95787775759249726340761718268587627955_i128];
RET.1 = (-5576491115405434410_i64) as i32;
RET.0 = 38569_u16 as f64;
RET.0 = 16497843003238485336_u64 as f64;
_17 = [(-31145_i16),20324_i16,(-32469_i16),20307_i16,25492_i16,(-5716_i16)];
RET.0 = 14579355308913253727_usize as f64;
Call(_20 = fn13(_5, _12, _14, _12, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0 = -Field::<(f64, i32)>(Variant(_20, 1), 5).0;
RET = (Field::<(f64, i32)>(Variant(_20, 1), 5).0, Field::<(f64, i32)>(Variant(_20, 1), 5).1);
place!(Field::<[usize; 7]>(Variant(_20, 1), 0)) = [7_usize,5_usize,7211242952369276846_usize,0_usize,11416653731910089060_usize,18062825589472594583_usize,10482548379673577902_usize];
SetDiscriminant(_20, 0);
place!(Field::<[u8; 1]>(Variant(_20, 0), 2)) = [19_u8];
place!(Field::<([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1])>(Variant(_20, 0), 0)).1.2 = (-2322079232724681293_i64) as f32;
place!(Field::<[u8; 1]>(Variant(_20, 0), 2)) = [39_u8];
RET.0 = 3302183450_u32 as f64;
place!(Field::<([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1])>(Variant(_20, 0), 0)).2 = (-15804_i16);
Goto(bb2)
}
bb2 = {
Call(_21 = dump_var(12_usize, 18_usize, Move(_18), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool,mut _5: *const bool) -> Adt52 {
mir! {
type RET = Adt52;
let _6: *mut ([i128; 7], bool);
let _7: Adt52;
let _8: *const bool;
let _9: *mut (u32, f64, i8, f64, bool);
let _10: [usize; 7];
let _11: Adt50;
let _12: isize;
let _13: [u128; 6];
let _14: isize;
let _15: isize;
let _16: Adt54;
let _17: isize;
let _18: u64;
let _19: [i128; 7];
let _20: (usize, bool);
let _21: char;
let _22: Adt48;
let _23: Adt57;
let _24: Adt52;
let _25: Adt58;
let _26: f32;
let _27: *const ([i128; 7], bool);
let _28: isize;
let _29: i32;
let _30: Adt48;
let _31: Adt58;
let _32: *mut (f64, i32);
let _33: Adt52;
let _34: [isize; 8];
let _35: f64;
let _36: isize;
let _37: ([i128; 7], bool);
let _38: *const ([i128; 7], bool);
let _39: Adt55;
let _40: u32;
let _41: char;
let _42: f32;
let _43: Adt59;
let _44: [i16; 6];
let _45: (i64,);
let _46: bool;
let _47: Adt50;
let _48: u16;
let _49: f32;
let _50: f64;
let _51: char;
let _52: ();
let _53: ();
{
Goto(bb1)
}
bb1 = {
_8 = _2;
_10 = [5_usize,8493615577004864277_usize,17770807659748711185_usize,12777795120673065880_usize,5037781316571381353_usize,13918284531747819656_usize,2_usize];
_10 = [3_usize,12168321795176936558_usize,4_usize,2_usize,0_usize,4_usize,918231766327946659_usize];
_10 = [1245870416252564576_usize,1_usize,2_usize,6892521688994581649_usize,2_usize,0_usize,4721713325284991816_usize];
_10 = [715751992334372280_usize,6037591400118201616_usize,16850140972499730296_usize,6295142135781308889_usize,5_usize,5268174757436239834_usize,5_usize];
_10 = [4812137745342509542_usize,4662438090767668684_usize,2752185000265110821_usize,12413323355461819092_usize,7_usize,13394127357626912003_usize,8050899027749561111_usize];
_10 = [2_usize,2_usize,8763081585487203649_usize,15426577515392305441_usize,2_usize,7283853219225614637_usize,0_usize];
_10 = [5_usize,6512508521204334388_usize,4914951218169148810_usize,6_usize,16109407012445685808_usize,4_usize,6823398099786539680_usize];
_10 = [9442232468286547844_usize,16281504014836892037_usize,942675020552238237_usize,2_usize,4_usize,5618856142816988576_usize,8920040450544161668_usize];
_10 = [1180108838606224661_usize,801589109072292556_usize,3371107372924859701_usize,2_usize,1_usize,5_usize,6_usize];
_10 = [1_usize,1_usize,4_usize,6797315946426276825_usize,4_usize,15812007396039558636_usize,4102289790897409100_usize];
Goto(bb2)
}
bb2 = {
_10 = [7_usize,3_usize,6_usize,0_usize,13098791322665902223_usize,5_usize,12666872090267429691_usize];
_10 = [4962278296446390328_usize,12394741004609658597_usize,14075556865474386585_usize,17235996616123038173_usize,10590714303553589597_usize,6_usize,6922127174876612959_usize];
_10 = [1_usize,13726156783159630379_usize,3_usize,727971959078968397_usize,7822016093338963809_usize,3_usize,16826364882231636840_usize];
_10 = [1_usize,1_usize,4211526193666927769_usize,6_usize,6_usize,5_usize,1600869161199212658_usize];
_10 = [13323866559996767509_usize,6_usize,5_usize,15099591685758360695_usize,15880478679785243249_usize,18312681055230137322_usize,0_usize];
_10 = [5_usize,6_usize,6_usize,2_usize,5_usize,5184550180505285054_usize,7_usize];
_10 = [9023013455081155331_usize,7_usize,3_usize,6_usize,4314700719546362216_usize,12999707287239050117_usize,3219912742080114797_usize];
_10 = [8262682445487645874_usize,17701673012020863488_usize,6_usize,3_usize,8422059063181640610_usize,11960810359378038502_usize,3588309924722496205_usize];
_10 = [5_usize,5_usize,13712948415478032548_usize,17201871111486275569_usize,1_usize,9897129356988463956_usize,17407525820116070121_usize];
_10 = [3801854339307246979_usize,3_usize,0_usize,16795792918408899101_usize,12866358016209651459_usize,5945682156018878274_usize,3_usize];
Goto(bb3)
}
bb3 = {
_10 = [4_usize,13424044111627953526_usize,5_usize,10464003744763759858_usize,7_usize,2_usize,4_usize];
_10 = [3_usize,6_usize,0_usize,5677971127379944833_usize,4_usize,2474251655380431258_usize,1_usize];
_10 = [9480006723474114697_usize,8449293492279349718_usize,7474552722086342109_usize,14641527099038101288_usize,0_usize,9537722834792645163_usize,13853490981642836245_usize];
_10 = [17157780894467660345_usize,8109704685577053229_usize,4929904629157319718_usize,8867270458063101993_usize,2_usize,3_usize,3_usize];
_10 = [7_usize,3_usize,5040494740449307974_usize,14280705713165198214_usize,17084374703030665150_usize,2_usize,2235932663074417566_usize];
_12 = -(-9223372036854775808_isize);
_12 = 15275949995864744808_u64 as isize;
_10 = [6_usize,791235998430175675_usize,7_usize,6_usize,16951168299164839321_usize,3_usize,2_usize];
_10 = [6_usize,16099782621346856715_usize,6399251530056558563_usize,6379407436101318188_usize,2387616438052505670_usize,17982741234346368561_usize,132849793439181612_usize];
_10 = [0_usize,1613588896855941824_usize,16363540389418474518_usize,10347175139958590662_usize,1_usize,2_usize,5_usize];
_10 = [2_usize,0_usize,77466325450381608_usize,2587271746542685929_usize,5252501736349796954_usize,7_usize,5_usize];
_10 = [2276970966710277727_usize,4_usize,13146572591685345344_usize,5_usize,6719772820575954586_usize,17404575380377024376_usize,11788429294786496085_usize];
_12 = (-30_isize);
_13 = [62295395212508818501737063446495871473_u128,55579615384673986654992199777467372421_u128,72763649718699728547386644049367209082_u128,262259536948050236292515204959274510533_u128,61359845652096649591053991643876243353_u128,205734941258300172272488627055690704214_u128];
_12 = 5954186517440982095_u64 as isize;
Goto(bb4)
}
bb4 = {
_10 = [1_usize,9251526609560089377_usize,2514349613564929330_usize,3_usize,4_usize,13034537024750927614_usize,0_usize];
_13 = [196180044438320584304620839728481626896_u128,29870611106808262102256021255703564475_u128,238706761422778212100969377356615944045_u128,244034018626909133091464614012176680478_u128,192683705558748912365850588230213205276_u128,191932626932519525015673108660171378318_u128];
_10 = [6_usize,8484093672748948124_usize,7058519901953588088_usize,6_usize,15044617510915490978_usize,10158943482674259097_usize,1_usize];
_13 = [171308949106231459336974133956570722862_u128,299977130285945364744493746008537171311_u128,98595295823025472003584347241039494292_u128,33757128178968994494821226566042645149_u128,42660964570578082523190091151836621594_u128,65874584028846165663646596873286104491_u128];
_10 = [7_usize,12334578319568129493_usize,5_usize,7_usize,6_usize,4_usize,2450332555708290542_usize];
_13 = [82667963252135145322333005824202095210_u128,218933571796350642794060504109336570661_u128,274745335203595474520142628766869141878_u128,215977547523298511012486475269967303620_u128,251665722843643700623928171697766405298_u128,230589764908843969210991077174280600804_u128];
Goto(bb5)
}
bb5 = {
_12 = 9223372036854775807_isize * (-9223372036854775808_isize);
_13 = [258929351661437511417421595938199332794_u128,34200025090294696207788634213224801922_u128,2374817728370563824422295051939625231_u128,296291619114929633949990073072383879097_u128,67180820414460086936646426065182621656_u128,253826781902678015324077104744947854074_u128];
Goto(bb6)
}
bb6 = {
_10 = [8377037998071107451_usize,4724664500251881349_usize,6_usize,4_usize,6_usize,17359207718834452453_usize,7_usize];
_12 = (-18933_i16) as isize;
_13 = [297979107452238603510155258263545709074_u128,145596609461316893957098164316678798212_u128,205195605659108941937085695608015361866_u128,93875068497417822724820223424419129415_u128,61353765004602677355066417721180037554_u128,171952564068574762361278726561485795449_u128];
_12 = 3923596055_u32 as isize;
_14 = (-4936585542095333303_i64) as isize;
_10 = [2911044653595116620_usize,6_usize,13402756371645335667_usize,17628192476512283120_usize,7_usize,1_usize,3_usize];
_10 = [12777975745335066041_usize,0_usize,4_usize,1_usize,0_usize,0_usize,7_usize];
_10 = [3_usize,14871951988706197519_usize,1_usize,4026036545791236206_usize,12994164212005801914_usize,2_usize,1765072717385597047_usize];
_14 = _12;
_12 = _14;
_15 = '\u{1c50b}' as isize;
Call(_9 = fn14(_2, _5, _3, _1, _4, _3, _3, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_17 = !_15;
_12 = _15 - _14;
_10 = [11888174170923356868_usize,1_usize,14577656492534766309_usize,4832029279760596252_usize,1031155643768089458_usize,15932330376410464853_usize,13129422630995562983_usize];
_13 = [297396499570963211597859450145574287104_u128,124864310163676582500514679999113275446_u128,277794553303852872453898595094272017006_u128,103329476346581199989175834975570551039_u128,237354150340434001309462472801569546491_u128,49762583135014505398062043414805025292_u128];
_12 = -_14;
_13 = [112626316647562446970390830023297283877_u128,303060675864839543262249591005979049444_u128,207105576020364595537969959251154542644_u128,114562728295692847529939959724496364204_u128,48945262625494729096089865968148764711_u128,40159116561664073671221818322119788206_u128];
_10 = [5_usize,4_usize,12491701983208487742_usize,1_usize,6_usize,3198125069733596804_usize,5_usize];
_15 = _14;
Goto(bb8)
}
bb8 = {
_19 = [(-148007612692928976679001032442448831841_i128),(-32880528540305975743692162248772692828_i128),(-23831058226982661016059517709473228877_i128),7647290151593931654360712781083888800_i128,144511586603946802253639119681004514439_i128,13598785184911563713012392057112915683_i128,(-104617185895607753498304259346829754650_i128)];
_14 = 40673_u16 as isize;
_20.0 = 2_usize * 7_usize;
_19 = [(-142118465603601076789127712727493171069_i128),119421402528678617825605850840190364083_i128,(-7118926094652492109921410438670940456_i128),(-118541426456282402147436289574406262498_i128),136132645348100891281372731749874948628_i128,33266209181780404372262023899472790949_i128,(-51137331188355560257098821904127426892_i128)];
_18 = 17890308218720400075_u64;
_20.1 = false;
_10 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_12 = !_14;
_18 = 1173222718232963590_u64;
_14 = _17;
Goto(bb9)
}
bb9 = {
_12 = 31_u8 as isize;
_10 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_13 = [212861789362372789415410360880594117891_u128,98118055899278396183546799338730961507_u128,218347831349705766460586590558793142615_u128,319616500674594473009739233801788743245_u128,275759276408317550413259961734200461049_u128,53835055893137408132623674233669204749_u128];
_20 = (5094078073465609594_usize, false);
_15 = _20.0 as isize;
_17 = _12;
_20 = Checked(18277027526871233170_usize * 1660025565730574409_usize);
_20.1 = true & false;
_20.1 = true;
_14 = _15;
_17 = _14 * _14;
_18 = 9776867013444500643_u64;
Goto(bb10)
}
bb10 = {
_14 = _15;
_17 = _15;
_13 = [203979673080979081305134709460389306079_u128,155921205805875050704740139867665777866_u128,106035497408095912383813852930884286738_u128,254925073171493368562191313287411670372_u128,26250074379837931825714988501830403444_u128,286497318267189411697174124124989306678_u128];
_21 = '\u{c8723}';
_14 = _17 * _17;
_13 = [5742089027899721352995646457856361547_u128,130447277683610003368716601492955643346_u128,162746145658527142163855750720777658482_u128,6814085220413658175953862899313763934_u128,88356900260087906063779856939351709135_u128,125569543343132094429333398219666623660_u128];
_15 = _20.1 as isize;
_21 = '\u{287c5}';
_20 = Checked(8185611069217284725_usize - 1_usize);
_20.1 = true;
_13 = [241463057867631239782095234458911991044_u128,218099731398844735531601707310065772963_u128,210581737691777119512032921825782639880_u128,37113966033439235313077586172428514622_u128,198151075753042317298855006395800630613_u128,124210420688191471536281237472810292564_u128];
_12 = !_14;
_25.fld2 = [216221600798511899543554310168625815906_u128,233446732697382360039576967088292078750_u128,146821779059217255751963719754092851173_u128,186184751247981083906616169414426886963_u128,163378184295719908984812770213141584492_u128,228591474184509208528767525170270710070_u128];
_25.fld6 = 131_u8 as i64;
_13 = [290983134101985088160522427874469450615_u128,87325921555897181584496382684355627158_u128,337376448597604106760638289029468902314_u128,253763464701803584573041107036985195807_u128,117361276956985857841306609406469919205_u128,277705301748942787832534271825399078918_u128];
_25.fld2 = [44004175004764788249590671739068430469_u128,288010850049763517531867703945186810705_u128,255504575516962412595865083421691004989_u128,226558826840572330762855520024517557104_u128,244404575705146218557565120177014818988_u128,339023980016055978770447820469413563989_u128];
_25.fld3.1 = 538252946_i32 << _14;
_25.fld4 = 10506_i16 >> _14;
_10 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_25.fld0 = Checked(_20.0 + _20.0);
_10 = [_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0];
_25.fld0.1 = !_20.1;
Goto(bb11)
}
bb11 = {
_25.fld4 = !20275_i16;
_13 = _25.fld2;
_17 = _14;
_26 = _25.fld3.1 as f32;
_13 = [143200209232092753302710492759030906357_u128,162420731318041948154151994740100526493_u128,122752582301648137532202770868524828917_u128,237776543061979257467927471364426323381_u128,142585745833357924202096096117212488015_u128,20990343786858613840874090632495175601_u128];
_20.1 = !_25.fld0.1;
_15 = _20.1 as isize;
_29 = _25.fld3.1 ^ _25.fld3.1;
Goto(bb12)
}
bb12 = {
_25.fld0.0 = _20.0 << _15;
Goto(bb13)
}
bb13 = {
_25.fld2 = [71971504843846242850753141043603515660_u128,91912858585029785937548972187155256043_u128,162419998198040514487917324525307567935_u128,255277914490582250388160305434172850480_u128,248292109312620937956273894054962503495_u128,33958046219514238462891242410882833239_u128];
_26 = _25.fld6 as f32;
_15 = _17 << _25.fld0.0;
_25.fld3.0 = (-57450362049400008600606998559237872941_i128) as f64;
_25.fld1 = [_20.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_20.1,_20.1,_25.fld0.1];
_25.fld4 = 32440382231749625985148498708585932453_u128 as i16;
_19 = [90185188669972120712141729096617624242_i128,115198008034555928987262290313326106549_i128,(-62265618348795256760307515354335663595_i128),90731923510029470347647617394449538482_i128,(-148340005334247727606451355370215835955_i128),140541076967170189879989465811120571794_i128,95098700788700741474668651373353658114_i128];
_25.fld3.0 = _15 as f64;
_15 = _12 * _17;
_25.fld3.0 = 19707_u16 as f64;
_28 = _17;
_12 = _17 ^ _17;
_28 = !_12;
match _18 {
0 => bb14,
1 => bb15,
9776867013444500643 => bb17,
_ => bb16
}
}
bb14 = {
_25.fld0.0 = _20.0 << _15;
Goto(bb13)
}
bb15 = {
_25.fld4 = !20275_i16;
_13 = _25.fld2;
_17 = _14;
_26 = _25.fld3.1 as f32;
_13 = [143200209232092753302710492759030906357_u128,162420731318041948154151994740100526493_u128,122752582301648137532202770868524828917_u128,237776543061979257467927471364426323381_u128,142585745833357924202096096117212488015_u128,20990343786858613840874090632495175601_u128];
_20.1 = !_25.fld0.1;
_15 = _20.1 as isize;
_29 = _25.fld3.1 ^ _25.fld3.1;
Goto(bb12)
}
bb16 = {
_14 = _15;
_17 = _15;
_13 = [203979673080979081305134709460389306079_u128,155921205805875050704740139867665777866_u128,106035497408095912383813852930884286738_u128,254925073171493368562191313287411670372_u128,26250074379837931825714988501830403444_u128,286497318267189411697174124124989306678_u128];
_21 = '\u{c8723}';
_14 = _17 * _17;
_13 = [5742089027899721352995646457856361547_u128,130447277683610003368716601492955643346_u128,162746145658527142163855750720777658482_u128,6814085220413658175953862899313763934_u128,88356900260087906063779856939351709135_u128,125569543343132094429333398219666623660_u128];
_15 = _20.1 as isize;
_21 = '\u{287c5}';
_20 = Checked(8185611069217284725_usize - 1_usize);
_20.1 = true;
_13 = [241463057867631239782095234458911991044_u128,218099731398844735531601707310065772963_u128,210581737691777119512032921825782639880_u128,37113966033439235313077586172428514622_u128,198151075753042317298855006395800630613_u128,124210420688191471536281237472810292564_u128];
_12 = !_14;
_25.fld2 = [216221600798511899543554310168625815906_u128,233446732697382360039576967088292078750_u128,146821779059217255751963719754092851173_u128,186184751247981083906616169414426886963_u128,163378184295719908984812770213141584492_u128,228591474184509208528767525170270710070_u128];
_25.fld6 = 131_u8 as i64;
_13 = [290983134101985088160522427874469450615_u128,87325921555897181584496382684355627158_u128,337376448597604106760638289029468902314_u128,253763464701803584573041107036985195807_u128,117361276956985857841306609406469919205_u128,277705301748942787832534271825399078918_u128];
_25.fld2 = [44004175004764788249590671739068430469_u128,288010850049763517531867703945186810705_u128,255504575516962412595865083421691004989_u128,226558826840572330762855520024517557104_u128,244404575705146218557565120177014818988_u128,339023980016055978770447820469413563989_u128];
_25.fld3.1 = 538252946_i32 << _14;
_25.fld4 = 10506_i16 >> _14;
_10 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_25.fld0 = Checked(_20.0 + _20.0);
_10 = [_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0,_25.fld0.0];
_25.fld0.1 = !_20.1;
Goto(bb11)
}
bb17 = {
_20 = Checked(_25.fld0.0 + _25.fld0.0);
_31.fld2 = [104473925381219168698511430511905069907_u128,3071289810233214373238387992228764574_u128,78561961487838327934288761752582004441_u128,117809242636424157768513128127637331725_u128,338592847512284741686035352439930940810_u128,166392638280304887556582395780647965107_u128];
_31.fld0.0 = _25.fld6 as usize;
_31.fld3.0 = -_25.fld3.0;
_10 = [_31.fld0.0,_25.fld0.0,_20.0,_20.0,_20.0,_20.0,_25.fld0.0];
_32 = core::ptr::addr_of_mut!(_31.fld3);
_31.fld0 = (_20.0, _25.fld0.1);
_18 = !18121846541946977499_u64;
_34 = [_14,_12,_12,_15,_14,_15,_15,_12];
_25.fld4 = (-8561_i16) - (-6552_i16);
_31.fld2 = _13;
(*_32).1 = 47714_u16 as i32;
_20 = _31.fld0;
_31.fld0 = (_20.0, _25.fld0.1);
_18 = !15349417233587354654_u64;
(*_32).0 = _25.fld3.0;
_20.1 = !_25.fld0.1;
_25.fld6 = -(-3158032390176733711_i64);
_31.fld4 = _25.fld0.0 as i16;
Goto(bb18)
}
bb18 = {
_18 = _21 as u64;
_31.fld5 = core::ptr::addr_of!(_37);
_19 = [(-49344036401900327136883190965725494577_i128),(-145058194429383585430624118586901726078_i128),(-148590530485843157069426005481427393342_i128),(-102189982417462362637649100852804972792_i128),84584490647890895608341040809147332326_i128,166705756280811712969386803733526664578_i128,129705374581322074484065123315416180944_i128];
_25.fld4 = _31.fld4 + _31.fld4;
_20 = (_25.fld0.0, _25.fld0.1);
_25.fld1 = [_20.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_25.fld0.1,_20.1,_20.1];
(*_32).0 = _25.fld3.0;
_31.fld1 = [_25.fld0.1,_31.fld0.1,_25.fld0.1,_31.fld0.1,_20.1,_31.fld0.1,_31.fld0.1];
_38 = core::ptr::addr_of!(_37);
_31.fld0 = _25.fld0;
(*_32).0 = _25.fld3.0;
(*_32).0 = _25.fld3.0;
_31.fld0.1 = _29 > _29;
_27 = core::ptr::addr_of!((*_38));
_6 = core::ptr::addr_of_mut!((*_38));
Call(_37.1 = fn15(_1, _5, _9, _1, _1, _5, _1, _8, _1), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_37.0 = _19;
_31.fld2 = [214362818184122527778653748416224765620_u128,170778181389628241215318643574494890740_u128,273006898956750269067862098405840269462_u128,203892132589445475451273294946978618220_u128,105144433859278840124456724860136128810_u128,313284576437333257833266361707611258820_u128];
_31.fld0 = (_20.0, (*_27).1);
_31.fld5 = core::ptr::addr_of!((*_6));
_40 = 980806505_u32 - 3693624046_u32;
(*_27).1 = _31.fld0.1;
_20.1 = !(*_38).1;
_13 = [63654350503191460713243679223184965981_u128,36723620619184791778869852317111581690_u128,257720999333095967283045572960234682488_u128,116139103102993299812364383297420047153_u128,258337942720487354371001681656646829065_u128,217660113563392950072422573746097431581_u128];
_31.fld6 = _25.fld6;
(*_38).0 = [81299537418400289927166294248378902776_i128,(-65100991494100188026272195544503327608_i128),111472115659652452889486879668487364290_i128,167389621028637719607567600969068233101_i128,25658138691340404170247908425062332442_i128,(-111469880021464232187413903202086655843_i128),6047093771897445454775225180699331106_i128];
_36 = _15 | _12;
_13 = [311560158978532404439782003436840534414_u128,128447306423218596588062566716593988228_u128,12656132074761253511319974162507315175_u128,185374146387079015537265225045955976080_u128,59838664098802827175025033679609318080_u128,43958942885359295827398064028387082468_u128];
_25.fld5 = _31.fld5;
_31.fld0.0 = _20.0;
_25 = Adt58 { fld0: _20,fld1: _31.fld1,fld2: _31.fld2,fld3: (*_32),fld4: _31.fld4,fld5: _38,fld6: _31.fld6 };
_26 = _18 as f32;
Goto(bb20)
}
bb20 = {
(*_6) = (_19, _25.fld0.1);
_20.0 = _18 as usize;
_45.0 = _31.fld6;
_12 = _36 << _28;
_42 = _26 - _26;
(*_32) = (_25.fld3.0, _29);
_25.fld3.1 = _29 & (*_32).1;
Call((*_32).1 = core::intrinsics::bswap(_25.fld3.1), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
_31.fld2 = [245730213296945765560206379075309431953_u128,227877173588574371691314170676123997613_u128,106225659054069769067737731514036986631_u128,254075957395284290735103593509604662729_u128,298667809619547954064392657879825573175_u128,259730934703713530286144461942330341021_u128];
(*_32) = _25.fld3;
_31.fld0.1 = _25.fld0.1;
_24 = Adt52::Variant1 { fld0: _10,fld1: _38,fld2: _34,fld3: _6,fld4: _25.fld4,fld5: (*_32),fld6: (-42341495149386852731793881793756742659_i128) };
_31.fld2 = [213443226590679889503980611061300935541_u128,305824808618202072805228386050162709682_u128,112797234761487187419243803611000408392_u128,67082112275035506786887344193860155949_u128,3297496593744991695401420954420767236_u128,310188011389635006384665786259590405387_u128];
_25.fld3.0 = (*_32).0;
(*_27) = (_19, _25.fld0.1);
_31.fld6 = _45.0;
_41 = _21;
(*_32).0 = _25.fld3.0;
(*_27) = (_19, _20.1);
_28 = -_36;
_21 = _41;
_36 = _12;
(*_6) = (_19, _31.fld0.1);
_25.fld0.0 = _31.fld0.0;
_31.fld4 = Field::<i16>(Variant(_24, 1), 4) + Field::<i16>(Variant(_24, 1), 4);
_25.fld5 = _38;
_25.fld0.1 = !(*_38).1;
(*_32).1 = _29 ^ Field::<(f64, i32)>(Variant(_24, 1), 5).1;
_44 = [_31.fld4,_25.fld4,_31.fld4,_25.fld4,_31.fld4,_25.fld4];
_31.fld3.0 = _25.fld3.0;
(*_27) = (_19, _20.1);
Goto(bb22)
}
bb22 = {
_35 = _42 as f64;
_25.fld1 = [_37.1,(*_38).1,_31.fld0.1,(*_6).1,(*_27).1,(*_27).1,_20.1];
(*_6) = (_19, _20.1);
_31.fld3.0 = -_25.fld3.0;
(*_27).1 = _25.fld0.1;
_25.fld0.1 = _31.fld3.1 != (*_32).1;
_25.fld5 = Field::<*const ([i128; 7], bool)>(Variant(_24, 1), 1);
(*_6) = (_19, _25.fld0.1);
(*_6).1 = _25.fld0.1;
_12 = !_15;
_20.0 = !_25.fld0.0;
_15 = _36 & _12;
(*_27) = (_19, _25.fld0.1);
place!(Field::<[usize; 7]>(Variant(_24, 1), 0)) = [_25.fld0.0,_20.0,_25.fld0.0,_20.0,_20.0,_31.fld0.0,_20.0];
_46 = _25.fld0.1;
(*_38).0 = [92389817261498244898403444512635224556_i128,146439870389502842446001611998477198820_i128,100442769306239562443081630021362296713_i128,146552836459398529457386575974858294859_i128,(-157605367406689613603845077502191152079_i128),59088030847214569045506699137739399555_i128,162274623829251848915581714577434892222_i128];
_34 = [_36,_17,_28,_12,_15,_36,_12,_28];
_35 = (*_32).0;
(*_32).0 = Field::<(f64, i32)>(Variant(_24, 1), 5).0;
_31.fld3 = (Field::<(f64, i32)>(Variant(_24, 1), 5).0, _25.fld3.1);
(*_38).1 = !_46;
_21 = _41;
RET = Adt52::Variant1 { fld0: _10,fld1: Field::<*const ([i128; 7], bool)>(Variant(_24, 1), 1),fld2: _34,fld3: _6,fld4: _25.fld4,fld5: (*_32),fld6: (-80851478627983984428037700023717266183_i128) };
_51 = _21;
(*_32) = (Field::<(f64, i32)>(Variant(RET, 1), 5).0, Field::<(f64, i32)>(Variant(RET, 1), 5).1);
place!(Field::<i16>(Variant(RET, 1), 4)) = -Field::<i16>(Variant(_24, 1), 4);
_25.fld5 = _38;
(*_6).0 = [(-49159942350047402486374796624123655816_i128),124041299337864632200531109182451243227_i128,149420427241057954796808252905840853292_i128,11405615175881698344263636421483140794_i128,14182847537734608989090131972561262222_i128,76409543242956641705690608747859656447_i128,117156807040273908055810267917003868200_i128];
_25.fld1 = [_25.fld0.1,_25.fld0.1,(*_38).1,(*_38).1,(*_27).1,(*_27).1,_46];
place!(Field::<i128>(Variant(RET, 1), 6)) = 66789030953689165941107745851991349859_i128 | 29527160786186007663223876071520878471_i128;
Goto(bb23)
}
bb23 = {
Call(_52 = dump_var(13_usize, 28_usize, Move(_28), 29_usize, Move(_29), 45_usize, Move(_45), 36_usize, Move(_36)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_52 = dump_var(13_usize, 37_usize, Move(_37), 12_usize, Move(_12), 17_usize, Move(_17), 40_usize, Move(_40)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_52 = dump_var(13_usize, 15_usize, Move(_15), 51_usize, Move(_51), 53_usize, _53, 53_usize, _53), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool,mut _8: *const bool) -> *mut (u32, f64, i8, f64, bool) {
mir! {
type RET = *mut (u32, f64, i8, f64, bool);
let _9: char;
let _10: *mut *mut (f64, i32);
let _11: u8;
let _12: (u8, i64, u64, [i128; 7]);
let _13: ([i128; 7], bool);
let _14: Adt44;
let _15: f64;
let _16: f64;
let _17: f64;
let _18: i16;
let _19: [i128; 7];
let _20: char;
let _21: Adt54;
let _22: i8;
let _23: *mut (u32, f64, i8, f64, bool);
let _24: [u128; 6];
let _25: isize;
let _26: u128;
let _27: u32;
let _28: isize;
let _29: u16;
let _30: (u32, f64, i8, f64, bool);
let _31: Adt57;
let _32: *mut *mut (f64, i32);
let _33: isize;
let _34: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32);
let _35: isize;
let _36: ();
let _37: ();
{
_9 = '\u{2d142}';
_9 = '\u{1c859}';
Goto(bb1)
}
bb1 = {
_11 = 37028_u16 as u8;
_9 = '\u{2a156}';
_12.0 = _11;
_12.0 = !_11;
_9 = '\u{c46bc}';
_13.0 = [(-81549872952297056608399119137985471163_i128),(-85557158020735141532473347366975256470_i128),(-96716976843769210295338295669505409397_i128),87170428004439431059182038236806408196_i128,21896091761423029877866083880059282767_i128,70794849173496802357027869774500392788_i128,86596793201400919407396214075615008958_i128];
_12.1 = (-1685200609799225510_i64);
_12.0 = 4080126947335085732_usize as u8;
_13.0 = [(-3501572701006665296751680615339171297_i128),(-4364243718397524459193016582027004947_i128),(-32316342401435195250142518567464119107_i128),148724853923157795397481949425027461936_i128,(-54811844230384305360050000856653782921_i128),151442509861880283339088296286555372332_i128,111105818417158332207515715792771844200_i128];
_13.0 = [122317574916702651719311992180697047603_i128,(-13918497401829409843919412765997580936_i128),(-128501595945003846107406533433797731179_i128),(-63452945250363338023835902853820590108_i128),(-147719644664303266143942589386372724890_i128),71401161720082180654597900504226871143_i128,(-1197893619597077914389873793150038938_i128)];
_12.2 = 8113825597006012274_u64;
Call(_12.1 = core::intrinsics::transmute(_12.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = (_11, 4741907461767001791_i64, 9966408127197535395_u64, _13.0);
_13 = (_12.3, true);
_14.fld2 = (_12.1,);
_11 = (-17732_i16) as u8;
_14.fld2 = (_12.1,);
_12.3 = _13.0;
_9 = '\u{4bf80}';
_12.1 = -_14.fld2.0;
_9 = '\u{bbb7b}';
_12 = (_11, _14.fld2.0, 5895620298709851843_u64, _13.0);
_12.3 = [(-57762116732728639863598787473758410981_i128),7966809419130208652069517480212747915_i128,(-164022040744425192445633255184183250680_i128),(-133534126466519741438147631045764235681_i128),(-125809173802804097842437693218424319393_i128),2333741461095300885360569312347853252_i128,(-10924294028056330093331448626606655352_i128)];
_12.2 = 7_usize as u64;
_12.0 = !_11;
_11 = _12.0 * _12.0;
_12.1 = _14.fld2.0;
_14.fld0 = [(-81490110769607389746620641046575293399_i128),17211244138169947742355047294406287584_i128,42036933045835796157638658983181315466_i128,119620056672191112884729920899400156701_i128,(-26406647161466182998825456057880749065_i128),(-6626095101729751136214705020776502496_i128),(-66850480561257110631921451654238814623_i128)];
_13 = (_12.3, false);
Goto(bb3)
}
bb3 = {
_14.fld1 = 5_usize as f32;
_12 = (_11, _14.fld2.0, 13482458881551597628_u64, _14.fld0);
_14.fld2.0 = _9 as i64;
_13 = (_14.fld0, false);
_9 = '\u{d0439}';
_12.3 = [(-135809771298597624182534327386845523747_i128),(-124314887386109771410130816818105253133_i128),(-152577263012957869541332664569644278481_i128),34364329533608847565652920621792357460_i128,(-115777538743980165370273550079062940086_i128),(-4745989756942954141275254052029665896_i128),(-29084852670228343145763773591764626348_i128)];
_16 = (-17697812346186083660597174069369708039_i128) as f64;
_14.fld2 = (_12.1,);
_12.3 = _14.fld0;
_14.fld1 = 227877598975655131470259845439223707121_u128 as f32;
_17 = _16 - _16;
_13.0 = _14.fld0;
_18 = 30_i8 as i16;
_14.fld2 = (_12.1,);
_11 = !_12.0;
_14.fld2 = (_12.1,);
_16 = _17 * _17;
_19 = _14.fld0;
_15 = 240689029771431817124991405409815977085_u128 as f64;
_9 = '\u{1091fb}';
_12 = (_11, _14.fld2.0, 11197886959099571119_u64, _14.fld0);
_14.fld0 = [107732210297316382918936036577645140777_i128,82543998383869106007549718245907106868_i128,166809375860755704271194919679523806742_i128,(-149314590683618840078401085829784718128_i128),118262495734341127699569576600403969833_i128,(-116924011562246827716922499622719707981_i128),133876811119509067699245002754457561601_i128];
_18 = (-15961_i16) << _14.fld2.0;
_11 = _12.0 - _12.0;
_12.0 = _9 as u8;
_14.fld0 = _19;
_14.fld1 = (-114_i8) as f32;
_9 = '\u{562ed}';
match _12.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
11197886959099571119 => bb10,
_ => bb9
}
}
bb4 = {
_12 = (_11, 4741907461767001791_i64, 9966408127197535395_u64, _13.0);
_13 = (_12.3, true);
_14.fld2 = (_12.1,);
_11 = (-17732_i16) as u8;
_14.fld2 = (_12.1,);
_12.3 = _13.0;
_9 = '\u{4bf80}';
_12.1 = -_14.fld2.0;
_9 = '\u{bbb7b}';
_12 = (_11, _14.fld2.0, 5895620298709851843_u64, _13.0);
_12.3 = [(-57762116732728639863598787473758410981_i128),7966809419130208652069517480212747915_i128,(-164022040744425192445633255184183250680_i128),(-133534126466519741438147631045764235681_i128),(-125809173802804097842437693218424319393_i128),2333741461095300885360569312347853252_i128,(-10924294028056330093331448626606655352_i128)];
_12.2 = 7_usize as u64;
_12.0 = !_11;
_11 = _12.0 * _12.0;
_12.1 = _14.fld2.0;
_14.fld0 = [(-81490110769607389746620641046575293399_i128),17211244138169947742355047294406287584_i128,42036933045835796157638658983181315466_i128,119620056672191112884729920899400156701_i128,(-26406647161466182998825456057880749065_i128),(-6626095101729751136214705020776502496_i128),(-66850480561257110631921451654238814623_i128)];
_13 = (_12.3, false);
Goto(bb3)
}
bb5 = {
_11 = 37028_u16 as u8;
_9 = '\u{2a156}';
_12.0 = _11;
_12.0 = !_11;
_9 = '\u{c46bc}';
_13.0 = [(-81549872952297056608399119137985471163_i128),(-85557158020735141532473347366975256470_i128),(-96716976843769210295338295669505409397_i128),87170428004439431059182038236806408196_i128,21896091761423029877866083880059282767_i128,70794849173496802357027869774500392788_i128,86596793201400919407396214075615008958_i128];
_12.1 = (-1685200609799225510_i64);
_12.0 = 4080126947335085732_usize as u8;
_13.0 = [(-3501572701006665296751680615339171297_i128),(-4364243718397524459193016582027004947_i128),(-32316342401435195250142518567464119107_i128),148724853923157795397481949425027461936_i128,(-54811844230384305360050000856653782921_i128),151442509861880283339088296286555372332_i128,111105818417158332207515715792771844200_i128];
_13.0 = [122317574916702651719311992180697047603_i128,(-13918497401829409843919412765997580936_i128),(-128501595945003846107406533433797731179_i128),(-63452945250363338023835902853820590108_i128),(-147719644664303266143942589386372724890_i128),71401161720082180654597900504226871143_i128,(-1197893619597077914389873793150038938_i128)];
_12.2 = 8113825597006012274_u64;
Call(_12.1 = core::intrinsics::transmute(_12.2), ReturnTo(bb2), UnwindUnreachable())
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
_13.0 = [66545568898887100881989104875452636428_i128,112964860895141949845091888119182051020_i128,(-129751706786692925803676053931831130249_i128),(-120768886395049901535991486492555037769_i128),102338145926532349634946616804443774584_i128,(-117166260968976646376806288269725053882_i128),129402024129378894761292753814107924539_i128];
_14.fld1 = 75317784366281233803330036513646742232_i128 as f32;
_18 = (-17200_i16);
_13.0 = [(-114405817235504386240853454903012475299_i128),(-149554211532259664544087087033491570057_i128),166552687427840247932014741426784369277_i128,(-47741088034558093428655433856474290037_i128),10134316746861305855968457137950460685_i128,80260734827151338482796280827076248744_i128,(-100450964984128001379942921507520612678_i128)];
_12.2 = 12315287664085811718_u64 << _12.1;
_13.1 = _12.2 <= _12.2;
_9 = '\u{eab17}';
_18 = (-17288_i16) >> _14.fld2.0;
match _12.1 {
0 => bb4,
1 => bb2,
2 => bb8,
3 => bb11,
4 => bb12,
4741907461767001791 => bb14,
_ => bb13
}
}
bb11 = {
_11 = 37028_u16 as u8;
_9 = '\u{2a156}';
_12.0 = _11;
_12.0 = !_11;
_9 = '\u{c46bc}';
_13.0 = [(-81549872952297056608399119137985471163_i128),(-85557158020735141532473347366975256470_i128),(-96716976843769210295338295669505409397_i128),87170428004439431059182038236806408196_i128,21896091761423029877866083880059282767_i128,70794849173496802357027869774500392788_i128,86596793201400919407396214075615008958_i128];
_12.1 = (-1685200609799225510_i64);
_12.0 = 4080126947335085732_usize as u8;
_13.0 = [(-3501572701006665296751680615339171297_i128),(-4364243718397524459193016582027004947_i128),(-32316342401435195250142518567464119107_i128),148724853923157795397481949425027461936_i128,(-54811844230384305360050000856653782921_i128),151442509861880283339088296286555372332_i128,111105818417158332207515715792771844200_i128];
_13.0 = [122317574916702651719311992180697047603_i128,(-13918497401829409843919412765997580936_i128),(-128501595945003846107406533433797731179_i128),(-63452945250363338023835902853820590108_i128),(-147719644664303266143942589386372724890_i128),71401161720082180654597900504226871143_i128,(-1197893619597077914389873793150038938_i128)];
_12.2 = 8113825597006012274_u64;
Call(_12.1 = core::intrinsics::transmute(_12.2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_12 = (_11, 4741907461767001791_i64, 9966408127197535395_u64, _13.0);
_13 = (_12.3, true);
_14.fld2 = (_12.1,);
_11 = (-17732_i16) as u8;
_14.fld2 = (_12.1,);
_12.3 = _13.0;
_9 = '\u{4bf80}';
_12.1 = -_14.fld2.0;
_9 = '\u{bbb7b}';
_12 = (_11, _14.fld2.0, 5895620298709851843_u64, _13.0);
_12.3 = [(-57762116732728639863598787473758410981_i128),7966809419130208652069517480212747915_i128,(-164022040744425192445633255184183250680_i128),(-133534126466519741438147631045764235681_i128),(-125809173802804097842437693218424319393_i128),2333741461095300885360569312347853252_i128,(-10924294028056330093331448626606655352_i128)];
_12.2 = 7_usize as u64;
_12.0 = !_11;
_11 = _12.0 * _12.0;
_12.1 = _14.fld2.0;
_14.fld0 = [(-81490110769607389746620641046575293399_i128),17211244138169947742355047294406287584_i128,42036933045835796157638658983181315466_i128,119620056672191112884729920899400156701_i128,(-26406647161466182998825456057880749065_i128),(-6626095101729751136214705020776502496_i128),(-66850480561257110631921451654238814623_i128)];
_13 = (_12.3, false);
Goto(bb3)
}
bb14 = {
_14.fld1 = _17 as f32;
_22 = 92087211950595779024256940555649221244_i128 as i8;
Goto(bb15)
}
bb15 = {
_17 = -_16;
_11 = _12.0;
_14.fld1 = _12.2 as f32;
_18 = 4106_u16 as i16;
Goto(bb16)
}
bb16 = {
_26 = 19695086255489596474098322709988218426_u128;
_14.fld2 = (_12.1,);
_17 = _16;
_16 = -_17;
_12 = (_11, _14.fld2.0, 7330716747048238541_u64, _19);
_14.fld2.0 = _18 as i64;
_9 = '\u{90f02}';
_14.fld0 = [(-13504423926849168754665586802920958364_i128),(-76540747527158402673288292349985473933_i128),96755441314200185338748547562411581205_i128,72221016367722650311753031555832039589_i128,(-126555729181357914428345481896739521173_i128),(-148150463756631709952377611727116636768_i128),(-4968781016556998826246186775796646038_i128)];
_18 = -9531_i16;
match _12.2 {
0 => bb1,
1 => bb2,
2 => bb13,
3 => bb10,
4 => bb5,
5 => bb11,
6 => bb7,
7330716747048238541 => bb17,
_ => bb8
}
}
bb17 = {
_20 = _9;
_20 = _9;
_15 = _12.0 as f64;
_14.fld2.0 = _12.1 >> _12.2;
_20 = _9;
_20 = _9;
match _12.1 {
0 => bb1,
1 => bb9,
2 => bb18,
3 => bb19,
4 => bb20,
5 => bb21,
6 => bb22,
4741907461767001791 => bb24,
_ => bb23
}
}
bb18 = {
Return()
}
bb19 = {
_11 = 37028_u16 as u8;
_9 = '\u{2a156}';
_12.0 = _11;
_12.0 = !_11;
_9 = '\u{c46bc}';
_13.0 = [(-81549872952297056608399119137985471163_i128),(-85557158020735141532473347366975256470_i128),(-96716976843769210295338295669505409397_i128),87170428004439431059182038236806408196_i128,21896091761423029877866083880059282767_i128,70794849173496802357027869774500392788_i128,86596793201400919407396214075615008958_i128];
_12.1 = (-1685200609799225510_i64);
_12.0 = 4080126947335085732_usize as u8;
_13.0 = [(-3501572701006665296751680615339171297_i128),(-4364243718397524459193016582027004947_i128),(-32316342401435195250142518567464119107_i128),148724853923157795397481949425027461936_i128,(-54811844230384305360050000856653782921_i128),151442509861880283339088296286555372332_i128,111105818417158332207515715792771844200_i128];
_13.0 = [122317574916702651719311992180697047603_i128,(-13918497401829409843919412765997580936_i128),(-128501595945003846107406533433797731179_i128),(-63452945250363338023835902853820590108_i128),(-147719644664303266143942589386372724890_i128),71401161720082180654597900504226871143_i128,(-1197893619597077914389873793150038938_i128)];
_12.2 = 8113825597006012274_u64;
Call(_12.1 = core::intrinsics::transmute(_12.2), ReturnTo(bb2), UnwindUnreachable())
}
bb20 = {
_14.fld1 = _17 as f32;
_22 = 92087211950595779024256940555649221244_i128 as i8;
Goto(bb15)
}
bb21 = {
Return()
}
bb22 = {
Return()
}
bb23 = {
_12 = (_11, 4741907461767001791_i64, 9966408127197535395_u64, _13.0);
_13 = (_12.3, true);
_14.fld2 = (_12.1,);
_11 = (-17732_i16) as u8;
_14.fld2 = (_12.1,);
_12.3 = _13.0;
_9 = '\u{4bf80}';
_12.1 = -_14.fld2.0;
_9 = '\u{bbb7b}';
_12 = (_11, _14.fld2.0, 5895620298709851843_u64, _13.0);
_12.3 = [(-57762116732728639863598787473758410981_i128),7966809419130208652069517480212747915_i128,(-164022040744425192445633255184183250680_i128),(-133534126466519741438147631045764235681_i128),(-125809173802804097842437693218424319393_i128),2333741461095300885360569312347853252_i128,(-10924294028056330093331448626606655352_i128)];
_12.2 = 7_usize as u64;
_12.0 = !_11;
_11 = _12.0 * _12.0;
_12.1 = _14.fld2.0;
_14.fld0 = [(-81490110769607389746620641046575293399_i128),17211244138169947742355047294406287584_i128,42036933045835796157638658983181315466_i128,119620056672191112884729920899400156701_i128,(-26406647161466182998825456057880749065_i128),(-6626095101729751136214705020776502496_i128),(-66850480561257110631921451654238814623_i128)];
_13 = (_12.3, false);
Goto(bb3)
}
bb24 = {
_12.3 = [67460061795953466464988069732102395455_i128,138757929734126647052809728648649908681_i128,57176548916243944747628341920517641807_i128,(-64087542977234504108328130179587795970_i128),127699051674974789727838455925295974373_i128,164813717428255708435644565348987925050_i128,21938906184856230631659383017077766601_i128];
_22 = _18 as i8;
_12.0 = _11 & _11;
RET = core::ptr::addr_of_mut!(_30);
(*RET).3 = -_15;
_20 = _9;
_29 = 1312_u16 & 54835_u16;
_33 = -9223372036854775807_isize;
_17 = _15 - _16;
_12.2 = 11323396030853768121_u64 + 3330062504905962636_u64;
_30 = (3399720504_u32, _16, _22, _16, _13.1);
(*RET).4 = _13.1;
_30.4 = _13.1 ^ _13.1;
_27 = 587712902_i32 as u32;
(*RET).3 = (*RET).1 - (*RET).1;
_30.3 = _30.1;
Goto(bb25)
}
bb25 = {
Call(_36 = dump_var(14_usize, 29_usize, Move(_29), 20_usize, Move(_20), 12_usize, Move(_12), 26_usize, Move(_26)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_36 = dump_var(14_usize, 18_usize, Move(_18), 33_usize, Move(_33), 37_usize, _37, 37_usize, _37), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *const bool,mut _2: *const bool,mut _3: *mut (u32, f64, i8, f64, bool),mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool,mut _8: *const bool,mut _9: *const bool) -> bool {
mir! {
type RET = bool;
let _10: Adt44;
let _11: [i32; 8];
let _12: i8;
let _13: f64;
let _14: char;
let _15: i64;
let _16: isize;
let _17: *mut (u32, f64, i8, f64, bool);
let _18: isize;
let _19: isize;
let _20: Adt55;
let _21: i64;
let _22: isize;
let _23: Adt46;
let _24: isize;
let _25: f64;
let _26: Adt59;
let _27: *mut (u32, f64, i8, f64, bool);
let _28: Adt44;
let _29: ((f64, i32),);
let _30: [bool; 7];
let _31: [usize; 7];
let _32: bool;
let _33: isize;
let _34: u32;
let _35: f32;
let _36: bool;
let _37: Adt52;
let _38: ();
let _39: ();
{
_10.fld2 = (4766657666267497775_i64,);
_10.fld2.0 = 1284209400307011142_i64;
RET = !false;
_10.fld2.0 = 7615208697121920124_i64;
_10.fld2.0 = (-4363022555274911383_i64);
RET = !false;
_10.fld2 = ((-9223006115349306913_i64),);
_11 = [950325696_i32,709586933_i32,451295882_i32,(-1976369399_i32),(-1216556748_i32),1995351144_i32,(-1424182995_i32),346297186_i32];
_10.fld1 = _10.fld2.0 as f32;
_10.fld1 = 27976_u16 as f32;
Goto(bb1)
}
bb1 = {
RET = _10.fld1 < _10.fld1;
_10.fld2.0 = -1447527063807739106_i64;
RET = false ^ true;
_11 = [1071573910_i32,1699165135_i32,(-1900505846_i32),1761803821_i32,(-1262452734_i32),56133729_i32,(-1369788625_i32),287656617_i32];
_10.fld0 = [(-21633705186586426420991145290534028691_i128),110020912137585906537508243836791790929_i128,(-96350022827892511869060600277046218502_i128),(-12557095287438718044943832005070631076_i128),(-120915707152529105484727956777949835045_i128),144641632255052128650164488270564129993_i128,(-109147524464539333223403607928701555629_i128)];
_10.fld2 = ((-3391029221430035240_i64),);
_12 = 7947747417460106162_u64 as i8;
_10.fld0 = [(-161721172254409516517253405308345693494_i128),(-106065506778938718031411558081198938099_i128),(-109882784153676324574048111657208191700_i128),(-81738564867953366804241697656167169934_i128),(-77562429402264521149453046803127242095_i128),(-81172601612725170355849762355546872763_i128),(-4101684233886551576089190281755420273_i128)];
_10.fld0 = [118263804320420267340500015985203279340_i128,(-63205689820953878824679150384298910947_i128),108855385672247154761298932763359944999_i128,(-11975668163299091473380621743201450737_i128),144335273290013172859032864627478125114_i128,35650374904228458805906606604660063689_i128,(-49537915773262183237053837876146721403_i128)];
_11 = [(-843050162_i32),1657475690_i32,(-1863021906_i32),2026753795_i32,(-1347404292_i32),193415641_i32,(-1692287604_i32),(-539875400_i32)];
_13 = 656005394_u32 as f64;
RET = false;
RET = !false;
Call(_10.fld0 = fn16(_5, _6, _7, _4, _8, _5, _9, _4, _4, _7, _7, _6, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = -(-111_i8);
RET = !false;
_13 = 1136994781_i32 as f64;
_14 = '\u{159ab}';
_10.fld2.0 = 2303212261701315296_i64;
_10.fld2.0 = -(-1547469306051738947_i64);
_10.fld2 = (7314038741165606913_i64,);
_15 = (-9223372036854775808_isize) as i64;
Goto(bb3)
}
bb3 = {
RET = true;
_10.fld0 = [(-83719383595166588198084059440614463575_i128),51315727673747736790854683696171191611_i128,75697901036167233654403703850337425109_i128,(-73947544235016566152713095033448466102_i128),(-169612030796022893906502853400545968246_i128),(-62245666764525510632466612986279490944_i128),(-169047491905029380966195935526481035272_i128)];
_10.fld2.0 = !_15;
_11 = [(-2126390404_i32),(-272698591_i32),(-1680120881_i32),552091086_i32,(-1119198243_i32),(-1111637517_i32),(-1042692329_i32),(-1510919202_i32)];
_10.fld2 = (_15,);
_11 = [(-822774125_i32),(-1420926953_i32),(-2127919202_i32),(-1685330264_i32),(-180691795_i32),142222462_i32,(-1407664111_i32),(-225700657_i32)];
_14 = '\u{382cc}';
_13 = 46115_u16 as f64;
RET = false;
_10.fld0 = [(-103910814799129243273640842135501478904_i128),(-146024726123555200908563318032881300445_i128),(-66603704021005852447526229009195758500_i128),74860368739534551049191988825376217828_i128,(-58572716509749074948859287618896224762_i128),(-17524017044862160117544143734457083706_i128),(-65830083614822189107590231957123766208_i128)];
_16 = -9223372036854775807_isize;
_15 = _10.fld2.0;
_13 = 120698089_u32 as f64;
_10.fld0 = [29492164207566567481297977977174982712_i128,(-60219508550747981619269028765284812339_i128),(-145589119856921087234657923704601601229_i128),(-79662210815859914843078726219634875158_i128),103121696679712138018390198432649648930_i128,(-68548617868058694394082688710056073960_i128),(-160944403421432493107879762264971864646_i128)];
_12 = 70_i8;
_17 = _3;
_17 = _3;
_11 = [(-364680873_i32),(-13782608_i32),1080794671_i32,270665292_i32,1733031386_i32,(-1503363835_i32),(-1196681822_i32),1596291640_i32];
_12 = (-127_i8) | 33_i8;
_12 = -(-102_i8);
_10.fld1 = _16 as f32;
_10.fld1 = _16 as f32;
_14 = '\u{ea0e9}';
_15 = 2761273429_u32 as i64;
_17 = _3;
_14 = '\u{cc97b}';
_18 = !_16;
Goto(bb4)
}
bb4 = {
_15 = _10.fld2.0;
_14 = '\u{e25ab}';
RET = true;
_16 = _18;
_18 = _14 as isize;
RET = false;
_17 = _3;
Goto(bb5)
}
bb5 = {
_10.fld2 = (_15,);
_12 = 87_i8 & 82_i8;
_19 = !_16;
_10.fld0 = [84341954125955347887501902369779152897_i128,(-129061469703518721412821545328465307268_i128),(-145303638366841356455085147227869872858_i128),73459176734988039977025369637010314830_i128,122863325992305755783153708830721522424_i128,(-47336661718460013238403367285673209731_i128),45999680131567823044911714395889851000_i128];
_19 = 236748542_u32 as isize;
_10.fld1 = 2667573631_u32 as f32;
_13 = 17178_u16 as f64;
RET = !false;
_10.fld2 = (_15,);
_10.fld2 = (_15,);
_10.fld0 = [(-41904810639547969416059240447688211916_i128),(-43960836284618723325278579573324025714_i128),(-141545349050395070005373535975790965427_i128),141443751987278246275854624043045280124_i128,53872446983438486048168026555815502565_i128,(-98723612162659551265405619588610980211_i128),21933184446092515983505222998205998117_i128];
_15 = _19 as i64;
Call(_10.fld2.0 = core::intrinsics::bswap(_15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10.fld0 = [94404130862152589996587168034028708505_i128,89271028480638906192683861722401947480_i128,142000332069816971865915057364106308004_i128,(-59573203291628568427081357492626081681_i128),(-111935867774312802789574057401186572957_i128),(-115021373144563287452613956762864483614_i128),11386679919083342419580287532899804367_i128];
_16 = 267154691456310854159950364714212579175_u128 as isize;
_12 = 11_i8 << _18;
_19 = _18;
Goto(bb7)
}
bb7 = {
RET = _15 < _10.fld2.0;
_3 = _17;
_19 = _16;
RET = !false;
_15 = -_10.fld2.0;
_10.fld2 = (_15,);
_21 = !_15;
_17 = _3;
_10.fld2 = (_15,);
_24 = 13570388724563996414_u64 as isize;
_12 = -95_i8;
_25 = _13 * _13;
_25 = -_13;
_15 = 1107593495_i32 as i64;
_18 = _24;
_12 = 91_i8 + 17_i8;
_19 = _16 << _18;
_21 = _15;
Goto(bb8)
}
bb8 = {
_25 = _13;
_23 = Adt46::Variant0 { fld0: 7_usize };
_15 = _10.fld2.0;
_17 = _3;
_17 = _3;
_13 = (-7925_i16) as f64;
_22 = 26305_i16 as isize;
_17 = _3;
_22 = _16;
place!(Field::<usize>(Variant(_23, 0), 0)) = 7876352237508369822_usize >> _22;
_20 = Adt55::Variant0 { fld0: _23 };
_29.0.1 = 1856755735_i32 + 43251472_i32;
_24 = _22;
_22 = _19;
_28.fld0 = [102882283708890351874737674999374852730_i128,30515479754350887093948963448202289861_i128,(-2134215559340052988679863213032688706_i128),(-18574262413104623210401559709873522863_i128),39171672179303079856395727813783874087_i128,(-122101943463336893816078213109779194465_i128),3126944068758455013618283948210513326_i128];
_28 = Adt44 { fld0: _10.fld0,fld1: _10.fld1,fld2: _10.fld2 };
_22 = 2600885373376685681_u64 as isize;
_27 = _17;
Goto(bb9)
}
bb9 = {
_10 = Adt44 { fld0: _28.fld0,fld1: _28.fld1,fld2: _28.fld2 };
_16 = !_19;
_31 = [Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0)];
_19 = _12 as isize;
_29.0 = (_13, 876487414_i32);
RET = true;
_28.fld1 = -_10.fld1;
_20 = Adt55::Variant0 { fld0: _23 };
_28.fld0 = [83267161663185718137365803206617228640_i128,(-148398889065819213415917904468573310760_i128),(-53741017823659637773966661297249948089_i128),15741390251854275356159522193679187778_i128,(-67256167083180006744519080631472891314_i128),109531054007820340276295148069603834516_i128,(-37991081244686210081819776275420937439_i128)];
_32 = !RET;
_13 = _25 + _29.0.0;
_27 = _3;
_10.fld2.0 = !_28.fld2.0;
_28.fld1 = -_10.fld1;
_14 = '\u{4ed18}';
_30 = [_32,RET,RET,RET,RET,_32,_32];
_19 = _22;
_13 = -_25;
Goto(bb10)
}
bb10 = {
_33 = _19 ^ _22;
_27 = _3;
_28.fld2 = _10.fld2;
_10 = _28;
_10 = Adt44 { fld0: _28.fld0,fld1: _28.fld1,fld2: _28.fld2 };
match _29.0.1 {
0 => bb3,
1 => bb11,
2 => bb12,
3 => bb13,
876487414 => bb15,
_ => bb14
}
}
bb11 = {
_10 = Adt44 { fld0: _28.fld0,fld1: _28.fld1,fld2: _28.fld2 };
_16 = !_19;
_31 = [Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0)];
_19 = _12 as isize;
_29.0 = (_13, 876487414_i32);
RET = true;
_28.fld1 = -_10.fld1;
_20 = Adt55::Variant0 { fld0: _23 };
_28.fld0 = [83267161663185718137365803206617228640_i128,(-148398889065819213415917904468573310760_i128),(-53741017823659637773966661297249948089_i128),15741390251854275356159522193679187778_i128,(-67256167083180006744519080631472891314_i128),109531054007820340276295148069603834516_i128,(-37991081244686210081819776275420937439_i128)];
_32 = !RET;
_13 = _25 + _29.0.0;
_27 = _3;
_10.fld2.0 = !_28.fld2.0;
_28.fld1 = -_10.fld1;
_14 = '\u{4ed18}';
_30 = [_32,RET,RET,RET,RET,_32,_32];
_19 = _22;
_13 = -_25;
Goto(bb10)
}
bb12 = {
_15 = _10.fld2.0;
_14 = '\u{e25ab}';
RET = true;
_16 = _18;
_18 = _14 as isize;
RET = false;
_17 = _3;
Goto(bb5)
}
bb13 = {
RET = _15 < _10.fld2.0;
_3 = _17;
_19 = _16;
RET = !false;
_15 = -_10.fld2.0;
_10.fld2 = (_15,);
_21 = !_15;
_17 = _3;
_10.fld2 = (_15,);
_24 = 13570388724563996414_u64 as isize;
_12 = -95_i8;
_25 = _13 * _13;
_25 = -_13;
_15 = 1107593495_i32 as i64;
_18 = _24;
_12 = 91_i8 + 17_i8;
_19 = _16 << _18;
_21 = _15;
Goto(bb8)
}
bb14 = {
_10.fld0 = [94404130862152589996587168034028708505_i128,89271028480638906192683861722401947480_i128,142000332069816971865915057364106308004_i128,(-59573203291628568427081357492626081681_i128),(-111935867774312802789574057401186572957_i128),(-115021373144563287452613956762864483614_i128),11386679919083342419580287532899804367_i128];
_16 = 267154691456310854159950364714212579175_u128 as isize;
_12 = 11_i8 << _18;
_19 = _18;
Goto(bb7)
}
bb15 = {
_31 = [Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(_23, 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0),Field::<usize>(Variant(Field::<Adt46>(Variant(_20, 0), 0), 0), 0)];
_34 = 28036_i16 as u32;
_18 = _16 & _24;
_33 = _16 ^ _16;
_35 = _28.fld1 - _10.fld1;
_29.0.1 = (-391762585_i32) | (-692819633_i32);
_32 = RET;
_24 = -_33;
_29.0 = (_13, 1767403743_i32);
_11 = [_29.0.1,_29.0.1,_29.0.1,_29.0.1,_29.0.1,_29.0.1,_29.0.1,_29.0.1];
_3 = _27;
_10.fld1 = _28.fld1;
_17 = _3;
_34 = 1289159161_u32;
place!(Field::<Adt46>(Variant(_20, 0), 0)) = _23;
_10.fld0 = [11305125695250351437046922984464749865_i128,125011996026852983085358652191182179788_i128,116275387920948674562609275752547523393_i128,(-31052205356239867087525584853577021939_i128),52306251662313799497625727928468509588_i128,124011254282762657861775482358050623774_i128,8973444495999428209545396585165210002_i128];
RET = _32;
_27 = _17;
_36 = !_32;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(15_usize, 30_usize, Move(_30), 11_usize, Move(_11), 34_usize, Move(_34), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(15_usize, 15_usize, Move(_15), 12_usize, Move(_12), 22_usize, Move(_22), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: *const bool,mut _2: *const bool,mut _3: *const bool,mut _4: *const bool,mut _5: *const bool,mut _6: *const bool,mut _7: *const bool,mut _8: *const bool,mut _9: *const bool,mut _10: *const bool,mut _11: *const bool,mut _12: *const bool,mut _13: *const bool) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _14: isize;
let _15: bool;
let _16: (usize, u64, ([i128; 7], bool));
let _17: [bool; 7];
let _18: [i128; 7];
let _19: bool;
let _20: [usize; 7];
let _21: Adt49;
let _22: isize;
let _23: u32;
let _24: (usize, u64, ([i128; 7], bool));
let _25: [bool; 7];
let _26: char;
let _27: i64;
let _28: Adt46;
let _29: ([i128; 7], bool);
let _30: (u8, i64, u64, [i128; 7]);
let _31: isize;
let _32: f64;
let _33: Adt46;
let _34: ();
let _35: ();
{
RET = [117680912705500359860587943182117839466_i128,36411973115612260493450432505431126676_i128,(-57825302068130358169387351901798552714_i128),87311698055635843729430259007861516983_i128,(-168822353443700768291061673231341648194_i128),(-44903716779031116814818778496932817726_i128),509990315722262452545123952321784909_i128];
RET = [(-106145574789519302976960007270709436830_i128),26560030389322529624829396143898670348_i128,110913826042763192660485974481561786443_i128,16925746132356637496079602341934279305_i128,28930043886998345727119665466604667308_i128,52859400304554743527554302871455979861_i128,(-92853346049089361020610150897062135991_i128)];
RET = [(-16144111438834862881448615404565056828_i128),73166404777019359156471419382596905576_i128,5949559050109375791395904798694576058_i128,(-4332369353265888706368703123338340006_i128),109164924188339778186233260835332180171_i128,(-50080431348855791787742045942985607446_i128),(-31174130448290118619047246982977735882_i128)];
_14 = 9223372036854775807_isize;
_14 = 8908869684276929311_u64 as isize;
RET = [(-67844946562122611070154269244078881237_i128),137476992152012110166861310562894171864_i128,15250655427300993461649546655132236779_i128,(-141996159792958562017901754073087295796_i128),74796135897435899274394996000698882812_i128,(-154133275580339759414280876879191185235_i128),81774879299774485698666695469057661178_i128];
RET = [42151021902813888042742921960044066136_i128,(-45814244145151792028707439160080835965_i128),(-168867505762690796618806186409768252837_i128),(-135137312771492556328116425703784157566_i128),(-110988254016815048806250446251058744748_i128),29295732123276621116822613425434203934_i128,(-147206445062588779521323352675380930330_i128)];
_14 = 64_isize ^ 118_isize;
_15 = !false;
RET = [(-69255446741868007828476883648070485933_i128),146857574566601559800592061606794321148_i128,(-165402880501001998242293167611210660292_i128),(-143437041673690220825807277934134323550_i128),133167804913002089441587818540871805686_i128,67524636058972545048772886373366605677_i128,136036646926390595599336132238642204586_i128];
RET = [(-74046292249589441752532285831759530911_i128),(-17005381741784468354998083975541762489_i128),64700989200818291699410658016135800095_i128,(-106783762023193315012124553684907078949_i128),(-9667443641540342669854254573212232576_i128),(-9584366320903257635544388628517909664_i128),1770346734150579905184084367547594711_i128];
RET = [6663030104633483567530926223241998340_i128,125094712723445555716937461066059594581_i128,(-136852218295273867100483532399433627397_i128),(-124734521275019197062179284679201717795_i128),162774828982207216945581754367578104960_i128,117590763238093027233508900764754079661_i128,(-10037769070630045642535909039444093705_i128)];
Goto(bb1)
}
bb1 = {
RET = [(-151936134855968644274581972778867010749_i128),(-65159761604418740846199090681351071732_i128),65217353300470880981092238492087988680_i128,71882436089044877354673174968433814156_i128,94080589019411796936772484364706229139_i128,(-27609257979972759583347499526425467939_i128),(-65773287652220256999516789724915039025_i128)];
RET = [61372235977959365852377861066198367291_i128,156951121929961179643876368884321187776_i128,120106063197194923991298458773025037698_i128,68460466737470601936701378346836870318_i128,39485395914308152422059205332453048612_i128,(-135724284290401113207096893437701245497_i128),3618430631735217402880938485738473121_i128];
RET = [(-42210572135879277101201661011820891572_i128),13904924010795766166986495770049361846_i128,(-15416345131052195210479754204945257719_i128),(-11028311385425109682415249381527438390_i128),(-90724284325496903613287098415645568140_i128),(-12097858979422890945611563713556091180_i128),84199160901571385543792750743154265636_i128];
Goto(bb2)
}
bb2 = {
_16.1 = !17076752819789613561_u64;
RET = [(-79505836416516604119603266855609633921_i128),(-82988017123055264080855346575754934876_i128),109781641063472432072397909385029506205_i128,165582066052695647641827954474885884033_i128,76743852738691842023905249517559803537_i128,52144961745129254232898104381265844677_i128,(-121241881946727859277166423748851690090_i128)];
_16.2.0 = [145425204461485108760146324383709307307_i128,101681887167750403167127654319578793076_i128,98121999794157864443940803830250012203_i128,(-31511594488028905236724763275649286638_i128),52046630649085189210819315120769091636_i128,100076926993008464102396096503770447862_i128,70645940346244052239981425853955175727_i128];
_16.2 = (RET, _15);
_16.0 = !12412128396114044947_usize;
_16.0 = 1_usize;
_14 = !(-9223372036854775808_isize);
_15 = _16.1 == _16.1;
_16.2.0 = RET;
_16.2 = (RET, _15);
match _16.0 {
0 => bb3,
1 => bb5,
_ => bb4
}
}
bb3 = {
RET = [(-151936134855968644274581972778867010749_i128),(-65159761604418740846199090681351071732_i128),65217353300470880981092238492087988680_i128,71882436089044877354673174968433814156_i128,94080589019411796936772484364706229139_i128,(-27609257979972759583347499526425467939_i128),(-65773287652220256999516789724915039025_i128)];
RET = [61372235977959365852377861066198367291_i128,156951121929961179643876368884321187776_i128,120106063197194923991298458773025037698_i128,68460466737470601936701378346836870318_i128,39485395914308152422059205332453048612_i128,(-135724284290401113207096893437701245497_i128),3618430631735217402880938485738473121_i128];
RET = [(-42210572135879277101201661011820891572_i128),13904924010795766166986495770049361846_i128,(-15416345131052195210479754204945257719_i128),(-11028311385425109682415249381527438390_i128),(-90724284325496903613287098415645568140_i128),(-12097858979422890945611563713556091180_i128),84199160901571385543792750743154265636_i128];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_16.2 = (RET, _15);
RET = [(-92747209489543185249479917450739288810_i128),46905716034038954354462813115712315201_i128,78376317753905745502392485983510568208_i128,(-78914673436872203686885242038004095449_i128),43397363523547723216843936038384548318_i128,(-140491276927604140272903294446280533751_i128),110100384261520370956582879213632507644_i128];
_14 = _16.1 as isize;
_14 = (-9223372036854775808_isize);
_15 = _14 == _14;
_15 = !_16.2.1;
_16.0 = !2_usize;
RET = [(-103681222280926210050573597861644876351_i128),(-124225384391439570336166778261005057449_i128),(-49539630068696605290086297372510395663_i128),154733147314421780450668127783562143819_i128,(-8003271823858839150664254348096956225_i128),(-54868596382101682714934421339884900155_i128),(-28338470639359516258041005022022867578_i128)];
_16.2 = (RET, _15);
Goto(bb6)
}
bb6 = {
_16.2.0 = RET;
_16.2.1 = _15;
_14 = -(-9223372036854775808_isize);
_14 = -9223372036854775807_isize;
_14 = 296349076023580289397334577974177348874_u128 as isize;
Goto(bb7)
}
bb7 = {
_16.2.0 = [112228776530322067444181532294404597785_i128,25087013009941473633342144163772082479_i128,(-162548184678871771963123156502576952780_i128),(-70714843571638245573862125026211922420_i128),85877394265052881438510144303913048323_i128,158846597862827657063536887142439328791_i128,57344803822569903750825888290592079052_i128];
_16.2.0 = RET;
_14 = !(-36_isize);
RET = [164571420890456117972765017900574266828_i128,1210517404850975455092910242102390916_i128,(-23814792958362594449057871577891169207_i128),59256918952446695663143495697262338272_i128,49635540614339005924820591184496199541_i128,162584840000580839935148972955323372257_i128,165685494088293862796664010705958620658_i128];
_16.0 = 8751312875245206896_usize * 7462845540158359688_usize;
Goto(bb8)
}
bb8 = {
_19 = _15 ^ _15;
_16.1 = 4270035529904359925_u64;
RET = _16.2.0;
RET = [(-71069230483367307665964625044210402219_i128),(-16309835634882491664710844599680833015_i128),80458089060189185755618537463564943950_i128,59879565324195381821625391288933926839_i128,142339500029846788424722626342429864073_i128,(-108028121742707434513861242790169504881_i128),7994921589827897501663023131725564639_i128];
_16.2 = (RET, _19);
RET = _16.2.0;
_16.1 = 6987144600244247592_u64;
_16.1 = (-101659979853898001192396410552331554681_i128) as u64;
_22 = 1616_u16 as isize;
_14 = '\u{d72fd}' as isize;
Goto(bb9)
}
bb9 = {
_20 = [_16.0,_16.0,_16.0,_16.0,_16.0,_16.0,_16.0];
_24.0 = !_16.0;
_19 = _16.2.1;
_25 = [_15,_19,_16.2.1,_16.2.1,_19,_15,_19];
_24.2.0 = [3141361761771647518504417522818624626_i128,92216192340275664777453716868049591352_i128,(-58733133209055618717723704114753081176_i128),(-134374729306495662448477014611296524856_i128),32389188320930241370591852391529191883_i128,106072933407837381281822210818834399282_i128,133341335488632735627054808296771688342_i128];
_23 = 3257285029_u32;
_26 = '\u{dd5fc}';
_15 = _16.2.1;
_24.2 = (_16.2.0, _15);
_24 = _16;
_20 = [_24.0,_24.0,_16.0,_24.0,_16.0,_24.0,_24.0];
_17 = [_15,_16.2.1,_24.2.1,_19,_15,_24.2.1,_24.2.1];
_20 = [_24.0,_24.0,_24.0,_24.0,_24.0,_24.0,_24.0];
RET = [2962839857134413751691978112973994979_i128,(-158097918368294702689430151686777896604_i128),(-98937980880778880445848957684597772054_i128),(-165131975155508915226711036087398167221_i128),83863313553870743631519169812749441381_i128,(-168223080468562719055036032298369777774_i128),63743872055979484753131073598534668073_i128];
_27 = 4986207718070681159_i64;
_16.0 = _24.0 - _24.0;
_16 = (_24.0, _24.1, _24.2);
_17 = [_24.2.1,_15,_16.2.1,_16.2.1,_24.2.1,_15,_16.2.1];
_22 = _26 as isize;
_18 = _16.2.0;
RET = _24.2.0;
_24.2.1 = !_15;
_24.1 = _16.1 - _16.1;
_24.2 = (RET, _16.2.1);
match _23 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
3257285029 => bb10,
_ => bb6
}
}
bb10 = {
_16.0 = !_24.0;
_23 = 1892651529_u32 * 3425102645_u32;
_14 = -_22;
_16.0 = _24.0;
_14 = _22 * _22;
_30.0 = (-78063779080610041148536529598301385752_i128) as u8;
_31 = _14 << _24.0;
_24.2.1 = _19;
_16.2.1 = _19 != _24.2.1;
_20 = [_24.0,_16.0,_16.0,_24.0,_24.0,_16.0,_16.0];
_14 = _31 & _31;
_32 = _14 as f64;
match _27 {
0 => bb11,
4986207718070681159 => bb13,
_ => bb12
}
}
bb11 = {
_16.2 = (RET, _15);
RET = [(-92747209489543185249479917450739288810_i128),46905716034038954354462813115712315201_i128,78376317753905745502392485983510568208_i128,(-78914673436872203686885242038004095449_i128),43397363523547723216843936038384548318_i128,(-140491276927604140272903294446280533751_i128),110100384261520370956582879213632507644_i128];
_14 = _16.1 as isize;
_14 = (-9223372036854775808_isize);
_15 = _14 == _14;
_15 = !_16.2.1;
_16.0 = !2_usize;
RET = [(-103681222280926210050573597861644876351_i128),(-124225384391439570336166778261005057449_i128),(-49539630068696605290086297372510395663_i128),154733147314421780450668127783562143819_i128,(-8003271823858839150664254348096956225_i128),(-54868596382101682714934421339884900155_i128),(-28338470639359516258041005022022867578_i128)];
_16.2 = (RET, _15);
Goto(bb6)
}
bb12 = {
_19 = _15 ^ _15;
_16.1 = 4270035529904359925_u64;
RET = _16.2.0;
RET = [(-71069230483367307665964625044210402219_i128),(-16309835634882491664710844599680833015_i128),80458089060189185755618537463564943950_i128,59879565324195381821625391288933926839_i128,142339500029846788424722626342429864073_i128,(-108028121742707434513861242790169504881_i128),7994921589827897501663023131725564639_i128];
_16.2 = (RET, _19);
RET = _16.2.0;
_16.1 = 6987144600244247592_u64;
_16.1 = (-101659979853898001192396410552331554681_i128) as u64;
_22 = 1616_u16 as isize;
_14 = '\u{d72fd}' as isize;
Goto(bb9)
}
bb13 = {
_17 = [_16.2.1,_24.2.1,_16.2.1,_15,_16.2.1,_19,_15];
_32 = 277773156470005336524174667164952655740_u128 as f64;
_19 = !_24.2.1;
_18 = [139262482388845059267945632614007113954_i128,117354637606412899130317665357603203171_i128,(-59608592863020262618901383152551374023_i128),(-7870881145999503803727916361897741532_i128),56691130586298599495832465386376530540_i128,169552133712826633188998638073480336269_i128,152600511929227463171582955147797169268_i128];
_30.3 = _18;
_16.2.0 = [(-156434657277550516559167231602670764626_i128),(-87277555181387242842754583027064804777_i128),10186916135697077992693082167135765125_i128,(-19914989630022227462715184881275225537_i128),85974133649473532933203874481348967763_i128,60104634307144731070013381282050342413_i128,(-7678318525350453795157882369838957034_i128)];
_16.0 = !_24.0;
_24.1 = !_16.1;
_16.2.0 = [102953858229266756599219362276103278054_i128,(-144840342474507863211511551765572793360_i128),151982600135084033187907639519080810154_i128,(-31131591902229055028927762623782691672_i128),(-79797414572280115228431079029276540229_i128),27580039783702894883260187233488903082_i128,(-64156701875141723165558657628895770520_i128)];
_29 = (_24.2.0, _24.2.1);
_27 = 24144_u16 as i64;
_17 = _25;
_24.2.0 = RET;
_24 = _16;
RET = [59916993224653605997916126098134742567_i128,(-114837463495041812417458264555512303275_i128),(-78289469239960343157987212928824577085_i128),(-135439182160636772266900369539027958555_i128),(-58898348429842267483397341742782931776_i128),(-59507748066732103212719680072747300892_i128),67739299497938416508045744143585987392_i128];
_30.1 = _27;
_16.2.0 = RET;
Goto(bb14)
}
bb14 = {
_16 = (_24.0, _24.1, _29);
_32 = 103826876765383154589583393992781417849_i128 as f64;
_17 = [_16.2.1,_15,_24.2.1,_24.2.1,_19,_24.2.1,_15];
_16.2 = (_18, _29.1);
_19 = _14 < _14;
_30 = (173_u8, _27, _24.1, _16.2.0);
RET = [106855578442052037407777361704253723143_i128,(-39795094728869320256518707732186855149_i128),80568003707457160639103920703048043971_i128,(-49811964024108489362091510445260183917_i128),(-106135490792838195520002387044787628641_i128),(-1302418543264762778263263081262213431_i128),(-154243032267792785258524237344974646652_i128)];
_16.0 = _24.0;
_26 = '\u{aee3a}';
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(16_usize, 29_usize, Move(_29), 16_usize, Move(_16), 23_usize, Move(_23), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(16_usize, 26_usize, Move(_26), 17_usize, Move(_17), 31_usize, Move(_31), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(16955202583032773465_u64), std::hint::black_box('\u{8fbba}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(91_i8), std::hint::black_box((-24710_i16)), std::hint::black_box((-819475994_i32)), std::hint::black_box(255_u8), std::hint::black_box((-108656575205005532608088012437375510476_i128)), std::hint::black_box(1413784627_u32));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: [i128; 7],
fld1: f32,
fld2: (i64,),
}
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: bool,
fld1: (f64, i32),
fld2: usize,
fld3: u16,
fld4: *mut (u32, f64, i8, f64, bool),
fld5: i128,
fld6: i64,

},
Variant1{
fld0: (i16, bool),
fld1: (f64, i32),
fld2: u32,
fld3: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),
fld4: i64,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: usize,

},
Variant1{
fld0: i128,
fld1: usize,
fld2: isize,
fld3: f64,

}}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [i128; 7],
fld1: u32,

},
Variant1{
fld0: *mut (f64, i32),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: usize,
fld1: Adt46,

},
Variant1{
fld0: [usize; 7],
fld1: f32,
fld2: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32),
fld3: i64,

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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u128,
fld1: [u128; 6],
fld2: *mut *mut (f64, i32),

},
Variant1{
fld0: i128,
fld1: *mut ([i128; 7], bool),
fld2: i64,
fld3: [i32; 8],
fld4: ([i128; 7], bool),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (f64, i32),

},
Variant1{
fld0: [i16; 6],
fld1: i16,
fld2: *mut *mut (f64, i32),

},
Variant2{
fld0: i64,
fld1: i32,
fld2: [bool; 7],
fld3: *mut *mut (f64, i32),
fld4: (usize, u64, ([i128; 7], bool)),

},
Variant3{
fld0: [i16; 6],
fld1: [isize; 8],
fld2: Adt46,
fld3: (f64, i32),
fld4: (u32, f64, i8, f64, bool),

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt50,
fld1: Adt44,
fld2: u8,
fld3: *mut (f64, i32),
fld4: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),
fld5: [isize; 8],
fld6: [usize; 7],
fld7: ((f64, i32),),

},
Variant1{
fld0: (u32, f64, i8, f64, bool),
fld1: (i64,),
fld2: isize,
fld3: u128,

},
Variant2{
fld0: Adt47,
fld1: (u32, f64, i8, f64, bool),
fld2: f64,
fld3: [u8; 1],
fld4: (([i128; 7], bool),),
fld5: (i64,),
fld6: [bool; 7],
fld7: Adt49,

},
Variant3{
fld0: u64,
fld1: (usize, bool),
fld2: isize,
fld3: (u8, i64, u64, [i128; 7]),
fld4: (([i128; 7], bool),),
fld5: [u8; 1],
fld6: Adt50,

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),
fld1: Adt45,
fld2: [u8; 1],

},
Variant1{
fld0: [usize; 7],
fld1: *const ([i128; 7], bool),
fld2: [isize; 8],
fld3: *mut ([i128; 7], bool),
fld4: i16,
fld5: (f64, i32),
fld6: i128,

},
Variant2{
fld0: bool,
fld1: Adt49,
fld2: (usize, bool),
fld3: f64,

},
Variant3{
fld0: [i16; 6],
fld1: char,
fld2: u16,
fld3: [i32; 8],
fld4: Adt50,
fld5: i32,
fld6: u128,
fld7: (u8, i64, u64, [i128; 7]),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
fld0: u16,
fld1: Adt51,
fld2: ([i128; 7], bool),
fld3: Adt49,
fld4: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32),
fld5: u32,
fld6: (usize, u64, ([i128; 7], bool)),

},
Variant1{
fld0: Adt49,
fld1: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32),

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [isize; 8],
fld1: Adt45,
fld2: (i16, bool),
fld3: [usize; 7],
fld4: (usize, u64, ([i128; 7], bool)),
fld5: (usize, bool),
fld6: Adt44,
fld7: [i32; 8],

},
Variant1{
fld0: (([i128; 7], bool),),
fld1: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32),
fld2: f64,
fld3: *mut *mut (f64, i32),
fld4: i16,
fld5: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),

},
Variant2{
fld0: *mut ([i128; 7], bool),
fld1: Adt51,
fld2: *const ([i128; 7], bool),

},
Variant3{
fld0: (usize, u64, ([i128; 7], bool)),
fld1: *mut (u32, f64, i8, f64, bool),
fld2: [isize; 8],
fld3: *const bool,
fld4: *mut (f64, i32),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt46,

},
Variant1{
fld0: [u128; 6],
fld1: u16,
fld2: isize,
fld3: (i64,),
fld4: (i16, bool),
fld5: Adt54,
fld6: usize,
fld7: ((f64, i32),),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *mut (u32, f64, i8, f64, bool),
fld1: Adt46,
fld2: *mut *mut (f64, i32),
fld3: (([i128; 7], bool),),
fld4: (usize, u64, ([i128; 7], bool)),
fld5: Adt55,

},
Variant1{
fld0: *mut ([i128; 7], bool),
fld1: [i128; 7],
fld2: *const bool,
fld3: ((f64, i32),),
fld4: Adt50,
fld5: u64,
fld6: i64,

},
Variant2{
fld0: Adt45,
fld1: Adt55,
fld2: isize,
fld3: [i128; 7],
fld4: *const ([i128; 7], bool),
fld5: f64,
fld6: (u32, f64, i8, f64, bool),

},
Variant3{
fld0: ((f64, i32),),
fld1: u8,
fld2: i32,
fld3: Adt52,
fld4: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),

}}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32),
fld1: f64,
fld2: *mut ([i128; 7], bool),
fld3: Adt52,
fld4: Adt44,
fld5: [u128; 6],
fld6: i64,
fld7: Adt45,

},
Variant1{
fld0: (u32, f64, i8, f64, bool),
fld1: [u8; 1],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: (usize, bool),
fld1: [bool; 7],
fld2: [u128; 6],
fld3: (f64, i32),
fld4: i16,
fld5: *const ([i128; 7], bool),
fld6: i64,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: bool,
fld1: (([i128; 7], bool),),
fld2: (usize, bool),
fld3: *mut *mut (f64, i32),
fld4: Adt56,
fld5: (usize, u64, ([i128; 7], bool)),
fld6: [isize; 8],
fld7: f64,

},
Variant1{
fld0: u16,
fld1: *mut ([i128; 7], bool),
fld2: ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32),

},
Variant2{
fld0: Adt57,
fld1: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),
fld2: (([i128; 7], bool),),
fld3: Adt50,

},
Variant3{
fld0: bool,
fld1: (u8, i64, u64, [i128; 7]),
fld2: Adt54,
fld3: i8,
fld4: i64,

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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: u16,
fld1: char,

},
Variant1{
fld0: (f64, i32),
fld1: Adt55,
fld2: Adt48,
fld3: i8,
fld4: i16,
fld5: (u8, i64, u64, [i128; 7]),
fld6: u8,
fld7: ([bool; 7], ((u8, i64, u64, [i128; 7]), (usize, u64, ([i128; 7], bool)), f32), i16, *mut ([i128; 7], bool), [u8; 1]),

}}

