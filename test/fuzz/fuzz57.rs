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
pub fn fn0(mut _1: u8,mut _2: i64) -> char {
mir! {
type RET = char;
let _3: bool;
let _4: [i16; 1];
let _5: Adt55;
let _6: isize;
let _7: (u128, char);
let _8: Adt49;
let _9: &'static i128;
let _10: *const (u16, u16);
let _11: [u8; 7];
let _12: usize;
let _13: usize;
let _14: i64;
let _15: isize;
let _16: Adt45;
let _17: Adt53;
let _18: ();
let _19: ();
{
_2 = !(-5164721730354150842_i64);
_1 = 89_u8 + 165_u8;
RET = '\u{d47a0}';
_2 = 3948532636336157230_i64 & (-6512244320304382509_i64);
RET = '\u{a57fb}';
_2 = -(-853337140889775544_i64);
_3 = false;
RET = '\u{1116c}';
RET = '\u{83a74}';
_2 = _1 as i64;
RET = '\u{e81db}';
_2 = 8214195703178689721_i64 * 4986989348749165251_i64;
_3 = false;
_4 = [(-23409_i16)];
RET = '\u{189f9}';
RET = '\u{e5f03}';
RET = '\u{15a9a}';
RET = '\u{5d31f}';
_5 = Adt55::Variant2 { fld0: _3 };
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.1 = RET;
_7.0 = 7_usize as u128;
_7.0 = 14927203406482556671_usize as u128;
_2 = (-5206800717468430595_i64);
_3 = Field::<bool>(Variant(_5, 2), 0);
RET = _7.1;
_7.1 = RET;
_2 = 4803109827230264144_i64 | 7825166276322448195_i64;
_7.1 = RET;
place!(Field::<bool>(Variant(_5, 2), 0)) = _3;
_4 = [13955_i16];
_1 = 26_u8;
_6 = (-9223372036854775808_isize);
_12 = 15151863866421502283_u64 as usize;
_11 = [_1,_1,_1,_1,_1,_1,_1];
_1 = 45_u8 ^ 2_u8;
_7.1 = RET;
RET = _7.1;
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463454151235394913435648 => bb10,
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
_1 = 167_u8 | 174_u8;
_6 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_5 = Adt55::Variant2 { fld0: _3 };
_14 = 2265359500_u32 as i64;
_7.1 = RET;
_14 = _2;
_7.0 = 271472166758496062663566045592086360452_u128;
_11 = [_1,_1,_1,_1,_1,_1,_1];
place!(Field::<bool>(Variant(_5, 2), 0)) = _2 > _14;
SetDiscriminant(_5, 0);
place!(Field::<u128>(Variant(_5, 0), 2)) = _7.0 << _1;
Call(_4 = fn1(_1, _3, _7, RET, _3, _7, Field::<u128>(Variant(_5, 0), 2), _6, _2, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<Adt45>(Variant(_5, 0), 4)).fld3 = [_7.1,RET,_7.1,_7.1,RET];
_6 = _1 as isize;
place!(Field::<*mut bool>(Variant(_5, 0), 6)) = core::ptr::addr_of_mut!(_3);
place!(Field::<Adt45>(Variant(_5, 0), 4)).fld1.1 = _2 as u16;
place!(Field::<Adt45>(Variant(_5, 0), 4)).fld2 = Field::<Adt45>(Variant(_5, 0), 4).fld1.1 << Field::<u128>(Variant(_5, 0), 2);
_7.1 = RET;
match _7.0 {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
271472166758496062663566045592086360452 => bb17,
_ => bb16
}
}
bb12 = {
_1 = 167_u8 | 174_u8;
_6 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_5 = Adt55::Variant2 { fld0: _3 };
_14 = 2265359500_u32 as i64;
_7.1 = RET;
_14 = _2;
_7.0 = 271472166758496062663566045592086360452_u128;
_11 = [_1,_1,_1,_1,_1,_1,_1];
place!(Field::<bool>(Variant(_5, 2), 0)) = _2 > _14;
SetDiscriminant(_5, 0);
place!(Field::<u128>(Variant(_5, 0), 2)) = _7.0 << _1;
Call(_4 = fn1(_1, _3, _7, RET, _3, _7, Field::<u128>(Variant(_5, 0), 2), _6, _2, _6), ReturnTo(bb11), UnwindUnreachable())
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
Return()
}
bb17 = {
_7.1 = RET;
_7.0 = !Field::<u128>(Variant(_5, 0), 2);
_2 = _14 ^ _14;
_7.1 = RET;
_7.0 = !Field::<u128>(Variant(_5, 0), 2);
_16.fld1.1 = Field::<Adt45>(Variant(_5, 0), 4).fld1.1 * Field::<Adt45>(Variant(_5, 0), 4).fld1.1;
_7.0 = !Field::<u128>(Variant(_5, 0), 2);
_16.fld2 = !_16.fld1.1;
Goto(bb18)
}
bb18 = {
Call(_18 = dump_var(0_usize, 7_usize, Move(_7), 3_usize, Move(_3), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u8,mut _2: bool,mut _3: (u128, char),mut _4: char,mut _5: bool,mut _6: (u128, char),mut _7: u128,mut _8: isize,mut _9: i64,mut _10: isize) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _11: u128;
let _12: bool;
let _13: isize;
let _14: [char; 5];
let _15: Adt46;
let _16: i128;
let _17: isize;
let _18: u64;
let _19: bool;
let _20: f64;
let _21: usize;
let _22: u16;
let _23: Adt55;
let _24: f32;
let _25: [i64; 2];
let _26: Adt50;
let _27: Adt49;
let _28: (f32, i32);
let _29: [u64; 6];
let _30: u128;
let _31: f32;
let _32: *const i64;
let _33: isize;
let _34: [i16; 1];
let _35: u16;
let _36: isize;
let _37: [u8; 4];
let _38: bool;
let _39: bool;
let _40: isize;
let _41: (u128, char);
let _42: [char; 5];
let _43: ();
let _44: ();
{
_2 = _5;
_6 = _3;
_10 = _8;
_5 = _10 < _10;
_10 = _3.1 as isize;
_3.0 = _7;
_3.1 = _6.1;
_2 = !_5;
RET = [(-21775_i16)];
_10 = _8 | _8;
_11 = !_3.0;
_10 = 2583_u16 as isize;
_11 = _3.0;
RET = [(-16606_i16)];
Call(_12 = fn2(_8, _7, _4, _3.0, _5, _10, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3.0 = _10 as u128;
_3.0 = !_11;
_3.1 = _6.1;
_4 = _6.1;
_4 = _3.1;
_3.0 = _7;
_4 = _6.1;
_3 = (_11, _6.1);
_6.1 = _4;
_1 = 73_u8 - 65_u8;
_6 = (_11, _3.1);
_11 = _3.0;
_7 = _6.0 * _6.0;
_4 = _3.1;
_9 = _1 as i64;
Goto(bb2)
}
bb2 = {
_9 = (-803330302612673300_i64) ^ 8047807999521988881_i64;
_3 = (_6.0, _4);
_14 = [_6.1,_4,_6.1,_3.1,_3.1];
_10 = _11 as isize;
_6.1 = _4;
_14 = [_3.1,_3.1,_4,_6.1,_3.1];
_3.1 = _4;
_11 = _7 - _3.0;
RET = [21529_i16];
_3 = (_11, _6.1);
Goto(bb3)
}
bb3 = {
_7 = !_6.0;
_6.1 = _3.1;
_3 = (_6.0, _6.1);
_5 = _12;
_6 = (_7, _4);
_17 = _8 & _10;
_7 = _6.0 & _11;
Goto(bb4)
}
bb4 = {
_3.0 = 1_usize as u128;
_8 = !_17;
_6 = (_11, _3.1);
_6.1 = _4;
_14 = [_3.1,_6.1,_3.1,_3.1,_4];
_4 = _3.1;
_4 = _3.1;
_7 = _11;
_6 = (_7, _4);
_3.0 = _6.0 | _11;
_3 = _6;
_1 = _3.1 as u8;
_13 = (-160286924402116416590445234492654252121_i128) as isize;
_16 = -(-152545413327375192326024371999300887088_i128);
_16 = 345858420_u32 as i128;
_6.1 = _4;
_3.1 = _6.1;
_6.0 = !_7;
RET = [(-11247_i16)];
_5 = !_12;
_6.0 = _7 << _11;
_7 = 17278203822635196602_u64 as u128;
Goto(bb5)
}
bb5 = {
_14 = [_4,_4,_6.1,_3.1,_6.1];
_12 = _5 <= _5;
_13 = _17 + _8;
_16 = 1564_i16 as i128;
_7 = !_11;
_13 = (-6552_i16) as isize;
_6 = _3;
_12 = _17 < _17;
_20 = _16 as f64;
_3.0 = _16 as u128;
_19 = _5;
_9 = 2517435134602470636_i64;
_20 = 63_i8 as f64;
_6 = _3;
_16 = _20 as i128;
Goto(bb6)
}
bb6 = {
_21 = 9284656966168968554_usize;
_11 = _7;
_20 = 8724_u16 as f64;
_16 = (-167282823269185466030974555171824535013_i128);
_3 = _6;
_5 = _19;
_22 = 21010_u16 - 26426_u16;
_18 = !14382412160340311677_u64;
_11 = _7 << _7;
_3.0 = _7 - _11;
_12 = !_5;
_5 = !_19;
_24 = _21 as f32;
_17 = (-25494_i16) as isize;
_5 = _19 == _19;
_18 = 1238150826925371098_u64 | 3221087051632556706_u64;
_3.1 = _6.1;
_19 = _5;
_22 = 21534_u16 & 5714_u16;
_8 = _10 + _10;
_7 = _3.0 | _3.0;
_13 = _8;
_12 = _11 != _11;
match _9 {
0 => bb7,
2517435134602470636 => bb9,
_ => bb8
}
}
bb7 = {
_3.0 = _10 as u128;
_3.0 = !_11;
_3.1 = _6.1;
_4 = _6.1;
_4 = _3.1;
_3.0 = _7;
_4 = _6.1;
_3 = (_11, _6.1);
_6.1 = _4;
_1 = 73_u8 - 65_u8;
_6 = (_11, _3.1);
_11 = _3.0;
_7 = _6.0 * _6.0;
_4 = _3.1;
_9 = _1 as i64;
Goto(bb2)
}
bb8 = {
_7 = !_6.0;
_6.1 = _3.1;
_3 = (_6.0, _6.1);
_5 = _12;
_6 = (_7, _4);
_17 = _8 & _10;
_7 = _6.0 & _11;
Goto(bb4)
}
bb9 = {
_17 = _13;
_21 = 5233265118359606335_usize;
_7 = _3.0;
_31 = _24 + _24;
_25 = [_9,_9];
_28 = (_31, (-870213919_i32));
_16 = _9 as i128;
_8 = _13;
_16 = _18 as i128;
_29 = [_18,_18,_18,_18,_18,_18];
_13 = _19 as isize;
_28.1 = _1 as i32;
_18 = 15729624071390710471_u64 - 12778726767409902848_u64;
_28 = (_24, 1447614757_i32);
_17 = 869949274_u32 as isize;
Goto(bb10)
}
bb10 = {
_28.0 = _24;
_4 = _6.1;
_6.0 = !_3.0;
_9 = _18 as i64;
_3 = (_6.0, _6.1);
_22 = 64544_u16;
_32 = core::ptr::addr_of!(_9);
_12 = !_5;
match _21 {
0 => bb9,
1 => bb2,
2 => bb3,
5233265118359606335 => bb12,
_ => bb11
}
}
bb11 = {
_3.0 = _10 as u128;
_3.0 = !_11;
_3.1 = _6.1;
_4 = _6.1;
_4 = _3.1;
_3.0 = _7;
_4 = _6.1;
_3 = (_11, _6.1);
_6.1 = _4;
_1 = 73_u8 - 65_u8;
_6 = (_11, _3.1);
_11 = _3.0;
_7 = _6.0 * _6.0;
_4 = _3.1;
_9 = _1 as i64;
Goto(bb2)
}
bb12 = {
RET = [13004_i16];
_10 = _13;
_20 = _24 as f64;
_28.0 = _31;
_1 = !72_u8;
_34 = [(-30140_i16)];
_30 = _3.0 - _7;
_14 = [_3.1,_3.1,_3.1,_4,_4];
_6.0 = _30 * _3.0;
_35 = !_22;
_23 = Adt55::Variant2 { fld0: _5 };
_11 = _6.0 + _3.0;
_10 = -_13;
_28.0 = _31;
_24 = _21 as f32;
_28.0 = _24 - _31;
_2 = !Field::<bool>(Variant(_23, 2), 0);
_16 = _31 as i128;
_36 = _10 ^ _13;
_34 = RET;
SetDiscriminant(_23, 1);
_13 = _10 << _10;
_9 = _28.1 as i64;
_30 = _11;
RET = [27925_i16];
_3.1 = _6.1;
match _22 {
0 => bb1,
1 => bb7,
2 => bb13,
3 => bb14,
64544 => bb16,
_ => bb15
}
}
bb13 = {
_3.0 = _10 as u128;
_3.0 = !_11;
_3.1 = _6.1;
_4 = _6.1;
_4 = _3.1;
_3.0 = _7;
_4 = _6.1;
_3 = (_11, _6.1);
_6.1 = _4;
_1 = 73_u8 - 65_u8;
_6 = (_11, _3.1);
_11 = _3.0;
_7 = _6.0 * _6.0;
_4 = _3.1;
_9 = _1 as i64;
Goto(bb2)
}
bb14 = {
_14 = [_4,_4,_6.1,_3.1,_6.1];
_12 = _5 <= _5;
_13 = _17 + _8;
_16 = 1564_i16 as i128;
_7 = !_11;
_13 = (-6552_i16) as isize;
_6 = _3;
_12 = _17 < _17;
_20 = _16 as f64;
_3.0 = _16 as u128;
_19 = _5;
_9 = 2517435134602470636_i64;
_20 = 63_i8 as f64;
_6 = _3;
_16 = _20 as i128;
Goto(bb6)
}
bb15 = {
_7 = !_6.0;
_6.1 = _3.1;
_3 = (_6.0, _6.1);
_5 = _12;
_6 = (_7, _4);
_17 = _8 & _10;
_7 = _6.0 & _11;
Goto(bb4)
}
bb16 = {
place!(Field::<(f64,)>(Variant(_23, 1), 5)).0 = _30 as f64;
place!(Field::<isize>(Variant(_23, 1), 2)) = _36;
_41.1 = _3.1;
_7 = _30;
place!(Field::<(*const (u16, u16), u16)>(Variant(_23, 1), 1)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_23, 1), 4)));
Goto(bb17)
}
bb17 = {
Call(_43 = dump_var(1_usize, 22_usize, Move(_22), 34_usize, Move(_34), 11_usize, Move(_11), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(1_usize, 21_usize, Move(_21), 10_usize, Move(_10), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_43 = dump_var(1_usize, 14_usize, Move(_14), 8_usize, Move(_8), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_43 = dump_var(1_usize, 17_usize, Move(_17), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: u128,mut _3: char,mut _4: u128,mut _5: bool,mut _6: isize,mut _7: isize) -> bool {
mir! {
type RET = bool;
let _8: [u64; 6];
let _9: [u8; 7];
let _10: f64;
let _11: usize;
let _12: isize;
let _13: ();
let _14: ();
{
RET = !_5;
RET = _2 != _2;
_1 = 118_i8 as isize;
_1 = 42_u8 as isize;
_4 = _2;
_5 = !RET;
_4 = _2 & _2;
RET = _7 != _7;
_6 = _7;
RET = _2 > _4;
Goto(bb1)
}
bb1 = {
_8 = [8054201937778526828_u64,12895279504589588757_u64,14774250058863417951_u64,3045729362883279784_u64,16911408905580159829_u64,17165865739075845734_u64];
_2 = _4 | _4;
Goto(bb2)
}
bb2 = {
_8 = [12175715989257605327_u64,12958657304096687143_u64,8344018674531024078_u64,2823289436649074833_u64,10267631187404440470_u64,11458019763525384514_u64];
_9 = [78_u8,138_u8,20_u8,175_u8,26_u8,128_u8,84_u8];
_7 = !_1;
_5 = RET;
RET = _5;
_9 = [208_u8,63_u8,184_u8,225_u8,251_u8,176_u8,56_u8];
_8 = [3226968142158094149_u64,4150944150505435570_u64,10791275086177322722_u64,7301710071531336113_u64,3172509460192051154_u64,9570918841591334609_u64];
_4 = 2648387857538634660_i64 as u128;
_9 = [110_u8,179_u8,202_u8,229_u8,25_u8,58_u8,63_u8];
_9 = [211_u8,174_u8,236_u8,64_u8,143_u8,41_u8,33_u8];
Call(RET = fn3(_7, _2, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _7;
_10 = 27240922272385450583076881906336906730_i128 as f64;
_6 = _7;
_6 = !_1;
_5 = RET == RET;
_1 = -_7;
_3 = '\u{381d}';
_10 = 4565994693498826438_u64 as f64;
_9 = [52_u8,37_u8,186_u8,88_u8,76_u8,160_u8,93_u8];
_12 = _6 * _6;
_2 = _3 as u128;
_7 = _1;
Goto(bb4)
}
bb4 = {
Call(_13 = dump_var(2_usize, 4_usize, Move(_4), 1_usize, Move(_1), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_13 = dump_var(2_usize, 2_usize, Move(_2), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: u128,mut _3: [u8; 7]) -> bool {
mir! {
type RET = bool;
let _4: Adt42;
let _5: [u8; 4];
let _6: [char; 5];
let _7: (f32, i32);
let _8: i32;
let _9: bool;
let _10: Adt54;
let _11: (f64,);
let _12: Adt44;
let _13: [i16; 1];
let _14: (u128, char);
let _15: bool;
let _16: isize;
let _17: [u8; 7];
let _18: u16;
let _19: Adt47;
let _20: u64;
let _21: u128;
let _22: [u8; 4];
let _23: f64;
let _24: (bool, isize);
let _25: isize;
let _26: [u64; 6];
let _27: [u8; 7];
let _28: (f32, i32);
let _29: [u8; 7];
let _30: char;
let _31: usize;
let _32: f32;
let _33: (f64,);
let _34: (f64,);
let _35: usize;
let _36: [char; 5];
let _37: i8;
let _38: ((u16, u16), i8, &'static i128, usize, [u8; 7]);
let _39: isize;
let _40: *mut i32;
let _41: Adt44;
let _42: ();
let _43: ();
{
_2 = !63279404925427574420087113942379754150_u128;
RET = true & false;
RET = false;
_2 = 322691549952217319299735445606812228711_u128 * 138857228613650747472582545836773609037_u128;
_3 = [37_u8,100_u8,117_u8,172_u8,5_u8,152_u8,84_u8];
_1 = (-20_isize) & 106_isize;
_1 = (-106_isize);
RET = true;
RET = _1 > _1;
RET = true;
_5 = [114_u8,255_u8,155_u8,76_u8];
_5 = [51_u8,53_u8,220_u8,25_u8];
Goto(bb1)
}
bb1 = {
_2 = !31445809326517834180897732572371235248_u128;
RET = !true;
_1 = (-7855856002968301948_i64) as isize;
_7.0 = 11_i8 as f32;
_5 = [250_u8,81_u8,96_u8,254_u8];
_7.0 = 2038937500_i32 as f32;
_8 = 1744666188_i32 | 1059567437_i32;
_3 = [236_u8,203_u8,96_u8,146_u8,31_u8,226_u8,95_u8];
RET = _1 == _1;
_6 = ['\u{9c620}','\u{c59bc}','\u{30bb4}','\u{97056}','\u{12b82}'];
_8 = (-742618858_i32);
_7.0 = (-5899409681492360016_i64) as f32;
_8 = 720873434_i32;
_1 = 9223372036854775807_isize << _8;
_7.1 = -_8;
_5 = [115_u8,99_u8,6_u8,91_u8];
_7.0 = 3005293744_u32 as f32;
_7.1 = 48926_u16 as i32;
_9 = !RET;
_6 = ['\u{fae11}','\u{a3c5c}','\u{c6b12}','\u{b28e8}','\u{71e5b}'];
_6 = ['\u{3f8dd}','\u{c3dcc}','\u{8393b}','\u{4c07b}','\u{d4351}'];
_5 = [58_u8,165_u8,147_u8,252_u8];
_8 = _7.1;
RET = _2 > _2;
_5 = [196_u8,197_u8,152_u8,15_u8];
Goto(bb2)
}
bb2 = {
_9 = !RET;
_6 = ['\u{d6cde}','\u{1834f}','\u{10b879}','\u{f7481}','\u{3ccae}'];
_7.0 = 903269993936021917_u64 as f32;
_8 = _7.1;
RET = _9;
_12.fld0.1 = _7.0 as isize;
_2 = 281458001474823312269353648306428276306_u128 * 235460228357415594349862600142715403785_u128;
_12.fld0.1 = _2 as isize;
_11.0 = 3155057294_u32 as f64;
_1 = 104_u8 as isize;
_11.0 = _1 as f64;
_12.fld1 = core::ptr::addr_of_mut!(_7.1);
Goto(bb3)
}
bb3 = {
_6 = ['\u{439a9}','\u{6119b}','\u{e0718}','\u{3dce0}','\u{71430}'];
_2 = !237392242582069860246932053970862784942_u128;
_12.fld1 = core::ptr::addr_of_mut!(_7.1);
_13 = [20288_i16];
_4 = Adt42::Variant0 { fld0: _8,fld1: _6,fld2: 2_usize,fld3: _3,fld4: _11.0 };
_12.fld1 = core::ptr::addr_of_mut!(_7.1);
_12.fld1 = core::ptr::addr_of_mut!(_8);
_11.0 = Field::<f64>(Variant(_4, 0), 4) + Field::<f64>(Variant(_4, 0), 4);
_14 = (_2, '\u{cefcc}');
_6 = [_14.1,_14.1,_14.1,_14.1,_14.1];
_14.1 = '\u{8dcf9}';
_7.1 = Field::<i32>(Variant(_4, 0), 0);
place!(Field::<usize>(Variant(_4, 0), 2)) = 3792539927993882615_u64 as usize;
place!(Field::<[char; 5]>(Variant(_4, 0), 1)) = _6;
_12.fld1 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_4, 0), 0)));
_12.fld0 = (RET, _1);
_1 = !_12.fld0.1;
_8 = _7.1;
_12.fld5 = [9787808421554069686_u64,16744406193848419302_u64,441178678929087118_u64,14238682644653507363_u64,9590183540032410675_u64,1210094300825486748_u64];
_14.0 = _2;
RET = _12.fld0.0 | _9;
_9 = !RET;
_14.1 = '\u{9047f}';
_8 = !Field::<i32>(Variant(_4, 0), 0);
_15 = !_9;
_14.0 = 83807057729497382011948383024290608517_i128 as u128;
Goto(bb4)
}
bb4 = {
_2 = _14.0 ^ _14.0;
_11 = (Field::<f64>(Variant(_4, 0), 4),);
RET = _9;
_12.fld0 = (_15, _1);
place!(Field::<f64>(Variant(_4, 0), 4)) = _11.0 - _11.0;
place!(Field::<[u8; 7]>(Variant(_4, 0), 3)) = [104_u8,143_u8,123_u8,169_u8,152_u8,65_u8,139_u8];
_7.0 = _7.1 as f32;
_12.fld0 = (_15, _1);
_12.fld0 = (_9, _1);
_14.1 = '\u{1b95f}';
RET = !_12.fld0.0;
RET = _9 ^ _15;
_5 = [35_u8,100_u8,18_u8,178_u8];
_10 = Adt54::Variant0 { fld0: _15,fld1: _13,fld2: 1543015832_u32,fld3: 7_i8,fld4: _7.0,fld5: _12.fld0 };
_11 = (Field::<f64>(Variant(_4, 0), 4),);
_16 = !_12.fld0.1;
_7 = (Field::<f32>(Variant(_10, 0), 4), _8);
Goto(bb5)
}
bb5 = {
place!(Field::<i32>(Variant(_4, 0), 0)) = _8 + _8;
_7.1 = _8 >> _2;
RET = !Field::<bool>(Variant(_10, 0), 0);
_17 = [113_u8,42_u8,16_u8,95_u8,34_u8,232_u8,147_u8];
_12.fld4 = core::ptr::addr_of_mut!(_9);
_11.0 = 5478187429850736226_i64 as f64;
_7 = (Field::<f32>(Variant(_10, 0), 4), Field::<i32>(Variant(_4, 0), 0));
place!(Field::<[u8; 7]>(Variant(_4, 0), 3)) = [243_u8,123_u8,255_u8,147_u8,194_u8,89_u8,162_u8];
_18 = !42795_u16;
_7 = (Field::<f32>(Variant(_10, 0), 4), Field::<i32>(Variant(_4, 0), 0));
_17 = _3;
_10 = Adt54::Variant0 { fld0: _9,fld1: _13,fld2: 1619766106_u32,fld3: 47_i8,fld4: _7.0,fld5: _12.fld0 };
_14.1 = '\u{e580b}';
place!(Field::<i8>(Variant(_10, 0), 3)) = 54_i8;
_14 = (_2, '\u{86ecf}');
_7.1 = -Field::<i32>(Variant(_4, 0), 0);
_9 = Field::<bool>(Variant(_10, 0), 0);
place!(Field::<[u8; 7]>(Variant(_4, 0), 3)) = [154_u8,103_u8,186_u8,30_u8,208_u8,220_u8,229_u8];
place!(Field::<(bool, isize)>(Variant(_10, 0), 5)) = (_12.fld0.0, _1);
Goto(bb6)
}
bb6 = {
place!(Field::<i32>(Variant(_4, 0), 0)) = _7.1 & _8;
_11.0 = -Field::<f64>(Variant(_4, 0), 4);
place!(Field::<usize>(Variant(_4, 0), 2)) = 3_usize;
_20 = 148_u8 as u64;
_12.fld0 = (RET, _1);
_21 = _14.0;
SetDiscriminant(_4, 1);
_24 = (_15, Field::<(bool, isize)>(Variant(_10, 0), 5).1);
_24.1 = _1;
place!(Field::<i8>(Variant(_10, 0), 3)) = (-100_i8);
_11.0 = 24278_i16 as f64;
_22 = _5;
place!(Field::<i128>(Variant(_4, 1), 2)) = 100966565800397246975258791729326459522_i128 >> _14.0;
place!(Field::<i16>(Variant(_4, 1), 3)) = 21066_i16 - (-24055_i16);
place!(Field::<u32>(Variant(_10, 0), 2)) = _11.0 as u32;
_23 = _20 as f64;
_24.1 = _12.fld0.1 + _12.fld0.1;
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_4, 1), 1)));
_12.fld5 = [_20,_20,_20,_20,_20,_20];
_22 = _5;
_24.0 = !_15;
Goto(bb7)
}
bb7 = {
_18 = 4501_u16 | 43139_u16;
_21 = _14.0 * _14.0;
_8 = _7.1 << _12.fld0.1;
place!(Field::<i128>(Variant(_4, 1), 2)) = 5948285826739102805_i64 as i128;
_20 = 14496586968430384661_usize as u64;
place!(Field::<[i16; 1]>(Variant(_10, 0), 1)) = [Field::<i16>(Variant(_4, 1), 3)];
_12.fld0 = _24;
SetDiscriminant(_10, 0);
_12.fld0 = _24;
_24.1 = -_12.fld0.1;
_1 = _16 & _24.1;
Goto(bb8)
}
bb8 = {
_19 = Adt47::Variant1 { fld0: _12.fld0.0,fld1: _17 };
_7.1 = _8;
place!(Field::<(bool, isize)>(Variant(_10, 0), 5)) = _24;
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).1 = !_18;
_7.1 = _8;
_24.0 = RET ^ _12.fld0.0;
_28.0 = _7.0;
place!(Field::<(u16, u16)>(Variant(_4, 1), 1)).0 = !_18;
_25 = _12.fld0.1;
place!(Field::<(u16, u16)>(Variant(_4, 1), 1)).1 = Field::<(u16, u16)>(Variant(_4, 1), 1).0;
_2 = _14.0 | _21;
place!(Field::<bool>(Variant(_10, 0), 0)) = _9;
place!(Field::<[i16; 1]>(Variant(_10, 0), 1)) = _13;
Goto(bb9)
}
bb9 = {
_7 = (_28.0, _8);
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_4, 1), 1)));
_11 = (_23,);
_12.fld5 = [_20,_20,_20,_20,_20,_20];
_14 = (_2, '\u{f002a}');
place!(Field::<i16>(Variant(_4, 1), 3)) = 1944_i16;
place!(Field::<i16>(Variant(_4, 1), 3)) = 15219_i16;
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_4, 1), 1)));
place!(Field::<[u8; 7]>(Variant(_19, 1), 1)) = _3;
_24 = (Field::<bool>(Variant(_10, 0), 0), _1);
_28 = (_7.0, _8);
_12.fld1 = core::ptr::addr_of_mut!(_28.1);
_22 = _5;
_7.0 = _28.0 * _28.0;
place!(Field::<(bool, isize)>(Variant(_10, 0), 5)).0 = !_15;
_11 = (_23,);
RET = !_15;
_28.1 = 2758394427_u32 as i32;
_25 = Field::<(bool, isize)>(Variant(_10, 0), 5).1;
_25 = _24.1;
_12.fld0 = Field::<(bool, isize)>(Variant(_10, 0), 5);
_14.1 = '\u{4b54e}';
_30 = _14.1;
Call(_33 = fn4(_2, _17, _17, _7.0, _24.1, _1, Move(_19), _30, _12.fld0, _24.1, _14, _24.1, _24.0, _12.fld4), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_34.0 = _33.0;
place!(Field::<u32>(Variant(_10, 0), 2)) = 1165822286_u32 - 170117988_u32;
_16 = _12.fld0.1;
place!(Field::<(bool, isize)>(Variant(_10, 0), 5)) = _24;
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).1 = Field::<(u16, u16)>(Variant(_4, 1), 1).1 ^ _18;
_33.0 = _34.0;
_25 = !_1;
_35 = 5_usize >> _24.1;
place!(Field::<(u16, u16)>(Variant(_4, 1), 1)) = (Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0).1, Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0).1);
_12.fld5 = [_20,_20,_20,_20,_20,_20];
_20 = !14753277895396590204_u64;
place!(Field::<f32>(Variant(_10, 0), 4)) = Field::<i128>(Variant(_4, 1), 2) as f32;
_29 = [70_u8,237_u8,70_u8,63_u8,154_u8,15_u8,66_u8];
_28.0 = -_7.0;
_8 = 112_u8 as i32;
place!(Field::<(u16, u16)>(Variant(_4, 1), 1)).0 = Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0).1 ^ _18;
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_4, 1), 1)));
_18 = !Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0).1;
_24.0 = !Field::<(bool, isize)>(Variant(_10, 0), 5).0;
_5 = [134_u8,203_u8,51_u8,192_u8];
place!(Field::<f32>(Variant(_10, 0), 4)) = _28.0;
place!(Field::<[i16; 1]>(Variant(_10, 0), 1)) = [Field::<i16>(Variant(_4, 1), 3)];
_14 = (_21, _30);
place!(Field::<(u16, u16)>(Variant(_4, 1), 1)) = (_18, _18);
_17 = _3;
_9 = !RET;
_18 = Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0).1;
_1 = !_12.fld0.1;
_6 = [_14.1,_30,_14.1,_14.1,_30];
match Field::<i16>(Variant(_4, 1), 3) {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb9,
15219 => bb12,
_ => bb11
}
}
bb11 = {
_7 = (_28.0, _8);
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_4, 1), 1)));
_11 = (_23,);
_12.fld5 = [_20,_20,_20,_20,_20,_20];
_14 = (_2, '\u{f002a}');
place!(Field::<i16>(Variant(_4, 1), 3)) = 1944_i16;
place!(Field::<i16>(Variant(_4, 1), 3)) = 15219_i16;
place!(Field::<(*const (u16, u16), u16)>(Variant(_4, 1), 0)).0 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_4, 1), 1)));
place!(Field::<[u8; 7]>(Variant(_19, 1), 1)) = _3;
_24 = (Field::<bool>(Variant(_10, 0), 0), _1);
_28 = (_7.0, _8);
_12.fld1 = core::ptr::addr_of_mut!(_28.1);
_22 = _5;
_7.0 = _28.0 * _28.0;
place!(Field::<(bool, isize)>(Variant(_10, 0), 5)).0 = !_15;
_11 = (_23,);
RET = !_15;
_28.1 = 2758394427_u32 as i32;
_25 = Field::<(bool, isize)>(Variant(_10, 0), 5).1;
_25 = _24.1;
_12.fld0 = Field::<(bool, isize)>(Variant(_10, 0), 5);
_14.1 = '\u{4b54e}';
_30 = _14.1;
Call(_33 = fn4(_2, _17, _17, _7.0, _24.1, _1, Move(_19), _30, _12.fld0, _24.1, _14, _24.1, _24.0, _12.fld4), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
SetDiscriminant(_4, 0);
place!(Field::<usize>(Variant(_4, 0), 2)) = !_35;
_12.fld0.0 = Field::<(bool, isize)>(Variant(_10, 0), 5).0 | _24.0;
place!(Field::<i8>(Variant(_10, 0), 3)) = _18 as i8;
_31 = Field::<usize>(Variant(_4, 0), 2) + _35;
_33 = _34;
SetDiscriminant(_10, 0);
place!(Field::<[char; 5]>(Variant(_4, 0), 1)) = [_30,_14.1,_30,_14.1,_14.1];
_34.0 = -_33.0;
place!(Field::<bool>(Variant(_10, 0), 0)) = !_9;
_12.fld5 = [_20,_20,_20,_20,_20,_20];
_7.0 = _18 as f32;
place!(Field::<i8>(Variant(_10, 0), 3)) = -(-22_i8);
place!(Field::<f32>(Variant(_10, 0), 4)) = _28.0 + _7.0;
place!(Field::<i32>(Variant(_4, 0), 0)) = _20 as i32;
_5 = [16_u8,50_u8,38_u8,212_u8];
_7.0 = Field::<f32>(Variant(_10, 0), 4);
_36 = [_14.1,_30,_30,_14.1,_30];
_5 = [114_u8,191_u8,93_u8,116_u8];
_24.1 = 441109676_u32 as isize;
_33.0 = -_34.0;
_37 = Field::<i8>(Variant(_10, 0), 3) | Field::<i8>(Variant(_10, 0), 3);
_20 = !8480940011479305566_u64;
_10 = Adt54::Variant0 { fld0: _12.fld0.0,fld1: _13,fld2: 1932812449_u32,fld3: _37,fld4: _7.0,fld5: _12.fld0 };
place!(Field::<usize>(Variant(_4, 0), 2)) = _31 + _31;
_11 = (_34.0,);
place!(Field::<(bool, isize)>(Variant(_10, 0), 5)).0 = !Field::<bool>(Variant(_10, 0), 0);
Goto(bb13)
}
bb13 = {
_26 = _12.fld5;
_33.0 = -_34.0;
_28.0 = -_7.0;
place!(Field::<bool>(Variant(_10, 0), 0)) = !_12.fld0.0;
RET = _34.0 <= _34.0;
place!(Field::<f64>(Variant(_4, 0), 4)) = _33.0 * _34.0;
_7.1 = -_28.1;
_28 = (Field::<f32>(Variant(_10, 0), 4), _7.1);
place!(Field::<[u8; 7]>(Variant(_4, 0), 3)) = [161_u8,163_u8,151_u8,23_u8,44_u8,115_u8,78_u8];
place!(Field::<[i16; 1]>(Variant(_10, 0), 1)) = [(-26503_i16)];
_7.0 = Field::<f32>(Variant(_10, 0), 4);
_24 = _12.fld0;
_9 = _33.0 <= Field::<f64>(Variant(_4, 0), 4);
_1 = _12.fld0.1 >> _18;
_10 = Adt54::Variant0 { fld0: RET,fld1: _13,fld2: 3022356692_u32,fld3: _37,fld4: _28.0,fld5: _24 };
place!(Field::<i32>(Variant(_4, 0), 0)) = _28.1;
_17 = _3;
_26 = [_20,_20,_20,_20,_20,_20];
_36 = [_30,_14.1,_30,_14.1,_30];
_33.0 = Field::<f64>(Variant(_4, 0), 4);
_32 = _12.fld0.1 as f32;
_12.fld0.0 = !_9;
_12.fld0.1 = 16181_i16 as isize;
_11.0 = Field::<f64>(Variant(_4, 0), 4);
_12.fld4 = core::ptr::addr_of_mut!(_12.fld0.0);
_28 = (_7.0, Field::<i32>(Variant(_4, 0), 0));
_34.0 = Field::<f64>(Variant(_4, 0), 4);
_40 = core::ptr::addr_of_mut!(_28.1);
Goto(bb14)
}
bb14 = {
Call(_42 = dump_var(3_usize, 15_usize, Move(_15), 31_usize, Move(_31), 16_usize, Move(_16), 36_usize, Move(_36)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_42 = dump_var(3_usize, 37_usize, Move(_37), 18_usize, Move(_18), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(3_usize, 14_usize, Move(_14), 29_usize, Move(_29), 17_usize, Move(_17), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u128,mut _2: [u8; 7],mut _3: [u8; 7],mut _4: f32,mut _5: isize,mut _6: isize,mut _7: Adt47,mut _8: char,mut _9: (bool, isize),mut _10: isize,mut _11: (u128, char),mut _12: isize,mut _13: bool,mut _14: *mut bool) -> (f64,) {
mir! {
type RET = (f64,);
let _15: bool;
let _16: char;
let _17: (u128, char);
let _18: [u8; 4];
let _19: f32;
let _20: (*const (u16, u16), u16);
let _21: (u16, u16);
let _22: (*const (u16, u16), u16);
let _23: usize;
let _24: [i64; 2];
let _25: usize;
let _26: isize;
let _27: *const (u16, u16);
let _28: u128;
let _29: bool;
let _30: u8;
let _31: isize;
let _32: bool;
let _33: f64;
let _34: f64;
let _35: Adt49;
let _36: &'static i128;
let _37: f32;
let _38: isize;
let _39: f64;
let _40: (f64,);
let _41: i128;
let _42: char;
let _43: [char; 5];
let _44: (f32, i32);
let _45: i8;
let _46: f32;
let _47: f64;
let _48: *const u16;
let _49: isize;
let _50: Adt47;
let _51: (u16, u16);
let _52: (f64,);
let _53: bool;
let _54: ();
let _55: ();
{
RET.0 = 97_i8 as f64;
_9 = (Field::<bool>(Variant(_7, 1), 0), _6);
(*_14) = Field::<bool>(Variant(_7, 1), 0);
RET.0 = 61_u8 as f64;
_11.0 = (-77_i8) as u128;
_8 = _11.1;
RET.0 = 58692_u16 as f64;
_12 = !_6;
_10 = !_12;
place!(Field::<[u8; 7]>(Variant(_7, 1), 1)) = [36_u8,205_u8,33_u8,197_u8,64_u8,75_u8,65_u8];
_12 = _5 >> _1;
_13 = _9.0;
place!(Field::<[u8; 7]>(Variant(_7, 1), 1)) = [30_u8,148_u8,16_u8,191_u8,52_u8,71_u8,218_u8];
_8 = _11.1;
_11.1 = _8;
SetDiscriminant(_7, 0);
RET.0 = (-25929_i16) as f64;
_1 = _11.0;
_11.0 = _1;
place!(Field::<usize>(Variant(_7, 0), 2)) = 2_usize - 14402244965613555217_usize;
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld0 = ((*_14), _10);
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld0.0 = (*_14);
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld5 = [18196247982835233302_u64,5439367299961651362_u64,15478604345076883712_u64,18003559738759908560_u64,395742926569370522_u64,6572862584344562422_u64];
_17.1 = _8;
RET.0 = Field::<usize>(Variant(_7, 0), 2) as f64;
_5 = 3606865498221572277_i64 as isize;
place!(Field::<i8>(Variant(_7, 0), 3)) = 18620_u16 as i8;
_16 = _11.1;
Call(place!(Field::<i8>(Variant(_7, 0), 3)) = fn5(_16, Field::<usize>(Variant(_7, 0), 2), RET, _14, RET.0, (*_14), _10, _13, (*_14), Field::<Adt44>(Variant(_7, 0), 4).fld0.1, _9, _12, Field::<Adt44>(Variant(_7, 0), 4).fld0, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<*const (f64,)>(Variant(_7, 0), 1)) = core::ptr::addr_of!(RET);
_6 = 1415784906_i32 as isize;
_16 = _17.1;
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld0 = (_9.0, _12);
place!(Field::<[u8; 7]>(Variant(_7, 0), 6)) = [127_u8,241_u8,233_u8,71_u8,26_u8,157_u8,133_u8];
_3 = [84_u8,25_u8,114_u8,87_u8,212_u8,227_u8,250_u8];
_6 = _4 as isize;
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld0.0 = _9.0;
_10 = 2460786350_u32 as isize;
place!(Field::<f32>(Variant(_7, 0), 5)) = 104_u8 as f32;
Goto(bb2)
}
bb2 = {
_9.1 = (-245328008746878124_i64) as isize;
_10 = Field::<Adt44>(Variant(_7, 0), 4).fld0.1;
_9.1 = _10 >> _10;
_17 = (_11.0, _11.1);
_4 = _1 as f32;
_6 = _9.1 & _9.1;
_11 = _17;
_1 = Field::<f32>(Variant(_7, 0), 5) as u128;
_6 = _12 - _12;
(*_14) = _12 < _12;
_16 = _11.1;
_15 = !(*_14);
_11.1 = _8;
_9.0 = !(*_14);
_13 = !_15;
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld5 = [11103730542163874104_u64,11945923104640603207_u64,10110809231904515507_u64,8975031600856143152_u64,17100615245829350274_u64,1683236124948783641_u64];
_11.1 = _16;
_9.0 = !_15;
_20.1 = 8668545593559926677_i64 as u16;
_9.0 = !_15;
_21.0 = _20.1;
Goto(bb3)
}
bb3 = {
_21.0 = !_20.1;
_21.1 = _21.0;
_15 = (*_14);
_21 = (_20.1, _20.1);
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld4 = core::ptr::addr_of_mut!((*_14));
_25 = 80918202836651306302753821769517367247_i128 as usize;
place!(Field::<[u8; 7]>(Variant(_7, 0), 6)) = [65_u8,147_u8,54_u8,91_u8,158_u8,232_u8,204_u8];
_18 = [81_u8,188_u8,91_u8,30_u8];
RET.0 = _1 as f64;
place!(Field::<i8>(Variant(_7, 0), 3)) = !114_i8;
_17 = (_11.0, _8);
RET.0 = 10129002838421537997_u64 as f64;
_2 = [16_u8,8_u8,232_u8,8_u8,218_u8,223_u8,173_u8];
_20.0 = core::ptr::addr_of!(_21);
_17.0 = 1172244241_u32 as u128;
_26 = _6;
_22 = _20;
place!(Field::<[u8; 7]>(Variant(_7, 0), 6)) = [192_u8,23_u8,108_u8,37_u8,65_u8,8_u8,65_u8];
Goto(bb4)
}
bb4 = {
_28 = _1 - _1;
(*_14) = _10 >= _6;
_29 = (*_14) >= (*_14);
_29 = !_13;
_11 = (_28, _8);
_9.1 = 181_u8 as isize;
_6 = -_26;
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld4 = _14;
place!(Field::<*const (f64,)>(Variant(_7, 0), 1)) = core::ptr::addr_of!(RET);
_32 = !_13;
_6 = _1 as isize;
_2 = [6_u8,174_u8,23_u8,17_u8,92_u8,207_u8,8_u8];
_24 = [(-1461991876722722308_i64),8402153093387785940_i64];
_19 = _4;
_9 = (_13, Field::<Adt44>(Variant(_7, 0), 4).fld0.1);
_17.1 = _8;
_6 = 9834_i16 as isize;
_17.0 = _1 ^ _11.0;
_22 = (_20.0, _20.1);
_20 = _22;
place!(Field::<Adt44>(Variant(_7, 0), 4)).fld4 = core::ptr::addr_of_mut!(_29);
_29 = _15;
place!(Field::<*const (f64,)>(Variant(_7, 0), 1)) = core::ptr::addr_of!(RET);
_14 = Field::<Adt44>(Variant(_7, 0), 4).fld4;
_21 = (_22.1, _22.1);
_25 = Field::<usize>(Variant(_7, 0), 2);
Call(_20 = fn7(_3, _9.0, _14, _9.1, Field::<Adt44>(Variant(_7, 0), 4).fld0, _15), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<[u8; 7]>(Variant(_7, 0), 6)) = _3;
_21.0 = _20.1;
_22.1 = 150095466179850041284064450487998601263_i128 as u16;
_10 = Field::<Adt44>(Variant(_7, 0), 4).fld0.1;
(*_14) = !_15;
_22.1 = _20.1;
_35 = Adt49::Variant0 { fld0: RET.0 };
_8 = _17.1;
Goto(bb6)
}
bb6 = {
_25 = !Field::<usize>(Variant(_7, 0), 2);
_26 = -_10;
_31 = 1592928641_i32 as isize;
Goto(bb7)
}
bb7 = {
_30 = 59_u8 << _26;
RET.0 = Field::<f64>(Variant(_35, 0), 0);
_21.1 = _20.1;
_4 = Field::<f32>(Variant(_7, 0), 5);
place!(Field::<usize>(Variant(_7, 0), 2)) = !_25;
(*_14) = _15;
_19 = Field::<f32>(Variant(_7, 0), 5) + _4;
_27 = _22.0;
_31 = _26 * _9.1;
_11.0 = (-405147122_i32) as u128;
_11.1 = _8;
_30 = 19420_i16 as u8;
_20.1 = Field::<f32>(Variant(_7, 0), 5) as u16;
place!(Field::<i8>(Variant(_7, 0), 3)) = (-36_i8) + 35_i8;
_20 = (_22.0, _21.1);
(*_27) = (_22.1, _20.1);
_17.0 = _11.0;
place!(Field::<[u8; 7]>(Variant(_7, 0), 6)) = [_30,_30,_30,_30,_30,_30,_30];
place!(Field::<i8>(Variant(_7, 0), 3)) = -(-123_i8);
_42 = _16;
_2 = _3;
_19 = Field::<f32>(Variant(_7, 0), 5);
_4 = -Field::<f32>(Variant(_7, 0), 5);
_25 = Field::<usize>(Variant(_7, 0), 2);
Call(RET = fn12(_20, _32, (*_27).1, (*_27).0, (*_27).0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_23 = _25 << _31;
_22.1 = _11.1 as u16;
_22 = _20;
_20.1 = _22.1 | (*_27).0;
_45 = Field::<i8>(Variant(_7, 0), 3);
_44.1 = (-1878190787_i32) & 514709780_i32;
_16 = _8;
_3 = [_30,_30,_30,_30,_30,_30,_30];
_10 = _31;
_36 = &_41;
_16 = _42;
_17.0 = RET.0 as u128;
Goto(bb9)
}
bb9 = {
_39 = _30 as f64;
place!(Field::<f64>(Variant(_35, 0), 0)) = RET.0 * RET.0;
_4 = Field::<f32>(Variant(_7, 0), 5) * _19;
_12 = _31 & _26;
(*_27).1 = _16 as u16;
RET.0 = _20.1 as f64;
(*_27).1 = 1107944765_u32 as u16;
_19 = Field::<f32>(Variant(_7, 0), 5) + _4;
(*_27).0 = !_20.1;
_45 = _44.1 as i8;
_6 = -_12;
_20.0 = _22.0;
_52.0 = -RET.0;
_17.0 = _1 & _28;
(*_27) = (_20.1, _22.1);
_44.0 = _4;
_46 = _19 * _19;
place!(Field::<f32>(Variant(_7, 0), 5)) = _19;
Goto(bb10)
}
bb10 = {
Call(_54 = dump_var(4_usize, 23_usize, Move(_23), 10_usize, Move(_10), 16_usize, Move(_16), 13_usize, Move(_13)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_54 = dump_var(4_usize, 26_usize, Move(_26), 3_usize, Move(_3), 24_usize, Move(_24), 9_usize, Move(_9)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_54 = dump_var(4_usize, 18_usize, Move(_18), 30_usize, Move(_30), 12_usize, Move(_12), 32_usize, Move(_32)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_54 = dump_var(4_usize, 29_usize, Move(_29), 55_usize, _55, 55_usize, _55, 55_usize, _55), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: char,mut _2: usize,mut _3: (f64,),mut _4: *mut bool,mut _5: f64,mut _6: bool,mut _7: isize,mut _8: bool,mut _9: bool,mut _10: isize,mut _11: (bool, isize),mut _12: isize,mut _13: (bool, isize),mut _14: isize) -> i8 {
mir! {
type RET = i8;
let _15: u128;
let _16: Adt41;
let _17: *mut *mut bool;
let _18: *mut bool;
let _19: *mut i32;
let _20: f64;
let _21: i128;
let _22: char;
let _23: char;
let _24: bool;
let _25: *const (u16, u16);
let _26: [u8; 7];
let _27: (u128, char);
let _28: f64;
let _29: u16;
let _30: usize;
let _31: [u64; 6];
let _32: f64;
let _33: ();
let _34: ();
{
_8 = (*_4);
_5 = (-65_i8) as f64;
_2 = 6_usize & 4_usize;
_6 = _9;
(*_4) = _6;
_13.0 = (*_4);
RET = (-115_i8) - 2_i8;
_2 = !13563025856449473795_usize;
_9 = !(*_4);
_7 = _12;
Goto(bb1)
}
bb1 = {
_4 = core::ptr::addr_of_mut!(_13.0);
_12 = 11884447670287208459_u64 as isize;
_13.0 = _11.0 ^ _6;
_4 = core::ptr::addr_of_mut!(_8);
RET = -93_i8;
_11.1 = _10 * _13.1;
_8 = _6 ^ _9;
_4 = core::ptr::addr_of_mut!(_8);
_13.0 = _8 <= (*_4);
_1 = '\u{8e2a9}';
_13.0 = _9 ^ (*_4);
_6 = !(*_4);
_17 = core::ptr::addr_of_mut!(_18);
_3 = (_5,);
_2 = RET as usize;
(*_4) = _7 > _7;
_10 = !_7;
_10 = _12;
_5 = _3.0 * _3.0;
_7 = 2488317034_u32 as isize;
Goto(bb2)
}
bb2 = {
(*_4) = _6 > _11.0;
_8 = _11.0 == _13.0;
_9 = _8;
(*_4) = _9 > _6;
_18 = core::ptr::addr_of_mut!((*_4));
_8 = _9 ^ _6;
_3.0 = -_5;
_18 = _4;
Call(_14 = fn6(_18, _13.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_4) = !_9;
_23 = _1;
_7 = 220_u8 as isize;
_20 = 16652970490657898374_u64 as f64;
_13 = ((*_18), _11.1);
_13 = _11;
_17 = core::ptr::addr_of_mut!(_4);
_15 = 50034166487465873986193650483928186390_u128 + 9365942779026075374039673803066498007_u128;
Goto(bb4)
}
bb4 = {
_13.0 = !_9;
RET = -60_i8;
(*_17) = core::ptr::addr_of_mut!(_6);
_6 = _9 | _13.0;
_26 = [176_u8,213_u8,75_u8,52_u8,163_u8,28_u8,211_u8];
_12 = _10 ^ _14;
(*_17) = core::ptr::addr_of_mut!((*_4));
_24 = !_13.0;
_5 = _2 as f64;
_8 = (*_4);
_21 = -29356740485402730875956102049733977409_i128;
_13.1 = -_11.1;
_26 = [106_u8,236_u8,12_u8,214_u8,219_u8,35_u8,137_u8];
(*_18) = (*_4);
_27.0 = _15 << _11.1;
_3 = (_20,);
_27.1 = _23;
_5 = _2 as f64;
match _14 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb5 = {
(*_4) = !_9;
_23 = _1;
_7 = 220_u8 as isize;
_20 = 16652970490657898374_u64 as f64;
_13 = ((*_18), _11.1);
_13 = _11;
_17 = core::ptr::addr_of_mut!(_4);
_15 = 50034166487465873986193650483928186390_u128 + 9365942779026075374039673803066498007_u128;
Goto(bb4)
}
bb6 = {
(*_4) = _6 > _11.0;
_8 = _11.0 == _13.0;
_9 = _8;
(*_4) = _9 > _6;
_18 = core::ptr::addr_of_mut!((*_4));
_8 = _9 ^ _6;
_3.0 = -_5;
_18 = _4;
Call(_14 = fn6(_18, _13.0), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4 = core::ptr::addr_of_mut!(_13.0);
_12 = 11884447670287208459_u64 as isize;
_13.0 = _11.0 ^ _6;
_4 = core::ptr::addr_of_mut!(_8);
RET = -93_i8;
_11.1 = _10 * _13.1;
_8 = _6 ^ _9;
_4 = core::ptr::addr_of_mut!(_8);
_13.0 = _8 <= (*_4);
_1 = '\u{8e2a9}';
_13.0 = _9 ^ (*_4);
_6 = !(*_4);
_17 = core::ptr::addr_of_mut!(_18);
_3 = (_5,);
_2 = RET as usize;
(*_4) = _7 > _7;
_10 = !_7;
_10 = _12;
_5 = _3.0 * _3.0;
_7 = 2488317034_u32 as isize;
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
_2 = (*_18) as usize;
(*_18) = _13.0;
match _14 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb12 = {
_4 = core::ptr::addr_of_mut!(_13.0);
_12 = 11884447670287208459_u64 as isize;
_13.0 = _11.0 ^ _6;
_4 = core::ptr::addr_of_mut!(_8);
RET = -93_i8;
_11.1 = _10 * _13.1;
_8 = _6 ^ _9;
_4 = core::ptr::addr_of_mut!(_8);
_13.0 = _8 <= (*_4);
_1 = '\u{8e2a9}';
_13.0 = _9 ^ (*_4);
_6 = !(*_4);
_17 = core::ptr::addr_of_mut!(_18);
_3 = (_5,);
_2 = RET as usize;
(*_4) = _7 > _7;
_10 = !_7;
_10 = _12;
_5 = _3.0 * _3.0;
_7 = 2488317034_u32 as isize;
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
(*_4) = !_9;
_23 = _1;
_7 = 220_u8 as isize;
_20 = 16652970490657898374_u64 as f64;
_13 = ((*_18), _11.1);
_13 = _11;
_17 = core::ptr::addr_of_mut!(_4);
_15 = 50034166487465873986193650483928186390_u128 + 9365942779026075374039673803066498007_u128;
Goto(bb4)
}
bb15 = {
_13.0 = !_9;
RET = -60_i8;
(*_17) = core::ptr::addr_of_mut!(_6);
_6 = _9 | _13.0;
_26 = [176_u8,213_u8,75_u8,52_u8,163_u8,28_u8,211_u8];
_12 = _10 ^ _14;
(*_17) = core::ptr::addr_of_mut!((*_4));
_24 = !_13.0;
_5 = _2 as f64;
_8 = (*_4);
_21 = -29356740485402730875956102049733977409_i128;
_13.1 = -_11.1;
_26 = [106_u8,236_u8,12_u8,214_u8,219_u8,35_u8,137_u8];
(*_18) = (*_4);
_27.0 = _15 << _11.1;
_3 = (_20,);
_27.1 = _23;
_5 = _2 as f64;
match _14 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb16 = {
(*_4) = _6 > _11.0;
_8 = _11.0 == _13.0;
_9 = _8;
(*_4) = _9 > _6;
_18 = core::ptr::addr_of_mut!((*_4));
_8 = _9 ^ _6;
_3.0 = -_5;
_18 = _4;
Call(_14 = fn6(_18, _13.0), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_11 = _13;
_10 = _13.1 * _12;
_9 = !_8;
_5 = _3.0;
_13.1 = 54916_u16 as isize;
_22 = _1;
_11 = ((*_4), _10);
_20 = 19520_i16 as f64;
(*_18) = _6 & _11.0;
RET = 67_i8 & (-54_i8);
_30 = 13362126259755524436_u64 as usize;
(*_18) = !_11.0;
_26 = [68_u8,44_u8,253_u8,69_u8,113_u8,182_u8,114_u8];
_3.0 = -_5;
_27.0 = _15 - _15;
_8 = (*_4);
_32 = _5 - _20;
_3.0 = _5 - _20;
_22 = _1;
_22 = _27.1;
Goto(bb18)
}
bb18 = {
Call(_33 = dump_var(5_usize, 22_usize, Move(_22), 12_usize, Move(_12), 15_usize, Move(_15), 23_usize, Move(_23)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(5_usize, 11_usize, Move(_11), 26_usize, Move(_26), 30_usize, Move(_30), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(5_usize, 9_usize, Move(_9), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut bool,mut _2: bool) -> isize {
mir! {
type RET = isize;
let _3: f32;
let _4: (f32, i32);
let _5: (u16, u16);
let _6: f64;
let _7: Adt49;
let _8: [u8; 7];
let _9: *mut *mut bool;
let _10: (f64,);
let _11: *mut *mut bool;
let _12: Adt52;
let _13: [u8; 7];
let _14: [i64; 2];
let _15: (u128, char);
let _16: Adt43;
let _17: f64;
let _18: *const i64;
let _19: ();
let _20: ();
{
RET = !(-117_isize);
(*_1) = !_2;
_2 = (*_1) & (*_1);
RET = 150_u8 as isize;
(*_1) = _2;
RET = 9223372036854775807_isize - 108_isize;
_2 = (*_1);
_3 = 8279687928210168359_i64 as f32;
_4 = (_3, (-413163471_i32));
_1 = core::ptr::addr_of_mut!((*_1));
_3 = _4.0 - _4.0;
_5 = (53197_u16, 11350_u16);
_2 = !(*_1);
RET = _3 as isize;
_3 = -_4.0;
_2 = (*_1) ^ (*_1);
_4.1 = _5.1 as i32;
_5.1 = _5.0;
_2 = !(*_1);
_1 = core::ptr::addr_of_mut!((*_1));
_6 = 436202325006436196_u64 as f64;
Call(_3 = core::intrinsics::transmute(_4.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.1 = 1074850722_i32;
_6 = (-61774125402864800503452518207628228727_i128) as f64;
RET = 12273996455500060418_usize as isize;
_4.1 = 103088800_i32 >> _5.1;
(*_1) = _2 | _2;
_2 = !(*_1);
RET = (-9223372036854775808_isize);
_7 = Adt49::Variant0 { fld0: _6 };
_3 = _4.0;
(*_1) = !_2;
SetDiscriminant(_7, 0);
_5 = (13553_u16, 59307_u16);
_2 = (*_1);
(*_1) = _2 == _2;
_4.0 = -_3;
_4.0 = _3;
_1 = core::ptr::addr_of_mut!((*_1));
_9 = core::ptr::addr_of_mut!(_1);
_4.0 = -_3;
_3 = _4.0;
_9 = core::ptr::addr_of_mut!((*_9));
_8 = [124_u8,37_u8,210_u8,73_u8,250_u8,227_u8,229_u8];
RET = (-4256300794785808828_i64) as isize;
_8 = [208_u8,167_u8,65_u8,247_u8,192_u8,156_u8,59_u8];
_9 = core::ptr::addr_of_mut!((*_9));
match _5.1 {
0 => bb2,
1 => bb3,
2 => bb4,
59307 => bb6,
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
(*_9) = core::ptr::addr_of_mut!((*_1));
_1 = core::ptr::addr_of_mut!(_2);
_4.1 = !646312142_i32;
_9 = core::ptr::addr_of_mut!(_1);
_5.0 = _5.1;
_4.0 = _3;
place!(Field::<f64>(Variant(_7, 0), 0)) = _6 - _6;
RET = !(-9223372036854775808_isize);
_8 = [163_u8,193_u8,30_u8,255_u8,40_u8,204_u8,8_u8];
(*_1) = _3 >= _4.0;
_5.1 = !_5.0;
RET = !40_isize;
SetDiscriminant(_7, 0);
_9 = core::ptr::addr_of_mut!(_1);
(*_9) = core::ptr::addr_of_mut!(_2);
(*_9) = core::ptr::addr_of_mut!((*_1));
(*_1) = !false;
(*_1) = !true;
_4.1 = -(-945083118_i32);
_4.1 = -116209678_i32;
match _5.0 {
0 => bb7,
1 => bb8,
59307 => bb10,
_ => bb9
}
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
_4 = (_3, (-1230452310_i32));
RET = 3517448268738959671_i64 as isize;
match _4.1 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607430537759146 => bb11,
_ => bb9
}
}
bb11 = {
RET = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_4.1 = 924542719_i32;
_4.0 = _3 * _3;
_7 = Adt49::Variant0 { fld0: _6 };
_10.0 = -_6;
_4 = (_3, (-7390617_i32));
_9 = core::ptr::addr_of_mut!((*_9));
SetDiscriminant(_7, 0);
_7 = Adt49::Variant0 { fld0: _6 };
(*_9) = core::ptr::addr_of_mut!(_2);
_2 = !true;
_5.0 = _5.1;
_11 = core::ptr::addr_of_mut!((*_9));
_4 = (_3, 1372391649_i32);
_8 = [111_u8,44_u8,191_u8,149_u8,151_u8,208_u8,171_u8];
RET = (-9223372036854775808_isize) << _5.0;
_9 = _11;
_6 = Field::<f64>(Variant(_7, 0), 0);
_4.0 = _3;
_14 = [901977482872893348_i64,5916925909718920112_i64];
_7 = Adt49::Variant0 { fld0: _10.0 };
_13 = [223_u8,223_u8,16_u8,203_u8,233_u8,135_u8,69_u8];
(*_11) = core::ptr::addr_of_mut!((*_1));
_12 = Adt52::Variant1 { fld0: 2401575927_u32 };
_15.0 = (-75_i8) as u128;
RET = (-9223372036854775808_isize);
match RET {
0 => bb4,
1 => bb7,
2 => bb3,
3 => bb12,
4 => bb13,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb12 = {
_4.1 = 1074850722_i32;
_6 = (-61774125402864800503452518207628228727_i128) as f64;
RET = 12273996455500060418_usize as isize;
_4.1 = 103088800_i32 >> _5.1;
(*_1) = _2 | _2;
_2 = !(*_1);
RET = (-9223372036854775808_isize);
_7 = Adt49::Variant0 { fld0: _6 };
_3 = _4.0;
(*_1) = !_2;
SetDiscriminant(_7, 0);
_5 = (13553_u16, 59307_u16);
_2 = (*_1);
(*_1) = _2 == _2;
_4.0 = -_3;
_4.0 = _3;
_1 = core::ptr::addr_of_mut!((*_1));
_9 = core::ptr::addr_of_mut!(_1);
_4.0 = -_3;
_3 = _4.0;
_9 = core::ptr::addr_of_mut!((*_9));
_8 = [124_u8,37_u8,210_u8,73_u8,250_u8,227_u8,229_u8];
RET = (-4256300794785808828_i64) as isize;
_8 = [208_u8,167_u8,65_u8,247_u8,192_u8,156_u8,59_u8];
_9 = core::ptr::addr_of_mut!((*_9));
match _5.1 {
0 => bb2,
1 => bb3,
2 => bb4,
59307 => bb6,
_ => bb5
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_5 = (6454_u16, 33707_u16);
_4.0 = -_3;
_3 = _4.0 - _4.0;
_17 = _10.0;
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(6_usize, 2_usize, Move(_2), 13_usize, Move(_13), 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [u8; 7],mut _2: bool,mut _3: *mut bool,mut _4: isize,mut _5: (bool, isize),mut _6: bool) -> (*const (u16, u16), u16) {
mir! {
type RET = (*const (u16, u16), u16);
let _7: u8;
let _8: u8;
let _9: f64;
let _10: *const u16;
let _11: *const (f64,);
let _12: *mut *mut bool;
let _13: Adt49;
let _14: (u16, u16);
let _15: char;
let _16: (f64,);
let _17: [u64; 6];
let _18: [i64; 2];
let _19: (bool, isize);
let _20: u8;
let _21: [u64; 6];
let _22: bool;
let _23: [i16; 1];
let _24: Adt56;
let _25: Adt54;
let _26: Adt55;
let _27: (f64,);
let _28: char;
let _29: (u16, u16);
let _30: Adt53;
let _31: [i16; 1];
let _32: [i16; 1];
let _33: i8;
let _34: f32;
let _35: [u64; 6];
let _36: u8;
let _37: ();
let _38: ();
{
_5.1 = _4;
_5 = (_2, _4);
RET.1 = 30098_u16;
_1 = [255_u8,74_u8,10_u8,67_u8,35_u8,250_u8,218_u8];
_1 = [45_u8,221_u8,28_u8,126_u8,116_u8,193_u8,120_u8];
RET.1 = 2_usize as u16;
_7 = 2_u8;
_2 = _6;
_1 = [_7,_7,_7,_7,_7,_7,_7];
_5.0 = (*_3) == (*_3);
_7 = 23_u8 << _5.1;
_7 = 42_u8;
_3 = core::ptr::addr_of_mut!((*_3));
RET.1 = 64598_u16 - 14963_u16;
_5.1 = !_4;
(*_3) = !_5.0;
_3 = core::ptr::addr_of_mut!(_5.0);
_6 = !(*_3);
_5.1 = _4 ^ _4;
_5 = (_2, _4);
_5.0 = !_6;
_5.0 = !_2;
_5 = (_6, _4);
_5.1 = _4;
_4 = _5.1 ^ _5.1;
(*_3) = !_2;
_1 = [_7,_7,_7,_7,_7,_7,_7];
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
42 => bb5,
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
_1 = [_7,_7,_7,_7,_7,_7,_7];
_8 = !_7;
_2 = (*_3);
_3 = core::ptr::addr_of_mut!((*_3));
RET.1 = 54089_u16;
(*_3) = !_2;
_8 = _7 | _7;
_5.1 = _4;
_1 = [_8,_7,_7,_8,_8,_7,_7];
_10 = core::ptr::addr_of!(RET.1);
_6 = _5.0;
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!((*_3));
_9 = 8898580522280475016_i64 as f64;
_9 = 1369159464_u32 as f64;
(*_3) = _6 == _6;
_1 = [_7,_8,_7,_8,_8,_7,_8];
(*_3) = _6;
_10 = core::ptr::addr_of!(RET.1);
_1 = [_8,_8,_8,_8,_8,_8,_8];
_7 = _8;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = _2;
Goto(bb6)
}
bb6 = {
_5.0 = _6;
_5.0 = !_6;
_5.0 = _2;
_12 = core::ptr::addr_of_mut!(_3);
(*_12) = core::ptr::addr_of_mut!((*_3));
(*_3) = !_6;
_9 = 7_usize as f64;
(*_12) = core::ptr::addr_of_mut!((*_3));
(*_10) = '\u{48f3e}' as u16;
_6 = _2 & (*_3);
_6 = (*_3);
_6 = _5.0;
Goto(bb7)
}
bb7 = {
_5.0 = _5.1 > _4;
_7 = _8 - _8;
_5 = (_2, _4);
_5 = (_2, _4);
RET.1 = 58263_u16 + 4016_u16;
(*_12) = core::ptr::addr_of_mut!((*_3));
_16 = (_9,);
_14.0 = RET.1 | RET.1;
_5 = (_2, _4);
_11 = core::ptr::addr_of!(_16);
_13 = Adt49::Variant0 { fld0: (*_11).0 };
(*_10) = 5869341579357317912_i64 as u16;
_11 = core::ptr::addr_of!((*_11));
(*_11) = (_9,);
(*_11).0 = -Field::<f64>(Variant(_13, 0), 0);
_7 = _8 + _8;
Call(_9 = fn8(_5, (*_12), (*_3), _7, (*_11).0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_3) = !_6;
_15 = '\u{101ee4}';
Goto(bb9)
}
bb9 = {
_20 = (-71_i8) as u8;
(*_3) = _2;
_18 = [(-3117099957274183022_i64),(-2028185081380679717_i64)];
_19.1 = (-796205424_i32) as isize;
SetDiscriminant(_13, 1);
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)) = (_14.0, _14.0);
_21 = [4319846764855749967_u64,7383782071150341071_u64,9957498986244917896_u64,2210713686100401376_u64,17057000713215239963_u64,12586773934437365899_u64];
_14.1 = _9 as u16;
_19.0 = _6;
(*_10) = 306074242800847268884598938781744679309_u128 as u16;
Call(place!(Field::<bool>(Variant(_13, 1), 0)) = fn9(_6, (*_3), _3, _19.0, (*_3), _5, _5.0, (*_3), _5.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_19.1 = _14.1 as isize;
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)).0 = _14.1;
_5.1 = (-137671663858668704303406230246402795621_i128) as isize;
_19.1 = _4 + _4;
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)).1 = RET.1;
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)).1 = _14.1 + _14.1;
_10 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_13, 1), 2)).1);
_17 = [12142705673456664855_u64,16028052102429541365_u64,12005246223452676495_u64,6911398386864847006_u64,321024129412992433_u64,10684051801260555321_u64];
_16 = (_9,);
_1 = [_20,_7,_20,_8,_7,_7,_8];
_8 = !_20;
(*_12) = core::ptr::addr_of_mut!(_5.0);
_29 = ((*_10), Field::<(u16, u16)>(Variant(_13, 1), 2).1);
_15 = '\u{cef9d}';
_18 = [1829027928706136347_i64,7336646106817420207_i64];
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)).1 = !RET.1;
_15 = '\u{b1179}';
(*_3) = Field::<bool>(Variant(_13, 1), 0) <= Field::<bool>(Variant(_13, 1), 0);
_22 = (*_3);
(*_11).0 = 5708276115913727753_u64 as f64;
_5.1 = _4 * _4;
(*_3) = _6 | _19.0;
_5 = (_22, _19.1);
Call(RET.0 = fn10((*_12), (*_3), _3, _14.0, _12, (*_12), _12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_11 = core::ptr::addr_of!(_16);
_14.0 = 277858365246986904426266143045681526476_u128 as u16;
_10 = core::ptr::addr_of!(place!(Field::<(u16, u16)>(Variant(_13, 1), 2)).0);
_29 = (_14.1, (*_10));
_19.0 = _5.0;
_16.0 = _9;
_8 = 14_i8 as u8;
_24.fld3.0 = 207772501659025998549014412661757737442_u128 ^ 298497989734980126569969153844316809925_u128;
_26 = Adt55::Variant2 { fld0: _19.0 };
_5.1 = -_19.1;
_23 = [24414_i16];
place!(Field::<bool>(Variant(_26, 2), 0)) = !_19.0;
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)) = (_14.1, _14.0);
_14.1 = _19.1 as u16;
Goto(bb12)
}
bb12 = {
_16.0 = (-5493607847275356208_i64) as f64;
place!(Field::<(u16, u16)>(Variant(_13, 1), 2)) = _14;
RET.1 = Field::<(u16, u16)>(Variant(_13, 1), 2).1;
_24.fld1 = _15;
place!(Field::<bool>(Variant(_26, 2), 0)) = !(*_3);
_35 = _21;
Goto(bb13)
}
bb13 = {
Call(_37 = dump_var(7_usize, 19_usize, Move(_19), 21_usize, Move(_21), 14_usize, Move(_14), 20_usize, Move(_20)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_37 = dump_var(7_usize, 4_usize, Move(_4), 2_usize, Move(_2), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_37 = dump_var(7_usize, 1_usize, Move(_1), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: (bool, isize),mut _2: *mut bool,mut _3: bool,mut _4: u8,mut _5: f64) -> f64 {
mir! {
type RET = f64;
let _6: Adt47;
let _7: isize;
let _8: *const u16;
let _9: f64;
let _10: *const (f64,);
let _11: u64;
let _12: isize;
let _13: f32;
let _14: i16;
let _15: (f32, i32);
let _16: ();
let _17: ();
{
_5 = 13513098217902105116_u64 as f64;
RET = _5 + _5;
(*_2) = _1.0;
_2 = core::ptr::addr_of_mut!(_1.0);
_5 = 1738955480372645133_u64 as f64;
_1 = (_3, (-90_isize));
(*_2) = _3;
_4 = 6_u8;
_3 = _1.0;
(*_2) = !_3;
_3 = (*_2);
_1 = (_3, (-64_isize));
_4 = 118_u8;
_7 = _1.1;
match _1.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211392 => bb6,
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
RET = _5;
(*_2) = _3 == _3;
_1.0 = _3;
_7 = _1.1;
_9 = _5;
_3 = (*_2);
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211392 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
RET = 229945279292823075599078662501986431656_u128 as f64;
_1.1 = -_7;
_1 = (_3, _7);
_9 = 230481177815660631717216006777418054627_u128 as f64;
_1 = (_3, _7);
_2 = core::ptr::addr_of_mut!(_1.0);
(*_2) = _3;
_2 = core::ptr::addr_of_mut!(_3);
_1.1 = 5_usize as isize;
_4 = 41_u8 >> _7;
_9 = -_5;
(*_2) = !_1.0;
_1 = ((*_2), _7);
_3 = _1.0;
_4 = 96_u8 >> _7;
RET = _5 - _5;
_1 = ((*_2), _7);
(*_2) = !_1.0;
(*_2) = _1.0;
(*_2) = _1.0 & _1.0;
_2 = core::ptr::addr_of_mut!((*_2));
_1 = (_3, _7);
_1.1 = _7;
_5 = -RET;
_4 = 95_u8 - 240_u8;
_3 = !_1.0;
match _1.1 {
0 => bb9,
1 => bb10,
340282366920938463463374607431768211392 => bb12,
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
_11 = 15294599235348999868_u64 + 3314984495893223216_u64;
_4 = 3_u8 & 249_u8;
_1.1 = -_7;
_9 = 13829758894339938654638240332151734944_i128 as f64;
_11 = !4530280939553169141_u64;
_1 = ((*_2), _7);
_3 = _1.0;
RET = _9 - _9;
_12 = 8790059605155496924_usize as isize;
_2 = core::ptr::addr_of_mut!((*_2));
_14 = (-32283_i16) << _7;
RET = _5;
_7 = RET as isize;
_11 = 12402673050018646972_u64 + 4411488362496898080_u64;
_9 = -RET;
_12 = _1.1;
match _12 {
0 => bb4,
1 => bb10,
2 => bb6,
340282366920938463463374607431768211392 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_13 = _5 as f32;
(*_2) = _14 < _14;
_14 = 964_i16 + (-7884_i16);
_15.0 = (-1775680374_i32) as f32;
RET = _9 - _9;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(8_usize, 7_usize, Move(_7), 11_usize, Move(_11), 1_usize, Move(_1), 17_usize, _17), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: bool,mut _3: *mut bool,mut _4: bool,mut _5: bool,mut _6: (bool, isize),mut _7: bool,mut _8: bool,mut _9: isize) -> bool {
mir! {
type RET = bool;
let _10: [u64; 6];
let _11: Adt49;
let _12: Adt49;
let _13: [u8; 7];
let _14: i8;
let _15: ();
let _16: ();
{
_3 = core::ptr::addr_of_mut!(_4);
_7 = _6.0;
(*_3) = _9 > _9;
RET = !_1;
_6.1 = !_9;
_9 = _6.1 + _6.1;
RET = (*_3);
_6.1 = !_9;
RET = !_7;
_6 = (RET, _9);
_7 = !_4;
_1 = RET;
_9 = -_6.1;
_6 = (_1, _9);
RET = !_4;
_1 = _6.1 <= _9;
_10 = [3180293594957777729_u64,14629518766620620550_u64,3197865834601069163_u64,4489118067506378735_u64,8748144664018109581_u64,1711690635468352162_u64];
_6.0 = (*_3) | _4;
_2 = _6.0;
_6.0 = !_2;
_8 = !_1;
_2 = !_4;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(9_usize, 2_usize, Move(_2), 4_usize, Move(_4), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *mut bool,mut _2: bool,mut _3: *mut bool,mut _4: u16,mut _5: *mut *mut bool,mut _6: *mut bool,mut _7: *mut *mut bool) -> *const (u16, u16) {
mir! {
type RET = *const (u16, u16);
let _8: (f64,);
let _9: u16;
let _10: Adt49;
let _11: [i16; 1];
let _12: (bool, isize);
let _13: [u8; 7];
let _14: u64;
let _15: *mut i32;
let _16: Adt49;
let _17: Adt50;
let _18: usize;
let _19: bool;
let _20: i64;
let _21: char;
let _22: [i64; 2];
let _23: i32;
let _24: [char; 5];
let _25: *mut bool;
let _26: (bool, isize);
let _27: u128;
let _28: (u16, u16);
let _29: bool;
let _30: [i64; 2];
let _31: Adt42;
let _32: [i16; 1];
let _33: f32;
let _34: [u64; 6];
let _35: isize;
let _36: bool;
let _37: ();
let _38: ();
{
_6 = core::ptr::addr_of_mut!((*_1));
(*_1) = _2 < _2;
(*_7) = core::ptr::addr_of_mut!(_2);
_2 = (*_1) | (*_3);
(*_7) = _1;
(*_5) = core::ptr::addr_of_mut!((*_3));
_4 = !31669_u16;
_7 = core::ptr::addr_of_mut!((*_5));
(*_1) = _2;
_1 = (*_7);
_3 = core::ptr::addr_of_mut!((*_6));
_9 = _4;
_7 = _5;
Goto(bb1)
}
bb1 = {
_7 = _5;
_5 = core::ptr::addr_of_mut!(_6);
_2 = (*_3) | (*_6);
(*_5) = core::ptr::addr_of_mut!((*_6));
_11 = [(-30563_i16)];
(*_6) = _2;
_6 = _1;
(*_1) = _2 >= _2;
_2 = !(*_1);
_1 = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_1));
_4 = !_9;
_12.0 = (*_6) & (*_1);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_6) = !_12.0;
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_1) = _2;
_11 = [(-17193_i16)];
(*_7) = (*_5);
(*_7) = core::ptr::addr_of_mut!((*_6));
_6 = (*_7);
(*_7) = (*_5);
_6 = (*_7);
(*_1) = _2;
_8.0 = (-91_isize) as f64;
(*_3) = _12.0;
_5 = core::ptr::addr_of_mut!((*_5));
Call(_7 = fn11((*_1), _6, _3, (*_5), (*_3), _6, _2, _1, _12.0, _2, (*_6), (*_5), (*_1), _6, _3, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.1 = 6_isize * 9223372036854775807_isize;
_12.1 = (-108_isize) * (-70_isize);
_3 = (*_5);
_12.1 = 9223372036854775807_isize;
_1 = core::ptr::addr_of_mut!((*_6));
_7 = core::ptr::addr_of_mut!((*_5));
(*_3) = _2 ^ _12.0;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_2;
_6 = _3;
_9 = 1411933413_i32 as u16;
_14 = 10492890793052509691_u64 ^ 6669305190068567169_u64;
(*_3) = _12.0 <= _2;
_13 = [186_u8,153_u8,43_u8,215_u8,29_u8,254_u8,140_u8];
_9 = !_4;
Goto(bb3)
}
bb3 = {
_5 = _7;
_11 = [22458_i16];
_12.1 = -9223372036854775807_isize;
_12.1 = 9223372036854775807_isize;
(*_5) = _3;
_8.0 = 210128231130203287566104359744521803219_u128 as f64;
_8.0 = 5142636894175570449_i64 as f64;
_21 = '\u{1007c0}';
_12 = ((*_6), (-9223372036854775808_isize));
_14 = 2944939092581355583_u64;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = _12.0;
_19 = (*_6) >= (*_6);
(*_6) = _2 < _12.0;
(*_6) = !_12.0;
_2 = (*_6);
_7 = _5;
(*_7) = _1;
_16 = Adt49::Variant0 { fld0: _8.0 };
_7 = core::ptr::addr_of_mut!((*_7));
(*_5) = core::ptr::addr_of_mut!((*_6));
_20 = 12879356160542389772_usize as i64;
_18 = 45_i8 as usize;
_2 = !(*_6);
_2 = !(*_3);
match _12.1 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb4 = {
_12.1 = 6_isize * 9223372036854775807_isize;
_12.1 = (-108_isize) * (-70_isize);
_3 = (*_5);
_12.1 = 9223372036854775807_isize;
_1 = core::ptr::addr_of_mut!((*_6));
_7 = core::ptr::addr_of_mut!((*_5));
(*_3) = _2 ^ _12.0;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_2;
_6 = _3;
_9 = 1411933413_i32 as u16;
_14 = 10492890793052509691_u64 ^ 6669305190068567169_u64;
(*_3) = _12.0 <= _2;
_13 = [186_u8,153_u8,43_u8,215_u8,29_u8,254_u8,140_u8];
_9 = !_4;
Goto(bb3)
}
bb5 = {
_7 = _5;
_5 = core::ptr::addr_of_mut!(_6);
_2 = (*_3) | (*_6);
(*_5) = core::ptr::addr_of_mut!((*_6));
_11 = [(-30563_i16)];
(*_6) = _2;
_6 = _1;
(*_1) = _2 >= _2;
_2 = !(*_1);
_1 = core::ptr::addr_of_mut!((*_3));
(*_5) = core::ptr::addr_of_mut!((*_1));
_4 = !_9;
_12.0 = (*_6) & (*_1);
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_6) = !_12.0;
(*_5) = core::ptr::addr_of_mut!((*_3));
(*_1) = _2;
_11 = [(-17193_i16)];
(*_7) = (*_5);
(*_7) = core::ptr::addr_of_mut!((*_6));
_6 = (*_7);
(*_7) = (*_5);
_6 = (*_7);
(*_1) = _2;
_8.0 = (-91_isize) as f64;
(*_3) = _12.0;
_5 = core::ptr::addr_of_mut!((*_5));
Call(_7 = fn11((*_1), _6, _3, (*_5), (*_3), _6, _2, _1, _12.0, _2, (*_6), (*_5), (*_1), _6, _3, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_14 = 18115080710251582182_u64 + 17018926872611678676_u64;
(*_7) = _1;
(*_3) = !_2;
_18 = 4_usize + 4_usize;
_4 = !_9;
_1 = core::ptr::addr_of_mut!(_2);
_10 = Adt49::Variant0 { fld0: Field::<f64>(Variant(_16, 0), 0) };
_10 = Move(_16);
(*_5) = _1;
_8 = (Field::<f64>(Variant(_10, 0), 0),);
_6 = core::ptr::addr_of_mut!((*_1));
_4 = _9 + _9;
_12 = ((*_3), 9223372036854775807_isize);
Goto(bb9)
}
bb9 = {
_12.0 = (*_6);
_12.1 = _20 as isize;
_13 = [14_u8,124_u8,82_u8,184_u8,35_u8,59_u8,209_u8];
_18 = Field::<f64>(Variant(_10, 0), 0) as usize;
(*_1) = (*_3);
_14 = 1490733965560167026_u64 - 3755496100290578524_u64;
_12 = ((*_1), 99_isize);
_18 = 7_usize;
_16 = Move(_10);
match _12.1 {
99 => bb11,
_ => bb10
}
}
bb10 = {
_12.1 = 6_isize * 9223372036854775807_isize;
_12.1 = (-108_isize) * (-70_isize);
_3 = (*_5);
_12.1 = 9223372036854775807_isize;
_1 = core::ptr::addr_of_mut!((*_6));
_7 = core::ptr::addr_of_mut!((*_5));
(*_3) = _2 ^ _12.0;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_2;
_6 = _3;
_9 = 1411933413_i32 as u16;
_14 = 10492890793052509691_u64 ^ 6669305190068567169_u64;
(*_3) = _12.0 <= _2;
_13 = [186_u8,153_u8,43_u8,215_u8,29_u8,254_u8,140_u8];
_9 = !_4;
Goto(bb3)
}
bb11 = {
_7 = core::ptr::addr_of_mut!(_1);
_20 = 117_i8 as i64;
_1 = core::ptr::addr_of_mut!((*_1));
(*_7) = core::ptr::addr_of_mut!((*_3));
Goto(bb12)
}
bb12 = {
(*_7) = core::ptr::addr_of_mut!((*_1));
_26.0 = (*_1);
_26.0 = (*_1);
_26.1 = -_12.1;
_9 = _8.0 as u16;
_22 = [_20,_20];
match _12.1 {
0 => bb3,
1 => bb4,
99 => bb14,
_ => bb13
}
}
bb13 = {
_7 = core::ptr::addr_of_mut!(_1);
_20 = 117_i8 as i64;
_1 = core::ptr::addr_of_mut!((*_1));
(*_7) = core::ptr::addr_of_mut!((*_3));
Goto(bb12)
}
bb14 = {
_5 = core::ptr::addr_of_mut!(_6);
_11 = [(-3025_i16)];
_4 = (-66_i8) as u16;
_15 = core::ptr::addr_of_mut!(_23);
_12.1 = _26.1 << _26.1;
_23 = 1025121780_i32;
_8.0 = _23 as f64;
_28.0 = !_4;
_15 = core::ptr::addr_of_mut!(_23);
(*_15) = !(-991887230_i32);
(*_7) = (*_5);
match _18 {
0 => bb6,
1 => bb4,
2 => bb3,
3 => bb15,
7 => bb17,
_ => bb16
}
}
bb15 = {
_7 = core::ptr::addr_of_mut!(_1);
_20 = 117_i8 as i64;
_1 = core::ptr::addr_of_mut!((*_1));
(*_7) = core::ptr::addr_of_mut!((*_3));
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
_29 = _2 <= _2;
RET = core::ptr::addr_of!(_28);
_26.1 = _12.1 >> _12.1;
(*RET).1 = !(*RET).0;
(*_6) = _29;
(*_3) = _19 & _12.0;
_27 = 121600935089678071570533462135114594464_u128 >> _26.1;
_11 = [(-25440_i16)];
_34 = [_14,_14,_14,_14,_14,_14];
_28.0 = !_9;
_2 = _12.0;
(*RET).0 = !(*RET).1;
(*_3) = _29;
_11 = [(-32201_i16)];
_8.0 = -Field::<f64>(Variant(_16, 0), 0);
(*_6) = _12.0;
_32 = _11;
Goto(bb18)
}
bb18 = {
Call(_37 = dump_var(10_usize, 9_usize, Move(_9), 13_usize, Move(_13), 19_usize, Move(_19), 34_usize, Move(_34)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_37 = dump_var(10_usize, 21_usize, Move(_21), 20_usize, Move(_20), 28_usize, Move(_28), 29_usize, Move(_29)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_37 = dump_var(10_usize, 2_usize, Move(_2), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: bool,mut _2: *mut bool,mut _3: *mut bool,mut _4: *mut bool,mut _5: bool,mut _6: *mut bool,mut _7: bool,mut _8: *mut bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: *mut bool,mut _13: bool,mut _14: *mut bool,mut _15: *mut bool,mut _16: *mut bool) -> *mut *mut bool {
mir! {
type RET = *mut *mut bool;
let _17: ((u16, u16), i8, &'static i128, usize, [u8; 7]);
let _18: ();
let _19: ();
{
(*_3) = _11 < _7;
(*_6) = _7 > _5;
(*_6) = _10 | _11;
_3 = _6;
(*_6) = _7;
(*_2) = !_1;
_6 = _8;
_17.0 = (8027_u16, 60674_u16);
(*_3) = _13;
(*_8) = _5 & _13;
RET = core::ptr::addr_of_mut!(_3);
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(11_usize, 1_usize, Move(_1), 7_usize, Move(_7), 5_usize, Move(_5), 19_usize, _19), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (*const (u16, u16), u16),mut _2: bool,mut _3: u16,mut _4: u16,mut _5: u16) -> (f64,) {
mir! {
type RET = (f64,);
let _6: *mut *mut bool;
let _7: i64;
let _8: usize;
let _9: u64;
let _10: [char; 5];
let _11: [u8; 4];
let _12: (f64,);
let _13: u128;
let _14: [i64; 2];
let _15: i16;
let _16: f64;
let _17: (f32, i32);
let _18: (f64,);
let _19: [u8; 4];
let _20: isize;
let _21: f64;
let _22: (bool, isize);
let _23: (f32, i32);
let _24: isize;
let _25: bool;
let _26: u16;
let _27: i8;
let _28: (u128, char);
let _29: i32;
let _30: (u128, char);
let _31: *mut [char; 5];
let _32: isize;
let _33: Adt49;
let _34: f64;
let _35: i8;
let _36: ();
let _37: ();
{
_3 = !_1.1;
RET.0 = _1.1 as f64;
RET.0 = 3241648204_u32 as f64;
_2 = !true;
_4 = '\u{807e6}' as u16;
Goto(bb1)
}
bb1 = {
_2 = false;
_2 = _5 < _5;
_1.1 = (-9223372036854775808_isize) as u16;
RET.0 = (-54_i8) as f64;
RET.0 = 80647539012573617894081648479839278351_i128 as f64;
_4 = _5;
_1.1 = !_4;
_3 = _4 << _5;
RET.0 = 414018100173554471_u64 as f64;
_5 = _3 ^ _3;
_5 = _1.1;
_7 = 231953110749568235_i64;
_5 = !_3;
_7 = (-9167704010646406378_i64) + (-2759886833280730697_i64);
_1.1 = _4 * _3;
_10 = ['\u{8374}','\u{51a97}','\u{5846c}','\u{4924e}','\u{73ec0}'];
_8 = 6_usize;
_12.0 = -RET.0;
_10 = ['\u{c5dec}','\u{10adb7}','\u{6f7c1}','\u{16569}','\u{17ae1}'];
_11 = [89_u8,66_u8,197_u8,232_u8];
RET.0 = -_12.0;
_3 = !_1.1;
Goto(bb2)
}
bb2 = {
_14 = [_7,_7];
_1.1 = _3 >> _5;
_1.1 = 12379079202701621929_u64 as u16;
_12.0 = (-7475_i16) as f64;
_17.0 = 136436988696760121469672820160303621783_i128 as f32;
RET.0 = _12.0 - _12.0;
_15 = 12209_i16;
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
6 => bb9,
_ => bb8
}
}
bb3 = {
_2 = false;
_2 = _5 < _5;
_1.1 = (-9223372036854775808_isize) as u16;
RET.0 = (-54_i8) as f64;
RET.0 = 80647539012573617894081648479839278351_i128 as f64;
_4 = _5;
_1.1 = !_4;
_3 = _4 << _5;
RET.0 = 414018100173554471_u64 as f64;
_5 = _3 ^ _3;
_5 = _1.1;
_7 = 231953110749568235_i64;
_5 = !_3;
_7 = (-9167704010646406378_i64) + (-2759886833280730697_i64);
_1.1 = _4 * _3;
_10 = ['\u{8374}','\u{51a97}','\u{5846c}','\u{4924e}','\u{73ec0}'];
_8 = 6_usize;
_12.0 = -RET.0;
_10 = ['\u{c5dec}','\u{10adb7}','\u{6f7c1}','\u{16569}','\u{17ae1}'];
_11 = [89_u8,66_u8,197_u8,232_u8];
RET.0 = -_12.0;
_3 = !_1.1;
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
_12.0 = 64683088507026438_u64 as f64;
_17.1 = (-872078736_i32);
_5 = !_3;
_18.0 = RET.0;
_17.0 = _8 as f32;
_17.1 = 126_i8 as i32;
_16 = -_12.0;
Goto(bb10)
}
bb10 = {
_8 = !6_usize;
RET.0 = (-103_i8) as f64;
_18 = RET;
_18 = (_12.0,);
RET = _18;
_20 = 9223372036854775807_isize;
_20 = -101_isize;
_18.0 = _8 as f64;
RET.0 = _15 as f64;
_1.1 = _3 | _4;
_5 = _3;
_13 = _2 as u128;
_4 = _1.1;
Call(_19 = core::intrinsics::transmute(_11), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_19 = [53_u8,151_u8,162_u8,138_u8];
_16 = _18.0 - _18.0;
_14 = [_7,_7];
Goto(bb12)
}
bb12 = {
_22.0 = _4 <= _4;
_12 = (_18.0,);
_17.0 = _15 as f32;
_9 = !11593541582225299276_u64;
_18 = (_12.0,);
_7 = (-763121168455818464_i64) >> _1.1;
RET.0 = _16 - _16;
_5 = _4;
RET.0 = -_16;
_5 = !_3;
_19 = [165_u8,162_u8,220_u8,198_u8];
_14 = [_7,_7];
_2 = !_22.0;
_15 = 218_u8 as i16;
_21 = _17.1 as f64;
_23 = (_17.0, _17.1);
_22 = (_2, _20);
_28.0 = !_13;
_17.0 = _23.0;
_16 = _17.1 as f64;
RET = (_16,);
_30.0 = _13 | _13;
_13 = _30.0 - _30.0;
_1.1 = _23.0 as u16;
_1.1 = _5;
Goto(bb13)
}
bb13 = {
_4 = !_3;
_28 = (_13, '\u{86f3e}');
_30 = _28;
_8 = 9037600299024999166_usize;
_21 = -_12.0;
_28 = (_13, _30.1);
_29 = _20 as i32;
_31 = core::ptr::addr_of_mut!(_10);
_26 = !_3;
_27 = (-120_i8) + 4_i8;
_20 = _27 as isize;
_30 = _28;
_3 = _5 | _26;
_24 = !_22.1;
_30 = _28;
_22 = (_2, _24);
_13 = _30.0 - _28.0;
_23.1 = _9 as i32;
_22 = (_2, _20);
_25 = !_2;
_30.1 = _28.1;
_22 = (_2, _20);
_15 = 9604_i16 - 4614_i16;
_31 = core::ptr::addr_of_mut!((*_31));
_17.1 = _23.1 << _7;
_34 = _18.0;
match _8 {
0 => bb1,
1 => bb4,
9037600299024999166 => bb14,
_ => bb5
}
}
bb14 = {
_30.0 = _28.0;
RET = _18;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(12_usize, 11_usize, Move(_11), 29_usize, Move(_29), 5_usize, Move(_5), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(12_usize, 9_usize, Move(_9), 4_usize, Move(_4), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(12_usize, 20_usize, Move(_20), 15_usize, Move(_15), 14_usize, Move(_14), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(157_u8), std::hint::black_box(2398027459993630773_i64));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: bool,
fld1: u8,
fld2: usize,
fld3: *const (f64,),
fld4: u32,
fld5: [u8; 4],
fld6: u128,
fld7: u64,

},
Variant1{
fld0: u128,
fld1: char,
fld2: i32,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: usize,
fld1: *const (u16, u16),
fld2: isize,
fld3: [u8; 4],
fld4: u64,
fld5: (f32, i32),

},
Variant1{
fld0: *const i64,
fld1: *mut bool,
fld2: u64,
fld3: i8,
fld4: u8,
fld5: (f32, i32),
fld6: (bool, isize),

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: i32,
fld1: [char; 5],
fld2: usize,
fld3: [u8; 7],
fld4: f64,

},
Variant1{
fld0: (*const (u16, u16), u16),
fld1: (u16, u16),
fld2: i128,
fld3: i16,

},
Variant2{
fld0: (u16, u16),
fld1: *const i64,
fld2: u64,
fld3: i8,
fld4: [char; 5],
fld5: i128,

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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *mut [char; 5],
fld1: i32,
fld2: *mut *mut bool,
fld3: i64,
fld4: i16,

},
Variant1{
fld0: i16,

},
Variant2{
fld0: *const (u16, u16),
fld1: [i64; 2],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: (bool, isize),
fld1: *mut i32,
fld2: *const i64,
fld3: *mut *mut [char; 5],
fld4: *mut bool,
fld5: [u64; 6],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: Adt41,
fld1: (*const (u16, u16), u16),
fld2: u16,
fld3: [char; 5],
fld4: *const (u16, u16),
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *mut *mut [char; 5],
fld1: *mut [char; 5],
fld2: Adt45,
fld3: *mut i32,
fld4: i16,
fld5: *mut *mut bool,
fld6: [u8; 4],

},
Variant1{
fld0: *mut i32,
fld1: f32,
fld2: (u128, char),
fld3: (f32, i32),
fld4: (*const (u16, u16), u16),
fld5: Adt41,
fld6: i128,

},
Variant2{
fld0: [u64; 6],
fld1: char,
fld2: (u128, char),
fld3: [i64; 2],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt46,
fld1: *const (f64,),
fld2: usize,
fld3: i8,
fld4: Adt44,
fld5: f32,
fld6: [u8; 7],

},
Variant1{
fld0: bool,
fld1: [u8; 7],

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
fld0: bool,
fld1: u64,
fld2: *mut [char; 5],
fld3: u8,
fld4: Adt47,

},
Variant1{
fld0: u128,
fld1: char,
fld2: (u16, u16),
fld3: *mut bool,
fld4: *mut *mut bool,
fld5: f32,
fld6: Adt41,
fld7: *mut *mut [char; 5],

},
Variant2{
fld0: Adt41,
fld1: u8,
fld2: *const u16,
fld3: [u64; 6],
fld4: Adt46,
fld5: i32,
fld6: *const (u16, u16),

},
Variant3{
fld0: usize,
fld1: (u16, u16),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: f64,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: (u16, u16),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: i16,
fld1: *const i64,
fld2: Adt42,
fld3: u16,

},
Variant1{
fld0: Adt45,
fld1: Adt49,
fld2: [i64; 2],

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: i8,
fld1: (f32, i32),
fld2: (f64,),

},
Variant1{
fld0: *mut [char; 5],
fld1: (*const (u16, u16), u16),
fld2: isize,
fld3: f64,
fld4: (bool, isize),
fld5: (f64,),
fld6: i64,
fld7: [char; 5],

},
Variant2{
fld0: u8,
fld1: Adt44,
fld2: f64,
fld3: i8,
fld4: [char; 5],
fld5: i32,
fld6: Adt47,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt41,
fld1: usize,
fld2: [u64; 6],
fld3: *mut *mut bool,
fld4: (f32, i32),
fld5: Adt48,
fld6: (u128, char),

},
Variant1{
fld0: u32,

}}
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *mut bool,
fld1: (f64,),
fld2: Adt52,
fld3: Adt42,
fld4: f64,

},
Variant1{
fld0: (f64,),
fld1: Adt48,
fld2: Adt51,
fld3: *mut *mut bool,
fld4: u128,
fld5: usize,

},
Variant2{
fld0: *const u16,
fld1: u8,
fld2: Adt45,
fld3: i8,
fld4: [i16; 1],
fld5: Adt43,
fld6: u16,
fld7: f32,

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: [i16; 1],
fld2: u32,
fld3: i8,
fld4: f32,
fld5: (bool, isize),

},
Variant1{
fld0: Adt41,
fld1: [u8; 7],
fld2: [u64; 6],
fld3: i8,
fld4: [i16; 1],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: (f64,),
fld1: *const (u16, u16),
fld2: u128,
fld3: *const i64,
fld4: Adt45,
fld5: Adt52,
fld6: *mut bool,

},
Variant1{
fld0: *mut *mut bool,
fld1: (*const (u16, u16), u16),
fld2: isize,
fld3: usize,
fld4: (u16, u16),
fld5: (f64,),

},
Variant2{
fld0: bool,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: u8,
fld1: char,
fld2: *mut *mut bool,
fld3: (u128, char),
}

